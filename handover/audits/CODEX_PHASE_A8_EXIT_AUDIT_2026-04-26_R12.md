# Codex Phase A → B Exit Audit (PPUT-CCL arc)
**Round**: R12
**Date**: 2026-04-26
**Reference**: see `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` for full chronology including atom commit chain + per-round verdicts/fixes.
**Test baseline**: 265 PASS + 29 ignored + 0 failed (Rust); 16/16 PASS (Python proxy tests)
**Trust Root**: 37-entry manifest verifies clean
**Prompt size**: 327800 chars

---

Reading prompt from stdin...
OpenAI Codex v0.125.0 (research preview)
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: danger-full-access
reasoning effort: xhigh
reasoning summaries: none
session id: 019dc8f8-9e46-7cc0-b7d1-26b91ac09009
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

# Phase A → B Exit Audit Packet (A8) — current-state snapshot

**Arc**: PPUT-CCL (`PREREG_PPUT_CCL_2026-04-26.md` round-4 PASS/PASS + `PREREG_AMENDMENT_p0_defer_2026-04-25.md`).
**Date**: 2026-04-26.
**Authority**: ArchitectAI commit (Art. V.1.2). Input to dual external audit (Codex + Gemini) per Art. V.1.3 + memory `feedback_dual_audit`.

**Decision rule**: PASS → Phase B (kernel instrumentation + PPUT accounting) authorized; CHALLENGE → in-cycle fixes; VETO → Phase A redesign.

**FC-trace**: meta-witness across FC1 / FC2 / FC3 (atoms instrument all three subgraphs).

**Document split**: this packet is the **stable current-state snapshot** of Phase A at exit — it describes WHAT IS, not how it got here. The chronological history of audit rounds + their in-cycle fix bundles lives in the **companion** document `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` (append-only). Past audit transcripts are at `handover/audits/{CODEX,GEMINI}_PHASE_A8_EXIT_AUDIT_2026-04-26{,_R2,_R3,_R4,_R5,_R6,_R7,...}.md`. Reviewers needing closure-of-prior-finding context read the history doc; reviewers verifying current-state correctness against Phase B prerequisites read THIS packet.

---

## § 1. Phase A scope and atom map

Phase A = pre-flight (days 1–3 of the 30-day arc). Decomposed into 8 atoms (A0–A7); atom A8 is the dual external audit gate represented by this packet.

- **A0** (a–e): harness modernization.
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

(Subsequent audit-cycle in-cycle fix bundle commits are recorded in `A8_AUDIT_HISTORY_2026-04-26.md`.)

## § 2. Current-state metrics

| Metric | Value | Source / verification |
|---|---|---|
| `cargo test --workspace` PASS | **265** | re-runnable; all suites green |
| `cargo test --workspace` ignored | 29 | Phase B+ deferred stubs |
| `cargo test --workspace` failed | **0** | — |
| `python3 scripts/test_llm_proxy.py` | **16/16 PASS** | proxy routing + round-robin conformance (also wrapped by `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` so it runs every `cargo test --workspace`) |
| Trust Root manifest entries | **37** | `genesis_payload.toml [trust_root]` count + matches `trust_root_immutability::test_trust_root_manifest_includes_b2_b4_files` required-paths list |
| `boot::tests::verify_trust_root_passes_on_intact_repo` | **PASS** | re-hashes match the manifest |
| `bash scripts/smoke_siliconflow.sh` | **PASS (3/3 keys)** | live API; cost ~$0.005 per run |
| FC-trace anchor sites in `evaluator.rs` | **9** | grep `fc_trace::emit_event(`; 8 in `run_swarm` + 1 in `run_oneshot` |
| `make_pput` arity | **24 positional args** | refactor to builder pattern is a known Phase B+ deferred work item |

## § 3. Per-atom acceptance evidence (current state)

### A0 (harness modernization) — `62c4e14`
- 4 governance rules (R-014 / R-015 / R-018 / R-019), constitution-special-case judge.sh hook (R-016 fc_trace_in_commit), `tests/fc_alignment_conformance.rs` 17-witness battery + 9 ignored stubs, 5 cases C-071..C-075 (constitution amendment process / Veto-AI scope / ArchitectAI commit authority / FC-first / DO-178C tool qualification).
- Closed by dual-audit cycle; no open P0.

### A1 (PREREG amendment) — `6be6eb4`
- File: `handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md`.
- Substitutes `p_0 = 0.10` (PREREG § 5.5 ceiling) for the calibration-derived value at every Gate H consumer.
- **Statistical framing**: `0.10` is the **least-strict admissible value** the PREREG ceiling allows. `j-RR ≤ p_0` makes a SMALLER `p_0` stricter, so this substitution is operationally permitted but is NOT a tighter-than-original guarantee. No Type-I inflation since `j-RR` is descriptive (PREREG § 5.4), outside the inferential family. The substitution is operationally permitted at any phase including Phase E; if/when § 3 re-calibration conditions complete, calibration upgrades the bar.
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
- New `experiments/minif2f_v4/src/budget_regime.rs`. 4-variant `BudgetRegime` enum: `total_proposal` (default; preserves the prior baseline) / `per_agent` (loop bound = base × N) / `token_total` (declared; startup-fatal `UnimplementedRegime`) / `wall_clock` (declared; startup-fatal). 2 new non-Optional v2 fields: `budget_regime` + `budget_max_transactions`.
- `run_swarm` startup: env-resolved budget with startup-fatal error path on misconfiguration.
- **FC-trace**: FC2-N22 + FC1-N7.
- 16 unit tests; PREREG_AMENDMENT § 3 condition 3 cleared.

### A6 (FC tracing) — `89994c7`
- New `experiments/minif2f_v4/src/fc_trace.rs`. Pure stdlib (zero new deps). 7-variant `FcId` enum (FC1-N7 / FC1-N11 / FC1-N12 / FC1-E18 / FC2-N20 / FC2-N22 / FC3-N31). `FC_TRACE=1` gate cached in `OnceLock`; `FC_TRACE_FILE=<path>` redirects emit to file.
- **9 wired anchor sites** (8 in `run_swarm` + 1 in `run_oneshot`):
  - FC2-N22 synthetic short-circuit (swarm)
  - FC2-N20 mr tick (swarm)
  - FC2-N22 OMEGA full-proof accept (swarm)
  - FC2-N22 OMEGA per-tactic accept (swarm)
  - FC2-N22 natural MaxTxExhausted (swarm; carries `budget_regime` payload)
  - FC1-N12 verify bracket (oneshot)
  - FC1-N12 swarm `verify_omega_detailed` path "alone"
  - FC1-N12 swarm `verify_omega_detailed` path "tape+payload"
  - FC1-N12 swarm `verify_partial`
- Per-run correlation: `experiments/minif2f_v4/src/run_id.rs::mint_run_id` mints one identifier per run, threaded into both `emit_event` and `make_pput` so FC events join v2 jsonl rows by equality.
- **FC-trace**: meta-witness for the 5-step compile loop.
- 7 tests on `fc_trace` (6 unit + 1 end-to-end smoke `tests/fc_trace_smoke.rs` exercising `FC_TRACE=1` in a child process). 3 unit tests on `run_id`.

### A7 (SiliconFlow plumbing) — `90953d6`
- `src/drivers/llm_proxy.py`: OpenAI-compatible local HTTP server with per-provider multi-key round-robin. `detect_provider()` routing matrix:
  - explicit `provider:...` prefix → that provider (if known)
  - any slash-form id (HuggingFace style: `Qwen/...`, `openai/...`, `meta-llama/...`, `deepseek-ai/...`) → `siliconflow`
  - bare `"deepseek"` substring (no slash) → `deepseek` (api.deepseek.com)
  - bare `qwen3-*` / `qwen-*` (no slash) → `dashscope`
  - else → `dashscope` (default fallback)
- 3 SiliconFlow keys (primary / secondary / tertiary) split concurrent traffic across separate rate-limit pools — V3L-27 / case C-027 single-key N=30 collapse mitigation.
- `scripts/smoke_siliconflow.sh` + `_smoke_siliconflow.py`: 3 keys × 1 probe (Qwen2.5-7B-Instruct, max_tokens=8). Verified all 3 keys responding.
- `scripts/test_llm_proxy.py`: 16-test routing + round-robin conformance suite (no live API). Wrapped by `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` so the suite runs on every `cargo test --workspace`. The wrapper FAILS CLOSED if `python3` is missing; explicit opt-out `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1` (logged loudly).
- **FC-trace**: FC1-N7 (δ/AI provider expansion).
- Memory: `reference_siliconflow.md` records SiliconFlow as the Phase D heterogeneous lane (NOT a probe-only target).

## § 4. Phase B → C exit checklist (from PREREG_AMENDMENT § 4) — Phase A side

The PREREG amendment shifted the Phase B → C gate. From the Phase A perspective, the items it lists are now satisfied:

- ❌ p_0 calibration jsonl frozen (was REQUIRED) → **DEFERRED with substitution per amendment § 2**: `p_0 = 0.10` hardcoded at every Gate H consumer.
- ✅ B1–B7 + B7-extra mode toggle infrastructure complete (pre-Phase A baseline; round-4 PASS/PASS).
- ✅ Phase A0 harness modernization complete (`62c4e14`).
- ✅ Tools qualified per case C-075 (DO-178C tool qualification): `runner.sh`, `compute_p0.py`, evaluator boot enforcement, etc.
- ✅ Trust Root verifies clean (`boot::tests::verify_trust_root_passes_on_intact_repo` PASS at 37-entry manifest).

## § 5. Risks and known limitations entering Phase B

1. **`per_agent` budget regime untested at runtime**. A5 unit tests verify the scaling math (`base × N`) and env-coupled resolver. No live-LLM run with `BUDGET_REGIME=per_agent` has been smoked. Phase B kernel instrumentation will be the first opportunity to observe its behavior on a real problem.
2. **FC-trace coverage still partial**. 9 wired anchor sites cover HALT decomposition (FC2-N22 × 4 exit paths) + mr tick (FC2-N20) + Lean oracle scope (FC1-N12 × 4 sites: oneshot + swarm `verify_omega_detailed` × 2 + swarm `verify_partial`). Still NOT emitting: FC1-N7 prompt-build, FC1-N11 ∏p decision diversity (per-proposal), FC1-E18 preserve-Q_t (per ∏p=0), FC3-N31 WAL append. The `FcId` enum reserves these variants. Phase B+ kernel instrumentation should fill them in as the Phase B emit boundary lands.
3. **SiliconFlow rate-limit at scale**. A7 verified 3 keys responding individually at N=1 concurrency. V3L-27 demonstrates collapse at N=30 single-key. The v4 multi-key round-robin should triple the safe N envelope but the actual sweet spot for our hetero swarm is unmeasured. Phase D heterogeneous-batch design should land a `--max-concurrency` knob (currently `LLM_PROXY_CONCURRENCY=5` env in proxy) tuned per provider.
4. **Heterogeneous swarm = Phase D, not B/C**. Per F-2026-04-25-02 + the `agent_models.rs` `PHASE_D_HETERO_GATE_ENV_VAR` invariant, Phases B and C MUST stay single-model so ablation axes are not confounded. A7's plumbing exists for future Phase D work; Phase B uses the existing `deepseek-v4-flash` thinking-off backbone unchanged.
5. **`make_pput` arity = 24 (Phase B+ refactor)**. The positional-arg list is a maintainability hazard; refactor to builder pattern (`PputResultBuilder`) recommended early in Phase B. Non-blocking; runtime correctness verified by 4 in-binary tests + the integration battery.

## § 6. Specific questions for auditors

(All prior-round questions and their closures are recorded in `A8_AUDIT_HISTORY_2026-04-26.md` — see that doc's chronological round-N entries for the current count + verdicts. The questions below are for the current-round reviewer pass.)

### Q1 — Code correctness spot-checks
- (Q1.a) Does `experiments/minif2f_v4/src/bin/evaluator.rs` mint a single `run_id` at function entry and pass it to BOTH `fc_trace::emit_event` and `make_pput` so FC events and v2 jsonl rows share one identifier? Verify by reading `run_swarm` and `run_oneshot` entry points + `make_pput` signature.
- (Q1.b) Does `src/drivers/llm_proxy.py::detect_provider` route `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B` to `siliconflow` (not `deepseek`)? Verify by reading the function + checking that `scripts/test_llm_proxy.py` includes `test_deepseek_slash_form_routes_to_siliconflow`.
- (Q1.c) Does `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` fail closed when `python3` is missing? Verify by reading the test source (appended to this packet) + checking that the panic message describes the failure rather than a silent skip.

### Q2 — PREREG amendment soundness
- (Q2.a) `p_0 = 0.10` substitution wording: is `PREREG_AMENDMENT_p0_defer_2026-04-25.md` § 2 + § 8 internally consistent + statistically correct (least-strict admissible; no Type-I inflation; substitution operative at any phase including Phase E)?
- (Q2.b) Re-calibration conditions in § 3 are PRE-REQUISITES for calibration to run, not guarantees that calibration completes before any specific phase. Is this framed correctly without claiming calibration MUST run before Phase E?
- (Q2.c) The amendment's SHA-256 is in Trust Root. Does `boot::tests::verify_trust_root_passes_on_intact_repo` pass cleanly?

### Q3 — Atomicity, FC-trace discipline, governance
- (Q3.a) Each of A1–A7 was committed as one atomic commit with `FC-trace: <FC?-N?>` in the message. Verify by re-reading commit messages. Any commit missing FC-trace? Any commit that touched `constitution.md`?
- (Q3.b) 5 cases C-071..C-075 sediment 2026-04-25 session decisions as constitutional precedent. Are the rulings standalone-readable + correctly cross-referenced?
- (Q3.c) Trust Root manifest 37 entries. Are all entries load-bearing (i.e., does tampering each one weaken the constitutional gate)?

### Q4 — Phase A → B exit decision
- (Q4.a) Spot-check: re-run `cargo test --workspace`; expect 265 PASS / 29 ignored / 0 failed.
- (Q4.b) Spot-check: re-run `python3 scripts/test_llm_proxy.py`; expect 16/16 PASS.
- (Q4.c) Spot-check: re-run `bash scripts/smoke_siliconflow.sh`; expect PASS (3/3 keys; live API; cost ~$0.005).
- (Q4.d) Are there any open P0 defects from any prior round? Cross-reference `A8_AUDIT_HISTORY_2026-04-26.md` to verify each in-cycle fix bundle's closures hold against current source.
- (Q4.e) Phase B's first sub-atom is "JSONL schema v2 + C_i full-cost aggregator" (notepad). Are there any Phase A artifacts that would BLOCK that scope?

### Q5 — Packet/history split
- (Q5.a) Is the split between `A8_EXIT_PACKET` (stable current-state) and `A8_AUDIT_HISTORY` (append-only chronology) consistent with the project's existing pattern (constitution + Art. V.3 amendment log; PREREG + PREREG_AMENDMENT; TRACE_MATRIX_v0/v1/v2)?
- (Q5.b) Is the history doc append-only — past round entries describe what was true at that round's snapshot, without retroactive edits?
- (Q5.c) Does this packet contain ANY round-N retrospective text, fix-shipped headers, "previous round caught X" claims, or historical lineage like "(post-A8e F4)" / "(added by A8eN)" / "(chain position N via A8e)" — anywhere? It should NOT — those belong only in the history doc.

---

End your audit with:
- **VERDICT**: PASS / CHALLENGE / VETO
- **Conviction**: low / medium / high
- **Recommendation**: PROCEED to Phase B (kernel instrumentation + PPUT accounting) / FIX-THEN-PROCEED / REDESIGN

PASS = current-state evidence passes Phase B prerequisites + no open P0 from prior rounds + no new substantive findings. CHALLENGE = correctable in one cycle. VETO = Phase A redesign required.


---

# Audit history (append-only chronology)

# A8 Phase A → B Exit Audit — Chronological History

**Companion**: `A8_EXIT_PACKET_2026-04-26.md` (canonical current-state). This doc is the **append-only** chronology of round-N outcomes + the in-cycle fixes that closed each round's findings. Past entries are FROZEN — corrections to historical facts go in a new "Correction (round-M)" sub-section under the affected round, never via in-place edit.

**Constitutional pattern**: mirrors `constitution.md` + Art. V.3 amendment log; mirrors `PREREG_PPUT_CCL_2026-04-26.md` (frozen) + `PREREG_AMENDMENT_p0_defer_2026-04-25.md` (delta). Per C-075 (DO-178C tool qualification) the gate machinery itself must be qualifiable; this history doc is in Trust Root so the chronology cannot be silently rewritten.

**Why this doc exists** (A8e7 structural rewrite, 2026-04-26): rounds 1–6 of the Phase A → B audit produced a recurring documentary CHALLENGE pattern. Diagnosis: `A8_EXIT_PACKET` was conflating two opposite temporal modes (stable-snapshot artifact + append-only chronology) in one document, so every fix to "current state" generated new staleness in "round-N retrospective" sections. Splitting the two modes into two documents removes the category error.

---

## Round 1 (2026-04-26) — first dual review

**Inputs**:
- Packet: `handover/audits/A8_EXIT_PACKET_2026-04-26.md` @ commit `60292dc`
- Test baseline: 261 PASS / 29 ignored / 0 failed
- Trust Root: 30-entry manifest (round-1 packet's count; later corrected to 31 — see round-2)

**Verdicts**:
- Codex R1: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26.md`
- Gemini R1: **VETO / high** — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26.md`
- Merged (per memory `feedback_dual_audit_conflict`, VETO > CHALLENGE > PASS): **VETO**

**Findings** (5 Codex + 4 Gemini; 1 Gemini Q1 finding about hybrid_v1 was determined invalid — pre-Phase A change):
1. (Codex#2 + Gemini Q4 — convergent) `run_corr_id` (FC events) vs `make_pput::run_id` (jsonl) ms drift; Phase D consumers cannot join FC events to v2 rows by equality.
2. (Codex#3) FC1-N12 emitted only in `run_oneshot`; the 2 `verify_omega_detailed` and 1 `verify_partial` calls in `run_swarm` increment `verifier_wait_ms` but don't emit FC events.
3. (Codex#4 + Gemini Q5 — convergent) `detect_provider` routing matrix: `Qwen/Qwen2.5-7B-Instruct` misroutes to DashScope; smoke bypasses proxy so the bug isn't caught.
4. (Codex#5) Trust Root manifest count off-by-1 (packet says 30, actual 31); genesis_payload.toml header still says "Total: 25 files".
5. (Codex#1) `PREREG_AMENDMENT § 2` calls `p_0=0.10` substitution "strictest possible substitute" — backwards (0.10 is least-strict admissible for `j-RR ≤ p_0`).
6. (Gemini Q4) Hand-rolled JSON encoder in `fc_trace.rs` vs `serde_json` already in deps — minor.
7. (Gemini Q5) Smoke test bypasses proxy so routing logic untested — convergent with #3.
8. (**Gemini VETO Q6**) Multi-key round-robin in `llm_proxy.py` (V3L-27 mitigation) lacks any automated conformance test. Manual verification via `[2,2,2]` distribution is one-time, not recurring. **REDESIGN-level for atom A7.**

**Round-1 fixes shipped (`A8e`, commit `5a56ff6`)** — six fixes:
- **F1** unified `run_id`: new `experiments/minif2f_v4/src/run_id.rs::mint_run_id` minted at run_swarm/run_oneshot entry; threaded into both `emit_event` and `make_pput`. Closes #1.
- **F2** new `scripts/test_llm_proxy.py` 15-test suite pinning round-robin `[2,2,2]` invariant. Closes Gemini VETO Q6 at the artifact level.
- **F3** `detect_provider` reordered: slash-form ⇒ siliconflow first; bare-qwen ⇒ dashscope only without slash. Closes #3.
- **F4** added FC1-N12 emit to all 3 swarm verify sites (`verify_omega_detailed` × 2 + `verify_partial`). Closes #2.
- **F5** Trust Root count + genesis_payload.toml header reconciled to 31 entries (corrected for A3's `agent_models.rs` already in TR before A1). Closes #4.
- **F6** PREREG_AMENDMENT § 2 reworded: `p_0=0.10` → least-strict admissible ceiling; explicit Type-I implications paragraph added. Closes #5.

---

## Round 2 (2026-04-26) — post-A8e

**Inputs**:
- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `5a56ff6` (post-A8e)
- Test baseline: 264 PASS / 29 ignored / 0 failed (Rust); 15/15 PASS (Python proxy tests)
- Trust Root: 33-entry manifest

**Verdicts**:
- Codex R2: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R2.md`
- Gemini R2: **CHALLENGE / high** (de-escalated from VETO) — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R2.md`
- Merged: **CHALLENGE**. No VETO. Both auditors confirmed round-1 fixes are letter-correct; remaining gaps procedural/documentary.

**Findings** (3 Codex + 2 Gemini, convergent):
1. (Codex R2#1 + Gemini R2#1) F2's `test_llm_proxy.py` exists but is documented as manual-only; no CI integration. Tests-not-running-automatically = documentation, not gates. Closes Gemini's round-1 VETO at the *artifact* level but NOT at the *process* level.
2. (Codex R2#2) PREREG_AMENDMENT § 2 was corrected (F6) but § 8 audit-requirements paragraph still says "strictest plausible bar is conservative" — direct contradiction with § 2.
3. (Codex R2#3 + Gemini R2#2) Packet § 5 Risk #5 ("No FC1-N12 emit in run_swarm verify path") is stale (closed by F4); packet § 6 Q7.a/b numbers stale (261/30 vs actual 264/33); TRACE_MATRIX has stale `run_corr_id` symbol row + "in CI" claim.

**Round-2 fixes shipped (`A8e2`, commit `0af47b7`)** — three fixes:
- **G1** `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` Rust integration test that shells to `python3 scripts/test_llm_proxy.py`; runs on every `cargo test --workspace`. Closes finding #1.
- **G2** PREREG_AMENDMENT § 8 reworded to remove "strictest plausible bar"; consistent with § 2 throughout. Closes finding #2.
- **G3** Packet § 2 cumulative table extended; § 5 Risk #5 removed; § 6 Q7.a/b numbers updated 264/34; TRACE_MATRIX `run_corr_id` row replaced with unified `run_id` row; "in CI" softened. Closes finding #3.

---

## Round 3 (2026-04-26) — post-A8e2

**Inputs**:
- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `0af47b7` (post-A8e2)
- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 15/15 PASS (Python)
- Trust Root: 34-entry manifest

**Verdicts**:
- Codex R3: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R3.md`
- Gemini R3: **CHALLENGE / high** — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R3.md`
- Merged: **CHALLENGE**. Both auditors flagged narrow doc/source-comment defects; no VETO.

**Findings**:
1. (Codex R3#1) Packet line 118 still says substitution is "Mathematically conservative (strictest plausible bar)"; Q2.a still says "described as conservative"; genesis_payload.toml header still says "conservative ceiling". The § 2 / § 8 fix in F6/G2 didn't propagate to these other sites.
2. (Codex R3#2) Packet § 3 A6 atom description says "6 anchor sites + 1 oneshot"; § 6 Q4.a says "FC1-N12 (oneshot verify only)"; § 6 Q4.d describes pre-F1 ms drift as if open. Stale relative to F4 + F1.
3. (Codex R3#3) `tests/llm_proxy_python_conformance.rs` returns success when `python3` is missing (soft skip via `eprintln + return`). For a VETO-closing conformance gate, missing python3 should fail closed.
4. (Gemini R3#1) Convergent with Codex R3#2 — Q4.d stale.
5. (Gemini R3#2 — non-blocking observation) `make_pput` signature has 21 args; refactor to builder pattern recommended for Phase B+.

**Round-3 fixes shipped (`A8e3`, commit `3d38ba5`)** — six fixes:
- **H1** Packet § 3 A1 description rewritten: "Mathematically conservative" → "least-strict admissible value" with Type-I implications + cross-ref to § 2.
- **H2** Packet § 3 A6 description bumped 6 → 9 anchor sites; explicitly listed F4-added swarm sites.
- **H3** Packet § 6 Q2.a / Q4.a / Q4.d marked **CLOSED** with closure rationale.
- **H4** `genesis_payload.toml` TR header phrasing about A1: "conservative ceiling" → "max-tolerated ceiling — least-strict admissible".
- **H5** TRACE_MATRIX § 5 item 7: "(commit pending) + 6 wired" → "CLOSED + 9 wired".
- **H6** G1 wrapper test now FAILS CLOSED on missing `python3`; explicit opt-out `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1` for deliberate downgrades.

---

## Round 4 (2026-04-26) — post-A8e3

**Inputs**:
- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `3d38ba5` (post-A8e3)
- Test baseline: 265 PASS / 29 ignored / 0 failed; 15/15 Python
- Trust Root: 34-entry manifest

**Verdicts**:
- Codex R4: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R4.md`
- Gemini R4: **PASS / high** — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R4.md`. *First round one auditor reached PASS.*
- Merged: **CHALLENGE**. Conservative merge wins per memory.

**Findings** (3 Codex + 1 Gemini non-blocking):
1. (Codex R4#1) Packet title + date metadata still say "round 2"; per-atom Trust Root deltas in § 3 stale (A5 says 25→26 vs actual 26→27, A6 26→27 vs 27→28, A7 27→30 vs 28→31).
2. (Codex R4#2 + R4#3) TRACE_MATRIX § 1 line 11 says "Six anchor sites wired in run_swarm" but 1 site is in run_oneshot.
3. (Codex R4#3) `tests/llm_proxy_python_conformance.rs` file header docstring still says "if not, it skips with a clear diagnostic" — contradicts H6's fail-closed runtime.
4. (Gemini R4 non-blocking) `make_pput` arg count is now 24 not 21 (F1 added run_id parameter). Reaffirms the round-3 deferred refactor.

**Round-4 fixes shipped (`A8e4`, commit `8693789`)** — three fixes:
- **I1** Packet metadata "round 2" → "running through rounds 1–N" with reader pointer to latest section.
- **I2** Per-atom Trust Root deltas in packet § 3 corrected: A5 → 26→27, A6 → 27→28, A7 → 28→31. TRACE_MATRIX § 1 line 11 anchor-site location corrected: "5 in run_swarm + 1 in run_oneshot".
- **I3** Wrapper docstring updated: "FAILS CLOSED with a clear diagnostic"; explicit opt-out env var documented.

---

## Round 5 (2026-04-26) — post-A8e4

**Inputs**:
- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `8693789` (post-A8e4)
- Test baseline: 265 PASS / 29 ignored / 0 failed; 15/15 Python
- Trust Root: 34-entry manifest

**Verdicts**:
- Codex R5: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R5.md`
- Gemini R5: **PASS / high** — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R5.md`
- Merged: **CHALLENGE**. Same split as R4.

**Findings** (2 Codex + 1 Gemini non-blocking):
1. (Codex R5#1) Packet missing Round-4 outcome + A8e4 fixes shipped section; `<pending>` commit placeholders for A8e2/A8e3/A8e4 never replaced.
2. (Codex R5#2) Packet § 6 Q6 round-1 question text still says "24 → 30 / 6 new entries" (pre-F5 count); TRACE_MATRIX top-bullet TR-deltas A5 "25→26" / A6 "26→27" still stale (only the per-atom paragraphs were corrected in I2, not the bullet headers).
3. (Gemini R5 non-blocking) `make_pput` arg count text in round-3 retrospective still says "21 positional args"; should be 24.

**Round-5 fixes shipped (`A8e5`, commit `1622017`)** — five fixes:
- **J1** Round-4 outcome + A8e4 fixes shipped section added to packet.
- **J2** `<pending>` placeholders replaced with actual SHAs (`0af47b7` / `3d38ba5` / `8693789`); A8e5 self-reference uses "this commit".
- **J3** Packet § 6 Q6 question text reworded to "24 → 34 / 10 new entries" with full enumerated list.
- **J4** TRACE_MATRIX A5/A6/A7 top-bullet TR-deltas corrected (matches canonical chain in genesis_payload.toml header).
- **J5** Round-3 retrospective `make_pput` arg count: 21 → 24.

---

## Round 6 (2026-04-26) — post-A8e5

**Inputs**:
- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `1622017` (post-A8e5)
- Test baseline: 265 PASS / 29 ignored / 0 failed; 15/15 Python
- Trust Root: 34-entry manifest

**Verdicts**:
- Codex R6: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R6.md`
- Gemini R6: **CHALLENGE / high** — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R6.md`
- Merged: **CHALLENGE**. Convergence regression vs R4/R5 (Gemini went from PASS back to CHALLENGE).

**Findings** (3 Codex + 2 Gemini; ONE real correctness bug):
1. (Codex R6#1) RQ14 closure criterion contradicts itself — packet has `<pending>` placeholders in J2 bullet + RQ14 itself.
2. (**Codex R6#2 — REAL BUG**) `detect_provider` misroutes HuggingFace-style `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B` to `api.deepseek.com` because `"deepseek" in m` wins before the slash check. The official DeepSeek API only serves bare `deepseek-chat` / `deepseek-v4-flash`, not Distill variants — would 404 on first invocation.
3. (Codex R6#3) TRACE_MATRIX A8e3 row still says "21 positional args" while packet was corrected to 24 in J5.
4. (Gemini R6#1) TRACE_MATRIX A6 line leads with "Six anchor sites wired" then immediately self-contradicts with "9 post-A8e total".
5. (Gemini R6#2) Packet § 3 A5/A6 fix attribution: "A8e3 fix H4 corrected" — H4 corrected the genesis_payload.toml comment; the packet § 3 numbers were corrected by I2 (A8e4).

**Round-6 fixes shipped (`A8e6`, commit `dbcf53a`)** — five fixes (1 real correctness + 4 doc):
- **K1** `<pending>` placeholder closure: J2 bullet + RQ14 reworded.
- **K2 (real bug)** `src/drivers/llm_proxy.py::detect_provider` reordered: slash-form is now FIRST routing heuristic. New `test_deepseek_slash_form_routes_to_siliconflow` in `scripts/test_llm_proxy.py` pins the regression. **16/16 Python proxy tests PASS** (was 15).
- **K3** TRACE_MATRIX A8e3 row: "21 positional args" → "was 21 at round-3; F1 added run_id, post-A8e count is 24".
- **K4** TRACE_MATRIX A6 row top-line: "Six anchor sites wired" → "9 anchor sites wired (8 in run_swarm + 1 in run_oneshot)" with original-6 + F4-added-3 breakdown.
- **K5** Packet § 3 A5/A6 fix attribution: "A8e3 fix H4 corrected" → "A8e4 fix I2 corrected".

---

## Round 7 (2026-04-26) — post-A8e7 structural rewrite

**Trigger for A8e7**: rounds 2–6 produced a recurring documentary CHALLENGE pattern that single-point fixes could not close. **Diagnosis**: `A8_EXIT_PACKET` was conflating stable-snapshot artifact + append-only chronology in one document. **Fix (`A8e7`, commit `aaedc9d`)**: split into two docs following the project's existing constitutional pattern (stable spec + delta log; cf. constitution.md + Art. V.3 amendment log; PREREG + PREREG_AMENDMENT; TRACE_MATRIX_v0 + v1 + v2). The packet becomes current-state only; THIS history doc becomes append-only chronology.

**Inputs**:
- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `aaedc9d` (post-A8e7)
- Test baseline: 265 PASS / 29 ignored / 0 failed; 16/16 Python proxy tests
- Trust Root: 35-entry manifest (A8_AUDIT_HISTORY added)

**Verdicts**:
- Codex R7: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R7.md`
- Gemini R7: **CHALLENGE / high** — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R7.md`
- Merged: **CHALLENGE**. Both auditors agree the split pattern is right; the implementation left lineage text in the packet that should have moved to the history doc only.

**Findings** (4 Codex + 2 Gemini):
1. (Codex R7#1) Packet § 2 + § 4 still report 34 Trust Root entries; actual is 35 after A8e7 added `A8_AUDIT_HISTORY_2026-04-26.md`. Stale reference at multiple sites in the packet.
2. (Codex R7#2) This history doc says A8e7 commit is `<this commit's SHA>` and "Round 7 dual audit pending after this commit lands" — but A8e7 is already in Trust Root as landed. Same placeholder-staleness pattern earlier rounds tried to eliminate.
3. (**Codex R7#3 + Gemini R7#1 — convergent**) Packet § 1 + § 3 still contain historical lineage text: "(post-A8e F6 + A8e2 G2)", "(added by A8e F4)", "(+ A8e fix F4)", "A8e6 fix K2", "(chain position 33 via A8e)". The A8e7 split's intent was that the packet describes WHAT IS (current state) without explaining HOW (round-N derivation); HOW belongs only in this history doc.
4. (**Codex R7#4 — substantive logic finding**) PREREG_AMENDMENT § 2 vs § 8 internal contradiction:
   - § 2 (b) claims "the conditions in § 3 ensure calibration runs *before* Phase E, so the loose substitution never reaches the artifact-acceptance moment"
   - § 3 lists 5 PRE-REQUISITES for calibration to run AT ALL (not guarantees of calibration completing before Phase E)
   - § 8 says "if those conditions never met, Phase E proceeds with the operationally-permitted ceiling substitution"
   The amendment claims both "calibration must run before Phase E" AND "ceiling-substitution is acceptable at Phase E without calibration". Cannot be both true.
5. (Gemini R7#2) Audit runner scripts don't append `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` to source files — Q1.c (verify fail-closed) cannot be verified from the packet alone.

**Round-7 fixes shipped (`A8e8`, commit `857872e`)** — five fixes:
- **M1** (Codex R7#1): Packet TR count bumped 34 → 35 wherever cited.
- **M2** (Codex R7#2): This history doc's A8e7 entry now stamps the actual SHA `aaedc9d` and round-7 verdicts (above) instead of "pending". Round-7 history sealed.
- **M3** (Codex R7#3 + Gemini R7#1): Packet rewritten to remove ALL historical lineage text. The packet describes the post-A8e6 state directly — e.g. "9 wired FC-trace anchor sites" with no breakdown of which fix added which. The HOW lives ONLY in this history doc.
- **M4** (Codex R7#4): PREREG_AMENDMENT § 2 + § 3 + § 8 reconciled: removed the false claim that § 3 "ensures calibration before Phase E"; § 8 now reads as the operative rule (Phase E proceeds with the substitution if § 3 conditions haven't completed). Clean single semantics: substitution is permitted throughout Phase B → Phase E; calibration UPGRADES the bar IF and WHEN the § 3 conditions all complete. Re-hashed in Trust Root.
- **M5** (Gemini R7#2): Audit runner scripts now append `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` so Q1.c is verifiable from the packet bundle alone.

---

## Round 8 (2026-04-26) — post-A8e8

**Inputs**:
- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `857872e` (post-A8e8)
- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 16/16 PASS (Python)
- Trust Root: 35-entry manifest

**Verdicts**:
- Codex R8: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R8.md`
- Gemini R8: **PASS / high** → PROCEED — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R8.md`
- Merged: **CHALLENGE**. Conservative merge wins per memory; substantive Codex finding (PREREG § 8 stale claim parallel to M4-fixed § 2) requires closure.

**Findings** (3 Codex; Gemini found no defects):
1. (**Codex R8#1 — substantive**) PREREG_AMENDMENT § 8's audit-requirements text still contains the stale claim "Gate H is Phase E and § 3 conditions ensure calibration runs first" — round-7 M4 fixed § 2(b)'s identical false claim but missed the parallel text in § 8. Logical contradiction with the post-M4 § 2 + § 3 framing.
2. (Codex R8#2) This history doc's round-7 entry left an unresolved placeholder `<aaedc9d-successor>` for the A8e8 commit SHA; cumulative table row 7 still says "pending pending pending" despite round-7 verdicts being recorded above.
3. (Codex R8#3) Audit runner scripts default `A8_AUDIT_ROUND=R2` (oldest fallback) and emit pre-A8e header metadata ("261 PASS / 30-entry manifest") — re-running the runners regenerates stale audit artifacts.

**Round-8 fixes shipped (`A8e9`, this commit)** — three fixes:
- **N1** (Codex R8#1): PREREG_AMENDMENT § 8 reworded to remove the residual "ensure calibration runs first" claim. Now consistent with § 2's least-strict-admissible framing across all three sections.
- **N2** (Codex R8#2): Round-7 entry stamped with actual A8e8 SHA `857872e`; cumulative table row 7 sealed with the actual round-7 verdicts (CHALLENGE/CHALLENGE) and finding count (4 Codex + 2 Gemini, including 1 substantive PREREG logic finding); round-8 row added with this round's verdicts.
- **N3** (Codex R8#3): Runner script header metadata refreshed to current state (265 PASS / 35-entry manifest); pre-A8e values removed.

---

## A8e12 (2026-04-26) — harness amplifier, no audit round

**Trigger**: not a fix-bundle responding to a specific dual-audit round. After round 10 the user directly directed (中文 "C，更新你的harness，加入判例") — option C from the round-10 reflection. The fix is constitutional discipline for the false-closure class that R8/R9 caught (N1 § 8 parallel miss + N3 false-claim). Sediments the lesson into binding precedent + runtime warn rule.

**Inputs**:
- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `b3b15a4` (post-A8e11, pre-R11)
- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 16/16 PASS (Python)
- Trust Root: 37-entry manifest

**Verdicts**: N/A (no audit run; this is harness setup pre-R11)

**A8e12 fixes shipped (commit `0170a99`)** — three threads (1 case + 1 rule + 1 self-audit):
- **Case C-076** `cases/C-076_commit_claim_diff_parity.yaml` — false-closure precedent. Rules: every commit asserting ≥2 distinct fix items requires per-claim `Verified:` proof line; multi-section parity check; audit transcripts append-only per C-075; R-020 enforces at pre-commit.
- **Rule R-020** `rules/active/R-020_commit_claim_diff_parity.yaml` + inline implementation in `.claude/hooks/judge.sh` (sibling of R-016 fc_trace_in_commit). WARN at pre-commit on multi-fix-bundle messages (≥2 fix tags or bullets) without `Verified:` lines.
- **Self-audit pass** before R11 — caught 4 documentary defects: runner header `35-entry` → `37-entry`, genesis comment "14 rules" → "15 rules", and regenerated cases/MANIFEST + rules/MANIFEST + judge.sh re-hashes. C-076 yaml + R-020 yaml are covered by their respective MANIFEST proxies (Trust Root manifest unchanged at 37 entries).

---

## Round 9 (2026-04-26) — post-A8e9

**Inputs**:
- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `6f327b0` (post-A8e9)
- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 16/16 PASS (Python)
- Trust Root: 35-entry manifest

**Verdicts**:
- Codex R9: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R9.md`
- Gemini R9: **PASS / high** → PROCEED — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R9.md`. *Second consecutive Gemini PASS with full Q1-Q5 spot-check verification + zero new findings.*
- Merged: **CHALLENGE**. Conservative merge.

**Findings** (2 Codex; Gemini 0):
1. (**Codex R9#1 — false-closure**) A8e9 fix N3 claimed "runner default A8_AUDIT_ROUND updated" but the source still defaulted to `R2`. The actual N3 implementation only refreshed header metadata, not the default. Re-running either runner without env still targeted `_R2`, silently overwriting the round-2 transcript.
2. (Codex R9#2) Packet § 6 says closures are recorded for "round-1..round-7" but history now contains round 8 (and round 9 about to land). Documentary drift class.

**Round-9 fixes shipped (`A8e10`, this commit)** — two fixes:
- **O1** (Codex R9#1): Both runner scripts now make `A8_AUDIT_ROUND` env var REQUIRED. No silent default; fail fast with usage message. Additionally, both runners refuse to overwrite an existing transcript at the resolved output path (round-N transcripts are append-only governance artifacts per C-075). Verified: invoking either without the env var prints the usage message and exits 2; invoking with `A8_AUDIT_ROUND=R2` (where the R2 transcript already exists) prints the overwrite refusal and exits 2.
- **O2** (Codex R9#2): Packet § 6 pointer reworded from "round-1..round-7" to "all prior rounds — see chronological round-N entries in history doc for current count". No longer ages.

---

## Round 10 (2026-04-26) — post-A8e10

**Inputs**:
- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `92d30f7` (post-A8e10)
- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 16/16 PASS (Python)
- Trust Root: 35-entry manifest

**Verdicts**:
- Codex R10: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R10.md`
- Gemini R10: **PASS / high** → PROCEED — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R10.md`. *Third consecutive Gemini PASS.*
- Merged: **CHALLENGE**. Codex caught two real defects in gate machinery (smoke + runner Trust Root coverage); Gemini did not.

**Findings** (2 Codex; Gemini 0):
1. (**Codex R10#1 — substantive**) `_smoke_siliconflow.py` doesn't fail closed on missing keys: the script skipped unset secondary/tertiary and returned PASS if any configured key responded. The packet's "PASS (3/3 keys)" claim was strictly stronger than what the smoke verified — a primary-only setup would silently PASS, replicating exactly the V3L-27 single-key collapse pattern the 3-key pool was meant to mitigate.
2. (**Codex R10#2 — substantive**) Audit runner scripts (`run_codex_phase_a8_exit_audit.sh` + `run_gemini_phase_a8_exit_audit.py`) are load-bearing gate tools (demonstrated by R8/R9's runner-default + false-closure defects) but were NOT in Trust Root. Per case C-075 (DO-178C tool qualification): tools producing data used by future dual audit must enter Trust Root.

**Round-10 fixes shipped (`A8e11`, this commit)** — two fixes (both substantive):
- **P1** (Codex R10#1): `_smoke_siliconflow.py` reworked to require ALL THREE keys configured AND respond. Missing OR failing key = FAIL (exit 1). Explicit opt-out `SILICONFLOW_SMOKE_ALLOW_PARTIAL=1` for credential-rotation testing returns exit 3 (partial-PASS, callers can distinguish). Verified: full-keys → exit 0; primary-only → exit 1 with diagnostic; primary-only + ALLOW_PARTIAL=1 → exit 3.
- **P2** (Codex R10#2): `run_codex_phase_a8_exit_audit.sh` + `run_gemini_phase_a8_exit_audit.py` added to Trust Root manifest. `trust_root_immutability` test required-paths list extended. Manifest 35 → 37.

---

## Round 11 (2026-04-26) — post-A8e12

**Inputs**:
- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `0170a99` (post-A8e12)
- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 16/16 PASS (Python)
- Trust Root: 37-entry manifest

**Verdicts**:
- Codex R11: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R11.md`
- Gemini R11: **CHALLENGE / high** — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R11.md`. *First time both auditors converged on multiple substantive findings since R7.*
- Merged: **CHALLENGE**. Two substantive findings (one architectural, one refactor) plus four documentary.

**Findings** (4 Codex + 3 Gemini):
1. (**Codex R11#1 — substantive architectural**) Trust Root proxy via `cases/MANIFEST.sha256` + `rules/MANIFEST.sha256` is documented as covering the child yaml files but not actually enforced at boot. `verify_trust_root` only hashes the flat `[trust_root]` entries; the manifest's child contents are not verified. A direct edit to a case or rule yaml without regenerating its parent manifest leaves boot still passing — silent governance drift.
2. (Codex R11#2) `A8_AUDIT_HISTORY` had not been updated to record A8e12 + round-11 entries; per the A8e7 split discipline these belong in the chronology doc.
3. (Codex R11#3) `TRACE_MATRIX_v2` § 5 still listed R-020 as `ground_truth_label` (the original A0a planning name; A8e12 took the slot for `commit_claim_diff_parity`); § 6 still said "5 cases C-071..C-075" (now 6 with C-076) and "14 rules" (now 15 with R-020).
4. (Codex R11#4) `A8_AUDIT_HISTORY` cumulative metrics paragraph had a contradiction: claimed "11 substantive findings" but the enumerated list ended with "= 9 real bugs".
5. (Gemini R11#1) `TRACE_MATRIX_v2` § 1 contains audit-fix-bundle retrospectives (A8e..A8e11 entries) that — per Gemini's reading of A8e7 — should live exclusively in `A8_AUDIT_HISTORY`.
6. (Gemini R11#2) Same critique applied to `genesis_payload.toml` header progression chain ("A0=24 → A1=25 → ...").
7. (**Gemini R11#3 — substantive refactor**) `make_pput` function signature uses `Option<u64>` for `total_run_token_count` / `failed_branch_count` / `total_wall_time_ms`, but every caller passes `Some(...)` and the body unwraps with `.unwrap_or(0)`. The mid-term P0-B fix collapsed the v2 struct fields to non-Option but didn't follow through to the function-parameter contract. Code clarity issue, not a runtime bug.

**Round-11 fixes shipped (`A8e13`, this commit)** — six fixes (2 substantive + 4 documentary):
- **Q1** (Codex R11#1 — substantive arch): `src/boot.rs` extended with `verify_child_manifest`. When a Trust Root entry path ends in `/MANIFEST.sha256`, the verifier now ALSO parses the manifest contents (GNU sha256sum format) and verifies each child file's actual hash against the claim. Manifests regenerated with repo-relative paths (was a mix of bare-filename and dir-relative). Two new unit tests pin the contract (`verify_trust_root_detects_child_manifest_tamper` + `verify_trust_root_passes_with_matching_child_manifest`). Plus: `src/boot.rs` itself added to Trust Root (meta-finding caught during fix — the verifier file was not previously protected by the verifier; tampering with boot.rs would silently bypass the entire gate). Manifest 37 → 38.
- **Q2** (Gemini R11#3 — substantive refactor): `make_pput` signature `Option<u64>` → plain `u64` for the 3 affected parameters. All 9 production call sites + 4 test fixtures updated. `wc.elapsed_ms()` returns `Option<u64>` upstream; call sites add `.unwrap_or(0)` at the boundary. `cargo test --workspace`: 267 PASS (was 265 + 2 new boot tests).
- **Q3** (Codex R11#2): `A8_AUDIT_HISTORY` extended with A8e12 entry + Round 11 outcome + A8e13 fixes shipped section + cumulative table rows for round 11/12.
- **Q4** (Codex R11#3 + Gemini R11#1+#2 partial): `TRACE_MATRIX_v2` § 1 audit-fix-bundle entries left as-is (the project's TRACE_MATRIX has always been a delta-log; trying to remove these introduces churn without solving the underlying tension). Added a footer note explaining the doc's append-only-delta-log nature with cross-ref to `A8_AUDIT_HISTORY` for round-by-round attribution. `genesis_payload.toml` header progression chain similarly kept (it documents the manifest's growth lineage which is referenced by the rules-MANIFEST proxy comment) with a cross-ref pointer added.
- **Q5** (Codex R11#3 specific items): `TRACE_MATRIX_v2` § 5 item 5 reworded — R-020 ground_truth_label slot was reassigned to `commit_claim_diff_parity` in A8e12; future ground-truth-label rule will be re-numbered. § 6 "5 cases" → "6 cases (C-071..C-076)"; "4 active rules" → "5 active (R-014/R-015/R-018/R-019/R-020)".
- **Q6** (Codex R11#4): cumulative metrics paragraph reworded — "11 substantive findings" enumeration extended to 13 (was missing R10's P1+P2 = 2 findings; now lists all explicitly).

---

## Cumulative metrics

| Round | Codex | Gemini | Merged | New findings | Real-bug findings | API cost (~$) |
|---|---|---|---|---|---|---|
| 1 | CHALLENGE | **VETO** | VETO | 9 | 5 | ~5 |
| 2 | CHALLENGE | CHALLENGE | CHALLENGE | 5 | 0 | ~5 |
| 3 | CHALLENGE | CHALLENGE | CHALLENGE | 5 (incl. 1 non-blocking) | 1 (H6 fail-closed) | ~5 |
| 4 | CHALLENGE | PASS | CHALLENGE | 4 (incl. 1 non-blocking) | 0 | ~5 |
| 5 | CHALLENGE | PASS | CHALLENGE | 3 (incl. 1 non-blocking) | 0 | ~5 |
| 6 | CHALLENGE | CHALLENGE | CHALLENGE | 5 | **1 (K2 routing)** | ~5 |
| 7 | CHALLENGE | CHALLENGE | CHALLENGE | 6 | **1 (M4 PREREG § 2 logic)** | ~5 |
| 8 | CHALLENGE | **PASS** | CHALLENGE | 3 | **1 (N1 PREREG § 8 logic)** | ~5 |
| 9 | CHALLENGE | **PASS** | CHALLENGE | 2 | 0 (false-closure caught — N3 was incomplete; no new substantive bugs) | ~5 |
| 10 | CHALLENGE | **PASS** | CHALLENGE | 2 | **2 (P1 smoke false-PASS + P2 runners not in TR)** | ~5 |
| 11 | CHALLENGE | CHALLENGE | CHALLENGE | 7 | **2 (Q1 boot.rs proxy not enforced + Q2 make_pput Option divergence)** | ~5 |
| 12 | pending | pending | pending | — | — | ~5 |

Cumulative cost so far ~$70 (12 rounds × ~$5–7 — note A8e12 was a no-audit harness amplifier between R10 and R11); within memory `feedback_dual_audit` Phase A reservation. **Real-bug yield: 13 substantive findings caught + closed across 11 rounds**: 5 routing/correctness in R1 (run_id ms drift / FC1-N12 swarm gap / Qwen-HF misroute / TR count off-by-1 / "strictest" wording reversed) + 1 fail-closed-gate in R3 (H6 wrapper soft-skip) + 1 routing collision in R6 (deepseek-ai/* misroute) + 1 PREREG § 2 logic in R7 (M4) + 1 PREREG § 8 logic in R8 (N1 — parallel-text miss) + 2 in R10 (P1 smoke false-PASS / P2 runners not in TR) + 2 in R11 (Q1 boot.rs proxy not enforced / Q2 make_pput Option<...> divergence). Plus 1 false-closure caught at R9 (N3 claimed runner default fix that wasn't shipped) — counted separately as a delivery-quality finding, not a system bug. The recurring documentary CHALLENGE class persisted longer than expected because each round's fix touched documentation in ways that left adjacent staleness; the A8e7 structural rewrite addressed the root cause (category error) but its implementation needed two more cycles (A8e8 + A8e9) to fully complete the lineage strip + cross-section consistency. **A8e12** added case C-076 + rule R-020 (commit-claim diff parity) as a Living Harness amplifier specifically targeting the false-closure / parallel-miss class — pre-commit WARN reminds the committer to grep-verify every claim against the actual diff.


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

**`p_0` for guardrail purposes**: take the **PREREG § 5.5 ceiling itself = 0.10** and use it as `p_0` directly. Any artifact j whose `j-RR` regression rate exceeds 0.10 fails Gate H per the original guardrail logic.

**Wording correction (A8 round-1 audit, Codex finding 1)**: an earlier draft of this section claimed `p_0 = 0.10` was the *strictest possible substitute*. That was backwards. The Gate H test is `j-RR ≤ p_0`, so a SMALLER `p_0` is stricter. `0.10` is the **maximum tolerated value** the PREREG admits — i.e., the LEAST strict admissible ceiling, not the strictest. Substituting it is the equivalent of running Gate H at the weakest threshold the PREREG ever sanctioned. The substitution is *operationally permitted* (the PREREG explicitly allows up to 0.10), but it is NOT a tighter-than-original guarantee.

**Statistical implications** (re-stated cleanly):
- No Type-I inflation. `j-RR` is a descriptive guardrail (PREREG § 5.4), not part of the inferential family — its threshold doesn't enter Holm-Bonferroni adjustment.
- The substitution may be LESS protective than an eventual calibrated `p_0 < 0.10`. If/when calibration runs (per § 3 conditions, which are PRE-REQUISITES — they do not by themselves *guarantee* calibration completes before any specific phase), the empirical estimate may tighten the bar; until then Gate H operates at the ceiling.
- Acceptable trade-off because (a) Phase B and C don't gate on `j-RR` (Gate H is a Phase E artifact gate), and (b) the substitution is **operationally permitted at any phase including Phase E** — see § 8 for the operative rule. If § 3 conditions complete before Phase E, calibration UPGRADES the bar; if not, Phase E proceeds with the ceiling substitution. There is no path under this amendment where Phase E is blocked by absence of calibration — that was an earlier draft's misclaim, removed by A8e8 fix M4 after Codex round-7 audit.

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
- Does substitution of `p_0 = 0.10` invalidate any Gate H statistical claim? (Should not — `j-RR` is descriptive (PREREG § 5.4), outside the inferential family, so no Type-I inflation. Per § 2 wording correction: 0.10 is the LEAST-strict admissible ceiling, NOT a tighter-than-original substitute — the substitution may be less protective than an eventual calibrated `p_0 < 0.10`, but it is **operationally permitted at any phase including Phase E** by the original PREREG § 5.5 ceiling. § 3 conditions are PRE-REQUISITES for calibration to run at all; they do NOT guarantee calibration completes before any specific phase.)
- Does deferral leave any phase blocked indefinitely? (Should not — § 3 lists explicit re-calibration conditions; if those never met, Phase E proceeds with the operationally-permitted ceiling substitution per § 2 final paragraph. Calibration UPGRADES the bar IF and WHEN § 3 conditions complete; absence of calibration is not a Phase E blocker.)

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

## experiments/minif2f_v4/src/run_id.rs

```rust
// A8e fix F1 — single run_id minted once per run.
//
// Codex#2 + Gemini Q4 (A8 round-1 audit): `run_corr_id` was generated
// at run_swarm/run_oneshot entry while make_pput's internal `run_id`
// was recomputed at the terminal emit site, causing millisecond drift
// between the two identifiers. Phase D consumers cannot reliably join
// FC events (stamped with run_corr_id) to v2 jsonl rows (stamped with
// run_id). Oneshot was even worse — it used `oneshot_{problem_file}`
// as the FC correlation key, completely disjoint from the eventual
// PputResult.run_id format.
//
// Fix: mint ONE run_id at function entry, thread to both emit_event
// and make_pput. Format mirrors the prior make_pput format
// (`{condition}_{problem_id}_{unix_ms}`) so existing v2 jsonl rows
// don't change shape.

/// TRACE_MATRIX correlation: stable per-run identifier. Format is
/// `{condition}_{problem_id}_{unix_ms}` where `problem_id` is the
/// file-stem of the .lean file (no extension). Phase D consumers join
/// on this exact string between fc_trace events and v2 jsonl rows.
pub fn mint_run_id(condition: &str, problem_file: &str) -> String {
    let problem_id = std::path::Path::new(problem_file)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(problem_file);
    let ts_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    format!("{}_{}_{}", condition, problem_id, ts_ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_matches_legacy_make_pput_format() {
        // Legacy make_pput used: format!("{}_{}_{}", condition, problem_id, ts).
        // Same shape preserves backward compat with downstream tools that
        // already parsed v2 jsonl run_id strings.
        let id = mint_run_id("n3", "/tmp/foo.lean");
        let parts: Vec<&str> = id.splitn(3, '_').collect();
        assert_eq!(parts[0], "n3");
        assert_eq!(parts[1], "foo");
        assert!(parts[2].parse::<u128>().is_ok(),
                "third segment must be unix-ms timestamp, got: {}", parts[2]);
    }

    #[test]
    fn handles_path_with_no_stem() {
        // Defensive: passing the literal path falls back to the input
        // string (avoids panicking on weird inputs).
        let id = mint_run_id("oneshot", "/");
        assert!(id.starts_with("oneshot_"));
    }

    #[test]
    fn distinguishes_conditions_for_same_problem() {
        let a = mint_run_id("n3", "/tmp/p.lean");
        let b = mint_run_id("oneshot", "/tmp/p.lean");
        assert!(a.starts_with("n3_p_"));
        assert!(b.starts_with("oneshot_p_"));
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

    // A8e fix F1 (Codex#2 + Gemini Q4): one run_id minted at function
    // entry, passed to both fc_event!s and make_pput. Eliminates the
    // millisecond drift between `run_corr_id` (was generated here) and
    // make_pput's internal recomputation. Phase D consumers can now
    // join FC events to v2 jsonl rows by `run_id` equality.
    let run_id = minif2f_v4::run_id::mint_run_id("oneshot", problem_file);

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
                                 acc.total_run_token_count(),
                                 acc.failed_branch_count,
                                 wc.elapsed_ms().unwrap_or(0),
                                 false, 1, 1, verifier_wait_ms,
                                 oneshot_regime, oneshot_budget_base, &run_id);
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
                // A8e fix F1: stamp the unified run_id (not the
                // round-1 `oneshot_{problem_file}` placeholder) so
                // Phase D can join by equality.
                &run_id, None, None,
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
                              acc.total_run_token_count(),
                              acc.failed_branch_count,
                              wc.elapsed_ms().unwrap_or(0),
                              false, 1, 1, verifier_wait_ms,
                              oneshot_regime, oneshot_budget_base, &run_id)
                }
                Ok(false) => {
                    // Lean rejected → neither leg.
                    make_pput(problem_file, "oneshot", model,
                              false, false, start, 0, 0, 1,
                              None, None, None, None, None,
                              acc.total_run_token_count(),
                              acc.failed_branch_count,
                              wc.elapsed_ms().unwrap_or(0),
                              false, 1, 1, verifier_wait_ms,
                              oneshot_regime, oneshot_budget_base, &run_id)
                }
                Err(e) => {
                    warn!("Oracle error: {}", e);
                    // Lean error → measurement failure → neither leg.
                    make_pput(problem_file, "oneshot", model,
                              false, false, start, 0, 0, 1,
                              None, None, None, None, None,
                              acc.total_run_token_count(),
                              acc.failed_branch_count,
                              wc.elapsed_ms().unwrap_or(0),
                              false, 1, 1, verifier_wait_ms,
                              oneshot_regime, oneshot_budget_base, &run_id)
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

    // A8e fix F1 (Codex#2 + Gemini Q4): single run_id minted ONCE per
    // run, threaded into both fc_event!s and make_pput. Replaces the
    // round-1 `run_corr_id` (FC events) ↔ make_pput-internal `run_id`
    // (v2 jsonl) split that introduced millisecond drift on the join key.
    let run_id = minif2f_v4::run_id::mint_run_id(&condition, problem_file);

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
                &run_id, Some(tx as u64), None,
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
                                       acc.total_run_token_count(),
                                       acc.failed_branch_count,
                                       wc.elapsed_ms().unwrap_or(0),
                                       false,
                                       proposal_hashes.len() as u64,
                                       proposal_count,
                                       verifier_wait_ms,
                                       budget_regime, budget_max_tx_base, &run_id);
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
            // Phase D consumer joins on (run_id, tx) to derive the
            // tape-growth curve and detect zero-tick stalls before they
            // become C-036 alarm events.
            minif2f_v4::fc_trace::emit_event(
                minif2f_v4::fc_trace::FcId::Fc2N20,
                &run_id, Some(tx as u64), None,
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
                                let v_alone_elapsed = v_t0.elapsed().as_millis() as u64;
                                verifier_wait_ms += v_alone_elapsed;
                                // A8e fix F4 (Codex#3): emit FC1-N12 for the swarm
                                // verify_omega_detailed call. Round-1 audit showed
                                // FC1-N12 was only emitted in oneshot, leaving the
                                // primary swarm verify path invisible to Phase D.
                                let r_alone_verdict = match &r_alone {
                                    Ok((true, _)) => "Ok(true)",
                                    Ok((false, _)) => "Ok(false)",
                                    Err(_) => "Err",
                                };
                                minif2f_v4::fc_trace::emit_event(
                                    minif2f_v4::fc_trace::FcId::Fc1N12,
                                    &run_id, Some(tx as u64), Some(agent_id.as_str()),
                                    &[
                                        ("verdict", minif2f_v4::fc_trace::json_str(r_alone_verdict)),
                                        ("elapsed_ms", v_alone_elapsed.to_string()),
                                        ("path", minif2f_v4::fc_trace::json_str("alone")),
                                    ],
                                );
                                let (full_proof, path_choice, r_final) = match &r_alone {
                                    Ok((true, _)) => (payload.clone(), "alone", r_alone.clone()),
                                    _ if !tape_chain.is_empty() => {
                                        // Path 2: tape + payload (A4 verifier_wait bracket)
                                        let combined = format!("{}\n{}", tape_chain, payload);
                                        let v_t1 = Instant::now();
                                        let r_combined = oracle.verify_omega_detailed(&combined);
                                        let v_combined_elapsed = v_t1.elapsed().as_millis() as u64;
                                        verifier_wait_ms += v_combined_elapsed;
                                        // A8e fix F4: FC1-N12 for the tape+payload retry.
                                        let r_combined_verdict = match &r_combined {
                                            Ok((true, _)) => "Ok(true)",
                                            Ok((false, _)) => "Ok(false)",
                                            Err(_) => "Err",
                                        };
                                        minif2f_v4::fc_trace::emit_event(
                                            minif2f_v4::fc_trace::FcId::Fc1N12,
                                            &run_id, Some(tx as u64), Some(agent_id.as_str()),
                                            &[
                                                ("verdict", minif2f_v4::fc_trace::json_str(r_combined_verdict)),
                                                ("elapsed_ms", v_combined_elapsed.to_string()),
                                                ("path", minif2f_v4::fc_trace::json_str("tape+payload")),
                                            ],
                                        );
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
                                            &run_id, Some(tx as u64), Some(agent_id.as_str()),
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
                                                        acc.total_run_token_count(),
                                                        acc.failed_branch_count,
                                                        wc.elapsed_ms().unwrap_or(0),
                                                        false,
                                                        proposal_hashes.len() as u64,
                                                        proposal_count,
                                                        verifier_wait_ms,
                                                        budget_regime, budget_max_tx_base, &run_id);
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
                                let v_partial_elapsed = v_t0.elapsed().as_millis() as u64;
                                verifier_wait_ms += v_partial_elapsed;
                                // A8e fix F4 (Codex#3): FC1-N12 emit for the
                                // step-verify path. Closes the swarm-side gap
                                // round-1 audit flagged.
                                let partial_verdict_str = match &verdict {
                                    PartialVerdict::Complete => "Complete",
                                    PartialVerdict::PartialOk => "PartialOk",
                                    PartialVerdict::Reject(_) => "Reject",
                                };
                                minif2f_v4::fc_trace::emit_event(
                                    minif2f_v4::fc_trace::FcId::Fc1N12,
                                    &run_id, Some(tx as u64), Some(agent_id.as_str()),
                                    &[
                                        ("verdict", minif2f_v4::fc_trace::json_str(partial_verdict_str)),
                                        ("elapsed_ms", v_partial_elapsed.to_string()),
                                        ("path", minif2f_v4::fc_trace::json_str("partial")),
                                    ],
                                );
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
                                            &run_id, Some(tx as u64), Some(agent_id.as_str()),
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
                                                        acc.total_run_token_count(),
                                                        acc.failed_branch_count,
                                                        wc.elapsed_ms().unwrap_or(0),
                                                        false,
                                                        proposal_hashes.len() as u64,
                                                        proposal_count,
                                                        verifier_wait_ms,
                                                        budget_regime, budget_max_tx_base, &run_id);
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
        &run_id, Some(max_transactions as u64), None,
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
              acc.total_run_token_count(),
              acc.failed_branch_count,
              wc.elapsed_ms().unwrap_or(0),
              true,
              proposal_hashes.len() as u64,
              proposal_count,
              verifier_wait_ms,
              budget_regime, budget_max_tx_base, &run_id)
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
    total_run_token_count: u64,
    failed_branch_count: u32,
    total_wall_time_ms: u64,
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
    // A8e fix F1 (Codex#2 + Gemini Q4): run_id minted by caller (run_swarm
    // or run_oneshot) at function entry; passed in here so the v2 jsonl
    // row stamps the SAME identifier the FC events used. No more ms drift.
    run_id: &str,
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

    // PREREG § 5 constitutional notation: C_i (full-run cost) + T_i (wall clock).
    let c_i = total_run_token_count;
    let t_i = total_wall_time_ms;

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

    PputResult {
        // ── B1 v2 schema fields ──
        schema_version: "v2.0".to_string(),
        // A8e fix F1: caller-supplied run_id (matches the FC-trace
        // correlation key emitted at every fc_event! site). No more
        // ms drift between the two identifiers.
        run_id: run_id.to_string(),
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
        failed_branch_count,
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
            2000, 0, 15_000,
            // A4: oneshot success — no max-tx, 1/1 unique, 4500ms in Lean.
            false, 1, 1, 4_500,
            // A5: oneshot stamps total_proposal + base=1 (single LLM call).
            minif2f_v4::budget_regime::BudgetRegime::TotalProposal, 1,
            "test_run_id",
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
            2000, 0, 15_000,
            // A4: same shape as success path; A4 fields are independent
            // of the H1 divergence signal we're testing here.
            false, 1, 1, 4_500,
            minif2f_v4::budget_regime::BudgetRegime::TotalProposal, 1,
            "test_run_id",
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
            8_000, 199, 120_000,
            true, 50, 200, 90_000,
            // A5: canonical Phase B baseline = total_proposal × 200.
            minif2f_v4::budget_regime::BudgetRegime::TotalProposal, 200,
            "test_run_id",
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
            2_000, 49, 40_000,
            false, 20, 50, 25_000,
            minif2f_v4::budget_regime::BudgetRegime::TotalProposal, 200,
            "test_run_id",
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
    syntax (e.g. `siliconflow:Qwen/Qwen2.5-7B-Instruct`); falls back
    to model-string heuristics for backward compat with v3 callers.

    Routing matrix (A8e fix F3 + A8e6 fix K2):
      explicit `provider:...`            → that provider (if known)
      slash-separated id (huggingface-style "Org/Model")
                                          → siliconflow (dispatches the
                                            full catalog including Qwen,
                                            openai-compat, Meta, AND
                                            `deepseek-ai/<distill>` SF-
                                            catalog DeepSeek variants —
                                            see K2 note below)
      bare contains "deepseek"           → deepseek (api.deepseek.com)
      bare "qwen3-*" / "qwen-*"          → dashscope (Aliyun Qwen direct)
      else                                → dashscope (default fallback)

    Round-1 audit caught a routing inversion: `Qwen/Qwen2.5-7B-Instruct`
    used to misroute to dashscope because `m.startswith("qwen")` won
    after the slash check. F3 promoted slash-form above bare-qwen.

    Round-6 audit (Codex R6#2) caught a related collision: HuggingFace-
    style `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B` is a SiliconFlow-
    catalog model (the official DeepSeek API only serves the canonical
    `deepseek-chat` / `deepseek-v4-flash` family, not the Distill
    variants). The pre-K2 logic checked `"deepseek" in m` BEFORE the
    slash check, so `deepseek-ai/...` misrouted to api.deepseek.com
    and 404'd. K2 swaps the order: slash-form is now the FIRST routing
    heuristic (after explicit prefix), and bare-deepseek substring
    only fires when there's no slash.
    """
    if ":" in model:
        prefix = model.split(":", 1)[0].lower()
        if prefix in PROVIDERS:
            return prefix
    if "/" in model:
        # Any slash-form (Qwen/..., openai/..., meta-llama/...,
        # deepseek-ai/...) goes to the heterogeneous catalog provider.
        # K2: this branch is FIRST so HuggingFace-style identifiers
        # never silently fall through to a bare-string provider that
        # doesn't actually serve them.
        return "siliconflow"
    m = model.lower()
    if "deepseek" in m:
        # Bare "deepseek-chat", "deepseek-v4-flash" → official endpoint.
        # Slash-form like "deepseek-ai/..." won't reach here per K2.
        return "deepseek"
    if m.startswith("qwen"):
        # Bare qwen3-*, qwen-* without slash = direct DashScope catalog.
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
    # A8e11 fix P1 (Codex R10#1): the V3L-27 mitigation requires THREE
    # keys for high-concurrency rate-limit pool splitting (case C-027).
    # Pre-P1 the smoke would PASS if only the primary key was set,
    # silently degrading the gate to a single-key probe and matching
    # exactly the V3L-27 collapse pattern.
    # Post-P1: ALL THREE keys must be set AND respond. Missing OR
    # failing key = FAIL. Explicit env-var opt-out
    # `SILICONFLOW_SMOKE_ALLOW_PARTIAL=1` lets a developer probe a
    # subset (e.g. for credential rotation testing); the bypass is
    # logged loudly and the result is downgraded to a partial-PASS
    # exit code that callers can distinguish.
    any_failed = False
    missing: list[str] = []
    failed: list[str] = []
    succeeded: list[str] = []
    for label, env_name in KEY_ENVS:
        key = os.environ.get(env_name, "").strip()
        if not key:
            missing.append(label)
            print(f"  [{label:9s}] {env_name}: NOT SET")
            continue
        ok, summary = probe_one(label, env_name, key)
        verdict = "OK  " if ok else "FAIL"
        print(f"  [{label:9s}] {env_name}: {verdict} {summary}")
        if ok:
            succeeded.append(label)
        else:
            failed.append(label)
            any_failed = True

    allow_partial = os.environ.get("SILICONFLOW_SMOKE_ALLOW_PARTIAL", "") == "1"

    if missing or any_failed:
        print(
            f"[A7-smoke] result: FAIL — "
            f"missing={missing or '(none)'} failed={failed or '(none)'} "
            f"ok={succeeded or '(none)'}"
        )
        if missing:
            print(
                "[A7-smoke] V3L-27 mitigation requires ALL 3 SiliconFlow "
                "keys configured (primary + secondary + tertiary). "
                "A single-key probe replicates the very collapse pattern "
                "the 3-key pool was meant to mitigate."
            )
        if allow_partial and not failed:
            print(
                "[A7-smoke] SILICONFLOW_SMOKE_ALLOW_PARTIAL=1 — explicit "
                "downgrade accepted. Exit code 3 = partial-PASS (not full)."
            )
            return 3
        return 1
    print("[A7-smoke] result: PASS (3/3 keys configured + responded)")
    return 0


if __name__ == "__main__":
    sys.exit(main())

```

## scripts/test_llm_proxy.py

```python
#!/usr/bin/env python3
"""A8e fix F3 — unit tests for src/drivers/llm_proxy.py routing logic.

Codex#4 (round-1 A8 audit) caught: `Qwen/Qwen2.5-7B-Instruct` misrouted
to DashScope because `m.startswith("qwen")` won after the slash check.
This test file pins the routing matrix as a CI conformance gate.

Also verifies the round-robin mechanic (F2) without invoking any cloud
API: `_build_clients` is monkeypatched to return a list of dummy
sentinels, then `get_client_round_robin` is called multiple times and
the per-key counter distribution is asserted.

Run: `python3 scripts/test_llm_proxy.py` (no pytest required).
"""
import os
import sys
import unittest

# Inject repo root so `from src.drivers import llm_proxy` works.
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, ROOT)

# Import via the file path (the proxy isn't a package; it's a script).
import importlib.util

spec = importlib.util.spec_from_file_location(
    "llm_proxy", os.path.join(ROOT, "src", "drivers", "llm_proxy.py")
)
llm_proxy = importlib.util.module_from_spec(spec)
spec.loader.exec_module(llm_proxy)


class RoutingMatrixTests(unittest.TestCase):
    """detect_provider over the canonical model id matrix."""

    def test_explicit_prefix_wins(self):
        self.assertEqual(llm_proxy.detect_provider("siliconflow:Qwen/Qwen2.5-7B-Instruct"), "siliconflow")
        self.assertEqual(llm_proxy.detect_provider("deepseek:deepseek-v4-flash"), "deepseek")
        self.assertEqual(llm_proxy.detect_provider("dashscope:qwen3-8b"), "dashscope")

    def test_unknown_explicit_prefix_falls_through_to_heuristic(self):
        # `nonsense:` is not in PROVIDERS — treat the whole string as
        # a bare model id and route by heuristic. A bare string with
        # a colon and no slash falls to the dashscope default.
        self.assertEqual(llm_proxy.detect_provider("nonsense:foo-bar"), "dashscope")

    def test_deepseek_substring(self):
        self.assertEqual(llm_proxy.detect_provider("deepseek-v4-flash"), "deepseek")
        self.assertEqual(llm_proxy.detect_provider("deepseek-chat"), "deepseek")
        self.assertEqual(llm_proxy.detect_provider("DeepSeek-V4-Flash"), "deepseek")

    def test_qwen_huggingface_style_routes_to_siliconflow(self):
        # A8e fix F3 (Codex#4): the round-1 bug was that this
        # misrouted to dashscope because "qwen" prefix won after the
        # slash check. Pinned here so it can never regress.
        self.assertEqual(
            llm_proxy.detect_provider("Qwen/Qwen2.5-7B-Instruct"),
            "siliconflow",
        )
        self.assertEqual(
            llm_proxy.detect_provider("Qwen/Qwen3.5-397B-A17B"),
            "siliconflow",
        )

    def test_other_huggingface_style_routes_to_siliconflow(self):
        self.assertEqual(llm_proxy.detect_provider("openai/gpt-4o"), "siliconflow")
        self.assertEqual(
            llm_proxy.detect_provider("meta-llama/Llama-3.1-70B-Instruct"),
            "siliconflow",
        )
        self.assertEqual(
            llm_proxy.detect_provider("THUDM/glm-4-9b-chat"),
            "siliconflow",
        )

    def test_deepseek_slash_form_routes_to_siliconflow(self):
        # A8e6 fix K2 (Codex R6#2): `deepseek-ai/DeepSeek-R1-Distill-*`
        # is a SiliconFlow-catalog model — the official DeepSeek API
        # at api.deepseek.com only serves bare `deepseek-chat` and
        # `deepseek-v4-flash`, not the Distill variants. Pre-K2 the
        # routing checked "deepseek" substring BEFORE the slash check
        # and misrouted these to api.deepseek.com, which returns 404.
        # Pinned here so the slash-vs-substring precedence never
        # regresses.
        self.assertEqual(
            llm_proxy.detect_provider("deepseek-ai/DeepSeek-R1-Distill-Qwen-7B"),
            "siliconflow",
        )
        self.assertEqual(
            llm_proxy.detect_provider("deepseek-ai/DeepSeek-V2.5"),
            "siliconflow",
        )
        # Sanity: bare deepseek model ids still route to deepseek.
        self.assertEqual(
            llm_proxy.detect_provider("deepseek-chat"),
            "deepseek",
        )
        self.assertEqual(
            llm_proxy.detect_provider("deepseek-v4-flash"),
            "deepseek",
        )

    def test_bare_qwen_routes_to_dashscope(self):
        # Bare model ids without a slash are direct DashScope catalog.
        self.assertEqual(llm_proxy.detect_provider("qwen3-8b"), "dashscope")
        self.assertEqual(llm_proxy.detect_provider("qwen-max"), "dashscope")

    def test_default_fallback_dashscope(self):
        self.assertEqual(llm_proxy.detect_provider("some-unknown-model"), "dashscope")
        self.assertEqual(llm_proxy.detect_provider(""), "dashscope")


class StripProviderPrefixTests(unittest.TestCase):
    def test_strips_known_prefix(self):
        self.assertEqual(
            llm_proxy.strip_provider_prefix("siliconflow:Qwen/Qwen2.5-7B-Instruct"),
            "Qwen/Qwen2.5-7B-Instruct",
        )
        self.assertEqual(
            llm_proxy.strip_provider_prefix("deepseek:deepseek-v4-flash"),
            "deepseek-v4-flash",
        )

    def test_leaves_unknown_prefix_intact(self):
        # `nonsense:foo` is NOT a known provider, so the colon is part
        # of the model identifier and must round-trip unchanged.
        self.assertEqual(
            llm_proxy.strip_provider_prefix("nonsense:foo-bar"),
            "nonsense:foo-bar",
        )

    def test_leaves_bare_model_intact(self):
        self.assertEqual(llm_proxy.strip_provider_prefix("qwen3-8b"), "qwen3-8b")
        self.assertEqual(
            llm_proxy.strip_provider_prefix("Qwen/Qwen2.5-7B-Instruct"),
            "Qwen/Qwen2.5-7B-Instruct",
        )


class RoundRobinTests(unittest.TestCase):
    """A8e fix F2 — round-robin distribution conformance.

    Verifies the V3L-27 single-key collapse mitigation without
    invoking any cloud API: monkeypatch `_build_clients` to return a
    fixed list of dummy strings, then call `get_client_round_robin`
    and assert the per-key counter advances [1,0,0] → [1,1,0] → [1,1,1]
    → [2,1,1] → [2,2,1] → [2,2,2] across 6 calls.
    """

    def setUp(self):
        # Reset the proxy module's process-global state between tests.
        llm_proxy.clients_by_provider.clear()
        llm_proxy._rr_counters.clear()
        llm_proxy._per_key_requests.clear()

    def test_three_key_round_robin_distributes_evenly(self):
        # Inject a 3-element dummy client pool for `siliconflow`.
        llm_proxy.clients_by_provider["siliconflow"] = ["k0", "k1", "k2"]
        llm_proxy._per_key_requests["siliconflow"] = [0, 0, 0]

        results = []
        for _ in range(6):
            client, idx = llm_proxy.get_client_round_robin("siliconflow")
            results.append((client, idx))

        # Indices must cycle 0, 1, 2, 0, 1, 2.
        self.assertEqual(
            [r[1] for r in results],
            [0, 1, 2, 0, 1, 2],
            "round-robin must visit every key in order before repeating",
        )
        # Each client must equal the corresponding pool element.
        for client, idx in results:
            self.assertEqual(client, f"k{idx}")

        # Final per_key_requests must be [2, 2, 2] — the documented
        # invariant from the A7 commit message and TRACE_MATRIX § 2.
        self.assertEqual(
            llm_proxy._per_key_requests["siliconflow"],
            [2, 2, 2],
            "after 6 calls the 3-key pool must distribute evenly",
        )

    def test_single_key_pool_always_returns_index_zero(self):
        # Single-key provider (e.g. deepseek) — round-robin must
        # degrade gracefully, not throw modulo-by-zero.
        llm_proxy.clients_by_provider["deepseek"] = ["only-key"]
        llm_proxy._per_key_requests["deepseek"] = [0]

        for expected_count in range(1, 5):
            client, idx = llm_proxy.get_client_round_robin("deepseek")
            self.assertEqual(idx, 0)
            self.assertEqual(client, "only-key")
            self.assertEqual(
                llm_proxy._per_key_requests["deepseek"],
                [expected_count],
            )

    def test_two_key_pool_alternates(self):
        # Two-key pool: 4 calls → [2, 2].
        llm_proxy.clients_by_provider["siliconflow"] = ["a", "b"]
        llm_proxy._per_key_requests["siliconflow"] = [0, 0]
        for _ in range(4):
            llm_proxy.get_client_round_robin("siliconflow")
        self.assertEqual(
            llm_proxy._per_key_requests["siliconflow"],
            [2, 2],
        )


class StatsAggregationTests(unittest.TestCase):
    def setUp(self):
        llm_proxy._reset_stats()

    def test_get_stats_includes_per_key_distribution(self):
        # Seed a 3-key pool and fire 3 calls.
        llm_proxy.clients_by_provider["siliconflow"] = ["k0", "k1", "k2"]
        llm_proxy._per_key_requests["siliconflow"] = [0, 0, 0]
        for _ in range(3):
            llm_proxy.get_client_round_robin("siliconflow")
        stats = llm_proxy._get_stats()
        self.assertEqual(stats["per_key_requests"]["siliconflow"], [1, 1, 1])

    def test_reset_clears_per_key_counters(self):
        llm_proxy.clients_by_provider["siliconflow"] = ["k0", "k1", "k2"]
        llm_proxy._per_key_requests["siliconflow"] = [3, 3, 3]
        llm_proxy._reset_stats()
        self.assertEqual(
            llm_proxy._per_key_requests["siliconflow"],
            [0, 0, 0],
            "_reset_stats must zero per-key counters too",
        )


if __name__ == "__main__":
    unittest.main(verbosity=2)

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
        // A8e13 fix Q1: src/boot.rs implements verify_trust_root itself
        // (per Codex R11#1 + C-075 — the verifier must be qualifiable;
        // tampering with boot.rs would silently bypass the entire gate).
        "src/boot.rs",
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
        // A8e fix F1: unified run_id minted once per run (was run_corr_id ms drift)
        "experiments/minif2f_v4/src/run_id.rs",
        // A8e fix F2/F3: routing matrix + round-robin Python conformance tests
        "scripts/test_llm_proxy.py",
        // A8e2 fix G1: Rust wrapper that runs the Python suite on every cargo test
        "experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs",
        // A8e7: append-only audit history (companion to A8_EXIT_PACKET; per C-075
        // gate machinery is constitutional — tampering with the chronology = silent
        // governance drift).
        "handover/audits/A8_AUDIT_HISTORY_2026-04-26.md",
        // A8e11 fix P2: audit runner scripts that assemble the packet, append
        // sources, and produce dual-audit transcripts. Per C-075 + Codex R10#2
        // these are load-bearing gate machinery (R8/R9 runner defects
        // demonstrated they are not incidental); silent tamper = silent gate bypass.
        "handover/audits/run_codex_phase_a8_exit_audit.sh",
        "handover/audits/run_gemini_phase_a8_exit_audit.py",
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

## experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs

```rust
// A8e2 fix G1 — recurring conformance gate for `src/drivers/llm_proxy.py`.
//
// Round-2 audit (Codex R2#1 + Gemini R2#1) caught: `scripts/test_llm_proxy.py`
// existed and was in Trust Root, but was only documented as a manual
// invocation. A test that does not run automatically is just
// documentation — it cannot prevent the V3L-27-class regression that
// Gemini's round-1 VETO targeted.
//
// This integration test bridges the Python proxy suite into
// `cargo test --workspace` so it runs on every Rust test invocation
// and on every CI pipeline that already exercises Rust tests. The
// test depends on the system `python3` interpreter being available;
// if not, it FAILS CLOSED with a clear diagnostic (A8e3 fix H6 —
// Codex round-3 caught the prior soft-skip behavior that masqueraded
// as a passing gate when Python was missing). Explicit opt-out:
// `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1` (must be set deliberately;
// the bypass is logged loudly to make the gate's absence visible).
//
// Constitutional anchor: meta-witness for atom A7 (case C-027 +
// V3L-27 mitigation — multi-key round-robin avoiding single-key
// rate-limit collapse).

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    // The minif2f_v4 crate's Cargo.toml lives at the repo's
    // experiments/minif2f_v4 path, so two parents up is the repo root.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("repo root reachable from CARGO_MANIFEST_DIR")
        .to_path_buf()
}

#[test]
fn proxy_python_conformance_suite_passes() {
    let root = repo_root();
    let script = root.join("scripts").join("test_llm_proxy.py");
    assert!(
        script.is_file(),
        "scripts/test_llm_proxy.py must exist at the canonical path; got {:?}",
        script
    );

    // A8e3 fix H6 (Codex R3#3): the wrapper MUST fail closed when
    // `python3` is missing. A "soft skip" is exactly the silent-pass
    // failure mode that Gemini's round-1 VETO targeted — a gate that
    // disappears under environmental drift is not a gate. If a runner
    // environment lacks Python, that's a CI configuration bug, not an
    // acceptable-skip case. Explicit opt-out:
    // `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1` — must be set deliberately,
    // never set by default. The bypass is logged loudly so the gate's
    // absence is visible in test output.
    let opt_out = std::env::var("SKIP_LLM_PROXY_PYTHON_CONFORMANCE")
        .as_deref() == Ok("1");
    if opt_out {
        eprintln!(
            "[G1] SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1 — gate explicitly \
             bypassed. This is a downgraded run; the V3L-27 round-robin \
             conformance battery did NOT execute."
        );
        return;
    }
    let python_check = Command::new("python3").arg("--version").output();
    assert!(
        python_check.is_ok(),
        "python3 not found on PATH; G1 conformance gate requires it. \
         Install python3 + the openai SDK (see scripts/test_llm_proxy.py \
         header) or — only with deliberate intent — set \
         SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1 to bypass."
    );

    let output = Command::new("python3")
        .arg(&script)
        .current_dir(&root)
        .output()
        .expect("spawn python3 scripts/test_llm_proxy.py");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "scripts/test_llm_proxy.py must exit 0 (round-robin + routing \
         conformance battery for V3L-27 mitigation).\n\
         status: {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        stdout,
        stderr
    );

    // Sanity-check the unittest summary line is present so a future
    // refactor that removes the test class definitions surfaces here
    // instead of silently skipping.
    assert!(
        stderr.contains("OK") || stdout.contains("OK"),
        "unittest output must contain the trailing 'OK' line.\n\
         stdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
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
- A5 (post-A4): per-agent budget normalization — new `budget_regime` module (`BUDGET_REGIME` + `MAX_TRANSACTIONS` env vars; 4-variant enum; pure parser + scaler + env-coupled resolver); `budget_regime` + `budget_max_transactions` added as non-Optional v2 fields on `RunAggregate` and the legacy `PputResult`; loop bound at `run_swarm` switched from hardcoded `let max_transactions = 200` to `resolve_budget(n_agents)` — default (env unset) preserves Phase B baseline (`total_proposal × 200`) bit-for-bit. PREREG_AMENDMENT_p0_defer § 3 condition 3 satisfied: `MaxTxExhausted` rows now disambiguated across N values. FC-trace: FC2-N22 (HALT decomposition by budget regime) + FC1-N7 (δ instances determining the per-agent share under PerAgent regime). Trust Root manifest 26 → 27 (corrected A8e5 J4 — the A1 PREREG amendment had taken the count to 25, A3's `agent_models.rs` to 26 before A5 landed; round-1 packet undercounted by 1).
- A6 (post-A5): per-line FC tagging via structured JSON events — new `fc_trace` module (pure stdlib; zero new deps); `FcId` enum (FC1-N7 / FC1-N11 / FC1-N12 / FC1-E18 / FC2-N20 / FC2-N22 / FC3-N31); `fc_event!`-style `emit_event` API; `FC_TRACE=1` gate (cached in `OnceLock`); `FC_TRACE_FILE=<path>` redirects emit to file (default sink stderr). 9 anchor sites wired (post-A8e total — 8 in `run_swarm` + 1 in `run_oneshot`): the original A6 commit shipped 6 sites (FC2-N22 synthetic short-circuit, FC2-N20 mr tick, FC2-N22 OMEGA full-proof, FC2-N22 OMEGA per-tactic, FC2-N22 natural MaxTxExhausted with `budget_regime` payload, FC1-N12 verify bracket in oneshot). A8e fix F4 added 3 more in `run_swarm` (FC1-N12 verify_omega_detailed × 2 paths + FC1-N12 verify_partial), bringing the post-A8e total to 9 production emit sites. Counted by grepping `fc_trace::emit_event(` in `experiments/minif2f_v4/src/bin/evaluator.rs` (Gemini R6 finding: round-5 phrasing "Six anchor sites" was stale relative to its own following sentence — corrected here in A8e6 K4). End-to-end smoke test exercises FC_TRACE=1 in a child process (subprocess required because `OnceLock` caches the gate-read; resolves item 7 of TRACE_MATRIX § 5 "Per-line FC tagging via tracing crate"). FC-trace: meta-witness for the 5-step compile loop (Proposal → Lean ground truth → Logging → Capability compilation → ↑H-VPPUT). Trust Root manifest 27 → 28 (corrected A8e5 J4; matches the canonical chain in `genesis_payload.toml` header).
- A7 (post-A6): heterogeneous-LLM provider plumbing — `src/drivers/llm_proxy.py` ported from v3 with one load-bearing v4 change (per-provider multi-key round-robin: 3 SiliconFlow keys split concurrent traffic across separate rate-limit pools, mitigating V3L-27 single-key N=30 collapse). `scripts/smoke_siliconflow.sh` + `scripts/_smoke_siliconflow.py` probe each of the 3 keys (Qwen/Qwen2.5-7B-Instruct, max_tokens=8) — A7 verified all 3 keys responding 2026-04-26 (1.5–3s latency, 33+1 tokens; round-robin distributes [2,2,2] across 6 calls). New `siliconflow:<model>` provider-prefix syntax in `detect_provider()` for unambiguous routing in `AGENT_MODELS` payloads (Phase D heterogeneous swarms). Memory `reference_siliconflow.md` records SiliconFlow as the heterogeneous-LLM lane (NOT a fallback target). FC-trace: FC1-N7 (δ/AI provider expansion — heterogeneous δ instances across SF catalog enable Phase D meta-loop). Trust Root manifest 28 → 31 (3 entries: `llm_proxy.py` + 2 smoke scripts).
- A8e12 (post-A8e11, harness amplifier per user directive 2026-04-26 中文 "C，更新你的harness，加入判例"): three threads — (1) **case C-076** sediments the false-closure lesson from A8e9 N3 + the multi-section parity miss from A8e8 M4 → A8e9 N1 into a binding precedent (every commit asserting ≥2 distinct fix items requires per-claim `Verified:` proof line); (2) **rule R-020** (`rules/active/R-020_commit_claim_diff_parity.yaml` + inline `.claude/hooks/judge.sh` implementation under R-016 sibling block) WARNs at pre-commit on multi-fix-bundle messages without `Verified:` lines; (3) **self-audit pass** before round-11 to catch documentary drift my own commits keep introducing — fixed `35-entry` → `37-entry` in both audit runner header printfs (post-A8e11 the manifest is 37 but the runners still printed the pre-A8e11 value), `proxy for 14 rules` → `proxy for 15 rules` in genesis_payload.toml comment, and regenerated `cases/MANIFEST.sha256` (50 → 51 entries with C-076) + `rules/MANIFEST.sha256` (14 → 15 entries with R-020) + re-hashed `judge.sh` (R-020 inline addition) + re-hashed both audit runners (35→37 edit). Trust Root manifest unchanged at 37 (no new TR entries; only re-hashes of existing entries — case yaml is covered by cases/MANIFEST.sha256 proxy, rule yaml by rules/MANIFEST.sha256 proxy, judge.sh inline change re-hashes the existing entry). FC-trace: FC3 (read-only / log subgraph — governance instrumentation amplifier; case + rule are Living Harness updates per memory `lesson_to_rule` skill).
- A8e11 (post-A8e10, FIX-THEN-PROCEED after A8 round-10 audit Codex CHALLENGE / Gemini PASS): two substantive gate-machinery hardenings. **P1 (Codex R10#1)**: `scripts/_smoke_siliconflow.py` now FAILS CLOSED when any of the 3 SiliconFlow keys is missing or unresponsive — pre-P1 the smoke would PASS on a single-key configuration, exactly replicating the V3L-27 single-key collapse pattern the 3-key pool was meant to mitigate (silent gate degradation). Explicit opt-out `SILICONFLOW_SMOKE_ALLOW_PARTIAL=1` returns exit 3 (partial-PASS) for credential rotation. Verified across full-keys / primary-only / primary-only-with-opt-out paths. Re-hashed in Trust Root. **P2 (Codex R10#2)**: `handover/audits/run_codex_phase_a8_exit_audit.sh` + `handover/audits/run_gemini_phase_a8_exit_audit.py` added to Trust Root manifest. Per case C-075 these are gate machinery (R8 N3 false-closure + R9 default-R2 defects demonstrated they are load-bearing); silent tamper would silently bypass the dual-audit gate. `trust_root_immutability` required-paths list extended. Manifest 35 → 37 (3 entries: smoke re-hash + 2 runners added). **Real-bug yield through 10 rounds: 11 substantive findings** (5 R1 + 1 R3 + 1 R6 + 1 R7 + 1 R8 + 2 R10 = 11; R2/4/5/8/9 had no substantive findings, only documentary).
- A8e10 (post-A8e9, FIX-THEN-PROCEED after A8 round-9 audit Codex CHALLENGE / Gemini PASS): two narrow fixes closing a false-closure from A8e9. **O1 (Codex R9#1 — false-closure)**: A8e9 fix N3 claimed "runner default `A8_AUDIT_ROUND` updated" but the source still defaulted to `R2`; re-running either runner without env silently overwrote the round-2 transcript. O1 makes `A8_AUDIT_ROUND` env var REQUIRED in both runners (fail fast with usage message; no silent default) AND refuses to overwrite an existing transcript at the resolved output path (per C-075 round-N transcripts are append-only governance artifacts). Verified: invoking either runner without env or attempting to overwrite an existing round prints diagnostic + exits 2 BEFORE any API call. **O2 (Codex R9#2)**: packet § 6 pointer "round-1..round-7" → "all prior rounds — see chronological round-N entries in history doc"; no longer ages with each new round. Trust Root manifest unchanged at 35 (only re-hash of `A8_AUDIT_HISTORY` to seal round-9 + queue round-10 row).
- A8e9 (post-A8e8, FIX-THEN-PROCEED after A8 round-8 audit Codex CHALLENGE / Gemini PASS): three fixes including one substantive PREREG logic miss. **N1 (substantive Codex R8#1)**: PREREG_AMENDMENT § 8 audit-requirements text still contained the stale claim "Gate H is Phase E and § 3 conditions ensure calibration runs first" — round-7 M4 fixed § 2(b)'s identical false claim but missed the parallel text in § 8. Now reworded so § 2 + § 3 + § 8 all consistently express "substitution operationally permitted at any phase including Phase E; § 3 conditions are pre-requisites for calibration to run, not guarantees of completion before any phase". Re-hashed in Trust Root. **N2 (Codex R8#2)**: A8_AUDIT_HISTORY round-7 entry stamped with actual A8e8 SHA `857872e` (was `<aaedc9d-successor>` placeholder); cumulative table row 7 sealed (CHALLENGE/CHALLENGE; 6 findings; 1 substantive); row 8 added (CHALLENGE/PASS; 3 findings; 1 substantive); row 9 left as pending. **N3 (Codex R8#3)**: audit runner script header metadata refreshed — was emitting "264 PASS + 30-entry manifest" (round-1 era values) every time the runners executed; now emits current state (265 PASS / 35-entry manifest) and references the history doc for chronology. Trust Root manifest unchanged at 35. **Real-bug yield through 8 rounds: 9 substantive findings caught + closed** (5 routing/correctness in R1, 1 fail-closed gate in R3, 1 routing collision in R6, 1 PREREG § 2 logic in R7, 1 PREREG § 8 logic in R8).
- A8e8 (post-A8e7, FIX-THEN-PROCEED after A8 round-7 audit Codex CHALLENGE / Gemini CHALLENGE — both convergent on rewrite-implementation gaps): five fixes including one substantive PREREG logic correction. **M1** packet § 2 + § 4 Trust Root count bumped 34 → 35 (post-A8e7 added `A8_AUDIT_HISTORY` to manifest). **M2** this history doc's A8e7 entry stamped with actual SHA `aaedc9d` + round-7 verdicts + A8e8 fix list (round-7 sealed). **M3** packet rewritten to strip ALL historical lineage text — no "(post-A8e F4)", "(added by A8eN)", "(chain position N via A8e)", "(+ A8eN fix XN)". Packet describes WHAT IS, no HOW; lineage lives only in this history doc. **M4 (substantive Codex R7#4)** PREREG_AMENDMENT § 2(b) corrected: removed false claim that "§ 3 conditions ensure calibration runs *before* Phase E"; § 3 conditions are PRE-REQUISITES for calibration TO RUN at all, not guarantees of completion before any specific phase. § 8 reads as the operative rule: substitution permitted at any phase including Phase E; calibration UPGRADES the bar IF and WHEN § 3 conditions complete. Single semantics, internally consistent across § 2 / § 3 / § 8. Re-hashed in Trust Root. **M5** audit runner scripts now append `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` so Q1.c (verify wrapper fail-closed) is verifiable from the packet bundle alone. Trust Root manifest unchanged at 35 (only re-hashes; no new files).
- A8e7 (post-A8e6, STRUCTURAL REWRITE per ultrathink constitutional alignment): rounds 2–6 produced a recurring documentary CHALLENGE pattern. Diagnosis: `A8_EXIT_PACKET` was conflating two opposite temporal modes (stable-snapshot artifact + append-only chronology) in one document — every fix to "current state" generated new staleness in "round-N retrospective" sections. Single-point fixes could not converge. **Resolution**: split the packet using the project's existing constitutional pattern (cf. `constitution.md` + Art. V.3 amendment log; `PREREG_PPUT_CCL_2026-04-26.md` + `PREREG_AMENDMENT_p0_defer_2026-04-25.md`; `TRACE_MATRIX_v0` + v1 + v2). New companion doc `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` carries the append-only chronology of rounds 1–6 + their in-cycle fix bundles. The packet itself becomes the canonical current-state snapshot — no round-N retrospectives, no fix-shipped sections, no cross-round staleness sources. FC-trace: FC3 (read-only / log subgraph — governance artifact restructure) + Art. V.1.2 (ArchitectAI commit authority on non-constitution files). Per case C-075 (DO-178C tool qualification) the new history doc is in Trust Root so the chronology cannot be silently rewritten. Trust Root manifest 34 → 35 (`A8_AUDIT_HISTORY_2026-04-26.md` added). Round 7 dual audit launches from this state.
- A8e6 (post-A8e5, FIX-THEN-PROCEED after A8 round-6 audit Codex CHALLENGE / Gemini CHALLENGE — both narrow): one real correctness fix + 4 doc fixes. **K1** packet `<pending>` placeholder cleanup (J2 + RQ14 wording) — same precedent as A8e5 J2 but applied to round-6 self-references. **K2 (real bug, Codex R6#2)** `src/drivers/llm_proxy.py` `detect_provider`: HuggingFace-style `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B` was misrouting to `api.deepseek.com` because `"deepseek" in m` won before the slash check; pre-K2 logic would 404 on every Distill variant since the official DeepSeek API doesn't serve them. K2 swaps the order — slash-form is now the FIRST routing heuristic (after explicit prefix), and bare-deepseek substring only fires when there's no slash. New `test_deepseek_slash_form_routes_to_siliconflow` in `scripts/test_llm_proxy.py` pins this (covers `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B` + `deepseek-ai/DeepSeek-V2.5` AS WELL AS the bare-form sanity case `deepseek-chat` → deepseek). **K3** TRACE_MATRIX A8e3 row stale "21 positional args" → 24 (matches packet update from A8e5 J5; Codex R6#3 caught the inconsistency between TRACE_MATRIX and the packet). **K4** TRACE_MATRIX A6 row top-line "Six anchor sites" → "9 anchor sites" with A6-original-6 + F4-added-3 breakdown (Gemini R6#1 caught the contradiction between the line's first sentence and its own follow-up). **K5** packet § 3 A5/A6 fix attribution: "A8e3 fix H4 corrected" → "A8e4 fix I2 corrected" (Gemini R6#2 caught the cross-reference error — H4 corrected the genesis_payload.toml TR-header comment, I2 corrected the packet § 3 numbers). Trust Root manifest unchanged at 34 (re-hashes of `llm_proxy.py` + `scripts/test_llm_proxy.py`; no new files).
- A8e5 (post-A8e4, FIX-THEN-PROCEED after A8 round-5 audit Codex CHALLENGE / Gemini PASS — same split as round-4): five narrow doc/source-text fixes; zero source-code changes. **J1** Round-4 outcome + A8e4 fixes shipped section added to the packet (was missing — Codex R5#1 caught the gap). **J2** `<pending>` commit placeholders for A8e2/A8e3/A8e4 replaced with actual SHAs (`0af47b7` / `3d38ba5` / `8693789`); A8e5 self-reference now uses "this commit" to break the placeholder-staleness pattern. **J3** packet § 6 Q6 question text reworded: "24 → 30 / 6 new entries" → "24 → 34 / 10 new entries"; full enumerated list added. **J4** TRACE_MATRIX § 1 top-bullet TR-deltas corrected: A5 `25 → 26` → `26 → 27`; A6 `26 → 27` → `27 → 28` (matches the canonical chain in `genesis_payload.toml` header). **J5** round-3 retrospective text about `make_pput` arg count: 21 → 24 (Gemini R5 spot-check; F1's `run_id` parameter brought the post-A8e count from 21 to 24). Trust Root manifest unchanged at 34 (no new files; doc-only changes). All findings are documentary; both auditors confirmed the code is sound + ready for Phase B.
- A8e4 (post-A8e3, FIX-THEN-PROCEED after A8 round-4 audit Codex CHALLENGE / Gemini PASS — split verdict, conservative merge wins per memory `feedback_dual_audit_conflict`): three narrow documentary defects Codex found in the packet itself. **I1** packet title + date: "round 2" → "running through rounds 1–N" (cumulative, with reader pointer to most-recent round section). **I2** per-atom Trust Root deltas in § 3: A5 25→26 → 26→27, A6 26→27 → 27→28, A7 27→30 → 28→31 (all corrected to match the chain in `genesis_payload.toml` header — round-1 packet undercounted because A3's `agent_models.rs` had already raised the count to 26 before A5 landed); TRACE_MATRIX § 1 line 11 anchor-site location: "Six anchor sites wired in `run_swarm`" → "Six anchor sites wired (5 in `run_swarm` + 1 in `run_oneshot`)" — same in § 5 item 7 closure note. **I3** `tests/llm_proxy_python_conformance.rs` file header docstring: "if not, it skips with a clear diagnostic" → "if not, it FAILS CLOSED with a clear diagnostic" (A8e3 H6 fixed the runtime but left the docstring stale; Codex R4#3 caught the contradiction). Trust Root manifest unchanged at 34 (only re-hash of the wrapper). No source-code defects, no architectural changes; this round is purely documentary alignment.
- A8e3 (post-A8e2, FIX-THEN-PROCEED after A8 round-3 audit Codex CHALLENGE / Gemini CHALLENGE — both convergent on packet staleness; no VETO): six narrow cleanup fixes. **H1** A8 packet § 3 A1 description: "Mathematically conservative" → "least-strict admissible value" with explicit Type-I implications. **H2** packet § 3 A6 description: 6 → 9 anchor sites (added F4 swarm sites explicitly). **H3** packet § 6 Q2.a / Q4.a / Q4.d marked CLOSED with closure rationale. **H4** `genesis_payload.toml` TR header phrasing about A1: "conservative ceiling" → "max-tolerated ceiling". **H5** § 5 item 7 here in TRACE_MATRIX: "(commit pending) + 6 wired" → "CLOSED + 9 wired". **H6** G1 wrapper test fail-closed on missing `python3` (was soft skip; Codex R3 caught the silent-pass risk); explicit opt-out `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1` for deliberate downgrades (logged loudly). Trust Root manifest unchanged at 34 (only re-hashes; no new files). Non-blocking observation deferred to Phase B+: `make_pput` signature was 21 positional args at round-3; A8e fix F1 added the `run_id` parameter, bringing the post-A8e count to **24** (Gemini R3 Finding 2 + Gemini R5/Codex R6 spot-checks; A8e6 K3 corrected here) — refactor to a builder pattern is recommended but not gating.
- A8e2 (post-A8e, FIX-THEN-PROCEED after A8 round-2 audit Codex CHALLENGE / Gemini CHALLENGE — both convergent): three dual-audit fixes. **G1** (Codex R2#1 + Gemini R2#1) `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` wraps `python3 scripts/test_llm_proxy.py` in a Rust integration test, making the V3L-27 round-robin conformance gate run on every `cargo test --workspace`. **G2** (Codex R2#2) `PREREG_AMENDMENT § 8` audit-requirements paragraph reworded — round-1 fix corrected § 2 but left § 8 still calling the substitution a "strictest plausible bar"; now consistent across the doc. Re-hashed in Trust Root. **G3** (Codex R2#3 + Gemini R2#2) `A8_EXIT_PACKET` updated for round-2 reality: § 2 cumulative-tests table extended with A8e/A8e2 columns, § 4 manifest-clean assertion bumped 30→34, § 5 round-1 risk #5 (no FC1-N12 in swarm verify path) removed (closed by F4), § 6 Q7.a/b numbers updated, round-2 outcome + round-3 fixes section appended; TRACE_MATRIX stale `run_corr_id` symbol row replaced; "in CI" softened to "via Rust wrapper post-A8e2". FC-trace: meta governance + V3L-27 mitigation gating (FC1-N7). Trust Root manifest 33 → 34 (`tests/llm_proxy_python_conformance.rs`).
- A8e (post-A7, FIX-THEN-PROCEED after A8 round-1 audit Codex CHALLENGE / Gemini VETO): six dual-audit fixes. **F1** (Codex#2 + Gemini Q4) `experiments/minif2f_v4/src/run_id.rs` mints one stable run_id per run, threaded into both `emit_event` and `make_pput` (replaces `run_corr_id` vs internal `run_id` ms drift; oneshot also stops using `oneshot_{problem_file}` placeholder). **F2** (Gemini VETO Q6) `scripts/test_llm_proxy.py` adds 15-test conformance battery — round-robin distributes [2,2,2] across 6 calls, single-key pool degrades to all-zero, two-key pool alternates [2,2]; exercises the V3L-27 mitigation without a live API. (A8e2 G1 wires this into `cargo test --workspace` via a Rust shell-out wrapper — see § 2 below.) **F3** (Codex#4) routing matrix bug fixed — `Qwen/Qwen2.5-7B-Instruct` was misrouting to DashScope because `m.startswith("qwen")` won after the slash check; now slash-form ⇒ siliconflow first, bare-qwen ⇒ dashscope second. **F4** (Codex#3) FC1-N12 emit added to all 3 swarm verify sites (`verify_omega_detailed` × 2 + `verify_partial`), each with verdict + elapsed_ms + path stamp; closes the swarm-side gap in the 5-step compile-loop witness. **F5** (Codex#5) `genesis_payload.toml` header + TRACE_MATRIX milestones reconciled to actual 31 entries (round-1 packet undercounted by 1 — agent_models.rs from A3 was already in TR before A1). **F6** (Codex#1) `PREREG_AMENDMENT § 2` wording corrected — `p_0=0.10` substitution is the LEAST-strict admissible ceiling (max tolerated value for `j-RR ≤ p_0`), NOT the strictest possible substitute as round-1 mistakenly claimed; explicit statistical-implications paragraph added (no Type-I inflation since `j-RR` is descriptive; substitution may be less protective than eventual calibrated `p_0 < 0.10`). FC-trace: FC1-N12 (oracle scope swarm coverage) + FC1-N7 (δ provider routing) + Art. V (amendment governance). Trust Root manifest 31 → 33 (run_id.rs + test_llm_proxy.py).

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
| `run_id::mint_run_id` correlation key + 9 wired FC events (A6 + A8e F4) | `experiments/minif2f_v4/src/run_id.rs` + `experiments/minif2f_v4/src/bin/evaluator.rs` | A6 introduced a per-run correlation id (originally named `run_corr_id`, now unified into `run_id`) anchoring all events. A8e fix F1 (Codex#2 + Gemini Q4) split this out into a dedicated module + threaded the same id into `make_pput` so FC events and v2 jsonl rows share one identifier (no ms drift). Anchor sites: FC2-N22 synthetic short-circuit / mr tick FC2-N20 / OMEGA full-proof FC2-N22 / OMEGA per-tactic FC2-N22 / natural MaxTxExhausted FC2-N22 (with `budget_regime` payload from A5) / FC1-N12 oneshot verify (A6) / FC1-N12 swarm `verify_omega_detailed` × 2 paths (A8e F4) / FC1-N12 swarm `verify_partial` (A8e F4). End-to-end smoke `tests/fc_trace_smoke.rs` exercises FC_TRACE=1 in a child process (forced because `OnceLock` caches the gate-read). | ✅ |
| `tests/llm_proxy_python_conformance.rs` Rust→Python wrapper (A8e2 fix G1) | `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` | A8 round-2 audit (Codex R2#1 + Gemini R2#1): the round-robin conformance battery in `scripts/test_llm_proxy.py` was only manual; tests that don't run automatically are documentation, not gates. G1 wires the Python suite into `cargo test --workspace` via a Rust integration test that shells to `python3` and asserts exit 0 + the unittest "OK" trailer. The V3L-27 mitigation (case C-027) is now a recurring conformance gate exercised on every commit that touches Rust + every CI pipeline that already exercises Rust tests. | ✅ |
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

**Total: 24 entries** (15 from B7 + 1 B7-extra rollback_sim + 4 dual-audit fixes + 4 A0 harness). A1 (PREREG amendment) → 25; A3 (agent_models.rs) → 26; A5 (budget_regime.rs) → 27; A6 (fc_trace.rs) → 28; A7 (llm_proxy.py + smoke_siliconflow.sh + _smoke_siliconflow.py) → 31; A8e (run_id.rs + test_llm_proxy.py) → 33; A8e2 (llm_proxy_python_conformance.rs) → 34; A8e7 (A8_AUDIT_HISTORY_2026-04-26.md append-only chronology) → 35; A8e11 (audit runner scripts run_codex/run_gemini per C-075 gate-machinery rule) → 37. When B7-extra calibration eventually runs, the calibration jsonl makes 38 entries; future Phase C's `--mode` flag binary (TBD location) makes 39.

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
5. **Ground-truth-label rule (was provisionally labeled R-020 in A0a planning)** — sketched but not implemented (grep on PputResult/RunAggregate field additions to enforce thesis claim 7 ground-truth source). Note: A8e12 took the R-020 slot for `commit_claim_diff_parity_warn` (case C-076); the ground-truth-label rule will be re-numbered when it lands. Defer to next harness cycle.
6. **FC2-N23 HaltReason full taxonomy as Rust enum** — currently only OmegaAccepted is typed; other 4 variants live as jsonl strings. Phase C+ Soft Law mode work may force this typing.
7. ~~**Per-line FC tagging via tracing crate** — Plan agent's recommendation in N-experiments brainstorm. Phase A6 deferred; will land before Phase B (homogeneous experiments).~~ **CLOSED**: A6 (`89994c7`) shipped `fc_trace.rs` + 6 production emit sites (5 in `run_swarm` + 1 in `run_oneshot`); A8e fix F4 added 3 more in `run_swarm` (`verify_omega_detailed` alone + tape+payload retry, plus `verify_partial`), bringing the post-A8e total to **9 wired anchor sites**. Implementation chose pure stdlib over the `tracing` crate to avoid a new dep tree; the macro surface (`emit_event` + `FcId` enum) was kept small so Phase D+ can swap to a real `tracing-subscriber` bridge locally.

## § 6. Updated counts (v2)

Compared to v1:
- ✅ count: 16 → **33** (+17 from fc_alignment_conformance.rs witness battery; +4 from new symbols/rules; +4 from manifest expansion; +5 case-law entries; -3 stale)
- 📅/📄 count: 9 → **9** (Phase 11+ deferred unchanged; some clarified with case references)
- 🔨/⚠️ count: 0 → **0** (no actionable rows pending in v2 scope)
- New cases: 6 (C-071..C-076; C-076 added in A8e12 — see `A8_AUDIT_HISTORY_2026-04-26.md` for derivation)
- New rules: 5 active (R-014/R-015/R-018/R-019/R-020) + 1 hook-level (R-016); rules-yaml total now 15

Manifest size milestones:
- B7 → 15
- B7-extra → 16
- B7-extra round-1 audit-fix → 20
- A0 (this v2) → 24
- A1 PREREG amendment → 25
- A3 agent_models.rs → 26
- A5 budget_regime.rs → 27
- A6 fc_trace.rs → 28
- A7 llm_proxy.py + smoke_siliconflow.{sh,py} → 31
- A8e run_id.rs + test_llm_proxy.py → 33
- A8e2 llm_proxy_python_conformance.rs → 34
- A8e7 A8_AUDIT_HISTORY_2026-04-26.md → 35
- A8e11 audit runner scripts (run_codex_phase_a8_exit_audit.sh + run_gemini_phase_a8_exit_audit.py) → **37**
- (planned) B7-extra calibration freeze → 38
- (planned) Phase C mode-flag binary → 39+

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
#     rules/MANIFEST.sha256 (proxy for 15 rules/active/R-*.yaml as of A8e12 — governance
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
#       max-tolerated ceiling — least-strict admissible per amendment
#       § 2 wording correction A8e fix F6; per case C-073 ArchitectAI
#       commit authority)
#
# Total: 37 files. Progression A0=24 → A1=25 (PREREG amendment) → A3=26
# (agent_models.rs) → A5=27 (budget_regime.rs) → A6=28 (fc_trace.rs) →
# A7=31 (llm_proxy.py + 2 smoke scripts) → A8e=33 (run_id.rs +
# test_llm_proxy.py) → A8e2=34 (llm_proxy_python_conformance.rs Rust
# wrapper) → A8e7=35 (A8_AUDIT_HISTORY append-only chronology) →
# A8e11=37 (audit runner scripts run_codex_phase_a8_exit_audit.sh +
# run_gemini_phase_a8_exit_audit.py — per C-075 gate-machinery
# tooling that produces audit transcripts must be Trust Root, both
# round-8 N3 false-closure and round-9 N3-followup defects
# demonstrated these are load-bearing gate tools).
# genesis_payload.toml itself is conceptually frozen but
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
"rules/MANIFEST.sha256" = "6d6fd54d46352213a857fdaf5cf70e8a893bbbd5467e5da45a3ed3314d01b0c2"
"rules/engine.py" = "932d9a2b7a3249a7eb5825c0b5c714a9913cd9aa9e058f789e64992b140e40b3"
".claude/hooks/judge.sh" = "f1f8f5094057dcea3c345951f7e73fd4d269f074a407829f19e4d75830a55fc2"
"tests/fc_alignment_conformance.rs" = "b3e75979ad2d175b9c45135be6ea1d94ce95184c6896468330c12dbfc1f719db"
"src/kernel.rs" = "893fd67534caf7a3d9abd6efbd202556348b6491cd6d4c6bdb224d2ad75b1af0"
"src/wal.rs" = "1ac7637aa09531b1c7232163f5df48f7193251594c4ed20e0d0fc85cea8f84dc"
"src/boot.rs" = "57a0609c4e4e70322419ebe31abe92432f80f2d3298aab29e90d45e4d91942fc"
"src/bus.rs" = "df28ffe514a3272a3d10fca4568fd424a76e754e9785c109a5459f163f7fd14c"
"src/drivers/llm_http.rs" = "615596b68956b4a8925110edc99aa746a5543527724ec394bb9dda163147ed7a"
"src/drivers/llm_proxy.py" = "22f632234a6d3e1a0547651537cf26846c64c47b85f5b1c3a1bcd1ae1f139997"
"scripts/smoke_siliconflow.sh" = "6ad54e7c0ab8221f475360dcad80eeeb0d6da0fd55c27acdb1cefb2b390f5341"
"scripts/_smoke_siliconflow.py" = "858d3b219c7e2d01cc370beb000e107b0abb8a211a63b6bf09d072afd02aeb24"
"scripts/test_llm_proxy.py" = "e1dd0306609fb3d2578b078bb21f0090c23978981f674ca0c6e3a60485ebf273"
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
"experiments/minif2f_v4/src/run_id.rs" = "5ff9db039131eaf4efeb65303d886265ef4c4f7e0e0f743467a7cbf22061781c"
"experiments/minif2f_v4/src/bin/evaluator.rs" = "537af991c9168c40ac3d866b164a274b4a587b098bb0fc471238a0ef37a37c6d"
"constitution.md" = "91e5913603eebaf134ac93d0f9ed7bb2b7916132203f6c2fae84d73329056ad6"
"cases/MANIFEST.sha256" = "eccbe35f776c64225d50c461780d1fe75a0a8510adc62b0dd2e96b46dc2dc555"
"handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json" = "69be702e3e0a4bb33c62bd02056495309289fd9271972dc733b4a1ba3dd908e1"
"handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" = "64d565b4451d1f770425530bb7ffa874da602ddfd9d714ce7f7db6bbfab69463"
"handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md" = "e1a0c97921fd77bd5a1268589bafbdc90e18e26a4feb6c86c6d8a91048d095e1"
"experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs" = "d62ee67cb564355fa26959c1f92a1152f95ab55d987c005abf7810e2da7b83b8"
"handover/audits/A8_AUDIT_HISTORY_2026-04-26.md" = "c39230ad1c910380e79024a5d1dbfe6b17758b150d7505320447a93c1c5e9837"
"handover/audits/run_codex_phase_a8_exit_audit.sh" = "a4555bd097eedc611a10692b91c26bfa4c0a9e76ef3087dba88f21bf5b2c7742"
"handover/audits/run_gemini_phase_a8_exit_audit.py" = "529a1b7ea8c870bec8083d0177854c41a8cc1c2c64836027bad9c71f55b441cc"

```

---

Give your INDEPENDENT Phase A → B exit audit. Cite §/file:line for every finding.

codex
I’ll audit the checked-out repo against the packet rather than relying on the packet’s claims. First I’m reading the local guidance and then I’ll spot-check the high-risk seams: Trust Root counts/hashes, amendment wording, proxy routing, run_id wiring, and the test gates.
exec
/bin/bash -lc 'pwd && git status --short && git rev-parse --short HEAD' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4
?? handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R12.md
c5b3711

exec
/bin/bash -lc 'find .. -name AGENTS.md -print' in /home/zephryj/projects/turingosv4
 succeeded in 1160ms:
../turingos/AGENTS.md
../omega_kepler_discovery/AGENTS.md
../omega/AGENTS.md
../turingosv2/AGENTS.md
../turingclaw/AGENTS.md

codex
The repo is on `c5b3711` with one untracked audit transcript file already present. I’m not going to modify or overwrite it; I’m using the working tree as evidence and treating that untracked file as outside the audited source unless it affects a cited claim.
exec
/bin/bash -lc "rg -n \"Total:|Total: 37|Trust Root|37|38|A8e13|Round 12|pending|src/boot.rs|verify_child_manifest|PASS \\(3/3|265 PASS|267 PASS|Trust Root entries\" genesis_payload.toml handover/audits/A8_EXIT_PACKET_2026-04-26.md handover/audits/A8_AUDIT_HISTORY_2026-04-26.md experiments/minif2f_v4/tests/trust_root_immutability.rs src/boot.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/tests/trust_root_immutability.rs:1:// PPUT-CCL Phase B7 — Trust Root immutability (PREREG § 1.8 + § 7 Gate B).
experiments/minif2f_v4/tests/trust_root_immutability.rs:3:// Boot computes SHA-256 of every Trust Root file at process start and
experiments/minif2f_v4/tests/trust_root_immutability.rs:7:// Trust Root manifest (PREREG § 1.8 + audit additions through 2026-04-25):
experiments/minif2f_v4/tests/trust_root_immutability.rs:52:    verify_trust_root(&repo_root()).expect("intact repo Trust Root verifies");
experiments/minif2f_v4/tests/trust_root_immutability.rs:58:    // with a single Trust Root entry whose recorded hash does not match
experiments/minif2f_v4/tests/trust_root_immutability.rs:82:    // MUST be in the Trust Root manifest. B6 added prompt_guard.rs.
experiments/minif2f_v4/tests/trust_root_immutability.rs:91:        // A8e13 fix Q1: src/boot.rs implements verify_trust_root itself
experiments/minif2f_v4/tests/trust_root_immutability.rs:94:        "src/boot.rs",
experiments/minif2f_v4/tests/trust_root_immutability.rs:154:            "Trust Root manifest missing required path: {path}\nactual keys: {keys:#?}"
src/boot.rs:1:// PPUT-CCL Phase B B7 — Trust Root + Boot freeze (PREREG § 1.8 + § 7).
src/boot.rs:94:        // not enforcement, before A8e13 fix Q1).
src/boot.rs:96:            verify_child_manifest(repo_root, &bytes)?;
src/boot.rs:106:fn verify_child_manifest(repo_root: &Path, bytes: &[u8]) -> Result<(), TrustRootError> {
src/boot.rs:314:    /// A8e13 fix Q1 conformance: child manifest tamper is detected even
handover/audits/A8_EXIT_PACKET_2026-04-26.md:47:| Trust Root manifest entries | **37** | `genesis_payload.toml [trust_root]` count + matches `trust_root_immutability::test_trust_root_manifest_includes_b2_b4_files` required-paths list |
handover/audits/A8_EXIT_PACKET_2026-04-26.md:49:| `bash scripts/smoke_siliconflow.sh` | **PASS (3/3 keys)** | live API; cost ~$0.005 per run |
handover/audits/A8_EXIT_PACKET_2026-04-26.md:123:- ✅ Trust Root verifies clean (`boot::tests::verify_trust_root_passes_on_intact_repo` PASS at 37-entry manifest).
handover/audits/A8_EXIT_PACKET_2026-04-26.md:145:- (Q2.c) The amendment's SHA-256 is in Trust Root. Does `boot::tests::verify_trust_root_passes_on_intact_repo` pass cleanly?
handover/audits/A8_EXIT_PACKET_2026-04-26.md:150:- (Q3.c) Trust Root manifest 37 entries. Are all entries load-bearing (i.e., does tampering each one weaken the constitutional gate)?
handover/audits/A8_EXIT_PACKET_2026-04-26.md:153:- (Q4.a) Spot-check: re-run `cargo test --workspace`; expect 265 PASS / 29 ignored / 0 failed.
handover/audits/A8_EXIT_PACKET_2026-04-26.md:155:- (Q4.c) Spot-check: re-run `bash scripts/smoke_siliconflow.sh`; expect PASS (3/3 keys; live API; cost ~$0.005).
genesis_payload.toml:68:# Total: 37 files. Progression A0=24 → A1=25 (PREREG amendment) → A3=26
genesis_payload.toml:73:# A8e11=37 (audit runner scripts run_codex_phase_a8_exit_audit.sh +
genesis_payload.toml:75:# tooling that produces audit transcripts must be Trust Root, both
genesis_payload.toml:91:source_pool_sha256 = "77179cf2598b0df707d78a6663e763121dfe8e73a6538073155f488feab95093"
genesis_payload.toml:98:"src/main.rs" = "622fee2d96a980d24f9fbaab3d0531c195a0a337fc3ddd2efb60bca90a1cfbf9"
genesis_payload.toml:107:"src/wal.rs" = "1ac7637aa09531b1c7232163f5df48f7193251594c4ed20e0d0fc85cea8f84dc"
genesis_payload.toml:108:"src/boot.rs" = "57a0609c4e4e70322419ebe31abe92432f80f2d3298aab29e90d45e4d91942fc"
genesis_payload.toml:111:"src/drivers/llm_proxy.py" = "22f632234a6d3e1a0547651537cf26846c64c47b85f5b1c3a1bcd1ae1f139997"
genesis_payload.toml:113:"scripts/_smoke_siliconflow.py" = "858d3b219c7e2d01cc370beb000e107b0abb8a211a63b6bf09d072afd02aeb24"
genesis_payload.toml:115:"src/sdk/prompt_guard.rs" = "b4f7b164770d1a7203b8143f773c66f748994d60a42ece38471f2f7f2839f4f1"
genesis_payload.toml:126:"experiments/minif2f_v4/src/bin/evaluator.rs" = "537af991c9168c40ac3d866b164a274b4a587b098bb0fc471238a0ef37a37c6d"
genesis_payload.toml:133:"handover/audits/A8_AUDIT_HISTORY_2026-04-26.md" = "c39230ad1c910380e79024a5d1dbfe6b17758b150d7505320447a93c1c5e9837"
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:5:**Constitutional pattern**: mirrors `constitution.md` + Art. V.3 amendment log; mirrors `PREREG_PPUT_CCL_2026-04-26.md` (frozen) + `PREREG_AMENDMENT_p0_defer_2026-04-25.md` (delta). Per C-075 (DO-178C tool qualification) the gate machinery itself must be qualifiable; this history doc is in Trust Root so the chronology cannot be silently rewritten.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:16:- Trust Root: 30-entry manifest (round-1 packet's count; later corrected to 31 — see round-2)
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:27:4. (Codex#5) Trust Root manifest count off-by-1 (packet says 30, actual 31); genesis_payload.toml header still says "Total: 25 files".
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:38:- **F5** Trust Root count + genesis_payload.toml header reconciled to 31 entries (corrected for A3's `agent_models.rs` already in TR before A1). Closes #4.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:48:- Trust Root: 33-entry manifest
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:71:- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 15/15 PASS (Python)
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:72:- Trust Root: 34-entry manifest
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:86:**Round-3 fixes shipped (`A8e3`, commit `3d38ba5`)** — six fixes:
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:91:- **H5** TRACE_MATRIX § 5 item 7: "(commit pending) + 6 wired" → "CLOSED + 9 wired".
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:99:- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `3d38ba5` (post-A8e3)
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:100:- Test baseline: 265 PASS / 29 ignored / 0 failed; 15/15 Python
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:101:- Trust Root: 34-entry manifest
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:109:1. (Codex R4#1) Packet title + date metadata still say "round 2"; per-atom Trust Root deltas in § 3 stale (A5 says 25→26 vs actual 26→27, A6 26→27 vs 27→28, A7 27→30 vs 28→31).
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:114:**Round-4 fixes shipped (`A8e4`, commit `8693789`)** — three fixes:
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:116:- **I2** Per-atom Trust Root deltas in packet § 3 corrected: A5 → 26→27, A6 → 27→28, A7 → 28→31. TRACE_MATRIX § 1 line 11 anchor-site location corrected: "5 in run_swarm + 1 in run_oneshot".
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:124:- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `8693789` (post-A8e4)
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:125:- Test baseline: 265 PASS / 29 ignored / 0 failed; 15/15 Python
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:126:- Trust Root: 34-entry manifest
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:134:1. (Codex R5#1) Packet missing Round-4 outcome + A8e4 fixes shipped section; `<pending>` commit placeholders for A8e2/A8e3/A8e4 never replaced.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:140:- **J2** `<pending>` placeholders replaced with actual SHAs (`0af47b7` / `3d38ba5` / `8693789`); A8e5 self-reference uses "this commit".
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:151:- Test baseline: 265 PASS / 29 ignored / 0 failed; 15/15 Python
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:152:- Trust Root: 34-entry manifest
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:160:1. (Codex R6#1) RQ14 closure criterion contradicts itself — packet has `<pending>` placeholders in J2 bullet + RQ14 itself.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:167:- **K1** `<pending>` placeholder closure: J2 bullet + RQ14 reworded.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:181:- Test baseline: 265 PASS / 29 ignored / 0 failed; 16/16 Python proxy tests
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:182:- Trust Root: 35-entry manifest (A8_AUDIT_HISTORY added)
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:190:1. (Codex R7#1) Packet § 2 + § 4 still report 34 Trust Root entries; actual is 35 after A8e7 added `A8_AUDIT_HISTORY_2026-04-26.md`. Stale reference at multiple sites in the packet.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:191:2. (Codex R7#2) This history doc says A8e7 commit is `<this commit's SHA>` and "Round 7 dual audit pending after this commit lands" — but A8e7 is already in Trust Root as landed. Same placeholder-staleness pattern earlier rounds tried to eliminate.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:202:- **M2** (Codex R7#2): This history doc's A8e7 entry now stamps the actual SHA `aaedc9d` and round-7 verdicts (above) instead of "pending". Round-7 history sealed.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:204:- **M4** (Codex R7#4): PREREG_AMENDMENT § 2 + § 3 + § 8 reconciled: removed the false claim that § 3 "ensures calibration before Phase E"; § 8 now reads as the operative rule (Phase E proceeds with the substitution if § 3 conditions haven't completed). Clean single semantics: substitution is permitted throughout Phase B → Phase E; calibration UPGRADES the bar IF and WHEN the § 3 conditions all complete. Re-hashed in Trust Root.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:213:- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 16/16 PASS (Python)
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:214:- Trust Root: 35-entry manifest
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:223:2. (Codex R8#2) This history doc's round-7 entry left an unresolved placeholder `<aaedc9d-successor>` for the A8e8 commit SHA; cumulative table row 7 still says "pending pending pending" despite round-7 verdicts being recorded above.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:229:- **N3** (Codex R8#3): Runner script header metadata refreshed to current state (265 PASS / 35-entry manifest); pre-A8e values removed.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:239:- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 16/16 PASS (Python)
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:240:- Trust Root: 37-entry manifest
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:247:- **Self-audit pass** before R11 — caught 4 documentary defects: runner header `35-entry` → `37-entry`, genesis comment "14 rules" → "15 rules", and regenerated cases/MANIFEST + rules/MANIFEST + judge.sh re-hashes. C-076 yaml + R-020 yaml are covered by their respective MANIFEST proxies (Trust Root manifest unchanged at 37 entries).
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:255:- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 16/16 PASS (Python)
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:256:- Trust Root: 35-entry manifest
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:277:- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 16/16 PASS (Python)
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:278:- Trust Root: 35-entry manifest
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:283:- Merged: **CHALLENGE**. Codex caught two real defects in gate machinery (smoke + runner Trust Root coverage); Gemini did not.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:286:1. (**Codex R10#1 — substantive**) `_smoke_siliconflow.py` doesn't fail closed on missing keys: the script skipped unset secondary/tertiary and returned PASS if any configured key responded. The packet's "PASS (3/3 keys)" claim was strictly stronger than what the smoke verified — a primary-only setup would silently PASS, replicating exactly the V3L-27 single-key collapse pattern the 3-key pool was meant to mitigate.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:287:2. (**Codex R10#2 — substantive**) Audit runner scripts (`run_codex_phase_a8_exit_audit.sh` + `run_gemini_phase_a8_exit_audit.py`) are load-bearing gate tools (demonstrated by R8/R9's runner-default + false-closure defects) but were NOT in Trust Root. Per case C-075 (DO-178C tool qualification): tools producing data used by future dual audit must enter Trust Root.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:291:- **P2** (Codex R10#2): `run_codex_phase_a8_exit_audit.sh` + `run_gemini_phase_a8_exit_audit.py` added to Trust Root manifest. `trust_root_immutability` test required-paths list extended. Manifest 35 → 37.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:299:- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 16/16 PASS (Python)
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:300:- Trust Root: 37-entry manifest
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:308:1. (**Codex R11#1 — substantive architectural**) Trust Root proxy via `cases/MANIFEST.sha256` + `rules/MANIFEST.sha256` is documented as covering the child yaml files but not actually enforced at boot. `verify_trust_root` only hashes the flat `[trust_root]` entries; the manifest's child contents are not verified. A direct edit to a case or rule yaml without regenerating its parent manifest leaves boot still passing — silent governance drift.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:316:**Round-11 fixes shipped (`A8e13`, this commit)** — six fixes (2 substantive + 4 documentary):
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:317:- **Q1** (Codex R11#1 — substantive arch): `src/boot.rs` extended with `verify_child_manifest`. When a Trust Root entry path ends in `/MANIFEST.sha256`, the verifier now ALSO parses the manifest contents (GNU sha256sum format) and verifies each child file's actual hash against the claim. Manifests regenerated with repo-relative paths (was a mix of bare-filename and dir-relative). Two new unit tests pin the contract (`verify_trust_root_detects_child_manifest_tamper` + `verify_trust_root_passes_with_matching_child_manifest`). Plus: `src/boot.rs` itself added to Trust Root (meta-finding caught during fix — the verifier file was not previously protected by the verifier; tampering with boot.rs would silently bypass the entire gate). Manifest 37 → 38.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:318:- **Q2** (Gemini R11#3 — substantive refactor): `make_pput` signature `Option<u64>` → plain `u64` for the 3 affected parameters. All 9 production call sites + 4 test fixtures updated. `wc.elapsed_ms()` returns `Option<u64>` upstream; call sites add `.unwrap_or(0)` at the boundary. `cargo test --workspace`: 267 PASS (was 265 + 2 new boot tests).
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:319:- **Q3** (Codex R11#2): `A8_AUDIT_HISTORY` extended with A8e12 entry + Round 11 outcome + A8e13 fixes shipped section + cumulative table rows for round 11/12.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:341:| 12 | pending | pending | pending | — | — | ~5 |

exec
/bin/bash -lc "rg -n \"fn run_swarm|fn run_oneshot|fn make_pput|mint_run_id|emit_event\\(|make_pput\\(\" experiments/minif2f_v4/src/bin/evaluator.rs experiments/minif2f_v4/src/run_id.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/src/run_id.rs:21:pub fn mint_run_id(condition: &str, problem_file: &str) -> String {
experiments/minif2f_v4/src/run_id.rs:42:        let id = mint_run_id("n3", "/tmp/foo.lean");
experiments/minif2f_v4/src/run_id.rs:54:        let id = mint_run_id("oneshot", "/");
experiments/minif2f_v4/src/run_id.rs:60:        let a = mint_run_id("n3", "/tmp/p.lean");
experiments/minif2f_v4/src/run_id.rs:61:        let b = mint_run_id("oneshot", "/tmp/p.lean");
experiments/minif2f_v4/src/bin/evaluator.rs:314:async fn run_oneshot(
experiments/minif2f_v4/src/bin/evaluator.rs:339:    let run_id = minif2f_v4::run_id::mint_run_id("oneshot", problem_file);
experiments/minif2f_v4/src/bin/evaluator.rs:386:                return make_pput(problem_file, "oneshot", model,
experiments/minif2f_v4/src/bin/evaluator.rs:411:            minif2f_v4::fc_trace::emit_event(
experiments/minif2f_v4/src/bin/evaluator.rs:441:                    make_pput(problem_file, "oneshot", model,
experiments/minif2f_v4/src/bin/evaluator.rs:453:                    make_pput(problem_file, "oneshot", model,
experiments/minif2f_v4/src/bin/evaluator.rs:465:                    make_pput(problem_file, "oneshot", model,
experiments/minif2f_v4/src/bin/evaluator.rs:488:async fn run_swarm(
experiments/minif2f_v4/src/bin/evaluator.rs:499:    let run_id = minif2f_v4::run_id::mint_run_id(&condition, problem_file);
experiments/minif2f_v4/src/bin/evaluator.rs:723:            minif2f_v4::fc_trace::emit_event(
experiments/minif2f_v4/src/bin/evaluator.rs:733:            let mut result = make_pput(problem_file, &condition, &run_model_label,
experiments/minif2f_v4/src/bin/evaluator.rs:775:            minif2f_v4::fc_trace::emit_event(
experiments/minif2f_v4/src/bin/evaluator.rs:1017:                                minif2f_v4::fc_trace::emit_event(
experiments/minif2f_v4/src/bin/evaluator.rs:1041:                                        minif2f_v4::fc_trace::emit_event(
experiments/minif2f_v4/src/bin/evaluator.rs:1113:                                        minif2f_v4::fc_trace::emit_event(
experiments/minif2f_v4/src/bin/evaluator.rs:1141:                                        return make_pput(problem_file, &condition, &run_model_label,
experiments/minif2f_v4/src/bin/evaluator.rs:1318:                                minif2f_v4::fc_trace::emit_event(
experiments/minif2f_v4/src/bin/evaluator.rs:1358:                                        minif2f_v4::fc_trace::emit_event(
experiments/minif2f_v4/src/bin/evaluator.rs:1370:                                        return make_pput(problem_file, &condition, &run_model_label,
experiments/minif2f_v4/src/bin/evaluator.rs:1460:    minif2f_v4::fc_trace::emit_event(
experiments/minif2f_v4/src/bin/evaluator.rs:1470:    make_pput(problem_file, &condition, &run_model_label,
experiments/minif2f_v4/src/bin/evaluator.rs:1484:fn make_pput(
experiments/minif2f_v4/src/bin/evaluator.rs:1769:        let result = make_pput(
experiments/minif2f_v4/src/bin/evaluator.rs:1834:        let result = make_pput(
experiments/minif2f_v4/src/bin/evaluator.rs:1877:        let result = make_pput(
experiments/minif2f_v4/src/bin/evaluator.rs:1935:        let mut result = make_pput(

exec
/bin/bash -lc "rg -n \"def detect_provider|deepseek-ai|if \\\"/\\\" in model|if \\\"deepseek\\\"|test_deepseek_slash_form_routes_to_siliconflow|unittest|OK\" src/drivers/llm_proxy.py scripts/test_llm_proxy.py experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
scripts/test_llm_proxy.py:17:import unittest
scripts/test_llm_proxy.py:33:class RoutingMatrixTests(unittest.TestCase):
scripts/test_llm_proxy.py:76:    def test_deepseek_slash_form_routes_to_siliconflow(self):
scripts/test_llm_proxy.py:77:        # A8e6 fix K2 (Codex R6#2): `deepseek-ai/DeepSeek-R1-Distill-*`
scripts/test_llm_proxy.py:86:            llm_proxy.detect_provider("deepseek-ai/DeepSeek-R1-Distill-Qwen-7B"),
scripts/test_llm_proxy.py:90:            llm_proxy.detect_provider("deepseek-ai/DeepSeek-V2.5"),
scripts/test_llm_proxy.py:113:class StripProviderPrefixTests(unittest.TestCase):
scripts/test_llm_proxy.py:140:class RoundRobinTests(unittest.TestCase):
scripts/test_llm_proxy.py:211:class StatsAggregationTests(unittest.TestCase):
scripts/test_llm_proxy.py:236:    unittest.main(verbosity=2)
experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs:93:    // Sanity-check the unittest summary line is present so a future
experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs:97:        stderr.contains("OK") || stdout.contains("OK"),
experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs:98:        "unittest output must contain the trailing 'OK' line.\n\
src/drivers/llm_proxy.py:196:def detect_provider(model):
src/drivers/llm_proxy.py:207:                                            `deepseek-ai/<distill>` SF-
src/drivers/llm_proxy.py:219:    style `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B` is a SiliconFlow-
src/drivers/llm_proxy.py:223:    slash check, so `deepseek-ai/...` misrouted to api.deepseek.com
src/drivers/llm_proxy.py:232:    if "/" in model:
src/drivers/llm_proxy.py:234:        # deepseek-ai/...) goes to the heterogeneous catalog provider.
src/drivers/llm_proxy.py:240:    if "deepseek" in m:
src/drivers/llm_proxy.py:242:        # Slash-form like "deepseek-ai/..." won't reach here per K2.

exec
/bin/bash -lc 'rg -n "least-strict|strictest|Phase E|ensure|guarantee|PRE-REQUISITES|operationally permitted|baseline_regression_rate" handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md genesis_payload.toml' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
genesis_payload.toml:6:#                         (PREREG § 1.8). baseline_regression_rate +
genesis_payload.toml:64:#       max-tolerated ceiling — least-strict admissible per amendment
genesis_payload.toml:92:baseline_regression_rate = 0.0  # PLACEHOLDER until Phase B7-extra calibration
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:23:PREREG § 5.5 calibration **DEFERRED** indefinitely with the following operative substitution for Phase B → C transition and Phase E Gate H requirements:
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:27:**Wording correction (A8 round-1 audit, Codex finding 1)**: an earlier draft of this section claimed `p_0 = 0.10` was the *strictest possible substitute*. That was backwards. The Gate H test is `j-RR ≤ p_0`, so a SMALLER `p_0` is stricter. `0.10` is the **maximum tolerated value** the PREREG admits — i.e., the LEAST strict admissible ceiling, not the strictest. Substituting it is the equivalent of running Gate H at the weakest threshold the PREREG ever sanctioned. The substitution is *operationally permitted* (the PREREG explicitly allows up to 0.10), but it is NOT a tighter-than-original guarantee.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:31:- The substitution may be LESS protective than an eventual calibrated `p_0 < 0.10`. If/when calibration runs (per § 3 conditions, which are PRE-REQUISITES — they do not by themselves *guarantee* calibration completes before any specific phase), the empirical estimate may tighten the bar; until then Gate H operates at the ceiling.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:32:- Acceptable trade-off because (a) Phase B and C don't gate on `j-RR` (Gate H is a Phase E artifact gate), and (b) the substitution is **operationally permitted at any phase including Phase E** — see § 8 for the operative rule. If § 3 conditions complete before Phase E, calibration UPGRADES the bar; if not, Phase E proceeds with the ceiling substitution. There is no path under this amendment where Phase E is blocked by absence of calibration — that was an earlier draft's misclaim, removed by A8e8 fix M4 after Codex round-7 audit.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:34:**`genesis_payload.toml [pput_accounting_0].baseline_regression_rate`**: setting deferred to ArchitectAI commit window. Current value `0.0` is recognized as INVALID PLACEHOLDER (would auto-fail any artifact with any regression). Until calibration runs, **Gate H consumers MUST hardcode `p_0 = 0.10`** at the consumption site, not read from `genesis_payload.toml`.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:50:5. **Phase D ArchitectAI runtime exists** — calibration is part of Gate H gating Phase E. Doing it before Phase D = calibrating against a counterfactual ArchitectAI that doesn't exist.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:65:Phase B → C dual-audit packet (next major milestone) must reference this amendment + show that Phase E Gate H consumer hardcodes `p_0 = 0.10`.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:94:- Does the amendment violate any PREREG § 5.5 constraint? (Should not — defer is operationally permitted given § 5.5 ceiling.)
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:95:- Does substitution of `p_0 = 0.10` invalidate any Gate H statistical claim? (Should not — `j-RR` is descriptive (PREREG § 5.4), outside the inferential family, so no Type-I inflation. Per § 2 wording correction: 0.10 is the LEAST-strict admissible ceiling, NOT a tighter-than-original substitute — the substitution may be less protective than an eventual calibrated `p_0 < 0.10`, but it is **operationally permitted at any phase including Phase E** by the original PREREG § 5.5 ceiling. § 3 conditions are PRE-REQUISITES for calibration to run at all; they do NOT guarantee calibration completes before any specific phase.)
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:96:- Does deferral leave any phase blocked indefinitely? (Should not — § 3 lists explicit re-calibration conditions; if those never met, Phase E proceeds with the operationally-permitted ceiling substitution per § 2 final paragraph. Calibration UPGRADES the bar IF and WHEN § 3 conditions complete; absence of calibration is not a Phase E blocker.)

codex
The first pass already shows a likely documentary fault: current code now includes `src/boot.rs` in the Trust Root, but the packet and genesis header still advertise 37 entries. I’m running the stated gates now to see whether that is only stale text or whether the boot verifier/test claims have drifted too.
exec
/bin/bash -lc 'cargo test --workspace' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc "python3 - <<'PY'
import re
text=open('genesis_payload.toml').read().splitlines()
in_tr=False
entries=[]
for i,line in enumerate(text,1):
    s=line.strip()
    if s=='[trust_root]':
        in_tr=True; continue
    if in_tr and s.startswith('['):
        break
    if in_tr and s.startswith('\"') and ' = ' in s:
        entries.append((i,s.split(' = ',1)[0].strip('\"')))
print(len(entries))
for line,path in entries:
    print(f'{line}:{path}')
PY" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
38
98:src/main.rs
99:Cargo.lock
100:handover/preregistration/scripts/run_p0_calibration.sh
101:handover/preregistration/scripts/compute_p0.py
102:rules/MANIFEST.sha256
103:rules/engine.py
104:.claude/hooks/judge.sh
105:tests/fc_alignment_conformance.rs
106:src/kernel.rs
107:src/wal.rs
108:src/boot.rs
109:src/bus.rs
110:src/drivers/llm_http.rs
111:src/drivers/llm_proxy.py
112:scripts/smoke_siliconflow.sh
113:scripts/_smoke_siliconflow.py
114:scripts/test_llm_proxy.py
115:src/sdk/prompt_guard.rs
116:experiments/minif2f_v4/src/lean4_oracle.rs
117:experiments/minif2f_v4/src/cost_aggregator.rs
118:experiments/minif2f_v4/src/wall_clock.rs
119:experiments/minif2f_v4/src/post_hoc_verifier.rs
120:experiments/minif2f_v4/src/jsonl_schema.rs
121:experiments/minif2f_v4/src/rollback_sim.rs
122:experiments/minif2f_v4/src/agent_models.rs
123:experiments/minif2f_v4/src/budget_regime.rs
124:experiments/minif2f_v4/src/fc_trace.rs
125:experiments/minif2f_v4/src/run_id.rs
126:experiments/minif2f_v4/src/bin/evaluator.rs
127:constitution.md
128:cases/MANIFEST.sha256
129:handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json
130:handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md
131:handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md
132:experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs
133:handover/audits/A8_AUDIT_HISTORY_2026-04-26.md
134:handover/audits/run_codex_phase_a8_exit_audit.sh
135:handover/audits/run_gemini_phase_a8_exit_audit.py

exec
/bin/bash -lc 'python3 scripts/test_llm_proxy.py' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc 'bash scripts/smoke_siliconflow.sh' in /home/zephryj/projects/turingosv4
 succeeded in 1181ms:
test_single_key_pool_always_returns_index_zero (__main__.RoundRobinTests.test_single_key_pool_always_returns_index_zero) ... ok
test_three_key_round_robin_distributes_evenly (__main__.RoundRobinTests.test_three_key_round_robin_distributes_evenly) ... ok
test_two_key_pool_alternates (__main__.RoundRobinTests.test_two_key_pool_alternates) ... ok
test_bare_qwen_routes_to_dashscope (__main__.RoutingMatrixTests.test_bare_qwen_routes_to_dashscope) ... ok
test_deepseek_slash_form_routes_to_siliconflow (__main__.RoutingMatrixTests.test_deepseek_slash_form_routes_to_siliconflow) ... ok
test_deepseek_substring (__main__.RoutingMatrixTests.test_deepseek_substring) ... ok
test_default_fallback_dashscope (__main__.RoutingMatrixTests.test_default_fallback_dashscope) ... ok
test_explicit_prefix_wins (__main__.RoutingMatrixTests.test_explicit_prefix_wins) ... ok
test_other_huggingface_style_routes_to_siliconflow (__main__.RoutingMatrixTests.test_other_huggingface_style_routes_to_siliconflow) ... ok
test_qwen_huggingface_style_routes_to_siliconflow (__main__.RoutingMatrixTests.test_qwen_huggingface_style_routes_to_siliconflow) ... ok
test_unknown_explicit_prefix_falls_through_to_heuristic (__main__.RoutingMatrixTests.test_unknown_explicit_prefix_falls_through_to_heuristic) ... ok
test_get_stats_includes_per_key_distribution (__main__.StatsAggregationTests.test_get_stats_includes_per_key_distribution) ... ok
test_reset_clears_per_key_counters (__main__.StatsAggregationTests.test_reset_clears_per_key_counters) ... ok
test_leaves_bare_model_intact (__main__.StripProviderPrefixTests.test_leaves_bare_model_intact) ... ok
test_leaves_unknown_prefix_intact (__main__.StripProviderPrefixTests.test_leaves_unknown_prefix_intact) ... ok
test_strips_known_prefix (__main__.StripProviderPrefixTests.test_strips_known_prefix) ... ok

----------------------------------------------------------------------
Ran 16 tests in 0.002s

OK

 succeeded in 7023ms:
[A7-smoke] SiliconFlow probe — model=Qwen/Qwen2.5-7B-Instruct max_tokens=8
  [primary  ] SILICONFLOW_API_KEY: OK   2287ms; tokens prompt=36 completion=1; content[:32]='ack'
  [secondary] SILICONFLOW_API_KEY_SECONDARY: OK   1352ms; tokens prompt=36 completion=1; content[:32]='ack'
  [tertiary ] SILICONFLOW_API_KEY_TERTIARY: OK   1358ms; tokens prompt=36 completion=1; content[:32]='ack'
[A7-smoke] result: PASS (3/3 keys configured + responded)

 succeeded in 29857ms:
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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 9.90s
     Running unittests src/lib.rs (target/debug/deps/minif2f_v4-2121f01f9a6ef751)

running 74 tests
test agent_models::tests::csv_entries_trimmed ... ok
test agent_models::tests::empty_csv_slot_rejected ... ok
test agent_models::tests::empty_parsed_broadcasts_global_model ... ok
test agent_models::tests::empty_env_parses_to_empty_vec ... ok
test agent_models::tests::heterogeneous_with_gate_passes ... ok
test agent_models::tests::positional_length_match_passes ... ok
test agent_models::tests::single_entry_broadcasts ... ok
test agent_models::tests::length_mismatch_rejected ... ok
test agent_models::tests::heterogeneous_without_gate_rejected ... ok
test agent_models::tests::single_entry_parses ... ok
test agent_models::tests::uniform_length_n_does_not_trip_hetero_gate ... ok
test budget_regime::tests::effective_per_agent_scales_linearly_in_n ... ok
test budget_regime::tests::effective_token_total_unimplemented ... ok
test budget_regime::tests::effective_per_agent_overflow_rejected ... ok
test budget_regime::tests::effective_wall_clock_unimplemented ... ok
test budget_regime::tests::label_strings_are_stable ... ok
test budget_regime::tests::n_agents_zero_does_not_panic ... ok
test budget_regime::tests::parse_max_transactions_empty_defaults_to_200 ... ok
test budget_regime::tests::parse_max_transactions_garbage_rejected ... ok
test budget_regime::tests::parse_max_transactions_negative_rejected ... ok
test budget_regime::tests::effective_total_proposal_invariant_under_n ... ok
test budget_regime::tests::parse_regime_empty_defaults_to_total_proposal ... ok
test budget_regime::tests::parse_max_transactions_valid ... ok
test budget_regime::tests::parse_regime_known_values ... ok
test budget_regime::tests::parse_max_transactions_zero_rejected ... ok
test budget_regime::tests::parse_regime_unknown_rejected ... ok
test budget_regime::tests::resolve_budget_default_preserves_phase_b_baseline ... ok
test budget_regime::tests::resolve_budget_token_total_startup_fatal ... ok
test cost_aggregator::tests::test_empty_accumulator_zero_total ... ok
test budget_regime::tests::resolve_budget_per_agent_via_env ... ok
test cost_aggregator::tests::test_failed_branches_counted_in_total_cost ... ok
test budget_regime::tests::resolve_budget_unknown_regime_via_env ... ok
test cost_aggregator::tests::test_tool_stdout_chars_div_4_approximation ... ok
test cost_aggregator::tests::test_flip_underflow_panics - should panic ... ok
test fc_trace::tests::emit_event_with_full_payload_does_not_panic ... ok
test fc_trace::tests::emit_event_with_no_kv_or_agent_does_not_panic ... ok
test fc_trace::tests::emit_is_no_op_when_disabled ... ok
test fc_trace::tests::fc_id_display_matches_as_str ... ok
test fc_trace::tests::json_str_escapes_required_chars ... ok
test jsonl_schema::tests::test_a4_tactic_diversity_helper ... ok
test fc_trace::tests::fc_id_strings_are_stable ... ok
test jsonl_schema::tests::test_a4_verifier_wait_bounded_by_total_wall_time ... ok
test jsonl_schema::tests::test_a4_decomposed_metrics_round_trip ... ok
test jsonl_schema::tests::test_a5_budget_regime_round_trip ... ok
test jsonl_schema::tests::test_jsonl_schema_v2_round_trip ... ok
test jsonl_schema::tests::test_pput_verified_zero_when_progress_zero ... ok
test lean4_oracle::tests::test_clean_tactic_accepted ... ok
test jsonl_schema::tests::test_legacy_jsonl_still_readable ... ok
test lean4_oracle::tests::test_forbidden_io_process ... ok
test lean4_oracle::tests::test_decide_tactic_permitted ... ok
test lean4_oracle::tests::test_forbidden_native_decide ... ok
test lean4_oracle::tests::test_correct_theorem_name_accepted ... ok
test lean4_oracle::tests::test_identity_theft_rejected ... ok
test lean4_oracle::tests::test_sorry_in_word_not_rejected ... ok
test post_hoc_verifier::tests::test_no_runtime_accept_zeros_both_pput ... ok
test post_hoc_verifier::tests::test_post_hoc_verified_without_runtime_still_zero_progress ... ok
test lean4_oracle::tests::test_sorry_rejected ... ok
test lean4_oracle::tests::test_word_boundary_function ... ok
test post_hoc_verifier::tests::test_pput_verified_matches_runtime_when_both_accept ... ok
test rollback_sim::tests::does_not_fire_before_threshold ... ok
test post_hoc_verifier::tests::test_pput_verified_zero_when_lean_rejects ... ok
test rollback_sim::tests::env_var_name_matches_prereg ... ok
test rollback_sim::tests::does_not_fire_after_threshold ... ok
test rollback_sim::tests::fires_at_threshold_when_enabled ... ok
test rollback_sim::tests::never_fires_when_disabled ... ok
test rollback_sim::tests::threshold_constant_matches_prereg ... ok
test run_id::tests::distinguishes_conditions_for_same_problem ... ok
test run_id::tests::handles_path_with_no_stem ... ok
test run_id::tests::shape_matches_legacy_make_pput_format ... ok
test wall_clock::tests::test_wall_clock_first_read_to_final_accept ... ok
test wall_clock::tests::test_wall_clock_unmarked_returns_none ... ok
test wall_clock::tests::test_wall_clock_final_accept_overwrites ... ok
test wall_clock::tests::test_wall_clock_first_read_idempotent ... ok
test wall_clock::tests::test_wall_clock_no_final_accept_uses_now ... ok

test result: ok. 74 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running unittests src/bin/evaluator.rs (target/debug/deps/evaluator-680e8e25b06524bc)

running 9 tests
test swarm_condition_tests::rejects_hybrid_v1_and_other_named_conditions ... ok
test swarm_condition_tests::n1_is_distinct_from_oneshot ... ok
test swarm_condition_tests::parses_valid_n_swarm_conditions ... ok
test swarm_condition_tests::rejects_malformed_n_conditions ... ok
test swarm_condition_tests::rejects_oneshot_condition ... ok
test v2_emit_tests::test_a4_emit_max_tx_exhaustion_row ... ok
test v2_emit_tests::test_a4_synthetic_short_circuit_does_not_set_hit_max_tx ... ok
test v2_emit_tests::test_emit_dispatches_as_v2 ... ok
test v2_emit_tests::test_emit_soft_law_divergence_signal ... ok

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
test rollback_sim_env_var_canonical_name ... ok
test rollback_sim_env_check_function_present ... ok
test rollback_sim_predicate_logic_at_threshold ... ok
test rollback_sim_threshold_constant_matches_prereg ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 17.17s

     Running tests/heldout_operational_sealing.rs (target/debug/deps/heldout_operational_sealing-af83cb32d83b799b)

running 5 tests
test test_l2_agent_prompt_context_blacklist ... ok
test test_l4_hash_and_seed_substring_co_occurrence ... ok
test test_l5_source_pool_enumeration_block ... ok
test test_l1_file_path_read_isolation ... ok
test test_l3_tool_call_no_hash_invocation_in_agent_code ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

     Running tests/llm_proxy_python_conformance.rs (target/debug/deps/llm_proxy_python_conformance-a61830d3ca32dbe0)

running 1 test
test proxy_python_conformance_suite_passes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.99s

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
test test_golden_path_requires_ground_truth ... ok
test test_failed_branches_in_total_cost ... ok
test test_all_model_tokens_counted ... ok
test test_no_metric_file_access_by_agents ... ok
test test_no_pput_in_agent_prompt ... ok
test test_tool_stdout_hash_logged ... ok
test test_no_hidden_unmetered_generation ... ok
test test_no_problem_id_hardcode ... ok
test test_heldout_ids_inaccessible ... ok
test test_wall_clock_first_read_to_final_accept ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

     Running tests/trust_root_immutability.rs (target/debug/deps/trust_root_immutability-7c21a1ad6ce805d7)

running 4 tests
test test_pput_accounting_0_section_present ... ok
test test_trust_root_simulated_write_aborts ... ok
test test_trust_root_manifest_includes_b2_b4_files ... ok
test test_trust_root_immutable_at_boot ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

     Running unittests src/lib.rs (target/debug/deps/turingosv4-cd2d82dded9eba82)

running 130 tests
test boot::tests::parse_errors_on_unquoted_key ... ok
test boot::tests::parse_errors_when_section_missing ... ok
test boot::tests::parse_strips_inline_comment_and_blanks ... ok
test boot::tests::verify_trust_root_detects_tamper_in_tempdir ... ok
test boot::tests::verify_trust_root_detects_child_manifest_tamper ... ok
test bus::tests::test_bus_basic_append ... ok
test boot::tests::verify_trust_root_passes_when_hash_matches_in_tempdir ... ok
test bus::tests::test_bus_classify_bounded ... ok
test bus::tests::test_bus_creates_market_on_append ... ok
test bus::tests::test_bus_forbidden_pattern_veto ... ok
test boot::tests::verify_trust_root_passes_with_matching_child_manifest ... ok
test bus::tests::test_bus_graveyard_feedback ... ok
test bus::tests::test_bus_halt_and_settle ... ok
test bus::tests::test_bus_ledger_integrity ... ok
test bus::tests::test_bus_snapshot ... ok
test bus::tests::test_bus_payload_too_long ... ok
test bus::tests::test_bus_serial_ordering ... ok
test bus::tests::test_bus_unknown_agent_vetoed ... ok
test drivers::llm_http::tests::test_client_creation ... ok
test bus::tests::test_bus_too_many_lines ... ok
test drivers::llm_http::tests::test_generate_request_serialization ... ok
test drivers::llm_http::tests::test_driver_error_display ... ok
test kernel::tests::test_append_and_retrieve ... ok
test kernel::tests::test_market_lifecycle ... ok
test kernel::tests::test_golden_path_trace ... ok
test kernel::tests::test_market_ticker ... ok
test kernel::tests::test_no_market_for_nonexistent_node ... ok
test kernel::tests::test_no_duplicate_market ... ok
test kernel::tests::test_reject_duplicate ... ok
test kernel::tests::test_resolve_all_markets ... ok
test kernel::tests::test_reject_dangling_citation ... ok
test ledger::tests::test_ledger_hash_chain_integrity ... ok
test ledger::tests::test_ledger_omega_vocabulary ... ok
test ledger::tests::test_ledger_append_and_verify ... ok
test ledger::tests::test_ledger_sequence_monotonic ... ok
test ledger::tests::test_tape_append_with_valid_citation ... ok
test ledger::tests::test_ledger_tamper_detection ... ok
test ledger::tests::test_tape_dag_branching ... ok
test ledger::tests::test_tape_empty ... ok
test ledger::tests::test_tape_append_root_node ... ok
test ledger::tests::test_tape_reject_dangling_citation ... ok
test ledger::tests::test_tape_reject_duplicate_id ... ok
test ledger::tests::test_tape_trace_ancestors ... ok
test ledger::tests::test_tape_time_arrow_ordering ... ok
test prediction_market::tests::test_assassin_profit ... ok
test prediction_market::tests::test_buy_yes_increases_yes_price ... ok
test prediction_market::tests::test_constant_product_invariant ... ok
test prediction_market::tests::test_buy_no_increases_no_price ... ok
test prediction_market::tests::test_ctf_conservation_1_coin_1_yes_1_no ... ok
test prediction_market::tests::test_initial_price_is_50_50 ... ok
test prediction_market::tests::test_create_market ... ok
test prediction_market::tests::test_no_double_resolution ... ok
test prediction_market::tests::test_no_trading_after_resolution ... ok
test prediction_market::tests::test_multiple_traders_price_discovery ... ok
test prediction_market::tests::test_pioneer_profit ... ok
test prediction_market::tests::test_redeem_requires_resolution ... ok
test prediction_market::tests::test_reject_zero_or_negative_amounts ... ok
test prediction_market::tests::test_prices_sum_to_one ... ok
test sdk::actor::tests::test_boltzmann_returns_none_empty_tape ... ok
test sdk::actor::tests::test_boltzmann_never_returns_none_with_nodes ... ok
test sdk::actor::tests::test_frontier_detection_parent_with_child ... ok
test sdk::actor::tests::test_frontier_detection_leaf ... ok
test sdk::error_abstraction::tests::classifier_version_is_stamped ... ok
test sdk::error_abstraction::tests::fixture_linarith_failed ... ok
test sdk::error_abstraction::tests::fixture_other_catchall ... ok
test sdk::error_abstraction::tests::fixture_rewrite_no_match ... ok
test sdk::error_abstraction::tests::fixture_simp_no_progress ... ok
test sdk::error_abstraction::tests::fixture_type_mismatch ... ok
test sdk::error_abstraction::tests::fixture_unexpected_token ... ok
test sdk::error_abstraction::tests::fixture_unknown_constant ... ok
test sdk::error_abstraction::tests::fixture_unsolved_goals ... ok
test sdk::error_abstraction::tests::labels_are_unique_and_stable ... ok
test sdk::prompt::tests::test_prompt_contains_no_example_values ... ok
test sdk::actor::tests::test_boltzmann_diversity_not_deterministic ... ok
test sdk::prompt::tests::test_prompt_includes_balance ... ok
test sdk::prompt::tests::test_prompt_surfaces_team_board ... ok
test sdk::actor::tests::test_lineage_score_increases_with_depth ... ok
test sdk::prompt::tests::test_prompt_surfaces_search_hits ... ok
test sdk::prompt_guard::tests::test_case_insensitive_match - should panic ... ok
test sdk::prompt_guard::tests::test_clean_prompt_passes ... ok
test sdk::prompt_guard::tests::test_empty_prompt_passes ... ok
test sdk::prompt_guard::tests::test_h_vpput_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_assignment_pattern_caught - should panic ... ok
test sdk::prompt::tests::test_prompt_truncates_errors_to_3 ... ok
test sdk::prompt_guard::tests::test_pput_runtime_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_m_verified_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_verified_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_substring_in_larger_text - should panic ... ok
test sdk::prompt_guard::tests::test_wbcg_caught - should panic ... ok
test sdk::protocol::tests::test_malformed_action_tag_rejected_not_fallback ... ok
test sdk::protocol::tests::test_deduct_negative_amount_rejected ... ok
test sdk::protocol::tests::test_no_byte_repair_on_invalid_escape ... ok
test sdk::protocol::tests::test_parse_action_tag_with_think_block ... ok
test sdk::protocol::tests::test_parse_bare_json_fallback ... ok
test sdk::protocol::tests::test_parse_no_action_returns_error ... ok
test sdk::protocol::tests::test_parse_invalid_json_returns_error ... ok
test sdk::protocol::tests::test_parse_with_invest_action ... ok
test sdk::protocol::tests::test_strip_multiple_think_blocks ... ok
test sdk::protocol::tests::test_strip_unclosed_think_block ... ok
test sdk::protocol::tests::test_strip_think_blocks ... ok
test sdk::protocol::tests::test_parse_action_tag_valid ... ok
test sdk::sandbox::tests::test_sandbox_captures_stderr ... ok
test sdk::sandbox::tests::test_sandbox_echo_command ... ok
test sdk::snapshot::tests::test_snapshot_balance_query ... ok
test sdk::tools::librarian::tests::test_board_post_append ... ok
test sdk::tools::librarian::tests::test_board_write_read_roundtrip ... ok
test sdk::tools::librarian::tests::test_build_compression_prompt ... ok
test sdk::tools::librarian::tests::test_compress_interval ... ok
test sdk::tools::librarian::tests::test_zero_interval_never_compresses ... ok
test sdk::tools::search::tests::test_sanitize_query ... ok
test sdk::tools::search::tests::test_search_empty_query ... ok
test sdk::tools::search::tests::test_search_nonexistent_path ... ok
test sdk::sandbox::tests::test_sandbox_nonzero_exit ... ok
test sdk::tools::wallet::tests::test_append_is_free ... ok
test sdk::tools::wallet::tests::test_genesis_allocation ... ok
test sdk::tools::wallet::tests::test_deduct_and_credit ... ok
test sdk::tools::wallet::tests::test_negative_deduct_rejected ... ok
test sdk::tools::wallet::tests::test_insufficient_balance_rejected ... ok
test sdk::tools::wallet::tests::test_no_double_genesis ... ok
test sdk::tools::wallet::tests::test_portfolio_tracking ... ok
test sdk::tools::wallet::tests::test_query_balance ... ok
test sdk::tools::wallet::tests::test_query_unknown_key ... ok
test sdk::tools::wallet::tests::test_unknown_agent_vetoed ... ok
test sdk::tools::wallet::tests::test_zero_deduct_rejected ... ok
test wal::tests::test_wal_replay_missing_file_is_empty ... ok
test wal::tests::test_wal_roundtrip_nodes_only ... ok
test boot::tests::verify_trust_root_passes_on_intact_repo ... ok
test wal::tests::test_wal_skip_malformed_line ... ok
test wal::tests::test_wal_roundtrip_mixed ... ok
test sdk::sandbox::tests::test_sandbox_timeout_kills_process ... ok

test result: ok. 130 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.07s

     Running unittests src/main.rs (target/debug/deps/turingosv4-ad648582b935fecb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/fc_alignment_conformance.rs (target/debug/deps/fc_alignment_conformance-239d53a59894a473)

running 26 tests
test fc1_n11_predicate_trait_register_api ... ignored, 🔨 Stage 3 unmerged — bus.register_predicate API + Predicate trait live on phase-z-wtool-tools branch only; not on main. Production path uses inline forbidden_patterns check in append_internal as the ∏p surface.
test fc1_n12_lean4_oracle_ground_truth_predicate ... ignored, Cross-crate — Lean4Oracle in minif2f_v4 sub-crate; covered in experiments/minif2f_v4/tests/fc_alignment_conformance.rs (separate file, separate atom)
test fc1_n1_q_state_carrier_constructible_with_default_config ... ok
test fc1_n11_n15_e18_pi_p_zero_preserves_q_t_via_forbidden_pattern ... ok
test fc1_n13_wtool_bus_append_present ... ok
test fc1_n7_delta_ai_client_constructible ... ok
test fc1_n4_tape_constructible_with_time_arrow ... ok
test fc2_n16_init_ai_orchestrator_swarm_oneshot ... ignored, Binary-only — run_swarm/run_oneshot are in evaluator binary, not lib; refactor needed to expose for direct integration testing
test fc1_n8_n9_n10_output_agent_output_parseable ... ok
test fc2_n20_n27_tick_mr_present ... ok
test fc2_n22_halt_via_halt_and_settle ... ok
test fc1_n6_input_universe_snapshot_via_bus ... ok
test fc2_n23_haltreason_full_taxonomy_typed ... ignored, 📅 Not yet typed as Rust enum — only OmegaAccepted exists; other 4 variants {MaxTxExhausted, WallClockCap, ComputeCapViolated, ErrorHalt} per CLAUDE.md report standard live as jsonl strings in extra map. Type promotion is Phase C+ work.
test fc2_n23_event_type_omega_accepted_canonical ... ok
test fc3_e15_e16_e17_constitutional_signaling ... ignored, 📅 Phase 11+ — automated runtime veto/abide signaling not implemented. Today: manual policy via CLAUDE.md Audit Standard
test fc3_e14_boot_panic_immediate_abort_documented ... ok
test fc3_n32_veto_ai_runtime ... ignored, 📅 Phase 11+ — Veto-AI runtime not implemented (manual Codex/Gemini dual-audit covers role today; Art. V.1.3 amendment 2026-04-25 narrowed scope to {PASS, VETO})
test fc3_n33_architect_ai_runtime ... ignored, 📅 Phase 11+ — ArchitectAI runtime not implemented (manual Claude code editing covers role today; Phase D will deliver. Art. V.1.2 amendment grants commit authority post-Veto-AI PASS)
test fc3_n34_parse_trust_root_section_helper ... ok
test fc3_n34_trust_root_error_taxonomy_present ... ok
test fc3_n31_logs_archive_wal_open_in_tempdir ... ok
test fc3_n40_logs_to_architect_feedback ... ignored, 📅 Phase 11+ — automated logs → ArchitectAI feedback loop not implemented. Phase D consumer reads jsonl + WAL + stderr (per THESIS_V2_GROUND_TRUTH_AUDIT findings C+D)
test fc3_n41_in_process_reinit_loop ... ignored, 📅 Phase 11+ — in-process re-init not implemented (external batch runner retry covers today). FC3-E14 immediate-abort leaf is what we have.
test fc3_n39_log_ledger_present_and_appendable ... ok
test fc3_s3_readonly_subgraph_manifest_size ... ok
test fc3_n34_readonly_guard_verify_trust_root_intact_repo ... ok

test result: ok. 17 passed; 0 failed; 9 ignored; 0 measured; 0 filtered out; finished in 0.07s

     Running tests/reward_pull_conservation.rs (target/debug/deps/reward_pull_conservation-cc5a4dd71482e4d5)

running 5 tests
test phase2_conservation_total_coins_bounded ... ok
test phase2_founder_grant_credits_yes_on_append ... ok
test phase2_no_grant_when_flag_off ... ok
test phase2_settle_pays_out_on_golden_path ... ok
test phase2_settle_zero_on_losing_side ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/wal_resume.rs (target/debug/deps/wal_resume-a4181e084fd48f51)

running 2 tests
test wal_empty_file_yields_fresh_bus ... ok
test wal_persists_appends_across_bus_drop ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

   Doc-tests minif2f_v4

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests turingosv4

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


codex
All three executable gates passed. The Rust suite is now 267 passed / 29 ignored / 0 failed, not the packet’s 265-pass expectation, which reinforces that the source has advanced past the packet snapshot. Next I’m checking whether any of that drift is substantive enough to block Phase B rather than just requiring packet/history/header repair.
exec
/bin/bash -lc "git show -s --format='%h%n%B' 6be6eb4 180a300 7f4bc0c a5c78e4 30f2a14 89994c7 90953d6" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
6be6eb4
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

180a300
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

7f4bc0c
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

a5c78e4
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

30f2a14
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

89994c7
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

90953d6
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


exec
/bin/bash -lc 'for c in 6be6eb4 180a300 7f4bc0c a5c78e4 30f2a14 89994c7 90953d6; do echo $c; git diff-tree --no-commit-id --name-only -r $c | sed '"'s/"'^/  /'"'; done" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
6be6eb4
  experiments/minif2f_v4/tests/trust_root_immutability.rs
  genesis_payload.toml
  handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md
180a300
  experiments/minif2f_v4/src/bin/evaluator.rs
  genesis_payload.toml
7f4bc0c
  experiments/minif2f_v4/src/agent_models.rs
  experiments/minif2f_v4/src/bin/evaluator.rs
  experiments/minif2f_v4/src/lib.rs
  experiments/minif2f_v4/tests/trust_root_immutability.rs
  genesis_payload.toml
  handover/alignment/TRACE_MATRIX_v2_2026-04-25.md
a5c78e4
  experiments/minif2f_v4/src/bin/evaluator.rs
  experiments/minif2f_v4/src/jsonl_schema.rs
  genesis_payload.toml
  handover/alignment/TRACE_MATRIX_v2_2026-04-25.md
30f2a14
  experiments/minif2f_v4/src/bin/evaluator.rs
  experiments/minif2f_v4/src/budget_regime.rs
  experiments/minif2f_v4/src/jsonl_schema.rs
  experiments/minif2f_v4/src/lib.rs
  experiments/minif2f_v4/tests/trust_root_immutability.rs
  genesis_payload.toml
  handover/alignment/TRACE_MATRIX_v2_2026-04-25.md
  rules/enforcement.log
89994c7
  experiments/minif2f_v4/examples/fc_trace_emit_one.rs
  experiments/minif2f_v4/src/bin/evaluator.rs
  experiments/minif2f_v4/src/fc_trace.rs
  experiments/minif2f_v4/src/lib.rs
  experiments/minif2f_v4/tests/fc_trace_smoke.rs
  experiments/minif2f_v4/tests/trust_root_immutability.rs
  genesis_payload.toml
  handover/alignment/TRACE_MATRIX_v2_2026-04-25.md
  rules/enforcement.log
90953d6
  docs/experiments.md
  experiments/minif2f_v4/tests/trust_root_immutability.rs
  genesis_payload.toml
  handover/alignment/TRACE_MATRIX_v2_2026-04-25.md
  scripts/_smoke_siliconflow.py
  scripts/smoke_siliconflow.sh
  src/drivers/llm_proxy.py

exec
/bin/bash -lc "nl -ba handover/audits/A8_EXIT_PACKET_2026-04-26.md | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# Phase A → B Exit Audit Packet (A8) — current-state snapshot
     2	
     3	**Arc**: PPUT-CCL (`PREREG_PPUT_CCL_2026-04-26.md` round-4 PASS/PASS + `PREREG_AMENDMENT_p0_defer_2026-04-25.md`).
     4	**Date**: 2026-04-26.
     5	**Authority**: ArchitectAI commit (Art. V.1.2). Input to dual external audit (Codex + Gemini) per Art. V.1.3 + memory `feedback_dual_audit`.
     6	
     7	**Decision rule**: PASS → Phase B (kernel instrumentation + PPUT accounting) authorized; CHALLENGE → in-cycle fixes; VETO → Phase A redesign.
     8	
     9	**FC-trace**: meta-witness across FC1 / FC2 / FC3 (atoms instrument all three subgraphs).
    10	
    11	**Document split**: this packet is the **stable current-state snapshot** of Phase A at exit — it describes WHAT IS, not how it got here. The chronological history of audit rounds + their in-cycle fix bundles lives in the **companion** document `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` (append-only). Past audit transcripts are at `handover/audits/{CODEX,GEMINI}_PHASE_A8_EXIT_AUDIT_2026-04-26{,_R2,_R3,_R4,_R5,_R6,_R7,...}.md`. Reviewers needing closure-of-prior-finding context read the history doc; reviewers verifying current-state correctness against Phase B prerequisites read THIS packet.
    12	
    13	---
    14	
    15	## § 1. Phase A scope and atom map
    16	
    17	Phase A = pre-flight (days 1–3 of the 30-day arc). Decomposed into 8 atoms (A0–A7); atom A8 is the dual external audit gate represented by this packet.
    18	
    19	- **A0** (a–e): harness modernization.
    20	- **A1**: PREREG amendment p_0 calibration deferral.
    21	- **A2**: P0a `swarm_N=1` mode + `parse_swarm_condition_n` unit tests.
    22	- **A3**: per-agent `AGENT_MODELS` env var (Phase B+C single-model invariant gate).
    23	- **A4**: decomposed metrics (`hit_max_tx` + `tactic_diversity` + `verifier_wait_ms`).
    24	- **A5**: per-agent budget normalization (`BUDGET_REGIME` + `MAX_TRANSACTIONS`).
    25	- **A6**: per-line FC tagging via structured JSON events (`fc_trace` module).
    26	- **A7**: SiliconFlow heterogeneous-LLM provider plumbing (proxy + 3-key smoke).
    27	- **A8**: this packet — Phase A → B exit audit.
    28	
    29	Atom commit chain (atomic, FC-traced, all under ArchitectAI commit authority — none touched `constitution.md`):
    30	
    31	```
    32	2e7f75a  A0a   d8950ee  A0b   2a65339  A0c   e94e1b9  A0d   62c4e14  A0e
    33	6be6eb4  A1    180a300  A2    7f4bc0c  A3    a5c78e4  A4    30f2a14  A5
    34	89994c7  A6    90953d6  A7    60292dc  A8 prep
    35	```
    36	
    37	(Subsequent audit-cycle in-cycle fix bundle commits are recorded in `A8_AUDIT_HISTORY_2026-04-26.md`.)
    38	
    39	## § 2. Current-state metrics
    40	
    41	| Metric | Value | Source / verification |
    42	|---|---|---|
    43	| `cargo test --workspace` PASS | **265** | re-runnable; all suites green |
    44	| `cargo test --workspace` ignored | 29 | Phase B+ deferred stubs |
    45	| `cargo test --workspace` failed | **0** | — |
    46	| `python3 scripts/test_llm_proxy.py` | **16/16 PASS** | proxy routing + round-robin conformance (also wrapped by `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` so it runs every `cargo test --workspace`) |
    47	| Trust Root manifest entries | **37** | `genesis_payload.toml [trust_root]` count + matches `trust_root_immutability::test_trust_root_manifest_includes_b2_b4_files` required-paths list |
    48	| `boot::tests::verify_trust_root_passes_on_intact_repo` | **PASS** | re-hashes match the manifest |
    49	| `bash scripts/smoke_siliconflow.sh` | **PASS (3/3 keys)** | live API; cost ~$0.005 per run |
    50	| FC-trace anchor sites in `evaluator.rs` | **9** | grep `fc_trace::emit_event(`; 8 in `run_swarm` + 1 in `run_oneshot` |
    51	| `make_pput` arity | **24 positional args** | refactor to builder pattern is a known Phase B+ deferred work item |
    52	
    53	## § 3. Per-atom acceptance evidence (current state)
    54	
    55	### A0 (harness modernization) — `62c4e14`
    56	- 4 governance rules (R-014 / R-015 / R-018 / R-019), constitution-special-case judge.sh hook (R-016 fc_trace_in_commit), `tests/fc_alignment_conformance.rs` 17-witness battery + 9 ignored stubs, 5 cases C-071..C-075 (constitution amendment process / Veto-AI scope / ArchitectAI commit authority / FC-first / DO-178C tool qualification).
    57	- Closed by dual-audit cycle; no open P0.
    58	
    59	### A1 (PREREG amendment) — `6be6eb4`
    60	- File: `handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md`.
    61	- Substitutes `p_0 = 0.10` (PREREG § 5.5 ceiling) for the calibration-derived value at every Gate H consumer.
    62	- **Statistical framing**: `0.10` is the **least-strict admissible value** the PREREG ceiling allows. `j-RR ≤ p_0` makes a SMALLER `p_0` stricter, so this substitution is operationally permitted but is NOT a tighter-than-original guarantee. No Type-I inflation since `j-RR` is descriptive (PREREG § 5.4), outside the inferential family. The substitution is operationally permitted at any phase including Phase E; if/when § 3 re-calibration conditions complete, calibration upgrades the bar.
    63	- **FC-trace**: FC1-N12 + Art. V.1.2 + cases C-073 + C-075.
    64	
    65	### A2 (`swarm_N=1` mode) — `180a300`
    66	- New `parse_swarm_condition_n` in `experiments/minif2f_v4/src/bin/evaluator.rs` discriminates `n<digits>` from `oneshot` / `hybrid_v1` / malformed. PREREG_AMENDMENT § 3 condition 2 cleared.
    67	- **FC-trace**: FC2-N16 + FC1-N11.
    68	- 5 unit tests.
    69	
    70	### A3 (`AGENT_MODELS` env var) — `7f4bc0c`
    71	- New `experiments/minif2f_v4/src/agent_models.rs`. Pure parser + expander + env-coupled resolver. Heterogeneity gated by `PHASE_D_HETERO_OK=1` — Phase B+C single-model invariant enforced at startup BEFORE any LLM call.
    72	- **FC-trace**: FC1-N7 (δ/AI canonical identity per Agent_i).
    73	- 11 unit tests.
    74	
    75	### A4 (decomposed metrics) — `a5c78e4`
    76	- 3 non-Optional v2 fields on `RunAggregate` + legacy `PputResult`: `hit_max_tx`, `tactic_diversity`, `verifier_wait_ms`. Helper `compute_tactic_diversity`.
    77	- **FC-trace**: FC2-N22 + FC1-N11 + FC1-N12.
    78	- 5 conformance tests.
    79	
    80	### A5 (budget regime) — `30f2a14`
    81	- New `experiments/minif2f_v4/src/budget_regime.rs`. 4-variant `BudgetRegime` enum: `total_proposal` (default; preserves the prior baseline) / `per_agent` (loop bound = base × N) / `token_total` (declared; startup-fatal `UnimplementedRegime`) / `wall_clock` (declared; startup-fatal). 2 new non-Optional v2 fields: `budget_regime` + `budget_max_transactions`.
    82	- `run_swarm` startup: env-resolved budget with startup-fatal error path on misconfiguration.
    83	- **FC-trace**: FC2-N22 + FC1-N7.
    84	- 16 unit tests; PREREG_AMENDMENT § 3 condition 3 cleared.
    85	
    86	### A6 (FC tracing) — `89994c7`
    87	- New `experiments/minif2f_v4/src/fc_trace.rs`. Pure stdlib (zero new deps). 7-variant `FcId` enum (FC1-N7 / FC1-N11 / FC1-N12 / FC1-E18 / FC2-N20 / FC2-N22 / FC3-N31). `FC_TRACE=1` gate cached in `OnceLock`; `FC_TRACE_FILE=<path>` redirects emit to file.
    88	- **9 wired anchor sites** (8 in `run_swarm` + 1 in `run_oneshot`):
    89	  - FC2-N22 synthetic short-circuit (swarm)
    90	  - FC2-N20 mr tick (swarm)
    91	  - FC2-N22 OMEGA full-proof accept (swarm)
    92	  - FC2-N22 OMEGA per-tactic accept (swarm)
    93	  - FC2-N22 natural MaxTxExhausted (swarm; carries `budget_regime` payload)
    94	  - FC1-N12 verify bracket (oneshot)
    95	  - FC1-N12 swarm `verify_omega_detailed` path "alone"
    96	  - FC1-N12 swarm `verify_omega_detailed` path "tape+payload"
    97	  - FC1-N12 swarm `verify_partial`
    98	- Per-run correlation: `experiments/minif2f_v4/src/run_id.rs::mint_run_id` mints one identifier per run, threaded into both `emit_event` and `make_pput` so FC events join v2 jsonl rows by equality.
    99	- **FC-trace**: meta-witness for the 5-step compile loop.
   100	- 7 tests on `fc_trace` (6 unit + 1 end-to-end smoke `tests/fc_trace_smoke.rs` exercising `FC_TRACE=1` in a child process). 3 unit tests on `run_id`.
   101	
   102	### A7 (SiliconFlow plumbing) — `90953d6`
   103	- `src/drivers/llm_proxy.py`: OpenAI-compatible local HTTP server with per-provider multi-key round-robin. `detect_provider()` routing matrix:
   104	  - explicit `provider:...` prefix → that provider (if known)
   105	  - any slash-form id (HuggingFace style: `Qwen/...`, `openai/...`, `meta-llama/...`, `deepseek-ai/...`) → `siliconflow`
   106	  - bare `"deepseek"` substring (no slash) → `deepseek` (api.deepseek.com)
   107	  - bare `qwen3-*` / `qwen-*` (no slash) → `dashscope`
   108	  - else → `dashscope` (default fallback)
   109	- 3 SiliconFlow keys (primary / secondary / tertiary) split concurrent traffic across separate rate-limit pools — V3L-27 / case C-027 single-key N=30 collapse mitigation.
   110	- `scripts/smoke_siliconflow.sh` + `_smoke_siliconflow.py`: 3 keys × 1 probe (Qwen2.5-7B-Instruct, max_tokens=8). Verified all 3 keys responding.
   111	- `scripts/test_llm_proxy.py`: 16-test routing + round-robin conformance suite (no live API). Wrapped by `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` so the suite runs on every `cargo test --workspace`. The wrapper FAILS CLOSED if `python3` is missing; explicit opt-out `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1` (logged loudly).
   112	- **FC-trace**: FC1-N7 (δ/AI provider expansion).
   113	- Memory: `reference_siliconflow.md` records SiliconFlow as the Phase D heterogeneous lane (NOT a probe-only target).
   114	
   115	## § 4. Phase B → C exit checklist (from PREREG_AMENDMENT § 4) — Phase A side
   116	
   117	The PREREG amendment shifted the Phase B → C gate. From the Phase A perspective, the items it lists are now satisfied:
   118	
   119	- ❌ p_0 calibration jsonl frozen (was REQUIRED) → **DEFERRED with substitution per amendment § 2**: `p_0 = 0.10` hardcoded at every Gate H consumer.
   120	- ✅ B1–B7 + B7-extra mode toggle infrastructure complete (pre-Phase A baseline; round-4 PASS/PASS).
   121	- ✅ Phase A0 harness modernization complete (`62c4e14`).
   122	- ✅ Tools qualified per case C-075 (DO-178C tool qualification): `runner.sh`, `compute_p0.py`, evaluator boot enforcement, etc.
   123	- ✅ Trust Root verifies clean (`boot::tests::verify_trust_root_passes_on_intact_repo` PASS at 37-entry manifest).
   124	
   125	## § 5. Risks and known limitations entering Phase B
   126	
   127	1. **`per_agent` budget regime untested at runtime**. A5 unit tests verify the scaling math (`base × N`) and env-coupled resolver. No live-LLM run with `BUDGET_REGIME=per_agent` has been smoked. Phase B kernel instrumentation will be the first opportunity to observe its behavior on a real problem.
   128	2. **FC-trace coverage still partial**. 9 wired anchor sites cover HALT decomposition (FC2-N22 × 4 exit paths) + mr tick (FC2-N20) + Lean oracle scope (FC1-N12 × 4 sites: oneshot + swarm `verify_omega_detailed` × 2 + swarm `verify_partial`). Still NOT emitting: FC1-N7 prompt-build, FC1-N11 ∏p decision diversity (per-proposal), FC1-E18 preserve-Q_t (per ∏p=0), FC3-N31 WAL append. The `FcId` enum reserves these variants. Phase B+ kernel instrumentation should fill them in as the Phase B emit boundary lands.
   129	3. **SiliconFlow rate-limit at scale**. A7 verified 3 keys responding individually at N=1 concurrency. V3L-27 demonstrates collapse at N=30 single-key. The v4 multi-key round-robin should triple the safe N envelope but the actual sweet spot for our hetero swarm is unmeasured. Phase D heterogeneous-batch design should land a `--max-concurrency` knob (currently `LLM_PROXY_CONCURRENCY=5` env in proxy) tuned per provider.
   130	4. **Heterogeneous swarm = Phase D, not B/C**. Per F-2026-04-25-02 + the `agent_models.rs` `PHASE_D_HETERO_GATE_ENV_VAR` invariant, Phases B and C MUST stay single-model so ablation axes are not confounded. A7's plumbing exists for future Phase D work; Phase B uses the existing `deepseek-v4-flash` thinking-off backbone unchanged.
   131	5. **`make_pput` arity = 24 (Phase B+ refactor)**. The positional-arg list is a maintainability hazard; refactor to builder pattern (`PputResultBuilder`) recommended early in Phase B. Non-blocking; runtime correctness verified by 4 in-binary tests + the integration battery.
   132	
   133	## § 6. Specific questions for auditors
   134	
   135	(All prior-round questions and their closures are recorded in `A8_AUDIT_HISTORY_2026-04-26.md` — see that doc's chronological round-N entries for the current count + verdicts. The questions below are for the current-round reviewer pass.)
   136	
   137	### Q1 — Code correctness spot-checks
   138	- (Q1.a) Does `experiments/minif2f_v4/src/bin/evaluator.rs` mint a single `run_id` at function entry and pass it to BOTH `fc_trace::emit_event` and `make_pput` so FC events and v2 jsonl rows share one identifier? Verify by reading `run_swarm` and `run_oneshot` entry points + `make_pput` signature.
   139	- (Q1.b) Does `src/drivers/llm_proxy.py::detect_provider` route `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B` to `siliconflow` (not `deepseek`)? Verify by reading the function + checking that `scripts/test_llm_proxy.py` includes `test_deepseek_slash_form_routes_to_siliconflow`.
   140	- (Q1.c) Does `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` fail closed when `python3` is missing? Verify by reading the test source (appended to this packet) + checking that the panic message describes the failure rather than a silent skip.
   141	
   142	### Q2 — PREREG amendment soundness
   143	- (Q2.a) `p_0 = 0.10` substitution wording: is `PREREG_AMENDMENT_p0_defer_2026-04-25.md` § 2 + § 8 internally consistent + statistically correct (least-strict admissible; no Type-I inflation; substitution operative at any phase including Phase E)?
   144	- (Q2.b) Re-calibration conditions in § 3 are PRE-REQUISITES for calibration to run, not guarantees that calibration completes before any specific phase. Is this framed correctly without claiming calibration MUST run before Phase E?
   145	- (Q2.c) The amendment's SHA-256 is in Trust Root. Does `boot::tests::verify_trust_root_passes_on_intact_repo` pass cleanly?
   146	
   147	### Q3 — Atomicity, FC-trace discipline, governance
   148	- (Q3.a) Each of A1–A7 was committed as one atomic commit with `FC-trace: <FC?-N?>` in the message. Verify by re-reading commit messages. Any commit missing FC-trace? Any commit that touched `constitution.md`?
   149	- (Q3.b) 5 cases C-071..C-075 sediment 2026-04-25 session decisions as constitutional precedent. Are the rulings standalone-readable + correctly cross-referenced?
   150	- (Q3.c) Trust Root manifest 37 entries. Are all entries load-bearing (i.e., does tampering each one weaken the constitutional gate)?
   151	
   152	### Q4 — Phase A → B exit decision
   153	- (Q4.a) Spot-check: re-run `cargo test --workspace`; expect 265 PASS / 29 ignored / 0 failed.
   154	- (Q4.b) Spot-check: re-run `python3 scripts/test_llm_proxy.py`; expect 16/16 PASS.
   155	- (Q4.c) Spot-check: re-run `bash scripts/smoke_siliconflow.sh`; expect PASS (3/3 keys; live API; cost ~$0.005).
   156	- (Q4.d) Are there any open P0 defects from any prior round? Cross-reference `A8_AUDIT_HISTORY_2026-04-26.md` to verify each in-cycle fix bundle's closures hold against current source.
   157	- (Q4.e) Phase B's first sub-atom is "JSONL schema v2 + C_i full-cost aggregator" (notepad). Are there any Phase A artifacts that would BLOCK that scope?
   158	
   159	### Q5 — Packet/history split
   160	- (Q5.a) Is the split between `A8_EXIT_PACKET` (stable current-state) and `A8_AUDIT_HISTORY` (append-only chronology) consistent with the project's existing pattern (constitution + Art. V.3 amendment log; PREREG + PREREG_AMENDMENT; TRACE_MATRIX_v0/v1/v2)?
   161	- (Q5.b) Is the history doc append-only — past round entries describe what was true at that round's snapshot, without retroactive edits?
   162	- (Q5.c) Does this packet contain ANY round-N retrospective text, fix-shipped headers, "previous round caught X" claims, or historical lineage like "(post-A8e F4)" / "(added by A8eN)" / "(chain position N via A8e)" — anywhere? It should NOT — those belong only in the history doc.
   163	
   164	---
   165	
   166	End your audit with:
   167	- **VERDICT**: PASS / CHALLENGE / VETO
   168	- **Conviction**: low / medium / high
   169	- **Recommendation**: PROCEED to Phase B (kernel instrumentation + PPUT accounting) / FIX-THEN-PROCEED / REDESIGN
   170	
   171	PASS = current-state evidence passes Phase B prerequisites + no open P0 from prior rounds + no new substantive findings. CHALLENGE = correctable in one cycle. VETO = Phase A redesign required.

exec
/bin/bash -lc "nl -ba genesis_payload.toml | sed -n '55,140p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    55	#     .claude/hooks/judge.sh (PreToolUse hook that invokes engine.py +
    56	#       implements R-016 fc_trace_in_commit + constitution.md special-case)
    57	#     tests/fc_alignment_conformance.rs (per CLAUDE.md "每个 ✅ 行 ≥1
    58	#       witness test"; tampering = silent constitutional drift, defeats
    59	#       FC1/2/3 ↔ symbol mapping enforcement)
    60	#
    61	#   2026-04-25 Phase A1 PREREG amendment (1):
    62	#     handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md
    63	#       (defers PREREG § 5.5 calibration; substitutes p_0 = 0.10
    64	#       max-tolerated ceiling — least-strict admissible per amendment
    65	#       § 2 wording correction A8e fix F6; per case C-073 ArchitectAI
    66	#       commit authority)
    67	#
    68	# Total: 37 files. Progression A0=24 → A1=25 (PREREG amendment) → A3=26
    69	# (agent_models.rs) → A5=27 (budget_regime.rs) → A6=28 (fc_trace.rs) →
    70	# A7=31 (llm_proxy.py + 2 smoke scripts) → A8e=33 (run_id.rs +
    71	# test_llm_proxy.py) → A8e2=34 (llm_proxy_python_conformance.rs Rust
    72	# wrapper) → A8e7=35 (A8_AUDIT_HISTORY append-only chronology) →
    73	# A8e11=37 (audit runner scripts run_codex_phase_a8_exit_audit.sh +
    74	# run_gemini_phase_a8_exit_audit.py — per C-075 gate-machinery
    75	# tooling that produces audit transcripts must be Trust Root, both
    76	# round-8 N3 false-closure and round-9 N3-followup defects
    77	# demonstrated these are load-bearing gate tools).
    78	# genesis_payload.toml itself is conceptually frozen but
    79	# not self-hashed (chicken-and-egg) — the [pput_accounting_0] section
    80	# values are the semantic anchor.
    81	#
    82	# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
    83	
    84	[pput_accounting_0]
    85	schema_version = "1.0"
    86	progress_definition = "1 iff Lean ground-truth verifies golden_path_payload"
    87	cost_definition = "sum(prompt_tokens + completion_tokens + tool_tokens) over all proposals in the run"
    88	time_definition = "wall_clock from first agent prompt construction to final Lean accept or external timeout"
    89	verified_predicate = "experiments/minif2f_v4/src/lean4_oracle.rs::verify_omega_detailed"
    90	heldout_sealed_hash = "51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b"
    91	source_pool_sha256 = "77179cf2598b0df707d78a6663e763121dfe8e73a6538073155f488feab95093"
    92	baseline_regression_rate = 0.0  # PLACEHOLDER until Phase B7-extra calibration
    93	baseline_regression_jsonl_sha256 = ""  # PLACEHOLDER until Phase B7-extra
    94	k_max = 10
    95	n_max = 34
    96	
    97	[trust_root]
    98	"src/main.rs" = "622fee2d96a980d24f9fbaab3d0531c195a0a337fc3ddd2efb60bca90a1cfbf9"
    99	"Cargo.lock" = "577446e8fe11e91bc8751bf13e5ddca6c5faa64d3309b878768c550d3e6feb98"
   100	"handover/preregistration/scripts/run_p0_calibration.sh" = "5f4a57dd8b8280ffe04bec89350a57d876d06cc179d9f8841a522e7bdcf1b8b7"
   101	"handover/preregistration/scripts/compute_p0.py" = "fd1dff97d582f4f509249852be500cba7d81efc9bab8fcb9872b2d85bb4e3052"
   102	"rules/MANIFEST.sha256" = "6d6fd54d46352213a857fdaf5cf70e8a893bbbd5467e5da45a3ed3314d01b0c2"
   103	"rules/engine.py" = "932d9a2b7a3249a7eb5825c0b5c714a9913cd9aa9e058f789e64992b140e40b3"
   104	".claude/hooks/judge.sh" = "f1f8f5094057dcea3c345951f7e73fd4d269f074a407829f19e4d75830a55fc2"
   105	"tests/fc_alignment_conformance.rs" = "b3e75979ad2d175b9c45135be6ea1d94ce95184c6896468330c12dbfc1f719db"
   106	"src/kernel.rs" = "893fd67534caf7a3d9abd6efbd202556348b6491cd6d4c6bdb224d2ad75b1af0"
   107	"src/wal.rs" = "1ac7637aa09531b1c7232163f5df48f7193251594c4ed20e0d0fc85cea8f84dc"
   108	"src/boot.rs" = "57a0609c4e4e70322419ebe31abe92432f80f2d3298aab29e90d45e4d91942fc"
   109	"src/bus.rs" = "df28ffe514a3272a3d10fca4568fd424a76e754e9785c109a5459f163f7fd14c"
   110	"src/drivers/llm_http.rs" = "615596b68956b4a8925110edc99aa746a5543527724ec394bb9dda163147ed7a"
   111	"src/drivers/llm_proxy.py" = "22f632234a6d3e1a0547651537cf26846c64c47b85f5b1c3a1bcd1ae1f139997"
   112	"scripts/smoke_siliconflow.sh" = "6ad54e7c0ab8221f475360dcad80eeeb0d6da0fd55c27acdb1cefb2b390f5341"
   113	"scripts/_smoke_siliconflow.py" = "858d3b219c7e2d01cc370beb000e107b0abb8a211a63b6bf09d072afd02aeb24"
   114	"scripts/test_llm_proxy.py" = "e1dd0306609fb3d2578b078bb21f0090c23978981f674ca0c6e3a60485ebf273"
   115	"src/sdk/prompt_guard.rs" = "b4f7b164770d1a7203b8143f773c66f748994d60a42ece38471f2f7f2839f4f1"
   116	"experiments/minif2f_v4/src/lean4_oracle.rs" = "70fae24cd17f410c10a092e797fcdedea962db3d7cb20f218d02303edae9e98c"
   117	"experiments/minif2f_v4/src/cost_aggregator.rs" = "896b6905dbca9e9736f8896cd5725c16b6e87c6ad3ff822e044975febed46a03"
   118	"experiments/minif2f_v4/src/wall_clock.rs" = "2c9197f8f93b7d130dc7b094a6664f8ece351ec85668921c5beb6d100a1a77ee"
   119	"experiments/minif2f_v4/src/post_hoc_verifier.rs" = "9a93ae5548827b60543df779c67a5f1201b49a681a55e2cbc6a8532e8bb131a2"
   120	"experiments/minif2f_v4/src/jsonl_schema.rs" = "5d0c5a2bcceba42abbc6f5cd58e045e2d5d7a9edc565db4515f3917a21dc520b"
   121	"experiments/minif2f_v4/src/rollback_sim.rs" = "9ccdf177813452bb83221bd780f2f7f10da619117aca8e44d3ecb355505fe561"
   122	"experiments/minif2f_v4/src/agent_models.rs" = "b736b91efb8a93d884b64028da79d5d6af0525e8c7e8f7222055118f43dc26ce"
   123	"experiments/minif2f_v4/src/budget_regime.rs" = "e0c2ba6533444e6178d9e12d0f182d29c3203a929ea0914d3098efd0397948ea"
   124	"experiments/minif2f_v4/src/fc_trace.rs" = "8578edf61148e50abdf8d29a2efdc5114ee20ab727c1ccd6c5a88ad07e7ffa09"
   125	"experiments/minif2f_v4/src/run_id.rs" = "5ff9db039131eaf4efeb65303d886265ef4c4f7e0e0f743467a7cbf22061781c"
   126	"experiments/minif2f_v4/src/bin/evaluator.rs" = "537af991c9168c40ac3d866b164a274b4a587b098bb0fc471238a0ef37a37c6d"
   127	"constitution.md" = "91e5913603eebaf134ac93d0f9ed7bb2b7916132203f6c2fae84d73329056ad6"
   128	"cases/MANIFEST.sha256" = "eccbe35f776c64225d50c461780d1fe75a0a8510adc62b0dd2e96b46dc2dc555"
   129	"handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json" = "69be702e3e0a4bb33c62bd02056495309289fd9271972dc733b4a1ba3dd908e1"
   130	"handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" = "64d565b4451d1f770425530bb7ffa874da602ddfd9d714ce7f7db6bbfab69463"
   131	"handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md" = "e1a0c97921fd77bd5a1268589bafbdc90e18e26a4feb6c86c6d8a91048d095e1"
   132	"experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs" = "d62ee67cb564355fa26959c1f92a1152f95ab55d987c005abf7810e2da7b83b8"
   133	"handover/audits/A8_AUDIT_HISTORY_2026-04-26.md" = "c39230ad1c910380e79024a5d1dbfe6b17758b150d7505320447a93c1c5e9837"
   134	"handover/audits/run_codex_phase_a8_exit_audit.sh" = "a4555bd097eedc611a10692b91c26bfa4c0a9e76ef3087dba88f21bf5b2c7742"
   135	"handover/audits/run_gemini_phase_a8_exit_audit.py" = "529a1b7ea8c870bec8083d0177854c41a8cc1c2c64836027bad9c71f55b441cc"

exec
/bin/bash -lc "nl -ba handover/audits/A8_AUDIT_HISTORY_2026-04-26.md | sed -n '300,355p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   300	- Trust Root: 37-entry manifest
   301	
   302	**Verdicts**:
   303	- Codex R11: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R11.md`
   304	- Gemini R11: **CHALLENGE / high** — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R11.md`. *First time both auditors converged on multiple substantive findings since R7.*
   305	- Merged: **CHALLENGE**. Two substantive findings (one architectural, one refactor) plus four documentary.
   306	
   307	**Findings** (4 Codex + 3 Gemini):
   308	1. (**Codex R11#1 — substantive architectural**) Trust Root proxy via `cases/MANIFEST.sha256` + `rules/MANIFEST.sha256` is documented as covering the child yaml files but not actually enforced at boot. `verify_trust_root` only hashes the flat `[trust_root]` entries; the manifest's child contents are not verified. A direct edit to a case or rule yaml without regenerating its parent manifest leaves boot still passing — silent governance drift.
   309	2. (Codex R11#2) `A8_AUDIT_HISTORY` had not been updated to record A8e12 + round-11 entries; per the A8e7 split discipline these belong in the chronology doc.
   310	3. (Codex R11#3) `TRACE_MATRIX_v2` § 5 still listed R-020 as `ground_truth_label` (the original A0a planning name; A8e12 took the slot for `commit_claim_diff_parity`); § 6 still said "5 cases C-071..C-075" (now 6 with C-076) and "14 rules" (now 15 with R-020).
   311	4. (Codex R11#4) `A8_AUDIT_HISTORY` cumulative metrics paragraph had a contradiction: claimed "11 substantive findings" but the enumerated list ended with "= 9 real bugs".
   312	5. (Gemini R11#1) `TRACE_MATRIX_v2` § 1 contains audit-fix-bundle retrospectives (A8e..A8e11 entries) that — per Gemini's reading of A8e7 — should live exclusively in `A8_AUDIT_HISTORY`.
   313	6. (Gemini R11#2) Same critique applied to `genesis_payload.toml` header progression chain ("A0=24 → A1=25 → ...").
   314	7. (**Gemini R11#3 — substantive refactor**) `make_pput` function signature uses `Option<u64>` for `total_run_token_count` / `failed_branch_count` / `total_wall_time_ms`, but every caller passes `Some(...)` and the body unwraps with `.unwrap_or(0)`. The mid-term P0-B fix collapsed the v2 struct fields to non-Option but didn't follow through to the function-parameter contract. Code clarity issue, not a runtime bug.
   315	
   316	**Round-11 fixes shipped (`A8e13`, this commit)** — six fixes (2 substantive + 4 documentary):
   317	- **Q1** (Codex R11#1 — substantive arch): `src/boot.rs` extended with `verify_child_manifest`. When a Trust Root entry path ends in `/MANIFEST.sha256`, the verifier now ALSO parses the manifest contents (GNU sha256sum format) and verifies each child file's actual hash against the claim. Manifests regenerated with repo-relative paths (was a mix of bare-filename and dir-relative). Two new unit tests pin the contract (`verify_trust_root_detects_child_manifest_tamper` + `verify_trust_root_passes_with_matching_child_manifest`). Plus: `src/boot.rs` itself added to Trust Root (meta-finding caught during fix — the verifier file was not previously protected by the verifier; tampering with boot.rs would silently bypass the entire gate). Manifest 37 → 38.
   318	- **Q2** (Gemini R11#3 — substantive refactor): `make_pput` signature `Option<u64>` → plain `u64` for the 3 affected parameters. All 9 production call sites + 4 test fixtures updated. `wc.elapsed_ms()` returns `Option<u64>` upstream; call sites add `.unwrap_or(0)` at the boundary. `cargo test --workspace`: 267 PASS (was 265 + 2 new boot tests).
   319	- **Q3** (Codex R11#2): `A8_AUDIT_HISTORY` extended with A8e12 entry + Round 11 outcome + A8e13 fixes shipped section + cumulative table rows for round 11/12.
   320	- **Q4** (Codex R11#3 + Gemini R11#1+#2 partial): `TRACE_MATRIX_v2` § 1 audit-fix-bundle entries left as-is (the project's TRACE_MATRIX has always been a delta-log; trying to remove these introduces churn without solving the underlying tension). Added a footer note explaining the doc's append-only-delta-log nature with cross-ref to `A8_AUDIT_HISTORY` for round-by-round attribution. `genesis_payload.toml` header progression chain similarly kept (it documents the manifest's growth lineage which is referenced by the rules-MANIFEST proxy comment) with a cross-ref pointer added.
   321	- **Q5** (Codex R11#3 specific items): `TRACE_MATRIX_v2` § 5 item 5 reworded — R-020 ground_truth_label slot was reassigned to `commit_claim_diff_parity` in A8e12; future ground-truth-label rule will be re-numbered. § 6 "5 cases" → "6 cases (C-071..C-076)"; "4 active rules" → "5 active (R-014/R-015/R-018/R-019/R-020)".
   322	- **Q6** (Codex R11#4): cumulative metrics paragraph reworded — "11 substantive findings" enumeration extended to 13 (was missing R10's P1+P2 = 2 findings; now lists all explicitly).
   323	
   324	---
   325	
   326	## Cumulative metrics
   327	
   328	| Round | Codex | Gemini | Merged | New findings | Real-bug findings | API cost (~$) |
   329	|---|---|---|---|---|---|---|
   330	| 1 | CHALLENGE | **VETO** | VETO | 9 | 5 | ~5 |
   331	| 2 | CHALLENGE | CHALLENGE | CHALLENGE | 5 | 0 | ~5 |
   332	| 3 | CHALLENGE | CHALLENGE | CHALLENGE | 5 (incl. 1 non-blocking) | 1 (H6 fail-closed) | ~5 |
   333	| 4 | CHALLENGE | PASS | CHALLENGE | 4 (incl. 1 non-blocking) | 0 | ~5 |
   334	| 5 | CHALLENGE | PASS | CHALLENGE | 3 (incl. 1 non-blocking) | 0 | ~5 |
   335	| 6 | CHALLENGE | CHALLENGE | CHALLENGE | 5 | **1 (K2 routing)** | ~5 |
   336	| 7 | CHALLENGE | CHALLENGE | CHALLENGE | 6 | **1 (M4 PREREG § 2 logic)** | ~5 |
   337	| 8 | CHALLENGE | **PASS** | CHALLENGE | 3 | **1 (N1 PREREG § 8 logic)** | ~5 |
   338	| 9 | CHALLENGE | **PASS** | CHALLENGE | 2 | 0 (false-closure caught — N3 was incomplete; no new substantive bugs) | ~5 |
   339	| 10 | CHALLENGE | **PASS** | CHALLENGE | 2 | **2 (P1 smoke false-PASS + P2 runners not in TR)** | ~5 |
   340	| 11 | CHALLENGE | CHALLENGE | CHALLENGE | 7 | **2 (Q1 boot.rs proxy not enforced + Q2 make_pput Option divergence)** | ~5 |
   341	| 12 | pending | pending | pending | — | — | ~5 |
   342	
   343	Cumulative cost so far ~$70 (12 rounds × ~$5–7 — note A8e12 was a no-audit harness amplifier between R10 and R11); within memory `feedback_dual_audit` Phase A reservation. **Real-bug yield: 13 substantive findings caught + closed across 11 rounds**: 5 routing/correctness in R1 (run_id ms drift / FC1-N12 swarm gap / Qwen-HF misroute / TR count off-by-1 / "strictest" wording reversed) + 1 fail-closed-gate in R3 (H6 wrapper soft-skip) + 1 routing collision in R6 (deepseek-ai/* misroute) + 1 PREREG § 2 logic in R7 (M4) + 1 PREREG § 8 logic in R8 (N1 — parallel-text miss) + 2 in R10 (P1 smoke false-PASS / P2 runners not in TR) + 2 in R11 (Q1 boot.rs proxy not enforced / Q2 make_pput Option<...> divergence). Plus 1 false-closure caught at R9 (N3 claimed runner default fix that wasn't shipped) — counted separately as a delivery-quality finding, not a system bug. The recurring documentary CHALLENGE class persisted longer than expected because each round's fix touched documentation in ways that left adjacent staleness; the A8e7 structural rewrite addressed the root cause (category error) but its implementation needed two more cycles (A8e8 + A8e9) to fully complete the lineage strip + cross-section consistency. **A8e12** added case C-076 + rule R-020 (commit-claim diff parity) as a Living Harness amplifier specifically targeting the false-closure / parallel-miss class — pre-commit WARN reminds the committer to grep-verify every claim against the actual diff.

exec
/bin/bash -lc "nl -ba src/boot.rs | sed -n '80,135p;300,360p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    80	        let bytes = fs::read(&full).map_err(|err| TrustRootError::FileRead {
    81	            path: full.clone(),
    82	            err,
    83	        })?;
    84	        let actual = hex_lower(&Sha256::digest(&bytes));
    85	        if actual != *expected {
    86	            return Err(TrustRootError::Tampered {
    87	                path: full,
    88	                expected: expected.clone(),
    89	                actual,
    90	            });
    91	        }
    92	        // Recurse into MANIFEST.sha256 children: the parent file's hash
    93	        // alone doesn't bind the children it claims (proxy was convention,
    94	        // not enforcement, before A8e13 fix Q1).
    95	        if rel_path.ends_with("/MANIFEST.sha256") {
    96	            verify_child_manifest(repo_root, &bytes)?;
    97	        }
    98	    }
    99	    Ok(())
   100	}
   101	
   102	/// TRACE_MATRIX FC3-N34 + case C-075: child-manifest recursion.
   103	/// Format = GNU `sha256sum` (`<64-hex>  <repo-relative-path>`).
   104	/// Paths resolve from `repo_root` (manifests are regenerated from
   105	/// the repo root, not from each manifest's parent dir).
   106	fn verify_child_manifest(repo_root: &Path, bytes: &[u8]) -> Result<(), TrustRootError> {
   107	    let text = std::str::from_utf8(bytes)
   108	        .map_err(|e| TrustRootError::GenesisParse(format!("manifest not utf-8: {e}")))?;
   109	    for (i, raw) in text.lines().enumerate() {
   110	        let line = raw.trim();
   111	        if line.is_empty() || line.starts_with('#') {
   112	            continue;
   113	        }
   114	        let (hex, child_rel) = line.split_once("  ").ok_or_else(|| {
   115	            TrustRootError::GenesisParse(format!("manifest line {}: {line:?}", i + 1))
   116	        })?;
   117	        if hex.len() != 64 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
   118	            return Err(TrustRootError::GenesisParse(format!(
   119	                "manifest line {}: bad hex {hex:?}", i + 1
   120	            )));
   121	        }
   122	        let path = repo_root.join(child_rel);
   123	        let actual = hex_lower(&Sha256::digest(
   124	            &fs::read(&path).map_err(|err| TrustRootError::FileRead { path: path.clone(), err })?,
   125	        ));
   126	        if actual != hex.to_ascii_lowercase() {
   127	            return Err(TrustRootError::Tampered {
   128	                path,
   129	                expected: hex.to_ascii_lowercase(),
   130	                actual,
   131	            });
   132	        }
   133	    }
   134	    Ok(())
   135	}
   300	    }
   301	
   302	    fn tempdir() -> PathBuf {
   303	        // Minimal tempdir without adding a `tempfile` dep.
   304	        let pid = std::process::id();
   305	        let nano = std::time::SystemTime::now()
   306	            .duration_since(std::time::UNIX_EPOCH)
   307	            .unwrap()
   308	            .as_nanos();
   309	        let dir = std::env::temp_dir().join(format!("turingosv4-boot-test-{pid}-{nano}"));
   310	        fs::create_dir_all(&dir).unwrap();
   311	        dir
   312	    }
   313	
   314	    /// A8e13 fix Q1 conformance: child manifest tamper is detected even
   315	    /// when the parent manifest hash itself is unchanged. This is the
   316	    /// scenario Codex R11#1 surfaced — pre-Q1, an attacker (or careless
   317	    /// developer) could edit a `cases/*.yaml` without regenerating
   318	    /// `cases/MANIFEST.sha256`, and boot would still pass because boot
   319	    /// only checked the manifest file's own hash, not its child entries.
   320	    #[test]
   321	    fn verify_trust_root_detects_child_manifest_tamper() {
   322	        let tmp = tempdir();
   323	        // Lay out a fake repo with a child manifest pointing at one
   324	        // child file. The PARENT manifest is hashed correctly into the
   325	        // [trust_root] section, but the CHILD file's actual content is
   326	        // tampered relative to what the parent manifest claims.
   327	        // Manifest paths are repo-relative per the project convention.
   328	        fs::create_dir_all(tmp.join("subdir")).unwrap();
   329	        fs::write(tmp.join("subdir/child.txt"), "tampered_content").unwrap();
   330	        let parent_text = format!("{}  subdir/child.txt\n", "0".repeat(64));
   331	        fs::write(tmp.join("subdir/MANIFEST.sha256"), &parent_text).unwrap();
   332	        let parent_hash = hex_lower(&Sha256::digest(parent_text.as_bytes()));
   333	        let genesis = format!(
   334	            "[pput_accounting_0]\nschema_version = \"1.0\"\n\n\
   335	             [trust_root]\n\"subdir/MANIFEST.sha256\" = \"{parent_hash}\"\n"
   336	        );
   337	        fs::write(tmp.join("genesis_payload.toml"), genesis).unwrap();
   338	        match verify_trust_root(&tmp).expect_err("child tamper must be detected") {
   339	            TrustRootError::Tampered { path, expected, actual } => {
   340	                assert!(path.ends_with("subdir/child.txt"),
   341	                    "expected error on child.txt, got: {path:?}");
   342	                assert_eq!(expected, "0".repeat(64));
   343	                assert_ne!(actual, expected);
   344	            }
   345	            other => panic!("expected Tampered on child, got {other:?}"),
   346	        }
   347	    }
   348	
   349	    /// Q1 conformance: child manifest with matching hashes verifies cleanly.
   350	    #[test]
   351	    fn verify_trust_root_passes_with_matching_child_manifest() {
   352	        let tmp = tempdir();
   353	        fs::create_dir_all(tmp.join("subdir")).unwrap();
   354	        let child_content = "the actual child content";
   355	        fs::write(tmp.join("subdir/child.txt"), child_content).unwrap();
   356	        let child_hash = hex_lower(&Sha256::digest(child_content.as_bytes()));
   357	        let parent_text = format!("{child_hash}  subdir/child.txt\n");
   358	        fs::write(tmp.join("subdir/MANIFEST.sha256"), &parent_text).unwrap();
   359	        let parent_hash = hex_lower(&Sha256::digest(parent_text.as_bytes()));
   360	        let genesis = format!(

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/tests/trust_root_immutability.rs | sed -n '84,160p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    84	    let keys: Vec<&str> = entries.iter().map(|(k, _)| k.as_str()).collect();
    85	
    86	    let required = [
    87	        // PREREG § 1.8 base
    88	        "src/kernel.rs",
    89	        "src/wal.rs",
    90	        "src/bus.rs",
    91	        // A8e13 fix Q1: src/boot.rs implements verify_trust_root itself
    92	        // (per Codex R11#1 + C-075 — the verifier must be qualifiable;
    93	        // tampering with boot.rs would silently bypass the entire gate).
    94	        "src/boot.rs",
    95	        "experiments/minif2f_v4/src/lean4_oracle.rs",
    96	        "constitution.md",
    97	        "cases/MANIFEST.sha256",
    98	        "handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json",
    99	        "handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md",
   100	        // Mid-term audit accounting layer
   101	        "src/drivers/llm_http.rs",
   102	        "experiments/minif2f_v4/src/cost_aggregator.rs",
   103	        "experiments/minif2f_v4/src/wall_clock.rs",
   104	        "experiments/minif2f_v4/src/post_hoc_verifier.rs",
   105	        "experiments/minif2f_v4/src/jsonl_schema.rs",
   106	        "experiments/minif2f_v4/src/bin/evaluator.rs",
   107	        // B6 add
   108	        "src/sdk/prompt_guard.rs",
   109	        // B7-extra add
   110	        "experiments/minif2f_v4/src/rollback_sim.rs",
   111	        // Phase A atom A3: per-agent AGENT_MODELS env var resolver
   112	        "experiments/minif2f_v4/src/agent_models.rs",
   113	        // Phase A atom A5: budget regime + MAX_TRANSACTIONS resolver
   114	        "experiments/minif2f_v4/src/budget_regime.rs",
   115	        // Phase A atom A6: FC-trace structured-event meta-witness
   116	        "experiments/minif2f_v4/src/fc_trace.rs",
   117	        // Phase A atom A7: heterogeneous-LLM provider plumbing (proxy + smoke)
   118	        "src/drivers/llm_proxy.py",
   119	        "scripts/smoke_siliconflow.sh",
   120	        "scripts/_smoke_siliconflow.py",
   121	        // A8e fix F1: unified run_id minted once per run (was run_corr_id ms drift)
   122	        "experiments/minif2f_v4/src/run_id.rs",
   123	        // A8e fix F2/F3: routing matrix + round-robin Python conformance tests
   124	        "scripts/test_llm_proxy.py",
   125	        // A8e2 fix G1: Rust wrapper that runs the Python suite on every cargo test
   126	        "experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs",
   127	        // A8e7: append-only audit history (companion to A8_EXIT_PACKET; per C-075
   128	        // gate machinery is constitutional — tampering with the chronology = silent
   129	        // governance drift).
   130	        "handover/audits/A8_AUDIT_HISTORY_2026-04-26.md",
   131	        // A8e11 fix P2: audit runner scripts that assemble the packet, append
   132	        // sources, and produce dual-audit transcripts. Per C-075 + Codex R10#2
   133	        // these are load-bearing gate machinery (R8/R9 runner defects
   134	        // demonstrated they are not incidental); silent tamper = silent gate bypass.
   135	        "handover/audits/run_codex_phase_a8_exit_audit.sh",
   136	        "handover/audits/run_gemini_phase_a8_exit_audit.py",
   137	        // 2026-04-25 dual-audit fixes
   138	        "src/main.rs",
   139	        "Cargo.lock",
   140	        "handover/preregistration/scripts/run_p0_calibration.sh",
   141	        "handover/preregistration/scripts/compute_p0.py",
   142	        // 2026-04-25 Phase A0 harness modernization
   143	        "rules/MANIFEST.sha256",
   144	        "rules/engine.py",
   145	        ".claude/hooks/judge.sh",
   146	        "tests/fc_alignment_conformance.rs",
   147	        // 2026-04-25 Phase A1 PREREG amendment
   148	        "handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md",
   149	    ];
   150	
   151	    for path in required {
   152	        assert!(
   153	            keys.contains(&path),
   154	            "Trust Root manifest missing required path: {path}\nactual keys: {keys:#?}"
   155	        );
   156	    }
   157	}
   158	
   159	#[test]
   160	fn test_pput_accounting_0_section_present() {

exec
/bin/bash -lc "nl -ba handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md | sed -n '20,100p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    20	
    21	## § 2. Amendment
    22	
    23	PREREG § 5.5 calibration **DEFERRED** indefinitely with the following operative substitution for Phase B → C transition and Phase E Gate H requirements:
    24	
    25	**`p_0` for guardrail purposes**: take the **PREREG § 5.5 ceiling itself = 0.10** and use it as `p_0` directly. Any artifact j whose `j-RR` regression rate exceeds 0.10 fails Gate H per the original guardrail logic.
    26	
    27	**Wording correction (A8 round-1 audit, Codex finding 1)**: an earlier draft of this section claimed `p_0 = 0.10` was the *strictest possible substitute*. That was backwards. The Gate H test is `j-RR ≤ p_0`, so a SMALLER `p_0` is stricter. `0.10` is the **maximum tolerated value** the PREREG admits — i.e., the LEAST strict admissible ceiling, not the strictest. Substituting it is the equivalent of running Gate H at the weakest threshold the PREREG ever sanctioned. The substitution is *operationally permitted* (the PREREG explicitly allows up to 0.10), but it is NOT a tighter-than-original guarantee.
    28	
    29	**Statistical implications** (re-stated cleanly):
    30	- No Type-I inflation. `j-RR` is a descriptive guardrail (PREREG § 5.4), not part of the inferential family — its threshold doesn't enter Holm-Bonferroni adjustment.
    31	- The substitution may be LESS protective than an eventual calibrated `p_0 < 0.10`. If/when calibration runs (per § 3 conditions, which are PRE-REQUISITES — they do not by themselves *guarantee* calibration completes before any specific phase), the empirical estimate may tighten the bar; until then Gate H operates at the ceiling.
    32	- Acceptable trade-off because (a) Phase B and C don't gate on `j-RR` (Gate H is a Phase E artifact gate), and (b) the substitution is **operationally permitted at any phase including Phase E** — see § 8 for the operative rule. If § 3 conditions complete before Phase E, calibration UPGRADES the bar; if not, Phase E proceeds with the ceiling substitution. There is no path under this amendment where Phase E is blocked by absence of calibration — that was an earlier draft's misclaim, removed by A8e8 fix M4 after Codex round-7 audit.
    33	
    34	**`genesis_payload.toml [pput_accounting_0].baseline_regression_rate`**: setting deferred to ArchitectAI commit window. Current value `0.0` is recognized as INVALID PLACEHOLDER (would auto-fail any artifact with any regression). Until calibration runs, **Gate H consumers MUST hardcode `p_0 = 0.10`** at the consumption site, not read from `genesis_payload.toml`.
    35	
    36	**`baseline_regression_jsonl_sha256`**: stays empty (calibration jsonl does not exist yet).
    37	
    38	## § 3. Conditions for re-calibration
    39	
    40	Calibration becomes worthwhile (and the deferral lifted) when ALL of:
    41	
    42	1. **N-experiments arc (Phase A-D of new plan, 2026-04-25 N-agents experiments) complete** — answers Q1/Q2/Q3 about N → PPUT, fixes (or rejects) the throttle hypothesis, sediments per-N best practices into evaluator. Without this, calibrating p_0 on a known-suboptimal N=3 swarm is calibrating against a moving baseline.
    43	
    44	2. **swarm_N=1 mode landed** (Phase A atom A2) — current `CONDITION=oneshot` is a different code path; PREREG § 5.5 ambiguous about which is the "control".
    45	
    46	3. **Per-agent budget normalization landed** (Phase A atom A5) — current `max_transactions=200` is fixed-tx budget; PREREG § 5.5 implicitly assumes tx-budget but doesn't specify; need explicit budget regime declaration for calibration to be reproducible.
    47	
    48	4. **Heterogeneous LLM agents experiment complete** (Phase A3.5 / E_hetero) — if hetero finds significant solve_rate uplift, the calibration must be done on the production model mix, not on homo n3 baseline.
    49	
    50	5. **Phase D ArchitectAI runtime exists** — calibration is part of Gate H gating Phase E. Doing it before Phase D = calibrating against a counterfactual ArchitectAI that doesn't exist.
    51	
    52	When ALL 5 conditions met: re-issue PREREG_AMENDMENT to lift the deferral + trigger the 576-run batch with the (then-current) production mode.
    53	
    54	## § 4. Impact on Phase B → C transition
    55	
    56	PREREG_PPUT_CCL_2026-04-26 § 5.5 originally listed p_0 calibration as a Phase B prerequisite ("Schedule: Phase B7 mandatory; not deferrable to Phase D"). This amendment **explicitly OVERRIDES that "not deferrable" clause** for the deferral conditions in § 3 above.
    57	
    58	Phase B → C exit checklist accordingly:
    59	- ❌ p_0 calibration jsonl frozen (was REQUIRED) → now DEFERRED with substitution per § 2
    60	- ✅ B1-B7 + B7-extra mode toggle infrastructure complete
    61	- ✅ Phase A0 harness modernization complete (post-2026-04-25 governance work)
    62	- ✅ Tools qualified (per case C-075): runner.sh, compute_p0.py, evaluator boot enforcement, etc.
    63	- ✅ Trust Root verifies clean
    64	
    65	Phase B → C dual-audit packet (next major milestone) must reference this amendment + show that Phase E Gate H consumer hardcodes `p_0 = 0.10`.
    66	
    67	## § 5. What this amendment does NOT change
    68	
    69	- **PREREG § 5.5 protocol itself** — the calibration *protocol* (288 control + 288 treatment paired runs, max-over-seeds, etc.) remains the agreed-upon procedure for IF calibration ever runs. Amendment defers the SCHEDULING, not the SCIENCE.
    70	- **PREREG § 1.8 Trust Root composition** — manifest entries unchanged by this amendment (this amendment doc is added per § 7 below).
    71	- **PREREG § 5.4 j-RR ≤ p_0 guardrail logic** — Gate H still uses the guardrail; just the p_0 source changes (hardcoded 0.10 instead of calibrated value).
    72	- **PREREG § 5.6 family total / N_max** — unchanged.
    73	- **All other PREREG § sections** — unchanged.
    74	
    75	## § 6. PREREG document treatment
    76	
    77	`PREREG_PPUT_CCL_2026-04-26.md` itself is **NOT EDITED** by this amendment. It remains the immutable round-4 frozen pre-registration. This amendment is a separate document referenced from § 5.5 forward via a pointer added to Trust Root manifest.
    78	
    79	This pattern is per CLAUDE.md "Common Law": amendments are recorded as separate cases / docs that supersede specific sections, leaving the original frozen for reproducibility. PREREG_PPUT_CCL_2026-04-26.md SHA-256 in Trust Root manifest UNCHANGED.
    80	
    81	## § 7. Trust Root impact
    82	
    83	Add this amendment doc to genesis_payload.toml [trust_root]:
    84	```
    85	"handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md" = "<sha256>"
    86	```
    87	
    88	Manifest size: 24 → 25 entries.
    89	
    90	## § 8. Audit requirement
    91	
    92	Per case C-073 ArchitectAI commit workflow: this amendment requires dual audit (Codex + Gemini, conservative VETO > CHALLENGE > PASS) before commit lands. Audit packet should specifically test:
    93	
    94	- Does the amendment violate any PREREG § 5.5 constraint? (Should not — defer is operationally permitted given § 5.5 ceiling.)
    95	- Does substitution of `p_0 = 0.10` invalidate any Gate H statistical claim? (Should not — `j-RR` is descriptive (PREREG § 5.4), outside the inferential family, so no Type-I inflation. Per § 2 wording correction: 0.10 is the LEAST-strict admissible ceiling, NOT a tighter-than-original substitute — the substitution may be less protective than an eventual calibrated `p_0 < 0.10`, but it is **operationally permitted at any phase including Phase E** by the original PREREG § 5.5 ceiling. § 3 conditions are PRE-REQUISITES for calibration to run at all; they do NOT guarantee calibration completes before any specific phase.)
    96	- Does deferral leave any phase blocked indefinitely? (Should not — § 3 lists explicit re-calibration conditions; if those never met, Phase E proceeds with the operationally-permitted ceiling substitution per § 2 final paragraph. Calibration UPGRADES the bar IF and WHEN § 3 conditions complete; absence of calibration is not a Phase E blocker.)
    97	
    98	## § 9. Cross-references
    99	
   100	- `PREREG_PPUT_CCL_2026-04-26.md` § 5.5 (the amended section, IMMUTABLE)

exec
/bin/bash -lc "nl -ba src/drivers/llm_proxy.py | sed -n '196,246p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   196	def detect_provider(model):
   197	    """Route by model identifier. v4 prefers explicit `provider:model`
   198	    syntax (e.g. `siliconflow:Qwen/Qwen2.5-7B-Instruct`); falls back
   199	    to model-string heuristics for backward compat with v3 callers.
   200	
   201	    Routing matrix (A8e fix F3 + A8e6 fix K2):
   202	      explicit `provider:...`            → that provider (if known)
   203	      slash-separated id (huggingface-style "Org/Model")
   204	                                          → siliconflow (dispatches the
   205	                                            full catalog including Qwen,
   206	                                            openai-compat, Meta, AND
   207	                                            `deepseek-ai/<distill>` SF-
   208	                                            catalog DeepSeek variants —
   209	                                            see K2 note below)
   210	      bare contains "deepseek"           → deepseek (api.deepseek.com)
   211	      bare "qwen3-*" / "qwen-*"          → dashscope (Aliyun Qwen direct)
   212	      else                                → dashscope (default fallback)
   213	
   214	    Round-1 audit caught a routing inversion: `Qwen/Qwen2.5-7B-Instruct`
   215	    used to misroute to dashscope because `m.startswith("qwen")` won
   216	    after the slash check. F3 promoted slash-form above bare-qwen.
   217	
   218	    Round-6 audit (Codex R6#2) caught a related collision: HuggingFace-
   219	    style `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B` is a SiliconFlow-
   220	    catalog model (the official DeepSeek API only serves the canonical
   221	    `deepseek-chat` / `deepseek-v4-flash` family, not the Distill
   222	    variants). The pre-K2 logic checked `"deepseek" in m` BEFORE the
   223	    slash check, so `deepseek-ai/...` misrouted to api.deepseek.com
   224	    and 404'd. K2 swaps the order: slash-form is now the FIRST routing
   225	    heuristic (after explicit prefix), and bare-deepseek substring
   226	    only fires when there's no slash.
   227	    """
   228	    if ":" in model:
   229	        prefix = model.split(":", 1)[0].lower()
   230	        if prefix in PROVIDERS:
   231	            return prefix
   232	    if "/" in model:
   233	        # Any slash-form (Qwen/..., openai/..., meta-llama/...,
   234	        # deepseek-ai/...) goes to the heterogeneous catalog provider.
   235	        # K2: this branch is FIRST so HuggingFace-style identifiers
   236	        # never silently fall through to a bare-string provider that
   237	        # doesn't actually serve them.
   238	        return "siliconflow"
   239	    m = model.lower()
   240	    if "deepseek" in m:
   241	        # Bare "deepseek-chat", "deepseek-v4-flash" → official endpoint.
   242	        # Slash-form like "deepseek-ai/..." won't reach here per K2.
   243	        return "deepseek"
   244	    if m.startswith("qwen"):
   245	        # Bare qwen3-*, qwen-* without slash = direct DashScope catalog.
   246	        return "dashscope"

exec
/bin/bash -lc "nl -ba scripts/test_llm_proxy.py | sed -n '70,100p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    70	        )
    71	        self.assertEqual(
    72	            llm_proxy.detect_provider("THUDM/glm-4-9b-chat"),
    73	            "siliconflow",
    74	        )
    75	
    76	    def test_deepseek_slash_form_routes_to_siliconflow(self):
    77	        # A8e6 fix K2 (Codex R6#2): `deepseek-ai/DeepSeek-R1-Distill-*`
    78	        # is a SiliconFlow-catalog model — the official DeepSeek API
    79	        # at api.deepseek.com only serves bare `deepseek-chat` and
    80	        # `deepseek-v4-flash`, not the Distill variants. Pre-K2 the
    81	        # routing checked "deepseek" substring BEFORE the slash check
    82	        # and misrouted these to api.deepseek.com, which returns 404.
    83	        # Pinned here so the slash-vs-substring precedence never
    84	        # regresses.
    85	        self.assertEqual(
    86	            llm_proxy.detect_provider("deepseek-ai/DeepSeek-R1-Distill-Qwen-7B"),
    87	            "siliconflow",
    88	        )
    89	        self.assertEqual(
    90	            llm_proxy.detect_provider("deepseek-ai/DeepSeek-V2.5"),
    91	            "siliconflow",
    92	        )
    93	        # Sanity: bare deepseek model ids still route to deepseek.
    94	        self.assertEqual(
    95	            llm_proxy.detect_provider("deepseek-chat"),
    96	            "deepseek",
    97	        )
    98	        self.assertEqual(
    99	            llm_proxy.detect_provider("deepseek-v4-flash"),
   100	            "deepseek",

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs | sed -n '38,86p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    38	    let root = repo_root();
    39	    let script = root.join("scripts").join("test_llm_proxy.py");
    40	    assert!(
    41	        script.is_file(),
    42	        "scripts/test_llm_proxy.py must exist at the canonical path; got {:?}",
    43	        script
    44	    );
    45	
    46	    // A8e3 fix H6 (Codex R3#3): the wrapper MUST fail closed when
    47	    // `python3` is missing. A "soft skip" is exactly the silent-pass
    48	    // failure mode that Gemini's round-1 VETO targeted — a gate that
    49	    // disappears under environmental drift is not a gate. If a runner
    50	    // environment lacks Python, that's a CI configuration bug, not an
    51	    // acceptable-skip case. Explicit opt-out:
    52	    // `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1` — must be set deliberately,
    53	    // never set by default. The bypass is logged loudly so the gate's
    54	    // absence is visible in test output.
    55	    let opt_out = std::env::var("SKIP_LLM_PROXY_PYTHON_CONFORMANCE")
    56	        .as_deref() == Ok("1");
    57	    if opt_out {
    58	        eprintln!(
    59	            "[G1] SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1 — gate explicitly \
    60	             bypassed. This is a downgraded run; the V3L-27 round-robin \
    61	             conformance battery did NOT execute."
    62	        );
    63	        return;
    64	    }
    65	    let python_check = Command::new("python3").arg("--version").output();
    66	    assert!(
    67	        python_check.is_ok(),
    68	        "python3 not found on PATH; G1 conformance gate requires it. \
    69	         Install python3 + the openai SDK (see scripts/test_llm_proxy.py \
    70	         header) or — only with deliberate intent — set \
    71	         SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1 to bypass."
    72	    );
    73	
    74	    let output = Command::new("python3")
    75	        .arg(&script)
    76	        .current_dir(&root)
    77	        .output()
    78	        .expect("spawn python3 scripts/test_llm_proxy.py");
    79	
    80	    let stdout = String::from_utf8_lossy(&output.stdout);
    81	    let stderr = String::from_utf8_lossy(&output.stderr);
    82	
    83	    assert!(
    84	        output.status.success(),
    85	        "scripts/test_llm_proxy.py must exit 0 (round-robin + routing \
    86	         conformance battery for V3L-27 mitigation).\n\

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '330,421p;488,506p;1008,1048p;1456,1498p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   330	    // a special case. The regime is informational here; no scaling.
   331	    let oneshot_regime = minif2f_v4::budget_regime::BudgetRegime::TotalProposal;
   332	    let oneshot_budget_base: u32 = 1;
   333	
   334	    // A8e fix F1 (Codex#2 + Gemini Q4): one run_id minted at function
   335	    // entry, passed to both fc_event!s and make_pput. Eliminates the
   336	    // millisecond drift between `run_corr_id` (was generated here) and
   337	    // make_pput's internal recomputation. Phase D consumers can now
   338	    // join FC events to v2 jsonl rows by `run_id` equality.
   339	    let run_id = minif2f_v4::run_id::mint_run_id("oneshot", problem_file);
   340	
   341	    let oracle = Lean4Oracle::new(
   342	        problem_statement.to_string(), theorem_name.to_string(), lean_path.to_string(),
   343	    );
   344	
   345	    // PPUT-CCL B3 (mid-term audit P0-C fix 2026-04-25): open the wall-clock
   346	    // bracket BEFORE prompt construction. PREREG § 5 / plan B3 define T_i
   347	    // as "first agent prompt construction → final Lean call". Marking after
   348	    // the construction (prior wiring) under-counted prompt-build time and
   349	    // forced the conformance test to relax its 7100ms assertion.
   350	    wc.mark_first_read();
   351	
   352	    // R-22 v2 clause 4 stays reject-only; the prompt must prevent fences at the source.
   353	    // Chat models (deepseek-chat, 2026-04-22) default to ```lean fences; verifier hard-rejects
   354	    // any response containing ``` so the instruction must be explicit. See F-2026-04-22-08.
   355	    let prompt = format!(
   356	        "Complete the following Lean 4 proof. Output ONLY the tactic proof body as raw Lean \
   357	         tokens. DO NOT wrap in markdown code fences (no ```). No prose, no backticks.\n\n{}",
   358	        problem_statement
   359	    );
   360	
   361	    let client = ResilientLLMClient::new(proxy_url, 1800, 2);
   362	    // Model-aware max_tokens: deepseek-chat caps at 8192; reasoner needs 16000 for thinking.
   363	    let max_toks = if model.contains("chat") { 8000 } else { 16000 };
   364	    let request = GenerateRequest {
   365	        model: model.to_string(),
   366	        messages: vec![Message { role: "user".into(), content: prompt }],
   367	        temperature: Some(0.2),
   368	        max_tokens: Some(max_toks),
   369	    };
   370	
   371	    // PPUT-CCL B6 runtime gate: scan the assembled prompt for PPUT scalars
   372	    // before the call goes out. Any leak aborts deterministically — Goodhart
   373	    // shield at the LLM-call boundary.
   374	    assert_no_metric_leak(&request.messages[0].content);
   375	    match client.generate(&request).await {
   376	        Ok(response) => {
   377	            acc.record_llm_call(response.prompt_tokens, response.completion_tokens);
   378	            acc.record_proposal(false);
   379	            // Rule 22 v2 clause 4: reject markdown fences
   380	            if response.content.contains("```") {
   381	                wc.mark_final_accept();
   382	                // P0-A: caller declares both runtime + post-hoc legs.
   383	                // Fence reject = neither leg fired.
   384	                // A4: no Lean call reached → verifier_wait_ms=0;
   385	                // 1 proposal made (the LLM response), 1 distinct.
   386	                return make_pput(problem_file, "oneshot", model,
   387	                                 false, false, start, 0, 0, 1,
   388	                                 None, None, None, None, None,
   389	                                 acc.total_run_token_count(),
   390	                                 acc.failed_branch_count,
   391	                                 wc.elapsed_ms().unwrap_or(0),
   392	                                 false, 1, 1, verifier_wait_ms,
   393	                                 oneshot_regime, oneshot_budget_base, &run_id);
   394	            }
   395	
   396	            // Phase A atom A4 (FC1-N12): bracket every Lean call so verifier
   397	            // wait is observable in the emitted v2 row.
   398	            let v_t0 = Instant::now();
   399	            let verdict = oracle.verify_omega(&response.content);
   400	            let v_elapsed = v_t0.elapsed().as_millis() as u64;
   401	            verifier_wait_ms += v_elapsed;
   402	            // A6 FC1-N12 (Lean oracle scope): per-call event with verdict
   403	            // + elapsed_ms. Phase D consumer derives the verifier-cost
   404	            // distribution and the verify-success rate. Run-level emit
   405	            // (no agent_id; oneshot has only one virtual agent).
   406	            let verdict_str = match &verdict {
   407	                Ok(true) => "Ok(true)",
   408	                Ok(false) => "Ok(false)",
   409	                Err(_) => "Err",
   410	            };
   411	            minif2f_v4::fc_trace::emit_event(
   412	                minif2f_v4::fc_trace::FcId::Fc1N12,
   413	                // A8e fix F1: stamp the unified run_id (not the
   414	                // round-1 `oneshot_{problem_file}` placeholder) so
   415	                // Phase D can join by equality.
   416	                &run_id, None, None,
   417	                &[
   418	                    ("verdict", minif2f_v4::fc_trace::json_str(verdict_str)),
   419	                    ("elapsed_ms", v_elapsed.to_string()),
   420	                ],
   421	            );
   488	async fn run_swarm(
   489	    problem_file: &str, problem_statement: &str, theorem_name: &str,
   490	    lean_path: &str, proxy_url: &str, model: &str, n_agents: usize,
   491	) -> PputResult {
   492	    let start = Instant::now();
   493	    let condition = format!("n{}", n_agents);
   494	
   495	    // A8e fix F1 (Codex#2 + Gemini Q4): single run_id minted ONCE per
   496	    // run, threaded into both fc_event!s and make_pput. Replaces the
   497	    // round-1 `run_corr_id` (FC events) ↔ make_pput-internal `run_id`
   498	    // (v2 jsonl) split that introduced millisecond drift on the join key.
   499	    let run_id = minif2f_v4::run_id::mint_run_id(&condition, problem_file);
   500	
   501	    let kernel = Kernel::new();
   502	    let config = BusConfig {
   503	        // Phase 2.1 (C-043 candidate): OMEGA-accepted proofs are auto-written
   504	        // as tape nodes (mandatory wtool per Art. IV). Full proofs can be
   505	        // long; raise bus caps so winning nodes don't get size-vetoed. Agent
   506	        // partials still typically <1200; no behavioural regression.
  1008	                                // A8e fix F4 (Codex#3): emit FC1-N12 for the swarm
  1009	                                // verify_omega_detailed call. Round-1 audit showed
  1010	                                // FC1-N12 was only emitted in oneshot, leaving the
  1011	                                // primary swarm verify path invisible to Phase D.
  1012	                                let r_alone_verdict = match &r_alone {
  1013	                                    Ok((true, _)) => "Ok(true)",
  1014	                                    Ok((false, _)) => "Ok(false)",
  1015	                                    Err(_) => "Err",
  1016	                                };
  1017	                                minif2f_v4::fc_trace::emit_event(
  1018	                                    minif2f_v4::fc_trace::FcId::Fc1N12,
  1019	                                    &run_id, Some(tx as u64), Some(agent_id.as_str()),
  1020	                                    &[
  1021	                                        ("verdict", minif2f_v4::fc_trace::json_str(r_alone_verdict)),
  1022	                                        ("elapsed_ms", v_alone_elapsed.to_string()),
  1023	                                        ("path", minif2f_v4::fc_trace::json_str("alone")),
  1024	                                    ],
  1025	                                );
  1026	                                let (full_proof, path_choice, r_final) = match &r_alone {
  1027	                                    Ok((true, _)) => (payload.clone(), "alone", r_alone.clone()),
  1028	                                    _ if !tape_chain.is_empty() => {
  1029	                                        // Path 2: tape + payload (A4 verifier_wait bracket)
  1030	                                        let combined = format!("{}\n{}", tape_chain, payload);
  1031	                                        let v_t1 = Instant::now();
  1032	                                        let r_combined = oracle.verify_omega_detailed(&combined);
  1033	                                        let v_combined_elapsed = v_t1.elapsed().as_millis() as u64;
  1034	                                        verifier_wait_ms += v_combined_elapsed;
  1035	                                        // A8e fix F4: FC1-N12 for the tape+payload retry.
  1036	                                        let r_combined_verdict = match &r_combined {
  1037	                                            Ok((true, _)) => "Ok(true)",
  1038	                                            Ok((false, _)) => "Ok(false)",
  1039	                                            Err(_) => "Err",
  1040	                                        };
  1041	                                        minif2f_v4::fc_trace::emit_event(
  1042	                                            minif2f_v4::fc_trace::FcId::Fc1N12,
  1043	                                            &run_id, Some(tx as u64), Some(agent_id.as_str()),
  1044	                                            &[
  1045	                                                ("verdict", minif2f_v4::fc_trace::json_str(r_combined_verdict)),
  1046	                                                ("elapsed_ms", v_combined_elapsed.to_string()),
  1047	                                                ("path", minif2f_v4::fc_trace::json_str("tape+payload")),
  1048	                                            ],
  1456	    // A6 FC2-N22 (HALT — natural MaxTxExhausted): the canonical
  1457	    // budget-exhausted exit. Phase D filters reason="MaxTxExhausted"
  1458	    // to compute solve_rate-vs-budget curves; pairs with the A5
  1459	    // budget_regime stamp on the v2 jsonl row.
  1460	    minif2f_v4::fc_trace::emit_event(
  1461	        minif2f_v4::fc_trace::FcId::Fc2N22,
  1462	        &run_id, Some(max_transactions as u64), None,
  1463	        &[
  1464	            ("reason", minif2f_v4::fc_trace::json_str("MaxTxExhausted")),
  1465	            ("budget_regime", minif2f_v4::fc_trace::json_str(budget_regime.label())),
  1466	            ("budget_max_transactions", budget_max_tx_base.to_string()),
  1467	            ("proposal_count", proposal_count.to_string()),
  1468	        ],
  1469	    );
  1470	    make_pput(problem_file, &condition, &run_model_label,
  1471	              false, false, start, 0, 0,
  1472	              max_transactions as u64, Some(tool_dist), upr,
  1473	              None, None, None,
  1474	              acc.total_run_token_count(),
  1475	              acc.failed_branch_count,
  1476	              wc.elapsed_ms().unwrap_or(0),
  1477	              true,
  1478	              proposal_hashes.len() as u64,
  1479	              proposal_count,
  1480	              verifier_wait_ms,
  1481	              budget_regime, budget_max_tx_base, &run_id)
  1482	}
  1483	
  1484	fn make_pput(
  1485	    problem: &str, condition: &str, model: &str,
  1486	    runtime_accepted: bool, post_hoc_verified: bool, start: Instant,
  1487	    gp_tokens: u64, gp_nodes: usize, tx_count: u64,
  1488	    tool_dist: Option<HashMap<String, u32>>,
  1489	    unique_payload_ratio: Option<f64>,
  1490	    gp_payload: Option<String>,
  1491	    gp_path: Option<String>,
  1492	    gp_proof_file: Option<String>,
  1493	    total_run_token_count: u64,
  1494	    failed_branch_count: u32,
  1495	    total_wall_time_ms: u64,
  1496	    // Phase A atom A4 (decomposed metrics). All callers must pass
  1497	    // explicit values — the v2 fields are non-Optional.
  1498	    hit_max_tx: bool,

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '1493,1530p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1493	    total_run_token_count: u64,
  1494	    failed_branch_count: u32,
  1495	    total_wall_time_ms: u64,
  1496	    // Phase A atom A4 (decomposed metrics). All callers must pass
  1497	    // explicit values — the v2 fields are non-Optional.
  1498	    hit_max_tx: bool,
  1499	    distinct_proposals: u64,
  1500	    total_proposals: u64,
  1501	    verifier_wait_ms: u64,
  1502	    // Phase A atom A5 (FC2-N22 budget regime stamp). Caller declares
  1503	    // the regime + base BEFORE the loop so MaxTxExhausted rows are
  1504	    // unambiguous about which partitioning rule produced them.
  1505	    budget_regime: minif2f_v4::budget_regime::BudgetRegime,
  1506	    budget_max_transactions: u32,
  1507	    // A8e fix F1 (Codex#2 + Gemini Q4): run_id minted by caller (run_swarm
  1508	    // or run_oneshot) at function entry; passed in here so the v2 jsonl
  1509	    // row stamps the SAME identifier the FC events used. No more ms drift.
  1510	    run_id: &str,
  1511	) -> PputResult {
  1512	    // PPUT-CCL Phase B B4 (mid-term audit P0-A fix 2026-04-25):
  1513	    // make_pput is now PURELY computational. The caller MUST decide both
  1514	    // `runtime_accepted` (did the evaluator's runtime gate fire?) and
  1515	    // `post_hoc_verified` (did Lean independently confirm the proof?). The
  1516	    // prior implementation derived `post_hoc_verified = has_gp` internally,
  1517	    // which would have laundered Phase C Soft Law fake-accepts into the
  1518	    // North Star pput_verified. Forcing the caller to pass both legs makes
  1519	    // Soft Law's design point unmissable: any caller that fakes runtime
  1520	    // accept must explicitly pass post_hoc_verified=verify_post_hoc(...)
  1521	    // or the divergence will surface immediately.
  1522	    //
  1523	    // Phase B all callers pass `(runtime_accepted, post_hoc_verified) = (X, X)`
  1524	    // because runtime IS Lean today. Phase C diverges at the Soft Law
  1525	    // mode call site, not inside this function.
  1526	    let has_gp = runtime_accepted; // legacy `has_golden_path` field semantics
  1527	    let elapsed = start.elapsed().as_secs_f64();
  1528	    let pput = if has_gp && elapsed > 0.0 { 100.0 / elapsed } else { 0.0 };
  1529	    // C-012 provenance: populated from env vars; None when unset (backward compat).
  1530	    let build_sha = std::env::var("BUILD_SHA").ok();

exec
/bin/bash -lc "rg -n \"fc_trace::emit_event\\(\" experiments/minif2f_v4/src/bin/evaluator.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
411:            minif2f_v4::fc_trace::emit_event(
723:            minif2f_v4::fc_trace::emit_event(
775:            minif2f_v4::fc_trace::emit_event(
1017:                                minif2f_v4::fc_trace::emit_event(
1041:                                        minif2f_v4::fc_trace::emit_event(
1113:                                        minif2f_v4::fc_trace::emit_event(
1318:                                minif2f_v4::fc_trace::emit_event(
1358:                                        minif2f_v4::fc_trace::emit_event(
1460:    minif2f_v4::fc_trace::emit_event(

exec
/bin/bash -lc 'rg -n "265|267|37-entry|38-entry|Trust Root|manifest|PASS" handover/audits/run_codex_phase_a8_exit_audit.sh handover/audits/run_gemini_phase_a8_exit_audit.py' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/audits/run_gemini_phase_a8_exit_audit.py:60:**Role**: skeptical adversarial reviewer. Independent of Codex. Conservative merge rule (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.
handover/audits/run_gemini_phase_a8_exit_audit.py:62:**Mandate**: Phase A is pre-flight (days 1–3 of the 30-day arc). 8 atoms (A0a–e + A1–A7) must be auditable as a unit before Phase B (kernel instrumentation + PPUT accounting) is authorized to start. PREREG_PPUT_CCL_2026-04-26.md (round-4 PASS/PASS, frozen) + PREREG_AMENDMENT_p0_defer_2026-04-25.md (Trust Root entry 25) are the contracts.
handover/audits/run_gemini_phase_a8_exit_audit.py:67:- **VERDICT**: PASS / CHALLENGE / VETO
handover/audits/run_gemini_phase_a8_exit_audit.py:153:    f"**Test baseline**: 265 PASS + 29 ignored + 0 failed (Rust); "
handover/audits/run_gemini_phase_a8_exit_audit.py:154:    f"16/16 PASS (Python proxy tests)\n"
handover/audits/run_gemini_phase_a8_exit_audit.py:155:    f"**Trust Root**: 37-entry manifest verifies clean\n"
handover/audits/run_codex_phase_a8_exit_audit.sh:35:**Role**: skeptical adversarial reviewer. Independent of Gemini. Conservative merge rule (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.
handover/audits/run_codex_phase_a8_exit_audit.sh:37:**Mandate**: Phase A is pre-flight (days 1–3 of the 30-day arc). 8 atoms (A0a–e + A1–A7) must be auditable as a unit before Phase B (kernel instrumentation + PPUT accounting) is authorized to start. PREREG_PPUT_CCL_2026-04-26.md (round-4 PASS/PASS, frozen) + PREREG_AMENDMENT_p0_defer_2026-04-25.md (Trust Root entry 25) are the contracts you're auditing against.
handover/audits/run_codex_phase_a8_exit_audit.sh:42:- **VERDICT**: PASS / CHALLENGE / VETO
handover/audits/run_codex_phase_a8_exit_audit.sh:105:  printf '**Test baseline**: 265 PASS + 29 ignored + 0 failed (Rust); 16/16 PASS (Python proxy tests)\n'
handover/audits/run_codex_phase_a8_exit_audit.sh:106:  printf '**Trust Root**: 37-entry manifest verifies clean\n'

exec
/bin/bash -lc 'rg -n "strictest|least-strict|Round-|A8e|Codex|Gemini|previous|fix|post-A8|chain position|added by|caught" handover/audits/A8_EXIT_PACKET_2026-04-26.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
5:**Authority**: ArchitectAI commit (Art. V.1.2). Input to dual external audit (Codex + Gemini) per Art. V.1.3 + memory `feedback_dual_audit`.
7:**Decision rule**: PASS → Phase B (kernel instrumentation + PPUT accounting) authorized; CHALLENGE → in-cycle fixes; VETO → Phase A redesign.
11:**Document split**: this packet is the **stable current-state snapshot** of Phase A at exit — it describes WHAT IS, not how it got here. The chronological history of audit rounds + their in-cycle fix bundles lives in the **companion** document `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` (append-only). Past audit transcripts are at `handover/audits/{CODEX,GEMINI}_PHASE_A8_EXIT_AUDIT_2026-04-26{,_R2,_R3,_R4,_R5,_R6,_R7,...}.md`. Reviewers needing closure-of-prior-finding context read the history doc; reviewers verifying current-state correctness against Phase B prerequisites read THIS packet.
37:(Subsequent audit-cycle in-cycle fix bundle commits are recorded in `A8_AUDIT_HISTORY_2026-04-26.md`.)
62:- **Statistical framing**: `0.10` is the **least-strict admissible value** the PREREG ceiling allows. `j-RR ≤ p_0` makes a SMALLER `p_0` stricter, so this substitution is operationally permitted but is NOT a tighter-than-original guarantee. No Type-I inflation since `j-RR` is descriptive (PREREG § 5.4), outside the inferential family. The substitution is operationally permitted at any phase including Phase E; if/when § 3 re-calibration conditions complete, calibration upgrades the bar.
104:  - explicit `provider:...` prefix → that provider (if known)
143:- (Q2.a) `p_0 = 0.10` substitution wording: is `PREREG_AMENDMENT_p0_defer_2026-04-25.md` § 2 + § 8 internally consistent + statistically correct (least-strict admissible; no Type-I inflation; substitution operative at any phase including Phase E)?
156:- (Q4.d) Are there any open P0 defects from any prior round? Cross-reference `A8_AUDIT_HISTORY_2026-04-26.md` to verify each in-cycle fix bundle's closures hold against current source.
162:- (Q5.c) Does this packet contain ANY round-N retrospective text, fix-shipped headers, "previous round caught X" claims, or historical lineage like "(post-A8e F4)" / "(added by A8eN)" / "(chain position N via A8e)" — anywhere? It should NOT — those belong only in the history doc.

exec
/bin/bash -lc 'rg -n "Total: 37|Total: 38|A8e13|37 → 38|38|267 PASS|265 PASS|pending" handover/alignment/TRACE_MATRIX_v2_2026-04-25.md genesis_payload.toml handover/audits/A8_AUDIT_HISTORY_2026-04-26.md handover/audits/A8_EXIT_PACKET_2026-04-26.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/audits/A8_EXIT_PACKET_2026-04-26.md:153:- (Q4.a) Spot-check: re-run `cargo test --workspace`; expect 265 PASS / 29 ignored / 0 failed.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:71:- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 15/15 PASS (Python)
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:86:**Round-3 fixes shipped (`A8e3`, commit `3d38ba5`)** — six fixes:
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:91:- **H5** TRACE_MATRIX § 5 item 7: "(commit pending) + 6 wired" → "CLOSED + 9 wired".
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:99:- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `3d38ba5` (post-A8e3)
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:100:- Test baseline: 265 PASS / 29 ignored / 0 failed; 15/15 Python
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:125:- Test baseline: 265 PASS / 29 ignored / 0 failed; 15/15 Python
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:134:1. (Codex R5#1) Packet missing Round-4 outcome + A8e4 fixes shipped section; `<pending>` commit placeholders for A8e2/A8e3/A8e4 never replaced.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:140:- **J2** `<pending>` placeholders replaced with actual SHAs (`0af47b7` / `3d38ba5` / `8693789`); A8e5 self-reference uses "this commit".
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:151:- Test baseline: 265 PASS / 29 ignored / 0 failed; 15/15 Python
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:160:1. (Codex R6#1) RQ14 closure criterion contradicts itself — packet has `<pending>` placeholders in J2 bullet + RQ14 itself.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:167:- **K1** `<pending>` placeholder closure: J2 bullet + RQ14 reworded.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:181:- Test baseline: 265 PASS / 29 ignored / 0 failed; 16/16 Python proxy tests
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:191:2. (Codex R7#2) This history doc says A8e7 commit is `<this commit's SHA>` and "Round 7 dual audit pending after this commit lands" — but A8e7 is already in Trust Root as landed. Same placeholder-staleness pattern earlier rounds tried to eliminate.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:202:- **M2** (Codex R7#2): This history doc's A8e7 entry now stamps the actual SHA `aaedc9d` and round-7 verdicts (above) instead of "pending". Round-7 history sealed.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:213:- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 16/16 PASS (Python)
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:223:2. (Codex R8#2) This history doc's round-7 entry left an unresolved placeholder `<aaedc9d-successor>` for the A8e8 commit SHA; cumulative table row 7 still says "pending pending pending" despite round-7 verdicts being recorded above.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:229:- **N3** (Codex R8#3): Runner script header metadata refreshed to current state (265 PASS / 35-entry manifest); pre-A8e values removed.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:239:- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 16/16 PASS (Python)
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:255:- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 16/16 PASS (Python)
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:277:- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 16/16 PASS (Python)
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:299:- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 16/16 PASS (Python)
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:316:**Round-11 fixes shipped (`A8e13`, this commit)** — six fixes (2 substantive + 4 documentary):
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:317:- **Q1** (Codex R11#1 — substantive arch): `src/boot.rs` extended with `verify_child_manifest`. When a Trust Root entry path ends in `/MANIFEST.sha256`, the verifier now ALSO parses the manifest contents (GNU sha256sum format) and verifies each child file's actual hash against the claim. Manifests regenerated with repo-relative paths (was a mix of bare-filename and dir-relative). Two new unit tests pin the contract (`verify_trust_root_detects_child_manifest_tamper` + `verify_trust_root_passes_with_matching_child_manifest`). Plus: `src/boot.rs` itself added to Trust Root (meta-finding caught during fix — the verifier file was not previously protected by the verifier; tampering with boot.rs would silently bypass the entire gate). Manifest 37 → 38.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:318:- **Q2** (Gemini R11#3 — substantive refactor): `make_pput` signature `Option<u64>` → plain `u64` for the 3 affected parameters. All 9 production call sites + 4 test fixtures updated. `wc.elapsed_ms()` returns `Option<u64>` upstream; call sites add `.unwrap_or(0)` at the boundary. `cargo test --workspace`: 267 PASS (was 265 + 2 new boot tests).
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:319:- **Q3** (Codex R11#2): `A8_AUDIT_HISTORY` extended with A8e12 entry + Round 11 outcome + A8e13 fixes shipped section + cumulative table rows for round 11/12.
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:341:| 12 | pending | pending | pending | — | — | ~5 |
genesis_payload.toml:68:# Total: 37 files. Progression A0=24 → A1=25 (PREREG amendment) → A3=26
genesis_payload.toml:91:source_pool_sha256 = "77179cf2598b0df707d78a6663e763121dfe8e73a6538073155f488feab95093"
genesis_payload.toml:115:"src/sdk/prompt_guard.rs" = "b4f7b164770d1a7203b8143f773c66f748994d60a42ece38471f2f7f2839f4f1"
genesis_payload.toml:126:"experiments/minif2f_v4/src/bin/evaluator.rs" = "537af991c9168c40ac3d866b164a274b4a587b098bb0fc471238a0ef37a37c6d"
genesis_payload.toml:133:"handover/audits/A8_AUDIT_HISTORY_2026-04-26.md" = "c39230ad1c910380e79024a5d1dbfe6b17758b150d7505320447a93c1c5e9837"
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:16:- A8e9 (post-A8e8, FIX-THEN-PROCEED after A8 round-8 audit Codex CHALLENGE / Gemini PASS): three fixes including one substantive PREREG logic miss. **N1 (substantive Codex R8#1)**: PREREG_AMENDMENT § 8 audit-requirements text still contained the stale claim "Gate H is Phase E and § 3 conditions ensure calibration runs first" — round-7 M4 fixed § 2(b)'s identical false claim but missed the parallel text in § 8. Now reworded so § 2 + § 3 + § 8 all consistently express "substitution operationally permitted at any phase including Phase E; § 3 conditions are pre-requisites for calibration to run, not guarantees of completion before any phase". Re-hashed in Trust Root. **N2 (Codex R8#2)**: A8_AUDIT_HISTORY round-7 entry stamped with actual A8e8 SHA `857872e` (was `<aaedc9d-successor>` placeholder); cumulative table row 7 sealed (CHALLENGE/CHALLENGE; 6 findings; 1 substantive); row 8 added (CHALLENGE/PASS; 3 findings; 1 substantive); row 9 left as pending. **N3 (Codex R8#3)**: audit runner script header metadata refreshed — was emitting "264 PASS + 30-entry manifest" (round-1 era values) every time the runners executed; now emits current state (265 PASS / 35-entry manifest) and references the history doc for chronology. Trust Root manifest unchanged at 35. **Real-bug yield through 8 rounds: 9 substantive findings caught + closed** (5 routing/correctness in R1, 1 fail-closed gate in R3, 1 routing collision in R6, 1 PREREG § 2 logic in R7, 1 PREREG § 8 logic in R8).
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:19:- A8e6 (post-A8e5, FIX-THEN-PROCEED after A8 round-6 audit Codex CHALLENGE / Gemini CHALLENGE — both narrow): one real correctness fix + 4 doc fixes. **K1** packet `<pending>` placeholder cleanup (J2 + RQ14 wording) — same precedent as A8e5 J2 but applied to round-6 self-references. **K2 (real bug, Codex R6#2)** `src/drivers/llm_proxy.py` `detect_provider`: HuggingFace-style `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B` was misrouting to `api.deepseek.com` because `"deepseek" in m` won before the slash check; pre-K2 logic would 404 on every Distill variant since the official DeepSeek API doesn't serve them. K2 swaps the order — slash-form is now the FIRST routing heuristic (after explicit prefix), and bare-deepseek substring only fires when there's no slash. New `test_deepseek_slash_form_routes_to_siliconflow` in `scripts/test_llm_proxy.py` pins this (covers `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B` + `deepseek-ai/DeepSeek-V2.5` AS WELL AS the bare-form sanity case `deepseek-chat` → deepseek). **K3** TRACE_MATRIX A8e3 row stale "21 positional args" → 24 (matches packet update from A8e5 J5; Codex R6#3 caught the inconsistency between TRACE_MATRIX and the packet). **K4** TRACE_MATRIX A6 row top-line "Six anchor sites" → "9 anchor sites" with A6-original-6 + F4-added-3 breakdown (Gemini R6#1 caught the contradiction between the line's first sentence and its own follow-up). **K5** packet § 3 A5/A6 fix attribution: "A8e3 fix H4 corrected" → "A8e4 fix I2 corrected" (Gemini R6#2 caught the cross-reference error — H4 corrected the genesis_payload.toml TR-header comment, I2 corrected the packet § 3 numbers). Trust Root manifest unchanged at 34 (re-hashes of `llm_proxy.py` + `scripts/test_llm_proxy.py`; no new files).
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:20:- A8e5 (post-A8e4, FIX-THEN-PROCEED after A8 round-5 audit Codex CHALLENGE / Gemini PASS — same split as round-4): five narrow doc/source-text fixes; zero source-code changes. **J1** Round-4 outcome + A8e4 fixes shipped section added to the packet (was missing — Codex R5#1 caught the gap). **J2** `<pending>` commit placeholders for A8e2/A8e3/A8e4 replaced with actual SHAs (`0af47b7` / `3d38ba5` / `8693789`); A8e5 self-reference now uses "this commit" to break the placeholder-staleness pattern. **J3** packet § 6 Q6 question text reworded: "24 → 30 / 6 new entries" → "24 → 34 / 10 new entries"; full enumerated list added. **J4** TRACE_MATRIX § 1 top-bullet TR-deltas corrected: A5 `25 → 26` → `26 → 27`; A6 `26 → 27` → `27 → 28` (matches the canonical chain in `genesis_payload.toml` header). **J5** round-3 retrospective text about `make_pput` arg count: 21 → 24 (Gemini R5 spot-check; F1's `run_id` parameter brought the post-A8e count from 21 to 24). Trust Root manifest unchanged at 34 (no new files; doc-only changes). All findings are documentary; both auditors confirmed the code is sound + ready for Phase B.
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:22:- A8e3 (post-A8e2, FIX-THEN-PROCEED after A8 round-3 audit Codex CHALLENGE / Gemini CHALLENGE — both convergent on packet staleness; no VETO): six narrow cleanup fixes. **H1** A8 packet § 3 A1 description: "Mathematically conservative" → "least-strict admissible value" with explicit Type-I implications. **H2** packet § 3 A6 description: 6 → 9 anchor sites (added F4 swarm sites explicitly). **H3** packet § 6 Q2.a / Q4.a / Q4.d marked CLOSED with closure rationale. **H4** `genesis_payload.toml` TR header phrasing about A1: "conservative ceiling" → "max-tolerated ceiling". **H5** § 5 item 7 here in TRACE_MATRIX: "(commit pending) + 6 wired" → "CLOSED + 9 wired". **H6** G1 wrapper test fail-closed on missing `python3` (was soft skip; Codex R3 caught the silent-pass risk); explicit opt-out `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1` for deliberate downgrades (logged loudly). Trust Root manifest unchanged at 34 (only re-hashes; no new files). Non-blocking observation deferred to Phase B+: `make_pput` signature was 21 positional args at round-3; A8e fix F1 added the `run_id` parameter, bringing the post-A8e count to **24** (Gemini R3 Finding 2 + Gemini R5/Codex R6 spot-checks; A8e6 K3 corrected here) — refactor to a builder pattern is recommended but not gating.
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:88:**Total: 24 entries** (15 from B7 + 1 B7-extra rollback_sim + 4 dual-audit fixes + 4 A0 harness). A1 (PREREG amendment) → 25; A3 (agent_models.rs) → 26; A5 (budget_regime.rs) → 27; A6 (fc_trace.rs) → 28; A7 (llm_proxy.py + smoke_siliconflow.sh + _smoke_siliconflow.py) → 31; A8e (run_id.rs + test_llm_proxy.py) → 33; A8e2 (llm_proxy_python_conformance.rs) → 34; A8e7 (A8_AUDIT_HISTORY_2026-04-26.md append-only chronology) → 35; A8e11 (audit runner scripts run_codex/run_gemini per C-075 gate-machinery rule) → 37. When B7-extra calibration eventually runs, the calibration jsonl makes 38 entries; future Phase C's `--mode` flag binary (TBD location) makes 39.
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:117:- 🔨/⚠️ count: 0 → **0** (no actionable rows pending in v2 scope)
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:135:- (planned) B7-extra calibration freeze → 38

exec
/bin/bash -lc "nl -ba handover/audits/run_codex_phase_a8_exit_audit.sh | sed -n '96,110p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    96	prompt_size=$(wc -c < "$TMP_PROMPT")
    97	echo "[codex a8 exit] prompt size: ${prompt_size} chars" >&2
    98	
    99	t0=$(date +%s)
   100	{
   101	  printf '# Codex Phase A → B Exit Audit (PPUT-CCL arc)\n'
   102	  printf '**Round**: %s\n' "$ROUND"
   103	  printf '**Date**: 2026-04-26\n'
   104	  printf '**Reference**: see `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` for full chronology including atom commit chain + per-round verdicts/fixes.\n'
   105	  printf '**Test baseline**: 265 PASS + 29 ignored + 0 failed (Rust); 16/16 PASS (Python proxy tests)\n'
   106	  printf '**Trust Root**: 37-entry manifest verifies clean\n'
   107	  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
   108	  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
   109	} > "$OUT"
   110	t1=$(date +%s)

exec
/bin/bash -lc "nl -ba handover/audits/run_gemini_phase_a8_exit_audit.py | sed -n '148,158p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   148	    f"**Round**: {round_label}\n"
   149	    f"**Date**: 2026-04-26\n"
   150	    f"**Reference**: see `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` "
   151	    f"for full chronology including atom commit chain + per-round "
   152	    f"verdicts/fixes.\n"
   153	    f"**Test baseline**: 265 PASS + 29 ignored + 0 failed (Rust); "
   154	    f"16/16 PASS (Python proxy tests)\n"
   155	    f"**Trust Root**: 37-entry manifest verifies clean\n"
   156	    f"**Elapsed**: {elapsed:.1f}s\n"
   157	    f"**Prompt size**: {len(brief):,} chars\n\n---\n\n"
   158	)

exec
/bin/bash -lc "nl -ba handover/alignment/TRACE_MATRIX_v2_2026-04-25.md | sed -n '82,92p;130,138p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    82	|---|---|
    83	| `rules/MANIFEST.sha256` (proxy for 14 rules/active/R-*.yaml) | Same pattern as cases/MANIFEST.sha256: glob hashed once, manifest tracked in Trust Root. Tampering with R-018 enforcement = "warn" silently bypasses constitution sudo gate. |
    84	| `rules/engine.py` | The interpreter of the rules. Tampering with engine.py = silent rule bypass even with intact rule files. |
    85	| `.claude/hooks/judge.sh` | The PreToolUse hook that invokes engine.py + implements R-016 fc_trace + constitution.md special-case. Tampering = bypass entire gate stack. |
    86	| `tests/fc_alignment_conformance.rs` | Witness battery for TRACE_MATRIX ✅ rows. Tampering = false PASS hides drift. |
    87	
    88	**Total: 24 entries** (15 from B7 + 1 B7-extra rollback_sim + 4 dual-audit fixes + 4 A0 harness). A1 (PREREG amendment) → 25; A3 (agent_models.rs) → 26; A5 (budget_regime.rs) → 27; A6 (fc_trace.rs) → 28; A7 (llm_proxy.py + smoke_siliconflow.sh + _smoke_siliconflow.py) → 31; A8e (run_id.rs + test_llm_proxy.py) → 33; A8e2 (llm_proxy_python_conformance.rs) → 34; A8e7 (A8_AUDIT_HISTORY_2026-04-26.md append-only chronology) → 35; A8e11 (audit runner scripts run_codex/run_gemini per C-075 gate-machinery rule) → 37. When B7-extra calibration eventually runs, the calibration jsonl makes 38 entries; future Phase C's `--mode` flag binary (TBD location) makes 39.
    89	
    90	## § 4. New constitutional case-law (A0c)
    91	
    92	5 new cases C-071..C-075 (commit 2a65339) sediment 2026-04-25 session decisions as constitutional precedent. Each cross-referenced in TRACE_MATRIX rows:
   130	- A7 llm_proxy.py + smoke_siliconflow.{sh,py} → 31
   131	- A8e run_id.rs + test_llm_proxy.py → 33
   132	- A8e2 llm_proxy_python_conformance.rs → 34
   133	- A8e7 A8_AUDIT_HISTORY_2026-04-26.md → 35
   134	- A8e11 audit runner scripts (run_codex_phase_a8_exit_audit.sh + run_gemini_phase_a8_exit_audit.py) → **37**
   135	- (planned) B7-extra calibration freeze → 38
   136	- (planned) Phase C mode-flag binary → 39+
   137	
   138	## § 7. Cross-references

codex
I audited the working tree at `c5b3711`. Core source checks pass, but the A8 gate artifacts are not internally current enough for a PASS.

**Findings**

1. **A8 current-state packet is stale after A8e13.**  
   The packet says it is the stable current-state snapshot ([A8_EXIT_PACKET_2026-04-26.md:11](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:11)), but still reports `265` Rust passes and a `37`-entry Trust Root ([lines 43, 47](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:43)), and repeats `37`/`265` in gate questions ([lines 123, 150, 153](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:123)). Current history says A8e13 changed the manifest `37 -> 38` and test baseline to `267 PASS` ([A8_AUDIT_HISTORY_2026-04-26.md:317-318](/home/zephryj/projects/turingosv4/handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:317)). `genesis_payload.toml` also has `src/boot.rs` in the manifest ([line 108](/home/zephryj/projects/turingosv4/genesis_payload.toml:108)) while its header still says `Total: 37 files` ([lines 68-77](/home/zephryj/projects/turingosv4/genesis_payload.toml:68)). This is a gate-document false-baseline defect, not a runtime defect.

2. **The Trust-Rooted audit runners still emit stale baselines.**  
   `run_codex_phase_a8_exit_audit.sh` prints `265 PASS` and `37-entry manifest` ([lines 105-106](/home/zephryj/projects/turingosv4/handover/audits/run_codex_phase_a8_exit_audit.sh:105)); the Gemini runner does the same ([run_gemini_phase_a8_exit_audit.py:153-155](/home/zephryj/projects/turingosv4/handover/audits/run_gemini_phase_a8_exit_audit.py:153)). Since these runners are themselves Trust Root entries ([genesis_payload.toml:134-135](/home/zephryj/projects/turingosv4/genesis_payload.toml:134)), this is load-bearing gate machinery drift.

3. **TRACE_MATRIX still has stale manifest-count lineage.**  
   It says A8e11 brought the manifest to `37` and that B7-extra calibration will make `38` ([TRACE_MATRIX_v2_2026-04-25.md:88](/home/zephryj/projects/turingosv4/handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:88), [lines 134-135](/home/zephryj/projects/turingosv4/handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:134)). A8e13 already used entry 38 for `src/boot.rs`, per history and manifest. Secondary, same class: the table still describes `rules/MANIFEST.sha256` as proxying 14 rules ([line 83](/home/zephryj/projects/turingosv4/handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:83)), while genesis now documents 15.

**Spot Checks**

- `cargo test --workspace`: PASS, `267` passed / `29` ignored / `0` failed.
- `python3 scripts/test_llm_proxy.py`: PASS, `16/16`.
- `bash scripts/smoke_siliconflow.sh`: PASS, `3/3` keys responded.
- Q1 code checks pass: unified `run_id` is minted in `run_oneshot` and `run_swarm` ([evaluator.rs:339](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:339), [499](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:499)); slash-form `deepseek-ai/...` routes to SiliconFlow ([llm_proxy.py:232-238](/home/zephryj/projects/turingosv4/src/drivers/llm_proxy.py:232)); Python wrapper fails closed if `python3` is missing ([llm_proxy_python_conformance.rs:65-72](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs:65)).
- PREREG amendment wording is now internally sound: least-strict ceiling, no Type-I inflation, no Phase E calibration guarantee ([PREREG_AMENDMENT_p0_defer_2026-04-25.md:27-34](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:27), [95-96](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:95)).

**VERDICT**: CHALLENGE  
**Conviction**: high  
**Recommendation**: FIX-THEN-PROCEED
2026-04-26T08:51:36.075685Z ERROR codex_core::session: failed to record rollout items: thread 019dc8f8-9e46-7cc0-b7d1-26b91ac09009 not found
tokens used
579,721
I audited the working tree at `c5b3711`. Core source checks pass, but the A8 gate artifacts are not internally current enough for a PASS.

**Findings**

1. **A8 current-state packet is stale after A8e13.**  
   The packet says it is the stable current-state snapshot ([A8_EXIT_PACKET_2026-04-26.md:11](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:11)), but still reports `265` Rust passes and a `37`-entry Trust Root ([lines 43, 47](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:43)), and repeats `37`/`265` in gate questions ([lines 123, 150, 153](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:123)). Current history says A8e13 changed the manifest `37 -> 38` and test baseline to `267 PASS` ([A8_AUDIT_HISTORY_2026-04-26.md:317-318](/home/zephryj/projects/turingosv4/handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:317)). `genesis_payload.toml` also has `src/boot.rs` in the manifest ([line 108](/home/zephryj/projects/turingosv4/genesis_payload.toml:108)) while its header still says `Total: 37 files` ([lines 68-77](/home/zephryj/projects/turingosv4/genesis_payload.toml:68)). This is a gate-document false-baseline defect, not a runtime defect.

2. **The Trust-Rooted audit runners still emit stale baselines.**  
   `run_codex_phase_a8_exit_audit.sh` prints `265 PASS` and `37-entry manifest` ([lines 105-106](/home/zephryj/projects/turingosv4/handover/audits/run_codex_phase_a8_exit_audit.sh:105)); the Gemini runner does the same ([run_gemini_phase_a8_exit_audit.py:153-155](/home/zephryj/projects/turingosv4/handover/audits/run_gemini_phase_a8_exit_audit.py:153)). Since these runners are themselves Trust Root entries ([genesis_payload.toml:134-135](/home/zephryj/projects/turingosv4/genesis_payload.toml:134)), this is load-bearing gate machinery drift.

3. **TRACE_MATRIX still has stale manifest-count lineage.**  
   It says A8e11 brought the manifest to `37` and that B7-extra calibration will make `38` ([TRACE_MATRIX_v2_2026-04-25.md:88](/home/zephryj/projects/turingosv4/handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:88), [lines 134-135](/home/zephryj/projects/turingosv4/handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:134)). A8e13 already used entry 38 for `src/boot.rs`, per history and manifest. Secondary, same class: the table still describes `rules/MANIFEST.sha256` as proxying 14 rules ([line 83](/home/zephryj/projects/turingosv4/handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:83)), while genesis now documents 15.

**Spot Checks**

- `cargo test --workspace`: PASS, `267` passed / `29` ignored / `0` failed.
- `python3 scripts/test_llm_proxy.py`: PASS, `16/16`.
- `bash scripts/smoke_siliconflow.sh`: PASS, `3/3` keys responded.
- Q1 code checks pass: unified `run_id` is minted in `run_oneshot` and `run_swarm` ([evaluator.rs:339](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:339), [499](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:499)); slash-form `deepseek-ai/...` routes to SiliconFlow ([llm_proxy.py:232-238](/home/zephryj/projects/turingosv4/src/drivers/llm_proxy.py:232)); Python wrapper fails closed if `python3` is missing ([llm_proxy_python_conformance.rs:65-72](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs:65)).
- PREREG amendment wording is now internally sound: least-strict ceiling, no Type-I inflation, no Phase E calibration guarantee ([PREREG_AMENDMENT_p0_defer_2026-04-25.md:27-34](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:27), [95-96](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:95)).

**VERDICT**: CHALLENGE  
**Conviction**: high  
**Recommendation**: FIX-THEN-PROCEED
