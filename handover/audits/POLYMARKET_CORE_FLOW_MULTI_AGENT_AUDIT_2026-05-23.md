# POLYMARKET_CORE_FLOW_MULTI_AGENT_AUDIT - 2026-05-23

**Scope**: local Class 3 diff implementing the Polymarket core user flow under
`TB-POLYMARKET-CORE-FLOW`.

**Risk class**: 3.

**Orchestrator**: Codex.

## Audit Team

| Role | Auditor | Verdict |
|------|---------|---------|
| Constitution / Flowchart | clean-context subagent | NO-VIOLATION |
| Karpathy Architect | clean-context subagent | NO-VIOLATION |
| Software Engineering | clean-context subagent | NO-VIOLATION after retry-flow fix |
| Economy / Evidence / Security | clean-context subagent | NO-VIOLATION |
| Real-User E2E | clean-context subagent | NO-VIOLATION - CORE-MECHANISM |
| Codex witness | clean-context Codex witness | NO-VIOLATION |
| AGY witness | `agy --print` independent witness | NO-VIOLATION |

AGY replaces Gemini for this atom per the user's 2026-05-23 instruction.

## Final AGY Finding Summary

AGY checked the current local diff and found:

- no constitutional violation;
- no reconstruction failure;
- no second-source drift;
- no API-key persistence or evidence leak;
- no hidden Class 4 surface;
- no new heavy abstraction.

AGY specifically cited the ChainTape/CAS replay path, root CAS reconstruction
for proposal telemetry, websocket-as-hint design, encrypted agent keystore, and
child-process-only LLM key environment aliases.

Final AGY verdict: `NO-VIOLATION`.

## Codex Witness Summary

The final clean-context Codex witness returned `NO-VIOLATION` on the same
post-cleanup diff. No unresolved `VIOLATION-FOUND`, `RECONSTRUCTION-FAILURE`,
or `SECOND-SOURCE-DRIFT` remained.

## Defects Found And Closed

- Retry after an earlier rejected WorkTx originally saw an open task market and
  skipped finalization. Fixed by reusing open task state only when safe, seeding
  escrow if needed, and continuing to WorkTx/MarketSeed/Verify/Finalize/EventResolve.
- Proposal CID originally risked pointing directly at artifacts without an
  inspectable market proposal envelope. Fixed by making `WorkTx.proposal_cid`
  point to `ProposalTelemetry`, whose `proposal_artifact_cid` reconstructs the
  delivered artifact or rejection capsule in the root CAS.
- Web live updates risked becoming a source of truth. Fixed by broadcasting only
  `agent_attempt_update { session_id }`; panel refetches the chain-derived view.

## Evidence Checklist

- `cargo check --features web`: PASS.
- frontend build and tests: PASS.
- targeted Polymarket and web flow Rust tests: PASS.
- serial workspace test: PASS.
- constitution gates: PASS.
- constitution matrix drift: PASS.
- trace matrix/R-022 checks: PASS.
- diff whitespace and restricted-surface fences: PASS.

This file records audit disposition only; command evidence is recorded through
the local `turingos_dev` run and PR body.
