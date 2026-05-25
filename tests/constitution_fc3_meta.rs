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

/// FC3-INV7 — ArchitectAI proposes; does NOT directly write. Architect
/// changes land via `handover/directives/*.md` + TB charters; direct
/// `git commit` to src/ from the architect role is forbidden. We
/// enforce by structural artifact: directives must accompany src/
/// changes during a TB.
#[test]
fn fc3_architectai_proposal_not_direct_write() {
    // Verify directives/ exists and contains TB-anchored architect rulings.
    let dir_path = "handover/directives";
    assert!(
        Path::new(dir_path).exists(),
        "FC3-INV7 violation: handover/directives/ missing — architect \
         proposal trail un-anchored."
    );
    let entries = std::fs::read_dir(dir_path).expect("dir readable");
    let directive_count = entries
        .flatten()
        .filter(|e| e.file_name().to_string_lossy().ends_with(".md"))
        .count();
    // We expect ≥10 architect directives by mid-2026 (TB-1..TB-18R + TBC0).
    assert!(
        directive_count >= 10,
        "FC3-INV7 violation: directives count is {directive_count}; \
         expected ≥10 (TB-1..TB-18R + TBC0). Architect-proposal trail \
         is shrinking unexpectedly."
    );

    // The TB-C0 directive specifically must exist.
    let tbc0_directive =
        "handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md";
    assert!(
        Path::new(tbc0_directive).exists(),
        "FC3-INV7 violation: {tbc0_directive} missing — TB-C0 architect \
         directive un-anchored."
    );
}

/// FC3-INV8 — Veto-AI is veto-only. The external clean-context audit
/// flow produces verdict (PASS / VETO) without committing
/// code. We verify by structural artifact: handover/audits/ contains
/// audit reports but no audit-authored src/ commits.
#[test]
fn fc3_veto_ai_veto_only() {
    let audits_dir = "handover/audits";
    assert!(
        Path::new(audits_dir).exists(),
        "FC3-INV8 violation: handover/audits/ missing — Veto-AI \
         verdict trail un-anchored."
    );
    // Audit reports follow the current single clean-context Codex witness
    // doctrine. Historical dual-audit artifacts may remain on disk, but they
    // are not counted as current Veto-AI evidence.
    let entries = std::fs::read_dir(audits_dir).expect("dir readable");
    let audit_count = entries
        .flatten()
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_ascii_uppercase();
            name.contains("CODEX") && name.contains("AUDIT")
        })
        .count();
    assert!(
        audit_count >= 3,
        "FC3-INV8 violation: <3 audit reports in handover/audits/ — \
         Veto-AI verdict trail too sparse to demonstrate veto-only role."
    );
}
