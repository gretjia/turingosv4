# DECISION — Librarian Broadcast In Half-Async Market Review

REAL-BCAST-1 plugs into the current half-async mechanism by freezing a
Librarian digest at deterministic barriers.

## Sequential Mode

```text
append evidence to CAS/ChainTape
derive LibrarianDigest
inject role crop into next PromptCapsule visible context
```

## Barriered Async Mode

```text
MarketReviewWindow opens
digest_cid + broadcast_epoch_id freeze
eligible agents receive prompts built from the same digest
responses carry digest_cid + epoch_id
barrier closes
responses commit in deterministic (agent_id, response_id) order
```

Digest mismatch, stale digest, or future digest rejects the response evidence.

## Future Full Async

Full async is not a REAL-BCAST-1 ship path. Future full async must use
`BroadcastEpoch { epoch_id, source_head_t, digest_cid, valid_from, valid_until,
task_tags }` and replay gates that prove no future digest and no global pointer.

