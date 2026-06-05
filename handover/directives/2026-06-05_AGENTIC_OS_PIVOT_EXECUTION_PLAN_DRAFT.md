# TuringOS Agentic OS Pivot Execution Plan

Date: 2026-06-05
Status: planning packet, not implementation evidence
Source request: `/orchestrate` plan synthesis from the attached 10-phase
Agentic OS pivot blueprint plus the architect guardrail memo.
Risk of this file: Class 0. The atoms described below include Class 2, Class 3,
and Class 4 candidates.

Canonical scope: this is the only master execution plan for the Agentic OS
pivot in this workspace. Files such as
`OS_PIVOT_AGENTIC_OS_SUBSTRATE_2026-06-05.md`, `TC_000_PATH_B_DECISION.md`,
and `CLAIM_INTEGRITY_MERGE_CONTRACT_2026-06-05.md` are child artifacts
produced by atoms in this plan; they are not competing master plans.

This plan does not authorize merging PR #280 or PR #283. Both remain audit-only
snapshots.

## 0. Orchestrator Decision

North star:

```text
TuringOS can boot.
TuringOS can host scoped agents.
TuringOS records every state transition and side effect on GitTape/ChainTape.
TuringOS accepts only predicate-passing transitions.
TuringOS derives wallet, market, price, scheduler, reports, and benchmarks from tape.
TuringOS replays without LLM/network calls.
```

One-line architecture:

```text
GitTape is paper; ExternalCall is syscall; AgentView is process isolation;
PredicateReceipt is kernel admission; Economy is a derived OS service;
Benchmark is a user workload.
```

Strategic ruling:

1. PR #280 is a quarry and evidence snapshot, not a production branch.
2. PR #283 is a quarry and evidence snapshot, not a production branch.
3. Claude P1 does not refute agent market economics. It scopes one negative
   result: price-as-parent-router under homogeneous agents, abundant budget,
   Lean proof-tree search, directional and not statistically definitive.
4. Market economy remains a first-class OS service, but only as a projection
   over GitTape/ChainTape. No second ledger.
5. All work cuts small PRs from latest `origin/main`. Any atom touching
   restricted surfaces follows AGENTS.md Class 3/4 cadence.

Current verified PR facts, 2026-06-05:

```text
#280 draft=true, title contains AUDIT ONLY and DO NOT MERGE,
head=codex/tc-operationalization-audit-snapshot-20260605, base=main,
pinned_head_oid=e1605911c883aea4ce842b7fee7d41bd0448f947.

#283 draft=true, title contains AUDIT ONLY and DO NOT MERGE,
head=claude/p1-realvalue-audit-snapshot-20260605,
base=audit-base-p1-branchpoint-20260605,
pinned_head_oid=4cfbc41e23042d8ff496ef775dc91e5af2c0f9d8.
```

Snapshot ancestry block for every production PR:

```bash
set -euo pipefail
git fetch origin pull/280/head:refs/audit-snapshots/pr-280
git fetch origin pull/283/head:refs/audit-snapshots/pr-283
test "$(git rev-parse refs/audit-snapshots/pr-280)" = e1605911c883aea4ce842b7fee7d41bd0448f947
test "$(git rev-parse refs/audit-snapshots/pr-283)" = 4cfbc41e23042d8ff496ef775dc91e5af2c0f9d8
for oid in \
  e1605911c883aea4ce842b7fee7d41bd0448f947 \
  4cfbc41e23042d8ff496ef775dc91e5af2c0f9d8
do
  if git merge-base --is-ancestor "$oid" HEAD; then
    echo "forbidden audit snapshot ancestor: $oid"
    exit 1
  fi
done
```

Expected: exit 0. Any `fetch`, OID mismatch, or ancestry hit is a hard block.
If blocked, the PR must be split again from `origin/main` or this plan must be
explicitly revised with new pinned audit-snapshot OIDs.

## 1. Constitution Landing Check

The plan intentionally handles a forward substrate wave. It must not become the
old forbidden loop of charter -> atom -> self-audit -> delayed test.

Result:

```text
AMBER rows found: 40
High-load categories:
  Art. 0 tape canonicality
  FC1 rtool/input/external attempt visibility
  FC2 boot/replay/no memory preseed
  FC3 logs feedback/re-init/ArchitectAI/Veto-AI externality
  shielding and dashboard-derived facts
Charter/audit justified: yes, only as a forward execution plan whose first
  executable atoms build the missing substrate needed to close those AMBER rows.
Anti-pattern match: none if each atom begins with a falsifiable gate and runs
  evidence before clean-context audit.
Verdict: PROCEED with this plan. Do not dispatch G1 audits as substitutes for
  implementation evidence.
```

Every atom below must state:

```text
FC nodes:
Risk class:
Allowed paths:
Restricted-surface check:
First failing gate:
Real evidence path:
Clean-context audit requirement:
Claim boundary:
```

## 2. Research Intake

External best-practice notes used by the orchestrator:

1. Event Sourcing and CQRS: Microsoft Azure Architecture Center treats the
   event store as the authoritative write model and materialized views as
   read-only projections optimized for queries. This supports the GitTape as
   source of truth and L6/L7/L9 as projections rule.
   Source: https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing
2. Projection cost: the same source warns that replaying events can be costly
   and materialized views are common. This justifies GitOid-watermarked
   projection caches, but only as derived views.
3. Git ref safety: `git update-ref <ref> <new> <old>` verifies the old object
   before moving a ref; `--stdin` supports transactions. This supports
   single-writer or OCC append, not raw file writes to `.git`.
   Source: https://git-scm.com/docs/git-update-ref
4. Git object integrity: `git fsck` verifies object connectivity and validity.
   This is a substrate integrity gate for generated GitTape repos.
   Source: https://git-scm.com/docs/git-fsck
5. External side effects: AWS Durable Execution guidance requires idempotency
   keys for external writes and deterministic event IDs for append-only logs.
   This supports ExternalCallIntent idempotency and replay dedupe.
   Source: https://docs.aws.amazon.com/durable-execution/patterns/best-practices/idempotency/
6. Crash and compensation: Azure Compensating Transaction guidance requires
   progress records, timeouts, resumability, idempotent commands, and
   correlation/audit from original operation to compensation. This supports
   Abandoned terminals and orphan sweeping.
   Source: https://learn.microsoft.com/en-us/azure/architecture/patterns/compensating-transaction
7. Agent evals: OpenAI evaluation guidance says multi-agent systems add a new
   nondeterminism point and should be driven by evals, with objective, dataset,
   metrics, and continuous evaluation. This supports preregistered market
   tracks and no benchmark headline without evidence.
   Source: https://platform.openai.com/docs/guides/evaluation-best-practices
8. Event envelope portability: CloudEvents exists because event producers
   otherwise describe events inconsistently. TuringOS does not need to adopt
   CloudEvents wholesale, but every tape event needs stable `id`, `type`,
   `source/run_id`, `time/logical_t`, `subject/parent`, and `data hash` fields.
   Source: https://cloudevents.io/

## 2a. Truth-Tier Table

No component outside Tier 2 may become runtime truth.

| Component | OS layer | truth_tier | canonical_record | cas_witness | replay_recipe |
|---|---:|---|---|---|---|
| Constitution and flowchart hashes | L0 | Tier 1 axiom | `constitution.md` + pinned hashes | none | Compile/start-time hash gates |
| GitTape/ChainTape | L2 | Tier 2 fact | `TapeEventEnvelope` commits under accepted/rejected/pending refs | payload CID per event | Replay from genesis to HEAD |
| CAS | L2 | Tier 2 fact | content-addressed objects | object hash is witness | Resolve every CID from tape and verify hash |
| Replay/audit verifier | L2/L9 | Tier 2 fact checker | verifier binary + command output | audit packet hash | Recompute state from ChainTape + CAS |
| ExternalCall outbox | L3 | Derived event family on Tier 2 | Intent + exactly one Terminal tape event | redacted request/result CID | Replay checks terminal state, never calls provider |
| AgentView | L4 | Derived view | none, view is built from tape prefix | CID allow/deny witnesses | Rebuild view from `allowed_tape_prefix` |
| PredicateReceipt | L5 | Tier 2 event, not predicate authority by itself | receipt tape event bound to registry root | private diagnostic CID optional | Replay re-executes active predicate registry |
| Economy Service | L6 | Derived projection | EconomyEvent tape payloads only | event payload CIDs | Rebuild wallet/market/price/settlement from tape |
| Projection cache | L6/L7 | Derived cache | none canonical | optional cache checksum | Valid only if `derived_from_tape_head == HEAD` |
| Scheduler policy | L7 | Derived decision event | SchedulerDecision tape event | decision input CIDs | Replay policy inputs from tape prefix |
| Workload adapter | L8 | User workload | adapter-specific tape events | workload evidence CIDs | Replay adapter receipts; do not change kernel authority |
| Reports/dashboards/LATEST | L9/Tier 3 or below | Derived view | none canonical | optional rendered artifact hash | Regenerate from ChainTape/CAS |

Acceptance for future plan revisions:

```bash
grep -n 'truth_tier' handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md
grep -n 'GitTape/ChainTape.*Tier 2 fact' handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md
grep -n 'Reports/dashboards/LATEST.*Derived view' handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md
```

## 2b. FC3 Scope Boundary

This plan does not claim full FC3 closure by itself.

| FC3 gap | Current matrix status | Plan handling |
|---|---|---|
| `logs -> feedback -> architectAI` | MISSING | Forward-bound. A12 may witness proposal-only behavior; runtime ArchitectAI feedback consumer requires its own future atom and risk classification. |
| `error -> re-init -> boot` | MISSING | Forward-bound unless A06 orphan sweeper and A13 OS E2E explicitly add restart/re-init semantics with tests. No broad FC3 closure claim before that. |
| `Veto-AI` runtime role | EXTERNAL_ONLY | Witness-only. No runtime write authority. |
| `ArchitectAI` runtime role | EXTERNAL_ONLY | Proposal-only. No direct write authority without governed future work. |

## 2c. OBLIGATIONS Impact

This plan adds and satisfies only the current planning obligation:

```text
OBL-006 satisfied by this plan document.
```

This plan does not close existing older must obligations:

```text
OBL-001 remains open.
OBL-004 remains in_progress.
OBL-005 remains blocked until the remaining FC3 feedback/re-init and related
production gaps are implemented or explicitly superseded.
```

Therefore this file must not be described as completing the entire pivot. It is
a drift-resistant execution contract for future atoms.

## 3. Locked Cross-Atom Contracts

These are plan-level contracts. An implementation atom may tighten types, but
it must not silently rename fields or create a second truth source.

### C1. Tape event envelope

```rust
pub struct TapeEventEnvelope {
    pub run_id: RunId,
    pub logical_t: u64,
    pub event_id: EventId,
    pub parent_event_id: Option<EventId>,
    pub input_tape_head: GitOid,
    pub author: ActorId,
    pub kind: TapeEventKind,
    pub payload_cid: Cid,
    pub payload_hash: Hash,
    pub idempotency_key: Option<IdempotencyKey>,
}

pub enum TapeEventKind {
    Boot,
    AgentProposal,
    PredicateReceipt,
    WorkAccepted,
    WorkRejected,
    ExternalCallIntent,
    ExternalCallResult,
    ExternalCallFailure,
    ExternalCallAbandoned,
    EconomyEvent,
    SchedulerDecision,
    Halt,
}
```

### C2. GitTape state

```rust
pub struct GitTapeState {
    pub run_id: RunId,
    pub repo_path: PathBuf,
    pub accepted_ref: RefName,
    pub rejected_ref: RefName,
    pub pending_ref: RefName,
    pub head_commit: GitOid,
    pub logical_t: u64,
}
```

Write rule:

```text
Only the tape writer may move refs.
Ref movement uses expected-old GitOid verification or an explicit single-writer queue.
Every generated repo passes git fsck --full.
```

### C3. Projection contract

```rust
pub trait TapeProjection: Sized {
    type Error;

    fn projection_id() -> &'static str;
    fn derive_from_tape(events: impl Iterator<Item = TapeEventEnvelope>)
        -> Result<Self, Self::Error>;
    fn derived_from_tape_head(&self) -> GitOid;
}

pub struct ProjectionCache<T> {
    pub projection: T,
    pub derived_from_tape_head: GitOid,
    pub last_applied_logical_t: u64,
}
```

Cache rule:

```text
Cache is legal only if derived_from_tape_head == current GitTape HEAD.
If stale, apply deltas from the cached head to current head.
Full replay is allowed in tests and repair paths, not on every scheduler read.
```

### C4. External call contract

```rust
pub struct ExternalCallIntent {
    pub intent_id: IntentId,
    pub logical_call_id: LogicalCallId,
    pub call_site: CallSite,
    pub run_id: RunId,
    pub request_hash: Hash,
    pub redacted_request_cid: Cid,
    pub idempotency_key: IdempotencyKey,
    pub timeout_ms: u64,
    pub logical_t: u64,
}

pub enum ExternalCallTerminal {
    Result {
        result_hash: Hash,
        usage: UsageReceipt,
        status: ProviderStatus,
        provider_request_id: Option<String>,
    },
    Failure {
        class: ExternalFailureClass,
        retryable: bool,
        public_summary: String,
    },
    Abandoned {
        reason: AbandonReason,
        may_have_spent: bool,
    },
}
```

Terminal rule:

```text
Each intent has exactly one terminal.
Clean halt requires pending_intents == 0.
Replay never calls network or LLM.
Boot sweeper appends Abandoned terminals for stale orphan intents.
Recovery after after_provider_before_terminal must not reissue a physical call
unless the provider supports the original idempotency_key and the retry path
proves the same logical_call_id/idempotency_key pair is reused.
If physical-call completion is uncertain, boot appends Abandoned with
may_have_spent=true instead of guessing success or retrying a non-idempotent
side effect.
```

### C5. AgentView contract

```rust
pub struct AgentProcess {
    pub agent_id: AgentId,
    pub role: AgentRole,
    pub lane: LaneId,
    pub budget: Budget,
    pub view_scope: ViewScope,
    pub independence_seed: Seed,
}

pub struct AgentView {
    pub public_task: PublicTaskView,
    pub allowed_tape_prefix: GitOid,
    pub allowed_market_projection: Option<MarketProjection>,
    pub allowed_error_summary: Vec<ErrorClass>,
    pub forbidden_cids: Vec<Cid>,
}
```

AgentView allowlist:

```text
visible_to_agent:
  public task
  tape prefix explicitly granted to this process
  public predicate reason class
  public economy projection derived from the granted prefix
  low-pollution summarized errors
  CID handles for audit-only artifacts, not raw bodies
```

AgentView denylist:

```text
never_visible_to_agent:
  hidden verifier/test body
  raw Lean stderr or raw provider stderr
  private diagnostic CID contents
  sibling private chains
  future tape events or future price broadcasts
  deep history without explicit operator override
```

### C6. PredicateReceipt contract

```rust
pub struct PredicateReceipt {
    pub registry_root: RegistryRoot,
    pub predicate_id: PredicateId,
    pub predicate_version: PredicateVersion,
    pub input_binding: Hash,
    pub input_event_hash: Hash,
    pub verifier_kind: VerifierKind,
    pub verifier_version: String,
    pub verdict: PredicateVerdict,
    pub public_reason_class: ReasonClass,
    pub private_diagnostic_cid: Option<Cid>,
    pub output_hash: Hash,
    pub recompute_path: RecomputePath,
}

pub enum PredicateVerdict {
    Pass,
    Fail,
}
```

### C7. Economy event contract

```rust
pub enum EconomyEvent {
    CoinMinted {
        mint_id: MintId,
        recipient: AgentId,
        amount: MicroCoin,
        reason: MintReason,
        genesis_or_on_init_only: bool,
    },
    TaskMarketOpened {
        market_id: MarketId,
        task_id: TaskId,
        bounty: MicroCoin,
        predicate_id: PredicateId,
    },
    PositionOpened {
        market_id: MarketId,
        agent_id: AgentId,
        side: YesNo,
        amount: MicroCoin,
        parent_event: EventId,
    },
    ChallengeSubmitted {
        market_id: MarketId,
        challenger: AgentId,
        target_event: EventId,
        stake: MicroCoin,
        public_reason_class: ReasonClass,
    },
    WorkSubmitted {
        work_id: WorkId,
        market_id: MarketId,
        agent_id: AgentId,
        work_cid: Cid,
        claimed_predicate_id: PredicateId,
    },
    PredicateResolved {
        work_id: WorkId,
        predicate_receipt_hash: Hash,
        verdict: PredicateVerdict,
    },
    PayoutSettled {
        market_id: MarketId,
        work_id: WorkId,
        winners: Vec<AgentId>,
        losers: Vec<AgentId>,
        amounts: Vec<MicroCoin>,
    },
    PriceBroadcast {
        market_id: MarketId,
        price_vector_hash: Hash,
        derived_from_tape_head: GitOid,
        policy_version: PolicyVersion,
    },
    BudgetAllocated {
        allocator_policy_id: PolicyId,
        recipient_agent_id: AgentId,
        amount: MicroCoin,
        derived_from_price_head: GitOid,
    },
}
```

Money rule:

```text
MicroCoin uses integer math only.
No f32/f64 in conservation, wallet, settlement, escrow, or budget paths.
Price can route attention or budget. Price cannot change predicate verdict.
```

### C8. Scheduler decision contract

```rust
pub trait SchedulerPolicy {
    fn policy_id(&self) -> PolicyId;
    fn select_next(
        &self,
        tape_prefix: GitOid,
        view: SchedulerView,
    ) -> SchedulerDecision;
}

pub struct SchedulerDecisionEvent {
    pub policy_id: PolicyId,
    pub input_tape_head: GitOid,
    pub scheduler_view_cid: Cid,
    pub candidate_set_cid: Cid,
    pub candidate_set_hash: Hash,
    pub policy_input_bundle_hash: Hash,
    pub selected_parent: Option<EventId>,
    pub random_seed_or_deterministic_reason: DecisionReason,
    pub price_projection_head: Option<GitOid>,
    pub scoped_agent_view_head: Option<GitOid>,
    pub decision_hash: Hash,
}
```

### C9. PR acceptance contract

Every production PR body must include:

```text
## OS Layer
L0/L1/L2/L3/L4/L5/L6/L7/L8/L9

## Risk Class
Class 0/1/2/3/4

## Exact File List
<all files>

## Forbidden Surface Check
constitution.md touched? yes/no
build.rs touched? yes/no
genesis_payload.toml touched? yes/no
sequencer/schema/signing touched? yes/no
src/lib.rs or runtime/mod.rs touched? yes/no

## Source of Truth Claim
Does this PR create or modify source of truth? yes/no
If yes, why is it ChainTape/GitTape?
If no, what derive_from_tape test proves it?

## Claim Boundary
Supported:
Unsupported:

## Tests
targeted:
constitution gates:
workspace if applicable:
diff checks:
secret scan:
raw evidence scan:

## Clean-Context Audit
required for Class 2+? yes/no
verdict:
```

## 4. Global Hard Blockers

Any hit blocks merge:

```text
B1. PR is based on #280/#283 by merge instead of clean split.
B2. PR mixes runtime code and bulk evidence JSON/logs.
B3. PR adds raw repo_*/cas_* logs to main.
B4. PR creates wallet/market/price as non-derived source of truth.
B5. PR lets price override predicate.
B6. PR claims TASK-PASS without verifier-backed TASK-PASS.
B7. PR touches build.rs/genesis_payload.toml outside declared trust-root atom.
B8. PR adds benchmark adapter into kernel authority path.
B9. PR contains PROVEN/DEFINITIVE/causal/world-first headline without G1-G6.
B10. PR leaks hidden tests/raw stderr/private diagnostics into AgentView.
B11. PR adds a new latest/global pointer as canonical input.
B12. PR adds Manager/Factory/Engine/Platform/Framework as fake future ceremony.
B13. PR introduces f32/f64 in money/economy conservation paths.
B14. PR audits before runnable evidence exists for Class 2+ ship work.
B15. PR rewrites historical ChainTape/L4/L4.E/CAS evidence.
```

## 5. Model and Agent Routing

Use `/orchestrate` phases. Do not spawn agents because the task feels large;
spawn them only when their outputs are disjoint and checkable.

```text
Planning research:
  BRIEF/fast: repo inventory, exact path checks, open PR overlap.
  STANDARD: best-practice research and source summary.
  DELIBERATIVE: constitution/drift critic.

Implementation per atom:
  Class 0/1 docs or isolated helpers: 1 implementer, BRIEF or STANDARD.
  Class 2 runtime wire-up: 1 implementer, STANDARD; add domain critic.
  Class 3 economy/CAS/capability: 1 implementer, DELIBERATIVE; no parallel code writers unless write sets are disjoint.
  Class 4 restricted surface: explicit per-atom §8 ratification first; clean-context audit before ship.

Review:
  Phase 5 critics: Constitution + Karpathy by default for this plan.
  Add Data-Integrity for tape/CAS/replay, Performance for projection caches,
  Security for external calls/capabilities, Cost-Budget for agent swarms.
  Phase 7 witness: one clean-context witness, closed verdict set.
  Phase 9 closing audit: reuse Phase 5 critics when possible.
```

Witness verdict domain for TuringOS code/governance:

```text
NO-VIOLATION
VIOLATION-FOUND <clause> <file>:<line>
RECONSTRUCTION-FAILURE <which-path>
SECOND-SOURCE-DRIFT <which-derived-view>
```

## 6. Atom Queue

Do not execute an atom out of order unless the predecessor field says it is
independent. Each atom must be its own PR unless the orchestrator explicitly
collapses adjacent Class 0 work.

Atom index:

| Atom | Layer | risk_class | observe_only by default | touches_restricted_surface |
|---|---:|---:|---|---|
| A00 | L0/L9 | 0 | yes | no |
| A01 | L0/L1/L2 | 0 | yes | no |
| A02 | L0/L9 | 0 now, 2 later | yes | no |
| A03 | L1 | 3 floor, 4 if authority changes | no | maybe: `build.rs`, `genesis_payload.toml` |
| A04 | L2 | 3 floor, 4 if sequencer/schema changes | no | maybe |
| A05 | L2/L6/L7/L9 | 2 or 3 | no | maybe if typed tx/CAS schema changes |
| A06 | L3/L1 | 2 or 3 | no | maybe if boot authority changes |
| A07 | L4 | 2 | no | no unless existing restricted surfaces are touched |
| A08 | L5/L8 | 2 or 3 | no | maybe if admission authority changes |
| A09 | L6 | 3 | no | no unless wallet/sequencer/schema touched |
| A10 | L6/L7 | 3 | yes for cache, no for economy tests | no |
| A11 | L7 | 2 or 3 | yes unless allocating real budget | no unless admission coupled |
| A12 | L2-L7 | 2 | witness-only | no unless runtime authority added |
| A13 | L1-L9 | 2 or 3 | no | maybe |
| A14 | L8/L9 | 2 or 3 | workload-only | no unless kernel authority touched |

Future atom packet shape:

```text
atom_id:
source_phase_or_quarry:
os_layer:
risk_class:
touched_FC_nodes_and_invariants:
objective_one_sentence:
allowed_paths_exact:
forbidden_paths_exact:
locked_contracts_C1_to_Cn_code_form:
source_of_truth_claim:
claim_boundary_supported:
claim_boundary_unsupported:
acceptance_criteria_exact_command_expected_output:
positive_controls_and_fail_closed_cases:
evidence_plan_or_no_real_run_rationale:
audit_plan_Phase5_Phase7_Phase9:
```

Machine-audit atom gate matrix:

| Atom | restricted_surface_check | first_failing_gate | real_evidence_path | audit_and_ratification | claim_boundary |
|---|---|---|---|---|---|
| A00 | no `src/**`, `tests/**`, or `handover/evidence/**` | `git diff --check` plus claim and `market_tape_shared` greps | no real run; docs-only diff is evidence | Class 0, no clean-context audit unless merged with another atom | freeze/pivot package only; no substrate completion |
| A01 | no runtime authority edits | ADR/TC grep for Path B, sole tape truth, derived projections | no real run; docs-only diff is evidence | Class 0, governs future Class 3/4 work | architecture contract only; no implementation proof |
| A02 | no `src/**`, `tests/**`, or evidence edits in docs atom | strong-claim grep and no `market_tape_shared` dependency grep | no real run; docs-only diff is evidence | Class 0 now; future gate work reclassified Class 2 | claim-integrity doctrine only |
| A03 | trust-root, constitution hash, signing payload, `build.rs`, and `genesis_payload.toml` require per-atom §8 | `cargo test --test constitution_tc_boot_trust_root_manifest` | boot manifest mismatch fixtures plus constitution gate output | clean-context audit required; Class 4 requires explicit A03 §8 phrase | boot trust-root gate only; no broader FC2 closure |
| A04 | sequencer admission, typed tx schema, signing payload, ChainTape refs require explicit classification; ChainTape-L4 is canonical and TDMA is compatibility-only | `cargo test --test tc_git_tape_ledger_hardening` and stale-head append positive control | generated GitTape/ChainTape repo plus `git fsck --full` output | clean-context audit required; §8 if restricted surfaces move | physical L2 writer hardening only; TDMA compat cannot claim OS L2 |
| A05 | CAS schema, typed tx, or projection authority changes require reclassification | `cargo test --test tape_event_envelope_roundtrip` and `cargo test --test tape_projection_replay` | replay fixtures proving projection derives from ChainTape/CAS | clean-context audit required if Class 2+ PR; Class 3 if economy/CAS integrity used | generic event/projection substrate only |
| A06 | boot authority, production LLM/network capability, CAS ObjectType, or provider write path changes require explicit classification | `cargo test --test tc_external_call_records` and `cargo test --test external_call_orphan_sweeper` | crash-matrix fixtures, offline replay output, pending-intent zero report | clean-context audit required for production path changes | ExternalCall syscall/outbox only; no provider benchmark claim |
| A07 | PromptCapsuleV2 schema or authority changes require explicit ratification | `cargo test --test tc_agent_view_shielding` plus hidden-oracle/private-diagnostic tests | leakage positive controls and web shielding test output | clean-context audit required for Class 2 PR | AgentView derived prefix view only; no new prompt authority |
| A08 | sequencer admission, typed tx predicate schema, predicate registry authority, or dedicated CAS ObjectType requires explicit ratification | `cargo test --test lean_judge_axiom_gate` and `cargo test --test predicate_receipt_replay` | Lean axiom fixtures plus PredicateReceipt replay output | clean-context audit required; §8 if admission authority changes | predicate receipt / Lean workload verifier only |
| A09 | `src/sdk/tools/wallet.rs`, sequencer, typed tx, CAS schema, or money authority changes require explicit classification | `cargo test --test economy_tape_replay` and `cargo test --test economy_conservation` | economy projection JSON plus conservation proof from tape | clean-context audit and money/economy Class 3 cadence required | L6 derived economy only; no second ledger |
| A10 | cache cannot write or replace ChainTape/CAS/predicate/settlement/admission authority | `cargo test --test projection_cache_not_source_of_truth` and `cargo test --test economy_projection_cache_watermark` | cache deletion/tamper fixtures plus byte-equivalent replay output | clean-context audit required for economy cache | performance cache only; never source of truth |
| A11 | no scheduler admission, budget allocation, or parallel write-path authority before A04/A05/A09/A10; restricted writer changes require §8 | `cargo test --test scheduler_policy_trace` and `cargo test --test scheduler_softmax_distribution` | SchedulerDecisionEvent replay fixtures and isolation tests | clean-context audit required if Class 2+; Class 3 if real budget allocated | routing/search projection only; price never predicate authority |
| A12 | witness-only unless runtime authority is separately ratified | `cargo test --test tc_universal_witness_counter_machine` through all A12 witnesses | universal witness run dirs and replay/tamper outputs | clean-context audit required for witness suite PR | witness coverage only; no full FC3 closure unless explicit |
| A13 | boot/economy/provider/capability changes reclassify; network-on is out of first E2E | `cargo test --test os_boot_to_replay_e2e` | `run_manifest.json`, `git_tape_repo/`, `replay_report.json`, receipts, economy projection, AgentView audit | clean-context audit required; Class 3 if real provider/economy calls | OS v0 smoke only; no solve-rate claim |
| A14 | workload adapters must not change kernel authority or claim source-of-truth status | `cargo test --test workload_adapter_claim_boundary` and `cargo test --test market_preregistration_contract` | preregistration packet, verifier-backed receipts, hash manifest | clean-context audit required before headline claims | workload/research boundary only; no TASK-PASS without verifier |

### A00. Freeze and Pivot Package

Layer: L0/L9 governance
Risk: Class 0
FC nodes: FC3-N30, FC3-N31, FC3:architectAI external proposal trail
Predecessor: none

Allowed paths:

```text
handover/directives/OS_PIVOT_AGENTIC_OS_SUBSTRATE_2026-06-05.md
.github/pull_request_template.md
handover/reports/P1_REALVALUE_SCOPE_CORRECTION_2026-06-05.md
```

Instructions:

1. State that #280 and #283 are audit-only.
2. State the priority order: OS substrate, tape-canonical economy, workload
   adapters, benchmark solve-rate.
3. Add the PR acceptance contract from C9 and blockers B1-B15 to the PR
   template.
4. Write P1 scope correction without demoting market economy.

Not in scope:

```text
No src/**
No tests/**
No handover/evidence/**
No benchmark claims
```

Acceptance:

```bash
git diff --name-only origin/main...HEAD
git diff --check
grep -RInE 'PROVEN|DEFINITIVE|world-first|TASK-PASS' \
  handover/directives/OS_PIVOT_AGENTIC_OS_SUBSTRATE_2026-06-05.md \
  .github/pull_request_template.md \
  handover/reports/P1_REALVALUE_SCOPE_CORRECTION_2026-06-05.md || true
grep -RInE '(^|[^A-Za-z])(mod|use)[[:space:]]+market_tape_shared|[m]arket_tape_shared::' \
  handover/directives/OS_PIVOT_AGENTIC_OS_SUBSTRATE_2026-06-05.md \
  .github/pull_request_template.md \
  handover/reports/P1_REALVALUE_SCOPE_CORRECTION_2026-06-05.md && exit 1 || true
```

Expected:

```text
Only allowed paths changed.
Strong claim grep returns only scoped warnings or none.
No source code or evidence files.
```

Audit:

```text
Class 0: no clean-context audit required unless merged with another atom.
```

### A01. Path B ADR and Layer Contract

Layer: L0/L1/L2 architecture
Risk: Class 0, but it governs future Class 3/4 work
FC nodes: FC1-N1, FC1-N3, FC2-N21, Art. 0.2, Art. 0.4
Predecessor: A00

Allowed paths:

```text
handover/architecture/ADR_2026-06-05_PATH_B_GITTAPE_AS_OS_SUBSTRATE.md
handover/directives/TC_000_PATH_B_DECISION.md
```

Instructions:

1. Create `handover/architecture/` if absent.
2. Declare Path B: GitTape/ChainTape is the sole `Q_t` source of truth.
3. Declare Vec<Node> and any memory ledger compatibility-only.
4. Declare wallet, market, price, search, dashboards, reports as projections.
5. Map L0-L9 layers and prohibited cross-layer edges.

Acceptance:

```bash
git diff --name-only origin/main...HEAD
git diff --check
grep -RIn 'GitTape/ChainTape.*sole\\|Path B\\|derived projection' \
  handover/architecture/ADR_2026-06-05_PATH_B_GITTAPE_AS_OS_SUBSTRATE.md \
  handover/directives/TC_000_PATH_B_DECISION.md
grep -RInE 'MarketTape|market_tape_shared|second ledger' \
  handover/architecture/ADR_2026-06-05_PATH_B_GITTAPE_AS_OS_SUBSTRATE.md \
  handover/directives/TC_000_PATH_B_DECISION.md || true
```

Expected:

```text
ADR says Path B.
ADR says no parallel market ledger.
No source code touched.
```

### A02. Claim Integrity Docs and Generic Gate Plan

Layer: L0/L9
Risk: Class 0 for docs, Class 2 later for gates
FC nodes: FC3 governance, L9 claim boundary
Predecessor: A00

Allowed paths:

```text
AGENTS.md
CLAUDE.md
skills/no-proven-checklist.md
.github/pull_request_template.md
handover/directives/CLAIM_INTEGRITY_MERGE_CONTRACT_2026-06-05.md
```

Instructions:

1. Port only claim-integrity doctrine, not `market_tape_shared` tests.
2. Add G1-G6 checklist for strong claims.
3. Specify that mechanism gates wait for generic ChainTape projection trait.
4. Prevent duplicate merge of #225 content by checking current main first.

Acceptance:

```bash
git diff --name-only origin/main...HEAD
git diff --check
grep -RInE 'PROVEN|DEFINITIVE|causal|world-first|TASK-PASS' \
  AGENTS.md CLAUDE.md skills/no-proven-checklist.md \
  .github/pull_request_template.md \
  handover/directives/CLAIM_INTEGRITY_MERGE_CONTRACT_2026-06-05.md || true
grep -RInE '(^|[^A-Za-z])(mod|use)[[:space:]]+market_tape_shared|[m]arket_tape_shared::' \
  AGENTS.md CLAUDE.md skills/no-proven-checklist.md \
  .github/pull_request_template.md \
  handover/directives/CLAIM_INTEGRITY_MERGE_CONTRACT_2026-06-05.md && exit 1 || true
```

Expected:

```text
Every strong-claim mention is inside a checklist or ban.
No src/**, tests/**, or evidence files.
```

### A03. Boot Trust Root Manifest Gate

Layer: L1
Risk: Class 3 floor; Class 4 if constitution authority, flowchart hash,
canonical signing payload, or genesis authority changes
FC nodes: FC2-N16, FC2-N18, FC2-N19, FC2-N21, FC3-N29
Predecessor: A01

Allowed paths:

```text
src/bin/turingos/cmd_boot.rs
src/runtime/boot_trust_root_manifest.rs
tests/constitution_tc_boot_trust_root_manifest.rs
build.rs
genesis_payload.toml
```

Current-state preflight:

```text
handover/directives/2026-06-05_A03_BOOT_TRUST_ROOT_MANIFEST_PREFLIGHT_AND_SECTION8_REQUEST.md
```

The preflight found that the planned `cmd_boot.rs`,
`src/runtime/boot_trust_root_manifest.rs`, and
`tests/constitution_tc_boot_trust_root_manifest.rs` paths do not currently
exist. The live trust-root verifier is `src/boot.rs::verify_trust_root`, with
the live boot call site in `src/main.rs` and existing witnesses in
`tests/fc_alignment_conformance.rs`, `tests/constitution_fc2_boot.rs`,
`tests/constitution_art_v3_amendment_log.rs`,
`tests/constitution_predicate_gate.rs`, and
`tests/constitution_flowchart_livenow.rs`. Therefore A03 runtime work is
blocked until the implementation authority is ratified. Docs-only preflight
work is allowed; runtime edits to trust-root / genesis / build / boot-call
authority require the per-atom §8 gate below.

Instructions:

1. Write failing tests first for manifest mismatch, constitution hash mismatch,
   predicate hash mismatch, and payload mismatch.
2. Boot validates trust root before any runtime claim.
3. No env var, test-only flag, or panic catch may bypass trust root failure.
4. PR body must explain every `build.rs` and `genesis_payload.toml` change.

Acceptance:

```bash
cargo test --test constitution_tc_boot_trust_root_manifest --no-fail-fast -- --test-threads=1
bash scripts/run_constitution_gates.sh
git diff --check
grep -RInE 'ALLOW|BYPASS|SKIP' src/bin/turingos/cmd_boot.rs src/runtime/boot_trust_root_manifest.rs tests/constitution_tc_boot_trust_root_manifest.rs || true
```

Expected:

```text
valid manifest passes.
all mismatch tests fail closed.
no bypass by env var or test-only flag.
```

Audit:

```text
Clean-context audit required.
Class 4 requires explicit per-atom §8 before implementation.
```

### A04. GitTape Physical Ledger and Single Writer

Layer: L2
Risk: Class 3 floor; Class 4 if sequencer admission, typed tx schema, or
canonical signing payload is touched
FC nodes: FC1-N1, FC1-N3, FC1-N13, FC1-N14, FC1-N15, FC2 replay
Predecessor: A01

Allowed paths:

```text
src/bottom_white/ledger/transition_ledger.rs
src/runtime/chain_tape_lease.rs
src/runtime/resume_preflight.rs
src/bottom_white/ledger/rejection_evidence.rs
src/bottom_white/cas/git_chain.rs
src/bottom_white/cas/store.rs
src/git_tape_ledger.rs
tests/tc_git_tape_ledger_hardening.rs
tests/git_tape_ledger_roundtrip.rs
tests/constitution_g1_resume.rs
tests/tb_6_verify_chaintape.rs
```

Current-state preflight:

```text
handover/directives/2026-06-05_A04_GITTAPE_PHYSICAL_LEDGER_PREFLIGHT.md
```

The preflight found that `src/git_tape_ledger.rs` is the older TDMA
`ImmutableTapeLedger` implementation. The canonical A04 implementation target
is the current OS ChainTape/L4 Git-backed writer, `Git2LedgerWriter`, in
`src/bottom_white/ledger/transition_ledger.rs`, with canonical refs under
`refs/chaintape/*` and compatibility alias `refs/transitions/main`. A04 also
crosses the outer single-writer surfaces in `src/runtime/chain_tape_lease.rs`
and `src/runtime/resume_preflight.rs`, plus the L4.E/CAS ref surfaces in
`src/bottom_white/ledger/rejection_evidence.rs` and
`src/bottom_white/cas/{git_chain.rs,store.rs}`. TDMA-only hardening is allowed
only as compatibility work and cannot claim OS L2 completion.

Instructions:

1. Port only the physical GitTape hardening core from #280.
2. Implement or preserve C1 and C2.
3. Use a single writer or OCC expected-old `GitOid` ref updates.
4. Do not swallow ref movement errors.
5. Reopen append resumes at `tn-N+1`.
6. Accepted, rejected, and pending refs are reconstructable and append-only.
7. Generated test repo runs `git fsck --full`.

Acceptance:

```bash
cargo test --test tc_git_tape_ledger_hardening --no-fail-fast -- --test-threads=1
cargo test --test git_tape_ledger_roundtrip --no-fail-fast -- --test-threads=1
cargo test --test constitution_matrix_drift --no-fail-fast
git diff --check
git -C <generated_repo> fsck --full
```

Positive controls:

```text
tampered event fails verify_integrity.
tampered ref fails verify_integrity.
concurrent stale-old-head append fails or retries without duplicate logical_t.
```

Audit:

```text
Clean-context audit required.
```

### A05. Tape Event Envelope and Projection Trait

Layer: L2/L6/L7/L9 bridge
Risk: Class 2; Class 3 if used by economy or CAS integrity
FC nodes: Art. 0.2, FC1-N1, FC1-N13, FC1-N14, FC1-N15
Predecessor: A04

Allowed paths:

```text
src/runtime/tape_event.rs
src/runtime/projection.rs
src/runtime/mod.rs
tests/tape_event_envelope_roundtrip.rs
tests/tape_projection_replay.rs
tests/constitution_headline_recompute_from_chaintape.rs
tests/constitution_router_name_matches_mechanism.rs
scripts/constitution_gates.manifest.toml
handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md
```

Current-state preflight:

```text
handover/directives/2026-06-05_A05_TAPE_EVENT_PROJECTION_PREFLIGHT.md
```

The preflight found that all new planned A05 source/test files are currently
missing except `scripts/constitution_gates.manifest.toml`. It also corrected
the allowed path set: `src/runtime/mod.rs` is needed for crate-visible runtime
modules, and `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` is needed if
A05 adds constitution gates. A05 is the generic event/projection contract used
by later economy, scheduler, and external-call atoms, so it must target the
ChainTape-L4 authority selected by A04. It must not derive completion from the
older TDMA `GitTapeLedger`, manifests, dashboards, stdout, or market-specific
structures.

Instructions:

1. Implement C1 and C3 as generic substrate, not market-specific code.
2. Add serde/roundtrip tests if local patterns allow serde.
3. Add generic claim gates only after projection trait exists.
4. Preserve any pre-existing edits in `src/runtime/mod.rs` and the matrix.
5. Include lying-manifest positive control.
6. Include softmax argmax-collapse positive control for router-name gate.

Acceptance:

```bash
cargo test --test tape_event_envelope_roundtrip --no-fail-fast -- --test-threads=1
cargo test --test tape_projection_replay --no-fail-fast -- --test-threads=1
cargo test --test constitution_headline_recompute_from_chaintape --no-fail-fast -- --test-threads=1
cargo test --test constitution_router_name_matches_mechanism --no-fail-fast -- --test-threads=1
bash scripts/run_constitution_gates.sh
cargo test --test constitution_matrix_drift --no-fail-fast
git diff --check
```

Expected:

```text
Projection recomputes from ChainTape/GitTape only.
No manifest-only proof.
No market_tape_shared dependency.
```

### A06. ExternalCall Outbox, Crash Matrix, and Orphan Sweeper

Layer: L3 and L1 boot recovery
Risk: Class 2; Class 3 if it changes production LLM/network capability paths
FC nodes: FC1-N7, FC1-N13, FC2-N22, FC2 boot/replay, FC3 logs archive
Predecessor: A04 and A05

Allowed paths:

```text
src/runtime/external_call.rs
src/runtime/tc_tape_canonical.rs
src/runtime/orphan_intent_sweeper.rs
src/runtime/mod.rs
src/bin/turingos/chat_client.rs
src/bin/turingos/cmd_llm.rs
src/bin/turingos/cmd_generate.rs
src/bin/turingos/cmd_spec.rs
src/drivers/llm_http.rs
src/web/spec.rs
src/web/generate.rs
tests/tc_external_call_records.rs
tests/tc_tape_canonical_repairs.rs
tests/external_call_orphan_sweeper.rs
tests/offline_replay_no_llm_dependency_static_check.rs
```

Current-state preflight:

```text
handover/directives/2026-06-05_A06_EXTERNAL_CALL_OUTBOX_PREFLIGHT.md
```

The preflight found that planned A06 modules/tests are missing except
`src/drivers/llm_http.rs`, but that file is not the whole production LLM surface.
The current CLI/Web production path routes through
`src/bin/turingos/chat_client.rs`, `cmd_llm.rs`, `cmd_generate.rs`,
`cmd_spec.rs`, and Web shellouts in `src/web/spec.rs` / `src/web/generate.rs`.
Existing `AttemptTelemetry` and `TerminalAbortRecord` schemas are evidence
capsules, not the planned `ExternalCallIntent -> ExternalCallTerminal` outbox.
A06 must wait for A04 ChainTape-L4 authority and A05 TapeEvent envelope before
wrapping production LLM/network calls.

Instructions:

1. Implement C4.
2. Every provider/network call writes Intent before physical call when
   possible and exactly one Terminal after.
3. Replay path is network-off and model-off.
4. Add crash cases:
   `before_intent`, `after_intent_before_provider`,
   `after_provider_before_terminal`, `provider_500`, `timeout`.
5. Boot sweeper appends `Abandoned { reason: OS_CRASH_RECOVERY }` for stale
   unclosed intents.
6. If provider may have spent tokens or money, terminal has
   `may_have_spent=true`.
7. Restart after `after_provider_before_terminal` must not reissue a physical
   provider call unless the same `logical_call_id` and `idempotency_key` are
   reused against a provider-supported idempotent endpoint; otherwise append an
   Abandoned terminal with `may_have_spent=true`.
8. Do not claim completion by wrapping only `src/drivers/llm_http.rs`; include
   the `chat_client` CLI/Web path or explicitly defer it with a failing
   inventory test.
9. Prefer CAS `ObjectType::Generic + schema_id` for request/response payloads
   unless dedicated ExternalCall CAS ObjectType variants receive explicit
   schema-risk ratification.

Acceptance:

```bash
cargo test --test tc_external_call_records --no-fail-fast -- --test-threads=1
cargo test --test tc_tape_canonical_repairs --no-fail-fast -- --test-threads=1
cargo test --test external_call_orphan_sweeper --no-fail-fast -- --test-threads=1
cargo test --test offline_replay_no_llm_dependency_static_check --no-fail-fast
cargo test distiller_in_budget --lib --no-fail-fast
git diff --check
```

Expected:

```text
clean halt requires pending_intents == 0.
replay does not call network or LLM.
orphan intent sweep is tape-visible, not memory cleanup.
after_provider_before_terminal recovery does not duplicate the physical call.
same logical_call_id/idempotency_key is reused for any permitted retry.
non-idempotent or uncertain provider completion becomes Abandoned
may_have_spent=true.
call-site inventory covers chat_client, cmd_llm, cmd_generate, cmd_spec,
web shellouts, and llm_http.
```

### A07. Agent Process Model and View Shielding

Layer: L4
Risk: Class 2
FC nodes: FC1-N5, FC1-N6, FC1-N7, FC3-N31, FC3 shielding invariants
Predecessor: A05

Allowed paths:

```text
src/runtime/tc_agent_view.rs
src/runtime/mod.rs
src/runtime/real5_roles.rs
src/runtime/prompt_capsule.rs
src/runtime/attempt_telemetry.rs
src/runtime/build_session_view.rs
src/runtime/audit_assertions.rs
src/runtime/test_run.rs
src/runtime/rejection_capsule.rs
src/web/artifact_bundle.rs
src/sdk/prompt.rs
src/sdk/your_position.rs
tests/tc_agent_view_shielding.rs
tests/constitution_real5_role_scoped_view.rs
tests/constitution_real5_prompt_capsule_v2.rs
tests/hidden_oracle_not_in_generation_prompt_bytes.rs
tests/hidden_oracle_set_cid_not_in_build_session_view.rs
tests/build_session_view_does_not_expose_private_diagnostic_cid.rs
tests/build_session_view_does_not_expose_test_scenario_set_cid.rs
tests/artifact_bundle_serve_rejects_private_diagnostic_cid.rs
tests/rejection_private_diagnostic_not_in_http_body.rs
```

Current-state preflight:

```text
handover/directives/2026-06-05_A07_AGENT_VIEW_SHIELDING_PREFLIGHT.md
```

The preflight found that `src/runtime/tc_agent_view.rs` and
`tests/tc_agent_view_shielding.rs` are missing. Existing role-view,
PromptCapsuleV2, BuildSessionView, hidden-oracle, and private-diagnostic tests
are useful shielding witnesses, but they are not a generic AgentView derived
from an allowed ChainTape prefix. A07 must wait for A05 TapeEvent/projection or
explicitly remain docs/test-shape work. Existing dirty edits in
`src/runtime/mod.rs`, `src/runtime/audit_assertions.rs`, and the hidden-oracle /
private-diagnostic tests must be preserved. Web-facing shielding tests must run
with `--features web`.

Instructions:

1. Implement C5 without exposing hidden verifier/test body.
2. Raw stderr stays in CAS/audit-only surfaces.
3. Price broadcast is prefix-bound; no future price leakage.
4. Sibling private chains are invisible unless reduced into public tape events.
5. Positive controls inject hidden text and must fail.
6. Do not change PromptCapsuleV2 schema shape without explicit Class 4
   ratification.

Acceptance:

```bash
cargo test --test tc_agent_view_shielding --no-fail-fast -- --test-threads=1
cargo test --test constitution_real5_role_scoped_view --no-fail-fast
cargo test --test constitution_real5_prompt_capsule_v2 --no-fail-fast
cargo test --test hidden_oracle_not_in_generation_prompt_bytes --no-fail-fast
cargo test --test hidden_oracle_set_cid_not_in_build_session_view --no-fail-fast
cargo test --features web --test build_session_view_does_not_expose_private_diagnostic_cid --no-fail-fast
cargo test --features web --test build_session_view_does_not_expose_test_scenario_set_cid --no-fail-fast
cargo test --features web --test artifact_bundle_serve_rejects_private_diagnostic_cid --no-fail-fast
cargo test --features web --test rejection_private_diagnostic_not_in_http_body --no-fail-fast
git diff --check
```

Expected:

```text
hidden_leak_count == 0 in fixtures.
positive-control leakage test fails before fix and passes after fix.
AgentView is prefix-bound and remains a derived view.
HTTP artifact/rejection routes do not expose private diagnostic CIDs.
```

### A08. PredicateReceipt and LeanJudge Axiom Gate

Layer: L5 and L8 Lean workload verifier
Risk: Class 2; Class 3 if receipt impacts admission authority
FC nodes: FC1-N11, FC1-N12, FC1-N14, FC1-N15
Predecessor: A05

Allowed paths:

```text
src/runtime/predicate_receipt.rs
src/runtime/mod.rs
tests/predicate_receipt_replay.rs
src/judges/lean_judge.rs
src/judges/mod.rs
tests/lean_judge_axiom_gate.rs
src/top_white/predicates/registry.rs
src/runtime/attempt_telemetry.rs
src/runtime/verification_result.rs
src/runtime/predicate_registry_loader.rs
tests/constitution_predicate_registry_binding.rs
tests/constitution_predicate_binding_activation.rs
tests/constitution_predicate_registry_replay.rs
tests/constitution_predicate_registry_immutability.rs
tests/constitution_predicate_result_wire_freeze.rs
tests/tb_18r_lean_verdict_kind_consistency.rs
tests/tb_18r_lean_result_cas_resolves.rs
tests/tb_18r_lean_reject_in_l4e.rs
```

Current-state preflight:

```text
handover/directives/2026-06-05_A08_PREDICATE_RECEIPT_LEAN_JUDGE_PREFLIGHT.md
```

The preflight found that all four originally planned A08 files are missing.
The current repo's equivalent Lean/predicate surfaces are
`src/top_white/predicates/registry.rs`, `src/runtime/attempt_telemetry.rs`,
`src/runtime/verification_result.rs`, and predicate-registry replay/binding
tests. `src/judges/lean_judge.rs` does not exist. Current
`VerificationResult::from_lean_run` treats `lean_exit_code == 0` as verified,
and existing source scans for `sorry` / `admit` / unsafe patterns are not a
`#print axioms` whitelist gate. Standalone LeanJudge tests can be Class 2; any
wire-up into predicate pass/fail authority must be classified separately and may
need per-atom §8.

Instructions:

1. Implement C6.
2. Port only LeanJudge verifier soundness from #283.
3. Do not port `lean_market_agent.rs`.
4. Do not port `market_tape_shared.rs`.
5. Lean exit-0 is not enough; `#print axioms` gate must fail closed.
6. `sorry`, `admit`, and unsafe shortcuts are rejected by source scan or
   existing local verifier pattern.
7. Include `src/runtime/mod.rs` and `src/judges/mod.rs` for new public modules.
8. Use `ObjectType::Generic + schema_id` for PredicateReceipt unless a
   dedicated CAS enum variant is explicitly ratified.
9. Keep A08.1 standalone judge tests separate from A08.3 predicate-authority
   wiring.

Acceptance:

```bash
cargo test lean_judge --lib --no-fail-fast
cargo test --test lean_judge_axiom_gate --no-fail-fast -- --test-threads=1
cargo test --test predicate_receipt_replay --no-fail-fast -- --test-threads=1
cargo test --test constitution_predicate_registry_binding --no-fail-fast -- --test-threads=1
cargo test --test constitution_predicate_binding_activation --no-fail-fast -- --test-threads=1
cargo test --test constitution_predicate_registry_replay --no-fail-fast -- --test-threads=1
cargo test --test constitution_predicate_registry_immutability --no-fail-fast
cargo test --test constitution_predicate_result_wire_freeze --no-fail-fast
cargo test --test tb_18r_lean_verdict_kind_consistency --no-fail-fast
cargo test --test tb_18r_lean_result_cas_resolves --no-fail-fast
cargo test --test tb_18r_lean_reject_in_l4e --no-fail-fast
git diff --check
grep -RIn 'market_tape_shared\\|lean_market_agent' src/judges src/runtime tests && exit 1 || true
```

Expected:

```text
non-whitelisted axiom -> axiom_rejected.
#print axioms failure -> axiom_rejected.
whitelisted theorem -> verified only after kernel compile success.
PredicateReceipt replay reconstructs verdict.
Lean exit 0 alone does not bypass axiom or forbidden-source gates.
No sequencer/typed-tx/CAS schema authority change occurs without explicit
ratification.
```

### A09. Economy Service v0

Layer: L6
Risk: Class 3
FC nodes: Art. 0.2, FC2-N21, FC2-N28, FC1-N11/N12 predicate boundary
Predecessor: A05 and A08
Preflight:
`handover/directives/2026-06-05_A09_ECONOMY_SERVICE_PREFLIGHT.md`

Allowed paths:

```text
src/economy/mod.rs
src/economy/events.rs
src/economy/projections.rs
src/economy/conservation.rs
src/economy/settlement.rs
src/economy/price_broadcast.rs
tests/economy_tape_replay.rs
tests/economy_conservation.rs
tests/economy_predicate_price_blindness.rs
tests/economy_no_parallel_ledger.rs
```

Preflight correction:

```text
A09 is not immediately implementable. Current repo has production
EconomicState and useful economy witnesses, but not the planned A09 C7
EconomyEvent projection surface. A09 must wait for A05 TapeEvent/projection
authority and A08 PredicateReceipt shape. Existing src/economy/ledger.rs and
EscrowVault must not become a second economy ledger. src/economy/mod.rs is in
scope only to export new A09 modules.
```

Instructions:

1. Implement C7.
2. Every wallet, market book, price index, reputation, escrow, settlement
   state is derived from C1/C3.
3. No root-level market ledger module.
4. No mint outside genesis/on_init.
5. Conservation uses integer math only.
6. No payout without `PredicateReceipt::Pass`.
7. Price broadcast references a tape prefix.

Acceptance:

```bash
cargo test --test economy_tape_replay --no-fail-fast -- --test-threads=1
cargo test --test economy_conservation --no-fail-fast -- --test-threads=1
cargo test --test economy_predicate_price_blindness --no-fail-fast -- --test-threads=1
cargo test --test economy_no_parallel_ledger --no-fail-fast -- --test-threads=1
cargo test --test constitution_economy_gate --no-fail-fast -- --test-threads=1
cargo test --test tb_14_price_index --no-fail-fast -- --test-threads=1
cargo test --test constitution_router_price_quote --no-fail-fast -- --test-threads=1
bash scripts/run_constitution_gates.sh
git diff --check
grep -RInE 'f32|f64' src/economy tests/economy_* && exit 1 || true
grep -RIn 'market_tape_shared' src tests && exit 1 || true
```

Expected:

```text
sum(wallet_balances) + escrow + open_positions == minted_total.
price can route, but cannot change predicate verdict.
replay reconstructs exact wallet/price/settlement state.
```

Audit:

```text
Clean-context audit required.
Obligation witness required if this atom claims to close any must obligation.
```

### A10. Projection Cache With GitOid Watermark

Layer: L6/L7 performance guardrail
Risk: Class 3 if economy projections are cached
FC nodes: Art. 0.2, FC1-N1, FC1-N13, FC1 dashboard-not-source invariant
Predecessor: A09
Preflight:
`handover/directives/2026-06-05_A10_PROJECTION_CACHE_PREFLIGHT.md`

Allowed paths:

```text
src/runtime/mod.rs
src/economy/projections.rs
src/economy/mod.rs
src/runtime/projection.rs
tests/economy_projection_cache_watermark.rs
tests/projection_cache_not_source_of_truth.rs
```

Preflight correction:

```text
A10 is preflight-only until A04/A05 and A09 exist. The runtime cache key is
projection id + projection version + current ChainTape/L4 GitOid. A cache hit
is legal only when derived_from_tape_head == current head. Stale cache can be a
delta-apply starting point only with ancestry proof; otherwise full replay is
mandatory. Cache cannot be canonical input to predicates, settlement, sequencer
admission, typed tx construction, ChainTape/CAS verification, or reports.
```

Instructions:

1. Implement C3 cache semantics.
2. Cache key is exactly the GitTape HEAD Oid plus projection id/version.
3. Stale cache uses delta apply from cached head to current head.
4. Full replay is available for repair and tests.
5. Cache cannot be accepted as canonical input to predicates.

Acceptance:

```bash
cargo test --test economy_projection_cache_watermark --no-fail-fast -- --test-threads=1
cargo test --test projection_cache_not_source_of_truth --no-fail-fast -- --test-threads=1
cargo test --test economy_tape_replay --no-fail-fast -- --test-threads=1
cargo test --test economy_conservation --no-fail-fast -- --test-threads=1
cargo test --test tb_18r_cas_reload_split_brain --no-fail-fast -- --test-threads=1
bash scripts/run_constitution_gates.sh
cargo test --test constitution_matrix_drift --no-fail-fast
git diff --check
grep -RInE 'f32|f64' src/economy tests/economy_* && exit 1 || true
```

Expected:

```text
cache hit only when derived_from_tape_head == current head.
stale cache updates by replaying deltas.
tampered cache is ignored or fails closed.
dropping cache does not change replay result.
```

### A11. Scheduler and Search Policies

Layer: L7
Risk: Class 2; Class 3 if policy allocates real budget/economy state
FC nodes: FC1-N7, FC1-N13, FC1 routing, L6 price projection boundary
Predecessor: A09 and A10

Allowed paths:

```text
src/scheduler/mod.rs
src/scheduler/policy.rs
src/scheduler/non_local_tree.rs
src/scheduler/softmax.rs
src/scheduler/parallel_lanes.rs
src/scheduler/forced_loop.rs
src/runtime/mod.rs
src/runtime/agent_scheduler.rs
src/sdk/actor.rs
src/state/price_index.rs
src/runtime/chain_tape_lease.rs
src/bottom_white/ledger/transition_ledger.rs
src/bin/turingos/cmd_generate.rs
src/web/generate.rs
tests/constitution_g5_scheduler.rs
tests/constitution_g6_observe_only.rs
tests/scheduler_policy_trace.rs
tests/scheduler_softmax_distribution.rs
tests/scheduler_parallel_isolation.rs
tests/scheduler_forced_loop_bounds.rs
```

Current-state preflight:

```text
handover/directives/2026-06-05_A11_SCHEDULER_SEARCH_PREFLIGHT.md
```

The preflight found that all planned `src/scheduler/*` modules and
`tests/scheduler_*` tests are currently missing. Existing
`src/runtime/agent_scheduler.rs` is an observe-only trace helper, not the C8
`SchedulerDecisionEvent` authority. Existing CLI/Web parallel worker fan-out is
not the planned `parallel_lanes` isolation contract. A11 must wait for A04
ChainTape authority, A05 TapeEvent/projection, A09 economy projection, and A10
projection-cache watermark semantics before claiming replayable scheduler
truth.

Instructions:

1. Implement C8.
2. Every routing decision writes a `SchedulerDecisionEvent`.
3. Softmax cannot silently become argmax.
4. Parallel lanes cannot share private error context.
5. Forced loop has max iterations, max tokens, and max wall-clock.
6. Price is optional policy input, never predicate authority.
7. Candidate sets and policy input bundles are CAS-backed and bound into the
   decision event; replay must not depend on memory-only scheduler state.

Acceptance:

```bash
cargo test --test scheduler_policy_trace --no-fail-fast -- --test-threads=1
cargo test --test scheduler_softmax_distribution --no-fail-fast -- --test-threads=1
cargo test --test scheduler_parallel_isolation --no-fail-fast -- --test-threads=1
cargo test --test scheduler_forced_loop_bounds --no-fail-fast -- --test-threads=1
cargo test --test constitution_g5_scheduler --no-fail-fast
cargo test --test constitution_g6_observe_only --no-fail-fast
git diff --check
```

Expected:

```text
equal prices distribute over >= 3/5 candidates in deterministic fixture.
argmax-collapse positive control fails.
every decision is reconstructable from tape prefix.
candidate_set_cid and policy_input_bundle_hash are sufficient to rebuild the
candidate set without memory-only state.
observe-only scheduler traces remain non-binding.
```

### A12. Universal Machine Witnesses

Layer: L2-L7 witness suite
Risk: Class 2
FC nodes: FC1 full loop, FC2 boot/replay/halt, FC3 logs/archive feedback
Predecessor: A04-A11
Preflight:
`handover/directives/2026-06-05_A12_UNIVERSAL_MACHINE_WITNESSES_PREFLIGHT.md`

Allowed paths:

```text
src/runtime/mod.rs
src/runtime/tc_universal_witness.rs
tests/tc_universal_witness_counter_machine.rs
tests/tc_universal_witness_branching.rs
tests/tc_universal_witness_external_call.rs
tests/tc_universal_witness_market.rs
tests/tc_universal_witness_agent_view.rs
tests/tc_universal_witness_self_bootstrap.rs
```

Preflight correction:

```text
A12 is a witness suite, not the OS substrate itself. All planned A12 files are
missing. Existing constitution_flowchart_livenow tests are intentionally narrow
and do not claim full FC1/FC2/FC3 liveness. W6 self-bootstrap remains
proposal-only unless a future FC3 runtime authority atom is separately
ratified. The original A12 text named W5 agent-view shielding but omitted the
dedicated `tests/tc_universal_witness_agent_view.rs` path and acceptance command;
this correction adds both.
```

Instructions:

1. W1 counter machine witness.
2. W2 branch-and-reject witness.
3. W3 external-call replay witness.
4. W4 market-settlement witness.
5. W5 agent-view-shielding witness.
6. W6 self-improvement/ArchitectAI proposal witness, proposal-only unless
   runtime authority exists.

Acceptance:

```bash
cargo test --test tc_universal_witness_counter_machine --no-fail-fast -- --test-threads=1
cargo test --test tc_universal_witness_branching --no-fail-fast -- --test-threads=1
cargo test --test tc_universal_witness_external_call --no-fail-fast -- --test-threads=1
cargo test --test tc_universal_witness_market --no-fail-fast -- --test-threads=1
cargo test --test tc_universal_witness_agent_view --no-fail-fast -- --test-threads=1
cargo test --test tc_universal_witness_self_bootstrap --no-fail-fast -- --test-threads=1
cargo test --test constitution_flowchart_livenow --no-fail-fast -- --test-threads=1
cargo test --test offline_replay_no_llm_dependency_static_check --no-fail-fast
cargo test --test replay_verifies_all_cross_cid_references_resolve --no-fail-fast
cargo test --features web --test build_session_view_does_not_expose_private_diagnostic_cid --no-fail-fast
cargo test --features web --test artifact_bundle_serve_rejects_private_diagnostic_cid --no-fail-fast
cargo test --features web --test rejection_private_diagnostic_not_in_http_body --no-fail-fast
git diff --check
```

Expected:

```text
replay byte-identical.
tamper test fails.
no network during replay.
accepted transitions have PredicateReceipt PASS.
rejected transitions remain on rejected tape.
```

### A13. Agentic OS v0 E2E CLI

Layer: L1-L9 integrated smoke
Risk: Class 2; Class 3 if real provider/economy/capability calls are enabled
FC nodes: FC1 full loop, FC2 boot/halt/replay, FC3 log archive
Predecessor: A03-A12
Preflight:
`handover/directives/2026-06-05_A13_AGENTIC_OS_E2E_CLI_PREFLIGHT.md`

Allowed paths:

```text
src/bin/turingos.rs
src/bin/turingos/cmd_boot.rs
src/bin/turingos/cmd_os.rs
src/runtime/mod.rs
src/runtime/os_run.rs
tests/os_boot_to_replay_e2e.rs
tests/os_market_settlement_e2e.rs
tests/os_agent_view_e2e.rs
fixtures/os/hello_agentic_task.json
```

Preflight correction:

```text
A13 is not implementable until A03-A12 exist. Current CLI has replay, verify
chaintape, generate, llm, and spec-adjacent commands, but no boot command and no
os command family. Existing generate/replay smoke tests must not be treated as
OS v0 E2E. The first A13 run is network-off and cannot claim solve-rate,
benchmark success, or market victory.
```

CLI contract:

```bash
turingos boot --verify-manifest
turingos os run --task fixtures/os/hello_agentic_task.json --policy single_tree --market on --network off
turingos os replay --run-dir <run-dir>
turingos os audit --run-dir <run-dir>
```

Instructions:

1. First E2E runs network-off.
2. Use mock provider or deterministic fixture for the first ship.
3. Produce run manifest, GitTape repo, replay report, predicate receipts,
   external call receipts, economy projection, and agent view audit.
4. Each derived artifact entry in the run manifest carries path, content hash or
   CID, `derived_from_tape_head`, CAS root when applicable, and replay recipe.
5. Do not claim solve-rate or benchmark success.

Acceptance:

```bash
cargo test --test os_boot_to_replay_e2e --no-fail-fast -- --test-threads=1
cargo test --test os_market_settlement_e2e --no-fail-fast -- --test-threads=1
cargo test --test os_agent_view_e2e --no-fail-fast -- --test-threads=1
cargo test --test cli_replay_smoke --no-fail-fast
cargo test --test cli_verify_chaintape_smoke --no-fail-fast
cargo test --workspace --no-fail-fast
bash scripts/run_constitution_gates.sh
cargo test --test constitution_matrix_drift --no-fail-fast
git diff --check
```

Expected artifacts:

```text
run_manifest.json
git_tape_repo/
replay_report.json
predicate_receipts.jsonl
external_call_receipts.jsonl
economy_projection.json
agent_view_audit.json
```

Expected assertions:

```text
git fsck --full passes.
replay_report.deterministic == true.
external_call.pending == 0.
economy_projection.conservation_ok == true.
every derived artifact has derived_from_tape_head == final ChainTape/L4 head.
every derived artifact has content hash or CID and replay recipe.
hidden_leak_count == 0.
unsupported_task_success_claim_count == 0.
```

### A14. Workload Adapters, Market Research, and Benchmark Boundary

Layer: L8/L9
Risk: Class 2 for adapters; Class 3 if real budget/economy experiments run
FC nodes: L8 workload adapter boundary, L9 evidence/report boundary, FC3 audit
Predecessor: A13
Preflight:
`handover/directives/2026-06-05_A14_WORKLOAD_ADAPTER_BOUNDARY_PREFLIGHT.md`

Allowed paths:

```text
experiments/market_p1_lean_router/
experiments/minif2f_v4/
src/workloads/mod.rs
src/workloads/lean/
src/workloads/swebench/
src/workloads/market_research/
src/workloads/benchmark_boundary.rs
tests/workload_adapter_claim_boundary.rs
tests/market_preregistration_contract.rs
handover/reports/<scoped-market-or-benchmark-report>.md
```

Preflight correction:

```text
A14 is a workload boundary atom, not kernel authority. Parent-plan adapter
paths/tests are missing. Existing market-autonomy, Polymarket, MiniF2F, and
benchmark witnesses are useful quarry, but they are not A14 adapter authority.
No TASK-PASS, benchmark solve-rate, or market victory headline is allowed
without verifier-backed and preregistered evidence.
```

Instructions:

1. Keep benchmark adapters in user workload path, not kernel authority path.
2. Mark every adapter result as one of:
   `real verifier-backed`, `structural smoke`, `participation canary`.
3. Market Track A: price-as-router, equal budget and controls.
4. Market Track B: scarce-budget allocation with heterogeneous specialists.
5. Market Track C: accountability, Sybil, settlement, challenge/slash.
6. Market Track D: long-horizon OS economy.
7. Every track requires preregistered MDE, sample size, ablations, equal budget,
   route decisions on tape, replay, hidden verifier shielding, and clean-context
   audit before headline claims.

Acceptance:

```bash
cargo test --test workload_adapter_claim_boundary --no-fail-fast -- --test-threads=1
cargo test --test market_preregistration_contract --no-fail-fast -- --test-threads=1
cargo test --test constitution_benchmark_manifest --no-fail-fast
cargo test --test constitution_market_autonomy_research_envelope --no-fail-fast
cargo test --test constitution_real11_claim_boundary --no-fail-fast
cargo test --test constitution_matrix_drift --no-fail-fast
if grep -RInE 'TASK-PASS|PROVEN|DEFINITIVE|causal|market beats' \
  experiments src/workloads handover/reports; then
  echo 'strong workload claim text requires verifier-backed allowlist gate'
  exit 1
fi
git diff --check
```

Expected:

```text
No TASK-PASS without verifier-backed TASK-PASS.
No market victory headline without preregistered evidence.
Raw evidence is archived by hash manifest, not dumped into main.
```

## 7. Extraction Map From Audit Snapshots

From PR #280:

```text
Use:
  Path B decision material
  GitTape/ChainTape hardening concepts
  ExternalCall outbox concepts
  AgentView shielding tests
  Boot trust root manifest concepts
  Universal witness ideas after substrate lands

Do not use:
  monolithic branch merge
  bulk task packets as one PR
  raw evidence blobs
  benchmark CLIs before OS substrate
  TASK-PASS wording unsupported by verifier-backed tasks
```

From PR #283:

```text
Use:
  LeanJudge axiom gate
  no-PROVEN / claim integrity gates
  market methodology caveats
  P1 scope-correction report language
  MarketEvent/derive/replay ideas only after remapping to GitTape

Do not use:
  market_tape_shared.rs as independent root module
  lean_market_agent.rs in the verifier atom
  8.7GB raw tape
  broad claim that market economics failed
  price-as-router result as economy-level refutation
```

## 8. Drift Controls

Every atom PR must include these mechanical checks:

```bash
set -euo pipefail
git fetch origin
git diff --name-only origin/main...HEAD
git diff --check
git diff --cached --check
git fetch origin pull/280/head:refs/audit-snapshots/pr-280
git fetch origin pull/283/head:refs/audit-snapshots/pr-283
test "$(git rev-parse refs/audit-snapshots/pr-280)" = e1605911c883aea4ce842b7fee7d41bd0448f947
test "$(git rev-parse refs/audit-snapshots/pr-283)" = 4cfbc41e23042d8ff496ef775dc91e5af2c0f9d8
for oid in \
  e1605911c883aea4ce842b7fee7d41bd0448f947 \
  4cfbc41e23042d8ff496ef775dc91e5af2c0f9d8
do
  if git merge-base --is-ancestor "$oid" HEAD; then
    echo "forbidden audit snapshot ancestor: $oid"
    exit 1
  fi
done
cargo fmt --check
cargo test --test constitution_matrix_drift --no-fail-fast
bash scripts/run_constitution_gates.sh
claim_hits=$(grep -RInE 'PROVEN|DEFINITIVE|causal|world-first|TASK-PASS' \
  handover src tests AGENTS.md CLAUDE.md .github \
  | grep -vE 'no-proven-checklist|CLAIM_INTEGRITY|pull_request_template|AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT|P1_REALVALUE_SCOPE_CORRECTION|A14_WORKLOAD_ADAPTER_BOUNDARY_PREFLIGHT' || true)
test -z "$claim_hits" || { printf '%s\n' "$claim_hits"; exit 1; }
grep -RInE 'api[_-]?key|Authorization|Bearer|SECRET|TOKEN' \
  . --exclude-dir target --exclude-dir .git || true
find handover/evidence -type f \
  \\( -name 'run_*.log' -o -name 'CRACK_*.txt' -o -path '*repo_*' -o -path '*cas_*' \\) -print
```

Additional checks by domain:

```text
GitTape:
  git -C <generated_repo> fsck --full
  stale-old-head append positive control

ExternalCall:
  replay network-off positive control
  orphan intent boot sweep positive control

AgentView:
  hidden text injection positive control

Economy:
  integer-only grep
  conservation invariant
  no price-overrides-predicate test

Scheduler:
  softmax distribution gate
  argmax-collapse positive control

Workload adapters:
  claim-boundary enum test
  no TASK-PASS grep unless verifier-backed fixture proves it
```

## 9. Full Plan Acceptance

The full Agentic OS pivot is accepted only when all of these are true:

```text
1. A00-A14 are merged through PR-only workflow from clean origin/main branches.
2. No PR is based on #280 or #283 by merge.
3. All Class 2+ atoms have real evidence before clean-context audit.
4. All Class 3/4 atoms have clean-context audit verdict with no unresolved violation.
5. All Class 4 atoms have explicit per-atom §8 ratification before implementation or ship.
6. cargo test --workspace --no-fail-fast exits 0 on the final OS v0 branch.
7. bash scripts/run_constitution_gates.sh exits 0.
8. cargo test --test constitution_matrix_drift --no-fail-fast exits 0.
9. GitTape generated repo passes git fsck --full.
10. OS v0 run produces run_manifest.json, git_tape_repo/, replay_report.json,
    predicate_receipts.jsonl, external_call_receipts.jsonl,
    economy_projection.json, and agent_view_audit.json.
11. replay_report.deterministic == true.
12. external_call.pending == 0 after halt and boot orphan sweep.
13. economy_projection.conservation_ok == true.
14. hidden_leak_count == 0.
15. unsupported_task_success_claim_count == 0.
16. No wallet/market/price/search/dashboard/report path is a source of truth.
17. No f32/f64 appears in economy money/conservation paths.
18. No raw hidden tests, raw stderr, private diagnostics, or bulk evidence logs
    are exposed to ordinary AgentView or mainline reports.
19. Every market or benchmark headline states its exact claim boundary.
20. Separate obligation witness reads `OBLIGATIONS.md` and emits exactly
    `OBL-ALL-CLOSED` before any PR or release claims the wave/repository is
    done. If it emits `OBL-OPEN-MUST`, `OBL-EVIDENCE-MISSING`, or
    `OBL-BLOCKER-UNVERIFIED`, the plan may remain a planning packet but cannot
    be shipped as complete.
```

Final ship command block:

```bash
git fetch origin
git diff --name-only origin/main...HEAD
git diff --check
cargo fmt --check
cargo test --workspace --no-fail-fast
bash scripts/run_constitution_gates.sh
cargo test --test constitution_matrix_drift --no-fail-fast
```

Final machine witness supplement:

```bash
set -euo pipefail

# PR-only / clean branch evidence.
test "$(git branch --show-current)" != "main"
gh pr view --json number,state,isDraft,headRefName,baseRefName,files

# Audit snapshot ancestry is pinned and fail-closed.
git fetch origin pull/280/head:refs/audit-snapshots/pr-280
git fetch origin pull/283/head:refs/audit-snapshots/pr-283
test "$(git rev-parse refs/audit-snapshots/pr-280)" = e1605911c883aea4ce842b7fee7d41bd0448f947
test "$(git rev-parse refs/audit-snapshots/pr-283)" = 4cfbc41e23042d8ff496ef775dc91e5af2c0f9d8
for oid in \
  e1605911c883aea4ce842b7fee7d41bd0448f947 \
  4cfbc41e23042d8ff496ef775dc91e5af2c0f9d8
do
  if git merge-base --is-ancestor "$oid" HEAD; then
    echo "forbidden audit snapshot ancestor: $oid"
    exit 1
  fi
done

# Evidence before audit, Class 3/4 audit, and §8 proof are checked by packet.
test -s handover/evidence/<atom-run>/command-output.txt
test -s handover/audits/<clean-context-audit>.md
grep -RIn 'NO-VIOLATION\|PROCEED' handover/audits/<clean-context-audit>.md
grep -RIn 'APPROVE-A[0-9][0-9].*SECTION8\|§8' handover/directives handover/tracer_bullets

# Generated GitTape and OS v0 artifacts.
git -C <generated_repo> fsck --full
test -s <run-dir>/run_manifest.json
test -s <run-dir>/replay_report.json
jq -e '.deterministic == true' <run-dir>/replay_report.json
jq -e '.external_call.pending == 0' <run-dir>/run_manifest.json
jq -e '.economy_projection.conservation_ok == true' <run-dir>/run_manifest.json
jq -e '. as $m | all($m.derived_artifacts[]; .derived_from_tape_head == $m.final_tape_head)' <run-dir>/run_manifest.json
jq -e 'all(.derived_artifacts[]; (.content_hash_or_cid | length) > 0 and (.replay_recipe | length) > 0)' <run-dir>/run_manifest.json

# Source-of-truth, money, leakage, claim, and obligation blockers.
grep -RInE '(^|[^A-Za-z])(mod|use)[[:space:]]+market_tape_shared|[m]arket_tape_shared::' src tests && exit 1 || true
grep -RInE 'f32|f64' src/economy tests/economy_* && exit 1 || true
grep -RInE 'raw stderr|hidden oracle|private diagnostic' src tests handover/reports && exit 1 || true
claim_hits=$(grep -RInE 'PROVEN|DEFINITIVE|causal|world-first|TASK-PASS|market beats' \
  handover src tests AGENTS.md CLAUDE.md .github \
  | grep -vE 'no-proven-checklist|CLAIM_INTEGRITY|pull_request_template|AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT|P1_REALVALUE_SCOPE_CORRECTION|A14_WORKLOAD_ADAPTER_BOUNDARY_PREFLIGHT' || true)
test -z "$claim_hits" || { printf '%s\n' "$claim_hits"; exit 1; }
test -s handover/audits/<obligation-witness>.md
grep -x 'OBL-ALL-CLOSED' handover/audits/<obligation-witness>.md
```

Final witness packet contains:

```text
task brief
risk class per atom
FC nodes per atom
current diff or PR range
evidence paths
acceptance commands and exact outputs
audit reports
OBLIGATIONS.md reconciliation
separate obligation witness verdict: OBL-ALL-CLOSED, or the wave cannot be
claimed complete
```

Closed witness verdict set:

```text
NO-VIOLATION
VIOLATION-FOUND <clause> <file>:<line>
RECONSTRUCTION-FAILURE <which-path>
SECOND-SOURCE-DRIFT <which-derived-view>
```

## 10. Directives To Sub-Agents

Codex directive:

```text
Do not merge or continue #280 as production.
Cut small PRs from latest origin/main.
Start with A00/A01/A04/A06/A07 according to dependency order.
Do not include benchmark CLIs, SWE-bench adapters, task packets, raw evidence,
or provider price as kernel budget authority.
Every PR must state OS layer, risk class, exact file list, source-of-truth
claim, targeted tests, constitution gate output, and clean-context audit if
Class 2+.
```

Claude directive:

```text
Do not merge or continue #283 as production.
P1 is valuable but scoped.
Do not say agent market economics failed.
Port LeanJudge axiom gate separately.
Port claim-integrity docs separately.
Retarget mechanism gates to generic ChainTape/GitTape projections after A05.
Extract market ideas into economy projections only; never merge
market_tape_shared.rs as source of truth.
```

## 11. What This Plan Does Not Claim

```text
It does not claim TuringOS solves SWE-bench, Lean, OSWorld, WebArena, or ToolBench.
It does not claim price beats controls.
It does not claim market economy is proven.
It does not close existing AMBER rows by writing this document.
It does not replace per-atom §8 authorization.
It does not replace real evidence, tests, or clean-context audit.
```

This file is the drift fence. The substrate is still built only by the atoms,
their tests, their evidence, and their audits.
