# TuringOS v4

> 🚨 **AGENT COLD-START — READ THIS FIRST** 🚨
>
> Any coding-agent CLI (Claude / Codex / Gemini / Aider / Cursor / Windsurf /
> Copilot / Warp / etc.) entering this repo MUST first read:
>
> 1. **`AGENTS.md`** — canonical universal agent contract (single source of truth)
> 2. **`HARNESS_PLAYBOOK.md`** — full operating manual (L1-L9 lessons, K-HARDEN 1-8)
> 3. **`skills/SUBAGENT_HARNESS.md`** — mandatory PRELUDE/POSTLUDE for subagent dispatch
> 4. **`handover/ai-direct/LATEST.md`** — current session state (derived view)
>
> Hard rules enforced mechanically (see `AGENTS.md` §14a):
> **PR-only workflow** · no `git push origin main` · no `git add .` · no sidecar staging
>
> If your CLI has its own discovery file (`GEMINI.md`, `CONVENTIONS.md`,
> `.cursorrules`, etc.) it's a thin pointer to `AGENTS.md`. **AGENTS.md wins
> on any conflict.**

---

TuringOS v4 is a tape-first constitutional operating substrate for LLM/AGI
agents. The authoritative state of a run is ChainTape plus CAS evidence; reports,
dashboards, and handover notes are materialized views.

## Handover State

Active: `handover/ai-direct/LATEST.md` (2026-05-29 post-merge
reconciliation after PR #212 and PR #214). This file is a derived view; if it
conflicts with ChainTape/CAS, executable gates, or merged PR receipts, trust the
authoritative evidence.

Current orientation package:

- `handover/ai-direct/LATEST.md` — current merged baseline and open caveats.
- `handover/reports/TURINGOS_REAL_ROADMAP_STATUS_2026-05-29.md` — correction
  of the stale "真实落地路线图" article against current main.
- `OBLIGATIONS.md` — OBL-001 through OBL-009 are currently `satisfied`.

Sessions #1-#54 are archived at:
`handover/ai-direct/LATEST_ARCHIVE_PRE_2026-05-20_sessions_1_to_54.md`.

## Current Main Status

As of `origin/main@13f2760f`, `main` includes the Phase E libgit2 cutover,
flowchart closure work, no-zombie/liveness gates, OBL-001 through OBL-009
closure, platform-agnostic audit doctrine, and the post-merge handover
correction. Current constitution gate result from a clean `origin/main`
worktree:

```text
bash scripts/run_constitution_gates.sh
# [k-1-5] total=164 failed=0
```

Current high-signal facts:

- **Git / tape substrate**: TDMA paths use `GitTapeLedger` via `git2`; runtime
  truth is ChainTape + CAS + replay, not dashboards or README text.
- **Obligation ledger**: `OBLIGATIONS.md` marks OBL-001 through OBL-009
  `satisfied`.
- **Audit doctrine**: one platform-agnostic clean-context audit by any capable
  fresh agent; the old vendor-specific single/dual-audit language is retired.
- **`turingos_dev`**: retired in PR #213; gate count dropped from 165 to 164
  because `constitution_dev_harness` was removed.
- **Internal market substrate**: CompleteSet, CPMM, atomic router, YES/NO
  positions, and replayed market actions are active typed-transaction surfaces.
  The generated market path now requires `MarketSeed -> CpmmPool ->
  BuyWithCoinRouter(YES) -> BuyWithCoinRouter(NO) -> Verify -> FinalizeReward
  -> EventResolve`. Price remains a signal, never predicate truth.
- **SWE-bench judge**: PR #212 wires `turingos tdma run --judge swebench` to a
  real SWE-bench hidden-test verifier path. Current honest result is loop 0/3,
  bare 0/3; the verifier/runtime path exists, but no loop advantage is claimed.
- **Known current blocker**: `cargo test --features web --test
  generate_emits_work_tx_smoke -- --nocapture` fails to compile because
  `src/web/dag_view.rs` imports private `TaskId` through `state::typed_tx`
  instead of `state::q_state`. This does not invalidate the constitution gate
  result, but it blocks that web-feature smoke until fixed.

### Recent Milestones

- **PR #214**: post-merge handover and roadmap-status correction.
- **PR #212**: SWE-bench TDMA hidden-test judge + review fixes.
- **PR #213**: platform-agnostic audit doctrine + `turingos_dev` retirement.
- **PR #211**: agent-presence / citation-DAG web surface and market panel
  enrichment.
- **PR #210**: liveness/no-zombie hardening and web/CLI kernel-invariant
  reinforcement.
- **PR #206**: 15-persona DeepSeek Chrome E2E obligation closure.

### Earlier Milestones

- **TB-SOFTWARE-3-0-CONSOLIDATION** (PRs #120, #122-#128): substrate
  hardening after Phase E.
- **TB-STRESS-PHASE-2** (PRs #129, #131, #132): 10-test adversarial stress
  battery, final tally 8 PASS / 1 PARTIAL / 1 NOT-EXECUTED / 0 FAIL.
- **Boundary-Ratification-Hygiene** (PR #78): process-hygiene boundary
  increment; explicit non-claim of OS-level hermetic/no-network sandboxing.

### V4 Product-CAK Hardening (P7.z) — 2026-05-20/21

`main` now contains the full Product-CAK evidence chain from spec interview
through artifact generation, preview, test run, and rejection — every step
anchored as a `ObjectType::EvidenceCapsule` in CAS with a deterministic
`schema_id` tag. The chain shape:

```
SpecCapsule
  └─ GenerationAttemptCapsule (C2)
        └─ ArtifactBundleManifest (C3) — typed `role` enum, path-traversal
              regex, cross-field invariants, immutability rule (every regen
              = new CID)
              ├─ PreviewRunCapsule (C6) — read-only observation; world head
              │                            unchanged
              ├─ TestRunCapsule (C11) — spec-derived TestScenarioSet,
              │                          hidden-oracle shielded,
              │                          accepted_delivery gate
              └─ GenerateRejectionCapsule (C8, L4.E) — v5-derived 4-tuple +
                    world_head_unchanged invariant; private_diagnostic_cid
                    shielded from HTTP body and BuildSessionView
                    └─ BuildSessionView (C7, derived projection, not
                          capsule) — rebuilds session from CAS without UI
                          session cache
                          └─ Offline replay + spec audit (C9) — CAS-only
                                `turingos replay --offline` with cross-CID
                                reference verification
```

Plus production-admission **C10**: `PromptPromotionReceipt` runtime guard
wired into 3 LLM startup sites in `cmd_generate.rs` and `cmd_llm.rs`;
refuses to start LLM unless a CAS-anchored promotion receipt with matching
prompt CID exists. Env-var bypass forbidden (`TURINGOS_BYPASS_PROMOTION_GUARD`
ignored).

**Cz Class 4** atom aligned `genesis_payload.toml`'s `trust_root` section
with post-merge content: rehashed `src/runtime/mod.rs`, `Cargo.toml`,
`src/bottom_white/cas/store.rs`, `tests/tb_7_legacy_append_regression.rs`;
removed 14 deleted `experiments/minif2f_v4/*` pins (user-authorized minif2f
deletion was part of C3); added 6 missing `pub mod` declarations
(`preview_run`, `build_session_view`, `replay`, `prompt_promotion`,
`test_scenario`, `test_run`). User §8 + Codex independent witness PROCEED.

Verification: `cargo test --lib boot::tests::` 8/8 PASS, including
`verify_trust_root_passes_on_intact_repo` (was failing on main pre-Cz with
`Tampered { expected: "05bf7151...", actual: "a3a09109..." }`).

7 new schema-ids in CAS, all `ObjectType::EvidenceCapsule`-tagged:
`turingos-generation-attempt-v1`, `turingos-artifact-bundle-v1`,
`turingos-preview-run-v1`, `turingos-generate-rejection-v1`,
`turingos-prompt-promotion-v1`, `turingos-test-scenario-set-v1`,
`turingos-test-run-v1`.

### Pre-P7.z baseline (preserved through session #56)

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
- **MiniF2F deletion** (2026-05-19/20, user-authorized cleanup): the
  `experiments/minif2f_v4/` subproject was removed from main as part of PR
  #46 (Atom C3). It is no longer a development benchmark package in this
  repository. The trust_root section in `genesis_payload.toml` was updated
  by Cz to drop the 14 minif2f file pins. Use `~/projects/turingosv3` or
  the prior session archives for any historical minif2f research; the
  current main is minif2f-free.

## Pull Request Ledger

This ledger is a README-level orientation view. When it conflicts with
ChainTape/CAS, executable gates, or PR evidence, trust the authoritative
evidence instead.

| PR | State | Main commit | Key information |
|---|---|---|---|
| [#214](https://github.com/gretjia/turingosv4/pull/214) | MERGED to `main` on 2026-05-29 | `13f2760f` | Docs-only post-merge handover correction. Updates `handover/ai-direct/LATEST.md` to PR #212 / 2026-05-29, records `164 failed=0`, notes `turingos_dev` retirement, and adds `TURINGOS_REAL_ROADMAP_STATUS_2026-05-29.md` correcting the stale roadmap article. |
| [#213](https://github.com/gretjia/turingosv4/pull/213) | MERGED to `main` on 2026-05-29 | `3a68d7c7` | Harness platform-agnostic unification. Generalizes clean-context audit doctrine, retires `turingos_dev` sidecar, preserves `AGENTS.md §10` as a retired anchor, and reduces the constitution gate count to 164 by removing `constitution_dev_harness`. |
| [#212](https://github.com/gretjia/turingosv4/pull/212) | MERGED to `main` on 2026-05-29 | `1f00012d` | SWE-bench TDMA hidden-test judge. Wires `turingos tdma run --judge swebench` to the real SWE-bench verifier path, fixes PR-review issues (TDMA state header, retry feedback, portable python default), and records honest loop 0/3 vs bare 0/3 evidence. |
| [#211](https://github.com/gretjia/turingosv4/pull/211) | MERGED to `main` on 2026-05-29 | `15d1ae7c` | Agent-presence / citation-DAG web surface and market panel enrichment. Adds session-scoped DAG/agent-presence UI work and records the follow-up liveness/backlink fixes. |
| [#210](https://github.com/gretjia/turingosv4/pull/210) | MERGED to `main` on 2026-05-28 | `b3d1a146` | Liveness/no-zombie hardening and real-market closure. Centralizes web artifact reads in the runtime kernel, adds recursive source/script liveness accounting, reinforces web/CLI kernel invariants, and fixes generate market flow to use real CPMM/router sequence. |
| [#207](https://github.com/gretjia/turingosv4/pull/207) | MERGED to `main` on 2026-05-28 | `594a632a` | Post-merge handover update after OBL-001 closure. Reconciles obligation status and handover docs after the real Chrome E2E evidence landed. |
| [#206](https://github.com/gretjia/turingosv4/pull/206) | MERGED to `main` on 2026-05-27 | `bb364f35` | OBL-001 DeepSeek Chrome E2E closure. Completes 15/15 real Chrome personas after 18 attempts, records redacted evidence under `handover/evidence/obl001_deepseek_chrome_20260527T171150Z/`, and closes with clean-context audit `NO-VIOLATION`. |
| [#132](https://github.com/gretjia/turingosv4/pull/132) | SQUASH-MERGED to `main` on 2026-05-23 | `6c12e092` | **TB-STRESS-PHASE-2 SHIP** — aggregate ship report + cumulative audits. 8 PASS / 1 PARTIAL / 1 NOT-EXECUTED / 0 FAIL across 10 adversarial tests. Constitution: NO-VIOLATION. Karpathy: PASS. ST-04 PARTIAL surfaced S2 `write_snapshot` VERIFIED in CAS; multi-turn blocked by upstream triage promotion-guard (workspace bootstrap dep, NOT S2 defect). |
| [#131](https://github.com/gretjia/turingosv4/pull/131) | SQUASH-MERGED to `main` on 2026-05-23 | `1ea99a2d` | TB-STRESS-PHASE-2 STRESS-1..10 — execution evidence for all 10 runners + runner robustness fixes (schema/workspace bootstrap/port). 10 evidence dirs under `handover/evidence/stress_st0*_<UTC_TS>/`, each with `summary.md` ending in `KILL: PASS` or `KILL: FAIL`. LLM cost ≈ $0 (mocks throughout). |
| [#129](https://github.com/gretjia/turingosv4/pull/129) | SQUASH-MERGED to `main` on 2026-05-23 | `22812db8` | TB-STRESS-PHASE-2 STRESS-0 — charter + §8 directive + 10 stress-test runner scripts in `scripts/stress/` + mock LLM provider (`_mock_llm_server.py`) + workspace bootstrap helper (`_ws_bootstrap.sh`). |
| [#128](https://github.com/gretjia/turingosv4/pull/128) | SQUASH-MERGED to `main` on 2026-05-23 | `78717d26` | **TB-SOFTWARE-3-0 SHIP** — aggregate ship report + cumulative constitution + Karpathy audits. Both verdicts GREEN. 9-item ship-gate fully met. |
| [#127](https://github.com/gretjia/turingosv4/pull/127) | SQUASH-MERGED to `main` on 2026-05-23 | `32e30d97` | TB-SOFTWARE-3-0 Atom S5 — `scripts/audit_legacy_bypass.sh` standalone reporting script (NOT a constitution gate) + `NO_LEGACY_BYPASS_CHECKLIST_2026-05-23.md`. Class 0+1. |
| [#126](https://github.com/gretjia/turingosv4/pull/126) | SQUASH-MERGED to `main` on 2026-05-23 | `ac95ac12` | TB-SOFTWARE-3-0 Atom S4.2 — `LLM_BOUNDARY_INVENTORY_2026-05-23.md` documenting 17 `chat_complete*` call sites + the deferred Class 3/4 abstraction packet (per K10). Class 0. |
| [#125](https://github.com/gretjia/turingosv4/pull/125) | SQUASH-MERGED to `main` on 2026-05-23 | `c2b6d954` | TB-SOFTWARE-3-0 Atom S4.1 — rename `src/bin/turingos/siliconflow_client.rs` → `chat_client.rs`. 7 cmd_*.rs import sites updated. NO `ChatProvider` enum (deferred per K10 until 2nd provider). Class 2. |
| [#124](https://github.com/gretjia/turingosv4/pull/124) | SQUASH-MERGED to `main` on 2026-05-23 | `1d35058d` | TB-SOFTWARE-3-0 Atom S3 — `BuildSessionViewError { Open, Read, Decode }` taxonomy in `src/runtime/build_session_view.rs`. Empty workspace stays `Ok(BuildStatus::SpecPending)`; only corruption surfaces as `Err`. 3 distinction tests. Class 2. |
| [#123](https://github.com/gretjia/turingosv4/pull/123) | SQUASH-MERGED to `main` on 2026-05-23 | `486adaa2` | TB-SOFTWARE-3-0 Atom S2 — private `GrillSessionSnapshot` in per-session CAS for cross-restart resume. `src/web/session_snapshot.rs` NEW (writes 418-byte capsules with `schema_id="turingos-web-grill-session-snapshot-v1"`); `src/web/spec.rs` calls `write_snapshot` after every successful turn and `load_latest_snapshot` on session-not-found before falling through to 404. Class 2. |
| [#122](https://github.com/gretjia/turingosv4/pull/122) | SQUASH-MERGED to `main` on 2026-05-23 | `7130cf91` | TB-SOFTWARE-3-0 Atom S1 — remove stdout-as-truth in `src/web/write.rs`: deleted `t_hash_*` synthesized id fallback and `simple_hash` FNV helper. `task/open` returns `502 BAD_GATEWAY` with `kind="task_id_parse_failed"` on stdout parse failure (no TaskEntry written, no WS broadcast). Class 2. |
| [#120](https://github.com/gretjia/turingosv4/pull/120) | SQUASH-MERGED to `main` on 2026-05-23 | `b0a2da1c` | TB-SOFTWARE-3-0 Atom S0.1 — package §8 directive + TB charter. Class 0. |
| [#78](https://github.com/gretjia/turingosv4/pull/78) | SQUASH-MERGED to `main` on 2026-05-21 | `38adc108` | Boundary-Ratification-Hygiene increment. Adds `FC_REAL_WORLD_BOUNDARY.md`, §8 ratification directive, sandbox boundary baseline, `SanitizedCommand` process-hygiene runner, product shell-out wiring, P7.z truthfulness hygiene, real-world meaning fixtures, and faster constitution-gate CI runner. Explicit non-claim: no OS-level hermetic/no-network sandbox; network policy claim remains `NotEnforced`. GitHub checks green; clean-context audits `NO-VIOLATION`. |
| Cz | MERGED via orchestrator local-merge on 2026-05-21 (no PR number; supersedes closed PR #55) | `9bdaddee` | Class 4 cumulative Trust Root realignment after all charter PRs landed. Updated 4 SHA pins (`src/runtime/mod.rs`, `Cargo.toml`, `src/bottom_white/cas/store.rs`, `tests/tb_7_legacy_append_regression.rs`); removed 14 deleted `experiments/minif2f_v4/*` pins; added 6 `pub mod` declarations missing from `src/runtime/mod.rs` (`preview_run`, `build_session_view`, `replay`, `prompt_promotion`, `test_scenario`, `test_run`); fixed `src/runtime/replay.rs` imports to source rejection types from `rejection_capsule` module. User §8 + Codex independent witness PROCEED. `cargo test --lib boot::tests::` 8/8 PASS post-Cz. |
| [#56](https://github.com/gretjia/turingosv4/pull/56) | MERGED to `main` on 2026-05-21 | `298a1a7b` | Docs-only audit records from session #56 (Class 0). Adds `CLAUDE_SESSION_56_GEMINI_P7Z_AUDIT_2026-05-21.md`, `CLAUDE_SESSION_56_REMEDIATION_REPORT_2026-05-21.md`, and `CLAUDE_SESSION_56_REMEDIATION_SECTION8_RECORDS_2026-05-21.md` to `handover/audits/`. §8 sign-offs for the 4 Class 3+ atoms remediated this session (C2-split, C10, C11, Cz). |
| [#55](https://github.com/gretjia/turingosv4/pull/55) | CLOSED, superseded | n/a | Initial Cz 1-line trust-root fix. Codex independent witness PROCEED on initial diff, but scope insufficient after charter PRs merged — the cumulative Cz commit (above) replaces it. |
| [#54](https://github.com/gretjia/turingosv4/pull/54) | SQUASH-MERGED to `main` on 2026-05-21 | `028f9881` | Atom C11: spec-derived `TestRunCapsule` + producer wire. `TestScenario` enum trimmed to 3 producer-bound variants (`EntrypointExists`, `HtmlParses`, `SandboxPolicyPreserved`); `TestRunCapsule` carries separate `test_scenario_set_cid` per v5 hidden-oracle pattern; producer wired in `cmd_generate.rs` post-bundle-write (helper in `test_run.rs` to preserve hidden-oracle static-grep); `BuildStatus::Accepted` derivation NOT wired into `src/state/sequencer.rs` (anti-wire invariant). 19 subtests across 6 test files PASS. §8 + Sonnet audit PROCEED. |
| [#53](https://github.com/gretjia/turingosv4/pull/53) | SQUASH-MERGED to `main` on 2026-05-21 | `412ebf6d` | Atom C10: `PromptPromotionReceipt` runtime guard wired into 3 LLM startup sites (`cmd_generate.rs::run_inner` before `chat_complete_blocking`, `cmd_llm.rs::run_triage`, `cmd_llm.rs::run_prompt_eval`); skipped `run_complete` (user-supplied prompt) and `run_prompt_promote` (no LLM call). Scaffold `--force` arg position fix. Env-var bypass forbidden test passes. §8 + Sonnet audit PROCEED 7/7. |
| [#52](https://github.com/gretjia/turingosv4/pull/52) | SQUASH-MERGED to `main` on 2026-05-21 | `a699dd61` | Atom C9: `turingos replay --offline` CLI flag wired to `runtime::replay::reconstruct_session()` (CAS-only). Existing 7-indicator ChainTape replay preserved as default mode. 3 spec-named tests added (`artifact_bundle_replay_reads_cas`, `build_session_replay_after_cache_delete`, `replay_verifies_all_cross_cid_references_resolve`). Static no-LLM proof via dependency grep (NOT runtime tracing). |
| [#51](https://github.com/gretjia/turingosv4/pull/51) | SQUASH-MERGED to `main` on 2026-05-21 | `0039bc6e` | Atom C8: L4.E `GenerateRejectionCapsule` HTTP shielding + 5 missing spec tests (`generate_fail_goes_l4e`, `user_error_does_not_leak_panic`, `privacy_fail_not_retryable`, `rejection_capsule_world_head_unchanged`, `rejection_capsule_4_tuple_present`). The world-head field is now treated as an observed writer result rather than a literal production self-report. v5-derived 4-tuple invariant. §8 self-signed under user delegation. |
| [#50](https://github.com/gretjia/turingosv4/pull/50) | SQUASH-MERGED to `main` on 2026-05-21 | `e35df8f9` | Atom C7: `BuildSessionView` derived from CAS via `schema_id` scan. Not a capsule; no `schema_id`, no CAS write. Private diagnostic CID + scenario set CID intentionally excluded. 6 spec-named test files (split + struct-field fix to align with current schemas). Ordering `(logical_t, cid)`. |
| [#49](https://github.com/gretjia/turingosv4/pull/49) | SQUASH-MERGED to `main` on 2026-05-21 | `9929bfc8` | Atom C6: `PreviewRunCapsule` + `GET /api/preview/:artifact_bundle_cid/file` endpoint. `SandboxPolicy` byte-stable enum (not free-form String). World-head unchanged operational test asserts exactly one commit advance on `CHAINTAPE_CAS_REF` per preview (the capsule put itself). No headless browser introduced. |
| [#48](https://github.com/gretjia/turingosv4/pull/48) | SQUASH-MERGED to `main` on 2026-05-21 | `074f6fe3` | Atom C5: CAS-backed bundle file serve route `GET /api/bundle/:artifact_bundle_cid/file?path=<rel>` with namespace shielding (rejects CIDs whose `schema_id != turingos-artifact-bundle-v1`). Path-traversal regex reused. Legacy `/api/artifact/:session_id/:name` route preserved. |
| [#47](https://github.com/gretjia/turingosv4/pull/47) | SQUASH-MERGED to `main` on 2026-05-21 | `35d53a1f` | Atom C4: `POST /api/generate` web response carries `artifact_bundle_cid: Option<String>` plus per-file `cid` and `sha256`. Additive only; serde `skip_serializing_if = "Option::is_none"` for backward-compatible JSON shape. Reads bundle CID from CAS via `latest_artifact_bundle_cid_for_session`, NOT a filesystem pointer. |
| [#46](https://github.com/gretjia/turingosv4/pull/46) | SQUASH-MERGED to `main` on 2026-05-21 | `364242f1` + `b81d2ce6` (hotfix for stranded conflict markers) | Atom C3: `ArtifactBundleManifest` CAS wire — typed `ArtifactFileRole` enum (Entrypoint/Source/Asset/Manifest/Test/Other), `entrypoint_must_match_files_path` cross-field invariant, path-traversal regex `^(?!/)(?!.*(?:^|/)\.\.(?:/|$)).+`, `previous_bundle_cid` provenance chain (every regen = new CID), `latest_artifact_bundle_cid_for_session` lookup. Also user-authorized `experiments/minif2f_v4/` deletion in same commit. |
| [#45](https://github.com/gretjia/turingosv4/pull/45) | SQUASH-MERGED to `main` on 2026-05-21 | `efc23c0c` | Atom C2: `GenerationAttemptCapsule` CAS wire — 1 LLM call → 1 capsule per `feedback_chaintape_externalized_proposal`; `AttemptOutcome` enum (`Success=0, ParseFailed=1, LlmApiError=2, NoFilesParsed=3, InternalIo=4`); `parent_attempt_cid` ordering chain. Includes C2/C8 split (rejection_capsule schema relocated from `generation_attempt.rs` to its own module `src/runtime/rejection_capsule.rs` per master plan §C8 spec). §8 + Sonnet audit PROCEED 10/10. |
| [#44](https://github.com/gretjia/turingosv4/pull/44) | SQUASH-MERGED to `main` on 2026-05-20 | `bed3589c` | Atom C1: V4 product baseline reality seal — `docs/roadmap/V4_PRODUCT_BASELINE_REALITY_SEAL.md` with machine-provable assertions for the spec→generate loop (grep/test commands per fact). Class 0 docs-only. |
| [#43](https://github.com/gretjia/turingosv4/pull/43) | SQUASH-MERGED to `main` on 2026-05-20 | `18c5163f` | Atom C0: fresh-clone web build gate. `build.rs` emits actionable error message (`run: cd frontend && npm ci && npm run build`) when `frontend/dist/main.js` is missing instead of silent failure. Class 1 build-adapter only. |
| [#42](https://github.com/gretjia/turingosv4/pull/42) | SQUASH-MERGED to `main` on 2026-05-20 | `e7ebd0cf` | Session #55 docs: V4 Product-CAK Hardening execution plan (master charter) + Gemini orchestrator boot prompt + LATEST.md archive (sessions #1–#54 → archive file). Class 0 docs-only. |
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
5. ~~**Frontend-build dependency**: `src/web/router.rs` uses
   `include_bytes!("../../frontend/dist/main.js")`, but `frontend/dist/`
   is gitignored. Fresh-clone `cargo build --features web` fails until
   `cd frontend && npm ci && npm run build`. Recommended: a `build.rs` that
   fails with a clear `npm run build` hint, or commit the dist artefact.~~
   **RESOLVED by Atom C0 (PR #43, 2026-05-20)**: `build.rs` now emits an
   actionable error message instructing the contributor to run
   `cd frontend && npm ci && npm run build` when `frontend/dist/main.js`
   is missing.

## Authoritative Orientation

Read these first for a cold start:

1. `AGENTS.md`
2. `CLAUDE.md`
3. `HARNESS_PLAYBOOK.md`
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

MiniF2F is no longer part of this repository's root workspace or core
constitution gate. Use the historical v3 repository or archived evidence for
old MiniF2F research.
