# Codex Phase A → B Exit Audit (PPUT-CCL arc) — round 2
**Date**: 2026-04-26 (post A8e fixes)
**Round**: R7
**Commits**: 2e7f75a / d8950ee / 2a65339 / e94e1b9 / 62c4e14 / 6be6eb4 / 180a300 / 7f4bc0c / a5c78e4 / 30f2a14 / 89994c7 / 90953d6 / 60292dc / 5a56ff6
**Test baseline**: 264 PASS + 29 ignored + 0 failed (Rust); 15/15 PASS (Python proxy tests)
**Trust Root**: 33-entry manifest verifies clean
**Prompt size**: 295957 chars

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
session id: 019dc88a-ce14-7570-9da8-fabfdca9f056
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

## Round 7 — A8e7 structural rewrite + relaunch

**Trigger**: rounds 2–6 produced a recurring documentary CHALLENGE pattern that single-point fixes could not close. **Diagnosis**: `A8_EXIT_PACKET` was conflating stable-snapshot artifact + append-only chronology in one document. **Fix** (commit `<this commit's SHA>` — A8e7): split into two docs following the project's existing constitutional pattern (stable spec + delta log; cf. constitution.md + Art. V.3 amendment log; PREREG + PREREG_AMENDMENT; TRACE_MATRIX_v0 + v1 + v2). The packet becomes current-state only; THIS history doc becomes append-only chronology.

**Round 7 dual audit pending after this commit lands.**

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
| 7 | pending | pending | pending | — | — | ~5 |

Cumulative cost so far ~$30; well within memory `feedback_dual_audit` Phase A reservation. Real-bug yield: 6 substantive findings caught + closed (5 in R1, 1 in R3, 1 in R6 = 7 real bugs discovered + fixed pre-Phase B). Remaining CHALLENGE findings post-A8e6 are pure documentary, traceable to the category error A8e7 now addresses.


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
- The substitution may be LESS protective than an eventual calibrated `p_0 < 0.10`. When calibration runs (per § 3 conditions), the empirical estimate likely tightens the bar; until then we operate at the ceiling.
- Acceptable trade-off because (a) Phase B and C don't gate on `j-RR` (Gate H is a Phase E artifact gate), and (b) the conditions in § 3 ensure calibration runs *before* Phase E, so the loose substitution never reaches the artifact-acceptance moment.

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
- Does substitution of `p_0 = 0.10` invalidate any Gate H statistical claim? (Should not — `j-RR` is descriptive (PREREG § 5.4), outside the inferential family, so no Type-I inflation. Per § 2 wording correction: 0.10 is the LEAST-strict admissible ceiling, NOT a tighter-than-original substitute — the substitution may be less protective than an eventual calibrated `p_0 < 0.10`, accepted because Gate H is Phase E and § 3 conditions ensure calibration runs first.)
- Does deferral leave any phase blocked indefinitely? (Should not — § 3 lists explicit re-calibration conditions; if those never met, Phase E proceeds with the operationally-permitted ceiling substitution per § 2 final paragraph.)

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
                                 Some(acc.total_run_token_count()),
                                 Some(acc.failed_branch_count),
                                 wc.elapsed_ms(),
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
                              Some(acc.total_run_token_count()),
                              Some(acc.failed_branch_count),
                              wc.elapsed_ms(),
                              false, 1, 1, verifier_wait_ms,
                              oneshot_regime, oneshot_budget_base, &run_id)
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
                              oneshot_regime, oneshot_budget_base, &run_id)
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
                                       Some(acc.total_run_token_count()),
                                       Some(acc.failed_branch_count),
                                       wc.elapsed_ms(),
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
                                                        Some(acc.total_run_token_count()),
                                                        Some(acc.failed_branch_count),
                                                        wc.elapsed_ms(),
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
                                                        Some(acc.total_run_token_count()),
                                                        Some(acc.failed_branch_count),
                                                        wc.elapsed_ms(),
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
              Some(acc.total_run_token_count()),
              Some(acc.failed_branch_count),
              wc.elapsed_ms(),
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
            Some(2000), Some(0), Some(15_000),
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
            Some(8_000), Some(199), Some(120_000),
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
            Some(2_000), Some(49), Some(40_000),
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
- A5 (post-A4): per-agent budget normalization — new `budget_regime` module (`BUDGET_REGIME` + `MAX_TRANSACTIONS` env vars; 4-variant enum; pure parser + scaler + env-coupled resolver); `budget_regime` + `budget_max_transactions` added as non-Optional v2 fields on `RunAggregate` and the legacy `PputResult`; loop bound at `run_swarm` switched from hardcoded `let max_transactions = 200` to `resolve_budget(n_agents)` — default (env unset) preserves Phase B baseline (`total_proposal × 200`) bit-for-bit. PREREG_AMENDMENT_p0_defer § 3 condition 3 satisfied: `MaxTxExhausted` rows now disambiguated across N values. FC-trace: FC2-N22 (HALT decomposition by budget regime) + FC1-N7 (δ instances determining the per-agent share under PerAgent regime). Trust Root manifest 26 → 27 (corrected A8e5 J4 — the A1 PREREG amendment had taken the count to 25, A3's `agent_models.rs` to 26 before A5 landed; round-1 packet undercounted by 1).
- A6 (post-A5): per-line FC tagging via structured JSON events — new `fc_trace` module (pure stdlib; zero new deps); `FcId` enum (FC1-N7 / FC1-N11 / FC1-N12 / FC1-E18 / FC2-N20 / FC2-N22 / FC3-N31); `fc_event!`-style `emit_event` API; `FC_TRACE=1` gate (cached in `OnceLock`); `FC_TRACE_FILE=<path>` redirects emit to file (default sink stderr). 9 anchor sites wired (post-A8e total — 8 in `run_swarm` + 1 in `run_oneshot`): the original A6 commit shipped 6 sites (FC2-N22 synthetic short-circuit, FC2-N20 mr tick, FC2-N22 OMEGA full-proof, FC2-N22 OMEGA per-tactic, FC2-N22 natural MaxTxExhausted with `budget_regime` payload, FC1-N12 verify bracket in oneshot). A8e fix F4 added 3 more in `run_swarm` (FC1-N12 verify_omega_detailed × 2 paths + FC1-N12 verify_partial), bringing the post-A8e total to 9 production emit sites. Counted by grepping `fc_trace::emit_event(` in `experiments/minif2f_v4/src/bin/evaluator.rs` (Gemini R6 finding: round-5 phrasing "Six anchor sites" was stale relative to its own following sentence — corrected here in A8e6 K4). End-to-end smoke test exercises FC_TRACE=1 in a child process (subprocess required because `OnceLock` caches the gate-read; resolves item 7 of TRACE_MATRIX § 5 "Per-line FC tagging via tracing crate"). FC-trace: meta-witness for the 5-step compile loop (Proposal → Lean ground truth → Logging → Capability compilation → ↑H-VPPUT). Trust Root manifest 27 → 28 (corrected A8e5 J4; matches the canonical chain in `genesis_payload.toml` header).
- A7 (post-A6): heterogeneous-LLM provider plumbing — `src/drivers/llm_proxy.py` ported from v3 with one load-bearing v4 change (per-provider multi-key round-robin: 3 SiliconFlow keys split concurrent traffic across separate rate-limit pools, mitigating V3L-27 single-key N=30 collapse). `scripts/smoke_siliconflow.sh` + `scripts/_smoke_siliconflow.py` probe each of the 3 keys (Qwen/Qwen2.5-7B-Instruct, max_tokens=8) — A7 verified all 3 keys responding 2026-04-26 (1.5–3s latency, 33+1 tokens; round-robin distributes [2,2,2] across 6 calls). New `siliconflow:<model>` provider-prefix syntax in `detect_provider()` for unambiguous routing in `AGENT_MODELS` payloads (Phase D heterogeneous swarms). Memory `reference_siliconflow.md` records SiliconFlow as the heterogeneous-LLM lane (NOT a fallback target). FC-trace: FC1-N7 (δ/AI provider expansion — heterogeneous δ instances across SF catalog enable Phase D meta-loop). Trust Root manifest 28 → 31 (3 entries: `llm_proxy.py` + 2 smoke scripts).
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

**Total: 24 entries** (15 from B7 + 1 B7-extra rollback_sim + 4 dual-audit fixes + 4 A0 harness). A1 (PREREG amendment) → 25; A3 (agent_models.rs) → 26; A5 (budget_regime.rs) → 27; A6 (fc_trace.rs) → 28; A7 (llm_proxy.py + smoke_siliconflow.sh + _smoke_siliconflow.py) → 31; A8e (run_id.rs + test_llm_proxy.py) → 33; A8e2 (llm_proxy_python_conformance.rs) → 34; A8e7 (A8_AUDIT_HISTORY_2026-04-26.md append-only chronology) → 35. When B7-extra calibration eventually runs, the calibration jsonl makes 36 entries; future Phase C's `--mode` flag binary (TBD location) makes 37.

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
7. ~~**Per-line FC tagging via tracing crate** — Plan agent's recommendation in N-experiments brainstorm. Phase A6 deferred; will land before Phase B (homogeneous experiments).~~ **CLOSED**: A6 (`89994c7`) shipped `fc_trace.rs` + 6 production emit sites (5 in `run_swarm` + 1 in `run_oneshot`); A8e fix F4 added 3 more in `run_swarm` (`verify_omega_detailed` alone + tape+payload retry, plus `verify_partial`), bringing the post-A8e total to **9 wired anchor sites**. Implementation chose pure stdlib over the `tracing` crate to avoid a new dep tree; the macro surface (`emit_event` + `FcId` enum) was kept small so Phase D+ can swap to a real `tracing-subscriber` bridge locally.

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
- A3 agent_models.rs → 26
- A5 budget_regime.rs → 27
- A6 fc_trace.rs → 28
- A7 llm_proxy.py + smoke_siliconflow.{sh,py} → 31
- A8e run_id.rs + test_llm_proxy.py → 33
- A8e2 llm_proxy_python_conformance.rs → 34
- A8e7 A8_AUDIT_HISTORY_2026-04-26.md → **35**
- (planned) B7-extra calibration freeze → 36
- (planned) Phase C mode-flag binary → 37+

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
#       max-tolerated ceiling — least-strict admissible per amendment
#       § 2 wording correction A8e fix F6; per case C-073 ArchitectAI
#       commit authority)
#
# Total: 35 files. Progression A0=24 → A1=25 (PREREG amendment) → A3=26
# (agent_models.rs) → A5=27 (budget_regime.rs) → A6=28 (fc_trace.rs) →
# A7=31 (llm_proxy.py + 2 smoke scripts) → A8e=33 (run_id.rs +
# test_llm_proxy.py) → A8e2=34 (llm_proxy_python_conformance.rs Rust
# wrapper) → A8e7=35 (A8_AUDIT_HISTORY append-only chronology — per
# C-075 the gate machinery itself must be tamper-evident).
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
"rules/MANIFEST.sha256" = "a84d114a12680c596e1a5458a954a5829b21baa4530f197b9aba65f95443be14"
"rules/engine.py" = "932d9a2b7a3249a7eb5825c0b5c714a9913cd9aa9e058f789e64992b140e40b3"
".claude/hooks/judge.sh" = "a2be9e6ed51e39f2e9cfd302d62a0234a772abc41f145702143d2557dd6fda3e"
"tests/fc_alignment_conformance.rs" = "b3e75979ad2d175b9c45135be6ea1d94ce95184c6896468330c12dbfc1f719db"
"src/kernel.rs" = "893fd67534caf7a3d9abd6efbd202556348b6491cd6d4c6bdb224d2ad75b1af0"
"src/wal.rs" = "1ac7637aa09531b1c7232163f5df48f7193251594c4ed20e0d0fc85cea8f84dc"
"src/bus.rs" = "df28ffe514a3272a3d10fca4568fd424a76e754e9785c109a5459f163f7fd14c"
"src/drivers/llm_http.rs" = "615596b68956b4a8925110edc99aa746a5543527724ec394bb9dda163147ed7a"
"src/drivers/llm_proxy.py" = "22f632234a6d3e1a0547651537cf26846c64c47b85f5b1c3a1bcd1ae1f139997"
"scripts/smoke_siliconflow.sh" = "6ad54e7c0ab8221f475360dcad80eeeb0d6da0fd55c27acdb1cefb2b390f5341"
"scripts/_smoke_siliconflow.py" = "778eea2988312f250efa47fcfe620d86187d01b96f07a98501f9795a333726ca"
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
"experiments/minif2f_v4/src/bin/evaluator.rs" = "290b2f0be3edc43b7f439506bf650743cd268a0b76ddd489fb3e5cc1da9a24ba"
"constitution.md" = "91e5913603eebaf134ac93d0f9ed7bb2b7916132203f6c2fae84d73329056ad6"
"cases/MANIFEST.sha256" = "dbe48bc11cac18ad1164c55b11aefd5be4a7b546e31c3df08fb4403c7f41e90d"
"handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json" = "69be702e3e0a4bb33c62bd02056495309289fd9271972dc733b4a1ba3dd908e1"
"handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" = "64d565b4451d1f770425530bb7ffa874da602ddfd9d714ce7f7db6bbfab69463"
"handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md" = "820645d2a5cc8aaeda3d68b923a874934206bb1c0ae7b4356e94148b90c28b4c"
"experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs" = "d62ee67cb564355fa26959c1f92a1152f95ab55d987c005abf7810e2da7b83b8"
"handover/audits/A8_AUDIT_HISTORY_2026-04-26.md" = "bf1e22a47a8fc312872ea382bb339fb5ce2849fe781e240d7b309059fecede5f"

```

---

Give your INDEPENDENT Phase A → B exit audit. Cite §/file:line for every finding.

codex
I’ll audit against the repository state, not just the pasted packet, so line citations can point to actual files. I’m starting with the repo instructions and then I’ll verify the specific Q1–Q5 claims plus test gates.
exec
/bin/bash -lc 'find .. -name AGENTS.md -print' in /home/zephryj/projects/turingosv4
 succeeded in 1599ms:
../turingos/AGENTS.md
../omega_kepler_discovery/AGENTS.md
../omega/AGENTS.md
../turingosv2/AGENTS.md
../turingclaw/AGENTS.md

exec
/bin/bash -lc 'git status --short' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
?? handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R7.md

exec
/bin/bash -lc "rg --files | rg '("'^|/)A8_(EXIT_PACKET|AUDIT_HISTORY)_2026-04-26'"\\.md"'$|PREREG_AMENDMENT_p0_defer_2026-04-25'"\\.md"'$|genesis_payload'"\\.toml"'$|TRACE_MATRIX_v2_2026-04-25'"\\.md"'$|llm_proxy_python_conformance'"\\.rs"'$|test_llm_proxy'"\\.py"'$|evaluator'"\\.rs"'$|trust_root_immutability'"\\.rs"'$'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs
experiments/minif2f_v4/tests/trust_root_immutability.rs
genesis_payload.toml
experiments/minif2f_v4/src/bin/evaluator.rs
handover/audits/A8_AUDIT_HISTORY_2026-04-26.md
handover/audits/A8_EXIT_PACKET_2026-04-26.md
scripts/test_llm_proxy.py
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md

codex
The worktree already has one untracked audit file; I’ll leave it untouched and base this review on committed/current source files plus test execution. The first mismatch I’m checking is whether the A8e7 split updated every Trust Root count consistently.
exec
/bin/bash -lc "nl -ba handover/audits/A8_EXIT_PACKET_2026-04-26.md | sed -n '1,240p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# Phase A → B Exit Audit Packet (A8) — current-state snapshot
     2	
     3	**Arc**: PPUT-CCL (`PREREG_PPUT_CCL_2026-04-26.md` round-4 PASS/PASS + `PREREG_AMENDMENT_p0_defer_2026-04-25.md`).
     4	**Date**: 2026-04-26 (current-state — most recent A8e re-issue noted in `A8_AUDIT_HISTORY_2026-04-26.md`).
     5	**Authority**: ArchitectAI commit (Art. V.1.2). Input to dual external audit (Codex + Gemini) per Art. V.1.3 + memory `feedback_dual_audit`.
     6	
     7	**Decision rule**: PASS → Phase B (kernel instrumentation + PPUT accounting) authorized; CHALLENGE → in-cycle fixes; VETO → Phase A redesign.
     8	
     9	**FC-trace**: meta-witness across FC1 / FC2 / FC3 (atoms instrument all three subgraphs).
    10	
    11	**Document split (A8e7)**: this packet is the **stable current-state snapshot** of Phase A at exit. The chronological history of audit rounds 1–N + their in-cycle fix bundles lives in the **companion** document `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` (append-only). Past audit transcripts are at `handover/audits/{CODEX,GEMINI}_PHASE_A8_EXIT_AUDIT_2026-04-26{,_R2,_R3,_R4,_R5,_R6,...}.md`. Reviewers needing the full chronology read the history doc; reviewers verifying current-state correctness against Phase B prerequisites read THIS packet.
    12	
    13	---
    14	
    15	## § 1. Phase A scope and atom map
    16	
    17	Phase A = pre-flight (days 1–3 of the 30-day arc). Decomposed into 8 atoms (A0–A7) plus the in-cycle fix bundles A8e..A8e6 produced during the multi-round audit. The atom-level deliverables:
    18	
    19	- **A0** (a–e): harness modernization. Closed by `62c4e14` (A0e exit audit + 7-item fixes).
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
    37	In-cycle audit-fix bundles (chronology in `A8_AUDIT_HISTORY`):
    38	
    39	```
    40	5a56ff6  A8e   0af47b7  A8e2  3d38ba5  A8e3  8693789  A8e4  1622017  A8e5  dbcf53a  A8e6
    41	```
    42	
    43	## § 2. Current-state metrics
    44	
    45	| Metric | Value | Source / verification |
    46	|---|---|---|
    47	| `cargo test --workspace` PASS | **265** | re-runnable; all suites green |
    48	| `cargo test --workspace` ignored | 29 | Phase B+ deferred stubs |
    49	| `cargo test --workspace` failed | **0** | — |
    50	| `python3 scripts/test_llm_proxy.py` | **16/16 PASS** | proxy routing + round-robin conformance |
    51	| Trust Root manifest entries | **34** | `genesis_payload.toml [trust_root]` count + matches `trust_root_immutability::test_trust_root_manifest_includes_b2_b4_files` required-paths list |
    52	| `boot::tests::verify_trust_root_passes_on_intact_repo` | **PASS** | re-hashes match the manifest |
    53	| `bash scripts/smoke_siliconflow.sh` | **PASS (3/3 keys)** | live API; cost ~$0.005 per run |
    54	| FC-trace anchor sites in `evaluator.rs` | **9** | grep `fc_trace::emit_event(`; 8 in `run_swarm` + 1 in `run_oneshot` |
    55	| `make_pput` arity | **24 positional args** | refactor to builder pattern deferred to Phase B+ (Gemini R3 non-blocking observation) |
    56	
    57	Trust Root manifest progression: A0=24 → A1=25 → A3=26 → A5=27 → A6=28 → A7=31 → A8e=33 → A8e2=34.
    58	
    59	## § 3. Per-atom acceptance evidence (current state)
    60	
    61	### A0 (harness modernization)
    62	- Closed by dual-audit cycle ending `62c4e14`. CODEX_PHASE_A0_EXIT_AUDIT + GEMINI_PHASE_A0_EXIT_AUDIT both transitioned to PASS-equivalent after the 7 in-cycle fixes; no open P0.
    63	
    64	### A1 (PREREG amendment) — `6be6eb4`
    65	- File: `handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md`.
    66	- Substitutes `p_0 = 0.10` (PREREG § 5.5 ceiling) for the calibration-derived value at every Gate H consumer.
    67	- **Statistical framing** (post-A8e F6 + A8e2 G2 wording corrections): `0.10` is the **least-strict admissible value** the PREREG ceiling allows; `j-RR ≤ p_0` makes a SMALLER `p_0` stricter, so this substitution is operationally permitted but NOT a tighter-than-original guarantee. No Type-I inflation since `j-RR` is descriptive (PREREG § 5.4), outside the inferential family. May be less protective than an eventual calibrated `p_0 < 0.10`; acceptable because Gate H is Phase E and § 3 conditions ensure calibration runs first.
    68	- **FC-trace**: FC1-N12 + Art. V.1.2 + cases C-073 + C-075.
    69	
    70	### A2 (`swarm_N=1` mode) — `180a300`
    71	- New `parse_swarm_condition_n` in `experiments/minif2f_v4/src/bin/evaluator.rs` discriminates `n<digits>` from `oneshot` / `hybrid_v1` / malformed. PREREG_AMENDMENT § 3 condition 2 cleared.
    72	- **FC-trace**: FC2-N16 + FC1-N11.
    73	- 5 unit tests.
    74	
    75	### A3 (`AGENT_MODELS` env var) — `7f4bc0c`
    76	- New `experiments/minif2f_v4/src/agent_models.rs`. Pure parser + expander + env-coupled resolver. Heterogeneity gated by `PHASE_D_HETERO_OK=1` — Phase B+C single-model invariant enforced at startup BEFORE any LLM call.
    77	- **FC-trace**: FC1-N7 (δ/AI canonical identity per Agent_i).
    78	- 11 unit tests.
    79	
    80	### A4 (decomposed metrics) — `a5c78e4`
    81	- 3 non-Optional v2 fields on `RunAggregate` + legacy `PputResult`: `hit_max_tx`, `tactic_diversity`, `verifier_wait_ms`. Helper `compute_tactic_diversity`.
    82	- **FC-trace**: FC2-N22 + FC1-N11 + FC1-N12.
    83	- 5 conformance tests.
    84	
    85	### A5 (budget regime) — `30f2a14`
    86	- New `experiments/minif2f_v4/src/budget_regime.rs`. 4-variant `BudgetRegime` enum: `total_proposal` (default; current behavior preserved bit-for-bit) / `per_agent` (loop bound = base × N) / `token_total` (declared; startup-fatal `UnimplementedRegime`) / `wall_clock` (declared; startup-fatal). 2 new non-Optional v2 fields: `budget_regime` + `budget_max_transactions`.
    87	- `run_swarm` startup: `let max_transactions = 200` → `resolve_budget(n_agents)` with startup-fatal error path.
    88	- **FC-trace**: FC2-N22 + FC1-N7.
    89	- 16 unit tests; PREREG_AMENDMENT § 3 condition 3 cleared.
    90	- Trust Root: `experiments/minif2f_v4/src/budget_regime.rs` added (chain position 27).
    91	
    92	### A6 (FC tracing) — `89994c7` (+ A8e fix F4)
    93	- New `experiments/minif2f_v4/src/fc_trace.rs`. Pure stdlib (zero new deps). 7-variant `FcId` enum (FC1-N7 / FC1-N11 / FC1-N12 / FC1-E18 / FC2-N20 / FC2-N22 / FC3-N31). `FC_TRACE=1` gate cached in `OnceLock`; `FC_TRACE_FILE=<path>` redirects emit to file.
    94	- **9 wired anchor sites** (post-A8e total — 8 in `run_swarm` + 1 in `run_oneshot`):
    95	  - FC2-N22 synthetic short-circuit (swarm)
    96	  - FC2-N20 mr tick (swarm)
    97	  - FC2-N22 OMEGA full-proof accept (swarm)
    98	  - FC2-N22 OMEGA per-tactic accept (swarm)
    99	  - FC2-N22 natural MaxTxExhausted (swarm; carries `budget_regime` payload from A5)
   100	  - FC1-N12 verify bracket (oneshot)
   101	  - FC1-N12 swarm `verify_omega_detailed` path "alone" (added by A8e F4)
   102	  - FC1-N12 swarm `verify_omega_detailed` path "tape+payload" (added by A8e F4)
   103	  - FC1-N12 swarm `verify_partial` (added by A8e F4)
   104	- Per-run correlation: `experiments/minif2f_v4/src/run_id.rs::mint_run_id` (added by A8e F1) — one identifier per run, threaded into both `emit_event` and `make_pput` so FC events join v2 jsonl rows by equality.
   105	- **FC-trace**: meta-witness for the 5-step compile loop.
   106	- 7 tests (6 unit + 1 end-to-end smoke `tests/fc_trace_smoke.rs` exercising `FC_TRACE=1` in a child process). Plus 3 unit tests in `run_id.rs`.
   107	- Trust Root: `fc_trace.rs` (chain position 28) + `run_id.rs` (chain position 33 via A8e).
   108	
   109	### A7 (SiliconFlow plumbing) — `90953d6` (+ A8e2 fix G1 + A8e6 fix K2)
   110	- `src/drivers/llm_proxy.py` ported from v3 with one load-bearing v4 change: per-provider multi-key round-robin. `detect_provider()` routing (post-A8e F3 + A8e6 K2): `provider:...` explicit prefix wins; else slash-form ⇒ `siliconflow` FIRST (covers `Qwen/...`, `openai/...`, `meta-llama/...`, `deepseek-ai/...` HuggingFace-style ids); else `"deepseek" in m` ⇒ `deepseek` (bare-form only); else `m.startswith("qwen")` ⇒ `dashscope`; else `dashscope` default.
   111	- 3 SiliconFlow keys (primary / secondary / tertiary) split concurrent traffic across separate rate-limit pools — V3L-27 / case C-027 single-key N=30 collapse mitigation.
   112	- `scripts/smoke_siliconflow.sh` + `_smoke_siliconflow.py`: 3 keys × 1 probe (Qwen2.5-7B-Instruct, max_tokens=8). Verified all 3 keys responding.
   113	- `scripts/test_llm_proxy.py`: 16-test routing + round-robin conformance suite (no live API). Wrapped by `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` (added by A8e2 G1) so the suite runs on every `cargo test --workspace`. The wrapper FAILS CLOSED if `python3` is missing (A8e3 H6); explicit opt-out `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1`.
   114	- **FC-trace**: FC1-N7 (δ/AI provider expansion).
   115	- Memory: `reference_siliconflow.md` records SiliconFlow as the Phase D heterogeneous lane (NOT a probe-only target).
   116	- Trust Root chain positions 29–31: `llm_proxy.py` + `smoke_siliconflow.sh` + `_smoke_siliconflow.py`. A8e adds `scripts/test_llm_proxy.py` (32) + `experiments/minif2f_v4/src/run_id.rs` (33). A8e2 adds `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` (34).
   117	
   118	## § 4. Phase B → C exit checklist (from PREREG_AMENDMENT § 4) — Phase A side
   119	
   120	The PREREG amendment shifted the Phase B → C gate. From the Phase A perspective, the items it lists are now satisfied:
   121	
   122	- ❌ p_0 calibration jsonl frozen (was REQUIRED) → **DEFERRED with substitution per amendment § 2**: `p_0 = 0.10` hardcoded at every Gate H consumer.
   123	- ✅ B1–B7 + B7-extra mode toggle infrastructure complete (pre-Phase A baseline; round-4 PASS/PASS).
   124	- ✅ Phase A0 harness modernization complete (`62c4e14`).
   125	- ✅ Tools qualified per case C-075 (DO-178C tool qualification): `runner.sh`, `compute_p0.py`, evaluator boot enforcement, etc.
   126	- ✅ Trust Root verifies clean (`boot::tests::verify_trust_root_passes_on_intact_repo` PASS at 34-entry manifest).
   127	
   128	## § 5. Risks and known limitations entering Phase B
   129	
   130	1. **`per_agent` budget regime untested at runtime**. A5 unit tests verify the scaling math (`base × N`) and env-coupled resolver. No live-LLM run with `BUDGET_REGIME=per_agent` has been smoked. Phase B kernel instrumentation will be the first opportunity to observe its behavior on a real problem.
   131	2. **FC-trace coverage still partial**. 9 wired anchor sites cover HALT decomposition (FC2-N22 × 4 exit paths) + mr tick (FC2-N20) + Lean oracle scope (FC1-N12 × 4 sites: oneshot + swarm `verify_omega_detailed` × 2 + swarm `verify_partial`). Still NOT emitting: FC1-N7 prompt-build, FC1-N11 ∏p decision diversity (per-proposal), FC1-E18 preserve-Q_t (per ∏p=0), FC3-N31 WAL append. The `FcId` enum reserves these variants. Phase B+ kernel instrumentation should fill them in as the Phase B emit boundary lands.
   132	3. **SiliconFlow rate-limit at scale**. A7 verified 3 keys responding individually at N=1 concurrency. V3L-27 demonstrates collapse at N=30 single-key. The v4 multi-key round-robin should triple the safe N envelope but the actual sweet spot for our hetero swarm is unmeasured. Phase D heterogeneous-batch design should land a `--max-concurrency` knob (currently `LLM_PROXY_CONCURRENCY=5` env in proxy) tuned per provider.
   133	4. **Heterogeneous swarm = Phase D, not B/C**. Per F-2026-04-25-02 + the `agent_models.rs` `PHASE_D_HETERO_GATE_ENV_VAR` invariant, Phases B and C MUST stay single-model so ablation axes are not confounded. A7's plumbing exists for future Phase D work; Phase B uses the existing `deepseek-v4-flash` thinking-off backbone unchanged.
   134	5. **`make_pput` arity = 24 (Phase B+ refactor)**. Gemini R3 + R5 spot-checks identified the positional-arg list as a maintainability hazard; refactor to builder pattern (`PputResultBuilder`) recommended early in Phase B. Non-blocking; runtime correctness verified by 4 in-binary tests + the integration battery.
   135	
   136	## § 6. Specific questions for auditors
   137	
   138	(All round-1..round-6 questions and their closures are recorded in `A8_AUDIT_HISTORY_2026-04-26.md`. The questions below are the round-N+1 set — a fresh reviewer pass that does NOT need to re-litigate prior closures.)
   139	
   140	### Q1 — Code correctness spot-checks
   141	- (Q1.a) Does `experiments/minif2f_v4/src/bin/evaluator.rs` mint a single `run_id` at function entry and pass it to BOTH `fc_trace::emit_event` and `make_pput` so FC events and v2 jsonl rows share one identifier? Verify by reading `run_swarm` and `run_oneshot` entry points + `make_pput` signature.
   142	- (Q1.b) Does `src/drivers/llm_proxy.py::detect_provider` route `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B` to `siliconflow` (not `deepseek`)? Verify by reading the function + checking that `scripts/test_llm_proxy.py` includes `test_deepseek_slash_form_routes_to_siliconflow`.
   143	- (Q1.c) Does `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` fail closed when `python3` is missing? Verify by reading the test + spot-checking that the panic message describes the failure rather than a silent skip.
   144	
   145	### Q2 — PREREG amendment soundness
   146	- (Q2.a) `p_0 = 0.10` substitution: is the wording in `PREREG_AMENDMENT_p0_defer_2026-04-25.md` § 2 + § 8 consistent + statistically correct (least-strict admissible, not "conservative" / "strictest")? Verify by full-text grep for "conservative" / "strictest" — the only acceptable matches are explicit retrospective notes.
   147	- (Q2.b) Re-calibration conditions in § 3 list 5 items. Does coupling Phase B → C to Phase D readiness create any forward-progress hazard?
   148	- (Q2.c) The amendment's SHA-256 is in Trust Root. Does `boot::tests::verify_trust_root_passes_on_intact_repo` pass cleanly?
   149	
   150	### Q3 — Atomicity, FC-trace discipline, governance
   151	- (Q3.a) Each of A1–A7 + A8e..A8e6 was committed as one atomic commit with `FC-trace: <FC?-N?>` in the message. Verify by re-reading commit messages. Any commit missing FC-trace? Any commit that touched `constitution.md`?
   152	- (Q3.b) 5 cases C-071..C-075 sediment 2026-04-25 session decisions as constitutional precedent (governance / Veto-AI / ArchitectAI commit authority / FC-first / DO-178C tool qualification). Are the rulings standalone-readable + correctly cross-referenced?
   153	- (Q3.c) Trust Root manifest 34 entries. Are all 10 Phase A additions load-bearing (i.e., does tampering each one weaken the constitutional gate)?
   154	
   155	### Q4 — Phase A → B exit decision
   156	- (Q4.a) Spot-check: re-run `cargo test --workspace`; expect 265 PASS / 29 ignored / 0 failed.
   157	- (Q4.b) Spot-check: re-run `python3 scripts/test_llm_proxy.py`; expect 16/16 PASS.
   158	- (Q4.c) Spot-check: re-run `bash scripts/smoke_siliconflow.sh`; expect PASS (3/3 keys; live API; cost ~$0.005).
   159	- (Q4.d) Are there any open P0 defects from any prior round (1–6)? Cross-reference `A8_AUDIT_HISTORY_2026-04-26.md` to verify each in-cycle fix bundle's closures hold against current source.
   160	- (Q4.e) Phase B's first sub-atom is "JSONL schema v2 + C_i full-cost aggregator" (notepad). Are there any Phase A artifacts that would BLOCK that scope? E.g., schema fields needed by B1 not yet in `RunAggregate`?
   161	
   162	### Q5 — Packet/history split (A8e7 structural rewrite)
   163	- (Q5.a) Is the split between `A8_EXIT_PACKET` (stable current-state) and `A8_AUDIT_HISTORY` (append-only chronology) consistent with the project's existing pattern (constitution + Art. V.3 amendment log; PREREG + PREREG_AMENDMENT)?
   164	- (Q5.b) Is the history doc append-only in spirit — i.e., do past round entries describe what was true at that round's snapshot, without retroactive edits?
   165	- (Q5.c) Does this packet (current-state) contain ANY round-N retrospective text, fix-shipped headers, or "previous round caught X" claims? It should NOT — those belong only in the history doc.
   166	
   167	---
   168	
   169	End your audit with:
   170	- **VERDICT**: PASS / CHALLENGE / VETO
   171	- **Conviction**: low / medium / high
   172	- **Recommendation**: PROCEED to Phase B (kernel instrumentation + PPUT accounting) / FIX-THEN-PROCEED / REDESIGN
   173	
   174	PASS = current-state evidence passes Phase B prerequisites + no open P0 from prior rounds + no new substantive findings. CHALLENGE = correctable in one cycle. VETO = Phase A redesign required.

exec
/bin/bash -lc "nl -ba handover/audits/A8_AUDIT_HISTORY_2026-04-26.md | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# A8 Phase A → B Exit Audit — Chronological History
     2	
     3	**Companion**: `A8_EXIT_PACKET_2026-04-26.md` (canonical current-state). This doc is the **append-only** chronology of round-N outcomes + the in-cycle fixes that closed each round's findings. Past entries are FROZEN — corrections to historical facts go in a new "Correction (round-M)" sub-section under the affected round, never via in-place edit.
     4	
     5	**Constitutional pattern**: mirrors `constitution.md` + Art. V.3 amendment log; mirrors `PREREG_PPUT_CCL_2026-04-26.md` (frozen) + `PREREG_AMENDMENT_p0_defer_2026-04-25.md` (delta). Per C-075 (DO-178C tool qualification) the gate machinery itself must be qualifiable; this history doc is in Trust Root so the chronology cannot be silently rewritten.
     6	
     7	**Why this doc exists** (A8e7 structural rewrite, 2026-04-26): rounds 1–6 of the Phase A → B audit produced a recurring documentary CHALLENGE pattern. Diagnosis: `A8_EXIT_PACKET` was conflating two opposite temporal modes (stable-snapshot artifact + append-only chronology) in one document, so every fix to "current state" generated new staleness in "round-N retrospective" sections. Splitting the two modes into two documents removes the category error.
     8	
     9	---
    10	
    11	## Round 1 (2026-04-26) — first dual review
    12	
    13	**Inputs**:
    14	- Packet: `handover/audits/A8_EXIT_PACKET_2026-04-26.md` @ commit `60292dc`
    15	- Test baseline: 261 PASS / 29 ignored / 0 failed
    16	- Trust Root: 30-entry manifest (round-1 packet's count; later corrected to 31 — see round-2)
    17	
    18	**Verdicts**:
    19	- Codex R1: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26.md`
    20	- Gemini R1: **VETO / high** — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26.md`
    21	- Merged (per memory `feedback_dual_audit_conflict`, VETO > CHALLENGE > PASS): **VETO**
    22	
    23	**Findings** (5 Codex + 4 Gemini; 1 Gemini Q1 finding about hybrid_v1 was determined invalid — pre-Phase A change):
    24	1. (Codex#2 + Gemini Q4 — convergent) `run_corr_id` (FC events) vs `make_pput::run_id` (jsonl) ms drift; Phase D consumers cannot join FC events to v2 rows by equality.
    25	2. (Codex#3) FC1-N12 emitted only in `run_oneshot`; the 2 `verify_omega_detailed` and 1 `verify_partial` calls in `run_swarm` increment `verifier_wait_ms` but don't emit FC events.
    26	3. (Codex#4 + Gemini Q5 — convergent) `detect_provider` routing matrix: `Qwen/Qwen2.5-7B-Instruct` misroutes to DashScope; smoke bypasses proxy so the bug isn't caught.
    27	4. (Codex#5) Trust Root manifest count off-by-1 (packet says 30, actual 31); genesis_payload.toml header still says "Total: 25 files".
    28	5. (Codex#1) `PREREG_AMENDMENT § 2` calls `p_0=0.10` substitution "strictest possible substitute" — backwards (0.10 is least-strict admissible for `j-RR ≤ p_0`).
    29	6. (Gemini Q4) Hand-rolled JSON encoder in `fc_trace.rs` vs `serde_json` already in deps — minor.
    30	7. (Gemini Q5) Smoke test bypasses proxy so routing logic untested — convergent with #3.
    31	8. (**Gemini VETO Q6**) Multi-key round-robin in `llm_proxy.py` (V3L-27 mitigation) lacks any automated conformance test. Manual verification via `[2,2,2]` distribution is one-time, not recurring. **REDESIGN-level for atom A7.**
    32	
    33	**Round-1 fixes shipped (`A8e`, commit `5a56ff6`)** — six fixes:
    34	- **F1** unified `run_id`: new `experiments/minif2f_v4/src/run_id.rs::mint_run_id` minted at run_swarm/run_oneshot entry; threaded into both `emit_event` and `make_pput`. Closes #1.
    35	- **F2** new `scripts/test_llm_proxy.py` 15-test suite pinning round-robin `[2,2,2]` invariant. Closes Gemini VETO Q6 at the artifact level.
    36	- **F3** `detect_provider` reordered: slash-form ⇒ siliconflow first; bare-qwen ⇒ dashscope only without slash. Closes #3.
    37	- **F4** added FC1-N12 emit to all 3 swarm verify sites (`verify_omega_detailed` × 2 + `verify_partial`). Closes #2.
    38	- **F5** Trust Root count + genesis_payload.toml header reconciled to 31 entries (corrected for A3's `agent_models.rs` already in TR before A1). Closes #4.
    39	- **F6** PREREG_AMENDMENT § 2 reworded: `p_0=0.10` → least-strict admissible ceiling; explicit Type-I implications paragraph added. Closes #5.
    40	
    41	---
    42	
    43	## Round 2 (2026-04-26) — post-A8e
    44	
    45	**Inputs**:
    46	- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `5a56ff6` (post-A8e)
    47	- Test baseline: 264 PASS / 29 ignored / 0 failed (Rust); 15/15 PASS (Python proxy tests)
    48	- Trust Root: 33-entry manifest
    49	
    50	**Verdicts**:
    51	- Codex R2: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R2.md`
    52	- Gemini R2: **CHALLENGE / high** (de-escalated from VETO) — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R2.md`
    53	- Merged: **CHALLENGE**. No VETO. Both auditors confirmed round-1 fixes are letter-correct; remaining gaps procedural/documentary.
    54	
    55	**Findings** (3 Codex + 2 Gemini, convergent):
    56	1. (Codex R2#1 + Gemini R2#1) F2's `test_llm_proxy.py` exists but is documented as manual-only; no CI integration. Tests-not-running-automatically = documentation, not gates. Closes Gemini's round-1 VETO at the *artifact* level but NOT at the *process* level.
    57	2. (Codex R2#2) PREREG_AMENDMENT § 2 was corrected (F6) but § 8 audit-requirements paragraph still says "strictest plausible bar is conservative" — direct contradiction with § 2.
    58	3. (Codex R2#3 + Gemini R2#2) Packet § 5 Risk #5 ("No FC1-N12 emit in run_swarm verify path") is stale (closed by F4); packet § 6 Q7.a/b numbers stale (261/30 vs actual 264/33); TRACE_MATRIX has stale `run_corr_id` symbol row + "in CI" claim.
    59	
    60	**Round-2 fixes shipped (`A8e2`, commit `0af47b7`)** — three fixes:
    61	- **G1** `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` Rust integration test that shells to `python3 scripts/test_llm_proxy.py`; runs on every `cargo test --workspace`. Closes finding #1.
    62	- **G2** PREREG_AMENDMENT § 8 reworded to remove "strictest plausible bar"; consistent with § 2 throughout. Closes finding #2.
    63	- **G3** Packet § 2 cumulative table extended; § 5 Risk #5 removed; § 6 Q7.a/b numbers updated 264/34; TRACE_MATRIX `run_corr_id` row replaced with unified `run_id` row; "in CI" softened. Closes finding #3.
    64	
    65	---
    66	
    67	## Round 3 (2026-04-26) — post-A8e2
    68	
    69	**Inputs**:
    70	- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `0af47b7` (post-A8e2)
    71	- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 15/15 PASS (Python)
    72	- Trust Root: 34-entry manifest
    73	
    74	**Verdicts**:
    75	- Codex R3: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R3.md`
    76	- Gemini R3: **CHALLENGE / high** — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R3.md`
    77	- Merged: **CHALLENGE**. Both auditors flagged narrow doc/source-comment defects; no VETO.
    78	
    79	**Findings**:
    80	1. (Codex R3#1) Packet line 118 still says substitution is "Mathematically conservative (strictest plausible bar)"; Q2.a still says "described as conservative"; genesis_payload.toml header still says "conservative ceiling". The § 2 / § 8 fix in F6/G2 didn't propagate to these other sites.
    81	2. (Codex R3#2) Packet § 3 A6 atom description says "6 anchor sites + 1 oneshot"; § 6 Q4.a says "FC1-N12 (oneshot verify only)"; § 6 Q4.d describes pre-F1 ms drift as if open. Stale relative to F4 + F1.
    82	3. (Codex R3#3) `tests/llm_proxy_python_conformance.rs` returns success when `python3` is missing (soft skip via `eprintln + return`). For a VETO-closing conformance gate, missing python3 should fail closed.
    83	4. (Gemini R3#1) Convergent with Codex R3#2 — Q4.d stale.
    84	5. (Gemini R3#2 — non-blocking observation) `make_pput` signature has 21 args; refactor to builder pattern recommended for Phase B+.
    85	
    86	**Round-3 fixes shipped (`A8e3`, commit `3d38ba5`)** — six fixes:
    87	- **H1** Packet § 3 A1 description rewritten: "Mathematically conservative" → "least-strict admissible value" with Type-I implications + cross-ref to § 2.
    88	- **H2** Packet § 3 A6 description bumped 6 → 9 anchor sites; explicitly listed F4-added swarm sites.
    89	- **H3** Packet § 6 Q2.a / Q4.a / Q4.d marked **CLOSED** with closure rationale.
    90	- **H4** `genesis_payload.toml` TR header phrasing about A1: "conservative ceiling" → "max-tolerated ceiling — least-strict admissible".
    91	- **H5** TRACE_MATRIX § 5 item 7: "(commit pending) + 6 wired" → "CLOSED + 9 wired".
    92	- **H6** G1 wrapper test now FAILS CLOSED on missing `python3`; explicit opt-out `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1` for deliberate downgrades.
    93	
    94	---
    95	
    96	## Round 4 (2026-04-26) — post-A8e3
    97	
    98	**Inputs**:
    99	- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `3d38ba5` (post-A8e3)
   100	- Test baseline: 265 PASS / 29 ignored / 0 failed; 15/15 Python
   101	- Trust Root: 34-entry manifest
   102	
   103	**Verdicts**:
   104	- Codex R4: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R4.md`
   105	- Gemini R4: **PASS / high** — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R4.md`. *First round one auditor reached PASS.*
   106	- Merged: **CHALLENGE**. Conservative merge wins per memory.
   107	
   108	**Findings** (3 Codex + 1 Gemini non-blocking):
   109	1. (Codex R4#1) Packet title + date metadata still say "round 2"; per-atom Trust Root deltas in § 3 stale (A5 says 25→26 vs actual 26→27, A6 26→27 vs 27→28, A7 27→30 vs 28→31).
   110	2. (Codex R4#2 + R4#3) TRACE_MATRIX § 1 line 11 says "Six anchor sites wired in run_swarm" but 1 site is in run_oneshot.
   111	3. (Codex R4#3) `tests/llm_proxy_python_conformance.rs` file header docstring still says "if not, it skips with a clear diagnostic" — contradicts H6's fail-closed runtime.
   112	4. (Gemini R4 non-blocking) `make_pput` arg count is now 24 not 21 (F1 added run_id parameter). Reaffirms the round-3 deferred refactor.
   113	
   114	**Round-4 fixes shipped (`A8e4`, commit `8693789`)** — three fixes:
   115	- **I1** Packet metadata "round 2" → "running through rounds 1–N" with reader pointer to latest section.
   116	- **I2** Per-atom Trust Root deltas in packet § 3 corrected: A5 → 26→27, A6 → 27→28, A7 → 28→31. TRACE_MATRIX § 1 line 11 anchor-site location corrected: "5 in run_swarm + 1 in run_oneshot".
   117	- **I3** Wrapper docstring updated: "FAILS CLOSED with a clear diagnostic"; explicit opt-out env var documented.
   118	
   119	---
   120	
   121	## Round 5 (2026-04-26) — post-A8e4
   122	
   123	**Inputs**:
   124	- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `8693789` (post-A8e4)
   125	- Test baseline: 265 PASS / 29 ignored / 0 failed; 15/15 Python
   126	- Trust Root: 34-entry manifest
   127	
   128	**Verdicts**:
   129	- Codex R5: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R5.md`
   130	- Gemini R5: **PASS / high** — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R5.md`
   131	- Merged: **CHALLENGE**. Same split as R4.
   132	
   133	**Findings** (2 Codex + 1 Gemini non-blocking):
   134	1. (Codex R5#1) Packet missing Round-4 outcome + A8e4 fixes shipped section; `<pending>` commit placeholders for A8e2/A8e3/A8e4 never replaced.
   135	2. (Codex R5#2) Packet § 6 Q6 round-1 question text still says "24 → 30 / 6 new entries" (pre-F5 count); TRACE_MATRIX top-bullet TR-deltas A5 "25→26" / A6 "26→27" still stale (only the per-atom paragraphs were corrected in I2, not the bullet headers).
   136	3. (Gemini R5 non-blocking) `make_pput` arg count text in round-3 retrospective still says "21 positional args"; should be 24.
   137	
   138	**Round-5 fixes shipped (`A8e5`, commit `1622017`)** — five fixes:
   139	- **J1** Round-4 outcome + A8e4 fixes shipped section added to packet.
   140	- **J2** `<pending>` placeholders replaced with actual SHAs (`0af47b7` / `3d38ba5` / `8693789`); A8e5 self-reference uses "this commit".
   141	- **J3** Packet § 6 Q6 question text reworded to "24 → 34 / 10 new entries" with full enumerated list.
   142	- **J4** TRACE_MATRIX A5/A6/A7 top-bullet TR-deltas corrected (matches canonical chain in genesis_payload.toml header).
   143	- **J5** Round-3 retrospective `make_pput` arg count: 21 → 24.
   144	
   145	---
   146	
   147	## Round 6 (2026-04-26) — post-A8e5
   148	
   149	**Inputs**:
   150	- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `1622017` (post-A8e5)
   151	- Test baseline: 265 PASS / 29 ignored / 0 failed; 15/15 Python
   152	- Trust Root: 34-entry manifest
   153	
   154	**Verdicts**:
   155	- Codex R6: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R6.md`
   156	- Gemini R6: **CHALLENGE / high** — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R6.md`
   157	- Merged: **CHALLENGE**. Convergence regression vs R4/R5 (Gemini went from PASS back to CHALLENGE).
   158	
   159	**Findings** (3 Codex + 2 Gemini; ONE real correctness bug):
   160	1. (Codex R6#1) RQ14 closure criterion contradicts itself — packet has `<pending>` placeholders in J2 bullet + RQ14 itself.
   161	2. (**Codex R6#2 — REAL BUG**) `detect_provider` misroutes HuggingFace-style `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B` to `api.deepseek.com` because `"deepseek" in m` wins before the slash check. The official DeepSeek API only serves bare `deepseek-chat` / `deepseek-v4-flash`, not Distill variants — would 404 on first invocation.
   162	3. (Codex R6#3) TRACE_MATRIX A8e3 row still says "21 positional args" while packet was corrected to 24 in J5.
   163	4. (Gemini R6#1) TRACE_MATRIX A6 line leads with "Six anchor sites wired" then immediately self-contradicts with "9 post-A8e total".
   164	5. (Gemini R6#2) Packet § 3 A5/A6 fix attribution: "A8e3 fix H4 corrected" — H4 corrected the genesis_payload.toml comment; the packet § 3 numbers were corrected by I2 (A8e4).
   165	
   166	**Round-6 fixes shipped (`A8e6`, commit `dbcf53a`)** — five fixes (1 real correctness + 4 doc):
   167	- **K1** `<pending>` placeholder closure: J2 bullet + RQ14 reworded.
   168	- **K2 (real bug)** `src/drivers/llm_proxy.py::detect_provider` reordered: slash-form is now FIRST routing heuristic. New `test_deepseek_slash_form_routes_to_siliconflow` in `scripts/test_llm_proxy.py` pins the regression. **16/16 Python proxy tests PASS** (was 15).
   169	- **K3** TRACE_MATRIX A8e3 row: "21 positional args" → "was 21 at round-3; F1 added run_id, post-A8e count is 24".
   170	- **K4** TRACE_MATRIX A6 row top-line: "Six anchor sites wired" → "9 anchor sites wired (8 in run_swarm + 1 in run_oneshot)" with original-6 + F4-added-3 breakdown.
   171	- **K5** Packet § 3 A5/A6 fix attribution: "A8e3 fix H4 corrected" → "A8e4 fix I2 corrected".
   172	
   173	---
   174	
   175	## Round 7 — A8e7 structural rewrite + relaunch
   176	
   177	**Trigger**: rounds 2–6 produced a recurring documentary CHALLENGE pattern that single-point fixes could not close. **Diagnosis**: `A8_EXIT_PACKET` was conflating stable-snapshot artifact + append-only chronology in one document. **Fix** (commit `<this commit's SHA>` — A8e7): split into two docs following the project's existing constitutional pattern (stable spec + delta log; cf. constitution.md + Art. V.3 amendment log; PREREG + PREREG_AMENDMENT; TRACE_MATRIX_v0 + v1 + v2). The packet becomes current-state only; THIS history doc becomes append-only chronology.
   178	
   179	**Round 7 dual audit pending after this commit lands.**
   180	
   181	---
   182	
   183	## Cumulative metrics
   184	
   185	| Round | Codex | Gemini | Merged | New findings | Real-bug findings | API cost (~$) |
   186	|---|---|---|---|---|---|---|
   187	| 1 | CHALLENGE | **VETO** | VETO | 9 | 5 | ~5 |
   188	| 2 | CHALLENGE | CHALLENGE | CHALLENGE | 5 | 0 | ~5 |
   189	| 3 | CHALLENGE | CHALLENGE | CHALLENGE | 5 (incl. 1 non-blocking) | 1 (H6 fail-closed) | ~5 |
   190	| 4 | CHALLENGE | PASS | CHALLENGE | 4 (incl. 1 non-blocking) | 0 | ~5 |
   191	| 5 | CHALLENGE | PASS | CHALLENGE | 3 (incl. 1 non-blocking) | 0 | ~5 |
   192	| 6 | CHALLENGE | CHALLENGE | CHALLENGE | 5 | **1 (K2 routing)** | ~5 |
   193	| 7 | pending | pending | pending | — | — | ~5 |
   194	
   195	Cumulative cost so far ~$30; well within memory `feedback_dual_audit` Phase A reservation. Real-bug yield: 6 substantive findings caught + closed (5 in R1, 1 in R3, 1 in R6 = 7 real bugs discovered + fixed pre-Phase B). Remaining CHALLENGE findings post-A8e6 are pure documentary, traceable to the category error A8e7 now addresses.

exec
/bin/bash -lc "nl -ba genesis_payload.toml | sed -n '1,140p'" in /home/zephryj/projects/turingosv4
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
    64	#       max-tolerated ceiling — least-strict admissible per amendment
    65	#       § 2 wording correction A8e fix F6; per case C-073 ArchitectAI
    66	#       commit authority)
    67	#
    68	# Total: 35 files. Progression A0=24 → A1=25 (PREREG amendment) → A3=26
    69	# (agent_models.rs) → A5=27 (budget_regime.rs) → A6=28 (fc_trace.rs) →
    70	# A7=31 (llm_proxy.py + 2 smoke scripts) → A8e=33 (run_id.rs +
    71	# test_llm_proxy.py) → A8e2=34 (llm_proxy_python_conformance.rs Rust
    72	# wrapper) → A8e7=35 (A8_AUDIT_HISTORY append-only chronology — per
    73	# C-075 the gate machinery itself must be tamper-evident).
    74	# genesis_payload.toml itself is conceptually frozen but
    75	# not self-hashed (chicken-and-egg) — the [pput_accounting_0] section
    76	# values are the semantic anchor.
    77	#
    78	# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
    79	
    80	[pput_accounting_0]
    81	schema_version = "1.0"
    82	progress_definition = "1 iff Lean ground-truth verifies golden_path_payload"
    83	cost_definition = "sum(prompt_tokens + completion_tokens + tool_tokens) over all proposals in the run"
    84	time_definition = "wall_clock from first agent prompt construction to final Lean accept or external timeout"
    85	verified_predicate = "experiments/minif2f_v4/src/lean4_oracle.rs::verify_omega_detailed"
    86	heldout_sealed_hash = "51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b"
    87	source_pool_sha256 = "77179cf2598b0df707d78a6663e763121dfe8e73a6538073155f488feab95093"
    88	baseline_regression_rate = 0.0  # PLACEHOLDER until Phase B7-extra calibration
    89	baseline_regression_jsonl_sha256 = ""  # PLACEHOLDER until Phase B7-extra
    90	k_max = 10
    91	n_max = 34
    92	
    93	[trust_root]
    94	"src/main.rs" = "622fee2d96a980d24f9fbaab3d0531c195a0a337fc3ddd2efb60bca90a1cfbf9"
    95	"Cargo.lock" = "577446e8fe11e91bc8751bf13e5ddca6c5faa64d3309b878768c550d3e6feb98"
    96	"handover/preregistration/scripts/run_p0_calibration.sh" = "5f4a57dd8b8280ffe04bec89350a57d876d06cc179d9f8841a522e7bdcf1b8b7"
    97	"handover/preregistration/scripts/compute_p0.py" = "fd1dff97d582f4f509249852be500cba7d81efc9bab8fcb9872b2d85bb4e3052"
    98	"rules/MANIFEST.sha256" = "a84d114a12680c596e1a5458a954a5829b21baa4530f197b9aba65f95443be14"
    99	"rules/engine.py" = "932d9a2b7a3249a7eb5825c0b5c714a9913cd9aa9e058f789e64992b140e40b3"
   100	".claude/hooks/judge.sh" = "a2be9e6ed51e39f2e9cfd302d62a0234a772abc41f145702143d2557dd6fda3e"
   101	"tests/fc_alignment_conformance.rs" = "b3e75979ad2d175b9c45135be6ea1d94ce95184c6896468330c12dbfc1f719db"
   102	"src/kernel.rs" = "893fd67534caf7a3d9abd6efbd202556348b6491cd6d4c6bdb224d2ad75b1af0"
   103	"src/wal.rs" = "1ac7637aa09531b1c7232163f5df48f7193251594c4ed20e0d0fc85cea8f84dc"
   104	"src/bus.rs" = "df28ffe514a3272a3d10fca4568fd424a76e754e9785c109a5459f163f7fd14c"
   105	"src/drivers/llm_http.rs" = "615596b68956b4a8925110edc99aa746a5543527724ec394bb9dda163147ed7a"
   106	"src/drivers/llm_proxy.py" = "22f632234a6d3e1a0547651537cf26846c64c47b85f5b1c3a1bcd1ae1f139997"
   107	"scripts/smoke_siliconflow.sh" = "6ad54e7c0ab8221f475360dcad80eeeb0d6da0fd55c27acdb1cefb2b390f5341"
   108	"scripts/_smoke_siliconflow.py" = "778eea2988312f250efa47fcfe620d86187d01b96f07a98501f9795a333726ca"
   109	"scripts/test_llm_proxy.py" = "e1dd0306609fb3d2578b078bb21f0090c23978981f674ca0c6e3a60485ebf273"
   110	"src/sdk/prompt_guard.rs" = "b4f7b164770d1a7203b8143f773c66f748994d60a42ece38471f2f7f2839f4f1"
   111	"experiments/minif2f_v4/src/lean4_oracle.rs" = "70fae24cd17f410c10a092e797fcdedea962db3d7cb20f218d02303edae9e98c"
   112	"experiments/minif2f_v4/src/cost_aggregator.rs" = "896b6905dbca9e9736f8896cd5725c16b6e87c6ad3ff822e044975febed46a03"
   113	"experiments/minif2f_v4/src/wall_clock.rs" = "2c9197f8f93b7d130dc7b094a6664f8ece351ec85668921c5beb6d100a1a77ee"
   114	"experiments/minif2f_v4/src/post_hoc_verifier.rs" = "9a93ae5548827b60543df779c67a5f1201b49a681a55e2cbc6a8532e8bb131a2"
   115	"experiments/minif2f_v4/src/jsonl_schema.rs" = "5d0c5a2bcceba42abbc6f5cd58e045e2d5d7a9edc565db4515f3917a21dc520b"
   116	"experiments/minif2f_v4/src/rollback_sim.rs" = "9ccdf177813452bb83221bd780f2f7f10da619117aca8e44d3ecb355505fe561"
   117	"experiments/minif2f_v4/src/agent_models.rs" = "b736b91efb8a93d884b64028da79d5d6af0525e8c7e8f7222055118f43dc26ce"
   118	"experiments/minif2f_v4/src/budget_regime.rs" = "e0c2ba6533444e6178d9e12d0f182d29c3203a929ea0914d3098efd0397948ea"
   119	"experiments/minif2f_v4/src/fc_trace.rs" = "8578edf61148e50abdf8d29a2efdc5114ee20ab727c1ccd6c5a88ad07e7ffa09"
   120	"experiments/minif2f_v4/src/run_id.rs" = "5ff9db039131eaf4efeb65303d886265ef4c4f7e0e0f743467a7cbf22061781c"
   121	"experiments/minif2f_v4/src/bin/evaluator.rs" = "290b2f0be3edc43b7f439506bf650743cd268a0b76ddd489fb3e5cc1da9a24ba"
   122	"constitution.md" = "91e5913603eebaf134ac93d0f9ed7bb2b7916132203f6c2fae84d73329056ad6"
   123	"cases/MANIFEST.sha256" = "dbe48bc11cac18ad1164c55b11aefd5be4a7b546e31c3df08fb4403c7f41e90d"
   124	"handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json" = "69be702e3e0a4bb33c62bd02056495309289fd9271972dc733b4a1ba3dd908e1"
   125	"handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" = "64d565b4451d1f770425530bb7ffa874da602ddfd9d714ce7f7db6bbfab69463"
   126	"handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md" = "820645d2a5cc8aaeda3d68b923a874934206bb1c0ae7b4356e94148b90c28b4c"
   127	"experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs" = "d62ee67cb564355fa26959c1f92a1152f95ab55d987c005abf7810e2da7b83b8"
   128	"handover/audits/A8_AUDIT_HISTORY_2026-04-26.md" = "bf1e22a47a8fc312872ea382bb339fb5ce2849fe781e240d7b309059fecede5f"

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
   117	        // A8e fix F1: unified run_id minted once per run (was run_corr_id ms drift)
   118	        "experiments/minif2f_v4/src/run_id.rs",
   119	        // A8e fix F2/F3: routing matrix + round-robin Python conformance tests
   120	        "scripts/test_llm_proxy.py",
   121	        // A8e2 fix G1: Rust wrapper that runs the Python suite on every cargo test
   122	        "experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs",
   123	        // A8e7: append-only audit history (companion to A8_EXIT_PACKET; per C-075
   124	        // gate machinery is constitutional — tampering with the chronology = silent
   125	        // governance drift).
   126	        "handover/audits/A8_AUDIT_HISTORY_2026-04-26.md",
   127	        // 2026-04-25 dual-audit fixes
   128	        "src/main.rs",
   129	        "Cargo.lock",
   130	        "handover/preregistration/scripts/run_p0_calibration.sh",
   131	        "handover/preregistration/scripts/compute_p0.py",
   132	        // 2026-04-25 Phase A0 harness modernization
   133	        "rules/MANIFEST.sha256",
   134	        "rules/engine.py",
   135	        ".claude/hooks/judge.sh",
   136	        "tests/fc_alignment_conformance.rs",
   137	        // 2026-04-25 Phase A1 PREREG amendment
   138	        "handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md",
   139	    ];
   140	
   141	    for path in required {
   142	        assert!(
   143	            keys.contains(&path),
   144	            "Trust Root manifest missing required path: {path}\nactual keys: {keys:#?}"
   145	        );
   146	    }
   147	}
   148	
   149	#[test]
   150	fn test_pput_accounting_0_section_present() {
   151	    // genesis_payload.toml must contain [pput_accounting_0] with the PREREG
   152	    // § 1.8 keys.
   153	    let genesis = read_genesis();
   154	    let body = extract_section(&genesis, "pput_accounting_0").expect("section present");
   155	    let body = body.as_str();
   156	
   157	    let required_keys = [
   158	        "schema_version",
   159	        "progress_definition",
   160	        "cost_definition",
   161	        "time_definition",
   162	        "verified_predicate",
   163	        "heldout_sealed_hash",
   164	        "source_pool_sha256",
   165	        "baseline_regression_rate",
   166	        "baseline_regression_jsonl_sha256",
   167	        "k_max",
   168	        "n_max",
   169	    ];
   170	    for key in required_keys {
   171	        let needle = format!("{key} =");
   172	        assert!(
   173	            body.contains(&needle),
   174	            "[pput_accounting_0] missing key: {key}"
   175	        );
   176	    }
   177	
   178	    // Frozen invariants from PREREG § 1.8: heldout sealed hash, k_max, n_max.
   179	    assert!(body.contains(
   180	        "\"51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b\""

exec
/bin/bash -lc "nl -ba handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
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
    25	**`p_0` for guardrail purposes**: take the **PREREG § 5.5 ceiling itself = 0.10** and use it as `p_0` directly. Any artifact j whose `j-RR` regression rate exceeds 0.10 fails Gate H per the original guardrail logic.
    26	
    27	**Wording correction (A8 round-1 audit, Codex finding 1)**: an earlier draft of this section claimed `p_0 = 0.10` was the *strictest possible substitute*. That was backwards. The Gate H test is `j-RR ≤ p_0`, so a SMALLER `p_0` is stricter. `0.10` is the **maximum tolerated value** the PREREG admits — i.e., the LEAST strict admissible ceiling, not the strictest. Substituting it is the equivalent of running Gate H at the weakest threshold the PREREG ever sanctioned. The substitution is *operationally permitted* (the PREREG explicitly allows up to 0.10), but it is NOT a tighter-than-original guarantee.
    28	
    29	**Statistical implications** (re-stated cleanly):
    30	- No Type-I inflation. `j-RR` is a descriptive guardrail (PREREG § 5.4), not part of the inferential family — its threshold doesn't enter Holm-Bonferroni adjustment.
    31	- The substitution may be LESS protective than an eventual calibrated `p_0 < 0.10`. When calibration runs (per § 3 conditions), the empirical estimate likely tightens the bar; until then we operate at the ceiling.
    32	- Acceptable trade-off because (a) Phase B and C don't gate on `j-RR` (Gate H is a Phase E artifact gate), and (b) the conditions in § 3 ensure calibration runs *before* Phase E, so the loose substitution never reaches the artifact-acceptance moment.
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
    95	- Does substitution of `p_0 = 0.10` invalidate any Gate H statistical claim? (Should not — `j-RR` is descriptive (PREREG § 5.4), outside the inferential family, so no Type-I inflation. Per § 2 wording correction: 0.10 is the LEAST-strict admissible ceiling, NOT a tighter-than-original substitute — the substitution may be less protective than an eventual calibrated `p_0 < 0.10`, accepted because Gate H is Phase E and § 3 conditions ensure calibration runs first.)
    96	- Does deferral leave any phase blocked indefinitely? (Should not — § 3 lists explicit re-calibration conditions; if those never met, Phase E proceeds with the operationally-permitted ceiling substitution per § 2 final paragraph.)
    97	
    98	## § 9. Cross-references
    99	
   100	- `PREREG_PPUT_CCL_2026-04-26.md` § 5.5 (the amended section, IMMUTABLE)
   101	- `cases/C-073_architect_ai_commit_authority.yaml` (governance basis)
   102	- `cases/C-075_do_178c_tool_qualification.yaml` (tool-readiness as re-calibration precondition)
   103	- `handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md` (context: cost asymmetry concern)
   104	- `handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md` (context: ground-truth feedback discipline)
   105	- `handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md` (PASS verdict on round-4 batch — but batch was 3-7 days not 8h, motivating this deferral)

exec
/bin/bash -lc 'rg -n "conservative|strictest|strict" handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md handover/audits/A8_EXIT_PACKET_2026-04-26.md genesis_payload.toml handover/alignment/TRACE_MATRIX_v2_2026-04-25.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:16:- A8e4 (post-A8e3, FIX-THEN-PROCEED after A8 round-4 audit Codex CHALLENGE / Gemini PASS — split verdict, conservative merge wins per memory `feedback_dual_audit_conflict`): three narrow documentary defects Codex found in the packet itself. **I1** packet title + date: "round 2" → "running through rounds 1–N" (cumulative, with reader pointer to most-recent round section). **I2** per-atom Trust Root deltas in § 3: A5 25→26 → 26→27, A6 26→27 → 27→28, A7 27→30 → 28→31 (all corrected to match the chain in `genesis_payload.toml` header — round-1 packet undercounted because A3's `agent_models.rs` had already raised the count to 26 before A5 landed); TRACE_MATRIX § 1 line 11 anchor-site location: "Six anchor sites wired in `run_swarm`" → "Six anchor sites wired (5 in `run_swarm` + 1 in `run_oneshot`)" — same in § 5 item 7 closure note. **I3** `tests/llm_proxy_python_conformance.rs` file header docstring: "if not, it skips with a clear diagnostic" → "if not, it FAILS CLOSED with a clear diagnostic" (A8e3 H6 fixed the runtime but left the docstring stale; Codex R4#3 caught the contradiction). Trust Root manifest unchanged at 34 (only re-hash of the wrapper). No source-code defects, no architectural changes; this round is purely documentary alignment.
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:17:- A8e3 (post-A8e2, FIX-THEN-PROCEED after A8 round-3 audit Codex CHALLENGE / Gemini CHALLENGE — both convergent on packet staleness; no VETO): six narrow cleanup fixes. **H1** A8 packet § 3 A1 description: "Mathematically conservative" → "least-strict admissible value" with explicit Type-I implications. **H2** packet § 3 A6 description: 6 → 9 anchor sites (added F4 swarm sites explicitly). **H3** packet § 6 Q2.a / Q4.a / Q4.d marked CLOSED with closure rationale. **H4** `genesis_payload.toml` TR header phrasing about A1: "conservative ceiling" → "max-tolerated ceiling". **H5** § 5 item 7 here in TRACE_MATRIX: "(commit pending) + 6 wired" → "CLOSED + 9 wired". **H6** G1 wrapper test fail-closed on missing `python3` (was soft skip; Codex R3 caught the silent-pass risk); explicit opt-out `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1` for deliberate downgrades (logged loudly). Trust Root manifest unchanged at 34 (only re-hashes; no new files). Non-blocking observation deferred to Phase B+: `make_pput` signature was 21 positional args at round-3; A8e fix F1 added the `run_id` parameter, bringing the post-A8e count to **24** (Gemini R3 Finding 2 + Gemini R5/Codex R6 spot-checks; A8e6 K3 corrected here) — refactor to a builder pattern is recommended but not gating.
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:18:- A8e2 (post-A8e, FIX-THEN-PROCEED after A8 round-2 audit Codex CHALLENGE / Gemini CHALLENGE — both convergent): three dual-audit fixes. **G1** (Codex R2#1 + Gemini R2#1) `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` wraps `python3 scripts/test_llm_proxy.py` in a Rust integration test, making the V3L-27 round-robin conformance gate run on every `cargo test --workspace`. **G2** (Codex R2#2) `PREREG_AMENDMENT § 8` audit-requirements paragraph reworded — round-1 fix corrected § 2 but left § 8 still calling the substitution a "strictest plausible bar"; now consistent across the doc. Re-hashed in Trust Root. **G3** (Codex R2#3 + Gemini R2#2) `A8_EXIT_PACKET` updated for round-2 reality: § 2 cumulative-tests table extended with A8e/A8e2 columns, § 4 manifest-clean assertion bumped 30→34, § 5 round-1 risk #5 (no FC1-N12 in swarm verify path) removed (closed by F4), § 6 Q7.a/b numbers updated, round-2 outcome + round-3 fixes section appended; TRACE_MATRIX stale `run_corr_id` symbol row replaced; "in CI" softened to "via Rust wrapper post-A8e2". FC-trace: meta governance + V3L-27 mitigation gating (FC1-N7). Trust Root manifest 33 → 34 (`tests/llm_proxy_python_conformance.rs`).
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:19:- A8e (post-A7, FIX-THEN-PROCEED after A8 round-1 audit Codex CHALLENGE / Gemini VETO): six dual-audit fixes. **F1** (Codex#2 + Gemini Q4) `experiments/minif2f_v4/src/run_id.rs` mints one stable run_id per run, threaded into both `emit_event` and `make_pput` (replaces `run_corr_id` vs internal `run_id` ms drift; oneshot also stops using `oneshot_{problem_file}` placeholder). **F2** (Gemini VETO Q6) `scripts/test_llm_proxy.py` adds 15-test conformance battery — round-robin distributes [2,2,2] across 6 calls, single-key pool degrades to all-zero, two-key pool alternates [2,2]; exercises the V3L-27 mitigation without a live API. (A8e2 G1 wires this into `cargo test --workspace` via a Rust shell-out wrapper — see § 2 below.) **F3** (Codex#4) routing matrix bug fixed — `Qwen/Qwen2.5-7B-Instruct` was misrouting to DashScope because `m.startswith("qwen")` won after the slash check; now slash-form ⇒ siliconflow first, bare-qwen ⇒ dashscope second. **F4** (Codex#3) FC1-N12 emit added to all 3 swarm verify sites (`verify_omega_detailed` × 2 + `verify_partial`), each with verdict + elapsed_ms + path stamp; closes the swarm-side gap in the 5-step compile-loop witness. **F5** (Codex#5) `genesis_payload.toml` header + TRACE_MATRIX milestones reconciled to actual 31 entries (round-1 packet undercounted by 1 — agent_models.rs from A3 was already in TR before A1). **F6** (Codex#1) `PREREG_AMENDMENT § 2` wording corrected — `p_0=0.10` substitution is the LEAST-strict admissible ceiling (max tolerated value for `j-RR ≤ p_0`), NOT the strictest possible substitute as round-1 mistakenly claimed; explicit statistical-implications paragraph added (no Type-I inflation since `j-RR` is descriptive; substitution may be less protective than eventual calibrated `p_0 < 0.10`). FC-trace: FC1-N12 (oracle scope swarm coverage) + FC1-N7 (δ provider routing) + Art. V (amendment governance). Trust Root manifest 31 → 33 (run_id.rs + test_llm_proxy.py).
genesis_payload.toml:64:#       max-tolerated ceiling — least-strict admissible per amendment
handover/audits/A8_EXIT_PACKET_2026-04-26.md:67:- **Statistical framing** (post-A8e F6 + A8e2 G2 wording corrections): `0.10` is the **least-strict admissible value** the PREREG ceiling allows; `j-RR ≤ p_0` makes a SMALLER `p_0` stricter, so this substitution is operationally permitted but NOT a tighter-than-original guarantee. No Type-I inflation since `j-RR` is descriptive (PREREG § 5.4), outside the inferential family. May be less protective than an eventual calibrated `p_0 < 0.10`; acceptable because Gate H is Phase E and § 3 conditions ensure calibration runs first.
handover/audits/A8_EXIT_PACKET_2026-04-26.md:146:- (Q2.a) `p_0 = 0.10` substitution: is the wording in `PREREG_AMENDMENT_p0_defer_2026-04-25.md` § 2 + § 8 consistent + statistically correct (least-strict admissible, not "conservative" / "strictest")? Verify by full-text grep for "conservative" / "strictest" — the only acceptable matches are explicit retrospective notes.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:27:**Wording correction (A8 round-1 audit, Codex finding 1)**: an earlier draft of this section claimed `p_0 = 0.10` was the *strictest possible substitute*. That was backwards. The Gate H test is `j-RR ≤ p_0`, so a SMALLER `p_0` is stricter. `0.10` is the **maximum tolerated value** the PREREG admits — i.e., the LEAST strict admissible ceiling, not the strictest. Substituting it is the equivalent of running Gate H at the weakest threshold the PREREG ever sanctioned. The substitution is *operationally permitted* (the PREREG explicitly allows up to 0.10), but it is NOT a tighter-than-original guarantee.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:92:Per case C-073 ArchitectAI commit workflow: this amendment requires dual audit (Codex + Gemini, conservative VETO > CHALLENGE > PASS) before commit lands. Audit packet should specifically test:
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:95:- Does substitution of `p_0 = 0.10` invalidate any Gate H statistical claim? (Should not — `j-RR` is descriptive (PREREG § 5.4), outside the inferential family, so no Type-I inflation. Per § 2 wording correction: 0.10 is the LEAST-strict admissible ceiling, NOT a tighter-than-original substitute — the substitution may be less protective than an eventual calibrated `p_0 < 0.10`, accepted because Gate H is Phase E and § 3 conditions ensure calibration runs first.)

exec
/bin/bash -lc 'rg -n "deferral|Phase E|conditions|calibration runs first|proceeds" handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md handover/audits/A8_EXIT_PACKET_2026-04-26.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/audits/A8_EXIT_PACKET_2026-04-26.md:20:- **A1**: PREREG amendment p_0 calibration deferral.
handover/audits/A8_EXIT_PACKET_2026-04-26.md:67:- **Statistical framing** (post-A8e F6 + A8e2 G2 wording corrections): `0.10` is the **least-strict admissible value** the PREREG ceiling allows; `j-RR ≤ p_0` makes a SMALLER `p_0` stricter, so this substitution is operationally permitted but NOT a tighter-than-original guarantee. No Type-I inflation since `j-RR` is descriptive (PREREG § 5.4), outside the inferential family. May be less protective than an eventual calibrated `p_0 < 0.10`; acceptable because Gate H is Phase E and § 3 conditions ensure calibration runs first.
handover/audits/A8_EXIT_PACKET_2026-04-26.md:147:- (Q2.b) Re-calibration conditions in § 3 list 5 items. Does coupling Phase B → C to Phase D readiness create any forward-progress hazard?
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:23:PREREG § 5.5 calibration **DEFERRED** indefinitely with the following operative substitution for Phase B → C transition and Phase E Gate H requirements:
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:31:- The substitution may be LESS protective than an eventual calibrated `p_0 < 0.10`. When calibration runs (per § 3 conditions), the empirical estimate likely tightens the bar; until then we operate at the ceiling.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:32:- Acceptable trade-off because (a) Phase B and C don't gate on `j-RR` (Gate H is a Phase E artifact gate), and (b) the conditions in § 3 ensure calibration runs *before* Phase E, so the loose substitution never reaches the artifact-acceptance moment.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:40:Calibration becomes worthwhile (and the deferral lifted) when ALL of:
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:50:5. **Phase D ArchitectAI runtime exists** — calibration is part of Gate H gating Phase E. Doing it before Phase D = calibrating against a counterfactual ArchitectAI that doesn't exist.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:52:When ALL 5 conditions met: re-issue PREREG_AMENDMENT to lift the deferral + trigger the 576-run batch with the (then-current) production mode.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:56:PREREG_PPUT_CCL_2026-04-26 § 5.5 originally listed p_0 calibration as a Phase B prerequisite ("Schedule: Phase B7 mandatory; not deferrable to Phase D"). This amendment **explicitly OVERRIDES that "not deferrable" clause** for the deferral conditions in § 3 above.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:65:Phase B → C dual-audit packet (next major milestone) must reference this amendment + show that Phase E Gate H consumer hardcodes `p_0 = 0.10`.
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:95:- Does substitution of `p_0 = 0.10` invalidate any Gate H statistical claim? (Should not — `j-RR` is descriptive (PREREG § 5.4), outside the inferential family, so no Type-I inflation. Per § 2 wording correction: 0.10 is the LEAST-strict admissible ceiling, NOT a tighter-than-original substitute — the substitution may be less protective than an eventual calibrated `p_0 < 0.10`, accepted because Gate H is Phase E and § 3 conditions ensure calibration runs first.)
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:96:- Does deferral leave any phase blocked indefinitely? (Should not — § 3 lists explicit re-calibration conditions; if those never met, Phase E proceeds with the operationally-permitted ceiling substitution per § 2 final paragraph.)
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:105:- `handover/audits/CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md` (PASS verdict on round-4 batch — but batch was 3-7 days not 8h, motivating this deferral)

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '240,430p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   240	    let problem_path = resolve_problem_path(problem_file, &minif2f_dir);
   241	    let (problem_statement, theorem_name) = match load_problem(&problem_path) {
   242	        Ok(v) => v,
   243	        Err(e) => { eprintln!("Failed to load: {}", e); std::process::exit(1); }
   244	    };
   245	
   246	    let lean_path = derive_lean_path(&minif2f_dir);
   247	    info!("Problem: {} | Condition: {} | Model: {}", problem_file, condition, model);
   248	
   249	    let result = match condition.as_str() {
   250	        "oneshot" => {
   251	            run_oneshot(problem_file, &problem_statement, &theorem_name,
   252	                       &lean_path, &proxy_url, &model).await
   253	        }
   254	        // Generic nN: parse any "n<digits>" → run_swarm with N agents.
   255	        // Supports N-scaling experiment (percolation curve mapping).
   256	        // **swarm_N=1** (CONDITION=n1) is the critical baseline for the
   257	        // 2026-04-25 N-experiments arc: same code path as n3/n8 swarm
   258	        // but with a single agent. NOT the same as `oneshot` (which
   259	        // skips the swarm loop, tape, mr ticks, ∏p product, etc.).
   260	        // Per Plan-agent NEXT-1 / Codex E0 / Gemini E1-Prime: every
   261	        // N-curve experiment MUST use n1 (not oneshot) as the N=1
   262	        // baseline to avoid code-path confound. Validated by unit
   263	        // test below: parse_swarm_condition_n("n1") == Some(1).
   264	        c if parse_swarm_condition_n(c).is_some() => {
   265	            let n = parse_swarm_condition_n(c).unwrap();
   266	            run_swarm(problem_file, &problem_statement, &theorem_name,
   267	                     &lean_path, &proxy_url, &model, n).await
   268	        }
   269	        "hybrid_v1" => {
   270	            // Mid-term audit P0-D fix 2026-04-25: hybrid_v1 was a Paper 1 era
   271	            // condition that ran run_oneshot, then on failure ran run_swarm,
   272	            // and merged via `..r2` field-spread. Codex flagged that the spread
   273	            // dropped the failed oneshot's C_i (failed_branch_count and
   274	            // total_run_token_count from r1 were silently discarded). PPUT-CCL
   275	            // arc does NOT use hybrid_v1 — it operates exclusively on `oneshot`
   276	            // and `n<N>` conditions per PREREG. Disabling here forces any
   277	            // pipeline that ships a stale hybrid_v1 invocation to surface the
   278	            // deprecation immediately rather than emit a corrupt C_i.
   279	            eprintln!("hybrid_v1 condition is deprecated for PPUT-CCL arc and was \
   280	                       disabled in mid-term audit P0-D fix 2026-04-25. The prior \
   281	                       implementation dropped the failed oneshot leg's C_i via a \
   282	                       `..r2` field-spread, corrupting full-run cost accounting. \
   283	                       Use `oneshot` or `n<N>` instead.");
   284	            std::process::exit(1);
   285	        }
   286	        other => { eprintln!("Unknown condition: {}", other); std::process::exit(1); }
   287	    };
   288	
   289	    // Output PPUT result as JSON (machine-readable for batch runner)
   290	    let json = serde_json::to_string(&result).unwrap();
   291	    println!("PPUT_RESULT:{}", json);
   292	
   293	    if result.has_golden_path {
   294	        info!("PPUT = {:.2}%/s (GP: {} nodes, {} tokens, {:.1}s)",
   295	              result.pput, result.gp_node_count, result.gp_token_count, result.time_secs);
   296	    } else {
   297	        info!("PPUT = 0 (no golden path in {:.1}s, {} tx)", result.time_secs, result.tx_count);
   298	    }
   299	}
   300	
   301	fn resolve_problem_path(problem_file: &str, minif2f_dir: &str) -> String {
   302	    if PathBuf::from(problem_file).exists() {
   303	        return problem_file.to_string();
   304	    }
   305	    let test_path = format!("{}/MiniF2F/Test/{}", minif2f_dir, problem_file);
   306	    if PathBuf::from(&test_path).exists() { return test_path; }
   307	    let valid_path = format!("{}/MiniF2F/Valid/{}", minif2f_dir, problem_file);
   308	    if PathBuf::from(&valid_path).exists() { return valid_path; }
   309	    eprintln!("Problem file not found: {}", problem_file);
   310	    std::process::exit(1);
   311	}
   312	
   313	/// Oneshot: single LLM call → verify → PPUT.
   314	async fn run_oneshot(
   315	    problem_file: &str, problem_statement: &str, theorem_name: &str,
   316	    lean_path: &str, proxy_url: &str, model: &str,
   317	) -> PputResult {
   318	    let start = Instant::now();
   319	    let mut acc = RunCostAccumulator::new();
   320	    let mut wc = RunWallClock::new();
   321	    // Phase A atom A4 (FC1-N12 oracle scope): cumulative wall-clock
   322	    // inside Lean for this oneshot run. A single verify_omega call,
   323	    // but bracket so future Phase C Soft Law mode that double-verifies
   324	    // accumulates correctly.
   325	    let mut verifier_wait_ms: u64 = 0;
   326	    // Phase A atom A5 (FC2-N22 budget regime stamp): oneshot has no
   327	    // transaction loop — it issues exactly one LLM call and returns.
   328	    // Stamp `total_proposal` + base=1 so downstream PPUT analysis can
   329	    // join oneshot rows on the same regime axis as swarm rows without
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
   389	                                 Some(acc.total_run_token_count()),
   390	                                 Some(acc.failed_branch_count),
   391	                                 wc.elapsed_ms(),
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
   422	            // B3: close the bracket AFTER the Lean call returns, regardless of
   423	            // verdict. Soft Law mode (Phase C) cannot escape the verify-time
   424	            // accounting by short-circuiting on runtime accept.
   425	            wc.mark_final_accept();
   426	            match verdict {
   427	                Ok(true) => {
   428	                    acc.flip_last_failed_to_accepted();
   429	                    let gp_tokens = response.completion_tokens as u64;
   430	                    let preview: String = response.content.chars().take(500).collect();

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '430,760p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   430	                    let preview: String = response.content.chars().take(500).collect();
   431	                    info!(">>> OMEGA ACCEPTED <<< (path=alone, payload[0..500]={:?})", preview);
   432	                    let proof_file = persist_proof_artifact(
   433	                        problem_file, theorem_name, problem_statement,
   434	                        &response.content, "alone", "oneshot",
   435	                    );
   436	                    // P0-A: Phase B oneshot success — runtime gate IS the
   437	                    // Lean verify call (oracle.verify_omega returned Ok(true)),
   438	                    // so both legs hold. Phase C Soft Law would inject a
   439	                    // separate `verify_post_hoc(&oracle, &response.content)`
   440	                    // call here and pass its result as post_hoc_verified.
   441	                    make_pput(problem_file, "oneshot", model,
   442	                              true, true, start, gp_tokens, 1, 1,
   443	                              None, None, Some(response.content.clone()),
   444	                              Some("alone".to_string()), proof_file,
   445	                              Some(acc.total_run_token_count()),
   446	                              Some(acc.failed_branch_count),
   447	                              wc.elapsed_ms(),
   448	                              false, 1, 1, verifier_wait_ms,
   449	                              oneshot_regime, oneshot_budget_base, &run_id)
   450	                }
   451	                Ok(false) => {
   452	                    // Lean rejected → neither leg.
   453	                    make_pput(problem_file, "oneshot", model,
   454	                              false, false, start, 0, 0, 1,
   455	                              None, None, None, None, None,
   456	                              Some(acc.total_run_token_count()),
   457	                              Some(acc.failed_branch_count),
   458	                              wc.elapsed_ms(),
   459	                              false, 1, 1, verifier_wait_ms,
   460	                              oneshot_regime, oneshot_budget_base, &run_id)
   461	                }
   462	                Err(e) => {
   463	                    warn!("Oracle error: {}", e);
   464	                    // Lean error → measurement failure → neither leg.
   465	                    make_pput(problem_file, "oneshot", model,
   466	                              false, false, start, 0, 0, 1,
   467	                              None, None, None, None, None,
   468	                              Some(acc.total_run_token_count()),
   469	                              Some(acc.failed_branch_count),
   470	                              wc.elapsed_ms(),
   471	                              false, 1, 1, verifier_wait_ms,
   472	                              oneshot_regime, oneshot_budget_base, &run_id)
   473	                }
   474	            }
   475	        }
   476	        Err(e) => {
   477	            // C-012: measurement failure ≠ verified failure.
   478	            // Do not emit PPUT_RESULT — batch runner must retry on resume.
   479	            // C-017: broadcast error explicitly (stderr, non-zero exit).
   480	            error!("LLM error: {}", e);
   481	            eprintln!("MEASUREMENT_ERROR oneshot LLM: {}", e);
   482	            std::process::exit(2);
   483	        }
   484	    }
   485	}
   486	
   487	/// Swarm: N agents, prediction market, Boltzmann routing → PPUT.
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
   507	        max_payload_chars: 8000,
   508	        max_payload_lines: 200,
   509	        system_lp_amount: 200.0,
   510	        // C-011: decide/omega/native_decide forbidden (brute-force precedent)
   511	        forbidden_patterns: vec![
   512	            "native_decide".into(), "decide".into(), "omega".into(),
   513	            "#eval".into(), "IO.Process".into(),
   514	            "IO.FS".into(), "run_tac".into(), "unsafe".into(),
   515	        ],
   516	    };
   517	
   518	    // Phase 1: opt-in tape persistence via env. WAL_DIR=<dir> enables WAL
   519	    // writes to <dir>/<problem>_<timestamp>.jsonl; resumes if file exists.
   520	    // Default off for backward-compat baseline runs.
   521	    let mut bus = if let Ok(wal_dir) = std::env::var("WAL_DIR") {
   522	        let problem_stem = std::path::Path::new(problem_file)
   523	            .file_stem().map(|s| s.to_string_lossy().into_owned())
   524	            .unwrap_or_else(|| "unknown".into());
   525	        let resume_id = std::env::var("WAL_RESUME_ID").ok();
   526	        let id = resume_id.unwrap_or_else(|| {
   527	            std::time::SystemTime::now()
   528	                .duration_since(std::time::UNIX_EPOCH)
   529	                .map(|d| d.as_secs().to_string())
   530	                .unwrap_or_else(|_| "0".into())
   531	        });
   532	        let wal_path = std::path::Path::new(&wal_dir)
   533	            .join(format!("{}_{}.jsonl", problem_stem, id));
   534	        info!("[wal] using {:?}", wal_path);
   535	        match TuringBus::with_wal_path(kernel, config, wal_path) {
   536	            Ok(b) => b,
   537	            Err(e) => {
   538	                error!("[wal] open failed: {} — falling back to in-memory", e);
   539	                TuringBus::new(Kernel::new(), BusConfig {
   540	                    max_payload_chars: 1200, max_payload_lines: 18,
   541	                    system_lp_amount: 200.0,
   542	                    forbidden_patterns: vec![
   543	                        "native_decide".into(), "decide".into(), "omega".into(),
   544	                        "#eval".into(), "IO.Process".into(), "IO.FS".into(),
   545	                        "run_tac".into(), "unsafe".into(),
   546	                    ],
   547	                })
   548	            }
   549	        }
   550	    } else {
   551	        TuringBus::new(kernel, config)
   552	    };
   553	    // Phase 4 (C-041 candidate): cross-problem wallet persistence. WALLET_STATE
   554	    // env points to a json file; if it exists we load agents' carried-over
   555	    // balances/portfolios, otherwise fresh genesis. No second mint under Law 2:
   556	    // genesis_done is serialised, so on_init is a no-op post first boot.
   557	    let wallet_state_path: Option<std::path::PathBuf> = std::env::var("WALLET_STATE")
   558	        .ok().map(std::path::PathBuf::from);
   559	    let wallet = wallet_state_path.as_ref()
   560	        .and_then(|p| WalletTool::load_from_disk(p))
   561	        .unwrap_or_else(|| WalletTool::new(10000.0));
   562	    if wallet_state_path.is_some() && wallet.genesis_done {
   563	        info!("[wallet] resumed from {:?}; existing agents carry balances",
   564	              wallet_state_path);
   565	    }
   566	    bus.mount_tool(Box::new(wallet));
   567	    bus.mount_tool(Box::new(Lean4Oracle::new(
   568	        problem_statement.to_string(), theorem_name.to_string(), lean_path.to_string(),
   569	    )));
   570	    bus.mount_tool(Box::new(SearchTool::new(
   571	        vec![format!("{}/MiniF2F/Test", std::env::var("MINIF2F_DIR")
   572	            .unwrap_or_else(|_| DEFAULT_MINIF2F_DIR.into()))], 20,
   573	    )));
   574	    bus.mount_tool(Box::new(LibrarianTool::new(
   575	        &format!("{}/skills", std::env::var("EXPERIMENT_DIR").unwrap_or_else(|_| ".".into())), 8,
   576	    )));
   577	
   578	    let agent_ids: Vec<String> = (0..n_agents).map(|i| format!("Agent_{}", i)).collect();
   579	    bus.init(&agent_ids);
   580	    // Phase 4: top-up ensure_agents for any IDs not in the loaded state (zero
   581	    // balance if post-genesis, genesis_coins only on first-ever boot).
   582	    if let Some(wallet) = bus.tools.iter_mut()
   583	        .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>())
   584	    {
   585	        wallet.ensure_agents(&agent_ids);
   586	    }
   587	
   588	    // Phase A atom A3 (FC1-N7 δ/AI): per-agent model assignment via the
   589	    // `AGENT_MODELS` env var. Default (unset/empty) broadcasts the global
   590	    // `model` to every Agent_i. Heterogeneous payloads require
   591	    // `PHASE_D_HETERO_OK=1` (Phase B+C single-model invariant — see
   592	    // `agent_models.rs` module header). Failure is fatal at startup so a
   593	    // misconfigured swarm cannot burn LLM budget on bad model identity.
   594	    let agent_models = match minif2f_v4::agent_models::resolve_agent_models(model, n_agents) {
   595	        Ok(v) => v,
   596	        Err(e) => {
   597	            eprintln!("AGENT_MODELS resolution failed: {}", e);
   598	            std::process::exit(1);
   599	        }
   600	    };
   601	    // Stamp on jsonl: uniform → single canonical name; heterogeneous (Phase D
   602	    // only, gated) → `hetero:{m1|m2|...}` so downstream PPUT analysis can
   603	    // distinguish single-model runs from heterogeneous swarm runs without
   604	    // having to crack open the genesis_payload model_snapshot field.
   605	    let run_model_label: String = {
   606	        let first = &agent_models[0];
   607	        if agent_models.iter().all(|m| m == first) {
   608	            first.clone()
   609	        } else {
   610	            let mut sorted: Vec<&str> = agent_models.iter().map(String::as_str).collect();
   611	            sorted.sort();
   612	            sorted.dedup();
   613	            format!("hetero:{}", sorted.join("|"))
   614	        }
   615	    };
   616	    info!("[swarm/{}] agent_models = [{}] (label={})", condition,
   617	          agent_models.join(","), run_model_label);
   618	
   619	    // Art. II.2.1: "不能抹杀群体异质性" — distinct skills per agent.
   620	    // V3 had Math/Bull/Bear roles. V4: tactic-strategy specialization.
   621	    let agent_skills: Vec<&str> = vec![
   622	        "Focus on algebraic simplification: ring, field_simp, linarith, nlinarith.",
   623	        "Focus on structural reasoning: induction, cases, rcases, constructor.",
   624	        "Focus on rewriting and normalization: simp, norm_num, rw, calc.",
   625	    ];
   626	
   627	    let client = ResilientLLMClient::new(proxy_url, 1800, 2);
   628	    let params = BoltzmannParams::from_env();
   629	    // C-012: seed the Boltzmann RNG so A/B runs are reproducible.
   630	    // Only the LLM sampling remains stochastic; same-problem paired comparison absorbs that.
   631	    let boltzmann_seed: u64 = std::env::var("BOLTZMANN_SEED")
   632	        .ok().and_then(|s| s.parse().ok()).unwrap_or(DEFAULT_BOLTZMANN_SEED);
   633	    let mut boltz_rng = StdRng::seed_from_u64(boltzmann_seed);
   634	
   635	    // Phase A atom A5 (FC2-N22 budget regime resolution): read
   636	    // BUDGET_REGIME + MAX_TRANSACTIONS env, validate at startup, and
   637	    // compute the loop bound. Errors abort BEFORE any LLM call so a
   638	    // misconfigured run cannot consume API budget. Default
   639	    // (env unset) = TotalProposal × 200, preserving Phase B baseline
   640	    // bit-for-bit. PREREG_AMENDMENT_p0_defer § 3 condition 3.
   641	    let (budget_regime, budget_max_tx_base, max_transactions) =
   642	        match minif2f_v4::budget_regime::resolve_budget(n_agents) {
   643	            Ok(t) => t,
   644	            Err(e) => {
   645	                eprintln!("BUDGET_REGIME resolution failed: {}", e);
   646	                std::process::exit(1);
   647	            }
   648	        };
   649	    info!("[budget] regime={} base={} effective_max_tx={} (n_agents={})",
   650	          budget_regime.label(), budget_max_tx_base, max_transactions, n_agents);
   651	    let max_transactions = max_transactions as usize;
   652	
   653	    // Art. IV map-reduce tick: periodic tape statistics (clock → mr → map/reduce)
   654	    let tick_interval: usize = std::env::var("TICK_INTERVAL")
   655	        .ok().and_then(|s| s.parse().ok()).unwrap_or(20);
   656	
   657	    // C-036 startup echo: per-agent (skill, temp) so debugging never grep-source.
   658	    let temp_ladder_on = std::env::var("TEMP_LADDER").ok().as_deref() == Some("1");
   659	    let agent_cfg: Vec<String> = (0..n_agents).map(|i| {
   660	        let s = i % agent_skills.len();
   661	        let t = if temp_ladder_on { (0.10_f64 + (i as f64) * 0.15).min(1.30) } else { 0.2 };
   662	        format!("Agent_{}:skill{}:t={:.2}", i, s, t)
   663	    }).collect();
   664	    info!("[swarm/{}] {}", condition, agent_cfg.join(" "));
   665	
   666	    // C-036 telemetry counters.
   667	    let mut tool_dist: HashMap<String, u32> = HashMap::new();
   668	    let mut omega_payload_hashes: HashSet<u64> = HashSet::new();
   669	    let mut omega_attempts: u32 = 0;
   670	    let mut zero_ticks_run: u32 = 0;
   671	    let mut zero_tick_warned = false;
   672	    // Phase A atom A4 (FC1-N11 ∏p decision diversity): hash every parsed
   673	    // proposal payload (append/complete/step) — broader than `omega_*`
   674	    // which only counts OMEGA attempts. Cheap proxy for semantic
   675	    // diversity (full embedding distance is Phase D+ work).
   676	    let mut proposal_hashes: HashSet<u64> = HashSet::new();
   677	    let mut proposal_count: u64 = 0;
   678	    // Phase A atom A4 (FC1-N12 oracle scope): cumulative wall-clock
   679	    // inside Lean for THIS run. Each verify_omega_detailed and
   680	    // verify_partial call brackets its own elapsed and adds it here.
   681	    let mut verifier_wait_ms: u64 = 0;
   682	    // PPUT-CCL B2: full-run cost C_i — every LLM call + tool stdout summed
   683	    // across all proposals (winning + failed branches). Read at terminal
   684	    // make_pput sites and stamped on the emitted jsonl row.
   685	    let mut acc = RunCostAccumulator::new();
   686	    // PPUT-CCL B3: full-run wall-clock T_i — first agent prompt → final Lean
   687	    // call. Opened on first tx's prompt build, closed before each return.
   688	    let mut wc = RunWallClock::new();
   689	    // Art. III.2: per-agent search result cache (bounded), fed into next prompt.
   690	    let mut search_cache: HashMap<String, Vec<String>> = HashMap::new();
   691	    // F-2026-04-19-05: cap searches per agent; beyond cap we remove `search`
   692	    // from the tool list so agents stop wasting budget on name-match misses.
   693	    let search_cap: u32 = std::env::var("SEARCH_CAP")
   694	        .ok().and_then(|s| s.parse().ok()).unwrap_or(20);
   695	    let mut search_count: HashMap<String, u32> = HashMap::new();
   696	    // PPUT-CCL B7-extra (PREREG § 5.5): calibration treatment toggle.
   697	    // When enabled, every proposal at tx >= ROLLBACK_TX_THRESHOLD is
   698	    // synthetically vetoed. Constitutionally that is FC1-E18 (∏p=0 → Q_t)
   699	    // applied repeatedly; the run then exhausts at FC2-N22 HALT via
   700	    // `HaltReason::MaxTxExhausted`. We short-circuit at the threshold tx
   701	    // for efficiency — see `rollback_sim.rs` module header for why this
   702	    // is observably equivalent to running the loop to natural exhaustion.
   703	    let rollback_sim_on = minif2f_v4::rollback_sim::rollback_simulation_enabled();
   704	    if rollback_sim_on {
   705	        info!("[rollback_sim] PREREG § 5.5 calibration treatment ON \
   706	               (synthetic veto at tx >= {})", minif2f_v4::rollback_sim::ROLLBACK_TX_THRESHOLD);
   707	    }
   708	
   709	    for tx in 0..max_transactions {
   710	        // PPUT-CCL B7-extra: short-circuit guard. Constitutional anchor
   711	        // FC1-E18 + FC2-N22 (existing MaxTxExhausted variant). Stamps
   712	        // tx_count at the threshold, not at max_transactions, so jsonl
   713	        // analysis can distinguish a calibration treatment exit from a
   714	        // real natural exhaustion.
   715	        if minif2f_v4::rollback_sim::should_simulate_rollback(tx as u64, rollback_sim_on) {
   716	            warn!("[rollback_sim] firing at tx={} — synthetic ∏p=0 from this tx, \
   717	                   short-circuit to MaxTxExhausted exit (cost-asymmetric: skips \
   718	                   ~150 LLM calls vs honest vetoed loop; downstream PPUT analysis \
   719	                   MUST honor synthetic_short_circuit=true on this row)", tx);
   720	            // A6 FC2-N22 (HALT): synthetic short-circuit path. Phase D
   721	            // join key: reason="SyntheticShortCircuit" disambiguates from
   722	            // natural MaxTxExhausted (which exits at tx=max_transactions).
   723	            minif2f_v4::fc_trace::emit_event(
   724	                minif2f_v4::fc_trace::FcId::Fc2N22,
   725	                &run_id, Some(tx as u64), None,
   726	                &[("reason", minif2f_v4::fc_trace::json_str("SyntheticShortCircuit"))],
   727	            );
   728	            wc.mark_final_accept();
   729	            // A4: synthetic short-circuit is NOT a max-tx exhaustion (it
   730	            // exits ~150 tx EARLY at the rollback threshold). hit_max_tx
   731	            // stays false — synthetic_short_circuit is the disambiguator
   732	            // for this calibration-treatment path.
   733	            let mut result = make_pput(problem_file, &condition, &run_model_label,
   734	                                       false, false, start, 0, 0,
   735	                                       tx as u64, Some(tool_dist), None,
   736	                                       None, None, None,
   737	                                       Some(acc.total_run_token_count()),
   738	                                       Some(acc.failed_branch_count),
   739	                                       wc.elapsed_ms(),
   740	                                       false,
   741	                                       proposal_hashes.len() as u64,
   742	                                       proposal_count,
   743	                                       verifier_wait_ms,
   744	                                       budget_regime, budget_max_tx_base, &run_id);
   745	            // B7-extra disambiguator: distinguish this calibration-treatment
   746	            // exit from a natural max-tx exhaustion in downstream PPUT
   747	            // analysis. See PputResult::synthetic_short_circuit doc-comment
   748	            // for the cost-asymmetry note.
   749	            result.synthetic_short_circuit = Some(true);
   750	            return result;
   751	        }
   752	
   753	        // PPUT-CCL B3 (mid-term audit P0-C fix 2026-04-25): open the wall-clock
   754	        // bracket at the top of the FIRST tx (before chain/skill/board build
   755	        // and before build_agent_prompt). Idempotent — only the first tx's
   756	        // call sticks; subsequent calls no-op. PREREG § 5 / plan B3 define
   757	        // T_i as "first agent prompt construction"; this is the earliest
   758	        // moment the agent begins constructing its prompt.
   759	        wc.mark_first_read();
   760	

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '760,1040p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   760	
   761	        // Map-reduce tick (Art. IV mermaid: clock → mr → tape)
   762	        if tick_interval > 0 && tx > 0 && tx % tick_interval == 0 {
   763	            let tape_len = bus.kernel.tape.time_arrow().len();
   764	            let market_count = bus.kernel.markets.len();
   765	            let ticker = bus.kernel.market_ticker(5);
   766	            let top_prices: Vec<String> = ticker.iter()
   767	                .map(|(id, p)| format!("{}:{:.0}%", id, p * 100.0))
   768	                .collect();
   769	            info!("[tick@tx{}] tape={} markets={} top={}", tx, tape_len, market_count,
   770	                top_prices.join(", "));
   771	            // A6 FC2-N20 (mr tick): clock → mr → tape per Art. IV.
   772	            // Phase D consumer joins on (run_id, tx) to derive the
   773	            // tape-growth curve and detect zero-tick stalls before they
   774	            // become C-036 alarm events.
   775	            minif2f_v4::fc_trace::emit_event(
   776	                minif2f_v4::fc_trace::FcId::Fc2N20,
   777	                &run_id, Some(tx as u64), None,
   778	                &[
   779	                    ("tape_len", tape_len.to_string()),
   780	                    ("market_count", market_count.to_string()),
   781	                ],
   782	            );
   783	            // Phase 6-emergent: refresh shared team board from facts only.
   784	            // Per-agent cumulative balance + recent tape-node authorship counts
   785	            // + top market prices. No instructions, no "should" — just state.
   786	            if std::env::var("EMERGENT_ROLES").ok().as_deref() == Some("1") {
   787	                let agents_sorted: Vec<String> = agent_ids.clone();
   788	                let mut author_counts: std::collections::HashMap<String, u32> =
   789	                    std::collections::HashMap::new();
   790	                for nid in bus.kernel.tape.time_arrow() {
   791	                    if let Some(n) = bus.kernel.tape.get(nid) {
   792	                        *author_counts.entry(n.author.clone()).or_insert(0) += 1;
   793	                    }
   794	                }
   795	                let wallet_balances: std::collections::HashMap<String, f64> =
   796	                    bus.tools.iter()
   797	                        .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
   798	                        .map(|w| w.balances.clone())
   799	                        .unwrap_or_default();
   800	                let mut board = format!("# tick@tx{} (tape_nodes={})\n", tx, tape_len);
   801	                for a in &agents_sorted {
   802	                    let bal = wallet_balances.get(a).copied().unwrap_or(10000.0);
   803	                    let delta = bal - 10000.0;
   804	                    let nodes = author_counts.get(a).copied().unwrap_or(0);
   805	                    board.push_str(&format!(
   806	                        "- {}: balance={:.0} (Δ{:+.0}), tape_nodes_authored={}\n",
   807	                        a, bal, delta, nodes));
   808	                }
   809	                if !top_prices.is_empty() {
   810	                    board.push_str(&format!("markets: {}\n", top_prices.join(", ")));
   811	                }
   812	                // Preserve any agent posts that were already in the file (append-only).
   813	                if let Some(lib) = bus.tools.iter()
   814	                    .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
   815	                {
   816	                    let existing = lib.read_board();
   817	                    // Keep only the POST lines (they carry agent-originated intent).
   818	                    let posts: String = existing.lines()
   819	                        .filter(|l| l.starts_with("## POST") || (l.starts_with(" ") == false && !l.starts_with("#") && !l.starts_with("-") && !l.starts_with("markets:")))
   820	                        .collect::<Vec<_>>()
   821	                        .join("\n");
   822	                    let full = if posts.is_empty() {
   823	                        board
   824	                    } else {
   825	                        format!("{}\n{}\n", board, posts)
   826	                    };
   827	                    let _ = lib.write_board(&full);
   828	                }
   829	            }
   830	            // C-036 zero-tick alarm: 5 consecutive ticks with no constitutional engine activity.
   831	            if tape_len == 0 && market_count == 0 {
   832	                zero_ticks_run += 1;
   833	                if zero_ticks_run >= 5 && !zero_tick_warned {
   834	                    warn!("[harness] {} consecutive zero-ticks (tape & markets idle) — \
   835	                           constitutional engines bypassed (Art. II.1/II.2 unused)", zero_ticks_run);
   836	                    zero_tick_warned = true;
   837	                }
   838	            } else {
   839	                zero_ticks_run = 0;
   840	            }
   841	        }
   842	
   843	        let agent_idx = tx % n_agents;
   844	        let agent_id = &agent_ids[agent_idx];
   845	        let snap = bus.snapshot();
   846	
   847	        let chain = if snap.tape.is_empty() {
   848	            problem_statement.to_string()
   849	        } else {
   850	            let nodes: Vec<String> = snap.tape.time_arrow().iter()
   851	                .filter_map(|id| snap.tape.get(id))
   852	                .map(|n| format!("[{}] {}: {}", n.id, n.author, n.payload))
   853	                .collect();
   854	            format!("{}\n\n=== Proof Chain ===\n{}", problem_statement, nodes.join("\n"))
   855	        };
   856	
   857	        let errors = bus.recent_rejections(agent_id, 3);
   858	        // Art. II.2.1: per-agent skill specialization + Librarian learned memory
   859	        let base_skill = agent_skills.get(agent_idx % agent_skills.len()).unwrap_or(&"");
   860	        let learned = bus.tools.iter()
   861	            .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
   862	            .and_then(|lib| lib.read_agent_memory(agent_id))
   863	            .unwrap_or_default();
   864	        let skill = if learned.is_empty() {
   865	            base_skill.to_string()
   866	        } else {
   867	            format!("{}\n\n{}", base_skill, learned)
   868	        };
   869	        let hits_ref: Vec<String> = search_cache.get(agent_id).cloned().unwrap_or_default();
   870	        let tools_desc = if search_count.get(agent_id).copied().unwrap_or(0) >= search_cap {
   871	            "append, complete, invest"
   872	        } else {
   873	            "append, complete, invest, search"
   874	        };
   875	        // Phase 6-emergent: read the shared team board. Gated by EMERGENT_ROLES=1
   876	        // so baseline behaviour is untouched. Board content is built by
   877	        // Librarian at periodic ticks (see refresh_board below).
   878	        let team_board: String = if std::env::var("EMERGENT_ROLES").ok().as_deref() == Some("1") {
   879	            bus.tools.iter()
   880	                .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
   881	                .map(|l| l.read_board())
   882	                .unwrap_or_default()
   883	        } else {
   884	            String::new()
   885	        };
   886	        let prompt = build_agent_prompt(
   887	            &chain, &skill, &snap.market_ticker, &errors, &hits_ref,
   888	            snap.get_balance(agent_id), tools_desc, &team_board,
   889	        );
   890	
   891	        // Phase A atom A3: bind δ for this agent_idx (same vector resolved
   892	        // once at run_swarm entry from AGENT_MODELS env). In Phase B+C this
   893	        // is uniform across all agent_idx; in Phase D it may diverge.
   894	        let agent_model = &agent_models[agent_idx];
   895	        // Model-aware max_tokens (same rule as oneshot branch). Per-agent so
   896	        // a heterogeneous Phase D swarm mixing chat + reasoner backbones gets
   897	        // the right ceiling per-call instead of a single global heuristic.
   898	        let max_toks = if agent_model.contains("chat") { 8000 } else { 16000 };
   899	        // Art. II.2.1 anti-homogeneity: per-agent temperature ladder breaks
   900	        // sampling correlation among role-distinct agents (F-2026-04-18-03).
   901	        // Disabled (keep at 0.2) when TEMP_LADDER!=1 to isolate the mechanism.
   902	        let temp: f64 = if std::env::var("TEMP_LADDER").ok().as_deref() == Some("1") {
   903	            (0.10_f64 + (agent_idx as f64) * 0.15).min(1.30)
   904	        } else {
   905	            0.2
   906	        };
   907	        let request = GenerateRequest {
   908	            model: agent_model.clone(),
   909	            messages: vec![Message { role: "user".into(), content: prompt }],
   910	            temperature: Some(temp),
   911	            max_tokens: Some(max_toks),
   912	        };
   913	
   914	        // PPUT-CCL B6 runtime gate (swarm path): swarm prompts include
   915	        // tape contents, board posts, search hits, and learned memory —
   916	        // any of these state surfaces could in principle inject a PPUT
   917	        // value at runtime even when the prompt builder is clean. Gate
   918	        // every tx, every agent, every iteration.
   919	        assert_no_metric_leak(&request.messages[0].content);
   920	        match client.generate(&request).await {
   921	            Ok(response) => {
   922	                acc.record_llm_call(response.prompt_tokens, response.completion_tokens);
   923	                // PPUT-CCL B2: every parsed proposal default-records as failed.
   924	                // OMEGA-accept return paths flip the last record before returning.
   925	                acc.record_proposal(false);
   926	                match parse_agent_output(&response.content) {
   927	                    Ok(action) => match action.tool.as_str() {
   928	                        "append" => {
   929	                            *tool_dist.entry("append".into()).or_insert(0) += 1;
   930	                            if let Some(payload) = &action.payload {
   931	                                // A4: record proposal for tactic_diversity.
   932	                                let mut ph = std::collections::hash_map::DefaultHasher::new();
   933	                                payload.hash(&mut ph);
   934	                                proposal_hashes.insert(ph.finish());
   935	                                proposal_count += 1;
   936	                                let prices: std::collections::HashMap<String, f64> =
   937	                                    snap.markets.iter()
   938	                                        .map(|(id, m)| (id.clone(), m.yes_price))
   939	                                        .collect();
   940	                                let parent = boltzmann_select_parent(
   941	                                    &snap.tape, &prices, &params, &mut boltz_rng
   942	                                );
   943	                                match bus.append(agent_id, payload, parent.as_deref()) {
   944	                                    Ok(BusResult::Appended { node_id }) => {
   945	                                        info!("[tx {}] {} +{}", tx, agent_id, node_id);
   946	                                        // Art. III.2 Librarian: every compress_interval appends,
   947	                                        // write mechanical summary (TopK error classes) to agent's
   948	                                        // learned.md. This is white-box compression (Art. I.2:
   949	                                        // deterministic statistical algorithm), not LLM-based.
   950	                                        if let Some(lib) = bus.tools.iter()
   951	                                            .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>()) {
   952	                                            if lib.should_compress() {
   953	                                                let errors = bus.recent_rejections(agent_id, 10);
   954	                                                let summary = format!(
   955	                                                    "# Learned patterns (auto-compressed)\n\
   956	                                                     Common errors: {}\n\
   957	                                                     Tape depth: {}\n",
   958	                                                    errors.join(", "),
   959	                                                    snap.tape.time_arrow().len(),
   960	                                                );
   961	                                                let _ = lib.write_agent_memory(agent_id, &summary);
   962	                                                info!("[tx {}] Librarian compressed for {}", tx, agent_id);
   963	                                            }
   964	                                        }
   965	                                    }
   966	                                    Ok(BusResult::Vetoed { reason }) => {
   967	                                        warn!("[tx {}] VETO: {}", tx, reason);
   968	                                    }
   969	                                    _ => {}
   970	                                }
   971	                            }
   972	                        }
   973	                        "complete" => {
   974	                            *tool_dist.entry("complete".into()).or_insert(0) += 1;
   975	                            if let Some(payload) = &action.payload {
   976	                                // Art. IV (∏p(output | Q_t)): Q_t (tape) feeds the verification
   977	                                // predicate. Dual-path: try payload-alone first (standalone proof
   978	                                // preserved), then tape+payload (tape-built proof). Accept whichever
   979	                                // succeeds. This keeps Q_t in the ∏p domain without punishing
   980	                                // self-contained proofs that ignored tape.
   981	                                let tape_chain: String = bus.kernel.tape.time_arrow().iter()
   982	                                    .filter_map(|id| bus.kernel.tape.get(id))
   983	                                    .map(|n| n.payload.clone())
   984	                                    .collect::<Vec<_>>()
   985	                                    .join("\n");
   986	                                let tape_len = bus.kernel.tape.time_arrow().len();
   987	                                // C-036: track payload diversity over what agent proposed.
   988	                                let mut h = std::collections::hash_map::DefaultHasher::new();
   989	                                payload.hash(&mut h);
   990	                                omega_payload_hashes.insert(h.finish());
   991	                                omega_attempts += 1;
   992	                                // A4: also record into the broader proposal set
   993	                                // for tactic_diversity (covers append/complete/step).
   994	                                proposal_hashes.insert(h.finish());
   995	                                proposal_count += 1;
   996	                                info!("[tx {}] OMEGA claim by {} (tape_nodes={}, payload_len={})",
   997	                                      tx, agent_id, tape_len, payload.len());
   998	                                let oracle = Lean4Oracle::new(
   999	                                    problem_statement.to_string(),
  1000	                                    theorem_name.to_string(),
  1001	                                    lean_path.to_string(),
  1002	                                );
  1003	                                // Path 1: payload alone (A4 verifier_wait bracket)
  1004	                                let v_t0 = Instant::now();
  1005	                                let r_alone = oracle.verify_omega_detailed(payload);
  1006	                                let v_alone_elapsed = v_t0.elapsed().as_millis() as u64;
  1007	                                verifier_wait_ms += v_alone_elapsed;
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

exec
/bin/bash -lc "nl -ba src/drivers/llm_proxy.py | sed -n '135,230p' && nl -ba scripts/test_llm_proxy.py | sed -n '1,125p' && nl -ba experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs | sed -n '1,140p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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
     1	#!/usr/bin/env python3
     2	"""A8e fix F3 — unit tests for src/drivers/llm_proxy.py routing logic.
     3	
     4	Codex#4 (round-1 A8 audit) caught: `Qwen/Qwen2.5-7B-Instruct` misrouted
     5	to DashScope because `m.startswith("qwen")` won after the slash check.
     6	This test file pins the routing matrix as a CI conformance gate.
     7	
     8	Also verifies the round-robin mechanic (F2) without invoking any cloud
     9	API: `_build_clients` is monkeypatched to return a list of dummy
    10	sentinels, then `get_client_round_robin` is called multiple times and
    11	the per-key counter distribution is asserted.
    12	
    13	Run: `python3 scripts/test_llm_proxy.py` (no pytest required).
    14	"""
    15	import os
    16	import sys
    17	import unittest
    18	
    19	# Inject repo root so `from src.drivers import llm_proxy` works.
    20	ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    21	sys.path.insert(0, ROOT)
    22	
    23	# Import via the file path (the proxy isn't a package; it's a script).
    24	import importlib.util
    25	
    26	spec = importlib.util.spec_from_file_location(
    27	    "llm_proxy", os.path.join(ROOT, "src", "drivers", "llm_proxy.py")
    28	)
    29	llm_proxy = importlib.util.module_from_spec(spec)
    30	spec.loader.exec_module(llm_proxy)
    31	
    32	
    33	class RoutingMatrixTests(unittest.TestCase):
    34	    """detect_provider over the canonical model id matrix."""
    35	
    36	    def test_explicit_prefix_wins(self):
    37	        self.assertEqual(llm_proxy.detect_provider("siliconflow:Qwen/Qwen2.5-7B-Instruct"), "siliconflow")
    38	        self.assertEqual(llm_proxy.detect_provider("deepseek:deepseek-v4-flash"), "deepseek")
    39	        self.assertEqual(llm_proxy.detect_provider("dashscope:qwen3-8b"), "dashscope")
    40	
    41	    def test_unknown_explicit_prefix_falls_through_to_heuristic(self):
    42	        # `nonsense:` is not in PROVIDERS — treat the whole string as
    43	        # a bare model id and route by heuristic. A bare string with
    44	        # a colon and no slash falls to the dashscope default.
    45	        self.assertEqual(llm_proxy.detect_provider("nonsense:foo-bar"), "dashscope")
    46	
    47	    def test_deepseek_substring(self):
    48	        self.assertEqual(llm_proxy.detect_provider("deepseek-v4-flash"), "deepseek")
    49	        self.assertEqual(llm_proxy.detect_provider("deepseek-chat"), "deepseek")
    50	        self.assertEqual(llm_proxy.detect_provider("DeepSeek-V4-Flash"), "deepseek")
    51	
    52	    def test_qwen_huggingface_style_routes_to_siliconflow(self):
    53	        # A8e fix F3 (Codex#4): the round-1 bug was that this
    54	        # misrouted to dashscope because "qwen" prefix won after the
    55	        # slash check. Pinned here so it can never regress.
    56	        self.assertEqual(
    57	            llm_proxy.detect_provider("Qwen/Qwen2.5-7B-Instruct"),
    58	            "siliconflow",
    59	        )
    60	        self.assertEqual(
    61	            llm_proxy.detect_provider("Qwen/Qwen3.5-397B-A17B"),
    62	            "siliconflow",
    63	        )
    64	
    65	    def test_other_huggingface_style_routes_to_siliconflow(self):
    66	        self.assertEqual(llm_proxy.detect_provider("openai/gpt-4o"), "siliconflow")
    67	        self.assertEqual(
    68	            llm_proxy.detect_provider("meta-llama/Llama-3.1-70B-Instruct"),
    69	            "siliconflow",
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
   101	        )
   102	
   103	    def test_bare_qwen_routes_to_dashscope(self):
   104	        # Bare model ids without a slash are direct DashScope catalog.
   105	        self.assertEqual(llm_proxy.detect_provider("qwen3-8b"), "dashscope")
   106	        self.assertEqual(llm_proxy.detect_provider("qwen-max"), "dashscope")
   107	
   108	    def test_default_fallback_dashscope(self):
   109	        self.assertEqual(llm_proxy.detect_provider("some-unknown-model"), "dashscope")
   110	        self.assertEqual(llm_proxy.detect_provider(""), "dashscope")
   111	
   112	
   113	class StripProviderPrefixTests(unittest.TestCase):
   114	    def test_strips_known_prefix(self):
   115	        self.assertEqual(
   116	            llm_proxy.strip_provider_prefix("siliconflow:Qwen/Qwen2.5-7B-Instruct"),
   117	            "Qwen/Qwen2.5-7B-Instruct",
   118	        )
   119	        self.assertEqual(
   120	            llm_proxy.strip_provider_prefix("deepseek:deepseek-v4-flash"),
   121	            "deepseek-v4-flash",
   122	        )
   123	
   124	    def test_leaves_unknown_prefix_intact(self):
   125	        # `nonsense:foo` is NOT a known provider, so the colon is part
     1	// A8e2 fix G1 — recurring conformance gate for `src/drivers/llm_proxy.py`.
     2	//
     3	// Round-2 audit (Codex R2#1 + Gemini R2#1) caught: `scripts/test_llm_proxy.py`
     4	// existed and was in Trust Root, but was only documented as a manual
     5	// invocation. A test that does not run automatically is just
     6	// documentation — it cannot prevent the V3L-27-class regression that
     7	// Gemini's round-1 VETO targeted.
     8	//
     9	// This integration test bridges the Python proxy suite into
    10	// `cargo test --workspace` so it runs on every Rust test invocation
    11	// and on every CI pipeline that already exercises Rust tests. The
    12	// test depends on the system `python3` interpreter being available;
    13	// if not, it FAILS CLOSED with a clear diagnostic (A8e3 fix H6 —
    14	// Codex round-3 caught the prior soft-skip behavior that masqueraded
    15	// as a passing gate when Python was missing). Explicit opt-out:
    16	// `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1` (must be set deliberately;
    17	// the bypass is logged loudly to make the gate's absence visible).
    18	//
    19	// Constitutional anchor: meta-witness for atom A7 (case C-027 +
    20	// V3L-27 mitigation — multi-key round-robin avoiding single-key
    21	// rate-limit collapse).
    22	
    23	use std::path::PathBuf;
    24	use std::process::Command;
    25	
    26	fn repo_root() -> PathBuf {
    27	    // The minif2f_v4 crate's Cargo.toml lives at the repo's
    28	    // experiments/minif2f_v4 path, so two parents up is the repo root.
    29	    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    30	        .parent()
    31	        .and_then(|p| p.parent())
    32	        .expect("repo root reachable from CARGO_MANIFEST_DIR")
    33	        .to_path_buf()
    34	}
    35	
    36	#[test]
    37	fn proxy_python_conformance_suite_passes() {
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
    87	         status: {:?}\nstdout:\n{}\nstderr:\n{}",
    88	        output.status,
    89	        stdout,
    90	        stderr
    91	    );
    92	
    93	    // Sanity-check the unittest summary line is present so a future
    94	    // refactor that removes the test class definitions surfaces here
    95	    // instead of silently skipping.
    96	    assert!(
    97	        stderr.contains("OK") || stdout.contains("OK"),
    98	        "unittest output must contain the trailing 'OK' line.\n\
    99	         stdout:\n{}\nstderr:\n{}",
   100	        stdout,
   101	        stderr
   102	    );
   103	}

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '1040,1270p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1040	                                        };
  1041	                                        minif2f_v4::fc_trace::emit_event(
  1042	                                            minif2f_v4::fc_trace::FcId::Fc1N12,
  1043	                                            &run_id, Some(tx as u64), Some(agent_id.as_str()),
  1044	                                            &[
  1045	                                                ("verdict", minif2f_v4::fc_trace::json_str(r_combined_verdict)),
  1046	                                                ("elapsed_ms", v_combined_elapsed.to_string()),
  1047	                                                ("path", minif2f_v4::fc_trace::json_str("tape+payload")),
  1048	                                            ],
  1049	                                        );
  1050	                                        if matches!(r_combined, Ok((true, _))) {
  1051	                                            *tool_dist.entry("complete_via_tape".into()).or_insert(0) += 1;
  1052	                                        }
  1053	                                        (combined, "tape+payload", r_combined)
  1054	                                    }
  1055	                                    _ => (payload.clone(), "alone", r_alone.clone()),
  1056	                                };
  1057	                                // PPUT-CCL B3: close bracket AFTER both Lean verify paths return.
  1058	                                // Soft Law (Phase C) cannot exit ahead of verify-time accounting.
  1059	                                wc.mark_final_accept();
  1060	                                match r_final {
  1061	                                    Ok((true, _)) => {
  1062	                                        // PPUT-CCL B2: this proposal verified — flip the failed
  1063	                                        // record made at parse time into the run's accepted slot.
  1064	                                        acc.flip_last_failed_to_accepted();
  1065	                                        // Phase 0 (C-039): persist the winning artifact so external
  1066	                                        // verifiers can re-run lean from disk alone.
  1067	                                        let preview: String = full_proof.chars().take(500).collect();
  1068	                                        info!(">>> OMEGA ACCEPTED <<< (path={}, payload[0..500]={:?})",
  1069	                                              path_choice, preview);
  1070	                                        let proof_file = persist_proof_artifact(
  1071	                                            problem_file, &theorem_name, &problem_statement,
  1072	                                            &full_proof, path_choice, agent_id,
  1073	                                        );
  1074	                                        // Phase 2.1 (C-043 candidate): mandatory wtool. Art. IV says
  1075	                                        // `∏p = 1 ⟹ Q_{t+1} = wtool(output)`. Before halting, write
  1076	                                        // the winning payload as a tape node through the standard
  1077	                                        // append pipeline. This automatically fires founder grant
  1078	                                        // (Phase 2 reward-pull) for the winning author and makes
  1079	                                        // every solve end with a canonical tape node on the GP.
  1080	                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
  1081	                                        *tool_dist.entry("omega_wtool".into()).or_insert(0) += 1;
  1082	                                        // Use oracle-blessed path: Lean has already accepted this
  1083	                                        // payload, so bus-level forbidden_patterns and size caps
  1084	                                        // would only re-reject legitimate tactics (e.g. `omega`,
  1085	                                        // `decide` used inside a verified proof — not brute-force).
  1086	                                        let omega_node_id = match bus.append_oracle_accepted(
  1087	                                            agent_id, payload, parent.as_deref(),
  1088	                                        ) {
  1089	                                            Ok(BusResult::Appended { node_id }) => Some(node_id),
  1090	                                            Ok(BusResult::Vetoed { reason }) => {
  1091	                                                warn!("[art-iv] OMEGA wtool VETO (unexpected after oracle accept): {}", reason);
  1092	                                                None
  1093	                                            }
  1094	                                            _ => None,
  1095	                                        };
  1096	                                        let tape_tokens: u64 = bus.kernel.tape.time_arrow().iter()
  1097	                                            .filter_map(|id| bus.kernel.tape.get(id))
  1098	                                            .map(|n| n.payload.len() as u64)
  1099	                                            .sum();
  1100	                                        // C-012: gp_tokens reflects the actual tape (now containing
  1101	                                        // the winner), no double-count needed.
  1102	                                        let gp_tokens = tape_tokens.max(response.completion_tokens as u64);
  1103	                                        let gp = bus.kernel.tape.time_arrow().to_vec();
  1104	                                        let gp_nodes = gp.len();
  1105	                                        if omega_node_id.is_some() {
  1106	                                            info!("[art-iv] OMEGA written as tape node; gp_nodes={}", gp_nodes);
  1107	                                        }
  1108	                                        bus.halt_and_settle(&gp).ok();
  1109	                                        // A6 FC2-N22 (HALT — OmegaAccepted via full proof): the
  1110	                                        // canonical success-path event. Phase D filters on
  1111	                                        // reason="OmegaAccepted" + gp_path="alone|tape+payload" to
  1112	                                        // build the OMEGA accept-rate timeseries.
  1113	                                        minif2f_v4::fc_trace::emit_event(
  1114	                                            minif2f_v4::fc_trace::FcId::Fc2N22,
  1115	                                            &run_id, Some(tx as u64), Some(agent_id.as_str()),
  1116	                                            &[
  1117	                                                ("reason", minif2f_v4::fc_trace::json_str("OmegaAccepted")),
  1118	                                                ("gp_path", minif2f_v4::fc_trace::json_str(path_choice)),
  1119	                                                ("gp_nodes", gp_nodes.to_string()),
  1120	                                            ],
  1121	                                        );
  1122	                                        // Phase 4: persist wallet state so next problem's run
  1123	                                        // inherits carried-over balances (reputation).
  1124	                                        if let Some(ref wp) = wallet_state_path {
  1125	                                            if let Some(w) = bus.tools.iter()
  1126	                                                .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
  1127	                                            {
  1128	                                                if let Err(e) = w.save_to_disk(wp) {
  1129	                                                    warn!("[wallet] save failed to {:?}: {}", wp, e);
  1130	                                                }
  1131	                                            }
  1132	                                        }
  1133	                                        let upr = if omega_attempts > 0 {
  1134	                                            Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
  1135	                                        } else { None };
  1136	                                        // P0-A: Phase B swarm complete — runtime gate IS the
  1137	                                        // Lean verify_omega_detailed call we just consumed
  1138	                                        // (Ok((true, _))). Both legs hold. Phase C Soft Law
  1139	                                        // would inject `verify_post_hoc(&oracle, &full_proof)`
  1140	                                        // here and pass its result as post_hoc_verified.
  1141	                                        return make_pput(problem_file, &condition, &run_model_label,
  1142	                                                        true, true,
  1143	                                                        start, gp_tokens, gp_nodes, tx as u64 + 1,
  1144	                                                        Some(tool_dist), upr,
  1145	                                                        Some(full_proof.clone()),
  1146	                                                        Some(path_choice.to_string()),
  1147	                                                        proof_file,
  1148	                                                        Some(acc.total_run_token_count()),
  1149	                                                        Some(acc.failed_branch_count),
  1150	                                                        wc.elapsed_ms(),
  1151	                                                        false,
  1152	                                                        proposal_hashes.len() as u64,
  1153	                                                        proposal_count,
  1154	                                                        verifier_wait_ms,
  1155	                                                        budget_regime, budget_max_tx_base, &run_id);
  1156	                                    }
  1157	                                    Ok((false, err_detail)) => {
  1158	                                        // Step-B v3: classify + record class label (C-022 shield).
  1159	                                        let class = classify_lean_error(&err_detail);
  1160	                                        bus.record_rejection(agent_id, class.label());
  1161	                                        // PPUT-CCL B2: rejection error feeds back into next prompt's
  1162	                                        // recent_rejections — count those bytes against C_i.
  1163	                                        acc.record_tool_stdout(&err_detail);
  1164	                                        let preview: String = payload.chars().take(300).collect();
  1165	                                        warn!("[tx {}] OMEGA rejected ({}). payload[0..300]={:?}", tx, class.label(), preview);
  1166	                                    }
  1167	                                    Err(e) => {
  1168	                                        warn!("[tx {}] OMEGA oracle error: {}", tx, e);
  1169	                                    }
  1170	                                }
  1171	                            }
  1172	                        }
  1173	                        "invest" => {
  1174	                            *tool_dist.entry("invest".into()).or_insert(0) += 1;
  1175	                            // Law 2: Only Investment Costs Money (1 Coin = 1 YES + 1 NO).
  1176	                            // Agent bets on a tape node's quality. This drives price signals
  1177	                            // (Art. II.2) which guide Boltzmann routing (Art. II.2.1).
  1178	                            // Direction: prefer explicit `direction` field (long/short);
  1179	                            // fall back to sign of amount (positive=long, negative=short).
  1180	                            // Bidirectional signals let agents express dissent (Art. II.2).
  1181	                            if let (Some(node_id), Some(amount)) = (&action.node, action.amount) {
  1182	                                let amt = amount.abs();
  1183	                                if amt > 0.0 {
  1184	                                    let buy_yes = match action.direction.as_deref() {
  1185	                                        Some("long") | Some("yes") | Some("LONG") | Some("YES") => true,
  1186	                                        Some("short") | Some("no") | Some("SHORT") | Some("NO") => false,
  1187	                                        _ => amount > 0.0,  // sign-based fallback
  1188	                                    };
  1189	                                    // Law 2 conservation: validate market BEFORE debit (no coin-loss path)
  1190	                                    let market_exists = bus.kernel.yes_price(node_id).is_some();
  1191	                                    if !market_exists {
  1192	                                        warn!("[tx {}] invest: no market for {} (hallucinated node?)", tx, node_id);
  1193	                                    } else {
  1194	                                        // Debit wallet → buy shares → record (atomic intent)
  1195	                                        let wallet_ok = bus.tools.iter_mut()
  1196	                                            .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>())
  1197	                                            .map(|w| w.deduct(agent_id, amt).is_ok())
  1198	                                            .unwrap_or(false);
  1199	                                        if wallet_ok {
  1200	                                            let result = if buy_yes {
  1201	                                                bus.kernel.buy_yes(node_id, amt)
  1202	                                            } else {
  1203	                                                bus.kernel.buy_no(node_id, amt)
  1204	                                            };
  1205	                                            match result {
  1206	                                                Ok(shares) => {
  1207	                                                    info!("[tx {}] {} invested {:.0} {} on {} → {:.1} shares",
  1208	                                                        tx, agent_id, amt,
  1209	                                                        if buy_yes { "YES" } else { "NO" },
  1210	                                                        node_id, shares);
  1211	                                                    if let Some(w) = bus.tools.iter_mut()
  1212	                                                        .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>()) {
  1213	                                                        if buy_yes {
  1214	                                                            w.record_shares(agent_id, node_id, shares, 0.0, 0.0);
  1215	                                                        } else {
  1216	                                                            w.record_shares(agent_id, node_id, 0.0, shares, 0.0);
  1217	                                                        }
  1218	                                                    }
  1219	                                                }
  1220	                                                Err(e) => {
  1221	                                                    // Market existed at check but buy failed — should not happen
  1222	                                                    warn!("[tx {}] invest buy error: {} (coins debited, shares not granted — Law 2 violation logged)", tx, e);
  1223	                                                }
  1224	                                            }
  1225	                                        } else {
  1226	                                            warn!("[tx {}] {} insufficient balance for invest", tx, agent_id);
  1227	                                        }
  1228	                                    }
  1229	                                }
  1230	                            }
  1231	                        }
  1232	                        "search" => {
  1233	                            // F-2026-04-19-05 cap: if over budget this agent's turn the
  1234	                            // search slot shouldn't even be offered, but the LLM may still
  1235	                            // emit `search` ignoring the prompt — record and skip execute.
  1236	                            let cnt = search_count.entry(agent_id.clone()).or_insert(0);
  1237	                            if *cnt >= search_cap {
  1238	                                *tool_dist.entry("search_capped".into()).or_insert(0) += 1;
  1239	                            } else {
  1240	                                *cnt += 1;
  1241	                                *tool_dist.entry("search".into()).or_insert(0) += 1;
  1242	                                // Law 1: search is free. Execute and cache top hits (Art. III.2).
  1243	                                if let Some(query) = &action.query {
  1244	                                    let hits = bus.tools.iter()
  1245	                                        .find_map(|t| t.as_any().downcast_ref::<SearchTool>())
  1246	                                        .map(|s| s.search(query))
  1247	                                        .unwrap_or_default();
  1248	                                    let trimmed: Vec<String> = hits.iter().take(5)
  1249	                                        .map(|p| p.rsplit('/').next().unwrap_or(p).to_string())
  1250	                                        .collect();
  1251	                                    // PPUT-CCL B2: search hits feed `hits_ref` into next prompt —
  1252	                                    // count the cached bytes against C_i.
  1253	                                    acc.record_tool_stdout(&trimmed.join("\n"));
  1254	                                    info!("[tx {}] {} search({:?}) → {} hits: {}",
  1255	                                          tx, agent_id, query, hits.len(), trimmed.join(","));
  1256	                                    search_cache.insert(agent_id.clone(), trimmed);
  1257	                                }
  1258	                            }
  1259	                        }
  1260	                        "post" => {
  1261	                            *tool_dist.entry("post".into()).or_insert(0) += 1;
  1262	                            // Phase 6-emergent: agent posts a short message to the
  1263	                            // shared Librarian board. Other agents see it on next
  1264	                            // prompt. State-only; no central role planner.
  1265	                            if let Some(msg) = &action.payload {
  1266	                                if let Some(lib) = bus.tools.iter()
  1267	                                    .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
  1268	                                {
  1269	                                    if let Err(e) = lib.post_to_board(agent_id, msg) {
  1270	                                        warn!("[tx {}] post failed: {}", tx, e);

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '1270,1455p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1270	                                        warn!("[tx {}] post failed: {}", tx, e);
  1271	                                    } else {
  1272	                                        info!("[tx {}] {} posted to board", tx, agent_id);
  1273	                                    }
  1274	                                }
  1275	                            }
  1276	                        }
  1277	                        "step" => {
  1278	                            // Phase 7 (C-043+ Turing δ-step): submit ONE tactic,
  1279	                            // oracle classifies the accumulated tape+tactic prefix
  1280	                            // as Complete / PartialOk / Reject. Writes a tape node
  1281	                            // on PartialOk and Complete so the DAG grows one cell
  1282	                            // at a time — the Art. IV semantics Turing 1936 defines.
  1283	                            *tool_dist.entry("step".into()).or_insert(0) += 1;
  1284	                            if let Some(tactic) = &action.payload {
  1285	                                // A4: record proposal for tactic_diversity.
  1286	                                let mut ph = std::collections::hash_map::DefaultHasher::new();
  1287	                                tactic.hash(&mut ph);
  1288	                                proposal_hashes.insert(ph.finish());
  1289	                                proposal_count += 1;
  1290	                                let tape_chain: String = bus.kernel.tape.time_arrow().iter()
  1291	                                    .filter_map(|id| bus.kernel.tape.get(id))
  1292	                                    .map(|n| n.payload.clone())
  1293	                                    .collect::<Vec<_>>()
  1294	                                    .join("\n");
  1295	                                let prefix = if tape_chain.is_empty() {
  1296	                                    tactic.clone()
  1297	                                } else {
  1298	                                    format!("{}\n{}", tape_chain, tactic)
  1299	                                };
  1300	                                let oracle = Lean4Oracle::new(
  1301	                                    problem_statement.to_string(),
  1302	                                    theorem_name.to_string(),
  1303	                                    lean_path.to_string(),
  1304	                                );
  1305	                                // A4: bracket the Lean partial-verify call.
  1306	                                let v_t0 = Instant::now();
  1307	                                let verdict = oracle.verify_partial(&prefix);
  1308	                                let v_partial_elapsed = v_t0.elapsed().as_millis() as u64;
  1309	                                verifier_wait_ms += v_partial_elapsed;
  1310	                                // A8e fix F4 (Codex#3): FC1-N12 emit for the
  1311	                                // step-verify path. Closes the swarm-side gap
  1312	                                // round-1 audit flagged.
  1313	                                let partial_verdict_str = match &verdict {
  1314	                                    PartialVerdict::Complete => "Complete",
  1315	                                    PartialVerdict::PartialOk => "PartialOk",
  1316	                                    PartialVerdict::Reject(_) => "Reject",
  1317	                                };
  1318	                                minif2f_v4::fc_trace::emit_event(
  1319	                                    minif2f_v4::fc_trace::FcId::Fc1N12,
  1320	                                    &run_id, Some(tx as u64), Some(agent_id.as_str()),
  1321	                                    &[
  1322	                                        ("verdict", minif2f_v4::fc_trace::json_str(partial_verdict_str)),
  1323	                                        ("elapsed_ms", v_partial_elapsed.to_string()),
  1324	                                        ("path", minif2f_v4::fc_trace::json_str("partial")),
  1325	                                    ],
  1326	                                );
  1327	                                // PPUT-CCL B3: close bracket after step-verify returns.
  1328	                                wc.mark_final_accept();
  1329	                                match verdict {
  1330	                                    PartialVerdict::Complete => {
  1331	                                        acc.flip_last_failed_to_accepted();
  1332	                                        info!(">>> OMEGA ACCEPTED <<< via step (depth={} after this write)",
  1333	                                              bus.kernel.tape.time_arrow().len() + 1);
  1334	                                        let proof_file = persist_proof_artifact(
  1335	                                            problem_file, &theorem_name, &problem_statement,
  1336	                                            &prefix, "per_tactic", agent_id,
  1337	                                        );
  1338	                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
  1339	                                        *tool_dist.entry("omega_wtool".into()).or_insert(0) += 1;
  1340	                                        let _ = bus.append_oracle_accepted(
  1341	                                            agent_id, tactic, parent.as_deref(),
  1342	                                        );
  1343	                                        let tape_tokens: u64 = bus.kernel.tape.time_arrow().iter()
  1344	                                            .filter_map(|id| bus.kernel.tape.get(id))
  1345	                                            .map(|n| n.payload.len() as u64)
  1346	                                            .sum();
  1347	                                        let gp_tokens = tape_tokens.max(response.completion_tokens as u64);
  1348	                                        let gp = bus.kernel.tape.time_arrow().to_vec();
  1349	                                        let gp_nodes = gp.len();
  1350	                                        bus.halt_and_settle(&gp).ok();
  1351	                                        let upr = if omega_attempts > 0 {
  1352	                                            Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
  1353	                                        } else { None };
  1354	                                        // A6 FC2-N22 (HALT — OmegaAccepted via per-tactic
  1355	                                        // PartialVerdict::Complete). Distinguished from the
  1356	                                        // full-proof OMEGA path by gp_path="per_tactic"; both
  1357	                                        // share reason="OmegaAccepted".
  1358	                                        minif2f_v4::fc_trace::emit_event(
  1359	                                            minif2f_v4::fc_trace::FcId::Fc2N22,
  1360	                                            &run_id, Some(tx as u64), Some(agent_id.as_str()),
  1361	                                            &[
  1362	                                                ("reason", minif2f_v4::fc_trace::json_str("OmegaAccepted")),
  1363	                                                ("gp_path", minif2f_v4::fc_trace::json_str("per_tactic")),
  1364	                                                ("gp_nodes", gp_nodes.to_string()),
  1365	                                            ],
  1366	                                        );
  1367	                                        // P0-A: Phase B swarm step Complete — runtime gate IS
  1368	                                        // the Lean verify_partial call (PartialVerdict::Complete).
  1369	                                        // Both legs hold. Phase C Soft Law diverges here.
  1370	                                        return make_pput(problem_file, &condition, &run_model_label,
  1371	                                                        true, true,
  1372	                                                        start, gp_tokens, gp_nodes, tx as u64 + 1,
  1373	                                                        Some(tool_dist), upr,
  1374	                                                        Some(prefix.clone()),
  1375	                                                        Some("per_tactic".to_string()),
  1376	                                                        proof_file,
  1377	                                                        Some(acc.total_run_token_count()),
  1378	                                                        Some(acc.failed_branch_count),
  1379	                                                        wc.elapsed_ms(),
  1380	                                                        false,
  1381	                                                        proposal_hashes.len() as u64,
  1382	                                                        proposal_count,
  1383	                                                        verifier_wait_ms,
  1384	                                                        budget_regime, budget_max_tx_base, &run_id);
  1385	                                    }
  1386	                                    PartialVerdict::PartialOk => {
  1387	                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
  1388	                                        match bus.append_oracle_accepted(
  1389	                                            agent_id, tactic, parent.as_deref(),
  1390	                                        ) {
  1391	                                            Ok(BusResult::Appended { node_id }) => {
  1392	                                                *tool_dist.entry("step_partial_ok".into()).or_insert(0) += 1;
  1393	                                                info!("[tx {}] {} step+{} partial OK (depth={})",
  1394	                                                      tx, agent_id, node_id,
  1395	                                                      bus.kernel.tape.time_arrow().len());
  1396	                                            }
  1397	                                            Ok(BusResult::Vetoed { reason }) => {
  1398	                                                warn!("[tx {}] step partial OK but bus vetoed: {}", tx, reason);
  1399	                                            }
  1400	                                            _ => {}
  1401	                                        }
  1402	                                    }
  1403	                                    PartialVerdict::Reject(reason) => {
  1404	                                        let class = classify_lean_error(&reason);
  1405	                                        bus.record_rejection(agent_id, class.label());
  1406	                                        // PPUT-CCL B2: step rejection reason flows into next prompt.
  1407	                                        acc.record_tool_stdout(&reason);
  1408	                                        *tool_dist.entry("step_reject".into()).or_insert(0) += 1;
  1409	                                        let preview = reason.chars().take(200).collect::<String>();
  1410	                                        warn!("[tx {}] step rejected ({}): {}", tx, class.label(), preview);
  1411	                                    }
  1412	                                }
  1413	                            }
  1414	                        }
  1415	                        other => {
  1416	                            *tool_dist.entry(format!("other:{}", other)).or_insert(0) += 1;
  1417	                        }
  1418	                    },
  1419	                    Err(e) => {
  1420	                        *tool_dist.entry("parse_fail".into()).or_insert(0) += 1;
  1421	                        // Step-B v3: parse failures feed the class graveyard too.
  1422	                        let class = classify_parse_error(&format!("{}", e));
  1423	                        bus.record_rejection(agent_id, class.label());
  1424	                        // PPUT-CCL B2: classifier label flows into next prompt's errors.
  1425	                        acc.record_tool_stdout(class.label());
  1426	                        warn!("[tx {}] parse: {} ({})", tx, e, class.label());
  1427	                    }
  1428	                }
  1429	            }
  1430	            Err(e) => {
  1431	                *tool_dist.entry("llm_err".into()).or_insert(0) += 1;
  1432	                warn!("[tx {}] LLM: {}", tx, e);
  1433	            }
  1434	        }
  1435	    }
  1436	
  1437	    let upr = if omega_attempts > 0 {
  1438	        Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
  1439	    } else { None };
  1440	    // Phase 4: also save wallet state on no-OMEGA exit. Agents may have
  1441	    // invested/lost Coin during the run; durability should not depend on a win.
  1442	    if let Some(ref wp) = wallet_state_path {
  1443	        if let Some(w) = bus.tools.iter()
  1444	            .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
  1445	        {
  1446	            let _ = w.save_to_disk(wp);
  1447	        }
  1448	    }
  1449	    // No OMEGA found → PPUT = 0
  1450	    // B3: close bracket on max-tx exhaustion path.
  1451	    // P0-A: max-tx exhaustion → neither leg fired.
  1452	    // A4: this is the canonical hit_max_tx=true site (ran the full
  1453	    // for-loop without OMEGA and without firing the synthetic
  1454	    // short-circuit, which would have returned earlier).
  1455	    wc.mark_final_accept();

exec
/bin/bash -lc "nl -ba src/drivers/llm_proxy.py | sed -n '228,255p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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
   247	    return "dashscope"
   248	
   249	
   250	def strip_provider_prefix(model):
   251	    """If model is `provider:foo/bar`, return `foo/bar`; else `model`."""
   252	    if ":" in model:
   253	        prefix, rest = model.split(":", 1)
   254	        if prefix.lower() in PROVIDERS:
   255	            return rest

exec
/bin/bash -lc "rg -n \"fc_trace::emit_event\\(|make_pput\\(\" experiments/minif2f_v4/src/bin/evaluator.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
386:                return make_pput(problem_file, "oneshot", model,
411:            minif2f_v4::fc_trace::emit_event(
441:                    make_pput(problem_file, "oneshot", model,
453:                    make_pput(problem_file, "oneshot", model,
465:                    make_pput(problem_file, "oneshot", model,
723:            minif2f_v4::fc_trace::emit_event(
733:            let mut result = make_pput(problem_file, &condition, &run_model_label,
775:            minif2f_v4::fc_trace::emit_event(
1017:                                minif2f_v4::fc_trace::emit_event(
1041:                                        minif2f_v4::fc_trace::emit_event(
1113:                                        minif2f_v4::fc_trace::emit_event(
1141:                                        return make_pput(problem_file, &condition, &run_model_label,
1318:                                minif2f_v4::fc_trace::emit_event(
1358:                                        minif2f_v4::fc_trace::emit_event(
1370:                                        return make_pput(problem_file, &condition, &run_model_label,
1460:    minif2f_v4::fc_trace::emit_event(
1470:    make_pput(problem_file, &condition, &run_model_label,
1484:fn make_pput(
1773:        let result = make_pput(
1838:        let result = make_pput(
1881:        let result = make_pput(
1939:        let mut result = make_pput(

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '1450,1555p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1450	    // B3: close bracket on max-tx exhaustion path.
  1451	    // P0-A: max-tx exhaustion → neither leg fired.
  1452	    // A4: this is the canonical hit_max_tx=true site (ran the full
  1453	    // for-loop without OMEGA and without firing the synthetic
  1454	    // short-circuit, which would have returned earlier).
  1455	    wc.mark_final_accept();
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
  1474	              Some(acc.total_run_token_count()),
  1475	              Some(acc.failed_branch_count),
  1476	              wc.elapsed_ms(),
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
  1493	    total_run_token_count: Option<u64>,
  1494	    failed_branch_count: Option<u32>,
  1495	    total_wall_time_ms: Option<u64>,
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
  1531	    let classifier_version = std::env::var("CLASSIFIER_VERSION").ok();
  1532	    let boltzmann_seed = std::env::var("BOLTZMANN_SEED")
  1533	        .ok().and_then(|s| s.parse::<u64>().ok());
  1534	
  1535	    // Mid-term audit P0-B fix 2026-04-25: collapse Optional accumulator/clock
  1536	    // values into required v2 fields. Phase B always has values for these
  1537	    // (B2 + B3 wire them at every emit site); the prior Option wrapping was
  1538	    // overly defensive and let the v2 schema slip from the contract.
  1539	    let c_i = total_run_token_count.unwrap_or(0);
  1540	    let t_i = total_wall_time_ms.unwrap_or(0);
  1541	    let failed_count = failed_branch_count.unwrap_or(0);
  1542	
  1543	    let progress_runtime = compute_progress_runtime(runtime_accepted);
  1544	    let progress_verified =
  1545	        compute_progress_verified(runtime_accepted, post_hoc_verified);
  1546	    let pput_runtime = compute_pput(progress_runtime, c_i, t_i);
  1547	    let pput_verified = compute_pput(progress_verified, c_i, t_i);
  1548	    let pput_m_verified = compute_pput_m(progress_verified, c_i, t_i);
  1549	
  1550	    // V2 fields read from env (per-process globals).
  1551	    let split = std::env::var("SPLIT").unwrap_or_else(|_| {
  1552	        eprintln!("[v2-emit] SPLIT env unset; defaulting to 'adaptation' \
  1553	                   (Phase B convention; pre-registration requires SPLIT \
  1554	                   for Phase C+ ablation runs)");
  1555	        "adaptation".to_string()

exec
/bin/bash -lc "git log --format='%h%x09%s%n%b%n---END---' -n 20 --" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
aaedc9d	A8e7: structural rewrite — split packet into current-state + history
FC-trace: FC3 (read-only / log subgraph — governance artifact
restructure) + Art. V.1.2 (ArchitectAI commit authority on
non-constitution files). Per case C-075 the audit history is a
constitutional artifact and lives in Trust Root.

Ultrathink constitutional alignment (per user directive 2026-04-26):
rounds 2–6 of the Phase A → B audit produced a recurring documentary
CHALLENGE pattern that single-point fixes could not converge.
Diagnosis is a CATEGORY ERROR in document design — A8_EXIT_PACKET
was conflating two opposite temporal modes:

  (1) Phase A→B exit constitutional artifact — stable snapshot,
      every code change forces full-document sync.
  (2) Multi-round audit retrospective log — append-only chronology,
      historical sections frozen.

Sharing one document means every fix to (1)'s "current state"
generates new staleness in (2)'s "round-N retrospective" — observed
empirically across rounds 2/3/4/5/6 (each closed ~50% of findings
but introduced ~30% new staleness elsewhere).

Constitutional precedent for the split: the project already uses
this exact pattern elsewhere:
  - constitution.md (frozen) + Art. V.3 amendment log (chronology)
  - PREREG_PPUT_CCL_2026-04-26.md (frozen, round-4 PASS/PASS) +
    PREREG_AMENDMENT_p0_defer_2026-04-25.md (delta)
  - TRACE_MATRIX_v0 (immutable) + _v1 (delta) + _v2 (delta)

A8e7 mirrors this:

NEW  handover/audits/A8_AUDIT_HISTORY_2026-04-26.md
     Append-only chronology of rounds 1–6 outcomes + A8e..A8e6 fixes
     shipped. Each round entry is FROZEN; corrections go in a new
     "Correction (round-M)" sub-section, never via in-place edit.
     Includes round-by-round verdict table + cumulative metrics.

REWRITE  handover/audits/A8_EXIT_PACKET_2026-04-26.md
     Canonical current-state snapshot. § 1 scope/atom-map, § 2
     current metrics, § 3 per-atom acceptance evidence (post-A8e6
     state), § 4 Phase B → C exit checklist, § 5 known limitations,
     § 6 fresh round-7 reviewer questions. NO round-N retrospective
     text, NO fix-shipped sections, NO cross-round staleness sources.

Both docs cross-reference but don't duplicate. Audit runners feed
both to reviewers (history is read-only context for closure
verification; packet is the live artifact under review).

Trust Root manifest 34 → 35 (A8_AUDIT_HISTORY added). Per C-075
the gate machinery itself must be tamper-evident — silently
rewriting "what happened in round N" would constitute governance
drift.

Verification:
- cargo test --workspace: 265 PASS / 29 ignored / 0 failed
- boot::tests::verify_trust_root_passes_on_intact_repo: PASS at 35
- trust_root_immutability tests: 4/4 PASS
- python3 scripts/test_llm_proxy.py: 16/16 PASS
- bash scripts/smoke_siliconflow.sh: PASS (3/3 keys; previous run)

Round 7 dual audit launches from this commit. Expected to verify:
  - The category error is structurally absent (Q5 questions in packet)
  - No new substantive findings (atoms A0–A7 are unchanged)
  - History doc is internally consistent + matches actual transcripts

Ultrathink decision rationale: per Art. V.1.3 / case C-072, the
strict Veto-AI gate is `{PASS, VETO}` only. R1–R6 had no VETO
post-A8e (R1's VETO closed by F2/G1). The CHALLENGE merge rule is
operational discipline (memory feedback_dual_audit_conflict),
stricter than constitutional minimum. At the constitutional gate
the project has cleared 5 consecutive rounds; the remaining work is
restoring auditability at the operational-discipline level. The
structural rewrite is the only fix-class that addresses the root
cause; continuing single-point iteration is bounded-cost wasteful
(per memory feedback_routines_entropy "routines must earn their
cost") and possibly non-convergent.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

---END---
dbcf53a	A8e6: round-6 fixes (1 real routing bug + 4 doc; Codex CHALLENGE / Gemini CHALLENGE)
FC-trace: FC1-N7 (delta provider routing — K2 real correctness fix)
+ meta governance (K1/K3/K4/K5 doc).

Round-6 audit verdict (per memory feedback_dual_audit_conflict):
- handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R6.md: CHALLENGE / high
- handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R6.md: CHALLENGE / high
- Merged: CHALLENGE. Codex caught a REAL routing bug (deepseek-ai/...
  slash-form misroutes to api.deepseek.com); Gemini caught 2 narrow
  doc defects.

K1 (Codex R6#1): packet placeholder closure
- A8e5 J2 bullet + RQ14 used `<pending>` as inline reference text
  but RQ14's closure criterion said "the only acceptable matches
  are inside the round-5 outcome paragraph" — self-contradicting.
- Reworded both to refer to "placeholder commit SHAs" generically;
  RQ14 acceptance text matches the actual remaining matches.

K2 (Codex R6#2 — REAL BUG): detect_provider routing precedence
- HuggingFace-style `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B` is a
  SiliconFlow-catalog model. Pre-K2 detect_provider checked
  `"deepseek" in model.lower()` BEFORE the `/` slash check, so
  these IDs misrouted to api.deepseek.com (which only serves bare
  deepseek-chat / deepseek-v4-flash and would 404 on Distill).
- K2 swaps the order: slash-form is now the FIRST heuristic (after
  explicit `provider:...` prefix), bare-deepseek substring only
  fires when there's no slash.
- New test test_deepseek_slash_form_routes_to_siliconflow in
  scripts/test_llm_proxy.py pins this regression: covers
  deepseek-ai/DeepSeek-R1-Distill-Qwen-7B + deepseek-ai/DeepSeek-V2.5
  routing to siliconflow, AND bare-form deepseek-chat /
  deepseek-v4-flash still routing to deepseek.
- 16 / 16 Python proxy tests PASS (was 15 before K2).
- No production failure observed because A7 smoke uses Qwen/... not
  deepseek-ai/...; Phase D heterogeneous would have hit it on first
  invocation.

K3 (Codex R6#3): TRACE_MATRIX make_pput arg count
- A8e3 row said "make_pput signature is now 21 positional args"
  (Gemini R3 finding 2). A8e5 J5 corrected the packet to 24 (after
  F1 added run_id), but TRACE_MATRIX wasn't updated.
- TRACE_MATRIX A8e3 row now says "was 21 at round-3; F1 added run_id,
  bringing post-A8e count to 24 (Gemini R3 + Gemini R5 + Codex R6
  spot-checks; A8e6 K3 corrected here)".

K4 (Gemini R6#1): TRACE_MATRIX A6 anchor count summary
- A6 paragraph led with "Six anchor sites wired" then immediately
  said "A8e fix F4 added 3 more ... bringing the post-A8e total
  to 9". Self-contradictory at the sentence level.
- Now leads with "9 anchor sites wired (post-A8e total — 8 in
  run_swarm + 1 in run_oneshot)" and breaks down the original-6
  + F4-added-3 explicitly.

K5 (Gemini R6#2): packet § 3 fix attribution
- A5 description said "A8e3 fix H4 corrected the prior 25→26 claim";
  A6 said "Same off-by-one correction as A5's delta".
- H4 corrected the genesis_payload.toml TR header comment text;
  I2 (A8e4 round-5) corrected the per-atom delta numbers in THIS
  packet § 3.
- Both attributions now read "A8e4 fix I2 corrected".

Trust Root manifest unchanged at 34 (re-hashes of llm_proxy.py +
scripts/test_llm_proxy.py; no new files).

Verification:
- cargo test --workspace: 265 PASS / 29 ignored / 0 failed
- python3 scripts/test_llm_proxy.py: 16/16 PASS (+1 K2 test)
- bash scripts/smoke_siliconflow.sh: PASS (3/3 keys)
- boot::tests::verify_trust_root_passes_on_intact_repo: PASS at 34
- trust_root_immutability tests: 4/4 PASS
- llm_proxy_python_conformance wrapper: PASS

Round-6 audit transcripts (_R6) committed with this fix bundle.

Trajectory: R1 VETO/CHALLENGE → R2/3 CHALLENGE/CHALLENGE → R4/5
CHALLENGE/PASS → R6 CHALLENGE/CHALLENGE → R7 (pending). The R6
re-flag from Gemini was a -1 step on the convergence axis but
both findings were narrow + closeable; Codex's K2 bug catch was
significant (a real routing correctness issue that would have
silently broken Phase D heterogeneous on first deepseek-ai/...
invocation).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

---END---
1622017	A8e5: round-5 packet self-consistency (Codex CHALLENGE / Gemini PASS -> 5 fixes)
FC-trace: meta governance (packet self-consistency).

Round-5 audit verdict (per memory feedback_dual_audit_conflict):
- handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R5.md: CHALLENGE / high
- handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R5.md: PASS / high (PROCEED)
- Merged: CHALLENGE. Same split as R4. Both auditors agree code is
  sound; Codex keeps catching procedural drift in the packet.

Five doc/source-text fixes; zero Rust source-code changes:

J1 (Codex R5#1): Round-4 outcome + A8e4 fixes shipped section
- The packet was missing the round-4 outcome paragraph and the
  A8e4 fixes shipped section — round-3 outcome was the bottom-most
  retrospective. Now appended (Round-4 outcome + Round-5 outcome +
  A8e5 self-reference) just before the closing § 1 divider.

J2 (Codex R5#1): commit SHA placeholders
- A8e2 fixes shipped header said `<pending>` → 0af47b7
- A8e3 fixes shipped header said `<pending>` → 3d38ba5
- A8e4 fixes shipped header said `<pending>` → 8693789
- A8e5 self-reference uses "(this commit)" instead of `<pending>`
  to break the placeholder-staleness pattern that bit us in R4 + R5.

J3 (Codex R5#2): § 6 Q6 question text
- "24 → 30" + "6 new entries" was the round-1 question; never
  refreshed for the corrected count. Now reads "24 → 34" with the
  full 10-entry list (including A3's agent_models.rs that the
  round-1 packet undercounted, plus the 4 entries added by A8e/A8e2).

J4 (Codex R5#2): TRACE_MATRIX top-bullet TR-deltas
- The per-atom paragraph headers in TRACE_MATRIX § 1 still said:
  - A5 "Trust Root manifest 25 → 26" → corrected to "26 → 27"
  - A6 "Trust Root manifest 26 → 27" → corrected to "27 → 28"
  - A7 phrasing tweaked for consistency.
- All match the canonical chain in genesis_payload.toml header:
  A0=24 → A1=25 → A3=26 → A5=27 → A6=28 → A7=31 → A8e=33 → A8e2=34.

J5 (Gemini R5 non-blocking): make_pput arg count
- Round-3 retrospective text said "21 positional args"; A8e fix F1
  added the run_id parameter, bringing the post-A8e count to 24.
- Updated to "was 21 at round-3 ... now 24". Refactor to builder
  pattern still deferred to Phase B+.

Trust Root manifest unchanged at 34 (no new files; doc-only).

Verification:
- cargo test --workspace: 265 PASS / 29 ignored / 0 failed
- boot::tests::verify_trust_root_passes_on_intact_repo: PASS
- Trust Root entries: 34 (matches genesis_payload.toml header chain
  and the required-paths list in trust_root_immutability test).

Round-5 audit transcripts (_R5) committed with this fix bundle.

Trajectory: R1 VETO/CHALLENGE → R2 CHALLENGE/CHALLENGE → R3
CHALLENGE/CHALLENGE → R4 CHALLENGE/PASS → R5 CHALLENGE/PASS → R6
(pending). Both auditors aligned on PROCEED for the code; the
remaining work is closing the procedural gap between the multi-round
packet and the actual repo state. Each round shrinks the finding
set by ~50%.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

---END---
8693789	A8e4: round-4 documentary cleanup (Codex CHALLENGE / Gemini PASS -> 3 fixes)
FC-trace: meta governance (packet self-consistency).

Round-4 audit verdict (per memory feedback_dual_audit_conflict, VETO >
CHALLENGE > PASS — split verdict, conservative merge wins):
- handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R4.md: CHALLENGE / high
- handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R4.md: PASS / high
- Merged: CHALLENGE. Gemini gave PROCEED; Codex caught 3 narrow
  documentary defects in the packet itself that I missed in A8e3.

Three fixes (all documentary; zero source-code changes apart from
docstring text):

I1 (Codex R4#1): packet metadata stale "round 2"
- A8 packet line 1 title was "round 2"; date said "round 2, post
  A8e fixes". Stale through 4 rounds of fixes.
- Title now "running through rounds 1–N" with explicit reader
  pointer to "Round-N outcome" + "Round-N fixes shipped" sections
  for the latest verdict / delta.

I2 (Codex R4#1 + R4#2): per-atom Trust Root deltas + anchor location
- Packet § 3 A5 said "Trust Root manifest 25 → 26"; actual chain
  per genesis_payload.toml header is 26 → 27 (off-by-1 because A3's
  agent_models.rs had already raised the count to 26 before A5).
- Packet § 3 A6 said "26 → 27"; actual is 27 → 28.
- Packet § 3 A7 said "27 → 30"; actual is 28 → 31.
- All three corrected with cross-ref to the corrected milestone
  chain in genesis_payload.toml header.
- TRACE_MATRIX § 1 line 11 said "Six anchor sites wired in run_swarm"
  but 1 site (oneshot verify) is actually in run_oneshot. Reworded
  to "Six anchor sites wired (5 in run_swarm + 1 in run_oneshot)".
- Same correction in § 5 item 7 closure note.

I3 (Codex R4#3): wrapper docstring stale "skip"
- experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs file
  header still said "if not, it skips with a clear diagnostic" — but
  A8e3 H6 changed the runtime to FAIL CLOSED.
- Docstring corrected: "if not, it FAILS CLOSED with a clear
  diagnostic (A8e3 fix H6 — Codex round-3 caught the prior soft-skip
  behavior)".
- Explicit opt-out env var documented in the docstring too.
- Re-hashed in Trust Root.

Trust Root manifest unchanged at 34 (only re-hash of the wrapper;
no new files).

Verification:
- cargo test -p turingosv4 --lib --tests: 152 PASS / 9 ignored / 0 failed
- cargo test -p minif2f_v4 --lib --tests --bins: 113 PASS / 20 ignored / 0 failed
- Cumulative Rust: 265 PASS / 29 ignored / 0 failed (unchanged from A8e2/A8e3)
- python3 scripts/test_llm_proxy.py: 15/15 PASS via cargo wrapper
- bash scripts/smoke_siliconflow.sh: PASS (3/3 keys)
- boot::tests::verify_trust_root_passes_on_intact_repo: PASS at 34

Round-4 audit transcripts (_R4) committed with this fix bundle for
round-5 reviewers to verify the closures.

Trajectory: round-1 VETO → round-2 CHALLENGE/CHALLENGE → round-3
CHALLENGE/CHALLENGE → round-4 CHALLENGE/PASS → round-5 (pending,
launched after this commit). Each round shrinks the finding count;
defects became progressively more procedural / cosmetic.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

---END---
3d38ba5	A8e3: round-3 dual-audit cleanup (Codex CHALLENGE / Gemini CHALLENGE -> 6 fixes)
FC-trace: meta governance (packet self-consistency) + V3L-27 mitigation
gating (FC1-N7 — H6 fail-closed wrapper).

Round-3 audit verdict (per memory feedback_dual_audit_conflict):
- handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R3.md: CHALLENGE / high
- handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R3.md: CHALLENGE / high
- Merged: CHALLENGE. Both auditors confirmed code is sound + ready for
  Phase B; only the packet itself failed final-pass rigor.

Six narrow fixes (5 documentary + 1 runtime hardening):

H1 (Codex R3#1): A8 packet § 3 A1 atom description
- "Mathematically conservative (strictest plausible bar; no Type-I
  inflation)" → explicit "least-strict admissible value" wording
  with full statistical implications + cross-ref to PREREG_AMENDMENT
  § 2 wording correction (A8e F6).

H2 (Codex R3#2 + Gemini R3#1): A8 packet § 3 A6 atom description
- "6 wired anchor sites + 1 oneshot" → "9 wired anchor sites total"
  with explicit list of the 3 swarm-side FC1-N12 sites added by F4
  (verify_omega_detailed × 2 paths + verify_partial).

H3 (Codex R3#1): A8 packet § 6 stale questions
- Q2.a marked CLOSED with closure rationale (substitution is
  least-strict admissible, not conservative).
- Q4.a marked PARTIALLY CLOSED (anchor count 6 → 9; FC1-N12 covers
  swarm path; remaining 4 unwired enum variants are Phase B+ work).
- Q4.d marked CLOSED with closure rationale (run_corr_id renamed
  to run_id, lifted to run_id.rs::mint_run_id, unified across emit
  + jsonl).

H4 (Codex R3#1): genesis_payload.toml Trust Root header
- "conservative ceiling" → "max-tolerated ceiling — least-strict
  admissible per amendment § 2 wording correction A8e fix F6".

H5 (Codex R3#2): TRACE_MATRIX § 5 item 7
- "(commit pending)" → "CLOSED"; "6 wired anchor sites" → "9 wired
  anchor sites" (A6 shipped 7 + A8e F4 added 3 — wait that's 10...
  actually A6 had 6 + 1 oneshot = 7 emit sites; A8e F4 added 3 = 10
  total emit calls. Let me recount in the actual code: synthetic +
  mr tick + OMEGA full-proof + OMEGA per-tactic + max-tx + oneshot
  verify + 2 verify_omega_detailed + verify_partial = 9 distinct
  anchor SITES, with the 2 verify_omega_detailed sharing the same
  conceptual anchor but different code paths. The packet's "9" is
  the production-emit-site count, which is the number that matters
  for Phase D consumers).

H6 (Codex R3#3): G1 wrapper test fail-closed
- experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs:
  missing python3 was a soft skip via eprintln + early return. That
  was exactly the silent-pass failure mode Gemini's round-1 VETO
  targeted — "test that doesn't run automatically is documentation".
- Now panics with diagnostic message when python3 is missing.
- Explicit opt-out: SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1 (logged
  loudly so the bypass is visible in test output).
- Verified fail-closed: PATH=/tmp <test_bin> panics; PATH=/tmp
  SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1 <test_bin> passes with the
  bypass message printed.
- Re-hashed in Trust Root.

Non-blocking observation deferred to Phase B+ (Gemini R3 Finding 2):
make_pput signature is now 21 positional args. Refactor to a builder
pattern (e.g. PputResultBuilder struct) is recommended but does NOT
gate Phase A → B exit. Tracked in this commit message + the packet's
round-3 fixes section.

Trust Root manifest unchanged at 34 entries (only re-hashed
llm_proxy_python_conformance.rs; no new files added).

Verification:
- cargo test -p turingosv4 --lib --tests: 152 PASS / 9 ignored / 0 failed
- cargo test -p minif2f_v4 --lib --tests --bins: 113 PASS / 20 ignored / 0 failed
- Cumulative Rust: 265 PASS / 29 ignored / 0 failed (unchanged from A8e2)
- python3 scripts/test_llm_proxy.py: 15/15 PASS via cargo wrapper
- bash scripts/smoke_siliconflow.sh: PASS (3/3 keys responding)
- boot::tests::verify_trust_root_passes_on_intact_repo: PASS at 34 entries
- H6 negative test (PATH=/tmp): wrapper panics with diagnostic
- H6 opt-out test (SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1): wrapper
  passes with loud bypass message

Round-3 audit transcripts (_R3) committed alongside this A8e3 fix
bundle so they are version-controlled before round-4 launches.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

---END---
0af47b7	A8e2: round-2 dual-audit fixes (Codex CHALLENGE / Gemini CHALLENGE -> 3 fixes)
FC-trace: meta governance + V3L-27 mitigation gating (FC1-N7).

Round-2 audit verdict (per memory feedback_dual_audit_conflict):
- handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R2.md: CHALLENGE / high
- handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R2.md: CHALLENGE / high (de-escalated from VETO)
- Merged: CHALLENGE. No VETO this round — both auditors confirmed
  the round-1 fixes are letter-correct; remaining gaps are procedural
  / documentary, not architectural.

Three fixes:

G1 (Codex R2#1 + Gemini R2#1): recurring conformance gate
- experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs — Rust
  integration test that shells to `python3 scripts/test_llm_proxy.py`
  + asserts exit 0 + checks for the unittest "OK" trailer.
- The Python suite (15 tests pinning round-robin [2,2,2] + routing
  matrix) now runs on every `cargo test --workspace` invocation —
  no longer "manual only".
- Closes Gemini's round-1 VETO ("test that doesn't run automatically
  is documentation, not a gate") at the procedural level.
- Falls back to soft skip if python3 not on PATH (visible eprintln
  rather than silent pass).

G2 (Codex R2#2): PREREG_AMENDMENT § 8 stale claim
- Round-1 fix corrected § 2 ("strictest possible substitute" was
  backwards — 0.10 is the LEAST-strict admissible ceiling for
  j-RR <= p_0). But § 8 audit-requirements paragraph still said
  "strictest plausible bar is conservative", directly contradicting
  the corrected § 2.
- § 8 reworded: explicit "j-RR is descriptive (PREREG § 5.4),
  outside the inferential family, so no Type-I inflation. Per § 2
  wording correction: 0.10 is the LEAST-strict admissible ceiling,
  NOT a tighter-than-original substitute". The amendment is now
  consistent across the doc.
- Re-hashed in Trust Root.

G3 (Codex R2#3 + Gemini R2#2): A8 packet + TRACE_MATRIX staleness
- Packet § 2 cumulative-tests table extended with A8e + A8e2 columns
  (counts: 261 -> 264 -> 265).
- Packet § 4 Trust-Root-clean assertion bumped 30 -> 34.
- Packet § 5 Risk #5 ("No FC1-N12 emit in run_swarm verify path")
  REMOVED (closed by A8e fix F4; both auditors flagged the staleness).
- Packet § 6 Q7.a 261 -> 265, Q7.b 30 -> 34.
- Packet appended: round-2 outcome paragraph + round-3 fixes
  paragraph + 4 round-3 questions (RQ6-RQ9).
- TRACE_MATRIX_v2 § 2 stale `run_corr_id` symbol row replaced with
  unified `run_id::mint_run_id` row covering all 9 anchor sites
  (oneshot + 4 swarm sites from A6 + 3 swarm verify from A8e F4).
- TRACE_MATRIX_v2 F2 description softened from "exercises the
  V3L-27 mitigation in CI" to "without a live API; A8e2 G1 wires
  this into cargo test --workspace via a Rust shell-out wrapper".
- TRACE_MATRIX_v2 § 1 trigger summary: A8e2 row added.
- TRACE_MATRIX_v2 § 3 manifest size milestones: A8e2 -> 34.
- genesis_payload.toml header reconciled to "Total: 34 files" with
  full progression A0=24 -> A1=25 -> A3=26 -> A5=27 -> A6=28 ->
  A7=31 -> A8e=33 -> A8e2=34.

Trust Root manifest 33 -> 34 (llm_proxy_python_conformance.rs added;
PREREG_AMENDMENT re-hashed). trust_root_immutability test
required-paths list extended.

A8 audit runners updated: round-2 transcripts saved with _R2 suffix
(A8_AUDIT_ROUND env var; default R2 for round-3 launch).

Verification:
- cargo test -p turingosv4 --lib --tests: 152 PASS / 9 ignored / 0 failed
- cargo test -p minif2f_v4 --lib --tests --bins: 113 PASS / 20 ignored / 0 failed
- Cumulative Rust: 265 PASS / 29 ignored / 0 failed (was 264; +1 G1 wrapper)
- python3 scripts/test_llm_proxy.py: 15/15 PASS (also runs via cargo test now)
- bash scripts/smoke_siliconflow.sh: PASS (3/3 keys)
- boot::tests::verify_trust_root_passes_on_intact_repo: PASS at 34 entries

Round-2 audit transcripts (_R2) committed alongside this A8e2 fix
bundle so they are version-controlled before round-3 launches.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

---END---
5a56ff6	A8e: round-1 dual-audit fixes (Codex CHALLENGE / Gemini VETO -> 6 fixes)
FC-trace: FC1-N12 (oracle scope swarm coverage F4) + FC1-N7 (delta
provider routing F3) + Art. V (amendment governance F6).

Round-1 audit verdict (per memory feedback_dual_audit_conflict, VETO >
CHALLENGE > PASS):
- handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26.md: CHALLENGE / high
- handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26.md: VETO / high
- Merged: VETO. FIX-THEN-PROCEED cycle.

Six fixes:

F1 (Codex#2 + Gemini Q4): unify run_id across fc_event + jsonl
- New module experiments/minif2f_v4/src/run_id.rs mints one stable
  identifier per run at run_swarm/run_oneshot entry.
- Threaded into make_pput as new param (replaces internal recompute).
- All 9 make_pput call sites + 4 test fixtures updated.
- Oneshot FC1-N12 now stamps the unified run_id (was bogus
  oneshot_{problem_file} placeholder).
- 3 unit tests in run_id::tests pin the format shape.

F2 (Gemini VETO Q6): round-robin conformance test
- scripts/test_llm_proxy.py — 15-test Python suite, no live API.
- 3-key pool: 6 calls -> [2,2,2] (the documented A7 invariant).
- Single-key pool: degrades to all-zero index.
- Two-key pool: alternates.
- Stats reset zeros per_key_requests.
- Exercises the V3L-27 / case C-027 mitigation in CI.

F3 (Codex#4): provider routing matrix
- src/drivers/llm_proxy.py detect_provider() rewritten:
  slash-form ⇒ siliconflow FIRST; bare-qwen ⇒ dashscope only when no slash.
- Was misrouting Qwen/Qwen2.5-7B-Instruct to dashscope because
  m.startswith("qwen") won after the slash check.
- Routing matrix pinned in test_llm_proxy.py: explicit prefix /
  deepseek-substring / Qwen-HF / openai-HF / meta-HF / bare-qwen /
  unknown-fallback.

F4 (Codex#3): swarm FC1-N12 emit
- Round-1 emitted FC1-N12 only in run_oneshot. The 2 verify_omega_detailed
  paths (alone + tape+payload retry) and the verify_partial path in
  run_swarm now each emit FC1-N12 with verdict + elapsed_ms + path stamp.
- Closes the swarm-side gap in the 5-step compile-loop meta-witness.

F5 (Codex#5): Trust Root count consistency
- Round-1 packet claimed 30 entries; actual is 31 (A3's agent_models.rs
  was already in TR before A1 PREREG amendment). genesis_payload.toml
  header still said "Total: 25 files" (pre-A0).
- Header reconciled: A0=24 -> A1=25 -> A3=26 -> A5=27 -> A6=28 ->
  A7=31 -> A8e=33. TRACE_MATRIX_v2 milestones updated.

F6 (Codex#1): PREREG_AMENDMENT § 2 wording
- Round-1 amendment text claimed p_0=0.10 was the "strictest possible
  substitute". Mathematically backwards: Gate H is j-RR <= p_0, so a
  SMALLER p_0 is stricter. 0.10 is the maximum tolerated value =
  least strict admissible ceiling.
- Reworded to clarify: substitution is operationally permitted but NOT
  a tighter-than-original guarantee. Statistical-implications paragraph
  added: no Type-I inflation (j-RR descriptive); may be less protective
  than eventual calibrated p_0 < 0.10; acceptable because Gate H is
  Phase E and § 3 conditions ensure calibration runs before Phase E.

Trust Root manifest 31 -> 33 (run_id.rs + test_llm_proxy.py added;
PREREG_AMENDMENT + evaluator.rs + llm_proxy.py re-hashed).

trust_root_immutability test required-paths list extended with the
two new entries; verifies clean post-fix.

Verification:
- cargo test -p turingosv4 --lib --tests: 152 PASS / 9 ignored / 0 failed
- cargo test -p minif2f_v4 --lib --tests --bins: 112 PASS / 20 ignored / 0 failed
- Cumulative Rust: 264 PASS / 29 ignored / 0 failed (was 261; +3 run_id tests)
- python3 scripts/test_llm_proxy.py: 15/15 PASS
- bash scripts/smoke_siliconflow.sh: PASS (3/3 keys responding)

Round-1 audit transcripts (Codex + Gemini) committed alongside this
A8e fix bundle so they are version-controlled before round-2 launches.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

---END---
60292dc	A8 prep: Phase A→B exit audit packet + Codex/Gemini runners
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

---END---
90953d6	A7: SiliconFlow heterogeneous-LLM provider plumbing (proxy + 3-key smoke)
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

---END---
89994c7	A6: per-line FC tagging via structured JSON events (fc_trace module)
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

---END---
30f2a14	A5: per-agent budget normalization (BUDGET_REGIME + MAX_TRANSACTIONS env vars)
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

---END---
a5c78e4	A4: decomposed metrics (hit_max_tx + tactic_diversity + verifier_wait_ms)
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

---END---
7f4bc0c	A3: per-agent AGENT_MODELS env var (Phase B+C single-model gate)
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

---END---
180a300	A2: P0a swarm_N=1 mode + parse_swarm_condition_n unit tests
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

---END---
6be6eb4	A1: PREREG amendment defer p_0 calibration + Trust Root 24->25
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

---END---
62c4e14	A0e: Phase A0 exit audit (CHALLENGE/CHALLENGE) + 7-item fixes
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

---END---
e94e1b9	A0d: TRACE_MATRIX_v2 + Trust Root manifest 20 → 24 (harness in TR)
FC-trace: FC3-S3 (readonly subgraph extension) + Art. V.1 (3-tier guard) +
          case C-075 DO-178C tool qualification

Phase A0 atom 4: governance instrumentation as constitutional artifact.
Per case C-075, tools producing frozen data MUST be in Trust Root.
Phase A0a/b/c shipped 4 such tools — this atom adds them to manifest.

NEW TRUST ROOT ENTRIES (4)

1. rules/MANIFEST.sha256 (proxy for 14 rules/active/R-*.yaml)
   Same pattern as cases/MANIFEST.sha256: glob hashed once, manifest
   tracked in Trust Root. Tampering with R-018 enforcement field
   (block→warn) silently bypasses constitution sudo gate.

2. rules/engine.py
   The interpreter of the rules. Tampering with engine.py = silent
   rule bypass even with intact rule files.

3. .claude/hooks/judge.sh
   PreToolUse hook: invokes engine.py + implements R-016 fc_trace_in_commit
   + constitution.md special-case (the bug where *.md skip-list
   silently allowed constitution.md edits during this session — see
   commit c061450 incident, fixed in 2e7f75a).

4. tests/fc_alignment_conformance.rs
   Witness battery for TRACE_MATRIX ✅ rows (A0b deliverable).
   Tampering with witnesses = false PASS hides constitutional drift.

TRACE_MATRIX_v2 doc covers:
  § 1 — 17 ⚠️ → ✅ status flips via fc_alignment_conformance witnesses
  § 2 — Phase A0 new symbols (rules, hooks, conformance battery)
  § 3 — Trust Root expansion 20 → 24 with per-entry justification
  § 4 — 5 new cases C-071..C-075 cross-referenced to FC anchors
  § 5 — Open work for v3 (per-line FC tracing, R-020 ground_truth_label,
        HaltReason full taxonomy, etc.)
  § 6 — Updated counts: 16 → 33 ✅; 9 deferred unchanged

experiments/minif2f_v4/tests/trust_root_immutability.rs::test_trust_root_manifest_includes_b2_b4_files
extended to assert 4 new entries present (rules/MANIFEST.sha256 + engine.py
+ judge.sh + fc_alignment_conformance.rs).

VERIFICATION

cargo test --workspace: 204 PASS + 29 ignored (unchanged from A0c).
boot::tests::verify_trust_root_passes_on_intact_repo: PASS — 24-entry
manifest re-verifies clean.

Manifest size milestones documented in v2 § 6:
  B7 → 15 → B7-extra → 16 → audit-fix → 20 → A0 → 24
  (planned) B7-extra calibration freeze → 25
  (planned) Phase C mode-flag binary → 26+

Next: A0e — Phase A0 exit dual audit (Codex + Gemini) on the harness
  modernization itself: do A0a-d together constitute valid governance
  upgrade per Art. V.1.2 ArchitectAI authority + case C-073 workflow?

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

---END---
2a65339	A0c: 5 new cases C-071..C-075 sediment 2026-04-25 session decisions
FC-trace: FC3-N3 (constitution-as-ground-truth extension via case law) +
          Art. V.1 amendments + memory feedback_fc_first_problem_handling

Phase A0 atom 3: case-law sediment. CLAUDE.md "Common Law (宪法 + 判例)"
+ "宪法高度压缩, 具体裁决查 cases/C-xxx.yaml". Past-success memory:
"按条款查 grep -l 'Art. I.1' cases/*.yaml" — case-law is the operational
indexing layer over the constitution.

5 NEW CASES (each ~80-150 lines yaml: incident / facts / ruling /
precedent):

C-071 Constitution amendment process
  Locks 4-step sudo flow: (a) verbatim "I authorize this constitution
  amendment", (b) Art. V.3 log entry, (c) commit triggers genesis SHA-256
  recompute, (d) post-amendment dual audit. Written in response to this
  session's c061450 commit which silently amended Art. V.1 because
  judge.sh:54 *.md skip-list bypassed R-018 — fix in 2e7f75a.

C-072 Veto-AI scope narrowing (Art. V.1.3 amendment 2026-04-25)
  Output domain MUST be 2-class {PASS, VETO}. White-list excludes
  subjective quality / performance / readability evaluation. Written in
  response to round-2 dual audit divergence (Codex VETO real defect /
  Gemini PASS) — conservative wins per feedback_dual_audit_conflict.

C-073 ArchitectAI commit authority (Art. V.1.2 amendment)
  Scope = ALL files EXCEPT constitution.md. Required workflow: FC-trace
  + cargo test + commit message + (Trust Root involved) sha256 recompute
  + (Phase boundary) dual audit. PREREG amendments, manifest expansion,
  governance instrumentation (rules/, hooks/, tests/) all ArchitectAI
  scope. Validates this session's 19 commits flow.

C-074 FC-first problem handling (memory feedback_fc_first_problem_handling)
  STOP before patch. Trace problem to FC1/FC2/FC3 element. Step 2 branches:
  (a) anchor found → fix from anchor (b) orphan → cite Cxxx (c) no anchor
  → escalate. Commit message must contain `FC-trace:`. R-016 hook enforces.
  Written from 5 in-session symptom-fix-induced regressions.

C-075 DO-178C tool qualification expanded Trust Root
  Trust Root MUST contain tools producing frozen data. Tool tampering =
  silent claim corruption. Locks 16→20 manifest expansion (B7-extra
  round-1 fix) + reserves future expansion to rules/active/*.yaml +
  judge.sh + tests/fc_alignment_conformance.rs (A0d v2 plan).

cases/MANIFEST.sha256 recomputed (45→50 entries); SHA-256 updated:
  3759631a... → dbe48bc1...
genesis_payload.toml [trust_root] updated; verify_trust_root_passes_on_intact_repo
re-passes (6/6 boot tests + 4/4 trust_root_immutability tests + 17/17
fc_alignment_conformance tests).

cargo test --workspace: 204 passed + 29 ignored (unchanged from A0b).

Cross-references between cases form a tight DAG:
  C-071 ←→ C-073 (sudo vs ArchitectAI scope distinction)
  C-072 → C-073 (Veto-AI PASS gates ArchitectAI commit)
  C-074 → all (FC-first applies to every fix)
  C-075 → C-073 (manifest is ArchitectAI scope)

Next: A0d — TRACE_MATRIX_v2 covering session new symbols + scripts +
              future Trust Root expansion to rules/ + hooks/ + tests/.
      A0e — Phase A0 exit dual audit on the harness work.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

---END---
d8950ee	A0b: tests/fc_alignment_conformance.rs witness battery
FC-trace: FC1 + FC2 + FC3 (constitutional flowcharts) +
          CLAUDE.md Alignment Standard

Phase A0 atom 2: mechanical witness for TRACE_MATRIX_v1 ✅ rows.
Per CLAUDE.md "Conformance tests: tests/fc_alignment_conformance.rs
— 每个 ✅ 行 ≥1 witness test；#[ignore] stub 覆盖 📅 deferred rows".

The file was MISSING from main branch (only existed in
.claude/worktrees/phase-8a-snapshot/). TRACE_MATRIX was effectively
nonenforcable — symbol drift would only surface at next dual audit.
Now rename / removal of any FC-anchored symbol fails compile or
panics at `cargo test`.

17 PASSING witnesses for ✅ rows:
  FC1-N1  Q_t carrier (TuringBus, BusConfig types)
  FC1-N4  tape constructible + time-arrow
  FC1-N6  UniverseSnapshot input
  FC1-N7  ResilientLLMClient (δ/AI)
  FC1-N8/N9/N10  AgentAction parser (output ⟨q_o, a_o⟩)
  FC1-N11/N15/E18  forbidden_pattern veto preserves Q_t
                   (production-path ground-truth-feedback)
  FC1-N13 wtool via TuringBus::append
  FC2-N20/N27 tick (TICK_INTERVAL + emit_mr_tick_node bus capability)
  FC2-N22 halt_and_settle (constitutional halt entry)
  FC2-N23 EventType::OmegaAccepted (the canonical typed variant)
  FC3-E14 boot panic immediate-abort (OBS_BOOT_FAIL_NOT_HALT exists)
  FC3-N31 Wal type (logs archive)
  FC3-N34 verify_trust_root + parse_trust_root_section + TrustRootError
  FC3-N34 trust_root verify on intact repo passes (B7 readonly guard)
  FC3-N39 Ledger + Ledger::append + LedgerEvent
  FC3-S3  manifest size assertion (>=20 entries)

9 IGNORED stubs (each documents WHY deferred):
  FC2-N23 full HaltReason taxonomy (only OmegaAccepted typed today)
  FC3-N32 Veto-AI runtime (📅 Phase 11+; manual dual-audit covers)
  FC3-N33 ArchitectAI runtime (📅 Phase D)
  FC3-N40 logs → ArchitectAI feedback (📅 Phase D, see THESIS_V2 audit)
  FC3-N41 in-process re-init loop (📅 Phase 11+)
  FC3-E15/E16/E17 constitutional signaling (📅 Phase 11+)
  FC1-N11 Predicate trait + bus.register_predicate (🔨 Phase Z unmerged)
  FC2-N16 run_swarm/run_oneshot (binary-only, refactor needed)
  FC1-N12 Lean4Oracle (cross-crate, separate file in minif2f_v4/tests)

Verification: cargo test --workspace = 204 passed (was 187, +17) +
29 ignored (was 20, +9). Trust Root unchanged (test file added, no
manifest entry needed).

Next: A0c — sediment session decisions to cases/C-071..C-075.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

---END---
2e7f75a	A0a: 4 new harness rules + judge.sh constitution-special-case
FC-trace: FC3-S3 (readonly subgraph) + Art. V.1 (3-tier guard) +
          CLAUDE.md Alignment Standard + memory feedback_fc_first

Phase A0 atom 1: harness modernization. Existing rules R-001..R-013
designed Phase 8 (2026-04-13), only cover pre-B7 surface. Phase B7+
(Trust Root, rollback_sim, Veto-AI, Art. V amendments) had no
matching rules — enforcement.log silent during session despite ~30
material edits to new architectural surface.

NEW RULES

R-014 trust_root_manifest_drift_warn (file_glob *.rs, warn):
  Reminds operator on every .rs edit that 20-file Trust Root manifest
  needs SHA-256 recompute + genesis_payload.toml entry update if file
  is tracked. Prevents silent boot::verify_trust_root panic at next run.
  FC-trace: FC3-S3 readonly subgraph manifest consistency.

R-015 trace_matrix_pub_symbol_warn (file_glob *.rs, grep
  "pub (fn|struct|enum|trait|const|mod) ", warn):
  Reminds new pub symbols need /// TRACE_MATRIX FCx-Nx doc-comment +
  TRACE_MATRIX_v?.md entry. Catches the F-2026-04-25-04 failure mode
  (B7 ship pub symbols without backlinks; user critique forced
  retroactive fix in commit 0cc48bc).
  FC-trace: CLAUDE.md "每个 src/ pub 符号必须映射到宪法 flowchart 元素".

R-018 constitution_amendment_sudo (file_glob constitution.md, BLOCK):
  Hard-blocks edits to constitution.md unless user explicitly types
  "I authorize this constitution amendment" + adds Art. V.3 amendment
  log entry + commit triggers genesis SHA-256 recompute. Prevents
  silent constitution drift by ArchitectAI.
  FC-trace: Art. V.1.1 amendment 2026-04-25 (sudo only for
  constitution.md).

R-019 model_snapshot_canonical_warn (file_glob *, grep "deepseek-chat",
  warn):
  Catches the 2026-04-25 failure where 576-run batch was launched
  with ACTIVE_MODEL=deepseek-chat (alias) instead of PREREG-pinned
  deepseek-v4-flash (canonical). Alias may silently drift to
  different backend; canonical name pins identity.
  FC-trace: FC1-N7 (δ/AI canonical identity) + memory
  project_deepseek_drift_2026-04-24.

JUDGE.SH CHANGES

Special-case constitution.md BEFORE skip-list:
  Root cause of why R-018 wouldn't have fired during c061450 (this
  session's amendment): judge.sh:54 skip-list pattern '*.md' matched
  constitution.md before any rule could evaluate. Now constitution.md
  bypasses the skip-list and goes through engine.py for R-018 to
  trigger BLOCK exit 2.

R-016 fc_trace_in_commit (PreToolUse on Bash containing "git commit",
  WARN):
  Greps the git commit command for "FC-trace:" in message body.
  Surfaces missing trace as warning (not block — user decides). Per
  memory feedback_fc_first_problem_handling.

VERIFICATION

4 smoke tests pass:
  1. constitution.md edit → R-018 BLOCK exit 2 ✓
  2. git commit without FC-trace → R-016 WARN ✓
  3. git commit with FC-trace → silent pass ✓
  4. other .md files (LATEST.md, handover docs) still skipped ✓

cargo test --workspace: 187/187 PASS + 20 ignored unchanged
(yaml + sh edits, no Rust file modified, Trust Root manifest
unchanged).

Next: A0b — tests/fc_alignment_conformance.rs witness battery for
TRACE_MATRIX_v1 ✅ rows; A0c — sediment session decisions to
cases/C-071..C-075; A0d — TRACE_MATRIX_v2 covering all session
new symbols + scripts (DO-178C tool qualification scope) +
manifest expansion to include rules/active/*.yaml.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

---END---

exec
/bin/bash -lc 'git log --oneline --decorate -n 25' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
aaedc9d (HEAD -> main) A8e7: structural rewrite — split packet into current-state + history
dbcf53a A8e6: round-6 fixes (1 real routing bug + 4 doc; Codex CHALLENGE / Gemini CHALLENGE)
1622017 A8e5: round-5 packet self-consistency (Codex CHALLENGE / Gemini PASS -> 5 fixes)
8693789 A8e4: round-4 documentary cleanup (Codex CHALLENGE / Gemini PASS -> 3 fixes)
3d38ba5 A8e3: round-3 dual-audit cleanup (Codex CHALLENGE / Gemini CHALLENGE -> 6 fixes)
0af47b7 A8e2: round-2 dual-audit fixes (Codex CHALLENGE / Gemini CHALLENGE -> 3 fixes)
5a56ff6 A8e: round-1 dual-audit fixes (Codex CHALLENGE / Gemini VETO -> 6 fixes)
60292dc A8 prep: Phase A→B exit audit packet + Codex/Gemini runners
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

exec
/bin/bash -lc 'git log --oneline -- constitution.md | head -20' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
c061450 Constitutional amendment 2026-04-25 (sudo)
a76127c Formalize Common Law system: 35 cases, article IDs, V3L lesson index
967fac1 Initial commit: TuringOS v4 microkernel

exec
/bin/bash -lc "python3 - <<'PY'
from pathlib import Path
text=Path('genesis_payload.toml').read_text()
in_tr=False
n=0
keys=[]
for line in text.splitlines():
    s=line.strip()
    if s=='[trust_root]':
        in_tr=True; continue
    if in_tr and s.startswith('['):
        break
    if in_tr and s.startswith('\"') and '\" =' in s:
        n+=1; keys.append(s.split('\"',2)[1])
print(n)
print(keys[-5:])
PY" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
35
['handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json', 'handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md', 'handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md', 'experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs', 'handover/audits/A8_AUDIT_HISTORY_2026-04-26.md']

codex
The code-level closures for the prior real bugs look intact: run_id is threaded through emit/make_pput, slash-form routing now wins before bare deepseek, and the Python wrapper fails closed unless explicitly opted out. The remaining issues I’m seeing are contract/document consistency, not runtime code defects.
exec
/bin/bash -lc 'for f in cases/C-071_constitution_amendment_process.yaml cases/C-072_veto_ai_scope_narrowing.yaml cases/C-073_architect_ai_commit_authority.yaml cases/C-074_fc_first_problem_handling.yaml cases/C-075_do_178c_tool_qualification.yaml; do echo $f; nl -ba $f | sed -n '"'1,80p'; done" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
cases/C-071_constitution_amendment_process.yaml
     1	id: C-071
     2	title: "Constitution amendment process — sudo + V.3 log entry + manifest hash recompute"
     3	constitution:
     4	  - "Art. V.1.1 amendment 2026-04-25: sudo applies *only* to constitution.md"
     5	  - "Art. V.3 (NEW 2026-04-25): Constitutional Amendment Log"
     6	  - "Art. V.1.2 amendment: ArchitectAI has commit authority on non-constitution files"
     7	  - "FC3-N3 constitution-as-ground-truth + FC3-S3 readonly subgraph"
     8	source_lessons:
     9	  - "C-069 (constitutional alignment audit protocol): TRACE_MATRIX 是必须"
    10	  - "C-070 (preregistration discipline): governance precedes commit"
    11	
    12	incident: |
    13	  2026-04-25 session: 用户 sudo-authorize 3 处 Art. V 修订:
    14	  V.1.1 (sudo 范围窄化), V.1.2 (ArchitectAI commit 权限), V.1.3 (JudgeAI →
    15	  Veto-AI 重命名 + 范围窄化). 同时新增 Art. V.3 修订日志子节. Commit
    16	  c061450.
    17	
    18	  执行后发现 R-018 constitution_amendment_sudo (rules/active/) **未触发** —
    19	  原因: judge.sh:54 skip-list pattern '*.md' 在 engine.py 评估之前
    20	  silently skipped constitution.md. 即 ArchitectAI 当时 silently
    21	  amended ground truth file without governance gate.
    22	
    23	  Root cause: harness rule 集冻结于 Phase 8 (2026-04-13), 没规则覆盖
    24	  constitution.md. 新加 R-018 (2026-04-25 commit 2e7f75a) + judge.sh
    25	  special-case constitution.md before skip-list 才修好.
    26	
    27	facts: |
    28	  Art. V.1.1 amendment 文字明确规定:
    29	    "人类 sudo 权限**仅且只**作用于 constitution.md 本身. Trust Root
    30	     清单中的其他载荷文件 (kernel.rs / oracle / accounting / 预注册等)
    31	     属于 ArchitectAI 的合法升级范围"
    32	
    33	  实际执行需要 3 段守护 (per Art. V.1.1 final paragraph):
    34	    (i)  Veto-AI 提案时校核违宪与否 (proposal-time)
    35	    (ii) Boot SHA-256 manifest 保证运行时一致 (run-time)
    36	    (iii) 人类 sudo 限定 constitution.md 范围 (commit-time, 仅这一类)
    37	
    38	  本次 incident 暴露: 没有 (i)+(iii) hook 保护，仅靠 Claude 自律. 不可
    39	  接受 — Art. V.1.3 amendment 明确 Veto-AI 必须 enforce.
    40	
    41	ruling: |
    42	  **本案宪法修订流程 frozen**:
    43	
    44	  Step 1 — 用户在 chat 显式 type 完整短语:
    45	    "I authorize this constitution amendment"
    46	    (verbatim, 不可省略, 不可改动 — R-018 grep 此短语作 sudo gate)
    47	
    48	  Step 2 — 修订必须包含 V.3 修订日志条目:
    49	    | 日期 | 触发者 | 章节 | 摘要 |
    50	    每修订强制添加, 无 V.3 entry → R-018 BLOCK
    51	
    52	  Step 3 — Commit 必须同时:
    53	    (a) 修订 constitution.md 本身
    54	    (b) 修订 V.3 表 (添加 entry)
    55	    (c) 重算 sha256sum constitution.md → 更新 genesis_payload.toml [trust_root]
    56	    (d) cargo test boot::tests::verify_trust_root_passes_on_intact_repo
    57	    (e) commit message 含 "FC-trace: <FC?-N?> [role: constitutional amendment]"
    58	
    59	  Step 4 — 修订后 dual audit (Codex + Gemini) 显式审"修订是否符合 Art. V":
    60	    重 round 持续到 PASS/PASS. CHALLENGE 可 documented + accepted by user.
    61	    VETO 必须 fix.
    62	
    63	precedent: |
    64	  - **R-018 BLOCK** is hard gate — 不可绕过. 即使 Claude 知道 user
    65	    意图, 没有 verbatim "I authorize this constitution amendment"
    66	    短语 → Edit 工具 reject (exit 2).
    67	
    68	  - **判例适用范围**: 仅 constitution.md. Trust Root 其它文件 (载荷
    69	    源码 / 预注册 / 案例 / 评分管线) 属 ArchitectAI commit 权限范围
    70	    (C-073), 不需要 Step 1 的 sudo 短语.
    71	
    72	  - **后续 incident 复盘 trigger**: 任何"我们修了 constitution 但没
    73	    走 4 step 流程" → 本案 reopen + R-018 严格化.
    74	
    75	  - **Cross-ref**: C-072 (Veto-AI 范围), C-073 (ArchitectAI 权限),
    76	    C-074 (FC-first), C-075 (DO-178C tool qualification).
cases/C-072_veto_ai_scope_narrowing.yaml
     1	id: C-072
     2	title: "Veto-AI 范围窄化 — 输出域 {PASS, VETO}, 不做主观评判"
     3	constitution:
     4	  - "Art. V.1.3 amendment 2026-04-25: JudgeAI → Veto-AI rename"
     5	  - "Art. V.1.3 (post-amendment): 白名单严格排除 — 主观质量 / 性能 / 可读性 等不属 Veto-AI 职责"
     6	  - "FC3-N5 / FC3-E17: Veto-AI veto edge to ArchitectAI"
     7	source_lessons:
     8	  - "C-010 Audit Standard: VETO > CHALLENGE > PASS"
     9	  - "C-035 Generator ≠ Evaluator (现在加: ≠ Quality Reviewer)"
    10	
    11	incident: |
    12	  本 session round-2 dual audit 出现关键不一致:
    13	  - Codex: VETO (caught self-inflicted regression — TRUST_ROOT_TAMPERED 被
    14	           silent absorb)
    15	  - Gemini: PASS (没看出此 defect)
    16	
    17	  Codex VETO 是基于"违宪"判定 — round-1 fix 引入新缺陷使 ground-truth
    18	  feedback 被 silent suppress, 违反 thesis claim 7 (white-box predicates
    19	  settle on ground truth). 这是合法的 Veto-AI 输出.
    20	
    21	  Gemini PASS 也是合法 — 它没观察到该违宪.
    22	
    23	  问题是: 之前 JudgeAI 命名容易让人理解为"全面质量评审", 包括 code style /
    24	  performance / readability. 那样 Veto-AI 会有大量 false-positive
    25	  (CHALLENGE 不该 VETO) 或 false-negative (该 VETO 的真违宪被淹没在主观
    26	  judgment 噪声中).
    27	
    28	facts: |
    29	  Art. V.1.3 amendment 文字 (post-2026-04-25) 明确:
    30	
    31	    Veto-AI 唯一的工作是:
    32	      "拿着宪法, 反复逐条校验 ArchitectAI 生成的新架构代码是否'违宪'."
    33	
    34	    它**不**做 (白名单严格排除):
    35	    - 主观质量评价 (代码可读性 / 工程口味)
    36	    - 性能或效率评判
    37	    - 测试覆盖率主观打分
    38	    - 任何与"违宪与否"**无关**的判定
    39	
    40	    它只做一件事:
    41	      "否决违宪提案."
    42	
    43	    输出域 = `{PASS, VETO}` — 不承担其他主观评价.
    44	
    45	ruling: |
    46	  **本案 Veto-AI 行为约束 frozen**:
    47	
    48	  当前 manual Veto-AI proxy = Codex + Gemini dual audit. 每次 audit 必须
    49	  按以下 rule:
    50	
    51	  Rule 1 — Output domain MUST be 2-class only:
    52	    PASS: 没违宪发现
    53	    VETO: 至少一处违宪 (cite which Article / FC element)
    54	
    55	    **NOT allowed**: CHALLENGE-as-third-class, "code quality concern", "style
    56	    suggestion", "performance recommendation". 这些属 ArchitectAI scope
    57	    (Art. V.1.2), 不属 Veto-AI scope.
    58	
    59	  Rule 2 — Memory feedback_dual_audit_conflict 仍生效:
    60	    Codex VETO + Gemini PASS = 保守裁决 = VETO (本 session round-2 用例).
    61	
    62	  Rule 3 — CHALLENGE 是 INFORMATIONAL 不是 BLOCKING:
    63	    Audit 输出 CHALLENGE 提示有改进空间但**不违宪** → ArchitectAI 可选择:
    64	    (a) 同 commit fix (推荐), (b) 后续 sprint 处理, (c) 显式接受不修.
    65	    本 session round-3 Gemini CHALLENGE (EXIT=0+empty PPUT_RESULT) 选了 (a).
    66	
    67	  Rule 4 — Veto-AI 不是 product reviewer:
    68	    用户产品体验 / API 设计 / 工程 ergonomics → 不在 Veto-AI 职责. 这些
    69	    需要单独 review pipeline 或人类 architect 决策.
    70	
    71	precedent: |
    72	  - **判定边界 test**: 当 audit 报告写 "I think this could be cleaner..."
    73	    或 "Performance might suffer because..." → 不是 VETO 信号. 不算入
    74	    feedback_dual_audit_conflict 的保守裁决计算.
    75	
    76	  - **判定边界 test 2**: 当 audit 报告 trace 到具体 Article (Art. I.1
    77	    bool signal / Art. V.1.1 sudo / FC3-S3 readonly / etc.) → 是合法
    78	    VETO 锚点.
    79	
    80	  - **future Veto-AI runtime** (FC3-N32, 📅 Phase 11+): 实装时必须
cases/C-073_architect_ai_commit_authority.yaml
     1	id: C-073
     2	title: "ArchitectAI commit 权限 — 非宪法变更经 Veto-AI PASS 后 commit, 无需 sudo"
     3	constitution:
     4	  - "Art. V.1.2 amendment 2026-04-25: ArchitectAI 拥有架构升级的 commit 权限"
     5	  - "Art. V.1.1 amendment: 人类 sudo 权限**仅且只**作用于 constitution.md"
     6	  - "FC3-N6 ArchitectAI"
     7	source_lessons:
     8	  - "C-071 (constitution amendment process): 区分 ArchitectAI vs human-architect 权限边界"
     9	  - "C-072 (Veto-AI scope): 输出 PASS 即开门"
    10	
    11	incident: |
    12	  本 session 大量 ArchitectAI commit 实际发生 (本 session 共 19 个 commits
    13	  in main branch by Claude as ArchitectAI):
    14	  - B7 Trust Root + Boot freeze (be42f43)
    15	  - B7 alignment fix (0cc48bc)
    16	  - B7-extra rollback toggle + calibration scripts (973a9fd, b0ae03e, ...)
    17	  - 4 round audit fixes (15b87fb, 1df1f62, d0d474e, 2e7f75a, ...)
    18	  - Constitution amendment (c061450) — 这一个走 sudo
    19	
    20	  之前的 ambiguity (Phase 8 era): 不清楚 Claude (作为 ArchitectAI proxy)
    21	  能否直接 commit 还是必须 wait user. 实际多次出现 "等用户 confirm" 又没
    22	  实质收益的 round-trip.
    23	
    24	  Art. V.1.2 amendment 解决该 ambiguity: ArchitectAI 有 commit 权限,
    25	  仅 constitution.md 例外.
    26	
    27	facts: |
    28	  Art. V.1.2 amendment 文字 (post-2026-04-25):
    29	
    30	    ArchitectAI 不仅"提出", 更具有**升级架构的 commit 权限**.
    31	    任何**不涉及 `constitution.md`** 的修改 — 包括 Trust Root 清单
    32	    (`genesis_payload.toml`) 本身的载荷条目增减与 SHA-256 重算 —
    33	    经 Veto-AI (V.1.3) 校核未发现违宪后, 由 ArchitectAI 直接落盘并
    34	    更新 manifest 对应字段. 此过程**不需要人类 sudo**.
    35	
    36	    理由: 宪法已经是 ground truth, ArchitectAI 的修改若未违宪则属于
    37	    宪法允许的演化空间; 强制 sudo 反而把人类拉回工程细节, 违反
    38	    V.1.1 "人类只立宪法"的 Meta 原则.
    39	
    40	ruling: |
    41	  **ArchitectAI commit 权限 frozen**:
    42	
    43	  Rule 1 — Scope:
    44	    ArchitectAI commit 权限覆盖 ALL files EXCEPT constitution.md.
    45	    包括但不限于:
    46	    - src/{kernel,wal,bus,boot,main}.rs + 子模块
    47	    - experiments/ 全部
    48	    - genesis_payload.toml 本身 (载荷条目增减 + hash 重算)
    49	    - rules/ + .claude/hooks/ (governance instrumentation)
    50	    - cases/*.yaml (案例沉淀)
    51	    - handover/ (alignment / preregistration / audits / etc.)
    52	    - tests/ (含 fc_alignment_conformance.rs)
    53	    - PREREG_*.md amendment (PREREG 不是宪法)
    54	
    55	  Rule 2 — Required workflow:
    56	    a. ArchitectAI 设计 fix
    57	    b. FC-trace 锚点 (per memory feedback_fc_first_problem_handling)
    58	    c. (理想) Veto-AI proposal-time gate — 当前 manual via dual audit
    59	    d. cargo test --workspace + smoke (按 feedback_smoke_before_batch)
    60	    e. commit message 含 "FC-trace: <FC?-N?>"
    61	    f. (大改动) 双外审 (Codex + Gemini) Phase 边界 dual audit
    62	    g. (Trust Root manifest 涉及) sha256sum 重算 + genesis update
    63	
    64	  Rule 3 — Distinction from sudo workflow (C-071):
    65	    constitution.md 改动: 必须 4-step sudo 流程
    66	    其它任何文件: ArchitectAI commit, no sudo
    67	
    68	  Rule 4 — Audit 反复仍未 PASS (3+ rounds):
    69	    可能 fix 不当 — escalate to human, 不要继续 round 4 round 5.
    70	    人类 review 后可以 (a) 撤回 fix, (b) 接受 CHALLENGE, (c)
    71	    sudo amendment 改 constitution 让原 fix 合规.
    72	
    73	precedent: |
    74	  - **本 session 19 commits 验证**: 全程 ArchitectAI commit, 仅 1 个
    75	    (c061450) 走 sudo. 流程 work.
    76	
    77	  - **Trust Root manifest 是 ArchitectAI scope** — 这是关键判定. 加新
    78	    文件 (e.g., A0c 加 5 cases/C-071..C-075) 时:
    79	      sha256sum cases/*.yaml | sort > cases/MANIFEST.sha256
    80	      sha256sum cases/MANIFEST.sha256 → genesis_payload.toml
cases/C-074_fc_first_problem_handling.yaml
     1	id: C-074
     2	title: "FC-first problem handling — 任何问题先 trace 到 FC1/FC2/FC3 再设计 fix"
     3	constitution:
     4	  - "FC1 (basic cycle line 326) + FC2 (init/halt/tick line 441) + FC3 (system topology line 670)"
     5	  - "CLAUDE.md Alignment Standard — '每个 src/ pub 符号必须映射到宪法 flowchart 元素'"
     6	  - "Art. V.1.3 amendment: Veto-AI 只判断违宪与否, FC anchor 是 audit 输入"
     7	source_lessons:
     8	  - "F-2026-04-25-04: B7 alignment retroactive fix (pub symbols 无 backlink)"
     9	  - "F-2026-04-25-08: round-3 audit lesson (fixes can introduce regressions; FC-first prevents symptom-fix)"
    10	
    11	incident: |
    12	  本 session 5 个明确"症状-fix"导致回归的 case:
    13	
    14	  1. B7 ship pub symbols 无 TRACE_MATRIX backlink (commit be42f43)
    15	     — 用户 critique → 0cc48bc 补
    16	     根因: ship 时没 trace 到 FC3-N34, fix 是"加 doc-comment" 而不是
    17	     "确认 FC anchor 后再 ship"
    18	
    19	  2. B7-extra "halt at tx 50" 无 FC 锚 → 用户"原子化对齐"critique
    20	     → 重 design 为 synthetic ∏p=0 + FC2-N22 MaxTxExhausted
    21	     根因: 设计阶段没 trace 到 FC1-E18 + FC2-N22, fix 是"找最简实现"
    22	     不是"先确定 FC mapping"
    23	
    24	  3. Round-1 Q7.b fix "任何非零 exit 发 synthetic UNSOLVED"
    25	     → round-2 Codex VETO (TRUST_ROOT_TAMPERED 被 silent absorb)
    26	     根因: fix 没 trace 到 thesis claim 7 (ground-truth feedback
    27	     不可被 silent suppress), fix 是"消除 sampling bias 症状"
    28	
    29	  4. Round-2 同类 problem_file_missing silent absorb (round-3 CHALLENGE)
    30	     根因: 同 (3) — 没 systematic 应用 ground-truth-feedback FC anchor
    31	
    32	  5. 576-run batch 用 deepseek-chat (alias) 而非 PREREG-pinned
    33	     deepseek-v4-flash (canonical) — 用户怒
    34	     根因: MODEL_SNAPSHOT 字段没 trace 到 FC1-N7 (δ/AI canonical
    35	     identity) — 把"能跑"当成"对的", 没追问"label 是否 canonical"
    36	
    37	  共同 pattern: 没 FC-trace 的 fix 倾向于"消除症状"而非"修复架构".
    38	
    39	facts: |
    40	  本 session 用户在 mid-session 直接提出 rule:
    41	    "出现问题你先对齐宪法和宪法中三个flowchart"
    42	
    43	  Memory feedback_fc_first_problem_handling.md 已永久化:
    44	    - Step 1: trace 问题到 FC1 / FC2 / FC3 元素
    45	    - Step 2: 判断 (a) 映射到具体 node/edge → fix derive from anchor
    46	              (b) orphan → cite cases/Cxxx 或 PREREG-§n.m 显式 justify
    47	              (c) 映射不到任何 FC → ESCALATE for sudo
    48	    - Step 3: commit message 含 `FC-trace: <FC?-N?> [role]` 或
    49	              `FC-trace: orphan / <ref>`
    50	    - Step 4: Phase 边界 7 red-line check 加第 8 条 (every commit 含 FC-trace)
    51	
    52	  R-016 fc_trace_in_commit hook 已实装 (commit 2e7f75a) — git commit
    53	  无 "FC-trace:" → judge.sh WARN.
    54	
    55	ruling: |
    56	  **本案 FC-first 流程 frozen**:
    57	
    58	  Rule 1 — Triggers (任一即触发):
    59	    - cargo test failure mid-atom
    60	    - dual audit VETO/CHALLENGE finding
    61	    - smoke 异常 / 假设强反
    62	    - cost / time / API drift 异常
    63	    - 未列入 failure-mode list 的新 failure mode
    64	
    65	  Rule 2 — STOP 在 patch 之前. 不允许"快速 fix":
    66	    在 trace 到 FC anchor 之前, 不写 code, 不 commit.
    67	
    68	  Rule 3 — Trace mechanism (优先级):
    69	    1. 问题"事关什么 invariant"? 找 invariant 对应 FC element
    70	    2. 问题"破坏了什么 edge"? 找 edge 对应 FC arrow
    71	    3. 问题"在哪个 subgraph 内"? 找 subgraph 对应 FCx-Sn
    72	
    73	  Rule 4 — Commit message 格式:
    74	    ```
    75	    <title>
    76	    FC-trace: <FC?-N?> [role: ...]  OR  FC-trace: orphan / <Cxxx | PREREG-§n.m>
    77	    <body>
    78	    ```
    79	
    80	  Rule 5 — Phase 边界 7 red-line 加第 8 条:
cases/C-075_do_178c_tool_qualification.yaml
     1	id: C-075
     2	title: "DO-178C tool qualification — 任何产生 frozen 数据的工具自身必须进 Trust Root"
     3	constitution:
     4	  - "PREREG § 1.8 Trust Root composition"
     5	  - "Art. V.1.1 amendment: sudo 仅 constitution; ArchitectAI 管 Trust Root manifest"
     6	  - "FC3-S3 readonly subgraph"
     7	  - "FC1-N7 (δ/AI) + FC1-N11 (∏p ground-truth) — 测量工具的 ground-truth 锚定"
     8	source_lessons:
     9	  - "C-012 (oracle preflight): 工具运行前必须验证可用"
    10	  - "C-070 (preregistration discipline): pre-reg precedes claim"
    11	
    12	incident: |
    13	  本 session B7-extra round-1 dual audit 发现 7 处 silent absorption
    14	  defect (Codex B1+B2+B3 + Gemini Q2.b/Q2.e + Q7.b). 共同 root cause:
    15	
    16	  PREREG § 1.8 frozen 16 文件 Trust Root, 但**漏了 3 类生产工具**:
    17	  1. src/main.rs — verify_trust_root call site (Q2.b VETO)
    18	  2. Cargo.lock — 依赖锁文件 (Q2.e VETO)
    19	  3. Calibration scripts (run_p0_calibration.sh + compute_p0.py) —
    20	     生产 frozen 数据 (p_0) 的工具自身
    21	
    22	  Codex round-3 又抓到 (B1): "evaluator binary 不调 verify_trust_root —
    23	  只 src/main.rs 调, 但 batch 不跑 main.rs". 即生产 binary 的 boot
    24	  路径没 enforce Trust Root, 仅 src/main.rs (开发 binary) 走完整流程.
    25	
    26	  这是经典 DO-178C tool qualification 缺位 — 资格化的工具 (Lean
    27	  compiler ground-truth oracle) 进了 Trust Root, 但调用 / 运行 /
    28	  分析 这些工具的 wrapper (evaluator boot, runner.sh, compute_p0.py)
    29	  没有.
    30	
    31	  16-file → 20-file 扩展在 commit 15b87fb 落地. 然后 d0d474e 把
    32	  problem_file_missing 路径也 hardened.
    33	
    34	facts: |
    35	  DO-178C 视角的 5 项原则 vs 本项目对照:
    36	
    37	  1. **Requirements traceability**:
    38	     ✓ TRACE_MATRIX_v0/v1 (FC ↔ symbol mapping)
    39	     ✓ tests/fc_alignment_conformance.rs (mechanical witness, A0b 此案前)
    40	
    41	  2. **Multiple-level review**:
    42	     ✓ Codex + Gemini dual audit (4 rounds 本 session 验证流程)
    43	
    44	  3. **Tool qualification** ← **本案焦点**:
    45	     ✓ Lean compiler (∏p ground truth) — 一直在
    46	     ✓ B7 拉伸到: cost_aggregator + wall_clock + post_hoc_verifier +
    47	       jsonl_schema + evaluator + llm_http (mid-term audit add)
    48	     ✓ B7-extra round-1 拉伸到: main.rs + Cargo.lock + runner.sh +
    49	       compute_p0.py (此案 ruling)
    50	
    51	  4. **Configuration management**:
    52	     ✓ git + Trust Root SHA-256 manifest
    53	     ⚠️ rules/active/*.yaml 自身**未在 Trust Root** — A0d (TRACE_MATRIX_v2)
    54	        预留扩展点
    55	
    56	  5. **Problem reporting**:
    57	     ✓ handover/architect-insights/ + handover/audits/
    58	
    59	ruling: |
    60	  **本案 tool qualification 范围 frozen**:
    61	
    62	  Rule 1 — Trust Root MUST 包含产生 frozen 数据的工具自身:
    63	    - 工具产生数据被未来 dual-audit 引用 → 工具进 Trust Root
    64	    - 工具产生 PREREG 冻结值 → 工具进 Trust Root
    65	    - 工具产生 calibration / baseline / threshold → 工具进 Trust Root
    66	
    67	  Rule 2 — Trust Root MUST 包含每个 production binary 的 boot
    68	            integrity check call site:
    69	    - evaluator binary 必须 startup 调 verify_trust_root (B7-extra
    70	      round-2 fix)
    71	    - 任何新 binary 同此规则
    72	
    73	  Rule 3 — 工具的 fail-fast 是必须的:
    74	    - set -euo pipefail 强制
    75	    - 子进程 exit code 必须 propagate
    76	    - 不允许 silent absorption (Codex round-2/3 lesson)
    77	
    78	  Rule 4 — Manifest 添加新工具的工作流 (per C-073 ArchitectAI scope):
    79	    a. 写或修改工具
    80	    b. sha256sum tool_path → 添加 genesis_payload.toml [trust_root]

exec
/bin/bash -lc 'rg -n "round-[0-9]|Round [0-9]|A8e[0-9]?|fix|caught|post-A8e|previous|retrospective|fixes shipped" handover/audits/A8_EXIT_PACKET_2026-04-26.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
3:**Arc**: PPUT-CCL (`PREREG_PPUT_CCL_2026-04-26.md` round-4 PASS/PASS + `PREREG_AMENDMENT_p0_defer_2026-04-25.md`).
4:**Date**: 2026-04-26 (current-state — most recent A8e re-issue noted in `A8_AUDIT_HISTORY_2026-04-26.md`).
7:**Decision rule**: PASS → Phase B (kernel instrumentation + PPUT accounting) authorized; CHALLENGE → in-cycle fixes; VETO → Phase A redesign.
11:**Document split (A8e7)**: this packet is the **stable current-state snapshot** of Phase A at exit. The chronological history of audit rounds 1–N + their in-cycle fix bundles lives in the **companion** document `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` (append-only). Past audit transcripts are at `handover/audits/{CODEX,GEMINI}_PHASE_A8_EXIT_AUDIT_2026-04-26{,_R2,_R3,_R4,_R5,_R6,...}.md`. Reviewers needing the full chronology read the history doc; reviewers verifying current-state correctness against Phase B prerequisites read THIS packet.
17:Phase A = pre-flight (days 1–3 of the 30-day arc). Decomposed into 8 atoms (A0–A7) plus the in-cycle fix bundles A8e..A8e6 produced during the multi-round audit. The atom-level deliverables:
19:- **A0** (a–e): harness modernization. Closed by `62c4e14` (A0e exit audit + 7-item fixes).
37:In-cycle audit-fix bundles (chronology in `A8_AUDIT_HISTORY`):
40:5a56ff6  A8e   0af47b7  A8e2  3d38ba5  A8e3  8693789  A8e4  1622017  A8e5  dbcf53a  A8e6
57:Trust Root manifest progression: A0=24 → A1=25 → A3=26 → A5=27 → A6=28 → A7=31 → A8e=33 → A8e2=34.
62:- Closed by dual-audit cycle ending `62c4e14`. CODEX_PHASE_A0_EXIT_AUDIT + GEMINI_PHASE_A0_EXIT_AUDIT both transitioned to PASS-equivalent after the 7 in-cycle fixes; no open P0.
67:- **Statistical framing** (post-A8e F6 + A8e2 G2 wording corrections): `0.10` is the **least-strict admissible value** the PREREG ceiling allows; `j-RR ≤ p_0` makes a SMALLER `p_0` stricter, so this substitution is operationally permitted but NOT a tighter-than-original guarantee. No Type-I inflation since `j-RR` is descriptive (PREREG § 5.4), outside the inferential family. May be less protective than an eventual calibrated `p_0 < 0.10`; acceptable because Gate H is Phase E and § 3 conditions ensure calibration runs first.
92:### A6 (FC tracing) — `89994c7` (+ A8e fix F4)
94:- **9 wired anchor sites** (post-A8e total — 8 in `run_swarm` + 1 in `run_oneshot`):
101:  - FC1-N12 swarm `verify_omega_detailed` path "alone" (added by A8e F4)
102:  - FC1-N12 swarm `verify_omega_detailed` path "tape+payload" (added by A8e F4)
103:  - FC1-N12 swarm `verify_partial` (added by A8e F4)
104:- Per-run correlation: `experiments/minif2f_v4/src/run_id.rs::mint_run_id` (added by A8e F1) — one identifier per run, threaded into both `emit_event` and `make_pput` so FC events join v2 jsonl rows by equality.
107:- Trust Root: `fc_trace.rs` (chain position 28) + `run_id.rs` (chain position 33 via A8e).
109:### A7 (SiliconFlow plumbing) — `90953d6` (+ A8e2 fix G1 + A8e6 fix K2)
110:- `src/drivers/llm_proxy.py` ported from v3 with one load-bearing v4 change: per-provider multi-key round-robin. `detect_provider()` routing (post-A8e F3 + A8e6 K2): `provider:...` explicit prefix wins; else slash-form ⇒ `siliconflow` FIRST (covers `Qwen/...`, `openai/...`, `meta-llama/...`, `deepseek-ai/...` HuggingFace-style ids); else `"deepseek" in m` ⇒ `deepseek` (bare-form only); else `m.startswith("qwen")` ⇒ `dashscope`; else `dashscope` default.
113:- `scripts/test_llm_proxy.py`: 16-test routing + round-robin conformance suite (no live API). Wrapped by `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` (added by A8e2 G1) so the suite runs on every `cargo test --workspace`. The wrapper FAILS CLOSED if `python3` is missing (A8e3 H6); explicit opt-out `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1`.
116:- Trust Root chain positions 29–31: `llm_proxy.py` + `smoke_siliconflow.sh` + `_smoke_siliconflow.py`. A8e adds `scripts/test_llm_proxy.py` (32) + `experiments/minif2f_v4/src/run_id.rs` (33). A8e2 adds `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` (34).
123:- ✅ B1–B7 + B7-extra mode toggle infrastructure complete (pre-Phase A baseline; round-4 PASS/PASS).
138:(All round-1..round-6 questions and their closures are recorded in `A8_AUDIT_HISTORY_2026-04-26.md`. The questions below are the round-N+1 set — a fresh reviewer pass that does NOT need to re-litigate prior closures.)
146:- (Q2.a) `p_0 = 0.10` substitution: is the wording in `PREREG_AMENDMENT_p0_defer_2026-04-25.md` § 2 + § 8 consistent + statistically correct (least-strict admissible, not "conservative" / "strictest")? Verify by full-text grep for "conservative" / "strictest" — the only acceptable matches are explicit retrospective notes.
151:- (Q3.a) Each of A1–A7 + A8e..A8e6 was committed as one atomic commit with `FC-trace: <FC?-N?>` in the message. Verify by re-reading commit messages. Any commit missing FC-trace? Any commit that touched `constitution.md`?
159:- (Q4.d) Are there any open P0 defects from any prior round (1–6)? Cross-reference `A8_AUDIT_HISTORY_2026-04-26.md` to verify each in-cycle fix bundle's closures hold against current source.
162:### Q5 — Packet/history split (A8e7 structural rewrite)
165:- (Q5.c) Does this packet (current-state) contain ANY round-N retrospective text, fix-shipped headers, or "previous round caught X" claims? It should NOT — those belong only in the history doc.

exec
/bin/bash -lc 'for c in 6be6eb4 180a300 7f4bc0c a5c78e4 30f2a14 89994c7 90953d6 5a56ff6 0af47b7 3d38ba5 8693789 1622017 dbcf53a aaedc9d; do echo "$c $(git log -1 --format=%s $c)"; git log -1 --format=%B $c | rg -n '"'FC-trace:' >/dev/null && echo '  FC-trace: yes' || echo '  FC-trace: NO'; git show --name-only --format= "'$c | rg '"'"'^constitution'"\\.md"'$'"' >/dev/null && echo '  constitution.md: TOUCHED' || echo '  constitution.md: not touched'; done" in /home/zephryj/projects/turingosv4
 succeeded in 293ms:
  FC-trace: yes
  constitution.md: not touched
90953d6 A7: SiliconFlow heterogeneous-LLM provider plumbing (proxy + 3-key smoke)
  FC-trace: yes
  constitution.md: not touched
5a56ff6 A8e: round-1 dual-audit fixes (Codex CHALLENGE / Gemini VETO -> 6 fixes)
  FC-trace: yes
  constitution.md: not touched
0af47b7 A8e2: round-2 dual-audit fixes (Codex CHALLENGE / Gemini CHALLENGE -> 3 fixes)
  FC-trace: yes
  constitution.md: not touched
3d38ba5 A8e3: round-3 dual-audit cleanup (Codex CHALLENGE / Gemini CHALLENGE -> 6 fixes)
  FC-trace: yes
  constitution.md: not touched
8693789 A8e4: round-4 documentary cleanup (Codex CHALLENGE / Gemini PASS -> 3 fixes)
  FC-trace: yes
  constitution.md: not touched
1622017 A8e5: round-5 packet self-consistency (Codex CHALLENGE / Gemini PASS -> 5 fixes)
  FC-trace: yes
  constitution.md: not touched
dbcf53a A8e6: round-6 fixes (1 real routing bug + 4 doc; Codex CHALLENGE / Gemini CHALLENGE)
  FC-trace: yes
  constitution.md: not touched
aaedc9d A8e7: structural rewrite — split packet into current-state + history
  FC-trace: yes
  constitution.md: not touched

codex
I’m running the required gates now. I’ll report the observed counts rather than relying on the packet’s stated counts.
exec
/bin/bash -lc 'cargo test --workspace' in /home/zephryj/projects/turingosv4
 succeeded in 38736ms:
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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 20.50s
     Running unittests src/lib.rs (target/debug/deps/minif2f_v4-2121f01f9a6ef751)

running 74 tests
test agent_models::tests::empty_csv_slot_rejected ... ok
test agent_models::tests::csv_entries_trimmed ... ok
test agent_models::tests::empty_env_parses_to_empty_vec ... ok
test agent_models::tests::empty_parsed_broadcasts_global_model ... ok
test agent_models::tests::heterogeneous_with_gate_passes ... ok
test agent_models::tests::length_mismatch_rejected ... ok
test agent_models::tests::single_entry_parses ... ok
test agent_models::tests::heterogeneous_without_gate_rejected ... ok
test agent_models::tests::single_entry_broadcasts ... ok
test agent_models::tests::positional_length_match_passes ... ok
test agent_models::tests::uniform_length_n_does_not_trip_hetero_gate ... ok
test budget_regime::tests::effective_per_agent_scales_linearly_in_n ... ok
test budget_regime::tests::effective_per_agent_overflow_rejected ... ok
test budget_regime::tests::effective_wall_clock_unimplemented ... ok
test budget_regime::tests::effective_token_total_unimplemented ... ok
test budget_regime::tests::effective_total_proposal_invariant_under_n ... ok
test budget_regime::tests::label_strings_are_stable ... ok
test budget_regime::tests::n_agents_zero_does_not_panic ... ok
test budget_regime::tests::parse_max_transactions_empty_defaults_to_200 ... ok
test budget_regime::tests::parse_max_transactions_garbage_rejected ... ok
test budget_regime::tests::parse_max_transactions_zero_rejected ... ok
test budget_regime::tests::parse_regime_empty_defaults_to_total_proposal ... ok
test budget_regime::tests::parse_max_transactions_negative_rejected ... ok
test budget_regime::tests::parse_max_transactions_valid ... ok
test budget_regime::tests::parse_regime_known_values ... ok
test budget_regime::tests::resolve_budget_default_preserves_phase_b_baseline ... ok
test budget_regime::tests::parse_regime_unknown_rejected ... ok
test budget_regime::tests::resolve_budget_unknown_regime_via_env ... ok
test budget_regime::tests::resolve_budget_token_total_startup_fatal ... ok
test budget_regime::tests::resolve_budget_per_agent_via_env ... ok
test cost_aggregator::tests::test_flip_underflow_panics - should panic ... ok
test cost_aggregator::tests::test_empty_accumulator_zero_total ... ok
test cost_aggregator::tests::test_failed_branches_counted_in_total_cost ... ok
test cost_aggregator::tests::test_tool_stdout_chars_div_4_approximation ... ok
test fc_trace::tests::emit_event_with_no_kv_or_agent_does_not_panic ... ok
test fc_trace::tests::emit_event_with_full_payload_does_not_panic ... ok
test fc_trace::tests::emit_is_no_op_when_disabled ... ok
test fc_trace::tests::fc_id_display_matches_as_str ... ok
test fc_trace::tests::fc_id_strings_are_stable ... ok
test fc_trace::tests::json_str_escapes_required_chars ... ok
test jsonl_schema::tests::test_a4_tactic_diversity_helper ... ok
test jsonl_schema::tests::test_a4_decomposed_metrics_round_trip ... ok
test jsonl_schema::tests::test_a5_budget_regime_round_trip ... ok
test jsonl_schema::tests::test_a4_verifier_wait_bounded_by_total_wall_time ... ok
test jsonl_schema::tests::test_jsonl_schema_v2_round_trip ... ok
test jsonl_schema::tests::test_pput_verified_zero_when_progress_zero ... ok
test lean4_oracle::tests::test_correct_theorem_name_accepted ... ok
test lean4_oracle::tests::test_clean_tactic_accepted ... ok
test lean4_oracle::tests::test_decide_tactic_permitted ... ok
test lean4_oracle::tests::test_forbidden_io_process ... ok
test lean4_oracle::tests::test_forbidden_native_decide ... ok
test lean4_oracle::tests::test_identity_theft_rejected ... ok
test lean4_oracle::tests::test_sorry_in_word_not_rejected ... ok
test post_hoc_verifier::tests::test_no_runtime_accept_zeros_both_pput ... ok
test jsonl_schema::tests::test_legacy_jsonl_still_readable ... ok
test lean4_oracle::tests::test_word_boundary_function ... ok
test lean4_oracle::tests::test_sorry_rejected ... ok
test post_hoc_verifier::tests::test_post_hoc_verified_without_runtime_still_zero_progress ... ok
test post_hoc_verifier::tests::test_pput_verified_matches_runtime_when_both_accept ... ok
test post_hoc_verifier::tests::test_pput_verified_zero_when_lean_rejects ... ok
test rollback_sim::tests::does_not_fire_after_threshold ... ok
test rollback_sim::tests::does_not_fire_before_threshold ... ok
test rollback_sim::tests::env_var_name_matches_prereg ... ok
test rollback_sim::tests::fires_at_threshold_when_enabled ... ok
test rollback_sim::tests::never_fires_when_disabled ... ok
test rollback_sim::tests::threshold_constant_matches_prereg ... ok
test run_id::tests::distinguishes_conditions_for_same_problem ... ok
test run_id::tests::shape_matches_legacy_make_pput_format ... ok
test run_id::tests::handles_path_with_no_stem ... ok
test wall_clock::tests::test_wall_clock_first_read_to_final_accept ... ok
test wall_clock::tests::test_wall_clock_unmarked_returns_none ... ok
test wall_clock::tests::test_wall_clock_final_accept_overwrites ... ok
test wall_clock::tests::test_wall_clock_first_read_idempotent ... ok
test wall_clock::tests::test_wall_clock_no_final_accept_uses_now ... ok

test result: ok. 74 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running unittests src/bin/evaluator.rs (target/debug/deps/evaluator-680e8e25b06524bc)

running 9 tests
test swarm_condition_tests::n1_is_distinct_from_oneshot ... ok
test swarm_condition_tests::parses_valid_n_swarm_conditions ... ok
test swarm_condition_tests::rejects_malformed_n_conditions ... ok
test swarm_condition_tests::rejects_hybrid_v1_and_other_named_conditions ... ok
test swarm_condition_tests::rejects_oneshot_condition ... ok
test v2_emit_tests::test_a4_synthetic_short_circuit_does_not_set_hit_max_tx ... ok
test v2_emit_tests::test_emit_dispatches_as_v2 ... ok
test v2_emit_tests::test_a4_emit_max_tx_exhaustion_row ... ok
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
test b4_post_hoc_verifier_progress_zero_on_runtime_reject ... ok
test b1_jsonl_schema_run_record_dispatcher_present ... ok
test b3_wall_clock_first_read_to_final_accept ... ok
test fc1_n12_lean4_oracle_constructible ... ok
test rollback_sim_env_check_function_present ... ok
test rollback_sim_env_var_canonical_name ... ok
test rollback_sim_threshold_constant_matches_prereg ... ok
test rollback_sim_predicate_logic_at_threshold ... ok

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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 15.46s

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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.98s

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

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

     Running unittests src/lib.rs (target/debug/deps/turingosv4-cd2d82dded9eba82)

running 128 tests
test boot::tests::parse_errors_when_section_missing ... ok
test boot::tests::parse_errors_on_unquoted_key ... ok
test boot::tests::parse_strips_inline_comment_and_blanks ... ok
test boot::tests::verify_trust_root_detects_tamper_in_tempdir ... ok
test bus::tests::test_bus_classify_bounded ... ok
test bus::tests::test_bus_basic_append ... ok
test bus::tests::test_bus_forbidden_pattern_veto ... ok
test boot::tests::verify_trust_root_passes_when_hash_matches_in_tempdir ... ok
test bus::tests::test_bus_creates_market_on_append ... ok
test bus::tests::test_bus_graveyard_feedback ... ok
test bus::tests::test_bus_halt_and_settle ... ok
test bus::tests::test_bus_ledger_integrity ... ok
test bus::tests::test_bus_payload_too_long ... ok
test bus::tests::test_bus_too_many_lines ... ok
test bus::tests::test_bus_snapshot ... ok
test bus::tests::test_bus_unknown_agent_vetoed ... ok
test bus::tests::test_bus_serial_ordering ... ok
test drivers::llm_http::tests::test_driver_error_display ... ok
test kernel::tests::test_append_and_retrieve ... ok
test drivers::llm_http::tests::test_client_creation ... ok
test drivers::llm_http::tests::test_generate_request_serialization ... ok
test kernel::tests::test_golden_path_trace ... ok
test kernel::tests::test_market_ticker ... ok
test kernel::tests::test_no_duplicate_market ... ok
test kernel::tests::test_no_market_for_nonexistent_node ... ok
test kernel::tests::test_market_lifecycle ... ok
test kernel::tests::test_reject_dangling_citation ... ok
test kernel::tests::test_reject_duplicate ... ok
test ledger::tests::test_ledger_hash_chain_integrity ... ok
test ledger::tests::test_ledger_append_and_verify ... ok
test ledger::tests::test_ledger_omega_vocabulary ... ok
test ledger::tests::test_ledger_sequence_monotonic ... ok
test ledger::tests::test_ledger_tamper_detection ... ok
test kernel::tests::test_resolve_all_markets ... ok
test ledger::tests::test_tape_append_root_node ... ok
test ledger::tests::test_tape_append_with_valid_citation ... ok
test ledger::tests::test_tape_dag_branching ... ok
test ledger::tests::test_tape_empty ... ok
test ledger::tests::test_tape_reject_dangling_citation ... ok
test ledger::tests::test_tape_time_arrow_ordering ... ok
test ledger::tests::test_tape_trace_ancestors ... ok
test ledger::tests::test_tape_reject_duplicate_id ... ok
test prediction_market::tests::test_buy_no_increases_no_price ... ok
test prediction_market::tests::test_buy_yes_increases_yes_price ... ok
test prediction_market::tests::test_assassin_profit ... ok
test prediction_market::tests::test_constant_product_invariant ... ok
test prediction_market::tests::test_create_market ... ok
test prediction_market::tests::test_ctf_conservation_1_coin_1_yes_1_no ... ok
test prediction_market::tests::test_multiple_traders_price_discovery ... ok
test prediction_market::tests::test_no_double_resolution ... ok
test prediction_market::tests::test_no_trading_after_resolution ... ok
test prediction_market::tests::test_initial_price_is_50_50 ... ok
test prediction_market::tests::test_prices_sum_to_one ... ok
test prediction_market::tests::test_redeem_requires_resolution ... ok
test prediction_market::tests::test_reject_zero_or_negative_amounts ... ok
test prediction_market::tests::test_pioneer_profit ... ok
test sdk::actor::tests::test_boltzmann_returns_none_empty_tape ... ok
test sdk::actor::tests::test_frontier_detection_leaf ... ok
test sdk::actor::tests::test_frontier_detection_parent_with_child ... ok
test sdk::actor::tests::test_boltzmann_never_returns_none_with_nodes ... ok
test sdk::error_abstraction::tests::classifier_version_is_stamped ... ok
test sdk::actor::tests::test_lineage_score_increases_with_depth ... ok
test sdk::error_abstraction::tests::fixture_other_catchall ... ok
test sdk::error_abstraction::tests::fixture_rewrite_no_match ... ok
test sdk::error_abstraction::tests::fixture_simp_no_progress ... ok
test sdk::error_abstraction::tests::fixture_linarith_failed ... ok
test sdk::error_abstraction::tests::fixture_type_mismatch ... ok
test sdk::error_abstraction::tests::fixture_unexpected_token ... ok
test sdk::error_abstraction::tests::fixture_unknown_constant ... ok
test sdk::error_abstraction::tests::labels_are_unique_and_stable ... ok
test sdk::prompt::tests::test_prompt_contains_no_example_values ... ok
test sdk::prompt::tests::test_prompt_includes_balance ... ok
test sdk::prompt::tests::test_prompt_surfaces_search_hits ... ok
test sdk::prompt::tests::test_prompt_surfaces_team_board ... ok
test sdk::prompt::tests::test_prompt_truncates_errors_to_3 ... ok
test sdk::prompt_guard::tests::test_case_insensitive_match - should panic ... ok
test sdk::prompt_guard::tests::test_clean_prompt_passes ... ok
test sdk::prompt_guard::tests::test_empty_prompt_passes ... ok
test sdk::prompt_guard::tests::test_h_vpput_caught - should panic ... ok
test sdk::actor::tests::test_boltzmann_diversity_not_deterministic ... ok
test sdk::prompt_guard::tests::test_pput_assignment_pattern_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_m_verified_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_runtime_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_verified_caught - should panic ... ok
test sdk::prompt_guard::tests::test_wbcg_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_substring_in_larger_text - should panic ... ok
test sdk::protocol::tests::test_malformed_action_tag_rejected_not_fallback ... ok
test sdk::protocol::tests::test_deduct_negative_amount_rejected ... ok
test sdk::protocol::tests::test_no_byte_repair_on_invalid_escape ... ok
test sdk::protocol::tests::test_parse_action_tag_with_think_block ... ok
test sdk::protocol::tests::test_parse_action_tag_valid ... ok
test sdk::protocol::tests::test_parse_bare_json_fallback ... ok
test sdk::protocol::tests::test_parse_invalid_json_returns_error ... ok
test sdk::protocol::tests::test_parse_no_action_returns_error ... ok
test sdk::protocol::tests::test_parse_with_invest_action ... ok
test sdk::protocol::tests::test_strip_multiple_think_blocks ... ok
test sdk::protocol::tests::test_strip_think_blocks ... ok
test sdk::protocol::tests::test_strip_unclosed_think_block ... ok
test sdk::error_abstraction::tests::fixture_unsolved_goals ... ok
test boot::tests::verify_trust_root_passes_on_intact_repo ... ok
test sdk::sandbox::tests::test_sandbox_captures_stderr ... ok
test sdk::snapshot::tests::test_snapshot_balance_query ... ok
test sdk::tools::librarian::tests::test_board_post_append ... ok
test sdk::sandbox::tests::test_sandbox_echo_command ... ok
test sdk::tools::librarian::tests::test_build_compression_prompt ... ok
test sdk::tools::librarian::tests::test_board_write_read_roundtrip ... ok
test sdk::tools::librarian::tests::test_compress_interval ... ok
test sdk::tools::librarian::tests::test_zero_interval_never_compresses ... ok
test sdk::tools::search::tests::test_sanitize_query ... ok
test sdk::sandbox::tests::test_sandbox_nonzero_exit ... ok
test sdk::tools::search::tests::test_search_empty_query ... ok
test sdk::tools::search::tests::test_search_nonexistent_path ... ok
test sdk::tools::wallet::tests::test_append_is_free ... ok
test sdk::tools::wallet::tests::test_deduct_and_credit ... ok
test sdk::tools::wallet::tests::test_genesis_allocation ... ok
test sdk::tools::wallet::tests::test_insufficient_balance_rejected ... ok
test sdk::tools::wallet::tests::test_negative_deduct_rejected ... ok
test sdk::tools::wallet::tests::test_no_double_genesis ... ok
test sdk::tools::wallet::tests::test_portfolio_tracking ... ok
test sdk::tools::wallet::tests::test_query_unknown_key ... ok
test sdk::tools::wallet::tests::test_query_balance ... ok
test wal::tests::test_wal_replay_missing_file_is_empty ... ok
test sdk::tools::wallet::tests::test_zero_deduct_rejected ... ok
test sdk::tools::wallet::tests::test_unknown_agent_vetoed ... ok
test wal::tests::test_wal_skip_malformed_line ... ok
test wal::tests::test_wal_roundtrip_nodes_only ... ok
test wal::tests::test_wal_roundtrip_mixed ... ok
test sdk::sandbox::tests::test_sandbox_timeout_kills_process ... ok

test result: ok. 128 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.07s

     Running unittests src/main.rs (target/debug/deps/turingosv4-ad648582b935fecb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/fc_alignment_conformance.rs (target/debug/deps/fc_alignment_conformance-239d53a59894a473)

running 26 tests
test fc1_n11_predicate_trait_register_api ... ignored, 🔨 Stage 3 unmerged — bus.register_predicate API + Predicate trait live on phase-z-wtool-tools branch only; not on main. Production path uses inline forbidden_patterns check in append_internal as the ∏p surface.
test fc1_n12_lean4_oracle_ground_truth_predicate ... ignored, Cross-crate — Lean4Oracle in minif2f_v4 sub-crate; covered in experiments/minif2f_v4/tests/fc_alignment_conformance.rs (separate file, separate atom)
test fc1_n1_q_state_carrier_constructible_with_default_config ... ok
test fc1_n4_tape_constructible_with_time_arrow ... ok
test fc1_n11_n15_e18_pi_p_zero_preserves_q_t_via_forbidden_pattern ... ok
test fc1_n13_wtool_bus_append_present ... ok
test fc2_n16_init_ai_orchestrator_swarm_oneshot ... ignored, Binary-only — run_swarm/run_oneshot are in evaluator binary, not lib; refactor needed to expose for direct integration testing
test fc1_n8_n9_n10_output_agent_output_parseable ... ok
test fc1_n7_delta_ai_client_constructible ... ok
test fc2_n20_n27_tick_mr_present ... ok
test fc1_n6_input_universe_snapshot_via_bus ... ok
test fc2_n23_haltreason_full_taxonomy_typed ... ignored, 📅 Not yet typed as Rust enum — only OmegaAccepted exists; other 4 variants {MaxTxExhausted, WallClockCap, ComputeCapViolated, ErrorHalt} per CLAUDE.md report standard live as jsonl strings in extra map. Type promotion is Phase C+ work.
test fc3_e15_e16_e17_constitutional_signaling ... ignored, 📅 Phase 11+ — automated runtime veto/abide signaling not implemented. Today: manual policy via CLAUDE.md Audit Standard
test fc2_n23_event_type_omega_accepted_canonical ... ok
test fc3_n32_veto_ai_runtime ... ignored, 📅 Phase 11+ — Veto-AI runtime not implemented (manual Codex/Gemini dual-audit covers role today; Art. V.1.3 amendment 2026-04-25 narrowed scope to {PASS, VETO})
test fc3_n33_architect_ai_runtime ... ignored, 📅 Phase 11+ — ArchitectAI runtime not implemented (manual Claude code editing covers role today; Phase D will deliver. Art. V.1.2 amendment grants commit authority post-Veto-AI PASS)
test fc3_e14_boot_panic_immediate_abort_documented ... ok
test fc2_n22_halt_via_halt_and_settle ... ok
test fc3_n34_parse_trust_root_section_helper ... ok
test fc3_n31_logs_archive_wal_open_in_tempdir ... ok
test fc3_n40_logs_to_architect_feedback ... ignored, 📅 Phase 11+ — automated logs → ArchitectAI feedback loop not implemented. Phase D consumer reads jsonl + WAL + stderr (per THESIS_V2_GROUND_TRUTH_AUDIT findings C+D)
test fc3_n41_in_process_reinit_loop ... ignored, 📅 Phase 11+ — in-process re-init not implemented (external batch runner retry covers today). FC3-E14 immediate-abort leaf is what we have.
test fc3_n39_log_ledger_present_and_appendable ... ok
test fc3_n34_trust_root_error_taxonomy_present ... ok
test fc3_s3_readonly_subgraph_manifest_size ... ok
test fc3_n34_readonly_guard_verify_trust_root_intact_repo ... ok

test result: ok. 17 passed; 0 failed; 9 ignored; 0 measured; 0 filtered out; finished in 0.05s

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


exec
/bin/bash -lc 'python3 scripts/test_llm_proxy.py' in /home/zephryj/projects/turingosv4
 succeeded in 982ms:
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

exec
/bin/bash -lc 'bash scripts/smoke_siliconflow.sh' in /home/zephryj/projects/turingosv4
 succeeded in 9599ms:
[A7-smoke] SiliconFlow probe — model=Qwen/Qwen2.5-7B-Instruct max_tokens=8
  [primary  ] SILICONFLOW_API_KEY: OK   5978ms; tokens prompt=36 completion=1; content[:32]='ack'
  [secondary] SILICONFLOW_API_KEY_SECONDARY: OK   1239ms; tokens prompt=36 completion=1; content[:32]='ack'
  [tertiary ] SILICONFLOW_API_KEY_TERTIARY: OK   1140ms; tokens prompt=36 completion=1; content[:32]='ack'
[A7-smoke] result: PASS (all configured keys responded)

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
/bin/bash -lc 'rg -n "pub struct RunAggregate|hit_max_tx|tactic_diversity|verifier_wait_ms|budget_regime|budget_max_transactions|total_run_token_count" experiments/minif2f_v4/src/jsonl_schema.rs experiments/minif2f_v4/src/bin/evaluator.rs | head -80' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/src/bin/evaluator.rs:70:    total_run_token_count: u64,
experiments/minif2f_v4/src/bin/evaluator.rs:93:    hit_max_tx: bool,
experiments/minif2f_v4/src/bin/evaluator.rs:97:    tactic_diversity: f64,
experiments/minif2f_v4/src/bin/evaluator.rs:102:    verifier_wait_ms: u64,
experiments/minif2f_v4/src/bin/evaluator.rs:109:    budget_regime: String,
experiments/minif2f_v4/src/bin/evaluator.rs:114:    budget_max_transactions: u32,
experiments/minif2f_v4/src/bin/evaluator.rs:179:    /// `total_run_token_count` (C_i) is **understated** vs a true 150-tx
experiments/minif2f_v4/src/bin/evaluator.rs:187:    // total_run_token_count / failed_branch_count / total_wall_time_ms /
experiments/minif2f_v4/src/bin/evaluator.rs:274:            // total_run_token_count from r1 were silently discarded). PPUT-CCL
experiments/minif2f_v4/src/bin/evaluator.rs:325:    let mut verifier_wait_ms: u64 = 0;
experiments/minif2f_v4/src/bin/evaluator.rs:331:    let oneshot_regime = minif2f_v4::budget_regime::BudgetRegime::TotalProposal;
experiments/minif2f_v4/src/bin/evaluator.rs:384:                // A4: no Lean call reached → verifier_wait_ms=0;
experiments/minif2f_v4/src/bin/evaluator.rs:389:                                 Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:392:                                 false, 1, 1, verifier_wait_ms,
experiments/minif2f_v4/src/bin/evaluator.rs:401:            verifier_wait_ms += v_elapsed;
experiments/minif2f_v4/src/bin/evaluator.rs:445:                              Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:448:                              false, 1, 1, verifier_wait_ms,
experiments/minif2f_v4/src/bin/evaluator.rs:456:                              Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:459:                              false, 1, 1, verifier_wait_ms,
experiments/minif2f_v4/src/bin/evaluator.rs:468:                              Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:471:                              false, 1, 1, verifier_wait_ms,
experiments/minif2f_v4/src/bin/evaluator.rs:641:    let (budget_regime, budget_max_tx_base, max_transactions) =
experiments/minif2f_v4/src/bin/evaluator.rs:642:        match minif2f_v4::budget_regime::resolve_budget(n_agents) {
experiments/minif2f_v4/src/bin/evaluator.rs:650:          budget_regime.label(), budget_max_tx_base, max_transactions, n_agents);
experiments/minif2f_v4/src/bin/evaluator.rs:681:    let mut verifier_wait_ms: u64 = 0;
experiments/minif2f_v4/src/bin/evaluator.rs:730:            // exits ~150 tx EARLY at the rollback threshold). hit_max_tx
experiments/minif2f_v4/src/bin/evaluator.rs:737:                                       Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:743:                                       verifier_wait_ms,
experiments/minif2f_v4/src/bin/evaluator.rs:744:                                       budget_regime, budget_max_tx_base, &run_id);
experiments/minif2f_v4/src/bin/evaluator.rs:931:                                // A4: record proposal for tactic_diversity.
experiments/minif2f_v4/src/bin/evaluator.rs:993:                                // for tactic_diversity (covers append/complete/step).
experiments/minif2f_v4/src/bin/evaluator.rs:1007:                                verifier_wait_ms += v_alone_elapsed;
experiments/minif2f_v4/src/bin/evaluator.rs:1034:                                        verifier_wait_ms += v_combined_elapsed;
experiments/minif2f_v4/src/bin/evaluator.rs:1148:                                                        Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:1154:                                                        verifier_wait_ms,
experiments/minif2f_v4/src/bin/evaluator.rs:1155:                                                        budget_regime, budget_max_tx_base, &run_id);
experiments/minif2f_v4/src/bin/evaluator.rs:1285:                                // A4: record proposal for tactic_diversity.
experiments/minif2f_v4/src/bin/evaluator.rs:1309:                                verifier_wait_ms += v_partial_elapsed;
experiments/minif2f_v4/src/bin/evaluator.rs:1377:                                                        Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:1383:                                                        verifier_wait_ms,
experiments/minif2f_v4/src/bin/evaluator.rs:1384:                                                        budget_regime, budget_max_tx_base, &run_id);
experiments/minif2f_v4/src/bin/evaluator.rs:1452:    // A4: this is the canonical hit_max_tx=true site (ran the full
experiments/minif2f_v4/src/bin/evaluator.rs:1459:    // budget_regime stamp on the v2 jsonl row.
experiments/minif2f_v4/src/bin/evaluator.rs:1465:            ("budget_regime", minif2f_v4::fc_trace::json_str(budget_regime.label())),
experiments/minif2f_v4/src/bin/evaluator.rs:1466:            ("budget_max_transactions", budget_max_tx_base.to_string()),
experiments/minif2f_v4/src/bin/evaluator.rs:1474:              Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:1480:              verifier_wait_ms,
experiments/minif2f_v4/src/bin/evaluator.rs:1481:              budget_regime, budget_max_tx_base, &run_id)
experiments/minif2f_v4/src/bin/evaluator.rs:1493:    total_run_token_count: Option<u64>,
experiments/minif2f_v4/src/bin/evaluator.rs:1498:    hit_max_tx: bool,
experiments/minif2f_v4/src/bin/evaluator.rs:1501:    verifier_wait_ms: u64,
experiments/minif2f_v4/src/bin/evaluator.rs:1505:    budget_regime: minif2f_v4::budget_regime::BudgetRegime,
experiments/minif2f_v4/src/bin/evaluator.rs:1506:    budget_max_transactions: u32,
experiments/minif2f_v4/src/bin/evaluator.rs:1539:    let c_i = total_run_token_count.unwrap_or(0);
experiments/minif2f_v4/src/bin/evaluator.rs:1582:        total_run_token_count: c_i,
experiments/minif2f_v4/src/bin/evaluator.rs:1591:        hit_max_tx,
experiments/minif2f_v4/src/bin/evaluator.rs:1592:        tactic_diversity: minif2f_v4::jsonl_schema::compute_tactic_diversity(
experiments/minif2f_v4/src/bin/evaluator.rs:1595:        verifier_wait_ms,
experiments/minif2f_v4/src/bin/evaluator.rs:1596:        budget_regime: budget_regime.label().to_string(),
experiments/minif2f_v4/src/bin/evaluator.rs:1597:        budget_max_transactions,
experiments/minif2f_v4/src/bin/evaluator.rs:1782:            minif2f_v4::budget_regime::BudgetRegime::TotalProposal, 1,
experiments/minif2f_v4/src/bin/evaluator.rs:1804:                assert_eq!(agg.total_run_token_count, 2000);
experiments/minif2f_v4/src/bin/evaluator.rs:1810:                assert_eq!(agg.hit_max_tx, false);
experiments/minif2f_v4/src/bin/evaluator.rs:1811:                assert_eq!(agg.tactic_diversity, 1.0);
experiments/minif2f_v4/src/bin/evaluator.rs:1812:                assert_eq!(agg.verifier_wait_ms, 4_500);
experiments/minif2f_v4/src/bin/evaluator.rs:1813:                assert!(agg.verifier_wait_ms <= agg.total_wall_time_ms,
experiments/minif2f_v4/src/bin/evaluator.rs:1814:                    "A4 invariant: verifier_wait_ms must not exceed total_wall_time_ms");
experiments/minif2f_v4/src/bin/evaluator.rs:1849:            minif2f_v4::budget_regime::BudgetRegime::TotalProposal, 1,
experiments/minif2f_v4/src/bin/evaluator.rs:1868:    /// `hit_max_tx=true` AND splits `solve_rate` from `tokens_per_solve`
experiments/minif2f_v4/src/bin/evaluator.rs:1889:            minif2f_v4::budget_regime::BudgetRegime::TotalProposal, 200,
experiments/minif2f_v4/src/bin/evaluator.rs:1901:                assert_eq!(agg.hit_max_tx, true);
experiments/minif2f_v4/src/bin/evaluator.rs:1905:                // tactic_diversity = 50/200 = 0.25 (notable correlation,
experiments/minif2f_v4/src/bin/evaluator.rs:1908:                assert!((agg.tactic_diversity - 0.25).abs() < 1e-9);
experiments/minif2f_v4/src/bin/evaluator.rs:1909:                // verifier_wait_ms ≤ total_wall_time_ms invariant.
experiments/minif2f_v4/src/bin/evaluator.rs:1910:                assert!(agg.verifier_wait_ms <= agg.total_wall_time_ms);
experiments/minif2f_v4/src/bin/evaluator.rs:1911:                assert_eq!(agg.verifier_wait_ms, 90_000);
experiments/minif2f_v4/src/bin/evaluator.rs:1924:    /// MUST NOT set hit_max_tx=true. The two exit paths look identical
experiments/minif2f_v4/src/bin/evaluator.rs:1928:    /// 200 tx and is tagged via `hit_max_tx`. Conflating them
experiments/minif2f_v4/src/bin/evaluator.rs:1931:    fn test_a4_synthetic_short_circuit_does_not_set_hit_max_tx() {
experiments/minif2f_v4/src/bin/evaluator.rs:1937:        // line ~622): hit_max_tx=false, then caller sets

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/jsonl_schema.rs | sed -n '40,135p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/jsonl_schema.rs | sed -n '132,170p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

codex
**Findings**
1. **A8e7 Trust Root state is stale in the current packet.**  
   The packet still reports 34 Trust Root entries and progression ending at A8e2: [A8_EXIT_PACKET_2026-04-26.md:51](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:51), [A8_EXIT_PACKET_2026-04-26.md:57](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:57), [A8_EXIT_PACKET_2026-04-26.md:126](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:126), [A8_EXIT_PACKET_2026-04-26.md:153](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:153).  
   Actual current Trust Root is 35 entries after A8e7, with `A8_AUDIT_HISTORY` added: [genesis_payload.toml:68](/home/zephryj/projects/turingosv4/genesis_payload.toml:68), [genesis_payload.toml:72](/home/zephryj/projects/turingosv4/genesis_payload.toml:72), [genesis_payload.toml:128](/home/zephryj/projects/turingosv4/genesis_payload.toml:128), and the immutability test requires it: [trust_root_immutability.rs:123](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/tests/trust_root_immutability.rs:123).  
   This is directly in the Q3/Q4 audit surface, so it must be corrected before PASS.

2. **A8e7 is still represented as partly pending / placeholder history.**  
   The history doc says A8e7 fix commit is `<this commit's SHA>` and “dual audit pending after this commit lands”: [A8_AUDIT_HISTORY_2026-04-26.md:175](/home/zephryj/projects/turingosv4/handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:175), [A8_AUDIT_HISTORY_2026-04-26.md:177](/home/zephryj/projects/turingosv4/handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:177), [A8_AUDIT_HISTORY_2026-04-26.md:179](/home/zephryj/projects/turingosv4/handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:179).  
   But A8e7 is already in the Trust Root progression as landed: [genesis_payload.toml:72](/home/zephryj/projects/turingosv4/genesis_payload.toml:72). This is the same placeholder/staleness class prior rounds were trying to eliminate.

3. **The current-state packet still contains fix-history text that Q5.c says should not be there.**  
   Q5.c says the packet should contain no round-N retrospective/fix-shipped/previous-round claims: [A8_EXIT_PACKET_2026-04-26.md:162](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:162), [A8_EXIT_PACKET_2026-04-26.md:165](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:165).  
   But the packet still includes “post-A8e F6 + A8e2 G2”, “added by A8e F4”, “A8e6 fix K2”, and similar lineage text: [A8_EXIT_PACKET_2026-04-26.md:67](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:67), [A8_EXIT_PACKET_2026-04-26.md:92](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:92), [A8_EXIT_PACKET_2026-04-26.md:101](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:101), [A8_EXIT_PACKET_2026-04-26.md:109](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:109), [A8_EXIT_PACKET_2026-04-26.md:113](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:113). The split is directionally right, but not cleanly executed.

4. **PREREG amendment wording still contradicts itself on whether calibration must precede Phase E.**  
   §2 says the substitution is acceptable because §3 conditions ensure calibration runs before Phase E: [PREREG_AMENDMENT_p0_defer_2026-04-25.md:31](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:31), [PREREG_AMENDMENT_p0_defer_2026-04-25.md:32](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:32).  
   §3 then makes recalibration depend on all five conditions, including hetero completion and Phase D runtime: [PREREG_AMENDMENT_p0_defer_2026-04-25.md:40](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:40), [PREREG_AMENDMENT_p0_defer_2026-04-25.md:48](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:48), [PREREG_AMENDMENT_p0_defer_2026-04-25.md:50](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:50), [PREREG_AMENDMENT_p0_defer_2026-04-25.md:52](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:52).  
   §8 says if those conditions never occur, Phase E proceeds with `p_0=0.10`: [PREREG_AMENDMENT_p0_defer_2026-04-25.md:95](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:95), [PREREG_AMENDMENT_p0_defer_2026-04-25.md:96](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:96). Pick one: calibration-before-Phase-E, or ceiling-substitution-at-Phase-E. Current text claims both.

**Verified**
- `cargo test --workspace`: 265 passed / 29 ignored / 0 failed.
- `python3 scripts/test_llm_proxy.py`: 16/16 passed.
- `bash scripts/smoke_siliconflow.sh`: PASS, all 3 configured SiliconFlow keys responded.
- Prior real code fixes appear closed: unified `run_id`, slash-form SiliconFlow routing, and fail-closed Python conformance wrapper are present.

**VERDICT**: CHALLENGE  
**Conviction**: high  
**Recommendation**: FIX-THEN-PROCEED
2026-04-26T06:51:45.576755Z ERROR codex_core::session: failed to record rollout items: thread 019dc88a-ce14-7570-9da8-fabfdca9f056 not found
tokens used
192,115
**Findings**
1. **A8e7 Trust Root state is stale in the current packet.**  
   The packet still reports 34 Trust Root entries and progression ending at A8e2: [A8_EXIT_PACKET_2026-04-26.md:51](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:51), [A8_EXIT_PACKET_2026-04-26.md:57](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:57), [A8_EXIT_PACKET_2026-04-26.md:126](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:126), [A8_EXIT_PACKET_2026-04-26.md:153](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:153).  
   Actual current Trust Root is 35 entries after A8e7, with `A8_AUDIT_HISTORY` added: [genesis_payload.toml:68](/home/zephryj/projects/turingosv4/genesis_payload.toml:68), [genesis_payload.toml:72](/home/zephryj/projects/turingosv4/genesis_payload.toml:72), [genesis_payload.toml:128](/home/zephryj/projects/turingosv4/genesis_payload.toml:128), and the immutability test requires it: [trust_root_immutability.rs:123](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/tests/trust_root_immutability.rs:123).  
   This is directly in the Q3/Q4 audit surface, so it must be corrected before PASS.

2. **A8e7 is still represented as partly pending / placeholder history.**  
   The history doc says A8e7 fix commit is `<this commit's SHA>` and “dual audit pending after this commit lands”: [A8_AUDIT_HISTORY_2026-04-26.md:175](/home/zephryj/projects/turingosv4/handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:175), [A8_AUDIT_HISTORY_2026-04-26.md:177](/home/zephryj/projects/turingosv4/handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:177), [A8_AUDIT_HISTORY_2026-04-26.md:179](/home/zephryj/projects/turingosv4/handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:179).  
   But A8e7 is already in the Trust Root progression as landed: [genesis_payload.toml:72](/home/zephryj/projects/turingosv4/genesis_payload.toml:72). This is the same placeholder/staleness class prior rounds were trying to eliminate.

3. **The current-state packet still contains fix-history text that Q5.c says should not be there.**  
   Q5.c says the packet should contain no round-N retrospective/fix-shipped/previous-round claims: [A8_EXIT_PACKET_2026-04-26.md:162](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:162), [A8_EXIT_PACKET_2026-04-26.md:165](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:165).  
   But the packet still includes “post-A8e F6 + A8e2 G2”, “added by A8e F4”, “A8e6 fix K2”, and similar lineage text: [A8_EXIT_PACKET_2026-04-26.md:67](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:67), [A8_EXIT_PACKET_2026-04-26.md:92](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:92), [A8_EXIT_PACKET_2026-04-26.md:101](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:101), [A8_EXIT_PACKET_2026-04-26.md:109](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:109), [A8_EXIT_PACKET_2026-04-26.md:113](/home/zephryj/projects/turingosv4/handover/audits/A8_EXIT_PACKET_2026-04-26.md:113). The split is directionally right, but not cleanly executed.

4. **PREREG amendment wording still contradicts itself on whether calibration must precede Phase E.**  
   §2 says the substitution is acceptable because §3 conditions ensure calibration runs before Phase E: [PREREG_AMENDMENT_p0_defer_2026-04-25.md:31](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:31), [PREREG_AMENDMENT_p0_defer_2026-04-25.md:32](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:32).  
   §3 then makes recalibration depend on all five conditions, including hetero completion and Phase D runtime: [PREREG_AMENDMENT_p0_defer_2026-04-25.md:40](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:40), [PREREG_AMENDMENT_p0_defer_2026-04-25.md:48](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:48), [PREREG_AMENDMENT_p0_defer_2026-04-25.md:50](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:50), [PREREG_AMENDMENT_p0_defer_2026-04-25.md:52](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:52).  
   §8 says if those conditions never occur, Phase E proceeds with `p_0=0.10`: [PREREG_AMENDMENT_p0_defer_2026-04-25.md:95](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:95), [PREREG_AMENDMENT_p0_defer_2026-04-25.md:96](/home/zephryj/projects/turingosv4/handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md:96). Pick one: calibration-before-Phase-E, or ceiling-substitution-at-Phase-E. Current text claims both.

**Verified**
- `cargo test --workspace`: 265 passed / 29 ignored / 0 failed.
- `python3 scripts/test_llm_proxy.py`: 16/16 passed.
- `bash scripts/smoke_siliconflow.sh`: PASS, all 3 configured SiliconFlow keys responded.
- Prior real code fixes appear closed: unified `run_id`, slash-form SiliconFlow routing, and fail-closed Python conformance wrapper are present.

**VERDICT**: CHALLENGE  
**Conviction**: high  
**Recommendation**: FIX-THEN-PROCEED
