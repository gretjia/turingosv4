---
name: harness-reflect
description: Harness self-reflection — evaluate rule effectiveness, health check, trace analysis
user_invocable: true
---

# /harness-reflect — Self-Reflection Loop

## Stages

### 1. Inventory
```
=== HARNESS INVENTORY ===
Incidents: N (incidents/)
Rules: M (rules/active/)
Enforced: X / N
Recent traces: traces/sessions/ (last 7 days)
```

### 2. Gap Analysis
- Incidents without corresponding rules?
- Incidents without corresponding cases (判例)?
- Rules that never trigger (stale)?
- Traces showing recurring patterns not captured by rules?
- Constitutional clauses without any case precedent?

### 3. Rule Effectiveness
- Most triggered rules (from enforcement.log)
- False positive rate estimate
- Rules that block vs. only warn

### 4. Health Score
```
Score = (enforced_incidents / total_incidents) × coverage_factor
```

### 5. Recommendations (max 5)
Propose: new rules, rule updates, stale rule removal, doc updates.
