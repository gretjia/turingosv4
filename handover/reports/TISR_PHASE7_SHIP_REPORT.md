# TISR Phase 7 Web MVP — Ship Report

**Date**: 2026-05-18
**Branch**: `codex/tisr-phase7-web` @ HEAD `4259ee53`
**Comparison base**: `75e6e6b7` (Phase 7 §8 architect ratification)
**Auditor verdict**: **PROCEED** · High confidence · 0 production defects
**Auditor record**: [`handover/audits/AUDITOR_TISR_PHASE7_R1_PROCEED.md`](../audits/AUDITOR_TISR_PHASE7_R1_PROCEED.md)
**Pre-ship gates**: **7/7 green**
**Branch state**: 21 commits ahead of `origin/codex/tisr-phase7-web`, **not pushed** (awaits architect push + PR to main)

---

## Verdict context

Phase 7 §8 ratified by architect (verbatim 2026-05-17 "都按你建议") set the four
load-bearing decisions: WebSocket transport, vanilla TS + Web Components,
W0 one-shot Trust Root rehash, `127.0.0.1:8080` hard. All four held through
ship, with one architect-ratified mid-flight erratum (W0.1, 2026-05-18,
verbatim "方案 B: W0.1 补丁 + 地道 axum ws 代码") closing the axum `ws`
feature-flag omission.

Mid-ship the architect re-oriented (2026-05-18, verbatim "方案 A: Merge main
+ 增加 W5/W6 atoms 用 Phase 6.3 spec+generate UI") from the original
Lean-substrate `task open` write-path to Phase 6.3's LLM-driven spec→generate
flow. The §6a charter criterion "ChainTape advanced by ≥2 txs via lean_market"
was deliberately abandoned in favor of "spec→generate substrate proven via
the W5+W6 wire + iframe-sandbox security held." This re-orientation is the
defining architectural decision of the Phase 7 ship.

---

## Round history

| Round | Commit | Verdict | Issue | Closed at |
|---|---|---|---|---|
| §6a v1 | (mid-flight) | n/a | Lean-substrate criterion obsolete post-architect-reorientation; superseded by §6a v2 | n/a (charter re-aim) |
| §6a v2 | `dd1d5724` | PARTIAL | `GenerateRequest` rejected bare `{ session_id }` payload (HTTP 422 click-through); iframe sandbox security held; substrate proven end-to-end | `4259ee53` (W6.1 fix + regression test) |
| **§5 R1** | **`4259ee53`** | **PROCEED** | (none) | — |

---

## Deliverables shipped

### Backend (Rust, `src/web/**` + `src/bin/turingos_web.rs`)

| Surface | Source | Tests |
|---|---|---|
| 14 axum routes (4 HTML + 3 JSON IR + 1 WS + 1 task-open POST + 1 static + 3 spec/generate/artifact + 1 build HTML) | `src/web/router.rs` | `cli_web_routes_smoke.rs` (44/44) |
| Server-side IR → HTML renderer with editorial typography injection | `src/web/render.rs` | `cli_web_smoke.rs` (38/38) |
| WebSocket handler with tagged-union `WsBroadcastMsg` (5 variants: `ir_update`, `task_created`, `spec_complete`, `generate_started`, `generate_complete`) | `src/web/ws.rs` | `cli_web_ws_smoke.rs` (41/41) |
| POST `/api/task/open` with input validation + Phase 6.1 CLI shellout | `src/web/write.rs` + `store.rs` | `cli_web_write_smoke.rs` (45/45) |
| POST `/api/spec/submit` (8-question grill scripted-mode wrapper) | `src/web/spec.rs` | `cli_web_spec_smoke.rs` (42/42) |
| POST `/api/generate` + GET `/api/artifact/:session/:name` with path-traversal canonicalization | `src/web/generate.rs` + `artifact.rs` | `cli_web_generate_smoke.rs` (44/44) |
| Frontend bundle embed via `include_bytes!` (no `tower-http/fs` dep) | `src/web/router.rs` | — |
| AppState shared broadcast channel + in-memory task store (W4.2) | `src/web/ws.rs` + `store.rs` | included above |

### Frontend (vanilla TS + Web Components, `frontend/**`)

| Custom element | Purpose | Lines |
|---|---|---|
| `<turingos-root>` | Host element; routes; subscribes to WS + dispatches `turingos:ir_update` CustomEvent | core |
| `<tos-text-block>` `<tos-table-block>` `<tos-agent-card-block>` `<tos-task-card-block>` `<tos-event-log-block>` `<tos-dashboard-panel-block>` | One per IR block-type; light-DOM; `data-block-type` set | W3 |
| `<tos-task-open-form>` | Legacy task-open form (W4) | W4 |
| `<tos-task-open-form>` (kept) | — | |
| `<turingos-status>` | Footer WS-state pill | W4.4 |
| **`<tos-spec-grill>`** | 8-question interview state machine (idle / loading / interviewing / submitting / spec_ready / error) | **W6** |
| **`<tos-spec-result>`** | Markdown-render of `spec.md` + capsule CID + "生成代码" CTA | **W6** |
| **`<tos-artifact-viewer>`** | iframe-sandboxed (`sandbox="allow-scripts"`) preview of LLM-generated `index.html` | **W6** |

- Bundle: `dist/main.js` **49.9 kB** (under 50 kB cap)
- Frontend src LOC: **2,405** (under 5,000 ceiling)
- Frontend tests: **73/73** (Node `--test` + `tsx`; XSS hygiene + iframe sandbox combo enforcement; anti-AI-aesthetic enforcement)

### Design system (W4.4, applied through W6)

Following architect-invoked Anthropic generative-UI guidance (Claude Cookbook
"Prompting for frontend aesthetics", Thariq Shihipar's "Unreasonable
Effectiveness of HTML", Claude Design help center):

- **Typography pair**: Fraunces variable serif (display, italic page titles) + JetBrains Mono (IDs/hashes/numerics/nav-labels in small-caps) + IBM Plex Sans (long-form body)
- **Color palette**: paper `#FAFAF7`, ink `#1A1817`, oxidized-teal accent `#1F6E6B`, hairline `#E5E3DC` — dark-mode counterparts auto-engaged via `prefers-color-scheme`
- **Status badges**: typographic with dot + 1px border (no flat-color pills); semantic colors (open=teal, accepted=moss `#3F6E3F`, rejected=brick `#9C3A2F`, finalized=amber `#A87431`)
- **Anti-AI-aesthetic enforcement**: `frontend/test/design-system.test.ts` asserts no Inter/Roboto/Arial + no purple-gradient-on-white
- **Visual evidence**: `handover/evidence/stage_phase7_web_w4_4/visual_self_check.md` + `handover/evidence/stage_phase7_web_w6/visual_self_check.md`

### Workspace setup (W4.3 + W4.3.1)

- `tests/frontend_e2e_setup_workspace.sh` — idempotent setup: builds `turingos`, `turingos_web`, `lean_market`, frontend bundle, runs `turingos init` + `turingos agent deploy --id agent_001 --role Solver`
- `tests/frontend_e2e_teardown_workspace.sh` — safe cleanup (refuses paths outside `tmp/` or `/tmp/`)
- `tests/frontend_e2e_setup_workspace_test.sh` — 8-assertion meta-test of the setup machinery itself
- `tests/frontend_e2e_README.md` — orchestration contract for the §6a verifier

---

## Trust Root events (chronological)

| Event | Commit | Cargo.toml | Cargo.lock | Ratification |
|---|---|---|---|---|
| W0 main rehash | `0adffd50` | `d9cb276d` → `53c19680` | `080b20c7` → `60ae38b4` | Phase 7 §8 §3 |
| W0.1 erratum | `38e3ff42` | `53c19680` → `46c1340a` | `60ae38b4` → `85a29755` | Architect chat 2026-05-18 (verbatim "方案 B") |
| Post-merge auto-merge regen | `5e3ae33e` (merge) | → `1cead96d` | regenerated → `48cbf884` | Architect chat 2026-05-18 (verbatim "方案 A") |

All three events recorded with full lineage in `genesis_payload.toml` entry
comments (TB-G G1.2-3 → CAS Git repair → Phase 6.3 alpha → Phase 7 W0+W0.1 →
merge). Predecessors `85a29755` (W0.1 lock) and `46c1340a` (W0.1 toml)
explicitly named superseded. `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo`
passes 1/1.

---

## Pre-ship gates (7/7 — re-verified at HEAD `4259ee53`)

| # | Gate | Result |
|---|---|---|
| 1 | `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` | 1/1 |
| 2 | `cargo test --features web --test cli_web_smoke --test cli_web_routes_smoke --test cli_web_ws_smoke --test cli_web_write_smoke --test cli_web_spec_smoke --test cli_web_generate_smoke` | **254/254** (38+44+41+45+42+44) |
| 3 | `cargo test --test cli_wrapper_plumbing` (Phase 6.1 invariant) | 5/5 |
| 4 | `cd frontend && npm test` | 73/73 |
| 5 | `cd frontend && npm run build` | `dist/main.js` 49.9 kB |
| 6 | `cargo fmt --all -- --check` | exit 0 |
| 7 | `cargo build --bin turingos` + `cargo build --bin turingos_web --features web` | both 0 |

---

## Branch ancestry (21 commits ahead of `origin/codex/tisr-phase7-web`)

```
4259ee53  W6.1   serde(default) fix + regression test (§6a v2 finding closed)
dd1d5724  W6     frontend spec-grill + spec-result + artifact-viewer (centerpiece UX)
9186e49a  fmt    Phase 6.3 post-merge fmt normalization (continuation)
39c91d3b  fmt    Phase 6.3 post-merge fmt normalization
85b5fc94  W5     backend spec + generate + artifact endpoints
5e3ae33e  merge  origin/main (Phase 6.3 alpha) → Phase 7
1357c4ca  W4.3.1 lean_market build step in setup script
57251caa  W4.4   design system + UI/UX polish (Anthropic generative-UI guidance)
90a34aef  W4.3   workspace setup + teardown scripts
f849cb47  W4.2   in-memory task store + /api/tasks merge
fc1e5cdc  W4.1   embed frontend bundle at /static/main.js
9a168044  W4     write-path POST /api/task/open
111cf8b2  W3     frontend Web Components per IR block-type
38e3ff42  W0.1   axum features=["ws"] erratum (architect-ratified)
fa950beb  W2     backend WebSocket (axum::extract::ws)
63187013  W1     backend HTTP read-endpoints (UI IR → HTML + JSON)
0adffd50  W0     axum scaffold + Cargo unlock + Trust Root rehash + frontend init
1cab5c78  fix    macOS portability — `chain_tape_lease.rs` errno accessor
                 (also on `codex/macos-portability` as `cdd2ad7e`, ready
                  for an independent main PR cherry-picked off `origin/main`)
```

Plus 3 inherited Phase 6.3 commits from the merge:

```
53cc4442  Docs: update README + LATEST after PR #4 Phase 6.0-6.3 alpha ship (#5)
ff866c53  TISR Phase 6.0-6.3 alpha: turingos CLI + LLM-driven spec/generate + CAS wire (#4)
88fa1f66  Update main handover after CAS repair merge
```

---

## Known limitations (auditor-cleared; documented for forward planning)

1. **Page 2 `/agents` serves a static IR fixture**, not the real
   `agent_pubkeys.json` from `TURINGOS_WEB_WORKSPACE`. The §6a v2 verifier
   noted "agent_001 not in DOM" as expected fixture-mode. Closing this is a
   natural Phase 7.x or Phase 8 atom — the workspace already contains the
   right data; the backend just needs a small `agent_pubkeys.json → IR`
   merge similar to W4.2's task store.

2. **`/audit` reuses the dashboard fixture** until a distinct audit-view IR
   is produced. Phase 6.3's `turingos audit dashboard` regeneration could
   feed it; not in Phase 7 scope.

3. **Real Qwen3-Coder-30B LLM never invoked from this Mac** — Phase 7 ship
   evidence used `TURINGOS_BACKEND_OVERRIDE` stubs (W4-era pattern) for the
   §6a v2 verifier. The web substrate is fully proven; only the LLM quality
   is unverified, which is Phase 6.3 territory (already PR-#4-audited). To
   run for real: set `SILICONFLOW_API_KEY` env var (architect's choice — the
   key is never written to disk per Phase 6.3 invariant).

4. **`lean_market` legacy task-open path remains** — `<tos-task-open-form>` +
   POST `/api/task/open` from W4 still exist. Per architect re-orientation,
   the canonical write path is now `/build` → spec → generate. The legacy
   form is acceptable as a secondary surface; could be removed in Phase 7.x
   trim if desired.

---

## How to demo

### With stub backend (works on any machine; no API key)

```bash
cd /Users/zephryj/work/turingosv4
bash tests/frontend_e2e_setup_workspace.sh tmp/phase7_demo
WORKSPACE="$(pwd)/tmp/phase7_demo"

# Stub at handover/evidence/stage_phase7_web_v2_*/turingos_stub.sh is
# already written; reuse OR write your own per the §6a verifier evidence dir.

TURINGOS_WEB_WORKSPACE="$WORKSPACE" \
TURINGOS_BACKEND_OVERRIDE="<path-to-stub>" \
EVID_ABS="$(pwd)" \
target/debug/turingos_web

# Browser → http://127.0.0.1:8080/build
# Walk through the 8 questions; submit; "生成代码"; preview index.html
# in the sandboxed iframe.

# Cleanup
bash tests/frontend_e2e_teardown_workspace.sh tmp/phase7_demo
```

### With real Qwen3-Coder-30B (requires architect's SiliconFlow key)

```bash
cd /Users/zephryj/work/turingosv4
bash tests/frontend_e2e_setup_workspace.sh tmp/phase7_demo
export TURINGOS_WEB_WORKSPACE="$(pwd)/tmp/phase7_demo"
export SILICONFLOW_API_KEY="sk-..."  # never written to disk per Phase 6.3 invariant

target/debug/turingos_web

# Browser → http://127.0.0.1:8080/build
# Real spec interview → real Qwen synthesis → real LLM-generated UI in iframe.
```

---

## How to push + open PR

```bash
git push -u origin codex/tisr-phase7-web
gh pr create \
  --base main \
  --title "TISR Phase 7 Web MVP" \
  --body "$(cat handover/audits/AUDITOR_TISR_PHASE7_R1_PROCEED.md)"
```

The macOS portability fix lives in parallel on `codex/macos-portability`
(commit `cdd2ad7e`, branched cleanly off `origin/main`). It can be pushed +
PR'd independently first, or merged with Phase 7 — either order works since
the diff is purely additive (cfg-arms for `__errno_location` vs `__error`)
and the file is not Trust-Root-pinned.

```bash
# Optional: independent Mac portability PR first
git push -u origin codex/macos-portability
gh pr create --base main --title "runtime: macOS-compatible errno accessor in chain_tape_lease" --body "Phase 6.1 substrate portability bugfix; cherry-pickable to main independently of Phase 7."
```

---

## Forward-bound (Phase 7.x candidates, NOT in this ship's scope)

- **Real /agents page** — read `<workspace>/agent_pubkeys.json` at request time, merge with IR fixture (small W7 atom; mirrors the W4.2 task-store merge pattern)
- **Real /audit page** — wrap `turingos audit dashboard --chaintape <workspace>` output as IR
- **Session history page** — list `<workspace>/sessions/*/spec.md` previews; let user restore prior spec
- **Welcome / landing** — surface `turingos welcome` subcommand as an editorial onboarding flow at `/`
- **Spec interview streaming** — currently scripted-mode batch POST; could stream question-by-question through WebSocket for live conversational feel
- **Multi-artifact preview** — current viewer focuses on `index.html`; richer file browser for multi-file generates
- **Public binding flag** — per Phase 7 §7 "non-loopback binding requires explicit flag with audit-log entry"; deferred from Phase 7 ship per `127.0.0.1:8080` hard

---

## Lessons captured (for future ship gates)

1. **Charter mid-flight re-orientation is acceptable when architect-ratified** — Phase 7 §8 was drafted pre-Phase-6.3-ship; when Phase 6.3 landed on main, the Lean-substrate Page 4 criterion was abandoned via explicit architect chat ratification. Both decisions (W0.1 erratum + Page-4 re-aim) were captured in commit messages + evidence dirs, so the auditor saw clean ratification trails.

2. **Subagent dispatch with `isolation: "worktree"` can drift base** — the W0 sonnet executor's worktree was rooted at `origin/HEAD` (main) instead of the orchestrator's current branch HEAD. The agent recovered functionally but produced a fat commit mixing W0 scope with Phase 6.1 catchup; orchestrator (Haiku 4.5) integrated cleanly by extracting only the W0-scope diff. Subsequent atoms dropped worktree isolation and ran in-place. **Recommendation for future Phase work**: when isolation is desired, pre-create the worktree on the correct branch and pass the path to the agent.

3. **Class-3 rehash discipline pays off downstream** — recording every Trust Root event with full predecessor lineage in `genesis_payload.toml` comments meant the §5 auditor could verify all 3 events in one read. No "fishing for context" required.

4. **§6a verifier finding flowed cleanly into a regression test** — the HTTP 422 bug from §6a v2 became `generate_accepts_bare_session_id_payload_regression_w6_1` in W6.1, locking in the contract gap permanently. The auditor explicitly noted this is sufficient evidence to close the finding without a §6a v3 re-run.

5. **Anthropic-aesthetic compliance can be made testable** — `frontend/test/design-system.test.ts` asserts no Inter/Roboto/Arial and no purple-gradient-on-white. This kind of "anti-cliché" assertion catches generic-AI-SaaS drift at CI time, not at the final visual review.

---

## FC-trace summary

Phase 7 touched these flowchart nodes (per Phase 7 §8 §2):

- **FC1-N5 / FC1-N6**: read view surface expanded from CLI-only to HTTP + HTML (server-side IR materialization) + WebSocket-pushed IR + browser-side Web Components
- **FC1-N10 / FC1-N13**: write actions surface — two paths, both routed through existing CLI shellouts (no new sequencer admission): POST `/api/task/open` → `turingos task open` (Phase 6.1 substrate); POST `/api/spec/submit` + POST `/api/generate` → `turingos spec` + `turingos generate` (Phase 6.3 substrate)
- **FC2-N16**: boot/genesis surface extended with the `turingos_web` bin entry + workspace setup scripts. Trust Root rehash sequence (W0 + W0.1 + merge regenerate) recorded; `boot.rs` verifier accepts.
- **FC3-N31 / FC3-N39**: HTML render + artifact preview are materialized views (light DOM, sandboxed iframe), never authoritative. Same regeneratability requirement honored.

No Class 4 surface touched. Auditor verified empty git diff on:
`src/kernel.rs`, `src/bus.rs`, `src/sdk/tools/wallet.rs`, `src/state/sequencer.rs`,
`src/state/typed_tx.rs`, `src/bottom_white/cas/schema.rs`, `src/main.rs`,
`src/lib.rs`.

---

**End of TISR Phase 7 Web MVP Ship Report.**
