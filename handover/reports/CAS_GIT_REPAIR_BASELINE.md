# CAS Git Repair Baseline

Date: 2026-05-17

## Scope

This baseline is recorded before implementation edits for the CAS Git
constitutional repair branch.

Constraints:

- Do not use `turingos_dev`.
- Work only in `/home/zephryj/projects/turingosv4-cas-git-repair`.
- Do not merge to main.
- Treat the package as Class 3 unless a restricted/Class 4 surface becomes
  necessary.

## Worktree State

```text
worktree: /home/zephryj/projects/turingosv4-cas-git-repair
branch: codex/cas-git-constitutional-repair
HEAD: 7b39499a6d416081d2eb5cae69cd9278a4fb72ed
status:
## codex/cas-git-constitutional-repair
upstream relation to origin/main: 0 ahead / 1 behind
disk:
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda1        99G   75G   20G  79% /
rust:
rustc 1.94.0 (4a4ef493e 2026-03-02)
cargo 1.94.0 (85eff7c80 2026-01-15)
```

## Baseline Commands

| Command | Result | Notes |
| --- | --- | --- |
| `git diff --check` | PASS | exit 0, no whitespace errors. |
| `cargo test --test constitution_head_t_c2_multi_ref --test tb_18r_cas_reload_split_brain --test co1_7_extra_cas_payload_round_trip --test tb_18r_lean_result_cas_resolves --test constitution_tape_canonical_gate --test constitution_no_parallel_ledger -- --test-threads=1` | PASS | 29 tests passed across the six targeted test targets. |
| `bash scripts/run_constitution_gates.sh` | BASELINE FAIL | 443 passed, 18 failed, 1 ignored. Existing red gates observed before implementation: `constitution_fc3_inv1_capsule_integrity_regen`, `constitution_shielding_evidence_binding`, `constitution_fc3_evidence_binding`, `constitution_l4e_body_integrity`. |
| `cargo test --workspace --no-fail-fast -- --test-threads=1` | ENV FAIL | Build exhausted the project filesystem: `No space left on device (os error 28)`, followed by linker `Bus error`. Repair worktree `target/` reached about 20G and was cleaned after recording. |

## Real-Problem Baseline

LLM/proxy preflight did not pass in this isolated worktree:

- `.env` is absent in `/home/zephryj/projects/turingosv4-cas-git-repair`.
- `DEEPSEEK_API_KEY` is not present in the process environment.

The following real-problem baseline commands were therefore not run; this is a
skip with an explicit preflight reason, not evidence:

- `bash scripts/run_g_phase_batch.sh cas_git_repair_baseline_<UTC> mini`
- `MAX_TX=12 PER_PROBLEM_TIMEOUT_S=1800 bash handover/tests/scripts/run_tb_18r_r9_evidence.sh`

## Baseline Metrics

| Metric | Baseline Value |
| --- | --- |
| CAS object count | Not applicable: no real-problem baseline run; targeted tests use ephemeral temp repos. |
| CAS disk usage | Not applicable for baseline real evidence; no LLM/proxy run executed. |
| Runtime repo disk usage | Not applicable for baseline real evidence; no LLM/proxy run executed. |
| Audit verdict | Not available: real-problem baseline skipped by preflight. |
| Chain invariant delta | Not available: real-problem baseline skipped by preflight. |
| Verifier/audit wall time | Not available: real-problem baseline skipped by preflight. |
