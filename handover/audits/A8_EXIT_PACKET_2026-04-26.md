# Phase A → B Exit Audit Packet (A8) — running through rounds 1–N

**Arc**: PPUT-CCL (`PREREG_PPUT_CCL_2026-04-26.md` round-4 PASS/PASS + amendment).
**Date**: 2026-04-26 (cumulative — most recent round noted at the bottom of the document; readers should consult § "Round-N outcome" for the latest verdict + § "Round-N fixes shipped" for the latest in-cycle delta).
**Authority**: ArchitectAI commit (Art. V.1.2). This packet is the input to dual external audit (Codex + Gemini) per Art. V.1.3 + memory `feedback_dual_audit`. Decision rule: PASS → Phase B (kernel instrumentation) authorized; CHALLENGE → in-cycle fixes; VETO → Phase A redesign.

**FC-trace**: meta-witness across FC1 / FC2 / FC3 (atoms instrument all three subgraphs).

## Round-1 outcome (2026-04-26)

- Codex: **CHALLENGE / high** — 5 findings (run_id ms drift; sparse FC1-N12 swarm coverage; routing matrix Qwen-HF misroute; Trust Root count off-by-1; PREREG amendment "strictest substitute" wording reversed).
- Gemini: **VETO / high** — same join-key + routing-conformance themes; emphasized Q6 absence of automated round-robin conformance test as REDESIGN-level for atom A7.
- Merged (per memory `feedback_dual_audit_conflict`, VETO > CHALLENGE > PASS): VETO. FIX-THEN-PROCEED cycle authorized.

## Round-2 fixes shipped (`A8e`, commit `5a56ff6`)

- **F1** unified `run_id` (new `run_id.rs` module + threaded into `make_pput`); oneshot stops using `oneshot_{problem_file}` placeholder. Closes Codex#2 + Gemini Q4.
- **F2** `scripts/test_llm_proxy.py` — 15-test Python suite pinning the round-robin invariant `[2,2,2]` across 6 calls (no live API). Closes Gemini VETO Q6.
- **F3** routing matrix bug fixed: slash-form ⇒ `siliconflow` FIRST in `detect_provider`; bare-qwen ⇒ `dashscope` only without slash. Closes Codex#4.
- **F4** swarm FC1-N12 emit at all 3 verify sites (`verify_omega_detailed` × 2 + `verify_partial`). Closes Codex#3.
- **F5** Trust Root count + header reconciled: actual 31 entries → 33 after A8e (was off-by-1 because `agent_models.rs` from A3 was already in TR before A1). Closes Codex#5.
- **F6** PREREG_AMENDMENT § 2 wording corrected: `p_0=0.10` is the **least-strict admissible ceiling** (max tolerated value), NOT the "strictest possible substitute". No Type-I inflation since `j-RR` is descriptive; explicit statistical-implications paragraph added. Closes Codex#1.

## Round-2 state to audit

- Cumulative tests: **264 PASS / 29 ignored / 0 failed** (was 261; +3 run_id unit tests).
- Python tests: 15/15 PASS in `scripts/test_llm_proxy.py`.
- Live smoke: `bash scripts/smoke_siliconflow.sh` PASS (3/3 keys responding 2026-04-26 05:0X UTC).
- Trust Root manifest: **33 entries** (was 31 before A8e; +run_id.rs + test_llm_proxy.py).
- Round-1 audit transcripts committed at `handover/audits/{CODEX,GEMINI}_PHASE_A8_EXIT_AUDIT_2026-04-26.md` for round-2 reviewers to verify the closures.

## Round-2 questions (in addition to § 6 of round-1)

- (RQ1) For each of the 6 fixes, verify the closure is complete: re-read F1's `run_id.rs` + `evaluator.rs` for any remaining `run_corr_id` / placeholder use; re-read F4's 3 emit sites for verdict-string correctness; re-read F6's amendment wording for any remaining "strictest" claim.
- (RQ2) Are there NEW defects introduced by the fixes? E.g., does F1's `run_id` parameter break the `make_pput` test fixtures (literal `"test_run_id"`)? Does F3 routing change misroute any model that DID work before?
- (RQ3) Is the 15-test `test_llm_proxy.py` battery actually load-bearing? Specifically: does it run in any CI pipeline, or only manually? If only manual, is its presence in Trust Root + the trust_root_immutability required-paths list enough to satisfy the "recurring conformance" bar Gemini's VETO required?
- (RQ4) F5 reconciles the count to 33. Verify by re-counting `^"` lines under `[trust_root]` in `genesis_payload.toml` and matching against the `required[]` array in `experiments/minif2f_v4/tests/trust_root_immutability.rs:79+`.
- (RQ5) F6 changed an immutable-by-convention amendment doc. Verify the amendment's NEW SHA-256 is in `[trust_root]` and the v0/v1 round-trip protocol still holds (the original PREREG round-4 doc is unchanged).

## Round-2 outcome (2026-04-26)

- Codex R2: **CHALLENGE / high** — 3 findings (F2 not recurring; PREREG_AMENDMENT § 8 still says "strictest plausible bar" contradicting § 2; A8 packet + TRACE_MATRIX stale on counts + risk #5 + run_corr_id symbol row + "in CI" claim).
- Gemini R2: **CHALLENGE / high** — 2 findings convergent with Codex (F2 needs CI integration; packet § 5 risk #5 stale).
- Merged: **CHALLENGE** (no VETO this round; both auditors confirm fixes are letter-correct, gaps are procedural/documentary).

## Round-3 fixes shipped (`A8e2`, commit `<pending>`)

- **G1** (Codex R2#1 + Gemini R2#1, hardened by A8e3 H6): `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` — Rust integration test that shells to `python3 scripts/test_llm_proxy.py` + asserts exit 0 + checks for the unittest "OK" trailer. Now exercised on every `cargo test --workspace` (PASS in A8e2/A8e3 verification). The Python suite is no longer "manual only" — it runs whenever the Rust tests run, which is every commit that touches Rust + every CI pipeline that already exercises Rust tests. **A8e3 H6 fail-closed**: missing `python3` is now a hard test failure (not a soft skip — Codex R3#3 caught the silent-pass risk). Explicit opt-out only via `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1` and the bypass is logged loudly. Closes the round-2 "recurring conformance gate" finding.
- **G2** (Codex R2#2): `PREREG_AMENDMENT § 8` audit-requirements paragraph reworded to remove the residual "strictest plausible bar is conservative" phrase that contradicted § 2's wording correction. Re-hashed in Trust Root.
- **G3** (Codex R2#3 + Gemini R2#2): A8 packet § 2 cumulative-test table extended with A8e + A8e2 columns; § 4 Trust-Root-clean assertion bumped 30→34; § 5 Risk #5 removed (closed by F4); § 6 Q7.a 261→265 + Q7.b 30→34. TRACE_MATRIX stale `run_corr_id` symbol row replaced; "in CI" softened to "via Rust wrapper post-A8e2".

## Round-3 questions (in addition to § 6 + round-2)

- (RQ6) Verify G1: run `cargo test -p minif2f_v4 --test llm_proxy_python_conformance` and confirm exit 0. Verify the test is added to TRACE_MATRIX_v2 § 2 with FC trace + closure rationale.
- (RQ7) Verify G2: re-grep `PREREG_AMENDMENT_p0_defer_2026-04-25.md` for any remaining "strictest" / "conservative" claim; the only acceptable use is the round-1 wording-correction note in § 2.
- (RQ8) Verify G3: re-count `genesis_payload.toml` `[trust_root]` entries (expect 34) and check TRACE_MATRIX_v2 manifest milestones list matches A0=24 → A1=25 → A3=26 → A5=27 → A6=28 → A7=31 → A8e=33 → A8e2=34.
- (RQ9) Look for any NEW staleness introduced by G3 — e.g., does the round-2 outcome paragraph accurately summarize the round-2 verdicts?

## Round-3 outcome (2026-04-26)

- Codex R3: **CHALLENGE / high** — 3 narrow findings (A8 packet line 118 still calls substitution "conservative"; packet § 3 A6 atom + Q4.a still say "FC1-N12 only in oneshot" + Q4.d still describes ms drift; G1 wrapper soft-skips on missing python3).
- Gemini R3: **CHALLENGE / high** — 1 narrow finding convergent with Codex (Q4.d stale) + non-blocking observation about `make_pput` arg count (21 args; deferred to Phase B+ refactor).
- Merged: **CHALLENGE**. Both auditors said code is sound + ready for Phase B; only the packet itself failed final-pass rigor.

## Round-4 fixes shipped (`A8e3`, commit `<pending>`)

Six narrow cleanup items. ALL documentary except H6 which adds a runtime fail-closed assertion.

- **H1** (Codex R3#1): A8 packet § 3 A1 atom description rewritten — removed "Mathematically conservative (strictest plausible bar)" + replaced with explicit "least-strict admissible value" + Type-I implications + cross-ref to PREREG_AMENDMENT § 2 wording correction.
- **H2** (Codex R3#2 + Gemini R3#1): A8 packet § 3 A6 atom description bumped 6 → 9 anchor sites; explicitly lists the 3 swarm-side FC1-N12 sites added by F4.
- **H3** (Codex R3#1): A8 packet § 6 Q2.a + Q4.a + Q4.d marked CLOSED with closure-rationale text and round-N origin; questions are no longer "open" for round-4 reviewers.
- **H4** (Codex R3#1): `genesis_payload.toml` Trust Root header comment about A1 PREREG amendment reworded — "conservative ceiling" → "max-tolerated ceiling — least-strict admissible".
- **H5** (Codex R3#2): TRACE_MATRIX § 5 item 7 now says "CLOSED" with explicit anchor count of 9 (was "commit pending" + "6 wired").
- **H6** (Codex R3#3): G1 wrapper test fails closed when `python3` is missing — was a soft skip via `eprintln + return`. Explicit opt-out `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1` for deliberate downgraded runs (logged loudly).

Note (Gemini R3 Finding 2, non-blocking): `make_pput` signature is now 21 positional args. Deferred to Phase B+ refactor (e.g. `PputResultBuilder` struct or named-arg pattern). Tracked here for the record but does NOT block Phase A → B exit.

## Round-4 questions (in addition to § 6 + round-2 + round-3)

- (RQ10) Verify H1: re-grep `A8_EXIT_PACKET_2026-04-26.md` for any remaining "conservative" / "strictest" claim about `p_0`. Acceptable uses: round-1/2/3 retrospective text describing what the packet USED to say.
- (RQ11) Verify H2: re-count anchor sites in `experiments/minif2f_v4/src/bin/evaluator.rs` by grepping `fc_trace::emit_event(`; expect 9 production sites (synthetic short-circuit + mr tick + OMEGA full-proof + OMEGA per-tactic + max-tx + oneshot verify + 2 swarm `verify_omega_detailed` + swarm `verify_partial`).
- (RQ12) Verify H6: cause `python3` to be missing (e.g. `PATH=/tmp cargo test --test llm_proxy_python_conformance`) and confirm the test FAILS rather than silently passes.
- (RQ13) Verify packet self-consistency: any other "conservative" claims about the substitution? Any other anchor-count mismatches? Any other contradictions between round-1 questions and round-2/3 closures?

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

|        | A0a baseline | A0e PASS | A4 land | A5 land | A6 land | A7 land | A8e | A8e2 |
|---|---|---|---|---|---|---|---|---|
| `cargo test --workspace` PASS | 187 | 204 | 234 | 254 | 261 | 261 | 264 | 265 |
| ignored | 20 | 29 | 29 | 29 | 29 | 29 | 29 | 29 |
| failed | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Trust Root manifest entries | 20 | 24 | 24 | 27 | 28 | 31 | 33 | 34 |
| Python `test_llm_proxy.py` | — | — | — | — | — | — | 15/15 | 15/15 |

(Trust Root counts re-tabulated in A8e2 fix G3: A3's `agent_models.rs` was already in TR before A1, which the round-1 packet undercounted by 1. A8e2 adds `tests/llm_proxy_python_conformance.rs` to TR, raising the count to 34.)

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
- Substitutes `p_0 = 0.10` (PREREG § 5.5 ceiling) for the calibration-derived value at every Gate H consumer. The substitution is operationally permitted (the PREREG explicitly admits up to 0.10 as the ceiling) but is the **least-strict admissible value** since `j-RR ≤ p_0` makes a SMALLER `p_0` stricter — see PREREG_AMENDMENT § 2 wording correction (round-1 audit Codex#1, A8e fix F6) for full statistical implications. No Type-I inflation since `j-RR` is descriptive (PREREG § 5.4), outside the inferential family. May be less protective than an eventual calibrated `p_0 < 0.10`; acceptable because Gate H is Phase E and § 3 conditions ensure calibration runs first. Re-calibration conditions in § 3 list 5 items (N-experiments arc complete / swarm_N=1 mode landed / per-agent budget normalization landed / hetero-LLM exp complete / Phase D ArchitectAI runtime exists).
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
- Trust Root manifest 26 → 27. (A8e3 fix H4 corrected the prior 25→26 claim — A3's `agent_models.rs` had already raised the count to 26 before A5; per the corrected milestone chain in `genesis_payload.toml` header.)

### A6 (FC tracing)
- New module `experiments/minif2f_v4/src/fc_trace.rs`. Pure stdlib (zero new deps). 7-variant `FcId` enum (FC1-N7 / FC1-N11 / FC1-N12 / FC1-E18 / FC2-N20 / FC2-N22 / FC3-N31). `FC_TRACE=1` gate cached in `OnceLock`; `FC_TRACE_FILE=<path>` redirects emit to file.
- 9 wired anchor sites total (round-1 had 6 — A8e fix F4 added 3 swarm verify sites): synthetic short-circuit / mr tick / OMEGA full-proof / OMEGA per-tactic / natural MaxTxExhausted (with budget_regime payload from A5) / oneshot FC1-N12 verify bracket / **swarm `verify_omega_detailed` × 2 paths (alone + tape+payload retry)** / **swarm `verify_partial`**.
- **FC-trace**: meta-witness for the 5-step compile loop.
- Tests: 7 (6 unit + 1 end-to-end smoke `tests/fc_trace_smoke.rs` exercising `FC_TRACE=1` in a child process — required because the gate is `OnceLock`-cached).
- Trust Root manifest 27 → 28. (Same off-by-one correction as A5's delta; chain matches `genesis_payload.toml` header.)
- Resolves TRACE_MATRIX_v2 § 5 item 7.

### A7 (SiliconFlow plumbing)
- `src/drivers/llm_proxy.py` ported from v3 with one load-bearing v4 change: per-provider multi-key round-robin. PROVIDERS map now holds a list of env names per provider; `get_client_round_robin` distributes via `_rr_counters` mod `len(clients)`. `/stats` exposes `per_key_requests` for observability. New `siliconflow:<model>` provider-prefix syntax.
- 3 SiliconFlow keys (primary / secondary / tertiary) split concurrent traffic across separate rate-limit pools — V3L-27 (case C-027) single-key N=30 401/429 collapse mitigation.
- `scripts/smoke_siliconflow.sh` + `_smoke_siliconflow.py`: 3 keys × 1 probe (Qwen2.5-7B-Instruct, max_tokens=8). Verified 2026-04-26: primary 2989ms, secondary 1546ms, tertiary 1549ms; 33+1 tokens; content="ack". Proxy round-robin verified [2,2,2] across 6 calls.
- **FC-trace**: FC1-N7 (δ/AI provider expansion).
- No new Rust tests (integration plumbing).
- Memory: `reference_siliconflow.md` records SiliconFlow as the Phase D heterogeneous lane (NOT a probe-only target) and the context-loss anti-pattern (check `.env` + project files BEFORE asking for credentials).
- Trust Root manifest 28 → 31 (3 entries: `llm_proxy.py` + 2 smoke scripts).

## § 4. Phase B → C exit checklist (from PREREG_AMENDMENT § 4) — Phase A side

The PREREG amendment shifted the Phase B → C gate. From the Phase A perspective, the items it lists are now satisfied:

- ❌ p_0 calibration jsonl frozen (was REQUIRED) → **DEFERRED with substitution per amendment § 2**: `p_0 = 0.10` hardcoded at every Gate H consumer.
- ✅ B1–B7 + B7-extra mode toggle infrastructure complete (pre-Phase A baseline; round-4 PASS/PASS).
- ✅ Phase A0 harness modernization complete (`62c4e14`).
- ✅ Tools qualified per case C-075 (DO-178C tool qualification): `runner.sh`, `compute_p0.py`, evaluator boot enforcement, etc.
- ✅ Trust Root verifies clean (`boot::tests::verify_trust_root_passes_on_intact_repo` PASS at 34-entry manifest post-A8e2).

## § 5. Risks and known limitations entering Phase B

1. **`per_agent` budget regime untested at runtime**. A5 unit tests verify the scaling math (`base × N`) and env-coupled resolver. No live-LLM run with `BUDGET_REGIME=per_agent` has been smoked. Phase B kernel instrumentation will be the first opportunity to observe its behavior on a real problem; defer treatment to PREREG re-calibration if any anomaly surfaces.
2. **FC-trace coverage still partial after A8e**. 9 wired anchor sites now cover HALT decomposition (FC2-N22 × 4 exit paths) + mr tick (FC2-N20) + Lean oracle scope (FC1-N12 × 4 sites: oneshot + swarm `verify_omega_detailed` × 2 + swarm `verify_partial`). Still NOT emitting: FC1-N7 prompt-build, FC1-N11 ∏p decision diversity (per-proposal), FC1-E18 preserve-Q_t (per ∏p=0), FC3-N31 WAL append. The `FcId` enum reserves these variants. Phase B+ kernel instrumentation should fill them in as the Phase B emit boundary lands.
3. **SiliconFlow rate-limit at scale**. A7 verified 3 keys responding individually at N=1 concurrency. V3L-27 demonstrates collapse at N=30 single-key. The v4 multi-key round-robin should triple the safe N envelope but the actual sweet spot for our hetero swarm is unmeasured. Phase D heterogeneous-batch design should land a `--max-concurrency` knob (currently `LLM_PROXY_CONCURRENCY=5` env in proxy) tuned per provider.
4. **Heterogeneous swarm = Phase D, not B/C**. Per F-2026-04-25-02 + the `agent_models.rs` `PHASE_D_HETERO_GATE_ENV_VAR` invariant, Phases B and C MUST stay single-model so ablation axes are not confounded. A7's plumbing exists for future Phase D work; Phase B uses the existing `deepseek-v4-flash` thinking-off backbone unchanged.

(Round-1 risk #5 "No FC1-N12 emit in run_swarm verify path" was CLOSED by A8e fix F4 — both auditors flagged the staleness in round-2 (Codex R2#3, Gemini R2#2). Removed from this list.)

## § 6. Specific questions for auditors

### Q1 — Atomicity and FC-trace discipline
- (Q1.a) Each of A1–A7 was committed as one atomic commit with `FC-trace: <FC?-N?>` in the commit message, per memory `feedback_fc_first_problem_handling`. Verify by re-reading each commit message. Are any commits missing FC-trace, or claiming an FC node that doesn't match the actual code change?
- (Q1.b) Did any commit touch `constitution.md`? (Per Art. V.1.1 + R-018 BLOCK, this is a violation absent sudo.) Trace each commit's file changes.
- (Q1.c) Are there code changes outside the listed atoms — e.g., drift from un-committed tweaks — that should have been their own atom?

### Q2 — PREREG amendment soundness (A1)
- (Q2.a) ~~Substituting `p_0 = 0.10` at the Gate H consumer (the PREREG § 5.5 ceiling) is described as conservative.~~ **CLOSED A8e F6 + A8e2 G2 + A8e3 H1**: the substitution is the LEAST-strict admissible ceiling (max tolerated value for `j-RR ≤ p_0`), NOT conservative. No Type-I inflation since `j-RR` is descriptive (PREREG § 5.4). Wording fixed in PREREG_AMENDMENT § 2 (A8e F6) + § 8 (A8e2 G2) + this packet's A1-atom description (A8e3 H1). No further verification needed.
- (Q2.b) Re-calibration conditions list 5 items (notepad ref + per-atom commits). Does this implicitly couple Phase B → C to Phase D readiness in a way that would block forward progress if Phase D slips?
- (Q2.c) The amendment doc itself is in Trust Root (entry 25). Is the amendment's own SHA-256 referenced anywhere that would prevent a silent re-edit?

### Q3 — Budget regime soundness (A5)
- (Q3.a) `BUDGET_REGIME=per_agent` scales the loop bound as `base × N`. For a swarm at N=8 with `MAX_TRANSACTIONS=200`, the loop runs 1600 iterations — each agent receives ~200 proposals. Does this match the brainstorm § A.3 "fixed proposal budget" intent, or is a per-agent counter (each agent independently capped at 200, regardless of round-robin pickup) more faithful?
- (Q3.b) `token_total` and `wall_clock` regimes are declared startup-fatal `UnimplementedRegime`. Is "fail loud" the right default, or should they fall back to `total_proposal` with a WARN log?
- (Q3.c) The default (env unset) preserves Phase B baseline `total_proposal × 200` bit-for-bit. Verify this is true under all code paths — including the synthetic short-circuit and error/timeout exits.

### Q4 — FC tracing coverage (A6)
- (Q4.a) ~~6 wired anchor sites cover only FC2-N22 (HALT, 4 paths) + FC2-N20 (mr tick) + FC1-N12 (oneshot verify only).~~ **PARTIALLY CLOSED A8e F4**: anchor count is now **9** (added swarm `verify_omega_detailed` × 2 + swarm `verify_partial`); FC1-N12 now covers the swarm path. FcId enum still has 4 unwired variants (FC1-N7, FC1-N11, FC1-E18, FC3-N31) — kept as Phase B+ kernel-instrumentation work. Verify the 9-site coverage is sufficient for the round-3 acceptance bar.
- (Q4.b) `OnceLock`-cached gate read means a process started with `FC_TRACE=0` (or unset) ignores any later runtime change. Acceptable for evaluator's one-process-per-problem model, but does it pose a risk for any test or runner that mutates the env mid-process?
- (Q4.c) Hand-rolled JSON encoder vs the `serde_json` already in deps. Was there a real reason to avoid `serde_json::to_string` here, or is this premature dep avoidance?
- (Q4.d) ~~`run_corr_id` format = `condition_problem_id_unix-ms`. `make_pput`'s `run_id` independently re-computes this with its own ts. The two will differ by milliseconds. Is the join semantics for Phase D consumers documented anywhere?~~ **CLOSED A8e F1**: `run_corr_id` was renamed to `run_id`, lifted to `experiments/minif2f_v4/src/run_id.rs::mint_run_id`, and threaded into both `emit_event` and `make_pput` so they stamp the same identifier (zero ms drift). Phase D joins by `run_id` equality. No further work.

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
- (Q7.a) Verify `cargo test --workspace` PASS count: 265 / 29 ignored / 0 failed (post-A8e2; +1 over A8e from the new Python-conformance Rust wrapper). Spot-check by re-running.
- (Q7.b) Verify `boot::tests::verify_trust_root_passes_on_intact_repo` PASS at 34-entry manifest (post-A8e2; +1 over A8e from `tests/llm_proxy_python_conformance.rs`).
- (Q7.c) Verify `scripts/smoke_siliconflow.sh` PASS (live API; cost ~$0.005).
- (Q7.d) Are there OPEN P0 defects from any earlier Phase A audit (A0e CHALLENGE/CHALLENGE → 7 fixes)? Re-read the A0e audit + verify each of the 7 items closed.
- (Q7.e) Phase B's first sub-atom is "JSONL schema v2 + C_i full-cost aggregator" (notepad). Are there any Phase A artifacts that would BLOCK that scope? E.g., schema fields needed by B1 that aren't yet in `RunAggregate`?

---

End your audit with:
- **VERDICT**: PASS / CHALLENGE / VETO
- **Conviction**: low / medium / high
- **Recommendation**: PROCEED to Phase B (kernel instrumentation + PPUT accounting) / FIX-THEN-PROCEED / REDESIGN

PASS = all 7 atoms cleanly land, no open P0, Phase B prerequisites met. CHALLENGE = correctable in one cycle. VETO = Phase A redesign required.
