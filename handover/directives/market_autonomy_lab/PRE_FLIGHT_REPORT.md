# Market Autonomy Lab Pre-Flight Report

Date: 2026-05-16

## Worktree

```text
path: /home/zephryj/projects/turingosv4-market-autonomy-lab
branch: codex/market-autonomy-lab-20260516
turingos_dev_run: dev_1778933784024_2984070
```

## FC / Risk

```text
FC1-N5 / FC1-N6: role-scoped read view and prompt input.
FC1-N8 / FC1-N10: voluntary externalized market action.
FC2-INV1: Trust Root and replay preflight.
FC3-N31 / FC3-INV3: CAS/digest/dashboard materialized views and shielding.
risk: Class 4 for any Trust-Root-pinned implementation atom; Class 0 for this report packet.
```

## Commands

| Command | Result | Evidence |
| --- | --- | --- |
| `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1` | PASS | `handover/evidence/dev_self_hosting/dev_1778933784024_2984070/artifacts/command_0001_stdout.txt` |
| `cargo fmt --all -- --check` | PASS | `handover/evidence/dev_self_hosting/dev_1778933784024_2984070/artifacts/command_0019_stdout.txt` |
| `git diff --check` | PASS | `handover/evidence/dev_self_hosting/dev_1778933784024_2984070/artifacts/command_0020_stdout.txt` |
| `bash scripts/run_constitution_gates.sh` | FAIL | `handover/evidence/dev_self_hosting/dev_1778933784024_2984070/artifacts/command_0004_stdout.txt` |
| `rsync -a --ignore-existing ...wave3...` | PASS | `handover/evidence/dev_self_hosting/dev_1778933784024_2984070/artifacts/command_0009_stdout.txt` |
| `rsync -a --ignore-existing ...m0...` | PASS | `handover/evidence/dev_self_hosting/dev_1778933784024_2984070/artifacts/command_0010_stdout.txt` |
| `rsync -a --ignore-existing ...P08...` | PASS | `handover/evidence/dev_self_hosting/dev_1778933784024_2984070/artifacts/command_0011_stdout.txt` |
| `bash scripts/run_constitution_gates.sh` | PASS | `handover/evidence/dev_self_hosting/dev_1778933784024_2984070/artifacts/command_0012_stdout.txt` |

Note: two early `turingos_dev record-command` calls and one late pair were
started in parallel and reused artifact numbering. The final `fmt` and
`diff --check` commands were rerun serially as `command_0019` and
`command_0020`; use those serial records as authoritative for this packet.

## Constitution Gate Failure

The gate runner reported:

```text
443 passed, 18 failed, 1 ignored
```

Observed failing gate groups:

```text
constitution_fc3_inv1_capsule_integrity_regen
constitution_shielding_evidence_binding
constitution_fc3_evidence_binding
constitution_l4e_body_integrity
```

Diagnostic reruns show the failures are missing historical evidence fixtures in
the independent worktree:

```text
handover/evidence/wave3_diagnostic_50p_2026-05-07T14-04-48Z/.../cas/.turingos_cas_index.jsonl
handover/evidence/m0_minif2f_harness_audit_2026-05-10_post_stage_c/.../runtime_repo
handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P08_aime_1983_p1/runtime_repo
```

The primary workspace at `/home/zephryj/projects/turingosv4` has those fixture
paths. The independent worktree does not, because untracked/ignored historical
evidence is not copied by `git worktree add`.

## Fixture Hydration

The missing immutable fixture directories were hydrated from the primary
workspace using `rsync -a --ignore-existing`, so existing worktree files were not
overwritten. The gate runner then reported:

```text
Totals: 461 passed, 0 failed, 1 ignored
PASS: all gates GREEN.
```

## Current Stop Status

```text
Trust Root: clean
format/diff hygiene: clean
constitution gate: clean after fixture hydration
research envelope preflight: Level2 allowed Trust Root rehash checkpoint
R15 hard10 evidence: PROCEED, Librarian off
R16 hard10 evidence: PROCEED, Librarian on
candidate boundary: E2 candidate pending audit
E2 claim: NOT MADE
```

## Current Verification Refresh

After the R14 CHALLENGE remediation and allowed Trust Root rehash:

```text
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1: PASS
cargo fmt --all -- --check: PASS
git diff --check: PASS
targeted atom gates: PASS
TURINGOS_RESEARCH_ENVELOPE=MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2 bash scripts/run_market_autonomy_research_preflight.sh:
  Level2 Ratification Checkpoint, genesis_payload touched for allowed Trust Root rehash
bash scripts/run_constitution_gates.sh:
  Totals: 461 passed, 0 failed, 1 ignored
```
