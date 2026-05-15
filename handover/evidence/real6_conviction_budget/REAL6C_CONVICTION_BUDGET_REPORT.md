# REAL-6C ConvictionBudget / PnL Feedback Report

## Scope

Architect target:

```text
REAL-6C — Conviction Budget / PnL Feedback
目标

恢复 v3 的经济压力，但保持 v4 合宪。

原则
free cognition
paid conviction
```

Implementation scope:

- Added `src/runtime/real6_conviction_budget.rs` as a pure materialized view over `QState` plus existing G3 `agent_pnl` helpers.
- Exported the helper from `src/runtime/mod.rs`.
- Wired the scoped ConvictionBudget into the production prompt read-view through `src/sdk/your_position.rs`.
- Wired the REAL-5 production role gateway in `experiments/minif2f_v4/src/bin/evaluator.rs` through `route_role_action_with_conviction_budget`, so below-cap Trader/MarketMaker/Challenger high-risk actions are blocked before production dispatch. The current authoritative L4.E lane remains the existing failure-path WorkTx predicate-failure path; the ConvictionBudget reason is carried in role/attempt side evidence.
- Wired significant-loss ConvictionBudget autopsies through the existing CAS `write_autopsy_capsule` contract in the evaluator post-turn path.
- Rehashed Trust Root for the modified pinned `src/runtime/mod.rs`.
- Rehashed Trust Root for the modified pinned `experiments/minif2f_v4/src/bin/evaluator.rs`.
- Added `tests/constitution_real6_conviction_budget.rs`.

No sequencer admission, typed transaction schema/discriminant, canonical signing payload, wallet, kernel, or bus changes were introduced by REAL-6C.

## Gate Mapping

| Gate | Evidence |
| --- | --- |
| SG-6C.1 PnL derived from ChainTape/CAS | `derive_conviction_budget` calls `compute_agent_pnl` and `bankruptcy_risk_cap_micro`, both derived from canonical replay/QState surfaces. |
| SG-6C.2 No PnL HashMap sidecar source-of-truth | REAL-6C module has no sidecar PnL table and no map-backed PnL source; test greps the source. |
| SG-6C.3 Agent prompt sees scoped PnL summary | `render_scoped_conviction_budget_summary` renders only the requested agent's budget/PnL summary. Regression fixture checks another agent's position does not leak. |
| SG-6C.4 Low-balance agent blocked from high-risk market actions | `conviction_action_allowed` blocks below-cap Trader/MarketMaker market conviction and Challenger high-risk challenge actions; evaluator production gateway uses `route_role_action_with_conviction_budget` before dispatch. |
| SG-6C.5 Low-balance agent not erased / reset | `derive_conviction_budget` still returns the agent id and read-side budget even below cap. |
| SG-6C.6 AutopsyCapsule generated after significant loss | `write_significant_loss_autopsy_to_cas` writes an AuditOnly `AgentAutopsyCapsule` through the existing CAS writer; evaluator post-turn path invokes it for significant losses. |

## Interpretation

REAL-6C is an observability and role-availability helper. It does not move money, admit or reject transactions by itself, or let price become truth.

The architecture remains:

```text
free cognition
paid conviction
```

Below risk cap:

```text
cannot Trader/Challenger high-risk action
can still observe/read/abstain/solve/possibly verify
```

## Commands

Recorded under `turingos_dev` run:

```text
dev_1778824777461_1299406
```

Current command evidence:

```text
command_0001: cargo test --test constitution_real6_conviction_budget
  exit 101 (intentional RED: module absent)

command_0002: cargo test --test constitution_real6_conviction_budget
  exit 0

command_0003: cargo fmt --all -- --check
  exit 1 (formatting red)

command_0004: cargo fmt --all -- --check
  exit 0

command_0005: cargo test --test constitution_real6_conviction_budget
  exit 0

command_0006: cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo
  exit 0

command_0007: bash scripts/run_constitution_gates.sh
  exit 0, 436 passed / 0 failed / 1 ignored

event_0008: broad record-diff artifact
  contaminated by historical branch dirty tree; not used as clean REAL-6C package evidence

command_0009: cargo test --workspace --no-fail-fast -- --test-threads=1
  exit 0 (pre-VETO workspace proof; superseded by R2 remediation commands)

command_0010: cargo test --test constitution_real6_conviction_budget
  exit 101 (R2 red: test pattern partially moved a String)

command_0011: cargo test --test constitution_real6_conviction_budget
  exit 0

command_0012: cargo check -p minif2f_v4 --bin evaluator
  exit 0

command_0013: cargo fmt --all -- --check
  exit 0

command_0014: cargo test --test constitution_real6_conviction_budget
  exit 0

command_0015: cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo
  exit 0

command_0016: scoped git diff over REAL-6C-touched source/test files
  exit 0

command_0017: filtered scoped diff for REAL-6C-specific hunks
  exit 0

command_0018: cargo test --test constitution_g3_your_position_prompt
  exit 0

command_0019: bash scripts/run_constitution_gates.sh
  exit 0, 436 passed / 0 failed / 1 ignored

command_0020: cargo test --workspace --no-fail-fast -- --test-threads=1
  exit 0
```

Clean R3b closeout evidence:

```text
dev_1778827530259_1394036

command_0001: cargo fmt --all -- --check
  exit 0

command_0002: cargo test --test constitution_real6_conviction_budget
  exit 0

command_0003: cargo test --test constitution_g3_your_position_prompt
  exit 0

command_0004: cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo
  exit 0

command_0005: bash scripts/run_constitution_gates.sh
  exit 0, 436 passed / 0 failed / 1 ignored

command_0006: cargo test --workspace --no-fail-fast -- --test-threads=1
  exit 0

command_0007: scoped git status for REAL-6C paths
  exit 0

command_0008: sha256sum for REAL-6C source/test/report/audit paths
  exit 0
```

The intermediate run `dev_1778827426590_1391207` is intentionally not used as
clean package evidence because two `record-command` writes were started
concurrently and may have collided on command numbering. R3b supersedes it with
strictly serial harness writes.

## Audit R1 VETO Remediation

`handover/audits/CODEX_REAL6C_IMPLEMENTATION_REVIEW.md` returned `VETO` on two classes:

1. helper-only implementation;
2. contaminated branch-level diff evidence due pre-existing dirty Trust Root rows.

Remediation:

- SG-6C.3 is now production-visible through `src/sdk/your_position.rs`.
- SG-6C.4 is now production-visible through the evaluator's REAL-5 role gateway. This closes the blocking requirement without claiming a new authoritative L4.E rejection class.
- SG-6C.6 now uses the existing CAS autopsy writer and evaluator post-turn path.
- The original broad `record-diff` artifact is treated as contaminated historical branch evidence. R2 review should rely on command_0016 / command_0017 scoped diff artifacts plus exact file hashes and verification commands, not on the broad command generated before VETO remediation.
