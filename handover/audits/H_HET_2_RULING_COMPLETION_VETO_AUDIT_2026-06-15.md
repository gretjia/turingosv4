> **Orchestrator disambiguation banner (added post-audit, 2026-06-15 — verdict body below is preserved verbatim).**
> This artifact is a **harness-side, development-time clean-context constitutional audit** —
> i.e. the constitution's "独立的非 Veto-AI 审计者 / independent NON-Veto-AI auditor"
> (`constitution.md:765`; `AGENTS.md §9` "clean-context audit witness"). It reviews a git
> diff before ship and ADOPTS the kernel Veto-AI's output-domain discipline (`{PASS, VETO}`,
> constitutionality-only, no subjective opinions) as its STANDARD.
> It is **NOT** the TuringOS kernel's constitutional **Veto-AI organ (Art V.1.3)** — that organ
> is an in-system FC3 runtime power that emits `VetoDecisionTx` onto the ChainTape to gate
> ArchitectAI's proposed changes (`constitution.md:740-765`, FC3 node `vetoAI`), and it did
> **NOT** execute for this change. The author-agent's self-label "(Art V.1.3)" immediately
> below is therefore a naming error: read it as "modeled on the Art V.1.3 *discipline*," not
> "is the Art V.1.3 *organ*."

# H-HET-2 Ruling-Completion — Clean-Context Veto-AI Constitutional Audit (2026-06-15)

**Role:** clean-context Veto-AI constitutional auditor (Art V.1.3). Verdict domain
EXACTLY `{PASS, VETO}` — no code-style / performance / coverage / architecture
opinions.

**Repo:** `/Users/zephryj/work/turingosv4`
**Branch / HEAD:** `claude/het-carrier-freeze` @ `cd8bbf1f` (confirmed).
**Commit range audited:** `git diff faee8c68~1..cd8bbf1f`
- `faee8c68` — atom1: BudgetAllocationTelemetry generic schema + ruling recorded
- `f897fb1a` — atoms 2-4: generic UCB routing_policy + carrier wire-up + gate 5.4
- `cd8bbf1f` — liveness hygiene + prereg draft + ledger

**Authoritative contract:** `handover/tracer_bullets/H_HET_2_ROUTING_POLICY_RULING_2026-06-15.md`
(`VERIFY_UCB_PRICE_PRIOR_FLOOR_V1`, APPROVED AS AMENDED).

**Files in range (`--name-only`):** OBLIGATIONS.md, genesis_payload.toml,
handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md,
handover/preregistration/H_HET_2_DYNAMIC_MODEL_BUDGET_PREREG_2026-06-15.md,
handover/tracer_bullets/H_HET_2_ROUTING_POLICY_DESIGN_2026-06-15.md,
handover/tracer_bullets/H_HET_2_ROUTING_POLICY_RULING_2026-06-15.md,
scripts/constitution_gates.manifest.toml, src/bin/lean_market_agent.rs,
src/runtime/budget_allocation_telemetry.rs, src/runtime/librarian_broadcast.rs,
src/runtime/mod.rs, src/runtime/routing_policy.rs,
tests/constitution_budget_decision_tape_canonical.rs,
tests/fixtures/liveness/production_module_liveness.toml.

---

## FINDINGS (production defects vs test-scaffold gaps)

**Production defects:** NONE.
**Test-scaffold gaps:** NONE blocking. (The full constitution suite carries one
pre-existing red, `script_liveness_inventory` — out of scope, untouched by this
range; documented in §10 below.)

---

## Q1 — Generic-kernel (no Lean in the two new generic files)

`grep -niE 'lean|sorry|tactic|mathlib'` over `src/runtime/routing_policy.rs` +
`src/runtime/budget_allocation_telemetry.rs` returns ONLY three doc-comment lines
that NAME the discipline (routing_policy.rs:6, :9, :10 — "no Lean/sorry/tactic/
mathlib", "carrier bin + LeanJudge supplies the counts"). Strict grep excluding
`//!`/`///`/`//` comments returns EMPTY: no Lean identifier, type, or constant in
code. PASS.

## Q2 — §17.1-G1 recompute-from-tape (gate truly recomputes; tamper caught)

`tests/constitution_budget_decision_tape_canonical.rs`:
- `allocation_recomputes_from_tape` (:101) writes a real
  `BudgetAllocationTelemetry` (built by running the actual
  `routing_policy::score_and_select`, make_record :43), reads it back from CAS,
  reconstructs `ModelInput`s from the recorded candidate rows
  (`inputs_from_record` :27), RE-RUNS `score_and_select`, and asserts the
  recomputed `selected_model_id`, `reason`, and per-model `score_bps` all match
  the loaded tape (:111-126). This is a genuine recompute via the same generic
  mechanism, not a sidecar read or trivially-true assertion. Tape-internal
  invariant `candidate_pull_sum == total_pulls_target_before` also asserted (:128).
- `tampered_selection_is_caught_by_recompute` (:133) flips the recorded winner to
  a different candidate and asserts the recompute DIVERGES (:145-148) — the gate
  can fail on a lying tape. PASS.

## Q3 — §17.3 no name-lie (floored deterministic UCB that DISTRIBUTES; not softmax)

`grep -niE 'softmax|hayek_softmax|priced_softmax|market_softmax'` over the two new
files returns ONLY routing_policy.rs:17 (a doc comment: "the name says UCB, the
code is UCB — not a softmax distributor"). No forbidden mechanism name on any
function/arm/type/const. The mechanism `score_and_select` (routing_policy.rs:154)
is integer UCB (vr_bps Beta(1,1) + bounded cold-start price prior + integer isqrt
count bonus) with a deadline-aware ε exploration floor — it DISTRIBUTES, proven
by `policy_distributes_not_argmax_collapse` (gate :154): 4 equal fresh arms each
owed 1 floor tick over a tight budget → all 4 funded (`funded.len()==4`, :177).
Lib twin `distributes_via_floor_not_argmax_collapse` (routing_policy.rs:390) green.
PASS.

## Q4 — Art III.4 Goodhart shield (router internals NOT in proposer prompt)

The proposer prompt is built by `build_prompt` (lean_market_agent.rs:919) whose
params are `(theorem, parent_body, parent_feedback, librarian, price_context)` —
NO router score/bonus/weight/threshold parameter. The single Stage-2 proposal call
(:2382 `llm.generate`) passes `model: tick_model` (which model is funded) and
`content: prompt` (built by `stage2_proof_prompt` → `build_prompt`, from non-router
inputs) + `sys`. All router internals (`score_and_select`, `routing_cfg`,
`score_bps`/`vr_bps`/`bonus_bps`, `floor_quota`) appear ONLY in the router-state
block + telemetry record (lines 1909-2105) and flow to CAS
(`BudgetAllocationTelemetry`) + input-blob CIDs — audit-visible, never
proposer-visible. `tick_model` reaches only `GenerateRequest.model` and
`ProposalTelemetry.model_id` (:2521). PASS.

## Q5 — §6 restricted surfaces

`git diff faee8c68~1..cd8bbf1f --name-only | grep -E 'kernel.rs|src/bus.rs|
sequencer.rs|typed_tx.rs|wallet.rs|bottom_white|cas/schema.rs'` → NONE of the
§6-listed surfaces touched.

Two trust-root files ARE in the diff and require notice:
- `src/runtime/mod.rs` — additive `pub mod routing_policy;` +
  `pub mod budget_allocation_telemetry;` declarations only (no §6 surface).
- `src/runtime/librarian_broadcast.rs` — added 7 H-HET-2 router schema-ids to the
  known-safe ignore list (+11 lines in `decode_librarian_candidate`); no admission
  / typed-tx / signing / schema change.

Both are pinned in `genesis_payload.toml` and the rehash comments cite the
"architect trust-root standing-rule authorization 2026-06-15 — each
CAS-schema/module addition rehashes its pin per-atom". The ruling doc §
"RoutingPolicyGenesisPin (NEW)" + "BudgetAllocationTelemetry schema ... are new
tape/CAS schema → gate-first, real evidence, Veto-AI, repeating the Art 0.4
Path-B declaration" supplies the per-atom authorization basis cited. No §6
admission/schema/signing rule is modified. PASS (no constitutional violation;
the touched trust-root files carry rehashed pins + cited standing-rule §8 cover).

## Q6 — Art 0.4 + trust-root sha256

`RoutingPolicyGenesisPin.art_0_4_path` declared `"B"`:
routing_policy.rs:279 (field), :481 (boot construction);
gate asserts it (:196, :203). Carrier emits the pin at boot
(lean_market_agent.rs:1936-1944, `art_0_4_path: "B"`).

Recomputed `shasum -a 256` of each pinned/changed file vs `genesis_payload.toml`
— ALL MATCH byte-exactly:

| file | sha256 (HEAD == payload) |
|---|---|
| src/runtime/mod.rs | c22842780e921a93f32428d51639fb08c2b444194d1a9f35de4c392863094d9e |
| src/runtime/librarian_broadcast.rs | 234ddcf6eba3b87b1f5e8261fccc0158ab3314913fa15092cb9eccac7883a457 |
| src/runtime/budget_allocation_telemetry.rs | 93b2fbd569f9c11ea2becd8533e05b1f967a3f4fd074b0c22d04134965db7131 |
| src/runtime/routing_policy.rs | 70642b95ec447cb878988c2cb3232e20e203d3f11c2cd46e392f3be8ee1a70f0 |

No mismatch among the under-audit files. (Pre-existing HEAD mismatches noted in
the ledger — judge.sh / llm_proxy.py — are out of this range's authorship and per
the task brief are NOT a veto basis here.) PASS.

## Q7 — Integer money

`grep -niE '\bf64\b|\bf32\b'` over the two new files: matches are ONLY doc
comments asserting "no f64 on the money/budget path" (telemetry.rs:55,
routing_policy.rs:40, :91, :109). Strict grep excluding comments → EMPTY. The
mechanism is all integer (u32/u64/u128, isqrt Newton's method, integer-rational
ε floor over a common denominator). PASS.

## Q8 — Ruling fidelity (`RoutingPolicyConfig::default()` vs ruling table)

routing_policy.rs:62-79 vs the ruling's "Approved default parameters" + verbatim
ruling block:
- W_VERIFY:W_PRICE = 8:1 → `w_verify: 8, w_price: 1` ✓
- price_component cap = 1250 → `price_component_cap_bps: 1250` ✓
- N_cold = 4 → `n_cold: 4` ✓
- C_UCB = 2500 → `c_ucb_bps: 2500` ✓
- N_hard_fail = 3 → `n_hard_fail: 3` ✓
- ε = min(0.10, 0.40/k) → `eps_cap_num/den = 10/100 = 0.10`,
  `eps_share_num/den = 40/100 → 0.40/k` ✓ (test `eps_floor_matches_ruling`:
  k=4→10%, k=8→5%)
- deterministic tie-break → `tie_break: TieBreak::Lexicographic` ✓
- price clamp [2500,7500] → `price_clamp_lo/hi_bps: 2500/7500` ✓ (ruling
  `clamp(...,2500,7500)`)
- policy_family/version strings match ruling. PASS.

## Q9 — Liveness honesty

`tests/fixtures/liveness/production_module_liveness.toml` (added in cd8bbf1f):
- `dynamic_model_budget_routing`: `classification = "product_workload"`,
  `status = "smoke_only"` (NOT historical_real_world_candidate),
  `real_world_evidence = []`, necessity states broad real-world evidence is
  "intentionally pending the architect-sign-off-gated paid run ...
  smoke_is_not_final_evidence"; closure_action = promote AFTER the paid run lands.
  Honest. ✓
- `het_experiment_probes`: `classification = "dev_only"`, `status = "smoke_only"`,
  excluded from the production no-zombie requirement. Honest. ✓
- Cited smoke_gates all exist as REAL test fns:
  `allocation_recomputes_from_tape`, `policy_distributes_not_argmax_collapse`,
  `genesis_pin_pins_the_frozen_policy` (in gate 5.4);
  `pool_target_reference_bodies_verify_clean`
  (tests/het_probe_pool_reference_bodies_verify.rs),
  `third_bug_first_line_shallow_mislabels_good_proof`
  (tests/het_third_bug_dealign_decisive.rs). ✓
- `git show cd8bbf1f -- src/bin/lean_market_agent.rs`: the ONLY change is moving an
  explanatory comment from between `#[path = "../market_tape_shared.rs"]` and
  `mod market_tape_shared;` to BELOW the `#[path]` attribute. The
  `#[path]`/`#[allow(dead_code)]`/`mod` triple is unchanged in effect — zero
  semantic change (fixes the `every_src_rust_file_is_reachable` path-attr parser).
  ✓
- prereg `H_HET_2_DYNAMIC_MODEL_BUDGET_PREREG_2026-06-15.md`: line 1 "(DRAFT)",
  line 3 "Status: DRAFT — awaiting architect sign-off. NO paid confirmatory run
  is [authorized]". Honest — no paid-run claim. PASS.

---

## Command outputs (run by this auditor)

```
$ git rev-parse --show-toplevel ; git branch --show-current ; git log --oneline -3
/Users/zephryj/work/turingosv4
claude/het-carrier-freeze
cd8bbf1f H-HET-2 ruling completion: liveness hygiene + prereg draft + ledger
f897fb1a H-HET-2 atoms 2-4: generic UCB router + carrier VerifyUcbPriceFloor + gate 5.4
faee8c68 H-HET-2 atom1: BudgetAllocationTelemetry generic schema + routing-policy ruling recorded

$ cargo test --test constitution_budget_decision_tape_canonical
running 5 tests
test tampered_selection_is_caught_by_recompute ... ok
test policy_distributes_not_argmax_collapse ... ok
test allocation_recomputes_from_tape ... ok
test budget_allocation_round_trips_through_cas ... ok
test genesis_pin_pins_the_frozen_policy ... ok
test result: ok. 5 passed; 0 failed

$ cargo test --bin lean_market_agent
test result: ok. 28 passed; 0 failed

$ cargo test -p turingosv4 --lib routing_policy
test result: ok. 9 passed; 0 failed; 845 filtered out

$ cargo test -p turingosv4 --lib budget_allocation_telemetry
test result: ok. 6 passed; 0 failed; 848 filtered out

$ cargo test --test constitution_production_module_liveness
running 12 tests
... (incl. every_src_rust_file_is_reachable_from_a_crate_or_binary_root,
        candidate_groups_have_real_world_chaintape_or_cas_evidence)
test result: ok. 12 passed; 0 failed
```

Manifest registration: gate `constitution_budget_decision_tape_canonical`
registered in `scripts/constitution_gates.manifest.toml` (added 2026-06-15).
Matrix: Art 0.2 H-HET-2 row added to CONSTITUTION_EXECUTION_MATRIX.md (🟢 GREEN,
with an explicit FALSIFIER column).

---

## §10 — Out-of-scope pre-existing condition (not a veto basis)

The full constitution gate suite (`scripts/run_constitution_gates.sh`) carries a
pre-existing red `script_liveness_inventory` (BearTriage untracked script, not
H-HET, untouched by this range; documented in the cd8bbf1f commit + ledger as the
sole remaining out-of-scope red). The targeted constitution gates relevant to this
range (gate 5.4 5/5, production_module_liveness 12/12) are all GREEN. This
pre-existing red is not authored by, and is unrelated to, the audited change.

---

## VERDICT

All nine constitutional questions verified against the repository with cited
file:line evidence and independently re-run gates. No constitutional violation
found: generic-kernel discipline holds (no Lean in src/runtime/ generic files),
§17.1-G1 recompute-from-tape is real and tamper-catching, §17.3 no name-lie
(floored deterministic UCB that distributes), Art III.4 Goodhart shield intact
(router internals tape-only, never in the proposer prompt), no §6 restricted
surface modified (the two touched trust-root files carry rehashed pins + cited
standing-rule §8 cover), Art 0.4 Path-B declared and all four pinned-file sha256
match byte-exactly, integer-only money path, ruling defaults reproduced exactly,
and the liveness/prereg classifications are honest (smoke_only / dev_only / DRAFT,
no paid run claimed).

**VERDICT: PASS**
