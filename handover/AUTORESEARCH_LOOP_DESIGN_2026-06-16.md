# Audited Autoresearch Loop — design v2 (2026-06-16, research-grounded)

A loop that drives a research question to a result autonomously, where **money is
spent only on runs proven valid first, and every iteration is gated by an
independent audit node that decides whether to continue.** It stops on success
(goal predicate met) or on a "cannot continue" verdict — and is built so the
dominant failure (paying for a buggy or invalid run) is structurally impossible.

> Supersedes v1. Operator constraint (2026-06-16): **no total budget cap; the
> objective is loop control tight enough that money is never wasted on a buggy or
> invalid run.** That makes the PRE-EXECUTION validity gate the load-bearing part.

## 0. The one principle

**Spend is asymmetric: thinking/validating is ~free, executing (LLM+Lean) is metered.
So the loop pays only after a free gate proves the program is correct and the experiment
is valid.** Three layers, in cost order:

1. **GATE-1 PRE-FLIGHT (free / near-free, blocks waste)** — runs before any paid token.
2. **EXECUTION (metered, with always-on driver-level circuit breakers)** — can be killed mid-run.
3. **GATE-2 AUDIT NODE (the "something that can say no")** — independent, REFUTE-default; decides continue.

Every control is enforced by the **loop driver, never by the agent or a system-prompt
instruction** — agents rationalize past their own budgets (documented in every runaway
postmortem). The agent proposes; the driver disposes.

## 1. Grounding (researched 2026-06; full citations §11)

- **Boris Cherny — Loop Engineering:** the engineer writes the loop, not the prompts; loop =
  "cron + decision logic"; three non-negotiable stops (max-iter, no-progress, token/$ cap);
  *"half of loop engineering is something that can say no"*; anchor files so each tick doesn't
  re-derive intent.
- **Runaway postmortems** ($47K/11-day clarification ping-pong; $4,200/63-hr exponential ramp):
  caused by (a) no duplicate-input detection, (b) no cost-velocity breaker, (c) prompt-level
  not infra-level budget, (d) quadratic context growth (context re-reads = ~52% of agent spend).
- **EviBound (arXiv 2511.05524):** an evidence-contract approval gate (no placeholder run_ids;
  status=FINISHED required *before* execution) eliminated 100% of hallucinated completions (8/8→0/8).
- **Harness-engineering / SE-ML sanity checks:** positive control (known-answer probe through the
  FULL pipeline) is the canonical defense against "all-Failed ⇒ blame the model" when the harness
  is the bug. Calibration pilot must make the budget BIND (affordable < repairable, else false null).
- **The Autonomy Gate (CLEAR) + ARIS + Arbor:** three-altitude verdict (deterministic → trajectory
  judge → held-out outcome), composite ≥0.80, unscored level = BLOCK, judge-failure fails OPEN to
  human; cross-*family* reviewer (intra-family judge–executor inflates agreement); held-out set gates
  merges (dev score only navigates); auto-approve authority earned via shadow-mode.
- **TuringOS constitution (native):** Art I machine-verifiable {0,1} predicates + PCP (疑罪从无);
  Art 0.2 tape-canonical (recompute from frozen tape, not self-report); Art III Goodhart shield
  (metric hidden from the proposer; eval harness immutable); §17 G1–G6 claim-integrity; 报忧 = honesty.
- **This session = empirical proof.** I ran this loop by hand; every independent REFUTE-default audit
  caught a real defect self-grading missed (source_commit bug, overstated routing claim, false
  deep-chain dichotomy). The audit node is the part that works.

## 2. The loop — one iteration

```
  ANCHOR  read frozen spec/prereg + tape state (re-read each tick; never re-derive; never grow history)
    │
  ┌─ GATE-1  PRE-FLIGHT VALIDITY  (free / near-free — NOTHING is billed until all pass) ─────────┐
  │  V1 harness integrity   eval/verifier file hash == pinned SHA (immutable; agent can't edit it) │
  │  V2 positive control    known-answer probe through the FULL pipeline returns the known answer  │
  │  V3 negative control    a known-FAIL probe is correctly rejected (gate can discriminate)       │
  │  V4 evidence contract   success predicate is bound, observable, no placeholders (EviBound)     │
  │  V5 budget-binding       dry-run cost estimate ⇒ affordable_N < repairable_N (else false null)  │
  │  V6 scope + schema       inputs valid, paths in allowlist, no restricted-surface, creds present │
  │  → ANY fail ⇒ FIX-or-STOP-DEAD with reason. Zero paid execution on a buggy/invalid setup.      │
  └────────────────────────────────────────────────────────────────────────────────────────────────┘
    │  (passed → spend authorized for THIS atom only, capped at its envelope)
  EXECUTE  real models + real Lean → append tape-canonical evidence   ⟂ driver circuit breakers (§4) live
    │
  SELF-CHECK  cheap deterministic recompute-from-tape (G1) + early-abort if unpromising
    │
  ┌─ GATE-2  AUDIT NODE  — independent, clean-context, REFUTE-default (NOT the worker) ─────────────┐
  │  L1 predicate (0-LLM)   gates green? derive_from_tape==manifest? budget conserved? → {0,1}       │
  │  L2 trajectory (judge)  did this atom ADVANCE the goal vs churn? did the prior fix hold on a     │
  │                         REAL run? cross-context (ideally cross-family) reviewer                  │
  │  L3 claim (§17 G1–G6)   is the headline over-stated? adversarially try to REFUTE the core claim  │
  │  composite ≥ 0.80 ∧ zero hard-violations ∧ every level scored; judge-fail ⇒ fail-OPEN to human  │
  └────────────────────────────────────────────────────────────────────────────────────────────────┘
    │
  VERDICT ∈ { CONTINUE · FIX-RETRY · STOP-SUCCESS · STOP-DEAD · ESCALATE-HUMAN }
    │
  BROADCAST  abstract the lesson → anchor/lesson-store (Art II; FAIL lessons 2× weight, 30-day decay)
    └──→ loop, or terminate
```

## 3. GATE-1 — the pre-flight validity gate (the waste-preventer)

This is the design's center of gravity, because it is where the operator's money is saved.
**None of it costs an LLM call** (or at most one tiny probe). All must pass before any paid run:

- **V1 immutable harness** — the evaluator/verifier (here: the Lean judge + tape verifier + gate
  manifest) is pinned by SHA and read-only to the worker. A run whose result didn't flow through the
  pinned harness is vetoed. (Defeats metric-gaming; Art III Goodhart shield.)
- **V2 positive control** — run a *known-solvable* probe through the entire pipeline; if it doesn't
  verify, the harness is broken → abort and fix, do NOT attribute failure to the experiment.
  (This session's `calib_core_add_comm` smoke is exactly this control.)
- **V3 negative control** — a known-bad input must be rejected; proves the gate can say no at all.
- **V4 evidence contract** — the iteration's success predicate must be machine-checkable, bound to a
  real artifact, no `TBD`/placeholder; reject ill-formed goals up front (EviBound; also kills the
  unsatisfiable-goal spin from the goal-loop-guard lesson).
- **V5 budget-binding pilot** — estimate cost from a 1-task / 5–10% canary on the *same code path*;
  proceed only if the budget will BIND (affordable sample < repairable sample). A non-binding pilot ⇒
  redesign, never a false null.
- **V6 scope/schema/creds** — inputs schema-valid, paths in allowlist, no §6 restricted surface,
  gateway/Lean reachable. Deterministic, zero-LLM.

## 4. Driver-level circuit breakers (always-on during execution; agent cannot bypass)

| breaker | trip condition (default) | action |
|---|---|---|
| duplicate-input hash | same `hash(tool,args)` seen **2×** in a run | `SelfLoopDetected` → halt (catches $47K ping-pong in cycle 1) |
| cost-velocity | spend-rate > `$X/hr` (set from pilot) | trip independent of cumulative cap (catches exponential ramp) |
| token/$ envelope | per-atom ceiling = **3× pilot-median**; monitored mode at 80% | terminate atom with structured failure at 100% |
| wall-clock | per-atom OS-level timeout | kill process, git-revert, record crash as DATA (no silent retry) |
| iteration cap | per-hypothesis `max_pivots=2, max_refines=10`, tree depth 2 | terminate → return best held-out artifact |
| no-progress | output-similarity ≥0.9 over rolling-5 **or** identical verdict ×K=3 | DIAGNOSE, not retry (goal-loop-guard) |
| bounded retry | >2–3 consecutive same-phase failures | halt + escalate (retries >3 almost always fail anyway) |
| per-iteration cost-growth | new_tokens > 1.5× prior-iteration mean | abort atom (runaway context snowball) |

Plus: **model tiering** (Haiku mechanical / Sonnet structured / Opus core+audit — ~12% of all-Opus
cost), **anchor reset each tick** (no quadratic history; progress lives in git/tape), **compress at
phase boundaries** with a cheap model (≈50% savings at W=2).

## 5. GATE-2 verdict domain & termination

| verdict | trigger | action |
|---|---|---|
| **CONTINUE** | goal progress, L1–L3 clean | next iteration |
| **FIX-RETRY** | recoverable defect (CHALLENGE) | re-enter with the fix; recurrence counts to no-progress |
| **STOP-SUCCESS** | goal predicate met ∧ L1–L3 PASS ∧ held-out gate ∧ (ship/paid) post-data clean audit PASS | terminate ✓ |
| **STOP-DEAD** | cannot continue (see below) | terminate ✗, dead-reason on tape |
| **ESCALATE-HUMAN** | human-only gate (Class-4 §8 / architect sign-off / paid authz / ambiguous VETO) | **park + surface, do not retry** |

**"Until" — STOP-DEAD on any:** budget/velocity/wall-clock/iteration breaker; K-consecutive no-progress;
**core hypothesis adversarially refuted** (research answered *no* — a valid terminus); **GATE-1
unsatisfiable** (needs a human-only decision → becomes ESCALATE; if unaddressed, park — never spin).

**Result acceptance (research metric):** single-metric ratchet with a rolling-5 noise floor →
KEEP / RETEST (within noise) / DISCARD; **held-out set gates the merge**, dev score only navigates.

## 6. Trust graduation & human rail

Start in **checkpoint mode** (human confirms each CONTINUE). The audit node's auto-CONTINUE authority
is *earned*: run it in **shadow** against the human's calls; promote to **auto** only after agreement
≥0.80 over a window with zero gate-passed/human-rejected cases (Autonomy Gate). **Adaptive SmartPause:**
when audit confidence < threshold, enqueue a human interrupt instead of auto-deciding. The human rail
is never removed — "the gate can always swing back."

## 7. Failure-mode → mechanism (each from a real source)

| failure | mechanism | source |
|---|---|---|
| pay for buggy harness | GATE-1 V1/V2/V3 positive+negative control, pinned hash | SE-ML; harness-engineering.ai |
| pay for invalid experiment | GATE-1 V4 evidence contract + V5 binding-budget pilot | EviBound 2511.05524; TuringOS pilot rule |
| $47K clarification loop | duplicate-input hash (K=2) | dev.to/$47K postmortem |
| $4,200 exponential ramp | cost-velocity breaker | $4,200 postmortem; TrueFoundry |
| prompt-budget ignored | enforce at driver/middleware, not prompt | waxell enforcement |
| quadratic context cost | anchor reset + phase-boundary compression | augmentcode; grislabs |
| self-lenient grading | independent clean-context REFUTE-default (cross-family ideal) | vadim.blog; ARIS 2605.03042 |
| audit node errors | fail OPEN to human | Autonomy Gate |
| metric gaming | immutable eval; criteria hidden from worker | datacamp autoresearch; Art III |
| overfit to eval | held-out gates merge | Arbor 2606.11926 |
| plausible-unsupported claim | §17 G1 recompute-from-tape; claim↔evidence map | ARIS; AGENTS.md §17 |

## 8. Instantiation — the H-HET-2 drive as an audited loop

- **Anchor:** `H_HET_2_DYNAMIC_MODEL_BUDGET_PREREG_2026-06-15.md` + charter + ruling + tape.
- **Goal predicate (prereg §4):** Primary-A positive (paired McNemar Δ_union) ∧ ≥1 Primary-B witness,
  replay+axiom-clean, §17 G1–G6.
- **Iterations:** (i) deep-theorem calibration (gate #4) → (ii) freeze target list + BestHOMO →
  (iii) confirmatory pilot K≥12 paired → (iv) scale + later-day re-run stability → (v) post-data audit.
- **GATE-1 for the calibration iteration (where your money is protected):** V1 pinned Lean-judge +
  tape-verifier (done — binary_sha256 in manifest); V2 positive control = `calib_core_add_comm`
  (already verifies); V3 negative control = a deliberately-wrong body must be rejected; V4 the
  per-(model,target) coverage predicate is machine-checkable; V5 the 2-theorem pilot already measured
  ~80–120s & ~3k tokens/theorem at ~12s/proposal — extend to a binding-budget estimate over the
  ~37 non-det pool before the full sweep; V6 mathlib reachable + axiom-clean.
- **ESCALATE-HUMAN (parked):** architect sign-off on the frozen target list + paid confirmatory
  authorization (prereg §9) + any Class-4 §8.
- **STOP-DEAD:** no Goldilocks targets survive calibration (no complementary coverage in this pool →
  H-HET-2 answered *no* here); BestHOMO dominates every cell; budget/velocity breaker.
- **Open refinements feeding GATE-1/L3:** served_model provenance (#1) + MODEL_RATES→CAS (#2) — the
  prereg assumes them; close before the paid confirmatory.

## 9. Implementation on the `Workflow` primitive

Each iteration = one `Workflow` run. GATE-1 = a cheap agent/script returning a `{pass, fails[]}`
schema (mostly deterministic Bash + one tiny positive-control probe) — the driver refuses to spend
if any fail. EXECUTE = `pipeline`/`parallel` over the atom. GATE-2 = a mandatory audit agent
(clean-context, REFUTE-default) with a `StructuredOutput` verdict schema. The **main loop is the
driver**: it reads GATE-1, meters execution against `budget.remaining()` + the breakers, reads the
GATE-2 verdict, and persists CONTINUE/STOP/ESCALATE + dead-reason to the tape. No-progress counter +
duplicate-hash live in the driver. ESCALATE writes a parked-decision record and halts. Begin in
checkpoint mode; graduate to auto via shadow agreement.

## 10. Default parameters (tune from the pilot, never guess)

```
max_pivots=2  max_refines=10  tree_depth=2          # per-hypothesis iteration caps
no_progress_K=3  output_similarity=0.90  dup_hash_K=2
per_atom_token_envelope = 3 × pilot_median          # hard; monitored at 80%
cost_velocity_trip = set from pilot $/hr            # independent of cumulative
wall_clock_per_atom = from pilot p99 × 1.5          # OS-level kill
retry_per_phase ≤ 2                                  # then halt+escalate
audit_composite ≥ 0.80 ; unscored level = BLOCK ; judge_fail = fail-open
noise_floor = std(rolling last 5) ; KEEP if Δ > 1.5σ else RETEST
model tiering: Haiku mechanical / Sonnet structured / Opus core+audit
```

## 11. Citations

Cherny loop: thenewstack.io/loop-engineering, medium mountain-movers three-stage, explainx.ai loop-engineering-2026.
Cost/runaway: dev.to $47K postmortem, medium $4,200 postmortem, waxell.ai enforcement, dev.to circuit-breakers,
grislabs 1000-runs, truefoundry rate-limiting, braintrust cost-tracking, leanopstech runaway.
Pre-flight validity: arXiv 2511.05524 (EviBound), se-ml.github.io sanity-check, harness-engineering.ai,
ranjankumar.in gated-execution, arXiv 2603.15676 quality-gate, arXiv 2601.08815 agent-contract, laikatest pilots.
Autoresearch gating: vadim.blog autonomy-gate, arXiv 2605.03042 (ARIS), arXiv 2606.11926 (Arbor),
arXiv 2605.20025 (AutoResearchClaw), datacamp autoresearch, dev.to dean0x eval-tools.
Native: constitution.md (Art I/0.2/III), AGENTS.md §17, and this session's audit trail under handover/audits/.
