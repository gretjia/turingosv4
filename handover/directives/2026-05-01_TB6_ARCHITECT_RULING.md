# Architect Ruling — TB-5 audit + TB-6 ChainTape wire-up directive + RSP-M NodeMarket future plan

**Date**: 2026-05-01
**Source**: User-relayed architect ruling (response to `handover/directives/2026-05-01_TB6_ARCHITECT_FULL_PROMPT.md`, commit `9f89dcc`).
**Branch at ingest**: `main` @ `9f89dcc` (TB-5 SHIPPED + post-TB-5 self-audit + chaintape gap + architect prompt).
**Form**: full natural-language ruling delivered in two structurally identical passes (one ChatGPT-style headered narrative + one dense sequential transcript). Both convey identical 7-decision verdict.
**Scope reach**: TB-5 audit acceptance + TB-6 charter pre-authorization + RSP-M NodeMarket/Polymarket future track + 5 memory updates + 1 explicit "no constitutional amendment" ruling.
**Status**: ARCHIVED. **NOT YET AUTHORIZED FOR EXECUTION.** Per `/architect-ingest` step 4: await explicit user approval before charter / memory writes / smoke renames.

---

## §0 Headline verdict (architect's own words)

> **TB-6 必须优先做 P2 Agent Runtime / Production ChainTape Wire-up。**
> 不要继续推进 RSP-3.2 Slash，不要启动 NodeMarket，不要扩展 P6 能力指标。

> **TB-6 不做更多经济功能。TB-6 只做一件事：让 TuringOS 的真实烟测进入 ChainTape。**

Reframed:

```
到 TB-5 为止，TuringOS 的内核是有的；
内核在 cargo test 中是绿的；
但没有任何 LLM-driven production run 真正经过 Sequencer::apply_one；
也没有任何 on-disk ChainTape；
所以我们现在无法审计真实 Agent 到底做了什么。
```

> 引擎是真的；台架测试是绿的；但还没有装到车上跑一圈。

---

## §1 TB-5 process audit

### §1.1 TB-5 acceptance with caveat

TB-5 kernel work **accepted** but must carry caveat:

```
Kernel green under tests.
Production ChainTape unexercised.
Smoke evidence is not audit-grade ChainTape.
```

What TB-5 actually shipped (verified):
- Anti-Oreo agent-vs-system ingress separation **structurally enforced** at runtime spine
- `emit_system_tx` constructs+signs internally
- `apply_one` stage 1.5 verifies via `PinnedSystemPubkeys`
- ChallengeResolve Released + UpheldDeferred paths green
- CTF conserved across mixed sequences
- 9-sub-field EconomicState + 5-holding invariant preserved
- `cargo test --workspace` = 617/617

### §1.2 What current "smoke evidence" actually proves (and does not)

Proves:
- LLM 跑了
- Lean 验证了
- `prompt_context_hash` 稳定
- proof artifact 可重验
- PPUT_RESULT 能写 stdout

Does NOT prove:
- `Sequencer::apply_one` was called
- `LedgerWriter::commit` was called
- `tx_payload_cid` was written to CAS
- `LedgerEntry` has `system_signature`
- `parent_ledger_root` chain exists
- replay can reconstruct QState
- L4.E shields rejected raw diagnostic
- economic state can be reconstructed from ChainTape

> 如果有人改 `n1_run.log` 里的两个字符，当前代码没有任何 invariant 会捕捉，因为它没有签名、没有链、没有 replay path。
> 这不是小问题，这是 **testing-platform gap**。

---

## §2 Binding decisions — D1 through D7

### D1 — TB-6 sequencing
**Ruling**: **Path A** — TB-6 = P2 Agent Runtime / Production ChainTape Wire-up.
**Not selected**: Path B (RSP-3.2 Slash).
**Rationale**: Slash without real ChainTape just expands the "5-TB kernel-only debt" into "6-TB kernel-only debt." User has already flagged inability to audit tape / git / economy / Agent actions.

### D2 — Smoke gate evolution
**Ruling**: **(b) Hard requirement from TB-6.**
TB-6 ship-gate smoke must produce walkable / replayable on-disk ChainTape.
Required conditions to call something a "smoke tape / ChainTape smoke":
1. Production binary triggers `Sequencer::apply_one`
2. `LedgerWriter::commit` writes on-disk chain
3. Each entry has `parent_ledger_root` / `resulting_ledger_root`
4. Each entry has `tx_payload_cid`
5. Each entry has `system_signature`
6. CAS artifact retrievable
7. Replay reconstructs QState / EconomicState
8. Rejected raw diagnostic does NOT enter agent-facing view

### D3 — Audit mode standard
**Ruling**: **(d) Hybrid by risk class.**

| Atom class | Audit mode |
|---|---|
| kernel-only additive | self-audit + targeted smoke OK |
| production wire-up | Codex implementation audit + Gemini architecture audit; if Gemini unavailable → must label as `degraded`, not pretend full dual audit |
| system-emitted economic mutator | stricter audit |
| NodeMarket / market resolve | stricter audit |

**TB-6 is production wire-up → self-audit alone insufficient.**

### D4 — Test-count reporting standard
**Ruling**: `cargo test --workspace` is canonical.
Required report shape:
```
command = cargo test --workspace
workspace_count = N
failed = 0
ignored = M
```
**Forbidden**: mixing root-crate `cargo test` count with workspace count.

### D5 — "Smoke tape" rename
**Ruling**: **Approved.**
- pre-TB-6 → `smoke evidence`
- post-TB-6 chain-backed → `ChainTape smoke / smoke tape`

"Tape" only for real ledger chain. "stdout + README + proof.lean" cannot be called tape.

### D6 — Memory updates
**Ruling**: **All 5 approved.**
- New: `feedback_workspace_test_canonical.md`
- New: `feedback_smoke_evidence_naming.md`
- New: `feedback_chaintape_wire_up_priority.md`
- Update: `feedback_dual_audit.md` (degraded mode labeling)
- Update: `feedback_iteration_cap_24h.md` (production wire-up exception)

### D7 — Constitution amendment
**Ruling**: **NO constitutional amendment.**
Rationale: This is a roadmap / testing-platform gap, not a constitutional gap. Constitution already mandates: top-level white-box manages quantization/broadcast/shielding, soft natural-language constraints insufficient → must become hard constraints, Agent cannot directly write state. Foundational laws (`Information is Free`, `Only Investment Costs Money`, `1 Coin = 1 YES + 1 NO`, `on_init` sole mint) already in place. Updates needed: ROADMAP / TB methodology / smoke naming. **constitution.md untouched.**

---

## §3 TB-6 charter pre-specification (architect-issued)

### §3.1 TB name + phase
- **Name**: `TB-6 — P2 Agent Runtime: Production ChainTape Wire-up`
- **phase_id**: `P2 (primary) + P1/P3 validation carry-forward`

### §3.2 Goal
Let at least one LLM-driven run truly traverse:
```
rtool/context
→ Agent proposal
→ WorkTx
→ Sequencer::apply_one
→ LedgerWriter::commit
→ on-disk ChainTape
→ replay / verify_chain / reconstruct
```

### §3.3 roadmap_exit_criteria_addressed
- P1:5 — accepted tx changes state_root / ledger_root
- P1:6 — rejected tx leaves state_root unchanged + writes L4.E
- P1:7 — L4 / L4.E chain deletion breaks verification
- P1:8 — state.db / QState reconstructable from L4
- P1:9 — rejected raw diagnostics do not enter materialized read view
- P2:6 — all Agent outputs enter CAS; ledger records CID
- P3 carry-forward — WorkTx.stake / escrow / ChallengeResolve invariants still replay under production ChainTape

### §3.4 kill_criteria_tested
- P1:1 — Agent cannot bypass wtool / Sequencer to mutate state
- P1:2 — rejected WorkTx does not advance state_root
- P1:3 — on-disk ChainTape can reconstruct state
- P1:4 — rejected raw log does not pollute agent read view
- P3:1 — no post-init mint
- P3:2 — stakeless WorkTx fails
- P3:3 — no ghost liquidity / market-free conservation

### §3.5 Required deliverable directory shape
```
handover/evidence/tb_6_chaintape_smoke_2026-05-XX/
  runtime_repo/
    .git/
    refs/transitions/main
    refs/rejections/main  (or equivalent)
  cas/
  ledger_entries.jsonl
  replay_report.json
  chaintape_report.md
  proof.lean
  pput_result.jsonl
  README.md
```

README must answer:
- What entered CAS?
- What entered L4?
- What entered L4.E?
- What was replayed?
- What was verified by signature?
- What was reconstructed?
- What did the Agent see / propose? Which branches were rejected? Which became accepted?

### §3.6 Atom plan (architect-issued, 8 atoms)

**Atom 0 — Charter + naming cleanup**
- Write TB-6 charter
- TB_LOG add TB-6 active
- ROADMAP update: P2 ChainTape wire-up precedes RSP-3.2 Slash
- NOTEPAD update: TB-5 shipped, but ChainTape production gap = high priority
- Rename historical "smoke tape" → "smoke evidence"

**Atom 1 — Production runtime repo bootstrap**
```rust
RuntimeChaintapeConfig {
    runtime_repo_path,
    cas_path,
    writer_kind: Git2LedgerWriter,
    run_id,
}
```
Production-like binary initializes: CasStore + Git2LedgerWriter + Sequencer + system keypair + initial QState.

**Atom 2 — Evaluator → Sequencer adapter**
First version (do NOT rewrite evaluator at once). Adapter only:
- Agent proposal / candidate proof → `WorkTx`
- Lean accept → accepted WorkTx path
- Lean fail / predicate fail → rejected WorkTx / L4.E path
- PPUT_RESULT remains output, but linked to ChainTape tx ids
Minimum: 1 accepted + 1 rejected WorkTx.

**Atom 3 — Chain-backed smoke run**
- Simple problem (`mathd_algebra_107`, small MAX_TX)
- Must produce ≥ 1 accepted WorkTx L4 entry + ≥ 1 rejected L4.E entry
- If no natural rejected branch: synthetic rejected WorkTx through production path with explicit `synthetic_rejection_for_l4e_gate = true` label.

**Atom 4 — Replay verifier**
New CLI / test:
```
verify_chaintape --repo <runtime_repo> --cas <cas> --run-id <run>
```
Output:
```json
{
  "l4_entries": 1,
  "l4e_entries": 1,
  "ledger_root_verified": true,
  "system_signatures_verified": true,
  "state_reconstructed": true,
  "economic_state_reconstructed": true,
  "cas_payloads_retrievable": true
}
```

**Atom 5 — Agent audit trail**
Each Agent proposal links: `agent_id`, `prompt_context_hash`, `read_set`, `write_set`, `proposal_cid`, `candidate_proof_cid`, `tx_id`, `predicate_results`, `accepted_or_rejected`, `rejection_class`.
**Do NOT record private chain-of-thought.** Record what the Agent saw + submitted + how system judged. (Aligns with constitutional "selective shielding": preserve auditable artifacts, do not broadcast raw thought streams.)

**Atom 6 — Branch / fork visibility**
Record: `tx_count`, `failed_branch_count`, `rollback_count`, candidate proposal CIDs, accepted tx_id, rejected tx_ids.
Records **proposal-level fork**, not chain-of-thought.

**Atom 7 — Audit + ship**
- Codex implementation audit: Does LLM-driven run produce on-disk ChainTape? Can it replay? Are signatures verified? Are CAS artifacts linked? Does L4.E exist for rejected path?
- Gemini architecture audit: Does TB-6 close honest-naming gap? Is smoke tape now chain-backed? Anti-Oreo still aligned?

---

## §4 RSP-M NodeMarket / Polymarket future track (post-TB-6)

### §4.1 Mathematical core
For any node-level event `E_node ∈ {YES, NO}`:
```
1 locked Coin = 1 YES_E + 1 NO_E

if E = YES: YES_E → 1 Coin, NO_E → 0
if E = NO:  YES_E → 0, NO_E → 1 Coin
```

In TuringOS, `E_node` = node accepted / survives challenge / gets reused.
Price `p_yes + p_no ≈ 1` is **statistical signal only, not truth.**

### §4.2 First-long economics (architect-issued)
```
WorkTx.stake     = proposer FirstLong exposure
ChallengeTx.stake = challenger Short / NO exposure
VerifyTx.bond    = verifier responsibility bond, NOT long/short
```

Currently these are stake/bond embryos. Full NodeMarket must wait until after TB-6 because without real ChainTape, NodeMarket long/short would only be in-memory state — unreplayable, unauditable.

### §4.3 RSP-M track sequence (post-TB-6)
- **RSP-M0**: NodeMarket decision record at `handover/alignment/DECISION_NODE_MARKET_FIRST_LONG_2026-05-XX.md`
- **RSP-M1**: NodePosition derived index (exposure index, NOT Coin holding)
- **RSP-M2**: NodeMarketEntry + PriceIndex v0 (statistical signal, no trading)
- **RSP-M3**: CompleteSet accounting (`CompleteSetMintTx` / `CompleteSetRedeemTx`)
- **RSP-M4+**: MarketOrder / trading layer (no automatic liquidity / no ghost liquidity / system-only `MarketResolveTx`)

### §4.4 RSP-M0 decision content
1. `WorkTx.stake` is FirstLong exposure
2. `ChallengeTx.stake` is Short / NO exposure
3. `VerifyTx.bond` is responsibility bond, not market position
4. Price is statistical signal, not truth
5. Node outcome resolved by predicates + ChallengeCourt + system-emitted resolution
6. No automatic liquidity injection
7. No ghost liquidity
8. Positions are exposure indexes, not Coin holdings

### §4.5 Updated overall TB sequence
```
TB-6:  P2 Production ChainTape Wire-up
TB-7:  P2 Agent proposal/fork audit trail OR RSP-M0/M1 NodePosition
TB-8:  RSP-M2 NodeMarketEntry + PriceIndex v0
TB-9:  RSP-3.2 Slash execution  (ONLY after real ChainTape replay exists)
TB-10: RSP-M3 CompleteSet accounting
TB-11: RSP-4 SettlementEngine / ContributionDAG
TB-12: RSP-M4 MarketOrder / trading layer
```

### §4.6 NodeMarket data structures (advance design)
```rust
pub enum PositionSide { Long, Short }

pub enum PositionKind { FirstLong, ChallengeShort, MarketBuy, MarketSell }

pub struct NodePosition {
    pub position_id: TxId,
    pub node_id: TxId,
    pub task_id: TaskId,
    pub owner: AgentId,
    pub side: PositionSide,
    pub amount: MicroCoin,
    pub source_tx: TxId,
    pub kind: PositionKind,
    pub opened_at_round: u64,
}

pub struct NodeMarketEntry {
    pub node_id: TxId,
    pub task_id: TaskId,
    pub event_kind: NodeMarketEventKind,
    pub status: NodeMarketStatus,
    pub opened_at_round: u64,
    pub resolved_at_round: Option<u64>,
    pub long_interest: MicroCoin,
    pub short_interest: MicroCoin,
}
```
**Invariants**: `NodePosition.amount` does NOT count toward `total_supply_micro`. `NodePosition` is exposure index, not Coin holding.

---

## §5 "How far from real testing"

### §5.1 Current real-smoke level
Verifies: LLM emits proof, Lean verifies, `prompt_context_hash` stable, `PPUT_RESULT` writes stdout.
Does NOT verify: ChainTape, Git2LedgerWriter, CAS linkage, system_signature, parent_ledger_root, ledger replay, economic state replay, agent fork trace, L4.E shielding, RSP money movement in production, NodeMarket/Polymarket mechanics.

### §5.2 Distance to milestones
- Min real ChainTape test: **+1 TB (TB-6, ~5–7 atoms)**
- Min real economy-mechanism test: **+2–3 TBs (TB-6/7/8)**
- Min real NodeMarket/Polymarket test: **+4–6 TBs**
- Min real Agent fork audit: **+2 TBs (TB-6 + TB-7)**

---

## §6 TB-6 forbidden list (architect-issued)

TB-6 must NOT do:
- SlashTx
- NodeMarket
- CompleteSet
- AMM
- P6 capability expansion
- MetaTape
- public chain
- Continue calling paper-trail "tape"

RSP-M (when activated, post-TB-6) must NOT:
- Auto-inject YES/NO liquidity
- Allow ghost liquidity
- Treat price as truth
- Treat NodePosition as Coin holding
- Let Agent emit `MarketResolveTx`
- Bypass system ingress barrier

---

## §7 Final execution order (architect-pre-authored, NOT YET RUN)

```
1. Accept TB-5 as kernel-green but production-ChainTape-red.
2. TB-6 = Path A: P2 Agent Runtime / Production ChainTape Wire-up.
3. Do not proceed to RSP-3.2 Slash before TB-6.
4. From TB-6 onward, ship-gate smoke must be chain-backed.
5. Rename all historical "smoke tape" → "smoke evidence"
   unless backed by LedgerEntry chain.
6. Canonical test count is cargo test --workspace.
7. TB-6 must produce at least one LLM-driven on-disk ChainTape:
   Sequencer::apply_one
   Git2LedgerWriter::commit
   CAS payloads
   LedgerEntry chain
   system_signature
   replay report
   reconstructed QState / EconomicState
   proposal-level agent audit trail
8. No P6 capability work, no h_vppu expansion, no MetaTape,
   no public chain, no NodeMarket during TB-6.
9. After TB-6, open RSP-M NodeMarket track:
   WorkTx.stake = first-long exposure
   ChallengeTx.stake = short / NO exposure
   VerifyTx.bond = responsibility bond
   price = statistical signal, not truth
   no ghost liquidity
   no automatic YES/NO injection
10. Use risk-class audit:
    production wire-up requires Codex implementation audit
    + strategic audit if available
    (degraded label if Gemini exhausted).
```

---

## §8 Ingest-time impact analysis (Layer 1 invariants)

| Layer 1 invariant | Directive impact | Status |
|---|---|---|
| `kernel.rs` zero-domain-knowledge | TB-6 wires production binary to existing kernel; adds NO domain knowledge to kernel | ✅ preserved |
| Append-Only DAG | TB-6 EXPLICITLY enforces production runs through `Sequencer::apply_one` + `LedgerWriter::commit` + on-disk chain | ✅ STRENGTHENED |
| Economic conservation (`5-holding CTF`, `1 Coin = 1 YES+1 NO`, on_init sole mint) | TB-6 forbids new economic mutators; RSP-M (future) explicitly forbids ghost liquidity / auto-injection / treating positions as Coin | ✅ preserved |
| Agent ≠ direct state writer (Anti-Oreo) | TB-6 mandates rejected raw diagnostic stays out of agent-facing view; agent audit trail records only what Agent saw + submitted, not chain-of-thought | ✅ STRENGTHENED |
| Append-Only DAG: rejected → L4.E (separate from L4) | Atom 3 mandates ≥1 accepted L4 entry + ≥1 rejected L4.E entry on production smoke | ✅ STRENGTHENED |
| Constitution.md hygiene | D7 explicitly rules NO amendment | ✅ preserved |

**Verdict**: Directive does NOT violate Layer 1. It TIGHTENS Layer 1 enforcement (production-path proof). Safe to authorize execution.

---

## §9 Pre-execution checklist (for user authorization)

Before authorizing, confirm intent on:

1. **Path A vs Path B confirmation** — TB-6 = ChainTape wire-up, NOT RSP-3.2 Slash. (Ruling D1.)
2. **Audit mode for TB-6** — Codex implementation + Gemini architecture; if Gemini exhausted, ship-time evidence carries explicit `degraded` label. (Ruling D3.)
3. **Smoke evidence rename scope** — confirm rename touches `handover/evidence/tb_{1..5}_*` directories AND any references in TB_LOG / NOTEPAD / READMEs. (Ruling D5.)
4. **Memory writes** — 3 new + 2 update memory files (Ruling D6) — confirm before writing to `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/`.
5. **Atom 0 sequencing** — should Atom 0 (charter + rename + ROADMAP/NOTEPAD update) be one commit or split? Architect prescribed it as a bundle.
6. **NodeMarket future track** — RSP-M is reserved-future, NOT TB-6. Confirm we are NOT pre-creating RSP-M0 decision record during TB-6.

**Until user authorizes, no execution.**

---

## §10 Original directive content (verbatim, both passes)

[Archived in repo log via commit body of this file's commit. Two parallel structurally-identical passes were delivered; both convey the same 7-decision verdict with the same atom plan + RSP-M track. Pass 1 = ChatGPT-style headered narrative ("最高优先级裁决" → "最终判断"); Pass 2 = dense sequential transcript ("一句话裁决" → "最终执行口令"). The reconstructed canonical form above merges both faithfully without omission of binding content.]

**Self-reported author of pass note**: ruling delivered AFTER reading commit `9f89dcc` (TB-6 architect full prompt) and `8c6d95f` (post-TB-5 self-audit + chaintape gap). Architect explicitly added D6 (memory updates) + D7 (constitution amendment ruling) on top of original 5-decision request.
