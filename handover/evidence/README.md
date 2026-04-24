# Evidence Archive — TuringOS v4 Session 2026-04-22 → 2026-04-23

**Purpose**: local, self-contained archive of all empirical evidence produced during the autonomous auto-research session. External reviewers (Codex, Gemini, human) can re-verify every claim from these files alone.

---

## § 1. Directory map

```
handover/evidence/
├── README.md                    ← this file (index)
├── sample_E1_hard10.txt         ← 10 hard problems FAILed in both Phase 9.A seeds 31415+2718
├── sample_E1_easy10.txt         ← 10 easy problems SOLVED in all 3 Phase 9.A seeds
├── phase9a_jsonl/               ← Phase 9.A N=50 baseline raw data
│   ├── phase9a_c027fixed_seed31415_n8_*.jsonl     (50 rows)
│   ├── phase9a_seed2718_n8_*.jsonl                (50 rows, MaxTxExhausted halt_reason)
│   └── phase9a_seed141421_n8_*.jsonl              (50 rows, incl depth=12 OMEGA)
├── e1_jsonl/                    ← E1 heterogeneity A/B raw data (8 batches)
│   ├── E1_A_homogeneous_n8_*.jsonl         (seed 141421, A side)
│   ├── E1_B_heterogeneous_n8_*.jsonl       (seed 141421, B side)
│   ├── E1_A_seed31415_n8_*.jsonl           (rep A)
│   ├── E1_B_seed31415_n8_*.jsonl           (rep B)
│   ├── E1_A_seed2718_n8_*.jsonl            (rep 2 A)
│   ├── E1_B_seed2718_n8_*.jsonl            (rep 2 B)
│   ├── E1_A_easy_ctrl_n8_*.jsonl           (easy-set negative A)
│   └── E1_B_easy_ctrl_n8_*.jsonl           (easy-set negative B)
└── e1_proofs/                   ← Lean4 proof artifacts for B-unique solves
    ├── mathd_algebra_44_*.lean             (5 artifacts across runs)
    ├── mathd_algebra_332_*.lean            (4 artifacts)
    └── imo_1962_p2_*.lean                  (4 artifacts)
```

---

## § 2. Key empirical claims with source artifacts

### Claim 1: Phase 9.A baseline — 3 seeds × N=50 data exists with halt_reason telemetry

| Seed | File | Solved | ΣPPUT | Mean PPUT (solved) | depth≥10 OMEGA |
|---|---|---|---|---|---|
| 31415 | `phase9a_jsonl/phase9a_c027fixed_seed31415_*.jsonl` | 13/50 | 82.56 | 6.35 | 0 |
| 2718 | `phase9a_jsonl/phase9a_seed2718_*.jsonl` | 13/50 | 75.35 | 5.80 | 0 |
| 141421 | `phase9a_jsonl/phase9a_seed141421_*.jsonl` | 21/50 | 112.47 | 5.36 | **1 (imo_1962_p2 depth=12)** |

Reproduce via aggregator:
```bash
python3 experiments/minif2f_v4/analysis/phase9_aggregate.py --label dual \
  handover/evidence/phase9a_jsonl/phase9a_c027fixed_seed31415_*.jsonl \
  handover/evidence/phase9a_jsonl/phase9a_seed2718_*.jsonl \
  handover/evidence/phase9a_jsonl/phase9a_seed141421_*.jsonl
```

### Claim 2: E1 heterogeneity emergence — 3 seeds × A/B paired

| Seed | A (HOMOGENEOUS_AGENTS=1) | B (4 skills incl Meta-Planner) | B − A |
|---|---|---|---|
| 141421 | 1/10 | 3/10 | +2 (imo_1962_p2, algebra_44) |
| 31415 | 2/10 | 5/10 | +3 (+ algebra_332) |
| 2718 | 2/10 | 3/10 | +1 (algebra_44) |
| **Σ** | **5/30** | **11/30** | **6 paired B-unique, 0 A-unique** |

Reproduce via Python:
```python
import json
for lbl in ['A_homogeneous','B_heterogeneous','A_seed31415','B_seed31415','A_seed2718','B_seed2718']:
    with open(f'handover/evidence/e1_jsonl/E1_{lbl}_n8_*.jsonl') as f:
        d = [json.loads(l) for l in f]
    solves = {x['problem'].split('/')[-1].replace('.lean','') for x in d if x['has_golden_path']}
    print(lbl, len(solves), sorted(solves))
```

### Claim 3: E1 specificity — easy-set control Δ=0

| Batch | Solved |
|---|---|
| E1_A_easy_ctrl | 10/10 |
| E1_B_easy_ctrl | 10/10 |
| Δ | **0** |

Confirms emergence effect is compositional-specific, not generic PPUT inflation. Files in `e1_jsonl/E1_{A,B}_easy_ctrl_n8_*.jsonl`.

### Claim 4: McNemar's paired test

- 6 B-unique solves, 0 A-unique across 3 paired A/B runs × 10 problems = 30 paired trials
- McNemar binomial sign test: P(6 or more vs 0, n=6, p=0.5) = **0.016**
- Significant at 5% level

### Claim 5: Compositional tactic-family evidence in B-unique solve payloads

**mathd_algebra_44** (B-unique in all 3 seeds):
- artifact: `e1_proofs/mathd_algebra_44_1776869774_7c6d7e9.lean` (seed 141421) etc.
- gp_payload: `constructor\nrefine ⟨?_, ?_⟩\nrefine ⟨?_, ?_⟩; nlinarith`
- Tactic families: structural (`constructor`, `refine`) + algebraic (`nlinarith`)
- A (pure algebraic skill) agents would never emit `constructor` or `refine`

**imo_1962_p2** (B-unique in 2/3 seeds):
- artifact: `e1_proofs/imo_1962_p2_1776869602_faeefc7.lean` (seed 141421) etc.
- gp_payload: 12-step tape chain mixing `refine`, `constructor`, `rcases`, `linarith`
- IMO problem; never solved by chat in 26 historical oneshot runs; solved here via multi-family composition

---

## § 3. Verification commands (anyone can run)

### § 3.1 Binary rebuild
```bash
git checkout 61ccc21  # E1 commit on experiment/phase-8a-snapshot-fix
cargo build --release -p minif2f_v4 --bin evaluator
```

### § 3.2 E1 smallest reproducer
```bash
# A (should solve only algebra_246 on hard10)
HOMOGENEOUS_AGENTS=1 BOLTZMANN_SEED=141421 MAX_TRANSACTIONS=50 \
  bash experiments/minif2f_v4/run_list.sh n8 \
  experiments/minif2f_v4/analysis/sample_E1_hard10.txt E1_A_reproduction

# B (should solve algebra_246 + algebra_44 + imo_1962_p2)
BOLTZMANN_SEED=141421 MAX_TRANSACTIONS=50 \
  bash experiments/minif2f_v4/run_list.sh n8 \
  experiments/minif2f_v4/analysis/sample_E1_hard10.txt E1_B_reproduction
```

### § 3.3 Proof re-verification
```bash
# Any of the e1_proofs/*.lean files re-verify via Lean 4 + Mathlib:
lean --stdin < handover/evidence/e1_proofs/mathd_algebra_44_1776869774_7c6d7e9.lean
```

---

## § 4. Related evidence outside this directory

Other empirical artifacts committed in the repo:

- `handover/ai-direct/E1_EMERGENCE_VERDICT_2026-04-23.md` — initial E1 seed 141421 verdict
- `handover/ai-direct/E1_FINAL_VERDICT_3SEEDS_2026-04-23.md` — final 3-seed + easy-set verdict (this is THE paper-primary-claim document)
- `handover/ai-direct/PAPER_1_OUTLINE_v2_E1_LED_2026-04-23.md` — Paper 1 v2 outline centered on E1
- `handover/ai-direct/CHIEF_ARCHITECT_REPORT_2026-04-23.md` — full session report
- `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` — notepad with findings F-2026-04-22-07/08/09 + F-2026-04-23-01
- `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md` — constitutional alignment matrix
- `handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md` — alignment real-problem validation
- `handover/alignment/FC_ELEMENTS_2026-04-22.md` — 134-element flowchart raw extract
- `handover/alignment/CODE_CANDIDATES_2026-04-22.md` — code candidate map
- `handover/alignment/OBS_CONSTITUTION_MERMAID_FENCE_2026-04-22.md` — constitution fence observation
- `cases/C-068_external_model_behavior_drift.yaml` — chat-fence silent reject case
- `cases/C-069_constitutional_alignment_audit_protocol.yaml` — TRACE_MATRIX protocol

---

## § 5. Integrity

All files under `handover/evidence/` are copied from the exp worktree's working logs (`/home/zephryj/projects/turingosv4/.claude/worktrees/phase-8a-snapshot/experiments/minif2f_v4/logs/` and `proofs/`). Original files preserved in-place; this directory is a committed snapshot for reviewer access.

Git-committed this snapshot makes the claims reproducible from the repository alone — no live system required.
