# Constitutional Full-Flow Benchmark Packet

Date: 2026-06-05

Worktree: `/Users/zephryj/work/turingosv4-tc-operationalization`

Source base: `39233aa7c868f0e9b37a7a29eb426279f41cf032`

Run id: `full-flow-flask5063-context-budgetfix-rerun2`

## Executive Verdict

This packet shows that the current `turingos` CLI can drive a constitutional
full-flow benchmark with FC1, FC2, and FC3 runtime receipts lit, market
participation present on tape, replay/CAS/signature reconstruction passing, and
a real SWE-bench/TDMA hidden-test task verdict attached.

The real task did not solve the SWE-bench instance. The correct task verdict is
`TASK-FAIL` with `benchmark_verdict=unresolved`. This packet therefore supports
the narrower statement: system flow and constitutional receipts passed for the
run; the selected real-world coding task remained unresolved after the declared
attempt budget.

Important scope boundary: the SWE-bench domain adapter did not land an accepted
domain `WorkTx` in this full-flow run. To exercise the full FC1/FC2/FC3 machine
after that domain rejection, `full_system_augment_current_kernel` attached a
system-participation canary and anchored market participation to that canary.
Therefore the market receipt is real and tape-visible inside the same full-flow
run, but it is not evidence that the unresolved SWE-bench domain patch itself
became an accepted market-backed work product.

No PR was opened from this work.

## Kernel Budget Design Constraint

The kernel budget model should stay generic. Kernel-level budget accounting
should use token counts and generic run limits only: prompt tokens, completion
tokens, total tokens, stage count, and attempt count. Provider brand, provider
channel, per-token price, market PnL, and currency conversion are non-kernel
derived accounting.

Future real budget can be computed outside the kernel from token receipts plus
a provider/model price table. That keeps the kernel close to the constitution
and the three flowcharts, while allowing external economic layers to price
OpenAI, DeepSeek, Anthropic, Qwen, or any future token source.

## Main Evidence

Primary benchmark packet:

```text
/tmp/turingos_real_task_benchmark_20260605/full_flow_packet_context_budgetfix_rerun2/constitutional_full_flow_benchmark_packet.json
sha256 1eec4428e6c3d6f5d73bf2f779bb54b3a5be1018ba0f0d2935034425b6bb91c1
```

Full-system participation report:

```text
/tmp/turingos_real_task_benchmark_20260605/full_flow_packet_context_budgetfix_rerun2/full_system_participation.json
sha256 bf87f554db41b8e41b0a811917a0c793e8f033d76d65ca48eeb87540b616ab5a
```

Replay report:

```text
/tmp/turingos_real_task_benchmark_20260605/full_flow_packet_context_budgetfix_rerun2/replay_report.json
sha256 9e199f1a11d73d756a50d8751862b1415d91358d59b8ed9562f4cb1364b03508
```

Real TDMA/SWE-bench task evidence:

```text
/tmp/turingos_real_task_benchmark_20260605/tdma_evidence_flask5063_context_budgetfix/manifest.json
sha256 7bb878632f0f71b21191ea922983f9bac7c829ebadee715fadde0a562bf2f1cf

/tmp/turingos_real_task_benchmark_20260605/tdma_evidence_flask5063_context_budgetfix/per_attempt_probes.jsonl
sha256 ff4aefacd91a877ea2c842e3af033c84e244029ba8882fd9df9c122fb4599d5f
```

## Commands

The full-flow packet was produced through the real `turingos` CLI:

```bash
target/debug/turingos benchmark full-flow run \
  --run-dir /tmp/turingos_real_task_benchmark_20260605/full_flow_packet_context_budgetfix_rerun2 \
  --run-id full-flow-flask5063-context-budgetfix-rerun2 \
  --constitution /Users/zephryj/work/turingosv4-tc-operationalization/constitution.md \
  --sample-json /tmp/turingos_real_task_benchmark_20260605/flask-5063-context.json \
  --llm-proxy-url http://127.0.0.1:8124 \
  --model deepseek-chat \
  --task-evidence-dir /tmp/turingos_real_task_benchmark_20260605/tdma_evidence_flask5063_context_budgetfix
```

The packet recorded these command-log steps:

```text
01_turingos_init
02_swebench_structural_smoke
03_full_system_augment
04_turingos_verify_chaintape
05_full_system_participation
```

The real TDMA/SWE-bench task was run earlier through the `turingos tdma` CLI:

```bash
target/debug/turingos tdma run \
  --workspace /tmp/turingos_real_task_benchmark_20260605/ws_context_budgetfix \
  --judge swebench \
  --role meta \
  --swebench-sample /tmp/turingos_real_task_benchmark_20260605/flask-5063-context.json \
  --swebench-python /Users/zephryj/.venv-swebench/bin/python3 \
  --swebench-workdir /tmp/turingos_real_task_benchmark_20260605/swebench_work_flask5063_context_budgetfix \
  --max-attempts-per-stage 3 \
  --evidence-dir /tmp/turingos_real_task_benchmark_20260605/tdma_evidence_flask5063_context_budgetfix
```

Proxy stats after packet generation:

```json
{
  "prompt_tokens": 1606,
  "completion_tokens": 2204,
  "total_tokens": 3810,
  "requests": 2,
  "errors": 0,
  "retries_429": 0,
  "estimated_count": 0,
  "per_key_requests": {
    "deepseek": [2]
  }
}
```

## Packet Summary

```json
{
  "schema_version": "turingosv4.benchmark.constitutional_full_flow.v1",
  "run_id": "full-flow-flask5063-context-budgetfix-rerun2",
  "entrypoint": "turingos benchmark full-flow run",
  "flow_verdict": "FLOW-PASS",
  "system_verdict": "SYSTEM-PASS",
  "task_verdict": {
    "kind": "TASK-FAIL",
    "source": "swebench_tdma_hidden_test_verifier",
    "benchmark_verdict": "unresolved",
    "closure_scope": "real_tdma_swebench",
    "final_closure_possible": false,
    "stages_completed": 0,
    "stages_total": 1,
    "total_attempts": 3,
    "leak_in_any_prompt": false
  }
}
```

Source state recorded by the packet:

```text
git_head: 39233aa7c868f0e9b37a7a29eb426279f41cf032
worktree_status_short lines: 217
```

The worktree is intentionally not presented as clean. The clean base is fixed,
but the implementation is still an uncommitted worktree change set. This is why
the packet is suitable for audit, not for immediate PR opening.

## Flowchart Receipts

FC1 runtime loop receipts:

```text
FC1-N1, FC1-N2, FC1-N3, FC1-N4, FC1-N5, FC1-N6, FC1-N7, FC1-N8,
FC1-N9, FC1-N10, FC1-N11, FC1-N12, FC1-N13, FC1-N14, FC1-N15
```

FC2 boot/full-architecture receipts:

```text
FC2-N16, FC2-N17, FC2-N18, FC2-N19, FC2-N20, FC2-N21, FC2-N22,
FC2-N23, FC2-N24, FC2-N25, FC2-N26, FC2-N27, FC2-N28
```

FC3 meta-architecture receipts:

```text
FC3-N29, FC3-N30, FC3-N31, FC3-N32, FC3-N33, FC3-N34, FC3-N35,
FC3-N36, FC3-N37, FC3-N38
```

Full-system verdict:

```json
{
  "full_system_participation": true,
  "full_system_verdict": "FULL_SYSTEM_LIT",
  "missing": [],
  "final_closure_possible": false
}
```

## Market Participation

Market participation was not merely a dashboard row. It was present in the
ChainTape/CAS-derived full-system participation report.

However, the market anchor was a system-participation canary inserted after the
SWE-bench domain adapter rejected the domain work. This distinction is
load-bearing for audit:

```json
{
  "domain_accepted_work_tx_id": null,
  "domain_rejected_work_l4e_count": 1,
  "domain_manifest_work_tx_landed": false,
  "market_anchor_source": "system_participation_canary_after_domain_rejection",
  "system_canary_work_tx_id": "worktx-full-system-participation-canary-full-system-canary"
}
```

Interpretation: market participation was real, signed, and on tape for the
constitutional full-flow run. It was not a market-backed acceptance of the
failed SWE-bench patch.

```json
{
  "present": true,
  "mode": "invest",
  "l4_market_tx_count": 3,
  "market_seed_txs": 1,
  "cpmm_pool_txs": 1,
  "cpmm_swap_txs": 0,
  "buy_with_coin_router_txs": 1,
  "event_resolve_txs": 0,
  "agent_market_action_txs": 1,
  "market_decision_trace_count": 1,
  "market_decision_submitted_count": 1,
  "market_decision_no_trade_count": 0,
  "market_decision_declined_count": 0,
  "market_opportunity_trace_count": 0
}
```

Transaction kind counts:

```json
{
  "architect_commit": 1,
  "architect_proposal": 1,
  "buy_with_coin_router": 1,
  "cpmm_pool": 1,
  "escrow_lock": 2,
  "log_feedback_archive": 1,
  "map_reduce_tick": 1,
  "market_seed": 1,
  "predicate_binding_activate": 1,
  "reinit_boot": 1,
  "reinit_request": 1,
  "task_open": 3,
  "terminal_summary": 1,
  "veto_decision": 1,
  "work": 1
}
```

## Replay Summary

```json
{
  "l4_entries": 18,
  "l4e_entries": 2,
  "ledger_root_verified": true,
  "system_signatures_verified": true,
  "state_reconstructed": true,
  "economic_state_reconstructed": true,
  "cas_payloads_retrievable": true,
  "agent_signatures_verified": true,
  "proposal_telemetry_cas_retrievable": true,
  "run_id": "full-flow-flask5063-context-budgetfix-rerun2",
  "epoch": 2,
  "detail": {
    "final_state_root_hex": "5d3ba2af71f530a7a35f15444fff3f318bbce99662fa6d2c110ac82e6b3a2256",
    "final_ledger_root_hex": "773c29fe2bf75af3b6335a058f09d4a39a5eea9b9b3744f6908498347a068a64",
    "head_commit_oid_hex": "56cf49c0dd6364dc95d19ee8eda593bed73d755a",
    "l4e_last_hash_hex": "2272a22b4eb07b92d824c3c8fc8218436b915211022a1fc703a492d6b06e7fbd",
    "replay_failure": null,
    "initial_q_state_loaded_from_disk": true
  }
}
```

## Real SWE-bench Task Result

Task instance: `pallets__flask-5063`

TDMA manifest summary:

```json
{
  "run_id": "turingos-tdma-swebench",
  "model_label": "deepseek-v4-pro",
  "problem_label": "turingos tdma --judge swebench pallets__flask-5063",
  "max_attempts_per_stage": 3,
  "stages_total": 1,
  "stages_completed": 0,
  "total_attempts": 3,
  "total_failed_attempts": 3,
  "distinct_judge_classes": ["hidden_test_failure"],
  "leak_in_any_prompt": false,
  "all_prompts_within_budget": true,
  "b_prompt_max": 5800,
  "prompt_tokens_min": 3578,
  "prompt_tokens_max": 3630,
  "total_llm_prompt_tokens": 9246,
  "total_llm_completion_tokens": 23075,
  "total_wall_clock_ms": 473870,
  "per_stage": [
    {
      "stage": "Repair",
      "attempts_used": 3,
      "final_constraints": 0,
      "outcome": "cap-reached"
    }
  ]
}
```

Per-attempt probe summary:

```text
Attempt 1: Retry, hidden_test_failure, patch failed to apply.
Attempt 2: Retry, hidden_test_failure, patch applied but two FAIL_TO_PASS tests remained unresolved.
Attempt 3: Retry, hidden_test_failure, patch failed to apply.
```

Official SWE-bench harness report excerpt:

```json
{
  "pallets__flask-5063": {
    "patch_is_None": false,
    "patch_exists": true,
    "patch_successfully_applied": true,
    "resolved": false,
    "tests_status": {
      "FAIL_TO_PASS": {
        "success": [],
        "failure": [
          "tests/test_cli.py::TestRoutes::test_subdomain",
          "tests/test_cli.py::TestRoutes::test_host"
        ]
      },
      "PASS_TO_PASS": {
        "failure": []
      }
    }
  }
}
```

This is the reason the packet must remain `TASK-FAIL/unresolved`.

## Verification Gates

The active implementation worktree passed the broad constitution gates:

```text
cargo test --test constitution_matrix_drift --no-fail-fast
bash scripts/run_constitution_gates.sh
[k-1-5] total=165 failed=0
```

Focused checks used during the run:

```text
cargo test --test cli_benchmark_full_flow --no-fail-fast -- --test-threads=1
cargo test swebench_test_judge --lib --no-fail-fast
cargo test distiller_in_budget --lib --no-fail-fast
cargo test --bin turingos swebench_prompt --no-fail-fast
cargo test --test constitution_production_module_liveness every_exported_module_has_exactly_one_liveness_group --no-fail-fast -- --test-threads=1
cargo test --test constitution_obligation_repair_reconciliation --no-fail-fast -- --test-threads=1
git diff --check
```

Observed results:

```text
cli_benchmark_full_flow: 3/3 passed
swebench_test_judge lib tests: 11/11 passed
distiller focused tests: 2/2 passed
turingos swebench_prompt tests: 2/2 passed
production_module_liveness targeted gate: 1/1 passed
obligation reconciliation: 3/3 passed
git diff --check: no output
restricted surface scan: no restricted source file matched
```

The broad changed-file claim/secret scan is noisy because this large worktree
contains historical reports, tests that intentionally mention forbidden claim
words, credential environment variable names, and raw-stderr shielding tests. Those
hits were reviewed as test/document vocabulary, not as actual secret values or
task-success claims.

## Code Excerpts for Audit

### Full-flow packet refuses false task success

```rust
let task_verdict = if let Some(task_evidence_dir) = &args.task_evidence_dir {
    task_verdict_from_tdma_evidence(task_evidence_dir)?
} else {
    task_verdict_from_domain(&domain)
};
if args.require_task_pass && task_verdict.kind != "TASK-PASS" {
    write_packet(...)?;
    return Err("--require-task-pass set but task verifier did not return TASK-PASS".into());
}
```

### TDMA task evidence determines real task verdict

```rust
let stages_completed = required_u64(&manifest, "stages_completed", &manifest_path)?;
let stages_total = required_u64(&manifest, "stages_total", &manifest_path)?;
let leak_in_any_prompt = manifest
    .get("leak_in_any_prompt")
    .and_then(Value::as_bool)
    .unwrap_or(true);
let task_pass = stages_total > 0 && stages_completed == stages_total && !leak_in_any_prompt;
Ok(TaskVerdict {
    kind: if task_pass { "TASK-PASS" } else { "TASK-FAIL" }.to_string(),
    source: "swebench_tdma_hidden_test_verifier".to_string(),
    benchmark_verdict: Some(if task_pass { "resolved" } else { "unresolved" }.to_string()),
    closure_scope: Some("real_tdma_swebench".to_string()),
    final_closure_possible: task_pass,
    stages_completed: Some(stages_completed),
    stages_total: Some(stages_total),
    total_attempts,
    leak_in_any_prompt: Some(leak_in_any_prompt),
})
```

### SWE-bench judge calls the official harness

```rust
let patch = canonicalize_unified_diff_for_harness(&patch);
let pred = serde_json::json!({
    "instance_id": self.instance_id,
    "model_name_or_path": self.model_name,
    "model_patch": patch,
});
let cmd = SanitizedCommand {
    program: self.python_bin.clone(),
    args: vec![
        "-m".into(),
        "swebench.harness.run_evaluation".into(),
        "--dataset_name".into(),
        self.dataset_name.clone(),
        "--predictions_path".into(),
        preds_abs.to_string_lossy().into_owned(),
        "--instance_ids".into(),
        self.instance_id.clone(),
        "--run_id".into(),
        run_id.clone(),
        "--namespace".into(),
        "none".into(),
        "--max_workers".into(),
        "1".into(),
        "--cache_level".into(),
        "instance".into(),
    ],
    cwd: self.work_dir.clone(),
    env: swebench_harness_env(),
    stdin: None,
    timeout: Duration::from_secs(60 * 60),
};
```

### Distiller shields raw verifier output from prompts

```rust
const MAX_TRACE_FRAME_CHARS: usize = 240;

pub struct TraceView {
    pub schema_version: String,
    pub reject_class: String,
    pub failed_predicate: String,
    pub top_frames: Vec<String>,
    pub bottom_frames: Vec<String>,
    pub touched_paths: Vec<String>,
    pub stderr_tail: String,
    pub raw_stderr_sha256: String,
}
```

The implementation truncates stack-frame lines and carries only structured
fields plus a raw-stderr hash into retry state.

### Full-system market augmentation avoids agent identity collisions

```rust
let trader_agent = select_unused_funded_agent(
    &mut reserved,
    &q,
    "full-system market trader",
    TRADER_BUY_MICRO,
)?;
```

```rust
fn select_unused_funded_agent(
    used: &mut BTreeSet<String>,
    q: &QState,
    role: &str,
    required_micro: i64,
) -> Result<String, String> {
    let mut candidates: Vec<(String, i64)> = q
        .economic_state_t
        .balances_t
        .0
        .iter()
        .filter_map(|(agent_id, balance)| {
            let id = agent_id.0.clone();
            let balance_micro = balance.micro_units();
            if used.contains(&id) || balance_micro < required_micro {
                None
            } else {
                Some((id, balance_micro))
            }
        })
        .collect();
    candidates.sort_by(|(agent_a, balance_a), (agent_b, balance_b)| {
        balance_b.cmp(balance_a).then_with(|| agent_a.cmp(agent_b))
    });
    if let Some((candidate, _balance_micro)) = candidates.into_iter().next() {
        used.insert(candidate.clone());
        return Ok(candidate);
    }
    Err(format!(
        "no unused funded identity with at least {required_micro} available for {role}; ..."
    ))
}
```

## Audit Caveats

1. The benchmark packet passed flow/system gates but did not solve
   `pallets__flask-5063`.
2. The packet source state records a dirty worktree with 217 status lines.
   This is acceptable for audit only if reviewers treat the packet as
   uncommitted implementation evidence, not as a release.
3. The JSON packet records `command_log`, not `proxy_stats`; proxy stats are
   included in this report for auditor convenience.
4. Market participation was present in the same full-flow run, but it was
   anchored to a `system_participation_canary_after_domain_rejection`, not to an
   accepted SWE-bench domain `WorkTx`. Market price did not override the task
   verifier. The task stayed unresolved.
5. A separate clean-context audit noted broader inherited JudgeAI tactic
   calibration concerns around default forbidden-pattern coverage and
   `append_oracle_accepted`. That surface is outside this OBL-015 benchmark
   path and touches restricted Bus/kernel-adjacent authority, so it is not fixed
   here. It should be handled as a separate explicit governance/JudgeAI task if
   reopened.
6. Lean is not part of the kernel authority for this run. Math/Lean remain
   workload or verifier layers. Kernel-level budget should remain token-count
   based and generic.

## Recommended Audit Questions

1. Does `task_verdict_from_tdma_evidence` correctly prevent structural smoke
   from being called real task success?
2. Does `full_system_participation.json` derive FC receipts from replayed
   ChainTape/CAS state rather than static flowchart prose?
3. Does market participation require an actual L4 market action or a
   tape-visible abstention, rather than dashboard-only text?
4. Are raw verifier outputs kept out of retry prompts and ordinary agent views?
5. Are provider prices and brand-specific economics kept out of kernel budget
   accounting?
6. Are the dirty worktree and unresolved SWE-bench task caveats prominent
   enough to block premature PR opening?
