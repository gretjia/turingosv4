# REAL-5S Scaffold Ratification Report

Date: 2026-05-15 UTC

Directive:

```text
REAL-5S  Scaffold ratification / clean-negative closure
```

Ratified claim:

```text
REAL-5 proves role scaffolding.
REAL-5 does not prove market emergence.
```

Required REAL-5S statements:

```text
role gateway blocks Trader proof-style leakage
Verifier behavior observed
Trader buy=0
NoPool dominates
No E2/E3 claim
```

## Evidence Sources

Primary post-VETO evidence:

```text
handover/evidence/g_phase_real_5_trader_first_b8_rolegate_20260514T192523Z
handover/evidence/g_phase_real_5_core3_b8_rolegate_20260514T192958Z
```

Final implementation audit:

```text
handover/audits/CODEX_REAL5_IMPLEMENTATION_REVIEW_R3.md
```

Closed Harness evidence:

```text
handover/evidence/dev_self_hosting/dev_1778788069384_807750
```

Completion and adversarial report:

```text
handover/evidence/real5_overnight_20260514/REAL5_COMPLETION_AND_ADVERSARIAL_TEST_REPORT.md
handover/evidence/real5_overnight_20260514/REAL5_OVERNIGHT_EXPERIMENT_LEDGER.md
```

## Scaffold Findings

The role gateway blocks Trader proof-style leakage.

The R2 audit VETO found that a trader-first run still allowed `Agent_0=Trader`
to reach legacy proof/verify production paths. The post-VETO remediation wired
the REAL-5 typed role gateway before the legacy evaluator action dispatch.

The post-VETO trader-first run:

```text
handover/evidence/g_phase_real_5_trader_first_b8_rolegate_20260514T192523Z
```

witnesses:

```text
accepted VerifyTx by Agent_0 Trader: 0
accepted WorkTx by Agent_0 Trader: 0
Agent_0 Trader role_turn outcomes: PolicyRejected=5
```

The R3 clean-context Codex review reports no blocking production defects and
confirms that Trader proof/verify output is routed to L4.E policy rejection
rather than WorkTx / VerifyTx production branches.

Verifier behavior observed.

REAL-5 bounded runs observed non-zero verification behavior before any
spontaneous market router buy:

```text
R5-CORE3-B20: verify=6
R5-TRADER-FIRST-B12: verify=5
R5-MARKET-K0-B12: verify=3
R5-ADV5-B8: verify=5
R5-TRADER-FIRST-B8-ROLEGATE: verify=1
```

Trader buy=0.

Both post-VETO rolegate runs have:

```text
buy_with_coin_router=0
```

The broader overnight ledger likewise records `0 router buys` across the
successful bounded REAL-5 true-problem runs.

NoPool dominates.

The overnight report records that all role-turn Trader NoTrade reasons observed
overnight were `NoPool`. This is the mechanism-level clean-negative signal:
role scaffolding exists, but post-accept node markets are generally unavailable
or too late for Trader turns to use.

No E2/E3 claim.

This report does not claim:

```text
E2: spontaneous live agent market action
E3: persistent role-differentiated market behavior
```

## Ratification Verdict

REAL-5S ratifies the scaffold-level result only:

```text
role-based generation scaffold,
tape-visible decisions/reasons,
typed gateway,
role-scoped view,
reconstructable ChainTape/CAS evidence.
```

It does not ratify market emergence.

Next phase:

```text
REAL-6 Event Timing & Lawful Pressure
```
