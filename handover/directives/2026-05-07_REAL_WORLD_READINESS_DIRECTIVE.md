# Real-World Readiness Directive — Stage D (2026-05-07)

**Status**: DRAFT — pre-conditions and forbidden list documented. Real-world activation
requires architect-side path decisions on oracle / challenge-court / safety boundary;
this directive lays out the gate structure but does NOT itself activate any real-world
domain.

**Authority**: `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md`
§3.4 (Stage D directive draft authorized; activation requires forward architect-side
path decisions; explicit per CLAUDE.md §10 + edge-case rule §5).

**Companion architect alignment docs**:
- `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_zh.md` §3 Stage D
- `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md` §4 Stage D + §9 Real-world

**Mode**: Constitutional Harness Engineering.

**Phase**: P5 — Real-world readiness scaffolding (post-Lean-Proof-Task-Market /
Polymarket sandbox / TB-18B M1/M2).

---

## §1. Purpose

Real-world tasks (i.e., TuringOS agent activity that affects state outside the local
chain — sending Slack messages, interacting with external APIs, managing real funds,
making decisions with irreversible external consequences) are FROZEN until this
directive's pre-conditions are met. This is per CLAUDE.md §20 freeze conditions and the
architect alignment doc §3 Stage D / §4 Stage D / §9.

This directive enumerates:
- Pre-conditions (mandatory documents and evidence)
- Forbidden activations (what MUST NOT happen until pre-conditions close)
- Per-domain assessment template
- Required runtime gates

It does NOT activate any specific real-world domain. Domain activation requires a
forward TB charter (e.g., TB-19 / TB-20+) under separate architect §8 sign-off.

## §2. Pre-conditions (MANDATORY before any real-world TB charter)

The following six documents MUST exist, be reviewed by architect, and be linked from
this directive's §6 sign-off section before ANY real-world activation TB is charterable:

### §2.1. REAL_WORLD_READINESS_REPORT

| Field | Required content |
|-------|------------------|
| Substrate readiness | TB-C0 SHIPPED FINAL ✅; TB-18R FINAL SHIPPED ✅; TB-18B M1/M2 SHIPPED (forward); TB-18D Polymarket P-M9 controlled smoke green (forward); HEAD_t C2 substrate (TB-18C, forward, RECOMMENDED). |
| Audit readiness | Codex + Gemini dual audit on all of the above closed; conservative ranking PASS-aggregate. |
| Constitution coverage | Constitution gates ≥97 GREEN (or higher); matrix RED rows = 0; AMBER rows enumerated and forward-bound; Gemini Q8 forward bindings (Art. 0.2 + PCP phase-2) closed. |
| Halt-reason taxonomy | All halt classes runtime-witnessed and replay-deterministic. |
| Storage form | HEAD_t multi-ref ChainTape (C2) preferred for production multi-domain runs. |

### §2.2. DOMAIN_SELECTION_CRITERIA

For each candidate real-world domain (e.g., "Slack message dispatch", "GitHub PR review",
"Polymarket arbitrage", "discrete arithmetic API queries"), the criteria document MUST
specify:

| Criterion | Question |
|-----------|----------|
| Predicate | Is there a bounded, automatable Boolean predicate that decides "task done correctly"? If no, domain is FORBIDDEN until predicate exists. |
| Oracle | Is there a deterministic oracle that resolves the predicate (Lean-style for math; controllable simulator for non-math; none → human-court)? |
| Reversibility | Can ALL agent actions in this domain be reverted within the challenge window? If not, irreversibility quantified. |
| Blast radius | Maximum economic / reputational / safety damage of a worst-case agent action. |
| Stakeholder exposure | Who outside the project sees / is affected? |
| Recovery path | If agent acts wrongly, recovery procedure. |

A domain that fails ANY of the above is FORBIDDEN.

### §2.3. ORACLE_REQUIREMENTS

| Requirement | Description |
|-------------|-------------|
| Determinism | Same input → same verdict (replay-deterministic). |
| Independence | Oracle MUST NOT be the agent under audit nor controlled by it. |
| Boundedness | Oracle response time bounded; no DoS vector via timeouts. |
| Auditability | Oracle decision artifacts (queries / responses / verdict) MUST be CAS-resident and L4-anchored. |
| Failure mode | Oracle unavailability → predicate = `Pending` (NOT `Pass` or `Fail`); routes to L4.E with `OracleUnavailable` rejection class or anchored EvidenceCapsule per Art. III shielding. |

### §2.4. CHALLENGE_COURT_REQUIREMENTS

| Requirement | Description |
|-------------|-------------|
| Window | Settled outcome held in escrow for a window N (block / time / configurable). |
| Stake | Challenger posts stake; if challenge succeeds, restitution + slashing of original outcome's beneficiary; if challenge fails, stake forfeited. |
| Adjudication | Court process documented; can be tier-1 system-only (oracle re-run with new inputs), tier-2 multi-oracle quorum, tier-3 human escalation. |
| Asymmetry | Challenge cost MUST be lower than challenge benefit when challenger is correct, AND MUST be higher than challenge benefit when challenger is incorrect. (`feedback_lossless_constitution_polymarket_directive` verification asymmetry framework.) |
| No double-jeopardy | Once challenge window closes and outcome finalizes, NOT re-openable except via Class-4 architect override. |

### §2.5. SAFETY_BOUNDARY

| Item | Specification |
|------|---------------|
| Per-domain budget | Maximum Coin / time / API-call budget per agent per domain. Hard cap at sequencer level. |
| Per-domain rate limit | Max actions per unit time. Hard cap at sequencer level. |
| Per-domain exposure cap | Maximum aggregate market exposure across YES + NO + LP positions. |
| Cross-domain firewall | Action in domain A MUST NOT escalate privileges in domain B without cross-domain capability tx (Class-4). |
| Emergency halt | Architect-only `EmergencyHaltTx` (system-only) that pauses all agent activity in a domain; does NOT resolve in-flight tasks; resumes only via architect §8. |

### §2.6. IRREVERSIBLE_ACTION_POLICY

| Item | Rule |
|------|------|
| Default | Irreversible external action is FORBIDDEN. |
| Exception | Architect-explicitly-ratified Class-4 capability per domain, with: (a) named irreversible action set; (b) human-in-the-loop confirmation required per action; (c) per-action escrow and challenge window before action submitted; (d) post-action audit trail. |
| Examples FORBIDDEN by default | Sending real Slack message; transferring real funds; creating real GitHub PR; deleting external resources; making API calls that incur monetary cost; making API calls that affect external system state visible to users. |
| Examples CONDITIONALLY ALLOWED | Read-only API queries with rate limits; sandbox / staging environment writes (clearly labeled non-production); fully-reversible API mutations (created resource can be deleted within window). |

## §3. Forbidden activations (until pre-conditions close)

Per architect alignment doc §3 / §4 / §9 + CLAUDE.md §20:

```
- no real-world domain without oracle
- no subjective task without predicate plan
- no irreversible external action
- no settlement before challenge window
- no price-as-truth in real-world resolution
- human escalation required for high-risk domains
- no real money before readiness gate
- no public chain before sandbox graduation
```

## §4. Per-domain runtime gates

When a real-world domain TB charter is filed, it MUST satisfy:

| Gate | Verification |
|------|--------------|
| Oracle CAS-resident | Every oracle query produces a CAS-resident `OracleQuery` + `OracleVerdict` object pair, both L4-anchored or capsule-anchored. |
| Predicate boundedness | `Predicate::evaluate(query, response) -> Verdict` returns within bounded time AND with deterministic re-evaluation on replay. |
| Challenge window honored | Outcome held in escrow ≥ window length; settle action gated on absence of pending challenge. |
| Slashing path live | Challenge-success path has a tested code path that slashes original beneficiary AND pays challenger restitution. |
| Per-domain budget enforced | Sequencer-side budget assertion before each action submission. |
| Per-domain rate limit enforced | Sequencer-side rate limit assertion. |
| Cross-domain firewall enforced | No implicit privilege escalation cross-domain. |
| Emergency halt tested | `EmergencyHaltTx` (architect-signed) tested in a smoke run; pause + resume cycle works end-to-end. |
| Replay deterministic | Real-world domain run replays identically (modulo external timestamps) given the same oracle responses. |

## §5. Forward TB sequencing (out of this directive's scope, recorded for clarity)

A real-world domain charter (e.g., TB-19) follows this skeleton:

```
TB-19 charter
  §1 Scope: which domain (e.g., "discrete arithmetic API queries on a sandbox endpoint")
  §2 Pre-conditions: this directive's §2.1..§2.6 docs all linked
  §3 FR: per §4 runtime gates above + domain-specific
  §4 CR: this directive's §3 forbidden list + per-domain extensions
  §5 SG: per-domain ship gates including a controlled smoke run
  §6 Forbidden list: this directive's §3 verbatim + domain-specific
  §7 Atoms: predicate + oracle + challenge court + safety boundary + smoke
  §8 §8 ship: per-Class-4-atom architect §8 + overall §8
```

The first real-world TB SHOULD be a low-risk read-only or sandbox-write domain to
exercise the runtime gates before any high-risk activation.

## §6. Activation sign-off (FORWARD; not signed in this directive)

Real-world activation requires a separate `handover/directives/YYYY-MM-DD_REAL_WORLD_DOMAIN_<slug>_§8_SIGN_OFF.md`
file per domain, citing:

1. This directive (pre-condition checklist).
2. The per-domain TB charter and its §8 sign-off.
3. The six §2 documents (REAL_WORLD_READINESS_REPORT etc.) all linked.
4. The Codex + Gemini dual-audit verdicts on the per-domain charter.
5. Explicit named architect approval that does NOT use single-word semantics.

## §7. Cross-references

- Architect alignment Stage D: `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_{zh,en}.md` §3 / §4 Stage D + §9 Real-world
- Parent authorization: `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md`
- TB-C0 §8 sign-off (FREEZE-lift authority): `handover/directives/2026-05-07_TBC0_ARCHITECT_§8_SIGN_OFF.md`
- TB-18R FINAL §8 sign-off: `handover/directives/2026-05-07_TB18R_FINAL_§8_SIGN_OFF.md`
- TB-18C HEAD_t C2 charter (substrate readiness): `handover/tracer_bullets/TB-18C_charter_2026-05-07.md`
- TB-18B M1/M2 charter (benchmark readiness): `handover/tracer_bullets/TB-18B_charter_2026-05-07.md`
- TB-18D Polymarket charter (sandbox-market readiness; Stage C feeds Stage D): `handover/tracer_bullets/TB-18D_charter_2026-05-07.md`
- Verification asymmetry framework: `handover/directives/2026-05-02_lossless_constitution_polymarket_directive__part_A_appendix_D_verification_asymmetry.md`
- 9-phase roadmap: `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md`
- Project decision map: `handover/architect-insights/PROJECT_DECISION_MAP_2026-04-27.md`
- CLAUDE.md §20 freeze conditions
- `feedback_launch_priority` (Lean Proof Task Market MVP precedes real-world)
- `feedback_real_problems_not_designed` (real public problems preferred over synthesis)
- `feedback_no_workarounds_strict_constitution` (no workaround / strict alignment)

---

**This directive is DRAFT-LANDED 2026-05-07.** AI coder MUST NOT initiate any real-world
domain charter before §2 pre-conditions are filed AND a per-domain TB charter receives
its own architect §8 sign-off.
