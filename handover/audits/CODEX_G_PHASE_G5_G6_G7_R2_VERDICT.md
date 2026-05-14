# Clean-Context Codex Audit — TB-G G5/G6/G7 + SG-G Packet R2

Date: 2026-05-14

Reviewer: clean-context Codex subagent

Verdict: CHALLENGE

## Findings

1. **CHALLENGE: current staged packet fails whitespace verification.**

   This is a packaging/evidence hygiene blocker, not a G5/G6/G7
   production-code defect. The reviewer ran:

   ```bash
   git diff --cached --check
   ```

   It exited non-zero and reported staged generated evidence artifacts with
   blank-line/trailing-whitespace errors, including:

   - `handover/evidence/dev_self_hosting/dev_1778720042689_71801/artifacts/command_0001_stdout.txt:45`
   - `handover/evidence/dev_self_hosting/dev_1778720042689_71801/artifacts/command_0003_stdout.txt:45`
   - `handover/evidence/dev_self_hosting/dev_1778720042689_71801/artifacts/command_0004_stdout.txt:6`
   - `handover/evidence/dev_self_hosting/dev_1778720042689_71801/artifacts/command_0006_stdout.txt:2782`
   - `handover/evidence/dev_self_hosting/dev_1778720042689_71801/artifacts/diff.patch:6`
   - `handover/evidence/g_phase_g7_structural_2026-05-14T00-00-00Z/RUN_REPORT_G5_G6_G7.md:572`

   Required closure: clean the staged packet or explicitly exclude raw generated
   evidence artifacts from the commit packet before SG-G closeout.

2. **CHALLENGE: the R-022 command evidence is present in the working tree but
   not in the current staged packet.**

   The working-tree `events.jsonl` records
   `python3 scripts/check_trace_matrix.py --mode commit` as event 9 with
   `exit_code: 0`, but the staged version stops at event 8. This is an
   evidence-packaging gap, not a source-code gap.

## R1 Closure

The R1 source-code issue itself is closed. The new public APIs now have
adjacent `/// TRACE_MATRIX ...` backlinks, including:

- `src/runtime/agent_scheduler.rs`
- `src/runtime/agent_role_classifier.rs`
- `src/runtime/g7_structural_smoke.rs`
- `src/sdk/market_context.rs`

No new restricted-surface changes were found in sequencer, TypedTx, kernel,
bus, wallet, signing payload, or CAS schema surfaces.

## Verdict

CHALLENGE
