# Stage C P-M2 (CompleteSetMergeTx rebuild) — §8 Sign-Off Packet (2026-05-09 session #29)

**Status**: CANDIDATE — awaiting (a) PRE-§8 dual audit verdicts (Codex G2 + Gemini, conservative-wins per `feedback_dual_audit` Class-4 timing rule) and (b) architect verbatim §8 sign-off.
**HEAD at verification**: `7af0db1` (local `main`; NOT pushed to `origin/main`; push gated on architect §8 per `feedback_no_batch_class4_signoff` per-atom cadence).
**Branch trail**: `feat/p-m2-rebuild` → `--no-ff` merge into `main` (commits `9d9a33c` atomic + `57a5b07` hook trail + `7af0db1` merge).
**Origin/main pre-Phase-F.1 baseline**: `ff2d401` (boot prompt commit).
**Authority chain**:
- `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md` §1.C row 1 ("P-M2 CompleteSetMergeTx (rebuild) | 4 STEP_B | Remove `timestamp_logical`; strict 6-field per architect §7.3 | per-atom §8 YES").
- `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md` §7.3 verbatim spec.
- `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md` §3.3 + SG-StageC-PM.* ship gates.
- `feedback_no_batch_class4_signoff` (NO BATCHING — P-M2 is its own atomic §8 cycle; P-M3+ NOT included).
- `feedback_dual_audit` Class-4 PRE-§8 timing rule (added 2026-05-09 from Stage C VETO lesson).

---

## §1. Architect §7.3 verbatim compliance — STRICT 6-field

**Architect manual §7.3 verbatim spec (reproduced exactly)**:

```rust
pub struct CompleteSetMergeTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub event_id: EventId,
    pub owner: AgentId,
    pub amount: ShareAmount,
    pub signature: AgentSignature,
}
```

**As-shipped at HEAD `7af0db1` (`src/state/typed_tx.rs` lines 1265-1273)**:

```rust
pub struct CompleteSetMergeTx {
    pub tx_id: TxId,                          //  1
    pub parent_state_root: Hash,              //  2
    pub event_id: EventId,                    //  3
    pub owner: AgentId,                       //  4
    pub amount: ShareAmount,                  //  5
    pub signature: AgentSignature,            //  6
}
```

**Field-name + type-first-token equality verified by `tests/constitution_architect_verbatim_struct_binding.rs::architect_verbatim_struct_field_bindings`**: P-M2 binding flipped from `LandingStatus::NotYetLanded` → `LandingStatus::Landed` in same commit; gate now enforces strict `(name, type)` pair equality. Codex G2 2026-05-09 defect 3 (`timestamp_logical` drift) is mechanically caught at gate-time if reintroduced.

**Architect §7.3 verbatim semantics block (reproduced exactly)**:

```
require owner YES >= amount
require owner NO >= amount
burn amount YES
burn amount NO
conditional_collateral_t[event] -= amount
balances_t[owner] += amount Coin
```

**As-shipped sequencer accept arm (`src/state/sequencer.rs` lines ~2105-2240)**:
- Step 3a: `pair.yes.units < amount.units` → `InsufficientSharesForMerge` (== "require owner YES >= amount").
- Step 3b: `pair.no.units < amount.units` → `InsufficientSharesForMerge` (== "require owner NO >= amount").
- Step 5a: `pair_mut.yes.units -= amount.units` + `pair_mut.no.units -= amount.units` (== "burn amount YES" + "burn amount NO").
- Step 5b: `collateral_entry -= amount.units as i64` (== "conditional_collateral_t[event] -= amount").
- Step 5c: `balances_t[owner] += amount.units as i64` (== "balances_t[owner] += amount Coin"; 1 share-unit = 1 micro-Coin per CompleteSetMint pattern).

**No event-state gate added** — architect §7.3 semantics block contains zero state-gating clauses. Test `merge_unavailable_after_final_redeem_if_shares_exhausted` formalises that merge is share-balance-bounded (not state-bounded). Strict verbatim alignment per `feedback_no_workarounds_strict_constitution`.

---

## §2. Charter ship gates (FR-PM2.* + SG-StageC-PM.*)

| Gate | Status | Verification |
|------|--------|--------------|
| **FR-PM2.1** NEW `CompleteSetMergeTx` typed-tx schema per architect manual struct | 🟢 PASS | `src/state/typed_tx.rs` 6-field strict; verified by `architect_verbatim_struct_field_bindings` Landed pair-equality |
| **FR-PM2.2** Semantics: require YES + NO; burn both; debit collateral; credit Coin | 🟢 PASS | `src/state/sequencer.rs` accept arm step-by-step matches architect §7.3 verbatim block 1:1 |
| **FR-PM2.3** 5 architect-mandated test names | 🟢 PASS | `tests/constitution_completeset_merge.rs`: `merge_yes_no_returns_coin` + `merge_requires_both_sides` + `merge_conserves_total_coin` + `merge_reduces_collateral` + `merge_unavailable_after_final_redeem_if_shares_exhausted` — all 5 PASS through live `Sequencer::submit_agent_tx` ingress |
| **FR-PM2.4** STEP_B parallel-branch + Trust Root rehash | 🟢 PASS | Branch `feat/p-m2-rebuild` → `--no-ff` merge to local main; 6 trust_root files rehashed (`typed_tx.rs` / `sequencer.rs` / `transition_ledger.rs` / `monetary_invariant.rs` / `verify.rs` / `run_summary.rs`); `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` PASS |
| **SG-StageC-PM.1** Per-phase ship gates | 🟢 PASS | This row's FR-PM2.1..4 |
| **SG-StageC-PM.2** `cargo test --workspace` GREEN; ≥1181 PASS | 🟢 PASS | **1331 PASS / 0 failed / 151 ignored** at HEAD `7af0db1`; +5 above pre-F.1 baseline 1326 (all from new `constitution_completeset_merge` gate) |
| **SG-StageC-PM.3** `bash scripts/run_constitution_gates.sh` GREEN; ≥97 PASS | 🟢 PASS | **198 PASS / 0 failed / 1 ignored** at HEAD `7af0db1`; +5 above pre-F.1 baseline 193 (registered new `constitution_completeset_merge` gate) |
| **SG-StageC-PM.4** Universal forbidden list audit clean | 🟢 PASS | Existing `tests/tb_18d_*` forbidden-list audits unaffected by P-M2 (no new f64 / no new module imports flagged) |
| **SG-StageC-PM.5** Polymarket forbidden list audit clean | 🟢 PASS | `constitution_market_quarantine` gate: 5/5 PASS at HEAD `7af0db1` (legacy CPMM still quarantined; merge introduces no new market substrate beyond what §7.3 names) |
| **SG-StageC-PM.6** Codex G1 charter ratification CLOSED | 🟢 PASS | Charter ratified by parent architect manual §7 + remediation directive §1.C; no separate G1 dispatch required for per-atom rebuild against verbatim spec |
| **SG-StageC-PM.7** G2 Codex + Gemini dual audit AFTER substrate green; conservative ranking | ⏸ DISPATCH PENDING | This packet's §6 dispatches Codex G2 + Gemini PRE-§8 per `feedback_dual_audit` Class-4 timing rule; verdicts cycle in working tree (rollback free); only AFTER both PASS does this packet ascend to architect §8 request |
| **SG-StageC-PM.8** Per-Class-4-atom architect §8 sign-off | ⏸ THIS PACKET | This document IS the §8 packet for P-M2; P-M3+ are NOT included (no batching per `feedback_no_batch_class4_signoff`) |

---

## §3. Phase E mechanism gate verification (defect-class catch witness)

The Phase E + E' + E'' gate set was built to mechanically catch the 4 Codex G2 2026-05-09 audit defects. Three gates are directly relevant to P-M2; verify each:

| Gate | Defect class caught | Status at HEAD `7af0db1` |
|------|---------------------|--------------------------|
| **E.1 verbatim binding** (`tests/constitution_architect_verbatim_struct_binding.rs`) | P-M2 timestamp_logical drift (Codex defect 3) | 🟢 P-M2 binding LANDED with strict `(name, type)` pair equality; CompleteSetMergeSigningPayload sibling binding LANDED (F-DEFERRAL-2 closure per remediation directive §9). Reintroducing `timestamp_logical` would FAIL gate at-time. |
| **E.2 atomic-rollback witness** (`tests/constitution_class4_atomic_rollback_witness.rs`) | P-M6 vacuous rollback test (Codex defect 2) | 🟢 N/A for P-M2 (single-mutation accept arm; not a 9-step composite). Gate continues to enforce against future P-M6 rebuild. |
| **E.3 strict-equality lint** (`tests/constitution_economy_strict_equality.rs`) | P-M6 `min()` weakening of CTF invariant (Codex defect 1) | 🟢 N/A for P-M2 (uses `assert_complete_set_balanced` unchanged; no new aggregate reduction introduced). |

**Defect-class prevention witness**: Phase E.1 was specifically designed to catch the P-M2 `timestamp_logical` drift class. Flipping P-M2 binding to Landed in the same commit as the rebuild is the gate-test exercise — if the implementation had drifted from architect §7.3, the binding flip would have failed the build before commit. The mechanism worked.

---

## §4. Atom-by-atom completion table

| Step | Class | Commit | Status |
|------|-------|--------|--------|
| F.1.0 Read architect §7.3 verbatim + P-M0/P-M1 baseline | 0 | (in-session research) | ✅ |
| F.1.1 Branch + implement CompleteSetMergeTx | 4 STEP_B | `9d9a33c` | ✅ |
| F.1.2 Sequencer admission arm | 4 STEP_B | `9d9a33c` | ✅ |
| F.1.3 5 verbatim tests + register gate | 1 + 0 | `9d9a33c` | ✅ |
| F.1.4 Flip E.1 P-M2 to Landed + add SigningPayload binding | 0 (test) | `9d9a33c` | ✅ |
| F.1.5 Trust Root rehash (6 files) | 4 STEP_B | `9d9a33c` | ✅ |
| F.1.6 Full validation (cargo + gates + Trust Root) | — | (HEAD verification) | ✅ |
| F.1.7 Atomic commit + `--no-ff` merge to local main | 0 | `9d9a33c` + `57a5b07` + `7af0db1` (merge) | ✅ |
| F.1.8 Draft §8 packet + dispatch dual audit PRE-§8 | 3 audit | (this document) | ⏸ DISPATCHING |
| F.1.9 Architect §8 wait + post-ship updates (push, LATEST.md, MEMORY.md, F.2 hand-off) | 0 + 4 ship | — | ⏸ AFTER §8 |

**STEP_B parallel-branch protocol** (Class-4 surfaces): all `src/state/typed_tx.rs` + `src/state/sequencer.rs` changes were developed on `feat/p-m2-rebuild` branch and merged via `--no-ff` only after `cargo test --workspace` GREEN + Trust Root PASS + E.1 binding gate flipped to Landed in same commit.

**No `cas/schema.rs` change**: CompleteSetMerge does not introduce a new CAS `ObjectType` — its payload routes through the standard `LedgerEntry.tx_payload_cid` path identical to CompleteSetMint / CompleteSetRedeem / MarketSeed. Hence schema.rs is NOT modified, NOT rehashed, and NOT in the trust_root delta.

---

## §5. FC1 invariant statement

P-M2 does NOT touch the externalized-attempt accounting path (`src/runtime/evaluator.rs` 6-paths, `src/runtime/attempt_telemetry.rs::r2_write_attempt_telemetry`). FC1 hard invariant `evaluator_reported_completed_llm_calls == l4_work_attempt_count + l4e_work_attempt_count + capsule_anchored_attempt_count` (per CLAUDE.md §6) holds bit-for-bit at HEAD `7af0db1` because:

- No new Lean / LLM call site introduced.
- No new evaluator path or counter added.
- `tool_dist.{step, parse_fail, llm_err}` accounting unaffected.
- CompleteSetMergeTx is an agent-signed economic mutation with no proof-attempt or evaluator-counter coupling.

The `constitution_fc1_runtime_loop` gate continues to PASS at HEAD `7af0db1` (counted in the 198 GREEN total).

---

## §6. Genesis-replayability statement

P-M2 preserves FC2 boot replayability:

- `genesis_payload.toml [trust_root]` updated for 6 STEP_B files (typed_tx.rs / sequencer.rs / transition_ledger.rs / monetary_invariant.rs / verify.rs / run_summary.rs). `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` PASS at HEAD `7af0db1`.
- No new `genesis_report` field; no new system-pubkey requirement; no new `on_init` mint surface.
- Replay reconstruction (`HeadTWitness::reconstruct_from_chaintape_refs` per Stage A3 multi-ref) unchanged — CompleteSetMerge appends `LedgerEntry { tx_kind: TxKind::CompleteSetMerge = 14, tx_payload_cid, ... }` through the same `transition_ledger::append` path as P-M0/P-M1 variants. No hidden filesystem pointer introduced.
- Constitution gates `constitution_fc2_boot` + `constitution_head_t_c2_multi_ref` + `constitution_no_parallel_ledger` + `markov_pointer_de_canonicalize` continue to PASS at HEAD `7af0db1` (counted in the 198 GREEN total).

A fresh replay from `genesis_report + ChainTape + CAS + agent_registry + system_pubkeys` reconstructs identically to runtime state per FC2 invariant.

---

## §7. F-DEFERRAL closure status

| Deferral | Status | Closure |
|----------|--------|---------|
| **F-DEFERRAL-1** (E.3 helper-alias scope) | 🟢 N/A for P-M2 | Phase F.1 introduces no helper-alias for CTF conservation logic. `monetary_invariant.rs` `assert_complete_set_balanced` remains the single source of truth; CompleteSetMerge accept arm calls it unchanged (no aliasing surface introduced). |
| **F-DEFERRAL-2** (E.1 signing-payload binding) | 🟢 CLOSED for P-M2 | `tests/constitution_architect_verbatim_struct_binding.rs` BINDINGS array now contains a sibling entry for `CompleteSetMergeSigningPayload` (Landed; 5-field architect-verbatim projection). Architect-permitted projection: 6 wire fields minus `signature` (mirrors existing TB-13 signing-payload pattern). Audit witness: `grep "SigningPayload" tests/constitution_architect_verbatim_struct_binding.rs` shows the new entry; total bindings now 2 (P-M2 wire + P-M2 signing). |

Per remediation directive §9, F-DEFERRAL-2 is per-atom; P-M4 and P-M6 rebuilds will add their own SigningPayload bindings at Phase F.3 / F.5 time.

---

## §8. NO BATCHING declaration

Per `feedback_no_batch_class4_signoff` (added 2026-05-09 from Stage C session #27 batch §8 VETO lesson):

> Every Class-4 atom requires its own per-atom architect §8; never batch.

This packet requests architect §8 for **P-M2 ALONE**. P-M3 (MarketSeedTx hardening, Class-3) does NOT proceed until P-M2 is ratified + pushed. P-M4 / P-M5 / P-M6 / P-M7 / P-M8 / P-M9 are NOT included in this sign-off.

The forward sequence per remediation directive §1.B + `feedback_no_batch_class4_signoff` is strictly:

```
F.1 P-M2 (Class-4) → §8 → push → F.2 P-M3 (Class-3, no §8) → F.3 P-M4 (Class-4) → §8 → push → ... → F.9 Stage C overall §8
```

No deviation from per-atom cadence is requested or implied.

---

## §9. PRE-§8 dual audit dispatch (this section's request)

Per `feedback_dual_audit` Class-4 timing rule (PRE-§8 dispatch at packet draft time, not after architect §8 request — added 2026-05-09 per remediation directive §1.B.5):

**Audit target**: HEAD `7af0db1` (local `main`); commit `9d9a33c` for the atomic P-M2 SHIPPED commit.

**Audit scope**:
1. **Verbatim alignment**: verify `CompleteSetMergeTx` 6-field struct + `CompleteSetMergeSigningPayload` 5-field signing projection match architect manual §7.3 exactly. NO `timestamp_logical` (Codex G2 2026-05-09 defect 3 prevention).
2. **Test body realism**: verify each of the 5 architect-mandated test names exercises the live sequencer accept arm with non-trivial assertions (not vacuous `assert!(true)` / not gate-bypassing fixture). Each test must reach `submit_and_apply` → `dispatch_transition` → `q_next` mutation.
3. **F-DEFERRAL closure**: verify F-DEFERRAL-2 SigningPayload binding is present in `tests/constitution_architect_verbatim_struct_binding.rs` BINDINGS as a sibling Landed entry. F-DEFERRAL-1 vacuously closed (no helper alias).
4. **Mechanism gate health**: confirm Phase E.1 / E.2 / E.3 gates remain GREEN at HEAD `7af0db1` and are not bypassed.
5. **No regression**: confirm `cargo test --workspace` 1331 PASS + `bash scripts/run_constitution_gates.sh` 198 PASS at HEAD `7af0db1`.
6. **Trust Root integrity**: confirm `boot::verify_trust_root_passes_on_intact_repo` PASS post-rehash; 6 trust_root entries updated correctly.

**Conservative-wins ranking** (per `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

**Dispatch protocol**:
- Codex G2: dispatched via `Agent` tool with subagent_type `codex:codex-rescue` from this session.
- Gemini parallel: dispatched via TuringOS Gemini wrapper (or marked degraded if unavailable per `feedback_dual_audit`).
- Round cap: 2 rounds (per `feedback_elon_mode_policy` elon-mode); round 3 needs explicit user authorization.

**Outcome routing**:
- Both PASS → architect §8 request.
- Either VETO → roll back P-M2 commit; reopen with patches; re-dispatch.
- Either CHALLENGE → patch in-place; re-dispatch (within round cap).

---

## §10. What this candidate sign-off ratifies (if signed)

If signed by the architect via verbatim multi-clause form (`好，确认可以 ship` or `同意 sign-off`), this directive ratifies:

| Atom | Class | Commit | Description |
|------|-------|--------|-------------|
| P-M2 atomic | 4 STEP_B | `9d9a33c` | CompleteSetMergeTx 6-field verbatim + admission arm + agent-sig verify + 5 architect tests + E.1 LANDED + F-DEFERRAL-2 closure |
| P-M2 hook trail | 0 | `57a5b07` | rules/enforcement.log R-022-PASS audit append |
| P-M2 STEP_B merge | 0 | `7af0db1` | --no-ff merge to local main |

**Cumulative trajectory at sign-off**:
- Constitution gates: 193 (pre-Phase-F.1 baseline) → **198** (+5 from `constitution_completeset_merge`).
- Workspace tests: 1326 (pre-Phase-F.1 baseline) → **1331** (+5 verbatim merge tests).
- Trust Root entries rehashed: **6** (typed_tx.rs / sequencer.rs / transition_ledger.rs / monetary_invariant.rs / verify.rs / run_summary.rs).
- Stage C atom rebuilds shipped: **1 of 3 Class-4** (P-M2 done; P-M4 + P-M6 remain pending Phase F.3 / F.5).

---

## §11. Architect §8 sign-off action

If the architect agrees, please respond on this directive (or a new sign-off doc at `2026-05-09_STAGE_C_POLYMARKET_PM2_§8_SIGN_OFF.md`) with verbatim form:

> **好，确认可以 ship**

or

> **同意 sign-off**

(single-clause forms like `ok`, `go`, `继续` do NOT constitute Class-4 §8 per CLAUDE.md §9.)

After §8:
1. AI assistant requests user authorization to push HEAD `7af0db1` to `origin/main`.
2. Update `handover/ai-direct/LATEST.md` (mark P-M2 SHIPPED FINAL; close Stage C VETO row's "待重建" status for P-M2).
3. Update `MEMORY.md` Active state row.
4. Move to **F.2 P-M3 re-apply** (Class-3, no §8 needed; renames `event_id_kind` → `event_id` per architect §7.5 verbatim — wait, that's P-M4. P-M3 is MarketSeedTx hardening — collateral-backed already; no Class-4 surface).

---

**End of P-M2 §8 sign-off candidate packet.**
