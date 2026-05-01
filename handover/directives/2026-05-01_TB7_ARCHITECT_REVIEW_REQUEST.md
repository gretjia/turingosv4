# TB-7 Architect Review Request — Per-LLM-Proposal WorkTx Routing (Frame B chaintape closure)

**Date**: 2026-05-01
**Branch state**: `main` @ `17c5e73` (TB-6 SHIPPED, all 8 atoms shipped 2026-05-01).
**Author**: Claude (post-TB-6 ship; user-authorized after dialogue on chaintape final-form distance).
**Predecessor**: `handover/directives/2026-05-01_TB6_ARCHITECT_RULING.md` (D1-D7 binding decisions for TB-6).

---

## §0 Headline ask

Architect, please rule on **5 binding decision items D1-D5** to authorize TB-7 charter at `handover/tracer_bullets/TB-7_charter_draft_2026-05-01.md`. The charter is currently DRAFT; no production code touched until you ratify.

**TL;DR proposal**: TB-7 = **Frame B** (every LLM proposal traverses `bus.submit_typed_tx → Sequencer::apply_one → on-disk ChainTape`), narrowed to NOT include FinalizeRewardTx (RSP-4 = TB-11) and NOT include SlashTx (RSP-3.2 = TB-9). 7 atoms; production wire-up class; STEP_B not triggered in current scope; 72h-to-feedback-loop discharge gate.

---

## §1 Files to read (binding-priority order)

Read these in this order before ruling:

| Priority | Path | Purpose |
|---|---|---|
| 1 | `handover/tracer_bullets/TB-7_charter_draft_2026-05-01.md` | The charter draft itself (this is what you're ratifying) |
| 2 | `handover/audits/RECURSIVE_AUDIT_TB_6_2026-05-01.md` | TB-6 ship audit — proves Frame A closure; informs Frame B scope |
| 3 | `handover/directives/2026-05-01_TB6_ARCHITECT_RULING.md` | Your prior ruling D1-D7 — TB-7 inherits all 7 decisions |
| 4 | `handover/tracer_bullets/TB-6_charter_2026-05-01.md` | TB-6 charter shape — TB-7 mirrors |
| 5 | `handover/evidence/tb_6_chaintape_smoke_2026-05-01/replay_report.json` | Frame A status: 7/7 indicators green on synthetic seed |
| 6 | `handover/evidence/tb_6_chaintape_smoke_2026-05-01/run_summary.json` | Frame A's run_summary; shows N=2 (synthetic) NOT N=20 (real LLM) |
| 7 | `handover/evidence/tb_6_chaintape_smoke_2026-05-01/agent_audit_trail.jsonl` | Atom 5 surface — synthetic-pair audit records; per-LLM-proposal wiring is what TB-7 closes |
| 8 | `experiments/minif2f_v4/src/bin/evaluator.rs:1190-1280` | Current LLM "append" branch — TB-7 Atom 2 wires `bus.submit_typed_tx` here |
| 9 | `experiments/minif2f_v4/src/bin/evaluator.rs:1670-1700` | Current OMEGA "complete" branch — TB-7 Atom 3 wires here |
| 10 | `src/runtime/agent_audit_trail.rs` | TB-6 Atom 5 module — TB-7's per-proposal wiring writes records using this surface |
| 11 | `src/runtime/run_summary.rs` | TB-6 Atom 6 module — TB-7's chain-derived PPUT (Atom 5) extends this pattern |
| 12 | `src/state/typed_tx.rs:WorkTx` (~line 800) | WorkTx struct — TB-7 wires real Ed25519 signatures here (currently zero bytes in synthetic adapter) |
| 13 | `src/bottom_white/ledger/system_keypair.rs` | TB-7 Atom 1 mirrors this shape for per-agent keypairs |
| 14 | `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` § 11.5 | Updated TB sequence; TB-7 placement |
| 15 | `CLAUDE.md` | STEP_B-protected file list; TB-7 must not touch |

Optional reading for additional context:
- `handover/audits/SELF_AUDIT_TB_5_SMOKE_TAPE_2026-05-01.md` (chaintape gap discovery — informed TB-6 sequencing)
- `handover/audits/STAGE_AUDIT_TB_1_TO_TB_5_2026-05-01.md` (cumulative debt picture)

---

## §2 Decision items — 5 binding rulings requested

### D1 — TB-7 sequencing — Frame B (recommended) vs RSP-M0/M1 vs RSP-3.2 Slash

Three sequencing options for TB-7:

**Option A (RECOMMENDED)**: Frame B = per-LLM-proposal WorkTx routing. Closes the structural surface that TB-6 Atom 5 explicitly deferred. Rationale:
- TB-6 ship evidence shows the chain has 2 entries (synthetic seed) while real LLM activity (~20 proposals on `mathd_algebra_107`) goes through legacy `bus.append`. The user can't yet audit "what the agent actually did" from the chain.
- PputResult is in-memory accumulator, not chain-derived. This is a hidden trust dependency: anyone reading the smoke evidence still trusts the evaluator's stdout to compute capability.
- Frame B is the prerequisite for any chain-derived capability metric (PPUT, h_vppu, capability compilation loop). Without Frame B, all P6 capability work remains evaluator-attested.

**Option B**: RSP-M0/M1 NodePosition derived index. Per ruling 2026-05-01 § 4.5 reserved-future. Activates "WorkTx.stake = first-long exposure" semantically without market mechanics.

**Option C**: RSP-3.2 Slash execution. Per ruling 2026-05-01 § 4.5 sequence position is TB-9. Closes UpheldDeferred → punishment loop.

**Stake**: Option A closes the user-observable "chain shows 2 entries while LLM ran 20 proposals" gap. Options B + C add economic surface without closing the LLM-activity-on-chain gap. Option A is the natural successor to TB-6.

**Recommended ruling**: **Option A — Frame B**. Codify in TB-7 charter § 0 + § 1.

---

### D2 — Agent keypair lifecycle — runtime-generated (recommended) vs persistent

Two options for per-agent Ed25519 keypair lifecycle:

**Option A (RECOMMENDED)**: Runtime-generated per evaluator-run. Each `agent_id` gets a fresh keypair at evaluator startup. Public key persisted to `<runtime_repo>/agent_pubkeys.json`. Private key in process memory; dropped at evaluator exit. Mirrors TB-5's PinnedSystemPubkeys shape.

**Option B**: Persistent across runs. Private keys stored encrypted in a keystore (`agent_keystore.encrypted` similar to system_keypair v0.5+). Allows agent identity continuity across problems for cross-problem reputation.

**Stake**: Option A is simpler and structurally proven (TB-5 system_keypair pattern). Option B requires encryption-at-rest infrastructure (already exists for system; would need agent-side variant). Option B unlocks cross-problem reputation but is OUT OF SCOPE for Frame B (reputation is RSP-4 / TB-11 territory).

**Recommended ruling**: **Option A — runtime-generated per-run**. Codify in TB-7 charter § 4.2.

---

### D3 — OMEGA-accept path scope — narrowed (recommended) vs full settlement loop

**Option A (RECOMMENDED — NARROWED)**: OMEGA accept → `WorkTx + VerifyTx` pair on chain. ChallengeWindow stays OPEN at TB-7 ship. NO FinalizeRewardTx (RSP-4 = TB-11). NO SlashTx (RSP-3.2 = TB-9).

**Option B (FULL)**: OMEGA accept → `WorkTx + VerifyTx + FinalizeRewardTx` with stub reward distribution. Closes the loop but introduces RSP-4-territory mechanics that the architect ruling § 4.5 sequenced as TB-11.

**Stake**: Option A keeps TB-7 within Frame B (proposal + verify on chain; no settlement). Option B partially activates Frame C economic loop, expanding scope. Architect ruling § 4.5 placed RSP-4 SettlementEngine at TB-11 explicitly.

**Recommended ruling**: **Option A — narrowed**. Codify in TB-7 charter § 4.3 + § 6 #21,22.

---

### D4 — Chain-derived PPUT tolerance — bit-exact-on-time-insensitive (recommended) vs full numeric tolerance

The Atom 5 deliverable is `runtime::chain_derived_pput::compute_pput_from_chain(...) -> PputResult`. The chain-derived value must match the in-memory accumulator's PputResult, but on which fields and with what tolerance?

**Option A (RECOMMENDED — BIT-EXACT ON TIME-INSENSITIVE)**: Bit-exact equality on `solved`, `verified`, `tx_count`, `proposal_count`, `golden_path_token_count`, `gp_payload`, `gp_path`, `gp_proof_file`, `tactic_diversity`, `tool_dist`, `failed_branch_count`. Time-sensitive fields (`total_wall_time_ms`, `verifier_wait_ms`, `pput_runtime`, `pput_verified`, `pput_m_verified`, `h_vppu`) excluded — chain replay is byte-deterministic but wall time is not.

**Option B (NUMERIC TOLERANCE)**: All fields must match within ε; for time fields ε is large (e.g., 50% relative).

**Stake**: Option A is structurally clean — separates the "chain attests structural facts" from the "wall clock is run-specific". Option B blurs this and creates flaky tests.

**Recommended ruling**: **Option A — bit-exact on time-insensitive only**. Codify in TB-7 charter § 4.4.

---

### D5 — Audit mode + bundling

**Option A (RECOMMENDED)**: TB-7 = production wire-up class per ruling D3 hybrid. Codex impl audit on TB-7 ship; Gemini arch with `degraded` fallback per `feedback_dual_audit`. **Bundle**: if TB-6 follow-up Codex impl audit on full TB-6 diff has not closed by TB-7 Atom 7, bundle the two audits.

**Option B**: TB-7 = kernel-only-additive class (relax). Self-audit + targeted smoke OK. NO Codex impl audit needed because no STEP_B-protected file touched.

**Stake**: Option A is conservative; matches TB-6 Atom 7 audit class. Option B is fast but loses the production wire-up rigor that ruling D3 mandated for chain-touching work.

**Recommended ruling**: **Option A — production wire-up class with bundling**. Codify in TB-7 charter § 4.5 + § 9.

---

## §3 Optional further decision items (architect discretion)

The following are NOT binding asks but architect input would help:

- **D6**: Should TB-7 bundle a memory-rule update codifying "Frame B = post-TB-6 natural successor" into `feedback_chaintape_wire_up_priority`?
- **D7**: Is the TB sequence post-TB-7 still TB-8 = RSP-M2 NodeMarketEntry, or does the user dialogue (Frame B/C/D) suggest re-sequencing?
- **D8**: Should the chain-derived PPUT (Atom 5) become a hard ship-gate from TB-7+ onward (i.e., "every TB ship from TB-7 must produce chain-derived capability metric") in the same way TB-6 made chain-backed smoke a hard gate (D2)?
- **D9**: Is there a constitution amendment implied? (Probably no — same reasoning as TB-6 D7 — this is roadmap, not constitutional.)

---

## §4 What I propose, in 3 lines

1. **Authorize TB-7 = Frame B** per D1 Option A.
2. **Narrow scope** per D3 Option A — no FinalizeRewardTx, no SlashTx.
3. **Production wire-up audit class** per D5 Option A — Codex + Gemini-with-degraded-fallback at ship; bundle pending TB-6 follow-up if not yet closed.

If architect agrees on these three, the charter draft at `handover/tracer_bullets/TB-7_charter_draft_2026-05-01.md` is ready to run as-is (modulo D2 + D4 fine-tuning). I'll await explicit "authorize TB-7" before touching production code.

---

## §5 Distance-from-final-form context (for sequencing input)

User dialogue post-TB-6 ship surfaced 4 frames:

| Frame | Description | Status | Distance |
|---|---|---|---|
| A | Chain runs, synthetic seed lands | ✅ TB-6 ship | 0 TB |
| B | Every real LLM action on chain | proposed TB-7 | 1 TB |
| C | Full economic loop (stake/slash/settle/market) on chain | TB-9 + TB-11 + RSP-M | 3-4 TB |
| D | Multi-org + public-chain anchor + autonomous self-amendment | P5 + P7 + P8 | 10+ TB |

Architect § 5.2 self-estimate aligned: "Min real economy-mechanism test: +2-3 TBs. Min real NodeMarket/Polymarket test: +4-6 TBs."

TB-7 = Frame B closure. Most user-observable next step (PputResult becomes chain-derived; "真题烟测" actually means real LLM activity on chain).

---

## §6 Pre-execution checklist (for user authorization, after architect ratifies)

Before authorizing TB-7 charter execution, confirm intent on:

1. **D1 Option A confirmed** — TB-7 = Frame B per-LLM-proposal routing.
2. **D2 + D3 + D4 + D5 ruled** — agent keypair lifecycle / OMEGA scope / PPUT tolerance / audit class.
3. **STEP_B preflight needed?** — current scope says NO (no restricted file touched). Confirm at Atom 1 STEP_B-equivalent doc if any sequencer.rs / bus.rs surface emerges.
4. **TB-6 follow-up Codex impl audit closure** — has it run? Should TB-7 bundle?
5. **Memory writes** — D6 ask; if no, no memory updates from TB-7 Atom 0.

**Until architect ratifies, no production code is touched.**

---

## §7 Original analysis context (verbatim)

User asked post-TB-6: "真实的chaintape的最终形态意味着具体是如何实现的，我们距离这个有多远？"

I responded with the 4-frame breakdown (Frame A through D). User then said "handover-update，写TB-7 charter Daft，并提供审计师prompt包含文件路径" — which is what produced this document + the charter draft + the LATEST.md update.

The recommendation set in this prompt is what I would have authored had the user said "go ahead and write TB-7 charter at architect's authority". Architect should rule on D1-D5 to either ratify or amend.

---

## §8 Cross-references

- **Charter draft (the thing being ratified)**: `handover/tracer_bullets/TB-7_charter_draft_2026-05-01.md`
- **TB-6 ruling (precedent)**: `handover/directives/2026-05-01_TB6_ARCHITECT_RULING.md`
- **TB-6 audit (precedent shape)**: `handover/audits/RECURSIVE_AUDIT_TB_6_2026-05-01.md`
- **9-phase roadmap**: `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md`
- **STEP_B protocol**: `STEP_B_PROTOCOL.md`
- **CLAUDE.md restricted file list**: `CLAUDE.md`
