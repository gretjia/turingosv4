# B — Atom-T design + empirical validation results

| Field | Value |
|-------|-------|
| Date | 2026-05-21 evening → 2026-05-22 morning |
| Phase | B (synthesis + impl + validation) |
| Commits | `d8e0fda4` (Atom-T impl), `0e190c95` (final report) |
| Word count | ~1500 |

## Why Atom-T

Phase A research (A1+A2) converged on a brutal finding: pre-Atom-T v4 wrote `parent_attempt_cid` chains in CAS but **never read them back** into the LLM prompt. The architect's directive — "agent 可以在 tape 上接力完成任务，图灵机真的在运转" — was architecturally impossible without closing this read-side gap.

A1 (minif2f historical study) confirmed the canonical externalization rule: every LLM call = 1 Attempt Node; chain via `parent_attempt_cid`. But the rule never required feedback injection; it only required externalization. Minif2f-era smokes were single-attempt or n1-budget; attempt-2-with-prior-attempt-feedback was never exercised.

A2 (code audit) confirmed cmd_generate.rs's prompt construction (lines 242–254 pre-Atom-T) was invariant across retries. The only thing that changed across attempts was `retry_index` and `parent_attempt_cid` in the capsule metadata — the LLM never saw any difference.

Conclusion: build Atom-T (minimum architectural change closing the read side) **before** running the test matrix, so the matrix can validate the relay actually fires.

## Design

### Helper function

In `src/bin/turingos/cmd_generate.rs`:

```rust
/// TRACE_MATRIX FC1-N4 / FC2-N18: Read prior rejection diagnostics from CAS
/// and format them for inclusion in the LLM prompt. Returns Some(feedback_text)
/// if a usable prior rejection exists for this session; None otherwise.
fn read_prior_rejection_feedback(
    workspace: &Path,
    session_id: &str,
) -> Option<String> {
    let cas_dir = workspace.join("cas");
    let store = CasStore::open(&cas_dir).ok()?;

    // Find latest GenerateRejectionCapsule for this session by logical_t.
    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
    let mut candidates = Vec::new();
    for cid in cids {
        let meta = store.metadata(&cid)?;
        if meta.schema_id.as_deref() == Some(GENERATE_REJECTION_CAPSULE_SCHEMA_ID) {
            let bytes = store.get(&cid).ok()?;
            let cap: GenerateRejectionCapsule = serde_json::from_slice(&bytes).ok()?;
            if cap.session_id == session_id {
                candidates.push((cap.logical_t, cap));
            }
        }
    }
    candidates.sort_by_key(|x| x.0);
    let latest = candidates.into_iter().last()?.1;

    let mut feedback = String::from(
        "=== PRIOR ATTEMPT FEEDBACK (relayed from CAS tape) ===\n\n"
    );
    feedback.push_str(&format!(
        "Your previous attempt for this same session FAILED.\n\
         Failure class: {:?}\n\
         Public summary: {}\n\
         Reason: {}\n\n",
        latest.reject_class, latest.public_error_summary, latest.reason
    ));

    // For HeuristicFailed: dig into linked TestRunCapsule for failed-scenario names.
    if matches!(latest.reject_class, RejectClass::HeuristicFailed) {
        if let Some(idx) = latest.reason.find("test_run_cid=") {
            let cid_hex = &latest.reason[idx + "test_run_cid=".len()..];
            let cid_hex = cid_hex.split_whitespace().next().unwrap_or(cid_hex);
            if let Some(failed_scenarios) = read_failed_scenarios_by_cid(&store, cid_hex) {
                if !failed_scenarios.is_empty() {
                    feedback.push_str("Specific failed test scenarios:\n");
                    for (name, detail) in failed_scenarios {
                        feedback.push_str(&format!("  - {}: {}\n", name, detail));
                    }
                    feedback.push_str("\n");
                }
            }
        }
    }

    feedback.push_str(
        "INSTRUCTIONS: This is your second (or later) chance. Please:\n\
         1. Re-read the spec below carefully.\n\
         2. Address the specific failure mode above.\n\
         3. Produce a CORRECTED file set in the same `### File: <path>` + fenced-code-block format.\n\
         4. Do not repeat the prior mistake.\n\n\
         === END FEEDBACK ===\n\n"
    );
    Some(feedback)
}
```

Plus a sibling `read_failed_scenarios_by_cid()` that parses the hex CID, loads the `TestRunCapsule`, and returns `(scenario_name, detail)` for each failing scenario.

### Prompt construction modification

```rust
let prior_feedback = read_prior_rejection_feedback(&workspace, &session_id);
let user_msg = if let Some(fb) = prior_feedback {
    eprintln!(
        "[generate] tape-relay: feeding prior rejection diagnostics into LLM prompt (attempt #{})",
        retry_index
    );
    format!(
        "{fb}Below is the spec. Generate the working code per the rules.\n\nspec source: {source}\n\n{spec_md}"
    )
} else {
    format!(
        "Below is the spec. Generate the working code per the rules.\n\nspec source: {source}\n\n{spec_md}"
    )
};
let messages = vec![
    ChatMessage::system(blackbox_system_prompt()),
    ChatMessage::user(user_msg),
];
```

### Constraints honored

- 0 new Cargo deps (no schema lib, no new HTTP, no spinner crate)
- 0 schema changes (reads existing `GenerateRejectionCapsule` + `TestRunCapsule` + `TestScenarioResult` fields)
- 0 Trust Root churn (no `Cargo.toml` / `Cargo.lock` modifications)
- First-attempt prompt unchanged (backward-compatible — `None` branch returns original)

### Implementation cost

| Item | LoC |
|------|-----|
| `read_prior_rejection_feedback()` | ~55 |
| `read_failed_scenarios_by_cid()` | ~20 |
| Prompt-construction branch | ~10 |
| Imports + minor refactor for `session_id`/`retry_index` ordering | ~10 |
| **Total source** | **~95** |
| `tests/tape_relay_feedback_loop.rs` (4 unit tests) | 383 |

Source-side delta on `cmd_generate.rs`: +141 / -19 = +122 net lines.

## Empirical validation

### Setup

Workspace: `/tmp/turingos-relay-direct-1779387733/` (preserved on tmpfs at time of writing).

```
provider:    deepseek
spec mode:   static --skip-llm
spec intent: 8 short answers describing a click-counter game with localStorage
```

### Sequence

| Run | TURINGOS_SILICONFLOW_ENDPOINT | DEEPSEEK key | Expected | Got |
|-----|-------------------------------|-------------|----------|-----|
| 1 | `http://127.0.0.1:1/v1/chat/completions` (force ConnRefused) | valid | LlmApiError + rejection capsule | ✓ rejection_cid=`b3b27c92…` |
| 2 | (unset → default SiliconFlow) | DeepSeek key (mismatch) | 401 + rejection + Atom-T should fire (1 prior rejection) | ✓ stderr line "tape-relay: feeding prior rejection diagnostics" appeared, rejection_cid=`0bd6867a…` |
| 3 | `https://api.deepseek.com/v1/chat/completions` | DeepSeek key (correct) | Atom-T should fire (2 prior rejections), LLM should succeed | ✓ stderr line appeared, ✓ ArtifactBundle written, ✓ TestRunCapsule PASS 2/2, ✓ 2442-byte HTML |

### CAS state after Run 3

```
attempt capsules:  3
rejection caps:    2
bundle caps:       1
test_run caps:     1
artifacts/index.html: 2442 bytes (Click Counter game with localStorage)
```

### Byte-level prompt-hash evidence

Reading each `GenerationAttemptCapsule` from CAS (via `git --git-dir=$WS/cas/.git cat-file blob <oid>`):

| Attempt | retry_index | parent_attempt_cid | outcome | prompt_hash |
|---------|-------------|-------------------|---------|-------------|
| 1 | 0 | `None` | `LlmApiError` | `0711ab17e96ecff8e285c30ec1ed0fc57294f9090814f4d3f16de1ae847bd513` |
| 2 | 1 | `297769b4fbaeeb97a1a9e6d403523649a85310fa52eb3c0a764c1f4de1ddc2bd` (= A1's CID) | `LlmApiError` | `ebee7de4c45019e0c6cb219f01fcfea067c4250c5a2c5162a306875286f268f0` |
| 3 | 2 | `9812ba58d9a1bffd4d0f71e85ed4436c9166b625e9c374183df3247a5f3790bc` (= A2's CID) | **`Success`** | `680c4b03cc9ed4bf7cac1f4f8069d7bbbe9ecd265a44ce6fa51ffa017ede247a` |

**Every prompt_hash differs**. This is the byte-level signature that Atom-T fed different content into the LLM on each attempt. Pre-Atom-T, all three would have been identical.

### Stderr signal

Captured verbatim from Runs 2 and 3:

```
[generate] tape-relay: feeding prior rejection diagnostics into LLM prompt (attempt #1)
[generate] tape-relay: feeding prior rejection diagnostics into LLM prompt (attempt #2)
```

(The `attempt #N` is `retry_index` — 0-based, so retry_index=1 = "attempt #1" of the relay, which is the second generate invocation.)

### Rejection capsule content (what Atom-T actually relayed)

Run 1's rejection capsule (what was fed into Run 2's prompt):

```json
{
  "schema_id": "turingos-generate-rejection-v1",
  "session_id": "default",
  "spec_capsule_cid": "f36a903e…",
  "generation_attempt_cid": "297769b4…",
  "triage_attempted": true,
  "reject_class": "LlmApiError",
  "public_error_summary": "HTTP transport error: error sending request for url (http://127.0.0.1:1/v1/chat/completions)",
  "reason": "llm_api_error",
  "private_diagnostic_cid": null,
  "retryable": true,
  "world_head_unchanged": true,
  "logical_t": 1779387733
}
```

This content was injected into Run 2's LLM prompt as a `=== PRIOR ATTEMPT FEEDBACK (relayed from CAS tape) ===` block. Run 2 then failed for a different reason (401) and wrote a SECOND rejection capsule. Run 3 saw the latest rejection (the 401), incorporated it, and succeeded.

### Final artifact (proof of life)

`/tmp/turingos-relay-direct-1779387733/artifacts/index.html` (2442 bytes):

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <title>Click Counter</title>
  …
  <button class="click-button" id="clickButton">Click</button>
  …
  const STORAGE_KEY = 'turingos_click_counter';
  …
</html>
```

The page is a working click-counter game with localStorage persistence — i.e. the spec was actually realized.

## Reproducible recipe

```bash
cargo build --bin turingos

WS=/tmp/turingos-relay-replay-$(date +%s)
mkdir -p $WS
target/debug/turingos init --project $WS --provider deepseek --force

cat > $WS/answers.json << 'EOF'
["A tiny click counter game.","Like an old web counter widget.","Counter persists in localStorage.","User opens page, sees big button labeled Click, clicks to increment.","Rapid clicks should still work.","No multiplayer, no animations.","30 days from now: 20 people clicked at least once.","OK: single-page click counter with localStorage."]
EOF

target/debug/turingos spec --workspace $WS \
  --answers-file $WS/answers.json --lang en --mode static --skip-llm

# Run 1 — force LlmApiError
TURINGOS_SILICONFLOW_ENDPOINT=http://127.0.0.1:1/v1/chat/completions \
  target/debug/turingos generate --workspace $WS

# Run 2 — correct endpoint, Atom-T should fire
set -a; source /home/zephryj/projects/turingosv4/.env; set +a
export TURINGOS_SILICONFLOW_ENDPOINT="https://api.deepseek.com/v1/chat/completions"
target/debug/turingos generate --workspace $WS 2>&1 | grep "tape-relay"
# Expected: [generate] tape-relay: feeding prior rejection diagnostics into LLM prompt (attempt #1)
```

## Outstanding / next-charter items

| Item | Why deferred | Trigger to revisit |
|------|--------------|---------------------|
| HeuristicFailed production proof | Atom-T handles it in code + 4 unit tests pass, but no production C11-rejected scenario was exercised end-to-end (X3 pilot succeeded, didn't trigger HeuristicFailed) | First real user-reported C11 test failure in a session that gets retried |
| Web-layer surface | `src/web/generate.rs` MAX_GENERATE_ATTEMPTS=3 loop benefits from Atom-T silently, but the web client doesn't see the relay signal | Web client wants visible "I'm using prior attempt info" UX |
| Cross-session relay | Atom-T scopes by session_id; cross-session (e.g. "improve v2 of this game") is a different feature | User requests provenance chain across spec_capsule_cids |
| Difficulty taxonomy recalibration | A3 was too pessimistic for 2026-era LLMs | Future test matrices reuse A3 task list |
| Constitutional gate | A test that asserts `prompt_hash` differs across attempts when prior rejection exists | First regression where Atom-T silently breaks |

## Closing note

Plan v6 took the architect's question — "is this a real Turing machine or just an LLM wrapper" — and made it answerable empirically by:

1. Acknowledging honestly via Phase A that pre-Atom-T v4 was the wrapper (write-only tape).
2. Building the minimum architectural change (Atom-T, ~95 LoC, 0 deps) to close the read-side gap.
3. Validating end-to-end via forced-failure + real DeepSeek that the relay actually fires and the LLM receives different content each attempt (byte-level proof via prompt_hash).

The cost of doing this honestly: 3 research sub-agent dispatches + 1 Atom-T sub-agent + 1 forced-failure direct test + 1 X3 pilot. ~3 hours of orchestrator time + ~$0.05-0.20 worth of DeepSeek tokens. The Turing machine ran. The tape was read.
