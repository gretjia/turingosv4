# Codex REAL-6D Implementation Review

Scope:

- `src/runtime/agent_scheduler.rs`
- `src/bin/audit_dashboard.rs`
- `tests/constitution_g6_observe_only.rs`
- `genesis_payload.toml`
- `handover/evidence/real6_scheduler_observe_only/REAL6D_OPPORTUNITY_SCHEDULER_REPORT.md`
- Harness run `handover/evidence/dev_self_hosting/dev_1778828648899_1429709/`

Risk / FC mapping reviewed:

- Risk class: Class 4 by harness manifest.
- FC nodes/invariants: FC1 observe-only scheduler trace, FC3 dashboard/report materialized view, economy price/PnL signals must not become admission or predicate authority.

## Findings

### CHALLENGE-1: `SchedulerDecisionTrace` is dashboard-only, not emitted to a CAS/ChainTape evidence path

The audited execution plan requires REAL-6D to "Emit SchedulerDecisionTrace to CAS/ChainTape evidence path" (`handover/directives/2026-05-15_REAL5S_REAL6_REAL7_REAL8_REAL9_EXECUTION_PLAN_AUDITED.md`, REAL-6D implementation atom 1). The current implementation defines the struct in `src/runtime/agent_scheduler.rs:43` and builds it only inside `audit_dashboard --run-report` at `src/bin/audit_dashboard.rs:2686`, then immediately renders it into the dashboard string at `src/bin/audit_dashboard.rs:2696`.

`rg SchedulerDecisionTrace` shows no writer/reader, no CAS schema ID, no `CasStore::put`, and no ChainTape anchor for this trace. The struct also lacks `Serialize` / `Deserialize`, so it is not currently a persistable evidence object. Harness command_0007/0008 prove the dashboard text renders, but stdout/dashboard output is a materialized view, not the required scheduler trace evidence path.

Impact: this is not an admission-safety defect, but it is a completion defect against the audited REAL-6D plan and the tape-first rule. A replay/audit consumer cannot recover the scheduler recommendation trace as a first-class evidence object from ChainTape/CAS.

Required closure: either persist the trace as a CAS object with a stable schema and an explicit ChainTape/evidence anchor, or obtain an explicit architect/user deferral changing REAL-6D from "emit trace evidence" to "dashboard-only derived view".

### CHALLENGE-2: `head_t` in the rendered trace is `tx_count:*`, not canonical HEAD_t

`audit_dashboard` passes `format!("tx_count:{}", report.run_facts.tx_count)` as the trace `head_t` (`src/bin/audit_dashboard.rs:2686-2687`). That is a useful report label, but it is not the constitutional `HEAD_t` witness shape (`state_root`, `l4_head`, `l4e_head`, `cas_root`, `economic_state_root`, `run_id`) described by `src/state/head_t_witness.rs:49`.

Impact: if the trace is later persisted, the recommendation will not be bound to a replayable chain head. Even as a dashboard view, the field name `head_t` overstates the binding.

Required closure: use a real `HeadTWitness` / L4 head-derived value, or rename the field in the rendered materialized view to make clear it is only a `tx_count` label.

### CHALLENGE-3: Harness sidecar metadata does not match the actual touched paths

`DevTaskManifest.json` for `dev_1778828648899_1429709` lists `allowed_paths` as only `src/runtime/agent_scheduler.rs`, while command_0009 records modified `genesis_payload.toml`, `src/bin/audit_dashboard.rs`, `src/runtime/agent_scheduler.rs`, and `tests/constitution_g6_observe_only.rs`, plus the untracked REAL-6D report. The run also records commands, but I do not see `record-diff`, `validate`, or `close` events in `events.jsonl`.

Impact: command evidence is useful and mostly sufficient for gate coverage, but the self-hosting harness packet is not a clean Class-4 closeout packet. It would not give a reviewer an accurate allowed-path boundary.

Required closure: open/repair a harness run whose allowed paths cover the actual REAL-6D touched files, record the diff, then validate/close after this audit is attached.

## Positive Checks

- `SchedulerDecisionTrace` contains the architect-requested fields in `src/runtime/agent_scheduler.rs:43-52`.
- `build_observe_only_scheduler_trace` forces `observe_only: true` at `src/runtime/agent_scheduler.rs:98-108`.
- The scheduler module remains pure/read-only: I found no sequencer submission, predicate evaluation, `TypedTx`, wallet, kernel, bus, or signing-payload mutation in `src/runtime/agent_scheduler.rs`.
- `audit_dashboard` renders the recommendation as "non-binding materialized view; price is signal, not truth" and explicitly says it does not change sequencer admission or L4/L4.E predicates (`src/runtime/agent_scheduler.rs:113-115`).
- `genesis_payload.toml` pins `src/bin/audit_dashboard.rs` to `3d2186d8325c32aee1b8dedeb251ef52d59f834128db84e2371912e07f111b6a`, matching the current `sha256sum`.
- Harness command_0004 ran `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` and passed.
- Harness command coverage includes fmt, G6/6D test, G5 scheduler regression, Trust Root, constitution gates, workspace test, real dashboard render, and grep for the REAL-6D dashboard markers.
- I found no REAL-6D-local evidence of price-as-truth, ghost liquidity, f64 economy, off-tape WAL truth, raw CoT, or raw log broadcast in the audited files.

## Scope Caveat

The broader working tree is dirty and includes modified restricted surfaces such as `src/state/sequencer.rs`, `src/state/typed_tx.rs`, `src/kernel.rs`, `src/bus.rs`, `src/sdk/tools/wallet.rs`, and `src/bottom_white/cas/schema.rs`. I did not attribute those to REAL-6D because the requested audit paths were narrower, but a ship audit over the whole current diff must audit those surfaces under their own Class-4 packets.

## Verdict

CHALLENGE
