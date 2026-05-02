# DECISION — Lamarckian Autopsy + Boltzmann Masking — 2026-05-02

**Authority**: architect directive 2026-05-02, ruling 11.4 + ruling 13 + ruling 14 + ruling 15 of Part C.
- Source: `handover/directives/2026-05-02_lossless_constitution_polymarket_directive.md`
- Verbatim: `..._part_C_updated_final_ruling.md` §2.6 (Autopsy), §2.8 (Boltzmann), §7 (Autopsy boundaries), §8 (Kelly), §9 (price-guided masking), §10 (split P_accept / P_progress)
**Status**: ratified by user authorization 2026-05-02.
**Sequencing**: lands at TB-14 (Boltzmann masking) + TB-15 (Lamarckian Autopsy / Markov Log Loom). NOT before.

---

## §1 Decision — Lamarckian Autopsy

```text
1. Autopsy is PRIVATE, agent-scoped feedback.
2. Autopsy is DERIVED from ChainTape evidence — never from agent's own LLM self-report.
3. Autopsy lands in agent-specific READ VIEW, not global Agent prompt.
4. Raw failure logs are SHIELDED.
5. Public summary aggregates only as a TYPICAL ERROR BROADCAST when N similar
   autopsies cluster.
6. Autopsy NEVER directly mutates Agent permissions; if it would, the change
   must route through ArchitectAI proposal → JudgeAI/VetoAI → canary deploy.
```

### 1.1 Canonical AgentAutopsyCapsule structure

```rust
AgentAutopsyCapsule {
    agent_id: AgentId,
    event_id: EventId,                      // bankruptcy/liquidation/forced-exit event
    loss_reason_class: LossReasonClass,     // enum, e.g. AdverseSelection | Overleverage | Goodhart
    violated_risk_rule: Option<RiskRuleId>, // protocol-level rule that triggered exit, if any
    suggested_policy_patch: Option<RiskPolicyPatchId>,
    evidence_cids: Vec<Cid>,                // ChainTape evidence anchors:
                                            //   positions, trades, prices, slippage,
                                            //   resolution, market pool state
    public_summary: ShortString,            // shareable when clustering threshold hit
    private_detail_cid: Cid,                // detailed analysis (private)
    created_at_logical_t: u64,
    created_at_round: RoundId,
}
```

### 1.2 Hard prohibitions on Autopsy

```text
A. NO global broadcast of raw liquidation logs.
   Even when N similar autopsies cluster into a typical-error broadcast,
   only the public_summary is shared, never raw_diagnostic.

B. NO LLM self-narrative as autopsy input.
   Autopsy facts come from ChainTape:
     positions, trades, prices, slippage, resolution, bankruptcy state,
     L4/L4.E entries, market pool state.
   The LLM may read the capsule afterward; it does not write it.

C. NO bypass of VetoAI for permission changes.
   Even if Autopsy recommends "remove this Agent's market access",
   the actual permission change must go through the meta loop:
     ArchitectAI proposal → JudgeAI/VetoAI → canary.

D. NO retroactive evidence rewrite (per feedback_no_retroactive_evidence_rewrite).
   Autopsy is created going-forward only. Old failures do not get
   retro-instrumented with capsules they never had.
```

### 1.3 Kelly Criterion as risk policy suggestion (NOT protocol enforcement)

```text
Kelly fraction (binary YES at price c, agent-private estimate p):
  f* = (p - c) / (1 - c)     if p > c
       0                     otherwise

Fractional Kelly (recommended cap):
  f = clamp(lambda * f*, 0, f_max)
  with lambda ∈ {0.25, 0.5}, f_max = single-node max position fraction.
```

Protocol enforces only:

```text
max_position_size
max_drawdown
max_slippage
max_leverage = 1
```

The Agent chooses its risk policy. Autopsy may **suggest** Kelly cap reduction; it does not impose.

Reason (verbatim from directive Part C §8):
> 但 TuringOS 不能强迫所有 Agent 用 Kelly。因为群体智慧需要异质策略。

---

## §2 Decision — Boltzmann Candidate Masking

### 2.1 Core rule

Boltzmann masking is a **read-view / scheduler policy**. It NEVER mutates ChainTape.

```text
Mask predicate: child may mask parent in scheduler candidate set IF
  child_price > parent_price + margin
  AND child_verification_status >= parent_verification_status
  AND child has no unresolved challenge
  AND child liquidity >= min_liquidity_threshold
ELSE
  parent remains in candidate set.

Even when masked, parent remains:
  - in ChainTape (no deletion, ever)
  - in materialized views (audit dashboard)
  - eligible for re-emergence if mask conditions reverse
  - sample-able with epsilon probability for exploration
```

### 2.2 Selection probability (Boltzmann form)

```text
P(select node_i) ∝ exp(beta * score_i)

score_i =
   w_price   * price_signal_i
 + w_verify  * verification_score_i
 + w_reuse   * reuse_score_i
 - w_risk    * challenge_risk_i
```

Exploration:

```text
With probability epsilon, sample uniformly from the unmasked set
(including parents that would otherwise be masked).
```

### 2.3 Two-axis price (NOT one)

The directive Part C §10 explicitly rejects single-price-as-two-truths:

```text
P_accept(node):  probability node passes verification / challenge
P_progress(node): probability node moves toward final goal

score(node) =
   alpha  * P_accept(node)
 + beta   * P_progress(node)
 + gamma  * novelty(node)
 - delta  * risk(node)

Masking requires BOTH:
  P_accept_child >= P_accept_parent
  AND P_progress_child > P_progress_parent + margin
```

This prevents Goodhart conflation where "looks correct" displaces "actually progresses."

### 2.4 Hard prohibitions on Boltzmann

```text
A. NO ChainTape deletion. Ever. Mask is read-view only.
B. NO predicate override. If child fails predicates, mask has no effect; child is rejected.
C. NO masking of unresolved-challenge nodes. A node under live challenge is NOT a stable mask candidate.
D. NO low-liquidity manipulation. min_liquidity_threshold prevents 1-trade flash-mask attacks.
E. NO single-price masking. Both P_accept and P_progress must support the mask.
```

---

## §3 Test obligations

### 3.1 At TB-14 (Boltzmann)

```text
1. Mask predicate gating:
   - All four conditions required; any single failure → no mask.
   - Tested on synthetic parent/child pairs.

2. ChainTape preservation invariant:
   - Before mask: ledger_root = R.
   - After mask: ledger_root = R (no mutation).
   - Materialized view shows mask state; ledger does not.

3. Predicate-override-fails test:
   - Child with high price BUT predicate failure → cannot mask parent;
     child enters L4.E.

4. Replay determinism:
   - Same chain → same mask state at any logical_t.

5. Two-axis enforcement:
   - High P_accept_child, low P_progress_child → no mask.
   - Symmetric for low P_accept.

6. Min-liquidity gate:
   - Below-threshold child cannot mask, even if price > parent price.
```

### 3.2 At TB-15 (Lamarckian Autopsy)

```text
1. Autopsy capsule derived from ChainTape only:
   - Inject deliberate LLM self-report saying "I lost because X";
     verify autopsy ignores it; relies on ledger evidence instead.

2. Private read-view scoping:
   - Agent A's autopsy is readable by Agent A only (and operators).
   - Agent B cannot retrieve Agent A's autopsy contents.

3. Public summary clustering:
   - N=3 similar autopsies trigger TypicalErrorBroadcast.
   - public_summary is shareable; raw evidence is not.

4. Permission-change bypass test:
   - Autopsy with suggested_policy_patch does NOT auto-apply.
   - Must traverse ArchitectAI → JudgeAI → canary.

5. Retroactive prohibition:
   - Old liquidation events (pre-TB-15) do NOT get retroactively
     instrumented with autopsy capsules.

6. Replay determinism:
   - Same chain → same set of autopsy capsules created.
```

---

## §4 Constitutional alignment

```text
Art. II.1  广播典型错误         — typical error broadcast pattern
Art. II.2  广播价格信号         — price as broadcast signal (statistical, not truth)
Art. III.1 屏蔽错误             — raw failure log shielding
Art. III.2 封装细节             — agent read-view scoping
Art. III.3 屏蔽相关性           — preventing global broadcast pollution
Art. III.4 屏蔽 Goodhart 问题   — two-axis price + predicate-first
Art. V.1   三权分立             — Architect/JudgeAI flow for permission changes
```

This decision implements existing constitutional principles in operational form. No constitution amendment required.

---

## §5 Open follow-ups

```text
- Specific TypicalErrorBroadcast clustering threshold (N=?) — finalize at TB-15.
- Boltzmann beta / exploration epsilon hyperparameter range — finalize at TB-14
  with smoke-evidence calibration; not a constitutional matter.
- Multi-axis score weights (alpha/beta/gamma/delta) — operator-tunable;
  must persist deterministically in chain configuration to preserve replay.
- Cross-agent autopsy referencing (e.g., Agent B reads Agent A's public_summary
  before making a similar trade) — defer; possibly RSP-M7+ scope.
```
