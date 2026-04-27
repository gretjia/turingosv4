# OBS — Predicate Visibility Trinity: Public / Private / Commit-Reveal (Whitepaper v2 § 5.1 Layer 1 + § 9.4)

**Date**: 2026-04-27
**Type**: alignment observation
**Source**: v2 whitepaper § 5.1 Layer 1 + § 9.4

---

## Observation

v2 formalizes a three-way visibility partition for the Predicate Registry:

| Class | Agent visibility | Use case |
|---|---|---|
| **Public** | full body visible | schema, permission, basic tests |
| **Private** | hidden from agents; visible to humans + auditors | hidden benchmark, anti-Goodhart evaluator |
| **Commit-Reveal** | hash committed up-front; partial test sample revealed at evaluation time | benchmark-leak-resistant scoring |

Core principle:

```
对人类和审计者白盒
对 Agent 分级可见
对状态转移确定执行
```

---

## Current state of implementation

`src/top_white/predicates/visibility.rs` already defines:

```rust
enum Visibility {
    Public,
    Private,
    CommitReveal,
}
```

The enum was added in **Wave 2 / CO1.5** (PredicateRegistry). 14 conformance tests pass.

### What works today

- `Public` and `Private` paths fully wired into `PredicateRegistry::register` and `lookup`.
- Goodhart shield is implemented at the visibility level (Private predicates are hidden from agent-facing read views).

### What v2 highlights as not-yet-implemented

- **Commit-Reveal flow** — the enum variant exists, but the actual commit (hash) → reveal (partial sample) protocol has no runtime support yet. v2 § 9.4 names this explicitly as a Goodhart defense. It is not currently load-bearing for any production transition.
- **Rotating adversarial sets** — v2 § 9.4 mentions, current registry has no rotation cadence.

---

## v2 framing — slight emphasis shift

Pre-v2 framing (from constitution Art. III.4): visibility is a Goodhart defense.

v2 framing (§ 5.1 Layer 1 + § 9.4): visibility is a **structural property of the Predicate Registry**, equally important as `code_hash`, `version`, or `input_schema`. It is part of the registry's data model, not a downstream policy.

**Practical implication**: predicate registry refactors (CO1.5+ extensions) should treat visibility as a first-class field alongside identity / version / schema.

---

## Open questions for future case authoring

1. **Commit-reveal protocol shape** — Merkle commitment of test sample IDs? Random reveal subset? Time-locked reveal?
2. **Rotation policy** — manual? deterministic from epoch? triggered by Goodhart-symptom detection?
3. **Audit-trail visibility** — when a Private predicate fails an agent's tx, what gets returned to the agent? Just `result=0`? Abstract category? Predicate ID?
4. **Cross-agent leak** — if an agent learns Private predicate ID via failure pattern, is that leak? Does ChainTape Layer 6 (Signal Indices) need a per-agent disclosure budget?

---

## Status

**Captured. Partial implementation exists (CO1.5).** No new case at this time. Promote to case only if:
- Commit-Reveal flow gets implemented (would warrant case to record design decision), OR
- A Goodhart symptom is observed and a rotation protocol is needed.

— ArchitectAI, 2026-04-27
