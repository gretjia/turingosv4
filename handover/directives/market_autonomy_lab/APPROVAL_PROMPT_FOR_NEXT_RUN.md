# Approval Prompt For Next Run

Use this prompt only if the user wants Codex to implement the first three
authorized atoms. It is intentionally scoped to avoid accidental Class-4
overreach.

```text
I approve the following TuringOS v4 Market Autonomy Lab Class-4 atoms for the
independent worktree:

worktree: /home/zephryj/projects/turingosv4-market-autonomy-lab
branch: codex/market-autonomy-lab-20260516

Approved atoms:

1. Atom 1 — BCAST-MARKET-COVERAGE-PATCH
   Allowed paths:
   - src/runtime/librarian_broadcast.rs
   - src/runtime/market_review.rs
   - src/runtime/market_decision_trace.rs
   - tests/constitution_librarian_*.rs
   - src/bin/audit_dashboard.rs
   - genesis_payload.toml

2. Atom 2 — EV-DIAGNOSTIC-PATCH
   Allowed paths:
   - src/runtime/ev_decision_trace.rs
   - src/runtime/economic_judgment.rs
   - experiments/minif2f_v4/src/bin/evaluator.rs
   - tests/constitution_real12_economic_judgment.rs
   - tests/constitution_real13a_ev_decision_trace.rs
   - src/bin/audit_dashboard.rs
   - genesis_payload.toml

3. Atom 3 — POLICYTRADER-BASELINE
   Allowed paths:
   - src/runtime/policy_trader_trace.rs
   - src/runtime/mod.rs
   - tests/constitution_policy_trader_trace.rs
   - src/bin/audit_dashboard.rs
   - scripts/run_real13_market_pressure_probe.sh
   - genesis_payload.toml

Authorization:
- You may update Trust Root hashes in genesis_payload.toml for the named files
  only if required by these atoms.
- You may not change TypedTx schema/discriminants.
- You may not change sequencer admission rules.
- You may not change canonical signing payloads.
- You may not change CAS ObjectType schema.
- You may not edit constitution.md or flowcharts.
- You may not introduce forced trade, price-as-truth, ghost liquidity,
  off-tape truth, f64/f32 money, raw CoT/raw prompt/raw completion/raw log
  broadcast, or live REAL-6B.
- You may not count scripted, counterfactual, or PolicyTrader action as E2.

Required workflow:
1. Keep the red gates already written.
2. Implement the smallest constitution-preserving fixes for Atom 1, then rerun
   its tests.
3. Implement the smallest constitution-preserving fixes for Atom 2, then rerun
   its tests.
4. Implement the smallest constitution-preserving fixes for Atom 3, then rerun
   its tests.
5. Rerun:
   - cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
   - cargo test --test constitution_librarian_market_no_trade -- --test-threads=1
   - cargo test --test constitution_real12_economic_judgment -- --test-threads=1
   - cargo test --test constitution_real13a_ev_decision_trace -- --test-threads=1
   - cargo test --test constitution_policy_trader_trace -- --test-threads=1
   - bash scripts/run_constitution_gates.sh
   - cargo test --workspace --no-fail-fast -- --test-threads=1
   - git diff --check
6. Request clean-context Codex audit before any ship-path or E2-candidate claim.
7. Only after green gates and audit, run true hard MiniF2F/Lean evidence using
   the hard10 floor:
   handover/preregistration/sample_E1v2_hard10_S20260423.txt
   sha256=138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc

Claim boundary:
- Do not write `E2 achieved`.
- Write only `E2 candidate pending audit` if the full evidence contract is met.
- If no live non-scripted agent-generated router/short-equivalent tx appears,
  write a clean-negative mechanism report and continue to the next bottleneck.
```

