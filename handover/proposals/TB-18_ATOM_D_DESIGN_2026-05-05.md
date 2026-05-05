# TB-18 Atom D-design — Lifecycle-order configurable resolution (verdict: NO atom D-impl needed in TB-18)

**Status**: **DESIGN-COMPLETE — Class 0 doc; verdict = atom D-impl SKIPPED in TB-18 (deviation from atom D in PRE-17.6 §6 with explicit position per `feedback_architect_deviation_stance`)**.
**Filed**: 2026-05-05 (TB-18 sequence position: after Atom A + Atom H0; before Atom C).
**Authority**:
  - PRE-17.6 deviation §6.D (atom D scope source) — `handover/proposals/TB-17_PRE_17_6_COMPREHENSIVE_ARENA_DEVIATION_2026-05-05.md` §2.2 + §6.4.
  - Architect TB-18 ratification ruling §1 Q2 + §2.7 + §3 (atom D-design step → STOP at Class 4 if reaches sequencer admission semantics) — `handover/directives/2026-05-05_TB18_CHARTER_RATIFICATION_ARCHITECT_RULING.md`.
  - PRE-17.6 §2.2 forensic finding (TB-16.x.2.6 Bankrupt → Expired overwrite precedent) — `handover/evidence/tb_16_x_2_6_smoke_2026-05-05/README.md` §Forensic-findings.

---

## §1 The PRE-17.6 §2.2 constraint verbatim

```text
Per `sequencer.rs:1259-1261`, FORCE_EXPIRE overwrites market state
`Bankrupt → Expired`. After Expired, redeem rejects (`RedeemBeforeResolution:
Expired ∉ {Finalized, Bankrupt}`).

The two refund paths (TaskBankruptcy refund vs CompleteSetRedeem) are
**mutually exclusive within a single market lifecycle** by design —
TaskBankruptcy's expire-on-Bankrupt path is the documented refund path;
the share-redeem path requires Bankrupt-stable state.

Resolution path: make the lifecycle order configurable per task.
Touches sequencer admission semantics — Class 3+ surface.
```

## §2 Today's lifecycle behavior (HEAD = commit `13a5ee0`; verified 2026-05-05)

`TaskMarketState` (in `src/state/q_state.rs`): single covering field — 4 variants `Open / Bankrupt / Expired / Finalized`. Stored as `task_markets_t[task_id].state`.

**Allowed transitions** (audited in `src/state/sequencer.rs:1167-1188 + 1337-1353`):

| From | To | Allowed | Path |
|---|---|---|---|
| Open | Bankrupt | ✅ | TaskBankruptcyTx (line 1342-1353) |
| Open | Expired | ✅ | TaskExpireTx (line 1175-1177) |
| Bankrupt | **Expired** | ✅ | TaskExpireTx (line 1175-1177; **the OVERWRITE — `tm.state = Expired` at line 1261**) |
| Expired | Bankrupt | ✅ | TaskBankruptcyTx (line 1342-1353) |
| Open | Finalized | implicit | (via FinalizeRewardTx; not modeled here) |
| Expired | * | reject | TaskExpire idempotent (line 1178-1184) |
| Bankrupt | * | reject | TaskBankruptcy idempotent (line 1345-1349) |
| Finalized | * | reject | terminal |

The Bankrupt → Expired transition (line 1261) is the OVERWRITE flagged by PRE-17.6 §2.2.

## §3 Why CompleteSetRedeem rejects after Bankrupt → Expired

`src/state/sequencer.rs` CompleteSetRedeem arm (the dispatch position depends on the round; current redeem implementation gates on `state ∈ {Finalized, Bankrupt}`). After `state = Expired`, redeem returns `RedeemBeforeResolution` because Expired is not in the allow set.

This is the constraint #2 PRE-17.6 §2.2 surfaced.

---

## §4 Resolution paths considered

### §4.1 Path A — Per-task config flag on `TaskMarketEntry` (Class 3 additive)

Add `expire_after_bankruptcy: bool` (or similar) field to `TaskMarketEntry` struct. Default `false` for new tasks ⇒ TaskExpire from Bankrupt REJECTS. Default `true` (legacy compatibility) for old chains. Per-task choice at TaskOpen time.

**Scope**: Modifies `TaskMarketEntry` canonical schema in `src/state/q_state.rs` + sequencer admission logic in `src/state/sequencer.rs:1167-1188`.

**Per architect Q2 hard rule**: this MODIFIES `task_markets_t` value type → modifies `economic_state_t` shape → modifies `QState` canonical state schema. While the architect Q2 list does NOT explicitly enumerate "QState schema mutation", it DOES include "sequencer admission semantics" — and admission decisions in TaskBankruptcy + TaskExpire + CompleteSetRedeem ALL depend on `task_markets_t[*].state` reads. Changing the value type is a sequencer admission semantics change.

**Class verdict**: Class 4. STOP per architect Q2.

### §4.2 Path B — `TaskLifecycle = history` (Class 4 schema bump)

Per architect §2.7 verbatim: "state transitions are append-only facts; later lifecycle marker does not erase earlier marker. TaskLifecycle = set / history / enum-with-history; 而不是单个可覆盖字段."

Replace `state: TaskMarketState` (single covering field) with `lifecycle_history: Vec<TaskLifecycleEvent>` where each event = `(state, logical_t, tx_id)`. Queries derive "effective state" from history.

**Scope**: Major canonical schema mutation in `src/state/q_state.rs`. Every Q-state hash recomputes. Existing chains' replay determinism breaks (none of them have `lifecycle_history`; serde-default would produce empty histories that never reproduce the original behavior).

**Class verdict**: **Class 4 — STOP per architect Q2** (canonical state schema + sequencer admission semantics + replay-determinism break for existing chains).

### §4.3 Path C — Multi-task chain (NO atom D-impl needed)

**Insight**: PRE-17.6 §2.2 said "mutually exclusive within a SINGLE MARKET LIFECYCLE". But atom B's substantive `comprehensive_arena.rs` is REQUIRED to drive ≥6 engineered Lean tasks against ONE chain (architect §2.8). **Each task has its own market**; each market has its own lifecycle.

The 13/13 tx-kind coverage Atom B needs:

| TX | Required path | Task |
|---|---|---|
| 1. work | OmegaConfirm or fail | task_A |
| 2. verify | follows work | task_A |
| 3. challenge | targets work | task_A or task_B |
| 4. challenge_resolve | follows challenge | (same task) |
| 5. finalize_reward | OMEGA-Confirm; success path | task_C (no challenge; clean OMEGA) |
| 6. task_open | required for every task | task_A, B, C, D, E, F |
| 7. escrow_lock | required for every task with stake | task_A, B, C, D, E, F |
| 8. complete_set_mint | provider mints shares | task_D (Open → Bankrupt → keep Bankrupt) |
| 9. **complete_set_redeem** | requires `state ∈ {Finalized, Bankrupt}` at redeem time | task_D (state=Bankrupt; NO TaskExpire after) |
| 10. market_seed | provider seeds collateral | task_D |
| 11. terminal_summary | system-emitted on MaxTxExhausted exit | task_E (MaxTx run) |
| 12. **task_expire** | refund Open or Bankrupt task | task_F (Open → Expired direct, OR Bankrupt → Expired) |
| 13. task_bankruptcy | mark task bankrupt | task_D + task_E + task_F |

Notice: **task_D takes the Bankrupt-stable path (CompleteSetRedeem); task_F takes the Bankrupt → Expired path (TaskExpire refund)**. Two SEPARATE tasks in the same chain. PRE-17.6 §2.2's "mutual exclusion within a single market lifecycle" does NOT apply to ACROSS-MARKET coverage.

Atom B drives multi-task → constraint #2 dissolves naturally → no atom D-impl needed in TB-18.

**Class verdict**: **NO-OP for atom D-impl**.

---

## §5 Verdict — Atom D-design conclusion

**Atom D-impl SKIPPED in TB-18.**

Reasons:
1. The PRE-17.6 §2.2 constraint dissolves when atom B uses multi-task structure (Path C).
2. Both Path A (per-task config flag) and Path B (lifecycle history) are Class 4 per architect Q2.
3. Class 4 escalation requires separate ratification + Phase Z′ rerun authorization. Atom D scope was originally "Class 3 OR 4 (TBD by atom design)"; atom D-design here verdicts that ANY direct fix to constraint #2 IS Class 4.
4. Per `feedback_class4_cannot_hide_in_class3`: stop atom D for separate ratification rather than bundle into TB-18 Class 3 envelope.
5. The architect §2.7 "lifecycle append-only" invariant (Path B) is a constitutional desideratum that survives as a TB-19+ forward trigger — NOT a TB-18 ship blocker because Path C closes the immediate atom-B 13/13 need.

**Sequence amendment**:
- Original architect §3 sequence: `... → D-design → C → D-if-Class3 → B → ...`
- Effective sequence post-D-design verdict: `... → D-design (NO-OP verdict) → C → B → ...` (D-impl skipped)

**SG-18.9 declaration** (TB-18 charter §1.4 SG-18.9 verbatim "atom D risk-class status flag"):
- Status flag: **`Class-4-stopped-pending-ratification`** (per Q2; Path A and Path B both Class 4).
- Implementation status: **NOT-IMPLEMENTED-IN-TB-18** (PRE-17.6 §2.2 constraint dissolved by atom B multi-task structure; architect §2.7 invariant deferred to TB-19+ as Class 4 schema upgrade).

**TB-18 ship claim narrowing** (per architect Q2 verbatim):

> 如果 Atom D 不能在 Class 3 内完成，而又没有 Class 4 ratification，那么 TB-18 不能声称:
>   single-chain 13/13 fully closed
> 最多只能声称:
>   formal benchmark substrate partially closed;
>   lifecycle-order constraint remains Class 4 forward trigger

TB-18 ship doc must declare:
```text
Single-chain 13/13: ACHIEVED via multi-task structure (atom B; ≥6 tasks
in ONE chain). PRE-17.6 §2.2 single-market lifecycle-order constraint
remains as Class 4 forward trigger to TB-19+ (architect §2.7 lifecycle-
append-only invariant; Path B canonical schema bump).
```

---

## §6 Forward triggers (TB-19+)

| Trigger | Source | Carry-forward |
|---|---|---|
| **Path B** TaskLifecycle = append-only history (architect §2.7 invariant) | architect §2.7 verbatim | TB-19+ as Class 4 canonical schema upgrade; requires Phase Z′ rerun + replay-determinism migration policy for pre-TB-19 chains |
| **PRE-17.6 §2.2** single-market lifecycle-order configurable | PRE-17.6 §2.2 + this design verdict | TB-19+ co-located with Path B (or separate Class 4 ratification if Path A preferred) |

---

## §7 Cross-references

- TB-18 charter §1.4 FR-18.6 + SG-18.5 + SG-18.9 — `handover/tracer_bullets/TB-18_charter_2026-05-05.md`
- Architect TB-18 ratification ruling §1 Q2 + §2.7 + §3 atom D-design step
- PRE-17.6 deviation §2.2 + §6.4 — `handover/proposals/TB-17_PRE_17_6_COMPREHENSIVE_ARENA_DEVIATION_2026-05-05.md`
- TB-16.x.2.6 forensic findings — `handover/evidence/tb_16_x_2_6_smoke_2026-05-05/README.md`
- Sequencer dispatch arms (audit references): `src/state/sequencer.rs:1160-1402` (TaskExpire + TaskBankruptcy)
- QState lifecycle enum: `src/state/q_state.rs` (TaskMarketState definition)
- Memory bindings: `feedback_class4_cannot_hide_in_class3` + `feedback_architect_deviation_stance` (this verdict states explicit position) + `feedback_no_workarounds_strict_constitution` (Path C is honest analysis, not 凑活).

---

**End of design.** Atom D-impl SKIPPED in TB-18; sequence proceeds to Atom C with explicit Class 4 forward trigger declared.
