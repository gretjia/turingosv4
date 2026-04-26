# AUDIT REPORT — TFR_MASTER_PLAN_2026-04-26 (Internal Auditor)

**Auditor**: Claude `auditor` subagent (read-only mode), parallel to `codex:codex-rescue` external auditor
**Date**: 2026-04-26
**Document under review**: `handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md` (commit `3674d5f`)
**Scope**: S0 exit gate dual-audit (one of two parallel auditors)

```
VERDICT: CHALLENGE
CONVICTION: medium-high
```

---

## CONSTITUTIONAL_CLAUSES_NOT_CLOSED

The plan covers Art. 0 / 0.1 / 0.3 / 0.4 robustly. Gaps in **Art. 0.2**:

1. **Art. 0.2 op #6 — WAL 强制 (non-opt-in)**: constitution says "WAL 持久化是 Phase E 强制项（不可 opt-in）". The plan never lands an atom that flips `WAL_DIR` from opt-in to mandatory. Path B's defense ("GitTape supersedes WAL after S5") is plausible but leaves S2-S5 transition window with WAL still opt-in. The constitution's mandate is unconditional. **At minimum, S2.3 should explicitly mandate WAL non-opt-in OR S0.7 should declare WAL semantically replaced by runtime_repo.** Currently neither is stated.

2. **Art. 0.2 op #2 — assert_eq! conformance for ALL parallel ledgers**: constitution requires "每个派生视图都必须有 `assert_eq!(view, derive_from_tape(tape))` 守恒测试". Plan covers wallet (S4.4), librarian (S5.5), tape round-trip (S2.5). **Missing explicit atoms for**: `RunCostAccumulator → derived view conformance`, `bus.graveyard → derived view conformance`, `search_cache → derived view conformance`, `FC trace → derived view conformance`. Atom S2.5 is a single round-trip test, not the per-derived-view conformance suite the constitution requires.

3. **Art. 0.2 修复义务 Commit 2** (RunCostAccumulator → derived view): the constitution explicitly enumerates this as Commit 2 of the 10-commit atomization. The TFR plan substitutes Path B's "cost lives in NodeCost on tape" but **never atomizes the explicit "RunCostAccumulator becomes derived view"** with conformance test. Audit recommendation 2 + AUDITOR Commit 2 are not directly closed.

4. **Art. 0.2 修复义务 Commit 5 (强制 WAL)**: same as gap (1) above.

## V_VIOLATIONS_NOT_CLOSED

Cross-checked against `TAPE_CANONICAL_AUDIT_2026-04-26_AUDITOR.md` 24-violation inventory:

- **V-06 RunWallClock parallel ledger**: S2.1 bumps `created_at` to u128 ms (resolution fix), but the audit's actual concern is `experiments/minif2f_v4/src/wall_clock.rs` (RunWallClock using `Instant`, non-serializable). No atom migrates `RunWallClock` to a tape-derivable struct. Constitutional T_i bracket reproducibility remains broken.
- **V-09 graveyard not derivable**: S3.3 puts failed branches on tape with `verified: false`, which sets the foundation, but no atom explicitly **deletes `bus.graveyard: HashMap`** or rewrites `recent_rejections_scoped` as a tape iterator. AUDITOR Commit 4 explicitly mandates this.
- **V-12 gp_token_count byte-count hack**: only **implicitly** addressed via Appendix A U3 → S3.5 + S4.6. The hack dies "because legacy bus.append dies", which is correct in principle but no atom has an explicit conformance test asserting `gp_token_count = Σ tape.iter(kind=AgentProposal).cost.completion_tokens`. Risk: hack survives in some calling path missed by S4.6 grep.
- **V-17 WAL opt-in**: see Constitutional gap (1) — plan does not explicitly address.
- **V-20 proof artifact sidecar**: `experiments/minif2f_v4/src/bin/evaluator.rs:1184-1188` `persist_proof_artifact` writes `proofs/*.lean` as JSONL-referenced sidecar, not on tape. **No atom in the plan addresses this.** This is a real Phase E reproducibility hole.
- **V-23 dormant `MinerTx { completion_tokens: u32 }`** at `src/sdk/actor.rs:151-158`: **No atom addresses this dormant schema artifact.** Will likely reappear as Phase E auditor finding post-TFR.

**5 of 24 V-violations have residual gaps**: V-06, V-09, V-12 (assertion gap), V-17, V-20, V-23. (V-12 is partial; the others are unaddressed.)

## STEP_B_PROTOCOL_LEAKS

The plan's authoritative restricted-file list per D9 + § 5.4: `src/{bus,kernel,wal,ledger}.rs` plus `src/sdk/tools/wallet.rs` (per Codex audit § 5 step 4 inclusion).

**Sprint-level claims**:
- S1, S2, S3, S4: explicitly declare STEP_B per-atom for restricted files. ✓
- **S5: NO sprint-level STEP_B declaration**. Yet:
  - **S5.1 leaks**: touches `src/bus.rs` (restricted, per D9) — no STEP_B claim attached
  - **S5.6 leaks**: touches `src/wal.rs` (restricted, per D9) — no STEP_B claim attached
  - S5.4 touches `src/sdk/tools/search.rs` (NOT restricted per current CLAUDE.md, but borderline)
  - S5.5 touches `src/sdk/tools/librarian.rs` (NOT restricted per current CLAUDE.md)
- **S6.3 (TR migration)**: Trust Root commits per § 5.4 require external dual-audit always. The plan does declare PASS/PASS at S6.3. ✓ but it is not explicitly framed as STEP_B.

**STEP_B leakage list**: **S5.1, S5.6** — these atoms touch restricted files (`src/bus.rs`, `src/wal.rs`) without STEP_B claim. Plan should add an explicit "**STEP_B_PROTOCOL files touched in S5: `src/bus.rs` (S5.1), `src/wal.rs` (S5.6)**" footer matching S1/S2/S3/S4 sprint pattern.

## RISKS_MISSING

R1-R14 documented. Missing or under-specified:

- **R15 (V-20 proof artifact off-tape)**: plan does not own this; Phase E audit will find it.
- **R16 (V-23 dormant MinerTx)**: same.
- **R17 (S6.1 heldout problem_id leakage via genesis.json)**: § 2.2 lifecycle puts `problem_id` into the genesis commit; if agents have any read access to `runtime_repo/.git/`, they can reverse-engineer the heldout identity. R7 mentions "commit messages leak heldout problem IDs" but does not break out the genesis.json case specifically. **The plan needs an explicit atom (or sub-atom under S6.1) that audits whether agent tools can read into `.git/` or the runtime repo objects directory.**
- **R18 (Phase D queueing during S0-S5 freeze)**: ArchitectAI cannot write user_space artifacts. What about ArchitectAI architectural proposals during this window? Are they batched and applied at S6 ceremony, or dropped? Risk register silent.
- **R19 (TR mutation-storm dual-audit gap)**: 7 TR mutations during TFR. Each is dual-audited per § 7.3, but if a stealth entry is added in atom N+1 and audit is at sprint boundary, between-atom states are reviewed only by internal `auditor` subagent. **Dual-audit catches per-sprint diffs but cannot guarantee no transient TR-immutability violation between atoms.** Constitutionally OK (sprint exit is the audit gate) but worth explicit in risk register.
- **R20 (CI does not enforce both `--features tape-git` and default on every PR)**: § S3.5 manual smoke at sprint exit only. Phantom-substrate-divergence risk goes undetected during in-sprint atom commits. **Recommend mandate that every CI run executes both feature paths.**
- **R21 (Auditor budget overrun)**: § 5.4 says $300-500 expected; PREREG cap raised to $800. If round-2/3 audits proliferate (A8 had 14 rounds), budget breaches. R3 names this loosely; needs hard escalation gate.

## RECOMMENDATIONS

**Must-fix (block S0 entry)**:

1. **Add explicit atoms for V-09 (delete graveyard), V-17 (mandate WAL or formally retire it), V-20 (proof artifact on tape), V-23 (delete dormant MinerTx)**. Without these, plan's "every V-XX has a closing atom" claim is false in 5 cases.
2. **Add S5 sprint-level STEP_B declaration** matching S1-S4 pattern. S5.1 and S5.6 touch restricted files (`bus.rs`, `wal.rs`) and need explicit per-atom dual-audit framing.
3. **Add explicit atom for "RunCostAccumulator → derived view + assert_eq! conformance"** per Art. 0.2 op #2 mandate and AUDITOR Commit 2. The "Path B subsumes this" argument is structurally sound but leaves the constitutional conformance test unwritten.
4. **Add S6.1 sub-atom**: explicit grep audit that `runtime_repo/.git/` is unreachable from agent tool surfaces (search.rs, file ops). The genesis.json containing `problem_id` is the highest-leverage Phase E leak vector.
5. **Resolve V-06 RunWallClock**: plan addresses `created_at` ms-precision but does not retire `RunWallClock` parallel ledger. Add explicit atom.

**Should-fix (CHALLENGE level)**:

6. **Address auditor's 7 Open Questions**: plan currently addresses only Q1 (legacy artifacts) implicitly via S6.4 "preserves discarded_12way_run_2026-04-24/". Q2-Q7 unanswered (caps for Lean error string, search hit corpus storage, Boltzmann seed strategy, RunCostAccumulator cache lifetime, etc.). Each should be either resolved in plan or explicitly listed in § 10.2 Open Questions for user decision.
7. **CI gate hardening**: mandate every CI run executes both `cargo test --workspace` AND `cargo test --workspace --features tape-git`. Currently only sprint-exit gates require it.
8. **Add explicit V-XX → atom matrix** in § 4 footer (currently only Appendix A maps U1-U9, leaving V-XX coverage scattered). A single table reading `V-01 → S2.1; V-02 → ...; V-24 → S5.7` makes residual gaps trivially auditable.
9. **TRACE_MATRIX_v3 bidirectional test scope**: § 6.4 describes parsing markdown + doc-comments. Add a literal grammar / regex spec in S0.5 to make the test deterministic. Currently "implementable" but under-specified.
10. **Explicit documented decision** on how `RunCostAccumulator` lifetime ends: kept as cache (with assert_eq) or deleted? Auditor Open Question #7 unresolved.

## NOTES

**What the plan gets RIGHT (worth preserving)**:

- The Node ↔ git mapping with multi-parent merge commits for citations (§ 2.3) is genuinely elegant; it makes the proof DAG topology = git DAG topology. This is the highest-leverage architectural decision in the plan.
- Path B's defense against the codex auditor's "completion_tokens patch declares victory" failure mode IS structurally sound (per-V-XX mapping, bidirectional TRACE_MATRIX, sprint structure). Provided the must-fix recommendations above are absorbed, the plan does not fall into that trap.
- The backward-compat shim strategy (§ 2.11) — `bus.append_v2` introduced in S1, legacy `bus.append` deleted in S4.6 — is the correct migration pattern and preserves "cargo test green at every commit" properly.
- Two-tier Π_p (in-process + on-disk hook, § 2.8) is belt-and-suspenders correct.
- PREREG amendment (§ 9 Proposal A) is well-formed and the alternative (§ 9.2 Proposal B) is honest about trade-offs.

**Why CHALLENGE not VETO**:

The 5 residual V-violation gaps (V-06, V-09 explicit, V-17, V-20, V-23) are real but addressable with 5-7 additional atoms inserted into existing sprints. They do not require structural redesign. The S5 STEP_B leak is a documentation fix. None of the gaps invalidate Path B as the right architectural choice.

**Why NOT PASS**:

The user invested 795 lines of architectural design and called this "the most important architectural correction in project history". Letting V-06, V-17, V-20, V-23 slip through means Phase E auditors will find these post-TFR-exit, undermining the "we paid 8 weeks once for clean foundation" narrative. The plan claims (Appendix A) "every U-atom maps... nothing is dropped" — but 5 V-XX from the AUDITOR's 24-violation list ARE dropped. That false-completeness claim is what tips this from PASS to CHALLENGE.

**Conviction note**:

I am medium-high confidence on the V-coverage gap analysis (cross-verified against AUDITOR doc inventory table 24 rows). I am medium confidence on the heldout-leakage-via-genesis-commit risk (would need agent-tool access boundary analysis I did not perform). I am high confidence on the S5 STEP_B leak (sprint footers compared directly).

**Files referenced in this audit** (absolute):
- /home/zephryj/projects/turingosv4/handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md
- /home/zephryj/projects/turingosv4/constitution.md (Art. 0.1–0.4 lines 39–151; Art. IV flowchart lines 540–610)
- /home/zephryj/projects/turingosv4/handover/architect-insights/TAPE_CANONICAL_AUDIT_2026-04-26_AUDITOR.md (V-01 through V-24 inventory)
- /home/zephryj/projects/turingosv4/handover/architect-insights/TAPE_CANONICAL_AUDIT_2026-04-26_CODEX.md (codex-rescue independent review)
- /home/zephryj/projects/turingosv4/handover/ai-direct/STEP_B_PROTOCOL.md (restricted-file change protocol)
- /home/zephryj/projects/turingosv4/CLAUDE.md (D9 restricted file list `src/{kernel,bus,wallet}.rs` — note: TFR plan correctly extends this to include `wal.rs` and `ledger.rs`; the gap is S5 not declaring STEP_B)

Conservative-merge instruction: with parallel `codex:codex-rescue` running, if either auditor returns CHALLENGE or VETO, this verdict (CHALLENGE) holds per dual-audit protocol. S0.1 may not commit until the must-fix list is absorbed.
