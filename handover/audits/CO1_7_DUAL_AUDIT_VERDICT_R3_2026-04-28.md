# CO1.7 Round-3 Dual External Audit — Merged Verdict ✅ PASS/PASS

**Date**: 2026-04-28
**Target**: spec v1.2 (`handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md`) + skeleton v1.2 (`src/bottom_white/ledger/transition_ledger.rs`) + system_keypair.rs v1.2 extension
**Auditors**: Codex (gpt-5-codex; 320k tokens) + Gemini 2.5 Pro (277k tokens)
**Conservative rule** (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS

---

## § 1 Verdicts

| Auditor | Verdict | Conviction | Top must-fix |
|---|---|---|---|
| **Codex** | **PASS** | High | None |
| **Gemini** | **PASS** | High | None |
| **Conservative merged** | **PASS** ✅ | High | **NONE** |

**Pre-implementation gate cleared**: per CLAUDE.md "Audit Standard" + memory `feedback_dual_audit`, CO1.7 implementation start is now unblocked.

---

## § 2 Closure verification (round-2 → round-3)

| Round-2 must-fix | v1.2 fix | Codex R3 | Gemini R3 |
|---|---|---|---|
| **R2-C3** actually close C3 in code | Wave 4-B additive: `CanonicalMessage::LedgerEntrySigning([u8;32])` + canonical_digest match arm + `transition_ledger_emitter::sign_ledger_entry` + skeleton test #9 | CLOSED | CLOSED |
| **R2-K3** head_t/commit return contradiction | Defer head_t mutation to CO1.7.5+; v1.x owns `ledger_root_t` only; spec § 0/§ 3/§ 5 updated | CLOSED | CLOSED (tighter scope) |
| **R2-C2-CAS** ObjectType::Transition undefined | Spec § 3 → `ObjectType::ProposalPayload` (existing variant) | CLOSED | CLOSED |
| **R2-typo** "8-field" → "9-field" | Spec § 0 corrected | CLOSED | CLOSED |

**4/4 round-2 must-fix items CLOSED**. No new defects introduced.

---

## § 3 Strategic re-affirmations

**Gemini Q4 (NEW v1.2 strategic concerns)**: "v1.2 changes do not introduce new strategic concerns; they increase confidence."
- Concentration of truth (LedgerEntrySigningPayload digest computation in transition_ledger): "desirable separation of concerns; strengthens module boundaries"
- Implementation confidence (test #9 real Ed25519 sign/verify): "lifts implementation confidence to the level required for a PASS"

**Codex Q-5 (new v1.2 issues)**: "No new blocking issue."
- Typed-sign-only invariant preserved (raw `sign_digest` path stays private)
- `pub(crate)` visibility on emitter module is appropriate
- Cross-epoch test correctly models D1 payload-binding threat
- TR manifest entries refreshed correctly

---

## § 4 Test verification (Codex Q-6 independent run)

| Test | Result |
|---|---|
| `cargo test --lib bottom_white::ledger::transition_ledger::` | **9/9 PASS** |
| `cargo test --lib` (full workspace) | **199/0 PASS** |
| `boot::tests::verify_trust_root_passes_on_intact_repo` | PASS |

---

## § 5 Implementation path (now unblocked)

Per spec § 13 estimated scope:
1. **CO1.7-impl proper** (3-5 days, post-PASS): fill `Sequencer` impl, `Git2LedgerWriter` impl, `dispatch_transition` enum match, full `replay_full_transition` impl. ~600-900 LoC + 4 CO1.7.5-stage conformance tests (deferred from skeleton).
2. **CO1.4-extra** (NEW atom; ~1-2 days): CAS index persistence enabling cold-replay. 150-300 LoC + 3-4 tests. C2 mitigation. Required for FullTransition replay across process restart.
3. **CO1.7.5+ wiring** (separate atom): `head_t` mutation when `Git2LedgerWriter` returns commit_sha; integration into `bus.rs`/`kernel.rs` (THIS will need STEP_B per CLAUDE.md "Code Standard").

Total downstream from CO1.7 PASS: 3 atoms estimated 5-9 days for full Wave 6 #1 closure.

---

## § 6 Cumulative cost

| Round | Codex tokens | Gemini tokens | Estimated $ |
|---|---|---|---|
| 1 | 129,132 | 90,232 | ~$7-12 |
| 2 | 216,592 | 215,917 | ~$10-16 |
| 3 | 320,113 | 277,413 | ~$8-14 |
| **CO1.7 audit total** | **665,837** | **583,562** | **~$25-42** |

Cumulative project audit spend: ~$135-202 / $890 mid-budget (~15-23%).

CO1.7 audit cost is in the upper range of project pattern (system_keypair was 1 round PASS @ ~$3-5; spec v1.4 was 4 rounds @ ~$25-40). Real value: 2 spec bugs (Q9 cycle, K2 transplant), 5 spec-vs-code divergences (DIV-1..5), 1 module-dep design improvement (opaque digest variant), all caught before any wasted CO1.7-impl work.

---

## § 7 Sedimented lessons (for memory / case ledger)

1. **Spec-only audit is insufficient for module-integrative atoms**: round-1 paper review missed DIV-1..5 + Q9 cycle + transplant attack vector — all surfaced via type-skeleton smoke. Sedimented: per memory `feedback_smoke_before_batch`, EVERY spec-driven atom should ship a type-skeleton smoke before round-1 audit.

2. **"Claim-vs-code" parity is a real audit failure mode**: round-2 Codex caught spec saying "C3 CLOSED" while system_keypair.rs had no LedgerEntry path. Sedimented: spec patch logs must reference actual git diffs, not aspirational language.

3. **Conservative-wins is sometimes asymmetric in volume**: Gemini PASSed v1.1 + v1.2 with high conviction; Codex CHALLENGEd both with concrete patch lists. The CHALLENGEs were closure-mechanical, not foundational disagreements. Pattern: design quality (Gemini PASS) + implementation discipline (Codex CHALLENGE) is the natural decomposition; both PASS only when both axes are clean.

4. **Module dependency cycles need design awareness up-front**: CanonicalMessage ↔ LedgerEntrySigningPayload was a real cycle; opaque-digest variant resolves cleanly. Sedimented: when a new typed signing primitive crosses module boundaries, design the variant payload up front to be either (a) primitive types, (b) types from a lower module, or (c) opaque digest bytes.

— ArchitectAI synthesis, 2026-04-28; CO1.7 PASS/PASS gate cleared 07:55 UTC.
