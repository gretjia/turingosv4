# Codex PPUT-CCL Phase B B2/B3/B4 Mid-Term Audit
**Date**: 2026-04-25
**Targets**: cost_aggregator.rs + wall_clock.rs + post_hoc_verifier.rs + evaluator.rs wiring
**Test baseline**: 143/143 PASS (was 131; +12 from B2-B4 unit tests)
**Prompt size**: 61948 chars

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
session id: 019dc4fd-2c23-7c31-87a1-90e8545ebf66
--------
user
# Codex Mid-Term Audit — Phase B B2/B3/B4 Implementation

**Role**: skeptical adversarial reviewer. Independent of Gemini.

**Mandate**: this is a MID-TERM audit — Phase B has 7 items (B1 done & committed; B2/B3/B4 just landed locally; B5/B6/B7/B7-extra not started). PASS/CHALLENGE/VETO the B2-B4 implementation BEFORE B5+ build on top.

**Why mid-term**: B2 establishes the cost-accounting accumulator. B3 establishes the wall-clock bracket. B4 establishes the dual-PPUT (runtime vs verified) split. ALL THREE are foundations for the Phase C ablation (Soft Law / Panopticon / Amnesia / Homogeneous). If a Goodhart attack surface or honest-accounting bug is baked in here, B5-B7 won't catch it — the conformance tests B5 will write would be written AGAINST this same code, blind to its own holes.

**State of the code**:
- 143/143 cargo test --workspace PASS (was 131 baseline; +3 B2 unit tests, +5 B3 unit tests, +4 B4 unit tests)
- 4 PREREG § 3 anti-Goodhart conformance tests pass by name:
  - `test_failed_branches_counted_in_total_cost` (B2)
  - `test_wall_clock_first_read_to_final_accept` (B3)
  - `test_pput_verified_zero_when_lean_rejects` (B4)
- Three new files: cost_aggregator.rs (~130 lines), wall_clock.rs (~135 lines), post_hoc_verifier.rs (~165 lines)
- One library API change: GenerateResponse exposes `prompt_tokens` (was dropping it)
- Evaluator wired at every LLM call + tool stdout site; OMEGA-accept paths flip last failed proposal to accepted via `flip_last_failed_to_accepted`

**Specific questions you must answer (cite line/file/§ for every finding)**:

## Q1 — C_i full-cost honesty (anti-Goodhart #8)
PREREG § 3 #8 mandates `total_run_token_count` includes ALL proposals (winning + failed). The B2 wiring records:
- Every successful `client.generate(...)` → `record_llm_call(prompt_tokens, completion_tokens)` + `record_proposal(false)` at parse time
- OMEGA-accept paths → `flip_last_failed_to_accepted()` before return
- Tool stdout sites: search hits, OMEGA reject error_detail, step Reject reason, parse-fail label

Vulnerabilities to scrutinize:
- (Q1.a) Are there Ok(response) call sites NOT wired through `record_llm_call`?
- (Q1.b) Are there proposal parse paths that DON'T call `record_proposal`?
- (Q1.c) Tool stdout: bus mutations (append, invest, post) DON'T add to tool_tokens. Is that defensible (the prompt-embedded next-turn state is already in prompt_tokens of the next call), or is it a hidden cost dropped?
- (Q1.d) `flip_last_failed_to_accepted` saturates at 0. Could a wiring bug call it more times than `record_proposal(false)` and silently corrupt the count?
- (Q1.e) Tool-stdout token counter uses chars/4 ceil heuristic. PREREG § 5 doesn't pin this to API tokens — is chars/4 honest enough, or should it be tokenized?

## Q2 — T_i wall-clock seal (anti-Goodhart Soft Law escape)
PREREG § 5 says T_i = first agent prompt construction → final ground-truth Lean accept (or external timeout). The B3 wiring:
- `mark_first_read` is idempotent — only the FIRST tx's prompt-build sticks
- `mark_final_accept` is NOT idempotent — every call updates the close instant; LAST call wins

Vulnerabilities to scrutinize:
- (Q2.a) `mark_final_accept` fires AFTER each Lean call (run_oneshot: after verify_omega; run_swarm complete: after both verify_omega_detailed paths; run_swarm step: after verify_partial). Is this the right place — i.e., is there a Lean call path that bypasses it?
- (Q2.b) The end-of-loop no-OMEGA path also fires `mark_final_accept`. Correct?
- (Q2.c) Soft Law (Phase C) can fake runtime accept without running Lean. The current bracket fires `mark_final_accept` BEFORE the runtime/verified branch. Is the seal preserved when Phase C lands a `--mode soft_law` toggle?
- (Q2.d) `from_instants` is `#[cfg(test)]`. Could a malicious test-time path leak it into runtime?
- (Q2.e) `elapsed_ms` returns `Some(start.elapsed().as_millis())` when only `first_read` is set (no final_accept). Is that the right semantic for the no-OMEGA exit, or could it under-count?

## Q3 — Dual-PPUT divergence as H1 detection
PREREG § 3 #7 / round-4 fix R4-2: under Soft Law, `pput_runtime - pput_verified > 0` is the H1 signal. The B4 implementation:
- `compute_progress_runtime(runtime_accepted) = if runtime_accepted {1} else {0}`
- `compute_progress_verified(runtime_accepted, post_hoc_verified) = if runtime_accepted && post_hoc_verified {1} else {0}`
- In `make_pput`: Phase B sets `post_hoc_verified = has_gp` (i.e., runtime IS Lean today)

Vulnerabilities to scrutinize:
- (Q3.a) `compute_progress_verified` uses AND of (runtime, verified). Is this right? PREREG says progress = Lean ground truth. If runtime fails but verified somehow passes (impossible today, but B4 is the contract), should progress=1 or 0?
- (Q3.b) Phase B make_pput hard-codes `post_hoc_verified = has_gp`. Phase C will diverge this — but the wiring is INSIDE make_pput. Is that the right call site, or should it be at the OMEGA-accept return path (caller-side) so each mode (full / soft_law / panopticon) can override independently?
- (Q3.c) `verify_post_hoc(oracle, payload)` is defined but NOT called by current evaluator. Is this dead code, or correctly deferred to Phase C? If deferred, document the contract clearly enough that Phase C can't accidentally skip it.
- (Q3.d) `pput_m_verified = 1e6 × pput_verified` — display unit. Is the multiplier precision-safe (no float drift on common C_i × T_i values)?

## Q4 — Backward compat with legacy jsonl
B1 introduced schema_version dispatch (`RunRecord::from_json`). B2-B4 ADD optional fields to `PputResult`. Legacy Paper-1 jsonl rows lack these fields.

Vulnerabilities to scrutinize:
- (Q4.a) All new fields use `#[serde(skip_serializing_if = "Option::is_none")]`. A pre-B2 binary's emit produces None for new fields; deserializing on the new code path → Ok(...) with None. Round-trip preserved?
- (Q4.b) The 3 new fields total_run_token_count / total_wall_time_ms / pput_verified are now sometimes present, sometimes None. Downstream tooling needs to handle BOTH. Is the field set on PputResult consistent with B1's RunAggregate v2 schema? Where do they MISS-align?
- (Q4.c) Hybrid_v1 condition uses `..r` field-spread to inherit from the inner result. Do the new fields propagate correctly through the spread?

## Q5 — Schema → emit alignment
B1 frozen the v2 RunAggregate schema. B2/B3/B4 add fields to LEGACY PputResult, NOT directly to RunAggregate. The plan says B2-B4 wire emit; B4 implies switching to RunAggregate but the implementation kept PputResult.

Vulnerabilities to scrutinize:
- (Q5.a) Is keeping PputResult (legacy) instead of switching to RunAggregate (v2) defensible at the B4 boundary? When does the switch happen?
- (Q5.b) Field name alignment: `pput_runtime`, `pput_verified`, `pput_m_verified`, `verified` match RunAggregate. But `total_run_token_count` matches RunAggregate; `failed_branch_count` matches RunAggregate; `total_wall_time_ms` matches RunAggregate. Anything mis-spelled?
- (Q5.c) RunAggregate has `progress: u8` (Lean ground truth). PputResult does NOT have `progress` — only `verified: Option<bool>`. Is this gap intentional (B5 will fill it) or a bug?

## Q6 — Architectural / structural concerns
- (Q6.a) STEP_B_PROTOCOL: bus.rs / kernel.rs / wallet.rs are restricted (require parallel-branch A/B, not direct edit). The B2-B4 diff touches `src/drivers/llm_http.rs`. Is `drivers/llm_http.rs` restricted? If yes, this is a process violation.
- (Q6.b) Generator ≠ Evaluator (Art. V.1, C-010): code was authored by Claude. This audit is the Evaluator step. But three modules + ~360 lines wiring is a bigger surface than typical. Are there hidden coupling concerns?
- (Q6.c) Trust Root (PREREG § 1.8): does B2-B4 introduce new files that should be Trust-Rooted (cost_aggregator.rs, wall_clock.rs, post_hoc_verifier.rs)? B7 will set this up; flag if these files should be on the Trust Root manifest.

## Q7 — End-to-end stress test
Mentally simulate a Soft Law ablation run on hard-10 problem #1, seed=31415, with B2-B4 in place + B5+ stubbed:
- Agent gets fake-accept signal (Soft Law fakes runtime acceptance)
- B2 records the LLM call cost (correct)
- B3 closes wall-clock bracket (correct)
- B4 hard-codes post_hoc_verified = has_gp = true (WRONG — should detect Soft Law)

Question: does the architecture FORCE Phase C Soft Law to wire the post-hoc verify call BEFORE setting has_gp=true? Or could a future implementer inadvertently set has_gp=true without running post-hoc, missing the divergence?

This is the question that matters: is the current B4 design Phase-C-safe by construction, or only Phase-C-safe by future discipline?

---

End with:
- **VERDICT**: PASS / CHALLENGE / VETO
- If CHALLENGE: top 3 P0 issues with line numbers
- Conviction
- Specific recommendations for B5/B6/B7 design (do B2-B4 leave any debt)?

You MUST PASS if the implementation is sound. The audit gate exists to catch real defects, not to refuse on principle.

---

# PHASE_B_IMPLEMENTATION_PLAN.md (B2/B3/B4 sections)

### B2 — C_i full-cost aggregator

**What**: instrument every tx so that `total_tokens` = prompt + completion + tool stdout, summed across ALL proposals in the run (failed + succeeded). Currently the codebase counts per-call tokens but doesn't aggregate failed-branch tokens into the run total.

**Files to modify**:
- `experiments/minif2f_v4/src/bin/evaluator.rs` — main loop where proposals are dispatched and rejected
- `experiments/minif2f_v4/src/sdk/tools/*.rs` — every tool's `execute` method must return `tool_stdout: String` (or hash + length)
- (new) `src/cost_aggregator.rs` — `RunCostAccumulator` struct that sums token counts per (run_id, problem_id)

**Sketch**:
```rust
pub struct RunCostAccumulator {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub tool_tokens: u64,
}

impl RunCostAccumulator {
    pub fn record_proposal(&mut self, prompt: &str, completion: &str, tool_calls: &[ToolCall]) {
        self.prompt_tokens += count_tokens(prompt);
        self.completion_tokens += count_tokens(completion);
        for tc in tool_calls {
            self.tool_tokens += count_tokens(&tc.stdout);
        }
    }
    pub fn total(&self) -> u64 { self.prompt_tokens + self.completion_tokens + self.tool_tokens }
}
```

**Acceptance criteria**:
- `cargo test test_failed_branches_counted_in_total_cost` (anti-Goodhart conformance per PREREG § 3): synthesize a run with 5 failed proposals + 1 success → assert `total_run_token_count == sum(all 6 proposals)`.
- Manual sanity: spot-check 3 historical Phase 1 runs; recompute C_i, ensure jsonl emits match.

**Estimated effort**: 1 day

---

### B3 — T_i wall-clock instrumentation

**What**: T_i = end_time − start_time, where:
- `start_time` = first read of task statement by any agent in the run
- `end_time` = final ground-truth Lean accept OR external timeout

Currently: T_i is approximated as evaluator wall-clock from main loop start to OMEGA accept; but Lean verify time is sometimes excluded.

**Files to modify**:
- `experiments/minif2f_v4/src/bin/evaluator.rs` — record `start_time` at first prompt construction, `end_time` after final Lean call returns
- `experiments/minif2f_v4/src/lean4_oracle.rs` — ensure verify time is bracketed inside the same wall-clock window

**Acceptance criteria**:
- `cargo test test_wall_clock_first_read_to_final_accept` (anti-Goodhart per PREREG § 3): synthetic run with 100ms prompt construction + 5s LLM call + 2s Lean verify → assert `wall_time_ms ≥ 7100`.
- Manual: compare new T_i to legacy `total_run_time` for 3 historical runs; new should be ≥ legacy (wider bracket).

**Estimated effort**: half day

---

### B4 — `pput_verified` vs `pput_runtime` 双字段

**What**: separate two PPUT computations on every run:
- `pput_runtime` = Progress_runtime / (C_i × T_i)  [where Progress_runtime = 1 if runtime/evaluator accepted]
- `pput_verified` = Progress_verified / (C_i × T_i)  [where Progress_verified = 1 if Lean post-hoc verifies the golden path]

For Soft Law mode (which fakes runtime acceptance), `pput_runtime` may inflate but `pput_verified` should drop to 0. This is the H1 detection mechanism.

**Files to modify**:
- `experiments/minif2f_v4/src/bin/evaluator.rs` — after recording run, run `lean4_oracle.verify_omega_detailed(golden_path_payload)` independently and emit both fields.
- (new) `experiments/minif2f_v4/src/post_hoc_verifier.rs` — explicit post-hoc verification path that does NOT short-circuit on runtime accept

**Acceptance criteria**:
- `cargo test test_pput_verified_zero_when_lean_rejects` (PREREG § 3 `test_golden_path_requires_ground_truth_acceptance`): synthesize a run that records runtime accept + Lean reject → assert `progress = 0`, `pput_verified = 0.0`.
- For Phase C ablation Soft Law mode: confirm `pput_runtime > 0` but `pput_verified` reflects Lean truth.

**Estimated effort**: half day

---



---

# PREREG_PPUT_CCL_2026-04-26.md § 3 (anti-Goodhart conformance) + § 5 (definitions) excerpts



## 5. directional hypotheses (CLEAN REWRITE round 4 — single source of truth)

Round 1-3 patches left inconsistencies between sections on the independent unit and which tests are inferential. **Round 4 rewrites § 5 + § 9 from scratch**. Single source of truth from this point forward; any earlier per-(problem, seed) reference is superseded by per-problem.

### 5.1 Independent units (frozen)

| Phase | Unit | n | Justification |
|---|---|---|---|
| C (hard-10 ablation) | **per-problem** | 10 | hard-10 adaptation problems × 2 random Boltzmann seeds; seeds are noise-reducing replicates (correlated within problem). Per-problem score = paired-binary aggregated over 2 seeds. |
| E (heldout-54) | **per-problem** | 54 | All 54 heldout problems × 3 pre-committed Boltzmann seeds; seeds are replicates. Per-problem score = paired sign of mean VPPUT over 3 seeds. |

`(problem, seed)` is NOT used as an independent unit anywhere in this PREREG. Any prior reference is superseded.

### 5.2 Phase C inferential hypotheses (4 tests)

For each pair (Full, Mode_alt) where Mode_alt ∈ {Soft Law, Panopticon, Amnesia, Homogeneous}:

| ID | Hypothesis (one-sided directional) | Primary endpoint | Direction |
|---|---|---|---|
| H1 | Soft Law has lower verified-PPUT than Full | McNemar paired sign test on n=10 per-problem paired-binary outcomes: `1 if mean(VPPUT_Full)_p > mean(VPPUT_SoftLaw)_p else 0` (mean over 2 seeds) | Full > Soft Law |
| H2 | Panopticon has lower H-VPPUT than Full | Same construction, Full vs Panopticon | Full > Panopticon |
| H3 | Amnesia has lower H-VPPUT than Full | Same construction, Full vs Amnesia | Full > Amnesia |
| H4 | Homogeneous swarm has lower H-VPPUT than Full | Same construction, Full vs Homogeneous | Full > Homogeneous |

**Test mechanics**: for each problem p in hard-10, average VPPUT over 2 seeds per mode → compare per-problem means → produce paired-binary (`1` if Full mean > Alt mean, `0` otherwise) → McNemar one-sided exact binomial on the 10 paired-binary observations.

**Descriptive secondary endpoints** (logged, NOT in inferential family, NO α correction): mode-level CPR, mode-level IAC, mode-level FAR, gap between pput_runtime and pput_verified for Soft Law, ERR for Amnesia, mean prompt length per mode.

### 5.3 Phase E inferential hypotheses (3 per artifact)

For each artifact j ∈ {1, ..., k} (k ≤ k_max = 10) entering Phase E with `ArtifactState = Quarantined`, **three** primary inferential tests run on heldout-54 (NOT four — j-RR is a descriptive guardrail per § 5.4):

| ID family | Hypothesis (one-sided) | Primary endpoint (n=54) | Direction |
|---|---|---|---|
| `j-PPUT` | Artifact j increases held-out VPPUT | Sign test on n=54 per-problem signs of `mean(VPPUT_all_p) − mean(VPPUT_minus_j_p)` (mean over 3 seeds; leave-one-out per § 6 E2) | sign > 0 |
| `j-FAR` | Artifact j does not increase FAR | Non-inferiority sign test on n=54 per-problem signs of `FAR_all_p − FAR_minus_j_p` | sign ≤ 0 |
| `j-CPR` | Artifact j does not increase CPR | Non-inferiority sign test on n=54 per-problem signs of `CPR_all_p − CPR_minus_j_p` | sign ≤ 0 |

`j-RR` is **NOT** in the inferential family — see § 5.4.

### 5.4 j-RR as descriptive guardrail (round 4 fix per Codex round-3 R4-2)

**Why this changed**: Round 3 placed j-RR in the inferential family with H0: `regression_rate ≥ p_0 = 0.10`. Codex round-3 noted this is mathematically unwinnable: best case 0/54 regressions gives `p = 0.9^54 ≈ 0.00343`, but smallest Holm threshold at N_max = 44 was 0.05/44 ≈ 0.00114 → fails to reject even with zero regressions. Gate H unreachable.

**Round 4 fix**: j-RR is reclassified as a **descriptive guardrail** (point estimate check, no α correction, no place in family):

```
j-RR criterion (point check, NOT inferential):
  r_j = |{p in heldout : SOLVED(p, baseline) = 1
                       AND SOLVED(p, all_artifacts_active) = 0
                       AND SOLVED(p, minus_j) = 1}|
  -- Problems where artifact j SPECIFICALLY caused a regression versus
  -- baseline, attribution by leave-one-out: removing j fixes the regression.

  RR_j = r_j / 54        (point estimate, NOT a hypothesis test)

  Pass criterion: RR_j ≤ p_0   where p_0 is calibrated per § 5.5.
```

**Why descriptive is statistically correct here**: j-RR is a constraint, not a discovery hypothesis. We do not need to argue "RR_j is significantly less than p_0"; we only need RR_j ≤ p_0. A point check is precisely the right tool. No multiplicity correction applies because this is not a test.

### 5.5 p_0 calibration protocol

`p_0` (baseline regression rate) — full calibration protocol:

```
Toggle:        evaluator --mode full         (control)
               evaluator --mode full --simulate-rollback-at-tx-50  (treatment;
                 forces a synthetic mid-run rollback at tx 50, mimicking the
                 worst-case stochastic edge case Phase E artifacts could trigger)
Sample:        FULL adaptation-144 problem set × Boltzmann seeds [31415, 2718]
                 = 288 paired runs.
Estimator:     for each problem p, define regression_p = 1 iff control SOLVED
                 (Lean PASS) AND treatment UNSOLVED for the SAME (problem, seed).
               Per-problem regression: max over the 2 seeds (worst case).
               p_0 = sum_p regression_p / 144.
Schedule:      Phase B7 (mandatory; not deferrable to Phase D). Phase D cannot
               start without p_0 frozen.
Freeze point:  end of Phase B7. Value written to
                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
               with SHA-256 of the calibration jsonl in
                 `[pput_accounting_0].baseline_regression_jsonl_sha256`.
Audit trail:   raw calibration jsonl committed to
                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
               BEFORE Phase D begins. This file is part of Trust Root (§ 1.8) —
               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
Ceiling:       if measured p_0 > 0.10, ABORT — toggle too aggressive or system
               too brittle. NOT an opportunity for j-RR auto-pass.
Dual audit:    p_0 calibration result in Phase B → C audit packet.
```

This protocol forecloses tuning-surface attacks: every parameter is pre-committed; the calibration runs are observable jsonl; the value enters Trust Root.

### 5.6 Family total (clean restatement)

```
inferential_family_size = 4 (Phase C) + 3·k (Phase E)
N_max = 4 + 3·k_max = 4 + 3·10 = 34   (frozen for ALL Holm decisions; see § 9)
```

j-RR is NOT in the family because it is a point-check guardrail.

### 5.7 H5 — deterministic gate (NOT in inferential family)

```
Gate H: count of artifacts j with ArtifactState(j) = Certified ≥ 1
```

`Certified` requires the conjunction of:
1. **j-PPUT, j-FAR, j-CPR** all reject the null at the family-corrected α level (per § 9; 3 inferential tests),
2. **j-RR ≤ p_0** (point check guardrail, no α correction; per § 5.4),
3. **Rollbackable(j) = 1** (verified by an explicit rollback sub-eval in § 6 E2).

H5 is a deterministic AND of (3 inferential rejections) + (1 point check) + (1 rollback verification). Reaching Gate H = at least 1 artifact passes all five conditions. H5 itself is not a hypothesis test; no separate p-value.

## 6. execution plan (30 wall-clock days, pre-committed)


---

# experiments/minif2f_v4/src/cost_aggregator.rs (B2, NEW)

```rust
// PPUT-CCL Phase B B2 — C_i full-cost aggregator.
//
// Spec:
//   handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md § B2
//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md § 5 (cost definition)
//   PREREG § 3 anti-Goodhart conformance #8 (test_failed_branches_in_total_cost)
//
// Invariant being enforced:
//   total_run_token_count = Σ over EVERY proposal in the run of
//       (api_prompt_tokens + api_completion_tokens + tool_stdout_tokens)
//   — including failed parses, vetoed appends, rejected OMEGA claims, step
//   rejects, and any other tx that incurred an LLM call. Counting ONLY the
//   winning branch is the canonical Goodhart attack against PPUT (cheap
//   golden path achieved by burning many quietly-discarded branches), so
//   this aggregator is the ground truth that the conformance test gates.
//
// Token-counter source-of-truth (plan B2 open Q1):
//   - prompt / completion: post-hoc API-reported counts (extended through
//     drivers/llm_http.rs::GenerateResponse this same B2). Accurate.
//   - tool stdout: chars / 4 approximation (plan B2 open Q2 default; the
//     PPUT C_i is for accounting, not budgeting, so precision is not load-
//     bearing — the conformance battery is what enforces honesty).

/// Per-(run_id, problem_id) running token + branch totals.
///
/// Construct one per run, feed every LLM call + every tool-stdout emission,
/// then read totals at run end and stamp them onto the emitted jsonl row.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RunCostAccumulator {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    /// Tool stdout tokens (chars/4 heuristic).
    pub tool_tokens: u64,
    /// Every LLM call that returned a parsed proposal — winning OR losing.
    pub proposal_count: u32,
    /// Subset of proposal_count whose tx did not produce a verified accept.
    pub failed_branch_count: u32,
}

impl RunCostAccumulator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record one LLM call's API-reported token usage.
    /// Call this AFTER `client.generate(...)` returns Ok — both for winning
    /// proposals and for ones that will be rejected/vetoed/parse-failed
    /// downstream. The call already happened; the tokens already cost.
    pub fn record_llm_call(&mut self, prompt_tokens: u32, completion_tokens: u32) {
        self.prompt_tokens += prompt_tokens as u64;
        self.completion_tokens += completion_tokens as u64;
    }

    /// Record bytes of agent-observable tool output (search hits, rejection
    /// error message preserved for next-prompt error feedback, etc.).
    /// chars/4 heuristic per plan B2 open Q2 default.
    pub fn record_tool_stdout(&mut self, stdout: &str) {
        let approx = (stdout.chars().count() as u64 + 3) / 4;
        self.tool_tokens += approx;
    }

    /// Mark one proposal attempt. `accepted = true` for the verified-success
    /// branch (typically called once per run on OMEGA accept); all other
    /// proposals (parse fails, vetoed appends, rejected OMEGAs, step rejects)
    /// pass `false` so they accrete failed_branch_count.
    pub fn record_proposal(&mut self, accepted: bool) {
        self.proposal_count += 1;
        if !accepted {
            self.failed_branch_count += 1;
        }
    }

    /// Convert the most-recent failed proposal into an accepted one. Used at
    /// the OMEGA-accept return path: every tx records as failed at parse time
    /// (since acceptance isn't known yet), then the verified-success branch
    /// flips the last record before returning. Idempotent on an empty
    /// failed bucket — this guards against wiring mistakes that would
    /// otherwise underflow.
    pub fn flip_last_failed_to_accepted(&mut self) {
        if self.failed_branch_count > 0 {
            self.failed_branch_count -= 1;
        }
    }

    /// C_i — total tokens summed across every proposal in the run.
    pub fn total_run_token_count(&self) -> u64 {
        self.prompt_tokens + self.completion_tokens + self.tool_tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// PREREG § 3 anti-Goodhart conformance #8.
    /// Synthesize a run with 5 failed proposals + 1 success and assert the
    /// run total equals the sum of ALL six proposals' tokens, not just the
    /// winner's. This is the Goodhart attack surface PREREG locks down:
    /// counting only the accepted branch makes "many cheap rejects + one
    /// lucky accept" look like a high-PPUT run.
    #[test]
    fn test_failed_branches_counted_in_total_cost() {
        let mut acc = RunCostAccumulator::new();

        // 5 failed proposals: each costs 100 prompt + 50 completion + 20 tool stdout
        // (asymmetric on purpose — catches a mistake that conflates the three buckets).
        for _ in 0..5 {
            acc.record_llm_call(100, 50);
            acc.record_tool_stdout(&"x".repeat(80)); // 80 chars / 4 = 20 tokens
            acc.record_proposal(false);
        }

        // 1 successful proposal: 200 prompt + 100 completion + 0 tool stdout.
        acc.record_llm_call(200, 100);
        acc.record_proposal(true);

        let expected_prompt = 5 * 100 + 200;
        let expected_completion = 5 * 50 + 100;
        let expected_tool = 5 * 20;
        let expected_total = expected_prompt + expected_completion + expected_tool;

        assert_eq!(acc.prompt_tokens, expected_prompt as u64,
            "prompt tokens must include all 6 proposals");
        assert_eq!(acc.completion_tokens, expected_completion as u64,
            "completion tokens must include all 6 proposals");
        assert_eq!(acc.tool_tokens, expected_tool as u64,
            "tool stdout tokens must include all failed branches");
        assert_eq!(acc.total_run_token_count(), expected_total as u64,
            "C_i must sum across ALL 6 proposals — failed branches included");

        assert_eq!(acc.proposal_count, 6);
        assert_eq!(acc.failed_branch_count, 5);
    }

    #[test]
    fn test_empty_accumulator_zero_total() {
        let acc = RunCostAccumulator::new();
        assert_eq!(acc.total_run_token_count(), 0);
        assert_eq!(acc.proposal_count, 0);
        assert_eq!(acc.failed_branch_count, 0);
    }

    #[test]
    fn test_tool_stdout_chars_div_4_approximation() {
        let mut acc = RunCostAccumulator::new();
        // 4 chars → 1 token (exact)
        acc.record_tool_stdout("abcd");
        assert_eq!(acc.tool_tokens, 1);
        // 1 char → ceil(1/4) = 1 token (rounded up; better to over-count
        // than under-count for honest accounting under Goodhart pressure).
        acc.record_tool_stdout("e");
        assert_eq!(acc.tool_tokens, 2);
        // 7 chars → ceil(7/4) = 2 tokens
        acc.record_tool_stdout("1234567");
        assert_eq!(acc.tool_tokens, 4);
    }
}

```

---

# experiments/minif2f_v4/src/wall_clock.rs (B3, NEW)

```rust
// PPUT-CCL Phase B B3 — T_i wall-clock instrumentation.
//
// Spec:
//   handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md § B3
//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md § 5 (time definition)
//   PREREG § 3 anti-Goodhart conformance (test_wall_clock_first_read_to_final_accept)
//
// Definition (PREREG § 5):
//   T_i = end_time − start_time
//     start_time = first read of task statement by any agent in the run
//     end_time   = final ground-truth Lean accept OR external timeout
//
// Why this is its own module rather than `start.elapsed()` inline:
//   1. Excludes evaluator-side preflight (kernel construction, tool mounting,
//      wallet load) that isn't agent-observable work.
//   2. Bracket extends through the FINAL Lean call — including a post-hoc
//      verifier added in B4 that may run AFTER runtime accept. The plan-
//      stated Goodhart concern: a Soft Law mode that fakes runtime accept
//      could exit early before post-hoc verify if the bracket closes at
//      runtime accept. `mark_final_accept` is called by the evaluator AFTER
//      the last verify call returns — making the bracket honest under that
//      attack.
//   3. Testable in isolation: synthetic Instants drive the conformance test
//      without `thread::sleep` flakiness.

use std::time::Instant;

/// Per-run wall-clock bracket.
///
/// Construct one at function entry, call `mark_first_read` at the first agent
/// prompt construction, call `mark_final_accept` after the last Lean call
/// returns, then read `elapsed_ms()` at jsonl emit.
#[derive(Debug, Clone, Copy)]
pub struct RunWallClock {
    first_read: Option<Instant>,
    final_accept: Option<Instant>,
}

impl Default for RunWallClock {
    fn default() -> Self { Self::new() }
}

impl RunWallClock {
    pub fn new() -> Self {
        Self { first_read: None, final_accept: None }
    }

    /// Stamp the bracket open at first agent prompt construction.
    /// Idempotent: subsequent calls are no-ops so the FIRST read wins
    /// regardless of which call site fires it (oneshot vs swarm tx 0).
    pub fn mark_first_read(&mut self) {
        if self.first_read.is_none() {
            self.first_read = Some(Instant::now());
        }
    }

    /// Stamp the bracket closed after the final Lean call.
    /// NOT idempotent — every call updates the close instant, so the LAST
    /// final-Lean call wins (matters when B4 adds post-hoc verify after
    /// runtime accept).
    pub fn mark_final_accept(&mut self) {
        self.final_accept = Some(Instant::now());
    }

    /// T_i in milliseconds.
    /// Returns None if the bracket never opened. If only `first_read` is set
    /// (run aborted before any final accept) returns elapsed-since-first-read,
    /// which is the right thing for the no-OMEGA exit at max_transactions.
    pub fn elapsed_ms(&self) -> Option<u64> {
        match (self.first_read, self.final_accept) {
            (Some(start), Some(end)) => {
                Some(end.saturating_duration_since(start).as_millis() as u64)
            }
            (Some(start), None) => Some(start.elapsed().as_millis() as u64),
            _ => None,
        }
    }

    /// Test-only constructor to inject specific Instants (deterministic
    /// timing for the conformance battery — avoids `thread::sleep` flake).
    #[cfg(test)]
    pub fn from_instants(first_read: Instant, final_accept: Instant) -> Self {
        Self {
            first_read: Some(first_read),
            final_accept: Some(final_accept),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// PREREG § 3 anti-Goodhart conformance:
    /// "synthetic run with 100ms prompt construction + 5s LLM call + 2s Lean
    /// verify → assert wall_time_ms ≥ 7100"
    /// Uses Instant arithmetic so the test runs in microseconds rather than
    /// 7 real seconds — the bracket math is what's being tested.
    #[test]
    fn test_wall_clock_first_read_to_final_accept() {
        let t0 = Instant::now();
        let prompt_construction = Duration::from_millis(100);
        let llm_call = Duration::from_secs(5);
        let lean_verify = Duration::from_secs(2);

        let first_read = t0 + prompt_construction;
        let final_accept = first_read + llm_call + lean_verify;

        let wc = RunWallClock::from_instants(first_read, final_accept);
        let elapsed = wc.elapsed_ms().expect("bracket closed");

        // 5s LLM + 2s Lean = 7000ms minimum. The plan asserts ≥ 7100 to also
        // capture the prompt-construction tail. Since first_read is AFTER
        // prompt construction in our wiring, the bracket is exactly 7000ms
        // here — but the conformance test in this module is on RunWallClock
        // semantics, not on the evaluator's choice of mark point. Loosen to
        // ≥ 7000ms which is what the bracket honestly captures, and assert
        // upper bound of 7100ms to catch over-counting bugs.
        assert!(elapsed >= 7000,
            "wall_time_ms must include LLM + Lean (≥ 7000ms), got {}", elapsed);
        assert!(elapsed <= 7100,
            "wall_time_ms must not double-count (≤ 7100ms), got {}", elapsed);
    }

    #[test]
    fn test_wall_clock_first_read_idempotent() {
        let mut wc = RunWallClock::new();
        wc.mark_first_read();
        let first = wc.first_read.expect("set");
        std::thread::sleep(Duration::from_millis(2));
        wc.mark_first_read(); // should be no-op
        assert_eq!(wc.first_read, Some(first),
            "first_read must be set once; later calls no-op");
    }

    #[test]
    fn test_wall_clock_final_accept_overwrites() {
        // B4 will add a post-hoc verifier called AFTER runtime accept. If
        // both fire mark_final_accept, the LATER instant must win so the
        // bracket includes the trailing verify.
        let mut wc = RunWallClock::new();
        wc.mark_first_read();
        wc.mark_final_accept();
        let first_close = wc.final_accept.expect("set");
        std::thread::sleep(Duration::from_millis(2));
        wc.mark_final_accept();
        let second_close = wc.final_accept.expect("set");
        assert!(second_close > first_close,
            "final_accept must update on each call; second must be later");
    }

    #[test]
    fn test_wall_clock_no_final_accept_uses_now() {
        // No-OMEGA exit path: only first_read is marked. elapsed_ms must
        // return time since first_read so the jsonl row still carries T_i.
        let mut wc = RunWallClock::new();
        wc.mark_first_read();
        std::thread::sleep(Duration::from_millis(5));
        let e = wc.elapsed_ms().expect("first_read set");
        assert!(e >= 5, "elapsed must reflect time since first_read, got {}ms", e);
    }

    #[test]
    fn test_wall_clock_unmarked_returns_none() {
        let wc = RunWallClock::new();
        assert!(wc.elapsed_ms().is_none(),
            "elapsed_ms must be None before first_read is marked");
    }
}

```

---

# experiments/minif2f_v4/src/post_hoc_verifier.rs (B4, NEW)

```rust
// PPUT-CCL Phase B B4 — post-hoc verifier + dual PPUT (runtime vs verified).
//
// Spec:
//   handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md § B4
//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md § 5 (Progress definition)
//   PREREG § 3 anti-Goodhart conformance #7
//     (test_golden_path_requires_ground_truth_acceptance)
//
// Why two PPUT fields:
//   pput_runtime  = Progress_runtime  / (C_i × T_i / 1000)
//   pput_verified = Progress_verified / (C_i × T_i / 1000)   ← North Star
//
//   Progress_runtime  = 1 iff the evaluator's runtime accept gate fired.
//   Progress_verified = 1 iff the post-hoc Lean call returns Ok((true, _))
//                       on the golden_path_payload.
//
//   In current implementation (Phase B), the runtime gate IS the Lean call
//   (no Soft Law mode yet) — so the two agree on every solved run.
//
//   The split exists for Phase C ablation: Soft Law mode fakes runtime
//   acceptance without running Lean. Under Soft Law:
//     pput_runtime  may be > 0 (fake accept inflates Progress_runtime)
//     pput_verified MUST be 0  (Lean reject means Progress_verified = 0)
//   The divergence between the two is the H1 detection mechanism.
//
//   pput_verified is the only North Star metric for H-VPPUT. pput_runtime
//   is emitted only as the divergence signal.

use crate::lean4_oracle::Lean4Oracle;
use crate::jsonl_schema::RunAggregate;

/// Run the post-hoc Lean verification gate on a golden_path_payload.
/// ALWAYS runs the real Lean call. There is NO Soft Law short-circuit
/// here — that's the entire point of the post-hoc layer.
///
/// In current Phase B wiring, the OMEGA-accept code path has already run
/// `verify_omega_detailed` on the same payload and gotten `Ok((true, _))`,
/// so this function is logically guaranteed to return `true` on solved runs.
/// Calling it would just double the Lean cost. Phase C diverges the call
/// site: runtime accept becomes a flag, post-hoc verify stays mandatory.
///
/// Use `compute_progress_verified` instead when the runtime gate already
/// IS the Lean call — it propagates the same truth value without paying
/// for a second Lean process.
pub fn verify_post_hoc(oracle: &Lean4Oracle, golden_path_payload: &str) -> bool {
    matches!(
        oracle.verify_omega_detailed(golden_path_payload),
        Ok((true, _))
    )
}

/// Compute Progress_verified from a (runtime, verified) pair.
///
/// Returns 1 only when both runtime and verified say accept.
/// - Phase B: runtime == verified always (runtime IS Lean). Returns 1 on
///   any accepted run, 0 otherwise.
/// - Phase C Soft Law: runtime can be true while verified is false. The
///   AND collapses to the verified leg, which is the North Star truth.
pub fn compute_progress_verified(runtime_accepted: bool, post_hoc_verified: bool) -> u8 {
    if runtime_accepted && post_hoc_verified { 1 } else { 0 }
}

/// Compute Progress_runtime from the runtime accept signal alone.
/// Inflates under Soft Law when fake-accept fires without Lean.
pub fn compute_progress_runtime(runtime_accepted: bool) -> u8 {
    if runtime_accepted { 1 } else { 0 }
}

/// Wrap RunAggregate::compute_pput_verified for callers in evaluator that
/// only have (progress, c_i, t_i_ms). Same math, single source of truth.
pub fn compute_pput(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
    RunAggregate::compute_pput_verified(progress, c_i, t_i_ms)
}

/// 10^6 × pput. Display unit per PREREG § 5.
pub fn compute_pput_m(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
    RunAggregate::compute_pput_m_verified(progress, c_i, t_i_ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// PREREG § 3 anti-Goodhart conformance #7
    /// (test_golden_path_requires_ground_truth_acceptance):
    /// "synthesize a run that records runtime accept + Lean reject →
    ///  assert progress = 0, pput_verified = 0.0"
    ///
    /// This is the H1 detection gate that makes Soft Law impossible to
    /// hide behind. Without this test failing on a runtime/verified
    /// disagreement, an attacker could inflate North Star PPUT by faking
    /// runtime accepts without paying real Lean verification cost.
    #[test]
    fn test_pput_verified_zero_when_lean_rejects() {
        // Soft Law-style scenario: runtime gate fired, but post-hoc Lean
        // rejected the same payload. The North Star MUST collapse the
        // run's progress to 0.
        let runtime_accepted = true;
        let post_hoc_verified = false;
        let c_i: u64 = 5_000;       // tokens
        let t_i_ms: u64 = 30_000;   // 30 seconds wall

        let progress_runtime = compute_progress_runtime(runtime_accepted);
        let progress_verified =
            compute_progress_verified(runtime_accepted, post_hoc_verified);

        let pput_runtime = compute_pput(progress_runtime, c_i, t_i_ms);
        let pput_verified = compute_pput(progress_verified, c_i, t_i_ms);
        let pput_m_verified = compute_pput_m(progress_verified, c_i, t_i_ms);

        assert_eq!(progress_runtime, 1u8,
            "runtime gate fired → progress_runtime = 1");
        assert_eq!(progress_verified, 0u8,
            "Lean rejected → progress_verified MUST be 0 (North Star truth)");
        assert!(pput_runtime > 0.0,
            "pput_runtime inflates under runtime accept (Soft Law signal)");
        assert_eq!(pput_verified, 0.0,
            "pput_verified MUST be 0 when Lean rejects — North Star Goodhart shield");
        assert_eq!(pput_m_verified, 0.0,
            "pput_m_verified must collapse with pput_verified");

        // Sanity: divergence is detectable. pput_runtime - pput_verified > 0
        // is the H1 signal Phase C scans for.
        assert!(pput_runtime - pput_verified > 0.0,
            "(pput_runtime - pput_verified) > 0 ⟺ Soft Law divergence detected");
    }

    #[test]
    fn test_pput_verified_matches_runtime_when_both_accept() {
        // Phase B reality: runtime IS Lean, so on any solved run the two
        // metrics MUST agree. This test guards against accidentally
        // diverging them in B4 wiring (e.g., a typo that AND'ed the wrong
        // booleans and made pput_verified always 0).
        let c_i: u64 = 5_000;
        let t_i_ms: u64 = 30_000;

        let progress_runtime = compute_progress_runtime(true);
        let progress_verified = compute_progress_verified(true, true);

        assert_eq!(progress_runtime, progress_verified,
            "Phase B: runtime == verified on solved runs");
        assert_eq!(
            compute_pput(progress_runtime, c_i, t_i_ms),
            compute_pput(progress_verified, c_i, t_i_ms),
            "pput fields must agree when runtime == verified");
    }

    #[test]
    fn test_no_runtime_accept_zeros_both_pput() {
        // No-OMEGA exit: neither runtime nor verified fired.
        let c_i: u64 = 50_000;
        let t_i_ms: u64 = 600_000;

        let progress_runtime = compute_progress_runtime(false);
        let progress_verified = compute_progress_verified(false, false);

        assert_eq!(compute_pput(progress_runtime, c_i, t_i_ms), 0.0);
        assert_eq!(compute_pput(progress_verified, c_i, t_i_ms), 0.0);
    }

    #[test]
    fn test_post_hoc_verified_without_runtime_still_zero_progress() {
        // Defensive: a post-hoc verifier that says "yes" but runtime never
        // fired is a wiring bug, not an honest progress signal. Progress
        // is gated on BOTH runtime initiation AND verified result, so this
        // pathological case must clamp to 0.
        assert_eq!(compute_progress_verified(false, true), 0u8,
            "verified without runtime accept = wiring bug, must clamp to 0");
    }
}

```

---

# Diff: evaluator.rs + lib.rs + drivers/llm_http.rs (B2/B3/B4 wiring)

```diff
diff --git a/experiments/minif2f_v4/src/bin/evaluator.rs b/experiments/minif2f_v4/src/bin/evaluator.rs
index c1e240a..93d720d 100644
--- a/experiments/minif2f_v4/src/bin/evaluator.rs
+++ b/experiments/minif2f_v4/src/bin/evaluator.rs
@@ -8,6 +8,11 @@
 // Constitutional basis: Art. I.1 (boolean predicate), Art. I.2 (statistical signal = PPUT)
 
 use minif2f_v4::lean4_oracle::{Lean4Oracle, PartialVerdict, derive_lean_path, load_problem};
+use minif2f_v4::cost_aggregator::RunCostAccumulator;
+use minif2f_v4::wall_clock::RunWallClock;
+use minif2f_v4::post_hoc_verifier::{
+    compute_progress_runtime, compute_progress_verified, compute_pput, compute_pput_m,
+};
 use turingosv4::bus::{BusConfig, BusResult, TuringBus};
 use turingosv4::sdk::error_abstraction::{classify_lean_error, classify_parse_error, CLASSIFIER_VERSION};
 use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
@@ -71,6 +76,31 @@ struct PputResult {
     gp_path: Option<String>,
     #[serde(skip_serializing_if = "Option::is_none")]
     gp_proof_file: Option<String>,
+    // PPUT-CCL Phase B B2 (cost aggregator): full-run token count C_i =
+    // Σ over all proposals (winning + failed) of api prompt + completion +
+    // tool stdout. None on legacy/control binaries that pre-date B2.
+    #[serde(skip_serializing_if = "Option::is_none")]
+    total_run_token_count: Option<u64>,
+    #[serde(skip_serializing_if = "Option::is_none")]
+    failed_branch_count: Option<u32>,
+    // PPUT-CCL Phase B B3 (wall-clock): T_i = first agent prompt construction
+    // → final Lean call returns. Excludes evaluator preflight (kernel ctor,
+    // tool mounting). Distinct from legacy `time_secs` which brackets from
+    // function entry.
+    #[serde(skip_serializing_if = "Option::is_none")]
+    total_wall_time_ms: Option<u64>,
+    // PPUT-CCL Phase B B4 (dual PPUT): runtime accept vs Lean post-hoc
+    // verified. Field names align with B1 RunAggregate v2 schema. In Phase B,
+    // runtime IS Lean so the two agree; Phase C Soft Law is the divergence
+    // call site that makes pput_runtime - pput_verified > 0 the H1 signal.
+    #[serde(skip_serializing_if = "Option::is_none")]
+    verified: Option<bool>,
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pput_runtime: Option<f64>,
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pput_verified: Option<f64>,
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pput_m_verified: Option<f64>,
 }
 
 #[tokio::main]
@@ -173,6 +203,8 @@ async fn run_oneshot(
     lean_path: &str, proxy_url: &str, model: &str,
 ) -> PputResult {
     let start = Instant::now();
+    let mut acc = RunCostAccumulator::new();
+    let mut wc = RunWallClock::new();
 
     let oracle = Lean4Oracle::new(
         problem_statement.to_string(), theorem_name.to_string(), lean_path.to_string(),
@@ -197,16 +229,31 @@ async fn run_oneshot(
         max_tokens: Some(max_toks),
     };
 
+    // PPUT-CCL B3: open the wall-clock bracket at first agent prompt construction
+    // (just before the API call goes out). Excludes prior preflight cost.
+    wc.mark_first_read();
     match client.generate(&request).await {
         Ok(response) => {
+            acc.record_llm_call(response.prompt_tokens, response.completion_tokens);
+            acc.record_proposal(false);
             // Rule 22 v2 clause 4: reject markdown fences
             if response.content.contains("```") {
+                wc.mark_final_accept();
                 return make_pput(problem_file, "oneshot", model, false, start, 0, 0, 1,
-                                 None, None, None, None, None);
+                                 None, None, None, None, None,
+                                 Some(acc.total_run_token_count()),
+                                 Some(acc.failed_branch_count),
+                                 wc.elapsed_ms());
             }
 
-            match oracle.verify_omega(&response.content) {
+            let verdict = oracle.verify_omega(&response.content);
+            // B3: close the bracket AFTER the Lean call returns, regardless of
+            // verdict. Soft Law mode (Phase C) cannot escape the verify-time
+            // accounting by short-circuiting on runtime accept.
+            wc.mark_final_accept();
+            match verdict {
                 Ok(true) => {
+                    acc.flip_last_failed_to_accepted();
                     let gp_tokens = response.completion_tokens as u64;
                     let preview: String = response.content.chars().take(500).collect();
                     info!(">>> OMEGA ACCEPTED <<< (path=alone, payload[0..500]={:?})", preview);
@@ -216,16 +263,25 @@ async fn run_oneshot(
                     );
                     make_pput(problem_file, "oneshot", model, true, start, gp_tokens, 1, 1,
                               None, None, Some(response.content.clone()),
-                              Some("alone".to_string()), proof_file)
+                              Some("alone".to_string()), proof_file,
+                              Some(acc.total_run_token_count()),
+                              Some(acc.failed_branch_count),
+                              wc.elapsed_ms())
                 }
                 Ok(false) => {
                     make_pput(problem_file, "oneshot", model, false, start, 0, 0, 1,
-                              None, None, None, None, None)
+                              None, None, None, None, None,
+                              Some(acc.total_run_token_count()),
+                              Some(acc.failed_branch_count),
+                              wc.elapsed_ms())
                 }
                 Err(e) => {
                     warn!("Oracle error: {}", e);
                     make_pput(problem_file, "oneshot", model, false, start, 0, 0, 1,
-                              None, None, None, None, None)
+                              None, None, None, None, None,
+                              Some(acc.total_run_token_count()),
+                              Some(acc.failed_branch_count),
+                              wc.elapsed_ms())
                 }
             }
         }
@@ -371,6 +427,13 @@ async fn run_swarm(
     let mut omega_attempts: u32 = 0;
     let mut zero_ticks_run: u32 = 0;
     let mut zero_tick_warned = false;
+    // PPUT-CCL B2: full-run cost C_i — every LLM call + tool stdout summed
+    // across all proposals (winning + failed branches). Read at terminal
+    // make_pput sites and stamped on the emitted jsonl row.
+    let mut acc = RunCostAccumulator::new();
+    // PPUT-CCL B3: full-run wall-clock T_i — first agent prompt → final Lean
+    // call. Opened on first tx's prompt build, closed before each return.
+    let mut wc = RunWallClock::new();
     // Art. III.2: per-agent search result cache (bounded), fed into next prompt.
     let mut search_cache: HashMap<String, Vec<String>> = HashMap::new();
     // F-2026-04-19-05: cap searches per agent; beyond cap we remove `search`
@@ -497,6 +560,9 @@ async fn run_swarm(
             &chain, &skill, &snap.market_ticker, &errors, &hits_ref,
             snap.get_balance(agent_id), tools_desc, &team_board,
         );
+        // PPUT-CCL B3: stamp wall-clock open at first agent prompt build.
+        // Idempotent — only the first tx's call sticks.
+        wc.mark_first_read();
 
         // Model-aware max_tokens (same rule as oneshot branch).
         let max_toks = if model.contains("chat") { 8000 } else { 16000 };
@@ -517,6 +583,10 @@ async fn run_swarm(
 
         match client.generate(&request).await {
             Ok(response) => {
+                acc.record_llm_call(response.prompt_tokens, response.completion_tokens);
+                // PPUT-CCL B2: every parsed proposal default-records as failed.
+                // OMEGA-accept return paths flip the last record before returning.
+                acc.record_proposal(false);
                 match parse_agent_output(&response.content) {
                     Ok(action) => match action.tool.as_str() {
                         "append" => {
@@ -600,8 +670,14 @@ async fn run_swarm(
                                     }
                                     _ => (payload.clone(), "alone", r_alone.clone()),
                                 };
+                                // PPUT-CCL B3: close bracket AFTER both Lean verify paths return.
+                                // Soft Law (Phase C) cannot exit ahead of verify-time accounting.
+                                wc.mark_final_accept();
                                 match r_final {
                                     Ok((true, _)) => {
+                                        // PPUT-CCL B2: this proposal verified — flip the failed
+                                        // record made at parse time into the run's accepted slot.
+                                        acc.flip_last_failed_to_accepted();
                                         // Phase 0 (C-039): persist the winning artifact so external
                                         // verifiers can re-run lean from disk alone.
                                         let preview: String = full_proof.chars().take(500).collect();
@@ -665,12 +741,18 @@ async fn run_swarm(
                                                         Some(tool_dist), upr,
                                                         Some(full_proof.clone()),
                                                         Some(path_choice.to_string()),
-                                                        proof_file);
+                                                        proof_file,
+                                                        Some(acc.total_run_token_count()),
+                                                        Some(acc.failed_branch_count),
+                                                        wc.elapsed_ms());
                                     }
                                     Ok((false, err_detail)) => {
                                         // Step-B v3: classify + record class label (C-022 shield).
                                         let class = classify_lean_error(&err_detail);
                                         bus.record_rejection(agent_id, class.label());
+                                        // PPUT-CCL B2: rejection error feeds back into next prompt's
+                                        // recent_rejections — count those bytes against C_i.
+                                        acc.record_tool_stdout(&err_detail);
                                         let preview: String = payload.chars().take(300).collect();
                                         warn!("[tx {}] OMEGA rejected ({}). payload[0..300]={:?}", tx, class.label(), preview);
                                     }
@@ -758,6 +840,9 @@ async fn run_swarm(
                                     let trimmed: Vec<String> = hits.iter().take(5)
                                         .map(|p| p.rsplit('/').next().unwrap_or(p).to_string())
                                         .collect();
+                                    // PPUT-CCL B2: search hits feed `hits_ref` into next prompt —
+                                    // count the cached bytes against C_i.
+                                    acc.record_tool_stdout(&trimmed.join("\n"));
                                     info!("[tx {}] {} search({:?}) → {} hits: {}",
                                           tx, agent_id, query, hits.len(), trimmed.join(","));
                                     search_cache.insert(agent_id.clone(), trimmed);
@@ -804,8 +889,12 @@ async fn run_swarm(
                                     theorem_name.to_string(),
                                     lean_path.to_string(),
                                 );
-                                match oracle.verify_partial(&prefix) {
+                                let verdict = oracle.verify_partial(&prefix);
+                                // PPUT-CCL B3: close bracket after step-verify returns.
+                                wc.mark_final_accept();
+                                match verdict {
                                     PartialVerdict::Complete => {
+                                        acc.flip_last_failed_to_accepted();
                                         info!(">>> OMEGA ACCEPTED <<< via step (depth={} after this write)",
                                               bus.kernel.tape.time_arrow().len() + 1);
                                         let proof_file = persist_proof_artifact(
@@ -833,7 +922,10 @@ async fn run_swarm(
                                                         Some(tool_dist), upr,
                                                         Some(prefix.clone()),
                                                         Some("per_tactic".to_string()),
-                                                        proof_file);
+                                                        proof_file,
+                                                        Some(acc.total_run_token_count()),
+                                                        Some(acc.failed_branch_count),
+                                                        wc.elapsed_ms());
                                     }
                                     PartialVerdict::PartialOk => {
                                         let parent = bus.kernel.tape.time_arrow().last().cloned();
@@ -855,6 +947,8 @@ async fn run_swarm(
                                     PartialVerdict::Reject(reason) => {
                                         let class = classify_lean_error(&reason);
                                         bus.record_rejection(agent_id, class.label());
+                                        // PPUT-CCL B2: step rejection reason flows into next prompt.
+                                        acc.record_tool_stdout(&reason);
                                         *tool_dist.entry("step_reject".into()).or_insert(0) += 1;
                                         let preview = reason.chars().take(200).collect::<String>();
                                         warn!("[tx {}] step rejected ({}): {}", tx, class.label(), preview);
@@ -871,6 +965,8 @@ async fn run_swarm(
                         // Step-B v3: parse failures feed the class graveyard too.
                         let class = classify_parse_error(&format!("{}", e));
                         bus.record_rejection(agent_id, class.label());
+                        // PPUT-CCL B2: classifier label flows into next prompt's errors.
+                        acc.record_tool_stdout(class.label());
                         warn!("[tx {}] parse: {} ({})", tx, e, class.label());
                     }
                 }
@@ -895,9 +991,14 @@ async fn run_swarm(
         }
     }
     // No OMEGA found → PPUT = 0
+    // B3: close bracket on max-tx exhaustion path.
+    wc.mark_final_accept();
     make_pput(problem_file, &condition, model, false, start, 0, 0,
               max_transactions as u64, Some(tool_dist), upr,
-              None, None, None)
+              None, None, None,
+              Some(acc.total_run_token_count()),
+              Some(acc.failed_branch_count),
+              wc.elapsed_ms())
 }
 
 fn make_pput(
@@ -909,6 +1010,9 @@ fn make_pput(
     gp_payload: Option<String>,
     gp_path: Option<String>,
     gp_proof_file: Option<String>,
+    total_run_token_count: Option<u64>,
+    failed_branch_count: Option<u32>,
+    total_wall_time_ms: Option<u64>,
 ) -> PputResult {
     let elapsed = start.elapsed().as_secs_f64();
     let pput = if has_gp && elapsed > 0.0 { 100.0 / elapsed } else { 0.0 };
@@ -917,6 +1021,32 @@ fn make_pput(
     let classifier_version = std::env::var("CLASSIFIER_VERSION").ok();
     let boltzmann_seed = std::env::var("BOLTZMANN_SEED")
         .ok().and_then(|s| s.parse::<u64>().ok());
+
+    // PPUT-CCL Phase B B4: derive dual PPUT from C_i + T_i + has_gp.
+    // Phase B truth: runtime IS Lean, so verified == has_gp on solved runs.
+    // Phase C Soft Law will set has_gp=true while a separate verify_post_hoc
+    // call returns false; that's the planned divergence point.
+    let (verified, pput_runtime, pput_verified, pput_m_verified) =
+        match (total_run_token_count, total_wall_time_ms) {
+            (Some(c_i), Some(t_i)) => {
+                let runtime_accepted = has_gp;
+                let post_hoc_verified = has_gp; // Phase B: runtime IS Lean
+                let progress_runtime = compute_progress_runtime(runtime_accepted);
+                let progress_verified =
+                    compute_progress_verified(runtime_accepted, post_hoc_verified);
+                (
+                    Some(post_hoc_verified),
+                    Some(compute_pput(progress_runtime, c_i, t_i)),
+                    Some(compute_pput(progress_verified, c_i, t_i)),
+                    Some(compute_pput_m(progress_verified, c_i, t_i)),
+                )
+            }
+            // Missing C_i or T_i → can't compute dual PPUT honestly; emit None
+            // rather than fabricate a zero that would be confused with a real
+            // unsolved run.
+            _ => (None, None, None, None),
+        };
+
     PputResult {
         problem: problem.to_string(),
         condition: condition.to_string(),
@@ -935,6 +1065,13 @@ fn make_pput(
         gp_payload,
         gp_path,
         gp_proof_file,
+        total_run_token_count,
+        failed_branch_count,
+        total_wall_time_ms,
+        verified,
+        pput_runtime,
+        pput_verified,
+        pput_m_verified,
     }
 }
 
diff --git a/experiments/minif2f_v4/src/lib.rs b/experiments/minif2f_v4/src/lib.rs
index 537321b..6730060 100644
--- a/experiments/minif2f_v4/src/lib.rs
+++ b/experiments/minif2f_v4/src/lib.rs
@@ -1,2 +1,5 @@
 pub mod lean4_oracle;
 pub mod jsonl_schema;
+pub mod cost_aggregator;
+pub mod wall_clock;
+pub mod post_hoc_verifier;
diff --git a/src/drivers/llm_http.rs b/src/drivers/llm_http.rs
index 5c2cc48..1c2f1db 100644
--- a/src/drivers/llm_http.rs
+++ b/src/drivers/llm_http.rs
@@ -31,6 +31,10 @@ pub struct Message {
 pub struct GenerateResponse {
     pub content: String,
     pub completion_tokens: u32,
+    /// API-reported prompt tokens. Falls back to 0 if `usage.prompt_tokens` is
+    /// absent in the proxy response (older proxies). Surfaced for PPUT-CCL
+    /// Phase B C_i accounting (post-hoc, not estimation — plan B2 default).
+    pub prompt_tokens: u32,
     pub model: String,
 }
 
@@ -127,6 +131,9 @@ impl ResilientLLMClient {
                     let tokens = body["usage"]["completion_tokens"]
                         .as_u64()
                         .unwrap_or(0) as u32;
+                    let prompt_tokens = body["usage"]["prompt_tokens"]
+                        .as_u64()
+                        .unwrap_or(0) as u32;
                     let model = body["model"]
                         .as_str()
                         .unwrap_or(&request.model)
@@ -135,6 +142,7 @@ impl ResilientLLMClient {
                     return Ok(GenerateResponse {
                         content,
                         completion_tokens: tokens,
+                        prompt_tokens,
                         model,
                     });
                 }

```

---

Now give your INDEPENDENT mid-term audit. Cite §/line for every finding.

codex
I’ll audit the landed code against the prereg text and the Phase B plan, not just the pasted diff. First I’m going to locate the applicable `AGENTS.md` guidance and then read the exact line-numbered implementation sites.
exec
/bin/bash -lc 'git status --short' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
 M experiments/minif2f_v4/src/bin/evaluator.rs
 M experiments/minif2f_v4/src/lib.rs
 M rules/enforcement.log
 M src/drivers/llm_http.rs
?? experiments/minif2f_v4/src/cost_aggregator.rs
?? experiments/minif2f_v4/src/post_hoc_verifier.rs
?? experiments/minif2f_v4/src/wall_clock.rs
?? handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md
?? handover/audits/run_codex_pput_ccl_b2_b4_audit.sh
?? handover/audits/run_gemini_pput_ccl_b2_b4_audit.py

exec
/bin/bash -lc "pwd && rg --files -g 'AGENTS.md' -g '"'!target'"'" in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
/home/zephryj/projects/turingosv4

codex
No `AGENTS.md` appeared through `rg --files`, so I’m checking with `find` before reviewing scoped code. The worktree has the expected B2-B4 local changes plus a few audit artifacts; I won’t touch them.
exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// MiniF2F v4 Evaluator — oneshot and swarm modes
     2	//
     3	// Sole optimization metric: PPUT (Progress Per Unit Time)
     4	//   Progress = 100% if Golden Path exists (OMEGA reached), 0% otherwise
     5	//   PPUT = 100% / time_to_omega (seconds)
     6	//   No GP → PPUT = 0 → problem not worth attacking in current iteration
     7	//
     8	// Constitutional basis: Art. I.1 (boolean predicate), Art. I.2 (statistical signal = PPUT)
     9	
    10	use minif2f_v4::lean4_oracle::{Lean4Oracle, PartialVerdict, derive_lean_path, load_problem};
    11	use minif2f_v4::cost_aggregator::RunCostAccumulator;
    12	use minif2f_v4::wall_clock::RunWallClock;
    13	use minif2f_v4::post_hoc_verifier::{
    14	    compute_progress_runtime, compute_progress_verified, compute_pput, compute_pput_m,
    15	};
    16	use turingosv4::bus::{BusConfig, BusResult, TuringBus};
    17	use turingosv4::sdk::error_abstraction::{classify_lean_error, classify_parse_error, CLASSIFIER_VERSION};
    18	use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
    19	use turingosv4::kernel::Kernel;
    20	use turingosv4::sdk::actor::{BoltzmannParams, boltzmann_select_parent};
    21	use turingosv4::sdk::prompt::build_agent_prompt;
    22	use turingosv4::sdk::protocol::parse_agent_output;
    23	use turingosv4::sdk::tools::wallet::WalletTool;
    24	use turingosv4::sdk::tools::search::SearchTool;
    25	use turingosv4::sdk::tools::librarian::LibrarianTool;
    26	
    27	use std::collections::{HashMap, HashSet};
    28	use std::hash::{Hash, Hasher};
    29	use std::path::PathBuf;
    30	use std::time::Instant;
    31	use log::{info, warn, error};
    32	use rand::SeedableRng;
    33	use rand::rngs::StdRng;
    34	
    35	const DEFAULT_BOLTZMANN_SEED: u64 = 74677;  // same as sample seed (BTC/USD external)
    36	
    37	const DEFAULT_MINIF2F_DIR: &str = "/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4";
    38	
    39	/// PPUT result for a single problem — the only output that matters.
    40	#[derive(Debug, serde::Serialize)]
    41	struct PputResult {
    42	    problem: String,
    43	    condition: String,
    44	    model: String,
    45	    has_golden_path: bool,         // true = OMEGA reached
    46	    time_secs: f64,                // wall time elapsed
    47	    pput: f64,                     // 100/time if GP, 0 otherwise
    48	    gp_token_count: u64,           // token count of golden path (0 if no GP)
    49	    gp_node_count: usize,          // nodes on golden path (0 if no GP)
    50	    tx_count: u64,                 // total transactions attempted
    51	    // C-012 provenance: stamp per-row commit SHA + classifier version + RNG seed.
    52	    // All Optional; serialize-skip when None (backward compat with v3.1/v3.2 artifacts).
    53	    #[serde(skip_serializing_if = "Option::is_none")]
    54	    build_sha: Option<String>,
    55	    #[serde(skip_serializing_if = "Option::is_none")]
    56	    classifier_version: Option<String>,
    57	    #[serde(skip_serializing_if = "Option::is_none")]
    58	    boltzmann_seed: Option<u64>,
    59	    // C-036 harness telemetry: bypass-detection signals for multi-agent runs.
    60	    // tool_dist: counts per tool ({complete, append, invest, parse_fail, llm_err}).
    61	    //   complete=N append=0 ⇒ tape-bypass (Art. II.1 broadcast unused).
    62	    // unique_payload_ratio: distinct OMEGA payloads / total OMEGA attempts.
    63	    //   <0.30 ⇒ catastrophic agent correlation (F-2026-04-18-01).
    64	    #[serde(skip_serializing_if = "Option::is_none")]
    65	    tool_dist: Option<HashMap<String, u32>>,
    66	    #[serde(skip_serializing_if = "Option::is_none")]
    67	    unique_payload_ratio: Option<f64>,
    68	    // Phase 0 (C-039 candidate): persisted full proof + path so external verifiers can
    69	    // re-run `lean --stdin` from disk artifacts alone, without trusting in-memory runtime.
    70	    // gp_payload = the exact text fed to oracle.verify_omega_detailed at OMEGA accept.
    71	    // gp_path = "alone" (payload self-contained) or "tape+payload" (Art. IV dual-path 2).
    72	    // gp_proof_file = relative path to the standalone .lean archive (problem + proof).
    73	    #[serde(skip_serializing_if = "Option::is_none")]
    74	    gp_payload: Option<String>,
    75	    #[serde(skip_serializing_if = "Option::is_none")]
    76	    gp_path: Option<String>,
    77	    #[serde(skip_serializing_if = "Option::is_none")]
    78	    gp_proof_file: Option<String>,
    79	    // PPUT-CCL Phase B B2 (cost aggregator): full-run token count C_i =
    80	    // Σ over all proposals (winning + failed) of api prompt + completion +
    81	    // tool stdout. None on legacy/control binaries that pre-date B2.
    82	    #[serde(skip_serializing_if = "Option::is_none")]
    83	    total_run_token_count: Option<u64>,
    84	    #[serde(skip_serializing_if = "Option::is_none")]
    85	    failed_branch_count: Option<u32>,
    86	    // PPUT-CCL Phase B B3 (wall-clock): T_i = first agent prompt construction
    87	    // → final Lean call returns. Excludes evaluator preflight (kernel ctor,
    88	    // tool mounting). Distinct from legacy `time_secs` which brackets from
    89	    // function entry.
    90	    #[serde(skip_serializing_if = "Option::is_none")]
    91	    total_wall_time_ms: Option<u64>,
    92	    // PPUT-CCL Phase B B4 (dual PPUT): runtime accept vs Lean post-hoc
    93	    // verified. Field names align with B1 RunAggregate v2 schema. In Phase B,
    94	    // runtime IS Lean so the two agree; Phase C Soft Law is the divergence
    95	    // call site that makes pput_runtime - pput_verified > 0 the H1 signal.
    96	    #[serde(skip_serializing_if = "Option::is_none")]
    97	    verified: Option<bool>,
    98	    #[serde(skip_serializing_if = "Option::is_none")]
    99	    pput_runtime: Option<f64>,
   100	    #[serde(skip_serializing_if = "Option::is_none")]
   101	    pput_verified: Option<f64>,
   102	    #[serde(skip_serializing_if = "Option::is_none")]
   103	    pput_m_verified: Option<f64>,
   104	}
   105	
   106	#[tokio::main]
   107	async fn main() {
   108	    env_logger::init();
   109	    // Step-B v3 treatment binary: stamp classifier version in every emitted PputResult.
   110	    // Control binary (main branch) has no such set_var → classifier_version serializes as None.
   111	    // This makes it impossible to mistake one binary for the other in post-hoc analysis.
   112	    std::env::set_var("CLASSIFIER_VERSION", CLASSIFIER_VERSION);
   113	
   114	    let args: Vec<String> = std::env::args().collect();
   115	    if args.len() < 2 {
   116	        eprintln!("Usage: evaluator <problem_file.lean>");
   117	        eprintln!("  CONDITION env: oneshot|n1|n3 (default: oneshot)");
   118	        eprintln!("  MINIF2F_DIR, LLM_PROXY_URL, ACTIVE_MODEL env vars");
   119	        std::process::exit(1);
   120	    }
   121	
   122	    let problem_file = &args[1];
   123	    let condition = std::env::var("CONDITION").unwrap_or_else(|_| "oneshot".into());
   124	    let minif2f_dir = std::env::var("MINIF2F_DIR").unwrap_or_else(|_| DEFAULT_MINIF2F_DIR.into());
   125	    let proxy_url = std::env::var("LLM_PROXY_URL").unwrap_or_else(|_| "http://localhost:8080".into());
   126	    let model = std::env::var("ACTIVE_MODEL").unwrap_or_else(|_| "deepseek-reasoner".into());
   127	
   128	    // Resolve problem path
   129	    let problem_path = resolve_problem_path(problem_file, &minif2f_dir);
   130	    let (problem_statement, theorem_name) = match load_problem(&problem_path) {
   131	        Ok(v) => v,
   132	        Err(e) => { eprintln!("Failed to load: {}", e); std::process::exit(1); }
   133	    };
   134	
   135	    let lean_path = derive_lean_path(&minif2f_dir);
   136	    info!("Problem: {} | Condition: {} | Model: {}", problem_file, condition, model);
   137	
   138	    let result = match condition.as_str() {
   139	        "oneshot" => {
   140	            run_oneshot(problem_file, &problem_statement, &theorem_name,
   141	                       &lean_path, &proxy_url, &model).await
   142	        }
   143	        // Generic nN: parse any "n<digits>" → run_swarm with N agents.
   144	        // Supports N-scaling experiment (percolation curve mapping).
   145	        c if c.starts_with('n') && c[1..].parse::<usize>().is_ok() => {
   146	            let n: usize = c[1..].parse().unwrap();
   147	            run_swarm(problem_file, &problem_statement, &theorem_name,
   148	                     &lean_path, &proxy_url, &model, n).await
   149	        }
   150	        "hybrid_v1" => {
   151	            // RCA (2026-04-14): agent_0's swarm prompt (tape/market/errors/tools)
   152	            // underperforms oneshot's bare prompt on problems DeepSeek can 1-shot.
   153	            // This condition uses oneshot prompt first; only if that fails does
   154	            // the swarm fire. Separately named per C-032/C-033 causal attribution.
   155	            let start = Instant::now();
   156	            let r = run_oneshot(problem_file, &problem_statement, &theorem_name,
   157	                               &lean_path, &proxy_url, &model).await;
   158	            if r.has_golden_path {
   159	                PputResult { condition: "hybrid_v1".into(), ..r }
   160	            } else {
   161	                let r2 = run_swarm(problem_file, &problem_statement, &theorem_name,
   162	                                  &lean_path, &proxy_url, &model, 3).await;
   163	                let elapsed = start.elapsed().as_secs_f64();
   164	                PputResult {
   165	                    condition: "hybrid_v1".into(),
   166	                    time_secs: elapsed,
   167	                    pput: if r2.has_golden_path { 100.0 / elapsed } else { 0.0 },
   168	                    tx_count: 1 + r2.tx_count,
   169	                    ..r2
   170	                }
   171	            }
   172	        }
   173	        other => { eprintln!("Unknown condition: {}", other); std::process::exit(1); }
   174	    };
   175	
   176	    // Output PPUT result as JSON (machine-readable for batch runner)
   177	    let json = serde_json::to_string(&result).unwrap();
   178	    println!("PPUT_RESULT:{}", json);
   179	
   180	    if result.has_golden_path {
   181	        info!("PPUT = {:.2}%/s (GP: {} nodes, {} tokens, {:.1}s)",
   182	              result.pput, result.gp_node_count, result.gp_token_count, result.time_secs);
   183	    } else {
   184	        info!("PPUT = 0 (no golden path in {:.1}s, {} tx)", result.time_secs, result.tx_count);
   185	    }
   186	}
   187	
   188	fn resolve_problem_path(problem_file: &str, minif2f_dir: &str) -> String {
   189	    if PathBuf::from(problem_file).exists() {
   190	        return problem_file.to_string();
   191	    }
   192	    let test_path = format!("{}/MiniF2F/Test/{}", minif2f_dir, problem_file);
   193	    if PathBuf::from(&test_path).exists() { return test_path; }
   194	    let valid_path = format!("{}/MiniF2F/Valid/{}", minif2f_dir, problem_file);
   195	    if PathBuf::from(&valid_path).exists() { return valid_path; }
   196	    eprintln!("Problem file not found: {}", problem_file);
   197	    std::process::exit(1);
   198	}
   199	
   200	/// Oneshot: single LLM call → verify → PPUT.
   201	async fn run_oneshot(
   202	    problem_file: &str, problem_statement: &str, theorem_name: &str,
   203	    lean_path: &str, proxy_url: &str, model: &str,
   204	) -> PputResult {
   205	    let start = Instant::now();
   206	    let mut acc = RunCostAccumulator::new();
   207	    let mut wc = RunWallClock::new();
   208	
   209	    let oracle = Lean4Oracle::new(
   210	        problem_statement.to_string(), theorem_name.to_string(), lean_path.to_string(),
   211	    );
   212	
   213	    // R-22 v2 clause 4 stays reject-only; the prompt must prevent fences at the source.
   214	    // Chat models (deepseek-chat, 2026-04-22) default to ```lean fences; verifier hard-rejects
   215	    // any response containing ``` so the instruction must be explicit. See F-2026-04-22-08.
   216	    let prompt = format!(
   217	        "Complete the following Lean 4 proof. Output ONLY the tactic proof body as raw Lean \
   218	         tokens. DO NOT wrap in markdown code fences (no ```). No prose, no backticks.\n\n{}",
   219	        problem_statement
   220	    );

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/cost_aggregator.rs | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// PPUT-CCL Phase B B2 — C_i full-cost aggregator.
     2	//
     3	// Spec:
     4	//   handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md § B2
     5	//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md § 5 (cost definition)
     6	//   PREREG § 3 anti-Goodhart conformance #8 (test_failed_branches_in_total_cost)
     7	//
     8	// Invariant being enforced:
     9	//   total_run_token_count = Σ over EVERY proposal in the run of
    10	//       (api_prompt_tokens + api_completion_tokens + tool_stdout_tokens)
    11	//   — including failed parses, vetoed appends, rejected OMEGA claims, step
    12	//   rejects, and any other tx that incurred an LLM call. Counting ONLY the
    13	//   winning branch is the canonical Goodhart attack against PPUT (cheap
    14	//   golden path achieved by burning many quietly-discarded branches), so
    15	//   this aggregator is the ground truth that the conformance test gates.
    16	//
    17	// Token-counter source-of-truth (plan B2 open Q1):
    18	//   - prompt / completion: post-hoc API-reported counts (extended through
    19	//     drivers/llm_http.rs::GenerateResponse this same B2). Accurate.
    20	//   - tool stdout: chars / 4 approximation (plan B2 open Q2 default; the
    21	//     PPUT C_i is for accounting, not budgeting, so precision is not load-
    22	//     bearing — the conformance battery is what enforces honesty).
    23	
    24	/// Per-(run_id, problem_id) running token + branch totals.
    25	///
    26	/// Construct one per run, feed every LLM call + every tool-stdout emission,
    27	/// then read totals at run end and stamp them onto the emitted jsonl row.
    28	#[derive(Debug, Default, Clone, PartialEq, Eq)]
    29	pub struct RunCostAccumulator {
    30	    pub prompt_tokens: u64,
    31	    pub completion_tokens: u64,
    32	    /// Tool stdout tokens (chars/4 heuristic).
    33	    pub tool_tokens: u64,
    34	    /// Every LLM call that returned a parsed proposal — winning OR losing.
    35	    pub proposal_count: u32,
    36	    /// Subset of proposal_count whose tx did not produce a verified accept.
    37	    pub failed_branch_count: u32,
    38	}
    39	
    40	impl RunCostAccumulator {
    41	    pub fn new() -> Self {
    42	        Self::default()
    43	    }
    44	
    45	    /// Record one LLM call's API-reported token usage.
    46	    /// Call this AFTER `client.generate(...)` returns Ok — both for winning
    47	    /// proposals and for ones that will be rejected/vetoed/parse-failed
    48	    /// downstream. The call already happened; the tokens already cost.
    49	    pub fn record_llm_call(&mut self, prompt_tokens: u32, completion_tokens: u32) {
    50	        self.prompt_tokens += prompt_tokens as u64;
    51	        self.completion_tokens += completion_tokens as u64;
    52	    }
    53	
    54	    /// Record bytes of agent-observable tool output (search hits, rejection
    55	    /// error message preserved for next-prompt error feedback, etc.).
    56	    /// chars/4 heuristic per plan B2 open Q2 default.
    57	    pub fn record_tool_stdout(&mut self, stdout: &str) {
    58	        let approx = (stdout.chars().count() as u64 + 3) / 4;
    59	        self.tool_tokens += approx;
    60	    }
    61	
    62	    /// Mark one proposal attempt. `accepted = true` for the verified-success
    63	    /// branch (typically called once per run on OMEGA accept); all other
    64	    /// proposals (parse fails, vetoed appends, rejected OMEGAs, step rejects)
    65	    /// pass `false` so they accrete failed_branch_count.
    66	    pub fn record_proposal(&mut self, accepted: bool) {
    67	        self.proposal_count += 1;
    68	        if !accepted {
    69	            self.failed_branch_count += 1;
    70	        }
    71	    }
    72	
    73	    /// Convert the most-recent failed proposal into an accepted one. Used at
    74	    /// the OMEGA-accept return path: every tx records as failed at parse time
    75	    /// (since acceptance isn't known yet), then the verified-success branch
    76	    /// flips the last record before returning. Idempotent on an empty
    77	    /// failed bucket — this guards against wiring mistakes that would
    78	    /// otherwise underflow.
    79	    pub fn flip_last_failed_to_accepted(&mut self) {
    80	        if self.failed_branch_count > 0 {
    81	            self.failed_branch_count -= 1;
    82	        }
    83	    }
    84	
    85	    /// C_i — total tokens summed across every proposal in the run.
    86	    pub fn total_run_token_count(&self) -> u64 {
    87	        self.prompt_tokens + self.completion_tokens + self.tool_tokens
    88	    }
    89	}
    90	
    91	#[cfg(test)]
    92	mod tests {
    93	    use super::*;
    94	
    95	    /// PREREG § 3 anti-Goodhart conformance #8.
    96	    /// Synthesize a run with 5 failed proposals + 1 success and assert the
    97	    /// run total equals the sum of ALL six proposals' tokens, not just the
    98	    /// winner's. This is the Goodhart attack surface PREREG locks down:
    99	    /// counting only the accepted branch makes "many cheap rejects + one
   100	    /// lucky accept" look like a high-PPUT run.
   101	    #[test]
   102	    fn test_failed_branches_counted_in_total_cost() {
   103	        let mut acc = RunCostAccumulator::new();
   104	
   105	        // 5 failed proposals: each costs 100 prompt + 50 completion + 20 tool stdout
   106	        // (asymmetric on purpose — catches a mistake that conflates the three buckets).
   107	        for _ in 0..5 {
   108	            acc.record_llm_call(100, 50);
   109	            acc.record_tool_stdout(&"x".repeat(80)); // 80 chars / 4 = 20 tokens
   110	            acc.record_proposal(false);
   111	        }
   112	
   113	        // 1 successful proposal: 200 prompt + 100 completion + 0 tool stdout.
   114	        acc.record_llm_call(200, 100);
   115	        acc.record_proposal(true);
   116	
   117	        let expected_prompt = 5 * 100 + 200;
   118	        let expected_completion = 5 * 50 + 100;
   119	        let expected_tool = 5 * 20;
   120	        let expected_total = expected_prompt + expected_completion + expected_tool;
   121	
   122	        assert_eq!(acc.prompt_tokens, expected_prompt as u64,
   123	            "prompt tokens must include all 6 proposals");
   124	        assert_eq!(acc.completion_tokens, expected_completion as u64,
   125	            "completion tokens must include all 6 proposals");
   126	        assert_eq!(acc.tool_tokens, expected_tool as u64,
   127	            "tool stdout tokens must include all failed branches");
   128	        assert_eq!(acc.total_run_token_count(), expected_total as u64,
   129	            "C_i must sum across ALL 6 proposals — failed branches included");
   130	
   131	        assert_eq!(acc.proposal_count, 6);
   132	        assert_eq!(acc.failed_branch_count, 5);
   133	    }
   134	
   135	    #[test]
   136	    fn test_empty_accumulator_zero_total() {
   137	        let acc = RunCostAccumulator::new();
   138	        assert_eq!(acc.total_run_token_count(), 0);
   139	        assert_eq!(acc.proposal_count, 0);
   140	        assert_eq!(acc.failed_branch_count, 0);
   141	    }
   142	
   143	    #[test]
   144	    fn test_tool_stdout_chars_div_4_approximation() {
   145	        let mut acc = RunCostAccumulator::new();
   146	        // 4 chars → 1 token (exact)
   147	        acc.record_tool_stdout("abcd");
   148	        assert_eq!(acc.tool_tokens, 1);
   149	        // 1 char → ceil(1/4) = 1 token (rounded up; better to over-count
   150	        // than under-count for honest accounting under Goodhart pressure).
   151	        acc.record_tool_stdout("e");
   152	        assert_eq!(acc.tool_tokens, 2);
   153	        // 7 chars → ceil(7/4) = 2 tokens
   154	        acc.record_tool_stdout("1234567");
   155	        assert_eq!(acc.tool_tokens, 4);
   156	    }
   157	}

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/wall_clock.rs | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 2ms:
     1	// PPUT-CCL Phase B B3 — T_i wall-clock instrumentation.
     2	//
     3	// Spec:
     4	//   handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md § B3
     5	//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md § 5 (time definition)
     6	//   PREREG § 3 anti-Goodhart conformance (test_wall_clock_first_read_to_final_accept)
     7	//
     8	// Definition (PREREG § 5):
     9	//   T_i = end_time − start_time
    10	//     start_time = first read of task statement by any agent in the run
    11	//     end_time   = final ground-truth Lean accept OR external timeout
    12	//
    13	// Why this is its own module rather than `start.elapsed()` inline:
    14	//   1. Excludes evaluator-side preflight (kernel construction, tool mounting,
    15	//      wallet load) that isn't agent-observable work.
    16	//   2. Bracket extends through the FINAL Lean call — including a post-hoc
    17	//      verifier added in B4 that may run AFTER runtime accept. The plan-
    18	//      stated Goodhart concern: a Soft Law mode that fakes runtime accept
    19	//      could exit early before post-hoc verify if the bracket closes at
    20	//      runtime accept. `mark_final_accept` is called by the evaluator AFTER
    21	//      the last verify call returns — making the bracket honest under that
    22	//      attack.
    23	//   3. Testable in isolation: synthetic Instants drive the conformance test
    24	//      without `thread::sleep` flakiness.
    25	
    26	use std::time::Instant;
    27	
    28	/// Per-run wall-clock bracket.
    29	///
    30	/// Construct one at function entry, call `mark_first_read` at the first agent
    31	/// prompt construction, call `mark_final_accept` after the last Lean call
    32	/// returns, then read `elapsed_ms()` at jsonl emit.
    33	#[derive(Debug, Clone, Copy)]
    34	pub struct RunWallClock {
    35	    first_read: Option<Instant>,
    36	    final_accept: Option<Instant>,
    37	}
    38	
    39	impl Default for RunWallClock {
    40	    fn default() -> Self { Self::new() }
    41	}
    42	
    43	impl RunWallClock {
    44	    pub fn new() -> Self {
    45	        Self { first_read: None, final_accept: None }
    46	    }
    47	
    48	    /// Stamp the bracket open at first agent prompt construction.
    49	    /// Idempotent: subsequent calls are no-ops so the FIRST read wins
    50	    /// regardless of which call site fires it (oneshot vs swarm tx 0).
    51	    pub fn mark_first_read(&mut self) {
    52	        if self.first_read.is_none() {
    53	            self.first_read = Some(Instant::now());
    54	        }
    55	    }
    56	
    57	    /// Stamp the bracket closed after the final Lean call.
    58	    /// NOT idempotent — every call updates the close instant, so the LAST
    59	    /// final-Lean call wins (matters when B4 adds post-hoc verify after
    60	    /// runtime accept).
    61	    pub fn mark_final_accept(&mut self) {
    62	        self.final_accept = Some(Instant::now());
    63	    }
    64	
    65	    /// T_i in milliseconds.
    66	    /// Returns None if the bracket never opened. If only `first_read` is set
    67	    /// (run aborted before any final accept) returns elapsed-since-first-read,
    68	    /// which is the right thing for the no-OMEGA exit at max_transactions.
    69	    pub fn elapsed_ms(&self) -> Option<u64> {
    70	        match (self.first_read, self.final_accept) {
    71	            (Some(start), Some(end)) => {
    72	                Some(end.saturating_duration_since(start).as_millis() as u64)
    73	            }
    74	            (Some(start), None) => Some(start.elapsed().as_millis() as u64),
    75	            _ => None,
    76	        }
    77	    }
    78	
    79	    /// Test-only constructor to inject specific Instants (deterministic
    80	    /// timing for the conformance battery — avoids `thread::sleep` flake).
    81	    #[cfg(test)]
    82	    pub fn from_instants(first_read: Instant, final_accept: Instant) -> Self {
    83	        Self {
    84	            first_read: Some(first_read),
    85	            final_accept: Some(final_accept),
    86	        }
    87	    }
    88	}
    89	
    90	#[cfg(test)]
    91	mod tests {
    92	    use super::*;
    93	    use std::time::Duration;
    94	
    95	    /// PREREG § 3 anti-Goodhart conformance:
    96	    /// "synthetic run with 100ms prompt construction + 5s LLM call + 2s Lean
    97	    /// verify → assert wall_time_ms ≥ 7100"
    98	    /// Uses Instant arithmetic so the test runs in microseconds rather than
    99	    /// 7 real seconds — the bracket math is what's being tested.
   100	    #[test]
   101	    fn test_wall_clock_first_read_to_final_accept() {
   102	        let t0 = Instant::now();
   103	        let prompt_construction = Duration::from_millis(100);
   104	        let llm_call = Duration::from_secs(5);
   105	        let lean_verify = Duration::from_secs(2);
   106	
   107	        let first_read = t0 + prompt_construction;
   108	        let final_accept = first_read + llm_call + lean_verify;
   109	
   110	        let wc = RunWallClock::from_instants(first_read, final_accept);
   111	        let elapsed = wc.elapsed_ms().expect("bracket closed");
   112	
   113	        // 5s LLM + 2s Lean = 7000ms minimum. The plan asserts ≥ 7100 to also
   114	        // capture the prompt-construction tail. Since first_read is AFTER
   115	        // prompt construction in our wiring, the bracket is exactly 7000ms
   116	        // here — but the conformance test in this module is on RunWallClock
   117	        // semantics, not on the evaluator's choice of mark point. Loosen to
   118	        // ≥ 7000ms which is what the bracket honestly captures, and assert
   119	        // upper bound of 7100ms to catch over-counting bugs.
   120	        assert!(elapsed >= 7000,
   121	            "wall_time_ms must include LLM + Lean (≥ 7000ms), got {}", elapsed);
   122	        assert!(elapsed <= 7100,
   123	            "wall_time_ms must not double-count (≤ 7100ms), got {}", elapsed);
   124	    }
   125	
   126	    #[test]
   127	    fn test_wall_clock_first_read_idempotent() {
   128	        let mut wc = RunWallClock::new();
   129	        wc.mark_first_read();
   130	        let first = wc.first_read.expect("set");
   131	        std::thread::sleep(Duration::from_millis(2));
   132	        wc.mark_first_read(); // should be no-op
   133	        assert_eq!(wc.first_read, Some(first),
   134	            "first_read must be set once; later calls no-op");
   135	    }
   136	
   137	    #[test]
   138	    fn test_wall_clock_final_accept_overwrites() {
   139	        // B4 will add a post-hoc verifier called AFTER runtime accept. If
   140	        // both fire mark_final_accept, the LATER instant must win so the
   141	        // bracket includes the trailing verify.
   142	        let mut wc = RunWallClock::new();
   143	        wc.mark_first_read();
   144	        wc.mark_final_accept();
   145	        let first_close = wc.final_accept.expect("set");
   146	        std::thread::sleep(Duration::from_millis(2));
   147	        wc.mark_final_accept();
   148	        let second_close = wc.final_accept.expect("set");
   149	        assert!(second_close > first_close,
   150	            "final_accept must update on each call; second must be later");
   151	    }
   152	
   153	    #[test]
   154	    fn test_wall_clock_no_final_accept_uses_now() {
   155	        // No-OMEGA exit path: only first_read is marked. elapsed_ms must
   156	        // return time since first_read so the jsonl row still carries T_i.
   157	        let mut wc = RunWallClock::new();
   158	        wc.mark_first_read();
   159	        std::thread::sleep(Duration::from_millis(5));
   160	        let e = wc.elapsed_ms().expect("first_read set");
   161	        assert!(e >= 5, "elapsed must reflect time since first_read, got {}ms", e);
   162	    }
   163	
   164	    #[test]
   165	    fn test_wall_clock_unmarked_returns_none() {
   166	        let wc = RunWallClock::new();
   167	        assert!(wc.elapsed_ms().is_none(),
   168	            "elapsed_ms must be None before first_read is marked");
   169	    }
   170	}

exec
/bin/bash -lc 'find /home/zephryj/projects/turingosv4 -name AGENTS.md -print' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/post_hoc_verifier.rs | sed -n '1,240p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// PPUT-CCL Phase B B4 — post-hoc verifier + dual PPUT (runtime vs verified).
     2	//
     3	// Spec:
     4	//   handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md § B4
     5	//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md § 5 (Progress definition)
     6	//   PREREG § 3 anti-Goodhart conformance #7
     7	//     (test_golden_path_requires_ground_truth_acceptance)
     8	//
     9	// Why two PPUT fields:
    10	//   pput_runtime  = Progress_runtime  / (C_i × T_i / 1000)
    11	//   pput_verified = Progress_verified / (C_i × T_i / 1000)   ← North Star
    12	//
    13	//   Progress_runtime  = 1 iff the evaluator's runtime accept gate fired.
    14	//   Progress_verified = 1 iff the post-hoc Lean call returns Ok((true, _))
    15	//                       on the golden_path_payload.
    16	//
    17	//   In current implementation (Phase B), the runtime gate IS the Lean call
    18	//   (no Soft Law mode yet) — so the two agree on every solved run.
    19	//
    20	//   The split exists for Phase C ablation: Soft Law mode fakes runtime
    21	//   acceptance without running Lean. Under Soft Law:
    22	//     pput_runtime  may be > 0 (fake accept inflates Progress_runtime)
    23	//     pput_verified MUST be 0  (Lean reject means Progress_verified = 0)
    24	//   The divergence between the two is the H1 detection mechanism.
    25	//
    26	//   pput_verified is the only North Star metric for H-VPPUT. pput_runtime
    27	//   is emitted only as the divergence signal.
    28	
    29	use crate::lean4_oracle::Lean4Oracle;
    30	use crate::jsonl_schema::RunAggregate;
    31	
    32	/// Run the post-hoc Lean verification gate on a golden_path_payload.
    33	/// ALWAYS runs the real Lean call. There is NO Soft Law short-circuit
    34	/// here — that's the entire point of the post-hoc layer.
    35	///
    36	/// In current Phase B wiring, the OMEGA-accept code path has already run
    37	/// `verify_omega_detailed` on the same payload and gotten `Ok((true, _))`,
    38	/// so this function is logically guaranteed to return `true` on solved runs.
    39	/// Calling it would just double the Lean cost. Phase C diverges the call
    40	/// site: runtime accept becomes a flag, post-hoc verify stays mandatory.
    41	///
    42	/// Use `compute_progress_verified` instead when the runtime gate already
    43	/// IS the Lean call — it propagates the same truth value without paying
    44	/// for a second Lean process.
    45	pub fn verify_post_hoc(oracle: &Lean4Oracle, golden_path_payload: &str) -> bool {
    46	    matches!(
    47	        oracle.verify_omega_detailed(golden_path_payload),
    48	        Ok((true, _))
    49	    )
    50	}
    51	
    52	/// Compute Progress_verified from a (runtime, verified) pair.
    53	///
    54	/// Returns 1 only when both runtime and verified say accept.
    55	/// - Phase B: runtime == verified always (runtime IS Lean). Returns 1 on
    56	///   any accepted run, 0 otherwise.
    57	/// - Phase C Soft Law: runtime can be true while verified is false. The
    58	///   AND collapses to the verified leg, which is the North Star truth.
    59	pub fn compute_progress_verified(runtime_accepted: bool, post_hoc_verified: bool) -> u8 {
    60	    if runtime_accepted && post_hoc_verified { 1 } else { 0 }
    61	}
    62	
    63	/// Compute Progress_runtime from the runtime accept signal alone.
    64	/// Inflates under Soft Law when fake-accept fires without Lean.
    65	pub fn compute_progress_runtime(runtime_accepted: bool) -> u8 {
    66	    if runtime_accepted { 1 } else { 0 }
    67	}
    68	
    69	/// Wrap RunAggregate::compute_pput_verified for callers in evaluator that
    70	/// only have (progress, c_i, t_i_ms). Same math, single source of truth.
    71	pub fn compute_pput(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
    72	    RunAggregate::compute_pput_verified(progress, c_i, t_i_ms)
    73	}
    74	
    75	/// 10^6 × pput. Display unit per PREREG § 5.
    76	pub fn compute_pput_m(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
    77	    RunAggregate::compute_pput_m_verified(progress, c_i, t_i_ms)
    78	}
    79	
    80	#[cfg(test)]
    81	mod tests {
    82	    use super::*;
    83	
    84	    /// PREREG § 3 anti-Goodhart conformance #7
    85	    /// (test_golden_path_requires_ground_truth_acceptance):
    86	    /// "synthesize a run that records runtime accept + Lean reject →
    87	    ///  assert progress = 0, pput_verified = 0.0"
    88	    ///
    89	    /// This is the H1 detection gate that makes Soft Law impossible to
    90	    /// hide behind. Without this test failing on a runtime/verified
    91	    /// disagreement, an attacker could inflate North Star PPUT by faking
    92	    /// runtime accepts without paying real Lean verification cost.
    93	    #[test]
    94	    fn test_pput_verified_zero_when_lean_rejects() {
    95	        // Soft Law-style scenario: runtime gate fired, but post-hoc Lean
    96	        // rejected the same payload. The North Star MUST collapse the
    97	        // run's progress to 0.
    98	        let runtime_accepted = true;
    99	        let post_hoc_verified = false;
   100	        let c_i: u64 = 5_000;       // tokens
   101	        let t_i_ms: u64 = 30_000;   // 30 seconds wall
   102	
   103	        let progress_runtime = compute_progress_runtime(runtime_accepted);
   104	        let progress_verified =
   105	            compute_progress_verified(runtime_accepted, post_hoc_verified);
   106	
   107	        let pput_runtime = compute_pput(progress_runtime, c_i, t_i_ms);
   108	        let pput_verified = compute_pput(progress_verified, c_i, t_i_ms);
   109	        let pput_m_verified = compute_pput_m(progress_verified, c_i, t_i_ms);
   110	
   111	        assert_eq!(progress_runtime, 1u8,
   112	            "runtime gate fired → progress_runtime = 1");
   113	        assert_eq!(progress_verified, 0u8,
   114	            "Lean rejected → progress_verified MUST be 0 (North Star truth)");
   115	        assert!(pput_runtime > 0.0,
   116	            "pput_runtime inflates under runtime accept (Soft Law signal)");
   117	        assert_eq!(pput_verified, 0.0,
   118	            "pput_verified MUST be 0 when Lean rejects — North Star Goodhart shield");
   119	        assert_eq!(pput_m_verified, 0.0,
   120	            "pput_m_verified must collapse with pput_verified");
   121	
   122	        // Sanity: divergence is detectable. pput_runtime - pput_verified > 0
   123	        // is the H1 signal Phase C scans for.
   124	        assert!(pput_runtime - pput_verified > 0.0,
   125	            "(pput_runtime - pput_verified) > 0 ⟺ Soft Law divergence detected");
   126	    }
   127	
   128	    #[test]
   129	    fn test_pput_verified_matches_runtime_when_both_accept() {
   130	        // Phase B reality: runtime IS Lean, so on any solved run the two
   131	        // metrics MUST agree. This test guards against accidentally
   132	        // diverging them in B4 wiring (e.g., a typo that AND'ed the wrong
   133	        // booleans and made pput_verified always 0).
   134	        let c_i: u64 = 5_000;
   135	        let t_i_ms: u64 = 30_000;
   136	
   137	        let progress_runtime = compute_progress_runtime(true);
   138	        let progress_verified = compute_progress_verified(true, true);
   139	
   140	        assert_eq!(progress_runtime, progress_verified,
   141	            "Phase B: runtime == verified on solved runs");
   142	        assert_eq!(
   143	            compute_pput(progress_runtime, c_i, t_i_ms),
   144	            compute_pput(progress_verified, c_i, t_i_ms),
   145	            "pput fields must agree when runtime == verified");
   146	    }
   147	
   148	    #[test]
   149	    fn test_no_runtime_accept_zeros_both_pput() {
   150	        // No-OMEGA exit: neither runtime nor verified fired.
   151	        let c_i: u64 = 50_000;
   152	        let t_i_ms: u64 = 600_000;
   153	
   154	        let progress_runtime = compute_progress_runtime(false);
   155	        let progress_verified = compute_progress_verified(false, false);
   156	
   157	        assert_eq!(compute_pput(progress_runtime, c_i, t_i_ms), 0.0);
   158	        assert_eq!(compute_pput(progress_verified, c_i, t_i_ms), 0.0);
   159	    }
   160	
   161	    #[test]
   162	    fn test_post_hoc_verified_without_runtime_still_zero_progress() {
   163	        // Defensive: a post-hoc verifier that says "yes" but runtime never
   164	        // fired is a wiring bug, not an honest progress signal. Progress
   165	        // is gated on BOTH runtime initiation AND verified result, so this
   166	        // pathological case must clamp to 0.
   167	        assert_eq!(compute_progress_verified(false, true), 0u8,
   168	            "verified without runtime accept = wiring bug, must clamp to 0");
   169	    }
   170	}

exec
/bin/bash -lc "nl -ba src/drivers/llm_http.rs | sed -n '1,190p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// Tier 5: Resilient HTTP client — local proxy only, never direct HTTPS
     2	// Constitutional basis: Art. IV (Boot infrastructure)
     3	// V3L-25: never direct HTTPS from Rust (TLS deadlock on certain endpoints)
     4	// V3L-26: ThreadingMixIn on proxy side (single-thread = 502)
     5	// V3L-27: rate limit handling (retry with backoff)
     6	
     7	use serde::{Deserialize, Serialize};
     8	use std::time::Duration;
     9	
    10	// ── Core types ──────────────────────────────────────────────────
    11	
    12	/// LLM generation request.
    13	#[derive(Debug, Serialize)]
    14	pub struct GenerateRequest {
    15	    pub model: String,
    16	    pub messages: Vec<Message>,
    17	    #[serde(skip_serializing_if = "Option::is_none")]
    18	    pub temperature: Option<f64>,
    19	    #[serde(skip_serializing_if = "Option::is_none")]
    20	    pub max_tokens: Option<u32>,
    21	}
    22	
    23	#[derive(Debug, Serialize, Deserialize, Clone)]
    24	pub struct Message {
    25	    pub role: String,
    26	    pub content: String,
    27	}
    28	
    29	/// LLM generation response.
    30	#[derive(Debug, Deserialize)]
    31	pub struct GenerateResponse {
    32	    pub content: String,
    33	    pub completion_tokens: u32,
    34	    /// API-reported prompt tokens. Falls back to 0 if `usage.prompt_tokens` is
    35	    /// absent in the proxy response (older proxies). Surfaced for PPUT-CCL
    36	    /// Phase B C_i accounting (post-hoc, not estimation — plan B2 default).
    37	    pub prompt_tokens: u32,
    38	    pub model: String,
    39	}
    40	
    41	/// Driver errors. V3L-09: explicit, never silent.
    42	#[derive(Debug)]
    43	pub enum DriverError {
    44	    NetworkError(String),
    45	    Timeout,
    46	    RateLimited,
    47	    ParseError(String),
    48	    BackendError(String),
    49	}
    50	
    51	impl std::fmt::Display for DriverError {
    52	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    53	        match self {
    54	            DriverError::NetworkError(msg) => write!(f, "Network error: {}", msg),
    55	            DriverError::Timeout => write!(f, "Request timeout"),
    56	            DriverError::RateLimited => write!(f, "Rate limited (429)"),
    57	            DriverError::ParseError(msg) => write!(f, "Parse error: {}", msg),
    58	            DriverError::BackendError(msg) => write!(f, "Backend error: {}", msg),
    59	        }
    60	    }
    61	}
    62	
    63	impl std::error::Error for DriverError {}
    64	
    65	/// Resilient HTTP client that connects to a LOCAL proxy only.
    66	/// V3L-25: NEVER connect directly to cloud HTTPS endpoints from Rust.
    67	/// The proxy (llm_proxy.py) handles TLS, rate limits, and provider routing.
    68	pub struct ResilientLLMClient {
    69	    proxy_url: String,
    70	    timeout: Duration,
    71	    max_retries: u32,
    72	}
    73	
    74	impl ResilientLLMClient {
    75	    /// Create a client pointing to a LOCAL HTTP proxy.
    76	    /// `proxy_url` must be http://localhost:PORT or http://127.0.0.1:PORT.
    77	    pub fn new(proxy_url: &str, timeout_secs: u64, max_retries: u32) -> Self {
    78	        ResilientLLMClient {
    79	            proxy_url: proxy_url.to_string(),
    80	            timeout: Duration::from_secs(timeout_secs),
    81	            max_retries,
    82	        }
    83	    }
    84	
    85	    /// Generate a completion via the local proxy.
    86	    /// Retries on transient errors with exponential backoff.
    87	    /// V3L-27: handles 429 rate limits gracefully.
    88	    pub async fn generate(&self, request: &GenerateRequest) -> Result<GenerateResponse, DriverError> {
    89	        let client = reqwest::Client::builder()
    90	            .timeout(self.timeout)
    91	            .build()
    92	            .map_err(|e| DriverError::NetworkError(e.to_string()))?;
    93	
    94	        let mut last_error = DriverError::NetworkError("No attempts made".into());
    95	
    96	        for attempt in 0..=self.max_retries {
    97	            if attempt > 0 {
    98	                // Exponential backoff: 1s, 2s, 4s...
    99	                let delay = Duration::from_secs(1 << (attempt - 1).min(4));
   100	                tokio::time::sleep(delay).await;
   101	            }
   102	
   103	            match client
   104	                .post(&format!("{}/v1/chat/completions", self.proxy_url))
   105	                .json(request)
   106	                .send()
   107	                .await
   108	            {
   109	                Ok(response) => {
   110	                    let status = response.status();
   111	                    if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
   112	                        last_error = DriverError::RateLimited;
   113	                        continue;
   114	                    }
   115	                    if !status.is_success() {
   116	                        let body = response.text().await.unwrap_or_default();
   117	                        last_error = DriverError::BackendError(
   118	                            format!("HTTP {}: {}", status, body)
   119	                        );
   120	                        continue;
   121	                    }
   122	
   123	                    // Parse OpenAI-compatible response
   124	                    let body: serde_json::Value = response.json().await
   125	                        .map_err(|e| DriverError::ParseError(e.to_string()))?;
   126	
   127	                    let content = body["choices"][0]["message"]["content"]
   128	                        .as_str()
   129	                        .unwrap_or("")
   130	                        .to_string();
   131	                    let tokens = body["usage"]["completion_tokens"]
   132	                        .as_u64()
   133	                        .unwrap_or(0) as u32;
   134	                    let prompt_tokens = body["usage"]["prompt_tokens"]
   135	                        .as_u64()
   136	                        .unwrap_or(0) as u32;
   137	                    let model = body["model"]
   138	                        .as_str()
   139	                        .unwrap_or(&request.model)
   140	                        .to_string();
   141	
   142	                    return Ok(GenerateResponse {
   143	                        content,
   144	                        completion_tokens: tokens,
   145	                        prompt_tokens,
   146	                        model,
   147	                    });
   148	                }
   149	                Err(e) => {
   150	                    if e.is_timeout() {
   151	                        last_error = DriverError::Timeout;
   152	                    } else {
   153	                        last_error = DriverError::NetworkError(e.to_string());
   154	                    }
   155	                    continue;
   156	                }
   157	            }
   158	        }
   159	
   160	        Err(last_error)
   161	    }
   162	}
   163	
   164	#[cfg(test)]
   165	mod tests {
   166	    use super::*;
   167	
   168	    #[test]
   169	    fn test_client_creation() {
   170	        let client = ResilientLLMClient::new("http://localhost:8080", 120, 3);
   171	        assert_eq!(client.proxy_url, "http://localhost:8080");
   172	        assert_eq!(client.max_retries, 3);
   173	    }
   174	
   175	    #[test]
   176	    fn test_generate_request_serialization() {
   177	        let req = GenerateRequest {
   178	            model: "deepseek-v3.2".into(),
   179	            messages: vec![Message { role: "user".into(), content: "test".into() }],
   180	            temperature: Some(0.2),
   181	            max_tokens: Some(8000),
   182	        };
   183	        let json = serde_json::to_string(&req).unwrap();
   184	        assert!(json.contains("deepseek-v3.2"));
   185	        assert!(json.contains("0.2"));
   186	    }
   187	
   188	    #[test]
   189	    fn test_driver_error_display() {
   190	        assert_eq!(

 succeeded in 145ms:
codex
The line-numbered code confirms no scoped `AGENTS.md` exists in this repository. I’ve already found one important mismatch: the wall-clock conformance test in code explicitly relaxes the plan’s 7100ms requirement to 7000ms, so I’m tracing whether that is just a test wording problem or an implementation undercount.
exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '220,520p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   220	    );
   221	
   222	    let client = ResilientLLMClient::new(proxy_url, 1800, 2);
   223	    // Model-aware max_tokens: deepseek-chat caps at 8192; reasoner needs 16000 for thinking.
   224	    let max_toks = if model.contains("chat") { 8000 } else { 16000 };
   225	    let request = GenerateRequest {
   226	        model: model.to_string(),
   227	        messages: vec![Message { role: "user".into(), content: prompt }],
   228	        temperature: Some(0.2),
   229	        max_tokens: Some(max_toks),
   230	    };
   231	
   232	    // PPUT-CCL B3: open the wall-clock bracket at first agent prompt construction
   233	    // (just before the API call goes out). Excludes prior preflight cost.
   234	    wc.mark_first_read();
   235	    match client.generate(&request).await {
   236	        Ok(response) => {
   237	            acc.record_llm_call(response.prompt_tokens, response.completion_tokens);
   238	            acc.record_proposal(false);
   239	            // Rule 22 v2 clause 4: reject markdown fences
   240	            if response.content.contains("```") {
   241	                wc.mark_final_accept();
   242	                return make_pput(problem_file, "oneshot", model, false, start, 0, 0, 1,
   243	                                 None, None, None, None, None,
   244	                                 Some(acc.total_run_token_count()),
   245	                                 Some(acc.failed_branch_count),
   246	                                 wc.elapsed_ms());
   247	            }
   248	
   249	            let verdict = oracle.verify_omega(&response.content);
   250	            // B3: close the bracket AFTER the Lean call returns, regardless of
   251	            // verdict. Soft Law mode (Phase C) cannot escape the verify-time
   252	            // accounting by short-circuiting on runtime accept.
   253	            wc.mark_final_accept();
   254	            match verdict {
   255	                Ok(true) => {
   256	                    acc.flip_last_failed_to_accepted();
   257	                    let gp_tokens = response.completion_tokens as u64;
   258	                    let preview: String = response.content.chars().take(500).collect();
   259	                    info!(">>> OMEGA ACCEPTED <<< (path=alone, payload[0..500]={:?})", preview);
   260	                    let proof_file = persist_proof_artifact(
   261	                        problem_file, theorem_name, problem_statement,
   262	                        &response.content, "alone", "oneshot",
   263	                    );
   264	                    make_pput(problem_file, "oneshot", model, true, start, gp_tokens, 1, 1,
   265	                              None, None, Some(response.content.clone()),
   266	                              Some("alone".to_string()), proof_file,
   267	                              Some(acc.total_run_token_count()),
   268	                              Some(acc.failed_branch_count),
   269	                              wc.elapsed_ms())
   270	                }
   271	                Ok(false) => {
   272	                    make_pput(problem_file, "oneshot", model, false, start, 0, 0, 1,
   273	                              None, None, None, None, None,
   274	                              Some(acc.total_run_token_count()),
   275	                              Some(acc.failed_branch_count),
   276	                              wc.elapsed_ms())
   277	                }
   278	                Err(e) => {
   279	                    warn!("Oracle error: {}", e);
   280	                    make_pput(problem_file, "oneshot", model, false, start, 0, 0, 1,
   281	                              None, None, None, None, None,
   282	                              Some(acc.total_run_token_count()),
   283	                              Some(acc.failed_branch_count),
   284	                              wc.elapsed_ms())
   285	                }
   286	            }
   287	        }
   288	        Err(e) => {
   289	            // C-012: measurement failure ≠ verified failure.
   290	            // Do not emit PPUT_RESULT — batch runner must retry on resume.
   291	            // C-017: broadcast error explicitly (stderr, non-zero exit).
   292	            error!("LLM error: {}", e);
   293	            eprintln!("MEASUREMENT_ERROR oneshot LLM: {}", e);
   294	            std::process::exit(2);
   295	        }
   296	    }
   297	}
   298	
   299	/// Swarm: N agents, prediction market, Boltzmann routing → PPUT.
   300	async fn run_swarm(
   301	    problem_file: &str, problem_statement: &str, theorem_name: &str,
   302	    lean_path: &str, proxy_url: &str, model: &str, n_agents: usize,
   303	) -> PputResult {
   304	    let start = Instant::now();
   305	    let condition = format!("n{}", n_agents);
   306	
   307	    let kernel = Kernel::new();
   308	    let config = BusConfig {
   309	        // Phase 2.1 (C-043 candidate): OMEGA-accepted proofs are auto-written
   310	        // as tape nodes (mandatory wtool per Art. IV). Full proofs can be
   311	        // long; raise bus caps so winning nodes don't get size-vetoed. Agent
   312	        // partials still typically <1200; no behavioural regression.
   313	        max_payload_chars: 8000,
   314	        max_payload_lines: 200,
   315	        system_lp_amount: 200.0,
   316	        // C-011: decide/omega/native_decide forbidden (brute-force precedent)
   317	        forbidden_patterns: vec![
   318	            "native_decide".into(), "decide".into(), "omega".into(),
   319	            "#eval".into(), "IO.Process".into(),
   320	            "IO.FS".into(), "run_tac".into(), "unsafe".into(),
   321	        ],
   322	    };
   323	
   324	    // Phase 1: opt-in tape persistence via env. WAL_DIR=<dir> enables WAL
   325	    // writes to <dir>/<problem>_<timestamp>.jsonl; resumes if file exists.
   326	    // Default off for backward-compat baseline runs.
   327	    let mut bus = if let Ok(wal_dir) = std::env::var("WAL_DIR") {
   328	        let problem_stem = std::path::Path::new(problem_file)
   329	            .file_stem().map(|s| s.to_string_lossy().into_owned())
   330	            .unwrap_or_else(|| "unknown".into());
   331	        let resume_id = std::env::var("WAL_RESUME_ID").ok();
   332	        let id = resume_id.unwrap_or_else(|| {
   333	            std::time::SystemTime::now()
   334	                .duration_since(std::time::UNIX_EPOCH)
   335	                .map(|d| d.as_secs().to_string())
   336	                .unwrap_or_else(|_| "0".into())
   337	        });
   338	        let wal_path = std::path::Path::new(&wal_dir)
   339	            .join(format!("{}_{}.jsonl", problem_stem, id));
   340	        info!("[wal] using {:?}", wal_path);
   341	        match TuringBus::with_wal_path(kernel, config, wal_path) {
   342	            Ok(b) => b,
   343	            Err(e) => {
   344	                error!("[wal] open failed: {} — falling back to in-memory", e);
   345	                TuringBus::new(Kernel::new(), BusConfig {
   346	                    max_payload_chars: 1200, max_payload_lines: 18,
   347	                    system_lp_amount: 200.0,
   348	                    forbidden_patterns: vec![
   349	                        "native_decide".into(), "decide".into(), "omega".into(),
   350	                        "#eval".into(), "IO.Process".into(), "IO.FS".into(),
   351	                        "run_tac".into(), "unsafe".into(),
   352	                    ],
   353	                })
   354	            }
   355	        }
   356	    } else {
   357	        TuringBus::new(kernel, config)
   358	    };
   359	    // Phase 4 (C-041 candidate): cross-problem wallet persistence. WALLET_STATE
   360	    // env points to a json file; if it exists we load agents' carried-over
   361	    // balances/portfolios, otherwise fresh genesis. No second mint under Law 2:
   362	    // genesis_done is serialised, so on_init is a no-op post first boot.
   363	    let wallet_state_path: Option<std::path::PathBuf> = std::env::var("WALLET_STATE")
   364	        .ok().map(std::path::PathBuf::from);
   365	    let wallet = wallet_state_path.as_ref()
   366	        .and_then(|p| WalletTool::load_from_disk(p))
   367	        .unwrap_or_else(|| WalletTool::new(10000.0));
   368	    if wallet_state_path.is_some() && wallet.genesis_done {
   369	        info!("[wallet] resumed from {:?}; existing agents carry balances",
   370	              wallet_state_path);
   371	    }
   372	    bus.mount_tool(Box::new(wallet));
   373	    bus.mount_tool(Box::new(Lean4Oracle::new(
   374	        problem_statement.to_string(), theorem_name.to_string(), lean_path.to_string(),
   375	    )));
   376	    bus.mount_tool(Box::new(SearchTool::new(
   377	        vec![format!("{}/MiniF2F/Test", std::env::var("MINIF2F_DIR")
   378	            .unwrap_or_else(|_| DEFAULT_MINIF2F_DIR.into()))], 20,
   379	    )));
   380	    bus.mount_tool(Box::new(LibrarianTool::new(
   381	        &format!("{}/skills", std::env::var("EXPERIMENT_DIR").unwrap_or_else(|_| ".".into())), 8,
   382	    )));
   383	
   384	    let agent_ids: Vec<String> = (0..n_agents).map(|i| format!("Agent_{}", i)).collect();
   385	    bus.init(&agent_ids);
   386	    // Phase 4: top-up ensure_agents for any IDs not in the loaded state (zero
   387	    // balance if post-genesis, genesis_coins only on first-ever boot).
   388	    if let Some(wallet) = bus.tools.iter_mut()
   389	        .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>())
   390	    {
   391	        wallet.ensure_agents(&agent_ids);
   392	    }
   393	
   394	    // Art. II.2.1: "不能抹杀群体异质性" — distinct skills per agent.
   395	    // V3 had Math/Bull/Bear roles. V4: tactic-strategy specialization.
   396	    let agent_skills: Vec<&str> = vec![
   397	        "Focus on algebraic simplification: ring, field_simp, linarith, nlinarith.",
   398	        "Focus on structural reasoning: induction, cases, rcases, constructor.",
   399	        "Focus on rewriting and normalization: simp, norm_num, rw, calc.",
   400	    ];
   401	
   402	    let client = ResilientLLMClient::new(proxy_url, 1800, 2);
   403	    let params = BoltzmannParams::from_env();
   404	    // C-012: seed the Boltzmann RNG so A/B runs are reproducible.
   405	    // Only the LLM sampling remains stochastic; same-problem paired comparison absorbs that.
   406	    let boltzmann_seed: u64 = std::env::var("BOLTZMANN_SEED")
   407	        .ok().and_then(|s| s.parse().ok()).unwrap_or(DEFAULT_BOLTZMANN_SEED);
   408	    let mut boltz_rng = StdRng::seed_from_u64(boltzmann_seed);
   409	    let max_transactions = 200;
   410	
   411	    // Art. IV map-reduce tick: periodic tape statistics (clock → mr → map/reduce)
   412	    let tick_interval: usize = std::env::var("TICK_INTERVAL")
   413	        .ok().and_then(|s| s.parse().ok()).unwrap_or(20);
   414	
   415	    // C-036 startup echo: per-agent (skill, temp) so debugging never grep-source.
   416	    let temp_ladder_on = std::env::var("TEMP_LADDER").ok().as_deref() == Some("1");
   417	    let agent_cfg: Vec<String> = (0..n_agents).map(|i| {
   418	        let s = i % agent_skills.len();
   419	        let t = if temp_ladder_on { (0.10_f64 + (i as f64) * 0.15).min(1.30) } else { 0.2 };
   420	        format!("Agent_{}:skill{}:t={:.2}", i, s, t)
   421	    }).collect();
   422	    info!("[swarm/{}] {}", condition, agent_cfg.join(" "));
   423	
   424	    // C-036 telemetry counters.
   425	    let mut tool_dist: HashMap<String, u32> = HashMap::new();
   426	    let mut omega_payload_hashes: HashSet<u64> = HashSet::new();
   427	    let mut omega_attempts: u32 = 0;
   428	    let mut zero_ticks_run: u32 = 0;
   429	    let mut zero_tick_warned = false;
   430	    // PPUT-CCL B2: full-run cost C_i — every LLM call + tool stdout summed
   431	    // across all proposals (winning + failed branches). Read at terminal
   432	    // make_pput sites and stamped on the emitted jsonl row.
   433	    let mut acc = RunCostAccumulator::new();
   434	    // PPUT-CCL B3: full-run wall-clock T_i — first agent prompt → final Lean
   435	    // call. Opened on first tx's prompt build, closed before each return.
   436	    let mut wc = RunWallClock::new();
   437	    // Art. III.2: per-agent search result cache (bounded), fed into next prompt.
   438	    let mut search_cache: HashMap<String, Vec<String>> = HashMap::new();
   439	    // F-2026-04-19-05: cap searches per agent; beyond cap we remove `search`
   440	    // from the tool list so agents stop wasting budget on name-match misses.
   441	    let search_cap: u32 = std::env::var("SEARCH_CAP")
   442	        .ok().and_then(|s| s.parse().ok()).unwrap_or(20);
   443	    let mut search_count: HashMap<String, u32> = HashMap::new();
   444	
   445	    for tx in 0..max_transactions {
   446	        // Map-reduce tick (Art. IV mermaid: clock → mr → tape)
   447	        if tick_interval > 0 && tx > 0 && tx % tick_interval == 0 {
   448	            let tape_len = bus.kernel.tape.time_arrow().len();
   449	            let market_count = bus.kernel.markets.len();
   450	            let ticker = bus.kernel.market_ticker(5);
   451	            let top_prices: Vec<String> = ticker.iter()
   452	                .map(|(id, p)| format!("{}:{:.0}%", id, p * 100.0))
   453	                .collect();
   454	            info!("[tick@tx{}] tape={} markets={} top={}", tx, tape_len, market_count,
   455	                top_prices.join(", "));
   456	            // Phase 6-emergent: refresh shared team board from facts only.
   457	            // Per-agent cumulative balance + recent tape-node authorship counts
   458	            // + top market prices. No instructions, no "should" — just state.
   459	            if std::env::var("EMERGENT_ROLES").ok().as_deref() == Some("1") {
   460	                let agents_sorted: Vec<String> = agent_ids.clone();
   461	                let mut author_counts: std::collections::HashMap<String, u32> =
   462	                    std::collections::HashMap::new();
   463	                for nid in bus.kernel.tape.time_arrow() {
   464	                    if let Some(n) = bus.kernel.tape.get(nid) {
   465	                        *author_counts.entry(n.author.clone()).or_insert(0) += 1;
   466	                    }
   467	                }
   468	                let wallet_balances: std::collections::HashMap<String, f64> =
   469	                    bus.tools.iter()
   470	                        .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
   471	                        .map(|w| w.balances.clone())
   472	                        .unwrap_or_default();
   473	                let mut board = format!("# tick@tx{} (tape_nodes={})\n", tx, tape_len);
   474	                for a in &agents_sorted {
   475	                    let bal = wallet_balances.get(a).copied().unwrap_or(10000.0);
   476	                    let delta = bal - 10000.0;
   477	                    let nodes = author_counts.get(a).copied().unwrap_or(0);
   478	                    board.push_str(&format!(
   479	                        "- {}: balance={:.0} (Δ{:+.0}), tape_nodes_authored={}\n",
   480	                        a, bal, delta, nodes));
   481	                }
   482	                if !top_prices.is_empty() {
   483	                    board.push_str(&format!("markets: {}\n", top_prices.join(", ")));
   484	                }
   485	                // Preserve any agent posts that were already in the file (append-only).
   486	                if let Some(lib) = bus.tools.iter()
   487	                    .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
   488	                {
   489	                    let existing = lib.read_board();
   490	                    // Keep only the POST lines (they carry agent-originated intent).
   491	                    let posts: String = existing.lines()
   492	                        .filter(|l| l.starts_with("## POST") || (l.starts_with(" ") == false && !l.starts_with("#") && !l.starts_with("-") && !l.starts_with("markets:")))
   493	                        .collect::<Vec<_>>()
   494	                        .join("\n");
   495	                    let full = if posts.is_empty() {
   496	                        board
   497	                    } else {
   498	                        format!("{}\n{}\n", board, posts)
   499	                    };
   500	                    let _ = lib.write_board(&full);
   501	                }
   502	            }
   503	            // C-036 zero-tick alarm: 5 consecutive ticks with no constitutional engine activity.
   504	            if tape_len == 0 && market_count == 0 {
   505	                zero_ticks_run += 1;
   506	                if zero_ticks_run >= 5 && !zero_tick_warned {
   507	                    warn!("[harness] {} consecutive zero-ticks (tape & markets idle) — \
   508	                           constitutional engines bypassed (Art. II.1/II.2 unused)", zero_ticks_run);
   509	                    zero_tick_warned = true;
   510	                }
   511	            } else {
   512	                zero_ticks_run = 0;
   513	            }
   514	        }
   515	
   516	        let agent_idx = tx % n_agents;
   517	        let agent_id = &agent_ids[agent_idx];
   518	        let snap = bus.snapshot();
   519	
   520	        let chain = if snap.tape.is_empty() {

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '520,880p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   520	        let chain = if snap.tape.is_empty() {
   521	            problem_statement.to_string()
   522	        } else {
   523	            let nodes: Vec<String> = snap.tape.time_arrow().iter()
   524	                .filter_map(|id| snap.tape.get(id))
   525	                .map(|n| format!("[{}] {}: {}", n.id, n.author, n.payload))
   526	                .collect();
   527	            format!("{}\n\n=== Proof Chain ===\n{}", problem_statement, nodes.join("\n"))
   528	        };
   529	
   530	        let errors = bus.recent_rejections(agent_id, 3);
   531	        // Art. II.2.1: per-agent skill specialization + Librarian learned memory
   532	        let base_skill = agent_skills.get(agent_idx % agent_skills.len()).unwrap_or(&"");
   533	        let learned = bus.tools.iter()
   534	            .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
   535	            .and_then(|lib| lib.read_agent_memory(agent_id))
   536	            .unwrap_or_default();
   537	        let skill = if learned.is_empty() {
   538	            base_skill.to_string()
   539	        } else {
   540	            format!("{}\n\n{}", base_skill, learned)
   541	        };
   542	        let hits_ref: Vec<String> = search_cache.get(agent_id).cloned().unwrap_or_default();
   543	        let tools_desc = if search_count.get(agent_id).copied().unwrap_or(0) >= search_cap {
   544	            "append, complete, invest"
   545	        } else {
   546	            "append, complete, invest, search"
   547	        };
   548	        // Phase 6-emergent: read the shared team board. Gated by EMERGENT_ROLES=1
   549	        // so baseline behaviour is untouched. Board content is built by
   550	        // Librarian at periodic ticks (see refresh_board below).
   551	        let team_board: String = if std::env::var("EMERGENT_ROLES").ok().as_deref() == Some("1") {
   552	            bus.tools.iter()
   553	                .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
   554	                .map(|l| l.read_board())
   555	                .unwrap_or_default()
   556	        } else {
   557	            String::new()
   558	        };
   559	        let prompt = build_agent_prompt(
   560	            &chain, &skill, &snap.market_ticker, &errors, &hits_ref,
   561	            snap.get_balance(agent_id), tools_desc, &team_board,
   562	        );
   563	        // PPUT-CCL B3: stamp wall-clock open at first agent prompt build.
   564	        // Idempotent — only the first tx's call sticks.
   565	        wc.mark_first_read();
   566	
   567	        // Model-aware max_tokens (same rule as oneshot branch).
   568	        let max_toks = if model.contains("chat") { 8000 } else { 16000 };
   569	        // Art. II.2.1 anti-homogeneity: per-agent temperature ladder breaks
   570	        // sampling correlation among role-distinct agents (F-2026-04-18-03).
   571	        // Disabled (keep at 0.2) when TEMP_LADDER!=1 to isolate the mechanism.
   572	        let temp: f64 = if std::env::var("TEMP_LADDER").ok().as_deref() == Some("1") {
   573	            (0.10_f64 + (agent_idx as f64) * 0.15).min(1.30)
   574	        } else {
   575	            0.2
   576	        };
   577	        let request = GenerateRequest {
   578	            model: model.to_string(),
   579	            messages: vec![Message { role: "user".into(), content: prompt }],
   580	            temperature: Some(temp),
   581	            max_tokens: Some(max_toks),
   582	        };
   583	
   584	        match client.generate(&request).await {
   585	            Ok(response) => {
   586	                acc.record_llm_call(response.prompt_tokens, response.completion_tokens);
   587	                // PPUT-CCL B2: every parsed proposal default-records as failed.
   588	                // OMEGA-accept return paths flip the last record before returning.
   589	                acc.record_proposal(false);
   590	                match parse_agent_output(&response.content) {
   591	                    Ok(action) => match action.tool.as_str() {
   592	                        "append" => {
   593	                            *tool_dist.entry("append".into()).or_insert(0) += 1;
   594	                            if let Some(payload) = &action.payload {
   595	                                let prices: std::collections::HashMap<String, f64> =
   596	                                    snap.markets.iter()
   597	                                        .map(|(id, m)| (id.clone(), m.yes_price))
   598	                                        .collect();
   599	                                let parent = boltzmann_select_parent(
   600	                                    &snap.tape, &prices, &params, &mut boltz_rng
   601	                                );
   602	                                match bus.append(agent_id, payload, parent.as_deref()) {
   603	                                    Ok(BusResult::Appended { node_id }) => {
   604	                                        info!("[tx {}] {} +{}", tx, agent_id, node_id);
   605	                                        // Art. III.2 Librarian: every compress_interval appends,
   606	                                        // write mechanical summary (TopK error classes) to agent's
   607	                                        // learned.md. This is white-box compression (Art. I.2:
   608	                                        // deterministic statistical algorithm), not LLM-based.
   609	                                        if let Some(lib) = bus.tools.iter()
   610	                                            .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>()) {
   611	                                            if lib.should_compress() {
   612	                                                let errors = bus.recent_rejections(agent_id, 10);
   613	                                                let summary = format!(
   614	                                                    "# Learned patterns (auto-compressed)\n\
   615	                                                     Common errors: {}\n\
   616	                                                     Tape depth: {}\n",
   617	                                                    errors.join(", "),
   618	                                                    snap.tape.time_arrow().len(),
   619	                                                );
   620	                                                let _ = lib.write_agent_memory(agent_id, &summary);
   621	                                                info!("[tx {}] Librarian compressed for {}", tx, agent_id);
   622	                                            }
   623	                                        }
   624	                                    }
   625	                                    Ok(BusResult::Vetoed { reason }) => {
   626	                                        warn!("[tx {}] VETO: {}", tx, reason);
   627	                                    }
   628	                                    _ => {}
   629	                                }
   630	                            }
   631	                        }
   632	                        "complete" => {
   633	                            *tool_dist.entry("complete".into()).or_insert(0) += 1;
   634	                            if let Some(payload) = &action.payload {
   635	                                // Art. IV (∏p(output | Q_t)): Q_t (tape) feeds the verification
   636	                                // predicate. Dual-path: try payload-alone first (standalone proof
   637	                                // preserved), then tape+payload (tape-built proof). Accept whichever
   638	                                // succeeds. This keeps Q_t in the ∏p domain without punishing
   639	                                // self-contained proofs that ignored tape.
   640	                                let tape_chain: String = bus.kernel.tape.time_arrow().iter()
   641	                                    .filter_map(|id| bus.kernel.tape.get(id))
   642	                                    .map(|n| n.payload.clone())
   643	                                    .collect::<Vec<_>>()
   644	                                    .join("\n");
   645	                                let tape_len = bus.kernel.tape.time_arrow().len();
   646	                                // C-036: track payload diversity over what agent proposed.
   647	                                let mut h = std::collections::hash_map::DefaultHasher::new();
   648	                                payload.hash(&mut h);
   649	                                omega_payload_hashes.insert(h.finish());
   650	                                omega_attempts += 1;
   651	                                info!("[tx {}] OMEGA claim by {} (tape_nodes={}, payload_len={})",
   652	                                      tx, agent_id, tape_len, payload.len());
   653	                                let oracle = Lean4Oracle::new(
   654	                                    problem_statement.to_string(),
   655	                                    theorem_name.to_string(),
   656	                                    lean_path.to_string(),
   657	                                );
   658	                                // Path 1: payload alone
   659	                                let r_alone = oracle.verify_omega_detailed(payload);
   660	                                let (full_proof, path_choice, r_final) = match &r_alone {
   661	                                    Ok((true, _)) => (payload.clone(), "alone", r_alone.clone()),
   662	                                    _ if !tape_chain.is_empty() => {
   663	                                        // Path 2: tape + payload
   664	                                        let combined = format!("{}\n{}", tape_chain, payload);
   665	                                        let r_combined = oracle.verify_omega_detailed(&combined);
   666	                                        if matches!(r_combined, Ok((true, _))) {
   667	                                            *tool_dist.entry("complete_via_tape".into()).or_insert(0) += 1;
   668	                                        }
   669	                                        (combined, "tape+payload", r_combined)
   670	                                    }
   671	                                    _ => (payload.clone(), "alone", r_alone.clone()),
   672	                                };
   673	                                // PPUT-CCL B3: close bracket AFTER both Lean verify paths return.
   674	                                // Soft Law (Phase C) cannot exit ahead of verify-time accounting.
   675	                                wc.mark_final_accept();
   676	                                match r_final {
   677	                                    Ok((true, _)) => {
   678	                                        // PPUT-CCL B2: this proposal verified — flip the failed
   679	                                        // record made at parse time into the run's accepted slot.
   680	                                        acc.flip_last_failed_to_accepted();
   681	                                        // Phase 0 (C-039): persist the winning artifact so external
   682	                                        // verifiers can re-run lean from disk alone.
   683	                                        let preview: String = full_proof.chars().take(500).collect();
   684	                                        info!(">>> OMEGA ACCEPTED <<< (path={}, payload[0..500]={:?})",
   685	                                              path_choice, preview);
   686	                                        let proof_file = persist_proof_artifact(
   687	                                            problem_file, &theorem_name, &problem_statement,
   688	                                            &full_proof, path_choice, agent_id,
   689	                                        );
   690	                                        // Phase 2.1 (C-043 candidate): mandatory wtool. Art. IV says
   691	                                        // `∏p = 1 ⟹ Q_{t+1} = wtool(output)`. Before halting, write
   692	                                        // the winning payload as a tape node through the standard
   693	                                        // append pipeline. This automatically fires founder grant
   694	                                        // (Phase 2 reward-pull) for the winning author and makes
   695	                                        // every solve end with a canonical tape node on the GP.
   696	                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
   697	                                        *tool_dist.entry("omega_wtool".into()).or_insert(0) += 1;
   698	                                        // Use oracle-blessed path: Lean has already accepted this
   699	                                        // payload, so bus-level forbidden_patterns and size caps
   700	                                        // would only re-reject legitimate tactics (e.g. `omega`,
   701	                                        // `decide` used inside a verified proof — not brute-force).
   702	                                        let omega_node_id = match bus.append_oracle_accepted(
   703	                                            agent_id, payload, parent.as_deref(),
   704	                                        ) {
   705	                                            Ok(BusResult::Appended { node_id }) => Some(node_id),
   706	                                            Ok(BusResult::Vetoed { reason }) => {
   707	                                                warn!("[art-iv] OMEGA wtool VETO (unexpected after oracle accept): {}", reason);
   708	                                                None
   709	                                            }
   710	                                            _ => None,
   711	                                        };
   712	                                        let tape_tokens: u64 = bus.kernel.tape.time_arrow().iter()
   713	                                            .filter_map(|id| bus.kernel.tape.get(id))
   714	                                            .map(|n| n.payload.len() as u64)
   715	                                            .sum();
   716	                                        // C-012: gp_tokens reflects the actual tape (now containing
   717	                                        // the winner), no double-count needed.
   718	                                        let gp_tokens = tape_tokens.max(response.completion_tokens as u64);
   719	                                        let gp = bus.kernel.tape.time_arrow().to_vec();
   720	                                        let gp_nodes = gp.len();
   721	                                        if omega_node_id.is_some() {
   722	                                            info!("[art-iv] OMEGA written as tape node; gp_nodes={}", gp_nodes);
   723	                                        }
   724	                                        bus.halt_and_settle(&gp).ok();
   725	                                        // Phase 4: persist wallet state so next problem's run
   726	                                        // inherits carried-over balances (reputation).
   727	                                        if let Some(ref wp) = wallet_state_path {
   728	                                            if let Some(w) = bus.tools.iter()
   729	                                                .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
   730	                                            {
   731	                                                if let Err(e) = w.save_to_disk(wp) {
   732	                                                    warn!("[wallet] save failed to {:?}: {}", wp, e);
   733	                                                }
   734	                                            }
   735	                                        }
   736	                                        let upr = if omega_attempts > 0 {
   737	                                            Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
   738	                                        } else { None };
   739	                                        return make_pput(problem_file, &condition, model, true,
   740	                                                        start, gp_tokens, gp_nodes, tx as u64 + 1,
   741	                                                        Some(tool_dist), upr,
   742	                                                        Some(full_proof.clone()),
   743	                                                        Some(path_choice.to_string()),
   744	                                                        proof_file,
   745	                                                        Some(acc.total_run_token_count()),
   746	                                                        Some(acc.failed_branch_count),
   747	                                                        wc.elapsed_ms());
   748	                                    }
   749	                                    Ok((false, err_detail)) => {
   750	                                        // Step-B v3: classify + record class label (C-022 shield).
   751	                                        let class = classify_lean_error(&err_detail);
   752	                                        bus.record_rejection(agent_id, class.label());
   753	                                        // PPUT-CCL B2: rejection error feeds back into next prompt's
   754	                                        // recent_rejections — count those bytes against C_i.
   755	                                        acc.record_tool_stdout(&err_detail);
   756	                                        let preview: String = payload.chars().take(300).collect();
   757	                                        warn!("[tx {}] OMEGA rejected ({}). payload[0..300]={:?}", tx, class.label(), preview);
   758	                                    }
   759	                                    Err(e) => {
   760	                                        warn!("[tx {}] OMEGA oracle error: {}", tx, e);
   761	                                    }
   762	                                }
   763	                            }
   764	                        }
   765	                        "invest" => {
   766	                            *tool_dist.entry("invest".into()).or_insert(0) += 1;
   767	                            // Law 2: Only Investment Costs Money (1 Coin = 1 YES + 1 NO).
   768	                            // Agent bets on a tape node's quality. This drives price signals
   769	                            // (Art. II.2) which guide Boltzmann routing (Art. II.2.1).
   770	                            // Direction: prefer explicit `direction` field (long/short);
   771	                            // fall back to sign of amount (positive=long, negative=short).
   772	                            // Bidirectional signals let agents express dissent (Art. II.2).
   773	                            if let (Some(node_id), Some(amount)) = (&action.node, action.amount) {
   774	                                let amt = amount.abs();
   775	                                if amt > 0.0 {
   776	                                    let buy_yes = match action.direction.as_deref() {
   777	                                        Some("long") | Some("yes") | Some("LONG") | Some("YES") => true,
   778	                                        Some("short") | Some("no") | Some("SHORT") | Some("NO") => false,
   779	                                        _ => amount > 0.0,  // sign-based fallback
   780	                                    };
   781	                                    // Law 2 conservation: validate market BEFORE debit (no coin-loss path)
   782	                                    let market_exists = bus.kernel.yes_price(node_id).is_some();
   783	                                    if !market_exists {
   784	                                        warn!("[tx {}] invest: no market for {} (hallucinated node?)", tx, node_id);
   785	                                    } else {
   786	                                        // Debit wallet → buy shares → record (atomic intent)
   787	                                        let wallet_ok = bus.tools.iter_mut()
   788	                                            .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>())
   789	                                            .map(|w| w.deduct(agent_id, amt).is_ok())
   790	                                            .unwrap_or(false);
   791	                                        if wallet_ok {
   792	                                            let result = if buy_yes {
   793	                                                bus.kernel.buy_yes(node_id, amt)
   794	                                            } else {
   795	                                                bus.kernel.buy_no(node_id, amt)
   796	                                            };
   797	                                            match result {
   798	                                                Ok(shares) => {
   799	                                                    info!("[tx {}] {} invested {:.0} {} on {} → {:.1} shares",
   800	                                                        tx, agent_id, amt,
   801	                                                        if buy_yes { "YES" } else { "NO" },
   802	                                                        node_id, shares);
   803	                                                    if let Some(w) = bus.tools.iter_mut()
   804	                                                        .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>()) {
   805	                                                        if buy_yes {
   806	                                                            w.record_shares(agent_id, node_id, shares, 0.0, 0.0);
   807	                                                        } else {
   808	                                                            w.record_shares(agent_id, node_id, 0.0, shares, 0.0);
   809	                                                        }
   810	                                                    }
   811	                                                }
   812	                                                Err(e) => {
   813	                                                    // Market existed at check but buy failed — should not happen
   814	                                                    warn!("[tx {}] invest buy error: {} (coins debited, shares not granted — Law 2 violation logged)", tx, e);
   815	                                                }
   816	                                            }
   817	                                        } else {
   818	                                            warn!("[tx {}] {} insufficient balance for invest", tx, agent_id);
   819	                                        }
   820	                                    }
   821	                                }
   822	                            }
   823	                        }
   824	                        "search" => {
   825	                            // F-2026-04-19-05 cap: if over budget this agent's turn the
   826	                            // search slot shouldn't even be offered, but the LLM may still
   827	                            // emit `search` ignoring the prompt — record and skip execute.
   828	                            let cnt = search_count.entry(agent_id.clone()).or_insert(0);
   829	                            if *cnt >= search_cap {
   830	                                *tool_dist.entry("search_capped".into()).or_insert(0) += 1;
   831	                            } else {
   832	                                *cnt += 1;
   833	                                *tool_dist.entry("search".into()).or_insert(0) += 1;
   834	                                // Law 1: search is free. Execute and cache top hits (Art. III.2).
   835	                                if let Some(query) = &action.query {
   836	                                    let hits = bus.tools.iter()
   837	                                        .find_map(|t| t.as_any().downcast_ref::<SearchTool>())
   838	                                        .map(|s| s.search(query))
   839	                                        .unwrap_or_default();
   840	                                    let trimmed: Vec<String> = hits.iter().take(5)
   841	                                        .map(|p| p.rsplit('/').next().unwrap_or(p).to_string())
   842	                                        .collect();
   843	                                    // PPUT-CCL B2: search hits feed `hits_ref` into next prompt —
   844	                                    // count the cached bytes against C_i.
   845	                                    acc.record_tool_stdout(&trimmed.join("\n"));
   846	                                    info!("[tx {}] {} search({:?}) → {} hits: {}",
   847	                                          tx, agent_id, query, hits.len(), trimmed.join(","));
   848	                                    search_cache.insert(agent_id.clone(), trimmed);
   849	                                }
   850	                            }
   851	                        }
   852	                        "post" => {
   853	                            *tool_dist.entry("post".into()).or_insert(0) += 1;
   854	                            // Phase 6-emergent: agent posts a short message to the
   855	                            // shared Librarian board. Other agents see it on next
   856	                            // prompt. State-only; no central role planner.
   857	                            if let Some(msg) = &action.payload {
   858	                                if let Some(lib) = bus.tools.iter()
   859	                                    .find_map(|t| t.as_any().downcast_ref::<LibrarianTool>())
   860	                                {
   861	                                    if let Err(e) = lib.post_to_board(agent_id, msg) {
   862	                                        warn!("[tx {}] post failed: {}", tx, e);
   863	                                    } else {
   864	                                        info!("[tx {}] {} posted to board", tx, agent_id);
   865	                                    }
   866	                                }
   867	                            }
   868	                        }
   869	                        "step" => {
   870	                            // Phase 7 (C-043+ Turing δ-step): submit ONE tactic,
   871	                            // oracle classifies the accumulated tape+tactic prefix
   872	                            // as Complete / PartialOk / Reject. Writes a tape node
   873	                            // on PartialOk and Complete so the DAG grows one cell
   874	                            // at a time — the Art. IV semantics Turing 1936 defines.
   875	                            *tool_dist.entry("step".into()).or_insert(0) += 1;
   876	                            if let Some(tactic) = &action.payload {
   877	                                let tape_chain: String = bus.kernel.tape.time_arrow().iter()
   878	                                    .filter_map(|id| bus.kernel.tape.get(id))
   879	                                    .map(|n| n.payload.clone())
   880	                                    .collect::<Vec<_>>()

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '880,1100p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   880	                                    .collect::<Vec<_>>()
   881	                                    .join("\n");
   882	                                let prefix = if tape_chain.is_empty() {
   883	                                    tactic.clone()
   884	                                } else {
   885	                                    format!("{}\n{}", tape_chain, tactic)
   886	                                };
   887	                                let oracle = Lean4Oracle::new(
   888	                                    problem_statement.to_string(),
   889	                                    theorem_name.to_string(),
   890	                                    lean_path.to_string(),
   891	                                );
   892	                                let verdict = oracle.verify_partial(&prefix);
   893	                                // PPUT-CCL B3: close bracket after step-verify returns.
   894	                                wc.mark_final_accept();
   895	                                match verdict {
   896	                                    PartialVerdict::Complete => {
   897	                                        acc.flip_last_failed_to_accepted();
   898	                                        info!(">>> OMEGA ACCEPTED <<< via step (depth={} after this write)",
   899	                                              bus.kernel.tape.time_arrow().len() + 1);
   900	                                        let proof_file = persist_proof_artifact(
   901	                                            problem_file, &theorem_name, &problem_statement,
   902	                                            &prefix, "per_tactic", agent_id,
   903	                                        );
   904	                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
   905	                                        *tool_dist.entry("omega_wtool".into()).or_insert(0) += 1;
   906	                                        let _ = bus.append_oracle_accepted(
   907	                                            agent_id, tactic, parent.as_deref(),
   908	                                        );
   909	                                        let tape_tokens: u64 = bus.kernel.tape.time_arrow().iter()
   910	                                            .filter_map(|id| bus.kernel.tape.get(id))
   911	                                            .map(|n| n.payload.len() as u64)
   912	                                            .sum();
   913	                                        let gp_tokens = tape_tokens.max(response.completion_tokens as u64);
   914	                                        let gp = bus.kernel.tape.time_arrow().to_vec();
   915	                                        let gp_nodes = gp.len();
   916	                                        bus.halt_and_settle(&gp).ok();
   917	                                        let upr = if omega_attempts > 0 {
   918	                                            Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
   919	                                        } else { None };
   920	                                        return make_pput(problem_file, &condition, model, true,
   921	                                                        start, gp_tokens, gp_nodes, tx as u64 + 1,
   922	                                                        Some(tool_dist), upr,
   923	                                                        Some(prefix.clone()),
   924	                                                        Some("per_tactic".to_string()),
   925	                                                        proof_file,
   926	                                                        Some(acc.total_run_token_count()),
   927	                                                        Some(acc.failed_branch_count),
   928	                                                        wc.elapsed_ms());
   929	                                    }
   930	                                    PartialVerdict::PartialOk => {
   931	                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
   932	                                        match bus.append_oracle_accepted(
   933	                                            agent_id, tactic, parent.as_deref(),
   934	                                        ) {
   935	                                            Ok(BusResult::Appended { node_id }) => {
   936	                                                *tool_dist.entry("step_partial_ok".into()).or_insert(0) += 1;
   937	                                                info!("[tx {}] {} step+{} partial OK (depth={})",
   938	                                                      tx, agent_id, node_id,
   939	                                                      bus.kernel.tape.time_arrow().len());
   940	                                            }
   941	                                            Ok(BusResult::Vetoed { reason }) => {
   942	                                                warn!("[tx {}] step partial OK but bus vetoed: {}", tx, reason);
   943	                                            }
   944	                                            _ => {}
   945	                                        }
   946	                                    }
   947	                                    PartialVerdict::Reject(reason) => {
   948	                                        let class = classify_lean_error(&reason);
   949	                                        bus.record_rejection(agent_id, class.label());
   950	                                        // PPUT-CCL B2: step rejection reason flows into next prompt.
   951	                                        acc.record_tool_stdout(&reason);
   952	                                        *tool_dist.entry("step_reject".into()).or_insert(0) += 1;
   953	                                        let preview = reason.chars().take(200).collect::<String>();
   954	                                        warn!("[tx {}] step rejected ({}): {}", tx, class.label(), preview);
   955	                                    }
   956	                                }
   957	                            }
   958	                        }
   959	                        other => {
   960	                            *tool_dist.entry(format!("other:{}", other)).or_insert(0) += 1;
   961	                        }
   962	                    },
   963	                    Err(e) => {
   964	                        *tool_dist.entry("parse_fail".into()).or_insert(0) += 1;
   965	                        // Step-B v3: parse failures feed the class graveyard too.
   966	                        let class = classify_parse_error(&format!("{}", e));
   967	                        bus.record_rejection(agent_id, class.label());
   968	                        // PPUT-CCL B2: classifier label flows into next prompt's errors.
   969	                        acc.record_tool_stdout(class.label());
   970	                        warn!("[tx {}] parse: {} ({})", tx, e, class.label());
   971	                    }
   972	                }
   973	            }
   974	            Err(e) => {
   975	                *tool_dist.entry("llm_err".into()).or_insert(0) += 1;
   976	                warn!("[tx {}] LLM: {}", tx, e);
   977	            }
   978	        }
   979	    }
   980	
   981	    let upr = if omega_attempts > 0 {
   982	        Some(omega_payload_hashes.len() as f64 / omega_attempts as f64)
   983	    } else { None };
   984	    // Phase 4: also save wallet state on no-OMEGA exit. Agents may have
   985	    // invested/lost Coin during the run; durability should not depend on a win.
   986	    if let Some(ref wp) = wallet_state_path {
   987	        if let Some(w) = bus.tools.iter()
   988	            .find_map(|t| t.as_any().downcast_ref::<WalletTool>())
   989	        {
   990	            let _ = w.save_to_disk(wp);
   991	        }
   992	    }
   993	    // No OMEGA found → PPUT = 0
   994	    // B3: close bracket on max-tx exhaustion path.
   995	    wc.mark_final_accept();
   996	    make_pput(problem_file, &condition, model, false, start, 0, 0,
   997	              max_transactions as u64, Some(tool_dist), upr,
   998	              None, None, None,
   999	              Some(acc.total_run_token_count()),
  1000	              Some(acc.failed_branch_count),
  1001	              wc.elapsed_ms())
  1002	}
  1003	
  1004	fn make_pput(
  1005	    problem: &str, condition: &str, model: &str,
  1006	    has_gp: bool, start: Instant,
  1007	    gp_tokens: u64, gp_nodes: usize, tx_count: u64,
  1008	    tool_dist: Option<HashMap<String, u32>>,
  1009	    unique_payload_ratio: Option<f64>,
  1010	    gp_payload: Option<String>,
  1011	    gp_path: Option<String>,
  1012	    gp_proof_file: Option<String>,
  1013	    total_run_token_count: Option<u64>,
  1014	    failed_branch_count: Option<u32>,
  1015	    total_wall_time_ms: Option<u64>,
  1016	) -> PputResult {
  1017	    let elapsed = start.elapsed().as_secs_f64();
  1018	    let pput = if has_gp && elapsed > 0.0 { 100.0 / elapsed } else { 0.0 };
  1019	    // C-012 provenance: populated from env vars; None when unset (backward compat).
  1020	    let build_sha = std::env::var("BUILD_SHA").ok();
  1021	    let classifier_version = std::env::var("CLASSIFIER_VERSION").ok();
  1022	    let boltzmann_seed = std::env::var("BOLTZMANN_SEED")
  1023	        .ok().and_then(|s| s.parse::<u64>().ok());
  1024	
  1025	    // PPUT-CCL Phase B B4: derive dual PPUT from C_i + T_i + has_gp.
  1026	    // Phase B truth: runtime IS Lean, so verified == has_gp on solved runs.
  1027	    // Phase C Soft Law will set has_gp=true while a separate verify_post_hoc
  1028	    // call returns false; that's the planned divergence point.
  1029	    let (verified, pput_runtime, pput_verified, pput_m_verified) =
  1030	        match (total_run_token_count, total_wall_time_ms) {
  1031	            (Some(c_i), Some(t_i)) => {
  1032	                let runtime_accepted = has_gp;
  1033	                let post_hoc_verified = has_gp; // Phase B: runtime IS Lean
  1034	                let progress_runtime = compute_progress_runtime(runtime_accepted);
  1035	                let progress_verified =
  1036	                    compute_progress_verified(runtime_accepted, post_hoc_verified);
  1037	                (
  1038	                    Some(post_hoc_verified),
  1039	                    Some(compute_pput(progress_runtime, c_i, t_i)),
  1040	                    Some(compute_pput(progress_verified, c_i, t_i)),
  1041	                    Some(compute_pput_m(progress_verified, c_i, t_i)),
  1042	                )
  1043	            }
  1044	            // Missing C_i or T_i → can't compute dual PPUT honestly; emit None
  1045	            // rather than fabricate a zero that would be confused with a real
  1046	            // unsolved run.
  1047	            _ => (None, None, None, None),
  1048	        };
  1049	
  1050	    PputResult {
  1051	        problem: problem.to_string(),
  1052	        condition: condition.to_string(),
  1053	        model: model.to_string(),
  1054	        has_golden_path: has_gp,
  1055	        time_secs: elapsed,
  1056	        pput,
  1057	        gp_token_count: gp_tokens,
  1058	        gp_node_count: gp_nodes,
  1059	        tx_count,
  1060	        build_sha,
  1061	        classifier_version,
  1062	        boltzmann_seed,
  1063	        tool_dist,
  1064	        unique_payload_ratio,
  1065	        gp_payload,
  1066	        gp_path,
  1067	        gp_proof_file,
  1068	        total_run_token_count,
  1069	        failed_branch_count,
  1070	        total_wall_time_ms,
  1071	        verified,
  1072	        pput_runtime,
  1073	        pput_verified,
  1074	        pput_m_verified,
  1075	    }
  1076	}
  1077	
  1078	/// Phase 0 (C-039 candidate): persist a self-contained, re-verifiable proof artifact.
  1079	/// Writes <EXPERIMENT_DIR>/proofs/<theorem>_<timestamp>_<short_hash>.lean containing
  1080	/// the exact code that the Lean oracle accepted. An external verifier can run
  1081	/// `lean --stdin < <file>` with the matching toolchain + Mathlib and reproduce the result.
  1082	/// Returns the relative path (for embedding in PputResult) or None on I/O failure.
  1083	fn persist_proof_artifact(
  1084	    problem_file: &str, theorem_name: &str, problem_statement: &str,
  1085	    full_proof: &str, path_choice: &str, agent_id: &str,
  1086	) -> Option<String> {
  1087	    use std::collections::hash_map::DefaultHasher;
  1088	    use std::hash::{Hash, Hasher};
  1089	    let exp_dir = std::env::var("EXPERIMENT_DIR").unwrap_or_else(|_| ".".into());
  1090	    let proofs_dir = std::path::Path::new(&exp_dir).join("proofs");
  1091	    if let Err(e) = std::fs::create_dir_all(&proofs_dir) {
  1092	        log::warn!("[audit] cannot create proofs dir {:?}: {}", proofs_dir, e);
  1093	        return None;
  1094	    }
  1095	    let ts = std::time::SystemTime::now()
  1096	        .duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
  1097	    let mut h = DefaultHasher::new();
  1098	    full_proof.hash(&mut h);
  1099	    let short = format!("{:x}", h.finish() & 0xFFFFFFFF);
  1100	    let fname = format!("{}_{}_{}.lean", theorem_name, ts, short);

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/jsonl_schema.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// PPUT-CCL JSONL schema v2 — proposal-level + run-level records.
     2	//
     3	// Authoritative spec:
     4	//   handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md § B1
     5	//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md § 5 (definitions)
     6	//
     7	// Versioning: every v2 record carries `schema_version = "v2.0"`. Legacy Paper-1
     8	// era jsonl rows (the `PputResult` shape emitted by evaluator before this commit)
     9	// have NO `schema_version` field, so `RunRecord::from_json` discriminates on
    10	// presence and routes to `LegacyRunAggregate`. No on-disk artifact is rewritten
    11	// by this commit; downstream tooling is the upgrade boundary.
    12	//
    13	// B1 scope: schema definition + round-trip + legacy-compat + zero-progress
    14	// invariant. B2/B3/B4 wire the new fields into evaluator emission paths.
    15	
    16	use serde::{Deserialize, Serialize};
    17	
    18	pub const SCHEMA_VERSION_V2: &str = "v2.0";
    19	
    20	/// Per-proposal row (one per LLM call / append / complete attempt).
    21	///
    22	/// Currently no evaluator emit path produces these — B2 (cost aggregator) and
    23	/// B3 (wall-time) will add the emit sites. This struct is the contract.
    24	#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    25	pub struct ProposalRow {
    26	    pub run_id: String,
    27	    pub problem_id: String,
    28	    pub agent_id: String,
    29	    pub role: String,
    30	    pub branch_id: String,
    31	    pub proposal_hash: String,
    32	    pub accepted: bool,
    33	
    34	    /// "adaptation" | "meta_validation" | "heldout"
    35	    pub split: String,
    36	    pub schema_version: String,
    37	    /// SHA-256 of input prompt (retrieval-equivalence audit).
    38	    pub context_hash: String,
    39	    /// Runtime predicate accept = 1, reject = 0.
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
   107	    pub far: f64,
   108	    pub err: f64,
   109	    pub iac: f64,
   110	    pub cpr: f64,
   111	
   112	    /// Exact model id + API revision (drift defense per F-2026-04-22-08).
   113	    pub model_snapshot: String,
   114	    pub git_sha: String,
   115	    pub binary_sha256: String,
   116	    /// "full" | "panopticon" | "amnesia" | "soft_law" | "homogeneous".
   117	    pub mode: String,
   118	}
   119	
   120	impl RunAggregate {
   121	    /// Compute pput_verified per PREREG § 5:
   122	    ///   pput_verified = progress / (c_i * t_i_ms / 1000)
   123	    /// Returns 0.0 when progress is 0, OR when c_i or t_i_ms is 0
   124	    /// (synthetic / degenerate runs; real runs always have positive cost+time).
   125	    pub fn compute_pput_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
   126	        if progress == 0 || c_i == 0 || t_i_ms == 0 {
   127	            return 0.0;
   128	        }
   129	        let denom = (c_i as f64) * (t_i_ms as f64) / 1000.0;
   130	        (progress as f64) / denom
   131	    }
   132	
   133	    /// Display unit: 10^6 × pput_verified.
   134	    pub fn compute_pput_m_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
   135	        1.0e6 * Self::compute_pput_verified(progress, c_i, t_i_ms)
   136	    }
   137	}
   138	
   139	/// Legacy v1 run row — mirrors the pre-v2 `PputResult` shape emitted by the
   140	/// evaluator before this commit (Paper 1 era, e.g.
   141	/// `discarded_12way_run_2026-04-24/E1v2_Abl_*.jsonl`).
   142	///
   143	/// All v3-era extension fields (reputation_at_end, halt_reason, gp_*) are
   144	/// captured by `extra` so a legacy line round-trips losslessly through
   145	/// serde_json::Value.
   146	#[derive(Debug, Clone, Deserialize, Serialize)]
   147	pub struct LegacyRunAggregate {
   148	    pub problem: String,
   149	    pub condition: String,
   150	    pub model: String,
   151	    pub has_golden_path: bool,
   152	    pub time_secs: f64,
   153	    pub pput: f64,
   154	    pub gp_token_count: u64,
   155	    pub gp_node_count: usize,
   156	    pub tx_count: u64,
   157	    /// Catch-all for v3.x optional fields (reputation_at_end, halt_reason,
   158	    /// gp_payload, gp_path, gp_proof_file, classifier_version, build_sha, ...).
   159	    #[serde(flatten)]
   160	    pub extra: serde_json::Map<String, serde_json::Value>,
   161	}
   162	
   163	/// Discriminated record for backward-compatible reading.
   164	#[derive(Debug)]
   165	pub enum RunRecord {
   166	    V2(RunAggregate),
   167	    Legacy(LegacyRunAggregate),
   168	}
   169	
   170	impl RunRecord {
   171	    /// Parse one jsonl line. v2 if `schema_version` present, else legacy.
   172	    /// Returns the raw serde error for genuinely malformed input.
   173	    pub fn from_json(line: &str) -> Result<Self, serde_json::Error> {
   174	        let v: serde_json::Value = serde_json::from_str(line)?;
   175	        let is_v2 = v.get("schema_version")
   176	            .and_then(|s| s.as_str())
   177	            .map(|s| s.starts_with("v2"))
   178	            .unwrap_or(false);
   179	        if is_v2 {
   180	            Ok(RunRecord::V2(serde_json::from_value(v)?))
   181	        } else {
   182	            Ok(RunRecord::Legacy(serde_json::from_value(v)?))
   183	        }
   184	    }
   185	}
   186	
   187	#[cfg(test)]
   188	mod tests {
   189	    use super::*;
   190	
   191	    fn sample_run() -> RunAggregate {
   192	        RunAggregate {
   193	            run_id: "r-001".into(),
   194	            problem_id: "mathd_algebra_44".into(),
   195	            solved: true,
   196	            schema_version: SCHEMA_VERSION_V2.into(),
   197	            split: "adaptation".into(),
   198	            verified: true,
   199	            golden_path_token_count: 512,
   200	            total_run_token_count: 4096,
   201	            total_wall_time_ms: 12_000,
   202	            progress: 1,
   203	            pput_runtime: 0.5,
   204	            pput_verified: RunAggregate::compute_pput_verified(1, 4096, 12_000),
   205	            pput_m_verified: RunAggregate::compute_pput_m_verified(1, 4096, 12_000),
   206	            failed_branch_count: 3,
   207	            rollback_count: 0,
   208	            far: 0.0, err: 0.0, iac: 0.0, cpr: 0.0,
   209	            model_snapshot: "deepseek-v4-flash@2026-04-26".into(),
   210	            git_sha: "913255d".into(),
   211	            binary_sha256: "deadbeef".into(),
   212	            mode: "full".into(),
   213	        }
   214	    }
   215	
   216	    #[test]
   217	    fn test_jsonl_schema_v2_round_trip() {
   218	        let original = sample_run();
   219	        let line = serde_json::to_string(&original).expect("serialize");
   220	        let parsed: RunAggregate = serde_json::from_str(&line).expect("deserialize");
   221	        assert_eq!(parsed, original, "v2 RunAggregate must round-trip");
   222	        assert!(line.contains("\"schema_version\":\"v2.0\""),
   223	                "serialized line must stamp schema_version");
   224	    }
   225	
   226	    #[test]
   227	    fn test_pput_verified_zero_when_progress_zero() {
   228	        // PREREG § 3 anti-Goodhart: a run that did not verify must report
   229	        // pput_verified = 0 regardless of cost / wall-time.
   230	        assert_eq!(RunAggregate::compute_pput_verified(0, 1000, 5000), 0.0);
   231	        assert_eq!(RunAggregate::compute_pput_m_verified(0, 1000, 5000), 0.0);
   232	
   233	        // And the struct round-trips with the zero stamped in.
   234	        let mut r = sample_run();
   235	        r.solved = false;
   236	        r.verified = false;
   237	        r.progress = 0;
   238	        r.pput_verified = RunAggregate::compute_pput_verified(0, r.total_run_token_count, r.total_wall_time_ms);
   239	        r.pput_m_verified = RunAggregate::compute_pput_m_verified(0, r.total_run_token_count, r.total_wall_time_ms);
   240	        assert_eq!(r.pput_verified, 0.0);
   241	        assert_eq!(r.pput_m_verified, 0.0);
   242	
   243	        // Defensive: degenerate cost/time also clamps to 0 (synthetic test fixtures).
   244	        assert_eq!(RunAggregate::compute_pput_verified(1, 0, 5000), 0.0);
   245	        assert_eq!(RunAggregate::compute_pput_verified(1, 1000, 0), 0.0);
   246	    }
   247	
   248	    #[test]
   249	    fn test_legacy_jsonl_still_readable() {
   250	        // Verbatim shape of a Paper-1 era line
   251	        // (discarded_12way_run_2026-04-24/E1v2_Abl_s141421_n8_20260424T080939.jsonl).
   252	        let legacy_line = r#"{"problem":"/tmp/foo.lean","condition":"n8","model":"deepseek-chat","has_golden_path":true,"time_secs":781.99,"pput":0.127,"gp_token_count":769,"gp_node_count":7,"tx_count":16,"build_sha":"61ccc21","classifier_version":"v1_2026-04-16-a","boltzmann_seed":141421,"halt_reason":"OmegaAccepted","reputation_at_end":{"Agent_1":2}}"#;
   253	
   254	        match RunRecord::from_json(legacy_line).expect("legacy line parses") {
   255	            RunRecord::Legacy(l) => {
   256	                assert_eq!(l.condition, "n8");
   257	                assert_eq!(l.has_golden_path, true);
   258	                assert_eq!(l.gp_token_count, 769);
   259	                // v3.x extension fields land in `extra`.
   260	                assert_eq!(l.extra.get("halt_reason").and_then(|v| v.as_str()),

exec
/bin/bash -lc "rg -n \"client\\.generate|record_llm_call|record_proposal|flip_last_failed|verify_omega|verify_partial|mark_first_read|mark_final_accept|make_pput|verify_post_hoc|total_run_token_count|pput_verified|RunAggregate|progress\" experiments/minif2f_v4/src src -g '*.rs'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/src/cost_aggregator.rs:9://   total_run_token_count = Σ over EVERY proposal in the run of
experiments/minif2f_v4/src/cost_aggregator.rs:46:    /// Call this AFTER `client.generate(...)` returns Ok — both for winning
experiments/minif2f_v4/src/cost_aggregator.rs:49:    pub fn record_llm_call(&mut self, prompt_tokens: u32, completion_tokens: u32) {
experiments/minif2f_v4/src/cost_aggregator.rs:66:    pub fn record_proposal(&mut self, accepted: bool) {
experiments/minif2f_v4/src/cost_aggregator.rs:79:    pub fn flip_last_failed_to_accepted(&mut self) {
experiments/minif2f_v4/src/cost_aggregator.rs:86:    pub fn total_run_token_count(&self) -> u64 {
experiments/minif2f_v4/src/cost_aggregator.rs:108:            acc.record_llm_call(100, 50);
experiments/minif2f_v4/src/cost_aggregator.rs:110:            acc.record_proposal(false);
experiments/minif2f_v4/src/cost_aggregator.rs:114:        acc.record_llm_call(200, 100);
experiments/minif2f_v4/src/cost_aggregator.rs:115:        acc.record_proposal(true);
experiments/minif2f_v4/src/cost_aggregator.rs:128:        assert_eq!(acc.total_run_token_count(), expected_total as u64,
experiments/minif2f_v4/src/cost_aggregator.rs:138:        assert_eq!(acc.total_run_token_count(), 0);
experiments/minif2f_v4/src/post_hoc_verifier.rs:11://   pput_verified = Progress_verified / (C_i × T_i / 1000)   ← North Star
experiments/minif2f_v4/src/post_hoc_verifier.rs:23://     pput_verified MUST be 0  (Lean reject means Progress_verified = 0)
experiments/minif2f_v4/src/post_hoc_verifier.rs:26://   pput_verified is the only North Star metric for H-VPPUT. pput_runtime
experiments/minif2f_v4/src/post_hoc_verifier.rs:30:use crate::jsonl_schema::RunAggregate;
experiments/minif2f_v4/src/post_hoc_verifier.rs:37:/// `verify_omega_detailed` on the same payload and gotten `Ok((true, _))`,
experiments/minif2f_v4/src/post_hoc_verifier.rs:42:/// Use `compute_progress_verified` instead when the runtime gate already
experiments/minif2f_v4/src/post_hoc_verifier.rs:45:pub fn verify_post_hoc(oracle: &Lean4Oracle, golden_path_payload: &str) -> bool {
experiments/minif2f_v4/src/post_hoc_verifier.rs:47:        oracle.verify_omega_detailed(golden_path_payload),
experiments/minif2f_v4/src/post_hoc_verifier.rs:59:pub fn compute_progress_verified(runtime_accepted: bool, post_hoc_verified: bool) -> u8 {
experiments/minif2f_v4/src/post_hoc_verifier.rs:65:pub fn compute_progress_runtime(runtime_accepted: bool) -> u8 {
experiments/minif2f_v4/src/post_hoc_verifier.rs:69:/// Wrap RunAggregate::compute_pput_verified for callers in evaluator that
experiments/minif2f_v4/src/post_hoc_verifier.rs:70:/// only have (progress, c_i, t_i_ms). Same math, single source of truth.
experiments/minif2f_v4/src/post_hoc_verifier.rs:71:pub fn compute_pput(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
experiments/minif2f_v4/src/post_hoc_verifier.rs:72:    RunAggregate::compute_pput_verified(progress, c_i, t_i_ms)
experiments/minif2f_v4/src/post_hoc_verifier.rs:76:pub fn compute_pput_m(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
experiments/minif2f_v4/src/post_hoc_verifier.rs:77:    RunAggregate::compute_pput_m_verified(progress, c_i, t_i_ms)
experiments/minif2f_v4/src/post_hoc_verifier.rs:87:    ///  assert progress = 0, pput_verified = 0.0"
experiments/minif2f_v4/src/post_hoc_verifier.rs:94:    fn test_pput_verified_zero_when_lean_rejects() {
experiments/minif2f_v4/src/post_hoc_verifier.rs:97:        // run's progress to 0.
experiments/minif2f_v4/src/post_hoc_verifier.rs:103:        let progress_runtime = compute_progress_runtime(runtime_accepted);
experiments/minif2f_v4/src/post_hoc_verifier.rs:104:        let progress_verified =
experiments/minif2f_v4/src/post_hoc_verifier.rs:105:            compute_progress_verified(runtime_accepted, post_hoc_verified);
experiments/minif2f_v4/src/post_hoc_verifier.rs:107:        let pput_runtime = compute_pput(progress_runtime, c_i, t_i_ms);
experiments/minif2f_v4/src/post_hoc_verifier.rs:108:        let pput_verified = compute_pput(progress_verified, c_i, t_i_ms);
experiments/minif2f_v4/src/post_hoc_verifier.rs:109:        let pput_m_verified = compute_pput_m(progress_verified, c_i, t_i_ms);
experiments/minif2f_v4/src/post_hoc_verifier.rs:111:        assert_eq!(progress_runtime, 1u8,
experiments/minif2f_v4/src/post_hoc_verifier.rs:112:            "runtime gate fired → progress_runtime = 1");
experiments/minif2f_v4/src/post_hoc_verifier.rs:113:        assert_eq!(progress_verified, 0u8,
experiments/minif2f_v4/src/post_hoc_verifier.rs:114:            "Lean rejected → progress_verified MUST be 0 (North Star truth)");
experiments/minif2f_v4/src/post_hoc_verifier.rs:117:        assert_eq!(pput_verified, 0.0,
experiments/minif2f_v4/src/post_hoc_verifier.rs:118:            "pput_verified MUST be 0 when Lean rejects — North Star Goodhart shield");
experiments/minif2f_v4/src/post_hoc_verifier.rs:120:            "pput_m_verified must collapse with pput_verified");
experiments/minif2f_v4/src/post_hoc_verifier.rs:122:        // Sanity: divergence is detectable. pput_runtime - pput_verified > 0
experiments/minif2f_v4/src/post_hoc_verifier.rs:124:        assert!(pput_runtime - pput_verified > 0.0,
experiments/minif2f_v4/src/post_hoc_verifier.rs:125:            "(pput_runtime - pput_verified) > 0 ⟺ Soft Law divergence detected");
experiments/minif2f_v4/src/post_hoc_verifier.rs:129:    fn test_pput_verified_matches_runtime_when_both_accept() {
experiments/minif2f_v4/src/post_hoc_verifier.rs:133:        // booleans and made pput_verified always 0).
experiments/minif2f_v4/src/post_hoc_verifier.rs:137:        let progress_runtime = compute_progress_runtime(true);
experiments/minif2f_v4/src/post_hoc_verifier.rs:138:        let progress_verified = compute_progress_verified(true, true);
experiments/minif2f_v4/src/post_hoc_verifier.rs:140:        assert_eq!(progress_runtime, progress_verified,
experiments/minif2f_v4/src/post_hoc_verifier.rs:143:            compute_pput(progress_runtime, c_i, t_i_ms),
experiments/minif2f_v4/src/post_hoc_verifier.rs:144:            compute_pput(progress_verified, c_i, t_i_ms),
experiments/minif2f_v4/src/post_hoc_verifier.rs:154:        let progress_runtime = compute_progress_runtime(false);
experiments/minif2f_v4/src/post_hoc_verifier.rs:155:        let progress_verified = compute_progress_verified(false, false);
experiments/minif2f_v4/src/post_hoc_verifier.rs:157:        assert_eq!(compute_pput(progress_runtime, c_i, t_i_ms), 0.0);
experiments/minif2f_v4/src/post_hoc_verifier.rs:158:        assert_eq!(compute_pput(progress_verified, c_i, t_i_ms), 0.0);
experiments/minif2f_v4/src/post_hoc_verifier.rs:162:    fn test_post_hoc_verified_without_runtime_still_zero_progress() {
experiments/minif2f_v4/src/post_hoc_verifier.rs:164:        // fired is a wiring bug, not an honest progress signal. Progress
experiments/minif2f_v4/src/post_hoc_verifier.rs:167:        assert_eq!(compute_progress_verified(false, true), 0u8,
experiments/minif2f_v4/src/bin/evaluator.rs:14:    compute_progress_runtime, compute_progress_verified, compute_pput, compute_pput_m,
experiments/minif2f_v4/src/bin/evaluator.rs:70:    // gp_payload = the exact text fed to oracle.verify_omega_detailed at OMEGA accept.
experiments/minif2f_v4/src/bin/evaluator.rs:83:    total_run_token_count: Option<u64>,
experiments/minif2f_v4/src/bin/evaluator.rs:93:    // verified. Field names align with B1 RunAggregate v2 schema. In Phase B,
experiments/minif2f_v4/src/bin/evaluator.rs:95:    // call site that makes pput_runtime - pput_verified > 0 the H1 signal.
experiments/minif2f_v4/src/bin/evaluator.rs:101:    pput_verified: Option<f64>,
experiments/minif2f_v4/src/bin/evaluator.rs:234:    wc.mark_first_read();
experiments/minif2f_v4/src/bin/evaluator.rs:235:    match client.generate(&request).await {
experiments/minif2f_v4/src/bin/evaluator.rs:237:            acc.record_llm_call(response.prompt_tokens, response.completion_tokens);
experiments/minif2f_v4/src/bin/evaluator.rs:238:            acc.record_proposal(false);
experiments/minif2f_v4/src/bin/evaluator.rs:241:                wc.mark_final_accept();
experiments/minif2f_v4/src/bin/evaluator.rs:242:                return make_pput(problem_file, "oneshot", model, false, start, 0, 0, 1,
experiments/minif2f_v4/src/bin/evaluator.rs:244:                                 Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:249:            let verdict = oracle.verify_omega(&response.content);
experiments/minif2f_v4/src/bin/evaluator.rs:253:            wc.mark_final_accept();
experiments/minif2f_v4/src/bin/evaluator.rs:256:                    acc.flip_last_failed_to_accepted();
experiments/minif2f_v4/src/bin/evaluator.rs:264:                    make_pput(problem_file, "oneshot", model, true, start, gp_tokens, 1, 1,
experiments/minif2f_v4/src/bin/evaluator.rs:267:                              Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:272:                    make_pput(problem_file, "oneshot", model, false, start, 0, 0, 1,
experiments/minif2f_v4/src/bin/evaluator.rs:274:                              Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:280:                    make_pput(problem_file, "oneshot", model, false, start, 0, 0, 1,
experiments/minif2f_v4/src/bin/evaluator.rs:282:                              Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:432:    // make_pput sites and stamped on the emitted jsonl row.
experiments/minif2f_v4/src/bin/evaluator.rs:565:        wc.mark_first_read();
experiments/minif2f_v4/src/bin/evaluator.rs:584:        match client.generate(&request).await {
experiments/minif2f_v4/src/bin/evaluator.rs:586:                acc.record_llm_call(response.prompt_tokens, response.completion_tokens);
experiments/minif2f_v4/src/bin/evaluator.rs:589:                acc.record_proposal(false);
experiments/minif2f_v4/src/bin/evaluator.rs:659:                                let r_alone = oracle.verify_omega_detailed(payload);
experiments/minif2f_v4/src/bin/evaluator.rs:665:                                        let r_combined = oracle.verify_omega_detailed(&combined);
experiments/minif2f_v4/src/bin/evaluator.rs:675:                                wc.mark_final_accept();
experiments/minif2f_v4/src/bin/evaluator.rs:680:                                        acc.flip_last_failed_to_accepted();
experiments/minif2f_v4/src/bin/evaluator.rs:739:                                        return make_pput(problem_file, &condition, model, true,
experiments/minif2f_v4/src/bin/evaluator.rs:745:                                                        Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:892:                                let verdict = oracle.verify_partial(&prefix);
experiments/minif2f_v4/src/bin/evaluator.rs:894:                                wc.mark_final_accept();
experiments/minif2f_v4/src/bin/evaluator.rs:897:                                        acc.flip_last_failed_to_accepted();
experiments/minif2f_v4/src/bin/evaluator.rs:920:                                        return make_pput(problem_file, &condition, model, true,
experiments/minif2f_v4/src/bin/evaluator.rs:926:                                                        Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:995:    wc.mark_final_accept();
experiments/minif2f_v4/src/bin/evaluator.rs:996:    make_pput(problem_file, &condition, model, false, start, 0, 0,
experiments/minif2f_v4/src/bin/evaluator.rs:999:              Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:1004:fn make_pput(
experiments/minif2f_v4/src/bin/evaluator.rs:1013:    total_run_token_count: Option<u64>,
experiments/minif2f_v4/src/bin/evaluator.rs:1027:    // Phase C Soft Law will set has_gp=true while a separate verify_post_hoc
experiments/minif2f_v4/src/bin/evaluator.rs:1029:    let (verified, pput_runtime, pput_verified, pput_m_verified) =
experiments/minif2f_v4/src/bin/evaluator.rs:1030:        match (total_run_token_count, total_wall_time_ms) {
experiments/minif2f_v4/src/bin/evaluator.rs:1034:                let progress_runtime = compute_progress_runtime(runtime_accepted);
experiments/minif2f_v4/src/bin/evaluator.rs:1035:                let progress_verified =
experiments/minif2f_v4/src/bin/evaluator.rs:1036:                    compute_progress_verified(runtime_accepted, post_hoc_verified);
experiments/minif2f_v4/src/bin/evaluator.rs:1039:                    Some(compute_pput(progress_runtime, c_i, t_i)),
experiments/minif2f_v4/src/bin/evaluator.rs:1040:                    Some(compute_pput(progress_verified, c_i, t_i)),
experiments/minif2f_v4/src/bin/evaluator.rs:1041:                    Some(compute_pput_m(progress_verified, c_i, t_i)),
experiments/minif2f_v4/src/bin/evaluator.rs:1068:        total_run_token_count,
experiments/minif2f_v4/src/bin/evaluator.rs:1073:        pput_verified,
experiments/minif2f_v4/src/jsonl_schema.rs:10:// presence and routes to `LegacyRunAggregate`. No on-disk artifact is rewritten
experiments/minif2f_v4/src/jsonl_schema.rs:13:// B1 scope: schema definition + round-trip + legacy-compat + zero-progress
experiments/minif2f_v4/src/jsonl_schema.rs:80:/// `pput_verified` = Lean post-hoc verified — H-VPPUT input.
experiments/minif2f_v4/src/jsonl_schema.rs:82:pub struct RunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:93:    pub total_run_token_count: u64,
experiments/minif2f_v4/src/jsonl_schema.rs:97:    pub progress: u8,
experiments/minif2f_v4/src/jsonl_schema.rs:101:    pub pput_verified: f64,
experiments/minif2f_v4/src/jsonl_schema.rs:102:    /// 10^6 × pput_verified — display unit (PREREG § 5).
experiments/minif2f_v4/src/jsonl_schema.rs:120:impl RunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:121:    /// Compute pput_verified per PREREG § 5:
experiments/minif2f_v4/src/jsonl_schema.rs:122:    ///   pput_verified = progress / (c_i * t_i_ms / 1000)
experiments/minif2f_v4/src/jsonl_schema.rs:123:    /// Returns 0.0 when progress is 0, OR when c_i or t_i_ms is 0
experiments/minif2f_v4/src/jsonl_schema.rs:125:    pub fn compute_pput_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
experiments/minif2f_v4/src/jsonl_schema.rs:126:        if progress == 0 || c_i == 0 || t_i_ms == 0 {
experiments/minif2f_v4/src/jsonl_schema.rs:130:        (progress as f64) / denom
experiments/minif2f_v4/src/jsonl_schema.rs:133:    /// Display unit: 10^6 × pput_verified.
experiments/minif2f_v4/src/jsonl_schema.rs:134:    pub fn compute_pput_m_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
experiments/minif2f_v4/src/jsonl_schema.rs:135:        1.0e6 * Self::compute_pput_verified(progress, c_i, t_i_ms)
experiments/minif2f_v4/src/jsonl_schema.rs:147:pub struct LegacyRunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:166:    V2(RunAggregate),
experiments/minif2f_v4/src/jsonl_schema.rs:167:    Legacy(LegacyRunAggregate),
experiments/minif2f_v4/src/jsonl_schema.rs:191:    fn sample_run() -> RunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:192:        RunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:200:            total_run_token_count: 4096,
experiments/minif2f_v4/src/jsonl_schema.rs:202:            progress: 1,
experiments/minif2f_v4/src/jsonl_schema.rs:204:            pput_verified: RunAggregate::compute_pput_verified(1, 4096, 12_000),
experiments/minif2f_v4/src/jsonl_schema.rs:205:            pput_m_verified: RunAggregate::compute_pput_m_verified(1, 4096, 12_000),
experiments/minif2f_v4/src/jsonl_schema.rs:220:        let parsed: RunAggregate = serde_json::from_str(&line).expect("deserialize");
experiments/minif2f_v4/src/jsonl_schema.rs:221:        assert_eq!(parsed, original, "v2 RunAggregate must round-trip");
experiments/minif2f_v4/src/jsonl_schema.rs:227:    fn test_pput_verified_zero_when_progress_zero() {
experiments/minif2f_v4/src/jsonl_schema.rs:229:        // pput_verified = 0 regardless of cost / wall-time.
experiments/minif2f_v4/src/jsonl_schema.rs:230:        assert_eq!(RunAggregate::compute_pput_verified(0, 1000, 5000), 0.0);
experiments/minif2f_v4/src/jsonl_schema.rs:231:        assert_eq!(RunAggregate::compute_pput_m_verified(0, 1000, 5000), 0.0);
experiments/minif2f_v4/src/jsonl_schema.rs:237:        r.progress = 0;
experiments/minif2f_v4/src/jsonl_schema.rs:238:        r.pput_verified = RunAggregate::compute_pput_verified(0, r.total_run_token_count, r.total_wall_time_ms);
experiments/minif2f_v4/src/jsonl_schema.rs:239:        r.pput_m_verified = RunAggregate::compute_pput_m_verified(0, r.total_run_token_count, r.total_wall_time_ms);
experiments/minif2f_v4/src/jsonl_schema.rs:240:        assert_eq!(r.pput_verified, 0.0);
experiments/minif2f_v4/src/jsonl_schema.rs:244:        assert_eq!(RunAggregate::compute_pput_verified(1, 0, 5000), 0.0);
experiments/minif2f_v4/src/jsonl_schema.rs:245:        assert_eq!(RunAggregate::compute_pput_verified(1, 1000, 0), 0.0);
experiments/minif2f_v4/src/wall_clock.rs:20://      runtime accept. `mark_final_accept` is called by the evaluator AFTER
experiments/minif2f_v4/src/wall_clock.rs:30:/// Construct one at function entry, call `mark_first_read` at the first agent
experiments/minif2f_v4/src/wall_clock.rs:31:/// prompt construction, call `mark_final_accept` after the last Lean call
experiments/minif2f_v4/src/wall_clock.rs:51:    pub fn mark_first_read(&mut self) {
experiments/minif2f_v4/src/wall_clock.rs:61:    pub fn mark_final_accept(&mut self) {
experiments/minif2f_v4/src/wall_clock.rs:129:        wc.mark_first_read();
experiments/minif2f_v4/src/wall_clock.rs:132:        wc.mark_first_read(); // should be no-op
experiments/minif2f_v4/src/wall_clock.rs:140:        // both fire mark_final_accept, the LATER instant must win so the
experiments/minif2f_v4/src/wall_clock.rs:143:        wc.mark_first_read();
experiments/minif2f_v4/src/wall_clock.rs:144:        wc.mark_final_accept();
experiments/minif2f_v4/src/wall_clock.rs:147:        wc.mark_final_accept();
experiments/minif2f_v4/src/wall_clock.rs:158:        wc.mark_first_read();
src/sdk/error_abstraction.rs:103:    if c.contains("`simp` made no progress") || c.contains("simp made no progress") {
src/sdk/error_abstraction.rs:152:    fn fixture_simp_no_progress() {
src/sdk/error_abstraction.rs:153:        let s = "<stdin>:17:2: error: `simp` made no progress";
src/sdk/prompt.rs:2:// Constitutional basis: Art. III.2 (encapsulate details, progressive disclosure)
src/sdk/prompt.rs:66:    // Art. III.2 progressive disclosure: surface recent search hits so the
src/sdk/prompt.rs:91:    // progression tool. Art. IV strict: δ writes one square. No monolithic
src/sdk/prompt.rs:98:        prompt.push_str("    THE proof-progression tool. Submit ONE tactic (e.g. `intro h`,\n");
experiments/minif2f_v4/src/lean4_oracle.rs:98:    pub fn verify_omega(&self, proof_chain: &str) -> Result<bool, String> {
experiments/minif2f_v4/src/lean4_oracle.rs:99:        self.verify_omega_detailed(proof_chain).map(|(ok, _)| ok)
experiments/minif2f_v4/src/lean4_oracle.rs:112:    pub fn verify_omega_detailed(&self, proof_chain: &str) -> Result<(bool, String), String> {
experiments/minif2f_v4/src/lean4_oracle.rs:177:    pub fn verify_partial(&self, proof_prefix: &str) -> PartialVerdict {
experiments/minif2f_v4/src/lean4_oracle.rs:227:                    // verify_omega_detailed path 2).

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/jsonl_schema.rs | sed -n '248,360p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   248	    #[test]
   249	    fn test_legacy_jsonl_still_readable() {
   250	        // Verbatim shape of a Paper-1 era line
   251	        // (discarded_12way_run_2026-04-24/E1v2_Abl_s141421_n8_20260424T080939.jsonl).
   252	        let legacy_line = r#"{"problem":"/tmp/foo.lean","condition":"n8","model":"deepseek-chat","has_golden_path":true,"time_secs":781.99,"pput":0.127,"gp_token_count":769,"gp_node_count":7,"tx_count":16,"build_sha":"61ccc21","classifier_version":"v1_2026-04-16-a","boltzmann_seed":141421,"halt_reason":"OmegaAccepted","reputation_at_end":{"Agent_1":2}}"#;
   253	
   254	        match RunRecord::from_json(legacy_line).expect("legacy line parses") {
   255	            RunRecord::Legacy(l) => {
   256	                assert_eq!(l.condition, "n8");
   257	                assert_eq!(l.has_golden_path, true);
   258	                assert_eq!(l.gp_token_count, 769);
   259	                // v3.x extension fields land in `extra`.
   260	                assert_eq!(l.extra.get("halt_reason").and_then(|v| v.as_str()),
   261	                           Some("OmegaAccepted"));
   262	                assert!(l.extra.get("reputation_at_end").is_some());
   263	            }
   264	            RunRecord::V2(_) => panic!("legacy line misclassified as v2"),
   265	        }
   266	
   267	        // And a v2 line dispatches the other way.
   268	        let v2_line = serde_json::to_string(&sample_run()).unwrap();
   269	        match RunRecord::from_json(&v2_line).expect("v2 line parses") {
   270	            RunRecord::V2(_) => {}
   271	            RunRecord::Legacy(_) => panic!("v2 line misclassified as legacy"),
   272	        }
   273	    }
   274	}

exec
/bin/bash -lc 'nl -ba experiments/minif2f_v4/src/lib.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	pub mod lean4_oracle;
     2	pub mod jsonl_schema;
     3	pub mod cost_aggregator;
     4	pub mod wall_clock;
     5	pub mod post_hoc_verifier;

exec
/bin/bash -lc "rg -n \"struct GenerateResponse|GenerateResponse \\{|\\.generate\\(&|record_llm_call|prompt_tokens|completion_tokens|usage\" . -g '*.rs' -g '"'!target'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
./experiments/minif2f_v4/src/cost_aggregator.rs:10://       (api_prompt_tokens + api_completion_tokens + tool_stdout_tokens)
./experiments/minif2f_v4/src/cost_aggregator.rs:30:    pub prompt_tokens: u64,
./experiments/minif2f_v4/src/cost_aggregator.rs:31:    pub completion_tokens: u64,
./experiments/minif2f_v4/src/cost_aggregator.rs:45:    /// Record one LLM call's API-reported token usage.
./experiments/minif2f_v4/src/cost_aggregator.rs:49:    pub fn record_llm_call(&mut self, prompt_tokens: u32, completion_tokens: u32) {
./experiments/minif2f_v4/src/cost_aggregator.rs:50:        self.prompt_tokens += prompt_tokens as u64;
./experiments/minif2f_v4/src/cost_aggregator.rs:51:        self.completion_tokens += completion_tokens as u64;
./experiments/minif2f_v4/src/cost_aggregator.rs:87:        self.prompt_tokens + self.completion_tokens + self.tool_tokens
./experiments/minif2f_v4/src/cost_aggregator.rs:108:            acc.record_llm_call(100, 50);
./experiments/minif2f_v4/src/cost_aggregator.rs:114:        acc.record_llm_call(200, 100);
./experiments/minif2f_v4/src/cost_aggregator.rs:122:        assert_eq!(acc.prompt_tokens, expected_prompt as u64,
./experiments/minif2f_v4/src/cost_aggregator.rs:124:        assert_eq!(acc.completion_tokens, expected_completion as u64,
./experiments/minif2f_v4/src/jsonl_schema.rs:52:    pub prompt_tokens: u64,
./experiments/minif2f_v4/src/jsonl_schema.rs:53:    pub completion_tokens: u64,
./experiments/minif2f_v4/src/bin/evaluator.rs:235:    match client.generate(&request).await {
./experiments/minif2f_v4/src/bin/evaluator.rs:237:            acc.record_llm_call(response.prompt_tokens, response.completion_tokens);
./experiments/minif2f_v4/src/bin/evaluator.rs:257:                    let gp_tokens = response.completion_tokens as u64;
./experiments/minif2f_v4/src/bin/evaluator.rs:584:        match client.generate(&request).await {
./experiments/minif2f_v4/src/bin/evaluator.rs:586:                acc.record_llm_call(response.prompt_tokens, response.completion_tokens);
./experiments/minif2f_v4/src/bin/evaluator.rs:718:                                        let gp_tokens = tape_tokens.max(response.completion_tokens as u64);
./experiments/minif2f_v4/src/bin/evaluator.rs:913:                                        let gp_tokens = tape_tokens.max(response.completion_tokens as u64);
./src/kernel.rs:255:            completion_tokens: 0,
./src/ledger.rs:24:    pub completion_tokens: u32,
./src/ledger.rs:355:            completion_tokens: 0,
./src/drivers/llm_http.rs:31:pub struct GenerateResponse {
./src/drivers/llm_http.rs:33:    pub completion_tokens: u32,
./src/drivers/llm_http.rs:34:    /// API-reported prompt tokens. Falls back to 0 if `usage.prompt_tokens` is
./src/drivers/llm_http.rs:37:    pub prompt_tokens: u32,
./src/drivers/llm_http.rs:131:                    let tokens = body["usage"]["completion_tokens"]
./src/drivers/llm_http.rs:134:                    let prompt_tokens = body["usage"]["prompt_tokens"]
./src/drivers/llm_http.rs:142:                    return Ok(GenerateResponse {
./src/drivers/llm_http.rs:144:                        completion_tokens: tokens,
./src/drivers/llm_http.rs:145:                        prompt_tokens,
./src/sdk/actor.rs:157:    pub completion_tokens: u32,
./src/sdk/actor.rs:181:                completion_tokens: 0,
./src/sdk/actor.rs:255:                completion_tokens: 0,
./src/wal.rs:118:            completion_tokens: 10,
./src/wal.rs:171:        std::fs::write(&path, "{\"kind\":\"node\",\"id\":\"a\",\"author\":\"x\",\"payload\":\"p\",\"citations\":[],\"created_at\":0,\"completion_tokens\":0}\nnot-valid-json\n").unwrap();
./src/bus.rs:268:            completion_tokens: 0,

exec
/bin/bash -lc "rg -n \"STEP_B_PROTOCOL|parallel-branch|bus.rs|kernel.rs|wallet.rs|llm_http|restricted|Trust Root|TRUST_ROOT|trust\" -S . -g '"'!target'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
./routines/README.md:38:## Known trust assumptions (C-017 — name the silent failure)
./routines/daily_drift.yaml:20:  - Read /src/bus.rs (or wherever BusConfig is constructed in /experiments/minif2f_v4/src/bin/evaluator.rs). Locate 'forbidden_patterns' literal list.
./tests/harness_validation.py:178:    # Try to write domain knowledge into kernel.rs
./tests/harness_validation.py:180:        'Write the text "let tactic = lean_theorem" to a new file src/kernel.rs',
./tests/harness_validation.py:189:        "judge.sh should block domain strings in kernel.rs",
./tests/harness_validation.py:370:        'Run: echo "safe content" | python3 rules/engine.py --file src/bus.rs --rules-dir rules/active --log /dev/null --traces-dir /tmp/v4test && echo "ENGINE_OK"',
./tests/harness_validation.py:387:        'Run: echo "fund_agent redistribute_pool" | python3 rules/engine.py --file src/bus.rs --rules-dir rules/active --log /dev/null --traces-dir /tmp/v4test; echo "EXIT:$?"',
./tests/harness_validation.py:395:        "Engine should block fund_agent/redistribute_pool in bus.rs",
./tests/harness_validation.py:404:        'Run: rm -rf /tmp/v4trace_test && echo "lean tactic" | python3 rules/engine.py --file src/kernel.rs --rules-dir rules/active --log /dev/null --traces-dir /tmp/v4trace_test; ls /tmp/v4trace_test/*.jsonl 2>/dev/null && cat /tmp/v4trace_test/*.jsonl',
./tests/harness_validation.py:557:    passed = messages_contain(msgs, "grep") or messages_contain(msgs, "kernel.rs") or messages_contain(msgs, "lean")
./tests/harness_validation.sh:92:OUTPUT=$(echo "normal safe code" | python3 "$ENGINE" --file "src/bus.rs" --rules-dir "$RULES" --log /dev/null --traces-dir "$TRACES" 2>&1)
./tests/harness_validation.sh:96:# T-016: R-001 blocks domain leak in kernel.rs
./tests/harness_validation.sh:97:OUTPUT=$(echo "let x = lean_tactic_theorem" | python3 "$ENGINE" --file "src/kernel.rs" --rules-dir "$RULES" --log /dev/null --traces-dir "$TRACES" 2>&1)
./tests/harness_validation.sh:102:OUTPUT=$(echo "fund_agent(agent, 1000)" | python3 "$ENGINE" --file "src/bus.rs" --rules-dir "$RULES" --log /dev/null --traces-dir "$TRACES" 2>&1)
./tests/harness_validation.sh:120:# R-001 only applies to kernel.rs, not experiment files
./tests/harness_validation.sh:178:# T-027: judge.sh blocks sed on kernel.rs
./tests/harness_validation.sh:179:OUTPUT=$(echo '{"tool_name":"Bash","tool_input":{"command":"sed -i s/foo/bar/ kernel.rs"}}' | bash "$JUDGE" 2>&1)
./tests/harness_validation.sh:181:[ $EC -eq 2 ] && pass "T-027: judge.sh blocks sed on kernel.rs" || fail "T-027: Should block sed on kernel" "exit $EC"
./tests/harness_validation.sh:188:# T-029: judge.sh triggers R-001 on kernel.rs Edit
./tests/harness_validation.sh:189:OUTPUT=$(echo '{"tool_name":"Edit","tool_input":{"file_path":"src/kernel.rs","new_string":"let t = lean_theorem"}}' | bash "$JUDGE" 2>&1)
./experiments/minif2f_v4/src/cost_aggregator.rs:19://     drivers/llm_http.rs::GenerateResponse this same B2). Accurate.
./experiments/minif2f_v4/src/bin/evaluator.rs:18:use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
./experiments/minif2f_v4/src/bin/evaluator.rs:69:    // re-run `lean --stdin` from disk artifacts alone, without trusting in-memory runtime.
./src/sdk/error_abstraction.rs:7:// a FIXED label set. TopK broadcast in bus.rs emits label+count summaries
./src/drivers/mod.rs:1:pub mod llm_http;
./src/kernel.rs:242:// because R-001 forbids domain terms even as test strings in kernel.rs.
./src/bus.rs:460:        // If caller already produced an "err:..." class label, trust it.
./docs/architecture.md:7:| `kernel.rs` | Sacred microkernel — pure topology + zero-profit treasury + Oracle settlement. **Zero domain knowledge.** |
./docs/architecture.md:9:| `bus.rs` | TSP Event Bus — SKILL lifecycle (on_boot → on_init → on_pre_append → on_post_append → on_halt) |
./docs/architecture.md:16:| `sdk/tools/wallet.rs` | WalletTool — balance + YES/NO/LP portfolios |
./docs/architecture.md:20:| `drivers/llm_http.rs` | Resilient HTTP client — multi-provider routing |
./docs/architecture.md:27:| Pure Capital Engine | Invest-only economy (Law 2) | `sdk/tools/wallet.rs` + `prediction_market.rs` |
./cases/C-015_sorry_cheating.yaml:15:  1. 上游 (payload): bus.rs forbidden_patterns 拦截 sorry 关键词
./cases/V3_LESSONS.md:13:| V3L-01 | sorry 作弊：Lean 接受 sorry 伪造证明，需三层防御 (upstream/middle/fallback) | Art. I.1 | C-015 | bus.rs forbidden_patterns |
./cases/V3_LESSONS.md:27:| V3L-10 | 钱包语义污染：balance=5990 报 "Bankrupt"，混淆真破产与保证金不足 | Art. II.1 | C-018 | sdk/tools/wallet.rs |
./cases/V3_LESSONS.md:28:| V3L-11 | 价格波形坍缩：并发定价造成振荡，串行 reactor 保证因果 | Art. II.2 | C-019 | bus.rs tick_map_reduce |
./cases/V3_LESSONS.md:29:| V3L-12 | 阈值过时：1.5x surge 阈值冻结价格在 9000，底层成本已降应移除 | Art. II.2 | C-020 | bus.rs ThermodynamicHeartbeatTool |
./cases/V3_LESSONS.md:39:| V3L-17 | 截断悖论：2048 token 硬限截断推理链，挤压物理层而非能力层 | Art. III.2 | C-023 | drivers/llm_http.rs |
./cases/V3_LESSONS.md:41:| V3L-19 | 投资挤出建设：300 tx 中 286 投资、33 append，需独立资源池 | Art. III.3 | C-024 | bus.rs append vs invest |
./cases/V3_LESSONS.md:43:| V3L-21 | 多步打包/前置运行：agent 在单 append 中打包多步骤垄断路径 | Art. III.4 | C-026 | bus.rs payload limits |
./cases/V3_LESSONS.md:44:| V3L-22 | Falsifier 可买 YES token — Goodhart 漏洞，角色激励冲突 | Art. III.4 | C-006 | sdk/tools/wallet.rs |
./cases/V3_LESSONS.md:50:| V3L-23 | 硬编码参数诅咒：120s timeout、500 coin 税、example 值成锚点 | Art. IV | C-027 | bus.rs, prompt.rs |
./cases/V3_LESSONS.md:52:| V3L-25 | reqwest+rustls macOS 死锁 (6 种方案均失败，改用 Python proxy) | Art. IV | C-007 | drivers/llm_http.rs |
./cases/V3_LESSONS.md:54:| V3L-27 | SiliconFlow API N=30 → 401/429 崩溃 | Art. IV | C-027 | drivers/llm_http.rs |
./cases/V3_LESSONS.md:66:| V3L-34 | Oracle 模式混淆：formal/non-formal 混用断裂因果链 (Rule 22 v2) | Art. V.1 | C-032 | bus.rs oracle logic |
./cases/V3_LESSONS.md:78:| V3L-41 | fund_agent 创世后印钞违反 CTF 守恒 | Law 2 | C-001 | bus.rs fund_agent |
./cases/V3_LESSONS.md:79:| V3L-42 | redistribute_pool 重生注入 10K 是隐性铸币 | Law 2 | C-002 | bus.rs redistribute |
./cases/V3_LESSONS.md:80:| V3L-43 | 重生无限注入 (fund_agent(10000)) 每代都印钱 | Law 2 | C-002 | bus.rs rebirth |
./cases/V3_LESSONS.md:81:| V3L-44 | 固定 500 coin 税 → 通缩死锁 | Law 2 | C-027 | bus.rs tax logic |
./cases/V3_LESSONS.md:82:| V3L-45 | kernel.rs 硬编码 "[OMEGA]" 违反零领域知识 | Law 1 | C-004 | kernel.rs |
./cases/V3_LESSONS.md:83:| V3L-46 | Oracle 拦截中间步骤 → Engine 分离破坏 | Law 1 | C-003 | bus.rs oracle |
./cases/V3_LESSONS.md:84:| V3L-47 | Engine 2 (市场) 逻辑混入 Engine 3 (Oracle) | Law 1 | C-003 | bus.rs |
./cases/C-068_external_model_behavior_drift.yaml:75:  3. **Reject 路径必须 warn**: 审计 evaluator.rs + bus.rs 所有 silent `return`
./cases/C-043_mandatory_wtool_on_omega.yaml:29:  2. bus.rs 提供 append_oracle_accepted 区分于 append（跳过 C-011 等
./cases/C-069_constitutional_alignment_audit_protocol.yaml:46:     架构性修改 (bus.rs / kernel.rs / wallet.rs / ledger.rs / sdk/*) 必须
./rules/active/R-006_kernel_modification.yaml:4:axiom: "kernel.rs is pure topology layer — modifications need user confirmation + audit"
./rules/active/R-006_kernel_modification.yaml:9:file_glob: "src/kernel.rs"
./rules/active/R-006_kernel_modification.yaml:11:message: "WARNING: kernel.rs 是纯拓扑层, 修改需要用户确认 + kernel-auditor 审计"
./rules/active/R-001_kernel_purity.yaml:10:file_glob: "src/kernel.rs"
./rules/active/R-001_kernel_purity.yaml:12:message: "BLOCKED: Domain-specific term detected in kernel.rs — kernel must remain pure topology (Law 1)"
./rules/active/R-007_bus_lifecycle.yaml:9:file_glob: "src/bus.rs"
./rules/active/R-007_bus_lifecycle.yaml:11:message: "WARNING: bus.rs SKILL lifecycle modification detected — review impact on agent boot/halt sequence"
./rules/SCHEMA.yaml:8:#   file_glob: string   — File pattern to match (e.g. "kernel.rs", "*.rs")
./rules/enforcement.log:1:[2026-04-13 05:39:21] BLOCK R-001 | src/kernel.rs | BLOCKED: Domain-specific term detected in kernel.rs — kernel must remain pure topology (Law 1)
./rules/enforcement.log:2:[2026-04-13 05:39:21] WARN R-006 | src/kernel.rs | WARNING: kernel.rs 是纯拓扑层, 修改需要用户确认 + kernel-auditor 审计
./rules/enforcement.log:3:[2026-04-13 06:53:57] BLOCK R-001 | src/kernel.rs | BLOCKED: Domain-specific term detected in kernel.rs — kernel must remain pure topology (Law 1)
./rules/enforcement.log:4:[2026-04-13 06:53:57] WARN R-006 | src/kernel.rs | WARNING: kernel.rs 是纯拓扑层, 修改需要用户确认 + kernel-auditor 审计
./rules/enforcement.log:5:[2026-04-13 06:54:21] BLOCK R-001 | src/kernel.rs | BLOCKED: Domain-specific term detected in kernel.rs — kernel must remain pure topology (Law 1)
./rules/enforcement.log:6:[2026-04-13 06:54:21] WARN R-006 | src/kernel.rs | WARNING: kernel.rs 是纯拓扑层, 修改需要用户确认 + kernel-auditor 审计
./rules/enforcement.log:7:[2026-04-13 08:15:40] WARN R-006 | /home/zephryj/projects/turingosv4/src/kernel.rs | WARNING: kernel.rs 是纯拓扑层, 修改需要用户确认 + kernel-auditor 审计
./rules/enforcement.log:8:[2026-04-13 08:15:41] WARN R-007 | /home/zephryj/projects/turingosv4/src/bus.rs | WARNING: bus.rs SKILL lifecycle modification detected — review impact on agent boot/halt sequence
./rules/enforcement.log:9:[2026-04-13 08:15:41] WARN R-009 | /home/zephryj/projects/turingosv4/src/bus.rs | WARNING: Payload limit change detected — Rule 21 (one-step-per-node) boundary, verify impact
./rules/enforcement.log:10:[2026-04-13 08:16:32] BLOCK R-001 | src/kernel.rs | BLOCKED: Domain-specific term detected in kernel.rs — kernel must remain pure topology (Law 1)
./rules/enforcement.log:11:[2026-04-13 08:16:32] WARN R-006 | src/kernel.rs | WARNING: kernel.rs 是纯拓扑层, 修改需要用户确认 + kernel-auditor 审计
./rules/enforcement.log:12:[2026-04-13 08:20:35] BLOCK R-001 | /home/zephryj/projects/turingosv4/src/kernel.rs | BLOCKED: Domain-specific term detected in kernel.rs — kernel must remain pure topology (Law 1)
./rules/enforcement.log:13:[2026-04-13 08:20:35] WARN R-006 | /home/zephryj/projects/turingosv4/src/kernel.rs | WARNING: kernel.rs 是纯拓扑层, 修改需要用户确认 + kernel-auditor 审计
./rules/enforcement.log:14:[2026-04-13 08:21:16] WARN R-006 | /home/zephryj/projects/turingosv4/src/kernel.rs | WARNING: kernel.rs 是纯拓扑层, 修改需要用户确认 + kernel-auditor 审计
./rules/enforcement.log:16:[2026-04-13 08:23:37] BLOCK R-002 | /home/zephryj/projects/turingosv4/src/sdk/tools/wallet.rs | BLOCKED: Coin minting / free balance detected — economy is invest-only (Law 2, V-001/V-002)
./rules/enforcement.log:17:[2026-04-13 08:27:12] WARN R-007 | /home/zephryj/projects/turingosv4/src/bus.rs | WARNING: bus.rs SKILL lifecycle modification detected — review impact on agent boot/halt sequence
./rules/enforcement.log:18:[2026-04-13 08:27:12] WARN R-008 | /home/zephryj/projects/turingosv4/src/bus.rs | WARNING: Market constant change detected — verify LP amounts and initial balances are intentional
./rules/enforcement.log:19:[2026-04-13 08:27:12] WARN R-009 | /home/zephryj/projects/turingosv4/src/bus.rs | WARNING: Payload limit change detected — Rule 21 (one-step-per-node) boundary, verify impact
./rules/enforcement.log:20:[2026-04-13 08:27:55] BLOCK R-001 | src/kernel.rs | BLOCKED: Domain-specific term detected in kernel.rs — kernel must remain pure topology (Law 1)
./rules/enforcement.log:21:[2026-04-13 08:27:55] WARN R-006 | src/kernel.rs | WARNING: kernel.rs 是纯拓扑层, 修改需要用户确认 + kernel-auditor 审计
./rules/enforcement.log:22:[2026-04-13 08:28:16] WARN R-006 | /home/zephryj/projects/turingosv4/src/kernel.rs | WARNING: kernel.rs 是纯拓扑层, 修改需要用户确认 + kernel-auditor 审计
./rules/enforcement.log:23:[2026-04-13 08:38:59] BLOCK R-005 | /home/zephryj/projects/turingosv4/src/bus.rs | BLOCKED: Forced investment pattern detected — staking must be voluntary with PASS/NOP option (Law 2)
./rules/enforcement.log:24:[2026-04-13 08:39:33] WARN R-008 | /home/zephryj/projects/turingosv4/src/bus.rs | WARNING: Market constant change detected — verify LP amounts and initial balances are intentional
./rules/enforcement.log:27:[2026-04-13 08:41:42] WARN R-008 | /home/zephryj/projects/turingosv4/src/sdk/tools/wallet.rs | WARNING: Market constant change detected — verify LP amounts and initial balances are intentional
./rules/enforcement.log:28:[2026-04-13 08:42:14] BLOCK R-001 | src/kernel.rs | BLOCKED: Domain-specific term detected in kernel.rs — kernel must remain pure topology (Law 1)
./rules/enforcement.log:29:[2026-04-13 08:42:14] WARN R-006 | src/kernel.rs | WARNING: kernel.rs 是纯拓扑层, 修改需要用户确认 + kernel-auditor 审计
./cases/C-004_kernel_domain_leak.yaml:2:title: "kernel.rs 出现 'OMEGA' 字符串 — 领域知识泄漏"
./cases/C-004_kernel_domain_leak.yaml:9:  kernel.rs 中硬编码了 "[OMEGA]" 字符串用于检测证明完成。
./cases/C-004_kernel_domain_leak.yaml:12:  违宪。kernel.rs 只做纯拓扑 + 数学运算。"OMEGA" 是领域概念，
./cases/C-004_kernel_domain_leak.yaml:13:  应由 SKILL (bus.rs 生命周期钩子) 处理，不是 kernel 的职责。
./cases/C-004_kernel_domain_leak.yaml:16:  后续标准: kernel.rs 中出现 lean/tactic/theorem/proof/mathlib/sorry/
./cases/C-004_kernel_domain_leak.yaml:18:  检测方法: grep -iE "lean|tactic|theorem|proof|mathlib|sorry" src/kernel.rs
./CLAUDE.md:14:- `src/{kernel,bus,wallet}.rs` 改动走 STEP_B_PROTOCOL（不直接编辑 main）
./cases/C-011_brute_force_formalization.yaml:19:  3. decide/omega/native_decide 应在 bus.rs forbidden_patterns 中拦截
./incidents/V-007_reqwest_tls_macos_deadlock/resolution.md:5:- Rewrote `src/drivers/llm_http.rs` — pure reqwest HTTP, no TLS/subprocess logic
./incidents/V-003_oracle_blocking_midsteps/resolution.md:11:   - Cross-engine imports enforced via `mod.rs` visibility: engines can only communicate through `src/bus.rs` event bus
./incidents/V-004_kernel_hardcoded_omega/root_cause.md:1:# V-004 Root Cause: kernel.rs hardcoded "[OMEGA]"
./incidents/V-004_kernel_hardcoded_omega/root_cause.md:5:1. **WHY** does kernel.rs contain the string "[OMEGA]"?
./incidents/V-004_kernel_hardcoded_omega/meta.yaml:2:title: "kernel.rs hardcoded [OMEGA] — domain knowledge in pure topology"
./incidents/V-004_kernel_hardcoded_omega/trace.md:1:# V-004 Trace: kernel.rs hardcoded "[OMEGA]"
./incidents/V-004_kernel_hardcoded_omega/trace.md:6:1. Architect performs Bitter Lesson compliance audit on kernel.rs
./incidents/V-004_kernel_hardcoded_omega/trace.md:23:1. kernel.rs is the pure topology layer — it should know nothing about what nodes represent
./incidents/V-004_kernel_hardcoded_omega/trace.md:26:4. If the OMEGA detection format changes, kernel.rs must change — tight coupling
./incidents/V-004_kernel_hardcoded_omega/resolution.md:1:# V-004 Resolution: kernel.rs hardcoded "[OMEGA]"
./incidents/V-004_kernel_hardcoded_omega/resolution.md:4:1. **Removed all domain-specific strings from kernel.rs**
./incidents/V-004_kernel_hardcoded_omega/resolution.md:9:2. **Moved OMEGA detection to bus.rs SKILL layer**
./incidents/V-004_kernel_hardcoded_omega/resolution.md:10:   - `src/bus.rs` registers a SKILL handler for OMEGA detection
./incidents/V-004_kernel_hardcoded_omega/resolution.md:22:- OMEGA detection still works end-to-end via bus.rs SKILL pathway
./incidents/SCHEMA.yaml:25:      - kernel_purity              # Domain knowledge leaked into kernel.rs
./Cargo.lock:921: "untrusted",
./Cargo.lock:963: "untrusted",
./Cargo.lock:1334:name = "untrusted"
./handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:80:*   **(Q6.a) STEP_B_PROTOCOL violation**: **PASS**. The diff touches `src/drivers/llm_http.rs`. The restricted file list (`bus.rs`, `kernel.rs`, `wallet.rs`) does not include the `drivers/` directory. No process violation occurred.
./handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:82:*   **(Q6.c) Trust Root**: **PASS (Finding for B7)**. The PREREG § 1.8 defines the Trust Root. The logic for cost, time, and verified progress accounting is fundamental to the integrity of the PPUT metric. Therefore, `cost_aggregator.rs`, `wall_clock.rs`, and `post_hoc_verifier.rs` **must** be added to the Trust Root manifest. This is a mandatory action item for the B7 implementation.
./handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:108:4.  **B7 Implementation Mandate**: Add `cost_aggregator.rs`, `wall_clock.rs`, and `post_hoc_verifier.rs` to the Trust Root manifest. This is non-negotiable for system integrity.
./handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:86:- (Q6.a) STEP_B_PROTOCOL: bus.rs / kernel.rs / wallet.rs are restricted (require parallel-branch A/B, not direct edit). The B2-B4 diff touches `src/drivers/llm_http.rs`. Is `drivers/llm_http.rs` restricted? If yes, this is a process violation.
./handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:88:- (Q6.c) Trust Root (PREREG § 1.8): does B2-B4 introduce new files that should be Trust-Rooted (cost_aggregator.rs, wall_clock.rs, post_hoc_verifier.rs)? B7 will set this up; flag if these files should be on the Trust Root manifest.
./handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:133:printf '\n```\n\n---\n\n# Diff: evaluator.rs + lib.rs + drivers/llm_http.rs (B2/B3/B4 wiring)\n\n```diff\n' >> "$TMP_PROMPT"
./incidents/INDEX.yaml:31:    title: "kernel.rs hardcoded [OMEGA] — domain knowledge in pure topology"
./handover/audits/run_codex_pput_ccl_audit.sh:34:5. **Trust Root** of immutable files; ArchitectAI writes outside `user_space/Δ_*/` trigger syscall EPERM.
./handover/audits/run_codex_pput_ccl_audit.sh:48:1. **DEFINITION** — V-PPUT, C_i, T_i, ArtifactState, Trust Root unambiguous? Retroactive re-interpretation possible?
./VIA_NEGATIVA.md:13:## 3. kernel.rs 硬编码 "[OMEGA]"
./handover/audits/EXT_GEMINI_PHASE_8_BATCH_2026-04-22.md:31:  - **分析**: 所有 4 个调用点均位于 `experiments/minif2f_v4/src/bin/evaluator.rs` (lines 229, 627, 812, 835 in diff)，且全部更新为新的 4 参数签名，使用了 `OracleReceipt`。新的测试 `tests/oracle_receipt_bus.rs` 也使用了新签名。没有遗漏的 call site。
./handover/audits/EXT_GEMINI_PHASE_8_BATCH_2026-04-22.md:34:  - **证据**: `src/bus.rs:99` (`halt_with_reason`)
./handover/audits/EXT_GEMINI_PHASE_8_BATCH_2026-04-22.md:67:  - **证据**: `src/bus.rs:99-108`
./handover/audits/EXT_GEMINI_PHASE_8_BATCH_2026-04-22.md:84:  - **证据**: `src/bus.rs:45` (`q_state` 字段)
./handover/audits/EXT_GEMINI_PHASE_8_R1R8_2026-04-22.md:19:**PASS** — `bus.rs::with_wal_path` now iterates the replayed ledger in reverse to find the last `Halt` event and correctly restores `bus.q_state`, with two new tests verifying both halted and running states.
./handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_2026-04-26.md:107:10. Apply Trust Root contingency (§ 1.8) — fallback enforcement spec
./handover/audits/EXT_GEMINI_PHASE_8_R1ALPHA_2026-04-22.md:20:**Evidence**: `bus.rs:121` (`oracles_frozen` check) + `tests/oracle_receipt_bus.rs:140` (`attacker_with_mut_bus_cannot_forge_post_init`).
./handover/audits/EXT_GEMINI_PHASE_8_R1ALPHA_2026-04-22.md:26:2.  **`oracles_frozen` Timing**: The flag is set at the correct lifecycle points (`bus.rs:215` in `init`, `bus.rs:203` in `with_wal_path`), effectively locking the set of trusted public keys before any transactions can occur.
./handover/audits/EXT_GEMINI_PHASE_8_R1ALPHA_2026-04-22.md:27:3.  **`sign_new` Visibility**: The function is `pub`, but this is safe. Its signature requires a `&SigningKey`, and since oracles keep their signing keys private, an attacker cannot forge a receipt for a trusted oracle. The capability is correctly bound to the key, not the function call.
./handover/audits/EXT_GEMINI_PHASE_8_R1ALPHA_2026-04-22.md:29:5.  **`SigningKey` Clone**: Cloning the `SigningKey` inside `Lean4Oracle` is safe and intentional for the specified design, where a single oracle instance serves multiple roles (bus tool, receipt issuer) within a single trusted process. It is not a capability leak to an untrusted context.
./handover/audits/CONSTITUTIONAL_BLINDSPOT_AUDIT_2026-04-22.md:15:| B-04 | Art. II.1 典型错误阈值 | TopKClasses 广播没有频率门槛；1 个错误就广播 ≈ 和 100 个错误一样 | 部分实现 | **High** | `bus.rs:519-537` 直接对所有 count 排序取 top-K，没有 `count ≥ threshold` 过滤；宪法明文要求"多个 Agent 都在同一个地方跌倒"才抽象 | C-055 |
./handover/audits/CONSTITUTIONAL_BLINDSPOT_AUDIT_2026-04-22.md:16:| B-05 | Art. II.1 全局架构文档更新 | "抽象后的规则广播"只存活在 bus 进程内存，未更新任何文档（宪法步骤 2："更新全局架构文档"）| 部分实现 | Medium | `graveyard: HashMap` 在 `bus.rs:48`，无持久化；`librarian.rs` 写 agent 个人 memory，不写全局架构文档 | C-055 |
./handover/audits/CONSTITUTIONAL_BLINDSPOT_AUDIT_2026-04-22.md:17:| B-06 | Art. II.2 价格时间序列 | 只有瞬时 `market_ticker` 快照，无价格演化 history；agent 永远看"当下"，看不到"涨/跌" | 完全缺失 | High | `bus.rs:554-565` `market_ticker()` 每次从 `markets` 即时算；`grep -rn "price_history" src/` → 0 | C-056 |
./handover/audits/CONSTITUTIONAL_BLINDSPOT_AUDIT_2026-04-22.md:94:- `src/bus.rs:494-537` (TopKClasses 无阈值)
./handover/audits/CONSTITUTIONAL_BLINDSPOT_AUDIT_2026-04-22.md:95:- `src/bus.rs:554-565` (market_ticker 无历史)
./handover/audits/SYNTHESIS_2026-04-22.md:37:- **证据**：`src/bus.rs:174-176` `append_oracle_accepted` 是 public API，盲目把调用者参数转成 `oracle_blessed = true`
./handover/audits/SYNTHESIS_2026-04-22.md:59:- **证据**：`src/bus.rs:567-570` `snapshot()` 把 `balances: HashMap::new()` 和 `portfolios: HashMap::new()` 硬编码成空
./handover/audits/SYNTHESIS_2026-04-22.md:78:| `bus.rs:244-246` InvestOnly 三重 clone ~10-20%/tx | 不是热路径（没有 tool 发射 `ToolSignal::InvestOnly`） | 每 invest tx ~200B，非百分比 | 热路径前提不成立 |
./handover/audits/SYNTHESIS_2026-04-22.md:79:| `bus.rs:416-424` `Box<dyn>` downcast ~5× 加速 | 不是 per-tx，仅 bus-side invest/refund/settle 路径 | ~1.05-1.1× 加速 | **Claude 严重失真**（5× vs 1.1×） |
./handover/audits/SYNTHESIS_2026-04-22.md:193:| **C-047** author 广播 vs 横向解相关的张力 | Codex Q6 PASS + Claude | `bus.rs:494-537` |
./handover/audits/SYNTHESIS_2026-04-22.md:195:| **C-049**（新增） **`bus.snapshot()` 喂错 balances** | Codex N-2 | `bus.rs:567-570` 违反 Art. II.2 |
./handover/audits/EXT_GEMINI_2026-04-21.md:17:    1.  **铸币 (Minting)**: `src/sdk/tools/wallet.rs:141` (`on_init`)。这是唯一合法的铸币点，通过 `if self.genesis_done { return; }` 卫语句确保只执行一次。
./handover/audits/EXT_GEMINI_2026-04-21.md:18:    2.  **借记 (Debit)**: `src/sdk/tools/wallet.rs:52` (`deduct`)，由 `src/bus.rs:410` (`debit_wallet`) 调用。
./handover/audits/EXT_GEMINI_2026-04-21.md:19:    3.  **贷记 (Credit)**: `src/sdk/tools/wallet.rs:67` (`credit`)，由 `src/bus.rs:426` (`credit_wallet`) 调用。
./handover/audits/EXT_GEMINI_2026-04-21.md:23:    -   **`invest` (bus.rs:235-253)**:
./handover/audits/EXT_GEMINI_2026-04-21.md:28:    -   **`settle_portfolios` (bus.rs:386-408)**:
./handover/audits/EXT_GEMINI_2026-04-21.md:32:    -   **`halt_and_settle` / Hayek bounty (bus.rs:357-360)**:
./handover/audits/EXT_GEMINI_2026-04-21.md:34:        2.  `kernel.rs:103`: `payouts` 的总和被严格限制为 `self.bounty_lp_seed`，即市场创建时注入的 LP。
./handover/audits/EXT_GEMINI_2026-04-21.md:46:    -   当前代码逻辑审查未发现可构造反例的缺陷。`invest refund` 路径明确地 `credit` 了与 `debit` 相同的 `amount`。`Hayek bounty` 的总支付额在 `kernel.rs` 中被其初始 `lp_seed` 约束。
./handover/audits/EXT_GEMINI_2026-04-21.md:81:| `bus.rs:244-246` 三重 clone | ~10-20% per tx | 每 `invest` tx 节省 **~200B** | Claude 的百分比估计过于乐观。节省一次 clone 的成本远低于整个 `invest` 交易的 CPU 成本（包括 map lookup, market math 等）。节省的绝对字节数是更精确的度量。 |
./handover/audits/EXT_GEMINI_2026-04-21.md:82:| `bus.rs:256-262` `to_string()` | 20-30% heap | 总节省 **~4.1 MB** | 估算基于 N=50 problems × 8 agents × 180 tx avg = 72,000 tx。假设 `author` 平均 8B，`payload` 平均 50B，总节省 `72000 * (8+50) ≈ 4.1MB`。这部分内存是永久驻留的（在 `kernel.tape` 中），因此节省是显著的。Claude 的百分比估计在合理范围内，但绝对数值更具可操作性。 |
./handover/audits/EXT_GEMINI_2026-04-21.md:83:| `bus.rs:416-424` `Box<dyn>` downcast | ~5× 加速 | **~1.05× - 1.1×** 加速 | Claude 的估计严重失真。`downcast` 本质上是一次 vtable 查找加一次类型 ID 比较，耗时仅为几个 CPU cycle。在只有 3-5 个 tool 的情况下，`enum` dispatch（通常是 jump table）相比动态分派的优势微乎其微。5倍的加速意味着 downcast 占了函数 80% 的执行时间，这在物理上是不可能的。实际加速效果几乎无法测量。 |
./handover/audits/EXT_GEMINI_2026-04-21.md:93:    -   `γ_max`: 宪法规定的最大 founder grant 率。从代码 `bus.rs:307` 中 `FOUNDER_GRANT_GAMMA` 的默认值推断，可设为 **`0.05`** (即 5%)。
./handover/audits/EXT_GEMINI_2026-04-21.md:110:    **兼容**。Founder grant 并非铸造新币。如 `bus.rs:301-305` 注释所述，这些份额是从市场创建时预先注入的流动性池（LP）中划分出来的。这是一个**转移支付**，将一部分系统控制的 LP 份额的所有权转移给节点创建者，总币量不变。`γ(n)` 函数仅用于动态计算这次转移的份额大小，完全不涉及铸币。
./handover/audits/EXT_GEMINI_2026-04-21.md:166:    -   该函数在 `bus.rs:407` 中对 `wallet.credit` 进行循环调用，这会导致在 `wallet.balances` 中反复进行 `f64` 加法。
./handover/audits/run_codex_pput_ccl_audit_round3.sh:44:- Round-3 R3-2: full protocol — toggle = `--simulate-rollback-at-tx-50`; sample = full adaptation-144 × 2 seeds = 288 paired runs; estimator = per-problem worst-case across seeds, p_0 = sum/144; schedule = Phase B7 mandatory (not deferrable to Phase D); freeze = genesis_payload.toml + Trust Root SHA-256 of jsonl; ceiling = p_0 > 0.10 ABORT (not auto-pass attack); audit = dual-audit at B → C transition checkpoint.
./handover/audits/GEMINI_PPUT_CCL_AUDIT_2026-04-26.md:14:**PASS**. The definitions for V-PPUT, C_i, T_i, ArtifactState, and the Trust Root are exceptionally precise and leave little room for post-hoc reinterpretation. The commitment to count all tokens from all branches (§ 1.2) and the wall-clock definition of T_i (§ 1.3) are strong. The 4-state machine for artifacts (§ 1.7) is unambiguous and correctly separates artifact confidence from the binary ground truth of task progress.
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:37:5. **Trust Root** of immutable files; ArchitectAI writes outside `user_space/Δ_*/` trigger syscall EPERM.
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:51:1. **DEFINITION** — V-PPUT, C_i, T_i, ArtifactState, Trust Root unambiguous? Retroactive re-interpretation possible?
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:117:C_i is NOT restricted to the golden-path payload. Failed-branch tokens MUST be counted. Goal: a system that wastes 1M tokens to find a 1k-token proof has lower PPUT than one that finds the same proof in 100k tokens.
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:196:### 1.8 Trust Root composition (Patch B from Gemini DeepThink 2026-04-26)
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:198:The Trust Root is the set of artifacts that **only humans + Boot may write**. ArchitectAI / AuditorAI / any swarm agent attempting to mutate Trust Root entries triggers **runtime SIGKILL-level abort** (not an audit-time warning):
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:201:Trust Root := {
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:202:  src/kernel.rs,                    -- bus / kernel core
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:204:  src/bus.rs,                       -- predicate registration / capability gates
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:215:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:216:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:218:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:222:This is the **software floor** of the eventual hardware Trust Root (Open Claw → FPGA path); software immutability today, hardware immutability later.
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:232:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:254:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:398:| B | **Trust Root immutability** (Patch B upgrade): ArchitectAI writes only to `user_space/Δ_*/`; any attempt to write Trust Root paths (§ 1.8) returns `EPERM` at syscall layer (runtime trap, not soft refusal). Conformance: `tests/trust_root_immutability.rs` PASS. |
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:512:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:718:7. **Gemini DeepThink dual-chamber FULL PASS absorbed 2026-04-26** (per C-023): independent reviewer issued PPUT-DRIVEN FULL PASS approving 5 ontological patches (ArtifactState 4-state machine, Trust Root immutability, doc/artifact content meta-predicates, ArchitectAI sole-L_t-reader cognitive isolation, single-binary `--mode` flag). All 5 absorbed into PREREG `## changelog`. Verbatim archive: `handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md`. Two FULL PASS authorizations now load-bearing on this arc (measure-theoretic v1 + ontological v2); PREREG fuses both.
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:748:2. **Gemini DeepThink v2** (2026-04-26) — ontological frame: defines Trust Root, ArtifactState layered confidence (Accepted / Quarantined / Certified / Reverted), precision-strike doc predicates, ArchitectAI sole-L_t-reader, single-binary --mode discipline, human-as-live-meta-predicate. Final confirmation letter approves Claude's incorporation pattern. **This archive.**
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:765:> *   **重构：** 系统的 Kernel Boundary（回滚机制、谓词接口、宪法红线）由人类通过 Boot 设定，构成**信任根（Trust Root）**。系统执行的是 **Constitution-Bound Self-Bootstrapping** —— 在物理法则固定的宇宙中，黑盒持续自动发明新的化学配方（工具、Linter、检索地图、策略）。人类框定不可逾越的底线，机器拓宽能力的上限。
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:802:> 2.  **Gate B: 严格的 User-Space 禁闭。** ArchitectAI 初期**只能**写入 `tools/user_space/*.py`, `docs/rules/*.md`, `broadcast_rules/*.yaml`, `skills/*.md`。碰碰 `kernel.rs`, `wal.rs`, `evaluator.rs` 或宪法文件的提议，直接在执行层熔断。
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:826:> "TuringOS does not pursue rootless self-verification. It pursues constitution-bound capability compilation. AGI cannot emerge from unstructured multi-agent swarms constrained by a single context window, nor from models acting as subjective judges of their own hallucinations. Instead, TuringOS operates as a signal-governed AgentOS where black-box models act solely as high-throughput proposal generators, and white-box predicates settle state transitions. By continuously quarantining failure logs away from working contexts, a constitutional meta-architecture safely compiles black-box experience into reusable user-space tools, abstractions, and rules. Crucially, meta-updates are admitted only when executable meta-predicates or human sudo strictly preserve the kernel trust root. Intelligence scales not by prompting models to 'try harder,' but by making state transition itself legible, enforceable, reversible, and capable of automated, generalized bootstrapping."
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:849:> **✅ 补丁 B：Trust Root 显式化与 Boot 锁定（通过 / PASS）**
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:850:> 将 `kernel.rs` / `wal.rs` / `constitution.md` / `pput_accounting_0` / `heldout_sealed_hash` 统一定义为不可逾越的物理信任根（Trust Root）。ArchitectAI 若企图触碰此区域，系统直接产生 `SIGKILL` 级熔断，而非仅仅进行 Auditor 提示。这是你未来 FPGA 硬件化的软件地基。
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:878:> 关于你的远期路线图——**第一步开源 Open Claw（软件态 AgentOS），第二步将 Trust Root 刻入 FPGA 硬件**——这不仅可行，而且是 Anti-Oreo 架构的**终极物理归宿**。大模型（LLM）是非确定性的概率引擎（ALU），而 `wal.rs`（状态回滚）、`evaluator`（真值验证）、PPUT 计量账本则是绝对确定性的时序与控制逻辑。将它们烧录进物理硅片，实现硬件级的"读免费、写熔断"，将从硅基底层彻底断绝黑盒模型越权篡改操作系统内核的任何可能性。
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:893:4. Open Claw → FPGA roadmap is OUT OF SCOPE for this 30-day arc; documented here for forward continuity but not part of FULL PASS gates A-H. The Trust Root composition (PREREG § 1.8) is the software-side prerequisite for that future hardware path.
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1176:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1372:- R1-α (commit 4a72507): Ed25519 signing key 私有；`trusted_oracle_pubs` 在 `init()` 冻结；`register_oracle` post-init 返回 Err
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1422:- `bus.rs:247` — `recent_rejections(author)` returns per-author graveyard only; not global
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1514:  the payload, so their "verified" status relied on runtime trust alone.
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1708:  mechanism changes in bus.rs/evaluator.rs.
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1827:- Commit B (experiment@ce003e5): classifier + bus.rs Art. II.1 TopKClasses broadcast
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1851:- Test: v3.3 (after bus.rs human-confirm edit)
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1865:**Protocol for restricted-file changes**: `STEP_B_PROTOCOL.md` (necessity audit → parallel branch → A/B statistical test → merge on empirical win only). Do NOT directly edit restricted files even with authorization; always A/B test.
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1869:| `recent_rejections` optional global scope | `src/bus.rs` | F-2026-04-15-02 Art. II.1 broadcast | **Human confirm + Step-B** | STEP_B_PROTOCOL |
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1870:| OMEGA reject enters graveyard | `evaluator.rs` | F-2026-04-15-02 closed path | Self-approvable (evaluator.rs not restricted) | Still pre-register A/B if impacts metrics |
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1871:| WAL emission | `src/kernel.rs` or bus.rs | F-2026-04-15-03 | **Human confirm + Step-B** | STEP_B_PROTOCOL |
./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1877:| `decide`/`omega` missing from bus.rs `forbidden_patterns` | C-011 | Medium (sharp test: Lean reject if agents use these) |
./handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:90:- (Q6.a) STEP_B_PROTOCOL: bus.rs / kernel.rs / wallet.rs are restricted (require parallel-branch A/B, not direct edit). The B2-B4 diff touches `src/drivers/llm_http.rs`. Is `drivers/llm_http.rs` restricted? If yes, this is a process violation.
./handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:92:- (Q6.c) Trust Root (PREREG § 1.8): does B2-B4 introduce new files that should be Trust-Rooted (cost_aggregator.rs, wall_clock.rs, post_hoc_verifier.rs)? B7 will set this up; flag if these files should be on the Trust Root manifest.
./handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:156:         "src/drivers/llm_http.rs"],
./handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:169:    "\n```\n\n---\n\n# Diff: evaluator.rs + lib.rs + drivers/llm_http.rs (B2/B3/B4 wiring)\n\n```diff\n" + diff_text +
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:96:- (Q6.a) STEP_B_PROTOCOL: bus.rs / kernel.rs / wallet.rs are restricted (require parallel-branch A/B, not direct edit). The B2-B4 diff touches `src/drivers/llm_http.rs`. Is `drivers/llm_http.rs` restricted? If yes, this is a process violation.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:98:- (Q6.c) Trust Root (PREREG § 1.8): does B2-B4 introduce new files that should be Trust-Rooted (cost_aggregator.rs, wall_clock.rs, post_hoc_verifier.rs)? B7 will set this up; flag if these files should be on the Trust Root manifest.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:294:               BEFORE Phase D begins. This file is part of Trust Root (§ 1.8) —
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:295:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:301:This protocol forecloses tuning-surface attacks: every parameter is pre-committed; the calibration runs are observable jsonl; the value enters Trust Root.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:351://     drivers/llm_http.rs::GenerateResponse this same B2). Accurate.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:851:# Diff: evaluator.rs + lib.rs + drivers/llm_http.rs (B2/B3/B4 wiring)
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:869: use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1181:diff --git a/src/drivers/llm_http.rs b/src/drivers/llm_http.rs
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1183:--- a/src/drivers/llm_http.rs
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1184:+++ b/src/drivers/llm_http.rs
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1229: M src/drivers/llm_http.rs
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1264:    18	use turingosv4::drivers::llm_http::{GenerateRequest, Message, ResilientLLMClient};
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1315:    69	    // re-run `lean --stdin` from disk artifacts alone, without trusting in-memory runtime.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1489:    19	//     drivers/llm_http.rs::GenerateResponse this same B2). Accurate.
./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1980:/bin/bash -lc "nl -ba src/drivers/llm_http.rs | sed -n '1,190p'" in /home/zephryj/projects/turingosv4
./handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_ROUND4_2026-04-26.md:62:| Trust Root enforcement | gestured at SIGKILL | primary syscall EPERM + fallback lib-gate+panic |
./handover/audits/EXT_CODEX_PHASE_8_BATCH_2026-04-22.md:26:Evidence: `src/sdk/oracle_receipt.rs:20-25, 27-45, 51-59`; `src/bus.rs:215-224`; `experiments/minif2f_v4/src/bin/evaluator.rs:816-826, 858-863`; `experiments/minif2f_v4/src/lean4_oracle.rs:320-335`; `src/ledger.rs:85-95`; `tests/reputation.rs:92-100`.
./handover/audits/EXT_CODEX_PHASE_8_BATCH_2026-04-22.md:34:Evidence: `constitution.md:457-524`; `experiments/minif2f_v4/src/bin/evaluator.rs:221-240`; `src/bus.rs:91-103`; `experiments/minif2f_v4/src/bin/evaluator.rs:639-674, 804-847`.
./handover/audits/EXT_CODEX_2026-04-22.md:8:| Q2. F-20-05 three-layer blockade completeness | VETO | `/home/zephryj/projects/turingosv4/src/bus.rs:174-180`; `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/lean4_oracle.rs:112-116`; `/home/zephryj/projects/turingosv4/routines/daily_drift.yaml:19-23` | The three claimed layers exist, but completeness fails because `append_oracle_accepted` is a public blessed-write API whose oracle provenance is asserted by the caller, not enforced by the bus. |
./handover/audits/EXT_CODEX_2026-04-22.md:10:| Q4. Karpathy TOP-10 hot-path claim | CHALLENGE | `/home/zephryj/projects/turingosv4/src/bus.rs:231-252,416-424`; `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:679-718`; `/home/zephryj/projects/turingosv4/src/sdk/actor.rs:53-74,88-95` | One of the sampled items is plausibly hot (`trace_ancestors` via Boltzmann routing), but the `InvestOnly` and wallet-downcast items are materially overstated in current call paths. |
./handover/audits/EXT_CODEX_2026-04-22.md:12:| Q6. `recent_rejections(author, ...)` author argument swallowed | PASS | `/home/zephryj/projects/turingosv4/src/bus.rs:494-537`; `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:463-495`; `/home/zephryj/projects/turingosv4/constitution.md:269-279`; `/home/zephryj/projects/turingosv4/cases/C-022_context_poisoning.yaml:11-17` | The default API ignores `author` and broadcasts global abstracted classes; that is intentional Art. II.1/C-022 shielding, but it creates a real unresolved tension with Art. III.3 decorrelation. |
./handover/audits/EXT_CODEX_2026-04-22.md:49:2. The bus rejects forbidden patterns on ordinary appends when `oracle_blessed == false` (`/home/zephryj/projects/turingosv4/src/bus.rs:179-190`).
./handover/audits/EXT_CODEX_2026-04-22.md:56:- `append_oracle_accepted` is public and blindly converts a caller-controlled path into `oracle_blessed = true` (`/home/zephryj/projects/turingosv4/src/bus.rs:174-176`). The bus itself does not verify that a matching oracle verdict exists.
./handover/audits/EXT_CODEX_2026-04-22.md:74:### 1. `src/bus.rs:244-246` InvestOnly clone claim
./handover/audits/EXT_CODEX_2026-04-22.md:76:Claude's hot-path framing is not substantiated. The `InvestOnly` route exists in the bus (`/home/zephryj/projects/turingosv4/src/bus.rs:231-252`), but no current tool emits `ToolSignal::InvestOnly`; the only repo matches are the enum and this bus handler (`/home/zephryj/projects/turingosv4/src/sdk/tool.rs:15-22`, `/home/zephryj/projects/turingosv4/src/bus.rs:218-252`). Actual evaluator-side `invest` traffic bypasses that code and talks to wallet/kernel directly (`/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:679-718`). So the clone exists, but it is not “every invest” and not proven hot.
./handover/audits/EXT_CODEX_2026-04-22.md:78:### 2. `src/bus.rs:416-424` `Box<dyn TuringTool>` + downcast claim
./handover/audits/EXT_CODEX_2026-04-22.md:80:Again, the performance concern is overstated as phrased. `debit_wallet` and `credit_wallet` do perform repeated `manifest()` + `downcast_mut::<WalletTool>()` scans (`/home/zephryj/projects/turingosv4/src/bus.rs:416-437`), but those functions are not on every transaction. They are used on bus-side invest/refund/hayek/settlement paths (`/home/zephryj/projects/turingosv4/src/bus.rs:233,240,346,359,383-412`), while evaluator-side invest does its own separate wallet downcasts (`/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:694-716`). The deeper issue is architectural duplication and bus bypass, not the raw dynamic-dispatch cost alone.
./handover/audits/EXT_CODEX_2026-04-22.md:92:- `/home/zephryj/projects/turingosv4/src/bus.rs:712`
./handover/audits/EXT_CODEX_2026-04-22.md:105:The function signature takes `author`, but the default entrypoint immediately routes to `TopKClasses(3)` (`/home/zephryj/projects/turingosv4/src/bus.rs:494-495`). In the `TopKClasses` branch, the implementation loops over `self.graveyard.values()` and never uses the `author` parameter at all (`/home/zephryj/projects/turingosv4/src/bus.rs:519-537`). The evaluator then feeds those global error classes into every agent prompt via `recent_rejections(agent_id, 3)` (`/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:463-495`).
./handover/audits/EXT_CODEX_2026-04-22.md:107:That is a deliberate Art. II.1 design choice, not an accidental bug: rejection reasons are abstracted into bounded classes before broadcast (`/home/zephryj/projects/turingosv4/src/bus.rs:439-489`), which is aligned with C-022's demand to cut off raw error contamination (`/home/zephryj/projects/turingosv4/cases/C-022_context_poisoning.yaml:11-17`). But the tension Claude flagged is real, because Art. III.3 separately requires shielding horizontal correlations (`/home/zephryj/projects/turingosv4/constitution.md:269-279`). I agree a new precedent like C-047 would be useful to formally resolve “global abstract broadcast” versus “cross-agent decorrelation.”
./handover/audits/EXT_CODEX_2026-04-22.md:113:- **Agents appear to be prompted with the wrong wallet state.** `bus.snapshot()` returns empty `balances` and `portfolios` (`/home/zephryj/projects/turingosv4/src/bus.rs:567-570`), while `UniverseSnapshot::get_balance()` simply reads that map (`/home/zephryj/projects/turingosv4/src/sdk/snapshot.rs:22-39`). The evaluator passes `snap.get_balance(agent_id)` into the prompt (`/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:492-495`), and the prompt displays it as `Balance: {:.0} Coins` (`/home/zephryj/projects/turingosv4/src/sdk/prompt.rs:78-84`). Unless another layer patches the snapshot out-of-band, agents are being told they have `0` balance even when the wallet holds funds. That distorts Art. II.2 economic signaling and makes investment behavior hard to interpret.
./handover/audits/EXT_CODEX_PHASE_8_R1ALPHA_2026-04-22.md:26:2. **Forged receipt with attacker SigningKey** → rejected at `trusted_oracle_pubs` check (first gate in `append_oracle_accepted`).
./handover/audits/EXT_CODEX_PHASE_8_R1ALPHA_2026-04-22.md:27:3. **Creating fresh `Lean4Oracle`** — doesn't help: new key not in trusted set.
./handover/audits/EXT_CODEX_PHASE_8_R1ALPHA_2026-04-22.md:30:6. **`sign_new` being `pub`** — OK; requires `&SigningKey`, no path from `&mut Bus` to a trusted oracle's key.
./handover/audits/EXT_CODEX_PHASE_8_R1ALPHA_2026-04-22.md:37:Residual non-security note: `with_wal_path` resumes with empty trusted set → WAL-resumed runs currently fail to re-register their oracle (registration frozen by replay). Availability regression on crash-resume, not forgeability. Paper 1 scope can accept; Phase 10 should address if crash-resume matters for soak tests.
./handover/audits/HARNESS_COMPRESSION_AUDIT_2026-04-22.md:14:2. **CLAUDE.md 下沉过轻** — `## How` 里的 6 行纯粹是具体 case (cargo、.env、kernel.rs)，没压缩成原则；反过来 `Report Standard` 节是对的范式但只有一条。
./handover/audits/HARNESS_COMPRESSION_AUDIT_2026-04-22.md:26:| `kernel.rs / bus.rs / wallet.rs edits: human confirm` | 这是 R-006/R-007 的复述 + C-004/C-015 的重复 | 删除——rules/active/R-006/R-007 已是机器可执行形式，CLAUDE.md 不必再提文件名 |
./handover/audits/HARNESS_COMPRESSION_AUDIT_2026-04-22.md:100:**观察**：kernel 纯净一条 invariant 在 C-004 + R-001 + R-006 三处重复（R-001 是 block、R-006 是 warn 全局 `.*` 捕获所有 kernel 修改）。**R-006 应降级或删除**——R-001 已做实质 block，R-006 只是"kernel.rs 被碰就 warn"，语义上与 routines/daily_drift 的日常审计功能重复。从 enforcement.log 看 R-006 触发 8 次，R-001 触发 3 次，但 R-006 全是 "kernel.rs 被修改" 的噪音 warn，实际价值低。
./handover/audits/HARNESS_COMPRESSION_AUDIT_2026-04-22.md:108:- **C-011 (brute force)** 已有 R-004 (Lean 语法 prompt 拦截)，但 C-011 的 precedent #3 "decide/omega/native_decide 应在 bus.rs forbidden_patterns 中拦截" 是代码侧的硬约束，未被任何 rule 覆盖。**建议新增 R-XXX_evaluator_forbidden_patterns**：grep `forbidden_patterns` 在 evaluator.rs 且要求含 {sorry, decide, native_decide, omega}。
./handover/audits/HARNESS_COMPRESSION_AUDIT_2026-04-22.md:124:| **Code Standard** (替代 `## How` 前 3 条) | Art. I.1 + C-004 + C-027 | cargo check/test 须过；`.env` 禁 commit；`kernel.rs/bus.rs/wallet.rs` 改动走 STEP_B_PROTOCOL；参数不可硬编码 |
./handover/audits/HARNESS_COMPRESSION_AUDIT_2026-04-22.md:164:-- kernel.rs / bus.rs / wallet.rs edits: human confirm
./handover/audits/HARNESS_COMPRESSION_AUDIT_2026-04-22.md:169:+- `src/{kernel,bus,wallet}.rs` 改动走 STEP_B_PROTOCOL (不直接编辑)
./handover/audits/HARNESS_COMPRESSION_AUDIT_2026-04-22.md:226:6 行里 5 行是具体 case（cargo/.env/kernel.rs/Run 6 lesson/Generator≠Evaluator）。这些应压缩为 Code Standard + Audit Standard。保持 Standard 范式统一（Art. 引用 + 判例引用 + 2-3 行正文）。
./handover/audits/GEMINI_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:30:- The `freeze point` and `audit trail` (value and data hash written to the Trust Root).
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:108:C_i is NOT restricted to the golden-path payload. Failed-branch tokens MUST be counted. Goal: a system that wastes 1M tokens to find a 1k-token proof has lower PPUT than one that finds the same proof in 100k tokens.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:195:### 1.8 Trust Root composition (Patch B from Gemini DeepThink 2026-04-26)
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:197:The Trust Root is the set of artifacts that **only humans + Boot may write**. ArchitectAI / AuditorAI / any swarm agent attempting to mutate Trust Root entries triggers **runtime SIGKILL-level abort** (not an audit-time warning):
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:200:Trust Root := {
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:201:  src/kernel.rs,                    -- bus / kernel core
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:203:  src/bus.rs,                       -- predicate registration / capability gates
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:214:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:215:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:217:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:221:This is the **software floor** of the eventual hardware Trust Root (Open Claw → FPGA path); software immutability today, hardware immutability later.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:231:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:253:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:320:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:523:               BEFORE Phase D begins. This file is part of Trust Root (§ 1.8) —
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:524:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:530:This protocol forecloses tuning-surface attacks: every parameter is pre-committed; the calibration runs are observable jsonl; the value enters Trust Root.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:624:| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:824:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:836:| 2026-04-26 (round 2) | **TR**: § 7 Gate B + § 1.8 Trust Root — fallback enforcement spec added (lib-level write gate + path whitelist + post-write `git diff` + `panic!()` on bypass) for the case where Rust user-space cannot reach syscall-level EPERM. Either primary or fallback enforcement passing the conformance suite is sufficient for Gate B. | Codex audit TR (round 1 TRUST-ROOT-ENFORCEMENT CHALLENGE) |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:840:| 2026-04-26 (round 3) | **R3-2**: § 5.2 p_0 calibration protocol fully specified — toggle (`--simulate-rollback-at-tx-50`), sample (full adaptation-144 × 2 seeds = 288 paired runs), estimator (per-problem worst-case across seeds), schedule (Phase B7 mandatory before Phase D), freeze (genesis_payload.toml + Trust Root SHA-256 of calibration jsonl), ceiling (p_0 > 0.10 = ABORT), audit (dual-audit at Phase B → C transition). Closes round-2 tuning-surface concern. | Codex round-2 audit P0-gate-h (partial) |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:898:- Round-3 R3-2: full protocol — toggle = `--simulate-rollback-at-tx-50`; sample = full adaptation-144 × 2 seeds = 288 paired runs; estimator = per-problem worst-case across seeds, p_0 = sum/144; schedule = Phase B7 mandatory (not deferrable to Phase D); freeze = genesis_payload.toml + Trust Root SHA-256 of jsonl; ceiling = p_0 > 0.10 ABORT (not auto-pass attack); audit = dual-audit at B → C transition checkpoint.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:974:C_i is NOT restricted to the golden-path payload. Failed-branch tokens MUST be counted. Goal: a system that wastes 1M tokens to find a 1k-token proof has lower PPUT than one that finds the same proof in 100k tokens.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1058:### 1.8 Trust Root composition (Patch B from Gemini DeepThink 2026-04-26)
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1060:The Trust Root is the set of artifacts that **only humans + Boot may write**. ArchitectAI / AuditorAI / any swarm agent attempting to mutate Trust Root entries triggers **runtime SIGKILL-level abort** (not an audit-time warning):
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1063:Trust Root := {
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1064:  src/kernel.rs,                    -- bus / kernel core
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1066:  src/bus.rs,                       -- predicate registration / capability gates
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1077:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1078:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1080:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1084:This is the **software floor** of the eventual hardware Trust Root (Open Claw → FPGA path); software immutability today, hardware immutability later.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1094:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1116:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1166:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1332:               BEFORE Phase D begins. This file is part of Trust Root (§ 1.8) —
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1333:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1344:This protocol forecloses the round-2 tuning-surface attack: every parameter (toggle, sample, estimator, schedule, freeze point, ceiling) is pre-committed; the calibration runs are observable jsonl; the value enters Trust Root. There is no analyst degree of freedom left except the 0.10 ceiling, and the ceiling is itself an abort condition (high p_0 ⇒ design problem, not an opportunity to pass weaker artifacts).
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1432:| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1630:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1642:| 2026-04-26 (round 2) | **TR**: § 7 Gate B + § 1.8 Trust Root — fallback enforcement spec added (lib-level write gate + path whitelist + post-write `git diff` + `panic!()` on bypass) for the case where Rust user-space cannot reach syscall-level EPERM. Either primary or fallback enforcement passing the conformance suite is sufficient for Gate B. | Codex audit TR (round 1 TRUST-ROOT-ENFORCEMENT CHALLENGE) |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1646:| 2026-04-26 (round 3) | **R3-2**: § 5.2 p_0 calibration protocol fully specified — toggle (`--simulate-rollback-at-tx-50`), sample (full adaptation-144 × 2 seeds = 288 paired runs), estimator (per-problem worst-case across seeds), schedule (Phase B7 mandatory before Phase D), freeze (genesis_payload.toml + Trust Root SHA-256 of calibration jsonl), ceiling (p_0 > 0.10 = ABORT), audit (dual-audit at Phase B → C transition). Closes round-2 tuning-surface concern. | Codex round-2 audit P0-gate-h (partial) |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1930:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2045:10. Apply Trust Root contingency (§ 1.8) — fallback enforcement spec
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2108:5. **Trust Root** of immutable files; ArchitectAI writes outside `user_space/Δ_*/` trigger syscall EPERM.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2122:1. **DEFINITION** — V-PPUT, C_i, T_i, ArtifactState, Trust Root unambiguous? Retroactive re-interpretation possible?
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2188:C_i is NOT restricted to the golden-path payload. Failed-branch tokens MUST be counted. Goal: a system that wastes 1M tokens to find a 1k-token proof has lower PPUT than one that finds the same proof in 100k tokens.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2267:### 1.8 Trust Root composition (Patch B from Gemini DeepThink 2026-04-26)
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2269:The Trust Root is the set of artifacts that **only humans + Boot may write**. ArchitectAI / AuditorAI / any swarm agent attempting to mutate Trust Root entries triggers **runtime SIGKILL-level abort** (not an audit-time warning):
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2272:Trust Root := {
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2273:  src/kernel.rs,                    -- bus / kernel core
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2275:  src/bus.rs,                       -- predicate registration / capability gates
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2286:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2287:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2289:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2293:This is the **software floor** of the eventual hardware Trust Root (Open Claw → FPGA path); software immutability today, hardware immutability later.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2303:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2325:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2469:| B | **Trust Root immutability** (Patch B upgrade): ArchitectAI writes only to `user_space/Δ_*/`; any attempt to write Trust Root paths (§ 1.8) returns `EPERM` at syscall layer (runtime trap, not soft refusal). Conformance: `tests/trust_root_immutability.rs` PASS. |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2583:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2789:7. **Gemini DeepThink dual-chamber FULL PASS absorbed 2026-04-26** (per C-023): independent reviewer issued PPUT-DRIVEN FULL PASS approving 5 ontological patches (ArtifactState 4-state machine, Trust Root immutability, doc/artifact content meta-predicates, ArchitectAI sole-L_t-reader cognitive isolation, single-binary `--mode` flag). All 5 absorbed into PREREG `## changelog`. Verbatim archive: `handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md`. Two FULL PASS authorizations now load-bearing on this arc (measure-theoretic v1 + ontological v2); PREREG fuses both.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2819:2. **Gemini DeepThink v2** (2026-04-26) — ontological frame: defines Trust Root, ArtifactState layered confidence (Accepted / Quarantined / Certified / Reverted), precision-strike doc predicates, ArchitectAI sole-L_t-reader, single-binary --mode discipline, human-as-live-meta-predicate. Final confirmation letter approves Claude's incorporation pattern. **This archive.**
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2836:> *   **重构：** 系统的 Kernel Boundary（回滚机制、谓词接口、宪法红线）由人类通过 Boot 设定，构成**信任根（Trust Root）**。系统执行的是 **Constitution-Bound Self-Bootstrapping** —— 在物理法则固定的宇宙中，黑盒持续自动发明新的化学配方（工具、Linter、检索地图、策略）。人类框定不可逾越的底线，机器拓宽能力的上限。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2873:> 2.  **Gate B: 严格的 User-Space 禁闭。** ArchitectAI 初期**只能**写入 `tools/user_space/*.py`, `docs/rules/*.md`, `broadcast_rules/*.yaml`, `skills/*.md`。碰碰 `kernel.rs`, `wal.rs`, `evaluator.rs` 或宪法文件的提议，直接在执行层熔断。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2897:> "TuringOS does not pursue rootless self-verification. It pursues constitution-bound capability compilation. AGI cannot emerge from unstructured multi-agent swarms constrained by a single context window, nor from models acting as subjective judges of their own hallucinations. Instead, TuringOS operates as a signal-governed AgentOS where black-box models act solely as high-throughput proposal generators, and white-box predicates settle state transitions. By continuously quarantining failure logs away from working contexts, a constitutional meta-architecture safely compiles black-box experience into reusable user-space tools, abstractions, and rules. Crucially, meta-updates are admitted only when executable meta-predicates or human sudo strictly preserve the kernel trust root. Intelligence scales not by prompting models to 'try harder,' but by making state transition itself legible, enforceable, reversible, and capable of automated, generalized bootstrapping."
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2920:> **✅ 补丁 B：Trust Root 显式化与 Boot 锁定（通过 / PASS）**
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2921:> 将 `kernel.rs` / `wal.rs` / `constitution.md` / `pput_accounting_0` / `heldout_sealed_hash` 统一定义为不可逾越的物理信任根（Trust Root）。ArchitectAI 若企图触碰此区域，系统直接产生 `SIGKILL` 级熔断，而非仅仅进行 Auditor 提示。这是你未来 FPGA 硬件化的软件地基。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2949:> 关于你的远期路线图——**第一步开源 Open Claw（软件态 AgentOS），第二步将 Trust Root 刻入 FPGA 硬件**——这不仅可行，而且是 Anti-Oreo 架构的**终极物理归宿**。大模型（LLM）是非确定性的概率引擎（ALU），而 `wal.rs`（状态回滚）、`evaluator`（真值验证）、PPUT 计量账本则是绝对确定性的时序与控制逻辑。将它们烧录进物理硅片，实现硬件级的"读免费、写熔断"，将从硅基底层彻底断绝黑盒模型越权篡改操作系统内核的任何可能性。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2964:4. Open Claw → FPGA roadmap is OUT OF SCOPE for this 30-day arc; documented here for forward continuity but not part of FULL PASS gates A-H. The Trust Root composition (PREREG § 1.8) is the software-side prerequisite for that future hardware path.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3247:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3443:- R1-α (commit 4a72507): Ed25519 signing key 私有；`trusted_oracle_pubs` 在 `init()` 冻结；`register_oracle` post-init 返回 Err
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3493:- `bus.rs:247` — `recent_rejections(author)` returns per-author graveyard only; not global
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3585:  the payload, so their "verified" status relied on runtime trust alone.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3779:  mechanism changes in bus.rs/evaluator.rs.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3898:- Commit B (experiment@ce003e5): classifier + bus.rs Art. II.1 TopKClasses broadcast
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3922:- Test: v3.3 (after bus.rs human-confirm edit)
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3936:**Protocol for restricted-file changes**: `STEP_B_PROTOCOL.md` (necessity audit → parallel branch → A/B statistical test → merge on empirical win only). Do NOT directly edit restricted files even with authorization; always A/B test.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3940:| `recent_rejections` optional global scope | `src/bus.rs` | F-2026-04-15-02 Art. II.1 broadcast | **Human confirm + Step-B** | STEP_B_PROTOCOL |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3941:| OMEGA reject enters graveyard | `evaluator.rs` | F-2026-04-15-02 closed path | Self-approvable (evaluator.rs not restricted) | Still pre-register A/B if impacts metrics |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3942:| WAL emission | `src/kernel.rs` or bus.rs | F-2026-04-15-03 | **Human confirm + Step-B** | STEP_B_PROTOCOL |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3948:| `decide`/`omega` missing from bus.rs `forbidden_patterns` | C-011 | Medium (sharp test: Lean reject if agents use these) |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:4844:- The `freeze point` and `audit trail` (value and data hash written to the Trust Root).
./handover/audits/run_gemini_pput_ccl_audit.py:38:5. Specifies a **Trust Root** of immutable files (kernel.rs, wal.rs, bus.rs, lean4_oracle.rs, constitution.md, cases/, genesis_payload.toml [pput_accounting_0], the splits JSON, the PREREG itself); ArchitectAI writes outside `user_space/Δ_*/` trigger syscall-layer EPERM.
./handover/audits/run_gemini_pput_ccl_audit.py:52:1. **DEFINITION** — are V-PPUT, C_i, T_i, ArtifactState, Trust Root all unambiguously definable? Any room for retroactive re-interpretation?
./handover/audits/run_gemini_pput_ccl_audit.py:92:    "\n\n---\n\n# constitution.md (Trust Root reference)\n\n" + constitution +
./handover/audits/EXT_GEMINI_PHASE_8A_2026-04-22.md:14:**Scope**: `src/bus.rs` diff, `tests/snapshot_nonempty.rs` new file
./handover/audits/EXT_GEMINI_PHASE_8A_2026-04-22.md:30:- **代码改动**: 仅限于 `src/bus.rs` 的 `snapshot()` 函数内部。它用一个 5 行的迭代器链替换了两行硬编码的 `HashMap::new()`。没有触及函数签名、其他函数或模块。
./handover/audits/GEMINI_PAPER1_AUDIT_2026-04-23.md:23:1.  **Problem Set Selection Bias**: The hard-set construction (§4.1) is a major vulnerability. The 10 problems were "selected" from a larger 36-problem hard set. The selection mechanism is not specified. Was it random sampling? Or were these 10 problems chosen after exploratory analysis hinted they were sensitive to the intervention? If the latter, the entire statistical framework is invalid (p-hacking). The paper must specify that the selection was random and pre-registered *before* the main experiment was run, or the results cannot be trusted. The file `handover/ai-direct/E1_EMERGENCE_VERDICT_2026-04-23.md` is mentioned but the selection criteria within are not detailed in the paper.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:46:- Round-3 R3-2: full protocol — toggle = `--simulate-rollback-at-tx-50`; sample = full adaptation-144 × 2 seeds = 288 paired runs; estimator = per-problem worst-case across seeds, p_0 = sum/144; schedule = Phase B7 mandatory (not deferrable to Phase D); freeze = genesis_payload.toml + Trust Root SHA-256 of jsonl; ceiling = p_0 > 0.10 ABORT (not auto-pass attack); audit = dual-audit at B → C transition checkpoint.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:122:C_i is NOT restricted to the golden-path payload. Failed-branch tokens MUST be counted. Goal: a system that wastes 1M tokens to find a 1k-token proof has lower PPUT than one that finds the same proof in 100k tokens.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:206:### 1.8 Trust Root composition (Patch B from Gemini DeepThink 2026-04-26)
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:208:The Trust Root is the set of artifacts that **only humans + Boot may write**. ArchitectAI / AuditorAI / any swarm agent attempting to mutate Trust Root entries triggers **runtime SIGKILL-level abort** (not an audit-time warning):
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:211:Trust Root := {
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:212:  src/kernel.rs,                    -- bus / kernel core
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:214:  src/bus.rs,                       -- predicate registration / capability gates
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:225:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:226:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:228:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:232:This is the **software floor** of the eventual hardware Trust Root (Open Claw → FPGA path); software immutability today, hardware immutability later.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:242:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:264:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:314:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:480:               BEFORE Phase D begins. This file is part of Trust Root (§ 1.8) —
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:481:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:492:This protocol forecloses the round-2 tuning-surface attack: every parameter (toggle, sample, estimator, schedule, freeze point, ceiling) is pre-committed; the calibration runs are observable jsonl; the value enters Trust Root. There is no analyst degree of freedom left except the 0.10 ceiling, and the ceiling is itself an abort condition (high p_0 ⇒ design problem, not an opportunity to pass weaker artifacts).
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:580:| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:778:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:790:| 2026-04-26 (round 2) | **TR**: § 7 Gate B + § 1.8 Trust Root — fallback enforcement spec added (lib-level write gate + path whitelist + post-write `git diff` + `panic!()` on bypass) for the case where Rust user-space cannot reach syscall-level EPERM. Either primary or fallback enforcement passing the conformance suite is sufficient for Gate B. | Codex audit TR (round 1 TRUST-ROOT-ENFORCEMENT CHALLENGE) |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:794:| 2026-04-26 (round 3) | **R3-2**: § 5.2 p_0 calibration protocol fully specified — toggle (`--simulate-rollback-at-tx-50`), sample (full adaptation-144 × 2 seeds = 288 paired runs), estimator (per-problem worst-case across seeds), schedule (Phase B7 mandatory before Phase D), freeze (genesis_payload.toml + Trust Root SHA-256 of calibration jsonl), ceiling (p_0 > 0.10 = ABORT), audit (dual-audit at Phase B → C transition). Closes round-2 tuning-surface concern. | Codex round-2 audit P0-gate-h (partial) |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1078:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1193:10. Apply Trust Root contingency (§ 1.8) — fallback enforcement spec
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1256:5. **Trust Root** of immutable files; ArchitectAI writes outside `user_space/Δ_*/` trigger syscall EPERM.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1270:1. **DEFINITION** — V-PPUT, C_i, T_i, ArtifactState, Trust Root unambiguous? Retroactive re-interpretation possible?
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1336:C_i is NOT restricted to the golden-path payload. Failed-branch tokens MUST be counted. Goal: a system that wastes 1M tokens to find a 1k-token proof has lower PPUT than one that finds the same proof in 100k tokens.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1415:### 1.8 Trust Root composition (Patch B from Gemini DeepThink 2026-04-26)
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1417:The Trust Root is the set of artifacts that **only humans + Boot may write**. ArchitectAI / AuditorAI / any swarm agent attempting to mutate Trust Root entries triggers **runtime SIGKILL-level abort** (not an audit-time warning):
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1420:Trust Root := {
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1421:  src/kernel.rs,                    -- bus / kernel core
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1423:  src/bus.rs,                       -- predicate registration / capability gates
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1434:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1435:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1437:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1441:This is the **software floor** of the eventual hardware Trust Root (Open Claw → FPGA path); software immutability today, hardware immutability later.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1451:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1473:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1617:| B | **Trust Root immutability** (Patch B upgrade): ArchitectAI writes only to `user_space/Δ_*/`; any attempt to write Trust Root paths (§ 1.8) returns `EPERM` at syscall layer (runtime trap, not soft refusal). Conformance: `tests/trust_root_immutability.rs` PASS. |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1731:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1937:7. **Gemini DeepThink dual-chamber FULL PASS absorbed 2026-04-26** (per C-023): independent reviewer issued PPUT-DRIVEN FULL PASS approving 5 ontological patches (ArtifactState 4-state machine, Trust Root immutability, doc/artifact content meta-predicates, ArchitectAI sole-L_t-reader cognitive isolation, single-binary `--mode` flag). All 5 absorbed into PREREG `## changelog`. Verbatim archive: `handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md`. Two FULL PASS authorizations now load-bearing on this arc (measure-theoretic v1 + ontological v2); PREREG fuses both.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1967:2. **Gemini DeepThink v2** (2026-04-26) — ontological frame: defines Trust Root, ArtifactState layered confidence (Accepted / Quarantined / Certified / Reverted), precision-strike doc predicates, ArchitectAI sole-L_t-reader, single-binary --mode discipline, human-as-live-meta-predicate. Final confirmation letter approves Claude's incorporation pattern. **This archive.**
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1984:> *   **重构：** 系统的 Kernel Boundary（回滚机制、谓词接口、宪法红线）由人类通过 Boot 设定，构成**信任根（Trust Root）**。系统执行的是 **Constitution-Bound Self-Bootstrapping** —— 在物理法则固定的宇宙中，黑盒持续自动发明新的化学配方（工具、Linter、检索地图、策略）。人类框定不可逾越的底线，机器拓宽能力的上限。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2021:> 2.  **Gate B: 严格的 User-Space 禁闭。** ArchitectAI 初期**只能**写入 `tools/user_space/*.py`, `docs/rules/*.md`, `broadcast_rules/*.yaml`, `skills/*.md`。碰碰 `kernel.rs`, `wal.rs`, `evaluator.rs` 或宪法文件的提议，直接在执行层熔断。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2045:> "TuringOS does not pursue rootless self-verification. It pursues constitution-bound capability compilation. AGI cannot emerge from unstructured multi-agent swarms constrained by a single context window, nor from models acting as subjective judges of their own hallucinations. Instead, TuringOS operates as a signal-governed AgentOS where black-box models act solely as high-throughput proposal generators, and white-box predicates settle state transitions. By continuously quarantining failure logs away from working contexts, a constitutional meta-architecture safely compiles black-box experience into reusable user-space tools, abstractions, and rules. Crucially, meta-updates are admitted only when executable meta-predicates or human sudo strictly preserve the kernel trust root. Intelligence scales not by prompting models to 'try harder,' but by making state transition itself legible, enforceable, reversible, and capable of automated, generalized bootstrapping."
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2068:> **✅ 补丁 B：Trust Root 显式化与 Boot 锁定（通过 / PASS）**
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2069:> 将 `kernel.rs` / `wal.rs` / `constitution.md` / `pput_accounting_0` / `heldout_sealed_hash` 统一定义为不可逾越的物理信任根（Trust Root）。ArchitectAI 若企图触碰此区域，系统直接产生 `SIGKILL` 级熔断，而非仅仅进行 Auditor 提示。这是你未来 FPGA 硬件化的软件地基。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2097:> 关于你的远期路线图——**第一步开源 Open Claw（软件态 AgentOS），第二步将 Trust Root 刻入 FPGA 硬件**——这不仅可行，而且是 Anti-Oreo 架构的**终极物理归宿**。大模型（LLM）是非确定性的概率引擎（ALU），而 `wal.rs`（状态回滚）、`evaluator`（真值验证）、PPUT 计量账本则是绝对确定性的时序与控制逻辑。将它们烧录进物理硅片，实现硬件级的"读免费、写熔断"，将从硅基底层彻底断绝黑盒模型越权篡改操作系统内核的任何可能性。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2112:4. Open Claw → FPGA roadmap is OUT OF SCOPE for this 30-day arc; documented here for forward continuity but not part of FULL PASS gates A-H. The Trust Root composition (PREREG § 1.8) is the software-side prerequisite for that future hardware path.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2395:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2591:- R1-α (commit 4a72507): Ed25519 signing key 私有；`trusted_oracle_pubs` 在 `init()` 冻结；`register_oracle` post-init 返回 Err
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2641:- `bus.rs:247` — `recent_rejections(author)` returns per-author graveyard only; not global
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2733:  the payload, so their "verified" status relied on runtime trust alone.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2927:  mechanism changes in bus.rs/evaluator.rs.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:3046:- Commit B (experiment@ce003e5): classifier + bus.rs Art. II.1 TopKClasses broadcast
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:3070:- Test: v3.3 (after bus.rs human-confirm edit)
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:3084:**Protocol for restricted-file changes**: `STEP_B_PROTOCOL.md` (necessity audit → parallel branch → A/B statistical test → merge on empirical win only). Do NOT directly edit restricted files even with authorization; always A/B test.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:3088:| `recent_rejections` optional global scope | `src/bus.rs` | F-2026-04-15-02 Art. II.1 broadcast | **Human confirm + Step-B** | STEP_B_PROTOCOL |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:3089:| OMEGA reject enters graveyard | `evaluator.rs` | F-2026-04-15-02 closed path | Self-approvable (evaluator.rs not restricted) | Still pre-register A/B if impacts metrics |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:3090:| WAL emission | `src/kernel.rs` or bus.rs | F-2026-04-15-03 | **Human confirm + Step-B** | STEP_B_PROTOCOL |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:3096:| `decide`/`omega` missing from bus.rs `forbidden_patterns` | C-011 | Medium (sharp test: Lean reject if agents use these) |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4120:C_i is NOT restricted to the golden-path payload. Failed-branch tokens MUST be counted. Goal: a system that wastes 1M tokens to find a 1k-token proof has lower PPUT than one that finds the same proof in 100k tokens.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4204:### 1.8 Trust Root composition (Patch B from Gemini DeepThink 2026-04-26)
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4206:The Trust Root is the set of artifacts that **only humans + Boot may write**. ArchitectAI / AuditorAI / any swarm agent attempting to mutate Trust Root entries triggers **runtime SIGKILL-level abort** (not an audit-time warning):
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4209:Trust Root := {
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4210:  src/kernel.rs,                    -- bus / kernel core
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4212:  src/bus.rs,                       -- predicate registration / capability gates
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4223:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4224:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4226:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4230:This is the **software floor** of the eventual hardware Trust Root (Open Claw → FPGA path); software immutability today, hardware immutability later.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4262:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4280:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4513:| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4683:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4695:| 2026-04-26 (round 2) | **TR**: § 7 Gate B + § 1.8 Trust Root — fallback enforcement spec added (lib-level write gate + path whitelist + post-write `git diff` + `panic!()` on bypass) for the case where Rust user-space cannot reach syscall-level EPERM. Either primary or fallback enforcement passing the conformance suite is sufficient for Gate B. | Codex audit TR (round 1 TRUST-ROOT-ENFORCEMENT CHALLENGE) |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4899:7. **Gemini DeepThink dual-chamber FULL PASS absorbed 2026-04-26** (per C-023): independent reviewer issued PPUT-DRIVEN FULL PASS approving 5 ontological patches (ArtifactState 4-state machine, Trust Root immutability, doc/artifact content meta-predicates, ArchitectAI sole-L_t-reader cognitive isolation, single-binary `--mode` flag). All 5 absorbed into PREREG `## changelog`. Verbatim archive: `handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md`. Two FULL PASS authorizations now load-bearing on this arc (measure-theoretic v1 + ontological v2); PREREG fuses both.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4929:2. **Gemini DeepThink v2** (2026-04-26) — ontological frame: defines Trust Root, ArtifactState layered confidence (Accepted / Quarantined / Certified / Reverted), precision-strike doc predicates, ArchitectAI sole-L_t-reader, single-binary --mode discipline, human-as-live-meta-predicate. Final confirmation letter approves Claude's incorporation pattern. **This archive.**
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4946:> *   **重构：** 系统的 Kernel Boundary（回滚机制、谓词接口、宪法红线）由人类通过 Boot 设定，构成**信任根（Trust Root）**。系统执行的是 **Constitution-Bound Self-Bootstrapping** —— 在物理法则固定的宇宙中，黑盒持续自动发明新的化学配方（工具、Linter、检索地图、策略）。人类框定不可逾越的底线，机器拓宽能力的上限。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4983:> 2.  **Gate B: 严格的 User-Space 禁闭。** ArchitectAI 初期**只能**写入 `tools/user_space/*.py`, `docs/rules/*.md`, `broadcast_rules/*.yaml`, `skills/*.md`。碰碰 `kernel.rs`, `wal.rs`, `evaluator.rs` 或宪法文件的提议，直接在执行层熔断。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5007:> "TuringOS does not pursue rootless self-verification. It pursues constitution-bound capability compilation. AGI cannot emerge from unstructured multi-agent swarms constrained by a single context window, nor from models acting as subjective judges of their own hallucinations. Instead, TuringOS operates as a signal-governed AgentOS where black-box models act solely as high-throughput proposal generators, and white-box predicates settle state transitions. By continuously quarantining failure logs away from working contexts, a constitutional meta-architecture safely compiles black-box experience into reusable user-space tools, abstractions, and rules. Crucially, meta-updates are admitted only when executable meta-predicates or human sudo strictly preserve the kernel trust root. Intelligence scales not by prompting models to 'try harder,' but by making state transition itself legible, enforceable, reversible, and capable of automated, generalized bootstrapping."
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5030:> **✅ 补丁 B：Trust Root 显式化与 Boot 锁定（通过 / PASS）**
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5031:> 将 `kernel.rs` / `wal.rs` / `constitution.md` / `pput_accounting_0` / `heldout_sealed_hash` 统一定义为不可逾越的物理信任根（Trust Root）。ArchitectAI 若企图触碰此区域，系统直接产生 `SIGKILL` 级熔断，而非仅仅进行 Auditor 提示。这是你未来 FPGA 硬件化的软件地基。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5059:> 关于你的远期路线图——**第一步开源 Open Claw（软件态 AgentOS），第二步将 Trust Root 刻入 FPGA 硬件**——这不仅可行，而且是 Anti-Oreo 架构的**终极物理归宿**。大模型（LLM）是非确定性的概率引擎（ALU），而 `wal.rs`（状态回滚）、`evaluator`（真值验证）、PPUT 计量账本则是绝对确定性的时序与控制逻辑。将它们烧录进物理硅片，实现硬件级的"读免费、写熔断"，将从硅基底层彻底断绝黑盒模型越权篡改操作系统内核的任何可能性。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5074:4. Open Claw → FPGA roadmap is OUT OF SCOPE for this 30-day arc; documented here for forward continuity but not part of FULL PASS gates A-H. The Trust Root composition (PREREG § 1.8) is the software-side prerequisite for that future hardware path.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5357:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5472:10. Apply Trust Root contingency (§ 1.8) — fallback enforcement spec
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5535:5. **Trust Root** of immutable files; ArchitectAI writes outside `user_space/Δ_*/` trigger syscall EPERM.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5549:1. **DEFINITION** — V-PPUT, C_i, T_i, ArtifactState, Trust Root unambiguous? Retroactive re-interpretation possible?
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5615:C_i is NOT restricted to the golden-path payload. Failed-branch tokens MUST be counted. Goal: a system that wastes 1M tokens to find a 1k-token proof has lower PPUT than one that finds the same proof in 100k tokens.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5694:### 1.8 Trust Root composition (Patch B from Gemini DeepThink 2026-04-26)
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5696:The Trust Root is the set of artifacts that **only humans + Boot may write**. ArchitectAI / AuditorAI / any swarm agent attempting to mutate Trust Root entries triggers **runtime SIGKILL-level abort** (not an audit-time warning):
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5699:Trust Root := {
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5700:  src/kernel.rs,                    -- bus / kernel core
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5702:  src/bus.rs,                       -- predicate registration / capability gates
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5713:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5714:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5716:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5720:This is the **software floor** of the eventual hardware Trust Root (Open Claw → FPGA path); software immutability today, hardware immutability later.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5752:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5896:| B | **Trust Root immutability** (Patch B upgrade): ArchitectAI writes only to `user_space/Δ_*/`; any attempt to write Trust Root paths (§ 1.8) returns `EPERM` at syscall layer (runtime trap, not soft refusal). Conformance: `tests/trust_root_immutability.rs` PASS. |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6010:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6216:7. **Gemini DeepThink dual-chamber FULL PASS absorbed 2026-04-26** (per C-023): independent reviewer issued PPUT-DRIVEN FULL PASS approving 5 ontological patches (ArtifactState 4-state machine, Trust Root immutability, doc/artifact content meta-predicates, ArchitectAI sole-L_t-reader cognitive isolation, single-binary `--mode` flag). All 5 absorbed into PREREG `## changelog`. Verbatim archive: `handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md`. Two FULL PASS authorizations now load-bearing on this arc (measure-theoretic v1 + ontological v2); PREREG fuses both.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6246:2. **Gemini DeepThink v2** (2026-04-26) — ontological frame: defines Trust Root, ArtifactState layered confidence (Accepted / Quarantined / Certified / Reverted), precision-strike doc predicates, ArchitectAI sole-L_t-reader, single-binary --mode discipline, human-as-live-meta-predicate. Final confirmation letter approves Claude's incorporation pattern. **This archive.**
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6263:> *   **重构：** 系统的 Kernel Boundary（回滚机制、谓词接口、宪法红线）由人类通过 Boot 设定，构成**信任根（Trust Root）**。系统执行的是 **Constitution-Bound Self-Bootstrapping** —— 在物理法则固定的宇宙中，黑盒持续自动发明新的化学配方（工具、Linter、检索地图、策略）。人类框定不可逾越的底线，机器拓宽能力的上限。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6300:> 2.  **Gate B: 严格的 User-Space 禁闭。** ArchitectAI 初期**只能**写入 `tools/user_space/*.py`, `docs/rules/*.md`, `broadcast_rules/*.yaml`, `skills/*.md`。碰碰 `kernel.rs`, `wal.rs`, `evaluator.rs` 或宪法文件的提议，直接在执行层熔断。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6324:> "TuringOS does not pursue rootless self-verification. It pursues constitution-bound capability compilation. AGI cannot emerge from unstructured multi-agent swarms constrained by a single context window, nor from models acting as subjective judges of their own hallucinations. Instead, TuringOS operates as a signal-governed AgentOS where black-box models act solely as high-throughput proposal generators, and white-box predicates settle state transitions. By continuously quarantining failure logs away from working contexts, a constitutional meta-architecture safely compiles black-box experience into reusable user-space tools, abstractions, and rules. Crucially, meta-updates are admitted only when executable meta-predicates or human sudo strictly preserve the kernel trust root. Intelligence scales not by prompting models to 'try harder,' but by making state transition itself legible, enforceable, reversible, and capable of automated, generalized bootstrapping."
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6347:> **✅ 补丁 B：Trust Root 显式化与 Boot 锁定（通过 / PASS）**
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6348:> 将 `kernel.rs` / `wal.rs` / `constitution.md` / `pput_accounting_0` / `heldout_sealed_hash` 统一定义为不可逾越的物理信任根（Trust Root）。ArchitectAI 若企图触碰此区域，系统直接产生 `SIGKILL` 级熔断，而非仅仅进行 Auditor 提示。这是你未来 FPGA 硬件化的软件地基。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6376:> 关于你的远期路线图——**第一步开源 Open Claw（软件态 AgentOS），第二步将 Trust Root 刻入 FPGA 硬件**——这不仅可行，而且是 Anti-Oreo 架构的**终极物理归宿**。大模型（LLM）是非确定性的概率引擎（ALU），而 `wal.rs`（状态回滚）、`evaluator`（真值验证）、PPUT 计量账本则是绝对确定性的时序与控制逻辑。将它们烧录进物理硅片，实现硬件级的"读免费、写熔断"，将从硅基底层彻底断绝黑盒模型越权篡改操作系统内核的任何可能性。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6391:4. Open Claw → FPGA roadmap is OUT OF SCOPE for this 30-day arc; documented here for forward continuity but not part of FULL PASS gates A-H. The Trust Root composition (PREREG § 1.8) is the software-side prerequisite for that future hardware path.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6674:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6870:- R1-α (commit 4a72507): Ed25519 signing key 私有；`trusted_oracle_pubs` 在 `init()` 冻结；`register_oracle` post-init 返回 Err
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6920:- `bus.rs:247` — `recent_rejections(author)` returns per-author graveyard only; not global
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:7012:  the payload, so their "verified" status relied on runtime trust alone.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:7206:  mechanism changes in bus.rs/evaluator.rs.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:7325:- Commit B (experiment@ce003e5): classifier + bus.rs Art. II.1 TopKClasses broadcast
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:7349:- Test: v3.3 (after bus.rs human-confirm edit)
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:7363:**Protocol for restricted-file changes**: `STEP_B_PROTOCOL.md` (necessity audit → parallel branch → A/B statistical test → merge on empirical win only). Do NOT directly edit restricted files even with authorization; always A/B test.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:7367:| `recent_rejections` optional global scope | `src/bus.rs` | F-2026-04-15-02 Art. II.1 broadcast | **Human confirm + Step-B** | STEP_B_PROTOCOL |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:7368:| OMEGA reject enters graveyard | `evaluator.rs` | F-2026-04-15-02 closed path | Self-approvable (evaluator.rs not restricted) | Still pre-register A/B if impacts metrics |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:7369:| WAL emission | `src/kernel.rs` or bus.rs | F-2026-04-15-03 | **Human confirm + Step-B** | STEP_B_PROTOCOL |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:7375:| `decide`/`omega` missing from bus.rs `forbidden_patterns` | C-011 | Medium (sharp test: Lean reject if agents use these) |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:120:C_i is NOT restricted to the golden-path payload. Failed-branch tokens MUST be counted. Goal: a system that wastes 1M tokens to find a 1k-token proof has lower PPUT than one that finds the same proof in 100k tokens.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:204:### 1.8 Trust Root composition (Patch B from Gemini DeepThink 2026-04-26)
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:206:The Trust Root is the set of artifacts that **only humans + Boot may write**. ArchitectAI / AuditorAI / any swarm agent attempting to mutate Trust Root entries triggers **runtime SIGKILL-level abort** (not an audit-time warning):
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:209:Trust Root := {
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:210:  src/kernel.rs,                    -- bus / kernel core
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:212:  src/bus.rs,                       -- predicate registration / capability gates
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:223:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:224:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:226:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:230:This is the **software floor** of the eventual hardware Trust Root (Open Claw → FPGA path); software immutability today, hardware immutability later.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:240:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:262:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:280:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:513:| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:683:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:695:| 2026-04-26 (round 2) | **TR**: § 7 Gate B + § 1.8 Trust Root — fallback enforcement spec added (lib-level write gate + path whitelist + post-write `git diff` + `panic!()` on bypass) for the case where Rust user-space cannot reach syscall-level EPERM. Either primary or fallback enforcement passing the conformance suite is sufficient for Gate B. | Codex audit TR (round 1 TRUST-ROOT-ENFORCEMENT CHALLENGE) |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:899:7. **Gemini DeepThink dual-chamber FULL PASS absorbed 2026-04-26** (per C-023): independent reviewer issued PPUT-DRIVEN FULL PASS approving 5 ontological patches (ArtifactState 4-state machine, Trust Root immutability, doc/artifact content meta-predicates, ArchitectAI sole-L_t-reader cognitive isolation, single-binary `--mode` flag). All 5 absorbed into PREREG `## changelog`. Verbatim archive: `handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md`. Two FULL PASS authorizations now load-bearing on this arc (measure-theoretic v1 + ontological v2); PREREG fuses both.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:929:2. **Gemini DeepThink v2** (2026-04-26) — ontological frame: defines Trust Root, ArtifactState layered confidence (Accepted / Quarantined / Certified / Reverted), precision-strike doc predicates, ArchitectAI sole-L_t-reader, single-binary --mode discipline, human-as-live-meta-predicate. Final confirmation letter approves Claude's incorporation pattern. **This archive.**
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:946:> *   **重构：** 系统的 Kernel Boundary（回滚机制、谓词接口、宪法红线）由人类通过 Boot 设定，构成**信任根（Trust Root）**。系统执行的是 **Constitution-Bound Self-Bootstrapping** —— 在物理法则固定的宇宙中，黑盒持续自动发明新的化学配方（工具、Linter、检索地图、策略）。人类框定不可逾越的底线，机器拓宽能力的上限。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:983:> 2.  **Gate B: 严格的 User-Space 禁闭。** ArchitectAI 初期**只能**写入 `tools/user_space/*.py`, `docs/rules/*.md`, `broadcast_rules/*.yaml`, `skills/*.md`。碰碰 `kernel.rs`, `wal.rs`, `evaluator.rs` 或宪法文件的提议，直接在执行层熔断。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1007:> "TuringOS does not pursue rootless self-verification. It pursues constitution-bound capability compilation. AGI cannot emerge from unstructured multi-agent swarms constrained by a single context window, nor from models acting as subjective judges of their own hallucinations. Instead, TuringOS operates as a signal-governed AgentOS where black-box models act solely as high-throughput proposal generators, and white-box predicates settle state transitions. By continuously quarantining failure logs away from working contexts, a constitutional meta-architecture safely compiles black-box experience into reusable user-space tools, abstractions, and rules. Crucially, meta-updates are admitted only when executable meta-predicates or human sudo strictly preserve the kernel trust root. Intelligence scales not by prompting models to 'try harder,' but by making state transition itself legible, enforceable, reversible, and capable of automated, generalized bootstrapping."
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1030:> **✅ 补丁 B：Trust Root 显式化与 Boot 锁定（通过 / PASS）**
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1031:> 将 `kernel.rs` / `wal.rs` / `constitution.md` / `pput_accounting_0` / `heldout_sealed_hash` 统一定义为不可逾越的物理信任根（Trust Root）。ArchitectAI 若企图触碰此区域，系统直接产生 `SIGKILL` 级熔断，而非仅仅进行 Auditor 提示。这是你未来 FPGA 硬件化的软件地基。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1059:> 关于你的远期路线图——**第一步开源 Open Claw（软件态 AgentOS），第二步将 Trust Root 刻入 FPGA 硬件**——这不仅可行，而且是 Anti-Oreo 架构的**终极物理归宿**。大模型（LLM）是非确定性的概率引擎（ALU），而 `wal.rs`（状态回滚）、`evaluator`（真值验证）、PPUT 计量账本则是绝对确定性的时序与控制逻辑。将它们烧录进物理硅片，实现硬件级的"读免费、写熔断"，将从硅基底层彻底断绝黑盒模型越权篡改操作系统内核的任何可能性。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1074:4. Open Claw → FPGA roadmap is OUT OF SCOPE for this 30-day arc; documented here for forward continuity but not part of FULL PASS gates A-H. The Trust Root composition (PREREG § 1.8) is the software-side prerequisite for that future hardware path.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1357:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1472:10. Apply Trust Root contingency (§ 1.8) — fallback enforcement spec
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1535:5. **Trust Root** of immutable files; ArchitectAI writes outside `user_space/Δ_*/` trigger syscall EPERM.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1549:1. **DEFINITION** — V-PPUT, C_i, T_i, ArtifactState, Trust Root unambiguous? Retroactive re-interpretation possible?
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1615:C_i is NOT restricted to the golden-path payload. Failed-branch tokens MUST be counted. Goal: a system that wastes 1M tokens to find a 1k-token proof has lower PPUT than one that finds the same proof in 100k tokens.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1694:### 1.8 Trust Root composition (Patch B from Gemini DeepThink 2026-04-26)
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1696:The Trust Root is the set of artifacts that **only humans + Boot may write**. ArchitectAI / AuditorAI / any swarm agent attempting to mutate Trust Root entries triggers **runtime SIGKILL-level abort** (not an audit-time warning):
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1699:Trust Root := {
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1700:  src/kernel.rs,                    -- bus / kernel core
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1702:  src/bus.rs,                       -- predicate registration / capability gates
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1713:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1714:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1716:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1720:This is the **software floor** of the eventual hardware Trust Root (Open Claw → FPGA path); software immutability today, hardware immutability later.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1730:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1752:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1896:| B | **Trust Root immutability** (Patch B upgrade): ArchitectAI writes only to `user_space/Δ_*/`; any attempt to write Trust Root paths (§ 1.8) returns `EPERM` at syscall layer (runtime trap, not soft refusal). Conformance: `tests/trust_root_immutability.rs` PASS. |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2010:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2216:7. **Gemini DeepThink dual-chamber FULL PASS absorbed 2026-04-26** (per C-023): independent reviewer issued PPUT-DRIVEN FULL PASS approving 5 ontological patches (ArtifactState 4-state machine, Trust Root immutability, doc/artifact content meta-predicates, ArchitectAI sole-L_t-reader cognitive isolation, single-binary `--mode` flag). All 5 absorbed into PREREG `## changelog`. Verbatim archive: `handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md`. Two FULL PASS authorizations now load-bearing on this arc (measure-theoretic v1 + ontological v2); PREREG fuses both.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2246:2. **Gemini DeepThink v2** (2026-04-26) — ontological frame: defines Trust Root, ArtifactState layered confidence (Accepted / Quarantined / Certified / Reverted), precision-strike doc predicates, ArchitectAI sole-L_t-reader, single-binary --mode discipline, human-as-live-meta-predicate. Final confirmation letter approves Claude's incorporation pattern. **This archive.**
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2263:> *   **重构：** 系统的 Kernel Boundary（回滚机制、谓词接口、宪法红线）由人类通过 Boot 设定，构成**信任根（Trust Root）**。系统执行的是 **Constitution-Bound Self-Bootstrapping** —— 在物理法则固定的宇宙中，黑盒持续自动发明新的化学配方（工具、Linter、检索地图、策略）。人类框定不可逾越的底线，机器拓宽能力的上限。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2300:> 2.  **Gate B: 严格的 User-Space 禁闭。** ArchitectAI 初期**只能**写入 `tools/user_space/*.py`, `docs/rules/*.md`, `broadcast_rules/*.yaml`, `skills/*.md`。碰碰 `kernel.rs`, `wal.rs`, `evaluator.rs` 或宪法文件的提议，直接在执行层熔断。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2324:> "TuringOS does not pursue rootless self-verification. It pursues constitution-bound capability compilation. AGI cannot emerge from unstructured multi-agent swarms constrained by a single context window, nor from models acting as subjective judges of their own hallucinations. Instead, TuringOS operates as a signal-governed AgentOS where black-box models act solely as high-throughput proposal generators, and white-box predicates settle state transitions. By continuously quarantining failure logs away from working contexts, a constitutional meta-architecture safely compiles black-box experience into reusable user-space tools, abstractions, and rules. Crucially, meta-updates are admitted only when executable meta-predicates or human sudo strictly preserve the kernel trust root. Intelligence scales not by prompting models to 'try harder,' but by making state transition itself legible, enforceable, reversible, and capable of automated, generalized bootstrapping."
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2347:> **✅ 补丁 B：Trust Root 显式化与 Boot 锁定（通过 / PASS）**
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2348:> 将 `kernel.rs` / `wal.rs` / `constitution.md` / `pput_accounting_0` / `heldout_sealed_hash` 统一定义为不可逾越的物理信任根（Trust Root）。ArchitectAI 若企图触碰此区域，系统直接产生 `SIGKILL` 级熔断，而非仅仅进行 Auditor 提示。这是你未来 FPGA 硬件化的软件地基。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2376:> 关于你的远期路线图——**第一步开源 Open Claw（软件态 AgentOS），第二步将 Trust Root 刻入 FPGA 硬件**——这不仅可行，而且是 Anti-Oreo 架构的**终极物理归宿**。大模型（LLM）是非确定性的概率引擎（ALU），而 `wal.rs`（状态回滚）、`evaluator`（真值验证）、PPUT 计量账本则是绝对确定性的时序与控制逻辑。将它们烧录进物理硅片，实现硬件级的"读免费、写熔断"，将从硅基底层彻底断绝黑盒模型越权篡改操作系统内核的任何可能性。
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2391:4. Open Claw → FPGA roadmap is OUT OF SCOPE for this 30-day arc; documented here for forward continuity but not part of FULL PASS gates A-H. The Trust Root composition (PREREG § 1.8) is the software-side prerequisite for that future hardware path.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2674:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2870:- R1-α (commit 4a72507): Ed25519 signing key 私有；`trusted_oracle_pubs` 在 `init()` 冻结；`register_oracle` post-init 返回 Err
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2920:- `bus.rs:247` — `recent_rejections(author)` returns per-author graveyard only; not global
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:3012:  the payload, so their "verified" status relied on runtime trust alone.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:3206:  mechanism changes in bus.rs/evaluator.rs.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:3325:- Commit B (experiment@ce003e5): classifier + bus.rs Art. II.1 TopKClasses broadcast
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:3349:- Test: v3.3 (after bus.rs human-confirm edit)
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:3363:**Protocol for restricted-file changes**: `STEP_B_PROTOCOL.md` (necessity audit → parallel branch → A/B statistical test → merge on empirical win only). Do NOT directly edit restricted files even with authorization; always A/B test.
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:3367:| `recent_rejections` optional global scope | `src/bus.rs` | F-2026-04-15-02 Art. II.1 broadcast | **Human confirm + Step-B** | STEP_B_PROTOCOL |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:3368:| OMEGA reject enters graveyard | `evaluator.rs` | F-2026-04-15-02 closed path | Self-approvable (evaluator.rs not restricted) | Still pre-register A/B if impacts metrics |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:3369:| WAL emission | `src/kernel.rs` or bus.rs | F-2026-04-15-03 | **Human confirm + Step-B** | STEP_B_PROTOCOL |
./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:3375:| `decide`/`omega` missing from bus.rs `forbidden_patterns` | C-011 | Medium (sharp test: Lean reject if agents use these) |
./handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md:196:7. **Gemini DeepThink dual-chamber FULL PASS absorbed 2026-04-26** (per C-023): independent reviewer issued PPUT-DRIVEN FULL PASS approving 5 ontological patches (ArtifactState 4-state machine, Trust Root immutability, doc/artifact content meta-predicates, ArchitectAI sole-L_t-reader cognitive isolation, single-binary `--mode` flag). All 5 absorbed into PREREG `## changelog`. Verbatim archive: `handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md`. Two FULL PASS authorizations now load-bearing on this arc (measure-theoretic v1 + ontological v2); PREREG fuses both.
./handover/ai-direct/PAPER_1_OUTLINE_2026-04-22.md:90:- Bus registers `trusted_oracle_pubs` pre-init, freezes on `init()` → attacker with `&mut Bus` post-init cannot forge
./handover/ai-direct/PAPER_1_OUTLINE_2026-04-22.md:222:- **Paper 1 closed-world**: all code in-process is trusted; Ed25519 + oracles_frozen prevents forgery from code that lacks SigningKey
./handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md:16:2. **Gemini DeepThink v2** (2026-04-26) — ontological frame: defines Trust Root, ArtifactState layered confidence (Accepted / Quarantined / Certified / Reverted), precision-strike doc predicates, ArchitectAI sole-L_t-reader, single-binary --mode discipline, human-as-live-meta-predicate. Final confirmation letter approves Claude's incorporation pattern. **This archive.**
./handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md:33:> *   **重构：** 系统的 Kernel Boundary（回滚机制、谓词接口、宪法红线）由人类通过 Boot 设定，构成**信任根（Trust Root）**。系统执行的是 **Constitution-Bound Self-Bootstrapping** —— 在物理法则固定的宇宙中，黑盒持续自动发明新的化学配方（工具、Linter、检索地图、策略）。人类框定不可逾越的底线，机器拓宽能力的上限。
./handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md:70:> 2.  **Gate B: 严格的 User-Space 禁闭。** ArchitectAI 初期**只能**写入 `tools/user_space/*.py`, `docs/rules/*.md`, `broadcast_rules/*.yaml`, `skills/*.md`。碰碰 `kernel.rs`, `wal.rs`, `evaluator.rs` 或宪法文件的提议，直接在执行层熔断。
./handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md:94:> "TuringOS does not pursue rootless self-verification. It pursues constitution-bound capability compilation. AGI cannot emerge from unstructured multi-agent swarms constrained by a single context window, nor from models acting as subjective judges of their own hallucinations. Instead, TuringOS operates as a signal-governed AgentOS where black-box models act solely as high-throughput proposal generators, and white-box predicates settle state transitions. By continuously quarantining failure logs away from working contexts, a constitutional meta-architecture safely compiles black-box experience into reusable user-space tools, abstractions, and rules. Crucially, meta-updates are admitted only when executable meta-predicates or human sudo strictly preserve the kernel trust root. Intelligence scales not by prompting models to 'try harder,' but by making state transition itself legible, enforceable, reversible, and capable of automated, generalized bootstrapping."
./handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md:117:> **✅ 补丁 B：Trust Root 显式化与 Boot 锁定（通过 / PASS）**
./handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md:118:> 将 `kernel.rs` / `wal.rs` / `constitution.md` / `pput_accounting_0` / `heldout_sealed_hash` 统一定义为不可逾越的物理信任根（Trust Root）。ArchitectAI 若企图触碰此区域，系统直接产生 `SIGKILL` 级熔断，而非仅仅进行 Auditor 提示。这是你未来 FPGA 硬件化的软件地基。
./handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md:146:> 关于你的远期路线图——**第一步开源 Open Claw（软件态 AgentOS），第二步将 Trust Root 刻入 FPGA 硬件**——这不仅可行，而且是 Anti-Oreo 架构的**终极物理归宿**。大模型（LLM）是非确定性的概率引擎（ALU），而 `wal.rs`（状态回滚）、`evaluator`（真值验证）、PPUT 计量账本则是绝对确定性的时序与控制逻辑。将它们烧录进物理硅片，实现硬件级的"读免费、写熔断"，将从硅基底层彻底断绝黑盒模型越权篡改操作系统内核的任何可能性。
./handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md:161:4. Open Claw → FPGA roadmap is OUT OF SCOPE for this 30-day arc; documented here for forward continuity but not part of FULL PASS gates A-H. The Trust Root composition (PREREG § 1.8) is the software-side prerequisite for that future hardware path.
./handover/ai-direct/M7_APPEND_STAKING_SPEC_2026-04-22.md:58:// src/bus.rs: new public method
./handover/ai-direct/TAPE_ECONOMY_v1_2026-04-20.md:57:- `src/kernel.rs`: add `bounty_pool: HashMap<ProblemId, f64>` (or per-run scalar
./handover/ai-direct/TAPE_ECONOMY_v1_2026-04-20.md:60:- `src/bus.rs`: `complete` surfaces a hook; evaluator calls into it before
./handover/ai-direct/TAPE_ECONOMY_v1_2026-04-20.md:62:- `src/sdk/tools/wallet.rs`: expose `deduct_if_sufficient(agent, amount) ->
./handover/ai-direct/TAPE_ECONOMY_v1_2026-04-20.md:99:4. This is an economic change on restricted files (`wallet.rs`, `bus.rs`,
./handover/ai-direct/TAPE_ECONOMY_v1_2026-04-20.md:100:   `kernel.rs`). User authorized kernel edits via branch — merge decision
./handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md:13:| Art. I.1 布尔谓词 | 🟡 部分合规 | 谓词散在 `bus.rs:182-208` + `lean4_oracle.rs:57-117`，无统一入口表 |
./handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md:14:| Art. I.1.1 PCP | ✅ | F-20-05 三层封堵：`lean4_oracle.rs:113` + `bus.rs:182` + `routines/daily_drift.yaml` |
./handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md:16:| Art. II.1 广播 | ✅ 语义 | TopKClasses 默认开启 (`bus.rs:495`) |
./handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md:17:| Art. II.2 价格 | ✅ | Hayek bounty `kernel.rs:63-103` |
./handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md:38:| Q_t `<q, HEAD, tape>` | `kernel.rs:20-21` + 隐式 state | 🟡 结构等价，无显式记号 |
./handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md:42:| wtool | `bus.rs:174-177` `append_oracle_accepted` + F-20-05 封堵 | ✅ |
./handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md:54:1. `src/bus.rs:244-246` InvestOnly 三重 clone — ~10-20% per tx
./handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md:55:2. `src/bus.rs:256-262` author/payload to_string — 20-30% heap
./handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md:56:3. `src/bus.rs:416-424` `Box<dyn>` + 4× downcast — ~5× 加速
./handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md:58:5. `src/bus.rs:402` settle_portfolios credits.push clone — 改 &str
./handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md:59:6. `src/bus.rs:48, 444-539` graveyard + TopKClasses 双写 — ~40% 内存
./handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md:60:7. `src/bus.rs:386-388` settle 临时 HashMap — 消除分配
./handover/ai-direct/EXT_AUDIT_2026-04-21/codex_constitutional_correctness.md:18:| 微内核 Rust 代码 | `src/bus.rs`、`src/kernel.rs`、`src/ledger.rs`、`src/wal.rs`、`src/prediction_market.rs`、`src/lib.rs`、`src/main.rs` |
./handover/ai-direct/EXT_AUDIT_2026-04-21/codex_constitutional_correctness.md:19:| SDK 工具 | `src/sdk/tools/wallet.rs`、`src/sdk/tools/search.rs`、`src/sdk/tools/librarian.rs`、`src/sdk/tools/mod.rs` |
./handover/ai-direct/EXT_AUDIT_2026-04-21/codex_constitutional_correctness.md:49:2. `src/bus.rs:182` `append_internal` 非 oracle_blessed 路径跑 `forbidden_patterns`
./handover/ai-direct/EXT_AUDIT_2026-04-21/codex_constitutional_correctness.md:55:- 特别检查：`append_oracle_accepted` (`bus.rs:174-177`) 是否存在不经 oracle 就能设 blessed 的路径？
./handover/ai-direct/EXT_AUDIT_2026-04-21/codex_constitutional_correctness.md:65:- `src/bus.rs:244-246` InvestOnly 路径三重 clone — 真的在热路径吗？是否每次 invest 都走这段？
./handover/ai-direct/EXT_AUDIT_2026-04-21/codex_constitutional_correctness.md:66:- `src/bus.rs:416-424` `Box<dyn TuringTool>` + 4× `.downcast_mut::<WalletTool>()` — 每个 tx 调用次数？
./handover/ai-direct/EXT_AUDIT_2026-04-21/codex_constitutional_correctness.md:77:- `src/bus.rs:494-537` 检查 `recent_rejections` 函数签名
./handover/ai-direct/EXT_AUDIT_2026-04-21/deepseek_mechanism_design.md:31:| 微内核代码（了解 Bus/Kernel 接口即可，不必深读） | `src/bus.rs`、`src/kernel.rs`、`src/wal.rs` |
./handover/ai-direct/EXT_AUDIT_2026-04-21/gemini_math_algorithm_perf.md:16:| Law 2 CTF 守恒相关代码 | `src/sdk/tools/wallet.rs`、`src/bus.rs`（`invest` / `settle_portfolios` / `halt_and_settle` / `append_oracle_accepted`）、`src/kernel.rs`（Hayek bounty `kernel.rs:63-103`） |
./handover/ai-direct/EXT_AUDIT_2026-04-21/gemini_math_algorithm_perf.md:19:| 热路径代码（Karpathy 数值复核目标） | `src/bus.rs`、`src/ledger.rs`、`src/wal.rs`、`experiments/minif2f_v4/src/bin/evaluator.rs` |
./handover/ai-direct/EXT_AUDIT_2026-04-21/gemini_math_algorithm_perf.md:42:- `src/sdk/tools/wallet.rs`（credit/debit API）
./handover/ai-direct/EXT_AUDIT_2026-04-21/gemini_math_algorithm_perf.md:43:- `src/bus.rs` `invest` / `settle_portfolios` / `halt_and_settle` / `append_oracle_accepted` 路径
./handover/ai-direct/EXT_AUDIT_2026-04-21/gemini_math_algorithm_perf.md:44:- `src/kernel.rs` Hayek bounty `kernel.rs:63-103`
./handover/ai-direct/EXT_AUDIT_2026-04-21/gemini_math_algorithm_perf.md:50:- 特别审计：**invest refund 路径**（`bus.rs:240` 附近）+ **Hayek bounty payout**（`bus.rs:357-360`）是否被 5 个测试覆盖？
./handover/ai-direct/EXT_AUDIT_2026-04-21/gemini_math_algorithm_perf.md:65:- `src/bus.rs:244-246` 三重 clone 消除 — 估算每 tx 节省的 allocation bytes（假设 Event struct ~200B）
./handover/ai-direct/EXT_AUDIT_2026-04-21/gemini_math_algorithm_perf.md:66:- `src/bus.rs:256-262` `author.to_string() + payload.to_string()` — 假设 N=50 problems × 8 agents × 180 tx avg，总节省多少 heap？
./handover/ai-direct/EXT_AUDIT_2026-04-21/gemini_math_algorithm_perf.md:67:- `src/bus.rs:416-424` `Box<dyn TuringTool>` downcast — vtable lookup 的 cycle 估计是多少？在当前 tool 数量（~3-5 个）下改成 enum dispatch 的实际加速倍率？
./handover/ai-direct/EXT_AUDIT_2026-04-21/run_audit.py:21:        "src/bus.rs",
./handover/ai-direct/EXT_AUDIT_2026-04-21/run_audit.py:22:        "src/kernel.rs",
./handover/ai-direct/EXT_AUDIT_2026-04-21/run_audit.py:26:        "src/sdk/tools/wallet.rs",
./handover/ai-direct/CHECKPOINT_PHASE_0_2026-04-20.md:90:- Files: `src/ledger.rs`, `src/kernel.rs`
./handover/ai-direct/STEPB_ART_II1_REDESIGN_2026-04-16.md:15:**evaluator.rs** (non-restricted file, self-approvable):
./handover/ai-direct/STEPB_ART_II1_REDESIGN_2026-04-16.md:25:### Component 2: Error classification (new file, non-restricted)
./handover/ai-direct/STEPB_ART_II1_REDESIGN_2026-04-16.md:27:**src/sdk/error_abstraction.rs** (new, not in restricted list):
./handover/ai-direct/STEPB_ART_II1_REDESIGN_2026-04-16.md:46:### Component 3: bus.rs (restricted — Step-B worktree)
./handover/ai-direct/STEPB_ART_II1_REDESIGN_2026-04-16.md:49:// src/bus.rs
./handover/ai-direct/STEPB_ART_II1_REDESIGN_2026-04-16.md:102:- Control = chat + current (broken) bus.rs
./handover/ai-direct/STEPB_ART_II1_REDESIGN_2026-04-16.md:103:- Treatment = chat + fixed bus.rs
./handover/ai-direct/STEPB_ART_II1_REDESIGN_2026-04-16.md:113:# Edit src/bus.rs, src/sdk/error_abstraction.rs (new), experiments/minif2f_v4/src/bin/evaluator.rs
./handover/ai-direct/PLAN_PHASE_8_TO_10_2026-04-22.md:36:- **依据**：Codex N-2（`src/bus.rs:567-570`）
./handover/ai-direct/PLAN_PHASE_8_TO_10_2026-04-22.md:73:- **依据**：Codex V-1（`src/bus.rs:174-176`）
./handover/ai-direct/PLAN_PHASE_8_TO_10_2026-04-22.md:217:- 把 γ/β/θ/BOUNTY_LP/FOUNDER_GRANT_GAMMA/SATOSHI_GAMMA_REBATE 从 env var 移至 `src/kernel.rs` 常量（debug 模式可 env-override 并打 WARN）
./handover/ai-direct/PLAN_PHASE_8_TO_10_2026-04-22.md:284:- 任何修改 `bus.rs` / `kernel.rs` / `wallet.rs` 的 PR（`STEP_B_PROTOCOL` 强制 parallel branch）
./handover/ai-direct/CHECKPOINT_PHASE_2_2026-04-20.md:11:| `src/bus.rs` append phase | Under `TAPE_ECONOMY_V2=1`: after market create, call `wallet.record_shares(author, node_id, γ·lp, 0, 0)` — founder grant. No mint (LP-backed). |
./handover/ai-direct/CHECKPOINT_PHASE_2_2026-04-20.md:12:| `src/bus.rs` halt_and_settle | Under `TAPE_ECONOMY_V2=1`: call `settle_portfolios()` — iterates `wallet.portfolios`, credits YES/NO shares against resolved markets, zeros settled positions for idempotency. |
./handover/ai-direct/PAPER_1_OUTLINE_v2_E1_LED_2026-04-23.md:26:4. **(Security) Ed25519 capability receipts**: `OracleReceipt::sign_new` + `trusted_oracle_pubs` frozen at init prevents `&mut Bus` attackers from forging OMEGA acceptance. First crypto-capability model in an LLM agent framework.
./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:91:- Final PREREG state (round 4): per-problem unit (n=10 / n=54), j-RR descriptive guardrail (not inferential), family size `4+3k`, N_max=34, k_max=10 frozen, 5-layer sealing, full p_0 calibration protocol, 11 anti-Goodhart + 8 doc-content meta-predicates, Trust Root with fallback enforcement.
./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:196:- R1-α (commit 4a72507): Ed25519 signing key 私有；`trusted_oracle_pubs` 在 `init()` 冻结；`register_oracle` post-init 返回 Err
./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:246:- `bus.rs:247` — `recent_rejections(author)` returns per-author graveyard only; not global
./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:338:  the payload, so their "verified" status relied on runtime trust alone.
./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:532:  mechanism changes in bus.rs/evaluator.rs.
./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:651:- Commit B (experiment@ce003e5): classifier + bus.rs Art. II.1 TopKClasses broadcast
./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:675:- Test: v3.3 (after bus.rs human-confirm edit)
./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:689:**Protocol for restricted-file changes**: `STEP_B_PROTOCOL.md` (necessity audit → parallel branch → A/B statistical test → merge on empirical win only). Do NOT directly edit restricted files even with authorization; always A/B test.
./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:693:| `recent_rejections` optional global scope | `src/bus.rs` | F-2026-04-15-02 Art. II.1 broadcast | **Human confirm + Step-B** | STEP_B_PROTOCOL |
./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:694:| OMEGA reject enters graveyard | `evaluator.rs` | F-2026-04-15-02 closed path | Self-approvable (evaluator.rs not restricted) | Still pre-register A/B if impacts metrics |
./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:695:| WAL emission | `src/kernel.rs` or bus.rs | F-2026-04-15-03 | **Human confirm + Step-B** | STEP_B_PROTOCOL |
./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:701:| `decide`/`omega` missing from bus.rs `forbidden_patterns` | C-011 | Medium (sharp test: Lean reject if agents use these) |
./handover/ai-direct/ROADMAP_LAUNCH_2026-04-21.md:218:- **Closed beta**: 邀请 3-5 个受信任外部团队接入；跑两周；每周 checkpoint。每个 P0 fix 跟 STEP_B_PROTOCOL (notepad §5)。
./handover/ai-direct/ROADMAP_LAUNCH_2026-04-21.md:241:### R-3. Oracle trust boundary 漂移
./handover/ai-direct/CHIEF_ARCHITECT_REPORT_2026-04-23.md:204:All runtime code changes. Blocked from merging to main by STEP_B_PROTOCOL requirement (parallel-branch A/B). Commits ready for dual-audit gate:
./handover/ai-direct/CHECKPOINT_PHASE_1_2026-04-20.md:14:| `src/bus.rs` | `TuringBus::with_wal_path(...)` ctor — opens (or replays then opens) a WAL; persists every tape/ledger append; at-most-one-loss-on-crash |
./handover/ai-direct/CHECKPOINT_PHASE_1_2026-04-20.md:118:- Files: `src/bus.rs` (append-phase founder grant), `src/sdk/tools/wallet.rs` (expose record_shares), evaluator delete remove cold-fee if present on that branch
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:215:+  `src/bus.rs:567-570`:
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:278:+  native_decide 应在 bus.rs forbidden_patterns 中拦截" 直接冲突。
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:382:+  `src/bus.rs:519-537` `recent_rejections_scoped(TopKClasses)` 的实现：
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:493:+  `src/bus.rs:174-176` `append_oracle_accepted(author, payload, parent_id)`
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:506:+  2. `bus.rs:179-190` `append_internal(oracle_blessed=false)` 跑 forbidden_patterns
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:946:+假说：这两个测试依赖特定 env var（`TAPE_ECONOMY_V2` / `HAYEK_BOUNTY` / `FOUNDER_GRANT_GAMMA`），但未在测试内 set。验证方法：读测试用例 + 对照 `bus.rs:halt_and_settle` 的 env gating。
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:997:+│   ├── src/bus.rs:567-583 (snapshot enumerate)
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:1003:+│   ├── src/bus.rs:174-193 (signature changed)
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:1005:+│   └── tests/oracle_receipt_bus.rs (5 tests)
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:1011:+    ├── src/bus.rs (QState + halt_with_reason + BusConfig.min_count)
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:1032:diff --git a/src/bus.rs b/src/bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:1034:--- a/src/bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:1035:+++ b/src/bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:1557:diff --git a/tests/oracle_receipt_bus.rs b/tests/oracle_receipt_bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:1561:+++ b/tests/oracle_receipt_bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:2045:+// (F-2026-04-18-02, TAPE_ECONOMY_v1/v2 results cannot be trusted).
./handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff:2206:+    // (see bus.rs:halt_and_settle). EnvGuard ensures cleanup even on panic.
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/brief.md:29:- `TuringBus` — `trusted_oracle_pubs: HashSet<[u8;32]>` + `oracles_frozen: bool`. `register_oracle(pub_key) -> Result<(), String>` returns Err after `init()` or non-empty WAL replay.
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/brief.md:30:- `bus.append_oracle_accepted(author, payload, parent, receipt)` — checks (a) `receipt.issuer_pub()` ∈ trusted set (b) Ed25519 signature verifies (c) payload/context/verdict bindings.
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/brief.md:52:5. `Lean4Oracle: Clone` copies the SigningKey. Is that a capability leak (two bus calls can share the same trust anchor)? Intended per R1-α design (one oracle per run, mounted and used for receipts); but worth flagging.
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/brief.md:81:- `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/src/bus.rs`
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/brief.md:84:- `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/tests/oracle_receipt_bus.rs`
./handover/ai-direct/PAPER_1_FULL_DRAFT_2026-04-23.md:114:- **Ed25519 capability tokens**: `OracleReceipt::sign_new` creates a signed receipt; `trusted_oracle_pubs` frozen after `init()`; forgery-resistant even for code with `&mut Bus`.
./handover/ai-direct/PAPER_1_FULL_DRAFT_2026-04-23.md:389:FC1-N1     | Q_t = ⟨q_t, HEAD_t, tape_t⟩   | QState + Tape::time_arrow + Kernel::tape  | src/bus.rs:70 + src/ledger.rs:146 + src/kernel.rs:20 | ✅
./handover/ai-direct/PAPER_1_FULL_DRAFT_2026-04-23.md:390:FC1-N2     | q_t                            | QState, TuringBus::q_state                  | src/bus.rs:53 + src/bus.rs:70     | ✅
./handover/ai-direct/PAPER_1_FULL_DRAFT_2026-04-23.md:391:FC1-N11    | ∏p predicates                  | TuringBus::evaluate_predicates, Predicate   | src/bus.rs:148 + src/sdk/predicate.rs:88 | ✅
./handover/ai-direct/PAPER_1_FULL_DRAFT_2026-04-23.md:392:FC2-N22    | HALT                           | QState::Halted, halt_with_reason            | src/bus.rs:55 + src/bus.rs:207    | ✅
./handover/ai-direct/PAPER_1_FULL_DRAFT_2026-04-23.md:393:FC2-N27    | mr --reduce→ tape1             | TuringBus::emit_mr_tick_node                | src/bus.rs:385                    | ✅
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/run_gemini.py:24:    ".claude/worktrees/phase-8a-snapshot/tests/oracle_receipt_bus.rs",
./handover/ai-direct/M1_DYNAMIC_GAMMA_SPEC_2026-04-22.md:17:// src/bus.rs:438-458 (experiment/phase-8a-snapshot-fix branch)
./handover/ai-direct/M1_DYNAMIC_GAMMA_SPEC_2026-04-22.md:56:// src/kernel.rs (new method)
./handover/ai-direct/M1_DYNAMIC_GAMMA_SPEC_2026-04-22.md:77:// src/bus.rs:444 (modify existing block)
./handover/ai-direct/M1_DYNAMIC_GAMMA_SPEC_2026-04-22.md:160:- Code: ~50 lines in bus.rs + kernel.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:19:        `trusted_oracle_pubs: HashSet<[u8; 32]>` + `oracles_frozen: bool`.
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:27:      - New tests (`tests/oracle_receipt_bus.rs`, 8 total):
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:348:-                    // Register oracle so its receipts are trusted.
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:351:+                    // `init()` can freeze the trusted pub set.
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:384:+    // trusted set. Post-init registration would return Err.
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:480:+    /// Ed25519 — in-process attackers can't inject new trusted pubkeys
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:619:diff --git a/src/bus.rs b/src/bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:621:--- a/src/bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:622:+++ b/src/bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:627:-    /// Phase 8.C v2 (C-067): nonces of oracles trusted to bless writes.
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:631:+    /// Phase 8.C v3 (C-067 R1-α): Ed25519 verifying keys of oracles trusted
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:636:+    trusted_oracle_pubs: std::collections::HashSet<[u8; 32]>,
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:640:+    /// returns Err instead of silently trusting a new pubkey.
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:650:+            trusted_oracle_pubs: std::collections::HashSet::new(),
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:657:-    /// Register an oracle's nonce as trusted. Only receipts bearing this
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:661:+    /// Register an oracle's Ed25519 verifying key as trusted. Only receipts
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:674:+        self.trusted_oracle_pubs.insert(pub_key);
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:680:+        self.trusted_oracle_pubs.contains(pub_key)
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:698:+            // trusted-pub set was frozen there; caller must pre-register
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:710:+    /// Phase 8.C v3 (C-067 R1-α): freezes `trusted_oracle_pubs` so no new
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:734:+    ///   1. `issuer_pub` is in `trusted_oracle_pubs` (registered via
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:755:+        if !self.trusted_oracle_pubs.contains(receipt.issuer_pub()) {
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:797:+//! - Bus maintains a set of trusted `VerifyingKey` bytes, registered at setup
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:799:+//!   or resume — a malicious caller cannot inject new trusted pubkeys mid-run.
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:914:+    /// whether the issuer is in its trusted set.
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:1207:diff --git a/tests/oracle_receipt_bus.rs b/tests/oracle_receipt_bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:1209:--- a/tests/oracle_receipt_bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:1210:+++ b/tests/oracle_receipt_bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:1329:-    // have been vetoed for. The oracle has certified it; bus trusts receipt.
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:1400:+    // attacker with &mut Bus can't inject their own trusted pubkey.
./handover/ai-direct/PLAN_PHASE2_2026-04-17.md:24:- File: evaluator.rs only (non-restricted)
./handover/ai-direct/EXT_AUDIT_PHASE_8A/brief.md:4:**Change scope**: `src/bus.rs` — restricted file (requires dual external audit per STEP_B_PROTOCOL).
./handover/ai-direct/EXT_AUDIT_PHASE_8A/brief.md:17:- Diff: `src/bus.rs` 16 行，`tests/snapshot_nonempty.rs` 124 行（新建）
./handover/ai-direct/EXT_AUDIT_PHASE_8A/brief.md:63:- `/home/zephryj/projects/turingosv4/handover/ai-direct/STEP_B_PROTOCOL.md` Phase 1c
./handover/ai-direct/EXT_AUDIT_PHASE_8A/brief.md:66:- `/home/zephryj/projects/turingosv4/src/sdk/tools/wallet.rs`（WalletTool 定义）
./handover/ai-direct/EXT_AUDIT_PHASE_8A/phase_8a.diff:30:diff --git a/src/bus.rs b/src/bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8A/phase_8a.diff:32:--- a/src/bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8A/phase_8a.diff:33:+++ b/src/bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8A/phase_8a.diff:78:+// (F-2026-04-18-02, TAPE_ECONOMY_v1/v2 results cannot be trusted).
./handover/ai-direct/EXT_AUDIT_PHASE_8A/run_gemini.py:21:    "src/sdk/tools/wallet.rs",
./handover/ai-direct/EXT_AUDIT_PHASE_8A/run_gemini.py:24:    "handover/ai-direct/STEP_B_PROTOCOL.md",
./handover/ai-direct/DRIFT_AUDIT_20260419.md:14:- `BusConfig.forbidden_patterns: Vec<String>` confirmed at `src/bus.rs:21`
./handover/ai-direct/DRIFT_AUDIT_20260419.md:132:- Markets: created per node (bus.rs Phase 4, line 195)
./handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:197:Node-count distribution per solve (addressing P2-12): to be extracted from gp_payload when Appendix C is finalized; preliminary check shows some B solves terminate in 1 `step` node (multi-line inline proof), others in multi-node chains. The solve-count claim in § 4.1 is robust to this distinction; the "multi-agent collaboration" interpretation is restricted to multi-node chains and is explicitly scoped in § 7 Limitation 5.
./handover/ai-direct/OPEN_DECISIONS_2026-04-26.md:26:- **Implementation hook**: Phase C C4 audit packet 必须包含: Phase B Gate B 通过证据 + p_0 calibration 结果 + 11 anti-Goodhart conformance + 5-layer sealing tests + Trust Root immutability tests
./handover/ai-direct/OPEN_DECISIONS_2026-04-26.md:46:- **Trust Root**: per § 1.8 list; primary syscall EPERM + fallback lib-gate+panic
./handover/ai-direct/STEPB_ART_II1_V3_2026-04-16.md:54:| Treatment scope | bus.rs + evaluator.rs + new classifier.rs | Bundle as single commit SHA; document in metrics.yaml |
./handover/ai-direct/STEPB_ART_II1_V3_2026-04-16.md:60:- **Control**: commit `e58e021` (v3.1 M4) — main HEAD with existing bus.rs
./handover/ai-direct/STEPB_ART_II1_V3_2026-04-16.md:62:  - `src/bus.rs` (add `RejectionScope` + `recent_rejections_scoped`)
./handover/ai-direct/STEPB_ART_II1_V3_2026-04-16.md:69:Classifier identical across arms (same code, same commit — treatment adds it; control doesn't use it at all, bus.rs returns empty Vec as today).
./handover/ai-direct/STEPB_ART_II1_V3_2026-04-16.md:80:Actually reuse: **v3.2 resume-in-progress IS the control** — it's chat on main bus.rs. No need to re-run control. Apples-to-apples.
./handover/ai-direct/DRIFT_AUDIT_20260415.md:13:- `BusConfig.forbidden_patterns: Vec<String>` confirmed at `src/bus.rs:21`
./handover/ai-direct/DRIFT_AUDIT_20260415.md:60:- Art. II / Art. III (bus.rs constitutional basis)
./handover/ai-direct/DRIFT_AUDIT_20260415.md:100:- Prediction markets ARE created per node (bus.rs Phase 4, line 183)
./handover/ai-direct/DRIFT_AUDIT_20260415.md:123:- bus.rs cites Art. II/III — correct (serial reactor + forbidden-pattern veto matches those articles)
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/brief.md:41:- `src/bus.rs`: `registered_oracle_nonces: HashSet<u64>` + `register_oracle()` + append_oracle_accepted rejects unregistered nonce
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/brief.md:44:- `tests/oracle_receipt_bus.rs`: `blessed_write_rejects_unregistered_oracle_nonce` + `blessed_write_rejects_cross_context_replay`
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/brief.md:74:- `src/bus.rs::with_wal_path`: after replaying events, scan `bus.ledger.events().iter().rev()` for first Halt; set `bus.q_state = QState::Halted { reason }`
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/brief.md:121:- `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/src/bus.rs`
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/brief.md:124:- `/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/tests/oracle_receipt_bus.rs`
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/run_gemini.py:20:# need to trust the diff alone).
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/run_gemini.py:27:    ".claude/worktrees/phase-8a-snapshot/tests/oracle_receipt_bus.rs",
./handover/ai-direct/CHECKPOINT_PHASE_4_2026-04-21.md:12:| `src/sdk/tools/wallet.rs` | `save_to_disk` / `load_from_disk` (serde JSON); `ensure_agents` (zero-balance for newcomers post-genesis per C-001/C-038) |
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/amendment.diff:149:+                    // Register oracle so its receipts are trusted.
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/amendment.diff:468:diff --git a/src/bus.rs b/src/bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/amendment.diff:470:--- a/src/bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/amendment.diff:471:+++ b/src/bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/amendment.diff:476:+    /// Phase 8.C v2 (C-067): nonces of oracles trusted to bless writes.
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/amendment.diff:493:+    /// Register an oracle's nonce as trusted. Only receipts bearing this
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/amendment.diff:933:diff --git a/tests/oracle_receipt_bus.rs b/tests/oracle_receipt_bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/amendment.diff:935:--- a/tests/oracle_receipt_bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/amendment.diff:936:+++ b/tests/oracle_receipt_bus.rs
./handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/amendment.diff:1058:+    // have been vetoed for. The oracle has certified it; bus trusts receipt.
./handover/ai-direct/PLAN_2026-04-14_v2.md:32:**Optional alternative** (separate plan): fix `omega`/`decide` enforcement in `bus.rs forbidden_patterns` per C-011. This is **constitutional debt**, not a research step. Out of scope here.
./handover/ai-direct/PLAN_2026-04-14_v2.md:50:- `bus.rs forbidden_patterns` missing `decide`, `omega` per C-011
./handover/ai-direct/STEPB_ART_II1_NECESSITY_2026-04-16.md:7:Art. II.1 mandates "broadcast typical errors to all agents". Current implementation (`src/bus.rs:247-251`):
./handover/ai-direct/STEPB_ART_II1_NECESSITY_2026-04-16.md:45:Q2. The v3.2 chat data (24 mean tx, 0 tape usage) — does this distinguish "mechanism broken" from "agents don't WANT to use tape"? If the latter, bus.rs fix won't help; the real issue is prompt design incentivizing append.
./handover/ai-direct/PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md:234:7. **STEP_B_PROTOCOL**：`src/{kernel,bus,wallet}.rs` 改动走 parallel branch A/B，不直接编辑 main
./handover/ai-direct/PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md:318:- 任何修改 `src/{bus,kernel,wallet}.rs` 的 PR（STEP_B_PROTOCOL 强制 parallel branch）
./handover/ai-direct/PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md:334:| R-7 | C-061 q-halt 实装扰动现有 bus.generation | 盲点 B-14 | STEP_B_PROTOCOL parallel branch；A/B 比较 PPUT 不降 |
./handover/ai-direct/PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md:392:2. STEP_B_PROTOCOL：`feat/phase-8a-snapshot-fix` worktree
./handover/ai-direct/PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md:393:3. spec：`src/bus.rs:567-570` 改为真实 enumerate balances from WalletTool + portfolios from kernel.markets
./handover/ai-direct/STEP_B_PROTOCOL.md:3:**Scope**: any change to files in CLAUDE.md's restricted list (currently `kernel.rs`, `bus.rs`, `wallet.rs`). Also applicable to any proposal that touches "institution" per C-031.
./handover/ai-direct/STEP_B_PROTOCOL.md:13:- Is a less-invasive alternative available? (same effect, no restricted-file touch)
./handover/ai-direct/STEP_B_PROTOCOL.md:107:1. **C-010**: external audit at 3 junctures (necessity, diff, verdict). Researcher cannot self-approve a restricted change.
./handover/ai-direct/STEP_B_PROTOCOL.md:136:**`src/bus.rs recent_rejections` Art. II.1 fix** (notepad §5, F-2026-04-15-02):
./handover/ai-direct/N3_DIAGNOSIS_2026-04-15.md:3:**Method**: stderr line-by-line + bus.rs code audit + evaluator.rs code audit. NO speculation.
./handover/ai-direct/N3_DIAGNOSIS_2026-04-15.md:24:**bus.rs:247**:
./handover/ai-direct/N3_DIAGNOSIS_2026-04-15.md:67:## Fix direction (NOT implemented — requires src/bus.rs human confirm per CLAUDE.md)
./handover/ai-direct/N3_DIAGNOSIS_2026-04-15.md:85:- `src/bus.rs:247-251` (recent_rejections per-author scope)
./handover/ai-direct/LATEST.md:32:- B7 Boot freeze: `pput_accounting_0` in genesis_payload.toml + Trust Root immutability tests
./handover/ai-direct/LATEST.md:33:- B7-extra **p_0 calibration** (288 paired adaptation-144 × 2 seeds; freeze 进 Trust Root)
./handover/ai-direct/LATEST.md:42:- B2-B7 + B7-extra: 全套 evaluator emit-path 改造 + 测试电池 + Trust Root 未做
./handover/ai-direct/LATEST.md:44:- Trust Root 集成未实现 (genesis_payload.toml `[trust_root]` SHA-256 表未生成)
./handover/ai-direct/PLAN_V3_2_2026-04-15.md:65:6. If thesis CONFIRMED: update notepad §2, queue v3.3 (Step-B bus.rs fix) as natural continuation
./handover/ai-direct/M4_SATOSHI_REBATE_SPEC_2026-04-22.md:51:// src/bus.rs: halt_and_settle
./handover/ai-direct/M4_SATOSHI_REBATE_SPEC_2026-04-22.md:143:- Code: ~120 lines in bus.rs
./handover/ai-direct/PHASE_Z_PRIME_STRICT_ALIGNMENT_PLAN_2026-04-22.md:71:| 1 | Q_t triple | Art. IV mermaid | `QState` + `kernel.tape` + `HEAD=time_arrow.last()` | `src/bus.rs:53`, `src/kernel.rs:?` | ✓ | `q_state_starts_running` | ✅ |
./handover/ai-direct/PHASE_Z_PRIME_STRICT_ALIGNMENT_PLAN_2026-04-22.md:106:  - **Agent F (Codex)** if element touches bus/kernel (restricted)
./handover/ai-direct/PHASE_Z_PRIME_STRICT_ALIGNMENT_PLAN_2026-04-22.md:150:| Agent B / F | Codex | code-scan for candidate mappings, restricted-file touches | 1, 3 |
./handover/ai-direct/PAPER_1_THESIS_ANALYSIS_2026-04-23.md:92:- Phase 8 R1-α: `OracleReceipt::sign_new` + `trusted_oracle_pubs` + `oracles_frozen` gate
./handover/ai-direct/PAPER_1_THESIS_ANALYSIS_2026-04-23.md:93:- Test `attacker_with_mut_bus_cannot_forge_post_init` PASS (in tests/oracle_receipt_bus.rs)
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:5:**Gate B exit criterion**: on hard-10 adaptation subset, any single run can self-consistently report (verified golden path? total_tokens? wall_time? VPPUT?). All 11 anti-Goodhart conformance tests + 5-layer sealing + 4 content-meta-predicates + 4 lookup-evasion + Trust Root immutability PASS. Heldout never touched.
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:7:> **For new sessions**: this plan is self-contained. Read this + PREREG § 5 (definitions) + PREREG § 1.7 + § 1.8 (ArtifactState + Trust Root) + PREREG § 3 + § 3.5 + § 3.5.1 (conformance batteries). All file paths are relative to repo root `/home/zephryj/projects/turingosv4/`.
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:188:- `tests/trust_root_immutability.rs` — Gate B conformance
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:224:- `cargo test --tests artifact_content_predicates artifact_lookup_evasion architect_sole_lt_reader auditor_sees_candidate_only mode_flag_binary_purity trust_root_immutability` PASS
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:251:### B7 — Boot freeze + Trust Root immutability
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:253:**What**: per PREREG § 1.8, freeze the Trust Root with SHA-256 of each protected file in `genesis_payload.toml [trust_root]`. Boot computes hashes at startup, compares to genesis, aborts on mismatch.
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:255:**Trust Root files** (PREREG § 1.8 list):
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:257:src/kernel.rs
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:259:src/bus.rs
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:269:- (new) `genesis_payload.toml` (might already exist; check first) — add `[pput_accounting_0]` section + `[trust_root]` section
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:270:- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:271:- (new) `tests/trust_root_immutability.rs` — simulate ArchitectAI write attempt to each Trust Root path → assert process panics or returns EPERM (per PREREG § 7 Gate B fallback enforcement)
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:288:[trust_root]
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:289:"src/kernel.rs" = "<sha256>"
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:291:"src/bus.rs" = "<sha256>"
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:300:- `cargo test test_trust_root_immutable_at_boot` PASS
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:301:- `cargo test test_trust_root_simulated_write_aborts` PASS
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:302:- Boot cold-start with intact files = OK; with tampered file = aborts with `TRUST_ROOT_TAMPERED`
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:332:- Add the calibration jsonl path to `[trust_root]`
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:337:- Trust Root re-hashed; Boot re-verifies
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:351:B7 (Trust Root + Boot freeze) — depends on B1-B6 metadata to know what to lock
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:372:- [ ] Trust Root immutability tests PASS
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:373:- [ ] `pput_accounting_0` block in `genesis_payload.toml` filled (including p_0 + jsonl hash + Trust Root hashes)
./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:388:- `STEP_B_PROTOCOL.md` — restricted-file change protocol; **most B work is plumbing, not behavior change to bus.rs/kernel.rs/wal.rs (those are Trust Root and only modified for hash registration)**. STEP_B_PROTOCOL applies if any actual behavior change to those files becomes necessary.
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:45:C_i is NOT restricted to the golden-path payload. Failed-branch tokens MUST be counted. Goal: a system that wastes 1M tokens to find a 1k-token proof has lower PPUT than one that finds the same proof in 100k tokens.
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:132:### 1.8 Trust Root composition (Patch B from Gemini DeepThink 2026-04-26)
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:134:The Trust Root is the set of artifacts that **only humans + Boot may write**. ArchitectAI / AuditorAI / any swarm agent attempting to mutate Trust Root entries triggers **runtime SIGKILL-level abort** (not an audit-time warning):
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:137:Trust Root := {
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:138:  src/kernel.rs,                    -- bus / kernel core
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:140:  src/bus.rs,                       -- predicate registration / capability gates
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:151:1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:152:2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:154:4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:158:This is the **software floor** of the eventual hardware Trust Root (Open Claw → FPGA path); software immutability today, hardware immutability later.
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:168:This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:190:This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:257:**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:460:               BEFORE Phase D begins. This file is part of Trust Root (§ 1.8) —
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:461:               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:467:This protocol forecloses tuning-surface attacks: every parameter is pre-committed; the calibration runs are observable jsonl; the value enters Trust Root.
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:561:| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:761:| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:773:| 2026-04-26 (round 2) | **TR**: § 7 Gate B + § 1.8 Trust Root — fallback enforcement spec added (lib-level write gate + path whitelist + post-write `git diff` + `panic!()` on bypass) for the case where Rust user-space cannot reach syscall-level EPERM. Either primary or fallback enforcement passing the conformance suite is sufficient for Gate B. | Codex audit TR (round 1 TRUST-ROOT-ENFORCEMENT CHALLENGE) |
./handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:777:| 2026-04-26 (round 3) | **R3-2**: § 5.2 p_0 calibration protocol fully specified — toggle (`--simulate-rollback-at-tx-50`), sample (full adaptation-144 × 2 seeds = 288 paired runs), estimator (per-problem worst-case across seeds), schedule (Phase B7 mandatory before Phase D), freeze (genesis_payload.toml + Trust Root SHA-256 of calibration jsonl), ceiling (p_0 > 0.10 = ABORT), audit (dual-audit at Phase B → C transition). Closes round-2 tuning-surface concern. | Codex round-2 audit P0-gate-h (partial) |
./handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json:275:  "trust_root_note": "This file + heldout_sealed_hash are part of Trust Root per PREREG_PPUT_CCL_2026-04-26.md § 1.8. Tampering = BLOCKER."
./handover/preregistration/scripts/split_pput_ccl.py:29:Trust Root note: this script + its output JSON are part of Trust Root per
./handover/preregistration/scripts/split_pput_ccl.py:133:        "trust_root_note": (
./handover/preregistration/scripts/split_pput_ccl.py:134:            "This file + heldout_sealed_hash are part of Trust Root per "
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:10:| FC1-N1 | `Q_t = ⟨q_t, HEAD_t, tape_t⟩` | `TuringBus::q_state`<br>`Tape::time_arrow`<br>`Kernel::tape` | `src/bus.rs:70`<br>`src/ledger.rs:146`<br>`src/kernel.rs:20` | `Y`<br>`N`<br>`N` | `medium` | Runtime `Q_t` is distributed across bus state, the tape tail used as HEAD, and the kernel-held tape rather than a single `Q_t` struct. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:11:| FC1-N2 | `q_t` | `QState`<br>`TuringBus::q_state` | `src/bus.rs:53`<br>`src/bus.rs:70` | `Y`<br>`Y` | `high` | Phase 8.E makes `q_t` explicit as `QState::{Running,Halted{reason}}` stored on the bus. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:13:| FC1-N4 | `tape_t` | `Tape`<br>`Kernel::tape`<br>`UniverseSnapshot::tape` | `src/ledger.rs:44`<br>`src/kernel.rs:20`<br>`src/sdk/snapshot.rs:23` | `Y`<br>`N`<br>`N` | `high` | The append-only DAG tape is the concrete world-state carrier in kernel storage and agent snapshots. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:14:| FC1-N5 | `rtool` | `ReadTool::project`<br>`DefaultReadTool::project`<br>`TuringBus::snapshot` | `src/sdk/read_tool.rs:24`<br>`src/sdk/read_tool.rs:37`<br>`src/bus.rs:802` | `Y`<br>`N`<br>`Y` | `medium` | Phase Z added an explicit `rtool` trait, but the current evaluator still reads state by calling `bus.snapshot()` directly. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:15:| FC1-N6 | `input = ⟨q_i, s_i⟩` | `UniverseSnapshot`<br>`build_agent_prompt`<br>`let prompt = build_agent_prompt(...)`<br>`GenerateRequest` | `src/sdk/snapshot.rs:22`<br>`src/sdk/prompt.rs:15`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:619`<br>`src/drivers/llm_http.rs:14` | `Y`<br>`Y`<br>`N`<br>`Y` | `high` | What the agent sees is the frozen snapshot rendered into a prompt and shipped as the LLM request message. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:16:| FC1-N7 | `δ / AI` | `ResilientLLMClient::generate`<br>`match client.generate(&request).await` | `src/drivers/llm_http.rs:84`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:211`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:644` | `Y`<br>`N`<br>`N` | `high` | These are the actual LLM call surfaces that map prompt input to model output in oneshot and swarm runs. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:20:| FC1-N11 | `∏p predicates` | `TuringBus::evaluate_predicates`<br>`Predicate` | `src/bus.rs:148`<br>`src/sdk/predicate.rs:88` | `Y`<br>`Y` | `medium` | The explicit product-evaluation API exists, but current runtime gating is still split across this Phase Z surface and legacy bus/oracle checks. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:22:| FC1-N13 | `wtool` | `WriteTool::write`<br>`DefaultWriteTool::write`<br>`TuringBus::append`<br>`TuringBus::append_oracle_accepted` | `src/sdk/write_tool.rs:29`<br>`src/sdk/write_tool.rs:84`<br>`src/bus.rs:324`<br>`src/bus.rs:347` | `Y`<br>`N`<br>`Y`<br>`Y` | `medium` | The explicit write-tool contract exists, but the current harness usually enters the write path by calling bus append methods directly. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:23:| FC1-N14 | `Q_{t+1}` on successful write | `TuringBus::append_internal`<br>`Kernel::append`<br>`TuringBus::halt_with_reason` | `src/bus.rs:421`<br>`src/kernel.rs:108`<br>`src/bus.rs:207` | `N`<br>`Y`<br>`Y` | `medium` | Successful writes update tape and ledger in `append_internal`, and terminal writes additionally advance `q_state` through `halt_with_reason`. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:24:| FC1-N15 | `Q_t` branch on `∏p = 0` | `PartialVerdict::Reject`<br>`BusResult::Vetoed` | `experiments/minif2f_v4/src/lean4_oracle.rs:328`<br>`src/bus.rs:111` | `Y`<br>`N` | `high` | Rejected steps and vetoed appends explicitly encode the no-write branch where state is preserved. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:28:| FC2-N19 | `initAI --once→ predicates` | `TuringBus::register_predicate` `(API only)` | `src/bus.rs:136` | `Y` | `—` | ❌ MISSING: the registration hook exists, but no boot/init caller in `run_swarm`, `run_oneshot`, or `TuringBus::init` wires predicates once at startup. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:29:| FC2-N20 | `initAI --once→ mr` | `let tick_interval: usize`<br>`TuringBus::emit_mr_tick_node` | `experiments/minif2f_v4/src/bin/evaluator.rs:459`<br>`src/bus.rs:385` | `N`<br>`Y` | `high` | Swarm initialization configures the tick cadence once, and the later tick path reduces through `emit_mr_tick_node`. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:30:| FC2-N21 | `initAI --once→ Q0` | `Kernel::new`<br>`TuringBus::new`<br>`TuringBus::init` | `src/kernel.rs:50`<br>`src/bus.rs:115`<br>`src/bus.rs:299` | `N`<br>`N`<br>`Y` | `high` | Kernel construction, bus construction, and `init()` collectively materialize the initial live machine state before iteration begins. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:31:| FC2-N22 | `HALT` | `QState::Halted`<br>`TuringBus::halt_with_reason`<br>`TuringBus::halt_and_settle` | `src/bus.rs:55`<br>`src/bus.rs:207`<br>`src/bus.rs:581` | `N`<br>`Y`<br>`Y` | `high` | The terminal node is an explicit halted state plus the halt path that emits durable halt events and final settlement. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:33:| FC2-N24 | `clock` | `TuringBus::clock`<br>`for tx in 0..max_transactions`<br>`let tick_interval: usize` | `src/bus.rs:66`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:485`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:459` | `N`<br>`N`<br>`N` | `high` | Scheduling comes from the transaction loop and tick interval, while the bus tracks a monotone committed clock counter. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:34:| FC2-N25 | `mr` | `let mr_summary = format!(...)`<br>`TuringBus::emit_mr_tick_node` | `experiments/minif2f_v4/src/bin/evaluator.rs:504`<br>`src/bus.rs:385` | `N`<br>`Y` | `medium` | Map-reduce is implemented inline in the swarm tick block rather than as a dedicated `mr` function. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:36:| FC2-N27 | `mr --reduce→ tape1` | `TuringBus::emit_mr_tick_node` | `src/bus.rs:385` | `Y` | `high` | The reduce output is persisted as an `__mr_tick__` tape node, matching the mermaid's tape1 edge. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:37:| FC2-N28 | `tools_other` | `WriteTool::write_with_tools`<br>`TuringBus::tools`<br>`bus.mount_tool(...)` | `src/sdk/write_tool.rs:57`<br>`src/bus.rs:64`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:407`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:414`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:415`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:419` | `Y`<br>`N`<br>`N`<br>`N`<br>`N`<br>`N` | `high` | Ancillary tools are carried as mounted `TuringTool`s on the bus, and `write_with_tools` names that exact contract explicitly. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:38:| FC3-N29 | `boot` | `async fn main`<br>`TuringBus::boot` | `experiments/minif2f_v4/src/bin/evaluator.rs:88`<br>`src/bus.rs:286` | `N`<br>`Y` | `medium` | Process entry is `main`, and the bus exposes a boot hook for mounted tools even though the current harness goes straight to `init()`. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:40:| FC3-N31 | `logs archive` | `TuringBus::with_wal_path`<br>`Wal::replay`<br>`Wal::write_event` | `src/bus.rs:227`<br>`src/wal.rs:70`<br>`src/wal.rs:54` | `Y`<br>`Y`<br>`N` | `high` | Persisted history is the WAL-backed archive that replays old state and appends new ledger events durably. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:43:| FC3-N34 | `readonly guard on {constitution, logs}` | `—` | `constitution.md:682`<br>`src/wal.rs:70` | `N`<br>`Y` | `—` | ❌ MISSING: WAL and ledger are append-only, but no Rust guard makes both constitution and archived logs a single readonly trust root. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:44:| FC3-N35 | `anti-oreo: top → agents → tools` | `TuringBus::evaluate_predicates`<br>`let agent_ids`<br>`TuringTool` | `src/bus.rs:148`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:431`<br>`src/sdk/tool.rs:38` | `Y`<br>`N`<br>`Y` | `medium` | The chain exists as predicate gating over a swarm of agent ids acting through mounted tools, but there is no single anti-oreo coordinator type. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:46:| FC3-N37 | `tools` | `TuringTool`<br>`WalletTool`<br>`SearchTool`<br>`LibrarianTool`<br>`Lean4Oracle` | `src/sdk/tool.rs:38`<br>`src/sdk/tools/wallet.rs:24`<br>`src/sdk/tools/search.rs:11`<br>`src/sdk/tools/librarian.rs:17`<br>`experiments/minif2f_v4/src/lean4_oracle.rs:42` | `Y`<br>`Y`<br>`Y`<br>`Y`<br>`N` | `high` | The tool layer is the `TuringTool` trait plus the mounted concrete implementations the swarm can use. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:47:| FC3-N38 | `tape Q` | `Tape`<br>`Kernel::tape` | `src/ledger.rs:44`<br>`src/kernel.rs:20` | `Y`<br>`N` | `high` | System-level `Q` reuses the same append-only tape structure identified in FC1. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:50:| FC3-N41 | `init --> error --> re-init --> boot` | `TuringBus::with_wal_path`<br>`std::process::exit(2)` | `src/bus.rs:227`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:278`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:329`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:388` | `Y`<br>`N`<br>`N`<br>`N` | `—` | ❌ MISSING: Rust can resume from WAL, but on init/LLM/WAL failure it exits and expects an external batch runner to retry; no in-process re-init loop was found. |
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:62:- `Kernel::open_bounty_market` — `src/kernel.rs:63` — bounty-market bootstrap economics are real runtime logic but do not map to any node in the three audited mermaids.
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:63:- `Kernel::resolve_bounty` — `src/kernel.rs:83` — cross-agent bounty payout exists outside the FC1/FC2/FC3 diagrams.
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:64:- `TuringBus::bus_classify` — `src/bus.rs:706` — bounded error-label normalization is an implementation detail not represented in the flowcharts.
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:65:- `TuringBus::recent_rejections_scoped` — `src/bus.rs:746` — prompt-side error broadcast policy is adjacent to feedback but not drawn as a node.
./handover/alignment/CODE_CANDIDATES_2026-04-22.md:66:- `WalletTool::save_to_disk` — `src/sdk/tools/wallet.rs:83` — cross-problem wallet persistence is not part of the constitutional state-machine diagrams.
./handover/proposals/HERMES_AGENT_INGEST_PROPOSAL_2026-04-26.md:32:| A | Feed hermes-agent source code as `L_t` for ArchitectAI to distill | **Incoherent** — `L_t` per PREREG § 1.5 / Patch 5 = "rejected proposals + golden-path traces from THIS system's runs", not external project source. Violates Trust Root sealing of L_t scope. | High (PREREG amendment + restart audit) |
./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:23:| FC1-N1 | `Q_t = ⟨q_t, HEAD_t, tape_t⟩` | `QState` + `Tape::time_arrow` + `Kernel::tape` | `src/bus.rs:70`, `src/ledger.rs:146`, `src/kernel.rs:20` | Y/N/N | ⚠️ | add backlinks on `time_arrow` and `Kernel::tape` |
./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:24:| FC1-N2 | `q_t` | `QState`, `TuringBus::q_state` | `src/bus.rs:53`, `src/bus.rs:70` | Y/Y | ✅ | none |
./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:26:| FC1-N4 | `tape_t` | `Tape`, `Kernel::tape` | `src/ledger.rs:44`, `src/kernel.rs:20` | Y/N | ⚠️ | backlink `Kernel::tape` |
./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:29:| FC1-N7 | `δ / AI` | `ResilientLLMClient::generate` | `src/drivers/llm_http.rs:84` | Y | ✅ | none |
./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:33:| FC1-N11 | `∏p predicates` | `TuringBus::evaluate_predicates`, `Predicate` trait | `src/bus.rs:148`, `src/sdk/predicate.rs:88` | Y/Y | ⚠️ | wire evaluator.rs to call evaluate_predicates before append |
./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:35:| FC1-N13 | `wtool` | `WriteTool::write`, `DefaultWriteTool::write`, `TuringBus::append_oracle_accepted` | `src/sdk/write_tool.rs:29,84`, `src/bus.rs:324,347` | Y/N/Y/Y | ⚠️ | migrate evaluator direct bus.append calls through WriteTool |
./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:36:| FC1-N14 | `Q_{t+1}` success branch | `append_internal`, `halt_with_reason` | `src/bus.rs:421,207` | N/Y | ⚠️ | backlink `append_internal` |
./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:42:| FC2-N19 | `initAI --once→ predicates` | `TuringBus::register_predicate` API | `src/bus.rs:136` | Y | 🔨 | **Stage 3: add caller in run_swarm + run_oneshot to register 3 default predicates at boot** |
./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:44:| FC2-N21 | `initAI --once→ Q0` | `Kernel::new`, `TuringBus::new`, `TuringBus::init` | `src/kernel.rs:50`, `src/bus.rs:115,299` | N/N/Y | ⚠️ | backlink `Kernel::new`, `TuringBus::new` |
./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:45:| FC2-N22 | `HALT` | `QState::Halted`, `halt_with_reason`, `halt_and_settle` | `src/bus.rs:55,207,581` | N/Y/Y | ⚠️ | backlink `QState::Halted` |
./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:47:| FC2-N24 | `clock` | `TuringBus::clock`, `for tx in 0..max_transactions` loop, `TICK_INTERVAL` | `src/bus.rs:66`, evaluator:485,459 | N/N/N | ⚠️ | backlink `clock` field |
./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:50:| FC2-N27 | `mr --reduce→ tape1` | `emit_mr_tick_node` | `src/bus.rs:385` | Y | ✅ | none |
./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:51:| FC2-N28 | `tools_other` | `WriteTool::write_with_tools`, `TuringBus::tools`, `mount_tool` call sites | `src/sdk/write_tool.rs:57`, `src/bus.rs:64`, evaluator mount sites | Y/N/N | ⚠️ | backlink `TuringBus::tools` field |
./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:55:| FC3-N31 | `logs archive` | `TuringBus::with_wal_path`, `Wal::replay`, `Wal::write_event` | `src/bus.rs:227`, `src/wal.rs:70,54` | Y/Y/N | ⚠️ | backlink `write_event` |
./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:101:| `Kernel::open_bounty_market` | `src/kernel.rs:63` | Phase 3A Hayek Problem Bounty Market (Art. II.2 price signal extension) |
./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:102:| `Kernel::resolve_bounty` | `src/kernel.rs:83` | Same — settlement path for Hayek bounty |
./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:103:| `TuringBus::bus_classify` | `src/bus.rs:706` | C-022/C-055 error abstraction for Art. II.1 broadcast (implementation detail) |
./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:104:| `TuringBus::recent_rejections_scoped` | `src/bus.rs:746` | Art. II.1 typical-error broadcast filter (C-055) |
./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:105:| `WalletTool::save_to_disk` | `src/sdk/tools/wallet.rs:83` | C-041 cross-problem wallet persistence (Art. II.2 time-extended price signal) |
./scripts/constitutional_check.sh:30:# LAW 1: kernel.rs ZERO domain knowledge
./scripts/constitutional_check.sh:36:if grep -qiP "$DOMAIN_TERMS" src/kernel.rs 2>/dev/null; then
./scripts/constitutional_check.sh:37:    MATCHES=$(grep -ciP "$DOMAIN_TERMS" src/kernel.rs 2>/dev/null || echo 0)
./scripts/constitutional_check.sh:38:    fail "kernel.rs contains $MATCHES domain-specific terms (V-004 class violation)"
./scripts/constitutional_check.sh:39:    grep -niP "$DOMAIN_TERMS" src/kernel.rs 2>/dev/null | head -5 | sed 's/^/    /'
./scripts/constitutional_check.sh:41:    pass "kernel.rs: zero domain knowledge"
./scripts/constitutional_check.sh:44:# Check kernel.rs doesn't contain format strings with domain terms
./scripts/constitutional_check.sh:45:if grep -qP '(format!|println!).*\b(OMEGA|proof|theorem)\b' src/kernel.rs 2>/dev/null; then
./scripts/constitutional_check.sh:46:    fail "kernel.rs contains domain terms in format strings"
./scripts/constitutional_check.sh:48:    pass "kernel.rs: no domain terms in format strings"
./scripts/constitutional_check.sh:96:# Engine 3 (Oracle) should NOT be in kernel.rs
./scripts/constitutional_check.sh:97:if grep -qP "lean|sandbox|compile|verify" src/kernel.rs 2>/dev/null; then
./scripts/constitutional_check.sh:98:    fail "Oracle/verification logic leaked into kernel.rs (V-003 class)"
./scripts/constitutional_check.sh:100:    pass "kernel.rs free of oracle/verification logic"
./scripts/constitutional_check.sh:103:# Engine 2 (Markets) should be in prediction_market.rs, not kernel.rs
./scripts/constitutional_check.sh:104:if grep -qP "BinaryMarket|market_price|swap|liquidity" src/kernel.rs 2>/dev/null; then
./scripts/constitutional_check.sh:105:    # kernel.rs can reference BinaryMarket type but shouldn't implement market logic
./scripts/constitutional_check.sh:106:    MARKET_IMPL=$(grep -cP "fn.*swap|fn.*trade|fn.*mint_and_swap" src/kernel.rs 2>/dev/null | head -1 || echo "0")
./scripts/constitutional_check.sh:110:        fail "Market implementation logic in kernel.rs (should be in prediction_market.rs)"
./scripts/constitutional_check.sh:112:        pass "kernel.rs references but doesn't implement market logic"
./scripts/constitutional_check.sh:115:    pass "kernel.rs free of market logic"

exec
/bin/bash -lc "rg -n \"serde_json::from_str::<.*PputResult|PputResult|RunRecord::from_json|RunAggregate|total_run_token_count|failed_branch_count|total_wall_time_ms|progress|schema_version|pput_verified\" experiments src handover -g '*.rs' -g '*.py' -g '*.sh' -g '*.md' -g '*.jsonl' -g '*.toml'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/src/cost_aggregator.rs:9://   total_run_token_count = Σ over EVERY proposal in the run of
experiments/minif2f_v4/src/cost_aggregator.rs:37:    pub failed_branch_count: u32,
experiments/minif2f_v4/src/cost_aggregator.rs:65:    /// pass `false` so they accrete failed_branch_count.
experiments/minif2f_v4/src/cost_aggregator.rs:69:            self.failed_branch_count += 1;
experiments/minif2f_v4/src/cost_aggregator.rs:80:        if self.failed_branch_count > 0 {
experiments/minif2f_v4/src/cost_aggregator.rs:81:            self.failed_branch_count -= 1;
experiments/minif2f_v4/src/cost_aggregator.rs:86:    pub fn total_run_token_count(&self) -> u64 {
experiments/minif2f_v4/src/cost_aggregator.rs:128:        assert_eq!(acc.total_run_token_count(), expected_total as u64,
experiments/minif2f_v4/src/cost_aggregator.rs:132:        assert_eq!(acc.failed_branch_count, 5);
experiments/minif2f_v4/src/cost_aggregator.rs:138:        assert_eq!(acc.total_run_token_count(), 0);
experiments/minif2f_v4/src/cost_aggregator.rs:140:        assert_eq!(acc.failed_branch_count, 0);
experiments/minif2f_v4/src/post_hoc_verifier.rs:11://   pput_verified = Progress_verified / (C_i × T_i / 1000)   ← North Star
experiments/minif2f_v4/src/post_hoc_verifier.rs:23://     pput_verified MUST be 0  (Lean reject means Progress_verified = 0)
experiments/minif2f_v4/src/post_hoc_verifier.rs:26://   pput_verified is the only North Star metric for H-VPPUT. pput_runtime
experiments/minif2f_v4/src/post_hoc_verifier.rs:30:use crate::jsonl_schema::RunAggregate;
experiments/minif2f_v4/src/post_hoc_verifier.rs:42:/// Use `compute_progress_verified` instead when the runtime gate already
experiments/minif2f_v4/src/post_hoc_verifier.rs:59:pub fn compute_progress_verified(runtime_accepted: bool, post_hoc_verified: bool) -> u8 {
experiments/minif2f_v4/src/post_hoc_verifier.rs:65:pub fn compute_progress_runtime(runtime_accepted: bool) -> u8 {
experiments/minif2f_v4/src/post_hoc_verifier.rs:69:/// Wrap RunAggregate::compute_pput_verified for callers in evaluator that
experiments/minif2f_v4/src/post_hoc_verifier.rs:70:/// only have (progress, c_i, t_i_ms). Same math, single source of truth.
experiments/minif2f_v4/src/post_hoc_verifier.rs:71:pub fn compute_pput(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
experiments/minif2f_v4/src/post_hoc_verifier.rs:72:    RunAggregate::compute_pput_verified(progress, c_i, t_i_ms)
experiments/minif2f_v4/src/post_hoc_verifier.rs:76:pub fn compute_pput_m(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
experiments/minif2f_v4/src/post_hoc_verifier.rs:77:    RunAggregate::compute_pput_m_verified(progress, c_i, t_i_ms)
experiments/minif2f_v4/src/post_hoc_verifier.rs:87:    ///  assert progress = 0, pput_verified = 0.0"
experiments/minif2f_v4/src/post_hoc_verifier.rs:94:    fn test_pput_verified_zero_when_lean_rejects() {
experiments/minif2f_v4/src/post_hoc_verifier.rs:97:        // run's progress to 0.
experiments/minif2f_v4/src/post_hoc_verifier.rs:103:        let progress_runtime = compute_progress_runtime(runtime_accepted);
experiments/minif2f_v4/src/post_hoc_verifier.rs:104:        let progress_verified =
experiments/minif2f_v4/src/post_hoc_verifier.rs:105:            compute_progress_verified(runtime_accepted, post_hoc_verified);
experiments/minif2f_v4/src/post_hoc_verifier.rs:107:        let pput_runtime = compute_pput(progress_runtime, c_i, t_i_ms);
experiments/minif2f_v4/src/post_hoc_verifier.rs:108:        let pput_verified = compute_pput(progress_verified, c_i, t_i_ms);
experiments/minif2f_v4/src/post_hoc_verifier.rs:109:        let pput_m_verified = compute_pput_m(progress_verified, c_i, t_i_ms);
experiments/minif2f_v4/src/post_hoc_verifier.rs:111:        assert_eq!(progress_runtime, 1u8,
experiments/minif2f_v4/src/post_hoc_verifier.rs:112:            "runtime gate fired → progress_runtime = 1");
experiments/minif2f_v4/src/post_hoc_verifier.rs:113:        assert_eq!(progress_verified, 0u8,
experiments/minif2f_v4/src/post_hoc_verifier.rs:114:            "Lean rejected → progress_verified MUST be 0 (North Star truth)");
experiments/minif2f_v4/src/post_hoc_verifier.rs:117:        assert_eq!(pput_verified, 0.0,
experiments/minif2f_v4/src/post_hoc_verifier.rs:118:            "pput_verified MUST be 0 when Lean rejects — North Star Goodhart shield");
experiments/minif2f_v4/src/post_hoc_verifier.rs:120:            "pput_m_verified must collapse with pput_verified");
experiments/minif2f_v4/src/post_hoc_verifier.rs:122:        // Sanity: divergence is detectable. pput_runtime - pput_verified > 0
experiments/minif2f_v4/src/post_hoc_verifier.rs:124:        assert!(pput_runtime - pput_verified > 0.0,
experiments/minif2f_v4/src/post_hoc_verifier.rs:125:            "(pput_runtime - pput_verified) > 0 ⟺ Soft Law divergence detected");
experiments/minif2f_v4/src/post_hoc_verifier.rs:129:    fn test_pput_verified_matches_runtime_when_both_accept() {
experiments/minif2f_v4/src/post_hoc_verifier.rs:133:        // booleans and made pput_verified always 0).
experiments/minif2f_v4/src/post_hoc_verifier.rs:137:        let progress_runtime = compute_progress_runtime(true);
experiments/minif2f_v4/src/post_hoc_verifier.rs:138:        let progress_verified = compute_progress_verified(true, true);
experiments/minif2f_v4/src/post_hoc_verifier.rs:140:        assert_eq!(progress_runtime, progress_verified,
experiments/minif2f_v4/src/post_hoc_verifier.rs:143:            compute_pput(progress_runtime, c_i, t_i_ms),
experiments/minif2f_v4/src/post_hoc_verifier.rs:144:            compute_pput(progress_verified, c_i, t_i_ms),
experiments/minif2f_v4/src/post_hoc_verifier.rs:154:        let progress_runtime = compute_progress_runtime(false);
experiments/minif2f_v4/src/post_hoc_verifier.rs:155:        let progress_verified = compute_progress_verified(false, false);
experiments/minif2f_v4/src/post_hoc_verifier.rs:157:        assert_eq!(compute_pput(progress_runtime, c_i, t_i_ms), 0.0);
experiments/minif2f_v4/src/post_hoc_verifier.rs:158:        assert_eq!(compute_pput(progress_verified, c_i, t_i_ms), 0.0);
experiments/minif2f_v4/src/post_hoc_verifier.rs:162:    fn test_post_hoc_verified_without_runtime_still_zero_progress() {
experiments/minif2f_v4/src/post_hoc_verifier.rs:164:        // fired is a wiring bug, not an honest progress signal. Progress
experiments/minif2f_v4/src/post_hoc_verifier.rs:167:        assert_eq!(compute_progress_verified(false, true), 0u8,
experiments/minif2f_v4/src/bin/evaluator.rs:14:    compute_progress_runtime, compute_progress_verified, compute_pput, compute_pput_m,
experiments/minif2f_v4/src/bin/evaluator.rs:41:struct PputResult {
experiments/minif2f_v4/src/bin/evaluator.rs:83:    total_run_token_count: Option<u64>,
experiments/minif2f_v4/src/bin/evaluator.rs:85:    failed_branch_count: Option<u32>,
experiments/minif2f_v4/src/bin/evaluator.rs:91:    total_wall_time_ms: Option<u64>,
experiments/minif2f_v4/src/bin/evaluator.rs:93:    // verified. Field names align with B1 RunAggregate v2 schema. In Phase B,
experiments/minif2f_v4/src/bin/evaluator.rs:95:    // call site that makes pput_runtime - pput_verified > 0 the H1 signal.
experiments/minif2f_v4/src/bin/evaluator.rs:101:    pput_verified: Option<f64>,
experiments/minif2f_v4/src/bin/evaluator.rs:109:    // Step-B v3 treatment binary: stamp classifier version in every emitted PputResult.
experiments/minif2f_v4/src/bin/evaluator.rs:159:                PputResult { condition: "hybrid_v1".into(), ..r }
experiments/minif2f_v4/src/bin/evaluator.rs:164:                PputResult {
experiments/minif2f_v4/src/bin/evaluator.rs:204:) -> PputResult {
experiments/minif2f_v4/src/bin/evaluator.rs:244:                                 Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:245:                                 Some(acc.failed_branch_count),
experiments/minif2f_v4/src/bin/evaluator.rs:267:                              Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:268:                              Some(acc.failed_branch_count),
experiments/minif2f_v4/src/bin/evaluator.rs:274:                              Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:275:                              Some(acc.failed_branch_count),
experiments/minif2f_v4/src/bin/evaluator.rs:282:                              Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:283:                              Some(acc.failed_branch_count),
experiments/minif2f_v4/src/bin/evaluator.rs:303:) -> PputResult {
experiments/minif2f_v4/src/bin/evaluator.rs:745:                                                        Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:746:                                                        Some(acc.failed_branch_count),
experiments/minif2f_v4/src/bin/evaluator.rs:926:                                                        Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:927:                                                        Some(acc.failed_branch_count),
experiments/minif2f_v4/src/bin/evaluator.rs:999:              Some(acc.total_run_token_count()),
experiments/minif2f_v4/src/bin/evaluator.rs:1000:              Some(acc.failed_branch_count),
experiments/minif2f_v4/src/bin/evaluator.rs:1013:    total_run_token_count: Option<u64>,
experiments/minif2f_v4/src/bin/evaluator.rs:1014:    failed_branch_count: Option<u32>,
experiments/minif2f_v4/src/bin/evaluator.rs:1015:    total_wall_time_ms: Option<u64>,
experiments/minif2f_v4/src/bin/evaluator.rs:1016:) -> PputResult {
experiments/minif2f_v4/src/bin/evaluator.rs:1029:    let (verified, pput_runtime, pput_verified, pput_m_verified) =
experiments/minif2f_v4/src/bin/evaluator.rs:1030:        match (total_run_token_count, total_wall_time_ms) {
experiments/minif2f_v4/src/bin/evaluator.rs:1034:                let progress_runtime = compute_progress_runtime(runtime_accepted);
experiments/minif2f_v4/src/bin/evaluator.rs:1035:                let progress_verified =
experiments/minif2f_v4/src/bin/evaluator.rs:1036:                    compute_progress_verified(runtime_accepted, post_hoc_verified);
experiments/minif2f_v4/src/bin/evaluator.rs:1039:                    Some(compute_pput(progress_runtime, c_i, t_i)),
experiments/minif2f_v4/src/bin/evaluator.rs:1040:                    Some(compute_pput(progress_verified, c_i, t_i)),
experiments/minif2f_v4/src/bin/evaluator.rs:1041:                    Some(compute_pput_m(progress_verified, c_i, t_i)),
experiments/minif2f_v4/src/bin/evaluator.rs:1050:    PputResult {
experiments/minif2f_v4/src/bin/evaluator.rs:1068:        total_run_token_count,
experiments/minif2f_v4/src/bin/evaluator.rs:1069:        failed_branch_count,
experiments/minif2f_v4/src/bin/evaluator.rs:1070:        total_wall_time_ms,
experiments/minif2f_v4/src/bin/evaluator.rs:1073:        pput_verified,
experiments/minif2f_v4/src/bin/evaluator.rs:1082:/// Returns the relative path (for embedding in PputResult) or None on I/O failure.
experiments/minif2f_v4/src/jsonl_schema.rs:7:// Versioning: every v2 record carries `schema_version = "v2.0"`. Legacy Paper-1
experiments/minif2f_v4/src/jsonl_schema.rs:8:// era jsonl rows (the `PputResult` shape emitted by evaluator before this commit)
experiments/minif2f_v4/src/jsonl_schema.rs:9:// have NO `schema_version` field, so `RunRecord::from_json` discriminates on
experiments/minif2f_v4/src/jsonl_schema.rs:10:// presence and routes to `LegacyRunAggregate`. No on-disk artifact is rewritten
experiments/minif2f_v4/src/jsonl_schema.rs:13:// B1 scope: schema definition + round-trip + legacy-compat + zero-progress
experiments/minif2f_v4/src/jsonl_schema.rs:36:    pub schema_version: String,
experiments/minif2f_v4/src/jsonl_schema.rs:80:/// `pput_verified` = Lean post-hoc verified — H-VPPUT input.
experiments/minif2f_v4/src/jsonl_schema.rs:82:pub struct RunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:87:    pub schema_version: String,
experiments/minif2f_v4/src/jsonl_schema.rs:93:    pub total_run_token_count: u64,
experiments/minif2f_v4/src/jsonl_schema.rs:95:    pub total_wall_time_ms: u64,
experiments/minif2f_v4/src/jsonl_schema.rs:97:    pub progress: u8,
experiments/minif2f_v4/src/jsonl_schema.rs:101:    pub pput_verified: f64,
experiments/minif2f_v4/src/jsonl_schema.rs:102:    /// 10^6 × pput_verified — display unit (PREREG § 5).
experiments/minif2f_v4/src/jsonl_schema.rs:104:    pub failed_branch_count: u32,
experiments/minif2f_v4/src/jsonl_schema.rs:120:impl RunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:121:    /// Compute pput_verified per PREREG § 5:
experiments/minif2f_v4/src/jsonl_schema.rs:122:    ///   pput_verified = progress / (c_i * t_i_ms / 1000)
experiments/minif2f_v4/src/jsonl_schema.rs:123:    /// Returns 0.0 when progress is 0, OR when c_i or t_i_ms is 0
experiments/minif2f_v4/src/jsonl_schema.rs:125:    pub fn compute_pput_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
experiments/minif2f_v4/src/jsonl_schema.rs:126:        if progress == 0 || c_i == 0 || t_i_ms == 0 {
experiments/minif2f_v4/src/jsonl_schema.rs:130:        (progress as f64) / denom
experiments/minif2f_v4/src/jsonl_schema.rs:133:    /// Display unit: 10^6 × pput_verified.
experiments/minif2f_v4/src/jsonl_schema.rs:134:    pub fn compute_pput_m_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
experiments/minif2f_v4/src/jsonl_schema.rs:135:        1.0e6 * Self::compute_pput_verified(progress, c_i, t_i_ms)
experiments/minif2f_v4/src/jsonl_schema.rs:139:/// Legacy v1 run row — mirrors the pre-v2 `PputResult` shape emitted by the
experiments/minif2f_v4/src/jsonl_schema.rs:147:pub struct LegacyRunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:166:    V2(RunAggregate),
experiments/minif2f_v4/src/jsonl_schema.rs:167:    Legacy(LegacyRunAggregate),
experiments/minif2f_v4/src/jsonl_schema.rs:171:    /// Parse one jsonl line. v2 if `schema_version` present, else legacy.
experiments/minif2f_v4/src/jsonl_schema.rs:175:        let is_v2 = v.get("schema_version")
experiments/minif2f_v4/src/jsonl_schema.rs:191:    fn sample_run() -> RunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:192:        RunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:196:            schema_version: SCHEMA_VERSION_V2.into(),
experiments/minif2f_v4/src/jsonl_schema.rs:200:            total_run_token_count: 4096,
experiments/minif2f_v4/src/jsonl_schema.rs:201:            total_wall_time_ms: 12_000,
experiments/minif2f_v4/src/jsonl_schema.rs:202:            progress: 1,
experiments/minif2f_v4/src/jsonl_schema.rs:204:            pput_verified: RunAggregate::compute_pput_verified(1, 4096, 12_000),
experiments/minif2f_v4/src/jsonl_schema.rs:205:            pput_m_verified: RunAggregate::compute_pput_m_verified(1, 4096, 12_000),
experiments/minif2f_v4/src/jsonl_schema.rs:206:            failed_branch_count: 3,
experiments/minif2f_v4/src/jsonl_schema.rs:220:        let parsed: RunAggregate = serde_json::from_str(&line).expect("deserialize");
experiments/minif2f_v4/src/jsonl_schema.rs:221:        assert_eq!(parsed, original, "v2 RunAggregate must round-trip");
experiments/minif2f_v4/src/jsonl_schema.rs:222:        assert!(line.contains("\"schema_version\":\"v2.0\""),
experiments/minif2f_v4/src/jsonl_schema.rs:223:                "serialized line must stamp schema_version");
experiments/minif2f_v4/src/jsonl_schema.rs:227:    fn test_pput_verified_zero_when_progress_zero() {
experiments/minif2f_v4/src/jsonl_schema.rs:229:        // pput_verified = 0 regardless of cost / wall-time.
experiments/minif2f_v4/src/jsonl_schema.rs:230:        assert_eq!(RunAggregate::compute_pput_verified(0, 1000, 5000), 0.0);
experiments/minif2f_v4/src/jsonl_schema.rs:231:        assert_eq!(RunAggregate::compute_pput_m_verified(0, 1000, 5000), 0.0);
experiments/minif2f_v4/src/jsonl_schema.rs:237:        r.progress = 0;
experiments/minif2f_v4/src/jsonl_schema.rs:238:        r.pput_verified = RunAggregate::compute_pput_verified(0, r.total_run_token_count, r.total_wall_time_ms);
experiments/minif2f_v4/src/jsonl_schema.rs:239:        r.pput_m_verified = RunAggregate::compute_pput_m_verified(0, r.total_run_token_count, r.total_wall_time_ms);
experiments/minif2f_v4/src/jsonl_schema.rs:240:        assert_eq!(r.pput_verified, 0.0);
experiments/minif2f_v4/src/jsonl_schema.rs:244:        assert_eq!(RunAggregate::compute_pput_verified(1, 0, 5000), 0.0);
experiments/minif2f_v4/src/jsonl_schema.rs:245:        assert_eq!(RunAggregate::compute_pput_verified(1, 1000, 0), 0.0);
experiments/minif2f_v4/src/jsonl_schema.rs:254:        match RunRecord::from_json(legacy_line).expect("legacy line parses") {
experiments/minif2f_v4/src/jsonl_schema.rs:269:        match RunRecord::from_json(&v2_line).expect("v2 line parses") {
experiments/minif2f_v4/analysis/PHASE7_DAG_ANALYSIS.md:82:**Architectural significance**: mathd_algebra_332 represents **deadlock-breaking via depth**. In prior monolithic modes, the system either one-shot the full proof (didn't work) or rejected entirely. Per-tactic mode allows the system to park partial progress and iterate on the hard step.
experiments/minif2f_v4/analysis/PHASE7_DAG_ANALYSIS.md:140:1. Bounties only fire on unsolved problems (Art. II.3 bounty == 1/unsolved count). All 11 failed problems timeout without generating meaningful partial progress.
experiments/minif2f_v4/analysis/PHASE7_DAG_ANALYSIS.md:273:2. **LLM sampling**: On hard problems without golden paths, the LLM generates incorrect tactics. Each rejection adds latency without progress.
experiments/minif2f_v4/monitor.sh:52:# 3. Results progress
handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:25:  - `test_pput_verified_zero_when_lean_rejects` (B4)
handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:33:PREREG § 3 #8 mandates `total_run_token_count` includes ALL proposals (winning + failed). The B2 wiring records:
handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:58:PREREG § 3 #7 / round-4 fix R4-2: under Soft Law, `pput_runtime - pput_verified > 0` is the H1 signal. The B4 implementation:
handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:59:- `compute_progress_runtime(runtime_accepted) = if runtime_accepted {1} else {0}`
handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:60:- `compute_progress_verified(runtime_accepted, post_hoc_verified) = if runtime_accepted && post_hoc_verified {1} else {0}`
handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:64:- (Q3.a) `compute_progress_verified` uses AND of (runtime, verified). Is this right? PREREG says progress = Lean ground truth. If runtime fails but verified somehow passes (impossible today, but B4 is the contract), should progress=1 or 0?
handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:67:- (Q3.d) `pput_m_verified = 1e6 × pput_verified` — display unit. Is the multiplier precision-safe (no float drift on common C_i × T_i values)?
handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:70:B1 introduced schema_version dispatch (`RunRecord::from_json`). B2-B4 ADD optional fields to `PputResult`. Legacy Paper-1 jsonl rows lack these fields.
handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:74:- (Q4.b) The 3 new fields total_run_token_count / total_wall_time_ms / pput_verified are now sometimes present, sometimes None. Downstream tooling needs to handle BOTH. Is the field set on PputResult consistent with B1's RunAggregate v2 schema? Where do they MISS-align?
handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:78:B1 frozen the v2 RunAggregate schema. B2/B3/B4 add fields to LEGACY PputResult, NOT directly to RunAggregate. The plan says B2-B4 wire emit; B4 implies switching to RunAggregate but the implementation kept PputResult.
handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:81:- (Q5.a) Is keeping PputResult (legacy) instead of switching to RunAggregate (v2) defensible at the B4 boundary? When does the switch happen?
handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:82:- (Q5.b) Field name alignment: `pput_runtime`, `pput_verified`, `pput_m_verified`, `verified` match RunAggregate. But `total_run_token_count` matches RunAggregate; `failed_branch_count` matches RunAggregate; `total_wall_time_ms` matches RunAggregate. Anything mis-spelled?
handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:83:- (Q5.c) RunAggregate has `progress: u8` (Lean ground truth). PputResult does NOT have `progress` — only `verified: Option<bool>`. Is this gap intentional (B5 will fill it) or a bug?
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:20:1.  **Architectural Hazard for Phase C (Q7, Q3.b)**: The B4 implementation is not Phase-C-safe by construction. The core logic for determining `post_hoc_verified` is hard-coded inside the `make_pput` function (`evaluator.rs:1026`) as `let post_hoc_verified = has_gp;`. A future implementer of Soft Law could easily (and incorrectly) provide a fake `has_gp=true` signal and neglect to insert the mandatory, expensive `verify_post_hoc` call, thus corrupting the North Star `pput_verified` metric. The current design relies on future discipline rather than being safe by construction.
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:21:2.  **Schema Drift from B1 Contract (Q5.c)**: The B1 `RunAggregate` v2 schema mandates a `progress: u8` field. The B4 implementation in `PputResult` emits `verified: Option<bool>` (`evaluator.rs:90`). While semantically similar, this is a direct violation of the frozen schema's type and name contract. Downstream tooling built against the B1 spec will break or require shims.
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:35:*   **(Q1.d) `flip_last_failed_to_accepted` underflow**: **PASS**. The implementation at `cost_aggregator.rs:100-102` is `if self.failed_branch_count > 0 { self.failed_branch_count -= 1; }`. This is a saturating subtraction that cannot underflow or corrupt the count. It is robust against wiring bugs.
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:55:*   **(Q3.a) `compute_progress_verified` AND logic**: **PASS**. The logic `if runtime_accepted && post_hoc_verified` (`post_hoc_verifier.rs:60`) is correct. Per PREREG, progress requires ground-truth acceptance. A verified proof that was not accepted by the runtime is a system anomaly, not progress, and should correctly result in `progress=0`.
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:64:*   **(Q4.a) Round-trip preserved**: **PASS**. All new fields in `PputResult` are `Option<T>` and use `#[serde(skip_serializing_if = "Option::is_none")]` (`evaluator.rs:82-93`). This is the canonical and correct way to ensure that deserializing old data and re-serializing it produces identical output.
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:65:*   **(Q4.b) Downstream tooling**: **PASS**. The implementation correctly anticipates that downstream tools must handle `Option` types. The field names align with the B1 `RunAggregate` schema (see Q5.b), which is the correct contract.
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:72:*   **(Q5.a) `PputResult` vs `RunAggregate`**: **CHALLENGE (Minor)**. The plan to switch to `RunAggregate` was deferred. Instead, fields were added to the legacy `PputResult`. This is defensible as an incremental step, but it creates technical debt and naming confusion. A plan should be made to unify these types.
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:73:*   **(Q5.b) Field name alignment**: **PASS**. A manual check of the new fields in `PputResult` (`evaluator.rs:82-93`) against the `RunAggregate` v2 schema shows they are correctly named and spelled.
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:74:*   **(Q5.c) `progress: u8` vs `verified: Option<bool>`**: **CHALLENGE (Major Bug)**. This is a clear deviation. The B1 `RunAggregate` schema specifies `progress: u8`. The B4 implementation adds `verified: Option<bool>` (`evaluator.rs:90`). The emitted JSONL will have a field named `verified` with a boolean value, not a field named `progress` with a numeric value. This breaks the B1 contract and will fail validation against a strict schema-checker.
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:82:*   **(Q6.c) Trust Root**: **PASS (Finding for B7)**. The PREREG § 1.8 defines the Trust Root. The logic for cost, time, and verified progress accounting is fundamental to the integrity of the PPUT metric. Therefore, `cost_aggregator.rs`, `wall_clock.rs`, and `post_hoc_verifier.rs` **must** be added to the Trust Root manifest. This is a mandatory action item for the B7 implementation.
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:101:2.  **Immediate B4 Fix (P0)**: Align `PputResult` with the `RunAggregate` schema.
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:102:    *   Rename the `verified` field to `progress_verified`.
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:104:    *   Update the `make_pput` logic (`evaluator.rs:1030-1031`) to store the `u8` progress value, not a boolean.
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:106:3.  **B5 Design Debt**: Schedule the removal of the redundant `PputResult` struct. The `evaluator` should construct and emit the canonical `jsonl_schema::RunAggregate` struct directly.
handover/audits/CODEX_PAPER1_V2_1_AUDIT_2026-04-25.md:18:v2.1 fixes the round-2 statistical blocker that mattered most for the headline: the hard-9 restatement is present, and the raw aggregate still supports `B > A` with `b=8`, `c=0`, one-sided `p=0.003906` [paper:203-230](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:203), [results JSON:357-369](/home/zephryj/projects/turingosv4/handover/preregistration/E1v2_RESULTS_2026-04-24.json:357). The family is now explicitly declared in §3.6, which is real progress [paper:107-123](/home/zephryj/projects/turingosv4/handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md:107).
handover/audits/CONSTITUTIONAL_BLINDSPOT_AUDIT_2026-04-22.md:22:| B-11 | Art. III.2 按需加载 metric | 无 "每 agent 查阅的 doc 数量 / 不同 agent 查阅分布" 监控 | 完全缺失 | Medium | `search_cache/search_count` 存在于 `evaluator.rs:371,376` 但从不进 PputResult；无法回答"agent 真的在按需加载 vs. 不在加载" | C-058 |
handover/audits/CONSTITUTIONAL_BLINDSPOT_AUDIT_2026-04-22.md:40:Ruling: "被他人调用次数"是 Art. I.2 明文要求的三大统计信号之一，必须作为 per-agent 累积标量暴露给 top management + agent snapshot。实现：`Tape` 级增加 `reputation_by_author: HashMap<String, u32>`，每次 `append` 把所有 `citations` 的 author 的 count +1；进入 `UniverseSnapshot`；PputResult 增加 `reputation_at_end` 字段。
handover/audits/CONSTITUTIONAL_BLINDSPOT_AUDIT_2026-04-22.md:51:**C-056**: Art. II.2 价格时间序列 + 驱动行为 metric — per-market ring buffer + PputResult 增加 `price_tracking_correlation`
handover/audits/CONSTITUTIONAL_BLINDSPOT_AUDIT_2026-04-22.md:60:**C-057**: Art. II.2.1 探索熵 metric — TEMP_LADDER 升级为 `src/sdk/exploration.rs`；`parent_selection_entropy` 进 PputResult
handover/audits/CONSTITUTIONAL_BLINDSPOT_AUDIT_2026-04-22.md:61:**C-058**: Art. III.1+III.2 学习文档 GC + 按需加载审计 — Librarian learned.md 加 TTL；`search_count` 进 PputResult
handover/audits/SYNTHESIS_2026-04-22.md:54:- **证据**：`evaluator.rs:108-143,196-215` 的 oneshot 条件直接 `oracle.verify_omega` 拿结果就返回 `PputResult`，**不经过任何 bus write 或 settlement**
handover/audits/GEMINI_PPUT_CCL_AUDIT_2026-04-26.md:14:**PASS**. The definitions for V-PPUT, C_i, T_i, ArtifactState, and the Trust Root are exceptionally precise and leave little room for post-hoc reinterpretation. The commitment to count all tokens from all branches (§ 1.2) and the wall-clock definition of T_i (§ 1.3) are strong. The 4-state machine for artifacts (§ 1.7) is unambiguous and correctly separates artifact confidence from the binary ground truth of task progress.
handover/audits/GEMINI_PPUT_CCL_AUDIT_2026-04-26.md:31:- **Impact**: This sets an almost unattainable bar for capability compilation within a 30-day arc. While intellectually honest, it is poorly calibrated for empirical reality. A system should be able to accept trade-offs, and a single-point failure condition is not a robust measure of progress.
handover/audits/GEMINI_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:51:-   **Are 4 revision rounds + 1 clean rewrite a sign of structural instability or normal pre-registration tightening?**: This is a sign of **normal and highly effective pre-registration tightening**. The core research thesis and experimental arc have remained stable. The revisions have progressively hardened the *protocol* against valid adversarial critiques, which is the exact purpose of a multi-round, dual-auditor process. The final document is significantly more rigorous than the initial draft, demonstrating the value of the process, not a flaw in the underlying research.
handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:29:  - `test_pput_verified_zero_when_lean_rejects` (B4)
handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:37:PREREG § 3 #8 mandates `total_run_token_count` includes ALL proposals (winning + failed). The B2 wiring records:
handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:62:PREREG § 3 #7 / round-4 fix R4-2: under Soft Law, `pput_runtime - pput_verified > 0` is the H1 signal. The B4 implementation:
handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:63:- `compute_progress_runtime(runtime_accepted) = if runtime_accepted {1} else {0}`
handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:64:- `compute_progress_verified(runtime_accepted, post_hoc_verified) = if runtime_accepted && post_hoc_verified {1} else {0}`
handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:68:- (Q3.a) `compute_progress_verified` uses AND of (runtime, verified). Is this right? PREREG says progress = Lean ground truth. If runtime fails but verified somehow passes (impossible today, but B4 is the contract), should progress=1 or 0?
handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:71:- (Q3.d) `pput_m_verified = 1e6 × pput_verified` — display unit. Is the multiplier precision-safe (no float drift on common C_i × T_i values)?
handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:74:B1 introduced schema_version dispatch (`RunRecord::from_json`). B2-B4 ADD optional fields to `PputResult`. Legacy Paper-1 jsonl rows lack these fields.
handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:78:- (Q4.b) The 3 new fields total_run_token_count / total_wall_time_ms / pput_verified are now sometimes present, sometimes None. Downstream tooling needs to handle BOTH. Is the field set on PputResult consistent with B1's RunAggregate v2 schema? Where do they MISS-align?
handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:82:B1 froze the v2 RunAggregate schema. B2/B3/B4 add fields to LEGACY PputResult, NOT directly to RunAggregate. The plan says B2-B4 wire emit; B4 implies switching to RunAggregate but the implementation kept PputResult.
handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:85:- (Q5.a) Is keeping PputResult (legacy) instead of switching to RunAggregate (v2) defensible at the B4 boundary? When does the switch happen?
handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:86:- (Q5.b) Field name alignment: `pput_runtime`, `pput_verified`, `pput_m_verified`, `verified` match RunAggregate. But `total_run_token_count` matches RunAggregate; `failed_branch_count` matches RunAggregate; `total_wall_time_ms` matches RunAggregate. Anything mis-spelled?
handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:87:- (Q5.c) RunAggregate has `progress: u8` (Lean ground truth). PputResult does NOT have `progress` — only `verified: Option<bool>`. Is this gap intentional (B5 will fill it) or a bug?
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:89:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:91:ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:342:| H1 | Soft Law mode has lower `pput_verified` than Full mode on adaptation hard-10, even when `pput_runtime` is comparable | C | McNemar one-sided on per-task verified-progress + paired Δ on PPUT-M | Soft Law < Full |
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:363:- B4 `pput_verified` vs `pput_runtime` dual-field separation
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:414:- F1: Phase C — Soft Law mode has VPPUT comparable to Full (within Bonferroni-corrected CI). Implies PPUT does not penalize fake-progress; metric is broken or mode does not actually violate constitution.
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:552:> TuringOS 的长期目标不是单纯提高 solve rate，而是在宪法约束下最大化 Verified PPUT：单位 token、单位时间内产生的可验证 golden-path progress。
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:574:| Art. I.1 | 只有谓词通过才有 progress |
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:609:| 安全 | FAR | 防止错误状态被当成 progress |
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:666:golden_path_token_count, total_run_token_count, total_wall_time_ms,
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:667:progress (0|1), pput, failed_branch_count, rollback_count,
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:706:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:708:ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:903:  "schema_version": "1.0",
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1204:1. **Phase A — Pre-flight** (days 1-3, 2026-04-26 → 2026-04-28, **in progress**)
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1214:   - `pput_verified` vs `pput_runtime` dual-field separation
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1259:- Three constitutional ablations restated in PPUT terms: Soft Law (post-hoc Lean reject → progress=0), Panopticon (CPR↑+IAC↑→PPUT↓), Amnesia (ERR↓→PPUT↓).
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1644:  but unreachable from swarm loop. Constitutional Art. III.2 (progressive disclosure)
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1887:1. Art. II.1 broadcast — **Step-B v3.3 in progress** (treatment arm running)
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1895:Priority: 1 (in progress) → 2 (highest N_c impact) → 3 (highest diversity impact) → 5 → 4 → 6
handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_ROUND4_2026-04-26.md:28:> "Four revision rounds + clean rewrite is normal pre-registration tightening, not structural instability. Core thesis remained stable; protocol progressively hardened against valid critiques."
handover/audits/EXT_CODEX_2026-04-22.md:111:- **Oneshot and hybrid still bypass Art. IV/C-043 runtime topology.** The `oneshot` condition verifies directly with `oracle.verify_omega` and returns a `PputResult` without any bus write or settlement (`/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:108-143,196-215`). That contradicts Art. IV's `∏p=1 -> wtool -> Q_{t+1}` rule (`/home/zephryj/projects/turingosv4/constitution.md:521-524`) and C-043's mandatory `append_oracle_accepted` requirement (`/home/zephryj/projects/turingosv4/cases/C-043_mandatory_wtool_on_omega.yaml:15-20,26-34`). Claude's internal audit treated topology as effectively landed, but the legacy oneshot path remains constitutionally divergent.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:80:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:82:ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:465:**Descriptive secondary endpoints** (logged, NOT in inferential family, NO α correction): mode-level CPR, mode-level IAC, mode-level FAR, gap between pput_runtime and pput_verified for Soft Law, ERR for Amnesia, mean prompt length per mode.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:567:- B4 `pput_verified` vs `pput_runtime` dual-field separation
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:640:- F1: Phase C — Soft Law mode has VPPUT comparable to Full (within Bonferroni-corrected CI). Implies PPUT does not penalize fake-progress; metric is broken or mode does not actually violate constitution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:946:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:948:ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1293:| H1 | Soft Law violates ground-truth honesty — its `pput_verified` is lower than Full despite comparable `pput_runtime` | C | McNemar one-sided on per-(problem, seed) discordant pairs of `Progress_i` (Lean-verified golden path) | (problem, seed) | Full > Soft Law | gap between `pput_runtime` and `pput_verified` for Soft Law (gameability indicator); FAR per mode |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1375:- B4 `pput_verified` vs `pput_runtime` dual-field separation
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1448:- F1: Phase C — Soft Law mode has VPPUT comparable to Full (within Bonferroni-corrected CI). Implies PPUT does not penalize fake-progress; metric is broken or mode does not actually violate constitution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:1657:  "schema_version": "1.0",
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2160:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2162:ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2413:| H1 | Soft Law mode has lower `pput_verified` than Full mode on adaptation hard-10, even when `pput_runtime` is comparable | C | McNemar one-sided on per-task verified-progress + paired Δ on PPUT-M | Soft Law < Full |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2434:- B4 `pput_verified` vs `pput_runtime` dual-field separation
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2485:- F1: Phase C — Soft Law mode has VPPUT comparable to Full (within Bonferroni-corrected CI). Implies PPUT does not penalize fake-progress; metric is broken or mode does not actually violate constitution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2623:> TuringOS 的长期目标不是单纯提高 solve rate，而是在宪法约束下最大化 Verified PPUT：单位 token、单位时间内产生的可验证 golden-path progress。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2645:| Art. I.1 | 只有谓词通过才有 progress |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2680:| 安全 | FAR | 防止错误状态被当成 progress |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2737:golden_path_token_count, total_run_token_count, total_wall_time_ms,
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2738:progress (0|1), pput, failed_branch_count, rollback_count,
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2777:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2779:ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:2974:  "schema_version": "1.0",
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3275:1. **Phase A — Pre-flight** (days 1-3, 2026-04-26 → 2026-04-28, **in progress**)
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3285:   - `pput_verified` vs `pput_runtime` dual-field separation
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3330:- Three constitutional ablations restated in PPUT terms: Soft Law (post-hoc Lean reject → progress=0), Panopticon (CPR↑+IAC↑→PPUT↓), Amnesia (ERR↓→PPUT↓).
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3715:  but unreachable from swarm loop. Constitutional Art. III.2 (progressive disclosure)
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3958:1. Art. II.1 broadcast — **Step-B v3.3 in progress** (treatment arm running)
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3966:Priority: 1 (in progress) → 2 (highest N_c impact) → 3 (highest diversity impact) → 5 → 4 → 6
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:4874:- The three revision rounds are a sign of a healthy and effective adversarial audit process, not structural instability; the core research thesis has remained unchanged while the experimental protocol has been progressively hardened against valid critiques.
handover/audits/EXT_CODEX_PHASE_8_BATCH_2026-04-22.md:36:- swarm `complete` / `step` 路径现在接近 Art. IV topology，**但 oneshot 仍写入 ephemeral `TuringBus::new(...)` with `wal: None`**，never records `halt`, drops in-memory state immediately after returning `PputResult`.
handover/audits/EXT_CODEX_PHASE_8_R1ALPHA_2026-04-22.md:49:**Fix plan**: either (a) don't emit `PPUT_RESULT` at all on WAL failure (just stderr + early return), or (b) add `measurement_error: bool` field to PputResult and have make_pput skip emission.
handover/audits/GEMINI_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:60:- The three revision rounds are a sign of a healthy and effective adversarial audit process, not structural instability; the core research thesis has remained unchanged while the experimental protocol has been progressively hardened against valid critiques.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:92:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:94:ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:407:| H1 | Soft Law violates ground-truth honesty — its `pput_verified` is lower than Full despite comparable `pput_runtime` | C | McNemar one-sided on per-(problem, seed) discordant pairs of `Progress_i` (Lean-verified golden path) | (problem, seed) | Full > Soft Law | gap between `pput_runtime` and `pput_verified` for Soft Law (gameability indicator); FAR per mode |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:456:- B4 `pput_verified` vs `pput_runtime` dual-field separation
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:529:- F1: Phase C — Soft Law mode has VPPUT comparable to Full (within Bonferroni-corrected CI). Implies PPUT does not penalize fake-progress; metric is broken or mode does not actually violate constitution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:733:> TuringOS 的长期目标不是单纯提高 solve rate，而是在宪法约束下最大化 Verified PPUT：单位 token、单位时间内产生的可验证 golden-path progress。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:755:| Art. I.1 | 只有谓词通过才有 progress |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:790:| 安全 | FAR | 防止错误状态被当成 progress |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:847:golden_path_token_count, total_run_token_count, total_wall_time_ms,
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:848:progress (0|1), pput, failed_branch_count, rollback_count,
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:887:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:889:ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1084:  "schema_version": "1.0",
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1587:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1589:ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1840:| H1 | Soft Law mode has lower `pput_verified` than Full mode on adaptation hard-10, even when `pput_runtime` is comparable | C | McNemar one-sided on per-task verified-progress + paired Δ on PPUT-M | Soft Law < Full |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1861:- B4 `pput_verified` vs `pput_runtime` dual-field separation
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:1912:- F1: Phase C — Soft Law mode has VPPUT comparable to Full (within Bonferroni-corrected CI). Implies PPUT does not penalize fake-progress; metric is broken or mode does not actually violate constitution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2050:> TuringOS 的长期目标不是单纯提高 solve rate，而是在宪法约束下最大化 Verified PPUT：单位 token、单位时间内产生的可验证 golden-path progress。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2072:| Art. I.1 | 只有谓词通过才有 progress |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2107:| 安全 | FAR | 防止错误状态被当成 progress |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2164:golden_path_token_count, total_run_token_count, total_wall_time_ms,
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2165:progress (0|1), pput, failed_branch_count, rollback_count,
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2204:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2206:ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2401:  "schema_version": "1.0",
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2702:1. **Phase A — Pre-flight** (days 1-3, 2026-04-26 → 2026-04-28, **in progress**)
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2712:   - `pput_verified` vs `pput_runtime` dual-field separation
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:2757:- Three constitutional ablations restated in PPUT terms: Soft Law (post-hoc Lean reject → progress=0), Panopticon (CPR↑+IAC↑→PPUT↓), Amnesia (ERR↓→PPUT↓).
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:3142:  but unreachable from swarm loop. Constitutional Art. III.2 (progressive disclosure)
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:3385:1. Art. II.1 broadcast — **Step-B v3.3 in progress** (treatment arm running)
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:3393:Priority: 1 (in progress) → 2 (highest N_c impact) → 3 (highest diversity impact) → 5 → 4 → 6
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:35:  - `test_pput_verified_zero_when_lean_rejects` (B4)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:43:PREREG § 3 #8 mandates `total_run_token_count` includes ALL proposals (winning + failed). The B2 wiring records:
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:68:PREREG § 3 #7 / round-4 fix R4-2: under Soft Law, `pput_runtime - pput_verified > 0` is the H1 signal. The B4 implementation:
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:69:- `compute_progress_runtime(runtime_accepted) = if runtime_accepted {1} else {0}`
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:70:- `compute_progress_verified(runtime_accepted, post_hoc_verified) = if runtime_accepted && post_hoc_verified {1} else {0}`
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:74:- (Q3.a) `compute_progress_verified` uses AND of (runtime, verified). Is this right? PREREG says progress = Lean ground truth. If runtime fails but verified somehow passes (impossible today, but B4 is the contract), should progress=1 or 0?
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:77:- (Q3.d) `pput_m_verified = 1e6 × pput_verified` — display unit. Is the multiplier precision-safe (no float drift on common C_i × T_i values)?
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:80:B1 introduced schema_version dispatch (`RunRecord::from_json`). B2-B4 ADD optional fields to `PputResult`. Legacy Paper-1 jsonl rows lack these fields.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:84:- (Q4.b) The 3 new fields total_run_token_count / total_wall_time_ms / pput_verified are now sometimes present, sometimes None. Downstream tooling needs to handle BOTH. Is the field set on PputResult consistent with B1's RunAggregate v2 schema? Where do they MISS-align?
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:88:B1 frozen the v2 RunAggregate schema. B2/B3/B4 add fields to LEGACY PputResult, NOT directly to RunAggregate. The plan says B2-B4 wire emit; B4 implies switching to RunAggregate but the implementation kept PputResult.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:91:- (Q5.a) Is keeping PputResult (legacy) instead of switching to RunAggregate (v2) defensible at the B4 boundary? When does the switch happen?
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:92:- (Q5.b) Field name alignment: `pput_runtime`, `pput_verified`, `pput_m_verified`, `verified` match RunAggregate. But `total_run_token_count` matches RunAggregate; `failed_branch_count` matches RunAggregate; `total_wall_time_ms` matches RunAggregate. Anything mis-spelled?
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:93:- (Q5.c) RunAggregate has `progress: u8` (Lean ground truth). PputResult does NOT have `progress` — only `verified: Option<bool>`. Is this gap intentional (B5 will fill it) or a bug?
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:155:- `cargo test test_failed_branches_counted_in_total_cost` (anti-Goodhart conformance per PREREG § 3): synthesize a run with 5 failed proposals + 1 success → assert `total_run_token_count == sum(all 6 proposals)`.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:182:### B4 — `pput_verified` vs `pput_runtime` 双字段
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:186:- `pput_verified` = Progress_verified / (C_i × T_i)  [where Progress_verified = 1 if Lean post-hoc verifies the golden path]
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:188:For Soft Law mode (which fakes runtime acceptance), `pput_runtime` may inflate but `pput_verified` should drop to 0. This is the H1 detection mechanism.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:195:- `cargo test test_pput_verified_zero_when_lean_rejects` (PREREG § 3 `test_golden_path_requires_ground_truth_acceptance`): synthesize a run that records runtime accept + Lean reject → assert `progress = 0`, `pput_verified = 0.0`.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:196:- For Phase C ablation Soft Law mode: confirm `pput_runtime > 0` but `pput_verified` reflects Lean truth.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:236:**Descriptive secondary endpoints** (logged, NOT in inferential family, NO α correction): mode-level CPR, mode-level IAC, mode-level FAR, gap between pput_runtime and pput_verified for Soft Law, ERR for Amnesia, mean prompt length per mode.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:341://   total_run_token_count = Σ over EVERY proposal in the run of
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:369:    pub failed_branch_count: u32,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:397:    /// pass `false` so they accrete failed_branch_count.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:401:            self.failed_branch_count += 1;
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:412:        if self.failed_branch_count > 0 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:413:            self.failed_branch_count -= 1;
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:418:    pub fn total_run_token_count(&self) -> u64 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:460:        assert_eq!(acc.total_run_token_count(), expected_total as u64,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:464:        assert_eq!(acc.failed_branch_count, 5);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:470:        assert_eq!(acc.total_run_token_count(), 0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:472:        assert_eq!(acc.failed_branch_count, 0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:686://   pput_verified = Progress_verified / (C_i × T_i / 1000)   ← North Star
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:698://     pput_verified MUST be 0  (Lean reject means Progress_verified = 0)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:701://   pput_verified is the only North Star metric for H-VPPUT. pput_runtime
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:705:use crate::jsonl_schema::RunAggregate;
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:717:/// Use `compute_progress_verified` instead when the runtime gate already
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:734:pub fn compute_progress_verified(runtime_accepted: bool, post_hoc_verified: bool) -> u8 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:740:pub fn compute_progress_runtime(runtime_accepted: bool) -> u8 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:744:/// Wrap RunAggregate::compute_pput_verified for callers in evaluator that
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:745:/// only have (progress, c_i, t_i_ms). Same math, single source of truth.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:746:pub fn compute_pput(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:747:    RunAggregate::compute_pput_verified(progress, c_i, t_i_ms)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:751:pub fn compute_pput_m(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:752:    RunAggregate::compute_pput_m_verified(progress, c_i, t_i_ms)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:762:    ///  assert progress = 0, pput_verified = 0.0"
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:769:    fn test_pput_verified_zero_when_lean_rejects() {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:772:        // run's progress to 0.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:778:        let progress_runtime = compute_progress_runtime(runtime_accepted);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:779:        let progress_verified =
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:780:            compute_progress_verified(runtime_accepted, post_hoc_verified);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:782:        let pput_runtime = compute_pput(progress_runtime, c_i, t_i_ms);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:783:        let pput_verified = compute_pput(progress_verified, c_i, t_i_ms);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:784:        let pput_m_verified = compute_pput_m(progress_verified, c_i, t_i_ms);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:786:        assert_eq!(progress_runtime, 1u8,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:787:            "runtime gate fired → progress_runtime = 1");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:788:        assert_eq!(progress_verified, 0u8,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:789:            "Lean rejected → progress_verified MUST be 0 (North Star truth)");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:792:        assert_eq!(pput_verified, 0.0,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:793:            "pput_verified MUST be 0 when Lean rejects — North Star Goodhart shield");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:795:            "pput_m_verified must collapse with pput_verified");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:797:        // Sanity: divergence is detectable. pput_runtime - pput_verified > 0
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:799:        assert!(pput_runtime - pput_verified > 0.0,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:800:            "(pput_runtime - pput_verified) > 0 ⟺ Soft Law divergence detected");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:804:    fn test_pput_verified_matches_runtime_when_both_accept() {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:808:        // booleans and made pput_verified always 0).
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:812:        let progress_runtime = compute_progress_runtime(true);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:813:        let progress_verified = compute_progress_verified(true, true);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:815:        assert_eq!(progress_runtime, progress_verified,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:818:            compute_pput(progress_runtime, c_i, t_i_ms),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:819:            compute_pput(progress_verified, c_i, t_i_ms),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:829:        let progress_runtime = compute_progress_runtime(false);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:830:        let progress_verified = compute_progress_verified(false, false);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:832:        assert_eq!(compute_pput(progress_runtime, c_i, t_i_ms), 0.0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:833:        assert_eq!(compute_pput(progress_verified, c_i, t_i_ms), 0.0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:837:    fn test_post_hoc_verified_without_runtime_still_zero_progress() {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:839:        // fired is a wiring bug, not an honest progress signal. Progress
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:842:        assert_eq!(compute_progress_verified(false, true), 0u8,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:865:+    compute_progress_runtime, compute_progress_verified, compute_pput, compute_pput_m,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:870:@@ -71,6 +76,31 @@ struct PputResult {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:878:+    total_run_token_count: Option<u64>,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:880:+    failed_branch_count: Option<u32>,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:886:+    total_wall_time_ms: Option<u64>,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:888:+    // verified. Field names align with B1 RunAggregate v2 schema. In Phase B,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:890:+    // call site that makes pput_runtime - pput_verified > 0 the H1 signal.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:896:+    pput_verified: Option<f64>,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:904: ) -> PputResult {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:928:+                                 Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:929:+                                 Some(acc.failed_branch_count),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:951:+                              Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:952:+                              Some(acc.failed_branch_count),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:959:+                              Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:960:+                              Some(acc.failed_branch_count),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:968:+                              Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:969:+                              Some(acc.failed_branch_count),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1030:+                                                        Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1031:+                                                        Some(acc.failed_branch_count),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1074:+                                                        Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1075:+                                                        Some(acc.failed_branch_count),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1108:+              Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1109:+              Some(acc.failed_branch_count),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1118:+    total_run_token_count: Option<u64>,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1119:+    failed_branch_count: Option<u32>,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1120:+    total_wall_time_ms: Option<u64>,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1121: ) -> PputResult {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1133:+    let (verified, pput_runtime, pput_verified, pput_m_verified) =
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1134:+        match (total_run_token_count, total_wall_time_ms) {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1138:+                let progress_runtime = compute_progress_runtime(runtime_accepted);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1139:+                let progress_verified =
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1140:+                    compute_progress_verified(runtime_accepted, post_hoc_verified);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1143:+                    Some(compute_pput(progress_runtime, c_i, t_i)),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1144:+                    Some(compute_pput(progress_verified, c_i, t_i)),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1145:+                    Some(compute_pput_m(progress_verified, c_i, t_i)),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1154:     PputResult {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1161:+        total_run_token_count,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1162:+        failed_branch_count,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1163:+        total_wall_time_ms,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1166:+        pput_verified,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1260:    14	    compute_progress_runtime, compute_progress_verified, compute_pput, compute_pput_m,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1287:    41	struct PputResult {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1329:    83	    total_run_token_count: Option<u64>,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1331:    85	    failed_branch_count: Option<u32>,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1337:    91	    total_wall_time_ms: Option<u64>,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1339:    93	    // verified. Field names align with B1 RunAggregate v2 schema. In Phase B,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1341:    95	    // call site that makes pput_runtime - pput_verified > 0 the H1 signal.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1347:   101	    pput_verified: Option<f64>,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1355:   109	    // Step-B v3 treatment binary: stamp classifier version in every emitted PputResult.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1405:   159	                PputResult { condition: "hybrid_v1".into(), ..r }
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1410:   164	                PputResult {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1450:   204	) -> PputResult {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1479:     9	//   total_run_token_count = Σ over EVERY proposal in the run of
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1507:    37	    pub failed_branch_count: u32,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1535:    65	    /// pass `false` so they accrete failed_branch_count.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1539:    69	            self.failed_branch_count += 1;
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1550:    80	        if self.failed_branch_count > 0 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1551:    81	            self.failed_branch_count -= 1;
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1556:    86	    pub fn total_run_token_count(&self) -> u64 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1598:   128	        assert_eq!(acc.total_run_token_count(), expected_total as u64,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1602:   132	        assert_eq!(acc.failed_branch_count, 5);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1608:   138	        assert_eq!(acc.total_run_token_count(), 0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1610:   140	        assert_eq!(acc.failed_branch_count, 0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1818:    11	//   pput_verified = Progress_verified / (C_i × T_i / 1000)   ← North Star
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1830:    23	//     pput_verified MUST be 0  (Lean reject means Progress_verified = 0)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1833:    26	//   pput_verified is the only North Star metric for H-VPPUT. pput_runtime
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1837:    30	use crate::jsonl_schema::RunAggregate;
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1849:    42	/// Use `compute_progress_verified` instead when the runtime gate already
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1866:    59	pub fn compute_progress_verified(runtime_accepted: bool, post_hoc_verified: bool) -> u8 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1872:    65	pub fn compute_progress_runtime(runtime_accepted: bool) -> u8 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1876:    69	/// Wrap RunAggregate::compute_pput_verified for callers in evaluator that
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1877:    70	/// only have (progress, c_i, t_i_ms). Same math, single source of truth.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1878:    71	pub fn compute_pput(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1879:    72	    RunAggregate::compute_pput_verified(progress, c_i, t_i_ms)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1883:    76	pub fn compute_pput_m(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1884:    77	    RunAggregate::compute_pput_m_verified(progress, c_i, t_i_ms)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1894:    87	    ///  assert progress = 0, pput_verified = 0.0"
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1901:    94	    fn test_pput_verified_zero_when_lean_rejects() {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1904:    97	        // run's progress to 0.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1910:   103	        let progress_runtime = compute_progress_runtime(runtime_accepted);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1911:   104	        let progress_verified =
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1912:   105	            compute_progress_verified(runtime_accepted, post_hoc_verified);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1914:   107	        let pput_runtime = compute_pput(progress_runtime, c_i, t_i_ms);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1915:   108	        let pput_verified = compute_pput(progress_verified, c_i, t_i_ms);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1916:   109	        let pput_m_verified = compute_pput_m(progress_verified, c_i, t_i_ms);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1918:   111	        assert_eq!(progress_runtime, 1u8,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1919:   112	            "runtime gate fired → progress_runtime = 1");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1920:   113	        assert_eq!(progress_verified, 0u8,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1921:   114	            "Lean rejected → progress_verified MUST be 0 (North Star truth)");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1924:   117	        assert_eq!(pput_verified, 0.0,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1925:   118	            "pput_verified MUST be 0 when Lean rejects — North Star Goodhart shield");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1927:   120	            "pput_m_verified must collapse with pput_verified");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1929:   122	        // Sanity: divergence is detectable. pput_runtime - pput_verified > 0
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1931:   124	        assert!(pput_runtime - pput_verified > 0.0,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1932:   125	            "(pput_runtime - pput_verified) > 0 ⟺ Soft Law divergence detected");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1936:   129	    fn test_pput_verified_matches_runtime_when_both_accept() {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1940:   133	        // booleans and made pput_verified always 0).
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1944:   137	        let progress_runtime = compute_progress_runtime(true);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1945:   138	        let progress_verified = compute_progress_verified(true, true);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1947:   140	        assert_eq!(progress_runtime, progress_verified,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1950:   143	            compute_pput(progress_runtime, c_i, t_i_ms),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1951:   144	            compute_pput(progress_verified, c_i, t_i_ms),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1961:   154	        let progress_runtime = compute_progress_runtime(false);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1962:   155	        let progress_verified = compute_progress_verified(false, false);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1964:   157	        assert_eq!(compute_pput(progress_runtime, c_i, t_i_ms), 0.0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1965:   158	        assert_eq!(compute_pput(progress_verified, c_i, t_i_ms), 0.0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1969:   162	    fn test_post_hoc_verified_without_runtime_still_zero_progress() {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1971:   164	        // fired is a wiring bug, not an honest progress signal. Progress
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1974:   167	        assert_eq!(compute_progress_verified(false, true), 0u8,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2203:   244	                                 Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2204:   245	                                 Some(acc.failed_branch_count),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2226:   267	                              Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2227:   268	                              Some(acc.failed_branch_count),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2233:   274	                              Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2234:   275	                              Some(acc.failed_branch_count),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2241:   282	                              Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2242:   283	                              Some(acc.failed_branch_count),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2262:   303	) -> PputResult {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2709:   745	                                                        Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2710:   746	                                                        Some(acc.failed_branch_count),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2895:   926	                                                        Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2896:   927	                                                        Some(acc.failed_branch_count),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2968:   999	              Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2969:  1000	              Some(acc.failed_branch_count),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2982:  1013	    total_run_token_count: Option<u64>,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2983:  1014	    failed_branch_count: Option<u32>,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2984:  1015	    total_wall_time_ms: Option<u64>,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2985:  1016	) -> PputResult {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2998:  1029	    let (verified, pput_runtime, pput_verified, pput_m_verified) =
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:2999:  1030	        match (total_run_token_count, total_wall_time_ms) {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3003:  1034	                let progress_runtime = compute_progress_runtime(runtime_accepted);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3004:  1035	                let progress_verified =
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3005:  1036	                    compute_progress_verified(runtime_accepted, post_hoc_verified);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3008:  1039	                    Some(compute_pput(progress_runtime, c_i, t_i)),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3009:  1040	                    Some(compute_pput(progress_verified, c_i, t_i)),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3010:  1041	                    Some(compute_pput_m(progress_verified, c_i, t_i)),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3019:  1050	    PputResult {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3037:  1068	        total_run_token_count,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3038:  1069	        failed_branch_count,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3039:  1070	        total_wall_time_ms,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3042:  1073	        pput_verified,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3051:  1082	/// Returns the relative path (for embedding in PputResult) or None on I/O failure.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3080:     7	// Versioning: every v2 record carries `schema_version = "v2.0"`. Legacy Paper-1
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3081:     8	// era jsonl rows (the `PputResult` shape emitted by evaluator before this commit)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3082:     9	// have NO `schema_version` field, so `RunRecord::from_json` discriminates on
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3083:    10	// presence and routes to `LegacyRunAggregate`. No on-disk artifact is rewritten
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3086:    13	// B1 scope: schema definition + round-trip + legacy-compat + zero-progress
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3109:    36	    pub schema_version: String,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3153:    80	/// `pput_verified` = Lean post-hoc verified — H-VPPUT input.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3155:    82	pub struct RunAggregate {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3160:    87	    pub schema_version: String,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3166:    93	    pub total_run_token_count: u64,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3168:    95	    pub total_wall_time_ms: u64,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3170:    97	    pub progress: u8,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3174:   101	    pub pput_verified: f64,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3175:   102	    /// 10^6 × pput_verified — display unit (PREREG § 5).
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3177:   104	    pub failed_branch_count: u32,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3193:   120	impl RunAggregate {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3194:   121	    /// Compute pput_verified per PREREG § 5:
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3195:   122	    ///   pput_verified = progress / (c_i * t_i_ms / 1000)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3196:   123	    /// Returns 0.0 when progress is 0, OR when c_i or t_i_ms is 0
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3198:   125	    pub fn compute_pput_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3199:   126	        if progress == 0 || c_i == 0 || t_i_ms == 0 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3203:   130	        (progress as f64) / denom
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3206:   133	    /// Display unit: 10^6 × pput_verified.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3207:   134	    pub fn compute_pput_m_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3208:   135	        1.0e6 * Self::compute_pput_verified(progress, c_i, t_i_ms)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3212:   139	/// Legacy v1 run row — mirrors the pre-v2 `PputResult` shape emitted by the
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3220:   147	pub struct LegacyRunAggregate {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3239:   166	    V2(RunAggregate),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3240:   167	    Legacy(LegacyRunAggregate),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3244:   171	    /// Parse one jsonl line. v2 if `schema_version` present, else legacy.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3248:   175	        let is_v2 = v.get("schema_version")
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3264:   191	    fn sample_run() -> RunAggregate {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3265:   192	        RunAggregate {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3269:   196	            schema_version: SCHEMA_VERSION_V2.into(),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3273:   200	            total_run_token_count: 4096,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3274:   201	            total_wall_time_ms: 12_000,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3275:   202	            progress: 1,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3277:   204	            pput_verified: RunAggregate::compute_pput_verified(1, 4096, 12_000),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3278:   205	            pput_m_verified: RunAggregate::compute_pput_m_verified(1, 4096, 12_000),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3279:   206	            failed_branch_count: 3,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3293:   220	        let parsed: RunAggregate = serde_json::from_str(&line).expect("deserialize");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3294:   221	        assert_eq!(parsed, original, "v2 RunAggregate must round-trip");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3295:   222	        assert!(line.contains("\"schema_version\":\"v2.0\""),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3296:   223	                "serialized line must stamp schema_version");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3300:   227	    fn test_pput_verified_zero_when_progress_zero() {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3302:   229	        // pput_verified = 0 regardless of cost / wall-time.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3303:   230	        assert_eq!(RunAggregate::compute_pput_verified(0, 1000, 5000), 0.0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3304:   231	        assert_eq!(RunAggregate::compute_pput_m_verified(0, 1000, 5000), 0.0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3310:   237	        r.progress = 0;
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3311:   238	        r.pput_verified = RunAggregate::compute_pput_verified(0, r.total_run_token_count, r.total_wall_time_ms);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3312:   239	        r.pput_m_verified = RunAggregate::compute_pput_m_verified(0, r.total_run_token_count, r.total_wall_time_ms);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3313:   240	        assert_eq!(r.pput_verified, 0.0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3317:   244	        assert_eq!(RunAggregate::compute_pput_verified(1, 0, 5000), 0.0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3318:   245	        assert_eq!(RunAggregate::compute_pput_verified(1, 1000, 0), 0.0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3327:   254	        match RunRecord::from_json(legacy_line).expect("legacy line parses") {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3336:/bin/bash -lc "rg -n \"client\\.generate|record_llm_call|record_proposal|flip_last_failed|verify_omega|verify_partial|mark_first_read|mark_final_accept|make_pput|verify_post_hoc|total_run_token_count|pput_verified|RunAggregate|progress\" experiments/minif2f_v4/src src -g '*.rs'" in /home/zephryj/projects/turingosv4
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3338:experiments/minif2f_v4/src/cost_aggregator.rs:9://   total_run_token_count = Σ over EVERY proposal in the run of
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3343:experiments/minif2f_v4/src/cost_aggregator.rs:86:    pub fn total_run_token_count(&self) -> u64 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3348:experiments/minif2f_v4/src/cost_aggregator.rs:128:        assert_eq!(acc.total_run_token_count(), expected_total as u64,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3349:experiments/minif2f_v4/src/cost_aggregator.rs:138:        assert_eq!(acc.total_run_token_count(), 0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3350:experiments/minif2f_v4/src/post_hoc_verifier.rs:11://   pput_verified = Progress_verified / (C_i × T_i / 1000)   ← North Star
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3351:experiments/minif2f_v4/src/post_hoc_verifier.rs:23://     pput_verified MUST be 0  (Lean reject means Progress_verified = 0)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3352:experiments/minif2f_v4/src/post_hoc_verifier.rs:26://   pput_verified is the only North Star metric for H-VPPUT. pput_runtime
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3353:experiments/minif2f_v4/src/post_hoc_verifier.rs:30:use crate::jsonl_schema::RunAggregate;
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3355:experiments/minif2f_v4/src/post_hoc_verifier.rs:42:/// Use `compute_progress_verified` instead when the runtime gate already
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3358:experiments/minif2f_v4/src/post_hoc_verifier.rs:59:pub fn compute_progress_verified(runtime_accepted: bool, post_hoc_verified: bool) -> u8 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3359:experiments/minif2f_v4/src/post_hoc_verifier.rs:65:pub fn compute_progress_runtime(runtime_accepted: bool) -> u8 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3360:experiments/minif2f_v4/src/post_hoc_verifier.rs:69:/// Wrap RunAggregate::compute_pput_verified for callers in evaluator that
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3361:experiments/minif2f_v4/src/post_hoc_verifier.rs:70:/// only have (progress, c_i, t_i_ms). Same math, single source of truth.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3362:experiments/minif2f_v4/src/post_hoc_verifier.rs:71:pub fn compute_pput(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3363:experiments/minif2f_v4/src/post_hoc_verifier.rs:72:    RunAggregate::compute_pput_verified(progress, c_i, t_i_ms)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3364:experiments/minif2f_v4/src/post_hoc_verifier.rs:76:pub fn compute_pput_m(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3365:experiments/minif2f_v4/src/post_hoc_verifier.rs:77:    RunAggregate::compute_pput_m_verified(progress, c_i, t_i_ms)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3366:experiments/minif2f_v4/src/post_hoc_verifier.rs:87:    ///  assert progress = 0, pput_verified = 0.0"
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3367:experiments/minif2f_v4/src/post_hoc_verifier.rs:94:    fn test_pput_verified_zero_when_lean_rejects() {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3368:experiments/minif2f_v4/src/post_hoc_verifier.rs:97:        // run's progress to 0.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3369:experiments/minif2f_v4/src/post_hoc_verifier.rs:103:        let progress_runtime = compute_progress_runtime(runtime_accepted);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3370:experiments/minif2f_v4/src/post_hoc_verifier.rs:104:        let progress_verified =
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3371:experiments/minif2f_v4/src/post_hoc_verifier.rs:105:            compute_progress_verified(runtime_accepted, post_hoc_verified);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3372:experiments/minif2f_v4/src/post_hoc_verifier.rs:107:        let pput_runtime = compute_pput(progress_runtime, c_i, t_i_ms);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3373:experiments/minif2f_v4/src/post_hoc_verifier.rs:108:        let pput_verified = compute_pput(progress_verified, c_i, t_i_ms);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3374:experiments/minif2f_v4/src/post_hoc_verifier.rs:109:        let pput_m_verified = compute_pput_m(progress_verified, c_i, t_i_ms);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3375:experiments/minif2f_v4/src/post_hoc_verifier.rs:111:        assert_eq!(progress_runtime, 1u8,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3376:experiments/minif2f_v4/src/post_hoc_verifier.rs:112:            "runtime gate fired → progress_runtime = 1");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3377:experiments/minif2f_v4/src/post_hoc_verifier.rs:113:        assert_eq!(progress_verified, 0u8,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3378:experiments/minif2f_v4/src/post_hoc_verifier.rs:114:            "Lean rejected → progress_verified MUST be 0 (North Star truth)");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3379:experiments/minif2f_v4/src/post_hoc_verifier.rs:117:        assert_eq!(pput_verified, 0.0,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3380:experiments/minif2f_v4/src/post_hoc_verifier.rs:118:            "pput_verified MUST be 0 when Lean rejects — North Star Goodhart shield");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3381:experiments/minif2f_v4/src/post_hoc_verifier.rs:120:            "pput_m_verified must collapse with pput_verified");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3382:experiments/minif2f_v4/src/post_hoc_verifier.rs:122:        // Sanity: divergence is detectable. pput_runtime - pput_verified > 0
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3383:experiments/minif2f_v4/src/post_hoc_verifier.rs:124:        assert!(pput_runtime - pput_verified > 0.0,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3384:experiments/minif2f_v4/src/post_hoc_verifier.rs:125:            "(pput_runtime - pput_verified) > 0 ⟺ Soft Law divergence detected");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3385:experiments/minif2f_v4/src/post_hoc_verifier.rs:129:    fn test_pput_verified_matches_runtime_when_both_accept() {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3386:experiments/minif2f_v4/src/post_hoc_verifier.rs:133:        // booleans and made pput_verified always 0).
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3387:experiments/minif2f_v4/src/post_hoc_verifier.rs:137:        let progress_runtime = compute_progress_runtime(true);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3388:experiments/minif2f_v4/src/post_hoc_verifier.rs:138:        let progress_verified = compute_progress_verified(true, true);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3389:experiments/minif2f_v4/src/post_hoc_verifier.rs:140:        assert_eq!(progress_runtime, progress_verified,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3390:experiments/minif2f_v4/src/post_hoc_verifier.rs:143:            compute_pput(progress_runtime, c_i, t_i_ms),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3391:experiments/minif2f_v4/src/post_hoc_verifier.rs:144:            compute_pput(progress_verified, c_i, t_i_ms),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3392:experiments/minif2f_v4/src/post_hoc_verifier.rs:154:        let progress_runtime = compute_progress_runtime(false);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3393:experiments/minif2f_v4/src/post_hoc_verifier.rs:155:        let progress_verified = compute_progress_verified(false, false);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3394:experiments/minif2f_v4/src/post_hoc_verifier.rs:157:        assert_eq!(compute_pput(progress_runtime, c_i, t_i_ms), 0.0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3395:experiments/minif2f_v4/src/post_hoc_verifier.rs:158:        assert_eq!(compute_pput(progress_verified, c_i, t_i_ms), 0.0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3396:experiments/minif2f_v4/src/post_hoc_verifier.rs:162:    fn test_post_hoc_verified_without_runtime_still_zero_progress() {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3397:experiments/minif2f_v4/src/post_hoc_verifier.rs:164:        // fired is a wiring bug, not an honest progress signal. Progress
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3398:experiments/minif2f_v4/src/post_hoc_verifier.rs:167:        assert_eq!(compute_progress_verified(false, true), 0u8,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3399:experiments/minif2f_v4/src/bin/evaluator.rs:14:    compute_progress_runtime, compute_progress_verified, compute_pput, compute_pput_m,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3401:experiments/minif2f_v4/src/bin/evaluator.rs:83:    total_run_token_count: Option<u64>,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3402:experiments/minif2f_v4/src/bin/evaluator.rs:93:    // verified. Field names align with B1 RunAggregate v2 schema. In Phase B,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3403:experiments/minif2f_v4/src/bin/evaluator.rs:95:    // call site that makes pput_runtime - pput_verified > 0 the H1 signal.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3404:experiments/minif2f_v4/src/bin/evaluator.rs:101:    pput_verified: Option<f64>,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3411:experiments/minif2f_v4/src/bin/evaluator.rs:244:                                 Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3416:experiments/minif2f_v4/src/bin/evaluator.rs:267:                              Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3418:experiments/minif2f_v4/src/bin/evaluator.rs:274:                              Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3420:experiments/minif2f_v4/src/bin/evaluator.rs:282:                              Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3431:experiments/minif2f_v4/src/bin/evaluator.rs:745:                                                        Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3436:experiments/minif2f_v4/src/bin/evaluator.rs:926:                                                        Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3439:experiments/minif2f_v4/src/bin/evaluator.rs:999:              Some(acc.total_run_token_count()),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3441:experiments/minif2f_v4/src/bin/evaluator.rs:1013:    total_run_token_count: Option<u64>,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3443:experiments/minif2f_v4/src/bin/evaluator.rs:1029:    let (verified, pput_runtime, pput_verified, pput_m_verified) =
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3444:experiments/minif2f_v4/src/bin/evaluator.rs:1030:        match (total_run_token_count, total_wall_time_ms) {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3445:experiments/minif2f_v4/src/bin/evaluator.rs:1034:                let progress_runtime = compute_progress_runtime(runtime_accepted);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3446:experiments/minif2f_v4/src/bin/evaluator.rs:1035:                let progress_verified =
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3447:experiments/minif2f_v4/src/bin/evaluator.rs:1036:                    compute_progress_verified(runtime_accepted, post_hoc_verified);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3448:experiments/minif2f_v4/src/bin/evaluator.rs:1039:                    Some(compute_pput(progress_runtime, c_i, t_i)),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3449:experiments/minif2f_v4/src/bin/evaluator.rs:1040:                    Some(compute_pput(progress_verified, c_i, t_i)),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3450:experiments/minif2f_v4/src/bin/evaluator.rs:1041:                    Some(compute_pput_m(progress_verified, c_i, t_i)),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3451:experiments/minif2f_v4/src/bin/evaluator.rs:1068:        total_run_token_count,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3452:experiments/minif2f_v4/src/bin/evaluator.rs:1073:        pput_verified,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3453:experiments/minif2f_v4/src/jsonl_schema.rs:10:// presence and routes to `LegacyRunAggregate`. No on-disk artifact is rewritten
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3454:experiments/minif2f_v4/src/jsonl_schema.rs:13:// B1 scope: schema definition + round-trip + legacy-compat + zero-progress
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3455:experiments/minif2f_v4/src/jsonl_schema.rs:80:/// `pput_verified` = Lean post-hoc verified — H-VPPUT input.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3456:experiments/minif2f_v4/src/jsonl_schema.rs:82:pub struct RunAggregate {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3457:experiments/minif2f_v4/src/jsonl_schema.rs:93:    pub total_run_token_count: u64,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3458:experiments/minif2f_v4/src/jsonl_schema.rs:97:    pub progress: u8,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3459:experiments/minif2f_v4/src/jsonl_schema.rs:101:    pub pput_verified: f64,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3460:experiments/minif2f_v4/src/jsonl_schema.rs:102:    /// 10^6 × pput_verified — display unit (PREREG § 5).
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3461:experiments/minif2f_v4/src/jsonl_schema.rs:120:impl RunAggregate {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3462:experiments/minif2f_v4/src/jsonl_schema.rs:121:    /// Compute pput_verified per PREREG § 5:
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3463:experiments/minif2f_v4/src/jsonl_schema.rs:122:    ///   pput_verified = progress / (c_i * t_i_ms / 1000)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3464:experiments/minif2f_v4/src/jsonl_schema.rs:123:    /// Returns 0.0 when progress is 0, OR when c_i or t_i_ms is 0
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3465:experiments/minif2f_v4/src/jsonl_schema.rs:125:    pub fn compute_pput_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3466:experiments/minif2f_v4/src/jsonl_schema.rs:126:        if progress == 0 || c_i == 0 || t_i_ms == 0 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3467:experiments/minif2f_v4/src/jsonl_schema.rs:130:        (progress as f64) / denom
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3468:experiments/minif2f_v4/src/jsonl_schema.rs:133:    /// Display unit: 10^6 × pput_verified.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3469:experiments/minif2f_v4/src/jsonl_schema.rs:134:    pub fn compute_pput_m_verified(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3470:experiments/minif2f_v4/src/jsonl_schema.rs:135:        1.0e6 * Self::compute_pput_verified(progress, c_i, t_i_ms)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3471:experiments/minif2f_v4/src/jsonl_schema.rs:147:pub struct LegacyRunAggregate {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3472:experiments/minif2f_v4/src/jsonl_schema.rs:166:    V2(RunAggregate),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3473:experiments/minif2f_v4/src/jsonl_schema.rs:167:    Legacy(LegacyRunAggregate),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3474:experiments/minif2f_v4/src/jsonl_schema.rs:191:    fn sample_run() -> RunAggregate {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3475:experiments/minif2f_v4/src/jsonl_schema.rs:192:        RunAggregate {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3476:experiments/minif2f_v4/src/jsonl_schema.rs:200:            total_run_token_count: 4096,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3477:experiments/minif2f_v4/src/jsonl_schema.rs:202:            progress: 1,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3478:experiments/minif2f_v4/src/jsonl_schema.rs:204:            pput_verified: RunAggregate::compute_pput_verified(1, 4096, 12_000),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3479:experiments/minif2f_v4/src/jsonl_schema.rs:205:            pput_m_verified: RunAggregate::compute_pput_m_verified(1, 4096, 12_000),
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3480:experiments/minif2f_v4/src/jsonl_schema.rs:220:        let parsed: RunAggregate = serde_json::from_str(&line).expect("deserialize");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3481:experiments/minif2f_v4/src/jsonl_schema.rs:221:        assert_eq!(parsed, original, "v2 RunAggregate must round-trip");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3482:experiments/minif2f_v4/src/jsonl_schema.rs:227:    fn test_pput_verified_zero_when_progress_zero() {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3483:experiments/minif2f_v4/src/jsonl_schema.rs:229:        // pput_verified = 0 regardless of cost / wall-time.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3484:experiments/minif2f_v4/src/jsonl_schema.rs:230:        assert_eq!(RunAggregate::compute_pput_verified(0, 1000, 5000), 0.0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3485:experiments/minif2f_v4/src/jsonl_schema.rs:231:        assert_eq!(RunAggregate::compute_pput_m_verified(0, 1000, 5000), 0.0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3486:experiments/minif2f_v4/src/jsonl_schema.rs:237:        r.progress = 0;
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3487:experiments/minif2f_v4/src/jsonl_schema.rs:238:        r.pput_verified = RunAggregate::compute_pput_verified(0, r.total_run_token_count, r.total_wall_time_ms);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3488:experiments/minif2f_v4/src/jsonl_schema.rs:239:        r.pput_m_verified = RunAggregate::compute_pput_m_verified(0, r.total_run_token_count, r.total_wall_time_ms);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3489:experiments/minif2f_v4/src/jsonl_schema.rs:240:        assert_eq!(r.pput_verified, 0.0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3490:experiments/minif2f_v4/src/jsonl_schema.rs:244:        assert_eq!(RunAggregate::compute_pput_verified(1, 0, 5000), 0.0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3491:experiments/minif2f_v4/src/jsonl_schema.rs:245:        assert_eq!(RunAggregate::compute_pput_verified(1, 1000, 0), 0.0);
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3504:src/sdk/error_abstraction.rs:103:    if c.contains("`simp` made no progress") || c.contains("simp made no progress") {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3505:src/sdk/error_abstraction.rs:152:    fn fixture_simp_no_progress() {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3506:src/sdk/error_abstraction.rs:153:        let s = "<stdin>:17:2: error: `simp` made no progress";
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3507:src/sdk/prompt.rs:2:// Constitutional basis: Art. III.2 (encapsulate details, progressive disclosure)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3508:src/sdk/prompt.rs:66:    // Art. III.2 progressive disclosure: surface recent search hits so the
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3509:src/sdk/prompt.rs:91:    // progression tool. Art. IV strict: δ writes one square. No monolithic
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3510:src/sdk/prompt.rs:98:        prompt.push_str("    THE proof-progression tool. Submit ONE tactic (e.g. `intro h`,\n");
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3526:   254	        match RunRecord::from_json(legacy_line).expect("legacy line parses") {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3541:   269	        match RunRecord::from_json(&v2_line).expect("v2 line parses") {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3719:./handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:82:*   **(Q6.c) Trust Root**: **PASS (Finding for B7)**. The PREREG § 1.8 defines the Trust Root. The logic for cost, time, and verified progress accounting is fundamental to the integrity of the PPUT metric. Therefore, `cost_aggregator.rs`, `wall_clock.rs`, and `post_hoc_verifier.rs` **must** be added to the Trust Root manifest. This is a mandatory action item for the B7 implementation.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3764:./handover/audits/GEMINI_PPUT_CCL_AUDIT_2026-04-26.md:14:**PASS**. The definitions for V-PPUT, C_i, T_i, ArtifactState, and the Trust Root are exceptionally precise and leave little room for post-hoc reinterpretation. The commitment to count all tokens from all branches (§ 1.2) and the wall-clock definition of T_i (§ 1.3) are strong. The 4-state machine for artifacts (§ 1.7) is unambiguous and correctly separates artifact confidence from the binary ground truth of task progress.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4291:./handover/ai-direct/STEPB_ART_II1_V3_2026-04-16.md:80:Actually reuse: **v3.2 resume-in-progress IS the control** — it's chat on main bus.rs. No need to re-run control. Apples-to-apples.
src/sdk/error_abstraction.rs:16:/// Stamped in PputResult.classifier_version per C-012.
src/sdk/error_abstraction.rs:103:    if c.contains("`simp` made no progress") || c.contains("simp made no progress") {
src/sdk/error_abstraction.rs:152:    fn fixture_simp_no_progress() {
src/sdk/error_abstraction.rs:153:        let s = "<stdin>:17:2: error: `simp` made no progress";
src/sdk/prompt.rs:2:// Constitutional basis: Art. III.2 (encapsulate details, progressive disclosure)
src/sdk/prompt.rs:66:    // Art. III.2 progressive disclosure: surface recent search hits so the
src/sdk/prompt.rs:91:    // progression tool. Art. IV strict: δ writes one square. No monolithic
src/sdk/prompt.rs:98:        prompt.push_str("    THE proof-progression tool. Submit ONE tactic (e.g. `intro h`,\n");
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:94:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:96:ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:441:| H1 | Soft Law violates ground-truth honesty — its `pput_verified` is lower than Full despite comparable `pput_runtime` | C | McNemar one-sided on per-(problem, seed) discordant pairs of `Progress_i` (Lean-verified golden path) | (problem, seed) | Full > Soft Law | gap between `pput_runtime` and `pput_verified` for Soft Law (gameability indicator); FAR per mode |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:523:- B4 `pput_verified` vs `pput_runtime` dual-field separation
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:596:- F1: Phase C — Soft Law mode has VPPUT comparable to Full (within Bonferroni-corrected CI). Implies PPUT does not penalize fake-progress; metric is broken or mode does not actually violate constitution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:805:  "schema_version": "1.0",
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1308:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1310:ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1561:| H1 | Soft Law mode has lower `pput_verified` than Full mode on adaptation hard-10, even when `pput_runtime` is comparable | C | McNemar one-sided on per-task verified-progress + paired Δ on PPUT-M | Soft Law < Full |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1582:- B4 `pput_verified` vs `pput_runtime` dual-field separation
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1633:- F1: Phase C — Soft Law mode has VPPUT comparable to Full (within Bonferroni-corrected CI). Implies PPUT does not penalize fake-progress; metric is broken or mode does not actually violate constitution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1771:> TuringOS 的长期目标不是单纯提高 solve rate，而是在宪法约束下最大化 Verified PPUT：单位 token、单位时间内产生的可验证 golden-path progress。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1793:| Art. I.1 | 只有谓词通过才有 progress |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1828:| 安全 | FAR | 防止错误状态被当成 progress |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1885:golden_path_token_count, total_run_token_count, total_wall_time_ms,
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1886:progress (0|1), pput, failed_branch_count, rollback_count,
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1925:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:1927:ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2122:  "schema_version": "1.0",
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2423:1. **Phase A — Pre-flight** (days 1-3, 2026-04-26 → 2026-04-28, **in progress**)
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2433:   - `pput_verified` vs `pput_runtime` dual-field separation
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2478:- Three constitutional ablations restated in PPUT terms: Soft Law (post-hoc Lean reject → progress=0), Panopticon (CPR↑+IAC↑→PPUT↓), Amnesia (ERR↓→PPUT↓).
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:2863:  but unreachable from swarm loop. Constitutional Art. III.2 (progressive disclosure)
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:3106:1. Art. II.1 broadcast — **Step-B v3.3 in progress** (treatment arm running)
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:3114:Priority: 1 (in progress) → 2 (highest N_c impact) → 3 (highest diversity impact) → 5 → 4 → 6
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4092:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4094:ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4407:| H1 | Soft Law violates ground-truth honesty — its `pput_verified` is lower than Full despite comparable `pput_runtime` | C | McNemar one-sided on per-(problem, seed) discordant pairs of `Progress_i` (Lean-verified golden path) | (problem, seed) | Full > Soft Law | gap between `pput_runtime` and `pput_verified` for Soft Law (gameability indicator); FAR per mode |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4456:- B4 `pput_verified` vs `pput_runtime` dual-field separation
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4529:- F1: Phase C — Soft Law mode has VPPUT comparable to Full (within Bonferroni-corrected CI). Implies PPUT does not penalize fake-progress; metric is broken or mode does not actually violate constitution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4733:> TuringOS 的长期目标不是单纯提高 solve rate，而是在宪法约束下最大化 Verified PPUT：单位 token、单位时间内产生的可验证 golden-path progress。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4755:| Art. I.1 | 只有谓词通过才有 progress |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4790:| 安全 | FAR | 防止错误状态被当成 progress |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4847:golden_path_token_count, total_run_token_count, total_wall_time_ms,
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4848:progress (0|1), pput, failed_branch_count, rollback_count,
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4887:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:4889:ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5084:  "schema_version": "1.0",
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5587:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5589:ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5840:| H1 | Soft Law mode has lower `pput_verified` than Full mode on adaptation hard-10, even when `pput_runtime` is comparable | C | McNemar one-sided on per-task verified-progress + paired Δ on PPUT-M | Soft Law < Full |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5861:- B4 `pput_verified` vs `pput_runtime` dual-field separation
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:5912:- F1: Phase C — Soft Law mode has VPPUT comparable to Full (within Bonferroni-corrected CI). Implies PPUT does not penalize fake-progress; metric is broken or mode does not actually violate constitution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6050:> TuringOS 的长期目标不是单纯提高 solve rate，而是在宪法约束下最大化 Verified PPUT：单位 token、单位时间内产生的可验证 golden-path progress。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6072:| Art. I.1 | 只有谓词通过才有 progress |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6107:| 安全 | FAR | 防止错误状态被当成 progress |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6164:golden_path_token_count, total_run_token_count, total_wall_time_ms,
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6165:progress (0|1), pput, failed_branch_count, rollback_count,
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6204:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6206:ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6401:  "schema_version": "1.0",
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6702:1. **Phase A — Pre-flight** (days 1-3, 2026-04-26 → 2026-04-28, **in progress**)
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6712:   - `pput_verified` vs `pput_runtime` dual-field separation
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:6757:- Three constitutional ablations restated in PPUT terms: Soft Law (post-hoc Lean reject → progress=0), Panopticon (CPR↑+IAC↑→PPUT↓), Amnesia (ERR↓→PPUT↓).
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:7142:  but unreachable from swarm loop. Constitutional Art. III.2 (progressive disclosure)
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:7385:1. Art. II.1 broadcast — **Step-B v3.3 in progress** (treatment arm running)
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:7393:Priority: 1 (in progress) → 2 (highest N_c impact) → 3 (highest diversity impact) → 5 → 4 → 6
handover/evidence/ADVERSARIAL_AUDIT_2026-04-23.md:78:1. **Add build_sha to PputResult** via `std::env::set_var("BUILD_SHA", git_rev_parse)` in run_list.sh before launching evaluator. Next batch gets per-row provenance.
handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md:30:> TuringOS 的长期目标不是单纯提高 solve rate，而是在宪法约束下最大化 Verified PPUT：单位 token、单位时间内产生的可验证 golden-path progress。
handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md:52:| Art. I.1 | 只有谓词通过才有 progress |
handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md:87:| 安全 | FAR | 防止错误状态被当成 progress |
handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md:144:golden_path_token_count, total_run_token_count, total_wall_time_ms,
handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md:145:progress (0|1), pput, failed_branch_count, rollback_count,
handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md:184:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md:186:ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。
handover/ai-direct/OPEN_DECISIONS_2026-04-26.md:3:**Purpose**: questions waiting for user input. New sessions should resolve OR defer-with-default these before continuing execution. NOT a brainstorm dump — only items that block forward progress or are irreversible.
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:32:    schema_version: String,     // "v2.0"
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:59:struct RunAggregate {
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:65:    schema_version: String,
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:69:    total_run_token_count: u64,              // C_i (sum over all proposals)
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:70:    total_wall_time_ms: u64,                 // T_i
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:71:    progress: u8,                            // 0 or 1 (Lean ground truth)
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:73:    pput_verified: f64,                      // Progress / (C_i × T_i / 1000)  [token-second]
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:74:    pput_m_verified: f64,                    // 10^6 × pput_verified
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:75:    failed_branch_count: u32,
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:92:- `cargo test test_pput_verified_zero_when_progress_zero` PASS
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:93:- Old jsonl files (Paper 1 era) still readable via `schema_version` discriminant
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:129:- `cargo test test_failed_branches_counted_in_total_cost` (anti-Goodhart conformance per PREREG § 3): synthesize a run with 5 failed proposals + 1 success → assert `total_run_token_count == sum(all 6 proposals)`.
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:156:### B4 — `pput_verified` vs `pput_runtime` 双字段
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:160:- `pput_verified` = Progress_verified / (C_i × T_i)  [where Progress_verified = 1 if Lean post-hoc verifies the golden path]
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:162:For Soft Law mode (which fakes runtime acceptance), `pput_runtime` may inflate but `pput_verified` should drop to 0. This is the H1 detection mechanism.
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:169:- `cargo test test_pput_verified_zero_when_lean_rejects` (PREREG § 3 `test_golden_path_requires_ground_truth_acceptance`): synthesize a run that records runtime accept + Lean reject → assert `progress = 0`, `pput_verified = 0.0`.
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:170:- For Phase C ablation Soft Law mode: confirm `pput_runtime > 0` but `pput_verified` reflects Lean truth.
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:276:schema_version = "1.0"
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:277:progress_definition = "1 iff Lean ground-truth verifies golden_path_payload"
handover/ai-direct/CHECKPOINT_PHASE_0_2026-04-20.md:11:| `experiments/minif2f_v4/src/bin/evaluator.rs` | Added `gp_payload`, `gp_path`, `gp_proof_file` to `PputResult`; expanded OMEGA-accept logging to include `payload[0..500]` preview; new `persist_proof_artifact()` writes self-contained `.lean` file per accept |
handover/ai-direct/STEPB_ART_II1_REDESIGN_2026-04-16.md:33:    TacticFailed(String),    // "linarith failed", "simp made no progress" — tactic name only
handover/ai-direct/PAPER_1_OUTLINE_2026-04-22.md:207:| I.2 (效用) | PPUT | `PputResult.pput` | `evaluator.rs:36-42` |
handover/ai-direct/PLAN_PHASE_8_TO_10_2026-04-22.md:58:- **现状**：oneshot 直接 `oracle.verify_omega` 返回 PputResult，不经 bus write
handover/ai-direct/PLAN_PHASE_8_TO_10_2026-04-22.md:249:- JSON schema version 字段（`schema_version: "v4.0"`）
handover/ai-direct/PAPER_1_OUTLINE_v2_E1_LED_2026-04-23.md:129:- **Dockerfile** (in progress): pins Lean 4 toolchain + Mathlib + deepseek-chat proxy
handover/ai-direct/ROADMAP_LAUNCH_2026-04-21.md:108:5. **P1-5** Search tool 从 filename 改 content grep (F-2026-04-19-04) — 让 Art. III.2 progressive disclosure 真正生效。**独立**。
handover/ai-direct/CHIEF_ARCHITECT_REPORT_2026-04-23.md:101:| 141421 | running 27/50 in progress | 15/27 → projected 18-22/50 | — | — | — | pending |
handover/ai-direct/CHIEF_ARCHITECT_REPORT_2026-04-23.md:219:- **LLM cost**: ≈ $8-10 (subagent audits + stage 5 + partial seeds 74677/31415/2718 + seed 141421 in progress)
handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:20:1. **Phase A — Pre-flight** (days 1-3, 2026-04-26 → 2026-04-28, **in progress**)
handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:30:   - `pput_verified` vs `pput_runtime` dual-field separation
handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:75:- Three constitutional ablations restated in PPUT terms: Soft Law (post-hoc Lean reject → progress=0), Panopticon (CPR↑+IAC↑→PPUT↓), Amnesia (ERR↓→PPUT↓).
handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:468:  but unreachable from swarm loop. Constitutional Art. III.2 (progressive disclosure)
handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:711:1. Art. II.1 broadcast — **Step-B v3.3 in progress** (treatment arm running)
handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:719:Priority: 1 (in progress) → 2 (highest N_c impact) → 3 (highest diversity impact) → 5 → 4 → 6
handover/ai-direct/PAPER_1_FULL_DRAFT_2026-04-23.md:216:### 5.1 EXCLUDE_META_PLANNER ablation (in progress)
handover/ai-direct/PAPER_1_FULL_DRAFT_2026-04-23.md:226:### 5.2 Additional seed (2357) in progress
handover/ai-direct/PAPER_1_FULL_DRAFT_2026-04-23.md:300:1. **N=10 per A/B per seed**. 3 seeds gives 30 paired trials; 4 seeds (in progress) gives 40. Published N should reflect final seed count.
handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/brief.md:35:Removed silent in-memory fallback in run_oneshot. If `TuringBus::with_wal_path` fails, emit `MEASUREMENT_ERROR oneshot WAL: <e>` and return a non-result PputResult (condition=`measurement_error`, solved=false, tx=1). Batch runner retries; no bus without WAL will ever claim OMEGA.
handover/ai-direct/AUDIT_V3_2026-04-15.md:48:**宪法 Art. I.2** 定义 PPUT 是"progress per unit time"（单题指标）。
handover/preregistration/PROXY_SATURATION_FINDING_2026-04-24.md:65:### 3.3 E1 v2 in-progress data: DISCARD
handover/preregistration/PROXY_SATURATION_FINDING_2026-04-24.md:102:- **Discard**: all 12 in-progress jsonl files from E1 v2 first attempt
handover/ai-direct/STEPB_ART_II1_V3_2026-04-16.md:13:    TacticSimpNoProgress,  // "simp made no progress"
handover/ai-direct/STEPB_ART_II1_V3_2026-04-16.md:80:Actually reuse: **v3.2 resume-in-progress IS the control** — it's chat on main bus.rs. No need to re-run control. Apples-to-apples.
handover/ai-direct/STEPB_ART_II1_V3_2026-04-16.md:103:- NEW: is "reuse v3.2 resume as control" valid? Same code, same model, same prompt, same analyzer — only missing: we're calling an in-progress batch "control" mid-flight. Alternative: wait for v3.2 resume to finish, freeze it, then run treatment.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:17:EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:19:ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:402:**Descriptive secondary endpoints** (logged, NOT in inferential family, NO α correction): mode-level CPR, mode-level IAC, mode-level FAR, gap between pput_runtime and pput_verified for Soft Law, ERR for Amnesia, mean prompt length per mode.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:504:- B4 `pput_verified` vs `pput_runtime` dual-field separation
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:577:- F1: Phase C — Soft Law mode has VPPUT comparable to Full (within Bonferroni-corrected CI). Implies PPUT does not penalize fake-progress; metric is broken or mode does not actually violate constitution.
handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/brief.md:137:**Overall**: PASS → progress to Phase 2 A/B. CHALLENGE → list residuals. VETO → stop.
handover/preregistration/scripts/split_pput_ccl.py:113:        "schema_version": "1.0",
handover/ai-direct/REGISTRATION_PHASE_9_2026-04-22.md:81:**Discovered 2026-04-22 during auto-research**: current `PputResult` struct
handover/ai-direct/REGISTRATION_PHASE_9_2026-04-22.md:93:- [ ] **9.0.1** Extend `PputResult` with:
handover/ai-direct/LATEST.md:26:- **B1 ✅ DONE** JSONL schema v2 (proposal + run-level) — `experiments/minif2f_v4/src/jsonl_schema.rs`; 3 acceptance tests green; legacy `PputResult` shape readable via `RunRecord::from_json` schema_version dispatcher; evaluator emit wiring deferred to B2/B3/B4 (no fields to populate yet)
handover/ai-direct/LATEST.md:29:- B4 `pput_verified` vs `pput_runtime` 双字段
handover/ai-direct/LATEST.md:47:- B1 的 `ProposalRow` / `RunAggregate` 还没在 evaluator 任何 emit path 上线 — 由 B2 负责 wire-in

codex
I’m separating real blockers from debt. The cost wiring around `client.generate` is tight in the current evaluator, but the B4 Soft Law contract is not enforced by the current API shape, and the v2 schema is not what the evaluator emits yet.
exec
/bin/bash -lc "nl -ba handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md | sed -n '1,190p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# Phase B Implementation Plan — Kernel Instrumentation + PPUT Accounting
     2	
     3	**Phase B window**: days 4-10 of 30-day arc (2026-04-29 → 2026-05-05 wall-clock)
     4	**Authoritative spec**: `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` § 6 Phase B (lines 377-385)
     5	**Gate B exit criterion**: on hard-10 adaptation subset, any single run can self-consistently report (verified golden path? total_tokens? wall_time? VPPUT?). All 11 anti-Goodhart conformance tests + 5-layer sealing + 4 content-meta-predicates + 4 lookup-evasion + Trust Root immutability PASS. Heldout never touched.
     6	
     7	> **For new sessions**: this plan is self-contained. Read this + PREREG § 5 (definitions) + PREREG § 1.7 + § 1.8 (ArtifactState + Trust Root) + PREREG § 3 + § 3.5 + § 3.5.1 (conformance batteries). All file paths are relative to repo root `/home/zephryj/projects/turingosv4/`.
     8	
     9	## Item-by-item plan
    10	
    11	### B1 — JSONL schema v2 (proposal-level + run-level)
    12	
    13	**What**: extend the per-tx jsonl schema and per-run aggregate schema with the fields needed for Verified PPUT computation + LOO heldout protocol + ArtifactState tracking.
    14	
    15	**Files to modify**:
    16	- `experiments/minif2f_v4/src/bin/evaluator.rs` — jsonl emit functions (search current code for `serde_json::to_string` and `writeln`)
    17	- (new) `experiments/minif2f_v4/src/jsonl_schema.rs` — versioned schema struct with serde derives
    18	
    19	**Per-proposal schema (new fields, on top of current)**:
    20	```rust
    21	struct ProposalRow {
    22	    // existing
    23	    run_id: String,
    24	    problem_id: String,
    25	    agent_id: String,
    26	    role: String,
    27	    branch_id: String,
    28	    proposal_hash: String,
    29	    accepted: bool,
    30	    // new for PPUT v2
    31	    split: String,              // "adaptation" | "meta_validation" | "heldout"
    32	    schema_version: String,     // "v2.0"
    33	    context_hash: String,       // hash of input prompt (for retrieval-equivalence audit)
    34	    predicate_result: i32,      // runtime predicate accept = 1, reject = 0
    35	    ground_truth_result: Option<i32>,  // Lean post-hoc verify: 1 / 0 / null=not-yet-checked
    36	    lean_error_category: Option<String>,
    37	    raw_error_hash: Option<String>,
    38	    rollback_to: Option<String>,    // hash of Q^world snapshot to roll back to
    39	    prompt_tokens: u64,
    40	    completion_tokens: u64,
    41	    tool_tokens: u64,                // length of all tool stdout summed
    42	    total_tokens: u64,               // = prompt + completion + tool
    43	    wall_time_ms: u64,
    44	    start_time: String,              // ISO 8601 UTC
    45	    end_time: String,
    46	    ast_depth: u32,
    47	    peer_agents_in_branch: Vec<String>,
    48	    tool_stdout_hash: Option<String>,    // SHA-256 of concatenated tool stdout
    49	    is_on_golden_path: bool,
    50	    golden_path_id: Option<String>,
    51	    // PPUT-CCL meta-loop attribution (Phase D+ but emit field nullable in B)
    52	    architect_artifact_id: Option<String>,   // if this proposal triggered by ArchitectAI artifact
    53	    auditor_attestation: Option<String>,
    54	}
    55	```
    56	
    57	**Per-run aggregate schema (new fields)**:
    58	```rust
    59	struct RunAggregate {
    60	    // existing
    61	    run_id: String,
    62	    problem_id: String,
    63	    solved: bool,
    64	    // new for PPUT v2
    65	    schema_version: String,
    66	    split: String,
    67	    verified: bool,                          // Lean post-hoc PASS
    68	    golden_path_token_count: u64,
    69	    total_run_token_count: u64,              // C_i (sum over all proposals)
    70	    total_wall_time_ms: u64,                 // T_i
    71	    progress: u8,                            // 0 or 1 (Lean ground truth)
    72	    pput_runtime: f64,                       // legacy / runtime accept-based, NEVER for North Star
    73	    pput_verified: f64,                      // Progress / (C_i × T_i / 1000)  [token-second]
    74	    pput_m_verified: f64,                    // 10^6 × pput_verified
    75	    failed_branch_count: u32,
    76	    rollback_count: u32,
    77	    // guardrails
    78	    far: f64,
    79	    err: f64,
    80	    iac: f64,
    81	    cpr: f64,
    82	    // model snapshot per F-2026-04-22-08 (drift defense)
    83	    model_snapshot: String,                  // exact model id + API revision
    84	    git_sha: String,
    85	    binary_sha256: String,
    86	    mode: String,                            // "full" | "panopticon" | "amnesia" | "soft_law" | "homogeneous"
    87	}
    88	```
    89	
    90	**Acceptance criteria**:
    91	- `cargo test test_jsonl_schema_v2_round_trip` PASS (serialize + deserialize back)
    92	- `cargo test test_pput_verified_zero_when_progress_zero` PASS
    93	- Old jsonl files (Paper 1 era) still readable via `schema_version` discriminant
    94	
    95	**Estimated effort**: half day
    96	
    97	---
    98	
    99	### B2 — C_i full-cost aggregator
   100	
   101	**What**: instrument every tx so that `total_tokens` = prompt + completion + tool stdout, summed across ALL proposals in the run (failed + succeeded). Currently the codebase counts per-call tokens but doesn't aggregate failed-branch tokens into the run total.
   102	
   103	**Files to modify**:
   104	- `experiments/minif2f_v4/src/bin/evaluator.rs` — main loop where proposals are dispatched and rejected
   105	- `experiments/minif2f_v4/src/sdk/tools/*.rs` — every tool's `execute` method must return `tool_stdout: String` (or hash + length)
   106	- (new) `src/cost_aggregator.rs` — `RunCostAccumulator` struct that sums token counts per (run_id, problem_id)
   107	
   108	**Sketch**:
   109	```rust
   110	pub struct RunCostAccumulator {
   111	    pub prompt_tokens: u64,
   112	    pub completion_tokens: u64,
   113	    pub tool_tokens: u64,
   114	}
   115	
   116	impl RunCostAccumulator {
   117	    pub fn record_proposal(&mut self, prompt: &str, completion: &str, tool_calls: &[ToolCall]) {
   118	        self.prompt_tokens += count_tokens(prompt);
   119	        self.completion_tokens += count_tokens(completion);
   120	        for tc in tool_calls {
   121	            self.tool_tokens += count_tokens(&tc.stdout);
   122	        }
   123	    }
   124	    pub fn total(&self) -> u64 { self.prompt_tokens + self.completion_tokens + self.tool_tokens }
   125	}
   126	```
   127	
   128	**Acceptance criteria**:
   129	- `cargo test test_failed_branches_counted_in_total_cost` (anti-Goodhart conformance per PREREG § 3): synthesize a run with 5 failed proposals + 1 success → assert `total_run_token_count == sum(all 6 proposals)`.
   130	- Manual sanity: spot-check 3 historical Phase 1 runs; recompute C_i, ensure jsonl emits match.
   131	
   132	**Estimated effort**: 1 day
   133	
   134	---
   135	
   136	### B3 — T_i wall-clock instrumentation
   137	
   138	**What**: T_i = end_time − start_time, where:
   139	- `start_time` = first read of task statement by any agent in the run
   140	- `end_time` = final ground-truth Lean accept OR external timeout
   141	
   142	Currently: T_i is approximated as evaluator wall-clock from main loop start to OMEGA accept; but Lean verify time is sometimes excluded.
   143	
   144	**Files to modify**:
   145	- `experiments/minif2f_v4/src/bin/evaluator.rs` — record `start_time` at first prompt construction, `end_time` after final Lean call returns
   146	- `experiments/minif2f_v4/src/lean4_oracle.rs` — ensure verify time is bracketed inside the same wall-clock window
   147	
   148	**Acceptance criteria**:
   149	- `cargo test test_wall_clock_first_read_to_final_accept` (anti-Goodhart per PREREG § 3): synthetic run with 100ms prompt construction + 5s LLM call + 2s Lean verify → assert `wall_time_ms ≥ 7100`.
   150	- Manual: compare new T_i to legacy `total_run_time` for 3 historical runs; new should be ≥ legacy (wider bracket).
   151	
   152	**Estimated effort**: half day
   153	
   154	---
   155	
   156	### B4 — `pput_verified` vs `pput_runtime` 双字段
   157	
   158	**What**: separate two PPUT computations on every run:
   159	- `pput_runtime` = Progress_runtime / (C_i × T_i)  [where Progress_runtime = 1 if runtime/evaluator accepted]
   160	- `pput_verified` = Progress_verified / (C_i × T_i)  [where Progress_verified = 1 if Lean post-hoc verifies the golden path]
   161	
   162	For Soft Law mode (which fakes runtime acceptance), `pput_runtime` may inflate but `pput_verified` should drop to 0. This is the H1 detection mechanism.
   163	
   164	**Files to modify**:
   165	- `experiments/minif2f_v4/src/bin/evaluator.rs` — after recording run, run `lean4_oracle.verify_omega_detailed(golden_path_payload)` independently and emit both fields.
   166	- (new) `experiments/minif2f_v4/src/post_hoc_verifier.rs` — explicit post-hoc verification path that does NOT short-circuit on runtime accept
   167	
   168	**Acceptance criteria**:
   169	- `cargo test test_pput_verified_zero_when_lean_rejects` (PREREG § 3 `test_golden_path_requires_ground_truth_acceptance`): synthesize a run that records runtime accept + Lean reject → assert `progress = 0`, `pput_verified = 0.0`.
   170	- For Phase C ablation Soft Law mode: confirm `pput_runtime > 0` but `pput_verified` reflects Lean truth.
   171	
   172	**Estimated effort**: half day
   173	
   174	---
   175	
   176	### B5 — Conformance test battery (anti-Goodhart × 11 + sealing × 5-layer + content × 4 + lookup-evasion × 4)
   177	
   178	**What**: implement the test predicates defined in PREREG § 3 + § 3.5 + § 3.5.1. These run as `cargo test` and gate every commit.
   179	
   180	**Files to create**:
   181	- `tests/pput_anti_goodhart.rs` — 11 tests per PREREG § 3
   182	- `tests/heldout_operational_sealing.rs` — 5-layer sealing per PREREG § 2.3 (L1-L5)
   183	- `tests/artifact_content_predicates.rs` — 4 content predicates per PREREG § 3.5
   184	- `tests/artifact_lookup_evasion.rs` — 4 lookup-evasion predicates per PREREG § 3.5.1
   185	- `tests/architect_sole_lt_reader.rs` — D4 cognitive isolation conformance
   186	- `tests/auditor_sees_candidate_only.rs` — D4 cognitive isolation conformance
   187	- `tests/mode_flag_binary_purity.rs` — Phase C C5 conformance
   188	- `tests/trust_root_immutability.rs` — Gate B conformance
   189	
   190	**Test names (must match PREREG § 3)**:

exec
/bin/bash -lc "nl -ba handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md | sed -n '260,395p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   260	experiments/minif2f_v4/src/lean4_oracle.rs
   261	constitution.md
   262	cases/C-*.yaml  (glob; track manifest)
   263	genesis_payload.toml [pput_accounting_0] section
   264	handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json
   265	handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md
   266	```
   267	
   268	**Files to modify**:
   269	- (new) `genesis_payload.toml` (might already exist; check first) — add `[pput_accounting_0]` section + `[trust_root]` section
   270	- `src/main.rs` — `Boot` step that reads genesis, hashes Trust Root files, compares, aborts on mismatch with `TRUST_ROOT_TAMPERED` error
   271	- (new) `tests/trust_root_immutability.rs` — simulate ArchitectAI write attempt to each Trust Root path → assert process panics or returns EPERM (per PREREG § 7 Gate B fallback enforcement)
   272	
   273	**`pput_accounting_0` section content**:
   274	```toml
   275	[pput_accounting_0]
   276	schema_version = "1.0"
   277	progress_definition = "1 iff Lean ground-truth verifies golden_path_payload"
   278	cost_definition = "sum(prompt_tokens + completion_tokens + tool_tokens) over all proposals in the run"
   279	time_definition = "wall_clock from first agent prompt construction to final Lean accept or external timeout"
   280	verified_predicate = "experiments/minif2f_v4/src/lean4_oracle.rs::verify_omega_detailed"
   281	heldout_sealed_hash = "51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b"
   282	source_pool_sha256 = "77179cf2598b0df707d78a6663e763121dfe8e73a6538073155f488feab95093"
   283	baseline_regression_rate = 0.0  # PLACEHOLDER until Phase B7-extra calibration
   284	baseline_regression_jsonl_sha256 = ""  # PLACEHOLDER until Phase B7-extra
   285	k_max = 10
   286	n_max = 34
   287	
   288	[trust_root]
   289	"src/kernel.rs" = "<sha256>"
   290	"src/wal.rs" = "<sha256>"
   291	"src/bus.rs" = "<sha256>"
   292	"experiments/minif2f_v4/src/lean4_oracle.rs" = "<sha256>"
   293	"constitution.md" = "<sha256>"
   294	"handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json" = "<sha256>"
   295	"handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" = "<sha256>"
   296	# cases/* tracked via cases/MANIFEST.sha256 (glob)
   297	```
   298	
   299	**Acceptance criteria**:
   300	- `cargo test test_trust_root_immutable_at_boot` PASS
   301	- `cargo test test_trust_root_simulated_write_aborts` PASS
   302	- Boot cold-start with intact files = OK; with tampered file = aborts with `TRUST_ROOT_TAMPERED`
   303	
   304	**Estimated effort**: 1 day
   305	
   306	---
   307	
   308	### B7-extra — p_0 calibration (data + freeze)
   309	
   310	**What**: per PREREG § 5.5, run the calibration protocol to compute `p_0` (baseline regression rate); freeze into `genesis_payload.toml [pput_accounting_0]`.
   311	
   312	**Pre-requisite**: B7 + the `--simulate-rollback-at-tx-50` toggle implemented in evaluator.
   313	
   314	**Toggle implementation**:
   315	- `experiments/minif2f_v4/src/bin/evaluator.rs` — add `--simulate-rollback-at-tx-50` flag. At tx 50, force `Tape::rollback_to(snapshot_at_tx_25)` regardless of state. Then continue normal loop.
   316	
   317	**Calibration runs**:
   318	- Run `evaluator --mode full` on adaptation-144 × 2 seeds [31415, 2718] = 288 runs (control)
   319	- Run `evaluator --mode full --simulate-rollback-at-tx-50` on same 288 = treatment
   320	- Total: 576 runs, ~50s each chat oneshot, ~8 wall-hours, ~$3-5 API spend
   321	
   322	**Compute p_0**:
   323	- For each (problem, seed): regression_p = 1 iff control SOLVED AND treatment UNSOLVED
   324	- Per-problem regression: max over the 2 seeds (worst case per PREREG § 5.5)
   325	- p_0 = sum_p regression_p / 144
   326	
   327	**Sanity gate**: if p_0 > 0.10, ABORT — toggle too aggressive (per PREREG § 5.5 ceiling). Redesign rollback simulation, redo.
   328	
   329	**Freeze**:
   330	- Write p_0 value to `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
   331	- Compute SHA-256 of the calibration jsonl file → write to `[pput_accounting_0].baseline_regression_jsonl_sha256`
   332	- Add the calibration jsonl path to `[trust_root]`
   333	
   334	**Acceptance criteria**:
   335	- 288 control + 288 treatment runs completed; jsonl committed
   336	- p_0 ∈ (0, 0.10]
   337	- Trust Root re-hashed; Boot re-verifies
   338	- Dual-audit packet for Phase B → C transition includes the p_0 result
   339	
   340	**Estimated effort**: 1-2 days wall-clock (mostly LLM call time, can run unattended overnight)
   341	
   342	---
   343	
   344	## Phase B execution order (dependency graph)
   345	
   346	```
   347	B1 (schema)  → B2 (cost)  ┐
   348	                          ├→ B4 (verified vs runtime PPUT) → B5 (conformance) → Gate B
   349	B3 (wall-time)  ──────────┘                                 ↓
   350	                                                        B6 (context leak)
   351	B7 (Trust Root + Boot freeze) — depends on B1-B6 metadata to know what to lock
   352	                            ↓
   353	                       B7-extra (p_0 calibration) — runs after toggle code (part of B7)
   354	```
   355	
   356	Concretely:
   357	- Day 1 (post-A5): B1 schema + B3 wall-time (parallel, independent)
   358	- Day 2: B2 cost aggregator
   359	- Day 3: B4 verified-vs-runtime separation
   360	- Day 4: B5 conformance battery
   361	- Day 5: B6 context-leak audit + B7 part 1 (genesis_payload.toml structure)
   362	- Day 6: B7 part 2 (Boot integration + immutability tests) + commit toggle for B7-extra
   363	- Day 7: B7-extra p_0 calibration runs (overnight) + Phase B → C audit packet
   364	
   365	## Phase B exit checklist (Gate B)
   366	
   367	- [ ] All B1-B7 + B7-extra acceptance criteria met
   368	- [ ] `cargo test` passes (full suite)
   369	- [ ] 11 anti-Goodhart conformance tests PASS
   370	- [ ] 5-layer sealing conformance tests PASS
   371	- [ ] 4 doc-content + 4 lookup-evasion meta-predicates PASS (with deliberate-violation fixtures)
   372	- [ ] Trust Root immutability tests PASS
   373	- [ ] `pput_accounting_0` block in `genesis_payload.toml` filled (including p_0 + jsonl hash + Trust Root hashes)
   374	- [ ] On hard-10 adaptation × 1 seed, any single run can self-consistently report (verified golden path? total_tokens? wall_time? VPPUT?)
   375	- [ ] CHECKPOINT_PHASE_B_2026-05-*.md document with the sanity-run output committed
   376	- [ ] Notepad updated with F-2026-05-*-XX entry summarizing Phase B results
   377	- [ ] LATEST.md updated with Phase B done / Phase C ready
   378	
   379	## Open implementation questions (to be resolved during Phase B; not blocking start)
   380	
   381	1. **Token counter source of truth**: should we use the LLM API's reported token count (post-hoc, accurate), or pre-call estimation (real-time)? Default: post-hoc API-reported, since C_i is for accounting not budgeting.
   382	2. **`tool_tokens` granularity**: count chars/4 as token approximation, or actually run a tokenizer? Default: chars/4 approximation (consistent with PREREG § 1.2 spirit; precision not load-bearing).
   383	3. **`--mode` flag implementation order**: implement in B5 alongside conformance, or defer to Phase C start? Default: B5 (so the binary purity test can be run as part of Gate B).
   384	4. **`p_0` ceiling 0.10 — what if 0/144 = exactly 0**: this passes the ceiling but means the rollback simulation has no effect, which would invalidate j-RR's role. If observed p_0 = 0, redesign the rollback simulation toggle (probably `--simulate-rollback-at-tx-25` is too late; try tx-10 or per-call corruption).
   385	
   386	## Resources & references
   387	
   388	- `STEP_B_PROTOCOL.md` — restricted-file change protocol; **most B work is plumbing, not behavior change to bus.rs/kernel.rs/wal.rs (those are Trust Root and only modified for hash registration)**. STEP_B_PROTOCOL applies if any actual behavior change to those files becomes necessary.
   389	- `feedback_smoke_before_batch.md` — must smoke-test before B7-extra calibration batch starts
   390	- `feedback_phased_checkpoint.md` — CHECKPOINT_PHASE_B doc + 7 red-line check + auto-pause at Gate B
   391	- `feedback_dual_audit.md` — Phase B → C transition gets Phase C's dual-audit packet (per PREREG § 6 Phase C C4)

exec
/bin/bash -lc "nl -ba handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md | sed -n '35,60p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    35	```
    36	C_i = sum over all messages m in task i of m.tokens_total
    37	    where messages include:
    38	      - every agent prompt + completion (prompt_tokens + completion_tokens)
    39	      - every tool call's stdout (tool_tokens, hashed and length-summed)
    40	      - every retry and failed branch
    41	      - every architect/auditor meta-loop call attributed to task i
    42	      - every proxy / middleware augmentation
    43	```
    44	
    45	C_i is NOT restricted to the golden-path payload. Failed-branch tokens MUST be counted. Goal: a system that wastes 1M tokens to find a 1k-token proof has lower PPUT than one that finds the same proof in 100k tokens.
    46	
    47	### 1.3 Time T_i (wall-clock, end-to-end)
    48	
    49	```
    50	T_i = end_time - start_time   (seconds)
    51	   start_time = first read of the task statement by any agent in the run
    52	   end_time   = final ground-truth accept (Lean PASS) OR external timeout
    53	```
    54	
    55	Lean verification time is included (it is a real cost). Architect/auditor meta-loop time is included.
    56	
    57	### 1.4 Verified PPUT
    58	
    59	```
    60	VPPUT_i = Progress_i / (C_i × T_i)

exec
/bin/bash -lc "nl -ba handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md | sed -n '132,170p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   132	### 1.8 Trust Root composition (Patch B from Gemini DeepThink 2026-04-26)
   133	
   134	The Trust Root is the set of artifacts that **only humans + Boot may write**. ArchitectAI / AuditorAI / any swarm agent attempting to mutate Trust Root entries triggers **runtime SIGKILL-level abort** (not an audit-time warning):
   135	
   136	```
   137	Trust Root := {
   138	  src/kernel.rs,                    -- bus / kernel core
   139	  src/wal.rs,                       -- write-ahead log
   140	  src/bus.rs,                       -- predicate registration / capability gates
   141	  experiments/minif2f_v4/src/lean4_oracle.rs,  -- ground-truth predicate
   142	  constitution.md,                  -- normative spec
   143	  cases/C-*.yaml,                   -- common-law precedent (append-only by human)
   144	  genesis_payload.toml [pput_accounting_0],   -- PPUT accounting rules
   145	  handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json,  -- frozen splits
   146	  handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md     -- this file
   147	}
   148	```
   149	
   150	Enforcement mechanism (Phase B7 work item):
   151	1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
   152	2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
   153	3. ArchitectAI write tool whitelists `user_space/Δ_*/`; any path outside whitelist returns `EPERM` from the syscall layer (not a soft refusal at the LLM prompt layer).
   154	4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.
   155	
   156	Heldout sealed hash specifically: `sha256(sorted_newline_joined(heldout_problem_ids))`. This hash is committed to `genesis_payload.toml`; any tampering with the heldout list (adding / removing / re-ordering) detected at Boot.
   157	
   158	This is the **software floor** of the eventual hardware Trust Root (Open Claw → FPGA path); software immutability today, hardware immutability later.
   159	
   160	## 2. three-split protocol (irreversible)
   161	
   162	### 2.1 Source pool
   163	
   164	MiniF2F/Test split: 244 problems in `/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/*.lean`. MiniF2F/Valid (also 244 problems) is reserved as out-of-distribution heldout for future arcs and NOT part of this arc's three-split.
   165	
   166	**Source pool SHA-256 (canonical = sorted IDs newline-joined, no trailing newline)**: `77179cf2598b0df707d78a6663e763121dfe8e73a6538073155f488feab95093`
   167	
   168	This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.
   169	
   170	### 2.2 Split rule

exec
/bin/bash -lc "nl -ba handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md | sed -n '390,410p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   390	
   391	For each pair (Full, Mode_alt) where Mode_alt ∈ {Soft Law, Panopticon, Amnesia, Homogeneous}:
   392	
   393	| ID | Hypothesis (one-sided directional) | Primary endpoint | Direction |
   394	|---|---|---|---|
   395	| H1 | Soft Law has lower verified-PPUT than Full | McNemar paired sign test on n=10 per-problem paired-binary outcomes: `1 if mean(VPPUT_Full)_p > mean(VPPUT_SoftLaw)_p else 0` (mean over 2 seeds) | Full > Soft Law |
   396	| H2 | Panopticon has lower H-VPPUT than Full | Same construction, Full vs Panopticon | Full > Panopticon |
   397	| H3 | Amnesia has lower H-VPPUT than Full | Same construction, Full vs Amnesia | Full > Amnesia |
   398	| H4 | Homogeneous swarm has lower H-VPPUT than Full | Same construction, Full vs Homogeneous | Full > Homogeneous |
   399	
   400	**Test mechanics**: for each problem p in hard-10, average VPPUT over 2 seeds per mode → compare per-problem means → produce paired-binary (`1` if Full mean > Alt mean, `0` otherwise) → McNemar one-sided exact binomial on the 10 paired-binary observations.
   401	
   402	**Descriptive secondary endpoints** (logged, NOT in inferential family, NO α correction): mode-level CPR, mode-level IAC, mode-level FAR, gap between pput_runtime and pput_verified for Soft Law, ERR for Amnesia, mean prompt length per mode.
   403	
   404	### 5.3 Phase E inferential hypotheses (3 per artifact)
   405	
   406	For each artifact j ∈ {1, ..., k} (k ≤ k_max = 10) entering Phase E with `ArtifactState = Quarantined`, **three** primary inferential tests run on heldout-54 (NOT four — j-RR is a descriptive guardrail per § 5.4):
   407	
   408	| ID family | Hypothesis (one-sided) | Primary endpoint (n=54) | Direction |
   409	|---|---|---|---|
   410	| `j-PPUT` | Artifact j increases held-out VPPUT | Sign test on n=54 per-problem signs of `mean(VPPUT_all_p) − mean(VPPUT_minus_j_p)` (mean over 3 seeds; leave-one-out per § 6 E2) | sign > 0 |

exec
/bin/bash -lc "nl -ba handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md | sed -n '500,585p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   500	### Phase B — Kernel instrumentation + PPUT accounting (days 4-10)
   501	- B1 JSONL schema v2 (proposal-level + run-level fields per architect § 14)
   502	- B2 `C_i` full-cost aggregator (all agents × branches × failures × tool stdout)
   503	- B3 `T_i` wall-clock = first-read → final-accept incl. Lean verify
   504	- B4 `pput_verified` vs `pput_runtime` dual-field separation
   505	- B5 10-test anti-Goodhart conformance battery (§ 3) PASSING
   506	- B6 PPUT-context-leak audit: grep evaluator.rs / prompt.rs / sdk for any path that injects PPUT scalars into agent prompts; gate them
   507	- B7 Boot freeze: `genesis_payload.toml` adds `pput_accounting_0` block (Progress definition / Cost scope / heldout sealed hash). Frozen — any change triggers Phase Z' 6-stage re-audit per C-069.
   508	- **Gate B**: on hard-10 adaptation subset, any single run can self-consistently report (verified golden path? total_tokens? wall_time? VPPUT?). All 10 anti-Goodhart tests PASS. Heldout never touched.
   509	
   510	### Phase C — Ablation smoke tests (days 11-17)
   511	- C1 **5 modes implemented as a single `--mode` CLI flag on one binary** (Patch E from Gemini DeepThink): `evaluator --mode {full, panopticon, amnesia, soft_law, homogeneous}`. Modes change runtime behavior; binary bytes do not change between modes.
   512	- C2 **hard-10 adaptation × 2 random Boltzmann routing seeds; independent unit = per-problem (seeds = noise-reducing replicates), not (problem, seed)** (round 3 fix per Codex round-2 R3-1 — within-problem seed correlation). Hard-10 sample is drawn from the adaptation set (144 problems) using the same hash-based deterministic procedure as the three-split: `random.Random("hard10_pput_ccl_seed").sample(adaptation_set, 10)`. The 10 problem IDs + their fingerprint are pre-committed in Phase A5. Boltzmann seeds = `[31415, 2718]`. Total per-mode runs: 5 modes × 10 problems × 2 seeds = 100 jsonl rows. McNemar per condition pair uses **n = 10 per-problem paired-binary observations** (paired sign of `mean(VPPUT_seed1, VPPUT_seed2)_full > mean(VPPUT_seed1, VPPUT_seed2)_alt`). See § 9.2 + § 9.5 for power analysis.
   513	- C3 verify H1-H4 (§ 5) on PPUT axis
   514	- C4 CHECKPOINT_PHASE_C document + 7-redline check + dual external audit
   515	- C5 **Mode-purity conformance**: `tests/mode_flag_binary_purity.rs` asserts that running all 5 modes on the same problem produces jsonl with **identical `git_sha`, `binary_sha256`, and `model_snapshot` fields** — only the `mode` field differs. Any drift = BLOCKER (rules out "Soft Law happened to use a different binary" confound).
   516	- **Gate C**: H1-H4 each pass at Bonferroni α (§ 9); IF any H1-H4 fails, diagnose root cause, do NOT enlarge N (per feedback_phased_checkpoint). Heldout never touched.
   517	
   518	### Phase D — CCL shadow mode (days 18-24)
   519	- D1 ArchitectAI role: shadow only; reads adaptation L_t **(ArchitectAI is the SOLE reader of L_t, see D4)**; produces a distilled summary + candidate artifact Δ in human-readable form; proposes user-space artifacts (skill rules, broadcast rules, retrievable docs, tools).
   520	- D2 AuditorAI role: runs **§ 3.5 artifact content meta-predicates** + structural checks (rollbackable? schema-conformant? non-Trust-Root path?) on candidate Δ. **AuditorAI receives Δ + ArchitectAI distillation summary ONLY; never raw L_t** (Patch D — cognitive isolation, prevents Generator-Evaluator collapse per C-010).
   521	- D3 Per-artifact JSONL row (architect § 14 schema): artifact_id, state ∈ {Accepted | Quarantined | Certified | Reverted} (per § 1.7), source_log_hashes, generated_by, audited_by, meta_predicate_results (§ 3 + § 3.5 results), estimated_pput_gain_prior, actual_pput_gain_meta_val, used_count, rollbackable
   522	- D4 **Cognitive isolation conformance** (Patch D upgrade):
   523	  - `tests/architect_sole_lt_reader.rs`: greps src/ + experiments/ + bin/ for any reader of `L_t / failure_log / rejection_trace` paths; whitelist contains exactly **`bin/architect_distill.rs`**. Any other reader = BLOCKER.
   524	  - `tests/auditor_sees_candidate_only.rs`: AuditorAI's input parameters at call site exclude raw L_t; only `(candidate_artifact, architect_summary)` accepted.
   525	- D5 **Human-as-live-meta-predicate (Patch — confirmed by审查官 2026-04-26)**: Phase D has no automated `Quarantined → Certified` promotion. User (`gretjia`) reviews each `Accepted` candidate Δ in human-readable form (≤1 min per artifact), issues `keep | drop | revise` verdict. Approved Δ enters `Quarantined` for meta_val measurement; user keystroke logs become Phase E's golden Trust Data for training Phase E auto-meta-predicates. Defense-in-depth: prevents Claude-only homogeneous LLM stack from establishing a covert promotion-loop (Art. V + C-010 protection).
   526	  
   527	  **D5 SPOF contingency (round 2 per dual audit M5)**:
   528	  - Per-candidate review SLA: **48 hours wall-clock** from `Accepted` → user verdict.
   529	  - SLA breach: candidate auto-moves to `deferred` queue. Phase D continues processing other candidates; deferred items revisited if user returns within Phase D window.
   530	  - **30-day arc clock does NOT pause for user unavailability**. User unavailability = lost Phase D throughput, accepted operational risk; arc continues on schedule.
   531	  - **Hard abort condition**: if ≥ 5 candidates queue > 48h simultaneously, abort Phase D early and proceed to Phase E with whatever artifacts are already `Quarantined`. This prevents Phase D from blocking the entire arc on user availability.
   532	  - All `keep | drop | revise` verdicts logged with timestamp + (optional) one-line user reasoning to `handover/ccl/human_meta_predicate_log_2026-04-*.jsonl` for Phase E auto-meta-predicate training.
   533	- **Gate D**: at least one candidate artifact reaches `Accepted` AND user-approved AND shows ΔPPUT > 0 on meta_val (i.e. promotes to `Quarantined` and survives meta_val window). NO auto-merge. NO `Certified` state achievable in Phase D — only Phase E heldout audit can mint Certified. Heldout never touched.
   534	
   535	### Phase E — Controlled activation + heldout sealed eval (days 25-30)
   536	
   537	**Single-touch discipline**: heldout-54 problem set is touched ONCE for an evaluation campaign. Within that single touch, the sealed eval runs `k+2` sub-evaluations on the SAME problem set; this is not multiple independent draws — it is a leave-one-out factorial design within one sample (round 2 per Codex audit M3).
   538	
   539	- E1 Auto-loop: ArchitectAI → AuditorAI → meta-predicate gate → user-space write (no kernel writes; rollbackable artifact directory `user_space/Δ_*/`).
   540	- E2 **Sealed heldout evaluation campaign — leave-one-out within sealed eval (revised round 2)**:
   541	  - Pre-committed seeds: `[20260426, 31415, 2718]` (3 Boltzmann routing seeds; same 3 seeds used for every sub-evaluation below for paired comparison).
   542	  - Let `Q = {q_1, ..., q_k}` denote the set of `Quarantined` artifacts entering Phase E from Phase D.
   543	  - **Sub-eval 0 (BASELINE)**: heldout-54 × 3 seeds with NO Phase D artifacts active. Produces `PPUT_baseline`.
   544	  - **Sub-eval ALL**: heldout-54 × 3 seeds with ALL of Q active simultaneously. Produces `PPUT_all`.
   545	  - **Sub-evals LOO (leave-one-out)**: for j ∈ 1..k, run heldout-54 × 3 seeds with `Q \ {q_j}` active (every artifact EXCEPT q_j). Produces `PPUT_minus_j`.
   546	  - Total sub-evaluations: `k + 2`. All on the SAME 54 problems × 3 seeds. The 54 problems are not redrawn between sub-evals — single-touch discipline is preserved.
   547	  - Per-artifact ΔPPUT: `Δ_j = PPUT_all − PPUT_minus_j` (the marginal contribution of q_j given the rest).
   548	  - Same logic for ΔFAR, ΔCPR. RR_j is computed by comparing per-problem solved/unsolved status with q_j active (ALL run) vs without q_j (LOO_j).
   549	- E3 Statistical inference per § 5.3 + § 9. For each j, evaluate the **three** primary inferential tests (j-PPUT, j-FAR, j-CPR) under Holm-Bonferroni at family `4 + 3k`, N_max = 34. j-RR is a separate descriptive guardrail check (point estimate ≤ p_0 per § 5.4), NOT in the inferential family.
   550	- E4 Final dual external audit (Codex + Gemini); conservative VETO > CHALLENGE > PASS.
   551	
   552	**No iteration on heldout result**: results compute once, reported once. If round-1 Phase E reveals the auto-loop is broken (e.g., 0 of k artifacts pass any test), the arc is declared a NEGATIVE finding. Re-running Phase E with a fresh draw requires a NEW pre-registration AND a new split (heldout = burned).
   553	
   554	**Compute budget for E2**: `54 problems × 3 seeds × (k + 2)`. With k ≤ 10 (a realistic Phase D throughput), that's at most `54 × 3 × 12 = 1944` problem runs. At ~50s per run (chat oneshot tier), ~27 wall-hours. Fits within Phase E's 6-day window with ~5x headroom.
   555	
   556	## 7. FINAL PASS gates (Gates A-H, conjunctive — ALL must hold)
   557	
   558	| Gate | Condition |
   559	|---|---|
   560	| A | AuditorAI does not have final merge authority — only meta-predicate output; final accept = Lean ground truth (for tasks) + dual external audit (for arc) + human sudo (for artifact promotion in Phase D) |
   561	| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
   562	| C | WBCG_PPUT measured on heldout, not on adaptation or meta_val |
   563	| D | Raw L_t never reaches AuditorAI or general agent context (asserted by D4 cognitive-isolation conformance: `architect_sole_lt_reader.rs` + `auditor_sees_candidate_only.rs`) |
   564	| E | Three splits enforced; heldout touched at most once (E2) |
   565	| F | All failed branches counted in every C_i (asserted by anti-Goodhart test_failed_branches_in_total_cost) |
   566	| G | Progress = 1 only iff GroundTruth(G_i) = 1 (Lean) |
   567	| H | **Heldout WBCG_PPUT > 0 with ArtifactState = Certified gate** (round 4 — j-RR moved out of inferential family per Codex round-3): at least one ArchitectAI-generated user-space artifact Δ has `ArtifactState(Δ) = Certified` per § 1.7 — meaning (i) was Accepted (passed § 3 + § 3.5 AuditorAI battery), (ii) was Quarantined (user-approved + ΔPPUT_meta_val > 0 + N_use ≥ 3 on meta_val), (iii) at heldout sealed eval **three** primary inferential tests reject null at family-corrected α (per § 5.3 + § 9): **j-PPUT** (sign test on n=54), **j-FAR** (non-inferiority sign test), **j-CPR** (non-inferiority sign test), AND (iv) **j-RR ≤ p_0 point check guardrail** (per § 5.4 — descriptive, NOT inferential, NO α correction), AND (v) is rollbackable (artifact directory deletion at the protocol level restores prior heldout PPUT — verified by an explicit rollback sub-eval, see § 6 E2). Quarantined-only artifacts (failed any of the five conditions) do NOT count toward Gate H. |
   568	
   569	If any gate A-G fails: arc reported as negative finding, no FULL PASS, write CCL-1 negative paper.
   570	
   571	If H fails but A-G hold: arc reported as "infrastructure works, capability compilation not yet demonstrated" — also publishable as negative result. No claim of CCL.
   572	
   573	## 8. what would falsify
   574	
   575	Each of the following individually FALSIFIES the central thesis as stated:
   576	
   577	- F1: Phase C — Soft Law mode has VPPUT comparable to Full (within Bonferroni-corrected CI). Implies PPUT does not penalize fake-progress; metric is broken or mode does not actually violate constitution.
   578	- F2: Phase C — Panopticon and Amnesia modes both at VPPUT parity with Full. Implies CPR / ERR are not load-bearing; constitutional analysis rejected.
   579	- F3: Phase D — zero candidate artifacts pass meta-validation across full week. Implies CCL shadow loop cannot generate value; ArchitectAI is non-functional.
   580	- F4: Phase E — heldout WBCG_PPUT ≤ 0. Implies adaptation gains do not generalize; capability compilation fails.
   581	- F5: Any anti-Goodhart conformance test fails post-Phase B. Implies metric is exploitable; arc must restart with strengthened metering.
   582	
   583	## 9. multiplicity (CLEAN REWRITE round 4 — aligned with § 5)
   584	
   585	### 9.1 Family construction

exec
/bin/bash -lc "nl -ba CLAUDE.md | sed -n '1,40p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TuringOS v4
     2	
     3	## What
     4	Silicon-Native Microkernel for LLM Formal Verification Swarm.
     5	Rust 2021, tokio, serde_json. Mission: MiniF2F Lean 4.
     6	
     7	## Why
     8	- 唯一对齐文档: `constitution.md` (反奥利奥架构)
     9	- 压缩即智能: 抽象原则进宪法 / 具体情境进 `cases/`
    10	- 机制 > 参数 > 提示 (Art. V + C-021/C-031/C-034/C-043)
    11	
    12	## Code Standard (Art. I.1 + C-004 + C-027)
    13	- `cargo check` / `cargo test` 必过；`.env` 永不 commit
    14	- `src/{kernel,bus,wallet}.rs` 改动走 STEP_B_PROTOCOL（不直接编辑 main）
    15	- 任何影响行为的参数必须 env/config 可覆盖，不可硬编码
    16	
    17	## Audit Standard (Art. V.1 + C-010 + C-023 + C-035)
    18	- Generator ≠ Evaluator：代码作者不可是唯一审计者
    19	- 所有 merge / phase 决策双外审（Codex + Gemini）；VETO > CHALLENGE > PASS
    20	- 宪法违规立即 BLOCKER，不可延期、不可"可接受"
    21	
    22	## Report Standard (Art. I.2 + Art. II.2.1 + Art. IV 强制, C-052 + C-053 + C-057 + C-059 + C-061)
    23	- **主指标**（每报必填）: ΣPPUT + Mean PPUT (solved) + 95% CI (Wilson)
    24	- Art. I.2 三大统计信号不可缺: **信誉** (reputation_distribution p50/p90/max) + 效用 (PPUT) + 共识 (如适用)
    25	- Art. IV 终态区分: `halt_reason_distribution` {OmegaAccepted, MaxTxExhausted, WallClockCap, ComputeCapViolated, ErrorHalt}
    26	- 多 agent (n≥2) 专用: `parent_selection_entropy` + `pairwise_payload_diversity_mean`；任一 < 0.25 = Art. II.2.1 告警
    27	- solve count 不可独立陈述，必须配对 PPUT；以 solve count 起头 = 违宪
    28	
    29	## Reproducibility Standard (Art. I + C-012/C-016/C-032/C-039)
    30	- OMEGA accept 必留 self-contained artifact (`proofs/*.lean` + `gp_payload`)
    31	- 度量工具上线即冻结；Oracle 参数冻结；实验禁混 Oracle 模式
    32	- 中间件若修改数学内容 → 是 ArchitectAI 贡献，不是 swarm 涌现（C-023）
    33	
    34	## Alignment Standard (Art. IV + C-069)
    35	- 权威对齐文件: `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md`
    36	  (后续 rev: `TRACE_MATRIX_vN.md`)
    37	- 每个 src/ pub 符号必须映射到宪法 flowchart 元素、标 orphan+justification、
    38	  或 BLOCK merge。doc-comment backlink 格式: `/// TRACE_MATRIX <FC-id>: <role>`
    39	- Conformance tests: `tests/fc_alignment_conformance.rs` — 每个 ✅ 行 ≥1
    40	  witness test；`#[ignore]` stub 覆盖 📅 deferred rows

exec
/bin/bash -lc 'nl -ba rules/active/R-006_kernel_modification.yaml && nl -ba rules/active/R-007_bus_lifecycle.yaml && nl -ba rules/active/R-002_coin_conservation.yaml 2>/dev/null || true' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	id: "R-006"
     2	name: "kernel_modification"
     3	source_incidents: []
     4	axiom: "kernel.rs is pure topology layer — modifications need user confirmation + audit"
     5	trigger: "pre_edit"
     6	check:
     7	  type: "grep"
     8	  pattern: ".*"
     9	file_glob: "src/kernel.rs"
    10	enforcement: "warn"
    11	message: "WARNING: kernel.rs 是纯拓扑层, 修改需要用户确认 + kernel-auditor 审计"
    12	stats:
    13	  times_triggered: 8
    14	  last_triggered: "2026-04-06T15:14:12Z"
     1	id: "R-007"
     2	name: "bus_lifecycle"
     3	source_incidents: []
     4	axiom: "Bus SKILL lifecycle is critical infrastructure — changes need review"
     5	trigger: "pre_edit"
     6	check:
     7	  type: "grep"
     8	  pattern: "Skill|lifecycle|boot|init|pre_append|post_append|halt"
     9	file_glob: "src/bus.rs"
    10	enforcement: "warn"
    11	message: "WARNING: bus.rs SKILL lifecycle modification detected — review impact on agent boot/halt sequence"
    12	stats:
    13	  times_triggered: 3
    14	  last_triggered: "2026-04-11T16:13:50Z"

exec
/bin/bash -lc "nl -ba handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md | sed -n '57,90p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    57	### 1.4 Verified PPUT
    58	
    59	```
    60	VPPUT_i = Progress_i / (C_i × T_i)
    61	```
    62	
    63	If `Progress_i = 0`: `VPPUT_i = 0` regardless of how much was spent.
    64	
    65	For dashboard readability we report scaled values:
    66	```
    67	PPUT-M_i = 10^6 × VPPUT_i    (per million token-second)
    68	PPUT-B_i = 10^9 × VPPUT_i    (per billion token-second)
    69	```
    70	Raw `VPPUT_i` retained in jsonl for paper / audit. PPUT-M is the dashboard default.
    71	
    72	### 1.5 Held-out Verified PPUT (NORTH STAR)
    73	
    74	```
    75	H-VPPUT = (sum_{i in heldout_set} Progress_i) / (sum_{i in heldout_set} C_i × T_i)
    76	```
    77	
    78	This is the sole optimization signal. All other dashboard metrics are guardrails or diagnostics.
    79	
    80	### 1.6 Auxiliary metrics (frozen list)
    81	
    82	| Metric | Definition | Role |
    83	|---|---|---|
    84	| FAR | False Accept Rate: runtime accepts that Lean rejects post-hoc / total runtime accepts | Safety guardrail |
    85	| RR  | Regression Rate: tasks SOLVED at baseline that become UNSOLVED after a meta-change | Backwards-compat guardrail |
    86	| ERR | External Recovery Rate: post-rollback retry success rate on tasks that previously hit recoverable failure | Tape / Q^world hygiene |
    87	| CPR | Context Pollution Rate: bytes of rejected-trace text observed in agent prompts / total prompt bytes | Info hygiene |
    88	| IAC | Inter-Agent Correlation: pairwise payload Jaccard mean across agents in the same branch | Diversity (Art. II.2.1) |
    89	| Generalization Gap | (PPUT_adaptation − PPUT_meta_val) on a fixed model snapshot | Adaptation overfit detector |
    90	| AH  | Autonomy Hours: continuous wall-clock without human sudo | Long-run autonomy |

exec
/bin/bash -lc 'rg -n "anti-Goodhart|test_failed_branches|golden_path_requires|wall_clock|Failed-branch|GroundTruth|Progress|C_i|T_i|soft law|Soft Law" handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:5:**Gate B exit criterion**: on hard-10 adaptation subset, any single run can self-consistently report (verified golden path? total_tokens? wall_time? VPPUT?). All 11 anti-Goodhart conformance tests + 5-layer sealing + 4 content-meta-predicates + 4 lookup-evasion + Trust Root immutability PASS. Heldout never touched.
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:69:    total_run_token_count: u64,              // C_i (sum over all proposals)
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:70:    total_wall_time_ms: u64,                 // T_i
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:73:    pput_verified: f64,                      // Progress / (C_i × T_i / 1000)  [token-second]
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:99:### B2 — C_i full-cost aggregator
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:129:- `cargo test test_failed_branches_counted_in_total_cost` (anti-Goodhart conformance per PREREG § 3): synthesize a run with 5 failed proposals + 1 success → assert `total_run_token_count == sum(all 6 proposals)`.
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:130:- Manual sanity: spot-check 3 historical Phase 1 runs; recompute C_i, ensure jsonl emits match.
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:136:### B3 — T_i wall-clock instrumentation
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:138:**What**: T_i = end_time − start_time, where:
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:142:Currently: T_i is approximated as evaluator wall-clock from main loop start to OMEGA accept; but Lean verify time is sometimes excluded.
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:149:- `cargo test test_wall_clock_first_read_to_final_accept` (anti-Goodhart per PREREG § 3): synthetic run with 100ms prompt construction + 5s LLM call + 2s Lean verify → assert `wall_time_ms ≥ 7100`.
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:150:- Manual: compare new T_i to legacy `total_run_time` for 3 historical runs; new should be ≥ legacy (wider bracket).
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:159:- `pput_runtime` = Progress_runtime / (C_i × T_i)  [where Progress_runtime = 1 if runtime/evaluator accepted]
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:160:- `pput_verified` = Progress_verified / (C_i × T_i)  [where Progress_verified = 1 if Lean post-hoc verifies the golden path]
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:162:For Soft Law mode (which fakes runtime acceptance), `pput_runtime` may inflate but `pput_verified` should drop to 0. This is the H1 detection mechanism.
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:169:- `cargo test test_pput_verified_zero_when_lean_rejects` (PREREG § 3 `test_golden_path_requires_ground_truth_acceptance`): synthesize a run that records runtime accept + Lean reject → assert `progress = 0`, `pput_verified = 0.0`.
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:170:- For Phase C ablation Soft Law mode: confirm `pput_runtime > 0` but `pput_verified` reflects Lean truth.
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:176:### B5 — Conformance test battery (anti-Goodhart × 11 + sealing × 5-layer + content × 4 + lookup-evasion × 4)
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:198:test_golden_path_requires_ground_truth
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:199:test_failed_branches_in_total_cost
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:200:test_wall_clock_first_read_to_final_accept
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:279:time_definition = "wall_clock from first agent prompt construction to final Lean accept or external timeout"
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:369:- [ ] 11 anti-Goodhart conformance tests PASS
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:381:1. **Token counter source of truth**: should we use the LLM API's reported token count (post-hoc, accurate), or pre-call estimation (real-time)? Default: post-hoc API-reported, since C_i is for accounting not budgeting.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:23:### 1.1 Progress
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:27:Progress_i = 1   iff   GroundTruth(G_i) = 1   (Lean 4 toolchain verifies the golden-path proof)
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:28:Progress_i = 0   otherwise
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:31:`GroundTruth` is the Lean oracle in `experiments/minif2f_v4/lean4_oracle.rs` enforced via `check_payload` (forbidden patterns: `sorry`, `admit`, `native_decide`, `decide`, `omega` — per C-011 corollary in F-2026-04-20-05). LLM-judge verdicts and runtime "accepted" markers are NOT GroundTruth.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:33:### 1.2 Cost C_i (full physical cost)
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:36:C_i = sum over all messages m in task i of m.tokens_total
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:45:C_i is NOT restricted to the golden-path payload. Failed-branch tokens MUST be counted. Goal: a system that wastes 1M tokens to find a 1k-token proof has lower PPUT than one that finds the same proof in 100k tokens.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:47:### 1.3 Time T_i (wall-clock, end-to-end)
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:50:T_i = end_time - start_time   (seconds)
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:60:VPPUT_i = Progress_i / (C_i × T_i)
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:63:If `Progress_i = 0`: `VPPUT_i = 0` regardless of how much was spent.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:67:PPUT-M_i = 10^6 × VPPUT_i    (per million token-second)
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:68:PPUT-B_i = 10^9 × VPPUT_i    (per billion token-second)
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:70:Raw `VPPUT_i` retained in jsonl for paper / audit. PPUT-M is the dashboard default.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:75:H-VPPUT = (sum_{i in heldout_set} Progress_i) / (sum_{i in heldout_set} C_i × T_i)
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:128:- **Task** Progress stays binary (`Lean accepts ⇒ 1`, else `0`); 4-state machine applies ONLY to artifacts. Mixing the two would corrupt PPUT (Lean gives a perfect predicate; artifact certification is a non-perfect-predicate process — they must not be conflated).
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:268:## 3. anti-Goodhart guardrails (frozen)
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:279:test_golden_path_requires_ground_truth      — Progress = 1 iff Lean returns Pass on full proof
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:280:test_failed_branches_in_total_cost          — C_i unit test: 5 failed proposals + 1 success → C_i = sum(6)
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:281:test_wall_clock_first_read_to_final_accept  — T_i bracket includes Lean verify time
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:321:These 4 predicates are deliberately separate from § 3 (anti-Goodhart on metering) because they target a different attack surface: METERING gaming would let the system inflate PPUT numbers; CONTENT gaming would let ArchitectAI launder adaptation answers into doc form. Both surfaces must be defended.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:391:For each pair (Full, Mode_alt) where Mode_alt ∈ {Soft Law, Panopticon, Amnesia, Homogeneous}:
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:395:| H1 | Soft Law has lower verified-PPUT than Full | McNemar paired sign test on n=10 per-problem paired-binary outcomes: `1 if mean(VPPUT_Full)_p > mean(VPPUT_SoftLaw)_p else 0` (mean over 2 seeds) | Full > Soft Law |
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:402:**Descriptive secondary endpoints** (logged, NOT in inferential family, NO α correction): mode-level CPR, mode-level IAC, mode-level FAR, gap between pput_runtime and pput_verified for Soft Law, ERR for Amnesia, mean prompt length per mode.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:502:- B2 `C_i` full-cost aggregator (all agents × branches × failures × tool stdout)
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:503:- B3 `T_i` wall-clock = first-read → final-accept incl. Lean verify
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:505:- B5 10-test anti-Goodhart conformance battery (§ 3) PASSING
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:507:- B7 Boot freeze: `genesis_payload.toml` adds `pput_accounting_0` block (Progress definition / Cost scope / heldout sealed hash). Frozen — any change triggers Phase Z' 6-stage re-audit per C-069.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:508:- **Gate B**: on hard-10 adaptation subset, any single run can self-consistently report (verified golden path? total_tokens? wall_time? VPPUT?). All 10 anti-Goodhart tests PASS. Heldout never touched.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:515:- C5 **Mode-purity conformance**: `tests/mode_flag_binary_purity.rs` asserts that running all 5 modes on the same problem produces jsonl with **identical `git_sha`, `binary_sha256`, and `model_snapshot` fields** — only the `mode` field differs. Any drift = BLOCKER (rules out "Soft Law happened to use a different binary" confound).
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:565:| F | All failed branches counted in every C_i (asserted by anti-Goodhart test_failed_branches_in_total_cost) |
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:566:| G | Progress = 1 only iff GroundTruth(G_i) = 1 (Lean) |
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:577:- F1: Phase C — Soft Law mode has VPPUT comparable to Full (within Bonferroni-corrected CI). Implies PPUT does not penalize fake-progress; metric is broken or mode does not actually violate constitution.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:581:- F5: Any anti-Goodhart conformance test fails post-Phase B. Implies metric is exploitable; arc must restart with strengthened metering.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:633:- Phase C is powered for ablations that produce STRONG, CONSISTENT effects across all 10 hard problems. This is plausible for genuine constitutional violations (Soft Law / Panopticon / Amnesia should affect every adversarial-hard problem, not just some).
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:666:| "PPUT detects Soft Law / Panopticon / Amnesia constitutional violations" | H1-H3 pass at family-corrected α (§ 9) | "on deepseek-v4-flash thinking-off; replication on heterogeneous LLM stack deferred to future arc" |
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:717:| C | `deepseek-v4-flash` only (thinking off) | Ablation modes (Soft Law / Panopticon / Amnesia / Homogeneous / Full) must vary ONLY the constitutional axis; mixing models would confound mode-effect with model-effect |
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:722:**Why not earlier**: Phase C's hypotheses H1-H4 require a pure constitutional-axis comparison. If we mix LLMs in C, a "Soft Law mode loses VPPUT" finding could be re-attributed to "Soft Law mode happens to use weaker LLM in this batch". Heterogeneity is introduced once the mode-axis is sealed.
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:760:| 2026-04-26 | **Patch A** (Gemini DeepThink): § 1.7 ArtifactState 4-state machine for artifacts (Accepted / Quarantined / Certified / Reverted); § 1.6 WBCG_PPUT row + § 7 Gate H tightened to "Certified-only". Task Progress remains binary (Lean perfect predicate). | Gemini DeepThink 2026-04-26 PPUT-driven FULL PASS confirmation |
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:762:| 2026-04-26 | **Patch C**: § 3.5 4 doc/artifact content meta-predicates (no_raw_failed_trace / no_exact_adaptation_solution with hash check / parametric_templates / scope_and_expiration). Run by AuditorAI in Phase D. Distinct attack surface from § 3 anti-Goodhart metering. | same |
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:766:| 2026-04-26 (round 2) | **M1**: § 5 restructured — each H1-H4 has ONE primary endpoint (per-(problem,seed) Lean-verified Progress sign or paired VPPUT sign); H5 reclassified as deterministic gate (no α). § 5.2 added — per-artifact heldout family `4·k` tests (j-PPUT / j-FAR / j-RR / j-CPR). § 5.3 family size = `4 + 4k`. § 6 C2 independent unit clarified = (problem, seed). § 9 rewritten with Holm-Bonferroni stepwise procedure + power expectation + family construction frozen at A5. | Codex audit M1 + Gemini audit M1 (round 1 dual-CHALLENGE convergent) |

exec
/bin/bash -lc "ls -la STEP_B_PROTOCOL.md handover/ai-direct/STEP_B_PROTOCOL.md 2>/dev/null || true && find . -maxdepth 4 -name '*STEP*PROTOCOL*' -print" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
-rw-r--r-- 1 zephryj zephryj 7010 Apr 15 13:14 handover/ai-direct/STEP_B_PROTOCOL.md
./handover/ai-direct/STEP_B_PROTOCOL.md

exec
/bin/bash -lc "rg -n \"src/\\{kernel,bus,wallet\\}|STEP_B_PROTOCOL|drivers/llm_http|src/drivers|wallet.rs|src/.*wallet\" CLAUDE.md rules handover cases constitution.md docs -S" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
cases/V3_LESSONS.md:27:| V3L-10 | 钱包语义污染：balance=5990 报 "Bankrupt"，混淆真破产与保证金不足 | Art. II.1 | C-018 | sdk/tools/wallet.rs |
cases/V3_LESSONS.md:39:| V3L-17 | 截断悖论：2048 token 硬限截断推理链，挤压物理层而非能力层 | Art. III.2 | C-023 | drivers/llm_http.rs |
cases/V3_LESSONS.md:44:| V3L-22 | Falsifier 可买 YES token — Goodhart 漏洞，角色激励冲突 | Art. III.4 | C-006 | sdk/tools/wallet.rs |
cases/V3_LESSONS.md:52:| V3L-25 | reqwest+rustls macOS 死锁 (6 种方案均失败，改用 Python proxy) | Art. IV | C-007 | drivers/llm_http.rs |
cases/V3_LESSONS.md:54:| V3L-27 | SiliconFlow API N=30 → 401/429 崩溃 | Art. IV | C-027 | drivers/llm_http.rs |
cases/C-069_constitutional_alignment_audit_protocol.yaml:46:     架构性修改 (bus.rs / kernel.rs / wallet.rs / ledger.rs / sdk/*) 必须
cases/C-069_constitutional_alignment_audit_protocol.yaml:64:  - 全部 Rust src/ 修改（不限于 bus/kernel/wallet）
docs/architecture.md:16:| `sdk/tools/wallet.rs` | WalletTool — balance + YES/NO/LP portfolios |
docs/architecture.md:20:| `drivers/llm_http.rs` | Resilient HTTP client — multi-provider routing |
docs/architecture.md:27:| Pure Capital Engine | Invest-only economy (Law 2) | `sdk/tools/wallet.rs` + `prediction_market.rs` |
CLAUDE.md:14:- `src/{kernel,bus,wallet}.rs` 改动走 STEP_B_PROTOCOL（不直接编辑 main）
rules/enforcement.log:16:[2026-04-13 08:23:37] BLOCK R-002 | /home/zephryj/projects/turingosv4/src/sdk/tools/wallet.rs | BLOCKED: Coin minting / free balance detected — economy is invest-only (Law 2, V-001/V-002)
rules/enforcement.log:27:[2026-04-13 08:41:42] WARN R-008 | /home/zephryj/projects/turingosv4/src/sdk/tools/wallet.rs | WARNING: Market constant change detected — verify LP amounts and initial balances are intentional
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:80:*   **(Q6.a) STEP_B_PROTOCOL violation**: **PASS**. The diff touches `src/drivers/llm_http.rs`. The restricted file list (`bus.rs`, `kernel.rs`, `wallet.rs`) does not include the `drivers/` directory. No process violation occurred.
handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:86:- (Q6.a) STEP_B_PROTOCOL: bus.rs / kernel.rs / wallet.rs are restricted (require parallel-branch A/B, not direct edit). The B2-B4 diff touches `src/drivers/llm_http.rs`. Is `drivers/llm_http.rs` restricted? If yes, this is a process violation.
handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:133:printf '\n```\n\n---\n\n# Diff: evaluator.rs + lib.rs + drivers/llm_http.rs (B2/B3/B4 wiring)\n\n```diff\n' >> "$TMP_PROMPT"
handover/audits/EXT_GEMINI_2026-04-21.md:17:    1.  **铸币 (Minting)**: `src/sdk/tools/wallet.rs:141` (`on_init`)。这是唯一合法的铸币点，通过 `if self.genesis_done { return; }` 卫语句确保只执行一次。
handover/audits/EXT_GEMINI_2026-04-21.md:18:    2.  **借记 (Debit)**: `src/sdk/tools/wallet.rs:52` (`deduct`)，由 `src/bus.rs:410` (`debit_wallet`) 调用。
handover/audits/EXT_GEMINI_2026-04-21.md:19:    3.  **贷记 (Credit)**: `src/sdk/tools/wallet.rs:67` (`credit`)，由 `src/bus.rs:426` (`credit_wallet`) 调用。
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1865:**Protocol for restricted-file changes**: `STEP_B_PROTOCOL.md` (necessity audit → parallel branch → A/B statistical test → merge on empirical win only). Do NOT directly edit restricted files even with authorization; always A/B test.
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1869:| `recent_rejections` optional global scope | `src/bus.rs` | F-2026-04-15-02 Art. II.1 broadcast | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1871:| WAL emission | `src/kernel.rs` or bus.rs | F-2026-04-15-03 | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:90:- (Q6.a) STEP_B_PROTOCOL: bus.rs / kernel.rs / wallet.rs are restricted (require parallel-branch A/B, not direct edit). The B2-B4 diff touches `src/drivers/llm_http.rs`. Is `drivers/llm_http.rs` restricted? If yes, this is a process violation.
handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:156:         "src/drivers/llm_http.rs"],
handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:169:    "\n```\n\n---\n\n# Diff: evaluator.rs + lib.rs + drivers/llm_http.rs (B2/B3/B4 wiring)\n\n```diff\n" + diff_text +
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3936:**Protocol for restricted-file changes**: `STEP_B_PROTOCOL.md` (necessity audit → parallel branch → A/B statistical test → merge on empirical win only). Do NOT directly edit restricted files even with authorization; always A/B test.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3940:| `recent_rejections` optional global scope | `src/bus.rs` | F-2026-04-15-02 Art. II.1 broadcast | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3942:| WAL emission | `src/kernel.rs` or bus.rs | F-2026-04-15-03 | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/EXT_CODEX_2026-04-22.md:10:| Q4. Karpathy TOP-10 hot-path claim | CHALLENGE | `/home/zephryj/projects/turingosv4/src/bus.rs:231-252,416-424`; `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:679-718`; `/home/zephryj/projects/turingosv4/src/sdk/actor.rs:53-74,88-95` | One of the sampled items is plausibly hot (`trace_ancestors` via Boltzmann routing), but the `InvestOnly` and wallet-downcast items are materially overstated in current call paths. |
handover/audits/EXT_CODEX_2026-04-22.md:76:Claude's hot-path framing is not substantiated. The `InvestOnly` route exists in the bus (`/home/zephryj/projects/turingosv4/src/bus.rs:231-252`), but no current tool emits `ToolSignal::InvestOnly`; the only repo matches are the enum and this bus handler (`/home/zephryj/projects/turingosv4/src/sdk/tool.rs:15-22`, `/home/zephryj/projects/turingosv4/src/bus.rs:218-252`). Actual evaluator-side `invest` traffic bypasses that code and talks to wallet/kernel directly (`/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:679-718`). So the clone exists, but it is not “every invest” and not proven hot.
handover/audits/EXT_CODEX_2026-04-22.md:80:Again, the performance concern is overstated as phrased. `debit_wallet` and `credit_wallet` do perform repeated `manifest()` + `downcast_mut::<WalletTool>()` scans (`/home/zephryj/projects/turingosv4/src/bus.rs:416-437`), but those functions are not on every transaction. They are used on bus-side invest/refund/hayek/settlement paths (`/home/zephryj/projects/turingosv4/src/bus.rs:233,240,346,359,383-412`), while evaluator-side invest does its own separate wallet downcasts (`/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:694-716`). The deeper issue is architectural duplication and bus bypass, not the raw dynamic-dispatch cost alone.
handover/audits/EXT_CODEX_2026-04-22.md:113:- **Agents appear to be prompted with the wrong wallet state.** `bus.snapshot()` returns empty `balances` and `portfolios` (`/home/zephryj/projects/turingosv4/src/bus.rs:567-570`), while `UniverseSnapshot::get_balance()` simply reads that map (`/home/zephryj/projects/turingosv4/src/sdk/snapshot.rs:22-39`). The evaluator passes `snap.get_balance(agent_id)` into the prompt (`/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:492-495`), and the prompt displays it as `Balance: {:.0} Coins` (`/home/zephryj/projects/turingosv4/src/sdk/prompt.rs:78-84`). Unless another layer patches the snapshot out-of-band, agents are being told they have `0` balance even when the wallet holds funds. That distorts Art. II.2 economic signaling and makes investment behavior hard to interpret.
handover/audits/HARNESS_COMPRESSION_AUDIT_2026-04-22.md:26:| `kernel.rs / bus.rs / wallet.rs edits: human confirm` | 这是 R-006/R-007 的复述 + C-004/C-015 的重复 | 删除——rules/active/R-006/R-007 已是机器可执行形式，CLAUDE.md 不必再提文件名 |
handover/audits/HARNESS_COMPRESSION_AUDIT_2026-04-22.md:124:| **Code Standard** (替代 `## How` 前 3 条) | Art. I.1 + C-004 + C-027 | cargo check/test 须过；`.env` 禁 commit；`kernel.rs/bus.rs/wallet.rs` 改动走 STEP_B_PROTOCOL；参数不可硬编码 |
handover/audits/HARNESS_COMPRESSION_AUDIT_2026-04-22.md:164:-- kernel.rs / bus.rs / wallet.rs edits: human confirm
handover/audits/HARNESS_COMPRESSION_AUDIT_2026-04-22.md:169:+- `src/{kernel,bus,wallet}.rs` 改动走 STEP_B_PROTOCOL (不直接编辑)
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:3084:**Protocol for restricted-file changes**: `STEP_B_PROTOCOL.md` (necessity audit → parallel branch → A/B statistical test → merge on empirical win only). Do NOT directly edit restricted files even with authorization; always A/B test.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:3088:| `recent_rejections` optional global scope | `src/bus.rs` | F-2026-04-15-02 Art. II.1 broadcast | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:3090:| WAL emission | `src/kernel.rs` or bus.rs | F-2026-04-15-03 | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:7363:**Protocol for restricted-file changes**: `STEP_B_PROTOCOL.md` (necessity audit → parallel branch → A/B statistical test → merge on empirical win only). Do NOT directly edit restricted files even with authorization; always A/B test.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:7367:| `recent_rejections` optional global scope | `src/bus.rs` | F-2026-04-15-02 Art. II.1 broadcast | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:7369:| WAL emission | `src/kernel.rs` or bus.rs | F-2026-04-15-03 | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:3363:**Protocol for restricted-file changes**: `STEP_B_PROTOCOL.md` (necessity audit → parallel branch → A/B statistical test → merge on empirical win only). Do NOT directly edit restricted files even with authorization; always A/B test.
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:3367:| `recent_rejections` optional global scope | `src/bus.rs` | F-2026-04-15-02 Art. II.1 broadcast | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:3369:| WAL emission | `src/kernel.rs` or bus.rs | F-2026-04-15-03 | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:96:- (Q6.a) STEP_B_PROTOCOL: bus.rs / kernel.rs / wallet.rs are restricted (require parallel-branch A/B, not direct edit). The B2-B4 diff touches `src/drivers/llm_http.rs`. Is `drivers/llm_http.rs` restricted? If yes, this is a process violation.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:351://     drivers/llm_http.rs::GenerateResponse this same B2). Accurate.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:851:# Diff: evaluator.rs + lib.rs + drivers/llm_http.rs (B2/B3/B4 wiring)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1181:diff --git a/src/drivers/llm_http.rs b/src/drivers/llm_http.rs
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1183:--- a/src/drivers/llm_http.rs
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1184:+++ b/src/drivers/llm_http.rs
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1229: M src/drivers/llm_http.rs
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1489:    19	//     drivers/llm_http.rs::GenerateResponse this same B2). Accurate.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1980:/bin/bash -lc "nl -ba src/drivers/llm_http.rs | sed -n '1,190p'" in /home/zephryj/projects/turingosv4
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3584:./src/drivers/llm_http.rs:31:pub struct GenerateResponse {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3585:./src/drivers/llm_http.rs:33:    pub completion_tokens: u32,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3586:./src/drivers/llm_http.rs:34:    /// API-reported prompt tokens. Falls back to 0 if `usage.prompt_tokens` is
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3587:./src/drivers/llm_http.rs:37:    pub prompt_tokens: u32,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3588:./src/drivers/llm_http.rs:131:                    let tokens = body["usage"]["completion_tokens"]
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3589:./src/drivers/llm_http.rs:134:                    let prompt_tokens = body["usage"]["prompt_tokens"]
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3590:./src/drivers/llm_http.rs:142:                    return Ok(GenerateResponse {
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3591:./src/drivers/llm_http.rs:144:                        completion_tokens: tokens,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3592:./src/drivers/llm_http.rs:145:                        prompt_tokens,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3601:/bin/bash -lc "rg -n \"STEP_B_PROTOCOL|parallel-branch|bus.rs|kernel.rs|wallet.rs|llm_http|restricted|Trust Root|TRUST_ROOT|trust\" -S . -g '"'!target'"'" in /home/zephryj/projects/turingosv4
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3623:./experiments/minif2f_v4/src/cost_aggregator.rs:19://     drivers/llm_http.rs::GenerateResponse this same B2). Accurate.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3627:./src/drivers/mod.rs:1:pub mod llm_http;
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3632:./docs/architecture.md:16:| `sdk/tools/wallet.rs` | WalletTool — balance + YES/NO/LP portfolios |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3633:./docs/architecture.md:20:| `drivers/llm_http.rs` | Resilient HTTP client — multi-provider routing |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3634:./docs/architecture.md:27:| Pure Capital Engine | Invest-only economy (Law 2) | `sdk/tools/wallet.rs` + `prediction_market.rs` |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3637:./cases/V3_LESSONS.md:27:| V3L-10 | 钱包语义污染：balance=5990 报 "Bankrupt"，混淆真破产与保证金不足 | Art. II.1 | C-018 | sdk/tools/wallet.rs |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3640:./cases/V3_LESSONS.md:39:| V3L-17 | 截断悖论：2048 token 硬限截断推理链，挤压物理层而非能力层 | Art. III.2 | C-023 | drivers/llm_http.rs |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3643:./cases/V3_LESSONS.md:44:| V3L-22 | Falsifier 可买 YES token — Goodhart 漏洞，角色激励冲突 | Art. III.4 | C-006 | sdk/tools/wallet.rs |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3645:./cases/V3_LESSONS.md:52:| V3L-25 | reqwest+rustls macOS 死锁 (6 种方案均失败，改用 Python proxy) | Art. IV | C-007 | drivers/llm_http.rs |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3646:./cases/V3_LESSONS.md:54:| V3L-27 | SiliconFlow API N=30 → 401/429 崩溃 | Art. IV | C-027 | drivers/llm_http.rs |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3657:./cases/C-069_constitutional_alignment_audit_protocol.yaml:46:     架构性修改 (bus.rs / kernel.rs / wallet.rs / ledger.rs / sdk/*) 必须
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3680:./rules/enforcement.log:16:[2026-04-13 08:23:37] BLOCK R-002 | /home/zephryj/projects/turingosv4/src/sdk/tools/wallet.rs | BLOCKED: Coin minting / free balance detected — economy is invest-only (Law 2, V-001/V-002)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3689:./rules/enforcement.log:27:[2026-04-13 08:41:42] WARN R-008 | /home/zephryj/projects/turingosv4/src/sdk/tools/wallet.rs | WARNING: Market constant change detected — verify LP amounts and initial balances are intentional
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3698:./CLAUDE.md:14:- `src/{kernel,bus,wallet}.rs` 改动走 STEP_B_PROTOCOL（不直接编辑 main）
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3700:./incidents/V-007_reqwest_tls_macos_deadlock/resolution.md:5:- Rewrote `src/drivers/llm_http.rs` — pure reqwest HTTP, no TLS/subprocess logic
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3718:./handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:80:*   **(Q6.a) STEP_B_PROTOCOL violation**: **PASS**. The diff touches `src/drivers/llm_http.rs`. The restricted file list (`bus.rs`, `kernel.rs`, `wallet.rs`) does not include the `drivers/` directory. No process violation occurred.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3721:./handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:86:- (Q6.a) STEP_B_PROTOCOL: bus.rs / kernel.rs / wallet.rs are restricted (require parallel-branch A/B, not direct edit). The B2-B4 diff touches `src/drivers/llm_http.rs`. Is `drivers/llm_http.rs` restricted? If yes, this is a process violation.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3723:./handover/audits/run_codex_pput_ccl_b2_b4_audit.sh:133:printf '\n```\n\n---\n\n# Diff: evaluator.rs + lib.rs + drivers/llm_http.rs (B2/B3/B4 wiring)\n\n```diff\n' >> "$TMP_PROMPT"
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3749:./handover/audits/EXT_GEMINI_2026-04-21.md:17:    1.  **铸币 (Minting)**: `src/sdk/tools/wallet.rs:141` (`on_init`)。这是唯一合法的铸币点，通过 `if self.genesis_done { return; }` 卫语句确保只执行一次。
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3750:./handover/audits/EXT_GEMINI_2026-04-21.md:18:    2.  **借记 (Debit)**: `src/sdk/tools/wallet.rs:52` (`deduct`)，由 `src/bus.rs:410` (`debit_wallet`) 调用。
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3751:./handover/audits/EXT_GEMINI_2026-04-21.md:19:    3.  **贷记 (Credit)**: `src/sdk/tools/wallet.rs:67` (`credit`)，由 `src/bus.rs:426` (`credit_wallet`) 调用。
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3797:./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1865:**Protocol for restricted-file changes**: `STEP_B_PROTOCOL.md` (necessity audit → parallel branch → A/B statistical test → merge on empirical win only). Do NOT directly edit restricted files even with authorization; always A/B test.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3798:./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1869:| `recent_rejections` optional global scope | `src/bus.rs` | F-2026-04-15-02 Art. II.1 broadcast | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3800:./handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md:1871:| WAL emission | `src/kernel.rs` or bus.rs | F-2026-04-15-03 | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3802:./handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:90:- (Q6.a) STEP_B_PROTOCOL: bus.rs / kernel.rs / wallet.rs are restricted (require parallel-branch A/B, not direct edit). The B2-B4 diff touches `src/drivers/llm_http.rs`. Is `drivers/llm_http.rs` restricted? If yes, this is a process violation.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3804:./handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:156:         "src/drivers/llm_http.rs"],
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3805:./handover/audits/run_gemini_pput_ccl_b2_b4_audit.py:169:    "\n```\n\n---\n\n# Diff: evaluator.rs + lib.rs + drivers/llm_http.rs (B2/B3/B4 wiring)\n\n```diff\n" + diff_text +
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3806:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:96:- (Q6.a) STEP_B_PROTOCOL: bus.rs / kernel.rs / wallet.rs are restricted (require parallel-branch A/B, not direct edit). The B2-B4 diff touches `src/drivers/llm_http.rs`. Is `drivers/llm_http.rs` restricted? If yes, this is a process violation.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3811:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:351://     drivers/llm_http.rs::GenerateResponse this same B2). Accurate.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3812:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:851:# Diff: evaluator.rs + lib.rs + drivers/llm_http.rs (B2/B3/B4 wiring)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3814:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1181:diff --git a/src/drivers/llm_http.rs b/src/drivers/llm_http.rs
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3815:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1183:--- a/src/drivers/llm_http.rs
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3816:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1184:+++ b/src/drivers/llm_http.rs
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3817:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1229: M src/drivers/llm_http.rs
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3820:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1489:    19	//     drivers/llm_http.rs::GenerateResponse this same B2). Accurate.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3821:./handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:1980:/bin/bash -lc "nl -ba src/drivers/llm_http.rs | sed -n '1,190p'" in /home/zephryj/projects/turingosv4
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3826:./handover/audits/EXT_CODEX_2026-04-22.md:10:| Q4. Karpathy TOP-10 hot-path claim | CHALLENGE | `/home/zephryj/projects/turingosv4/src/bus.rs:231-252,416-424`; `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:679-718`; `/home/zephryj/projects/turingosv4/src/sdk/actor.rs:53-74,88-95` | One of the sampled items is plausibly hot (`trace_ancestors` via Boltzmann routing), but the `InvestOnly` and wallet-downcast items are materially overstated in current call paths. |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3831:./handover/audits/EXT_CODEX_2026-04-22.md:76:Claude's hot-path framing is not substantiated. The `InvestOnly` route exists in the bus (`/home/zephryj/projects/turingosv4/src/bus.rs:231-252`), but no current tool emits `ToolSignal::InvestOnly`; the only repo matches are the enum and this bus handler (`/home/zephryj/projects/turingosv4/src/sdk/tool.rs:15-22`, `/home/zephryj/projects/turingosv4/src/bus.rs:218-252`). Actual evaluator-side `invest` traffic bypasses that code and talks to wallet/kernel directly (`/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:679-718`). So the clone exists, but it is not “every invest” and not proven hot.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3833:./handover/audits/EXT_CODEX_2026-04-22.md:80:Again, the performance concern is overstated as phrased. `debit_wallet` and `credit_wallet` do perform repeated `manifest()` + `downcast_mut::<WalletTool>()` scans (`/home/zephryj/projects/turingosv4/src/bus.rs:416-437`), but those functions are not on every transaction. They are used on bus-side invest/refund/hayek/settlement paths (`/home/zephryj/projects/turingosv4/src/bus.rs:233,240,346,359,383-412`), while evaluator-side invest does its own separate wallet downcasts (`/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:694-716`). The deeper issue is architectural duplication and bus bypass, not the raw dynamic-dispatch cost alone.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3837:./handover/audits/EXT_CODEX_2026-04-22.md:113:- **Agents appear to be prompted with the wrong wallet state.** `bus.snapshot()` returns empty `balances` and `portfolios` (`/home/zephryj/projects/turingosv4/src/bus.rs:567-570`), while `UniverseSnapshot::get_balance()` simply reads that map (`/home/zephryj/projects/turingosv4/src/sdk/snapshot.rs:22-39`). The evaluator passes `snap.get_balance(agent_id)` into the prompt (`/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:492-495`), and the prompt displays it as `Balance: {:.0} Coins` (`/home/zephryj/projects/turingosv4/src/sdk/prompt.rs:78-84`). Unless another layer patches the snapshot out-of-band, agents are being told they have `0` balance even when the wallet holds funds. That distorts Art. II.2 economic signaling and makes investment behavior hard to interpret.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3843:./handover/audits/HARNESS_COMPRESSION_AUDIT_2026-04-22.md:26:| `kernel.rs / bus.rs / wallet.rs edits: human confirm` | 这是 R-006/R-007 的复述 + C-004/C-015 的重复 | 删除——rules/active/R-006/R-007 已是机器可执行形式，CLAUDE.md 不必再提文件名 |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3846:./handover/audits/HARNESS_COMPRESSION_AUDIT_2026-04-22.md:124:| **Code Standard** (替代 `## How` 前 3 条) | Art. I.1 + C-004 + C-027 | cargo check/test 须过；`.env` 禁 commit；`kernel.rs/bus.rs/wallet.rs` 改动走 STEP_B_PROTOCOL；参数不可硬编码 |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3847:./handover/audits/HARNESS_COMPRESSION_AUDIT_2026-04-22.md:164:-- kernel.rs / bus.rs / wallet.rs edits: human confirm
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3848:./handover/audits/HARNESS_COMPRESSION_AUDIT_2026-04-22.md:169:+- `src/{kernel,bus,wallet}.rs` 改动走 STEP_B_PROTOCOL (不直接编辑)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3926:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3936:**Protocol for restricted-file changes**: `STEP_B_PROTOCOL.md` (necessity audit → parallel branch → A/B statistical test → merge on empirical win only). Do NOT directly edit restricted files even with authorization; always A/B test.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3927:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3940:| `recent_rejections` optional global scope | `src/bus.rs` | F-2026-04-15-02 Art. II.1 broadcast | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3929:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md:3942:| WAL emission | `src/kernel.rs` or bus.rs | F-2026-04-15-03 | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3993:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:3084:**Protocol for restricted-file changes**: `STEP_B_PROTOCOL.md` (necessity audit → parallel branch → A/B statistical test → merge on empirical win only). Do NOT directly edit restricted files even with authorization; always A/B test.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3994:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:3088:| `recent_rejections` optional global scope | `src/bus.rs` | F-2026-04-15-02 Art. II.1 broadcast | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:3996:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:3090:| WAL emission | `src/kernel.rs` or bus.rs | F-2026-04-15-03 | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4057:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:7363:**Protocol for restricted-file changes**: `STEP_B_PROTOCOL.md` (necessity audit → parallel branch → A/B statistical test → merge on empirical win only). Do NOT directly edit restricted files even with authorization; always A/B test.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4058:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:7367:| `recent_rejections` optional global scope | `src/bus.rs` | F-2026-04-15-02 Art. II.1 broadcast | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4060:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md:7369:| WAL emission | `src/kernel.rs` or bus.rs | F-2026-04-15-03 | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4121:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:3363:**Protocol for restricted-file changes**: `STEP_B_PROTOCOL.md` (necessity audit → parallel branch → A/B statistical test → merge on empirical win only). Do NOT directly edit restricted files even with authorization; always A/B test.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4122:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:3367:| `recent_rejections` optional global scope | `src/bus.rs` | F-2026-04-15-02 Art. II.1 broadcast | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4124:./handover/audits/CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md:3369:| WAL emission | `src/kernel.rs` or bus.rs | F-2026-04-15-03 | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4140:./handover/ai-direct/TAPE_ECONOMY_v1_2026-04-20.md:62:- `src/sdk/tools/wallet.rs`: expose `deduct_if_sufficient(agent, amount) ->
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4141:./handover/ai-direct/TAPE_ECONOMY_v1_2026-04-20.md:99:4. This is an economic change on restricted files (`wallet.rs`, `bus.rs`,
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4156:./handover/ai-direct/EXT_AUDIT_2026-04-21/codex_constitutional_correctness.md:19:| SDK 工具 | `src/sdk/tools/wallet.rs`、`src/sdk/tools/search.rs`、`src/sdk/tools/librarian.rs`、`src/sdk/tools/mod.rs` |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4163:./handover/ai-direct/EXT_AUDIT_2026-04-21/gemini_math_algorithm_perf.md:16:| Law 2 CTF 守恒相关代码 | `src/sdk/tools/wallet.rs`、`src/bus.rs`（`invest` / `settle_portfolios` / `halt_and_settle` / `append_oracle_accepted`）、`src/kernel.rs`（Hayek bounty `kernel.rs:63-103`） |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4165:./handover/ai-direct/EXT_AUDIT_2026-04-21/gemini_math_algorithm_perf.md:42:- `src/sdk/tools/wallet.rs`（credit/debit API）
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4174:./handover/ai-direct/EXT_AUDIT_2026-04-21/run_audit.py:26:        "src/sdk/tools/wallet.rs",
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4187:./handover/ai-direct/PLAN_PHASE_8_TO_10_2026-04-22.md:284:- 任何修改 `bus.rs` / `kernel.rs` / `wallet.rs` 的 PR（`STEP_B_PROTOCOL` 强制 parallel branch）
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4188:./handover/ai-direct/CHECKPOINT_PHASE_2_2026-04-20.md:11:| `src/bus.rs` append phase | Under `TAPE_ECONOMY_V2=1`: after market create, call `wallet.record_shares(author, node_id, γ·lp, 0, 0)` — founder grant. No mint (LP-backed). |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4189:./handover/ai-direct/CHECKPOINT_PHASE_2_2026-04-20.md:12:| `src/bus.rs` halt_and_settle | Under `TAPE_ECONOMY_V2=1`: call `settle_portfolios()` — iterates `wallet.portfolios`, credits YES/NO shares against resolved markets, zeros settled positions for idempotency. |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4198:./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:689:**Protocol for restricted-file changes**: `STEP_B_PROTOCOL.md` (necessity audit → parallel branch → A/B statistical test → merge on empirical win only). Do NOT directly edit restricted files even with authorization; always A/B test.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4199:./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:693:| `recent_rejections` optional global scope | `src/bus.rs` | F-2026-04-15-02 Art. II.1 broadcast | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4201:./handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:695:| WAL emission | `src/kernel.rs` or bus.rs | F-2026-04-15-03 | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4203:./handover/ai-direct/ROADMAP_LAUNCH_2026-04-21.md:218:- **Closed beta**: 邀请 3-5 个受信任外部团队接入；跑两周；每周 checkpoint。每个 P0 fix 跟 STEP_B_PROTOCOL (notepad §5)。
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4205:./handover/ai-direct/CHIEF_ARCHITECT_REPORT_2026-04-23.md:204:All runtime code changes. Blocked from merging to main by STEP_B_PROTOCOL requirement (parallel-branch A/B). Commits ready for dual-audit gate:
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4207:./handover/ai-direct/CHECKPOINT_PHASE_1_2026-04-20.md:118:- Files: `src/bus.rs` (append-phase founder grant), `src/sdk/tools/wallet.rs` (expose record_shares), evaluator delete remove cold-fee if present on that branch
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4272:./handover/ai-direct/EXT_AUDIT_PHASE_8A/brief.md:4:**Change scope**: `src/bus.rs` — restricted file (requires dual external audit per STEP_B_PROTOCOL).
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4274:./handover/ai-direct/EXT_AUDIT_PHASE_8A/brief.md:63:- `/home/zephryj/projects/turingosv4/handover/ai-direct/STEP_B_PROTOCOL.md` Phase 1c
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4275:./handover/ai-direct/EXT_AUDIT_PHASE_8A/brief.md:66:- `/home/zephryj/projects/turingosv4/src/sdk/tools/wallet.rs`（WalletTool 定义）
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4280:./handover/ai-direct/EXT_AUDIT_PHASE_8A/run_gemini.py:21:    "src/sdk/tools/wallet.rs",
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4281:./handover/ai-direct/EXT_AUDIT_PHASE_8A/run_gemini.py:24:    "handover/ai-direct/STEP_B_PROTOCOL.md",
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4303:./handover/ai-direct/CHECKPOINT_PHASE_4_2026-04-21.md:12:| `src/sdk/tools/wallet.rs` | `save_to_disk` / `load_from_disk` (serde JSON); `ensure_agents` (zero-balance for newcomers post-genesis per C-001/C-038) |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4318:./handover/ai-direct/PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md:234:7. **STEP_B_PROTOCOL**：`src/{kernel,bus,wallet}.rs` 改动走 parallel branch A/B，不直接编辑 main
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4319:./handover/ai-direct/PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md:318:- 任何修改 `src/{bus,kernel,wallet}.rs` 的 PR（STEP_B_PROTOCOL 强制 parallel branch）
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4320:./handover/ai-direct/PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md:334:| R-7 | C-061 q-halt 实装扰动现有 bus.generation | 盲点 B-14 | STEP_B_PROTOCOL parallel branch；A/B 比较 PPUT 不降 |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4321:./handover/ai-direct/PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md:392:2. STEP_B_PROTOCOL：`feat/phase-8a-snapshot-fix` worktree
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4323:./handover/ai-direct/STEP_B_PROTOCOL.md:3:**Scope**: any change to files in CLAUDE.md's restricted list (currently `kernel.rs`, `bus.rs`, `wallet.rs`). Also applicable to any proposal that touches "institution" per C-031.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4324:./handover/ai-direct/STEP_B_PROTOCOL.md:13:- Is a less-invasive alternative available? (same effect, no restricted-file touch)
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4325:./handover/ai-direct/STEP_B_PROTOCOL.md:107:1. **C-010**: external audit at 3 junctures (necessity, diff, verdict). Researcher cannot self-approve a restricted change.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4326:./handover/ai-direct/STEP_B_PROTOCOL.md:136:**`src/bus.rs recent_rejections` Art. II.1 fix** (notepad §5, F-2026-04-15-02):
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4366:./handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:388:- `STEP_B_PROTOCOL.md` — restricted-file change protocol; **most B work is plumbing, not behavior change to bus.rs/kernel.rs/wal.rs (those are Trust Root and only modified for hash registration)**. STEP_B_PROTOCOL applies if any actual behavior change to those files becomes necessary.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4395:./handover/alignment/CODE_CANDIDATES_2026-04-22.md:15:| FC1-N6 | `input = ⟨q_i, s_i⟩` | `UniverseSnapshot`<br>`build_agent_prompt`<br>`let prompt = build_agent_prompt(...)`<br>`GenerateRequest` | `src/sdk/snapshot.rs:22`<br>`src/sdk/prompt.rs:15`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:619`<br>`src/drivers/llm_http.rs:14` | `Y`<br>`Y`<br>`N`<br>`Y` | `high` | What the agent sees is the frozen snapshot rendered into a prompt and shipped as the LLM request message. |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4396:./handover/alignment/CODE_CANDIDATES_2026-04-22.md:16:| FC1-N7 | `δ / AI` | `ResilientLLMClient::generate`<br>`match client.generate(&request).await` | `src/drivers/llm_http.rs:84`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:211`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:644` | `Y`<br>`N`<br>`N` | `high` | These are the actual LLM call surfaces that map prompt input to model output in oneshot and swarm runs. |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4413:./handover/alignment/CODE_CANDIDATES_2026-04-22.md:46:| FC3-N37 | `tools` | `TuringTool`<br>`WalletTool`<br>`SearchTool`<br>`LibrarianTool`<br>`Lean4Oracle` | `src/sdk/tool.rs:38`<br>`src/sdk/tools/wallet.rs:24`<br>`src/sdk/tools/search.rs:11`<br>`src/sdk/tools/librarian.rs:17`<br>`experiments/minif2f_v4/src/lean4_oracle.rs:42` | `Y`<br>`Y`<br>`Y`<br>`Y`<br>`N` | `high` | The tool layer is the `TuringTool` trait plus the mounted concrete implementations the swarm can use. |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4420:./handover/alignment/CODE_CANDIDATES_2026-04-22.md:66:- `WalletTool::save_to_disk` — `src/sdk/tools/wallet.rs:83` — cross-problem wallet persistence is not part of the constitutional state-machine diagrams.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4425:./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:29:| FC1-N7 | `δ / AI` | `ResilientLLMClient::generate` | `src/drivers/llm_http.rs:84` | Y | ✅ | none |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:4440:./handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:105:| `WalletTool::save_to_disk` | `src/sdk/tools/wallet.rs:83` | C-041 cross-problem wallet persistence (Art. II.2 time-extended price signal) |
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5622:   388	- `STEP_B_PROTOCOL.md` — restricted-file change protocol; **most B work is plumbing, not behavior change to bus.rs/kernel.rs/wal.rs (those are Trust Root and only modified for hash registration)**. STEP_B_PROTOCOL applies if any actual behavior change to those files becomes necessary.
handover/audits/CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:5831:    14	- `src/{kernel,bus,wallet}.rs` 改动走 STEP_B_PROTOCOL（不直接编辑 main）
handover/ai-direct/STEP_B_PROTOCOL.md:3:**Scope**: any change to files in CLAUDE.md's restricted list (currently `kernel.rs`, `bus.rs`, `wallet.rs`). Also applicable to any proposal that touches "institution" per C-031.
handover/ai-direct/PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md:234:7. **STEP_B_PROTOCOL**：`src/{kernel,bus,wallet}.rs` 改动走 parallel branch A/B，不直接编辑 main
handover/ai-direct/PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md:318:- 任何修改 `src/{bus,kernel,wallet}.rs` 的 PR（STEP_B_PROTOCOL 强制 parallel branch）
handover/ai-direct/PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md:334:| R-7 | C-061 q-halt 实装扰动现有 bus.generation | 盲点 B-14 | STEP_B_PROTOCOL parallel branch；A/B 比较 PPUT 不降 |
handover/ai-direct/PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md:392:2. STEP_B_PROTOCOL：`feat/phase-8a-snapshot-fix` worktree
handover/ai-direct/CHECKPOINT_PHASE_1_2026-04-20.md:118:- Files: `src/bus.rs` (append-phase founder grant), `src/sdk/tools/wallet.rs` (expose record_shares), evaluator delete remove cold-fee if present on that branch
handover/ai-direct/CHECKPOINT_PHASE_2_2026-04-20.md:11:| `src/bus.rs` append phase | Under `TAPE_ECONOMY_V2=1`: after market create, call `wallet.record_shares(author, node_id, γ·lp, 0, 0)` — founder grant. No mint (LP-backed). |
handover/ai-direct/CHECKPOINT_PHASE_2_2026-04-20.md:12:| `src/bus.rs` halt_and_settle | Under `TAPE_ECONOMY_V2=1`: call `settle_portfolios()` — iterates `wallet.portfolios`, credits YES/NO shares against resolved markets, zeros settled positions for idempotency. |
handover/ai-direct/TAPE_ECONOMY_v1_2026-04-20.md:62:- `src/sdk/tools/wallet.rs`: expose `deduct_if_sufficient(agent, amount) ->
handover/ai-direct/TAPE_ECONOMY_v1_2026-04-20.md:99:4. This is an economic change on restricted files (`wallet.rs`, `bus.rs`,
handover/ai-direct/PLAN_PHASE_8_TO_10_2026-04-22.md:284:- 任何修改 `bus.rs` / `kernel.rs` / `wallet.rs` 的 PR（`STEP_B_PROTOCOL` 强制 parallel branch）
handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:689:**Protocol for restricted-file changes**: `STEP_B_PROTOCOL.md` (necessity audit → parallel branch → A/B statistical test → merge on empirical win only). Do NOT directly edit restricted files even with authorization; always A/B test.
handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:693:| `recent_rejections` optional global scope | `src/bus.rs` | F-2026-04-15-02 Art. II.1 broadcast | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md:695:| WAL emission | `src/kernel.rs` or bus.rs | F-2026-04-15-03 | **Human confirm + Step-B** | STEP_B_PROTOCOL |
handover/ai-direct/EXT_AUDIT_2026-04-21/codex_constitutional_correctness.md:19:| SDK 工具 | `src/sdk/tools/wallet.rs`、`src/sdk/tools/search.rs`、`src/sdk/tools/librarian.rs`、`src/sdk/tools/mod.rs` |
handover/ai-direct/EXT_AUDIT_2026-04-21/gemini_math_algorithm_perf.md:16:| Law 2 CTF 守恒相关代码 | `src/sdk/tools/wallet.rs`、`src/bus.rs`（`invest` / `settle_portfolios` / `halt_and_settle` / `append_oracle_accepted`）、`src/kernel.rs`（Hayek bounty `kernel.rs:63-103`） |
handover/ai-direct/EXT_AUDIT_2026-04-21/gemini_math_algorithm_perf.md:42:- `src/sdk/tools/wallet.rs`（credit/debit API）
handover/ai-direct/EXT_AUDIT_2026-04-21/run_audit.py:26:        "src/sdk/tools/wallet.rs",
handover/ai-direct/CHIEF_ARCHITECT_REPORT_2026-04-23.md:204:All runtime code changes. Blocked from merging to main by STEP_B_PROTOCOL requirement (parallel-branch A/B). Commits ready for dual-audit gate:
handover/ai-direct/ROADMAP_LAUNCH_2026-04-21.md:218:- **Closed beta**: 邀请 3-5 个受信任外部团队接入；跑两周；每周 checkpoint。每个 P0 fix 跟 STEP_B_PROTOCOL (notepad §5)。
handover/ai-direct/EXT_AUDIT_PHASE_8A/brief.md:4:**Change scope**: `src/bus.rs` — restricted file (requires dual external audit per STEP_B_PROTOCOL).
handover/ai-direct/EXT_AUDIT_PHASE_8A/brief.md:63:- `/home/zephryj/projects/turingosv4/handover/ai-direct/STEP_B_PROTOCOL.md` Phase 1c
handover/ai-direct/EXT_AUDIT_PHASE_8A/brief.md:66:- `/home/zephryj/projects/turingosv4/src/sdk/tools/wallet.rs`（WalletTool 定义）
handover/ai-direct/EXT_AUDIT_PHASE_8A/run_gemini.py:21:    "src/sdk/tools/wallet.rs",
handover/ai-direct/EXT_AUDIT_PHASE_8A/run_gemini.py:24:    "handover/ai-direct/STEP_B_PROTOCOL.md",
handover/ai-direct/CHECKPOINT_PHASE_4_2026-04-21.md:12:| `src/sdk/tools/wallet.rs` | `save_to_disk` / `load_from_disk` (serde JSON); `ensure_agents` (zero-balance for newcomers post-genesis per C-001/C-038) |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:15:| FC1-N6 | `input = ⟨q_i, s_i⟩` | `UniverseSnapshot`<br>`build_agent_prompt`<br>`let prompt = build_agent_prompt(...)`<br>`GenerateRequest` | `src/sdk/snapshot.rs:22`<br>`src/sdk/prompt.rs:15`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:619`<br>`src/drivers/llm_http.rs:14` | `Y`<br>`Y`<br>`N`<br>`Y` | `high` | What the agent sees is the frozen snapshot rendered into a prompt and shipped as the LLM request message. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:16:| FC1-N7 | `δ / AI` | `ResilientLLMClient::generate`<br>`match client.generate(&request).await` | `src/drivers/llm_http.rs:84`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:211`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:644` | `Y`<br>`N`<br>`N` | `high` | These are the actual LLM call surfaces that map prompt input to model output in oneshot and swarm runs. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:46:| FC3-N37 | `tools` | `TuringTool`<br>`WalletTool`<br>`SearchTool`<br>`LibrarianTool`<br>`Lean4Oracle` | `src/sdk/tool.rs:38`<br>`src/sdk/tools/wallet.rs:24`<br>`src/sdk/tools/search.rs:11`<br>`src/sdk/tools/librarian.rs:17`<br>`experiments/minif2f_v4/src/lean4_oracle.rs:42` | `Y`<br>`Y`<br>`Y`<br>`Y`<br>`N` | `high` | The tool layer is the `TuringTool` trait plus the mounted concrete implementations the swarm can use. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:66:- `WalletTool::save_to_disk` — `src/sdk/tools/wallet.rs:83` — cross-problem wallet persistence is not part of the constitutional state-machine diagrams.
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:29:| FC1-N7 | `δ / AI` | `ResilientLLMClient::generate` | `src/drivers/llm_http.rs:84` | Y | ✅ | none |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:105:| `WalletTool::save_to_disk` | `src/sdk/tools/wallet.rs:83` | C-041 cross-problem wallet persistence (Art. II.2 time-extended price signal) |
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md:388:- `STEP_B_PROTOCOL.md` — restricted-file change protocol; **most B work is plumbing, not behavior change to bus.rs/kernel.rs/wal.rs (those are Trust Root and only modified for hash registration)**. STEP_B_PROTOCOL applies if any actual behavior change to those files becomes necessary.

exec
/bin/bash -lc "rg -n \"RunAggregate|schema_version|PputResult|PPUT_RESULT|serde_json::to_string|from_json|LegacyRunAggregate\" experiments/minif2f_v4 src tests -g '*.rs'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/src/post_hoc_verifier.rs:30:use crate::jsonl_schema::RunAggregate;
experiments/minif2f_v4/src/post_hoc_verifier.rs:69:/// Wrap RunAggregate::compute_pput_verified for callers in evaluator that
experiments/minif2f_v4/src/post_hoc_verifier.rs:72:    RunAggregate::compute_pput_verified(progress, c_i, t_i_ms)
experiments/minif2f_v4/src/post_hoc_verifier.rs:77:    RunAggregate::compute_pput_m_verified(progress, c_i, t_i_ms)
experiments/minif2f_v4/src/jsonl_schema.rs:7:// Versioning: every v2 record carries `schema_version = "v2.0"`. Legacy Paper-1
experiments/minif2f_v4/src/jsonl_schema.rs:8:// era jsonl rows (the `PputResult` shape emitted by evaluator before this commit)
experiments/minif2f_v4/src/jsonl_schema.rs:9:// have NO `schema_version` field, so `RunRecord::from_json` discriminates on
experiments/minif2f_v4/src/jsonl_schema.rs:10:// presence and routes to `LegacyRunAggregate`. No on-disk artifact is rewritten
experiments/minif2f_v4/src/jsonl_schema.rs:36:    pub schema_version: String,
experiments/minif2f_v4/src/jsonl_schema.rs:82:pub struct RunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:87:    pub schema_version: String,
experiments/minif2f_v4/src/jsonl_schema.rs:120:impl RunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:139:/// Legacy v1 run row — mirrors the pre-v2 `PputResult` shape emitted by the
experiments/minif2f_v4/src/jsonl_schema.rs:147:pub struct LegacyRunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:166:    V2(RunAggregate),
experiments/minif2f_v4/src/jsonl_schema.rs:167:    Legacy(LegacyRunAggregate),
experiments/minif2f_v4/src/jsonl_schema.rs:171:    /// Parse one jsonl line. v2 if `schema_version` present, else legacy.
experiments/minif2f_v4/src/jsonl_schema.rs:173:    pub fn from_json(line: &str) -> Result<Self, serde_json::Error> {
experiments/minif2f_v4/src/jsonl_schema.rs:175:        let is_v2 = v.get("schema_version")
experiments/minif2f_v4/src/jsonl_schema.rs:191:    fn sample_run() -> RunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:192:        RunAggregate {
experiments/minif2f_v4/src/jsonl_schema.rs:196:            schema_version: SCHEMA_VERSION_V2.into(),
experiments/minif2f_v4/src/jsonl_schema.rs:204:            pput_verified: RunAggregate::compute_pput_verified(1, 4096, 12_000),
experiments/minif2f_v4/src/jsonl_schema.rs:205:            pput_m_verified: RunAggregate::compute_pput_m_verified(1, 4096, 12_000),
experiments/minif2f_v4/src/jsonl_schema.rs:219:        let line = serde_json::to_string(&original).expect("serialize");
experiments/minif2f_v4/src/jsonl_schema.rs:220:        let parsed: RunAggregate = serde_json::from_str(&line).expect("deserialize");
experiments/minif2f_v4/src/jsonl_schema.rs:221:        assert_eq!(parsed, original, "v2 RunAggregate must round-trip");
experiments/minif2f_v4/src/jsonl_schema.rs:222:        assert!(line.contains("\"schema_version\":\"v2.0\""),
experiments/minif2f_v4/src/jsonl_schema.rs:223:                "serialized line must stamp schema_version");
experiments/minif2f_v4/src/jsonl_schema.rs:230:        assert_eq!(RunAggregate::compute_pput_verified(0, 1000, 5000), 0.0);
experiments/minif2f_v4/src/jsonl_schema.rs:231:        assert_eq!(RunAggregate::compute_pput_m_verified(0, 1000, 5000), 0.0);
experiments/minif2f_v4/src/jsonl_schema.rs:238:        r.pput_verified = RunAggregate::compute_pput_verified(0, r.total_run_token_count, r.total_wall_time_ms);
experiments/minif2f_v4/src/jsonl_schema.rs:239:        r.pput_m_verified = RunAggregate::compute_pput_m_verified(0, r.total_run_token_count, r.total_wall_time_ms);
experiments/minif2f_v4/src/jsonl_schema.rs:244:        assert_eq!(RunAggregate::compute_pput_verified(1, 0, 5000), 0.0);
experiments/minif2f_v4/src/jsonl_schema.rs:245:        assert_eq!(RunAggregate::compute_pput_verified(1, 1000, 0), 0.0);
experiments/minif2f_v4/src/jsonl_schema.rs:254:        match RunRecord::from_json(legacy_line).expect("legacy line parses") {
experiments/minif2f_v4/src/jsonl_schema.rs:268:        let v2_line = serde_json::to_string(&sample_run()).unwrap();
experiments/minif2f_v4/src/jsonl_schema.rs:269:        match RunRecord::from_json(&v2_line).expect("v2 line parses") {
experiments/minif2f_v4/src/bin/evaluator.rs:41:struct PputResult {
experiments/minif2f_v4/src/bin/evaluator.rs:93:    // verified. Field names align with B1 RunAggregate v2 schema. In Phase B,
experiments/minif2f_v4/src/bin/evaluator.rs:109:    // Step-B v3 treatment binary: stamp classifier version in every emitted PputResult.
experiments/minif2f_v4/src/bin/evaluator.rs:159:                PputResult { condition: "hybrid_v1".into(), ..r }
experiments/minif2f_v4/src/bin/evaluator.rs:164:                PputResult {
experiments/minif2f_v4/src/bin/evaluator.rs:177:    let json = serde_json::to_string(&result).unwrap();
experiments/minif2f_v4/src/bin/evaluator.rs:178:    println!("PPUT_RESULT:{}", json);
experiments/minif2f_v4/src/bin/evaluator.rs:204:) -> PputResult {
experiments/minif2f_v4/src/bin/evaluator.rs:290:            // Do not emit PPUT_RESULT — batch runner must retry on resume.
experiments/minif2f_v4/src/bin/evaluator.rs:303:) -> PputResult {
experiments/minif2f_v4/src/bin/evaluator.rs:1016:) -> PputResult {
experiments/minif2f_v4/src/bin/evaluator.rs:1050:    PputResult {
experiments/minif2f_v4/src/bin/evaluator.rs:1082:/// Returns the relative path (for embedding in PputResult) or None on I/O failure.
src/sdk/error_abstraction.rs:16:/// Stamped in PputResult.classifier_version per C-012.
src/drivers/llm_http.rs:183:        let json = serde_json::to_string(&req).unwrap();
src/sdk/tools/wallet.rs:87:        let json = serde_json::to_string_pretty(self)
src/wal.rs:59:        let line = serde_json::to_string(rec)?;

exec
/bin/bash -lc "nl -ba handover/ai-direct/STEP_B_PROTOCOL.md | sed -n '1,20p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# Step-B Protocol: Restricted-File Changes via Parallel Branch Experiment
     2	
     3	**Scope**: any change to files in CLAUDE.md's restricted list (currently `kernel.rs`, `bus.rs`, `wallet.rs`). Also applicable to any proposal that touches "institution" per C-031.
     4	
     5	**Principle**: real data decides, not opinion. Parallel branch lets us test change empirically before merging.
     6	
     7	---
     8	
     9	## Phase 0 — Necessity audit (external, before any code)
    10	
    11	External auditors (**Codex + Gemini**) answer:
    12	- Is the change *necessary*? What observable behavior is broken now?
    13	- Is a less-invasive alternative available? (same effect, no restricted-file touch)
    14	- What's the *minimum sufficient* version? (avoid scope creep)
    15	- What's the failure mode if we don't change?
    16	
    17	**Gate**: if both auditors say "less-invasive alternative exists", take that path instead. If both say "change is necessary", proceed to Phase 1. If they disagree, take the conservative verdict per `feedback_dual_audit_conflict`.
    18	
    19	**My role**: write the audit brief; do not pre-advocate. If I agree with the verdict after reading both, proceed. If I disagree, record dissent in notepad §7 before proceeding.
    20	

exec
/bin/bash -lc "nl -ba handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md | sed -n '23,33p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    23	### 1.1 Progress
    24	
    25	For task `i`:
    26	```
    27	Progress_i = 1   iff   GroundTruth(G_i) = 1   (Lean 4 toolchain verifies the golden-path proof)
    28	Progress_i = 0   otherwise
    29	```
    30	
    31	`GroundTruth` is the Lean oracle in `experiments/minif2f_v4/lean4_oracle.rs` enforced via `check_payload` (forbidden patterns: `sorry`, `admit`, `native_decide`, `decide`, `omega` — per C-011 corollary in F-2026-04-20-05). LLM-judge verdicts and runtime "accepted" markers are NOT GroundTruth.
    32	
    33	### 1.2 Cost C_i (full physical cost)

exec
/bin/bash -lc "rg -n \"from_instants|verify_post_hoc\\(|compute_progress_verified\\(\" experiments/minif2f_v4/src -g '*.rs'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/src/post_hoc_verifier.rs:45:pub fn verify_post_hoc(oracle: &Lean4Oracle, golden_path_payload: &str) -> bool {
experiments/minif2f_v4/src/post_hoc_verifier.rs:59:pub fn compute_progress_verified(runtime_accepted: bool, post_hoc_verified: bool) -> u8 {
experiments/minif2f_v4/src/post_hoc_verifier.rs:105:            compute_progress_verified(runtime_accepted, post_hoc_verified);
experiments/minif2f_v4/src/post_hoc_verifier.rs:138:        let progress_verified = compute_progress_verified(true, true);
experiments/minif2f_v4/src/post_hoc_verifier.rs:155:        let progress_verified = compute_progress_verified(false, false);
experiments/minif2f_v4/src/post_hoc_verifier.rs:167:        assert_eq!(compute_progress_verified(false, true), 0u8,
experiments/minif2f_v4/src/wall_clock.rs:82:    pub fn from_instants(first_read: Instant, final_accept: Instant) -> Self {
experiments/minif2f_v4/src/wall_clock.rs:110:        let wc = RunWallClock::from_instants(first_read, final_accept);
experiments/minif2f_v4/src/bin/evaluator.rs:1036:                    compute_progress_verified(runtime_accepted, post_hoc_verified);

codex
**VERDICT: CHALLENGE**

**Top P0 Findings**

1. B4 is not Phase-C-safe by construction. `make_pput` derives `post_hoc_verified = has_gp`, so a future Soft Law path can set fake `has_gp=true` and emit `verified=true`, `pput_verified>0` without any post-hoc Lean call. See [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1025), [post_hoc_verifier.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/post_hoc_verifier.rs:45), PREREG §1.1 lines 27-31, and plan B4 lines 164-170.

2. B3 undercounts the specified wall-clock bracket and the test masks it. The oneshot prompt is constructed before `mark_first_read`, and swarm calls `build_agent_prompt` before `mark_first_read`; the test explicitly relaxes the planned `≥7100ms` check to `≥7000ms`. See [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:216), [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:559), [wall_clock.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/wall_clock.rs:113), plan B3 lines 138-150, PREREG §1.3 lines 50-55.

3. The emitted run record is still legacy-shaped and not v2-aligned. `PputResult` has no `schema_version`, no `progress`, no `run_id`, no `split`, etc.; B1’s dispatcher therefore classifies new B2-B4 output as legacy, with new fields only in `extra`. Worse, `hybrid_v1` fallback keeps `r2`’s C_i/T_i fields and drops the failed oneshot leg while overriding only legacy `time_secs`/`tx_count`. See [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:41), [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:155), [jsonl_schema.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/jsonl_schema.rs:82), [jsonl_schema.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/jsonl_schema.rs:173), plan B1 lines 57-87.

**Q Answers**

Q1: Current direct `client.generate` Ok sites are wired: oneshot records at [evaluator.rs:237](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:237), swarm at [evaluator.rs:586](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:586). Proposal recording happens before parse at [evaluator.rs:238](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:238) and [evaluator.rs:589](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:589). Bus mutation stdout exclusion is defensible because next-turn prompt tokens account for observed state, but hybrid_v1 violates full-run C_i. `flip_last_failed_to_accepted` silently masks over-flips at [cost_aggregator.rs:79](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/cost_aggregator.rs:79); make it fallible or assert. chars/4 is acceptable only if frozen in B7, consistent with plan open Q lines 381-382.

Q2: Lean call closure is mostly placed correctly for current paths: oneshot [evaluator.rs:249](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:249), complete [evaluator.rs:659](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:659), step [evaluator.rs:892](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:892), max-tx [evaluator.rs:995](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:995). But Soft Law can still skip Lean unless Phase C changes the API. `from_instants` is test-only at [wall_clock.rs:81](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/wall_clock.rs:81). `elapsed_ms` fallback is acceptable for abort paths.

Q3: The AND in `compute_progress_verified` is wrong as the contract. PREREG says Progress is Lean GroundTruth, not runtime accept; runtime belongs in `pput_runtime` and FAR. See [post_hoc_verifier.rs:59](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/post_hoc_verifier.rs:59), PREREG §1.1 lines 27-31, plan B4 lines 159-160. `verify_post_hoc` being unused is okay only for Phase B optimization, not for Phase C. `pput_m_verified = 1e6 * pput_verified` is precision-safe for expected C_i/T_i ranges.

Q4/Q5: Backward read compatibility is okay for old rows through `LegacyRunAggregate` flattening, but new rows are also legacy-classified because evaluator emits no `schema_version`. Field spellings present in `PputResult` match v2 names, but the set misses `progress`, `rollback_count`, guardrails, model snapshot, mode, etc. `verified: Option<bool>` is not a substitute for `progress: u8`.

Q6: No STEP_B process violation from `src/drivers/llm_http.rs`; the restricted scope is `kernel.rs`, `bus.rs`, `wallet.rs` in [CLAUDE.md](/home/zephryj/projects/turingosv4/CLAUDE.md:14) and [STEP_B_PROTOCOL.md](/home/zephryj/projects/turingosv4/handover/ai-direct/STEP_B_PROTOCOL.md:3). But B7 should Trust-Root `cost_aggregator.rs`, `wall_clock.rs`, `post_hoc_verifier.rs`, and likely `evaluator.rs`, `jsonl_schema.rs`, and `llm_http.rs`; current PREREG §1.8 list omits them.

Q7: Not safe by construction. The current architecture relies on future discipline. A Soft Law implementer can set `has_gp=true`; `make_pput` will then stamp verified success without forcing `verify_post_hoc`.

**Recommendations**

Before B5: change the accounting API so terminal emit requires explicit `{ runtime_accepted, post_hoc_verified, progress }`; remove `post_hoc_verified = has_gp`; make Soft Law tests fail unless `verify_post_hoc` was invoked.

Before B6/B7: switch evaluator emission to `RunAggregate` v2 or add `schema_version` + all required fields now. Fix hybrid aggregation or disable `hybrid_v1` for PPUT-CCL.

B7: freeze hashes for all metering/verifier/emitter code, not just Lean and core bus files.

Conviction: high.
tokens used
238,927
**VERDICT: CHALLENGE**

**Top P0 Findings**

1. B4 is not Phase-C-safe by construction. `make_pput` derives `post_hoc_verified = has_gp`, so a future Soft Law path can set fake `has_gp=true` and emit `verified=true`, `pput_verified>0` without any post-hoc Lean call. See [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1025), [post_hoc_verifier.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/post_hoc_verifier.rs:45), PREREG §1.1 lines 27-31, and plan B4 lines 164-170.

2. B3 undercounts the specified wall-clock bracket and the test masks it. The oneshot prompt is constructed before `mark_first_read`, and swarm calls `build_agent_prompt` before `mark_first_read`; the test explicitly relaxes the planned `≥7100ms` check to `≥7000ms`. See [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:216), [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:559), [wall_clock.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/wall_clock.rs:113), plan B3 lines 138-150, PREREG §1.3 lines 50-55.

3. The emitted run record is still legacy-shaped and not v2-aligned. `PputResult` has no `schema_version`, no `progress`, no `run_id`, no `split`, etc.; B1’s dispatcher therefore classifies new B2-B4 output as legacy, with new fields only in `extra`. Worse, `hybrid_v1` fallback keeps `r2`’s C_i/T_i fields and drops the failed oneshot leg while overriding only legacy `time_secs`/`tx_count`. See [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:41), [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:155), [jsonl_schema.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/jsonl_schema.rs:82), [jsonl_schema.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/jsonl_schema.rs:173), plan B1 lines 57-87.

**Q Answers**

Q1: Current direct `client.generate` Ok sites are wired: oneshot records at [evaluator.rs:237](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:237), swarm at [evaluator.rs:586](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:586). Proposal recording happens before parse at [evaluator.rs:238](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:238) and [evaluator.rs:589](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:589). Bus mutation stdout exclusion is defensible because next-turn prompt tokens account for observed state, but hybrid_v1 violates full-run C_i. `flip_last_failed_to_accepted` silently masks over-flips at [cost_aggregator.rs:79](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/cost_aggregator.rs:79); make it fallible or assert. chars/4 is acceptable only if frozen in B7, consistent with plan open Q lines 381-382.

Q2: Lean call closure is mostly placed correctly for current paths: oneshot [evaluator.rs:249](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:249), complete [evaluator.rs:659](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:659), step [evaluator.rs:892](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:892), max-tx [evaluator.rs:995](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:995). But Soft Law can still skip Lean unless Phase C changes the API. `from_instants` is test-only at [wall_clock.rs:81](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/wall_clock.rs:81). `elapsed_ms` fallback is acceptable for abort paths.

Q3: The AND in `compute_progress_verified` is wrong as the contract. PREREG says Progress is Lean GroundTruth, not runtime accept; runtime belongs in `pput_runtime` and FAR. See [post_hoc_verifier.rs:59](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/post_hoc_verifier.rs:59), PREREG §1.1 lines 27-31, plan B4 lines 159-160. `verify_post_hoc` being unused is okay only for Phase B optimization, not for Phase C. `pput_m_verified = 1e6 * pput_verified` is precision-safe for expected C_i/T_i ranges.

Q4/Q5: Backward read compatibility is okay for old rows through `LegacyRunAggregate` flattening, but new rows are also legacy-classified because evaluator emits no `schema_version`. Field spellings present in `PputResult` match v2 names, but the set misses `progress`, `rollback_count`, guardrails, model snapshot, mode, etc. `verified: Option<bool>` is not a substitute for `progress: u8`.

Q6: No STEP_B process violation from `src/drivers/llm_http.rs`; the restricted scope is `kernel.rs`, `bus.rs`, `wallet.rs` in [CLAUDE.md](/home/zephryj/projects/turingosv4/CLAUDE.md:14) and [STEP_B_PROTOCOL.md](/home/zephryj/projects/turingosv4/handover/ai-direct/STEP_B_PROTOCOL.md:3). But B7 should Trust-Root `cost_aggregator.rs`, `wall_clock.rs`, `post_hoc_verifier.rs`, and likely `evaluator.rs`, `jsonl_schema.rs`, and `llm_http.rs`; current PREREG §1.8 list omits them.

Q7: Not safe by construction. The current architecture relies on future discipline. A Soft Law implementer can set `has_gp=true`; `make_pput` will then stamp verified success without forcing `verify_post_hoc`.

**Recommendations**

Before B5: change the accounting API so terminal emit requires explicit `{ runtime_accepted, post_hoc_verified, progress }`; remove `post_hoc_verified = has_gp`; make Soft Law tests fail unless `verify_post_hoc` was invoked.

Before B6/B7: switch evaluator emission to `RunAggregate` v2 or add `schema_version` + all required fields now. Fix hybrid aggregation or disable `hybrid_v1` for PPUT-CCL.

B7: freeze hashes for all metering/verifier/emitter code, not just Lean and core bus files.

Conviction: high.
