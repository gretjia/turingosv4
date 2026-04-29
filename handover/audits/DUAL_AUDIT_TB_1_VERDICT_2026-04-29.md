# Dual External Audit Verdict — TB-1 Days 1-5 (Round 1)

**Date**: 2026-04-29
**Target**: TB-1 ship readiness (commits `063b003..6c04c26`)
**Auditors**:
- Codex (implementer-paranoid angle) — `CODEX_TB_1_AUDIT_2026-04-29.md`
- Gemini DeepThink (strategic / architectural angle) — `GEMINI_TB_1_AUDIT_2026-04-29.md`

**Merge rule** (memory `feedback_dual_audit_conflict`): **VETO > CHALLENGE > PASS** (conservative).

---

## Merged verdict: CHALLENGE

| Auditor | Verdict | Conviction |
|---|---|---|
| Codex r1 | **CHALLENGE** | high |
| Gemini r1 | PASS | 5/5 |

Per the conservative merge rule, **CHALLENGE wins** — TB-1 must NOT ship Day-7 unless one of these two paths is taken:

- **Path A (narrow the claim)**: amend the recharter / commit messages so TB-1's central claim is "P1/P3 primitives scaffolded; dispatch enforcement deferred to TB-2 RSP-1" — explicitly downgrading from "the v4 GitTape kernel honors the L4 / L4.E split". Tier-A unchanged. Light-touch (~1 hour: doc edits + commit message clarification).
- **Path B (close the gaps)**: address Codex P0s 1-4 directly via additional Tier-A tests + structural shielding patches before ship. Heavier (~3-6 hours: 4 patches + retests + round-2 audit per Elon-mode 2-round cap).
- **Path C (round 2)**: explicitly defer ship; run round-2 dual audit after Path-A or Path-B patches land.

**User decision required.** Do NOT auto-ship. Per `feedback_phased_checkpoint`: auto-pause at each gate.

---

## Auditor convergence

**Both auditors agree on**:
- The 9 Tier-A tests are technically green and well-written.
- The L4 / L4.E primitive split is architecturally sound.
- Trust Root manifest hygiene is flawless.
- STEP_B-protected files (kernel.rs / bus.rs / wallet.rs) were untouched.
- Day-4 h_vppu wire-up is correctly post-hoc in main() with the right `load → query → stamp → record → save` ordering.

**Auditors diverge on**:

| Question | Gemini reading | Codex reading |
|---|---|---|
| Does Tier-A prove the central claim? | "Architecturally sound" — primitives ready for TB-2. | "CHALLENGE — primitives only, not enforcement." |
| L4/L4.E disjointness | "Cryptographically isolated via domain-separation prefix." | "Disjointness not actually tested through dispatch_transition; sequencer is `NotYetImplemented` for all variants." |
| Monetary guard call sites | "Pure functions ready to call inside Sequencer match arms." | "Not yet wired anywhere; passes module tests + tb_1_acceptance only — silent bypass risk if dispatch never calls them." |
| Public/raw rejection record shielding | "Excellent — `From<&Record> for PublicView` is the projection." | "`RejectedSubmissionRecord` is `pub` and derives `Serialize`; `records()` returns raw refs — convention, not enforced." |
| Tier-A coverage of monetary invariant | "Sufficient for tracer bullet (closed-loop conservation)." | "Tier-A test 7 only touches balances + escrow; the all-six-subindex test exists at the unit level (`ctf_counts_all_six_holding_subindexes`) but is NOT in Tier-A." |

The divergence is fundamentally about **where TB-1's central claim should land on the spec-vs-enforcement spectrum**. Gemini reads "primitives shipped, ready for TB-2 wiring" as the claim. Codex reads "the v4 kernel honors the L4/L4.E split" as the claim and finds enforcement gaps.

---

## Codex P0 list (must-fix-before-unqualified-ship)

| # | Finding | Concrete remediation | Effort |
|---|---|---|---|
| **P0-1** | TB-1's central claim is over-claimed. The 9 Tier-A tests prove the primitives, NOT that "the v4 kernel honors the L4/L4.E split"; sequencer.rs `dispatch_transition` is `NotYetImplemented` for all K5 variants ([src/state/sequencer.rs:47](src/state/sequencer.rs#L47)) and `apply_one` early-returns on transition error before any L4.E append ([src/state/sequencer.rs:339](src/state/sequencer.rs#L339)). | EITHER (a) narrow the claim ("primitives scaffolded") in commit messages + recharter doc, OR (b) wire `dispatch_transition` + add a Tier-A test that exercises a real predicate-failed dispatch and asserts L4.E gets the record + L4 untouched. | ~1h (a) / ~4h (b) |
| **P0-2** | Tier-A test 7 (`test_p3_rsp0_exit_1_on_init_total_invariant`) only exercises balances + escrows; the all-six-subindex coverage exists at the unit level but is NOT in Tier-A. A regression that undercounts claims/stakes/task_markets bounty/challenge_cases bond would pass Tier-A. | Promote `ctf_counts_all_six_holding_subindexes` (currently in `monetary_invariant.rs` test module) into the Tier-A battery as a 10th blocking test, OR rewrite test 7 to redistribute through ALL six subindices. | ~30min |
| **P0-3** | `RejectedSubmissionRecord` is `pub`, derives `Serialize`, exposes `pub raw_diagnostic_cid`, and `RejectionEvidenceWriter::records()` returns full raw refs ([src/bottom_white/ledger/rejection_evidence.rs:82](src/bottom_white/ledger/rejection_evidence.rs#L82) + [:320](src/bottom_white/ledger/rejection_evidence.rs#L320)). The shield is convention, not type-system-enforced. Any future agent-facing serialization path that goes through `RejectedSubmissionRecord` instead of `PublicRejectionView` leaks the raw cid. | Either (a) `#[serde(skip_serializing)]` on `raw_diagnostic_cid`, OR (b) make `RejectedSubmissionRecord` `pub(crate)` and route external access only through `public_view()` / a privileged audit-only API. | ~30min |
| **P0-4** | `AcceptedLedger::load_from_path` calls `reconstruct_state` only, NOT `verify_chain` ([src/economy/ledger.rs:300](src/economy/ledger.rs#L300)). A `prev_hash`/`hash`/`logical_t`-only tamper can load successfully unless the caller separately verifies. The Tier-A bypass test catches one specific shape (mutating last entry's `resulting_state_root`) but misses fake-genesis, row-reorder, and parent-state-root-only mutations. | (a) Make `load_from_path` call `verify_chain(0, len)` before returning Ok; AND (b) add 3 more Tier-A tamper tests (fake-genesis, reorder, parent-state-root-only). | ~1h |

## Codex P1 list (should-fix; ship-with-OBS allowed)

- **P1-1**: Escrow vault has zero Tier-A coverage. Either promote `lock_escrow + release_escrow + overpayout-reject` into Tier-A, OR explicitly mark "escrow is RSP-1 scaffolding only" in the recharter.
- **P1-2**: P6 evidence (Day-4 mathd_algebra_107 jsonl runs) lives at `/tmp/tb1_day4_smoke_v2/` (untracked). Move to a tracked `handover/evidence/tb_1_day4_h_vppu/` directory so the evidence survives a reboot.
- **P1-3**: Add an integration test around the evaluator h_vppu main() wire-site (not just `HVppuHistory` API in isolation) once `experiments/minif2f_v4/tests/` grows.

## Gemini P1 list (observations only; not blocking)

- **P1-G1**: Long-term P6 baseline drift — when P6 gets its own dedicated TB, consider adding a parallel `static_baseline` field that never drops the first successful run (rolling-window N=3 makes long-term tracking hard).
- **P1-G2**: `assert_total_ctf_conserved`'s `exempt_tx_kinds` slice is forward-thinking and won't need rewriting at P3 RSP-4 / P8.

---

## Recommended path

**Path A (narrow the claim) — recommended.** Lowest risk; preserves the 7-day TB timebox; keeps TB-2 RSP-1 unblocked. Changes:

1. **`handover/tracer_bullets/TB-1_recharter_2026-04-29.md` § 1**: edit GOAL to read "discharge the **primitive scaffolding** for P1 + P3 RSP-0 ... `dispatch_transition` enforcement deferred to TB-2 RSP-1". Add an explicit bullet: "TB-1 ships PRIMITIVES + INVARIANTS, NOT dispatch enforcement."
2. **Day-7 ship commit message** (whenever it lands): mirror the narrowed claim.
3. **Tier-A test module-doc**: add a "Limitations" section listing what the battery does NOT prove (sequencer enforcement, monetary guard call sites, raw record shielding via type system).
4. **Optional sweetener**: P0-2 (~30min) — promote `ctf_counts_all_six_holding_subindexes` into Tier-A as a 10th blocking test. Closes the most concrete Codex P0 with minimal effort.
5. **Optional sweetener**: P0-3 (~30min) — `#[serde(skip_serializing)]` on `RejectedSubmissionRecord.raw_diagnostic_cid`. Type-enforces what was previously convention. Trivial patch.

After Path-A patches: ship Day-7 with the narrowed claim. **Skip round-2 audit** — Codex's CHALLENGE was specifically about claim scope, not about latent bugs. Path-A directly addresses the scope mismatch.

**Path B (close the gaps fully) — heavier**. Address all 4 Codex P0s. ~3-6 hours, then round-2 audit per Elon-mode 2-round cap.

**Path C (defer)** — if user wants TB-2 RSP-1 to fold in the dispatch_transition wiring as part of its scope rather than retrofit TB-1.

---

## Disposition (for user decision)

The user's overnight authorization was: "进行到送双外审并收集双外审结果给我睡觉回来看" — go up to launching the dual audit and collecting results. **That work is done.** The Day-7 ship gate is now the user's decision, with these three paths.

**Default if no decision**: do nothing — TB-1 stays at HEAD `6c04c26` with the audit verdict in the record. No further commits, no auto-ship.

---

## Compute spend (this round)

- Codex r1: 154,243-char prompt → **154,768 tokens used** (per Codex stderr); ~$3-4 estimated.
- Gemini r1: 197,491-char prompt → 53.0s API; ~$1-2 estimated.
- Total round-1 dual audit: ~**$5-6**. Well within the TB-1 $30 audit budget.

If round-2 is needed (Path B), reserve ~$10-15 for it.

---

## Cross-references

- TB-1 recharter: `handover/tracer_bullets/TB-1_recharter_2026-04-29.md`
- L4 / L4.E decision record: `handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`
- Tier-A battery: `tests/tb_1_acceptance.rs` (commit `6c04c26`)
- Day-4 live evidence: `/tmp/tb1_day4_smoke_v2/` (NB: untracked; P1-2 above)
- Codex full report: `handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md`
- Gemini full report: `handover/audits/GEMINI_TB_1_AUDIT_2026-04-29.md`
- Memory: `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS)
- Memory: `feedback_phased_checkpoint` (auto-pause at each gate)
- Memory: `feedback_elon_mode_policy` (round-cap=2)
