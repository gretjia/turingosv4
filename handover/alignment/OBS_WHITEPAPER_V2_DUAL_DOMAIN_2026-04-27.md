# OBS — 创造域 vs 安全域 (Whitepaper v2 § 7.2)

**Date**: 2026-04-27
**Type**: alignment observation (per C-069 hygiene; not a case, not a rule, not a constitutional edit)
**Source**: v2 whitepaper § 7.2

---

## Observation

v2 introduces an explicit **dual rejection mode** for boolean predicates:

| 域 | Default posture | Rationale |
|---|---|---|
| **创造域 (creative)** | 疑罪从无 (presumption of innocence) | Don't kill correct candidates; let large candidate pools enter statistical filtering |
| **安全域 (security)** | fail closed | Uncertainty → reject; protect irreversible state |

### Examples of each domain

**创造域**: code generation, UI proposal, text proposal, research hypothesis, open-ended search

**安全域**: constitution edit, sudo, privilege escalation, external transfer, irreversible state change, key access, production deploy

### Why this is novel

Constitution Art. I.1 currently specifies a single boolean-predicate semantics — pass = 1, fail = 0, no domain-conditioned softening. v2 § 7.2 makes the rejection threshold **risk-tier-conditional**: the same predicate logic, but the false-positive vs false-negative tradeoff changes by domain.

This is not in tension with Art. I.1 (a predicate is still 0/1) — it's a **policy layer above** Art. I.1 that selects which predicate set to apply at which threshold for which transition class.

---

## Implementation surfaces (forward-looking)

If this doctrine ratifies into a case (potentially C-077), the following surfaces would be touched:

- `src/top_white/predicates/registry.rs` — add `risk_tier: enum {Creative, Security}` field on registered predicates
- `src/bus.rs::forbidden_patterns` — already enforces 安全域 fail-closed for forbidden writes (e.g., constitution path)
- `src/wallet.rs` (or future `src/bottom_white/wallet/`) — sudo / key / external transfer paths must invoke the security-domain predicate set

Current code is **not in violation** — `forbidden_patterns` is already 安全域-style fail-closed; all current predicates are conservatively coded. v2 § 7.2 codifies the doctrine, not corrects an existing bug.

---

## Open questions for future case authoring

1. Is the domain partition discrete (Creative ⊕ Security) or continuous (a risk score)?
2. Who classifies a predicate? (Predicate author? Tool registry? Constitutional fiat?)
3. How are mixed-domain transitions handled? (e.g., a creative code edit that touches a sudo path)
4. Is there a third domain — "neutral" / "operational" — or are all transitions in one of the two?

These are not blocking. The observation is captured here so a future Wave can reference it when addressing predicate-registry policy attributes.

---

## Status

**Captured. Not codified.** No case (C-NNN), rule (R-NNN), or constitutional article amended at this time. Ratify into a case only when:
- Codex and/or Gemini audit cite this distinction in a verdict, OR
- A real bug in production / experimentation is traceable to lack of this distinction, OR
- A predicate-registry refactor would benefit from a `risk_tier` attribute

— ArchitectAI, 2026-04-27
