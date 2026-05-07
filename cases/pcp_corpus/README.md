# PCP Adversarial Corpus

**Authority**: HARNESS.md §3 G-012 (architect ruling 2026-05-07).
**Decision**: "Lean tactic-mutation adversarial corpus first; MiniF2F-v2 misalignment second."

## Purpose

Hard-code the 9 mutation classes the architect requires the harness to reject.
Each fixture is a Lean snippet plus the expected `AttemptOutcome` /
`RejectionClass` / `LeanVerdictKind` it should produce in TuringOS.

The synthetic gate `tests/constitution_pcp_corpus.rs` exercises the routing
table on this corpus. Real-Lean replay is a forward step (G-012 phase 2:
MiniF2F-v2 misalignment); the synthetic gate is the negative-control floor.

## Corpus index

| ID | Class | Lean fixture | Expected AttemptOutcome | Expected RejectionClass | Routes to |
|----|-------|--------------|--------------------------|--------------------------|-----------|
| 01 | valid proof | `01_valid/proof.lean` | `LeanPass` | n/a | L4 accepted |
| 02 | mutated invalid (general) | `02_mutated_invalid/proof.lean` | `LeanFail` | `LeanFailed=6` | L4.E |
| 03 | sorry insertion | `03_sorry_insertion/proof.lean` | `SorryBlock` | `SorryBlocked=8` | L4.E |
| 04 | type mismatch | `04_type_mismatch/proof.lean` | `LeanFail` | `LeanFailed=6` | L4.E |
| 05 | wrong theorem name | `05_wrong_theorem_name/proof.lean` | `LeanFail` | `LeanFailed=6` | L4.E |
| 06 | off-by-one arithmetic | `06_off_by_one_arith/proof.lean` | `LeanFail` | `LeanFailed=6` | L4.E |
| 07 | irrelevant theorem | `07_irrelevant_theorem/proof.lean` | `LeanFail` | `LeanFailed=6` | L4.E |
| 08 | partial tactic, final invalid | `08_partial_then_final_invalid/proof.lean` | `PartialAccepted` then `LeanFail` | `LeanFailed=6` (final) | CAS-only (partial) + L4.E (final) |
| 09 | parse-invalid output | `09_parse_invalid/proof.lean` | `ParseFail` | `ParseFailed=7` | L4.E |

## Manifest

`MANIFEST.json` enumerates each fixture with the expected verdict tuple. The
synthetic gate reads the manifest to drive its assertions; adding a new
mutation class is "drop fixture + append manifest row".

## Real-Lean replay

For the forward step:

```
cd cases/pcp_corpus/<NN>_<class>
lake env lean proof.lean
echo $?
```

The expected exit / stderr signature is captured in `expected.txt` per
fixture. Phase-2 MiniF2F-v2 misalignment will use these signatures as the
golden baseline.
