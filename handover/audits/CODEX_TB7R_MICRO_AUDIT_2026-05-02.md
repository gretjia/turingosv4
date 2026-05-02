# Codex TB-7R Micro-Audit (audit-point-1)

**Audit timestamp**: 2026-05-02T03:46:08Z  
**Auditor**: Codex  
**Range audited**: `HEAD~1..HEAD` = `696d10f..392a516` on `main`; hard-guardrail diff also checked as requested with `696d10f^..HEAD`.  
**Scope**: L4 / L4.E purity, ChainTape-mode fail-closed gate, genesis_report.json emission, on-chain TaskOpen+EscrowLock bootstrap pattern.

## Verdict line

**CHALLENGE** -- no VETO-level constitutional violation found, but Claim 7 does not hold exactly: new public items in `src/runtime/genesis_report.rs` use `TRACE_MATRIX FC2 (Boot / Genesis)` rather than the requested `TRACE_MATRIX FCx-Ny: <role>` form, and the public fields do not each carry TRACE_MATRIX doc-comments. This should be fixed or explicitly rebutted before ship audit #9.

## Per-claim findings

### Claim 1 -- L4 purity

**Finding: PASS, with artifact caveat.** I found no accepted L4 Work entry that fails the TB-7R predicates. The committed evidence does not include the raw CAS store directories, so I could not independently re-run byte-level `cas.get()` for every CID. The finding is therefore based on committed replay/dashboard artifacts plus the code paths that generated them.

- `tb_7_7_dag_capable_smoke_2026-05-01`: replay reports CAS payload and ProposalTelemetry retrieval green at `handover/evidence/tb_7_7_dag_capable_smoke_2026-05-01/replay_report.json:8` and `handover/evidence/tb_7_7_dag_capable_smoke_2026-05-01/replay_report.json:10`. The dashboard shows three L4 entries, exactly one L4 Work entry, and that Work has oracle marker at `handover/evidence/tb_7_7_dag_capable_smoke_2026-05-01/dashboard.txt:13`, `handover/evidence/tb_7_7_dag_capable_smoke_2026-05-01/dashboard.txt:55`, `handover/evidence/tb_7_7_dag_capable_smoke_2026-05-01/dashboard.txt:57`. The same dashboard reports `chain_oracle_verified: true` and renders the golden oracle WorkTx at `handover/evidence/tb_7_7_dag_capable_smoke_2026-05-01/dashboard.txt:35` and `handover/evidence/tb_7_7_dag_capable_smoke_2026-05-01/dashboard.txt:66`.
- `tb_7_chaintape_smoke_2026-05-01`: README says one L4 entry and that it is synthetic TaskOpen, while zero-stake WorkTx routes to L4.E at `handover/evidence/tb_7_chaintape_smoke_2026-05-01/README.md:10` and `handover/evidence/tb_7_chaintape_smoke_2026-05-01/README.md:19`; no accepted L4 Work exists there.
- `tb_7_real_smoke_5_problems_2026-05-01`: README states all five runs have zero L4 Work entries at `handover/evidence/tb_7_real_smoke_5_problems_2026-05-01/README.md:22`. Runs 1, 3, and 4 dashboards independently show L4 is TaskOpen only at `handover/evidence/tb_7_real_smoke_5_problems_2026-05-01/run_1_mathd_algebra_107/dashboard.txt:65`, `handover/evidence/tb_7_real_smoke_5_problems_2026-05-01/run_3_mathd_algebra_359/dashboard.txt:65`, and `handover/evidence/tb_7_real_smoke_5_problems_2026-05-01/run_4_aime_1997_p9/dashboard.txt:65`; runs 2 and 5 are covered by the README's all-five statement and the known CAS-race note at `handover/evidence/tb_7_real_smoke_5_problems_2026-05-01/README.md:24`.
- Source cross-check: ProposalTelemetry stores proposal bytes in CAS at `src/runtime/proposal_telemetry.rs:236` through `src/runtime/proposal_telemetry.rs:245`; ChainDerivedRunFacts only flips `chain_oracle_verified` after resolving a VerificationResult and seeing `verified == true` at `src/runtime/chain_derived_run_facts.rs:321`, `src/runtime/chain_derived_run_facts.rs:335`, and `src/runtime/chain_derived_run_facts.rs:336`; VerificationResult binds `proof_artifact_cid` at `src/runtime/verification_result.rs:47` and `src/runtime/verification_result.rs:62`.

### Claim 2 -- Fail-closed ChainTape gate and append bypass

**Finding: PASS.** The gate returns `Err` iff `TURINGOS_CHAINTAPE_PATH` is set and `condition == "oneshot"`: unset env returns `Ok` at `experiments/minif2f_v4/src/chaintape_mode_gate.rs:30`; the unsupported list contains only `oneshot` at `experiments/minif2f_v4/src/chaintape_mode_gate.rs:19`; the error branch is gated by that membership test at `experiments/minif2f_v4/src/chaintape_mode_gate.rs:33`. The evaluator calls the gate before dispatch at `experiments/minif2f_v4/src/bin/evaluator.rs:327`, before the `oneshot` branch at `experiments/minif2f_v4/src/bin/evaluator.rs:332`.

The four unit tests cover the truth table categories: ChainTape+oneshot fails at `experiments/minif2f_v4/src/chaintape_mode_gate.rs:50`, legacy+oneshot passes at `experiments/minif2f_v4/src/chaintape_mode_gate.rs:67`, ChainTape+n1/n5 passes at `experiments/minif2f_v4/src/chaintape_mode_gate.rs:81`, and ChainTape+unknown passes at `experiments/minif2f_v4/src/chaintape_mode_gate.rs:98`.

For bypass search, the live production `bus.append(` call is after the real WorkTx `bus.submit_typed_tx` path at `experiments/minif2f_v4/src/bin/evaluator.rs:1463`, `experiments/minif2f_v4/src/bin/evaluator.rs:1581`, and is explicitly `shadow_only` at `experiments/minif2f_v4/src/bin/evaluator.rs:1591` before the append at `experiments/minif2f_v4/src/bin/evaluator.rs:1599`. The `append_oracle_accepted` sites are also annotated `shadow_only` at `experiments/minif2f_v4/src/bin/evaluator.rs:1917`, `experiments/minif2f_v4/src/bin/evaluator.rs:2354`, and `experiments/minif2f_v4/src/bin/evaluator.rs:2408`. The regression scanner's scope and failure rule are documented at `tests/tb_7_legacy_append_regression.rs:16` through `tests/tb_7_legacy_append_regression.rs:19`, and its live evaluator test is at `tests/tb_7_legacy_append_regression.rs:93`.

### Claim 3 -- On-chain TaskOpen+EscrowLock

**Finding: PASS.** I found no exact forbidden `task_markets_t.insert` / `escrows_t.insert` preseed write in `src/` or `experiments/minif2f_v4/`. The preseed bootstrap builds TaskOpen and submits it through `bus.submit_typed_tx` at `experiments/minif2f_v4/src/bin/evaluator.rs:849` and `experiments/minif2f_v4/src/bin/evaluator.rs:857`; it builds EscrowLock and submits it through `bus.submit_typed_tx` at `experiments/minif2f_v4/src/bin/evaluator.rs:895` and `experiments/minif2f_v4/src/bin/evaluator.rs:902`.

The only runtime map insertions I found are the legitimate Sequencer transition-apply effects: the TaskOpen arm at `src/state/sequencer.rs:715` inserts the TaskMarketEntry at `src/state/sequencer.rs:734`, and the EscrowLock arm at `src/state/sequencer.rs:756` inserts the escrow at `src/state/sequencer.rs:780`.

### Claim 4 -- genesis_report.json emission

**Finding: PASS.** `GenesisReport` contains all nine requested fields: `constitution_hash` at `src/runtime/genesis_report.rs:32`, `runtime_repo` at `src/runtime/genesis_report.rs:36`, `cas_path` at `src/runtime/genesis_report.rs:40`, `system_pubkey_hash` at `src/runtime/genesis_report.rs:46`, `agent_pubkeys_path` at `src/runtime/genesis_report.rs:50`, `initial_balances` at `src/runtime/genesis_report.rs:55`, `task_id` at `src/runtime/genesis_report.rs:61`, `task_open_tx` at `src/runtime/genesis_report.rs:65`, and `escrow_lock_tx` at `src/runtime/genesis_report.rs:69`.

The evaluator populates and writes the report at `experiments/minif2f_v4/src/bin/evaluator.rs:991` through `experiments/minif2f_v4/src/bin/evaluator.rs:1008`. Optional preseed fields become `None` when preseed is disabled at `experiments/minif2f_v4/src/bin/evaluator.rs:980`, `experiments/minif2f_v4/src/bin/evaluator.rs:985`, and `experiments/minif2f_v4/src/bin/evaluator.rs:988`; the no-preseed test asserts JSON nulls for `task_id`, `task_open_tx`, and `escrow_lock_tx` at `src/runtime/genesis_report.rs:174` through `src/runtime/genesis_report.rs:195`.

### Claim 5 -- Hard-guardrail observance

**Finding: PASS.** The wider diff `696d10f^..HEAD` introduces guardrail text that names forbidden mechanisms only to prohibit or defer them; I found no code implementation of NodeMarket, role-taxonomy mutation, whale tracking, FinalizeRewardTx, SlashTx, MetaTape, predicate-registry mutation, constitution.md edits, RootBox touches, new TypedTx variants, or per-tactic decomposition logic. The TB-7R charter's hard guardrail list is explicit at `handover/tracer_bullets/TB-7R_charter_2026-05-01.md:264` through `handover/tracer_bullets/TB-7R_charter_2026-05-01.md:279`. The D7 resolution explicitly defers per-tactic decomposition at `handover/ai-direct/HANDOVER_TB_7_7_D7_RESOLVED_2026-05-01.md:16` and `handover/ai-direct/HANDOVER_TB_7_7_D7_RESOLVED_2026-05-01.md:19`.

I also checked that the protected implementation surfaces were not changed in this range: no diff to `constitution.md`, `src/state/typed_tx.rs`, predicate registry files, RootBox-named files, `src/bus.rs`, `src/kernel.rs`, `src/state/sequencer.rs`, or `src/sdk/tools/wallet.rs`.

### Claim 6 -- Three-node taxonomy placement

**Finding: PASS.** Occurrences are confined to TB-7R documentation/evidence/decision surfaces. The decision record defines the terms at `handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md:20`, `handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md:26`, and `handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md:31`, and explicitly forbids constitution, RootBox, and TRACE_MATRIX row-text placement at `handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md:82` through `handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md:88`. The authorization verdict allows these terms in TB-7R/docs/dashboard/decision-record surfaces at `handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md:97`.

Search found no occurrences in `constitution.md` or `handover/alignment/TRACE_MATRIX_v*`.

### Claim 7 -- TRACE_MATRIX backlinks on new pub items

**Finding: CHALLENGE.** The gate function satisfies the requested `FCx-Ny` shape: `experiments/minif2f_v4/src/chaintape_mode_gate.rs:21` documents `pub fn chaintape_supports_condition` at `experiments/minif2f_v4/src/chaintape_mode_gate.rs:29` with `TRACE_MATRIX FC1-N6`.

`src/runtime/genesis_report.rs` does not satisfy the exact claim. The public struct has a TRACE_MATRIX comment, but it is `TRACE_MATRIX FC2 (Boot / Genesis)` rather than `FCx-Ny` at `src/runtime/genesis_report.rs:22`; the public methods repeat the same non-`N` form at `src/runtime/genesis_report.rs:73`, `src/runtime/genesis_report.rs:88`, and `src/runtime/genesis_report.rs:98`. Public fields such as `constitution_hash`, `runtime_repo`, and `escrow_lock_tx` have ordinary doc-comments but no TRACE_MATRIX backlink at `src/runtime/genesis_report.rs:32`, `src/runtime/genesis_report.rs:36`, and `src/runtime/genesis_report.rs:69`. This conflicts with the claim as written. Per authorization C9, exact FC rows should be used if they exist, otherwise an orphan/TODO justification should be used without fabricating FC numbers (`handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md:66`).

## Aggregate concerns

1. **Trace backlink format gap**: Claim 7 fails for `src/runtime/genesis_report.rs`. This is not a VETO because I found no constitution or RootBox mutation, but it should be fixed or explicitly rebutted before ship audit #9.
2. **Claim 1 raw-CAS reproducibility gap**: The committed evidence contains replay/dashboard outputs but not the CAS stores needed to re-run direct CID resolution for the D7 accepted WorkTx. The artifacts are internally consistent, but ship audit #9 should carry raw CAS or a per-tx verifier transcript with the ProposalTelemetry CID, VerificationResult CID, and proof_artifact CID.
3. **Historical README carry-forward**: Checkpoint 2 records that the `tb_7_chaintape_smoke_2026-05-01/README.md` grandfathering annotation reverted under an editor hook at `handover/CHECKPOINT_TB7R_2_2026-05-02.md:51`. That is outside this micro-audit's seven claims, but it remains a cleanup item before final TB-7R packaging.

## Recommended actions before #9 ship audit

1. Update `src/runtime/genesis_report.rs` public API doc-comments to use an exact `TRACE_MATRIX FCx-Ny: <role>` row if one exists, or add a documented orphan/TODO justification consistent with verdict C9. Include the public struct and public methods at minimum; decide explicitly whether public fields are "pub items" for this local rule.
2. Re-run and archive targeted tests after the trace-comment fix: `cargo test -p turingosv4 runtime::genesis_report`, `cargo test -p minif2f_v4 chaintape_mode_gate`, and `cargo test -p turingosv4 --test tb_7_legacy_append_regression`.
3. For #9, include raw CAS evidence or a generated per-tx purity transcript so the accepted L4 Work predicate can be rerun directly rather than inferred from replay/dashboard artifacts.
4. Resolve or document the reverted grandfathering note for `handover/evidence/tb_7_chaintape_smoke_2026-05-01/README.md`.

## Verification commands run

```text
cargo test -p minif2f_v4 chaintape_mode_gate -- --nocapture
  result: PASS (4 tests)

cargo test -p turingosv4 runtime::genesis_report -- --nocapture
  result: PASS (4 tests)

cargo test -p turingosv4 --test tb_7_legacy_append_regression -- --nocapture
  result: PASS (3 tests)
```

---

## Remediation log (post-audit, Claude 2026-05-02)

**Status**: Claim 7 CHALLENGE addressed via TRACE_MATRIX § 3 orphan
registration. Aggregate concern #2 (raw-CAS reproducibility) deferred
to ship audit #9 per recommendation #3. Aggregate concern #3 (reverted
README annotation) deferred — non-blocking, listed in `handover/CHECKPOINT_TB7R_2_2026-05-02.md` Open Observations §1.

### Claim 7 fix detail

The two fabricated / mismapped backlinks were:

1. `experiments/minif2f_v4/src/chaintape_mode_gate.rs` line 21 used
   `TRACE_MATRIX FC1-N6` — but the canonical FC1-N6 is
   `input = ⟨q_i, s_i⟩` UniverseSnapshot (`handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:28`),
   unrelated to the gate's pre-routing predicate role.
2. `src/runtime/genesis_report.rs` used `TRACE_MATRIX FC2 (Boot / Genesis)`
   on the module + struct + 3 fns — but canonical FC2 is
   Append/Submit (`handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:197-298`),
   not Boot/Genesis. The architect verdict §6.1's "Flowchart 2: Boot / Genesis"
   shorthand does not align with the canonical TRACE_MATRIX FC numbering.

Per architect verdict §C9 ("NEVER fabricate FC numbers; if no precise
row, use FC-trace + register an orphan justification"), both are now
re-labeled as `TRACE_MATRIX § 3 orphan` with explicit Constitutional
Justification, and registered in `handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md`.

The `pub mod` declarations in `experiments/minif2f_v4/src/lib.rs` and
`src/runtime/mod.rs` carry the same orphan label. `src/runtime/mod.rs`
hash refreshed in `genesis_payload.toml` (R-014 trust root).

### Verification after fix

```text
command         = cargo test --workspace
workspace_count = 706
failed          = 0
ignored         = 150
```

All four `chaintape_mode_gate::tests` and four `runtime::genesis_report::tests`
remain GREEN. No behavioral code changed — only doc-comment text + one
trust-root hash entry.

### Audit verdict for ship audit #9

Codex CHALLENGE Claim 7 → **REMEDIATED** via orphan registration.
Aggregate concern #2 (raw-CAS purity transcript) carries forward to
ship-audit acceptance criteria. Aggregate concern #3 (atom-6 README
annotation hook) carries forward as non-blocking cleanup item.
