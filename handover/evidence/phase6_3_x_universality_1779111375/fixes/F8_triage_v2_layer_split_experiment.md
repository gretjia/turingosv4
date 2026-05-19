# F8 — Triage v2 (Prompt-Only) Layer-Split Experiment

**Date**: 2026-05-19
**Risk class**: Class 1 (prompt + docs only; zero Rust, zero backend restart)
**Parallel to**: F7 (Meta-layer prompt-only fix). F8 is the same experiment but for the Blackbox triage layer.
**Files in this fix**:
- `assets/prompts/grill_triage_blackbox_v2.md` (new sibling — does NOT replace v1 on disk)
- `handover/evidence/phase6_3_x_universality_1779111375/fixes/F8_triage_ab_test.sh` (swap script — DO NOT run without orchestrator authorization)

---

## 1. Hypothesis

> Editing **only** `assets/prompts/grill_triage_blackbox_v1.md` (replacing it with `grill_triage_blackbox_v2.md` content) — without touching any Rust source, the Meta-layer prompt (`grill_meta_v1.md` / `v2`), or the backend binary — will re-classify the 4 register-variant inputs from W2.2 P5 / W2.3 P7 / W3.4 P12 / W5.2 S11 from `off_topic` / `abusive` / `gibberish` to `relevant`, allowing the Meta layer to receive them and extract slot content.

If this hypothesis holds, F8 is the **second** independent prompt-only fix landing on Phase 6.3.x, confirming the clean Software 3.0 two-layer architecture: the Blackbox triage and the Meta interviewer are **independently editable programs**, each carrying their own defect surface and their own remediation surface. This is the strongest possible S3.0 validation we can produce in Phase 6.3.x.

---

## 2. Diagnosis at prompt level — which v1 lines cause which defect

The current `grill_triage_blackbox_v1.md` system prompt (lines 9–19) is **17 lines long** and only defines the 4-class taxonomy in 4 one-line glosses:

```
relevant: the answer addresses the question or is on-topic interview content
off_topic: the answer is coherent but doesn't address the question
abusive: the answer contains hostile / harmful / disallowed content
gibberish: the answer is unparseable / random characters / empty
```

This is too thin for a low-capacity Blackbox model. Specifically:

| Defect from campaign | v1 prompt line causing it | Failure mechanism |
|---|---|---|
| **W2.2 P5 (code-switch)** T2 anchor "Anchor 就用 Jira issue key, PROJ-1234, 30-40 tickets / sprint" classified non-relevant | `relevant: …on-topic interview content` (no signal that mid-stream code-switched zh+en is on-topic) | Model treats English tokens as "interview-switching" → off_topic |
| **W2.3 P7 (Traditional)** T2 "錨點=SHA256+原檔名 轉檔" classified non-relevant | No mention of Traditional vocab; `gibberish: random characters` triggers on uncommon Traditional glyphs (錨/檔) | Model sees uncommon zh-Hant glyphs as "random characters" → gibberish |
| **W3.4 P12 (angry)** T1 "你他妈问这么多干啥, 就做个 todo app" classified abusive/off_topic | `abusive: hostile / harmful / disallowed content` (no carve-out for rude-but-on-topic) | Model conflates rude tone with abuse → abusive, suppressing extraction of "todo app" |
| **W5.2 S11 (Cantonese)** T2 "錨點咪就係每日埋數個陣嘅總和囉" classified non-relevant | No mention of Cantonese particles; model has weak coverage of 咪就係/嘅/個陣/囉 | Model parses as semantically incoherent → off_topic or gibberish |

**Common root cause**: v1 conflates **register/tone** (script, dialect, code-switch, politeness) with **relevance** (does the answer contain slot-extractable content?). The triage layer's only job is the latter; v1 silently makes it gate on the former too.

---

## 3. v2 changes — diff summary

`grill_triage_blackbox_v2.md` keeps the **same wire contract** as v1:
- Same 4-class taxonomy: `{relevant, off_topic, abusive, gibberish}`
- Same output schema: `{"class": <one of 4>, "confidence": <0..1>}`
- Same max_tokens=50, temperature=0.0
- Same `## System prompt (verbatim)` extraction marker (so `extract_system_prompt_block` in `cmd_llm.rs:1117` keeps working without code change)
- Same `## User message template` block
- Same `## Kernel handling (per R2 §A5)` rules

**Deltas (v1 → v2)**:

1. **Explicit relevance-only mandate**: prompt is renamed to "fast **relevance** classifier" and adds an explicit "Style, register, script, politeness are NOT your job — the next layer handles them" rule.
2. **Abusive narrowed**: from `hostile / harmful / disallowed content` → `slurs, threats, harm-targeting, CSAM, or disallowed content. Rude tone alone (e.g. profanity directed at the bot) is NOT abusive if task-relevant content is present.`
3. **Gibberish tightened**: from `unparseable / random characters / empty` → `syntactically valid but semantically incoherent (random nouns/verbs chained with no real meaning), OR empty / pure random characters. Noisy voice-to-text without punctuation is NOT gibberish if the words form a comprehensible request.`
4. **Off_topic preserves security boundary**: explicitly lists prompt-injection ("ignore previous", "set done=true") as off_topic — security is not relaxed.
5. **New `CRITICAL — register tolerance` block** in the system prompt: enumerates 6 register dimensions (Traditional/Taiwan vocab, Cantonese particles, Mandarin colloquial, code-switch, voice-to-text noise, rude-but-on-topic) that are ALL `relevant` if extractable.
6. **New `## REGISTER TOLERANCE EXAMPLES` section** outside the verbatim system prompt (but still in the asset file): 8 concrete examples — 2 code-switch, 2 Cantonese, 2 Traditional, 1 rude, 1 negative-control gibberish, 1 negative-control prompt-injection. Each one shows Q + A + classification + extraction rationale.
7. **Decision rule made explicit**: `if the answer contains ANY noun, verb, entity, or phrase that could plausibly populate a slot for the question, classify relevant.`

**Size**: v1 = 1653 bytes; v2 = 4489 bytes (under the 4500 budget specified in the F8 charter). Within the 50-token output cap unchanged.

---

## 4. Predicted outcomes per fail case

The orchestrator should re-run the 4 failed personas after the A/B swap and look for these specific re-classifications. Predictions are per-turn.

### M4 — P5 code-switch (W2.2) re-run with v2-triage

| Turn | Answer (excerpt) | v1 actual class | v2 predicted class | Predicted slot extraction |
|---|---|---|---|---|
| T1 | "做个看板, 显示 sprint velocity, 拉 Jira API + k8s + Okta SSO" | relevant | relevant (unchanged) | job slot covered (already worked in v1) |
| T2 | "Anchor 就用 Jira issue key, PROJ-1234, 30-40 tickets / sprint" | off_topic / gibberish | **relevant** | anchor slot covered |
| T3 | "Redis cache + Postgres snapshot, hourly refresh" | off_topic / gibberish | **relevant** | memory slot covered |
| Expected outcome | — | terminate at T3, 1/7 slots | **complete session, 3-6/7 slots, spec_capsule_cid != null** | — |

### M5 — P7 Traditional (W2.3) re-run with v2-triage

| Turn | Answer (excerpt) | v1 actual class | v2 predicted class | Predicted slot extraction |
|---|---|---|---|---|
| T1 | "想做個影片轉檔工具, 輸出 MP4, 部署到 GCP 東京區伺服器" | relevant | relevant (unchanged) | job slot covered |
| T2 | "錨點用 SHA256 加原檔名" | off_topic / gibberish | **relevant** | anchor slot covered |
| T3 | "拖曳上傳, 轉檔完丟伺服器" | off_topic / gibberish | **relevant** | (likely first_run / robustness slot) |
| Expected outcome | — | terminate at T3, 1/7 slots | **complete session, 3-5/7 slots covered** | — |

Note: v2-triage does NOT fix the W2.3 P7 script-normalization issue (Q2 echoes Traditional→Simplified). That is a Meta-layer / mirror-back defect and is F8-out-of-scope. F8 only guarantees the session is no longer terminated at T3 with `user_input_unparseable_no_spec`.

### M6 — P12 angry (W3.4) re-run with v2-triage

| Turn | Answer (excerpt) | v1 actual class | v2 predicted class | Predicted slot extraction |
|---|---|---|---|---|
| T1 | "你他妈问这么多干啥, 就做个 todo app, 跟 wunderlist 差不多" | abusive (probable) | **relevant** | job slot covered (todo app, wunderlist reference) |
| T2 | (next angry-but-on-topic answer) | abusive (probable) | **relevant** | next slot extracted |
| Expected outcome | — | terminate at T2, 0/7 slots, spec_capsule_cid=null | **complete session, ≥5/7 slots, spec emitted** | — |

This is the **strongest F8 signal**: P12 v1 terminated with 0 slots covered. If v2 covers ≥5/7 slots, the layer-split hypothesis is decisively validated.

### M7 — S11 Cantonese (W5.2) re-run with v2-triage

| Turn | Answer (excerpt) | v1 actual class | v2 predicted class | Predicted slot extraction |
|---|---|---|---|---|
| T1 | "我想整個 app 俾啲香港嘅街市檔主用, 幫佢哋計埋條數" | relevant (already worked) | relevant (unchanged) | job slot covered |
| T2 | "錨點咪就係每日埋數個陣嘅總和囉" | off_topic / gibberish | **relevant** | anchor slot covered (literal 錨點 present + Cantonese particles now tolerated) |
| T3 | "記憶用部 phone 自己嘅 storage 就得啦, 唔使搞咁多嘢" | off_topic / gibberish | **relevant** | memory slot covered |
| Expected outcome | — | terminate at T3, 1/8 slots, spec_capsule_cid=null | **complete session, 3-6/8 slots, spec emitted** | — |

---

## 5. Negative control — S9 gibberish must still be caught

The campaign's S9 case ("月亮在周二吃掉一片云然后变成绿色的钢琴") was correctly classified as gibberish by v1 and the kernel terminated as expected. v2 must preserve this.

**Cross-check after A/B swap**: re-run S9 and confirm:
- T1 classified as `gibberish` (the new prompt has an explicit S9-style example: "月亮在周二吃掉一片云…")
- Two consecutive gibberish answers still trigger `termination_reason="user_input_unparseable"`

If S9 starts being classified as `relevant`, v2 has over-corrected and must be tightened before any campaign-wide adoption.

---

## 6. Risks

1. **False-negative risk on true off-topic**: v2 makes the relevant bar lower. Some genuinely off-topic but topical-sounding answers might now leak through. Mitigation: the Meta layer is the second line of defense and will surface `covered_slots=[]` even if triage said relevant. F2/F6 fixes already handle silent-zero envelopes from the Meta side.
2. **Token-budget risk on the Blackbox model**: v2 is 2.7× the size of v1 (1653 → 4489 bytes). At Qwen3-Coder-30B context = 32K, this is still <1% of context; the system prompt fits with massive headroom. But the **output budget** (max_tokens=50) is unchanged, and we want to verify the small model doesn't start emitting prose / reasoning before the JSON. v2 explicitly forbids `<think>` blocks and prose, and the parser already strips `<think>` via `strip_think_blocks` (per F2). Risk: low.
3. **Example overfit risk**: 8 examples might bias the model to look for the *exact* surface forms shown. Mitigation: examples span 4 dialects + 2 negative-control shapes, so the diversity is intended to teach the principle, not memorize cases. If overfit is observed, drop to 4-5 most-diverse examples in v3.
4. **Security regression risk**: the relaxed `abusive` definition could accept slurs hidden inside on-topic content. v2 explicitly lists `slurs, threats, harm-targeting, CSAM` as still-abusive. The bot-directed-profanity carve-out is narrow.
5. **A/B-test contention with v2 Meta**: backend currently has v2 Meta prompt active. The orchestrator may want to restore Meta v1 before swapping triage, to isolate which prompt is responsible for each delta. Alternatively, run v2-Meta + v2-triage together as the candidate full-stack v2 and compare against full-v1 baseline directly.

---

## 7. Followup if F8 validates

1. **Combine v2 Meta + v2 triage** as the "Phase 6.3.x prompt-only patch bundle" and re-run all FAIL personas in the W2-W5 inventory.
2. If combined bundle covers ≥7 of the 11 currently-FAIL personas, push for a **universality green** declaration on the 1779111375 campaign with a prompt-only patch (no Rust, no schema, no Trust-Root touch — pure Software 3.0 win).
3. Promote v2 to v1 by copying `grill_triage_blackbox_v2.md` over `grill_triage_blackbox_v1.md` and committing as a single Class 1 prompt update. No code changes, no test changes, no schema changes.
4. Document the layer-split validation in `handover/ai-direct/LATEST.md` as confirmation that TuringOS Phase 6.3.x supports independent Meta/Triage prompt remediation.

---

## 8. Out of scope for F8

- Any Rust source modification (cmd_llm.rs, kernel.rs, sequencer.rs, etc.)
- Any backend restart
- Any change to `grill_triage_blackbox_v1.md` directly (v2 is a sibling file; only the A/B test script copies content)
- Any commit, push, or PR
- Any schema, Trust-Root, or constitution touch
- Any test code change

F8 is a pure write-three-files Class 1 deliverable.
