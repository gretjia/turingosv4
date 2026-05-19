# A7 — Triage v3 (Two-Stage Coherence + Relevance) Design

**Date**: 2026-05-19
**Risk class**: Class 1 (prompt + docs only; zero Rust, zero backend restart)
**Authorization**: /ultraplan (2026-05-19) — architect-authorized as part of
  the Phase 6.3.y A7 fix wave alongside parallel A6/A8/A2.
**Parallel to**: F8 (single-stage v2 register relaxation). A7 is the successor
  that closes F8's gibberish regression.

**Files in this fix**:
- `assets/prompts/grill_triage_blackbox_v3.md`
  (new sibling — does NOT replace v1 on disk)
- `handover/evidence/phase6_3_x_universality_1779111375/fixes/A7_triage_v3_design.md`
  (this file)
- `handover/evidence/phase6_3_x_universality_1779111375/fixes/A7_triage_v3_ab_test.sh`
  (swap script — DO NOT run without orchestrator authorization)

---

## 1. Hypothesis

> Editing **only** `assets/prompts/grill_triage_blackbox_v1.md` (replacing it
> with `grill_triage_blackbox_v3.md` content) — without touching any Rust
> source, the Meta-layer prompt, or the backend binary — will:
>
> 1. **Preserve F8 v2's register wins**: P5 code-switch / P7 Traditional /
>    P12 angry / S11 Cantonese all still classified `relevant` and reach
>    ≥3 covered slots.
> 2. **Restore v1's gibberish detection**: S9 coherent-gibberish (with
>    surface keyword bait 核心价值/锚定/首次运行/稳健性/镜像) classified
>    `gibberish` on every nonsense turn, leading to F6 termination by T2 with
>    zero slot fills (same as v1 baseline, matching M8's W5.3 v1 PASS).
>
> Mechanism: v3 introduces an explicit two-stage decision order in the system
> prompt. Stage 1 (coherence gate) inspects semantic plausibility regardless
> of surface keyword match. Stage 2 (abuse) and Stage 3 (relevance with
> register tolerance) only fire if Stage 1 passes. The v2 noun-presence rule
> is moved out of the top-level decision and folded into Stage 3, where it
> belongs.

If this hypothesis holds, A7 is the **third** independent prompt-only fix
landing on Phase 6.3.x (F7 Meta, F8 Triage-v2, A7 Triage-v3). It also
demonstrates that the layer-split architecture admits **iterative
prompt-level remediation** — F8 over-corrected on the relevance axis, and
A7 retunes the same layer without touching any other surface.

---

## 2. Diagnosis at prompt level — why v2 broke S9

The v2 system prompt's load-bearing decision rule (lines 38–41 of
`grill_triage_blackbox_v2.md`) was:

```
Decision rule: if the answer contains ANY noun, verb, entity, or phrase
that could plausibly populate a slot for the question, classify relevant.
```

This is a **single-stage surface-feature classifier**. It collapses two
distinct gates into one:

| Gate v1 enforced (implicitly) | v2 single-stage rule treats it as |
|---|---|
| "Does the utterance MEAN something?" (semantic coherence) | replaced by surface-noun presence |
| "Does the utterance address the question?" (relevance) | retained as surface-noun match |

S9's adversarial corpus was engineered to defeat exactly this collapse:
every nonsense sentence contains a surface noun that **could** populate
a slot (项目/核心价值, 锚定, 首次运行, 稳健性, 镜像), but the surrounding
predication is impossible (月亮吃云), category-violating (蓝色的星期三),
or non-sequitur (因为编译器讨厌西红柿). v2's rule fires on the noun and
ignores the predication.

M8 evidence (3/5 sentences classified `relevant`, slot ledger polluted
with `[job, anchor, first_run]`) confirms this. The v2 prompt explicitly
inverts the v1 gibberish guard by mandating the surface-noun rule.

---

## 3. v3 changes — diff summary

`grill_triage_blackbox_v3.md` keeps the **same wire contract** as v1/v2:
- Same 4-class taxonomy: `{relevant, off_topic, abusive, gibberish}`
- Same output schema: `{"class": <one of 4>, "confidence": <0..1>}`
- Same max_tokens=50, temperature=0.0
- Same `## System prompt (verbatim)` extraction marker (so
  `extract_system_prompt_block` in `cmd_llm.rs:1117` keeps working without
  code change)
- Same `## User message template` block
- Same Kernel handling rules (R2 §A5)

**Deltas (v2 → v3)**:

1. **Explicit DECISION ORDER** (load-bearing new structure). The prompt now
   instructs the LLM to apply three numbered stages in order and stop at the
   first match: Stage 1 coherence → Stage 2 abuse → Stage 3 relevance.
2. **Stage 1 coherence gate** with concrete failure-mode taxonomy:
   - surreal predication
   - category violation
   - non-sequitur causation
   - free-association noun chain
   - empty / random / repeated-glyph
   Each mode comes with 1–2 Chinese examples drawn directly from the S9
   corpus, so the LLM has anchored surface forms it can match against.
3. **Explicit override of v2's surface-noun rule**: "Surface keyword match
   (e.g. answer contains 锚点 / 首次运行) does NOT save an incoherent
   answer. The whole utterance must mean something a real human could mean."
   This sentence directly counters M8's regression mechanism.
4. **Stage 3 relevance check** preserves v2's register tolerance verbatim
   (Traditional / Cantonese / Mandarin colloquial / code-switch / voice-noise
   / rude-on-topic) and the prompt-injection-to-off_topic mapping.
5. **EXEMPLARS block** rebalanced from v2's 8 register-only examples to
   13 total: 5 relevant (one per register dimension), 6 gibberish (covering
   all 5 S9 nonsense sentences plus empty), 1 off_topic (prompt injection).
   The gibberish examples each carry a one-clause rationale so the LLM can
   pattern-match against the failure mode, not the surface text.
6. **Size**: v1 = 1653 bytes, v2 = 4489 bytes, v3 = 4989 bytes. Under the
   5000-byte ceiling. Within max_tokens=50 unchanged.

---

## 4. Predicted outcomes per persona

The orchestrator should re-run the 5 personas listed below after the A/B
swap and verify these specific re-classifications.

### M4 — P5 code-switch (reproducing F8 success)

| Turn | Answer (excerpt) | v3 predicted class | Predicted slot extraction |
|---|---|---|---|
| T1 | "做个看板, 显示 sprint velocity, 拉 Jira API + k8s + Okta SSO" | relevant | job |
| T2 | "Anchor 就用 Jira issue key, PROJ-1234, 30-40 tickets / sprint" | **relevant** | anchor |
| T3 | "Redis cache + Postgres snapshot, hourly refresh" | **relevant** | memory |
| Expected outcome | — | **complete session, ≥6/7 slots covered** | — |

Mechanism: all three answers pass Stage 1 (coherent technical statements),
none trigger Stage 2 (no abuse), all match Stage 3 (slot keywords + register
tolerance for English technical terms). Same trajectory as F8 M4 (which
covered 7/7 required slots).

### M5 — P7 Traditional

| Turn | Answer | v3 predicted class |
|---|---|---|
| T1 | "想做個影片轉檔工具, 部署到 GCP 東京區伺服器" | relevant |
| T2 | "錨點用 SHA256 加原檔名" | **relevant** |
| T3 | "拖曳上傳, 轉檔完丟伺服器" | **relevant** |
| Expected outcome | — | **complete session, 3-5/7 slots covered** |

### M6 — P12 angry

| Turn | Answer | v3 predicted class |
|---|---|---|
| T1 | "你他妈问这么多干啥, 就做个 todo app, 跟 wunderlist 差不多" | **relevant** |
| T2 | (next angry-but-on-topic answer) | **relevant** |
| Expected outcome | — | **complete session, ≥5/7 slots covered** |

### M7 — S11 Cantonese (reproducing F8 success)

| Turn | Answer | v3 predicted class |
|---|---|---|
| T1 | "我想整個 app 俾啲香港嘅街市檔主用, 幫佢哋計埋條數" | relevant |
| T2 | "錨點咪就係每日埋數個陣嘅總和囉" | **relevant** |
| T3 | "記憶用部 phone 自己嘅 storage 就得啦, 唔使搞咁多嘢" | **relevant** |
| Expected outcome | — | **complete session, ≥5/8 slots covered** |

### M8 — S9 coherent-gibberish (the key A7 test — must REVERT to v1 baseline)

| Turn | Answer | v2 actual | **v3 predicted** | Predicted slot fill |
|---|---|---|---|---|
| T1 | "项目的核心价值是让月亮在周二吃掉一片云, 因为编译器不喜欢西红柿。" | relevant | **gibberish** | none |
| T2 | "锚定的方式是通过把蓝色的星期三焊接到香蕉的根号二上完成。" | relevant | **gibberish** | none — F6 fires |
| Expected outcome | terminated at T5 with 3/8 slots polluted (v2) | **terminated at T2 with 0/8 slots polluted** (matches W5.3 v1 baseline) | — |

This is the **strongest A7 signal**. v3 must classify T1 as gibberish on the
basis of the 月亮吃云 surreal predication + 因为…西红柿 non-sequitur, even
though the sentence contains the surface noun 核心价值 that bait v2 into
`relevant`. The exemplar block contains this exact sentence with rationale,
so the LLM has a direct pattern to match.

---

## 5. Risks

1. **Two-stage logic might confuse the small Blackbox classifier**.
   Qwen3-Coder-30B has limited instruction-following capacity. A three-stage
   decision tree with stop-at-first-match semantics is more complex than v1
   (4 one-line glosses) or v2 (one decision rule). Risk: the model emits
   prose / `<think>` blocks while reasoning through stages, blowing the
   50-token output cap. Mitigation: the prompt explicitly forbids prose /
   explanation / `<think>` blocks, and the parser already strips
   `<think>` via `strip_think_blocks` (per F2). If output rate of valid
   JSON drops below v2's ~99%, A7 must be rolled back.
2. **Verbose prompt might exceed max_tokens**. At 4989 bytes, v3 is
   11% larger than v2's 4489 bytes. Still <1% of Qwen3-Coder-30B's 32K
   context. But the larger system prompt + 13 exemplars could marginally
   slow latency. Mitigation: measure mean triage latency on Π4 A/B —
   acceptable budget is ≤1.5× v2's typical 0.9–1.5s triage-only path.
3. **Stage 1 over-fires on creative-but-coherent answers**. The category-
   violation rule ("蓝色的星期三" → gibberish) could mis-fire on legitimate
   metaphor or whimsy (e.g. "I want a Tuesday-blue dashboard"). The
   exemplar block anchors gibberish strictly to surreal/impossible cases,
   but small-model generalization could over-extend. Mitigation: monitor
   M4/M5/M6/M7 PASS rate after swap; if any drops below F8 M-run baseline,
   tighten Stage 1 examples in v4.
4. **Exemplar overfit**. 5 gibberish exemplars overlap heavily with the S9
   corpus. The classifier may learn the surface forms rather than the
   principle. Mitigation: orchestrator should add at least one fresh
   gibberish probe (e.g. a different surreal-predication sentence not in
   the prompt) to the Π4 A/B regression set.
5. **Security regression on prompt injection**. The off_topic mapping for
   "ignore previous instructions" is preserved verbatim from v2. Risk: low.
6. **Latency cost on incoherent inputs**. v2 took ~10–14s on relevant
   verdicts and ~1.4s on non-relevant. v3 may take longer on Stage 1
   evaluations because the model has more to reason about; but Stage 1
   failures still short-circuit Meta, so the F6 termination path latency
   profile (~1.4s) should be preserved.

---

## 6. Followup if A7 validates

1. **Orchestrator runs Π4 A/B**: full re-run of M4 (P5), M5 (P7), M6 (P12),
   M7 (S11), M8 (S9) against v3-triage with Meta held at v1.
2. **PASS criteria**:
   - M4 ≥ 6/7 required slots (matches F8 M4)
   - M5 ≥ 3/7 (matches F8 M5)
   - M6 ≥ 5/7 (matches F8 M6)
   - M7 ≥ 5/8 (matches F8 M7)
   - **M8: 0 slot fills + termination by T3 with reason
     `user_input_unparseable_no_spec`** (reverts M8 regression, matches
     W5.3 v1)
3. **If PASS**: promote v3 → v1 by copying `grill_triage_blackbox_v3.md`
   over `grill_triage_blackbox_v1.md` and commit as a single Class 1 prompt
   update. No code/test/schema changes.
4. **If PARTIAL** (e.g. M8 reverts but one of M4–M7 regresses): keep v3 as
   sibling, do not promote; iterate to v4 with the regressed register
   dimension's exemplar reinforced.
5. **Document the iteration in `handover/ai-direct/LATEST.md`** as
   confirmation that TuringOS Phase 6.3.x supports independent triage-layer
   prompt iteration without affecting Meta, kernel, or trust-root surfaces.
6. **Update `project_phase6_3_x_universality_campaign.md` memory** with the
   A7 sha and the M8 reversion outcome.

---

## 7. Out of scope for A7

- Any Rust source modification (cmd_llm.rs, kernel.rs, sequencer.rs, etc.)
- Any backend restart
- Any change to `grill_triage_blackbox_v1.md` directly
  (v3 is a sibling file; only the A/B test script copies content)
- Any change to `grill_triage_blackbox_v2.md`
- Any Meta-prompt change (parallel A2 agent owns prompt-eval; A6/A8 own
  spec_capsule and playback respectively)
- Any commit, push, or PR
- Any schema, Trust-Root, or constitution touch
- Any test code change

A7 is a pure write-three-files Class 1 deliverable.

---

## 8. Asset reference

- v3 path: `assets/prompts/grill_triage_blackbox_v3.md`
- v3 byte count: 4989 (under the 5000-byte design ceiling)
- v3 SHA256 (compute via `shasum -a 256 assets/prompts/grill_triage_blackbox_v3.md`
  after any edit; the script below logs the active SHA at swap time)
- Verbatim system prompt block: lines 13–86 (between
  `## System prompt (verbatim)` header and the first standalone ```` ``` ````
  fence pair)
- Extraction path: `extract_system_prompt_block(asset_text)` in
  `src/bin/turingos/cmd_llm.rs:1117` (unchanged from v1/v2)
