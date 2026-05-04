# TB-16 Comprehensive Arena Plan

**Run ID prefix**: `tb16-post-r3-smoke`
**Out dir**: `handover/evidence/tb_16_real_llm_arena_2026-05-04/post_r3_smoke`
**Wall-clock cap**: 1800000 ms (30 min)
**Compute cap**: 120000 tokens
**Cost ceiling**: $15
**LLM proxy**: http://localhost:18080
**Max-tx per task**: 2

## Sandbox preseed (architect §7.4 CR-16.5 + CR-16.7)

Reuses `runtime::bootstrap::default_pput_preseed_pairs()` (30_000_000 μC on_init mint).
Agent IDs are sandbox-prefixed: `tb7-7-sponsor`, `Agent_user_0`,
`Agent_solver_0..3`, `Agent_verifier_0`. Production-wallet patterns forbidden.

## 6-Task plan (design §4)

### Task 0 — A_happy_path

- **Description**: trivial Lean theorem; solver_0 finds proof; verifier confirms
- **Sponsor**: tb7-7-sponsor
- **Solver**: Agent_solver_0
- **Expected outcome**: OmegaAccepted -> FinalizeReward
- **Exercises**:
    - `TaskOpen`
    - `EscrowLock`
    - `Work`
    - `Verify`
    - `FinalizeReward`
    - `ProposalTelemetry`
    - `VerificationResult`
    - `NodePosition(Long)`

### Task 1 — B_challenge_dismissed

- **Description**: correct proof; solver_3 incorrectly challenges; verifier re-confirms
- **Sponsor**: tb7-7-sponsor
- **Solver**: Agent_solver_0
- **Challenger**: Agent_solver_3
- **Expected outcome**: ChallengeResolve(Released); challenger bond refunded
- **Exercises**:
    - `Work`
    - `Verify`
    - `Challenge`
    - `ChallengeResolve(Released)`
    - `NodePosition(ChallengeShort)`

### Task 2 — C_challenge_upheld

- **Description**: invalid proof; solver_3 correctly challenges; verifier confirms
- **Sponsor**: tb7-7-sponsor
- **Solver**: Agent_solver_0
- **Challenger**: Agent_solver_3
- **Expected outcome**: ChallengeResolve(UpheldDeferred); slash deferred to RSP-3.2
- **Exercises**:
    - `Work`
    - `Verify`
    - `Challenge`
    - `ChallengeResolve(UpheldDeferred)`

### Task 3 — D_exhaustion

- **Description**: hard Lean theorem; solver_1 exhausts MAX_TX; bankruptcy triggers autopsy
- **Sponsor**: tb7-7-sponsor
- **Solver**: Agent_solver_1
- **Expected outcome**: TerminalSummary + EvidenceCapsule; TaskBankruptcy + AgentAutopsyCapsule
- **Exercises**:
    - `TerminalSummary`
    - `EvidenceCapsule`
    - `TaskBankruptcy`
    - `AgentAutopsyCapsule`

### Task 4 — E_expiry

- **Description**: sponsor opens; no solver picks up; deadline elapses
- **Sponsor**: tb7-7-sponsor
- **Solver**: (none)
- **Expected outcome**: TaskExpire; sponsor refund
- **Exercises**:
    - `TaskOpen`
    - `EscrowLock`
    - `TaskExpire`

### Task 5 — F_complete_set_market

- **Description**: Agent_user_0 sponsors; MarketSeed + CompleteSetMint + redeem
- **Sponsor**: Agent_user_0
- **Solver**: Agent_solver_2
- **Expected outcome**: MarketSeed + CompleteSetMint + (resolution) + CompleteSetRedeem
- **Exercises**:
    - `MarketSeed`
    - `CompleteSetMint`
    - `CompleteSetRedeem`
    - `ConditionalCollateral`
    - `ConditionalShareBalances`

## Execution model

Atom 5 (this binary) v0 scope: emit this plan + sandbox preseed manifest.
Atom 6 (`handover/tests/scripts/run_real_llm_arena.sh`) executes the plan:
1. Bootstrap a fresh `runtime_repo/` + `cas/` via `evaluator --bootstrap-only`.
2. For each task A..F, subprocess `evaluator` with task-specific env vars
   (`TURINGOS_USER_TASK_MODE`, `TURINGOS_USER_TASK_BOUNTY_MICRO`,
   `TURINGOS_FORCE_CHALLENGE`, `TURINGOS_FORCE_EXHAUSTION`, etc.).
3. After all 6 tasks complete, run `audit_tape` over the resulting tape.
4. Run `audit_tape_tamper` (3 corruptions) over copies.
5. Run `generate_markov_capsule` to emit MARKOV_TB-16_<DATE>.json.
6. Run `audit_dashboard` to render dashboard.txt.
7. Re-run `audit_tape` to assert byte-identical verdict.json.

## Ship gate (design §7.1)

PASS iff:
1. Evaluator subprocess completes within 30-min wall clock + cost ceiling.
2. All 13 expected tx_kinds appear in tape_root.tx_kind_counts.
3. All 6 CAS object types reachable.
4. verdict.json `verdict == "PROCEED"` with all 38 assertions PASS.
5. Dashboard renders all 16 sections (incl. §15 live regen + §16 SANDBOX banner).
6. First Markov capsule emitted; constitution_hash matches.
7. Replay determinism: byte-identical verdict.json across two runs.

## Forbidden (architect §7.6 verbatim)

- No public chain. No real-money market. No external domain.
- No unbounded leverage. No AMM trading. No DPMM / pro-rata.
- No medical/legal/financial domains. No production user funds.

## Halt triggers (architect §7.7)

Instant stop (no round-2):
- Conservation failure (Layer D #17/18/19/20).
- Raw log leak (Layer F #28/29/30/31).
- Price-as-truth (re-dispatch reads compute_price_index).
- Non-sandbox funds used (production wallet pattern).
- Unresolved evidence gap (CAS missing for any L4 CID).
