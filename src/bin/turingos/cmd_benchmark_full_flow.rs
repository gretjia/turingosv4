//! TRACE_MATRIX FC2-N16: `turingos benchmark full-flow` CLI.
//!
//! This command is an orchestration entrypoint. It does not become a new source
//! of truth: it drives existing runtime binaries, then packages ChainTape/CAS
//! replay receipts and the task-verdict boundary into one audit packet.

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Duration;

use serde::Serialize;
use serde_json::Value;
use turingosv4::sdk::sanitized_runner::{
    env_allowlist_from_current, run_sanitized, SanitizedCommand, SanitizedOutput,
};

use crate::common::resolve_external_bin;

pub(crate) const SHORT_HELP: &str =
    "Run Constitutional Full-Flow Benchmark and export an auditable packet";
const FULL_FLOW_ENV_KEYS: &[&str] = &[
    "PATH",
    "HOME",
    "USER",
    "LANG",
    "TMPDIR",
    "SILICONFLOW_API_KEY",
    "SILICONFLOW_API_KEY_SECONDARY",
    "SILICONFLOW_API_KEY_TERTIARY",
    "DEEPSEEK_API_KEY",
    "DEEPSEEK_API_KEY_WORKER",
    "DEEPSEEK_API_KEY_SECONDARY",
    "OPENAI_API_KEY",
    "TURINGOS_SILICONFLOW_ENDPOINT",
];

pub(crate) const FULL_HELP: &str = r#"turingos benchmark full-flow — Constitutional Full-Flow Benchmark

USAGE:
    turingos benchmark full-flow run --run-dir <PATH>
                                      --run-id <ID>
                                      --constitution <constitution.md>
                                      --sample-json <SWE_SAMPLE_JSON>
                                      --llm-proxy-url <URL>
                                      --model <MODEL>
                                      [--task-evidence-dir <TDMA_EVIDENCE_DIR>]
                                      [--require-task-pass]

VERDICTS:
    FLOW-PASS      FC1/FC2/FC3 node-level runtime receipts are present.
    SYSTEM-PASS    replay, CAS retrieval, signatures, and packet consistency pass.
    TASK-PASS      real task verifier passed. SWE-bench structural smoke is NOT
                   TASK-PASS; hidden-test SWE-bench or an equivalent real verifier
    must pass before this label is legal.

TASK EVIDENCE:
    By default the packet records the current-kernel SWE-bench structural smoke
    as a non-TASK-PASS task boundary. Pass --task-evidence-dir to attach a real
    `turingos tdma run --judge swebench` evidence directory. The directory must
    contain manifest.json and per_attempt_probes.jsonl; TASK-PASS is emitted
    only when manifest stages_completed == stages_total and prompt leakage is
    false.

DESCRIPTION:
    Drives the real `turingos` CLI boundary through init and verify, then uses
    current-kernel runtime helpers to produce ChainTape/CAS task evidence,
    market participation, FC3 governance/re-init rows, FC1 L4.E rejection
    evidence, full-system participation receipts, and a benchmark packet.

    This command is not a proof by prose. The packet points auditors to the
    runtime repo, CAS, replay report, full-system participation report, command
    logs, source state, and task-verdict evidence.
"#;

#[derive(Debug)]
struct Args {
    action: String,
    run_dir: PathBuf,
    run_id: String,
    constitution: PathBuf,
    sample_json: PathBuf,
    llm_proxy_url: String,
    model: String,
    task_evidence_dir: Option<PathBuf>,
    require_task_pass: bool,
}

#[derive(Debug, Serialize)]
struct CommandRecord {
    step: String,
    argv: Vec<String>,
    cwd: String,
    exit_code: Option<i32>,
    timed_out: bool,
    stdout_path: String,
    stderr_path: String,
}

#[derive(Debug, Serialize)]
struct EvidencePaths {
    run_dir: String,
    runtime_repo: String,
    cas: String,
    sample_json: String,
    domain_manifest: String,
    augmentation_manifest: String,
    governance_capsule_index: String,
    replay_report: String,
    full_system_participation: String,
    task_evidence_dir: Option<String>,
}

#[derive(Debug, Serialize)]
struct TaskVerdict {
    kind: String,
    source: String,
    benchmark_verdict: Option<String>,
    closure_scope: Option<String>,
    final_closure_possible: bool,
    stages_completed: Option<u64>,
    stages_total: Option<u64>,
    total_attempts: Option<u64>,
    leak_in_any_prompt: Option<bool>,
}

#[derive(Debug, Serialize)]
struct SourceState {
    git_head: Option<String>,
    worktree_status_short: Option<String>,
}

#[derive(Debug, Serialize)]
struct BenchmarkPacket {
    schema_version: &'static str,
    run_id: String,
    entrypoint: &'static str,
    flow_verdict: String,
    system_verdict: String,
    task_verdict: TaskVerdict,
    evidence_paths: EvidencePaths,
    source_state: SourceState,
    command_log: Vec<CommandRecord>,
    notes: Vec<String>,
}

pub(crate) fn run(argv: &[String]) -> ExitCode {
    if argv.is_empty() || argv.iter().any(|a| a == "--help" || a == "-h") {
        println!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    let args = match parse_args(argv) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("turingos benchmark full-flow: {err}");
            eprintln!("{FULL_HELP}");
            return ExitCode::from(2);
        }
    };
    if args.action != "run" {
        eprintln!(
            "turingos benchmark full-flow: unknown action `{}`",
            args.action
        );
        eprintln!("{FULL_HELP}");
        return ExitCode::from(2);
    }
    match run_full_flow(args) {
        Ok(pass) => {
            if pass {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
        Err(err) => {
            eprintln!("turingos benchmark full-flow: {err}");
            ExitCode::from(1)
        }
    }
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let action = argv.first().ok_or("action required")?.clone();
    let mut run_dir = None;
    let mut run_id = None;
    let mut constitution = None;
    let mut sample_json = None;
    let mut llm_proxy_url = None;
    let mut model = None;
    let mut task_evidence_dir = None;
    let mut require_task_pass = false;

    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--run-dir" => {
                i += 1;
                run_dir = Some(PathBuf::from(
                    argv.get(i).ok_or("--run-dir requires value")?,
                ));
            }
            "--run-id" => {
                i += 1;
                run_id = Some(argv.get(i).ok_or("--run-id requires value")?.clone());
            }
            "--constitution" => {
                i += 1;
                constitution = Some(PathBuf::from(
                    argv.get(i).ok_or("--constitution requires value")?,
                ));
            }
            "--sample-json" => {
                i += 1;
                sample_json = Some(PathBuf::from(
                    argv.get(i).ok_or("--sample-json requires value")?,
                ));
            }
            "--llm-proxy-url" => {
                i += 1;
                llm_proxy_url = Some(argv.get(i).ok_or("--llm-proxy-url requires value")?.clone());
            }
            "--model" => {
                i += 1;
                model = Some(argv.get(i).ok_or("--model requires value")?.clone());
            }
            "--task-evidence-dir" => {
                i += 1;
                task_evidence_dir = Some(PathBuf::from(
                    argv.get(i).ok_or("--task-evidence-dir requires value")?,
                ));
            }
            "--require-task-pass" => require_task_pass = true,
            other => return Err(format!("unknown arg: {other}")),
        }
        i += 1;
    }

    Ok(Args {
        action,
        run_dir: run_dir.ok_or("--run-dir required")?,
        run_id: run_id.ok_or("--run-id required")?,
        constitution: constitution.ok_or("--constitution required")?,
        sample_json: sample_json.ok_or("--sample-json required")?,
        llm_proxy_url: llm_proxy_url.ok_or("--llm-proxy-url required")?,
        model: model.ok_or("--model required")?,
        task_evidence_dir,
        require_task_pass,
    })
}

fn run_full_flow(args: Args) -> Result<bool, String> {
    if let Some(parent) = args.run_dir.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("create run-dir parent {}: {e}", parent.display()))?;
    }
    let command_log_dir = args.run_dir.with_file_name(format!(
        "{}.command_logs",
        args.run_dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("full_flow")
    ));
    std::fs::create_dir_all(&command_log_dir).map_err(|e| format!("create command_logs: {e}"))?;

    let runtime_repo = args.run_dir.join("runtime_repo");
    let cas = args.run_dir.join("cas");
    let replay_report = args.run_dir.join("replay_report.json");
    let genesis_report = args.run_dir.join("genesis_report.json");
    let domain_manifest = args
        .run_dir
        .join("swebench_live_coding_repair_manifest.json");
    let augmentation_manifest = args.run_dir.join("full_system_augmentation_manifest.json");
    let governance_index = args.run_dir.join("governance_capsule_index.json");
    let participation_report = args.run_dir.join("full_system_participation.json");
    let mut commands = Vec::new();
    let cwd = std::env::current_dir().map_err(|e| format!("current dir: {e}"))?;
    let turingos = std::env::current_exe().map_err(|e| format!("current exe: {e}"))?;

    run_step(
        &mut commands,
        &command_log_dir,
        "01_turingos_init",
        turingos.clone(),
        vec![
            "init".into(),
            "--project".into(),
            args.run_dir.display().to_string(),
            "--template".into(),
            "proof".into(),
            "--provider".into(),
            "deepseek".into(),
        ],
        &cwd,
        Duration::from_secs(120),
    )?;

    let snapshot_dir = args.run_dir.join("repo_snapshots");
    std::fs::create_dir_all(&snapshot_dir).map_err(|e| format!("create repo_snapshots: {e}"))?;
    let copied_sample = snapshot_dir.join("swebench_sample.json");
    std::fs::copy(&args.sample_json, &copied_sample).map_err(|e| {
        format!(
            "copy sample {} -> {}: {e}",
            args.sample_json.display(),
            copied_sample.display()
        )
    })?;

    run_step(
        &mut commands,
        &command_log_dir,
        "02_swebench_structural_smoke",
        resolve_external_bin("swebench_live_coding_repair_current_kernel"),
        vec![
            "--runtime-repo".into(),
            runtime_repo.display().to_string(),
            "--cas".into(),
            cas.display().to_string(),
            "--run-id".into(),
            args.run_id.clone(),
            "--constitution".into(),
            args.constitution.display().to_string(),
            "--sample-json".into(),
            copied_sample.display().to_string(),
            "--llm-proxy-url".into(),
            args.llm_proxy_url.clone(),
            "--model".into(),
            args.model.clone(),
            "--out-dir".into(),
            args.run_dir.display().to_string(),
        ],
        &cwd,
        Duration::from_secs(300),
    )?;

    run_step(
        &mut commands,
        &command_log_dir,
        "03_full_system_augment",
        resolve_external_bin("full_system_augment_current_kernel"),
        vec![
            "--runtime-repo".into(),
            runtime_repo.display().to_string(),
            "--cas".into(),
            cas.display().to_string(),
            "--run-id".into(),
            args.run_id.clone(),
            "--constitution".into(),
            args.constitution.display().to_string(),
            "--out-dir".into(),
            args.run_dir.display().to_string(),
        ],
        &cwd,
        Duration::from_secs(300),
    )?;

    std::fs::copy(runtime_repo.join("genesis_report.json"), &genesis_report)
        .map_err(|e| format!("copy refreshed genesis_report.json: {e}"))?;

    run_step(
        &mut commands,
        &command_log_dir,
        "04_turingos_verify_chaintape",
        turingos,
        vec![
            "verify".into(),
            "chaintape".into(),
            "--repo".into(),
            runtime_repo.display().to_string(),
            "--cas".into(),
            cas.display().to_string(),
            "--run-id".into(),
            args.run_id.clone(),
            "--out".into(),
            replay_report.display().to_string(),
        ],
        &cwd,
        Duration::from_secs(300),
    )?;

    run_step(
        &mut commands,
        &command_log_dir,
        "05_full_system_participation",
        resolve_external_bin("full_system_participation_current_kernel"),
        vec![
            "--run-id".into(),
            args.run_id.clone(),
            "--family-id".into(),
            "swebench_live_coding_repair".into(),
            "--entrypoint".into(),
            "turingos benchmark full-flow run".into(),
            "--runtime-repo".into(),
            runtime_repo.display().to_string(),
            "--cas".into(),
            cas.display().to_string(),
            "--replay-report".into(),
            replay_report.display().to_string(),
            "--genesis-report".into(),
            genesis_report.display().to_string(),
            "--domain-manifest".into(),
            domain_manifest.display().to_string(),
            "--fc3-index".into(),
            governance_index.display().to_string(),
            "--require-full-system".into(),
            "--out".into(),
            participation_report.display().to_string(),
        ],
        &cwd,
        Duration::from_secs(300),
    )?;

    let participation: Value = read_json(&participation_report)?;
    let domain: Value = read_json(&domain_manifest)?;
    let flow_pass = participation
        .pointer("/flowchart_node_receipts/verdict")
        .and_then(Value::as_str)
        == Some("FLOW-PASS");
    let full_system_lit = participation
        .pointer("/verdict/full_system_verdict")
        .and_then(Value::as_str)
        == Some("FULL_SYSTEM_LIT");
    let replay_pass = participation
        .pointer("/replay/all_indicators_pass")
        .and_then(Value::as_bool)
        == Some(true);
    let system_pass = flow_pass && full_system_lit && replay_pass;
    let task_verdict = if let Some(task_evidence_dir) = &args.task_evidence_dir {
        task_verdict_from_tdma_evidence(task_evidence_dir)?
    } else {
        task_verdict_from_domain(&domain)
    };
    if args.require_task_pass && task_verdict.kind != "TASK-PASS" {
        write_packet(
            &args,
            commands,
            &runtime_repo,
            &cas,
            &copied_sample,
            &domain_manifest,
            &augmentation_manifest,
            &governance_index,
            &replay_report,
            &participation_report,
            args.task_evidence_dir.as_deref(),
            flow_pass,
            system_pass,
            task_verdict,
        )?;
        return Err("--require-task-pass set but task verifier did not return TASK-PASS".into());
    }

    write_packet(
        &args,
        commands,
        &runtime_repo,
        &cas,
        &copied_sample,
        &domain_manifest,
        &augmentation_manifest,
        &governance_index,
        &replay_report,
        &participation_report,
        args.task_evidence_dir.as_deref(),
        flow_pass,
        system_pass,
        task_verdict,
    )?;
    Ok(system_pass)
}

#[allow(clippy::too_many_arguments)]
fn write_packet(
    args: &Args,
    commands: Vec<CommandRecord>,
    runtime_repo: &Path,
    cas: &Path,
    sample_json: &Path,
    domain_manifest: &Path,
    augmentation_manifest: &Path,
    governance_index: &Path,
    replay_report: &Path,
    participation_report: &Path,
    task_evidence_dir: Option<&Path>,
    flow_pass: bool,
    system_pass: bool,
    task_verdict: TaskVerdict,
) -> Result<(), String> {
    let packet = BenchmarkPacket {
        schema_version: "turingosv4.benchmark.constitutional_full_flow.v1",
        run_id: args.run_id.clone(),
        entrypoint: "turingos benchmark full-flow run",
        flow_verdict: if flow_pass { "FLOW-PASS" } else { "FLOW-FAIL" }.to_string(),
        system_verdict: if system_pass {
            "SYSTEM-PASS"
        } else {
            "SYSTEM-FAIL"
        }
        .to_string(),
        task_verdict,
        evidence_paths: EvidencePaths {
            run_dir: args.run_dir.display().to_string(),
            runtime_repo: runtime_repo.display().to_string(),
            cas: cas.display().to_string(),
            sample_json: sample_json.display().to_string(),
            domain_manifest: domain_manifest.display().to_string(),
            augmentation_manifest: augmentation_manifest.display().to_string(),
            governance_capsule_index: governance_index.display().to_string(),
            replay_report: replay_report.display().to_string(),
            full_system_participation: participation_report.display().to_string(),
            task_evidence_dir: task_evidence_dir.map(|p| p.display().to_string()),
        },
        source_state: SourceState {
            git_head: git_capture(&["rev-parse", "HEAD"]),
            worktree_status_short: git_capture(&["status", "--short"]),
        },
        command_log: commands,
        notes: vec![
            "FLOW/SYSTEM receipts are derived from ChainTape, L4.E, CAS, replay, and full-system participation reports.".into(),
            "SWE-bench current-kernel structural smoke is not hidden-test TASK-PASS.".into(),
            "--task-evidence-dir binds a real TDMA/SWE-bench verifier manifest when present.".into(),
            "Do not open PR from this branch until packet audit, clean-context audit, and OBL witness pass.".into(),
        ],
    };
    write_json(
        &args
            .run_dir
            .join("constitutional_full_flow_benchmark_packet.json"),
        &packet,
    )
}

fn task_verdict_from_domain(domain: &Value) -> TaskVerdict {
    let benchmark_verdict = domain
        .get("benchmark_verdict")
        .and_then(Value::as_str)
        .map(str::to_string);
    let closure_scope = domain
        .get("closure_scope")
        .and_then(Value::as_str)
        .map(str::to_string);
    let final_closure_possible = domain
        .get("final_closure_possible")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let structural_pass = domain
        .get("patch_structurally_plausible")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let kind = if final_closure_possible
        && benchmark_verdict.as_deref() == Some("resolved")
        && closure_scope.as_deref() != Some("domain_adapter_smoke_only")
    {
        "TASK-PASS"
    } else if structural_pass {
        "TASK-STRUCTURAL-SMOKE-NOT-PASS"
    } else {
        "TASK-FAIL"
    };
    TaskVerdict {
        kind: kind.to_string(),
        source: "swebench_current_kernel_structural_smoke".to_string(),
        benchmark_verdict,
        closure_scope,
        final_closure_possible,
        stages_completed: None,
        stages_total: None,
        total_attempts: None,
        leak_in_any_prompt: None,
    }
}

fn task_verdict_from_tdma_evidence(evidence_dir: &Path) -> Result<TaskVerdict, String> {
    let manifest_path = evidence_dir.join("manifest.json");
    let probes_path = evidence_dir.join("per_attempt_probes.jsonl");
    if !probes_path.is_file() {
        return Err(format!(
            "task evidence missing per_attempt_probes.jsonl: {}",
            probes_path.display()
        ));
    }
    let manifest = read_json(&manifest_path)?;
    let stages_completed = required_u64(&manifest, "stages_completed", &manifest_path)?;
    let stages_total = required_u64(&manifest, "stages_total", &manifest_path)?;
    let total_attempts = manifest.get("total_attempts").and_then(Value::as_u64);
    let leak_in_any_prompt = manifest
        .get("leak_in_any_prompt")
        .and_then(Value::as_bool)
        .unwrap_or(true);
    let task_pass = stages_total > 0 && stages_completed == stages_total && !leak_in_any_prompt;
    Ok(TaskVerdict {
        kind: if task_pass { "TASK-PASS" } else { "TASK-FAIL" }.to_string(),
        source: "swebench_tdma_hidden_test_verifier".to_string(),
        benchmark_verdict: Some(if task_pass { "resolved" } else { "unresolved" }.to_string()),
        closure_scope: Some("real_tdma_swebench".to_string()),
        final_closure_possible: task_pass,
        stages_completed: Some(stages_completed),
        stages_total: Some(stages_total),
        total_attempts,
        leak_in_any_prompt: Some(leak_in_any_prompt),
    })
}

fn required_u64(value: &Value, key: &str, path: &Path) -> Result<u64, String> {
    value
        .get(key)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("{} missing numeric field `{key}`", path.display()))
}

fn run_step(
    records: &mut Vec<CommandRecord>,
    log_dir: &Path,
    step: &str,
    program: PathBuf,
    args: Vec<String>,
    cwd: &Path,
    timeout: Duration,
) -> Result<(), String> {
    let env = env_allowlist_from_current(FULL_FLOW_ENV_KEYS);
    let output = run_sanitized(SanitizedCommand {
        program,
        args,
        cwd: cwd.to_path_buf(),
        env,
        stdin: None,
        timeout,
    })
    .map_err(|e| format!("{step}: spawn failed: {e}"))?;
    let record = write_command_record(log_dir, step, &output)?;
    let success = output.success();
    let stdout_path = record.stdout_path.clone();
    let stderr_path = record.stderr_path.clone();
    records.push(record);
    if success {
        Ok(())
    } else {
        Err(format!(
            "{step}: command failed exit={:?} timed_out={} stdout={} stderr={}",
            output.exit_code, output.timed_out, stdout_path, stderr_path
        ))
    }
}

fn write_command_record(
    log_dir: &Path,
    step: &str,
    output: &SanitizedOutput,
) -> Result<CommandRecord, String> {
    let stdout_path = log_dir.join(format!("{step}.stdout"));
    let stderr_path = log_dir.join(format!("{step}.stderr"));
    std::fs::write(&stdout_path, &output.stdout)
        .map_err(|e| format!("write {}: {e}", stdout_path.display()))?;
    std::fs::write(&stderr_path, &output.stderr)
        .map_err(|e| format!("write {}: {e}", stderr_path.display()))?;
    Ok(CommandRecord {
        step: step.to_string(),
        argv: output.argv.clone(),
        cwd: output.cwd.display().to_string(),
        exit_code: output.exit_code,
        timed_out: output.timed_out,
        stdout_path: stdout_path.display().to_string(),
        stderr_path: stderr_path.display().to_string(),
    })
}

fn read_json(path: &Path) -> Result<Value, String> {
    let raw = std::fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("parse {}: {e}", path.display()))
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let json = serde_json::to_string_pretty(value)
        .map_err(|e| format!("encode {}: {e}", path.display()))?;
    std::fs::write(path, format!("{json}\n")).map_err(|e| format!("write {}: {e}", path.display()))
}

fn git_capture(args: &[&str]) -> Option<String> {
    let output = run_sanitized(SanitizedCommand {
        program: PathBuf::from("git"),
        args: args.iter().map(|s| (*s).to_string()).collect(),
        cwd: std::env::current_dir().ok()?,
        env: env_allowlist_from_current(&["PATH"]),
        stdin: None,
        timeout: Duration::from_secs(10),
    })
    .ok()?;
    if !output.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
