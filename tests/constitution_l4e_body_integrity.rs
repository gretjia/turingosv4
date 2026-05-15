//! Constitution gate — L4.E body integrity verification (assertion #51).
//!
//! Authority: closes the forward gap documented at
//! `tests/constitution_audit_tamper_3_of_3.rs::l4_refs_is_strict_subset_of_chain_refs_excluding_l4e`
//! (session #33 close: "audit does not deep-verify L4.E rejection_record
//! bodies, so tampering an L4.E blob is silent at audit-time").
//! Per `feedback_no_workarounds_strict_constitution` ("我不要凑活" /
//! 2026-05-10 user verbatim "我需要的是宪法约定的内容全部真实落地且可被验证"):
//! a constitutionally-undetectable tamper class is a constitutional
//! violation, not a deferred forward item.
//!
//! Stage A3 / HEAD_t C2 R3.5 introduced a dual-write for L4.E rejection
//! evidence:
//!   1. `runtime_repo/rejections.jsonl`            — JSONL chain (verified
//!                                                    at load via
//!                                                    `RejectionEvidenceWriter::open_jsonl::verify_chain`).
//!   2. `runtime_repo/.git/refs/chaintape/l4e`     — git-backed commit
//!                                                    chain (per
//!                                                    `rejection_evidence::advance_l4e_ref_for_record`).
//!
//! Pre-this-gate, only side (1) was verified. Side (2) was a "best-effort
//! attestation" — tampering reachable blobs or rewriting the ref was
//! silent. `assert_51_l4e_git_attestation_matches_jsonl` (session #34)
//! walks side (2), parses each commit's `rejection_record` blob, and
//! cross-checks against side (1). This gate is the executable face of
//! that closure.
//!
//! Real-evidence binding: every test below uses M0 P01 (2026-05-10 batch
//! at `handover/evidence/m0_minif2f_harness_audit_2026-05-10_post_stage_c/`)
//! as the source tape — it has 2 L4.E records + 2 git-side commits, and
//! its baseline audit verdict is PROCEED. Per
//! `feedback_real_problems_not_designed`: real-problem evidence preferred
//! over synthetic.
//!
//! Per `feedback_pre_runner_checklist` (evidence immutability): every
//! tamper test snapshots the source evidence to a tempdir before mutating.
//!
//! `FC-trace: FC1-N34 + FC1-N35 + FC2-INV1 + Art-0.4 + architect §B.9.3`.

use std::path::{Path, PathBuf};

use tempfile::TempDir;
use turingosv4::bottom_white::ledger::rejection_evidence::parse_and_verify_jsonl_record_bytes;
use turingosv4::runtime::audit_assertions::{
    assert_51_l4e_git_attestation_matches_jsonl, load_tape, AssertionVerdict, AuditInputs,
};
use turingosv4::runtime::audit_tamper::flip_largest_reachable_l4e_blob;

const SRC_PROBLEM_DIR: &str =
    "handover/evidence/m0_minif2f_harness_audit_2026-05-10_post_stage_c/P01_mathd_algebra_107";
const SRC_CONSTITUTION: &str = "constitution.md";

// ── Snapshot helpers ────────────────────────────────────────────────────────

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else {
            std::fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

/// Snapshot M0 P01 evidence (`runtime_repo/` + `cas/` + the constitution
/// file) into a tempdir. Returns (tempdir, AuditInputs) ready for
/// `load_tape` + assertion calls. The tempdir is RAII — cleaned on drop.
fn snapshot_m0_p01() -> (TempDir, AuditInputs) {
    let src_problem = PathBuf::from(SRC_PROBLEM_DIR);
    assert!(
        src_problem.exists(),
        "M0 P01 evidence missing at {SRC_PROBLEM_DIR}; gate test requires \
         the post-Stage-C M0 batch to be present in handover/evidence/. \
         Re-run scripts/m0_*.sh or unblock test by running M0 batch."
    );
    let src_runtime = src_problem.join("runtime_repo");
    let src_cas = src_problem.join("cas");
    assert!(
        src_runtime.is_dir(),
        "missing src runtime_repo {src_runtime:?}"
    );
    assert!(src_cas.is_dir(), "missing src cas {src_cas:?}");

    let tmp = TempDir::new().expect("tempdir");
    let dst_runtime = tmp.path().join("runtime_repo");
    let dst_cas = tmp.path().join("cas");
    copy_dir_recursive(&src_runtime, &dst_runtime).expect("copy runtime_repo");
    copy_dir_recursive(&src_cas, &dst_cas).expect("copy cas");

    let inputs = AuditInputs {
        runtime_repo: dst_runtime.clone(),
        cas_dir: dst_cas,
        agent_pubkeys: dst_runtime.join("agent_pubkeys.json"),
        pinned_pubkeys: dst_runtime.join("pinned_pubkeys.json"),
        genesis: dst_runtime.join("genesis_report.json"),
        constitution: PathBuf::from(SRC_CONSTITUTION),
        markov_pointer: None,
        alignment_dir: None,
    };
    (tmp, inputs)
}

// ── Positive control ────────────────────────────────────────────────────────

/// Untampered M0 P01 → assertion #51 PASSes. This is the baseline that
/// every tamper variant degrades from. If this fails, the assertion has
/// a false-positive bug or M0 P01 evidence has drifted.
#[test]
fn assert_51_pass_on_untampered_m0_p01() {
    let (_tmp, inputs) = snapshot_m0_p01();
    let tape = load_tape(&inputs).expect("load_tape on untampered M0 P01");
    let result = assert_51_l4e_git_attestation_matches_jsonl(&inputs, &tape);
    assert_eq!(
        result.id, 51,
        "assertion id must be 51 (Layer B; session #34 L4.E body integrity)"
    );
    assert!(
        matches!(result.result, AssertionVerdict::Pass),
        "assertion #51 must PASS on untampered M0 P01 evidence; got {:?} detail={:?}",
        result.result,
        result.detail
    );
}

// ── Tamper variants — each must HALT (not Pass) ────────────────────────────

/// Tamper a byte in the largest L4.E-reachable loose object (the
/// rejection_record blob) → assertion #51 must HALT (not silently PASS).
/// This is the constitutional core: pre-this-gate, this tamper was the
/// "silent at audit-time" case from the session #33 forward gap.
#[test]
fn assert_51_halts_on_l4e_blob_byte_flip() {
    let (_tmp, inputs) = snapshot_m0_p01();
    let detail = flip_largest_reachable_l4e_blob(&inputs.runtime_repo)
        .expect("flip_largest_reachable_l4e_blob must succeed on M0 P01");
    eprintln!("tamper detail: {detail}");

    let tape = load_tape(&inputs).expect(
        "load_tape must succeed on a tape with tampered L4.E git-side blob \
         (load_tape only reads the JSONL side, not the git-side blobs; \
         that's the gap we're closing — the audit must catch it)",
    );
    let result = assert_51_l4e_git_attestation_matches_jsonl(&inputs, &tape);
    assert!(
        matches!(result.result, AssertionVerdict::Halt),
        "assertion #51 MUST HALT on L4.E blob tampering (the canonical \
         pre-this-gate silent case); got {:?} detail={:?}",
        result.result,
        result.detail
    );
    let d = result.detail.unwrap_or_default();
    assert!(
        d.contains("L4.E position")
            || d.contains("zlib")
            || d.contains("peel_to_blob")
            || d.contains("body tampering")
            || d.contains("HashMismatch"),
        "halt detail should reference L4.E position / zlib / blob / hash \
         mismatch — got: {d}"
    );
}

/// Corrupt the `refs/chaintape/l4e` ref file directly (truncate to N-1
/// hex chars + zero-pad) → assertion #51 must HALT. Catches ref-rewrite
/// tampering even when the underlying blobs are intact.
#[test]
fn assert_51_halts_on_l4e_ref_corruption() {
    let (_tmp, inputs) = snapshot_m0_p01();
    let ref_path = inputs.runtime_repo.join(".git/refs/chaintape/l4e");
    assert!(
        ref_path.exists(),
        "M0 P01 must have refs/chaintape/l4e ref present"
    );
    let original = std::fs::read_to_string(&ref_path).expect("read l4e ref");
    let trimmed = original.trim();
    assert_eq!(trimmed.len(), 40, "git OID must be 40 hex chars");
    // Replace last 4 hex chars with zeros — points at non-existent commit.
    let mut bytes: Vec<char> = trimmed.chars().collect();
    let n = bytes.len();
    for i in (n - 4)..n {
        bytes[i] = '0';
    }
    let corrupt: String = bytes.into_iter().collect();
    std::fs::write(&ref_path, format!("{corrupt}\n")).expect("write corrupted ref");

    let tape = load_tape(&inputs).expect("load_tape unaffected by L4.E ref corruption");
    let result = assert_51_l4e_git_attestation_matches_jsonl(&inputs, &tape);
    assert!(
        matches!(result.result, AssertionVerdict::Halt),
        "assertion #51 MUST HALT on refs/chaintape/l4e corruption; got \
         {:?} detail={:?}",
        result.result,
        result.detail
    );
}

/// Delete `refs/chaintape/l4e` from a tape that has JSONL records → must
/// SKIP with a "pre-A3 JSONL-only mode" detail. This is the legitimate
/// pre-Stage-A3 evidence shape and must NOT be mistaken for tampering.
/// Validates the assertion's three-way (Pass / Halt / Skipped) routing.
#[test]
fn assert_51_skipped_on_pre_a3_jsonl_only_mode() {
    let (_tmp, inputs) = snapshot_m0_p01();
    let ref_path = inputs.runtime_repo.join(".git/refs/chaintape/l4e");
    std::fs::remove_file(&ref_path).expect("remove l4e ref");

    let tape = load_tape(&inputs).expect("load_tape on JSONL-only mode");
    assert!(
        tape.l4e_writer.len() > 0,
        "fixture invariant: JSONL must still have records after l4e ref removal"
    );
    let result = assert_51_l4e_git_attestation_matches_jsonl(&inputs, &tape);
    assert!(
        matches!(result.result, AssertionVerdict::Skipped),
        "assertion #51 must SKIP (not HALT) when refs/chaintape/l4e is \
         absent + JSONL has records — this is the legitimate pre-A3 / \
         FR-A3-HEAD-T-C2.6 backward-compat shape; got {:?} detail={:?}",
        result.result,
        result.detail
    );
    let d = result.detail.unwrap_or_default();
    assert!(
        d.contains("pre-A3") || d.contains("JSONL-only"),
        "skipped detail should explain pre-A3 mode; got: {d}"
    );
}

// ── parse_and_verify_jsonl_record_bytes self-tests ─────────────────────────

/// Helper round-trips bit-perfectly on the M0 P01 first JSONL line.
#[test]
fn parse_and_verify_helper_passes_on_valid_m0_p01_record() {
    let jsonl = std::fs::read_to_string(format!("{SRC_PROBLEM_DIR}/runtime_repo/rejections.jsonl"))
        .expect("read M0 P01 rejections.jsonl");
    let first_line = jsonl.lines().next().expect("at least one record");
    let parsed = parse_and_verify_jsonl_record_bytes(first_line.as_bytes())
        .expect("untampered record must parse + verify");
    assert!(
        parsed.submit_id > 0,
        "submit_id should be positive on real M0 P01 record (got {})",
        parsed.submit_id
    );
}

/// Flipping a single byte in the JSON content (without updating the
/// embedded `hash` field) makes the helper return HashMismatch.
#[test]
fn parse_and_verify_helper_rejects_tampered_field() {
    let jsonl = std::fs::read_to_string(format!("{SRC_PROBLEM_DIR}/runtime_repo/rejections.jsonl"))
        .expect("read M0 P01 rejections.jsonl");
    let first_line = jsonl.lines().next().expect("at least one record");
    // Replace the agent_id substring "tb6-smoke-sponsor" with a flipped
    // variant. compute_hash includes agent_id, so the embedded hash will
    // no longer recompute → HashMismatch.
    let tampered = first_line.replacen("tb6-smoke-sponsor", "tb6-smoke-TAMPER", 1);
    assert_ne!(tampered, first_line, "tamper must change the line");
    let err = parse_and_verify_jsonl_record_bytes(tampered.as_bytes())
        .expect_err("tampered record must NOT parse-and-verify");
    let s = format!("{err}");
    assert!(
        s.contains("chain break") || s.contains("HashMismatch") || s.contains("at 0"),
        "error should indicate hash mismatch; got: {s}"
    );
}

/// Garbage bytes (not JSON) → JsonlParse error, not panic.
#[test]
fn parse_and_verify_helper_rejects_garbage_bytes() {
    let err = parse_and_verify_jsonl_record_bytes(b"not even close to json}{")
        .expect_err("garbage must NOT parse-and-verify");
    let s = format!("{err}");
    assert!(
        s.contains("parse failure") || s.contains("JSONL") || s.contains("expected"),
        "error should indicate JSONL parse failure; got: {s}"
    );
}
