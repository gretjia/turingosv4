//! True-suite boot/CLI evidence helper.
//!
//! This binary is intentionally small: it calls the current runtime boot API,
//! writes the going-forward GenesisReport, resumes the same run's ChainTape
//! once, and emits one system MapReduceTick through the public system-emission
//! path.
//! It is a runner helper, not a new kernel path.

use std::path::PathBuf;
use std::process::ExitCode;

use turingosv4::runtime::genesis_report::GenesisReport;
use turingosv4::runtime::{build_chaintape_sequencer, RuntimeChaintapeConfig};
use turingosv4::state::sequencer::SystemEmitCommand;
use turingosv4::state::typed_tx::TickKind;

#[derive(Debug)]
struct Args {
    runtime_repo: PathBuf,
    cas: PathBuf,
    run_id: String,
    constitution: PathBuf,
}

fn usage() -> &'static str {
    "usage: boot_cli_current_kernel_fresh --runtime-repo <PATH> --cas <PATH> --run-id <ID> --constitution <constitution.md>"
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut runtime_repo: Option<PathBuf> = None;
    let mut cas: Option<PathBuf> = None;
    let mut run_id: Option<String> = None;
    let mut constitution: Option<PathBuf> = None;
    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "--runtime-repo" => {
                i += 1;
                runtime_repo = Some(
                    argv.get(i)
                        .ok_or("missing value after --runtime-repo")?
                        .into(),
                );
            }
            "--cas" => {
                i += 1;
                cas = Some(argv.get(i).ok_or("missing value after --cas")?.into());
            }
            "--run-id" => {
                i += 1;
                run_id = Some(argv.get(i).ok_or("missing value after --run-id")?.clone());
            }
            "--constitution" => {
                i += 1;
                constitution = Some(
                    argv.get(i)
                        .ok_or("missing value after --constitution")?
                        .into(),
                );
            }
            "--help" | "-h" => return Err(usage().into()),
            other => return Err(format!("unknown arg: {other}")),
        }
        i += 1;
    }
    Ok(Args {
        runtime_repo: runtime_repo.ok_or("--runtime-repo required")?,
        cas: cas.ok_or("--cas required")?,
        run_id: run_id.ok_or("--run-id required")?,
        constitution: constitution.ok_or("--constitution required")?,
    })
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let args = match parse_args(&argv) {
        Ok(args) => args,
        Err(msg) => {
            eprintln!("boot_cli_current_kernel_fresh: {msg}");
            eprintln!("{}", usage());
            return ExitCode::from(2);
        }
    };

    if let Err(err) = run(args).await {
        eprintln!("boot_cli_current_kernel_fresh: {err}");
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

async fn run(args: Args) -> Result<(), String> {
    let fresh_cfg = RuntimeChaintapeConfig {
        runtime_repo_path: args.runtime_repo.clone(),
        cas_path: args.cas.clone(),
        run_id: args.run_id.clone(),
        queue_capacity: 16,
        resume_existing_chain: false,
    };
    let fresh =
        build_chaintape_sequencer(&fresh_cfg).map_err(|e| format!("fresh boot failed: {e}"))?;
    fresh
        .shutdown()
        .await
        .map_err(|e| format!("fresh shutdown failed: {e}"))?;

    let resume_cfg = RuntimeChaintapeConfig {
        runtime_repo_path: args.runtime_repo.clone(),
        cas_path: args.cas.clone(),
        run_id: args.run_id.clone(),
        queue_capacity: 16,
        resume_existing_chain: true,
    };
    let resumed =
        build_chaintape_sequencer(&resume_cfg).map_err(|e| format!("resume boot failed: {e}"))?;
    resumed
        .sequencer
        .emit_system_tx(SystemEmitCommand::MapReduceTick {
            tick_kind: TickKind::Scheduled,
        })
        .await
        .map_err(|e| format!("resume map-reduce tick emit failed: {e}"))?;
    resumed
        .shutdown()
        .await
        .map_err(|e| format!("resume shutdown failed: {e}"))?;

    let report = GenesisReport {
        constitution_hash: GenesisReport::hash_constitution_md(&args.constitution),
        runtime_repo: args.runtime_repo.display().to_string(),
        cas_path: args.cas.display().to_string(),
        system_pubkey_hash: GenesisReport::hash_system_pubkey_manifest(&args.runtime_repo),
        agent_pubkeys_path: "agent_pubkeys.json".to_string(),
        initial_balances: vec![],
        task_id: None,
        task_open_tx: None,
        escrow_lock_tx: None,
        agent_model_assignment: vec![],
        model_assignment_manifest_cid: None,
        agent_role_assignment: vec![],
        role_assignment_manifest_cid: None,
    };
    report
        .write_to_runtime_repo(&args.runtime_repo)
        .map_err(|e| format!("write genesis_report.json: {e}"))?;

    println!(
        "boot_cli_current_kernel_fresh: wrote runtime_repo={} cas={}",
        args.runtime_repo.display(),
        args.cas.display()
    );
    Ok(())
}
