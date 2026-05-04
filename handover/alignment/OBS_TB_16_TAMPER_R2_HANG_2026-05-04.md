# OBS — TB-16 audit_tape_tamper Round 2 hang on audit_pipeline_smoke fixture

**Date**: 2026-05-04 (during TB-16 Atom 7 R3 prep)
**Severity**: Medium (observation; NOT a R3 ship blocker; pre-existing on git HEAD)
**Class**: tamper-harness defect
**Status**: OBS-deferred to TB-16.x
**Discovered by**: Claude Opus 4.7 during R3 surgical-fix verification

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

1. **Pre-existing**: reproduces on clean git HEAD before R3 fixes.
2. **R1 carry-forward valid**: tamper logic in `src/bin/audit_tape_tamper.rs`
   was NOT touched in R3. The R1 `tamper_report.json` (3/3 detected,
   committed `3cf4c36`) remains architecturally valid evidence that
   the harness CAN detect tampering.
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
