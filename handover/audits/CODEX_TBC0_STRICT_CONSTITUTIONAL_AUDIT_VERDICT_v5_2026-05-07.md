# Codex TB-C0 Strict Constitutional Re-Audit Verdict v5

Date: 2026-05-07  
Auditor: Codex re-auditor v5  
Target commit: `8f3a82b` (`TB-C0 round 8 — FC3-INV1 capsule integrity test + Art. V.3 amendment-log test`)  
Scope: round-8 closure of the two v4 §3 forward-bound residues.

## §1 Aggregate Verdict

**PASS.**

Round 8 correctly closes the two forward-bound items that v4 accepted as non-blocking residue:

1. **FC3-INV1 capsule integrity** is now exercised by `tests/constitution_fc3_inv1_capsule_integrity_regen.rs` on real TB-C0 batch CAS artifacts.
2. **Art. V.3 amendment-log executable coverage** is now exercised by `tests/constitution_art_v3_amendment_log.rs`.

Conservative ranking applied: no VETO-class constitutional violation found; no remaining CHALLENGE-class blocker found in the round-8 scope.

**TB-C0 IS READY FOR ARCHITECT §8 SIGN-OFF.**

Round 8 also defensibly promotes additional closure conditions beyond v4: closure #2 and closure #6 move from AMBER to GREEN.

## §2 Q-V5 Findings

### Q-V5-1 — FC3-INV1 capsule integrity test correctness

**Verdict: PASS.**

The four tests in `tests/constitution_fc3_inv1_capsule_integrity_regen.rs` are correct for the constitutional question they target:

- `fc3_inv1_capsule_id_is_content_addressable_p08` verifies the on-disk EvidenceCapsule CAS object is content-addressable: `Cid::from_content(bytes) == stored_cid`.
- `fc3_inv1_capsule_attempt_count_matches_at_count_p08` recomputes `attempt_count` by walking real `AttemptTelemetry` CAS objects and compares it to the restored capsule.
- `fc3_inv1_capsule_outcome_counts_match_at_walk_p08` recomputes capsule outcome counters from `AttemptTelemetry.outcome`.
- `fc3_inv1_capsule_integrity_secondary_problems` repeats the integrity checks on P05 and P07.

This is not a mere capsule-presence test. It performs a real CAS walk and verifies that the capsule is a derived view over the underlying AttemptTelemetry objects. The primary case is P08 `aime_1983_p1`, the richest real TB-C0 capsule case; P05 and P07 are real secondary cases. No synthetic data and no new LLM compute are introduced, satisfying `feedback_real_problems_not_designed`.

Outcome coverage is constitutionally acceptable. The capsule schema has fields for `LeanFail`, `SorryBlock`, `ParseFail`, and `PartialAccepted`, and all four are recomputed and asserted. `LeanPass` is correctly absent from capsule counters because omega-success does not trigger EvidenceCapsule emission under architect §6.1. `Aborted` is excluded per FR-18R.4 v2. `LlmErr` is an AttemptOutcome enum variant but has no dedicated EvidenceCapsule counter in the current schema; round 8 does not regress that surface.

### Q-V5-2 — Art. V.3 amendment-log test sufficiency

**Verdict: PASS.**

The six tests in `tests/constitution_art_v3_amendment_log.rs` satisfy directive line 430 ("no test = RED"). They are executable, concrete assertions, not documentation coverage and not `assert!(true)` placeholders:

- §5.3 exists and yields parseable rows.
- Every row has populated `date | trigger | section | summary`.
- Every trigger names `人类架构师` or `human architect`.
- Dates are strict `YYYY-MM-DD`.
- `constitution.md` SHA-256 matches the `genesis_payload.toml` trust-root manifest.
- Historical dates `2026-04-25` and `2026-04-26` remain recorded.

The three named attack vectors are covered:

- Silent `constitution.md` edit without manifest rehash: caught by V3.5.
- Retroactive amendment deletion: caught by V3.6 for the locked historical dates.
- ArchitectAI-without-human amendment: caught by V3.3.

### Q-V5-3 — Closure condition consistency

**Verdict: PASS.**

Closure #2 GREEN is defensible. The v4 blocker was the Art. V.3 RED row ("NEW test required"). Round 8 adds a real executable test file with six failing-capable assertions and updates the matrix row accordingly.

Closure #6 GREEN is defensible. The prior AMBER residue was capsule presence without integrity. Round 8 adds a standalone real-evidence integrity test that verifies CAS content-addressability plus AttemptTelemetry-derived capsule counts across the three TB-C0 capsule-producing problems.

The `scripts/run_constitution_gates.sh` extension is also consistent: it adds the two round-8 test files to the gate list.

### Q-V5-4 — Aggregate verdict

**PASS.**

No VETO or CHALLENGE remains in the narrow v5 scope.

## §3 Strengthening Recommendations

Non-blocking hardening recommendations:

1. Add a future byte-for-byte capsule regeneration test that reconstructs the full `EvidenceCapsule` object from chain/CAS inputs and asserts the resulting zero-identity canonical bytes hash to the anchored CID.
2. Add an explicit L4 anchor assertion for each tested problem: locate the `TerminalSummaryTx.evidence_capsule_cid`, then assert it equals the tested EvidenceCapsule CID.

These are hardening items, not blockers. Round 8 already closes the accepted v4 residues at the constitutional gate level.

## §4 §8 Readiness Final

**TB-C0 IS READY FOR ARCHITECT §8 SIGN-OFF.**

Round 8 promotes additional closure conditions to GREEN beyond v4:

- Closure #2: every critical row has a test.
- Closure #6: Markov / EvidenceCapsule passes FC3.

## §5 Cross-Refs

- Round-8 commit: `8f3a82b`
- FC3-INV1 test: `tests/constitution_fc3_inv1_capsule_integrity_regen.rs`
- Art. V.3 test: `tests/constitution_art_v3_amendment_log.rs`
- Gate runner: `scripts/run_constitution_gates.sh`
- Matrix updates: `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
- Prior PASS baseline: `handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_v4_2026-05-07.md`
- Prior witness residue: `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md`

Verification run by Codex v5:

```text
cargo test --test constitution_fc3_inv1_capsule_integrity_regen --test constitution_art_v3_amendment_log -- --test-threads=1

constitution_art_v3_amendment_log: 6 passed; 0 failed
constitution_fc3_inv1_capsule_integrity_regen: 4 passed; 0 failed
```
