# DECISION — LibrarianDigest Is Materialized View

`LibrarianDigest` is not a source of truth.

It is a deterministic, CAS-backed view derived from ChainTape/CAS evidence:
AttemptTelemetry, LeanResult, L4.E rejection summaries, EvidenceCapsule,
MarketDecisionTrace, NoTradeReasonTrace, EconomicJudgment, and EVDecisionTrace.

If a digest contradicts ChainTape/CAS, ChainTape/CAS wins.

MVP storage uses:

```text
ObjectType::Generic
schema_id = "turingosv4.librarian_digest.v1"
```

No new CAS ObjectType is introduced in REAL-BCAST-1.

