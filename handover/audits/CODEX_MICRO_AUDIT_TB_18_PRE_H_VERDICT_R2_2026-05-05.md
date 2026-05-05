---
audit_run_id: codex-tb18-g0-r2-2026-05-05
verdict_date: 2026-05-05
auditor: Codex external audit
workspace_HEAD: 82776d5752d10cb89fb5fdf9723dbffa8d3496b5
round1_verdict: handover/audits/CODEX_MICRO_AUDIT_TB_18_PRE_H_VERDICT_2026-05-05.md
---

OVERALL: PASS

Scope note: the requested file `handover/audits/_R2_AUDIT_TASKS.md` is absent in this workspace (`find ... -name _R2_AUDIT_TASKS.md` returned no paths). Per the user's autonomous-execution instruction, I did not refuse; I audited the round-1 CHALLENGE remediation claims from `CODEX_MICRO_AUDIT_TB_18_PRE_H_VERDICT_2026-05-05.md` plus the on-disk G0 CHALLENGE-resolved claims in the TB-18 evidence READMEs.

Section A — comprehensive_arena capsule reason fix: PASS

`experiments/minif2f_v4/src/bin/comprehensive_arena.rs:259-285` now passes a caller-supplied `reason: ExhaustionReason` into `write_evidence_capsule`; it is no longer hardcoded to `MaxTxExhausted`. Literal reads show task_C/D/E still pass `ExhaustionReason::MaxTxExhausted` (`:534`, `:609`, `:684`), while task_F passes `ExhaustionReason::DegradedLLM` (`:744`) and emits `RunOutcome::DegradedLLM` in the TerminalSummary (`:749-758`). This directly closes round-1 Q6's task_F `DegradedLLM`/`MaxTxExhausted` mismatch.

Section B — stricter audit_tape assertion: PASS

`src/runtime/audit_assertions.rs:1679-1742` now resolves every TerminalSummary `evidence_capsule_cid`, decodes the `EvidenceCapsule`, computes `cap.terminal_reason.to_run_outcome()`, and HALTs if it differs from `TerminalSummary.run_outcome`. Regression unit tests at `src/runtime/audit_assertions.rs:2888-2984` include the exact G0 mismatch pair (`RunOutcome::DegradedLLM` + `ExhaustionReason::MaxTxExhausted`) and the corrected pair.

Section C — old r1 chain now fails for the right reason: PASS

Direct replay of historical `tb_18_b_phase4_2026-05-05/r1` with the current `audit_tape` returned `verdict=BLOCK`, `passed=34`, `halted=1`; assertion 27 HALTed with: `TerminalSummary.run_outcome (DegradedLLM) != EvidenceCapsule.terminal_reason.to_run_outcome() (MaxTxExhausted) at L4 index 30`. The r1 CAS object still stringifies as `tb18-task-f`, `task-task_f_degraded`, `reason=MaxTxExhausted`, so r1 was preserved and not retroactively rewritten.

Section D — regenerated r2 evidence consistency: PASS

`handover/evidence/tb_18_b_phase4_2026-05-05/r2/evidence/SHARED_CHAIN_RUNS_REPORT.json` reports `chain_seed_id=tb18-arena-r2-g0fix`, `task_count=6`, and task_F `outcome=DegradedLLM`. `tx_kind_distribution.json` reports `distinct_tx_kinds=13` / `target_distinct_tx_kinds=13`. Direct CAS inspection of the r2 task_F EvidenceCapsule object (`backend_oid_hex=e6270c124514b6199b2835e8ad12a77988e6a779`) stringifies as `tb18-task-f`, `task-task_f_degraded`, `reason=DegradedLLM`; `audit_dashboard --json` over staged r2 bytes reports `tb18-task-f-run`, `run_outcome=DegradedLLM`, `evidence_capsule_cid_hex=8831108ac098891204298b0925f7d31af9911a51ba4f706476c4b25fbb79e64f`.

Section E — Atom F r2 gates: PASS

`handover/evidence/tb_18_single_chain_13_of_13/r2/verdict.json` is `PROCEED` with `passed=35`, `failed=0`, `halted=0`, `skipped=8`; assertion 27 is `Pass`. `cmp -s verdict.json verdict_replay.json` returned `0`. `tamper_report.json` has `detected_count=3`, `expected=3`, `all_detected=true`. Re-running `handover/tests/scripts/run_tb_18_atom_f_2026-05-05.sh --src-dir handover/evidence/tb_18_b_phase4_2026-05-05/r2 --out-dir <tmp>` exited `0` and reproduced PROCEED + byte-identical replay + tamper 3/3 + β-A FEASIBLE.

Section F — β-A / sidecar discipline: PASS

Literal absence checks returned absent for `LATEST_MARKOV_CAPSULE.txt` at project root, r2 `runtime_repo`, and r2 `cas`. The Atom F script invokes `audit_tape` without `--markov-pointer` or `--prior-chain-runtime-repo`; the only literal hit is a comment stating those flags are absent. `r2/beta_a_feasibility_check.json` records `markov_pointer_flag="absent"`, `prior_chain_runtime_repo_flag="absent"`, `terminal_summary_count=3`, and `verdict="FEASIBLE"`.

Section G — wire integrity / missed items: PASS

Sensitive canonical surfaces remain untouched by the remediation: `git diff --name-only HEAD -- src/state/sequencer.rs src/state/typed_tx.rs src/kernel.rs src/bus.rs src/sdk/tools/wallet.rs` returned no paths, and the same path set has no committed diff from `0c3a5e1..HEAD`. Current working diffs relevant to this remediation are limited to `experiments/minif2f_v4/src/bin/comprehensive_arena.rs`, `src/runtime/audit_assertions.rs`, the Atom F runner script, and TB-18 evidence READMEs. `cargo test --workspace --release` exited `0`; `cargo test --workspace --release -- --list` counted `1116` total tests and `--ignored --list` counted `150` ignored, i.e. `966/0/150` non-ignored/failed/ignored after the three new assertion-27 tests.

Advisory non-blockers:
- `handover/evidence/tb_18_b_phase4_2026-05-05/README.md` now identifies `r2` as canonical at the top, but lower sections still say "Run summary (r1; canonical)" and show r1 replay commands. The executable Atom F README/script and r2 artifacts are correct; this is documentation drift, not a substrate or gate failure.
- The missing `_R2_AUDIT_TASKS.md` should be restored or the R2 prompt should cite the actual on-disk scope file for future reproducibility.

Recommended pre-H remediations: none blocking.
