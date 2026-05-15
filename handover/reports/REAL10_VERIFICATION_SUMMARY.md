# REAL-10 Verification Summary

Date: 2026-05-15

Scope: REAL-10 Controlled Market Evidence Expansion.

Clean evidence directory:

```text
handover/evidence/real8x_market_ab_clean_20260515T141331Z/
```

Invalid evidence directory, preserved only as contamination/remediation record:

```text
handover/evidence/real8x_market_ab_20260515T134453Z/
```

## Targeted Checks

```bash
bash -n scripts/run_real8_market_ab_benchmark.sh
```

Result: exit 0.

```bash
cargo test --test constitution_real8_market_ab_benchmark
```

Result: 9 passed, 0 failed.

```bash
cargo test --test constitution_real10_trace_cleanup
```

Result: 6 passed, 0 failed.

```bash
cargo test --test constitution_real10_emergence_metrics
```

Final result after editorial repair: 4 passed, 0 failed.

```bash
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo
```

Result: 1 passed, 0 failed.

```bash
git diff --check
```

Result: exit 0.

## Constitution Gates

```bash
bash scripts/run_constitution_gates.sh
```

Latest result:

```text
Totals: 461 passed, 0 failed, 1 ignored
PASS: all gates GREEN.
```

The runner wrote:

```text
target/constitution_gate_report.json
target/constitution_gate_report.md
```

## Workspace Tests

```bash
cargo test --workspace --no-fail-fast -- --test-threads=1
```

Latest result: exit 0. The final output completed through:

```text
Doc-tests gix_capability_spike: ok
Doc-tests minif2f_v4: ok
Doc-tests turingosv4: ok
```

## Clean REAL-8X Evidence Facts

All arms used the same pinned problem set, model assignment, budgets, timeout,
max_tx, and shared config except allowlisted arm toggles.

`arm_config_manifests/REAL8X_CONFIG_AUDIT.json` reports:

```text
disallowed_config_drift=[]
```

Arm summary:

| Arm | Condition | exit/audit/tasks | market_tx_count | buy_with_coin_router | solve_rate |
| --- | --- | ---: | ---: | ---: | ---: |
| A | market disabled | 0 / PROCEED / 15 | 0 | 0 | 5/15 |
| B | market visible, no TaskOutcomeMarket | 0 / PROCEED / 15 | 10 | 0 | 5/15 |
| C | TaskOutcomeMarket enabled | 0 / PROCEED / 15 | 42 | 0 | 6/15 |
| D | TaskOutcomeMarket + scripted AttemptPrediction fixture | 0 / PROCEED / 15 | 38 | 0 | 4/15 |

Claim boundary:

```text
E1 satisfied for market-visible arms B/C/D.
E2 not achieved.
E3 not established.
E4 not established.
```

No REAL-10 evidence authorizes live REAL-6B, spontaneous emergence,
causal performance improvement, model ranking, price-as-truth, forced trade,
ghost liquidity, off-tape WAL truth, private CoT recording, or raw-log broadcast.
