//! REAL-14 independent E2 candidate verifier CLI.

use std::path::PathBuf;

use turingosv4::runtime::market_e2_candidate_verifier::{
    verify_market_e2_candidate, E2CandidateVerifierOptions, E2CandidateVerifierVerdict,
};

#[derive(Debug)]
struct Args {
    repo: PathBuf,
    cas: PathBuf,
    expect_count: Option<u64>,
    require_direct_prompt_capsule_provenance: bool,
    json_out: Option<PathBuf>,
    md_out: Option<PathBuf>,
}

fn parse_args() -> Result<Args, String> {
    let mut argv = std::env::args().skip(1).peekable();
    let mut repo = None;
    let mut cas = None;
    let mut expect_count = None;
    let mut require_direct_prompt_capsule_provenance = false;
    let mut json_out = None;
    let mut md_out = None;
    while let Some(arg) = argv.next() {
        match arg.as_str() {
            "--repo" => repo = Some(argv.next().ok_or("--repo requires a path")?.into()),
            "--cas" => cas = Some(argv.next().ok_or("--cas requires a path")?.into()),
            "--expect-count" => {
                let raw = argv.next().ok_or("--expect-count requires an integer")?;
                expect_count = Some(
                    raw.parse::<u64>()
                        .map_err(|e| format!("--expect-count parse failed: {e}"))?,
                );
            }
            "--require-direct-prompt-capsule-provenance" => {
                require_direct_prompt_capsule_provenance = true;
            }
            "--json-out" => {
                json_out = Some(argv.next().ok_or("--json-out requires a path")?.into())
            }
            "--md-out" => md_out = Some(argv.next().ok_or("--md-out requires a path")?.into()),
            "-h" | "--help" => return Err(help_text()),
            other => return Err(format!("unknown argument {other}\n\n{}", help_text())),
        }
    }
    Ok(Args {
        repo: repo.ok_or("--repo required")?,
        cas: cas.ok_or("--cas required")?,
        expect_count,
        require_direct_prompt_capsule_provenance,
        json_out,
        md_out,
    })
}

fn help_text() -> String {
    "real14_e2_candidate_verifier --repo <runtime_repo> --cas <cas> [--expect-count N] [--require-direct-prompt-capsule-provenance] [--json-out path] [--md-out path]".into()
}

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };
    let report = match verify_market_e2_candidate(
        &args.repo,
        &args.cas,
        E2CandidateVerifierOptions {
            expected_exact_join_count: args.expect_count,
            require_direct_prompt_capsule_provenance: args.require_direct_prompt_capsule_provenance,
            ..E2CandidateVerifierOptions::default()
        },
    ) {
        Ok(report) => report,
        Err(e) => {
            eprintln!("real14_e2_candidate_verifier: {e}");
            std::process::exit(2);
        }
    };
    let json = match serde_json::to_string_pretty(&report) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("real14_e2_candidate_verifier: JSON encode failed: {e}");
            std::process::exit(2);
        }
    };
    if let Some(path) = args.json_out {
        if let Err(e) = std::fs::write(&path, &json) {
            eprintln!("real14_e2_candidate_verifier: write {path:?}: {e}");
            std::process::exit(2);
        }
    } else {
        println!("{json}");
    }
    if let Some(path) = args.md_out {
        if let Err(e) = std::fs::write(&path, report.render_markdown()) {
            eprintln!("real14_e2_candidate_verifier: write {path:?}: {e}");
            std::process::exit(2);
        }
    }
    if report.verdict != E2CandidateVerifierVerdict::Proceed {
        std::process::exit(1);
    }
}
