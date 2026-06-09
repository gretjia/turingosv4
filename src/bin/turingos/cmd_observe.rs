//! TRACE_MATRIX FC1 + FC2 + FC3: `turingos observe` — unified top-level
//! liveness observer over a workspace's canonical tape.
//!
//! 1.0 blocker #5 fix: the observe-only mechanisms (`observe_fc_liveness`,
//! `reconstruct_vpput_from_tape`) were previously reachable ONLY from the
//! `livefc1_swarm_verify` bin (no user CLI). This subcommand exposes them as
//! the single entrypoint the E2E observer agent uses:
//!
//!   turingos observe --workspace <WS>
//!
//! It loads the workspace's `LoadedTape` (runtime_repo L4 + rejections.jsonl
//! L4.E + CAS) and prints the FC1/FC2/FC3 node liveness + no-zombie verdict +
//! the per-task integer VPPUT rollup. Pure READ — no mutation, no head
//! advance, no economic state change.
//!
//! Risk class: 1 (additive, read-only; no new Cargo deps; no architecture
//! change). Reuses the existing library observers verbatim.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use turingosv4::runtime::agent_scheduler::fc_liveness_observer::{
    build_inventory, observe_fc_liveness, render_fc_liveness_summary, FcNodeStatus,
};
use turingosv4::runtime::agent_scheduler::vpput_reconstruction::{
    reconstruct_vpput_from_tape, render_vpput_summary,
};
use turingosv4::runtime::audit_assertions::{load_tape, AuditInputs};

/// TRACE_MATRIX FC2-N16: `observe` short-help
pub(crate) const SHORT_HELP: &str =
    "Observe FC1/FC2/FC3 liveness + no-zombie + VPPUT rollup over a workspace tape (read-only)";

/// TRACE_MATRIX FC2-N16: `observe` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos observe — Read-only FC1/FC2/FC3 liveness + VPPUT observer

USAGE:
    turingos observe --workspace <PATH>

OPTIONS:
    --workspace <PATH>   Workspace directory (CLI root, or a session subdir;
                         the canonical runtime_repo + cas are resolved at the
                         workspace ROOT containing genesis_payload.toml).
    -h, --help           Print this help.

DESCRIPTION:
    Loads the workspace's canonical tape (runtime_repo L4 + rejections.jsonl
    L4.E + CAS) and prints, purely read-only:
      - FC1/FC2/FC3 constitution-flowchart node liveness + no-zombie verdict
      - the per-task integer VPPUT (Verified Progress Per Unit Token) rollup
    This is the unified top-level observer entrypoint for the E2E observer
    agent. It performs NO mutation and does NOT advance the chain head.
"#;

/// TRACE_MATRIX FC2-N16: `observe` dispatch entry.
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    match run_inner(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("turingos observe: {e}");
            ExitCode::from(1)
        }
    }
}

fn run_inner(args: &[String]) -> Result<(), String> {
    let mut workspace: Option<PathBuf> = None;
    let mut iter = args.iter();
    while let Some(a) = iter.next() {
        match a.as_str() {
            "--workspace" => {
                workspace = Some(PathBuf::from(
                    iter.next().ok_or("missing value after --workspace")?,
                ));
            }
            other => return Err(format!("unknown arg: {other}")),
        }
    }
    let workspace = workspace.ok_or("missing required flag: --workspace")?;
    if !workspace.exists() {
        return Err(format!("workspace not found: {}", workspace.display()));
    }

    // Resolve the workspace ROOT (the dir holding genesis_payload.toml). The
    // canonical runtime_repo + cas live there, NOT in a session subdir.
    let root = find_root_workspace(&workspace)
        .ok_or_else(|| {
            format!(
                "could not locate genesis_payload.toml within 3 parents of {}",
                workspace.display()
            )
        })?;

    let runtime_repo = root.join("runtime_repo");
    let cas_dir = root.join("cas");
    let genesis = root.join("genesis_payload.toml");
    // constitution.md lives at the source-repo CWD (the trust-rooted axiom
    // layer); fall back to the workspace root if a copy is present there.
    let constitution = {
        let cwd_constitution = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("constitution.md");
        if cwd_constitution.is_file() {
            cwd_constitution
        } else {
            root.join("constitution.md")
        }
    };

    if !runtime_repo.exists() {
        return Err(format!(
            "no canonical tape at {} (run `turingos generate` first to produce a runtime_repo)",
            runtime_repo.display()
        ));
    }

    let inputs = AuditInputs {
        runtime_repo: runtime_repo.clone(),
        cas_dir: cas_dir.clone(),
        agent_pubkeys: runtime_repo.join("agent_pubkeys.json"),
        pinned_pubkeys: runtime_repo.join("pinned_pubkeys.json"),
        genesis,
        constitution,
        markov_pointer: None,
        alignment_dir: None,
    };
    let tape = load_tape(&inputs).map_err(|e| format!("load_tape: {e}"))?;

    // ── FC liveness (FC1/FC2/FC3 node liveness + no-zombie) ──────────────────
    // Honest inventory: FC1 predicate-gated advance + FC2 map-reduce tick are
    // driven by `turingos generate`'s sequencer admissions on this tape. FC3
    // (the architect/canary governance leg) is claimed but honestly excused for
    // a product-generation workload (it is not a zombie — it is simply not
    // exercised by this task type).
    let inventory = build_inventory(vec![
        (
            "fc1_predicate_gated_advance".into(),
            vec!["FC1:predicates".into()],
            None,
        ),
        (
            "fc2_map_reduce_tick".into(),
            vec!["FC2:tick".into()],
            None,
        ),
        (
            "fc3_governance_loop".into(),
            vec!["FC3:constitution".into()],
            Some("not-exercised-by-product-generation-workload".into()),
        ),
    ]);
    let report = observe_fc_liveness(&tape, &inventory);

    println!("==================== turingos observe — workspace tape ====================");
    println!("workspace root : {}", root.display());
    println!("runtime_repo   : {}", runtime_repo.display());
    println!("cas            : {}", cas_dir.display());
    println!();
    println!("{}", render_fc_liveness_summary(&report));
    println!();

    let fc1_live: Vec<String> = report
        .fc1_nodes
        .iter()
        .filter(|r| r.status == FcNodeStatus::Live)
        .map(|r| r.node_id.clone())
        .collect();
    println!("(A) FC-liveness rollup:");
    println!("    FC1 live nodes        : {fc1_live:?}");
    println!(
        "    L4 entries={} L4.E entries={}",
        report.l4_entry_count, report.l4e_entry_count
    );
    println!(
        "    zombie_count={} no_zombie={}",
        report.zombie_count,
        report.no_zombies()
    );
    println!("    FC3 disposition       : {}", report.fc3_disposition);
    println!();

    // ── VPPUT reconstruction (per-task integer Verified PPUT) ────────────────
    let recon0 = reconstruct_vpput_from_tape(&tape, &[]);
    let all_task_ids: Vec<String> = recon0.tasks.iter().map(|t| t.task_id.clone()).collect();
    let recon = reconstruct_vpput_from_tape(&tape, &all_task_ids);
    println!("(B) VPPUT rollup:");
    println!("{}", render_vpput_summary(&recon));
    println!(
        "    ground_truth_solved={} (non-zero VPPUT iff a VerificationResult.verified oracle fired)",
        recon.ground_truth_solved_count()
    );

    Ok(())
}

/// Locate the workspace ROOT (the dir containing `genesis_payload.toml`),
/// starting from any path within the workspace tree. Walks at most 3 parents.
/// Mirrors `cmd_generate::find_root_workspace` — kept local so `observe` does
/// not depend on `generate`'s module surface.
fn find_root_workspace(start: &Path) -> Option<PathBuf> {
    const MAX_DEPTH: usize = 3;
    let mut current = start.to_path_buf();
    for _ in 0..=MAX_DEPTH {
        if current.join("genesis_payload.toml").is_file() {
            return Some(current);
        }
        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => return None,
        }
    }
    None
}
