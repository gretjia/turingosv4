# Probe — DeepSeek-V4 × TuringOS verify-retry loop vs bare single-shot on SWE-bench (real hidden-test verifier)

Date: 2026-05-28
Worktree: `turingosv4-probe-gpqa` @ branch `claude/probe-deepseek-v4-gpqa-20260528`
Author: Claude (continuation session)
Risk class: 2 (benchmark harness wire-up + judge); PR-only, not pushed.

---

## 0. Question

Does TuringOS's multi-step **verify-retry loop** beat a **bare single-shot** model
on hard, hardcoded coding tasks, when the verifier is *answer-independent* (real
SWE-bench hidden-test execution) and the loop is the *real* TuringOS kernel path
(`turingos tdma run --judge swebench`, not a Claude simulation)?

This is the coding-task companion to the GPQA probe
(`PROBE_DEEPSEEK_V4_GPQA_KARPATHY_MINIMAL_2026-05-28.md`), whose finding was that
on single-turn QA TuringOS is a byte-identical pass-through (capability Δ≈0, the
win is auditability). The hypothesis here was that a **multi-step loop + real
verifier** is where a capability difference could appear.

## 1. Honest headline

**At n=3 hermetic SWE-bench_Lite flask instances, neither arm resolved any
instance (loop 0/3, bare 0/3).** The dominant failure mode is **the patch-apply
stage, not the test stage**: deepseek-v4-pro (thinking-on) generates structurally
malformed unified diffs (wrong `@@` hunk line counts, fabricated `index` hashes,
wrong file paths, sometimes editing the test file) that GNU `patch` rejects, so
the hidden tests usually never run.

The loop's value is **real but instance-dependent and weak at this n**:

- On **flask-5063**, the loop's real apply-error feedback got the model to
  **cross the apply barrier on attempt 3** (the patch applied and the hidden
  tests actually ran — they then failed on logic). The bare single-shot **never
  crossed it**. This is a genuine pipeline-depth gain from verify-retry.
- On **flask-4045** and **flask-4992**, three retries did **not** get the model
  across the apply barrier.

So: **no resolve-rate difference (0/3 vs 0/3)**; a weak, 1-of-3 qualitative edge
in pipeline depth; loop costs ~3× requests and ~2–3.5× completion tokens.

The regime is **apply-barrier-limited**: because the model rarely produces an
*applicable* diff, the loop's distinctive lever (feeding back real failing-test
names) is mostly never reached. This hermetic-flask triad therefore *under-tests*
the loop's true test-feedback value. See §6 for what would actually exercise it.

## 2. Setup (all real, no simulation)

| Component | Value |
|---|---|
| Model (both arms) | `deepseek-v4-pro`, **thinking = on**, temperature 0.7, `max_tokens` 16000 |
| Provider path | DeepSeek direct API via local proxy `:8123` (`src/drivers/llm_proxy.py`) |
| Loop entry | `turingos tdma run --judge swebench --role meta --max-attempts-per-stage 3` (real `run_proof_with_ledger`, git tape) |
| Bare entry | `scripts/probe_bare_v4_swebench.py` — single-shot, **same** model/thinking/max_tokens |
| Verifier | official `swebench` 4.1.0 `harness.run_evaluation`, hidden-test exec in Docker (x86_64 via qemu on arm64), `--namespace none --max_workers 1 --cache_level instance`, **HF offline** (local cache) |
| Dataset | `princeton-nlp/SWE-bench_Lite` |
| Instances (hermetic, gold-gated) | `pallets__flask-5063`, `pallets__flask-4045`, `pallets__flask-4992` |

**Gold gate** (verifier validity): the official gold patch resolves each instance
`resolved=1` in the same harness (`handover/.../logs/goldsmoke_*.log`):
flask-5063 ✓ (~19s), flask-4045 ✓, flask-4992 ✓.

**Fairness anchors:**
- Bare arm's **system prompt is byte-identical** to the loop's Rust
  `SWEBENCH_SYSTEM_PROMPT` (sha256 `3a31d4e8678f…`, verified).
- Bare arm's user prompt mirrors `make_swebench_user_prompt` (same template);
  both arms send `[system, user]`, same model, same thinking-on, same
  `max_tokens`, same verifier invocation (HF-offline, same flags).
- **Shielding intact**: `leak_in_any_prompt = false` on all loop manifests;
  `gold_patch`/`test_patch` are absent from `SwebenchSampleInput` by construction
  (cmd_tdma.rs) so they can never enter any prompt. The retry feedback carries
  only failing-test NAMES or the model's own patch-apply error — never hidden
  test code or the reference patch.

## 3. Results — capability (resolve)

| Instance | Bare single-shot | Loop (≤3 attempts) | Resolved? |
|---|---|---|---|
| **flask-5063** | malformed diff → apply-fail (tests never ran) | a1 malformed → a2 malformed → **a3 APPLIED → hidden tests ran, 2 FAIL_TO_PASS still failing** | both **NO** — loop reached **test** stage, bare stuck at **apply** stage |
| **flask-4045** | apply-fail | a1 apply-fail → a2 apply-fail (wrong path `flask/blueprints.py` vs `src/flask/…`) → a3 apply-fail | both **NO** — loop stuck at apply |
| **flask-4992** | apply-fail | a1 apply-fail (malformed; tried to edit the test file) → a2 harness SSL flake† → a3 apply-fail | both **NO** — loop stuck at apply |

**Resolve rate: loop 0/3, bare 0/3.** Difference is pipeline-stage depth on 1/3
(flask-5063), not final resolution.

† flask-4992 attempt 2 hit `requests.exceptions.SSLError` during harness *setup*
(no run dir was created). Cause: the sanitized runner strips proxy vars, so a
Docker/registry HTTPS check can transiently fail without the local proxy. It is
environmental noise, not a flask-4992 hermeticity defect (its gold gate resolves;
its tests never ran that round). Attempts 1 & 3 were ordinary apply failures.

## 4. Results — cost

| Instance | Loop: requests / completion-tok / wall | Bare: requests / completion-tok |
|---|---|---|
| flask-5063 | 3 / 19,977 / 485 s | 1 / 5,876 |
| flask-4045 | 3 / 15,107 / 353 s | 1 / 5,736 |
| flask-4992 | 3 / 9,692 / 234 s | 1 / 2,538 |

- Loop = **~3× requests**, **~2–3.5× completion tokens**, **minutes-scale wall**
  (thinking reasoning + up to 3 Docker evals; Docker is x86_64-emulated on arm64
  so each eval is ~20–45 s).
- Completion-token cost is dominated by DeepSeek **reasoning** tokens (thinking-on
  produced 2.5k–7k completion tokens per call, mostly reasoning).

## 5. Results — loop behavior (does real feedback change the next attempt?)

Yes, observably — the model *reacts* to the shielded verifier feedback, but on
this triad it usually fails to fully fix the diff within 3 tries:

- **flask-5063**: a1 malformed @line 33 → feedback "your diff FAILED TO APPLY, fix
  the `@@` hunk headers" → a2 malformed @line 39 (different — the model changed the
  patch) → a3 **applies** → verdict becomes a *genuine* `hidden tests still failing
  — 2 FAIL_TO_PASS unresolved` (test_subdomain, test_host). The loop converted an
  apply-stage failure into a test-stage failure: real forward progress.
- **flask-4045**: feedback included a precise apply error ("Can't reopen file
  `flask/blueprints.py`: No such file") — yet the next attempts still mis-formatted.
- The judge's **feedback-quality fix** (see §7, bug #4) is what makes this legible:
  before it, the loop fed back a misleading offline-cache stderr tail; after it,
  the loop feeds back the real `patch: **** malformed patch at line N` signal.

**Auditability (qualitative, matches GPQA finding):** the loop emits a git
ChainTape + `per_attempt_probes.jsonl` + `manifest.json` (per-attempt judge class,
shielded reason, token counts, wall-clock, `leak_in_any_prompt=false`,
`outcome=cap-reached`). The bare arm emits only a single answer + one report.json.
TuringOS's durable edge here is the **shielded, replayable multi-attempt record**,
plus the weak apply-barrier-crossing capability gain on 1/3.

## 6. Interpretation & limits (no fake certainty)

- **n = 3, single repo family (flask), single model.** Small and narrow. Do not
  over-read. All three are hard enough that the model could not solve them.
- The **bottleneck is unified-diff formatting**, a model-level limitation, not the
  loop mechanism. Because patches rarely apply, the loop's signature lever
  (feeding back real *failing-test names* so the model fixes the *logic*) is
  almost never reached — only flask-5063 a3 got there, and only to a test
  failure. **This triad under-tests the hypothesis.**
- To actually test "does test-feedback retry beat single-shot": either (a) add a
  deterministic **diff-repair / fuzzy-apply** step so well-reasoned edits aren't
  lost to hunk-count arithmetic, or (b) use a model/regime that reliably emits
  applicable diffs, or (c) pick instances + a model where single-shot sometimes
  applies-but-fails so the loop's logic-feedback can demonstrably fix it.
- x86_64 emulation makes every Docker eval slow; this inflates wall-clock and made
  larger n impractical in one session.

## 7. Bugs found & fixed en route (Class 2, non-restricted surfaces)

The first loop run looked like "loop fails" but was **invalid** — the verifier
never executed hidden tests. Three real defects + one feedback-quality defect:

1. **Sanitized env stripped `HTTP(S)_PROXY`** → the swebench harness could not
   reach HuggingFace → dataset load 404 → `Evaluation 0/1`, no `report.json` →
   hidden tests never ran (mislabeled as "test failure").
   **Fix:** inject `HF_HUB_OFFLINE=1` + `HF_DATASETS_OFFLINE=1` in
   `SwebenchTestJudge` so the harness uses the local dataset cache, zero network.
   (`src/judges/swebench_test_judge.rs`)
2. **tdma ignored the toml's `thinking="on"`** — the Rust client serializes a
   `thinking:{type:enabled}` field but the proxy only read `enable_thinking`, and
   `cmd_tdma`'s `llm_call` hardcoded `thinking: None`. So the loop secretly ran
   thinking-**off**. **Fix:** `cmd_tdma` resolves `read_meta/blackbox_thinking`
   and threads it into `llm_call`; the proxy now honors **both** field shapes.
   (`src/bin/turingos/cmd_tdma.rs`, `src/drivers/llm_proxy.py`)
3. **`max_tokens=4000` too small for thinking-on** — reasoning_content consumed
   the whole budget, leaving the patch (`content`) empty. **Fix:** swebench path
   `max_tokens` → 16000. (`src/bin/turingos/cmd_tdma.rs`)
4. **Judge fed back a misleading error** — on a no-`report.json` harness error it
   returned the raw stderr tail (the offline-cache notice), useless to the model.
   **Fix:** `harness_failure_reason()` reads `run_instance.log` and surfaces the
   real, shielded patch-apply error ("your diff failed to apply … fix the `@@`
   hunk headers"). (`src/judges/swebench_test_judge.rs`)

All four are evaluator-adapter / driver / prompt-wiring changes; **no §6
restricted surface** (kernel/bus/wallet/sequencer/typed_tx/cas-schema) touched.
13→ judge unit tests pass; full workspace gates run in cleanup.

## 8. Evidence paths

- Loop evidence (ChainTape + probes + manifest): `handover/evidence/swebench_loop_20260528/loop_evidence_flask{5063_v4,4045,4992}/`
- Bare arm results: `handover/evidence/swebench_loop_20260528/logs/bare_flask{5063,4045,4992}.json`
- Gold gates: `handover/evidence/swebench_loop_20260528/logs/goldsmoke_flask{5063,4045,4992}.log`
- Offline-fix proof (gold resolves under sanitized+offline env): `.../logs/fixtest_offline.log`
- Invalid pre-fix run (kept as evidence of bug #1): `.../loop_evidence_flask5063/` (v1)
- Scripts: `scripts/probe_swebench_loop.sh`, `probe_bare_v4_swebench.py`, `probe_swebench_goldsmoke.sh`, `probe_swebench_expand.sh`, `start_proxy.sh`

## 9. Bottom line for the architect

- The verifier and the loop are now genuinely real and hermetic (gold-gated,
  shielded, replayable). That infrastructure is the durable deliverable.
- On this hard hermetic-flask triad, **the loop does not beat bare on resolve
  rate (both 0/3)**; it shows a real but weak, 1-of-3 edge in getting a malformed
  patch to *apply* via real feedback, at ~3× cost.
- The honest conclusion: with deepseek-v4-pro the SWE-bench bottleneck is
  **diff-formatting**, which caps both arms before the loop's test-feedback can
  matter. A fair test of the loop hypothesis needs a diff-repair step or a regime
  where single-shot patches at least apply. Recommend that as the next probe.
