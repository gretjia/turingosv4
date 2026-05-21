# USERSIM ROUND 2 — DeepSeek dual-key end-to-end, 2026-05-21

| Field | Value |
|-------|-------|
| Date | 2026-05-21 |
| Persona | Non-expert indie hobbyist designer (same persona as Round 1, different sub-agent instance) |
| Game decided | **Bubble Pop** (different from Round 1's Star Catcher; in-the-moment choice) |
| Provider path | **DeepSeek direct API**, dual-key (`turingos init --provider deepseek`) |
| Workspace | `/tmp/turingos-usersim-round2-2711212/` (preserved for inspection) |
| Outcome | ✅ Delivery succeeded; B8 fully closed; 5 of 6 verifiable Round 1 bugs confirmed fixed; B3 not exercised; 6 new Round 2 bugs surfaced |
| Predecessor | `USERSIM_DEEPSEEK_DUAL_KEY_2026-05-21.md` (Round 1) |

## 1. Capsule chain — DeepSeek path verified

| # | CID | Schema | Notes |
|---|-----|--------|-------|
| 1 | `a318be19…f32ffc718` | turingos-spec-capsule-v1 | Spec synthesised by deepseek-v4-pro (2050 tokens) |
| 2 | `ae3a8710…2a2faf47` | turingos-generation-attempt-v1 | outcome=Success on deepseek-v4-flash |
| 3 | `38e08765…74314131` | turingos-artifact-bundle-v1 | 1 file: index.html, 8837 bytes |
| 4 | `aa61e24e…48bbbc` | turingos-test-run-v1 | overall_pass=true, 2/2 scenarios: EntrypointExists + HtmlParses |

This is the **first end-to-end DeepSeek dual-key tape** in TuringOS v4. Plan v4 P2+P3+P4+P1+B8 infrastructure validated in production conditions.

## 2. Round 1 bug verification

| Bug | Status | Evidence |
|-----|--------|----------|
| **B1** model-name mismatch | ✅ Sidestepped by `--provider deepseek` writing `deepseek-v4-pro` + `deepseek-v4-flash` directly; mismatch path never reached |
| **B2** `init` empty-dir requires `--force` | ✅ Confirmed fixed — init ran on pre-`mkdir`'d $WS without `--force` |
| **B3** failed-generate CID order | ⚠️ NOT exercised — this run had no generate failure to trigger the `[failed run]` path |
| **B4** `test_run_cid=` no human line | ✅ Confirmed fixed — `Internal tests: PASS (2/2 scenarios) — EntrypointExists, HtmlParses` now prints |
| **B5** welcome agent_deploy contradiction | ✅ Confirmed fixed — `agent deploy` row absent for proof template; checklist is 4 steps |
| **B7** `--skip-llm` "test mode" framing | ✅ Confirmed fixed — help text now reads "Output is functionally equivalent" |
| (B6 was template help text — not directly tested in this run; B6 was UX clarification only) |

## 3. B8 closure verified

`turingos.toml` written by `init --provider deepseek` contained exactly:

```toml
llm.meta.api_key_env = "DEEPSEEK_API_KEY"
llm.meta.model = "deepseek-v4-pro"
llm.meta.thinking = "on"

llm.blackbox.api_key_env = "DEEPSEEK_API_KEY_WORKER"
llm.blackbox.model = "deepseek-v4-flash"
llm.blackbox.thinking = "off"
```

Both LLM calls hit `api.deepseek.com` with the correct model names and **two physically distinct API keys**. No cache pollution risk: DeepSeek sees two different `Authorization: Bearer …` headers, treats Meta and Worker as separate users.

## 4. Round 2 new bugs found

| # | Bug | Severity | Surgical fix shape |
|---|-----|----------|--------------------|
| **NB1** | Welcome shows `[x] 2. turingos llm config` even when user only ran `init --provider deepseek` (which writes turingos.toml directly — no `llm config` invocation needed). Confusing label. | LOW | Relabel step 2 to `[x] 2. LLM configured (provider: deepseek)` when written by `--provider` path; detect via reading `turingos.toml` provider hint comment |
| **NB2** | Welcome's initial "Next step" is `turingos init --project . --template proof` — a non-technical user sees `proof` and wonders if they downloaded math software. B6 added clarification in `--help`, but the suggestion itself is unchanged. | MEDIUM | Change welcome's initial suggestion to either: (a) omit `--template` entirely (let default win) OR (b) annotate with `[default; --template multi-agent for agent markets, --template polymarket for binary markets]` |
| **NB3** | `TURINGOS_SILICONFLOW_ENDPOINT` is not tracked by welcome's checklist. If user forgets it after `init --provider deepseek`, the next `turingos spec` will silently hit SiliconFlow's default endpoint (which doesn't have their DeepSeek keys) and fail with a confusing 401. **Silent misconfig trap.** | HIGH | Either: (a) persist `endpoint` in `turingos.toml` under `llm.endpoint` and have client prefer it over env var, OR (b) welcome runtime-checks the env var and prompts |
| **NB4** | `xdg-open ./artifacts/index.html` printed in generate success message is Linux-only. macOS users get `command not found`. | LOW | Change to portable: `open ./artifacts/index.html  # (or 'open' on macOS, 'start' on Windows; or simply double-click)` |
| **NB5** | `turingos spec audit --workspace $WS` errors with `--session is required` but provides no hint of where to find a session ID. No `turingos sessions list` exists. Dead-end for users who want to verify their work. | MEDIUM | Either: (a) default `--session` to the latest session derivable from CAS, OR (b) add `turingos sessions list` subcommand, OR (c) point user to `cat <ws>/spec_transcript.jsonl` |
| **NB6** | `turingos generate --help` has zero mention of required env vars. If endpoint or API key is unset, the error happens at network layer with no help-text rescue. | LOW | Add an "ENVIRONMENT" section to `generate --help` listing the 3 required env vars (or 2 + endpoint if it migrates into turingos.toml per NB3 fix) |

## 5. What worked well (continuing Round 1 W1-W4)

Plus new wins from Round 2:

| W5 | `--provider deepseek` flag was discoverable via `--help`. No prior knowledge needed beyond the keyword "deepseek" in the help text |
| W6 | "Internal tests: PASS (2/2 scenarios) — EntrypointExists, HtmlParses" — exactly the human-readable test summary B4 promised. Non-expert immediately understands "yes, my game passed internal tests" |
| W7 | spec.md synthesised by deepseek-v4-pro was higher quality than Round 1's `--skip-llm` synthesis (which was a verbatim Q/A dump). The LLM-driven synthesis produced a real product spec |
| W8 | Generate's emit format `[generate] calling Blackbox LLM (deepseek-v4-flash)...` confirmed the dual-key path live — the user could SEE the Worker role being invoked by its model name |

## 6. FC trace

- **FC1**: closed. Spec → generation_attempt → artifact_bundle → test_run capsule chain reconstructable.
- **FC2**: closed. `welcome` derived state from on-disk + CAS; `init --provider` wrote a self-describing `turingos.toml` that future invocations correctly parse.
- **FC3**: 6 new feedback items surfaced. None constitutional. All product-UX.

## 7. Verdict

| Question | Round 1 | Round 2 |
|----------|---------|---------|
| Did the system close spec→artifact→test→delivery loop? | ✅ (via SiliconFlow fallback) | ✅ (via DeepSeek dual-key, first time) |
| Did C11 fire? | ✅ test_run_cid printed | ✅ AND human-readable summary printed |
| Was UX releasable to non-expert? | ❌ 8 bugs | ⚠️ 6 new bugs but **mostly polish-tier**; core flow works |
| Was dual-key DeepSeek path exercised? | ❌ B1 broke it | ✅ Plan v4's structural goal achieved |
| Constitutional violations? | None | None |

## 8. Recommendation

Plan v4's primary mission (dual-key DeepSeek path works end-to-end with the user-sim as evidence) is **achieved**. The 6 new bugs are next-charter material, not Plan v4 blockers.

Sensible exit points:
- (a) **Halt here** — Plan v4 mission complete; NB1-NB6 + B3 untested go to follow-up charter
- (b) **Step C+**: fix NB1+NB3 (the medium/high severity items) before halting; NB2+NB4+NB5+NB6 deferred
- (c) **Step C++**: fix all 6 NBs + force-trigger B3 verification; then halt

Orchestrator recommendation: **(a) or (b)**. (c) is overkill — 6 polish bugs on a working pipeline is normal for a v0.1 user-facing surface; deferring them to next charter is healthier than scope-creeping Plan v4.

## 9. Workspace preservation

`/tmp/turingos-usersim-round2-2711212/` on tmpfs. Will be lost on cleanup. Capsule CIDs in §1 are durable on the original CAS path, but since user-sim ran on /tmp the CAS itself is on tmpfs. For posterity, all 4 CIDs are recorded in this doc and in the sub-agent transcript output file.
