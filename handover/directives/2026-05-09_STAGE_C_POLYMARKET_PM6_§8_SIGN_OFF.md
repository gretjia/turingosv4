# Stage C P-M6 (BuyWithCoinRouter rebuild) — §8 Sign-Off (2026-05-09 session #32)

**Status**: SHIPPED.
**HEAD on `origin/main`** (post-merge): TBD upon merge commit (will be referenced in LATEST.md).
**Branch trail**: `feat/p-m6-rebuild` (post-PRE-§8-audit cleanup commit) → `--no-ff` merge to `main`.
**Pre-§8 dual audit (PRE-§8 timing rule, E.5)**:
- Codex G2 R1: **PASS** (9/9; conviction high; PROCEED) — `handover/audits/CODEX_STAGE_C_PM6_AUDIT_2026-05-09_R1.md`.
- Gemini DeepThink R1: **PASS** (9/9; conviction high; PROCEED) — `handover/audits/GEMINI_STAGE_C_PM6_AUDIT_2026-05-09_R1.md`.
- Aggregate: **PASS** (conservative-merge VETO > CHALLENGE > PASS = PASS).
- Round cap 2 used 1.

---

## §1. Architect §8 verbatim sign-off

**User authorization at session #32 boot (2026-05-09)** — multi-clause Class-4 forward §8 grant per CLAUDE.md §10 multi-clause analysis:

> 给你授权自主执行直到polymarket全部落地并自主开展真题测试，给你授权调用LMM API，再次给你对齐文件，遇到问题，严格对齐，遇到边缘问题去对齐宪法

**Multi-clause structural analysis** (CLAUDE.md §10):

| Clause | Named act | Scope | Type |
|--------|-----------|-------|------|
| 1 | `授权` (authorize) + `自主执行` (autonomous execution) | `直到polymarket全部落地` (until Polymarket fully lands) | Forward Class-4 grant |
| 2 | `授权调用 LMM API` (authorize LLM API calls) | Real-problem testing | Operational grant |
| 3 | `再次给你对齐文件` (re-align with file) | architect manual §1-§9 | Spec re-binding |
| 4 | `严格对齐` (strict alignment) + `对齐宪法` (align to constitution) | Edge cases | Constitution discipline |

This is structurally equivalent to the prior P-M4 sign-off "签字，同意后续执行" + Stage A3 "同意 sign-off" multi-clause Class-4 §8 forms, but with broader scope (full Polymarket sequence vs single atom). Per CLAUDE.md §10:

> A one-word instruction may authorize candidate remediation only.
> It does not authorize final ratification or ship.

The user's instruction is multi-clause; named acts (`授权`/`授权`/`再次给你对齐文件`/`严格对齐`); explicit scope (`直到polymarket全部落地`); explicit constitutional discipline carve-out (`遇到边缘问题去对齐宪法`). This satisfies the §10 multi-clause Class-4 §8 form.

**Critical conditional**: forward batch authorization is **conditional on dual audit PASS** for each Class-4 atom in the sequence. Per `feedback_dual_audit` Class-4 timing rule + `feedback_dual_audit_conflict` conservative-wins ordering: if Codex G2 OR Gemini returns CHALLENGE/VETO, the user pre-authorization does NOT cover post-defect ship; standard remediation cycle applies.

**P-M6 dual audit verdict**: both **PASS** R1 first-try (9/9 each, conviction high, PROCEED). Authorization condition satisfied.

**P-M6 §8 ratification**: ACTIVE per multi-clause forward grant + R1 dual audit PASS.

---

## §2. PRE-§8 dual audit verdicts (R1; both PASS first-try)

| Auditor | Verdict | Conviction | Recommendation | Transcript |
|---------|---------|------------|----------------|------------|
| **Codex G2** | **PASS** (9/9) | high | PROCEED | `handover/audits/CODEX_STAGE_C_PM6_AUDIT_2026-05-09_R1.md` |
| **Gemini** | **PASS** (9/9) | high | PROCEED | `handover/audits/GEMINI_STAGE_C_PM6_AUDIT_2026-05-09_R1.md` |
| **Aggregate** | **PASS** | high | PROCEED | conservative-merge |

**Codex Q1-Q9 summary** (verbatim from R1 transcript):
- Q1 PASS — 9 architect-numbered steps present with no extra economic mutation; injection check before each step.
- Q2 PASS — `assert_complete_set_balanced` strict symmetric equality (Defect-1 patch).
- Q3 PASS — cfg(debug_assertions) gating correct; tests exhaustive; non-blocking note: stale `cfg(test)` comments (FIXED in follow-up commit; STEP_B files re-rehashed).
- Q4 PASS — u128 integer math; no f64/f32; k_post >= k_pre asserted.
- Q5 PASS — 8-wire-field tx; no timestamp_logical; event_id (not event_id_kind); E.1 Landed.
- Q6 PASS — F-DEFERRAL-2 closed via SigningPayload sibling binding.
- Q7 PASS — total_supply_micro conservation; symmetric Coin → collateral movement.
- Q8 PASS — replay-determinism preserved; release builds compile env hook to no-op.
- Q9 PASS — pool drain mathematically impossible; slippage acceptable; dust intentional; cfg(debug_assertions) sufficient.

**Gemini Q1-Q9 summary** (verbatim from R1 transcript):
- All 9 PASS; conviction high; recommendation PROCEED.
- Q1: 9 architect-numbered steps; bookkeeping steps 5+6+7 combined as sound interpretation.
- Q3: Defect-1 patch correct; symmetric branch + strict equality.
- Q4: Defect-2 patch robust; cfg(debug_assertions) gate; assertions exhaustive.
- Q9: pool drain mathematically impossible; slippage griefing acceptable v4 risk; dust accumulation intentional.

---

## §3. Defect closure summary (session #27 VETO targets)

| Defect | Target | Phase E mechanism | Phase F.5 closure |
|--------|--------|-------------------|-------------------|
| 1 | P-M6 monetary invariant `min(sum_yes, sum_no) == collateral` | E.3 strict-equality refactor (`assert_complete_set_balanced` symmetric/asymmetric split) | Router accept arm calls strict-symmetric path; tests 4 + 8 directly assert it; Codex Q2 + Gemini Q3 PASS |
| 2 | P-M6 vacuous `router_atomic_rollback_on_failure` | E.2 static-layer pattern catalog | cfg(debug_assertions) injection hook + dynamic-layer test 9 + defense-in-depth across all 9 steps; Codex Q3 + Gemini Q4 PASS |
| 3 | P-M2 `timestamp_logical` drift | E.1 verbatim struct binding gate | P-M6 follows minimal-shape pattern (NO timestamp_logical); E.1 P-M6 binding LANDED; Codex Q5 + Gemini Q5 PASS |
| 4 | P-M4 `event_id_kind` rename | E.1 verbatim struct binding gate | P-M6 uses `event_id`; E.1 P-M6 binding LANDED; Codex Q5 + Gemini Q5 PASS |

**All 4 session #27 VETO defects mechanically prevented from recurrence in future Class-4 atoms** by the Phase E + Phase F.5 substrate now in place.

---

## §4. Validation post-ship (HEAD `0e0df18` + comment-fix follow-up)

| Check | Pre-P-M6 (P-M5 baseline) | Post-P-M6 | Δ |
|-------|--------------------------|-----------|---|
| Constitution gates | 213/0/1 | **223/0/1** | +10 |
| Workspace tests | 1346/0/151 | **1356/0/151** | +10 |
| Trust Root verify | PASS | **PASS** | rehashed 6 STEP_B files (post follow-up: typed_tx + sequencer re-rehashed for comment fix) |
| `cargo check --workspace` | clean | **clean** | warnings-only baseline |

**E.1 P-M6 bindings**: BuyWithCoinRouterTx + BuyWithCoinRouterSigningPayload BOTH `LandingStatus::Landed`.
**E.2 P-M6 binding**: `BuyWithCoinRouter` `LandingStatus::Landed`; rollback test invokes `set_var("TURINGOS_TEST_ROUTER_FAIL_AT_STEP", ...)` per pattern catalog.
**F-DEFERRAL-1**: closed via vacuous attestation (no helper-alias introduced).
**F-DEFERRAL-2**: closed for P-M6 via sibling SigningPayload binding.

---

## §5. Phase F forward path (after P-M6 §8 SHIPPED)

Per remediation directive §1.B + §1.C + user pre-authorization scope `直到polymarket全部落地`:

| Phase | Atom | Class | §8 required | Audit required |
|-------|------|-------|-------------|----------------|
| F.6 | P-M7 PriceIndex (architect §7.8 view-only quote) | 1-2 | NO | self-audit |
| F.7 | P-M8 Audit views (architect §7.10 audit_tape view-shares/pools/prices/positions) | 1-2 | NO | self-audit |
| F.8 | P-M9 Controlled market smoke | 2-3 | NO (Class-3 framing per remediation directive) | self-audit |
| F.9 | Stage C overall §8 | 4 | YES | PRE-§8 dual audit dispatch (Codex G2 + Gemini) |

User pre-authorization covers F.6-F.8 + F.9 atomically per `直到polymarket全部落地` scope; F.9 still requires PRE-§8 dual audit per E.5 timing rule.

---

**End of P-M6 §8 sign-off (SHIPPED).**
