# No-PROVEN Claim Integrity Checklist

Status: active guidance
Class: 0
Parent plan: `handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`

Use this checklist before any PR, report, dashboard, README, or benchmark
headline uses strong claim language.

## Banned Without Evidence Packet

Do not use these terms unless G1-G6 below are all satisfied and cited:

```text
PROVEN
DEFINITIVE
causal
world-first
TASK-PASS
market beats
market failed
price is worse
```

## G1-G6 Evidence Checklist

```text
G1. Claim names exact regime, workload, agent set, budget, and verifier.
G2. Claim is recomputed from ChainTape/GitTape/CAS, not a manifest-only value.
G3. Claim has a positive control that would fail for a lying headline.
G4. Claim distinguishes TASK-PASS from FLOW-PASS, SYSTEM-PASS, smoke, or canary.
G5. Claim states unsupported adjacent claims.
G6. Class 2+ claim has clean-context audit after local evidence exists.
```

## Required Claim Boundary Block

```text
Supported:
  <exact claim and evidence path>

Unsupported:
  <adjacent claims this artifact does not prove>
```

## Market Claim Template

```text
In the tested regime <regime>, <mechanism> produced <metric> relative to
<controls>. This supports <narrow claim>. It does not test <untested market
functions>. It does not prove broad agent market economy success or failure.
```

## Acceptance Commands

```bash
claim_hits=$(grep -RInE 'PROVEN|DEFINITIVE|causal|world-first|TASK-PASS|market beats|market failed' \
  handover src tests AGENTS.md CLAUDE.md .github \
  | grep -vE 'no-proven-checklist|CLAIM_INTEGRITY|pull_request_template|AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT|P1_REALVALUE_SCOPE_CORRECTION|A14_WORKLOAD_ADAPTER_BOUNDARY_PREFLIGHT' || true)
test -z "$claim_hits" || { printf '%s\n' "$claim_hits"; exit 1; }
grep -RInE '(^|[^A-Za-z])(mod|use)[[:space:]]+market_tape_shared|[m]arket_tape_shared::' \
  AGENTS.md CLAUDE.md skills .github handover/directives && exit 1 || true
```
