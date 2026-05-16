# REAL-BCAST-1 Librarian Broadcast Loop Charter

REAL-BCAST-1 repairs the Top White broadcast loop.

It turns already externalized CAS/ChainTape evidence into sanitized,
role-scoped, CAS-backed Librarian notices that enter the next agent turn
through PromptCapsule read-set and visible context.

## Boundaries

```text
No new TypedTx.
No sequencer admission change.
No canonical signing payload change.
No CAS ObjectType schema change.
No out-of-band message system.
No global latest pointer.
No raw Lean stderr / prompt / completion / CoT / diagnostics.
LibrarianDigest is a materialized view, not source of truth.
```

Risk class is Class 3 unless a restricted authority surface is touched.

## FC Mapping

```text
FC1: rtool/input/Agent-next-turn broadcast view
FC2: barrier tick / replay determinism for half-async review windows
FC3: logs/capsule archive -> derived evidence capsule / dashboard view
Art II: selective broadcast
Art III: selective shielding
```

