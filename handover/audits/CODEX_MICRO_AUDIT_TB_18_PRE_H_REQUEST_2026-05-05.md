# TB-18 Atom G0 — Codex micro-audit request (pre-H-ladder substrate review)

**Status**: REQUEST FILED — awaiting Codex external audit run by user. AI-coder cannot autonomously launch /ultrareview or external Codex/Gemini audit; user invokes per their cloud-audit billing.
**Filed**: 2026-05-05.
**Authority**: TB-18 charter §1.4 SG-18.15 + architect TB-18 ratification ruling §2.1 + Q7 (G0 = pre-H micro-audit; G1 = post-H final ship audit).

---

## §1 Why this audit exists (architect §2.1 verbatim)

Architect §2.1 (G-before-H bug architect-flagged AI-coder blind spot):

> 如果 G 是 dual external audit, 而 H 是 M-ladder benchmark report, 那么 G before H 不能审计 H. 所以要么 G0 = pre-H micro-audit + H = M-ladder + G1 = final dual audit, 要么直接把 G 移到 H 后.

Q7 verdict: G0 Codex micro-audit AFTER F BEFORE H. Saves M-ladder compute on broken substrate.

## §2 Scope — what to audit

**SUBSTRATE atoms shipped at HEAD (TB-18 sequence atoms 0 → E → A → H0 → D-design → C → B-design):**

| Commit | Atom | What |
|---|---|---|
| `d3c8d78` | Atom 0 | TB-18 charter ratified-with-amendments + ruling archive |
| `8ad7a1d` | Atom E | OBS_R023 closure: hardcoded `RunOutcome::MaxTxExhausted` literal removed; caller-propagated via `terminal_exhaustion_reason` variable + `.to_run_outcome()` projection |
| `13a5ee0` | Atom A | drive_task API surface stub + per-LLM-call budget primitives + `RunOutcome::DegradedLLM` variant + DegradedLLM-emits-EvidenceCapsule wiring in run_swarm |
| `5c40d06` | Atom H0 | M0 small preflight (3 problems on real DeepSeek) — substrate validation evidence |
| `c025cdb` | Atom D-design | Class 4 escalation refusal + Path C multi-task verdict |
| `ae9530f` | Atom C | Deferred-finalize idempotency: 4/5 ship gates structurally enforced; Gate 3 PARTIAL → TB-19+ STEP_B forward trigger |
| `7bb18b4` | Atom B-design | Substantive comprehensive_arena spec + TB-18.B-impl forward trigger |

**EXCLUDED from G0 scope** (deferred to G1 or future TBs):
- Atom B-impl (deferred to TB-18.B-impl follow-up; not in G0 surface).
- Atom F single-chain 13/13 evidence (depends on B-impl; not produced).
- Atom H M-ladder evidence (G0 by definition is BEFORE H; H result is in G1 scope).

## §3 Audit questions (Codex prompt template)

```text
TB-18 Pre-H Substrate Micro-Audit (architect Q7 verdict)
Target: 7 commits d3c8d78..7bb18b4 on branch main (turingosv4 repo).
Scope: Atom A + Atom C + Atom E + Atom B-design + Atom D-design substrate.
Exclude: Atom B-impl (deferred); Atom F (depends on B); Atom H M-ladder
(post-G0).

Architect ruling 2026-05-05 §2.1 + §3 + §4 (FR-18.1..10 + CR-18.1..10
+ SG-18.1..16) is the binding spec. Read:
  handover/directives/2026-05-05_TB18_CHARTER_RATIFICATION_ARCHITECT_RULING.md
  handover/tracer_bullets/TB-18_charter_2026-05-05.md

Then verify each of the following:

Q1. Atom E correctness (`8ad7a1d`):
    - Verify `RunOutcome::MaxTxExhausted` literal scan in evaluator.rs
      lines 2940-3137 region returns ZERO matches.
    - Verify `ExhaustionReason::MaxTxExhausted` literal appears EXACTLY
      ONCE = function-header default initialization of
      `terminal_exhaustion_reason`.
    - Verify `to_run_outcome()` projection contract: 5 (now 6 after Atom A)
      ExhaustionReason variants project correctly to RunOutcome.
    - Verify: when atom A's HaltDegradedLLM fires, atom E's propagation
      pipeline emits EvidenceCapsule + TerminalSummary with
      outcome=DegradedLLM (NOT silently skipped per architect §2.5).

Q2. Atom A correctness (`13a5ee0`):
    - PerCallBudget defaults: 60s/call + 30 token floor + 10 consecutive
      cap + 600s aggregate. Match architect §B.9 M0 spec?
    - LLMCallBudgetTracker: counter resets on substantive responses (FR-18.2);
      verify with synthetic intermittent-trivial test pattern.
    - DegradedLLM evidence emission: confirm warn! log + terminal_exhaustion_
      reason mutation + break flow → atom E cleanup → EvidenceCapsule
      (NOT evidence-skip backdoor; architect §2.5).
    - drive_task stub: PendingAtomB error variant explicit; NOT silent
      delegation to run_swarm (per feedback_no_fake_menus).
    - run_oneshot path NOT wired (acceptable: single-call has no consecutive
      concept; rationale documented in evaluator.rs comment near line ~3636
      change OR explicit if needed).
    - RunOutcome variant addition is canonical: u8 discriminant 5 added; check
      typed_tx.rs that no chain-canonical-payload field shape change occurred.
    - genesis_payload.toml rehash (8128b3ed for evaluator.rs; 213251db for
      typed_tx.rs): predecessor lineage preserved; rationale captured in
      rehash comment.

Q3. Atom C correctness (`ae9530f`):
    - Test file accurately documents 4/5 STRUCTURALLY enforced gates +
      Gate 3 PARTIAL coverage.
    - Gate 3 documented test (`tb_18_c_gate3_partial_documented_via_assertion_
      on_existing_code`) panics if Open-blocking is detected (forward
      regression detection); verify the assertion logic.
    - Forward trigger to TB-19+ STEP_B_PROTOCOL Class 3 explicit and
      consistent with feedback_step_b_protocol.

Q4. Atom B-design correctness (`7bb18b4`):
    - SharedChain refactor spec (§4.1-§4.3) is technically sound.
    - 6-task engineered manifest (§4.5) covers all 13 architect tx kinds in
      single-chain-multi-task lifecycle.
    - Deferral rationale (§3) honors feedback_iteration_cap_24h Class 3 72h
      cap + feedback_step_b_protocol parallel-branch discipline.
    - TB-18.B-impl forward trigger explicit and time-boxed.

Q5. Atom D-design correctness (`c025cdb`):
    - Class 4 escalation analysis (§4.1 + §4.2): both Path A + Path B
      correctly identified as Class 4 per architect Q2 hard rule.
    - Path C (multi-task structure dissolves single-market constraint)
      analysis: technically correct that PRE-17.6 §2.2 mutual-exclusion is
      single-MARKET, not single-CHAIN.
    - architect §2.7 lifecycle-append-only invariant correctly captured as
      TB-19+ Class 4 forward trigger (carry-forward not papered over).

Q6. Wire integrity:
    - `cargo test --workspace --release` passes (target 962/0/150 at HEAD
      `7bb18b4`+; baseline 939 + 23 new TB-18 tests).
    - Trust root rehash entries are R-014 protocol compliant (predecessor
      lineage preserved; rationale captured in comment).

Q7. Anything missed:
    - Did the substrate ship surface forget anything required by
      architect §2.1-§2.10 + §4 + Q1-Q7?

Output verdict format:
  OVERALL: VETO | CHALLENGE | PASS
  Per-question (Q1-Q7): VETO | CHALLENGE | PASS + rationale
  Recommended pre-H remediations (if VETO/CHALLENGE).
```

## §4 What G0 verdict gates

Per TB-18 charter SG-18.15: "Codex micro-audit after F passes or issues are remediated before H."

- **VETO** → atom H BLOCKED until VETO resolved (re-audit).
- **CHALLENGE** → atom H proceeds with documented CHALLENGE response (commit message captures CHALLENGE-resolved status).
- **PASS** → atom H unblocked.

## §5 Why I (AI-coder) cannot run this autonomously

Per CLAUDE.md guidance: external audits (Codex / Gemini / dual-audit) require user-triggered cloud audit runs. /ultrareview is user-billed. AI-coder writes the audit scope doc; user invokes per their cloud-audit budget.

**To execute G0**: user runs `/ultrareview <branch>` OR invokes Codex against the 7-commit range with the Q1-Q7 prompt template above. Audit verdict file lands at `handover/audits/CODEX_MICRO_AUDIT_TB_18_PRE_H_VERDICT_2026-05-XX.md`.

## §6 Cross-references

- TB-18 charter §1.4 SG-18.15
- Architect TB-18 ratification ruling §2.1 + §3 + Q7
- TB-18 substrate commits: d3c8d78 / 8ad7a1d / 13a5ee0 / 5c40d06 / c025cdb / ae9530f / 7bb18b4
- Memory: `feedback_dual_audit` + `feedback_audit_after_evidence` (Q7 verdict source) + `feedback_audit_loop_roi_flip` (G0 is the ONE pre-H checkpoint per architect Q7 batch-audit compromise)

---

**Awaiting external Codex audit invocation.**
