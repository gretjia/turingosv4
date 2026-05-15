# CODEX REAL-6C Implementation Review

Task: REAL-6C ConvictionBudget / PnL Feedback implementation review.

Risk class: Class 4 package, because Trust Root pinned `src/runtime/mod.rs` and `genesis_payload.toml` were touched.

Touched FC / invariants reviewed:

- FC1 role action / economic feedback.
- FC2 replay-derived PnL / QState as canonical materialized state.
- FC3 materialized audit view.
- Art. III scoped prompt / shielding.

## Findings

### 1. Production defect: SG-6C.3 / SG-6C.4 / SG-6C.6 are helper-only, not production-visible

`src/runtime/real6_conviction_budget.rs:113` renders a scoped ConvictionBudget string, but the production prompt path is not wired to it. `src/sdk/prompt.rs:48` accepts `your_position`, not a ConvictionBudget block, and the evaluator prompt assembly at `experiments/minif2f_v4/src/bin/evaluator.rs:3223` passes `your_position` only. Repository usage confirms `render_scoped_conviction_budget_summary` is referenced only by the new test, report, and diff evidence.

Similarly, `src/runtime/real6_conviction_budget.rs:84` defines `conviction_action_allowed`, but the active role router at `src/runtime/real5_roles.rs:513` routes Trader/Challenger actions without consulting ConvictionBudget. This means low-balance high-risk role availability is not actually blocked by the REAL-6C package.

For autopsy, `src/runtime/real6_conviction_budget.rs:135` returns an in-memory `AgentAutopsyCapsule`, but it does not use the existing CAS writer contract at `src/runtime/autopsy_capsule.rs:253`, does not anchor evidence CIDs, and hardcodes `created_at_logical_t` / `created_at_round` to zero at `src/runtime/real6_conviction_budget.rs:178`. So SG-6C.6 is not a real "generated after significant loss" tape/CAS emission.

Impact: the implementation can pass the targeted test while production agents do not see the ConvictionBudget, production high-risk actions are not blocked by this helper, and significant-loss autopsies are not emitted to CAS/ChainTape. This is a production defect, not merely a test naming issue.

### 2. Class-4 evidence/package defect: recorded REAL-6C diff is not isolated to REAL-6C

The dev evidence diff at `handover/evidence/dev_self_hosting/dev_1778824777461_1299406/artifacts/diff.patch:1` starts with a broad `genesis_payload.toml` Trust Root rehash, not just the `src/runtime/mod.rs` row. It includes restricted or explicitly out-of-scope Trust Root rows, including:

- `src/kernel.rs` at `handover/evidence/dev_self_hosting/dev_1778824777461_1299406/artifacts/diff.patch:16`
- `src/bus.rs` at `handover/evidence/dev_self_hosting/dev_1778824777461_1299406/artifacts/diff.patch:19`
- `src/bottom_white/cas/schema.rs` at `handover/evidence/dev_self_hosting/dev_1778824777461_1299406/artifacts/diff.patch:158`
- `src/state/sequencer.rs` at `handover/evidence/dev_self_hosting/dev_1778824777461_1299406/artifacts/diff.patch:165`
- `src/state/typed_tx.rs` at `handover/evidence/dev_self_hosting/dev_1778824777461_1299406/artifacts/diff.patch:172`
- `src/bottom_white/ledger/system_keypair.rs` at `handover/evidence/dev_self_hosting/dev_1778824777461_1299406/artifacts/diff.patch:190`
- `src/bottom_white/ledger/transition_ledger.rs` at `handover/evidence/dev_self_hosting/dev_1778824777461_1299406/artifacts/diff.patch:193`

The `src/runtime/mod.rs` diff also exports `real5_roles`, `real6_task_outcome`, and `real6_attempt_prediction` together with `real6_conviction_budget` at `handover/evidence/dev_self_hosting/dev_1778824777461_1299406/artifacts/diff.patch:202`, so the recorded artifact is not a clean REAL-6C-only package.

Impact: the audit cannot substantiate the stated REAL-6C restriction "no sequencer admission / TypedTx schema / canonical signing payload / wallet / kernel / bus change" from this package evidence. Even if those source edits pre-existed, the Trust Root authority changes are bundled into the REAL-6C run artifact. This is a Class-4 packaging/evidence blocker.

### 3. Test-scaffold gap: the gates prove helpers, not FC1/FC2/FC3 behavior

`tests/constitution_real6_conviction_budget.rs:36` uses source-grep plus an in-memory `QState`; it does not replay ChainTape/CAS or prove provenance of the input state. `tests/constitution_real6_conviction_budget.rs:88` checks the helper renderer directly rather than the real prompt delivered to the agent. `tests/constitution_real6_conviction_budget.rs:116` checks `conviction_action_allowed` directly rather than the role/action router. `tests/constitution_real6_conviction_budget.rs:153` checks an in-memory capsule return rather than CAS persistence or chain anchoring.

Impact: SG-6C.1 / SG-6C.3 / SG-6C.4 / SG-6C.6 are under-tested at the integration boundary. The tests are useful unit coverage for a pure helper, but they are insufficient as Class-4 acceptance gates for the architect claims.

### 4. Evidence report is stale/incomplete relative to the recorded run

`handover/evidence/real6_conviction_budget/REAL6C_CONVICTION_BUDGET_REPORT.md:64` lists only command_0001 and command_0002, while the dev run records command_0003 through command_0007, including fmt red/green, targeted test rerun, Trust Root boot test, and constitution gates. The raw run evidence does contain those later commands in `handover/evidence/dev_self_hosting/dev_1778824777461_1299406/events.jsonl`, but the report does not package them.

Impact: not a production defect by itself, but the human-facing evidence package understates the actual verification trail and should not be treated as the complete report.

## Non-Findings / Checks

- The new `src/runtime/real6_conviction_budget.rs` helper itself is pure read-side over `QState` / existing G3 PnL helpers; I found no balance mutation, sequencer admission, TypedTx schema, signing payload, wallet, kernel, or bus code inside that file.
- The helper uses integer `i64` / `u128` arithmetic only; I found no `f64` / `f32` money-path math in the REAL-6C module.
- The scoped renderer does not include another agent's positions in the direct helper-level fixture.
- The current `genesis_payload.toml:219` row for `src/runtime/mod.rs` matches the current file hash `236f3ffc67e9c3680b115a15ee3e9295e0f512b03abd50394b40da9448427336`.
- Recorded verification evidence exists for `cargo test --test constitution_real6_conviction_budget` exit 0, `cargo fmt --all -- --check` exit 0 after red, `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` exit 0, and `bash scripts/run_constitution_gates.sh` exit 0 with `436 passed / 0 failed / 1 ignored`.

## Residual Risk

The core derived view helper is small and appears low-risk in isolation. The blocking risk is that the package currently claims production FC1/Art.III effects that are not wired, and the Class-4 evidence artifact is contaminated with unrelated Trust Root rehashes. A safe follow-up would either explicitly re-scope REAL-6C as "helper-only, gates 6C.3/6C.4/6C.6 deferred with user approval" or wire the helper into the real prompt/action/autopsy paths and regenerate clean, isolated Class-4 evidence.

VETO
