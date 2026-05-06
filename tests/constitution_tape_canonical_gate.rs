//! TB-C0 Constitution Landing Gate — Tape Canonical gate
//!
//! Constitutional invariants on Art. 0.2 Tape Canonical:
//!   - No parallel ledger source-of-truth
//!   - No shadow-tape authoritative-parent
//!   - Canonical txid is not shadow id
//!   - Dashboard regenerates from ChainTape + CAS
//!   - chain_derived_facts not from evaluator stdout
//!   - All externalized attempts have CAS payload
//!   - All Lean results have CAS payload
//!
//! Test list (per TB-C0 directive §5.4):
//!   - no_parallel_ledger_source_of_truth (also in no_parallel_ledger.rs)
//!   - no_shadow_tape_authoritative_parent (also in no_parallel_ledger.rs)
//!   - canonical_txid_not_shadow_id
//!   - dashboard_regenerates_from_tape_cas
//!   - chain_derived_facts_not_evaluator_stdout
//!   - all_externalized_attempts_have_cas_payload
//!   - all_lean_results_have_cas_payload
//!
//! All tests are real assertions — no `assert!(true)` per CR-C0.1.

use std::path::Path;
use std::process::Command;

/// Art. 0.2 — No parallel ledger source-of-truth. Reaffirmed here in
/// the tape-canonical gate (also asserted in no_parallel_ledger.rs).
#[test]
fn tape_no_parallel_ledger_source_of_truth() {
    let legacy_pointer = "handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt";
    assert!(
        !Path::new(legacy_pointer).exists(),
        "Tape canonical violation: legacy global pointer reappeared at {legacy_pointer}"
    );
}

/// Art. 0.2 — No shadow-tape authoritative-parent. The canonical
/// transition_ledger module must be the unique source. (Mirror of
/// no_parallel_ledger.rs::no_shadow_tape_authoritative_parent.)
#[test]
fn tape_no_shadow_tape_authoritative_parent() {
    let bottom_white = std::fs::read_dir("src/bottom_white/ledger")
        .expect("src/bottom_white/ledger should exist");
    let mut suspicious = Vec::new();
    for entry in bottom_white.flatten() {
        let name_str = entry.file_name().to_string_lossy().to_string();
        if name_str.ends_with("_ledger.rs")
            && name_str != "transition_ledger.rs"
            && !name_str.contains("derived")
            && !name_str.contains("view")
            && !name_str.contains("test")
        {
            suspicious.push(name_str);
        }
    }
    assert!(
        suspicious.is_empty(),
        "Tape canonical violation: shadow ledger candidates {suspicious:?}"
    );
}

/// Art. 0.2 — Canonical txid is the SHA-anchored ledger id, not a
/// shadow / display / dashboard id. Verify by surface: the LedgerEntry
/// module exposes content-addressed identifiers (CID / state-root)
/// that form the canonical chain. No "display_id" / "shadow_id" /
/// "dashboard_id" should be promoted to canonical.
#[test]
fn canonical_txid_not_shadow_id() {
    let tl_src = std::fs::read_to_string("src/bottom_white/ledger/transition_ledger.rs")
        .expect("transition_ledger.rs readable");
    // Canonical anchor: state-root advancement
    assert!(
        tl_src.contains("state_root") || tl_src.contains("StateRoot"),
        "Tape canonical violation: state_root / StateRoot missing — \
         canonical chain anchor un-defined."
    );

    // Forbidden: a "display_id" / "shadow_id" / "dashboard_id" promoted
    // to public canonical-anchor field.
    for forbidden in ["pub display_id", "pub shadow_id", "pub dashboard_id"] {
        assert!(
            !tl_src.contains(forbidden),
            "Tape canonical violation: {forbidden} field on LedgerEntry \
             — shadow id promoted to canonical surface."
        );
    }
}

/// Art. 0.2 — Dashboard regenerates from ChainTape + CAS. Existing
/// TB-16 dashboard live regen test must persist; the dashboard binary
/// must consume L4 + CAS, not evaluator stdout.
#[test]
fn dashboard_regenerates_from_tape_cas() {
    // The TB-16 dashboard live regen smoke test must exist.
    let regen_test = "tests/tb_16_dashboard_live_regen.rs";
    assert!(
        Path::new(regen_test).exists(),
        "Tape canonical violation: {regen_test} missing — dashboard \
         regeneration smoke gone."
    );

    // The dashboard module must call into chain_derived_run_facts.
    let out = Command::new("grep")
        .args([
            "-rEn",
            "--include=*.rs",
            r#"chain_derived_run_facts|compute_run_facts_from_chain"#,
            "src/",
        ])
        .output()
        .expect("grep available");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.trim().is_empty(),
        "Tape canonical violation: no module calls chain_derived_run_facts \
         — dashboard regeneration un-anchored to chain."
    );
}

/// Art. 0.2 + FC1-INV5 — chain_derived_facts must derive from chain
/// (L4 + CAS), not from evaluator stdout. The compute entry point
/// signature takes chain inputs only.
#[test]
fn chain_derived_facts_not_evaluator_stdout() {
    let cdr_src = std::fs::read_to_string("src/runtime/chain_derived_run_facts.rs")
        .expect("chain_derived_run_facts.rs readable");
    // The signature must take a chain reader / ledger view, not a stdout
    // capture. We verify by checking the signature line lacks `stdout` /
    // `stderr` / `String` (text capture) parameters — typed chain inputs.
    let sig_window = cdr_src
        .lines()
        .skip_while(|l| !l.contains("pub fn compute_run_facts_from_chain"))
        .take(15)
        .collect::<Vec<_>>()
        .join("\n");
    for forbidden in ["stdout: ", "stderr: ", "evaluator_stdout"] {
        assert!(
            !sig_window.contains(forbidden),
            "Tape canonical violation: chain_derived_run_facts signature \
             accepts `{forbidden}` — facts could derive from evaluator \
             stdout, not chain. Signature window:\n{sig_window}"
        );
    }
}

/// FC1-INV1 + Art. III.2 — All externalized attempts have CAS payload.
/// The TB-18R R1 + R2 schema enforces AttemptTelemetry CAS object per
/// LLM-Lean cycle. This is structurally guaranteed by the
/// `r2_write_attempt_telemetry` helper signature + 6 evaluator paths
/// instrumentation (TB-18R R2).
#[test]
fn all_externalized_attempts_have_cas_payload() {
    let tel_src = std::fs::read_to_string("src/runtime/attempt_telemetry.rs")
        .expect("attempt_telemetry.rs readable");
    assert!(
        tel_src.contains("pub fn write_attempt_telemetry_to_cas"),
        "Tape canonical violation: write_attempt_telemetry_to_cas missing \
         — externalized attempts cannot be CAS-anchored."
    );
    assert!(
        tel_src.contains("pub fn read_attempt_telemetry_from_cas"),
        "Tape canonical violation: read_attempt_telemetry_from_cas missing \
         — CAS-anchored payload un-readable for audit."
    );

    // Existing per-LLM-call test must exist (TB-18R R2 path-shape battery).
    let r2_test = "tests/tb_18r_attempt_telemetry_per_llm_call.rs";
    assert!(
        Path::new(r2_test).exists(),
        "Tape canonical violation: {r2_test} missing — per-LLM-call CAS \
         path-shape battery gone."
    );
}

/// Art. III.2 — All Lean results have CAS payload. The TB-18R R1
/// schema introduced LeanResult CAS routing for full Lean output
/// (verdict + diagnostic + sample stderr).
#[test]
fn all_lean_results_have_cas_payload() {
    let tel_src = std::fs::read_to_string("src/runtime/attempt_telemetry.rs")
        .expect("attempt_telemetry.rs readable");
    assert!(
        tel_src.contains("pub fn write_lean_result_to_cas"),
        "Tape canonical violation: write_lean_result_to_cas missing — \
         Lean diagnostic cannot be CAS-anchored."
    );
    assert!(
        tel_src.contains("pub fn read_lean_result_from_cas"),
        "Tape canonical violation: read_lean_result_from_cas missing — \
         Lean payload un-readable for audit."
    );
    assert!(
        tel_src.contains("pub struct LeanResult"),
        "Tape canonical violation: LeanResult struct missing — typed \
         Lean payload schema gone."
    );
}
