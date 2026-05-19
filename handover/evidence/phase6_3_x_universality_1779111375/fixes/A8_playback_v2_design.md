# A8 — Synthesis v2 (evidence-grounded playback) — Design Note

- **TB / fix id**: A8 (Phase 6.3.y, mini-wave v2 follow-up)
- **Surface (Class 1, prompt-only)**: `assets/prompts/grill_synthesis_{zh,en}_v2.md` (new files; v1 untouched)
- **Defect addressed**: D-NEW-1 / A1_fabricated_playback (verdict `mini_wave_v2/m5_p7_traditional_triagev2/verdict.json` §anomalies)
- **Status**: design + prompts authored; no Rust changes; no swap performed
- **Authorization**: architect /ultraplan 2026-05-19

---

## 1. Hypothesis

D-NEW-1 is a **synthesis-layer hallucination defect**, not a triage or extraction defect.
Evidence already proves triage v2 correctly populated `covered_slots = [job, anchor, memory, first_run, robustness, scope, acceptance]` from the M5 P7-Traditional answers (a 影片轉檔/mp4 transcoder with Redis + GCS Tokyo). Yet the LLM's `playback` text at T7 described an entirely different product (a "YouTube highlights / 精彩片段 extractor" with Canva-like editing, MP4/MOV input, 風格模板) — items 1–6 invented, only item 7 (acceptance: 5 YouTubers, no complaints) traceable to user input.

The v1 synthesis prompt (`assets/prompts/grill_synthesis_{zh,en}.md`, baked into `cmd_spec.rs::system_prompt()` line 1505–1554) frames the task as:

> "根据下面的 8 个问题 + 用户的 8 个回答，**综合**出一份 spec.md"
> "synthesise a spec.md"

A capable model reads "综合 / synthesise" as license to produce a coherent, plausible product spec — and when answers are sparse or transient-5xx-fragmented (M5 hit 6 5xxs across 7 turns), the model falls back on its prior over actual evidence.

**Hypothesis**: replacing the v1 prompt with an evidence-grounded v2 that (a) reframes the role from "product writer" to "stenographer", (b) requires a verbatim quote block before every section, and (c) hard-bans inventing product names / tech stack / features / categories the user didn't mention, will prevent the M5-style hallucination.

---

## 2. Diff summary v1 → v2

### Common to both zh and en

| Aspect | v1 | v2 |
|---|---|---|
| Role framing | "需求引导专家 / requirements-elicitation specialist" → produces a spec | **Stenographer / paraphraser** → transcribes what user said |
| Verb | 综合 / synthesise | **严格转写 / strictly transcribe** |
| Per-section evidence anchor | none | **mandatory `> 用户原话 / User said: ...` quote block before every section body**, citing source A<n> or `（未涉及） / (not covered in A1–A8)` |
| Hard NO list | one line ("不要扩写用户没说的功能 / don't invent features") | **5 explicit forbidden patterns**: product names, tech stacks, features, category-change, "probably wants" inference |
| Missing-info handling | append `## 还没问到 / ## Not Yet Asked` section listing what's missing | **same, plus**: explicitly states `（用户未提供具体信息） / (user did not provide specifics)` is a **preferred** legal output and forbids filling gaps with invented content |
| Slot ↔ section mapping | implicit (10 sections listed in order) | **explicit table** mapping each section to canonical slot id (job/anchor/memory/first_run/robustness/scope/acceptance/mirror) and default A<n> source |
| Mirror section | not separately required | **new dedicated `## 复述 / ## Mirror Playback` section**, 7 fridge-note lines, each traceable to a prior evidence anchor; A8 corrections must be reflected |
| Negative example | none | **§4 worked counter-example** using the actual M5 P7 defect pattern: A1=影片轉檔工具 → wrong output writes "YouTube 精彩片段提取器 / highlights extractor + Canva + Whisper" — labelled "严重违规 / severe violation" |
| Positive example | none | **§5 worked positive example** with the evidence-anchor format applied |
| Output length contract | `<!-- TURINGOS_SPEC_END -->` final line; no preamble | same |

### zh-specific changes

- Examples deliberately use **Traditional Chinese source quotes** (「想做一個影片轉檔工具…」) to demonstrate the script-preservation rule already won by Triage v2 in M5 (Trad vocab 影片/轉檔/檔案/錨點 must survive into the spec body, not be silently normalised).
- Adds explicit instruction that quotes preserve "原字、原标点、原繁简体" (original characters, punctuation, traditional/simplified script).

### en-specific changes

- Adds explicit instruction that quotes preserve "wording, punctuation, script, spelling" (handles Trad/Simp + UK/US spelling).
- Slightly shorter than zh after trimming (4483 vs 4346 bytes) — both within 4500-byte budget.

### Size

- v1 zh = 1755 B, v2 zh = 4346 B (+148%)
- v1 en = 1338 B, v2 en = 4483 B (+235%)
- Both v2 files ≤ 4500 B as required.

---

## 3. Predicted outcomes when re-run on M5 P7 evidence

Replay the M5 P7 Traditional session (8 user answers from `mini_wave_v2/m5_p7_traditional_triagev2/session_log.jsonl`) against the synthesis path with v2 prompts swapped in.

**Expected v2 playback content** (every line traceable):

```
## 一句话目标
> 用户原话：「想做一個影片轉檔工具, 支援拖曳上傳, 輸出 mp4」（来自 A1）
做一个影片转档工具，支持拖曳上传，输出 mp4。

## 像谁 (Reference)
> 用户原话：「錨點就是每個檔案的 SHA256 + 原檔名, 避免重複轉檔同一個檔」（来自 A2）
…用 SHA256 + 原檔名作为锚点，避免重复转档同一文件。

## 程序要记住的东西 (Memory)
> 用户原话：「記憶用 Redis 存任務狀態, 七天後自動清除。原始檔放 GCS bucket」（来自 A3）
- Redis 存任务状态
- 七天后自动清除
- 原始档放 GCS bucket

## 第一次使用 (First Run)
> 用户原话：「第一次使用: 直接拖檔到網頁, 不需註冊登入, 輸出檔下載完就走」（来自 A4）
1. 直接拖檔到網頁
2. 不需註冊登入
3. 輸出檔下載完就走

## 不能搞坏的情况 (Robustness)
> 用户原话：「上傳失敗自動重試三次; 轉檔失敗顯示具體錯誤 (FFmpeg stderr 摘要), 不要只說『失敗』」（来自 A5）
- 上傳失敗自動重試三次
- 轉檔失敗顯示 FFmpeg stderr 摘要

## 故意不做的 (Out of Scope)
> 用户原话：「個人創作者用, QPS 約 10, 單檔 ≤ 500MB。不做多人協作」（来自 A6）
- 不做多人协作
- 单档 > 500MB 不支持
- QPS > 10 不支持

## 算成功 (Acceptance)
> 用户原话：「驗收: 找五位 YouTuber 試用一週，沒人抱怨就算成功」（来自 A7）
找 5 位 YouTuber 试用一周，没人抱怨即成功。

## 复述 (Mirror)
> 用户原话：（A8 中未涉及；T8 mirror 在 M5 中未达）
1. 你要做一个影片转档工具…
2. 像 SHA256+原檔名 做锚点…
3. Redis 七天，原档 GCS…
4. 拖檔到网页，不登录…
5. 失败重试 3 次，显示 FFmpeg 摘要…
6. 不做多人协作，单档 ≤ 500MB…
7. 5 位 YouTuber 试用一周不被骂…
```

**Critically absent**: any mention of "YouTube highlights extractor / 精彩片段 / Canva / Whisper / 风格模板 / MOV input / 自动剪辑 / 字幕". If the v2 prompt is doing its job, those v1-hallucinated tokens cannot survive the verbatim-quote rule and the negative-example anchor.

### Falsification criteria

The hypothesis is falsified if any of:
1. v2 playback still introduces a product noun absent from A1–A8 (e.g. still writes "highlights extractor", "Canva", etc.).
2. v2 playback drops the evidence anchor format on any section.
3. v2 playback silently back-translates Trad → Simp tokens that were verbatim in A1–A8 (e.g. 影片 → 视频, 檔案 → 文件, 錨點 → 锚点 inside the verbatim quote block).
4. v2 playback fills a missing slot with invented content instead of using `（用户未提供具体信息） / (user did not provide specifics)`.

---

## 4. Risks and tradeoffs (architect-acknowledged)

- **R1 — stilted spec.md**: strict verbatim quoting may produce a spec that reads less like a polished product brief and more like a transcript. **Acceptable** per architect: the user can iterate; hallucination is the bigger harm.
- **R2 — `（用户未提供具体信息）` markers look unprofessional in delivered specs**: a delivered spec.md may show several `（未涉及）` blocks if the grill terminates early. **Acceptable** per architect: those markers are honest signal of what to ask next and feed naturally into the existing `## 还没问到` mechanism.
- **R3 — model may still rebel against the role swap**: stronger models sometimes ignore "stenographer" framing and revert to "helpful product writer". Mitigation: §4 negative example uses the exact M5 defect pattern as a labelled "严重违规", which should be salient enough for steerable models (claude-3.5-sonnet, gpt-4o, claude-opus-4-x).
- **R4 — token bloat**: v2 is ~2.5× v1 size. Each synthesis call adds ~3 KB of prompt tokens. **Acceptable**: synthesis runs once per session, not per turn.
- **R5 — no fallback for `synthesise_spec_md_no_llm`**: `cmd_spec.rs:1339` falls back to a deterministic template when no API key is set. That path does NOT use the synthesis prompt at all and is therefore unaffected by v1↔v2 swap. Out of scope.

---

## 5. A/B test plan (executed separately)

Swap script: `handover/evidence/phase6_3_x_universality_1779111375/fixes/A8_playback_v2_ab_test.sh`.

The swap is **file-rename based** — copies `grill_synthesis_{zh,en}_v2.md` over the v1 paths that `cmd_spec.rs::system_prompt()` reads at runtime (currently inline string literal). Note: the current implementation has the v1 prompt **inlined as a Rust string literal**, so a pure file swap will not take effect without either (a) wiring the prompt loader to read from disk, or (b) modifying the Rust string literal directly. The script implements path (a)-equivalent by replacing the inline literal via a temporary patch and rebuilding — see script body for details. Architect to confirm preferred approach before running.

Expected outcome per §3: v2 playback contains zero hallucinated product nouns and 100% evidence-anchored sections, with `（用户未提供具体信息）` markers where slots are uncovered.

---

## 6. Files authored by A8

- `assets/prompts/grill_synthesis_zh_v2.md` (4346 B)
- `assets/prompts/grill_synthesis_en_v2.md` (4483 B)
- `handover/evidence/phase6_3_x_universality_1779111375/fixes/A8_playback_v2_design.md` (this file)
- `handover/evidence/phase6_3_x_universality_1779111375/fixes/A8_playback_v2_ab_test.sh` (swap script)

**Not touched** (per constraints): v1 prompts, any Rust source, any test, backend, any other fix-line files. No commit, no push.
