// Phase A atom A3 — per-agent model assignment (`AGENT_MODELS` env var).
//
// Constitutional anchor: FC1-N7 (δ/AI canonical identity). Each Agent_i in
// the swarm path embodies one δ instance; today every Agent_i shares a
// single global δ pinned by `ACTIVE_MODEL` env var. Phase D introduces a
// heterogeneous swarm where Agent_i may bind a different δ. This module
// is the env-var → per-agent δ resolver.
//
// **Phase B+C invariant** (notepad F-2026-04-25-02 + memory
// `feedback_phased_checkpoint`): Phases B and C MUST stay single-model so
// the ablation axes (Soft Law / Panopticon / Amnesia / Homogeneous /
// Full) are not confounded by model identity. Heterogeneous assignment
// is therefore *gated* by `PHASE_D_HETERO_OK=1` — until the gate is set
// the resolver rejects any non-uniform `AGENT_MODELS` payload at startup,
// before a single LLM call goes out.
//
// Default behavior (env var unset OR empty): broadcast the global model
// (resolved from `ACTIVE_MODEL`) to every agent slot — preserves Phase B
// behavior bit-for-bit.

use std::collections::BTreeSet;
use std::fmt;

/// TRACE_MATRIX FC1-N7: env var name binding the per-Agent_i δ vector.
pub const AGENT_MODELS_ENV_VAR: &str = "AGENT_MODELS";

/// TRACE_MATRIX FC1-N7: Phase D heterogeneity gate. Required for any
/// AGENT_MODELS payload containing ≥2 distinct δ values. Phase B+C
/// invariant: single δ across all Agent_i.
pub const PHASE_D_HETERO_GATE_ENV_VAR: &str = "PHASE_D_HETERO_OK";
pub const G4_REQUIRED_MODEL_FAMILIES_ENV_VAR: &str = "TURINGOS_G4_REQUIRED_MODEL_FAMILIES";
pub const G4_SINGLE_MODEL_DIAGNOSTIC_ENV_VAR: &str = "TURINGOS_G4_SINGLE_MODEL_DIAGNOSTIC";

/// TRACE_MATRIX FC1-N7: startup-fatal failure modes when the per-agent
/// δ vector cannot be safely resolved. Each variant aborts the run
/// before the first LLM call, preserving budget under misconfiguration.
#[derive(Debug, PartialEq, Eq)]
pub enum AgentModelsError {
    /// `AGENT_MODELS` parsed to N entries but the swarm has M ≠ N agents.
    /// (Length 1 broadcasts; only N>1 mismatches reach this branch.)
    LengthMismatch { provided: usize, expected: usize },
    /// A CSV slot was empty after trim (e.g., `"a,,b"` or `",a"`).
    EmptyEntry { index: usize },
    /// Two or more distinct models were supplied without
    /// `PHASE_D_HETERO_OK=1`. Phase B+C single-model invariant.
    HeterogeneousWithoutGate { distinct: Vec<String> },
    /// G4.2 ship evidence requested a minimum number of model families but
    /// the resolver observed fewer. This fails closed unless the run is
    /// explicitly marked as a single-model diagnostic.
    InsufficientModelFamilies {
        observed: usize,
        required: usize,
        families: Vec<String>,
    },
}

impl fmt::Display for AgentModelsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LengthMismatch { provided, expected } => write!(
                f,
                "AGENT_MODELS length mismatch: {} provided, {} agents in swarm \
                 (use length 1 to broadcast or length == n_agents for positional)",
                provided, expected
            ),
            Self::EmptyEntry { index } => write!(
                f,
                "AGENT_MODELS entry at index {} is empty (CSV slot blank after trim)",
                index
            ),
            Self::HeterogeneousWithoutGate { distinct } => write!(
                f,
                "AGENT_MODELS contains {} distinct models {:?} but \
                 PHASE_D_HETERO_OK is not set to '1'. Phase B+C ablations \
                 require single-model invariant (notepad F-2026-04-25-02).",
                distinct.len(),
                distinct
            ),
            Self::InsufficientModelFamilies {
                observed,
                required,
                families,
            } => write!(
                f,
                "G4.2 model-family fail-closed: observed {} families {:?}, required {}. \
                 Set TURINGOS_G4_SINGLE_MODEL_DIAGNOSTIC=1 only for non-ship diagnostics.",
                observed, families, required
            ),
        }
    }
}

impl std::error::Error for AgentModelsError {}

/// TRACE_MATRIX FC1-N7: pure CSV parser for the `AGENT_MODELS` payload.
/// Empty input (env unset or empty string) → empty Vec (caller falls
/// back to broadcasting the global model). No env access — testable
/// without process-global state.
pub fn parse_agent_models(env_str: &str) -> Result<Vec<String>, AgentModelsError> {
    let trimmed = env_str.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    let entries: Vec<String> = trimmed.split(',').map(|s| s.trim().to_string()).collect();
    for (i, e) in entries.iter().enumerate() {
        if e.is_empty() {
            return Err(AgentModelsError::EmptyEntry { index: i });
        }
    }
    Ok(entries)
}

/// TRACE_MATRIX FC1-N7: validator + expander. Maps parsed CSV entries
/// to a per-Agent_i δ vector of length `n_agents`. Pure (no env access).
///
/// - parsed empty → broadcast `global_model` to every agent.
/// - parsed.len() == 1 → broadcast that single model.
/// - parsed.len() == n_agents → positional assignment.
/// - else → `LengthMismatch`.
///
/// Heterogeneity (≥2 distinct models in the resolved vector) requires
/// `hetero_gated == true`; otherwise `HeterogeneousWithoutGate`.
pub fn expand_agent_models(
    parsed: Vec<String>,
    global_model: &str,
    n_agents: usize,
    hetero_gated: bool,
) -> Result<Vec<String>, AgentModelsError> {
    let resolved = if parsed.is_empty() {
        vec![global_model.to_string(); n_agents]
    } else if parsed.len() == 1 {
        vec![parsed.into_iter().next().unwrap(); n_agents]
    } else if parsed.len() == n_agents {
        parsed
    } else {
        return Err(AgentModelsError::LengthMismatch {
            provided: parsed.len(),
            expected: n_agents,
        });
    };

    let distinct: BTreeSet<&str> = resolved.iter().map(String::as_str).collect();
    if distinct.len() > 1 && !hetero_gated {
        return Err(AgentModelsError::HeterogeneousWithoutGate {
            distinct: distinct.into_iter().map(String::from).collect(),
        });
    }
    Ok(resolved)
}

/// TRACE_MATRIX FC1-N7: env-coupled wrapper used by `run_swarm` to
/// produce the per-Agent_i δ vector. Composes `parse_agent_models` +
/// `expand_agent_models`; reads `AGENT_MODELS` and the Phase D
/// heterogeneity gate from process env.
pub fn resolve_agent_models(
    global_model: &str,
    n_agents: usize,
) -> Result<Vec<String>, AgentModelsError> {
    let raw = std::env::var(AGENT_MODELS_ENV_VAR).unwrap_or_default();
    let hetero_gated = std::env::var(PHASE_D_HETERO_GATE_ENV_VAR).as_deref() == Ok("1");
    let parsed = parse_agent_models(&raw)?;
    let resolved = expand_agent_models(parsed, global_model, n_agents, hetero_gated)?;
    let required_families = std::env::var(G4_REQUIRED_MODEL_FAMILIES_ENV_VAR)
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);
    let single_model_diagnostic =
        std::env::var(G4_SINGLE_MODEL_DIAGNOSTIC_ENV_VAR).as_deref() == Ok("1");
    if required_families > 0 && !single_model_diagnostic {
        let families = distinct_model_families(&resolved);
        if families.len() < required_families {
            return Err(AgentModelsError::InsufficientModelFamilies {
                observed: families.len(),
                required: required_families,
                families,
            });
        }
    }
    Ok(resolved)
}

pub fn distinct_model_families(models: &[String]) -> Vec<String> {
    let mut families: Vec<String> = models
        .iter()
        .map(|m| turingosv4::runtime::genesis_report::model_family_from_name(m))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    families.sort();
    families
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_env_parses_to_empty_vec() {
        assert_eq!(parse_agent_models("").unwrap(), Vec::<String>::new());
        assert_eq!(parse_agent_models("   ").unwrap(), Vec::<String>::new());
    }

    #[test]
    fn single_entry_parses() {
        assert_eq!(
            parse_agent_models("deepseek-v4-flash").unwrap(),
            vec!["deepseek-v4-flash".to_string()]
        );
    }

    #[test]
    fn csv_entries_trimmed() {
        assert_eq!(
            parse_agent_models("a, b ,c").unwrap(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }

    #[test]
    fn empty_csv_slot_rejected() {
        assert_eq!(
            parse_agent_models("a,,b"),
            Err(AgentModelsError::EmptyEntry { index: 1 })
        );
        assert_eq!(
            parse_agent_models(",a"),
            Err(AgentModelsError::EmptyEntry { index: 0 })
        );
        assert_eq!(
            parse_agent_models("a,"),
            Err(AgentModelsError::EmptyEntry { index: 1 })
        );
    }

    #[test]
    fn empty_parsed_broadcasts_global_model() {
        let v = expand_agent_models(vec![], "deepseek-v4-flash", 3, false).unwrap();
        assert_eq!(
            v,
            vec![
                "deepseek-v4-flash".to_string(),
                "deepseek-v4-flash".to_string(),
                "deepseek-v4-flash".to_string()
            ]
        );
    }

    #[test]
    fn single_entry_broadcasts() {
        let v = expand_agent_models(vec!["x".to_string()], "deepseek-v4-flash", 4, false).unwrap();
        assert_eq!(v, vec!["x".to_string(); 4]);
    }

    #[test]
    fn positional_length_match_passes() {
        let v = expand_agent_models(
            vec!["a".into(), "a".into(), "a".into()],
            "fallback",
            3,
            false,
        )
        .unwrap();
        assert_eq!(v, vec!["a".to_string(); 3]);
    }

    #[test]
    fn length_mismatch_rejected() {
        assert_eq!(
            expand_agent_models(vec!["a".into(), "b".into()], "g", 3, true,),
            Err(AgentModelsError::LengthMismatch {
                provided: 2,
                expected: 3
            })
        );
    }

    #[test]
    fn heterogeneous_without_gate_rejected() {
        let err = expand_agent_models(vec!["a".into(), "b".into(), "a".into()], "g", 3, false)
            .unwrap_err();
        match err {
            AgentModelsError::HeterogeneousWithoutGate { distinct } => {
                assert_eq!(distinct, vec!["a".to_string(), "b".to_string()]);
            }
            other => panic!("expected HeterogeneousWithoutGate, got {:?}", other),
        }
    }

    #[test]
    fn heterogeneous_with_gate_passes() {
        let v =
            expand_agent_models(vec!["a".into(), "b".into(), "a".into()], "g", 3, true).unwrap();
        assert_eq!(v, vec!["a".to_string(), "b".to_string(), "a".to_string()]);
    }

    #[test]
    fn uniform_length_n_does_not_trip_hetero_gate() {
        // Length-N positional payload that happens to be uniform must
        // pass without the gate — only *distinct* values trigger it.
        let v =
            expand_agent_models(vec!["a".into(), "a".into(), "a".into()], "g", 3, false).unwrap();
        assert_eq!(v, vec!["a".to_string(); 3]);
    }
}
