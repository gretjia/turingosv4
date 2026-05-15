# CODEX REAL-10 REAL-8X Evidence Claim Review

Scope: independent clean-context audit of the CLEAN REAL-8X evidence and claim boundary only.

Risk / FC boundary: Class 4 evidence/claim audit touching FC1 market/action evidence, FC2 replay/pinning evidence, FC3 report/materialized-view claim boundary, Art. III shielding, and market/economic gates. I did not edit source files or evidence.

## Findings

### Production Defects / Evidence-Integrity Blockers

None found.

- The clean evidence satisfies the REAL-8X plan gates for the audited boundary. The plan requires unchanged A/B/C/D arms, hard pinning, only allowlisted arm toggles, 15 tasks per arm, all arms audit `PROCEED`, descriptive reporting unless CI supports stronger claims, no E2 claim without live non-scripted router/short tx, and no E4 claim without statistical support (`handover/directives/2026-05-15_REAL10_CONTROLLED_MARKET_EVIDENCE_EXPANSION_EXECUTION_PLAN.md:269`, `:278`, `:290`, `:315`, `:316`, `:317`, `:318`, `:319`, `:320`, `:321`). The clean benchmark report records the pinned hashes and `tasks per arm = 15` (`handover/evidence/real8x_market_ab_clean_20260515T141331Z/REAL8_MARKET_AB_BENCHMARK_REPORT.md:6`, `:10`, `:11`, `:12`, `:13`, `:14`), the A/B/C/D arm definitions are unchanged (`.../REAL8_MARKET_AB_BENCHMARK_REPORT.md:28`, `:32`, `:33`, `:34`, `:35`), the arm summary records all exits `0`, audits `PROCEED`, and task counts `15` (`handover/evidence/real8x_market_ab_clean_20260515T141331Z/real8_arm_summary.tsv:2`, `:3`, `:4`, `:5`), and the config audit records `disallowed_config_drift: []` (`handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_config_manifests/REAL8X_CONFIG_AUDIT.json:13`, `:19`).
- The decision report correctly avoids E2/E3/E4 overclaims under the emergence definitions. E2 requires live, non-scripted, agent-generated router/short-equivalent action and explicitly excludes scripted AttemptPrediction fixtures (`handover/alignment/EMERGENCE_METRICS_E1_E2_E3_E4.md:22`, `:27`, `:29`, `:30`, `:31`, `:32`, `:35`, `:39`). E3 requires persistent ChainTape/CAS-derived behavioral differentiation and excludes role labels or a single-task diversity index alone (`.../EMERGENCE_METRICS_E1_E2_E3_E4.md:45`, `:50`, `:52`, `:54`, `:57`, `:60`, `:61`, `:63`). E4 requires statistically meaningful performance separation and excludes small-n descriptive benchmarks or market-tx increase alone (`.../EMERGENCE_METRICS_E1_E2_E3_E4.md:66`, `:71`, `:75`, `:85`, `:88`, `:91`, `:92`). The decision report states E2 `NOT ACHIEVED`, E3 `NOT ESTABLISHED`, and E4 `NOT ESTABLISHED` (`handover/evidence/real8x_market_ab_clean_20260515T141331Z/REAL10_DECISION_GATE_REPORT.md:49`, `:55`, `:60`) and repeats the forbidden claims boundary against autonomous emergence, causal performance improvement, and role differentiation from `role_diversity_index=5` (`.../REAL10_DECISION_GATE_REPORT.md:94`, `:96`, `:99`, `:100`).
- Arm D's scripted AttemptPrediction is correctly classified as non-E2. The clean evidence labels D as `TaskOutcomeMarket + scripted AttemptPrediction fixture` (`handover/evidence/real8x_market_ab_clean_20260515T141331Z/REAL8_MARKET_AB_BENCHMARK_REPORT.md:35`; `handover/evidence/real8x_market_ab_clean_20260515T141331Z/REAL10_DECISION_GATE_REPORT.md:34`), the aggregate tx counts show `buy_with_coin_router=0` for D (`handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_D/aggregate_verdict.json:20`, `:22`, `:24`), the dashboard shows D's scripted fixture count is 15 while router buy counts are 0 (`handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_D/audit_dashboard_run_report.txt:637`, `:638`, `:639`, `:640`), and the decision report explicitly states the scripted D fixture is non-qualifying for E2 (`handover/evidence/real8x_market_ab_clean_20260515T141331Z/REAL10_DECISION_GATE_REPORT.md:49`, `:52`, `:53`).

### Evidence / Reporting / Test-Scaffold Gaps

Non-blocking.

- The report path differs from the plan's Atom 5 nominal output path. The plan says to create `handover/reports/REAL10_DECISION_GATE_REPORT.md` (`handover/directives/2026-05-15_REAL10_CONTROLLED_MARKET_EVIDENCE_EXPANSION_EXECUTION_PLAN.md:325`, `:327`, `:330`), while the audited report is evidence-local at `handover/evidence/real8x_market_ab_clean_20260515T141331Z/REAL10_DECISION_GATE_REPORT.md` per the audit task. This is not an evidence-integrity blocker because the user explicitly scoped this audit to the evidence-local report, but a future cleanup could mirror or route the report to the planned reports path.
- The decision report carries a preflight caveat that clean rerun stdout reported dirty-tree non-evidence changes and disk around 19G, below the 20G recommendation (`handover/evidence/real8x_market_ab_clean_20260515T141331Z/REAL10_DECISION_GATE_REPORT.md:22`, `:23`, `:24`). The report preserves the caveat instead of hiding it; I do not treat it as a blocker for these already chain-backed arm results, but it should remain a preflight item for larger REAL-8Y-style reruns.
- The dashboard Section K structural-smoke section can be confusing when read outside the REAL-8X claim boundary. For example, D records `scripted_attempt_prediction_market_count: 15`, router buy counts `0`, `no_forced_live_investment: false`, and `market_actions_chain_visible: true` (`handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_D/audit_dashboard_run_report.txt:637`, `:638`, `:639`, `:640`, `:648`, `:649`). Because REAL-8X's audited claim rests on the benchmark summary, aggregate tx counts, and decision report, this is not a blocker; however, Section K should be treated as structural-smoke scaffold context rather than as the REAL-8X emergence verdict.
- The emergence metrics doc duplicates the E3 "Derived from ChainTape/CAS, not prompt labels" sentence (`handover/alignment/EMERGENCE_METRICS_E1_E2_E3_E4.md:52`, `:53`). This is editorial only and does not change the E3 gate.

## Verification Commands

Ran from `/home/zephryj/projects/turingosv4`.

```bash
awk -F '\t' 'BEGIN{print "arm\texit\taudit\ttasks\tsolve_rate\tmarket_tx_count\tfailed_branch\tlatency_ms\twasted"} NR>1{print $1,$3,$4,$5,$6,$8,$9,$10,$11}' OFS='\t' handover/evidence/real8x_market_ab_clean_20260515T141331Z/real8_arm_summary.tsv
```

Observed: A `0/PROCEED/15`, solve `5/15`, market tx `0`; B `0/PROCEED/15`, solve `5/15`, market tx `10`; C `0/PROCEED/15`, solve `6/15`, market tx `42`; D `0/PROCEED/15`, solve `4/15`, market tx `38`.

```bash
jq -r '[input_filename, .verdict, .passed, .failed, .halted, .skipped, .tx_kind_counts.market_seed, .tx_kind_counts.cpmm_pool, .tx_kind_counts.buy_with_coin_router] | @tsv' handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_{A,B,C,D}/aggregate_verdict.json
```

Observed: all four aggregate verdicts `PROCEED` with `passed=41`, `failed=0`, `halted=0`, `skipped=11`; `buy_with_coin_router=0` for A/B/C/D; market seed + pool counts were A `0+0`, B `5+5`, C `21+21`, D `19+19`.

```bash
jq -e '.disallowed_config_drift == []' handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_config_manifests/REAL8X_CONFIG_AUDIT.json
```

Observed: `true`, exit 0.

```bash
sha256sum handover/evidence/real8x_market_ab_clean_20260515T141331Z/problems.pinned.txt handover/evidence/real8x_market_ab_clean_20260515T141331Z/model_assignment.pinned.env handover/evidence/real8x_market_ab_clean_20260515T141331Z/budgets.pinned.env handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_config_manifests/REAL8X_SHARED_CONFIG.env
```

Observed: `0c484c4e6cfc949f608ad5ee568f86edb56b32d387cf1f8a375e4f044f82f437`, `62d1e5862881ff8124ffa0159df78c62f91dde52cedbdd5fb966774440051526`, `70d88fcf2cf0e0b8826145b9176237be58820e9006faaa3fe9435f418859a42e`, and `52b6e553430c25bb2902db6fb208973535941deb57b3f7a14103b2ad90abd176`, matching the benchmark report's pinned input hashes.

```bash
wc -l handover/evidence/real8x_market_ab_clean_20260515T141331Z/problems.pinned.txt handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_A/PROBLEMS.txt handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_B/PROBLEMS.txt handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_C/PROBLEMS.txt handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_D/PROBLEMS.txt
```

Observed: all five problem manifests have 15 lines.

```bash
sha256sum handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_A/PROBLEMS.txt handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_B/PROBLEMS.txt handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_C/PROBLEMS.txt handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_D/PROBLEMS.txt
```

Observed: all four arm problem manifests share hash `0c484c4e6cfc949f608ad5ee568f86edb56b32d387cf1f8a375e4f044f82f437`.

```bash
sha256sum handover/evidence/real8x_market_ab_clean_20260515T141331Z/arm_config_manifests/arm_{A,B,C,D}_shared.env
```

Observed: all four shared arm configs share hash `85006ec87f29e6acac51f9eb6515f7634b2751504157a79f04d5085077d02bf8`.

```bash
rg -n "real8x_market_ab_clean_20260515T141331Z|real8x_market_ab_20260515T134453Z" handover/evidence/real8x_market_ab_clean_20260515T141331Z/REAL10_DECISION_GATE_REPORT.md
```

Observed: the decision report names the clean directory as its source and references the contaminated prior directory only as invalid/unused.

```bash
rg -n "E1|E2|E3|E4|NOT ACHIEVED|NOT ESTABLISHED|scripted|role_diversity_index|descriptive|causal|emergence|Forbidden Claims|spontaneous|performance" handover/evidence/real8x_market_ab_clean_20260515T141331Z/REAL10_DECISION_GATE_REPORT.md
```

Observed: E1 is marked satisfied for market-visible arms, while E2 is `NOT ACHIEVED`, E3 and E4 are `NOT ESTABLISHED`, D scripted AttemptPrediction is identified as non-qualifying for E2, and forbidden claims include autonomous emergence, causal performance improvement, and role differentiation from `role_diversity_index=5`.

VERDICT: PROCEED
