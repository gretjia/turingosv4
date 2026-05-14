# OBS: Forward-Bound CAS Strict-Merkle C2 Enhancement

Date: 2026-05-14

Status: Forward-bound observation; not a current G-Phase blocker.

## Observation

The architect's G-Phase closeout update explicitly reclassifies the current
project state:

```text
当前主要剩余项集中在 G-Phase aggregate 和若干 forward rows。
```

For the constitution foundation, the update also calls out:

```text
C2 refs/chaintape/cas strict-Merkle 仍 forward-bound。
```

This matches the existing `CONSTITUTION_EXECUTION_MATRIX.md` Art. 0.4 row:

```text
refs/chaintape/cas strict-Merkle commit-chain redesign
is forward-bound to Stage A3.6 enhancement TB
```

## Current Boundary

- `refs/chaintape/{l4,l4e,cas}` production ref semantics are already landed and
  verified under real-LLM load.
- C1 baseline remains a backward-compatible alias.
- `refs/chaintape/cas` strict-Merkle commit-chain redesign remains a Stage A3.6
  enhancement TB, not a blocker for G4.2, G5/G6/G7 structural smoke, or SG-G
  closeout.

## Guardrail

Do not silently retire this observation during SG-G closeout. The correct action
is to carry it forward as a known enhancement while keeping the current G-Phase
focus on:

```text
G4.2 model identity replay
G5 opportunity scheduler
G6 price observe-only
G7 structural run6-equivalent smoke
SG-G overall packet
```
