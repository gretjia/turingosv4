# TB — Dynamic Model-Budget Market (H-HET-2) — CHARTER (draft, awaiting §8 freeze + Veto-AI)

**Status:** DRAFT for architect review. Authorized 2026-06-15 (architect audit
ruling #2: "授权起草 dynamic model-budget market TB charter"). No paid run until
the three hard gates (§5) are satisfied + this charter is frozen + Veto-AI PASS.

**Authority:** architect independent audit 2026-06-15 (rulings + 3 hard gates +
Art 0.4 declaration requirement) + the V3 "Good market example" (ζ-Sum Run 6,
90 agents × 6000tx, Qwen2.5-7B reaches OMEGA on an 18-step golden path via
scale+budget, DAG fully reconstructed from tape).

---

## 1. Why this experiment exists (what the H-HET-1 pilot did and did NOT show)

The H-HET-1 pilot (2026-06-15, audited NO-VIOLATION) is a **scoped null**: on a
det-family band, at **3 tx/agent** (NA=4, NR=3 = 12 proposals/cell), with proofs
of **golden-path depth mean 1.1 / max 2** (one-shot theorems), a fixed
round-robin heterogeneous market did not beat the best single model (Q397), on
solve rate (Wilson CIs overlap) or token-economics (Q397-homo dominated PPUT).

Two structural reasons that null says nothing about the architecture's thesis:

1. **Budget regime.** Per the V3 scaling law (`tx_budget ≥ agents × 20` for
   deep proof search), 3 tx/agent is the **failure regime** — V3 itself got
   depth 5 / NO proof at 3.3 tx/agent, and depth 18 / OMEGA at 19 tx/agent. A
   collaborative market cannot express value when each agent gets ~3 turns.
2. **Depth regime.** Det-family theorems are one-shot (`simp [...]; norm_num`).
   A market's value is collaborative *chain-building* on theorems no single
   model one-shots; shallow targets let a strong single model win trivially.

The genuine residual signal from H-HET-1 is **complementary coverage**: at the
homogeneous level DSHOMO uniquely solves `{lm_det_zero, lm_det_3x3}` and
Q397HOMO uniquely solves `lm_det_2x2`; no single model covers all. The current
carrier routes *which node*, never *which model gets budget*, so it cannot
convert latent coverage into a win. **This charter builds and tests the missing
mechanism.**

## 2. Treatment vs control (the lever is budget ROUTING, not the roster)

- **TREATMENT — dynamic model-budget market.** A priced/bandit carrier that
  reallocates the scarce resource (proposal-call budget) toward whichever model
  is currently verifying / whose price + abstracted-failure signal warrants it
  (Art II.2 price broadcast + Art II.2.1 explore/exploit + Art II.1 abstracted
  failure memory via Librarian). Concretely: budget should flow to Q397 on a
  theorem Q397 is cracking and to DeepSeek on one only DeepSeek cracks.
- **CONTROL-1 — best single model homogeneous** (Q397HOMO) at the SAME total
  token budget. This is the bar to beat (it dominated H-HET-1).
- **CONTROL-2 — fixed round-robin heterogeneous** (the H-HET-1 carrier) at the
  same budget. Isolates "dynamic routing" from "heterogeneous roster."

The claim is NOT "heterogeneity helps." It is: **dynamic budget routing converts
complementary coverage into capability the best single model cannot match at
equal-or-lower total budget.**

## 3. Targets — deep, hard, budget-Goldilocks (NOT shallow det one-liners)

Pre-select theorems where (a) no single model one-shots them (golden-path depth
must be able to exceed ~5), AND (b) the budget-Goldilocks property holds: some
model 0/K and another ≥1/K at the experiment's per-agent budget. Source pool =
the deeper theorems in `tests/fixtures/lean_theorems_pool.jsonl` + the V3-class
multi-step targets (ζ-regularization-style), explicitly EXCLUDING the
det-family one-liners that H-HET-1 already showed are one-shot. Budget set per
the scaling law: **tx/agent ≥ 20** (e.g. NA=4..8, NR≥20, or larger agent pools
à la V3's 90 agents if rate limits allow). Budget binds = unsolved-at-budget is
a real outcome, not a tooling artifact.

## 4. Primary claim + metrics (frozen before any paid run — hard gate, §5.2)

- **Primary metric:** UNION coverage — number of target theorems solved by the
  TREATMENT that are NOT solved by CONTROL-1 (Q397HOMO) within the same total
  token budget. Success = treatment solves ≥1 theorem Q397HOMO cannot, at
  equal-or-lower total tokens, replay-clean + axiom-clean.
- **Secondary:** token-pure golden-path tokens per solve; **serial** PPUT
  (ΣPPUT, Mean-PPUT(solved)) — NOT concurrency-contaminated wall-clock PPUT;
  per-model budget share vs per-model verify rate (does budget actually flow to
  the winning model?); Wilson CI on solve rate.
- **No PROVEN headline** without §17 G1–G6 (recompute-from-tape, real
  model+verifier, fair equal-budget baseline, ≥N seeds, post-data clean audit,
  no literal pass-condition).

## 5. The three hard gates (architect, 2026-06-15) — ALL required before paid run

1. **`model_id` must be tape-canonical.** Per-proposal model provenance must be
   reconstructable from the frozen ChainTape+CAS (the §8 `ProposalTelemetry.model_id`
   schema-v2 change), NOT inferred from a round-robin rule or read from a
   manifest/runner sidecar. Dynamic routing breaks the fixed 1:1 agent→model
   map, so the round-robin inference H-HET-1 relied on becomes invalid; without
   `model_id` on tape, per-model results do NOT count toward the primary metric.
2. **Serial / token-pure primary metric pre-registered.** The canonical
   efficiency metric is serial PPUT or a token-pure metric (golden-path tokens),
   fixed in the prereg. Concurrency-contaminated wall-clock PPUT may not be the
   canonical primary.
3. **Dynamic policy frozen before paid run.** The bandit/pricing rule (how
   budget reallocates) must be frozen + sha-pinned before spend, else it
   reintroduces p-hacking + Goodhart risk (Art III.4: the allocation metric must
   not be gameable by the proposers).

## 6. Power

K ≥ 12 preregistered seeds, within-seed Wilcoxon signed-rank pairing
(treatment vs each control on the SAME seed/target). Goldilocks pool pre-selected
per §3. Stable across a later-day re-run (no single-day flip).

## 7. Required artifact — tape-reconstructed DAG (the V3 deliverable)

For each solved target the experiment must emit, **reconstructed from the frozen
tape (not the manifest sidecar)**, a V3-style report: the citation DAG (roots →
golden path → branches), the golden-path steps with author agent + **model**
(now tape-canonical via §8), role/model activity breakdown, trading/price
breakdown, and whale/contested nodes. This is the "what I want to see" artifact:
proof that the market's collaborative structure is real and replayable, and the
direct visual test of whether budget flowed to the winning model. The DAG must
be a pure function of the tape (`assert view == derive_from_tape(tape)`).

## 8. FC trace + constitution alignment

- **Art II.2 (price broadcast drives emergence)** — finally implemented at the
  level that matters: budget (the scarce resource) is priced and routed, not
  just node-prices broadcast. H-HET-1's carrier was a half-implementation.
- **Art II.2.1 explore/exploit** — the bandit must not collapse to one model
  (preserve heterogeneity per Art III.3); pin an exploration floor.
- **Art III.3/III.4** — shield horizontal correlation (keep models decorrelated)
  and keep the allocation metric non-gameable.
- **Art 0.2** — model provenance tape-canonical (§8).
- **Art 0.4 PATH DECLARATION (required by architect):** this charter's
  tape/schema commit (the §8 `model_id` change) adopts **Path [A/B/C — TO BE
  DECLARED at commit]** for the `Q_t=⟨q_t,HEAD_t,tape_t⟩` version-control
  substrate. `model_id` is local hemostasis; it does NOT close the Art 0.4 debt
  (HEAD_t unimplemented, tape_t partial, no runtime git). The commit message
  must state which path and must not silently lower fidelity (sudo-only).
- FC1 N7 (predicate verify) / N11 (output-evidence) via the Lean judge; FC1 N5
  (broadcast injection) via the priced budget signal.

## 9. Predecessor / dependency

1. **§8 `ProposalTelemetry.model_id` (schema v2 + legacy v1 decoder + historical
   replay-equivalence test + manifest/hash + Veto-AI PASS) MUST land first**
   (hard gate 5.1). APPROVED 2026-06-15; implementation queued after the serial
   PPUT batch completes (no carrier-source edits during an active batch).
2. The freeze branch `claude/het-carrier-freeze` (`f73163f4`) may be pushed; it
   must NOT be merged to the main experiment path until: schema v2 landed +
   legacy replay pass + old 45 cells replay-clean + smoke replay-clean +
   Veto-AI PASS + this charter frozen (architect ruling #3).
3. New carrier routing logic is a Class 2-3 change to `lean_market_agent.rs`
   (model-budget allocation) — gate-first, real evidence, clean-context audit.

## 10. Done definition

Touched FC nodes + risk class stated; the dynamic-routing carrier passes unit +
constitution gates; a frozen-policy, prereg'd, K≥12 paid run lands replay-clean +
axiom-clean evidence; the tape-reconstructed DAG artifact is produced; primary
metric (union coverage at equal-or-lower budget) computed with Wilcoxon pairing;
clean-context audit (Veto-AI domain {PASS,VETO}) after data lands; §17 honored
(no PROVEN without G1–G6). OBLIGATIONS reconciled.
