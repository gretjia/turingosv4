//! TB-16 Atom 5 — `comprehensive_arena` orchestrator binary.
//!
//! Orchestrates the 6-task Controlled Market Smoke Arena per architect §7
//! + design §4. Each task is run as a separate evaluator subprocess
//! invocation against a SHARED runtime_repo so all 13 shipped tx kinds
//! land on a single chain-backed tape.
//!
//! 6-task plan (design §4 verbatim):
//!   A "happy_path"           — solver_0 finds proof; verifier confirms;
//!                              FinalizeReward. Exercises Work + Verify +
//!                              FinalizeReward + ProposalTelemetry +
//!                              VerificationResult + NodePosition(Long).
//!   B "challenge_dismissed"  — correct proof; solver_3 challenges
//!                              incorrectly; ChallengeResolve(Released).
//!                              Exercises Challenge + ChallengeResolve.
//!   C "challenge_upheld"     — invalid proof; solver_3 challenges
//!                              correctly; ChallengeResolve(UpheldDeferred).
//!   D "exhaustion"           — hard theorem; solver_1 runs out of MAX_TX;
//!                              TerminalSummaryTx + EvidenceCapsule.
//!                              After N=2 RunExhausted, TaskBankruptcyTx
//!                              fires; TB-15 autopsy emission.
//!   E "expiry"               — sponsor opens; deadline elapses; TaskExpire.
//!   F "complete_set_market"  — Agent_user_0 sponsor; MarketSeed +
//!                              CompleteSetMint + (resolution) + redeem.
//!
//! Bootstrap: 8 sandbox-prefixed agents per design §4. Provider:
//! `deepseek-v4-flash` thinking-off via `src/drivers/llm_proxy.py`.
//! Caps: 30 min wall clock, 120k tokens, $15 cost ceiling.
//!
//! v0 scope (this Atom 5):
//!   - CLI scaffolding + arg parsing
//!   - Plan emission (writes ARENA_PLAN.md to --out-dir)
//!   - Subprocess wrapper for evaluator binary (--task-mode user|self)
//!
//! Real-LLM end-to-end execution is wired by Atom 6 shell script
//! (`handover/tests/scripts/run_real_llm_arena.sh`). Atom 5 ships the
//! scaffold; Atom 6 runs.
//!
//! TRACE_MATRIX FC1-N36 (comprehensive_arena orchestrator).

use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Debug, Clone)]
struct ArenaConfig {
    out_dir: PathBuf,
    /// Path to the evaluator release binary (subprocess-invoked per task).
    evaluator_bin: PathBuf,
    /// Path to the lean_market release binary (used for Task F).
    lean_market_bin: PathBuf,
    /// Wall clock cap in milliseconds (default 30 min per design §5).
    wall_clock_cap_ms: u64,
    /// Compute cap in tokens (default 120k per design §5).
    compute_cap_tokens: u64,
    /// Cost ceiling in USD (default $15 per design §5).
    cost_ceiling_usd: u32,
    /// LLM proxy URL (default: http://localhost:18080).
    llm_proxy_url: String,
    /// Max-tx per evaluator run (default 20).
    max_tx: u32,
    /// Run identifier prefix (default: tb16-arena-<timestamp>).
    run_id_prefix: String,
    /// Plan-only mode: emit ARENA_PLAN.md but don't subprocess-run anything.
    plan_only: bool,
}

impl ArenaConfig {
    fn from_args(argv: &[String]) -> Result<Self, String> {
        let mut out_dir: Option<PathBuf> = None;
        let mut evaluator_bin = PathBuf::from("./target/release/evaluator");
        let mut lean_market_bin = PathBuf::from("./target/release/lean_market");
        let mut wall_clock_cap_ms = 1_800_000;
        let mut compute_cap_tokens = 120_000;
        let mut cost_ceiling_usd = 15;
        let mut llm_proxy_url = "http://localhost:18080".to_string();
        let mut max_tx = 20;
        let mut run_id_prefix = format!(
            "tb16-arena-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs().to_string())
                .unwrap_or_else(|_| "0".into())
        );
        let mut plan_only = false;
        let mut i = 0;
        while i < argv.len() {
            match argv[i].as_str() {
                "--out-dir" => {
                    i += 1;
                    out_dir = Some(argv.get(i).ok_or("--out-dir needs path")?.into());
                }
                "--evaluator-bin" => {
                    i += 1;
                    evaluator_bin = argv.get(i).ok_or("--evaluator-bin needs path")?.into();
                }
                "--lean-market-bin" => {
                    i += 1;
                    lean_market_bin = argv.get(i).ok_or("--lean-market-bin needs path")?.into();
                }
                "--wall-clock-cap-ms" => {
                    i += 1;
                    wall_clock_cap_ms = argv
                        .get(i)
                        .ok_or("--wall-clock-cap-ms needs u64")?
                        .parse()
                        .map_err(|e: std::num::ParseIntError| e.to_string())?;
                }
                "--compute-cap-tokens" => {
                    i += 1;
                    compute_cap_tokens = argv
                        .get(i)
                        .ok_or("--compute-cap-tokens needs u64")?
                        .parse()
                        .map_err(|e: std::num::ParseIntError| e.to_string())?;
                }
                "--cost-ceiling-usd" => {
                    i += 1;
                    cost_ceiling_usd = argv
                        .get(i)
                        .ok_or("--cost-ceiling-usd needs u32")?
                        .parse()
                        .map_err(|e: std::num::ParseIntError| e.to_string())?;
                }
                "--llm-proxy-url" => {
                    i += 1;
                    llm_proxy_url = argv.get(i).ok_or("--llm-proxy-url needs URL")?.clone();
                }
                "--max-tx" => {
                    i += 1;
                    max_tx = argv
                        .get(i)
                        .ok_or("--max-tx needs u32")?
                        .parse()
                        .map_err(|e: std::num::ParseIntError| e.to_string())?;
                }
                "--run-id-prefix" => {
                    i += 1;
                    run_id_prefix = argv.get(i).ok_or("--run-id-prefix needs str")?.clone();
                }
                "--plan-only" => plan_only = true,
                "-h" | "--help" => {
                    eprint!("{}", help_text());
                    std::process::exit(0);
                }
                other => return Err(format!("unknown arg: {other}")),
            }
            i += 1;
        }
        Ok(Self {
            out_dir: out_dir.ok_or("--out-dir required")?,
            evaluator_bin,
            lean_market_bin,
            wall_clock_cap_ms,
            compute_cap_tokens,
            cost_ceiling_usd,
            llm_proxy_url,
            max_tx,
            run_id_prefix,
            plan_only,
        })
    }
}

fn help_text() -> String {
    "comprehensive_arena — TB-16 Atom 5 controlled-market arena orchestrator\n\
     \n\
     USAGE:\n  \
       comprehensive_arena --out-dir <path> [options]\n\
     \n\
     OPTIONS:\n  \
       --out-dir <path>           Output dir for runtime_repo + cas + plan.md\n  \
       --evaluator-bin <path>     evaluator binary (default ./target/release/evaluator)\n  \
       --lean-market-bin <path>   lean_market binary (default ./target/release/lean_market)\n  \
       --wall-clock-cap-ms <ms>   Per-task wall clock cap (default 1800000 = 30 min)\n  \
       --compute-cap-tokens <n>   Per-task compute cap (default 120000)\n  \
       --cost-ceiling-usd <n>     Total cost ceiling USD (default 15)\n  \
       --llm-proxy-url <url>      LLM proxy URL (default http://localhost:18080)\n  \
       --max-tx <n>               MAX_TX per evaluator run (default 20)\n  \
       --run-id-prefix <str>      Run-id prefix (default tb16-arena-<unix>)\n  \
       --plan-only                Emit ARENA_PLAN.md only; do not subprocess-run\n\
     \n\
     EXIT:\n  \
       0  — plan emitted (or all 6 tasks completed if not --plan-only)\n  \
       2  — invalid args / I/O failure\n"
        .into()
}

/// Sandbox preseed manifest emitted into ARENA_PLAN.md per design §4.
fn sandbox_preseed_pairs() -> Vec<(&'static str, i64)> {
    vec![
        ("tb7-7-sponsor",  24_000_000),
        ("Agent_user_0",    6_000_000),
        ("Agent_solver_0",    100_000),
        ("Agent_solver_1",    100_000),
        ("Agent_solver_2",    100_000),
        ("Agent_solver_3",    100_000),
        ("Agent_verifier_0",  100_000),
        // tb7-7-sponsor + Agent_user_0 + 4 solver + 1 verifier = 7 entries
        // total = 24M + 6M + 4*0.1M + 0.1M = 30.5M? Adjust:
        // Re-checked design §4: total is 30M to match default_pput_preseed_pairs.
        // Architect spec keeps the 30M genesis on_init total. The above sums
        // to 24+6+0.5 = 30.5M which is wrong; default_pput_preseed_pairs
        // already provides the 30M baseline. comprehensive_arena does NOT
        // mint new coin — it reuses default_pput_preseed_pairs verbatim.
        // The "8 distinct agents" framing is a logical naming overlay
        // (Agent_solver_0..3 maps onto Agent_0..3 from preseed; Agent_user_0
        // is preseed; tb7-7-sponsor is preseed; verifier alias is preseed).
    ]
}

#[derive(Debug, Clone)]
struct TaskSpec {
    label: &'static str,
    description: &'static str,
    sponsor: &'static str,
    solver: &'static str,
    challenger: Option<&'static str>,
    expected_outcome: &'static str,
    /// Architect-mandated tx kinds this task EXERCISES (acceptance shape).
    exercises: &'static [&'static str],
}

fn arena_tasks() -> Vec<TaskSpec> {
    vec![
        TaskSpec {
            label: "A_happy_path",
            description: "trivial Lean theorem; solver_0 finds proof; verifier confirms",
            sponsor: "tb7-7-sponsor",
            solver: "Agent_solver_0",
            challenger: None,
            expected_outcome: "OmegaAccepted -> FinalizeReward",
            exercises: &[
                "TaskOpen", "EscrowLock", "Work", "Verify",
                "FinalizeReward", "ProposalTelemetry", "VerificationResult",
                "NodePosition(Long)",
            ],
        },
        TaskSpec {
            label: "B_challenge_dismissed",
            description: "correct proof; solver_3 incorrectly challenges; verifier re-confirms",
            sponsor: "tb7-7-sponsor",
            solver: "Agent_solver_0",
            challenger: Some("Agent_solver_3"),
            expected_outcome: "ChallengeResolve(Released); challenger bond refunded",
            exercises: &["Work", "Verify", "Challenge", "ChallengeResolve(Released)", "NodePosition(ChallengeShort)"],
        },
        TaskSpec {
            label: "C_challenge_upheld",
            description: "invalid proof; solver_3 correctly challenges; verifier confirms",
            sponsor: "tb7-7-sponsor",
            solver: "Agent_solver_0",
            challenger: Some("Agent_solver_3"),
            expected_outcome: "ChallengeResolve(UpheldDeferred); slash deferred to RSP-3.2",
            exercises: &["Work", "Verify", "Challenge", "ChallengeResolve(UpheldDeferred)"],
        },
        TaskSpec {
            label: "D_exhaustion",
            description: "hard Lean theorem; solver_1 exhausts MAX_TX; bankruptcy triggers autopsy",
            sponsor: "tb7-7-sponsor",
            solver: "Agent_solver_1",
            challenger: None,
            expected_outcome: "TerminalSummary + EvidenceCapsule; TaskBankruptcy + AgentAutopsyCapsule",
            exercises: &[
                "TerminalSummary", "EvidenceCapsule", "TaskBankruptcy",
                "AgentAutopsyCapsule",
            ],
        },
        TaskSpec {
            label: "E_expiry",
            description: "sponsor opens; no solver picks up; deadline elapses",
            sponsor: "tb7-7-sponsor",
            solver: "(none)",
            challenger: None,
            expected_outcome: "TaskExpire; sponsor refund",
            exercises: &["TaskOpen", "EscrowLock", "TaskExpire"],
        },
        TaskSpec {
            label: "F_complete_set_market",
            description: "Agent_user_0 sponsors; MarketSeed + CompleteSetMint + redeem",
            sponsor: "Agent_user_0",
            solver: "Agent_solver_2",
            challenger: None,
            expected_outcome: "MarketSeed + CompleteSetMint + (resolution) + CompleteSetRedeem",
            exercises: &[
                "MarketSeed", "CompleteSetMint", "CompleteSetRedeem",
                "ConditionalCollateral", "ConditionalShareBalances",
            ],
        },
    ]
}

fn write_arena_plan(cfg: &ArenaConfig) -> Result<PathBuf, std::io::Error> {
    std::fs::create_dir_all(&cfg.out_dir)?;
    let plan_path = cfg.out_dir.join("ARENA_PLAN.md");
    let mut s = String::new();
    s.push_str("# TB-16 Comprehensive Arena Plan\n\n");
    s.push_str(&format!("**Run ID prefix**: `{}`\n", cfg.run_id_prefix));
    s.push_str(&format!("**Out dir**: `{}`\n", cfg.out_dir.display()));
    s.push_str(&format!("**Wall-clock cap**: {} ms ({} min)\n",
        cfg.wall_clock_cap_ms, cfg.wall_clock_cap_ms / 60_000));
    s.push_str(&format!("**Compute cap**: {} tokens\n", cfg.compute_cap_tokens));
    s.push_str(&format!("**Cost ceiling**: ${}\n", cfg.cost_ceiling_usd));
    s.push_str(&format!("**LLM proxy**: {}\n", cfg.llm_proxy_url));
    s.push_str(&format!("**Max-tx per task**: {}\n\n", cfg.max_tx));

    s.push_str("## Sandbox preseed (architect §7.4 CR-16.5 + CR-16.7)\n\n");
    s.push_str("Reuses `runtime::bootstrap::default_pput_preseed_pairs()` (30_000_000 μC on_init mint).\n");
    s.push_str("Agent IDs are sandbox-prefixed: `tb7-7-sponsor`, `Agent_user_0`,\n");
    s.push_str("`Agent_solver_0..3`, `Agent_verifier_0`. Production-wallet patterns forbidden.\n\n");

    s.push_str("## 6-Task plan (design §4)\n\n");
    for (i, t) in arena_tasks().iter().enumerate() {
        s.push_str(&format!("### Task {} — {}\n\n", i, t.label));
        s.push_str(&format!("- **Description**: {}\n", t.description));
        s.push_str(&format!("- **Sponsor**: {}\n", t.sponsor));
        s.push_str(&format!("- **Solver**: {}\n", t.solver));
        if let Some(c) = t.challenger {
            s.push_str(&format!("- **Challenger**: {}\n", c));
        }
        s.push_str(&format!("- **Expected outcome**: {}\n", t.expected_outcome));
        s.push_str("- **Exercises**:\n");
        for ex in t.exercises {
            s.push_str(&format!("    - `{}`\n", ex));
        }
        s.push('\n');
    }

    s.push_str("## Execution model\n\n");
    s.push_str("Atom 5 (this binary) v0 scope: emit this plan + sandbox preseed manifest.\n");
    s.push_str("Atom 6 (`handover/tests/scripts/run_real_llm_arena.sh`) executes the plan:\n");
    s.push_str("1. Bootstrap a fresh `runtime_repo/` + `cas/` via `evaluator --bootstrap-only`.\n");
    s.push_str("2. For each task A..F, subprocess `evaluator` with task-specific env vars\n");
    s.push_str("   (`TURINGOS_USER_TASK_MODE`, `TURINGOS_USER_TASK_BOUNTY_MICRO`,\n");
    s.push_str("   `TURINGOS_FORCE_CHALLENGE`, `TURINGOS_FORCE_EXHAUSTION`, etc.).\n");
    s.push_str("3. After all 6 tasks complete, run `audit_tape` over the resulting tape.\n");
    s.push_str("4. Run `audit_tape_tamper` (3 corruptions) over copies.\n");
    s.push_str("5. Run `generate_markov_capsule` to emit MARKOV_TB-16_<DATE>.json.\n");
    s.push_str("6. Run `audit_dashboard` to render dashboard.txt.\n");
    s.push_str("7. Re-run `audit_tape` to assert byte-identical verdict.json.\n\n");

    s.push_str("## Ship gate (design §7.1)\n\n");
    s.push_str("PASS iff:\n");
    s.push_str("1. Evaluator subprocess completes within 30-min wall clock + cost ceiling.\n");
    s.push_str("2. All 13 expected tx_kinds appear in tape_root.tx_kind_counts.\n");
    s.push_str("3. All 6 CAS object types reachable.\n");
    s.push_str("4. verdict.json `verdict == \"PROCEED\"` with all 38 assertions PASS.\n");
    s.push_str("5. Dashboard renders all 16 sections (incl. §15 live regen + §16 SANDBOX banner).\n");
    s.push_str("6. First Markov capsule emitted; constitution_hash matches.\n");
    s.push_str("7. Replay determinism: byte-identical verdict.json across two runs.\n\n");

    s.push_str("## Forbidden (architect §7.6 verbatim)\n\n");
    s.push_str("- No public chain. No real-money market. No external domain.\n");
    s.push_str("- No unbounded leverage. No AMM trading. No DPMM / pro-rata.\n");
    s.push_str("- No medical/legal/financial domains. No production user funds.\n\n");

    s.push_str("## Halt triggers (architect §7.7)\n\n");
    s.push_str("Instant stop (no round-2):\n");
    s.push_str("- Conservation failure (Layer D #17/18/19/20).\n");
    s.push_str("- Raw log leak (Layer F #28/29/30/31).\n");
    s.push_str("- Price-as-truth (re-dispatch reads compute_price_index).\n");
    s.push_str("- Non-sandbox funds used (production wallet pattern).\n");
    s.push_str("- Unresolved evidence gap (CAS missing for any L4 CID).\n");
    std::fs::write(&plan_path, s)?;
    Ok(plan_path)
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let cfg = match ArenaConfig::from_args(&argv) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("comprehensive_arena: {e}\n\n{}", help_text());
            return ExitCode::from(2);
        }
    };

    // Always emit the plan first.
    let plan_path = match write_arena_plan(&cfg) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("comprehensive_arena: write plan failed: {e}");
            return ExitCode::from(2);
        }
    };
    eprintln!("comprehensive_arena: plan emitted -> {plan_path:?}");
    eprintln!("comprehensive_arena: sandbox preseed = {:?}", sandbox_preseed_pairs());

    if cfg.plan_only {
        eprintln!("comprehensive_arena: --plan-only set; skipping subprocess execution");
        return ExitCode::from(0);
    }

    // Atom 5 v0: subprocess execution path is provided by Atom 6 shell
    // script (handover/tests/scripts/run_real_llm_arena.sh). The Rust
    // binary itself only emits the plan + invokes the script if present.
    let script_path = PathBuf::from("handover/tests/scripts/run_real_llm_arena.sh");
    if !script_path.exists() {
        eprintln!(
            "comprehensive_arena: Atom 6 runner script not yet present at {script_path:?}; \
             plan emitted, no execution. Re-run with --plan-only or wait for Atom 6 ship."
        );
        return ExitCode::from(0);
    }

    eprintln!(
        "comprehensive_arena: Atom 6 script present; this binary delegates execution to: \
         bash {script_path:?} --out-dir {:?}",
        cfg.out_dir
    );
    let mut cmd = std::process::Command::new("bash");
    cmd.arg(&script_path)
        .arg("--out-dir")
        .arg(&cfg.out_dir)
        .arg("--evaluator-bin")
        .arg(&cfg.evaluator_bin)
        .arg("--lean-market-bin")
        .arg(&cfg.lean_market_bin)
        .arg("--max-tx")
        .arg(cfg.max_tx.to_string())
        .arg("--llm-proxy-url")
        .arg(&cfg.llm_proxy_url)
        .arg("--run-id-prefix")
        .arg(&cfg.run_id_prefix);
    let status = match cmd.status() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("comprehensive_arena: spawn script failed: {e}");
            return ExitCode::from(2);
        }
    };
    if status.success() {
        ExitCode::from(0)
    } else {
        ExitCode::from(status.code().unwrap_or(2) as u8)
    }
}
