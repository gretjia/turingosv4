# H-HET-2 Agent-Economy Experiment — New-Session Boot Prompt (2026-06-17)

> Paste this whole file as the new session's first message (or: "read
> `handover/H_HET_2_SESSION_BOOT_2026-06-17.md` and continue"). It is self-contained but points
> to the canonical frozen sources for detail. Prior session ran very long; this is the clean handoff.

## 0. Who / where / language

You are the implementation+orchestration agent continuing **TuringOS H-HET-2**, the *dynamic
model-budget agent-economy* experiment. Architect = **zephryj** (solo, Chinese-default user-facing
language; technical terms may stay English; he grants autonomy, wants clear decisions + honest
"bad news first" + no ceremony). You have autonomous decision authority; legal stop points are
all-merged or a Class-4 §8 / paid-run / architect-decision gate.

- **Repo:** `/Users/zephryj/work/turingosv4-converge`
- **Branch:** `claude/het-converge-2026-06-16` (the experiment branch; 13 ahead of `origin/main`,
  3 behind — main moved because this session harvested de-Lean/model_id/lean_judge/autoloop to it).
  Continue ON this branch. Rebasing onto the new main is OPTIONAL cleanup (de-Lean is the same content
  both places; a 123-file rebase is risky — only do it if you need main's other 3 commits, and verify green).
- **Dirty (expected):** `scripts/autoloop/h2_loop_state.json` (the loop tape), `src/bottom_white/cas/schema.rs`
  (a wire-pin test fix — already upstreamed via PR #345, so redundant here; safe to keep or `git checkout` it).
- **This is a home-dir-style context: repo CLAUDE.md/constitution are NOT auto-loaded.** Do the cold-start reads (§1).

## 1. MANDATORY cold-start reads (in order)

1. `AGENTS.md` (the universal agent contract: truth order, risk classes §5, restricted surfaces §6,
   PR-only §14a, audit doctrine §9, §17 claim-integrity, OBLIGATIONS §16).
2. `handover/preregistration/H_HET_2_DYNAMIC_MODEL_BUDGET_PREREG_2026-06-15.md` — the FROZEN prereg.
3. `handover/tracer_bullets/TB_DYNAMIC_MODEL_BUDGET_MARKET_charter_2026-06-15.md` + `H_HET_2_ROUTING_POLICY_RULING_2026-06-15.md`.
4. `H2_AGENT_ECONOMY_STATE.md` (live status + the audit-derived fix list) and `H2_LIVE_MECHANISM_SMOKE_REPORT.md`.
5. `handover/audits/H2_SMOKE_QC_RECURSIVE_AUDIT_2026-06-16.md` (the 6-auditor QC + the harness-validity audit).
6. `scripts/autoloop/README.md` + `handover/AUTORESEARCH_LOOP_DESIGN_2026-06-16.md` (the gated loop you run the experiment through).
7. `constitution.md` (axiom layer) — esp. Art 0.2 tape-canonical, Art I predicates, Art V.1.3 Veto-AI.

## 2. The experiment (frozen claim boundary)

**Hypothesis (prereg §1):** *dynamic model-budget routing converts complementary coverage into
capability the best single model (BestHOMO) cannot match at equal-or-lower total cost.* The lever is
budget **routing**, NOT the roster. **Primary metric (prereg §4):** `Δ_union` = per-(model,target)
union coverage over seed×target cells (exact paired sign/McNemar) **AND** ≥1 Primary-B existence
witness (a target TREAT solves ≥6/12 seeds AND BestHOMO 0/12), replay+axiom-clean, §17 G1–G6.
SUPPORTED = Primary-A positive AND ≥1 Primary-B. No PROVEN headline without §17 G1–G6.

**Mechanism (implemented, on branch):** `Policy::VerifyUcbPriceFloor` in `src/bin/lean_market_agent.rs`
routes the scarce proposal-call budget across the 4-model roster via `src/runtime/routing_policy.rs`
(FROZEN_POLICY_HASH `9fb0f612df2054049a3799869aafe6c401eb8c72c27a1e581d3ed901913f263a`, deterministic
UCB + bounded price-prior + ε-floor, no RNG). **`parent=None` BY DESIGN** (isolates the model-budget
lever from node-routing) → golden-path is structurally depth-1; **depth is a PRE-RUN target-selection
proxy, NOT a scoring metric** (do not re-litigate this — it cost the last session a refuted detour).
Decisions are tape-canonical: `BudgetAllocationTelemetry` (selection_reason, scored rows, budget) +
`ProposalTelemetry.model_id` (v2). Roster: `deepseek-ai/DeepSeek-V4-Pro, Qwen/Qwen3-32B,
zai-org/GLM-4.5-Air, Qwen/Qwen3.5-397B-A17B` (SiliconFlow via the proxy; rates in `src/market_tape_shared.rs`).

## 3. What is DONE

- Mechanism + tape-canonical decision records (model_id hard-gate MET); experiment-validity gates
  GA-0/2/3/5/6a/6b/7/8 (failable, registered); no-LLM counterfactual sim.
- **Live mechanism smoke PASS** (`smoke_ucb_001/002/003`): omega reached on a real Lean 4.24.0 verify;
  the router distributed across heterogeneous models ON the canonical tape (decoded byte-exact);
  replay byte-clean. `smoke_ucb_003` (7 ticks) even showed `selection_reason=UcbScore` once (value-driven,
  not just tie-break). **SMOKE ≠ evidence** (n small, single seed, easy targets).
- **6-auditor adversarial QC** → QC-CONCERNS (no VIOLATION): 3 core claims hold byte-level; corrected 6
  overstatements (routing was tie-break not value-driven at 2 ticks; model_id is the REQUESTED label not
  served-model; cost needs the external rate table; conservation not yet closed; etc.).
- **2 cheap integrity fixes, real-run verified** (on branch): **#3** run-path budget conservation
  (CALL-unit `bat::budget_alloc_fields`; GA-5 predicate `before − allocated_proposal_budget == after`)
  and **#5** manifest `binary_sha256` + `source_commit`.
- **Harness-validity audit (HARNESS-SOUND):** the canary's high failure rate is genuine mathlib
  API-recall difficulty, not a harness bug. Calibration will produce real solvability data.
- **Harvested to `origin/main` this session (PRs #344/#345/#346):** autoloop harness; de-Lean kernel
  migration + §8 model_id + lean_judge correctness (Class-4, Veto-AI PASS); BSD-portable gate runner.

## 4. NEXT action — Step 6: the deep-theorem calibration (gate #4)

Prereg §5/§9 gate #4 (the ONLY open technical blocker for a paid run, besides architect sign-off):
a **disjoint deep-theorem calibration pass** to freeze the target pool — for each candidate, confirm
(i) no single model one-shots it AND (ii) budget-Goldilocks holds, scored by per-(model,target) coverage,
at tx/agent ≳ 20. **Source pool:** the deeper theorems in `tests/fixtures/lean_theorems_pool.jsonl`
(44, all need mathlib), EXCLUDING the det-family one-liners.

**Run it through the gated autoloop** (`scripts/autoloop/`, config `h2_calibration.config.json`):
`autoloop.py preflight` (GATE-1: harness-hash + positive/negative control + budget-binding + scope) →
breakers → EXECUTE the carrier per (model,theorem) → GATE-2 independent REFUTE-default audit → record.
The loop tape `h2_loop_state.json` is currently `status: running, 3 iterations` (ESCALATE park +
V5 canary + a FIX-RETRY pausing for the perf fix below). Calibration only needs **1 seed** to classify
(not the confirmatory's 12). Money is trivial (**~$3–5 full sweep**); **wall-clock is THE constraint.**

### 4a. DO THIS FIRST — the persistent-Lean perf fix (architect-chosen, deferred from last session)

The wall-clock bottleneck is INTERNAL: the Lean judge spawns a **fresh `lean` process per verify**, each
reloading `import Mathlib` (the umbrella) = **4.3s warm / 22.6s cold ×up-to-80 verifies/run** — making a
148-run calibration (and the 12× larger confirmatory) impractically slow. **Fix before the sweep**
(architect authorized "build it + Veto-AI + auto-merge on PASS"). It touches the V1-pinned `lean_judge.rs`
(correctness-critical) so do it gated: implement → independent audit → re-pin harness hash → re-verify the
positive control + benchmark the speedup → keep the `lean`-per-verify path as a feature-flagged EQUIVALENCE
ORACLE and A/B it (assert byte-identical verdicts) before trusting the fast path.

**Integration plan (from the 2026-06-16 research; the /tmp research output is ephemeral, so this is the
durable copy):**
- Approach: **leanprover-community/repl** — load `import Mathlib` ONCE (`{"cmd":"import Mathlib"}`, no env)
  → `BASE_ENV`; each check `{"cmd":<body>,"env":BASE_ENV}` (NEVER chain envs); ~22s→~1s.
- **Version gotcha:** repl master pins Lean v4.31.0 — you MUST use the repl commit matching v4.24.0
  (the "bump toolchain to v4.24.0 #131", 2025-10-14) OR use **LeanInteract** (`pip install lean-interact`,
  `LocalProject("/Users/zephryj/work/mathlib4")`, auto-matches the toolchain — a spike was started in venv
  `/tmp/leanvenv` but interrupted) OR **kimina-lean-server** (`LEAN_SERVER_LEAN_VERSION=v4.24.0`).
- **Verdict equivalence to `lean -DwarningAsError=true`:** reject if any message `severity ∈ {error,warning}`
  OR `sorries` non-empty (the REPL does NOT auto-promote warnings; sorry surfaces as a warning).
- **Axiom check:** `{"cmd":"#print axioms <name>","env":BASE_ENV}`; parse `depends on axioms: [...]`; accept
  iff ⊆ `{propext, Classical.choice, Quot.sound}` (catches `native_decide`→`Lean.ofReduceBool`).
- **Lifecycle:** `lake env <repl>` for LEAN_PATH; blank-line (double-newline) terminator; per-proof
  `asyncio.wait_for` + `os.killpg` on timeout + restart; one process per core for parallelism; pickle
  the warm env for fast restart. Cleanest boundary = a small persistent Python verify-service; the Rust
  `LeanJudge` calls it (HTTP/socket) instead of spawning `lean`, feature-flagged with the oracle fallback.

### 4b. Recommended sequence

1. persistent-Lean fix (§4a) → gated, Veto-AI, merge/land, benchmark the speedup.
2. Step-6 calibration sweep (1 seed, the non-det pool, tx/agent≳20) through the gated autoloop → freeze
   the Goldilocks target pool (no-one-shot ∧ models-differ). **Watch the Goldilocks-null risk:** even the
   strongest model needed ~33 iid tries on an "easy" theorem under abstracted feedback; if the pool is
   uniformly hard-for-all-models, the complementary-coverage subset may be small/empty → that is a valid
   STOP-DEAD (H-HET-2 answered *no* on this pool), not a failure.
3. Close the remaining before-paid-run blockers (§5).
4. Prereg freeze/sign-off → confirmatory pilot (K≥12 seeds, arms DYN_HET / RR_HET / BestHOMO / ablation,
   paired McNemar) → post-data scientific clean-context audit (§9) → headline only if §17 G1–G6.

## 5. OPEN blockers before any PAID confirmatory run (do NOT run it until ALL are closed — §11)

- **gate #4 target pool** (the Step-6 calibration above) + **architect sign-off** on the frozen list & prereg.
- **#1 served_model provenance** — on-tape `model_id` is the router-REQUESTED label; the proxy echoes it and
  discards the upstream served model. Record served_model + assert served==requested + regression test.
- **#2 MODEL_RATES → CAS at genesis** — the per-model rate table is a compile-time const, not on tape; cost
  isn't tape-recomputable without it. Write it to CAS with the CID in GenesisPin.
- **#4 BudgetAllocationTelemetry replay reconstruction** — `verify.rs` reconstructs ProposalTelemetry but not
  the allocation; add it so `replay.json` witnesses allocation == derive_from_tape.
- **decision_source/action_source tape-canonical promotion** (currently BIN-only/null; **Class-4** schema bump).
- **GA-9** — `enable_thinking:false` on the pinned `src/drivers/llm_http.rs` (**Class-4 §8**) for a valid hard-target run.
- **Binary↔source provenance for runs** is now recorded (#5); keep using it.

## 6. Operating discipline (this session's hard-won lessons — honor them)

- **Research → deliberate → act**, esp. for 3rd-party tools (Lean/REPL/models): cheap focused web-research of
  authoritative docs/GitHub beats guess-loops.
- **Verify, don't trust** — independent clean-context REFUTE-default audits repeatedly caught the orchestrator's
  own bugs (overstated claims, a schema wire-pin bug, a lean_judge regression, a hallucinated file in a plan).
  Audit before concluding; the plan/agent output is a guide, not gospel.
- **Real test beats review** — run the FULL `cargo test --lib` / `--workspace` + `bash scripts/run_constitution_gates.sh`,
  not just gates+`cargo check` (lib unit-test failures hid from the last session). Real run from the real binary
  before claiming done. Never fake green; report FAIL verbatim.
- **Binding-budget pilot** before any multi-hour run; **checkpoint+resume** at the inner unit.
- **Class-4 (constitution/cas-schema/sequencer/typed_tx/trust-root):** §6 stop+classify; needs Veto-AI PASS
  (output domain {PASS,VETO}, constitutional only) + green-on-merged-bytes before merge; never edit a trust-root
  pin to mask a mismatch (re-pin on final bytes only).
- **PR-only (§14a):** agents open PRs; merge is the orchestrator's call. main = gretjia/turingosv4 (branch
  protection: PR-required, 0 approvals → `gh pr merge` self-merge works when the architect authorizes).
- **The gated autoloop is the harness:** never spend on a run until GATE-1 (validity) passes + breakers clear;
  GATE-2 (independent audit) decides continue; human-only gates (architect sign-off / paid authz / Class-4 §8) =
  ESCALATE-HUMAN → park, don't spin.

## 7. Prereqs (verified 2026-06-17)

- mathlib4 built at `/Users/zephryj/work/mathlib4/.lake/build/lib` (toolchain `leanprover/lean4:v4.24.0`).
- LLM gateway `http://localhost:8123/health` = 200 (proxy → SiliconFlow; can be slow — measure latency before a sweep).
- Pinned Lean `~/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean`.
- Carrier: `cargo build --bin lean_market_agent --bin verify_chaintape` (then `./target/debug/lean_market_agent --help`-style flags per `scripts/het_carrier_pilot.sh`).

## 8. First moves for the new session

1. Do the §1 cold-start reads + `python3 scripts/autoloop/autoloop.py status --state scripts/autoloop/h2_loop_state.json`.
2. Confirm prereqs (§7); `cargo check --all-targets` + `cargo test --lib` should be green on the branch.
3. Start the persistent-Lean fix (§4a) — research-grounded, gated, Veto-AI before merge. This unblocks a
   feasible calibration sweep.
4. Then drive Step-6 calibration (§4b) through the gated autoloop; surface the architect sign-off (ESCALATE)
   when the frozen target pool is ready, before any paid confirmatory run.
