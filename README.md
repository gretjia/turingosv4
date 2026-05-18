# TuringOS v4

TuringOS v4 is a tape-first constitutional operating substrate for LLM/AGI
agents. The authoritative state of a run is ChainTape plus CAS evidence; reports,
dashboards, and handover notes are materialized views.

## Current Main Status

- `main` includes the audited **TISR Phase 7 Web MVP** merged by PR #6
  as squash commit
  `eab583fd30f278db26ef2ab98c39eaf010333a22`. Phase 7 wraps the Phase 6.3
  `spec → generate → play` CLI flow in an HTTP + WebSocket server
  (`src/web/**` + `src/bin/turingos_web.rs`) plus a vanilla-TS +
  Web Components frontend (`frontend/**`). Run
  `cargo build --features web --bin turingos_web` (after `cd frontend &&
  npm run build`), then open `http://127.0.0.1:8080/welcome` for the
  onboarding wizard.
- Phase 7 highlights:
  - 14 axum routes (4 HTML + 3 JSON IR + 1 WS + spec/generate/artifact
    + task-open + static), all bound to `127.0.0.1:8080` (no flag /
    no env override).
  - Auto-retry on heuristic-fail (W8 → W8.1 → W8.2): server-side
    post-generate heuristic verifier with WS progress events. Closed
    via a 4-round real-LLM E2E with 3-role cross-validation
    (user-simulator + backend-observer + Test Director).
  - API key stays in-memory only
    (`AppState.api_key: Arc<Mutex<Option<String>>>`); never
    `localStorage`, never logged, never persisted.
  - Artifact viewer uses `iframe sandbox="allow-scripts"`-only with
    a `SANDBOX_ALLOWED_TOKENS` guard against any `allow-same-origin`
    combo. Path-traversal triple-defended in
    `src/web/artifact.rs` (whitelist regex + `canonicalize()` +
    prefix-check).
- `main` includes the audited **TISR Phase 6.0–6.3 alpha CLI stack**
  merged by PR #4 as squash commit
  `ff866c53fa2622b2a4d3a944df8cee70874e2834`.
- `turingos` CLI is the primary user entry point. The stack registers
  ~25 subcommands across families `init` / `report` / `verify` / `audit` /
  `preflight` / `replay` / `task` / `config` / `agent` / `batch` /
  `export` / `render` / `welcome` / `llm` / `spec` / `generate`. Run
  `turingos --help` for the full surface.
- Phase 6.3 adds a real SiliconFlow-backed two-LLM wire:
  Meta (reasoning) defaults to `deepseek-ai/DeepSeek-V3.2`; Blackbox
  (codegen) defaults to `Qwen/Qwen3-Coder-30B-A3B-Instruct`. The API key
  is never persisted to disk — only the env-var NAME is stored in
  `<workspace>/turingos.toml`.
- `turingos spec` runs an 8-question non-developer customer-development
  grill (Chinese-first), emits `spec.md`, and anchors the bytes in CAS as
  an `EvidenceCapsule` (`schema_id = turingos-spec-capsule-v1`). The CID is
  printed to stdout and is read back by `turingos welcome` to flip the
  "spec done" status. `turingos generate` then drives codegen against the
  Blackbox model.
- `main` also includes the audited CAS Git constitutional repair merged by
  PR #3 at commit `802b18053d063bd5503a6b0eb2e7b1f46ceda93b`. CAS now has
  a Git commit-chain layer while preserving `Cid = sha256(content)`;
  `refs/chaintape/cas` advances as a CAS commit head for new writes, and
  `CasStore::open()` / reload paths take the same chain lock used by
  `put()`.
- MiniF2F is a development benchmark package, not a fixed TuringOS kernel
  or OS gate. It is excluded from the root workspace and is only run
  explicitly via `--manifest-path experiments/minif2f_v4/Cargo.toml`.

## Phase 7 follow-ups (non-blocking, recorded in PR #6 squash body)

1. **Frontend bundle size drift**: `frontend/dist/main.js` is ~73 kB on
   the W8.2 evidence; the Phase 7 ship report claimed a ≤ 50 kB cap. No
   automated CI assertion currently catches future drift. Either bump
   the documented cap or add a `body.len() < 75_000` assertion to
   `tests/cli_web_routes_smoke.rs`.
2. **Frontend-build dependency**: `src/web/router.rs` uses
   `include_bytes!("../../frontend/dist/main.js")`, but `frontend/dist/`
   is gitignored. Fresh-clone `cargo build --features web` fails until
   `cd frontend && npm run build`. The constitution gate avoids it (it
   never builds with `--features web`); the E2E setup script handles
   it. Recommended: a `build.rs` that fails with a clear `npm run
   build` hint, or commit the dist artefact.

## Authoritative Orientation

Read these first for a cold start:

1. `AGENTS.md`
2. `CLAUDE.md`
3. `HARNESS_MANUAL.md`
4. `constitution.md`
5. `handover/ai-direct/LATEST.md`
6. `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
7. `handover/alignment/TRACE_FLOWCHART_MATRIX.md`

Truth order is defined in `AGENTS.md`: constitution and flowchart contracts
outrank ChainTape/CAS, gates, handover, dashboards, and README text.

## Core Checks

Preferred ship-level checks:

```bash
git diff --check
bash scripts/run_constitution_gates.sh
cargo test --workspace --no-fail-fast -- --test-threads=1
```

For MiniF2F development work, use the experiment manifest explicitly:

```bash
cargo test --manifest-path experiments/minif2f_v4/Cargo.toml -- --test-threads=1
```

Do not treat MiniF2F as a default root workspace or core constitution gate.
