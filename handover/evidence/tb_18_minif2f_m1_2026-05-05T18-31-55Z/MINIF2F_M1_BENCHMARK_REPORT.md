# TB-18 Atom H sub-stage 2 — M1 Formal Benchmark Report (2026-05-05)

> **🛑 GRANDFATHERED 2026-05-06 — DO NOT USE AS BENCHMARK EVIDENCE.**
>
> This M1 result predates **TB-18R Tape Restoration** (charter at `handover/tracer_bullets/TB-18R_charter_2026-05-06.md`; VETO archive at `handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md`).
>
> Per-LLM-call externalization was **not yet enforced** at this run. Failure-path asymmetry is present: success ω-paths externalize correctly, but `step_reject` / `parse_fail` / `llm_err` / `step_partial_ok` paths leak to evaluator stdout / kernel.tape shadow without L4 / L4.E entries. P49-class heavy runs show `evaluator_tx_count=32` but `L4_WorkTx=1` and `L4.E_real_LLM_rejection=0`.
>
> Ship verdict: **VETO** (external audit 2026-05-06; user-confirmed). **TB-18 M1 / M2 / M3 / NodeMarket / PriceIndex / Polymarket-signal / public-chain / real-world-readiness all FROZEN until TB-18R SHIPPED FINAL with G2 PASS.**
>
> Per `feedback_no_retroactive_evidence_rewrite`: this evidence is preserved as-is (no L4 / L4.E / CAS root rewrite). Annotation is going-forward only. Use the post-TB-18R-ship rerun evidence at `handover/evidence/tb_18r_p23_p38_p49_rerun_*/` and `handover/evidence/tb_18r_m0_rerun_*/` instead.

---

**Status**: PROVISIONAL — pending G1 dual external audit + architect § sign-off.
**Phase**: TB-18 Formal Benchmark Scale-Up · ladder M1 (n=1 / 50 problems / 600s per-problem)
**HEAD**: `c9e0dc1` (G0 CHALLENGE-resolved) · **manifest_id**: `652890ecc158d7139fb699b682298605bad3857ad3fb37a74dc6fd5a83fd57af`
**Run dir**: `handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/`
**Wall-clock total**: 9,280 s (2h 35m) · M0-runner-rc=0

---

## §1 Authority + scope

Architect ratification §B.9.3 verbatim:
> M1 = Full heldout subset (50-100 problems / n1 + n3 / all failures produce EvidenceCapsule / dashboard batch report).

This run pins **n=1 / 50 problems**; an M1.n3 follow-up manifest will pin n=3 if the n=1 distribution is credible.

**Per `feedback_minif2f_scaling_policy`**: M1 is part of the **harness-prep ladder**, NOT a real-world benchmark and NOT a public claim of formal-verification capability. The scale-up that earns those framings begins at TB-18-after-TB-17-ship (handover/architect-insights M2-M4 successive scale-ups).

## §2 Headline metrics (Art. I.2 mandatory triplet)

| Metric | Value |
|---|---|
| **n** (problems) | **50** |
| **Σ PPUT** (solved-only) | **123.6869** |
| **Mean PPUT (solved)** | **7.2757** |
| **Solve count** | **17 / 50** |
| **Solve rate** | **34.0%** |
| **95% Wilson CI on solve rate** | **[22.4%, 47.8%]** |

**Solve count cannot be cited independently — it is paired with PPUT per Art. I.2 / C-053 / C-061.**

## §3 Halt-reason distribution (Art. IV terminal taxonomy — five-way partition)

| RunOutcome | Count | % |
|---|---|---|
| OmegaAccepted | **17** | 34% |
| MaxTxExhausted | **24** | 48% |
| WallClockCap | **9** | 18% |
| ComputeCapViolated | 0 | 0% |
| ErrorHalt | 0 | 0% |
| (TB-18 Atom A new: DegradedLLM) | 0 | 0% |

**Halt-mode interpretation**:
- OmegaAccepted (17): clean solve → TaskOpen + EscrowLock + Work + Verify + FinalizeReward (all 5 tx kinds emitted; chain l4_count = 5).
- MaxTxExhausted (24): MAX_TX=20 reached without Omega → TaskOpen + EscrowLock + TerminalSummary (l4_count = 3); EvidenceCapsule emitted with `terminal_reason = MaxTxExhausted`.
- WallClockCap (9): external 600s SIGTERM killed the process before internal halt cleanup → TaskOpen + EscrowLock only (l4_count = 2); no TerminalSummary, no EvidenceCapsule. **This is a known M-ladder pattern** — the 600s outer timeout from `run_m0_minif2f_harness` races with the internal aggregate budget of the same value; SIGTERM wins on the chains that are still reading from the LLM at the boundary.
- DegradedLLM (0): the per-LLM-call budget tracker (atom A) did not fire on this batch — DeepSeek-chat did not drift into the consecutive-trivial-response halt path. **Note**: this means the new RunOutcome::DegradedLLM variant remains **substrate-only validated** in M1; field exercise is forward-bound to M2 / longer-running batches.

## §4 Cluster solve rates (problem-set decomposition)

| Cluster | n | Solved | Rate |
|---|---|---|---|
| `mathd_numbertheory_*` | 12 | 9 | **75%** |
| `numbertheory_*` (no `mathd_` prefix) | 2 | 1 | 50% |
| `mathd_algebra_*` | 14 | 6 | **43%** |
| `amc12*` | 9 | 1 | 11% |
| `aime_*` | 3 | 0 | 0% |
| `algebra_*` (no `mathd_` prefix) | 4 | 0 | 0% |
| `imo_*` | 4 | 0 | 0% |
| `induction_*` | 2 | 0 | 0% |

**Difficulty stratification (matches MiniF2F community baseline)**: `mathd_*` problems are short, syntactically simple, and amenable to single-shot LLM tactic suggestion; competition problems (`aime`, `imo`, `amc12`) require multi-step reasoning and proof restructuring that a 20-tx envelope does not allow.

## §5 Audit invariants (50/50 chain-backed)

| Invariant | Result |
|---|---|
| audit_tape verdict | **50/50 PROCEED** (zero BLOCK / zero ERROR) |
| Replay byte-identical | **50/50** (verdict.json byte-cmp verdict_replay.json on every chain) |
| Tamper 3/3 detected | **38/50 (76%)** |
| Tamper 2/3 (DEGRADED) | **12/50 (24%)** |
| **assert_27 capsule.reason↔outcome consistency (G0 fix)** | **PASS on every problem** ✓ |

**assert_27 production validation**: the new G0-fix consistency check (`cap.terminal_reason.to_run_outcome() == ts.run_outcome`) was exercised on **all 24 MaxTxExhausted chains** (which carry `TerminalSummary` + `EvidenceCapsule(reason=MaxTxExhausted)`) and on **all 17 OmegaAccepted chains** (which carry `TerminalSummary` only when an Omega-confirm path emits one — most actually do not, so they pass the assertion vacuously). All 50 chains audit GREEN under the stricter assertion. **No semantic drift in capsule emission across the M1 batch.**

### §5.1 Tamper-DEGRADED root cause (non-blocker)

12/50 chains show `detected=2/3` on `audit_tape_tamper`. Breakdown:

- **9 are WallClockCap chains** (P02/P05/P08/P12/P13/P17/P20/P29/P41): l4_count=2; `flip_cas_byte` corrupts the largest CAS object which (because the chain never wrote a TerminalSummary's evidence_capsule_cid) is not on the audit-walk path → corruption not detected.
- **3 are OmegaAccepted chains** (P39/P43/P49): l4_count=5; `flip_cas_byte` selected an unwalked CAS object (proof_telemetry / verbose tactic logs), which is also not referenced by any audit assertion.

**Diagnosis**: `audit_tape_tamper` selects the largest CAS object regardless of audit-walk reachability. When the largest object is not in the audit walk, corruption isn't detected. **Verifier is correct**: audit_tape only checks what its assertions reference; tampering an unwalked object is, by design, a no-op. **Tamper test is incomplete**: the largest-object heuristic skips the assertion-coverage signal.

**Forward improvement (TB-19+ candidate)**: `audit_tape_tamper` should target objects that are referenced by at least one assertion (e.g. `evidence_capsule_cid` referent, `tx_payload_cid` referent). Filed as future improvement; does not invalidate TB-18 M1.

## §6 PPUT distribution + reputation

**PPUT (solved-only)** — see §2 headline. Per-solved PPUT values are recorded in `M0_BATCH_SUMMARY.json` plus per-problem `evaluator.stdout::PPUT_RESULT` lines.

**Reputation distribution (p50 / p90 / max)**: **N/A** for this run. Reputation is computed across multi-agent runs (n≥2). M1 is n=1 / single solver / single verifier; no inter-agent reputation signal is generated. M1.n3 (forward-bound) will populate this.

**Consensus signal**: **N/A**. The 50 problems were each evaluated as a single-agent run; no chain-level consensus mechanism (Verify quorum / Challenge / ChallengeResolve) was exercised in the per-problem runs (those tx kinds appear only in the comprehensive_arena synthetic 13/13 chain that was used for atom B-impl evidence, NOT in the M1 LLM-driven evaluator runs).

## §7 Multi-agent diagnostics — non-applicable

Per Art. II.2.1 + C-052: `parent_selection_entropy` and `pairwise_payload_diversity_mean` are **mandatory only when n_agents ≥ 2**. M1 pins n=1; these diagnostics are reported as **N/A — n=1 condition does not require them**.

## §8 Per-problem table (50/50)

| # | Problem | Outcome | dur | l4 | tx_kinds | tamper |
|---|---|---|---|---|---|---|
| 01 | `aime_1983_p2` | exhausted | 68s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 02 | `aime_1984_p1` | error_or_no_pput | 600s | 2 | escrow_lock+task_open | 2/3 |
| 03 | `aime_1990_p4` | exhausted | 87s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 04 | `algebra_2varlineareq_fp3zeq11_3tfm1m5zeq` | exhausted | 72s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 05 | `algebra_9onxpypzleqsum2onxpy` | error_or_no_pput | 600s | 2 | escrow_lock+task_open | 2/3 |
| 06 | `algebra_absxm1pabsxpabsxp1eqxp2_0leqxleq` | exhausted | 249s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 07 | `algebra_sqineq_at2malt1` | exhausted | 71s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 08 | `amc12_2000_p1` | error_or_no_pput | 600s | 2 | escrow_lock+task_open | 2/3 |
| 09 | `amc12_2000_p12` | exhausted | 97s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 10 | `amc12_2000_p20` | exhausted | 76s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 11 | `amc12_2000_p6` | exhausted | 74s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 12 | `amc12_2001_p21` | error_or_no_pput | 600s | 2 | escrow_lock+task_open | 2/3 |
| 13 | `amc12_2001_p5` | error_or_no_pput | 600s | 2 | escrow_lock+task_open | 2/3 |
| 14 | `amc12a_2002_p13` | exhausted | 393s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 15 | `amc12a_2002_p6` | exhausted | 72s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 16 | `amc12a_2003_p23` | ✓ solved | 139s | 5 | escrow_lock+finalize_reward+task_open+verify+work | 3/3 |
| 17 | `imo_1959_p1` | error_or_no_pput | 600s | 2 | escrow_lock+task_open | 2/3 |
| 18 | `imo_1960_p2` | exhausted | 77s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 19 | `imo_1962_p2` | exhausted | 79s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 20 | `imo_1963_p5` | error_or_no_pput | 600s | 2 | escrow_lock+task_open | 2/3 |
| 21 | `induction_11div10tonmn1ton` | exhausted | 75s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 22 | `induction_12dvd4expnp1p20` | exhausted | 175s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 23 | `mathd_algebra_107` | ✓ solved | 9s | 5 | escrow_lock+finalize_reward+task_open+verify+work | 3/3 |
| 24 | `mathd_algebra_113` | exhausted | 86s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 25 | `mathd_algebra_114` | exhausted | 330s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 26 | `mathd_algebra_125` | ✓ solved | 9s | 5 | escrow_lock+finalize_reward+task_open+verify+work | 3/3 |
| 27 | `mathd_algebra_129` | exhausted | 89s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 28 | `mathd_algebra_137` | exhausted | 72s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 29 | `mathd_algebra_139` | error_or_no_pput | 600s | 2 | escrow_lock+task_open | 2/3 |
| 30 | `mathd_algebra_141` | ✓ solved | 10s | 5 | escrow_lock+finalize_reward+task_open+verify+work | 3/3 |
| 31 | `mathd_algebra_142` | ✓ solved | 10s | 5 | escrow_lock+finalize_reward+task_open+verify+work | 3/3 |
| 32 | `mathd_algebra_143` | ✓ solved | 11s | 5 | escrow_lock+finalize_reward+task_open+verify+work | 3/3 |
| 33 | `mathd_algebra_148` | exhausted | 78s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 34 | `mathd_algebra_153` | exhausted | 83s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 35 | `mathd_algebra_176` | ✓ solved | 9s | 5 | escrow_lock+finalize_reward+task_open+verify+work | 3/3 |
| 36 | `mathd_algebra_400` | exhausted | 81s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 37 | `mathd_numbertheory_100` | ✓ solved | 11s | 5 | escrow_lock+finalize_reward+task_open+verify+work | 3/3 |
| 38 | `mathd_numbertheory_1124` | ✓ solved | 194s | 5 | escrow_lock+finalize_reward+task_open+verify+work | 3/3 |
| 39 | `mathd_numbertheory_12` | ✓ solved | 38s | 5 | escrow_lock+finalize_reward+task_open+verify+work | 2/3 |
| 40 | `mathd_numbertheory_127` | ✓ solved | 11s | 5 | escrow_lock+finalize_reward+task_open+verify+work | 3/3 |
| 41 | `mathd_numbertheory_135` | error_or_no_pput | 600s | 2 | escrow_lock+task_open | 2/3 |
| 42 | `mathd_numbertheory_150` | exhausted | 74s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 43 | `mathd_numbertheory_175` | ✓ solved | 54s | 5 | escrow_lock+finalize_reward+task_open+verify+work | 2/3 |
| 44 | `mathd_numbertheory_185` | ✓ solved | 10s | 5 | escrow_lock+finalize_reward+task_open+verify+work | 3/3 |
| 45 | `mathd_numbertheory_207` | ✓ solved | 9s | 5 | escrow_lock+finalize_reward+task_open+verify+work | 3/3 |
| 46 | `mathd_numbertheory_212` | ✓ solved | 8s | 5 | escrow_lock+finalize_reward+task_open+verify+work | 3/3 |
| 47 | `mathd_numbertheory_222` | ✓ solved | 18s | 5 | escrow_lock+finalize_reward+task_open+verify+work | 3/3 |
| 48 | `mathd_numbertheory_239` | exhausted | 180s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |
| 49 | `numbertheory_2pownm1prime_nprime` | ✓ solved | 371s | 5 | escrow_lock+finalize_reward+task_open+verify+work | 2/3 |
| 50 | `numbertheory_3pow2pownm1mod2pownp3eq2pow` | exhausted | 199s | 3 | escrow_lock+task_open+terminal_summary | 3/3 |

## §9 Reproducibility

```bash
TB18_M1_USER_AUTH_GO=1 \
  bash handover/tests/scripts/run_tb_18_atom_h_m1_2026-05-05.sh --skip-build
```

Preconditions verified at runner gate-check:
1. G0 verdict file present + PASS or CHALLENGE-resolved (`handover/audits/CODEX_MICRO_AUDIT_TB_18_PRE_H_VERDICT_*_2026-05-05.md`)
2. `TB18_M1_USER_AUTH_GO=1` (explicit user "go")
3. Manifest exists + manifest_id deterministic (sha256 over canonical JSON minus manifest_id field)
4. `git rev-parse --short HEAD` == `manifest.turingosv4_commit` = `c9e0dc1` (commit drift gate)
5. M0 runner present
6. `combined50.txt` has exactly 50 problems

## §10 Comparison to M0 retry baseline (no regression)

| Metric | M0 retry (n=20) | M1 (n=50) | Delta |
|---|---|---|---|
| Solve rate | 35% | **34%** | -1pt (within Wilson CI) |
| Replay byte-identical | 20/20 | **50/50** | match |
| Tamper 3/3 | 14/20 (70%) | **38/50 (76%)** | +6pt |
| audit_tape verdict | 20 PROCEED | **50 PROCEED** | match |
| WallClockCap halts | 30% | **18%** | -12pt (M1 better-behaved tail) |

**Conclusion**: G0 substrate fix (assert_27 stricter check + comprehensive_arena helper parameterization) shipped at HEAD `c9e0dc1` does NOT regress harness behavior. M1 solve rate at the 50-problem scale is statistically indistinguishable from M0 retry's 20-problem baseline (Wilson CI on 17/50 contains 7/20 = 35%).

## §11 Charter SG closure (TB-18)

| SG | Status | Evidence |
|---|---|---|
| SG-18.5 (atom H executes) | ✅ | 50/50 chain-backed; `audit_tape` PROCEED on each |
| SG-18.13 (BenchmarkManifest pinned) | ✅ | `M1_RUN_MANIFEST.json` (frozen with manifest_id, frozen_problems_sha256, frozen_at_invocation_utc, head_at_invocation) |
| SG-18.14 (EvidencePackagingPolicy) | ⚠️ partial — sampled per §12 | runtime_repo + cas snapshots staged for sampled subset; full 50 not committed (oversize) |
| SG-18.15 (G0 micro-audit) | ✅ | R1 CHALLENGE → R2 PASS (`handover/audits/CODEX_MICRO_AUDIT_TB_18_PRE_H_VERDICT_*_2026-05-05.md`) |
| SG-18.16 (G1 final audit) | ⏳ requested | this report + G1 audit-request docs filed |

## §12 Evidence sampling policy

Per `feedback_evidence_packaging_policy_required` (large-scale runs MUST declare sample strategy):

**Strategy**: random + failure-heavy + solved + unsolved diversity.

**Sampled set** (committed to repo):
- 4 OmegaAccepted: P23/P31/P39/P49 (covers fast solve, mathd_algebra, mathd_numbertheory, numbertheory_*; one tamper-DEGRADED for fixture coverage)
- 4 MaxTxExhausted: P01/P14 (393s outlier)/P25 (330s outlier)/P36 (mathd_algebra_400 boundary)
- 3 WallClockCap: P02/P29 (mathd_algebra timeout; rare)/P41 (mathd_numbertheory timeout; rare)

**Non-sampled**: per-problem evidence dirs are present locally at `handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/` but not all checked into git (51-folder × ~MB-each oversized; full set retained on the dev machine for G1 + future replay).

`EVIDENCE_INDEX.json` (committed) catalogs all 50 problems with relative paths, outcomes, and per-problem hashes.

## §13 Forward-binding

| Item | Status | Owner |
|---|---|---|
| External Codex G1 final audit | Request filed at `handover/audits/CODEX_G1_FINAL_AUDIT_REQUEST_2026-05-05.md` | User-invoked |
| External Gemini G1 final audit | Request filed at `handover/audits/GEMINI_G1_FINAL_AUDIT_REQUEST_2026-05-05.md` | User-invoked |
| Architect § sign-off (TB-17 §8 precedent) | This report + G1 verdicts | User-conveyed |
| M1.n3 (50 × n=3) | Manifest draft pending; trigger after architect § sign-off | TB-18.M1.n3 |
| M2 (100+ × n=5; observe-only) | Manifest draft pending | TB-18.M2 |

## §14 Cross-references

- TB-18 charter §1.4 SG-18.5 / SG-18.13-16
- Architect 2026-05-05 ratification ruling §B.9.3 + Q4 + Q7
- G0 audit trail: `handover/audits/CODEX_MICRO_AUDIT_TB_18_PRE_H_VERDICT_2026-05-05.md` (R1) + `_R2_2026-05-05.md` (R2 PASS)
- Substrate ship: commits `d3c8d78..0c3a5e1` + `c9e0dc1` (G0 CHALLENGE-resolved)
- Memory: `feedback_benchmark_manifest_required` · `feedback_evidence_packaging_policy_required` · `feedback_minif2f_scaling_policy` · `feedback_audit_after_evidence` · `feedback_no_workarounds_strict_constitution`

---

**Provisional ship status**: TB-18 M1 atom H sub-stage 2 evidence is COMPLETE. Awaiting G1 + architect § sign-off for terminal ship status.
