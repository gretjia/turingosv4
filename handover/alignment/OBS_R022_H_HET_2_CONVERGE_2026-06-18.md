# OBS_R022 — H-HET-2 converge TRACE_MATRIX backlink deferral (2026-06-18)

**Scope:** R-022 (`scripts/check_trace_matrix.py`) §J.2 open-orphan justification for
the H-HET-2 mechanism `pub` symbols converging onto `main` via PR #347
(branch `claude/het-converge-2026-06-16`, merge commit `a4b99a3e`).

## Context

The H-HET-2 routing-policy mechanism (`VERIFY_UCB_PRICE_PRIOR_FLOOR_V1`, architect
ruling 2026-06-15) and its `market_tape_shared` substrate were authored on the
het-carrier-freeze / converge lineage and have not yet faced the R-022
**PR-to-`main`** CI gate (R-022 fires on NEW `pub` symbols in a PR vs `main`;
the het work merged across feature branches, never PR-to-`main`). As a result
49 `pub` symbols across the H-HET-2 surface lack inline `/// TRACE_MATRIX
<FC-id>: <role>` backlinks. R-022 correctly flags them on PR #347.

## Decision: §J.2 open-orphan registration (not inline backlinks yet)

These 49 symbols are registered as **§J.2 open orphans** with this doc as the
justification ref and a graduation target, following the precedented
`OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` bulk-ship pattern. Rationale:

1. **Honest deferral, not a traceability claim.** This registration does NOT
   assert these symbols are FC-traced; it records that backlinks are *pending*
   and scheduled. The symbols map (informally) to FC1 (agent-delta / predicate
   selection — routing_policy), Art-II librarian price signal, and Art-0.2
   tape-canonical telemetry (budget_allocation_telemetry / market_tape derives),
   but the exact per-symbol `FC-id: role` mapping needs an FC-taxonomy pass.
2. **Avoids an unrelated Class-4 trust-root touch.** Inline backlinks on
   `src/runtime/mod.rs:40/45` (the two `pub mod` lines) would change that
   trust-root-pinned file's sha256 → a genesis re-pin → a fresh §8 event. §J.2
   registration is Class-0 (derived-view doc only) and keeps the converge merge
   free of an additional Class-4 surface change.
3. **Reversible / scheduled.** Graduation = a follow-up "H-HET-2 TRACE backlink
   graduation" atom that adds the inline `/// TRACE_MATRIX` lines (and re-pins
   `mod.rs` under §8) once the FC mapping is ratified.

## Registered symbols (49)

`src/judges/lean_judge.rs` (1): VerifyBackend.
`src/market_tape_shared.rs` (20): MarketEvent, MarketTape, new, record,
verify_chain, derive_pools, MODEL_RATES, FALLBACK_IN_UPMT, FALLBACK_OUT_UPMT,
call_micro_usd, verify_chain_lines, derive_banked, derive_cost,
derive_cost_of_pass, derive_total_completion, derive_llm_calls, derive_failures,
head_commit_sha, first_is_genesis, derive_genesis.
`src/runtime/budget_allocation_telemetry.rs` (11): SelectionReason, ModelScoreRow,
BudgetAllocationTelemetry, candidate_pull_sum, MAX_PROPOSAL_TOKENS,
budget_alloc_fields, BudgetAllocationTelemetryError, write_to_cas, decode_bytes,
read_from_cas, read_from_cas_path.
`src/runtime/mod.rs` (2): budget_allocation_telemetry, routing_policy.
`src/runtime/routing_policy.rs` (15): TieBreak, RoutingPolicyConfig, policy_hash,
eps_floor, floor_quota, isqrt, ModelInput, Selection, score_and_select,
RoutingPolicyGenesisPin, RoutingPolicyError, write_policy_config_to_cas,
write_genesis_pin_to_cas, read_genesis_pin_from_cas, read_genesis_pin_from_cas_path.

The authoritative rows live in `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md`
§J.2 (Opened atom: "H-HET-2 converge (PR #347)"; Graduation target: "H-HET-2
TRACE backlink graduation").
