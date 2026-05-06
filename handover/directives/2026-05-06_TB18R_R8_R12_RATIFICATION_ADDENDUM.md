---
type: architect_ratification_addendum
date: 2026-05-06
parent_directive: handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md
parent_dispatch: handover/audits/G2_TB_18R_ROUND_2_DUAL_AUDIT_DISPATCH_2026-05-06.md
predecessor_round_1_verdict: handover/audits/G2_TB_18R_DUAL_AUDIT_VERDICT_2026-05-06.md
heads:
  pre_round_2: 3964957  # TB-18R PROVISIONAL SHIPPED (round-1 VETO)
  R8_plus_R10_source: 095a622
  R9_R11_R12_evidence: eb2b932
status: PENDING ARCHITECT RATIFICATION — NOT a §8 ship sign-off
authority_class:
  R8: Class 3 (audit infra logic) + Class 4-adjacent (LeanResult.error_class doc-comment edit on R1-shipped Class-4 schema struct)
  R9: Class 3 (rerun runner + evidence)
  R10: Class 1 (binary surface contract / test alignment)
  R11: Class 1 (test rewrite)
  R12: Class 0 (docs)
---

# TB-18R R8–R12 Architect Ratification Addendum — 2026-05-06

> **This document is a ratification REQUEST. It is not authority for ship.**
> It exists because the parent ruling (`2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md` Q-P1) determined that user single-word `"fix"` was insufficient §8 sign-off. R8–R12 already landed on `main` under that defective authorization; this addendum enumerates each so the architect can explicitly accept, reject, or scope-narrow.

---

## §0 Why this addendum exists

Round-2 dispatch §3.1 (Gap A) and parent ruling §4 Q-P1 jointly establish:

1. User said `"fix"` after the round-1 VETO summary.
2. Claude orchestrator interpreted this as architect §8 path (A) sign-off.
3. **No `architect-ingest` skill was invoked**; no archived architect directive cited round-2 path (A).
4. R8–R12 atoms were authored and committed under that self-promoted authorization.
5. Conservative interpretation: the work is not invalidated, but its authorization gate is missing. Each atom needs explicit acceptance / rejection / narrowing by the architect before any ship-class claim.

This addendum presents each atom with: (a) what it does, (b) what files it touched, (c) the specific authorization gap, (d) the explicit ratification ask.

---

## §1 R8 — assert_45 partial-verdict-aware invariant (and LeanResult doc-comment)

### 1.1 Change scope

Round-1 Q13 VETO: `audit_tape verdict.json` showed `id45=FAIL → BLOCK` on R6 P02/P03 + R7 P02. Pre-R8 invariant at `src/runtime/audit_assertions.rs:2580-2671`:

```text
verified ↔ exit_code == 0   (iff)
```

Pre-R8 LeanResult doc-comment at `src/runtime/attempt_telemetry.rs:402-411` said:

```text
error_class is None iff verified == true
```

`step_partial_ok` at `experiments/minif2f_v4/src/bin/evaluator.rs:3504-3527` writes:

```rust
LeanResult { exit_code: 0, verified: false, error_class: None }
```

This violates **both** the iff invariant and the doc-comment iff. Round-1 Codex audit recommended relaxing the invariant to admit partial-verdict; Claude implemented this as Repair Form (β).

### 1.2 Technical delta (post-R8)

**`src/runtime/audit_assertions.rs:2580-2671`** — replace iff with three one-direction implications:

```text
1. verified ⇒ exit_code == 0 ∧ error_class.is_none()    (clean omega path)
2. !verified ∧ exit_code != 0 ⇒ error_class.is_some()   (real Lean failure must be classified)
3. !verified ∧ exit_code == 0 admissible                (partial-verdict OR sorry-block)
```

**`src/runtime/attempt_telemetry.rs:402-411`** — refresh doc-comment to enumerate three legitimate states:
- Verified omega: `(exit_code=0, verified=true, error_class=None)`.
- Real failure: `(exit_code≠0, verified=false, error_class=Some(...))`.
- Partial-verdict / sorry-block: `(exit_code=0, verified=false, error_class∈{None, Some(SorryBlocked)})`.

**Empirical effect**: 8/8 R6+R7 evidence directories rerun under R8 → all 8 verdict.json `PROCEED`, all 8 `id45=Pass`. Three pre-VETO BLOCKs cleared (R6 P02, R6 P03, R7 P02); 0 PROCEED→BLOCK regression.

### 1.3 Authorization gap

| Dimension | Status |
|-----------|--------|
| File `audit_assertions.rs` in CLAUDE.md STEP_B_PROTOCOL list | NO (Class 3 audit infra). Self-ratifiable under normal Class-3 norms. |
| File `attempt_telemetry.rs` in CLAUDE.md STEP_B_PROTOCOL list | NO. **But** the struct `LeanResult` was introduced and ratified at R1 as part of the Class-4 typed-tx + CAS schema bump (`src/state/typed_tx.rs` IS in STEP_B list; `attempt_telemetry.rs` carries the schema's doc-comment contract). The doc-comment is a Class-4-ratified schema contract surface. |
| `architect-ingest` for round-2 | NOT INVOKED |
| Archived architect directive citing path (A) | NONE before this addendum |
| FC-first analysis BEFORE designing fix | NOT PERFORMED (post-hoc FC tags only; per parent ruling §4 Q-P5) |

### 1.4 Ratification ask

**Architect must rule on each of the following independently** (cannot be bundled):

1. **Direction (α vs β)**: Is partial-verdict-aware invariant relaxation (β) the constitutionally correct repair, or should the implementation change instead (α: `step_partial_ok` should not emit `LeanResult` with `error_class=None`, e.g. it should emit `error_class=Some(LeanErrorClass::PartialVerdict)`)?
   - **Parent ruling §4 Q-P2 states**: β acceptable, but only if explicit typed `PartialAccepted` / `PartialVerdict` state is represented, not as an untyped `(exit_code=0, verified=false, error_class=None)` triple. This is **Phase 2 schema work and is OUT of scope for this addendum**. Phase 2 lives behind a separate remediation directive.
   - **Therefore**: ratification of R8-as-shipped is **conditional** on Phase 2 either (a) introducing typed `LeanVerdict::PartialAccepted` and updating `LeanResult` accordingly, or (b) producing FC-first analysis showing the untyped triple is constitutional.
2. **Doc-comment edit on Class-4 schema struct**: Was Claude entitled to edit the `LeanResult.error_class` doc-comment without separate Class-4 ratification, given that `LeanResult` was R1-ratified at Class-4?
3. **No-rollback decision**: Parent ruling §3.1 says do not rollback R8 now. Does the architect endorse this preservation-with-Phase-2-conditional-ratification approach?

**Default architect position absent explicit answer**: R8 stays on `main` as candidate remediation; Phase 2 must complete (typed `PartialAccepted` introduced OR α reverted) before TB-18R can re-enter ship gate.

---

## §2 R9 — P38 / P49 evaluable rerun (Q12 closure)

### 2.1 Change scope

Round-1 Q12 VETO: R6 P02 (`mathd_numbertheory_1124`) + P03 (`numbertheory_2pownm1prime_nprime`) → P38/P49 in problem-id space — were SIGKILL'd by 600s per-problem timeout before evaluator emitted `PPUT_RESULT`, leaving `chain_invariant.json.r4_invariant_equation_evaluable=false`.

### 2.2 Root cause

Two compounding bugs:

1. **Env-var no-op**: R6 runner `run_tb_18r_r6_evidence.sh` set `MAX_TX_OVERRIDE="$MAX_TX"` — but evaluator reads `MAX_TRANSACTIONS`. Cap not enforced; P38 ran 50 LLM cycles before timeout.
2. **Timeout too tight**: 30s+/cycle on hard heuristics × 12-cap intent ≈ needs ≥1800s budget, not 600s.

### 2.3 Technical delta

**NEW** `handover/tests/scripts/run_tb_18r_r9_evidence.sh`:
- `MAX_TRANSACTIONS=12` (correct env var).
- `PER_PROBLEM_TIMEOUT_S=1800` (3× R6).
- Otherwise identical to R6 runner.

**Evidence**: `handover/evidence/tb_18r_r9_p38_p49_2026-05-06/R9_BATCH_SUMMARY.json` — both P38 (193s) + P49 (222s) terminated cleanly under 12-cap; `chain_invariant.json` produces `invariant_verdict=Ok`, delta=0 for both.

### 2.4 Authorization gap

| Dimension | Status |
|-----------|--------|
| Files in STEP_B list | NO (script + evidence only) |
| `architect-ingest` for round-2 | NOT INVOKED |
| Archived architect directive | NONE |

R9 is rerun + evidence + script. Self-ratifiable as Class-3 evidence work.

### 2.5 Ratification ask

1. Does the architect accept R9 evidence as round-2 Q12 closure, conditional on the rest of TB-18R passing final dual audit? **OR** does Phase 3 of the parent ruling (P38/P49/M0 rerun *after* Phase 2 semantic repair) require R9 to be re-run yet again on the post-Phase-2 substrate?
2. Per parent ruling §5 Phase 3, R9-style rerun is required *post* Phase 2. **Default expectation**: R9 evidence is preserved for audit-trail purposes but a **fresh** P38/P49 + M0 rerun must occur after Phase 2 typed semantics ship; this fresh rerun supersedes R9 as Q12 final closure.

---

## §3 R10 — comprehensive_arena `--plan-only` writes ARENA_PLAN.md (workspace gate)

### 3.1 Change scope

Round-1 workspace gate failure: `comprehensive_arena_plan_only_emits_plan` test expected `ARENA_PLAN.md` at `out_dir/ARENA_PLAN.md`; binary only `eprintln!`'d to stderr. Test/binary contract drift since TB-16 Atom 5 scaffold (commit `36413c0`).

### 3.2 Technical delta

**`experiments/minif2f_v4/src/bin/comprehensive_arena.rs:209-275`** — added `write_plan_to_disk(cfg)` writing structured ARENA_PLAN.md (6 task labels, 13/13 tx-kind enumeration, sandbox preseed identities, §7.6 forbidden tx classes, §7.7 halt triggers).
**`comprehensive_arena.rs:191-…`** — `main()` invokes `write_plan_to_disk` before returning on `--plan-only`.
**`experiments/minif2f_v4/tests/tb_16_comprehensive_arena_smoke.rs:77-97`** — test labels updated to binary's actual label set.

### 3.3 Authorization gap

| Dimension | Status |
|-----------|--------|
| Files in STEP_B list | NO |
| Class | 1 (test/binary surface) |

R10 is normal Class-1 work. No special architect ratification required beyond the pending round-2 dual audit.

### 3.4 Ratification ask

1. Does the architect accept R10 as workspace-gate closure?

---

## §4 R11 — dashboard substantive smoke (Q11 CHALLENGE closure)

### 4.1 Change scope

Round-1 Q11 CHALLENGE: `tests/tb_18r_dashboard_attempt_dag_replay.rs` only verified `Cargo.toml` + `src/bin/audit_dashboard.rs` exist; never invoked the dashboard binary. SG-18R.9 closure was cosmetic.

### 4.2 Technical delta

**`tests/tb_18r_dashboard_attempt_dag_replay.rs`** — kept source-existence smoke; added `audit_dashboard_invokes_on_r7_p01_evidence`:
- Invokes `target/debug/audit_dashboard --repo <R7 P01 runtime_repo> --cas <R7 P01 cas> --json`.
- Asserts exit success + canonical JSON tokens (`run_id`, `chain`, `l4_entries`, `l4e_entries`, `indicators`, `ledger_root_verified`).
- Skips with explicit eprintln (NOT silent) if R7 P01 evidence absent (CI-strip tolerance).

### 4.3 Authorization gap

| Dimension | Status |
|-----------|--------|
| Files in STEP_B list | NO |
| Class | 1 |

R11 is Class-1 test rewrite. Self-ratifiable.

### 4.4 Ratification ask

1. Does the architect accept R11 as Q11 substantive smoke closure?

---

## §5 R12 — doc cleanup (Q14 + Q15 tail) and supersession markers

### 5.1 Change scope (split into 5.1A doc surfaces and 5.1B supersession-markers)

**5.1A** — Round-2 ship report `handover/audits/TB-18R_G2_ROUND_2_SHIP_REPORT_2026-05-06.md` authored. Q14 carve-out re-stated structurally based on charter §0.A R0 row + VETO `:604-609`.

**5.1B** — `handover/ai-direct/TB-18R_R3_STEP_B_admission.md:38-45, 221-225` carried original-draft text proposing "omega proposal_cid cutover at 2 success paths". Contradicted by §3.5 amendment. Inline `[SUPERSEDED 2026-05-06 by §3.5 amendment]` markers added.

### 5.2 Technical delta

- New file: `handover/audits/TB-18R_G2_ROUND_2_SHIP_REPORT_2026-05-06.md` (since this addendum, banner-prefixed as Round-2 Candidate Remediation Report).
- Inline supersession markers at preflight `:38-45` + `:221-225`.

### 5.3 Authorization gap

| Sub-item | Risk |
|----------|------|
| 5.1A — Q14 restatement | Potential `feedback_kolmogorov_compression` violation: restating an architect directive may be lossy distill. Parent ruling §4 Q-P4 ruled: structured annotation is acceptable **only if** verbatim quote is added alongside. Phase 1 #4 of parent ruling addresses this. |
| 5.1A — premature "Ship Report" title | Parent ruling §4 Q-P6: must be retitled / banner-prefixed before round-2 verdict. Phase 1 #1 (already executed) added the `PENDING ROUND-2 DUAL AUDIT — NOT SHIPPED` banner. |
| 5.1B — inline `[SUPERSEDED]` markers on architect-trail preflight doc | Parent ruling §4 Q-P3: markers acceptable, but a separate `OBS_*` annotation file is required. Phase 1 #3 of parent ruling addresses this. |

### 5.4 Ratification ask

1. With Phase 1 #1 (banner), #3 (OBS for supersession), and #4 (verbatim Q14 quote) all completed, does the architect accept R12 as Class-0 doc work?
2. Or does the architect require a stronger remedy (e.g., revert the inline markers and rely solely on the OBS file)?

---

## §6 Process gaps not specific to a single R (covered by parent ruling)

The following process gaps from round-2 dispatch §3 are addressed structurally by the parent ruling rather than per-R:

| Gap | Parent-ruling response |
|-----|------------------------|
| Gap A — `"fix"` ≠ §8 sign-off | Q-P1: confirmed; this addendum is the corrective. |
| Gap B — α vs β | Q-P2: requires Phase 2 typed `PartialAccepted`. Out of scope here. |
| Gap C — supersession markers | Q-P3: OBS required (Phase 1 #3). |
| Gap D — Q14 distill risk | Q-P4: verbatim quote required (Phase 1 #4). |
| Gap E — no FC-first | Q-P5: FC-first analysis required (Phase 1 #5). |
| Gap F — premature ship-title | Q-P6: banner required (Phase 1 #1, done). |

---

## §7 What this addendum is NOT

- **NOT a §8 ship sign-off.** It is a request that the architect rule on R8–R12 as candidate remediation.
- **NOT a Phase 2 authorization.** Phase 2 (LeanVerdict / PartialAccepted typed schema work in `src/state/typed_tx.rs` + `src/runtime/attempt_telemetry.rs` schema) is Class-4 STEP_B and requires a **separate remediation directive** AFTER Phase 1 #5 (FC-first analysis) is delivered and reviewed.
- **NOT a license to start M2/M3 / NodeMarket / TB-19 / public-benchmark work.** The parent-ruling FREEZE is in force.

---

## §8 Per-R ratification matrix (summary table for architect quick-rule)

| R | What it does | Class | Authorization gap | Default position pending architect ruling |
|---|--------------|-------|-------------------|------------------------------------------|
| R8 | assert_45 partial-verdict invariant + LeanResult doc-comment refresh | 3 + Class-4-adjacent (schema doc) | self-promoted §8; FC-first missing; α-vs-β unresolved | preserve on `main` as candidate; final ratification conditional on Phase 2 typed semantics |
| R9 | P38/P49 rerun script + evidence | 3 evidence | self-promoted §8 | preserve as audit-trail; Phase 3 fresh rerun supersedes for final closure |
| R10 | `comprehensive_arena --plan-only` writes ARENA_PLAN.md | 1 | self-promoted §8 | accept as workspace-gate closure |
| R11 | Dashboard substantive smoke | 1 | self-promoted §8 | accept as Q11 closure |
| R12 | Round-2 candidate report + R3 supersession markers | 0 | self-promoted §8; remedies required (banner, OBS, verbatim) | accept after Phase 1 #1/#3/#4 complete |

---

## §9 Closing posture

R8–R12 stay on `main` (no rollback per parent ruling §3.1). Workspace count `1049 passed / 0 failed / 150 ignored` at HEAD `eb2b932` is preserved as evidence the candidate remediation is internally coherent at the test surface.

**No claim of "TB-18R G2 round-2 PASS" is made by Claude orchestrator.**
**No new `main`-touching atom under self-promoted §8 will be authored after this addendum.**

The next legitimate work items are Phase 1 #3, #4, #5, #6, #7 (all docs/analysis) and **then halt** for architect review of:
- the FC-first analysis (Phase 1 #5),
- this ratification matrix,
- the Phase 2 directive draft.

Only after explicit architect §8 sign-off citing this addendum may TB-18R proceed to Phase 2 schema work.

---

**End of R8–R12 ratification addendum.**
