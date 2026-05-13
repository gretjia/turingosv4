use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use turingosv4::runtime::dev_harness::{
    close_run, open_run, read_manifest, record_audit, record_command, record_diff_text,
    summarize_run, validate_run, DevHarnessError, DevOpenRequest,
};

fn main() -> ExitCode {
    match real_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(2)
        }
    }
}

fn real_main() -> Result<(), DevHarnessError> {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some((subcommand, rest)) = args.split_first() else {
        return Err(directive_error(
            "missing subcommand",
            "Use: turingos_dev open|record-diff|record-command|record-audit|validate|close|summarize ...",
        ));
    };

    match subcommand.as_str() {
        "open" => cmd_open(rest),
        "record-diff" => cmd_record_diff(rest),
        "record-command" => cmd_record_command(rest),
        "record-audit" => cmd_record_audit(rest),
        "validate" => cmd_validate(rest),
        "close" => cmd_close(rest),
        "summarize" => cmd_summarize(rest),
        other => Err(directive_error(
            "unknown subcommand",
            &format!(
                "Unknown `{other}`. Use one of: open, record-diff, record-command, record-audit, validate, close, summarize."
            ),
        )),
    }
}

fn cmd_open(args: &[String]) -> Result<(), DevHarnessError> {
    let title = required_flag(args, "--title", "turingos_dev open --title <title> ...")?;
    let module = required_flag(args, "--module", "turingos_dev open --module Harness ...")?;
    let risk = required_flag(args, "--risk", "turingos_dev open --risk 0..4 ...")?
        .parse::<u8>()
        .map_err(|_| directive_error("--risk is not an integer", "Use --risk 0, 1, 2, 3, or 4."))?;
    let fc_nodes = split_csv(&required_flag(
        args,
        "--fc",
        "Map the task to constitution flowchart nodes, e.g. --fc FC3-N33,FC3-N43.",
    )?);
    let allowed_paths = split_csv(&required_flag(
        args,
        "--allowed",
        "Declare allowed write paths, e.g. --allowed HARNESS.md,src/runtime/dev_harness.rs.",
    )?);
    let molecule_or_atom = optional_flag(args, "--unit").unwrap_or_else(|| "molecule".to_string());
    let acceptance_commands = optional_flag(args, "--accept")
        .map(|v| split_csv(&v))
        .unwrap_or_else(|| vec!["cargo check".to_string()]);
    let evidence_root = optional_flag(args, "--evidence-root")
        .map(PathBuf::from)
        .unwrap_or_else(default_evidence_root);

    let run = open_run(DevOpenRequest {
        evidence_root,
        title,
        module,
        molecule_or_atom,
        requested_risk_class: risk,
        fc_nodes,
        allowed_paths,
        acceptance_commands,
        human_intent: optional_flag(args, "--intent"),
        ratification: optional_flag(args, "--ratification"),
        git_head: current_git_head(),
    })?;
    println!(
        "{}",
        serde_json::json!({
            "run_id": run.run_id,
            "run_dir": run.run_dir,
        })
    );
    Ok(())
}

fn cmd_record_diff(args: &[String]) -> Result<(), DevHarnessError> {
    let run_dir = resolve_run_dir(args)?;
    let manifest = read_manifest(&run_dir)?;
    let diff = current_diff_for_allowed_paths(&manifest.allowed_paths)?;
    let artifact = record_diff_text(&run_dir, &diff)?;
    println!("{}", serde_json::to_string_pretty(&artifact)?);
    Ok(())
}

fn cmd_record_command(args: &[String]) -> Result<(), DevHarnessError> {
    let run_dir = resolve_run_dir(args)?;
    let split = args.iter().position(|arg| arg == "--").ok_or_else(|| {
        directive_error(
            "record-command missing command separator",
            "Use: turingos_dev record-command --run <run_id> -- cargo check",
        )
    })?;
    let command: Vec<&str> = args[split + 1..].iter().map(String::as_str).collect();
    let evidence = record_command(&run_dir, &command)?;
    println!("{}", serde_json::to_string_pretty(&evidence)?);
    if evidence.exit_code == 0 {
        Ok(())
    } else {
        Err(directive_error(
            "recorded command exited non-zero",
            "The failure was preserved as evidence. Fix from this output, then record another command.",
        ))
    }
}

fn cmd_record_audit(args: &[String]) -> Result<(), DevHarnessError> {
    let run_dir = resolve_run_dir(args)?;
    let reviewer = required_flag(
        args,
        "--reviewer",
        "Use --reviewer clean-context-codex unless a future directive says otherwise.",
    )?;
    let verdict = required_flag(args, "--verdict", "Use --verdict PROCEED|CHALLENGE|VETO.")?;
    let file = PathBuf::from(required_flag(
        args,
        "--file",
        "Use --file <audit.md> pointing at the clean-context review artifact.",
    )?);
    let findings_summary = optional_flag(args, "--summary").unwrap_or_default();
    let audit = record_audit(&run_dir, &reviewer, &verdict, &file, &findings_summary)?;
    println!("{}", serde_json::to_string_pretty(&audit)?);
    Ok(())
}

fn cmd_validate(args: &[String]) -> Result<(), DevHarnessError> {
    let run_dir = resolve_run_dir(args)?;
    let validation = validate_run(&run_dir)?;
    println!("{}", serde_json::to_string_pretty(&validation)?);
    Ok(())
}

fn cmd_close(args: &[String]) -> Result<(), DevHarnessError> {
    let run_dir = resolve_run_dir(args)?;
    let summary = close_run(&run_dir)?;
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}

fn cmd_summarize(args: &[String]) -> Result<(), DevHarnessError> {
    let run_dir = resolve_run_dir(args)?;
    let summary = summarize_run(&run_dir)?;
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}

fn resolve_run_dir(args: &[String]) -> Result<PathBuf, DevHarnessError> {
    if let Some(path) = optional_flag(args, "--run-dir") {
        return Ok(PathBuf::from(path));
    }
    let run_id = optional_flag(args, "--run")
        .or_else(|| env::var("TURINGOS_DEV_RUN").ok())
        .ok_or_else(|| {
            directive_error(
                "--run is required",
                "Use --run <run_id> or set TURINGOS_DEV_RUN explicitly. No global latest pointer is used.",
            )
        })?;
    Ok(default_evidence_root().join(run_id))
}

fn current_diff_for_allowed_paths(allowed_paths: &[String]) -> Result<String, DevHarnessError> {
    let mut diff = String::new();
    let tracked = Command::new("git")
        .arg("diff")
        .arg("--no-ext-diff")
        .arg("HEAD")
        .arg("--")
        .args(allowed_paths)
        .output()?;
    diff.push_str(&String::from_utf8_lossy(&tracked.stdout));

    for path in allowed_paths {
        let path_obj = Path::new(path);
        if !path_obj.exists() {
            continue;
        }
        let tracked_status = Command::new("git")
            .arg("ls-files")
            .arg("--error-unmatch")
            .arg(path)
            .output()?;
        if tracked_status.status.success() {
            continue;
        }
        diff.push_str(&format!("\ndiff --git a/dev/null b/{path}\n"));
        diff.push_str(&format!("--- /dev/null\n+++ b/{path}\n"));
        let body =
            fs::read_to_string(path_obj).unwrap_or_else(|_| "<binary or unreadable>\n".into());
        for line in body.lines() {
            diff.push('+');
            diff.push_str(line);
            diff.push('\n');
        }
    }
    Ok(diff)
}

fn required_flag(args: &[String], flag: &str, guidance: &str) -> Result<String, DevHarnessError> {
    optional_flag(args, flag)
        .ok_or_else(|| directive_error(&format!("{flag} is missing"), guidance))
}

fn optional_flag(args: &[String], flag: &str) -> Option<String> {
    args.windows(2)
        .find(|pair| pair[0] == flag)
        .map(|pair| pair[1].clone())
}

fn split_csv(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn default_evidence_root() -> PathBuf {
    PathBuf::from("handover/evidence/dev_self_hosting")
}

fn current_git_head() -> Option<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

fn directive_error(problem: &str, guidance: &str) -> DevHarnessError {
    DevHarnessError::InvalidInput(format!("{problem}. {guidance}"))
}
