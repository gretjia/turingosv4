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

Risk class: Class 3 for the CAS integrity/evidence-storage implementation,
plus a narrowly ratified Class 4 trust-root rehash.

Class 4 boundary check after user ratification:

- `src/bottom_white/cas/schema.rs`: not changed.
- typed tx schema/discriminants: not changed.
- sequencer admission: not changed. `src/state/sequencer.rs` is touched only to
  propagate CAS reload/read integrity errors from the existing rejection
  refinement helper; it does not change acceptance policy, rejection class
  taxonomy, typed tx shape, or signing payloads.
- canonical signing payload: not changed.
- constitution/flowcharts: not changed.
- trust-root authority file `genesis_payload.toml`: changed only after the
  user explicitly authorized Class 4 Trust Root rehash for
  `codex/cas-git-constitutional-repair`; the rehash is limited to pinned files
  already changed by this CAS Git repair.

Trust-root rehash commit:

- `16a9df3c3028bb955c9feacd7d1b4be40f653649`
- Rehashed pinned files: `Cargo.lock`, `Cargo.toml`,
  `src/runtime/evidence_capsule.rs`, `src/bottom_white/cas/mod.rs`,
  `src/bottom_white/cas/store.rs`, `src/state/sequencer.rs`,
  `src/bottom_white/ledger/transition_ledger.rs`.
- Clean-audit P1 closure adds Trust Root coverage for the new authority module
  `src/bottom_white/cas/git_chain.rs` and rehashes `src/state/sequencer.rs`
  after driver-level CAS integrity fail-closed behavior was tightened.

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
| `git diff --check` | PASS | PASS before trust-root rehash commit | Re-run again before final report commit. |
| `cargo fmt --all --check` | Not recorded in baseline | PASS before trust-root rehash | Re-run again before final report commit. |
| `cargo test --lib bottom_white::cas::store::tests -- --test-threads=1` | Not a baseline command | PASS, 33 passed | New targeted CAS chain/cache/fail-closed/lookup/backend-bound/large-legacy-prefix/rebuild-after-missing-cache/oversized-record coverage. |
| `cargo test --lib runtime::evidence_capsule::tests -- --test-threads=1` | Not a baseline command | PASS, 9 passed | New targeted gzip raw-log round-trip, manifest hash, legacy read, schema-id alignment, writer cap, and bounded decompression coverage. |
| `cargo test --test tb_18r_cas_reload_split_brain -- --test-threads=1` | Included in baseline suite | PASS, 7 passed | Adds read/reload integrity fail-closed coverage. |
| `cargo test --test constitution_head_t_c2_multi_ref -- --test-threads=1` | Included in baseline suite | PASS, 11 passed | Adds invalid/symbolic/rewind CAS ref boundary coverage. |
| Baseline targeted suite: `constitution_head_t_c2_multi_ref`, `tb_18r_cas_reload_split_brain`, `co1_7_extra_cas_payload_round_trip`, `tb_18r_lean_result_cas_resolves`, `constitution_tape_canonical_gate`, `constitution_no_parallel_ledger` | PASS, 29 passed | PASS, 34 passed | Five additional targeted assertions; no regression in targeted existing gates. |
| `cargo test -p minif2f_v4 --test trust_root_immutability -- --test-threads=1` | PASS at baseline HEAD by trust-root assumption | PASS, 4 passed | Confirms Class 4 rehash closed the prior Trust Root boot blocker. |
| `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1` | PASS at baseline HEAD by trust-root assumption | PASS, 1 passed | Confirms `genesis_payload.toml` matches current pinned file hashes. |
| `cargo test --lib state::sequencer::tests::run_fails_closed_on_cas_integrity_error_before_continuing_queue -- --test-threads=1` | Not a baseline command | PASS, 1 passed | New clean-audit P1 regression: driver-level CAS integrity error stops `run()` before later queue entries. |
| `bash scripts/run_constitution_gates.sh` | BASELINE FAIL, 443 passed / 18 failed / 1 ignored | FAIL, 446 passed / 18 failed / 1 ignored | Red gate count unchanged from pre-rehash final; three new CAS boundary assertions pass. |
| `cargo test --workspace --no-fail-fast -- --test-threads=1` | ENV FAIL, no space/linker bus error during compile | FAIL, 6 failed targets | Trust-root and `fc_alignment_conformance` failures are closed versus the pre-rehash 9-target failure set; remaining red targets are pre-existing evidence/dashboard gates. |
| `bash scripts/run_g_phase_batch.sh cas_git_repair_* mini` | PASS on isolated baseline worktree | PASS on repair branch after rehash | Both audit `PROCEED`; final adds CAS commit-chain auditability. |
| `MAX_TX=12 PER_PROBLEM_TIMEOUT_S=1800 ... run_tb_18r_r9_evidence.sh` | Audit `PROCEED`, R9 invariant red | Audit `PROCEED`, R9 invariant red | Same class of historical TB-18R R9 invariant failure remains; not introduced by CAS repair. |

Final workspace failed targets observed:

- `-p turingosv4 --test constitution_fc3_evidence_binding`
- `-p turingosv4 --test constitution_fc3_inv1_capsule_integrity_regen`
- `-p turingosv4 --test constitution_l4e_body_integrity`
- `-p turingosv4 --test constitution_librarian_real_evidence_binding`
- `-p turingosv4 --test constitution_shielding_evidence_binding`
- `-p turingosv4 --test tb_16_dashboard_live_regen`

Known red constitution evidence gates pre-existed this branch and remained at
the same total count. The earlier trust-root failures were branch-introduced
because the repair changed pinned files; they are now closed by explicit Class 4
rehash. The broad-suite CAS concurrency regression found during repair was
branch-introduced and fixed before this report: the CAS store suite now passes
`33` tests, including `concurrent_writers_share_index_without_race`.

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
| Sequencer driver fails closed instead of continuing after CAS integrity failure | `run_fails_closed_on_cas_integrity_error_before_continuing_queue` |
| New CAS git-chain authority module is Trust Root pinned | `trust_root_immutability` + `verify_trust_root_passes_on_intact_repo` |
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

The repair worktree has no `.env`, so the real-problem runs explicitly sourced
`/home/zephryj/projects/turingosv4/.env` without copying secrets into the
repair worktree. `DEEPSEEK_API_KEY` was present and the local proxy health
check at `http://localhost:8080/health` returned OK. Low-disk override
`TURINGOS_G_PHASE_LOW_DISK_OK=1` was used for mini runs because `/` had about
11-12G free, below the runner's 20G warning threshold.

Baseline mini evidence:

- Worktree:
  `/home/zephryj/projects/turingosv4-cas-git-repair-real-baseline`
- Commit: `7b39499a6d416081d2eb5cae69cd9278a4fb72ed`
- Path:
  `/home/zephryj/projects/turingosv4-cas-git-repair-real-baseline/handover/evidence/cas_git_repair_baseline_20260517T052432Z`
- Result: 3-task batch, chain continuity OK, `audit_tape=PROCEED`,
  `passed=41 failed=0 halted=0 skipped=11`,
  `PERSISTENCE_BINDING_REPORT is_passing=true n_witnessed=5`.

Final mini evidence:

- Worktree: `/home/zephryj/projects/turingosv4-cas-git-repair`
- Evidence commit:
  `16a9df3c3028bb955c9feacd7d1b4be40f653649`
- Path:
  `/home/zephryj/projects/turingosv4-cas-git-repair/handover/evidence/cas_git_repair_final_trustroot_20260517T054733Z`
- Result: 3-task batch, chain continuity OK, `audit_tape=PROCEED`,
  `passed=41 failed=0 halted=0 skipped=11`,
  `PERSISTENCE_BINDING_REPORT is_passing=true n_witnessed=5`.
- The earlier final attempt at
  `handover/evidence/cas_git_repair_final_20260517T052432Z` failed closed at
  boot with `TRUST_ROOT_TAMPERED` before the user-authorized rehash. A second
  attempt at
  `handover/evidence/cas_git_repair_final_trustroot_20260517T054706Z` was
  rejected by runner preflight because `genesis_payload.toml` was dirty; this
  was handled by committing the rehash before generating final evidence.

Baseline R9 evidence:

- Path:
  `/home/zephryj/projects/turingosv4-cas-git-repair-real-baseline/handover/evidence/cas_git_repair_baseline_r9_20260517T052432Z`
- P01 `mathd_numbertheory_1124`: `audit_tape=PROCEED`, invariant red with
  `delta=-1` / attempt vanished pre-chain.
- P02 `numbertheory_2pownm1prime_nprime`: `audit_tape=PROCEED`, invariant red
  with `delta=3`.
- v4 postprocess: `PASS=0 FAIL=2 NA=0`.

Final R9 evidence:

- Path:
  `/home/zephryj/projects/turingosv4-cas-git-repair/handover/evidence/cas_git_repair_final_r9_20260517T054833Z`
- P01 `mathd_numbertheory_1124`: `audit_tape=PROCEED`, invariant red with
  `delta=-1` / attempt vanished pre-chain.
- P02 `numbertheory_2pownm1prime_nprime`: `audit_tape=PROCEED`, invariant red
  with `delta=10`.
- v4 postprocess: `PASS=0 FAIL=2 NA=0`.

R9 interpretation: both baseline and final produce authoritative audit
`PROCEED` but fail the same family of TB-18R attempt-count invariant checks.
This branch does not claim to repair that historical R9 invariant issue.

## Efficiency And Storage Comparison

Mini batch metrics:

| Metric | Baseline mini | Final mini | Interpretation |
| --- | --- | --- | --- |
| Audit verdict | `PROCEED` | `PROCEED` | Equivalent audit result. |
| `passed/failed` | `41/0` | `41/0` | Equivalent audit count. |
| Runner `elapsed_s` | `35` | `42` | Final is slower in this tiny run; includes CAS chain overhead and live-run variance. |
| L4/L4E commit counts | `24/9` | `24/9` | Runtime ledger shape unchanged. |
| Runtime repo disk | `1.2M` | `1.2M` | No runtime ledger storage regression. |
| CAS Git object files | `66` | `335` | Expected overhead: each CAS object now has commit/tree/record metadata. |
| CAS chain commits | `0` | `67` | Improvement: CAS metadata is now reconstructable from `refs/chaintape/cas`. |
| CAS disk | `640K` | `2.2M` | Expected auditability overhead. |
| Evidence dir disk | `2.0M` | `3.5M` | Expected CAS chain metadata overhead. |

R9 metrics:

| Metric | Baseline R9 | Final R9 | Interpretation |
| --- | --- | --- | --- |
| Evidence dir disk | `2.1M` | `3.9M` | Expected CAS chain metadata overhead plus live-run variance. |
| P01 audit | `PROCEED` | `PROCEED` | Equivalent audit result. |
| P01 CAS objects / disk | `20 / 292K` | `100 / 856K` | CAS chain metadata overhead. |
| P01 runtime repo disk | `448K` | `440K` | Essentially equivalent. |
| P01 invariant | `delta=-1` | `delta=-1` | Same historical invariant family. |
| P02 audit | `PROCEED` | `PROCEED` | Equivalent audit result. |
| P02 CAS objects / disk | `79 / 748K` | `330 / 2.2M` | CAS chain metadata overhead. |
| P02 runtime repo disk | `528K` | `360K` | Live-run attempt count differed; not claimed as a CAS improvement. |
| P02 invariant | `delta=3` | `delta=10` | Same historical invariant family, different live-run trajectory. |

What improved:

- CAS metadata is now reconstructable from Git commit-chain history even when
  `.turingos_cas_index.jsonl` is missing.
- `refs/chaintape/cas` is now a real chain head and has `67` commits in final
  mini evidence instead of being a latest-blob pointer with no chain history.
- Trust-root boot now passes on the repair branch after explicit Class 4
  rehash, enabling final real-problem evidence without bypassing boot checks.

What regressed or costs more:

- CAS storage is larger because each CAS object now carries Git commit-chain
  metadata. This is the explicit tradeoff for replayable/auditable metadata.
- The tiny mini run's runner elapsed time was `42s` final versus `35s`
  baseline; this report does not claim runtime performance improvement.

What stayed equivalent:

- Mini audit verdict/counts, persistence binding, L4/L4E counts, and runtime
  repo disk usage.
- R9 audit verdicts remain `PROCEED` on both baseline and final.

Compression claim:

- New EvidenceCapsule raw logs are gzip-compressed and unit-tested for
  round-trip/hash verification. The real mini/R9 evidence above is not used to
  claim net raw-log compression savings because workload log composition is
  live-run dependent and the CAS chain metadata overhead dominates these small
  runs.

## Residual Risks And Non-Claims

- The branch is not merge-ready until user review accepts the Class 3 CAS
  repair plus the explicitly ratified Class 4 trust-root rehash.
- Workspace-wide tests still do not pass because several evidence/dashboard
  gates were already red at baseline. Trust-root failures from the repair branch
  are now closed.
- Real-problem evidence has run for mini and R9, but R9 still has the same
  historical attempt-count invariant family red in both baseline and final.
- The mini real-problem run used `TURINGOS_G_PHASE_LOW_DISK_OK=1` because the
  filesystem had only about 11-12G free versus the runner's 20G warning
  threshold.
- The new `flate2` production dependency changes `Cargo.lock`; this is useful
  for real gzip compression but requires supply-chain/trust-root review before
  merge. The trust-root hash is updated only on this repair branch after user
  ratification.
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

Round 8 verdict: `CHALLENGE`.

Round 8 findings and remediation:

- P1 Trust Root coverage gap: `src/bottom_white/cas/git_chain.rs` is the new
  CAS git-chain authority module but was not pinned in `genesis_payload.toml`.
  Fixed by adding the file to `[trust_root]` with sha256
  `b4174ab9edd566ca6b443582182fabaecf2fe12438b262e4653969e1eff74bf1`.
  Rechecked with `trust_root_immutability` and
  `verify_trust_root_passes_on_intact_repo`.
- P1 sequencer driver fail-open risk: `Sequencer::run` previously debug-logged
  every `ApplyError` and continued, so a CAS integrity error during rejection
  refinement could skip L4.E for that tx and still process later queue entries.
  Fixed by continuing only on ordinary `ApplyError::Transition`, while
  returning `SequencerError::ApplyFailed` for CAS/key/ledger/encoding/lock
  failures. Added
  `run_fails_closed_on_cas_integrity_error_before_continuing_queue`, which
  injects corrupt CAS metadata, asserts `run()` returns `ApplyFailed(Cas(..))`,
  and asserts later queued work is not processed.

Requested reviewer verdict format: `PROCEED | CHALLENGE | VETO`.

## Merge Guidance

Do not merge directly to `main` until user review confirms:

- Class 3 risk is acceptable;
- the user-authorized Class 4 trust-root rehash is acceptable as a branch-local
  authority update;
- final mini/R9 real-problem evidence and the R9 residual invariant limitation
  are acceptable;
- the remaining six workspace red targets are either fixed separately or
  explicitly accepted as pre-existing evidence/dashboard blockers outside this
  CAS repair;
- clean-context audit remains `PROCEED` after the trust-root rehash and final
  real-problem evidence update.

Recommended merge path after approval:

1. Review the CAS implementation commit and the trust-root rehash commit.
2. Review final mini/R9 evidence paths and this report's efficiency tradeoff.
3. Decide whether remaining evidence/dashboard red gates block this merge or
   belong to a separate repair branch.
4. Merge this branch into main only after evidence and audit agree.
