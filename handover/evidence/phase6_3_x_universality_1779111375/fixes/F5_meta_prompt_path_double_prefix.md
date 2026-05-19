# F5 — Web `--meta-prompt` Subprocess Arg Double-Prefix

- **Date**: 2026-05-18
- **Branch**: `codex/tisr-phase6-3-x-grill-driven`
- **Risk class**: 2 (production wire-up, evaluator/dispatcher adapter — no Class-3/4 surfaces touched)
- **Pre-fix HEAD**: `3e0fa79cbf999fafbf56eae67da2960a063cf5d5`
- **Surface**: `src/web/spec.rs::spec_turn_handler` (1 LOC behavior change + comment + regression test)
- **Authorization**: architect-authorized 2026-05-18 universality campaign (F1–F5 series)

---

## 1. Defect

After F4 landed, the web driven-mode `POST /api/spec/turn` setup call (turn-1)
still returned HTTP 500. The error class changed from `shellout_failed` (F4) to
a fresh, more diagnostic message originating in the `turingos llm complete`
subprocess:

```
IO 错误: reading --meta-prompt tmp/universality_campaign/tmp/universality_campaign/assets/prompts/grill_meta_v1.md: No such file or directory (os error 2)
```

The doubled `tmp/universality_campaign/tmp/universality_campaign/...` prefix is
the smoking gun.

### How it slipped past F4

F4 fixed the missing-meta-prompt-in-messages defect by:

1. computing `meta_prompt_path = PathBuf::from(&workspace).join("assets/prompts/grill_meta_v1.md")` — a CWD-relative full path used to `read_to_string` the asset content into `messages[0]`;
2. *also* passing that same `PathBuf` to the subprocess as `--meta-prompt`.

The asset-reading half (step 1) is correct and stays. The subprocess-arg half
(step 2) is wrong because `src/bin/turingos/cmd_llm.rs::complete_action`
re-prefixes any non-absolute `--meta-prompt` via `workspace.join(mp_path)`
(verified at `cmd_llm.rs:887-892`):

```rust
let resolved = if mp_path.is_absolute() {
    mp_path.clone()
} else {
    workspace.join(mp_path)   // <-- this re-prefixes the F4 full path
};
match fs::read(&resolved) { ... }
```

So the subprocess receives `tmp/universality_campaign/assets/...` (already
CWD-relative), re-prefixes it under `--workspace tmp/universality_campaign`,
and ENOENTs on the doubled path. The CLI exits non-zero with the IO error,
the web handler parses no envelope, returns 500.

**`--meta-prompt` is informational only.** Per CLAUDE.md §4.3 and the cmd_llm
header comment, the flag exists solely to compute
`PromptCapsule.system_prompt_template_hash` via sha256. The actual chat
messages are loaded from `--prompt-file`, where F4 already prepended
`messages[0]` with the meta-prompt content. So the path passed to
`--meta-prompt` only needs to *exist* for the hash; it never needs to match
what's in the messages array.

### Why F4's tests didn't catch this

F4's `web_spec_turn_prompt_*` regression tests assert on the in-process
`build_web_turn_prompt_json` output — they verify the messages array shape but
do not exercise the subprocess shell-out. The integration tests in
`tests/web_spec_turn_endpoint.rs` stub the binary via `TURINGOS_BACKEND_OVERRIDE`
to bypass real CLI invocation. The defect only surfaces with the real
`turingos` CLI, real workspace, real env (which the W9 / W10-R1 / present
smokes exercise). F5's regression test (below) closes that loop at unit-test
scope.

---

## 2. Fix

Decouple the two uses of the path:

- **Asset read path** (existing F4 behavior, kept): `let meta_prompt_path = PathBuf::from(&workspace).join("assets/prompts/grill_meta_v1.md")` — full CWD-relative `PathBuf` consumed by `std::fs::read_to_string`.
- **Subprocess `--meta-prompt` arg** (new F5 behavior): hardcoded workspace-relative literal `"assets/prompts/grill_meta_v1.md"`.

Behavior diff (in `src/web/spec.rs::spec_turn_handler`, ~line 1170):

```diff
-    let mp2 = meta_prompt_path.clone();
+    // FIX F5 (2026-05-18): pass the WORKSPACE-RELATIVE meta-prompt path to the
+    // subprocess. `cmd_llm::complete_action` resolves any non-absolute
+    // `--meta-prompt` value via `workspace.join(mp_path)` (informational sha256
+    // for PromptCapsule.system_prompt_template_hash). F4 passed the already-
+    // prefixed `<workspace>/assets/prompts/grill_meta_v1.md` here, which caused
+    // the subprocess to re-prefix and ENOENT on
+    // `<workspace>/<workspace>/assets/prompts/grill_meta_v1.md` →
+    // `{kind: shellout_failed}`. Hardcode the canonical asset-relative path to
+    // decouple from `meta_prompt_path` (which remains the CWD-relative read
+    // path used above by F4).
+    const META_PROMPT_REL: &str = "assets/prompts/grill_meta_v1.md";
     ...
             .arg("--meta-prompt")
-            .arg(&mp2)
+            .arg(META_PROMPT_REL)
```

### Triage shellout — no fix needed

The triage shellout at `src/web/spec.rs:942-963` invokes `turingos llm triage`
(not `complete`). Inspection confirms `triage` does not accept any
`--meta-prompt` (or analogous `--triage-prompt`) flag, so the defect class
does not apply. Verified via `cmd_llm.rs::ENV_HELP` and the triage arg parser
(`triage_action` subcommand). No second fix required.

### Session-capsule synthesize shellout — no fix needed

The `spec --synthesize-only` shellout at `src/web/spec.rs:1001-1013`
(termination path on 2 non-relevant triages) does not pass `--meta-prompt`
either. Not affected.

---

## 3. Regression test

Added `web_spec_turn_passes_workspace_relative_meta_prompt_arg` to the
existing `#[cfg(all(feature = "web", test))] mod tests` block in
`src/web/spec.rs`. The test pins three invariants:

1. The subprocess `--meta-prompt` literal does NOT contain the workspace prefix (the F4 bug shape).
2. The subprocess literal is not absolute (so cmd_llm's `workspace.join` semantics apply).
3. Joining the subprocess literal under the workspace yields the same path as the F4 in-process `read_to_string` path — i.e. the subprocess and the in-process loader agree on which file is hashed/read.

A sentinel assert pins the exact pathological doubled-prefix shape so a future
refactor that reintroduces it trips the test immediately.

```
test web::spec::tests::web_spec_turn_passes_workspace_relative_meta_prompt_arg ... ok
```

---

## 4. Verification

### 4.1 `cargo check --features web`

```
warning: `turingosv4` (bin "turingos_web") generated 16 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.40s
```

Clean (16 pre-existing dead-code warnings on `SlotState::Empty/Partial` and
`GrillSession` fields — unrelated to F5).

### 4.2 `cargo test --features web --bin turingos_web web::spec --no-fail-fast`

```
running 18 tests
test web::spec::tests::generate_session_id_format ... ok
test web::spec::tests::is_safe_session_id_accepts_valid ... ok
test web::spec::tests::extract_slots_empty_covered_returns_full_canonical_open ... ok
test web::spec::tests::build_coverage_summary_empty_state_lists_all_canonical_as_uncovered ... ok
test web::spec::tests::extract_slots_uses_canonical_vocab_and_computes_open ... ok
test web::spec::tests::build_coverage_summary_uses_canonical_vocab_not_draft ... ok
test web::spec::tests::is_safe_session_id_rejects_traversal ... ok
test web::spec::tests::parse_capsule_cid_from_stdout_finds_cid ... ok
test web::spec::tests::parse_capsule_cid_from_stdout_returns_none_on_no_match ... ok
test web::spec::tests::spec_questions_has_8_entries ... ok
test web::spec::tests::validate_answers_accepts_valid_8 ... ok
test web::spec::tests::validate_answers_rejects_empty_answer ... ok
test web::spec::tests::validate_answers_rejects_oversized_answer ... ok
test web::spec::tests::validate_answers_rejects_wrong_count ... ok
test web::spec::tests::web_spec_turn_passes_workspace_relative_meta_prompt_arg ... ok
test web::spec::tests::web_spec_turn_prompt_first_message_is_system_with_meta ... ok
test web::spec::tests::web_spec_turn_prompt_includes_meta_prompt ... ok
test web::spec::tests::web_spec_turn_prompt_real_asset_loads_and_prepends ... ok

test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 41 filtered out
```

All 18 web::spec tests green — including the 3 F4 meta-prompt assertions and
the new F5 double-prefix assertion.

### 4.3 `cargo test --features web --test web_spec_turn_endpoint --no-fail-fast`

```
test result: ok. 71 passed; 0 failed; 5 ignored; 0 measured; 0 filtered out
```

### 4.4 `cargo test --features web --test cli_web_spec_smoke --no-fail-fast`

```
test result: ok. 65 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### 4.5 End-to-end real-LLM smoke

```
$ cargo build --bin turingos_web --features web
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.32s

$ kill $(lsof -ti :8080)
$ nohup env SILICONFLOW_API_KEY="$SILICONFLOW_API_KEY" \
    TURINGOS_WEB_WORKSPACE=tmp/universality_campaign \
    ./target/debug/turingos_web > /tmp/f5_smoke.log 2>&1 &

$ SID="f5_smoke_$(date +%s)"  # → f5_smoke_1779115212

$ curl -s -X POST http://127.0.0.1:8080/api/spec/turn \
    -H 'Content-Type: application/json' \
    -d "{\"session_id\":\"$SID\",\"user_answer\":null,\"lang\":\"zh\"}"
```

**HTTP status: 200 OK** (previously 500 on F4).

Response body (pretty-printed; raw was single-line):

```json
{
  "turn_index": 1,
  "question_text": "你好！请描述一下你希望我帮你构建的这个工具或游戏。它主要是用来做什么的？或者，你是在什么样的情境下会想'要是有个工具能帮我做这个就好了'？",
  "covered_slots": [],
  "open_slots": ["job", "anchor", "memory", "first_run", "robustness", "scope", "acceptance", "mirror"],
  "confidence": 0.0,
  "done": false,
  "playback": null,
  "terminated": false,
  "spec_capsule_cid": null,
  "turn_capsule_cid": null
}
```

Key observations:

- `question_text` is a **real Chinese question** about the user's project — proves the meta-prompt was loaded, the LLM saw its interviewer contract, and emitted a valid envelope.
- `open_slots` is the canonical 8-slot vocabulary (F1 fix verified still good).
- `done: false` + `confidence: 0.0` correct for turn-1 with no prior answer.
- Backend log `/tmp/f5_smoke.log` contains NO `ERROR`, NO `reading --meta-prompt`, NO `prompt_capsule` warnings.

Session artifacts on disk:

```
$ ls tmp/universality_campaign/sessions/f5_smoke_1779115212/
capsules/
turn-1-prompt.json   # 3855 bytes — generated meta+coverage+turn-instruction prompt
```

Backend left running (orchestrator wants it persistent), pid 54674.

---

## 5. No-touch verification

Confirmed no changes to:

- Class-4 surfaces (kernel, bus, sequencer, typed_tx, wallet, schema.rs, RootBox, signing payloads)
- `Cargo.toml`, `Cargo.lock`
- `genesis_payload` / boot / on_init
- `handover/evidence/` ChainTape/CAS evidence (only this new doc + sessions/ runtime output)
- Other risk-class surfaces enumerated in CLAUDE.md §6

Git diff scope: `src/web/spec.rs` only (production change + 1 new test).

```
$ git diff --stat src/web/spec.rs
 src/web/spec.rs | <one production change + one test added>
```

---

## 6. Recommendation — defer to separate atom

The defect class "subprocess flag is workspace-relative; caller already has
the full CWD path; double-prefix ENOENT" is **recurring**:

- F4 introduced it (subprocess vs in-process loader).
- It is plausibly hiding elsewhere in any place that constructs a `PathBuf` via `workspace.join(...)` and then forwards the result as a flag value to a child `turingos` invocation.

**Proposed helper** (defer to a separate atom, NOT in F5 scope):

```rust
// src/bin/turingos/cmd_llm.rs (new pub fn) — or a shared util module
/// Resolve a path that may be passed by callers either as
/// workspace-relative or already-CWD-relative-with-workspace-prefix.
/// Returns the workspace-relative form suitable for re-prefixing under
/// any `--workspace`.
pub fn workspace_relative_arg(workspace: &Path, candidate: &Path) -> PathBuf {
    if let Ok(stripped) = candidate.strip_prefix(workspace) {
        stripped.to_path_buf()
    } else {
        candidate.to_path_buf()
    }
}
```

Callers wrap their `--meta-prompt` / `--prompt-file` / `--capsule-dir` /
`--turn-id` etc. arg-construction sites in `workspace_relative_arg(...)`
before forwarding to `Command::arg`. Auto-strips the workspace prefix iff
present, so accidental F4-style full-path forwarding becomes a no-op rather
than a 500.

**Scope of audit**: every `Command::new(turingos_bin).arg("--<flag>").arg(<PathBuf>)`
call site in `src/web/*.rs` and `src/bin/turingos/cmd_spec.rs` should be
swept once and either confirmed clean or routed through the helper.

Tag for next sprint planning: **`atom: workspace_relative_arg helper +
sweep`**, risk class 2, est. 30 min. Not blocking F5 ship.

---

## 7. F5 ship status

- F5 fix: **LANDED** (production change + regression test + real-LLM smoke green).
- W9 / W10-R1 regression: **CLEARED**. `POST /api/spec/turn` setup call now returns 200 with a real Chinese question.
- Frontend driven-mode (W8 URL `?mode=driven`) is now unblocked end-to-end on the backend side.
- Awaits: architect review of F1–F5 cumulative diff for ship sign-off per Phase 6.3.x ship gate.
