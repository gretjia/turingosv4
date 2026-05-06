# Constitution Audit — TB-18 M1 SOLVED Tape Non-Externalization

**Filed**: 2026-05-06
**Auditor**: Claude (internal pre-audit; external dual-audit requested via companion prompt)
**Subject**: TB-18 Atom H sub-stage 2 — M1 50-problem benchmark SOLVED chains
**Verdict (provisional)**: **VETO** — see §4 for axiom-by-axiom violations
**Risk class**: 2 (production wire-up debt; not Class 4 — no sequencer admission rule change required by fix)

---

## §0 What this audit asks

Does the M1 LLM-driven pipeline honor the constitutional axioms of `constitution.md` (《反奥利奥架构的反奥利奥架构》) and the architect-ratified ChainTape granularity (1 LLM call → 1 Attempt Node, 2026-05-01)?

**Short answer**: No. The driver collapses N≥1 externalized LLM proposals into 1 ChainTape Work tx. On the most heavily-iterated SOLVED problem (P49), N=32 → M=1, a 31-fold compression of authoritative ledger state. This is the exact violation case enumerated in `feedback_chaintape_externalized_proposal` (memory file at `/home/zephryj/.claude/projects/-home-zephryj-projects-turingosv4/memory/`).

---

## §1 Anchors

| Anchor | Location | Hash / commit |
|---|---|---|
| Constitution | `constitution.md` (886 lines) | `sha256: eec6954…0e59e03` (matches verdict.json `constitution_hash_hex`) |
| Repo HEAD at audit | git working tree | `b93582e` (TB-18 session #7 handover) |
| Architect ratification of "1 LLM call = 1 Attempt Node" | `~/.claude/projects/.../memory/feedback_chaintape_externalized_proposal.md` + `handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md` | architect verdict 2026-05-01 option B′ |
| TB-18 M1 evidence root | `handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/` | run_id `n1_*_1778014705063` family |
| comprehensive_arena evidence (protocol-level proof) | `handover/evidence/tb_18_single_chain_13_of_13/r2/` | commit `15b662c..c9e0dc1` |
| Run manifest | `handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/M1_RUN_MANIFEST.json` | manifest_id `652890ec…fd57af` |

The constitution hash on the running chain (`eec6954…`) is byte-identical to the constitution.md file at HEAD. The chain is provably running under this constitution. There is no "wrong-version" defense available.

---

## §2 Empirical ground truth

### 2.1 Aggregate: 17/17 SOLVED chains have identical L4 shape

Source: each `handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/P*/verdict.json` `tx_kind_counts` field; cross-verified against `MINIF2F_M1_BENCHMARK_REPORT.md` §8 per-problem table.

| Problem | tx_count (PPUT v2, internal) | gp_node_count | failed_branch | L4 chain length | L4.E real-LLM rejections |
|---|---|---|---|---|---|
| P23 mathd_algebra_107 | 1 | 1 | 0 | **5** | 0 |
| P26 mathd_algebra_125 | 1 | 1 | 0 | **5** | 0 |
| P30 mathd_algebra_141 | 1 | 1 | 0 | **5** | 0 |
| P31 mathd_algebra_142 | 1 | 1 | 0 | **5** | 0 |
| P32 mathd_algebra_143 | 1 | 1 | 0 | **5** | 0 |
| P35 mathd_algebra_176 | 1 | 1 | 0 | **5** | 0 |
| P37 mathd_numbertheory_100 | 1 | 1 | 0 | **5** | 0 |
| P40 mathd_numbertheory_127 | 1 | 1 | 0 | **5** | 0 |
| P44 mathd_numbertheory_185 | 1 | 1 | 0 | **5** | 0 |
| P45 mathd_numbertheory_207 | 1 | 1 | 0 | **5** | 0 |
| P46 mathd_numbertheory_212 | 1 | 1 | 0 | **5** | 0 |
| P47 mathd_numbertheory_222 | 2 | 1 | 1 | **5** | 0 |
| P39 mathd_numbertheory_12 | 3 | 1 | 2 | **5** | 0 |
| P43 mathd_numbertheory_175 | 5 | 2 | 3 | **5** | 0 |
| P38 mathd_numbertheory_1124 | 16 | 10 | 6 | **5** | 0 |
| P16 amc12a_2003_p23 | 23 | 2 | 21 | **5** | 0 |
| P49 numbertheory_2pownm1prime_nprime | **32** | **19** | **31** | **5** | **0** |

`L4 chain length = 5` for every SOLVED problem regardless of internal iteration. `L4.E real-LLM rejections = 0` in every chain — every chain has exactly 2 L4.E entries, both atom3 synthetic-gate fixtures unrelated to LLM-Lean iteration.

### 2.2 P49 forensic detail (most-iterated chain)

Source: `handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/P49_numbertheory_2pownm1prime_nprime/`

```
verdict.json :
  l4_count = 5
  l4e_count = 2
  cas_object_count = 13
  constitution_hash_hex = eec6954…0e59e03
  tx_kind_counts = {work:1, verify:1, task_open:1, escrow_lock:1, finalize_reward:1,
                    challenge:0, challenge_resolve:0, terminal_summary:0,
                    market_seed:0, complete_set_mint:0, complete_set_redeem:0,
                    reuse:0, task_expire:0, task_bankruptcy:0}

evaluator.stdout PPUT_RESULT v2 (single line):
  total_run_token_count = 42937
  golden_path_token_count = 2250
  total_wall_time_ms = 369414
  verifier_wait_ms = 184460  (50% of wall-clock spent waiting on Lean compiler)
  tx_count = 32
  gp_node_count = 19
  failed_branch_count = 31
  tactic_diversity = 0.84
  tool_dist = {step:31, step_partial_ok:18, step_reject:12, parse_fail:1, omega_wtool:1}

runtime_repo/agent_audit_trail.jsonl : 2 lines, both atom3 synthetic
  - taskopen-smoke-...-atom3-seed
  - worktx-smoke-...-atom3-l4e-synthetic-rejection

runtime_repo/rejections.jsonl : 2 lines, both atom3 synthetic
  - {agent_id:"tb6-smoke-sponsor", tx_kind:"TaskOpen", public_summary:"stale_parent_root"}
  - {agent_id:"tb6-smoke-agent",   tx_kind:"Work",     public_summary:"stale_parent_root"}

cas/.turingos_cas_index.jsonl : 13 entries
  - 5 sequencer ProposalPayload (TypedTx.v1) — wrappers for the 5 L4 tx
  - 2 atom3 agent_proposal_record (synthetic-gate)
  - 2 atom3 sequencer.rejection_path ProposalPayload (synthetic L4.E)
  - 1 TransitionError.display.v1 (synthetic rejection diagnostic)
  - 1 tb7-atom3-omega-pertactic-payload (783 bytes; the FINAL composite proof)
  - 1 verification_result.v1 (Lean's check on the FINAL composite)
  - 1 proposal_telemetry.v1 (telemetry for the FINAL composite)
```

**Zero of the 13 CAS objects represent any of the 31 intermediate LLM-Lean iterations.** All 31 collisions with the Lean compiler — exactly the kind of `real-world observation data` the constitution p.21 names as the engine of `Bottleneck Theorem` evolution — were discarded at evaluator process exit.

### 2.3 Final composite proof (the 1 Work tx payload)

Source: `handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/P49_numbertheory_2pownm1prime_nprime/proofs/numbertheory_2pownm1prime_nprime_1778015075_8249a32f.lean` (lines 22–64).

The final `.lean` file is a **concatenation** of 19 successful `have` statements + the closing `by_contra` argument. Examples of the redundancy revealing iteration ghost-trace:

- Lines 22–29 and 30–37: **identical** `have h2 : 2^n - 1 > 0 := by …` definition repeated twice.
- Lines 38–42, 47–49, 50: **eight** trivial `have hN : 2^n - 1 = 2^n - 1 := rfl` statements.
- Lines 59–63: `have hMersenne_prime` defined **five times**, with shadowing.

These are the textual residue of the LLM iterating against Lean. But the iteration trajectory itself — the order, the failed attempts, the Lean stderr after each — is not on chain.

### 2.4 comprehensive_arena counter-evidence (protocol layer is fine)

Source: `handover/evidence/tb_18_single_chain_13_of_13/r2/verdict.json` + `handover/evidence/tb_18_b_phase4_2026-05-05/r2/evidence/tx_kind_distribution.json`.

A single synthetic chain produces all 13 typed-tx kinds (TaskOpen, EscrowLock, Work, Verify, Challenge, ChallengeResolve, Reuse, CompleteSetMint, CompleteSetRedeem, MarketSeed, FinalizeReward, TerminalSummary, TaskExpire/TaskBankruptcy) signed by 5 distinct agent_ids (Agent_0..Agent_4 + Agent_user_0) within a single OS process. Audit verdict `PROCEED`, replay byte-identical, tamper 3/3 detected, β-A FEASIBLE.

This proves the protocol/sequencer/CAS layer can support arbitrary multi-tx, multi-agent chains. **The defect is in the LLM-driver path, not the protocol.**

---

## §3 Code citations — where the violation lives

### 3.1 Iteration loop boundary

`experiments/minif2f_v4/src/bin/evaluator.rs:1641`

```rust
for tx in 0..max_transactions {
    // … LLM call …
    // … Lean check via internal harness …
    // tool_dist counters increment HERE, not on chain:
}
```

Tool_dist increment sites (proof that each iteration is a discrete LLM-Lean cycle, not a tactic-within-call):

| Event | Line | Meaning |
|---|---|---|
| `step` | 2805 | one LLM-call → Lean accept (intermediate) |
| `step_partial_ok` | 3236 | one LLM-call → Lean partial-accept |
| `step_reject` | 3263 | one LLM-call → Lean reject |
| `parse_fail` | 3275 | one LLM-call → output unparseable |
| `omega_wtool` | 2317, 2861 | terminal omega-accept (single per run) |

For P49: `step:31 + omega_wtool:1 = 32` discrete LLM-Lean cycles. Each is **independently** decidable as accept/reject. None of step / step_partial_ok / step_reject sites call `bus.submit_typed_tx` — they only mutate the in-memory `tool_dist` counter.

### 3.2 ChainTape submission sites

`bus.submit_typed_tx` is called 28 times in `evaluator.rs`. Of those, only the following land on the success path of a SOLVED LLM run:

- `:750` `task_open_real` — once per problem
- `:823` `escrow_lock` — once per problem
- `:2161` `real_worktx` — **once per problem** (the final composite Work)
- `:2457` `work_tx` — duplicate path for OMEGA-confirm
- `:2500` `verify_tx` — once per problem
- finalize_reward path — once per problem

**No `submit_typed_tx` call site exists inside the per-iteration `for tx in 0..max_transactions` loop body that would emit one Work tx per LLM-Lean cycle.** The 2161 submission happens AFTER the loop exits with an accepted final composite proposal.

### 3.3 Protocol layer is open

`src/state/typed_tx.rs:234, 257, 281` — `WorkTx`, `VerifyTx`, `ChallengeTx` all defined and admittable.

`src/state/sequencer.rs` — searched for self-restriction patterns (`challenger == work_author`, `verifier_agent ==`, `same agent`, `cannot.*own`, `reject.*author`): **zero matches**. The sequencer admits any signed tx satisfying parent_state_root continuity + signature validity + per-kind invariants. There is NO protocol-level barrier to one process driving Work_i, Verify_i for i=1..32 under one or multiple agent_id signing keys.

`experiments/minif2f_v4/src/bin/comprehensive_arena.rs:312, 396, 487, 948` — `drive_task_a/b/c` + `main` — production demonstration that one process emits 13/13 typed-tx kinds over multiple agent_id signers in one chain, accepted by sequencer with audit_tape PROCEED verdict.

---

## §4 Eight axiom violations

Each row: axiom (constitution location) → expected behavior → measured behavior → impact.

### A1 — Tape amputated (constitution p.8–13 Flowchart 1+2; §3.3)

- **Axiom**: `Q_t = ⟨q_t, HEAD_t, tape_t⟩` where `tape_t = everything as files`. Each `δ(input)→output` cycle goes through `wtool` to `Q_{t+1}` under `∏p` validation.
- **Expected**: Each of P49's 32 LLM-Lean cycles writes its `(q_o, a_o)` to a chain entry → tape grows by 32 nodes.
- **Measured**: tape grows by 1 node. 31 cycles vanish at evaluator process exit.
- **Code anchor**: `evaluator.rs:1641` loop body increments `tool_dist` counters but never calls `submit_typed_tx`.
- **Impact**: The implemented system is `Q_t = ⟨q_t, HEAD_t, summary(tape_t)⟩`. This is not the bootstrapped Turing machine the constitution names; it is a final-state snapshot.

### A2 — Boolean predicate doesn't reach durable record (constitution p.2 布尔信号; p.4 屏蔽错误)

- **Axiom**: `∏p` is a hard predicate; every white-box adjudication (especially rejections) must persist as feedback signal.
- **Expected**: 12 `step_reject` events on P49 → 12 L4.E rejection entries with Lean stderr CIDs.
- **Measured**: P49 `rejections.jsonl` has 2 entries, both atom3-synthetic, neither LLM-related. 12 real Lean-rejection records exist only as a counter `tool_dist["step_reject"] = 12` in evaluator stdout.
- **Code anchor**: `evaluator.rs:3263` increments `tool_dist["step_reject"]` but does not append to L4.E.
- **Impact**: Any silent loophole in the in-process Lean wrapper (e.g. accepting a `sorry` placeholder as `step_partial_ok`) cannot be discovered post-hoc by `audit_tape`. Boolean signal degrades to soft.

### A3 — Goodhart shielding broken at the wrong layer (constitution p.5 屏蔽Goodhart)

- **Axiom**: Pricing operates on observable result; metric internals hidden from middle-black.
- **Expected**: External pricing oracles (audit_tape verdict, replay byte-cmp, tamper detection) can distinguish `1 LLM call → solve` (genuine δ) from `32 LLM calls → solve` (compute brute-force).
- **Measured**: Both produce identical 5-tx L4 chains, identical `tx_kind_counts`, identical audit_tape `PROCEED` verdicts. The two trajectories are indistinguishable on chain.
- **Data anchor**: §2.1 table — P23 (1 iteration, `nlinarith`) and P49 (32 iterations, 19-step composite) both have `L4 chain length = 5`.
- **Impact**: When PPUT-CCL or any future market reward is keyed on chain-state observables, the optimizer can pump `tx_count` without bound and pricing cannot detect it. This is `pricing → valuation` regression — exactly what 前置《群体智慧》"标价 vs 估值" rejects.

### A4 — PCP / weak-supervises-strong broken (constitution p.2 PCP 谓词; 《验证非对称性》T5)

- **Axiom**: Errors cannot hide long-term under random local audit; PCP guarantees verifier can sample any position.
- **Expected**: audit_tape's random-position sample space includes every reasoning step.
- **Measured**: audit_tape's positions are restricted to {task_open, escrow_lock, work, verify, finalize_reward} × {parent_state_root, signature, payload_cid}. None of the 35 assertions reach mathematical content (the `have` statements inside the proof file). Tamper-DEGRADED rate of 12/50 (24%) on M1 (per `MINIF2F_M1_BENCHMARK_REPORT.md` §5.1) is direct symptom: `flip_cas_byte` on the final-composite payload often hits unwalked-by-assertions territory.
- **Code anchor**: `src/runtime/audit_assertions.rs` — assertion set scope. `tamper_report.json` for any P49-class chain shows the largest CAS object frequently lies outside the assertion graph.
- **Impact**: Position space collapses from "all reasoning steps" to "5 ceremonial gates". T5 problem is downgraded to T1 (form-compliance only). Per 前置《验证非对称性》定理: under T1, `S` (solver) holds structural advantage; verifier loses `weak-supervises-strong` capability.

### A5 — Markov rule cannot fire (constitution p.18 马科夫规则)

- **Axiom**: V_{t+1} reads V_t's final error log + current constitution. Logs must be white-box adjudication output, not black-box self-report.
- **Expected**: ArchitectAI scanning L4.E across the M1 batch reads ~hundreds of `LeanFailed` rejections (failed branches across 50 problems × ~10 average iterations) and learns "system collides on X-class problems".
- **Measured**: L4.E across the entire 50-problem M1 batch contains 100 entries, all atom3-synthetic (2 per chain × 50 chains). Zero real LLM-vs-Lean collision records. `evaluator.stdout` PPUT v2 JSON contains aggregate counters but is per-process ephemeral self-report (not white-box adjudication).
- **Data anchor**: §2.2 P49 `rejections.jsonl` — both entries are atom3-fixtures.
- **Impact**: Markov bridge is severed at the source. ArchitectAI has no real-world feedback to mine.

### A6 — Bottleneck Theorem engineering fails (constitution p.21)

- **Axiom**: Evolution requires real-world observation data — compiler errors, API responses, sandbox crashes — recorded in logs.
- **Expected**: P49's 13 Lean-rejection events (12 step_reject + 1 parse_fail) are textbook real-world observation data and persist as L4.E + CAS-stored stderr.
- **Measured**: 13 collisions occurred, 0 persisted. The fuel that drives meta-architecture evolution (constitution p.21 "real-world feedback as a slap or a candy") is consumed at the driver layer.
- **Impact**: TuringOS's evolution engine (the meta-architecture loop, constitution p.16 figure 3) is operating on synthetic noise, not real data.

### A7 — Everything-is-a-File violated (《图灵机哲学》整篇)

- **Axiom**: Tape = files; head = path; state register = q. Replay can stop at any intermediate `(q_t, HEAD_t, tape_t)`.
- **Expected**: Replay of P49 chain at logical_t=7 reproduces the LLM's prompt, Lean's stderr, and the agent's chosen next action at iteration 7.
- **Measured**: Replay reproduces only the FINAL state. Intermediate states do not exist as files. The "head" position (logical_t) only takes 5 distinct values across the entire run.
- **Data anchor**: §2.2 — `cas_object_count = 13`, of which 5 are L4-tx wrappers and only 1 contains proof content.
- **Impact**: The replay-deterministic guarantee of `audit_tape` covers the L4 ceremony only; the actual reasoning trajectory is non-replayable.

### A8 — ArchitectAI / JudgeAI input pipeline broken (constitution p.14–16 三权分立)

- **Axiom**: ArchitectAI proposes new predicates by reading logs; logs are white-box adjudication output not black-box opinion.
- **Expected**: `handover/evidence/.../P*/runtime_repo/agent_audit_trail.jsonl` contains hash-chained per-iteration audit records that ArchitectAI/JudgeAI can read.
- **Measured**: `agent_audit_trail.jsonl` has 2 lines per chain, both atom3-synthetic. The actual iteration record exists only as `tool_dist` aggregate in `evaluator.stdout` — which is a black-box self-report (the evaluator decides what to count and how, with no external white-box check).
- **Impact**: 三权分立 collapses to two: human-architect + JudgeAI-on-evaluator-self-report. ArchitectAI has no white-box log to consume; if it tries, it is consuming valuation (黑盒意见) not pricing (白盒裁决).

### A9 — Architect-ratified granularity already violated

- **Anchor**: `feedback_chaintape_externalized_proposal` (memory file, 2026-05-01 architect verdict B′):
  > "When evaluating 'is the chain too short?', check whether the runtime emitted N externalized proposals but only M (M<N) entered ChainTape — **that IS a violation**."
  > "1 LLM call producing 1 compound payload = 1 Attempt Node"
- **Measured**: P49 had 32 distinct LLM calls (each a discrete `tool_dist` step event with its own LLM token cost — `total_run_token_count = 42937`, ~1300 tokens per call average, far exceeding any single-call budget). Only 1 entered ChainTape.
- **Impact**: This is **not a constitutional change request**. The fix is the implementation of an already-ratified invariant. No re-ratification needed for the substrate; only Class 2 production wire-up.

---

## §5 Root cause classification

The defect is **driver-layer**, not protocol-layer:

| Layer | Status | Evidence |
|---|---|---|
| Sequencer admission | OK | comprehensive_arena 13/13 chain accepted; no self-restriction grep hits in `src/state/sequencer.rs` |
| Typed tx schema | OK | Work/Verify/Challenge/etc. all defined in `src/state/typed_tx.rs:234+` |
| CAS / state_root chaining | OK | `verdict.json` byte-identical replay on all 50 chains |
| audit_tape assertions | OK on its own scope; **mis-scoped** for what should be on chain | 35 assertions all in 5-gate L4 ceremonial scope |
| **evaluator.rs swarm-loop driver** | **BROKEN** | Lines 1641 + 2805/3236/3263/3275 increment counters; no `submit_typed_tx` per cycle |

The sequencer would happily accept 32 Work tx for one solved problem. The driver does not emit them.

---

## §6 Proposed fix — ChainTape-Native Externalized δ (CN-δ)

### 6.1 Single axiom

**Every iteration of the LLM-driven loop emits one externalized proposal as a chain entry.** Either as L4 (when accepted by the immediate-Lean predicate) or L4.E (when rejected). The "1 LLM call = 1 Attempt Node" invariant becomes load-bearing rather than aspirational.

### 6.2 Implementation skeleton

`evaluator.rs::run_swarm` inner loop replacement (sketch):

```rust
let mut parent_state_root = bus.head_state_root().await;

for cycle in 0..max_transactions {
    // 1. δ: middle-black LLM call
    let prompt = build_prompt(parent_state_root, &tape_view);
    let llm_output = call_llm(&prompt).await?;
    let proposal_artifact_cid = bus.cas_put(/*ProposalArtifact*/ &llm_output).await?;

    // 2. wtool: emit externalized proposal as chain entry
    let work_tx = make_real_worktx_signed_by(
        agent_id_solver,
        parent_state_root,
        proposal_artifact_cid,
        cycle as u64,
    )?;
    let work_tx_id = bus.submit_typed_tx(work_tx).await?;  // <- THE FIX

    // 3. ∏p: bottom-white predicate — Lean compiler check
    let lean_result = run_lean_check(&llm_output).await?;

    match lean_result {
        LeanVerdict::Accept => {
            // 3a. emit VerifyTx; advance parent_state_root
            let verify_tx = make_real_verifytx_signed_by(agent_id_verifier, work_tx_id, Confirm);
            bus.submit_typed_tx(verify_tx).await?;
            parent_state_root = bus.head_state_root().await;
            if lean_result.is_omega { /* finalize_reward path */ break; }
        }
        LeanVerdict::Reject(stderr) => {
            // 3b. enter L4.E rejection ledger; parent_state_root unchanged
            // (the sequencer already routes failed admissions to L4.E; this requires
            //  emitting a syntactically-admittable WorkTx that fails a content predicate.
            //  The exact mechanism — submit-then-rejected vs explicit reject path — is
            //  the architect-clarification gate listed in §9.)
            let stderr_cid = bus.cas_put(/*LeanStderr*/ &stderr).await?;
            // Either:
            //   (i) the WorkTx above was already L4.E'd by sequencer if Work admission
            //       requires a content-predicate (would need new admission rule),
            //   (ii) OR: emit a follow-up VerifyTx{verdict=Disconfirm} which the
            //        sequencer routes to L4.E with raw_diagnostic_cid=stderr_cid.
            //  Path (ii) is preferred — it does NOT change sequencer admission;
            //  it uses the existing VerifyTx Disconfirm path. Class 2 not Class 4.
        }
    }
}
```

### 6.3 Concrete projected effect on P49

| Metric | Current | CN-δ projected |
|---|---|---|
| L4 entries | 5 | ~38 (32 Work + 19 Verify-Confirm, partial overlap; + 5 admin) |
| L4.E entries | 2 (synthetic only) | 2 + ~13 (LeanFailed real) |
| CAS objects | 13 | ~70 (32 proposals + 13 stderr + verify_results + admin) |
| audit_tape assertion positions | 5 ceremonial | ~80 (5×N reasoning step positions) |
| ArchitectAI-readable real-world collisions | 0 | 13 |
| Markov-readable V_t error log | empty | 13-entry L4.E |
| PCP audit position space | T1 | T5 |

### 6.4 Critically: this is NOT per-tactic decomposition (TB-8+ scope)

The CN-δ unit is **one LLM call → one chain entry**, exactly the granularity ratified 2026-05-01. If a single LLM call returns a `calc` block with 5 internal tactics, it remains 1 chain entry (1 Attempt Node). Per-tactic decomposition (each tactic as its own external tool call) remains TB-8+ scope.

The CN-δ fix is **only** the externalization of the existing per-LLM-call boundary, which the architect already ratified. The driver currently violates that boundary by collapsing N>1 calls into 1 entry. CN-δ restores the ratified invariant.

---

## §7 Cost / risk / class

### 7.1 Throughput impact

P49 wall-clock: 369 s. New per-cycle sequencer admit cost: ~10 ms × 32 = +0.32 s (0.09% overhead). Within noise.

### 7.2 Storage impact

Per problem: ~50 KB additional CAS (32 proposal_artifacts + 13 stderr + verify_results). 50-problem M1 batch: ~2.5 MB. Negligible.

### 7.3 Audit complexity impact

`audit_tape` assertion count: O(35) → O(35 + 5N). For P49 (N=32): ~195 assertions. Audit wall-clock per chain: 150 ms → ~600 ms. Acceptable.

### 7.4 Replay determinism

Risk: increased exposure to LLM non-determinism (DeepSeek drift per `project_deepseek_drift_2026-04-24` memory). Mitigation: each LLM call's proposal_artifact has its own CID; replay can byte-cmp per-cycle and quarantine drifted cycles without invalidating the whole chain. CN-δ makes drift **observable** rather than hiding it.

### 7.5 Class

**Class 2 production wire-up**:
- No typed-tx schema change (Work/Verify already exist with current shape)
- No sequencer admission rule change (preferred path uses existing VerifyTx Disconfirm → L4.E route)
- No canonical signing payload change
- No constitution.md edit

**Not Class 4**: by `feedback_class4_cannot_hide_in_class3` carve-out — sequencer admission / typed-tx schema bumps / canonical-signing-payload changes would trigger Class 4. CN-δ avoids all three.

**Not Class 3 either** (per `feedback_risk_class_audit`): no auth-crypto-money change. Solver agent and verifier agent under existing keypair registry. No new payment flow.

### 7.6 Dual-audit requirement

Per `feedback_dual_audit` + Class 2: full Codex + Gemini external audit before TB-19 ship. This audit (filed 2026-05-06) is the request seed; companion `AUDITOR_PROMPT_TB18_M1_TAPE_NON_EXTERNALIZATION_2026-05-06.md` is the structured prompt for both auditors.

---

## §8 Forward bind

| Step | Owner | Gate |
|---|---|---|
| External Codex audit on this report | user-invoked | VETO/CHALLENGE/PASS verdict on §4 axiom claims |
| External Gemini audit on this report | user-invoked | VETO/CHALLENGE/PASS verdict on §4 axiom claims |
| Architect § sign-off on CN-δ scope | user-conveyed | "yes proceed to TB-19 with CN-δ as main thrust" or modification |
| TB-19 charter | autonomous (per `feedback_tb_phase_tag_required`) | phase_id=P0/P1 internal-debt; roadmap_exit_criteria=A9 closure; kill_criteria=audit_tape green on first CN-δ chain |
| TB-19 24h-feedback gate | per `feedback_iteration_cap_24h` | first CN-δ chain on `mathd_algebra_107` (trivial 1-cycle case) within 24h; on `mathd_numbertheory_1124` (10-cycle) within 72h |

---

## §9 Open questions for auditors / architect

1. **L4.E routing path**: is the preferred CN-δ rejection-recording path (ii) — VerifyTx{verdict=Disconfirm} → sequencer routes to L4.E with raw_diagnostic_cid — actually wired in current `src/state/sequencer.rs`, or does it require a new admission predicate? If the latter, the fix may slip to Class 4 per the carve-out.
2. **Agent_id assignment**: does Verifier role require a distinct keypair (Agent_1) from Solver (Agent_0), or is solo-agent self-verify admissible? Constitution p.4 屏蔽相关性 suggests separate identities; existing `agent_keypairs.rs` registry supports both.
3. **Constitution amendment necessity**: is any axiom of CN-δ in tension with `feedback_chaintape_externalized_proposal`'s "compound payload = 1 Attempt Node" wording? Internal read: no — CN-δ keeps that invariant; it only forbids collapsing N>1 LLM calls into 1 Attempt Node.
4. **Backwards compatibility**: M1 evidence stays as is (per `feedback_no_retroactive_evidence_rewrite`). README grandfathering note added: "L0 one-shot witness (TB-18 driver scope); CN-δ from TB-19+". Confirm acceptable.

---

## §10 Companion artifacts

- Auditor prompt: `handover/audits/AUDITOR_PROMPT_TB18_M1_TAPE_NON_EXTERNALIZATION_2026-05-06.md`
- Tape evidence: `handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/`
- Protocol-layer counter-proof: `handover/evidence/tb_18_single_chain_13_of_13/r2/`
- Constitution: `constitution.md` (sha256 in §1)
- Architect ratification memory: `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_chaintape_externalized_proposal.md`

---

**End of internal pre-audit. External dual-audit verdicts pending.**
