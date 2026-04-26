# Phase A → B Exit Audit Packet (A8) — current-state snapshot

**Arc**: PPUT-CCL (`PREREG_PPUT_CCL_2026-04-26.md` round-4 PASS/PASS + `PREREG_AMENDMENT_p0_defer_2026-04-25.md`).
**Date**: 2026-04-26 (current-state — most recent A8e re-issue noted in `A8_AUDIT_HISTORY_2026-04-26.md`).
**Authority**: ArchitectAI commit (Art. V.1.2). Input to dual external audit (Codex + Gemini) per Art. V.1.3 + memory `feedback_dual_audit`.

**Decision rule**: PASS → Phase B (kernel instrumentation + PPUT accounting) authorized; CHALLENGE → in-cycle fixes; VETO → Phase A redesign.

**FC-trace**: meta-witness across FC1 / FC2 / FC3 (atoms instrument all three subgraphs).

**Document split (A8e7)**: this packet is the **stable current-state snapshot** of Phase A at exit. The chronological history of audit rounds 1–N + their in-cycle fix bundles lives in the **companion** document `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` (append-only). Past audit transcripts are at `handover/audits/{CODEX,GEMINI}_PHASE_A8_EXIT_AUDIT_2026-04-26{,_R2,_R3,_R4,_R5,_R6,...}.md`. Reviewers needing the full chronology read the history doc; reviewers verifying current-state correctness against Phase B prerequisites read THIS packet.

---

## § 1. Phase A scope and atom map

Phase A = pre-flight (days 1–3 of the 30-day arc). Decomposed into 8 atoms (A0–A7) plus the in-cycle fix bundles A8e..A8e6 produced during the multi-round audit. The atom-level deliverables:

- **A0** (a–e): harness modernization. Closed by `62c4e14` (A0e exit audit + 7-item fixes).
- **A1**: PREREG amendment p_0 calibration deferral.
- **A2**: P0a `swarm_N=1` mode + `parse_swarm_condition_n` unit tests.
- **A3**: per-agent `AGENT_MODELS` env var (Phase B+C single-model invariant gate).
- **A4**: decomposed metrics (`hit_max_tx` + `tactic_diversity` + `verifier_wait_ms`).
- **A5**: per-agent budget normalization (`BUDGET_REGIME` + `MAX_TRANSACTIONS`).
- **A6**: per-line FC tagging via structured JSON events (`fc_trace` module).
- **A7**: SiliconFlow heterogeneous-LLM provider plumbing (proxy + 3-key smoke).
- **A8**: this packet — Phase A → B exit audit.

Atom commit chain (atomic, FC-traced, all under ArchitectAI commit authority — none touched `constitution.md`):

```
2e7f75a  A0a   d8950ee  A0b   2a65339  A0c   e94e1b9  A0d   62c4e14  A0e
6be6eb4  A1    180a300  A2    7f4bc0c  A3    a5c78e4  A4    30f2a14  A5
89994c7  A6    90953d6  A7    60292dc  A8 prep
```

In-cycle audit-fix bundles (chronology in `A8_AUDIT_HISTORY`):

```
5a56ff6  A8e   0af47b7  A8e2  3d38ba5  A8e3  8693789  A8e4  1622017  A8e5  dbcf53a  A8e6
```

## § 2. Current-state metrics

| Metric | Value | Source / verification |
|---|---|---|
| `cargo test --workspace` PASS | **265** | re-runnable; all suites green |
| `cargo test --workspace` ignored | 29 | Phase B+ deferred stubs |
| `cargo test --workspace` failed | **0** | — |
| `python3 scripts/test_llm_proxy.py` | **16/16 PASS** | proxy routing + round-robin conformance |
| Trust Root manifest entries | **34** | `genesis_payload.toml [trust_root]` count + matches `trust_root_immutability::test_trust_root_manifest_includes_b2_b4_files` required-paths list |
| `boot::tests::verify_trust_root_passes_on_intact_repo` | **PASS** | re-hashes match the manifest |
| `bash scripts/smoke_siliconflow.sh` | **PASS (3/3 keys)** | live API; cost ~$0.005 per run |
| FC-trace anchor sites in `evaluator.rs` | **9** | grep `fc_trace::emit_event(`; 8 in `run_swarm` + 1 in `run_oneshot` |
| `make_pput` arity | **24 positional args** | refactor to builder pattern deferred to Phase B+ (Gemini R3 non-blocking observation) |

Trust Root manifest progression: A0=24 → A1=25 → A3=26 → A5=27 → A6=28 → A7=31 → A8e=33 → A8e2=34.

## § 3. Per-atom acceptance evidence (current state)

### A0 (harness modernization)
- Closed by dual-audit cycle ending `62c4e14`. CODEX_PHASE_A0_EXIT_AUDIT + GEMINI_PHASE_A0_EXIT_AUDIT both transitioned to PASS-equivalent after the 7 in-cycle fixes; no open P0.

### A1 (PREREG amendment) — `6be6eb4`
- File: `handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md`.
- Substitutes `p_0 = 0.10` (PREREG § 5.5 ceiling) for the calibration-derived value at every Gate H consumer.
- **Statistical framing** (post-A8e F6 + A8e2 G2 wording corrections): `0.10` is the **least-strict admissible value** the PREREG ceiling allows; `j-RR ≤ p_0` makes a SMALLER `p_0` stricter, so this substitution is operationally permitted but NOT a tighter-than-original guarantee. No Type-I inflation since `j-RR` is descriptive (PREREG § 5.4), outside the inferential family. May be less protective than an eventual calibrated `p_0 < 0.10`; acceptable because Gate H is Phase E and § 3 conditions ensure calibration runs first.
- **FC-trace**: FC1-N12 + Art. V.1.2 + cases C-073 + C-075.

### A2 (`swarm_N=1` mode) — `180a300`
- New `parse_swarm_condition_n` in `experiments/minif2f_v4/src/bin/evaluator.rs` discriminates `n<digits>` from `oneshot` / `hybrid_v1` / malformed. PREREG_AMENDMENT § 3 condition 2 cleared.
- **FC-trace**: FC2-N16 + FC1-N11.
- 5 unit tests.

### A3 (`AGENT_MODELS` env var) — `7f4bc0c`
- New `experiments/minif2f_v4/src/agent_models.rs`. Pure parser + expander + env-coupled resolver. Heterogeneity gated by `PHASE_D_HETERO_OK=1` — Phase B+C single-model invariant enforced at startup BEFORE any LLM call.
- **FC-trace**: FC1-N7 (δ/AI canonical identity per Agent_i).
- 11 unit tests.

### A4 (decomposed metrics) — `a5c78e4`
- 3 non-Optional v2 fields on `RunAggregate` + legacy `PputResult`: `hit_max_tx`, `tactic_diversity`, `verifier_wait_ms`. Helper `compute_tactic_diversity`.
- **FC-trace**: FC2-N22 + FC1-N11 + FC1-N12.
- 5 conformance tests.

### A5 (budget regime) — `30f2a14`
- New `experiments/minif2f_v4/src/budget_regime.rs`. 4-variant `BudgetRegime` enum: `total_proposal` (default; current behavior preserved bit-for-bit) / `per_agent` (loop bound = base × N) / `token_total` (declared; startup-fatal `UnimplementedRegime`) / `wall_clock` (declared; startup-fatal). 2 new non-Optional v2 fields: `budget_regime` + `budget_max_transactions`.
- `run_swarm` startup: `let max_transactions = 200` → `resolve_budget(n_agents)` with startup-fatal error path.
- **FC-trace**: FC2-N22 + FC1-N7.
- 16 unit tests; PREREG_AMENDMENT § 3 condition 3 cleared.
- Trust Root: `experiments/minif2f_v4/src/budget_regime.rs` added (chain position 27).

### A6 (FC tracing) — `89994c7` (+ A8e fix F4)
- New `experiments/minif2f_v4/src/fc_trace.rs`. Pure stdlib (zero new deps). 7-variant `FcId` enum (FC1-N7 / FC1-N11 / FC1-N12 / FC1-E18 / FC2-N20 / FC2-N22 / FC3-N31). `FC_TRACE=1` gate cached in `OnceLock`; `FC_TRACE_FILE=<path>` redirects emit to file.
- **9 wired anchor sites** (post-A8e total — 8 in `run_swarm` + 1 in `run_oneshot`):
  - FC2-N22 synthetic short-circuit (swarm)
  - FC2-N20 mr tick (swarm)
  - FC2-N22 OMEGA full-proof accept (swarm)
  - FC2-N22 OMEGA per-tactic accept (swarm)
  - FC2-N22 natural MaxTxExhausted (swarm; carries `budget_regime` payload from A5)
  - FC1-N12 verify bracket (oneshot)
  - FC1-N12 swarm `verify_omega_detailed` path "alone" (added by A8e F4)
  - FC1-N12 swarm `verify_omega_detailed` path "tape+payload" (added by A8e F4)
  - FC1-N12 swarm `verify_partial` (added by A8e F4)
- Per-run correlation: `experiments/minif2f_v4/src/run_id.rs::mint_run_id` (added by A8e F1) — one identifier per run, threaded into both `emit_event` and `make_pput` so FC events join v2 jsonl rows by equality.
- **FC-trace**: meta-witness for the 5-step compile loop.
- 7 tests (6 unit + 1 end-to-end smoke `tests/fc_trace_smoke.rs` exercising `FC_TRACE=1` in a child process). Plus 3 unit tests in `run_id.rs`.
- Trust Root: `fc_trace.rs` (chain position 28) + `run_id.rs` (chain position 33 via A8e).

### A7 (SiliconFlow plumbing) — `90953d6` (+ A8e2 fix G1 + A8e6 fix K2)
- `src/drivers/llm_proxy.py` ported from v3 with one load-bearing v4 change: per-provider multi-key round-robin. `detect_provider()` routing (post-A8e F3 + A8e6 K2): `provider:...` explicit prefix wins; else slash-form ⇒ `siliconflow` FIRST (covers `Qwen/...`, `openai/...`, `meta-llama/...`, `deepseek-ai/...` HuggingFace-style ids); else `"deepseek" in m` ⇒ `deepseek` (bare-form only); else `m.startswith("qwen")` ⇒ `dashscope`; else `dashscope` default.
- 3 SiliconFlow keys (primary / secondary / tertiary) split concurrent traffic across separate rate-limit pools — V3L-27 / case C-027 single-key N=30 collapse mitigation.
- `scripts/smoke_siliconflow.sh` + `_smoke_siliconflow.py`: 3 keys × 1 probe (Qwen2.5-7B-Instruct, max_tokens=8). Verified all 3 keys responding.
- `scripts/test_llm_proxy.py`: 16-test routing + round-robin conformance suite (no live API). Wrapped by `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` (added by A8e2 G1) so the suite runs on every `cargo test --workspace`. The wrapper FAILS CLOSED if `python3` is missing (A8e3 H6); explicit opt-out `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1`.
- **FC-trace**: FC1-N7 (δ/AI provider expansion).
- Memory: `reference_siliconflow.md` records SiliconFlow as the Phase D heterogeneous lane (NOT a probe-only target).
- Trust Root chain positions 29–31: `llm_proxy.py` + `smoke_siliconflow.sh` + `_smoke_siliconflow.py`. A8e adds `scripts/test_llm_proxy.py` (32) + `experiments/minif2f_v4/src/run_id.rs` (33). A8e2 adds `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` (34).

## § 4. Phase B → C exit checklist (from PREREG_AMENDMENT § 4) — Phase A side

The PREREG amendment shifted the Phase B → C gate. From the Phase A perspective, the items it lists are now satisfied:

- ❌ p_0 calibration jsonl frozen (was REQUIRED) → **DEFERRED with substitution per amendment § 2**: `p_0 = 0.10` hardcoded at every Gate H consumer.
- ✅ B1–B7 + B7-extra mode toggle infrastructure complete (pre-Phase A baseline; round-4 PASS/PASS).
- ✅ Phase A0 harness modernization complete (`62c4e14`).
- ✅ Tools qualified per case C-075 (DO-178C tool qualification): `runner.sh`, `compute_p0.py`, evaluator boot enforcement, etc.
- ✅ Trust Root verifies clean (`boot::tests::verify_trust_root_passes_on_intact_repo` PASS at 34-entry manifest).

## § 5. Risks and known limitations entering Phase B

1. **`per_agent` budget regime untested at runtime**. A5 unit tests verify the scaling math (`base × N`) and env-coupled resolver. No live-LLM run with `BUDGET_REGIME=per_agent` has been smoked. Phase B kernel instrumentation will be the first opportunity to observe its behavior on a real problem.
2. **FC-trace coverage still partial**. 9 wired anchor sites cover HALT decomposition (FC2-N22 × 4 exit paths) + mr tick (FC2-N20) + Lean oracle scope (FC1-N12 × 4 sites: oneshot + swarm `verify_omega_detailed` × 2 + swarm `verify_partial`). Still NOT emitting: FC1-N7 prompt-build, FC1-N11 ∏p decision diversity (per-proposal), FC1-E18 preserve-Q_t (per ∏p=0), FC3-N31 WAL append. The `FcId` enum reserves these variants. Phase B+ kernel instrumentation should fill them in as the Phase B emit boundary lands.
3. **SiliconFlow rate-limit at scale**. A7 verified 3 keys responding individually at N=1 concurrency. V3L-27 demonstrates collapse at N=30 single-key. The v4 multi-key round-robin should triple the safe N envelope but the actual sweet spot for our hetero swarm is unmeasured. Phase D heterogeneous-batch design should land a `--max-concurrency` knob (currently `LLM_PROXY_CONCURRENCY=5` env in proxy) tuned per provider.
4. **Heterogeneous swarm = Phase D, not B/C**. Per F-2026-04-25-02 + the `agent_models.rs` `PHASE_D_HETERO_GATE_ENV_VAR` invariant, Phases B and C MUST stay single-model so ablation axes are not confounded. A7's plumbing exists for future Phase D work; Phase B uses the existing `deepseek-v4-flash` thinking-off backbone unchanged.
5. **`make_pput` arity = 24 (Phase B+ refactor)**. Gemini R3 + R5 spot-checks identified the positional-arg list as a maintainability hazard; refactor to builder pattern (`PputResultBuilder`) recommended early in Phase B. Non-blocking; runtime correctness verified by 4 in-binary tests + the integration battery.

## § 6. Specific questions for auditors

(All round-1..round-6 questions and their closures are recorded in `A8_AUDIT_HISTORY_2026-04-26.md`. The questions below are the round-N+1 set — a fresh reviewer pass that does NOT need to re-litigate prior closures.)

### Q1 — Code correctness spot-checks
- (Q1.a) Does `experiments/minif2f_v4/src/bin/evaluator.rs` mint a single `run_id` at function entry and pass it to BOTH `fc_trace::emit_event` and `make_pput` so FC events and v2 jsonl rows share one identifier? Verify by reading `run_swarm` and `run_oneshot` entry points + `make_pput` signature.
- (Q1.b) Does `src/drivers/llm_proxy.py::detect_provider` route `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B` to `siliconflow` (not `deepseek`)? Verify by reading the function + checking that `scripts/test_llm_proxy.py` includes `test_deepseek_slash_form_routes_to_siliconflow`.
- (Q1.c) Does `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` fail closed when `python3` is missing? Verify by reading the test + spot-checking that the panic message describes the failure rather than a silent skip.

### Q2 — PREREG amendment soundness
- (Q2.a) `p_0 = 0.10` substitution: is the wording in `PREREG_AMENDMENT_p0_defer_2026-04-25.md` § 2 + § 8 consistent + statistically correct (least-strict admissible, not "conservative" / "strictest")? Verify by full-text grep for "conservative" / "strictest" — the only acceptable matches are explicit retrospective notes.
- (Q2.b) Re-calibration conditions in § 3 list 5 items. Does coupling Phase B → C to Phase D readiness create any forward-progress hazard?
- (Q2.c) The amendment's SHA-256 is in Trust Root. Does `boot::tests::verify_trust_root_passes_on_intact_repo` pass cleanly?

### Q3 — Atomicity, FC-trace discipline, governance
- (Q3.a) Each of A1–A7 + A8e..A8e6 was committed as one atomic commit with `FC-trace: <FC?-N?>` in the message. Verify by re-reading commit messages. Any commit missing FC-trace? Any commit that touched `constitution.md`?
- (Q3.b) 5 cases C-071..C-075 sediment 2026-04-25 session decisions as constitutional precedent (governance / Veto-AI / ArchitectAI commit authority / FC-first / DO-178C tool qualification). Are the rulings standalone-readable + correctly cross-referenced?
- (Q3.c) Trust Root manifest 34 entries. Are all 10 Phase A additions load-bearing (i.e., does tampering each one weaken the constitutional gate)?

### Q4 — Phase A → B exit decision
- (Q4.a) Spot-check: re-run `cargo test --workspace`; expect 265 PASS / 29 ignored / 0 failed.
- (Q4.b) Spot-check: re-run `python3 scripts/test_llm_proxy.py`; expect 16/16 PASS.
- (Q4.c) Spot-check: re-run `bash scripts/smoke_siliconflow.sh`; expect PASS (3/3 keys; live API; cost ~$0.005).
- (Q4.d) Are there any open P0 defects from any prior round (1–6)? Cross-reference `A8_AUDIT_HISTORY_2026-04-26.md` to verify each in-cycle fix bundle's closures hold against current source.
- (Q4.e) Phase B's first sub-atom is "JSONL schema v2 + C_i full-cost aggregator" (notepad). Are there any Phase A artifacts that would BLOCK that scope? E.g., schema fields needed by B1 not yet in `RunAggregate`?

### Q5 — Packet/history split (A8e7 structural rewrite)
- (Q5.a) Is the split between `A8_EXIT_PACKET` (stable current-state) and `A8_AUDIT_HISTORY` (append-only chronology) consistent with the project's existing pattern (constitution + Art. V.3 amendment log; PREREG + PREREG_AMENDMENT)?
- (Q5.b) Is the history doc append-only in spirit — i.e., do past round entries describe what was true at that round's snapshot, without retroactive edits?
- (Q5.c) Does this packet (current-state) contain ANY round-N retrospective text, fix-shipped headers, or "previous round caught X" claims? It should NOT — those belong only in the history doc.

---

End your audit with:
- **VERDICT**: PASS / CHALLENGE / VETO
- **Conviction**: low / medium / high
- **Recommendation**: PROCEED to Phase B (kernel instrumentation + PPUT accounting) / FIX-THEN-PROCEED / REDESIGN

PASS = current-state evidence passes Phase B prerequisites + no open P0 from prior rounds + no new substantive findings. CHALLENGE = correctable in one cycle. VETO = Phase A redesign required.
