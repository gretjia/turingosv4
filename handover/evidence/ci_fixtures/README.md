# CAS Git Repair CI Fixtures

These archives package the smallest real evidence fragments needed for
fresh-checkout constitution gates. They exist because the original
`handover/evidence/**/cas/` and `runtime_repo/` directories are intentionally
gitignored, while CI must still execute the evidence-binding tests without
local hydration.

- `tb_c0_capsule_cas_fixture.tgz`
  - Source:
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/`
  - Contents:
    `P08_aime_1983_p1/cas`, `P05_mathd_algebra_114/cas`,
    `P07_numbertheory_2pownm1prime_nprime/cas`
  - SHA-256:
    `bf89a75e907fadf9570a19ad0b5ce582fadad6e787c7754849f2e5bdf30d4192`

- `m0_p01_l4e_body_integrity_fixture.tgz`
  - Source:
    `handover/evidence/m0_minif2f_harness_audit_2026-05-10_post_stage_c/P01_mathd_algebra_107`
  - Contents:
    the P01 runtime/CAS evidence needed by assertion #51 L4.E body-integrity
    positive and tamper controls
  - SHA-256:
    `b0fe69febc3f1bf483d944d9ef20ff1484f36d64639f6e6cfdba0b9646dbe2fb`

- `wave3_50p_cas_sidecars_fixture.tgz`
  - Source:
    `handover/evidence/wave3_diagnostic_50p_2026-05-07T14-04-48Z/`
  - Contents:
    the 50 per-problem `cas/.turingos_cas_index.jsonl` sidecar files used by
    the shielding and FC3 raw-log evidence-binding gates
  - SHA-256:
    `ab7e60fb45b262ec4500fd2e22e5ed8ecb235299309f217ea223f19b51c4a642`

These are not synthetic fixtures and do not rewrite historical evidence; they
are compact copies of the ignored evidence fragments used by the existing
constitution gates.
