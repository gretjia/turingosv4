# OBS — Art. III.4 Prompt Pollution via acc.record_tool_stdout (2026-05-02)

**Class**: Observation (constitutional risk; architect verdict-silent)
**Driver**: Claude self-assessment 2026-05-02 ultrathink turn surfaced this; architect verdict 2026-05-02 did NOT explicitly address.
**Status**: **CORRECTED — premise stale per Codex round-1 ship-audit Q10 (2026-05-02).** No Art. III.4 prompt-pollution risk; original concern is empirically unfounded. Retained for traceability + closure path. **Not a TB-7R blocker (and not a future-ruling required item either).**

---

## §0 Correction (2026-05-02 post Codex round-1)

Codex round-1 ship audit Q10 verified the actual code paths and found:

> "I found no concrete cross-agent raw Lean error pollution in full-n5
> evidence; it contains no prompt/stdout transcript and only two synthetic
> audit-trail rows. Code also contradicts the OBS premise:
> `acc.record_tool_stdout` only increments token cost
> (`cost_aggregator.rs:57`). Prompt errors come from bounded class labels
> via `evaluator.rs:1344` and `bus.rs:576`, not raw Lean text."

The remainder of this document (§1–§7 below) is preserved as the
original concern + closure-path narrative, but the central premise —
that `acc.record_tool_stdout(&reason)` flows raw Lean diagnostics into
prompt context — is **incorrect**. Actual data flow:

1. `PartialVerdict::Reject(reason)` → `bus.record_rejection(agent_id, class.label())` (bounded label, not raw)
2. `PartialVerdict::Reject(reason)` → `acc.record_tool_stdout(&reason)` (token-cost accounting via `cost_aggregator.rs:57`; **does NOT write to prompt errors_history**)
3. Next-iteration prompt errors come from `prompt_builder` reading the bounded class labels at `evaluator.rs:1344` and `bus.rs:576` — never the raw `reason`

Because the raw `reason` is bounded to token-cost accounting, there is no
agent-to-agent diagnostic leakage path even in multi-agent runs.
Art. III.4 selective-broadcasting / shielding is honored.

**Closure**: this OBS is closed-as-empirically-unfounded. The
`coverage_denominator` OBS (companion file) absorbs the *Reject path
not reaching chain* concern, which is a coverage question, not a
prompt-pollution question.

---

## §1 The observation

Constitution Art. III.4 (selective broadcasting / shielding):

> "失败候选不能污染其他 Agent 上下文。
> 顶层白盒对系统信息做 量化、广播、屏蔽。"

In `experiments/minif2f_v4/src/bin/evaluator.rs:2430-2438` (`step` tool's
`PartialVerdict::Reject` branch):

```rust
PartialVerdict::Reject(reason) => {
    let class = classify_lean_error(&reason);
    bus.record_rejection(agent_id, class.label());
    // PPUT-CCL B2: step rejection reason flows into next prompt.
    acc.record_tool_stdout(&reason);
    *tool_dist.entry("step_reject".into()).or_insert(0) += 1;
    let preview = reason.chars().take(200).collect::<String>();
    warn!("[tx {}] step rejected ({}): {}", tx, class.label(), preview);
}
```

The line `acc.record_tool_stdout(&reason)` flows the **raw Lean error
reason** (full stderr text from Lean's verify_partial call) back to the
next prompt's "errors" section. The classifier label `class.label()`
also flows separately, but the raw `reason` is preserved verbatim.

**Implication under Art. III.4**: any agent reading the next-iteration
prompt sees the raw Lean error from a prior step's rejection. This means:

- A single agent can leak its own private failure modes into its own
  next-prompt. **This may be acceptable** (agent learning from its own
  mistakes is the design intent of PPUT-CCL B2).
- **Multi-agent runs (n5, n10, etc.)**: if `acc` is shared across
  agents (or if agents share the kernel.tape that includes prompt
  history), one agent's raw Lean error leaks into another agent's
  prompt. **This violates Art. III.4** — failure pollutes other-agent
  context.

## §2 The architectural ambiguity

Whether `acc` is per-agent or shared determines whether this is a
violation. Reading the code:

- `acc: AccountingState` is a per-run accumulator passed through the
  swarm loop.
- `acc.record_tool_stdout(reason)` accumulates into `errors_history`
  field that flows to `prompt_builder.build_prompt(...)`.
- In single-agent runs (n1), this is "agent learns from itself" — OK.
- In multi-agent runs (n5+), the prompt sharing semantic is currently
  NOT cleanly per-agent — `acc` is shared across the swarm tick.

A definitive answer requires reading `prompt_builder` + `acc` + the
swarm dispatch code in detail. Until then, this OBS records the risk.

## §3 Why architect verdict 2026-05-02 was silent

Claude raised this in self-assessment 2026-05-02 (the conversation that
preceded the architect's verdict). The architect verdict directly
addressed the parent_tx criterion gap and the coverage denominator
(see `OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md`), but did not
respond to the Art. III.4 prompt-pollution question.

Two possible readings:
- (a) Architect implicitly accepts the current PPUT-CCL B2 design
  (errors flow back; agent self-correction matters more than strict
  shielding). Under verdict A1=B′ + the n1 smoke's design intent,
  this is the more likely reading.
- (b) Architect didn't notice the question; it's pending.

Under either reading, this is **NOT a TB-7R ship blocker** per the
verdict's listed ship conditions. It is logged here so a future
constitutional review (or a future TB charter that touches the prompt
feedback path) can address it explicitly.

## §4 Why TB-7R is silent (deliberately)

TB-7R was scoped as "Constitution-Aligned Frame B Repair" — repairing
the L4/L4.E split, predicate evidence, genesis report, parent_tx state.
It was NOT scoped to redesign `acc.record_tool_stdout` semantics.
Touching that path would change LLM error-feedback shape (potentially
degrading PPUT) — a feature/performance trade-off requiring explicit
architect endorsement.

## §5 Recommended path (post-TB-7R)

When the architect next reviews the constitution-vs-PPUT trade-off,
they should rule on:

1. Is `acc.record_tool_stdout(raw_reason)` a violation of Art. III.4
   in multi-agent runs?
2. If yes, two fix options:
   - **Strict shielding**: change `acc.record_tool_stdout(raw_reason)`
     to `acc.record_tool_stdout(class.label())` — only classifier
     label flows back. PPUT may degrade because agent loses
     fine-grained Lean error detail.
   - **Per-agent isolation**: keep raw reasons but make `acc` strictly
     per-agent so an agent's reasons only flow into its own next
     prompt, never another agent's. Requires refactoring `acc` to
     `acc[agent_id]` map.
3. Either fix should also be reflected in
   `OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md`'s post-TB-7R hardening
   (which would route Reject through chain) — the chain-side
   `raw_diagnostic_cid` shielding can complement evaluator-side
   prompt-shielding.

## §6 Cross-references

- Constitution Art. III.4 (selective broadcasting / shielding) — `constitution.md`
- Code site: `experiments/minif2f_v4/src/bin/evaluator.rs:2430-2438`
- Companion OBS: `handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md`
- TB-7R verdict: `handover/directives/2026-05-02_TB7R_PARENT_TX_DAG_SMOKE_VERDICT.md`

## §7 Closure path

This OBS closes when the architect rules on the trade-off (§5) and
the chosen fix is implemented + verified by a multi-agent test that
demonstrates one agent's Lean error never appears in another agent's
prompt (under strict shielding) OR one agent's prompt only includes
its own prior errors (under per-agent isolation).
