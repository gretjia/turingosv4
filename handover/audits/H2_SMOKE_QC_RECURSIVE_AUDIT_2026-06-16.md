# H-HET-2 live-smoke — adversarial mid-flight QC + recursive audit (2026-06-16)

Independent clean-context audit artifact (AGENTS.md §9 / §17.1 G5). 6 refuting auditors (3 Opus on
thesis-critical, 3 Sonnet on mechanical) + 1 Opus adjudicator, no implementer transcript. Subject:
the H-HET-2 live mechanism smoke `smoke_ucb_001`, branch `claude/het-converge-2026-06-16` @ `205fb5d9`,
artifacts `/tmp/smoke_ucb_repo` + `/tmp/smoke_ucb_cas`. Each verdict cites file:line and byte-decoded
CAS evidence. Full machine output: workflow `wf_78a46e85-b73` (task `ws8v3ds8j`), 837k subagent tokens.

## Overall verdict: **QC-CONCERNS → CONTINUE_STEP6** (no VIOLATION; all 6 `blocks_continuation=false`)

Three core claims independently re-verified at the byte level and HOLD:
1. omega is a real Lean 4.24.0 verification (node1 VerifierResult `exit_code=0/verified=true/Verified`).
2. model_id is genuinely on the canonical tape (2 ProposalTelemetry.v2 + 2 BudgetAllocationTelemetry.v1 decode byte-exact).
3. replay byte-clean (`replay.json` all-true, `replay_failure=null`, 14 L4).

The mechanism is real and sound — NOT seeding, NOT a stub, NOT a §17.3 name-lie. Every CONCERN is a
"witnessed LESS than claimed" scope/maturity gap, not a "witnessed something FALSE".

## Per-dimension verdicts
| dim | focus | verdict | key finding (byte/line evidence) |
|-----|-------|---------|----------------------------------|
| A | router reality (§17.3) | CONCERN | NOT seeding & NOT a stub (refuted). But `selection_reason=TieBreak` on tape for BOTH ticks (scores tied 42500 / 43525) → winners by lexicographic tie-break (`routing_policy.rs:215`), NOT value-driven. UCB/price machinery real but non-decisive at 2 ticks. |
| B | omega + axiom safety | **PASS** | node1 CAS `exit_code=0/Verified`; axiom gate fail-closed after adding Classical.choice (`lean_judge.rs:75/95/486-493`); `native_decide` double-gated. Non-blocking: `verify_axioms_after_success` emits `axioms=[]`; `axiom_gate()` dead. |
| C | model_id authenticity | CONCERN | on-tape model_id = REQUESTED label (`lean_market_agent.rs:760/2079/2426`); proxy echoes `raw_model`, discards upstream `resp.model` (`llm_proxy.py:412`); driver parses but never uses `response.model`. manifest `model="deepseek-chat"` = dead back-compat default. |
| D | cost recomputability | CONCERN | `MODEL_RATES` compile-time const (`market_tape_shared.rs:146-159`), not on CAS / not hash-pinned; `derive_cost()` needs it as side-input. RoutingPolicyConfig IS SHA-pinned (algorithm params only). |
| E | gate vacuity + freshness | CONCERN | 3 tape gates genuinely failable but FIXTURE-LOCAL — none fire on the live smoke tape. Binary freshness UNVERIFIABLE (no binary-hash/source_commit in manifest; binary rebuilt in place post-run; circumstantial source-identity case strong). |
| F | budget half dormant? | CONCERN | REFUTED: BudgetAllocationTelemetry IS on tape (2 records, `object_type=Generic` + `schema_id=...budget_allocation_telemetry.v1`, blobs 18cb8c62/22ec434f). Narrower defect stands: run-path conservation cosmetic (`allocated_token_budget` hardcoded 900 vs balance −1, `lean_market_agent.rs:2139-2142`); replay doesn't reconstruct allocation. |

## Overstated claims corrected (in `H2_LIVE_MECHANISM_SMOKE_REPORT.md` + memory)
1. "value-driven UCB+price routing decision" → "tie-break leg; value machinery non-decisive at 2 ticks".
2. "model_id reflects the ACTUAL model" → "router-REQUESTED label; served-model unverified".
3. "cost recomputable from tape" → "recomputable given tape AND pinned MODEL_RATES const".
4. "BudgetAllocationTelemetry absent" → "present (object_type=Generic + schema_id)".
5. "demonstrates budget conservation" → "witnesses routing; conservation NOT closed".
6. any PROVEN/value-driven headline → "mechanism-existence witness, not confirmatory evidence".

## Required fixes BEFORE the paid confirmatory run (not before Step 6)
served_model provenance (C) · MODEL_RATES→CAS (D) · run-path conservation (F) · BudgetAllocationTelemetry
replay reconstruction (E/F) · binary-hash+HEAD in manifest (E) · + decision_source tape promotion (Class-4)
· GA-9 (Class-4 §8) · prereg freeze. Non-blocking bookkeeping: populate `axioms` from `parse_axiom_set`; remove dead `axiom_gate()`.
