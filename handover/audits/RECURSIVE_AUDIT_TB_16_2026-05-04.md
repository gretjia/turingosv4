# TB-16 R1 Dual External Audit — Convergent VETO

**Date**: 2026-05-04 (Atom 7 R1 closure analysis)
**Status**: **R1 BOTH VETO** — autonomous loop HALTED.
**Auditors**: Codex (impl-paranoid) + Gemini 2.5 Pro (architectural strategic).
**Conservative resolution**: VETO > CHALLENGE > PASS per `feedback_dual_audit_conflict`.
**Round cap**: 2 per `feedback_elon_mode_policy`. R2 NOT executed — VETOs surface real
  production-code defects + scope deficits, not test-scaffold edges. Per
  `feedback_audit_loop_roi_flip`: STOP iterating; surface to user.

---

## §1 R1 verdicts

| Auditor | Verdict | Conviction | Recommendation | Source |
|---|---|---|---|---|
| Codex | **VETO** | high | FIX-THEN-PROCEED | `handover/audits/CODEX_TB_16_SHIP_AUDIT_2026-05-04_R1.md` |
| Gemini | **VETO** | high | REDESIGN / RE-SCOPE | `handover/audits/GEMINI_TB_16_SHIP_AUDIT_2026-05-04_R1.md` |
| **Conservative** | **VETO** | high | **HALT autonomous; user decision required** | this doc |

---

## §2 Convergent VETOs (both auditors)

### V1 — TB-16 arena execution NOT delivered
- **Codex Q2/RQ1 VETO**: "TB-16 did not ship the required arena run. Architect requires
  compute + position + complete set + price + mask + autopsy and FR-16.2..7 before ship
  (`handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md:764-814`),
  but ship status explicitly defers FR-16.2..7 to Atom 6.1
  (`handover/ai-direct/TB-16_SHIP_STATUS_2026-05-04.md:33-46`). The runner only
  executes Task A and defers B-F (`handover/tests/scripts/run_real_llm_arena.sh:141-188`)."
- **Gemini Q5 VETO**: "The core deliverable is missing. Architect §7.3 requires a
  chain exercising all market functions; the ship status confirms this was deferred to Atom 6.1."
- **Convergent reading**: architect §7.3 is a SHIP-time spec; "infrastructure ready,
  arena gated" was a charter-level deferral that **was not architect-ratified**. Per
  `feedback_no_fake_menus`, the deferral framing in ship status §4 buries a deficit.

### V2 — Sandbox banner is advisory, not admission-enforced
- **Codex Q1/Q6/RQ4 VETO**: "sandbox is advisory, not admission-enforced.
  `sandbox_prefix` excludes `Agent_0..9` (`src/runtime/audit_assertions.rs:538-545`),
  while default preseed uses `Agent_0..9` (`src/runtime/bootstrap.rs:52-69`). Layer A
  #3 only scans `agent_pubkeys.json` (...:588-603), and shipped manifest is empty
  (.../runtime_repo/agent_pubkeys.json:1-3). `submit_agent_tx` has no sandbox-prefix
  gate (`src/state/sequencer.rs:2457-2484`)."
- **Gemini Q4 VETO**: "The sandbox check in `audit_dashboard.rs:818` only renders a
  banner on mixed-ID chains; it fails to enforce the architect's §7.7 HALT condition
  for 'non-sandbox funds used'."
- **Convergent reading**: §7.7 mandates HALT on non-sandbox funds; current implementation
  only flags AT AUDIT TIME, after the chain has already accepted production-pattern
  agents. Production-side admission gate missing.

---

## §3 Codex-additional VETOs

### V3 — Shipped evidence is BLOCK (not PROCEED)
- **Q2/Q8 VETO**: "shipped evidence is `verdict=BLOCK`, with missing evidence capsule
  halt. Tx counts show no Work/Verify/Challenge/CompleteSet/MarketSeed/etc.
  (`.../verdict.json:11-25`), Layer E #27 halts (`.../verdict.json:210-216`), and final
  verdict is BLOCK (`.../verdict.json:300-320`). Architect §7.7 says unresolved evidence
  gap is a halt (architect §7.7:901-908)."
- The audit_pipeline_smoke evidence README framed this as "H7 demonstrated live"; the
  auditor reads it as a **failed acceptance gate**.

### V4 — Runner script masks failures
- **Q2/Q7 VETO**: "the runner masks audit failures. `audit_tape`, tamper, Markov,
  dashboard, and replay are invoked with `|| true`
  (`handover/tests/scripts/run_real_llm_arena.sh:189-244`), and the script has no final
  nonzero exit on BLOCK/replay divergence (.../run_real_llm_arena.sh:257-267). Full
  tx-kind coverage is warning-only by default
  (`handover/tests/scripts/audit_tape_smoke_test.sh:25-31`, `:82-104`)."
- The script's intent (continue on per-step error to collect maximum diagnostic data)
  conflicts with ship-gate enforcement.

### V5 — Tamper detection BLOCK-baseline contamination
- **Q7 VETO**: "tamper detection is contaminated by an already-BLOCK baseline. The
  harness treats any `verdict == "BLOCK"` as detected (`src/bin/audit_tape_tamper.rs:333-336`);
  in evidence, `flip_l4_byte` still has L4 hash chain valid passing
  (`.../tamper_report.json:35-40`) and is 'detected' by the same E #27 halt
  (`.../tamper_report.json:198-201`)."
- **Defect**: when the baseline tape is already BLOCK, ANY tampering is "detected"
  trivially because the verdict was already BLOCK before tampering. The 3/3 detection
  rate is therefore **inflated** — true tamper-fence efficacy is unproven.

### V6 — Conservation invariant omits ChallengeTx bond
- **Q3 VETO**: "audit conservation is not production-equivalent for ChallengeTx.
  Audit sum omits `challenge_cases_t.bond` (`src/runtime/audit_assertions.rs:1020-1034`),
  while production monetary invariant includes it
  (`src/economy/monetary_invariant.rs:174-209`) and ChallengeTx moves bond there
  (`src/state/sequencer.rs:851-866`)."
- **Real production divergence**: audit_tape's Layer D #18 sums 4 holdings (balances +
  escrows + stakes + collateral) but production sums 5 (+ challenge_cases_t.bond).
  Audit can verify a chain "conserved" while production fails its own invariant.

---

## §4 Gemini-additional VETO

### V7 — Markov chain continuity break
- **Q8 VETO**: "The generated Markov capsule breaks evidence continuity
  (`previous_capsule_cid=null`), violating the 'no isolated islands' principle from
  architect spec CR-15.5."
- TB-15 already shipped a Markov capsule. TB-16's capsule should chain to it via
  `--prev-cid-hex`; the runner doesn't pass it.

---

## §5 CHALLENGEs (both auditors; aggregate)

| ID | Source | Defect | Surgical fix |
|---|---|---|---|
| Q4 | Gemini | byte-run privacy check uses canonical_encode but Cid serializes as JSON-decimal-array; check is ineffective for serde_json path | Add JSON-array form check (already in TB-15 halt-trigger #5; replicate to TB-16 #28) |
| Q4 | Codex | projection check targets `q.tape_view_t` not actual `UniverseSnapshot` agent path | Trace `bus.rs:508-558` agent prompt path; assert no autopsy bytes there |
| Q5 | Codex | replay differs from production `apply_one` at system-sig + autopsy-CAS side effects | Verify replay parity at `transition_ledger.rs:468-510` + `sequencer.rs:3026-3118` |
| Q6 | Gemini | risk envelope mis-classified Class 3 (read-only audit tooling) | Re-evaluate; possibly Class 2 self-audit was correct |
| Q9 | Gemini | ship status §4 misrepresents charter §3 deficit as "Atom 6.1 follow-up" | Honest ship-status rewrite |
| Q11 | Gemini | TRACE_MATRIX backlinks mis-attribute tamper assertions to FC1-N34 instead of FC1-N35 | Surgical doc edit on assertions #36-#38 |
| Q12 | Gemini | "+25 over baseline 759" math doesn't reconcile (759+25=784, not 905) | Honest test-count breakdown |

---

## §6 Why R2 is NOT auto-executed

Per `feedback_audit_loop_roi_flip`: when audit findings shift to **real production-code
defects** (not test-scaffold edges), iteration ROI has flipped. STOP and surface.

The R1 VETOs name:
- **Real production gate gap** (V2 — sandbox admission) — needs sequencer-level change
- **Real charter scope deficit** (V1 — arena run not shipped) — needs user-side mathlib
  build + Atom 6.1 multi-task evaluator extension OR re-charter as infrastructure-only
  with Class 2 downgrade
- **Real production-vs-audit divergence** (V6 — conservation sum) — needs audit_assertions
  to import production `assert_total_ctf_conserved` instead of inlining
- **Real runner script enforcement gap** (V4 — `|| true`) — needs script rewrite
- **Real tamper-baseline contamination** (V5) — needs BLOCK-baseline detection logic

These are NOT test-scaffold subtleties. They are ship-blocking defects that require
deliberate user decision on:

1. **Re-scope path A** — re-charter TB-16 as infrastructure-only with Class 2
   downgrade; ship Atom 6 as-is plus surgical fixes for V3/V4/V5/V6/V7. Spawn separate
   TB-16.1 charter for the actual arena run (architect §7.3 spec compliance).
2. **Re-scope path B** — keep TB-16 Class 3; build mathlib + extend evaluator for
   multi-task chain continuation; produce fresh arena run with all 13 tx kinds; fix
   V2 sandbox admission gate; re-audit. Estimated effort: 2-5 atom days.
3. **Re-scope path C** — accept R1 VETOs as authoritative; revert TB-16 commits 0..6;
   re-charter TB-16 from scratch with V1-V7 incorporated upfront.

---

## §7 Recommendation (per `feedback_architect_deviation_stance`)

**Take an explicit position**:

> Path A is the lowest-risk continuation. TB-16 Atoms 0-6 ship REAL infrastructure
> (38-assertion battery, audit_tape binaries, dashboard live regen + §16 SANDBOX
> banner, comprehensive_arena scaffold, run scripts, halt-trigger fixture). The
> gap is the **fresh arena execution** — which is genuinely user-side gated
> (mathlib build + multi-task evaluator extension). Surgical fixes for V3/V4/V5/V6/V7
> are 1-2 atom hours each. Re-charter as Class 2 self-audit envelope with explicit
> deferral of architect §7.3 fresh-arena requirement to a successor TB-16.1.
>
> Path B is the architect-spec-faithful continuation but requires multi-day
> investment (mathlib build, evaluator multi-task extension, sandbox admission
> gate, fresh arena run, full re-audit).
>
> Path C is the most conservative but discards 6 atoms of shipped infrastructure
> that is independently useful for ANY future TB.

**Recommended**: **Path A** (re-scope as infrastructure ship; spawn TB-16.1).

User decision required.

---

## §8 Surgical fix list (if Path A or fix-then-proceed)

**Quick wins (each ≤1 hour)**:

1. **V3 fix** — generate fresh chain with NO E #27 halt as audit_pipeline_smoke fixture
   (or use TB-14 chaintape smoke fixture; needs chain head with non-empty CAS for every
   TerminalSummary).
2. **V4 fix** — strip `|| true` from `run_real_llm_arena.sh` Steps 5-8; require explicit
   `--continue-on-error` flag if continuing past failures.
3. **V5 fix** — `audit_tape_tamper.rs:333-336` — pre-run audit on UNTAMPERED copy;
   `detected = !post_tamper_proceeds && pre_tamper_proceeds`.
4. **V6 fix** — `audit_assertions.rs:1020-1034` — replace inline 4-holding sum with
   call to `monetary_invariant::total_supply_micro` (or import the 5-holding sum
   formula).
5. **V7 fix** — `run_real_llm_arena.sh:201` — pass `--prev-cid-hex` from
   `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt` to chain TB-16 capsule to TB-15.
6. **Q11/Q12 fixes** — surgical doc edits.

**Hard fixes (multi-hour or needs architect ratification)**:

- V1 (fresh arena run): user-side mathlib + multi-task evaluator extension OR re-charter.
- V2 (sandbox admission gate): sequencer-level dispatch arm modification (Class 3 work);
  needs design ratification — do we HALT non-sandbox tx, or merely flag?

---

## §9 Cross-references

- TB-16 charter: `handover/tracer_bullets/TB-16_charter_2026-05-04.md`
- Architect spec: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §7
- Codex R1 audit: `handover/audits/CODEX_TB_16_SHIP_AUDIT_2026-05-04_R1.md`
- Gemini R1 audit: `handover/audits/GEMINI_TB_16_SHIP_AUDIT_2026-05-04_R1.md`
- Ship status: `handover/ai-direct/TB-16_SHIP_STATUS_2026-05-04.md`
- Audit pipeline evidence: `handover/evidence/tb_16_real_llm_arena_2026-05-04/`
- Memory (relevant): `feedback_dual_audit_conflict`, `feedback_audit_loop_roi_flip`,
  `feedback_audit_obs_bias`, `feedback_no_fake_menus`, `feedback_architect_deviation_stance`,
  `feedback_iteration_cap_24h`, `feedback_risk_class_audit`, `feedback_elon_mode_policy`.
