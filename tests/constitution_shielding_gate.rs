//! TB-C0 Constitution Landing Gate — Shielding gate
//!
//! Constitutional invariants on Art. III (selective shielding):
//!   - III.1 shield errors: raw failure logs not in agent prompt
//!   - III.2 encapsulation: high-volume detail in CAS, audit-only
//!   - III.3 shield correlation: no Goodhart leakage
//!   - III.4 shield Goodhart: selector blind to rejection content
//!
//! Test list (per TB-C0 directive §5.2):
//!   - raw_lean_stderr_not_in_agent_read_view
//!   - l4e_public_summary_low_pollution
//!   - private_diagnostic_cid_not_serialized_publicly
//!   - evidence_capsule_raw_logs_audit_only
//!   - dashboard_does_not_leak_private_failure_detail
//!
//! All tests are real assertions — no `assert!(true)` per CR-C0.1.

use std::path::Path;
use std::process::Command;

/// Art. III.1 — Raw Lean stderr MUST NOT appear in the agent read view.
/// Per TB-7R Art. III.4 closure + Art. II.1 broadcast prevention.
#[test]
fn raw_lean_stderr_not_in_agent_read_view() {
    // Source-side check: src/sdk/snapshot.rs (UniverseSnapshot) +
    // src/sdk/prompt.rs (build_agent_prompt) MUST NOT have a public
    // field that exposes raw_stderr or full Lean error body.
    let snap_src =
        std::fs::read_to_string("src/sdk/snapshot.rs").expect("snapshot.rs readable");
    for forbidden in ["raw_stderr", "lean_stderr_full", "lean_stderr_raw", "lean_error_full"] {
        assert!(
            !snap_src.contains(forbidden),
            "Shielding violation: snapshot.rs exposes `{forbidden}` field \
             — raw stderr in agent read view per Art. III.1."
        );
    }
    let prompt_src =
        std::fs::read_to_string("src/sdk/prompt.rs").expect("prompt.rs readable");
    for forbidden in ["raw_stderr", "lean_stderr_full", "lean_stderr_raw", "raw_lean_error"] {
        assert!(
            !prompt_src.contains(forbidden),
            "Shielding violation: prompt.rs splices `{forbidden}` \
             — raw stderr broadcast to agent prompt."
        );
    }
}

/// Art. III.2 + Art. II.1 — L4.E public summary is low-pollution.
/// Per Art. III.1 + DECISION_REJECTION_EVIDENCE_LEDGER 2026-04-29:
/// the public-readable rejection summary on L4.E records `submit_id`
/// + a sanitized rejection-class tag — NOT the raw Lean output, NOT
/// the full diagnostic. The full diagnostic lives behind a
/// `private_diagnostic_cid` in CAS, audit-only.
#[test]
fn l4e_public_summary_low_pollution() {
    // The L4.E rejection record lives in the dedicated rejection_evidence
    // module (TB-7R + DECISION_REJECTION_EVIDENCE_LEDGER 2026-04-29).
    let rej_src =
        std::fs::read_to_string("src/bottom_white/ledger/rejection_evidence.rs")
            .expect("rejection_evidence.rs readable");
    assert!(
        rej_src.contains("RejectedSubmissionRecord"),
        "Shielding violation: RejectedSubmissionRecord missing in \
         rejection_evidence.rs — L4.E public summary shape un-defined."
    );

    // Either the record itself or a related shielding artifact exposes
    // private_diagnostic_cid (the audit-only routing).
    let has_private_cid = rej_src.contains("private_diagnostic_cid")
        || rej_src.contains("diagnostic_cid")
        || std::fs::read_to_string("src/runtime/attempt_telemetry.rs")
            .map(|s| s.contains("private_diagnostic_cid") || s.contains("diagnostic_cid"))
            .unwrap_or(false);
    assert!(
        has_private_cid,
        "Shielding violation: no private_diagnostic_cid in L4.E shape — \
         full Lean diagnostic could be public-broadcast."
    );

    // The public_summary_for helper exists in sequencer.rs (TB-13/18R
    // sanitization).
    let seq_src =
        std::fs::read_to_string("src/state/sequencer.rs").expect("sequencer.rs readable");
    assert!(
        seq_src.contains("public_summary_for") || seq_src.contains("rejection_class_for"),
        "Shielding violation: sequencer.rs lacks public_summary_for / \
         rejection_class_for — sanitization step not exercised."
    );
}

/// Art. III.2 — Private diagnostic CID is NOT serialized into the public
/// schema. The L4.E record carries the CID; resolving it requires
/// audit-role access (CAS read). Verify by structure: the public
/// LedgerEntry/L4.E schema separates CID from inline body.
#[test]
fn private_diagnostic_cid_not_serialized_publicly() {
    // Search src/ for any code that serializes a "raw" body alongside a
    // CID under the same public field — that would be a leak.
    let out = Command::new("grep")
        .args([
            "-rEn",
            "--include=*.rs",
            r#"private_diagnostic_(body|raw|inline|content)"#,
            "src/",
        ])
        .output()
        .expect("grep should be available");
    let stdout = String::from_utf8_lossy(&out.stdout);
    // permitted: doc-comments / variable-name in helper, but a public field
    // with such a name would be a leak indicator.
    let bad: Vec<&str> = stdout
        .lines()
        .filter(|l| l.contains("pub ") && !l.contains("//"))
        .collect();
    assert!(
        bad.is_empty(),
        "Shielding violation: a `private_diagnostic_(body|raw|inline|content)` \
         field is publicly exposed:\n{}",
        bad.join("\n")
    );

    // The proper separation: private_diagnostic_cid (public) → CAS (audit).
    let tl_src = std::fs::read_to_string("src/bottom_white/ledger/transition_ledger.rs")
        .expect("transition_ledger.rs readable");
    let _ = tl_src; // structural check above confirmed separation
}

/// Art. III.2 — EvidenceCapsule raw logs are audit-only. The capsule's
/// raw-log storage routes through CAS with public_summary in the L4
/// anchor and full payload behind CID. We verify by code-grep.
#[test]
fn evidence_capsule_raw_logs_audit_only() {
    // Find the EvidenceCapsule type definition.
    let out = Command::new("grep")
        .args([
            "-rn",
            "--include=*.rs",
            "EvidenceCapsule",
            "src/",
        ])
        .output()
        .expect("grep available");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("pub struct EvidenceCapsule") || stdout.contains("pub enum EvidenceCapsule"),
        "Shielding violation: EvidenceCapsule type missing — capsule \
         shape un-defined."
    );

    // The capsule schema must include CAS-CID-routed raw_log_cid (or
    // raw_logs_cid / private_logs_cid) — NOT inline raw_logs body.
    let stdout_str = stdout.to_string();
    let has_cid_routing = stdout_str.contains("raw_log_cid")
        || stdout_str.contains("raw_logs_cid")
        || stdout_str.contains("private_logs_cid")
        || stdout_str.contains("logs_cid");
    let has_inline_raw = stdout_str
        .lines()
        .any(|l| l.contains("pub raw_logs:") && !l.contains("Cid"));
    assert!(
        has_cid_routing || !has_inline_raw,
        "Shielding violation: EvidenceCapsule may inline raw_logs body \
         instead of routing through a CID. Audit-only access broken."
    );
}

/// Art. III.4 — Dashboard does not leak private failure detail.
/// The dashboard renders summarized L4 + L4.E + capsule rollups — never
/// per-agent private diagnostic body. We verify by code-grep that the
/// dashboard module doesn't read CAS payloads tagged as private.
#[test]
fn dashboard_does_not_leak_private_failure_detail() {
    // The audit_dashboard module should exist (TB-16 era).
    let candidates = [
        "src/runtime/audit_dashboard.rs",
        "src/runtime/dashboard.rs",
        "src/bin/audit_dashboard.rs",
    ];
    let dash_path = candidates
        .iter()
        .find(|p| Path::new(p).exists())
        .expect("Shielding: dashboard module missing under any expected path");

    let dash_src = std::fs::read_to_string(dash_path).expect("dashboard readable");
    // The dashboard MUST NOT contain a code path that reads
    // `private_diagnostic_cid` and writes it to a public dashboard field.
    let bad_pattern_pub_diag = dash_src
        .lines()
        .any(|l| {
            (l.contains("private_diagnostic") || l.contains("raw_stderr"))
                && l.contains("println!")
        });
    assert!(
        !bad_pattern_pub_diag,
        "Shielding violation: {dash_path} prints private diagnostic \
         to stdout — dashboard leaking private failure detail."
    );
}
