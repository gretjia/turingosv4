# TuringOS v4 Auto-Research Notepad

**Purpose**: single source-of-truth for ongoing research state. Consult before any plan review or new experiment design. Update after every major finding.

**Hook**: `MEMORY.md` → `project_auto_research_notepad.md` points here. Loaded every session.

**Last updated**: 2026-04-15 ~06:00 UTC

---

## 1. Active experiments

| ID | Status | Details |
|---|---|---|
| v3.1 | Running ~27/50 | Batch ts=20260415T013559; conditions oneshot/n1/n3; seed=74677; started 01:35 UTC |
| v3.2 | Queued post-M4 | Chat-model test (deepseek-chat replacing deepseek-reasoner); same sample |
| v3.3 | Deferred | Requires Art. II.1 broadcast fix (bus.rs human confirm) |

## 2. Confirmed findings (evidence-backed, non-speculation)

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
