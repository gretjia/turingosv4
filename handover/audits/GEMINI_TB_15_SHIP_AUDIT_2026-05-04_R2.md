# Gemini TB-15 Ship Audit — Lamarckian Autopsy + Markov EvidenceCapsule (Class 2 retroactive dual audit)
**Round**: R2
**Date**: 2026-05-04
**Test baseline**: cargo test --workspace = 878 PASS / 0 FAILED / 150 ignored (TB-15 ship commit 2337381)
**Halt-trigger battery**: 6/6 GREEN (tests/tb_15_halt_triggers.rs)
**Trust Root**: GREEN (6 rehashes propagated correctly)
**Original audit envelope**: Class 2 self-audit per charter §4 (no Codex/Gemini at ship)
**Retroactive dual audit**: requested by user 2026-05-04 to verify Class 2 envelope held
**Elapsed**: 57.2s
**Prompt size**: 805,143 chars
**Audit mode**: architectural strategic (Codex covers impl-paranoid in parallel)

---

## Gemini TB-15 R2 Ship Audit — POST-REMEDIATION

**Role**: Architectural Strategic Reviewer
**Mandate**: Re-audit R1 findings for closure and identify any new cross-cutting concerns introduced by R2 changes.

---

### R2 Audit Findings

This audit verifies the closure of all findings from the R1 dual-audit (Gemini and Codex) and assesses the architectural soundness of the applied remediations.

#### **Q12 (VETO Closure): Activation Gate for Lamarckian Autopsy**

**Conviction**: High
**Status**: **VETO CLOSED**. The remediation is structurally sound and sufficient for ship.

1.  **Structural Soundness**: The activation gate, implemented via `pub fn is_autopsy_active_at(timestamp_logical: u64)` in `src/runtime/autopsy_capsule.rs`, is correctly and identically applied at both critical sites:
    *   **Dispatch Arm (State Mutation)**: In `src/state/sequencer.rs`, the `dispatch_transition` arm for `TaskBankruptcyTx` now gates the mutation of `q_next.economic_state_t.agent_autopsies_t`.
    *   **Apply_one (CAS Write)**: In `src/state/sequencer.rs`, the `apply_one` function's Stage 3.5 hook, which writes autopsy bytes to CAS, is gated by the same function.
    This dual-gating ensures that state mutation and CAS side-effects remain perfectly synchronized, preventing state-CAS divergence.

2.  **Retroactive Evidence Prevention**: The gate correctly uses the on-chain `bk.timestamp_logical` from the transaction itself. This upholds the `feedback_no_retroactive_evidence_rewrite` principle. When replaying a pre-TB-15 chain, an operator-set activation constant will ensure `is_autopsy_active_at` returns `false` for historical transactions, preventing the generation of spurious autopsy evidence that did not exist in the original L4.

3.  **Architectural Concerns**: The use of a compile-time `const TB15_AUTOPSY_ACTIVATION_LOGICAL_T: u64 = 0;` is a minor architectural weakness. It creates a procedural dependency, requiring a custom build with an overridden constant for any future replay of a (currently hypothetical) pre-TB-15 chain containing a `TaskBankruptcyTx`. A chain-resident marker would be more robust. However, the R2 context confirms that **zero production chains currently have a `TaskBankruptcyTx`**, which reduces the immediate risk of this procedural dependency to zero. The current solution is therefore sufficient.

**Recommendation**: The VETO is closed. I recommend creating an OBS item to track upgrading the compile-time activation gate to a chain-resident marker in a future TB as a hardening measure.

#### **Q7 (CHALLENGE Closure): `flowchart_hashes` in MarkovEvidenceCapsule**

**Conviction**: High
**Status**: **CHALLENGE CLOSED**. The remediation is correct and complete.

1.  **Correct Wiring**: The new `pub flowchart_hashes: Vec<Hash>` field on `MarkovEvidenceCapsule` (in `src/runtime/markov_capsule.rs`) correctly uses `#[serde(default)]`, ensuring backward compatibility with R1-generated capsules (which will deserialize with an empty `Vec`). The generator binary (`src/bin/generate_markov_capsule.rs`) is correctly wired to call the new `read_flowchart_hashes_from_matrix` parser and populate this field.

2.  **SG-15.7 Discharge**: This change fully discharges the literal requirement of ship gate SG-15.7 ("constitution hash AND flowchart hashes"). The R2 capsule artifact confirms the presence of all four canonical flowchart hashes. Halt-trigger #2 has been strengthened to verify this, providing ongoing regression protection.

3.  **Parser Robustness**: The parser in `read_flowchart_hashes_from_matrix` is brittle, as it relies on a highly specific string format within `TRACE_FLOWCHART_MATRIX.md`. However, this file is a canonical, version-controlled artifact, not arbitrary user input. The new test `read_flowchart_hashes_from_real_matrix` pins the parser's behavior against the current file format. Any future format change will break this test, forcing a corresponding update to the parser. This is an acceptable risk profile for an internal-facing tool.

#### **Other R1 Findings (Codex CHALLENGEs)**

**Conviction**: High
**Status**: **All 4 Codex findings have credible architectural closure.**

-   **Q3 (CAS residency)**: The generator binary's default behavior was corrected to ensure CAS residency. This is a procedural fix that closes the finding.
-   **Q4 (live override gate)**: The new `--include-prior-capsules N` argument provides a live code path that actively exercises the `TURINGOS_MARKOV_OVERRIDE` gate, with a distinct exit code on failure. This is a strong architectural remediation that moves the gate from a test-only concern to a live-binary feature.
-   **Q5 (byte-window scan)**: The halt-trigger was significantly strengthened to test against the actual `serde_json` serialization format of a `Cid` (an array of numbers), in addition to raw byte runs. This demonstrates a deep understanding of the failure mode and provides robust protection.
-   **Q9 (dashboard not regenerable)**: The deferral to TB-16 is architecturally sound. The critical privacy contract holds, as the dashboard's render function signature (`render_section_15` in `src/bin/audit_dashboard.rs`) proves it only receives event counts, not raw private data. Deferring the full live rebuild until TB-16 (the first run that will generate meaningful autopsy data) is a logical sequencing decision.

#### **New Findings & Cross-Cutting Concerns**

No new blocking issues or significant architectural regressions were introduced by the R2 changes.

-   **Backward Compatibility**: The use of `#[serde(default)]` on the new `flowchart_hashes` field correctly prevents any breaking change when deserializing R1-era `MarkovEvidenceCapsule` objects.
-   **Determinism**: The activation gate for autopsies is a pure function of on-chain data (`timestamp_logical`) and a build-time constant. For any given binary, its behavior is fully deterministic, solving the R1 VETO without introducing new problems.
-   **Test-Scaffold Edges**: Minor gaps, such as the lack of negative-path tests for the `read_flowchart_hashes_from_matrix` parser, were noted. However, these are low-risk issues related to the test scaffold, not production code. Per `feedback_audit_loop_roi_flip`, these do not warrant an R3 cycle.

---

## VERDICT: PASS

All R1 findings, including my Q12 VETO, have been closed with architecturally sound remediations. The R2 changes do not introduce new blocking concerns. The remaining minor issues (the compile-time activation constant and test-scaffold gaps) are low-risk and suitable for tracking as OBS items rather than blocking ship. The project has successfully passed the R2 audit gate.

**Conviction**: High
**Recommendation**: **PROCEED to SHIP** with the following OBS items:
-   **OBS-TB15-R2-Q12-UPGRADE**: Track upgrading the compile-time autopsy activation gate to a chain-resident marker for improved long-term robustness against operator error during historical chain replays.
-   **OBS-TB15-R2-Q7-TEST-HARDEN**: Track hardening the `read_flowchart_hashes_from_matrix` parser tests with negative-path cases (e.g., malformed input file).