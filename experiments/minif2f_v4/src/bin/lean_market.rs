//! TB-10 Atom 2 — Lean Proof Task Market user CLI.
//!
//! First user-facing product of TuringOS v4. Lets a human user post a Lean
//! theorem statement with a bounty, watch an Agent solve it under predicate
//! gating, and see the system pay out the solver's durable AgentId. Every
//! chain primitive (TaskOpenTx + EscrowLockTx + WorkTx + VerifyTx +
//! FinalizeRewardTx) was already shipped in TB-3..TB-8; TB-10 is the
//! user-facing wrapper that closes the 5-step compile loop end-to-end from
//! a non-evaluator entity.
//!
//! Architecture: lean_market is a SINGLE-PROCESS thin wrapper. The
//! `run-task` subcommand spawns the evaluator binary as a child process
//! with `TURINGOS_USER_TASK_MODE=1` + `TURINGOS_USER_TASK_BOUNTY_MICRO=<n>`
//! + a fresh chaintape path. The evaluator's preseed branch detects
//! user-mode and submits TaskOpen+EscrowLock signed by `Agent_user_0` (TB-9
//! durable keystore) instead of the legacy `tb7-7-sponsor` zero-signature
//! path, then runs its existing solver loop on the user-specified problem.
//!
//! The `view-*` subcommands open the post-run chaintape READ-ONLY via
//! `replay_full_transition` and report back to the user: task status,
//! sponsor balance, solver payout, replay verification. No live Sequencer
//! is bootstrapped during view operations (Sequencer fail-closes on
//! non-empty chaintape per TB-6 NonEmptyRuntimeRepo gate; replay is the
//! supported read-only path).
//!
//! Per architect directive 2026-05-02 Part C ruling 12+13 line 1594.
//! Per TB-10 charter §3 Atom 2 + ratification §1 Q1-Q8.

use std::path::{Path, PathBuf};
use std::process::Command;

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::system_keypair::PinnedSystemPubkeys;
use turingosv4::bottom_white::ledger::transition_ledger::{
    replay_full_transition, Git2LedgerWriter, LedgerWriter,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::money::MicroCoin;
use turingosv4::state::q_state::{AgentId, ClaimStatus, QState};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

const DEFAULT_BOUNTY_MICRO: i64 = 100_000;
const MIN_BOUNTY_MICRO: i64 = 100_000;
const DEFAULT_USER_SPONSOR: &str = "Agent_user_0";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let subcommand = args.first().map(|s| s.as_str()).unwrap_or("help");
    let sub_args: Vec<String> = args.iter().skip(1).cloned().collect();
    match subcommand {
        "run-task" => cmd_run_task(&sub_args),
        "view-task" => cmd_view_task(&sub_args),
        "view-wallet" => cmd_view_wallet(&sub_args),
        "view-replay" => cmd_view_replay(&sub_args),
        // TB-11 G3 carry-forward (TB-12 Atom 0.5b; architect 2026-05-03 §1.1):
        "tick" => cmd_tick(&sub_args),
        "view-bankruptcy" => cmd_view_bankruptcy(&sub_args),
        // TB-12 Atom 4 (architect 2026-05-03 §8 Atom 4):
        "view-positions" => cmd_view_positions(&sub_args),
        "help" | "-h" | "--help" => {
            print_help();
            std::process::exit(0);
        }
        _ => {
            eprintln!("lean_market: unknown subcommand `{subcommand}`");
            print_help();
            std::process::exit(2);
        }
    }
}

fn print_help() {
    println!(
        r#"lean_market — Lean Proof Task Market MVP (TB-10)
Usage:
  lean_market run-task    --problem <id> --bounty <micro> [--chaintape <path>] [--max-tx <n>]
                          [--max-secs <n>] [--evaluator-bin <path>] [--evaluator-arg <arg>]...
                            Bootstraps a fresh chaintape, posts a user-funded TaskOpen + EscrowLock
                            signed by Agent_user_0 (TB-9 durable keystore), then runs the evaluator's
                            real-LLM solver loop on the specified Lean problem. End-to-end: user posts
                            -> agent solves -> system verifies -> system pays.

  lean_market view-task   --chaintape <path>
                            Replays the chaintape read-only and reports task_markets_t entries,
                            claim status (Open/Verified/Finalized/Failed), payout amount, and
                            sponsor/solver durable agent_ids.

  lean_market view-wallet --chaintape <path> [--agent <id>]
                            Replays the chaintape read-only and reports balances_t for the
                            specified agent (default: Agent_user_0).

  lean_market view-replay --chaintape <path>
                            Replays the chaintape read-only and prints the 7-indicator verify
                            report. Exits 0 if all indicators GREEN, non-zero otherwise.

  lean_market help          Show this message.

Constraints:
  --problem <id>          A heldout-49 problem id (e.g. mathd_algebra_171). Resolved against
                          $TURINGOSV3_MINIF2F_DIR or fallback locations matching evaluator's
                          resolve_problem_path() convention.
  --bounty <micro>        Integer micro-Coin (1 Coin = 1_000_000 micro). Min {MIN_BOUNTY_MICRO} micro.
  --chaintape <path>      Directory for the runtime_repo. Defaults to a unique tempdir per run.

Constitution gates (TB-10 charter §5):
  * NO user-callable system_tx surface (no `lean_market settle/finalize/refund`)
  * NO post-init mint (Agent_user_0 funded only at on_init via runtime preseed)
  * NO new typed_tx variant (TB-11+)
  * NO arbitrary Lean source ingest (TB-13 Beta)
"#,
        MIN_BOUNTY_MICRO = MIN_BOUNTY_MICRO,
    );
}

// ────────────────────────────────────────────────────────────────────────────
// Argument parsing (lightweight; --flag value pairs).
// ────────────────────────────────────────────────────────────────────────────

fn arg_value(args: &[String], flag: &str) -> Option<String> {
    let mut iter = args.iter();
    while let Some(a) = iter.next() {
        if a == flag {
            return iter.next().cloned();
        }
    }
    None
}

fn arg_values_repeated(args: &[String], flag: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut iter = args.iter();
    while let Some(a) = iter.next() {
        if a == flag {
            if let Some(v) = iter.next() {
                out.push(v.clone());
            }
        }
    }
    out
}

// ────────────────────────────────────────────────────────────────────────────
// run-task: bootstrap chaintape + spawn evaluator with user-mode env vars.
// ────────────────────────────────────────────────────────────────────────────

fn cmd_run_task(args: &[String]) {
    let problem_id = arg_value(args, "--problem").unwrap_or_else(|| {
        eprintln!("lean_market run-task: --problem <id> is required");
        std::process::exit(2);
    });
    let bounty: i64 = arg_value(args, "--bounty")
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_BOUNTY_MICRO);
    if bounty < MIN_BOUNTY_MICRO {
        eprintln!("lean_market run-task: --bounty {bounty} below minimum {MIN_BOUNTY_MICRO} micro");
        std::process::exit(2);
    }
    let chaintape_path = arg_value(args, "--chaintape")
        .map(PathBuf::from)
        .unwrap_or_else(|| default_chaintape_path());
    let cas_path = chaintape_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(format!(
            "cas_{}",
            chaintape_path
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| "lean_market".into())
        ));
    let max_tx = arg_value(args, "--max-tx").unwrap_or_else(|| "10".into());
    let max_secs = arg_value(args, "--max-secs").unwrap_or_else(|| "600".into());
    let evaluator_bin = arg_value(args, "--evaluator-bin").unwrap_or_else(|| {
        // Default: same target dir as this binary.
        let exe = std::env::current_exe().expect("current_exe");
        exe.parent()
            .expect("exe parent")
            .join("evaluator")
            .display()
            .to_string()
    });
    let extra_args = arg_values_repeated(args, "--evaluator-arg");

    if chaintape_path.exists() {
        if let Ok(mut entries) = std::fs::read_dir(&chaintape_path) {
            if entries.next().is_some() {
                eprintln!(
                    "lean_market run-task: --chaintape {:?} is non-empty (Sequencer requires fresh dir per TB-6 NonEmptyRuntimeRepo gate)",
                    chaintape_path
                );
                std::process::exit(2);
            }
        }
    }
    std::fs::create_dir_all(&chaintape_path).expect("create_dir_all chaintape");
    std::fs::create_dir_all(&cas_path).expect("create_dir_all cas");

    // Resolve problem filename (evaluator's resolve_problem_path treats this
    // as a relative name and walks MiniF2F/Test/ + MiniF2F/Valid/).
    let problem_filename = if problem_id.ends_with(".lean") {
        problem_id.clone()
    } else {
        format!("{problem_id}.lean")
    };

    println!("[lean_market] run-task");
    println!("  problem      = {problem_id}");
    println!(
        "  bounty       = {bounty} micro ({:.4} coin)",
        bounty as f64 / 1_000_000.0
    );
    println!("  chaintape    = {chaintape_path:?}");
    println!("  cas          = {cas_path:?}");
    println!("  evaluator    = {evaluator_bin}");
    println!("  max_tx       = {max_tx}");
    println!("  max_secs     = {max_secs}");
    println!("  sponsor      = {DEFAULT_USER_SPONSOR}");
    println!();

    // Spawn evaluator with user-mode env vars set. The evaluator's preseed
    // branch (evaluator.rs:858+) detects TURINGOS_USER_TASK_MODE=1 and
    // submits TaskOpen+EscrowLock signed by Agent_user_0 with bounty
    // = TURINGOS_USER_TASK_BOUNTY_MICRO.
    let mut cmd = Command::new(&evaluator_bin);
    cmd.env("TURINGOS_USER_TASK_MODE", "1")
        .env("TURINGOS_USER_TASK_BOUNTY_MICRO", bounty.to_string())
        .env("TURINGOS_USER_TASK_SPONSOR", DEFAULT_USER_SPONSOR)
        .env("TURINGOS_CHAINTAPE_PATH", &chaintape_path)
        .env("TURINGOS_CAS_PATH", &cas_path)
        .env("TURINGOS_CHAINTAPE_PRESEED", "1")
        // ChainTape mode requires a swarm condition (CONDITION=n1+);
        // CONDITION=oneshot is fail-closed per evaluator.rs / TB-7R Deliverable B.
        // Default to n1 (single-agent swarm) — the canonical TB-7R/TB-8/TB-9
        // smoke condition. Caller can override by passing --evaluator-arg --mode
        // or setting CONDITION before invoking lean_market.
        .env(
            "CONDITION",
            std::env::var("CONDITION").unwrap_or_else(|_| "n1".into()),
        )
        .env("MAX_TX", &max_tx)
        .env("MAX_SECS", &max_secs);
    cmd.args(&extra_args);
    cmd.arg(&problem_filename);

    println!("[lean_market] spawning evaluator subprocess...");
    let status = cmd.status().expect("spawn evaluator");
    println!();
    println!("[lean_market] evaluator exited with status {status}");

    // Post-run summary: replay chaintape and print task status.
    if !chaintape_path.join("refs").exists() && !chaintape_path.join(".git").exists() {
        eprintln!(
            "[lean_market] no chain produced at {:?} — evaluator may have failed before bootstrap",
            chaintape_path
        );
        std::process::exit(status.code().unwrap_or(1));
    }
    println!();
    match replay_qstate(&chaintape_path, &cas_path) {
        Ok((q, l4_count)) => {
            println!("[lean_market] post-run chain summary:");
            println!("  L4 entries   = {l4_count}");
            print_user_task_summary(&q);
        }
        Err(e) => {
            eprintln!("[lean_market] replay failed: {e}");
        }
    }

    std::process::exit(status.code().unwrap_or(0));
}

fn default_chaintape_path() -> PathBuf {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let pid = std::process::id();
    let base = if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home)
            .join(".turingos")
            .join("lean_market_runs")
    } else {
        std::env::temp_dir().join("lean_market_runs")
    };
    base.join(format!("run_{ts}_{pid}"))
}

// ────────────────────────────────────────────────────────────────────────────
// view-task: read-only chain replay + print task_markets_t + claims_t status.
// ────────────────────────────────────────────────────────────────────────────

fn cmd_view_task(args: &[String]) {
    let chaintape = arg_value(args, "--chaintape").unwrap_or_else(|| {
        eprintln!("lean_market view-task: --chaintape <path> is required");
        std::process::exit(2);
    });
    let chaintape_path = PathBuf::from(&chaintape);
    let cas_path = derive_cas_path(&chaintape_path);
    match replay_qstate(&chaintape_path, &cas_path) {
        Ok((q, l4_count)) => {
            println!("[lean_market] view-task");
            println!("  chaintape    = {chaintape_path:?}");
            println!("  L4 entries   = {l4_count}");
            print_user_task_summary(&q);
        }
        Err(e) => {
            eprintln!("[lean_market] view-task replay failed: {e}");
            std::process::exit(1);
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// view-wallet: read-only chain replay + print balances_t for one agent.
// ────────────────────────────────────────────────────────────────────────────

fn cmd_view_wallet(args: &[String]) {
    let chaintape = arg_value(args, "--chaintape").unwrap_or_else(|| {
        eprintln!("lean_market view-wallet: --chaintape <path> is required");
        std::process::exit(2);
    });
    let agent = arg_value(args, "--agent").unwrap_or_else(|| DEFAULT_USER_SPONSOR.into());
    let chaintape_path = PathBuf::from(&chaintape);
    let cas_path = derive_cas_path(&chaintape_path);
    match replay_qstate(&chaintape_path, &cas_path) {
        Ok((q, l4_count)) => {
            println!("[lean_market] view-wallet");
            println!("  chaintape    = {chaintape_path:?}");
            println!("  L4 entries   = {l4_count}");
            let agent_id = AgentId(agent.clone());
            let bal = q
                .economic_state_t
                .balances_t
                .0
                .get(&agent_id)
                .copied()
                .unwrap_or(MicroCoin::zero());
            println!("  agent        = {agent}");
            println!(
                "  balance      = {} micro ({:.4} coin)",
                bal.micro_units(),
                bal.micro_units() as f64 / 1_000_000.0
            );
            println!("  total agents = {}", q.economic_state_t.balances_t.0.len());
            println!();
            println!("  full balances_t:");
            for (a, m) in q.economic_state_t.balances_t.0.iter() {
                println!("    {:30} {:>14} micro", a.0, m.micro_units());
            }
        }
        Err(e) => {
            eprintln!("[lean_market] view-wallet replay failed: {e}");
            std::process::exit(1);
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// view-replay: delegates to verify::verify_chaintape (7-indicator report).
// ────────────────────────────────────────────────────────────────────────────

// ────────────────────────────────────────────────────────────────────────────
// TB-11 G3 carry-forward subcommands (TB-12 Atom 0.5b; architect 2026-05-03 §1.1).
// ────────────────────────────────────────────────────────────────────────────

/// `lean_market tick` — POLICY PREVIEW MODE (TB-11 carry-forward MVP).
///
/// **Architecture limitation note**: actual on-chain emission of TaskExpireTx
/// requires Sequencer reattachment to an existing chaintape, which requires
/// system_keypair persistence (not yet implemented; the chaintape factory
/// `build_chaintape_sequencer` fails-closed on non-empty repo per
/// NonEmptyRuntimeRepo gate). Until that infrastructure lands, `tick` runs in
/// **policy preview mode**: replays QState read-only, computes which tasks
/// would be expired by the architect §6.2 policy
/// (`tb11_emit_expire_for_eligible` eligibility logic mirrored), and prints
/// what WOULD be expired. Actual emission requires a session-attached
/// evaluator path (e.g. evaluator detects a tick env var pre-loop and emits
/// before the main solver loop), which is the next-TB wire-up task.
///
/// This subcommand satisfies architect §8 Atom 0.5 "lean_market tick"
/// existence requirement; the audit-gate documents the deferred actual-emit
/// path explicitly.
fn cmd_tick(args: &[String]) {
    let chaintape = arg_value(args, "--chaintape").unwrap_or_else(|| {
        eprintln!("lean_market tick: --chaintape <path> is required");
        std::process::exit(2);
    });
    let expiry_delta: u64 = arg_value(args, "--expiry-delta")
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000);
    let current_logical_t: u64 = arg_value(args, "--current-logical-t")
        .and_then(|s| s.parse().ok())
        .unwrap_or(u64::MAX); // default = "way past any deadline"
    let chaintape_path = PathBuf::from(&chaintape);
    let cas_path = derive_cas_path(&chaintape_path);
    match replay_qstate(&chaintape_path, &cas_path) {
        Ok((q, l4_count)) => {
            println!("[lean_market] tick (POLICY PREVIEW MODE — TB-11 carry-forward MVP)");
            println!("  chaintape         = {chaintape_path:?}");
            println!("  L4 entries        = {l4_count}");
            println!("  expiry_delta      = {expiry_delta} logical_t");
            println!("  current_logical_t = {current_logical_t}");
            println!();
            // Mirror tb11_emit_expire_for_eligible eligibility logic.
            use turingosv4::state::q_state::TaskMarketState;
            let mut eligible_count = 0u32;
            let mut total_would_refund: i64 = 0;
            for (task_id, entry) in q.economic_state_t.task_markets_t.0.iter() {
                let reason = match entry.state {
                    TaskMarketState::Open => "Deadline",
                    TaskMarketState::Bankrupt => "BankruptcyTriggered",
                    _ => continue,
                };
                let elapsed = current_logical_t.saturating_sub(entry.opened_at_logical_t);
                if elapsed <= expiry_delta {
                    continue;
                }
                let has_finalized = q.economic_state_t.claims_t.0.values().any(|c| {
                    c.task_id == *task_id
                        && c.status == turingosv4::state::q_state::ClaimStatus::Finalized
                });
                if has_finalized {
                    continue;
                }
                for escrow_tx_id in entry.escrow_lock_tx_ids.iter() {
                    if let Some(esc) = q.economic_state_t.escrows_t.0.get(escrow_tx_id) {
                        eligible_count += 1;
                        total_would_refund += esc.amount.micro_units();
                        println!(
                            "  ELIGIBLE: task_id={} escrow_tx_id={} sponsor={} amount={} micro reason={}",
                            task_id.0,
                            escrow_tx_id.0,
                            esc.depositor.0,
                            esc.amount.micro_units(),
                            reason,
                        );
                    }
                }
            }
            println!();
            println!("  Eligible escrow rows : {eligible_count}");
            println!("  Total micro to refund: {total_would_refund}");
            println!();
            println!("  ⚠ POLICY PREVIEW ONLY — NO TaskExpireTx emitted to chain.");
            println!("    Actual emission requires session-attached evaluator path");
            println!("    (Sequencer reattachment to existing chaintape needs");
            println!("    system_keypair persistence; deferred to next TB wire-up).");
        }
        Err(e) => {
            eprintln!("[lean_market] tick replay failed: {e}");
            std::process::exit(1);
        }
    }
}

/// `lean_market view-positions` — TB-12 Atom 4 (architect 2026-05-03 §8 Atom 4).
/// Read-only listing of NodePosition exposure records from chaintape replay.
///
/// Architect-mandated LABEL DISCIPLINE: "Exposure records", NOT "Open market
/// balances". TB-12 is exposure index, NOT trading market. NodePosition is
/// IMMUTABLE EXPOSURE RECORD per architect §10; NOT a Coin holding (CR-12.1);
/// NodePosition.amount NOT in total_supply_micro (CR-12.2). NO trading / price
/// / settlement in TB-12.
fn cmd_view_positions(args: &[String]) {
    let chaintape = arg_value(args, "--chaintape").unwrap_or_else(|| {
        eprintln!("lean_market view-positions: --chaintape <path> is required");
        std::process::exit(2);
    });
    let node_filter = arg_value(args, "--node-id");
    let owner_filter = arg_value(args, "--owner");
    let chaintape_path = PathBuf::from(&chaintape);
    let cas_path = derive_cas_path(&chaintape_path);
    match replay_qstate(&chaintape_path, &cas_path) {
        Ok((q, l4_count)) => {
            println!(
                "[lean_market] view-positions  (TB-12 Exposure records — NOT live market balances)"
            );
            println!("  chaintape    = {chaintape_path:?}");
            println!("  L4 entries   = {l4_count}");
            if let Some(n) = node_filter.as_ref() {
                println!("  filter       = node_id={n}");
            }
            if let Some(o) = owner_filter.as_ref() {
                println!("  filter       = owner={o}");
            }
            println!();

            let positions = &q.economic_state_t.node_positions_t.0;
            if positions.is_empty() {
                println!("  (no NodePosition exposure records on this chaintape)");
                return;
            }

            let mut total_long: i64 = 0;
            let mut total_short: i64 = 0;
            let mut shown: u32 = 0;
            for (pid, pos) in positions.iter() {
                if let Some(n) = node_filter.as_ref() {
                    if pos.node_id.0 != *n {
                        continue;
                    }
                }
                if let Some(o) = owner_filter.as_ref() {
                    if pos.owner.0 != *o {
                        continue;
                    }
                }
                shown += 1;
                let side_str = format!("{:?}", pos.side);
                let kind_str = format!("{:?}", pos.kind);
                if pos.side == turingosv4::state::typed_tx::PositionSide::Long {
                    total_long += pos.amount.micro_units();
                } else {
                    total_short += pos.amount.micro_units();
                }
                println!(
                    "  position_id={} node_id={} side={} kind={} owner={} amount={} micro task_id={} @round={}",
                    pid.0,
                    pos.node_id.0,
                    side_str,
                    kind_str,
                    pos.owner.0,
                    pos.amount.micro_units(),
                    pos.task_id.0,
                    pos.opened_at_round,
                );
            }
            println!();
            println!("  Total Long  : {total_long} micro");
            println!("  Total Short : {total_short} micro");
            println!(
                "  Net         : {} micro (long − short)",
                total_long - total_short
            );
            println!(
                "  Records     : {shown} (of {} total in QState)",
                positions.len()
            );
            println!();
            println!("  ⚠ Architect §10: these are IMMUTABLE EXPOSURE RECORDS, not active");
            println!("    position balances. TB-12 has no trading / price / settlement layer.");
            println!("    NodePosition.amount is NOT a Coin holding and is NOT counted in");
            println!("    total_supply_micro (CR-12.1 + CR-12.2 invariant).");
        }
        Err(e) => {
            eprintln!("[lean_market] view-positions replay failed: {e}");
            std::process::exit(1);
        }
    }
}

/// `lean_market view-bankruptcy` — read-only listing of TaskMarketState::Bankrupt
/// entries from chaintape replay.
fn cmd_view_bankruptcy(args: &[String]) {
    let chaintape = arg_value(args, "--chaintape").unwrap_or_else(|| {
        eprintln!("lean_market view-bankruptcy: --chaintape <path> is required");
        std::process::exit(2);
    });
    let chaintape_path = PathBuf::from(&chaintape);
    let cas_path = derive_cas_path(&chaintape_path);
    match replay_qstate(&chaintape_path, &cas_path) {
        Ok((q, l4_count)) => {
            println!("[lean_market] view-bankruptcy");
            println!("  chaintape    = {chaintape_path:?}");
            println!("  L4 entries   = {l4_count}");
            println!();
            use turingosv4::state::q_state::TaskMarketState;
            let mut bk_count = 0u32;
            for (task_id, entry) in q.economic_state_t.task_markets_t.0.iter() {
                if entry.state == TaskMarketState::Bankrupt {
                    bk_count += 1;
                    println!(
                        "  task_id={} sponsor={} bankruptcy_at_logical_t={} total_escrow_locked={} micro",
                        task_id.0,
                        entry.publisher.0,
                        entry.bankruptcy_at_logical_t.unwrap_or(0),
                        entry.total_escrow.micro_units(),
                    );
                }
            }
            println!();
            if bk_count == 0 {
                println!("  (no Bankrupt task entries on this chaintape)");
            } else {
                println!("  {} bankrupt task(s)", bk_count);
            }
        }
        Err(e) => {
            eprintln!("[lean_market] view-bankruptcy replay failed: {e}");
            std::process::exit(1);
        }
    }
}

fn cmd_view_replay(args: &[String]) {
    let chaintape = arg_value(args, "--chaintape").unwrap_or_else(|| {
        eprintln!("lean_market view-replay: --chaintape <path> is required");
        std::process::exit(2);
    });
    let chaintape_path = PathBuf::from(&chaintape);
    let cas_path = derive_cas_path(&chaintape_path);
    match turingosv4::runtime::verify::verify_chaintape(
        &chaintape_path,
        &cas_path,
        &turingosv4::runtime::verify::VerifyOptions::default(),
    ) {
        Ok(report) => {
            println!("[lean_market] view-replay");
            println!("  chaintape    = {chaintape_path:?}");
            println!("  L4 entries   = {}", report.l4_entries);
            println!("  L4.E entries = {}", report.l4e_entries);
            println!(
                "  ledger_root_verified                 = {}",
                report.ledger_root_verified
            );
            println!(
                "  system_signatures_verified           = {}",
                report.system_signatures_verified
            );
            println!(
                "  state_reconstructed                  = {}",
                report.state_reconstructed
            );
            println!(
                "  economic_state_reconstructed         = {}",
                report.economic_state_reconstructed
            );
            println!(
                "  cas_payloads_retrievable             = {}",
                report.cas_payloads_retrievable
            );
            println!(
                "  agent_signatures_verified            = {}",
                report.agent_signatures_verified
            );
            println!(
                "  proposal_telemetry_cas_retrievable   = {}",
                report.proposal_telemetry_cas_retrievable
            );
            println!();
            if report.all_indicators_pass() {
                println!("  ✓ all 7 indicators GREEN");
                std::process::exit(0);
            } else {
                println!("  ✗ one or more indicators FAILED");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("[lean_market] view-replay failed: {e}");
            std::process::exit(1);
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Helpers — chaintape replay + summary rendering.
// ────────────────────────────────────────────────────────────────────────────

fn derive_cas_path(chaintape_path: &Path) -> PathBuf {
    if let Ok(p) = std::env::var("TURINGOS_CAS_PATH") {
        return PathBuf::from(p);
    }
    chaintape_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(format!(
            "cas_{}",
            chaintape_path
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| "default".into())
        ))
}

/// Replay the chaintape read-only and return the reconstructed QState.
/// Mirrors the replay sequence in `runtime::verify::verify_chaintape` but
/// surfaces the QState instead of just booleans.
fn replay_qstate(runtime_repo: &Path, cas_path: &Path) -> Result<(QState, u64), String> {
    use turingosv4::bottom_white::ledger::system_keypair::{SystemEpoch, SystemPublicKey};
    let pinned_path = runtime_repo.join("pinned_pubkeys.json");
    if !pinned_path.exists() {
        return Err(format!(
            "missing pinned_pubkeys.json at {pinned_path:?} — chaintape may be empty or not bootstrapped"
        ));
    }
    let pinned_bytes = std::fs::read(&pinned_path).map_err(|e| format!("read pinned: {e}"))?;
    let manifest: turingosv4::runtime::PinnedPubkeyManifest =
        serde_json::from_slice(&pinned_bytes).map_err(|e| format!("parse pinned: {e}"))?;
    let mut pinned = PinnedSystemPubkeys::new();
    for entry in &manifest.pubkeys {
        let pubkey_bytes = hex::decode_32(&entry.pubkey_hex)
            .map_err(|e| format!("decode pubkey for epoch {}: {e}", entry.epoch))?;
        pinned.insert(
            SystemEpoch::new(entry.epoch),
            SystemPublicKey::from_bytes(pubkey_bytes),
        );
    }

    // Initial QState: load from disk if present, else genesis.
    let initial_q_path = runtime_repo.join("initial_q_state.json");
    let initial_q: QState = if initial_q_path.exists() {
        let bytes = std::fs::read(&initial_q_path).map_err(|e| format!("read initial_q: {e}"))?;
        serde_json::from_slice(&bytes).map_err(|e| format!("parse initial_q: {e}"))?
    } else {
        QState::genesis()
    };

    // Read all L4 entries.
    let writer =
        Git2LedgerWriter::open(runtime_repo).map_err(|e| format!("open ledger writer: {e}"))?;
    let total = writer.len();
    let mut entries = Vec::with_capacity(total as usize);
    for logical_t in 1..=total {
        let entry = writer
            .read_at(logical_t)
            .map_err(|e| format!("read_at({logical_t}): {e}"))?;
        entries.push(entry);
    }
    let l4_count = total;

    // Open CAS (read-only).
    let cas = CasStore::open(cas_path).map_err(|e| format!("open cas: {e}"))?;

    // Predicate + tool registries (empty — replay does not need them).
    let predicate_registry = PredicateRegistry::from_boot_manifest(turingosv4::top_white::predicates::registry::BootPredicateManifest::empty()).expect("empty predicate manifest");
    let tool_registry = ToolRegistry::new();

    let q = replay_full_transition(
        &initial_q,
        &entries,
        &cas,
        &pinned,
        &predicate_registry,
        &tool_registry,
    )
    .map_err(|e| format!("replay error: {e:?}"))?;

    Ok((q, l4_count))
}

/// Render a user-friendly summary of the user-sponsored tasks in the chain.
fn print_user_task_summary(q: &QState) {
    let user_tasks: Vec<_> = q
        .economic_state_t
        .task_markets_t
        .0
        .iter()
        .filter(|(_, e)| {
            e.publisher.0.starts_with("Agent_user_") || e.publisher.0 == DEFAULT_USER_SPONSOR
        })
        .collect();
    if user_tasks.is_empty() {
        println!("  user-sponsored tasks: 0  (no Agent_user_* in task_markets_t.publisher)");
        // Fallback: also dump non-user tasks for forensic value
        if !q.economic_state_t.task_markets_t.0.is_empty() {
            println!("  ALL task_markets_t entries:");
            for (tid, entry) in q.economic_state_t.task_markets_t.0.iter() {
                println!(
                    "    {:40} sponsor={:20} total_escrow={} micro",
                    tid.0,
                    entry.publisher.0,
                    entry.total_escrow.micro_units()
                );
            }
        }
        return;
    }
    println!("  user-sponsored tasks: {}", user_tasks.len());
    for (task_id, entry) in &user_tasks {
        println!();
        println!("  ── task {} ──", task_id.0);
        println!("    sponsor          = {}", entry.publisher.0);
        println!(
            "    total_escrow     = {} micro ({:.4} coin)",
            entry.total_escrow.micro_units(),
            entry.total_escrow.micro_units() as f64 / 1_000_000.0
        );
        // Find claims for this task: claims_t entries whose task_id matches.
        let claims_for_task: Vec<_> = q
            .economic_state_t
            .claims_t
            .0
            .iter()
            .filter(|(_, c)| &c.task_id == *task_id)
            .collect();
        if claims_for_task.is_empty() {
            println!("    claim status     = (none yet — solver has not Confirmed)");
        }
        for (claim_id, claim) in &claims_for_task {
            let status_label = match claim.status {
                ClaimStatus::Open => "Open",
                ClaimStatus::Finalized => "Finalized",
                ClaimStatus::Slashed => "Slashed",
            };
            println!("    claim_id         = {}", claim_id.0);
            println!("    claim status     = {status_label}");
            println!("    solver           = {}", claim.claimant.0);
            println!(
                "    claim amount     = {} micro ({:.4} coin)",
                claim.amount.micro_units(),
                claim.amount.micro_units() as f64 / 1_000_000.0
            );
            // Sponsor balance after this task.
            if let Some(bal) = q.economic_state_t.balances_t.0.get(&entry.publisher) {
                println!(
                    "    sponsor balance  = {} micro ({:.4} coin)",
                    bal.micro_units(),
                    bal.micro_units() as f64 / 1_000_000.0
                );
            }
            // Solver balance (post-payout if Finalized).
            if let Some(bal) = q.economic_state_t.balances_t.0.get(&claim.claimant) {
                println!(
                    "    solver balance   = {} micro ({:.4} coin)",
                    bal.micro_units(),
                    bal.micro_units() as f64 / 1_000_000.0
                );
            }
        }
    }
}

// Tiny self-contained hex decoder (avoids adding a hex crate dep).
mod hex {
    pub fn decode_32(s: &str) -> Result<[u8; 32], String> {
        let mut out = [0u8; 32];
        let bytes = s.as_bytes();
        if bytes.len() != 64 {
            return Err(format!("expected 64 hex chars, got {}", bytes.len()));
        }
        for i in 0..32 {
            let hi = hex_nybble(bytes[i * 2])?;
            let lo = hex_nybble(bytes[i * 2 + 1])?;
            out[i] = (hi << 4) | lo;
        }
        Ok(out)
    }
    fn hex_nybble(b: u8) -> Result<u8, String> {
        match b {
            b'0'..=b'9' => Ok(b - b'0'),
            b'a'..=b'f' => Ok(10 + b - b'a'),
            b'A'..=b'F' => Ok(10 + b - b'A'),
            _ => Err(format!("non-hex byte 0x{b:02x}")),
        }
    }
}
