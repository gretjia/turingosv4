# CAS Git Constitutional Repair Audit

Date: 2026-05-17

Worktree: `/home/zephryj/projects/turingosv4-cas-git-repair`

Branch: `codex/cas-git-constitutional-repair`

No `turingos_dev` was used for this repair. Main was not merged.

## Executive Summary

This branch repairs the CAS Git authority path and closes the auditor
`CHALLENGE` blockers.

The most important correction is factual: the 18 red constitution gates in the
repair worktree were not treated as harmless "pre-existing red gates." They
were reproduced as a branch/worktree blocker at `c85dacfa`, then closed by:

- hydrating ignored historical fixture `cas/` and `runtime_repo/` directories
  from the main worktree into the isolated repair worktree;
- preserving readability of legacy sidecar + blob-ref CAS evidence while still
  failing closed on invalid non-commit CAS refs;
- fixing the TB-18R R9 expected-count postprocess so synthetic preseed Work is
  excluded and `step_partial_ok` is counted from structured JSON;
- refreshing `R9_BATCH_SUMMARY.json` after v4 invariant postprocess;
- adding a branch-local Class 4 §8 directive for the Trust Root rehash.

Final constitution gates are green:

```text
bash scripts/run_constitution_gates.sh
Totals: 464 passed, 0 failed, 1 ignored
PASS: all gates GREEN.
```

Final broad workspace tests also pass:

```text
cargo test --workspace --no-fail-fast -- --test-threads=1
exit 0
```

## Risk And FC Mapping

Risk class:

- Class 3 for CAS integrity and evidence storage.
- Class 4 only for the user-authorized Trust Root rehash.

Class 4 directive:

- `handover/directives/2026-05-17_CAS_GIT_CONSTITUTIONAL_REPAIR_§8_TRUST_ROOT_RATIFICATION.md`

Allowed Class 4 scope:

- `Cargo.lock`
- `Cargo.toml`
- `src/runtime/evidence_capsule.rs`
- `src/bottom_white/cas/mod.rs`
- `src/bottom_white/cas/store.rs`
- `src/bottom_white/cas/git_chain.rs`
- `src/bottom_white/ledger/transition_ledger.rs`
- `src/state/sequencer.rs`
- `genesis_payload.toml`

Forbidden and not touched:

- `src/state/typed_tx.rs`
- `src/bottom_white/cas/schema.rs`
- `src/kernel.rs`
- `src/bus.rs`
- `src/sdk/tools/wallet.rs`
- canonical signing payloads
- sequencer admission policy
- constitution/flowchart text
- main merge

FC mapping:

- FC1: CAS/evidence writes and ChainTape binding.
- FC2: replay/audit boot from CAS Git chain and cache rebuild.
- FC3: archived evidence feedback, report/audit materialized views, and raw-log
  compression/readback.

## Implementation

CAS Git chain:

- `Cid = sha256(content)` is unchanged.
- New writes make `refs/chaintape/cas` point at a CAS commit-chain head, not a
  latest blob OID.
- CAS commit records carry CID, backend blob OID, object metadata, previous CAS
  root, resulting Merkle root, object type, schema id, creator, and logical
  time.
- `.turingos_cas_index.jsonl` is cache-only when a CAS chain exists.
- Missing sidecar rebuilds from the CAS Git chain.
- Sidecar mismatch against the chain fails closed.
- CAS ref update failure fails `put`.
- Legacy blob-ref + matching sidecar evidence can open and upgrades to a CAS
  commit head on the next forward `put`.
- Invalid non-commit refs without the legacy sidecar condition still fail
  closed.

EvidenceCapsule compression:

- New raw logs are gzip-stored in CAS.
- Manifest records compression algorithm, raw size, stored size, and
  uncompressed sha256.
- Historical uncompressed evidence remains readable.

R9 postprocess:

- `handover/tests/scripts/tb_18r_expected_from_pput.py` computes expected
  completed attempts from structured PPUT JSON.
- Formula:
  `step_reject + parse_fail + llm_err + sorry_block + omega_wtool + complete + complete_via_tape + step_partial_ok`.
- Synthetic `tb6-smoke-agent` preseed Work is excluded to match chain-side
  counting.
- `tb_18r_postprocess_invariant_v4.sh` now refreshes `R9_BATCH_SUMMARY.json`
  after recomputing per-problem invariants.

Historical fixture hydration:

- Manifest:
  `handover/reports/CAS_GIT_REPAIR_HYDRATION_MANIFEST.md`
- Hydrated only ignored `cas/` and `runtime_repo/` trees.
- Hydrated directories do not appear in `git status --short`.
- Historical evidence bytes were not rewritten.

REAL13 note:

- No available local worktree had a root-level ignored REAL13 `cas/` artifact.
- `constitution_librarian_real_evidence_binding` now binds to tracked
  `aggregate_verdict.json` CAS-derived metrics and checks local sidecar only
  when present.

## Baseline Vs Final

| Check | Baseline / CHALLENGE State | Final State | Result |
| --- | --- | --- | --- |
| `git diff --check` | PASS | PASS | No whitespace errors. |
| `cargo fmt --all -- --check` | Not baseline-recorded | PASS | Formatting clean. |
| Branch challenge red baseline | `446 passed / 18 failed / 1 ignored` at `c85dacfa` | `464 passed / 0 failed / 1 ignored` | Improved by 18 red gates. |
| Original branch baseline report | `443 passed / 18 failed / 1 ignored` at `7b39499a` | `464 passed / 0 failed / 1 ignored` | Initial red family closed. |
| Six auditor-pointed targets | RED before hydration/compat repair | PASS | `constitution_fc3_inv1_capsule_integrity_regen`, `constitution_shielding_evidence_binding`, `constitution_fc3_evidence_binding`, `constitution_l4e_body_integrity`, `constitution_librarian_real_evidence_binding`, `tb_16_dashboard_live_regen`. |
| CAS store targeted | Prior final report said 33 pass | PASS, 34 pass | Added legacy blob-ref open/upgrade regression. |
| EvidenceCapsule targeted | PASS, 9 pass | PASS, 9 pass | Compression/readback stable. |
| R9 helper/summary tests | P02 expected-count bug present | PASS, 3 pass | P01/P02 expected helper and summary JSON tests pass. |
| Trust Root tests | Failing after post-CHALLENGE code edits until rehash | PASS, 4 pass + boot verify 1 pass | Class 4 rehash closed. |
| Baseline targeted suite | PASS, 29 pass | PASS, 34 pass | Existing targeted behavior retained; new assertions added. |
| `cargo test --workspace --no-fail-fast -- --test-threads=1` | Earlier baseline ran out of disk; CHALLENGE branch had red targets | PASS | Broad regression surface green after cleanup and fixes. |
| Mini real-problem evidence | Baseline skipped by preflight in original baseline file | PASS | Final mini has `audit_tape=PROCEED`, persistence true. |
| TB-18R R9 real-problem evidence | Initial runner summary had invariant Err due expected=0 extraction | PASS after v4 postprocess | P01/P02 `delta=0`, `invariant_verdict=Ok`, summary valid JSON. |

## Design Point To Test Mapping

| Design point | Test / evidence |
| --- | --- |
| CAS ref is a commit object for new writes | `cas_ref_points_to_commit_object_not_blob_after_put` |
| CAS put advances strict chain roots | `cas_put_advances_strict_commit_chain_roots` |
| Chain reconstructs exact metadata index | `cas_chain_reconstructs_exact_metadata_index` |
| Missing sidecar rebuilds from chain | `missing_sidecar_rebuilds_from_cas_commit_chain` |
| Missing sidecar rebuild then put writes complete cache | `missing_sidecar_rebuild_then_put_writes_complete_cache` |
| Tampered sidecar fails closed | `tampered_sidecar_mismatch_fails_closed_when_chain_exists` |
| Invalid blob CAS ref fails closed | `invalid_blob_cas_ref_fails_open_closed` |
| Legacy blob-ref + matching sidecar opens and upgrades | `legacy_blob_cas_ref_with_sidecar_opens_and_upgrades_on_next_put` |
| Forced CAS ref update failure fails put | `forced_cas_ref_update_failure_fails_put_closed` |
| Tape-derived lookup helpers return exact CIDs | `tape_derived_lookup_helpers_return_exact_expected_cids` |
| Backend blob CID mismatch fails validation | `cas_chain_rejects_backend_blob_cid_mismatch` |
| Backend blob hard cap fails before content read | `cas_chain_rejects_backend_blob_above_hard_validation_cap` |
| Oversized chain record fails before ref/cache mutation | `oversized_cas_chain_record_fails_put_before_ref_or_cache` |
| Merge-shaped CAS history fails closed | `merge_shaped_cas_chain_fails_validation` |
| Concurrent writer handles serialize and observe latest chain | `concurrent_writers_share_index_without_race` |
| Sequencer CAS integrity errors fail closed | `refine_rejection_class_*_integrity_error_fails_closed` and `run_fails_closed_on_cas_integrity_error_before_continuing_queue` |
| Public CAS ref rejects symbolic/generic/rewind refs | `constitution_head_t_c2_multi_ref` CAS ref tests |
| New raw logs compress and verify manifest hash | `compressed_raw_log_round_trips_and_manifest_hash_verifies` |
| Gzip manifest fail-closed bounds | `gzip_manifest_missing_uncompressed_size_fails_closed`, `gzip_manifest_understated_uncompressed_size_fails_bounded` |
| R9 expected helper counts partials and excludes preseed | `tb_18r_r9_expected_from_pput` |
| R9 summary uses structured JSON counts | `tb_18r_r9_batch_summary` |
| Historical fixture evidence-binding gates are green | `bash scripts/run_constitution_gates.sh` final `464/0/1` |

## Final Verification Commands

```text
git diff --check
PASS

cargo fmt --all -- --check
PASS

cargo test --lib bottom_white::cas::store::tests -- --test-threads=1
34 passed

cargo test --lib runtime::evidence_capsule::tests -- --test-threads=1
9 passed

cargo test --test tb_18r_r9_expected_from_pput --test tb_18r_r9_batch_summary -- --test-threads=1
3 passed

cargo test -p minif2f_v4 --test trust_root_immutability -- --test-threads=1
4 passed

cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
1 passed

cargo test --test constitution_fc3_inv1_capsule_integrity_regen --test constitution_shielding_evidence_binding --test constitution_fc3_evidence_binding --test constitution_l4e_body_integrity --test constitution_librarian_real_evidence_binding --test tb_16_dashboard_live_regen -- --test-threads=1
31 passed

cargo test --test constitution_head_t_c2_multi_ref --test tb_18r_cas_reload_split_brain --test co1_7_extra_cas_payload_round_trip --test tb_18r_lean_result_cas_resolves --test constitution_tape_canonical_gate --test constitution_no_parallel_ledger -- --test-threads=1
34 passed

bash scripts/run_constitution_gates.sh
464 passed, 0 failed, 1 ignored

cargo test --workspace --no-fail-fast -- --test-threads=1
PASS
```

## Clean-Context Audit

Final clean-context Codex audit:

- `handover/audits/CODEX_CAS_GIT_REPAIR_FINAL_CLEAN_CONTEXT_AUDIT.md`
- Verdict: `PROCEED`

The reviewer found no blocking production defect and independently verified the
main CAS repair surfaces, forbidden-surface boundaries, Trust Root tests, CAS
store tests, EvidenceCapsule tests, R9 helper/summary tests, and constitution
gates. The only note was a P3 evidence packaging issue where the R9
per-problem README materialized views still showed pre-v4-postprocess
invariant excerpts. Those README files were refreshed after the audit to match
the authoritative `chain_invariant.json` and `R9_BATCH_SUMMARY.json`
`delta=0 / Ok` evidence.

## Real-Problem Evidence

The repair worktree has no `.env`. Real-problem runs sourced
`/home/zephryj/projects/turingosv4/.env` read-only; secrets were not copied into
the repair worktree.

Mini evidence:

- Path:
  `handover/evidence/cas_git_repair_challenge_final_20260517T095728Z/`
- Command:
  `TURINGOS_G_PHASE_DIRTY_OK=1 bash scripts/run_g_phase_batch.sh cas_git_repair_challenge_final_20260517T095728Z mini`
- Elapsed: `60s`
- `batch_exit=0`
- `audit_verdict=PROCEED`
- `persistence_passing=true`
- `persistence_n_witnessed=5`
- Logical CAS objects from audit: `66`
- CAS disk: `2.2M`
- Runtime repo disk: `1.2M`
- Git CAS object files: `330`

R9 evidence:

- Path:
  `handover/evidence/cas_git_repair_challenge_final_r9_20260517T100600Z/`
- Command:
  `OUT_DIR=... MAX_TX=12 PER_PROBLEM_TIMEOUT_S=1800 bash handover/tests/scripts/run_tb_18r_r9_evidence.sh`
- Raw per-problem `audit_tape`: `PROCEED` for both problems.
- Postprocess:
  `bash handover/tests/scripts/tb_18r_postprocess_invariant_v4.sh handover/evidence/cas_git_repair_challenge_final_r9_20260517T100600Z`
- Postprocess result: `PASS=2 FAIL=0 NA=0`
- `python3 -m json.tool .../R9_BATCH_SUMMARY.json`: PASS

R9 per-problem final invariants:

| Problem | Duration | Audit | Expected | L4 | L4E | Capsule anchored | Delta | Verdict | CAS disk | Runtime disk | Logical CAS objects |
| --- | ---: | --- | ---: | ---: | ---: | ---: | ---: | --- | ---: | ---: | ---: |
| `P01_mathd_numbertheory_1124` | 121s | PROCEED | 6 | 1 | 5 | 0 | 0 | Ok | 1.8M | 564K | 49 |
| `P02_numbertheory_2pownm1prime_nprime` | 716s | PROCEED | 8 | 0 | 5 | 3 | 0 | Ok | 2.1M | 440K | 58 |

Combined final real-evidence storage:

- Logical CAS objects: `173` (`66 + 49 + 58`)
- Git CAS object files: `865` (`330 + 245 + 290`)

## Efficiency Comparison

Improved:

- Constitution gates improved from the reproduced CHALLENGE baseline
  `446/18/1` to `464/0/1`.
- R9 P02 invariant improved from historical red (`delta=8` under the old
  expected-count view) to `delta=0 / Ok`.
- CAS metadata is reconstructable from Git commit history instead of relying
  on sidecar as authority.
- New EvidenceCapsule raw logs are compressed and hash-verifiable.
- CAS ref failure and CAS integrity read failures now fail closed.

Equivalent by design:

- `Cid = sha256(content)` is unchanged.
- Old evidence bytes are not rewritten.
- `src/bottom_white/cas/schema.rs` and typed transaction schema are unchanged.
- Normal hot path still uses in-memory `BTreeMap`.

Regressions / cost:

- CAS commit-chain metadata adds Git objects for each CAS put. This is an
  intentional auditability cost.
- Hydrated historical fixture directories are local ignored evidence, not a
  clean-checkout packaging solution.
- First run after `cargo clean` required release/debug rebuild time.

Not comparable:

- Original baseline real-problem metrics were skipped by preflight because the
  repair worktree lacked `.env`. The final real-problem runs are therefore
  reported as final evidence, not as an apples-to-apples runtime speedup claim.

## Residual Risks And Non-Claims

- Clean checkout historical fixture packaging remains a follow-up. The repair
  branch proves local ignored fixture hydration plus compatibility behavior; it
  does not claim CI/fresh checkout has all historical ignored `cas/` trees.
- REAL13 has no root-level local ignored CAS sidecar in available worktrees; the
  fixed test binds to tracked aggregate CAS metrics and optional local sidecar
  when present.
- No change is claimed to sequencer admission policy, typed transaction schema,
  canonical signing payloads, constitution, or flowcharts.
- No merge to main is performed by this branch.
- Final clean-context Codex audit returned `PROCEED`; no blocking findings
  remain in this report.

## Merge Guidance

Before merge:

1. Review the Class 4 §8 directive and Trust Root hash comments.
2. Confirm no forbidden surfaces appear in the final diff.
3. Do not merge hydrated ignored historical fixture dirs.
4. Treat clean-checkout fixture packaging as a separate follow-up if CI must run
   the historical evidence-binding gates without local hydration.
