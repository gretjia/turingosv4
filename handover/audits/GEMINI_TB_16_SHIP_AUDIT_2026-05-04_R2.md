# Gemini TB-16 Ship Audit — Controlled Market Smoke Arena (Class 3 dual external audit)
**Round**: R2
**Date**: 2026-05-04
**Test baseline**: cargo test --workspace = 905 PASS / 0 FAILED / 150 ignored (TB-16 Atom 6 ship commit 3300fe2)
**Halt-trigger battery**: 13/13 GREEN (tests/tb_16_halt_triggers.rs)
**Trust Root**: GREEN (2 rehashes propagated correctly)
**Audit envelope**: Class 3 integration smoke (architect §7.7 — external audit MANDATORY at ship)
**Atom 6 v0 scope note**: infrastructure shipped + audit-pipeline smoke verified end-to-end on chain-backed real-LLM tape (TB-13 fixture); fresh 6-task arena execution gated on Atom 6.1 (multi-task chain continuation) + mathlib build
**Elapsed**: 74.9s
**Prompt size**: 397,661 chars
**Audit mode**: architectural strategic (Codex covers impl-paranoid in parallel)

---

This audit is conducted under the specified role of a skeptical, independent, and paranoid architectural reviewer. The findings are based solely on the provided documentation and evidence, measured against the architect's §7 spec as the ground truth.

---

### Audit Findings (Q1-Q12)

1.  **CR-16.1 + CR-16.5 (conservation + sandbox-only funds)**
    **CHALLENGE**: The primary conservation check, `assert_18_total_supply_conserved` (`src/runtime/audit_assertions.rs:1010`), only compares the initial total supply to the final total supply of the replayed chain. The design document §6.2 Layer D claims this assertion holds "at every L4 row," but the implementation does not verify this. A transient mint/burn event mid-chain that resolves by the final block would not be detected by this assertion, creating a potential conservation gap. While `assert_17_no_post_init_mint` relies on the sequencer's internal checks, a direct, per-block audit assertion is required for robust verification.
    *   **Conviction**: Medium.
    *   **Recommendation**: FIX-THEN-PROCEED. Modify `assert_18` to iterate through the replayed states at each L4 entry and verify constant total supply.

2.  **CR-16.4 (no raw failure broadcast) — privacy contract**
    **VETO**: The privacy check `assert_28_projection_no_autopsy_bytes` (`src/runtime/audit_assertions.rs:1181`) is critically flawed. It performs a `windows(32)` byte-run scan over a `canonical_encode`d (bincode) representation of the `AgentVisibleProjection`. However, `serde_json` serializes a `[u8; 32]` array as a list of decimals (e.g., `[123, 45, 67, ...]`), which does not produce a contiguous 32-byte run of the raw CID. If the projection agents actually receive is JSON-encoded, this privacy check is completely ineffective and provides a false sense of security. The on-the-wire format contract for agents is not specified, making this a blocking ambiguity.
    *   **Conviction**: High.
    *   **Recommendation**: REDESIGN. The privacy check must be robust to the actual serialization format received by agents. Either enforce a canonical binary format for all agent projections or rewrite the assertion to parse the specific format (e.g., JSON) and check for the private CIDs' values, not their byte representation.

3.  **CR-16.6 (replayability) — audit-from-tape contract**
    **PASS**: The `audit_tape` binary (`src/bin/audit_tape.rs`) and its underlying library (`src/runtime/audit_assertions.rs`) correctly adhere to the audit-from-tape contract. The `AuditInputs` struct (`src/runtime/audit_assertions.rs:68`) is composed entirely of file paths, and the `load_tape` function (`src/runtime/audit_assertions.rs:434`) reads only from these specified artifacts. There is no evidence of access to live process state or forbidden caches like `state.db`.

4.  **CR-16.7 (sandbox banner) — SG-16.8 enforcement**
    **VETO**: The system has two conflicting implementations for handling non-sandbox funds. Architect §7.7 explicitly mandates a **HALT** if "non-sandbox funds used." The `audit_tape` assertion `assert_03_sandbox_agent_prefix` (`src/runtime/audit_assertions.rs:704`) correctly implements this by returning a `Halt` verdict. However, the `audit_dashboard`'s `detect_sandbox_run` function (`src/bin/audit_dashboard.rs:800`) merely renders a banner if *any* sandbox ID is found, which would allow a mix of real and sandbox wallets to pass with just a warning. This is a severe violation of the safety contract, misrepresenting a halt condition as informational.
    *   **Conviction**: High.
    *   **Recommendation**: REDESIGN. The dashboard logic must be brought into alignment with the architect's halt trigger. A mixed-ID chain must either cause the dashboard to refuse to render or display a top-level HALT/BLOCK message, not just the informational sandbox banner.

5.  **FR-16.2..7 spec compliance**
    **VETO**: The architect §7 spec is not a menu of options; it is the ground truth for the deliverable. §7.2 and §7.3 explicitly require a scenario and functional execution of WorkTx FirstLong, ChallengeTx Short, CompleteSet, price updates, Boltzmann masking, and Autopsy generation. The `TB-16_SHIP_STATUS_2026-05-04.md` (§2) and `Atom 6 v0 SCOPE NOTE` explicitly state that the execution of this scenario is **deferred to Atom 6.1**. Shipping the infrastructure without the mandated content is a material failure to meet the spec. This violates the `feedback_no_fake_menus` principle; the charter promised an executed arena, not just the tools to build one.
    *   **Conviction**: High.
    *   **Recommendation**: REDESIGN. The team must either (A) execute the full arena run as specified in architect §7 and resubmit TB-16 for audit with the correct evidence, or (B) formally amend the charter to scope TB-16 as a tooling-only deliverable, which would necessitate re-evaluation of its risk class and ship gates.

6.  **Class 3 envelope discipline**
    **CHALLENGE**: TB-16 was chartered as Class 3 ("production wire-up"). However, the shipped code in Atoms 1-6 consists entirely of new, additive, read-only audit tooling and an orchestrator scaffold. No existing production dispatch arms in `sequencer.rs` were modified. Per `feedback_risk_class_audit`, this work fits a Class 2 (self-audit) envelope. The Class 3 designation was predicated on the *deferred* arena run that would have stressed the production paths. The classification is therefore premature and misaligned with the actual code shipped.
    *   **Conviction**: Medium.
    *   **Recommendation**: RETRO-CLASS-2-DOWNGRADE. Acknowledge the misclassification. The ship should have been Class 2, with a promotion to Class 3 for the Atom 6.1 follow-up that actually modifies or heavily exercises production paths.

7.  **Tamper detection (Layer H)**
    **CHALLENGE**: The tamper detection strategy has weaknesses. The implementation in `audit_tape_tamper.rs:309` (`flip_byte_in_first_blob`) is described in a comment as picking the "FIRST non-empty file," which is fragile and could target a git tree object instead of a commit blob, potentially exercising a different failure path than intended. While the implementation now appears more robust ("destructively zeroed back half"), the selection logic is not guaranteed to hit a commit. Furthermore, the set of three tamper modes is minimal; it does not cover attacks like swapping bootstrap files (`agent_pubkeys.json`, `pinned_pubkeys.json`) or injecting a malicious L4.E entry.
    *   **Conviction**: Medium.
    *   **Recommendation**: FIX-THEN-PROCEED. Refine the object selection logic in `flip_byte_in_first_blob` to specifically target a git commit or blob object. Consider adding bootstrap file swaps to the standard tamper set.

8.  **Markov chain continuity break**
    **VETO**: The `audit_pipeline_smoke` evidence generated a `MARKOV_TB-16_2026-05-03.json` capsule with `previous_capsule_cid=null`. This violates CR-15.5 ("Capsules are evidence compression, not isolated islands"), as the TB-15 capsule already existed. While the `run_real_llm_arena.sh` script now contains logic to chain to the previous head, any process that can generate an unchained capsule when a predecessor exists is a constitutional violation. The system must enforce continuity, not leave it to script-level correctness.
    *   **Conviction**: High.
    *   **Recommendation**: REDESIGN. The `generate_markov_capsule` binary should refuse to generate a capsule with a null `previous_capsule_cid` if a `LATEST_MARKOV_CAPSULE.txt` pointer exists, unless an explicit `--force-genesis` flag is provided.

9.  **Atom 6.1 charter integrity**
    **VETO**: The charter's Atom 6 plan (`handover/tracer_bullets/TB-16_charter_2026-05-04.md` §3) includes a commit message template stating "all 13 tx kinds ... exercised on chain-backed real-LLM run". The `TB-16_SHIP_STATUS_2026-05-04.md` (§2) directly contradicts this by deferring the run. This is a material misrepresentation of the work delivered versus the work chartered. The ship status was honest about the deficit, but the project should have been halted for charter non-conformance before shipping.
    *   **Conviction**: High.
    *   **Recommendation**: REDESIGN. The project must adhere to its charter. Either fulfill the charter's promise or formally amend it *before* declaring a ship.

10. **38-assertion battery completeness**
    **CHALLENGE**: The assertion battery is incomplete. There is no assertion in `audit_assertions.rs` that directly verifies CR-16.7 ("All market activity is sandbox-labeled"). This check is only performed in `audit_dashboard.rs:800` (`detect_sandbox_run`), and as noted in Q4, that implementation is flawed. A constitutional requirement as critical as sandbox labeling must have a corresponding machine-verifiable assertion in the core audit suite.
    *   **Conviction**: Medium.
    *   **Recommendation**: FIX-THEN-PROCEED. Add a new Layer B assertion that walks the L4 tape, decodes each transaction, and verifies that every agent ID involved matches the sandbox prefix list, halting if any do not.

11. **R-022 backlinks**
    **CHALLENGE**: The `TRACE_MATRIX` documentation is imprecise. The file-level comment in `src/runtime/audit_assertions.rs:27` claims `FC1-N34` (audit_tape) coverage for the entire file. However, assertions #36-38 are exercised by `audit_tape_tamper` (FC1-N35), not `audit_tape`. This creates a traceability gap.
    *   **Conviction**: Low.
    *   **Recommendation**: FIX-THEN-PROCEED. Add per-function or per-layer `TRACE_MATRIX` annotations to `audit_assertions.rs` for greater precision.

12. **Test count drift**
    **CHALLENGE**: The test count reporting in `TB-16_SHIP_STATUS_2026-05-04.md` (§3) is misleading. The headline claim of "+25" over the TB-15 baseline of 759 is arithmetically incorrect (759+25=784, not 905). While the text explains the discrepancy is due to including sub-package tests, this violates the principle of clear reporting. Per `feedback_workspace_test_canonical`, the metric should be consistent.
    *   **Conviction**: Low.
    *   **Recommendation**: FIX-THEN-PROCEED. Amend the ship status to either use a workspace-level baseline for comparison or provide a clear package-level breakdown instead of a misleading summary number.

---

### Summary

The core deliverable of TB-16—an executed, multi-agent, multi-transaction-type smoke arena—was not delivered. Instead, the project shipped the *tooling* for such a run and deferred the execution. This is a fundamental violation of the architect's specification and the project's own charter. Compounding this are critical safety and privacy flaws, including a potentially ineffective private data check (Q2), an unsafe sandbox banner implementation (Q4), and a break in the constitutional Markov evidence chain (Q8). The Class 3 designation was also premature for the work actually completed.

While the "audit-from-tape" infrastructure is a significant architectural step forward, it cannot be approved when it is used to validate a body of evidence that fails to meet the mission's primary objective and contains multiple blocking-level flaws.

## VERDICT: VETO
- Q2 VETO: The privacy check for autopsy data in agent projections (`src/runtime/audit_assertions.rs:1181`) is likely ineffective against JSON serialization, creating a potential raw log leak.
- Q4 VETO: The dashboard's sandbox banner logic (`src/bin/audit_dashboard.rs:800`) violates the architect's "non-sandbox funds used" HALT trigger (§7.7), misrepresenting a critical safety failure.
- Q5 VETO: The primary deliverable—an executed arena exercising FR-16.2..7—was deferred to Atom 6.1, a material violation of the architect's spec (`handover/directives/...` §7.3).
- Q8 VETO: The generated Markov capsule was not chained to the TB-15 predecessor, violating the constitutional requirement CR-15.5 for evidence continuity.
- Q9 VETO: The charter's Atom 6 plan (`handover/tracer_bullets/TB-16_charter_2026-05-04.md` §3) promised a run exercising all 13 tx kinds, which was not delivered, representing a material deficit.