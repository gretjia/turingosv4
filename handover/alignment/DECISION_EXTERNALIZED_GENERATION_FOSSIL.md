# DECISION: Externalized Generation Fossil

Every externalized role action must become fossilized on ChainTape / CAS.

Decision:

- Role action output is parsed into a typed payload.
- Typed payload is recorded in AttemptTelemetry / ActionTelemetry or an equivalent role trace.
- Predicate/audit routing sends the action to L4, L4.E, or a CAS reason trace.
- Raw private CoT, raw prompt bodies, raw completions, and raw diagnostics are not public evidence.

Dashboard/report output remains a materialized view only.
