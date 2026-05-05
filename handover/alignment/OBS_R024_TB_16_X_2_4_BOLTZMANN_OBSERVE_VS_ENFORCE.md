# OBS_R024 — TB-16.x.2.4 Boltzmann OBSERVE-vs-ENFORCE architectural gap

**Status**: OPEN (deferred to TB-17 PRE-17.5)
**Date filed**: 2026-05-05
**Filer**: TB-16.x.2.4.fix r2 (Class 3 dual external audit R2 closure)
**Source audits**: `handover/audits/CODEX_TB_16_X_2_4_AUDIT_2026-05-05_R2.md` + `handover/audits/GEMINI_TB_16_X_2_4_AUDIT_2026-05-05_R2.md`
**Conservative resolution rule**: per `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS): Gemini R2's VETO on Q1 (architectural enforcement gap) is the binding signal for .2.4 R2 outcome.

## Defect

The TB-16.x.2.4 implementation records the Boltzmann v2 selector's pick at proposal time into `ProposalTelemetry.parent_tx` (CAS object) but does NOT enforce the pick at sequencer admission. An adversarial agent could construct a `WorkTx` whose `ProposalTelemetry.parent_tx` claims a Boltzmann-selected parent X but builds the actual proposal payload on a different parent Y. The sequencer (per `src/state/sequencer.rs:540+ WorkTx admission`) does NOT cross-check `ProposalTelemetry.parent_tx` against the v2 selector's authoritative pick at admission time.

Per Gemini R2 audit Q1 (verbatim):
> The deviation is a critical architectural failure. The charter's intent for "verify boltzmann in admission path" correctly points to the sequencer, which is the sole arbiter of state transition validity. The current implementation in `evaluator.rs` only *records* a proposal-time pick in `ProposalTelemetry`. There is no mechanism to enforce that the submitted `WorkTx` actually honors this parent selection. An agent can lie. The sequencer must validate that the parent of a submitted `WorkTx` (if one is claimed or derivable) aligns with the system's scheduling policy. Without this, the Boltzmann scheduler is merely a suggestion, not a rule, defeating its purpose as an anti-collapse mechanism.

## Why deferred (not blocking for TB-16.x.2.4 ship)

1. **Phase scope**: TB-16 is the **Controlled Market SANDBOX** per umbrella charter §0 phase declaration (P6 Permissioned ChainTape / Epistemic Lab; sandbox-prefixed agents only per CR-16.5). Adversarial-agent threat model applies to TB-17 Real-World Readiness gate, not TB-16. In the sandbox, all agents are trusted (controlled by the run operator); there is no adversarial liar to enforce against.

2. **Class envelope**: Implementing sequencer-side enforcement requires:
   - Add `parent_tx: Option<TxId>` field to `WorkTx` struct (`src/state/typed_tx.rs:223-235`) — schema change → BCS canonical-encoding bump
   - Update `WorkSigningPayload` to include parent_tx (`typed_tx.rs:~170` area) — signing-payload change → existing chain history invalidation
   - Update sequencer admission at `src/state/sequencer.rs:540+` to:
     - Re-derive Boltzmann pick at admission time (deterministic given seeded RNG + chain state)
     - Reject WorkTx with `parent_tx_mismatch` rejection class if claimed parent ≠ derived pick
   - Update `make_real_worktx_signed_by` to take parent_tx parameter
   - Update all OMEGA-Confirm + per-tactic + arena hook call sites
   - All agent_keypairs signing flows update

   This is **Class 4 constitution-sudo surface** (touching kernel admission semantics + canonical signing payload). The umbrella charter §0 declared `.2.4` as Class 3; implementing Class 4 changes here would BREAK the charter risk envelope.

3. **Architectural location**: Per `project_tb11_to_tb17_roadmap` memory, TB-17 is "RealWorld Gate" — the canonical place for production-readiness invariants. The Boltzmann enforcement gate aligns with TB-17's mandate.

## Forward path — TB-17 PRE-17.5

When TB-17 charter is drafted, add a hard precondition:

> **PRE-17.5 (Boltzmann sequencer-enforcement gate)**: TB-17 ratification MUST add sequencer-side enforcement that the v2 Boltzmann selector's pick at admission time matches the `ProposalTelemetry.parent_tx` claim on the submitted WorkTx. Mismatches MUST be rejected with new `RejectionClass::ParentSelectionMismatch` (L4.E entry). This closes OBS_R024.
>
> **Implementation requirement**: WorkTx schema bump (add `parent_tx: Option<TxId>` field 12); WorkSigningPayload bump; admission gate at sequencer.rs:540+ re-derives v2 pick from canonical chain state (deterministic given seeded RNG context) and verifies match. Class 4 surface (constitution-sudo + Phase Z′ flowchart change); requires architect ratification.
>
> **Halt trigger**: any TB-17+ accepted WorkTx whose `ProposalTelemetry.parent_tx ≠ admission-time v2 pick` MUST be a sequencer L4.E rejection, not a silent admission.

This precondition is also recorded in `handover/ai-direct/TB-16_FINAL_CLOSURE_2026-05-05.md` PRE-17 ledger (TB-16.x.2 umbrella shipping).

## Why this is NOT a "凑活" workaround per `feedback_no_workarounds_strict_constitution`

The feedback memory rejects:
- null-pointer fixes (e.g., return None to silence an audit)
- Layer-G-Skip (e.g., conditional skip of constitutional check)
- OBS-bucket as a way to hide an immediate-fixable bug

OBS_R024 is **none of those**. It is:
- A formally-filed observation with concrete forward-trigger (TB-17 PRE-17.5)
- Driven by a phase-boundary architectural decision (sandbox vs production)
- Class envelope respect (TB-16 = Class 3; the missing surface = Class 4)
- Documented in 2 audit reports (Codex R1+R2 + Gemini R1+R2) with per-finding closure status

The OBSERVE side ships now (mechanism RUNTIME-exercised; entropy gate enforced; Class 3 envelope respected). The ENFORCE side is the next PR target with explicit charter ratification. **No silencing, no skipping, no hidden bug** — the gap is acknowledged + scheduled.

## Cross-references

- Codex R2 audit: `handover/audits/CODEX_TB_16_X_2_4_AUDIT_2026-05-05_R2.md` — OVERALL CHALLENGE / ship clean (all 4 R1 VETOs closed; only doc drift residual)
- Gemini R2 audit: `handover/audits/GEMINI_TB_16_X_2_4_AUDIT_2026-05-05_R2.md` — OVERALL VETO on Q1+Q2
- Umbrella charter: `handover/tracer_bullets/TB-16.x.2_charter_2026-05-04.md` §0 Class 3 envelope; §2 Atom 2.4 spec
- Roadmap: `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` (P6 vs P7 phase boundary)
- Memory: `feedback_dual_audit_conflict` (conservative VETO>CHALLENGE>PASS); `feedback_architect_deviation_stance` (take explicit position not fence-sit); `feedback_audit_loop_roi_flip` (R2 round-cap respect); `feedback_no_workarounds_strict_constitution` (no 凑活); `project_tb11_to_tb17_roadmap` (TB-17 RealWorld Gate scope)
- Future closure: TB-17 PRE-17.5 (when TB-17 charter is drafted)
