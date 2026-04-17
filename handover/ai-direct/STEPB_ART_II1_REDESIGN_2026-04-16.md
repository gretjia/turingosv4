# Step-B Phase 0 — Art. II.1 Fix REDESIGN (post Codex VETO/CHALLENGE)

## Why redesign

Codex identified 2 issues with v1 proposal:
- **Q3 CHALLENGE**: raw global fan-out violates Art. II.1 ("典型错误 **抽象** 出来") + C-022 (context poisoning risk). My v1 spec was "broadcast reason strings" — too raw.
- **Q5 VETO**: rejected protocol-forced append (correctly — C-034). But v1 didn't propose that.

Remaining conflict: Q2 "tx_count confound" — Codex: 24 vs 1.3 could be model differences, not mechanism. Valid partial concern; address via pre-reg A/B design.

## Redesigned fix (v2)

### Component 1: Ingestion (both auditors CONCUR on Q4)

**evaluator.rs** (non-restricted file, self-approvable):
```rust
// In OMEGA-reject branch (around line 283):
let err_class = classify_lean_error(&combined);  // NEW helper
bus.add_graveyard_entry(agent_id, &err_class);

// In parse-fail branch:
bus.add_graveyard_entry(agent_id, &format!("parse_fail:{}", err_class_of(&e)));
```

### Component 2: Error classification (new file, non-restricted)

**src/sdk/error_abstraction.rs** (new, not in restricted list):
```rust
// Map raw Lean / parse errors to normalized classes.
// C-022 shield: agents see class labels, not raw logs.
pub enum OracleErrClass {
    UnknownConstant,         // "unknown constant `X.y.z`"
    TacticFailed(String),    // "linarith failed", "simp made no progress" — tactic name only
    RewriteNoMatch,          // "did not find an occurrence"
    TypeMismatch,
    UnsolvedGoals,
    UnexpectedToken,
    HeartbeatExceeded,
    Other,
}

pub fn classify_lean_error(combined: &str) -> OracleErrClass { ... }
pub fn short_label(c: &OracleErrClass) -> String { ... }  // stable, terse: "err:linarith", "err:unknown_const"
```

### Component 3: bus.rs (restricted — Step-B worktree)

```rust
// src/bus.rs
pub enum RejectionScope { PerAuthor, Global, TopKClasses(usize) }

pub fn recent_rejections_scoped(&self, author: &str, max: usize, scope: RejectionScope) -> Vec<String> {
    match scope {
        RejectionScope::PerAuthor => self.graveyard.get(author).map(...)
        RejectionScope::Global => ... flatten all authors, most recent max
        RejectionScope::TopKClasses(k) =>
            // Count class occurrences across all authors; return top-k (label, count).
            // This is the Art. II.1 "abstract" mandate: broadcast common patterns, not raw instances.
    }
}

// Backward-compat wrapper (existing call sites untouched):
pub fn recent_rejections(&self, author: &str, max: usize) -> Vec<String> {
    self.recent_rejections_scoped(author, max, RejectionScope::TopKClasses(3))
}
```

### Component 4: evaluator.rs swarm loop uses new API

```rust
// Change recent_rejections → recent_rejections_scoped(agent_id, 3, RejectionScope::TopKClasses(3))
// Agents now see: "Top failure classes: linarith(8), unknown_const(5), rewrite_no_match(3)"
// — abstracted, bounded, C-022-safe.
```

## Constitutional reconciliation

| Concern | How addressed |
|---|---|
| Art. II.1 "abstract and broadcast" | `TopKClasses` explicitly abstracts raw errors into classes; broadcast is class-scoped |
| C-022 context poisoning | Agents receive class labels + counts, never raw Lean output. Classifier output space is bounded (7 classes) |
| C-034 mechanism > prompt | Change is in mechanism (bus API + classifier), not prompt tuning |
| Art. III.1 shield errors | Classifier IS a shield — raw → abstracted |
| C-017 silent failure | Unknown → `OracleErrClass::Other` (not dropped); logged |

## Pre-registered A/B design (addresses Q2 confound)

**Control**: main (HEAD = e58e021 v3.1 M4 + current v3.2 code)
**Treatment**: experiment/art-ii1-topk-classes

**Sample**: frozen seed=74677 N=50
**Model**: deepseek-chat (same as v3.2)
**Conditions**: all 3 (oneshot/n1/n3) on both branches
**Interleaving**: same 3-rotation
**Primary**: SolveRate pairwise per condition
**Decision rule** (frozen in metrics.yaml):
  - Merge if `ΔSolveRate(n3, treatment − control) ≥ 3` AND n1/oneshot non-regress (≥ −1)
  - Abandon if n3 regresses ≥ 2
  - Gray otherwise

**Why this isolates scaffold vs model confound (Codex Q2)**:
- Control = chat + current (broken) bus.rs
- Treatment = chat + fixed bus.rs
- **Model, prompt, timeout all identical**. Only the mechanism fix differs.
- If n3 improves strict win → mechanism was the bottleneck (H3 confirmed)
- If n3 flat → mechanism not critical; other bottleneck

## Phase 1 plan (worktree)

```bash
git worktree add .claude/worktrees/stepb-art-ii1 -b experiment/art-ii1-topk-classes
cd .claude/worktrees/stepb-art-ii1
# Edit src/bus.rs, src/sdk/error_abstraction.rs (new), experiments/minif2f_v4/src/bin/evaluator.rs
cargo test
# Commit to experiment branch; do NOT merge
```

## Q2 confound addendum

Codex Q2 pointed out tx_count gap could be model differences. True. The A/B design controls for this because both arms use `deepseek-chat`. If tx_count drops dramatically on treatment arm → confirms mechanism was the issue. If tx_count stays high → mechanism wasn't the bottleneck.

## Re-audit request

Both auditors please re-verify:
- Q3 (abstraction): does TopKClasses satisfy Art. II.1 "abstract" + C-022 shield?
- Q4: classifier covers OMEGA + parse + can handle unknown err (→ Other class)
- New: is the classifier itself a measurement tool (C-012 freeze required)?

Final verdict: PROCEED to Phase 1 / HOLD / VETO.
