# F4 — Web driven path drops meta-prompt from LLM messages array

- **Defect class**: Class-2 production wire-up defect, P0 / BLOCKING-FOR-SHIP
- **Branch**: `codex/tisr-phase6-3-x-grill-driven`
- **Pre-fix HEAD**: `3e0fa79cbf999fafbf56eae67da2960a063cf5d5`
- **Architect authorization**: 2026-05-18 autonomous campaign mandate
- **Risk class**: 2
- **Touched FC nodes**: FC1-N5 (read-view shielding at trust boundary) — same as F1/F3

## 1. Defect

The web driven mode is completely non-functional. The first `POST /api/spec/turn`
returns HTTP 500 `{"error":"llm complete failed: …","kind":"shellout_failed"}`.

Source: W9 real-LLM verifier verdict (clean-context Opus 4.7),
`handover/evidence/stage_phase6_3_x_grill_driven_1779111375/agent_verdict.json`,
step 3 verdict = `FAIL`, classification = `implementation_defect_blocks_web_path`.

The CLI driven mode (`turingos spec --mode driven`) at the same HEAD ran 12 turns
against real DeepSeek-V3.2, produced a session capsule + 23 turn capsules, and
exited clean. Only the web wire-up is broken.

## 2. Root-cause analysis

`src/web/spec.rs::build_web_turn_prompt_json` (old line ~1325) constructed the
LLM `messages` array as:

```
[
  {role: "system",  content: <coverage_summary>},
  // optional extra system nudge
  // last 3 turns
  {role: "user",    content: "Produce your turn-N output per the contract."},
]
```

The meta-prompt content (the interviewer contract that defines the role, the
8 canonical slots, the strict-JSON envelope schema, and the termination rules)
was **never included**. The function's doc comment justified this as:

> Simplified version (no meta-prompt file read; the shell-out to
> `turingos llm complete --meta-prompt` handles that server-side).

That assumption is wrong. `cmd_llm.rs` treats `--meta-prompt` as informational
only:

- `cmd_llm.rs:105` documents the flag as
  `--meta-prompt <PATH>          Meta-prompt asset path (informational; recorded in capsule).`
- `cmd_llm.rs:885-906` reads the file solely to compute its sha256, which is
  stored in the `PromptCapsule` as `system_prompt_template_hash`. The bytes are
  never spliced into the outgoing `chat_messages`.
- The `chat_messages` sent to the LLM (`cmd_llm.rs:693-722`) come exclusively
  from the prompt-file `messages` array on disk.

So the LLM received only the coverage summary + a one-line user instruction,
had no schema to follow, emitted free-form prose
(W9 verdict captured an example: `"Priya, a freelance graphic designer..."`),
and the strict-JSON parser raised `parse_failed`, which the web handler then
re-wrapped as `kind: "shellout_failed"`.

The CLI path at `src/bin/turingos/cmd_spec.rs::build_turn_prompt_json`
(lines 608-624) does the right thing: it prepends
`meta_prompt_content` as `messages[0]` with role `system`. The web path
**mirrored the structure but dropped this one step**.

### Why W10-R1 static audit missed it

The W10-R1 audit checked structural surfaces (does the function exist, does the
handler call it, does the shell-out pass `--meta-prompt`) but did not assert
**message-array completeness** against the CLI reference. A static parity check
of `build_web_turn_prompt_json` vs `build_turn_prompt_json` would have flagged
the missing `messages.push({role:"system", content: meta_prompt_content})` call
immediately. See §7 recommendation below.

## 3. Fix description

### 3a. `build_web_turn_prompt_json` signature now mirrors the CLI

Added `meta_prompt_content: &str` as the first parameter; prepended a
`role=system` message carrying that content as `messages[0]`. The remaining
message ordering (coverage summary → optional extra system → last-3-turns
pairs → final user instruction) is unchanged. The function-level doc comment
now records the F4 incident verbatim so the trap is documented at the call
site.

### 3b. Call site at the spec-turn handler now loads the asset

Where the previous code only constructed `meta_prompt_path` (informational, for
the `--meta-prompt` flag on the shell-out), the handler now also reads the file
via `tokio::task::spawn_blocking(std::fs::read_to_string)` and passes the bytes
through `build_web_turn_prompt_json`. Path resolution is unchanged
(`<workspace>/assets/prompts/grill_meta_v1.md`), matching the CLI default.

### 3c. Typed error for asset-missing

If `read_to_string` fails, the handler returns HTTP 500 with the new
`kind: "prompt_asset_missing"` rather than the misleading `shellout_failed`.
The error body taxonomy comment at the top of the module was updated to
document the new kind. This is a strictly-additive `kind` value (no client-side
contract change beyond a more precise signal); the `kind` field on `ErrorBody`
remains `Option<&'static str>` with the same JSON shape.

### 3d. Diff summary

- `src/web/spec.rs::build_web_turn_prompt_json`: added `meta_prompt_content`
  param + prepended system message at index 0 + rewrote doc comment to record
  F4 root cause
- `src/web/spec.rs::spec_turn_handler` (around old line 1088): inserted async
  `read_to_string` of the asset with typed-error fallback; updated the call to
  `build_web_turn_prompt_json` to pass the content
- `src/web/spec.rs` module-level error-body doc comment: added
  `prompt_asset_missing` to the `kind` taxonomy

## 4. Tests added (all in `src/web/spec.rs` `#[cfg(all(feature = "web", test))]`)

- **`web_spec_turn_prompt_includes_meta_prompt`** — constructs a synthetic
  meta-prompt + coverage summary, calls `build_web_turn_prompt_json`, parses
  the emitted JSON, asserts that the messages array contains at least one
  `role=system` message whose `content` equals the loaded meta-prompt
  verbatim. Pins the headline regression: prior to F4, the meta-prompt was
  not present anywhere in the array.

- **`web_spec_turn_prompt_first_message_is_system_with_meta`** — stronger
  ordering invariant: `messages[0].role == "system"`, `messages[0].content`
  starts with the canonical meta-prompt header and equals the input bytes
  exactly; `messages[1]` is the coverage summary system message;
  `messages.last()` is the user turn instruction referencing `turn-2`. Locks
  the canonical message order so future refactors cannot silently re-order.

- **`web_spec_turn_prompt_real_asset_loads_and_prepends`** — belt-and-braces:
  reads the actual production asset at `assets/prompts/grill_meta_v1.md`
  (CWD-relative — works in `cargo test` from repo root), asserts the asset
  contains `"TuringOS Spec Grill"`, then round-trips the asset bytes through
  `build_web_turn_prompt_json` and verifies `messages[0].content` equals the
  asset bytes exactly. Test is self-skipping (silent return) if the asset
  isn't readable from the current CWD, so it won't false-fail in odd CI
  contexts.

No new test fixture file was added — the third test reads the real production
asset directly, and the first two use inline string literals. This avoids
creating a parallel fixture that could drift out of sync with the canonical
asset.

## 5. Verification

### 5a. `cargo check --features web`

```
warning: `turingosv4` (bin "turingos_web") generated 16 warnings (run `cargo fix …` to apply 6 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.49s
```

Clean. The 16 warnings are pre-existing (variants `Empty`/`Partial` never
constructed, etc.) and unrelated to F4.

### 5b. `web::spec::tests` — all 17 pass, including 3 new F4 tests

```
test web::spec::tests::web_spec_turn_prompt_includes_meta_prompt ... ok
test web::spec::tests::web_spec_turn_prompt_first_message_is_system_with_meta ... ok
test web::spec::tests::web_spec_turn_prompt_real_asset_loads_and_prepends ... ok
…
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 41 filtered out
```

Includes F1's `extract_slots_*` tests and F3's `build_coverage_summary_*`
tests — both still green.

### 5c. `tests/web_spec_turn_endpoint.rs`

```
running 64 tests
test result: ok. 64 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### 5d. `tests/cli_web_spec_smoke.rs`

```
running 75 tests
test result: ok. 70 passed; 0 failed; 5 ignored; 0 measured; 0 filtered out
```

(5 ignored = pre-existing real-LLM gated tests, not regressions.)

### 5e. CLI grill regression tests

```
$ cargo test --test grill_envelope_parse --test cmd_llm_complete_stub
running 10 tests   …   test result: ok. 9 passed; 0 failed; 1 ignored
running 13 tests   …   test result: ok. 13 passed; 0 failed; 0 ignored
```

## 6. Judgment calls

1. **New error kind name**: chose `prompt_asset_missing` (mirrors the
   pre-existing `spec_md_missing` naming pattern) rather than overloading
   `shellout_failed` or inventing `io_error`. The frontend doesn't yet
   discriminate on `kind` for this path, but the W9 verifier explicitly noted
   that `shellout_failed` was a misleading signal — the new kind makes future
   triage immediate.

2. **No fixture file**: I considered adding
   `tests/fixtures/test_meta_prompt.md` but rejected it because (a) the third
   test already exercises the real asset path, (b) a fixture would create a
   parallel string that could drift, and (c) the first two tests' inline
   literals make the assertions self-contained and obvious.

3. **`tokio::task::spawn_blocking` for the read**: matches the surrounding
   handler's pattern (the `fs::write` for the prompt file is also wrapped this
   way) — keeps the async runtime unblocked even if the asset is on a slow
   FS.

4. **Function signature change vs. global**: I added `meta_prompt_content` as
   the first parameter (mirroring the CLI's ordering) rather than reading the
   file inside `build_web_turn_prompt_json`. This keeps the function pure
   (testable without I/O) and matches how the CLI structures the same code.

5. **Did not modify the `--meta-prompt` shell-out flag**: the flag is still
   passed (lines 1129-1130 unchanged) because `cmd_llm.rs` uses it for the
   `system_prompt_template_hash` field of the prompt capsule. Removing it
   would break capsule provenance.

## 7. Recommendation for `tests/fc_alignment_conformance.rs` (out of scope here)

The deeper structural defect is that the web and CLI paths can drift on
message-array shape without any test catching it. Recommend adding a
`tests/fc_alignment_conformance.rs` invariant:

> `build_web_turn_prompt_json(meta, cov, vec![], 1, None)` and
> `build_turn_prompt_json(meta, cov, &VecDeque::new(), 1, None)` must produce
> JSON whose `messages` arrays have identical (role, content) sequences.

This would catch F4-class defects (web path drops or adds messages) and would
also catch ordering drift (e.g. if one path moved coverage_summary ahead of
meta-prompt). Implementing it requires exposing `build_turn_prompt_json` from
the binary crate at least in `cfg(test)`, or extracting both functions into a
shared library module. Left for a follow-up TB because it touches the
CLI binary's module visibility, which is out of F4's allowed surface.

## 8. Surfaces touched

- `src/web/spec.rs` only (allowed per F1/F2/F3 precedent)

No Class-4 surfaces, no `Cargo.toml`/lock, no `genesis_payload`, no
kernel/bus/sdk/wallet, no `state/`, no `schema.rs`. No commit/push/checkout.

## 9. Cross-fix coordination

This fix landed on top of F1 (extract_slots vocab) and F3
(build_coverage_summary vocab), both already in the working tree at the
expected hunks. F2 (think-strip) lives in `cmd_llm.rs` and does not overlap.
No merge conflicts encountered.
