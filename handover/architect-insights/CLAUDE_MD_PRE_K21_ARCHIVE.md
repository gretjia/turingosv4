# Pre-K-2.1 Archive of CLAUDE.md (909 lines)

This is the historical full version of CLAUDE.md before K-2.1 slim (target ≤150 lines).
See current CLAUDE.md for active operating rules. Most rules now live in AGENTS.md;
CLAUDE.md is now Claude-Code-specific adapter.

---

# TuringOS v4 — CLAUDE.md

@AGENTS.md

This file is the Claude Code adapter and historical operating-law expansion for
this repository. The shared cross-agent harness contract lives in `AGENTS.md`;
Claude-specific memory, hooks, and workflow detail may live here. If this file
duplicates `AGENTS.md`, treat `AGENTS.md` as the shared router and use this file
for Claude-only mechanics.

## 0. Identity

TuringOS is a tape-first constitutional operating system for LLM / AGI agents.

Current primary mission:

- Lean / MiniF2F formal proof tasks
- ChainTape-first runtime
- Constitutional Harness Engineering

This repository is not an ordinary agent framework and not a benchmark wrapper.
It is an operating substrate where black-box agents can only affect the world through white-box tape, predicates, tools, signatures, and economic discipline.

The project exists to instantiate the Turing-machine discipline:

```
paper      = ChainTape / CAS / state ledger
pencil     = WorkTx / write tool / externalized proposal
rubber     = L4.E / rejection / revert / compensation / EvidenceCapsule
discipline = predicates / constitution gates / system-only tx / economic conservation
person     = black-box Agent
```

If meaningful activity is not on tape, it is not a TuringOS run.

***

## 1. Supreme Source-of-Truth Order

Read and obey in this order:

1. `constitution.md`
2. the three constitution flowcharts
3. ChainTape + CAS evidence
4. `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
5. `handover/alignment/TRACE_FLOWCHART_MATRIX.md`
6. `handover/ai-direct/LATEST.md`
7. `handover/tracer_bullets/TB_LOG.tsv`
8. current TB charter / directive / ratification
9. dashboard / reports / README files

Hierarchy:

```
constitution > flowcharts > ChainTape/CAS > executable gates > reports
```

Dashboard, stdout, evaluator counters, README, smoke summaries, audit text, and H-VPPU reports are not source of truth.
They are materialized views or evidence packaging.

If a report contradicts ChainTape/CAS, trust ChainTape/CAS.
If ChainTape/CAS contradict constitution gates, stop.

***

## 2. Prime Operating Mode — Constitutional Harness Engineering

This supersedes Atomic Agentic Engineering.

### 2.1 Required order

1. Constitution harness as executable tests
2. Minimal real run exercising tape
3. Implementation until harness green
4. External audit after evidence exists
5. Documentation packages proof, never substitutes for proof

Forbidden old loop:

```
charter -> atom -> self-audit -> external audit -> more docs -> delayed test
```

Required loop:

```
constitution gate -> real run -> debug -> fix -> rerun -> audit -> ship
```

If real evidence fails, return to implementation.
Do not spend audit cycles on a failing harness.

### 2.2 No tape, no test

The following are not TuringOS evidence by themselves:

- stdout logs
- private evaluator counters
- human-readable dashboard
- LLM self-report
- final proof only
- post-hoc README
- unanchored JSON
- memory-only preseed
- global latest pointer

A valid run must be reconstructable from:

```
genesis_report
+ ChainTape
+ CAS
+ agent registry
+ system pubkeys
+ replay/audit verifier
```

***

## 3. The Three Flowchart Gates

Every non-trivial TB must declare which flowchart gates it touches.

### 3.1 FC1 — Runtime Loop Gate

Canonical loop:

```
Q_t
-> rtool / scoped context
-> Agent externalized output
-> predicate / oracle
-> wtool / Sequencer
-> L4 accepted or L4.E rejection evidence
```

Hard invariant:

```
externalized_attempt_count
=
  L4_WorkTx_attempt_count
+ L4E_WorkTx_rejection_count
+ explicitly_anchored_capsule_attempt_count
```

Every externalized LLM-Lean cycle that affects proof state, future prompt context, Lean checking, final composite proof, economic state, scheduler state, price signal, or market logic must be tape-visible.

Routing rule:

```
predicate pass    -> L4 accepted
predicate fail    -> L4.E rejection evidence
high-volume evidence -> CAS EvidenceCapsule + L4 anchor
```

Private chain-of-thought is not recorded.
Externalized proposals, tool calls, Lean checks, parse failures, proof artifacts, and any output used by future system state are not private CoT once they affect the system.

### 3.2 FC2 — Boot / Genesis Gate

Every real evidence run must be replayable from:

- `genesis_report`
- ChainTape
- CAS
- agent registry
- system pubkeys

Forbidden:

- memory-only preseed
- post-hoc genesis reconstruction
- retroactive evidence rewrite
- global latest pointer as source of truth
- untracked system key / agent key

All production initialization must be represented by accepted chain events such as:

- `on_init`
- `TaskOpenTx`
- `EscrowLockTx`
- `AgentRegistry` entry
- system key pinning

### 3.3 FC3 — Meta / Markov Gate

`EvidenceCapsule` and `MarkovEvidenceCapsule` are derived views, not hidden ground truth.

Rules:

- raw logs shielded
- capsules derived from ChainTape + CAS
- latest capsule can guide the next run
- deep history requires explicit Markov override
- no global latest pointer as canonical input
- no automatic predicate/tool mutation by ArchitectAI
- JudgeAI / VetoAI remains veto-only

***

## 4. The Three Strategic Decisions Are No Longer Open

Do not stall on these again.

### 4.1 G-009 / HEAD_t

Decision: **Path C hybrid**.

Immediate C1 witness:

```
HEAD_t = {
  state_root,
  l4_head,
  l4e_head,
  cas_root,
  economic_state_root,
  run_id
}
```

Requirements:

- every accepted transition updates `HEAD_t`
- replay reconstructs `HEAD_t`
- dashboard reads derived state only
- no hidden current-state pointer

Later C2 production path:

```
libgit2-backed refs:
  refs/chaintape/l4
  refs/chaintape/l4e
  refs/chaintape/cas
```

Do not choose subprocess git as primary unless architect explicitly reverses this decision.

### 4.2 G-012 / PCP soundness

Decision: **Lean tactic-mutation adversarial corpus first; MiniF2F-v2 misalignment second**.

Minimum corpus:

- valid proof
- mutated invalid proof
- sorry insertion
- type mismatch
- wrong theorem name
- off-by-one arithmetic
- irrelevant theorem
- partial tactic accepted but final invalid
- parse-invalid output

Gate:

- valid proofs pass
- mutated invalid proofs fail
- invalid proofs never enter L4 accepted
- invalid proofs enter L4.E or anchored EvidenceCapsule

Synthetic adversarial tests are allowed as negative controls, but they cannot replace real public problem witnesses.

### 4.3 G-016 / G-019 / G-021 / G-028 / prompt persistence

Decision: **Class-3 PromptCapsule + L4 anchor by default**.

Default `PromptCapsule`:

```
PromptCapsule {
  prompt_context_hash,
  read_set,
  policy_version,
  hidden_fields_redacted,
  visible_context_cid,
  system_prompt_template_hash,
  agent_view_manifest_cid
}
```

Rules:

- `PromptCapsule` -> CAS
- `AttemptTelemetry` / `WorkTx` references `prompt_capsule_cid`
- L4/L4.E anchor references the attempt

Do not put full verbatim prompt into canonical tape by default.
Verbatim prompt may be stored only as encrypted/audit-only Class-4 artifact with explicit ratification.

***

## 5. Evidence Taxonomy

### 5.1 L4 accepted transition

A predicate-passing state transition that advances canonical state.

Examples:

- `TaskOpenTx`
- `EscrowLockTx`
- `WorkTx` accepted by predicates
- `VerifyTx` accepted
- `FinalizeRewardTx` system-only payout
- `RunExhaustedTx` if system-emitted terminal state
- `TaskExpireTx`

### 5.2 L4.E rejection evidence

A submitted transaction or externalized attempt that fails predicate/policy and must not advance accepted state.

Examples:

- `LeanFailed`
- `ParseFailed`
- `SorryBlocked`
- `StaleParentRoot`
- `InsufficientBalance`
- `SystemTxForbiddenOnAgentIngress`
- `InvalidPromptCapsule`

### 5.3 CAS high-dimensional evidence

Examples:

- `AttemptTelemetry`
- `LeanResult`
- Proposal payload
- `PromptCapsule`
- proof artifact
- raw Lean stderr/stdout
- `EvidenceCapsule`
- `AgentAutopsyCapsule`
- `MarkovEvidenceCapsule`

CAS objects must be reachable through ChainTape references or capsule manifests.

### 5.4 Dashboard / report

A read-only materialized view.
It must be deletable and regeneratable from ChainTape + CAS.

Never treat dashboard as source of truth.

***

## 6. Externalized Attempt Rule

An externalized attempt is any model output that is:

- parsed
- sent to Lean
- used as proof prefix
- used to build final composite proof
- used in future prompt context
- submitted to a tool
- used to change scheduling
- used in economic or market logic

For every externalized attempt:

- `AttemptTelemetry` must exist in CAS.
- `LeanResult` must exist if Lean was called.
- `PromptCapsule` must exist if prompt context influenced the attempt.
- The attempt must be represented in L4 or L4.E, or explicitly counted in an anchored EvidenceCapsule.

Required invariant:

```
evaluator_reported_completed_llm_calls
=
  l4_work_attempt_count
+ l4e_work_attempt_count
+ capsule_anchored_attempt_count
```

Canonical LHS scope (clarified 2026-05-07 per `OBS_TB18R_INV1_NONLLM_TX`):

```
evaluator_reported_completed_llm_calls
=
  tool_dist.step + tool_dist.parse_fail + tool_dist.llm_err
```

The LHS must NOT use `evaluator_reported_tx_count` — that field includes architect-mandated non-LLM admin scaffold (TB-6 atom-3 synthetic preseed, TB-C0 atom A.1 synthetic L4.E gate, sequencer system-terminal-summary) which inflates the count and produces false NegativeDelta on mixed-tx problems. Each of `step` / `parse_fail` / `llm_err` corresponds to one `r2_write_attempt_telemetry` call site — i.e., one externalized LLM-Lean cycle.

If this equality fails:

- HALT
- do not continue benchmark
- do not audit as pass
- do not ship

***

## 7. Constitution Landing Policy

Every constitution clause must have a row in:

`handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`

Each row must map:

```
clause / flowchart node
-> code surface
-> executable test
-> smoke/evidence witness
-> current status
-> kill condition
```

Allowed statuses:

- `LANDED`
- `PARTIAL`
- `NOT-LANDED`
- `BLOCKED-DECISION`
- `DEFERRED-FORWARD`
- `N/A`

Documentation-only coverage is not landed.

If a critical clause is `NOT-LANDED` or `BLOCKED-DECISION`, feature work touching that area is frozen.

***

## 8. Constitutional CI

The gate runner is authoritative:

```
bash scripts/run_constitution_gates.sh
make constitution
```

Expected maintained surfaces:

- `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
- `handover/alignment/TRACE_FLOWCHART_MATRIX.md`
- `tests/constitution_*.rs`
- `tests/fc_alignment_conformance.rs`

A constitution gate must be able to fail.
A test that cannot fail is documentation, not a gate.

***

## 9. Development Risk Classes

### Class 0 — Docs / charter / plan

Allowed:

- draft docs
- update charter
- update matrix
- update handover

Audit:

- self-check
- architect/user review if policy-affecting

### Class 1 — Additive isolated module

Allowed:

- pure helper
- parser
- formatter
- non-authoritative view

Audit:

- self-audit + `cargo test --workspace`

### Class 2 — Production wire-up

Examples:

- evaluator adapter
- dashboard regeneration
- ChainTape replay verifier
- benchmark harness

Audit:

- constitution harness
- minimal real run
- self-audit
- external implementation audit when non-trivial

### Class 3 — Auth / money / CAS integrity / capability / market / production evidence

Examples:

- `EconomicState` mutation
- system-emitted tx
- CAS evidence packaging
- market position / CompleteSet
- `audit_tape`
- controlled market smoke

Required order:

```
harness -> real evidence -> external audit
```

Do not audit before evidence exists.

### Class 4 — Constitution / sequencer admission / typed tx schema / canonical signing payload / RootBox

Requires:

- explicit architect ratification
- harness
- minimal real run if applicable
- external audit

Single-word messages such as:

- `fix`
- `go`
- `ok`
- `continue`
- `可以`

do not constitute Class-4 sign-off.

***

## 10. Authorization Semantics

For Class 3/4 or ship decisions, authorization must name:

- scope
- allowed path
- forbidden path
- risk class
- whether audit is required
- whether ship is authorized

A one-word instruction may authorize candidate remediation only.
It does not authorize final ratification or ship.

For VETO remediation, create or cite:

`handover/directives/YYYY-MM-DD_<topic>_REMEDIATION_DIRECTIVE.md`

It must state:

- authorized changes
- forbidden changes
- rollback requirement
- allowed files/surfaces
- ship gates

***

## 11. Pre-Action Gates

Before any runner script that mutates `handover/evidence/` or runs true evaluation:

invoke `/runner-preflight`

It must check:

- clean tree
- binary mtime/current HEAD
- evidence immutability
- risk class
- FC trace
- charter existence
- audit-round state

If preflight fails, do not run.

After TB shipped final or audit rounds > 3:

invoke `/harness-reflect`

Before adding a new `feedback_*.md`, ask:

> What mechanism will catch this next time?

If no mechanism exists, build mechanism first.

***

## 12. Code Standard

Required:

- `cargo check`
- `cargo test --workspace`
- `bash scripts/run_constitution_gates.sh`

Forbidden:

- `.env` commit
- hardcoded behavior parameter
- `f64` in money path
- memory-only canonical state
- shadow ledger source of truth
- dashboard-only source of truth

STEP_B protocol applies to:

- `src/kernel.rs`
- `src/bus.rs`
- `src/sdk/tools/wallet.rs`
- `src/state/sequencer.rs`
- `src/state/typed_tx.rs`
- `src/bottom_white/cas/schema.rs`
- canonical signing payload surfaces

Any change touching sequencer admission, typed tx schema, or canonical signing payload is at least Class-4 candidate until classified otherwise.

***

## 13. Economy Laws

The economic constitution:

- Information is Free
- Only Investment Costs Money
- 1 Coin = 1 YES + 1 NO
- `on_init` is the only legal base-Coin mint

Hard gates:

- reads/search/thinking do not spend core Coin
- writes/append/challenge/verify/settle require stake/escrow/bond as specified
- total Coin conserved after `on_init`
- YES/NO shares are claims, not Coin
- `NodePosition` is exposure index, not Coin
- `WalletTool` is read-only projection
- system tx cannot be agent-submitted
- no ghost liquidity
- no automatic YES/NO injection
- no `f64` money path

Market price is a statistical signal, not truth.

***

## 14. Predicate / Oracle Rules

Boolean predicates define hard boundary:

```
predicate pass -> may enter L4
predicate fail -> L4.E or anchored evidence
```

Lean verification must be represented as durable evidence:

- `LeanResult` CAS object
- proof artifact CID
- predicate result
- L4 / L4.E route

Do not let evaluator stdout become oracle truth.

Partial verdicts must be typed.
Do not allow ambiguous states such as:

```
exit_code = 0
verified = false
error_class = None
```

unless an explicit typed `PartialAccepted` / `PartialVerdict` state exists and is covered by tests.

***

## 15. Shielding Rules

Do not broadcast raw failure logs.

Allowed:

- `public_summary`
- low-pollution rejection class
- typical error summary
- private/audit-only diagnostic CID

Forbidden:

- raw Lean stderr in ordinary Agent read view
- raw autopsy broadcast
- hidden benchmark leak
- private predicate leak
- global context stuffing with historical logs

Agent read views must be scoped, reconstructable, and shielded.

***

## 16. Tape / ID Canonicality

Canonical IDs and shadow IDs must not be mixed.

Rules:

- canonical `WorkTx.tx_id` belongs to ChainTape / L4
- shadow tape id belongs only to legacy local kernel tape
- `PriceIndex` / `NodePosition` / `NodeMarket` must use canonical ids
- legacy `bus.append parent_id` must not receive canonical `TxId`

If a feature needs graph structure, build it from:

- L4 accepted `WorkTx`
- L4.E rejected attempts
- `ProposalTelemetry.parent_tx`
- `AttemptTelemetry.parent_attempt_tx`
- CAS artifacts

Do not read legacy shadow tape as source of truth.

***

## 17. Reporting Standard

Every run report must include:

- commit HEAD
- binary build identity
- command used
- risk class
- `genesis_report` path
- ChainTape path
- CAS path
- agent registry path
- system pubkeys
- `attempt_count_equality_report`
- replay report
- dashboard regeneration statement

For formal proof benchmark reports, include:

- ΣPPUT
- Mean PPUT on solved
- 95% CI if reporting aggregate
- `halt_reason_distribution`
- proposal / attempt counts
- accepted / rejected counts
- no fake accepted nodes status

Do not start a report with solve count alone.
Solve count without tape and PPUT is misleading.

***

## 18. Benchmark Rules

A benchmark is not valid unless:

- all externalized attempts are represented
- ChainTape/CAS evidence is restorable
- attempt equality holds
- failures are visible or anchored
- dashboard regenerates from evidence
- `BenchmarkManifest` pins model/problem/seed/Lean/mathlib/commit
- `EvidencePackagingPolicy` is satisfied

Before scale-up:

- P38
- P49
- M0 mini-batch

must pass constitution gates.

Large benchmark is forbidden while:

- attempt count mismatch exists
- FC gates red
- Art. III shielding gaps block prompt persistence
- `HEAD_t` witness absent
- PCP soundness corpus absent

MiniF2F large-scale testing is a formal benchmark stress test, not real-world readiness.

***

## 19. No Manipulation by Sequencing

Do not close easy gaps to create progress optics while load-bearing blockers remain red.

If any of the following is `BLOCKED-DECISION`, feature work touching that surface is frozen:

- `HEAD_t`
- PCP soundness
- `PromptCapsule` / prompt persistence
- system tx authorization
- tape canonical ID namespace
- economic conservation

Cosmetic waves cannot be used to claim constitution landing while load-bearing blockers remain unresolved.

***

## 20. Feature Freeze Conditions

Freeze new feature work if any of the following are red:

- FC1 Runtime Loop
- FC2 Boot
- FC3 Meta/Markov
- Tape canonicality
- Economy conservation
- No-fake-accepted
- System-tx-not-agent-submittable
- Dashboard-regeneratable
- Attempt equality

During freeze, do not implement:

- NodeMarket
- PriceIndex
- Polymarket signal
- AMM
- CompleteSet
- public chain
- real-world readiness
- benchmark publicity
- new market mechanics

unless the active TB explicitly exists to close a constitution gate related to that feature.

***

## 21. Handover Discipline

Dynamic state belongs in:

- `handover/ai-direct/LATEST.md`
- `handover/tracer_bullets/TB_LOG.tsv`

CLAUDE.md must not encode:

- current ship status
- current gate counts
- current HEAD
- current round count
- temporary freeze details

Use this file for stable operating law only.

Memory files belong in:

`~/.claude/projects/-home-zephryj-projects-turingosv4/memory/`

Do not duplicate TB_LOG facts into memory.
Memory is for recurring rules, surprises, and mechanisms.

***

## 22. Read Order for New Session

Default read order:

1. `CLAUDE.md`
2. `constitution.md`
3. `handover/ai-direct/LATEST.md`
4. Key Coding Principles: [KARPATHY_ARCHITECT.md](file:///home/zephryj/projects/turingosv4/skills/KARPATHY_ARCHITECT.md) & [KARPATHY_SIMPLE_CODE.md](file:///home/zephryj/projects/turingosv4/skills/KARPATHY_SIMPLE_CODE.md)
5. `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
6. `handover/alignment/TRACE_FLOWCHART_MATRIX.md`
7. current TB charter / directive
8. only then supporting docs

If a directive conflicts with constitution or flowcharts:

> constitution wins

If LATEST conflicts with ChainTape evidence:

> ChainTape evidence wins

***

## 23. User Context

The user is a solo researcher and vibe coder with limited programming background.

Default language:

- Chinese

Technical terms may remain English.

Prioritize:

- clear decisions
- explicit gates
- exact instructions for AI coder
- no fake certainty
- no ceremonial process
- tape-first implementation
- fast real-run feedback

Never hide behind process if the tape is wrong.
