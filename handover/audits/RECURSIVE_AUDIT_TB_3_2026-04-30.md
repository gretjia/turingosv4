# TB-3 Recursive Audit — Charter v2 + Architect 14 Decisions vs. Implementation

**Date**: 2026-04-30
**Branch**: `experiment/tb3-rsp1-formal-tx-surface` HEAD `2eee4ee`
**Charter**: `handover/tracer_bullets/TB-3_charter_2026-04-30.md` (DRAFT v2)
**Architect verdict**: 14 decisions from user 2026-04-30 chat session.
**Auditor mode**: self-audit (no external dual audit per user authorization 2026-04-30 — "我不认为需要再双审了, 可以直接进入开发, 但是开发完要对照架构师意见做recursive audit").

This audit walks each of the 14 architect decisions + 4 charter § 3 decision blocks + 15 charter § 5 forbidden lines against the implemented code on the experiment branch. Each row cites the src line / test name that enforces or proves the decision.

---

## 1. Architect 14 decisions — line-grounded verification

| # | Decision | Verification | Verdict |
|---|---|---|---|
| 1 | WorkTx schema 不加 `stake_lock_tx_id` | `src/state/typed_tx.rs:223-235` — WorkTx still has 11 wire fields (12 with TxStatus elision); no `stake_lock_tx_id` field. `grep -rn stake_lock_tx_id src/` returns zero hits. | ✅ |
| 2 | 不新增 YesStakeTx variant | `src/state/typed_tx.rs:729-739` — TypedTx enum has 9 variants: Work, Verify, Challenge, Reuse, FinalizeReward, TaskExpire, TerminalSummary, **TaskOpen, EscrowLock**. No YesStakeTx. `grep -rn "pub struct YesStakeTx\|YesStake(" src/` zero hits. | ✅ |
| 3 | Roadmap `yes_stake_tx` 解释为 WorkTx.stake 语义角色 | `src/state/typed_tx.rs:223-235` doc-comment + charter § 3.1 explicit ROADMAP↔WP layered ordering. WorkTx.stake field at `:233` is the canonical inline stake commitment. | ✅ |
| 4 | TB-3 新增 TypedTx 只包括 TaskOpenTx + EscrowLockTx | `src/state/typed_tx.rs:380` (TaskOpenTx) + `:403` (EscrowLockTx). Exactly 2 new variants. TypedTx enum at `:737-738` adds exactly these two. | ✅ |
| 5 | TaskMarketsIndex 迁移为 `BTreeMap<TaskId, _>` | `src/state/q_state.rs:251` — `pub struct TaskMarketsIndex(pub BTreeMap<TaskId, TaskMarketEntry>)` (was `BTreeMap<TxId, _>`). Migration commit `9af6d80` Atom 2. | ✅ |
| 6 | `bounty` 字段删除, 改为 `total_escrow` + `escrow_lock_tx_ids` | `src/state/q_state.rs:260-275` — TaskMarketEntry: `total_escrow: MicroCoin` + `escrow_lock_tx_ids: BTreeSet<TxId>` + `settlement_rule_hash: Hash`; NO `bounty` field. `grep -n "pub bounty:" src/state/q_state.rs` zero hits. | ✅ |
| 7 | `total_escrow` 是 derived aggregate, 不计入 holding | `src/economy/monetary_invariant.rs:94-118` — `total_supply_micro` sums 5 holdings (balances + escrows + stakes + claims + bond); explicit comment `// task_markets_t.total_escrow is INTENTIONALLY OMITTED — derived cache, not a holding`. Atom 2 commit `9af6d80`. | ✅ |
| 8 | 新 predicate `task_market.total_escrow == Σ escrows[e].amount where e.task_id == task_id` | `src/economy/monetary_invariant.rs:120-160` — `pub fn assert_task_market_total_escrow_matches_locks` enforces cache=truth. Test `task_market_total_escrow_matches_sum_of_escrow_locks` at `:561-602` verifies. Plus `total_supply_does_not_double_count_total_escrow` regression at `:526-559`. | ✅ |
| 9 | TaskOpenTx metadata-only, 不检查 sponsor balance, 不动钱 | `src/state/sequencer.rs:312-337` — TaskOpen arm: NO balance check, NO money mutation; only inserts TaskMarketEntry with `total_escrow: MicroCoin::zero()` + empty `escrow_lock_tx_ids`. Test U4 `dispatch_task_open_inserts_task_market_entry` verifies (balances + escrows untouched). | ✅ |
| 10 | EscrowLockTx 唯一 bounty funding path, 检查 balance, debit/credit, monetary_invariant 空 exempt | `src/state/sequencer.rs:344-413` — EscrowLock arm: balance check at step 3; balance debit + escrow credit + cache update at step 4; `assert_total_ctf_conserved(before, after, &[])` at step 5 (empty exempt list). Tests U6 + U7 + U8 + I21 + I22 verify. | ✅ |
| 11 | WorkTx admission 需要 `task_market.total_escrow > 0` + `work.stake > 0` + (NEW) `balance >= stake` | `src/state/sequencer.rs:243-272` — Step 4 (stake > 0), Step 5 (total_escrow > 0 via `task_markets_t.get(&work.task_id).map_or(false, |m| m.total_escrow.micro_units() > 0)`), Step 6 (solver solvency `balances_t[agent] >= work.stake`). Tests U9 + U10 + I24 + I25 + I26 verify. | ✅ |
| 12 | Bridge resurrection CI invariant | `tests/tb_3_bridge_deletion_invariant.rs` — Rust-native scanner over `src/**/*.rs`; asserts zero hits for `TxId(work.task_id.0.clone())` outside comment lines. Plus positive-control test verifies scanner works. Atom 7 commit `2eee4ee`. | ✅ |
| 13 | `RejectionClass::InsufficientBalance` 新增 (L4.E taxonomy) | `src/bottom_white/ledger/rejection_evidence.rs:56-75` — `RejectionClass::InsufficientBalance = 5` (additive; existing variants 0-4 unchanged). Mapped from `TransitionError::InsufficientBalance` at `src/state/sequencer.rs:131-134`. Test I26 verifies the new class fires correctly. | ✅ |
| 14 | L4.E 不能偷偷修改 economic_state; slash 通过未来显式 ChallengeResolveTx | `src/state/sequencer.rs::apply_one` — rejection path at the existing TB-2 implementation calls `rejection_writer.append_rejected` only; does NOT touch `q_w` or `economic_state`. **Test I28 `rejected_worktx_does_not_change_balances_escrows_stakes`** at `tests/tb_3_rsp1_formal_surface.rs:341-368` proves bit-identical pre/post `economic_state_t` for a rejected WorkTx. Slash deferred to RSP-2/3 explicit accepted ChallengeResolveTx (charter § 3.4 final paragraph). | ✅ |

---

## 2. Charter § 3 decision blocks — line-grounded verification

### § 3.1 WP-canonical implementation shape

| Item | Verification |
|---|---|
| `WorkTx.stake` remains inline | `src/state/typed_tx.rs:233` `pub stake: StakeMicroCoin` ✅ |
| No `YesStakeTx` TypedTx variant | TypedTx enum at `:729-739` has no YesStake variant ✅ |
| ROADMAP `yes_stake_tx` ↔ `WorkTx.stake` semantic mapping | Documented in charter § 3.1 + WorkTx arm doc-comment ✅ |
| Only 2 new variants (TaskOpen + EscrowLock) | `src/state/typed_tx.rs:380, :403` ✅ |
| WorkTx admission steps: stake > 0 + total_escrow > 0 + balance ≥ stake | `src/state/sequencer.rs:243-272` ✅ |

### § 3.2 No-double-counting decision

| Item | Verification |
|---|---|
| EscrowsIndex is source of truth for escrowed money | `src/state/q_state.rs:172-177` — `EscrowsIndex(BTreeMap<TxId, EscrowEntry>)` with EscrowEntry.amount the holding ✅ |
| `TaskMarketEntry.total_escrow` is derived aggregate / cached index, NOT a holding | `src/state/q_state.rs:264-265` doc-comment explicitly states "derived aggregate / cached index, NOT a money holding" ✅ |
| `total_supply_micro` does NOT count `task_markets_t.total_escrow` | `src/economy/monetary_invariant.rs:94-118` — comment `// task_markets_t.total_escrow is INTENTIONALLY OMITTED` ✅ |
| 6 → 5 holding migration | `total_supply_micro` sums 5 holdings (balances + escrows + stakes + claims + bond); test renamed `_six_` → `_five_` ✅ |
| Cache=truth invariant test | `task_market_total_escrow_matches_sum_of_escrow_locks` + `total_supply_does_not_double_count_total_escrow` ✅ |

### § 3.3 TaskOpen / EscrowLock semantics

| Item | Verification |
|---|---|
| TaskOpenTx metadata-only (no money) | `src/state/sequencer.rs:312-337` — only inserts TaskMarketEntry; no balance / escrow mutation ✅ |
| TaskOpenTx does NOT check sponsor balance | No balance check in TaskOpen arm ✅ |
| TaskOpenTx does NOT make WorkTx admissible by itself | `total_escrow == 0` after TaskOpen alone; WorkTx admission step 5 fails with EscrowMissing. Test I25 `submit_worktx_without_escrow_lock_appends_l4e_escrow_missing` ✅ |
| Idempotency: TaskAlreadyOpen reject | Test U5 `dispatch_task_open_rejects_when_already_open` ✅ |
| EscrowLockTx is the only RSP-1 bounty funding path | grep confirms: only `dispatch_transition`'s EscrowLock arm inserts into `escrows_t` and grows `task_markets_t[t].total_escrow`. Test fixtures use accepted-tx submission per charter § 5.6 ✅ |
| EscrowLockTx checks balance | `src/state/sequencer.rs:354-364` step 3 ✅ |
| EscrowLockTx debits balances + credits escrows + updates total_escrow + escrow_lock_tx_ids | `src/state/sequencer.rs:367-388` step 4 ✅ |
| `monetary_invariant` empty exempt list | `src/state/sequencer.rs:393-398` — `assert_total_ctf_conserved(before, after, &[])` ✅ |
| Pre-condition: task_markets_t[task_id] EXISTS or TaskNotOpen | `src/state/sequencer.rs:351-353` step 2 ✅ |

### § 3.4 WorkTx stake — lock-on-accept

| Item | Verification |
|---|---|
| Admission-time check: `balances_t[agent] >= work.stake` | `src/state/sequencer.rs:264-271` step 6 ✅ |
| Acceptance-time mutation: balance debit + stakes_t insert | `src/state/sequencer.rs:280-303` step 8 ✅ |
| `assert_total_ctf_conserved(q, q_next, &[])` enforces conservation | `src/state/sequencer.rs:305-311` step 9 ✅ |
| Rejection: economic_state_t UNCHANGED | Test I28 `rejected_worktx_does_not_change_balances_escrows_stakes` ✅ |
| L4.E never mutates economic state | apply_one rejection path verified by I28 ✅ |
| Slashing deferred to RSP-2/3 ChallengeResolveTx | Charter § 3.4 + `Verify`/`Challenge` arms remain `NotYetImplemented` at `:243-249` ✅ |
| YES/NO explicit split deferred to RSP-2 | `StakeEntry.amount: MicroCoin` (single field); no YES/NO subfields ✅ |

---

## 3. Charter § 5 forbidden red lines — verification

| # | Red line | Verification |
|---|---|---|
| 1 | No P5/P6/P7/P8 work | No new files under `src/meta/` or `src/multi_org/`; no h_vppu changes ✅ |
| 2 | No `economy::ledger::AcceptedLedger` as production accepted L4 | `apply_one` uses `transition_ledger::LedgerWriter`; AcceptedLedger untouched ✅ |
| 3 | No non-empty `exempt_tx_kinds` at runtime | `assert_total_ctf_conserved(before, after, &[])` at every dispatch arm ✅ |
| 4 | No kernel.rs / bus.rs / wallet.rs edits | `git diff main..HEAD --stat -- src/kernel.rs src/bus.rs src/sdk/tools/wallet.rs` returns empty ✅ |
| 5 | No YesStakeTx variant | Verified above ✅ |
| 6 | No WorkTx schema bump | Verified above ✅ |
| 7 | No challenge / verify / settlement work | `Verify` / `Challenge` / `Reuse` / `FinalizeReward` / `TaskExpire` / `TerminalSummary` arms remain `NotYetImplemented` ✅ |
| 8 | No reputation_update_tx | No new reputation work ✅ |
| 9 | No new EconomicState sub-fields | `q_state.rs::tests::economic_state_has_nine_sub_fields` still passes ✅ |
| 10 | No `task_markets_t.total_escrow` in any holding sum | `total_supply_micro` verified above; `total_supply_does_not_double_count_total_escrow` test ✅ |
| 11 | No L4.E mutation of economic_state | Test I28 ✅ |
| 12 | No per-node automatic liquidity injection | EscrowLock is the only `task_markets_t.total_escrow` writer; explicit balance debit per call ✅ |
| 13 | No ghost liquidity | All TB-3 fixtures use accepted-tx submission via `Sequencer::submit` (charter § 5.6); no direct `task_markets_t.total_escrow` setter outside dispatch arms ✅ |
| 14 | No same-charter retry on failure | Acceptance battery passes; no failure to retry ✅ |
| 15 | No bridge resurrection | `tests/tb_3_bridge_deletion_invariant.rs` enforces as CI invariant ✅ |

---

## 4. Charter § 7 three ship proofs — test coverage

### Proof 1 — formal admission spine + atomic balance/escrow flow
- I20 `submit_task_open_tx_appends_to_canonical_l4` — TaskOpen → 1 L4 row, 0 L4.E
- I21 `submit_escrow_lock_tx_appends_to_canonical_l4` — EscrowLock → 1 L4 row, 0 L4.E
- I22 `escrow_lock_atomic_balance_to_escrow_transfer` — 100 → 70 + 30 escrow, total_escrow cache holds, CTF conserved

### Proof 2 — bridge-deleted admission + accepted-WorkTx stake commitment
- I23 `submit_worktx_via_formal_surface_advances_state_root_and_locks_stake` — full happy path
- I24 `submit_worktx_without_task_open_appends_l4e_task_not_open` — TaskNotOpen → EscrowMissing
- I25 `submit_worktx_without_escrow_lock_appends_l4e_escrow_missing` — total_escrow == 0 → EscrowMissing
- I26 `submit_worktx_with_insufficient_solver_balance_appends_l4e_insufficient_balance` — NEW L4E class
- I27 `submit_worktx_with_zero_stake_appends_l4e_stake_insufficient` — TB-2 inheritance preserved
- I28 `rejected_worktx_does_not_change_balances_escrows_stakes` — L4.E never mutates economic state
- `bridge_pattern_does_not_resurrect_in_src` — CI invariant

### Proof 3 — replay invariant + ghost-liquidity impossibility + cache=truth
- I29 `replay_from_l4_only_reconstructs_economic_state` — 3-row L4 replay; cache=truth holds; CTF conserved
- I30 `property_no_sequence_violates_total_ctf_conservation` — deterministic 10-step sequence over {TaskOpen, EscrowLock + top-up, WorkTx accept, WorkTx reject, idempotency reject} preserves CTF + cache=truth at every accepted step
- TB-2 I13 `runtime_replay_from_l4_only_ignores_l4e` — extended to 3 L4 rows post-Atom-6; replay_full_transition reaches matching state_root + ledger_root

---

## 5. Test count summary

| Suite | Count | New TB-3 | Notes |
|---|---|---|---|
| `state::sequencer::tests` (in-crate unit) | 17 | 8 (U4-U11) | U3 migrated to lock-on-accept assertion |
| `state::typed_tx::tests` | 25 | 5 (T1-T5) | Goldens preserved |
| `economy::monetary_invariant::tests` | 13 | 3 | `_five_` rename + double-count regression + cache=truth |
| `tests/tb_3_rsp1_formal_surface.rs` | 11 | 11 (I20-I30) | Full charter § 7 ship proofs |
| `tests/tb_3_bridge_deletion_invariant.rs` | 2 | 2 | CI invariant + positive control |
| `tests/tb_2_runtime_boundary.rs` | 13 | 0 | I9-I13 fixture migrated; all preserved |
| All other suites | 460 | 0 | TB-2 + earlier preserved bit-identical |
| **Total** | **541** | **29** | **0 FAILED** |

---

## 6. Verdict

| Dimension | Status |
|---|---|
| Architect 14 decisions | 14/14 verified ✅ |
| Charter § 3 decision blocks | 4/4 verified ✅ |
| Charter § 5 forbidden red lines | 15/15 verified ✅ |
| Charter § 7 three ship proofs | 3/3 covered by integration tests ✅ |
| `cargo test --workspace` | 541 PASS / 0 FAILED ✅ |
| `cargo check --workspace` | clean (only pre-existing warnings) ✅ |
| Trust Root immutability | PASS post-rehash ✅ |

**TB-3 is ship-ready** vs the architect verdict + charter v2 + all 15 forbidden lines. 真题烟测 (`--smoke` Phase-C living regression) is the next gate.

---

## 7. Atom commit chain (experiment branch)

```
2eee4ee Atom 7 — Replay + property + bridge-resurrection invariants
fa85350 Atom 6 — WorkTx arm refactor: delete bridge + lock-on-accept
af807d1 Atom 5 — EscrowLock dispatch arm + apply_one + U6/U7/U8 + I21/I22
7c116dd Atom 4 — TaskOpen dispatch arm + apply_one + U4/U5 + I20
6757d40 Atom 3 — TypedTx ABI: TaskOpenTx + EscrowLockTx + 3 TransitionError + 1 L4ERejectionClass
9af6d80 Atom 2 — q_state schema migration + monetary_invariant 6→5 holding
0fb8dc3 (main) TB-3 charter v2 + STEP_B Phase-0 preflight v1
```

Total diff vs `main` (db2ad68 → 2eee4ee): 6 atom commits + 1 charter commit; ~1500 lines added across src + tests.

---

## 8. Cross-references

- Charter v2: `handover/tracer_bullets/TB-3_charter_2026-04-30.md`
- Phase-0 preflight v1: `handover/ai-direct/TB-3_RSP1_FORMAL_TX_SURFACE_2026-04-30.md`
- Memory `feedback_wp_vs_roadmap_reconciliation` (auto-loaded): durable rule from this cycle
- Architect verdict 2026-04-30 (chat session; not file-archived): the 14 decisions audited above
- TB-2 ship verdict (predecessor): `handover/audits/DUAL_AUDIT_TB_2_PHASE1C_VERDICT_2026-04-30.md`
