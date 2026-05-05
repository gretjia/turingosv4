# MiniF2F M0 Benchmark Report — TB-18 Atom H sub-stage 1 (2026-05-05)

**Status**: PRELIMINARY — TB-18 PROVISIONAL ship-time evidence; full M-ladder M1 + M2 → TB-18.H-impl follow-up.
**Filed**: 2026-05-05.
**TB-18 sequence position**: Atom H sub-stage 1 of 3 (M0 retry; M1 + M2 forward-bound).
**Authority**: TB-18 charter §1.4 SG-18.10/.11/.12/.13/.14 + architect TB-18 ratification ruling §B.9.3 + §3 atom H + Q5 (M0 dual-position H0 early + H late).

---

## §1 Required disclaimers (mandatory per architect §B.10.2 + §2.9 + §2.10 + SG-18.14)

```text
Formal benchmark capacity only.
Not real-world readiness.
No real-world domain.
No real funds.
No public settlement.
```

This is **harness audit evidence at M0 scale**, NOT a benchmark score claim. Per architect §B.9.3 verbatim:

> M0 — Benchmark harness audit:
>     20 known problems; chain-backed; no market;
>     prove no fake accepted.

### §1.1 Benchmark contamination disclosure (architect §2.10)

The MiniF2F problem set is **publicly available**. The model under test (`deepseek-chat`) was trained on data that LIKELY includes MiniF2F problems and/or paraphrases. Solve outcomes recorded here are **NOT** a model-novelty claim. They are a **system benchmark**: ChainTape continuity / per-LLM-call budget enforcement / replay determinism / no-fake-accepted / EvidenceCapsule emission, with the LLM as a fixed substrate (whose memorization vs reasoning vs generation behavior is OUT OF SCOPE for this report).

Per architect §2.10 verbatim: "system benchmark = ChainTape/replay/stability benchmark; not model capability SOTA claim".

### §1.2 NOT a benchmark score

Per `feedback_minif2f_scaling_policy`: "M0+M1 acceptable as harness-prep during TB-17 (NOT as benchmark); never claim real-world readiness from MiniF2F."

The numbers below describe the harness behavior on 20 known problems, NOT the system's general capability.

---

## §2 Configuration (frozen per BenchmarkManifest)

Per `handover/manifests/TB-18_BENCHMARK_MANIFEST.json` (frozen at batch start):

```text
problem_set:           MiniF2F valid (handover/tests/scripts/m0_problems.txt)
problem_count:         20
model:                 deepseek-chat (version captured at batch UTC timestamp)
model.temperature:     0.2
model.max_output_tokens: 8000
runtime.max_tx:        20
runtime.per_call_wallclock_seconds: 60
runtime.token_floor_threshold: 30
runtime.consecutive_trivial_response_cap: 10
runtime.aggregate_per_run_wallclock_seconds: 600
runtime.external_timeout_per_problem_seconds: 120
runtime.n_agents:      1 (n1; run_swarm path with single-agent)
boltzmann_mode:        observe-only
market_state:          disabled
real_funds:            false
public_settlement:     false
real_world_domain:     false
turingosv4_commit:     7bb18b4 (TB-18 substrate at batch start)
```

---

## §3 Outcomes table (per-problem)

`<TBD: filled after M0 retry completes; sourced from handover/evidence/tb_18_m0_retry_2026-05-05/r1/M0_BATCH_SUMMARY.json + per-problem verdict.json>`

Format:

| # | Problem | Outcome | Wall-clock | audit verdict | tamper | LLM calls | trivial_calls | DegradedLLM? |
|---|---|---|---|---|---|---|---|---|
| P01 | mathd_algebra_107 | TBD | TBD | TBD | TBD | TBD | TBD | TBD |
| ... |

---

## §4 Aggregate counts (per CLAUDE.md Report Standard Art. I.2)

### §4.1 PPUT (Frozen 5-step compile loop signal)

```text
ΣPPUT (sum across solved):     <TBD>
Mean PPUT (solved):            <TBD>
95% CI Wilson:                 <TBD>
solved_count:                  <TBD>
total_count:                   20
```

### §4.2 Reputation distribution (Art. I.2 statistical signal #1)

```text
reputation_distribution.p50:   <TBD>
reputation_distribution.p90:   <TBD>
reputation_distribution.max:   <TBD>
```

(M0 single-agent → reputation distribution is a degenerate 1-row signal; included for forward-binding consistency with M1/M2 multi-agent runs.)

### §4.3 Halt reason distribution (Art. IV terminal-state distinction)

```text
halt_reason_distribution = {
  OmegaAccepted:    <count>,    # solved
  MaxTxExhausted:   <count>,    # MAX_TX=20 exhausted naturally
  WallClockCap:     <count>,    # internal aggregate cap (atom A)
  ComputeCap:       <count>,
  ErrorHalt:        <count>,
  DegradedLLM:      <count>     # NEW atom A variant; consecutive-trivial cap
}
```

### §4.4 Multi-agent diversity signals (Art. II.2.1)

N/A — M0 is single-agent. M2 (TB-18.H-impl) will compute:
- `parent_selection_entropy`
- `pairwise_payload_diversity_mean`

Both must be ≥ 0.25 per Art. II.2.1; values < 0.25 = alarm.

---

## §5 Architect §B.9.3 M0 spec compliance

| Requirement | Status |
|---|---|
| 20 known problems | TBD pending M0 retry completion |
| chain-backed | ✅ Each problem produces full ChainTape (runtime_repo + cas + verdict + replay + tamper) |
| no market | ✅ MarketState=disabled in manifest; no FORCE_BANKRUPTCY / FORCE_EXPIRE / FORCE_REDEEM hooks set |
| prove no fake accepted | TBD: per-problem audit_tape PROCEED + on-disk proofs/*.lean for solved problems |

---

## §6 Architect §2.4 failure mode coverage (M0 spec verbatim)

| Mode | Coverage |
|---|---|
| solved problem | TBD |
| unsolved problem | TBD |
| LLM degraded / budget cap | TBD (depends on whether DeepSeek drift recurs in this 20-problem batch) |
| Lean failure | TBD (substantive proposals failing Lean verification) |
| EvidenceCapsule emission | TBD (problems hitting MaxTxExhausted produce EvidenceCapsule via atom E pipeline) |
| no fake accepted | TBD (audit_tape verdict + proof file presence cross-check) |

---

## §7 vs M0 r1 (predecessor 2026-05-05 11:48; commit `6471c28`)

| | M0 r1 | M0 retry (this report) |
|---|---|---|
| problems run | 3 of 20 (script killed at P03 hung 240s+) | 20 of 20 (this batch) |
| substrate | TB-16.x.2.6 ship state | TB-18 atom A budget-enforced |
| solved | 1 (P01 mathd_algebra_107) | TBD |
| hung 600s | 1 (P02 mathd_algebra_113) | TBD (atom A budget should halt earlier on drift) |

---

## §8 EvidencePackagingPolicy compliance (TB-7R/TB-8/TB-9 precedent)

Per `handover/policies/TB-18_EVIDENCE_PACKAGING_POLICY.md` §1: M0 = FULL restorable evidence.

Per problem: `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` + verdict + replay + tamper + evaluator stdout/stderr + audit_tape stderr + proofs/ (if solved).

Replay integrity check (per policy §5):
- `git fsck --strict` on extracted runtime_repo + cas: TBD
- `audit_tape` from extracted state verdict=PROCEED: TBD
- Replay byte-equal to committed verdict.json: TBD

---

## §9 Forward triggers

| Item | Forward-bound to |
|---|---|
| **M1** (50-100 × n1/n3; SAMPLED packaging) | TB-18.H-impl follow-up runs (multi-hour LLM compute) |
| **M2** (100+ × n5; SAMPLED packaging; Boltzmann observe-only multi-agent) | TB-18.H-impl follow-up runs (multi-day LLM compute) |
| **Replay integrity verification batch** | TB-18.H-impl follow-up commit |
| **Architect §2.4 EvidenceCapsule emission proof on natural MaxTxExhausted** | TB-18 M0 retry produces ≥1 chain reaching MaxTxExhausted naturally (TBD pending batch completion) |
| **DegradedLLM emission proof on natural drift** | TB-18 M0 retry IF DeepSeek drift recurs (or TB-18.H-impl with synthetic drift test mode) |

---

## §10 Cross-references

- Manifest: `handover/manifests/TB-18_BENCHMARK_MANIFEST.json`
- Packaging policy: `handover/policies/TB-18_EVIDENCE_PACKAGING_POLICY.md`
- Evidence dir: `handover/evidence/tb_18_m0_retry_2026-05-05/r1/`
- M0 r1 predecessor: `handover/evidence/m0_minif2f_harness_audit_2026-05-05/r1/`
- M0 problems: `handover/tests/scripts/m0_problems.txt` (20 problems)
- M0 runner script: `handover/tests/scripts/run_m0_minif2f_harness_2026-05-05.sh`
- TB-18 charter §1.4 SG-18.10/.11/.12/.13/.14
- Architect TB-18 ruling §B.9.3 + §2.4 + §2.9 + §2.10 + Q5

---

**End of preliminary report.** Final report fields filled in subsequent commit when M0 retry batch completes.
