# Gemini TB-15 Ship Audit — Lamarckian Autopsy + Markov EvidenceCapsule (Class 2 retroactive dual audit)
**Round**: R1
**Date**: 2026-05-04
**Test baseline**: cargo test --workspace = 878 PASS / 0 FAILED / 150 ignored (TB-15 ship commit 2337381)
**Halt-trigger battery**: 6/6 GREEN (tests/tb_15_halt_triggers.rs)
**Trust Root**: GREEN (6 rehashes propagated correctly)
**Original audit envelope**: Class 2 self-audit per charter §4 (no Codex/Gemini at ship)
**Retroactive dual audit**: requested by user 2026-05-04 to verify Class 2 envelope held
**Elapsed**: 68.2s
**Prompt size**: 782,805 chars
**Audit mode**: architectural strategic (Codex covers impl-paranoid in parallel)

---

This audit has been conducted per the mandate for a retroactive dual audit on Gemini TB-15. As the architectural strategic reviewer, my focus is on the structural integrity, constitutional conformance, and strategic soundness of the shipped artifacts, using the provided source code and documentation as evidence.

The overall architecture of TB-15 is sound in its primary goals: establishing a private autopsy mechanism and a forward-looking Markov summary to prevent context poisoning. The privacy boundaries are structurally well-enforced, and the separation of powers is respected. However, the audit has identified one blocking (VETO) issue related to replay determinism and one non-blocking (CHALLENGE) issue regarding a deviation from the literal specification.

---

## Architect-mandated audit questions (CR-15.x conformance + cross-cutting)

### 1. CR-15.1 + CR-15.2 architectural soundness
**Verdict**: PASS
**Finding**: The privacy boundary is structurally enforced, not merely by convention. The new field `agent_autopsies_t: AutopsyIndex` is correctly placed on `EconomicState` (`src/state/q_state.rs:278`) and is absent from the `AgentVisibleProjection` struct (`src/state/q_state.rs:131-135`). The enforcement is made structural by the halt-trigger test `raw_logs_not_in_general_read_view` (`tests/tb_15_halt_triggers.rs:18-56`), which performs a file-scan on `q_state.rs` to assert that the `AgentVisibleProjection` struct body does not contain forbidden tokens like `agent_autopsies_t` or `AgentAutopsyCapsule`. Any future attempt to add these fields would cause this test to fail, thus acting as a strong structural gate. No current code path projects these bytes to any agent-readable surface. The enforcement is provably tight.

### 2. CR-15.3 + CR-15.4 (three-power separation)
**Verdict**: PASS
**Finding**: The architectural separation is tight. The `suggested_policy_patch: Option<Cid>` field in `AgentAutopsyCapsule` (`src/runtime/autopsy_capsule.rs:163`) is treated as an opaque pointer. The writer function `write_autopsy_capsule` (`src/runtime/autopsy_capsule.rs:293`) accepts this field but has no logic to read or apply it. Crucially, the halt-trigger test `autopsy_does_not_mutate_predicates` (`tests/tb_15_halt_triggers.rs:90-120`) performs a file-scan of `src/runtime/autopsy_capsule.rs` to ensure no mutable references to registries (`&mut PredicateRegistry`, etc.) or mutator methods (`.register_predicate(`) exist. This structurally prevents the autopsy module from becoming a backdoor for automatic policy changes. The field is strictly a write-only suggestion awaiting a future, separate MetaTape consumer as intended by the spec.

### 3. CR-15.5 (capsules are evidence compression, not hidden source of truth)
**Verdict**: PASS
**Finding**: All fields in the shipped `MarkovEvidenceCapsule` (`src/runtime/markov_capsule.rs:39-85`) are derivable from on-disk artifacts.
- `capsule_id`, `sha256`: Derived from a hash of the other fields.
- `previous_capsule_cid`: Read from `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt`.
- `constitution_hash`: Derived via `sha256_of_file("constitution.md")` (`src/runtime/markov_capsule.rs:288`).
- `l4_root`, `l4e_root`, `cas_root`: Shipped as `Hash::ZERO` placeholders (`handover/evidence/tb_15_markov_capsule_2026-05-03/MARKOV_TB-15_2026-05-03.json`). This is an honest, documented v0 limitation, with the architectural intent that they will be derived from chain heads.
- `typical_errors`: Derived by `cluster_autopsies` over CAS-resident `AgentAutopsyCapsule`s.
- `unresolved_obs`: Derived by scanning `handover/alignment/OBS_*.md` files (`src/runtime/markov_capsule.rs:271`).
- `next_session_context_cid`: Derived from a JSON blob of other derivable data.
No "creator-only" fields introducing a hidden source of truth were found.

### 4. CR-15.6 (Markov default prevents context poisoning)
**Verdict**: PASS
**Finding**: The deferral of agent-side enforcement is explicitly ratified by the spec. The charter §1.3 states, "InitAI / agent-side enforcement is OUT OF SCOPE for TB-15... agent-side honoring of the Markov default is documented as a P5 follow-up requirement." TB-15 correctly ships the substrate: the `generate_markov_capsule` binary (`src/bin/generate_markov_capsule.rs`) and its underlying library function `try_deep_history_read_with_override_check` (`src/runtime/markov_capsule.rs:182`) enforce the `TURINGOS_MARKOV_OVERRIDE=1` gate. This is sufficient to meet the TB-15 scope.

### 5. FR-15.1 trigger-site coverage
**Verdict**: PASS
**Finding**: The v0 spec compliance is partial, but it is explicitly and honestly labeled as partial. The charter §1.2 ("Trigger contract") clearly states, "TB-15 v0 wires ONE production trigger site... TaskBankruptcyTx". It then explicitly lists `SlashLoss`, `ChallengeUnsuccessful`, and `VerifierBondLost` as "DEFERRED to TB-15.1 / TB-16". This level of documentation and scoping is excellent discipline and fully compliant with the charter's own terms.

### 6. FR-15.4 + FR-15.5 (Markov chain integrity)
**Verdict**: PASS
**Finding**: The chain is structurally tamper-evident. The `capsule_id` of capsule `N` is a cryptographic hash of its contents, which includes `previous_capsule_cid` (pointing to capsule `N-1`). If an adversary swaps the bytes of capsule `N-1` in CAS, its `capsule_id` would change. A verifier walking backwards from capsule `N` would request the original `capsule_id` of `N-1` from CAS and either receive a "not found" error or, if the adversary also updated the pointer in `N`, the `capsule_id` of `N` would change, breaking the link from `N+1`. This standard hash-chaining mechanism is sound.

### 7. SG-15.7 spec literal "constitution hash AND flowchart hashes"
**Verdict**: CHALLENGE
**Finding**: The shipped `MarkovEvidenceCapsule` struct (`src/runtime/markov_capsule.rs:39`) contains `constitution_hash: Hash` but lacks any field for `flowchart_hashes`. This is a literal deviation from the architect spec SG-15.7. While an implicit reference via `TRACE_FLOWCHART_MATRIX.md` might be argued, the spec's use of "AND" implies two distinct referential components. An explicit field would provide stronger, more direct evidence of alignment. This is a non-blocking challenge to force either a spec amendment or a patch to add the field for full compliance.
- **File/Line**: `src/runtime/markov_capsule.rs:39-85` (struct definition lacks the field).

### 8. Class 2 envelope discipline
**Verdict**: PASS
**Finding**: TB-15 held to the Class 2 envelope. The architect spec §6.7 provides the primary trigger: "Promote to Class 3 if it modifies Agent read-view authorization." TB-15 did not modify `AgentVisibleProjection` or any read-view authorization gates. The addition of a new hook (`Step 3.5`) to an existing dispatch arm (`TaskBankruptcyTx`) and a new `apply_one` stage (`Stage 3.5`) is additive functionality that does not cross the specified risk boundary. The self-audit was appropriate under the governing charter.

## Architectural strategic questions

### 9. Cross-cutting impact of EconomicState 12 → 13 sub-fields
**Verdict**: PASS
**Finding**: The bump appears complete. The ship status document (`handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md`) lists updates to four test fixtures: `tests/economic_state_reconstruct.rs`, `tests/q_state_reconstruct.rs`, `tests/six_axioms_alignment.rs`, and `tests/fc_alignment_conformance.rs`. Examination of these files confirms the assertions were updated from 12 to 13, for example in `tests/q_state_reconstruct.rs:101` (`assert_eq!(obj.len(), 13);`). This demonstrates due diligence in tracking the magic number.

### 10. Tape-canonical (Art.0.2) extension
**Verdict**: PASS
**Finding**: The project correctly uses a custom `canonical_encode` / `canonical_decode` mechanism, not a non-deterministic format like `serde_json`. This is evidenced by round-trip tests in the new modules (e.g., `src/runtime/autopsy_capsule.rs:775`) and the established discipline of testing for BTreeMap permutation independence (e.g., `src/state/typed_tx.rs:1700`). This discipline makes replay-determinism robust against `serde` implementation details.

### 11. TB-16 forward compat
**Verdict**: PASS
**Finding**: The pattern for graceful evolution is established and followed. The new `agent_autopsies_t` field on `EconomicState` (`src/state/q_state.rs:278`) uses `#[serde(default)]`, ensuring backward compatibility with older chain snapshots. This pattern can be applied to the capsule structs themselves in future TBs to add new fields without breaking changes.

### 12. "Going-forward only" discipline (feedback_no_retroactive_evidence_rewrite)
**Verdict**: VETO
**Finding**: This discipline is violated. The new logic in `src/state/sequencer.rs` is hooked into the `TaskBankruptcyTx` dispatch arm (`dispatch_transition` match arm, line `1331+`) and the `apply_one` function (`Stage 3.5`, line `1539+`). When replaying a pre-TB-15 chain, the sequencer will re-execute old `TaskBankruptcyTx` transactions. The new code path will fire, generating `AgentAutopsyCapsule` CIDs and inserting them into `agent_autopsies_t`. This mutates the replayed state, producing autopsies that did not exist in the original run. This is a critical violation of the replay-determinism contract (Art.0.2). The dispatch arm requires an activation gate (e.g., `if logical_t >= TB15_ACTIVATION_T`) to prevent retroactive evidence generation.
- **File/Line**: `src/state/sequencer.rs:1331` (start of `TaskBankruptcy` dispatch arm) and `src/state/sequencer.rs:1539` (start of `apply_one` hook).

---

## VERDICT: VETO
- **Q12 VETO**: Replaying a pre-TB-15 chain will spuriously generate `AgentAutopsyCapsule` entries for old `TaskBankruptcyTx` events, breaking replay-determinism. The dispatch arm lacks an activation gate. (`src/state/sequencer.rs:1331`, `src/state/sequencer.rs:1539`)
- **Q7 CHALLENGE**: `MarkovEvidenceCapsule` lacks the `flowchart_hashes` field mandated by the literal text of SG-15.7, representing a spec deviation. (`src/runtime/markov_capsule.rs:39`)

**Conviction**:
- **VETO**: High. Breaking replay-determinism is a foundational architectural failure.
- **CHALLENGE**: Medium. A clear spec deviation that needs to be reconciled, but not a blocker on its own.

**Recommendation**: **REDESIGN**. The replay-determinism break (Q12) requires more than a simple patch; it necessitates a standard architectural pattern for feature activation on a live chain. This pattern must be designed, implemented, and tested before TB-15 can be considered safe to ship. The spec deviation (Q7) should be addressed during this redesign loop. The ship should be blocked and the changes reverted or amended.