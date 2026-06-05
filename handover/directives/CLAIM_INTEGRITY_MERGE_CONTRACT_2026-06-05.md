# Claim Integrity Merge Contract

Date: 2026-06-05
Status: active Class 0 contract
Parent plan: `handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`

## Decision

Strong claims are not accepted on prose, stdout, dashboards, or manifest values.
They require reconstructable evidence and explicit claim boundaries.

## Scope

This contract applies to:

```text
PR titles
PR bodies
handover reports
benchmark reports
README or dashboard headlines
agent summaries intended for merge
```

## Rule

Any strong claim must satisfy G1-G6 from `skills/no-proven-checklist.md`.

```text
G1 exact regime
G2 recompute from ChainTape/GitTape/CAS
G3 positive control
G4 TASK-PASS separated from smoke/FLOW-PASS/SYSTEM-PASS/canary
G5 unsupported claims stated
G6 clean-context audit after evidence for Class 2+
```

## Mechanism Gates

Docs-only guidance is active now. Mechanism gates are intentionally deferred
until the generic ChainTape/GitTape projection trait lands.

Allowed future gate files:

```text
tests/constitution_headline_recompute_from_chaintape.rs
tests/constitution_router_name_matches_mechanism.rs
scripts/constitution_gates.manifest.toml
```

Forbidden dependency:

```text
market_tape_shared.rs
```

Required positive controls:

```text
lying manifest must fail headline recompute
softmax implementation that collapses to argmax must fail router-name gate
```

## Merge Blockers

```text
Strong claim without G1-G6 = block.
TASK-PASS without verifier-backed TASK-PASS = block.
Market-wide success or failure from P1 price-as-router alone = block.
market_tape_shared.rs dependency in claim gates = block.
Audit before runnable evidence = block for Class 2+.
```

## Acceptance

```bash
git diff --check
claim_hits=$(grep -RInE 'PROVEN|DEFINITIVE|causal|world-first|TASK-PASS' \
  AGENTS.md CLAUDE.md skills/no-proven-checklist.md \
  .github/pull_request_template.md \
  handover/directives/CLAIM_INTEGRITY_MERGE_CONTRACT_2026-06-05.md \
  | grep -vE 'no-proven-checklist|CLAIM_INTEGRITY|pull_request_template' || true)
test -z "$claim_hits" || { printf '%s\n' "$claim_hits"; exit 1; }
grep -RInE '(^|[^A-Za-z])(mod|use)[[:space:]]+market_tape_shared|[m]arket_tape_shared::' \
  AGENTS.md CLAUDE.md skills/no-proven-checklist.md \
  .github/pull_request_template.md \
  handover/directives/CLAIM_INTEGRITY_MERGE_CONTRACT_2026-06-05.md && exit 1 || true
```
