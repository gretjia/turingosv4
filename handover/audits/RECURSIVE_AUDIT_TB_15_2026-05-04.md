# Recursive Dual Audit — TB-15 (Lamarckian Autopsy + Markov EvidenceCapsule)

**Date**: 2026-05-04 (post TB-15 ship 2026-05-03 commit `2337381`)
**Trigger**: User request 2026-05-04 (`send a dual audit recursively align with §6 spec`).
**Original ship envelope**: Class 2 self-audit (no Codex/Gemini round at ship per charter §4 — `feedback_dual_audit` hybrid-by-risk-class).
**Retroactive audit envelope**: Recursive dual audit (Codex impl-paranoid + Gemini architectural strategic) until convergence.
**Convergence**: **R3 — both auditors PASS** (Codex: medium-high conviction PROCEED to SHIP; Gemini: high conviction PROCEED to SHIP).
**Conservative merge** (per `feedback_dual_audit_conflict` VETO > CHALLENGE > PASS): **PASS — TB-15 cleared for ship under retroactive dual-audit envelope**.

---

## §1 Recursion summary

| Round | Codex | Gemini | Conservative merge |
|---|---|---|---|
| **R1** | CHALLENGE × 5 (Q3, Q4, Q5, Q8/RQ7, Q9) | **VETO** Q12 + CHALLENGE Q7 | **VETO** (Gemini Q12 blocks) |
| **R2** | **VETO** Q3 + TB15-CAS-ID (NEW prod-defect) | PASS (R1 findings cleared) | **VETO** (Codex Q3 blocks) |
| **R3** | **PASS** (medium-high) | **PASS** (high) | **PASS** ✓ |

**Total**: 3 rounds across 2 auditors = 6 audit transcripts. Convergence on R3.

**Per `feedback_audit_loop_roi_flip`**: R1 challenges all targeted production code (correct to remediate). R2 Codex VETO was a real production-code defect (CAS-cid mismatch — also correct to remediate). R3 verdicts clean; only OBS items for forward arch / cross-cut. ROI did not flip; recursion was justified at every step.

**Per `feedback_elon_mode_policy`** round-cap=2: exceeded by 1 round to close the Codex R2 VETO. Justified by "real production-code defect found at R2" (not test-scaffold edge); user's explicit "send a dual audit recursively" mandate took precedence over default round-cap.

---

## §2 R1 findings + R2 remediations

### R1 verdicts (2026-05-04)

| ID | Source | Severity | Finding |
|---|---|---|---|
| Q12 | Gemini | **VETO** | Replay-determinism: pre-TB-15 chain replay generates spurious autopsies; needs activation gate |
| Q7/Q8 | Both | CHALLENGE | `MarkovEvidenceCapsule` lacks `flowchart_hashes` field per literal SG-15.7 |
| Q3 | Codex | CHALLENGE | First Markov shipped with `--no-cas` → CAS residency unproven |
| Q4 | Codex | CHALLENGE | Deep-history override is test fixture only; not enforced on live generator path |
| Q5 | Codex | CHALLENGE | Halt-trigger #5 byte-window scan would miss JSON-serialized Cid leak |
| Q9 | Codex | CHALLENGE | Dashboard §15 doesn't actually regenerate from ChainTape + CAS |

### R2 remediations applied

1. **Q12 closure**: NEW `pub const TB15_AUTOPSY_ACTIVATION_LOGICAL_T: u64 = 0;` + `pub fn is_autopsy_active_at(timestamp_logical: u64) -> bool` predicate in `src/runtime/autopsy_capsule.rs`. Both `dispatch_transition` TaskBankruptcyTx Step 3.5 AND `apply_one` Stage 3.5 wrapped in the gate. Default const = 0 keeps fresh chains always-active; pre-TB-15 chain migration would override. Verification baseline: ZERO production chains contain TaskBankruptcyTx (grep across 10 evidence runtime_repo dirs).

2. **Q7/Q8 closure**: NEW field `pub flowchart_hashes: Vec<Hash>` on `MarkovEvidenceCapsule` (additive, `#[serde(default)]`). NEW `pub fn read_flowchart_hashes_from_matrix(path)` parses TRACE_FLOWCHART_MATRIX.md §2 → 4 canonical hashes. Generator binary populates the field; halt-trigger #2 strengthened.

3. **Q3 closure**: R2 generator dropped `--no-cas`; capsule + next_session_context written to real CAS at `/tmp/tb15-r2-cas/`.

4. **Q4 closure**: NEW CLI arg `--include-prior-capsules N` on generator binary; values >0 actively call `try_deep_history_read_with_override_check` BEFORE any deep-history I/O. Default-deny path returns exit code 3.

5. **Q5 closure**: Halt-trigger #5 strengthened to scan canonical Cid array text form (`[170,170,...,170]`) + raw 32-byte run + canonical_encode bytes.

6. **Q9 closure**: OBS-deferred to TB-16 (`OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04.md`). Privacy contract holds structurally; live rebuild is TB-16 controlled-arena scope per ship status doc.

---

## §3 R2 findings + R3 remediations

### R2 verdicts (2026-05-04)

| ID | Source | Severity | Finding |
|---|---|---|---|
| Q3 NEW | Codex | **VETO** (high) | `LATEST_MARKOV_CAPSULE.txt` published `a94ae884...` but CAS index keyed Markov object under `e4932fca...`. `cas.get(&capsule.capsule_id)` would FAIL. Root cause: `capsule_id = sha256(prelim_bytes)` (with capsule_id+sha256 zeroed during hash) but `cas.put(final_bytes)` stored the post-population bytes whose sha256 differs. Breaks SG-15.3 next-session bootstrap. |
| TB15-CAS-ID | Codex | **VETO** (high) | Same self-CID/content-CID mismatch in `write_autopsy_capsule` and `derive_autopsies_for_bankruptcy`. `agent_autopsies_t` Cids deterministic but not fetchable from CAS. |

Gemini R2: PASS (cleared R1 findings; only minor OBS).

### R3 remediations applied

1. **Q3 + TB15-CAS-ID closure**: writers rewritten so `capsule_id == sha256(stored_bytes)`. Pattern:
   ```rust
   // Build capsule with capsule_id=Cid::default(), sha256=Hash::ZERO.
   // canonical_encode → THESE are the bytes stored in CAS.
   let stored_bytes = canonical_encode(&capsule)?;
   let cid = Cid::from_content(&stored_bytes);
   let cas_returned_cid = cas.put(&stored_bytes, ...)?;
   debug_assert_eq!(cas_returned_cid, cid);
   // Populate in-memory struct AFTER CAS write for ergonomic caller view.
   capsule.capsule_id = cid;
   capsule.sha256 = Hash(cid.0);
   ```

2. **NEW restore helpers**:
   - `pub fn restore_markov_capsule_from_cas_bytes(bytes) -> Result<MarkovEvidenceCapsule, ...>` — canonical_decode + re-derive capsule_id/sha256 from `Cid::from_content(&bytes)`.
   - `pub fn restore_autopsy_capsule_from_cas_bytes(bytes) -> Result<AgentAutopsyCapsule, ...>` — symmetric.

3. **NEW `BankruptcyAutopsyDerivation` struct** replaces tuple in `derive_autopsies_for_bankruptcy` return type. Carries `capsule + private_bytes + stored_capsule_bytes`. Apply_one writes `stored_capsule_bytes` (the EXACT bytes whose sha256 == capsule_id).

4. **NEW round-trip tests** assert the contract:
   - `runtime::markov_capsule::tests::write_markov_capsule_cas_resolvable_by_capsule_id` — `cas.get(&cap.capsule_id)` succeeds + retrieved bytes' sha256 == capsule_id + restore round-trip works.
   - `runtime::autopsy_capsule::tests::write_bankruptcy_autopsies_to_cas_round_trip` — extended with same R3 contract for every emitted Cid.

5. **R3 evidence regenerated** at `handover/evidence/tb_15_markov_capsule_2026-05-04/` includes `cas_index.jsonl` showing CAS object Cid (`f9e701b4...`) MATCHES `LATEST_MARKOV_CAPSULE.txt`.

---

## §4 R3 verdicts (2026-05-04)

### Gemini R3 — PASS (high conviction)

> "The R3 remediation directly and correctly resolves the critical capsule_id mismatch bug that triggered the Codex R2 VETO. The new writer pattern, while introducing a developer-experience hazard, ensures the fundamental contract of the content-addressed store (id == hash(content)) is now met for both MarkovEvidenceCapsule and AgentAutopsyCapsule. The new round-trip tests provide the necessary mechanical proof of closure."

> "Recommendation: PROCEED to SHIP. Address the footgun (API hardening, documentation) and the TB-11 bug as high-priority follow-up items in TB-16."

### Codex R3 — PASS (medium-high conviction)

> "R3 closes the Codex R2 VETO for TB-15. Q3 Markov closure holds: `write_markov_capsule` builds the capsule with `capsule_id = Cid::default()` and `sha256 = Hash::ZERO`, canonical-encodes those zeroed bytes, derives `cid = Cid::from_content(&stored_bytes)`, stores exactly `stored_bytes`, then populates the returned in-memory struct."

> "TB15-CAS-ID closure also holds: `write_autopsy_capsule` uses the same zeroed-stored-bytes pattern. `derive_autopsies_for_bankruptcy` carries `stored_capsule_bytes` in `BankruptcyAutopsyDerivation`, and `write_bankruptcy_autopsies_to_cas` writes those exact bytes to CAS. Sequencer dispatch reads `d.capsule.capsule_id`; apply writes through the deterministic helper, so the Cids remain agreement-locked."

> "R3 evidence matches the claim: `LATEST_MARKOV_CAPSULE.txt` is `f9e701...`, and `cas_index.jsonl` has the `MarkovEvidenceCapsule` object under the same CID byte array."

> "Recommendation: PROCEED to SHIP with OBS for TB-11 evidence-capsule cleanup and small test/doc hardening."

---

## §5 Open follow-ups (carry-forward, NOT R3 ship blockers)

| OBS ID | Source | Description | Target |
|---|---|---|---|
| **OBS-TB15-R2-Q12-UPGRADE** | Gemini R2 | Upgrade compile-time `TB15_AUTOPSY_ACTIVATION_LOGICAL_T` const to a chain-resident marker for improved long-term robustness against operator error during hypothetical pre-TB-15 chain migrations | TB-16+ |
| **OBS-TB15-R2-Q7-TEST-HARDEN** | Gemini R2 | Add negative-path tests for `read_flowchart_hashes_from_matrix` parser (malformed input, missing markers) | TB-16+ |
| **OBS-TB15-R3-FOOTGUN** | Gemini R3 | API hardening: add loud-failure assertions on `capsule_id` accessors when struct has `Cid::default()` (would catch consumers who skip the `restore_*` helper). Document the on-CAS vs in-memory asymmetry | TB-16+ |
| **OBS-TB-11-CAS-ID** | Both R3 | TB-11 `write_evidence_capsule` has the SAME CAS-cid mismatch bug as the original TB-15 (predates the R3 fix). Refactor to use the "store zeroed-identity, populate after" pattern + add `restore_evidence_capsule_from_cas_bytes` + round-trip test. Currently no production reader of EvidenceCapsule via cap.capsule_id, so non-blocking | TB-16+ |
| **OBS-TB15-R3-DEBUG-ASSERT** | Codex R3 | The `debug_assert_eq!` in writers is debug-build only. The real structural guarantee is `CasStore::put` returning `Cid::from_content(content)`. Consider promoting to release-build assertion or removing the redundant check | TB-16+ |
| **OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04** | Codex R1 Q9 | Dashboard §15 `autopsy_event_counts` hard-coded empty (build_report doesn't rebuild EconomicState from chain). Privacy contract holds structurally; live rebuild = TB-16 controlled-arena scope | TB-16 |

---

## §6 Architectural deltas summary (R1 → R3 cumulative)

### R2 deltas (against R1 ship `2337381`)
- NEW `TB15_AUTOPSY_ACTIVATION_LOGICAL_T: u64 = 0` const + `is_autopsy_active_at` predicate in `src/runtime/autopsy_capsule.rs`
- NEW `flowchart_hashes: Vec<Hash>` field on `MarkovEvidenceCapsule` (additive, `#[serde(default)]`)
- NEW `read_flowchart_hashes_from_matrix` parser
- NEW `--include-prior-capsules N` CLI arg + live override gate plumbing in generator binary
- Strengthened halt-trigger #5 (canonical Cid array form scan + raw 32-byte run + canonical_encode bytes)
- Strengthened halt-trigger #2 (verifies flowchart_hashes too)
- Activation gates wrapped around dispatch arm Step 3.5 + apply_one Stage 3.5 in `src/state/sequencer.rs`
- 4 new unit tests
- NEW `OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04.md`
- Trust Root rehash: `src/state/sequencer.rs` (R1→R2)

### R3 deltas (against R2)
- Refactored `write_markov_capsule` writer to "store zeroed-identity bytes, populate after" pattern
- Refactored `write_autopsy_capsule` writer with same pattern
- Refactored `derive_autopsies_for_bankruptcy` to return `Vec<BankruptcyAutopsyDerivation>` (struct with `capsule` + `private_bytes` + `stored_capsule_bytes` fields)
- NEW `BankruptcyAutopsyDerivation` struct
- NEW `restore_markov_capsule_from_cas_bytes` + `restore_autopsy_capsule_from_cas_bytes` helpers
- NEW round-trip test `write_markov_capsule_cas_resolvable_by_capsule_id`
- Extended `write_bankruptcy_autopsies_to_cas_round_trip` with R3 contract assertions
- Updated 2 callers in `src/state/sequencer.rs` (dispatch arm + apply_one Stage 3.5) to consume new struct
- Updated 1 caller in `tests/fc_alignment_conformance.rs` (FC1-N33 witness)
- Trust Root rehash: `src/state/sequencer.rs` (R2→R3) + `tests/fc_alignment_conformance.rs` (R2→R3)
- Re-emitted Markov capsule with corrected writer; new evidence dir at `handover/evidence/tb_15_markov_capsule_2026-05-04/`

### Cumulative test count progression
- R1 ship `2337381`: 878 PASS / 0 fail / 150 ignored
- R2 baseline: 881 PASS / 0 fail / 150 ignored (+3 R2 unit tests)
- R3 baseline: 882 PASS / 0 fail / 150 ignored (+1 R3 round-trip test)

---

## §7 Audit transcripts

- `handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md` (CHALLENGE × 5)
- `handover/audits/GEMINI_TB_15_SHIP_AUDIT_2026-05-04_R1.md` (VETO Q12 + CHALLENGE Q7)
- `handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md` (VETO Q3 + TB15-CAS-ID)
- `handover/audits/GEMINI_TB_15_SHIP_AUDIT_2026-05-04_R2.md` (PASS — R1 findings cleared)
- `handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md` (PASS — R2 VETO closed)
- `handover/audits/GEMINI_TB_15_SHIP_AUDIT_2026-05-04_R3.md` (PASS — R2 VETO closed)
- `handover/audits/run_codex_tb_15_ship_audit{,_r2,_r3}.sh` (runners)
- `handover/audits/run_gemini_tb_15_ship_audit{,_r2,_r3}.py` (runners)

## §8 Final verdict

**PASS PASS** — both auditors converged at R3 with PROCEED to SHIP recommendations. Conservative merge per `feedback_dual_audit_conflict`: PASS.

**Production claim hardening from R3**:
> TB-15 (post R3 closure) ships:
> - Lamarckian Autopsy substrate (per-agent, per-event AgentAutopsyCapsule; CAS-resident; AuditOnly default; sequencer-side `agent_autopsies_t` index NOT in AgentVisibleProjection)
> - Markov EvidenceCapsule rollup (constitution_hash + 4 canonical flowchart_hashes + L4/L4.E/CAS roots + previous_capsule_cid + typical_errors + unresolved_obs + next_session_context_cid)
> - **CAS-cid contract**: `cas.get(&capsule.capsule_id)` succeeds for every emitted capsule (Markov + Autopsy); on-CAS bytes have zeroed identity fields; in-memory struct returned by writer has populated fields; `restore_*` helpers convert between forms
> - **Replay determinism**: activation gate ensures pre-TB-15 chain replay does not generate spurious autopsies (verified baseline: zero pre-TB-15 chains contain TaskBankruptcyTx)
> - **Privacy**: 3 structural fences (halt-trigger #1 + #4 + #5) prevent raw bytes from leaking to AgentVisibleProjection or typical-error broadcast surface
> - **Default-deny gate**: TURINGOS_MARKOV_OVERRIDE=1 actively gates the live generator path when --include-prior-capsules > 0 (not just a library helper)

**TB-15 retroactive dual-audit envelope CLEARED.** Charter §4 Class 2 self-audit envelope is upheld in spirit (no AgentVisibleProjection mod; no read-view-auth mod) with R3 hardening for the production-defect Codex R2 caught.

Cross-references:
- TB-15 charter: `handover/tracer_bullets/TB-15_charter_2026-05-03.md`
- TB-15 ship status: `handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md`
- R3 evidence: `handover/evidence/tb_15_markov_capsule_2026-05-04/`
- Architect spec: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §6
