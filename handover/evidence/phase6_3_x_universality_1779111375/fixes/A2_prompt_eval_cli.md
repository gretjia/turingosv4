# A2 — `turingos llm prompt-eval` CLI

**Date**: 2026-05-19
**Phase**: 6.3.y (universality follow-up)
**Risk class**: 2 (production wire-up; reuses existing LLM client + parser; no Class-4 surface touched)
**Pre-fix HEAD**: `3e0fa79c`
**Branch**: `codex/tisr-phase6-3-x-grill-driven` (read-only git)
**Files in this atom**:
- `src/bin/turingos/cmd_llm.rs` — new `prompt-eval` sub-action (+~640 LOC)
- `tests/fixtures/grill_prompt_eval_fixture.jsonl` — starter fixture (8 rows: 2 meta + 4 triage + 2 playback)
- `tests/cmd_llm_prompt_eval_stub.rs` — CLI surface contract tests (9 tests)
- `tests/cmd_llm_prompt_eval_v1_vs_v2_triage.rs` — golden v1 vs v2 A/B test (4 active + 1 ignored live-LLM)
- `handover/evidence/phase6_3_x_universality_1779111375/fixes/A2_prompt_eval_cli.md` — this file

---

## 1. Mission

The Phase 6.3.x universality campaign (1779111375) discovered the canonical Software 3.0 production-readiness gap: **prompt edits cascade non-locally**. F8 (Triage v2) fixed register tolerance — Cantonese particles, Traditional vocab, code-switch zh+en, rude-but-on-topic input — all started classifying as `relevant`, unlocking the Meta layer for personas P5/P7/P12/S11. **But v2 broke gibberish detection**: 3 of 5 nonsense inputs the M8 trial threw at v2 were classified as `relevant` instead of `gibberish`. The widened "register tolerance" carve-outs over-relaxed the gibberish floor.

In a Software 1.0 system this would be caught by a unit-test regression on a fixed input table. In a Software 3.0 system — where the "program" is a markdown prompt fed to a black-box LLM — there was no such net. **`turingos llm prompt-eval` is that net.**

> Definition. `prompt-eval` runs a candidate prompt against a frozen JSONL fixture of Q/A test cases, scores each model output against an `expected_*` verdict, aggregates pass/fail, and exits non-zero if any row fails. Optionally computes a baseline-delta against the currently-promoted prompt to surface gained/regressed rows.

If A2 is correctly wired, every future prompt promotion (Meta, Triage, Playback) in Phase 6.3.x and beyond is gated by `prompt-eval` passing on the full fixture — i.e. M8-class non-local effects are caught **before** the prompt swap goes live.

---

## 2. CLI surface

```
turingos llm prompt-eval \
  --workspace <PATH>                         # required: turingos.toml workspace
  --prompt-file <PATH>                       # required: candidate prompt asset (.md or system-prompt .txt)
  --role <meta|blackbox|playback>            # required: which role to score against
  --fixture <PATH>                           # required: JSONL fixture
  [--meta-prompt <PATH>]                     # optional: reference meta-prompt path (recorded; informational)
  [--baseline-prompt <PATH>]                 # optional: compute v1 vs v2 delta
  [--lang zh|en]                             # default: zh (error message language)
```

**Output**: one JSON line on stdout with shape:

```json
{
  "ok": true|false,
  "role": "blackbox",
  "prompt_file": "...",
  "fixture": "...",
  "total": 8,
  "pass": 5,
  "fail": 3,
  "error": 0,
  "fail_ids": ["s9_gibberish_t1_negctrl", "a9_gibberish_random_negctrl", "..."],
  "per_row": [
    {"id": "...", "verdict": "PASS|FAIL|ERROR", "role": "...", "expected": {...}, "actual": {...}, "notes": "..."},
    ...
  ],
  "baseline_delta": null
    | {"baseline_prompt_file": "...", "baseline_pass": 4, "baseline_fail": 4,
       "candidate_pass": 5, "candidate_fail": 3,
       "gained_ids": ["p5_codeswitch_t2_anchor", "s11_cantonese_t2_anchor"],
       "regressed_ids": ["s9_gibberish_t1_negctrl", "a9_gibberish_random_negctrl"]}
}
```

**Exit codes**:
- `0` — all rows passed AND no baseline regressions
- `1` — one or more rows FAILED, or baseline-delta has non-empty `regressed_ids`
- `2` — http/network error (missing API key, transport failure)
- `4` — io error (missing fixture / prompt / workspace)
- `5` — invalid CLI args

---

## 3. Fixture format (JSONL)

One row per test case. Comments allowed: blank lines and lines starting with `#` are skipped. Fields are the union of all three roles; per-role fields are validated lazily at scoring time.

### Blackbox triage row

```json
{
  "id": "p5_codeswitch_t2_anchor",
  "tags": ["codeswitch", "register", "triage", "f8_win"],
  "question": "你想用什么作为这个工具的 anchor？",
  "user_answer": "Anchor 就用 Jira issue key, 比如 PROJ-1234, 一个 sprint 大概 30-40 个 ticket",
  "expected_class": "relevant",
  "expected_confidence_min": 0.5,
  "notes": "F8 win: code-switch zh+en technical content must classify as relevant"
}
```

### Meta interviewer row

```json
{
  "id": "mrs_chen_t1_job",
  "tags": ["mrs_chen", "meta", "extraction"],
  "history": [],
  "user_answer": "我儿子放学想玩俄罗斯方块那种简单的小游戏",
  "expected_covered_slots_subset": ["job"],
  "expected_done": false,
  "expected_confidence_min": 0.05,
  "notes": "first turn must extract job slot"
}
```

### Playback row

```json
{
  "id": "p7_traditional_playback_no_hallucination",
  "tags": ["playback", "p7", "no_hallucination", "traditional"],
  "covered_slots_input": {
    "job": "影片轉檔工具，輸出 MP4",
    "anchor": "SHA256 加原檔名",
    "memory": "轉檔之後存到伺服器",
    "first_run": "拖曳上傳介面",
    "robustness": "檔案過大時降低解析度",
    "scope": "只做 MP4 不做 GIF",
    "acceptance": "轉檔速度 1分鐘內"
  },
  "expected_contains_substrings": ["影片", "SHA256"],
  "expected_no_substrings": ["YouTube", "highlights", "Instagram"],
  "notes": "must reflect Traditional video-transcoder job; MUST NOT hallucinate YouTube/Instagram"
}
```

---

## 4. Scoring logic per role

| Role | Verdict = PASS iff |
|------|--------------------|
| `blackbox` | `class == expected_class` AND `confidence ≥ expected_confidence_min`. Parse-failure → FAIL with `notes` = parse error. |
| `meta` | `expected_covered_slots_subset ⊆ covered_slots` AND `done == expected_done` (if specified) AND `confidence ≥ expected_confidence_min`. Tolerates ```json``` markdown fences around envelope. |
| `playback` | All `expected_contains_substrings` are present in raw output AND no `expected_no_substrings` are present (case-sensitive substring check on the full model output). |

All three roles apply `turingosv4::sdk::protocol::strip_think_blocks` first (same think-strip semantics as `complete` and `triage` per F2). Network/transport errors are caught and recorded as `verdict = "ERROR"` (counted as fail for exit-code purposes); the run does NOT abort partway through — every row is attempted.

### Baseline-delta computation

When `--baseline-prompt` is set, each fixture row is also evaluated against the baseline prompt with identical messages/model/budget. The summary's `baseline_delta` reports:
- `gained_ids` = candidate PASS ∖ baseline PASS (improvements)
- `regressed_ids` = baseline PASS ∖ candidate PASS (regressions — **the M8-class signal**)

If `regressed_ids` is non-empty, `ok = false` and exit code 1, even when the candidate's per-row pass count is higher than baseline. This is the core safety property: a prompt swap that gains on one register but regresses on another is NOT auto-promoted.

---

## 5. Starter fixture composition (8 rows)

| ID | Role | Tag set | Expected verdict on v1 triage | Expected verdict on v2 triage |
|----|------|---------|-------------------------------|-------------------------------|
| `mrs_chen_t1_job` | meta | mrs_chen, extraction, positive | n/a (meta-role) | n/a (meta-role) |
| `mrs_chen_t2_anchor` | meta | mrs_chen, extraction, positive | n/a (meta-role) | n/a (meta-role) |
| `p5_codeswitch_t2_anchor` | blackbox | codeswitch, register, **f8_win** | FAIL (v1 mis-classes as off_topic) | **PASS** (F8 win) |
| `s11_cantonese_t2_anchor` | blackbox | cantonese, register, **f8_win** | FAIL (v1 mis-classes) | **PASS** (F8 win) |
| `s9_gibberish_t1_negctrl` | blackbox | gibberish, **m8_regression** | PASS (v1 correctly gibberish) | **FAIL** (M8 regression — v2 mis-classes as relevant) |
| `a9_gibberish_random_negctrl` | blackbox | gibberish, **m8_regression** | PASS | **FAIL** (M8 regression) |
| `p7_traditional_playback_no_hallucination` | playback | playback, p7, no_hallucination | n/a (playback-role) | n/a (playback-role) |
| `mrs_chen_playback_no_hallucination` | playback | playback, no_hallucination | n/a (playback-role) | n/a (playback-role) |

The four `blackbox` rows form the **golden A/B**: v1 triage scores 2/4 (gibberish-correct, register-wrong), v2 triage scores 2/4 (gibberish-wrong, register-correct). Neither prompt passes the full fixture — that is the **point**: this fixture defines the bar that a future triage v3 must clear.

The two `meta` mrs_chen rows establish the unchanged-by-prompt-swap baseline (Meta-prompt isn't touched in F8). The two `playback` rows are the P7 hallucination-resistance check from W2.3.

---

## 6. Verification

### 6.1 Unit / CLI surface tests

```bash
cargo test --test cmd_llm_prompt_eval_stub --test cmd_llm_prompt_eval_v1_vs_v2_triage
```

Result on `codex/tisr-phase6-3-x-grill-driven` HEAD + this patch:

```
running 9 tests
test starter_fixture_covers_required_tag_categories ... ok
test starter_fixture_parses_as_valid_jsonl ... ok
test prompt_eval_unknown_flag_fails ... ok
test prompt_eval_without_args_fails_args_exit5 ... ok
test prompt_eval_unknown_role_fails ... ok
test prompt_eval_missing_workspace_fails ... ok
test help_lists_prompt_eval_action ... ok
test prompt_eval_invalid_lang_fails ... ok
test prompt_eval_missing_fixture_file_fails_io_exit4 ... ok

test result: ok. 9 passed; 0 failed; 0 ignored

running 5 tests
test prompt_eval_v2_catches_m8_gibberish_regression ... ignored
test triage_v1_and_v2_prompt_assets_both_exist ... ok
test fixture_contains_m8_regression_negative_controls ... ok
test fixture_contains_f8_register_positive_controls ... ok
test prompt_eval_against_triage_v2_smoke_args_only ... ok

test result: ok. 4 passed; 0 failed; 1 ignored
```

The `prompt_eval_v2_catches_m8_gibberish_regression` test is gated behind `#[ignore]` because it requires `SILICONFLOW_API_KEY` + network. The local A2 author did NOT have an API key in this session; the test should be executed manually (or by the orchestrator) with:

```bash
SILICONFLOW_API_KEY=sk-... cargo test --test cmd_llm_prompt_eval_v1_vs_v2_triage \
  -- --ignored --nocapture
```

When that test passes, the v2 fail_ids will include at least one of `s9_gibberish_t1_negctrl` / `a9_gibberish_random_negctrl` — proving the eval mechanism correctly catches the M8 regression.

### 6.2 CLI surface smoke (no API key required)

```
$ turingos llm prompt-eval
{"ok":false,"error":{"kind":"args","detail":"参数错误: --workspace is required"}}
$ echo $?
5

$ turingos llm prompt-eval --workspace /tmp --prompt-file /nonexistent --role meta --fixture /nonexistent
{"ok":false,"error":{"kind":"io","detail":"IO 错误: reading prompt file /nonexistent: No such file or directory (os error 2)"}}
$ echo $?
4

$ turingos llm | grep -A 2 prompt-eval
    turingos llm prompt-eval
                        --workspace <PATH>
                        --prompt-file <PATH>
--
    prompt-eval  Regression-test a candidate prompt against a frozen Q/A
              fixture. Iterates the fixture rows, calls the appropriate LLM
              role (meta|blackbox|playback), and scores each output against
```

### 6.3 Recommended manual A/B (orchestrator, with API key)

```bash
# v1 baseline
turingos llm prompt-eval --workspace tmp/universality_campaign \
  --prompt-file assets/prompts/grill_triage_blackbox_v1.md \
  --role blackbox \
  --fixture tests/fixtures/grill_prompt_eval_fixture.jsonl | tee v1_triage_eval.json

# v2 candidate (expect M8 regression to surface)
turingos llm prompt-eval --workspace tmp/universality_campaign \
  --prompt-file assets/prompts/grill_triage_blackbox_v2.md \
  --role blackbox \
  --fixture tests/fixtures/grill_prompt_eval_fixture.jsonl \
  --baseline-prompt assets/prompts/grill_triage_blackbox_v1.md | tee v2_triage_eval.json
```

Expected v1 output (excerpt):
```
"pass": 2, "fail": 2, "fail_ids": ["p5_codeswitch_t2_anchor", "s11_cantonese_t2_anchor"]
```

Expected v2 output (excerpt):
```
"pass": 2, "fail": 2,
"fail_ids": ["s9_gibberish_t1_negctrl", "a9_gibberish_random_negctrl"],
"baseline_delta": {
  "gained_ids":    ["p5_codeswitch_t2_anchor", "s11_cantonese_t2_anchor"],
  "regressed_ids": ["s9_gibberish_t1_negctrl", "a9_gibberish_random_negctrl"]
}
```

Both runs exit code 1 (neither prompt passes the full fixture). That is the **desired** outcome — the eval surface correctly refuses to bless an A/B-incomplete prompt, and the orchestrator must produce a v3 that wins both columns before any promotion.

---

## 7. Followup atoms gated on `prompt-eval`

Per A2 charter, future prompt-promotion atoms in Phase 6.3.x/y must gate on this surface:

- **A7** (Triage v3): pre-promotion gate = `prompt-eval --prompt-file grill_triage_blackbox_v3.md --role blackbox --fixture grill_prompt_eval_fixture.jsonl --baseline-prompt grill_triage_blackbox_v1.md` must return exit 0 AND `baseline_delta.regressed_ids == []`.
- **A8** (Playback v2): same gate, role=playback, baseline=playback v1 (not yet shipped — A8 will create it).
- **A11** (Meta v2 promotion, if proposed): same gate, role=meta, baseline=`grill_meta_v1.md`.

The fixture itself is a **living artifact**. Each fix or campaign that discovers a new failure mode adds a row tagged with the originating wave (e.g. `m9_codeswitch_v3`, `f12_traditional_v4`). Rows are NEVER removed — only superseded by tighter expected_* fields or marked `# DEPRECATED` with a comment line above.

---

## 8. Out of scope for A2

- No PromptCapsule write on eval rows (eval is read-only; capsules are for production turns only).
- No web endpoint (`/api/llm/prompt-eval`); A2 is CLI-only. A future atom can add a web surface if the dashboard wants live A/B results.
- No fixture authoring UI; the JSONL format is intentionally hand-editable.
- No support for non-SiliconFlow providers; A2 inherits `siliconflow_client::chat_complete` and its endpoint-override convention (`TURINGOS_SILICONFLOW_ENDPOINT`).
- No triage v3 in this atom; A7 will draft it using this fixture as the acceptance criterion.

---

## 9. R2 constraint compliance

- **§A1**: no `AttemptTelemetry` write for eval turns — eval is not a grill turn.
- **§A2**: no PromptCapsule write — eval is read-only, so the `hidden_fields_redacted` invariant is moot.
- **§A5**: Blackbox role uses `read_blackbox_model`; Meta/Playback use `read_meta_model`. Same model-selection path as `complete` and `triage`.
- **§A7**: meta-prompt path is recorded informationally (parsed but not currently emitted in summary; can be added in a follow-up).
- **§8 LOC cap**: 5000 (R2.2 amendment); this atom adds ~640 LOC to `cmd_llm.rs`, bringing it from ~1359 to ~2000 — well within cap.

No Class-4 surface touched. No `state/*`, no `kernel.rs`, no `Cargo.toml/lock`, no `genesis_payload.toml`, no `bottom_white/cas/schema.rs`. All edits confined to:
- `src/bin/turingos/cmd_llm.rs` (Class 2 production wire-up)
- `tests/fixtures/grill_prompt_eval_fixture.jsonl` (new — Class 1 additive)
- `tests/cmd_llm_prompt_eval_*.rs` (new — Class 1 additive)
- `handover/evidence/.../A2_prompt_eval_cli.md` (this doc — Class 0)
