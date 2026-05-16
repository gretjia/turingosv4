# OBS_REAL13_MARKET_SNAPSHOT_CID_ALIAS

Date: 2026-05-16
Status: forward observation, non-blocking

## Observation

The clean-context REAL-13 implementation review found that the first REAL-13 implementation currently sets `EVDecisionTrace.market_snapshot_cid` to the existing prompt capsule CID in the evaluator path. This remains reconstructable through CAS plus `parent_state_root`, and it did not block the REAL-13 audit.

## Why It Matters

The field name `market_snapshot_cid` may imply an independent market snapshot object. Future REAL-13C/F/G/H work should either:

```text
1. write a dedicated market snapshot CAS object and store its CID here; or
2. rename/clarify the field in a future schema so it cannot be confused with an independent snapshot.
```

## Current Boundary

This observation does not authorize a schema change in the current package. It records a forward hardening item only.

Current REAL-13 claim remains:

```text
EVDecisionTrace and MarketReviewSummary are CAS-backed and dashboard-visible as materialized views.
E2 is not achieved.
```
