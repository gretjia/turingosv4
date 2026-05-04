#!/usr/bin/env bash
# Codex TB-15 ship audit — Class 2 envelope per charter §4 + architect §6.7
# Loop-mode instruction. Implementation-paranoid angle. Independent of
# Gemini ship audit (parallel, architectural strategic angle).
# Per memory feedback_dual_audit_conflict: VETO > CHALLENGE > PASS.

set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
ROUND="${TB15_AUDIT_ROUND:-R1}"
OUT="${ROOT}/handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_${ROUND}.md"
TMP_PROMPT="$(mktemp /tmp/tb15_codex_ship.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

if [ -e "$OUT" ]; then
  echo "[codex tb-15] error: $OUT already exists; refusing to overwrite" >&2
  exit 2
fi

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex TB-15 Ship Audit — implementation-paranoid

**Role**: skeptical adversarial implementer-reviewer for the TB-15
(Lamarckian Autopsy + Markov EvidenceCapsule) ship-gate dual external
audit. Independent of Gemini ship audit (parallel, architectural strategic
angle).

**Mandate**: TB-15 shipped under **Class 2 self-audit envelope** per
charter §4 ("Class 2 = self-audit per feedback_dual_audit hybrid-by-risk-
class; promotion to Class 3 trigger = AgentVisibleProjection mod or
sequencer dispatch arm beyond TaskBankruptcyTx"). User retroactively
requested dual audit on 2026-05-04 to verify the Class 2 envelope held
and the new sequencer hook (TaskBankruptcyTx Step 3.5 + apply_one Stage
3.5) is sound. **Per memory feedback_dual_audit_conflict**: VETO >
CHALLENGE > PASS. **Round cap = 2** per feedback_elon_mode_policy. **ROI
flip stop** per feedback_audit_loop_roi_flip if R2 challenges shift to
test-scaffold edges.

## Audit target — architect §6 spec verbatim (THIS IS THE GROUND TRUTH)

```text
4. TB-15 — Lamarckian Autopsy + Markov EvidenceCapsule

4.1 目标
把失败、爆仓、亏损、反复错误转化为私有学习与 Markov capsule，
而不是全局 raw-log 污染。

4.2 新增对象
pub struct AgentAutopsyCapsule {
    pub capsule_id: Cid,
    pub agent_id: AgentId,
    pub event_id: EventId,
    pub loss_amount: MicroCoin,
    pub loss_reason_class: LossReasonClass,
    pub violated_risk_rule: Option<RiskRuleId>,
    pub suggested_policy_patch: Option<Cid>,
    pub evidence_cids: Vec<Cid>,
    pub public_summary: String,
    pub private_detail_cid: Cid,
}
pub struct MarkovEvidenceCapsule {
    pub capsule_id: Cid,
    pub previous_capsule_cid: Option<Cid>,
    pub constitution_hash: Hash,
    pub l4_root: Hash,
    pub l4e_root: Hash,
    pub cas_root: Hash,
    pub typical_errors: Vec<TypicalErrorSummary>,
    pub unresolved_obs: Vec<ObsId>,
    pub next_session_context_cid: Cid,
}

4.3 Functional requirements
FR-15.1 Loss / bankruptcy / failed market event creates AgentAutopsyCapsule.
FR-15.2 Autopsy uses ChainTape/CAS evidence, not self-narration.
FR-15.3 MarkovEvidenceCapsule generated at end of TB/run.
FR-15.4 Next InitAI context defaults to constitution + latest capsule.
FR-15.5 Markov override is required for deep history reads.
FR-15.6 Public summary can broadcast typical error; private detail remains scoped.

4.4 Constitutional requirements
CR-15.1 Raw failure logs are not broadcast globally.
CR-15.2 Autopsy is private/scoped unless error becomes typical.
CR-15.3 ArchitectAI may propose improvements from logs, but cannot mutate constitution.
CR-15.4 JudgeAI/VetoAI remains veto-only.
CR-15.5 Capsules are evidence compression, not hidden source of truth.
CR-15.6 Markov default prevents context poisoning.

4.5 Ship gates
SG-15.1 Failed/losing agent gets private AutopsyCapsule.
SG-15.2 Raw private details do not enter other Agent read view.
SG-15.3 Latest Markov capsule can bootstrap next session.
SG-15.4 Deep-history read without override fails.
SG-15.5 Typical error broadcast uses summary, not raw log.
SG-15.6 Dashboard can regenerate capsule summary from ChainTape + CAS.
SG-15.7 Markov capsule references constitution hash and flowchart hashes.
SG-15.8 Autopsy does not mutate predicates/tools automatically.

4.6 Forbidden
No global raw autopsy broadcast.
No forced prompt stuffing of all past failures.
No automatic predicate mutation.
No MetaTape self-modification.
No constitution change.
No hidden source-of-truth capsule.
No private loss detail in public read view.

4.7 Loop-mode instruction
Class 2 unless touching Agent read-view gating deeply; Class 3 if
privacy/security gates are modified. Halt if raw logs leak to general
read view or Markov capsule becomes hidden source of truth.
```

## TB-15 ship anchors

- Charter: `handover/tracer_bullets/TB-15_charter_2026-05-03.md`
- Ship status: `handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md`
- Decision: `handover/alignment/DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN_2026-05-02.md`
- First Markov capsule: `handover/evidence/tb_15_markov_capsule_2026-05-03/`

```text
TB-15 commit chain:
  a14e01e  Atom 0 — charter ratified
  0316a81  Atom 1 — halt-trigger fixture (6 unimplemented stubs)
  6594d3f  Atom 2 — AgentAutopsyCapsule schema + writer
  f06d548  Atom 3 — AutopsyIndex + TaskBankruptcyTx wire-in
  a17f6ac  Atom 4 — TypicalErrorBroadcast clustering (cluster_autopsies)
  31be856  Atom 5 — MarkovEvidenceCapsule schema + generator binary
  2337381  Atom 6 SHIP — dashboard §15 + first Markov capsule

HEAD (2337381):
  cargo test --workspace = 878 passed / 0 failed / 150 ignored
                          (+75 net vs TB-14 ship 803)
  6/6 halt-triggers GREEN (tests/tb_15_halt_triggers.rs)
  Trust Root: GREEN (6 rehashes propagated)
```

## Files in scope

```text
NEW src/runtime/autopsy_capsule.rs    (Atoms 2 + 3 + 4; ~600 LoC + tests)
NEW src/runtime/markov_capsule.rs     (Atom 5; ~500 LoC + tests)
NEW src/bin/generate_markov_capsule.rs (Atom 5 CLI; ~250 LoC)
NEW tests/tb_15_halt_triggers.rs      (Atoms 1-5; 6 fixtures)
MOD src/state/typed_tx.rs             (Atom 2; +RiskRuleId newtype)
MOD src/bottom_white/cas/schema.rs    (Atom 2; +4 ObjectType variants)
MOD src/state/q_state.rs              (Atom 3; +AutopsyIndex newtype + agent_autopsies_t 13th sub-field)
MOD src/state/sequencer.rs            (Atom 3; TaskBankruptcyTx Step 3.5 + apply_one Stage 3.5)
MOD src/runtime/mod.rs                (Atoms 2 + 5; +pub mod autopsy_capsule + markov_capsule)
MOD src/bin/audit_dashboard.rs        (Atom 6; +render_section_15 + 4 SG-15.6 tests)
MOD genesis_payload.toml              (rehash for 6 modified files)
MOD tests/economic_state_reconstruct.rs (sub-field count 12→13)
MOD tests/q_state_reconstruct.rs      (sub-field count 12→13)
MOD tests/six_axioms_alignment.rs     (axiom_3 12→13)
MOD tests/fc_alignment_conformance.rs (+4 TB-15 witnesses)
```

## Architect-mandated audit questions (FR-15.x + CR-15.x + SG-15.x)

**Q1 (FR-15.1 / SG-15.1)**: Loss/bankruptcy/failed-market event creates
AgentAutopsyCapsule. TB-15 v0 wires ONE trigger: TaskBankruptcyTx
dispatch arm Step 3.5. Read `src/state/sequencer.rs:1331-1400`
(TaskBankruptcy arm). Verify: (a) the dispatch arm calls
`derive_autopsies_for_bankruptcy` with the PRE-bankruptcy snapshot
(`&q.economic_state_t`, NOT `q_next.economic_state_t`); (b) Cids are
inserted into `q_next.economic_state_t.agent_autopsies_t[event_id]`;
(c) the apply_one Stage 3.5 hook (`src/state/sequencer.rs:3055-3083`)
writes the capsule + private_detail bytes to CAS using the SAME
deterministic helper. Confirm dispatch and apply_one Cids agree
(replay-safety contract).

**Q2 (FR-15.2 / DECISION_LAMARCKIAN §1.2 hard prohibition B)**: Autopsy
uses ChainTape/CAS evidence, NOT self-narration. Read
`src/runtime/autopsy_capsule.rs::derive_autopsies_for_bankruptcy`
(~lines 480-540). Verify: (a) inputs are `&EconomicState` +
`&TaskBankruptcyTx` + 2 `u64`s; NO `&str` payload, NO LLM-side
narrative param; (b) `private_detail` JSON is built deterministically
from `stake_tx_id` + `staker.0` + `amount.micro_units()` only — no
free-text fields, no self-report. Cite the line where private_detail
is constructed.

**Q3 (FR-15.3 / SG-15.3)**: MarkovEvidenceCapsule generated at end of
TB/run; can bootstrap next session. Read
`src/bin/generate_markov_capsule.rs` + `src/runtime/markov_capsule.rs::write_markov_capsule`.
Verify: (a) capsule is CAS-resident as `ObjectType::MarkovEvidenceCapsule`;
(b) `next_session_context_cid` resolves to a JSON blob with
`boot_seq`; (c) `LATEST_MARKOV_CAPSULE.txt` pointer file is written
with capsule_id hex; (d) the generated capsule for TB-15 itself
(handover/evidence/tb_15_markov_capsule_2026-05-03/) has
constitution_hash matching sha256(constitution.md) at generation time.

**Q4 (FR-15.4 / FR-15.5 / SG-15.4 / CR-15.6 / halt-trigger #6)**: Markov
override required for deep-history reads; default-deny works. Read
`src/runtime/markov_capsule.rs::try_deep_history_read_with_override_check`
+ `override_set_from_env`. Verify: (a) false → `Err(DeepHistoryReadDenied)`;
(b) true → `Ok(())`; (c) the binary's `run()` reads
`override_set_from_env()` exactly once and logs the decision.
**CRITICAL**: is the gate ACTUALLY enforced anywhere in the
ingest path, or is it only a structural fixture? Trace whether
generate_markov_capsule binary's normal flow can ever reach a
"deep-history read" code path without going through the gate.

**Q5 (FR-15.6 / CR-15.2 / SG-15.5 / halt-trigger #5)**: Typical-error
broadcast uses public_summary, NEVER private_detail bytes. Read
`src/runtime/autopsy_capsule.rs::cluster_autopsies` +
`TypicalErrorSummary` struct. Verify: (a) the output struct's fields
are `loss_reason_class` + `count` + `exemplar_public_summary: String`
+ `exemplar_capsule_cids: Vec<Cid>` — no `private_detail_cid` field;
(b) the `exemplar_public_summary` is built from each input capsule's
`public_summary`, NOT from `private_detail_cid`. Run the
halt-trigger #5 test mentally and confirm it actually catches the
violation (input `Cid([0xAA;32])` byte run not present in serialized
output).

**Q6 (CR-15.1 / SG-15.2 / halt-trigger #1 / halt-trigger #4)**: Raw
private details do NOT enter `AgentVisibleProjection`. Read
`src/state/q_state.rs:131-135` (AgentVisibleProjection struct).
Verify: (a) the struct has only `views: BTreeMap<AgentId, NodeId>` +
`mask_set: BTreeSet<TxId>` (TB-14); NO `agent_autopsies_t` field, NO
AutopsyIndex field, NO AgentAutopsyCapsule field, NO private_detail_cid
field. Read `src/state/q_state.rs::AutopsyIndex` definition (around
line 740). Verify: (b) the value type is `Vec<Cid>`, NOT
`Vec<AgentAutopsyCapsule>`, NOT `Vec<u8>`. Confirm halt-trigger #4 file-
scan would catch a future regression.

**Q7 (CR-15.3 / CR-15.4 / SG-15.8 / halt-trigger #3)**: Autopsy does NOT
mutate predicates/tools automatically. Read
`src/runtime/autopsy_capsule.rs::write_autopsy_capsule` +
`derive_autopsies_for_bankruptcy` signatures. Verify: (a) NEITHER
function takes a mutable reference to `PredicateRegistry` /
`ToolRegistry` / `RiskPolicyRegistry`; (b) `suggested_policy_patch:
Option<Cid>` is opaque — the writer does not interpret or apply it; (c)
the file contains zero calls to `register_predicate` /
`unregister_predicate` / `patch_predicate` / `register_tool` /
`unregister_tool`. Confirm halt-trigger #3 file-scan is correctly
constructed (uses runtime-built byte literals to avoid self-trip).

**Q8 (SG-15.7 / halt-trigger #2)**: MarkovEvidenceCapsule references
constitution_hash. Read `src/runtime/markov_capsule.rs::MarkovEvidenceCapsule`
struct + `with_constitution_hash` + the binary's `sha256_of_file` call.
Verify: (a) `constitution_hash: Hash` field is required (not Option,
not serde-default-fallback); (b) the binary computes
`sha256_of_file(constitution.md)` and plumbs it through
write_markov_capsule unchanged; (c) for the actual emitted capsule
(`handover/evidence/tb_15_markov_capsule_2026-05-03/MARKOV_TB-15_2026-05-03.json`),
the recorded `constitution_hash` equals `sha256(constitution.md)` at
generation time. **CHALLENGE WELCOME**: spec also says "and flowchart
hashes" — does TB-15 capsule reference flowchart hashes? If not, is this
a deferred-with-justification or an unflagged spec gap?

**Q9 (SG-15.6)**: Dashboard regenerates capsule summary from ChainTape +
CAS. Read `src/bin/audit_dashboard.rs::render_section_15` (around line
1500+) + `read_latest_markov_pointer`. Verify: (a) the render function
signature accepts only `&[(String, u32)]` event counts + `Option<&str>`
Markov hex — STRUCTURALLY incapable of leaking raw bytes; (b) the
"AUTOPSY IS PRIVATE" banner is emitted; (c) 4 SG-15.6 unit tests in
`tb14_render_tests` mod cover banner / counts-only / empty-Markov
hint / default-deny explanation. **CHALLENGE WELCOME**: dashboard's
`autopsy_event_counts` field is empty Vec at TB-15 ship (build_report
doesn't rebuild EconomicState from chain). Is this an honest deferred-
to-TB-16, or does it break SG-15.6's "regenerate from ChainTape + CAS"
guarantee in any way that should have blocked ship?

## Implementation-paranoid scrutiny (RQ1-RQ8)

**RQ1 — derive_autopsies_for_bankruptcy idempotency under apply_one
re-execution**: the dispatch arm and apply_one BOTH call
`derive_autopsies_for_bankruptcy` with the same inputs. If apply_one is
re-run (e.g., crash recovery, replay), it will re-write the same CAS
bytes (Cid match → CAS dedupe). Trace the apply_one Stage 3.5 path and
confirm the CAS write failure mode is correct: what happens if CAS
write fails AFTER the dispatch arm already populated agent_autopsies_t
in q_next? Is the chain inconsistent, or does the error propagate as
ApplyError with no L4 commit?

**RQ2 — pre-snapshot vs post-snapshot for derive**: the dispatch arm
calls `derive_autopsies_for_bankruptcy(&q.economic_state_t, bk, ...)`
with the PRE-bankruptcy snapshot. The apply_one hook calls the same
helper with `&q_snapshot.economic_state_t` (also pre-bankruptcy). Are
both snapshots PROVABLY identical at the moment of derivation? If
either side accidentally used `q_next` (post-bankruptcy), the Cids
would diverge. Cite the EXACT line where each snapshot is captured.

**RQ3 — BTreeMap iteration determinism for derive**: the helper
iterates `pre_econ.stakes_t.0` (BTreeMap<TxId, StakeEntry>). Confirm:
(a) BTreeMap iteration is sorted by TxId (replay-deterministic);
(b) the output Vec preserves this order; (c) the agent_autopsies_t
Vec<Cid> push order matches dispatch and apply_one (or — if push
order doesn't matter — that the Vec is sorted before any equality
check).

**RQ4 — sub-field count discipline**: EconomicState bumped 12 → 13
sub-fields. Three test fixtures + one in-module assertion updated:
`tests/economic_state_reconstruct.rs` (twelve_ → thirteen_),
`tests/q_state_reconstruct.rs` (twelve_ → thirteen_),
`tests/six_axioms_alignment.rs` (axiom_3 12 → 13),
`src/state/q_state.rs::economic_state_has_thirteen_sub_fields`. Are
there OTHER tests in the workspace that hard-code "12" for EconomicState
sub-fields? Use `git grep "12"` or similar to look for hidden
hard-coded constants that would silently miss the bump.

**RQ5 — RiskRuleId newtype necessity**: `src/state/typed_tx.rs` adds
`pub struct RiskRuleId(pub String)` for AgentAutopsyCapsule's
`violated_risk_rule: Option<RiskRuleId>` field. TB-15 v0 NEVER
populates this field (always None — bankruptcy ≠ risk-rule violation).
Is this premature scaffolding (YAGNI) or necessary for forward
compatibility (architect §6.2 spec lists the field)? Defend or
challenge the choice.

**RQ6 — generate_markov_capsule binary error handling**: the binary's
`--no-cas` mode constructs the capsule deterministically without CAS
write. The full mode (`--cas-dir <path>`) writes to CAS via
`write_markov_capsule`. What happens if `--cas-dir` is provided but
the CasStore fails to open (path doesn't exist, permission denied,
git2 error)? Is the error propagation clean? Does it leave a partial
JSON pointer file?

**RQ7 — Markov capsule field "and flowchart hashes" drift**: SG-15.7
says "Markov capsule references constitution hash AND FLOWCHART HASHES"
(emphasis added). The shipped MarkovEvidenceCapsule struct has only
`constitution_hash: Hash` — no `flowchart_hashes: Vec<Hash>` or
similar. Is this a spec deviation? Audit-trace
`handover/alignment/TRACE_FLOWCHART_MATRIX.md` (if it exists) or any
prior TB doc that pins flowchart hashes — would the canonical
flowchart hashes be derivable from `previous_capsule_cid` chain alone?
Or is this an unflagged gap that should have been a charter
auto-resolution?

**RQ8 — Trust Root rehash chain integrity**: 6 trust_root entries
rehashed across Atoms 2/3/5/6 (src/runtime/mod.rs ×2 hops + q_state +
typed_tx + sequencer + cas/schema + audit_dashboard +
fc_alignment_conformance). Each rehash appended a new comment with
"Predecessor X superseded". Walk the genesis_payload.toml diff for
TB-15 commits and verify: (a) every modified file's hash is current;
(b) no stale "Predecessor X" claims (predecessor hash actually exists
in prior commit history).

## Verdict format

End your audit with one of:

```text
## VERDICT: PASS
(All Q1-Q9 + RQ1-RQ8 cleared; ship is clean for Class 2 envelope.)
```

```text
## VERDICT: CHALLENGE
- <claim id> CHALLENGE: <one-line reason + line refs>
- <repeat per challenge>
(round-2 will trigger feedback_elon_mode_policy auto-execute on
determinate-best surgical patch.)
```

```text
## VERDICT: VETO
- <claim id> VETO: <one-line BLOCKING reason + line refs>
(VETO blocks ship per feedback_dual_audit_conflict; VETO > CHALLENGE > PASS.)
```

Include conviction (low/medium/high) + recommendation (PROCEED to SHIP /
FIX-THEN-PROCEED / REDESIGN / RETRO-CLASS-3-PROMOTION).

Cite file:line for every finding.

Save your audit to: handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md.

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
