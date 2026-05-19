# F1 — `extract_slots` vocabulary fix (Class 2)

- Date: 2026-05-18
- Branch: `codex/tisr-phase6-3-x-grill-driven`
- Pre-fix HEAD: `3e0fa79cbf999fafbf56eae67da2960a063cf5d5`
- Risk class: 2 (production wire-up, additive)
- Authorization: architect autonomous-campaign mandate 2026-05-18
- Files changed: `src/web/spec.rs` only

## 1. The defect

`src/web/spec.rs::extract_slots` (pre-fix lines 691–720) returned the `open_slots`
vector that is broadcast in every web session's `SpecTurnAdvanced` /
`SpecTurnResponse`. It computed `open = all_slots - covered`, but `all_slots`
was hardcoded to Researcher-C's draft vocabulary:

```text
job_story, anchor, data_model, first_click, weird_user,
disappointment_boundary, success_test, playback
```

The canonical slot vocabulary (the names the LLM actually emits, defined in
`src/runtime/grill_envelope.rs:17-26` and consumed by `parse_and_validate`,
predicates P3/P4, termination predicate, etc.) is:

```text
job, anchor, memory, first_run, robustness, scope, acceptance, mirror
```

Because `covered_slots` arrives from the LLM envelope using canonical names,
the set difference always degenerated to the full junk list (with "anchor" as
the only accidental overlap). Effect: the frontend's "still missing X, Y, Z"
hints and the WS-side progress bar were always wrong. The LLM-facing
`coverage_summary` is built by `build_coverage_summary` (not by
`extract_slots`), so the LLM itself was unaffected by this defect — only the
browser projection was broken.

## 2. The fix

Replaced the inline junk list with an import of the canonical const.

Diff (extract):

```rust
+#[cfg(feature = "web")]
+use turingosv4::runtime::grill_envelope::CANONICAL_SLOTS;
...
 fn extract_slots(envelope: &serde_json::Value) -> (Vec<String>, Vec<String>) {
-    // CANONICAL_SLOTS from grill_envelope (8 slots)
-    let all_slots = [
-        "job_story", "anchor", "data_model", "first_click",
-        "weird_user", "disappointment_boundary", "success_test", "playback",
-    ];
+    // Vocabulary must mirror the canonical grill substrate; importing
+    // CANONICAL_SLOTS keeps the dependency explicit and tracks the source
+    // forever. The previous inline list used Researcher-C's draft vocabulary
+    // (job_story/data_model/first_click/...) which never matched what the LLM
+    // actually emits (job/memory/first_run/...). Result: the WS broadcast's
+    // open_slots field was always wrong, breaking the frontend progress hint.
     let covered: Vec<String> = envelope
         .get("covered_slots")
         ...
         .unwrap_or_default();

-    let open: Vec<String> = all_slots
+    let open: Vec<String> = CANONICAL_SLOTS
         .iter()
         .filter(|s| !covered.iter().any(|c| c.as_str() == **s))
-        .map(|s| s.to_string())
+        .map(|s| (*s).to_string())
         .collect();
     (covered, open)
 }
```

Note on the import path: `src/web/spec.rs` is included via `#[path =
"../web/mod.rs"]` only from `src/bin/turingos_web.rs`; it is not in
`src/lib.rs`. Therefore `crate::` inside this file resolves to the
`turingos_web` bin crate, not the library, and the canonical const must be
referenced as `turingosv4::runtime::grill_envelope::CANONICAL_SLOTS`.

The brief listed inline duplication with a comment as the acceptable fallback;
that fallback was not needed because the library-crate import resolves cleanly
under `--features web`.

## 3. The test added

Added to the existing `#[cfg(all(feature = "web", test))] mod tests` block at
the bottom of `src/web/spec.rs`:

- `web::spec::tests::extract_slots_uses_canonical_vocab_and_computes_open`
- `web::spec::tests::extract_slots_empty_covered_returns_full_canonical_open`

The first test feeds `covered_slots: ["job","anchor","memory"]`, asserts the
returned `open` set equals exactly `{first_run, robustness, scope, acceptance,
mirror}` (5 items), asserts every returned name is in `CANONICAL_SLOTS`, and
asserts none of the 7 banned draft names ever appear. This is the kind of
test that would have caught the original bug on day 1.

The second test pins the boundary case: empty covered → open equals the full
canonical 8-set.

## 4. Verification

### `cargo check --features web`

```text
warning: ... (16 dead-code warnings, pre-existing) ...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.67s
```

Clean compile. No new warnings introduced by this fix.

### Targeted regression tests

```text
$ cargo test --features web --bin turingos_web extract_slots
running 2 tests
test web::spec::tests::extract_slots_empty_covered_returns_full_canonical_open ... ok
test web::spec::tests::extract_slots_uses_canonical_vocab_and_computes_open ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 51 filtered out
```

### All `web::spec` unit tests

```text
$ cargo test --features web --bin turingos_web web::spec --no-fail-fast
running 12 tests
test web::spec::tests::extract_slots_empty_covered_returns_full_canonical_open ... ok
test web::spec::tests::extract_slots_uses_canonical_vocab_and_computes_open ... ok
test web::spec::tests::generate_session_id_format ... ok
test web::spec::tests::is_safe_session_id_rejects_traversal ... ok
test web::spec::tests::is_safe_session_id_accepts_valid ... ok
test web::spec::tests::parse_capsule_cid_from_stdout_finds_cid ... ok
test web::spec::tests::parse_capsule_cid_from_stdout_returns_none_on_no_match ... ok
test web::spec::tests::validate_answers_accepts_valid_8 ... ok
test web::spec::tests::spec_questions_has_8_entries ... ok
test web::spec::tests::validate_answers_rejects_empty_answer ... ok
test web::spec::tests::validate_answers_rejects_oversized_answer ... ok
test web::spec::tests::validate_answers_rejects_wrong_count ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 41 filtered out
```

### `web_spec_turn_endpoint` integration suite

```text
$ cargo test --features web --test web_spec_turn_endpoint --no-fail-fast
test result: ok. 65 passed; 0 failed; 5 ignored; 0 measured; 0 filtered out
```

(5 ignored tests require live binary / network — not run.)

### `cli_web_spec_smoke` integration suite

```text
$ cargo test --features web --test cli_web_spec_smoke --no-fail-fast
test result: ok. 59 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Full `cargo test --features web --no-fail-fast`

645 lib unit tests + many integration tests pass. 15 unrelated failures across
3 targets, none caused by this fix:

- `--lib boot::tests::verify_trust_root_passes_on_intact_repo` (1 failure;
  trust-root scan fails on dirty worktree — known precondition, the worktree
  has untracked W9 evidence and audit files).
- `--lib bottom_white::cas::store::tests::*` (9 failures; CAS storage chain
  tests — completely independent of `src/web/`).
- `--lib runtime::evidence_capsule::tests::write_evidence_capsule_to_cas_round_trip`
  (1 failure; uses CAS, same precondition).
- `--test constitution_router_buy_with_coin` (4 failures; market/coin path,
  independent of `src/web/`).
- `--test fc_alignment_conformance fc3_n34_readonly_guard_verify_trust_root_intact_repo`
  (1 failure; trust-root check, same precondition as boot test above).

All failures touch surfaces explicitly forbidden by this fix's allowed-path
list (`src/bottom_white/cas/`, market/coin, trust-root). They cannot be caused
by an edit confined to `src/web/spec.rs`. They are the W9-baseline / dirty-tree
precondition the campaign is already aware of.

## 5. Side-effects

None. The only file modified is `src/web/spec.rs`. No restricted surface was
touched (see allowed-path list in §6 of the dispatch brief; this fix touched
only the additive web wrapper).

`git diff --stat src/web/spec.rs`:

```text
 src/web/spec.rs | 103 ++++++++++++++++++++++++++++++++++++++++++++++++--------
 1 file changed, 89 insertions(+), 14 deletions(-)
```

(Of the 89 insertions, 65 are the new regression-test block; 8 are the
explanatory comment; the substantive prod-code delta is ~15 lines.)

## 6. Open observation (out of fix scope)

While reading the file I noticed `build_coverage_summary` at
`src/web/spec.rs:1286-1315` carries an identical inline junk-vocab list. That
function is on the LLM-facing path: it iterates the junk names and looks them
up in `coverage_state` (which is populated using canonical names from the
envelope), so every slot in the rendered coverage block always shows `[ ]` to
the LLM regardless of actual coverage. The brief's "LLM-facing flow is
unaffected" assertion may be a misread of which function builds the
coverage_summary — this looks like a parallel P0 defect on a separate code
path.

Per the fix brief, this is out of the F1 scope (the brief named only
`extract_slots`). It is being flagged via the spawned-task chip so the
architect can dispatch a sibling fix (F2) or defer.

## 7. Traceability

- Pre-fix HEAD: `3e0fa79cbf999fafbf56eae67da2960a063cf5d5`
- Branch (read-only): `codex/tisr-phase6-3-x-grill-driven`
- Did NOT commit, push, rebuild binary, or modify any restricted surface.
- W9 baseline run unaffected (uses its own already-loaded binary).
