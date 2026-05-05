# SAFETY_BOUNDARY — TuringOS v4 P7 human escalation, RootBox, sandbox-vs-production discipline

**Status**: TB-17 atom 5 — autonomous Class 0 draft.
**Filed**: 2026-05-05.
**Authority**: TB-17 charter §3 atom 5 + 2026-05-05 architect verdict §B.5 (FR-17.8, FR-17.10) + §B.6 (CR-17.6, CR-17.7, CR-17.10, CR-17.11, CR-17.14) + §B.7 (SG-17.6) + Q6.3 verbatim "human escalation must have timeout + default-safe-action".

---

## §1 Human escalation conditions (FR-17.8 + Q6.3)

A **human escalation** is the transfer of decision authority from automated machinery (oracle / quorum / sequencer) to a human attester (the **Human RootBox**). Escalation is a defined operational state, not a fallback.

### §1.1 Per-tier escalation triggers

| Tier | Triggers (any one fires escalation) |
|---|---|
| **T1** | oracle non-availability with budget exceeded; replay-byte-mismatch; structural invariant violation |
| **T2** | T1 list + multi-version oracle drift; canonical signing-payload mismatch |
| **T3** | T2 list + multi-oracle quorum disagreement (interquartile > threshold); >1 oracle Refuse |
| **T4** | T3 list + adversarial-oracle counterexample; ANY non-trivial verdict (default-on review) |

Cross-references: ORACLE_REQUIREMENTS.md §4.3 (manual escalation conditions); CHALLENGE_COURT_REQUIREMENTS.md §4.4 (resolution non-availability).

### §1.2 Escalation state machine

```
States:
  PENDING_AUTOMATED   — normal flow; oracle / quorum producing verdict
  ESCALATION_OPEN     — escalation triggered; automated machinery froze
  ESCALATION_TIMEOUT  — wallclock budget exceeded; default-safe-action fires
  RESOLVED_HUMAN      — human signature attested; flow resumes
  RESOLVED_DEFAULT    — default-safe-action committed (typically REJECT/BLOCK)

Transitions:
  PENDING_AUTOMATED -> ESCALATION_OPEN     (any trigger from §1.1 fires)
  ESCALATION_OPEN   -> RESOLVED_HUMAN      (human signature attestation arrives)
  ESCALATION_OPEN   -> ESCALATION_TIMEOUT  (wallclock budget exceeded)
  ESCALATION_TIMEOUT -> RESOLVED_DEFAULT   (default-safe-action commit per §3)
```

The state machine state is recorded on-chain (CAS-rooted EscalationStateUpdate objects); replay-deterministic per Art. 0.3.

---

## §2 Per-tier wallclock timeout + default-safe-action (Q6.3 verbatim)

```
Q6.3 "Human escalation" 必须有 timeout 规则
   否则系统可能在 escalation 上永久卡住。需要：
     human_escalation_required
     human_timeout
     default_safe_action
     no-settlement-until-resolution
```

### §2.1 Timeout budgets (per tier)

| Tier | `human_timeout` | `default_safe_action` |
|---|---|---|
| **T1** | 24h | `REJECT_AND_REFUND` (refund any deposits; reject candidate) |
| **T2** | 72h | `REJECT_AND_REFUND` |
| **T3** | 7d | `REJECT_AND_REFUND` (no quorum-overruling default-pass; conservative reject) |
| **T4** | 30d | `REJECT_AND_REFUND` + filed for architect-level review (PROHIBITED PILOT — placeholder) |

### §2.2 Default-safe-action rule (CR-17.14)

```
CR-17.14 verbatim: Human escalation must fail-safe, not fail-open.
```

Concrete: **the default action when a human does not respond within budget is REJECT, never AUTO-PASS.**

This binds even if economically inconvenient. The system's default behavior under uncertainty is to **conservatively refuse**, not to optimistically admit.

### §2.3 No-settlement-until-resolution rule (Q6.3 list)

A task in `ESCALATION_OPEN` or `ESCALATION_TIMEOUT` state has its settlement (per CHALLENGE_COURT_REQUIREMENTS §3) **BLOCKED** until either `RESOLVED_HUMAN` or `RESOLVED_DEFAULT`. Any deposit / bond / reward release is gated on the resolution state.

This binds across all tiers; the mechanism is the SAME blocking primitive that CR-17.4 already requires for challenge-window-open settlement.

### §2.4 Operator visibility

Dashboard surface (atom 5 §4) MUST display:
- count of tasks in `ESCALATION_OPEN`
- count in `ESCALATION_TIMEOUT`
- per-tier escalation rate over rolling N-day window
- alert when any task is approaching `human_timeout` (e.g., 80% of budget elapsed)

Atom 11 conformance battery may add a stub test for the dashboard renderer (defer if it requires Class 3 wire-up; this doc only specs the requirement).

---

## §3 Human RootBox protocol (CR-17.6)

```
CR-17.6 verbatim: No bypass of Human RootBox for high-risk domains.
```

### §3.1 RootBox identity

The **Human RootBox** is the human attester(s) authorized to produce binding `ResolutionAttestation` (per CHALLENGE_COURT_REQUIREMENTS §4.3). Each pilot domain has a designated RootBox identity per §3.2.

For TuringOS v4 D1 Lean pilot (TB-19): the RootBox = the architect (single signer) per the project's solo-researcher single-architect model.

For multi-org deployments (post-TB-26+ futures): RootBox = a multi-sig configuration.

### §3.2 RootBox key management

- Single-signer pilot (TB-19): architect's signing key; rotation per IRREVERSIBLE_ACTION_POLICY §2 subtype #8.
- Multi-sig pilots (future): N-of-M signature scheme; threshold = ⌈M/2⌉+1 (majority).
- Compromise procedure: if a RootBox key compromise is suspected, ALL pending `ESCALATION_OPEN` BLOCK additionally on emergency architect override (constitutional Phase Z′ analog).

### §3.3 RootBox signature payload

```
HumanResolutionAttestation {
    escalation_state_cid: Cid               (the EscalationStateUpdate triggering this)
    decision            : RootBoxDecision   // Accept / Reject / Inconclusive-RequireMoreEvidence
    rationale_cid       : Cid               (CAS-rooted human note; AuditOnly per CR-17.11)
    decided_at_logical_t: LogicalT
    decided_at_wallclock_unix: u64          (forensic; not canonical)
    rootbox_signature   : RootBoxSignature
}
```

`rationale_cid` privacy: per CR-17.11 — **not** broadcast to ordinary Agent read views; agent_autopsies_t-style scoping (TB-15 carry-forward).

### §3.4 No-bypass enforcement

For high-risk domains (T3+, or any T2 domain explicitly flagged as high-risk in DOMAIN_SELECTION_CRITERIA), all settlement decisions require RootBox signature. Sequencer admission of `FinalizeRewardTx` for such domains MUST verify the RootBox signature OR reject with `RootBoxAttestationMissing`.

For low-risk T1/T2 sandbox domains, RootBox is OPTIONAL but always available.

---

## §4 Sandbox / SHADOW / LIVE label discipline (CR-17.10 + carry-forward TB-16 CR-16.7)

Per CR-17.10 (dashboard / readiness reports are materialized views, not source of truth), the operational LABEL of each chain dictates how its outputs may be interpreted.

### §4.1 Three labels

```
SANDBOX  — sandbox-prefixed agents; no real funds; no external action;
           used for development, testing, smoke evidence, M0/M1 MiniF2F harness.
SHADOW   — production-shape agents; real chain; NO settlement to non-sandbox party;
           used to validate production behavior alongside SANDBOX before going LIVE.
LIVE     — production agents; settlement permitted within tier irreversibility profile;
           authorized only after architect explicit ratification per chain.
```

### §4.2 Label admission rules

| From label | To label | Required for promotion |
|---|---|---|
| (none) | SANDBOX | default for any new chain |
| SANDBOX | SHADOW | architect ratification + SG-17.* close per relevant TB |
| SHADOW | LIVE | architect ratification + (≥30d SHADOW evidence) + (no unresolved OBS) + RootBox key audit |
| LIVE | (any other) | architect-only (degradation requires ratification too) |

### §4.3 Label visibility

Dashboard MUST surface the label in §16-style banner (TB-16 carry-forward) for every chain rendering. Any output generated under SANDBOX label MUST NOT be cited in real-world commerce / public benchmark / regulatory submission.

### §4.4 Label CAS contract

```
ObjectType::ChainLabel — single object per chain genesis; immutable post-write
  fields:
    chain_genesis_cid : Cid
    label             : Sandbox | Shadow | Live
    authorized_by     : ArchitectSignature  // architect key for SHADOW+LIVE
    authorized_at_logical_t: LogicalT
```

Label transitions create new ChainLabel objects (append-only DAG); the latest label per chain is the materialized-view "current" label.

---

## §5 Per-domain safety profile template

For each candidate domain in DOMAIN_SELECTION_CRITERIA.md §2, the SAFETY_BOUNDARY profile collects:

```
Per-domain safety profile {
    domain_id              : DomainId
    tier                   : RiskTier  (T1/T2/T3/T4 from DOMAIN_SELECTION_CRITERIA)
    label                  : Sandbox | Shadow | Live
    escalation_triggers    : Vec<EscalationTrigger>  (§1.1 enumerated)
    human_timeout_seconds  : u64                      (§2.1)
    default_safe_action    : DefaultSafeAction        (§2.2 default REJECT_AND_REFUND)
    rootbox_required       : bool                     (§3.4)
    rootbox_identity       : RootBoxIdentityRef
    privacy_class          : PrivacyClass             (§6 below)
    irreversibility_floor  : IrreversibilityFloor     (cross-ref atom 6 §3 allowlist)
}
```

### §5.1 D1 Lean pilot safety profile (concrete)

```
domain_id              : "lean_minif2f_v0"
tier                   : T2
label                  : Sandbox  (TB-19 pilot starts SANDBOX; SHADOW/LIVE per architect)
escalation_triggers    : [oracle_unavailable_72h, replay_byte_mismatch,
                          canonical_signing_payload_mismatch]
human_timeout_seconds  : 259200      (72h per §2.1 T2)
default_safe_action    : REJECT_AND_REFUND
rootbox_required       : false       (T2 sandbox; promote to true on SHADOW)
rootbox_identity       : "architect_single_signer"
privacy_class          : Public      (Lean proofs are public artifacts; no
                                       human-PII-class private detail)
irreversibility_floor  : Reversible  (proof outputs are inert text; no external API write)
```

---

## §6 Privacy / raw-log shielding (FR-17.10 + CR-17.11)

```
CR-17.11 verbatim: Raw real-world evidence must not be broadcast into
                   ordinary Agent read views.
```

Carry-forward from TB-15 AutopsyCapsule + TB-11 EvidenceCapsule privacy infrastructure:

### §6.1 Privacy class taxonomy

```
PrivacyClass {
    Public          — output is intended for public broadcast (e.g., MiniF2F proof object)
    AuditOnly       — output is read-only by audit role (TB-11/15 default)
    HumanPII        — output may contain personally-identifying info; access restricted
    AdversaryHidden — output deliberately obscured from challengers (T4 only)
}
```

### §6.2 Privacy enforcement

- Per-domain `privacy_class` per §5 profile.
- Sequencer admission validates that `evidence_cids` referenced from L4 entries are PUBLIC class only.
- Non-PUBLIC evidence MUST live in `private_detail_cid` (TB-15 surface) AND only render to audit-role projections.
- Dashboard projections honor privacy class; no raw private detail in default render.

### §6.3 Real-world domain extension

For real-world tasks, `evidence_cids` may legitimately contain user-submitted content (e.g., a document the user wants verified). Such content MUST be classified at admission:
- If classified `AuditOnly` or `HumanPII`: admission proceeds with private_detail_cid path; sequencer NEVER places the raw bytes in an L4 entry.
- If classified `Public`: standard path.
- Misclassification = OBS-class event; emit autopsy + alert dashboard.

---

## §7 Failure modes catalogue

Per architect §B.5 / §B.6 / Q6.3:

| Failure | Trigger | Response |
|---|---|---|
| Oracle non-available beyond budget | ORACLE_REQUIREMENTS §6 | escalation → §2 budget → §2.2 default-safe-action |
| Multi-oracle disagreement | ORACLE_REQUIREMENTS §4.3 | escalation → human RootBox |
| Challenge-window settlement gate violated | sequencer attempts FinalizeReward with open ChallengeTx | rejection class FinalizeBlockedByOpenChallenge (existing); audit alert |
| Replay byte-mismatch | atom 11.B fc_alignment_conformance fails | RootBox immediately notified; Phase Z′ analog if confirmed |
| RootBox key compromise suspected | external operator alert | ALL escalations pause; emergency architect override |
| ChainLabel attempted illegal transition (e.g., SANDBOX → LIVE without architect signature) | sequencer admission of ChainLabel tx with invalid signature | rejection L4.E + alert |
| Privacy misclassification | raw private bytes detected in L4 admission | rejection + autopsy + audit alert |

Each failure emits an AgentAutopsyCapsule (or chain-level CapsuleEquivalent for non-agent failures) per TB-15 substrate.

---

## §8 Cross-references

- ORACLE_REQUIREMENTS.md atom 3 §4 (disagreement → escalation); §6 (non-availability protocol); §8 (attack surface).
- CHALLENGE_COURT_REQUIREMENTS.md atom 4 §3 (settlement gating); §4 (resolution authority).
- IRREVERSIBLE_ACTION_POLICY.md atom 6 §3 (allowlist criteria); §5 (verdict matrix integrates with `default_safe_action`).
- DOMAIN_SELECTION_CRITERIA.md atom 2 §3 (tier × oracle); §6.2 (D1 pilot inputs to §5.1 profile).
- 2026-05-05 architect verdict §B.5 (FR-17.8, FR-17.10) + §B.6 (CR-17.6, CR-17.7, CR-17.11, CR-17.14) + §B.7 (SG-17.6) + Q6.3 verbatim.
- TB-15 AgentAutopsyCapsule + privacy primitives.
- TB-11 EvidenceCapsule + CapsulePrivacyPolicy::AuditOnly.
- TB-16 CR-16.7 sandbox label carry-forward.
