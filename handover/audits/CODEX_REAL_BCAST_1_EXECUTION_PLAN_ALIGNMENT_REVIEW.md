# CODEX REAL-BCAST-1 Execution Plan Alignment Review

Verdict: `CHALLENGE`

## Findings

1. REAL-13 status sync was not a hard gate. The architect did not know the
   REAL-13 run state, so status/evidence must be synchronized before any
   REAL-BCAST or REAL-13A live decision.
2. Half-async / barriered async integration was directionally correct but not
   executable enough. MarketReviewWindow, MarketReviewResponse, and
   MarketReviewSummary need digest/epoch linkage and deterministic reject
   rules for stale/future/mismatched digests.
3. PromptCapsule linkage needed a hard gate: digest CID in
   `PromptCapsuleV2.read_set`, notice bytes in `visible_context_cid`, prompt
   hash changes when digest changes, and AttemptTelemetry links to the capsule.
4. PartialProgressSummary wording was too loose. Digest must never copy raw
   candidate body; only sanitized summary plus CID provenance is allowed.
5. CAS schema strategy was under-specified. MVP must use `ObjectType::Generic`
   and schema id, not a new CAS ObjectType.
6. Unknown schema fail-closed must use typed/shared decoders and avoid string
   scanning arbitrary CAS bytes.

## Required Corrections Incorporated

- Added G0 REAL-13 status sync as stop-the-line condition.
- Added explicit MarketReview digest/epoch fields and reject rules.
- Added `BroadcastEpoch` schema for async-ready future migration.
- Locked MVP CAS strategy to `ObjectType::Generic`.
- Added PromptCapsule hard gate.
- Added no-leak kill gates.
- Added selector negative fixtures.
- Added A/B gate with Librarian ON/OFF as the only variable.

## Verdict

The corrected plan may proceed after these gates are implemented as tests and
evidence.

