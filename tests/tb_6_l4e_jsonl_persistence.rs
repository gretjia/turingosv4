//! TB-6 Atom 1.2 — `RejectionEvidenceWriter` JSONL persistence integration tests.
//!
//! 5 tests T_R1-T_R5 verify the JSONL backend per preflight v2.1 §6.1 + Codex
//! round-1 F1 remediation.
//!
//! Charter: `handover/tracer_bullets/TB-6_charter_2026-05-01.md`.
//! Preflight: `handover/ai-direct/TB-6_PRODUCTION_CHAINTAPE_BOOTSTRAP_2026-05-01.md` v2.1.

use std::io::Write;

use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    RejectionClass, RejectionEvidenceError, RejectionEvidenceWriter,
};
use turingosv4::bottom_white::ledger::transition_ledger::TxKind;
use turingosv4::state::q_state::{AgentId, Hash};

fn synthetic_cid(byte: u8) -> Cid {
    Cid([byte; 32])
}

fn synthetic_hash(byte: u8) -> Hash {
    Hash([byte; 32])
}

fn append_synthetic_record(writer: &mut RejectionEvidenceWriter, submit_id: u64) -> Hash {
    writer.append_rejected(
        submit_id,
        synthetic_hash(0xAA),
        AgentId(format!("agent-{submit_id}")),
        TxKind::Work,
        synthetic_cid(0xBB),
        RejectionClass::PredicateFailed,
        Some(synthetic_cid(0xCC)),
        Some(format!("synthetic public summary {submit_id}")),
    )
}

#[test]
fn t_r1_open_jsonl_creates_empty_file_when_path_missing() {
    let tmp = TempDir::new().expect("tempdir");
    let path = tmp.path().join("nested").join("rejections.jsonl");
    assert!(!path.exists(), "precondition: path does not exist yet");
    let writer = RejectionEvidenceWriter::open_jsonl(path.clone()).expect("open_jsonl");
    assert!(path.exists(), "open_jsonl creates the file (and parent dirs)");
    assert_eq!(writer.len(), 0, "fresh writer has zero records");
    assert!(writer.is_jsonl_backed(), "backend is JsonlAppend");
    let contents = std::fs::read_to_string(&path).expect("read file");
    assert!(contents.is_empty(), "freshly-created file is empty");
}

#[test]
fn t_r2_append_rejected_persists_jsonl_line_and_in_memory_record() {
    let tmp = TempDir::new().expect("tempdir");
    let path = tmp.path().join("rejections.jsonl");
    let mut writer = RejectionEvidenceWriter::open_jsonl(path.clone()).expect("open_jsonl");

    let h1 = append_synthetic_record(&mut writer, 1);
    let h2 = append_synthetic_record(&mut writer, 2);

    // In-memory: chain hashes computed.
    assert_eq!(writer.len(), 2);
    assert_ne!(h1, Hash::ZERO);
    assert_ne!(h2, h1);

    // On-disk: 2 JSONL lines + can re-parse.
    let contents = std::fs::read_to_string(&path).expect("read file");
    let lines: Vec<&str> = contents.lines().collect();
    assert_eq!(lines.len(), 2, "two appended records → two JSONL lines");

    // Each line is valid JSON.
    for line in &lines {
        let _: serde_json::Value =
            serde_json::from_str(line).expect("each line is valid JSON");
    }

    // verify_chain succeeds on the in-memory + on-disk state.
    writer.verify_chain().expect("chain verifies");
}

#[test]
fn t_r3_reopen_jsonl_replays_existing_records_into_in_memory_chain() {
    let tmp = TempDir::new().expect("tempdir");
    let path = tmp.path().join("rejections.jsonl");

    // First open: write 3 records.
    {
        let mut writer = RejectionEvidenceWriter::open_jsonl(path.clone()).expect("open 1");
        append_synthetic_record(&mut writer, 1);
        append_synthetic_record(&mut writer, 2);
        append_synthetic_record(&mut writer, 3);
        assert_eq!(writer.len(), 3);
    } // writer dropped; file flushed.

    // Reopen: records should replay into the in-memory chain.
    let writer = RejectionEvidenceWriter::open_jsonl(path.clone()).expect("open 2 reopen");
    assert_eq!(writer.len(), 3, "reopen replays all 3 records");
    let recs = writer.records();
    assert_eq!(recs[0].submit_id, 1);
    assert_eq!(recs[1].submit_id, 2);
    assert_eq!(recs[2].submit_id, 3);
    // prev_hash chain is intact.
    assert_eq!(recs[0].prev_hash, Hash::ZERO);
    assert_eq!(recs[1].prev_hash, recs[0].hash);
    assert_eq!(recs[2].prev_hash, recs[1].hash);
    writer.verify_chain().expect("reopen chain verifies");
}

#[test]
fn t_r4_tampering_with_jsonl_line_fails_verify_chain_on_reopen() {
    let tmp = TempDir::new().expect("tempdir");
    let path = tmp.path().join("rejections.jsonl");

    // Write 2 records.
    {
        let mut writer = RejectionEvidenceWriter::open_jsonl(path.clone()).expect("open 1");
        append_synthetic_record(&mut writer, 1);
        append_synthetic_record(&mut writer, 2);
    }

    // Tamper: load JSONL, mutate first line's `submit_id`, write back.
    let contents = std::fs::read_to_string(&path).expect("read");
    let mut lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    // Replace `"submit_id":1,` with `"submit_id":99,` in the first line.
    lines[0] = lines[0].replace("\"submit_id\":1,", "\"submit_id\":99,");
    let tampered = lines.join("\n") + "\n";
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .expect("open write");
    f.write_all(tampered.as_bytes()).expect("write tampered");
    f.sync_data().expect("sync");
    drop(f);

    // Reopen MUST fail verify_chain (because the tampered line's hash no
    // longer matches its embedded `hash` field).
    let result = RejectionEvidenceWriter::open_jsonl(path);
    match result {
        Err(RejectionEvidenceError::HashMismatch { at }) => {
            assert_eq!(at, 0, "tampered first line breaks at index 0");
        }
        other => panic!("expected HashMismatch error; got {other:?}"),
    }
}

#[test]
fn t_r5_open_jsonl_default_writer_is_in_memory_backend() {
    // Default RejectionEvidenceWriter::new() has InMemory backend (no I/O).
    // Pre-existing TB-1 fixtures across the suite rely on this default
    // behavior; T_R5 documents the structural invariant.
    let writer = RejectionEvidenceWriter::new();
    assert!(!writer.is_jsonl_backed(), "default backend is InMemory");
    assert_eq!(writer.len(), 0);

    // open_jsonl with a fresh path → JSONL-backed.
    let tmp = TempDir::new().expect("tempdir");
    let path = tmp.path().join("rejections.jsonl");
    let writer_jsonl =
        RejectionEvidenceWriter::open_jsonl(path).expect("open_jsonl");
    assert!(writer_jsonl.is_jsonl_backed(), "open_jsonl backend is JsonlAppend");

    // Single-writer expectation: the writer takes ownership of the JSONL path
    // for the duration of its lifetime. Concurrent writers to the same path
    // are NOT supported; callers must ensure single-writer-per-path discipline
    // (Sequencer + ChaintapeBundle is the canonical owner). This test
    // documents the contract; the runtime owner enforces it.
}
