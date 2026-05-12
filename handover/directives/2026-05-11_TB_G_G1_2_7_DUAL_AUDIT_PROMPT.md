# TB-G G1.2-7 — FULL DUAL AUDIT PROMPT (Option B+ ruling Q5 cadence)

> **Audit kind**: full dual audit (Codex G2 + Gemini Pro DeepThink) per
> architect Option B+ ruling §Q5 verbatim "9-task persistent batch ship
> → Codex + Gemini full dual audit". Conservative-merge applies per
> `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS).
>
> **Audit subject**: 9-task chain-continuous batch at
> `handover/evidence/g_phase_g1_2_full_2026-05-11T22-31-30Z/` produced
> by `scripts/run_g_phase_batch.sh g_phase_g1_2_full_<TS> full`.
> Driver = `batch_evaluator` (G1.2-3) spawning `evaluator` once per
> problem with `TURINGOS_CHAINTAPE_RESUME=1` for task_k > 0.
>
> **Repo HEAD**: `b63ebeb` (post-evidence-commit; manifest git_head =
> `0e3e471`, the HEAD at run start). Clean-tree hygiene preserved.
>
> **Predecessor R2 verdict**: Codex G2 micro-audit on G1.2-6 R2 at
> `handover/audits/CODEX_G2_TB_G_G1_2_6_R2_VERDICT.md` returned
> PROCEED 9/9 PASS. This G1.2-7 audit covers the same 9 questions at
> 3x scale plus deeper inspection.
>
> **Constitutional anchors**: charter §1 G1.2-7 ship-gate row +
> `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md`
> §1 canonical + §3.1-§3.5 + §4 halt conditions + charter §0
> `kill_criteria_tested`.

---

## §1. Evidence inventory

Evidence dir: `handover/evidence/g_phase_g1_2_full_2026-05-11T22-31-30Z/`.

Expected files:
- `G_PHASE_BATCH_MANIFEST.json` — pre-batch pin; `git_head=0e3e471`,
  `problem_count=9`, `active_model=deepseek-chat`, `llm_proxy_url=
  http://localhost:8080`.
- `PROBLEMS.txt` — canonical 9-problem TB-N3 Phase 2 set:
  `mathd_algebra_107/125/141/113/114`, `mathd_numbertheory_1124`,
  `numbertheory_2pownm1prime_nprime`, `aime_1983_p1`, `aime_1984_p1`.
- `runtime_repo/` — ONE shared git repo (10 L4 transitions; 1 TaskOpen
  + 9 TerminalSummary), `agent_pubkeys.json`, `pinned_pubkeys.json`,
  `initial_q_state.json`, `genesis_report.json`.
- `cas/` — ONE shared content-addressed store (~3-4x mini object
  count).
- `BatchContinuationManifest.json` — G1.2-4 canonical `g1_2_v1` schema;
  9 task entries; continuity claim
  `tasks[k+1].start_head_t_hex == tasks[k].end_head_t_hex` for k ∈ {0..7}.
- `P000..P008/evaluator.stdout` + `.stderr` — per-task PPUT_RESULT.
- `aggregate_verdict.json` — `verdict=PROCEED passed=40 failed=0
  halted=0 skipped=11`.
- `PERSISTENCE_BINDING_REPORT.json` — `is_passing=true n_tasks=9
  n_witnessed=1` (model_identity Witnessed; 5 Empty).
- `batch_evaluator.log` — orchestrator log; 8 `ResumePreflight::Ok` +
  8 `ChainTapeLease ACQUIRED` lines.
- `run_log.txt` — canonical post-audit summary (`batch_exit=0
  audit_exit=0 audit_verdict=PROCEED persistence_passing=true
  persistence_n_witnessed=1`).

---

## §2. Audit questions (Q1..Q12 for full dual; expansion of G1.2-6 Q1..Q9)

Each auditor returns:
```
VERDICT: <PROCEED|CHALLENGE|VETO|HALT>
CONVICTION: <high|medium|low>
Q1: <PASS|CHALLENGE|VETO> [reason if not PASS]
...
Q12: ...
Notes: <free-form observations>
```

Conservative-merge: any CHALLENGE/VETO from either auditor blocks ship
until the issue is closed. PASS/PASS → G1.2-7 SHIPPED.

**Q1 (continuity at scale)**: Does the `BatchContinuationManifest.json`
carry 9 ordered entries with continuity invariant
`tasks[k+1].start_head_t_hex == tasks[k].end_head_t_hex` for every
k ∈ {0..7}? Does the live `runtime_repo` HEAD match
`tasks.last().end_head_t_hex`? At what L4 chain length?

**Q2 (one genesis at scale)**: Is there EXACTLY ONE
`genesis_report.json` in `runtime_repo/`? Confirm no per-task genesis
exists (e.g. no `P*_*/runtime_repo/genesis_report.json`). Reject if
`batch_evaluator` created a fresh runtime_repo at any `task_index > 0`.

**Q3 (audit_tape PROCEED at scale)**: Does `aggregate_verdict.json`
carry `verdict=PROCEED`? Inspect per-assertion rows for any
`Fail`/`Halted` that would block ship. Confirm `failed=0` and
`halted=0`. Skipped assertions should be the same 11 architect-named
clean-negative skips as G1.2-6 (low-activity batch + tamper-suite).

**Q4 (ResumePreflight active for all 8 boundaries)**: Verify
`batch_evaluator.log` shows
`batch_orchestrator: task_index=N ResumePreflight::Ok (head=... state_root=...
chain_length=N) → BoundaryPrep::Resume` for every N ∈ {1..8}. Reject
if any boundary fell through to `FreshGenesis` at `task_index > 0`.

**Q5 (ChainTapeLease)**: Verify `ChainTapeLease ACQUIRED` log line for
every N ∈ {1..8} with `holder_pid` matching `batch_evaluator`'s PID and
`batch_id` matching the manifest's `batch_id`. Confirm no residual
`chain_tape_lease.json` artefact post-run.

**Q6 (persistence binding)**: Verify
`PERSISTENCE_BINDING_REPORT.json` exists with `is_passing=true`,
`n_tasks=9`, and no Reset verdict. The architect §3.5 + charter §0
kill_criteria_tested #1 require absence of per-problem genesis reset
across the batch. Report which of the 6 fields are Witnessed vs Empty.

**Q7 (no per-task runtime_repo / CAS)**: List any directories matching
`P*_*/runtime_repo` and `P*_*/cas`. Should be EMPTY. The Option B+ §1
invariant is "same runtime_repo / same CAS / no memory-only cross-task
state".

**Q8 (constitutional FC2-Boot alignment)**: Confirm the batch used the
canonical `replay_full_transition` resume path
(`src/runtime/mod.rs::bootstrap_resume_state`), not a memory-only
state-passing mechanism. Reject if any subprocess spawned with
`TURINGOS_CHAINTAPE_RESUME=1` but the chain was empty.

**Q9 (kill-criteria across 9 problems)**: Audit against charter §0
`kill_criteria_tested` 1-5. The 9-task batch is permitted to be
low-activity (architect §3.5 clean-negative). It MUST NOT exhibit any:
- per-problem genesis reset (PersistenceBindingReport detects this)
- Predicate reading price / market / trace data (source-grep)
- Bankrupt-cap bypass (no agent stakes above
  `BANKRUPTCY_RISK_CAP_MICRO` — G3.2 not yet shipped so the cap is not
  enforced; check if any L4 WorkTx slipped past balance check)
- Hidden model switch (model identity stable across 9 tasks)
- Conservation flip (`assert_total_ctf_conserved` /
  `assert_no_post_init_mint` / `assert_complete_set_balanced`).

**Q10 (model identity stability at scale)**: Confirm `manifest.model =
deepseek-chat` is stable across all 9 task entries (not just the
header). Each per-task evaluator.stdout's PPUT_RESULT carries
`"model_snapshot":"deepseek-chat"` and `"model":"deepseek-chat"`.

**Q11 (architect §3.5 clean-negative classification)**: The smoke
produced 5 Empty fields (balances/positions/reputation/PnL/autopsy)
because deepseek-chat n1 did not solve any of the 9 problems within
the 200-tx budget. Confirm:
  - this is a substrate-only smoke (architect §3.5 permits) — NOT
    architecture failure
  - the 0 EscrowLock + 0 WorkTx-accepted + 0 Market activity is
    consistent with "LLM unable to solve any problem in this
    configuration", NOT a sequencer rejection bug
  - the persistence binding correctly reports Empty (which would be
    Reset if the substrate were resetting state) — `is_passing=true`
    on a non-zero batch is the architect-named clean-negative shape.

**Q12 (provenance hygiene)**: Confirm
`G_PHASE_BATCH_MANIFEST.json.git_head == 0e3e471` (the HEAD at run
start; the post-commit shipped HEAD `b63ebeb` is for the evidence
itself). Previous G1.2-6 R2 had a transient mismatch (manifest pinned
predecessor commit because R2 ran before its closure commit). G1.2-7
should be clean.

---

## §3. Halt conditions (block ship)

Per architect Option B+ ruling §4 + charter §0:
- Fresh `genesis_report.json` at `task_index > 0` (Q2 FAIL)
- `HEAD_t` discontinuity (Q1 FAIL)
- Reset verdict in PersistenceBindingReport (Q6 FAIL)
- `runtime_repo` or CAS path differs across tasks (Q7 FAIL)
- Subprocess fell back to legacy non-ChainTape path (Q4 FAIL)
- Aggregate audit_tape verdict ≠ PROCEED (Q3 FAIL)
- Hidden model switch (Q10 FAIL)
- Conservation flip (Q9 FAIL)

Any auditor returning CHALLENGE/VETO/HALT on Q1..Q12 blocks G1.2-7
ship until closed.

---

## §4. Output format

Each auditor emits in this order:

```
VERDICT: <PROCEED|CHALLENGE|VETO|HALT>
CONVICTION: <high|medium|low>
Q1: <PASS|CHALLENGE|VETO> [reason if not PASS, with file:line refs]
Q2: ...
Q12: ...
Notes: <free-form observations; flag any provenance/audit-trail gaps
       even if not blocking>
```

Then attach concrete file:line references substantiating each
verdict.

---

## §5. Cross-references

- Predecessor G1.2-6 R2 Codex verdict:
  `handover/audits/CODEX_G2_TB_G_G1_2_6_R2_VERDICT.md`
- G1.2-6 R2 full Codex transcript:
  `handover/audits/CODEX_G2_TB_G_G1_2_6_R2_MICRO_AUDIT.log`
- Option B+ ruling:
  `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md`
- Charter: `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
- CLAUDE.md §17 Report Standard + §13 Economy Laws + §14 Predicate /
  Oracle Rules + §19 No Manipulation by Sequencing
