//! TRACE_MATRIX FC1 + FC2 + FC3: `turingos os` A13 network-off E2E commands.
//!
//! The command group is deliberately fixture-bound for A13. It proves the OS
//! loop shape and replay/audit closure without provider/network execution or
//! production settlement authority.

use std::path::PathBuf;
use std::process::ExitCode;

#[path = "../../runtime/os_run.rs"]
mod os_run;

/// TRACE_MATRIX FC1 + FC2: `os run` short-help
pub(crate) const RUN_SHORT_HELP: &str = "Run a network-off Agentic OS fixture";

/// TRACE_MATRIX FC2: `os replay` short-help
pub(crate) const REPLAY_SHORT_HELP: &str = "Replay and verify an Agentic OS run directory";

/// TRACE_MATRIX FC3: `os audit` short-help
pub(crate) const AUDIT_SHORT_HELP: &str = "Audit an Agentic OS run directory";

/// TRACE_MATRIX FC1 + FC2: `os run` full --help text
pub(crate) const RUN_FULL_HELP: &str = r#"turingos os run — network-off Agentic OS fixture

USAGE:
    turingos os run --task <PATH> --policy single_tree --market on --network off [--out-dir <PATH>]

DESCRIPTION:
    Creates a deterministic A13 run directory with:
      - run_manifest.json
      - git_tape_repo/
      - replay_report.json
      - predicate_receipts.jsonl
      - external_call_receipts.jsonl
      - economy_projection.json
      - agent_view_audit.json

    The only supported A13 execution mode is --network off. External calls are
    represented by a closed deterministic intent/terminal pair.

OPTIONS:
    --task <PATH>            Fixture task JSON.
    --policy single_tree     A13 single-tree scheduler fixture.
    --market on              Enable derived market projection.
    --network off            Disable provider/network execution.
    --out-dir <PATH>         Output run directory. Defaults under target/.
    -h, --help               Print this help.
"#;

/// TRACE_MATRIX FC2: `os replay` full --help text
pub(crate) const REPLAY_FULL_HELP: &str = r#"turingos os replay — verify an Agentic OS run

USAGE:
    turingos os replay --run-dir <PATH>

DESCRIPTION:
    Recomputes artifact hashes, verifies every derived artifact is watermarked
    with the run's GitTape HEAD, and checks that the GitTape repo HEAD matches
    run_manifest.json.
"#;

/// TRACE_MATRIX FC3: `os audit` full --help text
pub(crate) const AUDIT_FULL_HELP: &str = r#"turingos os audit — audit an Agentic OS run

USAGE:
    turingos os audit --run-dir <PATH>

DESCRIPTION:
    Runs replay verification, then checks the A13 acceptance predicates:
    integer money conservation, no pending external intents, no unsupported
    success claims, and no private fixture data in the agent read view.
"#;

/// TRACE_MATRIX FC1 + FC2: `os run` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{RUN_FULL_HELP}");
        return ExitCode::SUCCESS;
    }

    let request = match parse_run_args(args) {
        Ok(request) => request,
        Err(err) => {
            eprintln!("error: {err}");
            return ExitCode::from(2);
        }
    };

    match os_run::run_network_off_fixture(request) {
        Ok(summary) => {
            println!("OS-RUN-COMPLETE");
            println!("run_dir={}", summary.run_dir.display());
            println!("final_tape_head={}", summary.final_tape_head);
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::from(2)
        }
    }
}

/// TRACE_MATRIX FC2: `os replay` dispatch entry
pub(crate) fn replay(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{REPLAY_FULL_HELP}");
        return ExitCode::SUCCESS;
    }

    let run_dir = match parse_run_dir(args) {
        Ok(path) => path,
        Err(err) => {
            eprintln!("error: {err}");
            return ExitCode::from(2);
        }
    };
    match os_run::replay_run_dir(&run_dir) {
        Ok(summary) => {
            println!("OS-REPLAY-OK");
            println!("run_dir={}", run_dir.display());
            println!("final_tape_head={}", summary.final_tape_head);
            println!("verified_artifacts={}", summary.verified_artifacts);
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::from(1)
        }
    }
}

/// TRACE_MATRIX FC3: `os audit` dispatch entry
pub(crate) fn audit(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{AUDIT_FULL_HELP}");
        return ExitCode::SUCCESS;
    }

    let run_dir = match parse_run_dir(args) {
        Ok(path) => path,
        Err(err) => {
            eprintln!("error: {err}");
            return ExitCode::from(2);
        }
    };
    match os_run::audit_run_dir(&run_dir) {
        Ok(summary) => {
            println!("PREDICATES-GREEN");
            println!("run_dir={}", run_dir.display());
            println!("final_tape_head={}", summary.final_tape_head);
            println!("checked_predicates={}", summary.checked_predicates);
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::from(1)
        }
    }
}

fn parse_run_args(args: &[String]) -> Result<os_run::RunRequest, String> {
    let mut task: Option<PathBuf> = None;
    let mut policy: Option<String> = None;
    let mut market: Option<String> = None;
    let mut network: Option<String> = None;
    let mut out_dir: Option<PathBuf> = None;

    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--task" => task = Some(next_path(&mut iter, "--task")?),
            "--policy" => policy = Some(next_value(&mut iter, "--policy")?),
            "--market" => market = Some(next_value(&mut iter, "--market")?),
            "--network" => network = Some(next_value(&mut iter, "--network")?),
            "--out-dir" => out_dir = Some(next_path(&mut iter, "--out-dir")?),
            other => return Err(format!("unknown os run option `{other}`")),
        }
    }

    let request = os_run::RunRequest {
        task: task.ok_or_else(|| "--task <PATH> is required".to_string())?,
        policy: policy.ok_or_else(|| "--policy single_tree is required".to_string())?,
        market: market.ok_or_else(|| "--market on is required".to_string())?,
        network: network.ok_or_else(|| "--network off is required".to_string())?,
        out_dir,
    };
    request.validate()?;
    Ok(request)
}

fn parse_run_dir(args: &[String]) -> Result<PathBuf, String> {
    let mut run_dir: Option<PathBuf> = None;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--run-dir" => run_dir = Some(next_path(&mut iter, "--run-dir")?),
            other => return Err(format!("unknown option `{other}`")),
        }
    }
    run_dir.ok_or_else(|| "--run-dir <PATH> is required".to_string())
}

fn next_value<'a>(
    iter: &mut impl Iterator<Item = &'a String>,
    flag: &str,
) -> Result<String, String> {
    iter.next()
        .cloned()
        .ok_or_else(|| format!("{flag} requires a value"))
}

fn next_path<'a>(
    iter: &mut impl Iterator<Item = &'a String>,
    flag: &str,
) -> Result<PathBuf, String> {
    Ok(PathBuf::from(next_value(iter, flag)?))
}
