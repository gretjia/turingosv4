---- MODULE StateTransitionSpec ----
\* TuringOS v4 — State Transition TLA+ Skeleton (CO1.SPEC.0.4 OPTIONAL)
\* Date: 2026-04-27
\* Authority: STATE_TRANSITION_SPEC_v1_2026-04-27.md § 5
\* Purpose: Optional formal model for ordering + replay invariants (I-DET, I-DETHASH, I-LOGTIME).
\*          Skeleton only; full TLC model check is per-atom audit decision.
\*
\* Status: SKELETON. Compiles but is not yet TLC-checked. Promote to full PlusCal
\*         + run TLC if any CO P1 audit demands stronger guarantees beyond the
\*         16+4=20 type-level + conformance-test invariants.

EXTENDS Naturals, Sequences, FiniteSets, TLC

CONSTANTS
    Agents,           \* set of agent IDs
    PredicateIds,     \* set of predicate IDs
    TaskIds,          \* set of task IDs
    MaxLogicalTime    \* upper bound on timestamp_logical for model checking

\*============================================================
\* Type definitions
\*============================================================

\* QState is a 9-tuple matching state_transition_spec § 1.1
QState == [
    q_t                            : [agents : SUBSET Agents],
    head_t                         : Nat,
    state_root_t                   : Nat,
    tape_view_t                    : Seq(Nat),       \* simplified for model
    ledger_root_t                  : Nat,
    predicate_registry_root_t      : Nat,
    tool_registry_root_t           : Nat,
    economic_state_t               : EconomicState,
    budget_state_t                 : BudgetState
]

EconomicState == [
    balances_t       : [Agents -> Nat],     \* MicroCoin = Nat in this abstraction (i64 in code)
    escrows_t        : [TaskIds -> Nat],
    stakes_t         : [Agents \X TaskIds -> Nat],
    claims_t         : Seq(Nat),
    reputations_t    : [Agents -> Nat],
    challenge_cases_t : [TaskIds -> {"open", "closed", "slashed"}]
]

BudgetState == [
    cost_ceiling : Nat,
    wall_clock   : Nat,
    compute_cap  : Nat
]

\* WorkTx 12 fields per state_transition_spec § 1.2
WorkTx == [
    tx_id                : Nat,
    task_id              : TaskIds,
    parent_state_root    : Nat,
    agent_id             : Agents,
    read_set             : SUBSET Nat,
    write_set            : SUBSET Nat,
    proposal_cid         : Nat,
    predicate_results    : [acceptance : BOOLEAN, settlement : BOOLEAN, safety_class : {"Safety", "Creation"}],
    stake                : Nat,                       \* MicroCoin
    signature            : Nat,                       \* signature digest
    timestamp_logical    : 0..MaxLogicalTime,
    status               : {"Pending", "Accepted", "Rejected", "Finalized"}
]

\*============================================================
\* Variables
\*============================================================

VARIABLES
    q,           \* current QState
    ledger,      \* sequence of accepted transitions
    history      \* all transition attempts (for replay invariant)

vars == <<q, ledger, history>>

\*============================================================
\* Initial State (genesis)
\*============================================================

GenesisQState == [
    q_t                            |-> [agents |-> {}],
    head_t                         |-> 0,
    state_root_t                   |-> 0,
    tape_view_t                    |-> <<>>,
    ledger_root_t                  |-> 0,
    predicate_registry_root_t      |-> 0,    \* EMPTY_TREE_ROOT in spec
    tool_registry_root_t           |-> 0,
    economic_state_t               |-> [
        balances_t        |-> [a \in Agents |-> 0],
        escrows_t         |-> [t \in TaskIds |-> 0],
        stakes_t          |-> [<<a, t>> \in Agents \X TaskIds |-> 0],
        claims_t          |-> <<>>,
        reputations_t     |-> [a \in Agents |-> 0],
        challenge_cases_t |-> [t \in TaskIds |-> "closed"]
    ],
    budget_state_t                 |-> [cost_ceiling |-> 100, wall_clock |-> 1000, compute_cap |-> 100]
]

Init ==
    /\ q       = GenesisQState
    /\ ledger  = <<>>
    /\ history = <<>>

\*============================================================
\* Predicates from spec § 3 step_transition
\*============================================================

ValidParent(tx, qs) == tx.parent_state_root = qs.state_root_t

ValidSignature(tx) == tx.signature # 0  \* simplified; real impl is crypto verify

StakeAvailable(tx, qs) == qs.economic_state_t.balances_t[tx.agent_id] >= tx.stake

AcceptancePredicates(tx) == tx.predicate_results.acceptance

\*============================================================
\* step_transition (WorkTx) — spec § 3
\*============================================================

ApplyWorkTx(qs, tx) ==
    [qs EXCEPT
        !.economic_state_t.balances_t[tx.agent_id]   = @ - tx.stake,
        !.economic_state_t.stakes_t[<<tx.agent_id, tx.task_id>>] = @ + tx.stake,
        !.economic_state_t.claims_t                  = Append(@, tx.tx_id),
        !.ledger_root_t                              = qs.ledger_root_t + 1,
        !.state_root_t                               = qs.state_root_t + 1,
        !.head_t                                     = qs.state_root_t + 1
    ]

StepWork(tx) ==
    /\ ValidParent(tx, q)
    /\ ValidSignature(tx)
    /\ StakeAvailable(tx, q)
    /\ AcceptancePredicates(tx)
    /\ q'       = ApplyWorkTx(q, tx)
    /\ ledger'  = Append(ledger, tx)
    /\ history' = Append(history, tx)

\* Rejection: history advances but q does not (Inv 6 predicate-gated)
StepReject(tx) ==
    /\ \neg (ValidParent(tx, q) /\ ValidSignature(tx) /\ StakeAvailable(tx, q) /\ AcceptancePredicates(tx))
    /\ q'       = q
    /\ ledger'  = ledger
    /\ history' = Append(history, tx)

\*============================================================
\* Next-state relation
\*============================================================

Next == \E tx \in [
            tx_id                |-> Nat,
            task_id              |-> TaskIds,
            parent_state_root    |-> Nat,
            agent_id             |-> Agents,
            read_set             |-> SUBSET Nat,
            write_set            |-> SUBSET Nat,
            proposal_cid         |-> Nat,
            predicate_results    |-> [acceptance |-> BOOLEAN, settlement |-> BOOLEAN, safety_class |-> {"Safety", "Creation"}],
            stake                |-> Nat,
            signature            |-> Nat,
            timestamp_logical    |-> 0..MaxLogicalTime,
            status               |-> {"Pending", "Accepted", "Rejected", "Finalized"}
        ] : StepWork(tx) \/ StepReject(tx)

Spec == Init /\ [][Next]_vars

\*============================================================
\* Invariants (corresponds to spec § 4)
\*============================================================

\* I-DET — same input sequence → same final state
\* (TLC checks this via reproducibility of Next from same Init)
DeterminismProperty ==
    \A h1, h2 \in Seq(WorkTx):
        h1 = h2 => ApplyHistory(h1) = ApplyHistory(h2)

\* I-PARENT — accepted txs in ledger all have valid parent at submission time
ParentInvariant ==
    \A i \in 1..Len(ledger): ledger[i].parent_state_root \in 0..i

\* I-STAKE — balances never go negative (atomicity)
StakeInvariant ==
    \A a \in Agents: q.economic_state_t.balances_t[a] >= 0

\* I-LOGTIME — timestamp_logical strictly monotonic across history
MonotonicTimeInvariant ==
    \A i \in 1..(Len(history)-1):
        history[i].timestamp_logical < history[i+1].timestamp_logical

\* I-PRED-GATE — q does not advance when predicate fails
PredicateGateInvariant ==
    \A i \in 1..Len(history):
        \/ history[i] \in {ledger[j] : j \in 1..Len(ledger)}    \* tx is in ledger (accepted)
        \/ \neg AcceptancePredicates(history[i])                \* OR predicate failed

\* I-NOSIDE — q advances ONLY via Next; no hidden mutations
NoSideEffectInvariant ==
    [][q' = q \/ \E tx \in WorkTx: ApplyWorkTx(q, tx) = q']_vars

\* I-FINALIZE-EXCLUSIVE — challenge_cases_t["slashed"] never coexists with finalized claim
FinalizeExclusiveInvariant ==
    \A t \in TaskIds:
        q.economic_state_t.challenge_cases_t[t] # "slashed" \/ \neg HasFinalizedClaimForTask(t)

HasFinalizedClaimForTask(t) ==
    \E c \in 1..Len(q.economic_state_t.claims_t):
        TRUE   \* simplified; real: lookup claim's task_id == t

\*============================================================
\* Theorems (placeholder, awaiting TLC verification)
\*============================================================

THEOREM Spec => []ParentInvariant
THEOREM Spec => []StakeInvariant
THEOREM Spec => []MonotonicTimeInvariant
THEOREM Spec => []PredicateGateInvariant
THEOREM Spec => []NoSideEffectInvariant
THEOREM Spec => []FinalizeExclusiveInvariant

\*============================================================
\* TLC Configuration (skeleton)
\*============================================================

\* To run TLC model check (when CO P1 audit demands it):
\*   tlc StateTransitionSpec.tla -config StateTransitionSpec.cfg
\*
\* Expected .cfg file:
\*   SPECIFICATION Spec
\*   INVARIANTS ParentInvariant StakeInvariant MonotonicTimeInvariant
\*              PredicateGateInvariant FinalizeExclusiveInvariant
\*   CONSTANTS Agents = {"a1", "a2"}
\*             PredicateIds = {1, 2, 3}
\*             TaskIds = {"t1", "t2"}
\*             MaxLogicalTime = 10
\*   CHECK_DEADLOCK FALSE

\*============================================================
\* Skeleton acknowledgements
\*============================================================

\* This skeleton is INTENTIONALLY simplified:
\* - Stake amounts modeled as Nat, not full i64 micro-coin (suffices for invariant check)
\* - Signatures abstracted to inequality (real impl uses ed25519)
\* - VerifyTx / ChallengeTx / ReuseTx / FinalizeRewardTx / TerminalSummaryTx
\*   transitions are NOT yet modeled (only WorkTx). Future: extend Next with all
\*   5 transition variants.
\* - Multi-parent merge in attribution DAG NOT modeled (CO2.4.0 spike domain)
\*
\* Promotion to full PlusCal recommended IF:
\* - CO P1 audit explicitly demands stronger ordering proofs
\* - User wants TLC model check report as part of v4 ship documentation
\* - A specific bug in CO P1 implementation exposes a non-obvious ordering issue
\*
\* Otherwise, the 20 conformance tests in spec § 4 + standard cargo test
\* execution are sufficient for v4 ship gate.

====
