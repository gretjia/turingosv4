# TB-G G1.2-6 — Codex Micro-Audit Prompt (Option B+ ruling Q5 cadence)

> **Audit kind**: micro-audit (single auditor; Codex G2 only) per architect
> Option B+ ruling §Q5 verbatim "3-task mini-smoke + ResumePreflight ship
> → Codex micro-audit". Full Codex + Gemini Pro dual audit is reserved
> for G1.2-7 (9-task batch).
>
> **Audit subject**: 3-task chain-continuous batch evidence
> dir produced by `scripts/run_g_phase_batch.sh g_phase_g1_2_mini_<TS>
> mini`. Driver = `batch_evaluator` (G1.2-3 binary; spawns the existing
> `evaluator` subprocess once per problem).
>
> **Constitutional anchor**: charter §1 G1.2-6 ship-gate row +
> `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md` §1
> canonical + §3.1 ResumePreflight + §3.2 ChainTapeLease + §3.3
> BatchContinuationManifest + §3.4 persistence evidence + §3.5 halt-
> and-record.

---

## §1. Evidence inventory

Evidence dir: `handover/evidence/<RUN_TAG>/` where `<RUN_TAG>` =
`g_phase_g1_2_mini_<TS>`.

Expected files:
- `G_PHASE_BATCH_MANIFEST.json` — pinned BEFORE batch launch; carries
  `batch_id`, `problem_count=3`, `shared_runtime_repo`, `shared_cas`,
  `active_model`, `llm_proxy_url`, `git_head`, `resume_semantic`.
- `PROBLEMS.txt` — first 3 of TB-N3 Phase 2 problem set:
  `mathd_algebra_107`, `mathd_algebra_125`, `mathd_algebra_141`.
- `runtime_repo/` — ONE shared git repo. Contains `agent_pubkeys.json`,
  `pinned_pubkeys.json`, `initial_q_state.json`, `genesis_report.json`,
  `refs/transitions/main` head pointing at the cumulative chain.
- `cas/` — ONE shared content-addressed store; objects accumulated across
  all 3 tasks.
- `BatchContinuationManifest.json` — G1.2-4 fact-identity. 3 task entries
  in commit order; `tasks[k+1].start_head_t_hex == tasks[k].end_head_t_hex`
  for k ∈ {0, 1}.
- `P00_<problem>/`, `P01_<problem>/`, `P02_<problem>/` — per-task
  `evaluator.stdout` + `evaluator.stderr`.
- `aggregate_verdict.json` — post-batch `audit_tape` over shared
  `runtime_repo + cas`; expected verdict = `PROCEED`.
- `batch_evaluator.log` — orchestrator-side log (boundary preps, lease
  acquires, subprocess exit codes, continuity check).
- `audit_tape.stderr` — audit_tape stderr stream.
- `run_log.txt` — one-line summary.

---

## §2. Audit questions (Codex G2)

Audit conviction levels: high / medium / low. Recommendation:
PROCEED / CHALLENGE / VETO / HALT. Per `feedback_dual_audit` Codex is the
"rigor finder"; flag any unbound assertion / silent fallback / replay-
determinism gap with high conviction.

**Q1 (continuity)**: Does the `BatchContinuationManifest.json` carry 3
ordered entries with `tasks[k+1].start_head_t_hex == tasks[k].end_head_t_hex`
for k ∈ {0, 1}? Does the live `runtime_repo` HEAD match
`tasks.last().end_head_t_hex`? Reject if any continuity claim is
synthesised (e.g. orchestrator copies the prior `end_head` into
`start_head` without re-reading the live ref).

**Q2 (one genesis)**: Is there exactly ONE `genesis_report.json` in
`runtime_repo/`? Confirm no per-task genesis exists (e.g. no
`P01_*/runtime_repo/genesis_report.json`). Reject if `batch_evaluator`
created a fresh runtime_repo at task_index > 0.

**Q3 (audit_tape PROCEED)**: Does `aggregate_verdict.json` carry
`"verdict": "PROCEED"`? Inspect the per-assertion rows for any
`Fail` / `Skipped` that would normally block ship. Reject if PROCEED is
asserted but supported by `audit_proceed=false` or any assertion blocks.

**Q4 (ResumePreflight active)**: Verify `batch_evaluator.log` shows
`prepare_task_boundary` invoked for task_1 and task_2 with
`BoundaryPrep::Resume`; confirm preflight verdict was Ok before each
subprocess spawn. Reject if any boundary fell through to `FreshGenesis`
at task_index > 0.

**Q5 (ChainTapeLease)**: Did the orchestrator acquire a lease before each
resume? Inspect the runtime_repo for `chain_tape_lease.json` artefacts
during the run (likely absent post-shutdown — confirm release on guard
drop). Optional: verify the lease holder_pid recorded in any
intermediate state matches the batch_evaluator PID.

**Q6 (persistence binding)**: For the 3-task mini smoke, the persistence
binding (G1.2-5 `bind_persistence`) is expected to report Witnessed for
`model_identity` and potentially Empty for the other 5 fields
(low-activity batch). Confirm no `Reset` verdict is reachable. If the
smoke produced real economic activity (e.g. an EscrowLock accepted by
the evaluator), confirm balances delta is non-zero.

**Q7 (no per-task runtime_repo)**: List any directories matching
`P*_*/runtime_repo` and `P*_*/cas`. Should be EMPTY. The Option B+ §1
invariant is "same runtime_repo / same CAS across all subprocesses".

**Q8 (constitutional alignment)**: Confirm the batch used the canonical
FC2-Boot `replay_full_transition` resume path (via
`build_chaintape_sequencer_with_initial_q` + `bootstrap_resume_state`),
not a memory-only state-passing mechanism. Reject if any subprocess
spawned with `TURINGOS_CHAINTAPE_RESUME=1` but the chain was empty.

**Q9 (constitutional kill-criteria)**: Audit against charter §0
`kill_criteria_tested` 1-5. The mini smoke is permitted to be
low-activity (architect §3.5 clean-negative). It MUST NOT exhibit any:
- per-problem genesis reset (balances reset, positions cleared,
  reputation zeroed between problems)
- Predicate reading price / market / trace data
- Bankrupt-cap bypass (no agent stakes above
  `BANKRUPTCY_RISK_CAP_MICRO`)
- Hidden model switch (model identity stable across tasks)
- Conservation flip (`assert_total_ctf_conserved` /
  `assert_no_post_init_mint` / `assert_complete_set_balanced`)

---

## §3. Output format

Codex G2 should emit (in this order):

```
VERDICT: <PROCEED|CHALLENGE|VETO|HALT>
CONVICTION: <high|medium|low>
Q1: <PASS|CHALLENGE|VETO> [reason if not PASS]
Q2: ...
Q9: ...
Notes: <free-form observations>
```

Then attach any concrete file:line references (e.g.
`BatchContinuationManifest.json:5 task[0].end_head_t_hex=...`) that
substantiate each verdict.

---

## §4. Halt conditions

Per architect Option B+ ruling §4 + charter §0 kill_criteria_tested,
any of the following blocks G1.2-7 scale-up:

- fresh `genesis_report.json` at `task_index > 0`
- `runtime_repo` or CAS path differs across tasks
- `HEAD_t` discontinuity between consecutive tasks
- agent balance / positions reset silently
- subprocess fell back to legacy non-ChainTape path
- aggregate audit_tape verdict ≠ PROCEED

If Codex returns CHALLENGE/VETO/HALT on any of Q1..Q9, halt scale-up to
G1.2-7. Address Codex's CHALLENGE before re-running.
