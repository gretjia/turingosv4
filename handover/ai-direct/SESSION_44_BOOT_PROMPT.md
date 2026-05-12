# Session #44 boot prompt — TB-G G2 MarketDecisionTrace audit + NoTradeReason extension

Copy-paste this into the next session's first message.

---

TB-G G2 forward (Class-2 autonomous; charter §1 Module G2 + G-Phase directive §3 second-priority "Market Decision Observability — 与 G1 同时做"). Session #43 closed at `origin/main` HEAD `1ce877d` — **G2P module 🟢 LANDED** (G2P.1+G2P.2+G2P.3 SHIPPED; Codex G2 single-auditor PROCEED 11/12; +17 constitution gates 342→359; matrix §R G2P 🔴→🟢).

Read in this order before any action:

1. `CLAUDE.md`
2. `handover/ai-direct/LATEST.md` (session #43 block at top is the live context; sessions #42/#41/#40 below are audit trail)
3. `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md` §1 Module G2 (MarketDecisionTrace audit + NoTradeReason extension)
4. `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md` §3 (priority ranking — G2 is parallel with G1) + §5 Module G2 verbatim signatures
5. `handover/evidence/g_phase_g2p_2026-05-12T01-34-37Z/aggregate_verdict.json` + `audit_dashboard --run-report` §F output as the empirical baseline (current MarketDecisionTrace surface) — note current §F renders `total_traces: 0` because TB-N3 A2 invest dispatch never fired (no agent submitted an invest action; consistent with empty market)
6. `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` §R G2 row (currently 🔴 RED pending atom landing)

Forward atoms in order (G2 is canonical NEXT per charter §1 + directive §3):

**G2.1** — Class 2; `NoTradeReason` audit + 2 net-new variants:
- EDIT `src/runtime/market_decision_trace.rs:38` — extend `NoTradeReason` enum with `NoPerceivedEdge` + `PromptBudgetExceeded` (today: 11 variants per architect §5 spec; spec lists 13. Verify `AmountExceedsBalance` is `InsufficientBalance` doc-alias per existing source comment.)
- EDIT `src/runtime/adapter.rs:1315` (`tb_n3_invest_to_router_tx` dispatch) — map any new sequencer admission error to the right variant
- EDIT `experiments/minif2f_v4/src/bin/evaluator.rs:~3120` (TB-N3 A2 invest dispatch arm) — wire the 2 new variants into the trace-emit path
- NEW `tests/constitution_g2_no_trade_reason_taxonomy.rs` (SG-G2.1 source-grep exhaustive 13-variant + SG-G2.6 trace-or-tx invariant)
- Constitutional anchor: G-Phase directive §5 verbatim `enum NoTradeReason` 9-variant list + charter §1 Module G2 row

**G2.2** — Class 2; `audit_dashboard --run-report` §F NoTradeReason rows:
- EDIT `src/bin/audit_dashboard.rs::render_tb_n3_run_report` §F section — `total_traces` is already there; add per-`NoTradeReason` count rows + a `submitted_vs_traced_ratio` row (architect §5 SG-G2.3 "NoTradeReason appears in dashboard and CAS")
- The walker already exists at `src/bin/audit_dashboard.rs:2231` (CAS scan for `schema_version="tb_n3.market_decision_trace.v1"`); just extend the render
- NEW `tests/constitution_g2_dashboard_no_trade_rows.rs` (SG-G2.4 fixture renders per-variant counts)
- Constitutional anchor: charter §1 Module G2 atom 2 + G-Phase directive §5 SG-G2.3

**G2.3** — Class 2; Failed-invest L4.E binding test:
- NEW `tests/constitution_g2_failed_invest_l4e.rs` — fixture submits a `BuyWithCoinRouterTx` that the router rejects (e.g. balance shortfall or pool not active); assert the rejected tx lands in L4.E with matching `RejectionClass`; AND assert the `MarketDecisionTrace::no_trade` was written to CAS with the right variant
- Constitutional anchor: G-Phase directive §5 SG-G2.4 verbatim "Failed invest attempts enter L4.E"

**G2 real-LLM smoke** — autonomous after G2.1+G2.2+G2.3 ship pure-code:
- Re-run `bash scripts/run_g_phase_batch.sh g_phase_g2_<TS> full`
- Expected delta vs G2P R1 (`g_phase_g2p_2026-05-12T01-34-37Z`): §F `total_traces > 0` if any agent invokes the `invest` tool; per-variant breakdown rendered. The architect §8.6 "Failed invest 也算有意义 tape activity" outcome is now observable post-batch even when zero invests actually submitted (the trace-or-tx invariant binds market-bearing turns to either a tx OR a NoTradeReason — silent absence is forbidden)
- Codex G2 single-auditor full audit per session #43 cadence ("Gemini 总是 all pass — 意义不大")

Halt-and-re-charter Class 4 trigger (same as G2P per Option B+ ruling §Q4): if implementation forces touching sequencer admission semantics / TypedTx canonical schema / canonical signing payload / system tx authorization / EconomicState schema / HEAD_t definition / constitution.md text — HALT and draft separate §8 packet. G2 design as Class 2 should NOT hit this; `NoTradeReason` is an enum tail-append on a runtime/ module (not state/typed_tx.rs).

Memory triggers to honor (unchanged from session #43):
- Before any `bash run_*.sh` runner: `/runner-preflight` skill (7-stage)
- Before drafting new TB charter / picking next atom sequence beyond what's named here: `/constitution-landing-check` skill (this session is already inside an active TB-G module sequence — likely not needed)
- After TB ship final OR audit rounds > 3: `/harness-reflect` skill
- On any FC1/FC2/FC3 problem: trace BEFORE designing fix
- Before writing new `feedback_*.md`: ask "what mechanism enforces this?" — build mechanism, not just norm

Mode: continuing TB-G sequence under existing parent §8 sign-off (G1.1 packet §6 "好，确认可以 ship" 2026-05-11 + G-Phase directive autonomous-forward authorization). No new §8 packet required for G2 (Class 2). G3.2 + G4.2 remain the next Class-4 §8 packet boundaries (not this session).

Disk hygiene: G2 real-LLM smoke will produce ~11M evidence dir (matches G2P R1 size). Pre-launch `df -h /home/zephryj` and `cargo clean` if < 20G free. Note that target/ has grown across G1/G2P cycles; consider `cargo clean` proactively if the working tree is heavily exercised.

Trust-root rehash hygiene (session #43 lesson): if `experiments/minif2f_v4/src/bin/evaluator.rs`, `src/runtime/mod.rs`, or `src/bin/audit_dashboard.rs` are edited, the smoke will fail with `TRUST_ROOT_TAMPERED` until `genesis_payload.toml [trust_root]` is updated. Pre-launch check: `sha256sum src/runtime/mod.rs src/bin/audit_dashboard.rs experiments/minif2f_v4/src/bin/evaluator.rs` against the manifest entries to catch this before the smoke wastes a cargo build cycle.

Codex zombie hygiene (session #42-#43 lesson): if user reports "Agent Thread Limit is full", run `ps -eo pid,etime,cmd | grep -E "codex.*broker|codex app-server" | grep -v grep` and SIGKILL anything > 1 day old. DO NOT kill the current Claude session's broker or the Zed codex-acp.

Codex dispatch route preference (per `feedback_codex_bash_exec_direct_dispatch`):
- Try `Skill: codex:rescue` first.
- If internal error or ≥2 rejections: fall back to `nohup codex exec --dangerously-bypass-approvals-and-sandbox -C /home/zephryj/projects/turingosv4 < /tmp/prompt.md > /tmp/codex_out.log 2>&1 &` then watch for literal `^VERDICT: (PROCEED|CHALLENGE|VETO|HALT)\b` lines (NOT the bare `^VERDICT:` pattern — that matches the audit prompt's template placeholders and trips false-positive completion).

Carry-forward Open Questions from session #43:
1. **CpmmPool auto-emitted but no BuyWithCoinRouter swap** (sessions #42/#43 carry-forward) — same outcome on G2P smoke (cpmm_swap=0). G2.1 SHOULD surface this as a non-trivial `total_traces` count once `NoTradeReason` is wired; if §F still shows `total_traces=0` post-G2 ship, that itself is a finding (no agent ever invokes invest, even when the prompt menu enables it).
2. **13-agent persistence shape** (sessions #42/#43 carry-forward) — confirm preseed-12 + boltzmann-seeded solver intent.
3. **WalletBackend trait** (charter §0.66, sessions #41-#43 carry-forward) — §8 packet during G-Phase or after G7?
4. **PromptCapsule observability closure (Q1 CHALLENGE)** — `handover/alignment/OBS_G2P_PROMPT_BODY_OBSERVABILITY_2026-05-12.md` forward closure. Should this land as a sibling G2.4 / G2P.4 atom (Class 2-3 PromptCapsule swarm-write) BEFORE G2 smoke, or AFTER G2.1+G2.2+G2.3 ship as a standalone follow-up? My read: AFTER — Q1 was CHALLENGE not VETO, and the G2 §F dashboard render is the more urgent observability surface.
5. **G3.x scope coordination for Gap-A/B** (reputation accumulation + bond return per `OBS_G2P_VERIFY_PEER_REWARD_2026-05-12.md`) — bundle into G3.2 §8 packet or split into G3.5 atom?

Architect dispatch options (when user wants forward Class-4 work; NOT this session):
- G3.2 sequencer risk-cap admission §8 packet → closes 病灶1 bankruptcy-cycle + Gap-A/B
- G4.2 model-assignment genesis schema §8 packet → multi-LLM persistent identity
- TB-H WalletBackend trait §8 packet → real-funds bridge forward principle

---
**Session #43 ship summary** (already in LATEST.md):
- 8 commits 6e374f9 → 1ce877d (3 atom ships + 2 trust-root rehashes + audit prompt draft + audit prompt instantiate + session close)
- 17 new constitution gates (G2P.1: 6 + G2P.2: 8 + G2P.3: 3); 14 new lib unit tests
- Codex G2 single-auditor PROCEED conviction medium 11/12 PASS at HEAD `27b6c3c`
- G2P 9-task real-LLM smoke at `handover/evidence/g_phase_g2p_2026-05-12T01-34-37Z/` (3316s; PROCEED 40/0/0/11; n_witnessed=4 baseline-matched)
- Architect §8.2 ship-gate empirical: peer_verifications_total=0; §8.5 OR-branch satisfied via §F.X auto-rendered MECHANISM BOTTLENECK
- Matrix §R G2P 🔴 → 🟢; G2/G3/G4/G5/G6/G7 still 🔴
- 2 new OBS files filed (G2P_VERIFY_PEER_REWARD Gap-A/B + G2P_PROMPT_BODY_OBSERVABILITY Q1 CHALLENGE forward closure)
