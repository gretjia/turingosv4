# USERSIM ROUND 3 — Validate Atom-K + B3 fix verification, 2026-05-21

| Field | Value |
|-------|-------|
| Date | 2026-05-21 |
| Persona | Non-expert indie hobbyist (third instance, fresh) |
| Game decided | Pixel Farm Defender (tower defense; different from R1 Star Catcher / R2 Bubble Pop) |
| Provider path | DeepSeek dual-key via `turingos init --provider deepseek` |
| Workspace | /tmp/turingos-usersim-round3-2746312/ |
| Outcome | NB3 fix confirmed; B3 regression discovered (PR #66 incomplete); NEW bug: `llm config --help` doesn't show help |
| Predecessor | USERSIM_ROUND2_DEEPSEEK_END_TO_END_2026-05-21.md |

## What worked (validation success)

| Item | Status | Evidence |
|------|--------|----------|
| NB3 fix (Atom-K) | ✅ | `welcome` prints "⚠ TURINGOS_SILICONFLOW_ENDPOINT overridden to: https://api.deepseek.com/v1/chat/completions" + `(default: ...)` companion line |
| B2 (init empty dir without --force) | ✅ | Still gives clean error on non-empty dir |
| B4 (Internal tests PASS line) | ✅ | `test_run_cid=...` followed by "Internal tests: PASS (2/2 scenarios) — EntrypointExists, HtmlParses" |
| B5 (no agent_deploy in welcome) | ✅ | Welcome output has 4 onboarding steps, no agent deploy row |
| B7 (--skip-llm reframe) | ✅ | "Use when the LLM provider is unavailable or to skip the synthesis pass" — no "test mode" framing |
| DeepSeek dual-key end-to-end | ✅ | 2nd successful real run; capsule chain complete; 8837-byte HTML delivered |
| C11 producer fires | ✅ | test_run_cid + overall_pass=true on success |
| Mission B (deliberate failure path) | ✅ | HTTP 401 triggered cleanly; B3 evidence captured |

## What's broken (Round 3 findings)

### X1 — B3 fix from PR #66 is incomplete

PR #66 claimed:
> B3: failed-generate CIDs route to stderr w/ [failed run] prefix after error message (was before, looked like success)

Round 3 stderr capture (verbatim, with deliberate invalid API key):

```
1. [generate] calling Blackbox LLM (deepseek-v4-flash)...
2. [failed run] generate did not deliver — see error below
3. [failed run] generation_attempt_cid=497aacee...a673b
4. [failed run] rejection_cid=53f95dd3...44818c7
5. turingos generate: HTTP 401 from SiliconFlow: {...invalid api key...}
```

Lines 3-4 (CIDs) are BEFORE line 5 (the error message). Line 2's preamble says "see error below" — but the actual error is BELOW the CIDs that the preamble was supposed to introduce. The fix added the `[failed run]` prefix correctly, but did NOT reorder.

**Severity**: MEDIUM. Non-expert reads stderr top-to-bottom; sees confusing CIDs before understanding what went wrong. Same UX problem B3 originally identified, just with extra prefix decoration.

### X2 — `turingos llm config --help` silently runs the command (NEW)

```
$ turingos llm config --help
# Writes ./turingos.toml to current working directory
# No help text printed at all
```

The DEEPSEEK + ANTHROPIC + OPENAI example blocks added by PR #70 are unreachable through the `--help` UX surface. They exist in FULL_HELP but the `config` subcommand handler doesn't honor `--help`.

**Severity**: HIGH. Side effect (writing turingos.toml to CWD) is worse than the missing help text — a user running `turingos llm config --help` from `/tmp` or `~/Downloads` could pollute unrelated directories.

### Other observations

- First generate attempt returned empty raw LLM response (0-byte `generate_raw_response.txt`). Auto-retry succeeded. The CLI should probably say "empty response, retrying..." rather than silently parse-failing on the first attempt.

## FC trace

- FC1: closed (proposal externalization works end-to-end with DeepSeek dual-key)
- FC2: closed (workspace reconstructable from CAS)
- FC3: 2 new feedback items (X1, X2). All product-UX, no constitutional violations.

## Verdict

NB3 fix from PR #70 is solid and confirmed working in a real run. Two new actionable bugs found, both surgical fixes (estimated 30 LoC total). Plan v4 mission status: dual-key DeepSeek path verified twice (R2 + R3), all P-atoms holding, only polish-tier UX bugs remain.

## Workspace evidence

`/tmp/turingos-usersim-round3-2746312/` on tmpfs (lost on reboot). CAS index captured 10 capsules across spec / generation_attempt / rejection / artifact_bundle / test_run schemas. All 4 verifiable CIDs preserved in this doc.
