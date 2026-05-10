# Next Session Boot Prompt — 2026-05-10 session #36 close (TB-N1-AGENT-ECONOMY Phase 2 SHIPPED FINAL: A3 + A4 both ratified)

> Paste the **`USER PROMPT`** block at the bottom into the next Claude session.

---

## State at session #36 close (2026-05-10)

- **HEAD on `origin/main`**: `db6fc3f` (5 commits ahead of session #35 close `e28b570`; pushed).
- **Branch `feat/n1-econ-a3-rebuild`**: merged + rebased into main; safe to delete locally.
- **Branch `feat/n1-econ-a4-rebuild`**: merged + rebased into main; safe to delete locally.
- **TB-N1-AGENT-ECONOMY Phase 2**: **SHIPPED FINAL** at session #36 (A3 + A4 both shipped serial with per-atom §8 sign-offs).
- **Constitution gates**: **279/0/1** (was 267 at session #35 close; +12 across A3+A4: `constitution_n1_agent_economy_a3` 5 tests + `constitution_n1_agent_economy_a4` 7 tests).
- **Workspace tests** (`--test-threads=1`): **1439/0/151** (was 1427 at session #35 close; +12 from new SG-N1-A3.* + SG-N1-A4.* gate tests).
- **Trust Root**: PASS (5 STEP_B files rehashed across A3+A4: `src/state/sequencer.rs`, `src/state/typed_tx.rs`, `src/state/q_state.rs`, `experiments/minif2f_v4/src/bin/evaluator.rs`, `src/bottom_white/ledger/transition_ledger.rs` test-fixture-only).
- **`CONSTITUTION_EXECUTION_MATRIX.md`**: 0 RED + 0 AMBER (preserved).
- **`TRACE_FLOWCHART_MATRIX.md`**: 0 RED + 0 AMBER + 37 GREEN + 3 N/A (preserved).
- **3-FC alignment**: FC1 + FC2 + FC3 all verified empirically 6/6 cells on A4 smoke; A3 smoke separately verified.
- **Architect ship-gate sets verified at HEAD**: 9/10 (SG-B3.1-6 / M2 still single open set; **now ELIGIBLE — Phase 2 charter §4 freeze conditions cleared**).

## What landed this session (#36)

| Phase | Commit | Class | Subject |
|-------|--------|-------|---------|
| Pre-A3 charter | `1077bb7` (prior session) | 0 + Class-4 forward grant | TB-N1-AGENT-ECONOMY Phase 2 charter + forward §8 grant ratified |
| A3 impl | `985e9fc` | 4 STEP_B | A3 agent-decided stake admission: typed_tx 1 RejectionClass + 1 TransitionError; sequencer Step-4b (stake>balance → StakeBalanceExceeded); protocol AgentAction.stake_micro; evaluator 3 OMEGA callsites threading; prompt schema doc; 5 SG-N1-A3.* tests; Trust Root rehash 4 STEP_B files |
| A3 smoke | `98cd9f4` | 2 | A3 6-cell smoke 6/6 GREEN, 1 OmegaAccepted |
| A3 R2 fix | `cbfb50b` / `c594f59` | 4 | SG-N1-A3.5 logic fix + R2 Codex Q4 saturating cast + Q6 schema imprecision |
| A3 packet + §8 | `010187b` / `dc24619` | 0 | §8 packet finalize + verbatim "好，确认可以 ship" sign-off |
| A3 post-ship | `535d760` | 2 | A3 post-merge final smoke 6/6 GREEN |
| Phase 2 LATEST.md (A3) | `80894cc` | 0 | LATEST.md A3 ship handover |
| A4 impl | `31fb6a2` | 4 STEP_B | A4 agent-callable verify-peer: typed_tx 3 RejectionClass + 3 TransitionError; q_state AgentVerificationsIndex 15→16 sub-fields; sequencer Step-2.5 + Step-3 rename + Step-3.5 + Step-5b; protocol verify_peer fields; evaluator dispatch + saturating cast; prompt schema doc; 7 SG-N1-A4.* tests; Trust Root rehash 4 STEP_B files |
| A4 smoke + script | `69910fe` | 2 + 1 | A4 n=2 swarm 6-cell smoke 6/6 GREEN, 1 OmegaAccepted (3rd consecutive); run_stage_b3.sh CONDITION env override |
| A4 packet | `fcd0c7a` | 0 | §8 packet finalize (R1 dual audit BOTH PASS first-try) |
| A4 §8 | `98c1908` | 0 | A4 verbatim "好，确认可以 ship" sign-off (second canonical §8 form in session) |
| Phase 2 LATEST.md (A4) | `db6fc3f` | 0 | LATEST.md A4 ship + Phase 2 closure handover |

**Total**: 12 commits across A3 + A4 (impl + smoke + audit-fix + packet + §8 + LATEST.md per atom).

## §1 — Critical files for cold-start orientation

Read in this order:

1. **`CLAUDE.md`** — project constitution (§3 three-flowchart gates; §10 Class-4 authorization; §22 read order).
2. **`handover/ai-direct/LATEST.md`** — top "✅ Session #36 (continued) ... Phase 2 SHIPPED FINAL" block.
3. **`handover/directives/2026-05-10_TB_N1_AGENT_ECONOMY_A4_§8_SIGN_OFF.md`** — most recent §8 sign-off; cites forward queue.
4. **`handover/directives/2026-05-10_TB_N1_AGENT_ECONOMY_A4_§8_PACKET.md`** — A4 full packet with §6 3-FC alignment analysis + §7 strict-vs-weak witness analysis.
5. **`handover/directives/2026-05-10_TB_N1_AGENT_ECONOMY_A3_§8_SIGN_OFF.md`** — A3 §8 (for forward-grant clause-2 reference).
6. **`handover/tracer_bullets/TB_N1_AGENT_ECONOMY_PHASE_2_charter_2026-05-10.md`** — charter (Phase 2 closed; reference for future atom scoping).
7. **`MEMORY.md`** — index. Key entries: `project_economy_prompt_landing_gap` (uptake gap A5 addresses); `OBS_TB_N1_A3_R2_I64_SATURATING_EDGE` (forward OBS); `feedback_no_batch_class4_signoff`.

## §2 — Pre-action gate (mandatory at next session start)

Per `MEMORY.md` MUST CHECK BEFORE:
- `/constitution-landing-check` — should return PROCEED (matrix unchanged at 0 AMBER).
- `/runner-preflight` — IF starting any `bash run_*.sh` runner script (M2 batch, A5 smoke, etc.).
- Per `feedback_constitutional_harness_engineering`: harness → real run → audit → ship.

## §3 — Forward queue (Phase 2 freeze lifts post-A4 ship)

**No active forward grant**: Phase 2 forward §8 grant clause 2 "授权 A3 + A4 串行全授权" fully discharged at A4 ship. New atom work requires fresh authorization per CLAUDE.md §10.

| Item | Class | Authority | Eligibility | Notes |
|------|-------|-----------|-------------|-------|
| **A5** Prompt economic feedback | 2 | Art. I.1.1 statistical signal feedback | ✅ ELIGIBLE | Addresses A3/A4 uptake gap empirically demonstrated (0/12 cells across 2 atoms × 6 cells each). Class-2 prompt-training work; no Class-4 §8 required. |
| **M2 100p batch** (SG-B3.1-6 1800 invocations) | 2-3 (operational) | TB-18B charter §1 + architect §Stage B spec | ✅ NOW ELIGIBLE | Phase 2 sequencer admission changes complete; charter §4 freeze conditions cleared. Use `scripts/run_stage_b3.sh` (now supports CONDITION env override post-A4). |
| **A6** Polymarket-agent-bridge | 4 STEP_B (Stage D-aligned) | Art. II.2 + §13 | DEFERRED | Needs separate architect §8 + Stage D ship gate. Forward-bound. |
| **Stage D** real-world readiness | unknown | architect §B.9.1 + CLAUDE.md §20 | DEFERRED | Behind explicit architect §8. NO autonomous start. |
| **PromptCapsule** evaluator wire-up | 3 | CLAUDE.md §4.3 G-016/G-019/G-021/G-028 | OPEN | Not blocking. |
| **CAS Merkle redesign** | 3-4 | Stage A3.6 enhancement TB | DEFERRED | Forward-bound. |

**Recommended next-session work** (per Constitutional Harness Engineering mode):

**Option (a) — A5 prompt economic feedback (Class-2; addresses uptake gap)**: empirical data from A3+A4 shows agents don't natively use stake_micro / verify_peer without prompt training. A5 closes this gap by extending the prompt with explicit examples (few-shot stake_micro usage; verify_peer multi-agent flow) and/or adding economy-aware reasoning instructions. Class-2 work; orthogonal to substrate; no §8 needed.

**Option (b) — M2 100p batch (canonical benchmark)**: now eligible since Phase 2 sequencer admission changes are complete. 1800 invocations (100 problems × n=3 × 3 seeds × 2 models). Uses `scripts/run_stage_b3.sh` (production runner). Per TB-18B charter §1 + architect §Stage B spec. Estimated wall time: 12-24 hours; uses LLM API budget significantly. **Recommend smoke-before-batch** per `feedback_smoke_before_batch`.

**Option (c) — Stage D real-world readiness**: needs architect ratification; deferred.

## §4 — Mechanism additions this session (forward defense)

- **NEW `tests/constitution_n1_agent_economy_a3.rs`** (5 SG-N1-A3.* tests) — agent-decided stake admission gate
- **NEW `tests/constitution_n1_agent_economy_a4.rs`** (7 SG-N1-A4.* tests) — agent-callable verify-peer
- **NEW state index `EconomicState.agent_verifications_t: AgentVerificationsIndex`** (`BTreeSet<(AgentId, TxId)>`; pure-additive; #[serde(default)]; NOT a Coin holding)
- **NEW 4 RejectionClass + 4 TransitionError variants** (StakeBalanceExceeded + VerifyBondOutOfBounds + VerifyTargetNotAccepted + VerifyDuplicate; pure-additive tail-append)
- **NEW agent tool actions**: `step` with optional `stake_micro` + NEW `verify_peer` tool (target_work_tx_id + verdict + bond_micro)
- **NEW sequencer admission gates**: WorkTx Step-4b + VerifyTx Step-2.5 + Step-3 rename + Step-3.5 + Step-5b
- **NEW evaluator dispatch arm**: `"verify_peer" =>` in `match action.tool.as_str()`
- **NEW `run_stage_b3.sh` CONDITION env override** (default n1; A4 SG-N1-A4.6 swarm smoke uses n2)
- **NEW handover docs**:
  - `handover/directives/2026-05-10_TB_N1_AGENT_ECONOMY_A3_§8_PACKET.md` + SIGN_OFF
  - `handover/directives/2026-05-10_TB_N1_AGENT_ECONOMY_A4_§8_PACKET.md` + SIGN_OFF
- **NEW MEMORY entry**: `OBS_TB_N1_A3_R2_I64_SATURATING_EDGE` (A3 R2 forward OBS — saturating cast edge case constitutionally unreachable per §13 mint ceiling)
- **6 NEW evidence dirs**:
  - `stage_b3_smoke_a3_20260510T114738Z/` (A3 pre-§8 smoke 6/6 GREEN)
  - `stage_b3_smoke_a3_post_ship_20260510T133947Z/` (A3 post-merge smoke 6/6)
  - `stage_b3_smoke_a4_20260510T222030Z/` (A4 pre-§8 swarm n=2 smoke 6/6 GREEN; 1 OmegaAccepted)
  - `stage_b3_smoke_a4_post_ship_20260510T231830Z/` (A4 post-merge smoke — committed post session #36 close if smoke completes before session boundary; otherwise present at next session)

## §5 — Pattern observations for future Class-4 work

**A3 R1 → R2 → A4 R1 first-try pattern**:
- A3 R1 Codex CHALLENGE: Q4 (`u as i64` wrap-negative when `u > i64::MAX`) + Q6 (prompt schema imprecise on rejection-class disambiguation)
- A3 R2 fixes: saturating cast `i64::try_from(u).unwrap_or(i64::MAX)` + precise schema doc distinguishing zero-stake vs over-balance rejection classes
- A4 implementation applied both R2 patterns **prophylactically** → R1 first-try BOTH PASS conviction high

**Lesson for future Class-4 atoms**: when introducing optional u64 fields cast to i64 in admission paths, use `i64::try_from(u).unwrap_or(i64::MAX)` saturating cast pattern from the start. When advertising new rejection classes in prompt schema, enumerate each class with explicit cause-effect mapping.

## §6 — Validation baseline at session #36 close

| Check | Value |
|-------|-------|
| HEAD (origin/main) | `db6fc3f` (pushed) |
| Constitution gates | 279 / 0 / 1 |
| Workspace tests | 1439 / 0 / 151 |
| Trust Root | PASS |
| `CONSTITUTION_EXECUTION_MATRIX.md` | 0 RED + 0 AMBER |
| `TRACE_FLOWCHART_MATRIX.md` | 0 RED + 0 AMBER + 37 GREEN + 3 N/A |
| FC1 / FC2 / FC3 | all GREEN; empirically verified 6/6 cells on A4 smoke |
| Architect ship-gate sets verified at HEAD | 9/10 (SG-B3.1-6 / M2 NOW ELIGIBLE post-Phase-2 ship) |

## §7 — Memory verification at next-session start

Verify these are present in `MEMORY.md`:
- `feedback_no_batch_class4_signoff` (used twice in session #36: A3 + A4)
- `feedback_step_b_protocol` (parallel-branch for both A3 + A4)
- `feedback_dual_audit` (Class-4 PRE-§8 timing; both atoms cleared)
- `feedback_dual_audit_conflict` (conservative-merge applied at A3 R1)
- `feedback_audit_loop_roi_flip` (A3 R2 OBS forward-bind rationale)
- `feedback_real_problems_not_designed` (strict-vs-weak witness honest disclosure for both atoms)
- `project_economy_prompt_landing_gap` (A5 forward atom addresses this)
- `OBS_TB_N1_A3_R2_I64_SATURATING_EDGE` (new session #36; saturating cast edge case)

If any are missing or stale, restore from session #36 commits.

## §8 — Empirical agent-uptake summary

Across 2 atoms (A3 + A4) × 6 cells each = **12 cells** of real-LLM smoke evidence:
- **0/12 cells** natively used new agent tools (`stake_micro` for A3; `verify_peer` for A4)
- Both DeepSeek-v4-flash and Qwen2.5-72B-Instruct emit `step` actions only
- Substrate-level mechanism landing is INDEPENDENT of agent-uptake-level work
- **OmegaAccepted reproducibility**: deepseek/aime_1983_p2 solved in 3 consecutive smokes (A3 pre-ship + A3 post-ship + A4 pre-§8), confirming substrate stability across all admission gate extensions

**Forward A5 (prompt economic feedback) is the natural next atom** to close the uptake gap via Class-2 prompt-training work.

---

## USER PROMPT (paste this into next Claude session)

```
Session #36 closed 2026-05-10 at HEAD `db6fc3f` on origin/main (5 commits
on top of session #35 close `e28b570`; pushed).

What landed this session:
- TB-N1-AGENT-ECONOMY Phase 2 SHIPPED FINAL: A3 + A4 both ratified
  with per-atom verbatim §8 sign-offs.
- A3 SHIPPED FINAL at HEAD dfc00e2 (sign-off
  2026-05-10_TB_N1_AGENT_ECONOMY_A3_§8_SIGN_OFF.md; user verbatim
  "好，确认可以 ship"). Agent-decided stake admission: typed_tx tail-
  append StakeBalanceExceeded; sequencer Step-4b; protocol stake_micro;
  evaluator 3 OMEGA callsites threading; 5 SG-N1-A3.* tests. PRE-§8
  dual audit R1 Codex CHALLENGE → R2 fix (saturating cast + precise
  schema) → user §8.3 Option A ship under R2 + OBS forward-bind
  (OBS_TB_N1_A3_R2_I64_SATURATING_EDGE).
- A4 SHIPPED FINAL at HEAD 98c1908 (sign-off
  2026-05-10_TB_N1_AGENT_ECONOMY_A4_§8_SIGN_OFF.md; user verbatim
  "好，确认可以 ship" — second canonical §8 form in session). Agent-
  callable verify-peer: typed_tx 3 NEW rejection classes
  (VerifyBondOutOfBounds + VerifyTargetNotAccepted + VerifyDuplicate);
  q_state AgentVerificationsIndex 15→16 sub-fields; sequencer
  Step-2.5 + Step-3 rename + Step-3.5 + Step-5b; protocol verify_peer
  fields; evaluator dispatch arm; 7 SG-N1-A4.* tests. PRE-§8 dual
  audit R1 first-try BOTH PASS (A3 R2 fixes applied prophylactically
  → first-try clean).
- Phase 2 forward §8 grant clause 2 "授权 A3 + A4 串行全授权" fully
  discharged. CLAUDE.md §13 stake/escrow/bond + Art. I.1.1 multi-
  agent verification agency layer closed at substrate level.

Constitution gates: 279 / 0 / 1 (was 267 at session #35; +12 across A3+A4)
Workspace tests:    1439 / 0 / 151 (was 1427; +12)
Trust Root: PASS (5 STEP_B files rehashed)
3-FC alignment: FC1 + FC2 + FC3 all 6/6 empirically verified on A4 smoke
Constitution + FC matrices: 0 RED + 0 AMBER preserved

Empirical agent-uptake gap (forward concern):
- 0/12 cells natively used new agent tools (stake_micro + verify_peer)
  across 2 atoms × 6 cells of real-LLM smoke
- Substrate-level mechanism INDEPENDENT of agent-uptake; A5 (prompt
  economic feedback Class-2) is the natural next atom to close gap
- OmegaAccepted reproducibility: deepseek/aime_1983_p2 solved in 3
  consecutive smokes; A4 substrate stability confirmed

Read first:
1. handover/ai-direct/NEXT_SESSION_PROMPT_2026-05-10_post_session_36.md
   (this prompt's source; full context + forward queue + atom options)
2. handover/ai-direct/LATEST.md "✅ Session #36 (continued)" block
3. handover/directives/2026-05-10_TB_N1_AGENT_ECONOMY_A4_§8_SIGN_OFF.md
4. handover/directives/2026-05-10_TB_N1_AGENT_ECONOMY_A4_§8_PACKET.md
   (§6 3-FC alignment + §7 strict-vs-weak witness analysis)
5. handover/directives/2026-05-10_TB_N1_AGENT_ECONOMY_A3_§8_SIGN_OFF.md
6. MEMORY.md (+OBS_TB_N1_A3_R2_I64_SATURATING_EDGE NEW; project_economy_
   prompt_landing_gap forward-relevant for A5)

Forward queue (Phase 2 freeze lifts post-A4 ship; NO active grant):

(a) A5 prompt economic feedback — Class-2; orthogonal to substrate;
    addresses A3/A4 uptake gap (0/12 cells empirically). RECOMMENDED.
(b) M2 100p batch (SG-B3.1-6 1800 invocations) — NOW ELIGIBLE per
    Phase 2 charter §4 freeze conditions met. Uses run_stage_b3.sh
    (CONDITION env override added at A4). 12-24 hr wall time; LLM budget.
(c) A6 Polymarket-agent-bridge — Class-4 STEP_B; Stage D-aligned;
    DEFERRED behind separate architect §8.
(d) Stage D real-world readiness — DEFERRED behind explicit architect
    §8.
(e) PromptCapsule evaluator wire-up (Class-3) — OPEN; not blocking.

Recommended next action: (a) A5 prompt economic feedback to close
the uptake gap empirically demonstrated this session. Per
feedback_constitutional_harness_engineering: harness → real run →
audit → ship. NOT charter → audit → atom (deprecated atomic-agentic
pattern).

If user wants to launch M2 batch (b) instead: run /runner-preflight
first + use scripts/run_stage_b3.sh production runner. Estimate
$50-100 LLM budget. Per feedback_smoke_before_batch: do a 6-cell smoke
first to verify pre-batch state.

Pattern observation for future Class-4 work: A3 R2 → A4 R1 first-try
demonstrated that R2 fixes from prior atom (saturating cast + precise
rejection-class schema doc) apply prophylactically to later atoms.
Future Class-4 atoms touching admission paths or new optional fields
should adopt these patterns from the start.
```

---

**End of session #36 close boot prompt.**
