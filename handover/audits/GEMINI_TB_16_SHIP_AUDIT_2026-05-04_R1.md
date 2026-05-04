# Gemini TB-16 Ship Audit — Controlled Market Smoke Arena (Class 3 dual external audit)
**Round**: R1
**Date**: 2026-05-04
**Test baseline**: cargo test --workspace = 905 PASS / 0 FAILED / 150 ignored (TB-16 Atom 6 ship commit 3300fe2)
**Halt-trigger battery**: 13/13 GREEN (tests/tb_16_halt_triggers.rs)
**Trust Root**: GREEN (2 rehashes propagated correctly)
**Audit envelope**: Class 3 integration smoke (architect §7.7 — external audit MANDATORY at ship)
**Atom 6 v0 scope note**: infrastructure shipped + audit-pipeline smoke verified end-to-end on chain-backed real-LLM tape (TB-13 fixture); fresh 6-task arena execution gated on Atom 6.1 (multi-task chain continuation) + mathlib build
**Elapsed**: 59.9s
**Prompt size**: 399,376 chars
**Audit mode**: architectural strategic (Codex covers impl-paranoid in parallel)

---

This audit is conducted with an adversarial, skeptical stance, prioritizing architectural integrity and strict adherence to the architect's mandate.

---

### Q1: CR-16.1 + CR-16.5 (Conservation + Sandbox-only funds)

**Finding**: The design correctly centralizes all genesis minting to `runtime::bootstrap::default_pput_preseed_pairs()` and relies on `assert_no_post_init_mint` (Layer D #17 in `src/runtime/audit_assertions.rs`) to prevent any post-init minting. The bootstrap pipeline for `comprehensive_arena` reads from this preseed configuration, and the `run_real_llm_arena.sh` script contains no logic that could introduce a new mint path. The defense-in-depth approach, where the audit tape verifier re-runs every transaction through a dispatch path that would halt on a conservation violation, appears sound. No TB-16-specific code paths that could bypass this check were identified.

**Verdict**: PASS.

---

### Q2: CR-16.4 (No raw failure broadcast) — privacy contract

**Finding**: The check `assert_28_projection_no_autopsy_bytes` in `src/runtime/audit_assertions.rs:1108` is fundamentally flawed. It scans for 32-byte runs of a `private_detail_cid`. However, the input it scans is the canonical-encoded `q.tape_view_t`, which is a `serde_json` serialization. `serde_json` serializes a `[u8; 32]` array as a JSON array of decimals (e.g., `[123, 45, 67, ...]`), not a contiguous block of 32 raw bytes. The byte-run check is therefore searching for a pattern that cannot exist in its input format, rendering the privacy check ineffective. The canonical encoding flow for the audit assertion does not match the byte-level reality of the serialized data.

**Verdict**: **CHALLENGE**.
- **Q2 CHALLENGE**: The privacy check in `assert_28_projection_no_autopsy_bytes` (`src/runtime/audit_assertions.rs:1108`) is ineffective; it scans for raw byte runs in a JSON-decimal-array serialization.

---

### Q3: CR-16.6 (Replayability) — audit-from-tape contract

**Finding**: The specified input set for the `audit_tape` binary (`src/bin/audit_tape.rs:37`) correctly adheres to the audit-from-tape contract. The inputs (`runtime_repo`, `cas_dir`, `agent_pubkeys`, `pinned_pubkeys`, `genesis`, `constitution`, `markov_pointer`, `alignment_dir`) are all on-disk artifacts of a completed run. The design explicitly forbids consulting live Sequencer state or the `state.db` cache, and the implementation in `audit_tape.rs` honors this by only accepting file paths. This structure correctly enforces that the verdict is derived solely from the canonical, persisted evidence.

**Verdict**: PASS.

---

### Q4: CR-16.7 (Sandbox banner) — SG-16.8 enforcement

**Finding**: The logic in `audit_dashboard.rs:818` (`detect_sandbox_run`) renders the SANDBOX banner if *any* sandbox-prefixed agent ID is found. This is insufficient and violates a halt condition. Architect §7.7 explicitly states to **"Halt if: ... non-sandbox funds used"**. If a chain contained a mix of sandbox IDs and a real, non-sandbox wallet ID, the current logic would simply render the banner and proceed. The correct, fail-safe behavior should be to trigger a HALT condition, as the presence of non-sandbox funds is a forbidden state. The current implementation prioritizes labeling over enforcement.

**Verdict**: **VETO**.
- **Q4 VETO**: The sandbox check in `audit_dashboard.rs:818` only renders a banner on mixed-ID chains; it fails to enforce the architect's §7.7 HALT condition for "non-sandbox funds used".

---

### Q5: FR-16.2..7 spec compliance

**Finding**: The architect spec §7 is the ground truth. §7.2 and §7.3 mandate a scenario and functional requirements that necessitate the *existence* of `WorkTx`, `ChallengeTx`, `CompleteSetMintTx`, price updates, masks, and autopsies on the shipped chain. The `TB-16_SHIP_STATUS_2026-05-04.md` §2 explicitly states that the fresh arena execution required to generate these transactions is **deferred to Atom 6.1**. The shipped artifact is the *infrastructure* to run the test, not the *result* of the test. This is a material failure to meet the core functional requirements of the spec. Shipping the tooling without the evidence it's meant to produce does not satisfy the mandate.

**Verdict**: **VETO**.
- **Q5 VETO**: The core deliverable is missing. Architect §7.3 requires a chain exercising all market functions; the ship status (`TB-16_SHIP_STATUS_2026-05-04.md` §2) confirms this was deferred to Atom 6.1.

---

### Q6: Class 3 envelope discipline

**Finding**: TB-16 was chartered as Class 3 ("production wire-up"). However, the delivered code in Atoms 2, 3, and 4 consists of new, additive, read-only audit paths (`audit_assertions`, `audit_tape`) and dashboard sections. It does not appear to modify the core production sequencer dispatch arms. The `comprehensive_arena` is a new, separate binary. Per `feedback_risk_class_audit`, Class 3 is for modifying production code paths. Additive, read-only audit infrastructure is typically Class 2 (self-audit). The risk classification appears inflated for the work delivered in Atom 6 v0. The *next* step, Atom 6.1, which would require chain-continuation logic in the evaluator/sequencer, *would* be Class 3.

**Verdict**: **CHALLENGE**.
- **Q6 CHALLENGE**: The risk envelope is mis-classified. TB-16 v0 adds read-only audit tooling, which is Class 2 work, not Class 3 production wire-up modification.

---

### Q7: Tamper detection (Layer H)

**Finding**: The tamper detection in `audit_tape_tamper.rs:276` (`flip_byte_in_first_blob`) is naive. It picks the *first non-empty file* in `.git/objects/`. This could be a Git `tree` object, not necessarily a `commit` or `blob` object. Corrupting a tree object has different failure modes than corrupting a commit's parent hash or a blob's content. It is not clear if the `verify_chaintape` pipeline is robust enough to detect all such corruption types. Furthermore, the 3-mode set is insufficient. It does not test for swapping bootstrap files like `agent_pubkeys.json` or `pinned_pubkeys.json`, which could lead to signature verification bypasses. It also doesn't test for more subtle L4.E injection attacks.

**Verdict**: **CHALLENGE**.
- **Q7 CHALLENGE**: Tamper detection in `audit_tape_tamper.rs:276` is not comprehensive; it may corrupt non-commit objects, and the test suite omits critical attack vectors like bootstrap file swaps.

---

### Q8: Markov chain continuity break

**Finding**: The `MARKOV_TB-16_2026-05-03.json` capsule generated by the audit pipeline smoke has `previous_capsule_cid=null`. This is a direct violation of the architectural principle established in TB-15. Architect spec §6 (for TB-15), CR-15.5 states: "Capsules are evidence compression, not isolated islands." Breaking the chain creates an isolated island of evidence, undermining the purpose of a continuous, auditable Markov chain of system states. The documentation provides no justification for this break.

**Verdict**: **VETO**.
- **Q8 VETO**: The generated Markov capsule breaks evidence continuity (`previous_capsule_cid=null`), violating the "no isolated islands" principle from architect spec CR-15.5.

---

### Q9: Atom 6.1 charter integrity

**Finding**: Charter §3, Atom 6, describes the deliverable as a run where "all 13 tx_kinds present". The `TB-16_SHIP_STATUS_2026-05-04.md` §4 buries this gap under an "Atom 6.1 follow-up". This is a `feedback_no_fake_menus` violation. The charter promised a complete meal, but the delivery is just the cutlery with a promise of food later. The ship status is not honest about this being a failure to meet the charter's goal for Atom 6; it frames it as a routine follow-up.

**Verdict**: **CHALLENGE**.
- **Q9 CHALLENGE**: The ship status misrepresents a failure to meet the charter's deliverable ("all 13 tx_kinds present") as a planned "Atom 6.1 follow-up", violating the `feedback_no_fake_menus` principle.

---

### Q10: 38-assertion battery completeness

**Finding**: A direct mapping of architect §7.4 (CR-16.x) to the 38 assertions listed in the design doc §6.2 shows complete coverage.
- CR-16.1 (Conservation) -> Layer D #18
- CR-16.2 (No ghost liquidity) -> Layer D #19
- CR-16.3 (No price override) -> Layer E #26
- CR-16.4 (No raw failure broadcast) -> Layer F #28
- CR-16.5 (No real user funds) -> Layer A #3
- CR-16.6 (Replayable) -> Layer C #12
- CR-16.7 (Sandbox-labeled) -> This is a UI/presentation requirement, correctly handled by the dashboard, while the underlying data invariant is checked by Layer A #3.
The assertion battery appears to fully cover the constitutional requirements.

**Verdict**: PASS.

---

### Q11: R-022 backlinks

**Finding**: The `TRACE_MATRIX` doc-comments are incorrect. The question correctly identifies that tamper assertions #36-#38 in `src/runtime/audit_assertions.rs` are traced to `FC1-N34` (`audit_tape`) but are exercised by `FC1-N35` (`audit_tape_tamper`). This indicates a lack of rigor in maintaining traceability, which is the entire point of the annotations. While a minor issue, it undermines confidence in the process.

**Verdict**: **CHALLENGE**.
- **Q11 CHALLENGE**: `TRACE_MATRIX` doc-comments are incorrect; e.g., tamper assertions in `src/runtime/audit_assertions.rs` are mis-attributed to `FC1-N34` instead of `FC1-N35`.

---

### Q12: Test count drift

**Finding**: The ship status §3 claims "+25 over TB-15 baseline 759" but reports a total of 905. The math (759 + 25 = 784) does not add up. The explanation that sub-package tests are now included makes the "+25" figure misleading. Per `feedback_workspace_test_canonical`, reporting should be consistent and transparent. Citing a net change that doesn't align with the baseline and total is poor practice.

**Verdict**: **CHALLENGE**.
- **Q12 CHALLENGE**: Test count reporting in `TB-16_SHIP_STATUS_2026-05-04.md` §3 is misleading; the reported net change (+25) does not reconcile with the baseline and total counts (759 vs 905).

---

## VERDICT: VETO

- **Q4 VETO**: The sandbox check in `audit_dashboard.rs:818` only renders a banner on mixed-ID chains; it fails to enforce the architect's §7.7 HALT condition for "non-sandbox funds used".
- **Q5 VETO**: The core deliverable is missing. Architect §7.3 requires a chain exercising all market functions; the ship status (`TB-16_SHIP_STATUS_2026-05-04.md` §2) confirms this was deferred to Atom 6.1.
- **Q8 VETO**: The generated Markov capsule breaks evidence continuity (`previous_capsule_cid=null`), violating the "no isolated islands" principle from architect spec CR-15.5.
- **Q2 CHALLENGE**: The privacy check in `assert_28_projection_no_autopsy_bytes` (`src/runtime/audit_assertions.rs:1108`) is ineffective; it scans for raw byte runs in a JSON-decimal-array serialization.
- **Q6 CHALLENGE**: The risk envelope is mis-classified. TB-16 v0 adds read-only audit tooling, which is Class 2 work, not Class 3 production wire-up modification.
- **Q7 CHALLENGE**: Tamper detection in `audit_tape_tamper.rs:276` is not comprehensive; it may corrupt non-commit objects, and the test suite omits critical attack vectors like bootstrap file swaps.
- **Q9 CHALLENGE**: The ship status misrepresents a failure to meet the charter's deliverable ("all 13 tx_kinds present") as a planned "Atom 6.1 follow-up", violating the `feedback_no_fake_menus` principle.
- **Q11 CHALLENGE**: `TRACE_MATRIX` doc-comments are incorrect; e.g., tamper assertions in `src/runtime/audit_assertions.rs` are mis-attributed to `FC1-N34` instead of `FC1-N35`.
- **Q12 CHALLENGE**: Test count reporting in `TB-16_SHIP_STATUS_2026-05-04.md` §3 is misleading; the reported net change (+25) does not reconcile with the baseline and total counts (759 vs 905).

**Conviction**: High.

**Recommendation**: **REDESIGN / RE-SCOPE**. This ship is blocked on three fundamental VETO-class issues. The most significant is that the primary deliverable mandated by the architect—an actual smoke test run exercising the specified market functions—was not delivered. The project cannot proceed until it either (A) delivers the required evidence as specified in architect §7, or (B) formally re-charters TB-16 as an infrastructure-only milestone with a corresponding risk-class downgrade and a new, separate charter for the actual market smoke test. The current state is unacceptable for a Class 3 ship.