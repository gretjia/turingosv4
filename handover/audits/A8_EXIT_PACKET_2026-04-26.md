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
