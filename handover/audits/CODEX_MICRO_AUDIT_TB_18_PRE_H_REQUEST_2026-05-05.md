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

**SUBSTRATE atoms shipped at HEAD `0c3a5e1` (TB-18 sequence atoms 0 → E → A → H0 → D-design → C → B-design → B-impl → F):**

| Commit | Atom | What |
|---|---|---|
| `d3c8d78` | Atom 0 | TB-18 charter ratified-with-amendments + ruling archive |
| `8ad7a1d` | Atom E | OBS_R023 closure: hardcoded `RunOutcome::MaxTxExhausted` literal removed; caller-propagated via `terminal_exhaustion_reason` variable + `.to_run_outcome()` projection |
| `13a5ee0` | Atom A | drive_task API surface stub + per-LLM-call budget primitives + `RunOutcome::DegradedLLM` variant + DegradedLLM-emits-EvidenceCapsule wiring in run_swarm |
| `5c40d06` | Atom H0 | M0 small preflight (3 problems on real DeepSeek) — substrate validation evidence |
| `c025cdb` | Atom D-design | Class 4 escalation refusal + Path C multi-task verdict |
| `ae9530f` | Atom C | Deferred-finalize idempotency: 4/5 ship gates structurally enforced; Gate 3 PARTIAL → TB-19+ STEP_B forward trigger |
| `7bb18b4` | Atom B-design | Substantive comprehensive_arena spec + TB-18.B-impl forward trigger |
| `15b662c` | Atom B-impl | **NEW IN G0 SCOPE** — single-process / single-runtime_repo / single-CAS / single-chain / 6 engineered tasks emitting 13/13 architect-mandated tx kinds in 2.8s wall-clock. SharedChain refactor (chain_runtime.rs); drive_task substantive body (TaskOpen + EscrowLock real-signed scaffold); comprehensive_arena rewrite (subprocess-spawn pattern eliminated per architect §2.8). Class 3 envelope intact (NO sequencer/typed_tx/canonical-payload changes). |
| `0c3a5e1` | Atom F | **NEW IN G0 SCOPE** — single-chain 13/13 audit_tape verdict GREEN (PROCEED + replay byte-identical + tamper 3/3 + β-A feasibility FEASIBLE). Evidence at `handover/evidence/tb_18_single_chain_13_of_13/`. NO production code changes; pure audit run + evidence + reproducibility script. |

**EXCLUDED from G0 scope** (deferred to G1):
- Atom H M-ladder evidence (G0 by definition is BEFORE H; H result is in G1 scope per architect Q7 batch-audit compromise).

## §3 Audit questions (Codex prompt template)

```text
TB-18 Pre-H Substrate Micro-Audit (architect Q7 verdict)
Target: 9 commits d3c8d78..0c3a5e1 on branch main (turingosv4 repo).
Scope: Atom 0 + Atom E + Atom A + Atom H0 + Atom D-design + Atom C +
       Atom B-design + Atom B-impl + Atom F substrate.
Exclude: Atom H M-ladder (post-G0; in G1 scope).

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

Q6. Atom B-impl correctness (`15b662c`):
    - SharedChain refactor (chain_runtime.rs ~430 lines): `from_env` lifted
      from evaluator.rs lines 659-789 + 794-833 (175 lines inline → 25-line
      destructure). Verify functional equivalence: chain construction,
      genesis bootstrap, agent_keypairs registration, chaintape_bundle wiring.
    - drive_task substantive body (drive_task.rs lines 175+): TaskOpenTx +
      EscrowLockTx real-signed scaffold; `PendingAtomB` variant REMOVED;
      new `ChaintapeRequired/AgentKeypairsRequired/SigningFailed/SubmitFailed`
      variants present; `DriveTaskResult` carries task_id + tx_ids +
      post_open_lock_state_root_hex.
    - comprehensive_arena rewrite: subprocess-spawn pattern eliminated per
      architect §2.8; 6 task-driver fns (`drive_task_a..f`) against ONE
      shared bundle; r1 evidence at
      `handover/evidence/tb_18_b_phase4_2026-05-05/r1/` shows
      chain_count=1, runtime_repo_count=1, cas_count=1, sequencer_count=1,
      task_count=6, distinct_tx_kinds=13/13.
    - Class envelope intact: NO src/state/sequencer.rs changes; NO
      src/state/typed_tx.rs schema changes; NO canonical-signing-payload
      digest changes; NO src/{kernel,bus}.rs / src/sdk/tools/wallet.rs
      changes (no STEP_B trigger); NO new TypedTx variants.
    - genesis_payload.toml rehash if any: predecessor lineage preserved;
      rationale captured in rehash comment.

Q7. Atom F correctness (`0c3a5e1`):
    - Five ship-gate asserts ALL GREEN per
      `handover/evidence/tb_18_single_chain_13_of_13/README.md`:
      (1) audit_tape verdict = PROCEED (35/0/0/8)
      (2) verdict.json byte-identical with verdict_replay.json
      (3) audit_tape_tamper detected_count = 3, all_detected = true
      (4) distinct_tx_kinds = 13/13 in single chain
      (5) β-A feasibility = FEASIBLE (no α sidecar; in-tape capsules)
    - β-A in-tape resolution check: LATEST_MARKOV_CAPSULE.txt absent at
      project root + runtime_repo + cas (TB-16.x.fix f2bb871 closure
      honored); audit_tape invoked WITHOUT --markov-pointer +
      --prior-chain-runtime-repo (genesis chain mode); TerminalSummary tx
      ×3 in chain provide future β-A resolver anchor points.
    - Atom F adds NO production code (only evidence + reproducibility
      script `handover/tests/scripts/run_tb_18_atom_f_2026-05-05.sh`).
    - Architect Q4 STOP gate "TB-18 cannot use α CLI sidecar to fake β-A
      success" structurally satisfied.

Q8. Wire integrity:
    - `cargo test --workspace --release` passes — verify 963/0/150 at HEAD
      `0c3a5e1` (baseline 939 + 24 new TB-18 tests through Atom F).
    - Trust root rehash entries are R-014 protocol compliant (predecessor
      lineage preserved; rationale captured in comment).

Q9. Anything missed:
    - Did the substrate ship surface forget anything required by
      architect §2.1-§2.10 + §4 + Q1-Q7?

Output verdict format:
  OVERALL: VETO | CHALLENGE | PASS
  Per-question (Q1-Q9): VETO | CHALLENGE | PASS + rationale
  Recommended pre-H remediations (if VETO/CHALLENGE).
```

## §4 What G0 verdict gates

Per TB-18 charter SG-18.15: "Codex micro-audit after F passes or issues are remediated before H."

- **VETO** → atom H BLOCKED until VETO resolved (re-audit).
- **CHALLENGE** → atom H proceeds with documented CHALLENGE response (commit message captures CHALLENGE-resolved status).
- **PASS** → atom H unblocked.

## §5 Why I (AI-coder) cannot run this autonomously

Per CLAUDE.md guidance: external audits (Codex / Gemini / dual-audit) require user-triggered cloud audit runs. /ultrareview is user-billed. AI-coder writes the audit scope doc; user invokes per their cloud-audit budget.

**To execute G0**: user runs `/ultrareview <branch>` OR invokes Codex against the 9-commit range with the Q1-Q9 prompt template above. Audit verdict file lands at `handover/audits/CODEX_MICRO_AUDIT_TB_18_PRE_H_VERDICT_2026-05-XX.md`.

### Quick-launch Codex prompt (copy/paste)

```text
Audit TB-18 atoms 0..F on branch main of turingosv4. HEAD is 0c3a5e1.
Substrate range: d3c8d78..0c3a5e1 (9 commits across 9 atoms: 0, E, A, H0,
D-design, C, B-design, B-impl, F).

Read these inputs first:
  handover/audits/CODEX_MICRO_AUDIT_TB_18_PRE_H_REQUEST_2026-05-05.md
  (you are looking at the request doc; §3 contains the Q1-Q9 audit
   questions binding for this run)
  handover/tracer_bullets/TB-18_charter_2026-05-05.md (binding spec)
  handover/directives/2026-05-05_TB18_CHARTER_RATIFICATION_ARCHITECT_RULING.md
  handover/evidence/tb_18_single_chain_13_of_13/README.md (atom F evidence)
  handover/evidence/tb_18_b_phase4_2026-05-05/README.md (atom B-impl evidence)

Verify each Q1-Q9 question in §3 of the request doc. Output verdict in the
format §3 mandates. Save your verdict to:
  handover/audits/CODEX_MICRO_AUDIT_TB_18_PRE_H_VERDICT_2026-05-05.md
```

## §6 Cross-references

- TB-18 charter §1.4 SG-18.15
- Architect TB-18 ratification ruling §2.1 + §3 + Q7
- TB-18 substrate commits: d3c8d78 / 8ad7a1d / 13a5ee0 / 5c40d06 / c025cdb / ae9530f / 7bb18b4 / 15b662c / 0c3a5e1
- Memory: `feedback_dual_audit` + `feedback_audit_after_evidence` (Q7 verdict source) + `feedback_audit_loop_roi_flip` (G0 is the ONE pre-H checkpoint per architect Q7 batch-audit compromise)

---

**Awaiting external Codex audit invocation.** Ready: 9-commit substrate sealed at HEAD `0c3a5e1`.
