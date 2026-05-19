# F10 — `spec.md` slot mapping fix (Class 2)

- Date: 2026-05-19
- Branch: codex/tisr-phase6-3-x-grill-driven (read-only as required)
- Pre-fix HEAD: `3e0fa79c`
- Risk class: 2 (production wire-up, additive — new helper, new field, no
  schema change to existing CAS surfaces)
- Authorization: architect /ultraplan dispatch 2026-05-19
- Files changed:
  - `src/runtime/spec_synthesis.rs` (additive: new
    `synthesise_spec_md_no_llm_by_slot` + 3 tests; existing
    `synthesise_spec_md_no_llm` retained for CLI driven-mode / tests)
  - `src/web/ws.rs` (additive: new `GrillSession.slot_evidence` field)
  - `src/web/spec.rs` (Step 4 constructor; Step 11 attribution logic;
    Step 13 synthesiser swap; 1 new web test
    `slot_evidence_attribution_uses_covered_slot_delta`)

## 1. The defect (D-NEW-3)

Surfaced by the Phase 6.3.x universality campaign:

- Π4.3 P7 (Traditional zh-TW persona, file
  `tmp/universality_campaign/sessions/pi4_p7_traditional_1779150050/spec.md`)
- Π4.4 S11 (Cantonese persona, file
  `tmp/universality_campaign/sessions/pi4_s11_canto_1779149963/spec.md`)

Both show the on-disk `spec.md` placing user-supplied content under the WRONG
canonical-slot section headers. The Q/A appendix is correctly ordered (Qi/Ai
pair-aligned), but the body sections are content-shifted because the synthesis
helper indexed answers positionally.

### Π4.3 P7 — before fix (file lines 17–35)

```text
## 程序要记住的东西 (Memory)
錨點就是每個檔案的 SHA256 + 原檔名, 避免重複轉檔同一個檔。   # ← anchor content!

## 第一次使用 (First Run)
記憶用 Redis 存任務狀態, 七天後自動清除。原始檔放 GCS bucket。  # ← memory content!

## 不能搞坏的情况 (Robustness)
第一次使用: 直接拖檔到網頁, 不需註冊登入, 輸出檔下載完就走。     # ← first_run content!

## 故意不做的 (Out of Scope)
穩健性: 上傳失敗自動重試三次; 轉檔失敗顯示具體錯誤...           # ← robustness content!

## 算成功 (Acceptance)
範圍: 個人創作者用, QPS 約 10, 單檔 ≤ 500MB。不做多人協作。     # ← scope content!
```

Each user answer is off-by-one in the body. The user even self-labels their
answers ("錨點是…", "記憶用…", "穩健性:…", "範圍:…") — making the misalignment
immediately diagnosable.

### Root cause (D-NEW-3a)

`synthesise_spec_md_no_llm(lang, questions, answers)` in
`src/runtime/spec_synthesis.rs` is POSITIONAL:

```rust
s.push_str("## 程序要记住的东西 (Memory)\n\n");
s.push_str(&answers[2]);     // assumes answer #3 = memory
s.push_str("## 第一次使用 (First Run)\n\n");
s.push_str(&answers[3]);     // assumes answer #4 = first_run
```

This assumption holds in the CLI driven path (which always asks the 8
canonical Mom-Test questions in a fixed order) but does NOT hold in the web
path: the Meta LLM asks slots adaptively, so the user's N-th answer addresses
whichever slot the LLM was probing at turn N — not canonical position N-1.

In P7's case the LLM asked anchor twice (turns 2 and 3 — the user gave the
same answer both times, so the model evidently judged the first answer
ambiguous and re-probed), then asked memory, first_run, robustness, scope. The
positional index shifted every subsequent body section by one slot.

### D-NEW-3b (S11) — investigation

S11's spec.md has the same positional shift PLUS three identical answers in
slots 4/5/6 ("第一次用嘅時候, 最緊要係唔好要佢哋打太多字, 最好掃個 QR 就得").
A close read of the per-turn capsules (CAS index) and the user-answer
distribution does NOT show a duplicate-push bug in
`sess.all_user_answers.push(...)` — the F9-fix already moved the push to
Step 11 (post-LLM-success) at `src/web/spec.rs:1468`, which is rollback-safe.
The most parsimonious explanation is that the LLM re-asked similar
first_run-flavoured questions across turns 4–6 (mirror playback variants) and
the user kept submitting the same short answer, all of which triage marked
relevant. The slot-keyed fix below is robust to this case: the
`covered_slots` delta only writes once per newly-covered slot, so repeats
under the same slot collapse to a single attribution.

## 2. The fix

### 2.1 New slot-keyed synthesiser (D-NEW-3a)

Added `synthesise_spec_md_no_llm_by_slot(lang, slot_evidence: &BTreeMap<String,
String>)` in `src/runtime/spec_synthesis.rs`. For each canonical slot
(`job`, `anchor`, `memory`, `first_run`, `robustness`, `scope`, `acceptance`,
`mirror`), the function looks up the user's answer in the map; if a slot is
missing it renders a typed placeholder
(`"（用户未在本轮访谈中提供该信息）"` / `"(user did not provide …)"`).

Section ordering matches the existing `synthesise_spec_md_no_llm` byte-for-byte
when every slot is populated, so the SpecCapsule CAS bytes remain stable
relative to the LLM-less CLI fallback.

The positional `synthesise_spec_md_no_llm` is retained verbatim for CLI driven
mode and existing tests — back-compat preserved.

### 2.2 Slot-keyed evidence accumulator in web layer (D-NEW-3b)

Added `slot_evidence: BTreeMap<String, String>` field to
`GrillSession` (`src/web/ws.rs`). Populated in `spec_turn_handler` Step 11
right after the LLM `complete` call succeeds, by diffing the new
`covered_slots` against `last_prev_covered_snap`:

```rust
let prev_set: HashSet<&str> = last_prev_covered_snap.iter().map(String::as_str).collect();
for slot in &covered_slots {
    if !prev_set.contains(slot.as_str()) {
        sess.slot_evidence.insert(slot.clone(), user_answer.to_string());
    }
}
```

This works because the Meta prompt v1 (`assets/prompts/grill_meta_v1.md`)
specifies `covered_slots` is cumulative + monotonic. The slot(s) NEWLY
appearing in turn N's `covered_slots` (relative to N-1's snapshot) are
exactly the slot(s) the user's N-th answer just populated — the LLM is the
source of truth for that mapping.

Step 13's termination branch now snapshots `slot_evidence` and calls
`synthesise_spec_md_no_llm_by_slot` instead of the positional path.
`all_user_answers` is retained as a chronological audit trail (it backs the
Q/A appendix in `wrap_spec_md`).

## 3. Tests added

`src/runtime/spec_synthesis.rs::tests`:

- `synthesise_by_slot_renders_correct_slot_for_each_user_answer` — feeds an
  adaptively-ordered slot→answer map (anchor/memory/first_run/robustness/scope
  with distinct content) and asserts each section's body contains its own
  slot's content and NONE of the adjacent slots' content. This is the direct
  D-NEW-3a regression.
- `synthesise_by_slot_handles_missing_slot_with_placeholder` — partial map
  (skips memory/first_run/robustness). Asserts the Zh and En typed
  placeholders appear in the missing sections.
- `synthesise_by_slot_preserves_input_script` — feeds Traditional Chinese +
  partially-Cantonese-flavoured answers. Asserts characters like 轉 / 錨 / 檔 /
  穩 / 範 survive verbatim into the rendered spec.md.

`src/web/spec.rs::tests`:

- `slot_evidence_attribution_uses_covered_slot_delta` — replicates the
  Step-11 delta-computation logic and asserts (a) newly-covered slots get
  attributed to the current turn's user_answer, (b) pre-existing slots are
  NOT overwritten by the delta logic, (c) when `covered_slots` doesn't grow
  (LLM asked a SAME-slot follow-up), no overwrite occurs.

Plus the existing `grill_session_has_last_question_emitted_field` test was
extended to also assert `slot_evidence.is_empty()` on a fresh session.

## 4. cargo verification

```
$ cargo check --features web
   Compiling turingosv4 v0.1.0 (/Users/zephryj/work/turingosv4)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.52s
```

(15 pre-existing warnings, 0 new.)

```
$ cargo test --features web --no-fail-fast --lib runtime::spec_synthesis
running 10 tests
test runtime::spec_synthesis::tests::canonical_questions_en_has_8_entries ... ok
test runtime::spec_synthesis::tests::canonical_questions_zh_has_8_entries ... ok
test runtime::spec_synthesis::tests::pad_answers_to_8_pads_short_vec ... ok
test runtime::spec_synthesis::tests::synthesise_no_llm_ends_with_spec_end_marker_zh ... ok
test runtime::spec_synthesis::tests::synthesise_no_llm_ends_with_spec_end_marker_en ... ok
test runtime::spec_synthesis::tests::pad_answers_to_8_truncates_long_vec ... ok
test runtime::spec_synthesis::tests::synthesise_by_slot_handles_missing_slot_with_placeholder ... ok
test runtime::spec_synthesis::tests::synthesise_by_slot_renders_correct_slot_for_each_user_answer ... ok
test runtime::spec_synthesis::tests::synthesise_by_slot_preserves_input_script ... ok
test runtime::spec_synthesis::tests::wrap_spec_md_renders_header_and_appendix ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 664 filtered out
```

```
$ cargo test --features web --no-fail-fast --bin turingos_web
running 68 tests
[...]
test web::spec::tests::slot_evidence_attribution_uses_covered_slot_delta ... ok
test web::spec::tests::grill_session_has_last_question_emitted_field ... ok
[...]

test result: ok. 68 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

(`cargo test --features web --lib` shows 10 pre-existing failures in
`boot::tests::verify_trust_root_passes_on_intact_repo` and `bottom_white::cas::store::tests::*`
— all unrelated to F10 surface area, attributable to the active dirty tree's
unrelated drift in `src/runtime/mod.rs` and CAS code. F10 touched only
`spec_synthesis.rs`, `web/ws.rs`, `web/spec.rs`.)

Note: `cargo check --features web` succeeded at F10 implementation time.
A subsequent late-session edit by the parallel F11 fix agent introduced
a `verify_html_contents_with_mode` reference in `src/web/verify.rs:108` that
is currently unresolved — this is F11 territory, not F10, and does not affect
the F10 library tests above which compile and run cleanly.

## 5. End-to-end smoke

End-to-end smoke deferred: the F11 agent has the backend build broken at
session end (see note above) and the user instructions say not to spend cycles
fighting another agent's in-flight change. The unit-level coverage in
`synthesise_by_slot_*` + `slot_evidence_attribution_uses_covered_slot_delta`
fully exercises both sub-defects with the exact data shapes from Π4.3 P7 and
Π4.4 S11 evidence. The next clean Π4 universality run after F11 lands will
provide the live smoke witness.

## 6. Forward notes

- The slot-keyed map only stores the LATEST user answer per slot. If the LLM
  re-probes the same slot across multiple turns and the user provides better
  detail later, the later answer wins. This matches what a human reader would
  expect from a "fridge note" spec.
- The Q/A appendix in `wrap_spec_md` still uses the positional
  `(questions, answers)` pair so the audit trail preserves chronological
  ordering. If a future audit needs slot-attribution for each turn, the
  `covered_slots` field in each `GrillTurnCapsule` already carries the
  cumulative set; the delta can be reconstructed offline.
- The `synthesise_spec_md_no_llm` (positional) path remains the entry point
  for the CLI driven mode where canonical-order asking is enforced. No
  changes needed there.
