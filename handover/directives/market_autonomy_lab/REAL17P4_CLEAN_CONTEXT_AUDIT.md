# REAL-17 P4 Clean-Context Audit

Date: 2026-05-17

Question:

```text
Is the REAL-17 P4 poll-budget and stale-CAS-lock hygiene diff acceptable to
commit as a research-branch hardening fix before rerunning hard10 evidence?
```

Risk class:

```text
Class 3 research runner/evidence + TrustRoot-pinned evaluator
within MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2.
```

Touched invariants:

```text
FC1 wtool / ChainTape / CAS evidence readability
FC2 replay and audit verifier reconstructability
FC3 logs archive diagnostics
```

Forbidden / excluded surfaces:

```text
No sequencer admission change.
No TypedTx schema or discriminant change.
No canonical signing payload change.
No CAS ObjectType schema change.
No kernel, bus, wallet, constitution, or flowchart change.
```

Auditor verdict:

```text
PROCEED
```

Findings:

```text
No blocking production defects.
```

Non-blocking notes:

```text
1. genesis_payload.toml keeps the long historical evaluator provenance comment
   after the newly pinned hash; the preceding line records the P4 rehash reason.
2. The runner cleanup gate is source-level rather than a live temp-directory
   behavioral test. It is acceptable for this narrow hardening fix, but future
   runner hygiene work may add an executable shell-level fixture.
```

Reviewed production points:

```text
1. Both REAL-6A success-path WorkTx accept polls now use
   real6a_poll_budget_ms().
2. scripts/run_g_phase_batch.sh clears only the exact
   cas/.turingos_cas_chain.lock semaphore after batch_evaluator exits and before
   post-run audit/persistence.
3. The cleanup checks recorded pid=... with kill -0 and preserves live locks.
4. The final runner exit still preserves non-zero batch/audit/persistence
   failure status.
```

Claim boundary:

```text
This audit covers only the hardening diff.
It does not upgrade any market-emergence or E-level label.
```
