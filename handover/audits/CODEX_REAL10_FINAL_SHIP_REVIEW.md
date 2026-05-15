# CODEX REAL-10 Final Ship Review

Reviewer: final independent clean-context Codex ship reviewer
Date: 2026-05-15
Scope: REAL-10 Controlled Market Evidence Expansion, treated as a Class 4 ship package.
FC boundary: FC1 runtime market/action evidence and stale-parent admission behavior; FC2 pinned benchmark inputs/replay; FC3 report/materialized views/claim boundary; Art. III shielding; market/economic gates.

## Findings

No blocking findings.

- REAL-10 implements the approved narrow plan without hidden live REAL-6B enablement or E2/E3/E4 overclaim. The ratification explicitly limits REAL-5S -> REAL-9 to a lawful market-pressure scaffold and rejects spontaneous emergence, causal improvement, price-as-truth, and live REAL-6B approval (`handover/directives/2026-05-15_REAL5S_REAL9_NARROW_RATIFICATION.md:10`, `:24`, `:26`, `:27`, `:30`, `:33`). The runner fail-closes on `TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION=1` and keeps Arm D as a scripted fixture only (`scripts/run_real8_market_ab_benchmark.sh:88`, `:89`, `:451`, `:454`). The E1/E2/E3/E4 definitions reject scripted E2, role-label-only E3, and small-n/market-count-only E4 (`handover/alignment/EMERGENCE_METRICS_E1_E2_E3_E4.md:22`, `:27`, `:35`, `:45`, `:56`, `:65`, `:87`, `:96`).

- TRACE/R-022 cleanup and stale-parent coverage are sufficient for ship. The cleanup report records the R-022 skip as a one-time exception, not a waiver (`handover/alignment/TRACE_MATRIX_BACKLINK_CLEANUP_REAL5S_REAL9.md:7`, `:16`, `:18`), and the regression gate parses the skip log and asserts no missing TRACE_MATRIX §J pairs (`tests/constitution_real10_trace_cleanup.rs:82`, `:89`, `:90`). The stale-parent behavioral test drives TaskOpen/EscrowLock/WorkTx, mutates state through `CompleteSetMint`, proves stale `VerifyTx` rejects with `StaleParent`, then proves refreshed `q_snapshot().state_root_t` accepts (`tests/constitution_real8_market_ab_benchmark.rs:347`, `:367`, `:378`, `:381`, `:386`, `:391`).

- Trust Root update matches the pinned TRACE_MATRIX change. The current `TRACE_MATRIX_v3_2026-04-27.md` hash is `fe02b8d07185d4a74a7a129097ba819af020c0029038e5355d67126350971736`, and `genesis_payload.toml` pins that exact value (`genesis_payload.toml:311`). The focused Trust Root test passed in this review.

- REAL-8X final claims use the clean evidence directory and preserve the contamination boundary. The top-level reports identify `handover/evidence/real8x_market_ab_clean_20260515T141331Z/` as the clean source and mark `handover/evidence/real8x_market_ab_20260515T134453Z/` invalid for conclusions (`handover/reports/REAL10_VERIFICATION_SUMMARY.md:7`, `:13`; `handover/reports/REAL10_DECISION_GATE_REPORT.md:3`, `:8`). The contaminated directory contains a preservation note explaining why it must not be cited as final evidence (`handover/evidence/real8x_market_ab_20260515T134453Z/REAL10_EVIDENCE_CONTAMINATION_NOTE.md:5`, `:18`, `:26`, `:35`).

- Clean REAL-8X evidence supports only the allowed descriptive conclusion. The benchmark report records pinned hashes, 15 tasks per arm, all arm exits `0`, all audits `PROCEED`, and market activity A=0, B=10, C=42, D=38 with `buy_with_coin_router=0` (`handover/evidence/real8x_market_ab_clean_20260515T141331Z/REAL8_MARKET_AB_BENCHMARK_REPORT.md:10`, `:14`, `:41`, `:42`, `:43`, `:44`). The config audit has `disallowed_config_drift=[]` (`handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_config_manifests/REAL8X_CONFIG_AUDIT.json:19`). The evidence-local decision report marks E2 not achieved, E3 not established, E4 not established, and repeats the forbidden claims (`handover/evidence/real8x_market_ab_clean_20260515T141331Z/REAL10_DECISION_GATE_REPORT.md:49`, `:55`, `:60`, `:94`).

## Open Questions / Assumptions

- I did not rerun full workspace tests per user scope guard. I relied on `handover/reports/REAL10_VERIFICATION_SUMMARY.md:57` and `:77` plus the existing `target/constitution_gate_report.json` timestamped `2026-05-15T14:47:16Z` with totals `461 passed, 0 failed, 1 ignored`.
- The current worktree contains many untracked REAL-10 docs/evidence files. Ship staging must include the intended untracked artifacts deliberately; this is staging hygiene, not a code or evidence-integrity blocker.
- The dev evidence sidecar still contains the recorded contaminated benchmark command. That is acceptable only because the final claim path uses the clean rerun and the contaminated directory is explicitly invalidated.

## Verification Commands Run

```text
git diff --check
observed: exit 0, no output

git diff --stat
observed: tracked diff is 4 files, 638 insertions, 14 deletions

bash -n scripts/run_real8_market_ab_benchmark.sh
observed: exit 0

cargo test --test constitution_real8_market_ab_benchmark
observed: 9 passed, 0 failed

cargo test --test constitution_real10_trace_cleanup
observed: 6 passed, 0 failed

cargo test --test constitution_real10_emergence_metrics
observed: 4 passed, 0 failed

cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo
observed: 1 passed, 0 failed

sha256sum handover/alignment/TRACE_MATRIX_v3_2026-04-27.md
observed: fe02b8d07185d4a74a7a129097ba819af020c0029038e5355d67126350971736

jq '.totals' target/constitution_gate_report.json
observed: passed=461, failed=0, ignored=1
```

VERDICT: PROCEED
