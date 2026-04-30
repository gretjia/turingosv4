# Dual Audit Round 1 — Merged Verdict — TB-4 Ship + TB-5 Charter v1

**Date**: 2026-04-30
**Audit subjects**: TB-4 ship soundness (HEAD `1b60237`) + TB-5 charter v1 plan (commit `1b60237`)
**Audit prompt** (binding scope): `handover/audits/EXTERNAL_AUDIT_PROMPT_TB_4_SHIP_TB_5_CHARTER_2026-04-30.md`
**Authority for merge**: `feedback_dual_audit_conflict` — VETO > CHALLENGE > PASS conservative-merge
**Architect ruling on outcome**: `handover/directives/2026-04-30_TB5_VETO_redesign_directive.md` (2026-04-30 ultrathink directive)

---

## 1. Round 1 audit status (per-auditor)

| Auditor | Tier | Status | Verdict file |
|---|---|---|---|
| **Codex** (implementer-paranoid) | full-fidelity (codex-cli 0.125.0; advanced runtime) | COMPLETED + ran cargo + sha256 + Lean re-verify | `handover/audits/CODEX_TB_4_SHIP_TB_5_CHARTER_AUDIT_2026-04-30.md` |
| **Gemini** (strategic / WP-canonical) | **DEGRADED** (`gemini-2.5-flash-lite` fallback after `gemini-2.5-pro` / `2.5-flash` / `3.1-pro-preview` returned 429 MODEL_CAPACITY_EXHAUSTED + low file-read coverage logged) | COMPLETED with caveat header | `handover/audits/GEMINI_TB_4_SHIP_TB_5_CHARTER_AUDIT_2026-04-30.md` |

**Round 1 status**: COMPLETED-WITH-CAVEAT. Gemini hit a sandbox-write quirk + capacity-exhausted strategic-tier; degraded fallback model produced a verdict that did NOT cross-validate Codex's substantive finding because the model did not successfully read source files (logged `Error executing tool read_file: File not found.`) and reasoned primarily from prompt's inline narrative summaries. Per architect directive § 4 Q5 + Q6, **Codex full-fidelity verdict controls**; Gemini degraded does NOT and CANNOT override Codex's VETO.

---

## 2. Per-auditor headline verdicts

### 2.1 Codex (full-fidelity)

```
Part A overall: CHALLENGE   (line-grounding mismatch in TB-4 charter; doc-only patch)
Part B overall: VETO        (structural system-tx ingress hole; charter v2 redesign required)
```

Verification anchors run by Codex (all GREEN locally):
- `cargo test --workspace` → PASS=571 / FAIL=0 ✓
- Trust Root manifest sha256 match (sequencer.rs / typed_tx.rs / q_state.rs all match `genesis_payload.toml:227-229`) ✓
- `grep` zero hits on `NoStakeTx` / `VerifierBondTx` / `ChallengeStakeTx` / `VerifierStakeTx` ✓
- TB-3 bridge invariant test (2 PASS / 0 FAIL) ✓
- 9-sub-field invariant + 5-holding CTF + I44 anti-drift CI all green ✓
- Lean re-verify: `proof_mathd_algebra_125.lean` typechecks (1 unused-var warning; non-blocking) ✓

### 2.2 Gemini (degraded-tier)

```
Part A overall: PASS        (A1-A6 all PASS)
Part B overall: PASS        (with Q3 + Q5/B7 minor CHALLENGE)
```

**Caveat (logged in verdict header)**: model `gemini-2.5-flash-lite`; fresh-eyes file reads largely failed; reasoning primarily from prompt-inline summaries. Did NOT read TB-5 charter v1 § 3.5 dispatch pseudocode in source-grounded fashion; consequently did NOT catch the B2 system-signature gap.

### 2.3 Cross-validation analysis

The two auditors **diverge on Part B** because of audit-coverage asymmetry, NOT analytical disagreement:

- Codex line-grounded against `src/state/sequencer.rs:863` (`Sequencer::submit` accepts bare `TypedTx` from any caller) AND `src/state/sequencer.rs:262` (`dispatch_transition` has no keypair / pinned-pubkey argument). Codex confirmed the gap exists in production code.
- Gemini did not actually read `sequencer.rs`. Gemini's PASS on B2 is uninformed; it is consistent with charter v1's English statement "system-emitted" without verifying that statement against live code.

The Codex finding **cannot be analytically refuted by an auditor that did not read the source**. Per `feedback_dual_audit_conflict`, conservative-merge applies.

---

## 3. Merge per-question (VETO > CHALLENGE > PASS)

### 3.1 Part A — TB-4 ship soundness

| Q | Codex | Gemini | **Merged** |
|---|---|---|---|
| A1 (charter §3 ten-block line-grounding) | CHALLENGE [doc: `q.logical_t` vs `q.q_t.current_round`] | PASS | **CHALLENGE** (Codex finding stands; doc-only patch required) |
| A2 (forbidden-list enforcement) | PASS | PASS | **PASS** |
| A3 (three ship proofs green) | PASS | PASS | **PASS** |
| A4 (directive Q1-Q7 + 5 anti-drift) | PASS | PASS | **PASS** |
| A5 (WP-canonical reconciliation; I44 anti-drift CI) | PASS | PASS | **PASS** |
| A6 (silent regressions vs TB-3) | PASS | PASS | **PASS** |
| **Part A merged** | — | — | **CHALLENGE** (A1 only; doc patch resolves) |

### 3.2 Part B — TB-5 charter v1 alignment

| Q | Codex | Gemini (degraded) | **Merged** |
|---|---|---|---|
| B1 (ChallengeResolveTx first-class allowed-named) | PASS | PASS | **PASS** |
| **B2 (system-emitted authority enforceable)** | **VETO** [structural; live dispatch lacks system_signature verification; agent forgery possible] | PASS (uninformed; did not read sequencer.rs) | **VETO** (Codex full-fidelity; Gemini did not cross-validate) |
| B3 (entry-shape additives 9-sub-field-safe) | PASS | PASS | **PASS** |
| B4 (Released vs UpheldDeferred atomicity) | PASS [conditional on B2 fix] | PASS | **PASS** (conditional on B2 fix) |
| B5 (28 forbidden lines RSP-3.1 boundary) | CHALLENGE [boundary right; #26 depends on B2 redesign] | n/a (lower granularity from Gemini) | **CHALLENGE** (correlated with B2 VETO; remediation in v2) |
| B6.Q1 (`challenge_window_length` source) | PASS (global default 10) | PASS (concur) | **PASS** |
| B6.Q2 (`accepted_at_round` placement) | PASS (BOTH Solver + Verifier) | PASS (concur) | **DIVERGENT** — architect directive § 7 Q2 **OVERRIDES**: defer accepted_at_round entirely (don't pollute TB-5 schema for TB-6 use) |
| B6.Q3 (`SystemSignatureForbiddenAtAgentSubmit` keep) | CHALLENGE [drop or redesign with real ingress split] | CHALLENGE (over-engineering) | **CHALLENGE** — architect directive § 7 Q3: **rename + keep** (`SystemTxForbiddenOnAgentIngress` as TB-5.0 core error) |
| B6.Q4 (enum vs bool) | PASS (enum) | PASS (concur) | **PASS** |
| B6.Q5 (audit mode A vs B) | CHALLENGE [Option A required] | CHALLENGE (user-decision) | **Option A** (architect directive § 4 Q4) |
| B6.Q6 (UpheldDeferred remove ChallengeCase) | PASS (do not remove) | PASS (concur) | **PASS** (architect directive § 7 Q6 reinforces with status-field option) |
| B6.Q7 (add ProvisionalAcceptTx) | PASS (do not add) | PASS (concur) | **PASS** |
| **B7 (audit-mode override)** | CHALLENGE [Option A] | CHALLENGE | **Option A** (architect directive § 4 Q4) |
| **B8 (unstated dependencies)** | **VETO** [system-tx authorization substrate missing] | n/a | **VETO** (consistent with B2; required redesign) |
| **Part B merged** | — | — | **VETO** (B2 + B8; charter v2 redesign required) |

---

## 4. Merged round 1 verdict

```
PART A: CHALLENGE (doc-only patch resolves; remediation = A1 below)
PART B: VETO     (structural redesign required; remediation = TB-5 charter v2)
```

**Round 2 dual audit**: NOT scheduled per architect directive § 4 Q5 + Q6 ("先动 charter v2 redesign，不等 Gemini" + "选 (a) — 接受 Codex full-fidelity + Gemini degraded 为 round 1 记录"). Round 2 dual audit is mandatory at TB-5 v2 STEP_B Phase-0 (per architect directive § 4 Q4 = Option A).

---

## 5. Top-3 must-fix items (architect directive § 9 binding execute order)

### Must-fix #1 — TB-5 charter v2 redesign (B2 + B8 VETO remediation)

Per architect directive § 4 Q2 + § 5 + § 6 + § 7 + § 9 + § 10:

```text
TB-5 v2 = "System-Emitted Resolution Gate + Challenge Bond Release"
  TB-5.0 — System Ingress Barrier
    submit_agent_tx(...) — agent variants only
    emit_system_tx(...)  — system-emitted variants only
    dispatch verifies system_signature (or internal capability via emit_system_tx)
  TB-5.1 — Released-only Challenge Bond Release
    Released:        challenge_cases_t.bond -> balances_t[challenger]; CTF round-trip
    UpheldDeferred:  marker only; ChallengeCase preserved for TB-6 slash
  TB-5.2 — Replay + anti-drift
    + agent_submit_rejects_system_variants CI
    + 6 forbidden-variant additions to TB-4 I44 list

Audit mode: Option A (dual external audit) at STEP_B Phase-0 + ship.
Forbidden boundary: NO slash / NO settlement / NO reputation / NO ProvisionalAccept / NO FinalizeReward / NO window-closure logic / NO automatic round-tick.
4 anti-drift renames (binding):
  resolve ≠ judge
  release ≠ settlement
  UpheldDeferred ≠ slash
  system_signature ≠ schema-only field (must be live-verified)
```

### Must-fix #2 — Audit-mode for TB-5 v2 ship: Option A (dual external)

Per architect directive § 4 Q4: TB-3 / TB-4's Option B (self-audit + 真实烟测) cannot continue normal-mode for TB-5 because TB-5 introduces a system-emitted economic mutator. Restore: STEP_B Phase-0 dual external audit + pre-ship implementation audit. If Gemini capacity remains constrained, allow degraded-tier with explicit verdict-header caveat (NOT to substitute strategic audit).

### Must-fix #3 — A1 doc patch: TB-4 charter `q.logical_t` → `q.q_t.current_round`

Per architect directive § 4 A1 authorization + § 9 step 10: separate commit. TB-4 charter v2 wording at `handover/tracer_bullets/TB-4_charter_2026-04-30.md` line 148 + line 195 (and any derived audit prose) updated to match shipped code at `src/state/sequencer.rs:480`. **No source change**. Lands as standalone commit (NOT mixed with TB-5 v2 redesign).

---

## 6. Optional improvements (non-blockers; architect-directive-listed)

- Lean proof artifact `proof_mathd_algebra_125.lean` line 18 unused-`h₀` warning — Codex flagged. Per architect directive § 8 not blocking; acceptable to fix opportunistically OR update reproducibility standard to "typecheck no errors; warnings non-blocking."
- `accepted_at_round` deferred from TB-5 entirely (architect directive § 7 Q2). If TB-5.1 tests need a "round" anchor, reuse existing `ChallengeCase.opened_at_round` (TB-4); TB-6 / RSP-3.2 introduces `accepted_at_round` when window-closure math is actually installed.
- Future: TB-5A or TB-6-pre — `AgentRegistry + Live Agent Signature Verification` (per architect directive § 4 Q3). Documented debt; NOT in TB-5 v2 scope.

---

## 7. Decision record

**Round 1 dual audit status**: COMPLETED-WITH-CAVEAT (Codex full-fidelity + Gemini degraded).

**Architect ruling**: ACCEPT Codex VETO; PROCEED to TB-5 charter v2 redesign without waiting for Gemini round 2; Option A audit mode reinstated for TB-5 v2 ship.

**TB-5 ship gate (when v2 charter user-reviewed and STEP_B Phase-0 launched)**:
1. Round-2 dual external audit (Codex full-fidelity + Gemini at strategic tier when capacity returns; degraded acceptable with caveat).
2. Conservative-merge per `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS).
3. Pre-ship implementation audit (Codex + Gemini narrowed scope on diff).

**This verdict file is canonical**. Subsequent decisions (charter v2 review, STEP_B Phase-0 verdict, ship verdict) reference back to this file as round 1 anchor.

---

## 8. Cross-references

- Codex round 1 verdict: `handover/audits/CODEX_TB_4_SHIP_TB_5_CHARTER_AUDIT_2026-04-30.md`
- Gemini round 1 (degraded) verdict: `handover/audits/GEMINI_TB_4_SHIP_TB_5_CHARTER_AUDIT_2026-04-30.md`
- Audit prompt: `handover/audits/EXTERNAL_AUDIT_PROMPT_TB_4_SHIP_TB_5_CHARTER_2026-04-30.md`
- Architect directive (binding TB-5 v2 redesign): `handover/directives/2026-04-30_TB5_VETO_redesign_directive.md`
- TB-4 ship directive (predecessor): `handover/directives/2026-04-30_TB4_directive.md`
- TB-5 charter v1 (SUPERSEDED-by-VETO): `handover/tracer_bullets/TB-5_charter_2026-04-30.md` @ commit `1b60237`
- TB-5 charter v2 (binding incarnation): same path post-redesign commit
- Memory rule: `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS conservative-merge)
- Memory rule: `feedback_session_label_codification` (codify directive in committed handover doc — this verdict file does that for round 1)
