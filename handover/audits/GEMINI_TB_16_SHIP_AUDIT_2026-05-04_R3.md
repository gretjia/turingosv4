# Gemini TB-16 R3 Ship Audit — Controlled Market Smoke Arena (Class 3 dual external audit)
**Round**: R3
**Date**: 2026-05-04
**Test baseline**: cargo test --workspace = 907 PASS / 0 FAILED / 150 ignored (TB-16 R3 commit 90848bb)
**Halt-trigger battery**: 13/13 GREEN (tests/tb_16_halt_triggers.rs)
**audit_pipeline_smoke verdict.json**: PROCEED 38/0/0/3 (3 skipped = Layer H tamper stubs at FC1-N35)
**audit_pipeline_smoke replay**: byte-identical to verdict.json
**Markov chain**: TB-16 capsule 737b4d22... chains to TB-15 head f9e701b4...
**Trust Root**: GREEN
**Audit envelope**: Class 3 integration smoke (architect §7.7 — external audit MANDATORY at ship)
**OBS-deferred**: audit_tape_tamper Round 2 hang (pre-existing on git HEAD; tamper logic untouched in R3; R1 carry-forward 3/3 detected)
**Elapsed**: 70.7s
**Prompt size**: 430,532 chars
**Audit mode**: architectural strategic R3 convergence (Codex covers impl-paranoid R3 in parallel)

---

# Gemini TB-16 Round 3 Ship Audit — Controlled Market Smoke Arena

**Role**: Skeptical adversarial reviewer, architectural strategic review (impl-paranoid angle).
**Mandate**: TB-16 R3 convergence audit. Conservative merge rule: VETO > CHALLENGE > PASS.
**Audit Baseline**: Commit `90848bb` (R3 prep).

This is the third and final round of my architectural audit for TB-16. My focus is on the adequacy of the R3 surgical fixes, the fitness of deferring the `audit_tape_tamper` hang, and an overall assessment of convergence to determine if the system is ready to ship.

---

### RQ1 — Q2 closure adequacy (JSON byte-run privacy)

**Finding**: The R3 fix to `assert_28_projection_no_autopsy_bytes` at `src/runtime/audit_assertions.rs:1483-1573` is adequate. My R2 VETO correctly identified that the original check for a raw 32-byte run was ineffective against a `serde_json` serialization of a CID as a decimal array. The new implementation now explicitly checks for both (a) the raw byte run in a `bincode` serialization and (b) the literal text of the JSON decimal array (e.g., `"[b₀,b₁,…,b₃₁]"`). This dual-check strategy, mirroring the pattern from TB-15 halt-trigger #5, robustly covers the two most plausible serialization paths for agent-visible data. While it is not a formal proof against all possible serialization formats (e.g., a custom `Debug` implementation), it directly and effectively closes the VETO-class gap. The remaining risk is sufficiently low for a Class 3 smoke test.

**Verdict**: PASS.

---

### RQ2 — Q1 closure adequacy (per-block conservation)

**Finding**: The new supplemental assertion `assert_d_total_supply_conserved_per_block` (id=40) at `src/runtime/audit_assertions.rs:1199-1259` is an excellent and robust closure for my R2 challenge. The O(N²) incremental replay strategy is not merely a brute-force version of the end-to-end check; it correctly and directly verifies the invariant at every single transaction step (L4 entry). This granularity is sufficient to catch transient mint/burn events that would be missed by an end-of-chain check, including multiple transactions within the same `logical_t` window that net to zero. The implementation also correctly halts on the first replay error, ensuring it doesn't proceed with a broken state.

**Verdict**: PASS.

---

### RQ3 — Q10 closure adequacy (machine-verifiable CR-16.7)

**Finding**: The new supplemental assertion `assert_a_chain_agent_ids_sandbox_prefixed` (id=41) at `src/runtime/audit_assertions.rs:657-712` is insufficient. While it correctly closes the gap for *accepted* transactions by walking the L4 ledger, it completely ignores *rejected* transactions logged in L4.E. The architect's mandate in §7.7 is to "Halt if: ... non-sandbox funds used". An *attempt* to use non-sandbox funds, even if rejected by other predicates, constitutes a violation of the sandbox's integrity and should be detected by the audit. The `HasSubmitter` trait is not used for L4.E entries, but the `tx_payload_cid` is available, allowing for the same `TypedTx` decoding and submitter check. This is a remaining audit gap.

**Verdict**: **CHALLENGE**.
- **RQ3 CHALLENGE**: The new sandbox-prefix assertion (id=41) fails to inspect rejected transactions in the L4.E log, leaving a gap in CR-16.7 enforcement (`src/runtime/audit_assertions.rs:657-712`).

---

### RQ4 — Q4 position-hold (architect §7.7 parsing)

**Finding**: The implementer's position—that the "non-sandbox funds used" HALT is an audit-time detection, not a sequencer admission gate—is architecturally consistent and defensible. The five halt conditions in architect §7.7 are listed as a parallel set. The other four (`conservation failure`, `raw log leak`, `price-as-truth`, `unresolved evidence gap`) are all implemented as audit-time assertions that operate on a completed chain. Interpreting the fifth condition in the same manner is a sound, parallel-structured reading of the spec. While a sequencer gate would be a stronger guarantee, the spec does not mandate it, and adding one would be a Class 3+/4 change. The current implementation of an audit-time HALT via Layer A assertions (#3 and the new #41) satisfies the literal requirement of the architect's directive for this Class 3 smoke test.

**Verdict**: PASS.

---

### RQ5 — Q11 closure (TRACE_MATRIX precision)

**Finding**: The R3 fix to the file-level doc-comment in `src/runtime/audit_assertions.rs` is adequate. It now correctly and precisely delineates which layers and assertions are exercised by `FC1-N34` (`audit_tape`) versus `FC1-N35` (`audit_tape_tamper`). This resolves the specific error from R2 (misattribution of Layer H) and provides a clear traceability map for R-022 consumers. While even greater per-function granularity is theoretically possible, the current per-layer binding to the executing binary is a logical and sufficient level of detail that meets the spirit of the requirement.

**Verdict**: PASS.

---

### RQ6 — Q12 closure (test-count math)

**Finding**: The closure is inadequate. The updated table in `TB-16_SHIP_STATUS_2026-05-04.md` §3, while more detailed, is still arithmetically incorrect. The sum of the deltas from the TB-15 baseline (882) is +25 (`13+5+3+2+2`), which should result in a pre-audit total of 907. The table claims a pre-audit total of 905. This implies an unstated subtractive delta of -2 tests, which the table omits. This violates the principle of clear and accurate reporting that was the basis of my original R2 challenge. This is a failure of documentation rigor.

**Verdict**: **CHALLENGE**.
- **RQ6 CHALLENGE**: The test-count math in `TB-16_SHIP_STATUS_2026-05-04.md` §3 remains incorrect and misleadingly omits subtractive deltas.

---

### RQ7 — OBS deferral fitness

**Finding**: The OBS deferral of the `audit_tape_tamper` hang is appropriate. Per `feedback_audit_obs_bias`, only "multi-hour future-arch" problems should be deferred at this stage. The diagnostic depth documented in `OBS_TB_16_TAMPER_R2_HANG_2026-05-04.md` is thorough, demonstrating that this is not a "cheap fix." The implementer has isolated the issue, formed a credible hypothesis involving low-level interactions between `git2`, `zlib`, and `bincode` on corrupted data, and verified it is not an R3 regression. Debugging this type of issue is non-trivial and meets the "multi-hour" complexity bar. The risk of deferral is low, as the primary `audit_tape` tool is unaffected and the tamper harness's past success provides some confidence in its design.

**Verdict**: PASS.

---

### RQ8 — Class 3 envelope discipline at R3

**Finding**: The cumulative scope of TB-16 warrants its Class 3 rating. While my R2 challenge noted that the initial atoms were Class 2 in nature, the subsequent delivery of the `comprehensive_arena` (Atom 5) and the execution of real-LLM runs against the sequencer (Atom 6, Step 4) are definitive Class 3 activities ("production wire-up"). The fact that the final R3 fixes are minor, read-only audit-side changes is a positive sign of convergence, not a reason to retroactively downgrade the risk classification of the entire, successfully executed integration smoke test.

**Verdict**: PASS.

---

### RQ9 — Convergence vs divergence

**Finding**: The audit history demonstrates clear **convergence**. We have moved from multiple VETOs in R1 (from both myself and Codex) on core functionality and evidence generation to a single real VETO in R2, which was closed in R3. My remaining R3 findings are two minor CHALLENGEs: one on the completeness of an audit assertion (RQ3) and one on a documentation table (RQ6). These findings are on the periphery of the system—the "test-scaffold edges." Per `feedback_audit_loop_roi_flip`, the return on investment for another full audit round has flipped. The core system is stable.

**Verdict**: PASS (on convergence).

---

## VERDICT: CHALLENGE

- **RQ3 CHALLENGE**: The sandbox-prefix assertion `assert_a_chain_agent_ids_sandbox_prefixed` (id=41) at `src/runtime/audit_assertions.rs:657-712` does not check rejected transactions in the L4.E log, leaving an audit gap for CR-16.7.
- **RQ6 CHALLENGE**: The test-count math in `TB-16_SHIP_STATUS_2026-05-04.md` §3 remains arithmetically incorrect and omits subtractive deltas, failing to provide a clear and accurate report.

**Conviction**: Medium.

**Recommendation**: **SHIP-WITH-OBS**. The round cap has been reached, and the audit process is demonstrating strong convergence. The remaining findings are minor, non-blocking challenges on the audit harness and documentation, not the core sequencer or economic logic. Per `feedback_audit_loop_roi_flip`, the ROI for an R4 has flipped. The two outstanding challenges should be converted to OBS tickets and addressed in a subsequent TB-16.x patch release. The TB-16 ship is clean for its Class 3 envelope.