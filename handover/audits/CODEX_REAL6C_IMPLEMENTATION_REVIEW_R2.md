# CODEX REAL-6C Implementation Review R2

Task: REAL-6C ConvictionBudget / PnL Feedback R2 implementation review after
R1 VETO remediation.

Risk class: Class 4 package because Trust Root pinned `src/runtime/mod.rs`,
`experiments/minif2f_v4/src/bin/evaluator.rs`, and `genesis_payload.toml` were
touched.

Touched FC / invariants reviewed:

- FC1 role action / economic feedback.
- FC2 replay-derived PnL / QState as canonical materialized state.
- FC3 materialized audit view / CAS autopsy evidence.
- Art. III scoped prompt / shielding.

## Findings

### 1. Production/tape semantics CHALLENGE: helper returns PolicyViolation, but production L4.E still records the generic predicate-failure path

`src/runtime/real6_conviction_budget.rs:131` returns
`RoleActionRoute::L4E { rejection_class: RejectionClass::PolicyViolation, ... }`
for below-cap high-risk actions, and the public summary is constructed at
`src/runtime/real6_conviction_budget.rs:142`.

The evaluator production branch does block the legacy action, but it discards
the route's `rejection_class`: the `RoleActionRoute::L4E` arm at
`experiments/minif2f_v4/src/bin/evaluator.rs:3405` only carries
`public_summary` into `real5_emit_role_gateway_rejection_to_l4e`, then sets
`real5_legacy_action_permitted = false` at
`experiments/minif2f_v4/src/bin/evaluator.rs:3454`. The actual L4.E emission
uses `AttemptOutcome::Aborted` at
`experiments/minif2f_v4/src/bin/evaluator.rs:739`, and the failure-path WorkTx
is emitted with `predicate_passes=false` at
`experiments/minif2f_v4/src/bin/evaluator.rs:689`.

On the sequencer side, `AttemptOutcome::Aborted` maps back to the base class at
`src/state/sequencer.rs:685`, and the base class is derived from the WorkTx
transition error at `src/state/sequencer.rs:4748`. Therefore the production
L4.E record is not the `PolicyViolation` class claimed by
`handover/evidence/real6_conviction_budget/REAL6C_CONVICTION_BUDGET_REPORT.md:23`
and `genesis_payload.toml:164`; it is the existing predicate-failure rejection
path with the ConvictionBudget reason present in side evidence, not as the
authoritative L4.E class.

Impact: this is not a full SG-6C.4 failure, because the high-risk action is
blocked before tool dispatch (`experiments/minif2f_v4/src/bin/evaluator.rs:3504`).
It is a production observability/tape-semantics defect relative to the package
claim "L4.E policy violation." Close by either threading the helper's
`rejection_class` / summary into the actual L4.E evidence path, or narrowing the
claim/tests to the current "blocked via predicate-failure WorkTx" semantics.

### 2. Evidence packaging CHALLENGE: R2 scope is reviewable, but the dev-run package boundary is still stale/dirty for Class 4 closeout

The R2 source scope can be reviewed from current file refs plus command_0016 and
command_0017, and the report correctly disclaims event_0008 as contaminated at
`handover/evidence/real6_conviction_budget/REAL6C_CONVICTION_BUDGET_REPORT.md:92`.
However, the turingos_dev manifest still does not match the R2 production
touches: `DevTaskManifest.json:12` lists allowed paths but omits both
`src/sdk/your_position.rs` and `experiments/minif2f_v4/src/bin/evaluator.rs`,
while the report states those files are in scope at
`handover/evidence/real6_conviction_budget/REAL6C_CONVICTION_BUDGET_REPORT.md:22`
and `handover/evidence/real6_conviction_budget/REAL6C_CONVICTION_BUDGET_REPORT.md:23`.

The formal FC witness still points only at the broad `artifacts/diff.patch` at
`handover/evidence/dev_self_hosting/dev_1778824777461_1299406/FCWitnessManifest.json:8`,
the same contaminated diff recorded at
`handover/evidence/dev_self_hosting/dev_1778824777461_1299406/events.jsonl:9`.
The later scoped commands are useful evidence, but they are command outputs:
command_0016 is a large dirty-file diff
(`handover/evidence/dev_self_hosting/dev_1778824777461_1299406/events.jsonl:17`),
and command_0017 is a grep-filtered diff
(`handover/evidence/dev_self_hosting/dev_1778824777461_1299406/events.jsonl:18`),
not a clean record-diff artifact with a corrected allowed-path contract.

Impact: this is an evidence packaging gap, not a production code defect. It
does mean R1 VETO finding #2 is not fully closed for Class 4 package closeout.
Close with a fresh scoped dev run or manifest/diff artifact that includes the
actual R2 touched paths and excludes the historical dirty-tree changes, or with
an explicit architect waiver that command_0016/0017 plus exact file refs are the
accepted package boundary.

## R1 VETO Closure

- R1 #1 helper-only implementation: closed for production reachability. The
  prompt path now appends ConvictionBudget through
  `src/sdk/your_position.rs:154`, and the evaluator passes the rendered
  `your_position` block into the production prompt at
  `experiments/minif2f_v4/src/bin/evaluator.rs:3220`. The role gateway now calls
  `route_role_action_with_conviction_budget` through
  `experiments/minif2f_v4/src/bin/evaluator.rs:421` and invokes it at
  `experiments/minif2f_v4/src/bin/evaluator.rs:3386`. Significant-loss autopsy
  emission is wired at `experiments/minif2f_v4/src/bin/evaluator.rs:6008`.
  Caveat: Finding #1 remains for the exact L4.E class/summary semantics.
- R1 #2 contaminated record-diff artifact: still open as a package challenge.
  The contaminated event_0008 is now explicitly disclaimed, but the replacement
  evidence is not yet a clean Class 4 package diff/manifest.

## Gate Checks

- SG-6C.1 / SG-6C.2: `derive_conviction_budget` derives from
  `compute_agent_pnl` and `bankruptcy_risk_cap_micro` at
  `src/runtime/real6_conviction_budget.rs:52`; I found no REAL-6C HashMap PnL
  sidecar source of truth.
- SG-6C.3: the agent prompt sees a scoped ConvictionBudget summary through
  `src/sdk/your_position.rs:154` and
  `experiments/minif2f_v4/src/bin/evaluator.rs:3179`. The REAL-6C test fixture
  checks no other-agent leak at `tests/constitution_real6_conviction_budget.rs:85`.
- SG-6C.4 / SG-6C.5: the helper blocks below-cap Trader/MarketMaker market
  actions and Challenger challenge actions at
  `src/runtime/real6_conviction_budget.rs:88`, while preserving the agent's
  read-side budget. Production dispatch is skipped when the role gateway rejects
  at `experiments/minif2f_v4/src/bin/evaluator.rs:3504`.
- SG-6C.6: `write_significant_loss_autopsy_to_cas` uses the existing CAS writer
  at `src/runtime/real6_conviction_budget.rs:213`, and the evaluator post-turn
  path invokes it at `experiments/minif2f_v4/src/bin/evaluator.rs:6028`. The
  gate restores the CAS-resident capsule at
  `tests/constitution_real6_conviction_budget.rs:212`.

## Non-Findings / Regression Checks

- I found no REAL-6C change to sequencer admission rules, TypedTx schema or
  discriminants, canonical signing payloads, wallet, kernel, or bus code in the
  reviewed REAL-6C-specific source paths. The current worktree does contain
  broad unrelated dirty history, so this statement is scoped to the R2
  ConvictionBudget hunks and not to the whole dirty branch.
- The REAL-6C helper uses integer `i64` / `u128` arithmetic only; I found no new
  `f32` / `f64` money-path math in `src/runtime/real6_conviction_budget.rs`.
- The prompt summary is per-viewer and does not expose raw diagnostics or
  another agent's positions. Autopsy private detail is written as
  `CapsulePrivacyPolicy::AuditOnly`.
- `genesis_payload.toml:164` matches current
  `experiments/minif2f_v4/src/bin/evaluator.rs` hash
  `c1cd185d49ff1020e136faafc8c379418d938436a634883f9b239daa78f7c8ec`, and
  `genesis_payload.toml:219` matches current `src/runtime/mod.rs` hash
  `236f3ffc67e9c3680b115a15ee3e9295e0f512b03abd50394b40da9448427336`.

## Verification Evidence Reviewed

- `command_0011`: `cargo test --test constitution_real6_conviction_budget` exit
  0, 4 passed.
- `command_0012`: `cargo check -p minif2f_v4 --bin evaluator` exit 0.
- `command_0013`: `cargo fmt --all -- --check` exit 0.
- `command_0015`: `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` exit 0.
- `command_0018`: `cargo test --test constitution_g3_your_position_prompt` exit
  0, 8 passed.
- `command_0019`: `bash scripts/run_constitution_gates.sh` exit 0, 436 passed /
  0 failed / 1 ignored.
- `command_0020`: `cargo test --workspace --no-fail-fast -- --test-threads=1`
  exit 0.

CHALLENGE
