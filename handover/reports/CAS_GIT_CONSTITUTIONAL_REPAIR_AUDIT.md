# CAS Git Constitutional Repair Audit

Date: 2026-05-17

Worktree: `/home/zephryj/projects/turingosv4-cas-git-repair`

Branch: `codex/cas-git-constitutional-repair`

Baseline: `handover/reports/CAS_GIT_REPAIR_BASELINE.md`

## Executive Summary

This branch repairs the CAS root path so `refs/chaintape/cas` is a Git
commit-chain head instead of a latest-blob pointer. `Cid = sha256(content)` is
unchanged. `src/bottom_white/cas/schema.rs` was not modified.

The sidecar `.turingos_cas_index.jsonl` is now a cache when a CAS commit-chain
is present:

- missing sidecar rebuilds from the CAS commit-chain;
- sidecar mismatch against the commit-chain fails closed;
- the hot path still uses an in-memory `BTreeMap`;
- legacy sidecar-only evidence remains readable when no CAS commit-chain
  exists;
- the first forward CAS-chain commit after legacy sidecar-only state carries a
  chunked `legacy_prefix_metadata` snapshot, so the historical prefix is
  reconstructable without rewriting old evidence or creating an oversized chain
  record.

New EvidenceCapsule raw logs are gzip-compressed before CAS write. The manifest
records compression algorithm, raw size, stored size, and uncompressed sha256.
Historical `none-tb11-mvp` manifest values remain readable through the new read
helper.

## Risk And Class Boundary

Risk class: Class 3 by default, because this touches CAS integrity and
production evidence storage.

Class 4 boundary check:

- `src/bottom_white/cas/schema.rs`: not changed.
- typed tx schema/discriminants: not changed.
- sequencer admission: not changed. `src/state/sequencer.rs` is touched only to
  propagate CAS reload/read integrity errors from the existing rejection
  refinement helper; it does not change acceptance policy, rejection class
  taxonomy, typed tx shape, or signing payloads.
- canonical signing payload: not changed.
- constitution/flowcharts: not changed.
- trust-root authority file `genesis_payload.toml`: not changed.

Important merge boundary: `Cargo.lock`, `src/runtime/evidence_capsule.rs`, and
`src/state/sequencer.rs` are trust-root pinned. This branch intentionally does
not rehash `genesis_payload.toml`; doing so is the Class 4 authority step that
must be explicitly ratified before merge.

## Constitution And Flowchart Mapping

Art. 0.2 Tape Canonical:

- Before: durable CAS metadata depended on the sidecar index. If the sidecar
  disappeared, CID-to-metadata lookup was not reconstructable from the Git
  substrate.
- After: each new CAS object is represented by a CAS commit-chain record under
  `refs/chaintape/cas`. The sidecar is a cache, not an independent source of
  truth.

Art. 0.4 Q_t is version-controlled state:

- Before: `refs/chaintape/cas` pointed at a Git blob OID, which gave a pointer
  but not a strict Git history of CAS metadata transitions.
- After: `refs/chaintape/cas` points at a Git commit object, with parent links
  forming a strict chain and commit trees storing CAS metadata and roots.

FC1:

- Touches `wtool -> Q_{t+1}` for CAS/evidence writes.
- The CAS write path now fails closed when the canonical CAS ref cannot advance.
- Lookup helpers derive CIDs from tape/CAS metadata by object type, schema id,
  creator, and logical time.

FC2:

- Touches boot/replay: `CasStore::open` can reconstruct metadata from the CAS
  Git chain when the sidecar cache is missing.
- Legacy sidecar-only repos remain readable if no commit-chain exists.

FC3:

- Touches archived evidence logs: new EvidenceCapsule raw logs are compressed
  with verifiable manifest fields, while old uncompressed manifests remain
  readable.

## Implementation Summary

Code changes:

- Added `src/bottom_white/cas/git_chain.rs`.
- Exported `cas::git_chain` from `src/bottom_white/cas/mod.rs`.
- Updated `CasStore::open`, `reload_index_from_sidecar`, `put`, and lookup
  helpers in `src/bottom_white/cas/store.rs`.
- Updated `Git2LedgerWriter::advance_chaintape_cas_to` and
  `head_chaintape_cas` so CAS refs validate as CAS commit-chain heads.
- Updated the sequencer rejection refinement path to propagate CAS
  sidecar/chain corruption instead of silently falling back.
- Added gzip compression/readback verification to
  `src/runtime/evidence_capsule.rs`.
- Added `flate2` to `Cargo.toml` and updated `Cargo.lock`.

CAS commit records include:

- schema version;
- CID;
- backend Git blob OID;
- object type;
- schema id;
- creator;
- logical time;
- size;
- previous CAS root;
- resulting CAS Merkle root;
- full `CasObjectMetadata`;
- optional `legacy_prefix_metadata` snapshot for the first forward commit after
  sidecar-only legacy state, stored as bounded tree blobs.

Strictness behavior:

- commit-chain record mirror fields must match the embedded metadata;
- parent/root sequence must validate during chain load;
- merge-shaped CAS histories are rejected;
- symbolic CAS refs and invalid direct CAS refs fail closed;
- public CAS ref updates reject valid-but-non-descendant rewinds/forks;
- chain-derived resulting root must match recomputation from accumulated
  metadata;
- CAS-chain append uses a lockfile timeout plus Git object-database refreshes
  so concurrent writer handles can serialize and then observe the latest chain
  objects before advancing the ref;
- CAS chain record blobs are read through a bounded worker path;
- backend blob validation reads Git ODB headers first, enforces a hard maximum
  before content read, then verifies size plus `sha256(content) == cid`;
- a present but non-commit `refs/chaintape/cas` fails closed;
- sidecar mismatch against a present chain fails closed;
- sequencer rejection refinement propagates CAS read/reload integrity errors;
- forced CAS ref update failure makes `put` return error before hot index or
  sidecar accepts the object.

Compatibility behavior:

- no CAS ref + sidecar present: read legacy sidecar;
- chain present + sidecar missing: rebuild from chain;
- chain present + sidecar mismatch: fail closed;
- CAS ref present but not a valid CAS commit-chain head: fail closed;
- first write after sidecar-only legacy state writes one forward CAS commit for
  the new object and includes a chunked legacy prefix metadata snapshot;
- old evidence bytes and old sidecar-only repos are not retroactively rewritten.

## Baseline Vs Final Command Table

| Command | Baseline | Final | Delta |
| --- | --- | --- | --- |
| `git diff --check` | PASS | PASS | Equivalent. |
| `cargo fmt --all --check` | Not recorded in baseline | PASS | Formatting verified. |
| `cargo test --lib bottom_white::cas::store::tests -- --test-threads=1` | Not a baseline command | PASS, 33 passed | New targeted CAS chain/cache/fail-closed/lookup/backend-bound/large-legacy-prefix/rebuild-after-missing-cache/oversized-record coverage. |
| `cargo test --lib runtime::evidence_capsule::tests -- --test-threads=1` | Not a baseline command | PASS, 9 passed | New targeted gzip raw-log round-trip, manifest hash, legacy read, schema-id alignment, writer cap, and bounded decompression coverage. |
| `cargo test --test tb_18r_cas_reload_split_brain -- --test-threads=1` | Included in baseline suite | PASS, 7 passed | Adds read/reload integrity fail-closed coverage. |
| `cargo test --test constitution_head_t_c2_multi_ref -- --test-threads=1` | Included in baseline suite | PASS, 11 passed | Adds invalid/symbolic/rewind CAS ref boundary coverage. |
| Baseline targeted suite: `constitution_head_t_c2_multi_ref`, `tb_18r_cas_reload_split_brain`, `co1_7_extra_cas_payload_round_trip`, `tb_18r_lean_result_cas_resolves`, `constitution_tape_canonical_gate`, `constitution_no_parallel_ledger` | PASS, 29 passed | PASS, 34 passed | Five additional targeted assertions; no regression in targeted existing gates. |
| `bash scripts/run_constitution_gates.sh` | BASELINE FAIL, 443 passed / 18 failed / 1 ignored | FAIL, 446 passed / 18 failed / 1 ignored | Red gate count unchanged; three new CAS boundary assertions pass. |
| `cargo test --workspace --no-fail-fast -- --test-threads=1` | ENV FAIL, no space/linker bus error during compile | FAIL, 9 failed targets | Final run got past compilation. A CAS concurrent-writer failure found during broad testing was fixed and rechecked with the full CAS store suite; remaining failed targets are trust-root plus pre-existing evidence/dashboard failures. |
| `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1` | PASS at baseline HEAD by trust-root assumption | FAIL | First reported tamper is `Cargo.lock`: expected `080b20...`, actual `5d373b...`. `genesis_payload.toml` was intentionally not rehashed. |

Final workspace failed targets observed:

- `-p minif2f_v4 --test trust_root_immutability`
- `-p turingosv4 --lib`
- `-p turingosv4 --test constitution_fc3_evidence_binding`
- `-p turingosv4 --test constitution_fc3_inv1_capsule_integrity_regen`
- `-p turingosv4 --test constitution_l4e_body_integrity`
- `-p turingosv4 --test constitution_librarian_real_evidence_binding`
- `-p turingosv4 --test constitution_shielding_evidence_binding`
- `-p turingosv4 --test fc_alignment_conformance`
- `-p turingosv4 --test tb_16_dashboard_live_regen`

Known red constitution evidence gates pre-existed this branch and remained at
the same total count. The trust-root failures are branch-introduced only because
the repair changes pinned files and the trust-root manifest was not rehashed.
The broad-suite CAS concurrency regression was branch-introduced during repair
and fixed before this report: the CAS store suite now passes `33` tests,
including `concurrent_writers_share_index_without_race`.

## Design Point To Test Mapping

| Design point | Test/evidence |
| --- | --- |
| CAS ref is commit object, not blob | `cas_ref_points_to_commit_object_not_blob_after_put` |
| CAS put advances strict chain roots | `cas_put_advances_strict_commit_chain_roots` |
| Chain reconstructs exact metadata index | `cas_chain_reconstructs_exact_metadata_index` |
| Missing sidecar rebuilds | `missing_sidecar_rebuilds_from_cas_commit_chain` |
| Missing sidecar rebuild followed by a new put keeps cache complete | `missing_sidecar_rebuild_then_put_writes_complete_cache` |
| Legacy prefix remains reconstructable after forward anchor | `missing_legacy_sidecar_with_forward_snapshot_rebuilds_successfully` |
| Large legacy prefix remains reconstructable without oversized chain records | `large_legacy_sidecar_prefix_rebuilds_from_chunked_chain` |
| Tampered sidecar fails closed | `tampered_sidecar_mismatch_fails_closed_when_chain_exists` |
| Present blob CAS ref fails closed | `invalid_blob_cas_ref_fails_open_closed` |
| Symbolic CAS ref fails closed | `sg_a3_cas_ref_symbolic_target_fails_closed` |
| Public CAS ref updater rejects rewind to a valid ancestor | `sg_a3_cas_ref_rejects_rewind_to_valid_ancestor` |
| Forced CAS ref failure fails `put` closed | `forced_cas_ref_update_failure_fails_put_closed` |
| Backend blob OID must resolve to bytes hashing to metadata CID | `cas_chain_rejects_backend_blob_cid_mismatch` |
| Backend blob hard cap rejects before content read | `cas_chain_rejects_backend_blob_above_hard_validation_cap` |
| CAS writer rejects records larger than replay's bounded reader cap | `oversized_cas_chain_record_fails_put_before_ref_or_cache` |
| Backend blob validation timeout fails closed | `forced_backend_validation_timeout_fails_put_closed` |
| Merge-shaped CAS history fails closed | `merge_shaped_cas_chain_fails_validation` |
| Concurrent writer handles serialize and observe latest chain state | `concurrent_writers_share_index_without_race` |
| Initial sequencer CAS read integrity error fails closed | `refine_rejection_class_initial_cas_read_integrity_error_fails_closed` |
| Sequencer reload integrity error fails closed | `refine_rejection_class_reload_integrity_error_fails_closed` |
| Public CAS ref updater rejects generic commits | `sg_a3_cas_ref_rejects_generic_commit_target` |
| Tape-derived lookup helpers return exact CIDs | `tape_derived_lookup_helpers_return_exact_expected_cids` |
| Compressed raw log round-trips and verifies manifest hash | `compressed_raw_log_round_trips_and_manifest_hash_verifies` |
| EvidenceManifest JSON schema and CAS metadata schema id agree | `compressed_raw_log_round_trips_and_manifest_hash_verifies` |
| EvidenceCapsule writer rejects raw logs above readback cap | `writer_rejects_raw_log_above_default_readback_cap` |
| Gzip manifest missing uncompressed size fails closed | `gzip_manifest_missing_uncompressed_size_fails_closed` |
| Gzip manifest understated size fails bounded | `gzip_manifest_understated_uncompressed_size_fails_bounded` |
| Existing targeted gates do not regress | 34-test targeted final suite PASS |
| Constitution gate red count does not regress | baseline 443/18/1; final 446/18/1 |

## Real-Problem Evidence

Baseline real-problem evidence was not run because LLM/proxy preflight failed:

- no `.env` in the repair worktree;
- no `DEEPSEEK_API_KEY`, `OPENAI_API_KEY`, or `ANTHROPIC_API_KEY` in the
  process environment;
- no `HTTP_PROXY`, `HTTPS_PROXY`, or `ALL_PROXY` in the process environment.

Final real-problem evidence was also not run for the same preflight reason.

Commands intentionally not executed as evidence:

- `bash scripts/run_g_phase_batch.sh cas_git_repair_baseline_<UTC> mini`
- `MAX_TX=12 PER_PROBLEM_TIMEOUT_S=1800 bash handover/tests/scripts/run_tb_18r_r9_evidence.sh`
- final-tag equivalents of the above.

This report makes no real-LLM performance or quality claim.

## Efficiency And Storage Comparison

Real-problem storage metrics are unavailable because both baseline and final
real-problem runs were skipped by preflight.

Mechanism-level changes:

- CAS now stores one Git commit record per new CAS object. This adds metadata
  overhead relative to a bare blob-ref pointer, but buys reconstructability,
  strict root history, and sidecar-cache auditability.
- New EvidenceCapsule raw logs are gzip-compressed. The unit test verifies a
  repetitive raw log stores fewer bytes than the uncompressed raw log and
  round-trips through manifest verification.
- Large legacy sidecar prefixes are stored in bounded per-entry tree blobs
  under the first forward CAS-chain commit. This raises commit-tree object count
  for migration commits but prevents one oversized record from blocking replay.
- For tiny or already-compressed logs, gzip may increase stored bytes. No
  branch-level efficiency claim is made without real workload measurements.

## Residual Risks And Non-Claims

- The branch is not merge-ready until the trust-root rehash path is explicitly
  ratified. `genesis_payload.toml` was intentionally not updated in this Class 3
  repair pass.
- Workspace-wide tests do not pass because trust-root verification fails on
  branch-modified pinned files and because several evidence/dashboard gates were
  already red at baseline.
- Real-problem evidence could not run because LLM credentials/proxy preflight
  failed in this isolated worktree.
- The new `flate2` production dependency changes `Cargo.lock`; this is useful
  for real gzip compression but requires supply-chain/trust-root review before
  merge.
- The CAS chain lock uses a simple lockfile with a 120s default timeout. It
  serializes this process family and refreshes Git object databases after
  waiting, but does not implement stale PID recovery.
- Git commit OIDs include deterministic logical-time signatures but still
  depend on Git object serialization and parent history. The canonical CAS CID
  remains `sha256(content)`.
- This branch does not repair the existing red constitution evidence gates; it
  only preserves their baseline red count.

## Clean-Context Audit

Round 1 verdict: `CHALLENGE`.

Round 1 findings and remediation:

- P1 historical synthesis risk: fixed by removing automatic sidecar-to-chain
  bootstrapping in favor of explicit forward-chain anchoring.
- P1 CAS ref boundary bypass: fixed by validating
  `Git2LedgerWriter::advance_chaintape_cas_to` targets as CAS-chain commits.
- P2 backend blob validation: fixed by verifying backend blob size and
  `sha256(content) == cid` during CAS commit append/rebuild.
- P2 constitutional test gap: fixed by adding a constitution test that rejects
  generic commit targets for `refs/chaintape/cas`.

Round 2 verdict: `CHALLENGE`.

Round 2 findings and remediation:

- P2 unbounded CAS-chain blob reads: fixed by reading CAS chain record blobs
  and backend blobs through bounded worker paths with timeout and size limits.
- P2 gzip readback could allocate before manifest validation: fixed by
  requiring gzip manifests to carry `size_bytes_uncompressed` and
  `uncompressed_sha256`, using bounded decompression, and testing missing or
  understated size failures.

Round 3 verdict: `CHALLENGE`.

Round 3 findings and remediation:

- P2 backend validation still lacked a hard cap: fixed by checking Git ODB
  header type/size before reading content, with
  `TURINGOS_CAS_CHAIN_MAX_BACKEND_BLOB_BYTES` as the overrideable cap.
- P2 invalid CAS ref could be treated as no chain: fixed by making a present
  non-commit `refs/chaintape/cas` fail closed during open/rebuild.
- P2 sequencer refinement swallowed reload corruption: fixed by adding a
  checked helper and using it in `record_rejection` so CAS integrity errors
  propagate as `ApplyError::Cas` instead of falling back.

Round 4 verdict: `CHALLENGE`.

Round 4 findings and remediation:

- P1 sidecar-only legacy entries remained authoritative after a chain exists:
  fixed by embedding `legacy_prefix_metadata` in the first forward CAS-chain
  commit, so missing sidecar rebuilds from chain.
- P1 sequencer still swallowed some initial CAS read integrity errors: fixed by
  classifying `AttemptTelemetryError::Cas` as fail-closed except for true
  `CidNotFound`.
- P2 symbolic CAS refs could be treated as no chain: fixed by making present
  symbolic/invalid CAS refs fail closed in chain discovery and ledger head
  lookup.
- P2 merge-shaped CAS histories were accepted: fixed by rejecting commits with
  more than one parent during chain validation.

Round 5 verdict: `CHALLENGE`.

Round 5 findings and remediation:

- P1 oversized legacy prefix snapshot could make the first forward CAS-chain
  commit unreplayable: fixed by storing legacy prefix metadata in bounded
  per-entry tree blobs and adding large-prefix rebuild coverage.
- P2 public CAS ref updater could rewind to a valid ancestor/fork: fixed by
  requiring the target to be the current head or a descendant of the current
  CAS head.
- P2 EvidenceManifest schema namespace drift: fixed by writing CAS metadata
  `schema_id = "v2/evidence_manifest"` for new v2 manifests and testing the
  manifest JSON/schema-id match.
- P2 writer/readback size mismatch: fixed by rejecting raw logs above
  `TURINGOS_EVIDENCE_LOG_MAX_UNCOMPRESSED_BYTES` before compression/write.

Post-Round-5 broad-suite remediation:

- The final workspace run exposed a branch-introduced CAS store concurrency
  failure in `concurrent_writers_share_index_without_race`.
- Fixed by extending the CAS-chain lock timeout to 120s and refreshing Git ODB
  state after lock wait/retry boundaries.
- Rechecked at the time with `cargo test --lib
  bottom_white::cas::store::tests -- --test-threads=1`: PASS, `31`
  passed. After Round 6 remediations, the same suite passes `33` tests.

Round 6 verdict: `CHALLENGE`.

Round 6 findings and remediation:

- P1 incomplete sidecar after missing-cache rebuild: fixed by rewriting a full
  sidecar cache from the prospective chain-derived index when a `put` follows a
  missing-sidecar rebuild, and tested with
  `missing_sidecar_rebuild_then_put_writes_complete_cache`.
- P1 writer could create an oversized CAS-chain record that replay would
  reject: fixed by checking `cas_chain_record.json` and `metadata.json` sizes
  against the bounded reader cap before creating the CAS commit/ref, and tested
  with `oversized_cas_chain_record_fails_put_before_ref_or_cache`.

Round 7 verdict: `PROCEED`.

Round 7 findings:

- No blocking production defects found.
- The reviewer verified that missing-sidecar rebuild followed by `put` rewrites
  a complete prospective cache before the sidecar becomes authoritative as a
  warm cache again.
- The reviewer verified that oversized CAS-chain record, metadata, and chunks
  are rejected before commit/ref/cache mutation.
- The reviewer did not find hidden Class 4 schema, admission, or signing
  payload changes.
- Residual trust-root, workspace broad-test, and real-problem evidence limits
  were accepted as correctly represented ship/report blockers rather than
  hidden production defects.

Requested reviewer verdict format: `PROCEED | CHALLENGE | VETO`.

## Merge Guidance

Do not merge directly to `main` until user review confirms:

- Class 3 risk is acceptable;
- the trust-root rehash/Class 4 authority step is ratified or the dependency
  and pinned-file strategy is revised;
- real-problem evidence can be rerun with credentials/proxy available;
- Round 7 clean-context audit `PROCEED` remains acceptable after any user-side
  review changes.

Recommended merge path after approval:

1. Ratify and perform the trust-root rehash if this implementation strategy is
   accepted.
2. Re-run targeted tests, constitution gates, and workspace tests.
3. Run real-problem baseline/final equivalents if LLM credentials are
   available.
4. Merge this branch into main only after evidence and audit agree.
