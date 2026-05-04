#!/usr/bin/env bash
# Codex TB-16 ship audit — Class 3 integration smoke per architect §7.7
# (external audit MANDATORY at ship). Implementation-paranoid angle.
# Independent of Gemini ship audit (parallel, architectural strategic angle).
# Per memory feedback_dual_audit_conflict: VETO > CHALLENGE > PASS.

set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
ROUND="${TB16_AUDIT_ROUND:-R1}"
OUT="${ROOT}/handover/audits/CODEX_TB_16_SHIP_AUDIT_2026-05-04_${ROUND}.md"
TMP_PROMPT="$(mktemp /tmp/tb16_codex_ship.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

if [ -e "$OUT" ]; then
  echo "[codex tb-16] error: $OUT already exists; refusing to overwrite" >&2
  exit 2
fi

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex TB-16 Ship Audit — implementation-paranoid

**Role**: skeptical adversarial implementer-reviewer for the TB-16
(Controlled Market Smoke Arena) ship-gate dual external audit.
Independent of Gemini ship audit (parallel, architectural strategic
angle).

**Mandate**: TB-16 shipped under **Class 3 integration smoke envelope**
per architect §7.7 ("AI coder may implement autonomously, but ship
requires external audit"). **Per memory feedback_dual_audit_conflict**:
VETO > CHALLENGE > PASS. **Round cap = 2** per
feedback_elon_mode_policy. **ROI flip stop** per
feedback_audit_loop_roi_flip if R2 challenges shift to test-scaffold
edges.

## Audit target — architect §7 spec verbatim (THIS IS THE GROUND TRUTH)

```text
7. TB-16 — Controlled Market Smoke Arena

7.1 目标
在受控沙盒中跑通: compute + position + complete set + price + mask + autopsy.
仍不开放真实市场。

7.2 Scenario
Lean task; multiple Agents; WorkTx FirstLong; ChallengeTx Short;
CompleteSet share inventory; PriceIndex updates; Boltzmann scheduler
selects next candidate; some agents lose positions; Autopsy generated.

7.3 Functional requirements
FR-16.1 At least 3 agents participate.
FR-16.2 At least one WorkTx creates FirstLongPosition.
FR-16.3 At least one ChallengeTx creates ShortPosition.
FR-16.4 At least one CompleteSetMintTx exists.
FR-16.5 At least one price update occurs.
FR-16.6 At least one Boltzmann mask event occurs.
FR-16.7 At least one AutopsyCapsule is generated.

7.4 Constitutional requirements
CR-16.1 Total Coin conserved.
CR-16.2 No ghost liquidity.
CR-16.3 No price overriding predicates.
CR-16.4 No raw failure broadcast.
CR-16.5 No real user funds.
CR-16.6 All activity replayable from ChainTape + CAS.
CR-16.7 All market activity is sandbox-labeled.

7.5 Ship gates
SG-16.1 Controlled market smoke produces replayable ChainTape.
SG-16.2 Dashboard shows positions, prices, masks, autopsies.
SG-16.3 No fake accepted nodes.
SG-16.4 Unsolved tasks show failure evidence / bankruptcy anchors.
SG-16.5 All market balances conserved.
SG-16.6 No unresolved evidence gaps.
SG-16.7 At least one loss -> autopsy path.
SG-16.8 Sandbox flag prevents real-money interpretation.

7.6 Forbidden
No public chain. No real-money market. No external domain.
No unbounded leverage. No AMM trading unless explicitly scoped.
No DPMM / pro-rata. No medical/legal/financial domains.
No production user funds.

7.7 Loop-mode instruction
Risk class: Class 3 integration smoke.
AI coder may implement autonomously, but ship requires external audit.
Halt if: any conservation failure; raw log leak; price-as-truth
behavior; non-sandbox funds used; unresolved evidence gap.
```

## TB-16 ship anchors

- Charter: `handover/tracer_bullets/TB-16_charter_2026-05-04.md`
- Ship status: `handover/ai-direct/TB-16_SHIP_STATUS_2026-05-04.md`
- Design (38 audit assertions): `handover/tests/REAL_LLM_COMPREHENSIVE_AUDIT_FROM_TAPE_DESIGN_2026-05-04.md`
- Audit pipeline evidence: `handover/evidence/tb_16_real_llm_arena_2026-05-04/`

```text
TB-16 commit chain:
  7d0d65b Atom 0 — charter ratified
  f7e5f0a Atom 1 — halt-trigger fixture (13 H1..H13 stubs)
  c0c890a Atom 2 — audit_assertions module (38 assertions × 8 layers)
  b4480d7 Atom 3 — audit_tape + audit_tape_tamper binaries
  4a7863e Atom 4 — dashboard §15 live regen + §16 SANDBOX banner
  36413c0 Atom 5 — comprehensive_arena evaluator harness scaffold
  3300fe2 Atom 6 SHIP (pre-audit) — run scripts + audit pipeline smoke

HEAD (3300fe2):
  cargo test --workspace = 905 passed / 0 failed / 150 ignored
  13/13 halt-triggers GREEN (tests/tb_16_halt_triggers.rs)
  Trust Root: GREEN (1 rehash: src/runtime/mod.rs Atom 2; src/bin/audit_dashboard.rs Atom 4)
```

## Files in scope

```text
NEW src/runtime/audit_assertions.rs       (Atom 2; ~960 LoC; 51 pub symbols)
NEW src/bin/audit_tape.rs                  (Atom 3; CLI ~180 LoC)
NEW src/bin/audit_tape_tamper.rs           (Atom 3; CLI ~280 LoC)
NEW experiments/minif2f_v4/src/bin/comprehensive_arena.rs (Atom 5; orchestrator scaffold ~370 LoC)
NEW tests/tb_16_halt_triggers.rs           (Atom 1; 13 halt fixtures)
NEW tests/tb_16_audit_tape_binary.rs       (Atom 3; 3 binary smoke tests)
NEW tests/tb_16_dashboard_live_regen.rs    (Atom 4; 2 dashboard tests)
NEW experiments/minif2f_v4/tests/tb_16_comprehensive_arena_smoke.rs (Atom 5; 2 tests)
NEW handover/tests/scripts/run_real_llm_arena.sh    (Atom 6; runner)
NEW handover/tests/scripts/audit_tape_smoke_test.sh (Atom 6; ship-gate)
MOD src/runtime/mod.rs                     (Atom 2; +pub mod audit_assertions)
MOD src/bin/audit_dashboard.rs             (Atom 4; live regen + §16 SANDBOX banner + DashboardReport.sandbox_run)
MOD genesis_payload.toml                   (rehash mod.rs + audit_dashboard.rs)
NEW handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/
    (verdict.json + verdict_replay.json + tamper_report.json +
     MARKOV_TB-16_2026-05-03.json + LATEST_MARKOV_CAPSULE.txt + dashboard.txt)
```

## Atom 6 v0 SCOPE NOTE (you are auditing this honestly)

The Atom 6 evidence shipped uses the TB-13 real-LLM smoke fixture as
the chain-backed substrate for the audit-pipeline smoke. **A fresh
6-task arena run that exercises FR-16.2..7 on a single chain is
deferred to Atom 6.1** (multi-task chain-continuation evaluator
extension) + mathlib build (user-side `lake exe cache get`). The
infrastructure (audit_assertions, audit_tape, audit_tape_tamper,
comprehensive_arena scaffold, dashboard live regen + §16 banner) is
ship-grade; the **fresh arena execution** is gated.

## Architect-mandated audit questions (FR-16.x + CR-16.x + SG-16.x)

**Q1 (FR-16.1)**: 8 sandbox-prefixed agents in `comprehensive_arena.rs`
preseed manifest (sandbox_preseed_pairs). Verify these IDs are sandbox-
prefixed AND that `runtime::audit_assertions::sandbox_prefix` admits
them all. Confirm Layer A #3 (sandbox_agent_prefix) PASSES on the
preseed.

**Q2 (FR-16.2..7)**: These are EXERCISE requirements — the arena must
emit each tx kind. Atom 6 ships infrastructure ONLY; fresh arena run
is Atom 6.1. Audit: is "infrastructure ready + fresh arena gated"
acceptable for ship per architect §7.7, or does §7.7 require an actual
13-tx-kind tape before architect signoff? Lookup architect §7.7
verbatim and verify.

**Q3 (CR-16.1 — conservation)**: `audit_assertions::assert_18_total_supply_conserved`
sums balances_t + escrows_t + stakes_t + conditional_collateral_t
against GENESIS_TOTAL_MICRO=30_000_000. Verify this matches
runtime::bootstrap::default_pput_preseed_pairs (architect §6.4
inheritance). Verify Layer D #21 (node_positions_excluded) + #22
(conditional_shares_excluded) STRUCTURALLY PROVE positions are NOT in
the supply sum (they would need to ALWAYS make `with_X != baseline`,
which requires non-empty index OR vacuous truth). Is the vacuous-truth
branch a real fence or a logic gap?

**Q4 (CR-16.4 / Layer F privacy)**: `assert_28_projection_no_autopsy_bytes`
canonical-encodes `q.tape_view_t` (the AgentVisibleProjection) and
checks for 32-byte runs of any private_detail_cid. Verify (a) the
encoding is the canonical one Agents would actually see; (b) the
32-byte run check catches Cid bytes if serialized (hint: serde_json
serializes [u8;32] as JSON array of decimals, NOT raw bytes). Is the
canonical_encode path the one that flows to Agents, or is there a
serde_json bypass? Trace `tape_view_t` flow from QState through
AgentVisibleProjection serialization in `runtime/{adapter,verify}.rs`.

**Q5 (CR-16.6 / replay)**: `assert_12_replay_state_root_matches_head`
+ `assert_16_replay_idempotent_across_calls`. Verify these run
`replay_full_transition` against the SAME entries twice and assert
identical state_root + ledger_root. Layer C #16 verdict_replay
byte-identity is the system-level acceptance gate. Is the replay path
in `audit_tape` identical to the replay path in production
`Sequencer::apply_one`? If they diverge, audit-from-tape verdict can
be PROCEED while production is broken.

**Q6 (CR-16.7 / SG-16.8 sandbox banner)**: `audit_dashboard.rs`
`render_section_16` renders SANDBOX banner when `sandbox_run=true`.
`detect_sandbox_run` walks L4 + agent_pubkeys for sandbox prefixes.
Verify: (a) the prefix list in `detect_sandbox_run` matches
`audit_assertions::sandbox_prefix` (consistency); (b) any
NON-sandbox agent_id in the chain causes sandbox_run=true to remain
true (the function early-returns on first match — is that the right
policy?). Architect §7.6 says "no production user funds" — what
prevents a sandbox-prefix agent from being conflated with a
production wallet? Confirm sandbox_run=true is an OR (any sandbox
match suffices) — but the deeper question: should a chain with MIXED
sandbox and non-sandbox IDs ALSO trip the banner, OR force a halt?

**Q7 (Layer H tamper / audit_tape_tamper binary)**: 3 corruption
modes — flip_l4_byte, flip_cas_byte, truncate_l4_ref. Verify each
mode actually corrupts something verify_chaintape's pipeline detects.
The `flip_byte_in_first_blob` helper picks ".git/objects/" first
non-empty file: this could be a tree object, not the L4 commit blob
itself. Does verify_chaintape detect tree-blob corruption (vs commit-
blob corruption)? If not, flip_l4_byte may sometimes produce
"detected" without actually testing L4 row tampering.

**Q8 (audit_tape verdict.json schema)**: Inspect
`handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json`.
Verify schema_version="v1/audit_tape_verdict"; tape_root has 6 fields
(l4_count, l4e_count, head_state_root_hex, head_ledger_root_hex,
cas_object_count, constitution_hash_hex); tx_kind_counts has 14 fields
(13 architect-required + reuse); assertions has ≥38 entries each with
{id, name, layer, result, detail}; passed+failed+halted+skipped sum
to assertions.len(); feature_coverage maps each TB to GREEN/YELLOW/RED;
verdict ∈ {PROCEED, BLOCK}.

**Q9 (R-022 backlinks)**: All 53 pub symbols in
`src/runtime/audit_assertions.rs` carry
`/// TRACE_MATRIX FC1-N34 + FC2-N31` doc-comments per R-022. Verify
this is genuine traceability (vs. boilerplate) — does any pub symbol
have a more specific FC role that should be cited? E.g. tamper
assertions are FC1-N35, not FC1-N34.

## Audit follow-ups (RQ — recursive questions; if R1 PASSES address these)

**RQ1 — Atom 6.1 readiness gap**: ship status §4 explicitly defers
multi-task chain continuation. Is this a CHARTER deficit (charter §3
Atom 6 implied a fresh 13-kind chain) or a legitimate phasing
(infrastructure first, fresh chain second)? Per
feedback_no_fake_menus, did the charter set the right expectation?

**RQ2 — H7 demonstration mode**: ship status §6 + evidence README
claim "H7 demonstrated live via TB-13 fixture's E #27 halt". This is
unintended demonstration — the fixture happens to have an evidence
gap because TB-13 didn't write evidence_capsule_cid to CAS. Should
ship status be more cautious in framing this as "demonstration" vs.
"accidentally-detected gap in upstream fixture"?

**RQ3 — Test count**: ship status §3 says 905 / 0 / 150 with +25 from
TB-15 baseline 759. Walk the math: 759 + 25 = 784, NOT 905. Does the
905 figure include sub-package counts (minif2f_v4 + gix_capability)
that weren't in the 759 baseline? If yes, the +25 framing is
misleading. Audit the ship-status §3 wording for fidelity.

**RQ4 — Architect §7.6 forbidden enforcement**: SG-16.8 verifies
sandbox flag PREVENTS real-money interpretation. But do any of the
audit assertions or fence mechanisms ACTIVELY BLOCK a non-sandbox
agent from entering the tape? Or is sandbox prefix advisory? Trace
through: if someone submits a TaskOpen with sponsor_agent="0xPROD",
what stops it? Layer A #3 would FAIL audit, but does it BLOCK
admission? (Architect §7.7 says HALT — "non-sandbox funds used".)
Verify: dispatch arms / submission gate enforce sandbox prefix at
sequencer level, OR does it only manifest at audit-tape time?

**RQ5 — Markov capsule continuity**: TB-16 audit_pipeline_smoke
emitted `MARKOV_TB-16_2026-05-03.json` with previous_capsule_cid=null
(genesis) — but TB-15 already shipped a Markov capsule chained from
its own genesis. Why didn't TB-16 chain previous_capsule_cid to TB-15
capsule? Is this a Markov-chain break or expected (different evidence
dirs, different chain heads)?

## Verdict format

End your audit with one of:

```text
## VERDICT: PASS
(All Q1-Q9 + RQ1-RQ5 cleared; ship is clean for Class 3 envelope.)
```

```text
## VERDICT: CHALLENGE
- <claim id> CHALLENGE: <one-line reason + line refs>
(round-2 will trigger feedback_elon_mode_policy auto-execute on
determinate-best surgical patch.)
```

```text
## VERDICT: VETO
- <claim id> VETO: <one-line BLOCKING reason + line refs>
(VETO blocks ship per feedback_dual_audit_conflict; VETO > CHALLENGE > PASS.)
```

Include conviction (low/medium/high) + recommendation (PROCEED to SHIP /
FIX-THEN-PROCEED / REDESIGN / RETRO-CLASS-4-PROMOTION).

Cite file:line for every finding.

Save your audit to: handover/audits/CODEX_TB_16_SHIP_AUDIT_2026-05-04_R1.md.

BRIEF_EOF

echo "  Codex audit prompt prepared at: $TMP_PROMPT" >&2
echo "  Output target: $OUT" >&2
echo "  Round: $ROUND" >&2
echo "  Invoking codex exec..." >&2

cat "$TMP_PROMPT" | codex exec --skip-git-repo-check --sandbox read-only --color never - > "$OUT.raw" 2>&1
EXIT=$?

if [ $EXIT -ne 0 ]; then
  echo "  codex exec returned exit code $EXIT" >&2
  echo "  partial output saved to $OUT.raw" >&2
fi

mv "$OUT.raw" "$OUT"
echo "  Audit saved: $OUT" >&2
exit $EXIT
