# Auditor Prompt — Constitution Audit of TB-18 M1 Tape Non-Externalization

**Filed**: 2026-05-06
**Solicits**: external dual audit (Codex + Gemini, model-agnostic)
**Verdict format**: VETO / CHALLENGE / PASS, axiom-by-axiom, with citation to specific code line or evidence file
**Engagement scope**: ~3-4 hours of work; full read of one audit report + 6 evidence files + 4 source files

---

## 1. Mission

You are an external auditor for the TuringOS v4 project. The internal architect has filed a **VETO**-class audit claiming that the TB-18 M1 50-problem benchmark violates 9 axioms of `constitution.md` and the architect-ratified ChainTape granularity from 2026-05-01.

Your job is to **independently verify or refute** each of the 9 claims by reading the actual code and the actual on-disk evidence — **not by reading or trusting the audit report's interpretation**. Your verdict carries Class-2 dual-audit weight per the project's `feedback_dual_audit` policy.

Critically, the auditee (the internal Claude that wrote the audit report) was **wrong twice in earlier conversation turns** about the same data — first claiming "5-tx linear chain implies no market mechanism", then claiming "P49 has 19 externalized CoT steps on chaintape", before reaching the current claim that "0/17 SOLVED chains externalize iteration to ChainTape". Treat the report as a hypothesis, not as ground truth. Verify by reading evidence directly.

---

## 2. Repository + commit anchor

```
repo path:    /home/zephryj/projects/turingosv4
git HEAD:     b93582e   (verify with: git -C /home/zephryj/projects/turingosv4 rev-parse --short HEAD)
git branch:   main
constitution sha256: eec695459c71fbef3685583485deb431fe3b561657b2f285b7c5e7e220e59e03
                     (verify with: sha256sum /home/zephryj/projects/turingosv4/constitution.md)
```

If your `git rev-parse` does not return `b93582e`, stop and request a fresh evidence bundle — this audit assumes that exact commit.

---

## 3. Files you must read

### 3.1 Primary audit report (the document under audit)

```
/home/zephryj/projects/turingosv4/handover/audits/CONSTITUTION_AUDIT_TB18_M1_TAPE_NON_EXTERNALIZATION_2026-05-06.md
```

This is the artifact whose claims you are verifying. Read fully before proceeding.

### 3.2 Tape evidence (50-problem M1 batch)

```
/home/zephryj/projects/turingosv4/handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/
```

Subdirectories of interest:

| Path | What it contains |
|---|---|
| `MINIF2F_M1_BENCHMARK_REPORT.md` | Aggregate report; §8 has per-problem L4 chain shape |
| `M1_RUN_MANIFEST.json` | Frozen manifest (model, budget, problem set sha256, commit) |
| `EVIDENCE_INDEX.json` | Per-problem hash + outcome catalog |
| `P23_mathd_algebra_107/` | **Trivial one-shot SOLVED**: tx_count=1, gp_node_count=1 |
| `P38_mathd_numbertheory_1124/` | **Multi-iteration SOLVED**: tx_count=16, gp_node_count=10 |
| `P49_numbertheory_2pownm1prime_nprime/` | **Most-iterated SOLVED**: tx_count=32, gp_node_count=19, failed_branch_count=31 |

For each `P*` problem dir, the structure is:

```
P*/verdict.json                              # audit_tape verdict + tape_root + tx_kind_counts
P*/verdict_replay.json                       # byte-identical replay verdict
P*/tamper_report.json                        # tamper-detection results
P*/evaluator.stdout                          # PPUT_RESULT v2 single-line JSON (key audit data)
P*/evaluator.stderr                          # evaluator log
P*/h_vppu_history.json                       # H-VPPU progression
P*/proofs/*.lean                             # final composite proof (if SOLVED)
P*/runtime_repo/agent_audit_trail.jsonl      # per-tx audit trail (currently 2 lines per chain; verify)
P*/runtime_repo/rejections.jsonl             # L4.E rejection records (currently 2 lines per chain; verify)
P*/runtime_repo/genesis_report.json          # chain genesis report
P*/runtime_repo/agent_pubkeys.json           # agent_id → pubkey registry
P*/runtime_repo/initial_q_state.json         # initial Q state at chain start
P*/cas/.turingos_cas_index.jsonl             # CAS object catalog
P*/cas/_dotgit_post_tar/objects/             # zlib-compressed CAS objects (git-style)
```

### 3.3 Counter-evidence: protocol-level proof

```
/home/zephryj/projects/turingosv4/handover/evidence/tb_18_single_chain_13_of_13/r2/
```

A synthetic single-chain that produces all 13 typed-tx kinds via 5 distinct agent_ids in one process. Used by the audit report to argue the protocol layer is not the bottleneck.

### 3.4 Source code under audit

| File | Lines to inspect | Why |
|---|---|---|
| `experiments/minif2f_v4/src/bin/evaluator.rs` | 1641 (loop boundary), 2805/2861/3236/3263/3275 (tool_dist increment), 750/823/2161/2457/2500 (submit_typed_tx call sites) | Driver-layer iteration loop |
| `experiments/minif2f_v4/src/bin/comprehensive_arena.rs` | 312/396/487/948 (drive_task_a/b/c + main) | Counter-proof that protocol supports multi-tx-per-chain |
| `src/state/typed_tx.rs` | 234 (WorkTx), 257 (VerifyTx), 281 (ChallengeTx), 626 (CompleteSet/MarketSeed comment), 851 (TB-13 region) | Typed-tx schema (verify Work/Verify/Challenge exist) |
| `src/state/sequencer.rs` | grep for self-restriction patterns | Verify no protocol-level barrier to multi-tx-per-iteration |

### 3.5 Constitution + architect ratification anchors

```
/home/zephryj/projects/turingosv4/constitution.md                 (886 lines; sha256 above)
/home/zephryj/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_chaintape_externalized_proposal.md
/home/zephryj/projects/turingosv4/handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md   (referenced; verify exists)
```

The memory file contains the architect-ratified granularity rule from 2026-05-01 (option B′). It is the load-bearing reference for axiom A9 in the audit report.

---

## 4. Specific verifications requested

For each verification: state your method (which file you read, which line/field), state the result, then state your verdict (verified / refuted / inconclusive).

### V1 — Aggregate L4 shape claim

**Claim** (audit report §2.1): All 17 SOLVED chains have `L4 chain length = 5` regardless of internal iteration count (1 to 32).

**To verify**:
- For at least 3 SOLVED problems (suggest: P23 trivial, P38 medium, P49 heavy), open `verdict.json` and read `tape_root.l4_count` and `tx_kind_counts`.
- Cross-check against `MINIF2F_M1_BENCHMARK_REPORT.md` §8 per-problem table.

**Pass condition**: every SOLVED chain has `l4_count = 5` and `work=verify=task_open=escrow_lock=finalize_reward=1`, all other tx kinds = 0.

### V2 — P49 internal iteration claim

**Claim** (audit report §2.2): P49 ran 32 LLM-Lean cycles (`step:31 + omega_wtool:1`) producing 31 failed branches, but emitted only 1 Work tx.

**To verify**:
- Read `P49_numbertheory_2pownm1prime_nprime/evaluator.stdout` — single PPUT_RESULT line — extract `tx_count`, `gp_node_count`, `failed_branch_count`, `tool_dist`.
- Read `P49_numbertheory_2pownm1prime_nprime/verdict.json` — `tx_kind_counts.work`.

**Pass condition**: `tx_count=32` AND `tool_dist.step=31` AND `tool_dist.omega_wtool=1` AND `verdict.json::tx_kind_counts.work=1`.

### V3 — L4.E content claim

**Claim** (audit report §2.2 + A5): P49's `rejections.jsonl` contains only atom3-synthetic gate rejections, zero LLM-vs-Lean collisions.

**To verify**:
- Read `P49/runtime_repo/rejections.jsonl` (jsonl, 2 lines).
- For each line, extract `agent_id`, `tx_kind`, `public_summary`.
- Cross-check against `P49/runtime_repo/agent_audit_trail.jsonl` (2 lines, atom3-synthetic).

**Pass condition**: both rejection entries have `agent_id` ∈ {`tb6-smoke-sponsor`, `tb6-smoke-agent`} (i.e. atom3-synthetic, not the actual LLM-driven Solver agent which would be `Agent_0` per the bootstrap pre-seed in `chain_runtime.rs:180`).

### V4 — CAS object content claim

**Claim** (audit report §2.2): P49's 13 CAS objects contain the 5 L4-tx wrappers + 4 atom3-synthetic-gate fixtures + 1 final composite proof + 1 verification_result + 1 telemetry + 1 rejection diagnostic. Zero objects represent intermediate iterations.

**To verify**:
- Read `P49/cas/.turingos_cas_index.jsonl` (13 lines).
- For each line, extract `creator`, `object_type`, `schema_id`, `size_bytes`, `created_at_logical_t`.
- Identify which of the 13 objects could plausibly hold an intermediate-iteration record.

**Pass condition**: no object has a `creator` or `schema_id` indicating per-iteration storage. Specifically: no `proposal_artifact_iteration_N` or similar.

### V5 — Final proof iteration ghost-trace

**Claim** (audit report §2.3): The final `.lean` proof file at `P49/proofs/numbertheory_2pownm1prime_nprime_*.lean` contains textual evidence of iteration (duplicate `have h2`, redundant `rfl`-only statements, shadowed `hMersenne_prime`).

**To verify**:
- Read the `.lean` file lines 22-64.
- Count: how many times is `have h2 : 2^n - 1 > 0 := by` defined? How many `have hN : 2^n - 1 = 2^n - 1 := rfl` statements? How many `have hMersenne_prime` statements?

**Pass condition**: the redundancies enumerated in audit report §2.3 are verbatim present in the file.

### V6 — Driver loop violation (code citation)

**Claim** (audit report §3.1, §3.2): `evaluator.rs:1641` is the iteration loop boundary; `tool_dist` increments at lines 2805/3236/3263/3275 are not paired with any `bus.submit_typed_tx` call inside the loop body.

**To verify**:
- Read `evaluator.rs` lines 1641–3300 (the swarm-loop body).
- Identify every `submit_typed_tx` call site within this range. Determine which fire inside the per-iteration body and which fire after loop exit on the success path.

**Pass condition**: zero `submit_typed_tx` calls inside the per-iteration body that would correspond to "this iteration's intermediate Work tx". The submission at line 2161 fires once-per-problem after the inner loop produces a final accepted composite, not once-per-iteration.

### V7 — Sequencer self-restriction absence

**Claim** (audit report §3.3): The sequencer admits any signed tx satisfying parent_state_root + signature + per-kind invariants. There is no protocol-level rule preventing one process from emitting Work_i, Verify_i for i=1..32.

**To verify**:
- Grep `src/state/sequencer.rs` for self-restriction patterns: `challenger.*work_author`, `verifier_agent ==`, `same_agent`, `cannot.*own`, `reject.*author`, `self_challenge`.
- Read `src/runtime/agent_keypairs.rs` to confirm `AgentKeypairRegistry` allows one process to hold multiple agent_id keypairs.
- Verify by inspection that `comprehensive_arena.rs` (which does emit Work + Verify + Challenge from one process across 5 agent_ids) is accepted by the sequencer.

**Pass condition**: zero hits for self-restriction patterns; AgentKeypairRegistry confirmed multi-agent; comprehensive_arena verdict is `PROCEED` per `tb_18_single_chain_13_of_13/r2/verdict.json`.

### V8 — Architect ratification text

**Claim** (audit report §4 A9): The memory `feedback_chaintape_externalized_proposal` explicitly names "M<N entered ChainTape — that IS a violation" as the violation pattern, and the M1 reality (N=32, M=1 on P49) instantiates it.

**To verify**:
- Read `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_chaintape_externalized_proposal.md` in full.
- Locate the exact phrase "that IS a violation".
- Confirm the rule is "1 LLM call producing 1 compound payload = 1 Attempt Node" and that the M1 reality (P49 collapsing 32 LLM calls into 1 Work tx) maps onto the violation pattern, not onto the legitimate "compound payload of 3 tactics = 1 node" exception.

**Pass condition**: the memory file contains the cited phrase verbatim, and the M1 reality matches the violation pattern (32 distinct LLM calls each costing tokens, not a single LLM call returning a compound payload — verifiable via `total_run_token_count = 42937` divided by ~1000-1300 token budget per call).

### V9 — CN-δ fix is Class 2, not Class 4

**Claim** (audit report §7.5): The proposed fix does not require sequencer admission rule change, typed-tx schema bump, or canonical signing payload change.

**To verify**:
- Read `src/state/typed_tx.rs:234` (`WorkTx`) and `:257` (`VerifyTx`). Confirm fields support: parent_state_root, proposal_artifact_cid, agent_id, signature, target_work_tx (for Verify), verdict (Confirm/Disconfirm).
- Read the L4.E rejection path in `src/state/sequencer.rs` (search for `append_rejected` and surrounding code, esp. line ~2939).
- Determine: can a `VerifyTx{verdict=Disconfirm}` route a Work tx to L4.E with `raw_diagnostic_cid = stderr_cid`? Or does this require a new admission predicate?

**Pass condition**: existing schema and admission rules suffice for CN-δ. If a new admission predicate IS required, the fix slips to Class 4 and the verdict should reflect that.

### V10 — Aggregate cost projection

**Claim** (audit report §7.1–7.3): CN-δ adds <1% wall-clock overhead, ~50KB CAS per problem, audit_tape complexity O(35 + 5N).

**To verify**:
- Sequencer admit timing: instrument `bus.submit_typed_tx` once on a synthetic 32-tx chain (or read existing benchmark from `tb_18_single_chain_13_of_13` data — 31 L4 entries on `refs/transitions/main`); divide by 32.
- CAS size: average size of `proposal_artifact + lean_stderr + verification_result` from existing P49 data ≈ ?
- Audit complexity: count current `audit_tape` assertions in `src/runtime/audit_assertions.rs`; project the linear scaling.

**Pass condition**: cited overheads are within an order of magnitude of the report's claims.

---

## 5. Verdict format

Return verdict in this exact shape:

```markdown
# External Audit Verdict — TB-18 M1 Tape Non-Externalization

**Auditor**: <Codex / Gemini / your name>
**Date**: <YYYY-MM-DD>
**Commit audited**: b93582e
**Constitution sha256 verified**: <yes / no>

## V1 — Aggregate L4 shape
**Verdict**: <verified / refuted / inconclusive>
**Method**: <which files read, which fields>
**Findings**: <2-4 sentences>

## V2 … V10
… (same format)

## Overall verdict on the 9 axiom violations (audit report §4)

| Axiom | Audit claim | My verdict | Reasoning |
|---|---|---|---|
| A1 | Tape amputated | <PASS/CHALLENGE/VETO> | <citation> |
| A2 | Boolean predicate not durable | … | … |
| A3 | Goodhart shielding misplaced | … | … |
| A4 | PCP / weak-supervises-strong broken | … | … |
| A5 | Markov rule cannot fire | … | … |
| A6 | Bottleneck Theorem fuel discarded | … | … |
| A7 | Everything-is-a-File violated | … | … |
| A8 | ArchitectAI/JudgeAI input pipeline broken | … | … |
| A9 | Architect-ratified granularity violated | … | … |

## Overall verdict on the proposed fix (audit report §6 CN-δ)

**Class assessment**: <Class 2 / Class 3 / Class 4> with reasoning.
**Implementation feasibility**: <feasible / requires schema change / requires constitution amendment>
**Recommended modifications**: <if any>

## Overall ship verdict

<VETO of TB-18 M1 SOLVED claims / CHALLENGE with required modifications / PASS with caveats>

## Open issues raised by my audit (not in original report)

<anything you noticed that the internal auditor missed>
```

---

## 6. Tone + scope discipline

- **Cite line numbers**, not paraphrases. Every claim should be re-derivable from your citations.
- **No bucket-OBS**: every axiom violation gets an individual verdict, not "all 9 OK" or "all 9 broken". Per `feedback_audit_obs_bias`, the internal auditor was expected to deliver this discipline; you are expected to apply it independently.
- **Take a position**: per `feedback_architect_deviation_stance`, do not flag for ratification — render verdict with reasoning. If you would PASS but want a modification, write CHALLENGE-with-required-mod, not "ratify-then-fix".
- **Don't be polite**: if you find the audit report's reasoning weak on any axiom, mark it CHALLENGE or VETO. Per `feedback_dual_audit_conflict`, conservative verdict wins on disagreement (VETO > CHALLENGE > PASS), so a real CHALLENGE matters.
- **Time-box**: aim for ~3 hours. If you can't reach a confident verdict on V9 or V10 within that budget, mark `inconclusive` and explain.

---

## 7. Reference glossary (for non-TuringOS auditors)

- **L4**: layer 4 ledger = ChainTape accepted state transitions.
- **L4.E**: layer 4 rejection-evidence ledger = admittedly-malformed-or-rejected tx records.
- **CAS**: content-addressed store; git-objects style. Each object has a CID = sha256 of content.
- **PPUT**: "Proof Production per Unit of Time" — TuringOS's North Star metric. v2 schema is the current PPUT_RESULT JSON shape.
- **Lean**: Lean 4 theorem prover. The bottom-white predicate for math problems.
- **OmegaAccepted**: terminal state where Lean confirms full proof.
- **MaxTxExhausted**: terminal state where the swarm-loop hits `max_transactions` without OMEGA.
- **WallClockCap**: external SIGTERM at 600s wall-clock.
- **audit_tape**: the post-hoc auditor binary that walks a chain and runs all assertions.
- **comprehensive_arena**: the 13/13 synthetic chain emitter used to prove protocol-level coverage.
- **agent_id**: pseudonymous Ed25519-keyed identity. Solver = Agent_0, Verifier could be Agent_1, etc.
- **state_root**: cryptographic Merkle root of the QState after each accepted L4 entry.
- **constitution.md**: the human-architect's read-only ground truth (《反奥利奥架构的反奥利奥架构》Chinese-language flagship doc).

---

**End of auditor prompt. File your verdict to `handover/audits/<AUDITOR>_VERDICT_TB18_M1_TAPE_NON_EXTERNALIZATION_<YYYY-MM-DD>.md`.**
