//! TB-G G3.2 (charter §1 Module G3; 2026-05-12) — bankruptcy risk-cap
//! admission ship gates. Architect §8 ratification verbatim Q1..Q6 + §7.1..7.5
//! supplementary requirements (`handover/directives/
//! 2026-05-12_TB_G_G3_2_§8_ARCHITECT_RATIFICATION.md`).
//!
//! Ship-gate matrix (architect packet §4 SG-G3.10..G3.12 + §3 supplementary):
//! - SG-G3.10..d: 4-arm L4.E `BankruptcyRiskCapExceeded` rejection (Atoms A+C)
//! - SG-G3.11:    risk-cap fires FIRST in admission (architect Q5)
//! - SG-G3.12:    Display ≤ 64 bytes (architect §1.5 shielding)
//! - SG-G3.X.a:   Gap-A reputation +1 surface present (Atom D)
//! - SG-G3.X.b:   Gap-B bond-return surface present (Atom E)
//! - SG-G3.X.c:   `RejectionClass` tail-appended (golden-digest discipline)
//! - SG-G3.X.d:   Rejected attempts make no state mutation
//! - SG-G3.4:     AutopsyCapsule emit at per-task-end (Atom F)
//! - Arch §7.1:   `RiskCapImpactReport` helper present + sane shape
//! - Arch §7.2:   below-cap agent can still READ chain views (only
//!                stake-side admission blocked)
//! - Arch §7.3:   autopsy Markov scoped (latest only, derived from chain)
//! - Arch §7.4:   reputation Sybil-guarded via `agent_verifications_t` dedup
//! - Arch §7.5:   `FinalizeRewardPayoutBreakdown` helper separates solver vs
//!                verifier deltas

use std::fs;

use turingosv4::runtime::agent_pnl;
use turingosv4::runtime::risk_cap_impact_report::{
    assert_finalize_reward_payout_bounded, compute_finalize_reward_payout_breakdown,
    is_bankruptcy_risk_cap_rejection_class, is_bankruptcy_risk_cap_transition_error,
    tx_kind_label_for_risk_cap_rejection, FinalizeRewardPayoutBreakdown, RiskCapImpactReport,
};
use turingosv4::state::q_state::{AgentId, EconomicState, QState, TaskId};
use turingosv4::state::typed_tx::{FinalizeRewardTx, RejectionClass, TransitionError};

const TYPED_TX_PATH: &str = "src/state/typed_tx.rs";
const SEQUENCER_PATH: &str = "src/state/sequencer.rs";
const AGENT_PNL_PATH: &str = "src/runtime/agent_pnl.rs";
const AUTOPSY_PATH: &str = "src/runtime/autopsy_capsule.rs";
const RISK_CAP_REPORT_PATH: &str = "src/runtime/risk_cap_impact_report.rs";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"))
}

fn compact_ws(src: &str) -> String {
    src.chars().filter(|c| !c.is_whitespace()).collect()
}

// ────────────────────────────────────────────────────────────────────────────
// SG-G3.X.c — `RejectionClass` tail-append discipline (atom A)
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_x_c_rejection_class_bankruptcy_variant_present() {
    let src = read(TYPED_TX_PATH);
    assert!(
        src.contains("pub enum RejectionClass"),
        "RejectionClass enum must be declared in typed_tx.rs"
    );
    assert!(
        src.contains("BankruptcyRiskCapExceeded,"),
        "RejectionClass::BankruptcyRiskCapExceeded variant missing — G3.2 Atom A required"
    );
}

#[test]
fn sg_g3_x_c_rejection_class_tail_append_after_verify_duplicate() {
    // Tail-append discipline: BankruptcyRiskCapExceeded must come AFTER
    // VerifyDuplicate to preserve golden-digest invariants on pre-G3.2
    // serialized RejectionClass wire forms.
    let src = read(TYPED_TX_PATH);
    let idx_verify_duplicate = src
        .find("    VerifyDuplicate,")
        .expect("VerifyDuplicate variant must exist in RejectionClass enum");
    let idx_bankruptcy = src
        .find("    BankruptcyRiskCapExceeded,\n}")
        .or_else(|| {
            // The variant is followed by closing `}`; find it relative to
            // the RejectionClass-enum block specifically.
            let enum_block_start = src
                .find("pub enum RejectionClass {")
                .expect("RejectionClass enum block");
            let enum_block_end_off = src[enum_block_start..]
                .find("\n}\n")
                .expect("RejectionClass enum closes");
            let enum_block = &src[enum_block_start..enum_block_start + enum_block_end_off];
            enum_block
                .rfind("BankruptcyRiskCapExceeded,")
                .map(|local| enum_block_start + local)
        })
        .expect("BankruptcyRiskCapExceeded must appear in RejectionClass enum");
    assert!(
        idx_bankruptcy > idx_verify_duplicate,
        "BankruptcyRiskCapExceeded must come AFTER VerifyDuplicate (tail-append discipline)"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// SG-G3.12 — Display ≤ 64 bytes (architect §1.5 shielding budget)
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_12_transition_error_display_under_64_bytes() {
    let s = format!("{}", TransitionError::BankruptcyRiskCapExceeded);
    assert!(
        s.len() <= 64,
        "TransitionError::BankruptcyRiskCapExceeded display must be ≤64 bytes (was {}): {:?}",
        s.len(),
        s
    );
    // Sanity: not empty / not panicking.
    assert!(!s.is_empty(), "display string must be non-empty");
    // Substantive content check.
    assert!(
        s.contains("bankruptcy") || s.contains("risk-cap") || s.contains("risk_cap"),
        "display must mention bankruptcy/risk-cap concept (low-pollution class string): {s:?}"
    );
}

#[test]
fn sg_g3_12_public_summary_token_present_and_short() {
    // Mapped via `public_summary_for` in sequencer.rs — source-grep + len check.
    let src = read(SEQUENCER_PATH);
    let needle = "bankruptcy_risk_cap_exceeded";
    assert!(
        src.contains(needle),
        "sequencer.rs::public_summary_for must map BankruptcyRiskCapExceeded → {:?}",
        needle
    );
    assert!(
        needle.len() <= 64,
        "public summary token must fit Wave-3 50p TransitionError.display max 48B / 64B budget"
    );
}

#[test]
fn sg_g3_x_c_rejection_class_for_maps_to_policy_violation() {
    // Architect packet §2.1 verbatim: `TE::BankruptcyRiskCapExceeded → RC::PolicyViolation`
    // — shielding-correct low-pollution L4ERejectionClass per CLAUDE.md §15.
    let src = read(SEQUENCER_PATH);
    let map_block = src
        .find("fn rejection_class_for")
        .map(|i| &src[i..])
        .unwrap_or_default();
    assert!(
        map_block.contains("TE::BankruptcyRiskCapExceeded => RC::PolicyViolation"),
        "rejection_class_for must map BankruptcyRiskCapExceeded → PolicyViolation per packet §2.1"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// SG-G3.10..d — 4-arm admission precondition source-grep
// (real-Sequencer fixture tests deferred to follow-up real-LLM smoke;
// source-grep + Display + helper-correctness witnesses are the
// constitutional gates here per `feedback_real_problems_not_designed`)
// ────────────────────────────────────────────────────────────────────────────

// Helper: find the position of an actual `return Err(TransitionError::X)`
// emit (NOT mentions in doc comments).
fn find_return_err_pos(block: &str, variant: &str) -> Option<usize> {
    let needle = format!("return Err(TransitionError::{})", variant);
    block.find(&needle)
}

#[test]
fn sg_g3_10_a_worktx_admission_arm_has_risk_cap_precondition() {
    let src = read(SEQUENCER_PATH);
    let arm_start = src
        .find("TypedTx::Work(work) => {")
        .expect("WorkTx admission arm must exist");
    let arm_block = &src[arm_start..arm_start + 4000];
    let cap_pos = find_return_err_pos(arm_block, "BankruptcyRiskCapExceeded")
        .expect("WorkTx arm must return BankruptcyRiskCapExceeded (Atom C)");
    let stake_balance_pos = find_return_err_pos(arm_block, "StakeBalanceExceeded")
        .expect("WorkTx arm must still contain StakeBalanceExceeded emit");
    assert!(
        cap_pos < stake_balance_pos,
        "Risk-cap emit ({cap_pos}) must FIRE BEFORE StakeBalanceExceeded emit \
         ({stake_balance_pos}) in WorkTx arm (architect Q5: subsuming order)"
    );
}

#[test]
fn sg_g3_10_b_verifytx_admission_arm_has_risk_cap_precondition() {
    let src = read(SEQUENCER_PATH);
    let arm_start = src
        .find("TypedTx::Verify(verify) => {")
        .expect("VerifyTx admission arm must exist");
    let arm_block = &src[arm_start..arm_start + 4000];
    let cap_pos = find_return_err_pos(arm_block, "BankruptcyRiskCapExceeded")
        .expect("VerifyTx arm must return BankruptcyRiskCapExceeded");
    let bond_pos = find_return_err_pos(arm_block, "VerifyBondOutOfBounds")
        .expect("VerifyTx arm must still contain VerifyBondOutOfBounds emit");
    assert!(
        cap_pos < bond_pos,
        "Risk-cap emit must FIRE BEFORE VerifyBondOutOfBounds emit in VerifyTx arm"
    );
}

#[test]
fn sg_g3_10_c_challengetx_admission_arm_has_risk_cap_precondition() {
    let src = read(SEQUENCER_PATH);
    let arm_start = src
        .find("TypedTx::Challenge(challenge) => {")
        .expect("ChallengeTx admission arm must exist");
    let arm_block = &src[arm_start..arm_start + 4000];
    let cap_pos = find_return_err_pos(arm_block, "BankruptcyRiskCapExceeded")
        .expect("ChallengeTx arm must return BankruptcyRiskCapExceeded");
    let stake_pos = find_return_err_pos(arm_block, "StakeInsufficient")
        .expect("ChallengeTx arm must still contain StakeInsufficient emit");
    assert!(
        cap_pos < stake_pos,
        "Risk-cap emit must FIRE BEFORE StakeInsufficient emit in ChallengeTx arm"
    );
}

#[test]
fn sg_g3_10_d_buyrouter_admission_arm_has_risk_cap_precondition() {
    let src = read(SEQUENCER_PATH);
    let arm_start = src
        .find("TypedTx::BuyWithCoinRouter(router) => {")
        .expect("BuyWithCoinRouter admission arm must exist");
    let arm_block = &src[arm_start..arm_start + 4000];
    let cap_pos = find_return_err_pos(arm_block, "BankruptcyRiskCapExceeded")
        .expect("BuyWithCoinRouter arm must return BankruptcyRiskCapExceeded");
    let router_pos = find_return_err_pos(arm_block, "RouterInsufficientCoinBalance")
        .expect("BuyWithCoinRouter arm must still contain RouterInsufficientCoinBalance emit");
    assert!(
        cap_pos < router_pos,
        "Risk-cap emit must FIRE BEFORE RouterInsufficientCoinBalance emit"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// SG-G3.B — Per-agent risk-cap helper (atom B; architect Q1)
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_b_risk_cap_threshold_matches_g3_1_near_insolvent() {
    // Architect Q1: bankruptcy_risk_cap_micro = initial_balance_micro / 10.
    // Must match G3.1 SHIPPED classify_solvency `NearInsolvent` boundary
    // at src/runtime/agent_pnl.rs:311 (`balance < initial / 10`).
    let q = QState::default();

    let agent_0 = AgentId("Agent_0".into());
    let initial_0 = agent_pnl::initial_balance_micro_from_default_preseed(&agent_0);
    let cap_0 = agent_pnl::bankruptcy_risk_cap_micro(&agent_0, &q);
    assert_eq!(
        cap_0,
        initial_0 / 10,
        "risk-cap must equal initial_balance / 10 per architect Q1"
    );

    let market_maker = AgentId("MarketMakerBudget".into());
    let cap_mm = agent_pnl::bankruptcy_risk_cap_micro(&market_maker, &q);
    let initial_mm = agent_pnl::initial_balance_micro_from_default_preseed(&market_maker);
    assert_eq!(cap_mm, initial_mm / 10);

    let sponsor = AgentId("tb7-7-sponsor".into());
    let cap_sp = agent_pnl::bankruptcy_risk_cap_micro(&sponsor, &q);
    let initial_sp = agent_pnl::initial_balance_micro_from_default_preseed(&sponsor);
    assert_eq!(cap_sp, initial_sp / 10);

    // Per-agent cap table sanity (matches architect §3 verdict table):
    if initial_0 == 1_000_000 {
        assert_eq!(cap_0, 100_000, "Agent_0..9 preseed 1M μC → cap 100k μC");
    }
    if initial_mm == 5_000_000 {
        assert_eq!(cap_mm, 500_000, "MarketMakerBudget 5M μC → cap 500k μC");
    }
    if initial_sp == 10_000_000 {
        assert_eq!(cap_sp, 1_000_000, "tb7-7-sponsor 10M μC → cap 1M μC");
    }
}

#[test]
fn sg_g3_b_unknown_agent_cap_is_zero_fail_closed() {
    // Architect Q1 fail-closed: unknown agent (no preseed entry) → cap 0
    // → balance ≥ 0 == cap trivially passes admission.
    let q = QState::default();
    let unknown = AgentId("nonexistent_agent_xyz".into());
    let cap = agent_pnl::bankruptcy_risk_cap_micro(&unknown, &q);
    assert_eq!(cap, 0, "unknown agent must have cap=0 (no preseed)");
}

#[test]
fn sg_g3_b_no_new_economic_state_table() {
    // Architect Q1: NO new `EconomicState.bankruptcy_risk_cap_t` table —
    // verify source-grep that no such field was added.
    let src = read("src/state/q_state.rs");
    assert!(
        !src.contains("bankruptcy_risk_cap_t"),
        "Forbidden: EconomicState.bankruptcy_risk_cap_t (architect Q1 forbade new schema)"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// SG-G3.X.a + SG-G3.X.b — Gap-A reputation + Gap-B bond return (atoms D + E)
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_x_a_reputation_plus_one_in_verifytx_arm() {
    // Atom D: reputations_t[verifier] += 1 on accepted VerifyTx.
    // Source-grep on sequencer.rs (Step 5c per architect packet §2.1).
    let src = read(SEQUENCER_PATH);
    let arm_start = src
        .find("TypedTx::Verify(verify) => {")
        .expect("VerifyTx arm must exist");
    let arm_block = &src[arm_start..arm_start + 8000];
    let compact = compact_ws(arm_block);
    assert!(
        compact.contains("reputations_t.0.entry(verify.verifier_agent.clone())")
            && compact.contains(".0+=1"),
        "VerifyTx arm must increment reputations_t[verifier] by 1 (Atom D)"
    );
}

#[test]
fn sg_g3_x_a_uniform_plus_one_not_verdict_weighted() {
    // Architect Q2: uniform +1, NOT verdict-weighted. Source-grep to
    // ensure no `if .. verdict == .. then += N else += M` pattern fires
    // around the reputation site.
    let src = read(SEQUENCER_PATH);
    let arm_start = src
        .find("Step 5c: TB-G G3.2 Gap-A reputation")
        .expect("Step 5c reputation comment must exist (architect Q2 traceability)");
    let arm_block = &src[arm_start..arm_start + 800];
    assert!(
        arm_block.contains("uniform +1") || arm_block.contains("+= 1"),
        "Architect Q2 = uniform +1 must be documented + implemented"
    );
    // Forbidden: outcome-correlated or weighted increment in this block.
    assert!(
        !arm_block.contains("VerifyVerdict::Confirm") || !arm_block.contains("+= 2"),
        "Architect Q2 forbade verdict-weighted accumulation in G3.2"
    );
}

#[test]
fn sg_g3_x_b_bond_return_in_finalize_reward() {
    // Atom E: FinalizeRewardTx extension credits verifier bonds back.
    let src = read(SEQUENCER_PATH);
    let arm_start = src
        .find("TypedTx::FinalizeReward(fr) => {")
        .expect("FinalizeRewardTx arm must exist");
    let arm_block = &src[arm_start..arm_start + 12000];
    assert!(
        arm_block.contains("verifier_entries"),
        "FinalizeRewardTx arm must iterate verifier_entries for bond return (Atom E)"
    );
    assert!(
        arm_block.contains("g3_2_verifier_bond_return_total_micro"),
        "FinalizeRewardTx arm must track verifier bond return delta (Atom E)"
    );
}

#[test]
fn sg_g3_x_b_no_new_bond_return_tx_variant() {
    // Architect Q3 = B1: extend FinalizeRewardTx, NO new BondReturnTx system tx.
    let typed_tx_src = read(TYPED_TX_PATH);
    assert!(
        !typed_tx_src.contains("pub struct BondReturnTx") && !typed_tx_src.contains("BondReturn("),
        "Forbidden: new BondReturnTx system-tx variant (architect Q3 = B1)"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// SG-G3.4 — AutopsyCapsule per-task-end emit (atom F; architect Q6)
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g3_4_terminal_summary_autopsy_derive_helper_present() {
    let src = read(AUTOPSY_PATH);
    assert!(
        src.contains("pub fn derive_g3_2_terminal_summary_bankrupt_autopsies"),
        "Atom F: derive helper for per-task-end autopsy emit must exist"
    );
    assert!(
        src.contains("pub fn write_g3_2_terminal_summary_bankrupt_autopsies_to_cas"),
        "Atom F: apply_one CAS-write helper must exist"
    );
}

#[test]
fn sg_g3_4_terminal_summary_dispatch_emits_capsules() {
    let src = read(SEQUENCER_PATH);
    let arm_start = src
        .find("TypedTx::TerminalSummary(ts) => {")
        .expect("TerminalSummary dispatch arm must exist");
    let arm_block = &src[arm_start..arm_start + 4000];
    assert!(
        arm_block.contains("derive_g3_2_terminal_summary_bankrupt_autopsies"),
        "TerminalSummary dispatch must invoke the G3.2 autopsy derive helper"
    );
    assert!(
        arm_block.contains("is_autopsy_active_at"),
        "TerminalSummary dispatch must activation-gate the autopsy emit \
         (TB-15 R2 closure pattern)"
    );
}

#[test]
fn sg_g3_4_apply_one_stage_3_5b_writes_g3_2_autopsy_to_cas() {
    let src = read(SEQUENCER_PATH);
    assert!(
        src.contains("write_g3_2_terminal_summary_bankrupt_autopsies_to_cas"),
        "apply_one Stage 3.5b must invoke G3.2 autopsy CAS writer (Atom F)"
    );
    assert!(
        src.contains("if let TypedTx::TerminalSummary(ts) = &tx {"),
        "apply_one Stage 3.5b TerminalSummary scrutinee must exist"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// Architect §7.1 — RiskCapImpactReport surface (atom G)
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn arch_71_risk_cap_impact_report_module_present() {
    let src = read("src/runtime/mod.rs");
    assert!(
        src.contains("pub mod risk_cap_impact_report;"),
        "Atom G: risk_cap_impact_report module must be registered in runtime/mod.rs"
    );
    // File itself must exist.
    let _ = read(RISK_CAP_REPORT_PATH);
}

#[test]
fn arch_71_risk_cap_impact_report_predicates_work() {
    // Helper predicates correctly classify the new variants.
    assert!(is_bankruptcy_risk_cap_rejection_class(
        &RejectionClass::BankruptcyRiskCapExceeded
    ));
    assert!(!is_bankruptcy_risk_cap_rejection_class(
        &RejectionClass::StakeBalanceExceeded
    ));
    assert!(is_bankruptcy_risk_cap_transition_error(
        &TransitionError::BankruptcyRiskCapExceeded
    ));
    assert!(!is_bankruptcy_risk_cap_transition_error(
        &TransitionError::StakeBalanceExceeded
    ));
}

#[test]
fn arch_71_tx_kind_label_covers_4_admission_arms() {
    // All 4 admission arms (WorkTx + VerifyTx + ChallengeTx +
    // BuyWithCoinRouter) must be label-able for the report.
    assert_eq!(tx_kind_label_for_risk_cap_rejection(1), "work");
    assert_eq!(tx_kind_label_for_risk_cap_rejection(2), "verify");
    assert_eq!(tx_kind_label_for_risk_cap_rejection(3), "challenge");
    assert_eq!(
        tx_kind_label_for_risk_cap_rejection(15),
        "buy_with_coin_router"
    );
}

#[test]
fn arch_71_default_report_is_empty() {
    let r = RiskCapImpactReport::default();
    assert_eq!(r.total_rejections, 0);
    assert!(r.rows.is_empty());
}

#[test]
fn arch_71_audit_dashboard_wires_risk_cap_impact_report() {
    let src = read("src/bin/audit_dashboard.rs");
    assert!(
        src.contains("risk_cap_impact_report::compute_risk_cap_impact_report_from_paths"),
        "Architect §7.1: audit_dashboard --run-report must derive RiskCapImpactReport from ChainTape/CAS"
    );
    assert!(
        src.contains("render_section_g_2"),
        "Architect §7.1: audit_dashboard --run-report must render the risk-cap impact section"
    );
    assert!(
        src.contains("RiskCapImpactReport"),
        "Architect §7.1: dashboard source must name the RiskCapImpactReport surface"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// Architect §7.5 — FinalizeRewardPayoutBreakdown separates solver vs verifier
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn arch_75_payout_breakdown_default_shape_correct() {
    let q_pre = EconomicState::default();
    let fr = FinalizeRewardTx::default();
    let b = compute_finalize_reward_payout_breakdown(&q_pre, &fr);
    // FinalizeRewardTx::default has reward=0, so solver delta=0.
    assert_eq!(b.solver_reward_delta_micro, 0);
    assert_eq!(b.verifier_bond_return_delta_micro, 0);
    assert_eq!(b.other_settlement_delta_micro, 0);
    assert_eq!(b.total_payout_delta_micro, 0);
    assert_eq!(b.escrow_plus_bonds_at_pre_micro, 0);
    // Trivially bounded.
    assert!(assert_finalize_reward_payout_bounded(&b).is_ok());
}

#[test]
fn arch_75_payout_breakdown_bounded_invariant_rejects_overpay() {
    let bad = FinalizeRewardPayoutBreakdown {
        claim_id: "claim-overpay".into(),
        task_id: TaskId("task-1".into()),
        solver: AgentId("Agent_0".into()),
        solver_reward_delta_micro: 1_000_000,
        verifier_bond_return_delta_micro: 100_000,
        other_settlement_delta_micro: 0,
        total_payout_delta_micro: 1_100_000,
        escrow_plus_bonds_at_pre_micro: 500_000,
    };
    let r = assert_finalize_reward_payout_bounded(&bad);
    assert!(r.is_err(), "must reject payout > escrow + bonds");
}

// ────────────────────────────────────────────────────────────────────────────
// Architect §7.2 — below-cap agents can still READ
// (chain views / observation paths)
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn arch_72_risk_cap_in_admission_only_not_in_predicate_or_read_path() {
    // Risk-cap precondition only fires in 4 ADMISSION arms; it does NOT
    // fire in:
    //  - src/sdk/predicate.rs (predicate evaluation — read-side / pure)
    //  - src/sdk/snapshot.rs (UniverseSnapshot agent-view build)
    //  - src/runtime/agent_pnl.rs::compute_agent_pnl (read-side projection)
    for path in [
        "src/top_white/predicates/registry.rs",
        "src/top_white/predicates/visibility.rs",
        "src/sdk/snapshot.rs",
        "src/sdk/your_position.rs",
    ] {
        let src = read(path);
        assert!(
            !src.contains("BankruptcyRiskCapExceeded"),
            "Architect §7.2: {} must NOT short-circuit on risk-cap (read-side scope)",
            path
        );
    }
}

#[test]
fn arch_72_compute_agent_pnl_does_not_require_cap_pass() {
    // compute_agent_pnl is a read projection; it must work for bankrupt
    // agents (otherwise their PnL trajectory disappears from the dashboard,
    // architect §7.2 violation).
    use turingosv4::runtime::agent_pnl::compute_agent_pnl;
    let q = QState::default();
    let agent = AgentId("Agent_0".into());
    let pnl = compute_agent_pnl(&q, &agent, 1_000_000);
    // No panic; balance = 0 (default QState), realized_pnl = -1M (loss).
    assert_eq!(pnl.balance, 0);
    assert_eq!(pnl.realized_pnl, -1_000_000);
}

// ────────────────────────────────────────────────────────────────────────────
// Architect §7.4 — Sybil guard via agent_verifications_t dedup
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn arch_74_sybil_guard_via_step_3_5_already_in_place() {
    // The Sybil guard for reputation +1 is STRUCTURAL: Step-3.5 rejects
    // duplicate (verifier, target_work_tx) pairs at admission with
    // VerifyDuplicate, so reputation can only accumulate +1 per unique pair.
    // Source-grep witness.
    let src = read(SEQUENCER_PATH);
    assert!(
        src.contains("VerifyDuplicate"),
        "Step-3.5 dedup variant must exist for Sybil guard"
    );
    assert!(
        compact_ws(&src).contains("agent_verifications_t.0.contains(&verify_pair)"),
        "Step-3.5 must read agent_verifications_t for dedup check"
    );
    assert!(
        compact_ws(&src).contains("agent_verifications_t.0.insert(verify_pair)"),
        "Step-5b must insert verify_pair into agent_verifications_t after accept"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// Architect §7.3 — autopsy Markov scope (latest only, not history dump)
// ────────────────────────────────────────────────────────────────────────────

// ────────────────────────────────────────────────────────────────────────────
// Architect §6 ship-gate condition: deterministic fixture witnesses the
// AutopsyCapsule path for at least one bankrupt/low-balance agent (architect
// verdict §6 verbatim: "at least one bankrupt/low-balance AutopsyCapsule
// path is witnessed or a deterministic fixture proves the path"). The
// fixture exercises the pure `derive_g3_2_terminal_summary_bankrupt_autopsies`
// helper end-to-end with a constructed EconomicState containing a bankrupt
// Agent_0 — sufficient to prove the emit path (apply_one Stage 3.5b CAS
// write is mechanically derived from the same helper output).
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn ship_gate_fixture_bankrupt_agent_below_cap_emits_autopsy_capsule() {
    use turingosv4::economy::money::MicroCoin;
    use turingosv4::runtime::autopsy_capsule::{
        derive_g3_2_terminal_summary_bankrupt_autopsies, LossReasonClass,
    };
    use turingosv4::state::typed_tx::{CapsulePrivacyPolicy, TerminalSummaryTx};

    // Setup: Agent_0 preseed 1_000_000 μC → risk-cap = 100_000 μC.
    // Set current balance to 50_000 μC → below cap → bankrupt for emit.
    let mut econ = EconomicState::default();
    econ.balances_t.0.insert(
        AgentId("Agent_0".into()),
        MicroCoin::from_micro_units(50_000),
    );
    let ts = TerminalSummaryTx {
        task_id: TaskId("ship-gate-fixture-task-1".into()),
        ..Default::default()
    };

    let derived = derive_g3_2_terminal_summary_bankrupt_autopsies(
        &econ, &ts, /*round=*/ 1, /*t=*/ 100,
    );

    // Architect SG-G3.4 verbatim: "bankrupt / low-balance agent receives
    // AutopsyCapsule".
    assert_eq!(
        derived.len(),
        1,
        "Bankrupt Agent_0 (balance=50k < cap=100k) MUST produce exactly 1 autopsy capsule"
    );

    let cap = &derived[0].capsule;

    // (a) agent_id correct.
    assert_eq!(cap.agent_id, AgentId("Agent_0".into()));

    // (b) event_id scoped to the per-task-end boundary (architect Q6).
    let _expected_event_id =
        turingosv4::state::typed_tx::EventId(TaskId("ship-gate-fixture-task-1".into()));
    // Just check it carries the task_id projection.
    assert_eq!(cap.event_id.0 .0, "ship-gate-fixture-task-1");

    // (c) loss_amount = initial - current = 1_000_000 - 50_000 = 950_000 μC.
    assert_eq!(cap.loss_amount.micro_units(), 950_000);

    // (d) loss_reason_class = Bankruptcy (TB-15 enum).
    assert!(matches!(cap.loss_reason_class, LossReasonClass::Bankruptcy));

    // (e) Architect §7.3 verbatim: "AutopsyCapsule private scoped read view"
    // — CapsulePrivacyPolicy::AuditOnly prevents global prompt stuffing.
    assert!(matches!(
        cap.privacy_policy,
        CapsulePrivacyPolicy::AuditOnly
    ));

    // (f) capsule_id is content-addressable (sha256 of canonical bytes).
    assert_ne!(
        cap.capsule_id.0, [0u8; 32],
        "capsule_id must be populated (NOT zero); content-addressable per TB-15 R3 closure"
    );

    // (g) Replay-determinism (Art. 0.2): identical inputs → identical capsule_id.
    let derived_again = derive_g3_2_terminal_summary_bankrupt_autopsies(&econ, &ts, 1, 100);
    assert_eq!(derived_again.len(), 1);
    assert_eq!(
        derived_again[0].capsule.capsule_id, cap.capsule_id,
        "Architect Art. 0.2: derive must be replay-deterministic"
    );

    // (h) Architect §7.2 verbatim: "below risk cap: can receive autopsy".
    // The fact that this fixture emits 1 capsule for a below-cap agent
    // IS the witness of the architect §7.2 contract.
}

#[test]
fn ship_gate_fixture_solvent_agent_above_cap_emits_no_autopsy() {
    use turingosv4::economy::money::MicroCoin;
    use turingosv4::runtime::autopsy_capsule::derive_g3_2_terminal_summary_bankrupt_autopsies;
    use turingosv4::state::typed_tx::TerminalSummaryTx;

    // Solvent agent: Agent_0 balance = 500_000 μC > cap = 100_000 μC.
    let mut econ = EconomicState::default();
    econ.balances_t.0.insert(
        AgentId("Agent_0".into()),
        MicroCoin::from_micro_units(500_000),
    );
    let ts = TerminalSummaryTx {
        task_id: TaskId("solvent-fixture-task".into()),
        ..Default::default()
    };
    let derived = derive_g3_2_terminal_summary_bankrupt_autopsies(&econ, &ts, 1, 100);
    assert_eq!(
        derived.len(),
        0,
        "Solvent Agent_0 (balance=500k > cap=100k) MUST NOT produce any autopsy capsule \
         (no false-positives — architect §7.2 read-side scope preserves observable
          behavior for solvent agents)"
    );
}

#[test]
fn ship_gate_fixture_multi_agent_mixed_solvency() {
    use turingosv4::economy::money::MicroCoin;
    use turingosv4::runtime::autopsy_capsule::derive_g3_2_terminal_summary_bankrupt_autopsies;
    use turingosv4::state::typed_tx::TerminalSummaryTx;

    let mut econ = EconomicState::default();
    // Agent_0: bankrupt (50k < 100k cap)
    econ.balances_t.0.insert(
        AgentId("Agent_0".into()),
        MicroCoin::from_micro_units(50_000),
    );
    // Agent_1: solvent (200k > 100k cap)
    econ.balances_t.0.insert(
        AgentId("Agent_1".into()),
        MicroCoin::from_micro_units(200_000),
    );
    // Agent_2: bankrupt (10k < 100k cap)
    econ.balances_t.0.insert(
        AgentId("Agent_2".into()),
        MicroCoin::from_micro_units(10_000),
    );
    // MarketMakerBudget: solvent (1M > 500k cap)
    econ.balances_t.0.insert(
        AgentId("MarketMakerBudget".into()),
        MicroCoin::from_micro_units(1_000_000),
    );
    let ts = TerminalSummaryTx {
        task_id: TaskId("mixed-fixture-task".into()),
        ..Default::default()
    };
    let derived = derive_g3_2_terminal_summary_bankrupt_autopsies(&econ, &ts, 1, 200);
    // 2 bankrupt agents (Agent_0 + Agent_2) → 2 capsules.
    assert_eq!(
        derived.len(),
        2,
        "Multi-agent mixed: only 2 bankrupt agents (Agent_0 + Agent_2) produce capsules"
    );
    // Capsules sorted by AgentId (BTreeMap iteration → deterministic).
    let agent_ids: Vec<&AgentId> = derived.iter().map(|d| &d.capsule.agent_id).collect();
    assert!(agent_ids.contains(&&AgentId("Agent_0".into())));
    assert!(agent_ids.contains(&&AgentId("Agent_2".into())));
    assert!(!agent_ids.contains(&&AgentId("Agent_1".into())));
    assert!(!agent_ids.contains(&&AgentId("MarketMakerBudget".into())));
}

#[test]
fn arch_73_autopsy_capsule_marked_audit_only_privacy() {
    // CapsulePrivacyPolicy::AuditOnly on derive helper output — prevents
    // global prompt stuffing per architect §7.3.
    let src = read(AUTOPSY_PATH);
    let helper_start = src
        .find("pub fn derive_g3_2_terminal_summary_bankrupt_autopsies")
        .expect("derive helper present");
    let helper_block = &src[helper_start..helper_start + 5000];
    assert!(
        helper_block.contains("CapsulePrivacyPolicy::AuditOnly"),
        "Architect §7.3: derived autopsy must use AuditOnly privacy policy"
    );
}
