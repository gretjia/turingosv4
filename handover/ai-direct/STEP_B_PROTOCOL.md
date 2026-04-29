# Step-B Protocol: Restricted-File Changes via Parallel Branch Experiment

**Scope**: any change to files in CLAUDE.md's restricted list (currently `src/kernel.rs`, `src/bus.rs`, `src/sdk/tools/wallet.rs`). Also applicable to any proposal that touches "institution" per C-031.

**Principle**: real data decides, not opinion. Parallel branch lets us test change empirically before merging.

---

## Phase 0 — Necessity audit (external, before any code)

External auditors (**Codex + Gemini**) answer:
- Is the change *necessary*? What observable behavior is broken now?
- Is a less-invasive alternative available? (same effect, no restricted-file touch)
- What's the *minimum sufficient* version? (avoid scope creep)
- What's the failure mode if we don't change?

**Gate**: if both auditors say "less-invasive alternative exists", take that path instead. If both say "change is necessary", proceed to Phase 1. If they disagree, take the conservative verdict per `feedback_dual_audit_conflict`.

**My role**: write the audit brief; do not pre-advocate. If I agree with the verdict after reading both, proceed. If I disagree, record dissent in notepad §7 before proceeding.

---

## Phase 1 — Parallel branch creation

### 1a. Worktree spawn
```bash
git worktree add .claude/worktrees/stepb-<slug> -b experiment/<slug>
```

Or via Agent tool with `isolation: worktree` frontmatter — for short experiments (<2h). Agent returns branch name + diff summary.

### 1b. Implementation in isolation
- Work in the experiment branch only.
- Main branch stays at the last audited-PASS state.
- Add fixture tests covering the change.
- Run `cargo test` on experiment branch → must be green.
- Commit with message `experiment/<slug>: <change>` (not merged).

### 1c. Implementation audit
External auditors (same two) review the **diff only**:
- Is the change minimal?
- Are tests sufficient?
- Does it introduce new constitutional debt?
- Any risk the diff itself is a Trojan (side-effects beyond scope)?

**Gate**: both PASS on diff → proceed to Phase 2. Any VETO → abandon branch or revise.

---

## Phase 2 — Statistical A/B test (pre-registered)

### 2a. Pre-register experimental design
Before any batch run, write a locked spec to `handover/ai-direct/AB_<slug>_<date>.md`:

- **Null hypothesis (H0)**: p(treatment solves) = p(control solves) on frozen sample
- **Primary metric**: SolveRate delta (per `metrics.yaml`)
- **Secondary metrics**: Aggregate_PPUT, mean wall time
- **Sample**: identical to M4 (seed=74677, N=50, fingerprint=796ead6c40351ae9)
- **Sample size justification**: binomial approximation — a 3-solve difference (6 pp on N=50) is ~2σ above null; 6-solve difference (12 pp) is ~4σ. Pre-register **strict-win threshold = ΔSolveRate ≥ 3** (consistent with v3.1 decision rule).
- **Statistical test**: McNemar's test for paired nominal data. Report p-value but do not use as primary (SolveRate delta is).
- **Decision rule**:
  - `ΔSolveRate ≥ 3`: treatment strict win → merge candidate
  - `-1 ≤ Δ ≤ 2`: inconclusive → either expand N (same seed family) or abandon
  - `ΔSolveRate ≤ -1`: treatment regresses → abandon branch
- **Interleaving**: run both conditions on each problem alternately (or in parallel if possible) to neutralize API drift (C-033).
- **Abort gate**: per-condition 20% / 30% (same as v3.1).

### 2b. Execute on both branches
- **Control branch** (main or last-PASS HEAD): run `run_interleaved.sh` with `TREATMENT_LABEL=control`
- **Treatment branch** (experiment/<slug>): run same script with `TREATMENT_LABEL=treatment`
- **Cost**: 2× the single-arm experiment. Pre-registered budget before spending.

### 2c. Freeze analyzer
`frozen_analysis.py` for A/B extension:
- Must support `--control <jsonl>` and `--treatment <jsonl>` flags
- Outputs: paired-comparison table, McNemar p, ΔSolveRate, discordant pairs list
- Must be fixture-tested before A/B run (C-012 mandatory freeze).

---

## Phase 3 — Verdict + commit path

### 3a. Read the data
After both branches finish, run `frozen_analysis.py --control ... --treatment ...`.

### 3b. Audit the verdict (again)
External auditors see **data only** (no researcher interpretation). They apply the pre-registered decision rule and return PASS / FAIL / INCONCLUSIVE.

### 3c. Merge or abandon
- **Treatment win (audits PASS)**: 
  - `git merge experiment/<slug> --no-ff` on main
  - Update notepad §2 with new F-id
  - If new constitutional pattern emerged → write case candidate
- **Treatment lose or inconclusive**:
  - `git branch -D experiment/<slug>` (or archive as tag `archive/<slug>_<date>`)
  - Update notepad §3 (retracted hypotheses) with finding
  - No commit to main; keep last-PASS HEAD

### 3d. Cleanup
- Remove worktree: `git worktree remove .claude/worktrees/stepb-<slug>`
- If branch archived: `git tag archive/<slug>_<date> experiment/<slug>` then delete branch

---

## What this protocol guarantees

1. **C-010**: external audit at 3 junctures (necessity, diff, verdict). Researcher cannot self-approve a restricted change.
2. **C-012**: pre-registered metrics, frozen sample, fixture-tested analyzer — no post-hoc metric shopping.
3. **C-033**: paired-comparison on same problems cleanly attributes any change to the code diff, not to sample drift.
4. **C-034**: the change is a mechanism edit validated by mechanism-level data, not prompt tuning.
5. **Art. V.2**: pre-registered budget. No scope creep.

## What this protocol does NOT do

- **Statistical power** is limited by frozen N=50. Small effects (Δ = 1-2 solves) stay inconclusive. Pre-register a larger N (100? 244?) if effect is expected small.
- **Non-stationary effects** (model drift, API latency changes) across days are not controlled unless both branches run interleaved or within same hour.
- **Multi-batch meta-analysis** is not automated; manual aggregation if we run the same A/B protocol multiple times.

---

## Integration with existing harness

| Existing artifact | How it connects |
|---|---|
| `AUTO_RESEARCH_NOTEPAD.md` §5 | Lists pending Step-B changes |
| `run_interleaved.sh` | Accepts `TREATMENT_LABEL` env for branch attribution |
| `frozen_analysis.py` | Extend with `--control`/`--treatment` for A/B |
| Routine A (daily drift) | Will detect if experiment/<slug> branch has uncommitted drift |
| Routine B (disabled) | N/A |
| Memory `project_auto_research_notepad` | Reminds to check if a Step-B change is due parallel-test not direct edit |

---

## First application candidate

**`src/bus.rs recent_rejections` Art. II.1 fix** (notepad §5, F-2026-04-15-02):

- Phase 0: necessity audit — agents converge on same hallucination; recent_errors broadcast currently per-author-only. Is the fix needed, or is v3.2 chat-model test a cheaper alternative?
- Phase 1: branch `experiment/art-ii1-global-graveyard`
- Phase 2: A/B on seed=74677 N=50 (same as v3.1)
- Phase 3: merge only if ΔSolveRate(n3) ≥ 3

Not to execute now — blocked on v3.1 completion + v3.2 comparison per notepad sequence.
