# F3 — `build_coverage_summary` vocabulary fix (Class 2)

- Date: 2026-05-18
- Branch: `codex/tisr-phase6-3-x-grill-driven`
- Pre-fix HEAD: `3e0fa79cbf999fafbf56eae67da2960a063cf5d5`
- Risk class: 2 (production wire-up, additive)
- Authorization: architect autonomous-campaign mandate 2026-05-18
- Files changed: `src/web/spec.rs` only
- Sibling fixes already landed in-place: F1 (`extract_slots`) + F2 (`<think>` strip in `cmd_llm`)

## 1. The defect

`src/web/spec.rs::build_coverage_summary` (pre-fix lines 1286–1315) carried the
**same junk slot vocabulary** that F1 just removed from `extract_slots`:

```text
job_story, anchor, data_model, first_click, weird_user,
disappointment_boundary, success_test, playback
```

The canonical vocabulary (defined in `src/runtime/grill_envelope.rs:17-26` and
used by the LLM's actually-emitted `covered_slots` envelope field, predicates
P3/P4, the termination predicate, and the `coverage_state` HashMap keys
populated by `spec_turn_handler`) is:

```text
job, anchor, memory, first_run, robustness, scope, acceptance, mirror
```

Severity is **strictly worse than F1**: where F1's `extract_slots` only fed the
WS broadcast (UI display), `build_coverage_summary` is on the **LLM prompt
assembly path** (called from `spec_turn_handler` at line ~1083, the result is
injected as a `system` message in the prompt JSON built by
`build_web_turn_prompt_json`). The `coverage_state` HashMap is keyed by
canonical names from `SlotState::Satisfied`/`Partial` inserts at line ~1187;
the function then iterated draft names and called `coverage_state.get("job_story")`,
which always returned `None`, so every slot rendered to the LLM as `[ ]`
regardless of actual coverage.

Symptoms in the LLM's behavior (per F1's chip-spawn report):

- redundant questions about slots it had just covered
- hallucinated coverage decisions
- never confidently emits `done=true` because its view of coverage is
  permanently empty
- wasted turns

This was correctly flagged in §6 of `F1_extract_slots_vocab.md` as a
"parallel P0 defect on a separate code path" and is the direct subject of F3.

## 2. The fix

Replaced the inline junk list with iteration over the already-imported
`CANONICAL_SLOTS` const. F1 already added the import at line 59:

```rust
use turingosv4::runtime::grill_envelope::CANONICAL_SLOTS;
```

Diff (essence):

```rust
 fn build_coverage_summary(
     coverage_state: &std::collections::HashMap<String, super::ws::SlotState>,
     turn_count: u32,
 ) -> String {
-    let slots = [
-        "job_story", "anchor", "data_model", "first_click",
-        "weird_user", "disappointment_boundary", "success_test", "playback",
-    ];
     let mut parts = Vec::new();
-    for slot in &slots {
+    for slot in CANONICAL_SLOTS {
         let mark = match coverage_state.get(*slot) {
             Some(super::ws::SlotState::Satisfied) => "[x]",
             Some(super::ws::SlotState::Partial) => "[~]",
             _ => "[ ]",
         };
         parts.push(format!("{mark} {slot}"));
     }
     ...
 }
```

A doc-comment block was added to the function explaining the defect, the fix,
the LLM-facing impact, and the choice of `CANONICAL_SLOTS` over `REQUIRED_SLOTS`
(see §5).

## 3. Tests added

Added to the existing `#[cfg(all(feature = "web", test))] mod tests` block in
`src/web/spec.rs` (the same block F1 added its 2 tests to):

### `build_coverage_summary_uses_canonical_vocab_not_draft`

Constructs a `coverage_state` HashMap with two canonical-keyed entries
(`{"job": Satisfied, "anchor": Partial}`), calls `build_coverage_summary` with
turn_count=3, and asserts:

- `[x] job` appears (canonical satisfied rendering)
- `[~] anchor` appears (canonical partial rendering)
- `[ ] memory`, `[ ] first_run`, `[ ] robustness`, `[ ] scope`,
  `[ ] acceptance`, `[ ] mirror` all appear (canonical uncovered rendering)
- header carries `turn 3`, trailer carries `Turns used: 3`
- every canonical slot id appears exactly once
- none of the 7 banned draft names (`job_story`, `data_model`, `first_click`,
  `weird_user`, `disappointment_boundary`, `success_test`, `playback`) appear
  anywhere in the output

This single assertion list would have caught the original bug on day 1.

### `build_coverage_summary_empty_state_lists_all_canonical_as_uncovered`

Boundary case. Empty coverage_state must yield all 8 canonical slots rendered
as `[ ]`, no `[x]` or `[~]` marks anywhere, and canonical order preserved
(`job` appears before `mirror`).

## 4. Verification

### `cargo check --features web`

```text
warning: ... (16 dead-code warnings, pre-existing) ...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.69s
```

Clean compile. No new warnings introduced by this fix.

### `web::spec` unit tests (`--bin turingos_web`)

```text
$ cargo test --features web --bin turingos_web web::spec --no-fail-fast
running 14 tests
test web::spec::tests::build_coverage_summary_empty_state_lists_all_canonical_as_uncovered ... ok
test web::spec::tests::build_coverage_summary_uses_canonical_vocab_not_draft ... ok
test web::spec::tests::extract_slots_empty_covered_returns_full_canonical_open ... ok
test web::spec::tests::extract_slots_uses_canonical_vocab_and_computes_open ... ok
test web::spec::tests::generate_session_id_format ... ok
test web::spec::tests::is_safe_session_id_accepts_valid ... ok
test web::spec::tests::is_safe_session_id_rejects_traversal ... ok
test web::spec::tests::parse_capsule_cid_from_stdout_finds_cid ... ok
test web::spec::tests::parse_capsule_cid_from_stdout_returns_none_on_no_match ... ok
test web::spec::tests::spec_questions_has_8_entries ... ok
test web::spec::tests::validate_answers_accepts_valid_8 ... ok
test web::spec::tests::validate_answers_rejects_empty_answer ... ok
test web::spec::tests::validate_answers_rejects_oversized_answer ... ok
test web::spec::tests::validate_answers_rejects_wrong_count ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 41 filtered out
```

Confirms: 12 pre-existing (incl. F1's 2 extract_slots tests) + 2 new F3 tests.
F1's `extract_slots_*` tests still pass.

### `web_spec_turn_endpoint` integration suite

```text
$ cargo test --features web --test web_spec_turn_endpoint --no-fail-fast
test result: ok. 67 passed; 0 failed; 5 ignored; 0 measured; 0 filtered out
```

(67 = the 65 F1 reported + the 2 new F3 unit tests, which this test binary also
picks up because it re-includes the `web` module.)

### `cli_web_spec_smoke` integration suite

```text
$ cargo test --features web --test cli_web_spec_smoke --no-fail-fast
test result: ok. 61 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

(61 = the 59 F1 reported + the 2 new F3 unit tests, same inclusion reason.)

## 5. Judgment calls

**Used `CANONICAL_SLOTS` (all 8) rather than `REQUIRED_SLOTS` (7).** The
function feeds the LLM's view of coverage. `mirror` is canonical but optional
per the termination predicate (the LLM may emit `done=true` without it). The
LLM still needs to *see* whether `mirror` has been covered so it can decide
whether to push for it on the next turn and so it knows the slot exists as a
valid emission target. Rendering all 8 is the conservative choice that mirrors
what the predicate vocabulary check (P3 `slots_in_vocab`) accepts. If a future
refactor wants to visually distinguish "required vs optional", it can render
all 8 with a marker prefix; that is out of F3 scope.

**Did NOT touch the function signature, the caller, or the prompt-template
contract.** The fix is the minimum delta that makes the existing
HashMap-keyed-by-canonical-names actually work as designed.

**Used `crate::web::SlotState` in the new tests** rather than `super::ws::SlotState`.
Inside the `mod tests` block, `super` resolves to the `spec` module, not the
`web` module, so `super::ws` does not exist. `crate::web::SlotState` works
because `web/mod.rs:31` re-exports it at the `web` module root and the bin
crate has `mod web;` at line 23 of `src/bin/turingos_web.rs`.

## 6. Side-effects

None. Only file modified is `src/web/spec.rs`. No restricted surface touched.
No commit, no push, no rebuild of the `turingos_web` binary. The W9 baseline
agent's binary inode is untouched (test binaries build to a separate target
path, which is what `cargo test` did here).

`git diff --stat src/web/spec.rs` (cumulative, includes F1):

```text
 src/web/spec.rs | 239 ++++++++++++++++++++++++++++++++++++++++++++++++++------
 1 file changed, 214 insertions(+), 25 deletions(-)
```

The F3-specific delta on top of F1: ~120 insertions, ~13 deletions
(~110 of those insertions are the two new regression tests + a ~20-line doc
comment on `build_coverage_summary`; the substantive prod-code change is
~5 lines: drop the inline array, iterate `CANONICAL_SLOTS`).

## 7. Workspace-wide pre-existing test failures

Same 15 unrelated failures F1 already noted, none caused by this fix and none
in the allowed-touch surface:

- `--lib boot::tests::verify_trust_root_passes_on_intact_repo` (1 failure;
  dirty-worktree precondition — untracked W9 evidence + audit files +
  this F3 evidence)
- `--lib bottom_white::cas::store::tests::*` (9 failures; CAS storage chain,
  independent of `src/web/`)
- `--lib runtime::evidence_capsule::tests::write_evidence_capsule_to_cas_round_trip`
  (1 failure; CAS, same precondition)
- `--test constitution_router_buy_with_coin` (4 failures; market/coin path,
  independent of `src/web/`)
- `--test fc_alignment_conformance fc3_n34_readonly_guard_verify_trust_root_intact_repo`
  (1 failure; trust-root scan, same dirty-worktree precondition)

All failures touch surfaces explicitly forbidden by F3's allowed-path list
(`src/bottom_white/cas/`, market/coin, trust-root, boot). They cannot be caused
by an edit confined to `src/web/spec.rs`.

## 8. Traceability

- Pre-fix HEAD: `3e0fa79cbf999fafbf56eae67da2960a063cf5d5`
- Branch (read-only): `codex/tisr-phase6-3-x-grill-driven`
- Did NOT commit, push, rebuild the `turingos_web` binary, or modify any
  restricted surface.
- W9 baseline run unaffected.
- F1 + F2 + F3 are now all in the same dirty-tree edit set on `src/web/spec.rs`
  (+ F2's `src/bin/turingos/cmd_llm.rs` + `tests/cmd_llm_strict_json_strip_think.rs`).
