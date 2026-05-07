# Codex TB-C0 strict constitutional re-audit verdict v3 - 2026-05-07

## Section 1 - Aggregate verdict

**Aggregate verdict: CHALLENGE.**

One-line rationale: round 7 closes the Bug 2 source discriminator and matrix normalization, but the strict aggregate script still cannot produce GAP for a node missing from all manifests because it derives the node universe only from observed manifest keys, and section 3 of the catalog still contains the literal stale "25/25 FC nodes have a tape witness" + "No GAPS" text, albeit as a negated historical note (`scripts/regenerate_post_fix_evidence.sh:171`, `scripts/regenerate_post_fix_evidence.sh:179`, `scripts/regenerate_post_fix_evidence.sh:200`, `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:197`).

Conservative ranking applied: VETO > CHALLENGE > PASS, matching the v2 rule (`handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_v2_2026-05-07.md:23`).

## Section 2 - Per-question verdicts

### Q-V3-1 - Strengthened Bug 2 filter

**Verdict: PASS.**

- The filter binds the requested synthetic sponsor condition with `record.agent_id.0 == "tb6-smoke-agent"` through `SYNTHETIC_GATE_SPONSOR_AGENT_ID` (`src/runtime/chain_derived_run_facts.rs:946`, `src/runtime/chain_derived_run_facts.rs:977`).
- It reads the marker file and requires `synthetic_rejection_for_l4e_gate == true` before any synthetic exclusion can fire (`src/runtime/chain_derived_run_facts.rs:948`, `src/runtime/chain_derived_run_facts.rs:950`, `src/runtime/chain_derived_run_facts.rs:954`, `src/runtime/chain_derived_run_facts.rs:995`).
- It decodes the WorkTx from CAS before checking stake, signature, and tx_id suffix (`src/runtime/chain_derived_run_facts.rs:979`, `src/runtime/chain_derived_run_facts.rs:980`, `src/runtime/chain_derived_run_facts.rs:981`).
- It requires zero stake, all-zero signature bytes, and the `-atom3-l4e-synthetic-rejection` tx_id suffix (`src/runtime/chain_derived_run_facts.rs:983`, `src/runtime/chain_derived_run_facts.rs:987`, `src/runtime/chain_derived_run_facts.rs:989`, `src/runtime/chain_derived_run_facts.rs:992`, `src/runtime/chain_derived_run_facts.rs:995`).
- Cardinality is defensive: when the marker is present and true, any count other than exactly 1 returns `ChainDerivedError::Cas` with a synthetic-gate cardinality diagnostic (`src/runtime/chain_derived_run_facts.rs:1009`, `src/runtime/chain_derived_run_facts.rs:1011`, `src/runtime/chain_derived_run_facts.rs:1012`, `src/runtime/chain_derived_run_facts.rs:1015`).
- Empirical post-fix classification is consistent with closure: all 9 runtime repos have the marker true (`handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P01_mathd_algebra_107/runtime_repo/synthetic_rejection_label.json:1`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P09_aime_1984_p1/runtime_repo/synthetic_rejection_label.json:1`), and the post-fix invariant files report `delta: 0` and `invariant_verdict: "Ok"` across the batch (`handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P01_mathd_algebra_107/chain_invariant_post_fix.json:4`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P01_mathd_algebra_107/chain_invariant_post_fix.json:6`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P09_aime_1984_p1/chain_invariant_post_fix.json:4`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P09_aime_1984_p1/chain_invariant_post_fix.json:6`).
- Trust-root rehash is present: `chain_derived_run_facts.rs` is pinned to `cdbca2e6...`, and the note says it supersedes round-6 hash `a4db21bb` for the 5-condition filter plus cardinality check (`genesis_payload.toml:253`).

No new Q-V3-1 robustness blocker remains.

### Q-V3-2 - Aggregate missing-node semantics

**Verdict: CHALLENGE.**

- Positive closure: the script now tracks per-node `missing` lists, appends a problem when a node is absent from that problem's manifest, and emits `missing_by` into JSON (`scripts/regenerate_post_fix_evidence.sh:180`, `scripts/regenerate_post_fix_evidence.sh:184`, `scripts/regenerate_post_fix_evidence.sh:189`, `scripts/regenerate_post_fix_evidence.sh:217`).
- Positive closure: GREEN is only assigned after red, all-missing, amber, and missing cases are excluded, and only when `len(green) == problem_count` (`scripts/regenerate_post_fix_evidence.sh:198`, `scripts/regenerate_post_fix_evidence.sh:200`, `scripts/regenerate_post_fix_evidence.sh:203`, `scripts/regenerate_post_fix_evidence.sh:206`).
- Positive evidence: emitted aggregate JSON includes `problem_count: 9`, per-node `missing_by`, and the observed post-fix summary `20 GREEN + 5 AMBER + 0 RED` (`handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:6`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:34`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:447`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:448`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:449`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:450`).
- Remaining condition: the "all 9 problems missing a node -> GAP" edge case is not actually enforceable for an expected node absent from every manifest, because `all_node_keys` is built only from the union of keys found in existing manifests (`scripts/regenerate_post_fix_evidence.sh:171`, `scripts/regenerate_post_fix_evidence.sh:172`, `scripts/regenerate_post_fix_evidence.sh:174`, `scripts/regenerate_post_fix_evidence.sh:179`). If a node is missing from all manifests, it is never inserted into `all_node_keys`, so the GAP branch cannot run for that node (`scripts/regenerate_post_fix_evidence.sh:200`, `scripts/regenerate_post_fix_evidence.sh:201`, `scripts/regenerate_post_fix_evidence.sh:202`).

Remaining condition: define the expected FC node universe independently of observed manifests, then aggregate every expected node so an all-missing expected node produces an emitted GAP row.

### Q-V3-3 - Catalog summary normalization

**Verdict: CHALLENGE.**

- Positive closure: section 3 no longer actively claims a single 25/25 tape-witness result; it now presents a witness-class breakdown with chain-resident GREEN, chain-resident AMBER, structural-only AMBER, and tamper-probe classes (`handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:173`, `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:175`, `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:181`, `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:182`, `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:183`, `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:184`).
- Positive closure: FC3-INV1 row is AMBER and says capsule presence is not integrity; recompute is not yet verified (`handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:131`).
- Positive closure: FC1-INV1 uses `externalized_llm_cycle_count` instead of legacy `tx_count` (`handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:84`).
- Positive evidence: post-fix architect checks use `chain_attempt_count == externalized_llm_cycle_count` and show `match: true` for representative P01 and P09 (`handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P01_mathd_algebra_107/architect_inv1_check_post_fix.json:2`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P01_mathd_algebra_107/architect_inv1_check_post_fix.json:4`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P01_mathd_algebra_107/architect_inv1_check_post_fix.json:6`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P09_aime_1984_p1/architect_inv1_check_post_fix.json:4`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P09_aime_1984_p1/architect_inv1_check_post_fix.json:6`).
- Remaining condition: the literal stale text `"25/25 FC nodes have a tape witness" + "No GAPS"` still appears inside section 3, even though it is quoted as a previous incorrect claim and superseded (`handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:197`). The Q-V3-3 check asked whether those phrases were gone; they are not gone.

Remaining condition: remove the literal stale phrase from section 3 or move it to an explicit changelog outside the normalized coverage summary.

### Q-V3-4 - Matrix closure normalization

**Verdict: PASS.**

- Closure #2 is AMBER and explicitly tied to Art. V.3 being RED because a new executable test is still required (`handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:83`, `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:190`).
- Closure #4 is GREEN and cites the post-fix all-9 `delta=0 verdict=Ok` invariant result (`handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:192`).
- Closure #6 is AMBER and explicitly matches the FC3 capsule-derived gate's AMBER status because integrity is not yet verified (`handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:118`, `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:194`).
- Closure #9 is GREEN and states the CI workflow exists plus `make constitution` runs locally (`handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:197`).
- Closure #10 is GREEN and binds the epistemic questions to chain-resident post-fix evidence (`handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:198`).
- Section I has a witness-class column and classifies each FC3 row by witness class, including chain-resident, structural-only, and structural (`handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:116`, `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:118`, `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:119`, `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:120`, `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:121`, `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:122`, `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:123`, `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:124`, `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:125`).

No remaining Q-V3-4 over-claim found.

### Q-V3-5 - Aggregate verdict

**Verdict: CHALLENGE.**

Reason: Q-V3-1 and Q-V3-4 are PASS, but Q-V3-2 and Q-V3-3 remain CHALLENGE; conservative ranking therefore yields CHALLENGE, not PASS (`scripts/regenerate_post_fix_evidence.sh:171`, `scripts/regenerate_post_fix_evidence.sh:179`, `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:197`, `handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_v2_2026-05-07.md:23`).

## Section 3 - New finding not in v2

**Finding V3-C1 - all-missing GAP branch is unreachable for an expected node missing from every manifest. Severity: CHALLENGE.**

The v2 finding said the aggregate missing-node edge case was not strict (`handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_v2_2026-05-07.md:450`, `handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_v2_2026-05-07.md:451`). Round 7 fixes partial missing-by tracking but exposes a sharper residual: `all_node_keys` is populated only from nodes that appear in at least one manifest, so an expected node absent from all 9 manifests has no aggregate row and cannot be reported as GAP (`scripts/regenerate_post_fix_evidence.sh:171`, `scripts/regenerate_post_fix_evidence.sh:172`, `scripts/regenerate_post_fix_evidence.sh:174`, `scripts/regenerate_post_fix_evidence.sh:179`, `scripts/regenerate_post_fix_evidence.sh:200`, `scripts/regenerate_post_fix_evidence.sh:202`).

No VETO-class constitutional violation found in round 7.

## Section 4 - Section-8 readiness recommendation

**NOT READY for section-8 sign-off.**

The prior empirical FC1 VETO blocker is materially closed by post-fix chain invariant evidence (`handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P01_mathd_algebra_107/chain_invariant_post_fix.json:4`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P01_mathd_algebra_107/chain_invariant_post_fix.json:6`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P09_aime_1984_p1/chain_invariant_post_fix.json:4`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P09_aime_1984_p1/chain_invariant_post_fix.json:6`), and the post-fix audit tape files show `passed: 39`, `failed: 0`, and `verdict: "PROCEED"` for the batch (`handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P01_mathd_algebra_107/verdict_post_fix.json:379`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P01_mathd_algebra_107/verdict_post_fix.json:380`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P01_mathd_algebra_107/verdict_post_fix.json:397`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P09_aime_1984_p1/verdict_post_fix.json:379`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P09_aime_1984_p1/verdict_post_fix.json:380`, `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P09_aime_1984_p1/verdict_post_fix.json:397`). However, section-8 should wait until Q-V3-2's expected-node universe/GAP semantics are fixed and Q-V3-3's stale section-3 phrase is removed (`scripts/regenerate_post_fix_evidence.sh:171`, `scripts/regenerate_post_fix_evidence.sh:179`, `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:197`).

## Section 5 - Cross-references to v1 and v2

- v1 verdict file: `handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_2026-05-07.md`; aggregate was VETO until further work (`handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_2026-05-07.md:4`).
- v2 verdict file: `handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_v2_2026-05-07.md`; aggregate was CHALLENGE (`handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_v2_2026-05-07.md:5`).
- v2 CHALLENGE items tracked here: Q-RR1 Bug 2 filter (`handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_v2_2026-05-07.md:56`, `handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_v2_2026-05-07.md:58`), Q-RR3 aggregate semantics (`handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_v2_2026-05-07.md:364`, `handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_v2_2026-05-07.md:366`), Q-RR4 catalog rewrite (`handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_v2_2026-05-07.md:463`, `handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_v2_2026-05-07.md:465`), and Q-RR5 matrix corrections (`handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_v2_2026-05-07.md:579`, `handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_v2_2026-05-07.md:581`).
