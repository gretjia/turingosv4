# PCP Corpus Phase-2 — MiniF2F-v2 Misalignment (Real-World Adversarial)

**Authority**: Gemini R1 audit Q8 forward-binding (`OBS_GEMINI_C_LAND_R1_Q8_FORWARD_BINDING_2026-05-07.md`) + TB-18B charter SG-18B.9.

**Predecessor**: `cases/pcp_corpus/` (Constitution Landing First synthetic phase-1; 9 designed mutations).

**Mode**: Constitutional Harness Engineering (real public problem; NOT synthesis per `feedback_real_problems_not_designed`).

---

## Why phase-2?

Phase-1 (`cases/pcp_corpus/`) covered the 9 mutation classes via *synthesized* Lean fixtures (e.g. `theorem pcp_corpus_05_wrong_theorem_name (n : Nat) : n + 0 = n := ...`). Per `feedback_real_problems_not_designed`: "real public problems preferred; synthesis forbidden". Gemini Q8 specifically required MiniF2F-v2 misalignment phase-2 as a forward step before TB-18B M2 ship.

Phase-2 derives all 9 mutation classes from a **single real MiniF2F problem** (`mathd_algebra_107`), so each adversarial fixture is a benign-looking variant of a real-world theorem rather than a contrived shape. This stresses the predicate / oracle layer the same way real-LLM mistakes would.

## Base problem

`mathd_algebra_107` (MiniF2F Test set):

```lean
theorem mathd_algebra_107
  (x y : ℝ)
  (h₀ : x^2 + 8 * x + y^2 - 6 * y = 0) :
  (x + 4)^2 + (y-3)^2 = 5^2 := by
  nlinarith
```

The canonical valid proof routes to `L4 accepted`; each mutation routes to `L4.E` with the typed `RejectionClass` documented in `MANIFEST.json`.

## 9 mutation classes (per CLAUDE.md §4.2 G-012)

| # | Class | Mutation | Routes to |
|---|-------|----------|-----------|
| 01 | valid_proof | (none — canonical) | L4 accepted |
| 02 | mutated_invalid_proof | `nlinarith` → `linarith` (insufficient power for nonlinear goal) | L4.E `LeanFailed` |
| 03 | sorry_insertion | `nlinarith` → `sorry` | L4.E `SorryBlocked` |
| 04 | type_mismatch | conclusion `(x + 4)^2` → `(x + 4 : ℕ)^2` (forces ℕ↔ℝ mismatch) | L4.E `LeanFailed` |
| 05 | wrong_theorem_name | invoke nonexistent `Mathlib.Tactic.Nlinarith2` | L4.E `LeanFailed` |
| 06 | off_by_one_arithmetic | conclusion `5^2` → `6^2` (true statement broken) | L4.E `LeanFailed` |
| 07 | irrelevant_theorem | apply `Nat.add_comm` where it cannot match | L4.E `LeanFailed` |
| 08 | partial_then_final_invalid | `ring_nf; nlinarith` followed by garbage that should fail | L4.E `LeanFailed` |
| 09 | parse_invalid | unbalanced parentheses; Lean parser rejects | L4.E `ParseFailed` |

## Constitutional invariant

Per CLAUDE.md §4.2 G-012:
- Valid proofs MUST pass.
- Mutated invalid proofs MUST fail.
- Invalid proofs MUST NOT enter L4 accepted.
- Invalid proofs MUST enter L4.E or anchored EvidenceCapsule.

Phase-2 enforces this on a real MiniF2F problem family rather than synthesized one-line theorems.

## Cross-references

- Phase-1 corpus: `cases/pcp_corpus/`
- Gemini Q8 OBS: `handover/alignment/OBS_GEMINI_C_LAND_R1_Q8_FORWARD_BINDING_2026-05-07.md`
- TB-18B charter SG-18B.9: `handover/tracer_bullets/TB-18B_charter_2026-05-07.md`
- CLAUDE.md §4.2 G-012 PCP soundness: `CLAUDE.md`
- `feedback_real_problems_not_designed` rationale
