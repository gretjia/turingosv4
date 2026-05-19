# F7 — S2 Falsifiability Experiment: Prompt-Only Fix for Grill LLM Defects

**Date**: 2026-05-18
**Atom class**: Class 1 (prompt + docs only; no Rust source change)
**Branch posture**: read-only git
**Wire contract preserved**: schema (TurnPayload), 8 canonical slots, 15-turn ceiling, JSON envelope, confidence/done semantics, playback requirement at done=true

---

## 1. Hypothesis (S2 falsifiability claim)

> **S2**: A prompt-only edit to `assets/prompts/grill_meta_v1.md` can fix the observed Wave 1 Mrs Chen defects — *slot-extraction-too-conservative* and *Voss-mirror restatement loop* — without any Rust code change.

If S2 holds, this is a STRONG Software 3.0 validation point at TuringOS scale: "the program IS the prompt" is operationally true at the surface where LLM behavior changes user-visible outcomes. Prompt revision IS software development.

If S2 falsifies, the failure surfaces a Software-3.0-paradigm gap that demands Rust kernel intervention — either because the prompt language is too weak to express the constraint, or because the LLM ignores the prompt in ways only deterministic gating can correct.

The experiment is performed against **one** persona (Mrs Chen) for the falsifiability witness, with cross-validation against Wave 1 P1/P4 as a regression check.

---

## 2. Observed defect (the bug we are trying to fix)

Source evidence: `handover/evidence/phase6_3_x_universality_1779111375/wave1/mrs_chen/session_log.jsonl` (9 events: 1 bootstrap + 7 answer turns; backend timeouts at turns 2/6/7 are F6 backend territory, NOT this atom).

Across the 5 LLM-successful turns the LLM:

| Turn | User answer (semantic content) | Slots semantically covered | LLM extracted `covered_slots` |
|------|--------------------------------|----------------------------|-------------------------------|
| 1→2  | tetris-like, son after school, WiFi unstable | job, anchor, robustness | `["job"]` |
| 3    | son plays 30min after school, no supervision | job (reinforce), scope, acceptance | `["job"]` |
| 4    | single-player, no online battle | scope, robustness | `["job"]` |
| 5    | (duplicate of turn 4) | (no new info) | `["job"]` |

Symptoms:

- **D-LLM-1 — Conservative extraction**: LLM extracts only `job`, even when the user clearly covers anchor, robustness, scope, acceptance. Confidence stuck at 0.1–0.2 through turn 5 despite ample raw signal.
- **D-LLM-2 — Voss-mirror loop**: turns 2, 3, 4, 5 ALL open with "听起来你是说…" trying to re-confirm the SAME `job` content rather than probing new slots. The Voss instruction is being applied unconditionally per turn.
- **D-LLM-3 — Single-slot-per-turn bias**: even when one user answer semantically covers 3 slots (turn 3), the LLM only credits 1.

These three are interrelated symptoms of the same root: the v1 prompt does not tell the LLM **how aggressive** extraction should be, **when** to Voss-mirror, or that **one answer can cover many slots**.

---

## 3. Diagnosis at the prompt level

Reading `assets/prompts/grill_meta_v1.md` line by line:

### 3.1 What v1 says

- Line 16 (methodology): `Mirror playback (Voss labeling): user confirms or corrects` — listed as a methodology element with no frequency guidance.
- Line 23 (constraints): `Build on the LATEST answer. Mirror back ("听起来你是说X，对吗？" / "If I heard right, you mean X — correct?").` — this is a directive to mirror back. Stated as if it applies **every** turn. No exception clause.
- Line 56 (semantics): `covered_slots: cumulative set of slot ids whose information you have extracted. Monotonic (only grows).` — defines monotonicity but says nothing about the extraction threshold. The LLM defaults to "I extracted it only if the user explicitly confirmed it under that slot's question."
- Nothing in v1 says one answer can address multiple slots.
- Nothing in v1 says "do not re-ask a covered slot."

### 3.2 Failure-mode → v1 line mapping

| Symptom | v1 root cause |
|---------|---------------|
| D-LLM-1 (conservative extraction) | Line 56 only says "monotonic". No threshold guidance. LLM defaults to high-confidence "explicit verbatim" extraction. |
| D-LLM-2 (Voss-mirror loop) | Line 23 mandates "Mirror back" with no frequency cap. Methodology line 16 reinforces it as one of the 8 elements. LLM interprets as "every turn". |
| D-LLM-3 (single-slot-per-turn) | Nothing in v1 explicitly licenses multi-slot extraction. The methodology bullets read as 8 items the LLM must cover, suggesting a 1:1 question:slot mapping. |

---

## 4. Proposed prompt changes (v1 → v2 diff)

`assets/prompts/grill_meta_v2.md` was authored as a sibling to v1 (not yet swapped). The substantive deltas:

### 4.1 New `EXTRACTION RULES` section (the load-bearing fix)

Five numbered rules, in order:

1. **Multi-slot extraction**: explicit example using Mrs Chen's exact turn-4 answer — "就给儿子一个人玩，不用联网对战" covers `scope` AND `robustness`. Forces the LLM to admit both at once.
2. **Semantic ≠ verbatim**: "If the answer entails the slot, the slot is covered. Don't wait for the exact word." Targets D-LLM-1.
3. **No re-ask of covered slots**: "After a slot enters `covered_slots`, NEVER ask another question targeting that slot." Targets D-LLM-2 (the loop is precisely "ask the job slot 5 times").
4. **Progress every turn**: "Every turn must either add ≥1 slot to `covered_slots` OR target a slot you have not yet asked." Prevents stalling.
5. **Confidence ≈ |covered_slots ∩ required| / 7**: gives the LLM a deterministic confidence rubric instead of letting it leave confidence at 0.1 forever. After 2-3 slots, confidence should be ≥ 0.3.

### 4.2 Reworked `QUESTIONING STYLE` (was `CONSTRAINTS`)

Old (v1 line 23):
> Build on the LATEST answer. Mirror back ("听起来你是说X，对吗？" / "If I heard right, you mean X — correct?").

New (v2):
> Build on the LATEST answer. **Voss mirror is SPARING, not every turn.** Open with "听起来你是说…" ONLY when the user's answer was ambiguous and needs confirmation. On normal turns, ask the next question directly. Never open more than 2 of any 5 turns with "听起来你是说…". The full mirror lives in the `playback` field at done=true.

This explicitly demotes Voss from "every turn" to "as needed", with a hard cap (≤ 2/5).

### 4.3 Methodology rewrite

v1's methodology was 8 bullets of methodology *names* (JTBD, Anchor, Memory, …) plus a separate list of slot ids. v2 merges them into the 8 slot ids with concrete one-line semantic descriptions, so the LLM has a clear extraction rubric for each slot directly inline.

### 4.4 What v2 preserves (wire contract — must not change)

- 8 canonical slot ids: `job, anchor, memory, first_run, robustness, scope, acceptance, mirror`.
- Required subset = 7 (mirror optional).
- `TurnPayload` JSON shape (turn, question, covered_slots, open_slots, confidence, done, rationale, optional playback).
- 15-turn hard ceiling, 6-12 soft target.
- `done=true ⇒ question=null` invariant.
- `confidence ≥ 0.8` AND `turn ≥ 4` AND required ⊆ covered termination predicate (Rust-side; documented in v2 for the LLM's awareness).
- Redaction policy line (`hidden_fields_redacted = true`, `"none-applies-grill-v1"`).
- Plain Chinese (or English if user opens in English).
- JSON-only output, no prose, no markdown fences.

### 4.5 Size

- v1: 3463 bytes
- v2: 3820 bytes (+10%; over the soft ~3500 cap but acceptable given the load-bearing additions; Meta context budget at ~128k tokens is not stressed)

---

## 5. Predicted outcomes (the falsifiable predictions)

For a Mrs Chen re-run with v2 (same user-answer script, fresh session):

### 5.1 PASS predictions (S2 validates if all hold)

- P1 — **Multi-slot extraction**: by turn 3 (after the "儿子放学后玩 30 分钟" answer), `covered_slots` includes at least `job` AND `acceptance` (the 30-min/day acceptance criterion). By turn 5 (after the single-player/no-online answer), `covered_slots` includes `job, scope, robustness` at minimum.
- P2 — **Confidence growth**: confidence reaches ≥ 0.5 by turn 5 (covered breadth ≈ 4/7 → 0.57).
- P3 — **Voss loop broken**: at most 1 turn out of turns 1–5 opens with "听起来你是说…" (versus all 4 LLM-successful follow-up turns in v1).
- P4 — **No same-slot re-ask**: no two consecutive turns target the same slot per the LLM's own `rationale`.
- P5 — **Wire contract intact**: every turn parses as a valid TurnPayload (schema predicate green); covered_slots ⊆ canonical 8 (vocab predicate green); covered_slots only grows (monotonicity predicate green).

### 5.2 PARTIAL outcomes (S2 partially validates)

- Only the Voss loop is fixed but extraction stays conservative → suggests the extraction rules are too abstract; needs more concrete examples.
- Only extraction improves but Voss loop persists → suggests the "≤ 2/5" cap is not strong enough; tighten to "only on ambiguity, full stop".
- v2 works on Mrs Chen but breaks P1 or P4 → overfit; needs to be re-balanced across personas.

### 5.3 FAIL outcomes (S2 falsifies for this defect class)

If predictions don't hold, candidate explanations to log:

- F-A: prompt language insufficient — the LLM ignores explicit rules in the prompt. Then the fix must be deterministic Rust-side post-processing (e.g. kernel reconciles `covered_slots` against an LLM-independent slot classifier).
- F-B: prompt size hits a sweet spot of attention; v2's longer prompt actually dilutes the slot list. Then the fix is restructuring, not adding content.
- F-C: the issue is in the Rust-side `coverage_state_summary` injection, not the meta-prompt itself — the kernel is silently overwriting the LLM's extraction. Then S2 is misdiagnosed; the bug is Rust-side.

Whichever fail mode lands becomes the followup atom's design input.

---

## 6. Risks

- **Overfit risk**: v2 uses Mrs Chen's exact turn-4 answer as an example. The LLM may learn "scope+robustness pairing" specifically and miss other multi-slot patterns. Mitigation: re-run v2 against Wave 1 P1 + P4 personas before declaring S2 validated. The example should be augmented with 1-2 more cross-domain examples if P1/P4 regress.
- **Wire-contract drift risk**: v2 restructured the methodology block. If the LLM emits a slot id from the old methodology names (e.g. "JTBD" instead of "job"), the vocab predicate rejects. Schema/vocab predicates will catch this; tape will show it as `turn_predicate_rejection` rather than silent corruption. Tested by: v2 explicitly lists the 8 canonical slot ids by exact wire name in both EXTRACTION RULES and field semantics.
- **Confidence-inflation risk**: P5 says confidence ≈ |covered ∩ required| / 7. LLM may now declare `done=true` earlier than v1 (good — fixes premature-stall), but if it OVER-extracts (claims a slot covered when the answer is genuinely thin), the dual-gate kernel predicate still requires confidence ≥ 0.8, so the bound holds. Worst case: bad spec.md; not bad tape.
- **Backend cache risk**: if `grill_meta_v1.md` is read once at backend boot and cached, the swap requires a restart. The A/B test script flags this; orchestrator must verify before re-running Mrs Chen.

---

## 7. Followup atoms (conditional on S2 validating)

- **A2 (Research-D backlog)**: build `turingos llm prompt-eval` — a CLI that takes a persona script + a prompt file and produces a deterministic verdict against the 5 PASS predictions. Systematizes this kind of S2-style experiment so prompt revisions become test-driven.
- **A4**: make the meta-prompt path TOML-driven (`turingos.toml: grill.meta_prompt = "assets/prompts/grill_meta_v2.md"`). Today the path is hardcoded; the A/B test relies on a filesystem swap hack. TOML routing makes v2/v3/v… cohabit cleanly.
- **A5 (if v2 succeeds)**: promote v2 → v1 (or rename and update Rust references). Requires Class-2 wire-up because `system_prompt_template_hash` in PromptCapsule changes — capsule schema unaffected but the hash on every new capsule shifts.
- **A6 (if v2 fails on P1/P4)**: add 2 more cross-domain extraction examples (e.g. a productivity tool persona, a creative writing persona) so the multi-slot pattern is not over-anchored to the tetris/single-child case.

---

## 8. Files produced by this atom

1. `assets/prompts/grill_meta_v2.md` — the proposed v2 meta-prompt (3820 bytes).
2. `handover/evidence/phase6_3_x_universality_1779111375/fixes/F7_s2_prompt_only_experiment.md` — this hypothesis document.
3. `handover/evidence/phase6_3_x_universality_1779111375/fixes/F7_s2_prompt_ab_test.sh` — executable A/B swap script (NOT YET RUN).

**Not modified**:
- `assets/prompts/grill_meta_v1.md` (preserved exactly)
- any Rust source
- any test
- any Cargo file
- any backend process

Per CLAUDE.md §9, this Class-1 atom requires self-audit only: prompt change is non-authoritative (kernel predicates still gate every turn); wire contract preserved; backup/restore script provided; falsifiability predictions are concrete and machine-checkable against the new session_log.jsonl.
