# CO1.1.4-pre1 Dual External Audit — Final Merged Verdict ✅ PASS/PASS

**Date**: 2026-04-28
**Atom**: CO1.1.4-pre1 Typed Tx ABI Surface
**Final state**: spec v1.2.2 (`handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md`) + impl v1.2.2 (`src/state/typed_tx.rs` + supporting derive additions in `economy/money.rs`, `cas/schema.rs`, `system_keypair.rs`) — committed `4d917ac`.
**Conservative rule** (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

---

## § 1 Final Verdicts

| Auditor | Round | Verdict | Conviction |
|---|---|---|---|
| **Gemini** | r3 (v1.2) | **PASS** | High |
| **Codex** | r5 (v1.2.2) | **PASS** | High |
| **Conservative merged** | — | **PASS** ✅ | High |

**Pre-implementation gate**: CLEARED. CO1.7-impl A2 (TypedTx + Sequencer + dispatch_transition + replay_full_transition) is now unblocked.

---

## § 2 Round-by-round summary

| Round | Codex | Gemini | Conservative | Patch Round Output |
|---|---|---|---|---|
| 1 | CHALLENGE/high | CHALLENGE/high | CHALLENGE | v1.1 (P1-P10): 10 patches; commit `e0e4565` |
| 2 | CHALLENGE/high | PASS/high | CHALLENGE | v1.2 (P11-P15 + GR-1/2/3): 5 must-fix + 3 recommendations; commit `f4649a9` |
| 3 | CHALLENGE/high (doc-hygiene only — code+spec PASS) | **PASS/high** | CHALLENGE | v1.2.1: 2 doc-comment fixes; commit `33e75b8` |
| 4 (Codex-only) | CHALLENGE/high (1 more doc-hygiene) | (carry-forward Gemini r3 PASS) | CHALLENGE | v1.2.2: 1 fix + 2 preemptive doc cleanups; commit `4d917ac` |
| 5 (Codex-only) | **PASS/high** | (carry-forward Gemini r3 PASS) | **PASS** ✅ | — |

---

## § 3 Closure verification (round-1 → round-5 cumulative)

All 18 round-1 + round-2 must-fix items are CLOSED.

| Codex r5 question | Result |
|---|---|
| Q1 `system_keypair.rs:229` doc | PASS — `TerminalSummarySigningPayload::canonical_digest()` |
| Q2 `typed_tx.rs:218` doc | PASS — `WorkSigningPayload::canonical_digest()` via `WorkTx::to_signing_payload()` |
| Q3 `typed_tx.rs:336` doc | PASS — `TerminalSummaryTx::to_signing_payload().canonical_digest()` + domain prefix |
| Q4 Other stale residue | PASS — exact residue greps for 6 candidate patterns: NO MATCHES |
| Q5 Test status | PASS — 224/0 lib PASS, 0 ignored |

---

## § 4 Cumulative Cost

| Round | Codex tokens | Gemini tokens | Estimated $ |
|---|---|---|---|
| 1 | 199,200 | 113,295 | ~$8-15 |
| 2 | 165,930 | 114,610 | ~$7-13 |
| 3 | 91,030 | 109,082 | ~$4-9 |
| 4 (Codex-only) | 121,526 | — | ~$3-6 |
| 5 (Codex-only) | 159,562 | — | ~$4-7 |
| **CO1.1.4-pre1 total** | **737,248** | **336,987** | **~$26-50** |

Cumulative project audit spend: ~$161-252 / $890 mid-budget (~18-28%).

CO1.1.4-pre1 audit cost is in the **mid-range** (CO1.7 spec was $25-42 across 3 rounds; CO1.1.4-pre1 was $26-50 across 5 rounds — the extra two rounds were Codex-only doc-hygiene closure passes, not full design rounds). Real value: 1 critical security gap closed (agent-sig domain separation), 4 derived schemas locked (FinalizeRewardTx + dual-sign rationale; TerminalSummaryTx full migration; ClaimId newtype; bincode codec wording fix), 1 constitutional gate established (Art 0.2 cross-atom ordering for CO1.4-extra), 5 sedimented lessons captured in round-1/round-2 verdicts.

---

## § 5 Implementation path (now unblocked)

Per CO1.7-impl R3 § 5 + spec § 13:

1. **CO1.7-impl A2** (Sequencer + transition stubs): now unblocked. ~150 LoC. Builds on TypedTx defined in this atom + Sequencer per CO1.7 spec § 3.
2. **CO1.7-impl A3** (dispatch_transition exhaustive enum match): blocked-by A2. ~50 LoC stubs returning `TransitionError::NotYetImplemented`.
3. **CO1.4-extra** (CAS index persistence): MUST ship before CO1.7-impl A4 per this atom's § 0.1 cross-atom ordering gate. Independent atom.
4. **CO1.7-impl A4** (replay_full_transition + 4 CO1.7.5-stage tests): blocked-by A2 + A3 + CO1.4-extra. ~200-300 LoC.
5. **CO1.7.5** (per-kind transition function bodies): final L4 atom; fills the `NotYetImplemented` stubs from A3 with real transition logic.

Total downstream from CO1.1.4-pre1 PASS: **5 atoms** estimated 6-12 days for full Wave 6 #1 closure.

---

## § 6 Sedimented lessons (cumulative across all 5 rounds)

1. **Single-source-of-truth for tx schemas matters** (round-1): TerminalSummaryTx living in `system_keypair.rs` (signer module) leaked into a "frozen" location. Sedimented: per-typed-tx struct lives in `state::typed_tx`; signers consume opaque digests.

2. **Spec wording must specify codec-library byte layout** (round-1): STATE § 2.5 v1.4 was wrong (`#[repr(u8)]` does not control serde wire format). Sedimented: when freezing a wire format, spec must include actual codec library version + verified-by-test byte layout.

3. **"Record-only" golden fixtures are not golden** (round-1): self-comparison ("two encodes match") is round-trip stability, not ABI freezing. Sedimented: every ABI-freeze atom MUST lock SHA-256 hex in v1.

4. **Domain separation is non-negotiable when one signature primitive serves multiple roles** (round-1): type-level distinction at the API surface is necessary but not sufficient; canonical_digest pre-image MUST encode the role via stable byte-prefix.

5. **Cold-replay → Art 0.2 cross-atom ordering must be explicit in spec** (round-1): declaring "X is a separate atom" is not enough; create explicit cross-atom ordering gate (CO1.4-extra MUST ship before CO1.7-impl A4).

6. **Single-call-site type-update is insufficient** (round-2): adding ClaimId newtype but missing SignalBundle::Finalize. Sedimented: when changing a tx-payload field type, grep for ALL consumers (incl. SignalBundle / runtime APIs / fixtures).

7. **Symmetric-API completion** (round-2): adding `TerminalSummarySigning` variant but not corresponding `FinalizeRewardSigning` / `TaskExpireSigning` for the other system-emitted txs. Sedimented: when introducing a typed signing primitive for one variant, confirm symmetric coverage for ALL siblings.

8. **Domain-prefix tests must use IDENTICAL bodies** (round-2): non-collision test using DIFFERENT bodies passes trivially even without domain prefix. Sedimented: load-bearing domain-separation tests MUST construct identical bytes, hash with each domain, assert distinct results.

9. **Spec drift after structural migration** (round-2): claim "row removed" must be verified with grep. Same pattern as CO1.7 R2-C3.

10. **Codex implementation discipline + Gemini design quality is a stable axis decomposition** (rounds 2+3): Gemini PASSed at v1.1 + v1.2; Codex CHALLENGEd both with concrete patch lists. CHALLENGEs were closure-mechanical, not foundational disagreements. PASS/PASS only when both axes are clean. Project pattern.

11. **Doc-hygiene closure can take multiple narrow rounds** (rounds 4+5): when an atom migrates a type across modules, doc-comment cleanup can take 2-3 narrow Codex rounds to fully converge. Sedimented: post-migration, run an explicit "grep all stale references" pass BEFORE dispatching any audit; consider a pre-audit doc-hygiene script as part of `validate` skill.

— ArchitectAI synthesis, 2026-04-28; CO1.1.4-pre1 PASS/PASS gate cleared 12:15 UTC.
