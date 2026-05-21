# A3 — v4 CLI wrap-points for TUI integration

| Field | Value |
|-------|-------|
| Date | 2026-05-21 |
| Phase | A (research) — dispatch 3 of 3 |
| Agent | Explore (read-only) |
| Sources | `/home/zephryj/projects/turingosv4/` working tree at HEAD `796fb986` |
| Word count | ~1700 |

## TL;DR

The TuringOS v4 CLI surface can be wrapped by a TUI with **mixed in-process/subprocess integration**. Four of five target subcommands (welcome, init, llm, spec) can be called in-process by invoking their entry functions directly and capturing structured outputs via library-ized capsule types (spec_capsule, artifact_bundle, test_run). The generate subcommand requires subprocess invocation due to blocking LLM calls. Output is presently line-buffered text (print!); refactoring to structured events via a `--json-events` mode would be cleaner but not essential for Phase 1.

## Subcommand Integration Map

| Subcommand | Entry Function | Args Shape | Return Shape | In-Process? | Notes |
|---|---|---|---|---|---|
| **welcome** | `cmd_welcome::run(&[String])` @ src/bin/turingos/cmd_welcome.rs:64 | `[--workspace <PATH>]` | `ExitCode` | **YES** | Read-only filesystem inspection. Returns `WorkspaceStatus` struct (lines 45–61) w/ 4 checklist booleans + spec_capsule_cid. Pure data, no side effects. |
| **init** | `cmd_init::run(&[String])` @ src/bin/turingos/cmd_init.rs:530 | `--project <PATH> [--template proof|polymarket|multi-agent] [--provider siliconflow|deepseek] [--force]` | `ExitCode` | **YES** | Filesystem write only (turingos.toml, genesis_payload.toml, agent_pubkeys.json). No network. No struct return; emits status via println!. TUI can capture exit code + suppress output. |
| **llm config** | `cmd_llm::run(&[String])` @ src/bin/turingos/cmd_llm.rs:238 | `config --workspace <PATH> [--meta-model <ID>] [--blackbox-model <ID>] [--api-key-env <ENV>] [--meta-thinking on|off]` | `ExitCode` | **YES** | Write to workspace-local turingos.toml. Entry point dispatches to `run_inner()` (line 284) → `write_config()` (line 360). Helper functions `read_meta_api_key_env()` (line 488), `read_blackbox_api_key_env()` (line 498) are `pub(crate)` and callable from other subcommands. |
| **spec** | `cmd_spec::run(&[String])` @ src/bin/turingos/cmd_spec.rs:185 | `--workspace <PATH> [--answers-file <PATH>] [--lang zh|en] [--mode static|driven] [--skip-llm]` | `ExitCode` | **MIXED** | Dispatcher calls either `run_inner()` (static mode; line 199) or `run_driven_mode()` (driven mode; line 250). Writes spec.md + spec_transcript.jsonl + CAS EvidenceCapsule (via `spec_capsule::write_spec_capsule()` @ src/runtime/spec_capsule.rs:83). LLM call is blocking; interactive stdin read happens in-process (lines 275–276). |
| **generate** | `cmd_generate::run(&[String])` @ src/bin/turingos/cmd_generate.rs:145 | `--workspace <PATH> [--from-capsule] [--max-files <N>] [--emit-transcript]` | `ExitCode` | **NO** | Calls blocking LLM via `chat_complete_blocking()` (line 299). Writes artifacts + CAS capsules (GenerationAttemptCapsule, ArtifactBundleManifest, TestRunCapsule). TUI must subprocess-out due to LLM blocking behavior + no cancellation mechanism. |

> Note: A3 originally said "must subprocess for generate." Atom-W wizard implementation can in-process the call but the TUI thread will block during the LLM call (~30-60 sec). For Phase-1 this is acceptable; if mid-generation cancellation becomes a requirement, switch to subprocess later.

## Stateful Input Collection Map

| Input | Current CLI Path | How TUI Bridges It | Can Accept via Rust Arg? |
|---|---|---|---|
| **Workspace path** | `--workspace <PATH>` or `--project <PATH>` (init only) | Pass as first String in `&[String]` args to entry fn | **YES** — every subcommand accepts this as `--workspace` except init (uses `--project`) |
| **API keys** | Env vars: `DEEPSEEK_API_KEY`, `DEEPSEEK_API_KEY_WORKER`, `SILICONFLOW_API_KEY` | TUI must set env vars before calling subcommand; no way to pass keys as Rust args (intentional: keys never touch disk). See cmd_llm.rs:488–498 which reads env-var NAME from toml, then looks up value in `std::env::var()`. | **NO** — keys are env-var-only for security. Pass key names via toml only. TUI sets `std::env::set_var()` after collecting via `stty -echo` prompt. |
| **Endpoint URL** | Env var: `TURINGOS_SILICONFLOW_ENDPOINT` | TUI reads from `siliconflow_client::endpoint()` (src/bin/turingos/siliconflow_client.rs); can override via env before subcommand call. | **NO** — env-var-only. TUI sets after provider choice. |
| **Provider preset** | `--provider deepseek|siliconflow` (init only) | Passed as arg to init entry fn. For llm config, resolved from turingos.toml if already written. | **YES** — string arg to init; read from toml thereafter. |
| **8 spec answers** | `--answers-file <PATH>` (JSON array) or stdin | TUI can write JSON file (preferred — Karpathy-lens persistence) or pipe stdin to entry fn. Stdin is read at line 275 via `interactive_gather(&questions)?`. | **PARTIAL** — `--answers-file` is full arg support; stdin requires TUI to hijack stdio before call. |

## Streaming Output Map

| File:Line | Output Purpose | Current Form | How TUI Surfaces It |
|---|---|---|---|
| cmd_welcome.rs:147–261 | Onboarding checklist + next-step hint | Printf text | Parse exit code; TUI renders checklist pane from in-process `WorkspaceStatus` struct (not from stdout). |
| cmd_init.rs:398–457 | Confirmation + next-step instructions | Printf text | Suppress stdout; TUI renders its own scaffold confirmation. |
| cmd_llm.rs:370–383 | Config confirmation + instructions | Printf text | Suppress stdout; TUI renders confirmation in modal. |
| cmd_spec.rs:333 | `[spec] calling Meta LLM (...)...` | Eprintln stderr | Capture via stderr redirect; parse model name + progress verb for TUI progress pane. |
| cmd_spec.rs:365–376 | Spec completion summary (artifact paths + CAS CID) | Printf text | Suppress stdout; read results from CAS (spec_capsule::latest_spec_capsule_cid() @ src/runtime/spec_capsule.rs:110). |
| cmd_generate.rs:298 | `[generate] calling Blackbox LLM (...)...` | Eprintln stderr | Capture via subprocess stderr (no in-process call). |
| cmd_generate.rs:435, 557, 571 | CIDs: generation_attempt_cid, artifact_bundle_cid, test_run_cid | Eprintln / println | Parse from subprocess stderr/stdout; store for artifact browsing. |
| cmd_generate.rs:573 | Test run summary (pass/fail per scenario) | Printf text + format_test_run_summary() | Read TestRunCapsule from CAS (src/runtime/test_run.rs:36–43); render as table in TUI. |

**Existing structured event patterns:** No `emit_event!` macro exists. Status quo is line-buffered text. Best path for Phase 1: **subprocess capture + structured CAS reads** (not text parsing). The spec_capsule, artifact_bundle, and test_run CAS types are fully serializable (serde JSON); TUI can read these from disk post-completion for display, avoiding stdout parsing.

> Karpathy-lens counter (see `B_KARPATHY_LENS_CRITIQUE.md` §1 Violation 3): polling CAS in a background thread creates a hidden data dependency between TUI and cmd_generate. Rejected. Phase 1 just lets cmd_generate's existing eprintln! lines flow through to the user's terminal.

## Structured Data Substrate

**Available for TUI rendering (no stdout parsing needed):**

- **WorkspaceStatus** (cmd_welcome.rs:45–61): 4 checklist booleans (init_done, llm_configured, spec_done, artifacts_done) + spec_capsule_cid string + agents_count + requires_agent_deploy flag. Computed on-demand from filesystem inspection.

- **turingos.toml entries** (cmd_llm.rs:471–530): Provider name, meta_model, blackbox_model, api_key_env name, thinking configs. Read via `read_meta_model()`, `read_blackbox_model()`, etc. (all pub(crate)).

- **SpecCapsuleSchemaID** (src/runtime/spec_capsule.rs:44): "turingos-spec-capsule-v1" — lets TUI find spec capsules in CAS index without scanning bytes. Read via `spec_capsule::latest_spec_capsule_cid()` (line 110).

- **ArtifactBundleManifest** (src/runtime/artifact_bundle.rs:34–44): schema_id, session_id, spec_capsule_cid, generation_attempt_cid, entrypoint, files array (each with path, cid, mime, role), bundle_size_bytes_total, created_at_logical_t. Deserialize from CAS to render artifact browser pane.

- **TestRunCapsule** (src/runtime/test_run.rs:36–43): artifact_bundle_cid, test_scenario_set_cid, results array (TestScenarioResult: scenario, pass, detail), overall_pass, logical_t. Deserialize from CAS to render per-scenario pass/fail table.

- **GenerationAttemptCapsule** (src/runtime/generation_attempt.rs line 22+): session_id, retry_index, logical_t, outcome (enum AttemptOutcome), parent_attempt_cid. Tracks retry chain for multi-attempt workflows.

All capsules are written to CAS by subcommands (spec.rs line 350, generate.rs lines 313–327) with schema_id tagged in the CAS index; TUI can list by schema_id and deserialize.

## Recommended TUI Integration Approach (adopted in Atom-W)

**In-process for all 5 commands. Synchronous (blocking on generate is OK for Phase 1).**

1. `welcome` — in-process call; emits its own stdout
2. `init` — in-process call with `--provider` + `--force` flags synthesized from wizard inputs
3. `llm config` — NOT called by wizard; `init --provider deepseek` already writes the toml correctly
4. `spec` — in-process call with `--answers-file` pointing at wizard-collected JSON
5. `generate` — in-process call; blocks the TUI thread during the LLM call (~30-60 sec); cmd_generate's eprintln! lines stream through to user's terminal naturally

**Rationale:**
- Avoids stdout parsing brittleness (text format can change without versioning).
- No subprocess fork = no Windows compatibility issues with shell quoting.
- In-process calls keep the UI responsive for welcome/init/llm; synchronous block during generate is acceptable since user is waiting for the game anyway.
- No refactor to v4 cmd_*.rs files needed; TUI uses existing public surface.

## Refactor Cost Estimate

**Zero refactor cost to existing v4 code.** All five subcommands are callable in their current form. The TUI integration layer needs:

- Thin wrapper around each entry fn signature: `pub(crate) fn run(&[String]) -> ExitCode`.
- Minimal `pub(crate)` visibility additions if any cmd_*.rs entry isn't already crate-visible.
- Bare ANSI escape codes for color (no `gag` / `console` / `crossterm` crates).
- Bare `stty -echo` via `std::process::Command` for password masking (POSIX only; Windows falls back to echoing — known Phase-1 limitation).

**LoC delta:** ~150-200 lines of TUI harness code (no changes to src/bin/turingos/*.rs).

**Future polish (not Phase 1):**
- Add a `--json-events` mode to spec/generate that emits structured event objects to stdout (one JSON line per progress event). This makes multiplexed subprocess I/O cleaner than text parsing. Cost: ~150 LoC in cmd_spec.rs, cmd_generate.rs.

## Cited call sites

- Entry functions: src/bin/turingos.rs:95–228 (SUBCOMMANDS registry)
- WorkspaceStatus struct: src/bin/turingos/cmd_welcome.rs:45–61
- API key env readers: src/bin/turingos/cmd_llm.rs:488–530
- Spec capsule schema: src/runtime/spec_capsule.rs:44, 83–105, 110
- Artifact bundle: src/runtime/artifact_bundle.rs:7–44, 60–100
- Test run: src/runtime/test_run.rs:20–43, 69–77
- Generation attempt: src/runtime/generation_attempt.rs (imports at cmd_generate.rs:34–36)
