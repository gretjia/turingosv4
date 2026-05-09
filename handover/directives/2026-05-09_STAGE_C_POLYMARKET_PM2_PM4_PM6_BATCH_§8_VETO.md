# Stage C Polymarket — P-M2 + P-M4 + P-M6 Batch §8 — ARCHITECT VETO

**Date**: 2026-05-09 session #28
**Authority**: User (architect-role) verbatim 2026-05-09: **「我是要 VETO + 全 rollback」** (multi-clause: stated verdict `VETO` + named remediation method `全 rollback` = full revert).
**Trigger**: G2 Codex audit (architect-proxy) returned aggregate VETO with 4 specific defect findings.
**Companion remediation directive**: `2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md`
**Packet under review**: `2026-05-09_STAGE_C_POLYMARKET_PM2_PM4_PM6_BATCH_§8_PACKET.md` (HEAD `7395aaa` at packet draft)

---

## §1. Authority chain

- User `auto mode` engaged 2026-05-09 with explicit delegation: "我没有任何信息去做判断，你根据事实依据和架构师的完整意见去做判断" → "调用 codex 审计来决定 veto 还是 challenge".
- Codex G2 audit dispatched (foreground, single-agent — Gemini parallel dual-audit deferred per user single-codex instruction).
- Codex aggregate verdict **VETO** (conservative-wins per `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS).
- User architect-role verbatim VETO: **「我是要 VETO + 全 rollback」**.
- Per CLAUDE.md §10, this directive operationalizes the VETO; remediation directive (companion file) operationalizes the rebuild.

---

## §2. Codex G2 audit findings (verbatim)

### P-M6 (Mint-and-Swap Router) — VETO (load-bearing)

**Defect 1 — Monetary invariant weakened to `min()`**:
- `src/economy/monetary_invariant.rs:516` accepts `min(sum_yes, sum_no) == collateral` instead of strict `sum_yes == collateral && sum_no == collateral`.
- Violates CLAUDE.md §13 economy laws ("1 Coin = 1 YES + 1 NO" + "no ghost liquidity") and architect manual §6.1 CTF invariant.
- Load-bearing: with CPMM pool reserves, YES and NO can temporarily diverge during swaps; `min()` admits ghost-liquidity edge cases.

**Defect 2 — Vacuous `router_atomic_rollback_on_failure` test**:
- `tests/constitution_router_buy_with_coin.rs:532` only triggers insufficient-balance failure.
- `src/state/sequencer.rs:2469` rejects insufficient-balance BEFORE `q_next` mutation begins (`:2514`).
- Test name verbatim-correct per architect §7.7 but body fails to exercise the 9-step composite atomic-rollback path.
- Load-bearing: no tape evidence proves atomicity; FC1 "tape-first" implicit invariant violated for composite tx.

### P-M4 (CpmmPool) — CHALLENGE (verbatim drift)

**Defect 3 — Schema field rename**:
- `src/state/q_state.rs:694` uses `event_id_kind`; architect §7.5 verbatim requires `event_id`.
- Non-load-bearing per Codex but breaks Stage A2/A3 verbatim-binding contract precedent.

### P-M2 (CompleteSetMergeTx) — CHALLENGE (verbatim drift)

**Defect 4 — Extra struct field**:
- `src/state/typed_tx.rs:1417` adds `timestamp_logical` field; architect §7.3 verbatim specifies 6 fields only.
- Field is also signed at `src/state/typed_tx.rs:1491` — schema deviation propagates to canonical signing payload.

### CR-StageC-PM.16 deviation — REJECTED

Charter `STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:201` verbatim:
> NO Class-4 typed-tx schema bump bundled across atoms. Each Class-4 atom (P-M2 / P-M4 if needed / P-M6 if needed) is its own STEP_B with per-atom architect §8 sign-off.

Batch packet §6 acknowledged the deviation; mitigation (intra-batch self-audit) was insufficient because:
- Self-audit didn't catch struct-field drift (P-M2 + P-M4)
- Self-audit didn't catch invariant weakening (P-M6 defect 1)
- Self-audit didn't catch test vacuity (P-M6 defect 2)

Codex external review caught all four. This validates `feedback_dual_audit` Class-4 = full dual policy.

---

## §3. Verdict

**Aggregate**: VETO (conservative-wins; P-M6 load-bearing).
**Per-atom**:
- P-M2: VETOED (cascade per packet §6 risk acceptance)
- P-M4: VETOED (cascade)
- P-M6: VETOED (load-bearing, root cause)
- P-M3: VETOED (collateral cascade — depends on P-M2 functionality)
- P-M5: VETOED (collateral cascade — depends on P-M4 pool)
- P-M7: VETOED (depends on P-M4 pool reserves for price)
- P-M8: VETOED (depends on P-M4 + P-M6 view targets)
- P-M9: VETOED (end-to-end smoke uses entire vetoed stack)

All Stage C session #27 work is rolled back. P-M0 (commit `d33c25a` session #25; SHIPPED with prior architect §8) is **preserved**.

---

## §4. Rollback method

Per packet §6 + §10 "full revert" option:

```bash
# 11 reverts in reverse-chronological order (newest first):
git revert --no-commit a734cbe   # boot prompt (predicated on ship)
git revert --no-commit d15b868   # overall §8 packet (predicated on ship)
git revert --no-commit 17230ca   # P-M9 SHIPPED
git revert --no-commit 48675a4   # P-M8 SHIPPED
git revert --no-commit ba3a35d   # P-M7 SHIPPED
git revert --no-commit 5609760   # batch §8 packet (this VETO target's predecessor)
git revert --no-commit -m 1 7395aaa   # Merge P-M6 router
git revert --no-commit -m 1 6d3cb0c   # Merge P-M5 swap
git revert --no-commit -m 1 8c74034   # Merge P-M4 pool
git revert --no-commit a227189   # P-M3
git revert --no-commit -m 1 bac20ba   # Merge P-M2
git commit -m "Stage C FULL ROLLBACK — architect §8 VETO 2026-05-09 ..."
```

Result: HEAD points one commit past the rollback (a single composite revert commit); pre-rollback state preserved in history (no force-push, no reset).

---

## §5. Post-rollback state expectations

- Constitution gates: 175/0/1 (back to HEAD `b468140` baseline)
- Workspace tests: ~1308 passed (pre-Stage-C count)
- Trust Root verify: PASS
- FC1 chain_invariant: GREEN (Stage C atoms didn't touch externalized-attempt path; rollback removes the surface; invariant unchanged)
- Untouched: Stage A2/A3 substrate, Stage B substrate, P-M0 quarantine + CompleteSet hardening, all prior architect §8 ratifications

---

## §6. Lesson — why dual-audit-after-evidence is mandatory for Class-4

Stage C session #27 chose batch §8 over per-atom §8 to save ~3-4 weeks wall-clock. The trade-off:
- Saved: ~3-4 weeks IF clean
- Cost: 1 day session #27 implementation + 1 day session #28 audit + ~2-3 weeks rebuild = net **negative**

This validates:
- `feedback_dual_audit` Class-4 = full dual (Codex + Gemini, but Codex VETO already determines outcome)
- `feedback_audit_after_evidence` (audit AFTER tape exists, not before)
- CR-StageC-PM.16 charter rule (per-atom §8 cadence; no batching for Class-4)
- `feedback_no_workarounds_strict_constitution` (我不要凑活 — strict-equality not min())

The session #27 architect (= user) accepted the batch risk explicitly. Session #28 audit caught the result. The mechanism worked — just at higher cost than per-atom would have.

---

## §7. Attestation

This VETO is filed at architect verbatim instruction. Cited authority: user message 2026-05-09 session #28 verbatim "我是要 VETO + 全 rollback". Codex audit transcript referenced at agent ID `a1e5cd6edeb8377bc` for replay.

**Status**: 🔴 VETOED — rollback authorized; remediation directive companion file governs rebuild.
