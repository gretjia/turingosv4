# Codex Phase A → B Exit Audit (PPUT-CCL arc)
**Date**: 2026-04-26
**Commits**: 2e7f75a / d8950ee / 2a65339 / e94e1b9 / 62c4e14 / 6be6eb4 / 180a300 / 7f4bc0c / a5c78e4 / 30f2a14 / 89994c7 / 90953d6
**Test baseline**: 261 PASS + 29 ignored + 0 failed (cumulative across A0–A7)
**Trust Root**: 30-entry manifest verifies clean
**Prompt size**: 247641 chars

---

Reading prompt from stdin...
OpenAI Codex v0.124.0 (research preview)
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: danger-full-access
reasoning effort: xhigh
reasoning summaries: none
session id: 019dc831-3930-7541-91fa-fad12a5f4783
--------
user
# Codex Phase A → B Exit Audit (PPUT-CCL arc)

**Role**: skeptical adversarial reviewer. Independent of Gemini. Conservative merge rule (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

**Mandate**: Phase A is pre-flight (days 1–3 of the 30-day arc). 8 atoms (A0a–e + A1–A7) must be auditable as a unit before Phase B (kernel instrumentation + PPUT accounting) is authorized to start. PREREG_PPUT_CCL_2026-04-26.md (round-4 PASS/PASS, frozen) + PREREG_AMENDMENT_p0_defer_2026-04-25.md (Trust Root entry 25) are the contracts you're auditing against.

The packet below is self-contained. Read it as a standalone document — your conclusions go to ArchitectAI, who will iterate on CHALLENGE items in the same audit cycle. The Phase A0 exit audit (CHALLENGE/CHALLENGE → 7 fixes) is the precedent for how rigorous to be.

End your audit with:
- **VERDICT**: PASS / CHALLENGE / VETO
- **Conviction**: low / medium / high
- **Recommendation**: PROCEED to Phase B / FIX-THEN-PROCEED / REDESIGN

Cite §/file:line for every finding. Be specific about which atom and which line.

---

# Phase A → B Exit Audit Packet (A8)

**Arc**: PPUT-CCL (`PREREG_PPUT_CCL_2026-04-26.md` round-4 PASS/PASS + amendment).
**Date**: 2026-04-26.
**Authority**: ArchitectAI commit (Art. V.1.2). This packet is the input to dual external audit (Codex + Gemini) per Art. V.1.3 + memory `feedback_dual_audit`. Decision rule: PASS → Phase B (kernel instrumentation) authorized; CHALLENGE → in-cycle fixes; VETO → Phase A redesign.

**FC-trace**: meta-witness across FC1 / FC2 / FC3 (atoms instrument all three subgraphs).

---

## § 1. Phase A scope and atom map

Phase A = pre-flight (days 1–3 of the 30-day arc). Decomposed into 8 atoms post the 2026-04-25 architect FULL PASS rewrite:

- **A0** (a–e): harness modernization. Closed by `62c4e14` (A0e exit audit + 7-item fixes).
- **A1**: PREREG amendment p_0 calibration deferral + Trust Root 24 → 25.
- **A2**: P0a `swarm_N=1` mode + `parse_swarm_condition_n` unit tests.
- **A3**: per-agent `AGENT_MODELS` env var (Phase B+C single-model invariant gate).
- **A4**: decomposed metrics (`hit_max_tx` + `tactic_diversity` + `verifier_wait_ms`).
- **A5**: per-agent budget normalization (`BUDGET_REGIME` + `MAX_TRANSACTIONS`).
- **A6**: per-line FC tagging via structured JSON events (`fc_trace` module).
- **A7**: SiliconFlow heterogeneous-LLM provider plumbing (proxy + 3-key smoke).
- **A8**: this packet — Phase A → B exit audit.

Commit chain (atomic, FC-traced, all under ArchitectAI commit authority — none touched `constitution.md`):

```
2e7f75a  A0a: 4 new harness rules + judge.sh constitution-special-case
d8950ee  A0b: tests/fc_alignment_conformance.rs witness battery
2a65339  A0c: 5 new cases C-071..C-075 sediment 2026-04-25 session decisions
e94e1b9  A0d: TRACE_MATRIX_v2 + Trust Root manifest 20 → 24 (harness in TR)
62c4e14  A0e: Phase A0 exit audit (CHALLENGE/CHALLENGE) + 7-item fixes
6be6eb4  A1:  PREREG amendment defer p_0 calibration + Trust Root 24 → 25
180a300  A2:  P0a swarm_N=1 mode + parse_swarm_condition_n unit tests
7f4bc0c  A3:  per-agent AGENT_MODELS env var (Phase B+C single-model gate)
a5c78e4  A4:  decomposed metrics (hit_max_tx + tactic_diversity + verifier_wait_ms)
30f2a14  A5:  per-agent budget normalization (BUDGET_REGIME + MAX_TRANSACTIONS env vars)
89994c7  A6:  per-line FC tagging via structured JSON events (fc_trace module)
90953d6  A7:  SiliconFlow heterogeneous-LLM provider plumbing (proxy + 3-key smoke)
```

## § 2. Test count and Trust Root deltas

|        | A0a baseline | A0e PASS | A4 land | A5 land | A6 land | A7 land |
|---|---|---|---|---|---|---|
| `cargo test --workspace` PASS | 187 | 204 | 234 | 254 | 261 | 261 |
| ignored | 20 | 29 | 29 | 29 | 29 | 29 |
| failed | 0 | 0 | 0 | 0 | 0 | 0 |
| Trust Root manifest entries | 20 | 24 | 24 | 26 | 27 | 30 |

A7 added no new Rust tests (plumbing + integration gate; acceptance via `scripts/smoke_siliconflow.sh` PASS verified 2026-04-26 04:58 UTC).

## § 3. Per-atom FC-trace map and acceptance evidence

### A0 (harness modernization)
**Closing audit**: `CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md` + `GEMINI_PHASE_A0_EXIT_AUDIT_2026-04-25.md`. Both returned CHALLENGE; 7 fixes landed in `62c4e14`. Final state PASS-equivalent (no open P0).
- A0a (4 rules + judge.sh): R-014 / R-015 / R-018 / R-019 + R-016 fc_trace_in_commit. **FC-trace**: governance instrumentation; not a single FC node.
- A0b (`tests/fc_alignment_conformance.rs`): 17 PASS witnesses + 9 `#[ignore]` stubs. **FC-trace**: meta-witness for FC1 / FC2 / FC3 ↔ Rust symbol mapping.
- A0c (5 cases C-071…C-075): constitution amendment process / Veto-AI scope / ArchitectAI commit authority / FC-first / DO-178C tool qualification. **FC-trace**: Art. V (anchors all FC).
- A0d (`TRACE_MATRIX_v2`): 17 ⚠️ → ✅ (status flips); manifest 20 → 24. **FC-trace**: meta.
- A0e: 7 fixes addressing dual-audit CHALLENGE items.

### A1 (PREREG amendment)
- File: `handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md`.
- Substitutes `p_0 = 0.10` (PREREG § 5.5 ceiling) for the calibration-derived value at every Gate H consumer. Mathematically conservative (strictest plausible bar; no Type-I inflation). Re-calibration conditions in § 3 list 5 items (N-experiments arc complete / swarm_N=1 mode landed / per-agent budget normalization landed / hetero-LLM exp complete / Phase D ArchitectAI runtime exists).
- **FC-trace**: FC1-N12 (∏p ground-truth oracle scope unchanged) + Art. V.1.2 (commit authority) + cases C-073 + C-075.
- Trust Root manifest 24 → 25.

### A2 (`swarm_N=1` mode)
- New `parse_swarm_condition_n` in `experiments/minif2f_v4/src/bin/evaluator.rs` discriminates `n<digits>` from `oneshot` / `hybrid_v1` / malformed. PREREG_AMENDMENT § 3 condition 2 cleared.
- **FC-trace**: FC2-N16 InitAI orchestration entry — discriminates between the two registered InitAI shapes (oneshot vs swarm). FC1-N11 ∏p path is reached only via swarm.
- Tests: 5 unit tests (`oneshot_returns_none` / `n1` / `n8` / `nfoo_rejected` / `n0_rejected`).

### A3 (`AGENT_MODELS` env var)
- New module `experiments/minif2f_v4/src/agent_models.rs`. Pure parser + expander + env-coupled resolver. Heterogeneity gated by `PHASE_D_HETERO_OK=1` — Phase B+C single-model invariant enforced at startup BEFORE any LLM call.
- **FC-trace**: FC1-N7 (δ/AI canonical identity per Agent_i).
- Tests: 11 unit tests (parse / expand / hetero gate / length mismatch).

### A4 (decomposed metrics)
- 3 non-Optional v2 fields on `RunAggregate` + legacy `PputResult`: `hit_max_tx`, `tactic_diversity`, `verifier_wait_ms`. Helper `compute_tactic_diversity`. All 9 `make_pput` call sites pass explicit values.
- **FC-trace**: FC2-N22 (HALT decomposition for `hit_max_tx`) + FC1-N11 (∏p decision diversity for `tactic_diversity`) + FC1-N12 (oracle scope for `verifier_wait_ms`).
- Tests: 5 (`test_a4_decomposed_metrics_round_trip`, `test_a4_tactic_diversity_helper`, `test_a4_verifier_wait_bounded_by_total_wall_time`, `test_a4_emit_max_tx_exhaustion_row`, `test_a4_synthetic_short_circuit_does_not_set_hit_max_tx`).

### A5 (budget regime)
- New module `experiments/minif2f_v4/src/budget_regime.rs`. 4-variant `BudgetRegime` enum: `total_proposal` (default; current behavior preserved bit-for-bit) / `per_agent` (loop bound = base × N) / `token_total` (declared; startup-fatal `UnimplementedRegime`) / `wall_clock` (declared; startup-fatal). 2 new non-Optional v2 fields: `budget_regime` + `budget_max_transactions`.
- `run_swarm` startup: `let max_transactions = 200` → `resolve_budget(n_agents)` with startup-fatal error path.
- **FC-trace**: FC2-N22 (HALT decomposition by budget regime) + FC1-N7 (δ instances determining the per-agent share under PerAgent regime).
- Tests: 16 (15 budget_regime unit + 1 jsonl_schema A5 round-trip).
- PREREG_AMENDMENT § 3 condition 3 cleared.
- Trust Root manifest 25 → 26.

### A6 (FC tracing)
- New module `experiments/minif2f_v4/src/fc_trace.rs`. Pure stdlib (zero new deps). 7-variant `FcId` enum (FC1-N7 / FC1-N11 / FC1-N12 / FC1-E18 / FC2-N20 / FC2-N22 / FC3-N31). `FC_TRACE=1` gate cached in `OnceLock`; `FC_TRACE_FILE=<path>` redirects emit to file.
- 6 wired anchor sites in `run_swarm` + 1 in `run_oneshot`: synthetic short-circuit / mr tick / OMEGA full-proof / OMEGA per-tactic / natural MaxTxExhausted (with budget_regime payload from A5) / oneshot verify bracket.
- **FC-trace**: meta-witness for the 5-step compile loop.
- Tests: 7 (6 unit + 1 end-to-end smoke `tests/fc_trace_smoke.rs` exercising `FC_TRACE=1` in a child process — required because the gate is `OnceLock`-cached).
- Trust Root manifest 26 → 27.
- Resolves TRACE_MATRIX_v2 § 5 item 7.

### A7 (SiliconFlow plumbing)
- `src/drivers/llm_proxy.py` ported from v3 with one load-bearing v4 change: per-provider multi-key round-robin. PROVIDERS map now holds a list of env names per provider; `get_client_round_robin` distributes via `_rr_counters` mod `len(clients)`. `/stats` exposes `per_key_requests` for observability. New `siliconflow:<model>` provider-prefix syntax.
- 3 SiliconFlow keys (primary / secondary / tertiary) split concurrent traffic across separate rate-limit pools — V3L-27 (case C-027) single-key N=30 401/429 collapse mitigation.
- `scripts/smoke_siliconflow.sh` + `_smoke_siliconflow.py`: 3 keys × 1 probe (Qwen2.5-7B-Instruct, max_tokens=8). Verified 2026-04-26: primary 2989ms, secondary 1546ms, tertiary 1549ms; 33+1 tokens; content="ack". Proxy round-robin verified [2,2,2] across 6 calls.
- **FC-trace**: FC1-N7 (δ/AI provider expansion).
- No new Rust tests (integration plumbing).
- Memory: `reference_siliconflow.md` records SiliconFlow as the Phase D heterogeneous lane (NOT a probe-only target) and the context-loss anti-pattern (check `.env` + project files BEFORE asking for credentials).
- Trust Root manifest 27 → 30.

## § 4. Phase B → C exit checklist (from PREREG_AMENDMENT § 4) — Phase A side

The PREREG amendment shifted the Phase B → C gate. From the Phase A perspective, the items it lists are now satisfied:

- ❌ p_0 calibration jsonl frozen (was REQUIRED) → **DEFERRED with substitution per amendment § 2**: `p_0 = 0.10` hardcoded at every Gate H consumer.
- ✅ B1–B7 + B7-extra mode toggle infrastructure complete (pre-Phase A baseline; round-4 PASS/PASS).
- ✅ Phase A0 harness modernization complete (`62c4e14`).
- ✅ Tools qualified per case C-075 (DO-178C tool qualification): `runner.sh`, `compute_p0.py`, evaluator boot enforcement, etc.
- ✅ Trust Root verifies clean (`boot::tests::verify_trust_root_passes_on_intact_repo` PASS at 30-entry manifest).

## § 5. Risks and known limitations entering Phase B

1. **`per_agent` budget regime untested at runtime**. A5 unit tests verify the scaling math (`base × N`) and env-coupled resolver. No live-LLM run with `BUDGET_REGIME=per_agent` has been smoked. Phase B kernel instrumentation will be the first opportunity to observe its behavior on a real problem; defer treatment to PREREG re-calibration if any anomaly surfaces.
2. **FC-trace coverage is sparse**. 6 wired anchor sites cover the HALT decomposition (FC2-N22 in 4 distinct exit paths) and one verify bracket. FC1-N11 ∏p decision diversity, FC1-E18 preserve-Q_t, and FC3-N31 WAL append are NOT yet emitting events — the `FcId` enum reserves the variants but no call site uses them. Phase B+ kernel instrumentation should fill these in as the Phase B emit boundary lands.
3. **SiliconFlow rate-limit at scale**. A7 verified 3 keys responding individually at N=1 concurrency. V3L-27 demonstrates collapse at N=30 single-key. The v4 multi-key round-robin should triple the safe N envelope but the actual sweet spot for our hetero swarm is unmeasured. Phase D heterogeneous-batch design should land a `--max-concurrency` knob (currently `LLM_PROXY_CONCURRENCY=5` env in proxy) tuned per provider.
4. **Heterogeneous swarm = Phase D, not B/C**. Per F-2026-04-25-02 + the `agent_models.rs` `PHASE_D_HETERO_GATE_ENV_VAR` invariant, Phases B and C MUST stay single-model so ablation axes are not confounded. A7's plumbing exists for future Phase D work; Phase B uses the existing `deepseek-v4-flash` thinking-off backbone unchanged.
5. **No FC1-N12 emit in `run_swarm` verify path**. A6 wired FC1-N12 only in `run_oneshot`. The two `verify_omega_detailed` and one `verify_partial` brackets in `run_swarm` still increment `verifier_wait_ms` (A4) but don't emit FC events. Phase B kernel instrumentation should add these (one event per Lean call, with verdict + elapsed_ms).

## § 6. Specific questions for auditors

### Q1 — Atomicity and FC-trace discipline
- (Q1.a) Each of A1–A7 was committed as one atomic commit with `FC-trace: <FC?-N?>` in the commit message, per memory `feedback_fc_first_problem_handling`. Verify by re-reading each commit message. Are any commits missing FC-trace, or claiming an FC node that doesn't match the actual code change?
- (Q1.b) Did any commit touch `constitution.md`? (Per Art. V.1.1 + R-018 BLOCK, this is a violation absent sudo.) Trace each commit's file changes.
- (Q1.c) Are there code changes outside the listed atoms — e.g., drift from un-committed tweaks — that should have been their own atom?

### Q2 — PREREG amendment soundness (A1)
- (Q2.a) Substituting `p_0 = 0.10` at the Gate H consumer (the PREREG § 5.5 ceiling) is described as conservative. Verify: does this introduce any Type-I inflation, multiplicity hazard, or family-construction inconsistency vs the original 576-run calibration protocol?
- (Q2.b) Re-calibration conditions list 5 items (notepad ref + per-atom commits). Does this implicitly couple Phase B → C to Phase D readiness in a way that would block forward progress if Phase D slips?
- (Q2.c) The amendment doc itself is in Trust Root (entry 25). Is the amendment's own SHA-256 referenced anywhere that would prevent a silent re-edit?

### Q3 — Budget regime soundness (A5)
- (Q3.a) `BUDGET_REGIME=per_agent` scales the loop bound as `base × N`. For a swarm at N=8 with `MAX_TRANSACTIONS=200`, the loop runs 1600 iterations — each agent receives ~200 proposals. Does this match the brainstorm § A.3 "fixed proposal budget" intent, or is a per-agent counter (each agent independently capped at 200, regardless of round-robin pickup) more faithful?
- (Q3.b) `token_total` and `wall_clock` regimes are declared startup-fatal `UnimplementedRegime`. Is "fail loud" the right default, or should they fall back to `total_proposal` with a WARN log?
- (Q3.c) The default (env unset) preserves Phase B baseline `total_proposal × 200` bit-for-bit. Verify this is true under all code paths — including the synthetic short-circuit and error/timeout exits.

### Q4 — FC tracing coverage (A6)
- (Q4.a) 6 wired anchor sites cover only FC2-N22 (HALT, 4 paths) + FC2-N20 (mr tick) + FC1-N12 (oneshot verify only). FcId enum has 7 variants but only 3 are emitted. Is the partial coverage acceptable for Phase A exit, or does this block Phase B (where the kernel instrumentation needs the full 5-step compile loop visible)?
- (Q4.b) `OnceLock`-cached gate read means a process started with `FC_TRACE=0` (or unset) ignores any later runtime change. Acceptable for evaluator's one-process-per-problem model, but does it pose a risk for any test or runner that mutates the env mid-process?
- (Q4.c) Hand-rolled JSON encoder vs the `serde_json` already in deps. Was there a real reason to avoid `serde_json::to_string` here, or is this premature dep avoidance?
- (Q4.d) `run_corr_id` format = `condition_problem_id_unix-ms`. `make_pput`'s `run_id` independently re-computes this with its own ts. The two will differ by milliseconds. Is the join semantics for Phase D consumers documented anywhere?

### Q5 — SiliconFlow plumbing (A7)
- (Q5.a) `detect_provider` model-prefix logic: a model id with `/` and not starting with "qwen" routes to `siliconflow`. Edge cases: `openai/gpt-4o`, `Qwen/Qwen2.5-7B-Instruct` (capital Q), `siliconflow:Qwen/...`. Verify the routing matrix is complete.
- (Q5.b) Round-robin counter `_rr_counters[provider]` increments unboundedly. Modulo wrap is at u64 max — practically unreachable, but is there a cleaner pattern (use `itertools.cycle` lazily)?
- (Q5.c) `_per_key_requests[provider]` list is mutated under the same `_rr_lock` as the counter. Is the lock granularity right (per-provider lists could use per-provider locks for higher concurrency)?
- (Q5.d) `LLM_PROXY_CONCURRENCY` defaults to 5. With 3 SF keys, that's 5 concurrent calls split across 3 keys ≈ 1.67 per key. Is this low enough to avoid V3L-27 collapse, or should Phase D recommend `LLM_PROXY_CONCURRENCY=15` (5 per key)?
- (Q5.e) Smoke is a single direct-SDK probe per key — bypasses the proxy. This is intentional (per-key verdict). But should there ALSO be a proxy-routed smoke as a follow-up (to catch routing bugs)?

### Q6 — Trust Root manifest expansion 24 → 30
6 new entries this Phase A: PREREG amendment (A1) + budget_regime.rs (A5) + fc_trace.rs (A6) + llm_proxy.py + smoke_siliconflow.sh + _smoke_siliconflow.py (A7).
- (Q6.a) Are all 6 truly load-bearing? E.g., does tampering with `_smoke_siliconflow.py` actually weaken the constitutional gate, or is it a one-shot acceptance script?
- (Q6.b) `llm_proxy.py` is in Python — Trust Root verifies SHA-256, but does NOT verify Python interpreter behavior. A subtle algorithmic tamper (e.g., always-key-0 instead of round-robin) would change the SHA but a no-op cosmetic edit wouldn't trigger any acceptance test. Is the smoke script's `[2,2,2]` invariant codified anywhere as a recurring conformance test?
- (Q6.c) `.env` (containing live API keys) is gitignored and NOT in Trust Root. Should the absence of `.env` be a Trust Root constraint (boot fails if `.env` is missing OR if it's tracked)?

### Q7 — Phase A → B exit decision
- (Q7.a) Verify `cargo test --workspace` PASS count: 261 / 29 ignored / 0 failed. Spot-check by re-running.
- (Q7.b) Verify `boot::tests::verify_trust_root_passes_on_intact_repo` PASS at 30-entry manifest.
- (Q7.c) Verify `scripts/smoke_siliconflow.sh` PASS (live API; cost ~$0.005).
- (Q7.d) Are there OPEN P0 defects from any earlier Phase A audit (A0e CHALLENGE/CHALLENGE → 7 fixes)? Re-read the A0e audit + verify each of the 7 items closed.
- (Q7.e) Phase B's first sub-atom is "JSONL schema v2 + C_i full-cost aggregator" (notepad). Are there any Phase A artifacts that would BLOCK that scope? E.g., schema fields needed by B1 that aren't yet in `RunAggregate`?

---

End your audit with:
- **VERDICT**: PASS / CHALLENGE / VETO
- **Conviction**: low / medium / high
- **Recommendation**: PROCEED to Phase B (kernel instrumentation + PPUT accounting) / FIX-THEN-PROCEED / REDESIGN

PASS = all 7 atoms cleanly land, no open P0, Phase B prerequisites met. CHALLENGE = correctable in one cycle. VETO = Phase A redesign required.


---

# Appended source files


## handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md

```
# PREREG Amendment — p_0 Calibration Deferral (2026-04-25)

**Authority**: ArchitectAI commit per Art. V.1.2 amendment + case C-073 (non-constitution PREREG amendment within ArchitectAI scope).
**FC-trace**: FC1-N12 (∏p ground-truth oracle scope unchanged) + Art. V.1.2 (commit authority) + C-073 + C-075.
**Predecessor**: `PREREG_PPUT_CCL_2026-04-26.md` § 5.5 (frozen, NOT modified by this amendment — see § 6 below).

---

## § 1. Triggering rationale

PREREG § 5.5 specifies p_0 calibration via 576 paired runs (144 adaptation × 2 seeds × {control, treatment}) with estimated cost "~8 wall-hours, ~$3-5". Empirical observation 2026-04-25 during launched batch (commit 650caf7+ era):

| Item | PREREG estimate | Empirical observation | Multiplier |
|---|---|---|---|
| Per-run wall-clock | ~50s | ~15-30 min average (hard problems hit max_transactions=200; aime_1988_p8 SOLVED at 28 min) | 30-40× |
| Total batch wall-clock | ~8 hours | Realistic 3-7 days (576 runs × 15 min serial; treatment short-circuits halve some) | 9-21× |
| API cost | $3-5 | Still ~$15-20 (DeepSeek-v4-flash thinking-off cheap) | 3-4× |

**The 8-hour estimate was based on "~50s/run chat oneshot" assumption that turned out wrong for swarm n3 condition on the adaptation-144 problem mix.** A 7-day batch is not "overnight"; user (mid-session 2026-04-25) explicitly questioned 576-run necessity given multiple unresolved engineering questions (N-agents → PPUT relationship, swarm_N=1 vs oneshot calibration, ground-truth feedback pipeline, etc.).

## § 2. Amendment

PREREG § 5.5 calibration **DEFERRED** indefinitely with the following operative substitution for Phase B → C transition and Phase E Gate H requirements:

**`p_0` for guardrail purposes**: take the **PREREG § 5.5 ceiling itself = 0.10** as the conservative upper bound. Any artifact j whose `j-RR` regression rate exceeds 0.10 fails Gate H per the original guardrail logic; setting `p_0 = 0.10` (the maximum tolerated value) is the strictest possible substitute when no calibrated tighter value exists. This is mathematically conservative: artifacts must clear the strictest plausible bar, not a narrower data-derived bar.

**`genesis_payload.toml [pput_accounting_0].baseline_regression_rate`**: setting deferred to ArchitectAI commit window. Current value `0.0` is recognized as INVALID PLACEHOLDER (would auto-fail any artifact with any regression). Until calibration runs, **Gate H consumers MUST hardcode `p_0 = 0.10`** at the consumption site, not read from `genesis_payload.toml`.

**`baseline_regression_jsonl_sha256`**: stays empty (calibration jsonl does not exist yet).

## § 3. Conditions for re-calibration

Calibration becomes worthwhile (and the deferral lifted) when ALL of:

1. **N-experiments arc (Phase A-D of new plan, 2026-04-25 N-agents experiments) complete** — answers Q1/Q2/Q3 about N → PPUT, fixes (or rejects) the throttle hypothesis, sediments per-N best practices into evaluator. Without this, calibrating p_0 on a known-suboptimal N=3 swarm is calibrating against a moving baseline.

2. **swarm_N=1 mode landed** (Phase A atom A2) — current `CONDITION=oneshot` is a different code path; PREREG § 5.5 ambiguous about which is the "control".

3. **Per-agent budget normalization landed** (Phase A atom A5) — current `max_transactions=200` is fixed-tx budget; PREREG § 5.5 implicitly assumes tx-budget but doesn't specify; need explicit budget regime declaration for calibration to be reproducible.

4. **Heterogeneous LLM agents experiment complete** (Phase A3.5 / E_hetero) — if hetero finds significant solve_rate uplift, the calibration must be done on the production model mix, not on homo n3 baseline.

5. **Phase D ArchitectAI runtime exists** — calibration is part of Gate H gating Phase E. Doing it before Phase D = calibrating against a counterfactual ArchitectAI that doesn't exist.

When ALL 5 conditions met: re-issue PREREG_AMENDMENT to lift the deferral + trigger the 576-run batch with the (then-current) production mode.

## § 4. Impact on Phase B → C transition

PREREG_PPUT_CCL_2026-04-26 § 5.5 originally listed p_0 calibration as a Phase B prerequisite ("Schedule: Phase B7 mandatory; not deferrable to Phase D"). This amendment **explicitly OVERRIDES that "not deferrable" clause** for the deferral conditions in § 3 above.

Phase B → C exit checklist accordingly:
- ❌ p_0 calibration jsonl frozen (was REQUIRED) → now DEFERRED with substitution per § 2
- ✅ B1-B7 + B7-extra mode toggle infrastructure complete
- ✅ Phase A0 harness modernization complete (post-2026-04-25 governance work)
- ✅ Tools qualified (per case C-075): runner.sh, compute_p0.py, evaluator boot enforcement, etc.
- ✅ Trust Root verifies clean

Phase B → C dual-audit packet (next major milestone) must reference this amendment + show that Phase E Gate H consumer hardcodes `p_0 = 0.10`.

## § 5. What this amendment does NOT change

- **PREREG § 5.5 protocol itself** — the calibration *protocol* (288 control + 288 treatment paired runs, max-over-seeds, etc.) remains the agreed-upon procedure for IF calibration ever runs. Amendment defers the SCHEDULING, not the SCIENCE.
- **PREREG § 1.8 Trust Root composition** — manifest entries unchanged by this amendment (this amendment doc is added per § 7 below).
- **PREREG § 5.4 j-RR ≤ p_0 guardrail logic** — Gate H still uses the guardrail; just the p_0 source changes (hardcoded 0.10 instead of calibrated value).
- **PREREG § 5.6 family total / N_max** — unchanged.
- **All other PREREG § sections** — unchanged.

## § 6. PREREG document treatment

`PREREG_PPUT_CCL_2026-04-26.md` itself is **NOT EDITED** by this amendment. It remains the immutable round-4 frozen pre-registration. This amendment is a separate document referenced from § 5.5 forward via a pointer added to Trust Root manifest.

This pattern is per CLAUDE.md "Common Law": amendments are recorded as separate cases / docs that supersede specific sections, leaving the original frozen for reproducibility. PREREG_PPUT_CCL_2026-04-26.md SHA-256 in Trust Root manifest UNCHANGED.

## § 7. Trust Root impact

Add this amendment doc to genesis_payload.toml [trust_root]:
```
"handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md" = "<sha256>"
```

Manifest size: 24 → 25 entries.

## § 8. Audit requirement

Per case C-073 ArchitectAI commit workflow: this amendment requires dual audit (Codex + Gemini, conservative VETO > CHALLENGE > PASS) before commit lands. Audit packet should specifically test:

- Does the amendment violate any PREREG § 5.5 constraint? (Should not — defer is operationally permitted given § 5.5 ceiling.)
- Does substitution of `p_0 = 0.10` invalidate any Gate H statistical claim? (Should not — strictest plausible bar is conservative; no Type-I inflation.)
- Does deferral leave any phase blocked indefinitely? (Should not — § 3 lists explicit re-calibration conditions; if those never met, Phase E proceeds with the conservative substitution per § 2 final paragraph.)

## § 9. Cross-references

- `PREREG_PPUT_CCL_2026-04-26.md` § 5.5 (the amended section, IMMUTABLE)
- `cases/C-073_architect_ai_commit_authority.yaml` (governance basis)
- `cases/C-075_do_178c_tool_qualification.yaml` (tool-readiness as re-calibration precondition)
- `handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md` (context: cost asymmetry concern)
- `handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md` (context: ground-truth feedback discipline)
- `handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md` (PASS verdict on round-4 batch — but batch was 3-7 days not 8h, motivating this deferral)

```

## experiments/minif2f_v4/src/agent_models.rs

```rust
// Phase A atom A3 — per-agent model assignment (`AGENT_MODELS` env var).
//
// Constitutional anchor: FC1-N7 (δ/AI canonical identity). Each Agent_i in
// the swarm path embodies one δ instance; today every Agent_i shares a
// single global δ pinned by `ACTIVE_MODEL` env var. Phase D introduces a
// heterogeneous swarm where Agent_i may bind a different δ. This module
// is the env-var → per-agent δ resolver.
//
// **Phase B+C invariant** (notepad F-2026-04-25-02 + memory
// `feedback_phased_checkpoint`): Phases B and C MUST stay single-model so
// the ablation axes (Soft Law / Panopticon / Amnesia / Homogeneous /
// Full) are not confounded by model identity. Heterogeneous assignment
// is therefore *gated* by `PHASE_D_HETERO_OK=1` — until the gate is set
// the resolver rejects any non-uniform `AGENT_MODELS` payload at startup,
// before a single LLM call goes out.
//
// Default behavior (env var unset OR empty): broadcast the global model
// (resolved from `ACTIVE_MODEL`) to every agent slot — preserves Phase B
// behavior bit-for-bit.

use std::collections::BTreeSet;
use std::fmt;

/// TRACE_MATRIX FC1-N7: env var name binding the per-Agent_i δ vector.
pub const AGENT_MODELS_ENV_VAR: &str = "AGENT_MODELS";

/// TRACE_MATRIX FC1-N7: Phase D heterogeneity gate. Required for any
/// AGENT_MODELS payload containing ≥2 distinct δ values. Phase B+C
/// invariant: single δ across all Agent_i.
pub const PHASE_D_HETERO_GATE_ENV_VAR: &str = "PHASE_D_HETERO_OK";

/// TRACE_MATRIX FC1-N7: startup-fatal failure modes when the per-agent
/// δ vector cannot be safely resolved. Each variant aborts the run
/// before the first LLM call, preserving budget under misconfiguration.
#[derive(Debug, PartialEq, Eq)]
pub enum AgentModelsError {
    /// `AGENT_MODELS` parsed to N entries but the swarm has M ≠ N agents.
    /// (Length 1 broadcasts; only N>1 mismatches reach this branch.)
    LengthMismatch { provided: usize, expected: usize },
    /// A CSV slot was empty after trim (e.g., `"a,,b"` or `",a"`).
    EmptyEntry { index: usize },
    /// Two or more distinct models were supplied without
    /// `PHASE_D_HETERO_OK=1`. Phase B+C single-model invariant.
    HeterogeneousWithoutGate { distinct: Vec<String> },
}

impl fmt::Display for AgentModelsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LengthMismatch { provided, expected } => write!(
                f,
                "AGENT_MODELS length mismatch: {} provided, {} agents in swarm \
                 (use length 1 to broadcast or length == n_agents for positional)",
                provided, expected
            ),
            Self::EmptyEntry { index } => write!(
                f,
                "AGENT_MODELS entry at index {} is empty (CSV slot blank after trim)",
                index
            ),
            Self::HeterogeneousWithoutGate { distinct } => write!(
                f,
                "AGENT_MODELS contains {} distinct models {:?} but \
                 PHASE_D_HETERO_OK is not set to '1'. Phase B+C ablations \
                 require single-model invariant (notepad F-2026-04-25-02).",
                distinct.len(),
                distinct
            ),
        }
    }
}

impl std::error::Error for AgentModelsError {}

/// TRACE_MATRIX FC1-N7: pure CSV parser for the `AGENT_MODELS` payload.
/// Empty input (env unset or empty string) → empty Vec (caller falls
/// back to broadcasting the global model). No env access — testable
/// without process-global state.
pub fn parse_agent_models(env_str: &str) -> Result<Vec<String>, AgentModelsError> {
    let trimmed = env_str.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    let entries: Vec<String> = trimmed.split(',').map(|s| s.trim().to_string()).collect();
    for (i, e) in entries.iter().enumerate() {
        if e.is_empty() {
            return Err(AgentModelsError::EmptyEntry { index: i });
        }
    }
    Ok(entries)
}

/// TRACE_MATRIX FC1-N7: validator + expander. Maps parsed CSV entries
/// to a per-Agent_i δ vector of length `n_agents`. Pure (no env access).
///
/// - parsed empty → broadcast `global_model` to every agent.
/// - parsed.len() == 1 → broadcast that single model.
/// - parsed.len() == n_agents → positional assignment.
/// - else → `LengthMismatch`.
///
/// Heterogeneity (≥2 distinct models in the resolved vector) requires
/// `hetero_gated == true`; otherwise `HeterogeneousWithoutGate`.
pub fn expand_agent_models(
    parsed: Vec<String>,
    global_model: &str,
    n_agents: usize,
    hetero_gated: bool,
) -> Result<Vec<String>, AgentModelsError> {
    let resolved = if parsed.is_empty() {
        vec![global_model.to_string(); n_agents]
    } else if parsed.len() == 1 {
        vec![parsed.into_iter().next().unwrap(); n_agents]
    } else if parsed.len() == n_agents {
        parsed
    } else {
        return Err(AgentModelsError::LengthMismatch {
            provided: parsed.len(),
            expected: n_agents,
        });
    };

    let distinct: BTreeSet<&str> = resolved.iter().map(String::as_str).collect();
    if distinct.len() > 1 && !hetero_gated {
        return Err(AgentModelsError::HeterogeneousWithoutGate {
            distinct: distinct.into_iter().map(String::from).collect(),
        });
    }
    Ok(resolved)
}

/// TRACE_MATRIX FC1-N7: env-coupled wrapper used by `run_swarm` to
/// produce the per-Agent_i δ vector. Composes `parse_agent_models` +
/// `expand_agent_models`; reads `AGENT_MODELS` and the Phase D
/// heterogeneity gate from process env.
pub fn resolve_agent_models(
    global_model: &str,
    n_agents: usize,
) -> Result<Vec<String>, AgentModelsError> {
    let raw = std::env::var(AGENT_MODELS_ENV_VAR).unwrap_or_default();
    let hetero_gated =
        std::env::var(PHASE_D_HETERO_GATE_ENV_VAR).as_deref() == Ok("1");
    let parsed = parse_agent_models(&raw)?;
    expand_agent_models(parsed, global_model, n_agents, hetero_gated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_env_parses_to_empty_vec() {
        assert_eq!(parse_agent_models("").unwrap(), Vec::<String>::new());
        assert_eq!(parse_agent_models("   ").unwrap(), Vec::<String>::new());
    }

    #[test]
    fn single_entry_parses() {
        assert_eq!(
            parse_agent_models("deepseek-v4-flash").unwrap(),
            vec!["deepseek-v4-flash".to_string()]
        );
    }

    #[test]
    fn csv_entries_trimmed() {
        assert_eq!(
            parse_agent_models("a, b ,c").unwrap(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }

    #[test]
    fn empty_csv_slot_rejected() {
        assert_eq!(
            parse_agent_models("a,,b"),
            Err(AgentModelsError::EmptyEntry { index: 1 })
        );
        assert_eq!(
            parse_agent_models(",a"),
            Err(AgentModelsError::EmptyEntry { index: 0 })
        );
        assert_eq!(
            parse_agent_models("a,"),
            Err(AgentModelsError::EmptyEntry { index: 1 })
        );
    }

    #[test]
    fn empty_parsed_broadcasts_global_model() {
        let v = expand_agent_models(vec![], "deepseek-v4-flash", 3, false).unwrap();
        assert_eq!(
            v,
            vec![
                "deepseek-v4-flash".to_string(),
                "deepseek-v4-flash".to_string(),
                "deepseek-v4-flash".to_string()
            ]
        );
    }

    #[test]
    fn single_entry_broadcasts() {
        let v = expand_agent_models(
            vec!["x".to_string()],
            "deepseek-v4-flash",
            4,
            false,
        )
        .unwrap();
        assert_eq!(v, vec!["x".to_string(); 4]);
    }

    #[test]
    fn positional_length_match_passes() {
        let v = expand_agent_models(
            vec!["a".into(), "a".into(), "a".into()],
            "fallback",
            3,
            false,
        )
        .unwrap();
        assert_eq!(v, vec!["a".to_string(); 3]);
    }

    #[test]
    fn length_mismatch_rejected() {
        assert_eq!(
            expand_agent_models(
                vec!["a".into(), "b".into()],
                "g",
                3,
                true,
            ),
            Err(AgentModelsError::LengthMismatch { provided: 2, expected: 3 })
        );
    }

    #[test]
    fn heterogeneous_without_gate_rejected() {
        let err = expand_agent_models(
            vec!["a".into(), "b".into(), "a".into()],
            "g",
            3,
            false,
        )
        .unwrap_err();
        match err {
            AgentModelsError::HeterogeneousWithoutGate { distinct } => {
                assert_eq!(distinct, vec!["a".to_string(), "b".to_string()]);
            }
            other => panic!("expected HeterogeneousWithoutGate, got {:?}", other),
        }
    }

    #[test]
    fn heterogeneous_with_gate_passes() {
        let v = expand_agent_models(
            vec!["a".into(), "b".into(), "a".into()],
            "g",
            3,
            true,
        )
        .unwrap();
        assert_eq!(v, vec!["a".to_string(), "b".to_string(), "a".to_string()]);
    }

    #[test]
    fn uniform_length_n_does_not_trip_hetero_gate() {
        // Length-N positional payload that happens to be uniform must
        // pass without the gate — only *distinct* values trigger it.
        let v = expand_agent_models(
            vec!["a".into(), "a".into(), "a".into()],
            "g",
            3,
            false,
        )
        .unwrap();
        assert_eq!(v, vec!["a".to_string(); 3]);
    }
}

```

## experiments/minif2f_v4/src/budget_regime.rs

```rust
// Phase A atom A5 — explicit per-agent budget regime (`BUDGET_REGIME` +
// `MAX_TRANSACTIONS` env vars).
//
// Constitutional anchor: FC2-N22 (HALT node, `MaxTxExhausted` variant).
// The transaction loop in `run_swarm` terminates on either OMEGA accept
// (FC1-E17) or budget exhaustion (FC2-N22 / `MaxTxExhausted`). The budget
// regime declares HOW the run-level budget is partitioned across the N
// δ instances (Agent_i, FC1-N7) so that PPUT comparisons across N values
// answer a well-posed question.
//
// Codex / Gemini N-agents brainstorm 2026-04-25 § A.3 frames four regimes:
//   - fixed transaction budget         (tx=200 for all N)
//   - fixed proposal / N×tx budget     (each agent gets tx=base proposals)
//   - fixed token budget               (cap on total LLM tokens)
//   - fixed wall-clock budget          (cap on T_i)
//
// In our codebase the inner loop (`for tx in 0..max_transactions`) invokes
// exactly ONE agent per `tx` (Boltzmann-routed). So "tx" already counts
// proposals, and the brainstorm's "fixed transaction budget" maps to
// `total_proposal` (loop bound = base, regardless of N — current
// behavior). The brainstorm's "N × tx = constant" is orthogonal: we want
// the loop bound to scale with N so each agent receives the same number
// of proposals — that's `per_agent`.
//
// PREREG_AMENDMENT_p0_defer § 3 condition 3 (2026-04-25) names this atom
// as a re-calibration prerequisite: "current max_transactions=200 is
// fixed-tx budget; PREREG § 5.5 implicitly assumes tx-budget but doesn't
// specify; need explicit budget regime declaration for calibration to be
// reproducible." A5 satisfies that by stamping the regime + base budget
// on every emitted v2 row.
//
// Phase A scope: implement `total_proposal` + `per_agent` (the two
// regimes that fall out of the existing tx loop). `token_total` /
// `wall_clock` require new exit conditions (cost / clock thresholds) and
// are declared startup-fatal `UnimplementedRegime` so a misconfigured
// `BUDGET_REGIME=token_total` aborts before burning LLM budget. These
// land in a later atom once the cost/clock exit machinery exists.

use std::fmt;

/// TRACE_MATRIX FC2-N22: env var selecting how the run-level transaction
/// budget partitions across N δ agents. Default (unset/empty) =
/// `total_proposal`, preserving Phase B baseline behavior bit-for-bit.
pub const BUDGET_REGIME_ENV_VAR: &str = "BUDGET_REGIME";

/// TRACE_MATRIX FC2-N22: env var setting the base transaction budget.
/// The effective loop bound is `effective_max_tx(regime, base, N)`.
/// Default 200 (Phase B baseline).
pub const MAX_TRANSACTIONS_ENV_VAR: &str = "MAX_TRANSACTIONS";

/// Default base budget when `MAX_TRANSACTIONS` env is unset.
/// Preserves the long-standing `let max_transactions = 200` baseline.
pub const DEFAULT_MAX_TRANSACTIONS: u32 = 200;

/// TRACE_MATRIX FC2-N22: budget regime variants. The first two are
/// implemented in Phase A; the latter two are declared so a downstream
/// run that wants them aborts at startup (UnimplementedRegime) instead
/// of silently falling back and burning budget under the wrong regime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetRegime {
    /// Loop bound = `base`, regardless of N. Each agent ends up with
    /// roughly `base / N` proposals. **Phase B baseline + default.**
    /// Brainstorm § A.3 "fixed transaction budget".
    TotalProposal,
    /// Loop bound = `base × N`. Each agent receives `base` proposals
    /// regardless of swarm size. Brainstorm § A.3 "N × tx = constant"
    /// reframed as "constant per-agent".
    PerAgent,
    /// Cap total LLM tokens (declared, not yet implemented). Requires a
    /// new exit condition tied to `RunCostAccumulator` thresholds.
    TokenTotal,
    /// Cap wall-clock T_i (declared, not yet implemented). Requires a
    /// new exit condition tied to `RunWallClock`.
    WallClock,
}

impl BudgetRegime {
    /// Stable string label stamped on jsonl rows. Stable across releases;
    /// downstream PPUT analysis joins on this exact string.
    pub fn label(&self) -> &'static str {
        match self {
            BudgetRegime::TotalProposal => "total_proposal",
            BudgetRegime::PerAgent => "per_agent",
            BudgetRegime::TokenTotal => "token_total",
            BudgetRegime::WallClock => "wall_clock",
        }
    }
}

/// TRACE_MATRIX FC2-N22: startup-fatal failure modes for the regime
/// resolver. Each variant aborts before the first LLM call so a
/// misconfigured run cannot consume API budget.
#[derive(Debug, PartialEq, Eq)]
pub enum BudgetError {
    /// `BUDGET_REGIME` value not in
    /// {`total_proposal`, `per_agent`, `token_total`, `wall_clock`}.
    UnknownRegime(String),
    /// `MAX_TRANSACTIONS` not parseable as positive u32.
    InvalidMaxTransactions(String),
    /// Caller asked for a regime whose exit machinery is not yet wired
    /// (`token_total` / `wall_clock`). Carries the requested variant so
    /// the startup error names what is missing.
    UnimplementedRegime(BudgetRegime),
    /// Effective loop bound would overflow u32 (`base × N > u32::MAX`).
    /// Realistically unreachable (would require base × N ≥ 2^32) but
    /// expressed in the type so the callers cannot panic on overflow.
    EffectiveBudgetOverflow { base: u32, n_agents: usize },
}

impl fmt::Display for BudgetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownRegime(s) => write!(
                f,
                "BUDGET_REGIME='{}' is not a known regime \
                 (expected total_proposal | per_agent | token_total | wall_clock)",
                s
            ),
            Self::InvalidMaxTransactions(s) => write!(
                f,
                "MAX_TRANSACTIONS='{}' is not a positive integer",
                s
            ),
            Self::UnimplementedRegime(r) => write!(
                f,
                "BUDGET_REGIME='{}' declared but its exit machinery is not yet \
                 implemented (Phase A scope = total_proposal + per_agent only). \
                 Aborting startup to avoid silent fallback under a different regime.",
                r.label()
            ),
            Self::EffectiveBudgetOverflow { base, n_agents } => write!(
                f,
                "effective_max_tx overflow: base={} × n_agents={} exceeds u32::MAX",
                base, n_agents
            ),
        }
    }
}

impl std::error::Error for BudgetError {}

/// TRACE_MATRIX FC2-N22: pure parser for the `BUDGET_REGIME` env value.
/// Empty (unset / blank-after-trim) → default `TotalProposal`. No env
/// access — testable without process-global state.
pub fn parse_budget_regime(env_str: &str) -> Result<BudgetRegime, BudgetError> {
    let trimmed = env_str.trim();
    if trimmed.is_empty() {
        return Ok(BudgetRegime::TotalProposal);
    }
    match trimmed {
        "total_proposal" => Ok(BudgetRegime::TotalProposal),
        "per_agent" => Ok(BudgetRegime::PerAgent),
        "token_total" => Ok(BudgetRegime::TokenTotal),
        "wall_clock" => Ok(BudgetRegime::WallClock),
        other => Err(BudgetError::UnknownRegime(other.to_string())),
    }
}

/// TRACE_MATRIX FC2-N22: pure parser for the `MAX_TRANSACTIONS` env
/// value. Empty (unset / blank-after-trim) → default
/// `DEFAULT_MAX_TRANSACTIONS`. Pure (no env access).
pub fn parse_max_transactions(env_str: &str) -> Result<u32, BudgetError> {
    let trimmed = env_str.trim();
    if trimmed.is_empty() {
        return Ok(DEFAULT_MAX_TRANSACTIONS);
    }
    match trimmed.parse::<u32>() {
        Ok(0) => Err(BudgetError::InvalidMaxTransactions(trimmed.to_string())),
        Ok(v) => Ok(v),
        Err(_) => Err(BudgetError::InvalidMaxTransactions(trimmed.to_string())),
    }
}

/// TRACE_MATRIX FC2-N22: scale the base budget by the regime + swarm
/// size. Pure. Returns the loop bound (`for tx in 0..effective_max_tx`).
///
/// - TotalProposal → base
/// - PerAgent      → base × n_agents (overflow-checked)
/// - TokenTotal / WallClock → UnimplementedRegime (Phase A scope)
///
/// `n_agents == 0` is rejected upstream (run_swarm precondition); we
/// pass it through here to stay pure but the multiplication is safe
/// (`base × 0 = 0`, which fails the `for tx in 0..0` loop fast).
pub fn effective_max_tx(
    regime: BudgetRegime,
    base: u32,
    n_agents: usize,
) -> Result<u32, BudgetError> {
    match regime {
        BudgetRegime::TotalProposal => Ok(base),
        BudgetRegime::PerAgent => {
            let n = n_agents as u64;
            let prod = (base as u64).saturating_mul(n);
            if prod > u32::MAX as u64 {
                return Err(BudgetError::EffectiveBudgetOverflow { base, n_agents });
            }
            Ok(prod as u32)
        }
        BudgetRegime::TokenTotal | BudgetRegime::WallClock => {
            Err(BudgetError::UnimplementedRegime(regime))
        }
    }
}

/// TRACE_MATRIX FC2-N22: env-coupled resolver invoked once at run_swarm
/// startup. Returns `(regime, base_max_tx, effective_max_tx)` so the
/// caller can both run the loop AND stamp the regime + base on the
/// emitted v2 row. Errors abort the run before the first LLM call.
pub fn resolve_budget(n_agents: usize) -> Result<(BudgetRegime, u32, u32), BudgetError> {
    let regime_raw = std::env::var(BUDGET_REGIME_ENV_VAR).unwrap_or_default();
    let base_raw = std::env::var(MAX_TRANSACTIONS_ENV_VAR).unwrap_or_default();
    let regime = parse_budget_regime(&regime_raw)?;
    let base = parse_max_transactions(&base_raw)?;
    let eff = effective_max_tx(regime, base, n_agents)?;
    Ok((regime, base, eff))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Per memory `feedback_env_var_test_lock`: tests that mutate
    // process-global env vars (BUDGET_REGIME / MAX_TRANSACTIONS) must
    // serialise to survive cargo's parallel runner.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    // Phase A atom A5 unit tests. Pure-fn surface first; the env-coupled
    // `resolve_budget` is tested at the bottom under the env mutex.

    #[test]
    fn parse_regime_empty_defaults_to_total_proposal() {
        assert_eq!(parse_budget_regime("").unwrap(), BudgetRegime::TotalProposal);
        assert_eq!(parse_budget_regime("   ").unwrap(), BudgetRegime::TotalProposal);
    }

    #[test]
    fn parse_regime_known_values() {
        assert_eq!(
            parse_budget_regime("total_proposal").unwrap(),
            BudgetRegime::TotalProposal
        );
        assert_eq!(
            parse_budget_regime("per_agent").unwrap(),
            BudgetRegime::PerAgent
        );
        assert_eq!(
            parse_budget_regime("token_total").unwrap(),
            BudgetRegime::TokenTotal
        );
        assert_eq!(
            parse_budget_regime("wall_clock").unwrap(),
            BudgetRegime::WallClock
        );
    }

    #[test]
    fn parse_regime_unknown_rejected() {
        match parse_budget_regime("foobar") {
            Err(BudgetError::UnknownRegime(s)) => assert_eq!(s, "foobar"),
            other => panic!("expected UnknownRegime, got {:?}", other),
        }
    }

    #[test]
    fn parse_max_transactions_empty_defaults_to_200() {
        assert_eq!(parse_max_transactions("").unwrap(), DEFAULT_MAX_TRANSACTIONS);
        assert_eq!(parse_max_transactions("   ").unwrap(), DEFAULT_MAX_TRANSACTIONS);
        assert_eq!(DEFAULT_MAX_TRANSACTIONS, 200);
    }

    #[test]
    fn parse_max_transactions_valid() {
        assert_eq!(parse_max_transactions("50").unwrap(), 50);
        assert_eq!(parse_max_transactions("1000").unwrap(), 1000);
    }

    #[test]
    fn parse_max_transactions_zero_rejected() {
        // 0 would make the loop never enter — almost certainly a config
        // bug, not an intentional zero-iteration request.
        match parse_max_transactions("0") {
            Err(BudgetError::InvalidMaxTransactions(s)) => assert_eq!(s, "0"),
            other => panic!("expected InvalidMaxTransactions, got {:?}", other),
        }
    }

    #[test]
    fn parse_max_transactions_negative_rejected() {
        match parse_max_transactions("-5") {
            Err(BudgetError::InvalidMaxTransactions(s)) => assert_eq!(s, "-5"),
            other => panic!("expected InvalidMaxTransactions, got {:?}", other),
        }
    }

    #[test]
    fn parse_max_transactions_garbage_rejected() {
        match parse_max_transactions("abc") {
            Err(BudgetError::InvalidMaxTransactions(s)) => assert_eq!(s, "abc"),
            other => panic!("expected InvalidMaxTransactions, got {:?}", other),
        }
    }

    #[test]
    fn effective_total_proposal_invariant_under_n() {
        // The defining property of TotalProposal: loop bound is
        // independent of N. This is what makes per-agent invocations
        // ≈ base/N at large N.
        for n in [1, 2, 3, 8, 13, 34usize] {
            assert_eq!(
                effective_max_tx(BudgetRegime::TotalProposal, 200, n).unwrap(),
                200,
                "TotalProposal should not scale with N (n={})", n
            );
        }
    }

    #[test]
    fn effective_per_agent_scales_linearly_in_n() {
        // The defining property of PerAgent: loop bound = base × N.
        for (base, n) in [(200, 1), (200, 8), (50, 13), (100, 3)] {
            let expected = (base as usize * n) as u32;
            assert_eq!(
                effective_max_tx(BudgetRegime::PerAgent, base, n).unwrap(),
                expected,
                "PerAgent should scale linearly (base={}, n={})", base, n
            );
        }
    }

    #[test]
    fn effective_token_total_unimplemented() {
        match effective_max_tx(BudgetRegime::TokenTotal, 200, 8) {
            Err(BudgetError::UnimplementedRegime(BudgetRegime::TokenTotal)) => {}
            other => panic!("expected UnimplementedRegime(TokenTotal), got {:?}", other),
        }
    }

    #[test]
    fn effective_wall_clock_unimplemented() {
        match effective_max_tx(BudgetRegime::WallClock, 200, 8) {
            Err(BudgetError::UnimplementedRegime(BudgetRegime::WallClock)) => {}
            other => panic!("expected UnimplementedRegime(WallClock), got {:?}", other),
        }
    }

    #[test]
    fn effective_per_agent_overflow_rejected() {
        // Construct a base × N that overflows u32. Realistically
        // unreachable (200 × 34 = 6800; the swarm cap is N_max = 34),
        // but the type-level guarantee matters under
        // misconfiguration.
        let huge = u32::MAX;
        match effective_max_tx(BudgetRegime::PerAgent, huge, 2) {
            Err(BudgetError::EffectiveBudgetOverflow { base, n_agents }) => {
                assert_eq!(base, huge);
                assert_eq!(n_agents, 2);
            }
            other => panic!("expected EffectiveBudgetOverflow, got {:?}", other),
        }
    }

    #[test]
    fn label_strings_are_stable() {
        // Downstream PPUT analysis joins on these exact strings;
        // changing them is a breaking change for the v2 schema.
        assert_eq!(BudgetRegime::TotalProposal.label(), "total_proposal");
        assert_eq!(BudgetRegime::PerAgent.label(), "per_agent");
        assert_eq!(BudgetRegime::TokenTotal.label(), "token_total");
        assert_eq!(BudgetRegime::WallClock.label(), "wall_clock");
    }

    #[test]
    fn n_agents_zero_does_not_panic() {
        // run_swarm enforces n_agents >= 1 upstream, but this module is
        // pure and must not panic on 0.
        assert_eq!(
            effective_max_tx(BudgetRegime::TotalProposal, 200, 0).unwrap(),
            200
        );
        assert_eq!(
            effective_max_tx(BudgetRegime::PerAgent, 200, 0).unwrap(),
            0
        );
    }

    /// Env-coupled wrapper round-trip: empty env (default) preserves
    /// the Phase B baseline (TotalProposal × 200).
    #[test]
    fn resolve_budget_default_preserves_phase_b_baseline() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::remove_var(BUDGET_REGIME_ENV_VAR);
        std::env::remove_var(MAX_TRANSACTIONS_ENV_VAR);

        let (regime, base, eff) = resolve_budget(8).unwrap();
        assert_eq!(regime, BudgetRegime::TotalProposal);
        assert_eq!(base, DEFAULT_MAX_TRANSACTIONS);
        assert_eq!(eff, DEFAULT_MAX_TRANSACTIONS);
    }

    /// PerAgent regime via env scales the loop bound linearly in N.
    /// Codex/Gemini brainstorm § A.3 "fixed proposal budget" reframed
    /// as constant per-agent.
    #[test]
    fn resolve_budget_per_agent_via_env() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var(BUDGET_REGIME_ENV_VAR, "per_agent");
        std::env::set_var(MAX_TRANSACTIONS_ENV_VAR, "50");

        let (regime, base, eff) = resolve_budget(8).unwrap();
        assert_eq!(regime, BudgetRegime::PerAgent);
        assert_eq!(base, 50);
        assert_eq!(eff, 400);

        std::env::remove_var(BUDGET_REGIME_ENV_VAR);
        std::env::remove_var(MAX_TRANSACTIONS_ENV_VAR);
    }

    /// Declared-but-unimplemented regime aborts startup so a
    /// misconfigured run cannot silently fall back to a different
    /// regime and burn LLM budget under the wrong partitioning rule.
    #[test]
    fn resolve_budget_token_total_startup_fatal() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var(BUDGET_REGIME_ENV_VAR, "token_total");
        std::env::remove_var(MAX_TRANSACTIONS_ENV_VAR);

        match resolve_budget(3) {
            Err(BudgetError::UnimplementedRegime(BudgetRegime::TokenTotal)) => {}
            other => panic!("expected UnimplementedRegime(TokenTotal), got {:?}", other),
        }

        std::env::remove_var(BUDGET_REGIME_ENV_VAR);
    }

    /// Unknown regime spelling aborts startup with the offending
    /// string surfaced in the error (operator-friendly diagnostic).
    #[test]
    fn resolve_budget_unknown_regime_via_env() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var(BUDGET_REGIME_ENV_VAR, "fixed_tx");

        match resolve_budget(3) {
            Err(BudgetError::UnknownRegime(s)) => assert_eq!(s, "fixed_tx"),
            other => panic!("expected UnknownRegime, got {:?}", other),
        }

        std::env::remove_var(BUDGET_REGIME_ENV_VAR);
    }
}

```

## experiments/minif2f_v4/src/fc_trace.rs

```rust
// Phase A atom A6 — per-line FC tagging via structured JSON events.
//
// Constitutional anchor: meta-witness for FC1 / FC2 / FC3 path coverage.
// Codex / Gemini N-agents brainstorm 2026-04-25 § D ("Constitutional /
// Engineering"): "add OpenTelemetry-style spans or structured JSONL
// events for the constitutional stages". TRACE_MATRIX_v2 § 5 item 7 had
// this deferred to Phase A6 with the note "will land before Phase B
// (homogeneous experiments)".
//
// Why now (Phase A pre-flight, not Phase B): once Phase B starts wiring
// kernel instrumentation (cost / wall-clock / dual PPUT), silent FC
// drift becomes catastrophic — a Soft Law mode that bypasses FC1-N12
// and stamps post-hoc verified=true would *look* like Phase B success.
// FC tracing in pre-flight gives a zero-cost-when-disabled audit trail
// that downstream Phase D ArchitectAI can replay deterministically.
//
// Design constraints:
//   1. Zero new crate dependencies (autonomous-safe; pure stdlib).
//   2. Zero overhead when `FC_TRACE` env is unset (single env::var()
//      check at startup; no per-call var read after).
//   3. Append-only, line-delimited JSON to stderr by default; or to
//      `FC_TRACE_FILE` for batch capture under runner.sh.
//   4. Stable event shape — Phase D consumers MUST be able to join
//      events on `fc_id` + `run_id` + monotonic timestamp without
//      schema knowledge.
//
// Event shape (one JSON object per line):
//   {"ts_ms": 1714123456789, "fc_id": "FC2-N22",
//    "run_id": "n3_mathd_42_169123", "tx": 17,
//    "agent_id": "Agent_2", "kv": {...arbitrary key-value...}}
//
// Phase D+ extension (out-of-scope for A6): convert to true OpenTelemetry
// spans + replace the file sink with a tracing-subscriber bridge. The
// macro surface here was kept small specifically so that swap is local.

use std::io::Write;
use std::sync::OnceLock;

/// TRACE_MATRIX FC-trace meta-witness: canonical FC node identifiers
/// the evaluator emits events for. Adding a new variant is a Phase B+
/// schema change — Phase D ArchitectAI joins on these strings.
///
/// `Display::fmt` produces the dash-separated form used in
/// TRACE_MATRIX rows (e.g. `FC2-N22`, NOT `FC2N22` or `fc2_n22`); that
/// stable string is what flows into the emitted JSON.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FcId {
    /// δ/AI prompt construction (per-Agent_i).
    Fc1N7,
    /// ∏p decision diversity — one event per accepted/rejected proposal
    /// payload (append/complete/step). Distinct vs total flow into the
    /// A4 `tactic_diversity` aggregate.
    Fc1N11,
    /// Lean oracle scope — bracketing every verify_omega /
    /// verify_omega_detailed / verify_partial call. Cumulative
    /// elapsed flows into the A4 `verifier_wait_ms` aggregate.
    Fc1N12,
    /// ∏p=0 → preserve Q_t. Fired when forbidden_pattern matches OR
    /// Lean returns Ok(false).
    Fc1E18,
    /// mr / map-reduce tick. Fired at every `tick_interval` boundary.
    Fc2N20,
    /// HALT — terminal node. `kv.reason` carries
    /// {OmegaAccepted, MaxTxExhausted, ErrorHalt} per FC2-N23.
    Fc2N22,
    /// WAL append (logs subgraph). Fired on every `bus.append_*`
    /// success when WAL_DIR is configured.
    Fc3N31,
}

impl FcId {
    /// Stable string used in emitted JSON. Phase D consumers and
    /// TRACE_MATRIX rows join on this exact spelling.
    pub fn as_str(&self) -> &'static str {
        match self {
            FcId::Fc1N7 => "FC1-N7",
            FcId::Fc1N11 => "FC1-N11",
            FcId::Fc1N12 => "FC1-N12",
            FcId::Fc1E18 => "FC1-E18",
            FcId::Fc2N20 => "FC2-N20",
            FcId::Fc2N22 => "FC2-N22",
            FcId::Fc3N31 => "FC3-N31",
        }
    }
}

impl std::fmt::Display for FcId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// FC_TRACE=1 enables event emission. Read once at first invocation
/// (cached in OnceLock) so subsequent emit calls are a single atomic
/// load. Kept process-global because the evaluator binary is one
/// process per run — no need for finer scoping.
static FC_TRACE_ENABLED: OnceLock<bool> = OnceLock::new();

/// FC_TRACE_FILE=<path> redirects emit to the file (truncate-on-open).
/// Default sink = stderr. Acquired once per process; the file handle
/// is held for the lifetime of the binary.
static FC_TRACE_SINK: OnceLock<std::sync::Mutex<Box<dyn Write + Send>>> =
    OnceLock::new();

const FC_TRACE_ENV_VAR: &str = "FC_TRACE";
const FC_TRACE_FILE_ENV_VAR: &str = "FC_TRACE_FILE";

/// True iff `FC_TRACE` env var is set to `1` at first call. Cheap to
/// call repeatedly (single OnceLock read).
pub fn fc_trace_enabled() -> bool {
    *FC_TRACE_ENABLED.get_or_init(|| {
        std::env::var(FC_TRACE_ENV_VAR).as_deref() == Ok("1")
    })
}

/// Emit one event line. Caller passes the FC node id + a slice of
/// (key, JSON-fragment) pairs. Caller is responsible for JSON-encoding
/// the values (use `json_str` for strings; integers / booleans as-is).
///
/// Events are skipped when `FC_TRACE` is unset — this is the cold path
/// in production runs.
///
/// `run_id` and `tx` are passed positionally so the macro stays
/// readable at the call site; future Phase D+ may promote them to a
/// thread-local context. `agent_id` is `None` for run-level events
/// (boot, halt, mr tick).
pub fn emit_event(
    fc: FcId,
    run_id: &str,
    tx: Option<u64>,
    agent_id: Option<&str>,
    kv: &[(&str, String)],
) {
    if !fc_trace_enabled() {
        return;
    }
    let ts_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    let mut line = String::with_capacity(128 + 32 * kv.len());
    line.push('{');
    write_kv_unchecked(&mut line, "ts_ms", &ts_ms.to_string(), false);
    line.push(',');
    write_kv_unchecked(&mut line, "fc_id", &json_str(fc.as_str()), false);
    line.push(',');
    write_kv_unchecked(&mut line, "run_id", &json_str(run_id), false);
    if let Some(t) = tx {
        line.push(',');
        write_kv_unchecked(&mut line, "tx", &t.to_string(), false);
    }
    if let Some(a) = agent_id {
        line.push(',');
        write_kv_unchecked(&mut line, "agent_id", &json_str(a), false);
    }
    if !kv.is_empty() {
        line.push_str(r#","kv":{"#);
        for (i, (k, v)) in kv.iter().enumerate() {
            if i > 0 {
                line.push(',');
            }
            write_kv_unchecked(&mut line, k, v, false);
        }
        line.push('}');
    }
    line.push('}');
    line.push('\n');

    // Lock-then-write. The Mutex is per-process; contention is bounded
    // by emit rate (~1-10 events per tx, max_tx ~200, n_agents ~8 →
    // < 16K events per run → negligible).
    let sink = FC_TRACE_SINK.get_or_init(init_sink);
    if let Ok(mut s) = sink.lock() {
        let _ = s.write_all(line.as_bytes());
    }
}

fn init_sink() -> std::sync::Mutex<Box<dyn Write + Send>> {
    let sink: Box<dyn Write + Send> = match std::env::var(FC_TRACE_FILE_ENV_VAR) {
        Ok(path) if !path.is_empty() => match std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
        {
            Ok(f) => Box::new(f),
            Err(_) => Box::new(std::io::stderr()),
        },
        _ => Box::new(std::io::stderr()),
    };
    std::sync::Mutex::new(sink)
}

/// JSON-escape a string with surrounding double quotes. Handles the
/// minimal escape set required by RFC 8259 § 7. Public so callers can
/// pre-encode string values for `kv` slots.
pub fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str(r#"\""#),
            '\\' => out.push_str(r"\\"),
            '\n' => out.push_str(r"\n"),
            '\r' => out.push_str(r"\r"),
            '\t' => out.push_str(r"\t"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn write_kv_unchecked(buf: &mut String, key: &str, value_json: &str, _trailing_comma: bool) {
    buf.push('"');
    buf.push_str(key);
    buf.push_str("\":");
    buf.push_str(value_json);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Serialise env mutation per memory feedback_env_var_test_lock.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn fc_id_strings_are_stable() {
        // Phase D consumers + TRACE_MATRIX rows join on these exact
        // spellings; any rename here is a breaking schema change.
        assert_eq!(FcId::Fc1N7.as_str(), "FC1-N7");
        assert_eq!(FcId::Fc1N11.as_str(), "FC1-N11");
        assert_eq!(FcId::Fc1N12.as_str(), "FC1-N12");
        assert_eq!(FcId::Fc1E18.as_str(), "FC1-E18");
        assert_eq!(FcId::Fc2N20.as_str(), "FC2-N20");
        assert_eq!(FcId::Fc2N22.as_str(), "FC2-N22");
        assert_eq!(FcId::Fc3N31.as_str(), "FC3-N31");
    }

    #[test]
    fn fc_id_display_matches_as_str() {
        assert_eq!(format!("{}", FcId::Fc2N22), "FC2-N22");
    }

    #[test]
    fn json_str_escapes_required_chars() {
        // RFC 8259 § 7 minimal escape set.
        assert_eq!(json_str(r#"a"b"#), r#""a\"b""#);
        assert_eq!(json_str(r"a\b"), r#""a\\b""#);
        assert_eq!(json_str("a\nb"), r#""a\nb""#);
        assert_eq!(json_str("a\tb"), r#""a\tb""#);
        // Control chars get \u escapes
        assert_eq!(json_str("\x01"), r#""\u0001""#);
        assert_eq!(json_str("\x1f"), r#""\u001f""#);
        // Plain ASCII passes through unchanged
        assert_eq!(json_str("simple"), r#""simple""#);
        // Empty
        assert_eq!(json_str(""), r#""""#);
    }

    #[test]
    fn emit_is_no_op_when_disabled() {
        // The `fc_trace_enabled()` cache is process-global and read
        // once. We can't reliably toggle it inside a test (OnceLock
        // semantics), so we exercise the public surface and assume
        // cold-path correctness from the type-level guarantee:
        // emit_event short-circuits before any I/O when the gate is
        // false. In real production the env var is unset and this is
        // the universal case.
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::remove_var(FC_TRACE_ENV_VAR);
        // First call seeds the OnceLock to false.
        let first = fc_trace_enabled();
        // Subsequent calls return the cached value regardless of env.
        std::env::set_var(FC_TRACE_ENV_VAR, "1");
        let second = fc_trace_enabled();
        // Both reads return whatever the OnceLock latched first.
        assert_eq!(first, second);
        std::env::remove_var(FC_TRACE_ENV_VAR);
    }

    #[test]
    fn emit_event_with_no_kv_or_agent_does_not_panic() {
        // Cold-path smoke: gate is whatever the OnceLock latched,
        // emit_event must not panic on a minimal call shape (run-level
        // event with no agent + empty kv slice).
        emit_event(FcId::Fc2N22, "test_run", None, None, &[]);
    }

    #[test]
    fn emit_event_with_full_payload_does_not_panic() {
        emit_event(
            FcId::Fc1N12,
            "test_run",
            Some(17),
            Some("Agent_2"),
            &[
                ("verdict", "true".to_string()),
                ("elapsed_ms", "523".to_string()),
                ("payload_hash", json_str("deadbeef")),
            ],
        );
    }
}

```

## experiments/minif2f_v4/src/jsonl_schema.rs

```rust
// PPUT-CCL JSONL schema v2 — proposal-level + run-level records.
//
// Authoritative spec:
//   handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md § B1
//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md § 5 (definitions)
//
// Versioning: every v2 record carries `schema_version = "v2.0"`. Legacy Paper-1
// era jsonl rows (the `PputResult` shape emitted by evaluator before this commit)
// have NO `schema_version` field, so `RunRecord::from_json` discriminates on
// presence and routes to `LegacyRunAggregate`. No on-disk artifact is rewritten
// by this commit; downstream tooling is the upgrade boundary.
//
// B1 scope: schema definition + round-trip + legacy-compat + zero-progress
// invariant. B2/B3/B4 wire the new fields into evaluator emission paths.

use serde::{Deserialize, Serialize};

pub const SCHEMA_VERSION_V2: &str = "v2.0";

/// Per-proposal row (one per LLM call / append / complete attempt).
///
/// Currently no evaluator emit path produces these — B2 (cost aggregator) and
/// B3 (wall-time) will add the emit sites. This struct is the contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProposalRow {
    pub run_id: String,
    pub problem_id: String,
    pub agent_id: String,
    pub role: String,
    pub branch_id: String,
    pub proposal_hash: String,
    pub accepted: bool,

    /// "adaptation" | "meta_validation" | "heldout"
    pub split: String,
    pub schema_version: String,
    /// SHA-256 of input prompt (retrieval-equivalence audit).
    pub context_hash: String,
    /// Runtime predicate accept = 1, reject = 0.
    pub predicate_result: i32,
    /// Lean post-hoc verify: 1 / 0 / null = not yet checked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_truth_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lean_error_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_error_hash: Option<String>,
    /// Hash of Q^world snapshot to roll back to (PREREG ArtifactState).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_to: Option<String>,

    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    /// Length of all tool stdout summed (B2).
    pub tool_tokens: u64,
    /// = prompt + completion + tool.
    pub total_tokens: u64,
    pub wall_time_ms: u64,
    /// ISO 8601 UTC.
    pub start_time: String,
    pub end_time: String,
    pub ast_depth: u32,
    pub peer_agents_in_branch: Vec<String>,
    /// SHA-256 of concatenated tool stdout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_stdout_hash: Option<String>,
    pub is_on_golden_path: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub golden_path_id: Option<String>,
    /// Phase D+ meta-loop attribution; nullable in Phase B.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architect_artifact_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auditor_attestation: Option<String>,
}

/// Per-run aggregate row.
///
/// `pput_runtime` = legacy / runtime-accept-based — NEVER the North Star.
/// `pput_verified` = Lean post-hoc verified — H-VPPUT input.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunAggregate {
    pub run_id: String,
    pub problem_id: String,
    pub solved: bool,

    pub schema_version: String,
    pub split: String,
    /// Lean post-hoc PASS (B4).
    pub verified: bool,
    pub golden_path_token_count: u64,
    /// C_i — sum over all proposals (B2).
    pub total_run_token_count: u64,
    /// T_i — wall-clock first-read → final-accept (B3).
    pub total_wall_time_ms: u64,
    /// 0 or 1 (Lean ground truth).
    pub progress: u8,
    /// Runtime/accept-based; may inflate under Soft Law (H1 detection).
    pub pput_runtime: f64,
    /// Verified PPUT — Progress / (C_i × T_i / 1000), units = 1/(token·second).
    pub pput_verified: f64,
    /// 10^6 × pput_verified — display unit (PREREG § 5).
    pub pput_m_verified: f64,
    pub failed_branch_count: u32,
    pub rollback_count: u32,

    /// Phase A atom A4: did the run reach `max_transactions` without OMEGA?
    /// True iff the natural max-tx exhaustion path fired. False on OMEGA
    /// accept, on the B7-extra synthetic short-circuit (which exits
    /// EARLY at the rollback threshold — counted under
    /// `synthetic_short_circuit`, not here), and on oneshot (no max-tx
    /// concept; only one LLM call). Co-reported with `solved` so
    /// downstream analysis can split `(solve_rate)` from `(PPUT on solved)`
    /// per Gemini N-agents brainstorm 2026-04-25 § A.4.
    pub hit_max_tx: bool,
    /// Phase A atom A4: distinct-payload-hash / total-proposal ratio
    /// across every parsed `append`/`complete`/`step` payload in the run.
    /// Range [0.0, 1.0]; 1.0 = every proposal unique; 0 proposals → 0.0
    /// by convention (synthetic / measurement-failure runs). Cheap proxy
    /// for the semantic-diversity metric proposed in the N-agents
    /// brainstorms (Gemini § A "Search Party"); embedding distance is
    /// Phase D+ work.
    pub tactic_diversity: f64,
    /// Phase A atom A4: cumulative wall-clock spent inside Lean verifier
    /// calls (`verify_omega` / `verify_omega_detailed` / `verify_partial`)
    /// across the full run, in milliseconds. By construction
    /// `verifier_wait_ms ≤ total_wall_time_ms`. Enables the Amdahl /
    /// USL decomposition Codex N-agents brainstorm § C proposed
    /// (parallel LLM time vs serial Lean time).
    pub verifier_wait_ms: u64,

    /// Phase A atom A5 (FC2-N22 HALT decomposition): which
    /// budget-regime governed the loop bound for this run. Stable
    /// label string (`total_proposal` | `per_agent` | `token_total` |
    /// `wall_clock`). PREREG_AMENDMENT_p0_defer § 3 condition 3 names
    /// this stamp as a re-calibration prerequisite — without it,
    /// `MaxTxExhausted` rows are ambiguous about which budget
    /// partitioning rule produced them. Oneshot runs (no swarm loop)
    /// stamp `total_proposal` + `budget_max_transactions=1`.
    pub budget_regime: String,
    /// Phase A atom A5: base transaction budget BEFORE the regime's
    /// scaling rule was applied. Under `total_proposal` the loop bound
    /// equals this; under `per_agent` the loop bound = base × n_agents.
    /// Stamping the base (not the effective bound) keeps cross-N
    /// comparisons interpretable in downstream analysis.
    pub budget_max_transactions: u32,

    pub far: f64,
    pub err: f64,
    pub iac: f64,
    pub cpr: f64,

    /// Exact model id + API revision (drift defense per F-2026-04-22-08).
    pub model_snapshot: String,
    pub git_sha: String,
    pub binary_sha256: String,
    /// "full" | "panopticon" | "amnesia" | "soft_law" | "homogeneous".
    pub mode: String,
}

impl RunAggregate {
    /// Compute pput_verified per PREREG § 5:
    ///   pput_verified = progress / (c_i * t_i_ms / 1000)
    /// Returns 0.0 when progress is 0, OR when c_i or t_i_ms is 0
    /// (synthetic / degenerate runs; real runs always have positive cost+time).
    pub fn compute_pput_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
        if progress == 0 || c_i == 0 || t_i_ms == 0 {
            return 0.0;
        }
        let denom = (c_i as f64) * (t_i_ms as f64) / 1000.0;
        (progress as f64) / denom
    }

    /// Display unit: 10^6 × pput_verified.
    pub fn compute_pput_m_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
        1.0e6 * Self::compute_pput_verified(progress, c_i, t_i_ms)
    }
}

/// Phase A atom A4 (FC1-N11 ∏p decision diversity): tactic_diversity
/// = distinct / total. 0 proposals → 0.0 by convention (no signal to
/// report). All-distinct → 1.0; all-identical → 1/total.
pub fn compute_tactic_diversity(distinct_proposals: u64, total_proposals: u64) -> f64 {
    if total_proposals == 0 {
        return 0.0;
    }
    let r = (distinct_proposals as f64) / (total_proposals as f64);
    // distinct must not exceed total (caller bug); clamp to [0, 1].
    r.clamp(0.0, 1.0)
}

/// Legacy v1 run row — mirrors the pre-v2 `PputResult` shape emitted by the
/// evaluator before this commit (Paper 1 era, e.g.
/// `discarded_12way_run_2026-04-24/E1v2_Abl_*.jsonl`).
///
/// All v3-era extension fields (reputation_at_end, halt_reason, gp_*) are
/// captured by `extra` so a legacy line round-trips losslessly through
/// serde_json::Value.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LegacyRunAggregate {
    pub problem: String,
    pub condition: String,
    pub model: String,
    pub has_golden_path: bool,
    pub time_secs: f64,
    pub pput: f64,
    pub gp_token_count: u64,
    pub gp_node_count: usize,
    pub tx_count: u64,
    /// Catch-all for v3.x optional fields (reputation_at_end, halt_reason,
    /// gp_payload, gp_path, gp_proof_file, classifier_version, build_sha, ...).
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Discriminated record for backward-compatible reading.
#[derive(Debug)]
pub enum RunRecord {
    V2(RunAggregate),
    Legacy(LegacyRunAggregate),
}

impl RunRecord {
    /// Parse one jsonl line. v2 if `schema_version` present, else legacy.
    /// Returns the raw serde error for genuinely malformed input.
    pub fn from_json(line: &str) -> Result<Self, serde_json::Error> {
        let v: serde_json::Value = serde_json::from_str(line)?;
        let is_v2 = v.get("schema_version")
            .and_then(|s| s.as_str())
            .map(|s| s.starts_with("v2"))
            .unwrap_or(false);
        if is_v2 {
            Ok(RunRecord::V2(serde_json::from_value(v)?))
        } else {
            Ok(RunRecord::Legacy(serde_json::from_value(v)?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_run() -> RunAggregate {
        RunAggregate {
            run_id: "r-001".into(),
            problem_id: "mathd_algebra_44".into(),
            solved: true,
            schema_version: SCHEMA_VERSION_V2.into(),
            split: "adaptation".into(),
            verified: true,
            golden_path_token_count: 512,
            total_run_token_count: 4096,
            total_wall_time_ms: 12_000,
            progress: 1,
            pput_runtime: 0.5,
            pput_verified: RunAggregate::compute_pput_verified(1, 4096, 12_000),
            pput_m_verified: RunAggregate::compute_pput_m_verified(1, 4096, 12_000),
            failed_branch_count: 3,
            rollback_count: 0,
            hit_max_tx: false,
            tactic_diversity: 1.0,
            verifier_wait_ms: 4_500,
            budget_regime: "total_proposal".into(),
            budget_max_transactions: 200,
            far: 0.0, err: 0.0, iac: 0.0, cpr: 0.0,
            model_snapshot: "deepseek-v4-flash@2026-04-26".into(),
            git_sha: "913255d".into(),
            binary_sha256: "deadbeef".into(),
            mode: "full".into(),
        }
    }

    #[test]
    fn test_jsonl_schema_v2_round_trip() {
        let original = sample_run();
        let line = serde_json::to_string(&original).expect("serialize");
        let parsed: RunAggregate = serde_json::from_str(&line).expect("deserialize");
        assert_eq!(parsed, original, "v2 RunAggregate must round-trip");
        assert!(line.contains("\"schema_version\":\"v2.0\""),
                "serialized line must stamp schema_version");
    }

    #[test]
    fn test_pput_verified_zero_when_progress_zero() {
        // PREREG § 3 anti-Goodhart: a run that did not verify must report
        // pput_verified = 0 regardless of cost / wall-time.
        assert_eq!(RunAggregate::compute_pput_verified(0, 1000, 5000), 0.0);
        assert_eq!(RunAggregate::compute_pput_m_verified(0, 1000, 5000), 0.0);

        // And the struct round-trips with the zero stamped in.
        let mut r = sample_run();
        r.solved = false;
        r.verified = false;
        r.progress = 0;
        r.pput_verified = RunAggregate::compute_pput_verified(0, r.total_run_token_count, r.total_wall_time_ms);
        r.pput_m_verified = RunAggregate::compute_pput_m_verified(0, r.total_run_token_count, r.total_wall_time_ms);
        assert_eq!(r.pput_verified, 0.0);
        assert_eq!(r.pput_m_verified, 0.0);

        // Defensive: degenerate cost/time also clamps to 0 (synthetic test fixtures).
        assert_eq!(RunAggregate::compute_pput_verified(1, 0, 5000), 0.0);
        assert_eq!(RunAggregate::compute_pput_verified(1, 1000, 0), 0.0);
    }

    #[test]
    fn test_legacy_jsonl_still_readable() {
        // Verbatim shape of a Paper-1 era line
        // (discarded_12way_run_2026-04-24/E1v2_Abl_s141421_n8_20260424T080939.jsonl).
        let legacy_line = r#"{"problem":"/tmp/foo.lean","condition":"n8","model":"deepseek-chat","has_golden_path":true,"time_secs":781.99,"pput":0.127,"gp_token_count":769,"gp_node_count":7,"tx_count":16,"build_sha":"61ccc21","classifier_version":"v1_2026-04-16-a","boltzmann_seed":141421,"halt_reason":"OmegaAccepted","reputation_at_end":{"Agent_1":2}}"#;

        match RunRecord::from_json(legacy_line).expect("legacy line parses") {
            RunRecord::Legacy(l) => {
                assert_eq!(l.condition, "n8");
                assert_eq!(l.has_golden_path, true);
                assert_eq!(l.gp_token_count, 769);
                // v3.x extension fields land in `extra`.
                assert_eq!(l.extra.get("halt_reason").and_then(|v| v.as_str()),
                           Some("OmegaAccepted"));
                assert!(l.extra.get("reputation_at_end").is_some());
            }
            RunRecord::V2(_) => panic!("legacy line misclassified as v2"),
        }

        // And a v2 line dispatches the other way.
        let v2_line = serde_json::to_string(&sample_run()).unwrap();
        match RunRecord::from_json(&v2_line).expect("v2 line parses") {
            RunRecord::V2(_) => {}
            RunRecord::Legacy(_) => panic!("v2 line misclassified as legacy"),
        }
    }

    // Phase A atom A4: decomposed metrics (Codex / Gemini N-agents brainstorm).
    // The 3 new fields (hit_max_tx, tactic_diversity, verifier_wait_ms) must
    // round-trip and obey their invariants; every emitted v2 row carries them
    // (they are non-Optional in the schema).

    #[test]
    fn test_a4_decomposed_metrics_round_trip() {
        let mut r = sample_run();
        r.hit_max_tx = true;
        r.tactic_diversity = 0.42;
        r.verifier_wait_ms = 1234;
        let line = serde_json::to_string(&r).unwrap();
        assert!(line.contains("\"hit_max_tx\":true"));
        assert!(line.contains("\"tactic_diversity\":0.42"));
        assert!(line.contains("\"verifier_wait_ms\":1234"));
        let parsed: RunAggregate = serde_json::from_str(&line).unwrap();
        assert_eq!(parsed, r);
    }

    #[test]
    fn test_a4_tactic_diversity_helper() {
        // All-distinct → 1.0
        assert_eq!(compute_tactic_diversity(8, 8), 1.0);
        // All-identical → 1/N
        assert!((compute_tactic_diversity(1, 8) - 0.125).abs() < 1e-12);
        // 0 proposals → 0.0 (no signal)
        assert_eq!(compute_tactic_diversity(0, 0), 0.0);
        assert_eq!(compute_tactic_diversity(0, 5), 0.0);
        // Caller bug (distinct > total) clamps to 1.0, never panics — keeps
        // emit path crash-free under accumulator wiring regression.
        assert_eq!(compute_tactic_diversity(9, 8), 1.0);
    }

    #[test]
    fn test_a5_budget_regime_round_trip() {
        // Phase A atom A5: every emitted v2 row must carry the budget
        // regime + base. The stable string labels and the u32 base
        // both serialize/deserialize cleanly, including the
        // non-default `per_agent` regime that scales with N.
        let mut r = sample_run();
        r.budget_regime = "per_agent".into();
        r.budget_max_transactions = 50;
        let line = serde_json::to_string(&r).unwrap();
        assert!(line.contains("\"budget_regime\":\"per_agent\""));
        assert!(line.contains("\"budget_max_transactions\":50"));
        let parsed: RunAggregate = serde_json::from_str(&line).unwrap();
        assert_eq!(parsed, r);
    }

    #[test]
    fn test_a4_verifier_wait_bounded_by_total_wall_time() {
        // Invariant required at every emit site: verifier wait is a strict
        // sub-interval of total wall time. Test the contract; emit-site
        // wiring is asserted in the conformance battery.
        let r = sample_run();
        assert!(
            r.verifier_wait_ms <= r.total_wall_time_ms,
            "verifier_wait_ms ({}) must be <= total_wall_time_ms ({})",
            r.verifier_wait_ms, r.total_wall_time_ms
        );
    }
}

```

## experiments/minif2f_v4/src/bin/evaluator.rs

```rust
// MiniF2F v4 Evaluator — oneshot and swarm modes
//
// Sole optimization metric: PPUT (Progress Per Unit Time)
//   Progress = 100% if Golden Path exists (OMEGA reached), 0% otherwise
//   PPUT = 100% / time_to_omega (seconds)
//   No GP → PPUT = 0 → problem not worth attacking in current iteration
//
// Constitutional basis: Art. I.1 (boolean predicate), Art. I.2 (statistical signal = PPUT)

use minif2f_v4::lean4_oracle::{Lean4Oracle, PartialVerdict, derive_lean_path, load_problem};
use minif2f_v4::cost_aggregator::RunCostAccumulator;
use minif2f_v4::wall_clock::RunWallClock;
use minif2f_v4::post_hoc_verifier::{
    compute_progress_runtime, compute_progress_verified, compute_pput, compute_pput_m,
};
use turingosv4::bus::{BusConfig, BusResult, TuringBus};
use turingosv4::sdk::error_abstraction::{classify_lean_error, classify_parse_error, CLASSIFIER_VERSION};
use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
use turingosv4::kernel::Kernel;
use turingosv4::sdk::actor::{BoltzmannParams, boltzmann_select_parent};
use turingosv4::sdk::prompt::build_agent_prompt;
use turingosv4::sdk::prompt_guard::assert_no_metric_leak;
use turingosv4::sdk::protocol::parse_agent_output;
use turingosv4::sdk::tools::wallet::WalletTool;
use turingosv4::sdk::tools::search::SearchTool;
use turingosv4::sdk::tools::librarian::LibrarianTool;

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::time::Instant;
use log::{info, warn, error};
use rand::SeedableRng;
use rand::rngs::StdRng;

const DEFAULT_BOLTZMANN_SEED: u64 = 74677;  // same as sample seed (BTC/USD external)

const DEFAULT_MINIF2F_DIR: &str = "/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4";

/// PPUT result for a single problem — the only output that matters.
///
/// Mid-term audit P0-B fix 2026-04-25: this struct now carries every B1
/// `RunAggregate` v2 field as a non-Optional, so emitted jsonl rows are
/// dispatched as `RunRecord::V2` by `RunRecord::from_json` (presence of
/// `schema_version` is the discriminant). Legacy diagnostic fields below
/// are kept as Option/skip-if-None for downstream tooling that already
/// reads them; serde silently drops them when parsing as `RunAggregate`
/// (no `deny_unknown_fields`), so V2-tooling reads the v2 contract while
/// PputResult-tooling sees the full diagnostic envelope.
#[derive(Debug, serde::Serialize)]
struct PputResult {
    // ── B1 RunAggregate v2 schema fields (all REQUIRED — non-Optional) ──
    /// Always "v2.0" — RunRecord::from_json discriminator.
    schema_version: String,
    /// Per-run identifier: condition + problem + timestamp.
    run_id: String,
    /// Problem identifier: theorem stem (basename of .lean without extension).
    problem_id: String,
    /// Legacy "did the run reach OMEGA" boolean (= runtime_accepted in B4 vocab).
    /// B1 v2 mandates this as `solved: bool`.
    solved: bool,
    /// "adaptation" | "meta_validation" | "heldout" — read from SPLIT env;
    /// default "adaptation" with stderr warning per Phase B convention.
    split: String,
    /// B4 dual-PPUT: post-hoc Lean verified result. Phase B == solved.
    verified: bool,
    /// Token count of the winning golden path (0 if no GP).
    golden_path_token_count: u64,
    /// B2 C_i — full-run token cost across all proposals.
    total_run_token_count: u64,
    /// B3 T_i — first agent prompt → final Lean call, in milliseconds.
    total_wall_time_ms: u64,
    /// 0 or 1 — Lean ground truth (= 1 iff runtime_accepted AND post_hoc_verified).
    progress: u8,
    /// B4 dual-PPUT: pput_runtime = progress_runtime / (C_i × T_i / 1000).
    pput_runtime: f64,
    /// B4 dual-PPUT: pput_verified = progress_verified / (C_i × T_i / 1000).
    pput_verified: f64,
    /// 10^6 × pput_verified — display unit per PREREG § 5.
    pput_m_verified: f64,
    /// B2 C_i sub-counter: count of proposals that did NOT verify.
    failed_branch_count: u32,
    /// Phase B always 0; Phase C+ when ArtifactState rollbacks land.
    rollback_count: u32,
    /// Phase A atom A4 (FC2-N22 HALT decomposition): true iff the run
    /// reached `max_transactions` without OMEGA. Distinguishes a real
    /// budget-exhausted run from an OMEGA-accept exit at the same
    /// `tx_count`. False on B7-extra synthetic short-circuit (which
    /// exits EARLY at the rollback threshold; that path is tagged via
    /// `synthetic_short_circuit` instead). False on oneshot (no max-tx
    /// concept). Co-reported with `solved` so analysis can split
    /// `(solve_rate)` from `(PPUT on solved)` per Gemini brainstorm.
    hit_max_tx: bool,
    /// Phase A atom A4 (FC1-N11 ∏p decision diversity): distinct /
    /// total over every parsed proposal payload (append/complete/step)
    /// in the run. 0 proposals → 0.0 by convention.
    tactic_diversity: f64,
    /// Phase A atom A4 (FC1-N12 oracle scope): cumulative wall-clock
    /// inside Lean verifier calls in milliseconds. Strict sub-interval
    /// of `total_wall_time_ms`. Enables Amdahl/USL serial-vs-parallel
    /// decomposition per Codex brainstorm § C.
    verifier_wait_ms: u64,
    /// Phase A atom A5 (FC2-N22 HALT decomposition): label of the
    /// budget regime that governed this run's loop bound. One of
    /// `total_proposal` | `per_agent` | `token_total` | `wall_clock`
    /// (the latter two declared but startup-fatal in Phase A). Required
    /// by PREREG_AMENDMENT_p0_defer § 3 condition 3 to disambiguate
    /// `MaxTxExhausted` rows across N values.
    budget_regime: String,
    /// Phase A atom A5: base transaction budget BEFORE regime scaling.
    /// Under `total_proposal` the effective loop bound = this value;
    /// under `per_agent` = this value × n_agents. Oneshot stamps 1
    /// (single LLM call, no loop concept).
    budget_max_transactions: u32,
    /// FAR guardrail (Phase B not yet computed; emit 0.0 placeholder).
    far: f64,
    /// ERR guardrail (Phase B not yet computed).
    err: f64,
    /// IAC guardrail (Phase B not yet computed).
    iac: f64,
    /// CPR guardrail (Phase B not yet computed).
    cpr: f64,
    /// Exact model id + API revision (drift defense per F-2026-04-22-08).
    model_snapshot: String,
    /// Trust Root provenance — git commit SHA at boot.
    git_sha: String,
    /// Trust Root binary fingerprint — Phase B placeholder; B7 fills.
    binary_sha256: String,
    /// "full" | "panopticon" | "amnesia" | "soft_law" | "homogeneous" — from
    /// MODE env, default "full" Phase B.
    mode: String,

    // ── Legacy diagnostic fields (preserved for downstream tooling) ──
    problem: String,
    condition: String,
    model: String,
    has_golden_path: bool,         // alias of `solved`; legacy field name
    time_secs: f64,                // wall time elapsed (function-entry bracket; legacy)
    pput: f64,                     // 100/time if GP, 0 otherwise (legacy display)
    gp_token_count: u64,           // alias of golden_path_token_count
    gp_node_count: usize,          // nodes on golden path (0 if no GP)
    tx_count: u64,                 // total transactions attempted
    // C-012 provenance: stamp per-row commit SHA + classifier version + RNG seed.
    // All Optional; serialize-skip when None (backward compat with v3.1/v3.2 artifacts).
    #[serde(skip_serializing_if = "Option::is_none")]
    build_sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    classifier_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boltzmann_seed: Option<u64>,
    // C-036 harness telemetry: bypass-detection signals for multi-agent runs.
    // tool_dist: counts per tool ({complete, append, invest, parse_fail, llm_err}).
    //   complete=N append=0 ⇒ tape-bypass (Art. II.1 broadcast unused).
    // unique_payload_ratio: distinct OMEGA payloads / total OMEGA attempts.
    //   <0.30 ⇒ catastrophic agent correlation (F-2026-04-18-01).
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_dist: Option<HashMap<String, u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_payload_ratio: Option<f64>,
    // Phase 0 (C-039 candidate): persisted full proof + path so external verifiers can
    // re-run `lean --stdin` from disk artifacts alone, without trusting in-memory runtime.
    // gp_payload = the exact text fed to oracle.verify_omega_detailed at OMEGA accept.
    // gp_path = "alone" (payload self-contained) or "tape+payload" (Art. IV dual-path 2).
    // gp_proof_file = relative path to the standalone .lean archive (problem + proof).
    #[serde(skip_serializing_if = "Option::is_none")]
    gp_payload: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gp_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gp_proof_file: Option<String>,
    /// PPUT-CCL B7-extra (PREREG § 5.5 calibration treatment): set to
    /// `Some(true)` iff the synthetic rollback short-circuit fired in
    /// this run — i.e. SIMULATE_ROLLBACK_AT_TX_50=1 AND the run reached
    /// `rollback_sim::ROLLBACK_TX_THRESHOLD`. Distinguishes calibration
    /// treatment exits from natural max-tx exhaustions (both stamp the
    /// same legacy halt path; this field is the disambiguator).
    ///
    /// Crucially: when `synthetic_short_circuit == Some(true)`, the run's
    /// `total_run_token_count` (C_i) is **understated** vs a true 150-tx
    /// vetoed loop, because the LLM calls for tx 51-199 never happened.
    /// `compute_p0.py` ignores cost (only joins on SOLVED/UNSOLVED), so
    /// p_0 estimation is unaffected; downstream PPUT analysis on these
    /// rows MUST honor this flag and exclude or specially treat them.
    #[serde(skip_serializing_if = "Option::is_none")]
    synthetic_short_circuit: Option<bool>,
    // Note (mid-term audit P0-B fix 2026-04-25): the prior Option versions of
    // total_run_token_count / failed_branch_count / total_wall_time_ms /
    // verified / pput_runtime / pput_verified / pput_m_verified were promoted
    // to non-Optional v2 fields above. Phase B always has values for them.
}

#[tokio::main]
async fn main() {
    env_logger::init();

    // Audit-fix 2026-04-25 (Codex B1 + Q2 — both auditors flagged): the
    // production batch runs *this* binary, not `src/main.rs`. Without a
    // verify_trust_root call here, the FC3-S3 readonly subgraph + FC2-N16
    // InitAI Trust Root enforcement does NOT actually fire on the calibration
    // batch. Boot must happen here, at the production entry point, before
    // any LLM call or jsonl emit.
    //
    // Repo root: CARGO_MANIFEST_DIR is `experiments/minif2f_v4`; repo root
    // is two levels up. canonicalize so a deployed binary still resolves
    // the genesis path it was built against.
    let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("evaluator: repo root resolves at build time");
    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
        // FC3-E14 immediate-abort variant. See OBS_BOOT_FAIL_NOT_HALT.
        panic!("TRUST_ROOT_TAMPERED at evaluator boot: {e}");
    }

    // Step-B v3 treatment binary: stamp classifier version in every emitted PputResult.
    // Control binary (main branch) has no such set_var → classifier_version serializes as None.
    // This makes it impossible to mistake one binary for the other in post-hoc analysis.
    std::env::set_var("CLASSIFIER_VERSION", CLASSIFIER_VERSION);

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: evaluator <problem_file.lean>");
        eprintln!("  CONDITION env: oneshot|n1|n3 (default: oneshot)");
        eprintln!("  MINIF2F_DIR, LLM_PROXY_URL, ACTIVE_MODEL env vars");
        std::process::exit(1);
    }

    let problem_file = &args[1];
    let condition = std::env::var("CONDITION").unwrap_or_else(|_| "oneshot".into());
    let minif2f_dir = std::env::var("MINIF2F_DIR").unwrap_or_else(|_| DEFAULT_MINIF2F_DIR.into());
    let proxy_url = std::env::var("LLM_PROXY_URL").unwrap_or_else(|_| "http://localhost:8080".into());
    // A0e-fix 2026-04-25 (Codex finding 3 + R-019): canonical name per
    // PREREG § 1.8. Was "deepseek-reasoner" (deprecated alias). Phase B+C
    // pinned model = deepseek-v4-flash thinking-off backend.
    // FC-trace: FC1-N7 (δ/AI canonical identity) + memory project_deepseek_drift_2026-04-24.
    let model = std::env::var("ACTIVE_MODEL").unwrap_or_else(|_| "deepseek-v4-flash".into());

    // Resolve problem path
    let problem_path = resolve_problem_path(problem_file, &minif2f_dir);
    let (problem_statement, theorem_name) = match load_problem(&problem_path) {
        Ok(v) => v,
        Err(e) => { eprintln!("Failed to load: {}", e); std::process::exit(1); }
    };

    let lean_path = derive_lean_path(&minif2f_dir);
    info!("Problem: {} | Condition: {} | Model: {}", problem_file, condition, model);

    let result = match condition.as_str() {
        "oneshot" => {
            run_oneshot(problem_file, &problem_statement, &theorem_name,
                       &lean_path, &proxy_url, &model).await
        }
        // Generic nN: parse any "n<digits>" → run_swarm with N agents.
        // Supports N-scaling experiment (percolation curve mapping).
        // **swarm_N=1** (CONDITION=n1) is the critical baseline for the
        // 2026-04-25 N-experiments arc: same code path as n3/n8 swarm
        // but with a single agent. NOT the same as `oneshot` (which
        // skips the swarm loop, tape, mr ticks, ∏p product, etc.).
        // Per Plan-agent NEXT-1 / Codex E0 / Gemini E1-Prime: every
        // N-curve experiment MUST use n1 (not oneshot) as the N=1
        // baseline to avoid code-path confound. Validated by unit
        // test below: parse_swarm_condition_n("n1") == Some(1).
        c if parse_swarm_condition_n(c).is_some() => {
            let n = parse_swarm_condition_n(c).unwrap();
            run_swarm(problem_file, &problem_statement, &theorem_name,
                     &lean_path, &proxy_url, &model, n).await
        }
        "hybrid_v1" => {
            // Mid-term audit P0-D fix 2026-04-25: hybrid_v1 was a Paper 1 era
            // condition that ran run_oneshot, then on failure ran run_swarm,
            // and merged via `..r2` field-spread. Codex flagged that the spread
            // dropped the failed oneshot's C_i (failed_branch_count and
            // total_run_token_count from r1 were silently discarded). PPUT-CCL
            // arc does NOT use hybrid_v1 — it operates exclusively on `oneshot`
            // and `n<N>` conditions per PREREG. Disabling here forces any
            // pipeline that ships a stale hybrid_v1 invocation to surface the
            // deprecation immediately rather than emit a corrupt C_i.
            eprintln!("hybrid_v1 condition is deprecated for PPUT-CCL arc and was \
                       disabled in mid-term audit P0-D fix 2026-04-25. The prior \
                       implementation dropped the failed oneshot leg's C_i via a \
                       `..r2` field-spread, corrupting full-run cost accounting. \
                       Use `oneshot` or `n<N>` instead.");
            std::process::exit(1);
        }
        other => { eprintln!("Unknown condition: {}", other); std::process::exit(1); }
    };

    // Output PPUT result as JSON (machine-readable for batch runner)
    let json = serde_json::to_string(&result).unwrap();
    println!("PPUT_RESULT:{}", json);

    if result.has_golden_path {
        info!("PPUT = {:.2}%/s (GP: {} nodes, {} tokens, {:.1}s)",
              result.pput, result.gp_node_count, result.gp_token_count, result.time_secs);
    } else {
        info!("PPUT = 0 (no golden path in {:.1}s, {} tx)", result.time_secs, result.tx_count);
    }
}

fn resolve_problem_path(problem_file: &str, minif2f_dir: &str) -> String {
    if PathBuf::from(problem_file).exists() {
        return problem_file.to_string();
    }
    let test_path = format!("{}/MiniF2F/Test/{}", minif2f_dir, problem_file);
    if PathBuf::from(&test_path).exists() { return test_path; }
    let valid_path = format!("{}/MiniF2F/Valid/{}", minif2f_dir, problem_file);
    if PathBuf::from(&valid_path).exists() { return valid_path; }
    eprintln!("Problem file not found: {}", problem_file);
    std::process::exit(1);
}

/// Oneshot: single LLM call → verify → PPUT.
async fn run_oneshot(
    problem_file: &str, problem_statement: &str, theorem_name: &str,
    lean_path: &str, proxy_url: &str, model: &str,
) -> PputResult {
    let start = Instant::now();
    let mut acc = RunCostAccumulator::new();
    let mut wc = RunWallClock::new();
    // Phase A atom A4 (FC1-N12 oracle scope): cumulative wall-clock
    // inside Lean for this oneshot run. A single verify_omega call,
    // but bracket so future Phase C Soft Law mode that double-verifies
    // accumulates correctly.
    let mut verifier_wait_ms: u64 = 0;
    // Phase A atom A5 (FC2-N22 budget regime stamp): oneshot has no
    // transaction loop — it issues exactly one LLM call and returns.
    // Stamp `total_proposal` + base=1 so downstream PPUT analysis can
    // join oneshot rows on the same regime axis as swarm rows without
    // a special case. The regime is informational here; no scaling.
    let oneshot_regime = minif2f_v4::budget_regime::BudgetRegime::TotalProposal;
    let oneshot_budget_base: u32 = 1;

    let oracle = Lean4Oracle::new(
        problem_statement.to_string(), theorem_name.to_string(), lean_path.to_string(),
    );

    // PPUT-CCL B3 (mid-term audit P0-C fix 2026-04-25): open the wall-clock
    // bracket BEFORE prompt construction. PREREG § 5 / plan B3 define T_i
    // as "first agent prompt construction → final Lean call". Marking after
    // the construction (prior wiring) under-counted prompt-build time and
    // forced the conformance test to relax its 7100ms assertion.
    wc.mark_first_read();

    // R-22 v2 clause 4 stays reject-only; the prompt must prevent fences at the source.
    // Chat models (deepseek-chat, 2026-04-22) default to ```lean fences; verifier hard-rejects
    // any response containing ``` so the instruction must be explicit. See F-2026-04-22-08.
    let prompt = format!(
        "Complete the following Lean 4 proof. Output ONLY the tactic proof body as raw Lean \
         tokens. DO NOT wrap in markdown code fences (no ```). No prose, no backticks.\n\n{}",
        problem_statement
    );

    let client = ResilientLLMClient::new(proxy_url, 1800, 2);
    // Model-aware max_tokens: deepseek-chat caps at 8192; reasoner needs 16000 for thinking.
    let max_toks = if model.contains("chat") { 8000 } else { 16000 };
    let request = GenerateRequest {
        model: model.to_string(),
        messages: vec![Message { role: "user".into(), content: prompt }],
        temperature: Some(0.2),
        max_tokens: Some(max_toks),
    };

    // PPUT-CCL B6 runtime gate: scan the assembled prompt for PPUT scalars
    // before the call goes out. Any leak aborts deterministically — Goodhart
    // shield at the LLM-call boundary.
    assert_no_metric_leak(&request.messages[0].content);
    match client.generate(&request).await {
        Ok(response) => {
            acc.record_llm_call(response.prompt_tokens, response.completion_tokens);
            acc.record_proposal(false);
            // Rule 22 v2 clause 4: reject markdown fences
            if response.content.contains("```") {
                wc.mark_final_accept();
                // P0-A: caller declares both runtime + post-hoc legs.
                // Fence reject = neither leg fired.
                // A4: no Lean call reached → verifier_wait_ms=0;
                // 1 proposal made (the LLM response), 1 distinct.
                return make_pput(problem_file, "oneshot", model,
                                 false, false, start, 0, 0, 1,
                                 None, None, None, None, None,
                                 Some(acc.total_run_token_count()),
                                 Some(acc.failed_branch_count),
                                 wc.elapsed_ms(),
                                 false, 1, 1, verifier_wait_ms,
                                 oneshot_regime, oneshot_budget_base);
            }

            // Phase A atom A4 (FC1-N12): bracket every Lean call so verifier
            // wait is observable in the emitted v2 row.
            let v_t0 = Instant::now();
            let verdict = oracle.verify_omega(&response.content);
            let v_elapsed = v_t0.elapsed().as_millis() as u64;
            verifier_wait_ms += v_elapsed;
            // A6 FC1-N12 (Lean oracle scope): per-call event with verdict
            // + elapsed_ms. Phase D consumer derives the verifier-cost
            // distribution and the verify-success rate. Run-level emit
            // (no agent_id; oneshot has only one virtual agent).
            let verdict_str = match &verdict {
                Ok(true) => "Ok(true)",
                Ok(false) => "Ok(false)",
                Err(_) => "Err",
            };
            minif2f_v4::fc_trace::emit_event(
                minif2f_v4::fc_trace::FcId::Fc1N12,
                &format!("oneshot_{}", problem_file), None, None,
                &[
                    ("verdict", minif2f_v4::fc_trace::json_str(verdict_str)),
                    ("elapsed_ms", v_elapsed.to_string()),
                ],
            );
            // B3: close the bracket AFTER the Lean call returns, regardless of
            // verdict. Soft Law mode (Phase C) cannot escape the verify-time
            // accounting by short-circuiting on runtime accept.
            wc.mark_final_accept();
            match verdict {
                Ok(true) => {
                    acc.flip_last_failed_to_accepted();
                    let gp_tokens = response.completion_tokens as u64;
                    let preview: String = response.content.chars().take(500).collect();
                    info!(">>> OMEGA ACCEPTED <<< (path=alone, payload[0..500]={:?})", preview);
                    let proof_file = persist_proof_artifact(
                        problem_file, theorem_name, problem_statement,
                        &response.content, "alone", "oneshot",
                    );
                    // P0-A: Phase B oneshot success — runtime gate IS the
                    // Lean verify call (oracle.verify_omega returned Ok(true)),
                    // so both legs hold. Phase C Soft Law would inject a
                    // separate `verify_post_hoc(&oracle, &response.content)`
                    // call here and pass its result as post_hoc_verified.
                    make_pput(problem_file, "oneshot", model,
                              true, true, start, gp_tokens, 1, 1,
                              None, None, Some(response.content.clone()),
                              Some("alone".to_string()), proof_file,
                              Some(acc.total_run_token_count()),
                              Some(acc.failed_branch_count),
                              wc.elapsed_ms(),
                              false, 1, 1, verifier_wait_ms,
                              oneshot_regime, oneshot_budget_base)
                }
                Ok(false) => {
                    // Lean rejected → neither leg.
                    make_pput(problem_file, "oneshot", model,
                              false, false, start, 0, 0, 1,
                              None, None, None, None, None,
                              Some(acc.total_run_token_count()),
                              Some(acc.failed_branch_count),
                              wc.elapsed_ms(),
                              false, 1, 1, verifier_wait_ms,
                              oneshot_regime, oneshot_budget_base)
                }
                Err(e) => {
                    warn!("Oracle error: {}", e);
                    // Lean error → measurement failure → neither leg.
                    make_pput(problem_file, "oneshot", model,
                              false, false, start, 0, 0, 1,
                              None, None, None, None, None,
                              Some(acc.total_run_token_count()),
                              Some(acc.failed_branch_count),
                              wc.elapsed_ms(),
                              false, 1, 1, verifier_wait_ms,
                              oneshot_regime, oneshot_budget_base)
                }
            }
        }
        Err(e) => {
            // C-012: measurement failure ≠ verified failure.
            // Do not emit PPUT_RESULT — batch runner must retry on resume.
            // C-017: broadcast error explicitly (stderr, non-zero exit).
            error!("LLM error: {}", e);
            eprintln!("MEASUREMENT_ERROR oneshot LLM: {}", e);
            std::process::exit(2);
        }
    }
}

/// Swarm: N agents, prediction market, Boltzmann routing → PPUT.
async fn run_swarm(
    problem_file: &str, problem_statement: &str, theorem_name: &str,
    lean_path: &str, proxy_url: &str, model: &str, n_agents: usize,
) -> PputResult {
    let start = Instant::now();
    let condition = format!("n{}", n_agents);

    // Phase A atom A6 (FC-trace correlation): pin a stable identifier at
    // run_swarm entry so every fc_event from this run shares one
    // correlation key. Format mirrors make_pput's run_id (condition +
    // problem_id + unix-ms) so Phase D joins via simple equality. The
    // value is read at every emit site below; cheap (clone of small
    // String).
    let run_corr_id = {
        let problem_id = std::path::Path::new(problem_file)
            .file_stem().and_then(|s| s.to_str()).unwrap_or(problem_file);
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis()).unwrap_or(0);
        format!("{}_{}_{}", condition, problem_id, ts)
    };

    let kernel = Kernel::new();
    let config = BusConfig {
        // Phase 2.1 (C-043 candidate): OMEGA-accepted proofs are auto-written
        // as tape nodes (mandatory wtool per Art. IV). Full proofs can be
        // long; raise bus caps so winning nodes don't get size-vetoed. Agent
        // partials still typically <1200; no behavioural regression.
        max_payload_chars: 8000,
        max_payload_lines: 200,
        system_lp_amount: 200.0,
        // C-011: decide/omega/native_decide forbidden (brute-force precedent)
        forbidden_patterns: vec![
            "native_decide".into(), "decide".into(), "omega".into(),
            "#eval".into(), "IO.Process".into(),
            "IO.FS".into(), "run_tac".into(), "unsafe".into(),
        ],
    };

    // Phase 1: opt-in tape persistence via env. WAL_DIR=<dir> enables WAL
    // writes to <dir>/<problem>_<timestamp>.jsonl; resumes if file exists.
    // Default off for backward-compat baseline runs.
    let mut bus = if let Ok(wal_dir) = std::env::var("WAL_DIR") {
        let problem_stem = std::path::Path::new(problem_file)
            .file_stem().map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "unknown".into());
        let resume_id = std::env::var("WAL_RESUME_ID").ok();
        let id = resume_id.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs().to_string())
                .unwrap_or_else(|_| "0".into())
        });
        let wal_path = std::path::Path::new(&wal_dir)
            .join(format!("{}_{}.jsonl", problem_stem, id));
        info!("[wal] using {:?}", wal_path);
        match TuringBus::with_wal_path(kernel, config, wal_path) {
            Ok(b) => b,
            Err(e) => {
                error!("[wal] open failed: {} — falling back to in-memory", e);
                TuringBus::new(Kernel::new(), BusConfig {
                    max_payload_chars: 1200, max_payload_lines: 18,
                    system_lp_amount: 200.0,
                    forbidden_patterns: vec![
                        "native_decide".into(), "decide".into(), "omega".into(),
                        "#eval".into(), "IO.Process".into(), "IO.FS".into(),
                        "run_tac".into(), "unsafe".into(),
                    ],
                })
            }
        }
    } else {
        TuringBus::new(kernel, config)
    };
    // Phase 4 (C-041 candidate): cross-problem wallet persistence. WALLET_STATE
    // env points to a json file; if it exists we load agents' carried-over
    // balances/portfolios, otherwise fresh genesis. No second mint under Law 2:
    // genesis_done is serialised, so on_init is a no-op post first boot.
    let wallet_state_path: Option<std::path::PathBuf> = std::env::var("WALLET_STATE")
        .ok().map(std::path::PathBuf::from);
    let wallet = wallet_state_path.as_ref()
        .and_then(|p| WalletTool::load_from_disk(p))
        .unwrap_or_else(|| WalletTool::new(10000.0));
    if wallet_state_path.is_some() && wallet.genesis_done {
        info!("[wallet] resumed from {:?}; existing agents carry balances",
              wallet_state_path);
    }
    bus.mount_tool(Box::new(wallet));
    bus.mount_tool(Box::new(Lean4Oracle::new(
        problem_statement.to_string(), theorem_name.to_string(), lean_path.to_string(),
    )));
    bus.mount_tool(Box::new(SearchTool::new(
        vec![format!("{}/MiniF2F/Test", std::env::var("MINIF2F_DIR")
            .unwrap_or_else(|_| DEFAULT_MINIF2F_DIR.into()))], 20,
    )));
    bus.mount_tool(Box::new(LibrarianTool::new(
        &format!("{}/skills", std::env::var("EXPERIMENT_DIR").unwrap_or_else(|_| ".".into())), 8,
    )));

    let agent_ids: Vec<String> = (0..n_agents).map(|i| format!("Agent_{}", i)).collect();
    bus.init(&agent_ids);
    // Phase 4: top-up ensure_agents for any IDs not in the loaded state (zero
    // balance if post-genesis, genesis_coins only on first-ever boot).
    if let Some(wallet) = bus.tools.iter_mut()
        .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>())
    {
        wallet.ensure_agents(&agent_ids);
    }

    // Phase A atom A3 (FC1-N7 δ/AI): per-agent model assignment via the
    // `AGENT_MODELS` env var. Default (unset/empty) broadcasts the global
    // `model` to every Agent_i. Heterogeneous payloads require
    // `PHASE_D_HETERO_OK=1` (Phase B+C single-model invariant — see
    // `agent_models.rs` module header). Failure is fatal at startup so a
    // misconfigured swarm cannot burn LLM budget on bad model identity.
    let agent_models = match minif2f_v4::agent_models::resolve_agent_models(model, n_agents) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("AGENT_MODELS resolution failed: {}", e);
            std::process::exit(1);
        }
    };
    // Stamp on jsonl: uniform → single canonical name; heterogeneous (Phase D
    // only, gated) → `hetero:{m1|m2|...}` so downstream PPUT analysis can
    // distinguish single-model runs from heterogeneous swarm runs without
    // having to crack open the genesis_payload model_snapshot field.
    let run_model_label: String = {
        let first = &agent_models[0];
        if agent_models.iter().all(|m| m == first) {
            first.clone()
        } else {
            let mut sorted: Vec<&str> = agent_models.iter().map(String::as_str).collect();
            sorted.sort();
            sorted.dedup();
            format!("hetero:{}", sorted.join("|"))
        }
    };
    info!("[swarm/{}] agent_models = [{}] (label={})", condition,
          agent_models.join(","), run_model_label);

    // Art. II.2.1: "不能抹杀群体异质性" — distinct skills per agent.
    // V3 had Math/Bull/Bear roles. V4: tactic-strategy specialization.
    let agent_skills: Vec<&str> = vec![
        "Focus on algebraic simplification: ring, field_simp, linarith, nlinarith.",
        "Focus on structural reasoning: induction, cases, rcases, constructor.",
        "Focus on rewriting and normalization: simp, norm_num, rw, calc.",
    ];

    let client = ResilientLLMClient::new(proxy_url, 1800, 2);
    let params = BoltzmannParams::from_env();
    // C-012: seed the Boltzmann RNG so A/B runs are reproducible.
    // Only the LLM sampling remains stochastic; same-problem paired comparison absorbs that.
    let boltzmann_seed: u64 = std::env::var("BOLTZMANN_SEED")
        .ok().and_then(|s| s.parse().ok()).unwrap_or(DEFAULT_BOLTZMANN_SEED);
    let mut boltz_rng = StdRng::seed_from_u64(boltzmann_seed);

    // Phase A atom A5 (FC2-N22 budget regime resolution): read
    // BUDGET_REGIME + MAX_TRANSACTIONS env, validate at startup, and
    // compute the loop bound. Errors abort BEFORE any LLM call so a
    // misconfigured run cannot consume API budget. Default
    // (env unset) = TotalProposal × 200, preserving Phase B baseline
    // bit-for-bit. PREREG_AMENDMENT_p0_defer § 3 condition 3.
    let (budget_regime, budget_max_tx_base, max_transactions) =
        match minif2f_v4::budget_regime::resolve_budget(n_agents) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("BUDGET_REGIME resolution failed: {}", e);
                std::process::exit(1);
            }
        };
    info!("[budget] regime={} base={} effective_max_tx={} (n_agents={})",
          budget_regime.label(), budget_max_tx_base, max_transactions, n_agents);
    let max_transactions = max_transactions as usize;

    // Art. IV map-reduce tick: periodic tape statistics (clock → mr → map/reduce)
    let tick_interval: usize = std::env::var("TICK_INTERVAL")
        .ok().and_then(|s| s.parse().ok()).unwrap_or(20);

    // C-036 startup echo: per-agent (skill, temp) so debugging never grep-source.
    let temp_ladder_on = std::env::var("TEMP_LADDER").ok().as_deref() == Some("1");
    let agent_cfg: Vec<String> = (0..n_agents).map(|i| {
        let s = i % agent_skills.len();
        let t = if temp_ladder_on { (0.10_f64 + (i as f64) * 0.15).min(1.30) } else { 0.2 };
        format!("Agent_{}:skill{}:t={:.2}", i, s, t)
    }).collect();
    info!("[swarm/{}] {}", condition, agent_cfg.join(" "));

    // C-036 telemetry counters.
    let mut tool_dist: HashMap<String, u32> = HashMap::new();
    let mut omega_payload_hashes: HashSet<u64> = HashSet::new();
    let mut omega_attempts: u32 = 0;
    let mut zero_ticks_run: u32 = 0;
    let mut zero_tick_warned = false;
    // Phase A atom A4 (FC1-N11 ∏p decision diversity): hash every parsed
    // proposal payload (append/complete/step) — broader than `omega_*`
    // which only counts OMEGA attempts. Cheap proxy for semantic
    // diversity (full embedding distance is Phase D+ work).
    let mut proposal_hashes: HashSet<u64> = HashSet::new();
    let mut proposal_count: u64 = 0;
    // Phase A atom A4 (FC1-N12 oracle scope): cumulative wall-clock
    // inside Lean for THIS run. Each verify_omega_detailed and
    // verify_partial call brackets its own elapsed and adds it here.
    let mut verifier_wait_ms: u64 = 0;
    // PPUT-CCL B2: full-run cost C_i — every LLM call + tool stdout summed
    // across all proposals (winning + failed branches). Read at terminal
    // make_pput sites and stamped on the emitted jsonl row.
    let mut acc = RunCostAccumulator::new();
    // PPUT-CCL B3: full-run wall-clock T_i — first agent prompt → final Lean
    // call. Opened on first tx's prompt build, closed before each return.
    let mut wc = RunWallClock::new();
    // Art. III.2: per-agent search result cache (bounded), fed into next prompt.
    let mut search_cache: HashMap<String, Vec<String>> = HashMap::new();
    // F-2026-04-19-05: cap searches per agent; beyond cap we remove `search`
    // from the tool list so agents stop wasting budget on name-match misses.
    let search_cap: u32 = std::env::var("SEARCH_CAP")
        .ok().and_then(|s| s.parse().ok()).unwrap_or(20);
    let mut search_count: HashMap<String, u32> = HashMap::new();
    // PPUT-CCL B7-extra (PREREG § 5.5): calibration treatment toggle.
    // When enabled, every proposal at tx >= ROLLBACK_TX_THRESHOLD is
    // synthetically vetoed. Constitutionally that is FC1-E18 (∏p=0 → Q_t)
    // applied repeatedly; the run then exhausts at FC2-N22 HALT via
    // `HaltReason::MaxTxExhausted`. We short-circuit at the threshold tx
    // for efficiency — see `rollback_sim.rs` module header for why this
    // is observably equivalent to running the loop to natural exhaustion.
    let rollback_sim_on = minif2f_v4::rollback_sim::rollback_simulation_enabled();
    if rollback_sim_on {
        info!("[rollback_sim] PREREG § 5.5 calibration treatment ON \
               (synthetic veto at tx >= {})", minif2f_v4::rollback_sim::ROLLBACK_TX_THRESHOLD);
    }

    for tx in 0..max_transactions {
        // PPUT-CCL B7-extra: short-circuit guard. Constitutional anchor
        // FC1-E18 + FC2-N22 (existing MaxTxExhausted variant). Stamps
        // tx_count at the threshold, not at max_transactions, so jsonl
        // analysis can distinguish a calibration treatment exit from a
        // real natural exhaustion.
        if minif2f_v4::rollback_sim::should_simulate_rollback(tx as u64, rollback_sim_on) {
            warn!("[rollback_sim] firing at tx={} — synthetic ∏p=0 from this tx, \
                   short-circuit to MaxTxExhausted exit (cost-asymmetric: skips \
                   ~150 LLM calls vs honest vetoed loop; downstream PPUT analysis \
                   MUST honor synthetic_short_circuit=true on this row)", tx);
            // A6 FC2-N22 (HALT): synthetic short-circuit path. Phase D
            // join key: reason="SyntheticShortCircuit" disambiguates from
            // natural MaxTxExhausted (which exits at tx=max_transactions).
            minif2f_v4::fc_trace::emit_event(
                minif2f_v4::fc_trace::FcId::Fc2N22,
                &run_corr_id, Some(tx as u64), None,
                &[("reason", minif2f_v4::fc_trace::json_str("SyntheticShortCircuit"))],
            );
            wc.mark_final_accept();
            // A4: synthetic short-circuit is NOT a max-tx exhaustion (it
            // exits ~150 tx EARLY at the rollback threshold). hit_max_tx
            // stays false — synthetic_short_circuit is the disambiguator
            // for this calibration-treatment path.
            let mut result = make_pput(problem_file, &condition, &run_model_label,
                                       false, false, start, 0, 0,
                                       tx as u64, Some(tool_dist), None,
                                       None, None, None,
                                       Some(acc.total_run_token_count()),
                                       Some(acc.failed_branch_count),
                                       wc.elapsed_ms(),
                                       false,
                                       proposal_hashes.len() as u64,
                                       proposal_count,
                                       verifier_wait_ms,
                                       budget_regime, budget_max_tx_base);
            // B7-extra disambiguator: distinguish this calibration-treatment
            // exit from a natural max-tx exhaustion in downstream PPUT
            // analysis. See PputResult::synthetic_short_circuit doc-comment
            // for the cost-asymmetry note.
            result.synthetic_short_circuit = Some(true);
            return result;
        }

        // PPUT-CCL B3 (mid-term audit P0-C fix 2026-04-25): open the wall-clock
        // bracket at the top of the FIRST tx (before chain/skill/board build
        // and before build_agent_prompt). Idempotent — only the first tx's
        // call sticks; subsequent calls no-op. PREREG § 5 / plan B3 define
        // T_i as "first agent prompt construction"; this is the earliest
        // moment the agent begins constructing its prompt.
        wc.mark_first_read();

        // Map-reduce tick (Art. IV mermaid: clock → mr → tape)
        if tick_interval > 0 && tx > 0 && tx % tick_interval == 0 {
            let tape_len = bus.kernel.tape.time_arrow().len();
            let market_count = bus.kernel.markets.len();
            let ticker = bus.kernel.market_ticker(5);
            let top_prices: Vec<String> = ticker.iter()
                .map(|(id, p)| format!("{}:{:.0}%", id, p * 100.0))
                .collect();
            info!("[tick@tx{}] tape={} markets={} top={}", tx, tape_len, market_count,
                top_prices.join(", "));
            // A6 FC2-N20 (mr tick): clock → mr → tape per Art. IV.
            // Phase D consumer joins on (run_corr_id, tx) to derive the
            // tape-growth curve and detect zero-tick stalls before they
            // become C-036 alarm events.
            minif2f_v4::fc_trace::emit_event(
                minif2f_v4::fc_trace::FcId::Fc2N20,
                &run_corr_id, Some(tx as u64), None,
                &[
                    ("tape_len", tape_len.to_string()),
                    ("market_count", market_count.to_string()),
                ],
            );
            // Phase 6-emergent: refresh shared team board from facts only.
            // Per-agent cumulative balance + recent tape-node authorship counts
            // + top market prices. No instructions, no "should" — just state.
            if std::env::var("EMERGENT_ROLES").ok().as_deref() == Some("1") {
                let agents_sorted: Vec<String> = agent_ids.clone();
                let mut author_counts: std::collections::HashMap<String, u32> =
                    std::collections::HashMap::new();
                for nid in bus.kernel.tape.time_arrow() {
                    if let Some(n) = bus.kernel.tape.get(nid) {
                        *author_counts.entry(n.author.clone()).or_insert(0) += 1;
                    }
                }
                let wallet_balances: std::collections::HashMap<String, f64> =
                    bus.tools.iter()
                        .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
                        .map(|w| w.balances.clone())
                        .unwrap_or_default();
                let mut board = format!("# tick@tx{} (tape_nodes={})\n", tx, tape_len);
                for a in &agents_sorted {
                    let bal = wallet_balances.get(a).copied().unwrap_or(10000.0);
                    let delta = bal - 10000.0;
                    let nodes = author_counts.get(a).copied().unwrap_or(0);
                    board.push_str(&format!(
                        "- {}: balance={:.0} (Δ{:+.0}), tape_nodes_authored={}\n",
                        a, bal, delta, nodes));
                }
                if !top_prices.is_empty() {
                    board.push_str(&format!("markets: {}\n", top_prices.join(", ")));
                }
                // Preserve any agent posts that were already in the file (append-only).
                if let Some(lib) = bus.tools.iter()
                    .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
                {
                    let existing = lib.read_board();
                    // Keep only the POST lines (they carry agent-originated intent).
                    let posts: String = existing.lines()
                        .filter(|l| l.starts_with("## POST") || (l.starts_with(" ") == false && !l.starts_with("#") && !l.starts_with("-") && !l.starts_with("markets:")))
                        .collect::<Vec<_>>()
                        .join("\n");
                    let full = if posts.is_empty() {
                        board
                    } else {
                        format!("{}\n{}\n", board, posts)
                    };
                    let _ = lib.write_board(&full);
                }
            }
            // C-036 zero-tick alarm: 5 consecutive ticks with no constitutional engine activity.
            if tape_len == 0 && market_count == 0 {
                zero_ticks_run += 1;
                if zero_ticks_run >= 5 && !zero_tick_warned {
                    warn!("[harness] {} consecutive zero-ticks (tape & markets idle) — \
                           constitutional engines bypassed (Art. II.1/II.2 unused)", zero_ticks_run);
                    zero_tick_warned = true;
                }
            } else {
                zero_ticks_run = 0;
            }
        }

        let agent_idx = tx % n_agents;
        let agent_id = &agent_ids[agent_idx];
        let snap = bus.snapshot();

        let chain = if snap.tape.is_empty() {
            problem_statement.to_string()
        } else {
            let nodes: Vec<String> = snap.tape.time_arrow().iter()
                .filter_map(|id| snap.tape.get(id))
                .map(|n| format!("[{}] {}: {}", n.id, n.author, n.payload))
                .collect();
            format!("{}\n\n=== Proof Chain ===\n{}", problem_statement, nodes.join("\n"))
        };

        let errors = bus.recent_rejections(agent_id, 3);
        // Art. II.2.1: per-agent skill specialization + Librarian learned memory
        let base_skill = agent_skills.get(agent_idx % agent_skills.len()).unwrap_or(&"");
        let learned = bus.tools.iter()
            .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
            .and_then(|lib| lib.read_agent_memory(agent_id))
            .unwrap_or_default();
        let skill = if learned.is_empty() {
            base_skill.to_string()
        } else {
            format!("{}\n\n{}", base_skill, learned)
        };
        let hits_ref: Vec<String> = search_cache.get(agent_id).cloned().unwrap_or_default();
        let tools_desc = if search_count.get(agent_id).copied().unwrap_or(0) >= search_cap {
            "append, complete, invest"
        } else {
            "append, complete, invest, search"
        };
        // Phase 6-emergent: read the shared team board. Gated by EMERGENT_ROLES=1
        // so baseline behaviour is untouched. Board content is built by
        // Librarian at periodic ticks (see refresh_board below).
        let team_board: String = if std::env::var("EMERGENT_ROLES").ok().as_deref() == Some("1") {
            bus.tools.iter()
                .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
                .map(|l| l.read_board())
                .unwrap_or_default()
        } else {
            String::new()
        };
        let prompt = build_agent_prompt(
            &chain, &skill, &snap.market_ticker, &errors, &hits_ref,
            snap.get_balance(agent_id), tools_desc, &team_board,
        );

        // Phase A atom A3: bind δ for this agent_idx (same vector resolved
        // once at run_swarm entry from AGENT_MODELS env). In Phase B+C this
        // is uniform across all agent_idx; in Phase D it may diverge.
        let agent_model = &agent_models[agent_idx];
        // Model-aware max_tokens (same rule as oneshot branch). Per-agent so
        // a heterogeneous Phase D swarm mixing chat + reasoner backbones gets
        // the right ceiling per-call instead of a single global heuristic.
        let max_toks = if agent_model.contains("chat") { 8000 } else { 16000 };
        // Art. II.2.1 anti-homogeneity: per-agent temperature ladder breaks
        // sampling correlation among role-distinct agents (F-2026-04-18-03).
        // Disabled (keep at 0.2) when TEMP_LADDER!=1 to isolate the mechanism.
        let temp: f64 = if std::env::var("TEMP_LADDER").ok().as_deref() == Some("1") {
            (0.10_f64 + (agent_idx as f64) * 0.15).min(1.30)
        } else {
            0.2
        };
        let request = GenerateRequest {
            model: agent_model.clone(),
            messages: vec![Message { role: "user".into(), content: prompt }],
            temperature: Some(temp),
            max_tokens: Some(max_toks),
        };

        // PPUT-CCL B6 runtime gate (swarm path): swarm prompts include
        // tape contents, board posts, search hits, and learned memory —
        // any of these state surfaces could in principle inject a PPUT
        // value at runtime even when the prompt builder is clean. Gate
        // every tx, every agent, every iteration.
        assert_no_metric_leak(&request.messages[0].content);
        match client.generate(&request).await {
            Ok(response) => {
                acc.record_llm_call(response.prompt_tokens, response.completion_tokens);
                // PPUT-CCL B2: every parsed proposal default-records as failed.
                // OMEGA-accept return paths flip the last record before returning.
                acc.record_proposal(false);
                match parse_agent_output(&response.content) {
                    Ok(action) => match action.tool.as_str() {
                        "append" => {
                            *tool_dist.entry("append".into()).or_insert(0) += 1;
                            if let Some(payload) = &action.payload {
                                // A4: record proposal for tactic_diversity.
                                let mut ph = std::collections::hash_map::DefaultHasher::new();
                                payload.hash(&mut ph);
                                proposal_hashes.insert(ph.finish());
                                proposal_count += 1;
                                let prices: std::collections::HashMap<String, f64> =
                                    snap.markets.iter()
                                        .map(|(id, m)| (id.clone(), m.yes_price))
                                        .collect();
                                let parent = boltzmann_select_parent(
                                    &snap.tape, &prices, &params, &mut boltz_rng
                                );
                                match bus.append(agent_id, payload, parent.as_deref()) {
                                    Ok(BusResult::Appended { node_id }) => {
                                        info!("[tx {}] {} +{}", tx, agent_id, node_id);
                                        // Art. III.2 Librarian: every compress_interval appends,
                                        // write mechanical summary (TopK error classes) to agent's
                                        // learned.md. This is white-box compression (Art. I.2:
                                        // deterministic statistical algorithm), not LLM-based.
                                        if let Some(lib) = bus.tools.iter()
                                            .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>()) {
                                            if lib.should_compress() {
                                                let errors = bus.recent_rejections(agent_id, 10);
                                                let summary = format!(
                                                    "# Learned patterns (auto-compressed)\n\
                                                     Common errors: {}\n\
                                                     Tape depth: {}\n",
                                                    errors.join(", "),
                                                    snap.tape.time_arrow().len(),
                                                );
                                                let _ = lib.write_agent_memory(agent_id, &summary);
                                                info!("[tx {}] Librarian compressed for {}", tx, agent_id);
                                            }
                                        }
                                    }
                                    Ok(BusResult::Vetoed { reason }) => {
                                        warn!("[tx {}] VETO: {}", tx, reason);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "complete" => {
                            *tool_dist.entry("complete".into()).or_insert(0) += 1;
                            if let Some(payload) = &action.payload {
                                // Art. IV (∏p(output | Q_t)): Q_t (tape) feeds the verification
                                // predicate. Dual-path: try payload-alone first (standalone proof
                                // preserved), then tape+payload (tape-built proof). Accept whichever
                                // succeeds. This keeps Q_t in the ∏p domain without punishing
                                // self-contained proofs that ignored tape.
                                let tape_chain: String = bus.kernel.tape.time_arrow().iter()
                                    .filter_map(|id| bus.kernel.tape.get(id))
                                    .map(|n| n.payload.clone())
                                    .collect::<Vec<_>>()
                                    .join("\n");
                                let tape_len = bus.kernel.tape.time_arrow().len();
                                // C-036: track payload diversity over what agent proposed.
                                let mut h = std::collections::hash_map::DefaultHasher::new();
                                payload.hash(&mut h);
                                omega_payload_hashes.insert(h.finish());
                                omega_attempts += 1;
                                // A4: also record into the broader proposal set
                                // for tactic_diversity (covers append/complete/step).
                                proposal_hashes.insert(h.finish());
                                proposal_count += 1;
                                info!("[tx {}] OMEGA claim by {} (tape_nodes={}, payload_len={})",
                                      tx, agent_id, tape_len, payload.len());
                                let oracle = Lean4Oracle::new(
                                    problem_statement.to_string(),
                                    theorem_name.to_string(),
                                    lean_path.to_string(),
                                );
                                // Path 1: payload alone (A4 verifier_wait bracket)
                                let v_t0 = Instant::now();
                                let r_alone = oracle.verify_omega_detailed(payload);
                                verifier_wait_ms += v_t0.elapsed().as_millis() as u64;
                                let (full_proof, path_choice, r_final) = match &r_alone {
                                    Ok((true, _)) => (payload.clone(), "alone", r_alone.clone()),
                                    _ if !tape_chain.is_empty() => {
                                        // Path 2: tape + payload (A4 verifier_wait bracket)
                                        let combined = format!("{}\n{}", tape_chain, payload);
                                        let v_t1 = Instant::now();
                                        let r_combined = oracle.verify_omega_detailed(&combined);
                                        verifier_wait_ms += v_t1.elapsed().as_millis() as u64;
                                        if matches!(r_combined, Ok((true, _))) {
                                            *tool_dist.entry("complete_via_tape".into()).or_insert(0) += 1;
                                        }
                                        (combined, "tape+payload", r_combined)
                                    }
                                    _ => (payload.clone(), "alone", r_alone.clone()),
                                };
                                // PPUT-CCL B3: close bracket AFTER both Lean verify paths return.
                                // Soft Law (Phase C) cannot exit ahead of verify-time accounting.
                                wc.mark_final_accept();
                                match r_final {
                                    Ok((true, _)) => {
                                        // PPUT-CCL B2: this proposal verified — flip the failed
                                        // record made at parse time into the run's accepted slot.
                                        acc.flip_last_failed_to_accepted();
                                        // Phase 0 (C-039): persist the winning artifact so external
                                        // verifiers can re-run lean from disk alone.
                                        let preview: String = full_proof.chars().take(500).collect();
                                        info!(">>> OMEGA ACCEPTED <<< (path={}, payload[0..500]={:?})",
                                              path_choice, preview);
                                        let proof_file = persist_proof_artifact(
                                            problem_file, &theorem_name, &problem_statement,
                                            &full_proof, path_choice, agent_id,
                                        );
                                        // Phase 2.1 (C-043 candidate): mandatory wtool. Art. IV says
                                        // `∏p = 1 ⟹ Q_{t+1} = wtool(output)`. Before halting, write
                                        // the winning payload as a tape node through the standard
                                        // append pipeline. This automatically fires founder grant
                                        // (Phase 2 reward-pull) for the winning author and makes
                                        // every solve end with a canonical tape node on the GP.
                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
                                        *tool_dist.entry("omega_wtool".into()).or_insert(0) += 1;
                                        // Use oracle-blessed path: Lean has already accepted this
                                        // payload, so bus-level forbidden_patterns and size caps
                                        // would only re-reject legitimate tactics (e.g. `omega`,
                                        // `decide` used inside a verified proof — not brute-force).
                                        let omega_node_id = match bus.append_oracle_accepted(
                                            agent_id, payload, parent.as_deref(),
                                        ) {
                                            Ok(BusResult::Appended { node_id }) => Some(node_id),
                                            Ok(BusResult::Vetoed { reason }) => {
                                                warn!("[art-iv] OMEGA wtool VETO (unexpected after oracle accept): {}", reason);
                                                None
                                            }
                                            _ => None,
                                        };
                                        let tape_tokens: u64 = bus.kernel.tape.time_arrow().iter()
                                            .filter_map(|id| bus.kernel.tape.get(id))
                                            .map(|n| n.payload.len() as u64)
                                            .sum();
                                        // C-012: gp_tokens reflects the actual tape (now containing
                                        // the winner), no double-count needed.
                                        let gp_tokens = tape_tokens.max(response.completion_tokens as u64);
                                        let gp = bus.kernel.tape.time_arrow().to_vec();
                                        let gp_nodes = gp.len();
                                        if omega_node_id.is_some() {
                                            info!("[art-iv] OMEGA written as tape node; gp_nodes={}", gp_nodes);
                                        }
                                        bus.halt_and_settle(&gp).ok();
                                        // A6 FC2-N22 (HALT — OmegaAccepted via full proof): the
                                        // canonical success-path event. Phase D filters on
                                        // reason="OmegaAccepted" + gp_path="alone|tape+payload" to
                                        // build the OMEGA accept-rate timeseries.
                                        minif2f_v4::fc_trace::emit_event(
                                            minif2f_v4::fc_trace::FcId::Fc2N22,
                                            &run_corr_id, Some(tx as u64), Some(agent_id.as_str()),
                                            &[
                                                ("reason", minif2f_v4::fc_trace::json_str("OmegaAccepted")),
                                                ("gp_path", minif2f_v4::fc_trace::json_str(path_choice)),
                                                ("gp_nodes", gp_nodes.to_string()),
                                            ],
                                        );
                                        // Phase 4: persist wallet state so next problem's run
                                        // inherits carried-over balances (reputation).
                                        if let Some(ref wp) = wallet_state_path {
                                            if let Some(w) = bus.tools.iter()
                                                .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
                                            {
                                                if let Err(e) = w.save_to_disk(wp) {
                                                    warn!("[wallet] save failed to {:?}: {}", wp, e);
                                                }
                                            }
                                        }
                                        let upr = if omega_attempts > 0 {
                                            Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
                                        } else { None };
                                        // P0-A: Phase B swarm complete — runtime gate IS the
                                        // Lean verify_omega_detailed call we just consumed
                                        // (Ok((true, _))). Both legs hold. Phase C Soft Law
                                        // would inject `verify_post_hoc(&oracle, &full_proof)`
                                        // here and pass its result as post_hoc_verified.
                                        return make_pput(problem_file, &condition, &run_model_label,
                                                        true, true,
                                                        start, gp_tokens, gp_nodes, tx as u64 + 1,
                                                        Some(tool_dist), upr,
                                                        Some(full_proof.clone()),
                                                        Some(path_choice.to_string()),
                                                        proof_file,
                                                        Some(acc.total_run_token_count()),
                                                        Some(acc.failed_branch_count),
                                                        wc.elapsed_ms(),
                                                        false,
                                                        proposal_hashes.len() as u64,
                                                        proposal_count,
                                                        verifier_wait_ms,
                                                        budget_regime, budget_max_tx_base);
                                    }
                                    Ok((false, err_detail)) => {
                                        // Step-B v3: classify + record class label (C-022 shield).
                                        let class = classify_lean_error(&err_detail);
                                        bus.record_rejection(agent_id, class.label());
                                        // PPUT-CCL B2: rejection error feeds back into next prompt's
                                        // recent_rejections — count those bytes against C_i.
                                        acc.record_tool_stdout(&err_detail);
                                        let preview: String = payload.chars().take(300).collect();
                                        warn!("[tx {}] OMEGA rejected ({}). payload[0..300]={:?}", tx, class.label(), preview);
                                    }
                                    Err(e) => {
                                        warn!("[tx {}] OMEGA oracle error: {}", tx, e);
                                    }
                                }
                            }
                        }
                        "invest" => {
                            *tool_dist.entry("invest".into()).or_insert(0) += 1;
                            // Law 2: Only Investment Costs Money (1 Coin = 1 YES + 1 NO).
                            // Agent bets on a tape node's quality. This drives price signals
                            // (Art. II.2) which guide Boltzmann routing (Art. II.2.1).
                            // Direction: prefer explicit `direction` field (long/short);
                            // fall back to sign of amount (positive=long, negative=short).
                            // Bidirectional signals let agents express dissent (Art. II.2).
                            if let (Some(node_id), Some(amount)) = (&action.node, action.amount) {
                                let amt = amount.abs();
                                if amt > 0.0 {
                                    let buy_yes = match action.direction.as_deref() {
                                        Some("long") | Some("yes") | Some("LONG") | Some("YES") => true,
                                        Some("short") | Some("no") | Some("SHORT") | Some("NO") => false,
                                        _ => amount > 0.0,  // sign-based fallback
                                    };
                                    // Law 2 conservation: validate market BEFORE debit (no coin-loss path)
                                    let market_exists = bus.kernel.yes_price(node_id).is_some();
                                    if !market_exists {
                                        warn!("[tx {}] invest: no market for {} (hallucinated node?)", tx, node_id);
                                    } else {
                                        // Debit wallet → buy shares → record (atomic intent)
                                        let wallet_ok = bus.tools.iter_mut()
                                            .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>())
                                            .map(|w| w.deduct(agent_id, amt).is_ok())
                                            .unwrap_or(false);
                                        if wallet_ok {
                                            let result = if buy_yes {
                                                bus.kernel.buy_yes(node_id, amt)
                                            } else {
                                                bus.kernel.buy_no(node_id, amt)
                                            };
                                            match result {
                                                Ok(shares) => {
                                                    info!("[tx {}] {} invested {:.0} {} on {} → {:.1} shares",
                                                        tx, agent_id, amt,
                                                        if buy_yes { "YES" } else { "NO" },
                                                        node_id, shares);
                                                    if let Some(w) = bus.tools.iter_mut()
                                                        .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>()) {
                                                        if buy_yes {
                                                            w.record_shares(agent_id, node_id, shares, 0.0, 0.0);
                                                        } else {
                                                            w.record_shares(agent_id, node_id, 0.0, shares, 0.0);
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    // Market existed at check but buy failed — should not happen
                                                    warn!("[tx {}] invest buy error: {} (coins debited, shares not granted — Law 2 violation logged)", tx, e);
                                                }
                                            }
                                        } else {
                                            warn!("[tx {}] {} insufficient balance for invest", tx, agent_id);
                                        }
                                    }
                                }
                            }
                        }
                        "search" => {
                            // F-2026-04-19-05 cap: if over budget this agent's turn the
                            // search slot shouldn't even be offered, but the LLM may still
                            // emit `search` ignoring the prompt — record and skip execute.
                            let cnt = search_count.entry(agent_id.clone()).or_insert(0);
                            if *cnt >= search_cap {
                                *tool_dist.entry("search_capped".into()).or_insert(0) += 1;
                            } else {
                                *cnt += 1;
                                *tool_dist.entry("search".into()).or_insert(0) += 1;
                                // Law 1: search is free. Execute and cache top hits (Art. III.2).
                                if let Some(query) = &action.query {
                                    let hits = bus.tools.iter()
                                        .find_map(|t| t.as_any().downcast_ref::<SearchTool>())
                                        .map(|s| s.search(query))
                                        .unwrap_or_default();
                                    let trimmed: Vec<String> = hits.iter().take(5)
                                        .map(|p| p.rsplit('/').next().unwrap_or(p).to_string())
                                        .collect();
                                    // PPUT-CCL B2: search hits feed `hits_ref` into next prompt —
                                    // count the cached bytes against C_i.
                                    acc.record_tool_stdout(&trimmed.join("\n"));
                                    info!("[tx {}] {} search({:?}) → {} hits: {}",
                                          tx, agent_id, query, hits.len(), trimmed.join(","));
                                    search_cache.insert(agent_id.clone(), trimmed);
                                }
                            }
                        }
                        "post" => {
                            *tool_dist.entry("post".into()).or_insert(0) += 1;
                            // Phase 6-emergent: agent posts a short message to the
                            // shared Librarian board. Other agents see it on next
                            // prompt. State-only; no central role planner.
                            if let Some(msg) = &action.payload {
                                if let Some(lib) = bus.tools.iter()
                                    .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
                                {
                                    if let Err(e) = lib.post_to_board(agent_id, msg) {
                                        warn!("[tx {}] post failed: {}", tx, e);
                                    } else {
                                        info!("[tx {}] {} posted to board", tx, agent_id);
                                    }
                                }
                            }
                        }
                        "step" => {
                            // Phase 7 (C-043+ Turing δ-step): submit ONE tactic,
                            // oracle classifies the accumulated tape+tactic prefix
                            // as Complete / PartialOk / Reject. Writes a tape node
                            // on PartialOk and Complete so the DAG grows one cell
                            // at a time — the Art. IV semantics Turing 1936 defines.
                            *tool_dist.entry("step".into()).or_insert(0) += 1;
                            if let Some(tactic) = &action.payload {
                                // A4: record proposal for tactic_diversity.
                                let mut ph = std::collections::hash_map::DefaultHasher::new();
                                tactic.hash(&mut ph);
                                proposal_hashes.insert(ph.finish());
                                proposal_count += 1;
                                let tape_chain: String = bus.kernel.tape.time_arrow().iter()
                                    .filter_map(|id| bus.kernel.tape.get(id))
                                    .map(|n| n.payload.clone())
                                    .collect::<Vec<_>>()
                                    .join("\n");
                                let prefix = if tape_chain.is_empty() {
                                    tactic.clone()
                                } else {
                                    format!("{}\n{}", tape_chain, tactic)
                                };
                                let oracle = Lean4Oracle::new(
                                    problem_statement.to_string(),
                                    theorem_name.to_string(),
                                    lean_path.to_string(),
                                );
                                // A4: bracket the Lean partial-verify call.
                                let v_t0 = Instant::now();
                                let verdict = oracle.verify_partial(&prefix);
                                verifier_wait_ms += v_t0.elapsed().as_millis() as u64;
                                // PPUT-CCL B3: close bracket after step-verify returns.
                                wc.mark_final_accept();
                                match verdict {
                                    PartialVerdict::Complete => {
                                        acc.flip_last_failed_to_accepted();
                                        info!(">>> OMEGA ACCEPTED <<< via step (depth={} after this write)",
                                              bus.kernel.tape.time_arrow().len() + 1);
                                        let proof_file = persist_proof_artifact(
                                            problem_file, &theorem_name, &problem_statement,
                                            &prefix, "per_tactic", agent_id,
                                        );
                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
                                        *tool_dist.entry("omega_wtool".into()).or_insert(0) += 1;
                                        let _ = bus.append_oracle_accepted(
                                            agent_id, tactic, parent.as_deref(),
                                        );
                                        let tape_tokens: u64 = bus.kernel.tape.time_arrow().iter()
                                            .filter_map(|id| bus.kernel.tape.get(id))
                                            .map(|n| n.payload.len() as u64)
                                            .sum();
                                        let gp_tokens = tape_tokens.max(response.completion_tokens as u64);
                                        let gp = bus.kernel.tape.time_arrow().to_vec();
                                        let gp_nodes = gp.len();
                                        bus.halt_and_settle(&gp).ok();
                                        let upr = if omega_attempts > 0 {
                                            Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
                                        } else { None };
                                        // A6 FC2-N22 (HALT — OmegaAccepted via per-tactic
                                        // PartialVerdict::Complete). Distinguished from the
                                        // full-proof OMEGA path by gp_path="per_tactic"; both
                                        // share reason="OmegaAccepted".
                                        minif2f_v4::fc_trace::emit_event(
                                            minif2f_v4::fc_trace::FcId::Fc2N22,
                                            &run_corr_id, Some(tx as u64), Some(agent_id.as_str()),
                                            &[
                                                ("reason", minif2f_v4::fc_trace::json_str("OmegaAccepted")),
                                                ("gp_path", minif2f_v4::fc_trace::json_str("per_tactic")),
                                                ("gp_nodes", gp_nodes.to_string()),
                                            ],
                                        );
                                        // P0-A: Phase B swarm step Complete — runtime gate IS
                                        // the Lean verify_partial call (PartialVerdict::Complete).
                                        // Both legs hold. Phase C Soft Law diverges here.
                                        return make_pput(problem_file, &condition, &run_model_label,
                                                        true, true,
                                                        start, gp_tokens, gp_nodes, tx as u64 + 1,
                                                        Some(tool_dist), upr,
                                                        Some(prefix.clone()),
                                                        Some("per_tactic".to_string()),
                                                        proof_file,
                                                        Some(acc.total_run_token_count()),
                                                        Some(acc.failed_branch_count),
                                                        wc.elapsed_ms(),
                                                        false,
                                                        proposal_hashes.len() as u64,
                                                        proposal_count,
                                                        verifier_wait_ms,
                                                        budget_regime, budget_max_tx_base);
                                    }
                                    PartialVerdict::PartialOk => {
                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
                                        match bus.append_oracle_accepted(
                                            agent_id, tactic, parent.as_deref(),
                                        ) {
                                            Ok(BusResult::Appended { node_id }) => {
                                                *tool_dist.entry("step_partial_ok".into()).or_insert(0) += 1;
                                                info!("[tx {}] {} step+{} partial OK (depth={})",
                                                      tx, agent_id, node_id,
                                                      bus.kernel.tape.time_arrow().len());
                                            }
                                            Ok(BusResult::Vetoed { reason }) => {
                                                warn!("[tx {}] step partial OK but bus vetoed: {}", tx, reason);
                                            }
                                            _ => {}
                                        }
                                    }
                                    PartialVerdict::Reject(reason) => {
                                        let class = classify_lean_error(&reason);
                                        bus.record_rejection(agent_id, class.label());
                                        // PPUT-CCL B2: step rejection reason flows into next prompt.
                                        acc.record_tool_stdout(&reason);
                                        *tool_dist.entry("step_reject".into()).or_insert(0) += 1;
                                        let preview = reason.chars().take(200).collect::<String>();
                                        warn!("[tx {}] step rejected ({}): {}", tx, class.label(), preview);
                                    }
                                }
                            }
                        }
                        other => {
                            *tool_dist.entry(format!("other:{}", other)).or_insert(0) += 1;
                        }
                    },
                    Err(e) => {
                        *tool_dist.entry("parse_fail".into()).or_insert(0) += 1;
                        // Step-B v3: parse failures feed the class graveyard too.
                        let class = classify_parse_error(&format!("{}", e));
                        bus.record_rejection(agent_id, class.label());
                        // PPUT-CCL B2: classifier label flows into next prompt's errors.
                        acc.record_tool_stdout(class.label());
                        warn!("[tx {}] parse: {} ({})", tx, e, class.label());
                    }
                }
            }
            Err(e) => {
                *tool_dist.entry("llm_err".into()).or_insert(0) += 1;
                warn!("[tx {}] LLM: {}", tx, e);
            }
        }
    }

    let upr = if omega_attempts > 0 {
        Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
    } else { None };
    // Phase 4: also save wallet state on no-OMEGA exit. Agents may have
    // invested/lost Coin during the run; durability should not depend on a win.
    if let Some(ref wp) = wallet_state_path {
        if let Some(w) = bus.tools.iter()
            .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
        {
            let _ = w.save_to_disk(wp);
        }
    }
    // No OMEGA found → PPUT = 0
    // B3: close bracket on max-tx exhaustion path.
    // P0-A: max-tx exhaustion → neither leg fired.
    // A4: this is the canonical hit_max_tx=true site (ran the full
    // for-loop without OMEGA and without firing the synthetic
    // short-circuit, which would have returned earlier).
    wc.mark_final_accept();
    // A6 FC2-N22 (HALT — natural MaxTxExhausted): the canonical
    // budget-exhausted exit. Phase D filters reason="MaxTxExhausted"
    // to compute solve_rate-vs-budget curves; pairs with the A5
    // budget_regime stamp on the v2 jsonl row.
    minif2f_v4::fc_trace::emit_event(
        minif2f_v4::fc_trace::FcId::Fc2N22,
        &run_corr_id, Some(max_transactions as u64), None,
        &[
            ("reason", minif2f_v4::fc_trace::json_str("MaxTxExhausted")),
            ("budget_regime", minif2f_v4::fc_trace::json_str(budget_regime.label())),
            ("budget_max_transactions", budget_max_tx_base.to_string()),
            ("proposal_count", proposal_count.to_string()),
        ],
    );
    make_pput(problem_file, &condition, &run_model_label,
              false, false, start, 0, 0,
              max_transactions as u64, Some(tool_dist), upr,
              None, None, None,
              Some(acc.total_run_token_count()),
              Some(acc.failed_branch_count),
              wc.elapsed_ms(),
              true,
              proposal_hashes.len() as u64,
              proposal_count,
              verifier_wait_ms,
              budget_regime, budget_max_tx_base)
}

fn make_pput(
    problem: &str, condition: &str, model: &str,
    runtime_accepted: bool, post_hoc_verified: bool, start: Instant,
    gp_tokens: u64, gp_nodes: usize, tx_count: u64,
    tool_dist: Option<HashMap<String, u32>>,
    unique_payload_ratio: Option<f64>,
    gp_payload: Option<String>,
    gp_path: Option<String>,
    gp_proof_file: Option<String>,
    total_run_token_count: Option<u64>,
    failed_branch_count: Option<u32>,
    total_wall_time_ms: Option<u64>,
    // Phase A atom A4 (decomposed metrics). All callers must pass
    // explicit values — the v2 fields are non-Optional.
    hit_max_tx: bool,
    distinct_proposals: u64,
    total_proposals: u64,
    verifier_wait_ms: u64,
    // Phase A atom A5 (FC2-N22 budget regime stamp). Caller declares
    // the regime + base BEFORE the loop so MaxTxExhausted rows are
    // unambiguous about which partitioning rule produced them.
    budget_regime: minif2f_v4::budget_regime::BudgetRegime,
    budget_max_transactions: u32,
) -> PputResult {
    // PPUT-CCL Phase B B4 (mid-term audit P0-A fix 2026-04-25):
    // make_pput is now PURELY computational. The caller MUST decide both
    // `runtime_accepted` (did the evaluator's runtime gate fire?) and
    // `post_hoc_verified` (did Lean independently confirm the proof?). The
    // prior implementation derived `post_hoc_verified = has_gp` internally,
    // which would have laundered Phase C Soft Law fake-accepts into the
    // North Star pput_verified. Forcing the caller to pass both legs makes
    // Soft Law's design point unmissable: any caller that fakes runtime
    // accept must explicitly pass post_hoc_verified=verify_post_hoc(...)
    // or the divergence will surface immediately.
    //
    // Phase B all callers pass `(runtime_accepted, post_hoc_verified) = (X, X)`
    // because runtime IS Lean today. Phase C diverges at the Soft Law
    // mode call site, not inside this function.
    let has_gp = runtime_accepted; // legacy `has_golden_path` field semantics
    let elapsed = start.elapsed().as_secs_f64();
    let pput = if has_gp && elapsed > 0.0 { 100.0 / elapsed } else { 0.0 };
    // C-012 provenance: populated from env vars; None when unset (backward compat).
    let build_sha = std::env::var("BUILD_SHA").ok();
    let classifier_version = std::env::var("CLASSIFIER_VERSION").ok();
    let boltzmann_seed = std::env::var("BOLTZMANN_SEED")
        .ok().and_then(|s| s.parse::<u64>().ok());

    // Mid-term audit P0-B fix 2026-04-25: collapse Optional accumulator/clock
    // values into required v2 fields. Phase B always has values for these
    // (B2 + B3 wire them at every emit site); the prior Option wrapping was
    // overly defensive and let the v2 schema slip from the contract.
    let c_i = total_run_token_count.unwrap_or(0);
    let t_i = total_wall_time_ms.unwrap_or(0);
    let failed_count = failed_branch_count.unwrap_or(0);

    let progress_runtime = compute_progress_runtime(runtime_accepted);
    let progress_verified =
        compute_progress_verified(runtime_accepted, post_hoc_verified);
    let pput_runtime = compute_pput(progress_runtime, c_i, t_i);
    let pput_verified = compute_pput(progress_verified, c_i, t_i);
    let pput_m_verified = compute_pput_m(progress_verified, c_i, t_i);

    // V2 fields read from env (per-process globals).
    let split = std::env::var("SPLIT").unwrap_or_else(|_| {
        eprintln!("[v2-emit] SPLIT env unset; defaulting to 'adaptation' \
                   (Phase B convention; pre-registration requires SPLIT \
                   for Phase C+ ablation runs)");
        "adaptation".to_string()
    });
    let mode = std::env::var("MODE").unwrap_or_else(|_| "full".to_string());
    let model_snapshot = std::env::var("MODEL_SNAPSHOT")
        .unwrap_or_else(|_| model.to_string());
    let git_sha = build_sha.clone().unwrap_or_default();
    let binary_sha256 = std::env::var("BINARY_SHA256").unwrap_or_default();

    // problem_id = basename without .lean
    let problem_id = std::path::Path::new(problem)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(problem)
        .to_string();
    // run_id = condition + problem_id + ts (collision-free for sequential runs)
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let run_id = format!("{}_{}_{}", condition, problem_id, ts);

    PputResult {
        // ── B1 v2 schema fields ──
        schema_version: "v2.0".to_string(),
        run_id,
        problem_id,
        solved: runtime_accepted,
        split,
        verified: post_hoc_verified,
        golden_path_token_count: gp_tokens,
        total_run_token_count: c_i,
        total_wall_time_ms: t_i,
        progress: progress_verified,
        pput_runtime,
        pput_verified,
        pput_m_verified,
        failed_branch_count: failed_count,
        // Phase B placeholders — Phase C+ wires these as the modes activate.
        rollback_count: 0,
        hit_max_tx,
        tactic_diversity: minif2f_v4::jsonl_schema::compute_tactic_diversity(
            distinct_proposals, total_proposals,
        ),
        verifier_wait_ms,
        budget_regime: budget_regime.label().to_string(),
        budget_max_transactions,
        far: 0.0, err: 0.0, iac: 0.0, cpr: 0.0,
        model_snapshot,
        git_sha,
        binary_sha256,
        mode,
        // ── Legacy diagnostic fields ──
        problem: problem.to_string(),
        condition: condition.to_string(),
        model: model.to_string(),
        has_golden_path: has_gp,
        time_secs: elapsed,
        pput,
        gp_token_count: gp_tokens,
        gp_node_count: gp_nodes,
        tx_count,
        build_sha,
        classifier_version,
        boltzmann_seed,
        tool_dist,
        unique_payload_ratio,
        gp_payload,
        gp_path,
        gp_proof_file,
        // B7-extra: only the calibration-treatment short-circuit site mutates
        // this to Some(true). Default = None (most callers).
        synthetic_short_circuit: None,
    }
}

/// Phase 0 (C-039 candidate): persist a self-contained, re-verifiable proof artifact.
/// Writes <EXPERIMENT_DIR>/proofs/<theorem>_<timestamp>_<short_hash>.lean containing
/// the exact code that the Lean oracle accepted. An external verifier can run
/// `lean --stdin < <file>` with the matching toolchain + Mathlib and reproduce the result.
/// Returns the relative path (for embedding in PputResult) or None on I/O failure.
fn persist_proof_artifact(
    problem_file: &str, theorem_name: &str, problem_statement: &str,
    full_proof: &str, path_choice: &str, agent_id: &str,
) -> Option<String> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let exp_dir = std::env::var("EXPERIMENT_DIR").unwrap_or_else(|_| ".".into());
    let proofs_dir = std::path::Path::new(&exp_dir).join("proofs");
    if let Err(e) = std::fs::create_dir_all(&proofs_dir) {
        log::warn!("[audit] cannot create proofs dir {:?}: {}", proofs_dir, e);
        return None;
    }
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let mut h = DefaultHasher::new();
    full_proof.hash(&mut h);
    let short = format!("{:x}", h.finish() & 0xFFFFFFFF);
    let fname = format!("{}_{}_{}.lean", theorem_name, ts, short);
    let path = proofs_dir.join(&fname);
    let header = format!(
        "-- TuringOS v4 Phase 0 audit artifact (C-039 candidate)\n\
         -- problem_file: {}\n\
         -- theorem: {}\n\
         -- path_choice: {} (alone | tape+payload)\n\
         -- accepted_by_agent: {}\n\
         -- timestamp_unix: {}\n\
         -- Reproduce: LEAN_PATH=<mathlib paths> lean --stdin < this_file\n\
         --\n",
        problem_file, theorem_name, path_choice, agent_id, ts
    );
    let body = format!("{}\n{}\n{}", header, problem_statement, full_proof);
    match std::fs::write(&path, body) {
        Ok(_) => Some(format!("proofs/{}", fname)),
        Err(e) => {
            log::warn!("[audit] cannot write proof artifact {:?}: {}", path, e);
            None
        }
    }
}

/// A2 (Phase A engineering atom 2 of 8): swarm-condition parser.
///
/// Returns `Some(N)` if `condition` matches `n<digits>` for any positive
/// integer N (including N=1, the swarm_N=1 baseline). Returns `None` for
/// `oneshot`, `hybrid_v1`, malformed (`n-1`, `nfoo`, ``, etc).
///
/// Per Plan-agent NEXT-1 / Codex E0 / Gemini E1-Prime brainstorm
/// (handover/brainstorms/): EVERY N-curve experiment in the 2026-04-25
/// N-experiments arc MUST use `n1` (not `oneshot`) as the N=1 baseline,
/// because `oneshot` skips the swarm loop, tape, mr ticks, and ∏p
/// product. Without this discrimination, any N→PPUT curve confounds
/// "agent count effect" with "different runtime architecture".
///
/// FC-trace: FC2-N16 InitAI orchestration entry — discriminates between
/// the two registered InitAI shapes (oneshot vs swarm). FC1-N11 ∏p path
/// is reached only via swarm (n*) condition.
pub(crate) fn parse_swarm_condition_n(condition: &str) -> Option<usize> {
    if !condition.starts_with('n') { return None; }
    let rest = &condition[1..];
    if rest.is_empty() { return None; }
    rest.parse::<usize>().ok().filter(|&n| n >= 1)
}

#[cfg(test)]
mod swarm_condition_tests {
    use super::parse_swarm_condition_n;

    #[test]
    fn parses_valid_n_swarm_conditions() {
        assert_eq!(parse_swarm_condition_n("n1"), Some(1));   // swarm_N=1 baseline
        assert_eq!(parse_swarm_condition_n("n3"), Some(3));   // current default swarm size
        assert_eq!(parse_swarm_condition_n("n8"), Some(8));   // hetero candidate size
        assert_eq!(parse_swarm_condition_n("n16"), Some(16)); // upper N for stress test
        assert_eq!(parse_swarm_condition_n("n100"), Some(100));
    }

    #[test]
    fn rejects_oneshot_condition() {
        // Critical: 'oneshot' MUST NOT parse as a swarm condition.
        // It's a different code path (single LLM call, no tape, no
        // ∏p product). The N-experiments arc relies on this distinction.
        assert_eq!(parse_swarm_condition_n("oneshot"), None);
    }

    #[test]
    fn rejects_hybrid_v1_and_other_named_conditions() {
        assert_eq!(parse_swarm_condition_n("hybrid_v1"), None);
        assert_eq!(parse_swarm_condition_n("full"), None);
        assert_eq!(parse_swarm_condition_n("soft_law"), None);
        assert_eq!(parse_swarm_condition_n("panopticon"), None);
        assert_eq!(parse_swarm_condition_n("amnesia"), None);
        assert_eq!(parse_swarm_condition_n("homogeneous"), None);
    }

    #[test]
    fn rejects_malformed_n_conditions() {
        assert_eq!(parse_swarm_condition_n(""), None);          // empty
        assert_eq!(parse_swarm_condition_n("n"), None);         // just prefix
        assert_eq!(parse_swarm_condition_n("nfoo"), None);      // non-digit
        assert_eq!(parse_swarm_condition_n("n-1"), None);       // negative (parses fail on usize)
        assert_eq!(parse_swarm_condition_n("n0"), None);        // zero (filter rejects)
        assert_eq!(parse_swarm_condition_n("n 3"), None);       // whitespace
        assert_eq!(parse_swarm_condition_n("3"), None);         // missing 'n' prefix
        assert_eq!(parse_swarm_condition_n("N3"), None);        // case-sensitive
    }

    #[test]
    fn n1_is_distinct_from_oneshot() {
        // The discriminant test: n1 and oneshot are different conditions
        // even though both run with effectively 1 agent. The PARSER
        // returns Some(1) for n1 and None for oneshot, which routes
        // them to different code paths in main().
        assert_eq!(parse_swarm_condition_n("n1"), Some(1));
        assert_eq!(parse_swarm_condition_n("oneshot"), None);
    }
}

#[cfg(test)]
mod v2_emit_tests {
    use super::*;
    use minif2f_v4::jsonl_schema::RunRecord;
    use std::sync::Mutex;

    // Per feedback_env_var_test_lock: tests that mutate process-global env
    // vars must serialize to survive cargo's parallel runner.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    /// Mid-term audit P0-B fix conformance:
    /// Every emitted PputResult row must dispatch as `RunRecord::V2(_)`,
    /// not `RunRecord::Legacy(_)`. The pre-fix evaluator emitted rows with
    /// no `schema_version` field, which forced B1's dispatcher to classify
    /// new B2-B4 output as Legacy + extras, silently breaking the v2 schema
    /// contract. This test fails the build if a future change drops the
    /// `schema_version` stamp or any required v2 field.
    #[test]
    fn test_emit_dispatches_as_v2() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var("SPLIT", "adaptation");
        std::env::set_var("MODE", "full");

        // Phase B success path: runtime + post-hoc both fired.
        let result = make_pput(
            "test_problem.lean", "oneshot", "deepseek-v4-flash",
            true, true, Instant::now(),
            500, 1, 1,
            None, None, None, None, None,
            Some(2000), Some(0), Some(15_000),
            // A4: oneshot success — no max-tx, 1/1 unique, 4500ms in Lean.
            false, 1, 1, 4_500,
            // A5: oneshot stamps total_proposal + base=1 (single LLM call).
            minif2f_v4::budget_regime::BudgetRegime::TotalProposal, 1,
        );

        let line = serde_json::to_string(&result).expect("serialize PputResult");

        // Schema discriminator must be present.
        assert!(
            line.contains("\"schema_version\":\"v2.0\""),
            "v2 emit must stamp schema_version=v2.0; got: {}",
            line
        );

        // Round-trip via RunRecord::from_json — must dispatch to V2.
        match RunRecord::from_json(&line).expect("v2 line parses") {
            RunRecord::V2(agg) => {
                assert_eq!(agg.schema_version, "v2.0");
                assert_eq!(agg.split, "adaptation");
                assert_eq!(agg.mode, "full");
                assert_eq!(agg.solved, true);
                assert_eq!(agg.verified, true);
                assert_eq!(agg.progress, 1u8);
                assert_eq!(agg.total_run_token_count, 2000);
                assert_eq!(agg.total_wall_time_ms, 15_000);
                assert!(agg.pput_verified > 0.0);
                assert_eq!(agg.pput_runtime, agg.pput_verified,
                    "Phase B: runtime IS Lean — pput_runtime must equal pput_verified");
                // A4 fields round-trip through emit.
                assert_eq!(agg.hit_max_tx, false);
                assert_eq!(agg.tactic_diversity, 1.0);
                assert_eq!(agg.verifier_wait_ms, 4_500);
                assert!(agg.verifier_wait_ms <= agg.total_wall_time_ms,
                    "A4 invariant: verifier_wait_ms must not exceed total_wall_time_ms");
            }
            RunRecord::Legacy(_) => panic!(
                "v2 emit MUST dispatch to RunRecord::V2, not Legacy. \
                 Schema-v2 contract regression — see B5 deferral checklist P0-B. \
                 Line was: {}", line
            ),
        }

        std::env::remove_var("SPLIT");
        std::env::remove_var("MODE");
    }

    /// Mid-term audit P0-B fix conformance (Soft Law H1 detection at the
    /// emit boundary): when runtime accepts but post-hoc Lean rejects, the
    /// emitted v2 row must show progress=0 and pput_verified=0 even with
    /// pput_runtime > 0. This is the divergence signal Phase C will scan.
    #[test]
    fn test_emit_soft_law_divergence_signal() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var("SPLIT", "adaptation");
        std::env::set_var("MODE", "soft_law");

        // Synthetic Soft Law-style emit: runtime says yes, Lean says no.
        let result = make_pput(
            "test_problem.lean", "oneshot", "deepseek-v4-flash",
            /*runtime_accepted=*/ true,
            /*post_hoc_verified=*/ false,
            Instant::now(),
            500, 1, 1,
            None, None, None, None, None,
            Some(2000), Some(0), Some(15_000),
            // A4: same shape as success path; A4 fields are independent
            // of the H1 divergence signal we're testing here.
            false, 1, 1, 4_500,
            minif2f_v4::budget_regime::BudgetRegime::TotalProposal, 1,
        );

        assert_eq!(result.progress, 0u8,
            "Lean rejected → progress MUST be 0 (North Star truth)");
        assert_eq!(result.verified, false);
        assert!(result.pput_runtime > 0.0,
            "pput_runtime inflates under runtime accept (the divergence signal)");
        assert_eq!(result.pput_verified, 0.0,
            "pput_verified MUST collapse to 0 when Lean rejects");
        assert!(result.pput_runtime - result.pput_verified > 0.0,
            "(pput_runtime - pput_verified) > 0 ⟺ Soft Law divergence detected");

        std::env::remove_var("SPLIT");
        std::env::remove_var("MODE");
    }

    /// Phase A atom A4 conformance: max-tx exhaustion path stamps
    /// `hit_max_tx=true` AND splits `solve_rate` from `tokens_per_solve`
    /// + `time_per_solve` correctly (per Gemini brainstorm 2026-04-25
    /// § A.4). This is the "swarm spent the budget but didn't solve"
    /// row that downstream analysis must distinguish from OMEGA accept.
    #[test]
    fn test_a4_emit_max_tx_exhaustion_row() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var("SPLIT", "adaptation");
        std::env::set_var("MODE", "full");

        // Synthetic max-tx exhaustion: 200 tx, neither leg fired, swarm
        // proposed 50 unique payloads out of 200 tries (collision rate
        // typical of mid-N swarm on a hard problem).
        let result = make_pput(
            "test_problem.lean", "n3", "deepseek-v4-flash",
            false, false, Instant::now(),
            0, 0, 200,
            None, None, None, None, None,
            Some(8_000), Some(199), Some(120_000),
            true, 50, 200, 90_000,
            // A5: canonical Phase B baseline = total_proposal × 200.
            minif2f_v4::budget_regime::BudgetRegime::TotalProposal, 200,
        );

        let line = serde_json::to_string(&result).expect("serialize PputResult");
        match RunRecord::from_json(&line).expect("v2 line parses") {
            RunRecord::V2(agg) => {
                // Decomposed-metric rule (Gemini brainstorm): on a max-tx
                // exhaustion, solve_rate=0 but tokens_per_solve / time_per_solve
                // are UNDEFINED (not 0). The contract here is that progress=0
                // → pput_verified=0, and downstream analysis must filter on
                // progress before averaging tokens/time.
                assert_eq!(agg.hit_max_tx, true);
                assert_eq!(agg.solved, false);
                assert_eq!(agg.progress, 0u8);
                assert_eq!(agg.pput_verified, 0.0);
                // tactic_diversity = 50/200 = 0.25 (notable correlation,
                // worth flagging — see C-036 unique_payload_ratio < 0.30
                // catastrophic-correlation threshold; A4 generalizes it).
                assert!((agg.tactic_diversity - 0.25).abs() < 1e-9);
                // verifier_wait_ms ≤ total_wall_time_ms invariant.
                assert!(agg.verifier_wait_ms <= agg.total_wall_time_ms);
                assert_eq!(agg.verifier_wait_ms, 90_000);
                assert_eq!(agg.total_wall_time_ms, 120_000);
            }
            RunRecord::Legacy(_) => panic!(
                "A4 max-tx row MUST dispatch to RunRecord::V2"
            ),
        }

        std::env::remove_var("SPLIT");
        std::env::remove_var("MODE");
    }

    /// Phase A atom A4 conformance: B7-extra synthetic short-circuit
    /// MUST NOT set hit_max_tx=true. The two exit paths look identical
    /// at `tx_count` time but mean different things — synthetic exits
    /// EARLY at the rollback threshold (~50 tx) and is tagged via
    /// `synthetic_short_circuit`; natural exhaustion runs the full
    /// 200 tx and is tagged via `hit_max_tx`. Conflating them
    /// neutralizes the calibration-treatment vs production split.
    #[test]
    fn test_a4_synthetic_short_circuit_does_not_set_hit_max_tx() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var("SPLIT", "adaptation");
        std::env::set_var("MODE", "full");

        // Mirror the synthetic short-circuit return shape (evaluator.rs
        // line ~622): hit_max_tx=false, then caller sets
        // synthetic_short_circuit=Some(true) on the result.
        let mut result = make_pput(
            "test_problem.lean", "n3", "deepseek-v4-flash",
            false, false, Instant::now(),
            0, 0, 50,
            None, None, None, None, None,
            Some(2_000), Some(49), Some(40_000),
            false, 20, 50, 25_000,
            minif2f_v4::budget_regime::BudgetRegime::TotalProposal, 200,
        );
        result.synthetic_short_circuit = Some(true);

        let line = serde_json::to_string(&result).expect("serialize PputResult");
        match RunRecord::from_json(&line).expect("v2 line parses") {
            RunRecord::V2(agg) => {
                // The disambiguator: hit_max_tx stays false on the
                // synthetic-treatment row even though the run did not
                // OMEGA. synthetic_short_circuit lives in the legacy
                // diagnostic envelope (not in v2 RunAggregate); the
                // raw `line` carries it for downstream tools.
                assert_eq!(agg.hit_max_tx, false,
                    "synthetic short-circuit MUST NOT set hit_max_tx — it exits EARLY");
            }
            RunRecord::Legacy(_) => panic!("A4 short-circuit row must dispatch as v2"),
        }
        assert!(line.contains("\"synthetic_short_circuit\":true"),
            "synthetic short-circuit must remain visible on the raw row");

        std::env::remove_var("SPLIT");
        std::env::remove_var("MODE");
    }
}

```

## src/drivers/llm_proxy.py

```python
#!/usr/bin/env python3
"""
LLM Proxy v4 — OpenAI-compatible local HTTP server with token metering.

Phase A atom A7. Adapted from v3's `src/drivers/llm_proxy.py` with one
load-bearing v4 change: per-provider multi-key round-robin so the three
SiliconFlow keys (SILICONFLOW_API_KEY / _SECONDARY / _TERTIARY) split
concurrent traffic across separate rate-limit pools — the V3L-27
N=30 → 401/429 collapse documented in `cases/V3_LESSONS.md` was
single-key. The same pattern extends to other providers if multiple
keys are configured.

Endpoints:
  POST /v1/chat/completions  (OpenAI-compatible, forwards to cloud APIs)
  GET  /health
  GET  /stats               (token counters + per-key request distribution)
  POST /stats/reset         (reset counters — call before each experiment)

Usage:
  SILICONFLOW_API_KEY=sk-xxx \\
  SILICONFLOW_API_KEY_SECONDARY=sk-yyy \\
  SILICONFLOW_API_KEY_TERTIARY=sk-zzz \\
    python3 src/drivers/llm_proxy.py --port 8080

Without --provider, model identity drives routing:
  - "deepseek-*" → deepseek
  - "Qwen/...", "openai/...", anything containing "/" → siliconflow
  - else → dashscope
"""
import os, sys, json, logging, argparse, time, threading, itertools
from http.server import HTTPServer, BaseHTTPRequestHandler
from socketserver import ThreadingMixIn
from openai import OpenAI, RateLimitError, APIStatusError

logging.basicConfig(level=logging.INFO, format='%(asctime)s %(levelname)s %(message)s')
log = logging.getLogger("llm_proxy")

# Each provider entry: (base_url, [env-var names tried in order]).
# Multiple env names = multi-key round-robin. The PRIMARY name MUST be
# first; any later names are optional fallback / additional pool keys.
PROVIDERS = {
    "dashscope": (
        "https://dashscope.aliyuncs.com/compatible-mode/v1",
        ["DASHSCOPE_API_KEY"],
    ),
    "aliyun": (
        "https://dashscope.aliyuncs.com/compatible-mode/v1",
        ["DASHSCOPE_API_KEY"],
    ),
    "siliconflow": (
        "https://api.siliconflow.cn/v1",
        [
            "SILICONFLOW_API_KEY",
            "SILICONFLOW_API_KEY_SECONDARY",
            "SILICONFLOW_API_KEY_TERTIARY",
        ],
    ),
    "deepseek": (
        "https://api.deepseek.com",
        ["DEEPSEEK_API_KEY"],
    ),
    "volcengine": (
        "https://ark.cn-beijing.volces.com/api/v3",
        ["VOLCENGINE_API_KEY"],
    ),
    "nvidia": (
        "https://integrate.api.nvidia.com/v1",
        ["NVIDIA_NIM_API_KEY"],
    ),
}

# Per-(provider, key-index) OpenAI client cache: provider -> list[OpenAI]
clients_by_provider = {}
# Round-robin counter per provider.
_rr_counters = {}
_rr_lock = threading.Lock()
# Per-key request counters for /stats observability.
_per_key_requests = {}  # provider -> list[int]


def _build_clients(provider):
    """Return list of OpenAI clients for `provider`, one per available key.

    Lazy. Caches in `clients_by_provider`. Raises ValueError if NO key
    is set for the provider.
    """
    if provider in clients_by_provider:
        return clients_by_provider[provider]
    base_url, key_envs = PROVIDERS[provider]
    keys = []
    for env_name in key_envs:
        v = os.environ.get(env_name, "").strip()
        if v:
            keys.append((env_name, v))
    if not keys:
        raise ValueError(
            f"No keys set for provider={provider}; tried env vars {key_envs}"
        )
    clients = [OpenAI(api_key=k, base_url=base_url) for (_, k) in keys]
    clients_by_provider[provider] = clients
    _per_key_requests[provider] = [0] * len(clients)
    log.info(
        f"[provider {provider}] resolved {len(clients)} key(s) from envs: "
        f"{[name for (name, _) in keys]}"
    )
    return clients


def get_client_round_robin(provider):
    """Return (client, key_index) using round-robin across configured keys."""
    clients = _build_clients(provider)
    with _rr_lock:
        idx = _rr_counters.get(provider, 0) % len(clients)
        _rr_counters[provider] = idx + 1
        _per_key_requests[provider][idx] += 1
    return clients[idx], idx


# ── Token Metering ──
_stats_lock = threading.Lock()
_stats = {
    "prompt_tokens": 0,
    "completion_tokens": 0,
    "total_tokens": 0,
    "requests": 0,
    "errors": 0,
    "retries_429": 0,
    "estimated_count": 0,
}


def _record_usage(prompt_tokens, completion_tokens):
    with _stats_lock:
        _stats["prompt_tokens"] += prompt_tokens
        _stats["completion_tokens"] += completion_tokens
        _stats["total_tokens"] += prompt_tokens + completion_tokens
        _stats["requests"] += 1


def _record_estimated():
    with _stats_lock:
        _stats["estimated_count"] += 1


def _record_error():
    with _stats_lock:
        _stats["errors"] += 1


def _record_retry():
    with _stats_lock:
        _stats["retries_429"] += 1


def _reset_stats():
    with _stats_lock:
        for k in _stats:
            _stats[k] = 0
        for prov in _per_key_requests:
            _per_key_requests[prov] = [0] * len(_per_key_requests[prov])


def _get_stats():
    with _stats_lock:
        snap = dict(_stats)
        snap["per_key_requests"] = {
            p: list(v) for p, v in _per_key_requests.items()
        }
        return snap


# ── Rate Limiter ──
_rate_lock = threading.Lock()
_rate_semaphore = threading.Semaphore(int(os.environ.get("LLM_PROXY_CONCURRENCY", "5")))
_cooldown_until = 0.0


def _wait_for_cooldown():
    global _cooldown_until
    now = time.time()
    if now < _cooldown_until:
        wait = _cooldown_until - now
        log.info(f"[RATE LIMITER] Cooling down {wait:.1f}s")
        time.sleep(wait)


def _trigger_cooldown(seconds):
    global _cooldown_until
    with _rate_lock:
        new_until = time.time() + seconds
        if new_until > _cooldown_until:
            _cooldown_until = new_until
            log.warning(f"[RATE LIMITER] Global cooldown {seconds}s")


def detect_provider(model):
    """Route by model identifier. v4 prefers explicit `provider:model`
    syntax (e.g. `siliconflow:Qwen/Qwen2.5-7B-Instruct`), but also
    handles legacy bare names for backward compat with v3 callers.
    """
    if ":" in model:
        prefix = model.split(":", 1)[0].lower()
        if prefix in PROVIDERS:
            return prefix
    m = model.lower()
    if "deepseek" in m:
        return "deepseek"
    if "/" in model and not m.startswith("qwen"):
        return "siliconflow"
    if m.startswith("qwen"):
        return "dashscope"
    return "dashscope"


def strip_provider_prefix(model):
    """If model is `provider:foo/bar`, return `foo/bar`; else `model`."""
    if ":" in model:
        prefix, rest = model.split(":", 1)
        if prefix.lower() in PROVIDERS:
            return rest
    return model


class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == "/health":
            self._json_response(200, {"status": "ok"})
        elif self.path == "/stats":
            self._json_response(200, _get_stats())
        else:
            self.send_error(404)

    def do_POST(self):
        if self.path == "/stats/reset":
            _reset_stats()
            self._json_response(200, {"status": "reset"})
            log.info("[STATS] Counters reset")
            return

        if "/v1/chat/completions" not in self.path:
            self.send_error(404)
            return

        length = int(self.headers.get("Content-Length", 0))
        body = json.loads(self.rfile.read(length)) if length else {}

        raw_model = body.get("model", "qwen3-8b")
        messages = body.get("messages", [])
        temperature = body.get("temperature", 0.5)
        max_tokens = body.get("max_tokens", 3072)
        enable_thinking = body.get("enable_thinking", False)

        provider = FORCED_PROVIDER or detect_provider(raw_model)
        model = strip_provider_prefix(raw_model)

        try:
            client, key_idx = get_client_round_robin(provider)

            extra = {}
            if enable_thinking:
                extra["extra_body"] = {"enable_thinking": True}
            elif "qwen3" in model.lower():
                extra["extra_body"] = {"enable_thinking": False}

            max_retries = 8
            content = ""
            reasoning = ""
            usage_prompt = 0
            usage_completion = 0
            estimated = False

            _wait_for_cooldown()
            _rate_semaphore.acquire()
            try:
                for attempt in range(max_retries + 1):
                    _wait_for_cooldown()
                    try:
                        if attempt == 0:
                            log.info(
                                f"→ {provider}#k{key_idx}/{model} "
                                f"(temp={temperature}, max_tok={max_tokens})"
                            )
                        else:
                            log.info(
                                f"→ {provider}#k{key_idx}/{model} "
                                f"(retry {attempt}/{max_retries})"
                            )

                        resp = client.chat.completions.create(
                            model=model,
                            messages=messages,
                            temperature=temperature,
                            max_tokens=max_tokens,
                            stream=False,
                            **extra,
                        )

                        msg = resp.choices[0].message
                        content = msg.content or ""
                        reasoning = getattr(msg, "reasoning_content", None) or ""

                        estimated = False
                        if resp.usage and resp.usage.completion_tokens:
                            usage_prompt = resp.usage.prompt_tokens or 0
                            usage_completion = resp.usage.completion_tokens or 0
                        else:
                            estimated = True
                            usage_prompt = sum(
                                len(m.get("content", "")) for m in messages
                            ) // 3
                            usage_completion = (len(content) + len(reasoning)) // 3

                        break  # success

                    except (RateLimitError, APIStatusError) as e:
                        is_429 = isinstance(e, RateLimitError) or (
                            hasattr(e, "status_code") and e.status_code == 429
                        )
                        if is_429 and attempt < max_retries:
                            _record_retry()
                            wait = min(2 ** attempt + 1, 30)
                            _trigger_cooldown(wait)
                            time.sleep(wait)
                        else:
                            raise
            finally:
                _rate_semaphore.release()

            _record_usage(usage_prompt, usage_completion)
            if estimated:
                _record_estimated()

            result = {
                "choices": [
                    {
                        "message": {
                            "role": "assistant",
                            "content": content,
                            "reasoning_content": reasoning if reasoning else None,
                        },
                        "finish_reason": "stop",
                    }
                ],
                "model": raw_model,
                "usage": {
                    "prompt_tokens": usage_prompt,
                    "completion_tokens": usage_completion,
                    "total_tokens": usage_prompt + usage_completion,
                    "estimated": estimated,
                },
            }

            log.info(
                f"← {provider}#k{key_idx}/{model}: {len(content)}c content, "
                f"{len(reasoning)}c reasoning, "
                f"{usage_prompt}+{usage_completion}={usage_prompt + usage_completion} tokens"
            )

            self._json_response(200, result)

        except Exception as e:
            _record_error()
            log.error(f"Error on {provider}/{raw_model}: {e}")
            self._json_response(500, {"error": {"message": str(e)}})

    def _json_response(self, code, data):
        self.send_response(code)
        self.send_header("Content-Type", "application/json")
        self.end_headers()
        self.wfile.write(json.dumps(data).encode())

    def log_message(self, format, *args):
        pass


class ThreadedHTTPServer(ThreadingMixIn, HTTPServer):
    daemon_threads = True


FORCED_PROVIDER = None


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--port",
        type=int,
        default=int(os.environ.get("LLM_PROXY_PORT", "8080")),
    )
    parser.add_argument(
        "--provider",
        type=str,
        default=None,
        help="Force all requests to this provider (overrides model-based routing)",
    )
    args = parser.parse_args()

    if args.provider:
        if args.provider not in PROVIDERS:
            log.error(
                f"Unknown provider: {args.provider}. Available: {list(PROVIDERS.keys())}"
            )
            sys.exit(1)
        FORCED_PROVIDER = args.provider
        log.info(f"Provider forced to: {args.provider}")

    # Pre-resolve which providers have keys configured (for /health-style
    # diagnostics). Don't fail boot on missing keys — they're only
    # required when the corresponding provider is actually invoked.
    configured = []
    for prov_name, (_, key_envs) in PROVIDERS.items():
        present = [e for e in key_envs if os.environ.get(e, "").strip()]
        if present:
            configured.append(f"{prov_name}({len(present)}k)")

    server = ThreadedHTTPServer(("127.0.0.1", args.port), Handler)
    log.info(f"LLM Proxy v4 listening on 127.0.0.1:{args.port}")
    log.info(f"Providers configured: {', '.join(configured) if configured else '(none)'}")
    log.info("Token metering: enabled (/stats, /stats/reset)")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        log.info("Shutting down")

```

## scripts/smoke_siliconflow.sh

```bash
#!/usr/bin/env bash
# Phase A atom A7 — SiliconFlow integration smoke.
#
# Probes each of the 3 SiliconFlow keys (primary / secondary / tertiary)
# with a minimal chat call against a cheap model. Reports per-key OK or
# FAIL without printing key material. Exits non-zero if any key fails.
#
# Why direct SDK probes (not via the proxy round-robin): we want a
# per-key verdict — the proxy's round-robin would obscure which specific
# key failed if one is rate-limited / revoked. After A7 PASSes, the
# evaluator's runtime path goes through llm_proxy.py.
#
# Cost: 3 calls × ~50 tokens each = ¥0.001 - ¥0.005 total. SiliconFlow
# Qwen2.5-7B-Instruct free tier covers this; backstop is the user's
# key budget. Aborts after the first key fails to bound spend.
#
# Usage:
#   bash scripts/smoke_siliconflow.sh
#
# Reads keys from .env (auto-loaded) or current shell env.

set -euo pipefail

cd "$(dirname "$0")/.."
ROOT="$(pwd)"

# Source .env if present; do not echo any value.
if [ -f .env ]; then
    set -a
    # shellcheck disable=SC1091
    . .env
    set +a
fi

# Sanity: at least the primary key must be set.
: "${SILICONFLOW_API_KEY:?SILICONFLOW_API_KEY not set; configure .env first}"

python3 "$ROOT/scripts/_smoke_siliconflow.py"

```

## scripts/_smoke_siliconflow.py

```python
#!/usr/bin/env python3
"""Phase A atom A7 — per-key SiliconFlow probe.

Invoked by `scripts/smoke_siliconflow.sh`. Reads the three keys from
env (`SILICONFLOW_API_KEY` / `_SECONDARY` / `_TERTIARY`), issues one
tiny chat-completion call per key, and reports OK/FAIL per key WITHOUT
printing any key material. Exits non-zero if any configured key fails.

Cost bound: 3 calls × ~50 tokens. Qwen2.5-7B-Instruct on SiliconFlow
free tier is the cheapest stable option (V3L-27 N=30 collapse caveat
applies only at high concurrency; one call per key is safe).
"""
import os
import sys
import time

try:
    from openai import OpenAI, APIStatusError, RateLimitError
except ImportError:
    print("[A7-smoke] FAIL: openai SDK not installed (pip install openai)")
    sys.exit(2)

KEY_ENVS = [
    ("primary", "SILICONFLOW_API_KEY"),
    ("secondary", "SILICONFLOW_API_KEY_SECONDARY"),
    ("tertiary", "SILICONFLOW_API_KEY_TERTIARY"),
]
BASE_URL = "https://api.siliconflow.cn/v1"
# Qwen2.5-7B-Instruct: smallest stable production model on SF free tier.
# Avoids expensive reasoning models during probe.
PROBE_MODEL = "Qwen/Qwen2.5-7B-Instruct"
PROBE_PROMPT = "Reply with the single word: ack"
PROBE_MAX_TOKENS = 8


def probe_one(label: str, env_name: str, key: str) -> tuple[bool, str]:
    """Return (ok, summary). Never returns the key in `summary`."""
    client = OpenAI(api_key=key, base_url=BASE_URL)
    t0 = time.time()
    try:
        resp = client.chat.completions.create(
            model=PROBE_MODEL,
            messages=[{"role": "user", "content": PROBE_PROMPT}],
            temperature=0.0,
            max_tokens=PROBE_MAX_TOKENS,
            stream=False,
        )
    except RateLimitError as e:
        return False, f"RateLimitError ({type(e).__name__}): {str(e)[:120]}"
    except APIStatusError as e:
        return False, f"APIStatusError {getattr(e, 'status_code', '?')}: {str(e)[:120]}"
    except Exception as e:
        return False, f"Error {type(e).__name__}: {str(e)[:120]}"
    dt_ms = int((time.time() - t0) * 1000)
    msg = resp.choices[0].message
    content = (msg.content or "").strip()
    usage = resp.usage
    pt = getattr(usage, "prompt_tokens", "?") if usage else "?"
    ct = getattr(usage, "completion_tokens", "?") if usage else "?"
    return True, (
        f"{dt_ms}ms; tokens prompt={pt} completion={ct}; "
        f"content[:32]={content[:32]!r}"
    )


def main() -> int:
    print(
        f"[A7-smoke] SiliconFlow probe — model={PROBE_MODEL} "
        f"max_tokens={PROBE_MAX_TOKENS}"
    )
    any_failed = False
    any_present = False
    for label, env_name in KEY_ENVS:
        key = os.environ.get(env_name, "").strip()
        if not key:
            print(f"  [{label:9s}] {env_name}: NOT SET — skipping")
            continue
        any_present = True
        ok, summary = probe_one(label, env_name, key)
        verdict = "OK  " if ok else "FAIL"
        print(f"  [{label:9s}] {env_name}: {verdict} {summary}")
        if not ok:
            any_failed = True
    if not any_present:
        print("[A7-smoke] FAIL: no SiliconFlow keys configured")
        return 2
    if any_failed:
        print("[A7-smoke] result: FAIL (one or more keys failed)")
        return 1
    print("[A7-smoke] result: PASS (all configured keys responded)")
    return 0


if __name__ == "__main__":
    sys.exit(main())

```

## experiments/minif2f_v4/tests/fc_trace_smoke.rs

```rust
// Phase A atom A6 — fc_trace end-to-end smoke.
//
// Spawns a tiny child process that calls `emit_event` with FC_TRACE=1
// + FC_TRACE_FILE=<tempfile>, then asserts the file contains a
// well-formed JSON line with the expected fc_id + run_id + agent_id +
// kv keys. The child-process boundary is required because
// `fc_trace_enabled()` caches the env var read in a `OnceLock` — once
// the parent test binary has read it (cold path: not set), no in-test
// env mutation can flip the gate. A subprocess gives a fresh OnceLock.
//
// This is the only behavioral witness the production wiring depends
// on: every FC anchor site emits via `emit_event`, and `emit_event`
// is exercised here under the gate-on path.

use std::io::Write;
use std::process::Command;

fn cargo_bin_dir() -> std::path::PathBuf {
    // The integration test binary lives at target/<profile>/deps/...;
    // examples are siblings under target/<profile>/examples. We want
    // `cargo run --example fc_trace_emit_one` so we just use cargo.
    std::env::current_dir().unwrap()
}

#[test]
fn fc_trace_file_receives_well_formed_json_event() {
    let tmpdir = std::env::temp_dir().join(format!(
        "fc_trace_smoke_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos()).unwrap_or(0)
    ));
    std::fs::create_dir_all(&tmpdir).expect("mkdir tmp");
    let trace_path = tmpdir.join("fc_trace.jsonl");

    // Write a tiny Rust harness that links against minif2f_v4 and
    // emits one event. Using `cargo run` directly is too slow; we
    // instead build a one-shot binary via `cargo build --bin` of an
    // example file we drop into the experiments crate. To stay
    // self-contained and avoid touching the workspace's bin set, we
    // spawn the test process to call its OWN wrapper via env injection
    // and re-exec.

    // Fallback approach: invoke the published evaluator binary in dry
    // mode. We don't have that. Simplest working pattern: shell out
    // to `cargo run -p minif2f_v4 --quiet --example fc_trace_emit_one`
    // which we provide as a sibling example file.

    let status = Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "-p",
            "minif2f_v4",
            "--example",
            "fc_trace_emit_one",
        ])
        .env("FC_TRACE", "1")
        .env("FC_TRACE_FILE", &trace_path)
        .current_dir(cargo_bin_dir())
        .status()
        .expect("spawn fc_trace_emit_one example");
    assert!(status.success(), "fc_trace_emit_one example must exit 0");

    let contents = std::fs::read_to_string(&trace_path)
        .expect("FC_TRACE_FILE must exist after emit");
    let lines: Vec<&str> = contents
        .lines()
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(lines.len(), 1, "exactly one event was emitted; file:\n{}", contents);
    let line = lines[0];

    // Must be valid JSON.
    let v: serde_json::Value =
        serde_json::from_str(line).expect("emitted line must be valid JSON");

    // Stable fields per fc_trace.rs event shape.
    assert!(v.get("ts_ms").and_then(|x| x.as_u64()).is_some());
    assert_eq!(
        v.get("fc_id").and_then(|x| x.as_str()),
        Some("FC2-N22"),
        "fc_id must be the stable string from FcId::as_str"
    );
    assert_eq!(
        v.get("run_id").and_then(|x| x.as_str()),
        Some("smoke_run_001")
    );
    assert_eq!(v.get("tx").and_then(|x| x.as_u64()), Some(42));
    assert_eq!(
        v.get("agent_id").and_then(|x| x.as_str()),
        Some("Agent_2")
    );
    let kv = v.get("kv").expect("kv block present");
    assert_eq!(
        kv.get("reason").and_then(|x| x.as_str()),
        Some("OmegaAccepted")
    );
    assert_eq!(kv.get("gp_nodes").and_then(|x| x.as_u64()), Some(7));

    // Cleanup.
    let _ = std::fs::remove_dir_all(&tmpdir);
    // Suppress unused-import lint when no Write needed in path above.
    let _ = std::io::sink().write_all(b"");
}

```

## experiments/minif2f_v4/tests/trust_root_immutability.rs

```rust
// PPUT-CCL Phase B7 — Trust Root immutability (PREREG § 1.8 + § 7 Gate B).
//
// Boot computes SHA-256 of every Trust Root file at process start and
// compares against the genesis_payload.toml [trust_root] manifest. Any
// mismatch = `TRUST_ROOT_TAMPERED` abort.
//
// Trust Root manifest (PREREG § 1.8 + audit additions through 2026-04-25):
//   src/main.rs                                       (audit-fix Q2.b)
//   src/kernel.rs
//   src/wal.rs
//   src/bus.rs
//   src/drivers/llm_http.rs                           (B2-B4 audit add)
//   src/sdk/prompt_guard.rs                           (B6 add)
//   Cargo.lock                                        (audit-fix Q2.e)
//   experiments/minif2f_v4/src/lean4_oracle.rs
//   experiments/minif2f_v4/src/cost_aggregator.rs     (B2)
//   experiments/minif2f_v4/src/wall_clock.rs          (B3)
//   experiments/minif2f_v4/src/post_hoc_verifier.rs   (B4)
//   experiments/minif2f_v4/src/jsonl_schema.rs        (B1)
//   experiments/minif2f_v4/src/rollback_sim.rs        (B7-extra)
//   experiments/minif2f_v4/src/agent_models.rs        (Phase A atom A3)
//   experiments/minif2f_v4/src/bin/evaluator.rs       (the wiring)
//   constitution.md
//   handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json
//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md
//   handover/preregistration/scripts/run_p0_calibration.sh   (audit-fix Q2)
//   handover/preregistration/scripts/compute_p0.py           (audit-fix Q2)
//   cases/MANIFEST.sha256                             (proxy for cases/*.yaml)

use std::fs;
use std::path::{Path, PathBuf};
use turingosv4::boot::{parse_trust_root_section, verify_trust_root, TrustRootError};

fn repo_root() -> PathBuf {
    // CARGO_MANIFEST_DIR for this test crate is experiments/minif2f_v4 — repo
    // root is two levels up.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("repo root resolves")
}

fn read_genesis() -> String {
    fs::read_to_string(repo_root().join("genesis_payload.toml")).expect("genesis exists")
}

#[test]
fn test_trust_root_immutable_at_boot() {
    // Cold-start with intact files: Boot computes SHA-256s, all match
    // genesis manifest, process continues. No abort.
    verify_trust_root(&repo_root()).expect("intact repo Trust Root verifies");
}

#[test]
fn test_trust_root_simulated_write_aborts() {
    // Simulated tampering: build a self-contained fake-repo in a tempdir
    // with a single Trust Root entry whose recorded hash does not match
    // the file content; assert verify_trust_root returns Tampered.
    let tmp = make_tempdir("trust_root_tamper");
    let zero_hash = "0".repeat(64);
    let genesis = format!(
        "[pput_accounting_0]\nschema_version = \"1.0\"\n\n[trust_root]\n\"only.txt\" = \"{zero_hash}\"\n"
    );
    fs::write(tmp.join("genesis_payload.toml"), genesis).unwrap();
    fs::write(tmp.join("only.txt"), "tampered content").unwrap();

    match verify_trust_root(&tmp) {
        Err(TrustRootError::Tampered { path, expected, actual }) => {
            assert!(path.ends_with("only.txt"));
            assert_eq!(expected, zero_hash);
            assert_ne!(actual, expected);
        }
        other => panic!("expected Tampered, got {other:?}"),
    }
}

#[test]
fn test_trust_root_manifest_includes_b2_b4_files() {
    // Mid-term audit recommendation: B2 (cost_aggregator), B3 (wall_clock),
    // B4 (post_hoc_verifier), B1 (jsonl_schema), evaluator.rs, llm_http.rs
    // MUST be in the Trust Root manifest. B6 added prompt_guard.rs.
    let entries = parse_trust_root_section(&read_genesis()).expect("parse trust_root");
    let keys: Vec<&str> = entries.iter().map(|(k, _)| k.as_str()).collect();

    let required = [
        // PREREG § 1.8 base
        "src/kernel.rs",
        "src/wal.rs",
        "src/bus.rs",
        "experiments/minif2f_v4/src/lean4_oracle.rs",
        "constitution.md",
        "cases/MANIFEST.sha256",
        "handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json",
        "handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md",
        // Mid-term audit accounting layer
        "src/drivers/llm_http.rs",
        "experiments/minif2f_v4/src/cost_aggregator.rs",
        "experiments/minif2f_v4/src/wall_clock.rs",
        "experiments/minif2f_v4/src/post_hoc_verifier.rs",
        "experiments/minif2f_v4/src/jsonl_schema.rs",
        "experiments/minif2f_v4/src/bin/evaluator.rs",
        // B6 add
        "src/sdk/prompt_guard.rs",
        // B7-extra add
        "experiments/minif2f_v4/src/rollback_sim.rs",
        // Phase A atom A3: per-agent AGENT_MODELS env var resolver
        "experiments/minif2f_v4/src/agent_models.rs",
        // Phase A atom A5: budget regime + MAX_TRANSACTIONS resolver
        "experiments/minif2f_v4/src/budget_regime.rs",
        // Phase A atom A6: FC-trace structured-event meta-witness
        "experiments/minif2f_v4/src/fc_trace.rs",
        // Phase A atom A7: heterogeneous-LLM provider plumbing (proxy + smoke)
        "src/drivers/llm_proxy.py",
        "scripts/smoke_siliconflow.sh",
        "scripts/_smoke_siliconflow.py",
        // 2026-04-25 dual-audit fixes
        "src/main.rs",
        "Cargo.lock",
        "handover/preregistration/scripts/run_p0_calibration.sh",
        "handover/preregistration/scripts/compute_p0.py",
        // 2026-04-25 Phase A0 harness modernization
        "rules/MANIFEST.sha256",
        "rules/engine.py",
        ".claude/hooks/judge.sh",
        "tests/fc_alignment_conformance.rs",
        // 2026-04-25 Phase A1 PREREG amendment
        "handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md",
    ];

    for path in required {
        assert!(
            keys.contains(&path),
            "Trust Root manifest missing required path: {path}\nactual keys: {keys:#?}"
        );
    }
}

#[test]
fn test_pput_accounting_0_section_present() {
    // genesis_payload.toml must contain [pput_accounting_0] with the PREREG
    // § 1.8 keys.
    let genesis = read_genesis();
    let body = extract_section(&genesis, "pput_accounting_0").expect("section present");
    let body = body.as_str();

    let required_keys = [
        "schema_version",
        "progress_definition",
        "cost_definition",
        "time_definition",
        "verified_predicate",
        "heldout_sealed_hash",
        "source_pool_sha256",
        "baseline_regression_rate",
        "baseline_regression_jsonl_sha256",
        "k_max",
        "n_max",
    ];
    for key in required_keys {
        let needle = format!("{key} =");
        assert!(
            body.contains(&needle),
            "[pput_accounting_0] missing key: {key}"
        );
    }

    // Frozen invariants from PREREG § 1.8: heldout sealed hash, k_max, n_max.
    assert!(body.contains(
        "\"51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b\""
    ), "heldout_sealed_hash diverges from PREREG § 2.3");
    assert!(body.contains("k_max = 10"), "k_max must be 10 per PREREG");
    assert!(body.contains("n_max = 34"), "n_max must be 34 per PREREG");
}

// --- helpers ---

fn extract_section(text: &str, name: &str) -> Option<String> {
    // Line-anchored scan: skip commented-out section headers (e.g. inside
    // the file's leading docstring) and only match real headers in column 0.
    let mut in_section = false;
    let mut body = String::new();
    let target = format!("[{name}]");
    for line in text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            continue;
        }
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_section = trimmed == target;
            continue;
        }
        if in_section {
            body.push_str(line);
            body.push('\n');
        }
    }
    if body.is_empty() {
        None
    } else {
        Some(body)
    }
}

fn make_tempdir(tag: &str) -> PathBuf {
    let pid = std::process::id();
    let nano = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir: PathBuf = std::env::temp_dir().join(format!("turingosv4-{tag}-{pid}-{nano}"));
    fs::create_dir_all(&dir).unwrap();
    let _: &Path = dir.as_path();
    dir
}

```

## experiments/minif2f_v4/examples/fc_trace_emit_one.rs

```rust
// Phase A atom A6 helper — emit one fc_event for the smoke test.
// Used by tests/fc_trace_smoke.rs to exercise the FC_TRACE=1 +
// FC_TRACE_FILE=<path> code path in a fresh OnceLock state. Production
// callers go through the run_swarm wiring, not this binary.

fn main() {
    minif2f_v4::fc_trace::emit_event(
        minif2f_v4::fc_trace::FcId::Fc2N22,
        "smoke_run_001",
        Some(42),
        Some("Agent_2"),
        &[
            (
                "reason",
                minif2f_v4::fc_trace::json_str("OmegaAccepted"),
            ),
            ("gp_nodes", "7".to_string()),
        ],
    );
}

```

## handover/alignment/TRACE_MATRIX_v2_2026-04-25.md

```
# TRACE_MATRIX v2 — Constitutional Flowchart ↔ Rust Code (2026-04-25 post-A0)

**Predecessor**: `TRACE_MATRIX_v1_2026-04-25.md`
**Trigger**: Phase A0 (harness modernization) shipped:
- A0a: 4 new rules (R-014/R-015/R-018/R-019) + judge.sh constitution-special-case + R-016 fc_trace_in_commit hook (commit 2e7f75a)
- A0b: tests/fc_alignment_conformance.rs witness battery — 17 PASS + 9 ignored stubs (commit d8950ee)
- A0c: 5 new cases C-071..C-075 sediment session decisions (commit 2a65339)
- A0d (this doc): Trust Root manifest 20 → 24 (this commit); v2 documents the harness as constitutional artifact
- A4 (post-A3): decomposed metrics — `hit_max_tx`, `tactic_diversity`, `verifier_wait_ms` added as non-Optional v2 fields + `compute_tactic_diversity` helper; per-row decomposition of `solve_rate` / `tokens_per_solve` / `time_per_solve` (all derivable from existing `progress` / `total_run_token_count` / `total_wall_time_ms`). FC-trace: FC2-N22 (HALT decomposition for `hit_max_tx`) + FC1-N11 (∏p decision diversity for `tactic_diversity`) + FC1-N12 (oracle scope for `verifier_wait_ms`).
- A5 (post-A4): per-agent budget normalization — new `budget_regime` module (`BUDGET_REGIME` + `MAX_TRANSACTIONS` env vars; 4-variant enum; pure parser + scaler + env-coupled resolver); `budget_regime` + `budget_max_transactions` added as non-Optional v2 fields on `RunAggregate` and the legacy `PputResult`; loop bound at `run_swarm` switched from hardcoded `let max_transactions = 200` to `resolve_budget(n_agents)` — default (env unset) preserves Phase B baseline (`total_proposal × 200`) bit-for-bit. PREREG_AMENDMENT_p0_defer § 3 condition 3 satisfied: `MaxTxExhausted` rows now disambiguated across N values. FC-trace: FC2-N22 (HALT decomposition by budget regime) + FC1-N7 (δ instances determining the per-agent share under PerAgent regime). Trust Root manifest 25 → 26.
- A6 (post-A5): per-line FC tagging via structured JSON events — new `fc_trace` module (pure stdlib; zero new deps); `FcId` enum (FC1-N7 / FC1-N11 / FC1-N12 / FC1-E18 / FC2-N20 / FC2-N22 / FC3-N31); `fc_event!`-style `emit_event` API; `FC_TRACE=1` gate (cached in `OnceLock`); `FC_TRACE_FILE=<path>` redirects emit to file (default sink stderr). Six anchor sites wired in `run_swarm`: FC2-N22 synthetic short-circuit, FC2-N20 mr tick, FC2-N22 OMEGA full-proof, FC2-N22 OMEGA per-tactic, FC2-N22 natural MaxTxExhausted (with `budget_regime` payload), FC1-N12 verify bracket (oneshot). End-to-end smoke test exercises FC_TRACE=1 in a child process (subprocess required because `OnceLock` caches the gate-read; resolves item 7 of TRACE_MATRIX § 5 "Per-line FC tagging via tracing crate"). FC-trace: meta-witness for the 5-step compile loop (Proposal → Lean ground truth → Logging → Capability compilation → ↑H-VPPUT). Trust Root manifest 26 → 27.
- A7 (post-A6): heterogeneous-LLM provider plumbing — `src/drivers/llm_proxy.py` ported from v3 with one load-bearing v4 change (per-provider multi-key round-robin: 3 SiliconFlow keys split concurrent traffic across separate rate-limit pools, mitigating V3L-27 single-key N=30 collapse). `scripts/smoke_siliconflow.sh` + `scripts/_smoke_siliconflow.py` probe each of the 3 keys (Qwen/Qwen2.5-7B-Instruct, max_tokens=8) — A7 verified all 3 keys responding 2026-04-26 (1.5–3s latency, 33+1 tokens; round-robin distributes [2,2,2] across 6 calls). New `siliconflow:<model>` provider-prefix syntax in `detect_provider()` for unambiguous routing in `AGENT_MODELS` payloads (Phase D heterogeneous swarms). Memory `reference_siliconflow.md` records SiliconFlow as the heterogeneous-LLM lane (NOT a fallback target). FC-trace: FC1-N7 (δ/AI provider expansion — heterogeneous δ instances across SF catalog enable Phase D meta-loop). Trust Root manifest 27 → 30 (proxy + 2 smoke scripts).

**Scope**: delta from v1. Read v0 + v1 first.

---

## § 1. Status flips: 17 ⚠️ → ✅ via fc_alignment_conformance.rs witnesses

A0b added the missing `tests/fc_alignment_conformance.rs` (was only in `.claude/worktrees/phase-8a-snapshot/`). 17 ✅ rows in TRACE_MATRIX now have automated witness tests. Symbol drift is now caught at `cargo test` time, not at next dual audit.

| FC ID | v1 Status | v2 Status | Witness test |
|---|---|---|---|
| FC1-N1 (Q_t carrier) | ⚠️ | ✅ | `fc1_n1_q_state_carrier_present` |
| FC1-N4 (tape) | ⚠️ | ✅ | `fc1_n4_tape_constructible_with_time_arrow` |
| FC1-N6 (input UniverseSnapshot) | ✅ | ✅ + witness | `fc1_n6_input_universe_snapshot_present` |
| FC1-N7 (δ/AI ResilientLLMClient) | ✅ | ✅ + witness | `fc1_n7_delta_ai_client_type` |
| FC1-N8/N9/N10 (output / q_o / a_o) | ✅ | ✅ + witness | `fc1_n8_n9_n10_output_agent_output_parseable` |
| FC1-N11 (∏p production-path forbidden_pattern) | ⚠️ | ✅ | `fc1_n11_n15_e18_pi_p_zero_preserves_q_t_via_forbidden_pattern` |
| FC1-N13 (wtool bus.append) | ⚠️ | ✅ | `fc1_n13_wtool_bus_append_present` |
| FC1-N15 / E18 (∏p=0 → Q_t preserve) | ⚠️ | ✅ | `fc1_n11_n15_e18_*` (same test) |
| FC2-N20/N27 (mr tick) | ✅ | ✅ + witness | `fc2_n20_n27_tick_mr_present` |
| FC2-N22 (HALT) | ⚠️ | ✅ | `fc2_n22_halt_via_halt_and_settle` |
| FC2-N23 (HaltReason — only OmegaAccepted typed) | ✅ | ✅ + witness | `fc2_n23_event_type_omega_accepted_canonical` |
| FC3-N31 (Wal logs archive) | ⚠️ | ✅ | `fc3_n31_logs_archive_wal_present` |
| FC3-N34 (readonly guard verify_trust_root) | ✅ | ✅ + 3 witnesses | `fc3_n34_*` (3 tests) |
| FC3-N39 (Ledger log) | ✅ | ✅ + witness | `fc3_n39_log_ledger_present_and_appendable` |
| FC3-S3 (readonly subgraph manifest) | (new in v1) | ✅ | `fc3_s3_readonly_subgraph_manifest_size` (>=20 entries assertion) |
| FC3-E14 (boot panic immediate-abort) | (new in v1) | ✅ | `fc3_e14_boot_panic_immediate_abort_documented` |
| (Veto-AI Art. V.1.3 amendment) | (cases C-072) | ✅ via case-law | C-072 yaml |

## § 2. New code symbols (Phase A0–A3)

| Symbol | File | FC anchor | Status |
|---|---|---|---|
| `tests/fc_alignment_conformance.rs` (17 witness fns + 9 ignored stubs) | `tests/fc_alignment_conformance.rs` | meta-witness for FC1/FC2/FC3 ↔ symbol mapping; CLAUDE.md "Conformance tests" requirement | ✅ |
| `rules/active/R-014_trust_root_manifest_drift.yaml` | `rules/active/R-014*.yaml` | FC3-S3 readonly subgraph runtime reminder | ✅ |
| `rules/active/R-015_trace_matrix_pub_symbol.yaml` | `rules/active/R-015*.yaml` | CLAUDE.md "每个 src/ pub 符号必须映射到宪法 flowchart 元素" | ✅ |
| `rules/active/R-018_constitution_amendment_sudo.yaml` | `rules/active/R-018*.yaml` | Art. V.1.1 amendment 2026-04-25 (sudo only for constitution.md) | ✅ |
| `rules/active/R-019_model_snapshot_canonical.yaml` | `rules/active/R-019*.yaml` | FC1-N7 δ/AI canonical identity | ✅ |
| `judge.sh` constitution.md special case | `.claude/hooks/judge.sh:50-67` | FC3-N3 sudo-gate enforcement (closes silent-bypass via `*.md` skip-list) | ✅ |
| `judge.sh` R-016 fc_trace_in_commit | `.claude/hooks/judge.sh:48-56` | FC-first rule (memory feedback_fc_first_problem_handling + case C-074) | ✅ |
| `parse_swarm_condition_n` (A2) | `experiments/minif2f_v4/src/bin/evaluator.rs` | FC2-N16 InitAI orchestration entry — discriminates `oneshot` vs `n<N>` swarm code paths; FC1-N11 ∏p reached only via swarm | ✅ |
| `agent_models::{AGENT_MODELS_ENV_VAR, PHASE_D_HETERO_GATE_ENV_VAR, AgentModelsError, parse_agent_models, expand_agent_models, resolve_agent_models}` (A3) | `experiments/minif2f_v4/src/agent_models.rs` | FC1-N7 δ/AI per-agent assignment; gates Phase B+C single-model invariant (notepad F-2026-04-25-02) | ✅ |
| `RunAggregate::{hit_max_tx, tactic_diversity, verifier_wait_ms}` + `compute_tactic_diversity` (A4) | `experiments/minif2f_v4/src/jsonl_schema.rs` | FC2-N22 HALT decomposition (hit_max_tx splits natural max-tx exhaustion from OMEGA accept and from B7-extra synthetic short-circuit); FC1-N11 ∏p decision diversity (tactic_diversity = distinct/total over append+complete+step proposals); FC1-N12 oracle scope (verifier_wait_ms = cumulative Lean wall-clock per run, ≤ total_wall_time_ms by construction) | ✅ |
| `make_pput` A4 args + per-call-site verifier brackets + per-tool proposal hashing (A4) | `experiments/minif2f_v4/src/bin/evaluator.rs` | wires the 3 fields at every emit site (oneshot + swarm OMEGA + swarm step Complete + swarm synthetic short-circuit + swarm natural max-tx exhaustion); 5 unit/conformance tests (`test_a4_decomposed_metrics_round_trip`, `test_a4_tactic_diversity_helper`, `test_a4_verifier_wait_bounded_by_total_wall_time`, `test_a4_emit_max_tx_exhaustion_row`, `test_a4_synthetic_short_circuit_does_not_set_hit_max_tx`) | ✅ |
| `budget_regime::{BUDGET_REGIME_ENV_VAR, MAX_TRANSACTIONS_ENV_VAR, DEFAULT_MAX_TRANSACTIONS, BudgetRegime, BudgetError, parse_budget_regime, parse_max_transactions, effective_max_tx, resolve_budget}` (A5) | `experiments/minif2f_v4/src/budget_regime.rs` | FC2-N22 HALT decomposition by budget regime — declares which partitioning rule (`total_proposal` / `per_agent` / `token_total` / `wall_clock`) governed the loop bound. Phase A scope = first two regimes implemented; latter two declared startup-fatal `UnimplementedRegime` so a misconfigured run aborts before consuming LLM budget. PREREG_AMENDMENT_p0_defer § 3 condition 3 dependency cleared. | ✅ |
| `RunAggregate::{budget_regime, budget_max_transactions}` + `PputResult::{budget_regime, budget_max_transactions}` (A5) | `experiments/minif2f_v4/src/jsonl_schema.rs` + `experiments/minif2f_v4/src/bin/evaluator.rs` | FC2-N22: every emitted v2 row stamps the regime label + base budget so downstream PPUT analysis can join on the partitioning rule. Loop bound at `run_swarm` startup = `resolve_budget(n_agents).effective_max_tx`; default (env unset) preserves the Phase B baseline `total_proposal × 200` bit-for-bit. 16 unit tests (15 in `budget_regime::tests` + 1 `test_a5_budget_regime_round_trip` in jsonl_schema). | ✅ |
| `fc_trace::{FcId, FC_TRACE_*ENV*, fc_trace_enabled, emit_event, json_str}` (A6) | `experiments/minif2f_v4/src/fc_trace.rs` | meta-witness for FC1 / FC2 / FC3 path coverage. 7-variant `FcId` enum produces stable strings (`FC1-N7` / `FC1-N11` / `FC1-N12` / `FC1-E18` / `FC2-N20` / `FC2-N22` / `FC3-N31`) that Phase D consumers + TRACE_MATRIX rows join on. `FC_TRACE=1` gate cached in `OnceLock` (zero-overhead in production). 6 unit tests (label stability + JSON escape + cold-path no-op). | ✅ |
| `run_corr_id` correlation key + 6 wired FC events (A6) | `experiments/minif2f_v4/src/bin/evaluator.rs` | per-run correlation id (`condition + problem_id + unix-ms`) anchors all events from one run. Anchor sites: FC2-N22 synthetic short-circuit / mr tick FC2-N20 / OMEGA full-proof FC2-N22 / OMEGA per-tactic FC2-N22 / natural MaxTxExhausted FC2-N22 (with `budget_regime` payload from A5) / FC1-N12 verify bracket in oneshot. End-to-end smoke `tests/fc_trace_smoke.rs` exercises FC_TRACE=1 in a child process (forced because `OnceLock` caches the gate-read). | ✅ |
| `llm_proxy.py` v4 (multi-key round-robin) + `detect_provider` `siliconflow:` prefix (A7) | `src/drivers/llm_proxy.py` | FC1-N7 δ/AI provider expansion — three SiliconFlow keys form a 3-element round-robin pool keyed on `_per_key_requests[provider]`. Phase D heterogeneous swarms can address SF models via `AGENT_MODELS=siliconflow:Qwen/Qwen2.5-7B-Instruct,...`. Mitigates V3L-27 (case C-027) single-key N=30 401/429 collapse documented in `cases/V3_LESSONS.md`. | ✅ |
| `smoke_siliconflow.sh` + `_smoke_siliconflow.py` (A7) | `scripts/smoke_siliconflow.sh` + `scripts/_smoke_siliconflow.py` | A7 acceptance gate — 3 keys × 1 probe each (Qwen2.5-7B-Instruct, max_tokens=8). Verified all 3 SiliconFlow keys responding 2026-04-26 + proxy round-robin distributes [2,2,2] across 6 calls. PASS gates Phase D heterogeneous-swarm work. | ✅ |

## § 3. Trust Root manifest expansion: 20 → 24

Per case **C-075 (DO-178C tool qualification)**: governance instrumentation is itself constitutional; tampering with rules / judge.sh / conformance tests = silent constitutional drift.

| New entry | Why in Trust Root |
|---|---|
| `rules/MANIFEST.sha256` (proxy for 14 rules/active/R-*.yaml) | Same pattern as cases/MANIFEST.sha256: glob hashed once, manifest tracked in Trust Root. Tampering with R-018 enforcement = "warn" silently bypasses constitution sudo gate. |
| `rules/engine.py` | The interpreter of the rules. Tampering with engine.py = silent rule bypass even with intact rule files. |
| `.claude/hooks/judge.sh` | The PreToolUse hook that invokes engine.py + implements R-016 fc_trace + constitution.md special-case. Tampering = bypass entire gate stack. |
| `tests/fc_alignment_conformance.rs` | Witness battery for TRACE_MATRIX ✅ rows. Tampering = false PASS hides drift. |

**Total: 24 entries** (15 from B7 + 1 B7-extra rollback_sim + 4 dual-audit fixes + 4 A0 harness). A1 (PREREG amendment) → 25; A5 (budget_regime.rs) → 26; A6 (fc_trace.rs) → 27; A7 (llm_proxy.py + smoke_siliconflow.sh + _smoke_siliconflow.py) → 30. When B7-extra calibration eventually runs, the calibration jsonl makes 31 entries; future Phase C's `--mode` flag binary (TBD location) makes 32.

## § 4. New constitutional case-law (A0c)

5 new cases C-071..C-075 (commit 2a65339) sediment 2026-04-25 session decisions as constitutional precedent. Each cross-referenced in TRACE_MATRIX rows:

| Case | Anchors | Rules / hooks enforcing |
|---|---|---|
| C-071 constitution amendment process | Art. V.1.1 + V.3 | R-018 (BLOCK) + judge.sh special-case |
| C-072 Veto-AI scope narrowing | Art. V.1.3 | manual via dual audit; future FC3-N32 runtime |
| C-073 ArchitectAI commit authority | Art. V.1.2 | implicit via 19-commit session validation |
| C-074 FC-first problem handling | All FC + Alignment Standard | R-016 (WARN on git commit without FC-trace) |
| C-075 DO-178C tool qualification | PREREG § 1.8 + Art. V.1.1 | R-014 (warn on .rs edit) + 24-file manifest expansion |

## § 5. Open work flagged for future TRACE_MATRIX_v3

1. **TRACE_MATRIX_v?.md docs themselves** — currently NOT in Trust Root (would cause self-reference loop). Acceptable since these are documentation, not enforcement. Phase D (when ArchitectAI runtime comes online) may need to formalize doc-Trust-Root semantics.
2. **rules/SCHEMA.yaml** — defines rule format but engine.py doesn't validate against it. Lower priority; add to Trust Root if SCHEMA itself is referenced by automated tests.
3. **build-check.sh + session-end.sh** — sister hooks of judge.sh. Lower-priority gates (build verification, session telemetry); add to Trust Root in next harness cycle.
4. **R-016 fc_trace_in_commit upgrade** — currently WARN-level. If post-Phase-D evidence shows FC-trace discipline still slipping, promote to BLOCK-level.
5. **R-020 ground_truth_label** — sketched in A0a planning but not implemented (grep on PputResult/RunAggregate field additions to enforce thesis claim 7 ground-truth source). Defer to next harness cycle.
6. **FC2-N23 HaltReason full taxonomy as Rust enum** — currently only OmegaAccepted is typed; other 4 variants live as jsonl strings. Phase C+ Soft Law mode work may force this typing.
7. ~~**Per-line FC tagging via tracing crate** — Plan agent's recommendation in N-experiments brainstorm. Phase A6 deferred; will land before Phase B (homogeneous experiments).~~ **A6 LANDED** (commit pending): `fc_trace.rs` module + 6 wired anchor sites. Implementation chose pure stdlib over the `tracing` crate to avoid a new dep tree; the macro surface (`emit_event` + `FcId` enum) was kept small so Phase D+ can swap to a real `tracing-subscriber` bridge locally.

## § 6. Updated counts (v2)

Compared to v1:
- ✅ count: 16 → **33** (+17 from fc_alignment_conformance.rs witness battery; +4 from new symbols/rules; +4 from manifest expansion; +5 case-law entries; -3 stale)
- 📅/📄 count: 9 → **9** (Phase 11+ deferred unchanged; some clarified with case references)
- 🔨/⚠️ count: 0 → **0** (no actionable rows pending in v2 scope)
- New cases: 5 (C-071..C-075)
- New rules: 4 active (R-014/R-015/R-018/R-019) + 1 hook-level (R-016)

Manifest size milestones:
- B7 → 15
- B7-extra → 16
- B7-extra round-1 audit-fix → 20
- A0 (this v2) → 24
- A1 PREREG amendment → 25
- A5 budget_regime.rs → 26
- A6 fc_trace.rs → 27
- A7 llm_proxy.py + smoke_siliconflow.{sh,py} → **30**
- (planned) B7-extra calibration freeze → 31
- (planned) Phase C mode-flag binary → 32+

## § 7. Cross-references

- `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md` (immutable baseline)
- `handover/alignment/TRACE_MATRIX_v1_2026-04-25.md` (B7 + B7-extra v1)
- `handover/alignment/FC_ELEMENTS_2026-04-22.md` (canonical FC node IDs)
- `handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md` (FC3-E14 vs FC2-N22 distinction)
- `handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md` (Findings A+B)
- `handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md` (Findings C+D)
- `cases/C-071`..`C-075`.yaml (Phase A0 case-law)
- `~/.claude/.../memory/feedback_fc_first_problem_handling.md` (FC-first rule memory)

```

## genesis_payload.toml

```toml
# TuringOS v4 — Genesis payload (Phase B7).
#
# Frozen at Phase B B7 (2026-04-25). Two binding sections:
#
#   [pput_accounting_0] — semantic invariants of the PPUT measure
#                         (PREREG § 1.8). baseline_regression_rate +
#                         baseline_regression_jsonl_sha256 are placeholders
#                         until B7-extra (p_0 calibration) lands.
#
#   [trust_root]        — SHA-256 of every load-bearing file. Boot
#                         (`turingosv4::boot::verify_trust_root`) recomputes
#                         each hash and aborts with TRUST_ROOT_TAMPERED on
#                         mismatch.
#
# Manifest derivation (independently re-derived in B7 from PREREG § 1.8 +
# B2-B4 mid-term audit recommendation + B6 prompt_guard add):
#
#   PREREG § 1.8 base (8):
#     src/kernel.rs, src/wal.rs, src/bus.rs,
#     experiments/minif2f_v4/src/lean4_oracle.rs,
#     constitution.md, cases/MANIFEST.sha256 (proxy for cases/*.yaml glob),
#     handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json,
#     handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md
#
#   Mid-term audit add (PPUT accounting layer, 6):
#     src/drivers/llm_http.rs (cost source of truth),
#     experiments/minif2f_v4/src/cost_aggregator.rs (B2),
#     experiments/minif2f_v4/src/wall_clock.rs (B3),
#     experiments/minif2f_v4/src/post_hoc_verifier.rs (B4),
#     experiments/minif2f_v4/src/jsonl_schema.rs (B1),
#     experiments/minif2f_v4/src/bin/evaluator.rs (the wiring)
#
#   B6 add (1):
#     src/sdk/prompt_guard.rs (PPUT-context-leak runtime gate)
#
#   B7-extra add (1):
#     experiments/minif2f_v4/src/rollback_sim.rs (PPUT-CCL § 5.5
#       calibration treatment toggle — synthetic ∏p=0 from tx 50,
#       constitutionally FC1-E18 + FC2-N22-MaxTxExhausted)
#
#   2026-04-25 dual-audit fixes (4):
#     src/main.rs (audit Q2.b VETO — verify_trust_root call site must be
#       in manifest; comment-out = silent bypass)
#     Cargo.lock (audit Q2.e VETO — supply-chain dep-version swap defense)
#     handover/preregistration/scripts/run_p0_calibration.sh (audit Q2 +
#       DO-178C tool qualification — runner is a frozen production tool)
#     handover/preregistration/scripts/compute_p0.py (same — estimator is
#       a frozen production tool)
#
#   2026-04-25 Phase A0 harness modernization (4):
#     rules/MANIFEST.sha256 (proxy for 14 rules/active/R-*.yaml — governance
#       rules that must not be silently weakened; per case C-075 DO-178C
#       tool qualification)
#     rules/engine.py (rule engine; tampering = silent rule bypass)
#     .claude/hooks/judge.sh (PreToolUse hook that invokes engine.py +
#       implements R-016 fc_trace_in_commit + constitution.md special-case)
#     tests/fc_alignment_conformance.rs (per CLAUDE.md "每个 ✅ 行 ≥1
#       witness test"; tampering = silent constitutional drift, defeats
#       FC1/2/3 ↔ symbol mapping enforcement)
#
#   2026-04-25 Phase A1 PREREG amendment (1):
#     handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md
#       (defers PREREG § 5.5 calibration; substitutes p_0 = 0.10
#       conservative ceiling until 5 re-calibration conditions met;
#       per case C-073 ArchitectAI commit authority)
#
# Total: 25 files. genesis_payload.toml itself is conceptually frozen but
# not self-hashed (chicken-and-egg) — the [pput_accounting_0] section
# values are the semantic anchor.
#
# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.

[pput_accounting_0]
schema_version = "1.0"
progress_definition = "1 iff Lean ground-truth verifies golden_path_payload"
cost_definition = "sum(prompt_tokens + completion_tokens + tool_tokens) over all proposals in the run"
time_definition = "wall_clock from first agent prompt construction to final Lean accept or external timeout"
verified_predicate = "experiments/minif2f_v4/src/lean4_oracle.rs::verify_omega_detailed"
heldout_sealed_hash = "51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b"
source_pool_sha256 = "77179cf2598b0df707d78a6663e763121dfe8e73a6538073155f488feab95093"
baseline_regression_rate = 0.0  # PLACEHOLDER until Phase B7-extra calibration
baseline_regression_jsonl_sha256 = ""  # PLACEHOLDER until Phase B7-extra
k_max = 10
n_max = 34

[trust_root]
"src/main.rs" = "622fee2d96a980d24f9fbaab3d0531c195a0a337fc3ddd2efb60bca90a1cfbf9"
"Cargo.lock" = "577446e8fe11e91bc8751bf13e5ddca6c5faa64d3309b878768c550d3e6feb98"
"handover/preregistration/scripts/run_p0_calibration.sh" = "5f4a57dd8b8280ffe04bec89350a57d876d06cc179d9f8841a522e7bdcf1b8b7"
"handover/preregistration/scripts/compute_p0.py" = "fd1dff97d582f4f509249852be500cba7d81efc9bab8fcb9872b2d85bb4e3052"
"rules/MANIFEST.sha256" = "a84d114a12680c596e1a5458a954a5829b21baa4530f197b9aba65f95443be14"
"rules/engine.py" = "932d9a2b7a3249a7eb5825c0b5c714a9913cd9aa9e058f789e64992b140e40b3"
".claude/hooks/judge.sh" = "a2be9e6ed51e39f2e9cfd302d62a0234a772abc41f145702143d2557dd6fda3e"
"tests/fc_alignment_conformance.rs" = "b3e75979ad2d175b9c45135be6ea1d94ce95184c6896468330c12dbfc1f719db"
"src/kernel.rs" = "893fd67534caf7a3d9abd6efbd202556348b6491cd6d4c6bdb224d2ad75b1af0"
"src/wal.rs" = "1ac7637aa09531b1c7232163f5df48f7193251594c4ed20e0d0fc85cea8f84dc"
"src/bus.rs" = "df28ffe514a3272a3d10fca4568fd424a76e754e9785c109a5459f163f7fd14c"
"src/drivers/llm_http.rs" = "615596b68956b4a8925110edc99aa746a5543527724ec394bb9dda163147ed7a"
"src/drivers/llm_proxy.py" = "c1bc508c64e39fbaad246b2c36781e8f37bc216c6d8a207ee25f37c2e8b13fcb"
"scripts/smoke_siliconflow.sh" = "6ad54e7c0ab8221f475360dcad80eeeb0d6da0fd55c27acdb1cefb2b390f5341"
"scripts/_smoke_siliconflow.py" = "778eea2988312f250efa47fcfe620d86187d01b96f07a98501f9795a333726ca"
"src/sdk/prompt_guard.rs" = "b4f7b164770d1a7203b8143f773c66f748994d60a42ece38471f2f7f2839f4f1"
"experiments/minif2f_v4/src/lean4_oracle.rs" = "70fae24cd17f410c10a092e797fcdedea962db3d7cb20f218d02303edae9e98c"
"experiments/minif2f_v4/src/cost_aggregator.rs" = "896b6905dbca9e9736f8896cd5725c16b6e87c6ad3ff822e044975febed46a03"
"experiments/minif2f_v4/src/wall_clock.rs" = "2c9197f8f93b7d130dc7b094a6664f8ece351ec85668921c5beb6d100a1a77ee"
"experiments/minif2f_v4/src/post_hoc_verifier.rs" = "9a93ae5548827b60543df779c67a5f1201b49a681a55e2cbc6a8532e8bb131a2"
"experiments/minif2f_v4/src/jsonl_schema.rs" = "5d0c5a2bcceba42abbc6f5cd58e045e2d5d7a9edc565db4515f3917a21dc520b"
"experiments/minif2f_v4/src/rollback_sim.rs" = "9ccdf177813452bb83221bd780f2f7f10da619117aca8e44d3ecb355505fe561"
"experiments/minif2f_v4/src/agent_models.rs" = "b736b91efb8a93d884b64028da79d5d6af0525e8c7e8f7222055118f43dc26ce"
"experiments/minif2f_v4/src/budget_regime.rs" = "e0c2ba6533444e6178d9e12d0f182d29c3203a929ea0914d3098efd0397948ea"
"experiments/minif2f_v4/src/fc_trace.rs" = "8578edf61148e50abdf8d29a2efdc5114ee20ab727c1ccd6c5a88ad07e7ffa09"
"experiments/minif2f_v4/src/bin/evaluator.rs" = "149f058b14e0ca414ffe57143fc7a7744b997242fe65cd078596d1b2f9d20422"
"constitution.md" = "91e5913603eebaf134ac93d0f9ed7bb2b7916132203f6c2fae84d73329056ad6"
"cases/MANIFEST.sha256" = "dbe48bc11cac18ad1164c55b11aefd5be4a7b546e31c3df08fb4403c7f41e90d"
"handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json" = "69be702e3e0a4bb33c62bd02056495309289fd9271972dc733b4a1ba3dd908e1"
"handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" = "64d565b4451d1f770425530bb7ffa874da602ddfd9d714ce7f7db6bbfab69463"
"handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md" = "737af38d4b5690d2ac0b3f818ea90ce0e2142fe2d4e444ce44819eadaa6c31d0"

```

---

Give your INDEPENDENT Phase A → B exit audit. Cite §/file:line for every finding.

codex
I’ll treat the packet as the contract, but I’m going to verify the repo state directly where the questions depend on commits, line-level wiring, tests, and Trust Root. First I’m checking the local instructions and repository status so the audit is grounded in the actual checkout.
exec
/bin/bash -lc 'git status --short' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
?? handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26.md

exec
/bin/bash -lc pwd in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4

exec
/bin/bash -lc 'find .. -name AGENTS.md -print' in /home/zephryj/projects/turingosv4
 succeeded in 1179ms:
../turingos/AGENTS.md
../omega_kepler_discovery/AGENTS.md
../omega/AGENTS.md
../turingosv2/AGENTS.md
../turingclaw/AGENTS.md

codex
The worktree has one untracked audit markdown file and no AGENTS.md under this repo path from `find ..`; I’ll leave the untracked file alone. Next I’m checking the atom commits and then I’ll run the requested test gates.
exec
/bin/bash -lc 'git log --oneline --decorate -n 20' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
60292dc (HEAD -> main) A8 prep: Phase A→B exit audit packet + Codex/Gemini runners
90953d6 A7: SiliconFlow heterogeneous-LLM provider plumbing (proxy + 3-key smoke)
89994c7 A6: per-line FC tagging via structured JSON events (fc_trace module)
30f2a14 A5: per-agent budget normalization (BUDGET_REGIME + MAX_TRANSACTIONS env vars)
a5c78e4 A4: decomposed metrics (hit_max_tx + tactic_diversity + verifier_wait_ms)
7f4bc0c A3: per-agent AGENT_MODELS env var (Phase B+C single-model gate)
180a300 A2: P0a swarm_N=1 mode + parse_swarm_condition_n unit tests
6be6eb4 A1: PREREG amendment defer p_0 calibration + Trust Root 24->25
62c4e14 A0e: Phase A0 exit audit (CHALLENGE/CHALLENGE) + 7-item fixes
e94e1b9 A0d: TRACE_MATRIX_v2 + Trust Root manifest 20 → 24 (harness in TR)
2a65339 A0c: 5 new cases C-071..C-075 sediment 2026-04-25 session decisions
d8950ee A0b: tests/fc_alignment_conformance.rs witness battery
2e7f75a A0a: 4 new harness rules + judge.sh constitution-special-case
650caf7 B7-extra round-4 audit: PASS/PASS — batch authorized
d0d474e B7-extra round-3 audit + fixes (Codex CHALLENGE / Gemini CHALLENGE)
584001a B7-extra round-2 audit packet (Codex VETO / Gemini PASS)
1df1f62 B7-extra round-2 fix: Codex VETO on crash-as-data + 2 minor
438a648 B7-extra code-simplifier pass (Karpathy aesthetic, 无损压缩即智能)
15b87fb B7-extra dual-audit fixes (Phase A+B+C)
c061450 Constitutional amendment 2026-04-25 (sudo)

exec
/bin/bash -lc "git show --name-only --format='COMMIT %h%n%B%nFILES' 6be6eb4 180a300 7f4bc0c a5c78e4 30f2a14 89994c7 90953d6" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
COMMIT 6be6eb4
A1: PREREG amendment defer p_0 calibration + Trust Root 24->25

FC-trace: PREREG (5.5 deferral; protocol unchanged) + Art. V.1.2
          (ArchitectAI commit authority on non-constitution PREREG) +
          C-073 + C-075 (tool qualification scope)

Phase A engineering atom 1 of 8 per the 2026-04-25 auto-research plan.

WHY DEFER

PREREG (5.5 estimated 576-run calibration as 8 hours, $3-5. Empirical
observation 2026-04-25 (during launched batch 650caf7 era):
- Per-run wall-clock ~15-30 min (was estimated ~50s) — 30-40x off
- Total batch 3-7 days realistic (was estimated 8h) — 9-21x off
- Cost still cheap (~$15-20)

The 8h estimate was based on chat-oneshot speed assumption that
proved wrong for swarm n3 on adaptation-144 problem mix (many aime
problems hit max_transactions=200 ceiling).

User mid-session 2026-04-25 explicitly questioned 576-run necessity
given multiple unresolved engineering questions (N-agents to PPUT
relationship, swarm_N=1 vs oneshot calibration, ground-truth
feedback pipeline).

OPERATIVE SUBSTITUTION

For Phase B->C transition + Phase E Gate H requirements:
- Take PREREG (5.5 ceiling itself = 0.10 as conservative upper
  bound. Strictest possible substitute when no calibrated tighter
  value exists. Mathematically conservative: artifacts must clear
  strictest plausible bar.
- genesis_payload.toml [pput_accounting_0].baseline_regression_rate
  remains 0.0 placeholder; Gate H consumers MUST hardcode 0.10 at
  consumption site, not read from genesis until calibration runs.
- baseline_regression_jsonl_sha256 stays empty.

5 RE-CALIBRATION PRECONDITIONS (lift deferral when ALL met):
1. N-experiments arc Phase A-D complete
2. swarm_N=1 mode landed
3. Per-agent budget normalization landed
4. Heterogeneous LLM agents experiment complete
5. Phase D ArchitectAI runtime exists

WHAT THIS DOES NOT CHANGE

- PREREG (5.5 protocol itself unchanged — calibration procedure
  intact for IF it ever runs
- PREREG (1.8 Trust Root composition (manifest entries unchanged
  by this amendment; amendment doc itself ADDED per (7)
- PREREG (5.4 j-RR <= p_0 guardrail logic unchanged; just p_0
  source changes
- All other PREREG (sections unchanged
- PREREG_PPUT_CCL_2026-04-26.md NOT EDITED (immutable; amendment
  is separate doc per CLAUDE.md Common Law pattern)

TRUST ROOT IMPACT

Add amendment doc to manifest:
- handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md
  (sha256 737af38d4b5690d2ac0b3f818ea90ce0e2142fe2d4e444ce44819eadaa6c31d0)
- Manifest size: 24 -> 25 entries
- experiments/minif2f_v4/tests/trust_root_immutability.rs extended
  to assert presence

DEFERRED AUDIT

Per case C-073 ArchitectAI commit workflow + CLAUDE.md "(merge /
phase decisions dual-audit": this amendment is INTERNAL Phase A
step, not phase boundary. Audit deferred to A8 Phase A->B exit
gate. Internal-step amendments don't trigger immediate dual-audit
overhead.

If audit later returns VETO/CHALLENGE on this amendment: revert
or fix per audit feedback.

VERIFICATION

cargo test --workspace: 213 PASS + 29 ignored (unchanged from A0e).
boot::tests::verify_trust_root_passes_on_intact_repo: PASS at
25-entry manifest.

Next: A2 (P0a swarm_N=1 mode in evaluator).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

FILES

experiments/minif2f_v4/tests/trust_root_immutability.rs
genesis_payload.toml
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md
COMMIT 180a300
A2: P0a swarm_N=1 mode + parse_swarm_condition_n unit tests

FC-trace: FC2-N16 InitAI orchestration (oneshot vs swarm discriminator)
          + FC1-N11 (swarm path traverses ∏p product; oneshot does not)

Phase A engineering atom 2 of 8 per the 2026-04-25 auto-research plan.

KEY FINDING — already exists, just needs witness

CONDITION=n1 already routes to run_swarm with n_agents=1 in the
existing generic n* parser (evaluator.rs:226 pre-fix). Effectively
swarm_N=1 mode has been available since the original n* code path
landed. A2 deliverable is the WITNESS that this routing is testable
+ documented as the correct N=1 baseline.

CHANGES

1. Refactored inline parser to pub(crate) parse_swarm_condition_n
   helper function with explicit doc-comment FC-trace.
   - Returns Some(N) for "n<digits>" with N >= 1
   - Returns None for oneshot, hybrid_v1, malformed inputs
   - Filters n0 (zero agents = degenerate) to None

2. Added 5 unit tests in inline cfg(test) mod swarm_condition_tests:
   - parses_valid_n_swarm_conditions (n1, n3, n8, n16, n100)
   - rejects_oneshot_condition (the critical discriminator)
   - rejects_hybrid_v1_and_other_named_conditions
   - rejects_malformed_n_conditions (empty, prefix-only, n-1, n0,
     case-sensitive, whitespace, missing-prefix)
   - n1_is_distinct_from_oneshot (the witness)

3. Updated match arm to call parse_swarm_condition_n (functionally
   equivalent to inline closure but testable).

4. Inline doc-comment in match arm explains the N-experiments arc
   significance: every N-curve experiment must use n1 not oneshot
   to avoid code-path confound (per Plan-agent NEXT-1, Codex E0,
   Gemini E1-Prime brainstorm consensus).

WHY THIS MATTERS FOR PHASE A1-A8 + N-EXPERIMENTS

Without explicit verification that n1 routes to swarm code:
- Q1 (same-difficulty PPUT ceiling) is uninterpretable: would
  N=1 (oneshot) vs N=3 (swarm) measure agent-count effect or
  runtime-architecture effect?
- Q2 (difficulty x N): same confound
- Q3 (throttle regime): same confound

With this witness:
- swarm_N=1 (CONDITION=n1) shares all swarm code paths
  (max_transactions, agent loop, mr ticks, ∏p product, Lean oracle)
  but with single agent, providing the apples-to-apples N=1
  baseline.

TRUST ROOT IMPACT

evaluator.rs SHA-256 recomputed:
  729a719a... -> d344a0506302f8abc5aa199c47dd31637245344f170431625ed8c9c9c4286815
genesis_payload.toml updated.

VERIFICATION

cargo test --workspace: 218 PASS (was 213; +5 swarm_condition_tests)
                        + 29 ignored
boot intact-repo verify: PASS at 25-entry manifest.

Next: A3 (P0e per-agent AGENT_MODELS env var support).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

FILES

experiments/minif2f_v4/src/bin/evaluator.rs
genesis_payload.toml
COMMIT 7f4bc0c
A3: per-agent AGENT_MODELS env var (Phase B+C single-model gate)

FC-trace: FC1-N7 (δ/AI canonical identity) + Phase B+C single-model
          invariant (notepad F-2026-04-25-02 + memory feedback_phased_checkpoint)

Phase A atom A3 — capability for binding distinct δ per Agent_i in the
swarm path. Default behavior (env unset) broadcasts the global model
from ACTIVE_MODEL to every agent_idx, preserving Phase B current
behavior bit-for-bit. Heterogeneous payloads (≥2 distinct models in
the resolved vector) are gated by PHASE_D_HETERO_OK=1; without the
gate the resolver aborts at startup before any LLM call.

NEW MODULE
- experiments/minif2f_v4/src/agent_models.rs:
  - parse_agent_models(env_str) — pure CSV parser, empty/blank → []
  - expand_agent_models(parsed, global, n_agents, hetero_gated) — pure
    validator + expander; broadcast/positional + Phase B/C gate
  - resolve_agent_models(global, n_agents) — env-coupled wrapper
  - AgentModelsError {LengthMismatch, EmptyEntry, HeterogeneousWithoutGate}
  - 10 unit tests (broadcast / positional / length mismatch / empty
    entry / hetero-gated / hetero-ungated / uniform-length-N)

WIRING (run_swarm only — run_oneshot is single-call)
- evaluator.rs: resolve once at run_swarm entry, fail-fast on error.
- Per-tx: GenerateRequest.model = agent_models[agent_idx].clone()
- max_toks heuristic switches per agent (chat vs reasoner ceilings).
- run_model_label stamped on jsonl: uniform → single canonical name;
  hetero → "hetero:m1|m2|..." for downstream PPUT analysis.

TRUST ROOT (Art. V.1.1 + R-014 + case C-075)
- genesis_payload.toml [trust_root]: + agent_models.rs (24→25 entries
  pre p_0 calibration); evaluator.rs SHA-256 recomputed.
- trust_root_immutability test extends required-list assertion.

ALIGNMENT (R-015)
- TRACE_MATRIX_v2 § 2 widened to "Phase A0–A3" with rows for
  parse_swarm_condition_n (A2) + agent_models module (A3).
- Per-pub-symbol /// TRACE_MATRIX FC1-N7 doc-comments on every const,
  enum, and fn in agent_models.rs.

VERIFICATION
- cargo test --workspace: 229 PASS (was 218) + 29 ignored (unchanged).
  Net +11 from agent_models unit tests + extended manifest assertion.

Next: A4 — decomposed metrics (solve_rate, tokens_per_solve,
time_per_solve, hit_max_tx, tactic_diversity, verifier_wait).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

FILES

experiments/minif2f_v4/src/agent_models.rs
experiments/minif2f_v4/src/bin/evaluator.rs
experiments/minif2f_v4/src/lib.rs
experiments/minif2f_v4/tests/trust_root_immutability.rs
genesis_payload.toml
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md
COMMIT a5c78e4
A4: decomposed metrics (hit_max_tx + tactic_diversity + verifier_wait_ms)

Phase A engineering atom 4 of 8 — per Codex/Gemini N-agents brainstorms 2026-04-25:
PPUT alone conflates solve_rate × tokens_per_solve × time_per_solve. Decompose at the
emit boundary so downstream analysis can split the three components and surface
mechanism-level effects (Amdahl/USL serial-vs-parallel, search-party diversity,
budget-exhaustion vs accept).

Three new non-Optional v2 fields on RunAggregate + PputResult:

- hit_max_tx (bool): true iff the run reached max_transactions WITHOUT OMEGA AND
  without firing the B7-extra synthetic short-circuit. Disambiguates natural budget
  exhaustion (FC2-N22 HALT via MaxTxExhausted) from OMEGA accept at the same
  tx_count and from the calibration-treatment short-circuit (which exits ~150 tx
  EARLY at the rollback threshold; tagged via synthetic_short_circuit instead).

- tactic_diversity (f64 ∈ [0, 1]): distinct-payload-hash / total over every parsed
  append/complete/step proposal in the run. Cheap proxy for the semantic-diversity
  metric the brainstorms proposed; embedding distance is Phase D+ work. Generalizes
  the existing C-036 unique_payload_ratio (which only counts OMEGA attempts).

- verifier_wait_ms (u64): cumulative wall-clock inside Lean (verify_omega /
  verify_omega_detailed / verify_partial). Strict sub-interval of total_wall_time_ms
  by construction. Enables the Amdahl/USL decomposition Codex § C proposed.

solve_rate / tokens_per_solve / time_per_solve are aggregate-derived from existing
fields (progress, total_run_token_count, total_wall_time_ms) — documented but not
new columns; the Gemini brainstorm § A.3 (PPUT decomposition) is a downstream
analysis pattern, not a per-row schema change.

Wiring (every make_pput call site):
- run_oneshot (4 sites): hit_max_tx=false, 1/1 proposals, verifier_wait_ms from
  verify_omega bracket
- run_swarm OMEGA accept: hit_max_tx=false, distinct/total from accumulator
- run_swarm step Complete: same
- run_swarm synthetic short-circuit: hit_max_tx=false (exits EARLY)
- run_swarm natural max-tx exhaustion: hit_max_tx=true (canonical site)

Tests +5 (229 → 234 PASS, 29 ignored unchanged):
- jsonl_schema: test_a4_decomposed_metrics_round_trip / test_a4_tactic_diversity_helper
  / test_a4_verifier_wait_bounded_by_total_wall_time
- evaluator v2_emit_tests: test_a4_emit_max_tx_exhaustion_row /
  test_a4_synthetic_short_circuit_does_not_set_hit_max_tx

Trust Root manifest: jsonl_schema.rs + evaluator.rs hashes refreshed (per R-014:
Trust Root files mutated → manifest re-hash mandatory).

TRACE_MATRIX_v2 § 2 widened with A4 row covering schema + helper + emit wiring;
header bullet added under "Phase A0–A3" → now Phase A0–A4.

Phase A→B gate remains at A8.

FC-trace: FC2-N22 (HALT decomposition) + FC1-N11 (∏p decision diversity) + FC1-N12 (oracle scope) + R-014 (Trust Root manifest)

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

FILES

experiments/minif2f_v4/src/bin/evaluator.rs
experiments/minif2f_v4/src/jsonl_schema.rs
genesis_payload.toml
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md
COMMIT 30f2a14
A5: per-agent budget normalization (BUDGET_REGIME + MAX_TRANSACTIONS env vars)

FC-trace: FC2-N22 (HALT decomposition by budget regime) + FC1-N7 (delta
instances determining the per-agent share under PerAgent regime).

PREREG_AMENDMENT_p0_defer 2026-04-25 sec 3 condition 3 satisfied: every
emitted v2 row now stamps the budget partitioning rule + base, so
MaxTxExhausted rows are unambiguous across N values when calibration
eventually runs. Default (env unset) preserves Phase B baseline
(total_proposal x 200) bit-for-bit.

New module experiments/minif2f_v4/src/budget_regime.rs:
- BUDGET_REGIME env var (total_proposal | per_agent | token_total |
  wall_clock); default total_proposal.
- MAX_TRANSACTIONS env var; default 200.
- 4-variant enum + pure parse_budget_regime / parse_max_transactions /
  effective_max_tx + env-coupled resolve_budget.
- Phase A scope = total_proposal + per_agent implemented; latter two
  declared startup-fatal UnimplementedRegime so a misconfigured run
  aborts before any LLM call.
- Per-agent regime: effective_max_tx = base x n_agents
  (Codex/Gemini brainstorm A.3 "N x tx = constant" reframed as
  constant per-agent).

Wiring:
- run_swarm: hardcoded let max_transactions = 200 replaced with
  resolve_budget(n_agents). Errors abort startup.
- make_pput: new args (budget_regime, budget_max_transactions); all 9
  call sites updated (4 oneshot + 4 swarm + 4 in-test fixtures).
- run_oneshot: stamps total_proposal + base=1 (single LLM call).

Schema v2:
- RunAggregate + PputResult: budget_regime: String,
  budget_max_transactions: u32 (both non-Optional).
- Stable label strings; downstream PPUT analysis joins on these.

Tests (+20 vs previous count, 254 PASS / 29 ignored / 0 failed):
- 15 budget_regime unit tests (parse / scaling / overflow / unknown
  regime / unimplemented regime / labels / n=0 safety).
- 4 env-coupled resolve_budget tests under static Mutex per memory
  feedback_env_var_test_lock (default preserves baseline; per_agent
  via env scales to base x N; token_total startup-fatal; unknown
  regime startup-fatal).
- 1 jsonl_schema test_a5_budget_regime_round_trip (per_agent regime
  serializes / deserializes through the v2 schema).

Trust Root:
- Re-hashed jsonl_schema.rs + evaluator.rs.
- Added experiments/minif2f_v4/src/budget_regime.rs to manifest.
- trust_root_immutability test required-paths list extended.
- Manifest size 25 -> 26.

TRACE_MATRIX_v2 sec 2: A5 row added covering budget_regime module +
RunAggregate fields + make_pput threading + 16 unit tests; sec 3
manifest size milestones updated (A1: 25, A5: 26, B7-extra calibration:
27, Phase C mode-flag binary: 28+).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

FILES

experiments/minif2f_v4/src/bin/evaluator.rs
experiments/minif2f_v4/src/budget_regime.rs
experiments/minif2f_v4/src/jsonl_schema.rs
experiments/minif2f_v4/src/lib.rs
experiments/minif2f_v4/tests/trust_root_immutability.rs
genesis_payload.toml
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md
rules/enforcement.log
COMMIT 89994c7
A6: per-line FC tagging via structured JSON events (fc_trace module)

FC-trace: meta-witness for FC1 / FC2 / FC3 path coverage. Closes
TRACE_MATRIX_v2 sec 5 item 7 (was the last open Phase A scope item
before A7/A8).

Codex N-agents brainstorm 2026-04-25 sec D recommended OpenTelemetry-
style structured JSONL events tagged with FC IDs so Phase D ArchitectAI
can replay the constitutional path without source access. Implementation
chose pure stdlib over the tracing crate to avoid a new dep tree; the
emit_event + FcId surface is small enough that Phase D+ can swap to a
tracing-subscriber bridge locally.

New module experiments/minif2f_v4/src/fc_trace.rs:
- 7-variant FcId enum (FC1-N7 / FC1-N11 / FC1-N12 / FC1-E18 / FC2-N20
  / FC2-N22 / FC3-N31). as_str produces dash-form labels Phase D
  consumers + TRACE_MATRIX rows join on.
- FC_TRACE=1 gate, cached in OnceLock (single env::var read at first
  call; zero per-event overhead in production where the gate is off).
- FC_TRACE_FILE=path redirects emit to file (default sink stderr).
- Hand-rolled minimal JSON encoder (json_str escapes RFC 8259 sec 7
  chars + control chars to \uXXXX).
- emit_event(fc, run_id, tx, agent_id, kv) writes one line.

Six anchor sites wired in run_swarm + run_oneshot:
- FC2-N22 synthetic short-circuit (B7-extra calibration treatment)
- FC2-N20 mr tick (clock to mr to tape per Art. IV)
- FC2-N22 OMEGA accept via full proof (gp_path = alone | tape+payload)
- FC2-N22 OMEGA accept via per-tactic Complete (gp_path = per_tactic)
- FC2-N22 natural MaxTxExhausted (carries A5 budget_regime payload)
- FC1-N12 Lean verify bracket in oneshot (verdict + elapsed_ms)

Per-run correlation: run_corr_id pinned at run_swarm entry as
condition + problem_id + unix-ms; mirrors make_pput run_id format
so Phase D joins on equality.

Tests (+7 vs A5):
- 6 unit tests in fc_trace::tests (label stability, JSON escape, cold
  path no-op, full-payload smoke).
- 1 end-to-end integration test tests/fc_trace_smoke.rs that spawns a
  child process (cargo run --example fc_trace_emit_one) with FC_TRACE=1
  + FC_TRACE_FILE=tmp; child must run in fresh process because
  fc_trace_enabled OnceLock caches the gate-read on first call.
- New helper experiments/minif2f_v4/examples/fc_trace_emit_one.rs.

Trust Root:
- Re-hashed evaluator.rs.
- Added experiments/minif2f_v4/src/fc_trace.rs to manifest.
- trust_root_immutability test required-paths list extended.
- Manifest size 26 -> 27.

TRACE_MATRIX_v2:
- Sec 1 trigger summary: A6 row added.
- Sec 2 new code symbols: 2 rows (fc_trace module + run_corr_id +
  6 wired anchor sites).
- Sec 3 manifest size milestones: A1 25, A5 26, A6 27, calibration 28,
  Phase C mode-flag binary 29+.
- Sec 5 open work item 7: struck out + cross-referenced this commit.

261 PASS / 29 ignored / 0 failed (A5 was 254 / 29; +7 A6 tests).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

FILES

experiments/minif2f_v4/examples/fc_trace_emit_one.rs
experiments/minif2f_v4/src/bin/evaluator.rs
experiments/minif2f_v4/src/fc_trace.rs
experiments/minif2f_v4/src/lib.rs
experiments/minif2f_v4/tests/fc_trace_smoke.rs
experiments/minif2f_v4/tests/trust_root_immutability.rs
genesis_payload.toml
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md
rules/enforcement.log
COMMIT 90953d6
A7: SiliconFlow heterogeneous-LLM provider plumbing (proxy + 3-key smoke)

FC-trace: FC1-N7 (delta/AI provider expansion — heterogeneous delta
instances across SiliconFlow catalog enable Phase D meta-loop).

User correction 2026-04-26: SiliconFlow is the project's heterogeneous-
LLM lane, NOT a fallback / probe-only target. Three keys provided for
high-concurrency multi-key round-robin. Memory reference_siliconflow.md
records this lesson (context-loss anti-pattern: check .env + project
files BEFORE asking the user where credentials live).

Plumbing brought into v4 from v3:
- src/drivers/llm_proxy.py — OpenAI-compatible local HTTP server with
  one load-bearing v4 change vs v3 fork: per-provider multi-key
  round-robin. PROVIDERS map now holds a list of env-var names per
  provider; clients_by_provider builds one OpenAI client per available
  key; get_client_round_robin distributes via _rr_counters mod len.
  Three SiliconFlow keys split concurrent traffic across separate
  rate-limit pools (mitigates V3L-27 / case C-027 single-key N=30
  401/429 collapse documented in cases/V3_LESSONS.md).
- detect_provider() now recognizes explicit `provider:model` syntax
  (e.g. siliconflow:Qwen/Qwen2.5-7B-Instruct) for unambiguous routing
  in AGENT_MODELS payloads.
- /stats endpoint exposes per_key_requests for observability.

Smoke probe (scripts/smoke_siliconflow.sh + _smoke_siliconflow.py):
- Direct SDK probe per key (NOT via proxy round-robin) so per-key
  verdict is unambiguous if any key is rate-limited or revoked.
- Qwen/Qwen2.5-7B-Instruct, max_tokens=8, temperature=0.0
- Reports OK / FAIL per key without printing key material.
- Verified all 3 keys responding 2026-04-26: primary 2989ms,
  secondary 1546ms, tertiary 1549ms; 33+1 tokens; content="ack".
- Proxy round-robin verified: 6 sequential calls distribute [2,2,2]
  across the 3 keys.

v4 .env mirrored from v3 .env with phase-A header (chmod 600;
gitignored verified). Keys present: SILICONFLOW_API_KEY{,_SECONDARY,
_TERTIARY}, DEEPSEEK_API_KEY, GEMINI_API_KEY, DASHSCOPE_API_KEY,
VOLCENGINE_API_KEY, NVIDIA_NIM_API_KEY.

Trust Root manifest 27 -> 30 (added llm_proxy.py + smoke_siliconflow.sh
+ _smoke_siliconflow.py). Verified by parent boot::tests::
verify_trust_root_passes_on_intact_repo + minif2f_v4 trust_root_
immutability tests.

TRACE_MATRIX_v2:
- Sec 1 trigger: A7 row added.
- Sec 2 new code symbols: 2 rows (proxy + smoke).
- Sec 3 manifest size: A1 25, A5 26, A6 27, A7 30, calibration 31,
  Phase C mode-flag binary 32+.

docs/experiments.md: env-var list extended with all 7 provider keys +
LLM_PROXY_URL; new "LLM Proxy (Phase A atom A7)" section documents
proxy invocation + provider routing; SiliconFlow model catalog notes
Qwen2.5-7B-Instruct verified; pin model_snapshot before Phase D batch.

No new Rust tests (A7 is plumbing + integration, not a Rust feature).
Acceptance gate = scripts/smoke_siliconflow.sh PASS — verified
2026-04-26 04:58 UTC. Memory reference_siliconflow.md added to
MEMORY.md index.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

FILES

docs/experiments.md
experiments/minif2f_v4/tests/trust_root_immutability.rs
genesis_payload.toml
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md
scripts/_smoke_siliconflow.py
scripts/smoke_siliconflow.sh
src/drivers/llm_proxy.py

exec
/bin/bash -lc "git show --name-only --format='COMMIT %h%n%B%nFILES' 62c4e14" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
COMMIT 62c4e14
A0e: Phase A0 exit audit (CHALLENGE/CHALLENGE) + 7-item fixes

FC-trace: FC3-S3 (readonly subgraph airtightness) + FC1-N7 (δ/AI canonical)
          + C-074 (FC-first commit discipline) + CLAUDE.md Alignment Standard

Phase A0 exit audit: BOTH Codex + Gemini returned CHALLENGE high
conviction, no VETO, both FIX-THEN-PROCEED. 7 fixes applied in same
cycle (items 1-7); 3 deferred (items L1-L3) to next harness cycle.

CONVERGENT FINDINGS (2 critical bypass paths):
- judge.sh constitution guard NOT airtight (Codex finding 1 +
  Gemini Q1.c): symlink basename bypass; empty-CONTENT bypass; Bash
  branch exits before guard so command-line mutations silently bypass
  R-018.
- R-016 fc_trace_in_commit greps COMMAND not message body; commit -F
  /tmp/msg silently bypasses (both Q1.d).

ITEM 1 — judge.sh airtight rewrite + git-prefix exemption (A0e-fix-2)
  - Hoisted constitution guard to TOP of script (before Bash branch)
  - realpath -m resolution defeats symlink basename bypass
  - Drop CONTENT-presence requirement (empty-CONTENT now blocked)
  - bash_targets_constitution greps for command-line mutation patterns
  - Skip the check on git-prefixed commands (avoids false-positive on
    quoted commit-message text mentioning the patterns literally)
  - Verified: 8 test cases pass

ITEM 2 — R-016 -F file inspection
  - Hook now extracts -F path, reads file, checks for FC-trace inside
  - Falls back to inline -m grep + interactive editor (best-effort warn)
  - WARN-level not block; user decides

ITEM 3 — Cross-crate fc_alignment_conformance.rs
  - experiments/minif2f_v4/tests/fc_alignment_conformance.rs NEW: 9 tests
  - Closes false claim in root file fc1_n12 ignored stub
  - Covers FC1-N12 Lean4Oracle + B7-extra rollback_sim + B1-B4 PPUT
    accounting layer

ITEM 4 — Strengthen weak witnesses
  - fc1_n1 / fc1_n6 / fc1_n7 / fc3_n31 upgraded from std::any::type_name
    to actual API calls. Catches behavioral regression now.

ITEM 5 — R-019 alias expansion + canonical defaults
  - R-019 grep covers all 3 deprecated aliases
  - evaluator.rs:203 default canonical name
  - run_p0_calibration.sh:43 default canonical name

ITEM 6 — R-014 alert-fatigue mitigation
  - file_glob brace-expansion attempted but Python fnmatch unsupported
  - Reverted to all-rust glob with TIGHTENED message
  - 20 -> 24 entries message updated

ITEM 7 — harness_validation.sh refresh
  - T-007: 10 -> 14 active rules
  - T-010: 35 -> 50 cases
  - 46/48 PASS (2 pre-existing fails predate A0e)

DEFERRED (next harness cycle):
- L1 trust_root_immutability.rs in Trust Root
- L2 R-018 verbatim-phrase + V.3-entry workflow per C-071
- L3 synthetic_short_circuit / AGENT_MODELS / BUDGET_MODE rules

VERIFICATION
- 213 cargo test --workspace PASS (was 204; +9 cross-crate fc_align)
  + 29 ignored
- All judge.sh test cases pass
- boot intact-repo verify PASS at 24-entry manifest

Phase A0 (harness modernization) NOW COMPLETE post-fix.
Next: Phase A engineering atoms A1-A8.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

FILES

.claude/hooks/judge.sh
experiments/minif2f_v4/src/bin/evaluator.rs
experiments/minif2f_v4/tests/fc_alignment_conformance.rs
genesis_payload.toml
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md
handover/audits/GEMINI_PHASE_A0_EXIT_AUDIT_2026-04-25.md
handover/audits/run_codex_phase_a0_exit_audit.sh
handover/audits/run_gemini_phase_a0_exit_audit.py
handover/brainstorms/CODEX_N_AGENTS_BRAINSTORM_2026-04-25.md
handover/brainstorms/GEMINI_N_AGENTS_BRAINSTORM_2026-04-25.md
handover/brainstorms/run_codex_n_agents_brainstorm.sh
handover/brainstorms/run_gemini_n_agents_brainstorm.py
handover/preregistration/scripts/run_p0_calibration.sh
rules/MANIFEST.sha256
rules/active/R-014_trust_root_manifest_drift.yaml
rules/active/R-019_model_snapshot_canonical.yaml
tests/fc_alignment_conformance.rs
tests/harness_validation.sh

exec
/bin/bash -lc "git show --name-only --format='COMMIT %h%n%B%nFILES' 60292dc" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
COMMIT 60292dc
A8 prep: Phase A→B exit audit packet + Codex/Gemini runners

FC-trace: meta (governance — Veto-AI proxy per case C-072 + dual-audit
per memory feedback_dual_audit).

Deliverables (audit not yet launched):
- handover/audits/A8_EXIT_PACKET_2026-04-26.md — self-contained packet
  recapping all 8 atoms (A0a-e + A1-A7), per-atom FC-trace map, test
  count progression (187 -> 204 -> 234 -> 254 -> 261), Trust Root
  manifest expansion (20 -> 30), 5 known limitations entering Phase B,
  7 question groups (Q1 atomicity / Q2 PREREG amendment soundness /
  Q3 budget regime / Q4 FC tracing / Q5 SiliconFlow / Q6 Trust Root /
  Q7 exit decision) totaling 23 specific check items.
- handover/audits/run_codex_phase_a8_exit_audit.sh — bash runner mirroring
  A0 pattern; appends 14 source files (PREREG amendment + 6 .rs +
  llm_proxy.py + 2 smoke + 3 tests + TRACE_MATRIX_v2 + genesis_payload).
- handover/audits/run_gemini_phase_a8_exit_audit.py — Python runner using
  Gemini 2.5 Pro generateContent API; same packet + sources.

Estimated prompt size: ~245K chars (well within Gemini 1M context).
Estimated cost: ~$15-20 across both auditors per memory feedback_dual_
audit + Phase A summary in F-2026-04-25-02. Cost-bounded by 16384 max
output tokens per side.

Audit launch is GATED on user confirmation per memory feedback_phased_
checkpoint — Phase A -> B is the canonical phase boundary that requires
explicit go-ahead. No audits invoked by this commit.

When launched (sequential to avoid colliding on disk output):
  bash handover/audits/run_codex_phase_a8_exit_audit.sh
  python3 handover/audits/run_gemini_phase_a8_exit_audit.py

Outputs land at:
  handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26.md
  handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26.md

Decision rule (per memory feedback_dual_audit_conflict): VETO > CHALLENGE
> PASS. PASS/PASS authorizes Phase B; CHALLENGE on either side requires
in-cycle fixes; any VETO blocks the gate.

Pre-flight verification:
- cargo test -p turingosv4 --lib --tests: 152 PASS / 9 ignored / 0 failed
- cargo test -p minif2f_v4 --lib --tests --bins: 109 PASS / 20 ignored / 0 failed
- Cumulative: 261 PASS / 29 ignored / 0 failed
- boot::tests::verify_trust_root_passes_on_intact_repo: PASS at 30-entry manifest
- scripts/smoke_siliconflow.sh: PASS (3/3 keys responding 2026-04-26 04:58 UTC)

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

FILES

handover/audits/A8_EXIT_PACKET_2026-04-26.md
handover/audits/run_codex_phase_a8_exit_audit.sh
handover/audits/run_gemini_phase_a8_exit_audit.py

codex
The A1–A7 commits all carry FC-trace text and none list `constitution.md`. The local branch also has a separate A8 prep commit on top that adds the audit packet/runners; I’ll account for it as audit scaffolding, not as an A1–A7 implementation atom. Running the workspace tests now.
exec
/bin/bash -lc 'cargo test --workspace' in /home/zephryj/projects/turingosv4
 succeeded in 71041ms:
   Compiling ring v0.17.14
   Compiling rustls v0.23.38
   Compiling rustls-webpki v0.103.11
   Compiling tokio-rustls v0.26.4
   Compiling hyper-rustls v0.27.8
   Compiling reqwest v0.12.28
   Compiling turingosv4 v0.1.0 (/home/zephryj/projects/turingosv4)
warning: unused import: `KernelError`
 --> src/bus.rs:8:29
  |
8 | use crate::kernel::{Kernel, KernelError};
  |                             ^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `TapeError`
 --> src/bus.rs:9:54
  |
9 | use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
  |                                                      ^^^^^^^^^

warning: unused imports: `Deserialize` and `Serialize`
  --> src/bus.rs:11:13
   |
11 | use serde::{Deserialize, Serialize};
   |             ^^^^^^^^^^^  ^^^^^^^^^

warning: unused import: `crate::sdk::snapshot::UniverseSnapshot`
 --> src/sdk/actor.rs:7:5
  |
7 | use crate::sdk::snapshot::UniverseSnapshot;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `std::collections::HashSet`
 --> src/sdk/actor.rs:9:5
  |
9 | use std::collections::HashSet;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ToolSignal`
 --> src/sdk/tools/search.rs:4:24
  |
4 | use crate::sdk::tool::{ToolSignal, TuringTool};
  |                        ^^^^^^^^^^

warning: unused import: `std::path::Path`
 --> src/sdk/tools/search.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `Deserialize`
 --> src/sdk/tools/librarian.rs:6:13
  |
6 | use serde::{Deserialize, Serialize};
  |             ^^^^^^^^^^^

warning: unused import: `Path`
 --> src/sdk/tools/librarian.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^

   Compiling minif2f_v4 v0.1.0 (/home/zephryj/projects/turingosv4/experiments/minif2f_v4)
warning: `turingosv4` (lib) generated 9 warnings (run `cargo fix --lib -p turingosv4` to apply 9 suggestions)
warning: `turingosv4` (lib test) generated 8 warnings (8 duplicates)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 24.02s
     Running unittests src/lib.rs (target/debug/deps/minif2f_v4-2121f01f9a6ef751)

running 71 tests
test agent_models::tests::empty_csv_slot_rejected ... ok
test agent_models::tests::csv_entries_trimmed ... ok
test agent_models::tests::empty_env_parses_to_empty_vec ... ok
test agent_models::tests::heterogeneous_without_gate_rejected ... ok
test agent_models::tests::empty_parsed_broadcasts_global_model ... ok
test agent_models::tests::heterogeneous_with_gate_passes ... ok
test agent_models::tests::length_mismatch_rejected ... ok
test agent_models::tests::positional_length_match_passes ... ok
test agent_models::tests::single_entry_broadcasts ... ok
test agent_models::tests::single_entry_parses ... ok
test agent_models::tests::uniform_length_n_does_not_trip_hetero_gate ... ok
test budget_regime::tests::effective_per_agent_scales_linearly_in_n ... ok
test budget_regime::tests::effective_per_agent_overflow_rejected ... ok
test budget_regime::tests::effective_token_total_unimplemented ... ok
test budget_regime::tests::effective_wall_clock_unimplemented ... ok
test budget_regime::tests::n_agents_zero_does_not_panic ... ok
test budget_regime::tests::label_strings_are_stable ... ok
test budget_regime::tests::parse_max_transactions_garbage_rejected ... ok
test budget_regime::tests::parse_max_transactions_empty_defaults_to_200 ... ok
test budget_regime::tests::effective_total_proposal_invariant_under_n ... ok
test budget_regime::tests::parse_max_transactions_negative_rejected ... ok
test budget_regime::tests::parse_max_transactions_valid ... ok
test budget_regime::tests::parse_regime_empty_defaults_to_total_proposal ... ok
test budget_regime::tests::parse_regime_known_values ... ok
test budget_regime::tests::parse_max_transactions_zero_rejected ... ok
test budget_regime::tests::parse_regime_unknown_rejected ... ok
test budget_regime::tests::resolve_budget_per_agent_via_env ... ok
test budget_regime::tests::resolve_budget_token_total_startup_fatal ... ok
test budget_regime::tests::resolve_budget_unknown_regime_via_env ... ok
test cost_aggregator::tests::test_empty_accumulator_zero_total ... ok
test budget_regime::tests::resolve_budget_default_preserves_phase_b_baseline ... ok
test cost_aggregator::tests::test_failed_branches_counted_in_total_cost ... ok
test cost_aggregator::tests::test_flip_underflow_panics - should panic ... ok
test fc_trace::tests::emit_event_with_full_payload_does_not_panic ... ok
test cost_aggregator::tests::test_tool_stdout_chars_div_4_approximation ... ok
test fc_trace::tests::fc_id_display_matches_as_str ... ok
test fc_trace::tests::emit_event_with_no_kv_or_agent_does_not_panic ... ok
test fc_trace::tests::emit_is_no_op_when_disabled ... ok
test fc_trace::tests::fc_id_strings_are_stable ... ok
test jsonl_schema::tests::test_a4_tactic_diversity_helper ... ok
test fc_trace::tests::json_str_escapes_required_chars ... ok
test jsonl_schema::tests::test_a4_verifier_wait_bounded_by_total_wall_time ... ok
test jsonl_schema::tests::test_a4_decomposed_metrics_round_trip ... ok
test jsonl_schema::tests::test_a5_budget_regime_round_trip ... ok
test jsonl_schema::tests::test_jsonl_schema_v2_round_trip ... ok
test jsonl_schema::tests::test_pput_verified_zero_when_progress_zero ... ok
test lean4_oracle::tests::test_clean_tactic_accepted ... ok
test lean4_oracle::tests::test_correct_theorem_name_accepted ... ok
test lean4_oracle::tests::test_decide_tactic_permitted ... ok
test jsonl_schema::tests::test_legacy_jsonl_still_readable ... ok
test lean4_oracle::tests::test_forbidden_io_process ... ok
test lean4_oracle::tests::test_identity_theft_rejected ... ok
test lean4_oracle::tests::test_forbidden_native_decide ... ok
test lean4_oracle::tests::test_sorry_in_word_not_rejected ... ok
test lean4_oracle::tests::test_word_boundary_function ... ok
test post_hoc_verifier::tests::test_no_runtime_accept_zeros_both_pput ... ok
test post_hoc_verifier::tests::test_post_hoc_verified_without_runtime_still_zero_progress ... ok
test lean4_oracle::tests::test_sorry_rejected ... ok
test post_hoc_verifier::tests::test_pput_verified_zero_when_lean_rejects ... ok
test rollback_sim::tests::does_not_fire_after_threshold ... ok
test post_hoc_verifier::tests::test_pput_verified_matches_runtime_when_both_accept ... ok
test rollback_sim::tests::does_not_fire_before_threshold ... ok
test rollback_sim::tests::fires_at_threshold_when_enabled ... ok
test rollback_sim::tests::never_fires_when_disabled ... ok
test rollback_sim::tests::threshold_constant_matches_prereg ... ok
test rollback_sim::tests::env_var_name_matches_prereg ... ok
test wall_clock::tests::test_wall_clock_first_read_to_final_accept ... ok
test wall_clock::tests::test_wall_clock_unmarked_returns_none ... ok
test wall_clock::tests::test_wall_clock_final_accept_overwrites ... ok
test wall_clock::tests::test_wall_clock_first_read_idempotent ... ok
test wall_clock::tests::test_wall_clock_no_final_accept_uses_now ... ok

test result: ok. 71 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running unittests src/bin/evaluator.rs (target/debug/deps/evaluator-680e8e25b06524bc)

running 9 tests
test swarm_condition_tests::n1_is_distinct_from_oneshot ... ok
test swarm_condition_tests::parses_valid_n_swarm_conditions ... ok
test swarm_condition_tests::rejects_malformed_n_conditions ... ok
test swarm_condition_tests::rejects_hybrid_v1_and_other_named_conditions ... ok
test swarm_condition_tests::rejects_oneshot_condition ... ok
test v2_emit_tests::test_a4_emit_max_tx_exhaustion_row ... ok
test v2_emit_tests::test_a4_synthetic_short_circuit_does_not_set_hit_max_tx ... ok
test v2_emit_tests::test_emit_soft_law_divergence_signal ... ok
test v2_emit_tests::test_emit_dispatches_as_v2 ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/architect_sole_lt_reader.rs (target/debug/deps/architect_sole_lt_reader-c2eeaa310f253b42)

running 3 tests
test test_architect_can_read_lt_jsonl ... ignored, Phase D — meta-loop not yet implemented
test test_architect_lt_read_is_logged ... ignored, Phase D — meta-loop not yet implemented
test test_auditor_cannot_read_lt_jsonl ... ignored, Phase D — meta-loop not yet implemented

test result: ok. 0 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/artifact_content_predicates.rs (target/debug/deps/artifact_content_predicates-ae5c6db93295a986)

running 4 tests
test test_docs_code_blocks_are_parametric_templates ... ignored, Phase D — AuditorAI artifact battery not yet implemented
test test_docs_contain_no_raw_failed_trace ... ignored, Phase D — AuditorAI artifact battery not yet implemented
test test_docs_do_not_include_exact_adaptation_solution ... ignored, Phase D — AuditorAI artifact battery not yet implemented
test test_docs_include_scope_and_expiration ... ignored, Phase D — AuditorAI artifact battery not yet implemented

test result: ok. 0 passed; 0 failed; 4 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/artifact_lookup_evasion.rs (target/debug/deps/artifact_lookup_evasion-a71491ef5129dce5)

running 4 tests
test test_docs_max_dict_cardinality ... ignored, Phase D — lookup-evasion battery not yet implemented
test test_docs_no_problem_id_keys ... ignored, Phase D — lookup-evasion battery not yet implemented
test test_docs_no_theorem_name_keys ... ignored, Phase D — lookup-evasion battery not yet implemented
test test_docs_rolling_hash_multi_window ... ignored, Phase D — lookup-evasion battery not yet implemented

test result: ok. 0 passed; 0 failed; 4 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/auditor_sees_candidate_only.rs (target/debug/deps/auditor_sees_candidate_only-a9edd1e3efd9b047)

running 3 tests
test test_auditor_has_no_raw_lt_in_context ... ignored, Phase D — meta-loop not yet implemented
test test_auditor_input_is_artifact_only ... ignored, Phase D — meta-loop not yet implemented
test test_auditor_verdict_writes_to_audit_log ... ignored, Phase D — meta-loop not yet implemented

test result: ok. 0 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/fc_alignment_conformance.rs (target/debug/deps/fc_alignment_conformance-60e2d98002a413ce)

running 9 tests
test b2_cost_aggregator_construct_and_record ... ok
test b1_jsonl_schema_run_record_dispatcher_present ... ok
test b4_post_hoc_verifier_progress_zero_on_runtime_reject ... ok
test b3_wall_clock_first_read_to_final_accept ... ok
test fc1_n12_lean4_oracle_constructible ... ok
test rollback_sim_env_check_function_present ... ok
test rollback_sim_env_var_canonical_name ... ok
test rollback_sim_predicate_logic_at_threshold ... ok
test rollback_sim_threshold_constant_matches_prereg ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/fc_trace_smoke.rs (target/debug/deps/fc_trace_smoke-a9be949038a28e41)

running 1 test
warning: unused import: `KernelError`
 --> src/bus.rs:8:29
  |
8 | use crate::kernel::{Kernel, KernelError};
  |                             ^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `TapeError`
 --> src/bus.rs:9:54
  |
9 | use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
  |                                                      ^^^^^^^^^

warning: unused imports: `Deserialize` and `Serialize`
  --> src/bus.rs:11:13
   |
11 | use serde::{Deserialize, Serialize};
   |             ^^^^^^^^^^^  ^^^^^^^^^

warning: unused import: `crate::sdk::snapshot::UniverseSnapshot`
 --> src/sdk/actor.rs:7:5
  |
7 | use crate::sdk::snapshot::UniverseSnapshot;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `std::collections::HashSet`
 --> src/sdk/actor.rs:9:5
  |
9 | use std::collections::HashSet;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ToolSignal`
 --> src/sdk/tools/search.rs:4:24
  |
4 | use crate::sdk::tool::{ToolSignal, TuringTool};
  |                        ^^^^^^^^^^

warning: unused import: `std::path::Path`
 --> src/sdk/tools/search.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `Deserialize`
 --> src/sdk/tools/librarian.rs:6:13
  |
6 | use serde::{Deserialize, Serialize};
  |             ^^^^^^^^^^^

warning: unused import: `Path`
 --> src/sdk/tools/librarian.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^

test fc_trace_file_receives_well_formed_json_event ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 44.51s

     Running tests/heldout_operational_sealing.rs (target/debug/deps/heldout_operational_sealing-af83cb32d83b799b)

running 5 tests
test test_l2_agent_prompt_context_blacklist ... ok
test test_l5_source_pool_enumeration_block ... ok
test test_l4_hash_and_seed_substring_co_occurrence ... ok
test test_l1_file_path_read_isolation ... ok
test test_l3_tool_call_no_hash_invocation_in_agent_code ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

     Running tests/mode_flag_binary_purity.rs (target/debug/deps/mode_flag_binary_purity-009530e1a6aaf97a)

running 6 tests
test test_mode_amnesia_drops_err ... ignored, Phase C — --mode flag not yet implemented in evaluator
test test_mode_flag_full_is_default ... ignored, Phase C — --mode flag not yet implemented in evaluator
test test_mode_flag_is_required_for_non_full_modes ... ignored, Phase C — --mode flag not yet implemented in evaluator
test test_mode_homogeneous_collapses_iac ... ignored, Phase C — --mode flag not yet implemented in evaluator
test test_mode_panopticon_increases_cpr_iac ... ignored, Phase C — --mode flag not yet implemented in evaluator
test test_mode_soft_law_diverges_runtime_from_verified ... ignored, Phase C — --mode flag not yet implemented in evaluator

test result: ok. 0 passed; 0 failed; 6 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/pput_anti_goodhart.rs (target/debug/deps/pput_anti_goodhart-9b39f4ace17f6d22)

running 10 tests
test test_failed_branches_in_total_cost ... ok
test test_golden_path_requires_ground_truth ... ok
test test_all_model_tokens_counted ... ok
test test_no_pput_in_agent_prompt ... ok
test test_no_metric_file_access_by_agents ... ok
test test_tool_stdout_hash_logged ... ok
test test_no_hidden_unmetered_generation ... ok
test test_no_problem_id_hardcode ... ok
test test_wall_clock_first_read_to_final_accept ... ok
test test_heldout_ids_inaccessible ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s

     Running tests/trust_root_immutability.rs (target/debug/deps/trust_root_immutability-7c21a1ad6ce805d7)

running 4 tests
test test_pput_accounting_0_section_present ... ok
test test_trust_root_manifest_includes_b2_b4_files ... ok
test test_trust_root_simulated_write_aborts ... ok
test test_trust_root_immutable_at_boot ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s

     Running unittests src/lib.rs (target/debug/deps/turingosv4-cd2d82dded9eba82)

running 128 tests
test boot::tests::parse_errors_on_unquoted_key ... ok
test boot::tests::parse_errors_when_section_missing ... ok
test boot::tests::parse_strips_inline_comment_and_blanks ... ok
test boot::tests::verify_trust_root_detects_tamper_in_tempdir ... ok
test boot::tests::verify_trust_root_passes_when_hash_matches_in_tempdir ... ok
test bus::tests::test_bus_basic_append ... ok
test bus::tests::test_bus_classify_bounded ... ok
test bus::tests::test_bus_forbidden_pattern_veto ... ok
test bus::tests::test_bus_creates_market_on_append ... ok
test bus::tests::test_bus_ledger_integrity ... ok
test bus::tests::test_bus_payload_too_long ... ok
test bus::tests::test_bus_serial_ordering ... ok
test bus::tests::test_bus_halt_and_settle ... ok
test bus::tests::test_bus_snapshot ... ok
test bus::tests::test_bus_graveyard_feedback ... ok
test bus::tests::test_bus_too_many_lines ... ok
test drivers::llm_http::tests::test_client_creation ... ok
test bus::tests::test_bus_unknown_agent_vetoed ... ok
test drivers::llm_http::tests::test_driver_error_display ... ok
test kernel::tests::test_append_and_retrieve ... ok
test drivers::llm_http::tests::test_generate_request_serialization ... ok
test kernel::tests::test_golden_path_trace ... ok
test kernel::tests::test_market_lifecycle ... ok
test kernel::tests::test_no_duplicate_market ... ok
test kernel::tests::test_no_market_for_nonexistent_node ... ok
test kernel::tests::test_market_ticker ... ok
test kernel::tests::test_resolve_all_markets ... ok
test kernel::tests::test_reject_duplicate ... ok
test ledger::tests::test_ledger_append_and_verify ... ok
test ledger::tests::test_ledger_hash_chain_integrity ... ok
test ledger::tests::test_ledger_omega_vocabulary ... ok
test ledger::tests::test_ledger_sequence_monotonic ... ok
test ledger::tests::test_tape_append_root_node ... ok
test ledger::tests::test_ledger_tamper_detection ... ok
test ledger::tests::test_tape_append_with_valid_citation ... ok
test kernel::tests::test_reject_dangling_citation ... ok
test ledger::tests::test_tape_reject_dangling_citation ... ok
test ledger::tests::test_tape_reject_duplicate_id ... ok
test ledger::tests::test_tape_time_arrow_ordering ... ok
test ledger::tests::test_tape_trace_ancestors ... ok
test prediction_market::tests::test_assassin_profit ... ok
test prediction_market::tests::test_buy_no_increases_no_price ... ok
test ledger::tests::test_tape_dag_branching ... ok
test prediction_market::tests::test_buy_yes_increases_yes_price ... ok
test prediction_market::tests::test_constant_product_invariant ... ok
test prediction_market::tests::test_ctf_conservation_1_coin_1_yes_1_no ... ok
test prediction_market::tests::test_create_market ... ok
test prediction_market::tests::test_initial_price_is_50_50 ... ok
test prediction_market::tests::test_no_double_resolution ... ok
test prediction_market::tests::test_multiple_traders_price_discovery ... ok
test prediction_market::tests::test_no_trading_after_resolution ... ok
test prediction_market::tests::test_pioneer_profit ... ok
test prediction_market::tests::test_redeem_requires_resolution ... ok
test prediction_market::tests::test_prices_sum_to_one ... ok
test prediction_market::tests::test_reject_zero_or_negative_amounts ... ok
test sdk::actor::tests::test_boltzmann_never_returns_none_with_nodes ... ok
test ledger::tests::test_tape_empty ... ok
test sdk::actor::tests::test_boltzmann_returns_none_empty_tape ... ok
test sdk::actor::tests::test_frontier_detection_leaf ... ok
test sdk::actor::tests::test_lineage_score_increases_with_depth ... ok
test sdk::actor::tests::test_frontier_detection_parent_with_child ... ok
test sdk::error_abstraction::tests::fixture_linarith_failed ... ok
test sdk::actor::tests::test_boltzmann_diversity_not_deterministic ... ok
test sdk::error_abstraction::tests::classifier_version_is_stamped ... ok
test sdk::error_abstraction::tests::fixture_other_catchall ... ok
test sdk::error_abstraction::tests::fixture_rewrite_no_match ... ok
test sdk::error_abstraction::tests::fixture_simp_no_progress ... ok
test sdk::error_abstraction::tests::fixture_type_mismatch ... ok
test sdk::error_abstraction::tests::fixture_unexpected_token ... ok
test sdk::error_abstraction::tests::fixture_unknown_constant ... ok
test sdk::error_abstraction::tests::labels_are_unique_and_stable ... ok
test sdk::error_abstraction::tests::fixture_unsolved_goals ... ok
test sdk::prompt::tests::test_prompt_surfaces_search_hits ... ok
test sdk::prompt::tests::test_prompt_contains_no_example_values ... ok
test sdk::prompt::tests::test_prompt_includes_balance ... ok
test sdk::prompt_guard::tests::test_case_insensitive_match - should panic ... ok
test sdk::prompt_guard::tests::test_clean_prompt_passes ... ok
test sdk::prompt_guard::tests::test_empty_prompt_passes ... ok
test sdk::prompt::tests::test_prompt_surfaces_team_board ... ok
test sdk::prompt_guard::tests::test_pput_assignment_pattern_caught - should panic ... ok
test sdk::prompt_guard::tests::test_h_vpput_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_m_verified_caught - should panic ... ok
test sdk::prompt::tests::test_prompt_truncates_errors_to_3 ... ok
test sdk::prompt_guard::tests::test_pput_runtime_caught - should panic ... ok
test sdk::prompt_guard::tests::test_wbcg_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_substring_in_larger_text - should panic ... ok
test sdk::prompt_guard::tests::test_pput_verified_caught - should panic ... ok
test sdk::protocol::tests::test_malformed_action_tag_rejected_not_fallback ... ok
test sdk::protocol::tests::test_deduct_negative_amount_rejected ... ok
test sdk::protocol::tests::test_no_byte_repair_on_invalid_escape ... ok
test sdk::protocol::tests::test_parse_action_tag_valid ... ok
test sdk::protocol::tests::test_parse_action_tag_with_think_block ... ok
test sdk::protocol::tests::test_parse_bare_json_fallback ... ok
test sdk::protocol::tests::test_parse_invalid_json_returns_error ... ok
test sdk::protocol::tests::test_parse_no_action_returns_error ... ok
test sdk::protocol::tests::test_parse_with_invest_action ... ok
test sdk::protocol::tests::test_strip_multiple_think_blocks ... ok
test sdk::protocol::tests::test_strip_unclosed_think_block ... ok
test sdk::protocol::tests::test_strip_think_blocks ... ok
test boot::tests::verify_trust_root_passes_on_intact_repo ... ok
test sdk::sandbox::tests::test_sandbox_captures_stderr ... ok
test sdk::snapshot::tests::test_snapshot_balance_query ... ok
test sdk::sandbox::tests::test_sandbox_echo_command ... ok
test sdk::tools::librarian::tests::test_board_post_append ... ok
test sdk::tools::librarian::tests::test_board_write_read_roundtrip ... ok
test sdk::tools::librarian::tests::test_compress_interval ... ok
test sdk::tools::librarian::tests::test_build_compression_prompt ... ok
test sdk::tools::librarian::tests::test_zero_interval_never_compresses ... ok
test sdk::tools::search::tests::test_search_empty_query ... ok
test sdk::tools::search::tests::test_sanitize_query ... ok
test sdk::tools::search::tests::test_search_nonexistent_path ... ok
test sdk::tools::wallet::tests::test_deduct_and_credit ... ok
test sdk::tools::wallet::tests::test_genesis_allocation ... ok
test sdk::tools::wallet::tests::test_append_is_free ... ok
test sdk::tools::wallet::tests::test_negative_deduct_rejected ... ok
test sdk::tools::wallet::tests::test_insufficient_balance_rejected ... ok
test sdk::tools::wallet::tests::test_portfolio_tracking ... ok
test sdk::tools::wallet::tests::test_no_double_genesis ... ok
test sdk::tools::wallet::tests::test_query_balance ... ok
test sdk::tools::wallet::tests::test_unknown_agent_vetoed ... ok
test sdk::sandbox::tests::test_sandbox_nonzero_exit ... ok
test sdk::tools::wallet::tests::test_query_unknown_key ... ok
test wal::tests::test_wal_replay_missing_file_is_empty ... ok
test sdk::tools::wallet::tests::test_zero_deduct_rejected ... ok
test wal::tests::test_wal_skip_malformed_line ... ok
test wal::tests::test_wal_roundtrip_nodes_only ... ok
test wal::tests::test_wal_roundtrip_mixed ... ok
test sdk::sandbox::tests::test_sandbox_timeout_kills_process ... ok

test result: ok. 128 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.09s

     Running unittests src/main.rs (target/debug/deps/turingosv4-ad648582b935fecb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/fc_alignment_conformance.rs (target/debug/deps/fc_alignment_conformance-239d53a59894a473)

running 26 tests
test fc1_n11_predicate_trait_register_api ... ignored, 🔨 Stage 3 unmerged — bus.register_predicate API + Predicate trait live on phase-z-wtool-tools branch only; not on main. Production path uses inline forbidden_patterns check in append_internal as the ∏p surface.
test fc1_n12_lean4_oracle_ground_truth_predicate ... ignored, Cross-crate — Lean4Oracle in minif2f_v4 sub-crate; covered in experiments/minif2f_v4/tests/fc_alignment_conformance.rs (separate file, separate atom)
test fc1_n11_n15_e18_pi_p_zero_preserves_q_t_via_forbidden_pattern ... ok
test fc1_n13_wtool_bus_append_present ... ok
test fc1_n1_q_state_carrier_constructible_with_default_config ... ok
test fc1_n4_tape_constructible_with_time_arrow ... ok
test fc1_n6_input_universe_snapshot_via_bus ... ok
test fc2_n16_init_ai_orchestrator_swarm_oneshot ... ignored, Binary-only — run_swarm/run_oneshot are in evaluator binary, not lib; refactor needed to expose for direct integration testing
test fc1_n7_delta_ai_client_constructible ... ok
test fc2_n20_n27_tick_mr_present ... ok
test fc2_n23_haltreason_full_taxonomy_typed ... ignored, 📅 Not yet typed as Rust enum — only OmegaAccepted exists; other 4 variants {MaxTxExhausted, WallClockCap, ComputeCapViolated, ErrorHalt} per CLAUDE.md report standard live as jsonl strings in extra map. Type promotion is Phase C+ work.
test fc2_n23_event_type_omega_accepted_canonical ... ok
test fc3_e14_boot_panic_immediate_abort_documented ... ok
test fc3_e15_e16_e17_constitutional_signaling ... ignored, 📅 Phase 11+ — automated runtime veto/abide signaling not implemented. Today: manual policy via CLAUDE.md Audit Standard
test fc3_n32_veto_ai_runtime ... ignored, 📅 Phase 11+ — Veto-AI runtime not implemented (manual Codex/Gemini dual-audit covers role today; Art. V.1.3 amendment 2026-04-25 narrowed scope to {PASS, VETO})
test fc3_n33_architect_ai_runtime ... ignored, 📅 Phase 11+ — ArchitectAI runtime not implemented (manual Claude code editing covers role today; Phase D will deliver. Art. V.1.2 amendment grants commit authority post-Veto-AI PASS)
test fc1_n8_n9_n10_output_agent_output_parseable ... ok
test fc2_n22_halt_via_halt_and_settle ... ok
test fc3_n31_logs_archive_wal_open_in_tempdir ... ok
test fc3_n34_parse_trust_root_section_helper ... ok
test fc3_n40_logs_to_architect_feedback ... ignored, 📅 Phase 11+ — automated logs → ArchitectAI feedback loop not implemented. Phase D consumer reads jsonl + WAL + stderr (per THESIS_V2_GROUND_TRUTH_AUDIT findings C+D)
test fc3_n41_in_process_reinit_loop ... ignored, 📅 Phase 11+ — in-process re-init not implemented (external batch runner retry covers today). FC3-E14 immediate-abort leaf is what we have.
test fc3_n34_trust_root_error_taxonomy_present ... ok
test fc3_n39_log_ledger_present_and_appendable ... ok
test fc3_s3_readonly_subgraph_manifest_size ... ok
test fc3_n34_readonly_guard_verify_trust_root_intact_repo ... ok

test result: ok. 17 passed; 0 failed; 9 ignored; 0 measured; 0 filtered out; finished in 0.07s

     Running tests/reward_pull_conservation.rs (target/debug/deps/reward_pull_conservation-cc5a4dd71482e4d5)

running 5 tests
test phase2_conservation_total_coins_bounded ... ok
test phase2_settle_pays_out_on_golden_path ... ok
test phase2_founder_grant_credits_yes_on_append ... ok
test phase2_no_grant_when_flag_off ... ok
test phase2_settle_zero_on_losing_side ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/wal_resume.rs (target/debug/deps/wal_resume-a4181e084fd48f51)

running 2 tests
test wal_empty_file_yields_fresh_bus ... ok
test wal_persists_appends_across_bus_drop ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

   Doc-tests minif2f_v4

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests turingosv4

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


codex
The test run is still active at the FC-trace integration test; that test shells out to `cargo run --example fc_trace_emit_one`, so the extra delay is expected. While it runs I’m reading the key source regions for line-accurate findings.
exec
/bin/bash -lc "nl -ba handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md | sed -n '1,140p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# PREREG Amendment — p_0 Calibration Deferral (2026-04-25)
     2	
     3	**Authority**: ArchitectAI commit per Art. V.1.2 amendment + case C-073 (non-constitution PREREG amendment within ArchitectAI scope).
     4	**FC-trace**: FC1-N12 (∏p ground-truth oracle scope unchanged) + Art. V.1.2 (commit authority) + C-073 + C-075.
     5	**Predecessor**: `PREREG_PPUT_CCL_2026-04-26.md` § 5.5 (frozen, NOT modified by this amendment — see § 6 below).
     6	
     7	---
     8	
     9	## § 1. Triggering rationale
    10	
    11	PREREG § 5.5 specifies p_0 calibration via 576 paired runs (144 adaptation × 2 seeds × {control, treatment}) with estimated cost "~8 wall-hours, ~$3-5". Empirical observation 2026-04-25 during launched batch (commit 650caf7+ era):
    12	
    13	| Item | PREREG estimate | Empirical observation | Multiplier |
    14	|---|---|---|---|
    15	| Per-run wall-clock | ~50s | ~15-30 min average (hard problems hit max_transactions=200; aime_1988_p8 SOLVED at 28 min) | 30-40× |
    16	| Total batch wall-clock | ~8 hours | Realistic 3-7 days (576 runs × 15 min serial; treatment short-circuits halve some) | 9-21× |
    17	| API cost | $3-5 | Still ~$15-20 (DeepSeek-v4-flash thinking-off cheap) | 3-4× |
    18	
    19	**The 8-hour estimate was based on "~50s/run chat oneshot" assumption that turned out wrong for swarm n3 condition on the adaptation-144 problem mix.** A 7-day batch is not "overnight"; user (mid-session 2026-04-25) explicitly questioned 576-run necessity given multiple unresolved engineering questions (N-agents → PPUT relationship, swarm_N=1 vs oneshot calibration, ground-truth feedback pipeline, etc.).
    20	
    21	## § 2. Amendment
    22	
    23	PREREG § 5.5 calibration **DEFERRED** indefinitely with the following operative substitution for Phase B → C transition and Phase E Gate H requirements:
    24	
    25	**`p_0` for guardrail purposes**: take the **PREREG § 5.5 ceiling itself = 0.10** as the conservative upper bound. Any artifact j whose `j-RR` regression rate exceeds 0.10 fails Gate H per the original guardrail logic; setting `p_0 = 0.10` (the maximum tolerated value) is the strictest possible substitute when no calibrated tighter value exists. This is mathematically conservative: artifacts must clear the strictest plausible bar, not a narrower data-derived bar.
    26	
    27	**`genesis_payload.toml [pput_accounting_0].baseline_regression_rate`**: setting deferred to ArchitectAI commit window. Current value `0.0` is recognized as INVALID PLACEHOLDER (would auto-fail any artifact with any regression). Until calibration runs, **Gate H consumers MUST hardcode `p_0 = 0.10`** at the consumption site, not read from `genesis_payload.toml`.
    28	
    29	**`baseline_regression_jsonl_sha256`**: stays empty (calibration jsonl does not exist yet).
    30	
    31	## § 3. Conditions for re-calibration
    32	
    33	Calibration becomes worthwhile (and the deferral lifted) when ALL of:
    34	
    35	1. **N-experiments arc (Phase A-D of new plan, 2026-04-25 N-agents experiments) complete** — answers Q1/Q2/Q3 about N → PPUT, fixes (or rejects) the throttle hypothesis, sediments per-N best practices into evaluator. Without this, calibrating p_0 on a known-suboptimal N=3 swarm is calibrating against a moving baseline.
    36	
    37	2. **swarm_N=1 mode landed** (Phase A atom A2) — current `CONDITION=oneshot` is a different code path; PREREG § 5.5 ambiguous about which is the "control".
    38	
    39	3. **Per-agent budget normalization landed** (Phase A atom A5) — current `max_transactions=200` is fixed-tx budget; PREREG § 5.5 implicitly assumes tx-budget but doesn't specify; need explicit budget regime declaration for calibration to be reproducible.
    40	
    41	4. **Heterogeneous LLM agents experiment complete** (Phase A3.5 / E_hetero) — if hetero finds significant solve_rate uplift, the calibration must be done on the production model mix, not on homo n3 baseline.
    42	
    43	5. **Phase D ArchitectAI runtime exists** — calibration is part of Gate H gating Phase E. Doing it before Phase D = calibrating against a counterfactual ArchitectAI that doesn't exist.
    44	
    45	When ALL 5 conditions met: re-issue PREREG_AMENDMENT to lift the deferral + trigger the 576-run batch with the (then-current) production mode.
    46	
    47	## § 4. Impact on Phase B → C transition
    48	
    49	PREREG_PPUT_CCL_2026-04-26 § 5.5 originally listed p_0 calibration as a Phase B prerequisite ("Schedule: Phase B7 mandatory; not deferrable to Phase D"). This amendment **explicitly OVERRIDES that "not deferrable" clause** for the deferral conditions in § 3 above.
    50	
    51	Phase B → C exit checklist accordingly:
    52	- ❌ p_0 calibration jsonl frozen (was REQUIRED) → now DEFERRED with substitution per § 2
    53	- ✅ B1-B7 + B7-extra mode toggle infrastructure complete
    54	- ✅ Phase A0 harness modernization complete (post-2026-04-25 governance work)
    55	- ✅ Tools qualified (per case C-075): runner.sh, compute_p0.py, evaluator boot enforcement, etc.
    56	- ✅ Trust Root verifies clean
    57	
    58	Phase B → C dual-audit packet (next major milestone) must reference this amendment + show that Phase E Gate H consumer hardcodes `p_0 = 0.10`.
    59	
    60	## § 5. What this amendment does NOT change
    61	
    62	- **PREREG § 5.5 protocol itself** — the calibration *protocol* (288 control + 288 treatment paired runs, max-over-seeds, etc.) remains the agreed-upon procedure for IF calibration ever runs. Amendment defers the SCHEDULING, not the SCIENCE.
    63	- **PREREG § 1.8 Trust Root composition** — manifest entries unchanged by this amendment (this amendment doc is added per § 7 below).
    64	- **PREREG § 5.4 j-RR ≤ p_0 guardrail logic** — Gate H still uses the guardrail; just the p_0 source changes (hardcoded 0.10 instead of calibrated value).
    65	- **PREREG § 5.6 family total / N_max** — unchanged.
    66	- **All other PREREG § sections** — unchanged.
    67	
    68	## § 6. PREREG document treatment
    69	
    70	`PREREG_PPUT_CCL_2026-04-26.md` itself is **NOT EDITED** by this amendment. It remains the immutable round-4 frozen pre-registration. This amendment is a separate document referenced from § 5.5 forward via a pointer added to Trust Root manifest.
    71	
    72	This pattern is per CLAUDE.md "Common Law": amendments are recorded as separate cases / docs that supersede specific sections, leaving the original frozen for reproducibility. PREREG_PPUT_CCL_2026-04-26.md SHA-256 in Trust Root manifest UNCHANGED.
    73	
    74	## § 7. Trust Root impact
    75	
    76	Add this amendment doc to genesis_payload.toml [trust_root]:
    77	```
    78	"handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md" = "<sha256>"
    79	```
    80	
    81	Manifest size: 24 → 25 entries.
    82	
    83	## § 8. Audit requirement
    84	
    85	Per case C-073 ArchitectAI commit workflow: this amendment requires dual audit (Codex + Gemini, conservative VETO > CHALLENGE > PASS) before commit lands. Audit packet should specifically test:
    86	
    87	- Does the amendment violate any PREREG § 5.5 constraint? (Should not — defer is operationally permitted given § 5.5 ceiling.)
    88	- Does substitution of `p_0 = 0.10` invalidate any Gate H statistical claim? (Should not — strictest plausible bar is conservative; no Type-I inflation.)
    89	- Does deferral leave any phase blocked indefinitely? (Should not — § 3 lists explicit re-calibration conditions; if those never met, Phase E proceeds with the conservative substitution per § 2 final paragraph.)
    90	
    91	## § 9. Cross-references
    92	
    93	- `PREREG_PPUT_CCL_2026-04-26.md` § 5.5 (the amended section, IMMUTABLE)
    94	- `cases/C-073_architect_ai_commit_authority.yaml` (governance basis)
    95	- `cases/C-075_do_178c_tool_qualification.yaml` (tool-readiness as re-calibration precondition)
    96	- `handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md` (context: cost asymmetry concern)
    97	- `handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md` (context: ground-truth feedback discipline)
    98	- `handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md` (PASS verdict on round-4 batch — but batch was 3-7 days not 8h, motivating this deferral)

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/budget_regime.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// Phase A atom A5 — explicit per-agent budget regime (`BUDGET_REGIME` +
     2	// `MAX_TRANSACTIONS` env vars).
     3	//
     4	// Constitutional anchor: FC2-N22 (HALT node, `MaxTxExhausted` variant).
     5	// The transaction loop in `run_swarm` terminates on either OMEGA accept
     6	// (FC1-E17) or budget exhaustion (FC2-N22 / `MaxTxExhausted`). The budget
     7	// regime declares HOW the run-level budget is partitioned across the N
     8	// δ instances (Agent_i, FC1-N7) so that PPUT comparisons across N values
     9	// answer a well-posed question.
    10	//
    11	// Codex / Gemini N-agents brainstorm 2026-04-25 § A.3 frames four regimes:
    12	//   - fixed transaction budget         (tx=200 for all N)
    13	//   - fixed proposal / N×tx budget     (each agent gets tx=base proposals)
    14	//   - fixed token budget               (cap on total LLM tokens)
    15	//   - fixed wall-clock budget          (cap on T_i)
    16	//
    17	// In our codebase the inner loop (`for tx in 0..max_transactions`) invokes
    18	// exactly ONE agent per `tx` (Boltzmann-routed). So "tx" already counts
    19	// proposals, and the brainstorm's "fixed transaction budget" maps to
    20	// `total_proposal` (loop bound = base, regardless of N — current
    21	// behavior). The brainstorm's "N × tx = constant" is orthogonal: we want
    22	// the loop bound to scale with N so each agent receives the same number
    23	// of proposals — that's `per_agent`.
    24	//
    25	// PREREG_AMENDMENT_p0_defer § 3 condition 3 (2026-04-25) names this atom
    26	// as a re-calibration prerequisite: "current max_transactions=200 is
    27	// fixed-tx budget; PREREG § 5.5 implicitly assumes tx-budget but doesn't
    28	// specify; need explicit budget regime declaration for calibration to be
    29	// reproducible." A5 satisfies that by stamping the regime + base budget
    30	// on every emitted v2 row.
    31	//
    32	// Phase A scope: implement `total_proposal` + `per_agent` (the two
    33	// regimes that fall out of the existing tx loop). `token_total` /
    34	// `wall_clock` require new exit conditions (cost / clock thresholds) and
    35	// are declared startup-fatal `UnimplementedRegime` so a misconfigured
    36	// `BUDGET_REGIME=token_total` aborts before burning LLM budget. These
    37	// land in a later atom once the cost/clock exit machinery exists.
    38	
    39	use std::fmt;
    40	
    41	/// TRACE_MATRIX FC2-N22: env var selecting how the run-level transaction
    42	/// budget partitions across N δ agents. Default (unset/empty) =
    43	/// `total_proposal`, preserving Phase B baseline behavior bit-for-bit.
    44	pub const BUDGET_REGIME_ENV_VAR: &str = "BUDGET_REGIME";
    45	
    46	/// TRACE_MATRIX FC2-N22: env var setting the base transaction budget.
    47	/// The effective loop bound is `effective_max_tx(regime, base, N)`.
    48	/// Default 200 (Phase B baseline).
    49	pub const MAX_TRANSACTIONS_ENV_VAR: &str = "MAX_TRANSACTIONS";
    50	
    51	/// Default base budget when `MAX_TRANSACTIONS` env is unset.
    52	/// Preserves the long-standing `let max_transactions = 200` baseline.
    53	pub const DEFAULT_MAX_TRANSACTIONS: u32 = 200;
    54	
    55	/// TRACE_MATRIX FC2-N22: budget regime variants. The first two are
    56	/// implemented in Phase A; the latter two are declared so a downstream
    57	/// run that wants them aborts at startup (UnimplementedRegime) instead
    58	/// of silently falling back and burning budget under the wrong regime.
    59	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
    60	pub enum BudgetRegime {
    61	    /// Loop bound = `base`, regardless of N. Each agent ends up with
    62	    /// roughly `base / N` proposals. **Phase B baseline + default.**
    63	    /// Brainstorm § A.3 "fixed transaction budget".
    64	    TotalProposal,
    65	    /// Loop bound = `base × N`. Each agent receives `base` proposals
    66	    /// regardless of swarm size. Brainstorm § A.3 "N × tx = constant"
    67	    /// reframed as "constant per-agent".
    68	    PerAgent,
    69	    /// Cap total LLM tokens (declared, not yet implemented). Requires a
    70	    /// new exit condition tied to `RunCostAccumulator` thresholds.
    71	    TokenTotal,
    72	    /// Cap wall-clock T_i (declared, not yet implemented). Requires a
    73	    /// new exit condition tied to `RunWallClock`.
    74	    WallClock,
    75	}
    76	
    77	impl BudgetRegime {
    78	    /// Stable string label stamped on jsonl rows. Stable across releases;
    79	    /// downstream PPUT analysis joins on this exact string.
    80	    pub fn label(&self) -> &'static str {
    81	        match self {
    82	            BudgetRegime::TotalProposal => "total_proposal",
    83	            BudgetRegime::PerAgent => "per_agent",
    84	            BudgetRegime::TokenTotal => "token_total",
    85	            BudgetRegime::WallClock => "wall_clock",
    86	        }
    87	    }
    88	}
    89	
    90	/// TRACE_MATRIX FC2-N22: startup-fatal failure modes for the regime
    91	/// resolver. Each variant aborts before the first LLM call so a
    92	/// misconfigured run cannot consume API budget.
    93	#[derive(Debug, PartialEq, Eq)]
    94	pub enum BudgetError {
    95	    /// `BUDGET_REGIME` value not in
    96	    /// {`total_proposal`, `per_agent`, `token_total`, `wall_clock`}.
    97	    UnknownRegime(String),
    98	    /// `MAX_TRANSACTIONS` not parseable as positive u32.
    99	    InvalidMaxTransactions(String),
   100	    /// Caller asked for a regime whose exit machinery is not yet wired
   101	    /// (`token_total` / `wall_clock`). Carries the requested variant so
   102	    /// the startup error names what is missing.
   103	    UnimplementedRegime(BudgetRegime),
   104	    /// Effective loop bound would overflow u32 (`base × N > u32::MAX`).
   105	    /// Realistically unreachable (would require base × N ≥ 2^32) but
   106	    /// expressed in the type so the callers cannot panic on overflow.
   107	    EffectiveBudgetOverflow { base: u32, n_agents: usize },
   108	}
   109	
   110	impl fmt::Display for BudgetError {
   111	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
   112	        match self {
   113	            Self::UnknownRegime(s) => write!(
   114	                f,
   115	                "BUDGET_REGIME='{}' is not a known regime \
   116	                 (expected total_proposal | per_agent | token_total | wall_clock)",
   117	                s
   118	            ),
   119	            Self::InvalidMaxTransactions(s) => write!(
   120	                f,
   121	                "MAX_TRANSACTIONS='{}' is not a positive integer",
   122	                s
   123	            ),
   124	            Self::UnimplementedRegime(r) => write!(
   125	                f,
   126	                "BUDGET_REGIME='{}' declared but its exit machinery is not yet \
   127	                 implemented (Phase A scope = total_proposal + per_agent only). \
   128	                 Aborting startup to avoid silent fallback under a different regime.",
   129	                r.label()
   130	            ),
   131	            Self::EffectiveBudgetOverflow { base, n_agents } => write!(
   132	                f,
   133	                "effective_max_tx overflow: base={} × n_agents={} exceeds u32::MAX",
   134	                base, n_agents
   135	            ),
   136	        }
   137	    }
   138	}
   139	
   140	impl std::error::Error for BudgetError {}
   141	
   142	/// TRACE_MATRIX FC2-N22: pure parser for the `BUDGET_REGIME` env value.
   143	/// Empty (unset / blank-after-trim) → default `TotalProposal`. No env
   144	/// access — testable without process-global state.
   145	pub fn parse_budget_regime(env_str: &str) -> Result<BudgetRegime, BudgetError> {
   146	    let trimmed = env_str.trim();
   147	    if trimmed.is_empty() {
   148	        return Ok(BudgetRegime::TotalProposal);
   149	    }
   150	    match trimmed {
   151	        "total_proposal" => Ok(BudgetRegime::TotalProposal),
   152	        "per_agent" => Ok(BudgetRegime::PerAgent),
   153	        "token_total" => Ok(BudgetRegime::TokenTotal),
   154	        "wall_clock" => Ok(BudgetRegime::WallClock),
   155	        other => Err(BudgetError::UnknownRegime(other.to_string())),
   156	    }
   157	}
   158	
   159	/// TRACE_MATRIX FC2-N22: pure parser for the `MAX_TRANSACTIONS` env
   160	/// value. Empty (unset / blank-after-trim) → default
   161	/// `DEFAULT_MAX_TRANSACTIONS`. Pure (no env access).
   162	pub fn parse_max_transactions(env_str: &str) -> Result<u32, BudgetError> {
   163	    let trimmed = env_str.trim();
   164	    if trimmed.is_empty() {
   165	        return Ok(DEFAULT_MAX_TRANSACTIONS);
   166	    }
   167	    match trimmed.parse::<u32>() {
   168	        Ok(0) => Err(BudgetError::InvalidMaxTransactions(trimmed.to_string())),
   169	        Ok(v) => Ok(v),
   170	        Err(_) => Err(BudgetError::InvalidMaxTransactions(trimmed.to_string())),
   171	    }
   172	}
   173	
   174	/// TRACE_MATRIX FC2-N22: scale the base budget by the regime + swarm
   175	/// size. Pure. Returns the loop bound (`for tx in 0..effective_max_tx`).
   176	///
   177	/// - TotalProposal → base
   178	/// - PerAgent      → base × n_agents (overflow-checked)
   179	/// - TokenTotal / WallClock → UnimplementedRegime (Phase A scope)
   180	///
   181	/// `n_agents == 0` is rejected upstream (run_swarm precondition); we
   182	/// pass it through here to stay pure but the multiplication is safe
   183	/// (`base × 0 = 0`, which fails the `for tx in 0..0` loop fast).
   184	pub fn effective_max_tx(
   185	    regime: BudgetRegime,
   186	    base: u32,
   187	    n_agents: usize,
   188	) -> Result<u32, BudgetError> {
   189	    match regime {
   190	        BudgetRegime::TotalProposal => Ok(base),
   191	        BudgetRegime::PerAgent => {
   192	            let n = n_agents as u64;
   193	            let prod = (base as u64).saturating_mul(n);
   194	            if prod > u32::MAX as u64 {
   195	                return Err(BudgetError::EffectiveBudgetOverflow { base, n_agents });
   196	            }
   197	            Ok(prod as u32)
   198	        }
   199	        BudgetRegime::TokenTotal | BudgetRegime::WallClock => {
   200	            Err(BudgetError::UnimplementedRegime(regime))
   201	        }
   202	    }
   203	}
   204	
   205	/// TRACE_MATRIX FC2-N22: env-coupled resolver invoked once at run_swarm
   206	/// startup. Returns `(regime, base_max_tx, effective_max_tx)` so the
   207	/// caller can both run the loop AND stamp the regime + base on the
   208	/// emitted v2 row. Errors abort the run before the first LLM call.
   209	pub fn resolve_budget(n_agents: usize) -> Result<(BudgetRegime, u32, u32), BudgetError> {
   210	    let regime_raw = std::env::var(BUDGET_REGIME_ENV_VAR).unwrap_or_default();
   211	    let base_raw = std::env::var(MAX_TRANSACTIONS_ENV_VAR).unwrap_or_default();
   212	    let regime = parse_budget_regime(&regime_raw)?;
   213	    let base = parse_max_transactions(&base_raw)?;
   214	    let eff = effective_max_tx(regime, base, n_agents)?;
   215	    Ok((regime, base, eff))
   216	}
   217	
   218	#[cfg(test)]
   219	mod tests {
   220	    use super::*;
   221	    use std::sync::Mutex;
   222	
   223	    // Per memory `feedback_env_var_test_lock`: tests that mutate
   224	    // process-global env vars (BUDGET_REGIME / MAX_TRANSACTIONS) must
   225	    // serialise to survive cargo's parallel runner.
   226	    static ENV_LOCK: Mutex<()> = Mutex::new(());
   227	
   228	    // Phase A atom A5 unit tests. Pure-fn surface first; the env-coupled
   229	    // `resolve_budget` is tested at the bottom under the env mutex.
   230	
   231	    #[test]
   232	    fn parse_regime_empty_defaults_to_total_proposal() {
   233	        assert_eq!(parse_budget_regime("").unwrap(), BudgetRegime::TotalProposal);
   234	        assert_eq!(parse_budget_regime("   ").unwrap(), BudgetRegime::TotalProposal);
   235	    }
   236	
   237	    #[test]
   238	    fn parse_regime_known_values() {
   239	        assert_eq!(
   240	            parse_budget_regime("total_proposal").unwrap(),
   241	            BudgetRegime::TotalProposal
   242	        );
   243	        assert_eq!(
   244	            parse_budget_regime("per_agent").unwrap(),
   245	            BudgetRegime::PerAgent
   246	        );
   247	        assert_eq!(
   248	            parse_budget_regime("token_total").unwrap(),
   249	            BudgetRegime::TokenTotal
   250	        );
   251	        assert_eq!(
   252	            parse_budget_regime("wall_clock").unwrap(),
   253	            BudgetRegime::WallClock
   254	        );
   255	    }
   256	
   257	    #[test]
   258	    fn parse_regime_unknown_rejected() {
   259	        match parse_budget_regime("foobar") {
   260	            Err(BudgetError::UnknownRegime(s)) => assert_eq!(s, "foobar"),

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/fc_trace.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// Phase A atom A6 — per-line FC tagging via structured JSON events.
     2	//
     3	// Constitutional anchor: meta-witness for FC1 / FC2 / FC3 path coverage.
     4	// Codex / Gemini N-agents brainstorm 2026-04-25 § D ("Constitutional /
     5	// Engineering"): "add OpenTelemetry-style spans or structured JSONL
     6	// events for the constitutional stages". TRACE_MATRIX_v2 § 5 item 7 had
     7	// this deferred to Phase A6 with the note "will land before Phase B
     8	// (homogeneous experiments)".
     9	//
    10	// Why now (Phase A pre-flight, not Phase B): once Phase B starts wiring
    11	// kernel instrumentation (cost / wall-clock / dual PPUT), silent FC
    12	// drift becomes catastrophic — a Soft Law mode that bypasses FC1-N12
    13	// and stamps post-hoc verified=true would *look* like Phase B success.
    14	// FC tracing in pre-flight gives a zero-cost-when-disabled audit trail
    15	// that downstream Phase D ArchitectAI can replay deterministically.
    16	//
    17	// Design constraints:
    18	//   1. Zero new crate dependencies (autonomous-safe; pure stdlib).
    19	//   2. Zero overhead when `FC_TRACE` env is unset (single env::var()
    20	//      check at startup; no per-call var read after).
    21	//   3. Append-only, line-delimited JSON to stderr by default; or to
    22	//      `FC_TRACE_FILE` for batch capture under runner.sh.
    23	//   4. Stable event shape — Phase D consumers MUST be able to join
    24	//      events on `fc_id` + `run_id` + monotonic timestamp without
    25	//      schema knowledge.
    26	//
    27	// Event shape (one JSON object per line):
    28	//   {"ts_ms": 1714123456789, "fc_id": "FC2-N22",
    29	//    "run_id": "n3_mathd_42_169123", "tx": 17,
    30	//    "agent_id": "Agent_2", "kv": {...arbitrary key-value...}}
    31	//
    32	// Phase D+ extension (out-of-scope for A6): convert to true OpenTelemetry
    33	// spans + replace the file sink with a tracing-subscriber bridge. The
    34	// macro surface here was kept small specifically so that swap is local.
    35	
    36	use std::io::Write;
    37	use std::sync::OnceLock;
    38	
    39	/// TRACE_MATRIX FC-trace meta-witness: canonical FC node identifiers
    40	/// the evaluator emits events for. Adding a new variant is a Phase B+
    41	/// schema change — Phase D ArchitectAI joins on these strings.
    42	///
    43	/// `Display::fmt` produces the dash-separated form used in
    44	/// TRACE_MATRIX rows (e.g. `FC2-N22`, NOT `FC2N22` or `fc2_n22`); that
    45	/// stable string is what flows into the emitted JSON.
    46	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    47	pub enum FcId {
    48	    /// δ/AI prompt construction (per-Agent_i).
    49	    Fc1N7,
    50	    /// ∏p decision diversity — one event per accepted/rejected proposal
    51	    /// payload (append/complete/step). Distinct vs total flow into the
    52	    /// A4 `tactic_diversity` aggregate.
    53	    Fc1N11,
    54	    /// Lean oracle scope — bracketing every verify_omega /
    55	    /// verify_omega_detailed / verify_partial call. Cumulative
    56	    /// elapsed flows into the A4 `verifier_wait_ms` aggregate.
    57	    Fc1N12,
    58	    /// ∏p=0 → preserve Q_t. Fired when forbidden_pattern matches OR
    59	    /// Lean returns Ok(false).
    60	    Fc1E18,
    61	    /// mr / map-reduce tick. Fired at every `tick_interval` boundary.
    62	    Fc2N20,
    63	    /// HALT — terminal node. `kv.reason` carries
    64	    /// {OmegaAccepted, MaxTxExhausted, ErrorHalt} per FC2-N23.
    65	    Fc2N22,
    66	    /// WAL append (logs subgraph). Fired on every `bus.append_*`
    67	    /// success when WAL_DIR is configured.
    68	    Fc3N31,
    69	}
    70	
    71	impl FcId {
    72	    /// Stable string used in emitted JSON. Phase D consumers and
    73	    /// TRACE_MATRIX rows join on this exact spelling.
    74	    pub fn as_str(&self) -> &'static str {
    75	        match self {
    76	            FcId::Fc1N7 => "FC1-N7",
    77	            FcId::Fc1N11 => "FC1-N11",
    78	            FcId::Fc1N12 => "FC1-N12",
    79	            FcId::Fc1E18 => "FC1-E18",
    80	            FcId::Fc2N20 => "FC2-N20",
    81	            FcId::Fc2N22 => "FC2-N22",
    82	            FcId::Fc3N31 => "FC3-N31",
    83	        }
    84	    }
    85	}
    86	
    87	impl std::fmt::Display for FcId {
    88	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    89	        f.write_str(self.as_str())
    90	    }
    91	}
    92	
    93	/// FC_TRACE=1 enables event emission. Read once at first invocation
    94	/// (cached in OnceLock) so subsequent emit calls are a single atomic
    95	/// load. Kept process-global because the evaluator binary is one
    96	/// process per run — no need for finer scoping.
    97	static FC_TRACE_ENABLED: OnceLock<bool> = OnceLock::new();
    98	
    99	/// FC_TRACE_FILE=<path> redirects emit to the file (truncate-on-open).
   100	/// Default sink = stderr. Acquired once per process; the file handle
   101	/// is held for the lifetime of the binary.
   102	static FC_TRACE_SINK: OnceLock<std::sync::Mutex<Box<dyn Write + Send>>> =
   103	    OnceLock::new();
   104	
   105	const FC_TRACE_ENV_VAR: &str = "FC_TRACE";
   106	const FC_TRACE_FILE_ENV_VAR: &str = "FC_TRACE_FILE";
   107	
   108	/// True iff `FC_TRACE` env var is set to `1` at first call. Cheap to
   109	/// call repeatedly (single OnceLock read).
   110	pub fn fc_trace_enabled() -> bool {
   111	    *FC_TRACE_ENABLED.get_or_init(|| {
   112	        std::env::var(FC_TRACE_ENV_VAR).as_deref() == Ok("1")
   113	    })
   114	}
   115	
   116	/// Emit one event line. Caller passes the FC node id + a slice of
   117	/// (key, JSON-fragment) pairs. Caller is responsible for JSON-encoding
   118	/// the values (use `json_str` for strings; integers / booleans as-is).
   119	///
   120	/// Events are skipped when `FC_TRACE` is unset — this is the cold path
   121	/// in production runs.
   122	///
   123	/// `run_id` and `tx` are passed positionally so the macro stays
   124	/// readable at the call site; future Phase D+ may promote them to a
   125	/// thread-local context. `agent_id` is `None` for run-level events
   126	/// (boot, halt, mr tick).
   127	pub fn emit_event(
   128	    fc: FcId,
   129	    run_id: &str,
   130	    tx: Option<u64>,
   131	    agent_id: Option<&str>,
   132	    kv: &[(&str, String)],
   133	) {
   134	    if !fc_trace_enabled() {
   135	        return;
   136	    }
   137	    let ts_ms = std::time::SystemTime::now()
   138	        .duration_since(std::time::UNIX_EPOCH)
   139	        .map(|d| d.as_millis() as u64)
   140	        .unwrap_or(0);
   141	
   142	    let mut line = String::with_capacity(128 + 32 * kv.len());
   143	    line.push('{');
   144	    write_kv_unchecked(&mut line, "ts_ms", &ts_ms.to_string(), false);
   145	    line.push(',');
   146	    write_kv_unchecked(&mut line, "fc_id", &json_str(fc.as_str()), false);
   147	    line.push(',');
   148	    write_kv_unchecked(&mut line, "run_id", &json_str(run_id), false);
   149	    if let Some(t) = tx {
   150	        line.push(',');
   151	        write_kv_unchecked(&mut line, "tx", &t.to_string(), false);
   152	    }
   153	    if let Some(a) = agent_id {
   154	        line.push(',');
   155	        write_kv_unchecked(&mut line, "agent_id", &json_str(a), false);
   156	    }
   157	    if !kv.is_empty() {
   158	        line.push_str(r#","kv":{"#);
   159	        for (i, (k, v)) in kv.iter().enumerate() {
   160	            if i > 0 {
   161	                line.push(',');
   162	            }
   163	            write_kv_unchecked(&mut line, k, v, false);
   164	        }
   165	        line.push('}');
   166	    }
   167	    line.push('}');
   168	    line.push('\n');
   169	
   170	    // Lock-then-write. The Mutex is per-process; contention is bounded
   171	    // by emit rate (~1-10 events per tx, max_tx ~200, n_agents ~8 →
   172	    // < 16K events per run → negligible).
   173	    let sink = FC_TRACE_SINK.get_or_init(init_sink);
   174	    if let Ok(mut s) = sink.lock() {
   175	        let _ = s.write_all(line.as_bytes());
   176	    }
   177	}
   178	
   179	fn init_sink() -> std::sync::Mutex<Box<dyn Write + Send>> {
   180	    let sink: Box<dyn Write + Send> = match std::env::var(FC_TRACE_FILE_ENV_VAR) {
   181	        Ok(path) if !path.is_empty() => match std::fs::OpenOptions::new()
   182	            .create(true)
   183	            .append(true)
   184	            .open(&path)
   185	        {
   186	            Ok(f) => Box::new(f),
   187	            Err(_) => Box::new(std::io::stderr()),
   188	        },
   189	        _ => Box::new(std::io::stderr()),
   190	    };
   191	    std::sync::Mutex::new(sink)
   192	}
   193	
   194	/// JSON-escape a string with surrounding double quotes. Handles the
   195	/// minimal escape set required by RFC 8259 § 7. Public so callers can
   196	/// pre-encode string values for `kv` slots.
   197	pub fn json_str(s: &str) -> String {
   198	    let mut out = String::with_capacity(s.len() + 2);
   199	    out.push('"');
   200	    for c in s.chars() {
   201	        match c {
   202	            '"' => out.push_str(r#"\""#),
   203	            '\\' => out.push_str(r"\\"),
   204	            '\n' => out.push_str(r"\n"),
   205	            '\r' => out.push_str(r"\r"),
   206	            '\t' => out.push_str(r"\t"),
   207	            c if (c as u32) < 0x20 => {
   208	                out.push_str(&format!("\\u{:04x}", c as u32));
   209	            }
   210	            c => out.push(c),
   211	        }
   212	    }
   213	    out.push('"');
   214	    out
   215	}
   216	
   217	fn write_kv_unchecked(buf: &mut String, key: &str, value_json: &str, _trailing_comma: bool) {
   218	    buf.push('"');
   219	    buf.push_str(key);
   220	    buf.push_str("\":");
   221	    buf.push_str(value_json);
   222	}
   223	
   224	#[cfg(test)]
   225	mod tests {
   226	    use super::*;
   227	    use std::sync::Mutex;
   228	
   229	    // Serialise env mutation per memory feedback_env_var_test_lock.
   230	    static ENV_LOCK: Mutex<()> = Mutex::new(());
   231	
   232	    #[test]
   233	    fn fc_id_strings_are_stable() {
   234	        // Phase D consumers + TRACE_MATRIX rows join on these exact
   235	        // spellings; any rename here is a breaking schema change.
   236	        assert_eq!(FcId::Fc1N7.as_str(), "FC1-N7");
   237	        assert_eq!(FcId::Fc1N11.as_str(), "FC1-N11");
   238	        assert_eq!(FcId::Fc1N12.as_str(), "FC1-N12");
   239	        assert_eq!(FcId::Fc1E18.as_str(), "FC1-E18");
   240	        assert_eq!(FcId::Fc2N20.as_str(), "FC2-N20");
   241	        assert_eq!(FcId::Fc2N22.as_str(), "FC2-N22");
   242	        assert_eq!(FcId::Fc3N31.as_str(), "FC3-N31");
   243	    }
   244	
   245	    #[test]
   246	    fn fc_id_display_matches_as_str() {
   247	        assert_eq!(format!("{}", FcId::Fc2N22), "FC2-N22");
   248	    }
   249	
   250	    #[test]
   251	    fn json_str_escapes_required_chars() {
   252	        // RFC 8259 § 7 minimal escape set.
   253	        assert_eq!(json_str(r#"a"b"#), r#""a\"b""#);
   254	        assert_eq!(json_str(r"a\b"), r#""a\\b""#);
   255	        assert_eq!(json_str("a\nb"), r#""a\nb""#);
   256	        assert_eq!(json_str("a\tb"), r#""a\tb""#);
   257	        // Control chars get \u escapes
   258	        assert_eq!(json_str("\x01"), r#""\u0001""#);
   259	        assert_eq!(json_str("\x1f"), r#""\u001f""#);
   260	        // Plain ASCII passes through unchanged

exec
/bin/bash -lc "nl -ba src/drivers/llm_proxy.py | sed -n '1,280p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	#!/usr/bin/env python3
     2	"""
     3	LLM Proxy v4 — OpenAI-compatible local HTTP server with token metering.
     4	
     5	Phase A atom A7. Adapted from v3's `src/drivers/llm_proxy.py` with one
     6	load-bearing v4 change: per-provider multi-key round-robin so the three
     7	SiliconFlow keys (SILICONFLOW_API_KEY / _SECONDARY / _TERTIARY) split
     8	concurrent traffic across separate rate-limit pools — the V3L-27
     9	N=30 → 401/429 collapse documented in `cases/V3_LESSONS.md` was
    10	single-key. The same pattern extends to other providers if multiple
    11	keys are configured.
    12	
    13	Endpoints:
    14	  POST /v1/chat/completions  (OpenAI-compatible, forwards to cloud APIs)
    15	  GET  /health
    16	  GET  /stats               (token counters + per-key request distribution)
    17	  POST /stats/reset         (reset counters — call before each experiment)
    18	
    19	Usage:
    20	  SILICONFLOW_API_KEY=sk-xxx \\
    21	  SILICONFLOW_API_KEY_SECONDARY=sk-yyy \\
    22	  SILICONFLOW_API_KEY_TERTIARY=sk-zzz \\
    23	    python3 src/drivers/llm_proxy.py --port 8080
    24	
    25	Without --provider, model identity drives routing:
    26	  - "deepseek-*" → deepseek
    27	  - "Qwen/...", "openai/...", anything containing "/" → siliconflow
    28	  - else → dashscope
    29	"""
    30	import os, sys, json, logging, argparse, time, threading, itertools
    31	from http.server import HTTPServer, BaseHTTPRequestHandler
    32	from socketserver import ThreadingMixIn
    33	from openai import OpenAI, RateLimitError, APIStatusError
    34	
    35	logging.basicConfig(level=logging.INFO, format='%(asctime)s %(levelname)s %(message)s')
    36	log = logging.getLogger("llm_proxy")
    37	
    38	# Each provider entry: (base_url, [env-var names tried in order]).
    39	# Multiple env names = multi-key round-robin. The PRIMARY name MUST be
    40	# first; any later names are optional fallback / additional pool keys.
    41	PROVIDERS = {
    42	    "dashscope": (
    43	        "https://dashscope.aliyuncs.com/compatible-mode/v1",
    44	        ["DASHSCOPE_API_KEY"],
    45	    ),
    46	    "aliyun": (
    47	        "https://dashscope.aliyuncs.com/compatible-mode/v1",
    48	        ["DASHSCOPE_API_KEY"],
    49	    ),
    50	    "siliconflow": (
    51	        "https://api.siliconflow.cn/v1",
    52	        [
    53	            "SILICONFLOW_API_KEY",
    54	            "SILICONFLOW_API_KEY_SECONDARY",
    55	            "SILICONFLOW_API_KEY_TERTIARY",
    56	        ],
    57	    ),
    58	    "deepseek": (
    59	        "https://api.deepseek.com",
    60	        ["DEEPSEEK_API_KEY"],
    61	    ),
    62	    "volcengine": (
    63	        "https://ark.cn-beijing.volces.com/api/v3",
    64	        ["VOLCENGINE_API_KEY"],
    65	    ),
    66	    "nvidia": (
    67	        "https://integrate.api.nvidia.com/v1",
    68	        ["NVIDIA_NIM_API_KEY"],
    69	    ),
    70	}
    71	
    72	# Per-(provider, key-index) OpenAI client cache: provider -> list[OpenAI]
    73	clients_by_provider = {}
    74	# Round-robin counter per provider.
    75	_rr_counters = {}
    76	_rr_lock = threading.Lock()
    77	# Per-key request counters for /stats observability.
    78	_per_key_requests = {}  # provider -> list[int]
    79	
    80	
    81	def _build_clients(provider):
    82	    """Return list of OpenAI clients for `provider`, one per available key.
    83	
    84	    Lazy. Caches in `clients_by_provider`. Raises ValueError if NO key
    85	    is set for the provider.
    86	    """
    87	    if provider in clients_by_provider:
    88	        return clients_by_provider[provider]
    89	    base_url, key_envs = PROVIDERS[provider]
    90	    keys = []
    91	    for env_name in key_envs:
    92	        v = os.environ.get(env_name, "").strip()
    93	        if v:
    94	            keys.append((env_name, v))
    95	    if not keys:
    96	        raise ValueError(
    97	            f"No keys set for provider={provider}; tried env vars {key_envs}"
    98	        )
    99	    clients = [OpenAI(api_key=k, base_url=base_url) for (_, k) in keys]
   100	    clients_by_provider[provider] = clients
   101	    _per_key_requests[provider] = [0] * len(clients)
   102	    log.info(
   103	        f"[provider {provider}] resolved {len(clients)} key(s) from envs: "
   104	        f"{[name for (name, _) in keys]}"
   105	    )
   106	    return clients
   107	
   108	
   109	def get_client_round_robin(provider):
   110	    """Return (client, key_index) using round-robin across configured keys."""
   111	    clients = _build_clients(provider)
   112	    with _rr_lock:
   113	        idx = _rr_counters.get(provider, 0) % len(clients)
   114	        _rr_counters[provider] = idx + 1
   115	        _per_key_requests[provider][idx] += 1
   116	    return clients[idx], idx
   117	
   118	
   119	# ── Token Metering ──
   120	_stats_lock = threading.Lock()
   121	_stats = {
   122	    "prompt_tokens": 0,
   123	    "completion_tokens": 0,
   124	    "total_tokens": 0,
   125	    "requests": 0,
   126	    "errors": 0,
   127	    "retries_429": 0,
   128	    "estimated_count": 0,
   129	}
   130	
   131	
   132	def _record_usage(prompt_tokens, completion_tokens):
   133	    with _stats_lock:
   134	        _stats["prompt_tokens"] += prompt_tokens
   135	        _stats["completion_tokens"] += completion_tokens
   136	        _stats["total_tokens"] += prompt_tokens + completion_tokens
   137	        _stats["requests"] += 1
   138	
   139	
   140	def _record_estimated():
   141	    with _stats_lock:
   142	        _stats["estimated_count"] += 1
   143	
   144	
   145	def _record_error():
   146	    with _stats_lock:
   147	        _stats["errors"] += 1
   148	
   149	
   150	def _record_retry():
   151	    with _stats_lock:
   152	        _stats["retries_429"] += 1
   153	
   154	
   155	def _reset_stats():
   156	    with _stats_lock:
   157	        for k in _stats:
   158	            _stats[k] = 0
   159	        for prov in _per_key_requests:
   160	            _per_key_requests[prov] = [0] * len(_per_key_requests[prov])
   161	
   162	
   163	def _get_stats():
   164	    with _stats_lock:
   165	        snap = dict(_stats)
   166	        snap["per_key_requests"] = {
   167	            p: list(v) for p, v in _per_key_requests.items()
   168	        }
   169	        return snap
   170	
   171	
   172	# ── Rate Limiter ──
   173	_rate_lock = threading.Lock()
   174	_rate_semaphore = threading.Semaphore(int(os.environ.get("LLM_PROXY_CONCURRENCY", "5")))
   175	_cooldown_until = 0.0
   176	
   177	
   178	def _wait_for_cooldown():
   179	    global _cooldown_until
   180	    now = time.time()
   181	    if now < _cooldown_until:
   182	        wait = _cooldown_until - now
   183	        log.info(f"[RATE LIMITER] Cooling down {wait:.1f}s")
   184	        time.sleep(wait)
   185	
   186	
   187	def _trigger_cooldown(seconds):
   188	    global _cooldown_until
   189	    with _rate_lock:
   190	        new_until = time.time() + seconds
   191	        if new_until > _cooldown_until:
   192	            _cooldown_until = new_until
   193	            log.warning(f"[RATE LIMITER] Global cooldown {seconds}s")
   194	
   195	
   196	def detect_provider(model):
   197	    """Route by model identifier. v4 prefers explicit `provider:model`
   198	    syntax (e.g. `siliconflow:Qwen/Qwen2.5-7B-Instruct`), but also
   199	    handles legacy bare names for backward compat with v3 callers.
   200	    """
   201	    if ":" in model:
   202	        prefix = model.split(":", 1)[0].lower()
   203	        if prefix in PROVIDERS:
   204	            return prefix
   205	    m = model.lower()
   206	    if "deepseek" in m:
   207	        return "deepseek"
   208	    if "/" in model and not m.startswith("qwen"):
   209	        return "siliconflow"
   210	    if m.startswith("qwen"):
   211	        return "dashscope"
   212	    return "dashscope"
   213	
   214	
   215	def strip_provider_prefix(model):
   216	    """If model is `provider:foo/bar`, return `foo/bar`; else `model`."""
   217	    if ":" in model:
   218	        prefix, rest = model.split(":", 1)
   219	        if prefix.lower() in PROVIDERS:
   220	            return rest
   221	    return model
   222	
   223	
   224	class Handler(BaseHTTPRequestHandler):
   225	    def do_GET(self):
   226	        if self.path == "/health":
   227	            self._json_response(200, {"status": "ok"})
   228	        elif self.path == "/stats":
   229	            self._json_response(200, _get_stats())
   230	        else:
   231	            self.send_error(404)
   232	
   233	    def do_POST(self):
   234	        if self.path == "/stats/reset":
   235	            _reset_stats()
   236	            self._json_response(200, {"status": "reset"})
   237	            log.info("[STATS] Counters reset")
   238	            return
   239	
   240	        if "/v1/chat/completions" not in self.path:
   241	            self.send_error(404)
   242	            return
   243	
   244	        length = int(self.headers.get("Content-Length", 0))
   245	        body = json.loads(self.rfile.read(length)) if length else {}
   246	
   247	        raw_model = body.get("model", "qwen3-8b")
   248	        messages = body.get("messages", [])
   249	        temperature = body.get("temperature", 0.5)
   250	        max_tokens = body.get("max_tokens", 3072)
   251	        enable_thinking = body.get("enable_thinking", False)
   252	
   253	        provider = FORCED_PROVIDER or detect_provider(raw_model)
   254	        model = strip_provider_prefix(raw_model)
   255	
   256	        try:
   257	            client, key_idx = get_client_round_robin(provider)
   258	
   259	            extra = {}
   260	            if enable_thinking:
   261	                extra["extra_body"] = {"enable_thinking": True}
   262	            elif "qwen3" in model.lower():
   263	                extra["extra_body"] = {"enable_thinking": False}
   264	
   265	            max_retries = 8
   266	            content = ""
   267	            reasoning = ""
   268	            usage_prompt = 0
   269	            usage_completion = 0
   270	            estimated = False
   271	
   272	            _wait_for_cooldown()
   273	            _rate_semaphore.acquire()
   274	            try:
   275	                for attempt in range(max_retries + 1):
   276	                    _wait_for_cooldown()
   277	                    try:
   278	                        if attempt == 0:
   279	                            log.info(
   280	                                f"→ {provider}#k{key_idx}/{model} "

codex
`cargo test --workspace` passed with the packet’s expected 261 passed / 29 ignored / 0 failed, including the boot Trust Root test. I’m running the SiliconFlow smoke now because A7 is an integration gate rather than a Rust test.
exec
/bin/bash -lc 'bash scripts/smoke_siliconflow.sh' in /home/zephryj/projects/turingosv4
 succeeded in 6165ms:
[A7-smoke] SiliconFlow probe — model=Qwen/Qwen2.5-7B-Instruct max_tokens=8
  [primary  ] SILICONFLOW_API_KEY: OK   1622ms; tokens prompt=36 completion=1; content[:32]='ack'
  [secondary] SILICONFLOW_API_KEY_SECONDARY: OK   1836ms; tokens prompt=36 completion=1; content[:32]='ack'
  [tertiary ] SILICONFLOW_API_KEY_TERTIARY: OK   1380ms; tokens prompt=36 completion=1; content[:32]='ack'
[A7-smoke] result: PASS (all configured keys responded)

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '360,760p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   360	        temperature: Some(0.2),
   361	        max_tokens: Some(max_toks),
   362	    };
   363	
   364	    // PPUT-CCL B6 runtime gate: scan the assembled prompt for PPUT scalars
   365	    // before the call goes out. Any leak aborts deterministically — Goodhart
   366	    // shield at the LLM-call boundary.
   367	    assert_no_metric_leak(&request.messages[0].content);
   368	    match client.generate(&request).await {
   369	        Ok(response) => {
   370	            acc.record_llm_call(response.prompt_tokens, response.completion_tokens);
   371	            acc.record_proposal(false);
   372	            // Rule 22 v2 clause 4: reject markdown fences
   373	            if response.content.contains("```") {
   374	                wc.mark_final_accept();
   375	                // P0-A: caller declares both runtime + post-hoc legs.
   376	                // Fence reject = neither leg fired.
   377	                // A4: no Lean call reached → verifier_wait_ms=0;
   378	                // 1 proposal made (the LLM response), 1 distinct.
   379	                return make_pput(problem_file, "oneshot", model,
   380	                                 false, false, start, 0, 0, 1,
   381	                                 None, None, None, None, None,
   382	                                 Some(acc.total_run_token_count()),
   383	                                 Some(acc.failed_branch_count),
   384	                                 wc.elapsed_ms(),
   385	                                 false, 1, 1, verifier_wait_ms,
   386	                                 oneshot_regime, oneshot_budget_base);
   387	            }
   388	
   389	            // Phase A atom A4 (FC1-N12): bracket every Lean call so verifier
   390	            // wait is observable in the emitted v2 row.
   391	            let v_t0 = Instant::now();
   392	            let verdict = oracle.verify_omega(&response.content);
   393	            let v_elapsed = v_t0.elapsed().as_millis() as u64;
   394	            verifier_wait_ms += v_elapsed;
   395	            // A6 FC1-N12 (Lean oracle scope): per-call event with verdict
   396	            // + elapsed_ms. Phase D consumer derives the verifier-cost
   397	            // distribution and the verify-success rate. Run-level emit
   398	            // (no agent_id; oneshot has only one virtual agent).
   399	            let verdict_str = match &verdict {
   400	                Ok(true) => "Ok(true)",
   401	                Ok(false) => "Ok(false)",
   402	                Err(_) => "Err",
   403	            };
   404	            minif2f_v4::fc_trace::emit_event(
   405	                minif2f_v4::fc_trace::FcId::Fc1N12,
   406	                &format!("oneshot_{}", problem_file), None, None,
   407	                &[
   408	                    ("verdict", minif2f_v4::fc_trace::json_str(verdict_str)),
   409	                    ("elapsed_ms", v_elapsed.to_string()),
   410	                ],
   411	            );
   412	            // B3: close the bracket AFTER the Lean call returns, regardless of
   413	            // verdict. Soft Law mode (Phase C) cannot escape the verify-time
   414	            // accounting by short-circuiting on runtime accept.
   415	            wc.mark_final_accept();
   416	            match verdict {
   417	                Ok(true) => {
   418	                    acc.flip_last_failed_to_accepted();
   419	                    let gp_tokens = response.completion_tokens as u64;
   420	                    let preview: String = response.content.chars().take(500).collect();
   421	                    info!(">>> OMEGA ACCEPTED <<< (path=alone, payload[0..500]={:?})", preview);
   422	                    let proof_file = persist_proof_artifact(
   423	                        problem_file, theorem_name, problem_statement,
   424	                        &response.content, "alone", "oneshot",
   425	                    );
   426	                    // P0-A: Phase B oneshot success — runtime gate IS the
   427	                    // Lean verify call (oracle.verify_omega returned Ok(true)),
   428	                    // so both legs hold. Phase C Soft Law would inject a
   429	                    // separate `verify_post_hoc(&oracle, &response.content)`
   430	                    // call here and pass its result as post_hoc_verified.
   431	                    make_pput(problem_file, "oneshot", model,
   432	                              true, true, start, gp_tokens, 1, 1,
   433	                              None, None, Some(response.content.clone()),
   434	                              Some("alone".to_string()), proof_file,
   435	                              Some(acc.total_run_token_count()),
   436	                              Some(acc.failed_branch_count),
   437	                              wc.elapsed_ms(),
   438	                              false, 1, 1, verifier_wait_ms,
   439	                              oneshot_regime, oneshot_budget_base)
   440	                }
   441	                Ok(false) => {
   442	                    // Lean rejected → neither leg.
   443	                    make_pput(problem_file, "oneshot", model,
   444	                              false, false, start, 0, 0, 1,
   445	                              None, None, None, None, None,
   446	                              Some(acc.total_run_token_count()),
   447	                              Some(acc.failed_branch_count),
   448	                              wc.elapsed_ms(),
   449	                              false, 1, 1, verifier_wait_ms,
   450	                              oneshot_regime, oneshot_budget_base)
   451	                }
   452	                Err(e) => {
   453	                    warn!("Oracle error: {}", e);
   454	                    // Lean error → measurement failure → neither leg.
   455	                    make_pput(problem_file, "oneshot", model,
   456	                              false, false, start, 0, 0, 1,
   457	                              None, None, None, None, None,
   458	                              Some(acc.total_run_token_count()),
   459	                              Some(acc.failed_branch_count),
   460	                              wc.elapsed_ms(),
   461	                              false, 1, 1, verifier_wait_ms,
   462	                              oneshot_regime, oneshot_budget_base)
   463	                }
   464	            }
   465	        }
   466	        Err(e) => {
   467	            // C-012: measurement failure ≠ verified failure.
   468	            // Do not emit PPUT_RESULT — batch runner must retry on resume.
   469	            // C-017: broadcast error explicitly (stderr, non-zero exit).
   470	            error!("LLM error: {}", e);
   471	            eprintln!("MEASUREMENT_ERROR oneshot LLM: {}", e);
   472	            std::process::exit(2);
   473	        }
   474	    }
   475	}
   476	
   477	/// Swarm: N agents, prediction market, Boltzmann routing → PPUT.
   478	async fn run_swarm(
   479	    problem_file: &str, problem_statement: &str, theorem_name: &str,
   480	    lean_path: &str, proxy_url: &str, model: &str, n_agents: usize,
   481	) -> PputResult {
   482	    let start = Instant::now();
   483	    let condition = format!("n{}", n_agents);
   484	
   485	    // Phase A atom A6 (FC-trace correlation): pin a stable identifier at
   486	    // run_swarm entry so every fc_event from this run shares one
   487	    // correlation key. Format mirrors make_pput's run_id (condition +
   488	    // problem_id + unix-ms) so Phase D joins via simple equality. The
   489	    // value is read at every emit site below; cheap (clone of small
   490	    // String).
   491	    let run_corr_id = {
   492	        let problem_id = std::path::Path::new(problem_file)
   493	            .file_stem().and_then(|s| s.to_str()).unwrap_or(problem_file);
   494	        let ts = std::time::SystemTime::now()
   495	            .duration_since(std::time::UNIX_EPOCH)
   496	            .map(|d| d.as_millis()).unwrap_or(0);
   497	        format!("{}_{}_{}", condition, problem_id, ts)
   498	    };
   499	
   500	    let kernel = Kernel::new();
   501	    let config = BusConfig {
   502	        // Phase 2.1 (C-043 candidate): OMEGA-accepted proofs are auto-written
   503	        // as tape nodes (mandatory wtool per Art. IV). Full proofs can be
   504	        // long; raise bus caps so winning nodes don't get size-vetoed. Agent
   505	        // partials still typically <1200; no behavioural regression.
   506	        max_payload_chars: 8000,
   507	        max_payload_lines: 200,
   508	        system_lp_amount: 200.0,
   509	        // C-011: decide/omega/native_decide forbidden (brute-force precedent)
   510	        forbidden_patterns: vec![
   511	            "native_decide".into(), "decide".into(), "omega".into(),
   512	            "#eval".into(), "IO.Process".into(),
   513	            "IO.FS".into(), "run_tac".into(), "unsafe".into(),
   514	        ],
   515	    };
   516	
   517	    // Phase 1: opt-in tape persistence via env. WAL_DIR=<dir> enables WAL
   518	    // writes to <dir>/<problem>_<timestamp>.jsonl; resumes if file exists.
   519	    // Default off for backward-compat baseline runs.
   520	    let mut bus = if let Ok(wal_dir) = std::env::var("WAL_DIR") {
   521	        let problem_stem = std::path::Path::new(problem_file)
   522	            .file_stem().map(|s| s.to_string_lossy().into_owned())
   523	            .unwrap_or_else(|| "unknown".into());
   524	        let resume_id = std::env::var("WAL_RESUME_ID").ok();
   525	        let id = resume_id.unwrap_or_else(|| {
   526	            std::time::SystemTime::now()
   527	                .duration_since(std::time::UNIX_EPOCH)
   528	                .map(|d| d.as_secs().to_string())
   529	                .unwrap_or_else(|_| "0".into())
   530	        });
   531	        let wal_path = std::path::Path::new(&wal_dir)
   532	            .join(format!("{}_{}.jsonl", problem_stem, id));
   533	        info!("[wal] using {:?}", wal_path);
   534	        match TuringBus::with_wal_path(kernel, config, wal_path) {
   535	            Ok(b) => b,
   536	            Err(e) => {
   537	                error!("[wal] open failed: {} — falling back to in-memory", e);
   538	                TuringBus::new(Kernel::new(), BusConfig {
   539	                    max_payload_chars: 1200, max_payload_lines: 18,
   540	                    system_lp_amount: 200.0,
   541	                    forbidden_patterns: vec![
   542	                        "native_decide".into(), "decide".into(), "omega".into(),
   543	                        "#eval".into(), "IO.Process".into(), "IO.FS".into(),
   544	                        "run_tac".into(), "unsafe".into(),
   545	                    ],
   546	                })
   547	            }
   548	        }
   549	    } else {
   550	        TuringBus::new(kernel, config)
   551	    };
   552	    // Phase 4 (C-041 candidate): cross-problem wallet persistence. WALLET_STATE
   553	    // env points to a json file; if it exists we load agents' carried-over
   554	    // balances/portfolios, otherwise fresh genesis. No second mint under Law 2:
   555	    // genesis_done is serialised, so on_init is a no-op post first boot.
   556	    let wallet_state_path: Option<std::path::PathBuf> = std::env::var("WALLET_STATE")
   557	        .ok().map(std::path::PathBuf::from);
   558	    let wallet = wallet_state_path.as_ref()
   559	        .and_then(|p| WalletTool::load_from_disk(p))
   560	        .unwrap_or_else(|| WalletTool::new(10000.0));
   561	    if wallet_state_path.is_some() && wallet.genesis_done {
   562	        info!("[wallet] resumed from {:?}; existing agents carry balances",
   563	              wallet_state_path);
   564	    }
   565	    bus.mount_tool(Box::new(wallet));
   566	    bus.mount_tool(Box::new(Lean4Oracle::new(
   567	        problem_statement.to_string(), theorem_name.to_string(), lean_path.to_string(),
   568	    )));
   569	    bus.mount_tool(Box::new(SearchTool::new(
   570	        vec![format!("{}/MiniF2F/Test", std::env::var("MINIF2F_DIR")
   571	            .unwrap_or_else(|_| DEFAULT_MINIF2F_DIR.into()))], 20,
   572	    )));
   573	    bus.mount_tool(Box::new(LibrarianTool::new(
   574	        &format!("{}/skills", std::env::var("EXPERIMENT_DIR").unwrap_or_else(|_| ".".into())), 8,
   575	    )));
   576	
   577	    let agent_ids: Vec<String> = (0..n_agents).map(|i| format!("Agent_{}", i)).collect();
   578	    bus.init(&agent_ids);
   579	    // Phase 4: top-up ensure_agents for any IDs not in the loaded state (zero
   580	    // balance if post-genesis, genesis_coins only on first-ever boot).
   581	    if let Some(wallet) = bus.tools.iter_mut()
   582	        .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>())
   583	    {
   584	        wallet.ensure_agents(&agent_ids);
   585	    }
   586	
   587	    // Phase A atom A3 (FC1-N7 δ/AI): per-agent model assignment via the
   588	    // `AGENT_MODELS` env var. Default (unset/empty) broadcasts the global
   589	    // `model` to every Agent_i. Heterogeneous payloads require
   590	    // `PHASE_D_HETERO_OK=1` (Phase B+C single-model invariant — see
   591	    // `agent_models.rs` module header). Failure is fatal at startup so a
   592	    // misconfigured swarm cannot burn LLM budget on bad model identity.
   593	    let agent_models = match minif2f_v4::agent_models::resolve_agent_models(model, n_agents) {
   594	        Ok(v) => v,
   595	        Err(e) => {
   596	            eprintln!("AGENT_MODELS resolution failed: {}", e);
   597	            std::process::exit(1);
   598	        }
   599	    };
   600	    // Stamp on jsonl: uniform → single canonical name; heterogeneous (Phase D
   601	    // only, gated) → `hetero:{m1|m2|...}` so downstream PPUT analysis can
   602	    // distinguish single-model runs from heterogeneous swarm runs without
   603	    // having to crack open the genesis_payload model_snapshot field.
   604	    let run_model_label: String = {
   605	        let first = &agent_models[0];
   606	        if agent_models.iter().all(|m| m == first) {
   607	            first.clone()
   608	        } else {
   609	            let mut sorted: Vec<&str> = agent_models.iter().map(String::as_str).collect();
   610	            sorted.sort();
   611	            sorted.dedup();
   612	            format!("hetero:{}", sorted.join("|"))
   613	        }
   614	    };
   615	    info!("[swarm/{}] agent_models = [{}] (label={})", condition,
   616	          agent_models.join(","), run_model_label);
   617	
   618	    // Art. II.2.1: "不能抹杀群体异质性" — distinct skills per agent.
   619	    // V3 had Math/Bull/Bear roles. V4: tactic-strategy specialization.
   620	    let agent_skills: Vec<&str> = vec![
   621	        "Focus on algebraic simplification: ring, field_simp, linarith, nlinarith.",
   622	        "Focus on structural reasoning: induction, cases, rcases, constructor.",
   623	        "Focus on rewriting and normalization: simp, norm_num, rw, calc.",
   624	    ];
   625	
   626	    let client = ResilientLLMClient::new(proxy_url, 1800, 2);
   627	    let params = BoltzmannParams::from_env();
   628	    // C-012: seed the Boltzmann RNG so A/B runs are reproducible.
   629	    // Only the LLM sampling remains stochastic; same-problem paired comparison absorbs that.
   630	    let boltzmann_seed: u64 = std::env::var("BOLTZMANN_SEED")
   631	        .ok().and_then(|s| s.parse().ok()).unwrap_or(DEFAULT_BOLTZMANN_SEED);
   632	    let mut boltz_rng = StdRng::seed_from_u64(boltzmann_seed);
   633	
   634	    // Phase A atom A5 (FC2-N22 budget regime resolution): read
   635	    // BUDGET_REGIME + MAX_TRANSACTIONS env, validate at startup, and
   636	    // compute the loop bound. Errors abort BEFORE any LLM call so a
   637	    // misconfigured run cannot consume API budget. Default
   638	    // (env unset) = TotalProposal × 200, preserving Phase B baseline
   639	    // bit-for-bit. PREREG_AMENDMENT_p0_defer § 3 condition 3.
   640	    let (budget_regime, budget_max_tx_base, max_transactions) =
   641	        match minif2f_v4::budget_regime::resolve_budget(n_agents) {
   642	            Ok(t) => t,
   643	            Err(e) => {
   644	                eprintln!("BUDGET_REGIME resolution failed: {}", e);
   645	                std::process::exit(1);
   646	            }
   647	        };
   648	    info!("[budget] regime={} base={} effective_max_tx={} (n_agents={})",
   649	          budget_regime.label(), budget_max_tx_base, max_transactions, n_agents);
   650	    let max_transactions = max_transactions as usize;
   651	
   652	    // Art. IV map-reduce tick: periodic tape statistics (clock → mr → map/reduce)
   653	    let tick_interval: usize = std::env::var("TICK_INTERVAL")
   654	        .ok().and_then(|s| s.parse().ok()).unwrap_or(20);
   655	
   656	    // C-036 startup echo: per-agent (skill, temp) so debugging never grep-source.
   657	    let temp_ladder_on = std::env::var("TEMP_LADDER").ok().as_deref() == Some("1");
   658	    let agent_cfg: Vec<String> = (0..n_agents).map(|i| {
   659	        let s = i % agent_skills.len();
   660	        let t = if temp_ladder_on { (0.10_f64 + (i as f64) * 0.15).min(1.30) } else { 0.2 };
   661	        format!("Agent_{}:skill{}:t={:.2}", i, s, t)
   662	    }).collect();
   663	    info!("[swarm/{}] {}", condition, agent_cfg.join(" "));
   664	
   665	    // C-036 telemetry counters.
   666	    let mut tool_dist: HashMap<String, u32> = HashMap::new();
   667	    let mut omega_payload_hashes: HashSet<u64> = HashSet::new();
   668	    let mut omega_attempts: u32 = 0;
   669	    let mut zero_ticks_run: u32 = 0;
   670	    let mut zero_tick_warned = false;
   671	    // Phase A atom A4 (FC1-N11 ∏p decision diversity): hash every parsed
   672	    // proposal payload (append/complete/step) — broader than `omega_*`
   673	    // which only counts OMEGA attempts. Cheap proxy for semantic
   674	    // diversity (full embedding distance is Phase D+ work).
   675	    let mut proposal_hashes: HashSet<u64> = HashSet::new();
   676	    let mut proposal_count: u64 = 0;
   677	    // Phase A atom A4 (FC1-N12 oracle scope): cumulative wall-clock
   678	    // inside Lean for THIS run. Each verify_omega_detailed and
   679	    // verify_partial call brackets its own elapsed and adds it here.
   680	    let mut verifier_wait_ms: u64 = 0;
   681	    // PPUT-CCL B2: full-run cost C_i — every LLM call + tool stdout summed
   682	    // across all proposals (winning + failed branches). Read at terminal
   683	    // make_pput sites and stamped on the emitted jsonl row.
   684	    let mut acc = RunCostAccumulator::new();
   685	    // PPUT-CCL B3: full-run wall-clock T_i — first agent prompt → final Lean
   686	    // call. Opened on first tx's prompt build, closed before each return.
   687	    let mut wc = RunWallClock::new();
   688	    // Art. III.2: per-agent search result cache (bounded), fed into next prompt.
   689	    let mut search_cache: HashMap<String, Vec<String>> = HashMap::new();
   690	    // F-2026-04-19-05: cap searches per agent; beyond cap we remove `search`
   691	    // from the tool list so agents stop wasting budget on name-match misses.
   692	    let search_cap: u32 = std::env::var("SEARCH_CAP")
   693	        .ok().and_then(|s| s.parse().ok()).unwrap_or(20);
   694	    let mut search_count: HashMap<String, u32> = HashMap::new();
   695	    // PPUT-CCL B7-extra (PREREG § 5.5): calibration treatment toggle.
   696	    // When enabled, every proposal at tx >= ROLLBACK_TX_THRESHOLD is
   697	    // synthetically vetoed. Constitutionally that is FC1-E18 (∏p=0 → Q_t)
   698	    // applied repeatedly; the run then exhausts at FC2-N22 HALT via
   699	    // `HaltReason::MaxTxExhausted`. We short-circuit at the threshold tx
   700	    // for efficiency — see `rollback_sim.rs` module header for why this
   701	    // is observably equivalent to running the loop to natural exhaustion.
   702	    let rollback_sim_on = minif2f_v4::rollback_sim::rollback_simulation_enabled();
   703	    if rollback_sim_on {
   704	        info!("[rollback_sim] PREREG § 5.5 calibration treatment ON \
   705	               (synthetic veto at tx >= {})", minif2f_v4::rollback_sim::ROLLBACK_TX_THRESHOLD);
   706	    }
   707	
   708	    for tx in 0..max_transactions {
   709	        // PPUT-CCL B7-extra: short-circuit guard. Constitutional anchor
   710	        // FC1-E18 + FC2-N22 (existing MaxTxExhausted variant). Stamps
   711	        // tx_count at the threshold, not at max_transactions, so jsonl
   712	        // analysis can distinguish a calibration treatment exit from a
   713	        // real natural exhaustion.
   714	        if minif2f_v4::rollback_sim::should_simulate_rollback(tx as u64, rollback_sim_on) {
   715	            warn!("[rollback_sim] firing at tx={} — synthetic ∏p=0 from this tx, \
   716	                   short-circuit to MaxTxExhausted exit (cost-asymmetric: skips \
   717	                   ~150 LLM calls vs honest vetoed loop; downstream PPUT analysis \
   718	                   MUST honor synthetic_short_circuit=true on this row)", tx);
   719	            // A6 FC2-N22 (HALT): synthetic short-circuit path. Phase D
   720	            // join key: reason="SyntheticShortCircuit" disambiguates from
   721	            // natural MaxTxExhausted (which exits at tx=max_transactions).
   722	            minif2f_v4::fc_trace::emit_event(
   723	                minif2f_v4::fc_trace::FcId::Fc2N22,
   724	                &run_corr_id, Some(tx as u64), None,
   725	                &[("reason", minif2f_v4::fc_trace::json_str("SyntheticShortCircuit"))],
   726	            );
   727	            wc.mark_final_accept();
   728	            // A4: synthetic short-circuit is NOT a max-tx exhaustion (it
   729	            // exits ~150 tx EARLY at the rollback threshold). hit_max_tx
   730	            // stays false — synthetic_short_circuit is the disambiguator
   731	            // for this calibration-treatment path.
   732	            let mut result = make_pput(problem_file, &condition, &run_model_label,
   733	                                       false, false, start, 0, 0,
   734	                                       tx as u64, Some(tool_dist), None,
   735	                                       None, None, None,
   736	                                       Some(acc.total_run_token_count()),
   737	                                       Some(acc.failed_branch_count),
   738	                                       wc.elapsed_ms(),
   739	                                       false,
   740	                                       proposal_hashes.len() as u64,
   741	                                       proposal_count,
   742	                                       verifier_wait_ms,
   743	                                       budget_regime, budget_max_tx_base);
   744	            // B7-extra disambiguator: distinguish this calibration-treatment
   745	            // exit from a natural max-tx exhaustion in downstream PPUT
   746	            // analysis. See PputResult::synthetic_short_circuit doc-comment
   747	            // for the cost-asymmetry note.
   748	            result.synthetic_short_circuit = Some(true);
   749	            return result;
   750	        }
   751	
   752	        // PPUT-CCL B3 (mid-term audit P0-C fix 2026-04-25): open the wall-clock
   753	        // bracket at the top of the FIRST tx (before chain/skill/board build
   754	        // and before build_agent_prompt). Idempotent — only the first tx's
   755	        // call sticks; subsequent calls no-op. PREREG § 5 / plan B3 define
   756	        // T_i as "first agent prompt construction"; this is the earliest
   757	        // moment the agent begins constructing its prompt.
   758	        wc.mark_first_read();
   759	
   760	        // Map-reduce tick (Art. IV mermaid: clock → mr → tape)

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '760,1120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   760	        // Map-reduce tick (Art. IV mermaid: clock → mr → tape)
   761	        if tick_interval > 0 && tx > 0 && tx % tick_interval == 0 {
   762	            let tape_len = bus.kernel.tape.time_arrow().len();
   763	            let market_count = bus.kernel.markets.len();
   764	            let ticker = bus.kernel.market_ticker(5);
   765	            let top_prices: Vec<String> = ticker.iter()
   766	                .map(|(id, p)| format!("{}:{:.0}%", id, p * 100.0))
   767	                .collect();
   768	            info!("[tick@tx{}] tape={} markets={} top={}", tx, tape_len, market_count,
   769	                top_prices.join(", "));
   770	            // A6 FC2-N20 (mr tick): clock → mr → tape per Art. IV.
   771	            // Phase D consumer joins on (run_corr_id, tx) to derive the
   772	            // tape-growth curve and detect zero-tick stalls before they
   773	            // become C-036 alarm events.
   774	            minif2f_v4::fc_trace::emit_event(
   775	                minif2f_v4::fc_trace::FcId::Fc2N20,
   776	                &run_corr_id, Some(tx as u64), None,
   777	                &[
   778	                    ("tape_len", tape_len.to_string()),
   779	                    ("market_count", market_count.to_string()),
   780	                ],
   781	            );
   782	            // Phase 6-emergent: refresh shared team board from facts only.
   783	            // Per-agent cumulative balance + recent tape-node authorship counts
   784	            // + top market prices. No instructions, no "should" — just state.
   785	            if std::env::var("EMERGENT_ROLES").ok().as_deref() == Some("1") {
   786	                let agents_sorted: Vec<String> = agent_ids.clone();
   787	                let mut author_counts: std::collections::HashMap<String, u32> =
   788	                    std::collections::HashMap::new();
   789	                for nid in bus.kernel.tape.time_arrow() {
   790	                    if let Some(n) = bus.kernel.tape.get(nid) {
   791	                        *author_counts.entry(n.author.clone()).or_insert(0) += 1;
   792	                    }
   793	                }
   794	                let wallet_balances: std::collections::HashMap<String, f64> =
   795	                    bus.tools.iter()
   796	                        .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
   797	                        .map(|w| w.balances.clone())
   798	                        .unwrap_or_default();
   799	                let mut board = format!("# tick@tx{} (tape_nodes={})\n", tx, tape_len);
   800	                for a in &agents_sorted {
   801	                    let bal = wallet_balances.get(a).copied().unwrap_or(10000.0);
   802	                    let delta = bal - 10000.0;
   803	                    let nodes = author_counts.get(a).copied().unwrap_or(0);
   804	                    board.push_str(&format!(
   805	                        "- {}: balance={:.0} (Δ{:+.0}), tape_nodes_authored={}\n",
   806	                        a, bal, delta, nodes));
   807	                }
   808	                if !top_prices.is_empty() {
   809	                    board.push_str(&format!("markets: {}\n", top_prices.join(", ")));
   810	                }
   811	                // Preserve any agent posts that were already in the file (append-only).
   812	                if let Some(lib) = bus.tools.iter()
   813	                    .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
   814	                {
   815	                    let existing = lib.read_board();
   816	                    // Keep only the POST lines (they carry agent-originated intent).
   817	                    let posts: String = existing.lines()
   818	                        .filter(|l| l.starts_with("## POST") || (l.starts_with(" ") == false && !l.starts_with("#") && !l.starts_with("-") && !l.starts_with("markets:")))
   819	                        .collect::<Vec<_>>()
   820	                        .join("\n");
   821	                    let full = if posts.is_empty() {
   822	                        board
   823	                    } else {
   824	                        format!("{}\n{}\n", board, posts)
   825	                    };
   826	                    let _ = lib.write_board(&full);
   827	                }
   828	            }
   829	            // C-036 zero-tick alarm: 5 consecutive ticks with no constitutional engine activity.
   830	            if tape_len == 0 && market_count == 0 {
   831	                zero_ticks_run += 1;
   832	                if zero_ticks_run >= 5 && !zero_tick_warned {
   833	                    warn!("[harness] {} consecutive zero-ticks (tape & markets idle) — \
   834	                           constitutional engines bypassed (Art. II.1/II.2 unused)", zero_ticks_run);
   835	                    zero_tick_warned = true;
   836	                }
   837	            } else {
   838	                zero_ticks_run = 0;
   839	            }
   840	        }
   841	
   842	        let agent_idx = tx % n_agents;
   843	        let agent_id = &agent_ids[agent_idx];
   844	        let snap = bus.snapshot();
   845	
   846	        let chain = if snap.tape.is_empty() {
   847	            problem_statement.to_string()
   848	        } else {
   849	            let nodes: Vec<String> = snap.tape.time_arrow().iter()
   850	                .filter_map(|id| snap.tape.get(id))
   851	                .map(|n| format!("[{}] {}: {}", n.id, n.author, n.payload))
   852	                .collect();
   853	            format!("{}\n\n=== Proof Chain ===\n{}", problem_statement, nodes.join("\n"))
   854	        };
   855	
   856	        let errors = bus.recent_rejections(agent_id, 3);
   857	        // Art. II.2.1: per-agent skill specialization + Librarian learned memory
   858	        let base_skill = agent_skills.get(agent_idx % agent_skills.len()).unwrap_or(&"");
   859	        let learned = bus.tools.iter()
   860	            .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
   861	            .and_then(|lib| lib.read_agent_memory(agent_id))
   862	            .unwrap_or_default();
   863	        let skill = if learned.is_empty() {
   864	            base_skill.to_string()
   865	        } else {
   866	            format!("{}\n\n{}", base_skill, learned)
   867	        };
   868	        let hits_ref: Vec<String> = search_cache.get(agent_id).cloned().unwrap_or_default();
   869	        let tools_desc = if search_count.get(agent_id).copied().unwrap_or(0) >= search_cap {
   870	            "append, complete, invest"
   871	        } else {
   872	            "append, complete, invest, search"
   873	        };
   874	        // Phase 6-emergent: read the shared team board. Gated by EMERGENT_ROLES=1
   875	        // so baseline behaviour is untouched. Board content is built by
   876	        // Librarian at periodic ticks (see refresh_board below).
   877	        let team_board: String = if std::env::var("EMERGENT_ROLES").ok().as_deref() == Some("1") {
   878	            bus.tools.iter()
   879	                .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
   880	                .map(|l| l.read_board())
   881	                .unwrap_or_default()
   882	        } else {
   883	            String::new()
   884	        };
   885	        let prompt = build_agent_prompt(
   886	            &chain, &skill, &snap.market_ticker, &errors, &hits_ref,
   887	            snap.get_balance(agent_id), tools_desc, &team_board,
   888	        );
   889	
   890	        // Phase A atom A3: bind δ for this agent_idx (same vector resolved
   891	        // once at run_swarm entry from AGENT_MODELS env). In Phase B+C this
   892	        // is uniform across all agent_idx; in Phase D it may diverge.
   893	        let agent_model = &agent_models[agent_idx];
   894	        // Model-aware max_tokens (same rule as oneshot branch). Per-agent so
   895	        // a heterogeneous Phase D swarm mixing chat + reasoner backbones gets
   896	        // the right ceiling per-call instead of a single global heuristic.
   897	        let max_toks = if agent_model.contains("chat") { 8000 } else { 16000 };
   898	        // Art. II.2.1 anti-homogeneity: per-agent temperature ladder breaks
   899	        // sampling correlation among role-distinct agents (F-2026-04-18-03).
   900	        // Disabled (keep at 0.2) when TEMP_LADDER!=1 to isolate the mechanism.
   901	        let temp: f64 = if std::env::var("TEMP_LADDER").ok().as_deref() == Some("1") {
   902	            (0.10_f64 + (agent_idx as f64) * 0.15).min(1.30)
   903	        } else {
   904	            0.2
   905	        };
   906	        let request = GenerateRequest {
   907	            model: agent_model.clone(),
   908	            messages: vec![Message { role: "user".into(), content: prompt }],
   909	            temperature: Some(temp),
   910	            max_tokens: Some(max_toks),
   911	        };
   912	
   913	        // PPUT-CCL B6 runtime gate (swarm path): swarm prompts include
   914	        // tape contents, board posts, search hits, and learned memory —
   915	        // any of these state surfaces could in principle inject a PPUT
   916	        // value at runtime even when the prompt builder is clean. Gate
   917	        // every tx, every agent, every iteration.
   918	        assert_no_metric_leak(&request.messages[0].content);
   919	        match client.generate(&request).await {
   920	            Ok(response) => {
   921	                acc.record_llm_call(response.prompt_tokens, response.completion_tokens);
   922	                // PPUT-CCL B2: every parsed proposal default-records as failed.
   923	                // OMEGA-accept return paths flip the last record before returning.
   924	                acc.record_proposal(false);
   925	                match parse_agent_output(&response.content) {
   926	                    Ok(action) => match action.tool.as_str() {
   927	                        "append" => {
   928	                            *tool_dist.entry("append".into()).or_insert(0) += 1;
   929	                            if let Some(payload) = &action.payload {
   930	                                // A4: record proposal for tactic_diversity.
   931	                                let mut ph = std::collections::hash_map::DefaultHasher::new();
   932	                                payload.hash(&mut ph);
   933	                                proposal_hashes.insert(ph.finish());
   934	                                proposal_count += 1;
   935	                                let prices: std::collections::HashMap<String, f64> =
   936	                                    snap.markets.iter()
   937	                                        .map(|(id, m)| (id.clone(), m.yes_price))
   938	                                        .collect();
   939	                                let parent = boltzmann_select_parent(
   940	                                    &snap.tape, &prices, &params, &mut boltz_rng
   941	                                );
   942	                                match bus.append(agent_id, payload, parent.as_deref()) {
   943	                                    Ok(BusResult::Appended { node_id }) => {
   944	                                        info!("[tx {}] {} +{}", tx, agent_id, node_id);
   945	                                        // Art. III.2 Librarian: every compress_interval appends,
   946	                                        // write mechanical summary (TopK error classes) to agent's
   947	                                        // learned.md. This is white-box compression (Art. I.2:
   948	                                        // deterministic statistical algorithm), not LLM-based.
   949	                                        if let Some(lib) = bus.tools.iter()
   950	                                            .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>()) {
   951	                                            if lib.should_compress() {
   952	                                                let errors = bus.recent_rejections(agent_id, 10);
   953	                                                let summary = format!(
   954	                                                    "# Learned patterns (auto-compressed)\n\
   955	                                                     Common errors: {}\n\
   956	                                                     Tape depth: {}\n",
   957	                                                    errors.join(", "),
   958	                                                    snap.tape.time_arrow().len(),
   959	                                                );
   960	                                                let _ = lib.write_agent_memory(agent_id, &summary);
   961	                                                info!("[tx {}] Librarian compressed for {}", tx, agent_id);
   962	                                            }
   963	                                        }
   964	                                    }
   965	                                    Ok(BusResult::Vetoed { reason }) => {
   966	                                        warn!("[tx {}] VETO: {}", tx, reason);
   967	                                    }
   968	                                    _ => {}
   969	                                }
   970	                            }
   971	                        }
   972	                        "complete" => {
   973	                            *tool_dist.entry("complete".into()).or_insert(0) += 1;
   974	                            if let Some(payload) = &action.payload {
   975	                                // Art. IV (∏p(output | Q_t)): Q_t (tape) feeds the verification
   976	                                // predicate. Dual-path: try payload-alone first (standalone proof
   977	                                // preserved), then tape+payload (tape-built proof). Accept whichever
   978	                                // succeeds. This keeps Q_t in the ∏p domain without punishing
   979	                                // self-contained proofs that ignored tape.
   980	                                let tape_chain: String = bus.kernel.tape.time_arrow().iter()
   981	                                    .filter_map(|id| bus.kernel.tape.get(id))
   982	                                    .map(|n| n.payload.clone())
   983	                                    .collect::<Vec<_>>()
   984	                                    .join("\n");
   985	                                let tape_len = bus.kernel.tape.time_arrow().len();
   986	                                // C-036: track payload diversity over what agent proposed.
   987	                                let mut h = std::collections::hash_map::DefaultHasher::new();
   988	                                payload.hash(&mut h);
   989	                                omega_payload_hashes.insert(h.finish());
   990	                                omega_attempts += 1;
   991	                                // A4: also record into the broader proposal set
   992	                                // for tactic_diversity (covers append/complete/step).
   993	                                proposal_hashes.insert(h.finish());
   994	                                proposal_count += 1;
   995	                                info!("[tx {}] OMEGA claim by {} (tape_nodes={}, payload_len={})",
   996	                                      tx, agent_id, tape_len, payload.len());
   997	                                let oracle = Lean4Oracle::new(
   998	                                    problem_statement.to_string(),
   999	                                    theorem_name.to_string(),
  1000	                                    lean_path.to_string(),
  1001	                                );
  1002	                                // Path 1: payload alone (A4 verifier_wait bracket)
  1003	                                let v_t0 = Instant::now();
  1004	                                let r_alone = oracle.verify_omega_detailed(payload);
  1005	                                verifier_wait_ms += v_t0.elapsed().as_millis() as u64;
  1006	                                let (full_proof, path_choice, r_final) = match &r_alone {
  1007	                                    Ok((true, _)) => (payload.clone(), "alone", r_alone.clone()),
  1008	                                    _ if !tape_chain.is_empty() => {
  1009	                                        // Path 2: tape + payload (A4 verifier_wait bracket)
  1010	                                        let combined = format!("{}\n{}", tape_chain, payload);
  1011	                                        let v_t1 = Instant::now();
  1012	                                        let r_combined = oracle.verify_omega_detailed(&combined);
  1013	                                        verifier_wait_ms += v_t1.elapsed().as_millis() as u64;
  1014	                                        if matches!(r_combined, Ok((true, _))) {
  1015	                                            *tool_dist.entry("complete_via_tape".into()).or_insert(0) += 1;
  1016	                                        }
  1017	                                        (combined, "tape+payload", r_combined)
  1018	                                    }
  1019	                                    _ => (payload.clone(), "alone", r_alone.clone()),
  1020	                                };
  1021	                                // PPUT-CCL B3: close bracket AFTER both Lean verify paths return.
  1022	                                // Soft Law (Phase C) cannot exit ahead of verify-time accounting.
  1023	                                wc.mark_final_accept();
  1024	                                match r_final {
  1025	                                    Ok((true, _)) => {
  1026	                                        // PPUT-CCL B2: this proposal verified — flip the failed
  1027	                                        // record made at parse time into the run's accepted slot.
  1028	                                        acc.flip_last_failed_to_accepted();
  1029	                                        // Phase 0 (C-039): persist the winning artifact so external
  1030	                                        // verifiers can re-run lean from disk alone.
  1031	                                        let preview: String = full_proof.chars().take(500).collect();
  1032	                                        info!(">>> OMEGA ACCEPTED <<< (path={}, payload[0..500]={:?})",
  1033	                                              path_choice, preview);
  1034	                                        let proof_file = persist_proof_artifact(
  1035	                                            problem_file, &theorem_name, &problem_statement,
  1036	                                            &full_proof, path_choice, agent_id,
  1037	                                        );
  1038	                                        // Phase 2.1 (C-043 candidate): mandatory wtool. Art. IV says
  1039	                                        // `∏p = 1 ⟹ Q_{t+1} = wtool(output)`. Before halting, write
  1040	                                        // the winning payload as a tape node through the standard
  1041	                                        // append pipeline. This automatically fires founder grant
  1042	                                        // (Phase 2 reward-pull) for the winning author and makes
  1043	                                        // every solve end with a canonical tape node on the GP.
  1044	                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
  1045	                                        *tool_dist.entry("omega_wtool".into()).or_insert(0) += 1;
  1046	                                        // Use oracle-blessed path: Lean has already accepted this
  1047	                                        // payload, so bus-level forbidden_patterns and size caps
  1048	                                        // would only re-reject legitimate tactics (e.g. `omega`,
  1049	                                        // `decide` used inside a verified proof — not brute-force).
  1050	                                        let omega_node_id = match bus.append_oracle_accepted(
  1051	                                            agent_id, payload, parent.as_deref(),
  1052	                                        ) {
  1053	                                            Ok(BusResult::Appended { node_id }) => Some(node_id),
  1054	                                            Ok(BusResult::Vetoed { reason }) => {
  1055	                                                warn!("[art-iv] OMEGA wtool VETO (unexpected after oracle accept): {}", reason);
  1056	                                                None
  1057	                                            }
  1058	                                            _ => None,
  1059	                                        };
  1060	                                        let tape_tokens: u64 = bus.kernel.tape.time_arrow().iter()
  1061	                                            .filter_map(|id| bus.kernel.tape.get(id))
  1062	                                            .map(|n| n.payload.len() as u64)
  1063	                                            .sum();
  1064	                                        // C-012: gp_tokens reflects the actual tape (now containing
  1065	                                        // the winner), no double-count needed.
  1066	                                        let gp_tokens = tape_tokens.max(response.completion_tokens as u64);
  1067	                                        let gp = bus.kernel.tape.time_arrow().to_vec();
  1068	                                        let gp_nodes = gp.len();
  1069	                                        if omega_node_id.is_some() {
  1070	                                            info!("[art-iv] OMEGA written as tape node; gp_nodes={}", gp_nodes);
  1071	                                        }
  1072	                                        bus.halt_and_settle(&gp).ok();
  1073	                                        // A6 FC2-N22 (HALT — OmegaAccepted via full proof): the
  1074	                                        // canonical success-path event. Phase D filters on
  1075	                                        // reason="OmegaAccepted" + gp_path="alone|tape+payload" to
  1076	                                        // build the OMEGA accept-rate timeseries.
  1077	                                        minif2f_v4::fc_trace::emit_event(
  1078	                                            minif2f_v4::fc_trace::FcId::Fc2N22,
  1079	                                            &run_corr_id, Some(tx as u64), Some(agent_id.as_str()),
  1080	                                            &[
  1081	                                                ("reason", minif2f_v4::fc_trace::json_str("OmegaAccepted")),
  1082	                                                ("gp_path", minif2f_v4::fc_trace::json_str(path_choice)),
  1083	                                                ("gp_nodes", gp_nodes.to_string()),
  1084	                                            ],
  1085	                                        );
  1086	                                        // Phase 4: persist wallet state so next problem's run
  1087	                                        // inherits carried-over balances (reputation).
  1088	                                        if let Some(ref wp) = wallet_state_path {
  1089	                                            if let Some(w) = bus.tools.iter()
  1090	                                                .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
  1091	                                            {
  1092	                                                if let Err(e) = w.save_to_disk(wp) {
  1093	                                                    warn!("[wallet] save failed to {:?}: {}", wp, e);
  1094	                                                }
  1095	                                            }
  1096	                                        }
  1097	                                        let upr = if omega_attempts > 0 {
  1098	                                            Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
  1099	                                        } else { None };
  1100	                                        // P0-A: Phase B swarm complete — runtime gate IS the
  1101	                                        // Lean verify_omega_detailed call we just consumed
  1102	                                        // (Ok((true, _))). Both legs hold. Phase C Soft Law
  1103	                                        // would inject `verify_post_hoc(&oracle, &full_proof)`
  1104	                                        // here and pass its result as post_hoc_verified.
  1105	                                        return make_pput(problem_file, &condition, &run_model_label,
  1106	                                                        true, true,
  1107	                                                        start, gp_tokens, gp_nodes, tx as u64 + 1,
  1108	                                                        Some(tool_dist), upr,
  1109	                                                        Some(full_proof.clone()),
  1110	                                                        Some(path_choice.to_string()),
  1111	                                                        proof_file,
  1112	                                                        Some(acc.total_run_token_count()),
  1113	                                                        Some(acc.failed_branch_count),
  1114	                                                        wc.elapsed_ms(),
  1115	                                                        false,
  1116	                                                        proposal_hashes.len() as u64,
  1117	                                                        proposal_count,
  1118	                                                        verifier_wait_ms,
  1119	                                                        budget_regime, budget_max_tx_base);
  1120	                                    }

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '1120,1260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1120	                                    }
  1121	                                    Ok((false, err_detail)) => {
  1122	                                        // Step-B v3: classify + record class label (C-022 shield).
  1123	                                        let class = classify_lean_error(&err_detail);
  1124	                                        bus.record_rejection(agent_id, class.label());
  1125	                                        // PPUT-CCL B2: rejection error feeds back into next prompt's
  1126	                                        // recent_rejections — count those bytes against C_i.
  1127	                                        acc.record_tool_stdout(&err_detail);
  1128	                                        let preview: String = payload.chars().take(300).collect();
  1129	                                        warn!("[tx {}] OMEGA rejected ({}). payload[0..300]={:?}", tx, class.label(), preview);
  1130	                                    }
  1131	                                    Err(e) => {
  1132	                                        warn!("[tx {}] OMEGA oracle error: {}", tx, e);
  1133	                                    }
  1134	                                }
  1135	                            }
  1136	                        }
  1137	                        "invest" => {
  1138	                            *tool_dist.entry("invest".into()).or_insert(0) += 1;
  1139	                            // Law 2: Only Investment Costs Money (1 Coin = 1 YES + 1 NO).
  1140	                            // Agent bets on a tape node's quality. This drives price signals
  1141	                            // (Art. II.2) which guide Boltzmann routing (Art. II.2.1).
  1142	                            // Direction: prefer explicit `direction` field (long/short);
  1143	                            // fall back to sign of amount (positive=long, negative=short).
  1144	                            // Bidirectional signals let agents express dissent (Art. II.2).
  1145	                            if let (Some(node_id), Some(amount)) = (&action.node, action.amount) {
  1146	                                let amt = amount.abs();
  1147	                                if amt > 0.0 {
  1148	                                    let buy_yes = match action.direction.as_deref() {
  1149	                                        Some("long") | Some("yes") | Some("LONG") | Some("YES") => true,
  1150	                                        Some("short") | Some("no") | Some("SHORT") | Some("NO") => false,
  1151	                                        _ => amount > 0.0,  // sign-based fallback
  1152	                                    };
  1153	                                    // Law 2 conservation: validate market BEFORE debit (no coin-loss path)
  1154	                                    let market_exists = bus.kernel.yes_price(node_id).is_some();
  1155	                                    if !market_exists {
  1156	                                        warn!("[tx {}] invest: no market for {} (hallucinated node?)", tx, node_id);
  1157	                                    } else {
  1158	                                        // Debit wallet → buy shares → record (atomic intent)
  1159	                                        let wallet_ok = bus.tools.iter_mut()
  1160	                                            .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>())
  1161	                                            .map(|w| w.deduct(agent_id, amt).is_ok())
  1162	                                            .unwrap_or(false);
  1163	                                        if wallet_ok {
  1164	                                            let result = if buy_yes {
  1165	                                                bus.kernel.buy_yes(node_id, amt)
  1166	                                            } else {
  1167	                                                bus.kernel.buy_no(node_id, amt)
  1168	                                            };
  1169	                                            match result {
  1170	                                                Ok(shares) => {
  1171	                                                    info!("[tx {}] {} invested {:.0} {} on {} → {:.1} shares",
  1172	                                                        tx, agent_id, amt,
  1173	                                                        if buy_yes { "YES" } else { "NO" },
  1174	                                                        node_id, shares);
  1175	                                                    if let Some(w) = bus.tools.iter_mut()
  1176	                                                        .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>()) {
  1177	                                                        if buy_yes {
  1178	                                                            w.record_shares(agent_id, node_id, shares, 0.0, 0.0);
  1179	                                                        } else {
  1180	                                                            w.record_shares(agent_id, node_id, 0.0, shares, 0.0);
  1181	                                                        }
  1182	                                                    }
  1183	                                                }
  1184	                                                Err(e) => {
  1185	                                                    // Market existed at check but buy failed — should not happen
  1186	                                                    warn!("[tx {}] invest buy error: {} (coins debited, shares not granted — Law 2 violation logged)", tx, e);
  1187	                                                }
  1188	                                            }
  1189	                                        } else {
  1190	                                            warn!("[tx {}] {} insufficient balance for invest", tx, agent_id);
  1191	                                        }
  1192	                                    }
  1193	                                }
  1194	                            }
  1195	                        }
  1196	                        "search" => {
  1197	                            // F-2026-04-19-05 cap: if over budget this agent's turn the
  1198	                            // search slot shouldn't even be offered, but the LLM may still
  1199	                            // emit `search` ignoring the prompt — record and skip execute.
  1200	                            let cnt = search_count.entry(agent_id.clone()).or_insert(0);
  1201	                            if *cnt >= search_cap {
  1202	                                *tool_dist.entry("search_capped".into()).or_insert(0) += 1;
  1203	                            } else {
  1204	                                *cnt += 1;
  1205	                                *tool_dist.entry("search".into()).or_insert(0) += 1;
  1206	                                // Law 1: search is free. Execute and cache top hits (Art. III.2).
  1207	                                if let Some(query) = &action.query {
  1208	                                    let hits = bus.tools.iter()
  1209	                                        .find_map(|t| t.as_any().downcast_ref::<SearchTool>())
  1210	                                        .map(|s| s.search(query))
  1211	                                        .unwrap_or_default();
  1212	                                    let trimmed: Vec<String> = hits.iter().take(5)
  1213	                                        .map(|p| p.rsplit('/').next().unwrap_or(p).to_string())
  1214	                                        .collect();
  1215	                                    // PPUT-CCL B2: search hits feed `hits_ref` into next prompt —
  1216	                                    // count the cached bytes against C_i.
  1217	                                    acc.record_tool_stdout(&trimmed.join("\n"));
  1218	                                    info!("[tx {}] {} search({:?}) → {} hits: {}",
  1219	                                          tx, agent_id, query, hits.len(), trimmed.join(","));
  1220	                                    search_cache.insert(agent_id.clone(), trimmed);
  1221	                                }
  1222	                            }
  1223	                        }
  1224	                        "post" => {
  1225	                            *tool_dist.entry("post".into()).or_insert(0) += 1;
  1226	                            // Phase 6-emergent: agent posts a short message to the
  1227	                            // shared Librarian board. Other agents see it on next
  1228	                            // prompt. State-only; no central role planner.
  1229	                            if let Some(msg) = &action.payload {
  1230	                                if let Some(lib) = bus.tools.iter()
  1231	                                    .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
  1232	                                {
  1233	                                    if let Err(e) = lib.post_to_board(agent_id, msg) {
  1234	                                        warn!("[tx {}] post failed: {}", tx, e);
  1235	                                    } else {
  1236	                                        info!("[tx {}] {} posted to board", tx, agent_id);
  1237	                                    }
  1238	                                }
  1239	                            }
  1240	                        }
  1241	                        "step" => {
  1242	                            // Phase 7 (C-043+ Turing δ-step): submit ONE tactic,
  1243	                            // oracle classifies the accumulated tape+tactic prefix
  1244	                            // as Complete / PartialOk / Reject. Writes a tape node
  1245	                            // on PartialOk and Complete so the DAG grows one cell
  1246	                            // at a time — the Art. IV semantics Turing 1936 defines.
  1247	                            *tool_dist.entry("step".into()).or_insert(0) += 1;
  1248	                            if let Some(tactic) = &action.payload {
  1249	                                // A4: record proposal for tactic_diversity.
  1250	                                let mut ph = std::collections::hash_map::DefaultHasher::new();
  1251	                                tactic.hash(&mut ph);
  1252	                                proposal_hashes.insert(ph.finish());
  1253	                                proposal_count += 1;
  1254	                                let tape_chain: String = bus.kernel.tape.time_arrow().iter()
  1255	                                    .filter_map(|id| bus.kernel.tape.get(id))
  1256	                                    .map(|n| n.payload.clone())
  1257	                                    .collect::<Vec<_>>()
  1258	                                    .join("\n");
  1259	                                let prefix = if tape_chain.is_empty() {
  1260	                                    tactic.clone()

exec
/bin/bash -lc "nl -ba genesis_payload.toml | sed -n '1,150p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TuringOS v4 — Genesis payload (Phase B7).
     2	#
     3	# Frozen at Phase B B7 (2026-04-25). Two binding sections:
     4	#
     5	#   [pput_accounting_0] — semantic invariants of the PPUT measure
     6	#                         (PREREG § 1.8). baseline_regression_rate +
     7	#                         baseline_regression_jsonl_sha256 are placeholders
     8	#                         until B7-extra (p_0 calibration) lands.
     9	#
    10	#   [trust_root]        — SHA-256 of every load-bearing file. Boot
    11	#                         (`turingosv4::boot::verify_trust_root`) recomputes
    12	#                         each hash and aborts with TRUST_ROOT_TAMPERED on
    13	#                         mismatch.
    14	#
    15	# Manifest derivation (independently re-derived in B7 from PREREG § 1.8 +
    16	# B2-B4 mid-term audit recommendation + B6 prompt_guard add):
    17	#
    18	#   PREREG § 1.8 base (8):
    19	#     src/kernel.rs, src/wal.rs, src/bus.rs,
    20	#     experiments/minif2f_v4/src/lean4_oracle.rs,
    21	#     constitution.md, cases/MANIFEST.sha256 (proxy for cases/*.yaml glob),
    22	#     handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json,
    23	#     handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md
    24	#
    25	#   Mid-term audit add (PPUT accounting layer, 6):
    26	#     src/drivers/llm_http.rs (cost source of truth),
    27	#     experiments/minif2f_v4/src/cost_aggregator.rs (B2),
    28	#     experiments/minif2f_v4/src/wall_clock.rs (B3),
    29	#     experiments/minif2f_v4/src/post_hoc_verifier.rs (B4),
    30	#     experiments/minif2f_v4/src/jsonl_schema.rs (B1),
    31	#     experiments/minif2f_v4/src/bin/evaluator.rs (the wiring)
    32	#
    33	#   B6 add (1):
    34	#     src/sdk/prompt_guard.rs (PPUT-context-leak runtime gate)
    35	#
    36	#   B7-extra add (1):
    37	#     experiments/minif2f_v4/src/rollback_sim.rs (PPUT-CCL § 5.5
    38	#       calibration treatment toggle — synthetic ∏p=0 from tx 50,
    39	#       constitutionally FC1-E18 + FC2-N22-MaxTxExhausted)
    40	#
    41	#   2026-04-25 dual-audit fixes (4):
    42	#     src/main.rs (audit Q2.b VETO — verify_trust_root call site must be
    43	#       in manifest; comment-out = silent bypass)
    44	#     Cargo.lock (audit Q2.e VETO — supply-chain dep-version swap defense)
    45	#     handover/preregistration/scripts/run_p0_calibration.sh (audit Q2 +
    46	#       DO-178C tool qualification — runner is a frozen production tool)
    47	#     handover/preregistration/scripts/compute_p0.py (same — estimator is
    48	#       a frozen production tool)
    49	#
    50	#   2026-04-25 Phase A0 harness modernization (4):
    51	#     rules/MANIFEST.sha256 (proxy for 14 rules/active/R-*.yaml — governance
    52	#       rules that must not be silently weakened; per case C-075 DO-178C
    53	#       tool qualification)
    54	#     rules/engine.py (rule engine; tampering = silent rule bypass)
    55	#     .claude/hooks/judge.sh (PreToolUse hook that invokes engine.py +
    56	#       implements R-016 fc_trace_in_commit + constitution.md special-case)
    57	#     tests/fc_alignment_conformance.rs (per CLAUDE.md "每个 ✅ 行 ≥1
    58	#       witness test"; tampering = silent constitutional drift, defeats
    59	#       FC1/2/3 ↔ symbol mapping enforcement)
    60	#
    61	#   2026-04-25 Phase A1 PREREG amendment (1):
    62	#     handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md
    63	#       (defers PREREG § 5.5 calibration; substitutes p_0 = 0.10
    64	#       conservative ceiling until 5 re-calibration conditions met;
    65	#       per case C-073 ArchitectAI commit authority)
    66	#
    67	# Total: 25 files. genesis_payload.toml itself is conceptually frozen but
    68	# not self-hashed (chicken-and-egg) — the [pput_accounting_0] section
    69	# values are the semantic anchor.
    70	#
    71	# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
    72	
    73	[pput_accounting_0]
    74	schema_version = "1.0"
    75	progress_definition = "1 iff Lean ground-truth verifies golden_path_payload"
    76	cost_definition = "sum(prompt_tokens + completion_tokens + tool_tokens) over all proposals in the run"
    77	time_definition = "wall_clock from first agent prompt construction to final Lean accept or external timeout"
    78	verified_predicate = "experiments/minif2f_v4/src/lean4_oracle.rs::verify_omega_detailed"
    79	heldout_sealed_hash = "51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b"
    80	source_pool_sha256 = "77179cf2598b0df707d78a6663e763121dfe8e73a6538073155f488feab95093"
    81	baseline_regression_rate = 0.0  # PLACEHOLDER until Phase B7-extra calibration
    82	baseline_regression_jsonl_sha256 = ""  # PLACEHOLDER until Phase B7-extra
    83	k_max = 10
    84	n_max = 34
    85	
    86	[trust_root]
    87	"src/main.rs" = "622fee2d96a980d24f9fbaab3d0531c195a0a337fc3ddd2efb60bca90a1cfbf9"
    88	"Cargo.lock" = "577446e8fe11e91bc8751bf13e5ddca6c5faa64d3309b878768c550d3e6feb98"
    89	"handover/preregistration/scripts/run_p0_calibration.sh" = "5f4a57dd8b8280ffe04bec89350a57d876d06cc179d9f8841a522e7bdcf1b8b7"
    90	"handover/preregistration/scripts/compute_p0.py" = "fd1dff97d582f4f509249852be500cba7d81efc9bab8fcb9872b2d85bb4e3052"
    91	"rules/MANIFEST.sha256" = "a84d114a12680c596e1a5458a954a5829b21baa4530f197b9aba65f95443be14"
    92	"rules/engine.py" = "932d9a2b7a3249a7eb5825c0b5c714a9913cd9aa9e058f789e64992b140e40b3"
    93	".claude/hooks/judge.sh" = "a2be9e6ed51e39f2e9cfd302d62a0234a772abc41f145702143d2557dd6fda3e"
    94	"tests/fc_alignment_conformance.rs" = "b3e75979ad2d175b9c45135be6ea1d94ce95184c6896468330c12dbfc1f719db"
    95	"src/kernel.rs" = "893fd67534caf7a3d9abd6efbd202556348b6491cd6d4c6bdb224d2ad75b1af0"
    96	"src/wal.rs" = "1ac7637aa09531b1c7232163f5df48f7193251594c4ed20e0d0fc85cea8f84dc"
    97	"src/bus.rs" = "df28ffe514a3272a3d10fca4568fd424a76e754e9785c109a5459f163f7fd14c"
    98	"src/drivers/llm_http.rs" = "615596b68956b4a8925110edc99aa746a5543527724ec394bb9dda163147ed7a"
    99	"src/drivers/llm_proxy.py" = "c1bc508c64e39fbaad246b2c36781e8f37bc216c6d8a207ee25f37c2e8b13fcb"
   100	"scripts/smoke_siliconflow.sh" = "6ad54e7c0ab8221f475360dcad80eeeb0d6da0fd55c27acdb1cefb2b390f5341"
   101	"scripts/_smoke_siliconflow.py" = "778eea2988312f250efa47fcfe620d86187d01b96f07a98501f9795a333726ca"
   102	"src/sdk/prompt_guard.rs" = "b4f7b164770d1a7203b8143f773c66f748994d60a42ece38471f2f7f2839f4f1"
   103	"experiments/minif2f_v4/src/lean4_oracle.rs" = "70fae24cd17f410c10a092e797fcdedea962db3d7cb20f218d02303edae9e98c"
   104	"experiments/minif2f_v4/src/cost_aggregator.rs" = "896b6905dbca9e9736f8896cd5725c16b6e87c6ad3ff822e044975febed46a03"
   105	"experiments/minif2f_v4/src/wall_clock.rs" = "2c9197f8f93b7d130dc7b094a6664f8ece351ec85668921c5beb6d100a1a77ee"
   106	"experiments/minif2f_v4/src/post_hoc_verifier.rs" = "9a93ae5548827b60543df779c67a5f1201b49a681a55e2cbc6a8532e8bb131a2"
   107	"experiments/minif2f_v4/src/jsonl_schema.rs" = "5d0c5a2bcceba42abbc6f5cd58e045e2d5d7a9edc565db4515f3917a21dc520b"
   108	"experiments/minif2f_v4/src/rollback_sim.rs" = "9ccdf177813452bb83221bd780f2f7f10da619117aca8e44d3ecb355505fe561"
   109	"experiments/minif2f_v4/src/agent_models.rs" = "b736b91efb8a93d884b64028da79d5d6af0525e8c7e8f7222055118f43dc26ce"
   110	"experiments/minif2f_v4/src/budget_regime.rs" = "e0c2ba6533444e6178d9e12d0f182d29c3203a929ea0914d3098efd0397948ea"
   111	"experiments/minif2f_v4/src/fc_trace.rs" = "8578edf61148e50abdf8d29a2efdc5114ee20ab727c1ccd6c5a88ad07e7ffa09"
   112	"experiments/minif2f_v4/src/bin/evaluator.rs" = "149f058b14e0ca414ffe57143fc7a7744b997242fe65cd078596d1b2f9d20422"
   113	"constitution.md" = "91e5913603eebaf134ac93d0f9ed7bb2b7916132203f6c2fae84d73329056ad6"
   114	"cases/MANIFEST.sha256" = "dbe48bc11cac18ad1164c55b11aefd5be4a7b546e31c3df08fb4403c7f41e90d"
   115	"handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json" = "69be702e3e0a4bb33c62bd02056495309289fd9271972dc733b4a1ba3dd908e1"
   116	"handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" = "64d565b4451d1f770425530bb7ffa874da602ddfd9d714ce7f7db6bbfab69463"
   117	"handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md" = "737af38d4b5690d2ac0b3f818ea90ce0e2142fe2d4e444ce44819eadaa6c31d0"

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '1240,1460p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1240	                        }
  1241	                        "step" => {
  1242	                            // Phase 7 (C-043+ Turing δ-step): submit ONE tactic,
  1243	                            // oracle classifies the accumulated tape+tactic prefix
  1244	                            // as Complete / PartialOk / Reject. Writes a tape node
  1245	                            // on PartialOk and Complete so the DAG grows one cell
  1246	                            // at a time — the Art. IV semantics Turing 1936 defines.
  1247	                            *tool_dist.entry("step".into()).or_insert(0) += 1;
  1248	                            if let Some(tactic) = &action.payload {
  1249	                                // A4: record proposal for tactic_diversity.
  1250	                                let mut ph = std::collections::hash_map::DefaultHasher::new();
  1251	                                tactic.hash(&mut ph);
  1252	                                proposal_hashes.insert(ph.finish());
  1253	                                proposal_count += 1;
  1254	                                let tape_chain: String = bus.kernel.tape.time_arrow().iter()
  1255	                                    .filter_map(|id| bus.kernel.tape.get(id))
  1256	                                    .map(|n| n.payload.clone())
  1257	                                    .collect::<Vec<_>>()
  1258	                                    .join("\n");
  1259	                                let prefix = if tape_chain.is_empty() {
  1260	                                    tactic.clone()
  1261	                                } else {
  1262	                                    format!("{}\n{}", tape_chain, tactic)
  1263	                                };
  1264	                                let oracle = Lean4Oracle::new(
  1265	                                    problem_statement.to_string(),
  1266	                                    theorem_name.to_string(),
  1267	                                    lean_path.to_string(),
  1268	                                );
  1269	                                // A4: bracket the Lean partial-verify call.
  1270	                                let v_t0 = Instant::now();
  1271	                                let verdict = oracle.verify_partial(&prefix);
  1272	                                verifier_wait_ms += v_t0.elapsed().as_millis() as u64;
  1273	                                // PPUT-CCL B3: close bracket after step-verify returns.
  1274	                                wc.mark_final_accept();
  1275	                                match verdict {
  1276	                                    PartialVerdict::Complete => {
  1277	                                        acc.flip_last_failed_to_accepted();
  1278	                                        info!(">>> OMEGA ACCEPTED <<< via step (depth={} after this write)",
  1279	                                              bus.kernel.tape.time_arrow().len() + 1);
  1280	                                        let proof_file = persist_proof_artifact(
  1281	                                            problem_file, &theorem_name, &problem_statement,
  1282	                                            &prefix, "per_tactic", agent_id,
  1283	                                        );
  1284	                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
  1285	                                        *tool_dist.entry("omega_wtool".into()).or_insert(0) += 1;
  1286	                                        let _ = bus.append_oracle_accepted(
  1287	                                            agent_id, tactic, parent.as_deref(),
  1288	                                        );
  1289	                                        let tape_tokens: u64 = bus.kernel.tape.time_arrow().iter()
  1290	                                            .filter_map(|id| bus.kernel.tape.get(id))
  1291	                                            .map(|n| n.payload.len() as u64)
  1292	                                            .sum();
  1293	                                        let gp_tokens = tape_tokens.max(response.completion_tokens as u64);
  1294	                                        let gp = bus.kernel.tape.time_arrow().to_vec();
  1295	                                        let gp_nodes = gp.len();
  1296	                                        bus.halt_and_settle(&gp).ok();
  1297	                                        let upr = if omega_attempts > 0 {
  1298	                                            Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
  1299	                                        } else { None };
  1300	                                        // A6 FC2-N22 (HALT — OmegaAccepted via per-tactic
  1301	                                        // PartialVerdict::Complete). Distinguished from the
  1302	                                        // full-proof OMEGA path by gp_path="per_tactic"; both
  1303	                                        // share reason="OmegaAccepted".
  1304	                                        minif2f_v4::fc_trace::emit_event(
  1305	                                            minif2f_v4::fc_trace::FcId::Fc2N22,
  1306	                                            &run_corr_id, Some(tx as u64), Some(agent_id.as_str()),
  1307	                                            &[
  1308	                                                ("reason", minif2f_v4::fc_trace::json_str("OmegaAccepted")),
  1309	                                                ("gp_path", minif2f_v4::fc_trace::json_str("per_tactic")),
  1310	                                                ("gp_nodes", gp_nodes.to_string()),
  1311	                                            ],
  1312	                                        );
  1313	                                        // P0-A: Phase B swarm step Complete — runtime gate IS
  1314	                                        // the Lean verify_partial call (PartialVerdict::Complete).
  1315	                                        // Both legs hold. Phase C Soft Law diverges here.
  1316	                                        return make_pput(problem_file, &condition, &run_model_label,
  1317	                                                        true, true,
  1318	                                                        start, gp_tokens, gp_nodes, tx as u64 + 1,
  1319	                                                        Some(tool_dist), upr,
  1320	                                                        Some(prefix.clone()),
  1321	                                                        Some("per_tactic".to_string()),
  1322	                                                        proof_file,
  1323	                                                        Some(acc.total_run_token_count()),
  1324	                                                        Some(acc.failed_branch_count),
  1325	                                                        wc.elapsed_ms(),
  1326	                                                        false,
  1327	                                                        proposal_hashes.len() as u64,
  1328	                                                        proposal_count,
  1329	                                                        verifier_wait_ms,
  1330	                                                        budget_regime, budget_max_tx_base);
  1331	                                    }
  1332	                                    PartialVerdict::PartialOk => {
  1333	                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
  1334	                                        match bus.append_oracle_accepted(
  1335	                                            agent_id, tactic, parent.as_deref(),
  1336	                                        ) {
  1337	                                            Ok(BusResult::Appended { node_id }) => {
  1338	                                                *tool_dist.entry("step_partial_ok".into()).or_insert(0) += 1;
  1339	                                                info!("[tx {}] {} step+{} partial OK (depth={})",
  1340	                                                      tx, agent_id, node_id,
  1341	                                                      bus.kernel.tape.time_arrow().len());
  1342	                                            }
  1343	                                            Ok(BusResult::Vetoed { reason }) => {
  1344	                                                warn!("[tx {}] step partial OK but bus vetoed: {}", tx, reason);
  1345	                                            }
  1346	                                            _ => {}
  1347	                                        }
  1348	                                    }
  1349	                                    PartialVerdict::Reject(reason) => {
  1350	                                        let class = classify_lean_error(&reason);
  1351	                                        bus.record_rejection(agent_id, class.label());
  1352	                                        // PPUT-CCL B2: step rejection reason flows into next prompt.
  1353	                                        acc.record_tool_stdout(&reason);
  1354	                                        *tool_dist.entry("step_reject".into()).or_insert(0) += 1;
  1355	                                        let preview = reason.chars().take(200).collect::<String>();
  1356	                                        warn!("[tx {}] step rejected ({}): {}", tx, class.label(), preview);
  1357	                                    }
  1358	                                }
  1359	                            }
  1360	                        }
  1361	                        other => {
  1362	                            *tool_dist.entry(format!("other:{}", other)).or_insert(0) += 1;
  1363	                        }
  1364	                    },
  1365	                    Err(e) => {
  1366	                        *tool_dist.entry("parse_fail".into()).or_insert(0) += 1;
  1367	                        // Step-B v3: parse failures feed the class graveyard too.
  1368	                        let class = classify_parse_error(&format!("{}", e));
  1369	                        bus.record_rejection(agent_id, class.label());
  1370	                        // PPUT-CCL B2: classifier label flows into next prompt's errors.
  1371	                        acc.record_tool_stdout(class.label());
  1372	                        warn!("[tx {}] parse: {} ({})", tx, e, class.label());
  1373	                    }
  1374	                }
  1375	            }
  1376	            Err(e) => {
  1377	                *tool_dist.entry("llm_err".into()).or_insert(0) += 1;
  1378	                warn!("[tx {}] LLM: {}", tx, e);
  1379	            }
  1380	        }
  1381	    }
  1382	
  1383	    let upr = if omega_attempts > 0 {
  1384	        Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
  1385	    } else { None };
  1386	    // Phase 4: also save wallet state on no-OMEGA exit. Agents may have
  1387	    // invested/lost Coin during the run; durability should not depend on a win.
  1388	    if let Some(ref wp) = wallet_state_path {
  1389	        if let Some(w) = bus.tools.iter()
  1390	            .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
  1391	        {
  1392	            let _ = w.save_to_disk(wp);
  1393	        }
  1394	    }
  1395	    // No OMEGA found → PPUT = 0
  1396	    // B3: close bracket on max-tx exhaustion path.
  1397	    // P0-A: max-tx exhaustion → neither leg fired.
  1398	    // A4: this is the canonical hit_max_tx=true site (ran the full
  1399	    // for-loop without OMEGA and without firing the synthetic
  1400	    // short-circuit, which would have returned earlier).
  1401	    wc.mark_final_accept();
  1402	    // A6 FC2-N22 (HALT — natural MaxTxExhausted): the canonical
  1403	    // budget-exhausted exit. Phase D filters reason="MaxTxExhausted"
  1404	    // to compute solve_rate-vs-budget curves; pairs with the A5
  1405	    // budget_regime stamp on the v2 jsonl row.
  1406	    minif2f_v4::fc_trace::emit_event(
  1407	        minif2f_v4::fc_trace::FcId::Fc2N22,
  1408	        &run_corr_id, Some(max_transactions as u64), None,
  1409	        &[
  1410	            ("reason", minif2f_v4::fc_trace::json_str("MaxTxExhausted")),
  1411	            ("budget_regime", minif2f_v4::fc_trace::json_str(budget_regime.label())),
  1412	            ("budget_max_transactions", budget_max_tx_base.to_string()),
  1413	            ("proposal_count", proposal_count.to_string()),
  1414	        ],
  1415	    );
  1416	    make_pput(problem_file, &condition, &run_model_label,
  1417	              false, false, start, 0, 0,
  1418	              max_transactions as u64, Some(tool_dist), upr,
  1419	              None, None, None,
  1420	              Some(acc.total_run_token_count()),
  1421	              Some(acc.failed_branch_count),
  1422	              wc.elapsed_ms(),
  1423	              true,
  1424	              proposal_hashes.len() as u64,
  1425	              proposal_count,
  1426	              verifier_wait_ms,
  1427	              budget_regime, budget_max_tx_base)
  1428	}
  1429	
  1430	fn make_pput(
  1431	    problem: &str, condition: &str, model: &str,
  1432	    runtime_accepted: bool, post_hoc_verified: bool, start: Instant,
  1433	    gp_tokens: u64, gp_nodes: usize, tx_count: u64,
  1434	    tool_dist: Option<HashMap<String, u32>>,
  1435	    unique_payload_ratio: Option<f64>,
  1436	    gp_payload: Option<String>,
  1437	    gp_path: Option<String>,
  1438	    gp_proof_file: Option<String>,
  1439	    total_run_token_count: Option<u64>,
  1440	    failed_branch_count: Option<u32>,
  1441	    total_wall_time_ms: Option<u64>,
  1442	    // Phase A atom A4 (decomposed metrics). All callers must pass
  1443	    // explicit values — the v2 fields are non-Optional.
  1444	    hit_max_tx: bool,
  1445	    distinct_proposals: u64,
  1446	    total_proposals: u64,
  1447	    verifier_wait_ms: u64,
  1448	    // Phase A atom A5 (FC2-N22 budget regime stamp). Caller declares
  1449	    // the regime + base BEFORE the loop so MaxTxExhausted rows are
  1450	    // unambiguous about which partitioning rule produced them.
  1451	    budget_regime: minif2f_v4::budget_regime::BudgetRegime,
  1452	    budget_max_transactions: u32,
  1453	) -> PputResult {
  1454	    // PPUT-CCL Phase B B4 (mid-term audit P0-A fix 2026-04-25):
  1455	    // make_pput is now PURELY computational. The caller MUST decide both
  1456	    // `runtime_accepted` (did the evaluator's runtime gate fire?) and
  1457	    // `post_hoc_verified` (did Lean independently confirm the proof?). The
  1458	    // prior implementation derived `post_hoc_verified = has_gp` internally,
  1459	    // which would have laundered Phase C Soft Law fake-accepts into the
  1460	    // North Star pput_verified. Forcing the caller to pass both legs makes

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '1460,1585p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1460	    // North Star pput_verified. Forcing the caller to pass both legs makes
  1461	    // Soft Law's design point unmissable: any caller that fakes runtime
  1462	    // accept must explicitly pass post_hoc_verified=verify_post_hoc(...)
  1463	    // or the divergence will surface immediately.
  1464	    //
  1465	    // Phase B all callers pass `(runtime_accepted, post_hoc_verified) = (X, X)`
  1466	    // because runtime IS Lean today. Phase C diverges at the Soft Law
  1467	    // mode call site, not inside this function.
  1468	    let has_gp = runtime_accepted; // legacy `has_golden_path` field semantics
  1469	    let elapsed = start.elapsed().as_secs_f64();
  1470	    let pput = if has_gp && elapsed > 0.0 { 100.0 / elapsed } else { 0.0 };
  1471	    // C-012 provenance: populated from env vars; None when unset (backward compat).
  1472	    let build_sha = std::env::var("BUILD_SHA").ok();
  1473	    let classifier_version = std::env::var("CLASSIFIER_VERSION").ok();
  1474	    let boltzmann_seed = std::env::var("BOLTZMANN_SEED")
  1475	        .ok().and_then(|s| s.parse::<u64>().ok());
  1476	
  1477	    // Mid-term audit P0-B fix 2026-04-25: collapse Optional accumulator/clock
  1478	    // values into required v2 fields. Phase B always has values for these
  1479	    // (B2 + B3 wire them at every emit site); the prior Option wrapping was
  1480	    // overly defensive and let the v2 schema slip from the contract.
  1481	    let c_i = total_run_token_count.unwrap_or(0);
  1482	    let t_i = total_wall_time_ms.unwrap_or(0);
  1483	    let failed_count = failed_branch_count.unwrap_or(0);
  1484	
  1485	    let progress_runtime = compute_progress_runtime(runtime_accepted);
  1486	    let progress_verified =
  1487	        compute_progress_verified(runtime_accepted, post_hoc_verified);
  1488	    let pput_runtime = compute_pput(progress_runtime, c_i, t_i);
  1489	    let pput_verified = compute_pput(progress_verified, c_i, t_i);
  1490	    let pput_m_verified = compute_pput_m(progress_verified, c_i, t_i);
  1491	
  1492	    // V2 fields read from env (per-process globals).
  1493	    let split = std::env::var("SPLIT").unwrap_or_else(|_| {
  1494	        eprintln!("[v2-emit] SPLIT env unset; defaulting to 'adaptation' \
  1495	                   (Phase B convention; pre-registration requires SPLIT \
  1496	                   for Phase C+ ablation runs)");
  1497	        "adaptation".to_string()
  1498	    });
  1499	    let mode = std::env::var("MODE").unwrap_or_else(|_| "full".to_string());
  1500	    let model_snapshot = std::env::var("MODEL_SNAPSHOT")
  1501	        .unwrap_or_else(|_| model.to_string());
  1502	    let git_sha = build_sha.clone().unwrap_or_default();
  1503	    let binary_sha256 = std::env::var("BINARY_SHA256").unwrap_or_default();
  1504	
  1505	    // problem_id = basename without .lean
  1506	    let problem_id = std::path::Path::new(problem)
  1507	        .file_stem()
  1508	        .and_then(|s| s.to_str())
  1509	        .unwrap_or(problem)
  1510	        .to_string();
  1511	    // run_id = condition + problem_id + ts (collision-free for sequential runs)
  1512	    let ts = std::time::SystemTime::now()
  1513	        .duration_since(std::time::UNIX_EPOCH)
  1514	        .map(|d| d.as_millis())
  1515	        .unwrap_or(0);
  1516	    let run_id = format!("{}_{}_{}", condition, problem_id, ts);
  1517	
  1518	    PputResult {
  1519	        // ── B1 v2 schema fields ──
  1520	        schema_version: "v2.0".to_string(),
  1521	        run_id,
  1522	        problem_id,
  1523	        solved: runtime_accepted,
  1524	        split,
  1525	        verified: post_hoc_verified,
  1526	        golden_path_token_count: gp_tokens,
  1527	        total_run_token_count: c_i,
  1528	        total_wall_time_ms: t_i,
  1529	        progress: progress_verified,
  1530	        pput_runtime,
  1531	        pput_verified,
  1532	        pput_m_verified,
  1533	        failed_branch_count: failed_count,
  1534	        // Phase B placeholders — Phase C+ wires these as the modes activate.
  1535	        rollback_count: 0,
  1536	        hit_max_tx,
  1537	        tactic_diversity: minif2f_v4::jsonl_schema::compute_tactic_diversity(
  1538	            distinct_proposals, total_proposals,
  1539	        ),
  1540	        verifier_wait_ms,
  1541	        budget_regime: budget_regime.label().to_string(),
  1542	        budget_max_transactions,
  1543	        far: 0.0, err: 0.0, iac: 0.0, cpr: 0.0,
  1544	        model_snapshot,
  1545	        git_sha,
  1546	        binary_sha256,
  1547	        mode,
  1548	        // ── Legacy diagnostic fields ──
  1549	        problem: problem.to_string(),
  1550	        condition: condition.to_string(),
  1551	        model: model.to_string(),
  1552	        has_golden_path: has_gp,
  1553	        time_secs: elapsed,
  1554	        pput,
  1555	        gp_token_count: gp_tokens,
  1556	        gp_node_count: gp_nodes,
  1557	        tx_count,
  1558	        build_sha,
  1559	        classifier_version,
  1560	        boltzmann_seed,
  1561	        tool_dist,
  1562	        unique_payload_ratio,
  1563	        gp_payload,
  1564	        gp_path,
  1565	        gp_proof_file,
  1566	        // B7-extra: only the calibration-treatment short-circuit site mutates
  1567	        // this to Some(true). Default = None (most callers).
  1568	        synthetic_short_circuit: None,
  1569	    }
  1570	}
  1571	
  1572	/// Phase 0 (C-039 candidate): persist a self-contained, re-verifiable proof artifact.
  1573	/// Writes <EXPERIMENT_DIR>/proofs/<theorem>_<timestamp>_<short_hash>.lean containing
  1574	/// the exact code that the Lean oracle accepted. An external verifier can run
  1575	/// `lean --stdin < <file>` with the matching toolchain + Mathlib and reproduce the result.
  1576	/// Returns the relative path (for embedding in PputResult) or None on I/O failure.
  1577	fn persist_proof_artifact(
  1578	    problem_file: &str, theorem_name: &str, problem_statement: &str,
  1579	    full_proof: &str, path_choice: &str, agent_id: &str,
  1580	) -> Option<String> {
  1581	    use std::collections::hash_map::DefaultHasher;
  1582	    use std::hash::{Hash, Hasher};
  1583	    let exp_dir = std::env::var("EXPERIMENT_DIR").unwrap_or_else(|_| ".".into());
  1584	    let proofs_dir = std::path::Path::new(&exp_dir).join("proofs");
  1585	    if let Err(e) = std::fs::create_dir_all(&proofs_dir) {

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/jsonl_schema.rs | sed -n '1,210p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// PPUT-CCL JSONL schema v2 — proposal-level + run-level records.
     2	//
     3	// Authoritative spec:
     4	//   handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md § B1
     5	//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md § 5 (definitions)
     6	//
     7	// Versioning: every v2 record carries `schema_version = "v2.0"`. Legacy Paper-1
     8	// era jsonl rows (the `PputResult` shape emitted by evaluator before this commit)
     9	// have NO `schema_version` field, so `RunRecord::from_json` discriminates on
    10	// presence and routes to `LegacyRunAggregate`. No on-disk artifact is rewritten
    11	// by this commit; downstream tooling is the upgrade boundary.
    12	//
    13	// B1 scope: schema definition + round-trip + legacy-compat + zero-progress
    14	// invariant. B2/B3/B4 wire the new fields into evaluator emission paths.
    15	
    16	use serde::{Deserialize, Serialize};
    17	
    18	pub const SCHEMA_VERSION_V2: &str = "v2.0";
    19	
    20	/// Per-proposal row (one per LLM call / append / complete attempt).
    21	///
    22	/// Currently no evaluator emit path produces these — B2 (cost aggregator) and
    23	/// B3 (wall-time) will add the emit sites. This struct is the contract.
    24	#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    25	pub struct ProposalRow {
    26	    pub run_id: String,
    27	    pub problem_id: String,
    28	    pub agent_id: String,
    29	    pub role: String,
    30	    pub branch_id: String,
    31	    pub proposal_hash: String,
    32	    pub accepted: bool,
    33	
    34	    /// "adaptation" | "meta_validation" | "heldout"
    35	    pub split: String,
    36	    pub schema_version: String,
    37	    /// SHA-256 of input prompt (retrieval-equivalence audit).
    38	    pub context_hash: String,
    39	    /// Runtime predicate accept = 1, reject = 0.
    40	    pub predicate_result: i32,
    41	    /// Lean post-hoc verify: 1 / 0 / null = not yet checked.
    42	    #[serde(skip_serializing_if = "Option::is_none")]
    43	    pub ground_truth_result: Option<i32>,
    44	    #[serde(skip_serializing_if = "Option::is_none")]
    45	    pub lean_error_category: Option<String>,
    46	    #[serde(skip_serializing_if = "Option::is_none")]
    47	    pub raw_error_hash: Option<String>,
    48	    /// Hash of Q^world snapshot to roll back to (PREREG ArtifactState).
    49	    #[serde(skip_serializing_if = "Option::is_none")]
    50	    pub rollback_to: Option<String>,
    51	
    52	    pub prompt_tokens: u64,
    53	    pub completion_tokens: u64,
    54	    /// Length of all tool stdout summed (B2).
    55	    pub tool_tokens: u64,
    56	    /// = prompt + completion + tool.
    57	    pub total_tokens: u64,
    58	    pub wall_time_ms: u64,
    59	    /// ISO 8601 UTC.
    60	    pub start_time: String,
    61	    pub end_time: String,
    62	    pub ast_depth: u32,
    63	    pub peer_agents_in_branch: Vec<String>,
    64	    /// SHA-256 of concatenated tool stdout.
    65	    #[serde(skip_serializing_if = "Option::is_none")]
    66	    pub tool_stdout_hash: Option<String>,
    67	    pub is_on_golden_path: bool,
    68	    #[serde(skip_serializing_if = "Option::is_none")]
    69	    pub golden_path_id: Option<String>,
    70	    /// Phase D+ meta-loop attribution; nullable in Phase B.
    71	    #[serde(skip_serializing_if = "Option::is_none")]
    72	    pub architect_artifact_id: Option<String>,
    73	    #[serde(skip_serializing_if = "Option::is_none")]
    74	    pub auditor_attestation: Option<String>,
    75	}
    76	
    77	/// Per-run aggregate row.
    78	///
    79	/// `pput_runtime` = legacy / runtime-accept-based — NEVER the North Star.
    80	/// `pput_verified` = Lean post-hoc verified — H-VPPUT input.
    81	#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    82	pub struct RunAggregate {
    83	    pub run_id: String,
    84	    pub problem_id: String,
    85	    pub solved: bool,
    86	
    87	    pub schema_version: String,
    88	    pub split: String,
    89	    /// Lean post-hoc PASS (B4).
    90	    pub verified: bool,
    91	    pub golden_path_token_count: u64,
    92	    /// C_i — sum over all proposals (B2).
    93	    pub total_run_token_count: u64,
    94	    /// T_i — wall-clock first-read → final-accept (B3).
    95	    pub total_wall_time_ms: u64,
    96	    /// 0 or 1 (Lean ground truth).
    97	    pub progress: u8,
    98	    /// Runtime/accept-based; may inflate under Soft Law (H1 detection).
    99	    pub pput_runtime: f64,
   100	    /// Verified PPUT — Progress / (C_i × T_i / 1000), units = 1/(token·second).
   101	    pub pput_verified: f64,
   102	    /// 10^6 × pput_verified — display unit (PREREG § 5).
   103	    pub pput_m_verified: f64,
   104	    pub failed_branch_count: u32,
   105	    pub rollback_count: u32,
   106	
   107	    /// Phase A atom A4: did the run reach `max_transactions` without OMEGA?
   108	    /// True iff the natural max-tx exhaustion path fired. False on OMEGA
   109	    /// accept, on the B7-extra synthetic short-circuit (which exits
   110	    /// EARLY at the rollback threshold — counted under
   111	    /// `synthetic_short_circuit`, not here), and on oneshot (no max-tx
   112	    /// concept; only one LLM call). Co-reported with `solved` so
   113	    /// downstream analysis can split `(solve_rate)` from `(PPUT on solved)`
   114	    /// per Gemini N-agents brainstorm 2026-04-25 § A.4.
   115	    pub hit_max_tx: bool,
   116	    /// Phase A atom A4: distinct-payload-hash / total-proposal ratio
   117	    /// across every parsed `append`/`complete`/`step` payload in the run.
   118	    /// Range [0.0, 1.0]; 1.0 = every proposal unique; 0 proposals → 0.0
   119	    /// by convention (synthetic / measurement-failure runs). Cheap proxy
   120	    /// for the semantic-diversity metric proposed in the N-agents
   121	    /// brainstorms (Gemini § A "Search Party"); embedding distance is
   122	    /// Phase D+ work.
   123	    pub tactic_diversity: f64,
   124	    /// Phase A atom A4: cumulative wall-clock spent inside Lean verifier
   125	    /// calls (`verify_omega` / `verify_omega_detailed` / `verify_partial`)
   126	    /// across the full run, in milliseconds. By construction
   127	    /// `verifier_wait_ms ≤ total_wall_time_ms`. Enables the Amdahl /
   128	    /// USL decomposition Codex N-agents brainstorm § C proposed
   129	    /// (parallel LLM time vs serial Lean time).
   130	    pub verifier_wait_ms: u64,
   131	
   132	    /// Phase A atom A5 (FC2-N22 HALT decomposition): which
   133	    /// budget-regime governed the loop bound for this run. Stable
   134	    /// label string (`total_proposal` | `per_agent` | `token_total` |
   135	    /// `wall_clock`). PREREG_AMENDMENT_p0_defer § 3 condition 3 names
   136	    /// this stamp as a re-calibration prerequisite — without it,
   137	    /// `MaxTxExhausted` rows are ambiguous about which budget
   138	    /// partitioning rule produced them. Oneshot runs (no swarm loop)
   139	    /// stamp `total_proposal` + `budget_max_transactions=1`.
   140	    pub budget_regime: String,
   141	    /// Phase A atom A5: base transaction budget BEFORE the regime's
   142	    /// scaling rule was applied. Under `total_proposal` the loop bound
   143	    /// equals this; under `per_agent` the loop bound = base × n_agents.
   144	    /// Stamping the base (not the effective bound) keeps cross-N
   145	    /// comparisons interpretable in downstream analysis.
   146	    pub budget_max_transactions: u32,
   147	
   148	    pub far: f64,
   149	    pub err: f64,
   150	    pub iac: f64,
   151	    pub cpr: f64,
   152	
   153	    /// Exact model id + API revision (drift defense per F-2026-04-22-08).
   154	    pub model_snapshot: String,
   155	    pub git_sha: String,
   156	    pub binary_sha256: String,
   157	    /// "full" | "panopticon" | "amnesia" | "soft_law" | "homogeneous".
   158	    pub mode: String,
   159	}
   160	
   161	impl RunAggregate {
   162	    /// Compute pput_verified per PREREG § 5:
   163	    ///   pput_verified = progress / (c_i * t_i_ms / 1000)
   164	    /// Returns 0.0 when progress is 0, OR when c_i or t_i_ms is 0
   165	    /// (synthetic / degenerate runs; real runs always have positive cost+time).
   166	    pub fn compute_pput_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
   167	        if progress == 0 || c_i == 0 || t_i_ms == 0 {
   168	            return 0.0;
   169	        }
   170	        let denom = (c_i as f64) * (t_i_ms as f64) / 1000.0;
   171	        (progress as f64) / denom
   172	    }
   173	
   174	    /// Display unit: 10^6 × pput_verified.
   175	    pub fn compute_pput_m_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
   176	        1.0e6 * Self::compute_pput_verified(progress, c_i, t_i_ms)
   177	    }
   178	}
   179	
   180	/// Phase A atom A4 (FC1-N11 ∏p decision diversity): tactic_diversity
   181	/// = distinct / total. 0 proposals → 0.0 by convention (no signal to
   182	/// report). All-distinct → 1.0; all-identical → 1/total.
   183	pub fn compute_tactic_diversity(distinct_proposals: u64, total_proposals: u64) -> f64 {
   184	    if total_proposals == 0 {
   185	        return 0.0;
   186	    }
   187	    let r = (distinct_proposals as f64) / (total_proposals as f64);
   188	    // distinct must not exceed total (caller bug); clamp to [0, 1].
   189	    r.clamp(0.0, 1.0)
   190	}
   191	
   192	/// Legacy v1 run row — mirrors the pre-v2 `PputResult` shape emitted by the
   193	/// evaluator before this commit (Paper 1 era, e.g.
   194	/// `discarded_12way_run_2026-04-24/E1v2_Abl_*.jsonl`).
   195	///
   196	/// All v3-era extension fields (reputation_at_end, halt_reason, gp_*) are
   197	/// captured by `extra` so a legacy line round-trips losslessly through
   198	/// serde_json::Value.
   199	#[derive(Debug, Clone, Deserialize, Serialize)]
   200	pub struct LegacyRunAggregate {
   201	    pub problem: String,
   202	    pub condition: String,
   203	    pub model: String,
   204	    pub has_golden_path: bool,
   205	    pub time_secs: f64,
   206	    pub pput: f64,
   207	    pub gp_token_count: u64,
   208	    pub gp_node_count: usize,
   209	    pub tx_count: u64,
   210	    /// Catch-all for v3.x optional fields (reputation_at_end, halt_reason,

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/tests/trust_root_immutability.rs | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// PPUT-CCL Phase B7 — Trust Root immutability (PREREG § 1.8 + § 7 Gate B).
     2	//
     3	// Boot computes SHA-256 of every Trust Root file at process start and
     4	// compares against the genesis_payload.toml [trust_root] manifest. Any
     5	// mismatch = `TRUST_ROOT_TAMPERED` abort.
     6	//
     7	// Trust Root manifest (PREREG § 1.8 + audit additions through 2026-04-25):
     8	//   src/main.rs                                       (audit-fix Q2.b)
     9	//   src/kernel.rs
    10	//   src/wal.rs
    11	//   src/bus.rs
    12	//   src/drivers/llm_http.rs                           (B2-B4 audit add)
    13	//   src/sdk/prompt_guard.rs                           (B6 add)
    14	//   Cargo.lock                                        (audit-fix Q2.e)
    15	//   experiments/minif2f_v4/src/lean4_oracle.rs
    16	//   experiments/minif2f_v4/src/cost_aggregator.rs     (B2)
    17	//   experiments/minif2f_v4/src/wall_clock.rs          (B3)
    18	//   experiments/minif2f_v4/src/post_hoc_verifier.rs   (B4)
    19	//   experiments/minif2f_v4/src/jsonl_schema.rs        (B1)
    20	//   experiments/minif2f_v4/src/rollback_sim.rs        (B7-extra)
    21	//   experiments/minif2f_v4/src/agent_models.rs        (Phase A atom A3)
    22	//   experiments/minif2f_v4/src/bin/evaluator.rs       (the wiring)
    23	//   constitution.md
    24	//   handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json
    25	//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md
    26	//   handover/preregistration/scripts/run_p0_calibration.sh   (audit-fix Q2)
    27	//   handover/preregistration/scripts/compute_p0.py           (audit-fix Q2)
    28	//   cases/MANIFEST.sha256                             (proxy for cases/*.yaml)
    29	
    30	use std::fs;
    31	use std::path::{Path, PathBuf};
    32	use turingosv4::boot::{parse_trust_root_section, verify_trust_root, TrustRootError};
    33	
    34	fn repo_root() -> PathBuf {
    35	    // CARGO_MANIFEST_DIR for this test crate is experiments/minif2f_v4 — repo
    36	    // root is two levels up.
    37	    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    38	        .join("..")
    39	        .join("..")
    40	        .canonicalize()
    41	        .expect("repo root resolves")
    42	}
    43	
    44	fn read_genesis() -> String {
    45	    fs::read_to_string(repo_root().join("genesis_payload.toml")).expect("genesis exists")
    46	}
    47	
    48	#[test]
    49	fn test_trust_root_immutable_at_boot() {
    50	    // Cold-start with intact files: Boot computes SHA-256s, all match
    51	    // genesis manifest, process continues. No abort.
    52	    verify_trust_root(&repo_root()).expect("intact repo Trust Root verifies");
    53	}
    54	
    55	#[test]
    56	fn test_trust_root_simulated_write_aborts() {
    57	    // Simulated tampering: build a self-contained fake-repo in a tempdir
    58	    // with a single Trust Root entry whose recorded hash does not match
    59	    // the file content; assert verify_trust_root returns Tampered.
    60	    let tmp = make_tempdir("trust_root_tamper");
    61	    let zero_hash = "0".repeat(64);
    62	    let genesis = format!(
    63	        "[pput_accounting_0]\nschema_version = \"1.0\"\n\n[trust_root]\n\"only.txt\" = \"{zero_hash}\"\n"
    64	    );
    65	    fs::write(tmp.join("genesis_payload.toml"), genesis).unwrap();
    66	    fs::write(tmp.join("only.txt"), "tampered content").unwrap();
    67	
    68	    match verify_trust_root(&tmp) {
    69	        Err(TrustRootError::Tampered { path, expected, actual }) => {
    70	            assert!(path.ends_with("only.txt"));
    71	            assert_eq!(expected, zero_hash);
    72	            assert_ne!(actual, expected);
    73	        }
    74	        other => panic!("expected Tampered, got {other:?}"),
    75	    }
    76	}
    77	
    78	#[test]
    79	fn test_trust_root_manifest_includes_b2_b4_files() {
    80	    // Mid-term audit recommendation: B2 (cost_aggregator), B3 (wall_clock),
    81	    // B4 (post_hoc_verifier), B1 (jsonl_schema), evaluator.rs, llm_http.rs
    82	    // MUST be in the Trust Root manifest. B6 added prompt_guard.rs.
    83	    let entries = parse_trust_root_section(&read_genesis()).expect("parse trust_root");
    84	    let keys: Vec<&str> = entries.iter().map(|(k, _)| k.as_str()).collect();
    85	
    86	    let required = [
    87	        // PREREG § 1.8 base
    88	        "src/kernel.rs",
    89	        "src/wal.rs",
    90	        "src/bus.rs",
    91	        "experiments/minif2f_v4/src/lean4_oracle.rs",
    92	        "constitution.md",
    93	        "cases/MANIFEST.sha256",
    94	        "handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json",
    95	        "handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md",
    96	        // Mid-term audit accounting layer
    97	        "src/drivers/llm_http.rs",
    98	        "experiments/minif2f_v4/src/cost_aggregator.rs",
    99	        "experiments/minif2f_v4/src/wall_clock.rs",
   100	        "experiments/minif2f_v4/src/post_hoc_verifier.rs",
   101	        "experiments/minif2f_v4/src/jsonl_schema.rs",
   102	        "experiments/minif2f_v4/src/bin/evaluator.rs",
   103	        // B6 add
   104	        "src/sdk/prompt_guard.rs",
   105	        // B7-extra add
   106	        "experiments/minif2f_v4/src/rollback_sim.rs",
   107	        // Phase A atom A3: per-agent AGENT_MODELS env var resolver
   108	        "experiments/minif2f_v4/src/agent_models.rs",
   109	        // Phase A atom A5: budget regime + MAX_TRANSACTIONS resolver
   110	        "experiments/minif2f_v4/src/budget_regime.rs",
   111	        // Phase A atom A6: FC-trace structured-event meta-witness
   112	        "experiments/minif2f_v4/src/fc_trace.rs",
   113	        // Phase A atom A7: heterogeneous-LLM provider plumbing (proxy + smoke)
   114	        "src/drivers/llm_proxy.py",
   115	        "scripts/smoke_siliconflow.sh",
   116	        "scripts/_smoke_siliconflow.py",
   117	        // 2026-04-25 dual-audit fixes
   118	        "src/main.rs",
   119	        "Cargo.lock",
   120	        "handover/preregistration/scripts/run_p0_calibration.sh",
   121	        "handover/preregistration/scripts/compute_p0.py",
   122	        // 2026-04-25 Phase A0 harness modernization
   123	        "rules/MANIFEST.sha256",
   124	        "rules/engine.py",
   125	        ".claude/hooks/judge.sh",
   126	        "tests/fc_alignment_conformance.rs",
   127	        // 2026-04-25 Phase A1 PREREG amendment
   128	        "handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md",
   129	    ];
   130	
   131	    for path in required {
   132	        assert!(
   133	            keys.contains(&path),
   134	            "Trust Root manifest missing required path: {path}\nactual keys: {keys:#?}"
   135	        );
   136	    }
   137	}
   138	
   139	#[test]
   140	fn test_pput_accounting_0_section_present() {
   141	    // genesis_payload.toml must contain [pput_accounting_0] with the PREREG
   142	    // § 1.8 keys.
   143	    let genesis = read_genesis();
   144	    let body = extract_section(&genesis, "pput_accounting_0").expect("section present");
   145	    let body = body.as_str();
   146	
   147	    let required_keys = [
   148	        "schema_version",
   149	        "progress_definition",
   150	        "cost_definition",
   151	        "time_definition",
   152	        "verified_predicate",
   153	        "heldout_sealed_hash",
   154	        "source_pool_sha256",
   155	        "baseline_regression_rate",
   156	        "baseline_regression_jsonl_sha256",
   157	        "k_max",
   158	        "n_max",
   159	    ];
   160	    for key in required_keys {
   161	        let needle = format!("{key} =");
   162	        assert!(
   163	            body.contains(&needle),
   164	            "[pput_accounting_0] missing key: {key}"
   165	        );
   166	    }
   167	
   168	    // Frozen invariants from PREREG § 1.8: heldout sealed hash, k_max, n_max.
   169	    assert!(body.contains(
   170	        "\"51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b\""
   171	    ), "heldout_sealed_hash diverges from PREREG § 2.3");
   172	    assert!(body.contains("k_max = 10"), "k_max must be 10 per PREREG");
   173	    assert!(body.contains("n_max = 34"), "n_max must be 34 per PREREG");
   174	}
   175	
   176	// --- helpers ---
   177	
   178	fn extract_section(text: &str, name: &str) -> Option<String> {
   179	    // Line-anchored scan: skip commented-out section headers (e.g. inside
   180	    // the file's leading docstring) and only match real headers in column 0.

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/agent_models.rs | sed -n '1,190p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// Phase A atom A3 — per-agent model assignment (`AGENT_MODELS` env var).
     2	//
     3	// Constitutional anchor: FC1-N7 (δ/AI canonical identity). Each Agent_i in
     4	// the swarm path embodies one δ instance; today every Agent_i shares a
     5	// single global δ pinned by `ACTIVE_MODEL` env var. Phase D introduces a
     6	// heterogeneous swarm where Agent_i may bind a different δ. This module
     7	// is the env-var → per-agent δ resolver.
     8	//
     9	// **Phase B+C invariant** (notepad F-2026-04-25-02 + memory
    10	// `feedback_phased_checkpoint`): Phases B and C MUST stay single-model so
    11	// the ablation axes (Soft Law / Panopticon / Amnesia / Homogeneous /
    12	// Full) are not confounded by model identity. Heterogeneous assignment
    13	// is therefore *gated* by `PHASE_D_HETERO_OK=1` — until the gate is set
    14	// the resolver rejects any non-uniform `AGENT_MODELS` payload at startup,
    15	// before a single LLM call goes out.
    16	//
    17	// Default behavior (env var unset OR empty): broadcast the global model
    18	// (resolved from `ACTIVE_MODEL`) to every agent slot — preserves Phase B
    19	// behavior bit-for-bit.
    20	
    21	use std::collections::BTreeSet;
    22	use std::fmt;
    23	
    24	/// TRACE_MATRIX FC1-N7: env var name binding the per-Agent_i δ vector.
    25	pub const AGENT_MODELS_ENV_VAR: &str = "AGENT_MODELS";
    26	
    27	/// TRACE_MATRIX FC1-N7: Phase D heterogeneity gate. Required for any
    28	/// AGENT_MODELS payload containing ≥2 distinct δ values. Phase B+C
    29	/// invariant: single δ across all Agent_i.
    30	pub const PHASE_D_HETERO_GATE_ENV_VAR: &str = "PHASE_D_HETERO_OK";
    31	
    32	/// TRACE_MATRIX FC1-N7: startup-fatal failure modes when the per-agent
    33	/// δ vector cannot be safely resolved. Each variant aborts the run
    34	/// before the first LLM call, preserving budget under misconfiguration.
    35	#[derive(Debug, PartialEq, Eq)]
    36	pub enum AgentModelsError {
    37	    /// `AGENT_MODELS` parsed to N entries but the swarm has M ≠ N agents.
    38	    /// (Length 1 broadcasts; only N>1 mismatches reach this branch.)
    39	    LengthMismatch { provided: usize, expected: usize },
    40	    /// A CSV slot was empty after trim (e.g., `"a,,b"` or `",a"`).
    41	    EmptyEntry { index: usize },
    42	    /// Two or more distinct models were supplied without
    43	    /// `PHASE_D_HETERO_OK=1`. Phase B+C single-model invariant.
    44	    HeterogeneousWithoutGate { distinct: Vec<String> },
    45	}
    46	
    47	impl fmt::Display for AgentModelsError {
    48	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    49	        match self {
    50	            Self::LengthMismatch { provided, expected } => write!(
    51	                f,
    52	                "AGENT_MODELS length mismatch: {} provided, {} agents in swarm \
    53	                 (use length 1 to broadcast or length == n_agents for positional)",
    54	                provided, expected
    55	            ),
    56	            Self::EmptyEntry { index } => write!(
    57	                f,
    58	                "AGENT_MODELS entry at index {} is empty (CSV slot blank after trim)",
    59	                index
    60	            ),
    61	            Self::HeterogeneousWithoutGate { distinct } => write!(
    62	                f,
    63	                "AGENT_MODELS contains {} distinct models {:?} but \
    64	                 PHASE_D_HETERO_OK is not set to '1'. Phase B+C ablations \
    65	                 require single-model invariant (notepad F-2026-04-25-02).",
    66	                distinct.len(),
    67	                distinct
    68	            ),
    69	        }
    70	    }
    71	}
    72	
    73	impl std::error::Error for AgentModelsError {}
    74	
    75	/// TRACE_MATRIX FC1-N7: pure CSV parser for the `AGENT_MODELS` payload.
    76	/// Empty input (env unset or empty string) → empty Vec (caller falls
    77	/// back to broadcasting the global model). No env access — testable
    78	/// without process-global state.
    79	pub fn parse_agent_models(env_str: &str) -> Result<Vec<String>, AgentModelsError> {
    80	    let trimmed = env_str.trim();
    81	    if trimmed.is_empty() {
    82	        return Ok(Vec::new());
    83	    }
    84	    let entries: Vec<String> = trimmed.split(',').map(|s| s.trim().to_string()).collect();
    85	    for (i, e) in entries.iter().enumerate() {
    86	        if e.is_empty() {
    87	            return Err(AgentModelsError::EmptyEntry { index: i });
    88	        }
    89	    }
    90	    Ok(entries)
    91	}
    92	
    93	/// TRACE_MATRIX FC1-N7: validator + expander. Maps parsed CSV entries
    94	/// to a per-Agent_i δ vector of length `n_agents`. Pure (no env access).
    95	///
    96	/// - parsed empty → broadcast `global_model` to every agent.
    97	/// - parsed.len() == 1 → broadcast that single model.
    98	/// - parsed.len() == n_agents → positional assignment.
    99	/// - else → `LengthMismatch`.
   100	///
   101	/// Heterogeneity (≥2 distinct models in the resolved vector) requires
   102	/// `hetero_gated == true`; otherwise `HeterogeneousWithoutGate`.
   103	pub fn expand_agent_models(
   104	    parsed: Vec<String>,
   105	    global_model: &str,
   106	    n_agents: usize,
   107	    hetero_gated: bool,
   108	) -> Result<Vec<String>, AgentModelsError> {
   109	    let resolved = if parsed.is_empty() {
   110	        vec![global_model.to_string(); n_agents]
   111	    } else if parsed.len() == 1 {
   112	        vec![parsed.into_iter().next().unwrap(); n_agents]
   113	    } else if parsed.len() == n_agents {
   114	        parsed
   115	    } else {
   116	        return Err(AgentModelsError::LengthMismatch {
   117	            provided: parsed.len(),
   118	            expected: n_agents,
   119	        });
   120	    };
   121	
   122	    let distinct: BTreeSet<&str> = resolved.iter().map(String::as_str).collect();
   123	    if distinct.len() > 1 && !hetero_gated {
   124	        return Err(AgentModelsError::HeterogeneousWithoutGate {
   125	            distinct: distinct.into_iter().map(String::from).collect(),
   126	        });
   127	    }
   128	    Ok(resolved)
   129	}
   130	
   131	/// TRACE_MATRIX FC1-N7: env-coupled wrapper used by `run_swarm` to
   132	/// produce the per-Agent_i δ vector. Composes `parse_agent_models` +
   133	/// `expand_agent_models`; reads `AGENT_MODELS` and the Phase D
   134	/// heterogeneity gate from process env.
   135	pub fn resolve_agent_models(
   136	    global_model: &str,
   137	    n_agents: usize,
   138	) -> Result<Vec<String>, AgentModelsError> {
   139	    let raw = std::env::var(AGENT_MODELS_ENV_VAR).unwrap_or_default();
   140	    let hetero_gated =
   141	        std::env::var(PHASE_D_HETERO_GATE_ENV_VAR).as_deref() == Ok("1");
   142	    let parsed = parse_agent_models(&raw)?;
   143	    expand_agent_models(parsed, global_model, n_agents, hetero_gated)
   144	}
   145	
   146	#[cfg(test)]
   147	mod tests {
   148	    use super::*;
   149	
   150	    #[test]
   151	    fn empty_env_parses_to_empty_vec() {
   152	        assert_eq!(parse_agent_models("").unwrap(), Vec::<String>::new());
   153	        assert_eq!(parse_agent_models("   ").unwrap(), Vec::<String>::new());
   154	    }
   155	
   156	    #[test]
   157	    fn single_entry_parses() {
   158	        assert_eq!(
   159	            parse_agent_models("deepseek-v4-flash").unwrap(),
   160	            vec!["deepseek-v4-flash".to_string()]
   161	        );
   162	    }
   163	
   164	    #[test]
   165	    fn csv_entries_trimmed() {
   166	        assert_eq!(
   167	            parse_agent_models("a, b ,c").unwrap(),
   168	            vec!["a".to_string(), "b".to_string(), "c".to_string()]
   169	        );
   170	    }
   171	
   172	    #[test]
   173	    fn empty_csv_slot_rejected() {
   174	        assert_eq!(
   175	            parse_agent_models("a,,b"),
   176	            Err(AgentModelsError::EmptyEntry { index: 1 })
   177	        );
   178	        assert_eq!(
   179	            parse_agent_models(",a"),
   180	            Err(AgentModelsError::EmptyEntry { index: 0 })
   181	        );
   182	        assert_eq!(
   183	            parse_agent_models("a,"),
   184	            Err(AgentModelsError::EmptyEntry { index: 1 })
   185	        );
   186	    }
   187	
   188	    #[test]
   189	    fn empty_parsed_broadcasts_global_model() {
   190	        let v = expand_agent_models(vec![], "deepseek-v4-flash", 3, false).unwrap();

exec
/bin/bash -lc "nl -ba scripts/smoke_siliconflow.sh | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	#!/usr/bin/env bash
     2	# Phase A atom A7 — SiliconFlow integration smoke.
     3	#
     4	# Probes each of the 3 SiliconFlow keys (primary / secondary / tertiary)
     5	# with a minimal chat call against a cheap model. Reports per-key OK or
     6	# FAIL without printing key material. Exits non-zero if any key fails.
     7	#
     8	# Why direct SDK probes (not via the proxy round-robin): we want a
     9	# per-key verdict — the proxy's round-robin would obscure which specific
    10	# key failed if one is rate-limited / revoked. After A7 PASSes, the
    11	# evaluator's runtime path goes through llm_proxy.py.
    12	#
    13	# Cost: 3 calls × ~50 tokens each = ¥0.001 - ¥0.005 total. SiliconFlow
    14	# Qwen2.5-7B-Instruct free tier covers this; backstop is the user's
    15	# key budget. Aborts after the first key fails to bound spend.
    16	#
    17	# Usage:
    18	#   bash scripts/smoke_siliconflow.sh
    19	#
    20	# Reads keys from .env (auto-loaded) or current shell env.
    21	
    22	set -euo pipefail
    23	
    24	cd "$(dirname "$0")/.."
    25	ROOT="$(pwd)"
    26	
    27	# Source .env if present; do not echo any value.
    28	if [ -f .env ]; then
    29	    set -a
    30	    # shellcheck disable=SC1091
    31	    . .env
    32	    set +a
    33	fi
    34	
    35	# Sanity: at least the primary key must be set.
    36	: "${SILICONFLOW_API_KEY:?SILICONFLOW_API_KEY not set; configure .env first}"
    37	
    38	python3 "$ROOT/scripts/_smoke_siliconflow.py"

exec
/bin/bash -lc "nl -ba scripts/_smoke_siliconflow.py | sed -n '1,160p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	#!/usr/bin/env python3
     2	"""Phase A atom A7 — per-key SiliconFlow probe.
     3	
     4	Invoked by `scripts/smoke_siliconflow.sh`. Reads the three keys from
     5	env (`SILICONFLOW_API_KEY` / `_SECONDARY` / `_TERTIARY`), issues one
     6	tiny chat-completion call per key, and reports OK/FAIL per key WITHOUT
     7	printing any key material. Exits non-zero if any configured key fails.
     8	
     9	Cost bound: 3 calls × ~50 tokens. Qwen2.5-7B-Instruct on SiliconFlow
    10	free tier is the cheapest stable option (V3L-27 N=30 collapse caveat
    11	applies only at high concurrency; one call per key is safe).
    12	"""
    13	import os
    14	import sys
    15	import time
    16	
    17	try:
    18	    from openai import OpenAI, APIStatusError, RateLimitError
    19	except ImportError:
    20	    print("[A7-smoke] FAIL: openai SDK not installed (pip install openai)")
    21	    sys.exit(2)
    22	
    23	KEY_ENVS = [
    24	    ("primary", "SILICONFLOW_API_KEY"),
    25	    ("secondary", "SILICONFLOW_API_KEY_SECONDARY"),
    26	    ("tertiary", "SILICONFLOW_API_KEY_TERTIARY"),
    27	]
    28	BASE_URL = "https://api.siliconflow.cn/v1"
    29	# Qwen2.5-7B-Instruct: smallest stable production model on SF free tier.
    30	# Avoids expensive reasoning models during probe.
    31	PROBE_MODEL = "Qwen/Qwen2.5-7B-Instruct"
    32	PROBE_PROMPT = "Reply with the single word: ack"
    33	PROBE_MAX_TOKENS = 8
    34	
    35	
    36	def probe_one(label: str, env_name: str, key: str) -> tuple[bool, str]:
    37	    """Return (ok, summary). Never returns the key in `summary`."""
    38	    client = OpenAI(api_key=key, base_url=BASE_URL)
    39	    t0 = time.time()
    40	    try:
    41	        resp = client.chat.completions.create(
    42	            model=PROBE_MODEL,
    43	            messages=[{"role": "user", "content": PROBE_PROMPT}],
    44	            temperature=0.0,
    45	            max_tokens=PROBE_MAX_TOKENS,
    46	            stream=False,
    47	        )
    48	    except RateLimitError as e:
    49	        return False, f"RateLimitError ({type(e).__name__}): {str(e)[:120]}"
    50	    except APIStatusError as e:
    51	        return False, f"APIStatusError {getattr(e, 'status_code', '?')}: {str(e)[:120]}"
    52	    except Exception as e:
    53	        return False, f"Error {type(e).__name__}: {str(e)[:120]}"
    54	    dt_ms = int((time.time() - t0) * 1000)
    55	    msg = resp.choices[0].message
    56	    content = (msg.content or "").strip()
    57	    usage = resp.usage
    58	    pt = getattr(usage, "prompt_tokens", "?") if usage else "?"
    59	    ct = getattr(usage, "completion_tokens", "?") if usage else "?"
    60	    return True, (
    61	        f"{dt_ms}ms; tokens prompt={pt} completion={ct}; "
    62	        f"content[:32]={content[:32]!r}"
    63	    )
    64	
    65	
    66	def main() -> int:
    67	    print(
    68	        f"[A7-smoke] SiliconFlow probe — model={PROBE_MODEL} "
    69	        f"max_tokens={PROBE_MAX_TOKENS}"
    70	    )
    71	    any_failed = False
    72	    any_present = False
    73	    for label, env_name in KEY_ENVS:
    74	        key = os.environ.get(env_name, "").strip()
    75	        if not key:
    76	            print(f"  [{label:9s}] {env_name}: NOT SET — skipping")
    77	            continue
    78	        any_present = True
    79	        ok, summary = probe_one(label, env_name, key)
    80	        verdict = "OK  " if ok else "FAIL"
    81	        print(f"  [{label:9s}] {env_name}: {verdict} {summary}")
    82	        if not ok:
    83	            any_failed = True
    84	    if not any_present:
    85	        print("[A7-smoke] FAIL: no SiliconFlow keys configured")
    86	        return 2
    87	    if any_failed:
    88	        print("[A7-smoke] result: FAIL (one or more keys failed)")
    89	        return 1
    90	    print("[A7-smoke] result: PASS (all configured keys responded)")
    91	    return 0
    92	
    93	
    94	if __name__ == "__main__":
    95	    sys.exit(main())

exec
/bin/bash -lc "nl -ba handover/alignment/TRACE_MATRIX_v2_2026-04-25.md | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TRACE_MATRIX v2 — Constitutional Flowchart ↔ Rust Code (2026-04-25 post-A0)
     2	
     3	**Predecessor**: `TRACE_MATRIX_v1_2026-04-25.md`
     4	**Trigger**: Phase A0 (harness modernization) shipped:
     5	- A0a: 4 new rules (R-014/R-015/R-018/R-019) + judge.sh constitution-special-case + R-016 fc_trace_in_commit hook (commit 2e7f75a)
     6	- A0b: tests/fc_alignment_conformance.rs witness battery — 17 PASS + 9 ignored stubs (commit d8950ee)
     7	- A0c: 5 new cases C-071..C-075 sediment session decisions (commit 2a65339)
     8	- A0d (this doc): Trust Root manifest 20 → 24 (this commit); v2 documents the harness as constitutional artifact
     9	- A4 (post-A3): decomposed metrics — `hit_max_tx`, `tactic_diversity`, `verifier_wait_ms` added as non-Optional v2 fields + `compute_tactic_diversity` helper; per-row decomposition of `solve_rate` / `tokens_per_solve` / `time_per_solve` (all derivable from existing `progress` / `total_run_token_count` / `total_wall_time_ms`). FC-trace: FC2-N22 (HALT decomposition for `hit_max_tx`) + FC1-N11 (∏p decision diversity for `tactic_diversity`) + FC1-N12 (oracle scope for `verifier_wait_ms`).
    10	- A5 (post-A4): per-agent budget normalization — new `budget_regime` module (`BUDGET_REGIME` + `MAX_TRANSACTIONS` env vars; 4-variant enum; pure parser + scaler + env-coupled resolver); `budget_regime` + `budget_max_transactions` added as non-Optional v2 fields on `RunAggregate` and the legacy `PputResult`; loop bound at `run_swarm` switched from hardcoded `let max_transactions = 200` to `resolve_budget(n_agents)` — default (env unset) preserves Phase B baseline (`total_proposal × 200`) bit-for-bit. PREREG_AMENDMENT_p0_defer § 3 condition 3 satisfied: `MaxTxExhausted` rows now disambiguated across N values. FC-trace: FC2-N22 (HALT decomposition by budget regime) + FC1-N7 (δ instances determining the per-agent share under PerAgent regime). Trust Root manifest 25 → 26.
    11	- A6 (post-A5): per-line FC tagging via structured JSON events — new `fc_trace` module (pure stdlib; zero new deps); `FcId` enum (FC1-N7 / FC1-N11 / FC1-N12 / FC1-E18 / FC2-N20 / FC2-N22 / FC3-N31); `fc_event!`-style `emit_event` API; `FC_TRACE=1` gate (cached in `OnceLock`); `FC_TRACE_FILE=<path>` redirects emit to file (default sink stderr). Six anchor sites wired in `run_swarm`: FC2-N22 synthetic short-circuit, FC2-N20 mr tick, FC2-N22 OMEGA full-proof, FC2-N22 OMEGA per-tactic, FC2-N22 natural MaxTxExhausted (with `budget_regime` payload), FC1-N12 verify bracket (oneshot). End-to-end smoke test exercises FC_TRACE=1 in a child process (subprocess required because `OnceLock` caches the gate-read; resolves item 7 of TRACE_MATRIX § 5 "Per-line FC tagging via tracing crate"). FC-trace: meta-witness for the 5-step compile loop (Proposal → Lean ground truth → Logging → Capability compilation → ↑H-VPPUT). Trust Root manifest 26 → 27.
    12	- A7 (post-A6): heterogeneous-LLM provider plumbing — `src/drivers/llm_proxy.py` ported from v3 with one load-bearing v4 change (per-provider multi-key round-robin: 3 SiliconFlow keys split concurrent traffic across separate rate-limit pools, mitigating V3L-27 single-key N=30 collapse). `scripts/smoke_siliconflow.sh` + `scripts/_smoke_siliconflow.py` probe each of the 3 keys (Qwen/Qwen2.5-7B-Instruct, max_tokens=8) — A7 verified all 3 keys responding 2026-04-26 (1.5–3s latency, 33+1 tokens; round-robin distributes [2,2,2] across 6 calls). New `siliconflow:<model>` provider-prefix syntax in `detect_provider()` for unambiguous routing in `AGENT_MODELS` payloads (Phase D heterogeneous swarms). Memory `reference_siliconflow.md` records SiliconFlow as the heterogeneous-LLM lane (NOT a fallback target). FC-trace: FC1-N7 (δ/AI provider expansion — heterogeneous δ instances across SF catalog enable Phase D meta-loop). Trust Root manifest 27 → 30 (proxy + 2 smoke scripts).
    13	
    14	**Scope**: delta from v1. Read v0 + v1 first.
    15	
    16	---
    17	
    18	## § 1. Status flips: 17 ⚠️ → ✅ via fc_alignment_conformance.rs witnesses
    19	
    20	A0b added the missing `tests/fc_alignment_conformance.rs` (was only in `.claude/worktrees/phase-8a-snapshot/`). 17 ✅ rows in TRACE_MATRIX now have automated witness tests. Symbol drift is now caught at `cargo test` time, not at next dual audit.
    21	
    22	| FC ID | v1 Status | v2 Status | Witness test |
    23	|---|---|---|---|
    24	| FC1-N1 (Q_t carrier) | ⚠️ | ✅ | `fc1_n1_q_state_carrier_present` |
    25	| FC1-N4 (tape) | ⚠️ | ✅ | `fc1_n4_tape_constructible_with_time_arrow` |
    26	| FC1-N6 (input UniverseSnapshot) | ✅ | ✅ + witness | `fc1_n6_input_universe_snapshot_present` |
    27	| FC1-N7 (δ/AI ResilientLLMClient) | ✅ | ✅ + witness | `fc1_n7_delta_ai_client_type` |
    28	| FC1-N8/N9/N10 (output / q_o / a_o) | ✅ | ✅ + witness | `fc1_n8_n9_n10_output_agent_output_parseable` |
    29	| FC1-N11 (∏p production-path forbidden_pattern) | ⚠️ | ✅ | `fc1_n11_n15_e18_pi_p_zero_preserves_q_t_via_forbidden_pattern` |
    30	| FC1-N13 (wtool bus.append) | ⚠️ | ✅ | `fc1_n13_wtool_bus_append_present` |
    31	| FC1-N15 / E18 (∏p=0 → Q_t preserve) | ⚠️ | ✅ | `fc1_n11_n15_e18_*` (same test) |
    32	| FC2-N20/N27 (mr tick) | ✅ | ✅ + witness | `fc2_n20_n27_tick_mr_present` |
    33	| FC2-N22 (HALT) | ⚠️ | ✅ | `fc2_n22_halt_via_halt_and_settle` |
    34	| FC2-N23 (HaltReason — only OmegaAccepted typed) | ✅ | ✅ + witness | `fc2_n23_event_type_omega_accepted_canonical` |
    35	| FC3-N31 (Wal logs archive) | ⚠️ | ✅ | `fc3_n31_logs_archive_wal_present` |
    36	| FC3-N34 (readonly guard verify_trust_root) | ✅ | ✅ + 3 witnesses | `fc3_n34_*` (3 tests) |
    37	| FC3-N39 (Ledger log) | ✅ | ✅ + witness | `fc3_n39_log_ledger_present_and_appendable` |
    38	| FC3-S3 (readonly subgraph manifest) | (new in v1) | ✅ | `fc3_s3_readonly_subgraph_manifest_size` (>=20 entries assertion) |
    39	| FC3-E14 (boot panic immediate-abort) | (new in v1) | ✅ | `fc3_e14_boot_panic_immediate_abort_documented` |
    40	| (Veto-AI Art. V.1.3 amendment) | (cases C-072) | ✅ via case-law | C-072 yaml |
    41	
    42	## § 2. New code symbols (Phase A0–A3)
    43	
    44	| Symbol | File | FC anchor | Status |
    45	|---|---|---|---|
    46	| `tests/fc_alignment_conformance.rs` (17 witness fns + 9 ignored stubs) | `tests/fc_alignment_conformance.rs` | meta-witness for FC1/FC2/FC3 ↔ symbol mapping; CLAUDE.md "Conformance tests" requirement | ✅ |
    47	| `rules/active/R-014_trust_root_manifest_drift.yaml` | `rules/active/R-014*.yaml` | FC3-S3 readonly subgraph runtime reminder | ✅ |
    48	| `rules/active/R-015_trace_matrix_pub_symbol.yaml` | `rules/active/R-015*.yaml` | CLAUDE.md "每个 src/ pub 符号必须映射到宪法 flowchart 元素" | ✅ |
    49	| `rules/active/R-018_constitution_amendment_sudo.yaml` | `rules/active/R-018*.yaml` | Art. V.1.1 amendment 2026-04-25 (sudo only for constitution.md) | ✅ |
    50	| `rules/active/R-019_model_snapshot_canonical.yaml` | `rules/active/R-019*.yaml` | FC1-N7 δ/AI canonical identity | ✅ |
    51	| `judge.sh` constitution.md special case | `.claude/hooks/judge.sh:50-67` | FC3-N3 sudo-gate enforcement (closes silent-bypass via `*.md` skip-list) | ✅ |
    52	| `judge.sh` R-016 fc_trace_in_commit | `.claude/hooks/judge.sh:48-56` | FC-first rule (memory feedback_fc_first_problem_handling + case C-074) | ✅ |
    53	| `parse_swarm_condition_n` (A2) | `experiments/minif2f_v4/src/bin/evaluator.rs` | FC2-N16 InitAI orchestration entry — discriminates `oneshot` vs `n<N>` swarm code paths; FC1-N11 ∏p reached only via swarm | ✅ |
    54	| `agent_models::{AGENT_MODELS_ENV_VAR, PHASE_D_HETERO_GATE_ENV_VAR, AgentModelsError, parse_agent_models, expand_agent_models, resolve_agent_models}` (A3) | `experiments/minif2f_v4/src/agent_models.rs` | FC1-N7 δ/AI per-agent assignment; gates Phase B+C single-model invariant (notepad F-2026-04-25-02) | ✅ |
    55	| `RunAggregate::{hit_max_tx, tactic_diversity, verifier_wait_ms}` + `compute_tactic_diversity` (A4) | `experiments/minif2f_v4/src/jsonl_schema.rs` | FC2-N22 HALT decomposition (hit_max_tx splits natural max-tx exhaustion from OMEGA accept and from B7-extra synthetic short-circuit); FC1-N11 ∏p decision diversity (tactic_diversity = distinct/total over append+complete+step proposals); FC1-N12 oracle scope (verifier_wait_ms = cumulative Lean wall-clock per run, ≤ total_wall_time_ms by construction) | ✅ |
    56	| `make_pput` A4 args + per-call-site verifier brackets + per-tool proposal hashing (A4) | `experiments/minif2f_v4/src/bin/evaluator.rs` | wires the 3 fields at every emit site (oneshot + swarm OMEGA + swarm step Complete + swarm synthetic short-circuit + swarm natural max-tx exhaustion); 5 unit/conformance tests (`test_a4_decomposed_metrics_round_trip`, `test_a4_tactic_diversity_helper`, `test_a4_verifier_wait_bounded_by_total_wall_time`, `test_a4_emit_max_tx_exhaustion_row`, `test_a4_synthetic_short_circuit_does_not_set_hit_max_tx`) | ✅ |
    57	| `budget_regime::{BUDGET_REGIME_ENV_VAR, MAX_TRANSACTIONS_ENV_VAR, DEFAULT_MAX_TRANSACTIONS, BudgetRegime, BudgetError, parse_budget_regime, parse_max_transactions, effective_max_tx, resolve_budget}` (A5) | `experiments/minif2f_v4/src/budget_regime.rs` | FC2-N22 HALT decomposition by budget regime — declares which partitioning rule (`total_proposal` / `per_agent` / `token_total` / `wall_clock`) governed the loop bound. Phase A scope = first two regimes implemented; latter two declared startup-fatal `UnimplementedRegime` so a misconfigured run aborts before consuming LLM budget. PREREG_AMENDMENT_p0_defer § 3 condition 3 dependency cleared. | ✅ |
    58	| `RunAggregate::{budget_regime, budget_max_transactions}` + `PputResult::{budget_regime, budget_max_transactions}` (A5) | `experiments/minif2f_v4/src/jsonl_schema.rs` + `experiments/minif2f_v4/src/bin/evaluator.rs` | FC2-N22: every emitted v2 row stamps the regime label + base budget so downstream PPUT analysis can join on the partitioning rule. Loop bound at `run_swarm` startup = `resolve_budget(n_agents).effective_max_tx`; default (env unset) preserves the Phase B baseline `total_proposal × 200` bit-for-bit. 16 unit tests (15 in `budget_regime::tests` + 1 `test_a5_budget_regime_round_trip` in jsonl_schema). | ✅ |
    59	| `fc_trace::{FcId, FC_TRACE_*ENV*, fc_trace_enabled, emit_event, json_str}` (A6) | `experiments/minif2f_v4/src/fc_trace.rs` | meta-witness for FC1 / FC2 / FC3 path coverage. 7-variant `FcId` enum produces stable strings (`FC1-N7` / `FC1-N11` / `FC1-N12` / `FC1-E18` / `FC2-N20` / `FC2-N22` / `FC3-N31`) that Phase D consumers + TRACE_MATRIX rows join on. `FC_TRACE=1` gate cached in `OnceLock` (zero-overhead in production). 6 unit tests (label stability + JSON escape + cold-path no-op). | ✅ |
    60	| `run_corr_id` correlation key + 6 wired FC events (A6) | `experiments/minif2f_v4/src/bin/evaluator.rs` | per-run correlation id (`condition + problem_id + unix-ms`) anchors all events from one run. Anchor sites: FC2-N22 synthetic short-circuit / mr tick FC2-N20 / OMEGA full-proof FC2-N22 / OMEGA per-tactic FC2-N22 / natural MaxTxExhausted FC2-N22 (with `budget_regime` payload from A5) / FC1-N12 verify bracket in oneshot. End-to-end smoke `tests/fc_trace_smoke.rs` exercises FC_TRACE=1 in a child process (forced because `OnceLock` caches the gate-read). | ✅ |
    61	| `llm_proxy.py` v4 (multi-key round-robin) + `detect_provider` `siliconflow:` prefix (A7) | `src/drivers/llm_proxy.py` | FC1-N7 δ/AI provider expansion — three SiliconFlow keys form a 3-element round-robin pool keyed on `_per_key_requests[provider]`. Phase D heterogeneous swarms can address SF models via `AGENT_MODELS=siliconflow:Qwen/Qwen2.5-7B-Instruct,...`. Mitigates V3L-27 (case C-027) single-key N=30 401/429 collapse documented in `cases/V3_LESSONS.md`. | ✅ |
    62	| `smoke_siliconflow.sh` + `_smoke_siliconflow.py` (A7) | `scripts/smoke_siliconflow.sh` + `scripts/_smoke_siliconflow.py` | A7 acceptance gate — 3 keys × 1 probe each (Qwen2.5-7B-Instruct, max_tokens=8). Verified all 3 SiliconFlow keys responding 2026-04-26 + proxy round-robin distributes [2,2,2] across 6 calls. PASS gates Phase D heterogeneous-swarm work. | ✅ |
    63	
    64	## § 3. Trust Root manifest expansion: 20 → 24
    65	
    66	Per case **C-075 (DO-178C tool qualification)**: governance instrumentation is itself constitutional; tampering with rules / judge.sh / conformance tests = silent constitutional drift.
    67	
    68	| New entry | Why in Trust Root |
    69	|---|---|
    70	| `rules/MANIFEST.sha256` (proxy for 14 rules/active/R-*.yaml) | Same pattern as cases/MANIFEST.sha256: glob hashed once, manifest tracked in Trust Root. Tampering with R-018 enforcement = "warn" silently bypasses constitution sudo gate. |
    71	| `rules/engine.py` | The interpreter of the rules. Tampering with engine.py = silent rule bypass even with intact rule files. |
    72	| `.claude/hooks/judge.sh` | The PreToolUse hook that invokes engine.py + implements R-016 fc_trace + constitution.md special-case. Tampering = bypass entire gate stack. |
    73	| `tests/fc_alignment_conformance.rs` | Witness battery for TRACE_MATRIX ✅ rows. Tampering = false PASS hides drift. |
    74	
    75	**Total: 24 entries** (15 from B7 + 1 B7-extra rollback_sim + 4 dual-audit fixes + 4 A0 harness). A1 (PREREG amendment) → 25; A5 (budget_regime.rs) → 26; A6 (fc_trace.rs) → 27; A7 (llm_proxy.py + smoke_siliconflow.sh + _smoke_siliconflow.py) → 30. When B7-extra calibration eventually runs, the calibration jsonl makes 31 entries; future Phase C's `--mode` flag binary (TBD location) makes 32.
    76	
    77	## § 4. New constitutional case-law (A0c)
    78	
    79	5 new cases C-071..C-075 (commit 2a65339) sediment 2026-04-25 session decisions as constitutional precedent. Each cross-referenced in TRACE_MATRIX rows:
    80	
    81	| Case | Anchors | Rules / hooks enforcing |
    82	|---|---|---|
    83	| C-071 constitution amendment process | Art. V.1.1 + V.3 | R-018 (BLOCK) + judge.sh special-case |
    84	| C-072 Veto-AI scope narrowing | Art. V.1.3 | manual via dual audit; future FC3-N32 runtime |
    85	| C-073 ArchitectAI commit authority | Art. V.1.2 | implicit via 19-commit session validation |
    86	| C-074 FC-first problem handling | All FC + Alignment Standard | R-016 (WARN on git commit without FC-trace) |
    87	| C-075 DO-178C tool qualification | PREREG § 1.8 + Art. V.1.1 | R-014 (warn on .rs edit) + 24-file manifest expansion |
    88	
    89	## § 5. Open work flagged for future TRACE_MATRIX_v3
    90	
    91	1. **TRACE_MATRIX_v?.md docs themselves** — currently NOT in Trust Root (would cause self-reference loop). Acceptable since these are documentation, not enforcement. Phase D (when ArchitectAI runtime comes online) may need to formalize doc-Trust-Root semantics.
    92	2. **rules/SCHEMA.yaml** — defines rule format but engine.py doesn't validate against it. Lower priority; add to Trust Root if SCHEMA itself is referenced by automated tests.
    93	3. **build-check.sh + session-end.sh** — sister hooks of judge.sh. Lower-priority gates (build verification, session telemetry); add to Trust Root in next harness cycle.
    94	4. **R-016 fc_trace_in_commit upgrade** — currently WARN-level. If post-Phase-D evidence shows FC-trace discipline still slipping, promote to BLOCK-level.
    95	5. **R-020 ground_truth_label** — sketched in A0a planning but not implemented (grep on PputResult/RunAggregate field additions to enforce thesis claim 7 ground-truth source). Defer to next harness cycle.
    96	6. **FC2-N23 HaltReason full taxonomy as Rust enum** — currently only OmegaAccepted is typed; other 4 variants live as jsonl strings. Phase C+ Soft Law mode work may force this typing.
    97	7. ~~**Per-line FC tagging via tracing crate** — Plan agent's recommendation in N-experiments brainstorm. Phase A6 deferred; will land before Phase B (homogeneous experiments).~~ **A6 LANDED** (commit pending): `fc_trace.rs` module + 6 wired anchor sites. Implementation chose pure stdlib over the `tracing` crate to avoid a new dep tree; the macro surface (`emit_event` + `FcId` enum) was kept small so Phase D+ can swap to a real `tracing-subscriber` bridge locally.
    98	
    99	## § 6. Updated counts (v2)
   100	
   101	Compared to v1:
   102	- ✅ count: 16 → **33** (+17 from fc_alignment_conformance.rs witness battery; +4 from new symbols/rules; +4 from manifest expansion; +5 case-law entries; -3 stale)
   103	- 📅/📄 count: 9 → **9** (Phase 11+ deferred unchanged; some clarified with case references)
   104	- 🔨/⚠️ count: 0 → **0** (no actionable rows pending in v2 scope)
   105	- New cases: 5 (C-071..C-075)
   106	- New rules: 4 active (R-014/R-015/R-018/R-019) + 1 hook-level (R-016)
   107	
   108	Manifest size milestones:
   109	- B7 → 15
   110	- B7-extra → 16
   111	- B7-extra round-1 audit-fix → 20
   112	- A0 (this v2) → 24
   113	- A1 PREREG amendment → 25
   114	- A5 budget_regime.rs → 26
   115	- A6 fc_trace.rs → 27
   116	- A7 llm_proxy.py + smoke_siliconflow.{sh,py} → **30**
   117	- (planned) B7-extra calibration freeze → 31
   118	- (planned) Phase C mode-flag binary → 32+
   119	
   120	## § 7. Cross-references
   121	
   122	- `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md` (immutable baseline)
   123	- `handover/alignment/TRACE_MATRIX_v1_2026-04-25.md` (B7 + B7-extra v1)
   124	- `handover/alignment/FC_ELEMENTS_2026-04-22.md` (canonical FC node IDs)
   125	- `handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md` (FC3-E14 vs FC2-N22 distinction)
   126	- `handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md` (Findings A+B)
   127	- `handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md` (Findings C+D)
   128	- `cases/C-071`..`C-075`.yaml (Phase A0 case-law)
   129	- `~/.claude/.../memory/feedback_fc_first_problem_handling.md` (FC-first rule memory)

exec
/bin/bash -lc "nl -ba handover/audits/A8_EXIT_PACKET_2026-04-26.md | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# Phase A → B Exit Audit Packet (A8)
     2	
     3	**Arc**: PPUT-CCL (`PREREG_PPUT_CCL_2026-04-26.md` round-4 PASS/PASS + amendment).
     4	**Date**: 2026-04-26.
     5	**Authority**: ArchitectAI commit (Art. V.1.2). This packet is the input to dual external audit (Codex + Gemini) per Art. V.1.3 + memory `feedback_dual_audit`. Decision rule: PASS → Phase B (kernel instrumentation) authorized; CHALLENGE → in-cycle fixes; VETO → Phase A redesign.
     6	
     7	**FC-trace**: meta-witness across FC1 / FC2 / FC3 (atoms instrument all three subgraphs).
     8	
     9	---
    10	
    11	## § 1. Phase A scope and atom map
    12	
    13	Phase A = pre-flight (days 1–3 of the 30-day arc). Decomposed into 8 atoms post the 2026-04-25 architect FULL PASS rewrite:
    14	
    15	- **A0** (a–e): harness modernization. Closed by `62c4e14` (A0e exit audit + 7-item fixes).
    16	- **A1**: PREREG amendment p_0 calibration deferral + Trust Root 24 → 25.
    17	- **A2**: P0a `swarm_N=1` mode + `parse_swarm_condition_n` unit tests.
    18	- **A3**: per-agent `AGENT_MODELS` env var (Phase B+C single-model invariant gate).
    19	- **A4**: decomposed metrics (`hit_max_tx` + `tactic_diversity` + `verifier_wait_ms`).
    20	- **A5**: per-agent budget normalization (`BUDGET_REGIME` + `MAX_TRANSACTIONS`).
    21	- **A6**: per-line FC tagging via structured JSON events (`fc_trace` module).
    22	- **A7**: SiliconFlow heterogeneous-LLM provider plumbing (proxy + 3-key smoke).
    23	- **A8**: this packet — Phase A → B exit audit.
    24	
    25	Commit chain (atomic, FC-traced, all under ArchitectAI commit authority — none touched `constitution.md`):
    26	
    27	```
    28	2e7f75a  A0a: 4 new harness rules + judge.sh constitution-special-case
    29	d8950ee  A0b: tests/fc_alignment_conformance.rs witness battery
    30	2a65339  A0c: 5 new cases C-071..C-075 sediment 2026-04-25 session decisions
    31	e94e1b9  A0d: TRACE_MATRIX_v2 + Trust Root manifest 20 → 24 (harness in TR)
    32	62c4e14  A0e: Phase A0 exit audit (CHALLENGE/CHALLENGE) + 7-item fixes
    33	6be6eb4  A1:  PREREG amendment defer p_0 calibration + Trust Root 24 → 25
    34	180a300  A2:  P0a swarm_N=1 mode + parse_swarm_condition_n unit tests
    35	7f4bc0c  A3:  per-agent AGENT_MODELS env var (Phase B+C single-model gate)
    36	a5c78e4  A4:  decomposed metrics (hit_max_tx + tactic_diversity + verifier_wait_ms)
    37	30f2a14  A5:  per-agent budget normalization (BUDGET_REGIME + MAX_TRANSACTIONS env vars)
    38	89994c7  A6:  per-line FC tagging via structured JSON events (fc_trace module)
    39	90953d6  A7:  SiliconFlow heterogeneous-LLM provider plumbing (proxy + 3-key smoke)
    40	```
    41	
    42	## § 2. Test count and Trust Root deltas
    43	
    44	|        | A0a baseline | A0e PASS | A4 land | A5 land | A6 land | A7 land |
    45	|---|---|---|---|---|---|---|
    46	| `cargo test --workspace` PASS | 187 | 204 | 234 | 254 | 261 | 261 |
    47	| ignored | 20 | 29 | 29 | 29 | 29 | 29 |
    48	| failed | 0 | 0 | 0 | 0 | 0 | 0 |
    49	| Trust Root manifest entries | 20 | 24 | 24 | 26 | 27 | 30 |
    50	
    51	A7 added no new Rust tests (plumbing + integration gate; acceptance via `scripts/smoke_siliconflow.sh` PASS verified 2026-04-26 04:58 UTC).
    52	
    53	## § 3. Per-atom FC-trace map and acceptance evidence
    54	
    55	### A0 (harness modernization)
    56	**Closing audit**: `CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md` + `GEMINI_PHASE_A0_EXIT_AUDIT_2026-04-25.md`. Both returned CHALLENGE; 7 fixes landed in `62c4e14`. Final state PASS-equivalent (no open P0).
    57	- A0a (4 rules + judge.sh): R-014 / R-015 / R-018 / R-019 + R-016 fc_trace_in_commit. **FC-trace**: governance instrumentation; not a single FC node.
    58	- A0b (`tests/fc_alignment_conformance.rs`): 17 PASS witnesses + 9 `#[ignore]` stubs. **FC-trace**: meta-witness for FC1 / FC2 / FC3 ↔ Rust symbol mapping.
    59	- A0c (5 cases C-071…C-075): constitution amendment process / Veto-AI scope / ArchitectAI commit authority / FC-first / DO-178C tool qualification. **FC-trace**: Art. V (anchors all FC).
    60	- A0d (`TRACE_MATRIX_v2`): 17 ⚠️ → ✅ (status flips); manifest 20 → 24. **FC-trace**: meta.
    61	- A0e: 7 fixes addressing dual-audit CHALLENGE items.
    62	
    63	### A1 (PREREG amendment)
    64	- File: `handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md`.
    65	- Substitutes `p_0 = 0.10` (PREREG § 5.5 ceiling) for the calibration-derived value at every Gate H consumer. Mathematically conservative (strictest plausible bar; no Type-I inflation). Re-calibration conditions in § 3 list 5 items (N-experiments arc complete / swarm_N=1 mode landed / per-agent budget normalization landed / hetero-LLM exp complete / Phase D ArchitectAI runtime exists).
    66	- **FC-trace**: FC1-N12 (∏p ground-truth oracle scope unchanged) + Art. V.1.2 (commit authority) + cases C-073 + C-075.
    67	- Trust Root manifest 24 → 25.
    68	
    69	### A2 (`swarm_N=1` mode)
    70	- New `parse_swarm_condition_n` in `experiments/minif2f_v4/src/bin/evaluator.rs` discriminates `n<digits>` from `oneshot` / `hybrid_v1` / malformed. PREREG_AMENDMENT § 3 condition 2 cleared.
    71	- **FC-trace**: FC2-N16 InitAI orchestration entry — discriminates between the two registered InitAI shapes (oneshot vs swarm). FC1-N11 ∏p path is reached only via swarm.
    72	- Tests: 5 unit tests (`oneshot_returns_none` / `n1` / `n8` / `nfoo_rejected` / `n0_rejected`).
    73	
    74	### A3 (`AGENT_MODELS` env var)
    75	- New module `experiments/minif2f_v4/src/agent_models.rs`. Pure parser + expander + env-coupled resolver. Heterogeneity gated by `PHASE_D_HETERO_OK=1` — Phase B+C single-model invariant enforced at startup BEFORE any LLM call.
    76	- **FC-trace**: FC1-N7 (δ/AI canonical identity per Agent_i).
    77	- Tests: 11 unit tests (parse / expand / hetero gate / length mismatch).
    78	
    79	### A4 (decomposed metrics)
    80	- 3 non-Optional v2 fields on `RunAggregate` + legacy `PputResult`: `hit_max_tx`, `tactic_diversity`, `verifier_wait_ms`. Helper `compute_tactic_diversity`. All 9 `make_pput` call sites pass explicit values.
    81	- **FC-trace**: FC2-N22 (HALT decomposition for `hit_max_tx`) + FC1-N11 (∏p decision diversity for `tactic_diversity`) + FC1-N12 (oracle scope for `verifier_wait_ms`).
    82	- Tests: 5 (`test_a4_decomposed_metrics_round_trip`, `test_a4_tactic_diversity_helper`, `test_a4_verifier_wait_bounded_by_total_wall_time`, `test_a4_emit_max_tx_exhaustion_row`, `test_a4_synthetic_short_circuit_does_not_set_hit_max_tx`).
    83	
    84	### A5 (budget regime)
    85	- New module `experiments/minif2f_v4/src/budget_regime.rs`. 4-variant `BudgetRegime` enum: `total_proposal` (default; current behavior preserved bit-for-bit) / `per_agent` (loop bound = base × N) / `token_total` (declared; startup-fatal `UnimplementedRegime`) / `wall_clock` (declared; startup-fatal). 2 new non-Optional v2 fields: `budget_regime` + `budget_max_transactions`.
    86	- `run_swarm` startup: `let max_transactions = 200` → `resolve_budget(n_agents)` with startup-fatal error path.
    87	- **FC-trace**: FC2-N22 (HALT decomposition by budget regime) + FC1-N7 (δ instances determining the per-agent share under PerAgent regime).
    88	- Tests: 16 (15 budget_regime unit + 1 jsonl_schema A5 round-trip).
    89	- PREREG_AMENDMENT § 3 condition 3 cleared.
    90	- Trust Root manifest 25 → 26.
    91	
    92	### A6 (FC tracing)
    93	- New module `experiments/minif2f_v4/src/fc_trace.rs`. Pure stdlib (zero new deps). 7-variant `FcId` enum (FC1-N7 / FC1-N11 / FC1-N12 / FC1-E18 / FC2-N20 / FC2-N22 / FC3-N31). `FC_TRACE=1` gate cached in `OnceLock`; `FC_TRACE_FILE=<path>` redirects emit to file.
    94	- 6 wired anchor sites in `run_swarm` + 1 in `run_oneshot`: synthetic short-circuit / mr tick / OMEGA full-proof / OMEGA per-tactic / natural MaxTxExhausted (with budget_regime payload from A5) / oneshot verify bracket.
    95	- **FC-trace**: meta-witness for the 5-step compile loop.
    96	- Tests: 7 (6 unit + 1 end-to-end smoke `tests/fc_trace_smoke.rs` exercising `FC_TRACE=1` in a child process — required because the gate is `OnceLock`-cached).
    97	- Trust Root manifest 26 → 27.
    98	- Resolves TRACE_MATRIX_v2 § 5 item 7.
    99	
   100	### A7 (SiliconFlow plumbing)
   101	- `src/drivers/llm_proxy.py` ported from v3 with one load-bearing v4 change: per-provider multi-key round-robin. PROVIDERS map now holds a list of env names per provider; `get_client_round_robin` distributes via `_rr_counters` mod `len(clients)`. `/stats` exposes `per_key_requests` for observability. New `siliconflow:<model>` provider-prefix syntax.
   102	- 3 SiliconFlow keys (primary / secondary / tertiary) split concurrent traffic across separate rate-limit pools — V3L-27 (case C-027) single-key N=30 401/429 collapse mitigation.
   103	- `scripts/smoke_siliconflow.sh` + `_smoke_siliconflow.py`: 3 keys × 1 probe (Qwen2.5-7B-Instruct, max_tokens=8). Verified 2026-04-26: primary 2989ms, secondary 1546ms, tertiary 1549ms; 33+1 tokens; content="ack". Proxy round-robin verified [2,2,2] across 6 calls.
   104	- **FC-trace**: FC1-N7 (δ/AI provider expansion).
   105	- No new Rust tests (integration plumbing).
   106	- Memory: `reference_siliconflow.md` records SiliconFlow as the Phase D heterogeneous lane (NOT a probe-only target) and the context-loss anti-pattern (check `.env` + project files BEFORE asking for credentials).
   107	- Trust Root manifest 27 → 30.
   108	
   109	## § 4. Phase B → C exit checklist (from PREREG_AMENDMENT § 4) — Phase A side
   110	
   111	The PREREG amendment shifted the Phase B → C gate. From the Phase A perspective, the items it lists are now satisfied:
   112	
   113	- ❌ p_0 calibration jsonl frozen (was REQUIRED) → **DEFERRED with substitution per amendment § 2**: `p_0 = 0.10` hardcoded at every Gate H consumer.
   114	- ✅ B1–B7 + B7-extra mode toggle infrastructure complete (pre-Phase A baseline; round-4 PASS/PASS).
   115	- ✅ Phase A0 harness modernization complete (`62c4e14`).
   116	- ✅ Tools qualified per case C-075 (DO-178C tool qualification): `runner.sh`, `compute_p0.py`, evaluator boot enforcement, etc.
   117	- ✅ Trust Root verifies clean (`boot::tests::verify_trust_root_passes_on_intact_repo` PASS at 30-entry manifest).
   118	
   119	## § 5. Risks and known limitations entering Phase B
   120	
   121	1. **`per_agent` budget regime untested at runtime**. A5 unit tests verify the scaling math (`base × N`) and env-coupled resolver. No live-LLM run with `BUDGET_REGIME=per_agent` has been smoked. Phase B kernel instrumentation will be the first opportunity to observe its behavior on a real problem; defer treatment to PREREG re-calibration if any anomaly surfaces.
   122	2. **FC-trace coverage is sparse**. 6 wired anchor sites cover the HALT decomposition (FC2-N22 in 4 distinct exit paths) and one verify bracket. FC1-N11 ∏p decision diversity, FC1-E18 preserve-Q_t, and FC3-N31 WAL append are NOT yet emitting events — the `FcId` enum reserves the variants but no call site uses them. Phase B+ kernel instrumentation should fill these in as the Phase B emit boundary lands.
   123	3. **SiliconFlow rate-limit at scale**. A7 verified 3 keys responding individually at N=1 concurrency. V3L-27 demonstrates collapse at N=30 single-key. The v4 multi-key round-robin should triple the safe N envelope but the actual sweet spot for our hetero swarm is unmeasured. Phase D heterogeneous-batch design should land a `--max-concurrency` knob (currently `LLM_PROXY_CONCURRENCY=5` env in proxy) tuned per provider.
   124	4. **Heterogeneous swarm = Phase D, not B/C**. Per F-2026-04-25-02 + the `agent_models.rs` `PHASE_D_HETERO_GATE_ENV_VAR` invariant, Phases B and C MUST stay single-model so ablation axes are not confounded. A7's plumbing exists for future Phase D work; Phase B uses the existing `deepseek-v4-flash` thinking-off backbone unchanged.
   125	5. **No FC1-N12 emit in `run_swarm` verify path**. A6 wired FC1-N12 only in `run_oneshot`. The two `verify_omega_detailed` and one `verify_partial` brackets in `run_swarm` still increment `verifier_wait_ms` (A4) but don't emit FC events. Phase B kernel instrumentation should add these (one event per Lean call, with verdict + elapsed_ms).
   126	
   127	## § 6. Specific questions for auditors
   128	
   129	### Q1 — Atomicity and FC-trace discipline
   130	- (Q1.a) Each of A1–A7 was committed as one atomic commit with `FC-trace: <FC?-N?>` in the commit message, per memory `feedback_fc_first_problem_handling`. Verify by re-reading each commit message. Are any commits missing FC-trace, or claiming an FC node that doesn't match the actual code change?
   131	- (Q1.b) Did any commit touch `constitution.md`? (Per Art. V.1.1 + R-018 BLOCK, this is a violation absent sudo.) Trace each commit's file changes.
   132	- (Q1.c) Are there code changes outside the listed atoms — e.g., drift from un-committed tweaks — that should have been their own atom?
   133	
   134	### Q2 — PREREG amendment soundness (A1)
   135	- (Q2.a) Substituting `p_0 = 0.10` at the Gate H consumer (the PREREG § 5.5 ceiling) is described as conservative. Verify: does this introduce any Type-I inflation, multiplicity hazard, or family-construction inconsistency vs the original 576-run calibration protocol?
   136	- (Q2.b) Re-calibration conditions list 5 items (notepad ref + per-atom commits). Does this implicitly couple Phase B → C to Phase D readiness in a way that would block forward progress if Phase D slips?
   137	- (Q2.c) The amendment doc itself is in Trust Root (entry 25). Is the amendment's own SHA-256 referenced anywhere that would prevent a silent re-edit?
   138	
   139	### Q3 — Budget regime soundness (A5)
   140	- (Q3.a) `BUDGET_REGIME=per_agent` scales the loop bound as `base × N`. For a swarm at N=8 with `MAX_TRANSACTIONS=200`, the loop runs 1600 iterations — each agent receives ~200 proposals. Does this match the brainstorm § A.3 "fixed proposal budget" intent, or is a per-agent counter (each agent independently capped at 200, regardless of round-robin pickup) more faithful?
   141	- (Q3.b) `token_total` and `wall_clock` regimes are declared startup-fatal `UnimplementedRegime`. Is "fail loud" the right default, or should they fall back to `total_proposal` with a WARN log?
   142	- (Q3.c) The default (env unset) preserves Phase B baseline `total_proposal × 200` bit-for-bit. Verify this is true under all code paths — including the synthetic short-circuit and error/timeout exits.
   143	
   144	### Q4 — FC tracing coverage (A6)
   145	- (Q4.a) 6 wired anchor sites cover only FC2-N22 (HALT, 4 paths) + FC2-N20 (mr tick) + FC1-N12 (oneshot verify only). FcId enum has 7 variants but only 3 are emitted. Is the partial coverage acceptable for Phase A exit, or does this block Phase B (where the kernel instrumentation needs the full 5-step compile loop visible)?
   146	- (Q4.b) `OnceLock`-cached gate read means a process started with `FC_TRACE=0` (or unset) ignores any later runtime change. Acceptable for evaluator's one-process-per-problem model, but does it pose a risk for any test or runner that mutates the env mid-process?
   147	- (Q4.c) Hand-rolled JSON encoder vs the `serde_json` already in deps. Was there a real reason to avoid `serde_json::to_string` here, or is this premature dep avoidance?
   148	- (Q4.d) `run_corr_id` format = `condition_problem_id_unix-ms`. `make_pput`'s `run_id` independently re-computes this with its own ts. The two will differ by milliseconds. Is the join semantics for Phase D consumers documented anywhere?
   149	
   150	### Q5 — SiliconFlow plumbing (A7)
   151	- (Q5.a) `detect_provider` model-prefix logic: a model id with `/` and not starting with "qwen" routes to `siliconflow`. Edge cases: `openai/gpt-4o`, `Qwen/Qwen2.5-7B-Instruct` (capital Q), `siliconflow:Qwen/...`. Verify the routing matrix is complete.
   152	- (Q5.b) Round-robin counter `_rr_counters[provider]` increments unboundedly. Modulo wrap is at u64 max — practically unreachable, but is there a cleaner pattern (use `itertools.cycle` lazily)?
   153	- (Q5.c) `_per_key_requests[provider]` list is mutated under the same `_rr_lock` as the counter. Is the lock granularity right (per-provider lists could use per-provider locks for higher concurrency)?
   154	- (Q5.d) `LLM_PROXY_CONCURRENCY` defaults to 5. With 3 SF keys, that's 5 concurrent calls split across 3 keys ≈ 1.67 per key. Is this low enough to avoid V3L-27 collapse, or should Phase D recommend `LLM_PROXY_CONCURRENCY=15` (5 per key)?
   155	- (Q5.e) Smoke is a single direct-SDK probe per key — bypasses the proxy. This is intentional (per-key verdict). But should there ALSO be a proxy-routed smoke as a follow-up (to catch routing bugs)?
   156	
   157	### Q6 — Trust Root manifest expansion 24 → 30
   158	6 new entries this Phase A: PREREG amendment (A1) + budget_regime.rs (A5) + fc_trace.rs (A6) + llm_proxy.py + smoke_siliconflow.sh + _smoke_siliconflow.py (A7).
   159	- (Q6.a) Are all 6 truly load-bearing? E.g., does tampering with `_smoke_siliconflow.py` actually weaken the constitutional gate, or is it a one-shot acceptance script?
   160	- (Q6.b) `llm_proxy.py` is in Python — Trust Root verifies SHA-256, but does NOT verify Python interpreter behavior. A subtle algorithmic tamper (e.g., always-key-0 instead of round-robin) would change the SHA but a no-op cosmetic edit wouldn't trigger any acceptance test. Is the smoke script's `[2,2,2]` invariant codified anywhere as a recurring conformance test?
   161	- (Q6.c) `.env` (containing live API keys) is gitignored and NOT in Trust Root. Should the absence of `.env` be a Trust Root constraint (boot fails if `.env` is missing OR if it's tracked)?
   162	
   163	### Q7 — Phase A → B exit decision
   164	- (Q7.a) Verify `cargo test --workspace` PASS count: 261 / 29 ignored / 0 failed. Spot-check by re-running.
   165	- (Q7.b) Verify `boot::tests::verify_trust_root_passes_on_intact_repo` PASS at 30-entry manifest.
   166	- (Q7.c) Verify `scripts/smoke_siliconflow.sh` PASS (live API; cost ~$0.005).
   167	- (Q7.d) Are there OPEN P0 defects from any earlier Phase A audit (A0e CHALLENGE/CHALLENGE → 7 fixes)? Re-read the A0e audit + verify each of the 7 items closed.
   168	- (Q7.e) Phase B's first sub-atom is "JSONL schema v2 + C_i full-cost aggregator" (notepad). Are there any Phase A artifacts that would BLOCK that scope? E.g., schema fields needed by B1 that aren't yet in `RunAggregate`?
   169	
   170	---
   171	
   172	End your audit with:
   173	- **VERDICT**: PASS / CHALLENGE / VETO
   174	- **Conviction**: low / medium / high
   175	- **Recommendation**: PROCEED to Phase B (kernel instrumentation + PPUT accounting) / FIX-THEN-PROCEED / REDESIGN
   176	
   177	PASS = all 7 atoms cleanly land, no open P0, Phase B prerequisites met. CHALLENGE = correctable in one cycle. VETO = Phase A redesign required.

exec
/bin/bash -lc "nl -ba handover/audits/A8_EXIT_PACKET_2026-04-26.md | sed -n '260,420p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
exec
/bin/bash -lc "rg -n \"FcId::|emit_event\\(|verify_omega_detailed|verify_partial|verify_omega\\(\" experiments/minif2f_v4/src/bin/evaluator.rs experiments/minif2f_v4/src/fc_trace.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/src/fc_trace.rs:55:    /// verify_omega_detailed / verify_partial call. Cumulative
experiments/minif2f_v4/src/fc_trace.rs:76:            FcId::Fc1N7 => "FC1-N7",
experiments/minif2f_v4/src/fc_trace.rs:77:            FcId::Fc1N11 => "FC1-N11",
experiments/minif2f_v4/src/fc_trace.rs:78:            FcId::Fc1N12 => "FC1-N12",
experiments/minif2f_v4/src/fc_trace.rs:79:            FcId::Fc1E18 => "FC1-E18",
experiments/minif2f_v4/src/fc_trace.rs:80:            FcId::Fc2N20 => "FC2-N20",
experiments/minif2f_v4/src/fc_trace.rs:81:            FcId::Fc2N22 => "FC2-N22",
experiments/minif2f_v4/src/fc_trace.rs:82:            FcId::Fc3N31 => "FC3-N31",
experiments/minif2f_v4/src/fc_trace.rs:127:pub fn emit_event(
experiments/minif2f_v4/src/fc_trace.rs:236:        assert_eq!(FcId::Fc1N7.as_str(), "FC1-N7");
experiments/minif2f_v4/src/fc_trace.rs:237:        assert_eq!(FcId::Fc1N11.as_str(), "FC1-N11");
experiments/minif2f_v4/src/fc_trace.rs:238:        assert_eq!(FcId::Fc1N12.as_str(), "FC1-N12");
experiments/minif2f_v4/src/fc_trace.rs:239:        assert_eq!(FcId::Fc1E18.as_str(), "FC1-E18");
experiments/minif2f_v4/src/fc_trace.rs:240:        assert_eq!(FcId::Fc2N20.as_str(), "FC2-N20");
experiments/minif2f_v4/src/fc_trace.rs:241:        assert_eq!(FcId::Fc2N22.as_str(), "FC2-N22");
experiments/minif2f_v4/src/fc_trace.rs:242:        assert_eq!(FcId::Fc3N31.as_str(), "FC3-N31");
experiments/minif2f_v4/src/fc_trace.rs:247:        assert_eq!(format!("{}", FcId::Fc2N22), "FC2-N22");
experiments/minif2f_v4/src/fc_trace.rs:292:        emit_event(FcId::Fc2N22, "test_run", None, None, &[]);
experiments/minif2f_v4/src/fc_trace.rs:297:        emit_event(
experiments/minif2f_v4/src/fc_trace.rs:298:            FcId::Fc1N12,
experiments/minif2f_v4/src/bin/evaluator.rs:162:    // gp_payload = the exact text fed to oracle.verify_omega_detailed at OMEGA accept.
experiments/minif2f_v4/src/bin/evaluator.rs:392:            let verdict = oracle.verify_omega(&response.content);
experiments/minif2f_v4/src/bin/evaluator.rs:404:            minif2f_v4::fc_trace::emit_event(
experiments/minif2f_v4/src/bin/evaluator.rs:405:                minif2f_v4::fc_trace::FcId::Fc1N12,
experiments/minif2f_v4/src/bin/evaluator.rs:678:    // inside Lean for THIS run. Each verify_omega_detailed and
experiments/minif2f_v4/src/bin/evaluator.rs:679:    // verify_partial call brackets its own elapsed and adds it here.
experiments/minif2f_v4/src/bin/evaluator.rs:722:            minif2f_v4::fc_trace::emit_event(
experiments/minif2f_v4/src/bin/evaluator.rs:723:                minif2f_v4::fc_trace::FcId::Fc2N22,
experiments/minif2f_v4/src/bin/evaluator.rs:774:            minif2f_v4::fc_trace::emit_event(
experiments/minif2f_v4/src/bin/evaluator.rs:775:                minif2f_v4::fc_trace::FcId::Fc2N20,
experiments/minif2f_v4/src/bin/evaluator.rs:1004:                                let r_alone = oracle.verify_omega_detailed(payload);
experiments/minif2f_v4/src/bin/evaluator.rs:1012:                                        let r_combined = oracle.verify_omega_detailed(&combined);
experiments/minif2f_v4/src/bin/evaluator.rs:1077:                                        minif2f_v4::fc_trace::emit_event(
experiments/minif2f_v4/src/bin/evaluator.rs:1078:                                            minif2f_v4::fc_trace::FcId::Fc2N22,
experiments/minif2f_v4/src/bin/evaluator.rs:1101:                                        // Lean verify_omega_detailed call we just consumed
experiments/minif2f_v4/src/bin/evaluator.rs:1271:                                let verdict = oracle.verify_partial(&prefix);
experiments/minif2f_v4/src/bin/evaluator.rs:1304:                                        minif2f_v4::fc_trace::emit_event(
experiments/minif2f_v4/src/bin/evaluator.rs:1305:                                            minif2f_v4::fc_trace::FcId::Fc2N22,
experiments/minif2f_v4/src/bin/evaluator.rs:1314:                                        // the Lean verify_partial call (PartialVerdict::Complete).
experiments/minif2f_v4/src/bin/evaluator.rs:1406:    minif2f_v4::fc_trace::emit_event(
experiments/minif2f_v4/src/bin/evaluator.rs:1407:        minif2f_v4::fc_trace::FcId::Fc2N22,

exec
/bin/bash -lc "rg -n \"p_0|baseline_regression_rate|0\\.10|Gate H|j-RR|regression\" handover/preregistration experiments src scripts -g '"'!target'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
scripts/constitutional_check.sh:167:    fail "Missing explicit rejection in protocol.rs (V-009 regression)"
scripts/constitutional_check.sh:173:    fail "Missing JSON prefix tolerance in protocol.rs (V-009 regression)"
experiments/minif2f_v4/run_batch.sh:83:# Catches missing Mathlib, toolchain mismatch, or sandbox regressions up front.
experiments/minif2f_v4/tests/pput_anti_goodhart.rs:107:/// the regression where a future code path adds a new LLM call but
experiments/minif2f_v4/tests/trust_root_immutability.rs:155:        "baseline_regression_rate",
experiments/minif2f_v4/tests/trust_root_immutability.rs:156:        "baseline_regression_jsonl_sha256",
experiments/minif2f_v4/src/jsonl_schema.rs:362:        // emit path crash-free under accumulator wiring regression.
experiments/minif2f_v4/src/bin/evaluator.rs:182:    /// p_0 estimation is unaffected; downstream PPUT analysis on these
experiments/minif2f_v4/src/bin/evaluator.rs:505:        // partials still typically <1200; no behavioural regression.
experiments/minif2f_v4/src/bin/evaluator.rs:660:        let t = if temp_ladder_on { (0.10_f64 + (i as f64) * 0.15).min(1.30) } else { 0.2 };
experiments/minif2f_v4/src/bin/evaluator.rs:902:            (0.10_f64 + (agent_idx as f64) * 0.15).min(1.30)
experiments/minif2f_v4/src/bin/evaluator.rs:1762:                 Schema-v2 contract regression — see B5 deferral checklist P0-B. \
experiments/minif2f_v4/exp_phase1_full.log:39:  [n1] ... SOLVED (10s PPUT=10.10)
experiments/minif2f_v4/analysis/PHASE7_DAG_ANALYSIS.md:11:Phase 7 achieves the **first structural parity** with v3 DAG topology: mixed-depth histogram {1:5, 3:1, 17:1, 20:1, 23:1} instead of prior phases' delta-function at {1:all}. The depth-23 imo_1964_p2 proof represents a genuine 23-step δ-chain, externally re-verifiable. However, step-only mode trades depth diversity for speed regression (9/20 vs 17/20 baseline), with 11 timeout failures due to per-tactic Lean elaboration latency. Economic mechanisms (Hayek bounty, rejection penalties) are live but don't yet show emergent role specialization seen in v3 run6.
experiments/minif2f_v4/analysis/phase2_ab_analyze.py:162:      - Paired ΔPPUT CI lies entirely below -0.10
experiments/minif2f_v4/analysis/phase2_ab_analyze.py:174:    if delta_hi < -0.10:
experiments/minif2f_v4/analysis/phase2_ab_analyze.py:175:        return (f"FAIL: Paired ΔPPUT CI upper {delta_hi:+.3f} < -0.10 (severe regression)")
experiments/minif2f_v4/analysis/phase2_ab_analyze.py:177:        return (f"FAIL: ΣPPUT gap {sigma_gap*100:.1f}% > 25% (severe regression)")
experiments/minif2f_v4/analysis/phase2_ab_analyze.py:192:    if delta_lo < 0 < delta_hi and sigma_gap > 0.10:
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s141421_n8_20260424T080939.err:3:[2026-04-24T08:12:06Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill3:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill1:t=0.85 Agent_6:skill2:t=1.00 Agent_7:skill3:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s141421_n8_20260424T080939.err:14:[2026-04-24T08:17:39Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill3:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill1:t=0.85 Agent_6:skill2:t=1.00 Agent_7:skill3:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s141421_n8_20260424T080939.err:49:[2026-04-24T08:32:39Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill3:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill1:t=0.85 Agent_6:skill2:t=1.00 Agent_7:skill3:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s141421_n8_20260424T080939.err:67:[2026-04-24T08:38:15Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill3:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill1:t=0.85 Agent_6:skill2:t=1.00 Agent_7:skill3:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_Abl_s31415_n8_20260424T080941.err:3:[2026-04-24T08:12:06Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill1:t=0.70 Agent_5:skill2:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill1:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_Abl_s31415_n8_20260424T080941.err:25:[2026-04-24T08:27:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill1:t=0.70 Agent_5:skill2:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill1:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_Abl_s31415_n8_20260424T080941.err:56:[2026-04-24T08:42:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill1:t=0.70 Agent_5:skill2:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill1:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s31415_n8_20260424T080941.err:3:[2026-04-24T08:12:06Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill3:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill1:t=0.85 Agent_6:skill2:t=1.00 Agent_7:skill3:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s31415_n8_20260424T080941.err:24:[2026-04-24T08:27:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill3:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill1:t=0.85 Agent_6:skill2:t=1.00 Agent_7:skill3:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s31415_n8_20260424T080941.err:64:[2026-04-24T08:42:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill3:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill1:t=0.85 Agent_6:skill2:t=1.00 Agent_7:skill3:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s2357_n8_20260424T080945.err:3:[2026-04-24T08:12:06Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill3:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill1:t=0.85 Agent_6:skill2:t=1.00 Agent_7:skill3:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s2357_n8_20260424T080945.err:15:[2026-04-24T08:18:17Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill3:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill1:t=0.85 Agent_6:skill2:t=1.00 Agent_7:skill3:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s2357_n8_20260424T080945.err:57:[2026-04-24T08:33:17Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill3:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill1:t=0.85 Agent_6:skill2:t=1.00 Agent_7:skill3:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s2357_n8_20260424T080945.err:65:[2026-04-24T08:36:12Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill3:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill1:t=0.85 Agent_6:skill2:t=1.00 Agent_7:skill3:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_Abl_s2357_n8_20260424T080945.err:3:[2026-04-24T08:12:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill1:t=0.70 Agent_5:skill2:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill1:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_Abl_s2357_n8_20260424T080945.err:24:[2026-04-24T08:27:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill1:t=0.70 Agent_5:skill2:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill1:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_Abl_s2357_n8_20260424T080945.err:57:[2026-04-24T08:42:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill1:t=0.70 Agent_5:skill2:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill1:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_Abl_s141421_n8_20260424T080939.err:3:[2026-04-24T08:12:06Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill1:t=0.70 Agent_5:skill2:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill1:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_Abl_s141421_n8_20260424T080939.err:24:[2026-04-24T08:25:09Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill1:t=0.70 Agent_5:skill2:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill1:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_Abl_s141421_n8_20260424T080939.err:70:[2026-04-24T08:40:09Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill1:t=0.70 Agent_5:skill2:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill1:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_Abl_s2718_n8_20260424T080943.err:3:[2026-04-24T08:12:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill1:t=0.70 Agent_5:skill2:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill1:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_Abl_s2718_n8_20260424T080943.err:24:[2026-04-24T08:27:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill1:t=0.70 Agent_5:skill2:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill1:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_Abl_s2718_n8_20260424T080943.err:61:[2026-04-24T08:42:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill1:t=0.70 Agent_5:skill2:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill1:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_A_s31415_n8_20260424T080941.err:3:[2026-04-24T08:12:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill0:t=0.25 Agent_2:skill0:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill0:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill0:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_A_s31415_n8_20260424T080941.err:22:[2026-04-24T08:27:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill0:t=0.25 Agent_2:skill0:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill0:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill0:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_A_s31415_n8_20260424T080941.err:48:[2026-04-24T08:42:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill0:t=0.25 Agent_2:skill0:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill0:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill0:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_A_s2357_n8_20260424T080945.err:3:[2026-04-24T08:12:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill0:t=0.25 Agent_2:skill0:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill0:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill0:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_A_s2357_n8_20260424T080945.err:29:[2026-04-24T08:27:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill0:t=0.25 Agent_2:skill0:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill0:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill0:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_A_s2357_n8_20260424T080945.err:53:[2026-04-24T08:42:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill0:t=0.25 Agent_2:skill0:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill0:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill0:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_A_s141421_n8_20260424T080939.err:3:[2026-04-24T08:12:06Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill0:t=0.25 Agent_2:skill0:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill0:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill0:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_A_s141421_n8_20260424T080939.err:24:[2026-04-24T08:27:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill0:t=0.25 Agent_2:skill0:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill0:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill0:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_A_s141421_n8_20260424T080939.err:50:[2026-04-24T08:42:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill0:t=0.25 Agent_2:skill0:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill0:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill0:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s2718_n8_20260424T080943.err:3:[2026-04-24T08:12:06Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill3:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill1:t=0.85 Agent_6:skill2:t=1.00 Agent_7:skill3:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s2718_n8_20260424T080943.err:35:[2026-04-24T08:27:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill3:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill1:t=0.85 Agent_6:skill2:t=1.00 Agent_7:skill3:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s2718_n8_20260424T080943.err:79:[2026-04-24T08:42:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill3:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill1:t=0.85 Agent_6:skill2:t=1.00 Agent_7:skill3:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_A_s2718_n8_20260424T080943.err:3:[2026-04-24T08:12:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill0:t=0.25 Agent_2:skill0:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill0:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill0:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_A_s2718_n8_20260424T080943.err:23:[2026-04-24T08:27:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill0:t=0.25 Agent_2:skill0:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill0:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill0:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_A_s2718_n8_20260424T080943.err:49:[2026-04-24T08:42:07Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill0:t=0.25 Agent_2:skill0:t=0.40 Agent_3:skill0:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill0:t=0.85 Agent_6:skill0:t=1.00 Agent_7:skill0:t=1.15
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:283:baseline_regression_rate = 0.0  # PLACEHOLDER until Phase B7-extra calibration
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:284:baseline_regression_jsonl_sha256 = ""  # PLACEHOLDER until Phase B7-extra
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:308:### B7-extra — p_0 calibration (data + freeze)
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:310:**What**: per PREREG § 5.5, run the calibration protocol to compute `p_0` (baseline regression rate); freeze into `genesis_payload.toml [pput_accounting_0]`.
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:322:**Compute p_0**:
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:323:- For each (problem, seed): regression_p = 1 iff control SOLVED AND treatment UNSOLVED
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:324:- Per-problem regression: max over the 2 seeds (worst case per PREREG § 5.5)
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:325:- p_0 = sum_p regression_p / 144
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:327:**Sanity gate**: if p_0 > 0.10, ABORT — toggle too aggressive (per PREREG § 5.5 ceiling). Redesign rollback simulation, redo.
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:330:- Write p_0 value to `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:331:- Compute SHA-256 of the calibration jsonl file → write to `[pput_accounting_0].baseline_regression_jsonl_sha256`
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:336:- p_0 ∈ (0, 0.10]
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:338:- Dual-audit packet for Phase B → C transition includes the p_0 result
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:353:                       B7-extra (p_0 calibration) — runs after toggle code (part of B7)
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:363:- Day 7: B7-extra p_0 calibration runs (overnight) + Phase B → C audit packet
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:373:- [ ] `pput_accounting_0` block in `genesis_payload.toml` filled (including p_0 + jsonl hash + Trust Root hashes)
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:384:4. **`p_0` ceiling 0.10 — what if 0/144 = exactly 0**: this passes the ceiling but means the rollback simulation has no effect, which would invalidate j-RR's role. If observed p_0 = 0, redesign the rollback simulation toggle (probably `--simulate-rollback-at-tx-25` is too late; try tx-10 or per-call corruption).
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s141421_n8_20260424T100952.err:3:[2026-04-24T10:10:03Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill3:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill1:t=0.85 Agent_6:skill2:t=1.00 Agent_7:skill3:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s141421_n8_20260424T100952.err:63:[2026-04-24T10:19:38Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill3:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill1:t=0.85 Agent_6:skill2:t=1.00 Agent_7:skill3:t=1.15
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s141421_n8_20260424T100952.err:87:[2026-04-24T10:21:36Z INFO  evaluator] [swarm/n8] Agent_0:skill0:t=0.10 Agent_1:skill1:t=0.25 Agent_2:skill2:t=0.40 Agent_3:skill3:t=0.55 Agent_4:skill0:t=0.70 Agent_5:skill1:t=0.85 Agent_6:skill2:t=1.00 Agent_7:skill3:t=1.15
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:1:# PREREG Amendment — p_0 Calibration Deferral (2026-04-25)
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:11:PREREG § 5.5 specifies p_0 calibration via 576 paired runs (144 adaptation × 2 seeds × {control, treatment}) with estimated cost "~8 wall-hours, ~$3-5". Empirical observation 2026-04-25 during launched batch (commit 650caf7+ era):
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:23:PREREG § 5.5 calibration **DEFERRED** indefinitely with the following operative substitution for Phase B → C transition and Phase E Gate H requirements:
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:25:**`p_0` for guardrail purposes**: take the **PREREG § 5.5 ceiling itself = 0.10** as the conservative upper bound. Any artifact j whose `j-RR` regression rate exceeds 0.10 fails Gate H per the original guardrail logic; setting `p_0 = 0.10` (the maximum tolerated value) is the strictest possible substitute when no calibrated tighter value exists. This is mathematically conservative: artifacts must clear the strictest plausible bar, not a narrower data-derived bar.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:27:**`genesis_payload.toml [pput_accounting_0].baseline_regression_rate`**: setting deferred to ArchitectAI commit window. Current value `0.0` is recognized as INVALID PLACEHOLDER (would auto-fail any artifact with any regression). Until calibration runs, **Gate H consumers MUST hardcode `p_0 = 0.10`** at the consumption site, not read from `genesis_payload.toml`.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:29:**`baseline_regression_jsonl_sha256`**: stays empty (calibration jsonl does not exist yet).
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:35:1. **N-experiments arc (Phase A-D of new plan, 2026-04-25 N-agents experiments) complete** — answers Q1/Q2/Q3 about N → PPUT, fixes (or rejects) the throttle hypothesis, sediments per-N best practices into evaluator. Without this, calibrating p_0 on a known-suboptimal N=3 swarm is calibrating against a moving baseline.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:43:5. **Phase D ArchitectAI runtime exists** — calibration is part of Gate H gating Phase E. Doing it before Phase D = calibrating against a counterfactual ArchitectAI that doesn't exist.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:49:PREREG_PPUT_CCL_2026-04-26 § 5.5 originally listed p_0 calibration as a Phase B prerequisite ("Schedule: Phase B7 mandatory; not deferrable to Phase D"). This amendment **explicitly OVERRIDES that "not deferrable" clause** for the deferral conditions in § 3 above.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:52:- ❌ p_0 calibration jsonl frozen (was REQUIRED) → now DEFERRED with substitution per § 2
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:58:Phase B → C dual-audit packet (next major milestone) must reference this amendment + show that Phase E Gate H consumer hardcodes `p_0 = 0.10`.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:64:- **PREREG § 5.4 j-RR ≤ p_0 guardrail logic** — Gate H still uses the guardrail; just the p_0 source changes (hardcoded 0.10 instead of calibrated value).
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:88:- Does substitution of `p_0 = 0.10` invalidate any Gate H statistical claim? (Should not — strictest plausible bar is conservative; no Type-I inflation.)
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:17:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:92:| WBCG_PPUT | sum over candidate artifacts Δ of `1[ArtifactState(Δ) = Certified]` where `Certified` requires (i) j-PPUT + j-FAR + j-CPR all reject null at family-corrected α per § 5.3 + § 9, (ii) j-RR ≤ p_0 point-check guardrail per § 5.4, (iii) Rollbackable(Δ) = 1 per § 6 E2 (see § 1.7 for ArtifactState; § 7 Gate H for full reachability conditions) | Capability-compilation success |
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:115:              family-corrected α (per § 5.3 + § 9), AND j-RR ≤ p_0 point check
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:118:              toward WBCG_PPUT (§ 1.6 + § 7 Gate H).
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:119:Reverted    : Δ failed any of the three primary tests, OR violated the j-RR
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:120:              point-check guardrail, OR caused regression in post-Phase-E
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:374:**Final FULL PASS** = WBCG_PPUT > 0 on heldout (Gate H, § 7).
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:406:For each artifact j ∈ {1, ..., k} (k ≤ k_max = 10) entering Phase E with `ArtifactState = Quarantined`, **three** primary inferential tests run on heldout-54 (NOT four — j-RR is a descriptive guardrail per § 5.4):
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:414:`j-RR` is **NOT** in the inferential family — see § 5.4.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:416:### 5.4 j-RR as descriptive guardrail (round 4 fix per Codex round-3 R4-2)
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:418:**Why this changed**: Round 3 placed j-RR in the inferential family with H0: `regression_rate ≥ p_0 = 0.10`. Codex round-3 noted this is mathematically unwinnable: best case 0/54 regressions gives `p = 0.9^54 ≈ 0.00343`, but smallest Holm threshold at N_max = 44 was 0.05/44 ≈ 0.00114 → fails to reject even with zero regressions. Gate H unreachable.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:420:**Round 4 fix**: j-RR is reclassified as a **descriptive guardrail** (point estimate check, no α correction, no place in family):
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:423:j-RR criterion (point check, NOT inferential):
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:427:  -- Problems where artifact j SPECIFICALLY caused a regression versus
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:428:  -- baseline, attribution by leave-one-out: removing j fixes the regression.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:432:  Pass criterion: RR_j ≤ p_0   where p_0 is calibrated per § 5.5.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:435:**Why descriptive is statistically correct here**: j-RR is a constraint, not a discovery hypothesis. We do not need to argue "RR_j is significantly less than p_0"; we only need RR_j ≤ p_0. A point check is precisely the right tool. No multiplicity correction applies because this is not a test.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:437:### 5.5 p_0 calibration protocol
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:439:`p_0` (baseline regression rate) — full calibration protocol:
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:448:Estimator:     for each problem p, define regression_p = 1 iff control SOLVED
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:450:               Per-problem regression: max over the 2 seeds (worst case).
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:451:               p_0 = sum_p regression_p / 144.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:453:               start without p_0 frozen.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:455:                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:457:                 `[pput_accounting_0].baseline_regression_jsonl_sha256`.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:462:Ceiling:       if measured p_0 > 0.10, ABORT — toggle too aggressive or system
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:463:               too brittle. NOT an opportunity for j-RR auto-pass.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:464:Dual audit:    p_0 calibration result in Phase B → C audit packet.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:476:j-RR is NOT in the family because it is a point-check guardrail.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:481:Gate H: count of artifacts j with ArtifactState(j) = Certified ≥ 1
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:486:2. **j-RR ≤ p_0** (point check guardrail, no α correction; per § 5.4),
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:489:H5 is a deterministic AND of (3 inferential rejections) + (1 point check) + (1 rollback verification). Reaching Gate H = at least 1 artifact passes all five conditions. H5 itself is not a hypothesis test; no separate p-value.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:549:- E3 Statistical inference per § 5.3 + § 9. For each j, evaluate the **three** primary inferential tests (j-PPUT, j-FAR, j-CPR) under Holm-Bonferroni at family `4 + 3k`, N_max = 34. j-RR is a separate descriptive guardrail check (point estimate ≤ p_0 per § 5.4), NOT in the inferential family.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:567:| H | **Heldout WBCG_PPUT > 0 with ArtifactState = Certified gate** (round 4 — j-RR moved out of inferential family per Codex round-3): at least one ArchitectAI-generated user-space artifact Δ has `ArtifactState(Δ) = Certified` per § 1.7 — meaning (i) was Accepted (passed § 3 + § 3.5 AuditorAI battery), (ii) was Quarantined (user-approved + ΔPPUT_meta_val > 0 + N_use ≥ 3 on meta_val), (iii) at heldout sealed eval **three** primary inferential tests reject null at family-corrected α (per § 5.3 + § 9): **j-PPUT** (sign test on n=54), **j-FAR** (non-inferiority sign test), **j-CPR** (non-inferiority sign test), AND (iv) **j-RR ≤ p_0 point check guardrail** (per § 5.4 — descriptive, NOT inferential, NO α correction), AND (v) is rollbackable (artifact directory deletion at the protocol level restores prior heldout PPUT — verified by an explicit rollback sub-eval, see § 6 E2). Quarantined-only artifacts (failed any of the five conditions) do NOT count toward Gate H. |
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:593:j-RR is excluded from the family (descriptive guardrail per § 5.4). H5 (Gate H) is excluded (deterministic AND per § 5.7).
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:665:| "TuringOS achieves capability compilation" | WBCG_PPUT > 0 on heldout (Gate H, § 7) | "on MiniF2F-Test heldout-54 with deepseek-v4-flash thinking-off + Phase D heterogeneous (v4-flash thinking-on + Gemini 2.5 Pro)" |
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:680:| Phase D ≥ 1 Quarantined, Phase E zero Certified | "Phase D produced k Quarantined artifact(s) with positive ΔPPUT on meta_validation; on heldout-54 sealed eval, no artifact passed all five Certified conditions (three inferential tests j-PPUT / j-FAR / j-CPR at family-corrected α + j-RR ≤ p_0 point check + Rollbackable=1); capability-compilation gain on Phase D set did not generalize to heldout under Holm-Bonferroni correction" |
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:682:| Phase E sealed eval but rollback test fails | "FULL PASS NOT achieved — Gate H sub-criterion (rollbackable) did not hold; artifact effects entangled with adaptation state and could not be cleanly reverted; arc reported as infrastructure-built / capability-not-cleanly-attributed" |
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:760:| 2026-04-26 | **Patch A** (Gemini DeepThink): § 1.7 ArtifactState 4-state machine for artifacts (Accepted / Quarantined / Certified / Reverted); § 1.6 WBCG_PPUT row + § 7 Gate H tightened to "Certified-only". Task Progress remains binary (Lean perfect predicate). | Gemini DeepThink 2026-04-26 PPUT-driven FULL PASS confirmation |
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:766:| 2026-04-26 (round 2) | **M1**: § 5 restructured — each H1-H4 has ONE primary endpoint (per-(problem,seed) Lean-verified Progress sign or paired VPPUT sign); H5 reclassified as deterministic gate (no α). § 5.2 added — per-artifact heldout family `4·k` tests (j-PPUT / j-FAR / j-RR / j-CPR). § 5.3 family size = `4 + 4k`. § 6 C2 independent unit clarified = (problem, seed). § 9 rewritten with Holm-Bonferroni stepwise procedure + power expectation + family construction frozen at A5. | Codex audit M1 + Gemini audit M1 (round 1 dual-CHALLENGE convergent) |
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:769:| 2026-04-26 (round 2) | **M4**: § 7 Gate H — RR criterion relaxed from brittle `RR = 0` to per-problem one-sided exact binomial test against pre-registered baseline regression rate `p_0` (calibrated in Phase B6/B7 on adaptation set). Reachability calibrated to empirical reality. | Gemini audit M4 (round 1 GATE-H REACHABILITY CHALLENGE — Codex missed) |
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:777:| 2026-04-26 (round 3) | **R3-2**: § 5.2 p_0 calibration protocol fully specified — toggle (`--simulate-rollback-at-tx-50`), sample (full adaptation-144 × 2 seeds = 288 paired runs), estimator (per-problem worst-case across seeds), schedule (Phase B7 mandatory before Phase D), freeze (genesis_payload.toml + Trust Root SHA-256 of calibration jsonl), ceiling (p_0 > 0.10 = ABORT), audit (dual-audit at Phase B → C transition). Closes round-2 tuning-surface concern. | Codex round-2 audit P0-gate-h (partial) |
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:781:| 2026-04-26 (round 4) | **R4-2**: § 5.4 j-RR moved out of inferential family (was mathematically unwinnable: 0.9^54 ≈ 0.00343 > 0.05/44 ≈ 0.00114; even zero regressions failed to reject). Now a descriptive guardrail: point check `RR_j ≤ p_0`, no α correction. Family size shrinks from 4+4k to 4+3k; N_max from 44 to 34. § 1.6 + § 1.7 + § 7 Gate H synced to "3 inferential + 1 guardrail + rollback" framing. | Codex round-3 audit P0-gate-h (partial; mathematical impossibility) |
handover/preregistration/scripts/run_p0_calibration.sh:2:# PPUT-CCL B7-extra — p_0 calibration runner (audit-fixed 2026-04-25).
handover/preregistration/scripts/run_p0_calibration.sh:8:#   - regression_p = 1 iff control SOLVED && treatment UNSOLVED, same (problem, seed)
handover/preregistration/scripts/run_p0_calibration.sh:9:#   - p_0 = sum_p max_seed(regression_p) / 144
handover/preregistration/scripts/run_p0_calibration.sh:20:#     (Codex B3) — p_0 > 0.10 ceiling triggers ABORT
handover/preregistration/scripts/run_p0_calibration.sh:165:# not 124, no Trust Root panic). Other panics indicate boot regression.
handover/preregistration/scripts/run_p0_calibration.sh:209:echo "=== p_0 calibration ==="
handover/preregistration/scripts/run_p0_calibration.sh:363:                # and produce a "valid" p_0=0 that gets frozen into Trust Root.
handover/preregistration/scripts/run_p0_calibration.sh:391:echo "║   p_0 CALIBRATION SUMMARY"
handover/preregistration/scripts/run_p0_calibration.sh:403:# Audit-fix Codex B3: invoke compute_p0 + propagate exit code (p_0 > 0.10
handover/preregistration/scripts/run_p0_calibration.sh:415:echo "[$(date -Is)] Running p_0 estimator (strict-complete mode)..."
handover/preregistration/scripts/run_p0_calibration.sh:427:    echo "✓ p_0 PASSED ceiling. Result: $P0_JSON"
handover/preregistration/scripts/run_p0_calibration.sh:432:    echo "✗ p_0 EXCEEDS ceiling (>0.10) — PREREG § 5.5 ABORT."
handover/preregistration/scripts/compute_p0.py:2:"""PPUT-CCL B7-extra — compute p_0 from calibration jsonl.
handover/preregistration/scripts/compute_p0.py:5:    For each (problem, seed): regression_p_seed = 1 iff control SOLVED
handover/preregistration/scripts/compute_p0.py:7:    Per-problem regression:   max over the 2 seeds (worst case).
handover/preregistration/scripts/compute_p0.py:8:    p_0:                      sum_p regression_p / N_problems.
handover/preregistration/scripts/compute_p0.py:10:Sanity gate: if p_0 > 0.10, ABORT — toggle too aggressive (PREREG § 5.5 ceiling).
handover/preregistration/scripts/compute_p0.py:64:    prior silently-skip behaviour biased p_0 by dropping incomplete pairs.
handover/preregistration/scripts/compute_p0.py:74:                    "runner stamping bug; refuse to compute p_0 on incomplete data"
handover/preregistration/scripts/compute_p0.py:80:                    "runner emitted twice; refuse to compute p_0 on duplicated data"
handover/preregistration/scripts/compute_p0.py:113:            f"got {len(c_problems)}. Refuse to compute p_0 on partial batch."
handover/preregistration/scripts/compute_p0.py:125:    # Per-problem worst-case regression (max over seeds).
handover/preregistration/scripts/compute_p0.py:126:    per_problem_regression: dict[str, int] = defaultdict(int)
handover/preregistration/scripts/compute_p0.py:130:    n_regression_pairs = 0
handover/preregistration/scripts/compute_p0.py:141:        regression = 1 if (cs and not ts) else 0
handover/preregistration/scripts/compute_p0.py:142:        if regression:
handover/preregistration/scripts/compute_p0.py:143:            n_regression_pairs += 1
handover/preregistration/scripts/compute_p0.py:144:        if regression > per_problem_regression[pid]:
handover/preregistration/scripts/compute_p0.py:145:            per_problem_regression[pid] = regression
handover/preregistration/scripts/compute_p0.py:153:    p0 = sum(per_problem_regression.values()) / n_problems
handover/preregistration/scripts/compute_p0.py:160:        "n_regression_pairs": n_regression_pairs,
handover/preregistration/scripts/compute_p0.py:161:        "n_regression_problems_max_seed": sum(per_problem_regression.values()),
handover/preregistration/scripts/compute_p0.py:163:        "p0_ceiling": 0.10,
handover/preregistration/scripts/compute_p0.py:164:        "ceiling_pass": p0 <= 0.10,
handover/preregistration/scripts/compute_p0.py:189:    print(f"\n[freeze] baseline_regression_jsonl_sha256 (control+treatment, in order):")
handover/preregistration/scripts/compute_p0.py:194:            f"\nERROR: p_0 = {result['p0']:.4f} > 0.10 — ABORT per PREREG § 5.5 ceiling.",

exec
/bin/bash -lc "awk 'BEGIN{in_tr=0;c=0} /"'^'"\\[trust_root\\]/{in_tr=1;next} /"'^'"\\[/{in_tr=0} in_tr && /"'^"/{c++} END{print c}'"' genesis_payload.toml" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
31

exec
/bin/bash -lc 'git diff --stat' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
exec
/bin/bash -lc "git ls-files .env --error-unmatch; printf 'exit=%s\\n' "'$?' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
error: pathspec '.env' did not match any file(s) known to git
Did you forget to 'git add'?
exit=1

exec
/bin/bash -lc "test -f .env && printf '.env present\\n' || printf '.env absent\\n'; git check-ignore -v .env || true" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
.env present
.gitignore:2:.env	.env

exec
/bin/bash -lc "nl -ba handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md | sed -n '406,490p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   406	For each artifact j ∈ {1, ..., k} (k ≤ k_max = 10) entering Phase E with `ArtifactState = Quarantined`, **three** primary inferential tests run on heldout-54 (NOT four — j-RR is a descriptive guardrail per § 5.4):
   407	
   408	| ID family | Hypothesis (one-sided) | Primary endpoint (n=54) | Direction |
   409	|---|---|---|---|
   410	| `j-PPUT` | Artifact j increases held-out VPPUT | Sign test on n=54 per-problem signs of `mean(VPPUT_all_p) − mean(VPPUT_minus_j_p)` (mean over 3 seeds; leave-one-out per § 6 E2) | sign > 0 |
   411	| `j-FAR` | Artifact j does not increase FAR | Non-inferiority sign test on n=54 per-problem signs of `FAR_all_p − FAR_minus_j_p` | sign ≤ 0 |
   412	| `j-CPR` | Artifact j does not increase CPR | Non-inferiority sign test on n=54 per-problem signs of `CPR_all_p − CPR_minus_j_p` | sign ≤ 0 |
   413	
   414	`j-RR` is **NOT** in the inferential family — see § 5.4.
   415	
   416	### 5.4 j-RR as descriptive guardrail (round 4 fix per Codex round-3 R4-2)
   417	
   418	**Why this changed**: Round 3 placed j-RR in the inferential family with H0: `regression_rate ≥ p_0 = 0.10`. Codex round-3 noted this is mathematically unwinnable: best case 0/54 regressions gives `p = 0.9^54 ≈ 0.00343`, but smallest Holm threshold at N_max = 44 was 0.05/44 ≈ 0.00114 → fails to reject even with zero regressions. Gate H unreachable.
   419	
   420	**Round 4 fix**: j-RR is reclassified as a **descriptive guardrail** (point estimate check, no α correction, no place in family):
   421	
   422	```
   423	j-RR criterion (point check, NOT inferential):
   424	  r_j = |{p in heldout : SOLVED(p, baseline) = 1
   425	                       AND SOLVED(p, all_artifacts_active) = 0
   426	                       AND SOLVED(p, minus_j) = 1}|
   427	  -- Problems where artifact j SPECIFICALLY caused a regression versus
   428	  -- baseline, attribution by leave-one-out: removing j fixes the regression.
   429	
   430	  RR_j = r_j / 54        (point estimate, NOT a hypothesis test)
   431	
   432	  Pass criterion: RR_j ≤ p_0   where p_0 is calibrated per § 5.5.
   433	```
   434	
   435	**Why descriptive is statistically correct here**: j-RR is a constraint, not a discovery hypothesis. We do not need to argue "RR_j is significantly less than p_0"; we only need RR_j ≤ p_0. A point check is precisely the right tool. No multiplicity correction applies because this is not a test.
   436	
   437	### 5.5 p_0 calibration protocol
   438	
   439	`p_0` (baseline regression rate) — full calibration protocol:
   440	
   441	```
   442	Toggle:        evaluator --mode full         (control)
   443	               evaluator --mode full --simulate-rollback-at-tx-50  (treatment;
   444	                 forces a synthetic mid-run rollback at tx 50, mimicking the
   445	                 worst-case stochastic edge case Phase E artifacts could trigger)
   446	Sample:        FULL adaptation-144 problem set × Boltzmann seeds [31415, 2718]
   447	                 = 288 paired runs.
   448	Estimator:     for each problem p, define regression_p = 1 iff control SOLVED
   449	                 (Lean PASS) AND treatment UNSOLVED for the SAME (problem, seed).
   450	               Per-problem regression: max over the 2 seeds (worst case).
   451	               p_0 = sum_p regression_p / 144.
   452	Schedule:      Phase B7 (mandatory; not deferrable to Phase D). Phase D cannot
   453	               start without p_0 frozen.
   454	Freeze point:  end of Phase B7. Value written to
   455	                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
   456	               with SHA-256 of the calibration jsonl in
   457	                 `[pput_accounting_0].baseline_regression_jsonl_sha256`.
   458	Audit trail:   raw calibration jsonl committed to
   459	                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
   460	               BEFORE Phase D begins. This file is part of Trust Root (§ 1.8) —
   461	               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
   462	Ceiling:       if measured p_0 > 0.10, ABORT — toggle too aggressive or system
   463	               too brittle. NOT an opportunity for j-RR auto-pass.
   464	Dual audit:    p_0 calibration result in Phase B → C audit packet.
   465	```
   466	
   467	This protocol forecloses tuning-surface attacks: every parameter is pre-committed; the calibration runs are observable jsonl; the value enters Trust Root.
   468	
   469	### 5.6 Family total (clean restatement)
   470	
   471	```
   472	inferential_family_size = 4 (Phase C) + 3·k (Phase E)
   473	N_max = 4 + 3·k_max = 4 + 3·10 = 34   (frozen for ALL Holm decisions; see § 9)
   474	```
   475	
   476	j-RR is NOT in the family because it is a point-check guardrail.
   477	
   478	### 5.7 H5 — deterministic gate (NOT in inferential family)
   479	
   480	```
   481	Gate H: count of artifacts j with ArtifactState(j) = Certified ≥ 1
   482	```
   483	
   484	`Certified` requires the conjunction of:
   485	1. **j-PPUT, j-FAR, j-CPR** all reject the null at the family-corrected α level (per § 9; 3 inferential tests),
   486	2. **j-RR ≤ p_0** (point check guardrail, no α correction; per § 5.4),
   487	3. **Rollbackable(j) = 1** (verified by an explicit rollback sub-eval in § 6 E2).
   488	
   489	H5 is a deterministic AND of (3 inferential rejections) + (1 point check) + (1 rollback verification). Reaching Gate H = at least 1 artifact passes all five conditions. H5 itself is not a hypothesis test; no separate p-value.
   490	

exec
/bin/bash -lc "python3 - <<'PY'
import importlib.util
spec=importlib.util.spec_from_file_location('llm_proxy','src/drivers/llm_proxy.py')
mod=importlib.util.module_from_spec(spec)
spec.loader.exec_module(mod)
for model in ['openai/gpt-4o','Qwen/Qwen2.5-7B-Instruct','siliconflow:Qwen/Qwen2.5-7B-Instruct','qwen3-8b','deepseek-v4-flash']:
    print(model, '->', mod.detect_provider(model), 'strip=', mod.strip_provider_prefix(model))
PY" in /home/zephryj/projects/turingosv4
 succeeded in 937ms:
openai/gpt-4o -> siliconflow strip= openai/gpt-4o
Qwen/Qwen2.5-7B-Instruct -> dashscope strip= Qwen/Qwen2.5-7B-Instruct
siliconflow:Qwen/Qwen2.5-7B-Instruct -> siliconflow strip= Qwen/Qwen2.5-7B-Instruct
qwen3-8b -> dashscope strip= qwen3-8b
deepseek-v4-flash -> deepseek strip= deepseek-v4-flash

exec
/bin/bash -lc 'rg -n "VERDICT|Finding|P0|CHALLENGE|VETO|Recommendation|FIX" handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md handover/audits/GEMINI_PHASE_A0_EXIT_AUDIT_2026-04-25.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/audits/GEMINI_PHASE_A0_EXIT_AUDIT_2026-04-25.md:21:**VERDICT**: **CHALLENGE**
handover/audits/GEMINI_PHASE_A0_EXIT_AUDIT_2026-04-25.md:25:**Recommendation**: **FIX-THEN-PROCEED**. The design principles of A0 are sound and address critical gaps in governance. However, the implementation contains several exploitable loopholes and process-noise issues that undermine the stated goal of preventing "silent drift". These flaws must be fixed within the A0 cycle before Phase A engineering atoms can safely begin. A full redesign is not required.
handover/audits/GEMINI_PHASE_A0_EXIT_AUDIT_2026-04-25.md:27:## 2. Detailed Findings
handover/audits/GEMINI_PHASE_A0_EXIT_AUDIT_2026-04-25.md:29:This audit assesses the A0 deliverables against the mandate to create a robust harness. Findings are organized by the specific questions posed.
handover/audits/GEMINI_PHASE_A0_EXIT_AUDIT_2026-04-25.md:36:    **CHALLENGE**. The provided rules do not cover other known areas of potential drift mentioned in the prompt. There are no rules governing `synthetic_short_circuit` field discipline, sanity-checking `AGENT_MODELS` environment variables, or enforcing the value domain of `BUDGET_MODE`. These omissions represent known-unknowns that remain un-gated, contrary to the spirit of A0.
handover/audits/GEMINI_PHASE_A0_EXIT_AUDIT_2026-04-25.md:39:    **CHALLENGE**. The rule in `rules/active/R-014_trust_root_manifest_drift.yaml` uses `file_glob: "*.rs"`. A review of `genesis_payload.toml` shows only a small subset of `.rs` files are in the Trust Root. This implementation will generate a high volume of false-positive warnings, leading to alert fatigue and increasing the probability that a genuine required update is missed. The rule's `check` logic should be narrowed to only fire if the edited file is actually listed in `genesis_payload.toml`.
handover/audits/GEMINI_PHASE_A0_EXIT_AUDIT_2026-04-25.md:42:    **CHALLENGE (High Severity)**. The logic in `.claude/hooks/judge.sh:58` (`[ "$(basename "$FILE_PATH")" = "constitution.md" ]`) is **NOT airtight**. It is vulnerable to a symlink bypass. An agent could create a symlink (`ln -s constitution.md my_other_doc.md`) and then edit `my_other_doc.md`. The `basename` would be `my_other_doc.md`, which would then match the `*.md` skip-list pattern at `judge.sh:75`, silently bypassing the R-018 sudo gate. This is a critical vulnerability that directly contradicts the mandate of C-071 and Art. V.1.1. The script must resolve the file's real path before performing the check.
handover/audits/GEMINI_PHASE_A0_EXIT_AUDIT_2026-04-25.md:45:    **CHALLENGE (High Severity)**. The check in `.claude/hooks/judge.sh:48-56` greps the `COMMAND` variable for `FC-trace:`. As correctly hypothesized, a command like `git commit -F /tmp/msg` would pass this check, as the `COMMAND` string itself does not contain the commit message body. This allows an agent to bypass the FC-first discipline mandated by C-074. The hook must be sophisticated enough to inspect the actual commit message content, not just the command-line arguments.
handover/audits/GEMINI_PHASE_A0_EXIT_AUDIT_2026-04-25.md:52:    **CHALLENGE**. The battery is inconsistent. While some tests like `fc1_n4_tape_constructible_with_time_arrow` are strong witnesses that call methods, others like `fc1_n1_q_state_carrier_present` use `std::any::type_name::<T>()`. This only proves the type *exists*, not that its constitutionally relevant API is intact. This is a weak form of witnessing that could allow for silent drift if, for example, a key method on `TuringBus` were removed but the type itself remained. The weaker witnesses should be strengthened to call the specific functions or methods cited as evidence in `TRACE_MATRIX_v2`.
handover/audits/GEMINI_PHASE_A0_EXIT_AUDIT_2026-04-25.md:103:    **CHALLENGE**. The R-016 rule was introduced in commit `2e7f75a`. The subsequent three commits (`d8950ee`, `2a65339`, `e94e1b9`) were made *after* this rule was notionally active. It is a procedural requirement that these commits contain a valid `FC-trace:`. Verification of the commit logs is required to close this challenge. This is a test of whether the new process was immediately adopted.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:25:**Role**: skeptical adversarial reviewer. Independent of Gemini. Decision rule: PASS → Phase A1-A8 engineering atoms can begin; CHALLENGE → fix in same cycle; VETO → A0 redesign.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:38:- Art. V.1.3: JudgeAI → Veto-AI; output domain {PASS, VETO} only
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:94:- **VERDICT**: PASS / CHALLENGE / VETO
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:96:- **Recommendation**: PROCEED to Phase A engineering atoms (A1-A8) / FIX-THEN-PROCEED / REDESIGN A0
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:98:PASS if: A0 closes the harness-gap claim AND no new P0 defect.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:111:  - "F-2026-04-25-05"  # B7-extra round-1 dual VETO including Trust Root manifest gaps
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:530:#[ignore = "📅 Phase 11+ — Veto-AI runtime not implemented (manual Codex/Gemini dual-audit covers role today; Art. V.1.3 amendment 2026-04-25 narrowed scope to {PASS, VETO})"]
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:688:- `handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md` (Findings A+B)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:689:- `handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md` (Findings C+D)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:740:#     src/main.rs (audit Q2.b VETO — verify_trust_root call site must be
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:742:#     Cargo.lock (audit Q2.e VETO — supply-chain dep-version swap defense)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:871:    重 round 持续到 PASS/PASS. CHALLENGE 可 documented + accepted by user.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:872:    VETO 必须 fix.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:895:title: "Veto-AI 范围窄化 — 输出域 {PASS, VETO}, 不做主观评判"
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:901:  - "C-010 Audit Standard: VETO > CHALLENGE > PASS"
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:906:  - Codex: VETO (caught self-inflicted regression — TRUST_ROOT_TAMPERED 被
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:910:  Codex VETO 是基于"违宪"判定 — round-1 fix 引入新缺陷使 ground-truth
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:918:  (CHALLENGE 不该 VETO) 或 false-negative (该 VETO 的真违宪被淹没在主观
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:936:    输出域 = `{PASS, VETO}` — 不承担其他主观评价.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:946:    VETO: 至少一处违宪 (cite which Article / FC element)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:948:    **NOT allowed**: CHALLENGE-as-third-class, "code quality concern", "style
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:953:    Codex VETO + Gemini PASS = 保守裁决 = VETO (本 session round-2 用例).
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:955:  Rule 3 — CHALLENGE 是 INFORMATIONAL 不是 BLOCKING:
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:956:    Audit 输出 CHALLENGE 提示有改进空间但**不违宪** → ArchitectAI 可选择:
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:958:    本 session round-3 Gemini CHALLENGE (EXIT=0+empty PPUT_RESULT) 选了 (a).
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:966:    或 "Performance might suffer because..." → 不是 VETO 信号. 不算入
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:971:    VETO 锚点.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:977:  - **Cross-ref**: C-073 (ArchitectAI commit authority — 接受 CHALLENGE
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:1055:    人类 review 后可以 (a) 撤回 fix, (b) 接受 CHALLENGE, (c)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:1104:     → round-2 Codex VETO (TRUST_ROOT_TAMPERED 被 silent absorb)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:1108:  4. Round-2 同类 problem_file_missing silent absorb (round-3 CHALLENGE)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:1139:    - dual audit VETO/CHALLENGE finding
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:1199:  1. src/main.rs — verify_trust_root call site (Q2.b VETO)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:1200:  2. Cargo.lock — 依赖锁文件 (Q2.e VETO)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:1317:d0d474e B7-extra round-3 audit + fixes (Codex CHALLENGE / Gemini CHALLENGE)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:1318:584001a B7-extra round-2 audit packet (Codex VETO / Gemini PASS)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:1319:1df1f62 B7-extra round-2 fix: Codex VETO on crash-as-data + 2 minor
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:1323:df77b0a B7-extra pre-batch dual audit: VETO/VETO
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:1496:  Output domain MUST be 2-class {PASS, VETO}. White-list excludes
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:1498:  response to round-2 dual audit divergence (Codex VETO real defect /
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:1884:     4	  - "F-2026-04-25-05"  # B7-extra round-1 dual VETO including Trust Root manifest gaps
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:2169:   221	#[ignore = "📅 Phase 11+ — Veto-AI runtime not implemented (manual Codex/Gemini dual-audit covers role today; Art. V.1.3 amendment 2026-04-25 narrowed scope to {PASS, VETO})"]
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:2260:    42	#     src/main.rs (audit Q2.b VETO — verify_trust_root call site must be
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:2262:    44	#     Cargo.lock (audit Q2.e VETO — supply-chain dep-version swap defense)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:2533:   108	- `handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md` (Findings A+B)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:2534:   109	- `handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md` (Findings C+D)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3053:    42	/// Mid-term audit P0-B fix 2026-04-25: this struct now carries every B1
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3167:   156	    // Note (mid-term audit P0-B fix 2026-04-25): the prior Option versions of
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3239:   228	            // Mid-term audit P0-D fix 2026-04-25: hybrid_v1 was a Paper 1 era
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3249:   238	                       disabled in mid-term audit P0-D fix 2026-04-25. The prior \
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3370:./handover/audits/DUAL_AUDIT_V2_VERDICT_2026-04-24.md:26:Both Phase 9.A and v2 A use identical harness settings (n=8, tx_cap=50, homogeneous algebraic skill); only the DeepSeek model could have changed. Conclusion: **`deepseek-chat` drifted between 2026-04-22 and 2026-04-24**, making `mathd_algebra_246` solvable from seeds that previously failed. This validates v2 Limitation #9 (model drift unverifiable) as a real, observed phenomenon in this experiment.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3372:./handover/audits/run_gemini_b7_extra_round4_audit.py:65:| Q7.a: API drift mitigation | CHALLENGE | 15b87fb: MODEL_SNAPSHOT (active_model@git_sha[:12]) + BUILD_SHA + canary timestamps logged |
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3378:./handover/audits/run_codex_b7_extra_round3_audit.sh:68:| Q7.a: API drift detection | CHALLENGE | 15b87fb: MODEL_SNAPSHOT (active_model@git_sha[:12]) + BUILD_SHA + canary timestamps |
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3393:./handover/audits/run_gemini_b7_extra_round3_audit.py:68:| Q7.a: API drift mitigation | CHALLENGE | 15b87fb: MODEL_SNAPSHOT (active_model@git_sha[:12]) + BUILD_SHA + canary timestamps logged |
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3405:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:79:| Q7.a: API drift detection | CHALLENGE | 15b87fb: MODEL_SNAPSHOT (active_model@git_sha[:12]) + BUILD_SHA + canary timestamps |
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3421:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1001:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3422:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1002:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3456:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3082:   344	    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3457:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3083:   345	    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3465:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3641:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1001:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3466:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3642:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1002:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3467:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3696:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3082:   344	    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3468:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3697:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3083:   345	    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3472:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3807:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:933:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3473:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3808:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:934:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3474:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3833:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2580:   292	    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3475:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3834:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2581:   293	    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3476:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3866:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4404:handover/preregistration/scripts/run_p0_calibration.sh:292:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3477:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3867:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4405:handover/preregistration/scripts/run_p0_calibration.sh:293:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3478:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4141:./handover/preregistration/scripts/run_p0_calibration.sh:344:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3479:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4142:./handover/preregistration/scripts/run_p0_calibration.sh:345:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3493:./handover/audits/run_codex_b7_extra_reaudit.sh:52:| Q7.a: API drift detection | CHALLENGE | 15b87fb: MODEL_SNAPSHOT (active_model@git_sha[:12]) + BUILD_SHA + canary timestamps |
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3517:./handover/audits/run_gemini_b7_extra_reaudit.py:56:| Q7.a: API drift mitigation | CHALLENGE | 15b87fb: MODEL_SNAPSHOT (active_model@git_sha[:12]) + BUILD_SHA + canary timestamps logged |
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3536:./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:17:    -   **Cost**: The cost accumulator is *not* identical. A true 150-tx vetoed loop would involve LLM calls for each proposal, accumulating cost. The short-circuit explicitly prevents this. This is acknowledged in Finding (B) (`B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS...`) and the `synthetic_short_circuit` doc comment (`evaluator.rs:149-151`), which directly contradicts the `rollback_sim.rs` header's claim.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3538:./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:53:-   **(Q4.c) Counter-argument (measurement artifact)**: **PASS**. This is the correct and necessary framing. The thesis describes the intended production behavior of the system. The calibration is a meta-process to measure a parameter *about* the system. It is acceptable for the measurement apparatus to not perfectly mirror the system under test, as long as the deviation is understood and accounted for. Finding (A) and the `synthetic_short_circuit` flag provide this accounting.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3539:./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:60:-   **(Q5.c) Downstream interpretation**: **PASS**. The `synthetic_short_circuit` flag and its associated doc comment (`evaluator.rs:149-151`) are sufficient mitigation for Finding (B) (cost asymmetry). Any downstream tool that misinterprets the cost of these rows is ignoring the explicit warning provided in the data schema.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3561:./handover/audits/DUAL_AUDIT_PAPER1_VERDICT_2026-04-23.md:45:| P1-11 | Build provenance (commits + model snapshot) | Tag a single release commit for the paper; specify deepseek-chat snapshot date/version. § 1.3 says "snapshot referenced in § 5" but § 5 has no such reference — dangling pointer. | Codex REPRO-1 + Gemini REPRO-1/3 |
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3567:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:63:| Q7.a: API drift detection | CHALLENGE | 15b87fb: MODEL_SNAPSHOT (active_model@git_sha[:12]) + BUILD_SHA + canary timestamps |
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3583:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:933:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3584:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:934:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3615:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2580:   292	    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3616:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2581:   293	    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3630:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4404:handover/preregistration/scripts/run_p0_calibration.sh:292:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3631:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4405:handover/preregistration/scripts/run_p0_calibration.sh:293:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3635:./handover/audits/run_codex_b7_extra_round4_audit.sh:65:| Q7.a: API drift detection | CHALLENGE | 15b87fb: MODEL_SNAPSHOT (active_model@git_sha[:12]) + BUILD_SHA + canary timestamps |
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3708:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3825:./handover/audits/EXT_CODEX_2026-04-22.md:8:| Q2. F-20-05 three-layer blockade completeness | VETO | `/home/zephryj/projects/turingosv4/src/bus.rs:174-180`; `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/lean4_oracle.rs:112-116`; `/home/zephryj/projects/turingosv4/routines/daily_drift.yaml:19-23` | The three claimed layers exist, but completeness fails because `append_oracle_accepted` is a public blessed-write API whose oracle provenance is asserted by the caller, not enforced by the bus. |
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3748:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:76:| Q7.a: API drift detection | CHALLENGE | 15b87fb: MODEL_SNAPSHOT (active_model@git_sha[:12]) + BUILD_SHA + canary timestamps |
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3764:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1060:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3765:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1061:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3792:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2992:   406	    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3793:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2993:   407	    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3805:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3610:handover/preregistration/scripts/run_p0_calibration.sh:406:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3806:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3611:handover/preregistration/scripts/run_p0_calibration.sh:407:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3827:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:4355:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1001:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3828:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:4356:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1002:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3831:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:4449:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3082:   344	    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3832:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:4450:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3083:   345	    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3834:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:4509:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3641:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1001:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3835:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:4510:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3642:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:1002:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3836:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:4538:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3696:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3082:   344	    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3837:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:4539:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3697:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3083:   345	    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3841:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:4593:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3807:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:933:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3842:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:4594:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3808:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:934:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3843:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:4605:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3833:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2580:   292	    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3844:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:4606:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3834:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2581:   293	    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3845:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:4627:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3866:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4404:handover/preregistration/scripts/run_p0_calibration.sh:292:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3846:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:4628:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:3867:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4405:handover/preregistration/scripts/run_p0_calibration.sh:293:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3847:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:4804:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4141:./handover/preregistration/scripts/run_p0_calibration.sh:344:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3848:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:4805:./handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md:4142:./handover/preregistration/scripts/run_p0_calibration.sh:345:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3854:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:5006:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:933:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3855:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:5007:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:934:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3858:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:5069:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2580:   292	    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3859:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:5070:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:2581:   293	    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3860:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:5131:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4404:handover/preregistration/scripts/run_p0_calibration.sh:292:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3861:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:5132:./handover/audits/CODEX_B7_EXTRA_REAUDIT_2026-04-25.md:4405:handover/preregistration/scripts/run_p0_calibration.sh:293:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3865:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:5241:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1060:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3866:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:5242:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:1061:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3869:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:5337:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2992:   406	    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3870:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:5338:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:2993:   407	    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3879:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:5374:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3610:handover/preregistration/scripts/run_p0_calibration.sh:406:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3880:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:5375:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:3611:handover/preregistration/scripts/run_p0_calibration.sh:407:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3902:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:5885:./handover/preregistration/scripts/run_p0_calibration.sh:406:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3903:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:5886:./handover/preregistration/scripts/run_p0_calibration.sh:407:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3914:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:6881:     head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:3915:./handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md:6882:     head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:4090:./handover/audits/EXT_CODEX_2026-04-22.md:8:| Q2. F-20-05 three-layer blockade completeness | VETO | `/home/zephryj/projects/turingosv4/src/bus.rs:174-180`; `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/lean4_oracle.rs:112-116`; `/home/zephryj/projects/turingosv4/routines/daily_drift.yaml:19-23` | The three claimed layers exist, but completeness fails because `append_oracle_accepted` is a public blessed-write API whose oracle provenance is asserted by the caller, not enforced by the bus. |
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:4099:./handover/audits/GEMINI_PHASE_A0_EXIT_AUDIT_2026-04-25.md:36:    **CHALLENGE**. The provided rules do not cover other known areas of potential drift mentioned in the prompt. There are no rules governing `synthetic_short_circuit` field discipline, sanity-checking `AGENT_MODELS` environment variables, or enforcing the value domain of `BUDGET_MODE`. These omissions represent known-unknowns that remain un-gated, contrary to the spirit of A0.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:4234:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:1424:The calibration jsonl rows are **per-run ground-truth-validated** (`verified` field comes from Lean4Oracle). Findings C+D are about **intra-run** ground-truth event granularity (per-tx, per-proposal) — needed by Phase D ArchitectAI but not by Phase B's measurement gate.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:4272:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3408:    94	The calibration jsonl rows are **per-run ground-truth-validated** (`verified` field comes from Lean4Oracle). Findings C+D are about **intra-run** ground-truth event granularity (per-tx, per-proposal) — needed by Phase D ArchitectAI but not by Phase B's measurement gate.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:4308:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3779:./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:17:    -   **Cost**: The cost accumulator is *not* identical. A true 150-tx vetoed loop would involve LLM calls for each proposal, accumulating cost. The short-circuit explicitly prevents this. This is acknowledged in Finding (B) (`B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS...`) and the `synthetic_short_circuit` doc comment (`evaluator.rs:149-151`), which directly contradicts the `rollback_sim.rs` header's claim.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:4310:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3786:./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:53:-   **(Q4.c) Counter-argument (measurement artifact)**: **PASS**. This is the correct and necessary framing. The thesis describes the intended production behavior of the system. The calibration is a meta-process to measure a parameter *about* the system. It is acceptable for the measurement apparatus to not perfectly mirror the system under test, as long as the deviation is understood and accounted for. Finding (A) and the `synthetic_short_circuit` flag provide this accounting.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:4311:./handover/audits/CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:3787:./handover/audits/GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md:60:-   **(Q5.c) Downstream interpretation**: **PASS**. The `synthetic_short_circuit` flag and its associated doc comment (`evaluator.rs:149-151`) are sufficient mitigation for Finding (B) (cost asymmetry). Any downstream tool that misinterprets the cost of these rows is ignoring the explicit warning provided in the data schema.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:4409:./handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md:94:The calibration jsonl rows are **per-run ground-truth-validated** (`verified` field comes from Lean4Oracle). Findings C+D are about **intra-run** ground-truth event granularity (per-tx, per-proposal) — needed by Phase D ArchitectAI but not by Phase B's measurement gate.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:4823:./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:93:  - **P0-C — B3 first-read placement undercounts T_i**: `mark_first_read` fired AFTER prompt construction in both run_oneshot and run_swarm; conformance test was relaxed `≥7100ms → ≥7000ms` to accommodate, which itself was a tell of spec divergence.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:4824:./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:100:  - **P0-C**: moved `wc.mark_first_read()` BEFORE prompt construction in both run_oneshot (before `let prompt = format!(...)`) and run_swarm (top of for-loop body, before chain/skill/board build). Tightened conformance test from `7000-7100ms` slack to strict `≥7100ms` per plan B3 spec.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:4858:./handover/ai-direct/E1_EMERGENCE_VERDICT_2026-04-23.md:5:**Design**: paired A/B, same BOLTZMANN_SEED=141421, same MAX_TRANSACTIONS=50, same 10-problem hard set (FAIL in both seed 31415 + seed 2718 Phase 9.A), same chat model deepseek-chat.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:4859:./handover/ai-direct/E1_EMERGENCE_VERDICT_2026-04-23.md:54:- Same model (`deepseek-chat`, no fine-tuning)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:4955:./handover/ai-direct/E1_FINAL_VERDICT_3SEEDS_2026-04-23.md:154:2. **All 3 seeds use same deepseek-chat snapshot**. Model-independence test (GPT-4, Claude Opus, Gemini) is out of scope for Paper 1 — a "next steps" section.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:4983:./handover/ai-direct/PHASE_2_5C_VERDICT_2026-04-22.md:7:- Model: `deepseek-chat` (both conditions), `temperature=0.2`, `max_tokens=8000`
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5156:./handover/preregistration/scripts/run_p0_calibration.sh:406:    head -1 "${OUT_PREFIX}_control.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' control:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5157:./handover/preregistration/scripts/run_p0_calibration.sh:407:    head -1 "${OUT_PREFIX}_treatment.jsonl" 2>/dev/null | python3 -c "import sys,json; r=json.loads(sys.stdin.read()); print(' treatment:', {k: r.get(k) for k in ('tx_count','solved','progress','synthetic_short_circuit','synthetic_timeout_or_crash')})" || true
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5197:    13	# Audit-fix 2026-04-25 (dual VETO):
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5280:    78	    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5281:    78	    OUT_PREFIX="$LOG_DIR/p0_smoke_${TIMESTAMP}"
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5284:    80	    OUT_PREFIX="$LOG_DIR/p0_smoke_hard_${TIMESTAMP}"
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5285:    80	    OUT_PREFIX="$LOG_DIR/p0_smoke_hard_${TIMESTAMP}"
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5288:    82	    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5289:    82	    OUT_PREFIX="$LOG_DIR/p0_calibration_${TIMESTAMP}"
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5518:   729	- Conservative merge: VETO > CHALLENGE > PASS (feedback_dual_audit_conflict)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5733:    60	    重 round 持续到 PASS/PASS. CHALLENGE 可 documented + accepted by user.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5734:    61	    VETO 必须 fix.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5819:    70	    人类 review 后可以 (a) 撤回 fix, (b) 接受 CHALLENGE, (c)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5843:     2	title: "Veto-AI 范围窄化 — 输出域 {PASS, VETO}, 不做主观评判"
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5849:     8	  - "C-010 Audit Standard: VETO > CHALLENGE > PASS"
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5854:    13	  - Codex: VETO (caught self-inflicted regression — TRUST_ROOT_TAMPERED 被
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5858:    17	  Codex VETO 是基于"违宪"判定 — round-1 fix 引入新缺陷使 ground-truth
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5866:    25	  (CHALLENGE 不该 VETO) 或 false-negative (该 VETO 的真违宪被淹没在主观
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5884:    43	    输出域 = `{PASS, VETO}` — 不承担其他主观评价.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5894:    53	    VETO: 至少一处违宪 (cite which Article / FC element)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5896:    55	    **NOT allowed**: CHALLENGE-as-third-class, "code quality concern", "style
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5901:    60	    Codex VETO + Gemini PASS = 保守裁决 = VETO (本 session round-2 用例).
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5903:    62	  Rule 3 — CHALLENGE 是 INFORMATIONAL 不是 BLOCKING:
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5904:    63	    Audit 输出 CHALLENGE 提示有改进空间但**不违宪** → ArchitectAI 可选择:
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5906:    65	    本 session round-3 Gemini CHALLENGE (EXIT=0+empty PPUT_RESULT) 选了 (a).
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5914:    73	    或 "Performance might suffer because..." → 不是 VETO 信号. 不算入
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5919:    78	    VETO 锚点.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5925:    84	  - **Cross-ref**: C-073 (ArchitectAI commit authority — 接受 CHALLENGE
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5952:    25	     → round-2 Codex VETO (TRUST_ROOT_TAMPERED 被 silent absorb)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5956:    29	  4. Round-2 同类 problem_file_missing silent absorb (round-3 CHALLENGE)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:5987:    60	    - dual audit VETO/CHALLENGE finding
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:6041:    17	  1. src/main.rs — verify_trust_root call site (Q2.b VETO)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:6042:    18	  2. Cargo.lock — 依赖锁文件 (Q2.e VETO)
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:6216:   635	也就是说，它只负责（偏）客观的违宪判定 —— 输出域 = `{PASS, VETO}` —— 不承担其他主观评价。任何把 Veto-AI 用于"代码评审"或"质量审计"的尝试都属于范畴越界，应由独立的非 Veto-AI 审计者承担。
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:6296:    19	- 所有 merge / phase 决策双外审（Codex + Gemini）；VETO > CHALLENGE > PASS
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:6420:test fc3_n32_veto_ai_runtime ... ignored, 📅 Phase 11+ — Veto-AI runtime not implemented (manual Codex/Gemini dual-audit covers role today; Art. V.1.3 amendment 2026-04-25 narrowed scope to {PASS, VETO})
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:6634:   576	        // PPUT-CCL B3 (mid-term audit P0-C fix 2026-04-25): open the wall-clock
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:7508:handover/alignment/CODE_CANDIDATES_2026-04-22.md:52:| FC3-N43 | `JudgeAI --veto→ ArchitectAI` | `—` | `constitution.md:713`<br>`CLAUDE.md:19` | `N`<br>`N` | `—` | ❌ MISSING: `VETO > CHALLENGE > PASS` is documented policy, not a runtime Rust veto channel between two agents. |
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:7769:.claude/worktrees/agent-a89c20c4/handover/ai-direct/DRIFT_AUDIT_20260419.md:161:- PLAN_PHASE3 cites Art. IV mermaid (verify-then-write loop) correctly — the plan is proposing to FIX code to match constitution. Correct use of citation.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:7804:.claude/worktrees/agent-a89c20c4/scripts/constitutional_check.sh:221:    echo "  VERDICT: ⛔ FAIL — $VIOLATIONS constitutional violation(s)"
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:8085:.claude/worktrees/agent-ae34e1cb/handover/ai-direct/DRIFT_AUDIT_20260419.md:161:- PLAN_PHASE3 cites Art. IV mermaid (verify-then-write loop) correctly — the plan is proposing to FIX code to match constitution. Correct use of citation.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:8125:.claude/worktrees/agent-ae34e1cb/scripts/constitutional_check.sh:221:    echo "  VERDICT: ⛔ FAIL — $VIOLATIONS constitutional violation(s)"
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:8268:.claude/worktrees/phase-8a-snapshot/scripts/constitutional_check.sh:221:    echo "  VERDICT: ⛔ FAIL — $VIOLATIONS constitutional violation(s)"
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:8273:.claude/worktrees/phase-8a-snapshot/handover/audits/EXT_CODEX_PHASE_8_BATCH_2026-04-22.md:19:| 5 | Trojan / constitutional debt | CHALLENGE | public surface 过宽（OracleReceipt 可自由构造 / Predicate trait 仍 unused），min_count=3 默认未跨 swarm 尺寸辩护 |
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:8294:.claude/worktrees/phase-8a-snapshot/handover/ai-direct/PHASE_2_5C_VERDICT_2026-04-22.md:82:**Constitutional implication**: this is not harness noise — it's chat's preferred oneshot tactic being correctly blocked. Options:
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:8295:.claude/worktrees/phase-8a-snapshot/handover/ai-direct/PHASE_2_5C_VERDICT_2026-04-22.md:84:- **Soft prompt hint**: "Use norm_num/omega/linarith/Decidable.decide; do NOT use bare `decide` or `native_decide` — they are forbidden per constitutional rule." **Risk**: C-031/C-034 hierarchy (机制 > 参数 > 提示); also chat might over-avoid legitimate `decide` uses.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:8304:.claude/worktrees/phase-8a-snapshot/handover/ai-direct/DRIFT_AUDIT_20260419.md:161:- PLAN_PHASE3 cites Art. IV mermaid (verify-then-write loop) correctly — the plan is proposing to FIX code to match constitution. Correct use of citation.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:8307:.claude/worktrees/phase-8a-snapshot/handover/ai-direct/PAPER_1_OUTLINE_2026-04-22.md:5:**Core thesis (2026-04-22 revision post Codex CHALLENGE)**: **First implementation of a Turing-machine-topology constitutional substrate for LLM swarms, with cryptographic capability tokens and runtime-verified three-process governance. We empirically test N Hayek-inspired market mechanisms on this substrate and report observed effects on PPUT (Progress Per Unit Time).**
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:8365:.claude/worktrees/phase-8a-snapshot/handover/ai-direct/ROADMAP_LAUNCH_2026-04-21.md:107:4. **P1-4** ArchitectAI + JudgeAI 真正实体化 (Art. V.1.2/1.3) — 从 constitutional debt §6 "Art. V 零案例覆盖" 落地。**依赖 P0-6 争议通道提供 logs feedback。**
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:8505:.claude/worktrees/phase-8a-snapshot/handover/ai-direct/ART_V_MIN_DESIGN_2026-04-22.md:133:- Any VETO → PR label `constitutional-veto` + CI fail
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:8512:.claude/worktrees/phase-8a-snapshot/handover/ai-direct/ART_V_MIN_DESIGN_2026-04-22.md:178:# 5. CI job (constitutional_gate.yml) sees two VETOs → block merge.
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:8622:**Findings**
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:8646:**VERDICT**: CHALLENGE  
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:8648:**Recommendation**: FIX-THEN-PROCEED
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:8653:**Findings**
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:8677:**VERDICT**: CHALLENGE  
handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md:8679:**Recommendation**: FIX-THEN-PROCEED

exec
/bin/bash -lc "nl -ba handover/audits/CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md | sed -n '1,160p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# Codex Phase A0 Exit Audit (Harness Modernization)
     2	**Date**: 2026-04-25
     3	**Commits**: 2e7f75a / d8950ee / 2a65339 / e94e1b9
     4	**Test baseline**: 204 PASS + 29 ignored
     5	**Trust Root**: 24-entry manifest verifies clean
     6	**Prompt size**: 64284 chars
     7	
     8	---
     9	
    10	Reading prompt from stdin...
    11	OpenAI Codex v0.124.0 (research preview)
    12	--------
    13	workdir: /home/zephryj/projects/turingosv4
    14	model: gpt-5.5
    15	provider: openai
    16	approval: never
    17	sandbox: danger-full-access
    18	reasoning effort: xhigh
    19	reasoning summaries: none
    20	session id: 019dc7bb-2ba2-72a1-a5ec-0a0fb2599e9a
    21	--------
    22	user
    23	# Codex Phase A0 Exit Audit — Harness Modernization
    24	
    25	**Role**: skeptical adversarial reviewer. Independent of Gemini. Decision rule: PASS → Phase A1-A8 engineering atoms can begin; CHALLENGE → fix in same cycle; VETO → A0 redesign.
    26	
    27	**Mandate**: Phase A0 is the harness modernization that PRECEDES Phase A engineering work. Per the auto-research execution plan: "harness 不修复, Phase A 之后的 atoms 也会有 silent drift". A0 must resolve the harness gaps before atoms A1-A8 (engineering: swarm_N=1, AGENT_MODELS, decomposed metrics, budgets, FC tracing, SiliconFlow smoke, Phase A→B exit).
    28	
    29	**A0 deliverables (4 commits)**:
    30	- `2e7f75a` A0a: 4 new rules R-014/R-015/R-018/R-019 + judge.sh constitution-special-case + R-016 fc_trace_in_commit hook
    31	- `d8950ee` A0b: tests/fc_alignment_conformance.rs witness battery (17 PASS + 9 ignored stubs)
    32	- `2a65339` A0c: 5 new cases C-071..C-075 (constitution amendment process / Veto-AI scope / ArchitectAI commit authority / FC-first / DO-178C tool qualification)
    33	- `e94e1b9` A0d: TRACE_MATRIX_v2 + Trust Root manifest 20 → 24 entries (rules/MANIFEST.sha256 + rules/engine.py + .claude/hooks/judge.sh + tests/fc_alignment_conformance.rs added)
    34	
    35	**Constitutional context** (Art. V amendment 2026-04-25, commit c061450 — sudo-authorized):
    36	- Art. V.1.1: sudo applies *only* to constitution.md
    37	- Art. V.1.2: ArchitectAI has commit authority on non-constitution files (this is the authority under which A0a-d landed without sudo)
    38	- Art. V.1.3: JudgeAI → Veto-AI; output domain {PASS, VETO} only
    39	
    40	**Verification done** (by ArchitectAI):
    41	- 4 smoke tests on judge.sh confirm: R-018 BLOCKS constitution.md edit / R-016 WARNS on git commit without FC-trace / silent pass on FC-traced commit / non-constitution .md still skipped
    42	- cargo test --workspace: 204 passed (was 187, +17 fc_alignment_conformance) + 29 ignored (was 20, +9 ignored stubs)
    43	- boot::tests::verify_trust_root_passes_on_intact_repo: PASS at 24-entry manifest
    44	
    45	## Specific questions
    46	
    47	### Q1 — Rule completeness for B7+ surface
    48	4 new rules cover: Trust Root manifest drift (R-014 warn), TRACE_MATRIX pub-symbol freshness (R-015 warn), constitution.md sudo gate (R-018 BLOCK), model-snapshot canonical labeling (R-019 warn). Plus R-016 fc_trace_in_commit (in judge.sh, not engine.py rule).
    49	
    50	Vulnerabilities:
    51	- (Q1.a) Are there OTHER B7+ surfaces still uncovered by rules? E.g., synthetic_short_circuit field ground-truth-feedback discipline (Q7.b round-2 lesson), AGENT_MODELS env var sanity, BUDGET_MODE env values, etc.?
    52	- (Q1.b) R-014 fires on EVERY *.rs edit (file_glob "*.rs"). Is the constant warn noise acceptable, or should it scope-narrow (e.g., file_glob covering only the 12 .rs files in current Trust Root, plus a per-rule meta about which manifest entries the file may map to)?
    53	- (Q1.c) R-018 BLOCK uses file_glob "constitution.md" + check.pattern ".*". It depends on judge.sh special-case BEFORE the *.md skip-list. Is the path-routing logic in judge.sh (lines 50-67) airtight, or could a creative path string (e.g., relative path with `./`, symlink) bypass?
    54	- (Q1.d) R-016 fc_trace_in_commit greps the COMMAND for `FC-trace:`. If user uses `git commit -F /tmp/msg` instead of `-m "..."`, the grep misses the file content. Is this a known limitation worth fixing now or post-A0?
    55	
    56	### Q2 — Conformance battery coverage and integrity
    57	17 witnesses + 9 ignored stubs. Each witness imports an FC-anchored symbol and references it.
    58	
    59	Vulnerabilities:
    60	- (Q2.a) Are the 17 witnesses TRULY witnesses (i.e., would fail to compile if the symbol is removed/renamed) or are some only "type_name::<T>()" calls that wouldn't catch behavioral regressions?
    61	- (Q2.b) The 9 ignored stubs each include `panic!()` body — they fire if `--ignored` flag is passed. Is this the right pattern, or should they be `#[ignore]` with `unimplemented!()` or empty body?
    62	- (Q2.c) Cross-crate symbols (Lean4Oracle in minif2f_v4) need a SECOND fc_alignment_conformance.rs in `experiments/minif2f_v4/tests/`. A0b ignored this. Is that an acceptable deferral or should A0e cover it?
    63	- (Q2.d) Binary-only symbols (run_swarm/run_oneshot in evaluator.rs) are ignored with rationale. Should A1-A8 atoms refactor evaluator to expose these as lib functions for test access?
    64	
    65	### Q3 — Case-law sediment quality
    66	5 cases C-071..C-075 cover the major architectural decisions of the 2026-04-25 session. Each follows the existing schema (incident / facts / ruling / precedent / cross-ref).
    67	
    68	Vulnerabilities:
    69	- (Q3.a) Are the cases TRUE precedents (binding for future similar incidents) or just session-summary docs? CLAUDE.md "Common Law" framing: "宪法高度压缩, 具体裁决查 cases/C-xxx.yaml". Do C-071..C-075 stand as standalone rulings without needing this session's chat history?
    70	- (Q3.b) C-073 (ArchitectAI commit authority) declares scope "ALL files EXCEPT constitution.md" — including PREREG amendments, Trust Root manifest, governance instrumentation. Is this correctly aligned with Art. V.1.2 amendment text, or does it overreach?
    71	- (Q3.c) C-074 (FC-first) overlaps significantly with memory feedback_fc_first_problem_handling.md. Is the duplication acceptable (memory = AI's runtime guide; case = constitutional precedent for future reference)?
    72	- (Q3.d) Cross-references between cases (C-071 ↔ C-073, etc.) form a DAG. Any circular reasoning hazard?
    73	
    74	### Q4 — Trust Root manifest expansion 20 → 24
    75	4 new entries: rules/MANIFEST.sha256 + rules/engine.py + .claude/hooks/judge.sh + tests/fc_alignment_conformance.rs.
    76	
    77	Vulnerabilities:
    78	- (Q4.a) Is the inclusion of rules/engine.py sufficient, or should rules/SCHEMA.yaml also be in Trust Root (defines rule format)?
    79	- (Q4.b) `.claude/hooks/judge.sh` is included; what about the other hooks (`build-check.sh` PostToolUse + `session-end.sh` Stop hook)? They run on every Edit/Write/Stop respectively. If they're tampered, is the gate weaker?
    80	- (Q4.c) `tests/fc_alignment_conformance.rs` is included. Should ALL tests be in Trust Root (they're the witness battery for behavioral correctness)? Or only the FC-conformance one?
    81	- (Q4.d) Trust Root self-reference: TRACE_MATRIX_v?.md docs themselves are NOT in Trust Root. v2 § 5 records this as future-deferred. Is this acceptable for now or a real gap?
    82	
    83	### Q5 — Constitutional alignment (meta-check)
    84	Per case C-073, A0a-d landed under ArchitectAI commit authority (no sudo). Per case C-072, Veto-AI proxy is the dual-audit pipeline (this audit).
    85	
    86	Vulnerabilities:
    87	- (Q5.a) Did A0a-d touch constitution.md? Trace each commit's file changes — verify no constitution.md modification.
    88	- (Q5.b) Did each commit message contain `FC-trace:`? Per R-016, missing FC-trace = WARN. Verify all 4 A0 commits have FC-trace.
    89	- (Q5.c) Phase A0 introduced harness that ENFORCES governance going forward. Is there a regression test that asserts A0 itself can't be silently rolled back? (E.g., if attacker reverts judge.sh change, do tests fail?)
    90	
    91	---
    92	
    93	End with:
    94	- **VERDICT**: PASS / CHALLENGE / VETO
    95	- **Conviction**: low / medium / high
    96	- **Recommendation**: PROCEED to Phase A engineering atoms (A1-A8) / FIX-THEN-PROCEED / REDESIGN A0
    97	
    98	PASS if: A0 closes the harness-gap claim AND no new P0 defect.
    99	
   100	---
   101	
   102	# Appended source files
   103	
   104	
   105	## rules/active/R-014_trust_root_manifest_drift.yaml (NEW)
   106	
   107	```yaml
   108	id: "R-014"
   109	name: "trust_root_manifest_drift_warn"
   110	source_incidents:
   111	  - "F-2026-04-25-05"  # B7-extra round-1 dual VETO including Trust Root manifest gaps
   112	fc_trace: "FC3-S3 (readonly subgraph) — manifest consistency"
   113	axiom: "PREREG § 1.8 + Art. V.1.1 amendment — Trust Root frozen at boot; any tracked-file edit requires SHA-256 recompute + genesis_payload.toml entry update, otherwise boot::verify_trust_root will panic at next start"
   114	trigger: "pre_edit"
   115	check:
   116	  type: "grep"
   117	  pattern: ".*"
   118	file_glob: "*.rs"
   119	enforcement: "warn"
   120	message: "REMINDER (R-014 / FC3-S3): if this file is in genesis_payload.toml [trust_root] (20 manifest entries: src/{kernel,wal,bus,main,boot}.rs / src/drivers/llm_http.rs / src/sdk/prompt_guard.rs / experiments/minif2f_v4/src/{lean4_oracle,cost_aggregator,wall_clock,post_hoc_verifier,jsonl_schema,rollback_sim}.rs / experiments/minif2f_v4/src/bin/evaluator.rs), you MUST: (1) recompute sha256sum after edit, (2) update the entry in genesis_payload.toml [trust_root], (3) cargo test boot::tests::verify_trust_root_passes_on_intact_repo to verify. Otherwise next process boot will panic with TRUST_ROOT_TAMPERED."
   121	stats:
   122	  times_triggered: 0
   123	  last_triggered: ""
   124	
   125	```
   126	
   127	## rules/active/R-015_trace_matrix_pub_symbol.yaml (NEW)
   128	
   129	```yaml
   130	id: "R-015"
   131	name: "trace_matrix_pub_symbol_warn"
   132	source_incidents:
   133	  - "F-2026-04-25-04"  # B7 alignment retroactive fix — pub symbols shipped without TRACE_MATRIX backlink
   134	fc_trace: "CLAUDE.md Alignment Standard — every src/ pub symbol must map to FC1/FC2/FC3 element"
   135	axiom: "Every pub fn / struct / enum / trait / const in src/ must carry /// TRACE_MATRIX FCx-Nx: <role> doc-comment OR be filed as orphan with explicit cases/Cxxx or PREREG-§n.m justification in TRACE_MATRIX_v?.md"
   136	trigger: "pre_edit"
   137	check:
   138	  type: "grep"
   139	  pattern: "pub (fn|struct|enum|trait|const|mod) "
   140	file_glob: "*.rs"
   141	enforcement: "warn"
   142	message: "REMINDER (R-015 / Alignment Standard): edit touches a pub symbol in a Rust file. New OR modified pub items MUST carry `/// TRACE_MATRIX FC?-N?: <role>` doc-comment AND have an entry in handover/alignment/TRACE_MATRIX_v?.md (current: v1_2026-04-25). If this is genuinely orphan, file under TRACE_MATRIX § 3 with explicit Constitutional Justification (cases/Cxxx or PREREG-§n.m). Untraced pub symbols cause silent constitutional drift."
   143	stats:
   144	  times_triggered: 0
   145	  last_triggered: ""
   146	
   147	```
   148	
   149	## rules/active/R-018_constitution_amendment_sudo.yaml (NEW)
   150	
   151	```yaml
   152	id: "R-018"
   153	name: "constitution_amendment_sudo"
   154	source_incidents:
   155	  - "F-2026-04-25-07"  # constitution amendment 2026-04-25 (sudo authorized)
   156	fc_trace: "Art. V.1.1 amendment — sudo applies *only* to constitution.md"
   157	axiom: "Constitution amendments require explicit human-architect sudo authorization. Per Art. V.1.1 + V.3 amendment log: every constitution.md modification must be (1) explicitly authorized by user in chat, (2) logged in V.3 amendment table with date / trigger / section / summary, (3) followed by genesis_payload.toml constitution.md SHA-256 recompute. ArchitectAI MUST NOT silently amend."
   158	trigger: "pre_edit"
   159	check:
   160	  type: "grep"

exec
/bin/bash -lc "nl -ba handover/audits/GEMINI_PHASE_A0_EXIT_AUDIT_2026-04-25.md | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# Gemini Phase A0 Exit Audit (Harness Modernization)
     2	**Date**: 2026-04-25
     3	**Commits**: 2e7f75a / d8950ee / 2a65339 / e94e1b9
     4	**Test baseline**: 204 PASS + 29 ignored
     5	**Trust Root**: 24-entry manifest verifies clean
     6	**Elapsed**: 47.3s
     7	**Prompt size**: 56,868 chars
     8	
     9	---
    10	
    11	# Phase A0 Exit Audit Report
    12	
    13	**To**: Codex Governance Board
    14	**From**: Independent Reviewer
    15	**Subject**: Verdict on Gemini Phase A0 Exit Audit — Harness Modernization
    16	
    17	---
    18	
    19	## 1. Executive Summary
    20	
    21	**VERDICT**: **CHALLENGE**
    22	
    23	**Conviction**: **High**
    24	
    25	**Recommendation**: **FIX-THEN-PROCEED**. The design principles of A0 are sound and address critical gaps in governance. However, the implementation contains several exploitable loopholes and process-noise issues that undermine the stated goal of preventing "silent drift". These flaws must be fixed within the A0 cycle before Phase A engineering atoms can safely begin. A full redesign is not required.
    26	
    27	## 2. Detailed Findings
    28	
    29	This audit assesses the A0 deliverables against the mandate to create a robust harness. Findings are organized by the specific questions posed.
    30	
    31	### Q1 — Rule completeness
    32	
    33	The new rules are a significant step forward, but the implementation and scope contain critical gaps.
    34	
    35	-   **(Q1.a) Other B7+ surfaces still uncovered?**
    36	    **CHALLENGE**. The provided rules do not cover other known areas of potential drift mentioned in the prompt. There are no rules governing `synthetic_short_circuit` field discipline, sanity-checking `AGENT_MODELS` environment variables, or enforcing the value domain of `BUDGET_MODE`. These omissions represent known-unknowns that remain un-gated, contrary to the spirit of A0.
    37	
    38	-   **(Q1.b) R-014 fires on EVERY *.rs edit. Acceptable noise or scope-narrow?**
    39	    **CHALLENGE**. The rule in `rules/active/R-014_trust_root_manifest_drift.yaml` uses `file_glob: "*.rs"`. A review of `genesis_payload.toml` shows only a small subset of `.rs` files are in the Trust Root. This implementation will generate a high volume of false-positive warnings, leading to alert fatigue and increasing the probability that a genuine required update is missed. The rule's `check` logic should be narrowed to only fire if the edited file is actually listed in `genesis_payload.toml`.
    40	
    41	-   **(Q1.c) R-018 BLOCK path-routing logic airtight?**
    42	    **CHALLENGE (High Severity)**. The logic in `.claude/hooks/judge.sh:58` (`[ "$(basename "$FILE_PATH")" = "constitution.md" ]`) is **NOT airtight**. It is vulnerable to a symlink bypass. An agent could create a symlink (`ln -s constitution.md my_other_doc.md`) and then edit `my_other_doc.md`. The `basename` would be `my_other_doc.md`, which would then match the `*.md` skip-list pattern at `judge.sh:75`, silently bypassing the R-018 sudo gate. This is a critical vulnerability that directly contradicts the mandate of C-071 and Art. V.1.1. The script must resolve the file's real path before performing the check.
    43	
    44	-   **(Q1.d) R-016 `git commit -F` bypass?**
    45	    **CHALLENGE (High Severity)**. The check in `.claude/hooks/judge.sh:48-56` greps the `COMMAND` variable for `FC-trace:`. As correctly hypothesized, a command like `git commit -F /tmp/msg` would pass this check, as the `COMMAND` string itself does not contain the commit message body. This allows an agent to bypass the FC-first discipline mandated by C-074. The hook must be sophisticated enough to inspect the actual commit message content, not just the command-line arguments.
    46	
    47	### Q2 — Conformance battery integrity
    48	
    49	The battery is a crucial addition, but its implementation could be hardened.
    50	
    51	-   **(Q2.a) Are witnesses TRULY witnesses?**
    52	    **CHALLENGE**. The battery is inconsistent. While some tests like `fc1_n4_tape_constructible_with_time_arrow` are strong witnesses that call methods, others like `fc1_n1_q_state_carrier_present` use `std::any::type_name::<T>()`. This only proves the type *exists*, not that its constitutionally relevant API is intact. This is a weak form of witnessing that could allow for silent drift if, for example, a key method on `TuringBus` were removed but the type itself remained. The weaker witnesses should be strengthened to call the specific functions or methods cited as evidence in `TRACE_MATRIX_v2`.
    53	
    54	-   **(Q2.b) Ignored stubs use `panic!()` body. Right pattern?**
    55	    **PASS**. This is the correct and robust pattern for ignored tests. It ensures that if a test is accidentally un-ignored, it will fail loudly rather than silently passing.
    56	
    57	-   **(Q2.c) Cross-crate symbols deferral acceptable?**
    58	    **PASS**. Deferring the witness test for `Lean4Oracle` to a separate `fc_alignment_conformance.rs` within its own crate (`experiments/minif2f_v4/tests/`) is an acceptable modularization strategy. The gap is documented in `tests/fc_alignment_conformance.rs:248`.
    59	
    60	-   **(Q2.d) Binary-only symbols ignored. Defer refactor?**
    61	    **PASS**. The harness has correctly identified an architectural limitation where symbols in `evaluator.rs` are not easily testable from the library. Documenting this in `tests/fc_alignment_conformance.rs:243` and deferring the refactoring to a later phase is a reasonable project management decision.
    62	
    63	### Q3 — Case-law sediment
    64	
    65	The new cases effectively formalize recent decisions into constitutional precedent.
    66	
    67	-   **(Q3.a) TRUE precedents or session-summary docs?**
    68	    **PASS**. The `ruling:` and `precedent:` sections in C-071 through C-075 use binding language (e.g., "frozen", "MUST", "NOT allowed"). They function as true precedents for future automated and human action.
    69	
    70	-   **(Q3.b) C-073 overreach vs Art. V.1.2?**
    71	    **PASS**. `cases/C-073_architect_ai_commit_authority.yaml` states ArchitectAI scope is "ALL files EXCEPT constitution.md". This is a direct and faithful interpretation of Art. V.1.2 ("ArchitectAI commit authority on non-constitution") and Art. V.1.1 ("sudo only constitution.md"). There is no overreach.
    72	
    73	-   **(Q3.c) C-074 duplication?**
    74	    **PASS**. The relationship between `cases/C-074_fc_first_problem_handling.yaml` and the `feedback_fc_first_problem_handling` memory is not duplication but *sedimentation*. The case formalizes an informal "memory" into binding case-law, which is a core function of this governance system.
    75	
    76	-   **(Q3.d) Case cross-references form DAG. Circular reasoning hazard?**
    77	    **PASS**. The cross-references between cases (e.g., C-071 ↔ C-073) represent a system of interdependent definitions of authority and process. This is analogous to legal frameworks and does not constitute fallacious circular reasoning. The system is coherent.
    78	
    79	### Q4 — Trust Root manifest expansion 20 → 24
    80	
    81	The expansion correctly identifies governance instrumentation as a constitutional artifact. The scoping decisions are sound.
    82	
    83	-   **(Q4.a) Should `rules/SCHEMA.yaml` also be in Trust Root?**
    84	    **PASS**. The rationale in `TRACE_MATRIX_v2_2026-04-25.md § 5` for excluding `SCHEMA.yaml` is sound: the engine does not currently use it for validation. Including it would be premature.
    85	
    86	-   **(Q4.b) `build-check.sh` + `session-end.sh` excluded — gap?**
    87	    **PASS**. `judge.sh` is the most critical pre-tool-use gate. Excluding the other, lower-priority hooks is a reasonable scoping decision for A0, as documented in `TRACE_MATRIX_v2 § 5`.
    88	
    89	-   **(Q4.c) Should ALL tests be in Trust Root?**
    90	    **PASS**. The decision to include only `tests/fc_alignment_conformance.rs` is correct. This file is unique in its role as a direct, mechanical witness for the constitution-to-code mapping. Including all tests would create excessive manifest churn for minimal gain in constitutional stability.
    91	
    92	-   **(Q4.d) `TRACE_MATRIX_v?.md` docs not in Trust Root. Acceptable?**
    93	    **PASS**. The rationale in `TRACE_MATRIX_v2 § 5` is correct. The document is descriptive, not prescriptive enforcement. The enforcement mechanism (`fc_alignment_conformance.rs`) *is* in the Trust Root. This avoids a problematic self-reference while securing the actual enforcement vector.
    94	
    95	### Q5 — Constitutional alignment (meta-check)
    96	
    97	The A0 changes themselves appear to follow the constitutional process they are designed to enforce, with one point requiring verification.
    98	
    99	-   **(Q5.a) Did A0a-d touch `constitution.md`?**
   100	    **PASS**. A review of the deliverable list confirms that the four A0 commits modified rules, tests, cases, and the trace matrix/manifest. None of them touched `constitution.md`, which would have required a separate `sudo` authorization.
   101	
   102	-   **(Q5.b) Did each A0 commit message contain `FC-trace:`?**
   103	    **CHALLENGE**. The R-016 rule was introduced in commit `2e7f75a`. The subsequent three commits (`d8950ee`, `2a65339`, `e94e1b9`) were made *after* this rule was notionally active. It is a procedural requirement that these commits contain a valid `FC-trace:`. Verification of the commit logs is required to close this challenge. This is a test of whether the new process was immediately adopted.
   104	
   105	-   **(Q5.c) Regression test that asserts A0 itself can't be silently rolled back?**
   106	    **PASS**. The combination of `boot::tests::verify_trust_root_passes_on_intact_repo` (run via `cargo test`) and the inclusion of the A0 artifacts (`judge.sh`, `rules/MANIFEST.sha256`, etc.) in `genesis_payload.toml` serves this function. A git rollback of A0 would cause a hash mismatch on boot, triggering a `TRUST_ROOT_TAMPERED` panic. The system correctly protects itself against this regression.

codex
**Findings**

1. **A1 CHALLENGE: `p_0 = 0.10` is not “mathematically conservative” as worded.**  
For Gate H, `j-RR` passes when `RR_j <= p_0`; the frozen PREREG says this is a descriptive point check, not an inferential test, and `p_0` is normally calibrated before use ([PREREG_PPUT_CCL_2026-04-26.md](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:420), [line 432](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:432)). The amendment substitutes the ceiling `0.10` and calls it the “strictest possible substitute” ([PREREG_AMENDMENT_p0_defer_2026-04-25.md](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:23), [line 25](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:25)). That is backwards for a `<=` guardrail: `0.10` is the least strict admissible ceiling, not the strictest. It does not inflate Type-I error because `j-RR` is outside the inferential family ([PREREG_PPUT_CCL_2026-04-26.md](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:414)), but it can weaken the regression guardrail versus any eventual calibrated `p_0 < 0.10`.

2. **A6 CHALLENGE: FC trace events cannot be reliably joined to JSONL rows.**  
`run_swarm` creates `run_corr_id` and says Phase D joins by equality ([evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:485)), but `make_pput` independently recomputes `run_id` with a later timestamp ([evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1511)). These IDs can differ by milliseconds. Oneshot FC1-N12 events use `oneshot_{problem_file}` instead of the emitted JSONL `run_id` ([evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:404)). Fix by generating one run ID once and passing it into both `emit_event` and `PputResult`.

3. **A6 CHALLENGE: coverage is too sparse for the claimed “5-step compile loop” witness.**  
The enum reserves FC1-N7, FC1-N11, FC1-N12, FC1-E18, FC2-N20, FC2-N22, FC3-N31 ([fc_trace.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/fc_trace.rs:47)), but production emits only FC2-N22, FC2-N20, and oneshot FC1-N12. The packet itself admits FC1-N11, FC1-E18, FC3-N31, and swarm FC1-N12 are missing ([A8_EXIT_PACKET_2026-04-26.md](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:121), [line 125](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:125)). The swarm verifier calls increment `verifier_wait_ms` but emit no FC1-N12 event ([evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1002), [line 1269](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1269)). Minimum fix before Phase B: emit FC1-N12 for all swarm Lean calls and repair the run ID join.

4. **A7 CHALLENGE: provider routing is inconsistent with its own contract, and the smoke does not exercise the proxy.**  
The proxy doc says `"Qwen/..."`, `"openai/..."`, and anything with `/` route to SiliconFlow ([llm_proxy.py](/home/zephryj/projects/turingosv4/src/drivers/llm_proxy.py:25)). The code routes `Qwen/Qwen2.5-7B-Instruct` to DashScope because `m.startswith("qwen")` wins after the slash check, while `openai/gpt-4o` routes to SiliconFlow ([llm_proxy.py](/home/zephryj/projects/turingosv4/src/drivers/llm_proxy.py:196)). Explicit `siliconflow:Qwen/...` works, but the implicit matrix is not clean. The smoke intentionally bypasses the proxy ([smoke_siliconflow.sh](/home/zephryj/projects/turingosv4/scripts/smoke_siliconflow.sh:8); [_smoke_siliconflow.py](/home/zephryj/projects/turingosv4/scripts/_smoke_siliconflow.py:36)), so it does not catch this. Add a proxy-routed conformance smoke or unit test for the routing matrix and `[2,2,2]` round-robin invariant.

5. **A8/Q6 CHALLENGE: Trust Root count evidence is internally inconsistent.**  
The packet says A7 lands at 30 entries ([A8_EXIT_PACKET_2026-04-26.md](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:49)) and Q6 says six new Phase A entries ([A8_EXIT_PACKET_2026-04-26.md](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:157)). The actual manifest has 31 entries; A3’s `agent_models.rs` is also in Trust Root ([genesis_payload.toml](/home/zephryj/projects/turingosv4/genesis_payload.toml:109)), and the required-list test includes it ([trust_root_immutability.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/tests/trust_root_immutability.rs:107)). `genesis_payload.toml` also still says “Total: 25 files” ([genesis_payload.toml](/home/zephryj/projects/turingosv4/genesis_payload.toml:67)). Functional verification passes, but the exit packet is not self-consistent.

**Verified Passes**

- A1–A7 commit messages contain `FC-trace:`; none of those commits touched `constitution.md`.
- No tracked worktree drift; only an untracked `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26.md`.
- `cargo test --workspace`: 261 passed / 29 ignored / 0 failed.
- `boot::tests::verify_trust_root_passes_on_intact_repo`: passed inside the workspace run.
- `scripts/smoke_siliconflow.sh`: passed live, 3/3 configured keys returned `ack`.

**VERDICT**: CHALLENGE  
**Conviction**: high  
**Recommendation**: FIX-THEN-PROCEED
tokens used
207,689
**Findings**

1. **A1 CHALLENGE: `p_0 = 0.10` is not “mathematically conservative” as worded.**  
For Gate H, `j-RR` passes when `RR_j <= p_0`; the frozen PREREG says this is a descriptive point check, not an inferential test, and `p_0` is normally calibrated before use ([PREREG_PPUT_CCL_2026-04-26.md](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:420), [line 432](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:432)). The amendment substitutes the ceiling `0.10` and calls it the “strictest possible substitute” ([PREREG_AMENDMENT_p0_defer_2026-04-25.md](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:23), [line 25](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:25)). That is backwards for a `<=` guardrail: `0.10` is the least strict admissible ceiling, not the strictest. It does not inflate Type-I error because `j-RR` is outside the inferential family ([PREREG_PPUT_CCL_2026-04-26.md](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:414)), but it can weaken the regression guardrail versus any eventual calibrated `p_0 < 0.10`.

2. **A6 CHALLENGE: FC trace events cannot be reliably joined to JSONL rows.**  
`run_swarm` creates `run_corr_id` and says Phase D joins by equality ([evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:485)), but `make_pput` independently recomputes `run_id` with a later timestamp ([evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1511)). These IDs can differ by milliseconds. Oneshot FC1-N12 events use `oneshot_{problem_file}` instead of the emitted JSONL `run_id` ([evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:404)). Fix by generating one run ID once and passing it into both `emit_event` and `PputResult`.

3. **A6 CHALLENGE: coverage is too sparse for the claimed “5-step compile loop” witness.**  
The enum reserves FC1-N7, FC1-N11, FC1-N12, FC1-E18, FC2-N20, FC2-N22, FC3-N31 ([fc_trace.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/fc_trace.rs:47)), but production emits only FC2-N22, FC2-N20, and oneshot FC1-N12. The packet itself admits FC1-N11, FC1-E18, FC3-N31, and swarm FC1-N12 are missing ([A8_EXIT_PACKET_2026-04-26.md](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:121), [line 125](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:125)). The swarm verifier calls increment `verifier_wait_ms` but emit no FC1-N12 event ([evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1002), [line 1269](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1269)). Minimum fix before Phase B: emit FC1-N12 for all swarm Lean calls and repair the run ID join.

4. **A7 CHALLENGE: provider routing is inconsistent with its own contract, and the smoke does not exercise the proxy.**  
The proxy doc says `"Qwen/..."`, `"openai/..."`, and anything with `/` route to SiliconFlow ([llm_proxy.py](/home/zephryj/projects/turingosv4/src/drivers/llm_proxy.py:25)). The code routes `Qwen/Qwen2.5-7B-Instruct` to DashScope because `m.startswith("qwen")` wins after the slash check, while `openai/gpt-4o` routes to SiliconFlow ([llm_proxy.py](/home/zephryj/projects/turingosv4/src/drivers/llm_proxy.py:196)). Explicit `siliconflow:Qwen/...` works, but the implicit matrix is not clean. The smoke intentionally bypasses the proxy ([smoke_siliconflow.sh](/home/zephryj/projects/turingosv4/scripts/smoke_siliconflow.sh:8); [_smoke_siliconflow.py](/home/zephryj/projects/turingosv4/scripts/_smoke_siliconflow.py:36)), so it does not catch this. Add a proxy-routed conformance smoke or unit test for the routing matrix and `[2,2,2]` round-robin invariant.

5. **A8/Q6 CHALLENGE: Trust Root count evidence is internally inconsistent.**  
The packet says A7 lands at 30 entries ([A8_EXIT_PACKET_2026-04-26.md](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:49)) and Q6 says six new Phase A entries ([A8_EXIT_PACKET_2026-04-26.md](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:157)). The actual manifest has 31 entries; A3’s `agent_models.rs` is also in Trust Root ([genesis_payload.toml](/home/zephryj/projects/turingosv4/genesis_payload.toml:109)), and the required-list test includes it ([trust_root_immutability.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/tests/trust_root_immutability.rs:107)). `genesis_payload.toml` also still says “Total: 25 files” ([genesis_payload.toml](/home/zephryj/projects/turingosv4/genesis_payload.toml:67)). Functional verification passes, but the exit packet is not self-consistent.

**Verified Passes**

- A1–A7 commit messages contain `FC-trace:`; none of those commits touched `constitution.md`.
- No tracked worktree drift; only an untracked `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26.md`.
- `cargo test --workspace`: 261 passed / 29 ignored / 0 failed.
- `boot::tests::verify_trust_root_passes_on_intact_repo`: passed inside the workspace run.
- `scripts/smoke_siliconflow.sh`: passed live, 3/3 configured keys returned `ack`.

**VERDICT**: CHALLENGE  
**Conviction**: high  
**Recommendation**: FIX-THEN-PROCEED
