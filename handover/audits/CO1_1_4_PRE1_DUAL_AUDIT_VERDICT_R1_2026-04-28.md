# CO1.1.4-pre1 Round-1 Dual External Audit — Merged Verdict

**Date**: 2026-04-28
**Target**: spec v1 (`handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md`) + impl (`src/state/typed_tx.rs` + supporting derive additions in `economy/money.rs`, `cas/schema.rs`, `system_keypair.rs`) — committed `227de72`.
**Auditors**: Codex (gpt-5-codex; 199K tokens) + Gemini 2.5 Pro (113K tokens).
**Conservative rule** (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

---

## § 1 Verdicts

| Auditor | Verdict | Conviction | Top must-fix count |
|---|---|---|---|
| **Codex** | **CHALLENGE** | High | 3 (sched-merged below; many sub-items per § Q-A..Q-J) |
| **Gemini** | **CHALLENGE** | High | 3 |
| **Conservative merged** | **CHALLENGE** | High | — |

**Pre-implementation gate**: NOT cleared. v1.1 patch round required before CO1.7-impl A2 unblocks.

---

## § 2 Convergent must-fix items (both auditors flagged)

| ID | Item | Codex frame | Gemini frame |
|---|---|---|---|
| **C-1** | **Agent-signature domain separation** | Q-E: `AgentSignature` has no signed-payload struct + no domain prefix; comments imply an "exclude signature" digest without a signing payload — impossible | Q7: type-level distinction insufficient; agent-signed payloads MUST add unique domain separators (`v4.agent_sig.work` vs `v4.agent_sig.verify` vs `v4.agent_sig.challenge`) to prevent type confusion attacks |
| **C-2** | **Lock golden fixture hex + expand test coverage** | Q-J: golden tests only assert length=64 + self-stability; don't lock hex; `TypedTx::TerminalSummary` excluded from round-trip / kind / golden tests | Q9: "phase 1 record-only" insufficient for an ABI-defining atom; must hardcode SHA-256 hex; add cross-variant non-collision + BTreeMap permutation independence + zero-value default tests |
| **C-3** | **Schema parity (TerminalSummary 8-field + claim-id newtype + signing-payload structs)** | Q-C must-fix-now: STATE § 1.5 has 8 fields; shipped 3-field placeholder is exactly the ABI being frozen; future additions are decode-breaking. Move tx schema OUT of `system_keypair.rs` into `state::typed_tx`; system_keypair signs opaque digest. Q-B: `claim_id: TxId` should be `claim_id: ClaimId` newtype (STATE speaks in ClaimId; finalization order is by claim_id). | Q6: FinalizeRewardTx system_signature might be REDUNDANT given LedgerEntrySigningPayload signs envelope; spec must explicitly distinguish "agent-to-runtime sign" from "runtime-to-ledger sign" or DROP the field; claim_id should be typed ClaimId not reused TxId |

---

## § 3 Codex-only must-fix items

| ID | Item | Codex citation |
|---|---|---|
| **CX-1** | **TransitionError taxonomy incomplete** | Q-G: missing `SignatureInvalid`, `StakeInsufficient`, predicate failures, target-not-found/not-verifiable, challenge-window-closed, counterexample-insufficient, tool errors, parent-not-accepted etc. STATE § 3 pseudocode raises these; code v1 lists only 10 stub variants. `NotYetImplemented` is acceptable as transition-stage sentinel but not as production error. |
| **CX-2** | **Bincode spec/code parity** | Q-D: STATE § 2.5 wording is wrong vs actual codec — bincode-2 serde encodes enum variant indices as **u32 BE** (not u8 / not `#[repr(u8)]`-controlled), and lengths as **u64 BE** (`usize` materialization). `#[repr(u8)]` does not control serde wire format. `#[serde(transparent)]` newtypes ARE wire-identical to inner values (confirmed). **Sub-implication**: STATE § 2.5 spec must be patched OR the codec settings must change to match (u8 variant indices via custom serde adapter). |
| **CX-3** | **TaskId vs TxId QState index mismatch** | Q-J: typed_tx.rs uses `TaskId` for task references, but current `QState.economic_state_t.{task_markets_t, escrows_t, stakes_t}` are keyed by `TxId` (q_state.rs:201, 161, 182). Future CO P2.x atoms WILL hit this divergence. Must either retrofit QState now or document migration plan. |

## § 4 Gemini-only must-fix items

| ID | Item | Gemini citation |
|---|---|---|
| **GM-1** | **Art 0.2 cold-replay constitutional commitment** | Q4: payload data lives in CAS (referenced via `tx_payload_cid: Cid`); CO1.7 spec § 0 already deferred CAS index persistence to **CO1.4-extra**. Until that ships, cold-restart loses payload data → tape is non-canonical → Art 0.2 violation. v1.1 of CO1.1.4-pre1 must explicitly gate PASS on a concrete commitment to ship CO1.4-extra alongside or before CO1.7-impl A4 (replay_full_transition). Strategic risk: shipping a constitutionally-non-compliant tape layer, even temporarily, is too high. |

---

## § 5 PASS items (both auditors)

- **Codex Q-A (D-1 TxStatus elision)**: PASS with patch note. STATE `step_transition` does NOT read `tx.status`; status is derived from accepted-tx history + ClaimsIndex + per-agent state. Constitutionally sound. Migration path documented.
- **Codex Q-H (HasSubmitter correctness)**: PASS. Work/Verify/Challenge delegate to actual submitter; Reuse returns None correctly (creator is royalty recipient, not submitter). Outer delegation correct.
- **Codex Q-I (atom scope creep)**: PASS with caveat (TerminalSummary placement — but that caveat already covered by C-3).
- **Gemini Q1-Q3, Q5, Q8**: implicit PASS (not in must-fix list). Constitutional alignment, Inv 3 interaction, v4/v4.1 boundary preservation, reconstructibility-via-Q-derivation, forward sustainability all OK.

---

## § 6 v1.1 patch plan

**Estimated scope**: ~300-500 LoC code + 60-100 LoC spec patches + 1 new module (`agent_signing_payloads`) + golden fixture rotation. ~0.5-1 day. Round-2 audit cost: ~$10-20.

| Patch | Maps to | Touches |
|---|---|---|
| **P1**: introduce `WorkSigningPayload` / `VerifySigningPayload` / `ChallengeSigningPayload` structs (subset of each tx, EXCLUDES signature; canonical_digest with domain prefix `b"v4.agent_sig.work.v1"` / etc.); add `verify_agent_signature(sig, payload, agent_pubkey)` API surface | C-1 | typed_tx.rs +~100 LoC; spec § 7 + new § 7.1 |
| **P2**: add `ClaimId(pub TxId)` newtype (spec § 4 + typed_tx.rs); update FinalizeRewardTx.claim_id; specify in spec § 4 that {task_id, solver, reward, royalty edges} are Q-derived at replay (NOT trusted from wire) | C-3 partial (claim-id) | typed_tx.rs + spec § 4 |
| **P3**: migrate `TerminalSummaryTx` from system_keypair.rs (3-field) → state::typed_tx (8-field per STATE § 1.5: tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature); drop the `system_keypair::TerminalSummaryTx`; system_keypair::sign_terminal_summary_tx accepts opaque digest | C-3 main | typed_tx.rs + system_keypair.rs (deletion + reroute) |
| **P4**: complete `TransitionError` taxonomy with all variants invoked in STATE § 3 pseudocode (~12-15 additional variants); keep `NotYetImplemented` as explicit stub sentinel | CX-1 | typed_tx.rs |
| **P5**: lock golden fixture hex (SHA-256 of canonical bytes for each TypedTx variant fixture); add cross-variant non-collision test; add BTreeMap permutation independence test; add zero-value default round-trip; include `TypedTx::TerminalSummary` in all test classes | C-2 | typed_tx.rs tests |
| **P6**: spec patch — STATE § 2.5 wording drift (variant index = u32 BE, lengths = u64 BE; `#[repr(u8)]` does NOT control wire format) — either fix the spec OR add a custom serde adapter forcing u8 discriminants. **Decision required**: cheap (spec patch) vs expensive (codec change forces re-encode of all existing fixtures). Given v1 is brand-new, recommend spec patch + accept u32/u64 sizing. | CX-2 | spec § 2.5 OR new serde adapter |
| **P7**: spec § 9 D-3 → resolved (no longer a divergence; full schema migrated). Remove D-3 row. | C-3 followup | spec § 9 |
| **P8**: spec § 5 add explicit Q-derived note for FinalizeRewardTx fields {task_id, solver, reward, royalty}; spec § 4 commit to making LedgerEntrySigningPayload the SOLE signing point for FinalizeRewardTx (drop FinalizeRewardTx.system_signature OR clarify dual-sign rationale) | C-3 + GM-2 | spec § 4 + § 5 |
| **P9**: spec patch — § 0 add explicit "Art 0.2 cold-replay gate": v1.1 PASS contingent on CO1.4-extra commitment (explicit cross-atom dependency; CO1.7-impl A4 must NOT ship before CO1.4-extra) | GM-1 | spec § 0 |
| **P10**: spec patch — § 9 add D-4 TaskId-vs-TxId-keyed-QState forward-migration plan (CO P2.1 TaskMarket atom owns the QState retrofit; v1.1 documents but does not perform) | CX-3 | spec § 9 + new appendix |

---

## § 7 Round structure forward

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| 1 | CHALLENGE (high) | CHALLENGE (high) | **CHALLENGE** | v1.1 patch round (P1-P10 above) |
| 2 | ⏳ | ⏳ | TBD | re-audit on v1.1; expected PASS or 1-issue CHALLENGE |
| 3+ | … | … | … | iterate to PASS/PASS |

**Pre-implementation gate** (for CO1.7-impl A2-A4): CO1.1.4-pre1 must reach `PASS/PASS` before A2 starts.

---

## § 8 Cumulative cost (this round)

| Auditor | Tokens | Estimated $ |
|---|---|---|
| Codex r1 | 199,200 | ~$5-10 |
| Gemini r1 | 113,295 | ~$3-5 |
| **Round 1 total** | **312,495** | **~$8-15** |

Tracks lower than CO1.7 spec round-1 ($7-12) because spec is shorter. Cumulative project audit spend: ~$143-217 / $890 mid-budget (~16-24%).

---

## § 9 Sedimented lessons (this round)

1. **Single-source-of-truth for tx schemas matters**: TerminalSummaryTx living in `system_keypair.rs` (signer module) leaked into a "frozen" location and made the ABI atom imports dependent on the placeholder. Sedimented: per-typed-tx struct should live in `state::typed_tx`; signers consume opaque digests.

2. **Spec § 2.5 wording drift**: STATE_TRANSITION_SPEC § 2.5 claimed bincode would emit `#[repr(u8)]` discriminants, which is FALSE — that attribute does not control serde wire format. Sedimented: when freezing a wire format, the spec must include the actual codec library version + verified-by-test byte layout, not the LANGUAGE-level repr.

3. **"Record-only" golden fixtures are not golden**: a self-comparison ("two encodes match") is round-trip stability, not ABI freezing. ABI freeze requires hardcoded hex against which any future encode is compared. Sedimented: every ABI-freeze atom MUST lock SHA-256 hex in v1; "phase 1 record-only" deferral is a CHALLENGE smell.

4. **Domain separation is non-negotiable when a 64-byte signature is reused for distinct semantic roles** (agent vs system; work vs verify vs challenge). Type-level distinction at the API surface is necessary but not sufficient; the canonical_digest pre-image must encode the role. Sedimented: when introducing a new signature primitive, the canonical_digest spec must include a stable role-prefix byte string (`b"v4.<actor>.<purpose>.v1"`).

5. **Cold-replay → Art 0.2 commitment is a real cross-atom gate**: declaring "CO1.4-extra is a separate atom" is not enough; the deferred dependency creates a window where the tape is non-canonical. Sedimented: v1.1 must explicitly state the cross-atom ordering constraint (CO1.4-extra MUST ship before CO1.7-impl A4 replay_full_transition) and PASS is contingent on that ordering being honored.

— ArchitectAI synthesis, 2026-04-28; Round-1 closure 2026-04-28; v1.1 patch round opens.
