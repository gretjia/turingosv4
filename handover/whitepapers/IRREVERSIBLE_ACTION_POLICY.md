# IRREVERSIBLE_ACTION_POLICY — TuringOS v4 P7 irreversibility ban + ≥8 candidate-action verdicts

**Status**: TB-17 atom 6 — autonomous Class 0 draft.
**Filed**: 2026-05-05.
**Authority**: TB-17 charter §3 atom 6 + 2026-05-05 architect verdict §B.5 (FR-17.9) + §B.6 (CR-17.3) + §B.7 (SG-17.8 ≥8 candidate actions) + Q6.2 verbatim 8-subtype list.

---

## §1 Irreversibility taxonomy

### §1.1 Definition

An **action** is **irreversible** iff it produces a **state change in a system outside TuringOS's chain-canonical state** that **cannot be undone** by any subsequent TuringOS action. Three sub-classes:

```
Reversible     — undo is possible by deterministic chain action; e.g., scratch-file write
Compensable    — undo is impractical but the harm is bounded and refundable; e.g., bond
                  forfeit (financial compensation possible)
Irreversible   — no TuringOS action can undo the consequence; external world is changed
                  in a way TuringOS does not control
```

### §1.2 Why this matters

TuringOS's Tape Canonical axiom (Art. 0.2) and Replay Determinism axiom (Art. 0.3) guarantee that everything **inside** the chain is reversible: any state can be recomputed, any decision audited, any rejection re-examined. **Irreversibility is the boundary** between TuringOS's well-controlled interior and the uncontrollable exterior.

P7 readiness requires that TuringOS **never crosses this boundary by accident** — every irreversible action must be a deliberate, ratified, sandbox-aware decision.

---

## §2 Subtype enumeration (architect Q6.2 verbatim — ≥8 subtypes)

Per architect §B.3 Q6.2 verbatim: "不可逆行为不只是'外部动作'。至少包括：[8 subtypes]". This is the authoritative enumeration; future TBs may add new subtypes but MUST NOT remove these.

```
Subtype #1: external_api_write
Subtype #2: payment
Subtype #3: publication
Subtype #4: message_sending
Subtype #5: physical_actuation
Subtype #6: deletion
Subtype #7: legal_medical_financial_advice
Subtype #8: credential_key_rotation
```

### §2.1 Subtype #1 — `external_api_write`

- **Definition**: HTTP POST/PUT/DELETE (or equivalent state-mutating call) to a non-sandbox endpoint.
- **Irreversibility class**: Irreversible (cannot guarantee remote system rollback).
- **Examples**: GitHub PR creation, public-facing webhook, third-party SaaS state mutation.
- **Default verdict**: `deny` for SANDBOX/SHADOW; `require-human` for LIVE T2; `deny` for T3+.

### §2.2 Subtype #2 — `payment`

- **Definition**: funds transfer; on-chain (cryptocurrency) or off-chain (bank/PSP).
- **Irreversibility class**: Irreversible (chain finality OR settlement finality).
- **Examples**: cryptocurrency transfer, fiat-via-Stripe payout, bounty release.
- **Default verdict**: `deny` for ALL labels in TB-17 (no real funds in TB-17); TB-19+ MAY allow `require-delay` for delayed-settlement small-bounty per architect §B.10.3 ratification.

### §2.3 Subtype #3 — `publication`

- **Definition**: emission of text/content to a public-broadcast channel.
- **Irreversibility class**: Irreversible (cannot guarantee deletion from third-party caches / archives / re-shares).
- **Examples**: arXiv paper submission, public GitHub commit to a public repo, social-media post, press release.
- **Default verdict**: `require-delay` for SANDBOX (publication of test artifacts permitted with sandbox label); `require-human` for SHADOW; `require-human` for LIVE T1/T2; `deny` for T3+.

### §2.4 Subtype #4 — `message_sending`

- **Definition**: outbound communication to a non-sandbox party (email, Slack, Discord, IM).
- **Irreversibility class**: Compensable to Irreversible (recipient may delete OR forward; cannot guarantee).
- **Examples**: notification email, automated DM, calendar invite.
- **Default verdict**: `require-human` for SANDBOX/SHADOW external messaging; `require-human` for LIVE T1/T2; `deny` for T3+.

### §2.5 Subtype #5 — `physical_actuation`

- **Definition**: command issued to a physical device (robotics, IoT, hardware).
- **Irreversibility class**: Irreversible (physical-world causation).
- **Examples**: robot arm motion, smart-lock unlock, voltage-controller setting, vehicle command.
- **Default verdict**: `deny` for ALL labels and ALL tiers, except via explicit architect-authored override (Phase Z′ analog).

### §2.6 Subtype #6 — `deletion`

- **Definition**: removal/destruction of a record/file/commit/branch.
- **Irreversibility class**: Irreversible (without backup) or Compensable (with backup + restore SLA).
- **Examples**: `rm -rf` of evidence dir, `git push --force` to public branch, database row delete, account closure.
- **Default verdict**: `require-human` for SANDBOX/SHADOW (destructive ops always reviewed); `deny` for LIVE except by architect-key signed CommitDeletionTx; **NEVER auto-deletion** of canonical chain state.

### §2.7 Subtype #7 — `legal_medical_financial_advice`

- **Definition**: regulated-domain output emitted to a human user (advisory role).
- **Irreversibility class**: Irreversible (recipient may act on it; legal/regulatory liability).
- **Examples**: medical diagnosis, legal opinion, financial trade recommendation, regulated-product advice.
- **Default verdict**: `deny` for ALL TuringOS labels and ALL tiers without explicit external-counsel ratification + regulated-jurisdiction approval. PROHIBITED PILOT (DOMAIN_SELECTION_CRITERIA §2.5 ban list).

### §2.8 Subtype #8 — `credential_key_rotation`

- **Definition**: change of a private-key / API-token / SSH-key / authentication-credential.
- **Irreversibility class**: Compensable (old credential becomes invalid; new path forward) BUT carries Irreversibility-style risk if mismanaged (lock-out, compromise window).
- **Examples**: architect signing-key rotation, CAS oracle binary signing-key rotation, RootBox key rotation.
- **Default verdict**: `require-human` for ALL labels; SHOULD only be triggered via deliberate operator-attested rotation procedure, NEVER as a side-effect of agent action.

---

## §3 Catalogue of forbidden actions (architect §8.6 verbatim, original TB-13→TB-17 directive)

These were stated in the original 2026-05-03 architect directive §8.6 and are preserved verbatim:

```
Architect §8.6 verbatim BAN list:
  No live external action.
  No real-world payout without oracle.
  No medical/legal/financial high-risk domain.
  No autonomous deployment.
  No public chain settlement.
  No agent-only arbitration.
  No irreversible external actuation.
  No real-world pilot before report approval.
```

Mapping to subtypes from §2:

| Architect ban | §2 subtype |
|---|---|
| No live external action | #1, #5 |
| No real-world payout without oracle | #2 + ORACLE_REQUIREMENTS.md §1 |
| No medical/legal/financial high-risk domain | #7 |
| No autonomous deployment | #1, #6 |
| No public chain settlement | #2 |
| No agent-only arbitration | (cross-cuts; CR-17.7 enforces; not a §2 subtype but a process rule) |
| No irreversible external actuation | #5 |
| No real-world pilot before report approval | (cross-cuts; SG-17.7 enforces; this whole TB-17) |

---

## §4 Allowlist criteria (when can an action be permitted?)

An action is **allowable** (verdict `allow` or `require-delay`) iff ALL of the following hold:

1. **Sandbox label** (CR-16.7 carry-forward + SAFETY_BOUNDARY §4 label discipline) — OR — explicit architect ratification for SHADOW/LIVE.
2. **Reversibility-bounded**:
   - Reversible (instant undo possible) — `allow`
   - Compensable + delayed-settlement window per FR-17.7 — `require-delay`
   - Irreversible — at most `require-human`, never `allow`
3. **Tier-compatible** (DOMAIN_SELECTION_CRITERIA.md §4 tier × irreversibility matrix).
4. **Audit-replay-able** (CR-17.10 + Art. 0.3): if the action leaves an external trace, the action MUST be recorded in chain (`ExternalActionTx` or equivalent) for replay/audit.
5. **Privacy-compatible** (CR-17.11): if the action discloses information, the privacy class of the disclosed content must be Public.
6. **OracleAttested** for any action with non-trivial verification asymmetry.

Anti-pattern (forbidden): allowing an action because "it's just a test" without the SANDBOX label being constitutionally bound.

---

## §5 Test fixture matrix (architect SG-17.8 ≥8 candidate-action verdicts; all four verdict classes)

Per architect SG-17.8 verbatim: "IRREVERSIBLE_ACTION_POLICY.md tests at least 8 candidate actions: allow / deny / require-human / require-delay."

| # | Candidate action | Subtype | Tier | Label | Verdict | Rationale (CR cite) |
|---|---|---|---|---|---|---|
| 1 | Write a Lean proof object to local CAS storage in sandbox arena | (none — pure compute) | T2 | SANDBOX | **allow** | not in §2 subtype; pure compute artifact; CR-17.10 |
| 2 | Submit GitHub PR with TB-17 charter changes from SANDBOX run | #1 (external_api_write) | T2 | SANDBOX | **deny** | external_api_write to non-sandbox; CR-17.3 |
| 3 | Auto-rotate the architect signing key after 90 days | #8 (credential_key_rotation) | (any) | (any) | **require-human** | architect-attested rotation only; never auto; CR-17.6 |
| 4 | Release 0.5 USD bounty to a TB-19 pilot solver after challenge window closes | #2 (payment) | T2 | LIVE | **require-delay** | delayed-settlement small-bounty per architect §B.10.3 ratification only; CR-17.4 |
| 5 | Auto-publish a successful Lean proof to the project arXiv preprint repo | #3 (publication) | T2 | LIVE | **require-human** | publication-class; human review + sandbox label discipline; CR-17.6 |
| 6 | Issue motion command to a physical robot to demonstrate solver capability | #5 (physical_actuation) | (any) | (any) | **deny** | physical_actuation banned; CR-17.3 + DOMAIN_SELECTION_CRITERIA §2.5 |
| 7 | Auto-delete an obsolete evidence directory > 30 days old | #6 (deletion) | (any) | (any) | **require-human** | destructive op; never auto; CR-17.6 + Art. 0.2 |
| 8 | Send a Slack DM to architect when atom 11 conformance test fails | #4 (message_sending) | T1 | SANDBOX | **require-human** | external message_sending; sandbox-label needs explicit operator authorization for the DM channel; CR-17.6 |
| 9 | Render the audit dashboard SVG for current chain state to local file | (none — pure read) | (any) | (any) | **allow** | pure read-render; no §2 subtype triggered; CR-17.10 |
| 10 | Auto-medical-advice when LLM agent recognizes a health-related TaskSpec | #7 (legal_medical_financial_advice) | T4 | (any) | **deny** | regulated-domain ban; CR-17.3 + DOMAIN_SELECTION_CRITERIA §2.5 |

**Verdict-class coverage check**:
- `allow`: rows 1, 9 ✓
- `deny`: rows 2, 6, 10 ✓
- `require-human`: rows 3, 5, 7, 8 ✓
- `require-delay`: row 4 ✓

All four classes exercised. ≥8 candidate-actions present (10 rows, satisfying SG-17.8 ≥8).

---

## §6 Admission gate spec (TB-18+ implementation hook)

When TB-18+ implementation lands, the irreversibility check integrates with sequencer admission as follows:

```
For each typed-tx that may produce an external effect:
  1. Identify the action subtype (§2 enumeration).
  2. Look up tier (DOMAIN_SELECTION_CRITERIA §3) + label (SAFETY_BOUNDARY §4) for chain.
  3. Apply tier × irreversibility matrix (DOMAIN_SELECTION_CRITERIA §4) →
     compute verdict ∈ {allow, deny, require-human, require-delay}.
  4. Apply verdict:
     - allow:           admit
     - deny:            reject to L4.E (RejectionClass::IrreversibilityDenied)
     - require-human:   freeze in ESCALATION_OPEN (SAFETY_BOUNDARY §1.2);
                        admission gated on RootBox attestation
     - require-delay:   admit conditionally; settlement BLOCKED until delay elapses
```

This gate is **NOT** implemented in TB-17. TB-18 charter is responsible for wiring it. The spec here is the contract that future implementation MUST satisfy.

---

## §7 Carry-forward integration

### §7.1 TB-15 AutopsyCapsule extension

If an action is admitted with `require-delay` or `require-human` verdict and it later results in a chain-detected violation (e.g., delay window violated, or RootBox signature missing at finalization), the relevant agent gets an AgentAutopsyCapsule with:

```
LossReasonClass extension:
  IrreversibilityViolation  (NEW; spec-only in TB-17; enum bump in TB-18+)
```

### §7.2 TB-16 CR-16.7 sandbox label carry-forward

The SANDBOX label discipline already enforced by TB-16 (CR-16.7) is the substrate this policy builds on. SAFETY_BOUNDARY §4 ratifies the three-label model (SANDBOX / SHADOW / LIVE) and this policy uses the labels as input to the §6 admission gate.

### §7.3 D1 Lean pilot

For DOMAIN_SELECTION_CRITERIA §6 D1 pilot:
- Subtype #2 (payment): TB-19 pilot starts SANDBOX with NO real funds; potential SHADOW/LIVE relaxation per architect ratification.
- Subtype #3 (publication): proof outputs are inert text artifacts; arXiv-style publication PROHIBITED in TB-19; SHADOW/LIVE may relax with `require-human`.
- Other subtypes: not triggered by D1 task shape.
- Hence D1 pilot exercises this policy minimally; its primary TuringOS exercise is the economic + audit + market loop, not irreversibility frontiers.

---

## §8 Cross-references

- DOMAIN_SELECTION_CRITERIA.md atom 2 §3 + §4 (tier × oracle / tier × irreversibility matrices); §2.5 ban list.
- SAFETY_BOUNDARY.md atom 5 §4 (label discipline); §1.2 escalation state machine; §3 RootBox.
- CHALLENGE_COURT_REQUIREMENTS.md atom 4 §3 (settlement gating).
- ORACLE_REQUIREMENTS.md atom 3 §1 (oracle attestation; ties to Allowlist criterion #6).
- 2026-05-05 architect verdict §B.3 Q6.2 (8-subtype verbatim) + §B.5 (FR-17.9) + §B.6 (CR-17.3) + §B.7 (SG-17.8 ≥8 verdicts).
- 2026-05-03 architect §8.6 verbatim BAN list (carry-forward).
- TB-15 AgentAutopsyCapsule + LossReasonClass extension.
- TB-16 CR-16.7 sandbox label.
- Constitution Art. 0.2 (Tape Canonical) + Art. 0.3 (Replay Determinism) + Art. III.1-4.
