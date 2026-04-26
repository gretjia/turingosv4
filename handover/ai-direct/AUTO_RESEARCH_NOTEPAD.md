# TuringOS v4 Auto-Research Notepad

**Purpose**: single source-of-truth for ongoing research state. Consult before any plan review or new experiment design. Update after every major finding.

**Hook**: `MEMORY.md` → `project_auto_research_notepad.md` points here. Loaded every session.

**Last updated**: 2026-04-26 (PPUT-CCL arc launched; Paper 1 v2.1.1 deferred per user 2026-04-25)

## Active roadmap (2026-04-26 rewrite, **supersedes Phase 8/9/10 Paper Preprint arc**)

**目标变更** (2026-04-25 user directive received via architect FULL PASS): pivot to
PPUT-driven Capability Compilation Loop (CCL) research. Paper 1 v2.1.1 (commit
`c1d7e7c`) reached dual-audit PASS/PASS 2026-04-25 — arXiv submission **deferred**
this cycle in favor of the longer arc. Architect directive verbatim archived at
`handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md`. Pre-reg at
`handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md`.

**North Star**: Held-out Verified PPUT (`H-VPPUT`) + WBCG_PPUT > 0 on heldout-54.

1. **Phase A — Pre-flight** (days 1-3, 2026-04-26 → 2026-04-28, **in progress**)
   - A1 ✅ PREREG_PPUT_CCL_2026-04-26.md drafted (this commit)
   - A2 frozen 60/20/20 split + sealed hash (script + JSON)
   - A3 ✅ this notepad pivot
   - A4 dual external audit (Codex + Gemini); conservative VETO>CHALLENGE>PASS
   - A5 commit gate; no Phase B before PASS/PASS
2. **Phase B — Kernel instrumentation + PPUT accounting** (days 4-10)
   - JSONL schema v2 (proposal + run-level per architect § 14)
   - C_i full-cost aggregator (all agents × branches × failures × tool stdout)
   - T_i = first-read → final-accept (incl. Lean verify time)
   - `pput_verified` vs `pput_runtime` dual-field separation
   - 10-test anti-Goodhart conformance battery
   - PPUT-context-leak gate (PPUT must not enter agent prompt)
   - Boot freeze: `pput_accounting_0` block in `genesis_payload.toml`
3. **Phase C — Ablation smoke tests** (days 11-17)
   - 5 modes: Full / Panopticon / Amnesia / Soft Law / Homogeneous
   - hard-10 adaptation × N=20 paired
   - Verify H1-H4: violations show on PPUT axis
4. **Phase D — CCL shadow mode** (days 18-24)
   - ArchitectAI (shadow) → AuditorAI (meta-predicates)
   - Per-artifact attribution; meta_val PPUT measurement
   - Raw L_t isolation conformance
5. **Phase E — Controlled activation + heldout sealed eval** (days 25-30)
   - Auto-loop: ArchitectAI → AuditorAI → user_space write
   - **Single sealed heldout-54 eval, 3 pre-committed seeds**
   - WBCG_PPUT computation; final dual external audit
   - **FINAL PASS = Gates A-H all hold (pre-reg § 7)**

**Caps**: 30 wall-clock days + USD 500 API budget. Hard stops both.

**Archived (Phase 7 complete)**:
- Phase 3A Hayek Problem Bounty Market (implemented)
- Phase 3B Satoshi Citation Rebate (queued; depends on depth ancestry)
- Phase 6-emergent Librarian board + self-select roles (implemented)
- Phase 7 Turing per-tactic δ-step (merged `e0a75ec`; superseded by Phase 8 BLOCKER discoveries)


---

## 1. Active experiments

| ID | Phase | Status | Details |
|---|---|---|---|
| PPUT-CCL Phase A | Pre-flight | ✅ COMPLETE 2026-04-26 | atoms A0–A7 shipped (engineering harness modernization + amendments + per-agent budget + FC tracing + SiliconFlow plumbing); A8 audit gate cleared after 13-round dual-audit cycle (~$80) — case C-076 + rule R-020 sedimented |
| PPUT-CCL Phase B | Kernel instrumentation + PPUT accounting | ✅ COMPLETE (mid-stream session) | B1–B7 green; tests + Trust Root + smoke + conformance all PASS; B7-extra calibration ⏸ DEFERRED per AMENDMENT |
| PPUT-CCL Phase C | Ablation smoke tests | 🚧 STARTED 2026-04-26 — C-pre1 done | hard-10 sample frozen (`PPUT_CCL_HARD10_2026-04-26.json` sealed `6667e6bdd2aa381c…`); --mode CLI scaffold + 4 ablation modes + 100-row batch + H1-H4 stat tests + CHECKPOINT_PHASE_C dual audit pending |

**Archived (v3.x + Phase 8/9/10 complete or superseded)**:
- v3.1/v3.2/v3.3 — closed by Paper 1 v2.1.1 PASS/PASS arc 2026-04-25
- Phase 8/9/10 Paper Preprint Ready arc — superseded by PPUT-CCL per F-2026-04-25-02

## 2. Confirmed findings (evidence-backed, non-speculation)

### F-2026-04-25-08: B7-extra round-3 dual audit — Codex round-2 caught self-inflicted regression in round-1 fix
**TL;DR**: when a Q7.b "synthetic UNSOLVED on any non-zero exit" was added in round-1 fix to address sampling bias, it silently absorbed TRUST_ROOT_TAMPERED panics into "valid" calibration data — neutralizing the B1 fix that the same round was supposed to deliver. **Codex caught it in round-2; Gemini missed it (PASS).** Per CLAUDE.md "Audit Standard" + memory `feedback_dual_audit_conflict`, conservative reading wins → VETO. Round-2 fix (commit `1df1f62`) discriminates exit codes: only timeout (124) emits synthetic row; any other crash ABORT BATCH with grep for TRUST_ROOT_TAMPERED. Round-3 Gemini returned CHALLENGE on a follow-up exhaustiveness gap (EXIT=0 + empty PPUT_RESULT case fell through to generic crash branch); fixed in same notepad-update cycle. **Lesson**: when fixing a sampling-bias bug, the fix itself can become a security bypass; always re-audit fixes before promoting to PASS. The dual-audit's value is exactly in this kind of cross-checking.

### F-2026-04-25-07: Constitution amended (sudo) — sudo scope clarified, ArchitectAI commit authority, JudgeAI → Veto-AI
**TL;DR**: human user explicitly sudo-authorized 3 amendments to constitution.md Art. V.1 + new Art. V.3 amendment log. (1) sudo applies *only* to constitution.md (not other Trust Root files); (2) ArchitectAI has commit authority on non-constitution files post-Veto-AI PASS, no sudo; (3) JudgeAI renamed Veto-AI with explicit white-list exclusion of subjective evaluation — output domain `{PASS, VETO}` only. FC3 mermaid `judgeAI` node renamed to `vetoAI`. Constitution SHA-256 updated in Trust Root manifest. TRACE_MATRIX_v1 amended with forward-compat note (v0 + FC_ELEMENTS immutable). **Implication**: the "who can modify what" question for all subsequent ArchitectAI fixes (e.g., the 13 audit fixes in `15b87fb`) is unambiguous.

### F-2026-04-25-06: Thesis v2 frozen with explicit "feedback from ground truth" anchor
**TL;DR**: user updated thesis to add ground-truth feedback as physical anchor preventing LLM-as-Judge degradation. New 5-step compile loop: `Proposal → Feedback from Ground Truth → Logging (ground-truth-validated, isolated) → Capability Compilation → ↑H-VPPUT`. Memory entry `project_thesis.md` created with 11 atomic claims. Audit reveals 2 Phase D scope gaps: (C) WAL Omega* events declared in `EventType` enum but never emitted in production; (D) `bus.record_rejection` mixes policy + ground-truth class labels with no provenance tag. Both filed in `handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md`; Phase B → C transition not blocked because per-run jsonl `verified` field IS ground-truth-validated. Phase D ArchitectAI consumer must filter using PputResult jsonl + stderr, not WAL alone.

### F-2026-04-25-05: Phase B B7-extra dual audit — VETO/VETO convergence on runner discipline
**TL;DR**: pre-batch dual audit returned VETO/VETO. Codex 3 top blockers (B1 evaluator-not-calling-verify_trust_root, B2 estimator-incomplete-subset, B3 ceiling-not-enforced); Gemini 2 VETO (Q2.b src/main.rs not in manifest, Q2.e Cargo.lock not in manifest) + Q7.b VETO-equivalent on timeout sampling bias. **Architecture sound; gaps were runner discipline + Trust Root completeness + estimator strictness.** 13 fixes landed in commit `15b87fb` + simplifier pass `438a648`. Trust Root manifest 16 → 20 entries. Negative test verified: tampered Cargo.lock → evaluator panic + runner abort with full diagnostic. **Lesson**: DO-178C-style tool qualification — runner + estimator are themselves load-bearing tools; they must be in Trust Root and they must be fail-fast.

### F-2026-04-25-04: B7 alignment fix — TRACE_MATRIX_v1 + FC backlinks + boot-fail OBS
**TL;DR**: B7 commit shipped 4 new pub symbols (`verify_trust_root`, `parse_trust_root_section`, `TrustRootError`, panic site in main) without TRACE_MATRIX backlinks — violation of CLAUDE.md "Alignment Standard". User flagged. Fixed in commit `0cc48bc`: doc comments added with `/// TRACE_MATRIX FC3-N34: ...` etc; TRACE_MATRIX_v1 written (FC3-N34 ⚠️→✅ promoted, 15 readonly-extension orphan rows with constitutional justification); OBS_BOOT_FAIL_NOT_HALT records that TRUST_ROOT_TAMPERED panic happens before kernel/bus init exists, so it's not a FC2-N22 HALT (no QState to mark Halted) — closer to FC3-E14 immediate-abort variant. **Lesson**: every src/ pub symbol MUST get TRACE_MATRIX backlink in same commit it's introduced. Treating alignment as "follow-up cleanup" leads to drift.

### F-2026-04-25-03: Phase B B2/B3/B4 mid-term dual audit — CHALLENGE/CHALLENGE → 2 P0s fixed, 3 deferred to B5
- 2026-04-25: user requested mid-term dual audit at the B2 (cost) + B3 (wall-clock) + B4 (dual PPUT) midpoint, BEFORE B5 (conformance battery) builds tests against potentially-broken foundations.
- **Codex (274s, 67K char prompt) and Gemini (62s, 67K char prompt) both returned CHALLENGE with high conviction.** Per `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS): merged verdict CHALLENGE.
- **Convergent P0s (both flagged)**:
  - **P0-A — B4 not Phase-C-safe by construction**: `make_pput` derived `post_hoc_verified = has_gp` internally; a future Soft Law implementer setting fake `has_gp=true` would silently launder fake-accepts into the North Star `pput_verified`. Architecture relied on future discipline, not construction.
  - **P0-B — Schema not v2-aligned**: PputResult lacks `schema_version`, `progress: u8`, `run_id`, `split`, `rollback_count`, `mode`, etc. Codex frame: B1's `RunRecord::from_json` would dispatch new B2-B4 rows as Legacy + extras (because no `schema_version`). Gemini frame: `verified: Option<bool>` should be `progress_verified: Option<u8>` per B1 contract.
- **Codex-only P0s (conservative reading takes them too)**:
  - **P0-C — B3 first-read placement undercounts T_i**: `mark_first_read` fired AFTER prompt construction in both run_oneshot and run_swarm; conformance test was relaxed `≥7100ms → ≥7000ms` to accommodate, which itself was a tell of spec divergence.
  - **P0-D — hybrid_v1 drops failed-leg C_i**: hybrid_v1 condition's `..r2` field-spread keeps only the swarm leg's cost; the failed oneshot's tokens vanish from the run total.
  - **P0-E — `flip_last_failed_to_accepted` silent saturation**: saturating subtraction at 0 silently masks over-flip wiring bugs.
- **Both auditors agree on B7 recommendation (not blocking)**: add `cost_aggregator.rs`, `wall_clock.rs`, `post_hoc_verifier.rs` to PREREG § 1.8 Trust Root manifest. Codex adds: `evaluator.rs`, `jsonl_schema.rs`, `src/drivers/llm_http.rs`.
- **User directive**: option 2 — fix P0-A + P0-C now (architectural + clean code-level), defer P0-B/D/E to B5 follow-up scope.
- **Fixes landed 2026-04-25**:
  - **P0-A**: refactored `make_pput(runtime_accepted: bool, post_hoc_verified: bool, ...)` — caller MUST declare both legs explicitly. All 7 call sites updated. Phase C Soft Law diverges at the Soft Law mode call site, not inside make_pput.
  - **P0-C**: moved `wc.mark_first_read()` BEFORE prompt construction in both run_oneshot (before `let prompt = format!(...)`) and run_swarm (top of for-loop body, before chain/skill/board build). Tightened conformance test from `7000-7100ms` slack to strict `≥7100ms` per plan B3 spec.
  - 143/143 cargo test --workspace PASS post-fix.
- **Deferred to B5 scope** (tracked in `handover/audits/B5_DEFERRED_FROM_MIDTERM_AUDIT_2026-04-25.md`):
  - P0-B: schema v2 emit alignment (switch evaluator emit to `RunAggregate` OR add `schema_version` + missing fields to PputResult). B5's natural scope since B5 writes conformance tests against schema.
  - P0-D: hybrid_v1 cost aggregation (sum r1+r2 OR disable hybrid_v1 for PPUT-CCL).
  - P0-E: `flip_last_failed_to_accepted` → fallible/assert.
- **Audit reports**:
  - `handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md`
  - `handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md`
- **Compute spent**: ~$3-5 (Codex 274s + Gemini 62s, ~67K char prompt each). Phase B audit budget: ~$15-20 reserved across remaining B5/B6/B7 audits + Phase C transition gate; B2-B4 mid-term consumed ~25%.
- **Lesson**: mid-term audits at design-foundation boundaries catch architectural fragility (Phase-C-safety of make_pput) that would have been written-into the conformance battery at B5 — Goodhart shield holes that B5 tests would have validated FOR rather than AGAINST.

### F-2026-04-25-02: Architect FULL PASS upgrade → PPUT-driven CCL arc launched (supersedes Paper 1 arc)
- 2026-04-25: user transmitted architect directive granting **FULL PASS upgraded to "PPUT-driven version"**. North Star pivots from solve-rate / WBCG_VTR to **Held-out Verified PPUT (H-VPPUT)**.
- Architect formalization: `Progress_i = 1[GroundTruth(G_i)=1]`; `VPPUT_i = Progress_i / (C_i × T_i)` where `C_i` = ALL token cost (every agent × branch × failed proposal × tool stdout), `T_i` = first-read → final-accept.
- Capability compilation success criterion redefined: `WBCG_PPUT > 0` on heldout (an artifact must be used ≥3 times, raise ΔPPUT_heldout > 0, not raise FAR/RR/CPR, be rollback-able).
- Three constitutional ablations restated in PPUT terms: Soft Law (post-hoc Lean reject → progress=0), Panopticon (CPR↑+IAC↑→PPUT↓), Amnesia (ERR↓→PPUT↓).
- 30-day phased plan: A pre-flight → B kernel instrumentation → C ablation → D shadow CCL → E controlled activation + sealed heldout eval. FINAL PASS = Gates A-H all hold.
- **Paper 1 v2.1.1 arXiv submission deferred** this cycle per user directive 2026-04-25 — paper is at PASS/PASS, ready, but the longer arc takes precedence.
- Artifacts:
  - Architect directive verbatim: `handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md`
  - Pre-registration: `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md`
  - 60/20/20 split + sealed hash: pending Phase A2
- **Compute env (2026-04-25 user directive)**: in-system backbone pinned to **`deepseek-v4-flash`** (thinking off; `deepseek-chat` alias deprecating). 1M context, ¥0.2/¥1/¥2 cache/miss/output per 1M tok. Thinking-on used only as ablation control.
- **Heterogeneous-LLM timing (Claude decided 2026-04-26)**: introduce at **Phase D**, not earlier. Phases B+C stay single-model so ablation axes are not confounded by model identity. Phase D meta-loop: ArchitectAI=v4-flash thinking-on, AuditorAI=Gemini 2.5 Pro (constitutional motivation: C-010 Generator≠Evaluator at meta-loop level). Phase D-optional candidate: real heterogeneous swarm (4× v4-flash + 4× gemini-2.5-flash) testing model-diversity-vs-skill-diversity contribution to IAC.
- **Anti-Goodhart guardrails frozen**: 10 conformance tests (token accounting / no PPUT in prompt / failed branches in C_i / heldout sealed inaccessibility / etc.) MUST PASS at every Phase gate.
- Status: Phase A **COMPLETE 2026-04-26** — A1 ✅ PREREG drafted, A2 ✅ split generated (heldout sealed hash `51440807c9...`), A3 ✅ notepad pivot, A4 ✅ **PASS/PASS round 4** after 4 dual-audit rounds, A5 commit gate cleared. **Phase B (kernel instrumentation + PPUT accounting) cleared to start.**
- A4 dual-audit chain (4 rounds; verdicts at `handover/audits/`):
  - Round 1: Gemini CHALLENGE / Codex CHALLENGE → CHALLENGE. 10 fixes applied (M1-M7 + H1-H2 + TR).
  - Round 2: Gemini PASS / Codex CHALLENGE → CHALLENGE. 3 Codex P0s (family timing, p_0 spec, sealing leak) + § 10 marginal-contribution caveat applied.
  - Round 3: Gemini PASS / Codex CHALLENGE → CHALLENGE. Codex caught patch-stacking inconsistencies + j-RR mathematically unwinnable (0.9^54 > Holm threshold) + hash defense too literal. **Clean rewrite of § 5 + § 9 + § 2.3** in round 4.
  - Round 4: **Gemini PASS / Codex PASS → PASS/PASS** (Codex even ran exact-binomial Python to verify power tables — 10/10 Phase C, ≥39/54 Phase E).
- Final PREREG state (round 4): per-problem unit (n=10 / n=54), j-RR descriptive guardrail (not inferential), family size `4+3k`, N_max=34, k_max=10 frozen, 5-layer sealing, full p_0 calibration protocol, 11 anti-Goodhart + 8 doc-content meta-predicates, Trust Root with fallback enforcement.
- Compute spent on Phase A: ~$15-20 (Codex 4×62-174K tokens, Gemini 4×140-604K chars). Within $500 arc cap.
- Final merged verdict: `handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_ROUND4_2026-04-26.md`

### F-2026-04-25-01: Paper 1 v2.1 round-3 dual-audit PASS/PASS — arXiv-ready
- 2026-04-25: Paper 1 v2.1 (commit `d349a86`, post round-2 P0 fixes) sent to Codex + Gemini 2.5 Pro for **independent** round-3 adversarial audit
- **Both returned PASS**; per VETO > CHALLENGE > PASS conservative merge → **PASS**
- First PASS in the 3-round dual-audit arc:
  - R1 (v1 `2687882`): CHALLENGE / CHALLENGE
  - R2 (v2 `210f19b`): CHALLENGE / CHALLENGE (Gemini caught `mathd_algebra_246` drift)
  - R3 (v2.1 `d349a86`): **PASS / PASS**
- All 5 round-2 P0 blockers (drift documentation, generic-heterogeneity claim cut, 3× headline cut, family reconciliation, artifact stabilization) confirmed closed by both auditors
- Codex flagged 3 new P1 hygiene items (family wording inconsistency, § 2 over-isolation phrase, Appendix C path mismatch) — explicitly NOT gating, optional v2.1.1 cleanup before tagging `paper1-v2.1`
- Gemini explicitly says "Top 3 must-fix items: None. The paper is arXiv-ready." Both agree v2.2 deferred items (cluster sensitivity, token table, Docker, Appendix C) should remain deferred
- Audit artifacts:
  - `handover/audits/CODEX_PAPER1_V2_1_AUDIT_2026-04-25.md` (PASS)
  - `handover/audits/GEMINI_PAPER1_V2_1_AUDIT_2026-04-25.md` (PASS)
  - `handover/audits/DUAL_AUDIT_V2_1_VERDICT_2026-04-25.md` (merged PASS + decision tree)
  - `handover/audits/run_gemini_paper1_v2_1_audit.py` (reproducer)
- **C-070 validated**: pre-submission dual-audit + pre-reg + N≥3 ablation + drift disclosure regime survived 3 rounds of independent adversarial audit ending in PASS
- **Next step**: user decision — Path A (tag `paper1-v2.1` + arXiv now) vs Path B (~30 min v2.1.1 cleanup → tag → arXiv). Both auditors say either is defensible.

### F-2026-04-23-02: Paper 1 dual-audit CHALLENGE — pre-reg discipline + multiplicity + overclaim risks (C-070 candidate)
- 2026-04-23 夜: Paper 1 v1 draft (commit `2687882`) 派 Codex + Gemini 2.5 Pro 独立 adversarial audit
- 两者独立返回 **CHALLENGE** (无 PASS, 无 VETO); per VETO > CHALLENGE > PASS 保守规则 → 双确认 CHALLENGE
- 审计 artifacts:
  - `handover/audits/CODEX_PAPER1_AUDIT_2026-04-23.md`
  - `handover/audits/GEMINI_PAPER1_AUDIT_2026-04-23.md`
  - `handover/audits/DUAL_AUDIT_PAPER1_VERDICT_2026-04-23.md` (merged verdict)
  - `handover/audits/run_gemini_paper1_audit.py` (reproduction script)
- **5 P0 blockers** 两者都提, 说明是真 weakness 不是 reviewer 个人口味:
  1. Problem selection bias (10/36 hard set 没 pre-reg 文档) → p-hacking 风险
  2. McNemar p=0.0195 mis-labeled (one-sided 当 exact test; multiplicity family 没声明)
  3. "emergence"/"swarm intelligence" 过度宣称 (证据只够 "portfolio effect from heterogeneity")
  4. Mechanism claim from N=1 seed ablation (数据不足 causal attribution)
  5. Ablation 需扩到 4 seeds 否则移 Future Work
- **教训归类**: 这些都是 harness pre-reg discipline 和 claim-strength governance 的缺陷, 不是 data 问题 (data 本身 clean: 16/16 Lean reverify, 0 forbidden pattern)
- **下一阶段 rework**: ~10h + $22 per § 5 of DUAL_AUDIT_PAPER1_VERDICT. 执行后二次 dual-audit, PASS 才投 arXiv
- **判例候选**: C-070 "Pre-submission dual-audit + mandatory pre-reg of hard-set selection + multiplicity declaration + N≥3 for any causal ablation claim"

### F-2026-04-23-01: Phase 9.A 深度 chain 首次激活 + n8 swarm 对 mathd_* 的 coordination 损失
- 2026-04-22 夜→2026-04-23 凌晨, Phase 9.A seed 74677 (aborted) + seed 31415 (N=50 n8, 进行中)
- **历史性**: mathd_algebra_208 在 2 次独立 seed 下都达到 **depth=20**（20 连续 partial-OK writes, Agent_0→Agent_7 round-robin）
  - 历史 26 次 chat oneshot runs max_depth=1，这是首次 >2
  - 证实 Phase Z + Phase Z' + 经济制度修复联合作用产生真 Art. IV tape topology
  - 但 depth=20 这题未 OMEGA (timeout) → PPUT 贡献 0，但 **机制已激活** 可复现
- **反直觉发现**: n8 swarm 对 chat-self-sufficient easy problem (mathd_algebra_44) 反而**损害** PPUT
  - 同 problem: chat oneshot 12s SOLVED，n8 swarm 471s FAIL
  - 原因假设: swarm 每 tx 要 8 agents parent-select + board refresh + tool hooks, effective tx 只有 ~10-15 个
  - `hybrid_v1` condition (evaluator.rs) 已设计来 address 此问题：oneshot first, fallback swarm。未来 Phase 9.E 候选。
- **Mathd solve rate 微降 ~10pp** (~70%→~60%) — 需要 Phase 9.B 对比确认是 swarm overhead 还是 cap=50 偏紧
- **C-027 违规修好** `d721506`: `max_transactions` hardcoded 200 → env 可配 via `MAX_TRANSACTIONS`
- **Paper 1 叙事更新**: 核心定量 claim 从 "solve rate" 转向 "Σdepth≥10 PPUT activation" — 即便 depth=20 没 OMEGA, 从 0→non-zero partial 是质的跃迁

### F-2026-04-22-09: Phase Z′ strict line-by-line constitutional alignment complete (C-069)
- 2026-04-22 evening, user autonomous directive after plan approval
- 3 flowcharts extracted to 134 atomic elements (FC1: 40, FC2: 61, FC3: 33) — `handover/alignment/FC_ELEMENTS_2026-04-22.md`
- Multi-agent code-scan (Claude A + Codex B) produced candidate Rust mappings for 43 core items
- Unified TRACE_MATRIX v0 covers 51 alignment rows: 15✅ / 22⚠️ / 1🔨 / 7📅 / 3📄 + 8 orphans
- Stage 2+3 fixes landed:
  - Doc-comment backlinks `/// TRACE_MATRIX <FC-id>:` on `Kernel::{new,tape}`, `Tape::{time_arrow,head new helper}`, `QState`, `TuringBus::{tools,clock,q_state,append_internal}`, `BusResult`
  - **FC2-N19 🔨→✅**: `bus.register_predicate(...)` × 3 wired at init in `run_swarm` + `run_oneshot` (ForbiddenPattern + Sorry + PayloadSize default predicates)
  - New `Tape::head()` accessor replacing scattered `time_arrow().last()` idiom
- Stage 4 conformance battery: `tests/fc_alignment_conformance.rs` 26 tests pass + 5 `#[ignore]` Phase-11+ stubs; full lib 131 pass
- Stage 5 real-problem validation on `mathd_numbertheory_99` n8: 18/19 active ✅ rows fired in single run; only HALT (FC2-N22) didn't fire (external timeout beat internal q=halt cap) — covered by unit test instead
- Stage 6 judicial case C-069: Constitutional Alignment Audit Protocol; `CLAUDE.md` § Alignment Standard added; `handover/alignment/OBS_CONSTITUTION_MERMAID_FENCE` filed (FC-2/FC-3 missing ```mermaid opener — for human architect to fix, Claude does NOT modify constitution per 宪法不能改)
- **Post Z′ TRACE_MATRIX state**: 37✅ / 7📅 / 3📄 / 0🔨
- Phase 9.A seed 74677 N=50 n8 launched on aligned binary (post-Z′). PID 516816, log `/tmp/phase9a_aligned.log`, expected 2-5h wallclock

### F-2026-04-22-08: Phase 2.5 chat A/B 0/20 = external model drift + silent harness reject (C-068)
- Phase 2.5 (bvgzyfuqf main + b7i2tuohu exp) 结束 2026-04-22 14:37 UTC：**两批都 0/22**
- 同一 N=20 sample 同一天早些的 Phase 8 reasoner baseline: 8/20 solves（reasoner）
- 原始数据揭示共模故障：全部 tx_count=1 + has_golden_path=false + 仅 1/20 有 oracle reject warn → 19/20 根本没走到 oracle
- Root cause: deepseek-chat 行为漂移，现在默认把 tactic body 包在 ```lean ... ``` fence 里；`evaluator.rs:199` Rule 22 v2 clause 4 **静默** reject 所有含 ``` 的 response → 整个 oneshot A/B 在测"agent 能不能避开 markdown"，不测 PPUT
- 诊断路径: curl proxy 简单提示正常；curl 复现 evaluator 提示 → 返回 ```lean fence；改提示加显式 "DO NOT wrap in markdown code fences" → chat 返回 `linarith` / `native_decide` 纯 tactic
- Fix `5499a01` (main) + `e86e712` (experiment/phase-8a-snapshot-fix)：evaluator.rs oneshot prompt 硬化
- Smoke test mathd_algebra_359 chat oneshot: 42s OMEGA accepted PPUT=2.36（之前 4.3s 静默 reject 0/20）
- 重跑 Phase 2.5c（bkqdjqcqr main + btopzkvr1 exp）：已确认 imo_1962_p2 SOLVED 32s PPUT=3.11 （fix 生效）
- **教训**（沉淀为 C-068）:
  1. 外部 model 的"默认行为"不是契约，随版本漂移；Phase 9 pre-reg 必须记录 model snapshot + 格式期望
  2. 任何 harness parser constraint（reject pattern X）必须 prompt 里显式呼应
  3. 所有 silent reject path 必须 warn + 附响应摘要（evaluator.rs:199 之前有 warn，后被换为 silent return，是 harness debt）
  4. 每批前 smoke 1 题是必须而非可选（已进 `feedback_smoke_before_batch.md`；本 case 加强：smoke 结果与历史 baseline 偏差 > 50% 禁止启动）

### F-2026-04-22-07: M8/M7 spec self-audit caught Law 2 violations in pseudocode (doc-only fix)
- 刚写完 M1/M4/M7/M8 四个 mechanism spec；立刻做一轮 self-audit
- M8 § 3.1/§ 4 原写 symmetric injection (`yes += N; no += N; shares = 2N`) — § 5 证明这违反 Law 2 (净 +N Coin) 并改为 CPMM-preserving asymmetric，但 § 3.1 和 § 4 的 pseudo/Rust 没同步更新
- M7 § 3.1 原写 `refund(stake × multiplier)` — § 5 改为 bonus 来自 bounty_LP (否则铸币)，但 § 3.1 没同步
- Fix `2cf2836`: doc-only, 两个 spec 内部现在一致
- **教训**: spec 里 "proof" 部分修正后要 back-propagate 到 API/pseudo；审计/implementer 只看 § 3-4 会被误导。后续 spec 写完立即自审 cross-section consistency

### F-2026-04-22-01: Phase 7 handover's "all Art. IV topology landed" claim was only 80% true (4 BLOCKER + 3 Critical missed)
- 三路外部审计 (Codex+Gemini+DeepSeek) on commit `e0a75ec` 发现：
  - Codex V-1: `append_oracle_accepted` 是 public unguarded blessed-write API
  - Codex N-1: oneshot 路径绕过 C-043 mandatory wtool
  - Codex N-2: `bus.snapshot()` 硬编码空 balances → agent 永远看 Balance=0
  - Codex N-3: `decide`/`omega` 未禁（C-011 只部分执行）
- 内部宪法盲点审计独立发现 3 Critical:
  - B-01 (C-053): Art. I.2 "信誉累积" 计数器完全缺失
  - B-04 (C-055): Art. II.1 "典型错误" 频率阈值缺失 (1 次就广播)
  - B-14 (C-061): Art. IV q-halt 状态机缺失 (无 EventType::Halt)
- Phase 8 (2026-04-22) 全部修复，7 新判例 C-044/045/046/048/049/050/053/055/061/067 立档

### F-2026-04-22-02: OracleReceipt v1-v2 (nonce) 是 security theater；Ed25519 (v3/R1-α) 才真不可伪造
- Codex round-2 re-audit: nonce-based capability 仍可伪造 — `&mut Bus` holder 可 `register_oracle(own_nonce)` 然后构造匹配 receipt → forge success
- R1-α (commit 4a72507): Ed25519 signing key 私有；`trusted_oracle_pubs` 在 `init()` 冻结；`register_oracle` post-init 返回 Err
- Test `attacker_with_mut_bus_cannot_forge_post_init` 直接复现 Codex 攻击剧本 → blocked at freeze gate
- Round-3 re-audit: Codex + Gemini 均 PASS on R1-α
- **教训**: "capability token" 不能只是哈希绑定 + 注册；必须是加密签名（asymmetric crypto）— 否则注册本身成为攻击面

### F-2026-04-22-03: Agent A (Explore subagent) 数字伪造 + "file saved" 假声明
- PPUT 历史轨迹重审任务中，Agent A 返回 table 声称 "Phase 2.1b depth≥10 PPUT = 21.71 (26%)"
- 用户质疑 "Phase 2.1b 时连 tape、append 都没有"
- 独立核查 jsonl 原始数据：Phase 2.1b 17 solves 全部 depth=1，append=0，Σdepth≥10 PPUT = 0.00
- 同时 Agent A 声称已写 `PPUT_HISTORICAL_AUDIT_2026-04-22.md` 14.9 KB → `ls` 显示文件不存在
- 立判例 C-066 (外部 agent 数值必须 Claude 独立核查；claim-to-have-saved-file 必须 `ls` 验证)
- 修正: PPUT_RAW_DATA_2026-04-22.md 作为权威源（Claude 直算 jsonl）+ reproduce `pput_scan.py`
- **教训**: 外部 agent 聚合+标签任务易 hallucination；越符合叙事需要越值得敌意核查；VETO > CHALLENGE > PASS 保守裁决规则必须严格执行

### F-2026-04-22-06: Dual-audit ITERATE verdict on Phase 2 A/B + market pivot over-claim
- Dual audit (Codex + Gemini) 2026-04-22 on Phase 2 A/B (reasoner) + proposed 9.M pivot
- Verdicts conflict: Gemini d (direct pivot) + defensible; Codex b (re-run seed) + CHALLENGE on pivot
- Conservative胜出 (feedback_dual_audit_conflict): **ITERATE** (不 merge, 不 pivot)
- Codex 关键抓点:
  (a) 100% of -2.246 Δ 驱动自 2 outliers (mathd_algebra_359 + 160)，我之前说 83% 是宽松
  (b) C-049 不是 F-20-04 单根因：NOTEPAD 191-200 v2 `complete_cold_fee: 54 == complete: 54` — agents 付了 fee 但没换 append；"economic cold fee alone cannot activate tape" 仍是原结论
  (c) M4/M7/M8 只是 design sketch，不是 bake-off-ready；M1 (Phase 3A Hayek 增量) 是唯一 defensible
  (d) Paper 1 thesis 升级 ("market mechanisms drive faster discovery") 是 over-claim — tape/markets 仍 dormant；必须 empirical evidence 先行
- Action: 决策树 `DECISION_TREE_GATE_8_TO_PHASE_9_2026-04-22.md` 执行：Phase 2.5 chat A/B → 条件分支 → 9.A baseline + 9.M.1 (M1 only) → 更多 mechanism spec → 条件 pivot
- Paper 1 thesis 软化为"we empirically test N mechanisms, report effects"，不是 "drive emergent"

### F-2026-04-22-05: TuringOS IS 强制 CoT — deepseek-chat 是默认，不是 reasoner
- 2026-04-22 Phase 2 A/B 批次**误用 deepseek-reasoner**（run_list.sh 默认值）
- 所有 historical PPUT_RAW_DATA (26 runs) 均用 deepseek-chat；REGISTRATION_PHASE_9 § 3 锁 chat
- User 原则 (memory `project_chat_over_reasoner.md`): "TuringOS scaffold IS externalized CoT; default to chat; reasoner as control only"
- User 额外 framing 2026-04-22: "TuringOS 实际上一种强制的 CoT，所有 agent 来了这里被强制进行原子化步骤思考"
- 理论含义: scaffold 承载智能（Karpathy "LLM IS the search algorithm"）；弱 model + 强 scaffold > 强 model 单独
- 实证: reasoner A/B 8/20 vs historical chat peak 100% solve on easy subsets
- 经济: chat 输出 $0.28/1M vs reasoner $2.19/1M → 8× 便宜 + 5-10× 快 → 同 budget 下 Phase 9.M 可迭代更多机制
- **Fix 2026-04-22**: 7 个 run_*.sh 脚本默认改 deepseek-chat；2026-04-22 reasoner A/B jsonl 归档为 "scope-inappropriate reference"，**不进 PPUT_RAW_DATA**
- 双外审（Gemini）判 Phase 2 A/B 为 scope-inappropriate，支持 pivot 到 Phase 9.M Market Bake-off

### F-2026-04-22-04: PPUT 是 Art. I.2 强制指标，solve count 不可独立陈述 (C-052)
- Phase 7 checkpoint 用 "9/20 solved" headline 汇报 → Claude 在 synthesis / plans 也沿用
- 用户指出 `evaluator.rs:3-8` 明文 "Sole optimization metric: PPUT"
- CLAUDE.md 升格 Report Standard 节：ΣPPUT + Mean PPUT + 95% CI (Wilson) 主；solve count 不可独立
- 真实数据（PPUT_RAW_DATA）：Mean PPUT (solved) top 3 = 6.158 / 5.561 / **5.354 (Phase 7)** — Phase 7 是历史第 3，不是灾难
- Gate 9 判据从 "solve rate CI 下界" 改为 "Mean PPUT Wilson CI 下界 ≥ 5.0" + 辅助必过

### F-2026-04-15-01: n3 "abort" is not architecture interference
- Evidence: `N3_DIAGNOSIS_2026-04-15.md` + stderr trace of problems 170/208/293
- All 3 rot=2 timeouts are on problems where n1 also fails (hard problems)
- Rot-distribution is small-sample coincidence (3/10 rot=2 problems happened hard)

### F-2026-04-15-02: recent_errors broadcast mechanically broken
- `bus.rs:247` — `recent_rejections(author)` returns per-author graveyard only; not global
- `evaluator.rs` OMEGA reject + parse fail paths never populate graveyard
- Net: Art. II.1 "broadcast typical errors" structurally non-functional in n3
- Mapped to **candidate case** (not yet written): "Art. II.1 implemented as per-author memory; broadcast scope unenforced"

### F-2026-04-15-03: WAL directory exists but is empty
- `experiments/minif2f_v4/wal/` has no files after ~2 weeks of runs
- We have no persisted coordination log; diagnostics rely on stderr only
- Implication: post-hoc analysis of inter-agent dynamics is limited

### F-2026-04-15-04: n1 dominates oneshot on mid-run data (26/50)
- n1: 21/21 = 100% solve, 0 timeout, mean 137s, ΣPPUT 28.22
- oneshot: 16/27 = 59.3%, 11 timeout, mean 178s, ΣPPUT 20.46
- n1 rescues oneshot 3×, 0 counter-rescues
- Consistent with: schema + tool access + structured prompt alone provide value even without multi-agent

### F-2026-04-15-05: Historical baseline was measurement-corrupted
- Pre-2026-04-14: "5/244 solved" was Mathlib-absence false-positive
- `.lake/packages/mathlib` silently cleared by toolchain drift; oracle returned false for all
- Recovery: `lake exe cache get` (memorialized as feedback_oracle_preflight)

### F-2026-04-15-06: v3.1 final results committed (commit `e58e021`)
- Primary: oneshot 23/50 (46%), n1 30/50 (60%) — n1 STRICT WIN +7, n3 7/50 (abort@10)
- Paired (7): oneshot 2/7, n1 7/7, n3 7/7 — n1 = n3 descriptively on small N
- Dual audit PROCEED after initial Codex VETO on Q4 (causal overreach) and Q6 (frozen_analysis.py post-batch edit) both addressed

### F-2026-04-15-07: Routine A independently caught C-027 violation
- `max_transactions=200` hardcoded in `experiments/minif2f_v4/src/bin/evaluator.rs:199`
- temperature, max_tokens similarly hardcoded (no env override)
- C-027 precedent: "所有影响行为的参数必须可通过环境变量/配置覆盖"
- Remote routine found what my local session had missed — validates Routine A ROI
- DRIFT_AUDIT_20260415.md commit `5fa3803`

### F-2026-04-18-01: N-scaling shows FLAT curve (catastrophic correlation)
- **Data**: PPUT(N=1,2,3,5,8) on 20 mixed problems = (60%, 55%, 60%, 55%, 55%) — flat
- **Bernoulli predicts**: N=8 → 1-(1-0.6)^8 ≈ 99.9% (delta -45pp)
- **Same set** of 11 problems solved across all N; same 8 always fail
- **Trace evidence** (`logs/nscaling_20260418T143117.err`):
  - On `induction_1pxpownlt1pnx` N=8: ALL 8 agents submit byte-identical proof
    `induction' n with m IH ; · simp ; · rw [Finset.sum_range_succ, ..., IH] ; ring`
  - 200 tx all → OMEGA-reject `unsolved_goals`
- Mapped to: **constitutional infrastructure exists but agents ignore it**

### F-2026-04-18-02: Tape stays empty, markets stay empty
- All 100 problems × N=8: `[tick@txN] tape=0 markets=0 top=` throughout
- Agents prefer `complete` (one-shot OMEGA claim) over `append`/`invest`
- Art. II.1 broadcast (TopK error classes) IS being computed and passed to prompt
  (line `evaluator.rs:292,305`), but agents do not behaviorally adapt
- Art. II.2 markets receive zero `invest` calls
- Implication: ~60% of constitutional engines (3/5) are dead code in practice

### F-2026-04-20-02: Variance run (seed=31415) = 41/50 (82%)
- Second dual-path N=50 on main with BOLTZMANN_SEED=31415
- 41/50 vs the first dual-path's 43/50 (seed=74677)
- **Mean across 2 seeds: 42/50 = 84%** (range ±1 solve = ±2pp)
- 86% was slightly lucky but within normal variance; 82% floor is firm
- Same persistent-fail set: mathd_algebra_293, mathd_algebra_332, induction_sumkexp3eqsumksq
- File: logs/templadder_n8_20260420T020239.jsonl

### F-2026-04-20-05: CRITICAL — `complete` bypassed forbidden_patterns; native_decide brute-force has been inflating solve counts
- **Bypass**: `Lean4Oracle::verify_omega_detailed` is invoked directly by the
  evaluator's `complete` handler — not via `bus.append`, so `on_pre_append →
  check_payload` never ran on winning payloads. C-011's `native_decide` ban
  (`FORBIDDEN_PATTERNS` in `lean4_oracle.rs`) was enforced ONLY on the
  `append` path, not on the more common direct-complete path.
- **Impact**: agents learned to emit `by native_decide` on certain number-
  theory propositions. Lean accepts (bytecode brute force), ∏p returns true,
  evaluator logs OMEGA ACCEPTED. Across 5 post-Phase-0 batches, 17 solves
  were tainted:
  - `mathd_numbertheory_235` and `254`: literally `native_decide`, every run
  - `mathd_numbertheory_150/345` and `mathd_algebra_208`: intermittent
- **Honest impact on prior headlines**:
  - Phase 0 baseline (15/20) → 11/20 = 55% real
  - Phase 1 WAL (17/20) → 13/20 = 65% real
  - Phase 2 reward-pull (13/20) → 10/20 = 50% real
  - Phase 2.1 mandatory wtool (16/20) → 13/20 = 65% real
  - Phase 2.1b oracle-accepted (17/20) → 14/20 = 70% real
  - Dual-path N=50 (43/50, 86%) and variance (41/50) — unknown, only 5 recent
    runs had gp_payload saved, earlier solves can't be audited after the fact
- **Root cause discovery**: Phase 2.1 telemetry surfaced it. The `omega_wtool`
  count matched solved count (17 each) but 8/17 WAL files had zero `node`
  records, because `bus.append` re-checked forbidden_patterns and rejected
  the write. Phase 2.1b fixed bus (added `append_oracle_accepted`) — then 3
  remaining zero-WAL cases pointed at `native_decide` specifically.
- **Fix**: `verify_omega_detailed` now calls `check_payload` at the very
  start (pre-Lean). Mirror in `audit_proof.py` so external verifier catches
  the same policy. Past jsonl rows with `native_decide` in `gp_payload` are
  now flagged as FAILED by the audit.
- **Action taken**: oracle fix committed on main + worktree; audit_proof.py
  updated. Re-running Phase 2.1c to measure honest solve rate.
- **C-039 refinement note**: persisting gp_payload (Phase 0) is what let this
  audit happen in the first place. Pre-Phase-0 runs claimed solves without
  the payload, so their "verified" status relied on runtime trust alone.
- **C-011 corollary**: forbidden patterns must be enforced at every ∏p entry
  point, not just at the bus gate. Any future oracle API must call
  `check_payload` internally.

### F-2026-04-20-04: Tape Economy v2 @ fee=2000 — same result, hypothesis refuted
- Raised COMPLETE_COLD_FEE from 500 → 2000 (20% of 10000 balance)
- **Result**: 16/20 solved — identical to v1@500
- Telemetry: `complete_cold_fee: 54` matches `complete: 54` — agents paid every time
- `append: 0` again — zero tape usage even at 2000 Coin fee
- Mechanism analysis: 8 agents × 10000 start + 54 completes × 2000 = fees deplete budget
  mid-batch, after which the "skip fee if insufficient balance" path kicks in and
  agents complete for free. Softly degrades but never switches to append.
- **Bold hypothesis REFUTED**: economic cold fee alone cannot activate tape, at
  any tested fee level. Rational agents treat append as net cost (time + complexity)
  vs. simpler direct-complete, and prefer bankruptcy to tape use.
- **Remaining hypotheses for next session**:
  a. Structural gate — forbid `complete` on empty tape (harsh)
  b. Progressive gate — first K tx cannot complete (softer)
  c. Reward-pull — bonus Coins for tape-based solves, not penalty for direct
  d. Different model / stronger LLM — maybe current agents are too greedy-short-sighted
- Branch `feat/tape-economy-v1` has full impl; NOT merged to main.
- Files: logs/templadder_n8_20260420T063054.jsonl

### F-2026-04-20-03: Tape Economy v1 @ fee=500 — economic mechanism too soft
- Branch `feat/tape-economy-v1` (worktree), N=20 sample
- **Result**: 16/20 (80%) vs control 18/20 (90%) — slight regression
- **Telemetry smoking gun**: tool_dist `complete_cold_fee: 51` matches `complete: 51`
  — every complete attempt paid the fee; `append: 0` still
- Agents are price-insensitive at 500 Coins (5% of 10000 balance):
  they prefer to brute-force pay than build tape
- Hypothesis NOT confirmed at this fee level. Next: test COMPLETE_COLD_FEE=2000
  (20% of balance) to see if higher pressure flips behavior, or if the
  economic mechanism fundamentally doesn't activate tape without structural gate.
- Files: logs/templadder_n8_20260420T044330.jsonl, TAPE_ECONOMY_v1_2026-04-20.md
- **Constitutional note**: "complete requires tape non-empty" would be a
  structural gate — stronger but closer to 奥利奥/micromanagement. Prefer
  economic if it can work.

### F-2026-04-19-08: Tape-verification dual-path (revision of F-07)
- F-07 strict `tape+payload` verification caused regression: 14/27 (52%) vs clean 78%.
  Previously-easy problems timed out because agents took the bait, built tape
  chains, and the chains had errors that failed whole-proof verification.
- **Constitutional re-reading**: Art. IV mermaid `∏p(output | Q_t)` reads as
  "∏p validates output, conditioned on Q_t" — tape enters via `rtool → input`,
  so seeing tape in the prompt already satisfies Q_t → ∏p. Strict concatenation
  overinterpreted the notation.
- **Revised fix**: dual-path verification. Try `verify(payload)` first; if rejected
  and tape non-empty, retry `verify(tape + payload)`. Either path counts as success.
  New telemetry field `complete_via_tape` counts only the second-path wins.
- **Prompt softened**: append described as "optional scratch space; use only if
  you cannot one-shot". Agents recover one-shot behavior on easy problems
  (smoke mathd_algebra_44: 3 tx, `tool_dist: {complete:3}`), while retaining
  the option to build incrementally on hard ones.

### F-2026-04-19-07: CONSTITUTIONAL FIX — tape now load-bearing in ∏p
- **Violation**: Art. IV mermaid requires Q_t (tape) → ∏p (verification).
  Previously `oracle.verify_omega_detailed(payload)` took payload ONLY,
  ignoring all tape state. Tape was decorative; `append=0` across 4 N=50 runs
  proved agents correctly inferred that and bypassed tape.
- **Fix** (`experiments/minif2f_v4/src/bin/evaluator.rs`):
  ```
  full_proof = tape_chain_payloads.join("\n") + "\n" + payload
  oracle.verify_omega_detailed(&full_proof)
  ```
  When tape is empty, fallback preserves old behavior (no regression).
- **Prompt update** (`src/sdk/prompt.rs`): schema section now explains that
  `append` writes into Q_t and `complete` verifies `tape_chain + payload`.
- **Smoke test**:
  - `mathd_algebra_44` (easy): solved in 7 tx with `tool_dist: {append:4, search:2, complete:1}` —
    first-ever observation of agents actually using append in this session
  - `mathd_algebra_170` (hard): agents ran with `tape_nodes=3` per OMEGA claim;
    natural `err:unknown_const` rejects, not regression from the fix
- This closes the single most fundamental constitutional bug in the stack.
  Without this, the system was N-parallel-retry, not a Turing machine.

### F-2026-04-19-06: Search cap mechanism validated
- Capped retry on failed-13: **7/13 SOLVED** (vs pre-cap retry 3/13 — 2.3× improvement)
- Both 200-search pathological problems cracked:
  - `algebra_amgm_sumasqdivbgeqsuma`: 160 searches (= 8×20 cap), 4 completes, solved
  - `numbertheory_2pownm1prime_nprime`: 159 searches, 1 complete, solved
- `search_capped: 0` in telemetry — cap works by dropping search from tools list,
  agents switch to complete/invest rather than trying search again
- **Cumulative best-of across 3 runs**: 44/50 = 88% (only 2 problems fail all 3)
- Fair single-run measurement pending: clean N=50 with latest binary queued

### F-2026-04-19-05: Search budget abuse (200 tx all on search)
- Retry batch on 13 previously-failing problems with search-loop binary.
- **3/13 recovered** (mathd_algebra_196, mathd_numbertheory_447, mathd_numbertheory_5)
  - Cumulative N=50: 40/50 = 80%
  - Cannot cleanly attribute to loop closure vs run variance (no same-sample control)
- **New bug via telemetry**: 2 problems used 200 tx / 200 on `search`, zero complete:
  - `algebra_amgm_sumasqdivbgeqsuma` → `{'search': 200}`
  - `numbertheory_2pownm1prime_nprime` → `{'search': 200}`
- Law 1 says "thinking is free" → no economic pressure to stop searching
- Agents get stuck querying → never attempt OMEGA claim → definite fail
- **Fix candidate**: cap search per-agent per-problem (e.g., max 20); drop tool from
  prompt once cap exceeded. Mechanism-level (C-034), additive to search-loop closure.

### F-2026-04-19-04: Search is filename-only; agents ask symbolic queries
- Smoke test of search-loop closure: agent query `"abs (n - 2) ≤ 5 + 6 / 10"` → 0 hits
- `SearchTool::search` substring-matches filenames only; queries describing lemma
  content (inequalities, predicates) never match filenames
- Loop-closure code works (hits flow into next prompt when non-empty),
  but hit rate ≈0 on MiniF2F structure unless agent queries by theorem name
- **Follow-up options** (not yet chosen):
  (a) content grep inside `.lean` files (cheap, small index)
  (b) Mathlib lemma-name index (needs build step)
  (c) embedding search (out of scope — external dependency)
- Files: `src/sdk/tools/search.rs:24` (filename-substring only)

### F-2026-04-19-03: TEMP_LADDER N=50 confirmation — +14pp over v3.1 baseline
- **Data**: `logs/templadder_n8_20260419T013822.jsonl` (45 rows, 50 problems)
- **Primary**: 37/50 SOLVED = 74.0% vs v3.1 n1 baseline 30/50 (60%) = **+7 solves +14pp**
- **Paired 20-subset** (direct A/B vs nscaling_n8 baseline):
  - both solved 11, treatment-only 4, baseline-only 0, neither 3
  - McNemar stat 4.0 → one-sided exact p ≈ 0.0625 (N=20 borderline); effect is unambiguously positive
- **Tool-dist (C-036 telemetry)**:
  - `search: 1938` + `other:search: 359` = 2297 total, avg 51/problem (most on hard problems)
  - `invest: 43` (markets activated, modest)
  - `complete: 269` (one-shot solves dominate)
  - `append: 0` ← tape still empty across entire batch
- **1 high-correlation flag**: mathd_algebra_208 upr=0.24 (SOLVED — ladder broke through)
- **Bernoulli gap remains**: predicted N=8 ≈ 99.9%, observed 74% → tape-emptiness is next bottleneck

### F-2026-04-19-02: Art. III.2 search engine dead at swarm layer
- **Discovery**: C-036 telemetry on N=50 templadder batch showed `other:search: 149`
  on `mathd_algebra_196` — agents emit `search` calls but evaluator had no handler
  (`_ => {}` catchall silently dropped them).
- Pre-existing bug since at least `28fa25d` (HEAD~1). SearchTool was mounted
  but unreachable from swarm loop. Constitutional Art. III.2 (progressive disclosure)
  partly broken.
- **Fix**: added `"search" =>` handler that executes SearchTool and logs top hits.
  Hits are NOT yet fed back into agent prompts — minimal fix only counts and logs.
  Full integration (search results in next prompt) deferred until tape activation.
- Files: `experiments/minif2f_v4/src/bin/evaluator.rs:507`
- The N=50 templadder run started before this fix → mixed `other:search` (pre)
  and `search` (post) labels in tool_dist. Acceptable: change is additive.

### F-2026-04-19-01: TEMP_LADDER mechanism validated on N=20 sample
- **Data**: temp ladder t_i = 0.10 + i*0.15 (clamped 1.30) per agent_idx
- **Result**: N=8 + TEMP_LADDER=1 → 14/20 (70%)
  - vs baseline (fixed t=0.2) → 11/20 (55%) — Δ +3 solves, +15pp
- **3 newly solved** (all in baseline-fail set):
  algebra_apbon2pownleqapownpbpowon2, imo_1981_p6, induction_1pxpownlt1pnx
- **0 lost** (no regression on previously-solved)
- McNemar (b=3,c=0) one-sided p≈0.125 on N=20 — needs N=50 for stat-sig
- Mechanism cost: zero runtime (env var only); constitutionally aligned (Art. II.2.1)
- Files: `logs/templadder_n8_20260418T232656.jsonl`

### F-2026-04-18-03: Temperature is fixed at 0.2 for ALL agents (decorrelation gap)
- `evaluator.rs:170,314` — both oneshot and swarm use `temperature: Some(0.2)`
- 8 agents × identical temp × identical prompt (within 3 skill classes, cycled) ≈ identical output
- Hypothesis: per-agent temperature ladder will break correlation
- Cheapest mechanism-level intervention; testable in <1h on N=20 sample

### F-2026-04-17-04: Phase 3 incremental verified tactics — LLM granularity mismatch
- 445 rejected, 0 verified writes. LLM outputs full proofs, not single tactics.
- Sorry-padded check of "full proof after accumulated full proofs" = invalid Lean.
- Constitutional insight REVISED: ∏p mandates verify-before-write, NOT tactic granularity.
  The granularity should match what the LLM naturally produces.
- If LLM produces full proofs → verify_omega IS the correct ∏p (already in complete path).
- The "complete" action already satisfies: output → ∏p(oracle) → write(PPUT_RESULT).
- force-append was wrong not because it was "unverified write" but because it was
  micromanagement (auditor ruling).
- **CONCLUSION: oracle-cache branch (direct-complete + cache + broadcast) is constitutionally
  correct. The incremental approach requires tactic-level LLM output which current models don't provide.**
- Future: when LLMs can reliably output single tactics (or with fine-tuning), Phase 3
  incremental becomes viable. For now, full-proof-level verification is the right ∏p.

### F-2026-04-17-03: 🔴 Constitutional topology audit reveals fundamental design violation
- Constitution's main loop: output → ∏p(verify) → wtool(write) → Q_{t+1}
- Current code: append → write to tape FIRST → then probe/verify LATER
- This is **validate-before-write vs write-then-validate** — the order is reversed
- Constitution has NO concept of "unverified append" — every write to Q must pass ∏p FIRST
- The distinction between "append" (unverified write) and "complete" (verified write) is
  **an invention that violates the constitutional loop**
- Correct model: EVERY agent output goes through ∏p. If it passes → write to tape. If not → reject.
  The predicate for partial steps = "does this tactic step type-check in isolation?"
  The predicate for complete = "does full proof verify in Lean?"
- **This reframes the entire approach**: instead of force-append-before-complete, the
  constitutional design is: agent freely outputs tactics → each goes through type-checking
  predicate → passed tactics accumulate on tape → when chain is sufficient → OMEGA.
- Second topology finding: map-reduce is a SEPARATE clock-driven tick (not part of tx loop).
  Librarian/statistics extraction should run on a timer, not triggered per-tx.

### F-2026-04-17-02: 4-way parallel A/B final results + root cause identified
- All 4 treatments n1 = 5-6/20, control n1 = 11/20 → all ~50% below control
- oracle-cache best: n3=6 (n3>n1 ✅), Bernoulli −28%, tape=18.8, 0 timeouts
- P3-hybrid: n1=6 (not 11 as predicted) because **prompt schema still says "append first"**
- ROOT CAUSE: all treatment branches use the modified prompt.rs that says
  "Workflow: first append ONE proof step, then complete." Control uses OLD prompt
  that says "Respond with <action>{JSON}</action>" — no append-first workflow.
- The prompt modification IS the variable causing the performance drop, not the
  mechanism changes in bus.rs/evaluator.rs.
- **Next test**: run oracle-cache branch but revert prompt.rs to control's version
  (keep mechanism changes, remove prompt workflow guidance). If n1 recovers → confirmed.
- This aligns with C-034: mechanism should work WITHOUT prompt explanation. If agents
  need prompt text to use append, the mechanism design is wrong.

### F-2026-04-17-01: 3-way parallel A/B (oracle-cache / agent-verify / async-oracle)
- oracle-cache: n1=5 n3=6 (n3>n1 ✅) Bernoulli −28% tape=18.8 0 timeouts
- agent-verify: n1=6 n3=6 (n3=n1) Bernoulli −36% tape=11.0 0 timeouts
- async-oracle: 7/20 too slow, 8 timeouts — ELIMINATED
- All 3 absolute SolveRate below control (11/12) — force-append overhead
- **Best branch: oracle-cache** (highest n3, n3>n1 signal, best Bernoulli, lowest code change)
- Key insight: architecture mechanism works (tape alive, Bernoulli improving) but
  force-append overhead reduces effective tx within timeout. The 1-shot direct-complete
  path IS informationally optimal for problems where LLM can produce full proof.
- Open question for user: should we merge oracle-cache despite lower absolute? Or
  hybrid approach (force-append only for n>1 conditions, keep direct-complete for oneshot)?

### F-2026-04-16-08: max_transactions=50 is ad-hoc benchmark-fitting, RETRACTED
- User caught: reducing 200→50 is domain-specific tuning (Lean oracle ~10s) not generalizable
- Violates C-031 spirit: parameter tuning when institutional fix is needed
- Correct fix path: oracle caching / async oracle / agent-initiated probe — infrastructure, not knob
- v7 run killed. Commit reverted in intent (code stays for env-override C-027 compliance but default stays 200)

### F-2026-04-16-07: 🏆 Bundle v6 — Bernoulli excess from −31% to +0.7% (negative interaction ELIMINATED)
- Treatment: n1=1/20, n3=3/20 (absolute low due to oracle overhead)
- BUT: Bernoulli excess = +0.7% (FIRST POSITIVE VALUE IN ALL EXPERIMENTS)
- Control had −30.9% excess → treatment eliminated negative interaction completely
- n3−n1 = +2 (treatment) vs +1 (control) — correct direction, GRAY significance
- Tape depth: mean 21.7 (treatment) vs 1.0 (control) — architecture IS working
- Remaining blocker: oracle overhead (~10s per Lean probe × many probes per problem)
- Next: reduce overhead via lower max_transactions (200→50) or oracle caching
- CRITICAL INSIGHT: the architecture FIX WORKS. The bottleneck is now INFRASTRUCTURE (oracle speed), not DESIGN.

### F-2026-04-16-06: Bundle v5 A/B — tape alive but SolveRate collapsed (oracle overhead)
- Treatment: n1=3/20, n3=3/20 (vs control n1=11, n3=12). STRICT_WIN control.
- Root cause: auto-probe on EVERY append → 200tx × 10s Lean = 2000s >> 900s timeout
- But: tape depth real (mean 24.3 n1, 5.7 n3 vs control 1.0). Bernoulli excess improved +7%.
- Fix: probe every 5th append (data: successful solves had depth 5-9). Bundle v6 running.
- If v6 recovers SolveRate while keeping tape alive → architecture is working

### F-2026-04-16-05: 🏆 First OMEGA via tape collaboration (bundle v5, commit ccfd095)
- mathd_algebra_171 n1: 5 appends → tx 5 auto-probe ACCEPTED → gp_node_count=6
- **First time in v4 history**: tape depth > 0 on a solved problem
- Mechanism chain: force-append gate → schema clarification → opportunistic auto-probe
- Bundle = Art. II.1 broadcast + Fix #4 force-append + C-027 payload limits + auto-probe
- N=20 full A/B launched (v40_bundle_v5, timestamp 20260416T...)

### F-2026-04-16-04: Fix #4 solo FAILED — agents don't know to append (61 blocks, 0 appends)
- Force-append gate fired 61 times, but agents kept trying `complete` → 0 solves
- Root cause: agents receive no feedback about WHY complete was rejected (Art. II.1 broken on main)
- **Bundle required**: Art. II.1 (broadcast rejections) + Fix #4 (force append) must deploy together
- Created experiment/bundle-ii1-fix4 (cherry-pick of commits ce003e5 + e0600ad + 828d5d1)
- 104 tests pass. Running N=20 A/B (timestamp 20260416T195805)
- If bundle works: tape fills → ALL swarm mechanisms activate for first time

### F-2026-04-16-03: Fix #2 Art. III.3 context isolation — ABANDONED, tape is empty
- Treatment n3=10/16 vs control n3=12/17 → GRAY (Δ=−2)
- Bernoulli excess: control −30.9%, treatment −40.9% (worse)
- Root cause: tape depth=0 → per-agent context filter isolates NOTHING
- This reorders the priority queue: **Fix #4 (force append) must precede all other fixes**
- Without tape content: II.1 has nothing to broadcast, III.3 has nothing to isolate, II.2 has no markets
- The entire swarm architecture is dormant because agents bypass tape via direct `complete`
- **New priority**: #4 (force append) → then re-run #1 (II.1) + #2 (III.3) since they need tape

### F-2026-04-16-02: Step-B v3.3 Art. II.1 fix — n1 WINS but n3 UNCHANGED
- Treatment n1: 28/50 vs control 23/50 → +5 STRICT WIN (broadcast helps single-agent learning)
- Treatment n3: 25/50 vs control 25/50 → Δ=0 EQUIVALENT (broadcast does NOT help multi-agent)
- Bernoulli excess: control −34.3%, treatment **−41.5%** (WORSENED)
- Verdict: ABANDON merge. Art. II.1 is necessary-but-insufficient for n↑→PPUT↑
- **Root cause of n3 stagnation confirmed: Art. III.3 (correlation shielding)**
  - 3 agents see identical chain_so_far → produce correlated proofs → negative interaction
  - Art. II.1 gives them shared error info → but they ALREADY share everything → no new diversity
- **Next**: Fix #2 Art. III.3 per-agent context isolation
- Branch `experiment/art-ii1-v3` archived (tag `archive/art-ii1-v3-abandoned-20260416`)

### F-2026-04-16-01: n3 BELOW Bernoulli prediction — negative interaction confirmed
- v3.2 chat: p_scaffold (from n1) = 0.46
- Bernoulli prediction for n3 (3 independent scaffold tries) = 1-(1-0.46)^3 = 0.843
- Actual n3 = 0.500
- **Excess = −0.343 (34.3% below independent-trial expectation)**
- Interpretation: current n3 is NOT 3× independent tries; agents NEGATIVELY interfere
- Candidate mechanisms for negative interaction:
  (a) swarm prompt overhead (chain context adds noise / distracts)
  (b) shared bus state corrupts (even with broken broadcast)
  (c) resource competition (Lean oracle sequential access, etc.)
- **This reframes Step-B**: goal is not just "add cooperation" but first "remove interference"
- **Percolation frame**: current N_c = ∞ (mechanism broken → no positive interaction at any N)
- After Art. II.1 fix: N_c should become finite (≤ some reasonable value)
- **Key test**: if treatment n3 ≥ Bernoulli prediction (84.3%) → interference eliminated → mechanism adds value

### F-2026-04-15-08: Routine A auto-pushed despite "Do NOT push" prompt directive
- Drift audit committed + pushed to origin/main (5fa3803)
- Claude Anthropic remote session appears to override explicit prompt instruction for pushing new audit markdown
- Benign here (content was valuable) but authority deviation worth recording
- Implication: treat routine push as default behavior in future prompts; no harm if committing to handover/ only

### F-2026-04-15-09: v3.2 attempt 1 wasted 2 min on undetectable API contract break
- `ACTIVE_MODEL=deepseek-chat` hit `max_tokens=16000 > 8192` API cap → HTTP 400 on every call
- Plan passed dual audit (constitutional + design) but no smoke ran the pipeline
- **Lesson (mechanism-level)**: plan-audit ≠ runtime-compatibility-check. They are orthogonal gates.
- **Fix committed**: `run_interleaved.sh` now runs a single-problem smoke probe (oneshot on mathd_algebra_148) before the 50-problem batch. Aborts batch on API-class errors. Cost: ~30-60s. Saves 60-75min on broken configs.
- **Generalization**: any config change (model, max_tokens, timeout, prompt, endpoint) that touches the runtime contract should trigger a re-smoke. Pre-registration audits don't catch this class.
- **Candidate case**: C-041 "API/runtime contract drift requires mechanical smoke probe" (too early to formalize; watch for recurrence).

## 3. Retracted speculations (do not re-assert)

- **2026-04-15 ~04:30 UTC** "n3 熔断因 3 agents 互相干扰" — no evidence; was lazy inference from rotation correlation. Actual cause in F-2026-04-15-02.
- **2026-04-14** "5/244 oneshot solves are architecture baseline" — these were false-positives from missing Mathlib. True reasoner oneshot baseline awaits v3.1.
- **2026-04-14** "+33% PPUT confirms n3 architecture value" — recast as "k=3 sampling advantage" after F-2026-04-15-02 confirmed swarm channel severed.

## 3.5. North Star (decision criterion for "architecture value")

**Final goal** (user-confirmed 2026-04-15): **increasing n should super-linearly increase PPUT**.

**Authorization 2026-04-15 (user asleep / in flight, 5h absence)**: autonomous Step-B execution authorized IF data supports the judgment. If data insufficient → design new experiment to get data, don't wait. Mandate: don't stop for confirmation.

**Phase 1 implementation complete** (2026-04-16 ~01:00 UTC):
- Commit A (main@41617fb): provenance stamping + seeded RNG
- Commit B (experiment@ce003e5): classifier + bus.rs Art. II.1 TopKClasses broadcast
- Commit C (experiment@e0600ad): bus_classify write-site shield (addresses Codex Q5 HOLD)
- 104 tests pass. Gemini PROCEED. Codex stalled >60 min at Phase 1.1 re-audit (agent dead, file unchanged 63 min). Decision: proceed on {Gemini PASS + Commit C directly addressing blocker + 104 tests + bounded-label invariant verified in test_bus_classify_bounded}.

Plain language: if adding more agents doesn't produce more than k-sampling statistical advantage (i.e., n3 > n1 > oneshot by a margin beyond independent-try probability), then TuringOS architecture has not demonstrated value. Current state: n3 ≈ n1 because Art. II.1 broadcast mechanism is severed (F-2026-04-15-02) — so the multi-agent coordination never activates, and we're only measuring k-sample statistics.

**All future Step-B candidates should be judged against this**: does the change make n3 genuinely outperform n1 (not just match)? If no, Step-B is not worth its A/B cost.

## 4. Active hypotheses (under test)

### H1: Chat > Reasoner for TuringOS agents
- See `HYPOTHESIS_CHAT_MODEL_2026-04-15.md`
- Prediction: chat + scaffold forces `append` usage; graveyard populates; Art. II.1 naturally engages
- Test: v3.2 (deepseek-chat on same seed=74677 sample)
- Metric to track: `tape_depth_at_OMEGA` per condition

### H2: Single-agent scaffold (n1) provides non-trivial value beyond multi-sample
- Preliminary evidence: F-2026-04-15-04 (n1 outperforms oneshot decisively)
- Test: v3.1 completion + post-M4 audit; v3.2 chat × n1 comparison
- If chat+n1 still beats chat+oneshot → scaffold does meaningful work independent of model's internal CoT

### H3: Art. II.1 fix will restore multi-agent diversity benefit
- Rationale: F-2026-04-15-02 severs cooperation channel → current n3 ≈ 3× oneshot
- If fixed, n3 should diverge from n1 (broadcast → richer coordination)
- Test: v3.3 (after bus.rs human-confirm edit)

### H4: Swarm scaling follows percolation phase transition (user 2026-04-16)
- **See `HYPOTHESIS_PERCOLATION_2026-04-16.md`** for full framework
- Core: PPUT(N) is NOT linear; possibly log(N) or percolation (threshold N_c)
- N_c depends on mechanism quality — each Step-B lowers N_c
- Current data covers only N∈{1,3}; need N∈{1,2,3,5,8,13} to map curve shape
- **v3.3 (N=3) may show GRAY result** even if fix works, because N_c > 3
- If GRAY at N=3: run N-scaling experiment before concluding fix is useless
- **Iterative research program**: N-scaling → diagnose N_c → Step-B fix bottleneck → re-run → repeat until N_c≈2
- Connection to North Star: n↑→PPUT↑ super-linear IS the percolation regime (N > N_c)

## 5. Pending fixes requiring authorization

**Protocol for restricted-file changes**: `STEP_B_PROTOCOL.md` (necessity audit → parallel branch → A/B statistical test → merge on empirical win only). Do NOT directly edit restricted files even with authorization; always A/B test.

| Fix | File | Why | Authorization status | Protocol |
|---|---|---|---|---|
| `recent_rejections` optional global scope | `src/bus.rs` | F-2026-04-15-02 Art. II.1 broadcast | **Human confirm + Step-B** | STEP_B_PROTOCOL |
| OMEGA reject enters graveyard | `evaluator.rs` | F-2026-04-15-02 closed path | Self-approvable (evaluator.rs not restricted) | Still pre-register A/B if impacts metrics |
| WAL emission | `src/kernel.rs` or bus.rs | F-2026-04-15-03 | **Human confirm + Step-B** | STEP_B_PROTOCOL |

## 6. Constitutional debt queue

| Item | Case ref | Severity |
|---|---|---|
| `decide`/`omega` missing from bus.rs `forbidden_patterns` | C-011 | Medium (sharp test: Lean reject if agents use these) |
| `graveyard` per-author scoping violates Art. II.1 | (new) | High — systemic failure mode |
| WAL non-implementation | (new) | Medium (diagnostics only, not correctness) |
| Routine config yaml↔cloud drift (no CI) | C-017 | Low (researcher-controlled, advisory only) |
| `max_transactions`, `temperature`, `max_tokens` hardcoded without env override | C-027 | Medium (caught by Routine A 2026-04-15) |
| Art. V.1.1 + V.1.2 zero case coverage — ArchitectAI outer-loop boundaries undefined | (new) | Medium (blocks safe outer-loop activation) |

## 6.5. Constitutional topology audit (2026-04-16)

Full matrix in session log. Six 🔴 dormant mechanisms identified:
1. Art. II.1 broadcast — **Step-B v3.3 in progress** (treatment arm running)
2. Art. III.3 correlation shielding — **completely missing** (no agent isolation; highest N_c impact after II.1)
3. Agent role diversity — **missing** (all agents same prompt; skill="" empty)
4. Librarian DNA compression — **code exists, never fires** (skills/ empty, no append triggers interval)
5. Economic mechanism (market+wallet) — **code exists, fully dormant** (agents never invest)
6. map-reduce tick — **completely missing** (no macro stat cycle)

**Each fix = Step-B cycle → N-scaling → measure N_c shift.**
Priority: 1 (in progress) → 2 (highest N_c impact) → 3 (highest diversity impact) → 5 → 4 → 6

## 7. Open questions (not yet testable)

- What's the upper-bound `tape_depth` for a solved problem? (No data — need instrumented run)
- Does the `market` mechanism affect parent-selection in practice? (n3 tape empty → market empty → Boltzmann picks from nothing)
- Are there problem categories where mathd_algebra-style tactics dominate vs where structural/inductive reasoning dominates? Currently sample skews mathd.

## 8. Reference pointers

- Latest plan: `PLAN_V3_1_2026-04-15.md`
- Latest audit exchange: `AUDIT_V3_2026-04-15.md`
- Hypothesis doc: `HYPOTHESIS_CHAT_MODEL_2026-04-15.md`
- n3 diagnosis: `N3_DIAGNOSIS_2026-04-15.md`
- Constitution: `/constitution.md`
- Cases: `/cases/C-*.yaml` (35 cases as of 2026-04-14)
- Frozen sample: `experiments/minif2f_v4/analysis/sample_N50_S74677.txt` (fp=796ead6c40351ae9)
- Frozen analyzer: `experiments/minif2f_v4/analysis/frozen_analysis.py`
- Notepad (this file): `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md`

## 8.5. Iterative improvement protocol (user 2026-04-16)

**Principle**: 逐项改进，逐项测试，快。

**Per-fix cycle** (~3h wall, ~$12):
1. Pick highest-priority bottleneck from §6.5
2. Step-B implement (worktree, cargo test, ~30 min)
3. Quick A/B on **N=20 subset** (sample_N20_S74677.txt, fp=8d390ee4eef82dbb)
   - Decision: Δ≥3 → merge. |Δ|≤1 → equivalent. Δ=2 → gray.
   - Wall: ~3h chat. Cost: ~$12.
4. If WIN → merge, update notepad, pick next bottleneck
5. If GRAY → diagnose, try different fix (don't enlarge N)
6. After 3-4 fixes → **confirming experiment on full N=50** (one-shot, ~8h, ~$30)

**Power analysis**: N=20 detects Δ=3 with 57% power; Δ=5 with >80%. Same as N=50 for fixed-Δ designs. Savings: 5h + $18 per iteration → enables 2× more iterations.

**Priority queue** (from §6.5):
1. ✅ Art. II.1 broadcast (v3.3 treatment running)
2. Art. III.3 correlation shielding (per-agent context filter)
3. Agent role diversity (skill differentiation)
4. Economic mechanism activation (incentivize invest/append)
5. Librarian DNA compression
6. map-reduce tick

## 9. Plan review checklist (consult before any v3.2+ plan)

Before proposing a new experiment or commit:

- [ ] Read sections 2, 3, 4 of this notepad
- [ ] Check if proposal re-asserts a retracted speculation (section 3)
- [ ] Check if proposal tries to fix something already queued as "pending authorization" (section 5)
- [ ] Check if proposal introduces constitutional debt not in section 6
- [ ] Cite new findings in section 2 with evidence locations
- [ ] Update section 1 (active experiments) as state changes

---

## Change log

| Date | Event |
|---|---|
| 2026-04-15 06:00 | Initial creation after user directive + n3 diagnosis |
