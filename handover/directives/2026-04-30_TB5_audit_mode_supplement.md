# Architect Directive Supplement — TB-5 Audit Mode (Codex-Only Authorization)

**Received**: 2026-04-30 (post round-1 dual-audit closure; post charter v2 commit `0b76307`).
**Mode**: chat directive supplement from user `gretjia`.
**Status**: ARCHIVED. **Supplements** `handover/directives/2026-04-30_TB5_VETO_redesign_directive.md` § 4 Q4 + § 11.4.
**Impact category**: operational policy modification (does NOT alter the substantive ruling on TB-5 v2 redesign; ONLY adjusts the audit-execution mechanism).

---

## 1. Verbatim ruling

> "不用等gemini，使用codex就可以"
>
> (Don't wait for Gemini; using Codex is sufficient.)

## 2. Context

Round-1 dual audit (per `handover/audits/DUAL_AUDIT_TB_4_SHIP_TB_5_CHARTER_VERDICT_2026-04-30.md`) was COMPLETED-WITH-CAVEAT:
- Codex full-fidelity verdict landed (Part B VETO; substantively correct).
- Gemini fell to **degraded tier** (`gemini-2.5-flash-lite` after `gemini-2.5-pro` / `gemini-2.5-flash` / `gemini-3.1-pro-preview` returned 429 MODEL_CAPACITY_EXHAUSTED). Degraded Gemini did NOT cross-validate the Codex VETO because file reads largely failed (logged `read_file: File not found`).

Per parent directive `2026-04-30_TB5_VETO_redesign_directive.md` § 4 Q4 + § 11.4, TB-5 v2 ship gate was set to **Option A (dual external audit)**, with explicit warning: "**不要把 degraded Gemini 当作完整战略审计**."

This supplement modifies that policy for the upcoming TB-5 v2 STEP_B Phase-0 audit round in light of Gemini's continued capacity unavailability.

## 3. Modified policy

```text
TB-5 v2 STEP_B Phase-0 audit  →  Codex-only round 2 (single-auditor).
TB-5 v2 ship audit            →  Codex-only (single-auditor) by default.

Gemini may be re-attempted opportunistically when strategic-tier capacity
returns. If Gemini at strategic-tier (gemini-2.5-pro or stronger) becomes
available before TB-5 v2 ship, attach as a supplemental verdict — but
Codex single-auditor verdict remains the ship-gate authority.

Degraded Gemini (gemini-2.5-flash-lite or weaker) is NOT a substitute and
MUST NOT be invoked as a stand-in for strategic-tier audit. Its absence
is preferable to its mis-attribution.
```

## 4. Reasoning (parent-directive override justification)

The parent directive § 4 Q4 explicitly anticipated this scenario:

> "如果 Gemini 继续 capacity 不可用，可以降级，但要在 verdict 文件中明确标记: Gemini degraded / not strategic-tier; Codex full-fidelity VETO controls. 不要把 degraded Gemini 当作完整战略审计."

The user's supplement extends this from "降级可接受 with caveat" to "Codex-only acceptable" — operationally equivalent under the constraint that strategic-tier Gemini remains unavailable. The substantive principle (Codex full-fidelity is the controlling verdict; degraded auditors don't pad the merge) is preserved.

Conservative-merge per `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS) still applies. Single-auditor Codex verdict simply means there's nothing to merge — Codex's verdict IS the merged verdict, with explicit caveat that no strategic-tier cross-validation occurred.

## 5. Constraints that REMAIN binding (NOT modified by this supplement)

- TB-5 v2 redesign substance per `2026-04-30_TB5_VETO_redesign_directive.md` § 4 Q1 + § 5 + § 6 + § 7 + § 11.
- Two-channel ingress architecture (Option 1: `submit_agent_tx` + `emit_system_tx`).
- 4 anti-drift renames (resolve ≠ judge / release ≠ settlement / UpheldDeferred ≠ slash / system_signature ≠ schema-only).
- Sub-atom split TB-5.0 + TB-5.1 + TB-5.2.
- 34 forbidden lines (charter v2 § 6).
- system_signature live-verification at dispatch entry (defense in depth) OR internal-only construction via `emit_system_tx`.
- ChallengeCase.status additive field (Open | Released | UpheldDeferred); Released zeros bond + flips status (no removal).
- Slashing remains TB-6 / RSP-3.2 territory.

## 6. Verdict-file caveat header (binding template)

Every Codex-only TB-5 verdict file MUST include this caveat header:

```text
**Audit Mode**: Codex-only (single-auditor) per directive supplement
2026-04-30 (handover/directives/2026-04-30_TB5_audit_mode_supplement.md).
Gemini strategic-tier (gemini-2.5-pro or stronger) was unavailable at
audit time due to MODEL_CAPACITY_EXHAUSTED 429 errors on the
cloudcode-pa.googleapis.com endpoint. Degraded Gemini was deliberately
NOT invoked as substitute per parent directive § 4 Q4 ("不要把 degraded
Gemini 当作完整战略审计").

This Codex verdict IS the ship-gate authority for the audited subject.
There is no second strategic auditor to merge against. Per
`feedback_dual_audit_conflict` conservative-merge, single Codex VETO /
CHALLENGE / PASS controls. If strategic-tier Gemini becomes available
post-ship, an opportunistic supplemental verdict may be appended but
does NOT override this round's verdict.
```

## 7. Cross-references

- Parent directive (substantive TB-5 v2 redesign authority): `handover/directives/2026-04-30_TB5_VETO_redesign_directive.md`
- Round-1 merged verdict (records Gemini degraded state): `handover/audits/DUAL_AUDIT_TB_4_SHIP_TB_5_CHARTER_VERDICT_2026-04-30.md`
- Charter v2 (binding incarnation; audit subject): `handover/tracer_bullets/TB-5_charter_2026-04-30.md`
- Memory rule: `feedback_dual_audit` (default-dual; this supplement modifies for TB-5 only)
- Memory rule: `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS; applies trivially to single-auditor case)
- Memory rule: `feedback_session_label_codification` (codify policy modifications in committed handover doc — this supplement does that)
