# TB-POLYMARKET-CORE-FLOW - Charter (2026-05-23)

**Risk class**: 3 - auth/key handling, money/economy state, CAS/ChainTape evidence,
market state, and production user-flow wiring.

**Orchestrator**: Codex, acting only as implementation coordinator and evidence
collector for this atom.

**User directive**: implement the multi-agent audit team and delivery plan for
the constitutional flowchart-backed Polymarket user flow. The user explicitly
replaced Gemini witness requirements with `agy` for this work.

## Scope

This charter covers the Class 3 PR that makes Polymarket a core mechanism in
the `/welcome -> spec grill -> generate -> panel -> artifact -> restart replay`
user flow:

- unblock real-user spec grill by increasing triage budget and disabling
  provider thinking for triage calls;
- keep welcome API keys in child-process environment aliases only;
- align DeepSeek-direct welcome init with the DeepSeek model namespace while
  preserving SiliconFlow defaults for SiliconFlow endpoints;
- run web generate with `--n-parallel-workers 3` by default while CLI remains
  default `1`;
- degrade old workspaces to `N=1` when worker preseed is missing;
- emit worker attempts and finalization through the canonical durable ChainTape
  and CAS path;
- derive websocket/panel state from replayed chain state, with websocket events
  acting only as live hints;
- keep open markets winner-free and only expose a winner after finalized replay.

## FC Binding

- FC2-N16 / FC2-N21: fresh init and Q0 preseed for worker/verifier/provider
  economic identities.
- FC1-N5 / FC1-N7: real user input reaches worker attempts without leaking
  private diagnostics or API keys.
- FC1-N10 / FC1-N11 / FC1-N13 / FC1-N14: candidate output, predicate result,
  sequencer write, and accepted Q advance remain canonical.
- FC3-N31 / FC3-N36 / FC3-N37 / FC3-N38: archived logs, worker swarm,
  bottom tools, and Q update are reconstructable from ChainTape plus CAS.

## Hard Non-Scope

This PR must not touch the following Class 4 surfaces:

- `src/state/typed_tx.rs`
- `src/state/sequencer.rs`
- `src/bus.rs`
- `src/bottom_white/cas/schema.rs`
- `constitution.md`
- root `genesis_payload.toml`
- canonical signing payload definitions
- sequencer admission rules

If any of the above becomes necessary, stop and reclassify as Class 4.

## Ship Gates

The PR cannot ship until:

1. Class 3 implementation evidence exists in tests or real replay evidence.
2. `cargo check --features web` passes.
3. targeted Polymarket/user-flow tests pass.
4. frontend build and tests pass.
5. `cargo test --workspace --no-fail-fast` passes.
6. `bash scripts/run_constitution_gates.sh` passes.
7. `cargo test --test constitution_matrix_drift` passes.
8. trace matrix/R-022 style checks pass.
9. clean-context Codex witness returns no unresolved violation.
10. `agy` independent witness returns no unresolved violation.

## Kill Criteria

- Any API key persisted to disk, logs, CAS, ChainTape, or evidence.
- Any dashboard/panel/websocket value becoming canonical market truth.
- Any WorkTx proposal CID that cannot reconstruct its artifact or rejection
  capsule from the root CAS.
- Any open market exposing a winner.
- Any N=3 claim on an old workspace that lacks worker preseed.
- Any hidden Class 4 surface modification.
