# Architect Directive — 2026-05-02 — Lossless Constitution Integration + Polymarket Absorption + Post-TB-8 Roadmap

**Source**: User-provided full directive package (3 layers, ordered; per user "以最后的为准").
**Status**: **INGESTED — AWAITING AUTHORIZATION** (per `/architect-ingest` SOP, archive ≠ execute).
**Predecessor state**: TB-7R SHIPPED 2026-05-02 (`55680bb` + `46716ae` + `17d69de` on `main`); P2 Frame B closed.
**Supersedes**: First-pass plan from same directive package (Part B below); the **Updated Final Ruling** (Part C) is canonical per user's "以最后的为准" instruction.

---

## §0 Directive structure (as received)

The user delivered three logical layers in one message:

```text
Part A: Lossless Constitution Integrated Edition (无损宪法整编版)
        — re-presents 反奥利奥架构的反奥利奥架构 + 3 preceding articles
        — adds 3 flowchart SHA256 hashes
        — adds annotation layer linking concepts back to source articles
        — preserves full article archives as appendices A/B/C/D

Part B: First architectural plan (TuringOS_Official_Launch_Plan_After_TB7R_2026-05-02)
        — supersession note: SUPERSEDED by Part C

Part C: Updated Final Ruling (更新后的最终裁决)
        — canonical per "以最后的为准"
        — absorbs Polymarket math but rejects ghost liquidity
        — gives 12 numbered architect rulings for AI coder execution
        — extends roadmap to v1.0 (TB-8 through TB-17)
        — mandates 4 new decision records + TRACE_FLOWCHART_MATRIX

Part D: Inline Gemini DeepThink evaluation + Polymarket work-notes
        — referenced by Part C as the trigger for the rejection of
          "automatic 100 YES + 100 NO injection per node"
```

---

## §1 Part A — Lossless Constitution: NEW artifacts

The lossless integrated edition is **mostly a re-presentation** of the existing constitution, but it introduces three durable new artifacts that the project has not previously archived:

### 1.1 Three flowchart SHA256 hashes (verbatim from directive)

```text
Flowchart 1a — Runtime loop, page 8 (rtool / input / Agent δ / output / predicates)
  SHA256: a474c6b9ded766504a4f644a4a1b3c545316d418f0250f36ec692fcdf98f09f5

Flowchart 1b — Runtime loop continuation, page 9 (predicates branch / write tool / Q_{t+1})
  SHA256: b822717b10332a2d8e789ba6af96fd4da4ff43a74afab679d1b82add9c32b64d

Flowchart 2 — Boot + full architecture, page 13 (Initialization + runtime loop + map-reduce + finalization)
  SHA256: 6a4bc9195bafd55bde968fd445cdd2926d6906a7f6a2b38071d4774a7f0de333

Flowchart 3 — Meta-architecture, page 17 (Constitution + logs archive + JudgeAI + ArchitectAI + anti-oreo runtime)
  SHA256: c159413984d0c6c5daa06605fea3a86a2ad4ab9c4284d0d20e0e525bf03aa9cd
```

**Engineering significance**: flowcharts are now elevated from explanatory diagrams to **verification axes**. Each TB henceforth must declare which flowchart element it touches. This requires a new `handover/alignment/TRACE_FLOWCHART_MATRIX.md`.

### 1.2 Three preceding articles formally inherited

```text
《群体智慧的架构：⚪⚫⚪反奥利奥理论》
  — black/white box, two-layer tools, three-layer anti-oreo,
    pricing-not-valuation, white-box feedback logs,
    prior/posterior separation, DSL.

《用图灵机哲学做出一个能通过长周期图灵测试的AI》
  — file system as tape, path as head, state register q,
    context (q,s), transition function δ, read/write loop,
    Turing-completeness proof, "everything is a file".

《验证的非对称性：弱者能不能监管强者？》
  — Solver/Verifier asymmetry, T1-T5 task taxonomy,
    PCP predicates, random local audit, weak-monitors-strong,
    T4 mimicry trap.
```

### 1.3 Markov rule formalization

The lossless edition makes explicit what was previously implicit: **InitAI must NOT read the full historical log on every iteration.** Default context is `constitution + latest error log of previous epoch`. Historical logs are preserved (logs archive as ground truth) but are not loaded into ArchitectAI's working context unless an explicit "顽疾问题 (chronic-disease problem)" override is invoked.

**Engineering implication**: requires `EvidenceCapsule` artifact at the end of every TB (folded into TB-15 in Part C's roadmap).

---

## §2 Part C — Updated Final Ruling: 12 numbered architect rulings

Quoted verbatim from directive (the "AI coder 直接执行口令" block at the end of Part C):

```text
Architect ruling:

1. Absorb the Polymarket notes, but do not resurrect ghost liquidity.

2. The correct core is:
   1 locked Coin = 1 YES_E + 1 NO_E.

3. WorkTx.stake = FirstLong exposure.

4. ChallengeTx.stake = Short / NO exposure.

5. VerifyTx.bond = responsibility bond, not market position.

6. CPMM formula is accepted:
     poolY * poolN = k
     buy_yes(payC):
       outY  = payC * poolY / (payC + poolN)
       getY  = payC + outY
       priceY = payC / getY

7. But MarketSeedTx must debit explicit treasury/LP/sponsor budget.
   No automatic per-node 100 YES + 100 NO injection.

8. on_init may allocate MarketMakerBudget; every node seed
   consumes that budget.

9. Do not remove RSP bounty / escrow. Market incentives supplement
   bounty; they do not replace it.

10. DPMM / pro-rata maker-protection is future experimental scope,
    not v1 core.

11. Add four decision records:
    - DECISION_POLYMARKET_CORE
    - DECISION_CPMM_MINT_AND_SWAP
    - DECISION_MARKET_SEED_NO_GHOST_LIQUIDITY
    - DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN

12. Current next TB remains Minimal Payout / FinalizeRewardTx.

13. NodeMarket starts after durable identity and Lean Proof Task
    Market MVP.

14. Boltzmann masking is read-view / scheduler policy only.
    It never deletes ChainTape parent nodes.

15. Lamarckian Autopsy is private evidence-derived memory,
    not global raw-log broadcast.
```

(Rulings 13-15 appended in the same execution block; numbered 12+ in original directive.)

---

## §3 Post-TB-8 → v1.0 Roadmap (Part C canonical version)

```text
TB-8   Minimal payout / FinalizeRewardTx
        — single solver, single verifier, no royalty, no DAG, no NodeMarket
        — system-only emission; agent submit MUST fail
        — payout_sum ≤ escrow; CTF conserved; replay reconstructs economic_state
        — STATUS: charter already drafted at handover/tracer_bullets/TB-8_charter_2026-05-02.md
                  (uncommitted; awaiting architect authorization — this directive serves as authorization trigger)

TB-9   Durable AgentRegistry + Wallet projection
        — persistent agent pubkey registry
        — WalletTool becomes read-only projection
        — EconomicState canonical; no f64 mutation
        — agent identity survives run restart

TB-10  Lean Proof Task Market MVP
        — TaskOpenTx + EscrowLockTx + WorkTx + VerifyTx + FinalizeRewardTx
        — first user-facing product

TB-11  RSP-M0/M1 NodeMarket Decision + Position Index
        — DECISION_NODEMARKET_POLYMARKET_CPMM.md
        — NodePosition (FirstLong | ChallengeShort | derived)
        — PriceIndex v0 (statistical signal only; not truth)
        — NO trading yet

TB-12  CompleteSet + MarketSeedTx
        — 1 locked Coin = 1 YES_E + 1 NO_E
        — CompleteSetMintTx / MergeTx / RedeemTx
        — MarketSeedTx debits treasury/LP/sponsor (NO ghost liquidity)
        — ConditionalCollateralIndex + ShareBalancesIndex

TB-13  CPMM Router / Mint-and-Swap
        — poolY * poolN = k
        — buy_yes / buy_no router
        — slippage monotonic; constant-product invariant test

TB-14  PriceIndex + Boltzmann Masking
        — price as scheduler signal only
        — child masks parent ONLY if:
            child_price > parent_price + margin
            AND child_verification_status >= parent
            AND child not under unresolved challenge
        — ChainTape NEVER deletes parent

TB-15  Lamarckian Autopsy / Markov Log Loom
        — bankruptcy/liquidation -> AgentPrivateAutopsyCapsule
        — derived from ChainTape evidence (not raw LLM self-report)
        — agent-scoped read view; raw logs shielded
        — Kelly fractional cap as suggestion, not protocol enforcement
        — EvidenceCapsule format for inter-session handover

TB-16  Beta with Market Signals
        — Lean Proof Task Market + basic NodeMarket price signal
        — limited tasks, real ChainTape, real payout, real replay

TB-17  Full Market Trading
        — MarketBuyTx / MarketSellTx / LP positions
        — NOT a v1.0 blocker
```

**v1.0 ship gate** (Part C):

```text
≥100 tasks replayable
all accepted proofs CAS-resolvable
no fake accepted nodes
no ghost liquidity
no agent-submitted system tx
dashboard regeneratable from ChainTape + CAS alone
durable agent identity
minimal payout stable
audit PASS
```

NOT required for v1.0:

```text
public chain anchoring
AMM / CPMM Router (deferred to v1.1+)
full NodeMarket trading
MetaTape
multi-org
royalty
per-tactic DAG
```

---

## §4 Polymarket Absorption: REJECT/ABSORB classification (Part C)

| Item from work-notes | Verdict | Rationale |
|---|---|---|
| `1 locked Coin = 1 YES_E + 1 NO_E` (CompleteSet core) | **ABSORB** | Aligns with existing CTF conservation; matches Polymarket official CTF model |
| `WorkTx.stake = FirstLong`, `ChallengeTx.stake = Short` | **ABSORB** | Maps to existing TB-3/TB-4 inline stake design (no new TypedTx variants) |
| CPMM formula `poolY * poolN = k` + Mint-and-Swap router | **ABSORB (deferred to TB-13)** | Math is correct; must be backed by `LiquiditySeedTx`/`MarketSeedTx` with explicit treasury/LP debit |
| `Each new node automatically gets 100 YES + 100 NO` | **REJECT (literal form)** | Ghost liquidity — violates `on_init`-唯一铸币点 + `1 Coin = 1 YES + 1 NO`. Rewrite as `MarketSeedTx` debiting `MarketMakerBudget` |
| `System market-maker can be 0-loss → no bounty needed` | **REJECT** | Adverse selection vs informed Agent makes 0-loss unattainable in CPMM; bounty + market are complementary, not substitutes |
| Dynamic Pari-Mutuel (DPMM) with pro-rata payout | **DEFER** | Different market class (not CTF); reserve as RSP-M7 experimental, not v1 core |
| Lamarckian Autopsy | **ABSORB (modified)** | Must be private (agent-scoped read view), evidence-derived (not LLM self-report), and never global raw-log broadcast. Matches Art. III屏蔽 + 选择性广播 |
| Kelly Criterion | **ABSORB (as policy, not protocol)** | Risk policy suggestion in autopsy; protocol enforces only `max_position_size`/`max_drawdown`/`max_leverage = 1` |
| Boltzmann masking with child-price > parent-price | **ABSORB (with predicate guard)** | Mask is read-view/scheduler only; ChainTape preservation is invariant; predicate status + challenge resolution must guard the mask |
| Price = oracle of two truths (符合规范 + 离目标更近) | **MODIFY** | Split into `P_accept(node)` and `P_progress(node)` to avoid Goodhart conflation |

---

## §5 Four new decision records mandated (Part C §11)

```text
1. handover/alignment/DECISION_POLYMARKET_CORE_2026-05-02.md
   1 Coin locked = 1 YES_E + 1 NO_E
   YES/NO shares are claims, not Coin
   price is statistical signal, not truth

2. handover/alignment/DECISION_CPMM_MINT_AND_SWAP_2026-05-02.md
   poolY * poolN = k
   buy_yes formula
   buy_no formula
   router flow
   no ghost liquidity

3. handover/alignment/DECISION_MARKET_SEED_NO_GHOST_LIQUIDITY_2026-05-02.md
   No automatic YES/NO injection
   MarketSeedTx must debit explicit budget
   on_init may allocate MarketMakerBudget

4. handover/alignment/DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN_2026-05-02.md
   Autopsy private (agent-scoped)
   Boltzmann masking read-view only
   price does NOT override predicate
   parent NEVER deleted from ChainTape
```

---

## §6 New alignment artifact mandated

```text
handover/alignment/TRACE_FLOWCHART_MATRIX.md

Format:
  Flowchart 1 Runtime Loop:
    rtool -> Agent -> predicates -> wtool -> Q_{t+1}
    Tests:
      every externalized proposal enters L4 or L4.E
      dashboard is materialized view

  Flowchart 2 Boot:
    InitAI -> Q0 -> runtime loop
    Tests:
      genesis_report exists
      TaskOpen/EscrowLock replay

  Flowchart 3 Meta:
    constitution/logs -> ArchitectAI/JudgeAI -> tools/log/Q
    Tests:
      EvidenceCapsule exists
      Markov default context

Each TB charter must declare which flowchart element(s) it touches.

Examples:
  TB-7R -> Flowchart 1 (runtime loop closure)
  TB-8  -> Flowchart 1 (settlement node) + Flowchart 2 (genesis_report continuity)
  TB-15 -> Flowchart 3 (Markov / EvidenceCapsule)
```

---

## §7 Cross-references

- TB-7R verdict (already shipped): `handover/directives/2026-05-02_TB7R_PARENT_TX_DAG_SMOKE_VERDICT.md`
- TB-8 charter (drafted, awaiting authorization): `handover/tracer_bullets/TB-8_charter_2026-05-02.md`
- 9-phase roadmap (still primary axis): `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md`
- Constitution canonical: `constitution.md` (NOT modified by this directive — Art. V.1.1 sudo-only)
- Open OBS that this directive does NOT close:
  - OBS-1 coverage denominator (PartialOk → Complete proof-prefix dependency, TB-8+ scope)
  - OBS-R022 TRACE_MATRIX orphans (`chaintape_mode_gate.rs`, `genesis_report.rs` need future TRACE_MATRIX rows)
  - CHECKPOINT_TB7R_2 #1 (legacy evidence README annotation revert by editor hook)

---

## §8 Verbatim Source Material — companion files

Per `feedback_kolmogorov_compression`, the full text of every part is preserved verbatim in companion files. Nothing is stored by reference to the chat transcript or external sources.

```text
2026-05-02_lossless_constitution_polymarket_directive__part_A_lossless_integrated_edition.md
  Part A main body (§0–§6: Kolmogorov framework + 4 SHA256 + annotation layer
  + inheritance matrix + reading path + visual asset references)

2026-05-02_lossless_constitution_polymarket_directive__part_A_appendix_A_constitution_pdf_extract.md
  Appendix A: PDF text extract of the constitution (pages 1–23)

2026-05-02_lossless_constitution_polymarket_directive__part_A_appendix_B_group_intelligence.md
  Appendix B: full text of《群体智慧的架构：⚪⚫⚪反奥利奥理论》

2026-05-02_lossless_constitution_polymarket_directive__part_A_appendix_C_turing_machine_philosophy.md
  Appendix C: full text of《用图灵机哲学做出一个能通过长周期图灵测试的AI》

2026-05-02_lossless_constitution_polymarket_directive__part_A_appendix_D_verification_asymmetry.md
  Appendix D: full text of《验证的非对称性：弱者能不能监管强者？》

2026-05-02_lossless_constitution_polymarket_directive__part_B_first_plan.md
  Part B: First architectural plan (verbatim, SUPERSEDED by Part C)

2026-05-02_lossless_constitution_polymarket_directive__part_C_updated_final_ruling.md
  Part C: Updated Final Ruling (verbatim, CANONICAL per "以最后的为准")
```

Sections §1–§7 of THIS file are a structured navigation layer (Kolmogorov annotation,
not summary): they reorganize the actionable rulings (12 numbered points,
TB roadmap, REJECT/ABSORB classification, decision-record list) into a single
audit-friendly view. The verbatim source for every claim in §1–§7 is in the
companion files above.

---

## §9 Layer 1 invariant impact detection

Per `/architect-ingest` SOP, the directive must be checked against the three Layer 1 invariants:

### 9.1 `kernel.rs` 零领域知识 — **NOT VIOLATED**

Every concrete change in the roadmap routes through `state/sequencer.rs` / `state/economic_state.rs` / `src/predicates/*` / `src/sdk/tools/*`, never through `kernel.rs`:

| TB | Touchpoint | Kernel impact |
|---|---|---|
| TB-8 FinalizeRewardTx | `state/sequencer.rs` system-tx dispatch arm | None — `kernel.rs` does not learn what payout means |
| TB-9 Durable AgentRegistry | persistent state index | None |
| TB-10 Lean Proof Task Market | `predicates/lean_*` + `state/sequencer.rs` | None |
| TB-11 NodePosition / TB-12 CompleteSet / TB-13 CPMM Router | `state/sequencer.rs` + dedicated economic state modules | None |
| TB-14 PriceIndex + Boltzmann | read-view / scheduler layer | None — masking does not flow through kernel |
| TB-15 EvidenceCapsule / Markov Log Loom | handover/evidence layer + InitAI prompt scaffolding | None |
| Flowchart hashes / 4 decision records / TRACE_FLOWCHART_MATRIX | pure documentation | None |

### 9.2 Append-Only DAG — **STRENGTHENED, NOT VIOLATED**

Multiple rulings *explicitly tighten* the append-only invariant:

```text
Ruling 14 (Part C): "Boltzmann masking is read-view / scheduler policy only.
                     It never deletes ChainTape parent nodes."

Markov rule (Part A §3 annotation):
  "历史日志保留但不进入默认 context"
  — preservation, not deletion.

Ruling 7 (Part C): MarketSeedTx must debit explicit budget; no automatic mint
  — ChainTape entries representing seed liquidity must reference an
    accountable source, no creation-out-of-nothing.
```

No item proposes erasing prior chain entries, mutating L4 entries, or
overwriting L4.E. `feedback_no_retroactive_evidence_rewrite` continues to apply.

### 9.3 Economic conservation — **STRENGTHENED, NOT VIOLATED**

The directive's central thesis IS conservation strengthening. The REJECT list
(Part C §2-§7) was constructed precisely to prevent ghost liquidity:

```text
REJECTED literally:
  "Each new node automatically gets 100 YES + 100 NO" (ghost liquidity)
  "System market-maker can be 0-loss → no bounty needed" (LP risk denial)
  "DPMM pro-rata payout breaking 1:1 redemption" (CTF semantic break)

STRENGTHENED:
  Constitution Law 1 (line 159): Information is Free
  Constitution Law 2 (line 160): Only Investment Costs Money — 1 Coin = 1 YES + 1 NO; on_init 是唯一合法铸币点
  TB-8 acceptance: payout_sum ≤ escrow; CTF conserved; replay reconstructs economic_state
  TB-12 acceptance: 1 locked Coin = 1 YES_E + 1 NO_E
  TB-13 acceptance: constant-product invariant; no supply increase
  Ruling 7: MarketSeedTx debits explicit treasury/LP/sponsor balance
  Ruling 8: on_init MarketMakerBudget consumed per seed
  Ruling 12: FinalizeRewardTx system-only; agent submit forbidden
```

I verified Laws 1-2 exist verbatim in `constitution.md:159-160`. The directive
correctly cites them and does not propose any constitution edit (ruling 15:
"Do not modify constitution.md unless explicitly sudo-authorized").

### 9.4 Verdict

```text
LAYER 1 INVARIANTS: NO VIOLATION
LAYER 1 STRENGTHENING: append-only DAG + economic conservation
SUDO TRIGGER: NONE (constitution.md not modified)
RISK CLASS:
  TB-8 = Class 3 (auth-crypto-money: first system-emitted economic mutator)
  TB-9 = Class 3 (durable identity affects payout authority)
  TB-10 = Class 3 (production wire-up + economic mutator at MVP boundary)
  TB-11 = Class 1 (NodePosition is additive index)
  TB-12 = Class 3 (CompleteSet algebra IS economic mutator)
  TB-13 = Class 3 (CPMM router moves money)
  TB-14 = Class 1 (read-view scheduler; no money movement)
  TB-15 = Class 1 (handover/evidence layer; no money movement)
  Flowchart-hashes / decision-records / TRACE_FLOWCHART_MATRIX = Class 0 (docs)

Per `feedback_dual_audit` + `feedback_risk_class_audit`:
  Class 0: no external audit
  Class 1: self-audit + workspace tests
  Class 3: dual audit (Codex + Gemini, hybrid by risk class)

Directive itself is Class 0 (it's a directive, not code).
Authorization to execute does NOT trigger Layer-1 review; individual TBs
(esp. TB-8 first) will go through their own dual-audit gates.
```

Per `/architect-ingest` step 4: **awaiting user authorization before executing
any item below.** Authorization options surfaced in the user-facing summary.
