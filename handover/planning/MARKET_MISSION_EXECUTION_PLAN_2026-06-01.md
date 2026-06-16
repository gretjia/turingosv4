# TuringOS Market-Mission — EXECUTION PLAN (architect-ratified 2026-06-01)

Execution-authorization layer over `MARKET_MISSION_TASK_PACKAGE_2026-06-01.md`. The task package is the
prereg/spec (GO); this document is the **gated execution contract**. Ratified by the architect's independent
Go/No-Go (2026-06-01) + two code-grounded refinements from this session.

## 0. Authorization (binding)

| Item | Verdict | Scope |
|---|---|---|
| Task package as prereg/spec | **GO** | structure, prior-art honesty, price-null counter-evidence, shuffle/flatbid causal gate |
| **TP-0A** (renamed, narrowed) | **GO — start now** | MarketTape-lite replay gate ONLY; bin-local; no lib.rs; no §6; no "full Art.0 canonical" claim |
| **TP-1** (spec lock) | **GO — start now, concurrent** | 5-pillar microstructure spec/test/prereg only; reads no counted result |
| **TP-2** counted sweep | **NO-GO until G0 ∧ G1** | coordinator/market smoke allowed after G0∧G1; counted only after both green |
| **TP-3** Art.V meta-market | **NO-GO / prereg-skeleton only** | frozen-competence-matrix design; headline = Sybil-defunding, not raw uplift |
| **TP-4** effective-N | **NO-GO until complete Stage-2 Q1** | park; if Q1=JUST_SAMPLING, rewrite as reputation+cost study |

**Campaign verdict (ratified):** *Weak-GO as a governance/auditability campaign; No-Go as an immediate
price-efficiency campaign.* Efficiency stays a **falsifiable scientific hypothesis (the upside)**, not the
narrative pillar.

## 1. Mission (reframed, accepted)

> **Primary:** In untrusted, heterogeneous, attackable agent ecosystems, TuringOS provides an **auditable,
> Sybil-resistant, Goodhart-shielded, accountable** agent-compute institution — verifiable & replayable from a
> frozen tape. **Upside (to be falsified, not assumed):** that this same price/ledger/predicate institution also
> allocates compute to verification-passing paths *more efficiently* than a coordinator-swarm.

The analogy is **rule-of-law / fraud-resistance**, NOT "capitalism > communism efficiency" — the repo's own
data (H0: single 7 ≥ shuffled 6 > market 5; 5× aggregation-null) says price is a near-null *efficiency*
allocator on a monolithic pool, but causal in *adversary-robust sequential allocation* (Sybil-defunding 10/10).

## 2. Two-level verdict (EXPLICIT split — supersedes the merged GO/WEAK-GO/NO-GO)

Report **A and B separately**; A is NEVER inferred from B.

- **Verdict A — price-causal efficiency** (the upside hypothesis):
  `MARKET > COORDINATOR-SYNTH-LLM` **AND** `MARKET > SHUFFLED-PRICE` **AND** `MARKET > FLATBID`, all on
  **banked@B**, paired (Wilcoxon + Holm), **p<0.05**, every headline arm replay-GREEN + axiom-clean.
  *Primary causal gate = MARKET > SHUFFLED-PRICE* (hardest control: same price distribution, permuted);
  MARKET > FLATBID is the secondary "any-dispersion-beats-uniform" gate. **Honest prior: A is NULL on the
  44-pool; A is tested only on the decomposable combination-target subset.**
- **Verdict B — institutional governance** (the defensible floor): even if A ties, B holds iff the market arm is
  replay-GREEN **∧** Sybil-resistant (TP-1 honest-bidding falsifier) **∧** Goodhart-shielded (predicate-id leak
  remap) **∧** failure-branches-on-tape **∧** integer cost-accounting reconstructable. Reported as *"accountable
  agent-governance substrate,"* explicitly NOT *"a cleverer/cheaper solver."*

## 3. Phased execution (each phase gated by the prior's exit)

### Phase 0 — TP-0A + TP-1 + epsilon-floor (concurrent, no counted run)
- **TP-0A — MarketTape-lite PPUT Replay Gate.** Promote in-bin `MarketTape` (`lean_hayek_market.rs:107-170`) to a
  **bin-local shared module via `#[path]`** (NOT `pub mod` in `lib.rs` — that is a trust-root/constitution touch
  per memory). Schema-aware `verify_market_tape` reconstructs banked/tokens/micro_usd for ONE pinned reference arm
  (`run_alloc`/`pool:`/`lean_hayek_alloc.v2`) + a het4 wallet/PnL arm. GenesisPin mandatory-first;
  `derive_cost` recomputes from `LlmCall`+`MODEL_RATES` (never reads manifest); `FailedProposal{verified:false}`
  on tape; one-byte tamper ⇒ non-zero exit. **No §6, no lib.rs, no real-git Q_t.**
  **Exit gate G0:** `verify_market_tape` exit 0 on the pinned arm; negative tests (genesis-first, tamper,
  failed-branch, derive-not-read) all pass-and-can-fail; `cargo test --workspace` + constitution gates green.
- **TP-1 — 5-pillar microstructure spec lock (doc/test/prereg only).** asset / oracle / honest-bidding /
  goodhart-shield / exploration, each a code-cited prereg pillar + a conformance predicate. **Goodhart = HARD
  gate:** reuse the existing green price-blindness anchor (`tb_14_halt_triggers::price_does_not_affect_predicate_result`)
  + spend the new test budget on the **predicate-id leak remap (11 sites)**. **Exploration floor =
  run-validity gate** (see §4 refinement #1). **Exit gate G1:** all five conformance predicates compile + static
  ones pass; prereg JSON frozen + externally SHA-pinned.
- **epsilon-floor (TP-1.5, Class-1, decoupled):** add `MIN_EPSILON` const + a **harness run-validity gate** that
  rejects/records epsilon below floor and writes epsilon to tape. Does NOT change `from_env`'s fail-soft (would
  break the deliberate epsilon=0 determinism tests). Independent small commit.

### Phase 2 — TP-2 SMOKE only (after G0 ∧ G1; no headline)
6 arms — **single · market · shuffled-price · flatbid · coordinator-concat · coordinator-synth-LLM** — on the
**8 combination-targets × 2 seeds**, ALL on the SAME authoritative MarketTape-lite substrate, equal reasoner-token
budget. **Exit gate G2:** every arm same-substrate + replay-GREEN, budget binds for the coordinators (decomposer +
workers + synth all in one accumulator), dynamic range present, `derive_dag` reconstructs subtask DAG.

### Phase 3 — TP-2 COUNTED (after G2)
8-subset PRIMARY, 44-pool NULL-CONFIRMATION. Compute Verdict A + Verdict B. ≥12 seeds, interleaved, Wilcoxon+Holm.
n=8 power handled per §4 refinement #2 (MDE pre-registered; subset expanded if feasible; a subset null is
"underpowered/inconclusive," not auto-NO-GO).

### Phase 4 — later
TP-3 prereg skeleton + frozen-competence-matrix design may proceed in parallel after G1, but **no counted
meta-market** until Phase-3 green. TP-4 stays parked until task #28 lands a COMPLETE Stage-2 Q1 verdict.

## 4. Refinements folded in (architect's corrections + this session's code-grounded additions)

**Architect's corrections (accepted verbatim):** two-level verdict split; mission reframe (governance primary,
efficiency falsifiable); TP-0A rename + bin-local `#[path]` + no-lib.rs; `MARKET > FLATBID` (directional, not
`≠`); two coordinators (concat = minimal central-planning baseline, synth-LLM = the realistic Kimi/Grok-class
baseline that must be beaten); single MarketTape-lite substrate for TP-2 v1 (no g1 ChainTape mixing); TP-3
headline = Sybil-defunding/calibration; TP-4 park until Stage-2 Q1.

**This session's refinements (code-grounded):**
1. **Epsilon floor mechanism** — do NOT clamp `price_index.rs::from_env`: `epsilon=0` is deliberately used for
   determinism in `src/sdk/actor.rs:203/225/279`, `tests/fc_alignment_conformance.rs:657`,
   `tests/tb_14_canonical_masking_smoke.rs:379/565`, and a `from_env` test at `price_index.rs:1104`. A `from_env`
   clamp would break those AND miss the struct-literal paths. Instead: `MIN_EPSILON` const + an
   **experiment-harness run-validity gate** (reject + tape-record). Keeps fail-soft intact.
2. **n=8 statistical power** — the decomposable subset is only 8 theorems; a +1/+2 difference may not reach
   p<0.05. Pre-register a **minimum detectable effect (MDE)** and either expand the curated decomposable-target
   set, or pre-commit that a subset null is **"underpowered/inconclusive," not NO-GO**. (NO-GO requires
   MARKET ≈ SHUFFLE with adequate power.)
3. **Explicit null-prereg** — pre-commit, in writing, that the 44-pool efficiency result is EXPECTED NULL (the
   repo's prior). The efficiency claim is tested ONLY on the decomposable subset, so a 44-pool null cannot be
   spun and a subset result cannot be retconned.
4. **MARKET > SHUFFLE is the PRIMARY causal gate** (flatbid secondary) — shuffle preserves the price
   distribution and only permutes assignment, so beating it is the load-bearing evidence that the price
   *ordering* (not mere dispersion) carries causal signal.

## 5. Hard rules (every phase)

No counted run before its gate. No §6 surface edit without per-atom §8 + PRE-§8 clean-context audit
(`sequencer.rs`/`typed_tx.rs`/`wallet.rs`/`kernel.rs`/`bus.rs`/`cas/schema.rs`). FC1/FC2/FC3 byte-identical.
Integer money only. PR-only. Every prereg JSON locked + externally SHA-pinned before any arm result is read.
`verify_market_tape` exit≠0 ⇒ arm EXCLUDED from headlines (pre-committed). Historical tapes re-verified, never
rewritten.

## 6. Open decisions (need the architect before Phase 3)

- **D1 — budget basis.** Equal **reasoner-TOKENS** is the only enforceable knob; micro_usd is a DERIVED report
  (equal-dollar across heterogeneous rates is unsatisfiable as a gate). Proposed: budget = equal reasoner tokens;
  cost-of-pass (dollars) reported as a secondary, not gated. **Confirm.**
- **D2 — decomposable-set size.** Expand the curated decomposable-target set beyond the current 8 (more power),
  or accept n=8 with a pre-registered MDE + "inconclusive-not-NO-GO" on a subset null? **Decide before Phase 3.**

## 7. Immediate next actions (Phase 0, authorized now)

1. TP-0A.1 — extract `MarketTape`/`MarketEvent` to `src/bin/market_tape_shared.rs`, re-point the bin via
   `#[path]`; behavior-preserving (existing P4-lite/LEAN-ALLOC tapes still verify-green).
2. TP-0A.2 — GenesisPin mandatory-first + `head_commit_sha` surrogate.
3. TP-1 — draft the 5-pillar prereg JSON + conformance test skeleton; the `MIN_EPSILON` const + run-validity-gate
   design.
(Concurrent; neither reads a counted result. D1/D2 do not block Phase 0.)
