# OBS_R022 — H-HET-2 converge TRACE_MATRIX backlink deferral (2026-06-18)

**Scope:** R-022 (`scripts/check_trace_matrix.py`) `[R-022-skip:]` justification ref
for the H-HET-2 mechanism `pub` symbols converging onto `main` via PR #347
(branch `claude/het-converge-2026-06-16`).

## Context

The H-HET-2 routing-policy mechanism (`VERIFY_UCB_PRICE_PRIOR_FLOOR_V1`, architect
ruling 2026-06-15) and its `market_tape_shared` substrate were authored on the
het-carrier-freeze / converge lineage and have not yet faced the R-022
**PR-to-`main`** CI gate (R-022 fires on NEW `pub` symbols in a PR vs `main`; the
het work merged across feature branches, never PR-to-`main`). 49 `pub` symbols
across the H-HET-2 surface lack inline `/// TRACE_MATRIX <FC-id>: <role>`
backlinks; R-022 correctly flags them on PR #347.

## Decision: `[R-022-skip:]` deferral (NOT §J.2, NOT inline backlinks yet)

Deferred via the R-022 commit-message skip-token, with this doc as the
justification ref. **Why skip-token rather than §J.2 registration:** the §J.2
table lives in `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md`, which is
**itself trust-root-pinned** in `genesis_payload.toml`. Editing it to add §J.2
rows changes its sha256 → a genesis re-pin → a Class-4 trust-root touch that is
NOT covered by the merge's §8 grant or the 2026-06-15 standing rule (which
scopes to CAS-schema/module additions, not traceability-doc edits). The
skip-token path is **Class-0** — it touches no pinned file and is an explicitly
sanctioned R-022 remediation.

This is an honest **deferral**, not a traceability claim: these symbols are NOT
yet FC-traced. They map (informally) to FC1 (agent-delta / predicate selection —
routing_policy), Art-II librarian price signal, and Art-0.2 tape-canonical
telemetry (budget_allocation_telemetry / market_tape derives), but the exact
per-symbol `FC-id: role` mapping needs an FC-taxonomy pass.

**Graduation:** a follow-up "H-HET-2 TRACE backlink graduation" atom adds the
inline `/// TRACE_MATRIX` lines under §8 (that atom also re-pins the
trust-root-pinned `src/runtime/mod.rs` + `TRACE_MATRIX_v3` together), once the FC
mapping is ratified.

## Deferred symbols (49)

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
