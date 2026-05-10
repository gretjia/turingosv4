# Next Session Boot Prompt — 2026-05-10 session #35 close (TB-N1-AGENT-ECONOMY Phase 1 SHIPPED + Phase 2 charter ratified)

> Paste the **`USER PROMPT`** block at the bottom into the next Claude session.

---

## State at session #35 close (2026-05-10)

- **HEAD on `origin/main`**: `ca2474d` (3 commits ahead of session #34 close `ff92646`; pushed).
- **Branch on `origin/feat/n1-econ-a3-rebuild`**: `1077bb7` (clean; ready for A3 STEP_B execution).
- **TB-N1-AGENT-ECONOMY Phase 1**: SHIPPED at `a5625a6`.
- **TB-N1-AGENT-ECONOMY Phase 2**: charter ratified at `1077bb7`; forward §8 grant (A3 + A4 serial) ACTIVE.
- **Constitution gates**: `267/0/1` (preserved at session #34 baseline).
- **Workspace tests** (`--test-threads=1`): `1427/0/151` (was 1418; +9 from A2 econ_position + prompt-block).
- **Trust Root**: PASS (post `experiments/minif2f_v4/src/bin/evaluator.rs` rehash `62834dff → 60f41bc8`).
- **`CONSTITUTION_EXECUTION_MATRIX.md`**: 0 RED + 0 AMBER (preserved).
- **`TRACE_FLOWCHART_MATRIX.md`**: 0 RED + 0 AMBER + 37 GREEN + 3 N/A (preserved).
- **FC1 / FC2 / FC3**: all GREEN.
- **Architect ship gates**: 9/10 GREEN; SG-B3.1-6 / M2 still single open set, **explicitly forbidden during Phase 2** per charter §4 + forward grant §3.

## What landed this session (#35)

| Phase | Commit | Class | Subject |
|-------|--------|-------|---------|
| Diagnosis | (unstaged smokes) | 2 | Empirical n=1 economy gap witness — 6-cell baseline smoke (`stage_b3_smoke_session35_20260510T082517Z`) showed `initial_balances=[]` + `accepted_tx_ids` only TaskOpen+terminal-summary (no EscrowLockTx) + prompt `Balance: 0 Coins` single line. Root cause: `TURINGOS_CHAINTAPE_PRESEED` env-default-off; M2 / Stage B3 batch never set it (other callers `comprehensive_arena.rs` + `lean_market.rs` did). |
| Phase 1 | `a5625a6` | 1+2 | A1 (Class-1) `scripts/run_stage_b3.sh` adds `TURINGOS_CHAINTAPE_PRESEED=1` → preseed engaged: 12 agents (tb7-7-sponsor + Agent_user_0 + Agent_0..9, 30M μC total); chain shows EscrowLockTx accepted to L4 per cell. A2 (Class-2) NEW `src/sdk/econ_position.rs::render_econ_position(q, agent_id)` reads canonical `EconomicState` (balances_t / stakes_t / claims_t / reputations_t); `build_agent_prompt` signature `balance: f64` → `econ_position: &str` renders under `=== Your Economic Position ===` heading; `experiments/minif2f_v4/src/bin/evaluator.rs:2098-2119` swap. +9 workspace tests (7 econ_position + 2 prompt). Trust Root rehashed `evaluator.rs`. Includes 3 smoke evidence dirs as load-bearing real-evidence per `feedback_real_problems_not_designed`. |
| Phase 2 | `1077bb7` | 0 + Class-4 forward grant | TB-N1-AGENT-ECONOMY Phase 2 charter (handover/tracer_bullets/) + forward §8 grant doc (handover/directives/). User verbatim Class-4 multi-clause forward grant: "批准 charter + 授权 A3 + A4 串行全授权" — clause 1 ratifies, clause 2 authorizes A3 then A4 serial conditional on per-atom dual audit PASS. Per `feedback_no_batch_class4_signoff` per-atom §8 cadence preserved. |
| Handover | `ca2474d` | 0 | LATEST.md session #35 close block. |

## §1 — Critical files for cold-start orientation

Read in this order:

1. **`CLAUDE.md`** — project constitution (§4 strategic decisions; §10 Class-4 authorization; §22 read order).
2. **`handover/ai-direct/LATEST.md`** — top "✅ Session #35 close" block.
3. **`handover/tracer_bullets/TB_N1_AGENT_ECONOMY_PHASE_2_charter_2026-05-10.md`** — Phase 2 charter.
4. **`handover/directives/2026-05-10_TB_N1_AGENT_ECONOMY_PHASE_2_FORWARD_§8_GRANT.md`** — forward §8 grant.
5. **`handover/alignment/N1_AGENT_ECONOMY_LANDING_GAP_2026-05-10_session35.md`** — empirical analysis + atom inventory.
6. **`MEMORY.md`** — `feedback_no_concurrent_dev_during_batch` (NEW session #34) + `project_economy_prompt_landing_gap` updates.
7. **`handover/evidence/stage_b3_smoke_session35_*/`** — 3 smoke evidence dirs.

## §2 — Pre-action gate (mandatory at next session start)

Per `MEMORY.md` MUST CHECK BEFORE:
- `/constitution-landing-check` — should return PROCEED (matrix unchanged at 0 AMBER).
- `/runner-preflight` — IF starting any `bash run_*.sh` runner script. Note Phase 2 forbids M2 batch (sequencer admission change in flight).
- Per `feedback_no_concurrent_dev_during_batch`: A3 STEP_B implementation is the active "batch" of code mutation; do NOT modify Trust-Root-pinned source files outside the feat branch + ship cycle.

## §3 — Forward queue (post Phase 1 ship; constitutional / FC / SG framing)

**Active forward grant**: A3 + A4 serial Class-4 STEP_B (per Phase 2 forward §8 grant `2026-05-10_..._FORWARD_§8_GRANT.md`). User verbatim "批准 charter + 授权 A3 + A4 串行全授权" — conditional on per-atom dual audit PASS.

| Item | Constitutional binding | Status |
|------|------------------------|--------|
| **(A3) Agent-decided stake** (Class-4 STEP_B) | CLAUDE.md §13 "writes/append/challenge/verify/settle require stake/escrow/bond as specified" — close agency layer for stake decision | **AUTHORIZED next-session start**. Branch `feat/n1-econ-a3-rebuild` ready (clean from main); see charter §2 atom inventory + §3 strict execution sequence. |
| **(A4) Agent-callable verify-peer** (Class-4 STEP_B) | §13 verify/bond agency + Art. I.1.1 multi-agent verification | **AUTHORIZED after A3 ships**. Per `feedback_no_batch_class4_signoff`: A4 starts only after A3 fully ships (per-atom §8 sign-off file written). |
| (A5) Prompt economic feedback (Class-2) | Art. I.1.1 statistical signal feedback to agent | OPEN; not in Phase 2 forward grant scope (separate Class-2 work). |
| (A6) Polymarket-agent-bridge (Class-4 STEP_B; Stage D-aligned) | Art. II.2 broadcast price signals + §13 verify/settle | DEFERRED — needs separate architect §8 + Stage D ship gate. |
| (B) Stage D real-world readiness | architect §B.9.1 explicit forbid + CLAUDE.md §20 freeze conditions | DEFERRED behind explicit architect ship gate (no spec exists). |
| (C) PromptCapsule evaluator wire-up (Class-3) | CLAUDE.md §4.3 G-016/G-019/G-021/G-028 | OPEN; not blocking Phase 2. |
| (D) CAS Merkle redesign (Class-3-4) | Stage A3.6 enhancement TB | DEFERRED. |
| **M2 100p batch under SG-B3.1-6** | Architect §Stage B spec | **FORBIDDEN during Phase 2** (per charter §4 + forward grant §3 — sequencer admission change in flight invalidates evidence). Eligible AFTER Phase 2 closes. |

**Recommended next-session path**: execute A3 STEP_B per Phase 2 forward grant.

## §4 — A3 STEP_B execution checklist (per Phase 2 charter §3)

Branch: `feat/n1-econ-a3-rebuild` (clean at HEAD `1077bb7`).

A3 implementation surface (per charter §2):
- `src/state/typed_tx.rs` — RejectionClass tail-append `StakeBalanceExceeded` + `TransitionError::StakeBalanceExceeded` variant
- `src/state/sequencer.rs` — WorkTx admission Step 4 extension: reject if `stake > agent_balance` (currently only checks `stake > 0`)
- `src/sdk/protocol.rs` — `AgentAction::Step` gains `stake_micro: Option<u64>` field
- `experiments/minif2f_v4/src/bin/evaluator.rs` — 3 OMEGA callsites at lines ~2329, 2645, 3231 (all currently use `TURINGOS_CHAINTAPE_PROPOSAL_STAKE_MICRO` env default 1000) thread `action.stake_micro` if present
- `src/sdk/prompt.rs` — step tool schema doc updated to mention `stake_micro` parameter
- `tests/constitution_n1_agent_economy_a3.rs` (NEW) — 5 ship gate tests:
  - SG-N1-A3.1: agent submits stake=0 → rejected with StakeInsufficient (existing behavior preserved)
  - SG-N1-A3.2: agent submits stake=balance+1 → rejected with NEW StakeBalanceExceeded
  - SG-N1-A3.3: agent submits stake=min_stake (1) → admitted (positive control)
  - SG-N1-A3.4: prompt's `Active stakes` line reflects per-cell agent-decided amounts post-A3
  - SG-N1-A3.5 (real-LLM smoke): 6-cell smoke shows ≥1 cell with WorkTx admitting agent-decided non-default stake
- `scripts/run_constitution_gates.sh` — register new gate file
- Trust Root rehash: sequencer.rs + typed_tx.rs + evaluator.rs (3 pinned files)

A3 protocol (per Phase 2 forward §8 grant §4):
1. STEP_B parallel-branch (already done: `feat/n1-econ-a3-rebuild`)
2. Implementation + cargo test --workspace + bash scripts/run_constitution_gates.sh GREEN
3. Real-LLM 6-cell smoke (`stage_b3_smoke_a3_<TS>` recommended)
4. PRE-§8 dual audit:
   - Codex G2 (`/codex:rescue` skill or direct dispatch)
   - Gemini DeepThink (separate dispatch)
   - BOTH PROCEED required per `feedback_dual_audit` Class-4
   - Round cap = 2 per `feedback_elon_mode_policy`
   - Conservative-merge VETO > CHALLENGE > PASS per `feedback_dual_audit_conflict`
5. Per-atom §8 sign-off: `handover/directives/2026-05-XX_TB_N1_AGENT_ECONOMY_A3_§8_SIGN_OFF.md` cites forward grant + R<final> dual audit PASS
6. Merge `feat/n1-econ-a3-rebuild` → `main` + final smoke + push
7. Update LATEST.md with A3 ship block

A3 kill condition (per charter §2): if SG-N1-A3.5 chain shows ZERO cells where agent picked non-default stake → A3 hasn't actually engaged agent agency; remediate or escalate.

## §5 — Forbidden during Phase 2 (per charter §4 + forward grant §3)

- **NO M2 batch run** (1800-cell SG-B3.1-6) — sequencer admission change in flight; M2 evidence would be invalidated
- **NO Polymarket-agent-bridge (A6)** — Stage D-aligned; needs separate architect §8
- **NO swarm n>1 batch** — substrate must close before swarm makes sense; A3+A4 are n=1 atoms
- **NO new typed_tx variant** — RejectionClass + TransitionError tail-append only
- **NO canonical signing payload change** — WorkTx + VerifyTx signing payloads unchanged
- **NO push to origin/main** without per-atom §8 sign-off (per atom; A3 first, then A4)

## §6 — Mechanism additions this session (forward defense)

- **NEW `src/sdk/econ_position.rs`** — agent-economy renderer; `render_econ_position(q, agent_id)` reads canonical EconomicState
- **MODIFIED `src/sdk/prompt.rs::build_agent_prompt`** — signature `balance: f64` → `econ_position: &str`; renders `=== Your Economic Position ===` block under heading
- **MODIFIED `experiments/minif2f_v4/src/bin/evaluator.rs:2098-2119`** — per-tx prompt build invokes render helper from `bus.sequencer.q_snapshot()`
- **MODIFIED `scripts/run_stage_b3.sh`** — `TURINGOS_CHAINTAPE_PRESEED=1` added to per-cell env block
- **NEW `handover/tracer_bullets/TB_N1_AGENT_ECONOMY_PHASE_2_charter_2026-05-10.md`** — Phase 2 charter
- **NEW `handover/directives/2026-05-10_TB_N1_AGENT_ECONOMY_PHASE_2_FORWARD_§8_GRANT.md`** — forward §8 grant
- **NEW `handover/alignment/N1_AGENT_ECONOMY_LANDING_GAP_2026-05-10_session35.md`** — empirical analysis
- **3 NEW evidence dirs** under `handover/evidence/stage_b3_smoke_session35_*/`

## §7 — Validation baseline at session #35 close

| Check | Value |
|-------|-------|
| HEAD (origin/main) | `ca2474d` (pushed) |
| Branch (origin/feat/n1-econ-a3-rebuild) | `1077bb7` (pushed; clean from main) |
| Constitution gates | 267/0/1 |
| Workspace tests | 1427/0/151 |
| Trust Root | PASS |
| `CONSTITUTION_EXECUTION_MATRIX.md` | 0 RED + 0 AMBER |
| `TRACE_FLOWCHART_MATRIX.md` | 0 RED + 0 AMBER + 37 GREEN + 3 N/A |
| FC1 / FC2 / FC3 | all GREEN |
| Architect ship-gate sets verified at HEAD | 9 / 10 (SG-B3.1-6 / M2 single open set; forbidden during Phase 2) |

## §8 — Memory verification at next-session start

Verify these are present in `MEMORY.md` (session-#35 additions / preserves):
- `feedback_no_workarounds_strict_constitution` (load-bearing for Phase 2 strict-constitution discipline)
- `feedback_no_batch_class4_signoff` (per-atom §8 cadence for A3 then A4)
- `feedback_step_b_protocol` (parallel-branch for sequencer admission changes)
- `feedback_dual_audit` (Class-4 timing rule: PRE-§8 not POST)
- `project_economy_prompt_landing_gap` (session #34 historical state; superseded by session #35 Phase 1 closure of perception layer)

If any are missing or stale, restore from session #35 commits.

---

## USER PROMPT (paste this into next Claude session)

```
Session #35 closed 2026-05-10 at HEAD `ca2474d` on origin/main (3 commits
on top of session #34 boot `ff92646`; pushed). Branch
`feat/n1-econ-a3-rebuild` at HEAD `1077bb7` (clean from main; pushed).

What landed this session:
- TB-N1-AGENT-ECONOMY Phase 1 SHIPPED at a5625a6 (Class-1+2):
  - A1: scripts/run_stage_b3.sh adds TURINGOS_CHAINTAPE_PRESEED=1
    so M2/Stage B3 batch engages the genesis preseed (12 agents,
    30M μCoin total). Empirically: pre-A1 chain had no
    EscrowLockTx; post-A1 chain shows escrowlock-task-...-tb7-7-d3-
    escrow accepted to L4 per cell. FC1 invariant Ok preserved.
  - A2: NEW src/sdk/econ_position.rs renders agent's canonical
    EconomicState (balances_t / stakes_t / claims_t / reputations_t)
    as `=== Your Economic Position ===` block in build_agent_prompt;
    signature changed `balance: f64` → `econ_position: &str`.
    Trust Root rehashed evaluator.rs 62834dff → 60f41bc8.
  - 3 smoke evidence dirs included as load-bearing real-evidence
    per feedback_real_problems_not_designed.
- TB-N1-AGENT-ECONOMY Phase 2 charter ratified at 1077bb7;
  forward §8 grant active for A3 + A4 serial Class-4 STEP_B
  conditional on per-atom dual audit PASS. User verbatim "批准
  charter + 授权 A3 + A4 串行全授权" — multi-clause forward Class-4
  grant per CLAUDE.md §10.

Constitution gates 267/0/1; workspace 1427/0/151; Trust Root PASS;
constitution matrix 0 RED + 0 AMBER; FC matrix 0 RED + 0 AMBER + 37
GREEN + 3 N/A.

Three strict-constitution holds carried forward (verbatim user
direction this + earlier sessions):
1. "我现在在引擎的开发阶段，我不要凑合，我需要的是宪法约定的内容
   全部真实落地且可被验证" (session #34) — drove the L4.E body
   integrity landing + this session's n=1 economy gap analysis.
2. "我不想听到哪种更简单，哪种更 cheap 这样的言论...我需要的是宪法
   以及宪法中三个 flow chart 的完整落地，还有架构师设计的 ship gate
   的完整的验证通过" (session #34) — forward work in constitutional
   / FC / SG framing only, NOT cost / ease.
3. "我要的是 TuringOS engine 有序完整落地" + "我不要凑活的方案，我
   不考虑成本和 easy" (session #35) — drove pivot from M2 launch to
   n=1 economy landing. M2 explicitly forbidden during Phase 2.

Read first:
1. handover/ai-direct/NEXT_SESSION_PROMPT_2026-05-10_post_session_35.md
   (this prompt's source; full context + atom checklist + spec citations)
2. handover/ai-direct/LATEST.md "✅ Session #35 close" block
3. handover/tracer_bullets/TB_N1_AGENT_ECONOMY_PHASE_2_charter_2026-05-10.md
4. handover/directives/2026-05-10_TB_N1_AGENT_ECONOMY_PHASE_2_FORWARD_§8_GRANT.md
5. handover/alignment/N1_AGENT_ECONOMY_LANDING_GAP_2026-05-10_session35.md
6. MEMORY.md (no session-#35 additions; carries session-#34 deltas)

Forward queue (constitutional / FC / SG framing only):

(A3) Agent-decided stake — Class-4 STEP_B. AUTHORIZED. Branch
     feat/n1-econ-a3-rebuild ready. Surface: typed_tx.rs RejectionClass
     tail-append + sequencer admission Step 4 extension + protocol
     AgentAction::Step optional stake_micro + 3 evaluator OMEGA
     callsites + prompt schema doc + 5 ship gate tests + Trust Root
     rehash. Protocol: implementation → smoke → PRE-§8 dual audit
     (Codex G2 + Gemini DeepThink BOTH PROCEED) → per-atom §8 →
     merge → push → A4 starts.

(A4) Agent-callable verify-peer — Class-4 STEP_B. AUTHORIZED after
     A3 ships per-atom §8. Same protocol pattern.

FORBIDDEN during Phase 2:
- M2 batch run (sequencer admission change in flight)
- Polymarket-agent-bridge (A6 → Stage D)
- swarm n>1 batch
- new typed_tx variant or canonical signing payload change
- push to origin/main without per-atom §8 sign-off

Recommended next action: switch to feat/n1-econ-a3-rebuild branch
+ start A3 implementation per charter §2 atom inventory + §3 strict
execution sequence. Per feedback_constitutional_harness_engineering:
harness → real run → audit → ship. NOT charter → audit → atom
(Phase 2 charter is already ratified; G1 charter audit is the
deprecated atomic-agentic-engineering pattern).
```

---

**End of session #35 close boot prompt.**
