# TB-N1-AGENT-ECONOMY Phase 2 — Charter

**Authority**: 2026-05-10 session #35 user verbatim **"我要的是 TuringOS engine 有序完整落地"** + earlier strategic frame **"A Agent n=1 和 Swarm Agents 都会触发经济制度，这是宪法的底层设计... Swarm Agent 方案需要得到一个结果，就是智能涌现，也就是 n>n-1 效率一定要比 n-1 时要更高"** + **"我不要凑活的方案，我不考虑成本和 easy"**.

**phase_id**: `P-N1-AGENT-ECON-PHASE-2`

**Predecessor**: TB-N1-AGENT-ECONOMY Phase 1 SHIPPED at HEAD `a5625a6` (A1 preseed engagement + A2 prompt economic-position block).

**Frame**: strict-constitution per `feedback_no_workarounds_strict_constitution`. Closes the **agency layer** of n=1 economy: Phase 1 closed the **passive perception** layer (agent sees state + EscrowLockTx accepted to L4); Phase 2 lets the agent **act on** its economic position (decide stake + verify peers).

## §0 — Class classification (Class-4 STEP_B)

Both A3 and A4 touch **sequencer admission semantics**. Per CLAUDE.md §9 + `feedback_class4_cannot_hide_in_class3`:

| Atom | Class | Why |
|------|-------|-----|
| **A3** agent-decided stake | **4 STEP_B** | Sequencer admission gate change: admit WorkTx if `stake ∈ [min_stake, balance]`; reject otherwise. New typed `RejectionClass::StakeOutOfBounds`. Sequencer admission is canonical per CLAUDE.md §12 STEP_B file list. |
| **A4** agent-callable verify-peer | **4 STEP_B** | Sequencer admission for agent-submitted VerifyTx: admit if `bond ∈ [min_bond, balance]` AND target_work_tx is canonical L4-accepted; reject otherwise. New rejection-class. VerifyTx already has `bond: StakeMicroCoin` field at schema level (no typed_tx.rs change). |

Both atoms require:
- STEP_B parallel-branch development (NOT direct main edit)
- PRE-§8 dual audit (Codex + Gemini) before architect §8 dispatch
- Per-atom architect §8 sign-off (NOT batched per `feedback_no_batch_class4_signoff`)
- Trust Root rehash if pinned file touched (sequencer.rs IS pinned; expect rehash)

## §1 — Constitutional binding

| Constitutional clause | Currently | Phase 2 after |
|-----------------------|-----------|----------------|
| **CLAUDE.md §13** "writes/append/challenge/verify/settle require stake/escrow/bond as specified" | structurally engaged (stake field exists; auto-set) | **agent-decision active** (stake within [min, balance] determined by agent at proposal time) |
| **CLAUDE.md §6** every externalized attempt is tape-visible | ✅ holds | ✅ holds (FC1 hard invariant maintained) |
| **CLAUDE.md §14** predicate gate boolean | ✅ holds | ✅ holds + **stake-bounds gate** added as predicate input |
| **Art. I.1.1** statistical signal feedback | one-way (PPUT computed; agent doesn't see) | partial: agent perceives stakes_t / claims_t (from Phase 1 A2); A5 forward (recent rewards / penalties) closes feedback loop |
| **FC1-N7** δ / Agent externalized output | static prompt | informed by economic position + stake-decision capability |

`roadmap_exit_criteria_addressed` (P0-P9 9-phase roadmap):
- **P3 economic primitives at agent layer** (stake / bond / verify) — currently structural; Phase 2 closes
- **P4 multi-agent dynamics precondition** (agent must have economic agency before swarm makes sense) — Phase 2 closes precondition

`kill_criteria_tested`:
- Agent submits `step` with `stake=0` → **must reject** (current behavior admits with auto-stake; Phase 2 inverts)
- Agent submits `step` with `stake>balance` → **must reject** with `StakeOutOfBounds` rejection class
- Agent submits `verify-peer` against non-existent target_work_tx → **must reject**
- Agent submits `verify-peer` with `bond>balance` → **must reject**
- Agent's prompt-displayed stake commitments must equal `Σ stakes_t.values().filter(staker == agent_id).amount` (post-A3 invariant)

## §2 — Atom inventory

### Atom A3 — Agent-decided stake on `step` tool

**Surface**: `experiments/minif2f_v4/src/protocol_step.rs` (or wherever the step tool action handler lives) + `src/sdk/protocol.rs` (tool action schema) + `src/state/sequencer.rs` admission arm + `src/state/typed_tx.rs::TransitionError::StakeOutOfBounds` variant.

**Behavior**:
1. `step` action JSON schema gains optional `stake_micro: u64` field. Default = `min_stake_per_problem` (read from env / static const) when absent.
2. Evaluator builds `WorkTx` with `stake = stake_micro`.
3. Sequencer admission gate: reject WorkTx if `stake < min_stake OR stake > agent_balance`. New `RejectionClass::StakeOutOfBounds = 10` (extension; if admission RejectionClass enum is at 9 currently, this is a tail-append).
4. `prompt.rs::build_agent_prompt` updates schema documentation block to mention the `stake_micro` parameter.

**Ship gates** (SG-N1-A3.*):
- SG-N1-A3.1 (test): `tests/constitution_n1_agent_economy_a3.rs` — agent submits stake=0 → rejected with StakeOutOfBounds
- SG-N1-A3.2 (test): agent submits stake=balance+1 → rejected with StakeOutOfBounds
- SG-N1-A3.3 (test): agent submits stake=min_stake → accepted (positive control)
- SG-N1-A3.4 (test): post-A3, prompt's `Active stakes` line reflects per-cell agent-decided amounts (NOT auto-stake)
- SG-N1-A3.5 (real-LLM smoke): 6-cell smoke (3 problems × 2 models × 1 seed × 1 rep) shows ≥1 cell with WorkTx admitting agent-decided non-default stake; chain_invariant Ok delta=0
- SG-N1-A3.6 (constitution gate test): tests/constitution_economy_gate.rs gains `agent_decided_stake_within_bounds` test
- Trust Root: rehash sequencer.rs + typed_tx.rs + evaluator.rs (all pinned)

**Kill condition**: SG-N1-A3.5 chain shows ZERO cells where agent picked non-default stake → A3 hasn't actually engaged agent agency.

### Atom A4 — Agent-callable `verify-peer` tool

**Surface**: `src/sdk/protocol.rs` (tool action schema) + `experiments/minif2f_v4/src/bin/evaluator.rs` (action handler dispatch) + `src/state/sequencer.rs` (admission arm extension; VerifyTx admission already exists per TB-7 OMEGA-Confirm path; needs path to admit AGENT-submitted VerifyTx not just system-emitted).

**Behavior**:
1. New tool: `{"tool":"verify_peer","target_work_tx_id":"<TxId>","verdict":"confirm|deny","bond_micro":<u64>}`.
2. Evaluator builds VerifyTx with the agent's signature; submits via `bus.submit_typed_tx`.
3. Sequencer admission: reject if (a) `bond < min_bond OR bond > agent_balance`, (b) `target_work_tx` not canonical L4-accepted, (c) target_work_tx already verified by this agent (no duplicate verification). Accept otherwise.
4. New `RejectionClass::VerifyBondOutOfBounds = 11` + `RejectionClass::VerifyTargetNotAccepted = 12` + `RejectionClass::VerifyDuplicate = 13`.

**Ship gates** (SG-N1-A4.*):
- SG-N1-A4.1 through SG-N1-A4.5 (test): all 4 reject paths + 1 admit path
- SG-N1-A4.6 (real-LLM smoke): n=2 swarm 6-cell smoke shows ≥1 cell where Agent_1 verifies Agent_0's WorkTx; chain_invariant Ok delta=0
- SG-N1-A4.7 (constitution gate): `verify_peer_admission_gate` test
- Trust Root: rehash sequencer.rs + typed_tx.rs + evaluator.rs (all pinned)

**Kill condition**: SG-N1-A4.6 shows zero agent-submitted VerifyTx → A4 hasn't engaged agent verify-agency.

### Future atoms (forward; NOT in Phase 2)

- **A5** (Class-2): prompt economic feedback (recent FinalizeRewardTx + rejection penalty). Independent of Phase 2.
- **A6** (Class-4 STEP_B; Stage D-aligned): Polymarket-agent-bridge (CompleteSet / CPMM / BuyWithCoinRouter as agent tools). Architect §8 needed; deferred behind explicit Stage D ship gate.

## §3 — Execution sequence (strict ordered)

For EACH of A3 then A4:

1. STEP_B parallel branch: `feat/n1-econ-a3-rebuild` / `feat/n1-econ-a4-rebuild`
2. Implementation: code + tests + Trust Root rehash + harness gate test
3. cargo test --workspace + bash scripts/run_constitution_gates.sh
4. Real-LLM smoke (6-cell deepseek + Qwen)
5. PRE-§8 dual audit: Codex G2 + Gemini DeepThink (PROCEED required from both per `feedback_dual_audit` Class-4)
6. Architect §8 packet draft: `handover/directives/2026-05-XX_TB_N1_AGENT_ECONOMY_A<n>_§8_PACKET.md`
7. Architect §8 sign-off: `handover/directives/2026-05-XX_TB_N1_AGENT_ECONOMY_A<n>_§8_SIGN_OFF.md` (verbatim multi-clause Class-4 form)
8. Merge feat/* → main
9. Final smoke at HEAD post-merge
10. LATEST.md handover update

NO batching: A3 ships fully (steps 1-10) BEFORE A4 starts. Per `feedback_no_batch_class4_signoff`.

## §4 — Forbidden additions

Inherited from CLAUDE.md §20 freeze conditions + per-charter:
- NO M2 batch run during Phase 2 (M2 evidence would be invalidated when A3/A4 lands; per `feedback_smoke_before_batch` + cost discipline)
- NO Polymarket-agent-bridge (A6 Stage D-aligned)
- NO swarm n>1 batch (Phase 2 substrate is n=1; swarm comes after Phase 2)
- NO new typed_tx variant (RejectionClass tail-append only, no schema bump)
- NO canonical signing payload change (WorkTx + VerifyTx signing payloads unchanged)
- NO push to origin/main without architect §8 sign-off

## §5 — Constitution Landing Gate dependency

Per CLAUDE.md §8 + `feedback_constitutional_harness_engineering`:
- Each ship gate test (SG-N1-A3.* + SG-N1-A4.*) MUST be added to `scripts/run_constitution_gates.sh`
- `bash scripts/run_constitution_gates.sh` must pass at every atom boundary

## §6 — Risk envelope

Per CLAUDE.md §10 / authorization scope:
- Class 4 surfaces touched: `src/state/sequencer.rs` (admission), `src/state/typed_tx.rs` (RejectionClass tail-append; NOT new variant)
- Class 3 surfaces touched: `src/sdk/protocol.rs` (tool action schema), `src/sdk/prompt.rs` (schema doc), `experiments/minif2f_v4/src/bin/evaluator.rs` (action dispatch)
- Trust Root files affected: sequencer.rs + typed_tx.rs + evaluator.rs
- No money path f64 introduction
- No system-emitted tx pathway changed (still system-only for TaskOpenTx / EscrowLockTx / FinalizeRewardTx)
- No memory-only canonical state introduced

## §7 — Authorization request

This charter requires architect §8 ratification BEFORE STEP_B execution begins. Per CLAUDE.md §10:
- Class-4 work requires explicit architect ratification
- Single-word authorizations (`fix`, `go`, `continue`, etc.) do NOT constitute Class-4 sign-off
- Multi-clause forward grants (e.g., user verbatim "授权自主执行直到 X") DO constitute Class-4 forward grant per `feedback_dual_audit` + CLAUDE.md §10 multi-clause analysis

Pending user verbatim authorization of this charter scope.

`FC-trace: §13 stake/escrow/bond agency layer + Art. I.1.1 statistical signal (forward; A5 closes) + FC1-N7 δ Agent externalized output enriched with economic decision capability + FC1 hard invariant (every WorkTx with stake_micro tape-visible).`

---

**End of TB-N1-AGENT-ECONOMY Phase 2 charter (draft; pending user §8).**
