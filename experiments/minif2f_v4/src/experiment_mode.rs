// Phase C atom C1a — explicit `--mode` CLI flag for ablation experiments.
//
// PREREG_PPUT_CCL_2026-04-26 § 6 C1: "5 modes implemented as a single
// `--mode` CLI flag on one binary: evaluator --mode {full, panopticon,
// amnesia, soft_law, homogeneous}. Modes change runtime behavior;
// binary bytes do not change between modes."
//
// PREREG § 5.2 hypotheses H1-H4 motivate the 5 variants:
//   - Full          — baseline (all constitutional invariants on)
//   - Soft Law      — runtime-accept fast → Lean post-hoc reject →
//                     Progress = 0 → VPPUT drops. Detection mechanism:
//                     pput_runtime > 0 but pput_verified = 0.
//   - Panopticon    — agents see each other's full context (violates
//                     Art. II.2.1 cognitive isolation). CPR↑ + IAC↑ +
//                     prompt_length↑ → tokens↑ → PPUT↓.
//   - Amnesia       — disable L_t injection (agents lose memory of
//                     prior tactics). ERR↓ → PPUT↓.
//   - Homogeneous   — all agents share one skill prompt (Paper-1 era
//                     A condition). Heterogeneity gain disappears.
//
// Phase A scope (this commit): implement `Full` only. The other 4
// variants are declared so a misconfigured invocation aborts BEFORE
// the first LLM call instead of silently falling back to Full and
// burning budget under the wrong constitutional regime.
//
// Pattern mirrors `budget_regime.rs` (Phase A atom A5):
//   - pure parser  (parse_experiment_mode)
//   - env-coupled resolver  (resolve_experiment_mode, CLI > env)
//   - startup-fatal UnimplementedMode for declared-but-not-wired
//
// Constitutional anchors:
//   - FC1-N7 (δ/AI canonical identity) — Homogeneous + Soft Law
//     diverge in the prompt or accept gate per agent.
//   - Art. II.2.1 (cognitive isolation) — Panopticon's defining axis.
//   - FC3 / Art. III.2 (read-only logs) — Amnesia's L_t suppression.
//   - FC1-N12 (Lean ground-truth oracle scope) — Soft Law's
//     runtime-vs-verified gap site.

use std::fmt;

/// TRACE_MATRIX FC1-N7 + FC1-N12 + Art. II.2.1: env var selecting the
/// experiment ablation mode. Default (unset / blank) = `Full`,
/// preserving the Phase B baseline behavior bit-for-bit. Var name
/// `MODE` is preserved from the pre-C1a stub for backwards
/// compatibility with already-emitted v2 jsonl rows + 4 in-binary
/// tests (`v2_emit_*` series).
pub const EXPERIMENT_MODE_ENV_VAR: &str = "MODE";

/// TRACE_MATRIX FC1-N7 + FC1-N12: experiment ablation mode variants.
/// Only `Full` is implemented in Phase A scope (this commit); the
/// other 4 are declared so invoking `--mode=soft_law` (etc.) aborts
/// startup with a typed error naming what is missing, instead of
/// silently falling back to Full and corrupting Phase C ablation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExperimentMode {
    /// All constitutional invariants on. **Phase B baseline + default.**
    /// `runtime_accepted == post_hoc_verified` at make_pput call site;
    /// agents reason from their own scratchpads only; L_t injected;
    /// agent skills heterogeneous per AGENT_MODELS / role pool.
    Full,
    /// Cognitive-isolation breach: agents see each other's contexts.
    /// Violates Art. II.2.1. Detection: prompt_length↑ + IAC↑ + CPR↑.
    /// Phase C C1d will wire the runtime; C1a declares it.
    Panopticon,
    /// L_t injection suppressed: agents have no memory of prior
    /// tactics. Detection: ERR↓ on the descriptive secondary endpoint
    /// (PREREG § 5.2). Phase C C1e will wire the runtime; C1a declares.
    Amnesia,
    /// Runtime-accept gate fakes acceptance regardless of Lean
    /// verdict; Lean still runs post-hoc and the verified flag
    /// records the truth. Detection: `pput_runtime > 0` but
    /// `pput_verified = 0` for failed proofs. Phase C C1b will wire
    /// the runtime; C1a declares it. The make_pput two-leg signature
    /// (mid-term P0-A fix 2026-04-25) was put in place specifically
    /// to make Soft Law's design point unmissable here.
    SoftLaw,
    /// All swarm agents share one skill prompt (Paper-1 era A
    /// condition). Detection: solve set narrows to single-skill
    /// reachability. Phase C C1c will wire the runtime; C1a declares.
    Homogeneous,
}

impl ExperimentMode {
    /// Stable string label stamped on `RunAggregate.mode` /
    /// `PputResult.mode` jsonl rows. Stable across releases;
    /// downstream PPUT analysis joins on these exact strings.
    pub fn label(&self) -> &'static str {
        match self {
            ExperimentMode::Full => "full",
            ExperimentMode::Panopticon => "panopticon",
            ExperimentMode::Amnesia => "amnesia",
            ExperimentMode::SoftLaw => "soft_law",
            ExperimentMode::Homogeneous => "homogeneous",
        }
    }
}

/// Startup-fatal failure modes for the mode resolver. Each variant
/// aborts before the first LLM call so a misconfigured run cannot
/// consume API budget under the wrong constitutional regime.
#[derive(Debug, PartialEq, Eq)]
pub enum ModeError {
    /// `--mode` value (or `MODE` env) not in
    /// {`full`, `panopticon`, `amnesia`, `soft_law`, `homogeneous`}.
    UnknownMode(String),
    /// Caller asked for a declared-but-not-yet-wired mode. Carries
    /// the requested variant so the startup error names what is
    /// missing. Phase A scope = `Full` only; C1b/c/d/e wire the rest.
    UnimplementedMode(ExperimentMode),
}

impl fmt::Display for ModeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownMode(s) => write!(
                f,
                "experiment mode '{}' is not a known mode \
                 (expected full | panopticon | amnesia | soft_law | homogeneous)",
                s
            ),
            Self::UnimplementedMode(m) => write!(
                f,
                "experiment mode '{}' is declared but its runtime is not yet \
                 implemented (Phase C C1a scope = full only; C1b/c/d/e wire \
                 soft_law/homogeneous/panopticon/amnesia respectively). \
                 Aborting startup to avoid silent fallback to a different \
                 constitutional regime.",
                m.label()
            ),
        }
    }
}

impl std::error::Error for ModeError {}

/// Pure parser for the mode label (CLI value or env value). Empty
/// (unset / blank-after-trim) → default `Full`. Pure (no env access);
/// directly testable without process-global state.
pub fn parse_experiment_mode(s: &str) -> Result<ExperimentMode, ModeError> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Ok(ExperimentMode::Full);
    }
    match trimmed {
        "full" => Ok(ExperimentMode::Full),
        "panopticon" => Ok(ExperimentMode::Panopticon),
        "amnesia" => Ok(ExperimentMode::Amnesia),
        "soft_law" => Ok(ExperimentMode::SoftLaw),
        "homogeneous" => Ok(ExperimentMode::Homogeneous),
        other => Err(ModeError::UnknownMode(other.to_string())),
    }
}

/// Phase A scope guard: only `Full` is wired today. Returns the input
/// mode unchanged if implemented; otherwise `UnimplementedMode`.
/// Pure. C1b/c/d/e will progressively delete branches from the
/// declared list.
pub fn ensure_implemented(mode: ExperimentMode) -> Result<ExperimentMode, ModeError> {
    match mode {
        ExperimentMode::Full => Ok(mode),
        ExperimentMode::Panopticon
        | ExperimentMode::Amnesia
        | ExperimentMode::SoftLaw
        | ExperimentMode::Homogeneous => Err(ModeError::UnimplementedMode(mode)),
    }
}

/// Env-coupled resolver invoked once at evaluator entry, BEFORE any
/// LLM call. Returns the resolved + implementation-validated mode.
///
/// Resolution precedence (CLI > env > default):
///   - `cli_arg = Some(value)` (the user passed `--mode <value>` or
///     `--mode=<value>`): MUST be a valid + implemented mode. Empty
///     string after trim is rejected (UnknownMode("")) — explicit
///     CLI flags don't get the env-default-empty-blank treatment.
///   - `cli_arg = None` (no `--mode` flag): falls through to
///     `MODE` env var; empty / unset → default `Full`.
///
/// Errors abort the binary before the first LLM call.
pub fn resolve_experiment_mode(cli_arg: Option<&str>) -> Result<ExperimentMode, ModeError> {
    if let Some(cli_value) = cli_arg {
        let trimmed = cli_value.trim();
        if trimmed.is_empty() {
            return Err(ModeError::UnknownMode(cli_value.to_string()));
        }
        return ensure_implemented(parse_experiment_mode(trimmed)?);
    }
    let env_str = std::env::var(EXPERIMENT_MODE_ENV_VAR).unwrap_or_default();
    ensure_implemented(parse_experiment_mode(&env_str)?)
}

/// Extract `--mode <value>` / `--mode=<value>` from the argv vector,
/// returning `Some(value)` if found and removing the flag tokens
/// in-place so the remaining positional args can be processed by the
/// existing positional-argv-1 problem_file logic.
///
/// Returns `Some("")` if `--mode` appears as the last arg with no
/// value — the resolver will reject empty as `UnknownMode("")`.
/// Multiple `--mode` instances: last-wins (later overrides earlier),
/// matching POSIX convention.
pub fn extract_mode_flag(args: &mut Vec<String>) -> Option<String> {
    let mut found: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--mode" {
            if i + 1 < args.len() {
                found = Some(args[i + 1].clone());
                args.drain(i..=i + 1);
            } else {
                found = Some(String::new());
                args.remove(i);
            }
        } else if let Some(val) = args[i].strip_prefix("--mode=") {
            found = Some(val.to_string());
            args.remove(i);
        } else {
            i += 1;
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Per memory `feedback_env_var_test_lock`: tests that mutate the
    // process-global MODE env var must serialise to survive cargo's
    // parallel runner. Same lock pattern as budget_regime.rs.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn parse_empty_defaults_to_full() {
        assert_eq!(parse_experiment_mode("").unwrap(), ExperimentMode::Full);
        assert_eq!(parse_experiment_mode("   ").unwrap(), ExperimentMode::Full);
    }

    #[test]
    fn parse_known_values() {
        assert_eq!(parse_experiment_mode("full").unwrap(), ExperimentMode::Full);
        assert_eq!(parse_experiment_mode("panopticon").unwrap(), ExperimentMode::Panopticon);
        assert_eq!(parse_experiment_mode("amnesia").unwrap(), ExperimentMode::Amnesia);
        assert_eq!(parse_experiment_mode("soft_law").unwrap(), ExperimentMode::SoftLaw);
        assert_eq!(parse_experiment_mode("homogeneous").unwrap(), ExperimentMode::Homogeneous);
    }

    #[test]
    fn parse_unknown_rejected() {
        match parse_experiment_mode("foobar") {
            Err(ModeError::UnknownMode(s)) => assert_eq!(s, "foobar"),
            other => panic!("expected UnknownMode, got {:?}", other),
        }
    }

    #[test]
    fn parse_case_sensitive() {
        // PREREG § 6 C1 spells the labels in lowercase. Stable string
        // labels per ExperimentMode::label().
        match parse_experiment_mode("Full") {
            Err(ModeError::UnknownMode(s)) => assert_eq!(s, "Full"),
            other => panic!("expected UnknownMode (case-sensitive), got {:?}", other),
        }
        match parse_experiment_mode("SOFT_LAW") {
            Err(ModeError::UnknownMode(s)) => assert_eq!(s, "SOFT_LAW"),
            other => panic!("expected UnknownMode (case-sensitive), got {:?}", other),
        }
    }

    #[test]
    fn label_strings_are_stable() {
        // Downstream PPUT analysis joins on these exact strings;
        // changing them is a breaking change for the v2 schema.
        assert_eq!(ExperimentMode::Full.label(), "full");
        assert_eq!(ExperimentMode::Panopticon.label(), "panopticon");
        assert_eq!(ExperimentMode::Amnesia.label(), "amnesia");
        assert_eq!(ExperimentMode::SoftLaw.label(), "soft_law");
        assert_eq!(ExperimentMode::Homogeneous.label(), "homogeneous");
    }

    #[test]
    fn ensure_implemented_full_only_in_phase_a_scope() {
        assert_eq!(
            ensure_implemented(ExperimentMode::Full).unwrap(),
            ExperimentMode::Full
        );
        for m in [
            ExperimentMode::Panopticon,
            ExperimentMode::Amnesia,
            ExperimentMode::SoftLaw,
            ExperimentMode::Homogeneous,
        ] {
            match ensure_implemented(m) {
                Err(ModeError::UnimplementedMode(got)) => assert_eq!(got, m),
                other => panic!("expected UnimplementedMode({:?}), got {:?}", m, other),
            }
        }
    }

    // --- CLI argv extractor ---

    #[test]
    fn extract_no_flag_returns_none() {
        let mut args = vec!["evaluator".into(), "problem.lean".into()];
        assert_eq!(extract_mode_flag(&mut args), None);
        assert_eq!(args, vec!["evaluator", "problem.lean"]);
    }

    #[test]
    fn extract_space_form() {
        let mut args = vec![
            "evaluator".into(),
            "--mode".into(),
            "soft_law".into(),
            "problem.lean".into(),
        ];
        assert_eq!(extract_mode_flag(&mut args), Some("soft_law".into()));
        assert_eq!(args, vec!["evaluator", "problem.lean"]);
    }

    #[test]
    fn extract_equals_form() {
        let mut args = vec![
            "evaluator".into(),
            "--mode=panopticon".into(),
            "problem.lean".into(),
        ];
        assert_eq!(extract_mode_flag(&mut args), Some("panopticon".into()));
        assert_eq!(args, vec!["evaluator", "problem.lean"]);
    }

    #[test]
    fn extract_after_positional() {
        // `--mode` can appear after the problem file (POSIX-flexible).
        let mut args = vec![
            "evaluator".into(),
            "problem.lean".into(),
            "--mode".into(),
            "amnesia".into(),
        ];
        assert_eq!(extract_mode_flag(&mut args), Some("amnesia".into()));
        assert_eq!(args, vec!["evaluator", "problem.lean"]);
    }

    #[test]
    fn extract_dangling_flag_returns_empty_string() {
        // --mode at end of argv with no value: extractor surfaces an
        // empty string; resolver then rejects it as UnknownMode("").
        let mut args = vec!["evaluator".into(), "problem.lean".into(), "--mode".into()];
        assert_eq!(extract_mode_flag(&mut args), Some(String::new()));
        assert_eq!(args, vec!["evaluator", "problem.lean"]);
    }

    #[test]
    fn extract_last_wins_for_repeated_flag() {
        let mut args = vec![
            "evaluator".into(),
            "--mode=full".into(),
            "problem.lean".into(),
            "--mode".into(),
            "homogeneous".into(),
        ];
        assert_eq!(extract_mode_flag(&mut args), Some("homogeneous".into()));
        assert_eq!(args, vec!["evaluator", "problem.lean"]);
    }

    // --- env-coupled resolver ---

    #[test]
    fn resolve_default_no_cli_no_env_is_full() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::remove_var(EXPERIMENT_MODE_ENV_VAR);

        assert_eq!(resolve_experiment_mode(None).unwrap(), ExperimentMode::Full);
    }

    #[test]
    fn resolve_env_full_explicit() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var(EXPERIMENT_MODE_ENV_VAR, "full");
        assert_eq!(resolve_experiment_mode(None).unwrap(), ExperimentMode::Full);
        std::env::remove_var(EXPERIMENT_MODE_ENV_VAR);
    }

    #[test]
    fn resolve_env_unimplemented_aborts() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var(EXPERIMENT_MODE_ENV_VAR, "soft_law");
        match resolve_experiment_mode(None) {
            Err(ModeError::UnimplementedMode(ExperimentMode::SoftLaw)) => {}
            other => panic!("expected UnimplementedMode(SoftLaw), got {:?}", other),
        }
        std::env::remove_var(EXPERIMENT_MODE_ENV_VAR);
    }

    #[test]
    fn resolve_cli_overrides_env() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var(EXPERIMENT_MODE_ENV_VAR, "homogeneous");
        // CLI says full → resolver returns Full (overrides env's
        // homogeneous which would otherwise fail UnimplementedMode).
        assert_eq!(
            resolve_experiment_mode(Some("full")).unwrap(),
            ExperimentMode::Full
        );
        std::env::remove_var(EXPERIMENT_MODE_ENV_VAR);
    }

    #[test]
    fn resolve_cli_unimplemented_aborts() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::remove_var(EXPERIMENT_MODE_ENV_VAR);
        match resolve_experiment_mode(Some("panopticon")) {
            Err(ModeError::UnimplementedMode(ExperimentMode::Panopticon)) => {}
            other => panic!("expected UnimplementedMode(Panopticon), got {:?}", other),
        }
    }

    #[test]
    fn resolve_cli_unknown_aborts() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::remove_var(EXPERIMENT_MODE_ENV_VAR);
        match resolve_experiment_mode(Some("fancy_mode")) {
            Err(ModeError::UnknownMode(s)) => assert_eq!(s, "fancy_mode"),
            other => panic!("expected UnknownMode, got {:?}", other),
        }
    }

    #[test]
    fn resolve_cli_empty_aborts_no_default_fallback() {
        // Explicit --mode= with empty value is rejected, NOT silently
        // defaulted to Full. The env-fallback path defaults empty to
        // Full but the CLI path is stricter (typing --mode= is a typo,
        // not "I want the default").
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::remove_var(EXPERIMENT_MODE_ENV_VAR);
        match resolve_experiment_mode(Some("")) {
            Err(ModeError::UnknownMode(s)) => assert_eq!(s, ""),
            other => panic!("expected UnknownMode(\"\"), got {:?}", other),
        }
        match resolve_experiment_mode(Some("   ")) {
            Err(ModeError::UnknownMode(_)) => {}
            other => panic!("expected UnknownMode for whitespace-only, got {:?}", other),
        }
    }
}
