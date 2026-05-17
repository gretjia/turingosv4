# TISR Phase 7 — Separate Charter Section 8 Packet

**Date drafted**: 2026-05-17 (post Phase 6.1 ship)
**Driver environment**: Mac Studio (local, macOS, Chrome installed)
**Predecessor**: Phase 6.1 §8 packet ratified 2026-05-17; Phase 6.1 shipped at commit `<TBD after R6 PROCEED>`
**Parallel sibling**: Phase 6.2 §8 packet (omega-vm headless) — independent, can ratify same day

---

## §1 Scope

This packet ratifies TISR Phase 7 Web MVP on the Mac Studio local track.
Phase 7 introduces the **first real generative-UI surface** for TuringOS
agent-OS read/write operations, driven by Claude Code via the
`mcp__Claude_in_Chrome__*` MCP tool stack against a real running Chrome
browser. The omega-vm headless track (Phase 6.2) and this local track
(Phase 7) proceed in parallel; both compile against the same git repo
but each touches an independent surface area.

Phase 7 ratifies:

1. **Backend web server** (`src/web/**` + new `[[bin]] turingos_web`):
   - HTTP listener on `127.0.0.1:8080` (no public binding by default;
     operator opt-in for non-localhost via flag)
   - HTML page serving from a fixture-backed Turing UI IR materializer
     (consumes the `experiments/tisr_ui_spike/` IR schema, but materializes
     to HTML instead of plain text)
   - JSON-RPC 2.0 read-endpoints over WebSocket OR Server-Sent Events
     for real-time ChainTape view updates (architect picks one in §3)
   - Read-only initially; one explicit write path through Phase 6.1's
     existing `turingos task open` shell-out (no new sequencer admission)

2. **Frontend** (`frontend/**`, NEW npm package, NOT a cargo workspace member):
   - React + TypeScript + Tailwind (or architect-substituted equivalent)
   - Consumes the Turing UI IR JSON over WebSocket/SSE
   - Renders the IR via component-per-block-type (text / table /
     agent_card / task_card / event_log / dashboard_panel)
   - No authoritative state; all backend interactions go through the
     existing shell-out CLI surface

3. **Cargo.toml unlock** (Trust Root rehash required; see §3):
   - New `[[bin]] name = "turingos_web"` entry
   - New dependencies: `axum = "0.7"`, `tower-http = "0.5"`,
     `tokio-tungstenite = "0.21"` (or `axum-extra` SSE; architect picks)
   - Existing `tokio` feature set extended if needed
   - All new deps gated behind `[features.web]` so 6.2 CLI-only builds
     stay unchanged

4. **MCP-driven testing** (no new crate dep; uses Claude Code's existing
   `mcp__Claude_in_Chrome__*` tool stack):
   - `navigate` + `get_page_text` + `javascript_tool` for E2E
   - `read_console_messages` + `read_network_requests` for diagnostics
   - Visual regression: screenshot diff via `mcp__Claude_in_Chrome__computer`
     captures (optional, architect-gated)
   - All tests run on Mac Studio with real Chrome; no headless CI for Phase 7
     (separate charter would address that)

This packet does NOT ratify:
- Phase 8 A2A deepening (DID bridge, MCP server export, periodic external
  anchoring)
- multimodal artifact storage (Phase 9+ AGI candidate)
- new typed transaction variants (still Class 4; separate per-variant §8)
- new signature types
- sequencer admission changes
- AgentProposedTaskOpen / AgentMarketSeeding / DirectSwapTx typed_tx
- new AgentRole variants

## §2 FC Mapping

Touched / extended nodes:
- FC1-N5 / FC1-N6: read view surface expanded from CLI-only to HTTP+HTML;
  same scoping/shielding rules apply (raw Lean stderr / private CoT
  shielded; only `public_summary` style fields rendered)
- FC1-N10 / FC1-N13: write action remains lawful — only one new path
  (browser-form-submits → `turingos task open` shell-out) — no new
  sequencer admission rule
- FC2-N16: boot/genesis surface unchanged (web server reads existing
  ChainTape; no new genesis path)
- FC3-N31 / FC3-N39: HTML render is a materialized view, NOT authority;
  same regeneratability requirement as CLI dashboards

New invariants for Phase 7:
- Web read view = `lib.rs` ChainTape replay → UI IR → HTML; never alternate
  source of truth
- Browser writes route through the existing `turingos task open` (or future
  Phase 8+ paths); no new HTTP write endpoint that bypasses CLI
- WebSocket / SSE deliver derived views only; not used as a tape
- `127.0.0.1:8080` only by default; non-loopback binding requires an
  explicit flag with audit-log entry

## §3 Risk Class

Default risk class: **Class 3** (production wire-up + auth-via-localhost
+ Trust Root rehash).

Sub-task classes:
- Class 0: docs, npm scaffold setup, fixture content
- Class 1: pure HTML/CSS rendering of fixed IR (no backend coupling)
- Class 2: backend HTTP server, WebSocket/SSE, frontend ↔ backend wiring
- Class 3: Cargo.toml unlock + Trust Root rehash, production HTML write
  paths (only `task open` shell-out is authorized here)
- Class 4 (NOT authorized in THIS packet; requires further architect §8):
  - cas/schema.rs ObjectType extensions (5 new variants per
    03_CODE_INTEGRATION_SPEC.md §3.6: UIEvent / A2AMessage / ArtifactRef
    if multimodal lands)
  - new typed_tx variants for browser-originated agent actions
  - new AgentRole variants

Critical decisions architect must make in §8 ratification:
- WebSocket vs SSE vs JSON-RPC over HTTP polling (architect picks; affects
  Cargo.toml deps and frontend complexity)
- React vs Vue vs Svelte vs vanilla TypeScript (architect picks)
- Trust Root rehash timing: pre-Phase-7-implementation or per-atom?

## §4 Allowed Paths

Allowed implementation surfaces:

- `src/web/**` (new)
- `src/bin/turingos_web.rs` (new bin)
- `frontend/**` (new npm package; NOT a cargo workspace member;
  outside cargo build)
- `Cargo.toml` (conditional — only for `[features.web]` deps and
  `[[bin]] turingos_web` entry; ratified by §8 signature)
- `Cargo.lock` (consequence of Cargo.toml changes)
- `tests/cli_web_*.rs` (new HTTP integration tests; Tokio-based)
- `tests/frontend_e2e_*.sh` (driver scripts for MCP Chrome automation;
  the actual tests run in Claude Code session, not cargo test)
- `experiments/tisr_ui_spike/**` (re-use for IR schema + fixtures)
- `handover/directives/2026-05-17_TISR_PHASE7_*` (this packet + amendments)
- `handover/reports/TISR_PHASE7_*`
- `handover/evidence/stage_phase7_web_*`
- `handover/audits/AUDITOR_TISR_PHASE7_*` (audit records produced by the
  clean-context Claude auditor agent; learning from
  Phase 6.1 R5 lesson)
- `handover/alignment/OBS_R022_TISR_PHASE7_*` (explicitly allowed)

This list is exhaustive. Disallowed paths trigger stop-and-ratify.

## §5 Exit Gates

All of the following must pass before §6 witness or ship:

- `cargo check` (root crate, with and without `--features web`)
- `cargo build --bin turingos` (CLI still works)
- `cargo build --bin turingos_web --features web`
- `cargo fmt --all -- --check`
- `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo`
  — Trust Root pass AFTER rehash with the new Cargo.toml entries
- `cargo test --workspace --no-fail-fast --features web` — 0 fail
- Frontend: `cd frontend && npm install && npm test && npm run build`
- E2E: a documented sequence of `mcp__Claude_in_Chrome__*` driver-script
  tests against the running localhost server
- One clean-context **Claude auditor agent** (`subagent_type: auditor`,
  `model: opus`, prompted to xhigh thinking depth) verdict = PROCEED.
  No external Codex CLI. See §9 for the multi-agent execution model.

## §6 Real Witness Requirement

Phase 7 ship requires producing `handover/evidence/stage_phase7_web_<timestamp>/`,
executed end-to-end by the autonomous Chrome-driving verification agent
specified in §6a. No human-in-the-loop. The 5-step pipeline:

1. Backend startup: `cargo run --bin turingos_web --features web`
   on Mac Studio; binds to `127.0.0.1:8080`
2. Frontend build + serve: `cd frontend && npm run build && npm run serve`
3. Browser session driven by the verifier agent via Chrome MCP:
   - Navigate to `http://localhost:8080`
   - Verify dashboard view renders (UI IR → HTML correctly)
   - Open agent view; verify agent registration data renders
   - Open task view; verify shellout-driven task lifecycle reflects
   - Trigger a write via the rendered "Open Task" form;
     `turingos task open` shells out via WebSocket-relayed RPC;
     verify ChainTape advances + new task visible in audit dashboard
4. Replay: same ChainTape replayed via `turingos replay` produces the
   same view; no divergence
5. Console + network logs captured via Chrome MCP; no panics; no XSS
   warnings; no auth/CSRF bypass attempts visible

The witness may be partial. NOT acceptable: a screenshot of HTML without
a working write-back path proving the substrate.

## §6a Autonomous Verification Agent (Chrome-driven)

The §6 pipeline is executed **end-to-end by an independent verification
agent**, dispatched once by the orchestrator and running unsupervised
until final verdict. Phase 7's verifier is fundamentally different from
Phase 6.2's because the work product (HTML rendered in a real browser)
is potentially subjective. To eliminate human-in-the-loop while preserving
trustworthy decisions, the verifier relies on **mechanical inspection
criteria over rendered DOM + network traffic + console logs**, not visual
judgment.

**Why this approach is sound**: per the research report on backend-agnostic
CLI testing (`mcp__Claude_in_Chrome` MCP tool stack), Chrome exposes DOM
text via `get_page_text`, executed JS state via `javascript_tool`, network
requests/responses via `read_network_requests`, and console messages via
`read_console_messages`. Every page state the agent needs to verify can
be expressed as: "the DOM contains X" + "no console.error occurred" +
"network request Y returned status Z". No "does this look right" question
enters the agent's decision loop.

### Agent specification

| Field | Value |
|---|---|
| Dispatch | `Agent` tool with `subagent_type: general-purpose`, `model: opus`, no isolation (driver session needs access to local Chrome) |
| Tools allowed | `mcp__Claude_in_Chrome__navigate`, `mcp__Claude_in_Chrome__get_page_text`, `mcp__Claude_in_Chrome__javascript_tool`, `mcp__Claude_in_Chrome__read_console_messages`, `mcp__Claude_in_Chrome__read_network_requests`, `mcp__Claude_in_Chrome__computer` (screenshot only), `mcp__Claude_in_Chrome__form_input`, `Bash` (start/stop servers; no source edit), `Read`, `Write` (`handover/evidence/stage_phase7_web_*` only), `Glob`, `Grep` |
| Tools forbidden | `Edit`, `mcp__Claude_in_Chrome__javascript_tool` write to production state (read-only DOM inspection only), `WebSearch`, `WebFetch` |
| Input | (a) commit SHA of Phase 7 ship candidate; (b) localhost URL + port; (c) the 5-step pipeline from §6; (d) per-page DOM/network/console criteria (see below); (e) evidence directory path |
| Output | `handover/evidence/stage_phase7_web_<timestamp>/agent_verdict.json` + per-page screenshots + console logs + network logs |
| Timeout | 60 min wall clock (npm build + cargo build + 5 page sessions + cleanup) |
| Failure escalation | Same as Phase 6.2: only if the agent crashes mid-run OR `agent_verdict.json` is unparseable. Otherwise the verdict is authoritative for the witness layer. |

### Per-page mechanical decision criteria

The verifier inspects each page via `get_page_text` + `javascript_tool`
calls. Decisions reduce to substring / structural / numeric comparisons —
zero visual judgment.

**Page 1: Dashboard view** (`http://localhost:8080/`)
- DOM contains text "TuringOS" AND text matching `/Phase \d/`
- DOM contains at least one `[data-block-type]` element
- Console message count: 0 with `level=error`
- Network: GET / returned status 200; one WebSocket OR SSE connection established (if SSE: `text/event-stream` content-type; if WS: HTTP 101 Upgrade)

**Page 2: Agent view** (`http://localhost:8080/agents`)
- DOM contains text "agent_001" (planted by verifier earlier via `turingos agent deploy`)
- DOM contains valid 10-AgentRole-set text (Solver / Verifier / Challenger / Trader / MarketMaker / Architect / Veto / Observer / BullTrader / BearTrader)
- Console error count: 0
- Network: `/api/agents` returned status 200 with `application/json` body containing `agent_001` key

**Page 3: Task view** (`http://localhost:8080/tasks`)
- DOM contains text matching `/task_[0-9a-f]+/` (a task id format)
- DOM contains status badge text matching one of: "open" / "accepted" / "rejected" / "finalized" / "bankrupt" / "expired"
- Console error count: 0
- Network: `/api/tasks` returned status 200

**Page 4: Form submission** (open task via browser form)
- Agent fills form fields via `mcp__Claude_in_Chrome__form_input` (problem id + bounty + agent id)
- Submits form
- Network: one POST or WS-send to backend with form payload
- Backend (verified out-of-band via shell-out): `lean_market run-task` was invoked (via existing turingos task open shellout); ChainTape advanced by at least 2 txs (TaskOpen + EscrowLock)
- DOM updates to show the new task in the task list within 5 sec
- Console error count: 0

**Page 5: Audit dashboard with the new task**
- After form submit, navigate to `http://localhost:8080/audit`
- DOM contains the task_id from page 4
- DOM contains the agent_id from page 4
- Console error count: 0

**Cross-check: backend ChainTape state**
After the browser session, verifier shells out to:
```
turingos report wallet --chaintape <workspace>
turingos replay --chaintape <workspace>
turingos audit dashboard --chaintape <workspace>
```
- Wallet shows the agent's balance changed (escrow locked)
- Replay 7-indicator report is all GREEN
- Audit dashboard contains the same task_id the browser saw
- Replay → audit dashboard regeneration → state_root matches the WebSocket-pushed state_root in step 5

### Anti-hallucination safeguards

The verifier writes ONLY to the evidence directory. It cannot:
- Edit source code
- Re-run cargo build (the binary is pinned to the audited commit at session start)
- Bypass the network capture (every HTTP/WS interaction is logged)
- Skip a page check (the 5-page sequence is fixed; failing any page = overall FAIL)

If the verifier finds a check failed, it does NOT attempt to fix the
underlying code. It logs the failure to evidence and continues with
remaining pages (so the architect + the §5-mandated clean-context
auditor agent see the FULL picture of what's broken, not just the first
failure).

### Evidence schema (`agent_verdict.json`)

```json
{
  "agent_id": "verifier_phase7_<uuid>",
  "branch_head": "<commit-sha>",
  "chrome_version": "<from mcp__Claude_in_Chrome>",
  "started_at_unix": <epoch>,
  "completed_at_unix": <epoch>,
  "wall_clock_seconds": <int>,
  "overall_verdict": "PASS" | "FAIL" | "PARTIAL",
  "page_results": [
    {
      "page": 1, "url": "http://localhost:8080/", "verdict": "PASS",
      "checks": [
        {"name": "contains_turingos_text", "expected": "TuringOS", "actual_excerpt": "...", "pass": true},
        {"name": "contains_phase_marker", "pattern": "Phase \\d", "match": "Phase 7", "pass": true},
        {"name": "console_error_count_zero", "actual": 0, "pass": true},
        ...
      ],
      "screenshot_path": "page1_dashboard.png",
      "console_log_path": "page1_console.json",
      "network_log_path": "page1_network.json"
    },
    ...
  ],
  "backend_crosscheck": {
    "wallet_balance_changed": true,
    "replay_all_green": true,
    "state_root_match": true
  },
  "fail_reasons": []
}
```

### Optional: Second-pass cross-verifier agent

For Class 3 ship gates, the orchestrator MAY dispatch a second verifier
agent (independent worktree, same model, same MCP tools) to re-run the
same §6 pipeline. The two verdicts must agree (both PASS or both FAIL)
for ship. Disagreement → halt; architect reviews evidence dirs from
both agents.

This is opt-in (orchestrator's choice); default is single-verifier.

## §7 Constraints

Hard:
- Trust Root rehash: AUTHORIZED in this packet for `Cargo.toml` +
  `Cargo.lock` only (NOT for `src/lib.rs`, NOT for `src/main.rs`, NOT
  for any other Trust Root pinned file). Rehash sequence must be
  recorded in `handover/evidence/stage_phase7_web_<timestamp>/trust_root_rehash.json`.
- Class 4 STEP_B surfaces (kernel / bus / wallet / sequencer / typed_tx
  / cas/schema / boot / RootBox / signing payloads) are NOT touched
  unless a FURTHER explicit §8 amendment ratifies them.
- No public network binding by default. Operator opt-in flag for
  non-loopback binding; audit log entry required.
- No multimodal data path (audio / video / images bigger than 64 KB
  thumbnails); architect-gated for Phase 8+.
- No user authentication / authorization beyond localhost trust; that's
  Phase 8+ scope.

Soft:
- Frontend codebase ≤ 5000 LOC for Phase 7 ship-witness; further
  expansion = Phase 7.x amendment
- E2E test scripts kept as bash + Claude Code MCP driver invocations;
  no Playwright / Cypress dependency
- Visual regression: snapshot via Chrome MCP `mcp__Claude_in_Chrome__computer`,
  diff manually; no perceptual-diff library introduction in this packet

## §8 Architect Sign-off

I, as the user/architect, hereby ratify TISR Phase 7 separate charter:
__authorize implementation on `codex/tisr-phase7-web` branch on the
Mac Studio local track (git pull from origin); allowed paths per §4;
exit gates per §5; real witness per §6; Class 3 default risk class with
Class 4 escalation rules per §3; Trust Root rehash authorized for
Cargo.toml + Cargo.lock only.__

This packet is independent of Phase 6.2 and may proceed in parallel.

Signed (verbatim, 2026-05-17): "都按你建议，你输出我在本地macstudio上claude
code的boot prompt，包括要git pull 的worktree信息" — confirmed in the
architect's 2026-05-17 conversation thread, in direct response to the
orchestrator's four Phase 7 decision recommendations:

**Resolved decisions** (all four ratified by the architect's verbatim
"都按你建议"):

1. **Real-time transport: WebSocket** (over SSE or JSON-RPC polling).
   Rationale: bidirectional, real-time ChainTape tape-update push;
   single connection per browser session; well-supported by axum +
   tokio-tungstenite; mature ecosystem.

2. **Frontend stack: vanilla TypeScript + Web Components** (over React /
   Vue / Svelte). Rationale: keeps ≤5000 LOC ceiling comfortable; no
   framework bundle bloat; native browser API (no virtual DOM tax);
   easier for the Phase 7 §6a Chrome-driven auditor to inspect (no
   shadow root unless explicitly opted in); future Phase 8+ can swap
   to a framework without rewriting state model.

3. **Trust Root rehash timing: W0 one-shot** (over per-atom). Rationale:
   single deterministic rehash event at the start of Phase 7; all
   atoms after W0 operate under the new Trust Root state; minimizes
   audit-window where Cargo.toml + Cargo.lock are mid-flux.

4. **`localhost:8080` only: HARD constraint** (over soft / network-exposed).
   Rationale: Phase 7 is local Mac Studio Web MVP, not a public service;
   non-loopback binding is Phase 8+ scope; eliminates entire class of
   auth/CSRF/CORS attack surface for this ship gate.

---

## §9 Multi-agent execution model (load-bearing for §5 audit gate)

Per architect directive 2026-05-17 (verbatim "调用自己的clean context
agent进行审计就可以...审计师的thinking level为xhigh...沿用我们成功的
multi-agents的工作方式"), Phase 7 abandons the external `codex exec`
CLI audit pattern. All audits are produced by Claude subagents
dispatched within the orchestrator's session via the `Agent` tool.

### Role × model × thinking-depth matrix

| Role | `subagent_type` | `model` | Thinking depth | Triggers / scope |
|---|---|---|---|---|
| Orchestrator (architect-side Claude Code session) | n/a (driver) | opus 4.7 | **xhigh** (prompt-driven) | Plan / dispatch / fan-in / unified review / commit-time decisions / Chrome MCP coordination |
| W0 foundation atom executor (axum scaffolding + npm init + Cargo.toml unlock) | general-purpose | sonnet | **high** (prompt: "treat Cargo.toml + Trust Root rehash as Class 3; cite every line you add; refuse to invent feature flags") | One-shot at Phase 7 start; Trust Root rehash sequence |
| W1 backend atom executors (HTTP read-endpoints) | general-purpose | sonnet | default | Per-endpoint atoms; backend-only; no JS |
| W2 backend wiring (WebSocket/SSE) | general-purpose | sonnet | **medium** (prompt: "consider lifecycle: connection, message framing, error path, reconnect") | One-shot per protocol choice |
| W3 frontend atom executors (HTML/CSS/TS per IR block-type) | general-purpose | sonnet | default | Per-component atoms |
| W4 write-path integration (browser → backend → CLI shellout) | general-purpose | sonnet | **medium** (prompt: "trace the request lifecycle end-to-end before edit") | Single critical-path atom |
| §6a autonomous verifier (Chrome-driven) | general-purpose | opus | **high** (prompt: "execute each check mechanically; record raw DOM/network/console; never substitute inference") | Once per ship-candidate commit; produces evidence + verdict.json + screenshots |
| **Clean-context auditor (the §5 ship gate)** | **auditor** | **opus 4.7** | **xhigh** (prompt: "audit with maximum rigor; cite file paths + line numbers; distinguish production defects from test-scaffold gaps; reason about Trust Root rehash sequence + Cargo.toml diff + Class 4 forward-bound boundary") | Once per ship-candidate; replaces former Codex CLI audit |
| (Optional) cross-verifier (independent second pass) | general-purpose | sonnet | default | If Class 3 ship gate or if first verifier returns CHALLENGE on a borderline check |

### Audit-gate enforcement

The §5 audit gate REQUIRES dispatching the auditor subagent from the
orchestrator session with these literal parameters:

```
Agent(
  description: "Phase 7 clean-context ship audit",
  subagent_type: "auditor",
  model: "opus",
  prompt: "<full audit packet specifying scope, comparison base, criteria;
           prompt MUST begin with the exact phrase 'Audit at xhigh thinking
           depth.' and explicitly cover:
           - Cargo.toml diff (new deps, [[bin]] entries, [features.web]
             gate, Trust Root rehash sequence)
           - src/web/** + src/bin/turingos_web.rs (new surface; class 2-3)
           - frontend/** (new npm package, separate compilation)
           - §6a verifier evidence directory (screenshots + network + console
             logs all parse-able)
           - Class 4 forward-bound items NOT touched
           - All paths in §4 allowed list>"
)
```

The auditor verdict is recorded at
`handover/audits/AUDITOR_TISR_PHASE7_<round>_<verdict>.md`. PROCEED →
ship; CHALLENGE → atom-level fix + re-audit; VETO → halt + architect
re-ratification.

### Cross-verifier escalation (Phase 7 only)

Phase 7's HTML output has more decision surface than Phase 6.2's CLI
output. If the auditor returns CHALLENGE on a borderline check
(e.g., "DOM contains expected text BUT also contains an unexpected
fragment"), the orchestrator MAY dispatch a second-pass verifier
(model: sonnet, default thinking) re-running the §6a Chrome MCP
sequence from scratch on the same commit. Two-verifier agreement is
required for ship under this escalation; disagreement halts to
architect review.

This is opt-in: default is single-verifier + single-auditor.

### Why this is sound on Mac Studio (no Codex)

1. **Mac Studio doesn't need codex-cli installed**. The Claude Agent
   SDK ships with the orchestrator's Claude Code session itself.
2. **Independence is preserved**: each `Agent` invocation gets a fresh
   context window — equivalent to clean-context Codex CLI.
3. **Chrome MCP is local**: `mcp__Claude_in_Chrome__*` tools drive the
   real Chrome browser installed on Mac Studio. The auditor doesn't
   need browser access (it reads the verifier's recorded evidence dir
   + the source diff).
4. **Reproducibility**: full prompt committed to evidence dir before
   dispatch; future re-audit can verify identical input.
5. **Thinking-depth control**: enforced via prompt prefix; auditor
   subagent type ships with read-only tools (Read, Glob, Grep, Bash)
   which is the right scope for an audit.

---

## Driver / orchestrator notes (not part of ratification)

### Environment setup checklist (Mac Studio)

```bash
# Once user-as-architect ratifies §8 above:
brew install rustup  # if not present
rustup install stable
rustup default stable
brew install node    # or use nvm
brew install --cask google-chrome  # if not present

git clone https://github.com/gretjia/turingosv4.git
cd turingosv4
git checkout codex/tisr-phase7-web  # branch I create from main after Phase 6.1 ship
```

### Tooling stack

Driver (Claude Code in this conversation):
- `mcp__Claude_in_Chrome__navigate` — URL navigation
- `mcp__Claude_in_Chrome__get_page_text` — capture rendered text
- `mcp__Claude_in_Chrome__javascript_tool` — DOM inspection / form fill
- `mcp__Claude_in_Chrome__read_console_messages` — capture JS errors
- `mcp__Claude_in_Chrome__read_network_requests` — capture HTTP/WS traffic
- `mcp__Claude_in_Chrome__computer` — visual / screenshot capture
- `mcp__Claude_in_Chrome__form_input` — form submission

### Sequencing

After §8 ratification AND Phase 6.1 ship:
- W0: I create branch `codex/tisr-phase7-web` from main; initial commit
  scaffolds `src/web/mod.rs` + minimal axum hello-world + frontend npm
  init
- W1: backend HTTP read-endpoints serving UI IR fixtures as HTML
- W2: WebSocket/SSE wiring + frontend npm-package skeleton
- W3: Frontend HTML/CSS/TS rendering matching the UI IR schema
- W4: Write path — browser form → backend → `turingos task open` shellout
- W5: E2E witness + clean-context auditor agent + ship

### Parallel-with-Phase-6.2 coordination

Same git remote, different branches:
- Phase 6.2 on `codex/tisr-phase6-2-cli` (omega-vm)
- Phase 7 on `codex/tisr-phase7-web` (Mac Studio)
- Cargo.toml is touched only on Phase 7 branch; Phase 6.2 leaves it alone
- Periodic `git fetch` + rebase on `main` once Phase 6.1 lands
- Final integration: Phase 6.2 → main first (smaller diff), then Phase 7 → main
  (larger diff with Cargo.toml unlock)

### No external Codex CLI needed on Mac Studio

Per architect directive 2026-05-17, all audits are produced by Claude
subagents dispatched via the `Agent` tool within the orchestrator's
Claude Code session. No external `codex exec` invocation; no codex-cli
npm dependency on Mac Studio. The auditor subagent runs in-process,
inherits the local repo / branch / target/ state, and writes its verdict
to `handover/audits/AUDITOR_TISR_PHASE7_<round>_<verdict>.md`. Audit
substrate is identical on omega-vm and Mac Studio (Claude Agent SDK).

See §9 for the full multi-agent execution model.
