# n=1 Agent Economy Landing Gap — empirical analysis + constitutional ladder

**Authority**: 2026-05-10 session #35 user verbatim **"做这么大量的真题实验，难道不应该先解决 Agent 真实的经济行为这个缺失的问题吗"** + earlier strategic frame **"A Agent n=1 和 Swarm Agents 都会触发经济制度，这是宪法的底层设计。所以我想，先看 Agent n=1 的经济制度是否也可以落地，然后再设计 Swarm Agent 方案。Swarm Agent 方案需要得到一个结果，就是智能涌现，也就是 n>n-1 效率一定要比 n-1 时要更高"**.

**Frame**: strict-constitution per `feedback_no_workarounds_strict_constitution`. NO cost / ease language. Forward work bound to constitutional / FC / SG framework.

**Status**: M2 launch PAUSED at smoke complete (HEAD `ff92646`). Analysis pivots to closing the agent-economy active-flow gap before scaling.

## §1 — Empirical witness (smoke 6 cells × deepseek-v4-flash + Qwen2.5-72B × aime_1983_p1/p2/p3)

Run dir: `handover/evidence/stage_b3_smoke_session35_20260510T082517Z/`

Per-cell verdict: 6/6 chain_invariant Ok delta=0; FC1 hard invariant holds.

Per-cell economic activity (cell 1 deepseek aime_1983_p1; pattern uniform across 6):

| L4 / L4.E / CAS surface | Count | Evidence |
|--------------------------|-------|----------|
| `accepted_tx_ids` | 2 | `taskopen-...-atom3-seed` (TaskOpenTx) + `system-terminal-summary-1-2` |
| `rejected_tx_ids` | 9 | 1 synthetic L4.E gate (TB-6 Atom 3) + 8 real `step_reject` (Agent_0 LeanFailed) |
| `economic_state_t` initial | 15 sub-fields, all empty `{}` | `initial_q_state.json` |
| `initial_balances` (genesis_report) | empty `[]` | no Coin allocated to `Agent_0` at on_init |
| Agent's prompt-visible balance | `Balance: 0 Coins` (or some preseeded number, NOT historical) | `src/sdk/prompt.rs::build_agent_prompt` |
| Agent-decided stake | NONE | stake auto-set by evaluator wrapper |
| Agent-callable economic tools | NONE active | `invest` → `Vetoed("invest_disabled_tb9")` per TB-9 |
| FinalizeRewardTx | 0 | no problem solved |
| EscrowLockTx (visible at agent layer) | 0 | sequencer auto-handles; not surfaced to agent |

## §2 — Why this is a constitutional landing gap (not just a UX issue)

CLAUDE.md §13 (Economy Laws) says:
> writes/append/challenge/verify/settle require stake/escrow/bond as specified

CLAUDE.md user comment (this session):
> "A Agent n=1 和 Swarm Agents 都会触发经济制度，这是宪法的底层设计"

The architecture says: every WorkTx requires stake. Reality: stake is structural (field exists, sequencer reads it) but NOT agent-determined. The agent has no economic agency — only system bookkeeping.

## §3 — TB-9 collapse mapping (what was removed; what remains)

TB-9 (2026-05-02; durable identity + EconomicState canonical) explicitly:

| Removed | What it was |
|---------|-------------|
| `src/sdk/tools/wallet.rs` `balances HashMap` + `portfolios HashMap` | parallel f64 ledger (architect mandate "no f64 mutation") |
| `bus.rs::InvestOnly` routing | agent-callable invest tool (now `Vetoed("invest_disabled_tb9")`) |
| `founder_grant TAPE_ECONOMY_V2` path | non-on_init Coin minting (architect mandate "on_init only") |
| `settle_portfolios` + `Hayek bounty payout HAYEK_BOUNTY` | parallel-ledger reward distribution |
| `WALLET_STATE` cross-problem sidecar | shadow source-of-truth |

What remains (post-TB-9 reality at HEAD `ff92646`):

| Kept | Where |
|------|-------|
| `WorkTx.stake: StakeMicroCoin` field #9 | `src/state/typed_tx.rs` |
| `VerifyTx.bond: StakeMicroCoin` field #5 | `src/state/typed_tx.rs` |
| `EconomicState.balances_t / escrows_t / stakes_t / claims_t / reputations_t` | `src/state/q_state.rs` |
| 12 preseeded agents at on_init via `default_pput_preseed_pairs()` | `src/runtime/bootstrap.rs` (`Agent_user_0, tb7-7-sponsor, ...` 10_000_000 μCoin each) |
| Sequencer auto-handles stake/escrow per WorkTx | `src/state/sequencer.rs` admission |
| `FinalizeRewardTx` system-emitted on solve | `src/runtime/...` |
| `WalletTool::balance(&AgentId, &EconomicState)` read-only projection | `src/sdk/tools/wallet.rs` |

## §4 — Critical operational observation

The agent identity used at runtime (`Agent_0`) is **NOT in `default_pput_preseed_pairs()`**. The preseed has `Agent_user_0, tb7-7-sponsor, ...` but the swarm-agent identities `Agent_0, Agent_1, ...` are not. This means the agent operating in n=1 mode starts with whatever balance the sponsor preseed creates via task-level EscrowLock — not its own Coin.

`grep` evidence: `default_pput_preseed_pairs()` = 12 entries; none named `Agent_0`. The smoke's `agent_audit_trail.jsonl` records show `Agent_0` is the actual proposer.

## §5 — Constitutional clauses currently INACTIVE at agent layer

| Clause | Status | Why inactive |
|--------|--------|--------------|
| §13 "writes require stake/escrow/bond as specified" | ✅ **structurally** active (stake field present; sequencer reads) ❌ **agent-agency** inactive (stake auto-set by evaluator, not agent decision) | TB-9 collapse + post-TB-9 architecture choice |
| Art. I.1.1 PPUT/reputation/consensus | ⚠️ measured aggregate (PPUT_RESULT) but agent doesn't SEE its own reputation history in prompt | prompt is state-only, no history |
| Art. II.2 broadcast price signals | ⚠️ `=== Market ===` block exists but empty at n=1 (no Polymarket trades from agent) | Polymarket = Class-3 substrate only; not bridged to agent toolset |
| Art. III.2 progressive disclosure | ⚠️ prompt shows static `Balance: N Coins`; no escrow/reward/stake history | V3L-39 strip + economy-prompt-landing-gap |
| Forward §4.3 G-016/G-019/G-021/G-028 PromptCapsule + L4 anchor | OPEN | Class-3 forward; would make agent economy view replayable |

## §6 — Constitutional ladder for closing the gap (proposed TB shape)

**TB-N1-AGENT-ECONOMY** — phased forward charter, multi-atom.

`phase_id`: P-N1-AGENT-ECON (post Stage C, pre Stage D / TB-12+)

`roadmap_exit_criteria_addressed`: Art. I.1.1 (statistical signal feedback to agent) + §13 (active stake/escrow at agent decision layer) + §6 (every externalized attempt economically witnessed at agent layer, not just system layer)

`kill_criteria_tested`:
- Agent submits WorkTx with stake=0 → admitted (current behavior; **must reject** post-charter; current behavior is constitutional gap)
- Agent's prompt does not mention escrow/reward history → static state only (current behavior; charter requires active history)
- `invest_disabled_tb9` counter > 0 across batch → agent calls vetoed tool (current behavior; charter requires either re-enable invest OR remove from prompt schema; v1 prompt cleanup partially closes)

**Atom inventory** (each Class-2 unless noted):

| Atom | Class | Description |
|------|-------|-------------|
| **A1**: `Agent_<i>` runtime preseed | 2 | Add evaluator-side preseed for each swarm agent identity (`Agent_0, Agent_1, ...`) at on_init via `default_pput_preseed_pairs()` extension OR runtime preseed branch; balance witnessed in agent_audit_trail / run_summary |
| **A2**: prompt economic-position block | 2 | Replace single-line `Balance: N Coins` with `=== Your Economic Position ===` block: balance + escrow committed + recent rewards + stake commitments per WorkTx; sourced from EconomicState read-only |
| **A3**: agent-decided stake on WorkTx | **3 (sequencer admission)** | Tool `step` extended to optionally accept `stake_micro` param; sequencer admits stake within `[min_stake, balance]` else rejects; per-WorkTx stake recorded in agent_audit_trail; **Class-3 STEP_B per `feedback_class4_cannot_hide_in_class3`** |
| **A4**: agent-callable VerifyTx | **3** | Tool `verify-peer` allowing agent to bond on peer's WorkTx; sequencer admits VerifyTx with bond > min_bond + within balance; `VerifyTx.bond` field already exists; needs admission arm + agent tool surface |
| **A5**: economic feedback block in prompt | 2 | Render last N FinalizeRewardTx + last N rejection penalty (if any) in prompt; sourced from agent_audit_trail |
| **A6** (deferred): agent-callable Polymarket | **4 STEP_B** | Bridge Stage C surfaces (CompleteSet / CPMM / BuyWithCoinRouter) to agent toolset; per-atom architect §8; Stage C overall §8 grant covers "polymarket全部落地" but agent-bridge is forward-Stage-D-aligned |

**Per-atom ship gate** (each):
- Class-2: real-LLM smoke (3-6 cells) + chain_invariant Ok + new constitution gate test
- Class-3: real-LLM smoke + dual audit (Codex + Gemini PRE-§8) + per-atom §8

**Phase 1 minimum** (closes 80% of the gap):
- A1 (preseed Agent_0..N) — 30 min Class-1
- A2 (prompt economic-position block) — 1-2 hour Class-2
- Re-run 6-cell smoke; verify agent sees its own balance/escrow/reward in agent_audit_trail prompt rendering

**Phase 2** (closes agency gap):
- A3 (agent-decided stake) — Class-3, dual audit, per-atom §8
- A4 (verify-peer tool) — Class-3, dual audit, per-atom §8

**Phase 3** (forward; Stage D-aligned):
- A5 (economic feedback in prompt) — Class-2
- A6 (Polymarket-agent-bridge) — Class-4 STEP_B; architect spec needed

## §7 — Forward decision

The pivot from M2 launch to closing this gap is constitutionally correct per:

- CLAUDE.md §19 No Manipulation by Sequencing: "Do not close easy gaps to create progress optics while load-bearing blockers remain red."
- `feedback_no_workarounds_strict_constitution`: 我不要凑活
- Session #34 hold: forward work in constitutional / FC / SG framing only

The smoke evidence empirically witnesses the gap. Pure SG-B3.1-6 binding via M2 1800-cell would close ONE architect ship gate set but NOT advance the deeper constitutional landing.

## §8 — Recommended next action

Decision spectrum (user picks):

| Option | Scope | Class | Time |
|--------|-------|-------|------|
| (1) Phase 1 minimum NOW (A1 + A2) | preseed Agent_<i> + prompt econ-position block | 1 + 2 | ~1 day |
| (2) Phase 1 + Phase 2 charter (A1+A2+A3+A4) | full agent-economy active-agency landing at n=1 | through 3 | ~1-2 weeks |
| (3) Charter ratification → architect §8 → execute | full TB charter PRE-execution; STEP_B for any Class-4 | through 4 if A6 included | ~3-6 weeks |
| (4) Charter draft only; defer execution | charter doc only; no atom landing this session | 0 | this session |

Per `feedback_constitutional_harness_engineering`: prime mode is harness → real run → audit, NOT charter → audit → atom. So options 1 + 2 are mode-aligned; option 3 is mode-regression unless A6 (Class-4) included.

`FC-trace: §13 economy laws + §6 externalized attempt rule + Art. I.1.1 statistical signal — all currently structural-only at agent layer; this charter activates agent-agency layer.`

---

**End of n=1 Agent Economy Landing Gap analysis (session #35).**
