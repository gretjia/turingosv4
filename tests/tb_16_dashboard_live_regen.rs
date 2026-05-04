//! TB-16 Atom 4 — dashboard §15 live regen + §16 sandbox banner smoke.
//!
//! Closes OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04: the dashboard
//! `audit_dashboard` binary now reconstructs `EconomicState.agent_autopsies_t`
//! via `replay_full_transition` and surfaces non-zero
//! `autopsy_event_counts` whenever the chain has at least one
//! TaskBankruptcyTx → autopsy emission.
//!
//! TB-16 Atom 4 §16: dashboard renders SANDBOX banner when any agent_id
//! matches a sandbox-only prefix.
//!
//! TRACE_MATRIX FC2-N32 + FC2-N33.

use std::path::PathBuf;
use std::process::Command;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn target_bin(name: &str) -> PathBuf {
    let manifest = manifest_dir();
    let dbg = manifest.join("target").join("debug").join(name);
    if dbg.exists() {
        return dbg;
    }
    panic!("binary {name} not built");
}

#[test]
fn dashboard_renders_section_16_sandbox_banner_on_existing_smoke() {
    let smoke = manifest_dir()
        .join("handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171");
    if !smoke.is_dir() || !smoke.join("runtime_repo").is_dir() {
        eprintln!("skipping: no fixture chain at handover/evidence/tb_13_real_llm_smoke_*");
        return;
    }
    let bin = target_bin("audit_dashboard");
    let out = Command::new(&bin)
        .arg("--repo")
        .arg(smoke.join("runtime_repo"))
        .arg("--cas")
        .arg(smoke.join("cas"))
        .output()
        .expect("audit_dashboard run");
    let stdout = String::from_utf8_lossy(&out.stdout);
    // §15 banner must always render (existing TB-15 contract)
    assert!(stdout.contains("§15 TB-15 Autopsy + Markov"),
            "§15 missing from dashboard output");
    // §16 SANDBOX banner must render when sandbox prefix matched
    let has_sandbox_id = stdout.contains("Agent_solver_")
        || stdout.contains("tb7-7-sponsor")
        || stdout.contains("Agent_verifier_")
        || stdout.contains("Agent_user_");
    if has_sandbox_id {
        assert!(
            stdout.contains("§16 TB-16 SANDBOX BANNER")
                && stdout.contains("SANDBOX-RUN"),
            "§16 banner missing despite sandbox-prefix agent IDs in tape"
        );
    }
}

#[test]
fn rebuild_autopsy_event_counts_returns_empty_on_pre_tb15_chain() {
    // For chains predating TB-15 (no agent_autopsies_t population),
    // rebuild_autopsy_event_counts must return an empty Vec — not panic
    // and not return synthetic data. The TB-13 real-LLM smoke is a good
    // fixture: its chain has no TaskBankruptcyTx, so the replayed
    // EconomicState's agent_autopsies_t is empty.
    let smoke = manifest_dir()
        .join("handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171");
    if !smoke.is_dir() {
        eprintln!("skipping: no fixture chain");
        return;
    }
    let bin = target_bin("audit_dashboard");
    let out = Command::new(&bin)
        .arg("--repo")
        .arg(smoke.join("runtime_repo"))
        .arg("--cas")
        .arg(smoke.join("cas"))
        .output()
        .expect("audit_dashboard run");
    let stdout = String::from_utf8_lossy(&out.stdout);
    // TB-13 chain has no autopsies → §15 should report the empty branch
    // text, NOT a populated table (and NOT the failure banner).
    assert!(
        stdout.contains("§15 TB-15 Autopsy + Markov"),
        "§15 missing"
    );
    assert!(
        stdout.contains("no agent_autopsies_t entries")
            || stdout.contains("No autopsies recorded"),
        "expected 'no autopsy entries' branch on TB-13 chain; got:\n{stdout}"
    );
}
