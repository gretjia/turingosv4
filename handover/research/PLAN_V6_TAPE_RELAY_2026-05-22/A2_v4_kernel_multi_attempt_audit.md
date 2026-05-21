# A2 — v4 kernel multi-attempt + tape-read capability audit

| Field | Value |
|-------|-------|
| Date | 2026-05-21 evening |
| Phase | A (research) — dispatch 2 of 3 |
| Agent | Explore (read-only code audit) |
| Word count | ~1900 |

## TL;DR

TuringOS v4's tape-relay capability (pre-Atom-T) was **structurally present but functionally limited**. The system recorded attempt chains (via `parent_attempt_cid`) and test results (via `TestRunCapsule`), but the CLI (`cmd_generate.rs`) did **not** feed prior failures into subsequent LLM prompts — each attempt sent the identical prompt. The web layer (`src/web/generate.rs`) implements heuristic retry with MAX_GENERATE_ATTEMPTS=3, but that retry loop was transparent to the core generation engine. Cross-invocation continuity existed only in the CAS history; no automatic "learn and retry" loop bridged across separate CLI calls. **Atom-T (commit `d8e0fda4`) closes this gap.**

## K1 — MAX_GENERATE_ATTEMPTS & Prompt Feedback

**Finding: Single prompt per CLI invocation; no feedback loop (pre-Atom-T).**

- **Constant location**: `src/web/generate.rs:59`
  - `pub(crate) const MAX_GENERATE_ATTEMPTS: u8 = 3;`
  - **Applies only to the web layer**, not the CLI binary.

- **CLI behavior** (`cmd_generate.rs`):
  - **One LLM call per `turingos generate` invocation** (lines 242–299).
  - The prompt is built once (lines 242–257) and sent to the Blackbox model via `chat_complete_blocking()` once (line 299).
  - **No retry loop inside the CLI**: a single LLM call, no loop re-attempting with different prompts.

- **Prompt construction (pre-Atom-T)** (lines 242–254):
  ```rust
  let messages = vec![
      ChatMessage::system(blackbox_system_prompt()),
      ChatMessage::user(format!(
          "Below is the spec. Generate the working code per the rules.\n\nspec source: {source}\n\n{spec_md}"
      )),
  ];
  ```
  The prompt is **static per invocation** — identical every time the same CLI command runs.

- **Retry mechanism location**: Shifted to the web layer (`src/web/generate.rs:227`):
  ```rust
  for attempt in 1u8..=MAX_GENERATE_ATTEMPTS {
      // ... spawn `turingos generate` subprocess ...
      // ... check exit code + run heuristic verification ...
      if attempt < MAX_GENERATE_ATTEMPTS && failed {
          continue;  // retry
      }
  }
  ```
  The web handler clears prior artifacts (line 241), re-spawns the CLI, and **expects non-deterministic improvement** due to LLM temperature/stochasticity, NOT because the prompt changed.

- **Answer to K1**:
  - **Attempts per invocation**: 1 (no retry inside CLI).
  - **Does the prompt change on retry?**: **NO** (pre-Atom-T) — the prompt is identical every time.
  - **Prompt feedback mechanism**: **Absent** pre-Atom-T; **present** post-Atom-T (PR `d8e0fda4`).

## K2 — parent_attempt_cid Chaining

**Finding: Chain is built across separate CLI invocations; not used by the generation logic (pre-Atom-T).**

- **Field definition** (`src/runtime/generation_attempt.rs:33`):
  ```rust
  pub parent_attempt_cid: Option<String>,
  ```

- **Population logic** (`src/bin/turingos/cmd_generate.rs:266–289`):
  ```rust
  let mut retry_index = 0u32;
  let mut parent_attempt_cid: Option<String> = None;

  if let Ok(store) = CasStore::open(&cas_dir) {
      let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
      // ... walk attempts, sort by logical_t, link to last ...
      attempts.sort_by_key(|x| x.0);
      if let Some(last) = attempts.last() {
          retry_index = last.2 + 1;
          parent_attempt_cid = Some(last.1.clone());
      }
  }
  ```
  Reads all prior `GenerationAttemptCapsule` for the same `session_id`, sorts by timestamp, and links the current attempt to the prior one (line 288). The chain is **recorded but never read** by the LLM prompt logic (pre-Atom-T).

- **Stored in the capsule** (line 392):
  ```rust
  parent_attempt_cid,
  ```

- **Scope of chaining**: **Across separate CLI invocations**, not within a single invocation.
  - Run 1: `turingos generate --workspace <ws>` → GenerationAttemptCapsule#1, retry_index=0, parent_attempt_cid=None
  - Run 2: `turingos generate --workspace <ws>` → GenerationAttemptCapsule#2, retry_index=1, parent_attempt_cid="<cid#1>"
  - The chain is **auditable** but **not fed back** (pre-Atom-T).

- **Answer to K2**:
  - Field exists and is populated (line 288).
  - Chain spans **across separate invocations**, not within one CLI call.
  - Attempts within a web-layer retry loop are **not chained** in CAS — only the final success writes a single GenerationAttemptCapsule.

## K3 — Cross-Invocation Tape Continuity (pre-Atom-T)

**Finding: System reads prior state but doesn't use it to adjust the prompt or patch artifacts.**

When a user runs `turingos generate --workspace <ws>` twice:

- **(a) START FRESH**: Each invocation sends the **same spec.md prompt** to the LLM, regardless of prior failures.
  - Line 192–213 reads spec.md once per invocation (from disk or CAS).
  - The LLM prompt is **not influenced by prior GenerationAttemptCapsule** state.
- **(b) READ PRIOR STATE**: The code does read the prior CAS chain (lines 269–289) to populate `parent_attempt_cid`, but **only for bookkeeping**.
- **(c) REGENERATE, not PATCH**: Artifacts are always regenerated from scratch. `ArtifactBundleManifest` has a `previous_bundle_cid` field (line 39 of `artifact_bundle.rs`) but the generate logic does **not** read or use this to do incremental patching.

- **Answer to K3 (pre-Atom-T)**: Strategy is **(a) START FRESH** (same spec, same prompt, different stochastic output). **Atom-T (d8e0fda4) flipped this to (b) READ + INJECT.**

## K4 — TestRunCapsule Feedback Loop

**Finding (pre-Atom-T): Tests gate acceptance but do NOT auto-trigger a new generate attempt.**

When `TestRunCapsule` reports `overall_pass=false`:

- **Gating behavior** (`src/bin/turingos/cmd_generate.rs:574–601`):
  ```rust
  if !overall_pass {
      let rej = GenerateRejectionCapsule {
          reject_class: RejectClass::HeuristicFailed,
          public_error_summary: "generated artifacts failed spec-derived tests".to_string(),
          reason: format!("heuristic_failed:test_run_cid={}", test_run_cid),
          retryable: true,
          ...
      };
      // ... write rejection capsule to CAS ...
      return Err(GenError::WithFooter { ... });  // ← EXIT, don't retry
  }
  ```
  - The CLI **writes a rejection capsule** (line 576–589) and **exits with an error** (line 596).
  - **No automatic retry loop** at the CLI level.

- **Web layer retries** (`src/web/generate.rs:227`) **but blindly** — re-spawns the same CLI command, doesn't pass test failures back into the LLM prompt.

- **Answer to K4 (pre-Atom-T)**:
  - **(a) Just write rejection and exit** — CLI behavior (line 596).
  - Web layer implements **(b) Automatically loop back** (via heuristic retry), but **without feedback** (same spec → same prompt).
  - **Is tape actually running?** The test loop (C11 producer) is invoked (line 563: `run_and_write_test_pipeline()`), but test results do **not feed back** into the LLM. Test failures only cause a rejection capsule to be written and the CLI to exit; the web layer retries the whole pipeline blindly.

**Post-Atom-T**: `read_prior_rejection_feedback()` reads the latest rejection. For `HeuristicFailed`, it additionally extracts `test_run_cid` from `reason`, loads the `TestRunCapsule`, and surfaces failed-scenario names. The next CLI invocation's prompt now carries that diagnostic.

## K5 — Capsule Cross-Reference Graph

```
SpecCapsule (spec_capsule.rs)
  ↓ [written by cmd_spec.rs]

GenerationAttemptCapsule (generation_attempt.rs:33)
  spec_capsule_cid ← SpecCapsule CID
  parent_attempt_cid ← prior GenerationAttemptCapsule (optional)
  raw_output_cid ← raw LLM response (EvidenceCapsule)

ArtifactBundleManifest (artifact_bundle.rs:38–39)
  spec_capsule_cid ← SpecCapsule CID
  generation_attempt_cid → GenerationAttemptCapsule (required)
  previous_bundle_cid ← prior ArtifactBundleManifest (optional)
  files[].cid → individual file bytes in CAS

TestRunCapsule (test_run.rs:38–39)
  artifact_bundle_cid → ArtifactBundleManifest (required, by CID reference)
  test_scenario_set_cid → TestScenarioSet (hidden-oracle, not exposed to LLM)

GenerateRejectionCapsule (rejection_capsule.rs:41)
  spec_capsule_cid ← SpecCapsule (optional)
  generation_attempt_cid → GenerationAttemptCapsule (optional)
  test_run_cid [in reason field as "heuristic_failed:test_run_cid=<hex>"]
  private_diagnostic_cid ← raw error/failure bytes (optional)
```

**Links READ vs WRITTEN (pre-Atom-T)**:

| Link | READ | WRITTEN | Where |
|------|------|---------|-------|
| SpecCapsule → latest | ✓ YES | ✓ YES | cmd_spec.rs writes; cmd_generate.rs reads spec_capsule_cid |
| GenerationAttemptCapsule → parent | ✓ YES (chain enumerate) | ✓ YES | cmd_generate.rs reads chain (line 269–289), writes link (line 288) |
| ArtifactBundleManifest → gen_attempt | ✓ YES (links to) | ✓ YES | cmd_generate.rs writes (line 548), web reads for response (line 456) |
| ArtifactBundleManifest → previous_bundle | ✗ NOT READ | ✓ YES | cmd_generate.rs writes (line 540), never read; not used for patching |
| TestRunCapsule → artifact_bundle | ✓ YES | ✓ YES | test_run.rs reads bundle by CID (line 87), writes test results (line 285) |
| TestRunCapsule → scenario_set | ✗ HIDDEN | ✓ YES | Hidden-oracle by design |
| GenerateRejectionCapsule → attempt | ✗ MOSTLY (pre-Atom-T) | ✓ YES | Written when generation fails; **read by Atom-T post-d8e0fda4** |

**Key asymmetry**: `ArtifactBundleManifest.previous_bundle_cid` is **written but never read**. This is by design — it's for auditing provenance, not for incremental patching.

## K6 — cmd_replay Capability

**Finding: Offline replay reconstructs the CAS-only attempt chain, not prompts/LLM calls.**

- **`--offline` mode** (`cmd_replay.rs:80–143`): Calls `runtime::replay::reconstruct_session()` (line 117), which reads CAS capsules and builds a step-by-step transcript. Verifies all cross-CID references resolve.
- **Outputs CAS state chain**, not the LLM prompts or the reasoning behind each generation attempt.
- **LLM prompt visibility**: NO — the prompts are not stored in CAS. Only the final artifact and test results are recorded. The `prompt_hash` field (SHA256) is stored (generation_attempt.rs:29), but not the full prompt text.

- **Answer to K6**: Offline replay can show "Attempt 1 CID, Attempt 2 CID, ..., final success CID" via the parent_attempt_cid chain. But **cannot show what the LLM was asked** or **how the prompt changed**. The byte-level proof that Atom-T worked is via `prompt_hash` comparison across attempts (which IS stored).

## K7 — Spec-Grill Multi-Attempt

**Finding: Spec modes have internal LLM loops but no automatic retry on heuristic failure.**

- **Static mode** (default): Eight hardcoded questions → user answers (stdin or `--answers-file`) → optionally synthesize via LLM (`--skip-llm` disables) → write spec.md + CAS capsule. **One synthesis LLM call** if synthesis is enabled (no retry loop).
- **Driven mode** (`--mode driven`): W6 atom — LLM-driven turn loop. Meta model asks questions (multiple turns). **No implicit retry**; just iteration to a terminal state.

- **Answer to K7**: Neither spec mode has a "retry the prompt with feedback" mechanism like the generate layer has (post-Atom-T).

## K8 — Observable Signals for "Tape Really Running"

Given the current code, here are signals a test can check:

1. **Attempt chain in CAS exists and is readable**:
   - Run `turingos replay --offline --workspace <ws> --session <id>` post-failure.
   - Verify `GenerationAttemptCapsule` chain: `attempt_1.parent_attempt_cid == None`, `attempt_2.parent_attempt_cid == <cid_1>`, etc.

2. **Test results are recorded and visible**: After a failed test, verify `TestRunCapsule` with `overall_pass=false`, and `TestRunCapsule.artifact_bundle_cid` points to the artifact that failed.

3. **Rejection capsule is written on test failure**: After a failed test, verify `GenerateRejectionCapsule` exists with `reject_class=HeuristicFailed` and `reason=heuristic_failed:test_run_cid=<cid>`.

4. **Atom-T stderr signal**: Look for `[generate] tape-relay: feeding prior rejection diagnostics into LLM prompt (attempt #N)` in stderr on retry.

5. **prompt_hash differs across attempts**: This is the byte-level proof. Pre-Atom-T → identical. Post-Atom-T → different (because rejection feedback is prepended).

6. **LLM response acts on the feedback**: Hardest to test mechanically. Best proxy: attempt N produces an artifact that addresses the failure mode called out in the rejection capsule of attempt N-1.

## Honest Assessment

**Pre-Atom-T (before commit `d8e0fda4`)**: v4 occupied the middle ground:

```
Fancy LLM Wrapper  ←——————→  Real Turing Machine
         |                           |
      v4 HERE                   (hypothetical)
```

- ✓ Records attempt history
- ✓ Records test results
- ✓ Retries blindly (web layer)
- ✓ Rejects broken artifacts
- ✗ No feedback loop
- ✗ No conditional retry
- ✗ No cross-invocation learning
- ✗ No state patching

**Post-Atom-T**: v4 now does READ-INJECT feedback for the rejection capsule. Empirically validated on a 3-attempt chain with byte-different prompt_hash on each (see `B_ATOM_T_DESIGN_AND_RESULTS.md`).

**What a "real tape-relay" looks like (achieved post-Atom-T)**:
1. Attempt 1 generates artifact + runs tests → `TestRunCapsule.overall_pass=false` (or any rejection class).
2. System reads test results / rejection diagnostics from CAS and constructs a new prompt: "Your prior attempt had issues: {failures}. Regenerate fixing these problems."
3. Attempt 2 LLM receives a **different prompt** informed by Attempt 1's failure (verified: `prompt_hash` differs).
4. Loop continues until `overall_pass=true`.

**Verdict**: post-Atom-T v4 is a real (small) Turing machine for the rejection-feedback dimension. The tape is now both recorded AND read.
