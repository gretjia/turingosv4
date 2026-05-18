# REAL-17 P4c Evidence Decision

Date: 2026-05-17

Status:

```text
candidate hardening evidence / not an E-level achieved claim
```

## Claim Boundary

P4c may be cited as REAL-17 direct PromptCapsule provenance hardening
evidence for the candidate-only market-autonomy line.

Allowed wording:

```text
E2 candidate pending audit
direct PromptCapsule provenance hardening evidence
market emergence candidate -- final audit PROCEED, hardening pending
```

Forbidden wording category:

```text
any achieved/proven/shipped E-level or market-emergence label
```

## Evidence Directory

```text
handover/evidence/market_autonomy_lab_real17P4c_tisr_main_hard10_direct_prompt_provenance_20260517T234451Z/arm_D
```

Parent benchmark directory:

```text
handover/evidence/market_autonomy_lab_real17P4c_tisr_main_hard10_direct_prompt_provenance_20260517T234451Z
```

The run binds to git commit:

```text
4360dcbf REAL-17 stabilize P4 evidence runner
```

## Pinned Inputs

```text
problem_set_sha256       138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc
model_assignment_sha256  72ddf962d103f406892c3be43906a6277d22bad265d9acc667d31b907ea43b9c
budget_sha256            cf225ff28c36d2fad8a72aeeb2f1f002cd11f70bee70b2a5cb72445c0ee7d5cb
```

Model assignment:

```text
ACTIVE_MODEL=deepseek-chat
AGENT_MODELS=
PHASE_D_HETERO_OK=1
TURINGOS_REAL5_ROLE_ASSIGNMENT=BullTrader,BearTrader,Solver,Verifier,Challenger
TURINGOS_G_PHASE_N_AGENTS=5
LLM_PROXY_URL=http://localhost:8080
```

Budget/config:

```text
MAX_TRANSACTIONS=30
PER_PROBLEM_TIMEOUT_S=900
TURINGOS_REAL6A_POLL_BUDGET_MS=120000
TURINGOS_REAL13_CANDIDATE_AMOUNT_MICRO=1000
TURINGOS_REAL13_POLICY_TRADER_THRESHOLD_BPS=0
```

## Audit Tape

File:

```text
aggregate_verdict.json
sha256=66bcb354a2f4110062e9a13fe9debdd9e0e57fc3f0c8f8fc5b732aedee78ca29
```

Result:

```text
verdict  PROCEED
passed   41
failed   0
halted   0
skipped  11
```

Tape root:

```text
l4_count             73
l4e_count            79
cas_object_count     1419
constitution_hash    eec695459c71fbef3685583485deb431fe3b561657b2f285b7c5e7e220e59e03
```

Relevant tx counts:

```text
task_open                 10
escrow_lock               10
market_seed               10
cpmm_pool                 10
buy_with_coin_router      13
event_resolve             10
terminal_summary          10
```

## Strict Exact-Join Verifier

File:

```text
REAL17P4C_E2_VERIFIER_STRICT_DIRECT.json
sha256=d599d929d4b47cd4a8fd3e15651b784b3af6e07ca36a93cbbe2074808bca1e71
```

Command:

```bash
cargo run --quiet --bin real14_e2_candidate_verifier -- \
  --repo handover/evidence/market_autonomy_lab_real17P4c_tisr_main_hard10_direct_prompt_provenance_20260517T234451Z/arm_D/runtime_repo \
  --cas handover/evidence/market_autonomy_lab_real17P4c_tisr_main_hard10_direct_prompt_provenance_20260517T234451Z/arm_D/cas \
  --require-direct-prompt-capsule-provenance \
  --json-out handover/evidence/market_autonomy_lab_real17P4c_tisr_main_hard10_direct_prompt_provenance_20260517T234451Z/arm_D/REAL17P4C_E2_VERIFIER_STRICT_DIRECT.json \
  --md-out handover/evidence/market_autonomy_lab_real17P4c_tisr_main_hard10_direct_prompt_provenance_20260517T234451Z/arm_D/REAL17P4C_E2_VERIFIER_STRICT_DIRECT.md
```

Result:

```text
verdict                                      PROCEED
l4_router_tx_count                           13
submitted_trace_tx_count                     13
exact_join_count                             13
direct_prompt_capsule_provenance_count       13
missing_direct_prompt_capsule_provenance     0
scripted_fixture_tx_count                    0
policy_counts_for_e2                         false
bcast_shielding                              PASS
```

Interpretation:

```text
P4c closes the prior indirect PromptCapsule linkage gap for this D-arm run.
All 13 exact-join rows have direct PromptCapsule provenance via
MarketDecisionProvenanceLink.
```

Residual risk:

```text
Some matched rows still report multiple EVDecisionTrace rows matching the
same agent/event/action and rely on exact router tx join for disambiguation.
This is a precision/reporting risk, not a missing direct PromptCapsule
provenance risk.
```

## Persistence Binding

File:

```text
PERSISTENCE_BINDING_REPORT.json
sha256=e211a29c9f4da103f09dd8ed44dd2369438d1846dc0ac05bc501a6629fd65dbf
```

Result:

```text
is_passing  true
n_witnessed 3
```

Witnessed fields:

```text
balances_total_micro 35000000 -> 32987000 across 10 tasks
model_identity       deepseek-chat stable across 10 tasks
pnl final_delta_micro=-1000000
```

## Performance Boundary

File:

```text
REAL16_MARKET_PERFORMANCE_REPORT.json
sha256=18f7a0b1329fbbd4fb28fe69aa1911176abfee24dfb7df05b4ef5ffacd4fee8f
```

Result:

```text
verdict       Veto
failure       fewer_than_two_ab_arms
arm_count     1
e4_candidate  false
```

This veto blocks any P4c A/B or E4 performance claim. It does not negate the
D-arm strict exact-join/direct-provenance hardening evidence above.

## Decision

```text
P4c is PROCEED as direct PromptCapsule provenance hardening evidence for
candidate-only market action.
```

P4c should not be used as:

```text
E4 evidence
replicated E2 evidence by itself
two-sided market stability evidence
market emergence proof
ship evidence
```

## Next In-Envelope Step

Proceed to the REAL-17 hardening items that P4c does not answer:

```text
1. NO-side stability probe.
2. Single uninterrupted multi-arm A/B/C/D or ablation run.
3. EV-to-action conversion analysis on TISR-main evidence.
4. Stronger E4a/E4b benchmark with pinned mixed difficulty set.
```
