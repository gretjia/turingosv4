# CODEX REAL-6C Implementation Review R3

Task: REAL-6C ConvictionBudget / PnL Feedback R3b closeout review.

Risk class: Class 4 package. Review scope is the current repository files,
current REAL-6C diff surface, and the specified evidence paths only. I did not
use implementation-thread history as evidence.

Touched FC / invariants reviewed:

- FC1 role action / predicate-failure routing / production dispatch boundary.
- FC2 ChainTape-derived QState / PnL materialized view.
- FC3 CAS-resident audit evidence / AutopsyCapsule shielding.
- Art. III scoped prompt / per-viewer shielding.

## Findings

No blocking findings.

R2 challenge #1 is closed by claim narrowing, not by changing the authoritative
L4.E class. The report now states that below-cap Trader/MarketMaker/Challenger
high-risk actions are "blocked before production dispatch" and that the
"current authoritative L4.E lane remains the existing failure-path WorkTx
predicate-failure path" with the ConvictionBudget reason in role/attempt side
evidence (`handover/evidence/real6_conviction_budget/REAL6C_CONVICTION_BUDGET_REPORT.md:23`).
The Trust Root comment carries the same narrowed language and explicitly says
there is no sequencer admission, TypedTx schema/discriminant/signing payload,
wallet, kernel, or bus change (`genesis_payload.toml:164`).

The code still has the helper-level `RoleActionRoute::L4E { rejection_class:
PolicyViolation, ... }` shape for role-gateway decisions
(`src/runtime/real6_conviction_budget.rs:131` and
`src/runtime/real6_conviction_budget.rs:142`), but production does not claim
that as the authoritative L4.E class. The evaluator discards the helper's
`rejection_class` at the gateway arm and keeps only `public_summary`
(`experiments/minif2f_v4/src/bin/evaluator.rs:3405`), writes side evidence as
`AttemptOutcome::Aborted` (`experiments/minif2f_v4/src/bin/evaluator.rs:739`),
then emits the existing predicate-failure WorkTx with `predicate_passes=false`
(`experiments/minif2f_v4/src/bin/evaluator.rs:689`). This matches the narrowed
R3 claim.

SG-6C.3 remains production-visible. The per-viewer `your_position` renderer
appends the scoped ConvictionBudget summary
(`src/sdk/your_position.rs:154`), and the prompt builder renders that block
under `=== Your Position ===` (`src/sdk/prompt.rs:163`). The evaluator derives
the prompt QState snapshot and passes `render_your_position` into
`build_agent_prompt` (`experiments/minif2f_v4/src/bin/evaluator.rs:3179` and
`experiments/minif2f_v4/src/bin/evaluator.rs:3220`).

SG-6C.4 / SG-6C.5 remain production-visible. The REAL-5 gateway adapter calls
`route_role_action_with_conviction_budget`
(`experiments/minif2f_v4/src/bin/evaluator.rs:421`), the per-turn budget is
derived from the QState snapshot (`experiments/minif2f_v4/src/bin/evaluator.rs:3184`),
and a rejected high-risk role action sets `real5_legacy_action_permitted=false`
before the legacy tool dispatch (`experiments/minif2f_v4/src/bin/evaluator.rs:3454`
and `experiments/minif2f_v4/src/bin/evaluator.rs:3504`). The helper blocks only
Trader/MarketMaker high-risk market actions and Challenger high-risk challenge
actions below cap (`src/runtime/real6_conviction_budget.rs:88`), while ordinary
read/observe/abstain/solve/verify availability is preserved at the ConvictionBudget
level.

SG-6C.6 remains a CAS autopsy path. `write_significant_loss_autopsy_to_cas`
computes significant loss from realized plus unrealized PnL, requires the loss
to meet the threshold, and writes through the existing `write_autopsy_capsule`
contract with `CapsulePrivacyPolicy::AuditOnly`
(`src/runtime/real6_conviction_budget.rs:183` and
`src/runtime/real6_conviction_budget.rs:213`). The evaluator invokes that writer
after each turn when a ChainTape bundle is present
(`experiments/minif2f_v4/src/bin/evaluator.rs:6008` and
`experiments/minif2f_v4/src/bin/evaluator.rs:6028`). The gate restores the
CAS-resident capsule and checks the AuditOnly policy
(`tests/constitution_real6_conviction_budget.rs:183` and
`tests/constitution_real6_conviction_budget.rs:212`).

R2 challenge #2 is closed for closeout evidence by R3b's serial scoped command
record. The R3b event chain is strictly ordered from open through command_0009
(`handover/evidence/dev_self_hosting/dev_1778827530259_1394036/events.jsonl:1`);
the recorded commands cover fmt, REAL-6C test, G3 prompt test, Trust Root,
constitution gates, workspace tests, scoped status, and scoped hashes
(`handover/evidence/dev_self_hosting/dev_1778827530259_1394036/events.jsonl:2`,
`handover/evidence/dev_self_hosting/dev_1778827530259_1394036/events.jsonl:3`,
`handover/evidence/dev_self_hosting/dev_1778827530259_1394036/events.jsonl:4`,
`handover/evidence/dev_self_hosting/dev_1778827530259_1394036/events.jsonl:5`,
`handover/evidence/dev_self_hosting/dev_1778827530259_1394036/events.jsonl:6`,
`handover/evidence/dev_self_hosting/dev_1778827530259_1394036/events.jsonl:7`,
`handover/evidence/dev_self_hosting/dev_1778827530259_1394036/events.jsonl:8`,
and `handover/evidence/dev_self_hosting/dev_1778827530259_1394036/events.jsonl:9`).
The status artifact scopes the REAL-6C package to evaluator, Trust Root,
runtime export, prompt renderer, report/audits, module, and test
(`handover/evidence/dev_self_hosting/dev_1778827530259_1394036/artifacts/command_0007_stdout.txt:1`).
The hash artifact pins the same package files
(`handover/evidence/dev_self_hosting/dev_1778827530259_1394036/artifacts/command_0008_stdout.txt:1`),
with a later report-only hash witness after the report text was updated
(`handover/evidence/dev_self_hosting/dev_1778827530259_1394036/artifacts/command_0009_stdout.txt:1`).
The report explicitly supersedes and excludes `dev_1778827426590_1391207`
(`handover/evidence/real6_conviction_budget/REAL6C_CONVICTION_BUDGET_REPORT.md:162`).

I am explicitly not adopting `dev_1778827426590_1391207` as clean package
evidence. Its events show overlapping command writes for command_0008: the
sha256 command starts at `1778827512404`, while the status command starts at
`1778827512387`, and both write `artifacts/command_0008_*`
(`handover/evidence/dev_self_hosting/dev_1778827426590_1391207/events.jsonl:9`
and `handover/evidence/dev_self_hosting/dev_1778827426590_1391207/events.jsonl:10`).
R3b fixes that by recording the status and hash commands serially.

## Non-Blocking Observations

The branch-wide worktree is still dirty outside the REAL-6C package, including
restricted surfaces. I did not treat those as REAL-6C evidence because R3b's
scoped status/hash artifacts do not adopt them, and the report explicitly says
REAL-6C introduced no sequencer admission, TypedTx schema/discriminant,
canonical signing payload, wallet, kernel, or bus change
(`handover/evidence/real6_conviction_budget/REAL6C_CONVICTION_BUDGET_REPORT.md:29`).
This verdict is scoped to the R3b REAL-6C package; staging should remain
intentional so unrelated restricted-surface drift is not swept into the
REAL-6C closeout.

R3b's `DevTaskManifest.json` still lists only
`src/runtime/real6_conviction_budget.rs` in `allowed_paths`
(`handover/evidence/dev_self_hosting/dev_1778827530259_1394036/DevTaskManifest.json:12`),
while the actual closeout scope is represented by command_0007/0008
status/hash artifacts. I do not consider this blocking for R3 because the R2
challenge was about replacing the broad contaminated diff with clean scoped
closeout evidence, and R3b provides that scoped status/hash record. Future
self-hosting package runs should keep manifest `allowed_paths` aligned with
the command scope.

I found no new REAL-6C money-path `f64` / `f32` or PnL HashMap sidecar in the
REAL-6C module; the gate's source check pins the no-HashMap sidecar claim
(`tests/constitution_real6_conviction_budget.rs:42`).

## Verification Evidence Reviewed

- R3b command_0001: `cargo fmt --all -- --check`, exit 0
  (`handover/evidence/dev_self_hosting/dev_1778827530259_1394036/events.jsonl:2`).
- R3b command_0002: `cargo test --test constitution_real6_conviction_budget`,
  exit 0, 4 passed
  (`handover/evidence/dev_self_hosting/dev_1778827530259_1394036/artifacts/command_0002_stdout.txt:2`).
- R3b command_0003: `cargo test --test constitution_g3_your_position_prompt`,
  exit 0, 8 passed
  (`handover/evidence/dev_self_hosting/dev_1778827530259_1394036/artifacts/command_0003_stdout.txt:2`).
- R3b command_0004:
  `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo`,
  exit 0, 1 passed
  (`handover/evidence/dev_self_hosting/dev_1778827530259_1394036/events.jsonl:5`).
- R3b command_0005: `bash scripts/run_constitution_gates.sh`, exit 0,
  436 passed / 0 failed / 1 ignored
  (`handover/evidence/dev_self_hosting/dev_1778827530259_1394036/artifacts/command_0005_stdout.txt:135`).
- R3b command_0006:
  `cargo test --workspace --no-fail-fast -- --test-threads=1`, exit 0
  (`handover/evidence/dev_self_hosting/dev_1778827530259_1394036/events.jsonl:7`).

PROCEED
