# P1 — External-audit VETO + code-correction record (2026-06-02)

Two independent external auditors reviewed the P1 full-audit package (`P1_FULL_AUDIT_PACKAGE_2026-06-02.md`,
binary HEAD `1a812204`). **Both returned VETO on the causal claim** (PASS on engineering, statistics honesty,
and the `lm_ineq1` math). The 2nd report is the authoritative one (architect: "以最后的内容为准"). This record
persists their findings (§17 G5: external audit as an independent artifact, not prose inside the result report)
and maps each to the implemented correction.

## Verdicts (both auditors)

| Dim | Auditor verdict | Core evidence |
|---|---|---|
| **A. Confound-shield integrity** | **FAILED** | `single` is budget-matched (24 proposals) but **not topology-matched** — hardcoded `own_last`, no root-restart/backtrack. Price arms (`market`/`autonomous`) make an extra skeptic (`bear`) LLM call `single` does not → compute asymmetry. |
| **B. autonomous-vs-market contrast** | **FATALLY CONFOUNDED** | "identical except who picks the node" is **false in code**: `market`→`build_prompt` (one parent body+error); `autonomous`→`build_autonomous_prompt` (top-6 **full** bodies+errors + all-node snippets) **in the same proof call**. The crack could be **global failure-context synthesis (~6× richer prompt)**, not free-choice routing. |
| **C. Cracks real?** | **PASS** (sandbox-limited) | `lm_ineq1` (3a²+5b²≥2√15·ab) math sound (SymPy-checked); harness Verified is a real `lean` exit-0; auditors could not independently recompile (no repo/Mathlib in their sandbox) → math PASS + documented-axiom-clean, not an independent kernel rerun. `lm_deriv1` unconfirmed → must not count. |
| **D. Statistics + over-read** | **PASS** | `p_holm=0.7500` reproduced exactly; rarity (1/9–2/9) honestly disclosed; no PROVEN/causal language. ("perfect"→"method correct but underpowered; controls missing.") |
| **E. Replay soundness** | **PASS** (economic) / **INCOMPLETE** (causal-cost) | economic_state replay from L4 sound; but LLM calls, prompt hashes, token counts, route decisions, price snapshots, axiom lists are **not** tape-canonical → causal-cost replay incomplete. |
| **F. Warrant the claim?** | **NO** | Violates the pre-registration: PRIMARY metric requires `market` solves ≥1 AND `parallel`/`shuffled_price`/`no_price` all 0 — but the counted run **omitted those controls**, and the crack was **`autonomous`, not `market`** (market 0/18). Subject mismatch + missing controls. |

**Verdict: VETO** the strong causal + pre-registered-win claim; **PASS** the engineering, statistics honesty,
`lm_ineq1` math, and economic-replay credibility. **Recoverable** via the corrections below.

## The corrections (implemented this session — workflow `wf8a015uq`)

| Auditor finding | Fix | Verified (真题真跑) |
|---|---|---|
| **B** (fatal confound) | **F1 decouple** `autonomous` → 2 LLM calls: Stage-1 routes on a **compact** summary (`build_route_summary`: index/price/conf/error-CLASS/age/hash — no body, no shielded error, no librarian) → parent index; Stage-2 builds the proof via the **one shared** `stage2_proof_prompt` = the *same* `build_prompt` market uses. | `--self-test` → `PROMPT-PARITY-OK sha=94f6153c…` exit 0; `#[test] stage2_prompt_byte_equals_market` PASS. Load-bearing: gate asserts `stage2==market AND stage2!=confound_b_control` (mutation-tested: injecting a leak trips both). Confound B **structurally removed**. |
| **C / E** (axiom not inline) | **F5 inline axiom gate**: `LeanJudge::verify` routes a clean exit-0 compile through `axiom_gate` (`#print axioms <name>`); Verified IFF transitive set ⊆ {propext, Classical.choice, Quot.sound}; fail-closed. | Real Lean v4.24.0: clean→Verified `axioms=[propext]`; `sorry`/`native_decide`→blocked; hand-axiom→`axiom_rejected`. `cargo test --lib judges::lean_judge` 12/0 (0.38s, real Lean). |
| **A.2 / E** (compute asymmetry hidden) | **F2 telemetry**: manifest carries proposal/route/bear LLM-call + token counts; invariant `total_model_tokens == proof+route+bear+completion`; route counted in PPUT denominator (no hidden discount for autonomous). | `#[test] f2_manifest_has_compute_telemetry_fields` PASS; self-test asserts the no-double-count invariant + `route_llm_calls>0` iff autonomous. |
| **A.1** (topology not matched) | **F3 baselines**: `single_restart` (root-restart or own_last), `single_tree_no_price` (any own node, no price), `parallel_restart` (N indep chains + root-restart, no shared price). | 3 `#[test]`s PASS: parse + Bulls-only + select_parent reaches root+own_last, never a non-own node. |
| **F** (controls not run; subject mismatch) | **Re-run design** (P1-RERUN): run the full control family + decoupled `autonomous`; report **two** pre-registrations — PREREG-1 (market-Hayek: market vs {single,parallel,shuffled_price,no_price}) and PREREG-2 (autonomous-freechoice, with the byte-identical condition). Do NOT use the market prereg to back the autonomous result. | Pending the corrected sweep. |

## Constitution

Edits in `src/bin/lean_market_agent.rs` (+1525) + `src/judges/lean_judge.rs` (+324) ONLY. `git diff --name-only`:
no §6 restricted surface. Integer money (every `f64` is routing-temp / time / PPUT-metric / LLM-probability-parse
→ integer percent before stake math; price = integer `RationalPrice` num/den, stakes i64 micro). FC1/FC2/FC3
untouched. Class 2 wire-up. Build green; `cargo test --bin lean_market_agent` 13/0; both workflow adversarial
audits (confound-B, correctness+constitution) PROCEED.

## Honest claim going forward (auditor-recommended, adopted)

NOT *"the market organization solves hard problems a single model cannot, due to loss-bearing price routing."*
The corrected re-run will report what the data warrants under the two pre-registrations + 3-layer stats
(existence / reliability w/ exact CI / causal w/ controls-fail + paired test). If `market` stays 0 and decoupled
`autonomous` cracks are rare/non-significant, the honest verdict is an **existence result for decoupled free-choice
routing**, scoped and non-causal — not a Hayek-price-routing win.
