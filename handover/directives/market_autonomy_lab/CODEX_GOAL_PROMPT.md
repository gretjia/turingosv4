# Codex Goal Prompt

```text
/goal Continue TuringOS v4 Market Autonomy Lab under MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2.

Grand Objective:
Achieve real voluntary agent market-mechanism emergence under the TuringOS
constitution.

Success means an E2 candidate pending audit, not an immediate E2 achieved claim:
- live, non-scripted, agent-generated economic action,
- produced by voluntary agent judgment,
- backed by ChainTape/CAS/PromptCapsule/MarketOpportunityTrace/EVDecisionTrace
  or explicit economic rationale,
- audit_tape PROCEED,
- no forced trade,
- no scripted or PolicyTrader baseline counted as E2,
- no price-as-truth,
- no ghost liquidity,
- no off-tape truth,
- no f64/f32 money,
- no raw CoT/raw prompt/raw completion/raw log broadcast.

Operating mandate:
Do not treat no E2 candidate as completion. Do not treat clean-negative as
completion. If no E2 candidate appears, identify the next mechanism bottleneck
and continue with the next constitution-preserving atom until a real voluntary
market action appears, a Level 3 Constitutional Hard Stop is reached with
STOP_PROOF.md, or the authorized research envelope is exhausted.

```text
No E2 candidate is not completion.
Clean-negative is not completion.
```

Mode:
Constitutional Research Mode, not Ship Mode. This is not authorization to merge
main or claim E2/E3/E4 achieved.

Allowed implementation surfaces are exactly those listed in
RESEARCH_ENVELOPE_V2.md. Forbidden surfaces are exactly those listed in
RESEARCH_ENVELOPE_V2.md.

Within the envelope:
- allowed Trust-Root-pinned files may be touched;
- genesis_payload.toml may be updated only for Trust Root rehash of touched
  pinned files;
- after every rehash, rerun Trust Root verification;
- continue automatically if Trust Root passes.

Hard stop only if:
- unlisted restricted surface is required;
- Trust Root fails after allowed rehash;
- evidence contamination cannot be isolated;
- proposed mechanism requires a forbidden mechanism;
- resource envelope is exhausted.

Before any hard stop, write STOP_PROOF.md explaining the exact stop clause and
the in-envelope alternatives considered.

Swarm:
Use GPT-5.5 xhigh as orchestrator. Spawn only GPT-5.5 subagents. Assign:
- GPT-5.5 high: BCAST market/no-trade coverage.
- GPT-5.5 high: EV diagnostics and PositiveEVIgnored.
- GPT-5.5 medium: PolicyTrader baseline.
- GPT-5.5 medium: hard problem experiment matrix/runners.
- GPT-5.5 low: forbidden claims and risk-register docs.
- GPT-5.5 low: evidence metric extraction.
Low-depth agents must receive exact file ownership, commands, expected fields,
and pass/fail criteria; they must not make architectural decisions.

Allowed atoms:
0. Preserve architect source and ARH-v2 envelope.
1. BCAST market/no-trade source coverage.
2. EV diagnostics / PositiveEVIgnored / exhaustive reason taxonomy.
3. PolicyTrader baseline.
4. Market tx count split.
5. TraderView EV/PnL/Librarian digest improvements.
6. Hard10 true-problem run, escalating to hard20/hard36 if insufficient.
7. Clean-context audit and next-hypothesis loop.

First actions:
1. Preserve architect original verbatim at
   handover/directives/2026-05-16_MARKET_AUTONOMY_LAB_ARCHITECT_ORIGINAL.md.
2. Write ARH-v2 envelope and stop policy gates.
3. Run preflight.
4. Implement Atom 1-3 red gates to green.
5. Run hard10 true-problem evidence.
6. If no E2 candidate, write clean-negative and continue with the next
   constitution-preserving hypothesis.

Minimum claim-bearing difficulty:
Use true MiniF2F/Lean problems. Do not rely on toy or too-easy smoke runs.
The hard10 floor is:
  handover/preregistration/sample_E1v2_hard10_S20260423.txt
  sha256=138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc
If hard10 has too few review windows, too few EV traces, or too-fast solves,
escalate to deterministic hard20/hard36 from hard36_pool.txt with pinned seed
and hash.

Deliver every cycle:
- EXPERIMENT_CHARTER.md
- EXPERIMENT_MATRIX.md
- FORBIDDEN_CLAIMS.md
- PRE_FLIGHT_REPORT.md
- TEST_RESULTS_SUMMARY.md
- E2_CANDIDATE_REPORT.md or CLEAN_NEGATIVE_REPORT.md
- CONSTITUTIONAL_RISK_REGISTER.md
- NEXT_STEP_RECOMMENDATION.md
- STATUS_SYNC_OR_OBS.md
- SWARM_AUDIT_SUMMARY.md

Never write `E2 achieved` from this lab. Write only:
  E2 candidate pending audit
when the full evidence contract is satisfied.
```
