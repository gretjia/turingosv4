# A04 GitTape Physical Ledger Preflight

Date: 2026-06-05

Parent plan:
`handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`

Atom: A04. GitTape Physical Ledger and Single Writer

Document role: Class 0 preflight. This document does not authorize L2 ledger,
sequencer, typed transaction, signing payload, or trust-root edits by itself.

## Decision

A04 must not blindly implement against the original parent-plan allowed path
set. That original text listed `src/git_tape_ledger.rs`, but the current
Agentic OS ChainTape/L4 Git writer is `Git2LedgerWriter` in
`src/bottom_white/ledger/transition_ledger.rs`.

Safe work now:

- docs-only preflight
- interface-contract correction
- test and audit recipe preparation

Blocked until explicit A04 implementation authorization:

- changing `src/bottom_white/ledger/transition_ledger.rs`
- changing `src/state/sequencer.rs`
- changing `src/state/typed_tx.rs`
- changing canonical signing payloads
- changing `src/git_tape_ledger.rs` as if it were the OS L2 writer

## Hard Blockers

- A04-HB1: A04 cannot claim OS L2 completion until the atom explicitly chooses
  and ratifies ChainTape-L4 authority rather than TDMA compatibility.
- A04-HB2: Sequencer admission, typed transaction schema, or canonical signing
  payload changes require explicit Class 4 §8 ratification.
- A04-HB3: Any append path without expected-old `GitOid` OCC or a single-writer
  sequencer, duplicate `logical_t` rejection, and generated-repo
  `git fsck --full` evidence is blocked.

## Current-State Facts

Original parent-plan A04 allowed paths before this preflight correction:

```text
src/git_tape_ledger.rs
tests/tc_git_tape_ledger_hardening.rs
tests/git_tape_ledger_roundtrip.rs
```

Corrected canonical A04 implementation path inventory:

```text
src/bottom_white/ledger/transition_ledger.rs
src/runtime/chain_tape_lease.rs
src/runtime/resume_preflight.rs
src/bottom_white/ledger/rejection_evidence.rs
src/bottom_white/cas/git_chain.rs
src/bottom_white/cas/store.rs
src/git_tape_ledger.rs
tests/tc_git_tape_ledger_hardening.rs
tests/git_tape_ledger_roundtrip.rs
tests/constitution_g1_resume.rs
tests/tb_6_verify_chaintape.rs
```

Existence check:

```text
EXISTS src/git_tape_ledger.rs
MISSING tests/tc_git_tape_ledger_hardening.rs
EXISTS tests/git_tape_ledger_roundtrip.rs
```

Relevant current dirty paths:

```text
src/git_tape_ledger.rs
tests/git_tape_ledger_roundtrip.rs
src/bottom_white/ledger/transition_ledger.rs
tests/constitution_g1_resume.rs
tests/tb_6_verify_chaintape.rs
```

Open PR overlap check:

```text
#280 AUDIT ONLY / DO NOT MERGE does not directly edit the current A04
     candidate implementation paths listed above, but remains a quarry for
     TC operationalization material.
#283 AUDIT ONLY / DO NOT MERGE does not define the A04 implementation path.
```

The overlap check must be repeated immediately before any implementation PR.

## Path Boundary

`src/git_tape_ledger.rs` is the older TDMA `ImmutableTapeLedger`
implementation:

```text
src/git_tape_ledger.rs:1    TRACE_MATRIX FC1a-substrate_seam + FC3-replay
src/git_tape_ledger.rs:24   uses crate::ledger::ImmutableTapeLedger
src/git_tape_ledger.rs:42   refs/tdma/verified_head
src/git_tape_ledger.rs:48   refs/tdma/ledger_tail
src/git_tape_ledger.rs:378  impl ImmutableTapeLedger for GitTapeLedger
src/git_tape_ledger.rs:417  commit(&mut self, req: CommitRequest) -> TapeNode
```

Its trait shape is not fail-closed for ref update errors:

```text
src/ledger.rs:575  pub trait ImmutableTapeLedger
src/ledger.rs:577  fn set_verified_head(&mut self, new_head: String)
src/ledger.rs:579  fn commit(&mut self, req: CommitRequest) -> TapeNode
```

Current code includes silent or ignored error surfaces that are incompatible
with A04's "do not swallow ref movement errors" rule unless the API is changed
or wrapped:

```text
src/git_tape_ledger.rs:399  let _ = r.delete()
src/git_tape_ledger.rs:406  invalid OID returns silently
src/git_tape_ledger.rs:409  let _ = r.set_target(...)
src/git_tape_ledger.rs:411  let _ = repo.reference(...)
src/git_tape_ledger.rs:497  let _ = r.set_target(...)
src/git_tape_ledger.rs:505  let _ = r.set_target(...)
src/git_tape_ledger.rs:544  let _ = walk_commits(...)
src/git_tape_ledger.rs:558  skip malformed commits silently
```

The current OS ChainTape/L4 writer is in
`src/bottom_white/ledger/transition_ledger.rs`:

```text
src/bottom_white/ledger/transition_ledger.rs:1
  L4 Transition Ledger
src/bottom_white/ledger/transition_ledger.rs:325
  LedgerWriter trait
src/bottom_white/ledger/transition_ledger.rs:331
  commit(&mut self, entry) -> Result<Hash, LedgerWriterError>
src/bottom_white/ledger/transition_ledger.rs:1045
  Git2LedgerWriter - git2-rs commit chain
src/bottom_white/ledger/transition_ledger.rs:1078
  refs/transitions/main compatibility ref
src/bottom_white/ledger/transition_ledger.rs:1098
  refs/chaintape/l4 canonical L4 head ref
src/bottom_white/ledger/transition_ledger.rs:1100
  refs/chaintape/l4e canonical L4.E head ref
src/bottom_white/ledger/transition_ledger.rs:1102
  refs/chaintape/cas canonical CAS root ref
src/bottom_white/ledger/transition_ledger.rs:1316
  Git2LedgerWriter::commit(...)
src/bottom_white/ledger/transition_ledger.rs:1390
  C1 alias refs/transitions/main update returns Result
```

Existing ChainTape tests already cover some A04-adjacent properties:

```text
src/bottom_white/ledger/transition_ledger.rs:1683
  git2_writer_append_and_read_back
src/bottom_white/ledger/transition_ledger.rs:1711
  git2_writer_rejects_logical_t_gap
src/bottom_white/ledger/transition_ledger.rs:1735
  git2_writer_reopen_recovers_chain
tests/constitution_g1_resume.rs:126
  resume sets next logical_t from chain length and appends N+1
tests/tb_6_verify_chaintape.rs:38
  verify_chaintape replay passes accepted L4 + rejected L4.E indicators
tests/dual_substrate_disjointness.rs:1
  TDMA GitTapeLedger and runtime ChainTape must remain disjoint
```

Outer single-writer / preflight surfaces exist, but they do not replace
canonical-ref fail-closed semantics:

```text
src/runtime/chain_tape_lease.rs:3
  ChainTapeLease guards writer ownership
src/runtime/chain_tape_lease.rs:192
  acquire_lease checks expected_head_t_hex
src/runtime/batch_orchestrator.rs:108
  batch boundary acquires ChainTapeLease
src/runtime/resume_preflight.rs:250
  resume preflight checks expected head
```

Known A04 hardening gap:

```text
src/bottom_white/ledger/rejection_evidence.rs:133
  advances refs/chaintape/l4e after L4.E append
src/bottom_white/ledger/rejection_evidence.rs:148
  let _ = advance_l4e_ref_for_record(...)
src/bottom_white/ledger/rejection_evidence.rs:154
  documents the update as best-effort and non-propagating
```

CAS ref handling is stronger and should be preserved:

```text
src/bottom_white/cas/store.rs:453
  append_cas_commit(...)
src/bottom_white/cas/git_chain.rs:280
  validate_cas_chain_head_oid(...)
src/bottom_white/cas/git_chain.rs:296
  validate_cas_chain_head_update(...)
```

## Risk Classification

Risk floor: Class 3, because A04 governs the physical ledger used by replay
and evidence.

Class 4 candidate triggers:

- changing sequencer admission
- changing typed tx schema or discriminants
- changing canonical signing payloads
- changing system signature semantics
- changing which ref is canonical for ChainTape L4/L4.E/CAS
- changing trust-root-pinned files without the corresponding ratified rehash

## Recommended Path

Do not treat `src/git_tape_ledger.rs` as the Agentic OS L2 writer unless the
architect explicitly chooses a TDMA-compatibility atom.

Preferred A04 implementation authority:

```text
src/bottom_white/ledger/transition_ledger.rs
src/bottom_white/ledger/rejection_evidence.rs
src/bottom_white/cas/git_chain.rs
src/bottom_white/cas/store.rs
src/runtime/chain_tape_lease.rs
src/runtime/resume_preflight.rs
tests/tc_chaintape_ledger_hardening.rs
tests/constitution_g1_resume.rs
tests/tb_6_verify_chaintape.rs
tests/dual_substrate_disjointness.rs
```

Allowed compatibility-only work:

```text
src/git_tape_ledger.rs
tests/git_tape_ledger_roundtrip.rs
```

Compatibility work must be explicitly labeled as TDMA legacy compatibility and
must not claim to satisfy OS ChainTape/L4 A04 by itself.

## Ratification Options

Valid implementation decision phrases:

```text
APPROVE-A04-SECTION8-CHAINTAPE-L4
APPROVE-A04-SECTION8-TDMA-COMPAT-ONLY
APPROVE-A04-SECTION8-DEFER
REJECT-A04-RUNTIME-FOR-NOW
```

Meanings:

- `CHAINTAPE-L4`: implement A04 against `Git2LedgerWriter` /
  `LedgerWriter` and the ChainTape refs.
- `TDMA-COMPAT-ONLY`: harden `src/git_tape_ledger.rs` as a legacy
  compatibility atom, but do not count it as OS L2 completion.
- `DEFER`: wait for broader TC operationalization review before touching
  runtime ledger code.
- `REJECT`: keep A04 blocked; continue with non-dependent docs or lower-risk
  atoms only.

## Atomized A04 Tasks

### A04.0 Preflight Lock

Description:
Record that the parent-plan allowed paths do not match the current OS L2
writer and that A04 implementation authority must be ratified.

Acceptance:

```bash
for f in \
  handover/directives/2026-06-05_A04_GITTAPE_PHYSICAL_LEDGER_PREFLIGHT.md \
  handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md
do
  git diff --no-index --check /dev/null "$f" || true
done
```

Expected:

```text
no whitespace errors.
preflight states that OS L2 work targets ChainTape/L4, not TDMA legacy tape.
```

### A04.1 Contract Correction

Description:
Before runtime implementation, revise or supersede A04's allowed paths so the
write set matches the selected ratification option.

Acceptance:

```text
Implementation PR body states exactly one authority:
CHAINTAPE-L4 or TDMA-COMPAT-ONLY.
No PR claims OS L2 completion from TDMA-only changes.
```

### A04.2 ChainTape-L4 Hardening

Description:
If `CHAINTAPE-L4` is approved, add hardening tests and minimal runtime fixes for
the existing `Git2LedgerWriter` path.

Required tests:

- append creates one canonical commit on `refs/chaintape/l4`
- `refs/transitions/main` remains a repairable alias, not a second source of
  truth
- reopen resumes at `N+1`
- stale or divergent ref movement fails closed or repairs from canonical C2
- logical_t gap fails without advancing head
- L4.E ref movement failure propagates or leaves canonical refs unchanged
- CAS ref movement validation remains fail-closed
- TDMA and runtime ChainTape object pools remain disjoint
- generated repo passes `git fsck --full`

Acceptance:

```bash
cargo test --test tc_chaintape_ledger_hardening --no-fail-fast -- --test-threads=1
cargo test --test constitution_g1_resume --no-fail-fast -- --test-threads=1
cargo test --test tb_6_verify_chaintape --no-fail-fast -- --test-threads=1
cargo test --test dual_substrate_disjointness --no-fail-fast
cargo test --test constitution_matrix_drift --no-fail-fast
git -C <generated_repo> fsck --full
git diff --check
```

Expected:

```text
single canonical L4 head.
no duplicate logical_t.
no swallowed ref movement errors on canonical refs.
L4.E no longer best-effort swallows ref movement failures.
reopen continues at N+1.
replay verifier still reconstructs accepted and rejected paths.
```

### A04.3 TDMA Compatibility Hardening

Description:
If `TDMA-COMPAT-ONLY` is approved, harden legacy `GitTapeLedger` without
claiming OS ChainTape completion.

Required tests:

- invalid verified_head OID fails through a Result-returning helper
- ref update failure is observable
- malformed commits do not silently disappear from integrity checks
- existing roundtrip tests remain green

Acceptance:

```bash
cargo test --test git_tape_ledger_roundtrip --no-fail-fast -- --test-threads=1
cargo test --test git_tape_ledger_head_and_belief --no-fail-fast -- --test-threads=1
git diff --check
```

Expected:

```text
legacy TDMA tape is harder to corrupt, but PR does not claim A04 OS L2 complete.
```

## Clean-Context Audit Prompt

Use only after implementation evidence exists.

```text
Role: clean-context constitutional audit witness.
Task: audit A04 GitTape Physical Ledger and Single Writer.
Risk: Class 3 floor; Class 4 if sequencer admission, typed tx schema,
canonical signing payload, or canonical ChainTape refs changed.
Touched FC nodes: FC1-N1, FC1-N3, FC1-N13, FC1-N14, FC1-N15, FC2 replay.
Inputs: parent plan A04, this preflight doc, current diff, exact command output.
Verdict domain:
NO-VIOLATION
VIOLATION-FOUND <constitutional-clause> <file>:<line>
RECONSTRUCTION-FAILURE <which-tape-or-cas-path-cannot-be-reconstructed>
SECOND-SOURCE-DRIFT <which-derived-view-is-usurping-ground-truth>
```

## Final Pre-Implementation Gate

A04 runtime implementation may start only when all are true:

- current open PR overlap has been checked again
- relevant dirty paths are understood
- a valid A04 ratification phrase exists
- implementation authority is exactly one of ChainTape-L4 or TDMA-compat-only
- the first implementation step is a failing hardening test
- no sequencer / typed-tx / signing-payload change is hidden inside a Class 3
  label
