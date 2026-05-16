# REAL-BCAST-1 Librarian Broadcast Loop — Execution Plan

## Summary

This plan implements the architect's broadcast repair as a Class 3 package.
It does not introduce new TypedTx, new sequencer admission, signing payload
changes, CAS ObjectType changes, or out-of-band messages.

Core path:

```text
CAS / ChainTape / EvidenceCapsule / AttemptTelemetry / LeanResult / L4.E
-> LibrarianSelector
-> LibrarianDigest
-> RoleNotificationProjector
-> PromptCapsule read_set / visible_context_cid
-> Agent next turn
```

## G0 — REAL-13 Status Sync

Before REAL-BCAST evidence or REAL-13A live micro-probe, write
`handover/directives/2026-05-16_REAL13_STATUS_SYNC_FOR_ARCHITECT.md`.
If the canonical evidence path or audit verdict cannot be verified, stop
REAL-BCAST A/B and REAL-13A live runs.

## Implementation Atoms

1. Charter and decision records:
   `REAL_BCAST_1_LIBRARIAN_BROADCAST_LOOP_CHARTER.md`,
   `DECISION_LIBRARIAN_DIGEST_MATERIALIZED_VIEW.md`,
   `DECISION_LIBRARIAN_BARRIERED_ASYNC_BROADCAST.md`.
2. Add `src/runtime/librarian_broadcast.rs` with
   `LibrarianSourceScope`, `LibrarianSelector`, `TypicalErrorCluster`,
   `PartialProgressSummary`, `LibrarianDigest`, `BroadcastEpoch`, role crop
   rendering, PromptCapsule binding helpers, and audit/no-leak helpers.
3. Use `ObjectType::Generic` with schema ids, especially
   `turingosv4.librarian_digest.v1`. Do not edit CAS ObjectType schema.
4. Extend `src/runtime/market_review.rs` so MarketReviewWindow/Response/Summary
   can carry `digest_cid` and `broadcast_epoch_id`; validate mismatch and
   future/stale epochs fail closed.
5. Extend `src/sdk/prompt.rs` with a bounded `=== Librarian Notices ===`
   prompt section.
6. Wire evaluator prompt construction so enabled Librarian notices are written
   to CAS, injected into visible prompt bytes, included in PromptCapsuleV2
   read_set, and linked through AttemptTelemetry.prompt_capsule_cid.
7. Add dashboard/audit materialized view helpers; dashboard is never source of
   truth.
8. Run REAL-BCAST ON/OFF A/B after digest injection is green. Only the
   Librarian flag may differ.

## Half-Async Contract

Sequential mode freezes a digest after a ChainTape/CAS append and before the
next role-scoped prompt.

Barriered async freezes `digest_cid` and `broadcast_epoch_id` at
MarketReviewWindow open. Every eligible prompt in that window uses the same
digest. Every response records the digest and epoch. Commit order remains
deterministic by `(agent_id, response_id)`.

Full async is not implemented in REAL-BCAST-1. Future full async must validate
`BroadcastEpoch { epoch_id, source_head_t, digest_cid, valid_from, valid_until,
task_tags }` to prevent future-digest and global-pointer drift.

## Tests

Targeted tests:

```bash
cargo test --test constitution_librarian_source_scope
cargo test --test constitution_librarian_selector
cargo test --test constitution_librarian_digest
cargo test --test constitution_librarian_role_projector
cargo test --test constitution_librarian_prompt_injection
cargo test --test constitution_librarian_half_async
cargo test --test constitution_librarian_no_raw_leakage
cargo test --test constitution_librarian_real_evidence_binding
```

Ship checks:

```bash
bash scripts/run_constitution_gates.sh
cargo test --workspace --no-fail-fast -- --test-threads=1
```

## Forbidden

```text
No raw Lean stderr/prompt/completion/CoT/diagnostics broadcast.
No untriaged historical log stuffing.
No global LATEST pointer.
No LibrarianDigest as source of truth.
No new TypedTx.
No CAS ObjectType schema edit in MVP.
No price-as-truth.
No forced trade.
No dashboard-as-truth.
No silent skip of unknown CAS payload.
No full async ship path in REAL-BCAST-1.
```

