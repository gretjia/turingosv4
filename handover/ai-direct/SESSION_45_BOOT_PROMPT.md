# Session #45 boot prompt — TB-G G3 Persistent PnL / Solvency module

Copy-paste this into the next session's first message.

---

TB-G G3 forward (charter §1 Module G3 "Persistent PnL / Solvency / Bankruptcy — Drucker lens"). Session #44 closed at `origin/main` HEAD `04c4b62` — **G2 module 🟢 LANDED** (G2.1+G2.2+G2.3 SHIPPED; Codex G2 single-auditor PROCEED 12/12; +17 constitution gates 359→376; matrix §R G2 🔴→🟢).

Read in this order before any action:

1. `CLAUDE.md`
2. `handover/ai-direct/LATEST.md` (session #44 block at top is the live context; sessions #43/#42/#41 below are audit trail)
3. `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md` §1 Module G3 (rows G3.1..G3.4)
4. `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md` §G3 verbatim `AgentMarketState` shape + SG-G3.1..SG-G3.5
5. `handover/evidence/g_phase_g2_2026-05-12T07-48-28Z/aggregate_verdict.json` + `audit_dashboard --run-report` §F + §F.A output as the empirical baseline (current MarketDecisionTrace + NoTradeReason surface) — note current §F.A renders all 13 variants at zero because the trace-or-tx wire is defensively correct but had no opportunity to fire in the G2 R1 batch (Same Open Question as G2P R1's `peer_verifications_total=0`; G5.1 opportunity scheduler is the canonical forward fix)
6. `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` §R G3 row (currently 🔴 RED pending atom landing)

Forward atoms in order (G3 is canonical NEXT per matrix §R + charter §1):

**G3.1** — Class 2; `compute_agent_pnl` derived view + 7-field `AgentMarketStateView`:
- NEW `src/runtime/agent_pnl.rs` — derived-view walker that reads canonical `EconomicState` (balances_t / stakes_t / claims_t / reputations_t / conditional_share_balances_t / cpmm_pools_t) + per-agent open positions + realized/unrealized PnL + solvency_status + reputation_score. Pure derivation; no state mutation.
- Architect §G3 verbatim 7-field shape: `{ agent_id, balance, open_positions, realized_pnl, unrealized_pnl, solvency_status, reputation_score }`. Use integer-rational math throughout (no f64; CLAUDE.md §13).
- NEW `tests/constitution_g3_pnl.rs` (SG-G3.1..G3.3 + SG-G3.9): genesis returns zero-pnl / post-BuyRouter cash drops + unrealized updates / 5 scenarios covered / 7 fields source-grep.
- Constitutional anchor: G-Phase directive §G3 SG-G3.1 "agent balance changes persist across tasks" + SG-G3.5 "PnL is visible in dashboard as materialized view".

**G3.4** — Class 2; §G PnL trajectory report + G1 SG-G1.7 dual-bind:
- EDIT `src/bin/audit_dashboard.rs::render_tb_n3_run_report` — add `## §G PnL trajectory` section between §F.X (peer-verify coverage) and §G (price-is-signal banner). Iterate `agent_pnl::compute_agent_pnl(q, agent_id)` for each agent_id in the canonical agent registry; render per-agent realized/unrealized rows (integer-rational). Silent-zero-forbidden contract per G2P.2 pattern: if all PnL is zero, render `MECHANISM BOTTLENECK` block (no router buys → no PnL movement).
- NEW `tests/constitution_g3_pnl_trajectory_evidence_binding.rs` — dual-binding test reads evidence dir, walks per-agent PnL trajectory across the persistent batch, asserts ≥1 non-flat row OR mechanism-bottleneck row.
- Constitutional anchor: charter §1 Module G3 atom G3.4 + G-Phase directive §G3 SG-G3.5 "PnL is visible in dashboard as materialized view".

**G3.3** — Class 3; `=== Your Position ===` per-viewer prompt block (Drucker framing):
- NEW `src/sdk/your_position.rs` per-viewer renderer (mirrors G2P.1 `pending_peer_reviews.rs` + N1 A2 `econ_position.rs` patterns). Renders the agent's own 7-field AgentMarketStateView with Drucker verbatim framing string ("Drucker: 'What gets measured gets managed' — your position drives your next decision").
- EDIT `experiments/minif2f_v4/src/bin/evaluator.rs:~2188` (`build_agent_prompt` call site) — add 10th param `your_position: &str` rendered from `agent_pnl::compute_agent_pnl` + format.
- EDIT `src/sdk/prompt.rs` — `build_agent_prompt` signature gains 10th param; canonical `=== Your Position ===` heading.
- NEW `tests/constitution_g3_your_position_prompt.rs` (SG-G3.6 + SG-G3.7 + SG-G3.13): per-viewer source-grep / non-default render witnessed / Drucker verbatim framing present / no other-agent PnL leak.
- Constitutional anchor: charter §1 Module G3 atom G3.3 + G-Phase directive §G3 architect verbatim Drucker framing.
- **Class 3 scope check**: G3.3 is Class 3 (prompt-block + signature bump to `build_agent_prompt` — wire-up via existing parent §8). NO sequencer admission change; NO typed_tx schema change; NO canonical signing payload touch. Compare to G2P.1 (Class 2) and N1 A2 (Class 2): the param-bump is the only Class-up vector. Confirm parent §8 G-Phase directive autonomous-forward authorization covers this; if user adjudicates Class 3 needs its own per-atom §8 packet, HALT and re-charter (per `feedback_no_batch_class4_signoff`).

**G3 real-LLM smoke** — autonomous after G3.1+G3.3+G3.4 ship pure-code:
- Re-run `bash scripts/run_g_phase_batch.sh g_phase_g3_<TS> full`
- Expected delta vs G2 R1 (`g_phase_g2_2026-05-12T07-48-28Z`): §G PnL trajectory rendered (will show all zeros if no router buys happen — silent-zero bottleneck row triggers). The G2.1 wire + G3.x render together close the per-agent observability surface even when economic activity is dormant.
- Codex G2 single-auditor full audit per session #43/#44 cadence ("Gemini 总是 all pass — 意义不大")

Halt-and-re-charter Class 4 trigger (same as G2 + G2P per Option B+ ruling §Q4): if implementation forces touching sequencer admission semantics / TypedTx canonical schema / canonical signing payload / system tx authorization / EconomicState schema / HEAD_t definition / constitution.md text — HALT and draft separate §8 packet. G3.1 + G3.4 design as Class 2 should NOT hit this; G3.3 prompt-block + signature bump as Class 3 should NOT hit this either. **G3.2 (solvency emitter + sequencer-side risk-cap admission)** is the next Class-4 §8 packet boundary AFTER G3.1+G3.3+G3.4 ship — do NOT attempt G3.2 in this session; queue it for a future per-atom §8 packet draft.

Memory triggers to honor (unchanged from session #44):
- Before any `bash run_*.sh` runner: `/runner-preflight` skill (7-stage; session #44 caught stale binaries before launch)
- Before drafting new TB charter / picking next atom sequence beyond what's named here: `/constitution-landing-check` skill (this session is already inside an active TB-G module sequence — likely not needed unless user asks for G5+ jump)
- After TB ship final OR audit rounds > 3: `/harness-reflect` skill
- On any FC1/FC2/FC3 problem: trace BEFORE designing fix
- Before writing new `feedback_*.md`: ask "what mechanism enforces this?" — build mechanism, not just norm

Mode: continuing TB-G sequence under existing parent §8 sign-off (G1.1 packet §6 "好，确认可以 ship" 2026-05-11 + G-Phase directive autonomous-forward authorization). No new §8 packet required for G3.1 + G3.4 (Class 2). G3.3 is Class 3 — confirm parent §8 forward authorization covers prompt-block + signature bump; if user adjudicates per-atom §8 needed, HALT. G3.2 + G4.2 remain the next Class-4 §8 packet boundaries (NOT this session).

Disk hygiene: G3 real-LLM smoke will produce ~9-11M evidence dir (matches G2 R1 size). Pre-launch `df -h /home/zephryj` and `cargo clean` if < 20G free. Session #44 ran with 20G free at launch (tight but successful); session #45 should clean `target/` proactively if working tree has been heavily exercised.

Trust-root rehash hygiene (session #43/#44 lesson): if `experiments/minif2f_v4/src/bin/evaluator.rs`, `src/runtime/mod.rs`, `src/bin/audit_dashboard.rs`, or `src/sdk/prompt.rs` are edited, the smoke will fail with `TRUST_ROOT_TAMPERED` until `genesis_payload.toml [trust_root]` is updated. Pre-launch check: `sha256sum src/runtime/mod.rs src/bin/audit_dashboard.rs experiments/minif2f_v4/src/bin/evaluator.rs src/sdk/prompt.rs` against the manifest entries to catch this before the smoke wastes a cargo build cycle. G2 session collapsed rehashes into atom commits for tighter atomicity (vs G2P's split-rehash pattern); follow the G2 pattern.

Codex zombie hygiene (sessions #42-#44 lesson): if user reports "Agent Thread Limit is full", run `ps -eo pid,etime,cmd | grep -E "codex.*broker|codex app-server" | grep -v grep` and SIGKILL anything > 1 day old. DO NOT kill the current Claude session's broker or the Zed codex-acp.

Codex dispatch route preference (per `feedback_codex_bash_exec_direct_dispatch` + session #44 reaffirmation):
- Try `Skill: codex:rescue` first.
- If internal error or ≥2 rejections (or user mid-session pain signal like "时间又很久了"): fall back to `nohup codex exec --dangerously-bypass-approvals-and-sandbox -C /home/zephryj/projects/turingosv4 < /tmp/prompt.md > /tmp/codex_out.log 2>&1 &`.
- **Monitor pattern lesson (session #44; new `feedback_monitor_codex_verdict_safer_signal`)**: do NOT poll the Codex log for `^VERDICT: (PROCEED|CHALLENGE|VETO|HALT)\b` — Codex `cat`s predecessor verdict files mid-audit and trips false-positive completion (G2 audit fired at log line 6590 which was a `cat G2P_VERDICT.md` echo). Use the safer signal: `until [ -s handover/audits/CODEX_<TB>_VERDICT.md ] || ! ps -p <codex-pid> >/dev/null 2>&1; do sleep 15; done`. Process-exit OR verdict-file-written is unforgeable.

Carry-forward Open Questions from session #44:
1. **`total_traces=0` empirical pattern across G2P R1 + G2 R1** (now 2 batches, same shape) — G5.1 opportunity scheduler + 7-action menu is the canonical forward fix (also closes G2P's `non_solver_verifications=0`). NOT this session — G5.1 is Class 3 and depends on `agent_scheduler.rs` design which forks the round-robin path. Expect to see another batch of zero rows in G3 R1; G3.4 §G silent-zero bottleneck row should render explicit explanation.
2. **13-agent persistence shape** (sessions #42/#43/#44 carry-forward) — confirm preseed-12 + boltzmann-seeded solver intent. G3.1 PnL walker iterates the agent registry; spot-verify the registry shape at G3.1 ship.
3. **WalletBackend trait** (charter §0.66, sessions #41-#44 carry-forward) — §8 packet during G-Phase or after G7? G3.1 reads `balances_t` directly today; future on-chain wallet swap would touch the same surface, so this question gets more urgent once G3.2 Class-4 lands.
4. **PromptCapsule observability closure (session #43 Q1 CHALLENGE)** — `handover/alignment/OBS_G2P_PROMPT_BODY_OBSERVABILITY_2026-05-12.md` forward closure. AFTER G3.x or sibling G2P.4? Same answer as session #43/#44: AFTER (Class 2-3 standalone; not urgent).
5. **G3.x scope coordination for Gap-A/B** (reputation accumulation + bond return per `OBS_G2P_VERIFY_PEER_REWARD_2026-05-12.md`) — bundle into G3.2 §8 packet or split into G3.5 atom? G3.2 is Class-4; the architect §8 packet for G3.2 is the natural binding point. Surface this for user adjudication at G3.2 §8 packet draft time.

Architect dispatch options (when user wants forward Class-4 work; NOT this session):
- G3.2 sequencer risk-cap admission §8 packet → closes 病灶1 bankruptcy-cycle + Gap-A/B + adds `BankruptcyRiskCapExceeded` RejectionClass tail-append
- G4.2 model-assignment genesis schema §8 packet → multi-LLM persistent identity
- TB-H WalletBackend trait §8 packet → real-funds bridge forward principle

---
**Session #44 ship summary** (already in LATEST.md):
- 4 commits f22140a → 04c4b62 (3 atom ships + session-close)
- 17 new constitution gates (G2.1: 8 + G2.2: 5 + G2.3: 4); 6 new lib unit tests + 17 gate-binding tests = +23
- Codex G2 single-auditor PROCEED conviction medium 12/12 PASS at HEAD `297042c` (cleaner than G2P R1's 11/12)
- G2 9-task real-LLM smoke at `handover/evidence/g_phase_g2_2026-05-12T07-48-28Z/` (3996s; PROCEED 40/0/0/11; n_witnessed=4 baseline-matched)
- Architect §G2 SG-G2.3 ship-gate empirical: dashboard §F.A 13-row stable block CONFIRMED rendered; CAS-side total_traces=0 (architect §8.5 OR-branch satisfied; same shape as G2P R1)
- Matrix §R G2 🔴 → 🟢; G3/G4/G5/G6/G7 still 🔴
- 2 Codex non-blocking notes: (1) provenance gap closed via push; (2) test-doc drift in SG-G2.5.d docstring vs body (test-scaffold edge)
- 1 new feedback memory: `feedback_monitor_codex_verdict_safer_signal` (process-exit OR verdict-file-written; not log grep)
