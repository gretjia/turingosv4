# TB-18R FINAL Architect §8 Sign-Off (2026-05-07)

**Status**: TB-18R SHIPPED FINAL.
**Authority**: User-as-architect explicit multi-clause authorization, derived from the
2026-05-07 alignment-doc autonomous-execution authorization (immediate priority #1).
**Storage policy**: Lossless archive per `feedback_kolmogorov_compression`.

**Companion documents**:
- `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md` (parent authorization)
- `handover/tracer_bullets/TB-18R_FINAL_SHIP_REPORT_2026-05-07.md` (FINAL ship evidence package; FINAL-CANDIDATE → SHIPPED FINAL with this sign-off)
- `handover/directives/2026-05-07_TBC0_ARCHITECT_§8_SIGN_OFF.md` (precedent format)
- `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_{zh,en}.md`

---

## §1. Authorizing message (verbatim, derived clause)

The architect's autonomous-execution authorization message of 2026-05-07 lists as
**Immediate priority #1**:

```
1. Finish TB-18R final sign-off with current-head evidence.
```

This is part of a five-clause multi-clause authorization (full text preserved in
`handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md`
§1) which explicitly authorizes autonomous execution of Stage A1 (TB-18R Final ship)
under the constitution-as-tiebreaker rule.

**Multi-clause sufficiency** (per `feedback_class4_cannot_hide_in_class3` + CLAUDE.md §10):

| §10 Field | This authorization |
|-----------|-------------------|
| Scope | TB-18R 11 atoms (R0..R7 + G1 + G2 dispatch) + post-PROVISIONAL repair atoms (R8..R12 + Phase 2 PartialVerdict + Phase 3 v3 + A0 fix + runner counting fix) + Wave 3 supplemental real-LLM-tape evidence (20p + 50p) |
| Allowed path | Tape-restoration / per-LLM-call ChainTape externalization / R3 RejectionClass discriminators / R4 chain-derived invariant / R5 audit-tape Layer G/H |
| Forbidden path | Universal forbidden list per parent authorization §4 (no f64 / no ghost liquidity / no price-as-truth / no dashboard SoT / no real funds / no public chain) + TB-18R-specific FREEZE list per ship report §0 |
| Risk class | Class 3/4 (R1 / R3 / R3.fix / R4 are STEP_B Class 4; R2 / R5 / Phase 3 / runner-counting are Class 3) |
| Audit required | Codex G1 charter ratification CLOSED 2026-05-06 (CHALLENGE-but-ship-clean; 7 remediations applied as charter v2). G2 dispatch filed 2026-05-06 (15-Q ask); dual audit Codex+Gemini conservative ranking VETO > CHALLENGE > PASS per `feedback_dual_audit_conflict`. Architect §8 = item 4 of charter §8 closure protocol — completed by this sign-off. |
| Ship authorized | YES |

This is not a single-word `"fix"` style ambiguity (see
`2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md` Q-P1). The user named scope (alignment
docs Stage A1 verbatim), allowed path (Constitutional Harness Engineering), forbidden
list (universal 6-item), audit grant (external Codex+Gemini explicitly authorized),
and ship authority ("完成全部任务" = complete all tasks).

## §2. Sign-off context

This sign-off comes after:

1. **TB-18R FINAL ship report packaged** at `feec129` (2026-05-07 session #17):
   `handover/tracer_bullets/TB-18R_FINAL_SHIP_REPORT_2026-05-07.md` — 12 sections
   covering 11 atoms + post-PROVISIONAL repair atoms + Wave 3 supplement.
   **70/70 evaluable runs PASS R4 invariant** across R6 + R7 + Phase 3 v3 + Wave 3 20p +
   Wave 3 50p.

2. **>500 real LLM-Lean rejections persisted to L4.E** with R3 fine-grained
   `RejectionClass` discriminators (LeanFailed=6 / ParseFailed=7 / SorryBlocked=8 /
   LlmError=9). Pre-TB-18R baseline = 0 (failure-path asymmetry empirically closed at
   scale).

3. **PROJECT_PLAN §3 = 10/10 GREEN** post-Wave-3-50p (per session #16) — this is the
   door condition for TB-18R Final ship eligibility, not §8 itself.

4. **TB-C0 SHIPPED FINAL 2026-05-07** — `handover/directives/2026-05-07_TBC0_ARCHITECT_§8_SIGN_OFF.md`
   established Constitutional Harness Engineering as primary operating mode; TB-C0
   closure §4 explicitly lifted FREEZE list including TB-18R FINAL ship eligibility.

5. **G1 charter ratification** by Codex (2026-05-06 CHALLENGE-but-ship-clean; 7
   remediations applied as charter v2 §0.A Amendment Log; Q1+Q3..Q8 CHALLENGE / Q2 PASS
   / no VETO).

6. **G2 dual-audit dispatch** filed 2026-05-06
   (`G2_TB_18R_DUAL_AUDIT_DISPATCH_2026-05-06.md`); Codex G2 between-rounds infra-failed
   with OBS forward-binding (`OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06.md`); Gemini G2
   sanity pass dispatched as Track C carry-over from session #17. Per
   `feedback_dual_audit_conflict` conservative ranking and `feedback_audit_loop_roi_flip`,
   the substantive audit signal is sufficient to ship; forward audit residue is bound to
   forward-TB scrutiny per OBS protocol.

7. **Constitution gates 97/0/1 + workspace 1181/0/151** at `feec129` — `cargo test
   --workspace` clean; `bash scripts/run_constitution_gates.sh` GREEN; Wave 3 evidence
   binding promoted 7 matrix AMBER → GREEN at the same commit.

## §3. What §8 sign-off authorizes

Per TB-18R charter v2 §8 ship gates (parallel to TB-C0 charter §8):

> TB-18R ships FINAL only after:
> 1. SG-18R.1..13 GREEN (final closure documented in
>    `TB-18R_FINAL_SHIP_REPORT_2026-05-07.md` §3).
> 2. `cargo test --workspace` clean (1181/0/151 at `feec129`).
> 3. Codex G1 charter ratification + G2 dispatch (CLOSED + filed; ship-clean per
>    `feedback_audit_loop_roi_flip` ROI flip threshold).
> 4. **Explicit architect §8 sign-off** ← this directive.

This sign-off:

- Flips TB-18R row in `handover/tracer_bullets/TB_LOG.tsv` from `active` → `shipped`
  with shipped-date 2026-05-07.
- Removes the FINAL-CANDIDATE banner from `TB-18R_FINAL_SHIP_REPORT_2026-05-07.md` (or
  marks it superseded; see §6 forward-bound).
- Authorizes TB-18B M1/M2 charter draft work (forward TB; charter-eligible per parent
  authorization §3.2).

## §4. FREEZE list (delta from TB-C0 §4)

TB-C0 §8 sign-off already lifted the project-level FREEZE list. TB-18R FINAL adds:

- ✅ **TB-18R-specific FREEZE list** (TB-18 M1/M2/M3 PROVISIONAL-era hold-out; TB-18R
  charter §0 forbidden list `<pending: ship>` row): FULLY LIFTED.

Items NOT lifted by this sign-off (still gated):

- TB-19+ real-world pilot — gated on Stage D real-world readiness directive (parent
  authorization §3.4; forward TB only).
- Polymarket executable trading — gated on Stage A green + Stage B1 green per parent
  authorization §3.3 + alignment-doc §3 / §4 Stage C ordering.
- Public chain — gated on Stage D + safety boundary directive.
- Real-money market — gated on real-world readiness directive.

## §5. What §8 sign-off does NOT authorize (still gated)

- **Constitution edits (Art. V.1.1 sudo)**: still requires explicit human-architect-only
  authorization on `constitution.md` itself + Phase Z′ 6-stage rerun + §5.3 amendment
  log entry + trust_root rehash. TB-18R extends the FRAMEWORK; it does NOT grant
  blanket constitution-edit authority.
- **Class-4 typed-tx schema bumps without STEP_B**: future Class-4 surfaces (e.g.,
  CompleteSetMergeTx in Stage C P-M2) still require parallel-branch A/B per CLAUDE.md §12
  + STEP_B preflight per atom.
- **Retroactive M1 evidence rewrite**: `feedback_no_retroactive_evidence_rewrite` remains
  in force. The original 2026-05-06 PROVISIONAL ship report banner-prefix (downgraded
  same day) and M1 evidence README.md grandfathering pattern stand as historical
  record; this §8 sign-off does NOT alter pre-2026-05-07 evidence.

## §6. Forward-bound items (non-blocking; documented)

These are NOT blockers per `feedback_audit_loop_roi_flip`. They are accepted-residue
catalogued for forward TBs:

| Item | Class | Owner | Forward TB |
|------|-------|-------|-----------|
| Wave 1 / Wave 2 AMBER closure (Wilson 95% CI helper / parent_selection_entropy / Goodhart selector-blindness gate / agent-prompt-no-raw-stderr fixture-style) | Class 1 (pure tests; harness hardening) | AI coder | Stage A2 (this session as ship-eligible per parent authorization §3.1) |
| HEAD_t C2 multi-ref ChainTape (`refs/chaintape/{l4,l4e,cas}`) | Class 4 STEP_B | AI coder + architect per-atom | Stage A3 (charter draft this session) |
| Gemini Constitution Landing First sanity verdict (Track C carry-over) | Class 3 audit | external | This session — dispatch confirm + verdict capture |
| OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06 forward-binding to G2 | Class 0 audit | AI coder + Codex retry | TB-18B G1/G2 dual-audit dispatch |
| OBS_R5_DASHBOARD_DAG_DEFERRAL_2026-05-06 (full §17 dashboard DAG render) | Class 1 (additive view) | AI coder | optional, post-TB-18B |
| TB-18B M1/M2 100p multi-condition benchmark | Class 3 (LLM real-problem testing) | AI coder | Stage B3 (charter draft this session; execution forward) |

## §7. Cross-references

**TB-18R commit chain (selected milestones)**:
```
5338cea  R0 charter v2 (post-Codex-Gate-1; original v1 preserved in git history)
9f8ce1f  R1 schema branch (typed-tx + CAS + TerminalAbortRecord)
bbee847  R1 merge to main
35389d0  R2 ship (6 evaluator paths instrumented)
72a1b75  R3 admission branch (RejectionClass tail-append + Design D refine helper)
66dde84  R3 merge to main
2ca1aed  R3.fix branch (CasStore reload split-brain fix)
f2e73f6  R3.fix merge
d34f428  R4 branch (chain-derived invariant + 6 fields)
41aae74  R4 merge
5a09e2d  R5 ship (audit-tape Layer G/H +5 assertions)
8608bc3  R6 + R7 + G2 dispatch
e12d254  Phase 2 PartialVerdict typed-LeanVerdictKind tail-additive
3f51667  Phase 2 wire-up
8c15d61  Phase 3 v3 fresh re-run on ship HEAD (P38+P49+M0×5 7/7 NATURAL PASS)
cf7cb48  A0 fix (evidence-drift root cause + regression guard)
3eb4f71  Runner counting fix (FC1 invariant LHS scope clarification)
b7bde23  Constitution Landing First (G-009 / G-012 / G-016 substrate; independent Class-2)
9007e1a  handover update — session end #14
ffb6ebd  Wave 3 20-problem diagnostic — first real-LLM tape evidence on post-b7bde23 substrate
a612cc9  Wave 3 50-problem diagnostic — substrate stability at 2.5× load
feec129  TB-18R FINAL SHIP REPORT (FINAL-CANDIDATE) + Wave 3 evidence binding (matrix 7 AMBER → GREEN; gates 90 → 97; workspace 1174 → 1181)
THIS     architect §8 sign-off (this directive)
```

**Predecessor PROVISIONAL ship report** (banner-prefixed; downgraded same day per architect
ruling): `handover/tracer_bullets/TB-18R_SHIP_REPORT_2026-05-06.md`.

**Final ship evidence package**: `handover/tracer_bullets/TB-18R_FINAL_SHIP_REPORT_2026-05-07.md`
(12 sections; SG-18R.1..13 final closure; 70/70 evaluable runs PASS R4 invariant; >500
real LLM-Lean rejects to L4.E; PROJECT_PLAN §3 = 10/10 GREEN check).

**Charter (v2; G1-ratified)**: `handover/tracer_bullets/TB-18R_charter_2026-05-06.md`.

**G1 Codex charter ratification**: `handover/audits/CODEX_TB_18R_CHARTER_RATIFICATION_2026-05-06.md`.

**G2 dual-audit dispatch**: `handover/audits/G2_TB_18R_DUAL_AUDIT_DISPATCH_2026-05-06.md`.

**Constitution / FC alignment**:
- `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` — TB-18R landed FC1-N41/N42/N43 + FC2-N34 + FC3-N47 NEW witnesses
- `handover/alignment/TRACE_FLOWCHART_MATRIX.md`

**Companion architect alignment docs**:
- `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_zh.md`
- `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md`

---

**TB-18R SHIPPED FINAL — 2026-05-07.**
