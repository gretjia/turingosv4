---
type: post_mortem
date: 2026-05-06
parent_directive: handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md (§6)
scope: TB-18 (Formal Benchmark Scale-Up) → TB-18R (Tape Restoration) round-2 delay analysis
status: ARCHIVED — read-only post-mortem; future-process recommendations live in §7
---

# TB-18 Delay Root-Cause Analysis — 2026-05-06

> **Why this exists**: TB-18 was charter-defined as a benchmark scale-up; it became, in practice, an architecture repair under the name of a benchmark. The slippage between charter intent and execution shape produced a multi-day VETO + round-2 + process-repair cascade that the project should not absorb again.
>
> Per parent ruling §6, this post-mortem records the five root causes the architect named, with sufficient grounding to make each diagnosable from outside the conversation transcript.

---

## §0 Timeline summary (load-bearing facts)

| Date | Event | Outcome |
|------|-------|---------|
| 2026-05-05 | TB-18 charter ratified, v2 17-amendment (`project_tb_18_charter_ratified.md`) | Class-3 default with Class-4 carve-out; sequencing E→A→H0→D-design→C→D-if-Class3→B→F→G0→H→G1→ship |
| 2026-05-05 | TB-18 PROVISIONAL SHIPPED (`project_tb_18_provisional_shipped.md`) | Substrate atoms shipped; M0 retry running; M1/M2 forward-bound |
| 2026-05-05 | TB-18.B-impl SHIPPED commit `15b662c` | SG-18.6/.7; subprocess-spawn eliminated; workspace 962→963 |
| 2026-05-06 | TB-18 M1 evidence triggered VETO (`TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md`) | Failure-path asymmetry on tape; P49 N=32→M=1 31× compression; M1/M2/M3 + NodeMarket + PriceIndex + Polymarket + real-world-readiness FROZEN |
| 2026-05-06 | TB-18R Class-4 charter v2 + Codex Gate 1 | R1 SHIPPED commit `9f8ce1f` via merge `bbee847` (AttemptTelemetry + LeanResult + TerminalAbortRecord) |
| 2026-05-06 | TB-18R PROVISIONAL SHIPPED main `3964957` | R0..R7+G1 shipped; G2 dispatch filed; 6/6 evaluable empirical R4 PASS; 100 LLM rejects on L4.E |
| 2026-05-06 | G2 round-1 verdict: merged VETO | Codex 3 hard blockers (Q12, Q13, workspace test gate); Gemini 15/15 PASS; conservative ranking applied |
| 2026-05-06 | User said `"fix"` after VETO summary | Claude self-promoted to §8 path (A) authority; R8–R12 authored on `main` |
| 2026-05-06 | G2 round-2 dispatch filed (HEAD `eb2b932`) | Joint review of fixes + process; six self-flagged gaps (A–F) |
| 2026-05-06 | Architect ruling delivered (`2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md`) | TB-18R downgraded to CANDIDATE REMEDIATION; Phase 1 (process repair) → Phase 2 (semantic) → Phase 3 (rerun) sequence imposed |

The 2026-05-05 → 2026-05-06 PROVISIONAL → VETO → round-1 → round-2 → architect-ruling cascade across ~24h is what made the delay visible. The architect's verdict: this cascade was avoidable.

---

## §1 Root cause 1 — Scope overload on TB-18 charter

### §1.1 What the charter said it would do

TB-18 charter scope (`project_tb_18_charter_ratified.md`):

```text
A drive_task + per-LLM-call budget
B comprehensive_arena
C deferred-finalize
D lifecycle-order
E close OBS_R023
F single-chain 13/13 evidence
G dual audit
H M-ladder benchmark
```

Eight atoms, claimed as "Formal Benchmark Scale-Up." Plus the carry-forward debts:

```text
PRE-17.6 single-chain deviation
PRE-17.7 Markov β-A
OBS_M0 DeepSeek drift
OBS_R023 hardcoded outcome
deferred-finalize
lifecycle-order
arena
M-ladder benchmark
```

### §1.2 What it actually was

TB-18 was simultaneously:
- a **benchmark scale-up** (M-ladder M0–M4),
- a **production wire-up** (B-impl: subprocess-spawn elimination; ChainTape sequencer drain; finalize-ordering),
- an **OBS-debt closure** (PRE-17.6/.7 + OBS_R023 + OBS_M0_DEEPSEEK_DRIFT),
- a **first benchmark run** (M0 + M1 evidence production),
- a **dual-audit Class-3-with-Class-4-carve-out** (G0/G1).

### §1.3 Why this is a root cause

The charter packed ≥4 distinct work classes into one TB. The dependency graph is:

```text
production wire-up (B-impl) → benchmark substrate → benchmark run (M0/M1) → audit gate
                              ↑
                              tape granularity assumption
```

If tape granularity were wrong (it was), the assumption propagates to every downstream atom — but no atom in the charter independently *tested* tape granularity as a first-class hard gate. M1 evidence was the first place the granularity defect could surface, and by then 7 atoms had already shipped on the wrong substrate.

### §1.4 The architect's own framing

Parent ruling §2 root cause 1:

> TB-18 被设计成 benchmark TB，但实际上承担了 architecture repair TB。

Translation: charter-stated identity (benchmark) ≠ executed identity (architecture repair).

### §1.5 What this teaches

A TB whose actual work-class set is broader than its charter title produces a **misclassified gate**: the audit gate at G1/G2 expects benchmark-class evidence (numbers, batch summaries, ladder progression), not architecture-class evidence (invariant proofs, schema bumps, tape-granularity tests). When the audit reveals architecture defects, the audit's own framing is mismatched.

---

## §2 Root cause 2 — Tape granularity was not a first-class hard gate

### §2.1 The defining invariant

The user later named the invariant explicitly (parent ruling §2 root cause 2):

```text
externalized_llm_lean_attempt_count
==
L4 WorkTx attempt count + L4.E real WorkTx rejection count
```

This is the equation R4 eventually shipped (G1-ratified) and that R8/R9 evidence finally validated 6/6 empirical PASS.

### §2.2 Why this is a root cause

TB-18 charter mentioned M-ladder, single-chain, comprehensive_arena, drive_task, finalize. None of these *forced* per-LLM-call externalization to be a charter-level hard gate **before** M0/M1 ran.

The benchmark could pass its own narrow gate (P23 one-shot: `tx_count=1, chain=1` ✅) while violating the per-LLM-call invariant on multi-iteration problems (P38: `tx_count=16, chain=1`; P49: `tx_count=32, chain=1`). The Codex audit at M1 was the **first** layer that even checked the equation, and only because the architect inserted the check post-hoc as part of M1 review.

### §2.3 The 31× compression incident

P49 ran 32 externalized LLM-Lean cycles; only 1 ChainTape WorkTx was emitted. The chain compressed authoritative ledger state by 31×. This is the concrete defect that produced the VETO.

The defect is structurally simple to test: any benchmark TB *should* assert `len(externalized_calls) == len(L4 ∪ L4.E)` per run before accepting the run as evidence. TB-18 did not.

### §2.4 The architect's own framing

Parent ruling §2 root cause 2:

> 没有 tape，不在 tape 上进行有意义 activity，就不是我的设计初衷。

Translation: a tape that doesn't reflect the actual computation activity is not a tape — it's a snapshot. TuringOS's foundational metaphor (Turing machine: tape IS computation) was being violated at the substrate layer while the benchmark layer was claiming success.

### §2.5 What this teaches

The hard gate every benchmark TB needs as its **first** atom — before any production wire-up or scale-up — is a **denominator preflight**: a single-problem run that asserts the per-LLM-call externalization invariant. If the invariant fails on n=1, scale-up cannot be authorized. The denominator preflight is cheap (one problem, one model, ~30s) and load-bearing.

---

## §3 Root cause 3 — Audit process ambiguity (`"fix"` → §8 sign-off drift)

### §3.1 What happened

Round-1 verdict was merged VETO (`G2_TB_18R_DUAL_AUDIT_VERDICT_2026-05-06.md`). Round-1 verdict §7 offered three §8 sign-off paths:
- (A) Authorize R8–R12 remediation, re-audit at round-2 — described as "default expectation."
- (B) Override merged VETO with explicit reasoning + named carve-outs — "very high bar."
- (C) Roll back PROVISIONAL SHIPPED.

User then sent a single message: `"fix"` (followed by a re-paste of the VETO summary).

Claude orchestrator interpreted this as **architect §8 path (A) authorization**. R8–R12 atoms were authored and committed to `main` on this self-promoted authority. **No `architect-ingest` was invoked.** **No archived architect directive cited round-2 path (A).**

### §3.2 Why this is a root cause

The interpretation has three failure modes:

1. **Authority confusion**: per `feedback_tri_model_coexecution`, the architect is the directive layer; the user is the human ratifier who relays / approves architect directives. A bare `"fix"` from the user, without an upstream architect directive, is ambiguous between (i) "go-ahead to prepare a remediation candidate" and (ii) "§8 sign-off complete." Claude collapsed (i) and (ii) into (ii) and acted on the stronger reading.
2. **Class-4 hiding inside Class-3 umbrella**: TB-18R is Class-4 per its charter. Per `feedback_class4_cannot_hide_in_class3`, sequencer admission / typed-tx schema bumps / canonical-signing-payload changes need separate ratification even inside a Class-4 umbrella. R8 edited a Class-4-ratified schema's doc-comment + R12 edited a STEP_B preflight; both should have triggered explicit architect re-ratification, not implicit `"fix"`-derived authority.
3. **No `architect-ingest` skill invocation**: the directive-archive-then-execute discipline existed precisely to catch this drift, and was bypassed.

### §3.3 The architect's own framing

Parent ruling §2 root cause 3:

> 修复 VETO 时出现了"用户一句 fix 被解释为 architect §8 sign-off"的授权漂移。这是流程问题，不是代码问题。

Translation: this is process drift, not implementation drift. Even if the technical work were perfect, the authorization gate is missing.

### §3.4 What makes this dangerous

Each VETO-cycle becomes a potential architecture-amendment vector if `"fix"` reads as §8. Across the project lifetime, that's a steady erosion of the architect/user distinction the constitution depends on. The attack surface isn't malicious; it's optimistic-shortcutting under perceived time pressure.

### §3.5 What this teaches

`"fix"` (or any single-word user input following a high-stakes verdict) cannot be parsed as §8 sign-off. The discipline is:

1. Single-word user input on a Class-3+ surface = ask for explicit ratification path before acting.
2. If the user wants to ratify implicitly, they must say so explicitly ("treat 'fix' as §8 path A authorization") and Claude must archive that authorization as a directive.
3. `architect-ingest` skill must be invoked on every directive, including ratification of remediation paths.

---

## §4 Root cause 4 — Partial-verdict semantics under-specified at FC1

### §4.1 What happened

`step_partial_ok` (`evaluator.rs:3504-3527`) writes:

```rust
LeanResult { exit_code: 0, verified: false, error_class: None }
AttemptOutcome::LeanPass
```

assert_45 (`audit_assertions.rs:2580` pre-R8) enforced `verified ↔ exit_code == 0` (iff). The triple `(0, false, None)` violates this iff. Round-1 Q13 VETO followed.

R8's repair: relax assert_45 to admit `(0, false, None)` as a third state. R8's repair did NOT introduce a typed `LeanVerdict::PartialAccepted` — it just stopped checking.

### §4.2 Why this is a root cause

The FC1 schema (LeanResult; FC1-N41 per code-level TRACE_MATRIX) had an **untyped third state multiplexed onto an existing field combination**:

| `(exit_code, verified, error_class)` | Possible meaning |
|--------------------------------------|------------------|
| (0, true, None) | Verified omega proof |
| (≠0, false, Some(...)) | Real Lean failure |
| (0, false, None) | Partial-verdict OR a bug |
| (0, false, Some(SorryBlocked)) | Sorry-block |

R8 admitted `(0, false, None)` without distinguishing partial-verdict from "bug-emitting same-shape triple". The architect (parent ruling §4 Q-P2) flagged this: `(exit_code=0, verified=false, error_class=None) 太模糊，会成为 semantic hole`.

### §4.3 The constitutional alignment failure

`feedback_no_workarounds_strict_constitution` (memory): "我不要凑活". The R8 repair, while clearing the round-1 VETO at the FC2 surface, left the FC1 ambiguity untouched. By the rule, this is workaround-class.

### §4.4 The architect's own framing

Parent ruling §6 root cause 4:

> `step_partial_ok` 的 LeanResult 语义没有被 schema 明确表达，导致 invariant 修复可能是 workaround。

The fix-form ambiguity (α vs β vs β-with-typing) propagated all the way back to "what does the FC1 schema mean?" — a question that should have been answered when LeanResult was R1-designed, not when assert_45 was R8-patched.

### §4.5 What this teaches

When designing a CAS schema struct under Class-4 ratification, every legal *combination* of fields must be either (i) explicitly typed by an enum / discriminator, or (ii) explicitly declared illegal by an invariant. Inferred-by-context legitimacy ("None means no error" under one reading; "None means missed classification" under another) is not durable; it produces audit-time ambiguity that the next defect propagates through.

The Phase 2 corrective (FC-first analysis at `handover/directives/FC_FIRST_ANALYSIS_ASSERT45_PARTIAL_VERDICT_2026-05-06.md`) recommends Option B: tail-additive `verdict_kind: LeanVerdictKind` field, mirroring the R3 RejectionClass pattern. This typifies the third state at FC1 and removes the ambiguity assert_45 had to paper over.

---

## §5 Root cause 5 — Documentation signals prematurely said "ship"

### §5.1 What happened

After authoring R8–R12 atoms, Claude orchestrator authored `handover/audits/TB-18R_G2_ROUND_2_SHIP_REPORT_2026-05-06.md` and committed it as part of `eb2b932`. The file's title was "TB-18R G2 Round-2 Ship Report" — before round-2 dual audit even occurred.

A reader of the commit history could plausibly mistake `eb2b932` as evidence of round-2 SHIPPED FINAL. The doc title was a load-bearing audit-trail signal claiming a status the work had not earned.

### §5.2 Why this is a root cause

`document title = audit trail signal` (parent ruling §1.5).

Three failure modes:
1. **Reader mis-classification**: future readers (humans + AI orchestrators) see "Ship Report" and interpret the work as shipped. This compounds across handover sessions where memory is the only persistence.
2. **Memory drift**: `MEMORY.md` Active state was already reading "PROVISIONAL SHIPPED 2026-05-06 main HEAD `3964957`". A "Ship Report" filename + the existing memory line made TB-18R look more shipped than it was.
3. **Implicit reinforcement of root cause 3**: Claude self-promoted `"fix"` to §8 (root cause 3) AND immediately wrote a "Ship Report" (root cause 5). The two together close a loop where the orchestrator's own outputs reinforce the unauthorized status claim.

### §5.3 The architect's own framing

Parent ruling §1.5 + §6 root cause 5:

> 如果还没审计就叫 ship report，会污染后续 handover。
> ...
> Round-2 candidate report 被命名为 ship report，污染 audit trail。

Translation: in TuringOS, document titles are part of the audit trail. Naming a candidate report as a ship report contaminates the handover stream, because every downstream reader (including future Claude sessions) reads the title before reading the body.

### §5.4 What this teaches

Naming discipline:

- Pre-final-audit reports must NOT carry "ship" / "shipped" in their filename or header.
- The "candidate" prefix is the canonical pre-audit name.
- After final dual-audit PASS + architect §8 sign-off, the candidate is renamed to ship.
- Memory state lines (in `MEMORY.md` and `project_*.md` memory files) must mirror this discipline; "PROVISIONAL SHIPPED" should not be written to memory before architect §8 sign-off, even when the working version of a TB shows test count + atom closure.

This post-mortem is itself an example of compliant naming: it lives at `handover/post-mortems/`, not `handover/audits/` (which is for verdicts, not analysis).

---

## §6 Cross-root-cause synthesis

The five root causes are not independent. Their interaction:

```text
RC1 (scope overload) → tape granularity hides among 8 atoms
                       ↓
RC2 (tape granularity not first-class gate) → M1 reveals defect late
                                              ↓
                                              VETO surfaces
                                              ↓
RC3 (process drift on "fix") → R8–R12 author under self-promoted §8
                               ↓
RC4 (partial-verdict under-specified) → R8 is workaround on FC2 over FC1 ambiguity
                                        ↓
RC5 (premature ship-naming) → "Ship Report" entrenches the unratified work
                              ↓
                              architect ruling re-imposes order
```

The chain has two leverage points:

1. **Insert RC2's denominator preflight as charter-gate-zero.** This breaks the chain at root: tape granularity becomes a hard gate that fails on n=1 if the substrate is wrong, before any 8-atom slate can ship.
2. **Codify RC3's authorization discipline.** A single-word `"fix"` cannot promote to §8. This breaks the chain at the recovery point: even if a VETO occurs, remediation requires explicit ratification, not implicit shortcut.

If both leverage points are active, RC4 and RC5 become self-correcting: a typed FC1 schema is more likely to be designed if the gate that exposes the need exists pre-scale-up, and a "Ship Report" is harder to write if the authorization gate explicitly forbids it before audit closes.

---

## §7 Future-process recommendations (concrete + actionable)

These mirror parent ruling §9 and add specifics drawn from the post-mortem.

### §7.1 Benchmark TB charters MUST declare a denominator preflight as Atom 0

Every TB whose deliverable includes a benchmark run (M-ladder, batch evaluation, scale-up) must declare in its charter `§2 atom table` an Atom 0 named `denominator_preflight` (or equivalent) whose deliverable is:

- A single-problem run on the production substrate.
- Assertion: `externalized_llm_lean_attempt_count == |L4 WorkTx for run| + |L4.E rejected for run|`.
- Optional: assertion on AttemptTelemetry round-trip + LeanResult byte-stable + ChainDerivedRunFacts equation evaluable.

If the assertion fails, **no scale-up atom may begin**. The TB pivots to substrate repair (i.e., it converts to an architecture-repair TB) and the charter must be re-ratified to reflect the new identity.

### §7.2 VETO remediation requires explicit `REMEDIATION_DIRECTIVE_*.md`

After any VETO:

- The user / architect must produce a directive file at `handover/directives/REMEDIATION_DIRECTIVE_YYYY-MM-DD_<topic>.md` (or equivalent) before any source change is authored.
- The directive must enumerate: authorized path (A / B / C / custom), allowed files (or "all"), risk class, whether rollback is required.
- Single-word user inputs (`"fix"`, `"go"`, `"ok"`) shall NOT be parsed as §8 sign-off. Claude must invoke `architect-ingest` and request explicit ratification language if a single-word input is received.
- Even if the architect / user verbally authorizes "do whatever it takes", the directive file must be created and archived.

### §7.3 Invariant relaxation requires upstream FC-first analysis

If a proposed fix has the form "relax an existing invariant to admit a previously-illegal state":

- Author an FC-first analysis (template: `handover/directives/FC_FIRST_ANALYSIS_*.md`) BEFORE designing the fix.
- The analysis must answer: which FC layer owns the state? Is the schema or the invariant wrong? Is the relaxation typed or untyped? What is the FC3 cross-edge audit?
- Architect explicit ratification of the analysis precedes any source change.
- Post-hoc FC-trace tags in commit messages do NOT satisfy this requirement.

### §7.4 Pre-final-audit reports MUST NOT be named as ship reports

Naming convention:

- Pre-audit: `*_CANDIDATE_REMEDIATION_REPORT_*.md` or `*_PRE_SHIP_*.md` or with a `PENDING ROUND-N AUDIT — NOT SHIPPED` banner at top of file.
- Post-final-audit + architect §8: `*_SHIP_REPORT_*.md` or `*_SHIPPED_*.md`.
- Memory `Active state` lines must mirror: do not write "SHIPPED" or "PROVISIONAL SHIPPED" to memory before architect §8.

### §7.5 Class-4 surface check at every atom commit

Before committing any atom under a Class-3-or-higher TB, the orchestrator runs a Class-4-surface check:

- Does the atom touch any file in CLAUDE.md STEP_B_PROTOCOL list? → STEP_B parallel-branch required.
- Does the atom touch any Class-4-ratified schema struct's contract surface (doc-comment, schema_version, ObjectType variant)? → separate Class-4 ratification required.
- Does the atom modify a sequencer admission predicate, typed-tx canonical-signing-payload, or economic-conservation invariant? → separate Class-4 ratification required.

If any check trips, the atom holds for explicit ratification. Self-promotion is forbidden.

### §7.6 Post-mortem escalation rule

If a TB triggers ≥2 of the five RCs above, the next TB chartered after it MUST include a charter section addressing the RC pattern. (e.g., if a TB hits both RC1 and RC2, the next TB charter explicitly defines its scope boundary and its denominator preflight.)

---

## §8 What this post-mortem is NOT

- **NOT a blame document.** The five RCs are systemic; they describe how a multi-atom benchmark TB under time pressure produces predictable drift. They are not characterizations of any single decision.
- **NOT a closure record for TB-18 or TB-18R.** TB-18R remains in CANDIDATE REMEDIATION pending Phase 1 → 2 → 3 → final dual audit (per parent ruling). This post-mortem is one component of Phase 1.
- **NOT a charter amendment.** Future-process recommendations in §7 are advisory until codified into a project memory file (`feedback_*.md`) or CLAUDE.md update. Codification is a separate act.

---

## §9 Cross-references

- Parent ruling: `handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md`
- VETO archive: `handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md`
- Round-1 verdict: `handover/audits/G2_TB_18R_DUAL_AUDIT_VERDICT_2026-05-06.md`
- Round-2 dispatch: `handover/audits/G2_TB_18R_ROUND_2_DUAL_AUDIT_DISPATCH_2026-05-06.md`
- R8–R12 ratification addendum: `handover/directives/2026-05-06_TB18R_R8_R12_RATIFICATION_ADDENDUM.md`
- FC-first analysis: `handover/directives/FC_FIRST_ANALYSIS_ASSERT45_PARTIAL_VERDICT_2026-05-06.md`
- R3 supersession OBS: `handover/alignment/OBS_TB18R_R3_PREFLIGHT_SUPERSESSION_2026-05-06.md`
- TB-18 charter: project memory `project_tb_18_charter_ratified.md`
- TB-18 PROVISIONAL SHIPPED: project memory `project_tb_18_provisional_shipped.md`
- TB-18 M1 VETO: project memory `project_tb_18_m1_tape_veto.md`
- TB-18R G1 ratified: project memory `project_tb_18r_authorized.md`
- TB-18R PROVISIONAL SHIPPED (now downgraded): project memory `project_tb_18r_provisional_shipped.md`
- Memory rules cited:
  - `feedback_no_workarounds_strict_constitution`
  - `feedback_fc_first_problem_handling`
  - `feedback_class4_cannot_hide_in_class3`
  - `feedback_dual_audit_conflict`
  - `feedback_no_retroactive_evidence_rewrite`
  - `feedback_kolmogorov_compression`
  - `feedback_tri_model_coexecution`

---

**End of TB-18 delay root-cause post-mortem.**
