//! TB-6 Atom 6 — `gen_run_summary` CLI (thin wrapper around
//! `runtime::run_summary::RunSummary::from_chaintape`).
//!
//! Emits `run_summary.json` for any chain-backed evidence directory. The
//! production-binary path also writes one automatically at end-of-run via
//! the evaluator's chaintape exit hook; this CLI is the standalone backfill
//! / forensic re-derivation entry-point.
//!
//! Usage:
//!   gen_run_summary --repo <runtime_repo> --cas <cas> --run-id <id>
//!     [--failed-branch-count <n>] [--rollback-count <n>] [--out <path>]

use std::path::PathBuf;

use turingosv4::runtime::run_summary::RunSummary;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let parsed = match parse_args(&argv[1..]) {
        Ok(a) => a,
        Err(msg) => {
            eprintln!("gen_run_summary: {msg}");
            eprintln!(
                "usage: gen_run_summary --repo <runtime_repo> --cas <cas> --run-id <id> \
                 [--failed-branch-count <n>] [--rollback-count <n>] [--out <path>]"
            );
            std::process::exit(2);
        }
    };

    let summary = match RunSummary::from_chaintape(
        &parsed.repo,
        &parsed.cas,
        &parsed.run_id,
        parsed.failed_branch_count,
        parsed.rollback_count,
    ) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("gen_run_summary: build failed: {e}");
            std::process::exit(2);
        }
    };

    let json = match serde_json::to_string_pretty(&summary) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("gen_run_summary: serialize failed: {e}");
            std::process::exit(2);
        }
    };

    if let Some(out) = parsed.out.as_ref() {
        if let Err(e) = std::fs::write(out, &json) {
            eprintln!("gen_run_summary: write {out:?} failed: {e}");
            std::process::exit(2);
        }
    } else {
        println!("{json}");
    }
}

#[derive(Debug)]
struct Args {
    repo: PathBuf,
    cas: PathBuf,
    run_id: String,
    failed_branch_count: u64,
    rollback_count: u64,
    out: Option<PathBuf>,
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut repo: Option<PathBuf> = None;
    let mut cas: Option<PathBuf> = None;
    let mut run_id: Option<String> = None;
    let mut failed_branch_count: u64 = 0;
    let mut rollback_count: u64 = 0;
    let mut out: Option<PathBuf> = None;
    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "--repo" => {
                i += 1;
                repo = Some(argv.get(i).ok_or("missing value after --repo")?.into());
            }
            "--cas" => {
                i += 1;
                cas = Some(argv.get(i).ok_or("missing value after --cas")?.into());
            }
            "--run-id" => {
                i += 1;
                run_id = Some(argv.get(i).ok_or("missing value after --run-id")?.clone());
            }
            "--failed-branch-count" => {
                i += 1;
                failed_branch_count = argv
                    .get(i)
                    .ok_or("missing value after --failed-branch-count")?
                    .parse()
                    .map_err(|e: std::num::ParseIntError| e.to_string())?;
            }
            "--rollback-count" => {
                i += 1;
                rollback_count = argv
                    .get(i)
                    .ok_or("missing value after --rollback-count")?
                    .parse()
                    .map_err(|e: std::num::ParseIntError| e.to_string())?;
            }
            "--out" => {
                i += 1;
                out = Some(argv.get(i).ok_or("missing value after --out")?.into());
            }
            "--help" | "-h" => return Err("--help requested".into()),
            other => return Err(format!("unknown arg: {other}")),
        }
        i += 1;
    }
    Ok(Args {
        repo: repo.ok_or("--repo required")?,
        cas: cas.ok_or("--cas required")?,
        run_id: run_id.ok_or("--run-id required")?,
        failed_branch_count,
        rollback_count,
        out,
    })
}
