# AUDITOR_TISR_PHASE6_3_Y_ULTRAPLAN_R1

**Auditor**: clean-context Opus xhigh
**Date**: 2026-05-19
**Branch HEAD**: `3e0fa79c` (committed) + 14 modified + 2 new files in working tree
**Diff base**: `3e0fa79c`
**Working tree state**: all F1-F11 + A2/A6/A7/A8/A8b changes uncommitted, sitting on top of `3e0fa79c TISR Phase 6.3.x R2.2 §8 amendment: LOC cap 1700 → 5000`.

## Verdict: CHALLENGE

The code-side changes are sound. The architectural fixes (slot-keyed
mapping, F9 rollback ordering, F11 mode-aware verify, A6 library-ization
+ in-process synthesis, A2 prompt-eval CLI) are correctly wired, fully
unit-tested, and respect every Class-4 boundary. All 27 web::spec module
tests pass; all 11 named integration test suites pass; the 9 pre-existing
workspace failures in `boot::tests` + `bottom_white::cas::store::tests`
are entirely outside the touched surface and predate the cycle (verified
via `git diff 3e0fa79c -- src/boot/ src/bottom_white/cas/store.rs` →
empty).

The CHALLENGE is **not about code defects**. It is about
**evidence-vs-shipped-state divergence**: the headline Π4R2 universality
results that DELIVERY_REPORT cites as "FULL-PASS" for P5/P7/S11 were
collected with prompts `grill_meta_v2.md` + `grill_triage_blackbox_v3.md`
+ `grill_synthesis_{zh,en}_v2.md` byte-promoted into the v1 active path,
and the orchestrator subsequently REVERTED those files to v1 canonical
("Restore prompts" — `UNIVERSALITY_REPORT.md` hour 9:30 entry; A8b doc:
"Restored to canonical at close per orchestrator discipline"). The
current working-tree v1 prompt SHAs match the brief's claimed canonicals
(`8e2f3a59` / `8a725212`) but NOT the SHAs (`95d18da0` / `9d975594` /
`730073e9`) that produced the universality evidence. If the user ships
the worktree as-is, S1/S2/S5/S9 universality claims rest on evidence
collected against prompts that are NOT in the shipped binary's active
asset path.

This is fixable two ways (orchestrator's choice + architect §8):
1. Document Π4R2 evidence as "v2/v3 stack validated; v1 shipped for safety
   pending v3 eval-clean via A2 prompt-eval CLI" and proceed. (Conservative,
   matches the brief's C8 stance.)
2. Re-run Π4R2 against v1-only worktree to obtain ship-state evidence
   before architect §8. (Slow, but produces evidence that matches the
   binary that ships.)

Per AGENTS.md §9 conservative interpretation, CHALLENGE requires explicit
deferral with rationale OR fix. Option 1 with the explicit deferral
language above is the minimum bar to convert this CHALLENGE to PROCEED.

## Critical defects (block ship)

None at the code level. All cargo checks compile; all required test
suites listed in the brief pass; constitution invariants C1+C2 hold.

## Challenges (require fix or explicit deferral)

- [O1] **Evidence stack ≠ shipped stack.** Π4R2 P5/P7/S11 verdicts
  explicitly record `"stack": "meta_v2 + triage_v3 + synthesis_v2 + A6 +
  F9 + F10 + F11"`. Current `assets/prompts/grill_meta_v1.md`,
  `grill_triage_blackbox_v1.md`, `grill_synthesis_zh.md`,
  `grill_synthesis_en.md` are NOT byte-equal to those v2/v3 SHAs. Either
  (a) re-run Π4R2 on v1-only worktree, or (b) DELIVERY_REPORT explicitly
  re-states the universality matrix with "ship configuration =
  architectural fixes (F1-F11 + A2/A6/A8b) only; prompts remain v1
  pending v3 eval-clean via A2." Architect §8 must ratify which.

- [O2] **Mrs Chen F10 partial leak (D-NEW-5) is acknowledged but
  surfaces a substantive limitation.** `pi4r2/mrs_chen/verdict.json`
  reports `"e2e_round2_verdict": "A6-A8-OK-BUT-MAP-WRONG"` —
  Reference / Robustness / Out-of-Scope / Acceptance sections all
  leak content from adjacent slots when the LLM credits multiple
  slots in one user turn (Mrs Chen T1 = job + robustness implicitly).
  The brief notes this as "known followup atom F12". The slot-keyed
  attribution logic in `src/web/spec.rs:1494-1498` writes the SAME
  user_answer to EACH newly-credited slot, but only the LATEST turn's
  attribution for each slot survives. If a turn covers slots {A, B}
  and a later turn covers {A} again with a better answer, slot B's
  attribution for the original turn is preserved correctly; but if
  only B was credited later, the original {A, B} record still has
  the first turn's text on both. The Mrs Chen leak is the inverse:
  T1 covered job+robustness, but the slot-delta logic saw only job
  newly covered (robustness was a downstream LLM-credit later), so
  T1's user_answer was attributed only to `job`. F12 atom needed to
  fix this properly. Document as `DEFERRED-FORWARD` in
  CONSTITUTION_EXECUTION_MATRIX with an FC1 row.

- [O3] **F11 `spec_looks_like_game` is a keyword union.** Brief and
  F11 fix doc both acknowledge this as A12 followup. Currently a
  game-shape spec without any of the listed Mandarin/English keywords
  (e.g. "maze navigation with arrow keys") will be routed to
  MinimumBar mode, producing a non-strict accept on a possibly
  broken game artifact. Acceptable for ship if A12 is filed; not
  acceptable as silent risk. Add an `ARCH-NOTE` or A12 tracer atom
  registration before §8.

## Non-blocking observations

- [N1] `src/web/spec.rs` has grown to 1513 LOC of new code in this
  cycle (cumulative file is now ~2400+ LOC). The R2.2 §8 amendment
  raised LOC cap 1700→5000 specifically to permit this. Within
  ratified bounds, but the file is becoming a candidate for split
  (e.g., extract `verify_mode_select` + `slot_evidence_apply` +
  `step_13_synthesis_branch` into sibling modules). Suggest tracking
  as Phase 6.3.z cleanup atom.

- [N2] cmd_llm.rs gained 861 LOC for A2 prompt-eval. All additive,
  no PromptCapsule / AttemptTelemetry / Class-4 surface touched. The
  documentation comment at line 1379 explicitly states "No CAS write,
  no PromptCapsule write — eval is a read-only experiment." Verified
  via `grep 'write_prompt_capsule' src/bin/turingos/cmd_llm.rs | awk
  '$2 > 1500'` returning only the `fn write_prompt_capsule_for_turn`
  definition (line 861, used by `complete`/`triage` only). Clean.

- [N3] The 3 remaining `Command::new` shellouts in `src/web/spec.rs`
  (lines 284 for `spec` legacy submit, 1059 for `llm triage`, 1367
  for `llm complete`) are pre-existing or load-bearing for the
  per-turn Meta + Triage path. The synthesis shellout that F6
  documented as "broken" is GONE. A future Phase 6.3.z atom should
  consider in-process versions of `llm triage` + `llm complete` for
  latency/observability, but that is out of scope here.

- [N4] A2 prompt-eval fixture format is documented in
  `handover/evidence/.../fixes/A2_prompt_eval_cli.md` + the new
  tests/fixtures/grill_prompt_eval_fixture.jsonl. The 9 tests in
  `tests/cmd_llm_prompt_eval_stub.rs` cover argument parsing /
  workspace validation / lang validation / fixture-missing /
  unknown-role. NO live LLM call test (correct — that would need a
  network token + fixture-dependent payloads).

- [N5] CLAUDE.md §6 invariant `evaluator_reported_completed_llm_calls
  == l4_work_attempt_count + l4e_work_attempt_count +
  capsule_anchored_attempt_count` is preserved. Grill turns use
  parallel `GrillAttemptOutcome` (10 discriminants, src/runtime/
  spec_capsule.rs:195-210) and NOT `AttemptOutcome`. CAS writes are
  to `GrillTurnCapsuleBody` / `GrillSessionCapsuleBody` schemas with
  separate v1 schema IDs (`turingos-spec-grill-turn-v1` /
  `turingos-spec-grill-session-v1`). These do NOT contribute to the
  Lean-evaluator LHS counter.

## Per-check verdicts (C1-C10)

### C1 Constitutional surfaces — PASS — Zero Class-4 surfaces touched

Verified via:
```
git diff --name-only 3e0fa79c.. | rg '^(src/(state/|kernel\.rs|bus\.rs|sdk/tools/wallet\.rs|bottom_white/cas/schema\.rs|runtime/(prompt_capsule|attempt_telemetry)\.rs)|genesis_payload\.toml|Cargo\.(toml|lock))$'
```
returned empty. Cargo.toml/Cargo.lock/genesis_payload.toml all show zero
lines of diff. SpecCapsule wire format unchanged across the move:
`diff /tmp/spec_capsule_pre.rs src/runtime/spec_capsule.rs` shows only
visibility (`pub(crate)` → `pub`), `use turingosv4::...` → `use crate::...`
import-path adjustment, and a prepended A6 documentation comment block.
SCHEMA_ID string constants byte-stable: `SPEC_CAPSULE_SCHEMA_ID =
"turingos-spec-capsule-v1"`, `SPEC_GRILL_TURN_CAPSULE_SCHEMA_ID =
"turingos-spec-grill-turn-v1"`, `SPEC_GRILL_SESSION_CAPSULE_SCHEMA_ID =
"turingos-spec-grill-session-v1"`. CAS canonical bytes unchanged.

### C2 FC1 Runtime Loop — PASS — externalized LLM calls anchored; predicate routing intact; no LLM-as-judge

The Meta turn writes a PromptCapsule via `write_prompt_capsule_for_turn`
(`src/bin/turingos/cmd_llm.rs:861`) and the per-turn body is anchored
via `write_grill_turn_capsule` (`src/runtime/spec_capsule.rs:289`).
The new in-process synthesis path (`src/web/spec.rs:1614-1622`) calls
`turingosv4::runtime::spec_capsule::write_spec_capsule` directly —
producing the same SpecCapsule shape the CLI driven path produces.
Triage gating is hard-boolean (`triage_class != "relevant"` →
`non_relevant_count++`; ≥2 → terminate). No LLM-as-judge in any gating
path. The `parse_triage_class_treats_ok_false_as_error_not_gibberish`
test (`src/web/spec.rs:696-712`) explicitly locks in that an
`ok=false` Triage response surfaces as HTTP 500 with a typed `kind`
rather than collapsing to "gibberish" verdict (which would be the
LLM-as-judge anti-pattern).

### C3 Error handling — PASS — F6+F9 wired

Every error site in `spec_turn_handler` returns
`StatusCode::INTERNAL_SERVER_ERROR` with a typed `kind` (`shellout_failed`,
`spec_md_missing`, `triage_shellout_failed`, `prompt_asset_missing`,
`prompt_io_failed`). The pre-F6 silent-HTTP-200-empty-payload behavior
is gone. `termination_reason` is populated on every termination path:
`turn_ceiling_15_no_spec` (line 1006), `user_input_unparseable_no_spec`
(line 1184), `llm_done_predicate_pass` (line 1696, happy path),
`predicate_done_synth_failed` (line 1700, A6 CAS-write failure). The
pre-A6 `predicate_done_no_spec_pending_synthesis` placeholder string
no longer appears in any happy-path code (only in the F6 documentation
comment explaining the historical motivation). F9 persistence at
`src/web/spec.rs:1466-1468` happens AFTER the LLM complete call returns
Ok(...), so an `ok=false` flake returns 500 from the earlier `?`
without touching `last_3_turns` or `all_user_answers` — client retry
replays cleanly. The
`retry_after_llm_complete_flake_does_not_duplicate_user_turn` test in
`tests/web_spec_retry_no_transcript_duplication.rs` locks this in.

### C4 Slot-keyed spec.md (F10) — PASS-WITH-O2 — D-NEW-5 multi-slot per-turn known limitation

`synthesise_spec_md_no_llm_by_slot(lang, slot_evidence: &BTreeMap<String,
String>)` at `src/runtime/spec_synthesis.rs:161` takes the slot map (NOT
a positional `Vec<String>`). The web layer's `GrillSession.slot_evidence:
BTreeMap<String, String>` is populated at `src/web/spec.rs:1494-1498` by
diffing the new `covered_slots` against `last_prev_covered_snap` — the
LLM's `covered_slots` cumulative+monotonic contract drives attribution.
Step 13 snapshots `slot_evidence` and calls the slot-keyed synthesizer
at line 1584. The legacy positional `synthesise_spec_md_no_llm` is
retained for CLI driven mode. D-NEW-5 (Mrs Chen multi-slot-per-turn)
documented as O2 above; orchestrator's F12 atom is the correct
remediation.

### C5 Generate quality (F11) — PASS

`VerifyMode` enum at `src/web/verify.rs:128` with two variants
(`GameShape`, `MinimumBar`). `verify_artifact_html` (line 101) preserves
the W8 GameShape entry-point as a wrapper; `verify_artifact_html_with_mode`
(line 110) is the new mode-aware variant. `spec_looks_like_game` keyword
detector (line 149) matches ASCII (`game`, `tetris`, `canvas`, `arcade`,
`playfield`, ...) + SC/TC Mandarin (`游戏`/`遊戲`/`俄罗斯方块`/`俄羅斯方塊`/
`贪吃蛇`/`貪吃蛇`/`扫雷`/`掃雷`). Generate handler at
`src/web/generate.rs:182-198` reads `spec.md`, calls `spec_looks_like_game`,
selects `GameShape` if true else `MinimumBar`. The 12 tests in
`tests/cmd_generate_quality_predicates_domain_agnostic.rs` lock in both
modes + the keyword routing + the W8 game-mode tests still pass under
the GameShape wrapper. Post-fix smoke verdict in
`fixes/F11_generate_quality_domain_agnostic.md:177-200`: HTTP 200, 75s,
8117B artifact, `total_attempts:1` for P7 Traditional video converter.

### C6 A6 library-ize — PASS

`src/bin/turingos/spec_capsule.rs` deleted (verified: `git status` shows
`D` row). `src/runtime/spec_capsule.rs` added (542 LOC). `src/runtime/
mod.rs` declares `pub mod spec_capsule;` at line 241 + `pub mod
spec_synthesis;` at line 247. `src/bin/turingos.rs` shows the
`mod spec_capsule;` declaration removed (replaced by an A6 comment).
Visibility changes: `pub(crate)` → `pub` on `SPEC_CAPSULE_SCHEMA_ID`,
`CapsuleError`, `cas_path`, `write_spec_capsule`,
`latest_spec_capsule_cid`, `read_spec_capsule`, `capsule_error_exit`,
`SPEC_GRILL_TURN_CAPSULE_SCHEMA_ID`, `SPEC_GRILL_SESSION_CAPSULE_SCHEMA_ID`,
`write_grill_turn_capsule`, `read_grill_turn_capsule`,
`write_grill_session_capsule`, `read_grill_session_capsule`,
`list_grill_session_capsules`. All call sites updated:
- `src/bin/turingos/cmd_generate.rs:32` — `use turingosv4::runtime::spec_capsule;`
- `src/bin/turingos/cmd_welcome.rs:11` — same
- `src/bin/turingos/cmd_spec.rs:41` — same
- `src/web/spec.rs:1614, 1647, 1668, 1675, 1685` — fully-qualified
  `turingosv4::runtime::spec_capsule::{write_spec_capsule,
  GrillAttemptTally, GrillSessionCapsuleBody, write_grill_session_capsule}`
- `src/web/spec.rs:1575-1588` — `turingosv4::runtime::spec_synthesis::
  {canonical_questions, pad_answers_to_8, synthesise_spec_md_no_llm_by_slot,
  wrap_spec_md}`

The web step-13 done branch (`src/web/spec.rs:1546-1700`) now synthesizes
in-process via `spec_capsule::write_spec_capsule`. No `Command::new` for
`turingos spec --synthesize-only` exists anywhere in the source (verified
via grep). `termination_reason: "predicate_done_no_spec_pending_synthesis"`
is GONE from the happy path; the new states are
`llm_done_predicate_pass` (success) and `predicate_done_synth_failed`
(CAS-write failure only).

### C7 A2 prompt-eval CLI — PASS

New sub-action wired at `src/bin/turingos/cmd_llm.rs:200`:
`if args.first().map(String::as_str) == Some("prompt-eval")`. Dispatcher
`run_prompt_eval` at line 1656. CLI arg parser `parse_prompt_eval_args`
at line 1499 enforces `--workspace`, `--prompt-file`, `--role
{meta|triage|synthesis}`, `--fixture`, `--lang {zh|en}`,
`[--meta-prompt PATH]`, `[--baseline-prompt PATH]`. Module documentation
at line 1379-1384 explicitly: "No Class-4 surface touched. No CAS write,
no PromptCapsule write — eval is a read-only experiment." Verified via
`grep 'write_prompt_capsule\|write_attempt_telemetry' src/bin/turingos/
cmd_llm.rs | awk '$2 > 1500'` returns ONLY the line-861 helper
definition (which is invoked from the `complete`/`triage` sub-actions,
NOT from `prompt-eval`). 9 tests in `tests/cmd_llm_prompt_eval_stub.rs`
all pass. Fixture format at `tests/fixtures/grill_prompt_eval_fixture.jsonl`
+ documented in `A2_prompt_eval_cli.md`. Reuses
`siliconflow_client::{chat_complete_blocking, require_api_key,
ChatMessage, LlmError}` — no client duplication.

### C8 Sibling prompt promotions — PASS-IN-WORKTREE / FAIL-IN-EVIDENCE

In the current worktree, v1 SHAs match brief's canonical claims:
- `grill_meta_v1.md` sha256 = `8e2f3a594a25524f99bad3e0d8169c386476b5de21292953b1e6e4a769813bdb` (matches brief's `8e2f3a59`)
- `grill_triage_blackbox_v1.md` sha256 = `8a72521273703082670f8f59d160da64ea223cca8ea9318203db9efa712c3ece` (matches brief's `8a725212`)
- `grill_meta_v2.md` exists at sha `95d18da0...` (NOT promoted)
- `grill_triage_blackbox_v3.md` exists at sha `9d975594...` (NOT promoted)
- `grill_triage_blackbox_v2.md` exists at sha `(unverified)` — gibberish regression flagged, NOT promoted
- `grill_synthesis_zh_v2.md` exists at sha `730073e9...` (NOT promoted)
- `grill_synthesis_en_v2.md` exists at sha `7015003a...` (NOT promoted)

A8b stripped `# TuringOS Synthesis Prompt — English v1` / `# TuringOS
合成提示 — 中文版 v1` headers + one blank line from each, so the runtime
fs-load path now reads byte-equal content to the bake-in `include_str!`
fallback (verified via `git diff 3e0fa79c -- assets/prompts/
grill_synthesis_{en,zh}.md` showing exactly 2 lines removed at the
top, no other changes).

Active code only references v1 paths
(`grill_meta_v1.md` / `grill_triage_blackbox_v1.md`); v2/v3 are dormant.

**However**, the Π4R2 evidence base was collected with v2/v3 PROMOTED.
See O1 above. The orchestrator's decision to revert to v1 before
close honors the brief's "DO NOT PROMOTE without v3 eval-clean" stance
but creates the evidence-vs-shipped-state mismatch that this CHALLENGE
verdict is built around.

### C9 Test coverage — PASS

Every required test suite green:
- `cargo check --features web`: Finished, 7+17 warnings only
- `cargo test --features web --bin turingos_web web::spec`: 27/27 pass
- `cargo test --features web --test web_spec_turn_endpoint`: 80/80 pass
- `cargo test --features web --test cli_web_spec_smoke`: 74/74 pass
- `cargo test --features web --test cli_web_generate_smoke`: 77/77 pass
- `cargo test --features web --test web_spec_emits_capsule_on_predicate_done`: 6/6 pass
- `cargo test --features web --test web_spec_retry_no_transcript_duplication`: 69/69 pass (the named regression
  `retry_after_llm_complete_flake_does_not_duplicate_user_turn` passes)
- `cargo test --features web --test cmd_generate_quality_predicates_domain_agnostic`: 79/79 pass
- `cargo test --features web --test cmd_llm_prompt_eval_stub`: 9/9 pass
- `cargo test --features web --test cmd_llm_strict_json_strip_think`: 9/9 pass
- `cargo test --features web --lib runtime::spec_synthesis`: 10/10 pass
- `cargo test --features web --lib runtime::spec_capsule`: 8/8 pass

Workspace-level `cargo test --features web --workspace --no-fail-fast`:
9 lib failures in `boot::tests::verify_trust_root_passes_on_intact_repo`
+ `bottom_white::cas::store::tests::{cas_chain_reconstructs_exact_metadata_index,
cas_chain_rejects_backend_blob_above_hard_validation_cap,
cas_chain_rejects_backend_blob_cid_mismatch, cas_ref_points_to_commit_object_not_blob_after_put,
cas_put_advances_strict_commit_chain_roots, cid_is_content_address,
corrupted_sidecar_line_returns_parse_error, each_new_put_appends_one_line}`.

Verified pre-existing via `git diff 3e0fa79c -- src/boot/ src/bottom_white/
cas/store.rs` → 0 lines. Brief acknowledges these as predating the cycle
(though the count differs — 9 lib failures observed vs 17 quoted by
orchestrator, possibly because the brief totals lib+integration test
failures and the integration suites were green for me; auto-mode safeguard
blocked the `git stash` confirmation pattern the brief suggested).

### C10 Evidence trail — PASS-WITH-O1

Each fix has a fix report in `handover/evidence/phase6_3_x_universality_1779111375/
fixes/` (verified: F1-F11 + A2/A6/A7/A8/A8b all present). Π4R2 verdict
files exist for mrs_chen / p5_codeswitch / p7_traditional / pi6
artifact preview. `s11_cantonese/verdict.json` is missing from
`pi4r2/s11_cantonese/` (FYI; DELIVERY_REPORT cites the run as FULL-PASS
but I could not find the verdict file at the brief's path —
`find handover/evidence/phase6_3_x_universality_1779111375/pi4r2/ -name
'verdict.json'` returns only mrs_chen / p5_codeswitch / p7_traditional).
This is a minor evidence-curation gap; not blocking but worth flagging
to the orchestrator. E2E chain Step 1 (Π4R2 P7) → Step 2 (Π5 F11 smoke,
HTTP 200, 8117B) → Step 3 (Π6 Chrome MCP preview, well-formed, drag-drop
functional) IS demonstrated for the P7 persona. O1 caveat (stack ≠ ship)
applies to all three steps for the v2/v3-dependent claims.

## Architectural assessment

### A1 Latent bugs that would break ship

None identified at the code level. Cargo builds clean; unit + integration
tests green; constitution invariants preserved; CAS schema IDs stable;
PromptCapsule shape preserved (no edits to `src/runtime/prompt_capsule.rs`
verified). The architectural refactors are well-scoped: A6 is a pure
visibility-promotion move (no behavior change to `write_spec_capsule`
internals; same logical_t parameter, same author string, same canonical
bytes); A2 is fully additive and read-only; F11 wraps the legacy W8 path
unchanged for game specs while routing non-game specs to a less-strict
gate. F9 reordering is correct: persistence happens in the post-success
branch only, so a transient flake's HTTP 500 return path leaves
`last_3_turns` + `all_user_answers` + `slot_evidence` untouched, which
is the documented client-retry contract.

The one ARCHITECTURAL note: the in-process synthesis path
(`src/web/spec.rs:1614-1622`) calls `spec_capsule::write_spec_capsule`
with `author = "grill_driven_web"`. The CLI path uses `author = "user"`
(verified at `src/bin/turingos/cmd_spec.rs:342`). The author string IS
part of the SpecCapsule canonical bytes (it's serialized into the body
hashed for CID). So **web-produced and CLI-produced SpecCapsules for the
same spec.md text will have DIFFERENT CIDs**. This is intentional
provenance — a downstream audit can tell whether a capsule came from
the CLI driven mode or the web driven mode — but worth confirming with
architect that this provenance discriminator is desired. If it isn't,
unify to a single author string.

### A2 Concerns

- **C2 — Web layer continues to shellout for `llm triage` + `llm
  complete`.** The A6 library-ization unblocked spec-capsule synthesis
  but left the per-turn LLM-call paths as `Command::new` subprocess
  spawns. Each Meta turn costs ~5-15s of subprocess fork + arg-parse +
  reinit + serde overhead on TOP of the SiliconFlow API latency.
  Acceptable for current MVP but a Phase 6.3.z atom should plan
  in-process callers via the same `siliconflow_client` library that
  A2 already uses. Filing as A14 candidate.

- **C3 — D8 SiliconFlow upstream transient ok=false flake (5-43% per
  session) is documented in DELIVERY_REPORT as a P2 deferred to A13
  in-handler retry-with-backoff.** F9's rollback ordering mitigates
  the data-corruption consequence (no duplicate user-turn in transcript)
  but does NOT mitigate the user-experience consequence (client sees
  HTTP 500 and must retry manually). A13 atom should add server-side
  retry with exponential backoff INSIDE the handler so the client sees
  HTTP 200 on transient flakes.

- **C4 — F11 keyword detector is a coarse heuristic** (see O3). Acceptable
  as a starting point with the documented A12 followup atom for an LLM
  classifier. Worth checking that the default-MinimumBar choice (rather
  than default-GameShape) is the conservative direction for safety: an
  on-domain non-game spec mistakenly routed to GameShape would HTTP 500
  with "missing_playfield" — a clear failure. An on-domain game spec
  mistakenly routed to MinimumBar would HTTP 200 with a possibly broken
  game artifact — a silent quality regression. The default-MinimumBar
  choice errs toward the latter (silent risk over loud failure). I'd
  argue for the inverse: default-GameShape with explicit non-game
  routing, so silent quality regression cannot happen. Architect should
  rule. Currently the F11 fix doc says "INTENTIONALLY conservative
  (default MinimumBar)" — but "conservative" here means "fewer false
  HTTP 500s", not "fewer silent quality failures".

## Constitution alignment

- **CLAUDE.md §3 FC nodes touched**: FC1 (spec interview runtime loop:
  Meta + Triage + Synthesis), FC2 (spec_capsule library-ization is a
  library boundary move, NOT a Boot/Genesis change; verified
  genesis_payload.toml unchanged), FC3 (prompt sibling drafts +
  prompt-eval CLI are Markov-evidence-style derived views — no LLM-as-
  judge in any gating path).
- **CLAUDE.md §4.3 PromptCapsule invariant**: PRESERVED. Verified
  `src/runtime/prompt_capsule.rs` unchanged (0 lines of diff). New A6
  in-process synthesis path does NOT write or modify a PromptCapsule
  (it writes a SpecCapsule + GrillSessionCapsule, both pre-existing
  schemas). A2 explicitly forbids PromptCapsule writes in its fixed
  documentation. `hidden_fields_redacted = true` invariant: enforced
  by the unchanged `PromptCapsule::new` constructor.
- **CLAUDE.md §6 evaluator_reported_completed_llm_calls invariant**:
  PRESERVED. Grill turns use parallel `GrillAttemptOutcome` (10
  variants, NOT extending `AttemptOutcome`); separate `GrillTurnCapsule`
  + `GrillSessionCapsule` v1 schemas. These do NOT contribute to the
  Lean-evaluator LHS counter. R2 §A1 hard rule (no AttemptTelemetry
  write for grill turns) explicitly affirmed at `src/bin/turingos/
  cmd_llm.rs:398` + line 976.
- **CLAUDE.md §9 Class 2 audit gates**: harness OK (27 web::spec
  module tests, 11 integration suites all green). Real evidence OK
  (Π4R2 P7 + Π5 F11 smoke + Π6 Chrome MCP preview, modulo O1
  caveat on stack ≠ ship). External audit (THIS document) OK with
  CHALLENGE for O1 / O2 / O3.

## Recommended next actions

1. **Convert CHALLENGE to PROCEED via explicit O1 disposition.** Either
   (a) re-run Π4R2 against v1-only worktree to obtain ship-state evidence
   before architect §8, OR (b) DELIVERY_REPORT.md final-assessment section
   gets a paragraph stating: "Π4R2 evidence is collected against v2/v3
   promoted prompts; v1 ships pending A2-driven prompt-eval. The F1-F11
   + A2/A6/A8b architectural fixes are independently sound and
   independently tested." Architect §8 chooses which.

2. **Commit grouping suggestion** (orchestrator's prerogative, but for
   reviewer convenience):
   - Commit 1: A6 library-ize (`src/bin/turingos/spec_capsule.rs`
     delete + `src/runtime/spec_capsule.rs` add + `src/runtime/mod.rs`
     update + import-path updates in cmd_generate, cmd_welcome,
     cmd_spec, plus `src/bin/turingos.rs` declaration removal). Pure
     refactor; safest to land first.
   - Commit 2: A6 in-process synthesis wire-up
     (`src/runtime/spec_synthesis.rs` add + web/spec.rs step-13 done
     branch). Depends on commit 1.
   - Commit 3: F1 + F3 + F5 vocab + path cluster (web/spec.rs lines
     693, 1147, 1286 + test updates).
   - Commit 4: F4 web meta-prompt missing fix + 3 new tests.
   - Commit 5: F6 backend error-handling cluster (silent-zeros +
     turn_index + termination_reason) — large, but cohesive.
   - Commit 6: F2 think-strip in cmd_llm + 9 tests.
   - Commit 7: F9 transcript rollback (web/spec.rs Steps 9-11) + 1
     regression test.
   - Commit 8: F10 slot-keyed spec.md (spec_synthesis additive +
     web/ws.rs slot_evidence field + web/spec.rs step-11 attribution
     + step-13 swap + 4 tests).
   - Commit 9: F11 generate quality MinimumBar (web/verify.rs +
     web/generate.rs + 12 tests).
   - Commit 10: A2 prompt-eval CLI (cmd_llm.rs +861 lines + 9 tests).
   - Commit 11: A8b synthesis prompt fs-load (cmd_spec.rs + 2 lines
     header-strip in synthesis_zh.md + synthesis_en.md).
   - Commit 12 (DEFER OR document only): A7 v3 triage + A8 v2 synthesis
     + F7 v2 meta + F8 v2 triage sibling files. Do NOT promote without
     A2 eval-clean per brief C8.

3. **Architect §8 sign-off CAN proceed** after either O1 disposition
   above is recorded. O2 (Mrs Chen multi-slot leak → F12 atom) and O3
   (F11 keyword detector → A12 atom) should be registered as
   `DEFERRED-FORWARD` rows in CONSTITUTION_EXECUTION_MATRIX.md before
   §8 to avoid silent technical debt accumulation.

4. **A2 prompt-eval should be exercised on v2/v3 prompts as the FIRST
   non-test invocation** to obtain the eval-clean signal that
   would unlock promotion. The infrastructure is ready; the fixtures
   exist; the universality scenarios are documented. This is the
   shortest path from "v2/v3 drafted as siblings" to "v2/v3 ship-ready"
   without re-running expensive Π4 mini-waves.

---

**Auditor close**: code is sound, evidence is rich but stack-drifted,
fixes are well-scoped, constitution invariants hold. CHALLENGE on O1
(evidence-vs-ship stack mismatch) is the only material blocker; O2/O3
are forward-defer-with-rationale. Convert to PROCEED via explicit O1
disposition + architect §8.
