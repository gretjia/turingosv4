<!--
  Archive header — added by /architect-ingest 2026-05-07.
  ===================================================================
  Source           : architect message, 2026-05-07
  Companion file   : 2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_zh.md
  Status           : ARCHIVED — top-tier architect alignment document.
  Authorization    : NOT YET AUTHORIZED for execution. Receiving an
                     instruction is not authorization to execute.
                     Action items inside (TB-18R Final closure,
                     constitution AMBER closure, HEAD_t C2,
                     CompleteSetMergeTx, CpmmPool, Mint-and-Swap
                     Router, restricted beta, etc.) require explicit
                     Class-3 / Class-4 ratification per CLAUDE.md §10
                     before any TB charter executes them.
  Layer-1 impact   : None detected.
                     - kernel.rs zero-domain-knowledge: not amended
                     - Append-Only DAG: not amended
                     - Economic conservation: reaffirmed, not amended
                     - Constitution clauses: no new clauses; restates
                       existing FC1/FC2/FC3, Art. III shielding, CTF
                       invariant `1 Coin = 1 YES + 1 NO`, no-f64-money,
                       no-ghost-liquidity, predicate-as-truth.
  Operating-mode   : Reaffirms Constitutional Harness Engineering as
                     Prime Operating Mode (CLAUDE.md §2). No deviation.
  Freeze-status    : Document instructs continued freeze on public
                     benchmark claim, real-world readiness, public chain,
                     real-money market, NodeMarket trading, AMM/CPMM
                     router, formal H-VPPU claim — until critical gates
                     are green. This is consistent with current
                     `MEMORY.md` / TB-C0 freeze-lift carve-outs.
  Intended use     : Canonical engineering spec for AI coders for the
                     Polymarket / CPMM track. Future TB charters that
                     touch market mechanics MUST cite this document
                     (and its Chinese twin) in their
                     `roadmap_exit_criteria_addressed` field.
  Companion records: companion Chinese doc holds the architect's
                     narrative-style alignment view; this English doc
                     is the structured engineering manual. Both kept
                     verbatim per Kolmogorov-compression rule
                     (`feedback_kolmogorov_compression`).
  ===================================================================
-->

# TuringOS Constitution-Harness → Launch Plan + Polymarket/CPMM Implementation Manual

Date: 2026-05-07
Purpose: current-state audit, constitution-first harness plan, launch roadmap, and AI-coder executable Polymarket/CPMM implementation guide.

***

## 0. Executive verdict

Current progress is materially better than the pre-reset state.

The project has moved from "audit-driven atomic engineering" to "Constitutional Harness Engineering". The latest reported current snapshot indicates:

- workspace tests: 1181 passed / 0 failed / 151 ignored;
- constitution gates: 97 passed / 0 failed / 1 ignored;
- matrix RED rows: 0;
- matrix AMBER rows: 13;
- HEAD_t C1 is implemented via real git2-rs/libgit2 commit OIDs updating q.head_t;
- Wave 3 evidence binding promoted several AMBER rows to GREEN.

However, the project is not yet in "constitution fully landed" state. Remaining AMBER rows still matter, especially around empirical Art. 0 binding, PPUT report discipline, selective broadcast/shielding, boot Q_0 / no memory-only preseed, and deeper Art. III shielding. Feature expansion is allowed only if it does not cross these gates.

Main directive:

```
Do not return to old Atomic Agentic Engineering.
Use Constitutional Harness Engineering:
  constitution gate -> real run -> debug -> fix -> audit -> ship.
```

***

## 1. Current-state audit

### 1.1 Accepted as landed or substantially landed

#### 1.1.1 Tape-first operating mode

CLAUDE.md now correctly centers:

```
Tape-first:
  paper / write tool / rubber / strict predicates decide L4 / L4.E.
No tape activity = not a TuringOS test.
```

This is the right operating philosophy.

#### 1.1.2 FC1/FC2/FC3 gate vocabulary exists

The three constitutional flowcharts have been turned into operational gates:

```
FC1 Runtime Loop:
  Q_t -> rtool/context -> Agent output -> predicate/oracle -> wtool -> L4 or L4.E

FC2 Boot:
  genesis_report + ChainTape + CAS + agent registry + system pubkeys

FC3 Meta/Markov:
  EvidenceCapsule / Markov capsule = derived view, not hidden ground truth.
```

This is correct. Future features must cite these gates.

#### 1.1.3 HEAD_t C1 appears grounded

Based on reported source audit:

- Git2LedgerWriter uses real git2-rs/libgit2.
- L4 accepted entries are real Git commits on refs/transitions/main.
- advance_head_t captures real 40-hex commit OID after commit and stores it in q.head_t.
- head_t_witness.rs covers the six-field witness.

This means C1 is real, not fake hash theater. C2 remains future work.

#### 1.1.4 TB-18R Phase 3 / Wave 3 evidence improved constitution gates

Reported status:

- Constitution gates: 90 -> 97 PASS.
- Workspace tests: 1174 -> 1181 PASS.
- Wave 3 evidence binding added 7 tests.

This is exactly the kind of evidence-binding the harness should require.

#### 1.1.5 Polymarket base objects already partially landed

Reported source audit says the following are already landed:

- CompleteSetMintTx
- CompleteSetRedeemTx
- MarketSeedTx
- ConditionalCollateralIndex
- ConditionalShareBalances
- NodePosition / PriceIndex
- BoltzmannMaskPolicy + boltzmann_select_parent_v2
- ChallengeResolveTx system-signing

So the previous claim that Redeem and Boltzmann were entirely missing was inaccurate.

### 1.2 Still not fully landed

#### 1.2.1 C2 Git ref unification

C1 is real, but C2 remains:

```
refs/chaintape/l4
refs/chaintape/l4e
refs/chaintape/cas
```

Acceptance requires L4, L4.E, and CAS roots to be represented as coherent Git refs, not only L4 transitions/main.

#### 1.2.2 Remaining AMBER rows

Known remaining AMBER areas:

- Art. 0 empirical binding rows
- Art. I.2 PPUT reporting discipline
- Art. II selective broadcast rows
- Art. III shielding rows
- Art. IV boot Q_0 + no_memory_only_preseed

These are not cosmetic. They directly determine whether large benchmark and market tests can be trusted.

#### 1.2.3 Polymarket is not fully implemented

Even though CompleteSetMint/Redeem/MarketSeed exist, the full market is not implemented.

Known intentional gaps:

- CompleteSetMergeTx
- CPMM / AMM Router
- SwapTx
- LiquidityPool reserves
- real trading
- LP share accounting
- market settlement dashboard
- ShareBalances export CLI
- exposure-adjusted PPUT

These should not be started before benchmark and constitution gates justify market work.

***

## 2. Freeze / unfreeze rules

### 2.1 Freeze remains for

Until the constitutional critical gates are green:

- No public benchmark claim.
- No "formal benchmark passed" claim.
- No real-world readiness.
- No public chain.
- No real-money market.
- No NodeMarket trading.
- No AMM/CPMM router.
- No external-domain tasks.

### 2.2 Allowed now

Allowed work:

- Constitution gates
- Wave 1/2/3 closures
- P38/P49/M0 diagnostic real runs
- 20p / 50p diagnostic MiniF2F
- TB-18R Final sign-off packaging
- C2 Git refs design
- polymarket implementation charter drafting
- non-mutating documentation/manuals

### 2.3 Allowed after critical gates

After:

- constitution gates green
- P38/P49 attempt equality green
- HEAD_t C1 green
- PromptCapsule anchored
- PCP synthetic corpus green
- no RED rows
- remaining AMBER justified or forward-bound

then resume:

- TB-18B benchmark scale-up
- TB-19 real-world pilot design
- TB-20 sandbox pilot
- TB-21 restricted beta / market expansion

***

## 3. Global harness gates

### 3.1 FC1 Runtime Loop

Requirement:

Every externalized Agent output that affects system state must be represented in ChainTape/CAS.

Acceptance:

```
evaluator_reported_completed_llm_calls
=
l4_work_attempt_count
+ l4e_work_attempt_count
+ capsule_anchored_attempt_count
```

Halt if:

- Lean reject appears only in stdout.
- Attempt count mismatch.
- Legacy authoritative append path used.
- Dashboard needs stdout to reconstruct core facts.

### 3.2 FC2 Boot

Requirement:

Every run replayable from genesis_report + ChainTape + CAS + agent registry + system pubkeys.

Acceptance:

- fresh replay rebuilds q_state, economic_state, HEAD_t, dashboard facts.

Halt if:

- memory-only preseed
- missing genesis_report
- global pointer source-of-truth
- post-hoc reconstruction

### 3.3 FC3 Markov / Meta

Requirement:

EvidenceCapsule / MarkovEvidenceCapsule must be derived from ChainTape + CAS.

Acceptance:

- No global latest pointer.
- Raw logs shielded.
- Latest capsule context derived from tape/CAS.
- Deep history requires override.

Halt if:

- raw logs leak
- capsule is hidden source-of-truth
- global pointer reappears
- automatic predicate/tool mutation

### 3.4 Economy gate

Requirement:

- Information is Free.
- Only Investment Costs Money.
- 1 Coin = 1 YES + 1 NO.
- on_init is only base Coin mint.

Acceptance:

- total_coin_conserved
- no post-init mint
- no ghost liquidity
- wallet read-only
- no f64 in money path
- system tx not agent-submittable

***

## 4. Roadmap from now to launch

### Stage A — Constitution landing closure

#### A1. TB-18R Final

Goal:

```
Ship TB-18R only after current-head real evidence validates tape restoration.
```

Required evidence:

- P38
- P49
- M0 mini-batch
- attempt equality report
- audit_tape report
- dashboard regeneration report

Ship gates:

```
SG-A1.1 P38 attempt equality green.
SG-A1.2 P49 attempt equality green.
SG-A1.3 M0 mini-batch green.
SG-A1.4 no fake accepted nodes.
SG-A1.5 every real Lean reject represented in L4.E or anchored EvidenceCapsule.
SG-A1.6 chain facts derived from ChainTape/CAS, not evaluator stdout.
SG-A1.7 final dual audit PASS under VETO > CHALLENGE > PASS.
```

#### A2. Wave 1 / Wave 2 AMBER closure

Goal:

```
Close remaining no-dependency static and parser/manifest AMBER rows.
```

Targets:

- Art. 0 empirical binding
- Art. I.2 PPUT reporting discipline
- Art. II selective broadcast
- Art. III shielding
- Art. IV boot Q_0 / no memory-only preseed

Ship gates:

```
SG-A2.1 constitution gates >= current 97 and no regression.
SG-A2.2 all new gate files included in scripts/run_constitution_gates.sh.
SG-A2.3 every matrix promotion has a real witness.
SG-A2.4 no doc-only GREEN promotions.
```

#### A3. HEAD_t C2 design / implementation

Goal:

```
Upgrade C1 HEAD_t witness into C2 multi-ref libgit2 production refs.
```

Required refs:

- refs/chaintape/l4
- refs/chaintape/l4e
- refs/chaintape/cas

Ship gates:

```
SG-A3.1 L4 head ref advances on accepted transition.
SG-A3.2 L4.E head ref advances on rejected evidence.
SG-A3.3 CAS root ref advances when CAS evidence added.
SG-A3.4 replay reconstructs HEAD_t from refs.
SG-A3.5 no hidden filesystem pointer.
```

### Stage B — Formal benchmark scale-up

#### B1. 20p diagnostic

Goal:

```
Expose remaining architecture bugs under small real MiniF2F batch.
```

Ship gates:

```
SG-B1.1 all runs chain-backed.
SG-B1.2 all failures have L4.E or EvidenceCapsule.
SG-B1.3 dashboard regenerates.
SG-B1.4 no evidence drift.
SG-B1.5 BenchmarkManifest exists.
```

#### B2. 50p controlled benchmark

Goal:

```
Intermediate benchmark confidence.
```

Ship gates:

```
SG-B2.1 50/50 chain_invariant reports exist.
SG-B2.2 aggregate report not dependent on stdout.
SG-B2.3 PPUT report discipline green.
SG-B2.4 no hidden excluded runs.
```

#### B3. 100p / M2 benchmark

Only after B1/B2 green.

Ship gates:

```
SG-B3.1 sampled full replay works.
SG-B3.2 failure-heavy sample replay works.
SG-B3.3 solved sample replay works.
SG-B3.4 unsolved sample replay works.
SG-B3.5 EvidencePackagingPolicy satisfied.
SG-B3.6 no public SOTA/benchmark claim unless audit-approved.
```

### Stage C — Polymarket / RSP-M implementation

Do not start executable trading before Stage A is green and Stage B at least B1 is green.

Stages:

- C1 Polymarket Manual / DECISION records
- C2 CompleteSet audit hardening
- C3 CompleteSetMergeTx
- C4 ShareBalances export
- C5 MarketSeed sanity + collateral accounting
- C6 CPMM LiquidityPool + Mint-and-Swap Router
- C7 controlled market smoke
- C8 restricted beta market

Detailed manual is in section 7 below.

### Stage D — Real-world readiness

After benchmark and market sandbox.

- D1 Real-world readiness documents
- D2 oracle design
- D3 challenge court design
- D4 irreversible action policy
- D5 low-risk pilot design
- D6 sandbox pilot
- D7 restricted beta

No real-world task before:

- oracle
- challenge
- delayed settlement
- human escalation
- irreversible-action ban
- safety boundary

***

## 5. Key acceptance standards by location

### 5.1 Evaluator

Must:

- emit AttemptTelemetry for every externalized LLM-Lean cycle
- emit LeanResult for every Lean check
- route pass/fail to L4/L4.E or anchored capsule
- produce chain-derived run facts

Halt if:

- N attempts collapse into 1 WorkTx

### 5.2 Sequencer

Must:

- enforce system-only tx
- preserve total Coin
- prevent fake accepted nodes
- advance HEAD_t correctly
- write Git-backed L4
- route rejection to L4.E

Halt if:

- system tx accepted via agent ingress
- post-init mint
- predicate failure enters L4

### 5.3 CAS

Must:

- store PromptCapsule
- AttemptTelemetry
- LeanResult
- EvidenceCapsule
- MarkovEvidenceCapsule
- proof artifacts

Halt if:

- CID exists but payload missing
- CAS race
- unresolvable evidence used as proof

### 5.4 Dashboard

Must:

- be regeneratable from ChainTape + CAS
- never be source-of-truth
- show attempt DAG
- show L4/L4.E split
- show economic movement
- show market exposure
- show price as signal, not truth

Halt if:

- dashboard requires evaluator stdout for core facts

### 5.5 Market / Polymarket

Must:

- use integer/rational math
- not f64
- no ghost liquidity
- no automatic YES/NO injection
- shares are claims, not Coin
- collateral is Coin holding
- price cannot override predicates

Halt if:

- liquidity created without collateral
- shares counted as Coin
- f64 money path
- price-as-truth

***

## 6. Polymarket design principles

### 6.1 Core CTF invariant

For every binary event:

```
1 locked Coin = 1 YES_E + 1 NO_E
```

YES/NO are claims.
They are not Coin.
Locked collateral is Coin holding.

### 6.2 TuringOS event types

Allowed event classes:

```
E_accept:
  node accepted by predicate
E_survive:
  node survives challenge window
E_bankrupt:
  task/run failed or exhausted
E_resolution:
  system ChallengeResolve / TaskBankruptcy outcome
```

Do not create market over subjective events without a predicate / oracle plan.

### 6.3 WorkTx and ChallengeTx mapping

```
WorkTx.stake -> FirstLong exposure
ChallengeTx.stake -> Short / NO exposure
VerifyTx.bond -> responsibility bond, not market position
```

### 6.4 Bounty vs market

Do not remove bounty.

```
Task bounty = base labor incentive
Market exposure = information/risk incentive
```

Markets supplement bounty.
They do not replace escrow reward in v1.

***

## 7. AI-coder Polymarket implementation manual

### 7.1 Phase P-M0 — Quarantine legacy CPMM

Before any market work:

```
src/prediction_market.rs legacy f64 CPMM must not be imported by new code.
```

Tasks:

```
1. Mark module as legacy/quarantined.
2. Add grep/compile test:
   no TB-market module imports prediction_market legacy API.
3. Add no_f64 test for new market modules.
```

Ship gates:

```
legacy_cpm_api_not_imported_by_new_market
no_f64_in_market_modules
```

### 7.2 Phase P-M1 — CompleteSet audit hardening

Current status indicates Mint/Redeem exist. Harden them.

Required objects:

- ConditionalCollateralIndex
- ConditionalShareBalances
- CompleteSetMintTx
- CompleteSetRedeemTx

Mint semantics:

```
balances_t[owner] -= amount
conditional_collateral_t[event_id] += amount
share_balance[(owner,event,YES)] += amount
share_balance[(owner,event,NO)] += amount
```

Redeem semantics:

```
requires resolved outcome
if outcome=YES:
  burn YES shares
  owner balance += share_amount
  collateral -= share_amount
if outcome=NO:
  burn NO shares
  owner balance += share_amount
  collateral -= share_amount
losing side receives 0
```

Tests:

```
mint_one_coin_creates_one_yes_one_no
mint_conserves_total_coin
shares_not_counted_as_coin
redeem_unavailable_before_resolution
redeem_yes_after_yes_pays_yes_not_no
redeem_no_after_no_pays_no_not_yes
redeem_cannot_exceed_share_balance
redeem_debits_collateral
```

### 7.3 Phase P-M2 — CompleteSetMergeTx

Purpose:

```
allow 1 YES + 1 NO -> 1 Coin before resolution
```

This gives agents a non-settlement exit and prepares liquidity mechanics.

Struct:

```rust
pub struct CompleteSetMergeTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub event_id: EventId,
    pub owner: AgentId,
    pub amount: ShareAmount,
    pub signature: AgentSignature,
}
```

Semantics:

```
require owner YES >= amount
require owner NO >= amount
burn amount YES
burn amount NO
conditional_collateral_t[event] -= amount
balances_t[owner] += amount Coin
```

Tests:

```
merge_yes_no_returns_coin
merge_requires_both_sides
merge_conserves_total_coin
merge_reduces_collateral
merge_unavailable_after_final_redeem_if shares exhausted
```

### 7.4 Phase P-M3 — MarketSeedTx hardening

MarketSeedTx must be collateral-backed.

Semantics option A:

```
provider deposits seedC Coin
CompleteSetMintTx-like operation creates seedC YES + seedC NO
YES/NO shares go to pool inventory
collateral locks seedC
```

Struct:

```rust
pub struct MarketSeedTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub event_id: EventId,
    pub provider: AgentId,
    pub collateral_amount: MicroCoin,
    pub signature: AgentSignature,
}
```

Tests:

```
market_seed_debits_provider
market_seed_creates_yes_no_inventory
market_seed_fails_insufficient_balance
market_seed_no_ghost_liquidity
market_seed_conserves_total_coin
```

### 7.5 Phase P-M4 — LiquidityPool state

Add only after CompleteSet and MarketSeed are hardened.

State:

```rust
pub struct CpmmPool {
    pub event_id: EventId,
    pub pool_yes: ShareAmount,
    pub pool_no: ShareAmount,
    pub lp_total_shares: LpShareAmount,
    pub status: PoolStatus,
}
```

Rules:

```
pool_yes and pool_no are share balances controlled by pool
pool reserves are not Coin
lp shares are not Coin
k = pool_yes * pool_no
```

Tests:

```
pool_created_from_seed_inventory
pool_reserves_not_counted_as_coin
lp_shares_not_counted_as_coin
pool_cannot_exist_without_collateralized_shares
```

### 7.6 Phase P-M5 — CPMM Swap YES/NO only

Before Coin router, implement pure share swap.

Buy YES with NO:

```
input: dN > 0
outY = floor(dN * poolY / (poolN + dN))
poolN1 = poolN + dN
poolY1 = poolY - outY
```

With integer rounding:

```
poolY1 * poolN1 >= poolY * poolN
```

because floor keeps dust in pool.

Symmetric Buy NO with YES:

```
outN = floor(dY * poolN / (poolY + dY))
poolY1 = poolY + dY
poolN1 = poolN - outN
```

Tests:

```
swap_no_for_yes_constant_product_non_decreasing
swap_yes_for_no_constant_product_non_decreasing
swap_fails_zero_input
swap_fails_insufficient_pool_output
swap_respects_min_out_slippage
swap_uses_integer_math_no_f64
```

### 7.7 Phase P-M6 — Mint-and-Swap Router

This implements the architect's formula.

#### BuyYesWithCoinRouter

User pays payC.

Atomic steps:

```
1. Debit buyer Coin by payC.
2. Lock payC collateral.
3. Mint payC YES + payC NO to router.
4. Transfer payC YES to buyer.
5. Swap payC NO into CPMM pool.
6. Pool receives dN = payC NO.
7. Router receives outY YES:
   outY = floor(payC * poolY / (poolN + payC))
8. Transfer outY YES to buyer.
9. buyer receives getY = payC + outY.
```

Effective price:

```
priceY = payC / getY
```

Post-reserves:

```
poolY1 = poolY - outY
poolN1 = poolN + payC
```

Integer invariant:

```
poolY1 * poolN1 >= poolY * poolN
```

#### BuyNoWithCoinRouter

Symmetric:

```
outN = floor(payC * poolN / (poolY + payC))
getN = payC + outN
poolY1 = poolY + payC
poolN1 = poolN - outN
```

Tests:

```
buy_yes_with_coin_matches_formula
buy_no_with_coin_matches_symmetric_formula
buy_yes_debits_coin_locks_collateral
buy_yes_mints_complete_set
buy_yes_transfers_retained_yes_plus_swap_yes
buy_yes_respects_min_yes_out
buy_yes_no_f64
buy_yes_no_ghost_liquidity
router_atomic_rollback_on_failure
```

### 7.8 Phase P-M7 — PriceIndex from CPMM / exposure

Price is signal only.

```
price_yes_effective = quote_payC / quote_getY
price_no_effective = quote_payC / quote_getN
```

Do not use price to decide predicate truth.

Tests:

```
price_quote_does_not_change_state
price_signal_not_predicate
price_does_not_make_failed_node_accepted
low_liquidity_warning
```

### 7.9 Phase P-M8 — Audit tools

Add:

```
audit_tape view-shares
audit_tape view-pools
audit_tape view-prices
audit_tape view-positions
```

Must show:

- owner YES/NO shares
- conditional collateral
- pool reserves
- LP shares
- NodePositions
- price signal

Tests:

```
audit_view_shares_matches_state
audit_view_pools_matches_state
dashboard_regenerates_market_view
```

### 7.10 Phase P-M9 — Controlled market smoke

Only after all above.

Scenario:

```
Lean task
Agent A WorkTx FirstLong
Agent B ChallengeTx Short
MarketSeedTx by sponsor or treasury
BuyYesWithCoin
BuyNoWithCoin
PriceIndex update
Task resolved
Redeem / merge
Autopsy if loss
```

Gates:

- no ghost liquidity
- total coin conserved
- no price-as-truth
- no raw log broadcast
- all activity replayable

***

## 8. Polymarket forbidden list

Never implement:

- automatic per-node 100 YES + 100 NO without collateral
- Treasury magic seed without debit
- f64 money math
- DPMM / pro-rata payout inside CTF track
- price-based settlement
- agent-submitted MarketResolveTx
- agent-submitted system resolution
- AMM before CompleteSet
- trading before audit tools
- public chain before sandbox
- real money before readiness gate

***

## 9. Final launch route

### Launch Alpha

Requirements:

- constitution gates green
- Lean Proof Task Market stable
- payout stable
- attempt equality stable
- dashboard regeneration stable
- PCP synthetic corpus green
- PromptCapsule anchored
- HEAD_t C1/C2 sufficient

### Launch Beta

Add:

- CompleteSet + Merge + MarketSeed
- PriceIndex signal
- audit tools
- controlled market smoke
- no real-money trading

### Launch v1

Add:

- restricted beta with capped funds
- human escalation
- delayed settlement
- market router only after audit
- clear user risk disclosure

### Real-world

Only after:

- REAL_WORLD_READINESS_REPORT
- ORACLE_REQUIREMENTS
- CHALLENGE_COURT_REQUIREMENTS
- SAFETY_BOUNDARY
- IRREVERSIBLE_ACTION_POLICY
- human sign-off

***

## 10. AI coder master command

```
Proceed with Constitution Harness first.

Do not start Polymarket executable features until:
- current constitution gates are green,
- TB-18R Final is signed,
- 20p/50p diagnostic evidence is stable,
- CompleteSet current implementation is audited.

When starting Polymarket:
1. quarantine legacy f64 CPMM;
2. harden CompleteSet Mint/Redeem;
3. implement CompleteSetMergeTx;
4. harden MarketSeedTx;
5. implement integer CpmmPool;
6. implement share-only swap;
7. implement Mint-and-Swap router;
8. add audit views;
9. run controlled market smoke.

At every phase:
- no f64,
- no ghost liquidity,
- no price-as-truth,
- no dashboard source-of-truth,
- no real funds,
- no public chain.
```
