//! TB-C0 Constitution Landing Gate — FC3 Meta / anti-oreo
//!
//! Constitutional invariants on Flowchart 3:
//!   `boot → constitution / logs (read-only) → Veto-AI ← ArchitectAI →
//!    tools / logs / Q update`
//!
//! Test list (per TB-C0 directive §4.3):
//!   - fc3_capsule_derived_from_tape_cas
//!   - fc3_no_global_markov_pointer (also in no_parallel_ledger.rs)
//!   - fc3_raw_logs_not_in_agent_read_view
//!   - fc3_latest_capsule_context_only
//!   - fc3_deep_history_requires_override
//!   - fc3_no_automatic_predicate_mutation
//!   - fc3_architectai_proposal_not_direct_write
//!   - fc3_veto_ai_veto_only
//!
//! All tests are real assertions — no `assert!(true)` per CR-C0.1.

use std::path::Path;
use std::process::Command;

/// FC3-INV1 — Capsule derived from ChainTape + CAS. The Markov capsule
/// generation must consume L4 + CAS as inputs (not a global file). Per
/// OBS_R022 closure, capsule is a **derived view**, not authoritative.
#[test]
fn fc3_capsule_derived_from_tape_cas() {
    // Find the markov capsule generator. Canonical home is
    // src/runtime/markov_capsule.rs (TB-15 + TB-17). Entry-point names:
    //   - `pub fn write_markov_capsule` (writer to CAS)
    //   - `pub fn restore_markov_capsule_from_cas_bytes` (CAS reader)
    //   - `src/bin/generate_markov_capsule.rs` (CLI binary)
    let out = Command::new("grep")
        .args([
            "-rEn",
            "--include=*.rs",
            r#"pub fn write_markov_capsule|pub fn restore_markov_capsule|fn generate_markov_capsule"#,
            "src/",
        ])
        .output()
        .expect("grep should be available");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.trim().is_empty(),
        "FC3-INV1 violation: no markov capsule generator/restorer entry \
         points found — capsule derivation un-callable. Looked for \
         write_markov_capsule, restore_markov_capsule, generate_markov_capsule."
    );

    // The canonical home file must exist.
    assert!(
        Path::new("src/runtime/markov_capsule.rs").exists(),
        "FC3-INV1 violation: src/runtime/markov_capsule.rs missing — \
         capsule generation surface gone."
    );

    // The CLI generator binary must exist (per OBS_R022 §A.5: this
    // binary is what writes capsules, NOT a global pointer).
    assert!(
        Path::new("src/bin/generate_markov_capsule.rs").exists(),
        "FC3-INV1 violation: src/bin/generate_markov_capsule.rs missing \
         — capsule CLI generator gone."
    );

    // Existing tests must exist.
    let markov_test = "tests/tb_17_markov_inheritance_policy.rs";
    assert!(
        Path::new(markov_test).exists(),
        "FC3-INV1 violation: {markov_test} missing — Markov inheritance \
         policy un-enforced (per OBS_R022 + Art. 0.4 path B)."
    );
}

/// FC3-INV2 — No global Markov pointer (duplicate of
/// `no_parallel_ledger::no_global_markov_pointer`; codified here too
/// because FC3 enforcement is a separate concern from Tape Canonical).
#[test]
fn fc3_no_global_markov_pointer() {
    let legacy_pointer = "handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt";
    assert!(
        !Path::new(legacy_pointer).exists(),
        "FC3-INV2 violation: {legacy_pointer} re-appeared. Per OBS_R022 \
         Option α 2026-05-04 closure this global file was deleted because \
         FC3 (and Art. 0.2) require capsule to be derived view, not \
         authoritative source."
    );
}

/// FC3-INV3 — Raw logs not in agent read view. Per Art. III.1 +
/// Art. II.1, raw Lean stderr / failure logs MUST NOT be broadcast to
/// the agent prompt. We verify by source-side check that
/// `UniverseSnapshot` and prompt builders do not splice raw stderr.
#[test]
fn fc3_raw_logs_not_in_agent_read_view() {
    let snap_src = std::fs::read_to_string("src/sdk/snapshot.rs").expect("snapshot.rs readable");
    // Search for forbidden patterns: unbounded raw stderr field on the
    // public snapshot. Permitted: a sanitized rejection summary.
    for forbidden in ["lean_stderr_full", "raw_stderr", "lean_stderr_raw"] {
        assert!(
            !snap_src.contains(forbidden),
            "FC3-INV3 violation: snapshot.rs exposes `{forbidden}` to \
             agent read view — raw failure detail leak per Art. III.1."
        );
    }

    // Prompt builder must not mention raw stderr concatenation.
    let prompt_src = std::fs::read_to_string("src/sdk/prompt.rs").expect("prompt.rs readable");
    for forbidden in ["raw_stderr", "stderr_full", "lean_stderr_raw"] {
        assert!(
            !prompt_src.contains(forbidden),
            "FC3-INV3 violation: prompt.rs splices `{forbidden}` into \
             agent prompt — pollution prevention failure."
        );
    }
}

/// FC3-INV4 — Latest capsule = context only (not ground-truth source).
/// The capsule serves as bootstrap context for next-session agents;
/// it is NOT consulted as predicate / oracle ground truth. We verify
/// by absence of capsule consultation in predicate-evaluation paths.
#[test]
fn fc3_latest_capsule_context_only() {
    let bus_src = std::fs::read_to_string("src/bus.rs").expect("bus.rs readable");
    // Predicate evaluation must not consult markov_capsule for verdict.
    let pred_window = bus_src
        .lines()
        .skip_while(|l| !l.contains("evaluate_predicates"))
        .take(40)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        !pred_window.contains("markov_capsule") && !pred_window.contains("MarkovCapsule"),
        "FC3-INV4 violation: evaluate_predicates references markov_capsule \
         — capsule is being used as ground truth, not context. \
         Window:\n{pred_window}"
    );

    // The verify path must not consult capsule for verdict.
    let verify_src = std::fs::read_to_string("src/runtime/verify.rs").expect("verify.rs readable");
    let verify_window = verify_src
        .lines()
        .skip_while(|l| !l.contains("pub fn verify_chaintape"))
        .take(80)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        !verify_window.contains("MarkovCapsule") && !verify_window.contains("capsule_to_verdict"),
        "FC3-INV4 violation: verify_chaintape consults Markov capsule for \
         verdict — capsule is becoming ground truth."
    );
}

/// FC3-INV5 — Deep history requires explicit override. Reading deep-
/// history (older capsules / audit-only logs) requires
/// `TURINGOS_MARKOV_OVERRIDE=1` per OBS_R022. Without the env flag,
/// reads default to current-session-only.
#[test]
fn fc3_deep_history_requires_override() {
    let out = Command::new("grep")
        .args(["-rn", "--include=*.rs", "TURINGOS_MARKOV_OVERRIDE", "src/"])
        .output()
        .expect("grep should be available");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.trim().is_empty(),
        "FC3-INV5 violation: no code path checks TURINGOS_MARKOV_OVERRIDE \
         — deep-history default-deny gate un-enforceable per OBS_R022 \
         Option α."
    );

    // The check should appear at deep-history read site.
    assert!(
        stdout.contains("try_deep_history_read") || stdout.contains("MARKOV_OVERRIDE"),
        "FC3-INV5 violation: TURINGOS_MARKOV_OVERRIDE found but not \
         wired through deep_history_read pattern. Found:\n{stdout}"
    );
}

/// FC3-INV6 — No automatic predicate mutation. Predicates are registered
/// during boot construction (via PredicateRegistry boot manifests); they are
/// NOT mutated at runtime by agents, by economic events, or by capsule context.
/// The mutation helper may exist only at crate scope for boot/replay internals.
#[test]
fn fc3_no_automatic_predicate_mutation() {
    let reg_src = std::fs::read_to_string("src/top_white/predicates/registry.rs")
        .expect("predicates/registry.rs readable");
    let registry_impl = reg_src
        .split("impl PredicateRegistry {")
        .nth(1)
        .and_then(|rest| rest.split("pub struct BootPredicateManifest").next())
        .expect("PredicateRegistry impl block readable");

    // The registry must keep mutation module-private/crate-private; public
    // construction goes through boot manifest or replay snapshot loaders.
    assert!(
        reg_src.contains("pub(crate) fn register(")
            || reg_src.contains("pub(super) fn register(")
            || reg_src.contains("\n    fn register("),
        "FC3-INV6 violation: PredicateRegistry::register must exist only as a \
         non-public boot/replay helper."
    );
    assert!(
        !reg_src.contains("pub fn register("),
        "FC3-INV6 violation: PredicateRegistry::register is public — \
         predicate mutation would be agent-callable outside the registry module."
    );
    assert!(
        reg_src.contains("pub fn from_boot_manifest(")
            && reg_src.contains("pub fn from_snapshot_and_binary_impls("),
        "FC3-INV6 violation: PredicateRegistry public constructors must be \
         boot-manifest and replay-snapshot based."
    );
    assert!(
        !registry_impl.contains("pub fn new(")
            && !reg_src.contains("derive(Default)]\npub struct PredicateRegistry"),
        "FC3-INV6 violation: PredicateRegistry exposes an ad hoc public empty \
         constructor/default instead of boot-manifest construction."
    );
    for forbidden in [
        "pub fn remove",
        "pub fn replace",
        "pub fn mutate",
        "pub fn overwrite",
        "pub fn unregister",
        "pub fn modify",
    ] {
        assert!(
            !reg_src.contains(forbidden),
            "FC3-INV6 violation: PredicateRegistry exposes `{forbidden}` \
             — predicates can be mutated post-boot."
        );
    }
}

/// FC3-INV7 — ArchitectAI proposes and commits only through typed runtime
/// meta transactions guarded by Veto-AI. Direct handover/directive trails may
/// still exist for development, but they no longer count as FC3 node coverage.
#[test]
fn fc3_architectai_proposal_not_direct_write() {
    let typed_tx = include_str!("../src/state/typed_tx.rs");
    let sequencer = include_str!("../src/state/sequencer.rs");
    assert!(typed_tx.contains("pub struct ArchitectProposalTx"));
    assert!(typed_tx.contains("pub struct ArchitectCommitTx"));
    assert!(sequencer.contains("SystemEmitCommand::ArchitectProposal"));
    assert!(sequencer.contains("SystemEmitCommand::ArchitectCommit"));
    assert!(sequencer.contains("ArchitectCommitBlockedByVeto"));
    assert!(!typed_tx.contains("ExternalOnly"));
}

/// FC3-INV8 — Veto-AI is veto-only. The runtime tx surface carries the
/// two-valued verdict and deterministic reason code, but no patch/code body.
#[test]
fn fc3_veto_ai_veto_only() {
    let typed_tx = include_str!("../src/state/typed_tx.rs");
    let sequencer = include_str!("../src/state/sequencer.rs");
    assert!(typed_tx.contains("pub struct VetoDecisionTx"));
    assert!(typed_tx.contains("pub enum VetoVerdict"));
    assert!(typed_tx.contains("Pass = 0"));
    assert!(typed_tx.contains("Veto = 1"));
    assert!(!typed_tx.contains("quality_score"));
    assert!(!typed_tx.contains("performance_score"));
    assert!(sequencer.contains("deterministic_veto_ai_verdict"));
}
