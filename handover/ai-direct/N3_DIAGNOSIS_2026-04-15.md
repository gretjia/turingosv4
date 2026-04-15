# n3 Real-Data Diagnosis — 2026-04-15

**Method**: stderr line-by-line + bus.rs code audit + evaluator.rs code audit. NO speculation.

## Confirmed facts

### F1. Abort rotation distribution is coincidence

All 3 n3 timeouts fall on rot=2 problems (170, 208, 293). But:
- `mathd_algebra_170`: n3 TIMEOUT, n1 SOLVED 399s, oneshot FAIL
- `mathd_algebra_208`: n3 TIMEOUT, n1 TIMEOUT, oneshot FAIL
- `mathd_algebra_293`: n3 TIMEOUT, n1 TIMEOUT, oneshot ?

n1 fails on 2/3 → these are genuinely hard problems. Rotation-2 sample coincidence (only 3 rot=2 problems in first 10; all happen to be hard).

### F2. Agents converge on identical hallucinated Mathlib API

`mathd_algebra_170` tx 0, 2, 4 (different Agents): all cite non-existent `Int.abs_le.mp` + `Int.abs_le.mpr`. 3 agents, 3 independent hallucinations of the same wrong API.

`mathd_algebra_208` tx 0, 1, 2, 3: all fail at `Real.rpow_mul` rewrite with same error `Tactic rewrite failed: Did not find an occurrence of the pattern`.

### F3. recent_errors broadcast mechanism is structurally broken

**bus.rs:247**:
```rust
pub fn recent_rejections(&self, author: &str, max: usize) -> Vec<String> {
    self.graveyard.get(author)   // per-author, not global
        .map(|v| v.iter().rev().take(max).cloned().collect())
        .unwrap_or_default()
}
```
Each agent receives ONLY its own history — not other agents' errors. Art. II.1's "broadcast" is effectively "private memory".

**evaluator.rs OMEGA rejection**:
```rust
Ok(false) => {
    warn!("[tx {}] OMEGA rejected. payload[0..300]={:?}", tx, preview);
}
```
OMEGA rejections do not enter bus or graveyard. Parse failures also do not. Only bus.append vetoes populate the graveyard. Since agents mostly use `complete` (never `append`), the graveyard stays empty.

**Combined effect**: `recent_errors = []` almost always in n3 mode. Agents have zero shared learning signal.

### F4. WAL is empty

`/home/zephryj/projects/turingosv4/experiments/minif2f_v4/wal/` contains 0 files. No write-ahead log is being emitted. We assumed we had WAL-based diagnostics; we don't.

## Constitutional recursive audit

| Article / Case | Status | Evidence |
|---|---|---|
| Art. II.1 broadcast typical errors | **VIOLATED mechanically** | F3 — per-author + OMEGA path skipped |
| Art. II.2 price signals | Partially alive | market_ticker transmitted but no nodes → no real markets |
| Art. V.1.3 JudgeAI independent | OK | Lean oracle separate from proposer |
| C-022 (context poisoning) inverse | **Active** | Errors never enter context → no correction loop |
| C-033 emergence requires causal proof | **Current n3 claim fails this** | Claimed swarm, but coordination channel is broken |
| C-034 mechanism > prompt | **Our own setup violates** | recent_errors is prompt-level affordance with mechanism gap behind it |

## Real interpretation of n3's abort

**Not**: "3 agents interfere with each other" (earlier speculation, now retracted).

**Actual**: n3 = 3 independent oneshot attempts, where the architectural "cooperation channel" (recent_errors broadcast) is mechanically severed. When problems are hard, all 3 independently hallucinate similar wrong proofs. 3 timeouts stack → abort gate triggered.

This is consistent with user's thesis (chat_over_reasoner): reasoner internal CoT already produces complete proofs → no pressure to use `append` → graveyard stays empty → no broadcast. The external scaffold is present but dormant.

## Fix direction (NOT implemented — requires src/bus.rs human confirm per CLAUDE.md)

**F3 addresses** (candidate case material for post-experiment review):

1. **Global graveyard query**: `recent_rejections` should take optional `scope = per_author | global` — default global for Art. II.1 compliance.
2. **OMEGA rejection enters graveyard**: extend evaluator.rs so Ok(false) from oracle calls `bus.add_graveyard_entry("OMEGA", reason)`. Parse fails likewise.
3. **Top-level abstraction** (Art. II.1 full): when 2+ agents receive same error (string match), promote to "typical error" broadcast with higher prominence in prompt.

**Priority vs v3.2 chat-model test**: the chat-model test should run FIRST because:
- If chat + broken-scaffold performs similarly to reasoner + broken-scaffold, we learn nothing about scaffold value
- If chat + broken-scaffold fails dramatically AND chat + fixed-scaffold recovers, we've proven scaffold matters
- Architecture fix alone (without model change) won't distinguish "scaffold value" from "reasoner carrying"

Sequence: v3.1 (current, reasoner) → v3.2 (chat + broken scaffold) → v3.3 (chat + fixed Art. II.1)

## Evidence index

- stderr `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/logs/v31_20260415T013559.err` lines for mathd_algebra_170/208/293 under `condition=n3`
- `src/bus.rs:247-251` (recent_rejections per-author scope)
- `experiments/minif2f_v4/src/bin/evaluator.rs` OMEGA rejection branch (line ~277-283 post-diagnostic edit)
- `experiments/minif2f_v4/wal/` — empty directory
