# External Audit Request — Carrier "WorkTx did not advance" blocker (H-HET-2 calibration)

**Date:** 2026-06-17
**Audience:** external auditor (no access to our repository or filesystem — all relevant code and data are embedded inline below)
**What we need from you:** an independent opinion on (1) the most likely root cause of the failure described, (2) whether it is a genuine carrier defect or an expected consequence of the run configuration, and (3) which of the proposed fix directions is soundest for the experiment we are running. Concrete questions are listed at the end.

---

## 1. System context (enough to reason about the bug)

We run a **tape-first agent-economy substrate**. Relevant pieces:

- A **carrier binary** (`lean_market_agent`) runs an LLM proof-search "market." Multiple agents propose Lean proof bodies for a fixed target theorem over several rounds. Each proposal is verified by a **real Lean kernel**; a verified proof is "OMEGA" (the run's success condition).
- Every meaningful action is a **typed transaction (tx)** submitted to a **Sequencer**. The Sequencer applies txs **asynchronously**: submission only *queues* the tx; a background driver (`Sequencer::run`) applies it and advances the chain's **state root**. So the carrier, after submitting a tx, must **poll** until the state root changes before continuing.
- Per proof node the carrier submits a sequence of txs: `TaskOpen → EscrowLock → WorkTx` (the `WorkTx` carries an economic **stake**, escrowed; conservation is sequencer-enforced).

We are about to run a **calibration experiment** ("Step-6"): for each (model, theorem) we run a **homogeneous single-model** configuration and record whether it reaches OMEGA, to classify which theorems are "Goldilocks" (some models solve, others fail — at a fixed per-agent budget). The budget regime is `n_agents = 4`, `n_rounds = 20` (≈ 20 tx/agent), 1 seed.

**Recent change (relevant):** we replaced the Lean verifier's "spawn a fresh `lean` process per verify (~6.7 s each)" with a **persistent Lean service** that loads Mathlib once and verifies in ~25 ms — a **~130–260× speedup per verify**, proven **byte-identical in verdict** (see §4.3). Net effect on the carrier: **proof nodes are now produced ~hundreds of times faster**, so txs are submitted into the async Sequencer at a much higher cadence than before.

---

## 2. The failure (symptom)

The first calibration cell on the fast path — strongest model on a non-trivial analysis theorem, full budget — **ran ~29 minutes, then aborted with `WorkTx did not advance` and wrote no result manifest.**

Exact carrier invocation (single homogeneous model, full budget, fast verifier path enabled):

```
lean_market_agent \
  --problem lm_lim1 \
  --policy verify_ucb_price_floor \
  --models Qwen/Qwen3.5-397B-A17B \
  --n-agents 4 --n-rounds 20 --seed 42 \
  --bank <theorem pool> --mathlib-dir <mathlib> \
  --lean-verify-service true \
  --proxy-url http://localhost:8123 \
  --out manifest.json
```

The carrier's entire captured stdout/stderr for the run was **two lines**:

```
[lean_verify_service] ready: import Mathlib loaded in 19.06s
lean_market_agent: WorkTx did not advance
```

The post-run cost/diagnostic tool (which reads the manifest) therefore reported nothing usable:

```json
{
  "canary": { "tokens": 0, "wall_s": 1755, "omega": null },
  "full_sweep": { "runs": 148, "n_theorems": 37, "n_models": 4, "n_seeds": 1 },
  "projection": { "total_tokens": 0, "wall_hours": 72.2 },
  "budget_binds": false
}
```

Reading: the persistent Lean service started fine (`ready ... 19.06s`); the run proceeded for ~1755 s (29 min); then a `WorkTx` submission failed to advance the chain and the carrier exited **without writing a manifest** (so `tokens`/`omega` are absent, and the 72 h projection is meaningless).

For contrast, an immediately prior **harness positive-control** with the *4-model* roster on an *easy* theorem, **without** the fast path (per-process `lean`), `n_rounds=4`, **succeeded** (reached OMEGA, wrote a manifest). The relevant differences between that passing run and the failing one: fast-path **on** vs off; **single** model vs 4-model roster; **hard, never-solved** theorem vs easy; `n_rounds=20` vs 4; ~29 min vs short.

---

## 3. Embedded code — the failure mechanism

### 3.1 The submit-and-await helper that produced the error

```rust
async fn submit_await(
    seq: &Sequencer,
    tx: TypedTx,
    pre: Hash,
    label: &str,
) -> Result<Hash, String> {
    seq.submit_agent_tx(tx)
        .await
        .map_err(|e| format!("submit {label}: {e:?}"))?;
    tb8_await_state_root_advance(seq, pre, 5_000)
        .await
        .map_err(|_| format!("{label} did not advance"))
}
```

The error string is `"{label} did not advance"`, here `label = "WorkTx"`. Note the submission (`submit_agent_tx`) **did not** error — the failure is purely that the state root did not advance after submission.

### 3.2 The await-advance poll (the `5_000`)

```rust
pub async fn tb8_await_state_root_advance(
    sequencer: &Sequencer,
    pre_state_root: Hash,
    poll_budget_ms: u64,   // called with 5_000
) -> Result<Hash, ()> {
    let deadline = Instant::now() + Duration::from_millis(poll_budget_ms);
    while Instant::now() < deadline {
        if let Ok(q) = sequencer.q_snapshot() {
            if q.state_root_t != pre_state_root {
                return Ok(q.state_root_t);
            }
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    Err(())
}
```

So `WorkTx did not advance` means: **the submitted WorkTx was not reflected in the sequencer's state root within 5 seconds** (polling every 20 ms). The submission was accepted into the queue; the async `Sequencer::run` driver either rejected it on apply (e.g., an admission / conservation rule), applied it too late, or did not apply it.

### 3.3 The WorkTx submission site (per proof node)

```rust
// ... preceding per-node txs: TaskOpen(node), Escrow(node) already submitted+awaited ...
let pcid = put_proposal(&args.cas, &args.run_id, &agent, step_idx,
                        parent_tx.clone(), &body, tokens, &tick_model, lt)?;
lt += 2;
let work = make_real_worktx_signed_by(
    &mut kp, &node_task, &agent, root, work_stake, "lm", pcid, true, lt,
).map_err(|e| format!("WorkTx: {e}"))?;
let work_tx_id = match &work {
    TypedTx::Work(w) => w.tx_id.0.clone(),
    _ => return Err("not WorkTx".into()),
};
root = submit_await(&seq, work, root, "WorkTx").await?;   // <-- the failure
```

Each node thus submits `TaskOpen → Escrow → WorkTx`, and the `WorkTx` carries `work_stake` (a per-node economic stake derived from the proposal's confidence). With `n_agents=4, n_rounds=20` and a never-solved theorem, up to ~80 such node sequences are submitted in the run.

---

## 4. What we have already proven (to scope the bug)

### 4.1 The persistent Lean service started and worked
`[lean_verify_service] ready: import Mathlib loaded in 19.06s` — the service spawned inside the carrier, loaded Mathlib once, and signaled ready. No verifier error appears anywhere in the run output.

### 4.2 The harness/spend-gate is green
An independent pre-flight gate passed all checks: pinned-verifier-bytes integrity; LLM-gateway / Lean / Mathlib reachability; **positive control reached OMEGA** (carrier solved a known theorem end-to-end, 4-model roster, per-process `lean`); negative control (the axiom-soundness gate) rejected bad proofs (9/9).

### 4.3 The fast verifier is byte-identical in verdict (so the *verdict* is not the bug)
We built an A/B equivalence oracle that runs the **same** verify call through both backends (per-process `lean` vs persistent service) and compares the decision-bearing outputs. Result over **44 real pool theorems + 5 adversarial cases = 49/49 byte-identical**: same verdict (Verified / Failed / blocked), same `#print axioms` soundness footprint, same reject classification. The decision the verifier returns is therefore **provably unaffected** by the fast path. (What *did* change is the **cadence** — ~25 ms vs ~6.7 s per verify; see §5.)

---

## 5. Our analysis — candidate root causes

The failure is in the **sequencer/economic submission path**, not the verifier (the service worked; the verdict is byte-identical). The submitted WorkTx did not advance the state root within 5 s. Candidate causes, with the evidence for/against each:

**(A) Timing/cadence race exposed by the fast path.** Previously each node took ~6.7 s (the verify), giving the async `Sequencer::run` driver ample time to apply each tx before the next. With the fast path (~25 ms verify), the carrier submits `TaskOpen/Escrow/WorkTx` bursts ~hundreds of times faster. `tb8_await_state_root_advance` is a fixed 5 s wall-clock poll. If the async apply driver falls behind under the higher submission rate, a WorkTx's state-root advance could miss the 5 s window. **For:** the only configuration that newly broke is the one that newly went fast; the await is purely time-based. **Against:** 5 s is generous for a single tx; and the run was 29 min long, suggesting many nodes *did* advance before one failed (so the driver is not globally stalled).

**(B) Economic / escrow exhaustion over a long single-model run.** Each WorkTx escrows a `work_stake`. Over ~80 nodes by a single model on a never-solved theorem, the agent's balance/escrow could be depleted, after which the sequencer's conservation rule rejects the WorkTx on apply → state root never advances → 5 s timeout. **For:** the failure is late (after a long run), single-model (one balance bearing all stakes), and on a never-solved theorem (no OMEGA payout to replenish). **Against:** we have not yet confirmed the balance trajectory.

**(C) Degenerate single-model market under the `verify_ucb_price_floor` policy.** This policy is a *routing/price* mechanism designed for a heterogeneous roster; with a single model the routing is degenerate. A degenerate market state could produce an unadvanceable WorkTx. **For:** the passing positive control used the 4-model roster; the failing run used a single model. **Against:** single-model is a legitimate input; a degenerate routing should still produce valid txs.

**(D) A genuine sequencer-apply defect** (a WorkTx that *should* apply does not), independent of the above.

**What is NOT a candidate:** the verifier verdict (proven byte-identical, §4.3) and the verifier service health (started fine).

Honesty note: we cannot yet distinguish (A)–(D) from one failed run. The fast path is a *plausible indirect* contributor (it changed the submission cadence into a time-bounded async poll), even though it provably did not change any verdict.

---

## 6. The discriminating experiment we propose to run

To isolate timing (A) from config/economic (B,C,D), re-run the **identical** configuration (`lm_lim1`, single model `Qwen/Qwen3.5-397B-A17B`, `n_agents=4`, `n_rounds=20`) **with the fast path OFF** (per-process `lean`, ~6.7 s/verify):

- If it **also** fails `WorkTx did not advance` → the fast path is exonerated; the cause is config/economic/sequencer (B/C/D) and is **pre-existing**.
- If it **succeeds** (or fails differently/later) → the fast-path cadence (A) is implicated, and the fix is on the submission/await path (e.g., backpressure, or a non-time-based advance condition).

This run is slow (~hours at 6.7 s/verify × up to 80 nodes) but decisive. We have not run it yet — we are seeking your read first to avoid burning the wrong experiment.

---

## 7. Proposed fix directions (and rationale)

We see three non-exclusive directions; we want your view on soundness and on which to try first.

1. **Make the WorkTx advance robust regardless of cadence.** Replace the fixed 5 s wall-clock `tb8_await_state_root_advance` with either (a) a bounded retry/backpressure that waits on the *queue drain* rather than a fixed wall-clock, or (b) an explicit per-tx applied-confirmation rather than a state-root-changed poll. **Rationale:** if (A) is the cause, this removes a brittle time dependency without changing economics. **Risk:** could mask a real rejection (B/C/D) as a hang — must distinguish "not yet applied" from "rejected."

2. **Change the homogeneous-arm policy for the calibration.** Our calibration must measure each single model's solo coverage at equal budget (the "BestHOMO" baseline). We used `verify_ucb_price_floor` with a single-model roster (to differ from the heterogeneous TREATMENT only in roster). An alternative is a **non-market** policy (independent N-agent attempts, no price/escrow market machinery), which would avoid the degenerate single-model market and the per-node stake economics entirely. **Rationale:** if (B)/(C) is the cause, this sidesteps it and is arguably a cleaner per-model capability measure. **Risk to experiment validity:** the BestHOMO baseline must be a *fair, equal-budget* control versus the heterogeneous-routing TREATMENT; changing the homogeneous policy could make the comparison apples-to-oranges. This is the crux question for you.

3. **Treat budget-exhaustion as a recordable terminal state, not an abort.** If (B) is the cause (escrow depleted → no further WorkTx), the run should record `omega=false` with a manifest, **not** crash with `WorkTx did not advance`. The "failed" cells (a model that does NOT solve at budget) are *exactly* the data the calibration needs (the "0/K" half of the Goldilocks test). An abort that writes no manifest destroys that signal. **Rationale:** correctness of the experiment's data collection, independent of which of (A)–(D) holds.

---

## 8. Specific questions for the external auditor

1. Given §3 (the `submit_await` / `tb8_await_state_root_advance` mechanism is a fixed 5 s state-root-changed poll over an **async** apply driver) and §5(A), how seriously should we weight the **fast-path cadence** hypothesis? Is a 5 s wall-clock advance-poll a sound pattern under high submission rates, or is it inherently racy and due for replacement (direction 1)?
2. For an economic substrate where each WorkTx escrows a stake (§3.3), is **escrow/balance exhaustion over a long single-model run** (§5B) the more parsimonious explanation than a timing race? What evidence would most cheaply confirm or refute it?
3. Experiment-validity question (§7.2): to measure a **single model's** solo coverage as the **BestHOMO** baseline against a **heterogeneous dynamic-routing** treatment **at equal budget**, is it sounder to (a) run the homogeneous arm under the *same* market/routing policy with a one-model roster, or (b) run it under a *non-market* independent-attempts policy? Which preserves a fair equal-budget comparison?
4. Do you agree that **direction 3** (budget-exhaustion must be a recorded terminal state with a manifest, never an abort) is necessary regardless of root cause — because the unsolved cells are required data, not errors?
5. Is the **discriminating experiment** in §6 the right next step, or is there a cheaper, more decisive diagnostic you would run first?

---

### Appendix — what is already done and proven (context for scope)

- The fast verifier change is **committed** and self-contained (verifier-backend seam + a verifier soundness-footprint fix + the A/B oracle + the opt-in carrier flag); the default verifier path is unchanged; the fast path is opt-in.
- The A/B oracle (49/49 byte-identical, §4.3) and the harness pre-flight gate (§4.2) are green.
- This blocker surfaced on the **first** carrier run that combined: fast verifier path **on** + single homogeneous model + full budget (`n_rounds=20`) + a never-solved deep theorem. It was caught by an intentional **single-cell binding-budget probe before** committing to the full multi-hour, ~$3–5, 148-cell sweep — i.e., the guardrail worked as designed.
