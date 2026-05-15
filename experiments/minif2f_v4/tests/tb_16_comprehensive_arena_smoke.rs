//! TB-16 Atom 5 — `comprehensive_arena` smoke test.
//!
//! Verifies:
//! - The binary builds + responds to --help.
//! - --plan-only mode emits an ARENA_PLAN.md with the 6-task block,
//!   sandbox preseed manifest, and architect §7.7 halt-trigger summary.
//! - All 13 architect-required tx kinds are referenced in the emitted plan.
//!
//! Real-LLM end-to-end execution is exercised by Atom 6's
//! handover/tests/scripts/run_real_llm_arena.sh; that path is NOT
//! covered by this unit test (gated on LLM proxy availability + 30 min
//! wall clock).
//!
//! TRACE_MATRIX FC1-N36.

use std::path::PathBuf;
use std::process::Command;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn target_bin(name: &str) -> PathBuf {
    // The minif2f_v4 package builds binaries into the WORKSPACE target/
    // dir, not the package-local target/. Use parent traversal.
    let workspace_root = manifest_dir()
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| manifest_dir());
    let dbg = workspace_root.join("target").join("debug").join(name);
    if dbg.exists() {
        return dbg;
    }
    panic!("binary {name} not built at {dbg:?}");
}

#[test]
fn comprehensive_arena_help_succeeds() {
    let bin = target_bin("comprehensive_arena");
    let out = Command::new(&bin)
        .arg("--help")
        .output()
        .expect("comprehensive_arena --help");
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stderr),
        String::from_utf8_lossy(&out.stdout)
    );
    assert!(
        combined.contains("comprehensive_arena") && combined.contains("USAGE"),
        "help text malformed: {combined}"
    );
}

#[test]
fn comprehensive_arena_plan_only_emits_plan() {
    let bin = target_bin("comprehensive_arena");
    let out_dir = std::env::temp_dir().join(format!("tb16_arena_smoke_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out_dir);
    let status = Command::new(&bin)
        .arg("--out-dir")
        .arg(&out_dir)
        .arg("--plan-only")
        .status()
        .expect("comprehensive_arena run");
    assert!(status.success(), "comprehensive_arena exited {status:?}");
    let plan_path = out_dir.join("ARENA_PLAN.md");
    assert!(plan_path.exists(), "ARENA_PLAN.md missing at {plan_path:?}");

    let plan = std::fs::read_to_string(&plan_path).expect("read plan");

    // 6-task block present (binary's canonical engineered labels per
    // header docstring §3 Atom B + design §4.5).
    for label in [
        "task_A_happy_path",
        "task_B_challenge_released",
        "task_C_market_lifecycle",
        "task_D_exhaustion_bankruptcy_expire",
        "task_E_exhaustion_no_bankruptcy",
        "task_F_degraded_llm",
    ] {
        assert!(plan.contains(label), "task `{label}` missing from plan");
    }

    // 13 architect-required tx kinds referenced
    for tx_kind in [
        "Work",
        "Verify",
        "Challenge",
        "TaskOpen",
        "EscrowLock",
        "CompleteSetMint",
        "CompleteSetRedeem",
        "MarketSeed",
        "FinalizeReward",
        "ChallengeResolve",
        "TerminalSummary",
        "TaskExpire",
        "TaskBankruptcy",
    ] {
        assert!(
            plan.contains(tx_kind),
            "tx_kind `{tx_kind}` missing from plan"
        );
    }

    // Sandbox preseed sandbox-labeled
    assert!(plan.contains("tb7-7-sponsor"));
    assert!(plan.contains("Agent_solver_"));
    assert!(plan.contains("Agent_user_0"));

    // Architect §7.6 forbidden + §7.7 halt triggers section present
    assert!(plan.contains("Forbidden") || plan.contains("forbidden"));
    assert!(plan.contains("Halt trigger") || plan.contains("halt trigger"));
}
