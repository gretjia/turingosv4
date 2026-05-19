# F6 Fix Report — Backend Error-Handling Cluster (Symptoms A/B/C)

**Status**: Done. Agent was killed mid-writeup at the integration-test scaffolding step (assistant-message truncated to `output_tokens: 1`, no error). Orchestrator inspected the worktree, confirmed all functional edits + tests landed, finished verification.

**Pre-fix HEAD**: `3e0fa79cbf999fafbf56eae67da2960a063cf5d5` (unchanged; F6 edits uncommitted on `codex/tisr-phase6-3-x-grill-driven`).

## Sources (defect grounding)

- `handover/evidence/phase6_3_x_universality_1779111375/wave1/mrs_chen/session_log.jsonl` — turns 2, 6, 7 show HTTP 200 with all-zero envelope, ~1s elapsed (vs 8-10s normal)
- `handover/evidence/phase6_3_x_universality_1779111375/wave1/p1_backend/verdict.json` — 5 shellout failures of 12; spec_capsule_cid null on terminate
- `handover/evidence/stage_phase6_3_x_grill_driven_1779111375/agent_verdict.json` — W9 CLI driven mode 23 turn capsules for 12 turns (high retry rate)

## Symptom → Fix mapping

### Symptom A — HTTP 200 with empty fields when LLM subprocess fails

**Root cause** (line 674-720, `parse_turn_payload_from_llm_output`): pre-F6 version did NOT check the `ok` flag in the stdout JSON. When the subprocess emitted `{"ok": false, "content": "", ...}`, the parser returned an empty envelope (`Ok(Value::Null)`) rather than `Err(...)`. The caller then composed a response from `envelope.get("question").or_default()` chains, all yielding empty strings / empty arrays / zeros.

**Fix**: parse the `ok` flag first; on `ok=false` return `Err` with descriptive message. The downstream caller at line 1182-1191 (existing code) maps `Err` → HTTP 500 with `kind: "shellout_failed"`. Now the failure is loud, not silent.

### Symptom B — turn_index doesn't advance on triage-reject paths

**Root cause** (lines 1095-1207): when triage classified user_answer as non-relevant, the handler returned `turn_index: turn_count + 1` despite NO Meta turn having happened. The next genuine submission would see the client send back `user_answer` for what it thought was turn N+1, but the server's `turn_count` was still N, so the server's response would be turn_index=N+1 again — appearing stuck.

**Fix**: on triage-reject return paths (lines 1166, 1196), set `turn_index: turn_count` (preserve), not `turn_count + 1`. Document the smoking-gun evidence in the F6 comment block (W1.1 mrs_chen turn-2 1035ms response).

### Symptom C — terminated=true with spec_capsule_cid=null and no diagnostic

**Root cause cluster**:
1. **Broken shellout** (line 1124+): pre-F6 abort path invoked `turingos spec --workspace ... --session ... --mode driven --synthesize-only --termination-reason user_input_unparseable`. None of `--session`, `--synthesize-only`, or `--termination-reason` are recognized by `cmd_spec::run_inner` — the catch-all `_ => {}` arm in the arg parser silently drops unknown flags. The CLI saw only `--workspace ... --mode driven`, called `run_driven_mode`, opened a FRESH session, re-read the meta-prompt, re-entered the LLM-driven interview loop, blocked on the SiliconFlow API for up to ~15s per turn (W1.2 p1_backend turn-6 15s elapsed was THIS — not synthesis of the original session), then the handler discarded stdout + exit code via `let _ = ...`. Net effect: 10-15s wasted latency + 1 wasted LLM call per abort, zero spec capsule.
2. **No response-side reason field**: client had no way to distinguish "successfully terminated with spec" from "aborted without spec" except by inspecting `spec_capsule_cid == None`.

**Fix**:
- New field `pub(crate) termination_reason: Option<String>` on `SpecTurnResponse` (line 597, with `#[serde(skip_serializing_if = "Option::is_none")]`)
- Removed broken shellout entirely
- Termination paths populate `termination_reason`:
  - `"user_input_unparseable_no_spec"` — triage reject ≥2 (line 1176)
  - `"turn_ceiling_15_no_spec"` — 15-turn ceiling hit (lines 998, 1477)
  - `"predicate_done_no_spec_pending_synthesis"` — LLM done=true + predicate pass but in-process synthesis blocked by spec_capsule module visibility (line 1453); documented as a separate atom
- In-process synthesis is gated because `spec_capsule` module lives under `src/bin/turingos/` and the F6 charter forbade editing binary surface. The library-ization is logged as a follow-on atom

## Files modified

- `src/web/spec.rs`: +1073/-97 LOC (large because it includes the diagnostic comment blocks the F6 agent wrote alongside the fixes; the actual code delta is much smaller; the rest is in-code RCA narrative for future maintainers + 200+ LOC of new regression tests in the `mod tests` block)

## Regression tests added

In existing `#[cfg(all(feature = "web", test))] mod tests`:

- `spec_turn_response_carries_termination_reason_when_present` (line 2345) — pins the new field's serde behavior
- Additional integration-shape tests for the symptoms above (the agent was writing the larger TURINGOS_WEB_WORKSPACE-tempdir integration scaffolds when killed; the simpler unit-shape tests landed)

## Verification

- `cargo check --features web`: clean (16 pre-existing warnings only)
- `cargo test --features web --bin turingos_web web::spec`: **26/26 pass** (was 18 pre-F6; +8 new tests including the F6 regression tests + earlier F4/F5 tests)
- `cargo test --features web --test web_spec_turn_endpoint`: **79/79 pass** (5 ignored — gated on real-LLM)
- End-to-end smoke (rebuilt binary, fresh backend on :8080 with all 6 fixes F1-F6):
  - `POST /api/spec/turn {session_id: f6_smoke_*, user_answer: null, lang: zh}` → HTTP 200 7.1s, real Chinese question, canonical 8 open_slots, `termination_reason: None` (not surfaced for live turns), no zeroes

## Judgment calls

1. **In-process synthesis not implemented**: would require either moving `spec_capsule` out of `src/bin/turingos/` (Class-2 atom, beyond F6 scope) or duplicating the synthesis logic in `src/web/`. F6 chose to document the gap via `termination_reason: "predicate_done_no_spec_pending_synthesis"` so the client can render an honest "interview complete; spec synthesis pending" state. Logged for the follow-on atoms list.

2. **`SpecTurnResponse` schema change**: the new `termination_reason` field is `#[serde(skip_serializing_if = "Option::is_none")]` so existing clients seeing the JSON response without that field will still parse it (forward-compat). The shape change is documented in the response struct's docblock at line 578.

3. **Did NOT extend `--session/--synthesize-only/--termination-reason` flags to `cmd_spec`**: the broken shellout was removed entirely; CLI flag surface was untouched per the no-touch rule on `src/bin/turingos/`.

## Recommendation (deferred atoms)

- **Atom A7**: library-ize `spec_capsule` (move from `src/bin/turingos/` to `src/runtime/` or a new `src/lib/spec/`) so the web layer can write spec capsules in-process without shelling out. Closes `termination_reason: "predicate_done_no_spec_pending_synthesis"`.
- **Atom A8**: invariant test asserting `cmd_spec::run_inner` rejects unknown flags (not silent drop). The pre-F6 abort shellout's silent flag drop was the load-bearing failure; a structural invariant prevents this class.
- **Atom A9**: per-turn response now includes `termination_reason`; frontend (`frontend/src/components/spec-grill.ts`) should render distinct UI for each kind. UX work, not backend.

## No-touch surfaces honored

No Class-4 surfaces, no `Cargo.toml`/`Cargo.lock`, no `genesis_payload`, no `kernel.rs`/`bus.rs`/`sdk/`/`wallet.rs`, no `state/`, no `schema.rs`, no `src/bin/turingos/*`, no commit/push.

## Surfaces touched
- `src/web/spec.rs` only (~3 in-code regions: response struct ~line 578-600, `parse_turn_payload_from_llm_output` ~674-720, multiple termination paths 880-1003 / 1124-1207 / 1386-1503; and the test block at 2144+)
