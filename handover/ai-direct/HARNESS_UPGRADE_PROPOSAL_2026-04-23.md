# Harness Upgrade Proposal — Lessons from Paper 1 Dual-Audit

**Date**: 2026-04-23
**Trigger**: Paper 1 v1 dual-audit (Codex + Gemini) both returned CHALLENGE on 5 independent P0 blockers. Data were clean (16/16 Lean reverify, 0 forbidden patterns); the failures were in claim governance, not data integrity.
**Goal**: propose harness improvements so the next auto-research cycle does not repeat these weaknesses.
**Status**: proposal for user approval. Not yet implemented.

---

## § 1. Root-cause analysis of what the current harness missed

| Audit finding | Harness gap |
|---|---|
| Problem-selection bias (hard10 ⊂ 36) | No `preregistration/` concept; samples created ad-hoc |
| McNemar mislabeled | No multiplicity-family tracker; no pre-reg of primary endpoint |
| "Emergence" overclaim | No claim-strength linter; any doc may use strong words unchecked |
| Mechanism from N=1 ablation | No policy enforcing N≥3 seeds for causal claims |
| `build_sha = None` on 80 rows | Fix landed DURING this session (run_list.sh), but existing rows stale |
| Symmetry defense false | No automated prompt-pair comparison before claiming "symmetric" |
| Paper-level claims drifted from data | No pre-commit check comparing claim language to evidence files |

These 7 gaps form **three clusters**:
1. **Pre-registration discipline** (endpoint, sample, multiplicity, stopping rule)
2. **Claim-strength governance** (linter, N-seed policy, token-level prompt comparison)
3. **Provenance integrity** (build_sha, evidence index sync)

---

## § 2. Concrete harness upgrades

Ranked by ROI (biggest impact first).

### § 2.1 Pre-registration system (P0)

**Problem**: Current flow is "design → run → write claim". Reviewer objection: "show me you didn't pick the 10 problems after seeing 36-problem results".

**Proposal**:
- New directory `handover/preregistration/` — each experiment opens a PREREG before any `run_list.sh` invocation
- Template `PREREG_<experiment_id>_<date>.md` with mandatory sections:
  - `primary_endpoint` — one specific statistic (e.g., "paired McNemar one-sided p < 0.05 on hard-set solve count")
  - `secondary_endpoints` — full list, with Bonferroni threshold declared
  - `sample_construction` — either "random with seed X" + script OR exhaustive pool + reason
  - `stopping_rule` — N seeds, N problems, wallclock
  - `directional / two-sided hypothesis` — explicit
  - `what-would-falsify` — what outcome would make us abandon the claim
- `tools/prereg_check.py` — parses a REGISTRATION and enforces: primary endpoint is one line, multiplicity family ≥ declared, sample script deterministic

**Example that would have saved Paper 1 v1**:
```markdown
# PREREG_E1_HETEROGENEITY_2026-04-22

primary_endpoint:
  statistic: McNemar exact binomial one-sided
  sample: 10 problems randomly drawn from the 36-problem hard pool
          via sample.py --seed 31415_141421 --hard-pool-source FILE
  threshold: p < 0.05 (one-sided)

secondary_endpoints:
  - easy-set Δ (descriptive only, not inferential)
  - per-seed dominance (descriptive only)
  - ablation mechanism (descriptive only, Bonferroni α=0.0125)

directional_hypothesis: B > A (heterogeneous > homogeneous)

stopping_rule: 4 Boltzmann seeds × 10 problems; abandon if per-seed var > X

what_would_falsify: A-unique ≥ 2 across 4 seeds, OR easy-set Δ > 0
```

If we had pre-reg'd this, both audits' P0-1 + P0-2 go away.

**Effort**: 1 day dev for template + check script.

### § 2.2 Claim-strength linter (P0)

**Problem**: The draft used "emergence", "strictly dominates", "first application of...", "mechanism" without corresponding-evidence gates.

**Proposal**: `tools/claim_lint.py` scans `handover/ai-direct/PAPER_*_DRAFT*.md` for strong-claim regexes:

| Regex | Required evidence |
|---|---|
| `\bemergence\b` / `\bemergent\b` | Cite N=? seeds, statistical test, p-value, one-sided/two-sided, sample construction script |
| `\bmechanism\b` / `\bcausal\b` | Cite ablation N≥3 seeds OR propensity/IV |
| `\bfirst\b.*application` / `\bnovel\b` / `\bunprecedented\b` | Cite prior-art search (Google Scholar / arXiv) + negative result |
| `strictly dominates` | Cite 100% containment across all subgroups |
| `significantly` / `highly significant` | Cite p-value + multiplicity correction |

Linter emits `MUST-CITE: <claim> line N` warnings; PR blocked until each claim has a `<!-- evidence: file.md § X -->` comment right after.

**Example**: the phrase "This identifies the meta-strategic role — not arbitrary heterogeneity — as the mechanism" would fire `\bmechanism\b` → demand N≥3 ablation seeds. Since we had N=1, commit blocked.

**Effort**: 1 day dev for linter + integration with git pre-commit hook.

### § 2.3 N≥3 seed policy for causal claims (P0)

**Problem**: We ran ablation on 1 seed and claimed mechanism.

**Proposal**: `tools/ablation_gate.py`:
- Reads a REGISTRATION's `causal_claims` list
- For each, scans `handover/evidence/` for matching jsonl files
- Requires: same sample × ≥3 seeds × same other-controls
- Emits CHALLENGE if fewer; block paper draft commit until satisfied

**Effort**: 0.5 day dev.

### § 2.4 Prompt-symmetry checker (P1)

**Problem**: We claimed skill_0 and skill_3 prompts are "symmetric in specificity". Both Codex + Gemini called bullshit — skill_3 is meta-cognitive instruction, skill_0 is object-level tool list.

**Proposal**: `tools/prompt_symmetry_check.py`:
- Token-count comparison per skill prompt
- Classify each token: { tactic_name, verb, meta_instruction_keyword ("review", "propose", "shift"), noun }
- If two prompts claimed symmetric but class-histograms differ by > ε → linter fires

**Alternative (simpler)**: any paper claim of "prompt X and prompt Y are symmetric" requires `symmetry_test.md` showing token-class breakdown side-by-side.

**Effort**: 0.5 day.

### § 2.5 Evidence archive auto-sync (P1)

**Problem**: `handover/evidence/README.md` listed 8 E1 batches but 11 existed (added seed 2357 + ablation mid-session; README not updated).

**Proposal**: `tools/evidence_sync.sh` in pre-commit hook:
- Scan `handover/evidence/e1_jsonl/` + `phase9a_jsonl/`
- Compare to tables in evidence README.md
- Fail commit if any jsonl exists without README row

**Effort**: 0.5 day.

### § 2.6 build_sha enforcement in evaluator (P1, partly done)

**Problem**: Original 80 rows had `build_sha = None`. Session fix at run_list.sh level (`BUILD_SHA=$(git rev-parse --short HEAD)`), but evaluator should ALSO enforce.

**Proposal**: evaluator.rs fail-fast: if `BUILD_SHA` env unset AND git rev-parse fails, refuse to run.

```rust
let build_sha = std::env::var("BUILD_SHA")
    .ok()
    .or_else(|| std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output().ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string()))
    .expect("BUILD_SHA unset and git rev-parse failed — cannot record provenance");
```

**Effort**: 15 min.

### § 2.7 Claim → evidence linking scheme (P2)

**Problem**: Paper 1 v1 has dangling pointer "deepseek-chat snapshot referenced in § 5" — § 5 has no such reference.

**Proposal**: Markdown convention `[[evidence:<file>:<section>]]` — any claim needs this tag. A `tools/check_refs.py` validates all such tags resolve. Borrow from Zettelkasten / Obsidian linking.

**Effort**: 0.5 day.

---

## § 3. Priorities for next auto-research cycle

If user wants a new auto-research session, here's the order of harness fixes BEFORE starting:

| Priority | Fix | Effort | Rationale |
|---|---|---|---|
| Must-do | § 2.1 Pre-registration | 1 day | The single biggest P0 risk; reviewers will otherwise kill paper |
| Must-do | § 2.3 N≥3 ablation policy | 0.5 day | Prevents the "Meta-Planner from N=1" class of error |
| Must-do | § 2.6 build_sha fail-fast | 15 min | Cheap; eliminates provenance defect |
| Should-do | § 2.2 Claim linter | 1 day | Prevents "emergence" drift in prose |
| Should-do | § 2.4 Prompt symmetry | 0.5 day | Prevents future "symmetric" defense issues |
| Nice-to-have | § 2.5 Evidence auto-sync | 0.5 day | Saves index-staleness pain |
| Defer | § 2.7 Claim→evidence links | 0.5 day | Quality-of-life, not correctness |

**Total "must-do" effort**: ~1.75 days before next research starts.

**Must-do is not blocking for pure data-gathering work** (e.g., running more Phase 9.A seeds, or re-running E1 on larger hard-pool draw). Only blocks paper drafting.

---

## § 4. What the user should decide before the next auto-research

1. **What is the next research question?** E1 was emergence. Next candidates:
   - E1 v2: redo with pre-registration + full 36-problem hard pool (the fix for Paper 1)
   - E2: larger N model-independence test (GPT-4 / Claude / Gemini as δ)
   - E3: Meta-Planner ablation at N=4 seeds (directly addresses P0-5)
   - E4: depth-chain forcing (agents forbidden from one-step solves → tests if depth emergence actually matters)
   - Alternative: pause research, draft Paper 1 v2 + resubmit to dual-audit

2. **Harness upgrades first?** My recommendation: do § 2.1 + § 2.3 + § 2.6 (≈1.75 days) BEFORE next experiment. These prevent 80% of the P0 risks.

3. **Budget**: dual-audit round #2 will cost ~$2-3 (Gemini API + Codex subagent). Worth it.

4. **Scope**: Paper 1 rework is ~10h + $22 per DUAL_AUDIT_PAPER1_VERDICT § 5. If user wants to do that first (fix, not new research), that's defensible.

---

## § 5. Updated AUTO_RESEARCH_NOTEPAD cross-reference

`F-2026-04-23-02` in `AUTO_RESEARCH_NOTEPAD.md` records this dual-audit event and points to:
- `handover/audits/CODEX_PAPER1_AUDIT_2026-04-23.md`
- `handover/audits/GEMINI_PAPER1_AUDIT_2026-04-23.md`
- `handover/audits/DUAL_AUDIT_PAPER1_VERDICT_2026-04-23.md`
- `cases/C-070_preregistration_and_multiplicity_discipline.yaml`
- this file (`handover/ai-direct/HARNESS_UPGRADE_PROPOSAL_2026-04-23.md`)

All indexed, searchable, committed.

---

## § 6. Summary for user

**Lesson learned**: our harness is great at code and data integrity (Rust is strict, TRACE_MATRIX + conformance tests work, adversarial audit scripts catch real issues). It is weak at **claim integrity** — governing what we say about the data before writing it up.

**Three harness additions** would close most of the gap:
1. Pre-registration directory + template
2. N≥3 ablation policy
3. Claim-strength linter

**Recommended path for next auto-research**:
- Spend 1.75 days implementing the "must-do" harness upgrades
- Then redo Paper 1 pre-registration + rerun on full 36-problem pool (addresses P0-1, P0-5 simultaneously)
- Then dual-audit round 2

Or — if speed matters more than claim strength — skip harness and just fix Paper 1 per DUAL_AUDIT_PAPER1_VERDICT § 6. User's call.
