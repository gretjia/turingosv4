# CHALLENGE_COURT_REQUIREMENTS — TuringOS v4 P7 challenge & resolution architecture

**Status**: TB-17 atom 4 — autonomous Class 0 draft.
**Filed**: 2026-05-05.
**Authority**: TB-17 charter §3 atom 4 + 2026-05-05 architect verdict §B.5 (FR-17.6, FR-17.7) + §B.6 (CR-17.4, CR-17.7) + §B.7 (SG-17.5).

---

## §1 Challenge window definitions (FR-17.6)

A **challenge window** is the bounded interval between admission of a candidate solution and the moment its outcome becomes irrevocable. During the window, qualified parties may submit `ChallengeTx` (TB-13 surface) referencing the admitted solution + evidence per §2; resolution proceeds per §4.

### §1.1 Per-tier window minimums

(Cross-reference DOMAIN_SELECTION_CRITERIA.md §3 tier table; ORACLE_REQUIREMENTS.md §8.4 latency budgets.)

| Tier | Minimum window | Rationale | Settlement default |
|---|---|---|---|
| **T1** | 1 logical step (immediate-after-admission) | predicate is total + cheap; spurious challenges are easy to dismiss | settle in same step |
| **T2** | logical-step bounded (config; default ≥ 8 steps in sandbox; ≥ 24h wallclock in production) | Lean / Coq verifier output is canonical; window length protects against transient oracle outage | delayed settlement per FR-17.7 |
| **T3** | wallclock-bounded ≥ 72h | multi-oracle quorum + manual review possible; quorum disagreement may need follow-up evidence | delayed settlement; partial release acceptable per pilot architect ratification |
| **T4** | wallclock-bounded ≥ 7d AND human review default-on | adversarial-tier; cannot pre-commit minimum | settlement BLOCKED until human signature |

**Window override**: a TaskSpec MAY request a longer-than-default window (always allowed); shorter windows require **architect ratification** (CR-17.4 enforcement).

### §1.2 Logical-step vs wallclock semantics

- **Logical-step windows** (T1/T2 sandbox): measured in chain-tape advance count; deterministic; replayable.
- **Wallclock windows** (T2 production / T3 / T4): measured by externally-attested timestamps written to L4 (TB-13 already wires `attested_at_wallclock_unix` for ChallengeTx evidence).

The chain-canonical state advances on logical-step windows; wallclock windows are enforced by oracle/human attestations carried IN the chain (so windows are still tape-derived per Art. 0.2).

---

## §2 Evidence requirements (FR-17.6 + Art. 0.2)

### §2.1 Evidence rooting rule

All challenge evidence MUST be:
- **CAS-rooted** (Cids referenced from L4 / L4.E entries).
- **Replayable** (re-fetch + recompute reproduces same Cid).
- **Off-chain-raw-log dependency forbidden** per Art. 0.2 (no parallel ledger; cf. OBS_R022 ruling).

```
ChallengeTx fields (TB-13 baseline + this charter additions):
  challenger_id          : AgentId
  challenged_solution_cid: Cid       (L4 or CAS reference to admitted candidate)
  evidence_cids          : Vec<Cid>  (≥1; this section §2.2 catalog)
  alleged_violation      : ViolationClass
  bond                   : MicroCoin (challenge bond; §3 protocol)
  signature              : AgentSignature
```

### §2.2 Evidence type catalog

Per oracle tier, the admissible evidence types differ:

| Tier | Admissible evidence | Cid type |
|---|---|---|
| T1 | counter-input that produces opposite verdict | TaskSpec-or-CandidateSolution Cid |
| T2 | counter-proof object / counter-corpus-passage Cid | proof file or corpus passage |
| T3 | adverse oracle attestation Cid; OR explicit oracle-disagreement quorum violation | OracleAttestation Cid |
| T4 | (PROHIBITED PILOT — placeholder for architect-authored design) | n/a |

Evidence Cids resolved at admission time; missing evidence → `EvidenceUnresolvable` rejection to L4.E (NOT silent admission; CR-17.14).

### §2.3 Evidence schema validation

Per ORACLE_REQUIREMENTS.md §8.6: evidence schema is per-oracle-defined; admission validates against the schema; malformed evidence → L4.E `EvidenceMalformed`. Atom 11 conformance battery may add a fixture for the schema validator if a test scaffold is straightforward (defer if schema-validator implementation requires Class 3 wire-up — TB-19+ scope).

---

## §3 Settlement-after-challenge protocol (CR-17.4 + FR-17.7)

```
CR-17.4 verbatim: No settlement before challenge window closes.
```

### §3.1 Sequencer-side enforcement

- `FinalizeRewardTx` admission BLOCKED while challenge window for the underlying WorkTx is open.
  - `FinalizeRewardTx` for solution X attempted with open ChallengeTx targeting X → rejection class `FinalizeBlockedByOpenChallenge` (existing TB-13 / TB-14 surface).
  - This is already enforced; TB-17 ratifies it as the production-spec-level invariant.
- For T3+: multi-oracle quorum gate must produce `OracleQuorumVerdict` BEFORE settlement is admissible.
- Sandbox arena (TB-16) shipped this enforcement; D1 pilot (TB-19) inherits it.

### §3.2 Per-tier delayed settlement

Per FR-17.7: settlement = release of any deposit / bond / reward to a real-world counterparty.

| Tier | Settlement timing |
|---|---|
| T1 | 1 logical step post-window-close |
| T2 | 1 logical step in sandbox; ≥ 24h wallclock in production |
| T3 | quorum verdict + (≥ 72h wallclock) — whichever later |
| T4 | quorum verdict + human signature + (≥ 7d wallclock) |

**No real funds in TB-17.** Sandbox-prefix labels (CR-16.5 carry-forward) prevent any payout to a non-sandbox party. TB-19+ MAY relax to delayed-settlement small-bounty per architect §B.10.3 — that relaxation requires its own architect ratification.

### §3.3 Partial release / staged settlement (T3+ option)

For T3 tasks that may have legitimate intermediate verifiable milestones, partial release is **permitted iff explicitly ratified at TaskSpec authoring time**. Default = single-shot settlement at window-close.

Partial release MUST satisfy:
- Each release stage has its own challenge sub-window.
- Total released across stages bounded by total bond + reward.
- Earlier stage's release does not waive later challenges.

(Spec only; not implemented in TB-17.)

---

## §4 Resolution authority (CR-17.7 + ORACLE_REQUIREMENTS §4)

### §4.1 Authority hierarchy

```
1. Deterministic predicate (T1/T2 oracle)
   - If oracle reaches Verified or Refuted, that IS the resolution.
   - Challenge merely re-invokes the oracle on the counter-evidence.

2. Multi-oracle quorum (T3)
   - Quorum verdict per ORACLE_REQUIREMENTS §4.1.
   - Disagreement (interquartile range > threshold) → escalate to step 3.

3. Human-attested resolution (T3 unresolved + T4 always)
   - Per FR-17.8 + atom 5 SAFETY_BOUNDARY.
   - Human signature is canonical; written to L4 as ResolutionAttestation.

4. Architect override (Phase Z′ analog)
   - Reserved for systemic-bug discovery (e.g., oracle-binary compromise).
   - Requires Phase Z′ ratification per constitution.
```

### §4.2 Agent-only arbitration FORBIDDEN (CR-17.7 verbatim)

```
CR-17.7: No agent-only arbitration.
```

An agent cannot adjudicate a challenge it has no oracle / human backing for. Even in the sandbox, multi-agent disagreement REQUIRES either oracle invocation or human escalation; consensus among LLM agents is **NOT** acceptable as resolution authority.

This binds:
- No JudgeAI agent may produce a binding verdict (JudgeAI is veto-only per Art. V.1).
- No "majority-of-agents" verdict admissible.
- No agent-vote-based market settlement (CR-17.5 carry-forward; price-as-truth forbidden).

### §4.3 Resolution attestation contract

```
ResolutionAttestation {
    challenge_tx_cid    : Cid          (the ChallengeTx being resolved)
    resolved_verdict    : ResolvedVerdict  // ChallengerWins / DefenderWins / Inconclusive
    resolution_authority: ResolutionAuthority  // OraclePredicate / OracleQuorum / HumanAttestation / ArchitectOverride
    authority_evidence  : Vec<Cid>     // OracleAttestation Cids OR human-signature Cid
    attested_at_logical_t: LogicalT
    signature           : AttesterSignature  // oracle key OR human key
}

Storage: ObjectType::ResolutionAttestation in CAS;
         L4 ChallengeResolveTx (TB-13 shipped) cites this Cid.
```

### §4.4 Resolution non-availability

Per CR-17.14 fail-safe: if resolution authority cannot be produced within the configured wallclock budget, BLOCK settlement; raise dashboard alert; do NOT auto-resolve.

Atom 5 SAFETY_BOUNDARY § (timeout + default-safe-action) defines the operator-visible behavior.

---

## §5 Carry-forward integration with TB-13/14/15

### §5.1 TB-13 ChallengeTx (existing)

ChallengeTx is the on-chain primitive for opening a challenge. TB-17 atom 4 adds production-readiness requirements ON TOP of the existing TB-13 surface; no schema change needed for atom 4.

### §5.2 TB-14 ChallengeResolveTx (existing)

ChallengeResolveTx admits the resolution attestation per §4.3. TB-17 atom 4 ratifies the ResolutionAttestation schema as the canonical evidence carrier; existing TB-14 surface accepts it.

### §5.3 TB-15 AgentAutopsyCapsule (existing)

When ChallengeResolveTx fires with `ChallengerWins`, the LOSING defender's stake / bond is forfeit. Per TB-15 + TB-17 atom 4, this triggers:
- TB-15-shipped AgentAutopsyCapsule with `LossReasonClass::ChallengeUnsuccessful` (for failed challenger) or `LossReasonClass::SlashLoss` (for slashed defender).
- AutopsyCapsule emission stays AuditOnly (CR-17.11 carry-forward; no global broadcast).

For real-world domains (T2+ production), the public_summary string MUST elide private domain-specific evidence (per CR-17.11 + atom 5 SAFETY_BOUNDARY.md §X privacy).

---

## §6 Failed-challenge → autopsy contract (extends TB-15 LossReasonClass)

```
LossReasonClass extension for real-world TB-19+:
  ChallengeUnsuccessful  (T1+T2 — challenger lost; bond forfeit)
  ChallengeOracleDisagree (T3 — quorum produced but unfavorable)
  ChallengeHumanReversed (T3+T4 — human escalation overruled challenger)
```

These extensions are spec-only in TB-17; implementation in TB-19 charter when the LossReasonClass enum is bumped (additive; non-breaking).

---

## §7 Per-pilot challenge configuration (D1 Lean pilot)

Per DOMAIN_SELECTION_CRITERIA.md §6.2 pilot spec:

| Aspect | D1 setting |
|---|---|
| Tier | T2 |
| Min window | 8 logical steps (sandbox); 24h wallclock if production-relabeled |
| Evidence type | counter-proof object Cid OR counter-Mathlib-version-proof |
| Resolution authority | deterministic — `lean --run` with pinned Lean + Mathlib |
| Disagreement protocol | N/A (single oracle) |
| Settlement timing | 1 logical step post-window in sandbox; ≥ 24h wallclock in production |
| Partial release | NOT permitted in pilot |
| Failed-challenge autopsy | TB-15-existing surface; LossReasonClass::ChallengeUnsuccessful |

---

## §8 Challenge bond protocol (FR-17.6 sub-spec)

To prevent spurious challenges:

- Each ChallengeTx MUST include a non-zero bond (existing TB-13 field).
- Bond size ∈ {tier-default, TaskSpec-override}.
- Failed challenge → bond forfeit (TB-13 shipped slash semantics carried forward).
- Successful challenge → bond returned + reward portion shifted.

Production T2 pilot D1: bond = max(tier-default, max-recommended).

---

## §9 Cross-references

- ORACLE_REQUIREMENTS.md atom 3 §2 (per-tier oracle architecture); §4 (disagreement).
- DOMAIN_SELECTION_CRITERIA.md §3 (tier × oracle matrix); §6 (D1 pilot spec).
- SAFETY_BOUNDARY.md atom 5 (human escalation; timeout; RootBox).
- IRREVERSIBLE_ACTION_POLICY.md atom 6 (settlement irreversibility constraints).
- 2026-05-05 architect verdict §B.5 (FR-17.6, FR-17.7) + §B.6 (CR-17.4, CR-17.7) + §B.7 (SG-17.5).
- TB-13 ChallengeTx (`handover/tracer_bullets/TB-13_charter_2026-05-03.md`).
- TB-14 ChallengeResolveTx (`handover/tracer_bullets/TB-14_charter_2026-05-03.md`).
- TB-15 AgentAutopsyCapsule (`handover/tracer_bullets/TB-15_charter_2026-05-03.md`).
