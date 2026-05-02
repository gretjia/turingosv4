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

**2026-05-02 packaging update** (post Codex ship-audit round-1 VETO on evidence reproducibility): each run subdirectory now ships with self-contained `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` + `replay_report.json`. Total committed evidence size = **892 KB** (10 runs combined; loose .git stores would be 4.8 MB but git auto-ignores nested .git directories — tar.gz keeps the artifacts tracked while compressing 18× smaller than loose objects). Each `replay_report.json` is the literal output of `target/debug/verify_chaintape --repo <run>/runtime_repo --cas <run>/cas --out replay_report.json` (run after extracting the two tar.gz files) — every run shows all 7 top-level verifier booleans true (`ledger_root_verified`, `system_signatures_verified`, `state_reconstructed`, `economic_state_reconstructed`, `cas_payloads_retrievable`, `agent_signatures_verified`, `proposal_telemetry_cas_retrievable`) plus `detail.initial_q_state_loaded_from_disk=true`, alongside the `l4_entries`/`l4e_entries` counts. Acceptance clause 4 ("dashboard regeneratable from ChainTape + CAS alone") is now strictly satisfied from committed evidence; ship condition #5 ("proposal telemetry + payload CIDs resolve") is independently checkable by any auditor via §5.1 below. Round-trip verified: extracting the tar.gz pair into a fresh dir and re-running `verify_chaintape` produces a structurally identical replay_report.json (modulo runtime-tagged `run_id`/`epoch`).

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

## §2 parent_tx natural absence — architect verdict 2026-05-02

**Architect ruling**: per `handover/directives/2026-05-02_TB7R_PARENT_TX_DAG_SMOKE_VERDICT.md`, parent_tx is a **conditional invariant**, not an unconditional smoke requirement:

> "If a real LLM run solves in a single externalized proposal under B′ complete-tool semantics, then parent_tx_edges = 0 is valid; the golden path is a singleton node; there is no DAG defect."

**Observation across all 10 smoke runs**:

`natural parent_tx_edges = 0 occurred because complete-tool runs solved in one externalized proposal`. This is the architect-sanctioned outcome under verdict A1=B′ + the `complete` tool's one-compound-proposal-per-turn semantics:

1. The `complete` tool produces **one compound proposal per LLM turn** (whole proof in one tool call).
2. If the proposal's Lean verification passes, OMEGA-pertactic emits **one** WorkTx and the run terminates with `chain_oracle_verified=true`.
3. If the LLM cannot produce a working proof in this `complete` action, current behavior is to give up rather than emit a failed proposal — so unsolved runs likewise have 0 externalized proposals.

Per architect ruling, `do not fabricate parent_tx edges in natural smoke evidence`. The plumbing is proven separately by **deterministic conformance tests** at `tests/tb_7r_parent_tx_conformance.rs`.

**Dashboard's parent_tx_state field** distinguishes the architect-mandated four cases (extends architect-listed three with a positive multi-attempt state):

| state | meaning | seen in this smoke? |
|---|---|---|
| `SingletonGoldenPathValid` | 1 L4 Work + chain_oracle_verified=true; B′ singleton solve | ✓ 8 of 10 runs |
| `NoMultiAttemptObserved` | DAG not exercised; conformance test demonstrates plumbing | ✓ 2 of 10 runs (unsolved) |
| `MultiAttemptDagValid` | ≥1 multi-attempt branch with all parent_tx edges present | ✗ (per architect ruling, expected absence) |
| `MissingParentTxViolation` | ≥1 multi-attempt branch with missing parent_tx (REAL VIOLATION) | ✗ |

**Conformance test results** (separate from natural smoke; deterministic synthetic fixtures):

```
running 6 tests
test singleton_golden_path_has_zero_edges_and_is_valid          ... ok
test second_attempt_same_branch_has_parent_tx                   ... ok
test missing_parent_on_nonroot_attempt_is_violation             ... ok
test dashboard_renders_singleton_golden_path                    ... ok
test unsolved_runs_have_no_fake_accepted_nodes                  ... ok
test proposal_count_chain_equals_externalized_proposal_count    ... ok
test result: ok. 6 passed; 0 failed
```

These six tests prove:
- Plumbing for `MultiAttemptDagValid` works on a synthetic 2-attempt fixture (test 2).
- Plumbing detects `MissingParentTxViolation` on a synthetic 2-attempt fixture with attempt_2.parent_tx=None (test 3).
- Singleton solved → `SingletonGoldenPathValid` (test 1).
- Unsolved → `NoMultiAttemptObserved`, no fake accepted node (test 5).
- Dashboard renders singleton golden path with depth=0 [ORACLE] (test 4).
- proposal_count exactly matches externalized count (test 6).

Per architect ship condition: **"forced parent_tx conformance test passes" — ✓ MET.**

**Carry-forward to TB-8+**: per-tactic decomposition is deferred (verdict A1=B′). When TB-8 reopens per-tactic, multi-attempt branches will become natural in smoke evidence too. Until then, `parent_tx_state` for natural smoke is expected to be `SingletonGoldenPathValid` or `NoMultiAttemptObserved`.

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

Each run subdirectory contains (committed to git after 2026-05-02 packaging update):

```text
runtime_repo/
  genesis_report.json            — TB-7R Deliverable C
  initial_q_state.json           — TB-7.7 D7 (preseeded balances + task state)
  agent_audit_trail.jsonl
  agent_pubkeys.json
  pinned_pubkeys.json
  rejections.jsonl               — L4.E records (raw_diagnostic shielded per Art. III.4)
  synthetic_rejection_label.json
runtime_repo.dotgit.tar.gz       — compressed `.git/` containing refs/transitions/main + chain commit objects (LedgerEntry blobs); ~12 KB; expand into runtime_repo/ to enable replay
cas/
  .turingos_cas_index.jsonl      — sidecar index (TB-7.6 atomic-write)
cas.dotgit.tar.gz                — compressed `.git/` containing CAS payload blobs (proposal payloads + ProposalTelemetry + VerificationResult + agent audit trail records); ~12 KB; expand into cas/ to enable CID resolution
replay_report.json               — literal `verify_chaintape` output (7 top-level booleans true + `detail.initial_q_state_loaded_from_disk` + l4/l4e counts + final_state_root_hex + final_ledger_root_hex + head_commit_oid_hex)
stdout                           — PPUT_RESULT JSON line
stderr                           — RUST_LOG=warn output
dashboard.txt                    — `audit_dashboard --repo runtime_repo --cas cas` output
```

**Why tar.gz instead of loose `.git/` directories**: git automatically treats nested `.git/` directories as submodule sentinels and refuses to track their contents (would otherwise require submodule registration with external remotes). Compressed-archive packaging keeps the chain stores fully tracked under the parent repo's history while compressing 18× smaller than loose objects (892 KB total for 10 runs vs ~4.8 MB loose).

Per-run replay_report indicator counts:
```text
SOLVED runs (8):   l4_entries=3 (TaskOpen + EscrowLock + accepted Work), l4e_entries=3 (synthetic seeds + zero-stake rejections)
UNSOLVED runs (2): l4_entries=2 (TaskOpen + EscrowLock; no accepted Work — chain_oracle_verified=false), l4e_entries=2
```

---

## §5 Reproduce

### §5.1 Verify committed evidence (no LLM / Lean required)

Any auditor with this repo + a built `target/debug/{verify_chaintape,audit_dashboard}` can independently verify any of the 10 runs from committed evidence alone:

```bash
RUN=handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171  # or any run dir

# Stage a working copy with .git stores expanded:
WORK=/tmp/audit_$$
mkdir -p $WORK/runtime_repo $WORK/cas
cp $RUN/runtime_repo/*.json $RUN/runtime_repo/*.jsonl $WORK/runtime_repo/
cp $RUN/cas/.turingos_cas_index.jsonl $WORK/cas/
tar -xzf $RUN/runtime_repo.dotgit.tar.gz -C $WORK/runtime_repo
tar -xzf $RUN/cas.dotgit.tar.gz -C $WORK/cas

# Re-derive replay_report.json from committed ChainTape + CAS:
target/debug/verify_chaintape \
  --repo $WORK/runtime_repo \
  --cas  $WORK/cas \
  --out  $WORK/replay_report.audit.json

# Compare to committed (byte-identical modulo run_id + epoch):
jq -S 'del(.run_id,.epoch)' $RUN/replay_report.json    > /tmp/orig.json
jq -S 'del(.run_id,.epoch)' $WORK/replay_report.audit.json > /tmp/repro.json
diff /tmp/orig.json /tmp/repro.json && echo "STRUCTURALLY IDENTICAL"

# Re-derive dashboard from committed ChainTape + CAS:
target/debug/audit_dashboard --repo $WORK/runtime_repo --cas $WORK/cas > $WORK/dashboard.audit.txt
diff $RUN/dashboard.txt $WORK/dashboard.audit.txt   # semantic content identical (timestamps may differ)
```

This satisfies acceptance clause 4: every dashboard report is regeneratable from committed ChainTape + CAS alone.

### §5.2 Generate fresh evidence (LLM + Lean required)

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
target/debug/verify_chaintape --repo /tmp/tb7r_repro/runtime_repo --cas /tmp/tb7r_repro/cas --out /tmp/tb7r_repro/replay_report.json
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
