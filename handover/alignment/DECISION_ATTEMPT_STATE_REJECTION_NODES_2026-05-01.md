# Decision Record — Three-Node Taxonomy (Attempt / State / Rejection-Evidence)

**Date**: 2026-05-01
**Status**: ACCEPTED (docs-level only; NOT a constitution amendment)
**Driver**: architect verdict 2026-05-01 (`handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md` §A2).
**Authority**: user `gretjia` formal verdict 2026-05-01.
**Decision class**: terminology / dashboard semantics; binds TB-7R + Developer Manual + Audit Reports.
**Reversibility**: text-level reversible; revising would not affect ledger schema.

---

## Decision

The three-node taxonomy below is normative for TB-7R / dashboards /
decision records / developer documentation. It is **NOT** a
constitution amendment and MUST NOT be added to `constitution.md`,
RootBox, or any sudo-gated artifact.

```text
Attempt Node
  = every real externalized LLM proposal.
  = represented in ChainTape as either L4 accepted WorkTx
    OR L4.E rejected evidence WorkTx.
  = NEVER both. NEVER neither.

State Node
  = a predicate-passed L4 accepted transition.
  = advances state_root_t and ledger_root_t.
  = forms the L4 accepted spine on which Q_t -> Q_{t+1} is computed.

Rejection Evidence Node
  = a predicate-failed candidate.
  = lives in L4.E with monotone submit_id orthogonal to logical_t.
  = does NOT advance state_root_t.
  = raw_diagnostic_cid is shielded by default;
    public_summary + aggregate counters are the only public surface.
```

## Reasoning

### 1. Term hygiene

Prior usage mixed "node append" / "WorkTx" / "transition" / "rejection
record" inconsistently. Three-node taxonomy collapses the conceptual
surface to one orthogonal axis (Attempt / State / Rejection-Evidence)
that maps cleanly to the existing L4 / L4.E ledger schema. No code
rename required (`feedback` is `B5` of TB-7R verdict: docs only).

### 2. Why NOT in constitution.md

Constitution flowchart modifications require Phase Z′ 6-stage rerun
(`Audit Standard` in CLAUDE.md). The three-node taxonomy is a
*descriptive* aid for what the existing ledger already does — not a
new constitutional rule. Putting it in constitution.md would:

- Force a Phase Z′ rerun for a docs-level clarification.
- Conflate "what the system does" with "the spec it answers to."
- Risk drift if a future schema reorganization renames terms.

### 3. Boundary against thesis claim

Externalized proposal ≠ private CoT. The three-node taxonomy applies
ONLY to externalized proposals. A model's internal chain-of-thought
that never produced a tool call / external output is NOT an Attempt
Node and is NOT counted in `proposal_count_chaintape_attempts`. This
boundary is enforced at the ChainTape ingestion site — the binding
contract is "if it became a `submit_typed_tx`, it is an Attempt Node;
if it stayed in the model's hidden state, it is invisible to TuringOS."

## Implementation contract

### Allowed surfaces

- TB-7R charter (`handover/tracer_bullets/TB-7R_charter_2026-05-01.md`)
- Audit dashboard (`src/bin/audit_dashboard.rs`) report sections
- Audit reports (`handover/audits/*.md`)
- TB ship reports (`handover/RECURSIVE_AUDIT_*.md`)
- Decision records (this file + future)
- Developer Manual (when authored)
- Memory entries (`feedback_*` / `project_*`)

### Forbidden surfaces

- `constitution.md` (any subsection)
- `cases/C-*.yaml` headline rule text (case body MAY reference, but rule itself stays in existing constitutional terms)
- `RootBox` / sudo-gated artifacts
- `TRACE_MATRIX_*` row text (a row MAY backlink, but the row's flowchart-id text stays canonical)
- src/ identifier renames (existing `WorkTx`, `RejectedSubmissionRecord`, `accepted_worktx_*` stay)

### Definitional anchors (use these in dashboards)

```text
Attempt count
  = L4 WorkTx count + L4.E WorkTx-rejection count

Acceptance count (= State Node count)
  = L4 WorkTx count where verification_result_cid resolves to
    VerificationResult.verified == true

Rejection count (= Rejection Evidence Node count)
  = L4.E records where tx_kind == Work AND
    rejection_class ∈ { PredicateFailed, LeanFailed, ... }

Externalized proposal count (truth source for "did all candidates land?")
  = the count emitted by the runtime at ChainTape submit site
    (NOT counting LLM internal CoT, NOT counting unbroadcast drafts)
```

### What NOT to count as an Attempt Node

```text
- Private CoT / hidden reasoning
- Model output that never produced a submit_typed_tx call
- Speculative tactic strings the agent explored but did not externalize
- Tool-call drafts that were retracted before bus.submit_typed_tx
- Compound proposals: 1 LLM call producing N tactic statements
  in one externalized payload IS 1 Attempt Node
  (per-tactic-N decomposition is TB-8+ scope, not TB-7R)
```

## Cross-references

- Driving verdict: `handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md` §A1+A2
- L4 / L4.E ledger separation: `handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`
- TB-7R charter: `handover/tracer_bullets/TB-7R_charter_2026-05-01.md`
- Memory: `feedback_chaintape_externalized_proposal.md` (B′ rule)
