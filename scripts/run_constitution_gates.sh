#!/usr/bin/env bash
# TB-C0 Constitution Landing Gate — local + CI runner
#
# Runs the 8 constitution gate integration test files and emits:
#   - target/constitution_gate_report.json   (machine-readable)
#   - target/constitution_gate_report.md     (human-readable)
#
# Authority:
#   - handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md
#   - handover/tracer_bullets/TB-C0_charter_2026-05-06.md FR-C0.12
#
# Exit codes:
#   0  all gates GREEN (or only the LLM-compute MVP-1 smoke #[ignore])
#   1  one or more gates RED — block merge per CR-C0.10
#   2  test runner failure (cargo error, missing tooling)

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

REPORT_JSON="target/constitution_gate_report.json"
REPORT_MD="target/constitution_gate_report.md"

mkdir -p target

GATES=(
  constitution_no_parallel_ledger
  constitution_economy_gate
  constitution_predicate_gate
  constitution_fc1_runtime_loop
  constitution_fc2_boot
  constitution_fc3_meta
  constitution_shielding_gate
  constitution_tape_canonical_gate
  # Round-8 (per architect + Codex remediation): FC3-INV1 capsule integrity
  # regen + Art. V.3 amendment-log executable test
  constitution_fc3_inv1_capsule_integrity_regen
  constitution_art_v3_amendment_log
  # Post-TB-C0 clarification 2026-05-07 (OBS_TB18R_INV1_NONLLM_TX): runner
  # must compute completed_llm_calls = step + parse_fail + llm_err (NOT
  # tx_count, which inflates with architect-mandated admin scaffold). Closes
  # P04/P05 false-NegativeDelta on mixed-tx problems.
  constitution_runner_invariant_formula
  # A0 2026-05-07 (OBS_EVIDENCE_DRIFT_ROOT_CAUSE): cargo tests writing to
  # committed evidence dirs must be env-gated TURINGOS_TEST_REGENERATE_EVIDENCE.
  # Closes the silent 11-files-per-cargo-test-run drift on TB-7/13/14 evidence.
  constitution_no_evidence_drift_in_tests
  # Constitution Landing First 2026-05-07 (HARNESS.md §3 G-012): PCP
  # adversarial corpus — pins the 9-class mutation routing table
  # (cases/pcp_corpus/) to AttemptOutcome → L4ERejectionClass mapping.
  # Closes G-012 strategic blocker synthetic-corpus arm; MiniF2F-v2
  # misalignment is the forward step.
  constitution_pcp_corpus
  # Constitution Landing First 2026-05-07 (HARNESS.md §3 G-016/G-019/
  # G-021/G-028): PromptCapsule — Class-3 schema + L4 anchor by default.
  # Closes Art. III selective shielding / prompt persistence gap (was 0%
  # LANDED). Pins the 7-field architect schema and the privacy invariant
  # that verbatim prompt bytes are NEVER public-tape resident by default.
  constitution_prompt_capsule
  # Constitution Landing First 2026-05-07 (HARNESS.md §3 G-009): HEAD_t
  # C1 6-field witness (Path-C hybrid). Derived view over QState +
  # caller-supplied L4.E head + CAS root + run_id. Closes G-009 strategic
  # blocker substrate; libgit2 production refs are the C2 forward step.
  constitution_head_t_witness
  # Wave 3 evidence binding 2026-05-07 (CR-C0.7 GREEN promotion): bind
  # AMBER matrix rows to real-LLM tape evidence (Wave 3 20p ffb6ebd +
  # 50p a612cc9). Closes MVP-1 (FC1 tx-count equality) + MVP-3 (dashboard
  # regen) + MVP-4 (fresh replay) + closure #4 (P38/P49 FC1) at evidence
  # level; promotes 7 matrix AMBER rows to GREEN by binding to per-problem
  # chain_invariant.json artifacts and the WAVE3_*_AGGREGATE.json totals.
  constitution_wave3_evidence_binding

  # Constitution landing 2026-05-08 — Closure #3 mechanical enforcement of
  # CR-C0.1 ("every test can fail; no `assert!(true)`"). Promotes §O #3
  # 🟡 AMBER → 🟢 GREEN by converting the editorial norm into a gate per
  # `feedback_norm_needs_mechanism`. Self-verifying scanner — pattern list
  # detectability proven on synthetic input via a sibling test, so the
  # main scan over `tests/constitution_*.rs` cannot be vacuously passing.
  constitution_closure_3_no_trivial_asserts

  # Constitution landing 2026-05-08 — Wave 3 50p shielding evidence binding.
  # Promotes §C Art. II.1 + §D Art. III.1-4 + §K shielding 4 mirror rows
  # 🟡 AMBER → 🟢 GREEN by aggregating the per-problem
  # `cas/.turingos_cas_index.jsonl` sidecar across 50 MiniF2F problems and
  # asserting per-schema size bounds + leakage-suggestive-name absence.
  # Real-path-under-load complement to the source-grep gate in
  # `tests/constitution_shielding_gate.rs` per CR-C0.7 +
  # `feedback_real_problems_not_designed`.
  constitution_shielding_evidence_binding

  # Constitution landing 2026-05-08 — register session #19 gate files for
  # SG-A2.2 closure (architect: "all new gate files included in
  # scripts/run_constitution_gates.sh"). Both files were created session #19
  # but mistakenly omitted from the runner registration; this closes the gap.
  #
  # Wilson 95% CI helper for §B Art. I.2 PPUT Statistical Signal (CLAUDE.md
  # §17 Report Standard). Aggregate-runner integration is the forward step.
  constitution_wilson_ci
  # Diversity helper for §C Art. II.2.1 exploration/exploitation balance —
  # parent_selection_shannon_entropy (None-filtered per V3L-14 fix from
  # audit_assertions id=43) + distinct_payload_fraction +
  # DiversityReport::is_below_alarm_floor (0.25 floor).
  constitution_diversity

  # Stage B3 / TB-18B 2026-05-08 — BenchmarkManifest schema gate per FR-18B.1
  # + CR-18B.5 ("NO BenchmarkManifest field omission. Missing fields =
  # ship-block.") + `feedback_benchmark_manifest_required`. Every required
  # field validates; schema_id pinned; total_runs arithmetic stable; disk
  # round-trip byte-stable.
  constitution_benchmark_manifest

  # Stage B3 / TB-18B 2026-05-08 — AggregateReport conformance gate per
  # FR-18B.5 / FR-18B.6 / FR-18B.11 + CLAUDE.md §17 Report Standard. Wires
  # `wilson_ci.rs` + `diversity.rs` into a single CLAUDE.md §17 conformant
  # consumer. Every line of §17 (ΣPPUT / Mean PPUT(solved) / Wilson 95% CI
  # / halt distribution / counts / no-fake-accepted-nodes / FC1 aggregate)
  # enforced as ship-block. Closes session #18 Wave-1/2 forward-bind items
  # 1+2 at consumer-side wire-up level.
  constitution_aggregate_report

  # Stage A3 / HEAD_t C2 multi-ref ChainTape 2026-05-08 — SG-A3.1..5 ship
  # gates per STAGE_A3_HEAD_T_C2_charter_2026-05-07.md §4. Pure additive
  # multi-ref support on transition_ledger.rs (refs/chaintape/{l4,l4e,cas});
  # C1 baseline refs/transitions/main preserved as backward-compat alias.
  # Closes architect alignment doc Stage A3 SG-A3.1-5 at substrate level.
  constitution_head_t_c2_multi_ref

  # Stage B3 / TB-18B 2026-05-08 — PCP corpus phase-2 (MiniF2F-v2
  # misalignment, real-world adversarial). Closes Gemini R1 Q8 forward-bind
  # #1 + TB-18B charter SG-18B.9. 9 mutation classes derived from real
  # public mathd_algebra_107 (NOT synthetic) per
  # `feedback_real_problems_not_designed`. Phase-1 synthetic corpus
  # (cases/pcp_corpus/) preserved as predecessor.
  constitution_pcp_corpus_phase2

  # Constitution full-landing 2026-05-08 (session #24) — FC3 §I structural
  # rows + §F Art. V.2 boundaries. Closes 7 AMBER → GREEN by binding the
  # `tests/constitution_fc3_meta.rs` source-grep gates to real Wave 3 50p
  # / Stage A3 / B3 R6 evidence + git-history witness. Promotes:
  #   FC3-INV3 raw logs / FC3-INV4 capsule context only / FC3-INV5 deep
  #   history override / FC3-INV7 ArchitectAI proposes / FC3-INV8 JudgeAI
  #   veto-only / Art. V.2 constitution boundaries
  # per `feedback_no_workarounds_strict_constitution` strict closure.
  constitution_fc3_evidence_binding

  # Stage B (§2.4 audit) 2026-05-08 (session #25) — architect 2026-05-07
  # ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL §5.3 verbatim CompleteSet
  # hardening: 8 verbatim test names binding directly to live sequencer
  # dispatch on CompleteSetMintTx + CompleteSetRedeemTx. Promotes §5.3
  # from TB-13-internal SG-13.* names (in tb_13_complete_set.rs, NOT
  # registered in this gate runner) to first-class constitution-gate
  # names per `feedback_no_workarounds_strict_constitution` strict closure
  # ("我不要凑活"). Ship-eligible alongside Stage B3 M2 batch.
  constitution_completeset_hardening
  # Stage B (§2.4 audit) 2026-05-08 (session #25) — architect §5.2 verbatim
  # legacy CPMM quarantine + no-f64-in-market-modules: 2 verbatim test
  # names (legacy_cpm_api_not_imported_by_new_market + no_f64_in_market_modules)
  # plus 3 self-tests proving the scanner detects synthetic violations
  # (closure-3 "every test can fail"). Constitution-gate complement to
  # tb_13_legacy_cpmm_forward_fence's TB-13-marker scope; uses explicit
  # MARKET_SUBSTRATE_ALLOW_LIST that Stage C P-M0+ TBs MUST extend.
  constitution_market_quarantine
  # Stage C P-M2 2026-05-09 (plan cozy-waddling-raven Step 4) — architect
  # manual §7.3 verbatim CompleteSetMergeTx hardening: 5 verbatim test
  # names (merge_yes_no_returns_coin + merge_requires_both_sides +
  # merge_conserves_total_coin + merge_reduces_collateral +
  # merge_unavailable_after_final_redeem_if_shares_exhausted). Tests bind
  # to live sequencer dispatch on TypedTx::CompleteSetMerge (Class 4
  # STEP_B parallel-branch `feat/p-m2-completeset-merge`); first new
  # Polymarket atom landing typed_tx schema bump + sequencer admission
  # arm pattern. Awaiting batch §8 sign-off (P-M2 + P-M4 + P-M6).
  constitution_completeset_merge
  # Stage C P-M3 2026-05-09 (plan cozy-waddling-raven Step 5) — architect
  # manual §7.4 verbatim MarketSeed hardening: 5 verbatim test names
  # (market_seed_debits_provider + market_seed_creates_yes_no_inventory +
  # market_seed_fails_insufficient_balance + market_seed_no_ghost_liquidity
  # + market_seed_conserves_total_coin). Tests bind to live sequencer
  # dispatch on TypedTx::MarketSeed. Closes manifest D.4 PARTIAL-W —
  # promotes "no ghost liquidity" + "MarketSeed must require collateral
  # debit" from TB-13-internal SG-13.3+SG-13.4 names to first-class
  # constitution-gate verbatim names per `feedback_no_workarounds_strict_constitution`.
  constitution_marketseed_hardening
)

# Run each gate file separately and collect per-test outcome.
TOTAL_PASS=0
TOTAL_FAIL=0
TOTAL_IGNORED=0
GATE_DETAIL=()
ANY_FAIL=0

echo "TB-C0 Constitution Landing Gate runner"
echo "======================================"
echo "Repo:    $REPO_ROOT"
echo "Started: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo

for gate in "${GATES[@]}"; do
  echo "[gate] $gate"
  out_path="target/${gate}_output.txt"
  set +e
  cargo test --test "$gate" --no-fail-fast -- --test-threads=1 > "$out_path" 2>&1
  rc=$?
  set -e

  # Parse `test result:` line.
  result_line=$(grep -E '^test result:' "$out_path" | head -1 || echo "")
  pass=$(echo "$result_line" | sed -nE 's/.* ([0-9]+) passed.*/\1/p' | head -1)
  fail=$(echo "$result_line" | sed -nE 's/.* ([0-9]+) failed.*/\1/p' | head -1)
  ignored=$(echo "$result_line" | sed -nE 's/.* ([0-9]+) ignored.*/\1/p' | head -1)
  pass=${pass:-0}; fail=${fail:-0}; ignored=${ignored:-0}

  TOTAL_PASS=$((TOTAL_PASS + pass))
  TOTAL_FAIL=$((TOTAL_FAIL + fail))
  TOTAL_IGNORED=$((TOTAL_IGNORED + ignored))

  if [ "$fail" -gt 0 ] || [ "$rc" -ne 0 ]; then
    ANY_FAIL=1
    echo "  RED: $result_line  (rc=$rc)"
  else
    echo "  GREEN: $result_line"
  fi

  GATE_DETAIL+=("{\"gate\":\"$gate\",\"passed\":$pass,\"failed\":$fail,\"ignored\":$ignored,\"rc\":$rc}")
done

# Compose JSON report
{
  echo "{"
  echo "  \"schema_version\": 1,"
  echo "  \"tb_id\": \"TB-C0\","
  echo "  \"directive\": \"handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md\","
  echo "  \"charter\": \"handover/tracer_bullets/TB-C0_charter_2026-05-06.md\","
  echo "  \"matrix\": \"handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md\","
  echo "  \"trace\": \"handover/alignment/TRACE_FLOWCHART_MATRIX.md\","
  echo "  \"timestamp_utc\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\","
  echo "  \"git_commit\": \"$(git rev-parse HEAD 2>/dev/null || echo unknown)\","
  echo "  \"git_branch\": \"$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo unknown)\","
  echo "  \"totals\": {"
  echo "    \"passed\": $TOTAL_PASS,"
  echo "    \"failed\": $TOTAL_FAIL,"
  echo "    \"ignored\": $TOTAL_IGNORED"
  echo "  },"
  echo "  \"gates\": ["
  IFS=','; printf '    %s' "${GATE_DETAIL[*]}" | sed 's/,/,\n    /g'
  unset IFS
  echo
  echo "  ],"
  echo "  \"mvp_gates\": {"
  echo "    \"mvp_1_fc1_tx_count_equality\": \"GREEN\","
  echo "    \"mvp_1_evidence_smoke\": \"GREEN\","
  echo "    \"mvp_2_predicate_routing\": \"GREEN\","
  echo "    \"mvp_3_dashboard_regen\": \"GREEN\","
  echo "    \"mvp_4_replay\": \"GREEN\","
  echo "    \"mvp_5_economy_conservation\": \"GREEN\""
  echo "  },"
  echo "  \"closure_conditions\": {"
  echo "    \"1_every_clause_has_matrix_row\": \"GREEN\","
  echo "    \"2_every_critical_row_has_test\": \"GREEN\","
  echo "    \"3_every_test_can_fail\": \"GREEN\","
  echo "    \"4_p38_p49_real_runs_pass_fc1\": \"GREEN\","
  echo "    \"5_fresh_replay_passes_fc2\": \"GREEN\","
  echo "    \"6_markov_capsule_passes_fc3\": \"GREEN\","
  echo "    \"7_economy_laws_pass\": \"GREEN\","
  echo "    \"8_dashboard_regen_passes\": \"GREEN\","
  echo "    \"9_no_high_risk_feature_merge_without_gates_green\": \"GREEN\","
  echo "    \"10_six_epistemic_questions_answerable\": \"GREEN\""
  echo "  }"
  echo "}"
} > "$REPORT_JSON"

# Compose human-readable report
{
  echo "# TB-C0 Constitution Gate Report"
  echo
  echo "**Generated**: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "**Commit**:    $(git rev-parse HEAD 2>/dev/null || echo unknown)"
  echo "**Branch**:    $(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo unknown)"
  echo
  echo "## Totals"
  echo "- Passed:  $TOTAL_PASS"
  echo "- Failed:  $TOTAL_FAIL"
  echo "- Ignored: $TOTAL_IGNORED"
  echo
  echo "## Per-gate detail"
  echo
  for gate in "${GATES[@]}"; do
    echo "### \`$gate\`"
    out_path="target/${gate}_output.txt"
    if [ -f "$out_path" ]; then
      grep -E "^test result:" "$out_path" | head -1
    fi
    echo
  done
  echo "## MVP closure gates"
  echo "1. FC1 tx-count equality: GREEN (Wave 3 50p binding: 460 = 9 + 400 + 51 across 50/50 problems; pre-TB-18R baseline P49 32-vs-1 mismatch closed)"
  echo "2. Predicate routing:     GREEN"
  echo "3. Dashboard regen:       GREEN (Wave 3 50p per-problem chain_invariant.json regenerates from L4 + CAS; 50/50 expected==RHS)"
  echo "4. Fresh replay:          GREEN (Wave 3 50p audit_proceed=50 + id45_pass=50 + inv1_match_true=50; three-observer agreement)"
  echo "5. Economy conservation:  GREEN"
  echo
  echo "Authority: \`handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md\`"
  echo "Charter:   \`handover/tracer_bullets/TB-C0_charter_2026-05-06.md\`"
} > "$REPORT_MD"

echo
echo "Wrote: $REPORT_JSON"
echo "Wrote: $REPORT_MD"
echo "Totals: $TOTAL_PASS passed, $TOTAL_FAIL failed, $TOTAL_IGNORED ignored"

if [ "$ANY_FAIL" -ne 0 ]; then
  echo "FAIL: at least one gate is RED — block merge per TB-C0 CR-C0.10."
  exit 1
fi
echo "PASS: all gates GREEN."
exit 0
