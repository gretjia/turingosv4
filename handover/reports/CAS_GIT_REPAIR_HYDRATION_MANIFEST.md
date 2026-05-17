# CAS Git Repair Historical Fixture Hydration Manifest

Date: 2026-05-17

Worktree: `/home/zephryj/projects/turingosv4-cas-git-repair`

Source worktree: `/home/zephryj/projects/turingosv4`

Purpose: hydrate ignored historical `cas/` and `runtime_repo/` fixture
directories needed by evidence-binding tests. These directories are cache/raw
evidence artifacts ignored by `.gitignore`; they are not committed.

## Hydrated Roots

| Evidence root | Hydrated `cas/` + `runtime_repo/` dirs | Hydrated files | Final size |
| --- | ---: | ---: | ---: |
| `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z` | 18 | 1107 | 8.7M |
| `handover/evidence/wave3_diagnostic_50p_2026-05-07T14-04-48Z` | 100 | 5105 | 39M |
| `handover/evidence/m0_minif2f_harness_audit_2026-05-10_post_stage_c` | 160 | 8696 | 64M |
| `handover/evidence/stage_a3_r5_smoke_2026-05-08T05-40-39Z` | 2 | 112 | 856K |
| `handover/evidence/stage_a3_r35_smoke_2026-05-08T06-02-28Z` | 2 | 125 | 944K |
| `handover/evidence/stage_b3_r6_minim1_2026-05-08T06-07-32Z` | 16 | 1115 | 8.2M |
| `handover/evidence/real13_market_pressure_probe_20260516T071216Z` | 0 | 0 | 132K |
| `handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171` | 2 | 70 | 496K |

## Ignore Verification

Representative hydrated files are ignored by the repo root `.gitignore`:

```text
.gitignore:38:handover/evidence/**/cas/        handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P08_aime_1983_p1/cas/.turingos_cas_index.jsonl
.gitignore:39:handover/evidence/**/runtime_repo/ handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P08_aime_1983_p1/runtime_repo/rejections.jsonl
.gitignore:38:handover/evidence/**/cas/        handover/evidence/wave3_diagnostic_50p_2026-05-07T14-04-48Z/P01_mathd_algebra_107/cas/.turingos_cas_index.jsonl
.gitignore:39:handover/evidence/**/runtime_repo/ handover/evidence/m0_minif2f_harness_audit_2026-05-10_post_stage_c/P01_mathd_algebra_107/runtime_repo/rejections.jsonl
.gitignore:39:handover/evidence/**/runtime_repo/ handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/rejections.jsonl
```

Normal `git status --short` does not list the hydrated historical fixture
directories. It lists only tracked edits and pre-existing untracked final CAS
repair evidence directories.

## REAL13 Note

`real13_market_pressure_probe_20260516T071216Z` has no root-level ignored
`cas/` or `runtime_repo/` artifact in the main worktree or sibling local
worktrees checked during repair. The binding test now uses the tracked
`aggregate_verdict.json` non-empty `tape_root.cas_object_count` and report
CAS-derived metrics, while still checking a local sidecar is non-empty if one
is present.
