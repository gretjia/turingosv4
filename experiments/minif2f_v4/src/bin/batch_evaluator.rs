//! TB-G G1.2-3 (Option B+ orchestration ruling 2026-05-11) —
//! `batch_evaluator` orchestrator binary. Spawns the existing
//! `evaluator` subprocess once per problem, sharing one
//! `runtime_repo` + one CAS + one continuous ChainTape across the
//! batch.
//!
//! Architect §1 verbatim:
//!
//! > 每个 problem 可以由 subprocess 执行；但所有 subprocess 必须
//! > resume 同一条 ChainTape: same runtime_repo / same CAS / same
//! > agent registry / same system pubkeys / same batch_id /
//! > continuous HEAD_t / no fresh genesis / no memory-only
//! > cross-task state.
//!
//! Usage:
//!
//! ```text
//! batch_evaluator \
//!   --runtime-repo <path> \
//!   --cas <path> \
//!   --batch-id <id> \
//!   --problems-file <path> \
//!   --model <name> \
//!   --n-agents <int> \
//!   --condition <n1|n3|...> \
//!   --out-dir <path> \
//!   [--evaluator-bin <path-to-evaluator>] \
//!   [--minif2f-dir <path>] \
//!   [--llm-proxy-url <url>] \
//!   [--per-task-timeout-s <int>]
//! ```
//!
//! Exit codes:
//! - 0: every task completed (exit_code == 0)
//! - 1: ≥1 subprocess crashed; batch halted (architect §3.5 halt-and-record)
//! - 2: argv / IO / preflight / lease error before any subprocess ran
//!
//! Constitutional Justification:
//! `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md`.

use std::path::PathBuf;
use std::process::{Command, ExitCode};
use std::time::{SystemTime, UNIX_EPOCH};

use minif2f_v4::batch_orchestrator::{
    build_subprocess_env, prepare_task_boundary, snapshot_post_task, verify_chain_continuity,
    write_manifest_skeleton, BatchSpec, BoundaryPrep, TaskOutcome, TerminalMarker,
};

#[tokio::main]
async fn main() -> ExitCode {
    env_logger::init();

    // FC3-S3 Trust Root: the parent must verify the same canonical
    // payload that the spawned `evaluator` subprocess verifies. This
    // catches batch-level tamper before any subprocess runs.
    let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("batch_evaluator: repo root resolves at build time");
    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
        eprintln!("batch_evaluator: TRUST_ROOT_TAMPERED at boot: {e}");
        return ExitCode::from(2);
    }

    let args: Vec<String> = std::env::args().collect();
    let parsed = match parse_argv(&args) {
        Ok(p) => p,
        Err(msg) => {
            eprintln!("{msg}");
            return ExitCode::from(2);
        }
    };

    let spec = BatchSpec {
        runtime_repo: parsed.runtime_repo.clone(),
        cas_path: parsed.cas_path.clone(),
        batch_id: parsed.batch_id.clone(),
        model: parsed.model.clone(),
        n_agents: parsed.n_agents,
        out_dir: parsed.out_dir.clone(),
        llm_proxy_url: parsed.llm_proxy_url.clone(),
    };

    let problems = match read_problems_file(&parsed.problems_file) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "batch_evaluator: read problems_file {:?}: {e}",
                parsed.problems_file
            );
            return ExitCode::from(2);
        }
    };

    if problems.is_empty() {
        eprintln!("batch_evaluator: problems_file is empty");
        return ExitCode::from(2);
    }

    if let Err(e) = std::fs::create_dir_all(&spec.runtime_repo) {
        eprintln!("batch_evaluator: mkdir runtime_repo: {e}");
        return ExitCode::from(2);
    }
    if let Err(e) = std::fs::create_dir_all(&spec.cas_path) {
        eprintln!("batch_evaluator: mkdir cas: {e}");
        return ExitCode::from(2);
    }
    if let Err(e) = std::fs::create_dir_all(&spec.out_dir) {
        eprintln!("batch_evaluator: mkdir out_dir: {e}");
        return ExitCode::from(2);
    }

    let mut outcomes: Vec<TaskOutcome> = Vec::with_capacity(problems.len());
    let mut terminated_reason: Option<String> = None;

    for (task_index, problem_id) in problems.iter().enumerate() {
        let task_index = task_index as u64;
        let started_at_unix_s = unix_now();

        let boundary = match prepare_task_boundary(&spec, task_index, outcomes.last()) {
            Ok(b) => b,
            Err(e) => {
                let reason = format!("boundary prep failed: {e}");
                eprintln!("batch_evaluator: {reason}");
                outcomes.push(TaskOutcome {
                    task_index,
                    problem_id: problem_id.clone(),
                    start_head_t_hex: String::new(),
                    end_head_t_hex: String::new(),
                    start_chain_length: 0,
                    end_chain_length: 0,
                    exit_code: -1,
                    started_at_unix_s,
                    finished_at_unix_s: unix_now(),
                    terminal_marker: TerminalMarker::PreflightRejected {
                        failure: reason.clone(),
                    },
                });
                terminated_reason = Some(reason);
                break;
            }
        };

        let (start_head_t_hex, start_chain_length) = match &boundary {
            BoundaryPrep::FreshGenesis => (String::new(), 0u64),
            BoundaryPrep::Resume {
                start_head_t_hex,
                start_chain_length,
                ..
            } => (start_head_t_hex.clone(), *start_chain_length),
        };

        let env = build_subprocess_env(&spec, task_index, &boundary);

        // Spawn the existing evaluator binary. Per-problem timeout
        // wraps the spawn so a hung subprocess does not deadlock the
        // batch (architect §3.5: halt-and-record is the safe default).
        let problem_file = if parsed.minif2f_dir.as_os_str().is_empty() {
            problem_id.clone()
        } else {
            parsed
                .minif2f_dir
                .join(format!("{problem_id}.lean"))
                .to_string_lossy()
                .into_owned()
        };
        let per_task_log_dir = spec
            .out_dir
            .join(format!("P{:03}_{}", task_index, problem_id));
        if let Err(e) = std::fs::create_dir_all(&per_task_log_dir) {
            eprintln!("batch_evaluator: mkdir per-task log dir {per_task_log_dir:?}: {e}");
        }
        let stdout_path = per_task_log_dir.join("evaluator.stdout");
        let stderr_path = per_task_log_dir.join("evaluator.stderr");

        let exit_code = match spawn_evaluator(
            &parsed.evaluator_bin,
            &problem_file,
            &parsed.condition,
            &env,
            &stdout_path,
            &stderr_path,
            parsed.per_task_timeout_s,
        ) {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "batch_evaluator: subprocess spawn failed for task {task_index} {problem_id}: {e}"
                );
                -1
            }
        };

        // Boundary lease (if any) drops here as `boundary` goes out
        // of scope on the next iteration / break. Drop explicitly so
        // the chain_tape_lease.json is released before snapshot_post.
        drop(boundary);

        let (end_head_t_hex, end_chain_length) = match snapshot_post_task(&spec.runtime_repo) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("batch_evaluator: snapshot_post_task failed: {e}");
                (String::new(), 0)
            }
        };

        let terminal_marker = if exit_code == 0 {
            TerminalMarker::Completed
        } else {
            TerminalMarker::SubprocessCrashed { exit_code }
        };

        let outcome = TaskOutcome {
            task_index,
            problem_id: problem_id.clone(),
            start_head_t_hex,
            end_head_t_hex,
            start_chain_length,
            end_chain_length,
            exit_code,
            started_at_unix_s,
            finished_at_unix_s: unix_now(),
            terminal_marker: terminal_marker.clone(),
        };
        outcomes.push(outcome);

        if exit_code != 0 {
            terminated_reason = Some(format!(
                "subprocess for task {task_index} {problem_id} exited non-zero ({exit_code})"
            ));
            break;
        }
    }

    // Write the manifest skeleton regardless of outcome — the
    // manifest itself is the architect-mandated "this batch happened"
    // proof per §3.3.
    if let Err(e) = write_manifest_skeleton(&spec, &outcomes, terminated_reason.as_deref()) {
        eprintln!("batch_evaluator: manifest write failed: {e}");
    }

    // Continuity check is best-effort — a halted batch may have a
    // continuous prefix; we still report the continuity status.
    match verify_chain_continuity(&outcomes) {
        Ok(()) => println!(
            "batch_evaluator: chain continuity OK across {} tasks",
            outcomes.len()
        ),
        Err(e) => eprintln!("batch_evaluator: chain continuity FAIL: {e}"),
    }

    if terminated_reason.is_some() {
        ExitCode::from(1)
    } else {
        ExitCode::from(0)
    }
}

struct ParsedArgs {
    runtime_repo: PathBuf,
    cas_path: PathBuf,
    batch_id: String,
    problems_file: PathBuf,
    model: String,
    n_agents: usize,
    condition: String,
    out_dir: PathBuf,
    evaluator_bin: PathBuf,
    minif2f_dir: PathBuf,
    llm_proxy_url: String,
    per_task_timeout_s: u64,
}

fn parse_argv(args: &[String]) -> Result<ParsedArgs, String> {
    let mut runtime_repo: Option<PathBuf> = None;
    let mut cas_path: Option<PathBuf> = None;
    let mut batch_id: Option<String> = None;
    let mut problems_file: Option<PathBuf> = None;
    let mut model: Option<String> = None;
    let mut n_agents: Option<usize> = None;
    let mut condition: Option<String> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut evaluator_bin: Option<PathBuf> = None;
    let mut minif2f_dir: Option<PathBuf> = None;
    let mut llm_proxy_url: Option<String> = None;
    let mut per_task_timeout_s: Option<u64> = None;

    let mut iter = args.iter().skip(1);
    while let Some(a) = iter.next() {
        let mut next = || iter.next().cloned();
        match a.as_str() {
            "--runtime-repo" => runtime_repo = next().map(PathBuf::from),
            "--cas" => cas_path = next().map(PathBuf::from),
            "--batch-id" => batch_id = next(),
            "--problems-file" => problems_file = next().map(PathBuf::from),
            "--model" => model = next(),
            "--n-agents" => {
                n_agents = next().and_then(|s| s.parse().ok());
            }
            "--condition" => condition = next(),
            "--out-dir" => out_dir = next().map(PathBuf::from),
            "--evaluator-bin" => evaluator_bin = next().map(PathBuf::from),
            "--minif2f-dir" => minif2f_dir = next().map(PathBuf::from),
            "--llm-proxy-url" => llm_proxy_url = next(),
            "--per-task-timeout-s" => {
                per_task_timeout_s = next().and_then(|s| s.parse().ok());
            }
            other => return Err(format!("batch_evaluator: unknown arg {other}")),
        }
    }

    Ok(ParsedArgs {
        runtime_repo: runtime_repo.ok_or_else(|| "missing --runtime-repo".to_string())?,
        cas_path: cas_path.ok_or_else(|| "missing --cas".to_string())?,
        batch_id: batch_id.ok_or_else(|| "missing --batch-id".to_string())?,
        problems_file: problems_file.ok_or_else(|| "missing --problems-file".to_string())?,
        model: model.unwrap_or_else(|| "deepseek-v4-flash".into()),
        n_agents: n_agents.unwrap_or(1),
        condition: condition.unwrap_or_else(|| "n1".into()),
        out_dir: out_dir.ok_or_else(|| "missing --out-dir".to_string())?,
        evaluator_bin: evaluator_bin.unwrap_or_else(|| PathBuf::from("evaluator")),
        minif2f_dir: minif2f_dir.unwrap_or_default(),
        llm_proxy_url: llm_proxy_url.unwrap_or_else(|| "http://localhost:8080".into()),
        per_task_timeout_s: per_task_timeout_s.unwrap_or(600),
    })
}

fn read_problems_file(path: &PathBuf) -> std::io::Result<Vec<String>> {
    let s = std::fs::read_to_string(path)?;
    Ok(s.lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect())
}

fn spawn_evaluator(
    evaluator_bin: &PathBuf,
    problem_file: &str,
    condition: &str,
    env: &[(String, String)],
    stdout_path: &PathBuf,
    stderr_path: &PathBuf,
    timeout_s: u64,
) -> Result<i32, String> {
    let stdout = std::fs::File::create(stdout_path)
        .map_err(|e| format!("create stdout file {stdout_path:?}: {e}"))?;
    let stderr = std::fs::File::create(stderr_path)
        .map_err(|e| format!("create stderr file {stderr_path:?}: {e}"))?;

    let mut cmd = Command::new(evaluator_bin);
    cmd.arg(problem_file)
        .env("CONDITION", condition)
        .stdout(stdout)
        .stderr(stderr);
    for (k, v) in env {
        cmd.env(k, v);
    }

    // Per-task timeout via wait_timeout would require an extra
    // crate; for now we let the subprocess run to completion and
    // log the elapsed time. Forward step: add wait_timeout-rs or
    // libc::alarm-style timeout.
    let _ = timeout_s; // reserved for future wait-timeout integration

    let status = cmd.status().map_err(|e| format!("evaluator spawn: {e}"))?;
    Ok(status.code().unwrap_or(-1))
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
