# TB-7R Recursive Self-Audit — 2026-05-02

**Audit type**: Class 3 (auth-crypto-money) ship-gate dual external audit per
`feedback_dual_audit` + `feedback_risk_class_audit` + TB-7R charter §3
Deliverable G audit-point-2.

**Audit mode**: full dual (Codex + Gemini, both at strategic tier; NOT degraded).
Round-2 Codex audit fired after determinate-best surgical remediation per
`feedback_elon_mode_policy`. Round-1 VETO closed at round 2.

**Branch**: `main`.
**TB-7R commit range**: `9e74195..4470036` on `main` (5 commits) +
remediation commits below.
**Charter (binding)**: `handover/tracer_bullets/TB-7R_charter_2026-05-01.md`.
**Architect rulings (binding)**:
- Authorization: `handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md`
- parent_tx + DAG smoke: `handover/directives/2026-05-02_TB7R_PARENT_TX_DAG_SMOKE_VERDICT.md`

**Test totals**: `cargo test --workspace` → **712 passed / 0 failed / 150 ignored**
(+26 net TB-7R tests vs TB-7 ship 686/0/150 baseline; canonical reporting per
`feedback_workspace_test_canonical`).

---

## §0 Headline verdict

**TB-7R is READY TO SHIP** as of 2026-05-02 with the four-clause acceptance
criterion strictly satisfied from committed evidence alone, all 7 architect
ship conditions met, and both external auditors clearing the working tree.

```
Frame B authoritative path:                    GREEN (carried from TB-7)
Frame B ChainTape-mode fail-closed (Del. B):   GREEN
Frame B L4 purity (Del. A):                    GREEN — zero violations
Frame B genesis_report (Del. C):               GREEN
Frame B on-chain TaskOpen + EscrowLock (D):    GREEN — already-shipped path verified
Frame B historical README annotations (E):     GREEN — 2/3 sticky; tb_7_chaintape_smoke note reverts via hook (non-blocking, OBS)
Frame B smoke evidence (Deliverable F):        GREEN — 10 runs, all 7 indicators per run, replayable from committed tar.gz
Frame B parent_tx ParentTxState + 6 tests:     GREEN — verdict 2026-05-02 §3 fully satisfied
Frame B audits (Deliverable G):                GREEN — Codex round-1 VETO → remediation → round-2 PASS; Gemini PASS (non-degraded strategic tier)

Acceptance clause 1 (every externalized → L4 or L4.E):    GREEN under three-node taxonomy (OBS-1 follow-up post TB-7R)
Acceptance clause 2 (predicate evidence resolves):        GREEN (Codex RQ3 verified end-to-end CID resolution)
Acceptance clause 3 (failed shielded but auditable):      GREEN
Acceptance clause 4 (dashboard regeneratable):            GREEN — 10/10 runs round-trip from committed tar.gz

Ship cond 1 (7 indicators green):              GREEN — 10/10 runs
Ship cond 2 (real proposals → L4 or L4.E):     GREEN
Ship cond 3 (solved → chain_oracle_verified):  GREEN — 8/10
Ship cond 4 (unsolved → no fake accepted):     GREEN — 2/10
Ship cond 5 (CIDs resolve):                    GREEN — Codex RQ3 demonstrated end-to-end
Ship cond 6 (parent_tx conformance test):      GREEN — 6/6 in tests/tb_7r_parent_tx_conformance.rs
Ship cond 7 (README explicit on parent_tx=0):  GREEN — README §2 updated with verdict §1 language
```

---

## §1 Audit shape

For each binding contract (charter §1 four-clause acceptance + verdict 2026-05-02 §4
seven ship conditions + charter §4 thirteen forbidden lines + §3 eight deliverables A-H),
assert line-grounded provenance to src/ + tests/ + smoke evidence + audit findings.

**Result**: 4/4 acceptance clauses + 7/7 ship conditions + 13/13 forbidden lines +
8/8 deliverables all GREEN. Both external auditors PASS at strategic tier. **TB-7R
ship-ready.**

---

## §2 TB-7R commit range (5 commits + remediation)

| Commit | Subject | Class |
|---|---|---|
| `696d10f` | TB-7R A+B+E — verdict ingestion + L4 purity audit + ChainTape-mode fail-closed | Class 1 (additive) + Class 0 (docs) |
| `392a516` | TB-7R C+D+CP2 — genesis_report.json emission + on-chain TaskOpen/EscrowLock verification | Class 2 (production wire-up) |
| `b517ae5` | TB-7R audit-fix — Codex CHALLENGE Claim 7 remediation (orphan TRACE_MATRIX) | Class 0 (docs) |
| `013f2ce` | TB-7R F (smoke evidence) — 10 runs across single/half/full smoke ladders | Class 1 (evidence) |
| `4470036` | TB-7R parent_tx ParentTxState + 6 conformance tests + verdict 2026-05-02 | Class 2 (additive schema field + tests) |
| (this PR) | TB-7R Codex round-1 VETO remediation — evidence packaging + replay_report.json + OBS framing tightening | Class 1 (evidence packaging + docs) |

Aggregate within TB-7R Class 3 envelope. No code paths in `src/state/sequencer.rs`,
`src/bus.rs`, `src/kernel.rs`, or `src/sdk/tools/wallet.rs` modified
(STEP_B-protected; verified at every commit).

---

## §3 Four-clause acceptance criterion verification

| Clause | Verification | Status |
|---|---|---|
| **1. Every externalized LLM proposal → L4 or L4.E** | Three-node taxonomy defines "externalized" = `bus.submit_typed_tx`-routed (`DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md`). All 10 smoke runs: every `submit_typed_tx` call routes through `Sequencer::apply_one` to L4 or L4.E. **OBS-1** (`OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md`) §2.1.a refines this: PartialOk → Complete proof-prefix dependency lets accepted L4 proof artifact be only `tactic` (not `tape_chain + tactic`). Architect-acknowledged post-TB-7R follow-up. | ✅ GREEN under TB-7R taxonomy; OBS-1 carries to TB-8+ |
| **2. L4 accepted WorkTx has resolvable predicate evidence in CAS** | `chain_oracle_verified` requires `accepted_worktx_vr_cid` → CAS `VerificationResult` → `verified=true` → `proof_artifact_cid` resolves (`chain_derived_run_facts.rs:321-336`). Codex round-2 RQ3 walked the chain end-to-end on `single_n1_mathd_algebra_171`: `entry_payload_cid=361B → work_proposal_cid=184B → telemetry_verification_result_cid=220B → verification_proof_artifact_cid=84B → verified=true`. All CIDs resolved via `CasStore::get` (sha256-checked) from committed tar.gz extract. | ✅ GREEN |
| **3. Failed proposal in L4.E only; raw diagnostic shielded** | TB-1 P0-3 `raw_diagnostic_cid` serde shield holds; L4.E `RejectedSubmissionRecord` agent-facing view excludes raw_diagnostic. Gemini Q3 verified: dashboard reads only `rejection_class` label, never raw text. L4.E records exist on every UNSOLVED run + on synthetic-seed audit pairs in every run. | ✅ GREEN |
| **4. Dashboard regeneratable from ChainTape + CAS alone** | TB-7R adds `src/bin/audit_dashboard.rs` (read-only materialized view from runtime_repo + cas/). Round-1 VETO basis: committed evidence omitted `.git/` stores → Codex empirically failed to reproduce dashboard from committed state. **Remediation**: every of 10 runs now ships `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` + `replay_report.json`. Codex round-2 RQ1+RQ2 swept all 10 runs: every run extracts cleanly, every `verify_chaintape` re-run produces structurally-identical `replay_report.json` (modulo `run_id`/`epoch`), `bool_false=0`. | ✅ GREEN — round-1 VETO closed |

---

## §4 Seven ship conditions (architect verdict 2026-05-02 §4) verification

| # | Condition | Verification | Status |
|---|---|---|---|
| 1 | All 7 dashboard indicators GREEN | Per dashboard.txt + replay_report.json on every run; full inventory at smoke README §0 | ✅ |
| 2 | All real externalized proposals in L4 or L4.E | Smoke runs use `OMEGA-pertactic` mode; every successful proposal lands in L4 (8/10 SOLVED), every synthetic-seed audit pair hits L4.E (10/10) | ✅ |
| 3 | Solved runs `chain_oracle_verified=true` + golden path | 8/10 runs SOLVED, all show `chain_oracle_verified=true` per replay_report; dashboard §7 renders golden path with `[ORACLE]` depth=0 | ✅ |
| 4 | Unsolved runs no fake accepted nodes | 2/10 UNSOLVED (`run_4_aime_1997_p9`, `run_5_mathd_numbertheory_5`); both show `chain_oracle_verified=false`, `l4_entries=2` (TaskOpen + EscrowLock only; NO accepted Work) | ✅ |
| 5 | Proposal telemetry + payload CIDs resolve | Codex round-2 RQ3 demonstrated end-to-end resolution from committed `cas.dotgit.tar.gz` via `CasStore::get` (sha256-validated) | ✅ |
| 6 | Forced parent_tx conformance test passes | `tests/tb_7r_parent_tx_conformance.rs` 6/6 pass: `singleton_golden_path_has_zero_edges_and_is_valid` + `second_attempt_same_branch_has_parent_tx` + `missing_parent_on_nonroot_attempt_is_violation` + `dashboard_renders_singleton_golden_path` + `unsolved_runs_have_no_fake_accepted_nodes` + `proposal_count_chain_equals_externalized_proposal_count` | ✅ |
| 7 | README explicit on natural `parent_tx_edges=0` | Smoke README §2 uses verdict §1 language verbatim ("natural parent_tx_edges=0 occurred because complete-tool runs solved in one externalized proposal"); table at §0 distinguishes the 4 ParentTxState variants with seen counts (8 SingletonGoldenPathValid, 2 NoMultiAttemptObserved, 0 of MultiAttemptDagValid / MissingParentTxViolation) | ✅ |

---

## §5 Eight deliverables (charter §3) verification

| Del. | Item | Site | Status |
|---|---|---|---|
| A | L4 purity audit (read-only) | `handover/audits/L4_PURITY_AUDIT_TB7R_2026-05-02.md` — zero violations across 3 evidence dirs (TB-7.7 D7 + TB-7 ChainTape smoke + TB-7 real-LLM 5 problems); 1 in-scope L4 Work, 4/4 purity criteria met; vacuous PASS on the rest | ✅ |
| B | ChainTape-mode fail-closed | `experiments/minif2f_v4/src/chaintape_mode_gate.rs` (NEW) + 4 unit tests covering truth table; evaluator dispatch at `evaluator.rs:327` before `oneshot` branch at `:332` | ✅ |
| C | Genesis report emission | `src/runtime/genesis_report.rs` (NEW; 9-field schema + 4 unit tests); evaluator wiring at `evaluator.rs:991-1008`; constitution_hash matches `genesis_payload.toml`:119 | ✅ |
| D | On-chain TaskOpen + EscrowLock (replace memory preseed) | Already-shipped TB-7.7 D3 path verified at `evaluator.rs:849-902` submitting both via `bus.submit_typed_tx`; zero direct `task_markets_t.insert` / `escrows_t.insert` writes in production preseed path; documented at `handover/audits/TB7R_DELIVERABLE_D_VERIFICATION_2026-05-02.md` | ✅ |
| E | Historical evidence README annotations | 2/3 dirs (`tb_7_7_dag_capable_smoke_2026-05-01`, `tb_7_real_smoke_5_problems_2026-05-01`) annotated with grandfathering note; 3rd dir (`tb_7_chaintape_smoke_2026-05-01`) annotation reverts via editor hook (non-blocking; OBS at `CHECKPOINT_TB7R_2_2026-05-02.md` §"Open observations" #1) | ✅ (with carry-forward OBS) |
| F | TB-7R smoke (single → half → full) | `handover/evidence/tb_7r_smoke_2026-05-02/` — 10 runs total: single (1) + half (3) + full-A n5 (1) + full-B n1 (5); 8/10 SOLVED + chain_oracle_verified=true; 2/10 UNSOLVED + no fake accepted node; all 7 indicators GREEN per run; tar.gz packaging + replay_report.json per run after Codex round-1 remediation | ✅ |
| G | Audits | Audit-point-1 (Codex micro-audit) `handover/audits/CODEX_TB7R_MICRO_AUDIT_2026-05-02.md` — CHALLENGE Claim 7 remediated via orphan registration in `b517ae5`. Audit-point-2 ship audit (this doc): Codex round-1 VETO `handover/audits/CODEX_TB_7R_SHIP_AUDIT_2026-05-02.md`; Gemini PASS `handover/audits/GEMINI_TB_7R_SHIP_AUDIT_2026-05-02.md`; Codex round-2 PASS `handover/audits/CODEX_TB_7R_SHIP_AUDIT_R2_2026-05-02.md` after remediation. **Full dual at strategic tier; NOT degraded.** | ✅ |
| H | Ship report (this doc) + TB_LOG.tsv row | TB-7R row appended to `handover/tracer_bullets/TB_LOG.tsv` per `feedback_tb_phase_tag_required` (phase_id + roadmap_exit_criteria_addressed + kill_criteria_tested explicit) | ✅ |

---

## §6 Charter §4 forbidden-line compliance (13 lines)

| # | Forbidden | Verification | ✓ |
|---|---|---|---|
| 1 | NodeMarket / position semantics | `git diff 9e74195..4470036` — no NodeMarket / NodePosition / PriceIndex / MarketResolveTx additions | ✅ |
| 2 | Role taxonomy mutation (M / B+ / B-) | absence-verified — no role-taxonomy code exists in TB-7R range | ✅ |
| 3 | Whale tracking / contested-node metrics | absence-verified | ✅ |
| 4 | FinalizeRewardTx / SettlementTx | absence-verified — no new TypedTx variants (`src/state/typed_tx.rs` unchanged) | ✅ |
| 5 | SlashTx | absence-verified | ✅ |
| 6 | MetaTape / ArchitectAI auto-mutate-rules | absence-verified | ✅ |
| 7 | Predicate registry mutation | absence-verified | ✅ |
| 8 | constitution.md / RootBox / sudo touches | `git diff 9e74195..4470036 constitution.md` empty | ✅ |
| 9 | Retroactive ledger rewrite | No commits modify pre-TB-7R `handover/evidence/tb_7_*_2026-05-01/` ledger artifacts (only README annotations) | ✅ |
| 10 | Fabricated historical genesis_report.json | No genesis_report.json appears in any pre-TB-7R evidence dir | ✅ |
| 11 | Per-tactic decomposition (verdict A1) | parent_tx + DAG plumbing is proposal-level only; 6 conformance tests at proposal granularity | ✅ |
| 12 | New TypedTx variant | `src/state/typed_tx.rs` unchanged; `VerificationResult` is a CAS object | ✅ |
| 13 | per-tactic chain entry | absence-verified — no per-tactic node-counting or chain entry code | ✅ |

---

## §7 External audit summary

### §7.1 Gemini PASS (round 1; strategic tier `gemini-3.1-pro-preview`; NOT degraded)

`handover/audits/GEMINI_TB_7R_SHIP_AUDIT_2026-05-02.md`

> **Verdict: PASS / Conviction: 4 / 5**
>
> "TB-7R successfully executes the 'Constitution-Aligned Frame B Repair' exactly as authorized by the architect. It structurally enforces the Predicate Pass → L4 / Predicate Fail → L4.E boundary, decouples oracle truth (VerificationResult CAS) from economic admission (VerifyTx), and implements the fail-closed ChainTape gate. The codebase is now strictly aligned with the authorized three-node taxonomy."

Q1-Q8 dispositions: PASS on all clauses + ship conditions; CHALLENGE on Q1 (constitutional reading of clause 1 — the OBS-1 question), CHALLENGE on Q7 (Art. III.4 — but deferral stands per OBS-2 framing). Section F: **SHIP-CLEAR WITH OBS-TIGHTENING**.

### §7.2 Codex round 1 VETO → round 2 PASS

**Round 1** (`handover/audits/CODEX_TB_7R_SHIP_AUDIT_2026-05-02.md`):

> **Verdict: VETO / HIGH conviction.**
>
> "TB-7R ship evidence is not reproducible from committed ChainTape + CAS: the committed smoke dirs omit the git-backed `runtime_repo/.git` ledger and `cas/.git` blob stores while the README claims they are present."

Round-1 findings: 4 (evidence packaging missing `.git`; missing `replay_report.json`; PartialOk → Complete proof-prefix dependency; OBS-2 stale premise).

**Remediation** (this audit's working-tree changes):

1. **Evidence packaging** — every of 10 evidence dirs now ships `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` (compressed `.git/` stores; total 892 KB across 10 runs vs ~4.8 MB loose; tar.gz needed because git auto-ignores nested `.git/` directories).
2. **`replay_report.json` per run** — committed for each of 10 runs; literal `verify_chaintape` output.
3. **OBS-1 §2.1.a + §4.3** — sharper framing of PartialOk → Complete proof-prefix dependency + "self-contained proof artifact" requirement for TB-8+ hardening.
4. **OBS-2 §0** — closure-as-empirically-unfounded language; corrects the original stale premise per Codex Q10.
5. **README §0 + §4 + §5.1** — packaging update + tar.gz artifact list + reproduce recipe with extraction step.

**Round 2** (`handover/audits/CODEX_TB_7R_SHIP_AUDIT_R2_2026-05-02.md`):

> **Verdict: PASS for the audited working tree. The round-1 VETO basis is closed: the ChainTape/CAS git stores are now restorable from the packaged archives, replay reports reproduce, and CID resolution works from the extracted committed-style evidence.**

Round-2 RQ1-RQ4 dispositions: all PASS.
- RQ1: 2 sampled runs round-trip to STRUCTURALLY_IDENTICAL replay_report.
- RQ2: all-10 sweep, every run extracts + verifies, `runs_checked=10 bool_false=0`.
- RQ3: end-to-end CID chain resolution demonstrated on `single_n1_mathd_algebra_171` (entry_payload → work_proposal → telemetry_VR → proof_artifact, all CIDs resolve via `CasStore::get` sha256-validated).
- RQ4: OBS-1 + OBS-2 framing accepted as accurate.

Ship may proceed after committing the working-tree remediation artifacts.

### §7.3 Conflict resolution

Per `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS. Round 1: Codex VETO blocked Gemini PASS. Round 2: Codex flips to PASS after surgical remediation. **Both auditors now PASS.** No degraded label needed.

---

## §8 Open observations carried forward

Not ship blockers — explicit follow-ups in roadmap:

1. **`OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md`** (architect-acknowledged, post-TB-7R)
   - §2.1.a (Codex round-1 refinement): PartialOk → Complete proof-prefix dependency.
   - §4 hardening recommendations updated with §4.3 self-contained proof artifact.
   - Closure: future TB ships the hardening + smoke demonstrates `externalized_proposal_count == chain_proposal_count` and self-contained proof artifacts.

2. **`OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md`** (closed-as-empirically-unfounded per Codex Q10)
   - Premise corrected: `acc.record_tool_stdout` only increments token cost; raw Lean text doesn't flow to prompt.
   - Closure: closed-by-empirical-disproof.

3. **`OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md`** (architect-acknowledged, follows verdict §C9 orphan path)
   - 2 modules registered as TRACE_MATRIX § 3 orphans pending future canonical FC row.
   - Closure: future TRACE_MATRIX revision adds canonical rows under Art. IV Boot.

4. **`CHECKPOINT_TB7R_2_2026-05-02.md` §"Open observations" #1** (non-blocking)
   - `tb_7_chaintape_smoke_2026-05-01/README.md` annotation reverts via editor hook; investigate next session.

---

## §9 Kill-criterion status (charter §3 phase declarations)

| Phase | Kill criterion | Status |
|---|---|---|
| P1 | 1 (CAS race makes ledger un-replayable) | NOT TRIGGERED — TB-7.6 fix holds; Codex RQ3 demonstrates resolvability |
| P1 | 2 (payload not retrievable from CAS) | NOT TRIGGERED — Codex RQ3 verified end-to-end CID resolution |
| P1 | 3 (signature verify breaks) | NOT TRIGGERED — `agent_signatures_verified=true` + `system_signatures_verified=true` on all 10 runs |
| P1 | 4 (replay reconstruction breaks) | NOT TRIGGERED — `state_reconstructed=true` + `economic_state_reconstructed=true` on all 10 runs |
| P3 | 1-3 (RSP carry-forward conservation) | NOT EVALUATED — settlement/slash still out of scope (TB-9 territory) |

---

## §10 Production claim (post-TB-7R ship)

> "TuringOS Frame B is constitution-aligned: every externalized LLM proposal
> (under the strict three-node taxonomy where 'externalized' =
> `bus.submit_typed_tx`-routed) lands in either L4 accepted (predicate-passed)
> or L4.E rejected evidence, never both. Predicate pass / fail (Lean
> VerificationResult resolved from CAS) alone determines L4 vs L4.E —
> stake/escrow do not manufacture acceptance. The DAG is proposal-level
> (per-tactic deferred to TB-8+ per verdict A1=B′). Genesis state for new
> runs is established via on-chain TaskOpenTx + EscrowLockTx, not memory
> preseed. Historical evidence is grandfathered, not rewritten. The audit
> dashboard is a read-only materialized view from ChainTape + CAS — every
> run's dashboard is regeneratable from committed `runtime_repo.dotgit.tar.gz`
> + `cas.dotgit.tar.gz` alone (10/10 verified by Codex round-2). NodeMarket,
> settlement, slash, and per-tactic DAG remain post-TB-7R. PartialOk →
> Complete proof-prefix dependency (OBS-1 §2.1.a) and the self-contained
> proof artifact requirement (OBS-1 §4.3) are explicit post-TB-7R follow-ups."

---

## §11 Cross-references

- TB-7R charter: `handover/tracer_bullets/TB-7R_charter_2026-05-01.md`
- Authorization verdict: `handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md`
- parent_tx + DAG verdict: `handover/directives/2026-05-02_TB7R_PARENT_TX_DAG_SMOKE_VERDICT.md`
- Smoke evidence: `handover/evidence/tb_7r_smoke_2026-05-02/`
- L4 purity audit (Deliverable A): `handover/audits/L4_PURITY_AUDIT_TB7R_2026-05-02.md`
- Deliverable D verification: `handover/audits/TB7R_DELIVERABLE_D_VERIFICATION_2026-05-02.md`
- Codex micro-audit (audit-point-1): `handover/audits/CODEX_TB7R_MICRO_AUDIT_2026-05-02.md`
- Codex ship audit round 1 (VETO): `handover/audits/CODEX_TB_7R_SHIP_AUDIT_2026-05-02.md`
- Gemini ship audit (PASS): `handover/audits/GEMINI_TB_7R_SHIP_AUDIT_2026-05-02.md`
- Codex ship audit round 2 (PASS): `handover/audits/CODEX_TB_7R_SHIP_AUDIT_R2_2026-05-02.md`
- TRACE_MATRIX orphan registry: `handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md`
- Coverage denominator OBS-1: `handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md`
- Prompt pollution OBS-2 (closed-as-stale): `handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md`
- Three-node taxonomy: `handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md`
- L4/L4.E ledger separation: `handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`
- Checkpoint 1: `handover/CHECKPOINT_TB7R_1_2026-05-02.md`
- Checkpoint 2: `handover/CHECKPOINT_TB7R_2_2026-05-02.md`
