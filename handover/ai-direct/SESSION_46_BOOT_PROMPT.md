# Session #46 boot prompt — TB-G G3.2 §8 packet draft + parallel G2P observability closure

Copy-paste this into the next session's first message.

---

TB-G forward (charter §1 Module G3 atom G3.2 "Solvency emitter + sequencer-side risk-cap admission" + Module G2P observability closure). Session #45 closed at `origin/main` HEAD `290113b` — **G3 observability layer 🟡 AMBER LANDED** (G3.1+G3.4+G3.3 SHIPPED; Codex G2 single-auditor PROCEED 12/12 conviction HIGH — best TB-G audit result so far; +26 constitution gates 376→402; matrix §R G3 🔴→🟡; architect §G3 SG-G3.5 empirically SATISFIED with 3/13 non-flat §G rows).

Read in this order before any action:

1. `CLAUDE.md`
2. `handover/ai-direct/LATEST.md` (session #45 block at top is the live context; sessions #44/#43/#42 below are audit trail)
3. `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md` §1 Module G3 row G3.2 (the Class-4 STEP_B atom) + Module G3 §3 "Architect §8 packets required (3 Class-4 atoms)"
4. `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md` §G3 verbatim SG-G3.2 / SG-G3.3 / SG-G3.4 module-level ship gates (untouched by session #45)
5. `handover/alignment/OBS_G2P_VERIFY_PEER_REWARD_2026-05-12.md` (Gap-A reputation accumulation + Gap-B bond return — both forward-bound to G3.2 Class-4 §8 packet per session #43 OBS framing)
6. `handover/alignment/OBS_G2P_PROMPT_BODY_OBSERVABILITY_2026-05-12.md` (Q1 CHALLENGE from G2P R1 — PromptCapsule swarm-write closure)
7. `handover/audits/CODEX_G2_TB_G_G3_VERDICT.md` (session #45 PROCEED 12/12 high; Codex's 3 non-blocking notes are baseline for G3.2 packet design — provenance / test-strength / multi-ref ChainTape)
8. `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` §R G3 row (now 🟡 AMBER pending G3.2; G4 / G5 / G6 / G7 still 🔴)

**Mode**: session #46 is primarily a **Class-4 §8 packet DRAFTING session**, NOT a Class-4 landing session. G3.2 modifies sequencer admission (Class-4 STEP_B; CLAUDE.md §12 — `src/state/sequencer.rs` is in the STEP_B file list) + tail-appends `BankruptcyRiskCapExceeded` `RejectionClass` (Class-4; `src/state/typed_tx.rs` schema bump). Both require explicit architect §8 sign-off BEFORE code lands. Session #46 produces the packet for architect review; landing happens in session #47+ AFTER ratification.

---

Forward atoms in order:

**G3.2 §8 packet** (Class-4 STEP_B; ARCHITECT §8 PACKET DRAFT — NO landing this session):

The packet must specify what session #47+ will land. Substantive content per charter §1 Module G3 atom G3.2 + G-Phase directive §G3 + OBS_G2P_VERIFY_PEER_REWARD:

- **Surface 1: Risk-cap admission** in `src/state/sequencer.rs` — 4 admission arms (`WorkTx` + `BuyWithCoinRouter` + `Challenge` + `Verify`) gain a `BANKRUPTCY_RISK_CAP_MICRO` precondition; agents whose `balance + recoverable_open_positions` falls below the cap cannot stake/pay/bond. Constant value is part of the §8 packet (recommend 100_000 μC = 10% of `Agent_0..9` preseed 1.0 Coin, matching G3.1's `SolvencyStatus::NearInsolvent` threshold so session #45's read-only classifier becomes the production admission key).
- **Surface 2: `BankruptcyRiskCapExceeded` `RejectionClass`** tail-appended in `src/state/typed_tx.rs` (Phase E.1 verbatim-binding gate enforced — sibling new variant pattern from Stage C P-M6 lesson). Display string < 64 bytes per architect SG-G3.12.
- **Surface 3: `AgentAutopsyCapsule` emit** at the problem-end boundary when an agent crosses into `Bankrupt` solvency — reuse the existing `src/runtime/autopsy_capsule.rs::AgentAutopsyCapsule::new` (no schema bump needed; just a new emit site).
- **Surface 4 (OPTIONAL — bundle decision pending architect adjudication; surface explicitly in §3.Q.bundle of the packet)**: close OBS_G2P_VERIFY_PEER_REWARD Gap-A (reputation accumulation on VerifyTx) + Gap-B (bond return at run-resolve) as G3.2 sub-atoms (they touch the same sequencer admission surfaces). Session #44/#45 OBS framing was "forward-bound to G3.2 Class-4 §8". The packet should propose either (A) bundle Gap-A/B into G3.2 + single §8 review OR (B) split into a sibling G3.5 atom + separate §8 packet. Recommend (A) per `feedback_no_batch_class4_signoff` semantics — Gap-A/B and risk-cap admission are the SAME sequencer surface; the no-batch rule prohibits batching DISTINCT Class-4 atoms across §8 packets, NOT bundling sub-surfaces of one Class-4 atom into one §8 packet.

Packet structure (mirror `handover/directives/2026-05-11_TB_G_G1_1_§8_PACKET.md` shape per charter §3):

- §1 Constitutional anchor — link CLAUDE.md §12 STEP_B + §13 economy laws + §14 predicate boundary + charter §1 Module G3 atom G3.2 + G-Phase directive §G3 SG-G3.2/SG-G3.3/SG-G3.4
- §2 Adjacent-surface inventory — every file the change touches + every Class-4 boundary it crosses
- §3 Open questions Q1..Qn for architect — including the Gap-A/B bundle adjudication (Surface 4 above)
- §4 Proposed kill-conditions / fail-on-fix scaffolds (what tests must exist BEFORE the §8 packet ships)
- §5 Trust-root rehash plan (which files will rehash post-edit)
- §6 Ship-condition for architect §8 sign-off (verbatim format; canonical Class-4 §8 forms per CLAUDE.md §10 multi-clause analysis)
- §7 Rollback plan if the §8 sign-off is later withdrawn (Stage C P-M2..P-M9 rollback precedent)

Forward atom name for the packet file: `handover/directives/2026-05-12_TB_G_G3_2_§8_PACKET.md`.

After packet draft + commit, HALT and request architect review via the standard `/architect-ingest` skill route. Do not modify `src/state/sequencer.rs` or `src/state/typed_tx.rs` this session.

**G2P observability closure** (Class 2-3 PARALLEL track — can ship autonomously while G3.2 packet awaits architect; closes G2P R1 Q1 CHALLENGE):

Per `handover/alignment/OBS_G2P_PROMPT_BODY_OBSERVABILITY_2026-05-12.md`:

- Write a `PromptCapsule` CAS object per LLM call in the evaluator swarm path. Schema already exists at `src/runtime/prompt_capsule.rs` (10KB module landed in prior TB).
- Per CLAUDE.md §4.3 the canonical default is **Class-3 PromptCapsule + L4 anchor** (`PromptCapsule -> CAS; AttemptTelemetry / WorkTx references prompt_capsule_cid; L4/L4.E anchor references the attempt`).
- This is the closure path for session #43 G2P Q1 CHALLENGE ("production evidence bundle does not capture prompt bodies → cannot empirically prove `=== Pending Peer Reviews ===` reached the LLM"). Same closure also benefits future G3.3 audits (proving `=== Your Position ===` reached the LLM).
- Class check: Class-3 because PromptCapsule is `production evidence` per CLAUDE.md §9; per `feedback_class4_cannot_hide_in_class3` confirm no sequencer admission / typed_tx schema / canonical signing payload touch.
- If user adjudicates the closure needs its own per-atom §8, HALT and re-charter. Otherwise ship under parent §8 G-Phase autonomous-forward authorization (precedent: G2P.1 / G3.3 Class-3 prompt-block shipped under same).

Forward atom name: `G2P.4` (sibling to G2P.1/.2/.3).

**G4.2 §8 packet** (Class-4 STEP_B; OPTIONAL parallel draft):

If G3.2 packet drafting finishes early in the session, optionally draft the G4.2 `[agent_model_assignment]` genesis schema §8 packet at `handover/directives/2026-05-12_TB_G_G4_2_§8_PACKET.md`. Sister Class-4 atom per charter §3; same template shape as G3.2. Closes module-level architect §G4 SG-G4.3/SG-G4.4 (model-identity persistence across tasks).

Do NOT batch G3.2 + G4.2 into one §8 packet — per `feedback_no_batch_class4_signoff`, each Class-4 atom requires its own per-atom §8 review.

---

Halt-and-re-charter Class-4 triggers (NEW for session #46):

- If the G3.2 packet design needs to extend `EconomicState` schema (e.g. new sub-field for tracking risk-cap-violation count) — HALT and reframe as TWO §8 packets (one for schema, one for admission); CLAUDE.md §9 Class-4 file list includes `src/state/typed_tx.rs` BUT `EconomicState` schema changes have historically been bundled with sequencer changes (TB-13 Atom 2 precedent). Surface to user.
- If `BANKRUPTCY_RISK_CAP_MICRO` constant value should be per-agent-keyed (not global) — HALT and ratify with architect; introduces complexity.
- If Gap-A reputation accumulation needs to mutate state OUTSIDE the sequencer admission arms (e.g. via a system-emitted tx at run-resolve) — HALT; system-tx authorization is its own architect boundary (CLAUDE.md §13 "system tx cannot be agent-submitted").

Memory triggers to honor (unchanged from session #45):
- Before drafting new TB charter / picking next atom sequence beyond what's named here: `/constitution-landing-check` skill — this session is in `Constitutional Harness Engineering` mode, not Atomic Agentic Engineering
- Before `bash run_*.sh` runner: `/runner-preflight` skill (no smoke planned this session — packet drafting only)
- After audit rounds > 3: `/harness-reflect` skill
- On any FC1/FC2/FC3 problem: trace BEFORE designing fix
- Before writing new `feedback_*.md`: ask "what mechanism enforces this?" — build mechanism, not just norm
- **NEW for §8 packet drafting**: every adjacent-surface row in packet §2 MUST cite an FC node (FC1/FC2/FC3) and a CLAUDE.md section; orphan rows = re-draft

---

Carry-forward Open Questions from session #45:

1. **Matrix G3 framing** — settled to AMBER session #45; user override to GREEN remains 1 edit away. Reconfirm with user at session #46 open if their boot-prompt directive intent was 🟢 not 🟡.
2. **`total_traces=0` empirical pattern** across G2P R1 + G2 R1 + G3 R1 (now 3 batches) — G5.1 opportunity scheduler + 7-action menu is the canonical forward fix. NOT this session — G5.1 is Class 3 and depends on `agent_scheduler.rs` design which forks the round-robin path. After G3.2 + G4.2 ship.
3. **G3.3 Class-3 retro ratification** — Codex Q9 PASS confirms structure (no Class-4 vectors); no retro §8 needed. Surface for record at session #46 open if user prefers explicit acknowledgement.
4. **WalletBackend trait** (charter §0.66, sessions #41-#45 carry-forward) — §8 packet during G-Phase or after G7? G3.2 admission reads `balances_t` directly today; future on-chain wallet swap touches the same surface, so this question gets more urgent once G3.2 lands.
5. **Cost basis tracking on conditional shares** (NEW — surfaced during G3.1 design) — current `unrealized_pnl` semantics use cost basis = 1 μC / share-pair (mint cash flow). For BuyWithCoinRouter-derived asymmetric positions, the real cost basis differs from the mint convention. If architect wants strict cost-basis tracking, that's an additional `EconomicState.cost_basis_t` sub-field — Class-4 schema change. Surface in G3.2 packet §3 if relevant; otherwise carry forward to a later atom.

---

Architect dispatch options (after packets drafted; user-initiated):

- G3.2 §8 packet (drafted this session) → architect §8 review → architect verdict (PROCEED / VETO / CHALLENGE) → session #47 lands G3.2 if PROCEED
- G4.2 §8 packet (drafted this session if time) → independent architect §8 review
- G2P.4 PromptCapsule swarm-write (Class 2-3 autonomous; ships under parent §8) → can ship same session as packet drafting if time permits

---

Disk hygiene: no smoke planned this session, but `cargo check` + `cargo test --lib` for any Class-3 / G2P.4 work will use ~5G under `target/`. Pre-action `df -h /home/zephryj` if < 18G free → `cargo clean` first.

Trust-root rehash hygiene: G3.2 + G4.2 packet files are pure markdown drafts — no Trust Root rehash needed for packet commits. If G2P.4 lands and touches `experiments/minif2f_v4/src/bin/evaluator.rs` (swarm path) or `src/runtime/mod.rs` (module registration for PromptCapsule helper), rehash per the G2/G3 collapsed-rehash pattern (bundle Trust Root update into the atom commit).

Codex zombie hygiene: if user reports "Agent Thread Limit is full", run `ps -eo pid,etime,cmd | grep -E "codex.*broker|codex app-server" | grep -v grep` and SIGKILL anything > 1 day old. DO NOT kill the current Claude session's broker or the Zed codex-acp.

Codex dispatch route preference (unchanged from session #45 — confirmed by 17-min wall G3 audit):
- Try `Skill: codex:rescue` first.
- If internal error or ≥2 rejections (or user mid-session pain signal): fall back to `nohup codex exec --dangerously-bypass-approvals-and-sandbox -C /home/zephryj/projects/turingosv4 < /tmp/prompt.md > /tmp/codex_out.log 2>&1 &`.
- **Monitor pattern**: `until [ -s handover/audits/CODEX_<TB>_VERDICT.md ] || ! ps -p <codex-pid> >/dev/null 2>&1; do sleep 15; done` (`feedback_monitor_codex_verdict_safer_signal` — process-exit OR verdict-file-written; not log grep).
- Session #45 lesson: Codex GPT-5 can pause ~5-6 min between the "I'm going to overwrite the artifacts" message and the actual verdict file write — this is normal LLM-thinking pause, NOT a hang. Total audit wall was 17 min for G3 vs 10 min for G2. Don't intervene before 25 min elapsed.

---

**Session #45 ship summary** (already in LATEST.md):
- 5 commits 97e6527 → 290113b (G3.1 / G3.4 / G3.3 / audit prompt / session-close)
- 26 new constitution gates 376 → 402 (G3.1: 12 + G3.4: 6 + G3.3: 8); +6 your_position + 10 agent_pnl + 16 lib unit = +32 lib tests
- Codex G2 single-auditor PROCEED conviction HIGH 12/12 PASS at HEAD `9fde94d` (best TB-G audit result; G2 R1 was 12/12 medium, G2P R1 was 11/12 medium)
- G3 9-task real-LLM smoke at `handover/evidence/g_phase_g3_2026-05-12T11-02-27Z/` (3088s; PROCEED 40/0/0/11; n_witnessed=4 baseline-matched)
- Architect §G3 SG-G3.5 ship-gate empirical: §G PnL trajectory rendered with **3/13 NON-FLAT rows** (`tb7-7-sponsor` escrow -100k / `Agent_0` stake+claim positions=2 / `MarketMakerBudget` collateral positions=1) — better outcome than G2 R1 / G2P R1 all-zero shape; silent-zero MECHANISM BOTTLENECK correctly ABSENT
- Matrix §R G3 🔴 → 🟡 AMBER (G3 OBSERVABILITY LAYER LANDED; G3.2 admission layer still pending §8 packet — strict reading per `feedback_no_workarounds_strict_constitution`)
- 3 Codex non-blocking notes carried forward as G3.2 packet inputs: (1) provenance / dirty-worktree pattern; (2) test-strength gap (SG-G3.8.b doesn't assert exact cause strings); (3) multi-ref ChainTape (refs/chaintape/l4 + refs/transitions/main both match manifest)
