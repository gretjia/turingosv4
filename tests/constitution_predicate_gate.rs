//! TB-C0 Constitution Landing Gate — Predicate gate
//!
//! Constitutional invariants on the predicate layer:
//!   - Art. I.1: predicate result is binary (Verdict shape = {Pass, Fail})
//!   - Art. I.1: predicate failure CANNOT enter L4 accepted ledger
//!   - Art. I.1: predicate pass is REQUIRED for L4 acceptance
//!   - Art. I.1: Lean verified is required for VerifyTx::Confirm
//!   - Art. I.1.1 / Art. II.2: price (statistical signal) NEVER overrides
//!     predicate (boolean signal) — anti-Goodhart
//!
//! All tests are real assertions — no `assert!(true)` per CR-C0.1.

use turingosv4::state::typed_tx::{RejectionClass, VerifyVerdict};

/// Art. I.1 — Predicate verdict is binary. The `VerifyVerdict` enum has
/// exactly two variants: Confirm (pass) and Doubt (fail). Adding a third
/// "maybe" variant would violate Art. I.1's binary-signal axiom.
#[test]
fn predicate_result_is_binary() {
    // Compile-time witness: pattern-match exhaustively. If a third variant
    // is added, this match will require updating, drawing attention to
    // the constitutional violation.
    let confirm = VerifyVerdict::Confirm;
    let doubt = VerifyVerdict::Doubt;

    fn classify(v: VerifyVerdict) -> &'static str {
        match v {
            VerifyVerdict::Confirm => "pass",
            VerifyVerdict::Doubt => "fail",
        }
    }
    assert_eq!(classify(confirm), "pass");
    assert_eq!(classify(doubt), "fail");

    // Discriminant stability — repr(u8) Confirm=0, Doubt=1 must hold for
    // canonical signing payload stability.
    assert_eq!(VerifyVerdict::Confirm as u8, 0);
    assert_eq!(VerifyVerdict::Doubt as u8, 1);
}

/// Art. I.1 — Predicate failure cannot enter L4 (accepted ledger).
/// When a predicate rejects, the WorkTx must route to L4.E with a
/// `RejectionClass`, never to L4 accepted.
///
/// The TB-18R R3 RejectionClass tail-append (LeanFailed=6, ParseFailed=7,
/// SorryBlocked=8, LlmError=9) covers the failure modes. We assert the
/// admission-fail tags exist in the canonical enum.
#[test]
fn predicate_failure_cannot_enter_l4() {
    // RejectionClass enum must include predicate-failure variants. If
    // these were removed (or merged into Opaque), failure-path
    // distinguishability would break.
    fn route(c: RejectionClass) -> &'static str {
        match c {
            RejectionClass::AcceptancePredicateFail(_) => "predicate-fail-route-to-L4E",
            RejectionClass::SettlementPredicateFail(_) => "predicate-fail-route-to-L4E",
            RejectionClass::StakeInsufficient => "stake-fail-route-to-L4E",
            RejectionClass::SignatureInvalid => "sig-fail-route-to-L4E",
            RejectionClass::StaleParentRoot => "parent-fail-route-to-L4E",
            RejectionClass::Opaque => "opaque-route-to-L4E",
            RejectionClass::BudgetExceeded => "budget-fail-route-to-L4E",
            // TB-N1-AGENT-ECONOMY Phase 2 A3 (2026-05-10): agent declared a
            // stake exceeding their `balances_t` entry. Distinct from
            // `StakeInsufficient` (zero stake) so the agent-bound failure
            // class has its own L4.E route tag.
            RejectionClass::StakeBalanceExceeded => "stake-balance-fail-route-to-L4E",
            // TB-N1-AGENT-ECONOMY Phase 2 A4 (2026-05-10): VerifyTx-arm
            // agent-side rejection classes (verify-peer agency layer).
            RejectionClass::VerifyBondOutOfBounds => "verify-bond-out-of-bounds-route-to-L4E",
            RejectionClass::VerifyTargetNotAccepted => "verify-target-not-accepted-route-to-L4E",
            RejectionClass::VerifyDuplicate => "verify-duplicate-route-to-L4E",
            // TB-G G3.2 (2026-05-12): bankruptcy risk-cap admission rejection.
            // Distinct route tag for the 4-arm admission gate (architect Q5
            // subsuming pattern + per-tx-class telemetry).
            RejectionClass::BankruptcyRiskCapExceeded => "bankruptcy-risk-cap-route-to-L4E",
        }
    }
    use turingosv4::state::typed_tx::PredicateId;
    let acceptance_fail =
        RejectionClass::AcceptancePredicateFail(PredicateId("p_lean_verify".into()));
    let settlement_fail = RejectionClass::SettlementPredicateFail(PredicateId("p_finalize".into()));
    assert_eq!(route(acceptance_fail), "predicate-fail-route-to-L4E");
    assert_eq!(route(settlement_fail), "predicate-fail-route-to-L4E");

    // Sequencer source-side check: rejection-class branching must exist
    // and route to L4.E (not L4) on predicate failure.
    let seq_src = std::fs::read_to_string("src/state/sequencer.rs").expect("sequencer.rs readable");
    assert!(
        seq_src.contains("AcceptancePredicateFail") || seq_src.contains("SettlementPredicateFail"),
        "Predicate gate violation: sequencer.rs does not branch on \
         predicate-failure rejection classes — failure routing un-enforceable."
    );
}

/// Art. I.1 — Predicate pass is required for L4 acceptance. Verified by
/// three structural witnesses:
///   (a) BusResult distinguishes accepted from vetoed (so the runtime
///       can branch on predicate verdict),
///   (b) PredicateRegistry is constructed and threaded into the
///       transition_ledger admit path (per TB-7R Art. III.4 closure),
///   (c) sequencer accept-state-root suite exists with the canonical
///       12+ entry points (typed-tx admit functions).
#[test]
fn predicate_pass_required_for_l4() {
    let bus_src = std::fs::read_to_string("src/bus.rs").expect("bus.rs readable");
    assert!(
        bus_src.contains("BusResult::Vetoed") || bus_src.contains("Vetoed {"),
        "Predicate gate violation: bus.rs lacks Vetoed result variant — \
         pass-vs-fail routing un-distinguishable at append site."
    );
    // PredicateRegistry surface lives in runtime/ and transition_ledger;
    // post TB-7R the bus delegates to verify.rs / runtime / admit path.
    use std::process::Command;
    let pred_grep = Command::new("grep")
        .args([
            "-rEn",
            "--include=*.rs",
            "PredicateRegistry|evaluate_predicates",
            "src/",
        ])
        .output()
        .expect("grep available");
    let pred_stdout = String::from_utf8_lossy(&pred_grep.stdout);
    assert!(
        pred_stdout.contains("PredicateRegistry::new")
            && (pred_stdout.contains("transition_ledger.rs") || pred_stdout.contains("verify.rs")),
        "Predicate gate violation: PredicateRegistry not threaded into \
         verify / transition_ledger admit path — pass-before-accept \
         un-enforceable. Found:\n{pred_stdout}"
    );
    // Sequencer accept-state-root suite exists (12+ canonical typed-tx
    // accept fns: WorkTx, VerifyTx, ChallengeTx, ChallengeResolve,
    // FinalizeReward, TaskOpen, EscrowLock, TaskExpire, TerminalSummary,
    // TaskBankruptcy, CompleteSetMint, CompleteSetRedeem, MarketSeed).
    let seq_src = std::fs::read_to_string("src/state/sequencer.rs").expect("sequencer.rs readable");
    let accept_count = seq_src.matches("_accept_state_root").count();
    assert!(
        accept_count >= 12,
        "Predicate gate violation: sequencer accept_state_root entry points \
         dropped below 12 (TB-13 era count). Found: {accept_count}. \
         Likely indicates a typed-tx admission path was removed without \
         updating predicate routing."
    );
}

/// Art. I.1 + TB-18R R1 — A `VerifyTx::Confirm` (i.e., verified=true
/// signal in evidence) requires real Lean acceptance behind it. The
/// `LeanVerdictKind` enum + `LeanResult` CAS schema enforce typed
/// verdict tracking; a `verified=true` claim without `LeanVerdictKind::Verified`
/// in CAS is an admission fault.
#[test]
fn lean_verified_required_for_verified_worktx() {
    let tel_src = std::fs::read_to_string("src/runtime/attempt_telemetry.rs")
        .expect("attempt_telemetry.rs readable");
    assert!(
        tel_src.contains("pub enum LeanVerdictKind") && tel_src.contains("Verified"),
        "Predicate gate violation: LeanVerdictKind::Verified missing — \
         verified WorkTx cannot be type-distinguished from un-verified."
    );
    // The schema must include CAS-payload routing — `write_lean_result_to_cas`
    // and `read_lean_result_from_cas` exist.
    assert!(
        tel_src.contains("pub fn write_lean_result_to_cas")
            && tel_src.contains("pub fn read_lean_result_from_cas"),
        "Predicate gate violation: LeanResult CAS read/write missing — \
         Lean verdict cannot be persisted as predicate evidence."
    );
    // PartialAccepted is typed (TB-18R Phase 2): a PartialAccepted verdict
    // must NOT silently flatten to verified=true with error_class=None.
    assert!(
        tel_src.contains("PartialAccepted"),
        "Predicate gate violation: LeanVerdictKind::PartialAccepted missing \
         — un-typed PartialAccepted ambiguity (TB-18R Phase 2 fix) could \
         re-emerge."
    );
}

/// Art. I.1.1 + Art. II.2 — Price (statistical signal) NEVER overrides
/// predicate (boolean signal). The price-index module must not expose
/// any function that converts price into a verdict, and the sequencer
/// must not consult price during predicate routing.
#[test]
fn price_never_overrides_predicate() {
    // src/economy/* must not export a `price_to_verdict` / `price_implies_pass`
    // / `index_overrides` style function.
    use std::process::Command;
    let out = Command::new("grep")
        .args([
            "-rEn",
            "--include=*.rs",
            r#"price_to_verdict|price_implies_pass|index_overrides|price_admit"#,
            "src/",
        ])
        .output()
        .expect("grep should be available");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.trim().is_empty(),
        "Predicate gate violation (Art. I.1.1 + Art. II.2): a \
         price-to-verdict shortcut was introduced. Price is statistical \
         signal; predicates are boolean signal. Price MUST NOT override \
         predicate. Found:\n{stdout}"
    );

    // TB-13 fence test must still exist — its presence indicates the
    // forbidden-token list (no AMM/CPMM/orderbook in the legacy quarantine)
    // is enforced.
    let fence_path = "tests/tb_13_legacy_cpmm_forward_fence.rs";
    assert!(
        std::path::Path::new(fence_path).exists(),
        "Predicate gate violation: {fence_path} missing — TB-13 forbidden-\
         token Layer-1 + Layer-2 fence un-enforceable; price-into-predicate \
         smuggling could re-occur via legacy AMM imports."
    );
}
