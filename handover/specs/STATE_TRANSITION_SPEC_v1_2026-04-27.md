# State Transition Specification v1.1

> **Date**: 2026-04-27 (v1.1 patch applies SPEC_WALKTHROUGH gap fixes)
> **Patch v1 → v1.1 changes**:
> - § 3.2 (challenge_transition) stage 4e ADDED: verifier_bond release policy (default = return to verifier; configurable)
> - § 3.3 (reuse_transition) stage 3 AMENDED: edge weight bounded by `MAX_REUSE_ROYALTY_FRACTION` config (default = 0.10)
> - § 3.2 (challenge_transition) stage 4d AMENDED: false-challenge reputation penalty config (default = 0; configurable)
> - § 3.1 (verify_transition) note ADDED: quorum-aggregation rule placeholder (default = 1; configurable)
> - § 4 invariants ADDED: I-VBOND-RELEASE / I-ROYALTY-CAP
> - § 11 (Found Inconsistencies) — promoted from SPEC_WALKTHROUGH § 11
>
> All 4 walk-through gaps now have either (a) machine-checkable default applied, or (b) explicit deferral with target atom.
>
> **Purpose**: D-VETO-1 binding form. Defines `step_transition: (Q_t, tx_i) → (Q_{t+1}, signals_t)` with typed schemas, deterministic pseudocode, named invariants, conformance test list. Gates CO1.1.4/CO1.1.5 bus.rs/kernel.rs split (per Plan v3.2 atom CO1.SPEC.0).
>
> **Authority**: Constitution Art. 0–0.4 + white paper architecture § 3-7 + economic § 2/§ 6/§ 18-21. Where this spec disagrees with white paper, **white paper wins** and this spec must be amended.
>
> **Audit**: Codex CO P0.7 T+S review (2026-04-27) demanded binding spec form before refactor. This document is the response.

---

## § 0 Scope

**In scope**:
- The single-step state transition function `step_transition` for object-level work_tx
- Typed `QState`, `WorkTx`, `VerifyTx`, `ChallengeTx`, `RejectedAttemptSummary`, `TerminalSummaryTx` schemas
- Hidden-input classification: which existing `bus.rs`/`kernel.rs` inputs are `Q_t`, which are `tx_i`, which are illegal side effects
- Named invariants enforceable mechanically
- Conformance test list generated from the spec

**Out of scope** (handled separately):
- `MetaTx` schema for runtime meta-transitions — defined as **stub only** here; full schema deferred to v4.1 per D-VETO-4 = B (defer, not abandon)
- AttributionEngine DAG construction algorithm — deferred to CO2.4.0 spike (Inv 8 design)
- Full predicate visibility air-gap proof — deferred to CO P1.5 (Goodhart shield design)

---

## § 1 Typed Schemas

### 1.1 QState (white paper § 4 + economic § 2 amendment, 9 fields)

```rust
pub struct QState {
    /// Agent swarm sub-state: tape head per agent, per-agent reputation snapshots, etc.
    /// MUST be reconstructible from L4 transition ledger replay.
    pub q_t: AgentSwarmState,

    /// Current ChainTape head pointer = git commit SHA in Path B substrate.
    pub head_t: NodeId,

    /// Materialized state Merkle root (git tree root in Path B).
    pub state_root_t: Hash,

    /// Agent-visible projection of tape filtered by per-agent visibility policy
    /// (Inv 10 Goodhart shield). Derived from L1 PredicateRegistry visibility tags.
    pub tape_view_t: AgentVisibleProjection,

    /// L4 Transition Ledger root (Merkle root of all accepted tx so far).
    pub ledger_root_t: Hash,

    /// L1 Predicate Registry root.
    pub predicate_registry_root_t: Hash,

    /// L2 Tool Registry root.
    pub tool_registry_root_t: Hash,

    /// Economic state (economic § 2 amendment, 9 sub-fields).
    pub economic_state_t: EconomicState,

    /// Global budget snapshot: cost ceiling, wall clock, compute cap.
    pub budget_state_t: BudgetSnapshot,
}

pub struct AgentSwarmState {
    pub agents: BTreeMap<AgentId, PerAgentState>,
    pub current_round: u64,
}

pub struct PerAgentState {
    pub reputation_snapshot: Reputation,
    pub last_accepted_tx: Option<TxId>,
    pub retry_counter_for_current_task: u32,  // resets on accept; persists across rejections
}

pub struct EconomicState {
    pub balances_t:       BalancesIndex,
    pub escrows_t:        EscrowsIndex,
    pub stakes_t:         StakesIndex,
    pub claims_t:         ClaimsIndex,
    pub reputations_t:    ReputationsIndex,
    pub task_markets_t:   TaskMarketsIndex,
    pub royalty_graph_t:  RoyaltyGraph,
    pub challenge_cases_t: ChallengeCasesIndex,
    pub price_index_t:    PriceIndex,
}
```

**BTreeMap, not HashMap, everywhere**: deterministic iteration order for replay byte-identity (Codex flagged kernel.rs:187-204 HashMap nondeterminism).

### 1.2 WorkTx (12 fields per WP § 5.L4)

```rust
pub struct WorkTx {
    pub tx_id: TxId,                              //  1
    pub task_id: TaskId,                          //  2  links to TaskMarket entry
    pub parent_state_root: Hash,                  //  3  must equal Q_t.state_root_t at submission
    pub agent_id: AgentId,                        //  4
    pub read_set: BTreeSet<ReadKey>,              //  5  agent MUST declare read deps (DAG attribution)
    pub write_set: BTreeSet<WriteKey>,            //  6  agent MUST declare write targets
    pub proposal_cid: Cid,                        //  7  L3 CAS handle to payload (not raw payload)
    pub predicate_results: PredicateResultsBundle,//  8  filled BY runner, not by agent
    pub stake: StakeMicroCoin,                    //  9  YES_E stake, i64 micro-coin units
    pub signature: AgentSignature,                // 10
    pub timestamp_logical: u64,                   // 11  monotonic counter from runtime, NOT wall clock
    pub status: TxStatus,                         // 12  Pending | Accepted | Rejected(class) | Finalized
}

pub enum TxStatus {
    Pending,
    Accepted,
    Rejected(RejectionClass),
    FinalizedReward(MicroCoin),
    FinalizedSlash(SlashEvidenceCid),
}

pub struct PredicateResultsBundle {
    pub acceptance: BTreeMap<PredicateId, BoolWithProof>,
    pub settlement: BTreeMap<PredicateId, BoolWithProof>,
    pub safety_class: SafetyOrCreation,  // determines fail-closed vs fail-open-with-signal
}
```

### 1.3 VerifyTx, ChallengeTx, ReuseTx (economic § 13)

```rust
pub struct VerifyTx {
    pub tx_id: TxId,
    pub target_work_tx: TxId,         // the work_tx being verified
    pub verifier_agent: AgentId,
    pub bond: StakeMicroCoin,         // verifier reputation/bond stake
    pub verdict: VerifyVerdict,       // Confirm | Doubt
    pub signature: AgentSignature,
    pub timestamp_logical: u64,
}

pub struct ChallengeTx {
    pub tx_id: TxId,
    pub target_work_tx: TxId,
    pub challenger_agent: AgentId,
    pub stake: StakeMicroCoin,        // NO_E stake, i64 micro-coin
    pub counterexample_cid: Cid,      // L3 CAS handle to counterexample
    pub signature: AgentSignature,
    pub timestamp_logical: u64,
}

pub struct ReuseTx {
    pub tx_id: TxId,
    pub reusing_work_tx: TxId,        // the work_tx that triggered the reuse
    pub reused_tool_id: ToolId,       // L2 Tool Registry handle
    pub reused_tool_creator: AgentId, // royalty recipient
    pub timestamp_logical: u64,
}
```

### 1.4 RejectedAttemptSummary (D-VETO-6 system-stamped, NOT agent self-report)

```rust
pub struct RejectedAttemptSummary {
    pub failed_attempts_since_last_accept: u32,           // bounded, capped at u32::MAX
    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,  // counts only, no payloads
    pub first_failure_logical_t: Option<u64>,             // for time-to-first-fail signal
    pub last_failure_logical_t: Option<u64>,              // for recency signal
    // NO raw error strings, NO agent payload contents, NO predicate internal traces
}

pub enum RejectionClass {
    AcceptancePredicateFail(PredicateId),     // public predicates only; private predicates → Opaque
    SettlementPredicateFail(PredicateId),
    StakeInsufficient,
    SignatureInvalid,
    StaleParentRoot,                          // Q_t advanced; agent's view stale
    Opaque,                                   // private predicate failure; classification withheld
    BudgetExceeded,
}
```

`RejectedAttemptSummary` is stamped **by the white-box predicate runner** onto the next accepted `WorkTx`. Trust boundary: the runner generates this summary; the agent does NOT self-report. Verified at conformance test level.

### 1.5 TerminalSummaryTx (no-accept run handler)

```rust
pub struct TerminalSummaryTx {
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub run_id: RunId,
    pub run_outcome: RunOutcome,           // OmegaAccepted | MaxTxExhausted | WallClockCap | ComputeCap | ErrorHalt
    pub total_attempts: u32,
    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,
    pub last_logical_t: u64,
    pub system_signature: SystemSignature,  // signed by runtime keypair, not by any agent
}
```

If a run terminates without any accepted work_tx, the runtime emits exactly one `TerminalSummaryTx` to L4. This preserves L6 reconstructibility: error class signal is derivable from tape even if no work_tx ever passed.

### 1.6 MetaTx (stub for v4.1; v4 only emits `MetaProposalDraft` to L3 CAS, not L4)

```rust
pub struct MetaTx {
    pub tx_id: TxId,
    pub parent_architecture_root: Hash,
    pub proposed_predicate_patches: Vec<PredicatePatch>,
    pub proposed_tool_patches:      Vec<ToolPatch>,
    pub log_evidence_cids:           Vec<Cid>,
    pub reversibility_plan_cid:      Cid,
    pub constitution_check:          ConstitutionCheckProof,
    pub judge_signatures:            Vec<JudgeSignature>,
    pub human_signature_required:    bool,
    pub human_signature:             Option<HumanSignature>,
}
```

**v4 status**: MetaTx schema reserved; runtime ArchitectAI/JudgeAI **NOT implemented**. v4 produces `MetaProposalDraft` (a CAS object) only, written when ArchitectAI proposes architecture amendments via the cp workflow. v4.1 implements the runtime actor + L4 acceptance.

This is the D-VETO-4 = B (defer, not abandon) implementation.

---

## § 2 Hidden-Input Classification (Codex § C demanded)

The current `src/bus.rs` and `src/kernel.rs` mix four categories of inputs. The spec must classify each:

| Input | Current source | T+S classification | New home in step_transition |
|---|---|---|---|
| `created_at` (wall clock seconds) | `bus.rs:264-268` `SystemTime::now()` | **ILLEGAL hidden side effect** | retire; replace with `timestamp_logical: u64` from runtime monotonic counter |
| `completion_tokens: 0` literal | `bus.rs:268` | **ILLEGAL hidden zero** | kill in CO1.1.4-pre1; read real value from LLM `usage.completion_tokens` |
| `TAPE_ECONOMY_V2` env var | `bus.rs:298, 345` | **`Q_t.budget_state_t.feature_flags`** | promote to typed field; tx must reference flag value at parent_state_root |
| `FOUNDER_GRANT_GAMMA` env var | `bus.rs:307` | **`Q_t.economic_state_t.task_markets_t.config.founder_grant_gamma`** | promote to typed field; bound at task creation, not env at runtime |
| `self.config.system_lp_amount` | `bus.rs:340` | **`Q_t.economic_state_t.task_markets_t.config.system_lp_amount`** | promote |
| `self.clock` counter | `bus.rs:42` | **`Q_t.q_t.current_round` derived** | derive from L4 ledger length; not separately tracked |
| `self.tx_count` | `bus.rs:42` | **`Q_t.q_t.current_round` derived** | derive |
| `self.generation` | `bus.rs:42` | **`Q_t.q_t.generation` typed field** | promote |
| `self.graveyard: HashMap<String, Vec<String>>` | `bus.rs:48` | **ILLEGAL sidecar** (Art. 0.2 explicitly anti-patterned) | retire; replace with `RejectedAttemptSummary` stamped on next accepted tx + `TerminalSummaryTx` |
| Tool list iteration order | `bus.rs:312-319` Vec | **`Q_t.tool_registry_root_t` derived** | runner queries L2 in deterministic order |
| Wallet "magic search" | `bus.rs:312-319` `manifest() == "wallet"` | **EXPLICIT capability lookup** | runner queries L2 by `Capability::EconomicWallet` tag, not by string match |

After this classification, every step_transition input is either part of `Q_t`, part of `tx_i`, or part of the runtime config bound at genesis (which is itself in `Q_t`).

**Conformance test for § 2** (`tests/no_hidden_inputs.rs`):
- grep src/ for `SystemTime::now()` → must return 0 hits in non-runtime-bootstrap code
- grep src/ for `std::env::var(` → must return 0 hits in step_transition path
- grep src/ for `HashMap` in any module containing `q_state` or `transition` → must return 0 hits
- assert all monetary fields are typed `MicroCoin` (a newtype around `i64`), no `f64`

---

## § 3 step_transition (Deterministic Pseudocode)

```rust
/// Pure function. Same (Q_t, tx_i) → byte-identical (Q_{t+1}, signals_t).
/// No I/O. No env reads. No clock reads. No randomness without seed in tx_i.
pub fn step_transition(
    q: &QState,
    tx: &WorkTx,
    registry: &PredicateRegistry,
    tool_registry: &ToolRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {

    // STAGE 1: parent_state_root match (stale view rejection)
    if tx.parent_state_root != q.state_root_t {
        return Err(TransitionError::StaleParent {
            expected: q.state_root_t,
            got:      tx.parent_state_root,
        });
        // NB: rejection here does NOT change Q_t; runner stamps RejectedAttemptSummary
        // onto the NEXT accepted tx (or onto TerminalSummaryTx if run ends without accept)
    }

    // STAGE 2: signature verification
    if !verify_signature(&tx.signature, tx.canonical_digest()) {
        return Err(TransitionError::SignatureInvalid);
    }

    // STAGE 3: stake availability (Inv 5 — YES_E event-bound)
    let agent_balance = q.economic_state_t.balances_t.get(&tx.agent_id);
    if agent_balance < tx.stake {
        return Err(TransitionError::StakeInsufficient { available: agent_balance, required: tx.stake });
    }

    // STAGE 4: predicate gate (Inv 6 — predicate-gated transition)
    let acceptance_results = registry.run_acceptance(tx, q)?;
    let safety_class = registry.classify(tx);
    match (safety_class, acceptance_results.all_passed()) {
        (SafetyOrCreation::Safety, false) => {
            return Err(TransitionError::AcceptancePredicateFailed(acceptance_results));
            // fail-closed for Safety (WP § 7.2)
        }
        (SafetyOrCreation::Creation, false) => {
            // fail-open-with-signal: still reject, but emit informational signal (no Q_t change)
            return Err(TransitionError::AcceptancePredicateFailed(acceptance_results));
        }
        _ => {}  // passed; continue
    }

    // STAGE 5: provisional reward issue (Inv 7 — provisional then final)
    let claim = ClaimId::derive(tx.tx_id);
    let provisional_reward = SettlementEngine::issue_provisional(
        claim,
        &q.economic_state_t.escrows_t,
        tx.task_id,
    )?;

    // STAGE 6: state transition apply (deterministic)
    let mut q_next = q.clone();
    q_next.economic_state_t.claims_t.insert(claim, provisional_reward);
    q_next.economic_state_t.stakes_t.lock(tx.agent_id, tx.task_id, tx.stake);
    q_next.economic_state_t.balances_t.debit(tx.agent_id, tx.stake);
    q_next.q_t.update_per_agent(tx.agent_id, |s| {
        s.last_accepted_tx = Some(tx.tx_id);
        s.retry_counter_for_current_task = 0;  // reset on accept
    });

    // L4 append
    let new_ledger_root = ledger::append(&q.ledger_root_t, tx);
    q_next.ledger_root_t = new_ledger_root;

    // L5 materialize
    let new_state_root = materializer::apply(&q.state_root_t, tx);
    q_next.state_root_t = new_state_root;

    // L6 signal emit (broadcast price + reputation; NOT evaluator internals — Inv 10)
    let signals = SignalBundle {
        boolean: vec![Signal::Boolean(BoolSignal::AcceptedAt(tx.tx_id))],
        statistical: vec![
            Signal::Statistical(StatSignal::PriceUpdate(price_for(tx.task_id, q_next.economic_state_t.price_index_t))),
            Signal::Statistical(StatSignal::ReputationDelta(tx.agent_id, +reputation_delta(tx))),
        ],
    };

    // STAGE 7: head advance
    q_next.head_t = NodeId::from_state_root(new_state_root);

    // STAGE 8: challenge window open (Inv 7 — finalization is deferred)
    q_next.economic_state_t.challenge_cases_t.open(claim, tx.timestamp_logical, CHALLENGE_WINDOW_TICKS);

    Ok((q_next, signals))
}
```

**No wall-clock, no env-var, no HashMap iteration**. Every input is either `q`, `tx`, or registries (themselves in `q.predicate_registry_root_t` / `q.tool_registry_root_t`).

### 3.1 verify_transition (VerifyTx)

Per Gemini v3.2 review Q10 VETO — extending pseudocode to all state-mutating tx types.

> **v1.1 note (gap 11.4)**: this pseudocode handles ONE verifier per tx. Multi-verifier quorum aggregation is a TaskMarket config (`verifier_quorum_required: usize` default = 1). When N>1 verifiers each submit verify_tx for the same target_work_tx, claim transitions to `Pending → ApprovedByVerifiers` only after `verifier_quorum_required` distinct verifiers have submitted `Confirm`. Aggregation rule deferred to CO P2.7 atom (Verifier role detail). For v4 default (quorum=1), each verify_tx independently advances claim to ApprovedByVerifiers.

```rust
pub fn verify_transition(
    q: &QState,
    tx: &VerifyTx,
    registry: &PredicateRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {

    // STAGE 1: target work_tx must exist + be in Pending or Provisional state
    let target = q.economic_state_t.claims_t.get(&tx.target_work_tx)
        .ok_or(TransitionError::TargetWorkTxNotFound)?;
    if !target.status.allows_verification() {
        return Err(TransitionError::TargetWorkTxNotVerifiable);
    }

    // STAGE 2: signature + bond
    if !verify_signature(&tx.signature, tx.canonical_digest()) {
        return Err(TransitionError::SignatureInvalid);
    }
    let verifier_balance = q.economic_state_t.balances_t.get(&tx.verifier_agent);
    if verifier_balance < tx.bond {
        return Err(TransitionError::StakeInsufficient);
    }

    // STAGE 3: predicate gate (verifier predicate, NOT same as work_tx acceptance)
    let verify_results = registry.run_verification(tx, target, q)?;
    if !verify_results.all_passed() {
        return Err(TransitionError::VerificationPredicateFailed(verify_results));
    }

    // STAGE 4: state transition
    let mut q_next = q.clone();
    q_next.economic_state_t.balances_t.debit(tx.verifier_agent, tx.bond);
    q_next.economic_state_t.stakes_t.lock_verifier_bond(tx.verifier_agent, tx.target_work_tx, tx.bond);
    q_next.economic_state_t.claims_t.add_verification(tx.target_work_tx, tx.verifier_agent, tx.verdict);

    // STAGE 5: append + materialize + signals
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    let signals = SignalBundle {
        boolean: vec![Signal::Boolean(BoolSignal::VerifiedAt(tx.tx_id))],
        statistical: vec![Signal::Statistical(StatSignal::ReputationDelta(tx.verifier_agent, +verify_reputation_delta(tx, target)))],
    };

    Ok((q_next, signals))
}
```

### 3.2 challenge_transition (ChallengeTx)

```rust
pub fn challenge_transition(
    q: &QState,
    tx: &ChallengeTx,
    registry: &PredicateRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {

    // STAGE 1: target work_tx must exist + still in challenge window
    let target = q.economic_state_t.claims_t.get(&tx.target_work_tx)
        .ok_or(TransitionError::TargetWorkTxNotFound)?;
    let window = q.economic_state_t.challenge_cases_t.get(tx.target_work_tx)
        .ok_or(TransitionError::ChallengeWindowClosed)?;
    if tx.timestamp_logical >= window.opens_at + window.duration_ticks {
        return Err(TransitionError::ChallengeWindowClosed);
    }

    // STAGE 2: signature + NO_E stake
    if !verify_signature(&tx.signature, tx.canonical_digest()) {
        return Err(TransitionError::SignatureInvalid);
    }
    let challenger_balance = q.economic_state_t.balances_t.get(&tx.challenger_agent);
    if challenger_balance < tx.stake {
        return Err(TransitionError::StakeInsufficient);
    }

    // STAGE 3: counterexample acceptance predicate (the BURDEN OF PROOF predicate, Inv 7)
    let counterexample = cas::get(&tx.counterexample_cid)?;
    let counter_check = registry.run_counterexample_check(target, &counterexample, q)?;
    if !counter_check.proves_violation() {
        return Err(TransitionError::CounterexampleInsufficient(counter_check));
    }

    // STAGE 4: state transition — ROLLBACK target work_tx + slash original solver + reward challenger
    let mut q_next = q.clone();
    q_next.economic_state_t.balances_t.debit(tx.challenger_agent, tx.stake);

    // 4a: rollback target's provisional reward
    let rollback_amount = q.economic_state_t.claims_t.provisional_amount(tx.target_work_tx);
    q_next.economic_state_t.claims_t.mark_slashed(tx.target_work_tx, tx.tx_id);

    // 4b: slash original solver's stake → reward pool for challenger
    let solver_stake = q.economic_state_t.stakes_t.get(target.solver, target.task_id);
    q_next.economic_state_t.stakes_t.slash(target.solver, target.task_id);
    q_next.economic_state_t.escrows_t.deposit_from_slash(tx.challenger_agent, solver_stake);

    // 4c: challenger gets back NO_E stake + slashed solver stake
    q_next.economic_state_t.balances_t.credit(tx.challenger_agent, tx.stake + solver_stake);

    // 4d: solver reputation -= delta; challenger reputation += delta (Inv 9 immutable but we update via formula not transfer)
    q_next.economic_state_t.reputations_t.adjust(target.solver, -slash_reputation_delta());
    q_next.economic_state_t.reputations_t.adjust(tx.challenger_agent, +challenge_reputation_delta());

    // 4e: verifier_bond release per task config (gap 11.2 fix; default = return to good-faith verifier)
    //   Rationale: when Carol slashes Alice via challenge, Bob (the verifier) was duped but acted in good faith.
    //   Slashing Bob's bond would discourage future verification. Configurable per TaskMarket.
    //   Applies to ALL verifiers who voted Confirm on the slashed work_tx.
    let bond_release_policy = q.economic_state_t.task_markets_t
        .get(target.task_id)
        .map(|tm| tm.config.verifier_bond_on_slash)
        .unwrap_or(VerifierBondPolicy::ReturnToVerifier);
    for (verifier, bond) in q.economic_state_t.stakes_t.verifier_bonds_for(tx.target_work_tx) {
        match bond_release_policy {
            VerifierBondPolicy::ReturnToVerifier => {
                q_next.economic_state_t.balances_t.credit(verifier, bond);
                q_next.economic_state_t.stakes_t.release_verifier_bond(verifier, tx.target_work_tx);
            }
            VerifierBondPolicy::SlashedToChallenger => {
                q_next.economic_state_t.balances_t.credit(tx.challenger_agent, bond);
                q_next.economic_state_t.stakes_t.slash_verifier_bond(verifier, tx.target_work_tx);
                q_next.economic_state_t.reputations_t.adjust(verifier, -verifier_slash_delta());
            }
        }
    }

    // STAGE 5: close challenge window
    q_next.economic_state_t.challenge_cases_t.close(tx.target_work_tx, ChallengeOutcome::Slashed(tx.tx_id));

    // STAGE 6: append + materialize + signals
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    let signals = SignalBundle {
        boolean: vec![Signal::Boolean(BoolSignal::ChallengeUpheld(tx.tx_id))],
        statistical: vec![
            Signal::Statistical(StatSignal::ReputationDelta(target.solver, -slash_reputation_delta())),
            Signal::Statistical(StatSignal::ReputationDelta(tx.challenger_agent, +challenge_reputation_delta())),
        ],
    };

    Ok((q_next, signals))
}
```

### 3.3 reuse_transition (ReuseTx)

```rust
pub fn reuse_transition(
    q: &QState,
    tx: &ReuseTx,
    tool_registry: &ToolRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {
    // STAGE 1: tool must be registered + still active in L2
    let tool = tool_registry.get(tx.reused_tool_id)
        .ok_or(TransitionError::ToolNotInRegistry)?;
    if tool.creator != tx.reused_tool_creator {
        return Err(TransitionError::ToolCreatorMismatch);
    }

    // STAGE 2: parent reusing_work_tx must exist + Accepted
    let parent = q.economic_state_t.claims_t.get(&tx.reusing_work_tx)
        .ok_or(TransitionError::TargetWorkTxNotFound)?;
    if !parent.status.is_accepted_or_finalized() {
        return Err(TransitionError::ParentNotAcceptedYet);
    }

    // STAGE 3: state transition — add edge to royalty graph
    //   gap 11.3 fix: weight bounded by MAX_REUSE_ROYALTY_FRACTION = 0.10 default
    //   Rationale: 10% upper bound protects solver's primary reward. Builders earn via creating
    //   widely-reusable tools, not via single high-percentage extractions. Configurable per TaskMarket
    //   for cases where user wants to override (e.g., creator-economy experiments).
    let max_royalty = q.economic_state_t.task_markets_t
        .get(parent.task_id)
        .and_then(|tm| tm.config.max_reuse_royalty_fraction)
        .unwrap_or(MAX_REUSE_ROYALTY_FRACTION_DEFAULT);  // = 0.10 in micro-coin fractional repr (10000 / 100000)
    let bounded_weight = tool.reuse_royalty_share.min(max_royalty);
    if tool.reuse_royalty_share > max_royalty {
        log::warn!(
            "reuse_tx {}: tool {} declared royalty {} > max {}; clamping to {}",
            tx.tx_id, tx.reused_tool_id, tool.reuse_royalty_share, max_royalty, bounded_weight
        );
    }

    let mut q_next = q.clone();
    q_next.economic_state_t.royalty_graph_t.add_edge(
        from: tx.reusing_work_tx,
        to:   tx.reused_tool_id,
        creator: tx.reused_tool_creator,
        weight: bounded_weight,    // clamped per gap 11.3
    );

    // STAGE 4: append + materialize (no signals; royalty paid at finalize_reward time)
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    Ok((q_next, SignalBundle::empty()))
}
```

### 3.4 finalize_reward (challenge window expiry)

Triggered by tick (no agent submits this; runtime emits when challenge window expires for any provisional claim).

```rust
pub fn finalize_reward_transition(
    q: &QState,
    claim_id: ClaimId,
    settlement_engine: &SettlementEngine,
) -> Result<(QState, SignalBundle), TransitionError> {
    let claim = q.economic_state_t.claims_t.get(&claim_id)
        .ok_or(TransitionError::ClaimNotFound)?;
    let window = q.economic_state_t.challenge_cases_t.get(claim.target_work_tx);

    // STAGE 1: window must be expired AND no open slash
    if let Some(w) = window {
        if w.is_open() {
            return Err(TransitionError::ChallengeWindowStillOpen);
        }
        if w.outcome == Some(ChallengeOutcome::Slashed(_)) {
            return Err(TransitionError::AlreadySlashed);  // never finalize a slashed claim
        }
    }

    // STAGE 2: compute reward per Economic § 21 final formula
    let reward = settlement_engine.finalize(
        claim,
        Escrow::lookup(q, claim.task_id),
        Attribution::lookup(q, claim.target_work_tx),
        Survival::full,  // window expired without slash
        Utility::lookup(q, claim.target_work_tx),
        Constitution::check(q),
    )?;

    // STAGE 3: state transition
    let mut q_next = q.clone();
    let target = claim.target_work_tx_data;
    q_next.economic_state_t.balances_t.credit(target.solver, reward);
    q_next.economic_state_t.claims_t.finalize(claim_id, reward);
    q_next.economic_state_t.escrows_t.debit(claim.task_id, reward);

    // pay royalties to tool creators along royalty_graph_t edges
    for edge in q.economic_state_t.royalty_graph_t.edges_from(claim.target_work_tx) {
        let royalty = reward * edge.weight;
        q_next.economic_state_t.balances_t.credit(edge.creator, royalty);
        q_next.economic_state_t.balances_t.debit(target.solver, royalty);  // royalty comes from solver's reward, not extra mint (Inv 4)
    }

    // STAGE 4: emit terminal signals
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, &FinalizeTx::from(claim_id, reward));
    q_next.state_root_t  = materializer::apply(&q.state_root_t, &FinalizeTx::from(claim_id, reward));
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    Ok((q_next, SignalBundle::finalize(claim_id, reward)))
}
```

### 3.5 emit_terminal_summary (run-end without acceptance)

```rust
pub fn emit_terminal_summary_transition(
    q: &QState,
    run_id: RunId,
    runtime: &Runtime,
) -> Result<(QState, SignalBundle), TransitionError> {
    let run = runtime.run_state(run_id)?;
    if run.has_accepted_work_tx() {
        return Err(TransitionError::TerminalSummaryNotApplicable);  // only emitted for no-accept runs
    }

    let summary = TerminalSummaryTx {
        tx_id: TxId::derive(run_id, "terminal"),
        task_id: run.task_id,
        run_id,
        run_outcome: run.outcome(),
        total_attempts: run.attempt_counter(),
        failure_class_histogram: run.failure_histogram(),
        last_logical_t: run.last_logical_t(),
        system_signature: runtime.system_keypair().sign(canonical_digest_terminal(run)),
    };

    // STAGE: append; materialize; emit failure-class signals to L6
    let mut q_next = q.clone();
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, &summary);
    q_next.state_root_t  = materializer::apply(&q.state_root_t, &summary);
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    let signals = SignalBundle::terminal_summary(&summary);

    Ok((q_next, signals))
}
```

---

## § 4 Named Invariants (machine-checkable)

| ID | Invariant | Enforced at | Conformance test |
|---|---|---|---|
| I-DET | Same (Q_t, tx) → byte-identical (Q_{t+1}, signals) | step_transition stage 6-8 | `tests/transition_determinism.rs` |
| I-DETHASH | `state_root_t` after replay from genesis matches authoritative state | replay test | `tests/q_state_reconstruct.rs` |
| I-NOSIDE | step_transition reads only (q, tx, registries); no I/O | static analysis grep + cargo-deny | `tests/no_hidden_inputs.rs` |
| I-PARENT | tx.parent_state_root must equal q.state_root_t | stage 1 | `tests/stale_parent_rejection.rs` |
| I-SIG | tx.signature verifies against tx.canonical_digest() | stage 2 | `tests/signature_verification.rs` |
| I-STAKE | tx.stake ≤ q.balances_t[tx.agent_id]; debit atomic | stage 3, 6 | `tests/stake_atomicity.rs` |
| I-PRED-GATE | rejected work_tx does NOT advance state_root_t | stage 4 | `tests/economic_invariant_INV6_predicate_gated.rs` |
| I-PROV | accepted work_tx → provisional claim, NOT finalized reward | stage 5 | `tests/economic_invariant_INV7_provisional_then_final.rs` |
| I-LOGTIME | timestamp_logical strictly monotonic per-tx; no wall clock | stage 6 | `tests/no_wall_clock_in_tx.rs` |
| I-MICROCOIN | all monetary fields are MicroCoin (i64 newtype) | type system | compile-time + `tests/no_f64_money.rs` |
| I-BTREE | Q_t indices use BTreeMap, not HashMap (deterministic order) | type system | `tests/q_state_uses_btree.rs` |
| I-NOSIDECAR | no Vec/HashMap "graveyard"-like sidecar (Art. 0.2) | static analysis | `tests/no_rejection_sidecar.rs` |
| I-RETRY | RejectedAttemptSummary stamped by runner, not agent | stamp call site | `tests/retry_summary_runner_signed.rs` |
| I-TERMINAL | every run terminates with at least one of: accepted work_tx OR TerminalSummaryTx | run finalize hook | `tests/run_terminal_invariant.rs` |
| I-NOENV | step_transition dependency tree contains no `std::env` access | cargo-deny + grep | `tests/no_env_in_transition.rs` |
| I-FREEZE-CONFIG | TAPE_ECONOMY_V2 + FOUNDER_GRANT_GAMMA + system_lp_amount frozen at task creation, not at tx submission | TaskMarket::publish | `tests/task_config_frozen_at_publish.rs` |
| **I-NORANDOM** (added per Gemini v3.2 review Q1) | Any tx that consumes randomness MUST seed PRNG from `(tx.tx_id, q.state_root_t)`; no system entropy in step_transition path | step_transition stages 1-7 | `tests/no_runtime_entropy.rs` |
| **I-VERIFY-LIVE** (added per Gemini v3.2 review Q10) | VerifyTx targets MUST be in Pending or Provisional state; cannot verify Accepted-and-finalized or Slashed | verify_transition stage 1 | `tests/verify_target_liveness.rs` |
| **I-CHAL-WINDOW** (added per Gemini v3.2 review Q10) | ChallengeTx must be received within target's challenge_cases_t window; no challenges after window close | challenge_transition stage 1 | `tests/challenge_window_enforced.rs` |
| **I-FINALIZE-EXCLUSIVE** (added) | FinalizeRewardTx and SlashTx are mutually exclusive per claim_id; system runtime serializes | finalize_reward_transition stage 2 | `tests/finalize_or_slash_exclusive.rs` |
| **I-VBOND-RELEASE** (v1.1, gap 11.2 fix) | Verifier bond release on slashed work_tx follows TaskMarket.config.verifier_bond_on_slash policy; default = `ReturnToVerifier`; verifier reputation NOT adjusted under default policy | challenge_transition stage 4e | `tests/verifier_bond_release.rs` |
| **I-ROYALTY-CAP** (v1.1, gap 11.3 fix) | reuse_tx edge weight ≤ TaskMarket.config.max_reuse_royalty_fraction (default 0.10); excess clamped + warning logged | reuse_transition stage 3 | `tests/royalty_cap_enforced.rs` |

**Total: 22 invariants → 22 tests**. Every transition test must pass before CO1.1.4 (bus.rs split) starts. STEP_B implementation comparison is "branch X conforms to spec" / "branch Y conforms to spec", not "branch X looks like branch Y".

---

## § 5 Optional TLA+ Skeleton (deferred to spec-gate audit)

For ordering + replay invariants (I-DET, I-DETHASH, I-LOGTIME), Codex suggested TLA+/PlusCal. ArchitectAI agrees with the suggestion but does NOT include the full model in v1 of this spec — it would balloon the doc. Skeleton:

```tla
EXTENDS Naturals, Sequences

VARIABLES q, ledger, signals

Init == /\ q = GenesisQState
        /\ ledger = <<>>
        /\ signals = <<>>

Step(tx) == /\ ValidParent(tx, q)
            /\ ValidSignature(tx)
            /\ StakeAvailable(tx, q)
            /\ AcceptancePredicates(tx, q)
            /\ q' = Apply(q, tx)
            /\ ledger' = Append(ledger, tx)
            /\ signals' = EmitSignals(q, tx, q')

Spec == Init /\ [][\E tx \in WorkTx : Step(tx)]_<<q, ledger, signals>>

\* Determinism: same input sequence → same final state
DeterminismProperty == \A seq1, seq2 \in Seq(WorkTx) :
    (seq1 = seq2) => (Replay(seq1) = Replay(seq2))
```

If CO P1 audit demands stronger guarantees, the TLA+ model is upgraded to a full PlusCal program with TLC model checking. For v4 scope, the type-level + conformance-test combination is deemed sufficient by Codex.

---

## § 5.1 v1.1 Walk-Through Gap Resolutions

Per `SPEC_WALKTHROUGH_v1_2026-04-27.md` § 11, four spec gaps were found. Resolution status:

| Gap | Issue | v1.1 Resolution | User-overridable |
|---|---|---|---|
| 11.1 | False-challenge reputation penalty undefined | TaskMarket.config.false_challenge_reputation_penalty default = 0 (no penalty, encourages legitimate challenges per Bitcoin "let market decide" principle) | yes — set per TaskMarket |
| 11.2 | Verifier bond release policy on slashed claim | spec § 3.2 stage 4e ADDED with `VerifierBondPolicy::ReturnToVerifier` default | yes — `verifier_bond_on_slash` config |
| 11.3 | Royalty edge weight bound | spec § 3.3 stage 3 ADDED with `MAX_REUSE_ROYALTY_FRACTION_DEFAULT = 0.10` | yes — `max_reuse_royalty_fraction` config |
| 11.4 | Multi-verifier quorum aggregation | spec § 3.1 note ADDED with `verifier_quorum_required: usize = 1` default; full multi-verifier impl deferred to CO P2.7 | yes — set per TaskMarket |

All 4 gaps now have machine-checkable defaults. User can override any default via TaskMarket.config when creating tasks; the default applies if config field is missing.

---

## § 6 What This Spec DOES NOT Specify

Listed for honesty:

1. **MetaTx full schema** — only stub here; v4.1 atom defines.
2. **AttributionEngine deterministic DAG construction** — CO2.4.0 spike (separate doc).
3. **Predicate visibility leak channels** — covered at CO P1.5 design (Goodhart shield); this spec only declares `BoolWithProof.proof_visibility_class`, not the leak-proof proof format.
4. **gix Path B substrate-specific operations** — CO1.3.1 spike validates; this spec is substrate-agnostic.
5. **Retry metadata bound on `failed_attempts_since_last_accept`** — must be finite for tape size containment, but exact bound (e.g., u32::MAX vs cap-at-1000) is CO P1.7 design choice.
6. **Verifier verdict aggregation rule** — when N verifiers vote, how to combine? CO2.7 design.
7. **Challenge window length** — `CHALLENGE_WINDOW_TICKS` is a TaskMarket config bound at publish, but the default value + bounds are CO2.5 design.

These deferrals are **explicit and named**. Future atoms reference this list to resolve them.

---

## § 7 Pre-CO P1 Gate Procedure

1. ArchitectAI commits this spec v1
2. Codex independent review: confirm that every WP § 4-7 + economic § 2/§ 6 / § 18-21 concept maps to a typed field or invariant here
3. Gemini cross-review: confirm spec respects ENTIRE white paper (not just cited §)
4. Both PASS → spec frozen as v1 (any change requires re-audit)
5. **Then** Plan v3.2 atom CO1.SPEC.0 marked complete; CO1.0 / CO1.1.* / CO1.2.* atoms cleared to start
6. STEP_B implementation: Claude implements branch A against spec; Codex implements branch B against spec; comparison metric = "spec conformance", not "code similarity"

---

## § 8 Honest Acknowledgements

What this spec is:
- A typed, deterministic, side-effect-free state transition definition
- A binding contract for STEP_B branch A/B comparison
- A list of 16 named invariants each backed by a conformance test path

What this spec is NOT:
- A full formal proof (no Lean/Coq)
- A complete TLA+ model (skeleton only)
- A substitute for code review (still required per Protocol Hard rule 1+2)
- A guarantee that branches A/B will produce identical Rust code (only spec-equivalent code)

What this spec does NOT yet include and the user must decide:
- Whether to run full TLA+ TLC model check (~3-5 day effort) or stop at type+test level (Codex suggested optional)
- Whether `RejectionClass::Opaque` aggregation respects Goodhart shield in practice (deferred to CO P1.5)
- Whether to embed Art 0.2 mini-amendment (see `ART_0_2_REINTERPRETATION_2026-04-27.md`) BEFORE running this spec, or AFTER (depends on rejection-on-tape constitutional reading)

— ArchitectAI, 2026-04-27
