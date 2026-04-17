# Step-B Phase 0 — Necessity Audit for Art. II.1 Broadcast Fix

Generated: 2026-04-16 (autonomous, user absent)

## Problem

Art. II.1 mandates "broadcast typical errors to all agents". Current implementation (`src/bus.rs:247-251`):

```rust
pub fn recent_rejections(&self, author: &str, max: usize) -> Vec<String> {
    self.graveyard.get(author)
        .map(|v| v.iter().rev().take(max).cloned().collect())
        .unwrap_or_default()
}
```

- **Per-author scope** (not global) — violates "broadcast"
- **Populated only by bus.append veto path** — OMEGA rejections + parse failures never enter graveyard
- Net: `recent_errors = []` nearly always in swarm mode

## Empirical evidence (N=76 OMEGA wins across v3.1 reasoner + v3.2 chat)

| Experiment-Condition | Solves | Direct complete (depth=1) | Tape used (depth>1) | Mean tx_count |
|---|---|---|---|---|
| v3.1 oneshot | 23 | 23 | 0 | 1.0 |
| v3.1 n1 | 30 | 30 | 0 | 1.3 |
| v3.1 n3 | 7 | 7 | 0 | 1.1 |
| v3.2 oneshot | 0 | — | — | — |
| v3.2 n1 | 18 | 18 | 0 | **24.2** |
| v3.2 n3 | 18 | 18 | 0 | **12.9** |
| **Total** | **76** | **76 (100%)** | **0 (0%)** | — |

**v3.2 chat with broken scaffold**: mean tx_count 24× oneshot. Agents retry the same failed proof repeatedly. If recent_errors broadcast were alive, 2nd/3rd attempt would see the previous error and try a different tactic. It doesn't → proof of mechanism-level non-function.

## Proposed fix (preview for audit)

1. `recent_rejections` take optional `scope: PerAuthor | Global`; default **Global** for Art. II.1 compliance
2. evaluator.rs OMEGA-reject branch calls `bus.add_graveyard_entry(author, reason_snippet)` when Lean rejects
3. Parse-fail branch likewise enters graveyard with `"parse: <err>"` tag

## Questions for external auditors

Q1. Does the evidence establish F-2026-04-15-02 (Art. II.1 structurally severed) conclusively? Or is there a confound (e.g., reasoner only ever needs 1 tx, so the 0% tape usage is model-choice, not mechanism breakage)?

Q2. The v3.2 chat data (24 mean tx, 0 tape usage) — does this distinguish "mechanism broken" from "agents don't WANT to use tape"? If the latter, bus.rs fix won't help; the real issue is prompt design incentivizing append.

Q3. Is the proposed fix minimal? Does it introduce new C-022 (context poisoning) risk by sharing more state across agents?

Q4. Should the fix also include OMEGA-rejection specific graveyard entries (errors the Lean oracle emitted), not just parse failures? More info per C-017 but larger blast radius if Lean errors poison context (C-022).

Q5. Alternative: instead of fixing broadcast, force agents to `append` via prompt (C-034 "mechanism not prompt" violation — rejected). Or alter the protocol so `complete` requires a prior `append` chain (mechanism — C-034 compliant). Should Step-B consider the protocol change instead of recent_rejections change?

Q6. Pre-registered A/B sample: same seed=74677 N=50 frozen sample. Is that the right granularity, or should we use a subset that hotspots tape-usage-sensitive problems?

## Audit request

Each question: CONCUR / CHALLENGE / VETO + cite (Art. / C-xxx / file:line).

Final verdict: NECESSARY_AS_PROPOSED / NECESSARY_BUT_SCOPE_DIFFERENT / NOT_NECESSARY.

If NOT_NECESSARY: what's the less-invasive alternative that achieves n↑→PPUT↑ super-linear?
