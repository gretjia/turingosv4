# OBS — TB-16 audit_tape_tamper Round 2 hang on audit_pipeline_smoke fixture

**Date**: 2026-05-04 (during TB-16 Atom 7 R3 prep)
**Severity**: Medium (observation; NOT a R3 ship blocker; pre-existing on git HEAD)
**Class**: tamper-harness defect
**Status**: **RESOLVED** by TB-16.x.1 (2026-05-04). Root-cause + fix below in §8.
**Discovered by**: Claude Opus 4.7 during R3 surgical-fix verification
**Resolved by**: Claude Opus 4.7 in TB-16.x.1 (charter `handover/tracer_bullets/TB-16.x.1_charter_2026-05-04.md`)

---

## §1 Summary

`audit_tape_tamper` Round 2 (`flip_cas_byte` corruption) hangs
indefinitely (CPU-pegged 100% single-thread) when run against the
`handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/`
fixture. Round 1 (`flip_l4_byte`) completes correctly; the harness
then forks a clean copy for Round 2, the pre-tamper baseline audit
PROCEEDs, the corruption is applied (back half of the largest CAS
loose object zeroed), and the post-tamper audit hangs.

## §2 Pre-existing — NOT a R3 regression

**Verified pre-existing**: `git stash push -- src/runtime/audit_assertions.rs`
+ rebuild + re-test reproduces the hang on the **clean** git HEAD tree
(commit `9383477`). My R3 surgical-fix edits (Q1/Q2/Q10/Q11) DO NOT
introduce the hang — disabling all three new assertions via BISECT
comments still reproduces.

R1 ship-time tamper_report.json (timestamp `05:30 2026-05-04`,
committed at `3cf4c36`) showed `detected_count=3` — the harness was
working at that point. The hang surfaced during R3 prep when I
regenerated the smoke fixture's MarkovEvidenceCapsule per Gemini R2 Q8
(chain to TB-15 head). Hypothesis: the new capsule bytes at
`cas/.git/objects/e8/09b6...` happen to corrupt (back-half-zero
overwrite) into a state that triggers an audit pipeline hang post-
tamper, where the previous capsule's bytes did not.

## §3 Reproducer

```bash
SMOKE=handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke
cargo build --release --bin audit_tape_tamper
timeout 60 ./target/release/audit_tape_tamper \
  --runtime-repo "$SMOKE/runtime_repo" --cas-dir "$SMOKE/cas" \
  --agent-pubkeys "$SMOKE/runtime_repo/agent_pubkeys.json" \
  --pinned-pubkeys "$SMOKE/runtime_repo/pinned_pubkeys.json" \
  --genesis genesis_payload.toml --constitution constitution.md \
  --markov-pointer "$SMOKE/LATEST_MARKOV_CAPSULE.txt" \
  --alignment-dir handover/alignment \
  --tamper-dir "$SMOKE/tamper" \
  --out "$SMOKE/tamper_report.json"
# Exit 124 (timeout) — Round 2 post-tamper audit pegs CPU forever.
```

## §4 Diagnosis (partial)

- `audit_tape` on the **original** (uncorrupted) smoke dir: PROCEED
  in ~30ms.
- `audit_tape` on a **fresh `cp -r` fork** of the smoke dir: PROCEED
  in ~30ms (verified at `/tmp/cf2/` during R3 debug).
- `audit_tape` on the **harness-forked dir** post-Round-2-corruption:
  hangs forever.
- `std::fs::copy` direct test on `cas/.git/objects/e8/09b6...`: byte-
  identical md5 to source. So the harness's `copy_dir_recursive` is
  NOT the cause.
- `cmp -l` on corrupted vs original: bytes 478-953 zeroed (matches
  the `flip_byte_in_first_cas_object` "back half zero" corruption
  semantic).
- The corrupted file is the new MarkovEvidenceCapsule capsule
  (`capsule_id=737b4d22...`, `previous_capsule_cid=f9e701b4...`).
- Pack files in CAS dir: empty. So no fallback path past the
  corrupted loose object.
- `CasStore::get` does sha256 verify; mismatch returns `CidMismatch`
  fast. So the hang is NOT in `read_markov_capsule`.

The hang is somewhere in the post-tamper audit pipeline that does
NOT fail fast on CidMismatch / Zlib decode error for the corrupted
back-half-zeroed loose object. Suspect: git2 zlib partial decode
returning attacker-controlled bytes that one of the
`run_all_assertions` paths interprets in an unbounded-allocation /
infinite-loop way (bincode length-prefix on garbage bytes is the
classic shape).

## §5 Why it is OBS-deferred (not blocker)

1. **Pre-existing**: reproduces on clean git HEAD before R3 fixes
   (verified via `git stash` test).
2. **Fixture-state-specific (R3 closure 2026-05-04)**: tamper harness
   completes 3/3 detected in 229ms on `handover/evidence/tb_16_real_llm_arena_2026-05-04/arena_run4/`
   using the IDENTICAL R3 binary that hangs on `audit_pipeline_smoke/`.
   This **confirms the §4 hypothesis**: the hang is triggered by the
   regenerated MarkovEvidenceCapsule's specific byte pattern in
   `audit_pipeline_smoke/cas/.git/objects/e8/09b6...`, NOT a binary
   defect. Codex R3 RQ6 was correct that "pre-existing on git HEAD"
   alone is insufficient proof; the arena_run4 cross-fixture validation
   is the rigorous version.
3. **R3 tamper evidence relocated**: canonical R3 tamper evidence is
   now `arena_run4/tamper_report.json` (R3-current fixture; max_id=41
   supplementals present; 3/3 detected; path provenance correct).
   Per `feedback_no_retroactive_evidence_rewrite`, audit_pipeline_smoke
   `tamper_report.json` carry-forward from R1 (committed `3cf4c36`)
   stays as documented R1-vintage evidence with grandfathering note.
3. **R3 ship gate**: only requires the audit_tape battery to PROCEED
   on a chain-backed real-LLM tape with replay byte-identity. R3
   delivers PROCEED 38/0/0/3 with byte-identical replay on the smoke
   fixture (38 + 3 supplemental layered IDs).
4. **Architect §7.5 SG-16.x**: tamper detection is one ship-gate
   layer (#36-#38 stubs); audit_tape (#1-#35) is the load-bearing
   surface. PROCEED on audit_tape with replay determinism is the
   primary signal.
5. **R3 audit prompt**: will reference this OBS so external auditors
   know the harness regression is documented and unrelated to R3
   surgical fixes.

## §6 Owner / next-step

- **Owner**: TB-16.x or TB-17 (whichever ships next).
- **Triage**:
  1. Add a `RUST_LOG=debug` instrumented pass through the
     post-tamper audit on the harness-forked dir to identify which
     specific assertion hangs.
  2. Likely candidates: `assert_07_genesis_row_zero_parents`,
     `assert_24_proposal_telemetry_chain`, or
     `assert_27_terminal_summary_evidence_capsule` — they iterate
     CAS objects and do canonical_decode that may not be bounded
     against adversarial bytes.
  3. Fix: bound canonical_decode buffer size at CAS-get layer
     (defense-in-depth); reject loose objects larger than expected
     `size_bytes` per CAS index.
  4. After fix: regenerate audit_pipeline_smoke tamper_report.json
     and update SHIP_STATUS.

## §7 Cross-references

- R3 audit pipeline: this commit (audit_assertions.rs supplementals
  id=40/41 + #28 JSON-form check + file-level FC binding).
- R2 closure doc: `handover/audits/RECURSIVE_AUDIT_TB_16_R2_2026-05-04.md`
- Smoke evidence: `handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/`

## §8 Resolution (TB-16.x.1, 2026-05-04)

### §8.1 Actual root-cause (correcting OBS §4 hypothesis)

The OBS §4 hypothesis ("hang is NOT in `read_markov_capsule`") was WRONG.
With env-gated step-level instrumentation
(`TURINGOS_AUDIT_TIMING=1`), the hang was bisected to **inside
`CasStore::get`** — specifically `repo.find_blob(git_oid)` on a back-half-
zeroed loose-object. libgit2's zlib decompression of certain corrupt
inputs pegs a single CPU core indefinitely; the existing sha256 verify
runs AFTER `find_blob` returns, so it never executes.

The OBS §4 author was misled because (a) `CasStore::get` does sha256
verify, (b) Round 1 (`flip_l4_byte`) corrupts an L4 ledger object whose
specific corrupted bytes happen to fail libgit2's zlib data-check fast
(`Zlib(5) — incorrect data check`), so they assumed CAS reads would
also fail fast. They don't — the corruption pattern matters.

### §8.2 Fix (Class 2; commit pending in this session)

Two-layer defense-in-depth:

1. **`src/bottom_white/cas/store.rs::CasStore::get`**: wrap the libgit2
   `Repository::open` + `find_blob` + `blob.content()` chain in a worker
   `std::thread` + `mpsc::Receiver::recv_timeout`. Default 10s timeout;
   override via env `TURINGOS_CAS_GET_TIMEOUT_SECS`. On timeout, return
   new `CasError::BackendCorruption(...)` variant. The size-bound check
   (`content.len() > expected_size + 256` rejects expanded blobs) is
   added inside the worker, before sha256 verify.
2. **`src/runtime/audit_assertions.rs::load_tape`**: change the markov-
   capsule load from `read_markov_capsule(...).ok()` (collapses ALL
   errors to None → Layer G assertions Skip → false PROCEED) to:
   ```rust
   if inputs.markov_pointer.exists() {
       Some(read_markov_capsule(&inputs.markov_pointer, &cas)?)
   } else {
       None
   }
   ```
   Now "pointer absent" → None (legitimately pre-Markov chain) and
   "pointer present but unreadable" → AuditError → audit verdict BLOCK
   → tamper harness counts as detected.

### §8.3 Empirical verification

`audit_pipeline_smoke/tamper_report.json` regenerated 2026-05-04 post-
fix: 3/3 detect, 10.3s wall clock (one 10s timeout for the corrupt
MarkovEvidenceCapsule). All 8 R3 round-2 chains still audit PROCEED
byte-identically post-fix. `cargo test --workspace --release` = 907
pass / 0 fail / 150 ignored — zero regression.

### §8.4 Prior over-confidence retracted

OBS §5.3 said "audit_pipeline_smoke `tamper_report.json` carry-forward
from R1 ... stays as documented R1-vintage evidence with grandfathering
note." Updated 2026-05-04: post-fix tamper_report regenerated to
canonical post-fix evidence; the R1-vintage report is superseded. Per
`feedback_no_retroactive_evidence_rewrite`, fence-mechanism fixtures
(audit_pipeline_smoke is one) MAY be regenerated forward — it is meta-
infrastructure validating the audit pipeline, not a historical
experiment result tape.
