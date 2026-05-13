VERDICT: PROCEED

Findings:

- No ship-blocking findings. R1's production dashboard gap is closed: `render_tb_n3_run_report`
  now invokes `compute_risk_cap_impact_report_from_paths(repo, cas_path)` and appends
  `report.render_section_g_2()` before the §H price banner, so `--run-report` no
  longer omits the architect §7.1 RiskCapImpactReport surface
  (`src/bin/audit_dashboard.rs:2280`).
- The rendered §G.2 section includes the required attribution columns:
  `agent_id`, `balance_before_micro`, `risk_cap_micro`, `tx_kind`, `task_id`,
  `another_agent_continued`, and `solve_outcome`
  (`src/runtime/risk_cap_impact_report.rs:75`, `src/runtime/risk_cap_impact_report.rs:87`).
- The dashboard section is not stub/dashboard-only text. The path walker reads
  L4.E rejection records, loads rejected typed-tx payloads from CAS, opens the L4
  ledger, and replays QState at the rejection `parent_state_root` via
  `replay_full_transition` before computing balance and risk cap
  (`src/runtime/risk_cap_impact_report.rs:156`,
  `src/runtime/risk_cap_impact_report.rs:166`,
  `src/runtime/risk_cap_impact_report.rs:183`,
  `src/runtime/risk_cap_impact_report.rs:223`,
  `src/runtime/risk_cap_impact_report.rs:329`,
  `src/runtime/risk_cap_impact_report.rs:357`).
- The remediation adds a gate that would fail if the dashboard no longer names,
  derives, and renders the RiskCapImpactReport path
  (`tests/constitution_g3_bankruptcy_risk_cap.rs:463`).
- Trust Root remediation is consistent with the current files: `src/runtime/mod.rs`
  hash `1d52b65301c2b90d5f62c816831548a5c8840f608ef3a6b09efebe2004b2b41b`
  and `src/bin/audit_dashboard.rs` hash
  `46da2914372a48680ca12670e44b30eb2668bd618560f9860f1a0409f7fceaa9`
  match `genesis_payload.toml` (`genesis_payload.toml:219`,
  `genesis_payload.toml:243`).

Non-blocking observation:

- `tx_kind_label_for_risk_cap_rejection(u16)` still documents itself as mirroring
  `TxKind` ids but maps stale values (`1/2/3/15`) while the current enum is
  `Work=0`, `Verify=1`, `Challenge=2`, `BuyWithCoinRouter=17`
  (`src/runtime/risk_cap_impact_report.rs:412`,
  `src/bottom_white/ledger/transition_ledger.rs:53`). This is not ship-blocking
  for the R1 closure because the dashboard path uses the correct enum match in
  `risk_cap_tx_kind_label(TxKind)` and `rg` shows the stale helper is only used
  by tests (`src/runtime/risk_cap_impact_report.rs:382`). Forward cleanup should
  correct the helper/tests to avoid future audit-view drift.

Open Questions / Assumptions:

- I treated `src/runtime/mod.rs`'s `pub mod dev_harness;` as the pre-existing
  Unified Harness sidecar described in the brief. It is included in the current
  Trust Root hash and does not replace canonical ChainTape/CAS truth.
- I did not rerun the full workspace because the self-hosting evidence already
  records the full command with exit 0; I independently reviewed the artifact
  hashes and reran focused current-tree checks.

Verification Evidence Reviewed:

- R1 verdict reviewed:
  `handover/audits/CODEX_G2_TB_G_G3_2_VERDICT.md` -> CHALLENGE for missing
  dashboard RiskCapImpactReport wire/render.
- Self-hosting evidence reviewed:
  `handover/evidence/dev_self_hosting/dev_1778668340170_3888334/`.
  `artifacts/diff.patch` SHA256 matched
  `12427cb69703879edc733c3412004e09cfb426e0329e88e8d675d5a3f023139a`.
- Recorded command evidence reviewed:
  `command_0002` `cargo test --test constitution_g3_bankruptcy_risk_cap`
  exit 0, `33 passed`; `command_0003`
  `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo`
  exit 0; `command_0004` `bash scripts/run_constitution_gates.sh`
  exit 0, `Totals: 435 passed, 0 failed, 1 ignored`, `PASS: all gates GREEN.`;
  `command_0005` `cargo test --workspace --no-fail-fast -- --test-threads=1`
  exit 0.
- Commands I ran independently:
  `sha256sum handover/evidence/dev_self_hosting/dev_1778668340170_3888334/artifacts/diff.patch src/runtime/mod.rs src/bin/audit_dashboard.rs`;
  `rg -n "f64|f32" src/runtime/risk_cap_impact_report.rs src/bin/audit_dashboard.rs tests/constitution_g3_bankruptcy_risk_cap.rs src/state/sequencer.rs src/state/typed_tx.rs src/runtime/autopsy_capsule.rs src/runtime/agent_pnl.rs`;
  `cargo test --test constitution_g3_bankruptcy_risk_cap arch_71_audit_dashboard_wires_risk_cap_impact_report -- --exact`
  -> exit 0, `1 passed`; `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo`
  -> exit 0, `1 passed`.

Final Verdict: PROCEED
