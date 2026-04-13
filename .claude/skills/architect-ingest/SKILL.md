---
name: architect-ingest
description: Ingest architect directives — archive fully, detect axiom impact, wait for authorization
user_invocable: true
---

# /architect-ingest — Directive Ingestion

**Core: 接收指令 ≠ 授权执行。Archive and analyze only.**

## Procedure

### 1. Full Archive
Save to `handover/directives/YYYY-MM-DD_<topic>.md`:
- Complete directive content (no omissions)
- Design philosophy and rationale

### 2. Impact Detection
Check if directive affects Layer 1 invariants:
- kernel.rs 零领域知识
- Append-Only DAG
- Economic conservation
If Layer 1 violated → flag VIOLATION, do not execute.

### 3. Verbal Insight Mode
For short philosophical principles (≤ 50 chars):
Save to `handover/architect-insights/YYYY-MM-DD_<slug>.md`

### 4. Await Authorization
Present analysis to user. Execute ONLY after explicit approval.
