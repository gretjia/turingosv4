use turingosv4::bottom_white::cas::Cid;
use turingosv4::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
use turingosv4::economy::money::MicroCoin;
use turingosv4::economy::settlement::{
    settlement_projection_from_receipt, SettlementBlockReason, SettlementStatus,
};
use turingosv4::runtime::predicate_receipt::PredicateReceipt;
use turingosv4::state::price_index::RationalPrice;
use turingosv4::state::q_state::{AgentId, Hash, TaskId, TxId};
use turingosv4::state::typed_tx::{ClaimId, FinalizeRewardTx, PredicateId};

fn finalize_reward() -> FinalizeRewardTx {
    FinalizeRewardTx {
        tx_id: TxId("settle-gamma".into()),
        claim_id: ClaimId::new("work-gamma"),
        task_id: TaskId("task-gamma".into()),
        solver: AgentId("solver".into()),
        reward: MicroCoin::from_micro_units(90_000),
        parent_state_root: Hash::ZERO,
        epoch: SystemEpoch::new(1),
        timestamp_logical: 10,
        system_signature: SystemSignature::from_bytes([0u8; 64]),
    }
}

fn receipt(result: bool) -> PredicateReceipt {
    PredicateReceipt {
        predicate_id: PredicateId("settlement-predicate".into()),
        subject_tx_id: TxId("work-gamma".into()),
        tape_event_id: Some("dddddddddddddddddddddddddddddddddddddddd:9".into()),
        tape_head_oid: "dddddddddddddddddddddddddddddddddddddddd".into(),
        logical_t: 9,
        input_cid: Cid::from_content(b"settlement-input"),
        verdict_cid: Cid::from_content(if result { b"pass" } else { b"fail" }),
        registry_root: Hash::ZERO,
        result,
    }
}

#[test]
fn predicate_fail_blocks_payout_even_when_price_is_high() {
    let tx = finalize_reward();
    let high_price = RationalPrice {
        numerator: 99,
        denominator: 100,
    };

    let settlement = settlement_projection_from_receipt(
        &tx,
        Some(&receipt(false)),
        Some(Cid::from_content(b"fail-receipt")),
        MicroCoin::from_micro_units(90_000),
        Some(high_price),
    )
    .expect("settlement projection");

    assert_eq!(settlement.status, SettlementStatus::Blocked);
    assert_eq!(
        settlement.block_reason,
        Some(SettlementBlockReason::PredicateFailed)
    );
    assert_eq!(settlement.payout_amount, MicroCoin::zero());
    assert!(
        !settlement.eligible,
        "price is signal only; failed predicate cannot become payable"
    );
}

#[test]
fn predicate_pass_allows_escrow_backed_payout_even_when_price_is_low() {
    let tx = finalize_reward();
    let low_price = RationalPrice {
        numerator: 1,
        denominator: 100,
    };

    let settlement = settlement_projection_from_receipt(
        &tx,
        Some(&receipt(true)),
        Some(Cid::from_content(b"pass-receipt")),
        MicroCoin::from_micro_units(90_000),
        Some(low_price),
    )
    .expect("settlement projection");

    assert_eq!(settlement.status, SettlementStatus::Eligible);
    assert_eq!(settlement.block_reason, None);
    assert!(settlement.eligible);
    assert_eq!(
        settlement.payout_amount,
        MicroCoin::from_micro_units(90_000)
    );
}
