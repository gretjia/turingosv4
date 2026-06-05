# Constitutional Full-Flow Benchmark Plan

Date: 2026-06-05

Active obligation: OBL-015.

Goal: use the real `turingos` CLI to run a Constitutional Full-Flow Benchmark
that lights FC1/FC2/FC3 node-level runtime receipts, includes market
participation in the same task flow, runs SWE-bench or an equivalent real task
to a task verdict, and exports an auditable benchmark packet. Do not open a PR
until the packet itself passes.

## Contracts

- Entry point: `turingos benchmark full-flow run`.
- Kernel boundary: Lean/theorem proving is a workload or verifier layer, not a
  kernel dependency. Kernel receipts stay close to ChainTape, CAS, replay,
  predicates, refs, capabilities, scheduling, and governance.
- Fact source: receipts derive from L4, L4.E, CAS, replay reports, genesis, and
  trust-root manifests. `TRACE_FLOWCHART_MATRIX.md` is only a node spec, not a
  fact source.
- Verdicts are separated:
  - `FLOW-PASS`: FC1/FC2/FC3 required runtime receipts are present.
  - `SYSTEM-PASS`: replay, CAS retrieval, signatures, source/worktree, and
    packet consistency checks pass.
  - `TASK-PASS`: the real task verifier passes. Current-kernel SWE-bench
    structural smoke must not be promoted to `TASK-PASS`.
- PR gate: no PR before `constitutional_full_flow_benchmark_packet.json` passes
  its own audit checks, then clean-context audit and OBL witness pass.

## Execution Atoms

### Atom B1: CLI Contract

Implement `turingos benchmark full-flow run`.

Required args:

```bash
turingos benchmark full-flow run \
  --run-dir <PATH> \
  --run-id <ID> \
  --constitution constitution.md \
  --sample-json <SWE_SAMPLE_JSON> \
  --llm-proxy-url <URL> \
  --model <MODEL>
```

Optional task-verifier args:

```bash
  --task-mode swebench-tdma \
  --workspace <PATH> \
  --role <meta|blackbox> \
  --swebench-python <PATH> \
  --swebench-dataset <NAME> \
  --swebench-workdir <PATH> \
  --require-task-pass
```

Ship gate:

- `turingos benchmark full-flow --help` lists FC1/FC2/FC3, market, `FLOW-PASS`,
  `SYSTEM-PASS`, and `TASK-PASS`.
- The command writes `constitutional_full_flow_benchmark_packet.json`.
- In smoke mode, packet may pass FLOW/SYSTEM but must not claim `TASK-PASS`.

### Atom B2: FC Node Runtime Receipt Report

Extend `full_system_participation_current_kernel` with
`flowchart_node_receipts`.

Required structure:

```json
{
  "flowchart_node_receipts": {
    "verdict": "FLOW-PASS",
    "missing": [],
    "fc1": [{"node_id": "FC1-N1", "status": "present", "receipt_kind": "...", "evidence": ["..."]}],
    "fc2": [],
    "fc3": []
  }
}
```

Ship gate:

- Receipts derive from replayed L4/L4.E/CAS/genesis/trust-root paths.
- FC1-N15 requires a real L4.E rejection receipt.
- FC2 document nodes may be marked `document-pinned` only when constitution or
  trust-root hash evidence is present; they are recorded, not silently omitted.
- FC3 Veto-AI receipt must show the runtime verdict domain is only
  `{PASS,VETO}`.

### Atom B3: Market + Rejection Participation

Extend the full-system augmentation path so the same run contains:

- an accepted agent market action;
- a `MarketDecisionTrace` for the accepted action;
- a controlled failed-invest `BuyWithCoinRouterTx` routed to L4.E.

Ship gate:

- replay `l4e_entries > 0`;
- `flowchart_node_receipts.fc1` contains `FC1-N15` with `status=present`;
- market participation remains `present=true`;
- no schema, typed-tx discriminant, or sequencer admission change.

### Atom B4: Benchmark Packet

The packet must include:

- schema/version/run id;
- source git head and worktree status summary;
- entrypoint and command log;
- paths for runtime repo, CAS, replay, participation report, task evidence;
- `FLOW-PASS`/`SYSTEM-PASS`/task verdict;
- explicit caveat if the run is only structural SWE-bench smoke;
- no `PROVEN`, causal, or `TASK-PASS` headline without real verifier pass.

Ship gate:

- smoke packet passes tests but task verdict is not `TASK-PASS`;
- `--require-task-pass` fails closed unless task verifier passes;
- final packet can be copied to an external auditor without local chat context.

### Atom B5: Real Task Benchmark

Preferred real task:

```bash
turingos tdma run \
  --judge swebench \
  --role meta \
  --swebench-sample <sample.json> \
  --swebench-python /Users/zephryj/.venv-swebench/bin/python \
  --swebench-workdir <run>/swebench_work \
  --max-attempts-per-stage 3 \
  --evidence-dir <run>/tdma_evidence
```

If SWE-bench dependencies are unavailable, use an equivalent real task only if
it has a true verifier independent of the model answer. Otherwise emit a
blocked packet and do not mark OBL-015 complete.

## Verification

Development gates:

```bash
/Users/zephryj/.cargo/bin/cargo test --test cli_benchmark_full_flow --no-fail-fast
/Users/zephryj/.cargo/bin/cargo test --test constitution_true_suite_swebench_runner --no-fail-fast
```

Broad gates before ship:

```bash
/Users/zephryj/.cargo/bin/cargo test --workspace --no-fail-fast
bash scripts/run_constitution_gates.sh
/Users/zephryj/.cargo/bin/cargo test --test constitution_matrix_drift --no-fail-fast
```

Structural scans:

```bash
git diff --name-only origin/main...HEAD | grep -E 'src/(kernel|bus|state/sequencer|state/typed_tx|sdk/tools/wallet|bottom_white/cas/schema)\.rs'
grep -R -nE 'PROVEN|DEFINITIVE|causal|isolated lever|X > Y' handover src tests
grep -R -nE 'raw.*stderr|Lean.*stderr|api[_-]?key|Authorization|Bearer' handover src tests
```
