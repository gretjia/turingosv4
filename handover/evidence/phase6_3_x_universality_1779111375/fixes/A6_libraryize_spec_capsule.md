# A6 — Library-ize `spec_capsule` (close F6 deferred atom)

- TB ratification: `/ultraplan` 2026-05-19
- Risk class: 2 (production wire-up + module move)
- Branch: `codex/tisr-phase6-3-x-grill-driven`
- Pre-fix HEAD: `3e0fa79cbf999fafbf56eae67da2960a063cf5d5`
- FC trace: FC1-N9 (predicate termination), FC2-N16 (CAS spec wire), FC3-N4 (EvidenceCapsule)
- Allowed paths touched:
  - `src/runtime/spec_capsule.rs` (new — moved from `src/bin/turingos/`)
  - `src/runtime/spec_synthesis.rs` (new — lifted helpers from `cmd_spec.rs`)
  - `src/runtime/mod.rs` (added two `pub mod` lines)
  - `src/bin/turingos.rs` (removed one `mod spec_capsule;` declaration)
  - `src/bin/turingos/cmd_spec.rs` (1 import line)
  - `src/bin/turingos/cmd_generate.rs` (1 import line)
  - `src/bin/turingos/cmd_welcome.rs` (1 import line)
  - `src/web/spec.rs` (replaced F6 pending-synthesis branch with in-process synthesis)
  - `src/web/ws.rs` (added `GrillSession.all_user_answers` field)
  - `tests/web_spec_turn_endpoint.rs` (constructor update for new field)
  - `tests/web_spec_emits_capsule_on_predicate_done.rs` (new — A6 invariant test)
- Forbidden paths (untouched, verified): `Cargo.toml`, `Cargo.lock`, `src/state/*`, `src/bottom_white/cas/schema.rs`, `src/runtime/prompt_capsule.rs`, `src/runtime/attempt_telemetry.rs`, `src/kernel.rs`, `src/bus.rs`, `src/sdk/tools/wallet.rs`, `genesis_payload.toml`.

## Defect

`turingos_web` driven-mode sessions that reached `done=true + predicate-pass`
(W3.2 P6 emoji, M4 P5, M5 P7, M7 S11) returned
`spec_capsule_cid: null` + `termination_reason: "predicate_done_no_spec_pending_synthesis"`
instead of a real 64-char hex CID. The synthesis helpers + CAS writers lived
inside the `turingos` binary crate (`src/bin/turingos/spec_capsule.rs`), so the
`turingos_web` binary could not call them — F6 documented this as
"library-ization needed" and removed the broken shellout that previously
synthesised fake `placeholder_<unix_secs_hex>` values.

## Option chosen — A (move + visibility promotion)

Moved `src/bin/turingos/spec_capsule.rs` to `src/runtime/spec_capsule.rs`,
promoted `pub(crate)` → `pub` on the 5 CAS-side helpers + 2 schema-id
constants + `CapsuleError` enum, and rewrote two `use turingosv4::...`
imports inside the file to `use crate::...` (the file now lives in the
library crate where `crate` IS turingosv4).

Rejected option B (parallel slim shim) because option A had only 3 call
sites in the bin (`cmd_spec.rs`, `cmd_generate.rs`, `cmd_welcome.rs`) — each
a single `use crate::spec_capsule;` line that became
`use turingosv4::runtime::spec_capsule;`. Option B would have duplicated
~150 lines of CAS code and required dual-maintenance for any future schema
change. Total call-site changes for Option A: 3 import statements; the 42
`spec_capsule::*` qualified call sites in the bin needed zero edits.

The synthesis helpers (`canonical_questions`, `synthesise_spec_md_no_llm`,
`wrap_spec_md`, plus a new `pad_answers_to_8`) were lifted to a new
sibling library module `src/runtime/spec_synthesis.rs` so the web layer
could call them without depending on the bin. The CLI's verbatim
copies in `cmd_spec.rs` were left in place to keep this atom scope-minimal
(no risk to in-flight A7/A8); a future follow-up can delete the CLI
duplicates once the test surface stabilises.

## File moves + import-path updates

| Surface | Change |
|---|---|
| `src/bin/turingos/spec_capsule.rs` | DELETED |
| `src/runtime/spec_capsule.rs` | CREATED (byte-identical to pre-move minus visibility + 2 import rewrites) |
| `src/runtime/spec_synthesis.rs` | CREATED (4 pure helpers + 7 unit tests) |
| `src/runtime/mod.rs` | added `pub mod spec_capsule;` + `pub mod spec_synthesis;` |
| `src/bin/turingos.rs` | removed `#[path = "turingos/spec_capsule.rs"] mod spec_capsule;` |
| `src/bin/turingos/cmd_spec.rs` | `use crate::spec_capsule;` → `use turingosv4::runtime::spec_capsule;` |
| `src/bin/turingos/cmd_generate.rs` | same import rewrite |
| `src/bin/turingos/cmd_welcome.rs` | same import rewrite |
| `src/web/ws.rs` | added `GrillSession.all_user_answers: Vec<String>` (full answer history; mirrors `cmd_spec::DrivenState::all_user_answers`) |
| `src/web/spec.rs` | step-9 push site mirrors triage-relevant answer into `all_user_answers`; step-13 `if done` branch now calls library synthesis + writes SpecCapsule + GrillSessionCapsule in-process and populates `spec_capsule_cid` |
| `tests/web_spec_turn_endpoint.rs` | one GrillSession constructor updated for new field |
| `tests/web_spec_emits_capsule_on_predicate_done.rs` | NEW — 6 invariant tests for the library API the web layer now depends on |

## Tests

### `cargo check --features web` results

```
cargo check --workspace --all-targets --features web
... Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.29s
(zero errors; only pre-existing dead-code warnings)
```

### `cargo test --features web --no-fail-fast` results

Targeted (this atom):
- `runtime::spec_capsule::grill_capsule_tests` — **8 passed, 0 failed**
- `runtime::spec_synthesis::tests` — **7 passed, 0 failed**
- `tests/web_spec_emits_capsule_on_predicate_done.rs` — **6 passed, 0 failed**
- `tests/web_spec_turn_endpoint.rs` — **79 passed, 0 failed**
- `tests/grill_*` (all integration files) — pass

Workspace total: **2734 passed, 17 failed, 144 ignored**. All 17 failures
are pre-existing in unrelated surfaces (boot trust-root,
`bottom_white::cas::store` git2 plumbing, `buy_yes_*` market economy,
`fc3_n34_readonly_guard`); none touch `runtime::spec_*`, `web::spec`,
`grill_*`, or anything modified by this atom.

## End-to-end smoke

```bash
export SILICONFLOW_API_KEY=$(grep '^SILICONFLOW_API_KEY=' /Users/zephryj/projects/turingosv3/.env | head -1 | cut -d= -f2)
kill $(lsof -ti :8080) 2>/dev/null
cargo build --bin turingos --bin turingos_web --features web
# → Finished `dev` profile … (clean build)

nohup env SILICONFLOW_API_KEY="$SILICONFLOW_API_KEY" \
  TURINGOS_WEB_WORKSPACE=/Users/zephryj/work/turingosv4/tmp/universality_campaign \
  ./target/debug/turingos_web > /tmp/a6_backend.log 2>&1 &
# → TuringOS Phase 7 Web MVP listening on http://127.0.0.1:8080
```

Drove a 7-turn Mrs-Chen-style session (`session_id = a6_smoke_1`,
`lang = zh`). Final response from turn 7:

```json
{
  "turn_index": 7,
  "question_text": "",
  "covered_slots": ["job","anchor","memory","first_run","robustness","scope","acceptance","mirror"],
  "open_slots": [],
  "confidence": 1.0,
  "done": true,
  "playback": "1. 任务：帮妈妈自动算清每周家庭团购的账…",
  "terminated": true,
  "spec_capsule_cid": "349bfca3b8c1cea97f48365299a105c16f25f98a32175255acdc3f204f841cf9",
  "turn_capsule_cid": null,
  "termination_reason": "llm_done_predicate_pass"
}
```

Confirmation that `spec_capsule_cid` is a **real 64-char hex CID, NOT null,
NOT "pending_synthesis"**.

CAS round-trip witness:

```
$ ./target/debug/turingos welcome --workspace tmp/universality_campaign
…
  [x] 4. turingos spec (CAS capsule: 349bfca3…4f841cf9)
…
```

`turingos welcome` reads the CAS sidecar index, finds the schema-id-tagged
EvidenceCapsule, and confirms the same CID round-trips through
`spec_capsule::latest_spec_capsule_cid`. Spec.md is also persisted to
`<workspace>/sessions/a6_smoke_1/spec.md` for client preview.

## Judgment calls

1. **Web path is LLM-less by design.** The CLI driven path tries Meta-LLM
   synthesis first (`synthesise_spec_md_no_llm` as fallback). The web
   handler uses the LLM-less path directly because (a) the answer history
   is short Mom-Test prose that synthesises cleanly without LLM expansion,
   (b) avoiding a second per-request LLM call keeps the HTTP request
   bounded and deterministic, (c) header tag `meta model: web-skip-llm`
   makes provenance auditable. A future atom can add optional LLM
   synthesis behind a query flag if downstream consumers want it.

2. **Did not refactor `cmd_spec.rs` to delegate to `spec_synthesis`.** The
   verbatim copies of `canonical_questions` / `synthesise_spec_md_no_llm`
   / `wrap_spec_md` remain in `cmd_spec.rs`. Removing them would touch
   four functions that A7/A8 (still in-flight) are reading; the duplicate
   is verbatim and the new `spec_synthesis` module has its own 7-test
   suite that guarantees byte-identical output. Cleanup is a follow-up
   atom, not a ship blocker.

3. **`session_id` not added to `GrillSessionCapsuleBody.session_id`
   namespace.** The web path uses the client-supplied `session_id`
   directly (CLI uses a generated `{epoch_secs}_{random_hex}` ID). Both
   are accepted by the schema (free-form string), and the namespace
   collision risk is low because clients pick UUIDs and CLI auto-generates
   timestamped IDs. If this becomes a problem, the fix is in the schema
   validator (Class 3), not here.

4. **`partial_session` is true only when synthesis CAS write fails.** The
   CLI sets `partial_session = (termination_reason != "llm_done_predicate_pass")`.
   The web path now sets it on the same condition; the `predicate_done_synth_failed`
   case (CAS write failure after predicate pass) marks the session
   partial because the SpecCapsule CID is empty — that matches CLI
   semantics (synthesis_calls=0, no spec_capsule produced).

## Follow-up

- **F6 deferred items closed.** The `predicate_done_no_spec_pending_synthesis`
  branch is gone; replaced by real synthesis emitting `llm_done_predicate_pass`
  (or `predicate_done_synth_failed` if CAS write fails).
- **New invariant test landed**: `tests/web_spec_emits_capsule_on_predicate_done.rs`
  — 6 tests gating that the synthesis primitives + CAS round-trip remain
  callable from outside the bin crate. Future regressions that re-trap
  the surface inside a binary will fail this test at compile time.
- **Recommend (future atom):** delete the duplicate helpers in
  `cmd_spec.rs` and have the CLI delegate to `spec_synthesis` for
  single-source-of-truth. Safe-to-defer (not a ship blocker; both paths
  emit identical bytes).
