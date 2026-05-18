//! REAL-15 role differentiation verifier CLI.

use std::path::PathBuf;

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::market_e2_candidate_verifier::E2CandidateVerifierReport;
use turingosv4::runtime::role_differentiation::{
    summarize_role_differentiation_from_runs, RoleDifferentiationRunInput,
    RoleDifferentiationVerdict,
};

#[derive(Debug)]
struct Args {
    run_ids: Vec<String>,
    cas_paths: Vec<PathBuf>,
    e2_reports: Vec<PathBuf>,
    audit_tape_proceeds: Vec<bool>,
    json_out: Option<PathBuf>,
    md_out: Option<PathBuf>,
}

fn parse_args() -> Result<Args, String> {
    let mut argv = std::env::args().skip(1).peekable();
    let mut run_ids = Vec::new();
    let mut cas_paths = Vec::new();
    let mut e2_reports = Vec::new();
    let mut audit_tape_proceeds = Vec::new();
    let mut json_out = None;
    let mut md_out = None;

    while let Some(arg) = argv.next() {
        match arg.as_str() {
            "--run-id" => run_ids.push(argv.next().ok_or("--run-id requires a value")?),
            "--cas" => cas_paths.push(argv.next().ok_or("--cas requires a path")?.into()),
            "--e2-report" => {
                e2_reports.push(argv.next().ok_or("--e2-report requires a path")?.into())
            }
            "--audit-tape-proceed" => {
                let raw = argv
                    .next()
                    .ok_or("--audit-tape-proceed requires true|false")?;
                audit_tape_proceeds.push(match raw.as_str() {
                    "true" => true,
                    "false" => false,
                    other => {
                        return Err(format!(
                            "--audit-tape-proceed must be true|false, got {other}"
                        ))
                    }
                });
            }
            "--json-out" => {
                json_out = Some(argv.next().ok_or("--json-out requires a path")?.into())
            }
            "--md-out" => md_out = Some(argv.next().ok_or("--md-out requires a path")?.into()),
            "-h" | "--help" => return Err(help_text()),
            other => return Err(format!("unknown argument {other}\n\n{}", help_text())),
        }
    }

    let n = run_ids.len();
    if n == 0 {
        return Err("--run-id/--cas/--e2-report/--audit-tape-proceed group required".into());
    }
    if cas_paths.len() != n || e2_reports.len() != n || audit_tape_proceeds.len() != n {
        return Err(format!(
            "mismatched run groups: run_id={} cas={} e2_report={} audit={}",
            n,
            cas_paths.len(),
            e2_reports.len(),
            audit_tape_proceeds.len()
        ));
    }

    Ok(Args {
        run_ids,
        cas_paths,
        e2_reports,
        audit_tape_proceeds,
        json_out,
        md_out,
    })
}

fn help_text() -> String {
    "real15_role_differentiation_verifier --run-id <id> --cas <cas> --e2-report <json> --audit-tape-proceed true|false [repeat group] [--json-out path] [--md-out path]".into()
}

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };

    let mut cas_stores = Vec::new();
    let mut e2_reports = Vec::new();
    for (idx, cas_path) in args.cas_paths.iter().enumerate() {
        let cas = match CasStore::open(cas_path) {
            Ok(cas) => cas,
            Err(e) => {
                eprintln!("real15_role_differentiation_verifier: open CAS {cas_path:?}: {e}");
                std::process::exit(2);
            }
        };
        let report_bytes = match std::fs::read(&args.e2_reports[idx]) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!(
                    "real15_role_differentiation_verifier: read {:?}: {e}",
                    args.e2_reports[idx]
                );
                std::process::exit(2);
            }
        };
        let e2_report: E2CandidateVerifierReport = match serde_json::from_slice(&report_bytes) {
            Ok(report) => report,
            Err(e) => {
                eprintln!(
                    "real15_role_differentiation_verifier: decode {:?}: {e}",
                    args.e2_reports[idx]
                );
                std::process::exit(2);
            }
        };
        cas_stores.push(cas);
        e2_reports.push(e2_report);
    }

    let inputs: Vec<_> = args
        .run_ids
        .iter()
        .enumerate()
        .map(|(idx, run_id)| {
            RoleDifferentiationRunInput::new(
                run_id,
                &cas_stores[idx],
                &e2_reports[idx],
                args.audit_tape_proceeds[idx],
            )
        })
        .collect();
    let report = match summarize_role_differentiation_from_runs(&inputs) {
        Ok(report) => report,
        Err(e) => {
            eprintln!("real15_role_differentiation_verifier: {e}");
            std::process::exit(2);
        }
    };

    let json = match serde_json::to_string_pretty(&report) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("real15_role_differentiation_verifier: JSON encode failed: {e}");
            std::process::exit(2);
        }
    };
    if let Some(path) = args.json_out {
        if let Err(e) = std::fs::write(&path, &json) {
            eprintln!("real15_role_differentiation_verifier: write {path:?}: {e}");
            std::process::exit(2);
        }
    } else {
        println!("{json}");
    }
    if let Some(path) = args.md_out {
        if let Err(e) = std::fs::write(&path, report.render_markdown()) {
            eprintln!("real15_role_differentiation_verifier: write {path:?}: {e}");
            std::process::exit(2);
        }
    }

    if report.verdict == RoleDifferentiationVerdict::Veto {
        std::process::exit(1);
    }
}
