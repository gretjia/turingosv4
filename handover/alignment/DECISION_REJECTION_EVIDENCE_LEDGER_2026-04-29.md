# Decision Record — Rejection Evidence Ledger separate from Accepted Transition Ledger

**Date**: 2026-04-29
**Status**: ACCEPTED
**Driver**: external audit 2026-04-29 CF-1 (`handover/audits/2026-04-29_external_audit.md`)
**Authority**: user `gretjia` chat authorization on 2026-04-29 ("授权 6 个 items 全部执行")
**Decision class**: architectural boundary, P1 GitTape Kernel ledger semantics
**Reversibility**: in-principle reversible only via constitution amendment (Layer-1 architectural commitment)

---

## Decision

**Rejected submissions never enter the L4 accepted-transition ledger.**

The two ledgers are distinct, with distinct identifiers, distinct hash chains, and distinct contribution to `Q_t`:

```text
L4   Accepted Transition Ledger  (= existing src/bottom_white/ledger/transition_ledger.rs)
     - identifier:        logical_t (monotone-increasing)
     - hash chain:        ledger_root chain over accepted entries
     - Q_t contribution:  advances state_root_t and ledger_root_t
     - replay use:        replay_full_transition reconstructs Q_t from L4 only
     - source of truth:   accepted state-machine transitions

L4.E Rejection Evidence Ledger   (= new src/bottom_white/ledger/rejection_evidence.rs)
     - identifier:        submit_id (assigned at submit time, monotone but
                          orthogonal to logical_t — not all submit_ids become
                          logical_ts)
     - hash chain:        rejection_root chain over rejected records
     - Q_t contribution:  NONE — does NOT mutate state_root_t or ledger_root_t
     - read view:         raw_diagnostic_cid is private/shielded; only
                          aggregate counter and public_summary may surface
                          in another Agent's materialized view
     - consumers:         P4 ErrorClusterer, P3 ChallengeCourt, P3 ReputationIndex,
                          P5 ArchitectAI proposal flow (post-RSP-3)
```

## Reasoning

Three reasons, in order of severity:

### 1. Replay semantics

If rejected entries shared `logical_t` with accepted entries, the invariant "every `logical_t` is an accepted state transition; replay produces bit-equal `state_root`" would break. Then `replay_full_transition` would either need to skip `logical_t`s (defeats the monotone-time abstraction) or apply rejected entries as no-ops (defeats the auditability of why a `logical_t` exists at all).

Splitting the ledgers means:
- `replay_full_transition` consumes L4 only and produces a deterministic `state_root`.
- L4.E is auditable independently — useful for forensic / clusterer / reputation queries — but does not interfere with state reconstruction.

### 2. Goodhart shield

Accepted transitions are public constitutional state by definition. Rejected diagnostics are *failure detail*: stack traces, predicate-error strings, raw LLM output. If both lived in the same ledger, every Agent's materialized view would have to choose between (a) seeing all of L4 plus rejected diagnostics — context contamination + Goodhart attack surface, or (b) filtering at read time — which puts privacy logic in `materializer.py` instead of in the ledger schema, where it belongs.

Splitting means:
- L4 entries are accepted state — Agents' read views see them per the standard `read_set_authorized` predicate.
- L4.E entries default to private (raw_diagnostic_cid hidden); only the aggregate counter / public_summary surface unless an Agent has a privileged role (P3 ChallengeCourt, P5 ArchitectAI sandboxed read).

### 3. Context contamination prevention

The constitutional "selective broadcasting / selective shielding" principle (Art. III.4) requires that one Agent's failure does not pollute another Agent's prompt. A single-ledger model would force every read view to scrub rejected entries — an opt-out model that is fragile against future code paths.

Splitting means: by default, L4.E is shielded; access requires explicit role grant; policy violation is detectable as a schema-level read-set-authorization violation, not a content-level filtering bug.

## Implementation contract

### Sequencer behavior

```rust
// src/state/sequencer.rs (CF-4 minimum WorkTx dispatch lands at TB-1 Day-3+)

while let Some(tx) = queue_rx.recv().await {
    let submit_id = /* assigned at submit time */;
    match self.apply_one(tx.clone()) {
        Ok(entry) => {
            // L4 path: accepted transition
            self.transition_ledger.append(entry)?;
            // logical_t increments here; state_root + ledger_root advance
        }
        Err(ApplyError::Transition(inner)) => {
            // L4.E path: rejection evidence
            self.rejection_writer.append_rejected(RejectedSubmissionRecord {
                submit_id,
                parent_state_root: q_snapshot.state_root_t,
                agent_id: tx.agent_id(),
                tx_kind: tx.kind(),
                tx_payload_cid: tx.payload_cid(),
                rejection_class: classify(&inner),
                raw_diagnostic_cid: Some(cas_put(inner.raw_diagnostic())),
                public_summary: Some(inner.public_summary()),
                prev_hash: self.rejection_writer.head_hash(),
                hash: /* computed */,
                timestamp_logical_submit: now(),
            })?;
            // CRITICAL: do NOT mutate q.state_root_t or q.ledger_root_t.
            // Specifically: do NOT call self.transition_ledger.append(...) here.
        }
        Err(ApplyError::Infrastructure(inner)) => {
            // separate class: not L4, not L4.E by default; may halt or
            // route to operator log per OBS_BOOT_FAIL_NOT_HALT-style policy.
        }
    }
}
```

### Read-view shielding

`materializer.py` (or its Rust equivalent) MUST NOT include `raw_diagnostic_cid` content in any Agent's read view by default. Permitted L4.E projections in a default read view:

- aggregate counters (`rejection_count_by_class`, `rejection_count_by_agent`)
- `public_summary` (the predicate's intentionally-broadcast short string)
- the `submit_id` itself (not the payload)

Privileged roles (challenge court, architect sandbox) may opt in to richer projections via explicit role-grant, audited.

### Tests required (TB-1 Day-3 Tier-A)

- `test_p1_kill_2_rejected_tx_no_state_advance`: L4 `logical_t` NOT incremented; L4.E record IS appended.
- `test_p1_kill_4_rejected_log_isolated`: another Agent's view has aggregate counter only; raw_diagnostic_cid content NOT visible.
- `test_p1_kill_4b_rejection_chain_breaks_on_row_deletion`: L4.E hash chain integrity.

## Out-of-scope (this decision does not say)

- Whether L4.E is content-addressed (CAS) under-the-hood. Recommendation: yes (each `raw_diagnostic` and each `RejectedSubmissionRecord` payload becomes a CID), but the decision binds only the ledger-separation semantics, not the storage backend.
- Whether L4.E entries can be garbage-collected after a retention window. Recommendation: yes, per Art. III.4 selective broadcasting; specific retention policy is a P4 Information Loom design decision.
- Whether L4.E is replicated across multi-org peers in P6. Recommendation: yes for `public_summary` + aggregate counters; raw diagnostics are local-only by default. Specific multi-org policy is P6 work.

## Cross-references

- Audit driving this decision: `handover/audits/2026-04-29_external_audit.md` § CF-1
- ROADMAP P1 amendment: `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` § P1 Build / Exit 6 / Exit 9
- TB-1 Day-3 charter: `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` Day-3 (Tier-A tests 1-6)
- Memory hint: `~/.claude/.../memory/feedback_rejection_evidence_separate.md`
- Existing transition ledger code (= L4): `src/bottom_white/ledger/transition_ledger.rs`
- New rejection evidence module home (= L4.E): `src/bottom_white/ledger/rejection_evidence.rs` (to be created at TB-1 Day-3)
