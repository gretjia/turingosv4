# Codex TB-1 Path A++ MICRO-AUDIT
**Date**: 2026-04-29
**Target**: P0-2 / P0-3 / P0-4 closures (NARROW; no Gemini; no round-2 full audit)
**HEAD**: cb0d456ec75bca14ca6fad8a1719642323abbcdb
**Prompt size**: 95263 chars (Day-6 round-1 was 154KB; aim < 20KB)

---

Reading prompt from stdin...
OpenAI Codex v0.125.0 (research preview)
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: danger-full-access
reasoning effort: xhigh
reasoning summaries: none
session id: 019ddaa3-eefe-7cc0-8334-08ff1db9ef03
--------
user
# Codex TB-1 Path A++ Micro-Audit (closed questions only)

**Role**: skeptical adversarial reviewer.
**Mandate**: NARROW. Three closed questions on three landed patches.

**Background (read once; do NOT debate)**:
- Day-6 dual audit returned Codex CHALLENGE / Gemini PASS → merged CHALLENGE
  per `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS).
- User adopted Path A++: narrow the TB-1 ship claim AND close Codex P0-2 +
  P0-3 + P0-4 (the three lowest-cost code-side P0s). Codex P0-1 (runtime
  enforcement of the L4/L4.E split through `dispatch_transition`) is
  INTENTIONALLY NOT addressed in TB-1; it is the primary scope of the renamed
  TB-2 ("P1/P3 Runtime Boundary Closure + RSP-1"). DO NOT re-litigate P0-1.

**Decision rule**: each question independently returns one of:
- **PASS** — patch closes the question; no further action.
- **CHALLENGE** — concrete defect remains; specify the smallest patch that closes it.
- (No VETO disposition for this micro-audit; if you want to escalate beyond
  the three questions, file a separate CHALLENGE on the OUT-OF-SCOPE check
  at the bottom.)

---

## Q1 (P0-2). Does Tier-A now cover all six `EconomicState` holding subindexes?

**Codex P0-2 finding (Day-6)**: Tier-A test 7 only redistributes through
balances + escrows; the all-six-subindex coverage existed only at the
unit level (`ctf_counts_all_six_holding_subindexes` in
`src/economy/monetary_invariant.rs::tests`).

**Path A++ patch**: a 10th Tier-A blocking test was added. Verify:

1. Does this test exercise ALL six holding subindexes (`balances_t`,
   `escrows_t`, `stakes_t`, `claims_t`, `task_markets_t.bounty`,
   `challenge_cases_t.bond`)?
2. Does it route through `assert_total_ctf_conserved` (the production-path
   guard) — NOT `total_supply_micro` directly?
3. Would a regression that drops ANY single subindex from
   `total_supply_micro` cause this test to FAIL with a unique, identifiable
   delta (not a generic round-trip equality)?
4. Is the test in the `Tier-A — BLOCKING` section, NOT marked `#[ignore]`,
   and NOT in Tier-B?

If all four: **PASS**. Otherwise: **CHALLENGE** with the smallest patch that
closes the gap.

## Q2 (P0-3). Does `raw_diagnostic_cid` now fail closed under raw-record serialization?

**Codex P0-3 finding (Day-6)**: `RejectedSubmissionRecord` is `pub`, derives
`Serialize`, exposes `pub raw_diagnostic_cid`, and `records()` returns full
raw refs. Shielding was convention only — any agent-facing serialization
that bypassed `PublicRejectionView` would leak the raw cid.

**Path A++ patch**: `#[serde(skip_serializing, default)]` added on
`raw_diagnostic_cid`. Verify:

1. Is `serde_json::to_value(&record)` for a `RejectedSubmissionRecord` with
   `Some(cid)` raw diagnostic now structurally missing the
   `raw_diagnostic_cid` key?
2. Is the in-memory forensic access via `RejectionEvidenceWriter::records()`
   STILL able to read `raw_diagnostic_cid.is_some()` (i.e., the shield is
   serialization-side, not destructive)?
3. Is there an explicit unit test asserting both #1 and #2 in this commit?
4. Is the limitation (capability-gated forensic API is a future TB)
   documented in the field doc-comment so a future maintainer doesn't
   re-expose the cid?

If all four: **PASS**. Otherwise: **CHALLENGE**.

(Note on hash-chain: with `#[serde(skip_serializing, default)]`, a
persist→rehydrate cycle would lose `raw_diagnostic_cid` and the chain hash
would not re-verify. RSP-0 is in-memory only, so this is acceptable for TB-1.
If you believe this creates a TB-2 footgun, raise it as an OUT-OF-SCOPE
finding, not a Q2 CHALLENGE.)

## Q3 (P0-4). Does `AcceptedLedger::load_from_path` now reject `prev_hash` / entry-`hash` / `logical_t` tampers?

**Codex P0-4 finding (Day-6)**: `load_from_path` called `reconstruct_state`
only; tampers to fields NOT checked by `reconstruct_state` (`prev_hash`,
the entry `hash` field, `logical_t` row-deletion) would load successfully.

**Path A++ patch**: `load_from_path` now calls `verify_chain(0, len)`
BEFORE `reconstruct_state`. Verify:

1. Does the new `load_from_path` body call `verify_chain` first AND propagate
   its `Err` via `?` BEFORE `reconstruct_state` runs?
2. Are there explicit unit tests in `src/economy/ledger.rs::tests` that:
   - Persist a clean chain, mutate ONLY `entries[1].prev_hash`, then assert
     `load_from_path` returns `Err(LedgerError::HashMismatch { at_index: 1 })`;
   - Persist a clean chain, mutate ONLY `entries[0].hash`, then assert
     `load_from_path` returns `Err(LedgerError::HashMismatch { at_index: 0 })`;
   - Persist a 3-entry chain, REMOVE the middle row (so `logical_t=3` ends
     up at index 1), then assert `load_from_path` returns either
     `LogicalTGap { at_index: 1 }` or `HashMismatch { at_index: 1 }`?
3. Does the existing Tier-A `test_p1_kill_1_no_wtool_bypass`
   (`resulting_state_root` mutation on the last entry) STILL pass, since
   `verify_chain` now catches it earlier than `reconstruct_state`?

If all three: **PASS**. Otherwise: **CHALLENGE**.

## Out-of-scope check (file as a separate finding only if applicable)

Did the three patches accidentally:
- Touch a STEP_B-protected file (`src/state/sequencer.rs`, `src/bus.rs`,
  `src/sdk/tools/wallet.rs`)?
- Modify `dispatch_transition` or any production sequencer logic?
- Re-pin a Trust Root manifest entry without the corresponding file change?
- Introduce a NEW `pub` API surface beyond `load_from_path_unverified`-style
  controlled escape hatches?

If any: file as a separate **OUT-OF-SCOPE** finding (not a Q1/Q2/Q3 CHALLENGE).
If none: state explicitly "no out-of-scope drift detected".

---

# Output format

```
## Q1 (P0-2): PASS|CHALLENGE
<one-paragraph reasoning + smallest closing patch if CHALLENGE>

## Q2 (P0-3): PASS|CHALLENGE
<one-paragraph reasoning + smallest closing patch if CHALLENGE>

## Q3 (P0-4): PASS|CHALLENGE
<one-paragraph reasoning + smallest closing patch if CHALLENGE>

## Out-of-scope drift: <none | finding>

## Merged disposition: PASS-ALL-THREE | CHALLENGE-ON-{Q1|Q2|Q3}
```

Be terse. No section headers beyond the four above. No re-introduction of
P0-1. No "but you should also...".


---

## XREF: tests/tb_1_acceptance.rs (Path A++ Tier-A 10/10; P0-2 closure is test #10)

```rust
//! TB-1 Day-5 final acceptance battery — Tier-A 10 BLOCKING + Tier-B 4 NON-BLOCKING.
//!
//! Charter: `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` § Day-5.
//! Path A++ amendments (2026-04-29, post Day-6 dual audit): Tier-A grew from 9
//! to 10 (P0-2 promoted), and the limitations section was added to make TB-1's
//! narrowed ship claim explicit. See
//! `handover/audits/DUAL_AUDIT_TB_1_VERDICT_2026-04-29.md` § Recommended path.
//!
//! Tier discipline (audit CF-5 "lighter option"): TB-1 ships when ALL Tier-A
//! tests are green. Tier-B tests are captured as artifacts but DO NOT gate
//! ship; if a Tier-B test goes red, file as a follow-up TB rather than
//! blocking TB-1's P1/P3 RSP-0 deliverable.
//!
//! Tier-A (BLOCKING — P1 + P3 RSP-0 correctness):
//!   1. test_p1_kill_1_no_wtool_bypass                       (P1 kill 1)
//!   2. test_p1_kill_2_rejected_tx_no_state_advance          (P1 kill 2)
//!   3. test_p1_kill_3_ledger_reconstructable                (P1 kill 3)
//!   4. test_p1_kill_4_rejected_log_isolated                 (P1 kill 4)
//!   5. test_p1_exit_7_l4_chain_breaks_on_row_deletion       (P1 Exit 7)
//!   6. test_p1_kill_4b_rejection_chain_breaks_on_row_deletion (P1 kill 4b)
//!   7. test_p3_rsp0_exit_1_on_init_total_invariant          (P3 RSP-0 Exit 1)
//!   8. test_p3_rsp0_exit_2_read_is_free                     (P3 RSP-0 Exit 2)
//!   9. test_p3_kill_1_no_post_init_mint                     (P3 kill 1)
//!  10. test_p3_rsp0_total_supply_counts_all_six_subindexes  (P0-2 path-A++)
//!
//! Tier-B (NON-BLOCKING — P6 anchor evidence + future-RSP placeholders):
//!  11. test_at1_evaluator_solves_mathd_algebra_107_n3       (#[ignore]: live LLM)
//!  12. test_at2_l4_entry_per_dispatched_tx                  (#[ignore]: WorkTx dispatch
//!                                                           body lands TB-2 RSP-1)
//!  13. test_at3_h_vppu_non_null_on_second_run               (UNIT form; live form
//!                                                           verified by Day-4 evidence)
//!  14. test_at4_econ_balance_delta_non_zero                 (#[ignore]: RSP-1)
//!
//! AT-5 (winning-tactic-in-prompt-context) is DESCOPED per recharter — moves
//! to a future P5 MetaTape v1 TB after P3 RSP-3 lands.
//!
//! ─── Limitations (Path A++ narrowed claim) ──────────────────────────────────
//!
//! TB-1 ships **P1/P3 primitives + invariant scaffolding**, NOT runtime
//! dispatch enforcement. The Tier-A battery PROVES:
//!   - L4 / L4.E split as data structures (hash chains, projections, tamper
//!     detection, type-shielded raw diagnostic).
//!   - Monetary invariant pure functions (no_post_init_mint, total CTF
//!     conservation across all six holding subindexes, read-is-free at
//!     tx-level).
//!
//! The Tier-A battery DOES NOT prove (deferred to TB-2 RSP-1):
//!   - That `Sequencer::dispatch_transition` actually CALLS these guards on
//!     the production path — `dispatch_transition` returns
//!     `NotYetImplemented` for all 7 K5 `TypedTx` variants today.
//!   - That a real predicate-failed `WorkTx` dispatched through the sequencer
//!     lands in L4.E (rather than aborting before any append) — `apply_one`
//!     early-returns on transition error.
//!   - That `assert_total_ctf_conserved` / `assert_read_is_free` /
//!     `assert_no_post_init_mint` are wired into any production call site —
//!     they pass module + Tier-A tests but no caller has been audited to
//!     invoke them yet.
//!   - SDK-boundary read-is-free (rtool / search / private think): only the
//!     tx-level no-fee invariant is covered here.
//!   - That CAS namespacing prevents raw diagnostic leakage if a
//!     `raw_diagnostic_cid` is shared — the type shield prevents accidental
//!     serialization, but capability-gated forensic access lands in a later
//!     TB.

use std::collections::{BTreeMap, BTreeSet};

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    PublicRejectionView, RejectionClass, RejectionEvidenceError, RejectionEvidenceWriter,
};
use turingosv4::bottom_white::ledger::transition_ledger::TxKind;
use turingosv4::economy::ledger::{AcceptedLedger, LedgerError};
use turingosv4::economy::money::{MicroCoin, StakeMicroCoin, MICRO_PER_COIN};
use turingosv4::economy::monetary_invariant::{
    assert_read_is_free, assert_total_ctf_conserved, MonetaryError,
};
use turingosv4::state::q_state::{AgentId, EconomicState, Hash, TxId};
use turingosv4::state::typed_tx::{
    AgentSignature, BoolWithProof, PredicateId, PredicateResultsBundle, ReadKey,
    SafetyOrCreation, TaskId, TypedTx, WorkTx, WriteKey,
};

// ────────────────────────────────────────────────────────────────────────────
// Fixtures
// ────────────────────────────────────────────────────────────────────────────

fn fixture_work_tx(suffix: u32) -> TypedTx {
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId(format!("acc-{}", suffix)),
        BoolWithProof {
            value: true,
            proof_cid: Some(Cid([0x11; 32])),
        },
    );
    let mut settlement = BTreeMap::new();
    settlement.insert(
        PredicateId(format!("set-{}", suffix)),
        BoolWithProof {
            value: true,
            proof_cid: None,
        },
    );
    let mut read_set = BTreeSet::new();
    read_set.insert(ReadKey(format!("k.r.{}", suffix)));
    let mut write_set = BTreeSet::new();
    write_set.insert(WriteKey(format!("k.w.{}", suffix)));
    TypedTx::Work(WorkTx {
        tx_id: TxId(format!("worktx-{}", suffix)),
        task_id: TaskId(format!("task-{}", suffix)),
        parent_state_root: Hash::ZERO,
        agent_id: AgentId("alice".into()),
        read_set,
        write_set,
        proposal_cid: Cid([0x13; 32]),
        predicate_results: PredicateResultsBundle {
            acceptance,
            settlement,
            safety_class: SafetyOrCreation::Safety,
        },
        stake: StakeMicroCoin::from_micro_units(1_000_000),
        signature: AgentSignature::from_bytes([0x77u8; 64]),
        timestamp_logical: suffix as u64,
    })
}

fn cid(byte: u8) -> Cid {
    Cid([byte; 32])
}

fn agent(s: &str) -> AgentId {
    AgentId(s.to_string())
}

fn coin(n: i64) -> MicroCoin {
    MicroCoin::from_coin(n).unwrap()
}

// ════════════════════════════════════════════════════════════════════════════
// Tier-A — BLOCKING
// ════════════════════════════════════════════════════════════════════════════

// ────────────────────────────────────────────────────────────────────────────
// (1) P1 kill 1 — no wtool bypass
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_1_no_wtool_bypass() {
    let mut l = AcceptedLedger::new();
    for i in 1..=3 {
        l.append_accepted(&fixture_work_tx(i)).unwrap();
    }
    let canonical_root = l.current_state_root();

    let tmp = tempfile::NamedTempFile::new().unwrap();
    l.persist(tmp.path()).unwrap();

    // Bypass: directly overwrite state.db without going through L4.
    let raw = std::fs::read(tmp.path()).unwrap();
    let mut tampered: Vec<turingosv4::economy::ledger::AcceptedEntry> =
        serde_json::from_slice(&raw).unwrap();
    tampered.last_mut().unwrap().resulting_state_root = Hash([0xFF; 32]);
    let bytes = serde_json::to_vec(&tampered).unwrap();
    std::fs::write(tmp.path(), bytes).unwrap();

    // Reconstruction MUST fail: explicit error OR diverged root.
    match AcceptedLedger::load_from_path(tmp.path()) {
        Err(_) => {} // bypass detected — expected
        Ok((_, reconstructed)) => assert_ne!(
            reconstructed, canonical_root,
            "bypass mutation must not survive a round-trip through reconstruct_state"
        ),
    }
}

// ────────────────────────────────────────────────────────────────────────────
// (2) P1 kill 2 — rejected tx does not advance state
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_2_rejected_tx_no_state_advance() {
    let mut l4 = AcceptedLedger::new();
    let mut l4e = RejectionEvidenceWriter::new();

    l4.append_accepted(&fixture_work_tx(1)).unwrap();
    let baseline_root = l4.current_state_root();
    let baseline_logical_t = l4.len();

    l4e.append_rejected(
        42,
        baseline_root,
        agent("alice"),
        TxKind::Work,
        cid(0x20),
        RejectionClass::PredicateFailed,
        Some(cid(0xAA)),
        None,
    );

    assert_eq!(
        l4.current_state_root(),
        baseline_root,
        "rejected tx must NOT advance L4 state_root"
    );
    assert_eq!(
        l4.len(),
        baseline_logical_t,
        "rejected tx must NOT advance L4 logical_t"
    );

    assert_eq!(l4e.len(), 1, "rejection produces exactly one L4.E record");
    let r = &l4e.records()[0];
    assert_eq!(r.submit_id, 42);
    assert!(r.raw_diagnostic_cid.is_some());
    assert!(l4e.verify_chain().is_ok());
}

// ────────────────────────────────────────────────────────────────────────────
// (3) P1 kill 3 — ledger reconstructable
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_3_ledger_reconstructable() {
    let mut l = AcceptedLedger::new();
    for i in 1..=4 {
        l.append_accepted(&fixture_work_tx(i)).unwrap();
    }
    let pre_drop_root = l.current_state_root();

    let tmp = tempfile::NamedTempFile::new().unwrap();
    l.persist(tmp.path()).unwrap();

    drop(l);

    let (l_reborn, reconstructed_root) = AcceptedLedger::load_from_path(tmp.path()).unwrap();
    assert_eq!(
        reconstructed_root, pre_drop_root,
        "reconstructed state_root must be bit-equal to pre-drop state_root"
    );
    assert_eq!(l_reborn.len(), 4);
    assert!(l_reborn.verify_chain(0, 4).is_ok());
}

// ────────────────────────────────────────────────────────────────────────────
// (4) P1 kill 4 — rejected log is isolated from agent-facing read view
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_4_rejected_log_isolated() {
    let mut l4e = RejectionEvidenceWriter::new();
    l4e.append_rejected(
        7,
        Hash::ZERO,
        agent("alice"),
        TxKind::Work,
        cid(0x10),
        RejectionClass::PredicateFailed,
        Some(cid(0xBE)),
        Some("predicate acceptance failed for acc-7".into()),
    );

    let view: Vec<PublicRejectionView> = l4e.public_view();
    assert_eq!(view.len(), 1);

    let json = serde_json::to_value(&view[0]).unwrap();
    let obj = json.as_object().expect("PublicRejectionView serializes as object");
    assert!(
        !obj.contains_key("raw_diagnostic_cid"),
        "raw_diagnostic_cid must NOT appear in agent-facing public view"
    );
    assert_eq!(
        obj.get("public_summary").and_then(|v| v.as_str()),
        Some("predicate acceptance failed for acc-7")
    );

    assert!(
        l4e.records()[0].raw_diagnostic_cid.is_some(),
        "L4.E forensic record must retain raw_diagnostic_cid (shielding is structural, not destructive)"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// (5) P1 Exit 7 — L4 hash chain breaks on row deletion
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_exit_7_l4_chain_breaks_on_row_deletion() {
    let mut l = AcceptedLedger::new();
    for i in 1..=5 {
        l.append_accepted(&fixture_work_tx(i)).unwrap();
    }
    assert!(l.verify_chain(0, 5).is_ok());

    l.tamper_remove_entry(2);

    let r = l.verify_chain(0, 4);
    match r {
        Err(LedgerError::LogicalTGap { at_index: 2, .. })
        | Err(LedgerError::HashMismatch { at_index: 2 }) => {}
        other => panic!(
            "deleting an L4 row must break the chain at index 2; got {:?}",
            other
        ),
    }
}

// ────────────────────────────────────────────────────────────────────────────
// (6) P1 kill 4b — L4.E hash chain breaks on row deletion
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_4b_rejection_chain_breaks_on_row_deletion() {
    let mut l4e = RejectionEvidenceWriter::new();
    for i in 1..=3u64 {
        l4e.append_rejected(
            i,
            Hash::ZERO,
            agent("alice"),
            TxKind::Work,
            cid(0x10),
            RejectionClass::PredicateFailed,
            None,
            None,
        );
    }
    assert!(l4e.verify_chain().is_ok());

    l4e.tamper_remove_record(1);
    let r = l4e.verify_chain();
    assert!(
        matches!(r, Err(RejectionEvidenceError::HashMismatch { at: 1 })),
        "deleting row 1 must surface as HashMismatch at the new index 1; got {:?}",
        r
    );
}

// ────────────────────────────────────────────────────────────────────────────
// (7) P3 RSP-0 Exit 1 — on_init total invariant across N tx sequence
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p3_rsp0_exit_1_on_init_total_invariant() {
    // After on_init mint, total_coin must be invariant across an arbitrary
    // sequence of redistribution txs (no further mints, no burns). We model
    // a 5-step alice/bob/escrow shuffle and assert assert_total_ctf_conserved
    // succeeds at each step with an empty exempt list.
    let mut s = EconomicState::default();
    s.balances_t.0.insert(agent("alice"), coin(100));
    let baseline = s.clone();

    // Step 1: alice → bob 30
    let mut s1 = EconomicState::default();
    s1.balances_t.0.insert(agent("alice"), coin(70));
    s1.balances_t.0.insert(agent("bob"), coin(30));
    assert_eq!(assert_total_ctf_conserved(&baseline, &s1, &[]), Ok(()));

    // Step 2: bob 30 → escrow
    use turingosv4::state::q_state::EscrowEntry;
    let mut s2 = EconomicState::default();
    s2.balances_t.0.insert(agent("alice"), coin(70));
    s2.escrows_t.0.insert(
        TxId("e-1".into()),
        EscrowEntry { amount: coin(30), depositor: agent("bob") },
    );
    assert_eq!(assert_total_ctf_conserved(&s1, &s2, &[]), Ok(()));

    // Step 3: escrow back to bob
    let mut s3 = EconomicState::default();
    s3.balances_t.0.insert(agent("alice"), coin(70));
    s3.balances_t.0.insert(agent("bob"), coin(30));
    assert_eq!(assert_total_ctf_conserved(&s2, &s3, &[]), Ok(()));

    // Step 4: alice 70 → carol
    let mut s4 = EconomicState::default();
    s4.balances_t.0.insert(agent("carol"), coin(70));
    s4.balances_t.0.insert(agent("bob"), coin(30));
    assert_eq!(assert_total_ctf_conserved(&s3, &s4, &[]), Ok(()));

    // Step 5: full round-trip back to baseline
    let mut s5 = EconomicState::default();
    s5.balances_t.0.insert(agent("alice"), coin(100));
    assert_eq!(assert_total_ctf_conserved(&s4, &s5, &[]), Ok(()));

    // Final cross-check: end == start (closed-system loop).
    assert_eq!(
        s5.balances_t.0.get(&agent("alice")),
        baseline.balances_t.0.get(&agent("alice")),
        "round-trip must restore baseline"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// (8) P3 RSP-0 Exit 2 — read-is-free (rtool / search / think MUST have fee=0)
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p3_rsp0_exit_2_read_is_free() {
    // K5 has no dedicated read-tx variants today; the structural guard runs
    // against ALL TxKinds. assert_read_is_free(kind, fee=0) must succeed for
    // every variant; non-zero fee on ANY variant must surface as ReadCharged.
    for kind in [
        TxKind::Work,
        TxKind::Verify,
        TxKind::Challenge,
        TxKind::Reuse,
        TxKind::FinalizeReward,
        TxKind::TaskExpire,
        TxKind::TerminalSummary,
    ] {
        assert_eq!(
            assert_read_is_free(kind, 0),
            Ok(()),
            "fee=0 must pass for TxKind={:?}",
            kind
        );
    }

    // Anti-Goodhart: any non-zero fee at any kind is structurally rejected.
    assert_eq!(
        assert_read_is_free(TxKind::Reuse, 1),
        Err(MonetaryError::ReadCharged {
            tx_kind: TxKind::Reuse,
            fee: 1
        })
    );
    assert_eq!(
        assert_read_is_free(TxKind::Work, 9999),
        Err(MonetaryError::ReadCharged {
            tx_kind: TxKind::Work,
            fee: 9999
        })
    );
}

// ────────────────────────────────────────────────────────────────────────────
// (9) P3 kill 1 — no post-init mint (rejected route MUST go to L4.E, not L4)
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p3_kill_1_no_post_init_mint() {
    // The numeric kill: any post-init mint surfaces as MonetaryError::PostInitMint.
    let before = EconomicState::default();
    let mut after = EconomicState::default();
    after.balances_t.0.insert(agent("alice"), coin(100));
    let r = assert_total_ctf_conserved(&before, &after, &[]);
    assert_eq!(
        r,
        Err(MonetaryError::PostInitMint {
            delta_micro: 100 * MICRO_PER_COIN
        }),
        "any non-exempt supply increase must surface as PostInitMint"
    );

    // The structural kill: L4.E (NOT L4) is the home for the rejection record.
    // Simulate the dispatch_transition rejection path by writing the rejection
    // ONLY to L4.E and asserting L4 is untouched.
    let l4 = AcceptedLedger::new();
    let pre_root = l4.current_state_root();
    let pre_logical_t = l4.len();

    let mut l4e = RejectionEvidenceWriter::new();
    l4e.append_rejected(
        99,
        pre_root,
        agent("alice"),
        TxKind::Work,
        cid(0x30),
        RejectionClass::InvariantViolation,
        Some(cid(0xC0)),
        Some("PostInitMint: delta_micro=100000000".into()),
    );

    // L4 untouched; L4.E has the record.
    assert_eq!(l4.current_state_root(), pre_root);
    assert_eq!(l4.len(), pre_logical_t);
    assert_eq!(l4e.len(), 1);
    assert!(matches!(
        l4e.records()[0].rejection_class,
        RejectionClass::InvariantViolation
    ));
}

// ────────────────────────────────────────────────────────────────────────────
// (10) P3 RSP-0 — total_supply counts ALL SIX holding subindexes
//      (Path A++ / Codex P0-2 audit 2026-04-29)
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p3_rsp0_total_supply_counts_all_six_subindexes() {
    // Tier-A 7 (`test_p3_rsp0_exit_1_on_init_total_invariant`) only redistributes
    // through balances + escrows. A regression that silently undercounts any
    // OTHER holding subindex (stakes_t / claims_t / task_markets_t.bounty /
    // challenge_cases_t.bond) would still pass test 7.
    //
    // This test seeds each of the six holding fields with a power-of-two amount
    // (1, 2, 4, 8, 16, 32 → sum = 63) and asserts the conservation guard counts
    // the FULL supply. Any single dropped subindex shows up as a unique deficit
    // in `delta_micro` (e.g., missing claims_t alone produces 55 instead of 63).
    use turingosv4::state::q_state::{ChallengeCase, ClaimEntry, EscrowEntry, StakeEntry, TaskMarketEntry};

    let mut seeded = EconomicState::default();
    seeded.balances_t.0.insert(agent("a"), coin(1));
    seeded
        .escrows_t
        .0
        .insert(TxId("e".into()), EscrowEntry { amount: coin(2), depositor: agent("a") });
    seeded
        .stakes_t
        .0
        .insert(TxId("s".into()), StakeEntry { amount: coin(4), staker: agent("a") });
    seeded
        .claims_t
        .0
        .insert(TxId("c".into()), ClaimEntry { amount: coin(8), claimant: agent("a") });
    seeded.task_markets_t.0.insert(
        TxId("m".into()),
        TaskMarketEntry { publisher: agent("a"), bounty: coin(16), ..Default::default() },
    );
    let mut cc = ChallengeCase::default();
    cc.bond = coin(32);
    cc.challenger = agent("a");
    seeded.challenge_cases_t.0.insert(TxId("ch".into()), cc);

    // Conservation across before(empty) → after(seeded) MUST be detected as a
    // mint of exactly 63 coin = 63 * MICRO_PER_COIN. If any subindex is missed
    // by `total_supply_micro`, this delta will be wrong.
    let before = EconomicState::default();
    let r = assert_total_ctf_conserved(&before, &seeded, &[]);
    assert_eq!(
        r,
        Err(MonetaryError::PostInitMint {
            delta_micro: 63 * MICRO_PER_COIN
        }),
        "monetary invariant must sum balances + escrows + stakes + claims + bounty + bond"
    );

    // Inverse direction: closing the seeded state back to empty is read as a
    // burn of the SAME magnitude — proves all six subindexes are decremented
    // symmetrically by the sum.
    let r_inv = assert_total_ctf_conserved(&seeded, &before, &[]);
    assert_eq!(
        r_inv,
        Err(MonetaryError::TotalCtfBurn {
            delta_micro: -(63 * MICRO_PER_COIN)
        }),
        "monetary invariant must symmetrically detect a burn across all six subindexes"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// Tier-B — NON-BLOCKING (artifacts; do not gate ship)
// ════════════════════════════════════════════════════════════════════════════

// (11) AT-1 P6 anchor — evaluator solves mathd_algebra_107 in n3 mode.
//
// Verified out-of-band by the Day-4 live runs documented in commit 50a1d67:
// RUN 1 + RUN 2 both produced solved=true with gp_payload=nlinarith. Capturing
// this here as an #[ignore] live integration test so the assertion is REGISTERED
// in the harness even though it requires a running LLM proxy + DEEPSEEK_API_KEY
// to execute. Run manually with:
//   cargo test test_at1_evaluator_solves_mathd_algebra_107_n3 -- --ignored
#[test]
#[ignore = "Tier-B P6 anchor: requires live LLM proxy + DEEPSEEK_API_KEY; verified Day-4 (commit 50a1d67)"]
fn test_at1_evaluator_solves_mathd_algebra_107_n3() {
    // Live form would shell out to target/release/evaluator with CONDITION=n3
    // ACTIVE_MODEL=deepseek-chat MAX_TRANSACTIONS=10 and assert solved=true on
    // the JSONL row. Body intentionally empty — TB-1 ship gate is the manually-
    // observable Day-4 evidence in /tmp/tb1_day4_smoke_v2/run{1,2}.jsonl.
}

// (12) AT-2 — each tx in evaluator run produces an L4 LedgerEntry.
// Non-blocking until WorkTx dispatch_transition body lands at TB-2 RSP-1.
#[test]
#[ignore = "Tier-B: WorkTx dispatch_transition body lands TB-2 RSP-1; evaluator currently uses legacy emit path"]
fn test_at2_l4_entry_per_dispatched_tx() {
    // When TB-2 RSP-1 wires the WorkTx → AcceptedLedger::append_accepted path,
    // un-ignore this test and assert: for every successful evaluator tx, exactly
    // one L4 entry is appended; the entry's tx_payload_hash equals the tx's
    // canonical hash; verify_chain(0, n) succeeds at the end of the run.
}

// (13) AT-3 — h_vppu non-null on a 2nd-run row.
//
// The live form (2 evaluator invocations producing JSONL rows) is verified by
// the Day-4 evidence at /tmp/tb1_day4_smoke_v2/run2.jsonl (commit 50a1d67):
// run 2 carried `h_vppu=6.215891726697228`. The unit-level CONTRACT (capacity-3
// rolling history; record-then-query semantics; persistence round-trip) is
// covered by the 9 unit tests inside `minif2f_v4::h_vppu_history` itself
// (cargo test -p minif2f_v4 --lib h_vppu_history → 9/9 PASS).
//
// This file is a top-level integration test for the `turingosv4` crate and
// cannot import the `minif2f_v4` experiments crate (asymmetric path dep).
// Registering the AT-3 contract here as a `#[ignore]` documentation stub so
// the harness lists it explicitly; un-ignore + relocate to
// `experiments/minif2f_v4/tests/` if/when minif2f_v4 grows an integration
// test directory.
#[test]
#[ignore = "Tier-B AT-3: covered by minif2f_v4 lib tests + Day-4 live evidence (commit 50a1d67); cannot import minif2f_v4 from turingosv4 integration tests"]
fn test_at3_h_vppu_non_null_on_second_run() {
    // No body — see ignore reason above.
}

// (14) AT-4 — PputResult.econ_balance_delta non-zero.
// Non-blocking until TB-2 RSP-1's escrow_lock_tx + yes_stake_tx fire. RSP-0
// (Day-2) only proves the conservation invariant + scaffolds escrow/balances
// structures; actual non-zero deltas need the RSP-1 wiring.
#[test]
#[ignore = "Tier-B: needs RSP-1 escrow_lock_tx + yes_stake_tx wiring (TB-2)"]
fn test_at4_econ_balance_delta_non_zero() {
    // When TB-2 RSP-1 lands, un-ignore and assert: for an evaluator run that
    // exercises an escrow_lock_tx, PputResult.econ_balance_delta is Some(non-zero).
    // RSP-0 today only ships the conservation invariant + escrow_vault scaffolding.
}

```


---

## XREF: src/bottom_white/ledger/rejection_evidence.rs (P0-3 closure: serde shield + new test)

```rust
//! L4.E rejection-evidence ledger — TB-1 Day-3 P1.
//!
//! Charter authority:
//! - `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` Day-3.
//! - `handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`
//!   (architectural commitment to L4 / L4.E split, post external audit
//!   2026-04-29 CF-1).
//! - ROADMAP P1 Exit 6 (rejected tx ≠ state_root advance), Exit 9
//!   (rejected log not visible in another agent's read view).
//!
//! Constitutional authority:
//! - Inv 7 — accepted spine and rejection-evidence are disjoint ledgers;
//!   rejections never mutate `state_root_t` / `ledger_root_t`.
//! - Inv 10 (Goodhart shield) — raw rejection diagnostics are isolated
//!   from agent-facing materialized views; only `public_summary` is
//!   permitted to cross the agent boundary.
//! - Art. III.4 (selective shielding) — rejection raw content is shielded
//!   by default; explicit opt-in via `public_summary`.
//!
//! Scope (RSP-0 minimum-viable):
//! - In-memory `Vec<RejectedSubmissionRecord>` chained via `prev_hash`.
//! - `submit_id` (NOT `logical_t`) keys each record per the L4 / L4.E split:
//!   accepted spine takes the canonical counter; rejection-evidence carries
//!   an independent submit-side counter from `Sequencer::next_submit_id`.
//! - `raw_diagnostic_cid` is a CAS handle to the raw error bytes; the
//!   `PublicRejectionView` projection (used to materialize agent-facing
//!   read views) DOES NOT carry that field — structural shielding rather
//!   than runtime access-control.
//!
//! Out of scope (deferred):
//! - Persistence backend (Git2 commit chain on `refs/rejections/main` —
//!   future RSP / TB).
//! - SystemSignature attestation per record (CO1.7.5+ when system_keypair
//!   gets a `CanonicalMessage::RejectionEvidence` variant).
//! - Cross-agent visibility policy machinery (CO P2.7).
//!
//! /// TRACE_MATRIX Inv 7 + Inv 10 + ROADMAP P1:6/P1:9: L4.E rejection-evidence ledger.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::bottom_white::cas::schema::Cid;
use crate::bottom_white::ledger::transition_ledger::TxKind;
use crate::state::q_state::{AgentId, Hash};

// ────────────────────────────────────────────────────────────────────────────
// RejectionClass — taxonomy of why a submission was rejected
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:6 — coarse rejection-class discriminator.
///
/// Stable byte-encoding via `#[repr(u8)]` so the discriminator can ride into
/// the canonical hash deterministically across compiler versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum RejectionClass {
    /// A `top_white::predicates` acceptance gate returned `false`.
    PredicateFailed = 0,
    /// A higher-level policy gate (visibility / quorum / quota) said no.
    PolicyViolation = 1,
    /// `Inv 3` escrow-lock missing for a write-side mutation.
    EscrowMissing = 2,
    /// `monetary_invariant` (Inv 4 / 基本法 1) flagged a conservation break.
    InvariantViolation = 3,
    /// `canonical_decode` of the submitted bytes failed.
    MalformedPayload = 4,
}

// ────────────────────────────────────────────────────────────────────────────
// RejectedSubmissionRecord — one row on the L4.E chain
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:6/P1:9 — one rejection-evidence row.
///
/// Distinguished from `LedgerEntry` (the L4 accepted spine):
/// - keyed by `submit_id` (not `logical_t`);
/// - records `parent_state_root` for the snapshot-at-submit but never a
///   `resulting_state_root` (rejection MUST NOT advance state);
/// - `raw_diagnostic_cid` holds the raw error content shielded behind a CAS
///   handle (not exposed in agent-facing views);
/// - `public_summary` is the ONLY field permitted to cross the agent boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RejectedSubmissionRecord {
    /// Independent submit-side counter from `Sequencer::next_submit_id`.
    pub submit_id: u64,
    /// State-root snapshot at submit time — recorded for forensics; NEVER
    /// advanced by rejection (Inv 7).
    pub parent_state_root: Hash,
    /// Submitter agent (opaque string).
    pub agent_id: AgentId,
    /// Discriminator over the submitted (now-rejected) `TypedTx` variant.
    pub tx_kind: TxKind,
    /// CAS handle to the canonical-encoded source `TypedTx`.
    pub tx_payload_cid: Cid,
    /// Coarse why-class (one of `RejectionClass`).
    pub rejection_class: RejectionClass,
    /// CAS handle to the raw diagnostic bytes (e.g. predicate counter-example).
    /// `None` when no raw payload is captured. NEVER exposed via `PublicRejectionView`.
    ///
    /// **TB-1 P0-3 type shield** (Codex audit 2026-04-29): `#[serde(skip_serializing,
    /// default)]` ensures that EVEN IF a future caller bypasses
    /// `PublicRejectionView` and serializes a raw `RejectedSubmissionRecord`, the
    /// raw cid is structurally absent from the output. Forensic in-memory access
    /// continues via `RejectionEvidenceWriter::records()`. A capability-gated
    /// audit-only API replaces this skip in a later TB; until then, the persisted
    /// form is INTENTIONALLY incomplete (rehydration recovers `None` and the
    /// chain hash will not re-verify — RSP-0 is in-memory only).
    #[serde(skip_serializing, default)]
    pub raw_diagnostic_cid: Option<Cid>,
    /// Agent-facing summary string. `None` when no public summary is permitted
    /// (raw-diagnostic-only mode). The ONLY field that crosses the agent boundary.
    pub public_summary: Option<String>,
    /// Hash of the immediately-preceding rejection record; `Hash::ZERO` for the first.
    pub prev_hash: Hash,
    /// SHA-256 over the nine fields above plus a domain-separation prefix.
    pub hash: Hash,
}

impl RejectedSubmissionRecord {
    fn compute_hash(
        submit_id: u64,
        parent_state_root: &Hash,
        agent_id: &AgentId,
        tx_kind: TxKind,
        tx_payload_cid: &Cid,
        rejection_class: RejectionClass,
        raw_diagnostic_cid: &Option<Cid>,
        public_summary: &Option<String>,
        prev_hash: &Hash,
    ) -> Hash {
        let mut h = Sha256::new();
        h.update(b"turingosv4.l4e_rejection_evidence.v1");
        h.update(submit_id.to_be_bytes());
        h.update(parent_state_root.0);
        h.update((agent_id.0.len() as u64).to_be_bytes());
        h.update(agent_id.0.as_bytes());
        h.update((tx_kind as u8).to_be_bytes());
        h.update(tx_payload_cid.0);
        h.update((rejection_class as u8).to_be_bytes());
        match raw_diagnostic_cid {
            Some(c) => {
                h.update([1u8]);
                h.update(c.0);
            }
            None => h.update([0u8]),
        }
        match public_summary {
            Some(s) => {
                h.update([1u8]);
                h.update((s.len() as u64).to_be_bytes());
                h.update(s.as_bytes());
            }
            None => h.update([0u8]),
        }
        h.update(prev_hash.0);
        Hash(h.finalize().into())
    }
}

// ────────────────────────────────────────────────────────────────────────────
// PublicRejectionView — agent-facing projection (Inv 10 Goodhart shield)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX Inv 10 + ROADMAP P1:9 — agent-facing projection.
///
/// **Structural** isolation: the type itself does not carry
/// `raw_diagnostic_cid`. Materializing this view from a
/// `RejectedSubmissionRecord` cannot accidentally leak the raw diagnostic
/// because there is no field to write it into.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicRejectionView {
    pub submit_id: u64,
    pub parent_state_root: Hash,
    pub agent_id: AgentId,
    pub tx_kind: TxKind,
    pub rejection_class: RejectionClass,
    pub public_summary: Option<String>,
}

impl From<&RejectedSubmissionRecord> for PublicRejectionView {
    fn from(r: &RejectedSubmissionRecord) -> Self {
        Self {
            submit_id: r.submit_id,
            parent_state_root: r.parent_state_root,
            agent_id: r.agent_id.clone(),
            tx_kind: r.tx_kind,
            rejection_class: r.rejection_class,
            public_summary: r.public_summary.clone(),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// RejectionEvidenceError — chain-walk failure taxonomy
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:6 — error returned by `RejectionEvidenceWriter::verify_chain`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RejectionEvidenceError {
    /// `prev_hash` chain or per-record hash diverged at the given index
    /// (covers row deletion, field tampering, and reordering).
    HashMismatch { at: usize },
}

impl std::fmt::Display for RejectionEvidenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HashMismatch { at } => write!(f, "rejection-evidence chain break at index {}", at),
        }
    }
}

impl std::error::Error for RejectionEvidenceError {}

// ────────────────────────────────────────────────────────────────────────────
// RejectionEvidenceWriter — append + verify + project-to-public
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:6/P1:9 — RSP-0 in-memory rejection-evidence writer.
///
/// One `Vec<RejectedSubmissionRecord>`; `prev_hash` chained; `submit_id`
/// monotonicity is the caller's responsibility (the writer trusts the
/// `Sequencer::next_submit_id` issuer). No `logical_t` field — accepted
/// spine and rejection-evidence are intentionally disjoint per the L4 / L4.E
/// split (`DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`).
#[derive(Debug, Clone, Default)]
pub struct RejectionEvidenceWriter {
    records: Vec<RejectedSubmissionRecord>,
}

impl RejectionEvidenceWriter {
    /// TRACE_MATRIX P1:6 — empty writer.
    pub fn new() -> Self {
        Self::default()
    }

    /// TRACE_MATRIX P1:6 — count of recorded rejections.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// TRACE_MATRIX P1:6 — empty predicate.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// TRACE_MATRIX P1:6 — last record's hash, or `Hash::ZERO` for empty chain.
    pub fn last_hash(&self) -> Hash {
        self.records.last().map(|r| r.hash).unwrap_or(Hash::ZERO)
    }

    /// TRACE_MATRIX P1:6/P1:9 — append a rejection record; returns the new chain hash.
    ///
    /// CRITICAL: this method MUST NOT be called from the L4 (accepted) write
    /// path — Inv 7 forbids state-root advance on rejection. The caller's
    /// dispatch logic decides which ledger receives the record.
    #[allow(clippy::too_many_arguments)]
    pub fn append_rejected(
        &mut self,
        submit_id: u64,
        parent_state_root: Hash,
        agent_id: AgentId,
        tx_kind: TxKind,
        tx_payload_cid: Cid,
        rejection_class: RejectionClass,
        raw_diagnostic_cid: Option<Cid>,
        public_summary: Option<String>,
    ) -> Hash {
        let prev_hash = self.last_hash();
        let hash = RejectedSubmissionRecord::compute_hash(
            submit_id,
            &parent_state_root,
            &agent_id,
            tx_kind,
            &tx_payload_cid,
            rejection_class,
            &raw_diagnostic_cid,
            &public_summary,
            &prev_hash,
        );
        let record = RejectedSubmissionRecord {
            submit_id,
            parent_state_root,
            agent_id,
            tx_kind,
            tx_payload_cid,
            rejection_class,
            raw_diagnostic_cid,
            public_summary,
            prev_hash,
            hash,
        };
        self.records.push(record);
        hash
    }

    /// TRACE_MATRIX P1:6 — verify the rejection-evidence chain end-to-end.
    ///
    /// Returns `Err(HashMismatch)` if any single field of any record was
    /// tampered, or if a row was deleted (the surviving row's `prev_hash`
    /// no longer matches its predecessor's `hash`).
    pub fn verify_chain(&self) -> Result<(), RejectionEvidenceError> {
        let mut prev = Hash::ZERO;
        for (i, r) in self.records.iter().enumerate() {
            if r.prev_hash != prev {
                return Err(RejectionEvidenceError::HashMismatch { at: i });
            }
            let recomputed = RejectedSubmissionRecord::compute_hash(
                r.submit_id,
                &r.parent_state_root,
                &r.agent_id,
                r.tx_kind,
                &r.tx_payload_cid,
                r.rejection_class,
                &r.raw_diagnostic_cid,
                &r.public_summary,
                &r.prev_hash,
            );
            if recomputed != r.hash {
                return Err(RejectionEvidenceError::HashMismatch { at: i });
            }
            prev = r.hash;
        }
        Ok(())
    }

    /// TRACE_MATRIX P1:9 — read-only record slice (for L4.E forensics; full
    /// records carry `raw_diagnostic_cid` and MUST NOT be exposed across the
    /// agent boundary; use `public_view` for that).
    pub fn records(&self) -> &[RejectedSubmissionRecord] {
        &self.records
    }

    /// TRACE_MATRIX Inv 10 + P1:9 — agent-facing projection.
    ///
    /// `PublicRejectionView` does not carry `raw_diagnostic_cid` by type
    /// construction; this method's output is safe to materialize into another
    /// agent's read view.
    pub fn public_view(&self) -> Vec<PublicRejectionView> {
        self.records.iter().map(PublicRejectionView::from).collect()
    }

    /// TRACE_MATRIX P1:6 — TAMPER-ONLY hook used by kill-criteria integration
    /// tests (`test_p1_kill_4b_rejection_chain_breaks_on_row_deletion`).
    /// `#[doc(hidden)]` + `tamper_` prefix flags any production use as a
    /// reviewable violation; kept `pub` only so integration tests in `tests/`
    /// can reach it (they link against the lib without `cfg(test)` enabled).
    #[doc(hidden)]
    pub fn tamper_remove_record(&mut self, idx: usize) {
        self.records.remove(idx);
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Inline correctness tests; cross-cutting P1 kill acceptance tests live in
// `tests/tb_1_p1_acceptance.rs`.
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn cid(byte: u8) -> Cid {
        Cid([byte; 32])
    }
    fn agent(s: &str) -> AgentId {
        AgentId(s.to_string())
    }

    #[test]
    fn append_records_and_chains() {
        let mut w = RejectionEvidenceWriter::new();
        let h1 = w.append_rejected(
            1,
            Hash::ZERO,
            agent("alice"),
            TxKind::Work,
            cid(0x10),
            RejectionClass::PredicateFailed,
            Some(cid(0xAA)),
            Some("predicate acc1 returned false".into()),
        );
        let h2 = w.append_rejected(
            2,
            Hash::ZERO,
            agent("bob"),
            TxKind::Verify,
            cid(0x11),
            RejectionClass::PolicyViolation,
            None,
            None,
        );
        assert_eq!(w.len(), 2);
        assert_ne!(h1, Hash::ZERO);
        assert_ne!(h2, Hash::ZERO);
        assert_eq!(w.records()[1].prev_hash, h1);
        assert_eq!(w.last_hash(), h2);
        assert!(w.verify_chain().is_ok());
    }

    #[test]
    fn public_view_omits_raw_diagnostic_cid() {
        let mut w = RejectionEvidenceWriter::new();
        w.append_rejected(
            1,
            Hash::ZERO,
            agent("alice"),
            TxKind::Work,
            cid(0x10),
            RejectionClass::PredicateFailed,
            Some(cid(0xAA)), // raw diagnostic bytes
            Some("acc1 false".into()),
        );
        let view = w.public_view();
        assert_eq!(view.len(), 1);
        // Structural isolation: `PublicRejectionView` doesn't have a
        // `raw_diagnostic_cid` field. Round-trip via JSON to assert the
        // serialized form also omits it.
        let json = serde_json::to_value(&view[0]).unwrap();
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("raw_diagnostic_cid"));
        assert_eq!(obj.get("public_summary").unwrap(), "acc1 false");
    }

    #[test]
    fn raw_diagnostic_cid_skipped_in_record_serialization() {
        // TB-1 P0-3 type shield (Codex audit 2026-04-29): even if a caller
        // bypasses PublicRejectionView and serializes a raw
        // RejectedSubmissionRecord, raw_diagnostic_cid must NOT appear in the
        // serialized form. Forensic in-memory access still works.
        let mut w = RejectionEvidenceWriter::new();
        w.append_rejected(
            1,
            Hash::ZERO,
            agent("alice"),
            TxKind::Work,
            cid(0x10),
            RejectionClass::PredicateFailed,
            Some(cid(0xAA)), // raw diagnostic present in-memory
            Some("acc1 false".into()),
        );
        let record = &w.records()[0];

        // Forensic access: in-memory field is populated.
        assert!(
            record.raw_diagnostic_cid.is_some(),
            "in-memory forensic access must still see the raw cid"
        );

        // Serialization: field MUST be structurally absent.
        let json = serde_json::to_value(record).unwrap();
        let obj = json.as_object().expect("record serializes as object");
        assert!(
            !obj.contains_key("raw_diagnostic_cid"),
            "raw_diagnostic_cid must not serialize on RejectedSubmissionRecord"
        );

        // The other shielded-but-public fields stay present.
        assert!(obj.contains_key("submit_id"));
        assert!(obj.contains_key("public_summary"));
    }

    #[test]
    fn verify_detects_field_tamper() {
        let mut w = RejectionEvidenceWriter::new();
        w.append_rejected(
            1,
            Hash::ZERO,
            agent("alice"),
            TxKind::Work,
            cid(0x10),
            RejectionClass::PredicateFailed,
            None,
            Some("ok".into()),
        );
        w.append_rejected(
            2,
            Hash::ZERO,
            agent("bob"),
            TxKind::Verify,
            cid(0x11),
            RejectionClass::PolicyViolation,
            None,
            None,
        );
        // Tamper public_summary on record 0; per-record hash should now
        // disagree with its computed value.
        w.records[0].public_summary = Some("tampered".into());
        let r = w.verify_chain();
        assert!(matches!(r, Err(RejectionEvidenceError::HashMismatch { at: 0 })));
    }
}

```


---

## XREF: src/economy/ledger.rs (P0-4 closure: verify_chain default + three new tamper tests)

```rust
//! L4 accepted-only ledger wrapper — TB-1 Day-3 P1.
//!
//! Charter authority:
//! - `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` Day-3.
//! - ROADMAP P1 Exit 5 (state_root advances on accept), Exit 6 (state_root
//!   unchanged on reject), Exit 7 (ledger hash chain), Exit 8 (state.db
//!   reconstructable from chaintape).
//! - `handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`:
//!   accepted transitions ONLY land here; rejected submissions go to L4.E
//!   (`bottom_white::ledger::rejection_evidence`).
//!
//! Constitutional authority:
//! - WP § 5.L4 — ChainTape Layer 4 spine; one entry per accepted transition.
//! - Art IV (Boot) — every Q_t field MUST be reconstructible by replaying L4.
//! - Inv 7 (no rejection on the accepted spine) — rejections never advance
//!   `state_root_t` / `ledger_root_t`.
//!
//! Scope (RSP-0 minimum-viable wrapper):
//! - Self-contained accepted-only hash chain over `TypedTx` canonical bytes.
//! - `append_accepted` advances `logical_t` and chains `prev_hash`.
//! - `verify_chain(start, end)` walks the hash chain over `[start, end)`.
//! - `reconstruct_state` replays L4 only and returns the canonical
//!   `state_root_t` (L4.E is intentionally NOT consulted).
//! - Persistence helpers (`persist` / `load_from_path`) provide the
//!   "drop state.db; reconstruct from L4" round-trip used by P1 kill
//!   acceptance tests.
//!
//! Out of scope (deferred to CO1.7.5+):
//! - `SystemSignature` attachment (full signing payload + epoch binding).
//! - `dispatch_transition` re-run (state_root mutation requires CO1.8).
//! - Real `Git2LedgerWriter` commit chain — that's the production backend
//!   over `refs/transitions/main`; this RSP-0 wrapper uses an in-memory Vec.
//!
//! /// TRACE_MATRIX WP § 5.L4 + Art IV + ROADMAP P1:5/P1:6/P1:7/P1:8: L4 accepted-only ledger.

use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::bottom_white::ledger::transition_ledger::{canonical_encode, TxKind};
use crate::state::q_state::Hash;
use crate::state::typed_tx::TypedTx;

// ────────────────────────────────────────────────────────────────────────────
// AcceptedEntry — one row on the L4 accepted-only chain
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:5 — one accepted-only L4 row.
///
/// All seven fields enter the hash; tampering any single field breaks
/// `verify_chain` at the affected index. The `tx_payload_hash` is the
/// SHA-256 over the bincode-canonical encoding of the source `TypedTx`,
/// re-using the lower-level `canonical_encode` from `transition_ledger`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcceptedEntry {
    /// 1-based monotonic counter; advances ONLY on accept (not on reject —
    /// rejections take a `submit_id` on L4.E instead, per the L4/L4.E split).
    pub logical_t: u64,
    /// Hash of the immediately-preceding entry; `Hash::ZERO` for the first row.
    pub prev_hash: Hash,
    /// Discriminator over the source `TypedTx` variant.
    pub tx_kind: TxKind,
    /// SHA-256 of `canonical_encode(tx)` — content-address of the payload.
    pub tx_payload_hash: Hash,
    /// State-root before this entry was applied.
    pub parent_state_root: Hash,
    /// State-root after this entry was applied. Computed by `next_state_root`
    /// (the RSP-0 toy mutator); a real `dispatch_transition` lands in CO1.7.5.
    pub resulting_state_root: Hash,
    /// SHA-256 over the six fields above plus a domain-separation prefix.
    pub hash: Hash,
}

impl AcceptedEntry {
    fn compute_hash(
        logical_t: u64,
        prev_hash: &Hash,
        tx_kind: TxKind,
        tx_payload_hash: &Hash,
        parent_state_root: &Hash,
        resulting_state_root: &Hash,
    ) -> Hash {
        let mut h = Sha256::new();
        h.update(b"turingosv4.l4_accepted.v1");
        h.update(logical_t.to_be_bytes());
        h.update(prev_hash.0);
        h.update((tx_kind as u8).to_be_bytes());
        h.update(tx_payload_hash.0);
        h.update(parent_state_root.0);
        h.update(resulting_state_root.0);
        Hash(h.finalize().into())
    }
}

// ────────────────────────────────────────────────────────────────────────────
// LedgerError — shared error taxonomy for append / verify / reconstruct
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:5/P1:6/P1:7/P1:8 — error taxonomy for the L4 wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    /// `verify_chain` walked off the end of `entries`.
    OutOfBounds { len: usize, requested_end: usize },
    /// Hash mismatch at the given chain index (prev_hash break OR entry hash break).
    HashMismatch { at_index: usize },
    /// `logical_t` is not the expected `index + 1` value.
    LogicalTGap { at_index: usize, expected: u64, got: u64 },
    /// `parent_state_root` doesn't match the running replay state.
    ParentStateMismatch { at_index: usize },
    /// `canonical_encode` of the source `TypedTx` failed.
    Encode(String),
    /// File system or JSON serialization error during persist / load.
    Io(String),
}

impl std::fmt::Display for LedgerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfBounds { len, requested_end } => {
                write!(f, "verify_chain end={} exceeds chain len={}", requested_end, len)
            }
            Self::HashMismatch { at_index } => {
                write!(f, "L4 hash chain break at index {}", at_index)
            }
            Self::LogicalTGap { at_index, expected, got } => write!(
                f,
                "logical_t gap at index {}: expected {}, got {}",
                at_index, expected, got
            ),
            Self::ParentStateMismatch { at_index } => {
                write!(f, "parent_state_root mismatch at index {}", at_index)
            }
            Self::Encode(e) => write!(f, "canonical_encode failed: {}", e),
            Self::Io(e) => write!(f, "persistence I/O failed: {}", e),
        }
    }
}

impl std::error::Error for LedgerError {}

/// TRACE_MATRIX P1:7 — `verify_chain` failure alias; kept distinct from
/// `ReconstructError` so callers can pattern-match on chain-walk vs replay.
pub type ChainError = LedgerError;
/// TRACE_MATRIX P1:8 — `reconstruct_state` / `load_from_path` failure alias;
/// distinct from `ChainError` so replay errors are syntactically separable.
pub type ReconstructError = LedgerError;

// ────────────────────────────────────────────────────────────────────────────
// AcceptedLedger — the wrapper itself
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX P1:5/P1:6/P1:7/P1:8 — accepted-only L4 hash chain (RSP-0).
///
/// Single source of truth for the accepted spine. Rejected transitions
/// MUST NOT touch this struct; they take a `submit_id` on L4.E
/// (`bottom_white::ledger::rejection_evidence`).
#[derive(Debug, Clone, Default)]
pub struct AcceptedLedger {
    entries: Vec<AcceptedEntry>,
    current_state_root: Hash,
}

impl AcceptedLedger {
    /// TRACE_MATRIX Art IV Boot — empty L4 (genesis state_root is `Hash::ZERO`).
    pub fn new() -> Self {
        Self::default()
    }

    /// TRACE_MATRIX P1:5 — count of accepted rows.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// TRACE_MATRIX P1:5 — empty predicate.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// TRACE_MATRIX P1:5 — current canonical `state_root_t`.
    pub fn current_state_root(&self) -> Hash {
        self.current_state_root
    }

    /// TRACE_MATRIX P1:5 — append-accepted entry; advances `logical_t` by 1.
    ///
    /// Advances `current_state_root` via the toy mutator `next_state_root`.
    /// Returns the freshly-built `AcceptedEntry` (clone of what was pushed).
    pub fn append_accepted(&mut self, tx: &TypedTx) -> Result<AcceptedEntry, LedgerError> {
        let bytes = canonical_encode(tx).map_err(|e| LedgerError::Encode(e.to_string()))?;
        let tx_payload_hash = sha256_of(&bytes);
        let prev_hash = self.entries.last().map(|e| e.hash).unwrap_or(Hash::ZERO);
        let logical_t = (self.entries.len() as u64) + 1;
        let parent_state_root = self.current_state_root;
        let tx_kind = tx.tx_kind();
        let resulting_state_root = next_state_root(&parent_state_root, &tx_payload_hash);
        let hash = AcceptedEntry::compute_hash(
            logical_t,
            &prev_hash,
            tx_kind,
            &tx_payload_hash,
            &parent_state_root,
            &resulting_state_root,
        );
        let entry = AcceptedEntry {
            logical_t,
            prev_hash,
            tx_kind,
            tx_payload_hash,
            parent_state_root,
            resulting_state_root,
            hash,
        };
        self.entries.push(entry.clone());
        self.current_state_root = resulting_state_root;
        Ok(entry)
    }

    /// TRACE_MATRIX P1:7 — verify hash-chain integrity over `[start, end)`.
    ///
    /// Returns `Err(HashMismatch)` if any single field (logical_t, prev_hash,
    /// tx_payload_hash, parent_state_root, resulting_state_root, tx_kind, or
    /// the entry hash itself) was tampered.
    pub fn verify_chain(&self, start: usize, end: usize) -> Result<(), ChainError> {
        if end > self.entries.len() {
            return Err(LedgerError::OutOfBounds {
                len: self.entries.len(),
                requested_end: end,
            });
        }
        if start > end {
            return Err(LedgerError::OutOfBounds {
                len: self.entries.len(),
                requested_end: start,
            });
        }
        let mut prev = if start == 0 {
            Hash::ZERO
        } else {
            self.entries[start - 1].hash
        };
        for i in start..end {
            let e = &self.entries[i];
            let expected_logical_t = (i as u64) + 1;
            if e.logical_t != expected_logical_t {
                return Err(LedgerError::LogicalTGap {
                    at_index: i,
                    expected: expected_logical_t,
                    got: e.logical_t,
                });
            }
            if e.prev_hash != prev {
                return Err(LedgerError::HashMismatch { at_index: i });
            }
            let recomputed = AcceptedEntry::compute_hash(
                e.logical_t,
                &e.prev_hash,
                e.tx_kind,
                &e.tx_payload_hash,
                &e.parent_state_root,
                &e.resulting_state_root,
            );
            if recomputed != e.hash {
                return Err(LedgerError::HashMismatch { at_index: i });
            }
            prev = e.hash;
        }
        Ok(())
    }

    /// TRACE_MATRIX P1:8 — replay L4 only; recompute the canonical `state_root_t`.
    ///
    /// L4.E is intentionally NOT consulted: rejected submissions never affect
    /// `state_root_t` (Inv 7).
    pub fn reconstruct_state(&self) -> Result<Hash, ReconstructError> {
        let mut s = Hash::ZERO;
        for (i, e) in self.entries.iter().enumerate() {
            if e.parent_state_root != s {
                return Err(LedgerError::ParentStateMismatch { at_index: i });
            }
            let expected = next_state_root(&s, &e.tx_payload_hash);
            if e.resulting_state_root != expected {
                return Err(LedgerError::HashMismatch { at_index: i });
            }
            s = e.resulting_state_root;
        }
        Ok(s)
    }

    /// TRACE_MATRIX P1:8 — persist entries to `state_path` for cold restart.
    pub fn persist(&self, state_path: &Path) -> Result<(), LedgerError> {
        let bytes = serde_json::to_vec(&self.entries).map_err(|e| LedgerError::Io(e.to_string()))?;
        std::fs::write(state_path, bytes).map_err(|e| LedgerError::Io(e.to_string()))?;
        Ok(())
    }

    /// TRACE_MATRIX P1:8 — load entries from `state_path`, verify the hash
    /// chain end-to-end, and recompute the canonical `state_root_t`.
    ///
    /// **Fail-closed default** (TB-1 P0-4, Codex audit 2026-04-29):
    /// `verify_chain(0, len)` runs BEFORE `reconstruct_state` so any tamper of
    /// `prev_hash`, the entry `hash`, `logical_t` (row reorder/duplication), or
    /// `tx_kind` is caught at load time — `reconstruct_state` alone only checks
    /// `parent_state_root` and re-derives `resulting_state_root`, leaving those
    /// other fields unchecked. Used by the "drop state.db; reconstruct from L4"
    /// kill test: any direct mutation that bypassed the L4 path is washed out.
    pub fn load_from_path(state_path: &Path) -> Result<(Self, Hash), ReconstructError> {
        let bytes = std::fs::read(state_path).map_err(|e| LedgerError::Io(e.to_string()))?;
        let entries: Vec<AcceptedEntry> =
            serde_json::from_slice(&bytes).map_err(|e| LedgerError::Io(e.to_string()))?;
        let mut l = Self {
            entries,
            current_state_root: Hash::ZERO,
        };
        let len = l.entries.len();
        l.verify_chain(0, len)?;
        let s = l.reconstruct_state()?;
        l.current_state_root = s;
        Ok((l, s))
    }

    /// TRACE_MATRIX P1:7 — read-only entry slice (for replay / debug / external
    /// tooling that wants to inspect the chain without mutating it).
    pub fn entries(&self) -> &[AcceptedEntry] {
        &self.entries
    }

    /// TRACE_MATRIX P1:7 — TAMPER-ONLY hook used by kill-criteria integration
    /// tests to simulate adversarial row deletion. The `tamper_` prefix and
    /// `#[doc(hidden)]` mark this as not part of the supported API; production
    /// callers MUST NOT use it. Kept `pub` (rather than `cfg(test)`) only so
    /// integration tests in `tests/` can reach it; integration tests link
    /// against the lib without `cfg(test)` enabled.
    #[doc(hidden)]
    pub fn tamper_remove_entry(&mut self, idx: usize) {
        self.entries.remove(idx);
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Helpers
// ────────────────────────────────────────────────────────────────────────────

fn sha256_of(bytes: &[u8]) -> Hash {
    let mut h = Sha256::new();
    h.update(bytes);
    Hash(h.finalize().into())
}

/// RSP-0 toy state mutator: `next = SHA-256(domain || prev_state_root || tx_payload_hash)`.
///
/// This is a minimum-viable demonstration of the state-root-advances-on-accept
/// invariant. The real `dispatch_transition`-driven state_root mutation lands
/// in CO1.7.5 / CO1.8 (proper economic + agent-swarm state evolution).
fn next_state_root(prev: &Hash, tx_payload_hash: &Hash) -> Hash {
    let mut h = Sha256::new();
    h.update(b"turingosv4.l4_state_root.v1");
    h.update(prev.0);
    h.update(tx_payload_hash.0);
    Hash(h.finalize().into())
}

// ────────────────────────────────────────────────────────────────────────────
// Inline correctness tests (round-trip + tamper detection on every field).
// Cross-cutting P1 kill acceptance tests live in `tests/tb_1_p1_acceptance.rs`.
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::q_state::{AgentId, TxId};
    use crate::state::typed_tx::{
        AgentSignature, BoolWithProof, PredicateId, PredicateResultsBundle, ReadKey,
        SafetyOrCreation, TaskId, TypedTx, WorkTx, WriteKey,
    };
    use crate::bottom_white::cas::schema::Cid;
    use crate::economy::money::StakeMicroCoin;
    use std::collections::{BTreeMap, BTreeSet};

    fn fixture_work_tx(suffix: u32) -> TypedTx {
        let mut acceptance = BTreeMap::new();
        acceptance.insert(
            PredicateId(format!("acc-{}", suffix)),
            BoolWithProof { value: true, proof_cid: Some(Cid([0x11; 32])) },
        );
        let mut settlement = BTreeMap::new();
        settlement.insert(
            PredicateId(format!("set-{}", suffix)),
            BoolWithProof { value: true, proof_cid: None },
        );
        let mut read_set = BTreeSet::new();
        read_set.insert(ReadKey(format!("k.r.{}", suffix)));
        let mut write_set = BTreeSet::new();
        write_set.insert(WriteKey(format!("k.w.{}", suffix)));
        TypedTx::Work(WorkTx {
            tx_id: TxId(format!("worktx-{}", suffix)),
            task_id: TaskId(format!("task-{}", suffix)),
            parent_state_root: Hash::ZERO,
            agent_id: AgentId("alice".into()),
            read_set,
            write_set,
            proposal_cid: Cid([0x13; 32]),
            predicate_results: PredicateResultsBundle {
                acceptance,
                settlement,
                safety_class: SafetyOrCreation::Safety,
            },
            stake: StakeMicroCoin::from_micro_units(1_000_000),
            signature: AgentSignature::from_bytes([0x77u8; 64]),
            timestamp_logical: suffix as u64,
        })
    }

    #[test]
    fn append_advances_logical_t_and_state_root() {
        let mut l = AcceptedLedger::new();
        assert_eq!(l.len(), 0);
        assert_eq!(l.current_state_root(), Hash::ZERO);

        let e1 = l.append_accepted(&fixture_work_tx(1)).unwrap();
        assert_eq!(e1.logical_t, 1);
        assert_eq!(e1.prev_hash, Hash::ZERO);
        assert_eq!(e1.parent_state_root, Hash::ZERO);
        assert_ne!(e1.resulting_state_root, Hash::ZERO);
        assert_eq!(l.current_state_root(), e1.resulting_state_root);

        let e2 = l.append_accepted(&fixture_work_tx(2)).unwrap();
        assert_eq!(e2.logical_t, 2);
        assert_eq!(e2.prev_hash, e1.hash);
        assert_eq!(e2.parent_state_root, e1.resulting_state_root);
    }

    #[test]
    fn verify_chain_passes_on_clean_chain() {
        let mut l = AcceptedLedger::new();
        for i in 1..=5 {
            l.append_accepted(&fixture_work_tx(i)).unwrap();
        }
        assert!(l.verify_chain(0, 5).is_ok());
        assert!(l.verify_chain(0, 0).is_ok());
        assert!(l.verify_chain(2, 4).is_ok());
    }

    #[test]
    fn verify_chain_out_of_bounds_rejected() {
        let mut l = AcceptedLedger::new();
        l.append_accepted(&fixture_work_tx(1)).unwrap();
        let r = l.verify_chain(0, 5);
        assert!(matches!(r, Err(LedgerError::OutOfBounds { .. })));
    }

    #[test]
    fn reconstruct_state_round_trip() {
        let mut l = AcceptedLedger::new();
        for i in 1..=4 {
            l.append_accepted(&fixture_work_tx(i)).unwrap();
        }
        let pre = l.current_state_root();
        let reconstructed = l.reconstruct_state().unwrap();
        assert_eq!(pre, reconstructed);
    }

    #[test]
    fn persist_and_load_round_trip() {
        let mut l = AcceptedLedger::new();
        for i in 1..=3 {
            l.append_accepted(&fixture_work_tx(i)).unwrap();
        }
        let pre = l.current_state_root();

        let tmp = tempfile::NamedTempFile::new().unwrap();
        l.persist(tmp.path()).unwrap();
        let (l2, post) = AcceptedLedger::load_from_path(tmp.path()).unwrap();
        assert_eq!(pre, post);
        assert_eq!(l2.len(), 3);
    }

    #[test]
    fn load_from_path_rejects_prev_hash_tamper() {
        // TB-1 P0-4 (Codex audit 2026-04-29): load_from_path MUST run
        // verify_chain. A prev_hash-only tamper is the canonical case where
        // reconstruct_state alone is insufficient — reconstruct_state checks
        // parent_state_root and recomputes resulting_state_root, but does not
        // touch prev_hash. With the fail-closed default, a load on a tampered
        // chain MUST surface as HashMismatch BEFORE reconstruct_state runs.
        let mut l = AcceptedLedger::new();
        for i in 1..=3 {
            l.append_accepted(&fixture_work_tx(i)).unwrap();
        }
        let tmp = tempfile::NamedTempFile::new().unwrap();
        l.persist(tmp.path()).unwrap();

        let raw = std::fs::read(tmp.path()).unwrap();
        let mut tampered: Vec<AcceptedEntry> = serde_json::from_slice(&raw).unwrap();
        // Mutate prev_hash on row index 1 — leaves parent_state_root and
        // resulting_state_root chains intact, so reconstruct_state would
        // succeed in the absence of verify_chain.
        tampered[1].prev_hash = Hash([0xAB; 32]);
        std::fs::write(tmp.path(), serde_json::to_vec(&tampered).unwrap()).unwrap();

        let r = AcceptedLedger::load_from_path(tmp.path());
        assert!(
            matches!(r, Err(LedgerError::HashMismatch { at_index: 1 })),
            "load_from_path must reject prev_hash tamper at index 1; got {:?}",
            r
        );
    }

    #[test]
    fn load_from_path_rejects_entry_hash_tamper() {
        // TB-1 P0-4: tampering the entry `hash` field directly. Same rationale
        // as prev_hash — invisible to reconstruct_state, caught by verify_chain.
        let mut l = AcceptedLedger::new();
        for i in 1..=3 {
            l.append_accepted(&fixture_work_tx(i)).unwrap();
        }
        let tmp = tempfile::NamedTempFile::new().unwrap();
        l.persist(tmp.path()).unwrap();

        let raw = std::fs::read(tmp.path()).unwrap();
        let mut tampered: Vec<AcceptedEntry> = serde_json::from_slice(&raw).unwrap();
        tampered[0].hash = Hash([0xCD; 32]);
        std::fs::write(tmp.path(), serde_json::to_vec(&tampered).unwrap()).unwrap();

        let r = AcceptedLedger::load_from_path(tmp.path());
        assert!(
            matches!(r, Err(LedgerError::HashMismatch { at_index: 0 })),
            "load_from_path must reject entry-hash tamper at index 0; got {:?}",
            r
        );
    }

    #[test]
    fn load_from_path_rejects_logical_t_gap() {
        // TB-1 P0-4: logical_t row-deletion / reorder. Caught by the LogicalTGap
        // arm of verify_chain — invisible to reconstruct_state because
        // logical_t never enters its checks.
        let mut l = AcceptedLedger::new();
        for i in 1..=3 {
            l.append_accepted(&fixture_work_tx(i)).unwrap();
        }
        let tmp = tempfile::NamedTempFile::new().unwrap();
        l.persist(tmp.path()).unwrap();

        let raw = std::fs::read(tmp.path()).unwrap();
        let mut tampered: Vec<AcceptedEntry> = serde_json::from_slice(&raw).unwrap();
        // Drop the middle row — surviving row at index 1 still claims logical_t=3.
        tampered.remove(1);
        std::fs::write(tmp.path(), serde_json::to_vec(&tampered).unwrap()).unwrap();

        let r = AcceptedLedger::load_from_path(tmp.path());
        assert!(
            matches!(
                r,
                Err(LedgerError::LogicalTGap { at_index: 1, .. })
                    | Err(LedgerError::HashMismatch { at_index: 1 })
            ),
            "load_from_path must reject row-deletion at index 1; got {:?}",
            r
        );
    }
}

```


---

## XREF: TB-1_recharter_2026-04-29.md (claim narrowing — Path A++ amendment + Day-7 ship gate)

```markdown
# TB-1 Re-Charter — Days 2-7 against P0-P9 phase model (2026-04-29)

**Authority**: architect directive 2026-04-29 (`handover/directives/2026-04-29_9_phase_roadmap.md`) + user `gretjia` chat authorization. Canonical roadmap: `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md`.

**Amended 2026-04-29 (post-audit)**: external auditor's CF-1 / CF-3 / CF-5 incorporated per `handover/audits/2026-04-29_external_audit.md` and user authorization on 2026-04-29. Specific amendments: Day-3 wording switched to L4 / L4.E split (rejected submissions go to L4.E rejection-evidence ledger, NOT to L4 with `status=rejected`); Day-2 framing sharpened (WalletTool = read-only projection of `EconomicState.balances_t`, not "legacy adapter"); Day-5 acceptance gate downgraded so P1/P3 are blocking and P6 artifacts are non-blocking until RSP-1.

**Amended 2026-04-29 — Path A++ (post Day-6 dual audit)**: Day-6 returned Codex CHALLENGE / Gemini PASS; per the conservative merge rule (VETO > CHALLENGE > PASS) this is a CHALLENGE verdict. User ruling 2026-04-29: adopt **Path A++ = narrow the ship claim + close the three lowest-cost Codex P0s + track Day-4 evidence**. Specifically:

1. **Central claim narrowed** (this section, plus § 1 GOAL, plus the Day-7 ship gate): TB-1 ships **P1/P3 RSP-0 primitives + invariant scaffolding**. TB-1 does NOT claim that the v4 runtime kernel honors L4/L4.E split — `Sequencer::dispatch_transition` is `NotYetImplemented` for all 7 K5 `TypedTx` variants and `apply_one` early-returns on transition error, so neither L4-on-accept nor L4.E-on-reject is exercised through the production path today. Runtime dispatch enforcement is **explicitly deferred to TB-2**.
2. **P0-2 closed**: a 10th Tier-A blocking test (`test_p3_rsp0_total_supply_counts_all_six_subindexes`) covers all six holding subindexes (`balances_t` + `escrows_t` + `stakes_t` + `claims_t` + `task_markets_t.bounty` + `challenge_cases_t.bond`).
3. **P0-3 closed**: `RejectedSubmissionRecord.raw_diagnostic_cid` carries `#[serde(skip_serializing, default)]`. Even if a future caller bypasses `PublicRejectionView` and serializes a raw record, the raw cid is structurally absent from the output. (Capability-gated forensic API is a TB-2/TB-3 follow-up.)
4. **P0-4 closed**: `AcceptedLedger::load_from_path` now calls `verify_chain(0, len)` BEFORE `reconstruct_state` — `prev_hash` / entry `hash` / `logical_t` row-deletion tampers that `reconstruct_state` alone misses are now caught at load time.
5. **Day-4 evidence migrated** from `/tmp/tb1_day4_smoke_v2/` to `handover/evidence/tb_1_day4_h_vppu/` (Codex P1-2). The post-hoc `h_vppu` stamping in `experiments/minif2f_v4/src/main.rs` (rather than inside `make_pput`) is registered as an **approved spec divergence**: `h_vppu` depends on history (I/O + side effect), keeping `make_pput` pure was the intentional engineering call.

Day-7 ship runs a **narrowed Codex micro-audit** (no Gemini, no large prompt) on the three closed code points. Round-2 dual audit is NOT required because Codex's CHALLENGE was about claim scope, not latent bugs.

**TB-2 candidate is renamed** from "P3 RSP-1" to **"P1/P3 Runtime Boundary Closure + RSP-1"**: primary scope is making `WorkTx` actually traverse `dispatch_transition` (accepted → L4 append; rejected → L4.E append; monetary guards as the admission/rejection oracle). RSP-1's escrow_lock_tx + yes_stake_tx ride that closure, not the other way around.

**Original charter**: commit `4ecb708` body. Original GOAL was *"One MiniF2F adaptation problem solved end-to-end at HEAD with the full v4 5-step compile loop active per-tx + economy hooks firing per-tx + L4 ledger commits per-tx + h_vppu computed in PputResult."* That goal bundled four different layer-jumps (P1 ledger, P3 economy, P5 capability compilation, P6 metric) into one 7-day TB.

**Re-charter (this doc)**: keeps Day 1 (already shipped at `063b003`); re-tags Days 2-7 against the 9-phase model; descopes one acceptance test (AT-5) that properly belongs to a P5 MetaTape TB after P3 is green.

**Charter scope**: Days 2-7 only. Day 1 is shipped and final.
**Active TB**: TB-1.
**phase_id**: P1+P3+P6 (P1 primary; P3 RSP-0 secondary; P6 instrumentation tertiary).
**Budget**: remaining of original 7 days × ≤$30 API.

---

## 1. Re-tagged GOAL

**Path A++ narrowed reading (2026-04-29, ruling)**: TB-1 ships **P1/P3 RSP-0 primitives + invariant scaffolding** as data structures + pure functions. TB-1 does NOT claim runtime dispatch enforcement; that is TB-2's primary scope.

> Discharge the **primitive scaffolding** for P1 + P3 RSP-0 by demonstrating, in unit + Tier-A integration form:
>
> 1. (P1 Exit 5,6 — *as primitives*) `AcceptedLedger::append_accepted` advances `state_root` and `logical_t`; `RejectionEvidenceWriter::append_rejected` does NOT;
> 2. (P1 Exit 7 — *as primitives*) deleting any L4 or L4.E row breaks the corresponding hash chain;
> 3. (P1 Exit 8 — *as primitives*) `AcceptedLedger::load_from_path` reconstructs the canonical `state_root` from L4 only AND verifies the chain end-to-end (P0-4 fail-closed default);
> 4. (P1 Exit 9 — *as type-shielded primitive*) `PublicRejectionView` carries no `raw_diagnostic_cid` field; `#[serde(skip_serializing)]` on `RejectedSubmissionRecord.raw_diagnostic_cid` extends that shield to direct-record serialization (P0-3);
> 5. (P3 RSP-0 Exit 1,2 — *as pure functions*) `assert_total_ctf_conserved` rejects post-init mint and unauthorized burn across ALL six holding subindexes (P0-2); `assert_read_is_free` rejects any K5 `TxKind` carrying a non-zero per-tx fee;
> 6. (P3 RSP-0 Exit 5 — *scaffolded only*) `EscrowVault::lock_escrow` / `release_escrow` exist as a minimum-viable BTreeMap; live admission of `WorkTx` against an `escrow_lock_tx` is **TB-2**, not TB-1.
> 7. (P6 instrumentation — *anchor only, non-blocking*) `h_vppu` field present and non-null on at least one row of the Day-4 live evidence at `handover/evidence/tb_1_day4_h_vppu/`.

**TB-1 explicitly does NOT prove**:

- That `Sequencer::dispatch_transition` actually CALLS the monetary guards or appends to L4/L4.E on the production path. `dispatch_transition` is `NotYetImplemented` for all 7 K5 `TypedTx` variants today and `apply_one` early-returns on transition error — runtime closure is TB-2.
- That `WorkTx` traverses an admission gate. `EscrowVault` exists as scaffolding; admission semantics live in RSP-1 (TB-2).
- That `provisional_accept` vs full payout is enforced anywhere. The `settlement_tx.payout_sum ≤ escrow_pool` exit was downgraded to TB-2 alongside the broader runtime closure.

This replaces the previous *"5-step compile loop active per-tx"* goal — step 4 (Capability Compilation) is **out of TB-1 scope** (it's P5 MetaTape work that requires a green P3).

## 2. Days 2-7 schedule (revised)

### Day 2 — P3 RSP-0: monetary invariant + on_init unique mint

**phase_id**: P3 (RSP-0 micro-version)
**Exit addressed**: P3:1, P3:2, P3:5 (`on_init` total Coin invariant; rtool/think don't deduct; escrow required for market admission)
**Kill tested**: P3:1 (post-init mint MUST fail), P3:2 (stakeless write MUST fail)

**Build**:
- `src/economy/monetary_invariant.rs` — module exposing:
  - `pub fn assert_no_post_init_mint(tx: &TypedTx, q: &QState) -> Result<(), MonetaryError>`
  - `pub fn assert_total_ctf_conserved(before: &EconomicState, after: &EconomicState, exempt_tx_kinds: &[TxKind]) -> Result<(), MonetaryError>`
  - `pub fn assert_read_is_free(tx_kind: TxKind, fee: u64) -> Result<(), MonetaryError>` (rtool/search/think MUST have fee=0)
- `src/economy/escrow_vault.rs` — minimum-viable BTreeMap<TaskId, EscrowEntry>:
  - `pub fn lock_escrow(task_id, sponsor, amount) -> EscrowReceipt`
  - `pub fn release_escrow(task_id, payout_map) -> Result<(), EscrowError>` (asserts sum ≤ amount before release)
- Unit tests: post-init mint rejected; total CTF conserved across N=10 random tx sequences; escrow over-payout rejected; escrow under-payout accepted (residual returns to sponsor).

**FROZEN today**: `src/sdk/tools/wallet.rs` (STEP_B-protected); `kernel.rs`; `bus.rs`; `genesis_payload.toml [trust_root]` constitution_root entry.

**WalletTool framing (sharpened post-audit 2026-04-29 CF-3)**: `src/sdk/tools/wallet.rs` is **NOT a legacy mutable adapter** — it is a *read-only projection* of `QState.economic_state_t.balances_t`. Mutations to economic state happen exclusively through the canonical RSP path (`SettlementEngine` / `EscrowVault` / `StakeManager` / `monetary_invariant`). No new RSP code may depend on `WalletTool.credit()` or on `WalletTool` mutating its `HashMap<String, f64>` to represent canonical balance state. Existing `WalletTool` tests stay temporarily as legacy behavior tests; they get removed or rewritten as RSP-1/RSP-2 lands.

**Acceptance signal**: `cargo test -p turingosv4 economy::` ≥ 6 tests green; running 1 evaluator shot still produces JSONL row (no regression in P6 capability path).

### Day 3 — P1 GitTape Kernel hardening

**phase_id**: P1
**Exit addressed**: P1:5 (state_root advances on accept), P1:6 (state_root unchanged on reject), P1:7 (ledger hash chain), P1:8 (state.db reconstruction), P1:9 (rejected-log isolation)
**Kill tested**: P1:1 (no wtool bypass), P1:2 (rejected tx ≠ state_root advance), P1:3 (state.db reconstructable), P1:4 (no read-view pollution)

[...]

### Day 7 — Ship (Path A++ narrowed)

Day-7 ship runs:

1. **Codex micro-audit** (no Gemini, no large prompt) on three closed questions:
   - Does Tier-A now cover all six `EconomicState` holding subindexes (P0-2)?
   - Does `raw_diagnostic_cid` now fail closed under raw-record serialization (P0-3)?
   - Does `AcceptedLedger::load_from_path` now reject `prev_hash` / entry-`hash` / row-deletion tampers (P0-4)?
2. If micro-audit returns PASS on the three closed points: ship.
3. **Ship commit must use the narrowed claim verbatim**: "TB-1 ships P1/P3 RSP-0 primitives and invariant scaffolding; runtime enforcement deferred to TB-2." NOT: "the v4 GitTape kernel honors the L4/L4.E split."

On ship:

- TB_LOG.tsv: TB-1 row → status=`shipped`; capability_metric updated to reference `handover/evidence/tb_1_day4_h_vppu/run2.jsonl` (`h_vppu=6.215891726697228`, deepseek-chat, mathd_algebra_107, n3) and explicitly tag `runtime_enforcement=deferred_TB2`; ship_commits range filled.
- Post the **renamed TB-2 candidate to user**: **TB-2 = P1/P3 Runtime Boundary Closure + RSP-1**. Primary scope: real `WorkTx` traversing `dispatch_transition` (accepted → L4; rejected → L4.E; monetary guards as the admission/rejection oracle). RSP-1's `escrow_lock_tx` + `yes_stake_tx` ride that closure, not the other way around.

If Day-6 returns VETO:

- Write `handover/alignment/OBS_TB-1_FAILED_2026-04-29.md` with diagnosis layer (P1 / P3 / P6 instrumentation / charter scope).
- Revert OR keep-with-OBS (NOT for kill criteria; only for Exit-criteria coverage gaps).
- Charter MUST change before retry.

## 3. Out-of-scope items moved to future TBs

| Original AT | Reason | Future home |
|---|---|---|
| AT-5 (winning-tactic in prompt context) | Step-4 Capability Compilation = P5 MetaTape, requires P3 RSP-3 green first | TB-N (P5 MetaTape v1; post-P3-RSP-3) |
| SHA-256 upgrade for prompt_context_hash | Touches Cargo.lock + Trust Root re-hash; cleanest in a dedicated cleanup TB | TB-2 cleanup or TB-3 P5 prep |
| Per-tx FC events for every economy mutation | Belongs in P4 Information Loom signal-routing TB | TB-N (P4 v0) |

## 4. Things this re-charter does NOT change

- Day 1 (shipped at `063b003`) — final, no rewind.
- TB-1 budget — same 7 days × ≤$30 API as original charter.
- The 5 frozen files per TB-1 ship surface (`evaluator.rs`, `jsonl_schema.rs`, `src/economy/ledger.rs` [new], `tests/tb_1_acceptance.rs` [new], TB_LOG.tsv) — same surface; Day 2 adds `src/economy/monetary_invariant.rs` + `src/economy/escrow_vault.rs` to the surface, both new files (no STEP_B file edited).
- 24h iteration cap (memory `feedback_iteration_cap_24h`) — every Day must produce evaluator pass/fail signal within 24h.
- Trust Root protocol (R-014 + R-018) — unchanged; any new file going into the manifest follows the established hash-update protocol.

## 5. Acceptance for re-charter itself

The re-charter ships when:

- This doc is committed.
- TB_LOG.tsv reflects the new column schema with TB-1 phase_id correctly tagged.
- AUTO_RESEARCH_NOTEPAD.md TB methodology v2 references this doc.
- Day 2 work begins with the new `src/economy/monetary_invariant.rs` skeleton.

The re-charter is reverted if any of:

- User retracts the P0-P9 ordering authorization (no expected trigger).
- Day 2 monetary_invariant tests reveal that `on_init` mint-only is silently bypassed by an existing code path (would be a P0 ALREADY-FAILED kill criterion → escalate before continuing).

```


---

## XREF: Recent commit log

```
cb0d456 TB-1 status report to external architect (overnight self-audit)
65d7275 TB-1 Day-6: dual external audit r1 → CHALLENGE/PASS → merged CHALLENGE
6c04c26 TB-1 Day-5: Tier-A 9 acceptance battery (consolidated; 9/9 PASS)
50a1d67 TB-1 Day-4: P6 h_vppu_history instrumentation (NEW file)
846279f TB-1 Day-3: P1 GitTape ledger.rs + rejection_evidence.rs (NEW files)
451cc66 TB-1 Day-2: P3 RSP-0 monetary_invariant + escrow_vault (NEW files)
947e67a TB_LOG.tsv: TB-1 row Tier-A/B annotation; primary-secondary-tertiary phase tag (audit-ingestion wave A #6)
c82db19 DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29: L4 vs L4.E architectural commitment (audit-ingestion wave A #5)
edef868 TB-1 re-charter amendment: L4/L4.E split + WalletTool projection + Tier-A/B (audit-ingestion wave A #4)
538b697 ROADMAP amendment: P0.R + L4/L4.E split + P3 forbidden + dependency graph (audit-ingestion wave A #3)
6b100eb docs/economics.md: rewrite as RSP-0/RSP-1 ground rules (audit-ingestion wave A #2)
0709819 External audit 2026-04-29: archive verbatim (audit-ingestion wave A #1)
d29218e TB-1 re-charter (Days 2-7): P1 + P3 RSP-0 + P6 instrumentation
b433dfd TB_LOG.tsv schema v2: add phase_id + Exit/kill columns; backfill TB-0/TB-1
b983c4a TB methodology v2: P0-P9 phase tagging + kill-criteria gate
1435769 Architect directive 2026-04-29: archive 9-phase roadmap + canonical P0-P9 doc
063b003 TB-1 Day-1 spike: prompt_context_hash + h_vppu fields land; step-4 plumbing exists end-to-end on mathd_algebra_107
```

codex
I’ll verify the landed code rather than relying on the excerpts, then I’ll answer only the closed audit questions.
exec
/bin/bash -lc 'git status --short && git log --oneline -n 8' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc "pwd && rg --files -g 'AGENTS.md' -g '"'!target'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
 M genesis_payload.toml
 M handover/tracer_bullets/TB-1_recharter_2026-04-29.md
 M handover/tracer_bullets/TB_LOG.tsv
 M rules/enforcement.log
 M src/bottom_white/ledger/rejection_evidence.rs
 M src/economy/ledger.rs
 M tests/tb_1_acceptance.rs
?? handover/audits/CODEX_TB_1_PATH_A_PP_MICROAUDIT_2026-04-29.md
?? handover/audits/run_codex_tb_1_path_a_pp_microaudit.sh
?? handover/evidence/tb_1_day4_h_vppu/
cb0d456 TB-1 status report to external architect (overnight self-audit)
65d7275 TB-1 Day-6: dual external audit r1 → CHALLENGE/PASS → merged CHALLENGE
6c04c26 TB-1 Day-5: Tier-A 9 acceptance battery (consolidated; 9/9 PASS)
50a1d67 TB-1 Day-4: P6 h_vppu_history instrumentation (NEW file)
846279f TB-1 Day-3: P1 GitTape ledger.rs + rejection_evidence.rs (NEW files)
451cc66 TB-1 Day-2: P3 RSP-0 monetary_invariant + escrow_vault (NEW files)
947e67a TB_LOG.tsv: TB-1 row Tier-A/B annotation; primary-secondary-tertiary phase tag (audit-ingestion wave A #6)
c82db19 DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29: L4 vs L4.E architectural commitment (audit-ingestion wave A #5)

 exited 1 in 0ms:
/home/zephryj/projects/turingosv4

codex
The worktree has uncommitted Path A++ files, so I’m auditing the current workspace state. I didn’t find a repo-local `AGENTS.md` with the first search; I’ll check the parent scopes and then inspect the exact code/tests.
exec
/bin/bash -lc 'for d in / /home /home/zephryj /home/zephryj/projects /home/zephryj/projects/turingosv4; do test -f "$d/AGENTS.md" && printf '"'%s/AGENTS.md\\n' \""'$d"; done' in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
exec
/bin/bash -lc "rg -n \"test_p3_rsp0_total_supply_counts_all_six_subindexes|Tier-A|Tier-B|assert_total_ctf_conserved|balances_t|escrows_t|stakes_t|claims_t|task_markets_t|challenge_cases_t|#\\[ignore\" tests/tb_1_acceptance.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
1://! TB-1 Day-5 final acceptance battery — Tier-A 10 BLOCKING + Tier-B 4 NON-BLOCKING.
4://! Path A++ amendments (2026-04-29, post Day-6 dual audit): Tier-A grew from 9
9://! Tier discipline (audit CF-5 "lighter option"): TB-1 ships when ALL Tier-A
10://! tests are green. Tier-B tests are captured as artifacts but DO NOT gate
11://! ship; if a Tier-B test goes red, file as a follow-up TB rather than
14://! Tier-A (BLOCKING — P1 + P3 RSP-0 correctness):
24://!  10. test_p3_rsp0_total_supply_counts_all_six_subindexes  (P0-2 path-A++)
26://! Tier-B (NON-BLOCKING — P6 anchor evidence + future-RSP placeholders):
27://!  11. test_at1_evaluator_solves_mathd_algebra_107_n3       (#[ignore]: live LLM)
28://!  12. test_at2_l4_entry_per_dispatched_tx                  (#[ignore]: WorkTx dispatch
32://!  14. test_at4_econ_balance_delta_non_zero                 (#[ignore]: RSP-1)
40://! dispatch enforcement. The Tier-A battery PROVES:
47://! The Tier-A battery DOES NOT prove (deferred to TB-2 RSP-1):
54://!   - That `assert_total_ctf_conserved` / `assert_read_is_free` /
56://!     they pass module + Tier-A tests but no caller has been audited to
75:    assert_read_is_free, assert_total_ctf_conserved, MonetaryError,
140:// Tier-A — BLOCKING
345:    // a 5-step alice/bob/escrow shuffle and assert assert_total_ctf_conserved
348:    s.balances_t.0.insert(agent("alice"), coin(100));
353:    s1.balances_t.0.insert(agent("alice"), coin(70));
354:    s1.balances_t.0.insert(agent("bob"), coin(30));
355:    assert_eq!(assert_total_ctf_conserved(&baseline, &s1, &[]), Ok(()));
360:    s2.balances_t.0.insert(agent("alice"), coin(70));
361:    s2.escrows_t.0.insert(
365:    assert_eq!(assert_total_ctf_conserved(&s1, &s2, &[]), Ok(()));
369:    s3.balances_t.0.insert(agent("alice"), coin(70));
370:    s3.balances_t.0.insert(agent("bob"), coin(30));
371:    assert_eq!(assert_total_ctf_conserved(&s2, &s3, &[]), Ok(()));
375:    s4.balances_t.0.insert(agent("carol"), coin(70));
376:    s4.balances_t.0.insert(agent("bob"), coin(30));
377:    assert_eq!(assert_total_ctf_conserved(&s3, &s4, &[]), Ok(()));
381:    s5.balances_t.0.insert(agent("alice"), coin(100));
382:    assert_eq!(assert_total_ctf_conserved(&s4, &s5, &[]), Ok(()));
386:        s5.balances_t.0.get(&agent("alice")),
387:        baseline.balances_t.0.get(&agent("alice")),
444:    after.balances_t.0.insert(agent("alice"), coin(100));
445:    let r = assert_total_ctf_conserved(&before, &after, &[]);
489:fn test_p3_rsp0_total_supply_counts_all_six_subindexes() {
490:    // Tier-A 7 (`test_p3_rsp0_exit_1_on_init_total_invariant`) only redistributes
492:    // OTHER holding subindex (stakes_t / claims_t / task_markets_t.bounty /
493:    // challenge_cases_t.bond) would still pass test 7.
498:    // in `delta_micro` (e.g., missing claims_t alone produces 55 instead of 63).
502:    seeded.balances_t.0.insert(agent("a"), coin(1));
504:        .escrows_t
508:        .stakes_t
512:        .claims_t
515:    seeded.task_markets_t.0.insert(
522:    seeded.challenge_cases_t.0.insert(TxId("ch".into()), cc);
528:    let r = assert_total_ctf_conserved(&before, &seeded, &[]);
540:    let r_inv = assert_total_ctf_conserved(&seeded, &before, &[]);
551:// Tier-B — NON-BLOCKING (artifacts; do not gate ship)
558:// this here as an #[ignore] live integration test so the assertion is REGISTERED
563:#[ignore = "Tier-B P6 anchor: requires live LLM proxy + DEEPSEEK_API_KEY; verified Day-4 (commit 50a1d67)"]
574:#[ignore = "Tier-B: WorkTx dispatch_transition body lands TB-2 RSP-1; evaluator currently uses legacy emit path"]
593:// Registering the AT-3 contract here as a `#[ignore]` documentation stub so
598:#[ignore = "Tier-B AT-3: covered by minif2f_v4 lib tests + Day-4 live evidence (commit 50a1d67); cannot import minif2f_v4 from turingosv4 integration tests"]
608:#[ignore = "Tier-B: needs RSP-1 escrow_lock_tx + yes_stake_tx wiring (TB-2)"]

exec
/bin/bash -lc "rg -n \"raw_diagnostic_cid|raw_diagnostic_cid_skipped|skip_serializing|records\\(|PublicRejectionView|capability-gated\" src/bottom_white/ledger/rejection_evidence.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
25://! - `raw_diagnostic_cid` is a CAS handle to the raw error bytes; the
26://!   `PublicRejectionView` projection (used to materialize agent-facing
79:/// - `raw_diagnostic_cid` holds the raw error content shielded behind a CAS
98:    /// `None` when no raw payload is captured. NEVER exposed via `PublicRejectionView`.
100:    /// **TB-1 P0-3 type shield** (Codex audit 2026-04-29): `#[serde(skip_serializing,
102:    /// `PublicRejectionView` and serializes a raw `RejectedSubmissionRecord`, the
104:    /// continues via `RejectionEvidenceWriter::records()`. A capability-gated
108:    #[serde(skip_serializing, default)]
109:    pub raw_diagnostic_cid: Option<Cid>,
127:        raw_diagnostic_cid: &Option<Cid>,
140:        match raw_diagnostic_cid {
161:// PublicRejectionView — agent-facing projection (Inv 10 Goodhart shield)
167:/// `raw_diagnostic_cid`. Materializing this view from a
171:pub struct PublicRejectionView {
180:impl From<&RejectedSubmissionRecord> for PublicRejectionView {
266:        raw_diagnostic_cid: Option<Cid>,
277:            &raw_diagnostic_cid,
288:            raw_diagnostic_cid,
315:                &r.raw_diagnostic_cid,
328:    /// records carry `raw_diagnostic_cid` and MUST NOT be exposed across the
330:    pub fn records(&self) -> &[RejectedSubmissionRecord] {
336:    /// `PublicRejectionView` does not carry `raw_diagnostic_cid` by type
339:    pub fn public_view(&self) -> Vec<PublicRejectionView> {
340:        self.records.iter().map(PublicRejectionView::from).collect()
396:        assert_eq!(w.records()[1].prev_hash, h1);
402:    fn public_view_omits_raw_diagnostic_cid() {
416:        // Structural isolation: `PublicRejectionView` doesn't have a
417:        // `raw_diagnostic_cid` field. Round-trip via JSON to assert the
421:        assert!(!obj.contains_key("raw_diagnostic_cid"));
426:    fn raw_diagnostic_cid_skipped_in_record_serialization() {
428:        // bypasses PublicRejectionView and serializes a raw
429:        // RejectedSubmissionRecord, raw_diagnostic_cid must NOT appear in the
442:        let record = &w.records()[0];
446:            record.raw_diagnostic_cid.is_some(),
454:            !obj.contains_key("raw_diagnostic_cid"),
455:            "raw_diagnostic_cid must not serialize on RejectedSubmissionRecord"

exec
/bin/bash -lc "rg -n \"load_from_path|verify_chain\\(0, len\\)|reconstruct_state\\(\\)|load_from_path_rejects|test_p1_kill_1_no_wtool_bypass\" src/economy/ledger.rs tests/tb_1_acceptance.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
tests/tb_1_acceptance.rs:15://!   1. test_p1_kill_1_no_wtool_bypass                       (P1 kill 1)
tests/tb_1_acceptance.rs:148:fn test_p1_kill_1_no_wtool_bypass() {
tests/tb_1_acceptance.rs:167:    match AcceptedLedger::load_from_path(tmp.path()) {
tests/tb_1_acceptance.rs:235:    let (l_reborn, reconstructed_root) = AcceptedLedger::load_from_path(tmp.path()).unwrap();
src/economy/ledger.rs:24://! - Persistence helpers (`persist` / `load_from_path`) provide the
src/economy/ledger.rs:145:/// TRACE_MATRIX P1:8 — `reconstruct_state` / `load_from_path` failure alias;
src/economy/ledger.rs:301:    /// `verify_chain(0, len)` runs BEFORE `reconstruct_state` so any tamper of
src/economy/ledger.rs:307:    pub fn load_from_path(state_path: &Path) -> Result<(Self, Hash), ReconstructError> {
src/economy/ledger.rs:316:        l.verify_chain(0, len)?;
src/economy/ledger.rs:317:        let s = l.reconstruct_state()?;
src/economy/ledger.rs:459:        let reconstructed = l.reconstruct_state().unwrap();
src/economy/ledger.rs:473:        let (l2, post) = AcceptedLedger::load_from_path(tmp.path()).unwrap();
src/economy/ledger.rs:479:    fn load_from_path_rejects_prev_hash_tamper() {
src/economy/ledger.rs:480:        // TB-1 P0-4 (Codex audit 2026-04-29): load_from_path MUST run
src/economy/ledger.rs:501:        let r = AcceptedLedger::load_from_path(tmp.path());
src/economy/ledger.rs:504:            "load_from_path must reject prev_hash tamper at index 1; got {:?}",
src/economy/ledger.rs:510:    fn load_from_path_rejects_entry_hash_tamper() {
src/economy/ledger.rs:525:        let r = AcceptedLedger::load_from_path(tmp.path());
src/economy/ledger.rs:528:            "load_from_path must reject entry-hash tamper at index 0; got {:?}",
src/economy/ledger.rs:534:    fn load_from_path_rejects_logical_t_gap() {
src/economy/ledger.rs:551:        let r = AcceptedLedger::load_from_path(tmp.path());
src/economy/ledger.rs:558:            "load_from_path must reject row-deletion at index 1; got {:?}",

codex
The target assertions are present at a glance. I’m checking the exact diffs now for the drift clause, especially the trust-root manifest and any accidental production/runtime touches.
exec
/bin/bash -lc 'git diff --name-only' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
genesis_payload.toml
handover/tracer_bullets/TB-1_recharter_2026-04-29.md
handover/tracer_bullets/TB_LOG.tsv
rules/enforcement.log
src/bottom_white/ledger/rejection_evidence.rs
src/economy/ledger.rs
tests/tb_1_acceptance.rs

exec
/bin/bash -lc 'git diff -- src/state/sequencer.rs src/bus.rs src/sdk/tools/wallet.rs src/economy/ledger.rs src/bottom_white/ledger/rejection_evidence.rs tests/tb_1_acceptance.rs | rg -n "''^diff --git|''^'"\\+\\s*pub |dispatch_transition|load_from_path_unverified|raw_diagnostic_cid|verify_chain|test_p3_rsp0_total_supply_counts_all_six|test_p1_kill_1_no_wtool_bypass|"'^'"\\+\\s*#\\[ignore\"" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
1:diff --git a/src/bottom_white/ledger/rejection_evidence.rs b/src/bottom_white/ledger/rejection_evidence.rs
19:     pub raw_diagnostic_cid: Option<Cid>,
27:+    fn raw_diagnostic_cid_skipped_in_record_serialization() {
30:+        // RejectedSubmissionRecord, raw_diagnostic_cid must NOT appear in the
47:+            record.raw_diagnostic_cid.is_some(),
55:+            !obj.contains_key("raw_diagnostic_cid"),
56:+            "raw_diagnostic_cid must not serialize on RejectedSubmissionRecord"
67:diff --git a/src/economy/ledger.rs b/src/economy/ledger.rs
81:+    /// `verify_chain(0, len)` runs BEFORE `reconstruct_state` so any tamper of
94:+        l.verify_chain(0, len)?;
106:+        // verify_chain. A prev_hash-only tamper is the canonical case where
122:+        // succeed in the absence of verify_chain.
137:+        // as prev_hash — invisible to reconstruct_state, caught by verify_chain.
161:+        // arm of verify_chain — invisible to reconstruct_state because
188:diff --git a/tests/tb_1_acceptance.rs b/tests/tb_1_acceptance.rs
209:+//!  10. test_p3_rsp0_total_supply_counts_all_six_subindexes  (P0-2 path-A++)
237:+//!   - That `Sequencer::dispatch_transition` actually CALLS these guards on
238:+//!     the production path — `dispatch_transition` returns
250:+//!     `raw_diagnostic_cid` is shared — the type shield prevents accidental
266:+fn test_p3_rsp0_total_supply_counts_all_six_subindexes() {
342: // Non-blocking until WorkTx dispatch_transition body lands at TB-2 RSP-1.
344: #[ignore = "Tier-B: WorkTx dispatch_transition body lands TB-2 RSP-1; evaluator currently uses legacy emit path"]
346:     // canonical hash; verify_chain(0, n) succeeds at the end of the run.

exec
/bin/bash -lc 'git diff -- genesis_payload.toml' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
diff --git a/genesis_payload.toml b/genesis_payload.toml
index 18380c4..41d7e89 100644
--- a/genesis_payload.toml
+++ b/genesis_payload.toml
@@ -210,7 +210,8 @@ boot_attestation_hash = "PENDING_v4_X_SELF_REFERENTIAL_COMPUTE"
 "src/economy/monetary_invariant.rs" = "4250b1e68a2952435e5f848c95e5fec028f142551e646a2c020f038ba1c0ab2a"
 "src/economy/escrow_vault.rs" = "95d6b4ec182dd0f181785190c9e8a71a0631fad4a7e255a152938708a52c7319"
 # 2026-04-29 TB-1 Day-3 — P1 GitTape kernel hardening: L4 accepted-only ledger wrapper (NEW file).
-"src/economy/ledger.rs" = "dfebf2dc476d2e6076c5e98dcd1974291490fd4bea86d249d14f90d7340796e8"
+# 2026-04-29 TB-1 Path A++ — load_from_path now calls verify_chain(0,len) before reconstruct_state (P0-4 fail-closed default); +3 tamper-detection unit tests (prev_hash, entry hash, logical_t row deletion).
+"src/economy/ledger.rs" = "eb17bb85c901c59bec9045cbb6d16ae2f2b08031c35b211756f62ab1c99100f2"
 "tests/walkthrough_inv3_conservation.rs" = "e1e7bc1130dfb3edb4eb1a7aa458557f1134879ae297eecb3e8d46a269434198"
 "src/top_white/mod.rs" = "69b18a5d4c33954c81dccd41d48de770ac1d951daaf655f3a76857d7b492a4af"
 "src/top_white/predicates/mod.rs" = "3d04c1eaa09c65faf9afc038b275df5d2d7fa23b983ec38c1ad8f910f2841e28"
@@ -293,7 +294,8 @@ boot_attestation_hash = "PENDING_v4_X_SELF_REFERENTIAL_COMPUTE"
 # 2026-04-28 CO1.7 type skeleton (pre-audit) — caught spec § 1 circular-digest bug.
 "src/bottom_white/ledger/transition_ledger.rs" = "9178de67c922086477c7946c58f4d0d43393d6f3933c8aae86e5072f7abc635e"
 # 2026-04-29 TB-1 Day-3 — P1 GitTape kernel hardening: L4.E rejection-evidence ledger (NEW file).
-"src/bottom_white/ledger/rejection_evidence.rs" = "84f8982ea94ae8bba08adf66f1d7c8a861ac702c06ff74bc11c4da63081977f8"
+# 2026-04-29 TB-1 Path A++ — RejectedSubmissionRecord.raw_diagnostic_cid carries #[serde(skip_serializing, default)] type shield (P0-3); +1 unit test asserting raw cid is structurally absent from raw-record serialization.
+"src/bottom_white/ledger/rejection_evidence.rs" = "2532624348f8acd8220a78868370ed0325310f8237742fc6d31eede0c71d3ee7"
 "tests/system_keypair_generation.rs" = "94d2de928a0f32e71860cbf9125a85a89bb0ded53f6e55c2e16b6370b9dcaa4b"
 "tests/system_keypair_load_and_decrypt.rs" = "460ce3e0b13d523e62b0818285a414a69a91b1afc51c0e0eba7a521674dfaa55"
 "tests/system_keypair_sign_only_from_runner.rs" = "b0874933f684ba47a254728939d30327a0b59392221e51ec4b2207b901bbe65c"

exec
/bin/bash -lc 'git diff --stat' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
 genesis_payload.toml                               |   6 +-
 .../tracer_bullets/TB-1_recharter_2026-04-29.md    |  59 +++++++---
 handover/tracer_bullets/TB_LOG.tsv                 |   3 +-
 rules/enforcement.log                              |   5 +
 src/bottom_white/ledger/rejection_evidence.rs      |  48 +++++++++
 src/economy/ledger.rs                              |  98 ++++++++++++++++-
 tests/tb_1_acceptance.rs                           | 119 +++++++++++++++++++--
 7 files changed, 310 insertions(+), 28 deletions(-)

codex
The drift check shows no protected STEP_B files in the diff and no `dispatch_transition` production change. The manifest changes are limited to files also changed in this patch set; I’m running the narrow tests to confirm the code paths compile and behave.
exec
/bin/bash -lc 'cargo test -p turingosv4 test_p3_rsp0_total_supply_counts_all_six_subindexes raw_diagnostic_cid_skipped_in_record_serialization load_from_path_rejects_prev_hash_tamper load_from_path_rejects_entry_hash_tamper load_from_path_rejects_logical_t_gap test_p1_kill_1_no_wtool_bypass' in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
error: unexpected argument 'raw_diagnostic_cid_skipped_in_record_serialization' found

Usage: cargo test [OPTIONS] [TESTNAME] [-- [ARGS]...]

For more information, try '--help'.

exec
/bin/bash -lc 'cargo test -p turingosv4 --lib raw_diagnostic_cid_skipped_in_record_serialization -- --exact' in /home/zephryj/projects/turingosv4
 succeeded in 288ms:
warning: unused import: `KernelError`
 --> src/bus.rs:8:29
  |
8 | use crate::kernel::{Kernel, KernelError};
  |                             ^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `TapeError`
 --> src/bus.rs:9:54
  |
9 | use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
  |                                                      ^^^^^^^^^

warning: unused imports: `Deserialize` and `Serialize`
  --> src/bus.rs:13:13
   |
13 | use serde::{Deserialize, Serialize};
   |             ^^^^^^^^^^^  ^^^^^^^^^

warning: unused import: `crate::sdk::snapshot::UniverseSnapshot`
 --> src/sdk/actor.rs:7:5
  |
7 | use crate::sdk::snapshot::UniverseSnapshot;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ToolSignal`
 --> src/sdk/tools/search.rs:4:24
  |
4 | use crate::sdk::tool::{ToolSignal, TuringTool};
  |                        ^^^^^^^^^^

warning: unused import: `std::path::Path`
 --> src/sdk/tools/search.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `Deserialize`
 --> src/sdk/tools/librarian.rs:6:13
  |
6 | use serde::{Deserialize, Serialize};
  |             ^^^^^^^^^^^

warning: unused import: `Path`
 --> src/sdk/tools/librarian.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/cas/store.rs:26:12
   |
26 | use git2::{ObjectType as Git2ObjectType, Repository};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/ledger/transition_ledger.rs:36:12
   |
36 | use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: variable does not need to be mutable
   --> src/top_white/predicates/registry.rs:255:17
    |
255 |             let mut h = Sha256::new();
    |                 ----^
    |                 |
    |                 help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: function `task` is never used
   --> src/economy/monetary_invariant.rs:227:8
    |
227 |     fn task(s: &str) -> TaskId {
    |        ^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:560:19
    |
560 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: function `sign_terminal_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:586:19
    |
586 |     pub(crate) fn sign_terminal_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_finalize_reward` is never used
   --> src/bottom_white/ledger/system_keypair.rs:597:19
    |
597 |     pub(crate) fn sign_finalize_reward(
    |                   ^^^^^^^^^^^^^^^^^^^^

warning: function `sign_task_expire` is never used
   --> src/bottom_white/ledger/system_keypair.rs:608:19
    |
608 |     pub(crate) fn sign_task_expire(
    |                   ^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:627:19
    |
627 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib test) generated 17 warnings (run `cargo fix --lib -p turingosv4 --tests` to apply 11 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running unittests src/lib.rs (target/debug/deps/turingosv4-337b37021c2192ab)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 274 filtered out; finished in 0.00s


codex
One filter was too exact for Rust’s module-qualified test names and selected zero tests. I’ll rerun with prefix filters so the assertions actually execute.
exec
/bin/bash -lc 'cargo test -p turingosv4 --lib raw_diagnostic_cid_skipped_in_record_serialization' in /home/zephryj/projects/turingosv4
 succeeded in 258ms:
warning: unused import: `KernelError`
 --> src/bus.rs:8:29
  |
8 | use crate::kernel::{Kernel, KernelError};
  |                             ^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `TapeError`
 --> src/bus.rs:9:54
  |
9 | use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
  |                                                      ^^^^^^^^^

warning: unused imports: `Deserialize` and `Serialize`
  --> src/bus.rs:13:13
   |
13 | use serde::{Deserialize, Serialize};
   |             ^^^^^^^^^^^  ^^^^^^^^^

warning: unused import: `crate::sdk::snapshot::UniverseSnapshot`
 --> src/sdk/actor.rs:7:5
  |
7 | use crate::sdk::snapshot::UniverseSnapshot;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ToolSignal`
 --> src/sdk/tools/search.rs:4:24
  |
4 | use crate::sdk::tool::{ToolSignal, TuringTool};
  |                        ^^^^^^^^^^

warning: unused import: `std::path::Path`
 --> src/sdk/tools/search.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `Deserialize`
 --> src/sdk/tools/librarian.rs:6:13
  |
6 | use serde::{Deserialize, Serialize};
  |             ^^^^^^^^^^^

warning: unused import: `Path`
 --> src/sdk/tools/librarian.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/cas/store.rs:26:12
   |
26 | use git2::{ObjectType as Git2ObjectType, Repository};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/ledger/transition_ledger.rs:36:12
   |
36 | use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: variable does not need to be mutable
   --> src/top_white/predicates/registry.rs:255:17
    |
255 |             let mut h = Sha256::new();
    |                 ----^
    |                 |
    |                 help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: function `task` is never used
   --> src/economy/monetary_invariant.rs:227:8
    |
227 |     fn task(s: &str) -> TaskId {
    |        ^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:560:19
    |
560 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: function `sign_terminal_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:586:19
    |
586 |     pub(crate) fn sign_terminal_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_finalize_reward` is never used
   --> src/bottom_white/ledger/system_keypair.rs:597:19
    |
597 |     pub(crate) fn sign_finalize_reward(
    |                   ^^^^^^^^^^^^^^^^^^^^

warning: function `sign_task_expire` is never used
   --> src/bottom_white/ledger/system_keypair.rs:608:19
    |
608 |     pub(crate) fn sign_task_expire(
    |                   ^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:627:19
    |
627 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib test) generated 17 warnings (run `cargo fix --lib -p turingosv4 --tests` to apply 11 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running unittests src/lib.rs (target/debug/deps/turingosv4-337b37021c2192ab)

running 1 test
test bottom_white::ledger::rejection_evidence::tests::raw_diagnostic_cid_skipped_in_record_serialization ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 273 filtered out; finished in 0.00s


exec
/bin/bash -lc 'cargo test -p turingosv4 --lib load_from_path_rejects' in /home/zephryj/projects/turingosv4
 succeeded in 261ms:
warning: unused import: `KernelError`
 --> src/bus.rs:8:29
  |
8 | use crate::kernel::{Kernel, KernelError};
  |                             ^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `TapeError`
 --> src/bus.rs:9:54
  |
9 | use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
  |                                                      ^^^^^^^^^

warning: unused imports: `Deserialize` and `Serialize`
  --> src/bus.rs:13:13
   |
13 | use serde::{Deserialize, Serialize};
   |             ^^^^^^^^^^^  ^^^^^^^^^

warning: unused import: `crate::sdk::snapshot::UniverseSnapshot`
 --> src/sdk/actor.rs:7:5
  |
7 | use crate::sdk::snapshot::UniverseSnapshot;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ToolSignal`
 --> src/sdk/tools/search.rs:4:24
  |
4 | use crate::sdk::tool::{ToolSignal, TuringTool};
  |                        ^^^^^^^^^^

warning: unused import: `std::path::Path`
 --> src/sdk/tools/search.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `Deserialize`
 --> src/sdk/tools/librarian.rs:6:13
  |
6 | use serde::{Deserialize, Serialize};
  |             ^^^^^^^^^^^

warning: unused import: `Path`
 --> src/sdk/tools/librarian.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/cas/store.rs:26:12
   |
26 | use git2::{ObjectType as Git2ObjectType, Repository};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/ledger/transition_ledger.rs:36:12
   |
36 | use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: variable does not need to be mutable
   --> src/top_white/predicates/registry.rs:255:17
    |
255 |             let mut h = Sha256::new();
    |                 ----^
    |                 |
    |                 help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: function `task` is never used
   --> src/economy/monetary_invariant.rs:227:8
    |
227 |     fn task(s: &str) -> TaskId {
    |        ^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:560:19
    |
560 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: function `sign_terminal_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:586:19
    |
586 |     pub(crate) fn sign_terminal_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_finalize_reward` is never used
   --> src/bottom_white/ledger/system_keypair.rs:597:19
    |
597 |     pub(crate) fn sign_finalize_reward(
    |                   ^^^^^^^^^^^^^^^^^^^^

warning: function `sign_task_expire` is never used
   --> src/bottom_white/ledger/system_keypair.rs:608:19
    |
608 |     pub(crate) fn sign_task_expire(
    |                   ^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:627:19
    |
627 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib test) generated 17 warnings (run `cargo fix --lib -p turingosv4 --tests` to apply 11 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running unittests src/lib.rs (target/debug/deps/turingosv4-337b37021c2192ab)

running 3 tests
test economy::ledger::tests::load_from_path_rejects_prev_hash_tamper ... ok
test economy::ledger::tests::load_from_path_rejects_logical_t_gap ... ok
test economy::ledger::tests::load_from_path_rejects_entry_hash_tamper ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 271 filtered out; finished in 0.00s


exec
/bin/bash -lc 'cargo test -p turingosv4 --test tb_1_acceptance test_p3_rsp0_total_supply_counts_all_six_subindexes' in /home/zephryj/projects/turingosv4
 succeeded in 289ms:
warning: unused import: `KernelError`
 --> src/bus.rs:8:29
  |
8 | use crate::kernel::{Kernel, KernelError};
  |                             ^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `TapeError`
 --> src/bus.rs:9:54
  |
9 | use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
  |                                                      ^^^^^^^^^

warning: unused imports: `Deserialize` and `Serialize`
  --> src/bus.rs:13:13
   |
13 | use serde::{Deserialize, Serialize};
   |             ^^^^^^^^^^^  ^^^^^^^^^

warning: unused import: `crate::sdk::snapshot::UniverseSnapshot`
 --> src/sdk/actor.rs:7:5
  |
7 | use crate::sdk::snapshot::UniverseSnapshot;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `std::collections::HashSet`
 --> src/sdk/actor.rs:9:5
  |
9 | use std::collections::HashSet;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ToolSignal`
 --> src/sdk/tools/search.rs:4:24
  |
4 | use crate::sdk::tool::{ToolSignal, TuringTool};
  |                        ^^^^^^^^^^

warning: unused import: `std::path::Path`
 --> src/sdk/tools/search.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `Deserialize`
 --> src/sdk/tools/librarian.rs:6:13
  |
6 | use serde::{Deserialize, Serialize};
  |             ^^^^^^^^^^^

warning: unused import: `Path`
 --> src/sdk/tools/librarian.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^

warning: unused imports: `MICRO_PER_COIN` and `MicroCoin`
  --> src/economy/monetary_invariant.rs:26:29
   |
26 | use crate::economy::money::{MicroCoin, MICRO_PER_COIN};
   |                             ^^^^^^^^^  ^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/cas/store.rs:26:12
   |
26 | use git2::{ObjectType as Git2ObjectType, Repository};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/ledger/transition_ledger.rs:36:12
   |
36 | use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_rejected_attempt_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:549:19
    |
549 |     pub(crate) fn sign_rejected_attempt_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:560:19
    |
560 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: function `sign_terminal_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:586:19
    |
586 |     pub(crate) fn sign_terminal_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_finalize_reward` is never used
   --> src/bottom_white/ledger/system_keypair.rs:597:19
    |
597 |     pub(crate) fn sign_finalize_reward(
    |                   ^^^^^^^^^^^^^^^^^^^^

warning: function `sign_task_expire` is never used
   --> src/bottom_white/ledger/system_keypair.rs:608:19
    |
608 |     pub(crate) fn sign_task_expire(
    |                   ^^^^^^^^^^^^^^^^

warning: function `sign_epoch_rotation_proof` is never used
   --> src/bottom_white/ledger/system_keypair.rs:616:19
    |
616 |     pub(crate) fn sign_epoch_rotation_proof(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:627:19
    |
627 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib) generated 19 warnings (run `cargo fix --lib -p turingosv4` to apply 12 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running tests/tb_1_acceptance.rs (target/debug/deps/tb_1_acceptance-9b2fe0f6ff43f57a)

running 1 test
test test_p3_rsp0_total_supply_counts_all_six_subindexes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s


exec
/bin/bash -lc 'cargo test -p turingosv4 --test tb_1_acceptance test_p1_kill_1_no_wtool_bypass' in /home/zephryj/projects/turingosv4
 succeeded in 426ms:
warning: unused import: `KernelError`
 --> src/bus.rs:8:29
  |
8 | use crate::kernel::{Kernel, KernelError};
  |                             ^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `TapeError`
 --> src/bus.rs:9:54
  |
9 | use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
  |                                                      ^^^^^^^^^

warning: unused imports: `Deserialize` and `Serialize`
  --> src/bus.rs:13:13
   |
13 | use serde::{Deserialize, Serialize};
   |             ^^^^^^^^^^^  ^^^^^^^^^

warning: unused import: `crate::sdk::snapshot::UniverseSnapshot`
 --> src/sdk/actor.rs:7:5
  |
7 | use crate::sdk::snapshot::UniverseSnapshot;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `std::collections::HashSet`
 --> src/sdk/actor.rs:9:5
  |
9 | use std::collections::HashSet;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ToolSignal`
 --> src/sdk/tools/search.rs:4:24
  |
4 | use crate::sdk::tool::{ToolSignal, TuringTool};
  |                        ^^^^^^^^^^

warning: unused import: `std::path::Path`
 --> src/sdk/tools/search.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `Deserialize`
 --> src/sdk/tools/librarian.rs:6:13
  |
6 | use serde::{Deserialize, Serialize};
  |             ^^^^^^^^^^^

warning: unused import: `Path`
 --> src/sdk/tools/librarian.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^

warning: unused imports: `MICRO_PER_COIN` and `MicroCoin`
  --> src/economy/monetary_invariant.rs:26:29
   |
26 | use crate::economy::money::{MicroCoin, MICRO_PER_COIN};
   |                             ^^^^^^^^^  ^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/cas/store.rs:26:12
   |
26 | use git2::{ObjectType as Git2ObjectType, Repository};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/ledger/transition_ledger.rs:36:12
   |
36 | use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_rejected_attempt_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:549:19
    |
549 |     pub(crate) fn sign_rejected_attempt_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:560:19
    |
560 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: function `sign_terminal_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:586:19
    |
586 |     pub(crate) fn sign_terminal_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_finalize_reward` is never used
   --> src/bottom_white/ledger/system_keypair.rs:597:19
    |
597 |     pub(crate) fn sign_finalize_reward(
    |                   ^^^^^^^^^^^^^^^^^^^^

warning: function `sign_task_expire` is never used
   --> src/bottom_white/ledger/system_keypair.rs:608:19
    |
608 |     pub(crate) fn sign_task_expire(
    |                   ^^^^^^^^^^^^^^^^

warning: function `sign_epoch_rotation_proof` is never used
   --> src/bottom_white/ledger/system_keypair.rs:616:19
    |
616 |     pub(crate) fn sign_epoch_rotation_proof(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:627:19
    |
627 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib) generated 19 warnings (run `cargo fix --lib -p turingosv4` to apply 12 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running tests/tb_1_acceptance.rs (target/debug/deps/tb_1_acceptance-9b2fe0f6ff43f57a)

running 1 test
test test_p1_kill_1_no_wtool_bypass ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s


exec
/bin/bash -lc 'sha256sum src/economy/ledger.rs src/bottom_white/ledger/rejection_evidence.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
eb17bb85c901c59bec9045cbb6d16ae2f2b08031c35b211756f62ab1c99100f2  src/economy/ledger.rs
2532624348f8acd8220a78868370ed0325310f8237742fc6d31eede0c71d3ee7  src/bottom_white/ledger/rejection_evidence.rs

exec
/bin/bash -lc "rg -n 'src/economy/ledger.rs|src/bottom_white/ledger/rejection_evidence.rs|tests/tb_1_acceptance.rs|src/state/sequencer.rs|src/bus.rs|src/sdk/tools/wallet.rs' genesis_payload.toml" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
19:#     src/kernel.rs, src/wal.rs, src/bus.rs,
146:"src/bus.rs" = "ee231a6bfc445889eaed4da756335d5029331042937e965f63b3a193893379c9"
214:"src/economy/ledger.rs" = "eb17bb85c901c59bec9045cbb6d16ae2f2b08031c35b211756f62ab1c99100f2"
228:"src/state/sequencer.rs" = "34b7f65fa6ca4ff8b5fa2a12acedebf9667ccb1ff9567a060083b8130d1f1aa0"
298:"src/bottom_white/ledger/rejection_evidence.rs" = "2532624348f8acd8220a78868370ed0325310f8237742fc6d31eede0c71d3ee7"

exec
/bin/bash -lc 'git diff --unified=0 -- src/state/sequencer.rs src/bus.rs src/sdk/tools/wallet.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
exec
/bin/bash -lc "git diff --unified=0 -- src/economy/ledger.rs src/bottom_white/ledger/rejection_evidence.rs tests/tb_1_acceptance.rs | rg -n '"'^'"\\+.*(pub fn|pub struct|pub enum|pub type|pub\\(|pub )'" in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
codex
## Q1 (P0-2): PASS
Tier-A test #10 seeds all six holding subindexes with distinct power-of-two amounts and routes only through `assert_total_ctf_conserved`. Dropping any one subindex changes the asserted `delta_micro` from `63 * MICRO_PER_COIN` to a unique lower value; test is in Tier-A, not ignored.

## Q2 (P0-3): PASS
`raw_diagnostic_cid` has `#[serde(skip_serializing, default)]`; the unit test serializes a raw `RejectedSubmissionRecord`, asserts the key is absent, and also asserts `records()[0].raw_diagnostic_cid.is_some()`. Field docs capture the future capability-gated forensic API limitation.

## Q3 (P0-4): PASS
`load_from_path` calls `verify_chain(0, len)?` before `reconstruct_state()?`. The three requested tamper tests exist and pass, and `test_p1_kill_1_no_wtool_bypass` still passes with chain verification catching the final-entry mutation earlier.

## Out-of-scope drift: none
No out-of-scope drift detected.

## Merged disposition: PASS-ALL-THREE
2026-04-29T19:09:52.638350Z ERROR codex_core::session: failed to record rollout items: thread 019ddaa3-eefe-7cc0-8334-08ff1db9ef03 not found
tokens used
57,276
## Q1 (P0-2): PASS
Tier-A test #10 seeds all six holding subindexes with distinct power-of-two amounts and routes only through `assert_total_ctf_conserved`. Dropping any one subindex changes the asserted `delta_micro` from `63 * MICRO_PER_COIN` to a unique lower value; test is in Tier-A, not ignored.

## Q2 (P0-3): PASS
`raw_diagnostic_cid` has `#[serde(skip_serializing, default)]`; the unit test serializes a raw `RejectedSubmissionRecord`, asserts the key is absent, and also asserts `records()[0].raw_diagnostic_cid.is_some()`. Field docs capture the future capability-gated forensic API limitation.

## Q3 (P0-4): PASS
`load_from_path` calls `verify_chain(0, len)?` before `reconstruct_state()?`. The three requested tamper tests exist and pass, and `test_p1_kill_1_no_wtool_bypass` still passes with chain verification catching the final-entry mutation earlier.

## Out-of-scope drift: none
No out-of-scope drift detected.

## Merged disposition: PASS-ALL-THREE
