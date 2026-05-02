# TB-7R Smoke Evidence — 2026-05-02

**Date**: 2026-05-02
**TB**: TB-7R (Constitution-Aligned Frame B Repair)
**Source**: `target/debug/evaluator` HEAD = `b517ae5`, branch `main`
**Model**: `deepseek-v4-flash` via local LLM proxy at `localhost:8080`
**Lean**: 4.29.1 (`/home/zephryj/.elan/bin/lean`)
**Mathlib**: `/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/.lake/build`
**Verdict acceptance reference**: `handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md` §F

---

## §0 Headline

| Step | Config | Outcome | parent_tx edges | chain_oracle_verified |
|---|---|---|---|---|
| Single | n1 × `mathd_algebra_171` × MAX_TX=10 | SOLVED | none | true |
| Half-1 | n1 × `mathd_algebra_171` × MAX_TX=20 | SOLVED | none | true |
| Half-2 | n1 × `mathd_algebra_107` × MAX_TX=20 | SOLVED | none | true |
| Half-3 | n1 × `mathd_algebra_359` × MAX_TX=20 | SOLVED | none | true |
| Full-A | n5 × `mathd_algebra_171` × MAX_TX=20 | SOLVED | none | true |
| Full-B-1 | n1 × `mathd_algebra_171` × MAX_TX=20 | SOLVED | none | true |
| Full-B-2 | n1 × `mathd_algebra_107` × MAX_TX=20 | SOLVED | none | true |
| Full-B-3 | n1 × `mathd_algebra_359` × MAX_TX=20 | SOLVED | none | true |
| Full-B-4 | n1 × `aime_1997_p9` × MAX_TX=20 | UNSOLVED | none | false |
| Full-B-5 | n1 × `mathd_numbertheory_5` × MAX_TX=20 | UNSOLVED | none | false |

**Aggregate**: 8/10 SOLVED + chain_oracle_verified=true; 2/10 UNSOLVED + chain_oracle_verified=false. **All 7 indicators GREEN on every run.** **No fake accepted node** in any UNSOLVED run.

---

## §1 Verdict §F acceptance — claim by claim

### Single smoke (1 problem × MAX_TX small)

```text
Pass criterion: genesis_report.json valid; ≥1 attempt node in ChainTape
                (L4 or L4.E); replay reconstructs.
```

| Criterion | Status | Citation |
|---|---|---|
| genesis_report.json valid | ✓ | `single_n1_mathd_algebra_171/runtime_repo/genesis_report.json` (9 fields populated, constitution_hash matches genesis_payload.toml) |
| ≥1 attempt node in ChainTape | ✓ | dashboard §2: `L4=3, L4.E=3` (1 real-LLM L4 Work + 3 synthetic-seed L4.E Work / Verify) |
| replay reconstructs | ✓ | dashboard §2: `ledger_root_verified ✓`, `state_reconstructed ✓` |

**Single smoke: PASS.**

### Half smoke (3 problems × MAX_TX=20)

```text
Pass criterion: same as single, plus parent_tx edges visible
                when multiple externalized proposals exist on same branch.
```

| Run | 7 indicators | chain_oracle_verified | parent_tx edges |
|---|---|---|---|
| Half-1 | ALL 7 GREEN | true ✓ | none (1-attempt solve) |
| Half-2 | ALL 7 GREEN | true ✓ | none (1-attempt solve) |
| Half-3 | ALL 7 GREEN | true ✓ | none (1-attempt solve) |

The "when multiple externalized proposals exist on same branch" conditional is NOT triggered — every half-smoke problem solved on the first OMEGA-pertactic emission. Per architect verdict A1=B′, this is the correct behavior: 1 compound proposal = 1 Attempt Node, no per-tactic decomposition.

**Half smoke: PASS** (parent_tx criterion vacuously satisfied; conditional not triggered).

### Full smoke (5 problems OR CONDITION=n5 × MAX_TX≥20)

```text
Pass criterion: ≥2 agent_ids; ≥1 parent_tx edge;
                all externalized proposals in ChainTape (L4 or L4.E);
                solved problem has chain_oracle_verified golden proposal;
                unsolved problem has L4.E failures and no fake accepted node.
```

Two complementary runs were captured (the verdict's `(or CONDITION=n5)` parenthetical):

#### Full-A (CONDITION=n5 × `mathd_algebra_171` × MAX_TX=20)

| Criterion | Status | Citation |
|---|---|---|
| ≥2 agent_ids | ✓ | dashboard §4: Agent_0 + tb6-smoke-agent + tb6-smoke-sponsor + tb7-7-sponsor |
| ≥1 parent_tx edge | **✗** | dashboard §6: `(no branch edges)` |
| All externalized proposals in ChainTape | ✓ | proposal_count=2; runtime_externalized=2 |
| Solved problem has chain_oracle_verified golden proposal | ✓ | dashboard §3 + §7 |
| Unsolved-problem clauses | N/A | (this run solved) |

#### Full-B (5 problems × n1 × MAX_TX=20)

| Run | 7 indicators | chain_oracle_verified | L4.E real-LLM Work? | parent_tx edges |
|---|---|---|---|---|
| Full-B-1 (mathd_algebra_171, SOLVED) | ALL 7 GREEN | true ✓ | n/a | none |
| Full-B-2 (mathd_algebra_107, SOLVED) | ALL 7 GREEN | true ✓ | n/a | none |
| Full-B-3 (mathd_algebra_359, SOLVED) | ALL 7 GREEN | true ✓ | n/a | none |
| Full-B-4 (aime_1997_p9, UNSOLVED) | ALL 7 GREEN | false ✓ | none (LLM gave up before externalizing any proposal) | none |
| Full-B-5 (mathd_numbertheory_5, UNSOLVED) | ALL 7 GREEN | false ✓ | none (same shape as B-4) | none |

| Criterion | Status |
|---|---|
| ≥2 agent_ids (across all 5 runs) | ✓ |
| ≥1 parent_tx edge | **✗** |
| All externalized proposals in ChainTape | ✓ |
| Solved problems have chain_oracle_verified golden proposal | ✓ (3/3 solved) |
| Unsolved problems have L4.E failures | **partial** — synthetic-seed L4.E entries present, but no real-LLM L4.E Work (LLM gave up entirely without externalizing a failed proposal) |
| Unsolved problems have no fake accepted node | ✓ |

**Full smoke: PARTIAL PASS** — 4 of 6 sub-criteria pass strictly; 2 sub-criteria are **architecturally not satisfiable under verdict A1=B′ + the `complete` tool** (see §2 below).

---

## §2 Open observation — `parent_tx edge` criterion structural absence

**Observation**: Across all 10 runs in this smoke, `parent_tx` edges are universally absent. Dashboard §6 always reports `(no branch edges — proposals are root-only or telemetry parent_tx is None)`.

**Root cause analysis**:

`parent_tx` edges link two proposals on the same `(agent_id, branch_id)` lineage. They appear when an agent makes ≥2 externalized proposals in the same run. Under TB-7R's architecture as defined by verdict A1=B′:

1. The `complete` tool produces **one compound proposal per LLM turn** (whole proof in one tool call).
2. If the proposal's Lean verification passes, OMEGA-pertactic emits **one** WorkTx and the run terminates with `chain_oracle_verified=true`.
3. If the LLM cannot produce a working proof, current behavior is to **give up entirely** rather than emit a failed proposal followed by a retry.

Therefore under B′ + `complete` tool, the natural runtime shape is **1 successful proposal OR 0 proposals**, never N>1 proposals on the same branch. `parent_tx` edges are architecturally absent in the success and failure paths exercised by `mathd_*` and `aime_*` problems with DeepSeek-v4-flash + 20-tx budget.

**Where parent_tx edges WOULD appear** (under TB-7R rules, not synthesized):

- A problem where the LLM emits a proposal that fails Lean verification AND then makes a follow-up proposal on the same branch before giving up. This requires either (a) a model that retries on Lean errors with externalized intermediate output, or (b) a higher-level orchestration that surfaces failed proposals before retry.
- Per-tactic decomposition where each tactic is its own externalized tool call (deferred to TB-8+ per verdict A1).

**Why this is structural truth, not a TB-7R defect**:

The `parent_tx` plumbing is wired correctly (TB-7.7 D2 commit `a39c31b`; `last_tx_by_agent_branch` map in `evaluator.rs`; `ProposalTelemetry::new_with_parent`). When two proposals occur on the same branch, the second's `parent_tx` IS populated. The TB-7.7 D2 unit tests verify this. The smoke evidence above just doesn't trigger that scenario under B′ + the current tool surface.

**Where this OBS belongs in the audit chain**:

- This file (smoke evidence README) — surfaces the observation against verdict §F criteria.
- Ship audit (Task #9) — auditor should explicitly assess whether the structural absence is acceptable as TB-7R-grade evidence under B′, OR whether the smoke is incomplete and a different problem class is needed.
- TB-8+ charter — when per-tactic decomposition is reopened, parent_tx edges become natural evidence.

The **honest reading** of this smoke: TB-7R Frame B is structurally complete (every solved proposal lands as L4 + VerificationResult; every dashboard regenerates from ChainTape + CAS), but the verdict §F full-smoke `parent_tx edge` criterion presupposes a runtime scenario that does not occur under verdict A1=B′. Future TB-8+ smoke design should explicitly choose a problem class that exercises multi-proposal branches (or relax the criterion).

---

## §3 What this evidence proves (vs verdict §11 acceptance)

```text
For every externalized LLM proposal:                        ✓
  it is represented as either L4 accepted WorkTx or L4.E rejected evidence.
For every L4 accepted WorkTx:                                ✓
  predicate evidence (VerificationResult CAS) exists and resolves.
For every failed proposal:                                   ✓ (partial — see §2)
  it is in L4.E only; raw_diagnostic shielded but auditable.
For every dashboard report:                                  ✓
  it can be deleted and regenerated from ChainTape + CAS alone.
```

The "every failed proposal" clause is satisfied for the failed-proposals that actually externalized (zero in this smoke; LLM gave up without emitting). The "no fake accepted node" rule is correctly enforced — UNSOLVED runs have `chain_oracle_verified=false` and no L4 Work entry.

---

## §4 Per-run artifacts

Each run subdirectory contains:

```text
runtime_repo/
  genesis_report.json    — TB-7R Deliverable C
  initial_q_state.json   — TB-7.7 D7 (preseeded balances + task state)
  agent_audit_trail.jsonl
  agent_pubkeys.json
  pinned_pubkeys.json
  rejections.jsonl       — L4.E records
  synthetic_rejection_label.json
  (.git refs/transitions/main = L4 chain)
cas/
  .git/                  — CAS git store
  .turingos_cas_index.jsonl  — sidecar index (TB-7.6 atomic-write)
stdout                   — PPUT_RESULT JSON line
stderr                   — RUST_LOG=warn output
dashboard.txt            — `audit_dashboard --repo runtime_repo --cas cas` output
```

---

## §5 Reproduce

```bash
mkdir -p /tmp/tb7r_repro/{runtime_repo,cas}
TURINGOS_CHAINTAPE_PATH=/tmp/tb7r_repro/runtime_repo \
TURINGOS_CAS_PATH=/tmp/tb7r_repro/cas \
TURINGOS_CHAINTAPE_PRESEED=1 \
TURINGOS_RUN_ID=tb7r-repro \
CONDITION=n5 \
MAX_TRANSACTIONS=20 \
target/debug/evaluator mathd_algebra_171.lean

target/debug/audit_dashboard --repo /tmp/tb7r_repro/runtime_repo --cas /tmp/tb7r_repro/cas
```

---

## §6 Cross-references

- TB-7R authorization: `handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md`
- TB-7R charter: `handover/tracer_bullets/TB-7R_charter_2026-05-01.md`
- L4 purity audit: `handover/audits/L4_PURITY_AUDIT_TB7R_2026-05-02.md`
- Codex micro-audit: `handover/audits/CODEX_TB7R_MICRO_AUDIT_2026-05-02.md`
- TRACE_MATRIX orphan registry: `handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md`
- Three-node taxonomy: `handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md`
- Pre-TB-7R smoke (grandfathered): `handover/evidence/tb_7_7_dag_capable_smoke_2026-05-01/`, `tb_7_chaintape_smoke_2026-05-01/`, `tb_7_real_smoke_5_problems_2026-05-01/`
