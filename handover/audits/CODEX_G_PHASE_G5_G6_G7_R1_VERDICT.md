# Clean-Context Codex Audit — TB-G G5/G6/G7 + SG-G Packet R1

Date: 2026-05-14

Reviewer: clean-context Codex subagent

Verdict: CHALLENGE

## Findings

1. **CHALLENGE: new public runtime APIs lack required `/// TRACE_MATRIX`
   backlinks, so the closeout is likely to fail R-022/pre-commit alignment
   enforcement.**

   This is a harness/alignment defect, not a production behavior defect. The
   R-022 checker blocks new `pub` items without a nearby
   `/// TRACE_MATRIX ...` doc comment or approved orphan/skip path:
   `scripts/check_trace_matrix.py:8`,
   `scripts/check_trace_matrix.py:322`,
   `scripts/check_trace_matrix.py:366`.

   Affected examples cited by the reviewer:

   - `src/runtime/agent_scheduler.rs:10`
   - `src/runtime/agent_scheduler.rs:34`
   - `src/runtime/agent_role_classifier.rs:7`
   - `src/runtime/agent_role_classifier.rs:50`
   - `src/runtime/g7_structural_smoke.rs:4`
   - `src/runtime/g7_structural_smoke.rs:25`
   - `src/sdk/market_context.rs:21`
   - `src/sdk/market_context.rs:61`

   Required closure: add trace backlinks or an explicit accepted orphan
   justification, then run the R-022/pre-commit path.

## Checked / No Production Defect Found

- No hidden Class-4 source change found. The diff is empty for
  `src/state/sequencer.rs`, `src/state/typed_tx.rs`, `src/kernel.rs`,
  `src/bus.rs`, wallet, and CAS schema.
- The `genesis_payload.toml` change appears to be Trust Root rehash metadata
  for `src/runtime/mod.rs` and `src/bin/audit_dashboard.rs`, not a genesis
  schema change.
- Price remains observe-only in the changed surfaces. `market_context` adds
  trace hints and filters open challenged targets without predicate authority,
  and §J states price is signal, not truth with no predicate authority.
- No raw prompt/completion/CoT leak found in G5/G6/G7. §I is built from public
  per-agent counts, and the role classifier consumes only activity counters.
- The G7 packet/report is framed as structural regeneration, not a new
  run6-volume claim.

## Verdict

CHALLENGE
