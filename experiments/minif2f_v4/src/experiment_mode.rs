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
// Phase C atom progression:
//   - C1a: implement `Full` only; declare other 4 startup-fatal so
//     misconfigured runs abort before the first LLM call instead of
//     silently falling back to Full.
//   - C1b: wire `SoftLaw` runtime via `apply_mode_to_accept`. H1
//     detection: `pput_runtime > 0` + `pput_verified = 0` gap on
//     Lean-rejected proofs.
//   - C1c: wire `Homogeneous` runtime via `skill_index_for_agent`.
//     H4 detection: solve set narrows to single-skill reachability.
//   - C1d: wire `Panopticon` via `is_panopticon`. H2 detection:
//     context-length / tokens grow O(N) per tx → cost dilution.
//   - C1e (this commit): wire `Amnesia` via `is_amnesia`. The
//     agent-facing proof chain projection (L_t) is suppressed
//     each tx — agent sees only the problem statement, no prior
//     nodes. Forces re-derivation from scratch every proposal.
//     Detection: ERR=0 (Effective Recall Rate); time/token inflation
//     per tx. Internal verification paths (tape+payload Lean
//     re-verify in run_swarm) are NOT touched — those are not
//     agent memory.
//
// All 5 modes wired post-C1e. Phase C C2 100-row batch + C3 H1-H4
// stat tests + C4 CHECKPOINT_PHASE_C dual audit are the next
// milestones; C5 binary purity test now has all 5 modes available
// for cross-mode comparison.
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
    /// Violates the cognitive-isolation invariant of the swarm
    /// economic design. C1d wires the runtime via `is_panopticon` —
    /// every agent's prompt at every tx receives the merged
    /// learned-memory of ALL N agents (cross-agent leakage), where
    /// Full mode would inject only the focal agent's own memory.
    /// Detection (H2): context length grows ~O(N) per tx → tokens↑
    /// → PPUT↓; the failure-mode is cost dilution, not loss of
    /// signal.
    Panopticon,
    /// L_t injection suppressed: agents have no memory of prior
    /// tactics. C1e wires the runtime via `is_amnesia` — at the
    /// per-tx agent prompt-construction site, the chain is forced
    /// to the problem-statement-only form (the same form used when
    /// `snap.tape.is_empty()`), so every proposal is generated from
    /// scratch with no carry-forward of prior partial tactics or
    /// rejected payloads. Detection: ERR=0 (PREREG § 5.2 secondary
    /// endpoint); time/token inflation per tx because agents must
    /// re-derive what previous tx already established.
    Amnesia,
    /// Runtime-accept gate fakes acceptance regardless of Lean
    /// verdict; Lean still runs post-hoc and the verified flag
    /// records the truth. Detection: `pput_runtime > 0` but
    /// `pput_verified = 0` for failed proofs. C1b wires the runtime
    /// via `apply_mode_to_accept` — every `make_pput` call site
    /// flows the (lean_rt, lean_ph) pair through that helper, and
    /// SoftLaw forces the first leg to true. The make_pput two-leg
    /// signature (mid-term P0-A fix 2026-04-25) was put in place
    /// specifically to make this design point unmissable.
    SoftLaw,
    /// All swarm agents share one skill prompt (Paper-1 era A
    /// condition). Detection: solve set narrows to single-skill
    /// reachability per H4. C1c wires the runtime via
    /// `skill_index_for_agent(Homogeneous, _, _) = 0` — every agent
    /// resolves to `agent_skills[0]` (algebraic) regardless of
    /// agent_idx, and the startup echo log shows `skill0` for all N.
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

/// Phase A→C progression scope guard. Returns the input mode unchanged
/// if its runtime is wired; otherwise `UnimplementedMode`. Pure. Each
/// of C1c/d/e progressively deletes one branch from the
/// declared-unimplemented list.
///
/// All 5 modes wired (post-C1e):
///   - Full        (Phase B baseline; pass-through everywhere)
///   - SoftLaw     (C1b — apply_mode_to_accept)
///   - Homogeneous (C1c — skill_index_for_agent)
///   - Panopticon  (C1d — is_panopticon)
///   - Amnesia     (C1e — is_amnesia)
///
/// `ensure_implemented` is now total — no UnimplementedMode return.
/// The function shape is preserved for forward-compatibility (a
/// future Phase F mode can re-introduce the gate without renaming
/// the helper) and to satisfy the API contract `--mode` already
/// validates against.
pub fn ensure_implemented(mode: ExperimentMode) -> Result<ExperimentMode, ModeError> {
    match mode {
        ExperimentMode::Full
        | ExperimentMode::SoftLaw
        | ExperimentMode::Homogeneous
        | ExperimentMode::Panopticon
        | ExperimentMode::Amnesia => Ok(mode),
    }
}

/// TRACE_MATRIX FC1-N12 + Art. III.2: apply the experiment mode to
/// the (runtime_accepted, post_hoc_verified) pair before make_pput.
/// Pure.
///
/// The make_pput signature was split into two legs at the 2026-04-25
/// mid-term audit P0-A fix specifically so Phase C ablations could
/// diverge here without laundering fake-accepts into the North Star
/// `pput_verified` field. Every make_pput call site funnels its
/// (lean_runtime_accepted, lean_post_hoc_verified) pair through this
/// helper; the mode parameter selects the divergence behavior.
///
/// Mode behaviors:
///   - **Full**: pass-through (Phase B identity — runtime IS Lean).
///   - **SoftLaw**: force runtime_accepted=true regardless of Lean.
///     post_hoc_verified preserves the Lean truth. The (true, false)
///     gap on Lean-rejected proofs is H1's detection mechanism. The
///     transform is uniform across all call sites — fence-reject /
///     max-tx-exhausted / synthetic-short-circuit also flip to
///     runtime-accept under Soft Law. PREREG § 5.2's `pput_runtime`
///     vs `pput_verified` gap is observed end-to-end.
///   - **Homogeneous / Panopticon / Amnesia**: pass-through; those
///     ablations diverge at prompt construction or skill assignment,
///     not at the accept axis. (Will be confirmed when C1c/d/e wire.)
///
/// Determinism: pure function of (mode, lean_rt, lean_ph). No env
/// reads, no I/O. Idempotent. Property: for any mode m, applying
/// twice equals applying once: `apply(m, apply(m, rt, ph)) ==
/// apply(m, rt, ph)`. Proven by the Soft Law branch ignoring its
/// rt input and by all other branches being pass-through.
pub fn apply_mode_to_accept(
    mode: ExperimentMode,
    lean_runtime_accepted: bool,
    lean_post_hoc_verified: bool,
) -> (bool, bool) {
    match mode {
        ExperimentMode::SoftLaw => (true, lean_post_hoc_verified),
        ExperimentMode::Full
        | ExperimentMode::Panopticon
        | ExperimentMode::Amnesia
        | ExperimentMode::Homogeneous => (lean_runtime_accepted, lean_post_hoc_verified),
    }
}

/// TRACE_MATRIX FC1-N7: predicate flagging Panopticon mode for
/// cognitive-isolation-breach call sites. Pure.
///
/// Phase C atom C1d uses this at the per-tx prompt-construction
/// site to expand the focal agent's learned-memory injection from
/// "this agent's own memory" to "the merged memory of all N agents".
/// Token cost grows ~O(N) per tx; detection mechanism for H2 (PREREG
/// § 5.2): pput_runtime / pput_verified both drop relative to Full
/// because cost goes up faster than signal at every N.
///
/// Helper exists separately from `apply_mode_to_accept` and
/// `skill_index_for_agent` because the call site needs to make a
/// branching choice between "fetch one agent's memory" vs "fetch
/// every agent's memory, then join" — the former is a single I/O
/// call, the latter is N I/O calls plus a join. A pure pred lets
/// the caller branch cleanly.
pub fn is_panopticon(mode: ExperimentMode) -> bool {
    matches!(mode, ExperimentMode::Panopticon)
}

/// TRACE_MATRIX FC3 + Art. III.2: predicate flagging Amnesia mode for
/// the L_t (proof chain) suppression call site. Pure.
///
/// Phase C atom C1e uses this at the per-tx agent-prompt
/// construction site to force `chain = problem_statement.to_string()`
/// regardless of `snap.tape` contents. Internal verification paths
/// that build `tape+payload` for Lean re-verify are NOT gated on
/// this predicate — Amnesia is about agent memory, not the verifier.
///
/// Detection mechanism (PREREG § 5.2 secondary endpoint, ERR):
/// Effective Recall Rate drops to 0 because the agent cannot recall
/// any prior partial tactic; time/token inflation per tx because
/// each proposal must re-derive what earlier tx already established.
pub fn is_amnesia(mode: ExperimentMode) -> bool {
    matches!(mode, ExperimentMode::Amnesia)
}

/// TRACE_MATRIX FC1-N7 + Art. II.2.1: select the skill index for a
/// swarm agent given the experiment mode. Pure.
///
/// The full skill pool (currently 3: algebraic / structural /
/// rewriting) is owned by the caller (run_swarm); this helper just
/// chooses the index.
///
/// Mode behaviors:
///   - **Homogeneous**: always 0 (every agent resolves to skill[0]
///     regardless of agent_idx). Forces single-skill reachability —
///     PREREG § 5.2 H4's defining axis. Paper-1 era's A condition.
///   - **Full / SoftLaw / Panopticon / Amnesia**: cycle through the
///     pool by `agent_idx % n_skills`. Heterogeneity per Art. II.2.1
///     ("不能抹杀群体异质性"); SoftLaw doesn't change skill axis,
///     only the accept axis; Panopticon and Amnesia diverge at
///     prompt construction (still TBD per C1d/e).
///
/// Edge case: `n_skills == 0` returns 0 (caller's lookup will return
/// the empty-string default; equivalent to no-skill prompt). The
/// production path always has n_skills == 3 today.
pub fn skill_index_for_agent(mode: ExperimentMode, agent_idx: usize, n_skills: usize) -> usize {
    if n_skills == 0 {
        return 0;
    }
    match mode {
        ExperimentMode::Homogeneous => 0,
        ExperimentMode::Full
        | ExperimentMode::SoftLaw
        | ExperimentMode::Panopticon
        | ExperimentMode::Amnesia => agent_idx % n_skills,
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
        assert_eq!(
            parse_experiment_mode("panopticon").unwrap(),
            ExperimentMode::Panopticon
        );
        assert_eq!(
            parse_experiment_mode("amnesia").unwrap(),
            ExperimentMode::Amnesia
        );
        assert_eq!(
            parse_experiment_mode("soft_law").unwrap(),
            ExperimentMode::SoftLaw
        );
        assert_eq!(
            parse_experiment_mode("homogeneous").unwrap(),
            ExperimentMode::Homogeneous
        );
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
    fn ensure_implemented_post_c1e_all_modes_pass() {
        // C1e completes the 5-mode wiring; ensure_implemented is now total.
        for m in [
            ExperimentMode::Full,
            ExperimentMode::SoftLaw,
            ExperimentMode::Homogeneous,
            ExperimentMode::Panopticon,
            ExperimentMode::Amnesia,
        ] {
            assert_eq!(ensure_implemented(m).unwrap(), m);
        }
    }

    #[test]
    fn is_panopticon_predicate() {
        assert!(is_panopticon(ExperimentMode::Panopticon));
        for m in [
            ExperimentMode::Full,
            ExperimentMode::SoftLaw,
            ExperimentMode::Homogeneous,
            ExperimentMode::Amnesia,
        ] {
            assert!(
                !is_panopticon(m),
                "is_panopticon should be false for {:?}",
                m
            );
        }
    }

    #[test]
    fn is_amnesia_predicate() {
        assert!(is_amnesia(ExperimentMode::Amnesia));
        for m in [
            ExperimentMode::Full,
            ExperimentMode::SoftLaw,
            ExperimentMode::Homogeneous,
            ExperimentMode::Panopticon,
        ] {
            assert!(!is_amnesia(m), "is_amnesia should be false for {:?}", m);
        }
    }

    #[test]
    fn predicates_are_mutually_exclusive() {
        // No mode should satisfy both is_panopticon and is_amnesia.
        // (They're orthogonal axes: cognitive isolation vs memory.)
        for m in [
            ExperimentMode::Full,
            ExperimentMode::SoftLaw,
            ExperimentMode::Homogeneous,
            ExperimentMode::Panopticon,
            ExperimentMode::Amnesia,
        ] {
            assert!(
                !(is_panopticon(m) && is_amnesia(m)),
                "is_panopticon and is_amnesia must be mutually exclusive (mode {:?})",
                m
            );
        }
    }

    // --- apply_mode_to_accept ---

    #[test]
    fn apply_full_is_passthrough() {
        // Full mode is the identity transform on (rt, ph). Phase B
        // baseline — runtime IS Lean today.
        for (rt, ph) in [(true, true), (true, false), (false, true), (false, false)] {
            assert_eq!(
                apply_mode_to_accept(ExperimentMode::Full, rt, ph),
                (rt, ph),
                "Full should be passthrough for (rt={}, ph={})",
                rt,
                ph
            );
        }
    }

    #[test]
    fn apply_soft_law_forces_runtime_accept() {
        // Soft Law: rt always true; ph preserved.
        // (true, true)   → (true, true)   — Lean accept; both legs
        // (true, false)  → (true, false)  — Lean reject post-hoc
        // (false, true)  → (true, true)   — runtime would have rejected
        //                                    but Lean accepts (rare; pass-through ph)
        // (false, false) → (true, false)  — Lean rejected; runtime fakes accept
        //                                    THIS IS THE H1 DETECTION POINT
        assert_eq!(
            apply_mode_to_accept(ExperimentMode::SoftLaw, true, true),
            (true, true)
        );
        assert_eq!(
            apply_mode_to_accept(ExperimentMode::SoftLaw, true, false),
            (true, false)
        );
        assert_eq!(
            apply_mode_to_accept(ExperimentMode::SoftLaw, false, true),
            (true, true)
        );
        assert_eq!(
            apply_mode_to_accept(ExperimentMode::SoftLaw, false, false),
            (true, false)
        );
    }

    #[test]
    fn apply_other_modes_passthrough_pre_implementation() {
        // Homogeneous / Panopticon / Amnesia don't affect the accept
        // axis (they diverge at prompt construction / skill assignment).
        // Pre-implementation behavior: pass-through. C1c/d/e will not
        // change this branch — they'll touch other code paths.
        for m in [
            ExperimentMode::Homogeneous,
            ExperimentMode::Panopticon,
            ExperimentMode::Amnesia,
        ] {
            for (rt, ph) in [(true, true), (true, false), (false, true), (false, false)] {
                assert_eq!(
                    apply_mode_to_accept(m, rt, ph),
                    (rt, ph),
                    "mode {:?} should be passthrough for (rt={}, ph={})",
                    m,
                    rt,
                    ph
                );
            }
        }
    }

    #[test]
    fn apply_idempotent_for_all_modes() {
        // Property: apply(m, apply(m, rt, ph)) == apply(m, rt, ph).
        // Trivially true for pass-through modes; for SoftLaw it holds
        // because (true, ph) → (true, ph) (rt input ignored both times).
        for m in [
            ExperimentMode::Full,
            ExperimentMode::SoftLaw,
            ExperimentMode::Panopticon,
            ExperimentMode::Amnesia,
            ExperimentMode::Homogeneous,
        ] {
            for (rt, ph) in [(true, true), (true, false), (false, true), (false, false)] {
                let once = apply_mode_to_accept(m, rt, ph);
                let twice = apply_mode_to_accept(m, once.0, once.1);
                assert_eq!(
                    once, twice,
                    "idempotence failed for mode {:?}, input ({}, {})",
                    m, rt, ph
                );
            }
        }
    }

    #[test]
    fn apply_soft_law_preserves_post_hoc_verified() {
        // Critical property: SoftLaw must NEVER mutate the post-hoc
        // (Lean truth) leg. Otherwise it could launder fake accepts
        // into the North Star pput_verified field. This test pins
        // that invariant against future careless edits.
        for ph in [true, false] {
            let (_rt, ph_out) = apply_mode_to_accept(ExperimentMode::SoftLaw, true, ph);
            assert_eq!(ph_out, ph, "SoftLaw must preserve ph; got {}", ph_out);
            let (_rt, ph_out) = apply_mode_to_accept(ExperimentMode::SoftLaw, false, ph);
            assert_eq!(ph_out, ph, "SoftLaw must preserve ph; got {}", ph_out);
        }
    }

    // --- skill_index_for_agent ---

    #[test]
    fn skill_idx_full_cycles_modulo() {
        // Full mode preserves the heterogeneous cycling pattern.
        for n_skills in [1usize, 2, 3, 5] {
            for idx in 0..(2 * n_skills) {
                assert_eq!(
                    skill_index_for_agent(ExperimentMode::Full, idx, n_skills),
                    idx % n_skills,
                    "Full should cycle modulo n_skills (idx={}, n={})",
                    idx,
                    n_skills,
                );
            }
        }
    }

    #[test]
    fn skill_idx_homogeneous_always_zero() {
        // Homogeneous mode resolves every agent to skill[0]. This
        // is H4's defining axis — Paper-1 era's A condition.
        for n_skills in [1usize, 2, 3, 5, 13] {
            for idx in 0..20 {
                assert_eq!(
                    skill_index_for_agent(ExperimentMode::Homogeneous, idx, n_skills),
                    0,
                    "Homogeneous must return 0 (idx={}, n={})",
                    idx,
                    n_skills,
                );
            }
        }
    }

    #[test]
    fn skill_idx_other_modes_passthrough() {
        // SoftLaw / Panopticon / Amnesia don't change the skill axis;
        // they diverge at the accept axis (SoftLaw) or prompt context
        // (Panopticon / Amnesia, when wired).
        for m in [
            ExperimentMode::SoftLaw,
            ExperimentMode::Panopticon,
            ExperimentMode::Amnesia,
        ] {
            for n_skills in [1usize, 2, 3, 5] {
                for idx in 0..(2 * n_skills) {
                    assert_eq!(
                        skill_index_for_agent(m, idx, n_skills),
                        idx % n_skills,
                        "mode {:?} should be passthrough (idx={}, n={})",
                        m,
                        idx,
                        n_skills,
                    );
                }
            }
        }
    }

    #[test]
    fn skill_idx_zero_n_returns_zero() {
        // Edge case: empty skill pool. Caller's pool lookup falls
        // through to the empty-string default. Production has
        // n_skills == 3 today.
        for m in [
            ExperimentMode::Full,
            ExperimentMode::SoftLaw,
            ExperimentMode::Homogeneous,
            ExperimentMode::Panopticon,
            ExperimentMode::Amnesia,
        ] {
            assert_eq!(skill_index_for_agent(m, 0, 0), 0);
            assert_eq!(skill_index_for_agent(m, 99, 0), 0);
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
    fn resolve_env_all_modes_implemented_post_c1e() {
        // Post-C1e all 5 modes pass the implementation gate. The
        // pre-C1e UnimplementedMode-via-env tests are gone — there
        // is no longer any mode that aborts startup. Unknown spellings
        // still abort (resolve_cli_unknown_aborts and
        // resolve_cli_empty_aborts_no_default_fallback below).
        let _guard = ENV_LOCK.lock().unwrap();
        for (label, expected) in [
            ("full", ExperimentMode::Full),
            ("soft_law", ExperimentMode::SoftLaw),
            ("homogeneous", ExperimentMode::Homogeneous),
            ("panopticon", ExperimentMode::Panopticon),
            ("amnesia", ExperimentMode::Amnesia),
        ] {
            std::env::set_var(EXPERIMENT_MODE_ENV_VAR, label);
            assert_eq!(resolve_experiment_mode(None).unwrap(), expected);
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

    // resolve_cli_unimplemented_aborts removed in C1e — there are no
    // longer any unimplemented modes; the UnimplementedMode error
    // variant is preserved on `ModeError` for forward-compat (a future
    // Phase F mode would land in declared-but-not-wired state) but is
    // unreachable via the production code path in Phase C.

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
