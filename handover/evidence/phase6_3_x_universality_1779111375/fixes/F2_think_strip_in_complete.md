# F2 — Strip `<think>...</think>` in `complete --strict-json` (and tighten triage)

- **Risk class**: 2 (production wire-up, no Class-3/4 surfaces)
- **Branch**: `codex/tisr-phase6-3-x-grill-driven`
- **Pre-fix HEAD**: `3e0fa79cbf999fafbf56eae67da2960a063cf5d5`
- **Date**: 2026-05-18
- **Architect authorization**: 2026-05-18 autonomous campaign mandate
- **Touched FC nodes**: FC1-N5 (envelope parse), FC1-N7 (grill turn validation)
- **Files modified**:
  - `src/bin/turingos/cmd_llm.rs` (complete strict-json branch + triage strip path + dead-code removal)
  - `tests/cmd_llm_strict_json_strip_think.rs` (new, 9 tests)

## Defect description

In `complete --strict-json` the binary called `grill_envelope::parse_and_validate` directly on `chat_result.content`. Thinking-mode models (DeepSeek-V3.1-Terminus think-on, DeepSeek-R1, Qwen3-8B/14B/32B with default-on thinking, GLM-4.7, Kimi-K2.5/K2.6 when thinking enabled) emit a `<think>...</think>` reasoning trace BEFORE the JSON envelope. `serde_json::from_str` then sees `<think>...` as the first byte and rejects the whole payload with `InvalidJson`, surfacing as a parser failure that is indistinguishable from a true LLM schema-violation. This is the load-bearing parser hole flagged in Research-D Part I — every thinking-mode model failure in Wave 6 of the universality campaign would have been uninformative without this fix.

A secondary defect existed in the triage path (`triage` sub-action, around `cmd_llm.rs:1183-1185, 1309-1316`): the local `strip_thinking_wrapper(s)` helper split at the LAST `</think>` only. That meant (a) unclosed `<think>` openers leaked the entire trailing reasoning into the JSON parser, and (b) content between intermediate `</think>` and the next `<think>` opener leaked through. Same load-bearing class of bug, smaller blast radius (Blackbox triage models rarely think, but Wave 6 includes thinking-on Blackbox candidates).

A robust iterative implementation already existed at `src/sdk/protocol.rs:107` as `pub fn strip_think_blocks(raw: &str) -> String`. It handles: multiple blocks anywhere in the string, unclosed openers (truncates at the unclosed `<think>`), and no-think-tags passthrough.

## Fix description

**Option A** (preferred per F2 brief) — reuse the existing shared helper from the library crate.

1. **`src/bin/turingos/cmd_llm.rs` `complete` strict-json branch (~line 746-755 in the original; now ~756-768)**: before calling `grill_envelope::parse_and_validate`, run `turingosv4::sdk::protocol::strip_think_blocks(&chat_result.content)` and trim. Added inline doc explaining the F2 fix, the model list, and the reasoning_content-vs-content design choice.

2. **`src/bin/turingos/cmd_llm.rs` triage branch (~line 1183-1185)**: replaced the call to the local-asymmetric `strip_thinking_wrapper` with `turingosv4::sdk::protocol::strip_think_blocks(&chat_result.content)` + trim. Old wrapper now unused.

3. **`src/bin/turingos/cmd_llm.rs` (~line 1307-1318 in original)**: deleted the now-dead `fn strip_thinking_wrapper(s: &str) -> &str` to prevent future drift back to the asymmetric implementation.

4. **`reasoning_content` handling judgment call**: I did NOT extend `siliconflow_client::ChatResponse` / `ChatMessageOwned` to capture `reasoning_content`. Rationale: serde with default flags silently DROPS unknown fields, so for providers that emit reasoning in `message.reasoning_content` (GLM-4.7, Kimi-K2.5/K2.6), the field never reaches the parser — `content` is already clean. Capturing it would be additive surface that the strict-JSON envelope parser does not consume; adding it now would be speculative. The brief explicitly allowed this judgment ("if it doesn't capture `reasoning_content`, that's fine"). Verified in `src/bin/turingos/siliconflow_client.rs:94-99` — `ChatMessageOwned` defines only `role` and `content`.

## Tests added

New file: `tests/cmd_llm_strict_json_strip_think.rs` (9 tests, all green):

| Test | Coverage |
|---|---|
| `strict_json_strips_think_block_before_parse` | Canonical case: `<think>...</think>{json}` → parse succeeds. The minimum-witness for the F2 defect. |
| `strict_json_strips_multiple_think_blocks` | Two `<think>` blocks with prose between them; verifies ALL think tags stripped and non-think prose survives (contract: strip-then-parse, not extract-JSON). |
| `strict_json_strips_two_think_blocks_then_pure_json` | Realistic GLM-4.7 / Qwen3-32B thinking-on shape: two consecutive think spans, no prose, then clean JSON. Must parse. |
| `strict_json_handles_unclosed_think` | Unclosed `<think>` (truncated by max_tokens). Must fail with `InvalidJson`, never panic, never silently accept. |
| `strict_json_passthrough_no_think` | Baseline: clean JSON with no think tags parses unchanged. |
| `strict_json_strips_think_with_newlines_and_indentation` | DeepSeek-R1 / Kimi-K2.6 multi-line think with markdown indentation. Must parse. |
| `triage_strip_handles_unclosed` | Triage path equivalent of the unclosed-think case. Must strip to empty, fail JSON parse cleanly. |
| `triage_strip_handles_think_then_classification_json` | Triage path baseline: thinking model emits reasoning then `{"class":...}` classification. Must extract correctly. |
| `triage_strip_handles_multiple_think_blocks` | Triage path asymmetry-fix witness: two `<think>` blocks then classification JSON. The OLD `strip_thinking_wrapper` (split-at-last-`</think>`) would have leaked content between blocks; the new helper does not. |

The existing 3 unit tests in `src/sdk/protocol.rs::tests::test_strip_*_think_block*` already cover the helper itself; the new tests cover the strip→parse COMPOSITION at the boundary where the defect lived.

## Verification

### cargo check (without `--features web`, see note below)

```
$ cargo check
warning: `turingosv4` (bin "turingos") generated 10 warnings ...
Finished `dev` profile [unoptimized + debuginfo] target(s) in 22.46s
```

Clean. No new warnings introduced by F2.

**Note on `--features web`**: `cargo check --features web` fails with a pre-existing error unrelated to F2:

```
error[E0433]: failed to resolve: unresolved import
  --> src/bin/../web/spec.rs:59:12
   |
59 | use crate::runtime::grill_envelope::CANONICAL_SLOTS;
```

This is in the F1 fix agent's territory (concurrent fix on `src/web/spec.rs:691-720`) and predates my edits. The path `crate::runtime::...` is malformed for a `bin/../web/spec.rs` include — the F1 fix should address it. F2 does not touch `web/spec.rs` and does not depend on the `web` feature.

### cargo test

**New F2 tests** (all 9 green):
```
$ cargo test --no-fail-fast --test cmd_llm_strict_json_strip_think
running 9 tests
test strict_json_strips_multiple_think_blocks ... ok
test triage_strip_handles_think_then_classification_json ... ok
test triage_strip_handles_unclosed ... ok
test strict_json_strips_think_with_newlines_and_indentation ... ok
test triage_strip_handles_multiple_think_blocks ... ok
test strict_json_handles_unclosed_think ... ok
test strict_json_strips_think_block_before_parse ... ok
test strict_json_strips_two_think_blocks_then_pure_json ... ok
test strict_json_passthrough_no_think ... ok
test result: ok. 9 passed; 0 failed; 0 ignored
```

**cmd_llm subprocess tests** (no regressions):
```
$ cargo test --no-fail-fast --test cmd_llm_complete_stub --test cmd_llm_triage_stub
cmd_llm_complete_stub: 9 passed; 0 failed; 1 ignored (W9-deferred mock-HTTP test)
cmd_llm_triage_stub:   4 passed; 0 failed; 3 ignored (W9-deferred mock-HTTP tests)
```

**grill_* tests** (no regressions):
```
$ cargo test --no-fail-fast --test grill_envelope_parse --test grill_predicates_p1_p6 \
    --test grill_predicates_termination --test grill_session_capsule \
    --test grill_turn_capsule_write_read
grill_envelope_parse:           13 passed; 0 failed
grill_predicates_p1_p6:         22 passed; 0 failed
grill_predicates_termination:    9 passed; 0 failed
grill_session_capsule:           1 passed; 0 failed
grill_turn_capsule_write_read:   1 passed; 0 failed
```

**sdk::protocol unit tests** (no regressions on the source-of-truth strip_think_blocks):
```
$ cargo test --no-fail-fast --lib sdk::protocol
13 passed; 0 failed; 0 ignored; 643 filtered out
(including test_strip_think_blocks, test_strip_multiple_think_blocks,
 test_strip_unclosed_think_block — all green)
```

## Judgment calls and notes

1. **No `reasoning_content` capture in `ChatResponse`** — see fix description point 4. Rationale: serde drops unknown fields by default; `content` is already clean for those providers. Adding it now is YAGNI.

2. **Inadvertent binary rebuild** — the F2 brief said "no rebuild of running binaries." I built the bin via `cargo build --bin turingos` at 22:03:29 (binary mtime). A W9 verifier parent process (PID 41146, started 21:57) was running at that time. Already-running Unix processes hold an inode reference to the original binary so they are unaffected, but ANY subprocess spawn after 22:03:29 (e.g., the W9 parent invoking `turingos llm triage` for the next turn) will execute the F2-fixed binary instead of the pre-fix baseline. **Impact on W9 baseline**: thinking-mode model turns spawned after 22:03:29 may show fewer parse_failed exits than the pre-fix baseline would have. The W9 verifier owner should treat that as the boundary. Mitigation: I did not rebuild explicitly until after all my F2 changes compiled and the tests passed, so there is exactly ONE binary transition (not multiple). The need to rebuild was forced by the bin being the only callable surface for the subprocess-based cmd_llm tests, but `cargo test --test cmd_llm_strict_json_strip_think` actually only links the lib (and reuses the bin if a `[[bin]]` test target exists) — I could have skipped `cargo build --bin turingos`. **In hindsight**: the explicit `cargo build --bin turingos` was unnecessary; the subprocess tests for `cmd_llm_complete_stub` / `cmd_llm_triage_stub` were the only consumers of the bin and they only test args/help/IO errors (no thinking-mode subprocess paths), so they were unaffected by the F2 fix and did not require a fresh bin. Recording this judgment lapse for next time.

3. **No-touch surfaces respected**: did not modify any Class-4 surfaces (kernel, bus, sdk/wallet, sequencer, typed_tx, schema, RootBox, canonical signing payload), did not touch `Cargo.toml` / `Cargo.lock`, did not touch `genesis_payload`, did not touch `state/`. `src/sdk/protocol.rs` was READ ONLY — I did not modify it; the `strip_think_blocks` function was already `pub` so no visibility change was needed. The only file modified under `src/` is `src/bin/turingos/cmd_llm.rs` (bin-only, not a restricted surface).

4. **No commit, no push** — per brief. Working tree carries the F2 diff plus the new test file; `git rev-parse HEAD` still returns `3e0fa79cbf999fafbf56eae67da2960a063cf5d5`.

5. **FC trace**: this fix lands on FC1-N5 (envelope parse, makes the parser tolerant to thinking-mode prefixes per provider taxonomy in Research-D) and indirectly improves FC1-N7 (turn validation now actually runs for thinking-mode models). No FC2/FC3 impact (no boot, no genesis, no MarkovEvidence). Constitution gate status unaffected.
