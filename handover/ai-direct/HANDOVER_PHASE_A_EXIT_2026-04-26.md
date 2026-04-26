# TuringOS v4 — Phase A → B Exit Handover

**Session date**: 2026-04-26
**Session scope**: Phase A engineering atoms A0–A7 (carried over from prior session) + A8 dual-audit gate (this session, 13 rounds + harness amplifier)
**Phase A status**: complete; Phase B authorization recommended on cumulative evidence
**Latest commit**: `50b5afc` (A8e15: round-13 closure)
**Repo state**: 267 PASS / 29 ignored / 0 failed (Rust); 16/16 PASS (Python proxy conformance); Trust Root 38 entries; recursive child-manifest enforcement live.

> **New-session entry**: read this doc + `LATEST.md` (which points here) + `A8_EXIT_PACKET_2026-04-26.md` (current state) + `A8_AUDIT_HISTORY_2026-04-26.md` (13-round chronology) + `PHASE_B_IMPLEMENTATION_PLAN.md` for Phase B starting point. These five files are sufficient to resume without context.

---

## § 1. What this session shipped

### Phase A engineering atoms (commits 6be6eb4 .. 90953d6, prior session's mid-stream)
- A0a–e harness modernization (rules + cases + TRACE_MATRIX_v2)
- A1 PREREG amendment p_0 calibration deferral
- A2 swarm_N=1 mode + parse_swarm_condition_n
- A3 AGENT_MODELS env var + Phase B+C single-model gate
- A4 decomposed metrics (hit_max_tx + tactic_diversity + verifier_wait_ms)
- A5 BUDGET_REGIME + MAX_TRANSACTIONS env vars
- A6 fc_trace.rs + 7-variant FcId enum + 9 wired anchor sites
- A7 SiliconFlow heterogeneous-LLM plumbing (proxy + 3-key smoke)

### A8 audit cycle (commits 60292dc .. 50b5afc, this session)
- A8 prep packet + Codex/Gemini runners (`60292dc`)
- 13 dual-audit rounds (R1 .. R13) with cumulative ~$80 / $500 cap = 16% spend
- 14 substantive bugs caught + closed across rounds (see § 3 below)
- A8e..A8e15 fix bundles (15 in-cycle iterations)
- A8e7 structural rewrite: split `A8_EXIT_PACKET` (current-state) from `A8_AUDIT_HISTORY` (append-only chronology) — eliminated documentary-staleness category error
- A8e12 harness amplifier: case **C-076** (commit-claim diff parity) + rule **R-020** (judge.sh hook WARNs on multi-fix bundles without `Verified:` lines)

---

## § 2. Verified state at A8e15

| Metric | Value | How to verify |
|---|---|---|
| `cargo test --workspace` | 267 PASS / 29 ignored / 0 failed | re-run `cargo test --workspace` |
| `python3 scripts/test_llm_proxy.py` | 16/16 PASS | also wrapped in cargo test via `tests/llm_proxy_python_conformance.rs` |
| `bash scripts/smoke_siliconflow.sh` | PASS (3/3 keys) | live API; ~$0.005 per run; fail-closed if any key missing |
| Trust Root manifest entries | 38 | `grep -c '^"' genesis_payload.toml`; `boot::tests::verify_trust_root_passes_on_intact_repo` PASS |
| Recursive child-manifest verification | live | `verify_child_manifest` parses `cases/MANIFEST.sha256` + `rules/MANIFEST.sha256`, hashes each child file, fails on any divergence |
| Cases | 76 (C-001..C-076) | `cases/MANIFEST.sha256` covers them via TR proxy |
| Rules | 15 active (R-001..R-020 with gaps) | `rules/MANIFEST.sha256` covers them via TR proxy |
| FC-trace anchor sites in evaluator.rs | 9 | grep `fc_trace::emit_event(`; 8 in run_swarm + 1 in run_oneshot |

---

## § 3. Substantive bugs caught + closed across 13 rounds

This is the load-bearing yield of the audit cycle — bugs that would have surfaced at Phase B+ runtime if not caught:

| Round | Atom touched | Defect | Fix |
|---|---|---|---|
| R1 | A6 | `run_corr_id` (FC events) vs `make_pput::run_id` (jsonl) ms drift; oneshot used `oneshot_{problem_file}` placeholder | F1: `run_id::mint_run_id` minted once at run_swarm/run_oneshot entry, threaded into both emit_event AND make_pput |
| R1 | A6 | FC1-N12 emitted only in run_oneshot; the 2 swarm verify_omega_detailed + 1 verify_partial calls had no FC events | F4: 3 new emit sites in run_swarm |
| R1 | A7 | `Qwen/Qwen2.5-7B-Instruct` misrouted to DashScope (m.startswith("qwen") won after slash check) | F3: slash-form ⇒ siliconflow first, bare-qwen ⇒ dashscope |
| R1 | A1 | PREREG_AMENDMENT § 2 called p_0=0.10 "strictest possible substitute" — backwards (smaller p_0 is stricter for j-RR ≤ p_0) | F6: corrected wording, statistical implications paragraph added |
| R1 | A8 | Trust Root manifest count off-by-1 (packet said 30, actual 31) | F5: reconciled |
| R3 | A8e2 | `tests/llm_proxy_python_conformance.rs` wrapper soft-skipped on missing python3 — silent gate degradation | H6: fail-closed with explicit opt-out env var |
| R6 | A7 | `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B` (HF-style SF-catalog) routed to api.deepseek.com (which doesn't serve Distill variants → 404) | K2: slash-form is FIRST routing heuristic, not after deepseek-substring |
| R7 | A1 | PREREG_AMENDMENT § 2(b) claimed "§ 3 conditions ensure calibration runs *before* Phase E" — false (§ 3 are pre-requisites, not guarantees) | M4: § 2(b) reworded |
| R8 | A1 | § 8 audit-requirements text had identical false claim parallel to § 2(b) — round-7 M4 fixed § 2 but missed § 8 | N1: § 8 reworded |
| R10 | A7 | `_smoke_siliconflow.py` would PASS on single-key configuration — silently replicating V3L-27 single-key collapse | P1: requires ALL 3 keys; explicit opt-out for credential rotation only |
| R10 | A8e7 | Audit runner scripts NOT in Trust Root despite being load-bearing gate machinery | P2: added to TR manifest + required-paths |
| R11 | A0/A8e | Trust Root proxy via cases/rules MANIFEST.sha256 was convention only; boot.rs only hashed parent, not children | Q1: `verify_child_manifest` recursive verification; src/boot.rs ALSO added to TR (verifier protected by verifier) |
| R11 | A4/A5 | `make_pput` had `Option<u64>` for params that are non-Option in the v2 PputResult struct + every caller passed `Some(...)` | Q2: signature → plain `u64`; struct + signature now match |
| R12 | A6 / Art. III.2 | `search_cache[agent]` was injected into every prompt's `hits_ref` even after agent hit `SEARCH_CAP` and tool was stripped — agent reasoned from stale results forever | R2: single `cap_hit` boolean gates BOTH the tool list AND the cache injection |

Plus 1 false-closure caught at R9 (A8e9 N3 commit message claimed runner-default fix that the diff didn't ship) — counted separately as a delivery-quality finding.

---

## § 4. New harness amplifier (A8e12)

The recurring documentary CHALLENGE pattern across rounds 2–6 + 8–9 was driven by my (Claude's) execution quality on multi-round metadata fixups: every commit message overstated fix coverage relative to the actual diff. After R10 the user directed C — sediment the lesson into binding case + runtime warn rule.

- **Case `cases/C-076_commit_claim_diff_parity.yaml`** — every commit asserting ≥2 distinct fix items requires per-claim `Verified:` proof line. Multi-section parity check for cross-document edits. Audit transcripts append-only per C-075.
- **Rule `rules/active/R-020_commit_claim_diff_parity.yaml`** + inline implementation in `.claude/hooks/judge.sh` — WARNs at pre-commit on multi-fix-bundle messages (≥2 fix tags or bullets like F1/M2/N3) without any `Verified:` line.
- **Acceptance gate enforcement** — judge.sh fires the WARN; the committer is responsible for grep-verifying each claim.

**Empirical limit**: R-020 catches the syntactic pattern (multiple fix tags without Verified) but NOT the semantic pattern ("you bumped a number; did you grep ALL parallel references"). R12 R1 and R13 S2 are exactly this class — they slipped past R-020. Open question for next session: should R-020 be extended (or a sibling R-021 added) to also flag commits that introduce numeric updates, prompting the committer to grep-cascade across the repo?

---

## § 5. What the audit was NOT

Per user's mid-cycle question 2026-04-26:

The 13-round audit was **adversarial dual external review** (Codex + Gemini, skeptical-reviewer mandate, conservative VETO > CHALLENGE > PASS merge). It was **not DO-178C**. The project's case **C-075** invokes DO-178C tool-qualification *as analogy* for "gate machinery must be qualifiable"; that's one slice of DO-178C used as inspiration, not the standard's full objective set.

**Not done**: PSAC/SDP/SVP/SCMP/SQAP planning artifacts; DAL declarations per failure-condition severity; HLR↔LLR↔test traceability matrix in DO-178C format; structural coverage analysis (statement / decision / MC/DC); formal TQL-1..TQL-5 tool qualification; FCA/PCA configuration audits; software conformity review.

**What IS in place**: dual external adversarial review (project-level), Trust Root SHA verification (constitutional), recursive child-manifest enforcement (A8e13 Q1), test pass rate (267 PASS), live smoke on heterogeneous provider (3-key SF), fc_trace events at HALT decomposition (FC2-N22 × 4 paths) + Lean oracle scope (FC1-N12 × 4 paths) + mr tick (FC2-N20).

If Phase B+ work needs DO-178C-grade rigor, this is a separate engineering investment.

---

## § 6. Constitutional alignment notes

Per user directive 2026-04-26: not "did this commit edit constitution.md" but "could this fix violate any FC1/FC2/FC3 invariant or any Article". Honest retrospective for substantive A8e fixes in this session:

- **A8e13 Q1 (boot.rs recursive verify)**: strengthens FC3-N34 readonly-subgraph enforcement; the proxy was documented but unenforced. ✅
- **A8e13 Q1b (boot.rs added to TR)**: required by C-075 + Art. V.1.2 self-qualification — verifier must be qualifiable. ✅
- **A8e13 Q2 (make_pput Option → u64)**: brings function contract into line with v2 struct fields per Art. I.2 Report Standard (PREREG § 5). ✅
- **A8e13b (Karpathy compression)**: explicit Art. I.1 "压缩即智能 / 反奥利奥架构" alignment; -33 LOC net while preserving correctness. ✅
- **A8e14 R2 (search_cache cleared on SEARCH_CAP)**: closes a gap between code and constitutional intent (Art. III.2 + F-2026-04-19-05 / C-027 lineage: cap is supposed to stop agent from wasting budget on stale search context). The pre-R2 behavior partially defeated the cap's intent. ✅
- **A8e14 R1 + A8e15 (documentary cascades)**: zero FC/Article surface. ✅

Zero constitution.md edits across 15 fix bundles.

---

## § 7. Code state — file pointers for next session

### New code modules (this session — within engineering atoms A0–A7):
- `experiments/minif2f_v4/src/agent_models.rs` (A3) — per-agent model assignment + Phase B+C single-model gate
- `experiments/minif2f_v4/src/budget_regime.rs` (A5) — BUDGET_REGIME enum + MAX_TRANSACTIONS resolver
- `experiments/minif2f_v4/src/fc_trace.rs` (A6) — structured JSON event emitter + FcId enum
- `experiments/minif2f_v4/src/run_id.rs` (A8e F1) — single per-run identifier minted once, threaded everywhere
- `src/drivers/llm_proxy.py` (A7) — multi-key round-robin OpenAI-compatible proxy
- `scripts/_smoke_siliconflow.py` + `scripts/smoke_siliconflow.sh` (A7) — 3-key fail-closed smoke
- `scripts/test_llm_proxy.py` (A8e F2) — 16-test routing + round-robin conformance battery
- `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` (A8e2 G1) — Rust wrapper running the Python suite on every cargo test (fail-closed on missing python3)
- `experiments/minif2f_v4/tests/fc_trace_smoke.rs` (A6) — end-to-end FC_TRACE=1 child-process verification

### Modified code (this session — A8e fix bundles):
- `src/boot.rs` (A8e13 Q1) — `verify_child_manifest` recursive enforcement; boot.rs ALSO added to TR
- `experiments/minif2f_v4/src/bin/evaluator.rs` (A8e F1 + F4 + Q2 + R2) — unified run_id; FC1-N12 swarm sites; make_pput plain types; cap_hit gate on search cache
- `.claude/hooks/judge.sh` (A8e12 R-020) — multi-fix-bundle WARN
- `genesis_payload.toml` — TR manifest 24 → 38; recursive-enforcement note
- `cases/MANIFEST.sha256` — regenerated from repo-root with 51 entries (was 50; C-076 added in A8e12)
- `rules/MANIFEST.sha256` — regenerated from repo-root with 15 entries (was 14; R-020 added in A8e12)

### Governance / handover (this session):
- `cases/C-076_commit_claim_diff_parity.yaml` (A8e12)
- `rules/active/R-020_commit_claim_diff_parity.yaml` (A8e12)
- `handover/audits/A8_EXIT_PACKET_2026-04-26.md` (A8 prep + A8e7 split + many round-N revisions)
- `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` (A8e7 split — append-only chronology)
- `handover/audits/run_codex_phase_a8_exit_audit.sh` + `run_gemini_phase_a8_exit_audit.py` (A8 prep, hardened in A8e10 + A8e11)
- 13 round-N audit transcripts: `handover/audits/{CODEX,GEMINI}_PHASE_A8_EXIT_AUDIT_2026-04-26[_R<N>].md`
- `handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md` — § 2 + § 8 wording corrected (A8e F6 + G2 + M4 + N1)
- `handover/alignment/TRACE_MATRIX_v2_2026-04-25.md` — extended with A8e..A8e14 entries; § 6 manifest milestones updated 24 → 38

### Memory updates (this session):
- `reference_siliconflow.md` (A7) — SiliconFlow as Phase D heterogeneous lane + context-loss anti-pattern lesson

---

## § 8. Known limitations entering Phase B

1. **`per_agent` budget regime untested at runtime** — A5 unit tests verify the scaling math (`base × N`) and env-coupled resolver. No live-LLM run with `BUDGET_REGIME=per_agent` smoked. Phase B is the first opportunity.

2. **FC-trace coverage still partial** — 9 wired anchor sites cover HALT decomposition (FC2-N22 × 4) + mr tick (FC2-N20) + Lean oracle scope (FC1-N12 × 4). NOT yet emitting: FC1-N7 prompt-build, FC1-N11 ∏p decision diversity (per-proposal), FC1-E18 preserve-Q_t (per ∏p=0), FC3-N31 WAL append. The FcId enum reserves these variants. Phase B kernel instrumentation should fill them in as new code lands.

3. **SiliconFlow rate-limit at scale** — A7 verified 3 keys at N=1 concurrency. V3L-27 demonstrates collapse at N=30 single-key. Multi-key round-robin should triple the safe N envelope but the actual sweet spot is unmeasured. Phase D heterogeneous-batch design should land a `--max-concurrency` knob (`LLM_PROXY_CONCURRENCY=5` env in proxy currently) tuned per provider.

4. **Heterogeneous swarm = Phase D, not B/C** — per F-2026-04-25-02 + the `agent_models.rs` `PHASE_D_HETERO_GATE_ENV_VAR` invariant, Phase B+C MUST stay single-model so ablation axes are not confounded. A7's plumbing exists for future Phase D work; Phase B uses `deepseek-v4-flash` thinking-off backbone.

5. **`make_pput` arity = 24** — refactor to builder pattern (`PputResultBuilder`) recommended early in Phase B. Non-blocking; runtime correctness verified by 4 in-binary tests + the integration battery.

6. **R-020 doesn't catch numeric-cascade staleness** — see § 4 above. Open question for next session.

---

## § 9. Phase B starting point

Per `handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md`:

Phase B tasks **B1–B7** are listed as "DONE" in `LATEST.md` from prior session — they were the lead-in work that made the PREREG drafting possible (jsonl_schema.rs / cost_aggregator.rs / wall_clock.rs / post_hoc_verifier.rs / Trust Root + boot freeze). What's NOT done from Phase B per the plan:

- **B7-extra calibration**: deferred per `PREREG_AMENDMENT_p0_defer_2026-04-25.md` (5 conditions must complete first, including "N-experiments arc complete" + "Phase D ArchitectAI runtime exists" — operationally pushed to post-Phase D)

The next concrete work is **Phase C** per the 30-day arc roadmap (`AUTO_RESEARCH_NOTEPAD.md` § Active roadmap):

> 3. **Phase C — Ablation smoke tests** (days 11-17)
>    - 5 modes: Full / Panopticon / Amnesia / Soft Law / Homogeneous
>    - hard-10 adaptation × N=20 paired
>    - Verify H1-H4: violations show on PPUT axis

Which means: Phase A→B exit clears Phase B as well (B1-B7 already DONE in prior session). The next session can start on **Phase C** ablation smoke tests, OR re-confirm Phase B Gate B exit before starting Phase C.

**Recommend** the next session:
1. Re-run `cargo test --workspace` + `bash scripts/smoke_siliconflow.sh` to confirm 267 PASS / 3-key smoke passing
2. Decide: jump to Phase C (ablation 5 modes × 10 problems × 2 seeds = 100 jsonl rows per PREREG § 2), OR run Phase B Gate B exit audit first
3. If ablation: read `PREREG_PPUT_CCL_2026-04-26.md` § 2 (ablation conditions) + § 5 (H1-H4 hypotheses) + § 6 (statistical plan) before launching

---

## § 10. Lessons sedimented this session

- **C-076 / R-020**: false-closure prevention discipline. Every commit asserting ≥2 fix items needs per-claim `Verified:` proof line.
- **Memory `reference_siliconflow.md`**: SiliconFlow is the Phase D heterogeneous-LLM lane (rich catalog), NOT a probe-only target. Context-loss anti-pattern: check `.env` + project files BEFORE asking for credentials.
- **A8e7 split pattern**: stable-snapshot artifact + append-only chronology. Conflating the two in one document creates documentary-staleness churn (rounds 2–6 manifestation). Project's `constitution.md` + `Art. V.3 amendment log` and `PREREG` + `PREREG_AMENDMENT` show the same pattern.
- **Recursive Trust Root proxy enforcement (A8e13 Q1)**: the manifest proxy was convention-only; recursive verification is what makes "tampering with cases/C-001 silently bypasses boot" actually a Tampered error.
- **Verifier in Trust Root (A8e13 Q1b)**: meta-finding — the verifier file (boot.rs) wasn't protected by the verifier. Tampering with verify_trust_root would silently bypass the entire FC3 readonly subgraph. Now in TR.
- **Karpathy elegance ≠ delete documentation**: kept `c_i` / `t_i` renames in make_pput because they make the math read like PREREG § 5; only dropped redundant renames + verbose ad-hoc comments.

---

## § 11. Trajectory + cost budget

Audit cost: ~$80 / $500 cap = 16%. Within memory `feedback_dual_audit` Phase A reservation.

Test count progression: 187 (pre-A0) → 204 (A0e PASS) → 234 (A4) → 254 (A5) → 261 (A6) → 264 (A8e) → 265 (A8e2) → 267 (A8e13 +2 boot recursive tests).

Round verdicts: R1 V/CH → R2-3 CH/CH → R4-5 CH/PASS → R6-7 CH/CH → R8-9 CH/PASS → R10 CH/PASS → R11-12 CH/CH → R13 CH/PASS. **6 Gemini PASS verdicts; 0 VETO post-R1; CHALLENGE asymptote at R13 (zero substantive findings).**

---

## § 12. Pointers for next session (5-file reading list)

1. **This doc** (`HANDOVER_PHASE_A_EXIT_2026-04-26.md`) — session summary + Phase A exit state
2. **`handover/ai-direct/LATEST.md`** — entry pointer; updated this session
3. **`handover/audits/A8_EXIT_PACKET_2026-04-26.md`** — current-state Phase A exit packet (post-A8e15)
4. **`handover/audits/A8_AUDIT_HISTORY_2026-04-26.md`** — append-only 13-round audit chronology (cumulative metrics + per-round verdicts/fixes)
5. **`handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md`** + amendment — frozen research contract; § 2 ablation conditions + § 5 H1–H4 hypotheses are Phase C inputs

Plus the **AUTO_RESEARCH_NOTEPAD.md** `§ Active roadmap` to confirm where in the 30-day arc the next session resumes.
