# A03 Boot Trust Root Manifest Preflight And Section-8 Request

Date: 2026-06-05

Parent plan:
`handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`

Atom: A03. Boot Trust Root Manifest Gate

Document role: Class 0 preflight and ratification request. This document does
not authorize runtime, trust-root, genesis, or build-script edits by itself.

## Decision

A03 must not proceed as a normal implementation atom until the implementation
surface is ratified. The plan-listed paths do not match the current repository
shape, and the actual boot trust-root surface is a trust-root / constitution
authority surface under `AGENTS.md`.

Safe work now:

- docs-only preflight
- test-design notes
- clean-context audit prompt preparation

Blocked until explicit per-atom authorization:

- editing `src/boot.rs`
- changing trust-root manifest semantics
- editing `genesis_payload.toml`
- editing `build.rs` for trust-root or boot authority
- moving boot authority into a new module

## Hard Blockers

- A03-HB1: Runtime trust-root, genesis, build-script, or boot-call edits are
  blocked unless the PR cites one valid A03 §8 ratification phrase from this
  document.
- A03-HB2: A03 cannot claim completion by creating new wrapper paths while the
  live authority remains `src/boot.rs::verify_trust_root` and `src/main.rs`
  boot wiring untested.
- A03-HB3: No env var, test-only flag, panic catch, or manifest fallback may
  bypass a trust-root mismatch.

## Current-State Facts

Planned A03 allowed paths from the parent plan:

```text
src/bin/turingos/cmd_boot.rs
src/runtime/boot_trust_root_manifest.rs
tests/constitution_tc_boot_trust_root_manifest.rs
build.rs
genesis_payload.toml
```

Existence check:

```text
MISSING src/bin/turingos/cmd_boot.rs
MISSING src/runtime/boot_trust_root_manifest.rs
MISSING tests/constitution_tc_boot_trust_root_manifest.rs
EXISTS build.rs
EXISTS genesis_payload.toml
EXISTS src/boot.rs
EXISTS tests/fc_alignment_conformance.rs
EXISTS tests/constitution_fc2_boot.rs
EXISTS tests/constitution_art_v3_amendment_log.rs
```

Relevant current dirty paths:

```text
build.rs
genesis_payload.toml
tests/fc_alignment_conformance.rs
```

Open PR overlap check:

```text
#280 AUDIT ONLY / DO NOT MERGE touches build.rs, genesis_payload.toml,
     TC_002_BOOT_TRUST_ROOT_MANIFEST.md, and related TC packet docs.
#283 AUDIT ONLY / DO NOT MERGE does not define the A03 implementation path,
     but it overlaps general governance docs such as AGENTS.md / CLAUDE.md.
```

The overlap does not block a docs-only preflight, but it blocks blind runtime
implementation dispatch. Any A03 implementation PR must explain whether it
depends on, rejects, or supersedes #280's TC-002 snapshot material.

## Existing Trust-Root Surfaces

The current boot entry point calls the verifier directly:

```text
src/main.rs:12      main()
src/main.rs:14      turingosv4::boot::verify_trust_root(&repo_root)
src/main.rs:15      panic!("TRUST_ROOT_TAMPERED: {e}")
```

The current trust-root verifier already exists in `src/boot.rs`:

```text
src/boot.rs:36       TrustRootError taxonomy
src/boot.rs:97       verify_trust_root(repo_root)
src/boot.rs:100      parse [trust_root] from genesis_payload.toml
src/boot.rs:107      verify_constitution_root_section(...)
src/boot.rs:125      recurse into child MANIFEST.sha256 entries
src/boot.rs:321      parse_trust_root_section(...)
src/boot.rs:440      intact repo verifier test
src/boot.rs:467      tamper detection test
src/boot.rs:513      child manifest tamper test
```

The root manifest currently pins the boot and restricted authority surface:

```text
genesis_payload.toml:134  "src/main.rs"
genesis_payload.toml:145  "src/boot.rs"
genesis_payload.toml:158  "constitution.md"
genesis_payload.toml:280  "src/state/sequencer.rs"
genesis_payload.toml:281  "src/state/typed_tx.rs"
```

Existing constitutional witnesses:

```text
tests/fc_alignment_conformance.rs:23   imports verify_trust_root
tests/fc_alignment_conformance.rs:167  FC3-N34 intact repo trust-root witness
tests/fc_alignment_conformance.rs:183  parse_trust_root_section witness
tests/fc_alignment_conformance.rs:235  manifest size witness
tests/constitution_fc2_boot.rs:197     boot.rs must contain trust-root logic
tests/constitution_art_v3_amendment_log.rs:169
  constitution.md hash must match genesis_payload.toml trust_root entry
```

Related boot-time predicate / tick witnesses:

```text
src/runtime/mod.rs:697                    build_chaintape_sequencer(...)
src/runtime/mod.rs:700                    starts from QState::genesis()
src/runtime/mod.rs:826                    loads boot predicate registry
src/runtime/predicate_registry_loader.rs:8
  load_replay_registry uses BootPredicateManifest::v8_production()
src/state/sequencer.rs:4614              PredicateBindingActivate system tx path
src/state/sequencer.rs:4672              MapReduceTick system tx path
tests/constitution_predicate_gate.rs:104 predicate_pass_required_for_l4
tests/constitution_flowchart_livenow.rs:275
  FC2 fresh boot / replay / resume liveness area
```

This means the planned new files may be useful as a wrapper or focused test
surface, but they cannot be treated as the only authority.

## Risk Classification

Risk floor: Class 3, because A03 is about boot trust-root integrity and
constitutional evidence.

Class 4 candidate triggers:

- changes to any trust-root or constitution / flowchart authority surface
- changes to `genesis_payload.toml` as a genesis authority artifact
- changes to canonical signing payload semantics
- changes that alter which boot check is authoritative
- changes that move or replace `src/boot.rs::verify_trust_root`
- changes that move or bypass the `src/main.rs` boot call site
- changes to predicate-binding or MapReduceTick boot authority in restricted
  sequencer / typed-tx surfaces

Under `AGENTS.md`, Class 4 requires explicit per-atom section-8
architect/user ratification before implementation or ship. Short replies such
as `go`, `ok`, `continue`, or `can` are not sufficient.

## Recommended Minimal Implementation Shape After Ratification

Do not start by inventing a second boot authority.

Preferred slice:

1. Keep `src/boot.rs::verify_trust_root` as the existing authoritative
   verifier unless the ratification explicitly says to migrate it.
2. Add the planned integration test file
   `tests/constitution_tc_boot_trust_root_manifest.rs`.
3. The new test should exercise the current public boot API first.
4. Only add `src/runtime/boot_trust_root_manifest.rs` if it is a thin,
   non-authoritative wrapper or data-shape helper around the existing verifier.
5. Only edit `src/main.rs` if the ratification explicitly covers the boot
   call-site contract.
6. Only edit `build.rs` or `genesis_payload.toml` when the PR body explains
   the exact reason, prior hash/authority, and replacement authority.

Forbidden shortcuts:

- no env-var bypass
- no test-only bypass
- no panic-catching bypass
- no second trust-root ledger
- no broad rehash of `genesis_payload.toml` to make tests green
- no replacement of `src/boot.rs` without explicit Class 4 ratification

## Atomized A03 Tasks

### A03.0 Preflight Lock

Description:
Record the actual implementation surface and the open-PR overlap before any
runtime work.

Instructions:

- keep this as docs-only
- do not touch `src/`, `build.rs`, or `genesis_payload.toml`
- cite the current existing verifier and tests

Acceptance:

```bash
git diff --check -- \
  handover/directives/2026-06-05_A03_BOOT_TRUST_ROOT_MANIFEST_PREFLIGHT_AND_SECTION8_REQUEST.md \
  handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md
```

Expected:

```text
no whitespace errors.
preflight states that A03 runtime edits are blocked pending ratification.
```

### A03.1 Ratification Decision

Description:
Choose the implementation authority before runtime work begins.

Valid ratification options:

```text
APPROVE-A03-SECTION8-KEEP-SRC-BOOT
APPROVE-A03-SECTION8-WRAPPER-MODULE
APPROVE-A03-SECTION8-DEFER-TO-TC002
REJECT-A03-RUNTIME-FOR-NOW
```

Meanings:

- `KEEP-SRC-BOOT`: keep `src/boot.rs` authoritative; add focused tests only.
- `WRAPPER-MODULE`: add `src/runtime/boot_trust_root_manifest.rs` as a thin
  wrapper/helper, while `src/boot.rs` remains authoritative.
- `DEFER-TO-TC002`: do not implement from this plan until #280/TC-002 is
  reviewed and either extracted or rejected.
- `REJECT`: keep A03 blocked; continue with non-dependent atoms only.

Acceptance:

```text
The chosen ratification phrase is present in an architect/user message or
directive and is referenced by the implementation PR.
```

### A03.2 Failing Tests First

Description:
Add focused tests before implementation changes.

Test cases:

- valid manifest passes
- file hash mismatch fails closed
- constitution hash mismatch fails closed
- child manifest payload mismatch fails closed
- missing `[trust_root]` fails closed
- missing `[constitution_root]` fails closed
- env var cannot bypass failure

Acceptance:

```bash
cargo test --test constitution_tc_boot_trust_root_manifest --no-fail-fast -- --test-threads=1
```

Expected before implementation:

```text
new tests fail for the missing behavior they introduce, not for compile errors
or unresolved imports.
```

### A03.3 Minimal Runtime Wiring

Description:
Implement only the ratified authority shape.

Acceptance:

```bash
cargo test --test constitution_tc_boot_trust_root_manifest --no-fail-fast -- --test-threads=1
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --exact
cargo test --test fc_alignment_conformance fc3_n34_readonly_guard_verify_trust_root_intact_repo -- --exact
cargo test --test constitution_art_v3_amendment_log v3_constitution_hash_matches_trust_root_manifest -- --exact
cargo test --test constitution_predicate_gate predicate_pass_required_for_l4 -- --exact
cargo test --test constitution_flowchart_livenow --no-fail-fast -- --test-threads=1
cargo test --test fc_alignment_conformance fc3_n34 --no-fail-fast
cargo test --test constitution_fc2_boot --no-fail-fast
bash scripts/run_constitution_gates.sh
git diff --check
grep -RInE 'ALLOW|BYPASS|SKIP|panic::catch_unwind' \
  src/main.rs src/boot.rs src/bin/turingos/cmd_boot.rs \
  src/runtime/boot_trust_root_manifest.rs \
  tests/constitution_tc_boot_trust_root_manifest.rs || true
```

Expected:

```text
valid manifest passes.
all mismatch tests fail closed.
existing FC2/FC3 witnesses still pass.
no bypass by env var or test-only flag.
no second trust-root authority.
```

## Clean-Context Audit Prompt

Use this only after implementation evidence exists.

```text
Role: clean-context constitutional audit witness.
Task: audit A03 Boot Trust Root Manifest Gate.
Risk: Class 3 floor; Class 4 if trust-root / constitution / genesis authority
changed.
Touched FC nodes: FC2-N16, FC2-N18, FC2-N19, FC2-N21, FC3-N29, FC3-N34.
Inputs: current diff, parent plan A03, this preflight doc, exact command output.
Forbidden verdict basis: style, taste, coverage vibes, performance preference.
Verdict domain:
NO-VIOLATION
VIOLATION-FOUND <constitutional-clause> <file>:<line>
RECONSTRUCTION-FAILURE <which-tape-or-cas-path-cannot-be-reconstructed>
SECOND-SOURCE-DRIFT <which-derived-view-is-usurping-ground-truth>
```

## Final Pre-Implementation Gate

A03 runtime implementation may start only when all are true:

- this preflight doc exists
- current open PR overlap has been checked again
- relevant dirty paths are understood
- an explicit A03 section-8 ratification phrase exists
- implementation write set is narrowed to ratified paths
- the first implementation step is a failing test or a compile-failing
  witness, not production wiring
