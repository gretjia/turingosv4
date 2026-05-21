# TuringOS v4

TuringOS v4 is a tape-first constitutional operating substrate for LLM/AGI
agents. The authoritative state of a run is ChainTape plus CAS evidence; reports,
dashboards, and handover notes are materialized views.

## Handover state

Active: `handover/ai-direct/LATEST.md` (session #55, 2026-05-20 — V4 Product-CAK Hardening charter).

Sessions #1–#54 archived at:
`handover/ai-direct/LATEST_ARCHIVE_PRE_2026-05-20_sessions_1_to_54.md`

## Current Main Status

`main` currently includes PR #3, #4, #5, #6, #7, #8, #10, and #11. PR #1,
#2, and #9 were closed without merge and are not mainline state.

- `main` includes the audited **TISR Phase 6.3.y grill-driven Generative UI
  ship unit** merged by PR #11 as merge commit
  `300fb563ae57d971610b923d83fc55ab083ae245`. This preserves the six
  ship-unit commits for auditability:
  - A6 + A8b: move `spec_capsule` into `src/runtime/` and load synthesis
    prompts from assets instead of inline literals.
  - F2 + A2: strip `<think>` blocks in strict JSON completion paths and add
    `turingos llm prompt-eval` for future prompt regression gates.
  - F11: make `cmd_generate` quality predicates domain-agnostic via
    `VerifyMode::MinimumBar`.
  - F1/F3/F4/F5/F6/F9/F10: harden the web spec turn loop, meta-prompt wiring,
    error handling, transcript rollback, and slot-keyed spec synthesis.
  - Archive v2/v3 sibling prompts as candidates, not active production
    prompts.
  - Add the ultraplan evidence and clean-context audit/disposition trail.
- Phase 6.3.y demonstrated the Step 0 -> Step 3 Generative UI path
  (spec interview -> CAS-anchored spec capsule -> code generation -> browser
  preview) on the P7 Traditional Chinese persona. The shipped binary remains
  on canonical v1 prompts; v2/v3 prompt evidence is archived and conditional,
  pending a future A11 promotion via the A2 prompt-eval gate.
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
  an `EvidenceCapsule` (`schema_id = turingos-spec-capsule-v1`). The spec
  capsule logic now lives in `src/runtime/spec_capsule.rs` so both CLI and web
  paths can synthesize and verify capsules through the same library surface.
  The CID is printed to stdout and is read back by `turingos welcome` to flip
  the "spec done" status. `turingos generate` then drives codegen against the
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

## Pull Request Ledger

This ledger is a README-level orientation view. When it conflicts with
ChainTape/CAS, executable gates, or PR evidence, trust the authoritative
evidence instead.

| PR | State | Main commit | Key information |
|---|---|---|---|
| [#11](https://github.com/gretjia/turingosv4/pull/11) | MERGED to `main` on 2026-05-19 | `300fb563ae57d971610b923d83fc55ab083ae245` | Phase 6.3.y grill-driven Generative UI ship unit. Ships F1-F11 + A2/A6/A8b code fixes, A2 prompt-eval CLI, runtime `spec_capsule`, web spec-loop hardening, domain-agnostic generate quality predicates, v2/v3 prompt candidates archived but not active, and the ultraplan evidence/audit trail. |
| [#10](https://github.com/gretjia/turingosv4/pull/10) | MERGED to `main` on 2026-05-18 | `7a2ae7f7bf6fa2f9ce3cbfcf7a307462b7c69db1` | REAL-17 Polymarket robustness increment. Adds `real17p21.market_order_ticket.v1` CAS sidecar, Bull/Bear market-order evidence wiring, forced positive-control router/settlement gates, slippage/balance/finalized-market rejection gates, YES/NO settlement and redeem checks, and explicit no-overclaim boundary. |
| [#9](https://github.com/gretjia/turingosv4/pull/9) | CLOSED, not merged | n/a | Superseded REAL-17 Polymarket robustness branch. It did not land on `main`; use PR #10 as the mainline Polymarket robustness record. |
| [#8](https://github.com/gretjia/turingosv4/pull/8) | MERGED to `main` on 2026-05-18 | `886f7596f02683301aee7663b2bdb9c4a83c0a2a` | REAL-17 market emergence hardening on the CAS-main baseline. Adds MarketDecision provenance sidecar support, exact-join verifier support for PromptCapsule provenance counts, PositiveEVIgnored/role-differentiation/E4a pressure-efficiency gates, runner/poll stabilization, BearTrader NO-side semantics clarification, and clean-context audit `PROCEED`. Does not claim E2/E3/E4 or market emergence proven. |
| [#7](https://github.com/gretjia/turingosv4/pull/7) | MERGED to `main` on 2026-05-18 | `8c1032c0dd4c046ff3b21d866545f3d818ece041` | Docs-only README refresh after PR #6. Recorded the Phase 7 Web MVP status, run instructions, security notes, and non-blocking Phase 7 follow-ups. |
| [#6](https://github.com/gretjia/turingosv4/pull/6) | MERGED to `main` on 2026-05-18 | `eab583fd30f278db26ef2ab98c39eaf010333a22` | TISR Phase 7 Web MVP. Wraps `spec -> generate -> play` in an axum HTTP/WebSocket server plus vanilla TypeScript/Web Components frontend, onboarding wizard, in-memory API key handling, sandboxed artifact viewer, task-open/write route, server-side heuristic auto-retry, and 4-round real-LLM closure. |
| [#5](https://github.com/gretjia/turingosv4/pull/5) | MERGED to `main` on 2026-05-17 | `53cc4442253f49753d76d8126de51a1c9ddbc1b7` | Docs/handover refresh after PR #4. Updated README and `handover/ai-direct/LATEST.md` to reflect the Phase 6.0-6.3 alpha CLI stack ship. |
| [#4](https://github.com/gretjia/turingosv4/pull/4) | MERGED to `main` on 2026-05-17 | `ff866c53fa2622b2a4d3a944df8cee70874e2834` | TISR Phase 6.0-6.3 alpha CLI stack. Lands `turingos` CLI families, real SiliconFlow two-LLM config, Chinese-first non-developer `spec` grill, CAS-anchored spec capsules, `generate` codegen path, and 3/3 real-LLM E2E evidence. |
| [#3](https://github.com/gretjia/turingosv4/pull/3) | MERGED to `main` on 2026-05-17 | `802b18053d063bd5503a6b0eb2e7b1f46ceda93b` | CAS Git constitutional repair. Adds the Git commit-chain layer for CAS while preserving `Cid = sha256(content)`, advances `refs/chaintape/cas` for new writes, and aligns CAS reload/open paths with the chain lock. |
| [#2](https://github.com/gretjia/turingosv4/pull/2) | CLOSED, not merged | n/a | TISR Phase 6.0/6.1 alpha `turingos init` first slice targeting `worktree-tisr-2026-05-17`, structurally superseded on main by PR #4. Do not merge into current `main`. |
| [#1](https://github.com/gretjia/turingosv4/pull/1) | CLOSED, not merged | n/a | TISR-001 research and Phase 6.0/6.1 ratification material. Its key research/directive content is represented in later mainline Phase 6 docs and ship history; do not merge this old PR. |

## Phase 6.3.y / 7 follow-ups

1. **Prompt promotion**: v2/v3 grill prompts are archived in `assets/prompts/`
   but are not active. Promote them only through the A11 atom using
   `turingos llm prompt-eval` on a richer eval fixture.
2. **Multi-slot slot ledger**: Mrs Chen-style answers can cover multiple
   canonical slots in one turn. F10's shipped slot-keyed mapping is improved,
   but the full fix is deferred to F12 multi-slot per-turn ledger.
3. **Provider flake hardening**: SiliconFlow transient empty `ok=false`
   responses remain a quality issue; A13 should add in-handler retry/backoff.
4. **Frontend bundle size drift**: `frontend/dist/main.js` is ~84 kB after
   PR #11. The Phase 7 ship report claimed a lower cap, and no automated CI
   assertion currently catches future drift. Either bump the documented cap
   or add a bundle-size assertion to the web route tests.
5. **Frontend-build dependency**: `src/web/router.rs` uses
   `include_bytes!("../../frontend/dist/main.js")`, but `frontend/dist/`
   is gitignored. Fresh-clone `cargo build --features web` fails until
   `cd frontend && npm ci && npm run build`. Recommended: a `build.rs` that
   fails with a clear `npm run build` hint, or commit the dist artefact.

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

## Build

To build the project with the web-enabled features, you must build the frontend first. The canonical build sequence is:

1. Build the frontend assets:
   ```bash
   cd frontend
   npm ci
   npm run build
   cd ..
   ```
2. Build the Rust binary:
   ```bash
   cargo build --features web --bin turingos_web
   ```

If you attempt to run `cargo build --features web` without building the frontend first, the build will fail with an error message instructing you to build the frontend.

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
