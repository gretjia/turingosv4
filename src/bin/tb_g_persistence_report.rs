//! TB-G G1.2-6/7 (Option B+ orchestration ruling 2026-05-11 §3.4) —
//! `tb_g_persistence_report` evidence enricher.
//!
//! Reads a shipped batch evidence directory:
//!   <RUN_DIR>/runtime_repo/       (shared chain)
//!   <RUN_DIR>/cas/                (shared CAS)
//!   <RUN_DIR>/BatchContinuationManifest.json
//!
//! For each task boundary recorded in the manifest, replays the chain
//! via `replay_full_transition` (FC2-Boot canonical) to obtain the
//! post-task `QState`. Calls `bind_persistence` to classify each of
//! the six architect-required persisted fields (balances / positions /
//! reputation / PnL / autopsy / model identity) as
//! `Witnessed | Empty | Reset`. Writes the report as
//! `<RUN_DIR>/PERSISTENCE_BINDING_REPORT.json`.
//!
//! Codex G1.2-6 micro-audit Q6 closure: gives the auditor a CAS-derived
//! evidence artifact for the persistence-binding shape, instead of
//! "Reset/Witnessed/Empty verdicts are not evidenced".
//!
//! Usage:
//!
//!   tb_g_persistence_report --run-dir <path>
//!
//! Exit codes:
//!   0  report written; no Reset verdicts
//!   1  report written; at least one Reset verdict (kill-criterion #1)
//!   2  IO / replay / manifest error before report could be written

use std::path::PathBuf;
use std::process::ExitCode;

use turingosv4::runtime::batch_continuation_manifest::BatchContinuationManifest;
use turingosv4::runtime::persistence_evidence::{
    bind_persistence, replay_task_end_snapshots_from_disk,
};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let run_dir = match parse_argv(&args) {
        Ok(p) => p,
        Err(msg) => {
            eprintln!("{msg}");
            return ExitCode::from(2);
        }
    };

    let runtime_repo = run_dir.join("runtime_repo");
    let cas_path = run_dir.join("cas");
    let manifest_path = run_dir.join("BatchContinuationManifest.json");
    let out_path = run_dir.join("PERSISTENCE_BINDING_REPORT.json");

    let manifest_json = match std::fs::read_to_string(&manifest_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("tb_g_persistence_report: read {manifest_path:?}: {e}");
            return ExitCode::from(2);
        }
    };
    let manifest: BatchContinuationManifest = match serde_json::from_str(&manifest_json) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("tb_g_persistence_report: parse manifest: {e}");
            return ExitCode::from(2);
        }
    };

    let (initial_q, snapshots) =
        match replay_task_end_snapshots_from_disk(&runtime_repo, &cas_path, &manifest) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("tb_g_persistence_report: replay failed: {e}");
                return ExitCode::from(2);
            }
        };

    let report = bind_persistence(&initial_q, &snapshots, &manifest);

    let json = match serde_json::to_string_pretty(&report) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("tb_g_persistence_report: serialize report: {e}");
            return ExitCode::from(2);
        }
    };
    if let Err(e) = std::fs::write(&out_path, json) {
        eprintln!("tb_g_persistence_report: write {out_path:?}: {e}");
        return ExitCode::from(2);
    }

    println!(
        "tb_g_persistence_report: wrote {out_path:?} n_tasks={} n_witnessed={} is_passing={}",
        report.n_tasks,
        report.n_witnessed(),
        report.is_passing()
    );

    if report.is_passing() {
        ExitCode::from(0)
    } else {
        eprintln!("tb_g_persistence_report: FAIL — at least one Reset verdict (kill-criterion #1)");
        ExitCode::from(1)
    }
}

fn parse_argv(args: &[String]) -> Result<PathBuf, String> {
    let mut run_dir: Option<PathBuf> = None;
    let mut iter = args.iter().skip(1);
    while let Some(a) = iter.next() {
        match a.as_str() {
            "--run-dir" => {
                run_dir = iter.next().cloned().map(PathBuf::from);
            }
            other => return Err(format!("tb_g_persistence_report: unknown arg {other}")),
        }
    }
    run_dir.ok_or_else(|| "tb_g_persistence_report: --run-dir required".to_string())
}
