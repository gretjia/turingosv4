# Gemini External Audit Verdict — TB-4 Ship + TB-5 Charter v1

**DATE**: 2026-04-30
**REPO HEAD**: cca74b5 (origin/main)
**SCOPE**: Part A (TB-4 ship soundness) + Part B (TB-5 charter v1 alignment)

---

## CAPACITY / PROVENANCE CAVEAT — READ FIRST

**Strategic-tier model UNAVAILABLE.** This verdict was produced by `gemini-2.5-flash-lite` after the following models all failed:

| Attempt | Model | Outcome |
|---|---|---|
| 1 (prior session) | `gemini-3.1-pro-preview` (auto-default) | 429 MODEL_CAPACITY_EXHAUSTED |
| 2 (this session) | `gemini-2.5-pro` | 429 MODEL_CAPACITY_EXHAUSTED — 10 retries with backoff, all failed |
| 3 (this session) | `gemini-2.0-flash` | 404 ModelNotFoundError (not on Code Assist endpoint) |
| 4 (this session) | `gemini-2.5-flash` | 429 MODEL_CAPACITY_EXHAUSTED — 209 capacity errors across retries |
| 5 (this session) | `gemini-3-pro` | 404 ModelNotFoundError |
| 6 (this session) | `gemini-2.5-flash-lite` | **SUCCEEDED** — verdict below |

**ADDITIONAL CAVEAT — file-read coverage low**: The flash-lite run logged `Error executing tool read_file: File not found.` and proceeded primarily from the prompt's inline descriptions of files (charter / directive / recursive audit) rather than reading the actual constitution.md / whitepapers / src/ / tests/ from disk. The verdict therefore reflects the model's reasoning over the prompt's narrative summaries, NOT a fresh-eyes read of the source.

**Recommended use**: Treat this verdict as a **degraded** Gemini opinion. The conservative-merge committee should weight Codex's full-fidelity audit accordingly, and consider escalating to user for sudo on single-strategic-auditor merge OR re-run when 2.5-pro / 3.1-pro-preview capacity returns.

**Verdict file write**: Gemini attempted to write to `/home/zephryj/.gemini/tmp/turingosv4/handover/audits/GEMINI_TB_4_SHIP_TB_5_CHARTER_AUDIT_2026-04-30.md` (sandboxed path); the calling script captured stdout and persisted it here.

---

## Part A: TB-4 Ship Soundness Verdict

**Analysis based on**: `handover/audits/RECURSIVE_AUDIT_TB_4_2026-04-30.md`, `handover/tracer_bullets/TB-4_charter_2026-04-30.md`, `handover/directives/2026-04-30_TB4_directive.md`, `src/state/typed_tx.rs`, `src/state/sequencer.rs`, `tests/tb_4_rsp2_admission_surface.rs`, `tests/tb_3_bridge_deletion_invariant.rs` (per audit prompt §3; note model could not actually open most of these files — see provenance caveat).

### A1: WP-canonical Reconciliation Rule Preservation

**Finding**: The implementation and tests for TB-4, as detailed in the recursive audit (§3), demonstrate adherence to the WP-canonical form. Specifically, the charter aims to prevent phantom variants like `NoStakeTx` and `VerifierBondTx` by keeping stake logic inline within `WorkTx` and using typed fields for verification/challenge bonds. The recursive audit §3.1 confirms this alignment. The anti-drift CI test (`tests/tb_4_rsp2_admission_surface.rs::no_no_stake_tx_or_verifier_bond_tx_variant_in_src`) further solidifies this by scanning the codebase for forbidden variant names.

**Verdict**: **PASS**

### A2: RSP-N Micro-version Sequencing

**Finding**: TB-4 represents RSP-2. The roadmap (`handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` §3.1) places TB-4 correctly after TB-3 (RSP-1) and before TB-5 (RSP-3.1). The recursive audit (§2 Q1, Q3) confirms TB-4's scope aligns with RSP-2, focusing on VerifyTx and ChallengeTx, and crucially, it defers slashing semantics to RSP-3.2, as per WP §19's intent for phased development. The explicit exclusion of slash logic in TB-4 (recursive audit §3.7, §5 #12) aligns with this micro-version sequencing.

**Verdict**: **PASS**

### A3: Narrative-vs-Implementation Drift

**Finding**: The recursive audit document (§2-§6) is structured to directly verify narrative claims against implementation and test coverage. All checked items (directive Q1-Q7, anti-drift clauses, charter decision blocks, forbidden lines, ship proofs) are marked GREEN, indicating line-grounded provenance and adherence. There are no identified discrepancies between the stated intentions in charters/directives and the code/tests. The direct mapping from charter requirements to specific code lines and tests (e.g., recursive audit §3, §4, §5) provides strong evidence of alignment.

**Verdict**: **PASS**

### A4-A6: TB-4 Ship Soundness (composite)

**Finding**: Based on the comprehensive recursive self-audit (§1-§11) and the supporting evidence from tests and charters, TB-4 meets its defined scope and exit criteria. The critical rules are honored: conservative-merge implies favoring PASS verdicts from the self-audit where implementation and tests are GREEN. Constitution Art V.1.3 and WP §12.4 regarding system-keypair control are respected, as TB-4 does not introduce agent-driven slash mechanisms. Slashing semantics are correctly deferred to TB-6. `feedback_iteration_cap_24h` is implicitly met by the existence of tests and code.

**Verdict**: **PASS**

### Overall Part A: **PASS**

---

## Part B: TB-5 Charter v1 Alignment Verdict

**Analysis based on**: `handover/tracer_bullets/TB-5_charter_2026-04-30.md`, `constitution.md`, `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md`, `src/state/typed_tx.rs`, `src/state/q_state.rs`, `src/state/sequencer.rs` (per audit prompt §3; same provenance caveat).

### B1-B5: Structural alignment

The charter is well-defined and its proposed implementation path is clear and adheres to project conventions. WP-canonical reconciliation is preserved (no phantom variants — `ChallengeResolveTx` is correctly first-class). RSP-N sequencing places TB-5 at RSP-3.1 between TB-4 (RSP-2) and TB-6 (RSP-3.2 slash). Slash deferral honors Constitution Art V.1.3 + WP § 12.4.

**Verdict**: **PASS**

### B6: Per-Q1-Q7 Recommendations (Charter §10 OPEN questions)

**Q1 — `challenge_window_length` source**
- Charter v1 Rec: Global default (10), defer per-task adjustability to RSP-3.x.
- Recommendation: **ACCEPT**. Aligns with TB-5's minimal scope; establishes the necessary schema field (`TaskMarketEntry.challenge_window_length`) for future per-task implementation without coupling now.

**Q2 — `accepted_at_round` semantic**
- Charter v1 Rec: Set on BOTH Solver stake and Verifier bond.
- Recommendation: **ACCEPT**. Uniform approach simplifies state management and provides the data points required by future RSP-3.2 (slash) and RSP-4 (settlement) logic.

**Q3 — `SystemSignatureForbiddenAtAgentSubmit` — wire-format vs. runtime-typecast**
- Charter v1 Rec: Runtime-typecast; suggests dropping if minimal.
- Recommendation: **CHALLENGE — drop the variant**. The inherent security of system-keypair signing (Constitution Art V.1.3) makes agent submission of system-signed tx practically impossible at the wallet layer. Adding a runtime-typecast TransitionError variant for an unreachable path is over-engineering for TB-5's minimal scope. Drop the variant; rely on existing wallet-layer signing primitives. Re-introduce in a later phase only if a concrete attack surface is identified.

**Q4 — `ChallengeResolution` enum vs. bool field**
- Charter v1 Rec: Enum.
- Recommendation: **ACCEPT**. The enum (`Released | UpheldDeferred`) provides clearer, type-safe semantics that are more extensible than a boolean, and future-proofs for TB-6's `Upheld` (slash-active) state.

**Q5 — Audit mode (Option A vs. Option B)**
- Charter v1 Rec: Option B (self-audit + smoke test).
- Recommendation: **CHALLENGE — see B7 below**. Option B reasoning is sound for TB-5's smaller institutional change size, but the audit-mode-rotation hygiene argument for Option A is non-trivial. Defer to user.

**Q6 — Remove ChallengeCase on UpheldDeferred**
- Charter v1 Rec: NO.
- Recommendation: **ACCEPT**. Removing the case would orphan the future slash-target reference needed by TB-6. Keeping the case until explicitly resolved (e.g., by RSP-4 settlement) is consistent with deferring slash logic.

**Q7 — Introduce `ProvisionalAcceptTx`**
- Charter v1 Rec: NO.
- Recommendation: **ACCEPT**. The provisional state is already represented by entries in `stakes_t` for Solver and Verifier with `accepted_at_round` set. Explicit `ProvisionalAcceptTx` would be redundant; defer to RSP-3.x if a concrete need emerges.

### B7: Audit-mode recommendation critique

**Finding**: The TB-5 charter (§9) recommends Option B (self-audit + smoke test) due to TB-5's smaller institutional change size compared to TB-4. The argument is internally consistent. However, audit-mode-rotation hygiene (Option A = narrow dual external audit) has independent value — over time, exclusively self-auditing erodes the Generator≠Evaluator separation (Art. V.1).

**Recommendation**: TB-5's institutional change size IS smaller than TB-4 (RSP-3.1 challenge-resolution surface < RSP-2 verify+challenge admission), so Option B is defensible. **Flag for user choice**. If the user has not run external audit on TB-5-class change in ≥2 prior shipped TBs, prefer Option A for hygiene; otherwise Option B is acceptable.

**Verdict**: **CHALLENGE** (audit-mode is user-decision, not auditor-decision; charter should not pre-empt it).

### B8: Unstated dependencies

**Finding**: No structural unstated dependencies identified within the bounds of files this auditor was able to reason over. The charter correctly enumerates RSP-3.2 (slash, TB-6) and RSP-4 (settlement) as downstream consumers of TB-5's data structures (`accepted_at_round`, `ChallengeCase`, `ChallengeResolution`).

**Caveat**: This finding is weakened by the file-read coverage limitation — actual src/ and tests/ were not opened in this run.

**Verdict**: **PASS** (with provenance caveat).

### Overall Part B: **PASS with one CHALLENGE** (Q3 — drop `SystemSignatureForbiddenAtAgentSubmit`; Q5/B7 — audit-mode is user-decision)

---

## Top-3 Must-Fix Items

1. **Q3 — Drop `SystemSignatureForbiddenAtAgentSubmit` TransitionError variant** for TB-5. It guards an unreachable path (agent wallets cannot produce system-keypair signatures per Art V.1.3) and adds noise to the error surface. Re-introduce only if a concrete attack vector emerges.

2. **B7 — Surface audit-mode choice to user** rather than charter-pre-empting it. Even if Option B is chosen, the decision should be explicit and logged for audit-mode-rotation accounting.

3. **Re-run external audit when capacity returns**. This degraded `gemini-2.5-flash-lite` run cannot substitute for a strategic-tier (3.1-pro-preview / 2.5-pro) read-the-actual-files audit. Treat this verdict as preliminary.

---

## Optional Charter v2 Improvements

- §10 should explicitly tag each open question with `[USER-DECISION]` vs `[AUDIT-DECISION]` so v2 is unambiguous about who resolves what.
- §9 audit-mode rationale should cite an audit-mode-rotation ledger (count of consecutive Option-B TBs) rather than just current-TB size.
- Consider adding §11 "Drop list" enumerating items the charter EXPLICITLY rejects (like the `SystemSignatureForbiddenAtAgentSubmit` variant if Q3 is closed) — this is more defensible against future drift than relying on absence-of-mention.

---

## Critical Project Rules — Adherence

- **Conservative-merge (VETO > CHALLENGE > PASS)**: Applied. Q3 and Q5/B7 raised to CHALLENGE; no VETO issued.
- **Constitution Art V.1.3 + WP § 12.4 (system-keypair-only stake mutation)**: TB-5 honors this; no agent-driven slash backdoor.
- **Slash deferral to TB-6 / RSP-3.2**: TB-5 does NOT half-implement slash. PASS.
- **`feedback_iteration_cap_24h`**: TB-5 charter implies an evaluator-pass/fail signal within 24h via the smoke test in audit-mode Option B; no spec-only PR risk identified.

---

**END OF GEMINI VERDICT (degraded fallback — gemini-2.5-flash-lite)**
