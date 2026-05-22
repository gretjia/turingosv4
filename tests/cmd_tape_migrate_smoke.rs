//! TRACE_MATRIX FC1a-substrate_seam + FC3-replay:
//! Atom 23 — KILL-migrate-1 smoke test for the tape-migrate subcommand.
//!
//! Verifies that exporting an evidence dir produces a GitTapeLedger repo
//! whose dump_all_nodes count matches the source's chaintape.jsonl line
//! count.
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_GENERATE_PHASE_E_DIRECTIVE_AND_§8.md

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use tempfile::TempDir;

use turingosv4::git_tape_ledger::GitTapeLedger;
use turingosv4::ledger::ImmutableTapeLedger;

fn write_fixture_chaintape(dir: &std::path::Path, lines: usize) -> PathBuf {
    fs::create_dir_all(dir).unwrap();
    let path = dir.join("chaintape.jsonl");
    let mut body = String::new();
    for i in 0..lines {
        let kind = if i == 0 { "AgentProposal" } else if i % 3 == 0 { "RetryBeliefState" } else { "AgentProposal" };
        let line = serde_json::json!({
            "hash": format!("h-{}", i),
            "kind": kind,
            "verified": false,
            "parent": null,
            "scope": {
                "run_id": "fixture-run",
                "task_id": "fixture-task",
                "verified_parent": "H0",
            },
            "attempt_ordinal": i,
            "reject_class": "fixture",
        });
        body.push_str(&line.to_string());
        body.push('\n');
    }
    fs::write(&path, body).unwrap();
    path
}

fn turingos_bin() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    // tests run from project root; binary is at target/{debug,release}/turingos
    // Prefer release if it exists (matches deployment); fall back to debug.
    let release = PathBuf::from(manifest_dir).join("target/release/turingos");
    let debug = PathBuf::from(manifest_dir).join("target/debug/turingos");
    if release.exists() {
        release
    } else {
        debug
    }
}

#[test]
fn kill_migrate_1_dump_count_matches_source_lines() {
    let src_dir = TempDir::new().unwrap();
    let target_dir = TempDir::new().unwrap();
    let src_path = src_dir.path().to_path_buf();
    let target_path = target_dir.path().join("migrated.git");

    write_fixture_chaintape(&src_path, 5);

    let bin = turingos_bin();
    if !bin.exists() {
        // Cannot run end-to-end without the binary built. Skip rather than
        // panic — the binary may be built by a higher-level test runner.
        eprintln!("turingos binary not found at {}; skipping", bin.display());
        return;
    }

    let out = Command::new(&bin)
        .arg("tape-migrate")
        .arg("export")
        .arg("--from")
        .arg(&src_path)
        .arg("--to")
        .arg(&target_path)
        .output()
        .expect("spawn turingos");
    assert!(
        out.status.success(),
        "turingos tape-migrate failed: stdout={:?} stderr={:?}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let ledger = GitTapeLedger::open(&target_path).expect("open migrated");
    let dump = ledger.dump_all_nodes();
    assert_eq!(dump.len(), 5, "dump_all_nodes should match source line count");
}
