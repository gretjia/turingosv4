//! TB-6 Atom 4 — `verify_chaintape` CLI (thin wrapper around `runtime::verify`).
//!
//! Architect ruling 2026-05-01 § 3.6 Atom 4 deliverable. Re-opens a
//! `runtime_repo` + `cas_path` produced by Atom 1's bootstrap factory, replays
//! the L4 chain through `replay_full_transition` (the I-DETHASH witness),
//! verifies every entry's `system_signature` against the persisted
//! `pinned_pubkeys.json`, and emits a `replay_report.json` to stdout (or to
//! `--out <path>` if provided).
//!
//! Usage:
//!   verify_chaintape --repo <runtime_repo_path> --cas <cas_path> [--run-id <id>] [--out <path>]
//!
//! Exit code:
//!   0  — every architect-mandated boolean indicator passed.
//!   1  — at least one indicator failed (replay_report.json still emitted).
//!   2  — verifier could not start (manifest missing / CAS unreadable / I/O).

use std::path::PathBuf;

use turingosv4::runtime::verify::{verify_chaintape, VerifyOptions};

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let parsed = match parse_args(&argv[1..]) {
        Ok(a) => a,
        Err(msg) => {
            eprintln!("verify_chaintape: {msg}");
            eprintln!(
                "usage: verify_chaintape --repo <runtime_repo_path> --cas <cas_path> \
                 [--run-id <id>] [--out <path>]"
            );
            std::process::exit(2);
        }
    };

    let opts = VerifyOptions {
        expected_run_id: parsed.run_id.clone(),
    };

    let report = match verify_chaintape(&parsed.repo, &parsed.cas, &opts) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("verify_chaintape: bootstrap failed: {e}");
            std::process::exit(2);
        }
    };

    let json = serde_json::to_string_pretty(&report).unwrap_or_else(|e| {
        eprintln!("verify_chaintape: serialize report failed: {e}");
        std::process::exit(2);
    });

    if let Some(out) = parsed.out.as_ref() {
        if let Err(e) = std::fs::write(out, &json) {
            eprintln!("verify_chaintape: write {out:?} failed: {e}");
            std::process::exit(2);
        }
    } else {
        println!("{json}");
    }

    std::process::exit(if report.all_indicators_pass() { 0 } else { 1 });
}

#[derive(Debug)]
struct Args {
    repo: PathBuf,
    cas: PathBuf,
    run_id: Option<String>,
    out: Option<PathBuf>,
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut repo: Option<PathBuf> = None;
    let mut cas: Option<PathBuf> = None;
    let mut run_id: Option<String> = None;
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
            "--out" => {
                i += 1;
                out = Some(argv.get(i).ok_or("missing value after --out")?.into());
            }
            "--help" | "-h" => {
                return Err("--help requested".into());
            }
            other => return Err(format!("unknown arg: {other}")),
        }
        i += 1;
    }
    Ok(Args {
        repo: repo.ok_or("--repo required")?,
        cas: cas.ok_or("--cas required")?,
        run_id,
        out,
    })
}
