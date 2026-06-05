# Constitutional Full-Flow Benchmark Clean-Context Audit

Date: 2026-06-05

Auditor: Locke (`019e957d-6909-7161-95d0-8857fed0672e`)

Scope: OBL-015 benchmark packet and implementation evidence for
`turingos benchmark full-flow run`.

Evidence reviewed:

```text
/Users/zephryj/work/turingosv4-tc-operationalization/handover/reports/CONSTITUTIONAL_FULL_FLOW_BENCHMARK_PACKET_2026-06-05.md
/tmp/turingos_real_task_benchmark_20260605/full_flow_packet_context_budgetfix_rerun2/constitutional_full_flow_benchmark_packet.json
/tmp/turingos_real_task_benchmark_20260605/full_flow_packet_context_budgetfix_rerun2/full_system_participation.json
/tmp/turingos_real_task_benchmark_20260605/full_flow_packet_context_budgetfix_rerun2/replay_report.json
/tmp/turingos_real_task_benchmark_20260605/tdma_evidence_flask5063_context_budgetfix/manifest.json
/tmp/turingos_real_task_benchmark_20260605/tdma_evidence_flask5063_context_budgetfix/per_attempt_probes.jsonl
```

Initial audit verdict: `CHALLENGE`.

Initial findings:

1. The report under-disclosed that full-flow market participation was anchored
   to `system_participation_canary_after_domain_rejection`, not to an accepted
   SWE-bench domain `WorkTx`.
2. The auditor noted an inherited, non-OBL-015 JudgeAI tactic-calibration
   concern around default forbidden-pattern coverage and Bus-adjacent oracle
   acceptance paths.

Remediation:

The benchmark report now explicitly states:

```text
domain_accepted_work_tx_id=null
domain_rejected_work_l4e_count=1
domain_manifest_work_tx_landed=false
market_anchor_source=system_participation_canary_after_domain_rejection
system_canary_work_tx_id=worktx-full-system-participation-canary-full-system-canary
```

It also states that the market receipt is real and tape-visible inside the same
full-flow run, but is not evidence that the failed SWE-bench patch became an
accepted market-backed work product. The inherited JudgeAI concern is recorded
as outside the OBL-015 benchmark path and deferred to a separate explicit
governance/JudgeAI task if reopened, because it touches restricted
Bus/kernel-adjacent authority.

Final audit confirmation:

```text
Findings on the re-audited CHALLENGE items: none remaining.

The updated report now explicitly separates the real same-run market receipt
from SWE-bench domain acceptance: it states the SWE-bench domain adapter did not
land an accepted domain WorkTx, embeds the canary anchor fields, and says the
market receipt is not evidence that the failed patch became an accepted
market-backed work product.

The inherited JudgeAI tactic-calibration issue is also now explicitly caveated
as outside the OBL-015 benchmark path and deferred to a separate restricted
Bus/kernel-adjacent governance task if reopened. I verified the new report hash
matches d719811f21d39fb9e32f4840e6738eb3e5db4e316959c10125a6105158998762;
headline/security greps and git diff --check -- report were clean.

VERDICT: PROCEED
```

Final verdict: `PROCEED`.
