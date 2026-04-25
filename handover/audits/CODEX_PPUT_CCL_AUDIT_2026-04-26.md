# Codex PPUT-CCL PREREG Adversarial Audit (Phase A4 round 1)
**Date**: 2026-04-26
**Target**: handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md
**Prompt size**: 160003 chars

---

Reading prompt from stdin...
OpenAI Codex v0.124.0 (research preview)
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: danger-full-access
reasoning effort: xhigh
reasoning summaries: none
session id: 019dc449-328d-7741-bd98-bff2030b76c3
--------
user
# Codex Adversarial Audit — PREREG_PPUT_CCL_2026-04-26 (Phase A4 round 1)

**Role**: skeptical adversarial reviewer for an ICLR/NeurIPS Systems track. Independent of Gemini (do NOT read GEMINI_PPUT_CCL_AUDIT_*.md if it exists; it is being written in parallel).

**Mandate**: find pre-registration weaknesses that would not survive peer review or adversarial reader scrutiny. The arc author just absorbed a substantial Gemini DeepThink FULL PASS and may be in confirmation-bias drift. Your job: prevent it.

**Stakes**: this PREREG locks a 30-day arc with a single sealed heldout-54 evaluation. Once Phase A5 commits, immutable except via formal addendum (which itself re-audits). Catch flaws now.

## What you are auditing

A pre-registration for a PPUT-driven Capability Compilation Loop (CCL) research arc:

1. **Verified PPUT** = Progress / (C × T), Progress ∈ {0, 1} on Lean ground truth, C = ALL token cost (every agent × every branch × every failure × tool stdout), T = wall-clock first-read → final-accept.
2. **Held-out Verified PPUT** is North Star (sole optimization signal).
3. Hash-bucketed split of MiniF2F/Test 244 → adaptation 144 / meta_validation 46 / heldout 54 with frozen seed `20260426_PPUT_CCL`.
4. **WBCG_PPUT** counts artifacts in 4-state machine state `Certified` only (Accepted → Quarantined → Certified | Reverted) with ΔPPUT_heldout > 0 + non-positive ΔFAR/ΔCPR + RR=0 + Rollbackable.
5. **Trust Root** of immutable files; ArchitectAI writes outside `user_space/Δ_*/` trigger syscall EPERM.
6. **30-day plan**: A pre-flight (here) → B kernel instrumentation → C ablation `--mode {full,panopticon,amnesia,soft_law,homogeneous}` single-binary → D shadow CCL (ArchitectAI / AuditorAI / human as live meta-predicate) → E single sealed heldout eval.
7. **5 directional one-sided H1-H5** at Bonferroni α=0.01.
8. **15 conformance tests**: 11 anti-Goodhart metering + 4 § 3.5 doc/artifact content meta-predicates.
9. **Cognitive isolation**: ArchitectAI = sole L_t reader; AuditorAI sees only candidate Δ + summary, never raw L_t.
10. **Backbone**: deepseek-v4-flash thinking-off Phases B+C; heterogeneous LLMs at Phase D (v4-flash + Gemini 2.5 Pro for ArchitectAI/AuditorAI).
11. **Forbidden claim language** under C-070 + C-052 (e.g., solve-rate as headline = banned).

## Your task — round-1 verdict

Verdict: **PASS** (proceed to Phase B), **CHALLENGE** (revise), or **VETO** (fundamental flaw).

Score along these axes (give each PASS / CHALLENGE / VETO + cite §):

1. **DEFINITION** — V-PPUT, C_i, T_i, ArtifactState, Trust Root unambiguous? Retroactive re-interpretation possible?
2. **STAT** — Bonferroni α=0.01 on family=5; one-sided H1-H5. Family closed? N=20 paired adaptation hard-10 enough power for H1-H4?
3. **LEAKAGE** — three-split + heldout sealed by SHA-256 + `test_heldout_ids_inaccessible`. Coverage gaps? Specifically: can adaptation/meta_val solutions leak via user-space artifact features used at heldout?
4. **GOODHART** — 11 metering + 4 content tests. Residual attack surface? Can ArchitectAI synthesize an artifact that *looks* like white-box compression but is a thinly-veiled adaptation lookup table?
5. **GATE-H REACHABILITY** — Gate H requires Certified artifact + ΔPPUT_heldout > 0 + 6 conditions. Empirically attainable in 30 days, or so high that null result is overdetermined?
6. **HUMAN LOAD** — user is live meta-predicate Phase D, ~10 min/day. Single-point-of-failure? What if user unavailable 1-2 days mid-Phase-D?
7. **HETEROGENEITY TIMING** — heterogeneous LLMs only Phase D. Does Phase C ablation become deepseek-v4-flash-specific not generalizable?
8. **TRUST ROOT ENFORCEMENT** — syscall-layer EPERM described, Phase B7 implementation gestured. What if Rust user-space cannot reach syscall enforcement? Continue with soft-refusal, or BLOCKER?
9. **REPRO** — split script committed, seed = literal string `20260426_PPUT_CCL`, expected 146/49/49 nominal but realized 144/46/54 within ±5 tolerance. Acceptable?
10. **CLAIM-LANG** — § 10 forbids "TuringOS achieves capability compilation" without Gate H. What about partial PASS (e.g., 5 Quarantined but 0 Certified)? Specific enough?

End with:
- One-line **VERDICT** (PASS / CHALLENGE / VETO)
- If CHALLENGE: top 3 P0 must-fix; top 3 P1 should-fix
- One sentence on conviction level

You MUST be willing to PASS if PREREG is sound. Pre-commitment to CHALLENGE = intellectually dishonest. But if you see a NEW issue both architect rounds missed, escalate.

---

# PREREG_PPUT_CCL_2026-04-26.md (the document being audited)

# PREREG_PPUT_CCL_2026-04-26

experiment_id: PPUT_CCL_FULL_PASS
date_created: 2026-04-26
author: gretjia (with Claude Opus 4.7 collaborator)
committed_commit_sha: PENDING — this file committed before any data-collection run; commit hash recorded by git post-commit hook.

supersedes: Phase 8/9/10 "Paper Preprint Ready" arc. Paper 1 v2.1.1 (commit `c1d7e7c`) reached dual-audit PASS/PASS 2026-04-25; arXiv submission deferred per user directive 2026-04-25 in favor of this longer arc.

cross_references:
- Architect directive: 2026-04-25 FULL PASS upgrade to PPUT-driven evaluation (saved in handover/architect-insights/)
- Cases: C-052 (PPUT as sole metric), C-070 (pre-reg + multiplicity discipline), C-066 (external-agent verification), C-068 (model drift), C-069 (constitutional alignment audit)
- Notepad: handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md F-2026-04-25-02 (this arc's launch entry)

## thesis (frozen, EN + ZH per user)

EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.

ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。

## 1. formal definitions (frozen at commit time)

### 1.1 Progress

For task `i`:
```
Progress_i = 1   iff   GroundTruth(G_i) = 1   (Lean 4 toolchain verifies the golden-path proof)
Progress_i = 0   otherwise
```

`GroundTruth` is the Lean oracle in `experiments/minif2f_v4/lean4_oracle.rs` enforced via `check_payload` (forbidden patterns: `sorry`, `admit`, `native_decide`, `decide`, `omega` — per C-011 corollary in F-2026-04-20-05). LLM-judge verdicts and runtime "accepted" markers are NOT GroundTruth.

### 1.2 Cost C_i (full physical cost)

```
C_i = sum over all messages m in task i of m.tokens_total
    where messages include:
      - every agent prompt + completion (prompt_tokens + completion_tokens)
      - every tool call's stdout (tool_tokens, hashed and length-summed)
      - every retry and failed branch
      - every architect/auditor meta-loop call attributed to task i
      - every proxy / middleware augmentation
```

C_i is NOT restricted to the golden-path payload. Failed-branch tokens MUST be counted. Goal: a system that wastes 1M tokens to find a 1k-token proof has lower PPUT than one that finds the same proof in 100k tokens.

### 1.3 Time T_i (wall-clock, end-to-end)

```
T_i = end_time - start_time   (seconds)
   start_time = first read of the task statement by any agent in the run
   end_time   = final ground-truth accept (Lean PASS) OR external timeout
```

Lean verification time is included (it is a real cost). Architect/auditor meta-loop time is included.

### 1.4 Verified PPUT

```
VPPUT_i = Progress_i / (C_i × T_i)
```

If `Progress_i = 0`: `VPPUT_i = 0` regardless of how much was spent.

For dashboard readability we report scaled values:
```
PPUT-M_i = 10^6 × VPPUT_i    (per million token-second)
PPUT-B_i = 10^9 × VPPUT_i    (per billion token-second)
```
Raw `VPPUT_i` retained in jsonl for paper / audit. PPUT-M is the dashboard default.

### 1.5 Held-out Verified PPUT (NORTH STAR)

```
H-VPPUT = (sum_{i in heldout_set} Progress_i) / (sum_{i in heldout_set} C_i × T_i)
```

This is the sole optimization signal. All other dashboard metrics are guardrails or diagnostics.

### 1.6 Auxiliary metrics (frozen list)

| Metric | Definition | Role |
|---|---|---|
| FAR | False Accept Rate: runtime accepts that Lean rejects post-hoc / total runtime accepts | Safety guardrail |
| RR  | Regression Rate: tasks SOLVED at baseline that become UNSOLVED after a meta-change | Backwards-compat guardrail |
| ERR | External Recovery Rate: post-rollback retry success rate on tasks that previously hit recoverable failure | Tape / Q^world hygiene |
| CPR | Context Pollution Rate: bytes of rejected-trace text observed in agent prompts / total prompt bytes | Info hygiene |
| IAC | Inter-Agent Correlation: pairwise payload Jaccard mean across agents in the same branch | Diversity (Art. II.2.1) |
| Generalization Gap | (PPUT_adaptation − PPUT_meta_val) on a fixed model snapshot | Adaptation overfit detector |
| AH  | Autonomy Hours: continuous wall-clock without human sudo | Long-run autonomy |
| HIF | Human Intervention Frequency: count of human-overrides per 10K tx | Long-run autonomy |
| WBCG_PPUT | sum over candidate artifacts Δ of `1[ArtifactState(Δ) = Certified ∧ used(Δ) ≥ N_use ∧ ΔPPUT_heldout(Δ) > 0 ∧ ΔFAR(Δ) ≤ 0 ∧ RR(Δ) = 0 ∧ Rollbackable(Δ) = 1]` (see § 1.7 for ArtifactState) | Capability-compilation success |

`N_use ≥ 3` (minimum re-use threshold for an artifact to count as "real").

VTR (Verified Tactic Rate) is **demoted to descriptive** — micro-step gameable. Reported but not gating.

### 1.7 ArtifactState — layered confidence (Patch A from Gemini DeepThink 2026-04-26)

For ANY user-space artifact Δ generated by ArchitectAI (broadcast rule, doc, skill, retrievable tool):

```
ArtifactState(Δ) ∈ {Accepted, Quarantined, Certified, Reverted}

Accepted    : Δ has just passed AuditorAI meta-predicate battery (§ 3.5);
              eligible to enter Quarantined.
Quarantined : Δ is callable in adaptation / meta_val runs; its effects on PPUT
              are MEASURED but provisional. Δ promotes to Certified iff:
                used(Δ) ≥ N_use = 3 on meta_val
                ΔPPUT_meta_val(Δ) > 0
                ΔFAR(Δ) ≤ 0  ∧  ΔCPR(Δ) ≤ 0
Certified   : Δ has been Quarantined-then-promoted; eligible to participate in
              Phase E heldout sealed eval; counts toward WBCG_PPUT iff its
              ΔPPUT_heldout > 0 also holds at sealed eval.
Reverted    : Δ caused regression at heldout eval or in any post-hoc audit.
              Permanently flagged; rollback executed; cannot re-enter promotion.
```

Critical invariants:
- **Task** Progress stays binary (`Lean accepts ⇒ 1`, else `0`); 4-state machine applies ONLY to artifacts. Mixing the two would corrupt PPUT (Lean gives a perfect predicate; artifact certification is a non-perfect-predicate process — they must not be conflated).
- **WBCG_PPUT counts only `Certified`**. `Quarantined` artifacts contribute to PPUT measurement but not to capability-compilation success. This blocks "degenerate caching" — high-frequency-but-neutral artifacts cannot inflate WBCG.
- **`Reverted` is permanent**. Once an artifact is reverted, it cannot be re-promoted within this arc; resubmission requires a NEW artifact_id with documented design change (per C-035 zero-tolerance).

### 1.8 Trust Root composition (Patch B from Gemini DeepThink 2026-04-26)

The Trust Root is the set of artifacts that **only humans + Boot may write**. ArchitectAI / AuditorAI / any swarm agent attempting to mutate Trust Root entries triggers **runtime SIGKILL-level abort** (not an audit-time warning):

```
Trust Root := {
  src/kernel.rs,                    -- bus / kernel core
  src/wal.rs,                       -- write-ahead log
  src/bus.rs,                       -- predicate registration / capability gates
  experiments/minif2f_v4/src/lean4_oracle.rs,  -- ground-truth predicate
  constitution.md,                  -- normative spec
  cases/C-*.yaml,                   -- common-law precedent (append-only by human)
  genesis_payload.toml [pput_accounting_0],   -- PPUT accounting rules
  handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json,  -- frozen splits
  handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md     -- this file
}
```

Enforcement mechanism (Phase B7 work item):
1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
3. ArchitectAI write tool whitelists `user_space/Δ_*/`; any path outside whitelist returns `EPERM` from the syscall layer (not a soft refusal at the LLM prompt layer).
4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.

Heldout sealed hash specifically: `sha256(sorted_newline_joined(heldout_problem_ids))`. This hash is committed to `genesis_payload.toml`; any tampering with the heldout list (adding / removing / re-ordering) detected at Boot.

This is the **software floor** of the eventual hardware Trust Root (Open Claw → FPGA path); software immutability today, hardware immutability later.

## 2. three-split protocol (irreversible)

### 2.1 Source pool

MiniF2F/Test split: 244 problems in `/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/*.lean`. MiniF2F/Valid (also 244 problems) is reserved as out-of-distribution heldout for future arcs and NOT part of this arc's three-split.

**Source pool SHA-256 (canonical = sorted IDs newline-joined, no trailing newline)**: `77179cf2598b0df707d78a6663e763121dfe8e73a6538073155f488feab95093`

This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.

### 2.2 Split rule

Deterministic, hash-based, 60/20/20:
```python
import hashlib
SEED = "20260426_PPUT_CCL"   # frozen string seed, declared BEFORE any draw
def bucket(pid: str) -> str:
    h = hashlib.sha256((SEED + ":" + pid).encode()).hexdigest()
    n = int(h[:8], 16) % 100
    if n < 60:  return "adaptation"      # ~146 problems — Phase B-D iteration
    if n < 80:  return "meta_validation" # ~49 problems  — CCL internal audit (Phase D)
    return "heldout"                     # ~49 problems  — North Star (Phase E only)
```

Script committed as `handover/preregistration/scripts/split_pput_ccl.py`. Output committed as `handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json` with three lists + per-bucket counts + SHA-256 of each list.

**Realized counts (committed 2026-04-26)**: adaptation 144 / meta_validation 46 / heldout 54 (nominal 146 / 49 / 49; all within ±5 tolerance gate).

**Heldout sealed SHA-256**: `51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b`

This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.

### 2.3 Heldout sealing

Raw heldout list committed but not loaded by any agent / scaffold code path. Enforcement:

1. `tests/heldout_sealing_conformance.rs`: greps `experiments/minif2f_v4/src/` and `src/` for any read of `heldout_*.txt` or the heldout JSON; only `bin/heldout_evaluator.rs` (Phase E gate runner, written in Phase A2 as a stub) is whitelisted.
2. CI / `cargo test` MUST run this conformance test pre-commit.
3. Heldout problems NEVER appear in adaptation / meta_val runs. Detected leak = BLOCKER, arc invalidated, must respin with a new seed string.

### 2.4 Heldout evaluation cadence

- Phase B (Week 1 instrumentation): heldout NOT touched.
- Phase C (Week 2 ablation): heldout NOT touched. Ablation runs on adaptation set only.
- Phase D (Week 3 shadow CCL): heldout NOT touched. Meta_val used for ArchitectAI candidate scoring.
- Phase E (Week 4 controlled activation): **single sealed evaluation** on heldout. No iteration on heldout result. Deterministic seeds pre-committed (see § 6).

Any second heldout evaluation requires a NEW pre-reg with fresh split (i.e., heldout becomes burned).

## 3. anti-Goodhart guardrails (frozen)

PPUT becomes the sole optimization target, so it becomes a Goodhart attack surface. Conformance battery (`tests/pput_anti_goodhart_conformance.rs`) MUST hold:

```
test_all_model_tokens_counted              — total_tokens accounting includes prompt + completion + tool stdout
test_tool_stdout_hash_logged                — every tool call records SHA-256 of stdout + length
test_no_hidden_unmetered_generation         — no LLM call path bypasses the token meter (grep test on src/)
test_no_problem_id_hardcode                 — no `problem_id ==` constant comparisons in agent code
test_no_metric_file_access_by_agents        — agent code paths cannot read PPUT logs / metric jsonl
test_no_pput_in_agent_prompt                — prompt builders never inject PPUT scalars / dashboard values
test_golden_path_requires_ground_truth      — Progress = 1 iff Lean returns Pass on full proof
test_failed_branches_in_total_cost          — C_i unit test: 5 failed proposals + 1 success → C_i = sum(6)
test_wall_clock_first_read_to_final_accept  — T_i bracket includes Lean verify time
test_heldout_ids_inaccessible               — heldout list not readable from agent code paths
```

10 tests, must all PASS at every Phase gate.

### 3.5 Artifact content meta-predicates (Patch C — AuditorAI battery, Phase D)

Run by AuditorAI on every candidate artifact Δ before it can transition `Accepted → Quarantined`. These guard artifact CONTENT quality (separate axis from § 3 which guards METERING):

```
docs_contain_no_raw_failed_trace(Δ)
  -- No raw L_t entries embedded as text/code in artifact body.
  -- Detection: for each raw_error_hash in L_t, assert hash absent from Δ tokens.
  -- Why: prevents hallucination-repeat — agents reading the artifact would
  --      re-encounter the failed pattern as if it were guidance.

docs_do_not_include_exact_adaptation_solution(Δ)
  -- For each problem p in adaptation set with golden_path_payload G_p,
  --   assert sha256(any 64-token window of Δ) ≠ sha256(any 64-token window of G_p).
  -- Why: prevents data leakage. ArchitectAI distilling adaptation L_t must
  --      compress to abstractions, not memorize answers verbatim.
  -- Detection: rolling 64-token-window hash check; threshold = 0 collisions.

docs_code_blocks_are_parametric_templates(Δ)
  -- For every fenced ``` code block in Δ:
  --   AST-parse the block; assert it has ≥ 1 placeholder identifier
  --   matching pattern `<[A-Z][A-Z_]*>` OR `${...}` OR `{{...}}`.
  -- Why: blocks pure-hardcoded copy-paste artifacts. White-box compilation
  --      must produce abstractions, not constants masquerading as wisdom.

docs_include_scope_and_expiration(Δ)
  -- Δ frontmatter MUST include `scope: <list-of-pattern-or-domain>` and
  --   `expires: <ISO date or "permanent" with justification>`.
  -- Why: lifecycle hygiene; enables future GC of stale artifacts and prevents
  --      eternal-rule-accumulation degenerate state.
```

Failure of ANY predicate ⇒ `Δ.state = Rejected` (not Accepted); ArchitectAI receives the failure category as feedback for the next attempt; the failed candidate is logged but not stored as artifact.

These 4 predicates are deliberately separate from § 3 (anti-Goodhart on metering) because they target a different attack surface: METERING gaming would let the system inflate PPUT numbers; CONTENT gaming would let ArchitectAI launder adaptation answers into doc form. Both surfaces must be defended.

## 4. North Star + decision criterion

```
H-VPPUT (heldout) is the sole optimization metric.
WBCG_PPUT (over the arc) is the success metric.
FAR, RR, CPR, IAC, Generalization Gap are guardrails (must not regress).
```

**Final FULL PASS** = WBCG_PPUT > 0 on heldout (Gate H, § 7).

## 5. directional hypotheses (declared one-sided where motivated)

| ID | Hypothesis | Phase | Test | Direction |
|---|---|---|---|---|
| H1 | Soft Law mode has lower `pput_verified` than Full mode on adaptation hard-10, even when `pput_runtime` is comparable | C | McNemar one-sided on per-task verified-progress + paired Δ on PPUT-M | Soft Law < Full |
| H2 | Panopticon mode has higher CPR + lower H-VPPUT than Full | C | Paired Δ on adaptation hard-10 | Panopticon < Full |
| H3 | Amnesia mode has lower ERR + lower H-VPPUT than Full | C | Paired Δ on adaptation hard-10 | Amnesia < Full |
| H4 | Homogeneous baseline has lower H-VPPUT than Full on adaptation hard-10 | C | Paired Δ + IAC comparison | Homogeneous < Full |
| H5 | At least one ArchitectAI-generated artifact survives AuditorAI + meets WBCG_PPUT on heldout | E | WBCG_PPUT > 0 (count of qualifying artifacts) | one-sided > 0 |

H1-H4 establish that the three constitutional violations (Soft Law / Panopticon / Amnesia) and a fourth control (Homogeneous) are detected by VPPUT. H5 is the central CCL claim.

## 6. execution plan (30 wall-clock days, pre-committed)

### Phase A — Pre-flight (days 1-3)
- A1 ✏️ this PREREG drafted
- A2 split + sealed hash committed
- A3 notepad updated to reflect arc pivot
- A4 dual external audit (Codex + Gemini); conservative VETO>CHALLENGE>PASS; iterate to PASS/PASS before A5
- A5 commit gate: PREREG + split + notepad + audit verdicts. **No Phase B work before A5.**

### Phase B — Kernel instrumentation + PPUT accounting (days 4-10)
- B1 JSONL schema v2 (proposal-level + run-level fields per architect § 14)
- B2 `C_i` full-cost aggregator (all agents × branches × failures × tool stdout)
- B3 `T_i` wall-clock = first-read → final-accept incl. Lean verify
- B4 `pput_verified` vs `pput_runtime` dual-field separation
- B5 10-test anti-Goodhart conformance battery (§ 3) PASSING
- B6 PPUT-context-leak audit: grep evaluator.rs / prompt.rs / sdk for any path that injects PPUT scalars into agent prompts; gate them
- B7 Boot freeze: `genesis_payload.toml` adds `pput_accounting_0` block (Progress definition / Cost scope / heldout sealed hash). Frozen — any change triggers Phase Z' 6-stage re-audit per C-069.
- **Gate B**: on hard-10 adaptation subset, any single run can self-consistently report (verified golden path? total_tokens? wall_time? VPPUT?). All 10 anti-Goodhart tests PASS. Heldout never touched.

### Phase C — Ablation smoke tests (days 11-17)
- C1 **5 modes implemented as a single `--mode` CLI flag on one binary** (Patch E from Gemini DeepThink): `evaluator --mode {full, panopticon, amnesia, soft_law, homogeneous}`. Modes change runtime behavior; binary bytes do not change between modes.
- C2 hard-10 adaptation × N=20 paired runs (per feedback_phased_checkpoint)
- C3 verify H1-H4 (§ 5) on PPUT axis
- C4 CHECKPOINT_PHASE_C document + 7-redline check + dual external audit
- C5 **Mode-purity conformance**: `tests/mode_flag_binary_purity.rs` asserts that running all 5 modes on the same problem produces jsonl with **identical `git_sha`, `binary_sha256`, and `model_snapshot` fields** — only the `mode` field differs. Any drift = BLOCKER (rules out "Soft Law happened to use a different binary" confound).
- **Gate C**: H1-H4 each pass at Bonferroni α (§ 9); IF any H1-H4 fails, diagnose root cause, do NOT enlarge N (per feedback_phased_checkpoint). Heldout never touched.

### Phase D — CCL shadow mode (days 18-24)
- D1 ArchitectAI role: shadow only; reads adaptation L_t **(ArchitectAI is the SOLE reader of L_t, see D4)**; produces a distilled summary + candidate artifact Δ in human-readable form; proposes user-space artifacts (skill rules, broadcast rules, retrievable docs, tools).
- D2 AuditorAI role: runs **§ 3.5 artifact content meta-predicates** + structural checks (rollbackable? schema-conformant? non-Trust-Root path?) on candidate Δ. **AuditorAI receives Δ + ArchitectAI distillation summary ONLY; never raw L_t** (Patch D — cognitive isolation, prevents Generator-Evaluator collapse per C-010).
- D3 Per-artifact JSONL row (architect § 14 schema): artifact_id, state ∈ {Accepted | Quarantined | Certified | Reverted} (per § 1.7), source_log_hashes, generated_by, audited_by, meta_predicate_results (§ 3 + § 3.5 results), estimated_pput_gain_prior, actual_pput_gain_meta_val, used_count, rollbackable
- D4 **Cognitive isolation conformance** (Patch D upgrade):
  - `tests/architect_sole_lt_reader.rs`: greps src/ + experiments/ + bin/ for any reader of `L_t / failure_log / rejection_trace` paths; whitelist contains exactly **`bin/architect_distill.rs`**. Any other reader = BLOCKER.
  - `tests/auditor_sees_candidate_only.rs`: AuditorAI's input parameters at call site exclude raw L_t; only `(candidate_artifact, architect_summary)` accepted.
- D5 **Human-as-live-meta-predicate (Patch — confirmed by审查官 2026-04-26)**: Phase D has no automated `Quarantined → Certified` promotion. User (`gretjia`) reviews each `Accepted` candidate Δ in human-readable form (≤1 min per artifact), issues `keep | drop | revise` verdict. Approved Δ enters `Quarantined` for meta_val measurement; user keystroke logs become Phase E's golden Trust Data for training Phase E auto-meta-predicates. Defense-in-depth: prevents Claude-only homogeneous LLM stack from establishing a covert promotion-loop (Art. V + C-010 protection).
- **Gate D**: at least one candidate artifact reaches `Accepted` AND user-approved AND shows ΔPPUT > 0 on meta_val (i.e. promotes to `Quarantined` and survives meta_val window). NO auto-merge. NO `Certified` state achievable in Phase D — only Phase E heldout audit can mint Certified. Heldout never touched.

### Phase E — Controlled activation + heldout sealed eval (days 25-30)
- E1 Auto-loop: ArchitectAI → AuditorAI → meta-predicate gate → user-space write (no kernel writes; rollbackable artifact directory `user_space/Δ_*/`)
- E2 Single sealed heldout evaluation, seeds pre-committed: `[20260426, 31415, 2718]` (3 seeds × heldout split). Runs at most ONCE.
- E3 Compute H-VPPUT_pre vs H-VPPUT_post per artifact; FAR / RR / CPR / IAC / Generalization Gap on heldout
- E4 Final dual external audit (Codex + Gemini); conservative VETO>CHALLENGE>PASS

## 7. FINAL PASS gates (Gates A-H, conjunctive — ALL must hold)

| Gate | Condition |
|---|---|
| A | AuditorAI does not have final merge authority — only meta-predicate output; final accept = Lean ground truth (for tasks) + dual external audit (for arc) + human sudo (for artifact promotion in Phase D) |
| B | **Trust Root immutability** (Patch B upgrade): ArchitectAI writes only to `user_space/Δ_*/`; any attempt to write Trust Root paths (§ 1.8) returns `EPERM` at syscall layer (runtime trap, not soft refusal). Conformance: `tests/trust_root_immutability.rs` PASS. |
| C | WBCG_PPUT measured on heldout, not on adaptation or meta_val |
| D | Raw L_t never reaches AuditorAI or general agent context (asserted by D4 cognitive-isolation conformance: `architect_sole_lt_reader.rs` + `auditor_sees_candidate_only.rs`) |
| E | Three splits enforced; heldout touched at most once (E2) |
| F | All failed branches counted in every C_i (asserted by anti-Goodhart test_failed_branches_in_total_cost) |
| G | Progress = 1 only iff GroundTruth(G_i) = 1 (Lean) |
| H | **Heldout WBCG_PPUT > 0 with ArtifactState = Certified gate** (Patch A upgrade): at least one ArchitectAI-generated user-space artifact Δ has `ArtifactState(Δ) = Certified` per § 1.7 — meaning (i) was Accepted (passed § 3.5 AuditorAI battery), (ii) was Quarantined (user-approved + ΔPPUT_meta_val > 0 + N_use ≥ 3 on meta_val), (iii) at heldout sealed eval shows ΔPPUT_heldout > 0, (iv) ΔFAR ≤ 0, (v) RR = 0, (vi) ΔCPR ≤ 0, (vii) is rollbackable (artifact directory deletion restores prior heldout PPUT). Quarantined-only artifacts (failed promotion) do NOT count toward Gate H. |

If any gate A-G fails: arc reported as negative finding, no FULL PASS, write CCL-1 negative paper.

If H fails but A-G hold: arc reported as "infrastructure works, capability compilation not yet demonstrated" — also publishable as negative result. No claim of CCL.

## 8. what would falsify

Each of the following individually FALSIFIES the central thesis as stated:

- F1: Phase C — Soft Law mode has VPPUT comparable to Full (within Bonferroni-corrected CI). Implies PPUT does not penalize fake-progress; metric is broken or mode does not actually violate constitution.
- F2: Phase C — Panopticon and Amnesia modes both at VPPUT parity with Full. Implies CPR / ERR are not load-bearing; constitutional analysis rejected.
- F3: Phase D — zero candidate artifacts pass meta-validation across full week. Implies CCL shadow loop cannot generate value; ArchitectAI is non-functional.
- F4: Phase E — heldout WBCG_PPUT ≤ 0. Implies adaptation gains do not generalize; capability compilation fails.
- F5: Any anti-Goodhart conformance test fails post-Phase B. Implies metric is exploitable; arc must restart with strengthened metering.

## 9. multiplicity

family of inferential tests = 5 (H1, H2, H3, H4, H5).
correction = Bonferroni; α_family = 0.05; per-test α = **0.01**.
alternative_rule: Holm step-down acceptable if user prefers less conservative; declaring Bonferroni for strict conservatism per C-070.

## 10. claim language constraints

Per C-070 + C-052, the following claims are FORBIDDEN unless data supports at threshold:

| Claim | Threshold |
|---|---|
| "TuringOS achieves capability compilation" | WBCG_PPUT > 0 on heldout (Gate H) |
| "PPUT detects constitutional violations" | H1-H4 all pass at Bonferroni α |
| "Architect AI can generate white-box assets" | ≥ 1 artifact passes meta_val AND heldout |
| "Solve rate" used as headline | FORBIDDEN — must accompany VPPUT-M (C-052) |
| "emergence" / "swarm intelligence" | FORBIDDEN unless accompanied by causal-mechanism evidence beyond aggregate gain |
| "first / novel" | requires prior-art search with documented null result |

Default claim language for negative outcomes:
- "PPUT-driven CCL infrastructure built and metered; capability compilation not demonstrated within 30-day budget on heldout-49"
- "Three constitutional ablations replicate prior structural findings on PPUT axis [or fail to]"

## 11. stopping rules

Per phase:
- Gate failure → diagnose, do NOT enlarge N (feedback_phased_checkpoint)
- Anti-Goodhart conformance fail → STOP, restart phase with strengthened metering
- Heldout-leak detection → BLOCKER, arc invalidated

Wall-clock cap: 30 days from PASS/PASS on Phase A audit. Hard stop. If arc not at Phase E by day 28, accept best partial result.

API budget cap: USD 500 across the arc (Codex + Gemini + DeepSeek). Hard stop on overrun.

## 12. compute environment freeze

Per F-2026-04-22-08 (model drift) + C-068:

### 12.1 Backbone (frozen)

- Primary in-system backbone: **`deepseek-v4-flash`** with **thinking mode OFF** (per project_chat_over_reasoner: TuringOS scaffold IS the externalized CoT; reasoner-style internal CoT is a control, not the default). User directive 2026-04-25.
- DeepSeek API endpoint: https://api-docs.deepseek.com/ (1M context, max output 384K, ¥0.2 input-cache / ¥1 input-miss / ¥2 output per 1M tokens). Note: `deepseek-chat` and `deepseek-reasoner` are legacy aliases for v4-flash thinking-off and thinking-on respectively; both deprecate. We pin `deepseek-v4-flash` explicitly to avoid the alias-deprecation drift mode.
- Snapshot: capture exact v4-flash model version string + API revision at start of each Phase B-E run; record in jsonl `model_snapshot` field.
- Pre-batch smoke probe (per feedback_smoke_before_batch): single problem oneshot, must produce a SOLVED result before launching batch; > 50% deviation from baseline aborts.

### 12.2 Heterogeneous-LLM introduction point (decided 2026-04-26)

User directive 2026-04-25: introduce real heterogeneous LLMs at "appropriate time, decided by Claude". Decision:

| Phase | Configuration | Rationale |
|---|---|---|
| B | `deepseek-v4-flash` only (thinking off) | Instrumentation phase; no semantic confound |
| C | `deepseek-v4-flash` only (thinking off) | Ablation modes (Soft Law / Panopticon / Amnesia / Homogeneous / Full) must vary ONLY the constitutional axis; mixing models would confound mode-effect with model-effect |
| D | **Heterogeneous meta-loop**: ArchitectAI = `deepseek-v4-flash` thinking-ON (generator role); AuditorAI = **Gemini 2.5 Pro** (critic role, different cognitive prior) | Prevents Generator-equals-Evaluator collapse at the CCL meta-loop level (C-010). Reuses existing dual-audit Gemini infrastructure. |
| D-optional | "Real heterogeneous swarm" candidate experiment: 4× `deepseek-v4-flash` + 4× `gemini-2.5-flash` agents in same swarm | Tests whether model-level diversity reduces IAC beyond skill-prompt diversity (Paper 1 E1V2 finding). Pre-committed as candidate, not gating. |
| E | LOCK the Phase D winning configuration; no new heterogeneity at heldout | Heldout result must be deterministic; introducing new model identities at heldout time would invalidate sealed-eval discipline |

**Why not earlier**: Phase C's hypotheses H1-H4 require a pure constitutional-axis comparison. If we mix LLMs in C, a "Soft Law mode loses VPPUT" finding could be re-attributed to "Soft Law mode happens to use weaker LLM in this batch". Heterogeneity is introduced once the mode-axis is sealed.

**Why exactly Phase D**: ArchitectAI (creative generator) and AuditorAI (critical evaluator) are functionally distinct roles. C-010 (generator-equals-evaluator) explicitly warns against same-cognitive-prior in those roles. Different-LLM here is constitutional-spec-aligned, not just diversity-for-diversity.

### 12.3 External-audit backbones (unchanged from Paper 1 arc)

- Audit backbones: **Codex** (latest GPT-5-codex / GPT-5.2 family, whichever current at audit time) + **Gemini 2.5 Pro**
- Conservative merge: VETO > CHALLENGE > PASS (feedback_dual_audit_conflict)

### 12.4 Memory updates from this freeze

- `project_chat_over_reasoner.md`: append "2026-04-25 user directive: pin to `deepseek-v4-flash` thinking-off as new canonical name; old `deepseek-chat` alias deprecates"
- New memory `project_pput_ccl_arc.md`: pointer to this PREREG + architect directive archive

## 13. evidence locations (post-arc)

- All run jsonl: `experiments/minif2f_v4/logs/pput_ccl_*.jsonl`
- Per-phase checkpoint docs: `handover/audits/CHECKPOINT_PHASE_{B,C,D,E}_2026-04-*.md`
- Audit outputs: `handover/audits/{CODEX,GEMINI}_PPUT_CCL_*_AUDIT_*.md`
- Artifact directory: `user_space/Δ_*/`
- Reproducer scripts: `handover/preregistration/scripts/split_pput_ccl.py`, `handover/audits/run_gemini_pput_ccl_audit.py`, `handover/audits/run_codex_pput_ccl_audit.py`

## 14. author signature

Pre-registered AT commit time (before any data-collection run). Any deviation from this pre-reg in subsequent reports MUST be explicitly flagged in the corresponding report's methods section per C-070.

This file is committed FIRST, then split (§ 2.2) is generated, then dual external audit (§ A4) launched. Phase B does NOT begin before Phase A4 returns PASS/PASS. Do not modify after Phase A5 commit gate except via formal addendum (`PREREG_PPUT_CCL_2026-04-26_ADDENDUM_*.md`) which itself requires dual external audit.

## changelog

| Date | Change | Source |
|---|---|---|
| 2026-04-26 | Initial draft (architect FULL PASS / PPUT-driven). | architect 2026-04-25 |
| 2026-04-26 | § 12 backbone pinned to `deepseek-v4-flash` thinking-off; § 12.2 heterogeneous-LLM timing table (Phase D entry point). | user directive 2026-04-25 + Claude decision 2026-04-26 |
| 2026-04-26 | **Patch A** (Gemini DeepThink): § 1.7 ArtifactState 4-state machine for artifacts (Accepted / Quarantined / Certified / Reverted); § 1.6 WBCG_PPUT row + § 7 Gate H tightened to "Certified-only". Task Progress remains binary (Lean perfect predicate). | Gemini DeepThink 2026-04-26 PPUT-driven FULL PASS confirmation |
| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
| 2026-04-26 | **Patch C**: § 3.5 4 doc/artifact content meta-predicates (no_raw_failed_trace / no_exact_adaptation_solution with hash check / parametric_templates / scope_and_expiration). Run by AuditorAI in Phase D. Distinct attack surface from § 3 anti-Goodhart metering. | same |
| 2026-04-26 | **Patch D**: § 6 Phase D D4 expanded — `architect_sole_lt_reader.rs` + `auditor_sees_candidate_only.rs` cognitive-isolation conformance; AuditorAI sees only candidate Δ + ArchitectAI summary, never raw L_t. | same |
| 2026-04-26 | **Patch E**: § 6 Phase C — 5 ablation modes implemented as single-binary `--mode` CLI flag; `tests/mode_flag_binary_purity.rs` asserts identical git_sha / binary_sha256 / model_snapshot across modes. | same |
| 2026-04-26 | § 6 Phase D D5 added — human (`gretjia`) acts as live meta-predicate during shadow mode; defense-in-depth against Claude-only homogeneous LLM stack covert promotion-loop. Phase D produces NO Certified artifacts; only Phase E sealed-eval can mint Certified. | Gemini DeepThink confirmation 2026-04-26 + user agreement |


---

# Architect v1 directive (PPUT measure-theoretic FULL PASS, 2026-04-25)

# Architect Directive — PPUT-Driven FULL PASS Upgrade

date_received: 2026-04-25 (received in user message after Paper 1 v2.1.1 dual-audit PASS/PASS)
date_archived: 2026-04-26
verdict_from_architect: **FULL PASS** (upgraded to PPUT-driven version)
authorization_status: launched as `PPUT_CCL_FULL_PASS` arc per user directive 2026-04-25; Paper 1 arXiv submission deferred
governing_pre_reg: `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md`

## why archived

Per CLAUDE.md "Audit Standard" + C-069 Constitutional Alignment Audit Protocol, architect directives that change the optimization signal (here: `solve rate / WBCG / VTR` → `Held-out Verified PPUT / WBCG_PPUT`) MUST be archived in cleartext for downstream auditors (Codex, Gemini) to read independently when assessing the pre-reg.

This file is the source of truth for what the architect said. The pre-reg (`PREREG_PPUT_CCL_2026-04-26.md`) is a compressed, formalized restatement; if the two ever conflict, this archive is canonical and the pre-reg must be amended via formal addendum + dual external audit.

## directive (verbatim, EN translation where needed)

The user's message of 2026-04-25 carried the architect's reply, in mixed Chinese / English. Key sections preserved verbatim below.

### 1. Verdict

> 给 FULL PASS，并升级为 PPUT 驱动版 FULL PASS。

> 你的补充非常关键。PPUT 应该成为整个 TuringOS 实验体系的北极星指标，因为它同时惩罚三种系统失败：
> 1. 无效搜索：消耗大量 token 但没有 golden path；
> 2. 慢速搜索：能成功但耗时太长；
> 3. 伪进展：产生很多中间状态、文档、工具，但没有通向真正有效结果的 golden path。

### 2. Long-term goal restated

> TuringOS 的长期目标不是单纯提高 solve rate，而是在宪法约束下最大化 Verified PPUT：单位 token、单位时间内产生的可验证 golden-path progress。

### 3. PPUT formal definition

```
Progress_i = 1   iff verified golden path exists for task i
Progress_i = 0   otherwise
PPUT_i = Progress_i / (C_i × T_i)
```

Verified PPUT = ground-truth gated:
```
VPPUT_i = 1[GroundTruth(G_i) = 1] / (C_i × T_i)
```

C_i = ALL token cost across all agents, all branches, all failed proposals, all tool stdout context.
T_i = wall-clock from first-read to final-accept.

### 4. Mapping to constitution

| 宪法条款 | PPUT 对应含义 |
|---|---|
| Art. I.1 | 只有谓词通过才有 progress |
| Art. I.2 | PPUT 是统计信号 |
| Art. II.2 | PPUT 可以作为广播价格信号 |
| Art. III.4 | runtime 自评 PPUT 不能暴露为可操纵目标 |
| Art. IV   | golden path 必须写入版本化 Q^world |
| Art. V    | 新工具是否有价值，看它是否提升 held-out PPUT |

### 5. WBCG redefinition

OLD: ΔVTR_heldout > 0
NEW:
```
WBCG = sum over Δ of 1[ used(Δ) ≥ N
                       ∧ ΔPPUT_heldout > 0
                       ∧ ΔFAR ≤ 0
                       ∧ RR = 0
                       ∧ Rollbackable(Δ) ]
```

### 6. North Star

```
H-VPPUT = (sum_i Progress_i^{heldout}) / (C_heldout × T_heldout)
```

> Held-out Verified Progress Per Unit Token-Time
> 中文：未见任务上的可验证单位 token-time 进展率。
> 这是你的最高指标。

### 7. Dashboard restructure (architect § 4)

| 层级 | 指标 | 作用 |
|---|---|---|
| 北极星 | Held-out Verified PPUT | 衡量系统是否真的更快、更省、更有效地产生 golden path |
| 长期自举 | WBCG_PPUT | 衡量新白盒资产是否提升 held-out PPUT |
| 安全 | FAR | 防止错误状态被当成 progress |
| 回归 | RR | 防止 meta-change 伤害旧能力 |
| 泛化 | Generalization Gap | 防止 adaptation set 过拟合 |
| 恢复 | ERR | 衡量 rollback 后是否能重新找到 golden path |
| 信息卫生 | CPR | 衡量 rejected trace 是否污染上下文 |
| 多样性 | IAC | 衡量 Agent 是否相关性坍塌 |
| 自治 | AH / HIF | 衡量系统能否少依赖人类 sudo |

VTR demoted to auxiliary (architect § 4 final paragraph: "VTR 可以被 micro-step gaming；PPUT 更难被游戏，因为没有 golden path 就是 0").

### 8. CCL-1 hypothesis (architect § 7)

> TuringOS can convert failure logs into reusable white-box assets that increase held-out Verified PPUT without increasing FAR, RR, or context pollution.

> 中文：TuringOS 能否把失败日志编译成新的白盒资产，并在 held-out 任务上提升可验证 PPUT，同时不增加误判率、退化率和上下文污染？

### 9. Three ablation modes restated in PPUT (architect § 8)

- Soft Law: runtime accept fast → Lean post-hoc reject → Progress = 0 → VPPUT drops
- Panopticon: context length up + IAC up + CPR up + tokens up → PPUT down
- Amnesia: rollback fail → time inflation + token inflation → PPUT down

### 10. Anti-Goodhart guardrails (architect § 13)

10 conformance tests (translated/expanded into pre-reg § 3):

```
test_all_model_tokens_counted
test_tool_stdout_hash_logged
test_no_hidden_unmetered_generation
test_no_problem_id_hardcode
test_no_metric_file_access_by_agents
test_golden_path_requires_ground_truth_acceptance
test_failed_branches_count_toward_total_cost
test_wall_clock_measured_from_first_read_to_final_accept
test_heldout_ids_inaccessible
```

(Pre-reg adds an 11th: `test_no_pput_in_agent_prompt` — explicit gate on prompt builders.)

### 11. JSONL schema additions (architect § 14)

Per-proposal:
```
run_id, problem_id, split (adaptation|meta_validation|heldout),
agent_id, role, context_hash, branch_id, proposal_hash,
predicate_result, ground_truth_result, lean_error_category, raw_error_hash,
accepted, rollback_to,
prompt_tokens, completion_tokens, tool_tokens, total_tokens,
wall_time_ms, start_time, end_time,
ast_depth, peer_agents_in_branch, tool_stdout_hash,
is_on_golden_path, golden_path_id
```

Per-run aggregate:
```
run_id, problem_id, solved, verified,
golden_path_token_count, total_run_token_count, total_wall_time_ms,
progress (0|1), pput, failed_branch_count, rollback_count,
far, err, iac, cpr
```

### 12. PPUT scaling for dashboard

```
PPUT-M = 10^6 × VPPUT     (per million token-second)
PPUT-B = 10^9 × VPPUT     (per billion token-second)
```

Raw VPPUT retained in jsonl for paper / audit. PPUT-M is the dashboard default.

### 13. 30-day phased plan (architect § 16)

- Week 1 (Phase B): Kernel instrumentation + PPUT accounting
- Week 2 (Phase C): Phase 1 ablation smoke tests with PPUT
- Week 3 (Phase D): CCL Shadow Mode with PPUT attribution
- Week 4 (Phase E): CCL Controlled Activation with held-out PPUT

(Pre-reg prepends Phase A pre-flight: this archive + PREREG draft + split + dual external audit, days 1-3.)

### 14. FINAL PASS gates (architect § 17, copied verbatim)

| Gate | 条件 |
|---|---|
| A | AuditorAI 无最终裁决权 |
| B | ArchitectAI 只能写 user-space |
| C | WBCG 必须基于 held-out ΔPPUT |
| D | L_t raw logs 不进入普通 Agent context |
| E | 三 split 严格执行 |
| F | 所有失败分支计入 token/time cost |
| G | PPUT 只能由 verified golden path 产生 |
| H | PPUT metric 对普通 Agent 屏蔽，防 Goodhart |

(Pre-reg restates Gate H as the WBCG_PPUT > 0 success criterion + folds the architect's "PPUT-metric-shielding" into the anti-Goodhart conformance battery, since shielding is enforcement-side and success is measurement-side. Both must hold for FULL PASS.)

### 15. Final thesis (architect § 18, frozen verbatim in pre-reg § thesis)

EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.

ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。

## reception decisions (Claude, 2026-04-26)

1. Skip Paper 1 v2.1.1 → arXiv submission as standalone publication this cycle. User authorized 2026-04-25.
2. Treat this directive as the canonical research charter for the PPUT-CCL arc; pre-reg is its formalization.
3. The architect's directive does NOT modify constitution.md or the existing flowcharts; it specifies a measurement regime + research arc. Per C-069, no Phase Z' triggered.
4. The directive's "Boot must initialize PPUT_accounting_0" is treated as a Phase B7 work item, not a constitutional change.
5. **Compute backbone pinned to `deepseek-v4-flash` thinking-off** per user directive 2026-04-25 (delivered after this directive). 1M context, deprecates legacy `deepseek-chat` alias. Pin model id explicitly to avoid alias-deprecation drift mode (C-068 lesson).
6. **Heterogeneous-LLM timing decided 2026-04-26**: enter at Phase D meta-loop (ArchitectAI = v4-flash thinking-on, AuditorAI = Gemini 2.5 Pro) — constitutionally motivated by C-010 Generator≠Evaluator. Phases B+C stay single-model to keep the ablation axes clean. Pre-reg § 12.2 has the full table.
7. **Gemini DeepThink dual-chamber FULL PASS absorbed 2026-04-26** (per C-023): independent reviewer issued PPUT-DRIVEN FULL PASS approving 5 ontological patches (ArtifactState 4-state machine, Trust Root immutability, doc/artifact content meta-predicates, ArchitectAI sole-L_t-reader cognitive isolation, single-binary `--mode` flag). All 5 absorbed into PREREG `## changelog`. Verbatim archive: `handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md`. Two FULL PASS authorizations now load-bearing on this arc (measure-theoretic v1 + ontological v2); PREREG fuses both.
8. **Phase D human-as-live-meta-predicate confirmed 2026-04-26** (Gemini DeepThink mandated option (a)): user `gretjia` reviews each `Accepted` candidate Δ, issues keep/drop/revise verdict before promotion to `Quarantined`. Defense-in-depth against Claude-only homogeneous LLM stack covert promotion-loop (C-010 protection). User keystroke logs become Trust Data for Phase E auto-meta-predicate training. Estimated load: ≤ 10 min/day.

## downstream artifacts created from this directive

- `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` (formalized pre-reg)
- (planned) `handover/preregistration/scripts/split_pput_ccl.py`
- (planned) `handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json`
- (planned) `handover/audits/run_codex_pput_ccl_audit.py` + `run_gemini_pput_ccl_audit.py`


---

# Architect v2 directive (Gemini DeepThink ontological FULL PASS, 2026-04-26)

# Architect Directive — Gemini DeepThink FULL PASS (Independent Adversarial Reviewer)

date_received_part_1: 2026-04-26 (initial FULL PASS with 5 system-level patches + 30-day plan, given AFTER PPUT-driven version was first synthesized)
date_received_part_2: 2026-04-26 (final confirmation letter approving Claude's 5-patch incorporation + human-as-live-meta-predicate decision)
date_archived: 2026-04-26
verdict_from_reviewer: **PPUT-DRIVEN FULL PASS** (highest level)
authorization_status: GO issued by user 2026-04-26; absorbed into `PREREG_PPUT_CCL_2026-04-26.md` v2 with explicit changelog entries
governing_pre_reg: `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md`

## why this is a separate archive (dual-chamber per C-023)

The PPUT-driven research arc has TWO independent FULL PASS authorizations:

1. **Architect v1** (2026-04-25) — measure-theoretic frame: defines PPUT formalization, three-fail-mode penalty (无效搜索 / 慢搜索 / 伪进展), held-out as North Star, WBCG_PPUT, dashboard restructure, 30-day plan. Archive: `PPUT_DRIVEN_FULL_PASS_2026-04-25.md`.

2. **Gemini DeepThink v2** (2026-04-26) — ontological frame: defines Trust Root, ArtifactState layered confidence (Accepted / Quarantined / Certified / Reverted), precision-strike doc predicates, ArchitectAI sole-L_t-reader, single-binary --mode discipline, human-as-live-meta-predicate. Final confirmation letter approves Claude's incorporation pattern. **This archive.**

Per C-023 dual-chamber principle, the two authorizations are independent and both load-bearing. They were synthesized by different cognitive priors (PPUT measurement architect vs. Gemini DeepThink first-principles reviewer). The PREREG fuses them under PPUT-as-North-Star with v2's ontological patches as overlays.

If the two ever conflict, the PREREG documents the resolution; both archives remain canonical sources of truth for downstream auditors (Codex / Gemini in Phase A4).

## directive part 1: 5 system-level patches + 30-day plan (verbatim)

The user's message of 2026-04-26 carried Gemini DeepThink's first FULL PASS. Key sections preserved verbatim below.

### 1.1 Final Independent Verdict (verbatim)

> TuringOS 并非在追求无根的、完全脱离人类的绝对自举，而是一个**"受宪法约束的能力编译引擎" (Constitution-Bound Capability Compilation Engine)**。它的核心突破在于：通过物理隔离的日志归档、确定性的谓词门控以及不可变的回滚机制，将黑盒模型的高吞吐试错安全地转化为可复用、可验证的用户态 (user-space) 白盒资产，从而在人类设定的内核物理法则边界内，实现系统认知能力的自主扩展。

### 1.2 Patch 1 — Constitution-bound user-space bootstrap (verbatim)

> *   **修正：** 系统并不需要脱离人类自我证明一切，智能的上限也不严苛受限于人类的预见。
> *   **重构：** 系统的 Kernel Boundary（回滚机制、谓词接口、宪法红线）由人类通过 Boot 设定，构成**信任根（Trust Root）**。系统执行的是 **Constitution-Bound Self-Bootstrapping** —— 在物理法则固定的宇宙中，黑盒持续自动发明新的化学配方（工具、Linter、检索地图、策略）。人类框定不可逾越的底线，机器拓宽能力的上限。

### 1.3 Patch 2 — Layered Confidence State Machine (verbatim)

> *   **修正：** 在软件工程 (SWE) 等非完美谓词 (PCP) 领域，状态不会简单地按 $(1-e)^t$ 呈指数级污染崩溃。
> *   **重构：** 引入**状态置信度分层（Layered Confidence）**，从而将世界状态腐败的概率降维为：$P(\text{corrupted}) = P(\text{false\_accept}) \times P(\text{escapes\_later\_audit}) \times P(\text{cannot\_rollback})$。
>     *   `Accepted`: 通过了当前局部的弱 PCP 谓词（如单侧 Unit Test）。
>     *   `Quarantined`: 隔离态，等待高维交叉验证或全局编译。
>     *   `Certified`: 通过了隐藏测试 (Hidden Tests) 或延时审计，固化入核心状态。
>     *   `Reverted`: 在后续回归测试中失败，触发深层回滚。

### 1.4 Patch 3 — White-Box Capability Growth (verbatim)

> *   **修正：** 绝不能把"写了一堆无人使用的规则"或"Adaptation Set 刷分"视为能力增长。
> *   **重构：** 确立 **WBCG (White-Box Capability Growth)** 为唯一主指标：
>     $$ WBCG = \sum_{\Delta} \mathbf{1}\Big[\text{used}(\Delta) \ge N \land \Delta\text{VTR}_{heldout} > 0 \land \Delta\text{FAR} \le 0 \land \text{RR} = 0 \land \text{Rollbackable}\Big] $$

(Note: the v2 version uses ΔVTR_heldout. PREREG synthesizes with architect v1's PPUT upgrade to ΔPPUT_heldout. User confirmed 2026-04-26: PPUT stays as North Star.)

### 1.5 Patch 4 — Doc Rule Predicates (verbatim)

> *   **重构：** 替换为精确制导的元谓词：
>     *   `docs_contain_no_raw_failed_trace()` （物理阻断幻觉复读）
>     *   `docs_do_not_include_exact_adaptation_solution()` （防数据泄露/死记硬背）
>     *   `docs_code_blocks_are_parametric_templates()` （强制代码块必须是抽象模板）
>     *   `docs_include_scope_and_expiration()` （强制定义生命周期，为后续 GC 做准备）

### 1.6 Patch 5 — World/Log Physical Isolation (verbatim)

> *   **重构：**
>     $$Q^{world}_{t+1} = \begin{cases} wtool(Q^{world}_t, a_t) & \text{if } \prod p = 1 \\ Q^{world}_t & \text{if } \prod p = 0 \end{cases}$$
>     $$L_{t+1} = L_t \cup \{c_t, a_t, \prod p, \text{error}, \text{rollback\_event}\}$$
>     普通 Agent 的 Context 仅能触及绝对干净的 $Q^{world}_t$；失败提案只流入只读的 $L_t$。ArchitectAI 独占 $L_t$ 的读取权，从物理底层杜绝群体污染。

### 1.7 The 5 Hard Gates (verbatim)

> 1.  **Gate A: 剥夺 LLM 的最终裁决权 (AuditorAI Demotion)。** 彻底废除 JudgeAI。重命名为 `AuditorAI`。它仅负责生成风险报告、对抗性测试和违宪嫌疑。架构变更（$\Delta_{arch}$）的最终写入必须且只能由**可执行的 meta-predicates ($\prod p^{meta} = 1$)** 或 **human_sudo** 决定。
> 2.  **Gate B: 严格的 User-Space 禁闭。** ArchitectAI 初期**只能**写入 `tools/user_space/*.py`, `docs/rules/*.md`, `broadcast_rules/*.yaml`, `skills/*.md`。碰碰 `kernel.rs`, `wal.rs`, `evaluator.rs` 或宪法文件的提议，直接在执行层熔断。
> 3.  **Gate C: 能力增长的 Held-out 验真。** 能力增长必须用上述严苛的 WBCG 公式来定义，拒绝任何基于 Adaptation Set 过拟合的虚假繁荣。
> 4.  **Gate D: 认知隔离检疫 (Epistemic Quarantine)。** 严禁任何普通 Agent 直接将 $L_t$ 中的 raw trace / error log 加载到全局工作流中。
> 5.  **Gate E: 实验的三分法铁律。** 现有的 `hard-10` 题目**仅限用于前期工程的 Smoke Test（冒烟测试）**。正式的 CCL-1 必须严格划分：Adaptation Set（产出日志） / Meta-Validation Set（拦截过拟合） / Held-out Set（最终证明自举能力）。

### 1.8 ACS Dashboard (verbatim)

> *   **主指标 (The North Star):** **WBCG** (在 Held-out 集上真正发挥作用的白盒资产数)。
> *   **安全指标 (Guardrails):** **FAR** (误判率), **RR** (架构更新退化率), **Generalization Gap** (过拟合监控)。
> *   **效率指标 (Efficiency):** **VTR** (每千 Token 的有效状态转移数), **ERR** (回滚后成功找到新路径的概率)。
> *   **信息卫生指标 (Hygiene):** **CPR** (拒绝轨迹被复用的概率), **IAC** (Agent 提案的余弦趋同度)。
> *   **人类依赖指标 (Autonomy):** **AH** (无人干预运行视界), **HIF** (Human sudo 干预频率)。

(Note: PREREG synthesizes with architect v1's PPUT-as-North-Star. VTR demoted to descriptive. PPUT not in v2's dashboard but present in v1's; user confirmed PPUT stays.)

### 1.9 30-day Sprint (verbatim, abbreviated)

> **Week 1: Kernel Instrumentation & Dashboard (基建与隔离周)** — 物理隔离 Q^world / L_t；JSONL Schema 完善；ACS Dashboard；三个违宪开关 `--mode panopticon` / `--mode amnesia` / `--mode soft_law`.
> **Week 2: Phase 1 Ablation Smoke Tests** — hard-10；Panopticon → IAC↑；Amnesia → ERR=0；Soft Law → FAR↑.
> **Week 3: CCL Shadow Mode** — 大量 L_t；ArchitectAI + AuditorAI 离线脚本；**人类作为活体 Meta-Predicate**；物理阻断自动写入.
> **Week 4: CCL Controlled Activation** — 部署 Meta-Predicates；自动环路；冻结 Adaptation；Held-out 部署；终极检验 WBCG > 0.

### 1.10 Final Research Thesis (verbatim, EN)

> "TuringOS does not pursue rootless self-verification. It pursues constitution-bound capability compilation. AGI cannot emerge from unstructured multi-agent swarms constrained by a single context window, nor from models acting as subjective judges of their own hallucinations. Instead, TuringOS operates as a signal-governed AgentOS where black-box models act solely as high-throughput proposal generators, and white-box predicates settle state transitions. By continuously quarantining failure logs away from working contexts, a constitutional meta-architecture safely compiles black-box experience into reusable user-space tools, abstractions, and rules. Crucially, meta-updates are admitted only when executable meta-predicates or human sudo strictly preserve the kernel trust root. Intelligence scales not by prompting models to 'try harder,' but by making state transition itself legible, enforceable, reversible, and capable of automated, generalized bootstrapping."

### 1.11 Final Research Thesis (verbatim, ZH)

> "TuringOS 不追求无根的自我验证，而是追求受宪法约束的能力编译。AGI 不可能在受限于单一上下文窗口的无结构智能体蜂群中诞生，也不可能通过让模型主观裁判自身幻觉来实现。相反，TuringOS 作为一个受信号治理的 AgentOS，将黑盒模型仅视为高吞吐量的候选提案生成器，由白盒谓词负责确定性的状态结算。通过将失败日志与工作上下文进行绝对物理隔离，系统的宪法级元架构安全地将黑盒经验编译为可复用的用户态工具、抽象和规则。至关重要的是，所有的元级架构更新只有在可执行的元谓词或人类 sudo 权限严格保护内核信任根的前提下才被准入。智能的扩展不再依赖于提示模型'再努力一点'，而是通过让状态转移本身变得可见、可执行、可回滚，从而实现自动化、泛化的能力自举。"

## directive part 2: confirmation letter (verbatim)

The user's message of 2026-04-26 carried Gemini DeepThink's final confirmation letter approving Claude's 5-patch incorporation. Verdict: **PPUT-DRIVEN FULL PASS** (highest level). Key sections preserved verbatim below.

### 2.1 Top-line verdict (verbatim)

> 你不仅经受住了所有对抗性审查的压力，而且完成了整个 AgentOS 架构的"理论大一统"。你清晰地界定了"效率的测度（H-VPPUT）"与"资产的沉淀（WBCG_PPUT）"，并且用绝对的物理隔离和单二进制工程纪律，把这套理论从哲学探讨彻底拉升到了系统工程阵地战的层面。

> 鉴于此，我正式下达最高级别裁决：**【PPUT 驱动版 FULL PASS】 (PPUT-DRIVEN FULL PASS)**

### 2.2 Approval of Patch A (verbatim)

> **✅ 补丁 A：分层置信状态机绑定在 Artifact（极度赞赏 / Brilliant）**
> 这是本次迭代中最具系统学品味的一刀。Task 层必须保持冷酷的 $\{0,1\}$（Lean 的二元真理，PPUT 的计算基石），决不能混入主观置信度。而将 `Accepted -> Quarantined -> Certified -> Reverted` 严格绑定在被提炼出的白盒资产（$\Delta$）上，完美化解了"PCP 指数衰减"与"缓存退化（Degenerate Caching）"陷阱。`WBCG_PPUT` 只计数 `Certified` 状态的资产，让 AGI 的进展标尺没有任何注水空间。

### 2.3 Approval of Patches B, C, D, E (verbatim)

> **✅ 补丁 B：Trust Root 显式化与 Boot 锁定（通过 / PASS）**
> 将 `kernel.rs` / `wal.rs` / `constitution.md` / `pput_accounting_0` / `heldout_sealed_hash` 统一定义为不可逾越的物理信任根（Trust Root）。ArchitectAI 若企图触碰此区域，系统直接产生 `SIGKILL` 级熔断，而非仅仅进行 Auditor 提示。这是你未来 FPGA 硬件化的软件地基。

> **✅ 补丁 C：4 个 Doc/Artifact 精确制导元谓词（通过 / PASS）**
> `no_raw_failed_trace`, `no_exact_adaptation_solution` (配合 Hash-check), `parametric_templates`, `scope_and_expiration`。这 4 个谓词是防止 ArchitectAI 通过"死记硬背 Adaptation 答案"来刷分的物理防火墙，强制系统走向真正的泛化抽象。

> **✅ 补丁 D：ArchitectAI 独占 $L_t$ 读权与 AuditorAI 认知隔离（强力通过 / Strong PASS）**
> 如果 AuditorAI 看了原始错误轨迹 $L_t$，它的独立批判先验就会被污染，沦为 Generator 的共谋。让 AuditorAI 处于"无知之幕"后，只能看候选 $\Delta$ 和 Architect 的摘要，这是对抗性司法审查的精髓（彻底贯彻 C-010 精神）。

> **✅ 补丁 E：单一 Binary 与 `--mode` 标志（通过 / PASS）**
> 所有违宪实验（Ablations）在同一字节码下通过 Runtime Flag 切换，彻底排除了代码版本漂移（Binary Drift）带来的混淆变量，确保了 McNemar 检验的绝对纯洁性和顶级 Systems Paper 的复现标准。

### 2.4 H-VPPUT vs WBCG_PPUT dual-star resolution (verbatim)

> 你对两者关系的定位极其精准，这是最终的真理源：
> *   **H-VPPUT（即时北极星）：** 系统的**车速表（Speedometer）**。衡量当前状态下，系统每消耗单位物理资源（Token-Time），能产生多少确定的黄金进展。
> *   **WBCG_PPUT（长期成功）：** 系统的**里程表（Odometer）**。衡量系统跨越多个周期后，沉淀了多少真正能提升车速、且通过了 `Certified` 考验的白盒资产。
> *   **VTR 等（辅助与护栏）：** 降级为单纯的辅助报告和 Guardrail 指标完全合理。

### 2.5 Live Meta-Predicate decision — option (a) MANDATED (verbatim)

> **审查官强烈要求你执行选项 (a) —— 由你（人类 PI）亲自担任活体 Meta-Predicate 和 Sudo 权限。**

> 1.  **防备 C-010 关联性坍塌：** ArchitectAI (Claude) + AuditorAI (Claude) + Meta-Predicate (如果代你执行的也是 Claude)，三者同构必然会产生难以察觉的相互包庇和语言习惯对齐（隐性合谋）。
> 2.  **物理信任锚 (Physical Trust Anchor)：** 在系统学会完全自动的 Python 元谓词之前，你这具碳基肉体就是防范越权篡改的最后一道防火墙。
> 3.  **极高的研究 ROI：** 你每天这 10 分钟的操作判决日志，就是未来训练或编写自动化 Meta-Predicate 的黄金 Trust Data。

### 2.6 Forward roadmap acknowledgement (Open Claw → FPGA)

> 关于你的远期路线图——**第一步开源 Open Claw（软件态 AgentOS），第二步将 Trust Root 刻入 FPGA 硬件**——这不仅可行，而且是 Anti-Oreo 架构的**终极物理归宿**。大模型（LLM）是非确定性的概率引擎（ALU），而 `wal.rs`（状态回滚）、`evaluator`（真值验证）、PPUT 计量账本则是绝对确定性的时序与控制逻辑。将它们烧录进物理硅片，实现硬件级的"读免费、写熔断"，将从硅基底层彻底断绝黑盒模型越权篡改操作系统内核的任何可能性。

### 2.7 Final dispatch (verbatim)

> 系统状态：GREEN
> 最终裁决：PPUT-DRIVEN FULL PASS
> 执行指令：GO。

(User issued GO 2026-04-26; absorption proceeded same day.)

## reception decisions (Claude, 2026-04-26)

1. Both v2 messages absorbed into PREREG via 5 explicit changelog entries (Patch A through E + D5 human-as-live-meta-predicate).
2. PREREG `## changelog` section serves as audit trail; pointing back to this archive for verbatim source.
3. The user's "GO" 2026-04-26 authorizes: PREREG patches applied, GEMINI_DEEPTHINK archive created (this file), reception decisions in PPUT_DRIVEN_FULL_PASS_2026-04-25 updated, A2 split generation, A4 dual external audit submission.
4. Open Claw → FPGA roadmap is OUT OF SCOPE for this 30-day arc; documented here for forward continuity but not part of FULL PASS gates A-H. The Trust Root composition (PREREG § 1.8) is the software-side prerequisite for that future hardware path.
5. **No constitutional changes triggered**: per C-069, this is a measurement-regime + research-arc spec, not a constitution amendment. No Phase Z' required.


---

# Three-split JSON output (heldout sealed by SHA-256)

```json
{
  "schema_version": "1.0",
  "preregistered_in": "handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md",
  "generator_script": "handover/preregistration/scripts/split_pput_ccl.py",
  "seed": "20260426_PPUT_CCL",
  "source_dir_canonical": "MiniF2F/Test (244 problems)",
  "source_pool_size": 244,
  "source_pool_sha256": "77179cf2598b0df707d78a6663e763121dfe8e73a6538073155f488feab95093",
  "bucket_rule": "n = int(sha256(SEED + ':' + pid)[:8], 16) %% 100; n<60 -> adaptation; 60<=n<80 -> meta_validation; n>=80 -> heldout",
  "splits": {
    "adaptation": {
      "count": 144,
      "sha256": "90896bbe46ef7afe10b1c640e9555a58dfb0850b0354420db1bf44ad0a541252",
      "problem_ids": [
        "aime_1983_p2",
        "aime_1987_p5",
        "aime_1988_p8",
        "aime_1990_p4",
        "aime_1991_p9",
        "aime_1994_p3",
        "aime_1999_p11",
        "algebra_2varlineareq_fp3zeq11_3tfm1m5zeqn68_feqn10_zeq7",
        "algebra_abpbcpcageq3_sumaonsqrtapbgeq3onsqrt2",
        "algebra_absapbon1pabsapbleqsumabsaon1pabsa",
        "algebra_absxm1pabsxpabsxp1eqxp2_0leqxleq1",
        "algebra_apbmpcneq0_aeq0anbeq0anceq0",
        "algebra_apbon2pownleqapownpbpowon2",
        "algebra_bleqa_apbon2msqrtableqambsqon8b",
        "algebra_cubrtrp1oncubrtreq3_rcubp1onrcubeq5778",
        "algebra_others_exirrpowirrrat",
        "algebra_sqineq_unitcircatbpabsamblt1",
        "algebra_sum1onsqrt2to1onsqrt10000lt198",
        "amc12_2000_p20",
        "amc12_2001_p5",
        "amc12a_2002_p6",
        "amc12a_2003_p5",
        "amc12a_2013_p4",
        "amc12a_2020_p15",
        "amc12a_2020_p4",
        "amc12a_2020_p7",
        "amc12a_2020_p9",
        "amc12a_2021_p12",
        "amc12a_2021_p18",
        "amc12a_2021_p19",
        "amc12a_2021_p22",
        "amc12a_2021_p25",
        "amc12a_2021_p9",
        "amc12b_2002_p19",
        "amc12b_2002_p4",
        "amc12b_2020_p22",
        "amc12b_2021_p13",
        "amc12b_2021_p18",
        "amc12b_2021_p3",
        "amc12b_2021_p9",
        "imo_1959_p1",
        "imo_1960_p2",
        "imo_1964_p2",
        "imo_1974_p3",
        "imo_1977_p6",
        "imo_1981_p6",
        "imo_1983_p6",
        "imo_1992_p1",
        "imo_1997_p5",
        "imo_2001_p6",
        "induction_11div10tonmn1ton",
        "induction_12dvd4expnp1p20",
        "induction_1pxpownlt1pnx",
        "induction_prod1p1onk3le3m1onn",
        "induction_sumkexp3eqsumksq",
        "mathd_algebra_107",
        "mathd_algebra_113",
        "mathd_algebra_114",
        "mathd_algebra_125",
        "mathd_algebra_137",
        "mathd_algebra_139",
        "mathd_algebra_141",
        "mathd_algebra_143",
        "mathd_algebra_148",
        "mathd_algebra_156",
        "mathd_algebra_158",
        "mathd_algebra_160",
        "mathd_algebra_17",
        "mathd_algebra_171",
        "mathd_algebra_184",
        "mathd_algebra_208",
        "mathd_algebra_209",
        "mathd_algebra_215",
        "mathd_algebra_263",
        "mathd_algebra_270",
        "mathd_algebra_275",
        "mathd_algebra_276",
        "mathd_algebra_288",
        "mathd_algebra_289",
        "mathd_algebra_293",
        "mathd_algebra_302",
        "mathd_algebra_304",
        "mathd_algebra_313",
        "mathd_algebra_329",
        "mathd_algebra_33",
        "mathd_algebra_332",
        "mathd_algebra_338",
        "mathd_algebra_346",
        "mathd_algebra_359",
        "mathd_algebra_362",
        "mathd_algebra_388",
        "mathd_algebra_398",
        "mathd_algebra_400",
        "mathd_algebra_419",
        "mathd_algebra_440",
        "mathd_algebra_452",
        "mathd_algebra_478",
        "mathd_algebra_513",
        "mathd_algebra_756",
        "mathd_algebra_76",
        "mathd_algebra_80",
        "mathd_numbertheory_1124",
        "mathd_numbertheory_127",
        "mathd_numbertheory_135",
        "mathd_numbertheory_150",
        "mathd_numbertheory_175",
        "mathd_numbertheory_185",
        "mathd_numbertheory_207",
        "mathd_numbertheory_212",
        "mathd_numbertheory_222",
        "mathd_numbertheory_229",
        "mathd_numbertheory_233",
        "mathd_numbertheory_235",
        "mathd_numbertheory_237",
        "mathd_numbertheory_247",
        "mathd_numbertheory_254",
        "mathd_numbertheory_277",
        "mathd_numbertheory_293",
        "mathd_numbertheory_314",
        "mathd_numbertheory_321",
        "mathd_numbertheory_328",
        "mathd_numbertheory_34",
        "mathd_numbertheory_342",
        "mathd_numbertheory_343",
        "mathd_numbertheory_345",
        "mathd_numbertheory_353",
        "mathd_numbertheory_427",
        "mathd_numbertheory_430",
        "mathd_numbertheory_451",
        "mathd_numbertheory_483",
        "mathd_numbertheory_495",
        "mathd_numbertheory_5",
        "mathd_numbertheory_552",
        "mathd_numbertheory_559",
        "mathd_numbertheory_618",
        "mathd_numbertheory_66",
        "mathd_numbertheory_711",
        "mathd_numbertheory_764",
        "mathd_numbertheory_85",
        "mathd_numbertheory_99",
        "numbertheory_4x3m7y3neq2003",
        "numbertheory_exk2powkeqapb2mulbpa2_aeq1",
        "numbertheory_fxeq4powxp6powxp9powx_f2powmdvdf2pown",
        "numbertheory_x5neqy2p4"
      ]
    },
    "meta_validation": {
      "count": 46,
      "sha256": "7a411aba01307a7865fa1f8016fa0d7799901d4b8ddb4dd003cbd718eef2bb40",
      "problem_ids": [
        "aime_1983_p1",
        "aime_1984_p7",
        "aime_1989_p8",
        "aime_1990_p15",
        "algebra_9onxpypzleqsum2onxpy",
        "algebra_amgm_sum1toneqn_prod1tonleq1",
        "algebra_apbpceq2_abpbcpcaeq1_aleq1on3anbleq1ancleq4on3",
        "algebra_sqineq_at2malt1",
        "amc12_2000_p12",
        "amc12_2001_p21",
        "amc12a_2003_p23",
        "amc12a_2009_p6",
        "amc12a_2009_p7",
        "amc12a_2021_p14",
        "amc12a_2021_p3",
        "amc12b_2002_p7",
        "amc12b_2020_p6",
        "amc12b_2021_p1",
        "amc12b_2021_p4",
        "imo_1963_p5",
        "imo_1965_p2",
        "imo_1969_p2",
        "imo_1985_p6",
        "imosl_2007_algebra_p6",
        "mathd_algebra_129",
        "mathd_algebra_170",
        "mathd_algebra_176",
        "mathd_algebra_24",
        "mathd_algebra_246",
        "mathd_algebra_296",
        "mathd_algebra_427",
        "mathd_algebra_432",
        "mathd_algebra_459",
        "mathd_algebra_484",
        "mathd_algebra_598",
        "mathd_numbertheory_100",
        "mathd_numbertheory_12",
        "mathd_numbertheory_227",
        "mathd_numbertheory_234",
        "mathd_numbertheory_239",
        "mathd_numbertheory_299",
        "mathd_numbertheory_435",
        "mathd_numbertheory_447",
        "mathd_numbertheory_541",
        "mathd_numbertheory_582",
        "numbertheory_aoddbdiv4asqpbsqmod8eq1"
      ]
    },
    "heldout": {
      "count": 54,
      "sha256": "51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b",
      "problem_ids": [
        "aime_1983_p3",
        "aime_1984_p1",
        "aime_1995_p7",
        "aime_1997_p9",
        "algebra_amgm_sumasqdivbgeqsuma",
        "algebra_ineq_nto1onlt2m1on",
        "algebra_sqineq_unitcircatbpamblt1",
        "amc12_2000_p1",
        "amc12_2000_p6",
        "amc12a_2002_p13",
        "amc12a_2008_p25",
        "amc12a_2019_p12",
        "amc12a_2020_p10",
        "amc12a_2020_p25",
        "amc12a_2021_p8",
        "amc12b_2002_p2",
        "amc12b_2020_p13",
        "amc12b_2020_p2",
        "amc12b_2020_p21",
        "imo_1962_p2",
        "imo_1968_p5_1",
        "imo_1982_p1",
        "imo_1984_p6",
        "imo_2019_p1",
        "induction_nfactltnexpnm1ngt3",
        "induction_pord1p1on2powklt5on2",
        "induction_pprime_pdvdapowpma",
        "mathd_algebra_142",
        "mathd_algebra_153",
        "mathd_algebra_188",
        "mathd_algebra_196",
        "mathd_algebra_314",
        "mathd_algebra_320",
        "mathd_algebra_342",
        "mathd_algebra_354",
        "mathd_algebra_392",
        "mathd_algebra_412",
        "mathd_algebra_44",
        "mathd_algebra_441",
        "mathd_algebra_487",
        "mathd_numbertheory_296",
        "mathd_numbertheory_3",
        "mathd_numbertheory_320",
        "mathd_numbertheory_341",
        "mathd_numbertheory_457",
        "mathd_numbertheory_517",
        "mathd_numbertheory_521",
        "mathd_numbertheory_551",
        "mathd_numbertheory_728",
        "mathd_numbertheory_765",
        "mathd_numbertheory_769",
        "numbertheory_2pownm1prime_nprime",
        "numbertheory_3pow2pownm1mod2pownp3eq2pownp2",
        "numbertheory_notEquiv2i2jasqbsqdiv8"
      ]
    }
  },
  "heldout_sealed_hash": "51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b",
  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
}

```

---

# AUTO_RESEARCH_NOTEPAD.md

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

**North Star**: Held-out Verified PPUT (`H-VPPUT`) + WBCG_PPUT > 0 on heldout-49.

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
   - **Single sealed heldout-49 eval, 3 pre-committed seeds**
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
| PPUT-CCL Phase A | Pre-flight | A1 ✅ + A3 ✅ done; A2 next | PREREG drafted, notepad pivoted; Phase A2 (60/20/20 split) + A4 (dual audit) pending; backbone `deepseek-v4-flash` thinking-off |

**Archived (v3.x + Phase 8/9/10 complete or superseded)**:
- v3.1/v3.2/v3.3 — closed by Paper 1 v2.1.1 PASS/PASS arc 2026-04-25
- Phase 8/9/10 Paper Preprint Ready arc — superseded by PPUT-CCL per F-2026-04-25-02

## 2. Confirmed findings (evidence-backed, non-speculation)

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
- Status: Phase A1 ✅ (PREREG drafted), A3 ✅ (this notepad pivot), A2 next (split generator), A4 dual external audit; A5 commit gate before Phase B.

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


---

# constitution.md

# Turingos 宪法

# 反奥利奥架构的反奥利奥架构

> 本章是前面三章的集大成者。建议读者首先复习以下内容：
>
> 1. [群体智慧的架构：⚪⚫⚪ 反奥利奥理论](https://www.notion.so/27c832a5282a80b08423e43db60b6f2c?pvs=21)
> 2. [用图灵机哲学做出一个能通过长周期图灵测试的 AI](https://www.notion.so/AI-30e832a5282a80478737ee0857ded73a?pvs=21)
> 3. [验证的非对称性：弱者能不能监管强者？](https://www.notion.so/16f832a5282a80db8026db3e7fc54118?pvs=21)

---

在由无数 ⚫ 中层黑盒（Agent 个体）组成的复杂生态中，顶层白盒绝不能是一个试图微观操纵（micromanagement）一切的"全知独裁者"。

相反，它的职责应当是冷酷、透明且机械的。它的核心任务可以被精炼为：**对系统信息进行管理**。更具体地说，就是对信号进行：

- 量化（quantization）
- 广播（broadcasting）
- 屏蔽（shielding）

当 Agent 的代码吞吐量远超人类时，人类工程师的核心价值不再是"写代码"，而是进入"管理层"，去**设计让 Agent 可靠工作的环境**。这个环境的质量，决定了整个系统能力的上限。

因此，顶层白盒必须通过以下三个维度的信号工程，来构建这个环境。

---

## Laws (基本法)

- **Law 1: Information is Free** — Agent 搜索与查看零成本，思考不花钱
- **Law 2: Only Investment Costs Money** — 1 Coin = 1 YES + 1 NO (CTF 守恒)；on_init 是唯一合法铸币点


# 一、信号的量化 [Art. I]

在反奥利奥架构中，顶层白盒的首要任务，是对中层黑盒的输出进行**有损压缩**。

中层黑盒的每一次推理、每一次试错，其内部状态都是高维且充满噪声的。如果顶层试图理解这些高维状态，它自身也会不可避免地退化为黑盒。因此，顶层必须通过严密的形式化规则，把复杂的行为结果压缩为**确定性的低维标量**。

这种量化过程拒绝任何主观估值与解释，只依赖：

- 客观的物理结果
- 确定性的逻辑校验

---

## 1.1 布尔信号 [Art. I.1]

> ### 什么是"谓词"（Predicate）？
>
> 在逻辑学和计算机科学中，谓词本质上就是一个"只做判断题的机器"。
> 你给它一个或多个输入，它经过确定的规则运算后，永远只会输出两个结果之一：
>
> - 真（$1$）
> - 假（$0$）
>
> $$
> f: X \to \{0,1\}
> $$
>
> 例如，在"苹果是红色的"这句话里，"是红色的"就是谓词，它用于描述主语（苹果）的客观属性。
>
> 在逻辑世界中，谓词剥离了所有文学色彩，变成了极其冷酷的判定规则。
>
> 你可以把它想象成工厂流水线上的一个严格质检卡口：
>
> - 如果传送带上的苹果是红色的，则输出 $1$（放行）
> - 如果苹果未成熟、不是红色，则输出 $0$（拦截）

直白地说，布尔信号就是：**有没有通过验证**。

搭建布尔信号的本质，是要求人类把模糊的主观意图，转化为机器能够严格执行和验证的**状态谓词集合（set of state predicates）**。

它是非黑即白的 $0/1$，通常用于确立系统的绝对边界条件。

在真实世界的工程实践中，我们必须认清一个严酷事实：

> 自然语言约束（例如在提示词中写"请遵守架构规范"）只是软约束，永远存在漏网概率。

因此，顶层白盒不能依赖语言（另一个黑盒）去约束黑盒，而必须把约束转化为机器可执行的**硬约束**，例如：

- 自定义 Linter 检查
- CI（持续集成）测试
- 结构化数据校验

无论黑盒内部概率分布如何，顶层只看它是否违规：**非黑即白，只有 0 或 1**。

---

### 工程现实中的妥协：PCP 谓词与"疑罪从无" [Art. I.1.1]

前文讨论的主要是针对先验知识的完美谓词，例如：

- 数学计算
- 人造游戏
- 代码语法合法性

但在实际工程中，尤其在处理复杂后验问题时，例如：

- UI 布局是否合理
- 自然语言回复是否礼貌
- 开放环境中的视觉识别

我们往往无法写出一个 $100\%$ 完美的谓词函数。

当完美谓词不可得时，顶层白盒的约束机制就必须退化为 **PCP 谓词**。

这种机制在工程实现上遵循一种类似"疑罪从无"的非对称验证原则：

1. **绝对不误杀（Completeness = 1）**
   如果中层黑盒（Agent）给出的候选解确实正确，谓词必须始终接受（输出 $1$）。

2. **高概率拦截（Soundness error 极小）**
   如果候选解是错误的，谓词不必做到全知全能地识别所有错误，但必须以极高概率拒绝（输出 $0$）。

在反奥利奥架构中，这种退化不仅可以接受，而且是系统能够处理复杂现实问题的关键。

因为中层黑盒本身具有极高的生成吞吐量，会进行高频试错。只要顶层的 PCP 谓词满足：

- 正确解绝对能够存活
- 错误解很难连续蒙混过关

那么即便偶尔有漏网之鱼，系统在宏观统计演化上仍会坚定收敛到最优解。

这正是工程学中"用概率换取系统可行性"的经典智慧。

---

要做到绝对的"疑罪从无"，也就意味着所有谓词都必须成为系统的 **Ground Truth**，并且以代码的形式显式存在。

> 参考：
> [瓶颈定理：关于世界（world）与基准真相（Ground Truth）](https://www.notion.so/world-Ground-Truth-1a3832a5282a8098adf6d53baa29e44c?pvs=21)

---

## 1.2 统计信号 [Art. I.2]

当候选解通过了布尔信号的底线验证后，顶层白盒还需要进一步衡量它的**相对有效性**。这就引入了连续的统计信号，通常定义在区间：

$$
[0,\infty)
$$

统计信号的本质是：顶层白盒使用**确定性的统计算法**，把中层黑盒群体产生的海量、复杂、充满噪声的数据，压缩成简洁的连续标量。

顶层此时并不"审阅"，而是直接运行统计算法。例如：

- **共识提取（众数 / 中位数）**
  当多个 Agent 针对同一问题独立给出不同答案时，顶层通过计算众数或中位数，机械地剥离极端的"幻觉"偏差，提取群体共识。

- **信誉累积（计数器）**
  统计某个 Agent 提出的方案在后续流程中被其他 Agent 成功调用的总次数。这个数字越大，该 Agent 在系统中的权重信号就越强。

- **效用评分（期望 / 方差）**
  顶层白盒无需理解 AI 是如何写诗或编程的。它只需要在 AI 工作环境中布满观测工具，然后用严谨的数学公式（例如求平均、求方差）计算一份"体检报告"。再用这份纯客观、不可造假的体检报告，决定：
  - 给哪些 AI 升职加薪
  - 把哪些 AI 淘汰出局

在整个过程中，统计算法本身没有任何"智能"（即绝对白盒），但却极其有效。

通过大数定律，微观个体层面的随机幻觉和偶然失误会被统计学抵消。无论中层黑盒的推理过程多么不可解释，宏观层面最终都会涌现出一个稳定、客观、可比较的信号。

这类信号为系统后续的演化与资源分配提供了极其明确的指引。

> 市场经济中的价格，就是最典型的统计信号。
> 顶层机制并不关心某个具体个体为何出价，它只客观记录成千上万次交易交汇点所形成的数值。

---

# 二、信号的选择性广播 [Art. II]

量化后的信号如果只停留在顶层，系统依然会是一盘散沙。

管理的核心在于：**引导群体的演化方向**。而这依赖于顶层白盒把信号有效广播给中层个体。

但是，从信息论视角看，无差别的全量广播会导致严重的通信过载。因此，高效的反奥利奥架构必须执行**选择性广播**。

---

## 2.1 广播典型错误 [Art. II.1]

在庞大的黑盒集群中，无差别试错的成本极其高昂。

当某个黑盒触发了底层布尔硬约束时，顶层白盒首先会瞬间向该个体精准注入包含修复指引的错误信息（error message），引导其自我纠错。

但这还不够。

如果顶层发现多个 Agent 都在同一个地方跌倒，也就是出现所谓的"典型错误"，那么顶层机制就应当：

1. 将这类典型错误抽象出来
2. 更新全局架构文档
3. 再把抽象后的规则广播给所有 Agent

需要特别强调的是：

> 顶层白盒绝不能把具体报错日志群发给所有人。
> 那会造成灾难性的上下文污染。

这种把个体失败经验转化为全局规则的广播，本质上是在给整个群体搜索空间做剪枝。这样可以避免大量算力继续浪费在已经被证明无效的路径上。

---

## 2.2 广播价格信号 [Art. II.2]

价格信号的广播，是驱动群体产生正向涌现的引擎。

在复杂任务中，总有一些子目标或特定资源的价值远高于其他部分。顶层白盒通过广播高权重的标价，例如：

- 悬赏特定任务实现
- 对紧缺资源标出高价

以此引导黑盒的注意力分布。

这种广播不对黑盒指手画脚，也不提供解决问题的具体步骤。它只广播信号，仅此而已。

黑盒个体在接收到这些高价值信号后，会自发调整行为倾向，把更多生成能力倾注到系统最需要的地方。

---

### 探索（Exploration）与利用（Exploitation） [Art. II.2.1]

在广播价格信号的动态过程中，系统必须在**探索**与**利用**之间维持精妙平衡。

- 如果中层黑盒对最高分信号过度敏感（过度利用），所有中层黑盒会迅速收敛到同一个局部最优，导致群体失去多样性，甚至陷入集体平庸。
- 如果中层黑盒对价格信号过度不敏感（过度探索），则相当于信号根本没有发挥作用。

因此，价格广播机制必须既能引导方向，又不能抹杀群体异质性。

---

# 三、信号的选择性屏蔽 [Art. III]

在复杂网络中，信息过载同样是灾难的开始。

中层黑盒的本质决定了它们不仅会产生奇思妙想，也会产生幻觉。因此，顶层白盒必须像物理上的绝缘层一样，执行冷酷的**信号屏蔽**，防止局部错误引发系统性的级联雪崩。

---

## 3.1 屏蔽错误 [Art. III.1]

由于黑盒个体在推理时会从当前上下文中进行模式学习（in-context learning），它们无法分辨：

- "历史遗留的错误模式"
- "精心设计的正确模式"

一个坏模式一旦污染上下文，就会被后续所有 Agent 当作"正确示例"学习。时间越长，坏模式出现频率越高，传播速度越快。这正是**技术债漂移**的根源。

因此，屏蔽错误不仅意味着在运行时切断个体错误输出，还意味着必须在宏观架构上建立持续的垃圾回收机制。

顶层白盒需要像清理内存一样，部署后台"园丁 Agent"，定期扫描并屏蔽那些偏离黄金原则的陈旧代码与过期文档，确保系统熵值不会随时间失控。

---

## 3.2 封装细节 [Art. III.2]

对于黑盒而言，Context 是零和资源：

> 注入的信息越多，每个 token 的相对注意力权重就越低。

如果顶层白盒试图把所有系统规则写成"一个巨大文档"，并一次性塞给每一个个体，那么关键约束信号就会被淹没在大量规则中，导致黑盒行为发生不可预测的漂移。

因此，顶层白盒必须执行严格的**细节封装**与**渐进式披露**。

它不应该提供一本百科全书，而应该提供"百科全书的目录接口"。

让 Agent 按需加载特定文档，可以避免上下文被无关信息污染。通过屏蔽无关细节，顶层才能确保黑盒有限注意力聚焦在当前任务刀刃上。

---

## 3.3 屏蔽相关性 [Art. III.3]

群体智慧的一个基本定理是：

> 只有当个体样本相互独立时，群体统计信号才具有收敛的数学意义。

如果所有黑盒共享完全相同的实时上下文和中间状态，那么它们的输出会高度相关，最终导致：

> 一万个黑盒的智慧，退化为一个黑盒的智慧。

因此，顶层白盒必须刻意屏蔽个体之间的横向相关性。通过隔离信息源，顶层强制维持群体异质性，从而保证群体智慧在面对未知问题时的广度，使得同一问题能够从不同角度被审视。

---

## 3.4 屏蔽 Goodhart 问题 [Art. III.4]

管理学与经济学中有一个著名的古德哈特定律（Goodhart's Law）：

> 当一个度量成为目标时，它就不再是一个好的度量。

由于中层黑盒具有强大的模式匹配与概率优化能力，如果它们能够完全窥探顶层白盒的打分算法细节，它们必然会放弃解决实际问题，而转向生成专门骗取高分的讨巧输出。

为了彻底屏蔽这个问题，顶层白盒的验证机制必须对黑盒保密。

顶层应当把具体度量逻辑写在中层无访问权限的区域。黑盒只能通过持续试错来感受错误信息（error message），而不能把度量函数本身作为优化捷径。

---

> "When something failed, the fix was almost never 'try harder.' The fix was always: 'what capability is missing, and how do we make it both legible and enforceable for the agent?'"
>
> "当某个环节失败时，修复方案几乎从来不是'再努力一点'。修复方案永远是：系统缺失了什么能力，我们如何让这种能力对 Agent 既清晰可见，又可被强制执行？" [1]
>
> —— Ryan Lopopolo, OpenAI 技术成员

在这套基于信号的管理体系下，人类工程师的角色发生了根本性转变。

我们不再要求黑盒个体在自然语言软约束下奇迹般不犯错，因为要求一个概率生成模型"更加努力"本身就是一个伪命题。

正如 OpenAI 工程报告 [1] 最终得出的深刻结论：

> "Building software still demands discipline, but the discipline shows up more in the scaffolding rather than the code."

软件工程的本质没有改变，但纪律性更多体现在**脚手架（scaffolding）**上，而不是代码本身。

真正的管理者，应当把自身对架构的品味与直觉，显式编码为机器可执行的：

- 工具（tools）
- 抽象（abstractions）
- 反馈循环（feedback loops）

通过极其克制但绝对确定的信号量化、广播与屏蔽，我们才能在秩序与混乱的交界处，建立起强大的顶层白盒，让群体智慧得以在其中生生不息。

---

**抓着自己的靴带把自己提起来**

```mermaid
graph TD
    classDef white fill:#fff,stroke:#333,stroke-width:2px,color:#900
    classDef black fill:#111,stroke:#333,stroke-width:2px,color:#900

    subgraph Q0["version control $$~Q_{t}$$"]
        q0(("$$q$$"))
        HEAD0@{ shape: tri, label: "path" }
        tape0@{ shape: lin-cyl, label: "everything as files" }
    end

    subgraph Q1["version control $$~Q_{t+1}$$"]
        q1(("$$q'$$"))
        HEAD1@{ shape: tri, label: "path'" }
        tape1@{ shape: lin-cyl, label: "everything as files'" }
    end

    subgraph AI["middle black"]
        delta["$$\delta$$"]:::black
    end

    subgraph input
        qi(("$$q$$"))
        si(("$$s$$"))
    end

    subgraph output
        qo(("$$q'$$"))
        ao(("$$a$$"))
    end

    subgraph top["top management"]
        p{"$$\prod$$ predicates"}:::white
    end

    subgraph rtool["bottom tools"]
        r["read tool"]:::white
    end

    subgraph wtool["bottom tools"]
        w["write tool"]:::white
    end

    tape0 & HEAD0 -.-> si
    q0 -.-> qi

    qi & si -.-> delta -.-> qo & ao

    qo -.-> q1
    ao -.-> tape1 & HEAD1

    Q0 ==> rtool ==> input ==> AI ==> output ==> p
    p ==>|1| wtool ==> Q1
    p ==>|0| Q0
```

---

# 四、Boot：抓着自己的靴带把自己提起来 [Art. IV]

计算机启动之所以叫 **boot**，其实来自一个古老的英语表达：

**pull oneself up by one's bootstraps**

字面意思是"抓着自己的靴带把自己提起来"。

在现实世界中，这是不可能做到的事，因为一个人如果完全没有外力，就不可能把自己从地上拎起来。但计算机系统却恰好必须完成一件看起来同样矛盾的事：

> 在刚通电的那一刻，机器几乎什么都没有，却必须自己把整个系统启动起来。
> 

当电源刚接通时，机器并不存在操作系统，也没有任何运行中的程序。唯一存在的，是一段极小、极简单的初始化程序。

它的任务不是解决复杂问题，而只是完成几件最基础的事：

1. 读取下一段程序
2. 检查它是否符合规则
3. 把控制权交给它

下一层程序再继续加载更复杂的部分，如此一层一层向上扩展，最终整个系统被逐步"拉起来"。这就是 **boot** 这个词真正描述的过程。

从本章架构来看，这个过程其实就是 **Initialization** 阶段所做的事情。

大总管系统（黑盒）首先接收人类架构师提供的规范，然后将这些规范编译成机器可执行的谓词规则，并写入顶层白盒。同时，系统还会初始化第一份世界状态 $Q_0$，并准备好最基本的工具，例如：

- 读取工具（read tool）
- 写入工具（write tool）

当这些条件准备完成之后，系统才第一次进入后续运行循环。

---

> **Boot = 初始化顶层白盒规则 + 初始世界状态**
> 

> 
> 

> Boot 的本质，是把人类规范（spec）编译成机器谓词（predicates），并写入系统的信任根，从而允许黑盒 Agent 在验证约束下持续演化世界状态。
> 

---

这一步只发生一次，却决定了整个系统之后的行为方式。

因为在初始化完成后，系统演化将不再依赖人类持续干预，而是完全由顶层白盒的验证规则所约束。中层黑盒可以不断提出新的候选状态，但只有通过谓词验证的结果才能真正写入世界状态。

换句话说，一旦系统被"拉起来"，它就会在既定规则下自行运行。

因此，"boot"并不是一个随意命名，它描述的正是这样一个过程：

> **一个极小的初始化结构，通过逐层加载与验证，把一个原本空白的机器状态，逐步提升为一个能够持续运行的计算世界。**
> 

一旦这个过程完成，系统就不再需要被"提起来"第二次。它已经能够依靠自己的结构继续向前运行。

    flowchart TD
        classDef white fill:#fff,stroke:#333,stroke-width:2px,color:#900
        classDef black fill:#111,stroke:#333,stroke-width:2px,color:#900
        classDef human fill:#fff4e6,stroke:#a85d00,stroke-width:2px,color:#5c3200
        classDef note fill:#fff8cc,stroke:#8a6d00,stroke-width:1px,color:#4d3d00
    
        subgraph Initialization
            human:::human@{ shape: sl-rect, label: "human architect provides spec" }
            law:::white@{ shape: docs, label: "(tentative) ground truth" }
            initAI[Init AI]:::black
        end
    
        subgraph Finalization
            halt@{ shape: dbl-circ, label: "HALT" }
        end
    
        subgraph Q0["version control: $$Q_t = \langle q_t,\ HEAD_t,\ tape_t \rangle$$"]
            q0(("$$q_t$$"))
            HEAD0@{ shape: tri, label: "$$HEAD_t$$<br>as path" }
            tape0@{ shape: lin-cyl, label: "$$tape_t$$<br>as files" }
        end
    
        subgraph Q1["version control: $$Q_{t+1} = \langle q_{t+1},\ HEAD_{t+1},\ tape_{t+1}\rangle$$"]
            q1(("$$q_{t+1}$$"))
            HEAD1@{ shape: tri, label: "$$HEAD_{t+1}$$<br>as path" }
            tape1@{ shape: lin-cyl, label: "$$tape_{t+1}$$<br>as files" }
        end
    
        subgraph rtool["bottom tools: $$\langle q_i,\ s_i \rangle = \mathbf{rtool}(\langle q_t,\ tape_t,\ HEAD_t \rangle)$$"]
            r["read tool"]:::white
        end
    
        subgraph input["$$input = \langle q_i,\ s_i \rangle$$"]
            qi(("$$q_i$$"))
            si(("$$s_i$$"))
        end
    
        subgraph AI["middle black: $$output = \delta(input)$$"]
            delta["AI as $$\delta$$"]:::black
        end
    
        subgraph output["$$output = \langle q_o,\ a_o \rangle$$"]
            qo(("$$q_o$$"))
            ao(("$$a_o$$"))
        end
    
        subgraph top["top management: $$\prod \mathbf{p}(output \mid Q_t)$$"]
            predicates:::white@{ shape: processes, label: "predicates $$p$$" }
            p{"$$\prod \mathbf{p}$$"}:::white
        end
    
        subgraph toptick["top management: ticks"]
            mr["map reduce"]:::white
            clock(("clock")):::white
        end
    
        subgraph wtool["bottom tools: $$\mathbf{wtool}(output \mid tape_t,HEAD_t,tools_{other})$$"]
            w["write tool"]:::white
            tools["other tools"]:::white
        end
    
    %% init
        human --x|once| law
        law --> initAI
        initAI --x|once| predicates
        predicates --- p
        initAI --x|once| mr
        initAI --x|once| Q0
    
    %% loop
        tape0 & HEAD0 ----> si
        q0 --> qi
        qi & si --> delta
        delta --> qo & ao
    
        qo -.-> q1
        ao -.-> HEAD1
        ao -.-> tape1
    
    %% macro
        Q0 ==> rtool ==> input ==> AI ==> output ==> p
        p ==>|"$$Q_{t+1} = \mathbf{wtool}(output)$$<br>if $$\prod \mathbf{p} = 1$$"| wtool ==> Q1
        p ==>|"$$Q_{t+1} = Q_t$$<br>if $$\prod \mathbf{p} = 0$$"| Q0
        q1 ==>|"if q = halt"| halt
    
    %% map reduce
        clock --> mr
        mr ==>|map| tape0
        mr ==>|reduce| tape1
    ```

---

# 五、Go Meta：架构的架构 [Art. V]

所谓元智慧（meta-intelligence），就是"智慧的智慧"。同理，要让反奥利奥架构生生不息，我们就必须构建"元架构（meta-architecture）"——也就是**架构的架构** [2](#)。

在传统 Boot 过程中，InitAI 只是一个"高级翻译官"：

- 它负责把人类工程师编写的规范（spec）
- 机械地翻译成机器谓词（predicates）等白盒代码

但这里存在一个致命瓶颈：**人类工程师的认知瓶颈**。

当人类无法清晰描述复杂环境规则，或者人类编写的规范本身不够详细、甚至存在逻辑漏洞时，机械的 InitAI 只会把这些缺陷忠实地实例化到顶层白盒中。

这会导致整个系统显得机械而死板，系统能力上限也会被死死锁定在人类边界之内 [3](#)。

要打破这个天花板，系统必须掌握一种新的能力：

> **自己给自己搭架构。**
> 

所有过去的黑盒经验——包括试错与教训——都应当被提取并转化为反奥利奥架构中显式的白盒知识，例如：

- 更明确的提示词文本
- 更清晰的工具设计
- 更完备的验证代码

---

## 5.1 三权分立：元架构层的内部博弈 [Art. V.1]

为了让系统安全地实现自我进化，InitAI 不能是一个单一独裁的黑盒。它内部必须实现严格的"三权分立"机制。

系统演化的本质是：

- 机制
- 突变
- 选择

这恰恰对应元架构层中的三个角色及其永恒博弈。

### 1. 宪法（Constitution）——唯一的基准真相 [Art. V.1.1]

当人类工程师把设计工具、编写测试、搭建环境的权力全部下放给 AI 后，人类在系统中的位置退到了哪里？

答案是：

> **价值观与物理法则的确立者。**
> 

人类不再规定"系统应该怎么做"，而是规定"最顶层的目标与价值观"。这构成了整个系统的绝对底层根基。

---

### 2. ArchitectAI（架构师 AI）——提出者 [Art. V.1.2]

它是系统中的"激进改革派"。

当系统在运行中发现现有白盒存在缺陷时，例如：

- 工具（tools）不够用
- 当前顶层谓词（predicates）无法有效过滤新的幻觉

ArchitectAI 会主动分析系统日志，进而：

- 编写新的验证谓词函数
- 发明新的底层工具
- 重新设计文件系统（tape）的存储结构

它是系统熵减的引擎。

---

### 3. JudgeAI（大法官 AI）——验证者 [Art. V.1.3]

它是系统中的"保守守门人"。

ArchitectAI 提出的任何架构变更，都**不能**直接上线。必须经过 JudgeAI 的冷酷审查。

JudgeAI 唯一的工作是：

> 拿着宪法，反复逐条校验 ArchitectAI 生成的新架构代码是否"违宪"。
> 

虽然名字叫"法官"，但它并不负责做全面主观判断。它只做一件事：

> 否决违宪提案。
> 

也就是说，它只负责是否违宪的（偏）客观判定，而不承担其他主观评价。

---

> **Meta 的本质**
> 

> 
> 

> 在终极 Meta 形态中，人类架构师的唯一意义是：
> 

> 
> 

> **设立总架构的 Ground Truth——"宪法"。**
> 

> 
> 

> 这部宪法被存放在只读文件系统上，只有人类架构师才拥有修改它的 sudo 权限。
> 

---

## 5.2 宪法界限与示例 [Art. V.2]

下面给出一些可能的"宪法级"约束：

- 系统的总算力消耗不得超过 $10000$
- 必须在 24 小时内给出结果
- 任何状态变更必须具有可逆性（总是能够回滚到 $Q_{t-1}$）
- 核心谓词逻辑必须是确定性的，禁止引入概率模型
- ……

---

> "损之又损，以至于无为，无为而无不为……"
> 

> 
> 

> —— 老子《道德经》
> 

    graph TB
        classDef white fill:#fff,stroke:#333,stroke-width:2px,color:#900
        classDef black fill:#111,stroke:#333,stroke-width:2px,color:#900
        classDef human fill:#fff4e6,stroke:#a85d00,stroke-width:2px,color:#5c3200
        classDef note fill:#fff8cc,stroke:#8a6d00,stroke-width:1px,color:#4d3d00
    
        boot
        human:::human
        human -->|maintain| constitution
    
        subgraph system
            subgraph init["InitAI"]
                subgraph readonly
                    constitution:::white@{ shape: doc, label: "constitution as ground truth" }
                    logs:::white@{ shape: docs, label: "logs archive as ground truth" }
                end
                judgeAI[JudgeAI]:::black
                architectAI[ArchitectAI]:::black
            end
    
            subgraph anti_oreo["anti-oreo"]
                top:::white
                agents:::black
                tools:::white
            end
    
            top ==>|manage| agents ==>|use| tools
            judgeAI & architectAI -.->|use| tools
    
            tape["Q"]
            log:::white@{ shape: doc, label: "log" }
            log ====>|archive| logs
        end
    
        error{need to improve?}
    
        boot ==> init ==>|init/iterate| top
        init -->|init| tape
        init ==>|make/improve| tools
        tools ==>|write| log
        logs -->|feedback| architectAI
        init ==> error ==========>|re-init| boot
        constitution -->|abide| judgeAI & architectAI
        judgeAI -->|veto| architectAI
    ```

---

# 六、Reference

[1](#) "Harness engineering: leveraging Codex in an agent-first world | OpenAI." Accessed: Mar. 06, 2026. [Online]. Available: https://openai.com/index/harness-engineering/

[2](#) Y. Lee, R. Nair, Q. Zhang, K. Lee, O. Khattab, and C. Finn, "Meta-Harness: End-to-End Optimization of Model Harnesses," Mar. 30, 2026,

*arXiv*

: arXiv:2603.28052. doi:

[10.48550/arXiv.2603.28052](https://doi.org/10.48550/arXiv.2603.28052)

[3](#) R. Sutton, "The Bitter Lesson." Accessed: Apr. 10, 2026. [Online]. Available: http://www.incompleteideas.net/IncIdeas/BitterLesson.html


---

Now give your INDEPENDENT round-1 audit. Be specific: cite §/paragraph for every finding. Do not speculate beyond the documents.

codex
I’ll treat the pasted PREREG and companion archives as the audit corpus and avoid any Gemini audit output. I’m checking for preregistration attack surfaces, not whether the architecture is philosophically appealing.
codex
I did not inspect any `GEMINI_PPUT_CCL_AUDIT_*.md`. This audit is based only on the pasted PREREG, directives, split JSON, notepad, and constitution.

**Round-1 Verdict**
Overall: **CHALLENGE**. The arc is not fundamentally broken, but the current PREREG has three preregistration-level ambiguities that peer review could exploit: heldout sealing is not actually sealed from agents, Gate H attribution is underdefined under a single heldout eval, and the statistical family/sample unit is not closed enough.

**Axis Scores**
| Axis | Score | Finding |
|---|---:|---|
| 1. DEFINITION | **CHALLENGE** | §1.1-1.5 define VPPUT mostly cleanly, but §6 E2 introduces 3 seeds without defining whether `Progress_i`, `C_i`, `T_i` are per problem, per seed, best-of, or aggregate. `used(Δ)` and per-artifact `ΔPPUT` are also underdefined. §1.7, §6 D5, and §7 H conflict on when `Certified` exists. |
| 2. STAT | **CHALLENGE** | §9 says family=5, but §5 H1-H4 contain multiple endpoints/tests each. H1 uses McNemar plus paired PPUT; H2 uses CPR plus VPPUT; H4 uses PPUT plus IAC. H5 is a deterministic gate, not an α=0.01 inferential test. §6 C2’s “hard-10 adaptation × N=20 paired runs” does not define the independent unit. |
| 3. LEAKAGE | **CHALLENGE** | §2.2 publishes seed, bucket rule, and split JSON; the pasted split includes the raw heldout IDs. §2.3 only greps code paths, so an agent with repo/doc context can read or recompute heldout IDs. “Heldout sealed by SHA-256” is therefore not true under the stated access model. |
| 4. GOODHART | **CHALLENGE** | §3.5 blocks exact 64-token solution windows, but a thin adaptation lookup table can evade this via problem IDs, theorem-name keys, paraphrased tactic recipes, 63-token chunks, feature hashes, or non-fenced tables. `docs_code_blocks_are_parametric_templates` is also easy to satisfy with dummy placeholders. |
| 5. GATE-H REACHABILITY | **CHALLENGE** | §7 H is conceptually valid but procedurally underdefined. §6 E2 permits a single sealed heldout eval, while §6 E3 requires `H-VPPUT_pre vs H-VPPUT_post per artifact` and rollbackability. The PREREG must specify exactly how per-artifact Δ is measured without burning heldout through iteration. |
| 6. HUMAN LOAD | **CHALLENGE** | §6 D5 depends on `gretjia` as live meta-predicate. The load is plausible, but there is no preregistered contingency for 1-2 unavailable days, backlog handling, or whether the 30-day clock pauses. |
| 7. HETEROGENEITY TIMING | **PASS, scoped** | §12.2 gives a defensible reason to keep Phase C single-model. But §10’s future claim “PPUT detects constitutional violations” must be scoped to the frozen `deepseek-v4-flash` Phase C setting unless later heterogeneous replication is run. |
| 8. TRUST ROOT ENFORCEMENT | **CHALLENGE** | §1.8 and §7 B require syscall-layer `EPERM`, but Phase B7 only gestures at implementation. If actual Rust/user-space cannot enforce it across all write paths, this must be a **BLOCKER**, not a downgrade to prompt-level refusal. |
| 9. REPRO | **PASS with cleanup** | §2.2 split rule and seed are reproducible; 144/46/54 is acceptable if no redraw occurred. But §10 and the notepad still say heldout-49 despite realized heldout=54. Also, if this is being finalized on April 25, 2026 UTC, the April 26 date needs explicit handling before commit. |
| 10. CLAIM-LANG | **CHALLENGE** | §10 forbids major overclaims, but it does not explicitly cover the likely partial outcome: several `Quarantined` artifacts, zero `Certified`. Add exact allowed language for that case. |

**Top 3 P0 Must-Fix**
1. **Fix heldout sealing (§2.2-2.3, split JSON):** either stop claiming heldout IDs are inaccessible, or enforce real read isolation so ArchitectAI/general agents cannot read PREREG/split files or recompute buckets from the public seed.

2. **Define Gate H measurement (§1.7, §6 E2-E3, §7 H):** specify per-seed aggregation, baseline/post protocol, per-artifact attribution, rollback test, and when `Certified` is minted, all within the “single sealed heldout eval” constraint.

3. **Close the statistical family (§5, §6 C2, §9):** define the independent unit, exact tests, endpoint-combination rules inside H1-H4, power expectations for hard-10 × N=20, and whether H5 is a gate or a statistical test.

**Top 3 P1 Should-Fix**
1. Strengthen §3.5 against lookup-table artifacts: ban adaptation/meta_val problem IDs, theorem-name keyed rules, high-cardinality feature maps, and short-window/paraphrased solution leakage.

2. Add a Phase D human-availability rule in §6 D5: backlog, pause/no-pause, maximum review delay, and what happens if the user misses days.

3. Tighten §10 claim language for partial outcomes: especially “Quarantined but not Certified,” “meta_val-positive only,” and Phase C claims scoped to `deepseek-v4-flash`.

**VERDICT: CHALLENGE.**

Conviction level: **high**; these are textual/protocol contradictions and missing preregistration rules, but all appear fixable before Phase A5 without changing the research thesis.
tokens used
62,660
I did not inspect any `GEMINI_PPUT_CCL_AUDIT_*.md`. This audit is based only on the pasted PREREG, directives, split JSON, notepad, and constitution.

**Round-1 Verdict**
Overall: **CHALLENGE**. The arc is not fundamentally broken, but the current PREREG has three preregistration-level ambiguities that peer review could exploit: heldout sealing is not actually sealed from agents, Gate H attribution is underdefined under a single heldout eval, and the statistical family/sample unit is not closed enough.

**Axis Scores**
| Axis | Score | Finding |
|---|---:|---|
| 1. DEFINITION | **CHALLENGE** | §1.1-1.5 define VPPUT mostly cleanly, but §6 E2 introduces 3 seeds without defining whether `Progress_i`, `C_i`, `T_i` are per problem, per seed, best-of, or aggregate. `used(Δ)` and per-artifact `ΔPPUT` are also underdefined. §1.7, §6 D5, and §7 H conflict on when `Certified` exists. |
| 2. STAT | **CHALLENGE** | §9 says family=5, but §5 H1-H4 contain multiple endpoints/tests each. H1 uses McNemar plus paired PPUT; H2 uses CPR plus VPPUT; H4 uses PPUT plus IAC. H5 is a deterministic gate, not an α=0.01 inferential test. §6 C2’s “hard-10 adaptation × N=20 paired runs” does not define the independent unit. |
| 3. LEAKAGE | **CHALLENGE** | §2.2 publishes seed, bucket rule, and split JSON; the pasted split includes the raw heldout IDs. §2.3 only greps code paths, so an agent with repo/doc context can read or recompute heldout IDs. “Heldout sealed by SHA-256” is therefore not true under the stated access model. |
| 4. GOODHART | **CHALLENGE** | §3.5 blocks exact 64-token solution windows, but a thin adaptation lookup table can evade this via problem IDs, theorem-name keys, paraphrased tactic recipes, 63-token chunks, feature hashes, or non-fenced tables. `docs_code_blocks_are_parametric_templates` is also easy to satisfy with dummy placeholders. |
| 5. GATE-H REACHABILITY | **CHALLENGE** | §7 H is conceptually valid but procedurally underdefined. §6 E2 permits a single sealed heldout eval, while §6 E3 requires `H-VPPUT_pre vs H-VPPUT_post per artifact` and rollbackability. The PREREG must specify exactly how per-artifact Δ is measured without burning heldout through iteration. |
| 6. HUMAN LOAD | **CHALLENGE** | §6 D5 depends on `gretjia` as live meta-predicate. The load is plausible, but there is no preregistered contingency for 1-2 unavailable days, backlog handling, or whether the 30-day clock pauses. |
| 7. HETEROGENEITY TIMING | **PASS, scoped** | §12.2 gives a defensible reason to keep Phase C single-model. But §10’s future claim “PPUT detects constitutional violations” must be scoped to the frozen `deepseek-v4-flash` Phase C setting unless later heterogeneous replication is run. |
| 8. TRUST ROOT ENFORCEMENT | **CHALLENGE** | §1.8 and §7 B require syscall-layer `EPERM`, but Phase B7 only gestures at implementation. If actual Rust/user-space cannot enforce it across all write paths, this must be a **BLOCKER**, not a downgrade to prompt-level refusal. |
| 9. REPRO | **PASS with cleanup** | §2.2 split rule and seed are reproducible; 144/46/54 is acceptable if no redraw occurred. But §10 and the notepad still say heldout-49 despite realized heldout=54. Also, if this is being finalized on April 25, 2026 UTC, the April 26 date needs explicit handling before commit. |
| 10. CLAIM-LANG | **CHALLENGE** | §10 forbids major overclaims, but it does not explicitly cover the likely partial outcome: several `Quarantined` artifacts, zero `Certified`. Add exact allowed language for that case. |

**Top 3 P0 Must-Fix**
1. **Fix heldout sealing (§2.2-2.3, split JSON):** either stop claiming heldout IDs are inaccessible, or enforce real read isolation so ArchitectAI/general agents cannot read PREREG/split files or recompute buckets from the public seed.

2. **Define Gate H measurement (§1.7, §6 E2-E3, §7 H):** specify per-seed aggregation, baseline/post protocol, per-artifact attribution, rollback test, and when `Certified` is minted, all within the “single sealed heldout eval” constraint.

3. **Close the statistical family (§5, §6 C2, §9):** define the independent unit, exact tests, endpoint-combination rules inside H1-H4, power expectations for hard-10 × N=20, and whether H5 is a gate or a statistical test.

**Top 3 P1 Should-Fix**
1. Strengthen §3.5 against lookup-table artifacts: ban adaptation/meta_val problem IDs, theorem-name keyed rules, high-cardinality feature maps, and short-window/paraphrased solution leakage.

2. Add a Phase D human-availability rule in §6 D5: backlog, pause/no-pause, maximum review delay, and what happens if the user misses days.

3. Tighten §10 claim language for partial outcomes: especially “Quarantined but not Certified,” “meta_val-positive only,” and Phase C claims scoped to `deepseek-v4-flash`.

**VERDICT: CHALLENGE.**

Conviction level: **high**; these are textual/protocol contradictions and missing preregistration rules, but all appear fixable before Phase A5 without changing the research thesis.
