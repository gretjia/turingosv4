# CODEX REAL-7 Implementation Review

## Findings

1. [P1 production defect] `audit_dashboard --run-report` can mark REAL-7
   `minimum_tier_green` using safety/equivalence booleans that are not derived
   from ChainTape/CAS. The report builder derives many structural counts from
   L4/CAS (`task_count`, `Verify`, `Challenge`, `EventResolve`, router counts,
   and REAL-6B schema-CAS count), but then hard-codes
   `forced_live_investment: false`, `no_ghost_liquidity: true`,
   `clean_v3_comparison: true`, `role_classifier_output: true`,
   `price_observe_only: true`, `no_price_as_truth: true`, and
   `dashboard_regenerated: true` in
   `/home/zephryj/projects/turingosv4/src/bin/audit_dashboard.rs:2796`.
   Those booleans feed the gate conjunction in
   `/home/zephryj/projects/turingosv4/src/runtime/g7_structural_smoke.rs:60`.
   A future run with the same tx counts but a missing proof for one of these
   guardrails could still render GREEN. This violates the REAL-7 dashboard
   requirement that the §K view regenerate from ChainTape+CAS rather than
   dashboard-only assertions.

2. [P1 test-scaffold/evidence gap] The self-hosting harness artifact for the
   cited Class-4 REAL-7 run is under-scoped relative to the reviewed package.
   `DevTaskManifest.json` lists only
   `src/runtime/g7_structural_smoke.rs` as an allowed path and reports no
   restricted-surface hits at
   `/home/zephryj/projects/turingosv4/handover/evidence/dev_self_hosting/dev_1778836686627_1669621/DevTaskManifest.json:12`.
   The actual audit surface includes Trust Root-pinned dashboard/evaluator
   rewires: `genesis_payload.toml` rehashes
   `src/bin/audit_dashboard.rs` for REAL-7 at
   `/home/zephryj/projects/turingosv4/genesis_payload.toml:243` and
   `experiments/minif2f_v4/src/bin/evaluator.rs` at
   `/home/zephryj/projects/turingosv4/genesis_payload.toml:164`.
   The gate script registers the REAL-6B/REAL-7 tests at
   `/home/zephryj/projects/turingosv4/scripts/run_constitution_gates.sh:724`,
   but the cited harness manifest/diff does not cover those files. For a
   Class-4 package, the harness evidence must either include the actual touched
   path set or explicitly point to the earlier dev runs that own those diffs.

3. [P2 test-scaffold gap] The AttemptPredictionMarket evidence is honest as a
   scripted fixture, not live market emergence, but one test name/field shape can
   be overread. The module explicitly says REAL-6B is design plus scripted
   fixture only and adds no live scheduling or tx schema at
   `/home/zephryj/projects/turingosv4/src/runtime/real6_attempt_prediction.rs:3`.
   The fixture is persisted as a CAS `Generic` object with schema id
   `real6b.attempt_prediction_fixture.v1` at
   `/home/zephryj/projects/turingosv4/src/runtime/real6_attempt_prediction.rs:329`;
   the gate test verifies CAS round-trip at
   `/home/zephryj/projects/turingosv4/tests/constitution_real6_attempt_prediction_market.rs:171`.
   That is sufficient for the stated "scripted AttemptPredictionMarket" boundary,
   but it should not be described as spontaneous ChainTape market activity.

4. [P3 report hygiene gap] The persistence report says there were no autopsy
   capsules because there were "no event resolutions" at
   `/home/zephryj/projects/turingosv4/handover/evidence/g_phase_real_7_structural_smoke_r7_20260515T0927Z/PERSISTENCE_BINDING_REPORT.json:23`.
   The canonical aggregate shows `event_resolve: 3` at
   `/home/zephryj/projects/turingosv4/handover/evidence/g_phase_real_7_structural_smoke_r7_20260515T0927Z/aggregate_verdict.json:30`.
   This does not block REAL-7 because `loss_occurred=false`, but the explanatory
   text is stale and should not be reused as proof.

## Evidence Reviewed

The main structural counts are genuinely backed by ChainTape/CAS evidence. The
primary run reports 36 L4 entries, 21 L4.E entries, and 190 CAS objects in
`aggregate_verdict.json:4`; the tx-kind counts include 3 TaskOpen, 3
EscrowLock, 3 MarketSeed, 3 CpmmPool, 6 BuyWithCoinRouter, 3 Work, 3 Verify, 3
Challenge, 3 FinalizeReward, 3 TerminalSummary, and 3 EventResolve at
`aggregate_verdict.json:11`. The regenerated dashboard renders the same
structural shape in §C and §K at
`audit_dashboard_run_report.md:332` and
`audit_dashboard_run_report.md:474`.

The scripted AttemptPrediction fixture is present in CAS three times, one per
task, with schema id `real6b.attempt_prediction_fixture.v1` at
`cas/.turingos_cas_index.jsonl:7`, `:77`, and `:137`. The dashboard derives
`scripted_attempt_prediction_market_count` by scanning CAS metadata for that
schema id in `/home/zephryj/projects/turingosv4/src/bin/audit_dashboard.rs:2755`.

The run itself completed and audit_tape passed: `run_log.txt` records
`batch_exit=0`, `audit_exit=0`, `audit_verdict=PROCEED`,
`persistence_passing=true`, and `persistence_n_witnessed=5` at
`/home/zephryj/projects/turingosv4/handover/evidence/g_phase_real_7_structural_smoke_r7_20260515T0927Z/run_log.txt:6`.
I also regenerated §K locally from the cited runtime_repo/CAS with
`cargo run --bin audit_dashboard -- --repo ... --cas ... --run-report`; it
reproduces `minimum_tier_green: true` and the same structural counts, with
warnings only.

## Audit Questions

1. REAL-7 structural tx/count gates are satisfied by ChainTape/CAS evidence, not
   only dashboard text. However, the §K GREEN verdict still depends on
   hard-coded dashboard safety booleans, so the implementation does not yet fully
   satisfy the "dashboard derives from ChainTape+CAS" standard.

2. I did not find evidence that the REAL-7 harness diff itself changed
   sequencer admission, typed-tx schema/discriminants/signing payloads,
   kernel/bus/wallet, or CAS schema. The current Class-4 package evidence is
   nevertheless under-scoped because the cited dev harness does not cover the
   evaluator/dashboard/genesis/script/test surfaces under review.

3. The scripted fixture boundary is mostly honest: the code and report avoid
   claiming spontaneous emergence or forced live LLM investment. Keep that
   boundary explicit; do not treat the REAL-6B CAS fixture as live
   AttemptPredictionMarket emergence.

4. Trust Root and gate registrations are present: the evaluator/dashboard hashes
   are pinned in `genesis_payload.toml`, the Trust Root check was recorded green,
   and `scripts/run_constitution_gates.sh` includes the REAL-6B and REAL-7 gates.
   The dashboard's REAL-7 rehash comment overstates derivation for the hard-coded
   booleans and should be corrected with the code.

5. Ship should be blocked until the §K booleans are derived from evidence or
   removed from the minimum-tier conjunction, and until the Class-4 harness
   packet accounts for the real touched path set.

CHALLENGE
