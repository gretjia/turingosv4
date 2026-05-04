OpenAI Codex v0.128.0 (research preview)
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: read-only
reasoning effort: xhigh
reasoning summaries: none
session id: 019df0e0-4b76-7e42-87d0-83e713f4534a
--------
user
# Codex TB-15 R3 Ship Audit — POST-R2-VETO-CLOSURE

## R3 CONTEXT — Codex R2 VETO closure (READ THIS FIRST)

**Codex R2 verdict (2026-05-04 R2)**: VETO conviction=high, FIX-THEN-PROCEED.
- **Q3 VETO**: `LATEST_MARKOV_CAPSULE.txt` published `a94ae884...` but CAS index keyed Markov object under `e4932fca...`. `cas.get(&capsule.capsule_id)` would FAIL. Root cause: `capsule_id = sha256(prelim_bytes)` (with capsule_id+sha256 zeroed during hash) but `cas.put(final_bytes)` stored the post-population bytes whose sha256 differs. Breaks SG-15.3 next-session bootstrap.
- **TB15-CAS-ID VETO**: same self-CID/content-CID mismatch in `write_autopsy_capsule` and `derive_autopsies_for_bankruptcy`.

(Gemini R2 verdict was PASS; only minor OBS items. Codex caught this real production bug.)

**R3 remediations applied** (verify each was correctly implemented):

1. **Q3 VETO closure (write_markov_capsule)**: writer rewritten so capsule_id == sha256(stored_bytes). Specifically: build capsule with capsule_id=Cid::default()/sha256=Hash::ZERO; canonical_encode → those are the bytes stored in CAS; capsule_id = Cid::from_content(stored_bytes); populate the in-memory struct fields after CAS write. The on-CAS bytes have capsule_id+sha256 zeroed; the in-memory struct returned to caller has them populated. cas.get(&capsule.capsule_id) succeeds. NEW helper `restore_markov_capsule_from_cas_bytes(bytes)` canonical-decodes + re-derives capsule_id/sha256 from `Cid::from_content(&bytes)`.

2. **TB15-CAS-ID VETO closure (write_autopsy_capsule + derive_autopsies_for_bankruptcy)**: same writer pattern fix. derive_autopsies_for_bankruptcy now returns `Vec<BankruptcyAutopsyDerivation>` (struct with `capsule + private_bytes + stored_capsule_bytes` fields) instead of `Vec<(AgentAutopsyCapsule, Vec<u8>)>`. The dispatch arm reads `d.capsule.capsule_id`; apply_one writes `d.stored_capsule_bytes` (the EXACT bytes whose sha256 == capsule_id). NEW helper `restore_autopsy_capsule_from_cas_bytes(bytes)`.

3. **NEW round-trip tests**:
   - `runtime::markov_capsule::tests::write_markov_capsule_cas_resolvable_by_capsule_id` — asserts `cas.get(&cap.capsule_id)` succeeds + retrieved bytes' sha256 == capsule_id + restore round-trip works.
   - `runtime::autopsy_capsule::tests::write_bankruptcy_autopsies_to_cas_round_trip` — extended with same R3 contract.

**R3 baseline**:
- `cargo test --workspace` = 882 PASS / 0 fail / 150 ignored
- R3 capsule_id `f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312`
- CAS index for MarkovEvidenceCapsule: `f9e701b4...` (MATCHES `LATEST_MARKOV_CAPSULE.txt`)
- Trust Root: GREEN (sequencer.rs rehashed `f4119a41 → 0453e735`; fc_alignment_conformance.rs rehashed `5e257f27 → 904cb5f5`)
- R3 evidence at `handover/evidence/tb_15_markov_capsule_2026-05-04/` includes `cas_index.jsonl` proving Cid match

## R3 mandate

**Specifically verify the Codex R2 VETO findings are now closed**:

- **Q3 closure check**: read `src/runtime/markov_capsule.rs::write_markov_capsule`. Confirm:
  (a) capsule struct built with capsule_id=Cid::default(), sha256=Hash::ZERO BEFORE canonical_encode
  (b) `stored_bytes = canonical_encode(&capsule)` happens BEFORE populating capsule_id+sha256
  (c) `cid = Cid::from_content(&stored_bytes)` is the value plumbed into capsule.capsule_id
  (d) `cas.put(&stored_bytes, ...)` stores the SAME bytes Cid was computed from
  (e) `debug_assert_eq!(cas_returned_cid, cid)` enforces the contract at runtime
  (f) `restore_markov_capsule_from_cas_bytes` correctly reverses the operation
  (g) the R3 round-trip test actually exercises `cas.get(&cap.capsule_id)` and passes

- **TB15-CAS-ID closure check**: same checks for `write_autopsy_capsule` AND `derive_autopsies_for_bankruptcy` AND `write_bankruptcy_autopsies_to_cas`. Confirm `BankruptcyAutopsyDerivation` correctly carries `stored_capsule_bytes` from derive to CAS write.

- **Cross-cut concern**: TB-11 `write_evidence_capsule` likely has the SAME bug (predates TB-15; same prelim/final pattern). Is this a new VETO that should block TB-15 ship, or is it acceptable to OBS-defer because no production consumer of EvidenceCapsule via cap.capsule_id exists yet?

- **NEW findings welcome**, but per `feedback_audit_loop_roi_flip`: if R3 challenges shift to test-scaffold edges, recommend ship-with-OBS rather than R4.

Emit a fresh verdict (PASS / CHALLENGE / VETO).

---

## R2 CONTEXT (for reference) — R1 verdict + R2 remediation summary

**R1 dual-audit verdicts (2026-05-04 R1)**:
- Gemini R1: VETO Q12 (replay-determinism for pre-TB-15 chain replay) + CHALLENGE Q7 (flowchart_hashes missing)
- Codex R1: CHALLENGE × 5 — Q3 (--no-cas weakness), Q4 (override gate not enforced live), Q5 (halt-trigger #5 byte-window weakness), Q8/RQ7 (flowchart_hashes missing, same as Gemini Q7), Q9 (dashboard not actually regenerable)

**R2 remediations applied** (verify each was correctly implemented):

1. **Q12 VETO closure (activation gate)**:
   - NEW `pub const TB15_AUTOPSY_ACTIVATION_LOGICAL_T: u64 = 0;` in `src/runtime/autopsy_capsule.rs`
   - NEW `pub fn is_autopsy_active_at(timestamp_logical: u64) -> bool` predicate
   - Both `dispatch_transition` TaskBankruptcyTx Step 3.5 AND `apply_one` Stage 3.5 now wrapped in `if is_autopsy_active_at(bk.timestamp_logical)` gate
   - Verification baseline: ZERO production chains contain TaskBankruptcyTx (grep across 10 evidence runtime_repo dirs on 2026-05-04)
   - Default const = 0 keeps fresh chains always-active; pre-TB-15 chain migration would override

2. **Q7/Q8 (flowchart_hashes) closure**:
   - NEW field `pub flowchart_hashes: Vec<Hash>` on `MarkovEvidenceCapsule` (additive, `#[serde(default)]`)
   - NEW `pub fn read_flowchart_hashes_from_matrix(path)` parses TRACE_FLOWCHART_MATRIX.md §2 → 4 canonical hashes (1a, 1b, 2, 3)
   - Generator binary now reads + populates the field; halt-trigger #2 strengthened to verify the field is set
   - R2 capsule emitted: `handover/evidence/tb_15_markov_capsule_2026-05-04/MARKOV_TB-15-R2_2026-05-03.json`

3. **Q3 (CAS residency) closure**:
   - R2 generator run dropped `--no-cas`; capsule + next_session_context written to real CAS at `/tmp/tb15-r2-cas/`
   - CAS git internals contain the objects

4. **Q4 (live override gate) closure**:
   - NEW CLI arg `--include-prior-capsules N` on generator binary
   - When N > 0, binary calls `try_deep_history_read_with_override_check(override_set_from_env())` BEFORE any deep-history I/O
   - Default-deny path returns exit code 3; override path proceeds

5. **Q5 (halt-trigger #5 byte-window weakness) closure**:
   - Strengthened to scan BOTH (a) the canonical Cid JSON-array text form (`[170,170,...,170]` for `Cid([0xAA;32])`) AND (b) raw 32-byte run AND (c) canonical_encode bytes

6. **Q9 (dashboard not regenerable) — OBS-deferred to TB-16**:
   - OBS doc at `handover/alignment/OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04.md`
   - Privacy contract HOLDS (input signature `&[(String, u32)]` + `Option<&str>` is structurally byte-leak-proof)
   - Live rebuild requires walking replayed EconomicState → naturally TB-16 controlled-arena scope

**R2 baseline**:
- `cargo test --workspace` = 881 PASS / 0 fail / 150 ignored (+3 new tests vs R1 ship 878)
- R2 capsule_id `a94ae8848f66d1335285bd9800cf2dd161ea69fa601502cbe3a2bccc26fe3132`
- Trust Root: GREEN (sequencer.rs rehashed `9fa59362 → f4119a41`)
- All 6 halt-triggers GREEN (R2 strengthened #2 + #5)

## R2 mandate

Re-audit each R1 finding with these specific questions:
- Q12 (Gemini VETO): is the activation gate STRUCTURALLY sound? `is_autopsy_active_at(bk.timestamp_logical)` gates both dispatch + apply_one identically. Are there edge cases (timestamp_logical wraparound, off-by-one)?
- Q7/Q8: is the `flowchart_hashes` field correctly wired? Do the 4 hashes match architect §2 verbatim?
- Q3: does the R2 capsule prove CAS residency?
- Q4: is the live override gate genuinely enforced? Any bypass path?
- Q5: does the strengthened halt-trigger #5 catch JSON-serialized Cid leaks?
- Q9: is the OBS deferral honest? Privacy contract still holds?

Emit a fresh verdict (PASS / CHALLENGE / VETO). NEW findings welcome; old findings must be tested for closure.

Per `feedback_audit_loop_roi_flip`: if R2 CHALLENGEs shift from production-code defects to test-scaffold edges, that's the ROI-flip stop signal — recommend ship-with-OBS rather than R3.

---

## Original R1 audit context (verbatim, for reference) — R2 mandate above takes precedence

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


warning: Codex could not find bubblewrap on PATH. Install bubblewrap with your OS package manager. See the sandbox prerequisites: https://developers.openai.com/codex/concepts/sandboxing#prerequisites. Codex will use the vendored bubblewrap in the meantime.
codex
I’ll audit the R3 closure against the current workspace and AGENTS guidance first, then check the exact writer/restore/test paths and any nearby evidence files. This sandbox is read-only, so I can verify and report, but I won’t be able to write the requested audit file unless the environment changes.
exec
/bin/bash -lc "pwd && rg --files -g 'AGENTS.md' -g '"'!target'"'" in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
/home/zephryj/projects/turingosv4

exec
/bin/bash -lc 'git status --short' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
 M genesis_payload.toml
 M handover/ai-direct/LATEST.md
 M handover/evidence/tb_13_chaintape_smoke_2026-05-03/README.md
 M handover/evidence/tb_13_chaintape_smoke_2026-05-03/agent_pubkeys.json
 M handover/evidence/tb_13_chaintape_smoke_2026-05-03/replay_report.json
 M handover/evidence/tb_14_chaintape_smoke_2026-05-03/README.md
 M handover/evidence/tb_14_chaintape_smoke_2026-05-03/agent_pubkeys.json
 M handover/evidence/tb_14_chaintape_smoke_2026-05-03/genesis_report.json
 M handover/evidence/tb_14_chaintape_smoke_2026-05-03/pinned_pubkeys.json
 M handover/evidence/tb_14_chaintape_smoke_2026-05-03/replay_report.json
 M handover/evidence/tb_7_chaintape_smoke_2026-05-01/agent_pubkeys.json
 M handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json
 M handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt
 D handover/markov_capsules/MARKOV_TB-15_2026-05-03.json
 M rules/enforcement.log
 M src/bin/generate_markov_capsule.rs
 M src/runtime/autopsy_capsule.rs
 M src/runtime/markov_capsule.rs
 M src/state/sequencer.rs
 M tests/fc_alignment_conformance.rs
 M tests/tb_15_halt_triggers.rs
?? experiments/minif2f_v4/h_vppu_history.json
?? handover/alignment/OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04.md
?? handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md
?? handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md
?? handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw
?? handover/audits/GEMINI_TB_15_SHIP_AUDIT_2026-05-04_R1.md
?? handover/audits/GEMINI_TB_15_SHIP_AUDIT_2026-05-04_R2.md
?? handover/audits/run_codex_tb_15_ship_audit.sh
?? handover/audits/run_codex_tb_15_ship_audit_r2.sh
?? handover/audits/run_codex_tb_15_ship_audit_r3.sh
?? handover/audits/run_gemini_tb_15_ship_audit.py
?? handover/audits/run_gemini_tb_15_ship_audit_r2.py
?? handover/audits/run_gemini_tb_15_ship_audit_r3.py
?? handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/cas/
?? handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/
?? handover/evidence/tb_15_markov_capsule_2026-05-04/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/
?? handover/markov_capsules/MARKOV_TB-15-R3_2026-05-03.json
?? handover/tests/

exec
/bin/bash -lc "rg -n \"write_markov_capsule|restore_markov_capsule_from_cas_bytes|write_autopsy_capsule|derive_autopsies_for_bankruptcy|BankruptcyAutopsyDerivation|write_bankruptcy_autopsies_to_cas|write_evidence_capsule\" src tests handover -g '"'!target'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/audits/run_codex_tb_15_ship_audit_r3.sh:27:- **TB15-CAS-ID VETO**: same self-CID/content-CID mismatch in `write_autopsy_capsule` and `derive_autopsies_for_bankruptcy`.
handover/audits/run_codex_tb_15_ship_audit_r3.sh:33:1. **Q3 VETO closure (write_markov_capsule)**: writer rewritten so capsule_id == sha256(stored_bytes). Specifically: build capsule with capsule_id=Cid::default()/sha256=Hash::ZERO; canonical_encode → those are the bytes stored in CAS; capsule_id = Cid::from_content(stored_bytes); populate the in-memory struct fields after CAS write. The on-CAS bytes have capsule_id+sha256 zeroed; the in-memory struct returned to caller has them populated. cas.get(&capsule.capsule_id) succeeds. NEW helper `restore_markov_capsule_from_cas_bytes(bytes)` canonical-decodes + re-derives capsule_id/sha256 from `Cid::from_content(&bytes)`.
handover/audits/run_codex_tb_15_ship_audit_r3.sh:35:2. **TB15-CAS-ID VETO closure (write_autopsy_capsule + derive_autopsies_for_bankruptcy)**: same writer pattern fix. derive_autopsies_for_bankruptcy now returns `Vec<BankruptcyAutopsyDerivation>` (struct with `capsule + private_bytes + stored_capsule_bytes` fields) instead of `Vec<(AgentAutopsyCapsule, Vec<u8>)>`. The dispatch arm reads `d.capsule.capsule_id`; apply_one writes `d.stored_capsule_bytes` (the EXACT bytes whose sha256 == capsule_id). NEW helper `restore_autopsy_capsule_from_cas_bytes(bytes)`.
handover/audits/run_codex_tb_15_ship_audit_r3.sh:38:   - `runtime::markov_capsule::tests::write_markov_capsule_cas_resolvable_by_capsule_id` — asserts `cas.get(&cap.capsule_id)` succeeds + retrieved bytes' sha256 == capsule_id + restore round-trip works.
handover/audits/run_codex_tb_15_ship_audit_r3.sh:39:   - `runtime::autopsy_capsule::tests::write_bankruptcy_autopsies_to_cas_round_trip` — extended with same R3 contract.
handover/audits/run_codex_tb_15_ship_audit_r3.sh:52:- **Q3 closure check**: read `src/runtime/markov_capsule.rs::write_markov_capsule`. Confirm:
handover/audits/run_codex_tb_15_ship_audit_r3.sh:58:  (f) `restore_markov_capsule_from_cas_bytes` correctly reverses the operation
handover/audits/run_codex_tb_15_ship_audit_r3.sh:61:- **TB15-CAS-ID closure check**: same checks for `write_autopsy_capsule` AND `derive_autopsies_for_bankruptcy` AND `write_bankruptcy_autopsies_to_cas`. Confirm `BankruptcyAutopsyDerivation` correctly carries `stored_capsule_bytes` from derive to CAS write.
handover/audits/run_codex_tb_15_ship_audit_r3.sh:63:- **Cross-cut concern**: TB-11 `write_evidence_capsule` likely has the SAME bug (predates TB-15; same prelim/final pattern). Is this a new VETO that should block TB-15 ship, or is it acceptable to OBS-defer because no production consumer of EvidenceCapsule via cap.capsule_id exists yet?
handover/audits/run_codex_tb_15_ship_audit_r3.sh:276:`derive_autopsies_for_bankruptcy` with the PRE-bankruptcy snapshot
handover/audits/run_codex_tb_15_ship_audit_r3.sh:286:`src/runtime/autopsy_capsule.rs::derive_autopsies_for_bankruptcy`
handover/audits/run_codex_tb_15_ship_audit_r3.sh:296:`src/bin/generate_markov_capsule.rs` + `src/runtime/markov_capsule.rs::write_markov_capsule`.
handover/audits/run_codex_tb_15_ship_audit_r3.sh:340:`src/runtime/autopsy_capsule.rs::write_autopsy_capsule` +
handover/audits/run_codex_tb_15_ship_audit_r3.sh:341:`derive_autopsies_for_bankruptcy` signatures. Verify: (a) NEITHER
handover/audits/run_codex_tb_15_ship_audit_r3.sh:356:write_markov_capsule unchanged; (c) for the actual emitted capsule
handover/audits/run_codex_tb_15_ship_audit_r3.sh:378:**RQ1 — derive_autopsies_for_bankruptcy idempotency under apply_one
handover/audits/run_codex_tb_15_ship_audit_r3.sh:380:`derive_autopsies_for_bankruptcy` with the same inputs. If apply_one is
handover/audits/run_codex_tb_15_ship_audit_r3.sh:389:calls `derive_autopsies_for_bankruptcy(&q.economic_state_t, bk, ...)`
handover/audits/run_codex_tb_15_ship_audit_r3.sh:425:`write_markov_capsule`. What happens if `--cas-dir` is provided but
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:220:`derive_autopsies_for_bankruptcy` with the PRE-bankruptcy snapshot
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:230:`src/runtime/autopsy_capsule.rs::derive_autopsies_for_bankruptcy`
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:240:`src/bin/generate_markov_capsule.rs` + `src/runtime/markov_capsule.rs::write_markov_capsule`.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:284:`src/runtime/autopsy_capsule.rs::write_autopsy_capsule` +
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:285:`derive_autopsies_for_bankruptcy` signatures. Verify: (a) NEITHER
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:300:write_markov_capsule unchanged; (c) for the actual emitted capsule
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:322:**RQ1 — derive_autopsies_for_bankruptcy idempotency under apply_one
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:324:`derive_autopsies_for_bankruptcy` with the same inputs. If apply_one is
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:333:calls `derive_autopsies_for_bankruptcy(&q.economic_state_t, bk, ...)`
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:369:`write_markov_capsule`. What happens if `--cas-dir` is provided but
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:635:/bin/bash -lc 'rg -n "TB15_AUTOPSY_ACTIVATION_LOGICAL_T|is_autopsy_active_at|derive_autopsies_for_bankruptcy|cluster_autopsies|private_detail|TypicalErrorSummary" src/runtime/autopsy_capsule.rs' in /home/zephryj/projects/turingosv4
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:668:460:// TB-15 Atom 3 — `derive_autopsies_for_bankruptcy` (PURE deterministic helper)
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:670:488:pub fn derive_autopsies_for_bankruptcy(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:676:551:/// `derive_autopsies_for_bankruptcy` (same inputs → same Cids as the
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:677:564:    let derived = derive_autopsies_for_bankruptcy(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:691:748:    // Atom 3 — derive_autopsies_for_bankruptcy tests
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:692:760:        // not strictly needed by derive_autopsies_for_bankruptcy itself.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:693:805:    /// TB-15 Atom 3 — derive_autopsies_for_bankruptcy: per-staker
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:694:816:        let derived = derive_autopsies_for_bankruptcy(&econ, &bk, /*round=*/ 5, /*t=*/ 100);
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:695:843:    /// TB-15 Atom 3 — derive_autopsies_for_bankruptcy: same inputs →
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:696:855:        let a = derive_autopsies_for_bankruptcy(&econ, &bk, 3, 50);
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:697:856:        let b = derive_autopsies_for_bankruptcy(&econ, &bk, 3, 50);
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:699:866:    /// TB-15 Atom 3 — derive_autopsies_for_bankruptcy: no stakers on
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:700:880:        let derived = derive_autopsies_for_bankruptcy(&econ, &bk, 0, 0);
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:725:1042:        let derived = derive_autopsies_for_bankruptcy(&econ, &bk, 7, 42);
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:730:/bin/bash -lc "rg -n \"is_autopsy_active_at|TaskBankruptcyTx|Stage 3\\.5|Step 3\\.5|derive_autopsies_for_bankruptcy|q_snapshot|q_next\" src/state/sequencer.rs" in /home/zephryj/projects/turingosv4
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:794:1367:            // using the same `derive_autopsies_for_bankruptcy` helper
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:797:1376:                    crate::runtime::autopsy_capsule::derive_autopsies_for_bankruptcy(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:929:/bin/bash -lc 'rg -n "flowchart_hashes|read_flowchart_hashes_from_matrix|MarkovEvidenceCapsule|try_deep_history_read_with_override_check|override_set_from_env|include-prior-capsules|write_markov_capsule|sha256_of_file|LATEST_MARKOV|boot_seq" src/runtime/markov_capsule.rs src/bin/generate_markov_capsule.rs' in /home/zephryj/projects/turingosv4
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:933:src/bin/generate_markov_capsule.rs:43:    try_deep_history_read_with_override_check, write_markov_capsule, MarkovGenError, ObsId,
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:951:src/bin/generate_markov_capsule.rs:352:        write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:953:src/bin/generate_markov_capsule.rs:370:            other => format!("write_markov_capsule: {other}"),
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:957:src/runtime/markov_capsule.rs:10://! Generator surface: `write_markov_capsule(...)` (CAS-emitting) + binary
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:968:src/runtime/markov_capsule.rs:227:pub fn write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:985:src/runtime/markov_capsule.rs:444:    /// TB-15 Atom 5 — write_markov_capsule writes 2 CAS objects
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:986:src/runtime/markov_capsule.rs:448:    fn write_markov_capsule_to_cas_round_trip() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:987:src/runtime/markov_capsule.rs:461:        let cap = write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:989:src/runtime/markov_capsule.rs:491:    /// TB-15 Atom 5 — write_markov_capsule deterministic (same inputs →
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:990:src/runtime/markov_capsule.rs:494:    fn write_markov_capsule_deterministic_capsule_id() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:991:src/runtime/markov_capsule.rs:501:            write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:992:src/runtime/markov_capsule.rs:533:        let cap_1 = write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:993:src/runtime/markov_capsule.rs:548:        let cap_2 = write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:1189:   460	// TB-15 Atom 3 — `derive_autopsies_for_bankruptcy` (PURE deterministic helper)
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:1217:   488	pub fn derive_autopsies_for_bankruptcy(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:1284:   551	/// `derive_autopsies_for_bankruptcy` (same inputs → same Cids as the
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:1289:   556	pub fn write_bankruptcy_autopsies_to_cas(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:1297:   564	    let derived = derive_autopsies_for_bankruptcy(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:1347:  1367	            // using the same `derive_autopsies_for_bankruptcy` helper
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:1356:  1376	                    crate::runtime::autopsy_capsule::derive_autopsies_for_bankruptcy(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:1467:  3099	                let _ = crate::runtime::autopsy_capsule::write_bankruptcy_autopsies_to_cas(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:1714:   227	pub fn write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:1921:   444	    /// TB-15 Atom 5 — write_markov_capsule writes 2 CAS objects
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:1925:   448	    fn write_markov_capsule_to_cas_round_trip() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:1938:   461	        let cap = write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:1968:   491	    /// TB-15 Atom 5 — write_markov_capsule deterministic (same inputs →
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:1971:   494	    fn write_markov_capsule_deterministic_capsule_id() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:1978:   501	            write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:2010:   533	        let cap_1 = write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:2025:   548	        let cap_2 = write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:2164:    43	    try_deep_history_read_with_override_check, write_markov_capsule, MarkovGenError, ObsId,
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:2473:   352	        write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:2491:   370	            other => format!("write_markov_capsule: {other}"),
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:3234:tests/tb_15_halt_triggers.rs:137:// write_autopsy_capsule signature MUST NOT accept any &mut PredicateRegistry
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:3335:   253	pub fn write_autopsy_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:3766:    80	Re-running the binary with identical inputs yields the same `capsule_id`. Verified by `write_markov_capsule_deterministic_capsule_id` unit test (extended in R2 to include flowchart_hashes in determinism check).
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4321:/bin/bash -lc "rg -n \"struct EvidenceCapsule|write_evidence_capsule|capsule_id|canonical_encode\\(&capsule|ObjectType::EvidenceCapsule|write.*to_cas_round_trip\" src/runtime src -S" in /home/zephryj/projects/turingosv4
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4329:src/runtime/adapter.rs:467:/// via `evidence_capsule::write_evidence_capsule`).
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4330:src/runtime/adapter.rs:474:///   1. Build EvidenceCapsule via `evidence_capsule::write_evidence_capsule`.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4347:src/runtime/markov_capsule.rs:448:    fn write_markov_capsule_to_cas_round_trip() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4351:src/runtime/markov_capsule.rs:494:    fn write_markov_capsule_deterministic_capsule_id() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4367:src/runtime/evidence_capsule.rs:208:pub fn write_evidence_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4377:src/runtime/evidence_capsule.rs:337:    fn write_evidence_capsule_to_cas_round_trip() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4378:src/runtime/evidence_capsule.rs:355:        let capsule = write_evidence_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4383:src/runtime/evidence_capsule.rs:400:    fn write_evidence_capsule_deterministic_capsule_id() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4384:src/runtime/evidence_capsule.rs:418:            write_evidence_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4385:src/runtime/evidence_capsule.rs:438:            write_evidence_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4413:src/runtime/autopsy_capsule.rs:641:    fn write_autopsy_capsule_to_cas_round_trip() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4417:src/runtime/autopsy_capsule.rs:695:    fn write_autopsy_capsule_deterministic_capsule_id() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4424:src/runtime/autopsy_capsule.rs:1013:    fn write_bankruptcy_autopsies_to_cas_round_trip() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4426:src/runtime/adapter.rs:467:/// via `evidence_capsule::write_evidence_capsule`).
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4427:src/runtime/adapter.rs:474:///   1. Build EvidenceCapsule via `evidence_capsule::write_evidence_capsule`.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4444:src/runtime/markov_capsule.rs:448:    fn write_markov_capsule_to_cas_round_trip() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4448:src/runtime/markov_capsule.rs:494:    fn write_markov_capsule_deterministic_capsule_id() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4464:src/runtime/evidence_capsule.rs:208:pub fn write_evidence_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4474:src/runtime/evidence_capsule.rs:337:    fn write_evidence_capsule_to_cas_round_trip() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4475:src/runtime/evidence_capsule.rs:355:        let capsule = write_evidence_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4480:src/runtime/evidence_capsule.rs:400:    fn write_evidence_capsule_deterministic_capsule_id() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4481:src/runtime/evidence_capsule.rs:418:            write_evidence_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4482:src/runtime/evidence_capsule.rs:438:            write_evidence_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4510:src/runtime/autopsy_capsule.rs:641:    fn write_autopsy_capsule_to_cas_round_trip() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4514:src/runtime/autopsy_capsule.rs:695:    fn write_autopsy_capsule_deterministic_capsule_id() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4521:src/runtime/autopsy_capsule.rs:1013:    fn write_bankruptcy_autopsies_to_cas_round_trip() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4734:   208	pub fn write_evidence_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4769:src/runtime/adapter.rs:474:///   1. Build EvidenceCapsule via `evidence_capsule::write_evidence_capsule`.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4891:   337	    fn write_evidence_capsule_to_cas_round_trip() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:4909:   355	        let capsule = write_evidence_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:5347:handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md.raw:240:`src/bin/generate_markov_capsule.rs` + `src/runtime/markov_capsule.rs::write_markov_capsule`.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:5351:handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md.raw:929:/bin/bash -lc 'rg -n "flowchart_hashes|read_flowchart_hashes_from_matrix|MarkovEvidenceCapsule|try_deep_history_read_with_override_check|override_set_from_env|include-prior-capsules|write_markov_capsule|sha256_of_file|LATEST_MARKOV|boot_seq" src/runtime/markov_capsule.rs src/bin/generate_markov_capsule.rs' in /home/zephryj/projects/turingosv4
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:5354:handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md.raw:933:src/bin/generate_markov_capsule.rs:43:    try_deep_history_read_with_override_check, write_markov_capsule, MarkovGenError, ObsId,
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:5372:handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md.raw:951:src/bin/generate_markov_capsule.rs:352:        write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:5374:handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md.raw:953:src/bin/generate_markov_capsule.rs:370:            other => format!("write_markov_capsule: {other}"),
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:5462:handover/evidence/tb_15_markov_capsule_2026-05-03/README.md:70:| SG-15.1 | Failed/losing agent gets private AutopsyCapsule | GREEN — TaskBankruptcyTx dispatch arm Step 3.5 emits per-staker capsule via `derive_autopsies_for_bankruptcy` (verified by `derive_autopsies_emits_one_per_staker_target_only`) |
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:5555:  1042	        let derived = derive_autopsies_for_bankruptcy(&econ, &bk, 7, 42);
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:5653:- Q3 VETO: the R2 `LATEST_MARKOV_CAPSULE.txt` pointer is not CAS-resolvable. `CasStore::put` keys objects by sha256 of the stored bytes, and `get` requires that key: [store.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs:177). But `write_markov_capsule` computes `capsule_id` from prelim bytes, then stores different final bytes and ignores the returned CAS cid: [markov_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/markov_capsule.rs:286), [markov_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/markov_capsule.rs:293). The pointer file publishes `a94ae884...`, while the CAS index stores the Markov object under `e4932fca...`: [LATEST_MARKOV_CAPSULE.txt](/home/zephryj/projects/turingosv4/handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt:1), [.turingos_cas_index.jsonl](/tmp/tb15-r2-cas/.turingos_cas_index.jsonl:2). This breaks SG-15.3 bootstrap-by-latest-capsule and Markov chain resolvability.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md:5671:- Q3 VETO: the R2 `LATEST_MARKOV_CAPSULE.txt` pointer is not CAS-resolvable. `CasStore::put` keys objects by sha256 of the stored bytes, and `get` requires that key: [store.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs:177). But `write_markov_capsule` computes `capsule_id` from prelim bytes, then stores different final bytes and ignores the returned CAS cid: [markov_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/markov_capsule.rs:286), [markov_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/markov_capsule.rs:293). The pointer file publishes `a94ae884...`, while the CAS index stores the Markov object under `e4932fca...`: [LATEST_MARKOV_CAPSULE.txt](/home/zephryj/projects/turingosv4/handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt:1), [.turingos_cas_index.jsonl](/tmp/tb15-r2-cas/.turingos_cas_index.jsonl:2). This breaks SG-15.3 bootstrap-by-latest-capsule and Markov chain resolvability.
handover/audits/GEMINI_TB_15_SHIP_AUDIT_2026-05-04_R1.md:29:**Finding**: The architectural separation is tight. The `suggested_policy_patch: Option<Cid>` field in `AgentAutopsyCapsule` (`src/runtime/autopsy_capsule.rs:163`) is treated as an opaque pointer. The writer function `write_autopsy_capsule` (`src/runtime/autopsy_capsule.rs:293`) accepts this field but has no logic to read or apply it. Crucially, the halt-trigger test `autopsy_does_not_mutate_predicates` (`tests/tb_15_halt_triggers.rs:90-120`) performs a file-scan of `src/runtime/autopsy_capsule.rs` to ensure no mutable references to registries (`&mut PredicateRegistry`, etc.) or mutator methods (`.register_predicate(`) exist. This structurally prevents the autopsy module from becoming a backdoor for automatic policy changes. The field is strictly a write-only suggestion awaiting a future, separate MetaTape consumer as intended by the spec.
handover/audits/RECURSIVE_AUDIT_TB_11_2026-05-02.md:19:| Art. 0.2 Tape Canonical              | EvidenceCapsule canonical-encoded; capsule_id = sha256 of canonical bytes (content-addressed self-reference) | `evidence_capsule::tests::write_evidence_capsule_to_cas_round_trip` asserts `capsule.capsule_id.0 == capsule.sha256.0` |
handover/audits/RECURSIVE_AUDIT_TB_11_2026-05-02.md:34:| EvidenceCapsule canonical bytes deterministic across replays | `write_evidence_capsule_deterministic_capsule_id` test (Atom 3 U) — same inputs across two distinct CAS instances yield identical capsule_id + compressed_log_cid + manifest_cid |
handover/audits/RECURSIVE_AUDIT_TB_11_2026-05-02.md:106:| Replay non-determinism (capsule_id drift)              | `write_evidence_capsule_deterministic_capsule_id` test locks bit-for-bit Cid stability                     |
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R4.md:2973:handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R3.md:2385:handover/tracer_bullets/TB_LOG.tsv:39:# 2026-05-03 TB-12 SHIPPED — "P3 carry-forward + P4 primary: Node Exposure Index per architect 2026-05-03 ruling §3 + §10". Class 3 envelope (Atom 1 = Class 2 schema; Atom 2 = Class 3 dispatch wire; Atom 6 = Class 3 dual audit Codex + Gemini both ✓). Self-audit PASS; Codex CHALLENGE × 2 (Q4 doc-drift on 4-vs-5-holding total_supply; Q5 legacy CPMM in src/prediction_market.rs predates TB-12) — both resolved as documentation/scope clarifications via recursive audit §10 + OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md (TB-13 prerequisite); Gemini round-1 PASS / high conviction / PROCEED to SHIP (gemini-2.5-pro, 896k char prompt, 48.2s API; 8/8 audit questions PASS). User-architect ultrathink verified architect §9 strict alignment before SHIP — found 4 SG-12.x test-name drifts (SG-12.5 / 12.6 / 12.7 / 12.8 names didn't exactly match architect's "passes" strings); all 4 fixed BEFORE SHIP per feedback_no_retroactive_evidence_rewrite (NOT post-ship patch): SG-12.5 renamed to sg_12_5_replay_reconstructs_node_positions; SG-12.6 ADDED sg_12_6_dashboard_view_positions_works inside src/bin/audit_dashboard.rs #[cfg(test)] mod tb12_render_tests with 4 cases + forbidden-token grep; SG-12.7 renamed to sg_12_7_no_market_trading_variants_introduced; SG-12.8 ADDED sg_12_8_no_node_market_entry_as_canonical_state at integration-test level (q_state.rs unit alias kept as defense-in-depth). 8/8 SG-12.x by architect §9.3 EXACT names PASS. cargo test --workspace = 759/0/150 (+12 net vs TB-11 ship 747; +28 vs TB-10 ship 731). Architectural deltas: NEW src/state/typed_tx.rs (NodePosition struct + PositionSide enum + PositionKind enum); EXTEND src/state/q_state.rs (NodePositionsIndex flat shape + EconomicState 10→11 sub-fields with +node_positions_t serde-default); EXTEND src/state/sequencer.rs (WorkTx + ChallengeTx accept-arm pure-additive NodePosition write — no balances_t / stakes_t / challenge_cases_t / total_supply mutation; CR-12.1 + CR-12.2 verified); EXTEND src/economy/monetary_invariant.rs (NodePosition NOT in 4-holding total_supply_micro; structural — assert_total_ctf_conserved unchanged); EXTEND src/bin/audit_dashboard.rs (§13 Node exposure records + render_section_13 pure-fn helper for SG-12.6 unit-testability + LABEL DISCIPLINE "exposure records" NOT "Open market balances" per architect §8 Atom 4); EXTEND experiments/minif2f_v4/src/bin/evaluator.rs (TB-11 G3/G4 carry-forward — 4 EvidenceCapsule counters + write_evidence_capsule + tb11_emit_terminal_summary_for_run wired at MAX_TX exhausted exit point); EXTEND experiments/minif2f_v4/src/bin/lean_market.rs (3 new subcommands: tick POLICY PREVIEW MODE — actual emission deferred to system_keypair persistence + view-bankruptcy read-only listing + view-positions read-only NodePosition browser); REHASH genesis_payload.toml trust_root for src/state/sequencer.rs + src/state/typed_tx.rs + src/state/q_state.rs + src/state/mod.rs + src/bin/audit_dashboard.rs + experiments/minif2f_v4/src/bin/evaluator.rs (lean_market.rs outside trust root by TB-10 precedent). Halting triggers (architect §7) NONE triggered: CTF conservation passes (Gemini Q1+Q4); position-tx field derivation exact (position_fields_derived_from_source_tx_exactly test PASS); NodePosition NOT counted as Coin (4-holding model unchanged); replay deterministic (sg_12_5 PASS); no Codex/Gemini VETO. Production claim: "TB-12 establishes a flat, immutable, non-monetary NodePositionsIndex anchoring WorkTx.stake → FirstLong + ChallengeTx.stake → ChallengeShort exposure records on canonical L4-derived QState. NodePosition is exposure record NOT active position balance per architect §10 critical insight; TB-13 CompleteSet introduces real YES/NO claims; TB-14 PriceIndex computes price from long/short interest; TB-16 controlled-arena introduces P&L. Charter Q6 (ii.5) sync mode honored — dual audit complete + user-architect ultrathink verdict before SHIP." Open follow-ups (carry-forward, NOT ship blockers): TB-13 prerequisite quarantine of src/prediction_market.rs legacy f64 CPMM (OBS-tracked); system_keypair persistence + reattachment factory for lean_market tick on-chain emission; real-LLM zeta-regularization rerun (Atom 0.5(a) wired the call site; manual user-driven session post-ship).
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R4.md:3581:handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R3.md:10786:handover/tracer_bullets/TB_LOG.tsv:39:# 2026-05-03 TB-12 SHIPPED — "P3 carry-forward + P4 primary: Node Exposure Index per architect 2026-05-03 ruling §3 + §10". Class 3 envelope (Atom 1 = Class 2 schema; Atom 2 = Class 3 dispatch wire; Atom 6 = Class 3 dual audit Codex + Gemini both ✓). Self-audit PASS; Codex CHALLENGE × 2 (Q4 doc-drift on 4-vs-5-holding total_supply; Q5 legacy CPMM in src/prediction_market.rs predates TB-12) — both resolved as documentation/scope clarifications via recursive audit §10 + OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md (TB-13 prerequisite); Gemini round-1 PASS / high conviction / PROCEED to SHIP (gemini-2.5-pro, 896k char prompt, 48.2s API; 8/8 audit questions PASS). User-architect ultrathink verified architect §9 strict alignment before SHIP — found 4 SG-12.x test-name drifts (SG-12.5 / 12.6 / 12.7 / 12.8 names didn't exactly match architect's "passes" strings); all 4 fixed BEFORE SHIP per feedback_no_retroactive_evidence_rewrite (NOT post-ship patch): SG-12.5 renamed to sg_12_5_replay_reconstructs_node_positions; SG-12.6 ADDED sg_12_6_dashboard_view_positions_works inside src/bin/audit_dashboard.rs #[cfg(test)] mod tb12_render_tests with 4 cases + forbidden-token grep; SG-12.7 renamed to sg_12_7_no_market_trading_variants_introduced; SG-12.8 ADDED sg_12_8_no_node_market_entry_as_canonical_state at integration-test level (q_state.rs unit alias kept as defense-in-depth). 8/8 SG-12.x by architect §9.3 EXACT names PASS. cargo test --workspace = 759/0/150 (+12 net vs TB-11 ship 747; +28 vs TB-10 ship 731). Architectural deltas: NEW src/state/typed_tx.rs (NodePosition struct + PositionSide enum + PositionKind enum); EXTEND src/state/q_state.rs (NodePositionsIndex flat shape + EconomicState 10→11 sub-fields with +node_positions_t serde-default); EXTEND src/state/sequencer.rs (WorkTx + ChallengeTx accept-arm pure-additive NodePosition write — no balances_t / stakes_t / challenge_cases_t / total_supply mutation; CR-12.1 + CR-12.2 verified); EXTEND src/economy/monetary_invariant.rs (NodePosition NOT in 4-holding total_supply_micro; structural — assert_total_ctf_conserved unchanged); EXTEND src/bin/audit_dashboard.rs (§13 Node exposure records + render_section_13 pure-fn helper for SG-12.6 unit-testability + LABEL DISCIPLINE "exposure records" NOT "Open market balances" per architect §8 Atom 4); EXTEND experiments/minif2f_v4/src/bin/evaluator.rs (TB-11 G3/G4 carry-forward — 4 EvidenceCapsule counters + write_evidence_capsule + tb11_emit_terminal_summary_for_run wired at MAX_TX exhausted exit point); EXTEND experiments/minif2f_v4/src/bin/lean_market.rs (3 new subcommands: tick POLICY PREVIEW MODE — actual emission deferred to system_keypair persistence + view-bankruptcy read-only listing + view-positions read-only NodePosition browser); REHASH genesis_payload.toml trust_root for src/state/sequencer.rs + src/state/typed_tx.rs + src/state/q_state.rs + src/state/mod.rs + src/bin/audit_dashboard.rs + experiments/minif2f_v4/src/bin/evaluator.rs (lean_market.rs outside trust root by TB-10 precedent). Halting triggers (architect §7) NONE triggered: CTF conservation passes (Gemini Q1+Q4); position-tx field derivation exact (position_fields_derived_from_source_tx_exactly test PASS); NodePosition NOT counted as Coin (4-holding model unchanged); replay deterministic (sg_12_5 PASS); no Codex/Gemini VETO. Production claim: "TB-12 establishes a flat, immutable, non-monetary NodePositionsIndex anchoring WorkTx.stake → FirstLong + ChallengeTx.stake → ChallengeShort exposure records on canonical L4-derived QState. NodePosition is exposure record NOT active position balance per architect §10 critical insight; TB-13 CompleteSet introduces real YES/NO claims; TB-14 PriceIndex computes price from long/short interest; TB-16 controlled-arena introduces P&L. Charter Q6 (ii.5) sync mode honored — dual audit complete + user-architect ultrathink verdict before SHIP." Open follow-ups (carry-forward, NOT ship blockers): TB-13 prerequisite quarantine of src/prediction_market.rs legacy f64 CPMM (OBS-tracked); system_keypair persistence + reattachment factory for lean_market tick on-chain emission; real-LLM zeta-regularization rerun (Atom 0.5(a) wired the call site; manual user-driven session post-ship).
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R4.md:14993:rules/enforcement.log:1499:2026-05-02T19:10:09Z R-022-PASS commit=b6201b777b2e6c57e9ee6c12295b815d0edc5ebf file=src/runtime/evidence_capsule.rs line=208 symbol=pub_fn_write_evidence_capsule reason="doc-block backlink found"
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:156:`derive_autopsies_for_bankruptcy` with the PRE-bankruptcy snapshot
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:166:`src/runtime/autopsy_capsule.rs::derive_autopsies_for_bankruptcy`
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:176:`src/bin/generate_markov_capsule.rs` + `src/runtime/markov_capsule.rs::write_markov_capsule`.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:220:`src/runtime/autopsy_capsule.rs::write_autopsy_capsule` +
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:221:`derive_autopsies_for_bankruptcy` signatures. Verify: (a) NEITHER
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:236:write_markov_capsule unchanged; (c) for the actual emitted capsule
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:258:**RQ1 — derive_autopsies_for_bankruptcy idempotency under apply_one
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:260:`derive_autopsies_for_bankruptcy` with the same inputs. If apply_one is
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:269:calls `derive_autopsies_for_bankruptcy(&q.economic_state_t, bk, ...)`
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:305:`write_markov_capsule`. What happens if `--cas-dir` is provided but
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:486:  1364	            // the bytes using the same `derive_autopsies_for_bankruptcy`
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:490:  1368	                crate::runtime::autopsy_capsule::derive_autopsies_for_bankruptcy(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:601:  3084	            let _ = crate::runtime::autopsy_capsule::write_bankruptcy_autopsies_to_cas(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:858:   437	pub fn derive_autopsies_for_bankruptcy(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:921:   500	/// `derive_autopsies_for_bankruptcy` (same inputs → same Cids as the
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:926:   505	pub fn write_bankruptcy_autopsies_to_cas(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:934:   513	    let derived = derive_autopsies_for_bankruptcy(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:1011:   590	    fn write_autopsy_capsule_to_cas_round_trip() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:1049:   253	pub fn write_autopsy_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:1205:   409	// TB-15 Atom 3 — `derive_autopsies_for_bankruptcy` (PURE deterministic helper)
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:1240:    10	//! Generator surface: `write_markov_capsule(...)` (CAS-emitting) + binary
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:1444:   214	pub fn write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:1600:   365	    /// TB-15 Atom 5 — write_markov_capsule writes 2 CAS objects
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:1604:   369	    fn write_markov_capsule_to_cas_round_trip() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:1617:   382	        let cap = write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:1645:   410	    /// TB-15 Atom 5 — write_markov_capsule deterministic (same inputs →
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:1648:   413	    fn write_markov_capsule_deterministic_capsule_id() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:1655:   420	            write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:1686:   451	        let cap_1 = write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:1700:   465	        let cap_2 = write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:1800:    42	    override_set_from_env, scan_unresolved_obs, sha256_of_file, write_markov_capsule, MarkovGenError,
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:2027:   269	        write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:2044:   286	            other => format!("write_markov_capsule: {other}"),
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:3159:   110	// write_autopsy_capsule signature MUST NOT accept any &mut PredicateRegistry
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:3808:handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:83:- `src/runtime/markov_capsule.rs` (Atom 5): `ObsId` newtype + `MarkovEvidenceCapsule` struct + `with_constitution_hash` constructor + `try_deep_history_read_with_override_check` gate + `override_set_from_env` + `write_markov_capsule` + `scan_unresolved_obs` + `sha256_of_file` + `MarkovGenError`. 8 in-module tests.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:3814:handover/tracer_bullets/TB-15_charter_2026-05-03.md:14:- **Flowchart 1 (runtime)**: `FC1-N32` = `write_autopsy_capsule` writer (CAS-resident; sequencer-emitted; private-by-default). `FC1-N33` = TaskBankruptcyTx dispatch arm autopsy emission (per-agent loss anchor for FR-15.1 first wire-in).
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:3820:handover/tracer_bullets/TB-15_charter_2026-05-03.md:74:    FC1-N32 = write_autopsy_capsule (CAS writer; AuditOnly default;
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:4570:    70	| SG-15.1 | Failed/losing agent gets private AutopsyCapsule | GREEN — TaskBankruptcyTx dispatch arm Step 3.5 emits per-staker capsule via `derive_autopsies_for_bankruptcy` (verified by `derive_autopsies_emits_one_per_staker_target_only`) |
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:4593:    93	Re-running the binary with identical inputs yields the same `capsule_id`. Verified by `write_markov_capsule_deterministic_capsule_id` unit test.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:4822:src/bin/generate_markov_capsule.rs:42:    override_set_from_env, scan_unresolved_obs, sha256_of_file, write_markov_capsule, MarkovGenError,
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:4871:handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:83:- `src/runtime/markov_capsule.rs` (Atom 5): `ObsId` newtype + `MarkovEvidenceCapsule` struct + `with_constitution_hash` constructor + `try_deep_history_read_with_override_check` gate + `override_set_from_env` + `write_markov_capsule` + `scan_unresolved_obs` + `sha256_of_file` + `MarkovGenError`. 8 in-module tests.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:4904:handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:83:- `src/runtime/markov_capsule.rs` (Atom 5): `ObsId` newtype + `MarkovEvidenceCapsule` struct + `with_constitution_hash` constructor + `try_deep_history_read_with_override_check` gate + `override_set_from_env` + `write_markov_capsule` + `scan_unresolved_obs` + `sha256_of_file` + `MarkovGenError`. 8 in-module tests.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:4933:src/bin/generate_markov_capsule.rs:42:    override_set_from_env, scan_unresolved_obs, sha256_of_file, write_markov_capsule, MarkovGenError,
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5027:src/runtime/markov_capsule.rs:369:    fn write_markov_capsule_to_cas_round_trip() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5112:tests/tb_15_halt_triggers.rs:110:// write_autopsy_capsule signature MUST NOT accept any &mut PredicateRegistry
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5233:handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:82:- `src/runtime/autopsy_capsule.rs` (Atoms 2 + 3 + 4): `LossReasonClass` enum (8 variants) + `AgentAutopsyCapsule` struct + `format_public_summary` + `write_autopsy_capsule` + `derive_autopsies_for_bankruptcy` (PURE) + `write_bankruptcy_autopsies_to_cas` + `cluster_autopsies` + `TypicalErrorSummary` + `AutopsyWriteError`. 15 in-module tests.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5234:handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:83:- `src/runtime/markov_capsule.rs` (Atom 5): `ObsId` newtype + `MarkovEvidenceCapsule` struct + `with_constitution_hash` constructor + `try_deep_history_read_with_override_check` gate + `override_set_from_env` + `write_markov_capsule` + `scan_unresolved_obs` + `sha256_of_file` + `MarkovGenError`. 8 in-module tests.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5263:genesis_payload.toml:142:"tests/fc_alignment_conformance.rs" = "5e257f275d86032a957101b7898293fa9d6cdfc13a4bdd2a5f630fb9e940b408"  # rehashed by TB-15 Atom 6 (2026-05-03; architect §6.2 ruling): +4 TB-15 witnesses (FC1-N32 write_autopsy_capsule + FC1-N33 derive_autopsies_for_bankruptcy + FC2-N30 cluster_autopsies + FC3-N43 MarkovEvidenceCapsule + try_deep_history_read_with_override_check default-deny gate). Predecessor TB-14 Atom 6 B′ step 4 hash 751c78c8 superseded.  # rehashed by TB-14 Atom 6 B′ step 4 (2026-05-03; architect ruling §3+§4 — canonical-graph rewire): fc2_n28_mask_set_publication_witness updated to build CanonicalNodeGraph (BTreeMap<TxId, BTreeSet<TxId>>) directly instead of shadow Tape; compute_mask_set new signature (econ, edges, policy, price_index). Predecessor B′ step 1+2 hash c1f14584 superseded.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5266:genesis_payload.toml:164:"experiments/minif2f_v4/src/bin/evaluator.rs" = "3530a2d5ac98ed8db0d037bc45fda03594c11596850ecb6be71855c6dee3abf0"  # rehashed by TB-14 Atom 6 follow-up (2026-05-03; architect ruling step 1 — Codex R1 primary VETO closure): the canonical TxId returned by `boltzmann_select_parent_v2` MUST NOT flow into legacy shadow `bus.append` parent_id (kernel.tape uses a different (shadow) id namespace; canonical TxId becomes a dangling citation → kernel.append rejects). Per architect ruling: "Use None unless a real shadow id exists." No canonical → shadow id mapping is currently available; pass None (legacy default). The v2 selector still runs (its result captured as `_v2_canonical_pick` for observability + future canonical wire-up — see B′ step 4 CanonicalNodeGraph + parent_tx replacement for last_tx_by_agent), but its output is explicitly NOT passed to bus.append. Closes Codex R1 primary VETO defect #1.  # rehashed by TB-14 Atom 6 (2026-05-03; closing OBS_TB_12_LEGACY_CPMM_QUARANTINE): production wire-swap. (1) Imports updated: `BoltzmannParams + boltzmann_select_parent` (legacy f64) → `boltzmann_select_parent_v2 + BoltzmannMaskPolicy` (integer-rational). (2) `BusConfig` literals (×2) drop `system_lp_amount` field. (3) `let params = BoltzmannParams::from_env()` → `let policy = BoltzmannMaskPolicy::from_env()`. (4) Tick-time logging derives `market_count` + top-N price ticker from `bus.snapshot().price_index` (cross-multiplication argmax; rendered `n/d`, never decimal). (5) Per-tx prompt build derives `market_ticker_str` from `snap.price_index` + queries `prompt_balance` from `bus.sequencer.q_snapshot().economic_state_t.balances_t` (replaces legacy `snap.get_balance`; the `f64` is the `prompt.rs` render contract — `prompt.rs` is not a TB-14 module surface). (6) Boltzmann selector call replaced: `boltzmann_select_parent(&snap.tape, &prices, &params, &mut rng)` → `boltzmann_select_parent_v2(&snap.price_index, &snap.mask_set, &policy, &mut rng).map(|tx| tx.0)`. Predicate-blind (CR-14.1 + halt-trigger #1) — purely a scheduling priority pick. Predecessor TB-12 Atom 0.5(a) hash 8f1b97f1 superseded. (2026-05-03; architect 2026-05-03 ruling §1.1 + §8 Atom 0.5; TB-11 G3/G4 carry-forward closure): added 4 EvidenceCapsule counters (tb11_lean_error_count / tb11_sorry_block_count / tb11_protocol_parse_failure_count / tb11_partial_accept_count) instrumented at existing classify_lean_error / classify_parse_error / step_partial_ok call sites; sorry-block vs lean-error split via err_detail.contains("sorry") || err_detail.contains("forbidden_payload"); MAX_TX exhausted exit point (inside `if let Some(bundle) = chaintape_bundle`, BEFORE bundle.shutdown) now writes EvidenceCapsule to CAS via runtime::evidence_capsule::write_evidence_capsule + emits TerminalSummary on-chain via runtime::adapter::tb11_emit_terminal_summary_for_run with capsule_cid; bundle.shutdown drains the queued TerminalSummary tx via apply_one. Mirror of TB-8 tb8_emit_finalize_after_verify pattern. Predecessor TB-10 Atom 1+3 hash a5b30cd1 superseded.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5273:genesis_payload.toml:235:"src/runtime/evidence_capsule.rs" = "5266ff43f53cd1a54d3cdbb87e40b886ea4439e109f27514711846f7c84b9e03"  # rehashed by TB-11 Atom 3 (2026-05-02; architect §6.1 ruling): EvidenceCapsule writer added — write_evidence_capsule fn + CapsuleWriteError + 2 round-trip tests. TB-11 MVP stores raw log uncompressed (gzip wrapping deferred to TB-15 Markov Loom). Predecessor Atom 1 hash 7008edce superseded.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5282:genesis_payload.toml:279:"src/state/sequencer.rs" = "9fa5936230738e5f72b14713a96238ad53583c08cdd6207d8832d27b009f50a9"  # rehashed by TB-15 Atom 3 (2026-05-03; architect §6.2 ruling): TaskBankruptcyTx dispatch arm Step 3.5 — pure derive_autopsies_for_bankruptcy(&q.economic_state_t, bk, q.q_t.current_round, bk.timestamp_logical) populates q_next.economic_state_t.agent_autopsies_t[event_id] with deterministic Cids per losing-staker. apply_one Stage 3.5 — write_bankruptcy_autopsies_to_cas() side-effect writes the capsule + private_detail bytes to CAS using the same deterministic helper (replay-safe identical Cids). NO change to monetary invariants (assert_no_post_init_mint + assert_total_ctf_conserved unchanged). NO predicate registry mutation (CR-15.3 + halt-trigger #3). NOT projected to AgentVisibleProjection (CR-15.1 + halt-trigger #1). Predecessor TB-14 Atom 6 B′ step 4 hash 1c6ba82f superseded.  # rehashed by TB-14 Atom 6 B′ step 4 (2026-05-03; architect ruling §3+§4 — canonical-graph builder): NEW `Sequencer::compute_canonical_edges_at_head() -> BTreeMap<TxId, BTreeSet<TxId>>` walks L4 + reads CAS-resident ProposalTelemetry.parent_tx for each accepted WorkTx → builds canonical parent → children edge map. Replay-deterministic per Art.0.2 (L4 + CAS payloads both replay-deterministic). Empty fallback on lock poisoned / CAS missing / decode error / no parent_tx. **Halt-trigger #2 fence preserved**: the new method body uses `crate::bottom_white::ledger::transition_ledger::canonical_decode` + `crate::runtime::proposal_telemetry::read_from_cas` (both NON-TB-14 modules — TB-3 + TB-7 surface respectively). NO TB-14 imports added to sequencer.rs `use` block. The forbidden tokens (compute_price_index / compute_mask_set / NodeMarketEntry / RationalPrice / BoltzmannMaskPolicy) remain absent from sequencer.rs. Predecessor TB-13 Atom 6 round-5 hash 2b8a1b36 superseded. (2026-05-03; Codex RQ5 remediation): drop redundant inner-consistency check `redeem.outcome != redeem.resolution_ref.claimed_outcome` from CompleteSetRedeem dispatch — ResolutionRef wrapper removed in typed_tx.rs (claimed_outcome was a redundant copy of redeem.outcome; resolution_tx_id was opaque/L4-unvalidated dead wire weight). Step 1 deleted; remaining steps renumbered 1..6. State-mismatch path (Finalized+No / Bankrupt+Yes → InvalidResolutionRef) preserved via existing match arm using redeem.outcome directly. Predecessor TB-12 Atom 2 hash be6a3ee8 superseded.  # rehashed by TB-12 Atom 2 (2026-05-03; architect 2026-05-03 ruling §3 + §8 Atom 2; Class 3): WorkTx accept arm + ChallengeTx accept arm extended with **pure additive** NodePosition side-effect — accepted WorkTx with stake>0 → FirstLong NodePosition write; accepted ChallengeTx with stake>0 → ChallengeShort NodePosition write (task_id Q-derived from target stakes_t entry). VerifyTx accept arm UNCHANGED (FR-12.3 + CR-12.8: VerifyTx.bond is responsibility bond, not market side). NO change to balances_t / stakes_t / challenge_cases_t / total_supply (CR-12.1 + CR-12.2 — NodePosition.amount NOT a Coin holding, NOT in 5-holding CTF sum). Predecessor TB-11 Atom 2 hash 2d861c68 superseded.  # rehashed by TB-11 Atom 1 (2026-05-02; architect §6.2 ruling): system_message_for_verification + system_signature_of + system_epoch_of arms extended for TaskBankruptcy; submit_agent_tx ingress fail-closed extended; dispatch_transition stub arm for TaskBankruptcy (full body lands in Atom 2); test fixture initializers updated for additive TaskExpire/TerminalSummary fields. Predecessor TB-8 round-2 hash b945cc07 superseded.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5430:   142	"tests/fc_alignment_conformance.rs" = "5e257f275d86032a957101b7898293fa9d6cdfc13a4bdd2a5f630fb9e940b408"  # rehashed by TB-15 Atom 6 (2026-05-03; architect §6.2 ruling): +4 TB-15 witnesses (FC1-N32 write_autopsy_capsule + FC1-N33 derive_autopsies_for_bankruptcy + FC2-N30 cluster_autopsies + FC3-N43 MarkovEvidenceCapsule + try_deep_history_read_with_override_check default-deny gate). Predecessor TB-14 Atom 6 B′ step 4 hash 751c78c8 superseded.  # rehashed by TB-14 Atom 6 B′ step 4 (2026-05-03; architect ruling §3+§4 — canonical-graph rewire): fc2_n28_mask_set_publication_witness updated to build CanonicalNodeGraph (BTreeMap<TxId, BTreeSet<TxId>>) directly instead of shadow Tape; compute_mask_set new signature (econ, edges, policy, price_index). Predecessor B′ step 1+2 hash c1f14584 superseded.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5452:   164	"experiments/minif2f_v4/src/bin/evaluator.rs" = "3530a2d5ac98ed8db0d037bc45fda03594c11596850ecb6be71855c6dee3abf0"  # rehashed by TB-14 Atom 6 follow-up (2026-05-03; architect ruling step 1 — Codex R1 primary VETO closure): the canonical TxId returned by `boltzmann_select_parent_v2` MUST NOT flow into legacy shadow `bus.append` parent_id (kernel.tape uses a different (shadow) id namespace; canonical TxId becomes a dangling citation → kernel.append rejects). Per architect ruling: "Use None unless a real shadow id exists." No canonical → shadow id mapping is currently available; pass None (legacy default). The v2 selector still runs (its result captured as `_v2_canonical_pick` for observability + future canonical wire-up — see B′ step 4 CanonicalNodeGraph + parent_tx replacement for last_tx_by_agent), but its output is explicitly NOT passed to bus.append. Closes Codex R1 primary VETO defect #1.  # rehashed by TB-14 Atom 6 (2026-05-03; closing OBS_TB_12_LEGACY_CPMM_QUARANTINE): production wire-swap. (1) Imports updated: `BoltzmannParams + boltzmann_select_parent` (legacy f64) → `boltzmann_select_parent_v2 + BoltzmannMaskPolicy` (integer-rational). (2) `BusConfig` literals (×2) drop `system_lp_amount` field. (3) `let params = BoltzmannParams::from_env()` → `let policy = BoltzmannMaskPolicy::from_env()`. (4) Tick-time logging derives `market_count` + top-N price ticker from `bus.snapshot().price_index` (cross-multiplication argmax; rendered `n/d`, never decimal). (5) Per-tx prompt build derives `market_ticker_str` from `snap.price_index` + queries `prompt_balance` from `bus.sequencer.q_snapshot().economic_state_t.balances_t` (replaces legacy `snap.get_balance`; the `f64` is the `prompt.rs` render contract — `prompt.rs` is not a TB-14 module surface). (6) Boltzmann selector call replaced: `boltzmann_select_parent(&snap.tape, &prices, &params, &mut rng)` → `boltzmann_select_parent_v2(&snap.price_index, &snap.mask_set, &policy, &mut rng).map(|tx| tx.0)`. Predicate-blind (CR-14.1 + halt-trigger #1) — purely a scheduling priority pick. Predecessor TB-12 Atom 0.5(a) hash 8f1b97f1 superseded. (2026-05-03; architect 2026-05-03 ruling §1.1 + §8 Atom 0.5; TB-11 G3/G4 carry-forward closure): added 4 EvidenceCapsule counters (tb11_lean_error_count / tb11_sorry_block_count / tb11_protocol_parse_failure_count / tb11_partial_accept_count) instrumented at existing classify_lean_error / classify_parse_error / step_partial_ok call sites; sorry-block vs lean-error split via err_detail.contains("sorry") || err_detail.contains("forbidden_payload"); MAX_TX exhausted exit point (inside `if let Some(bundle) = chaintape_bundle`, BEFORE bundle.shutdown) now writes EvidenceCapsule to CAS via runtime::evidence_capsule::write_evidence_capsule + emits TerminalSummary on-chain via runtime::adapter::tb11_emit_terminal_summary_for_run with capsule_cid; bundle.shutdown drains the queued TerminalSummary tx via apply_one. Mirror of TB-8 tb8_emit_finalize_after_verify pattern. Predecessor TB-10 Atom 1+3 hash a5b30cd1 superseded.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5523:   235	"src/runtime/evidence_capsule.rs" = "5266ff43f53cd1a54d3cdbb87e40b886ea4439e109f27514711846f7c84b9e03"  # rehashed by TB-11 Atom 3 (2026-05-02; architect §6.1 ruling): EvidenceCapsule writer added — write_evidence_capsule fn + CapsuleWriteError + 2 round-trip tests. TB-11 MVP stores raw log uncompressed (gzip wrapping deferred to TB-15 Markov Loom). Predecessor Atom 1 hash 7008edce superseded.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5615:      - + derive_autopsies_for_bankruptcy(pre_econ, bk, round, t)
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5621:      - + write_bankruptcy_autopsies_to_cas(cas, pre_econ, bk, ...) writer
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5626:        write_bankruptcy_autopsies_to_cas_round_trip.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5632:      - apply_one Stage 3.5 — write_bankruptcy_autopsies_to_cas() side-effect
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5689:    FC-trace: FC1-N32 + FC1-N33 (write_autopsy_capsule + apply_one hook)
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5729:+"src/state/sequencer.rs" = "9fa5936230738e5f72b14713a96238ad53583c08cdd6207d8832d27b009f50a9"  # rehashed by TB-15 Atom 3 (2026-05-03; architect §6.2 ruling): TaskBankruptcyTx dispatch arm Step 3.5 — pure derive_autopsies_for_bankruptcy(&q.economic_state_t, bk, q.q_t.current_round, bk.timestamp_logical) populates q_next.economic_state_t.agent_autopsies_t[event_id] with deterministic Cids per losing-staker. apply_one Stage 3.5 — write_bankruptcy_autopsies_to_cas() side-effect writes the capsule + private_detail bytes to CAS using the same deterministic helper (replay-safe identical Cids). NO change to monetary invariants (assert_no_post_init_mint + assert_total_ctf_conserved unchanged). NO predicate registry mutation (CR-15.3 + halt-trigger #3). NOT projected to AgentVisibleProjection (CR-15.1 + halt-trigger #1). Predecessor TB-14 Atom 6 B′ step 4 hash 1c6ba82f superseded.  # rehashed by TB-14 Atom 6 B′ step 4 (2026-05-03; architect ruling §3+§4 — canonical-graph builder): NEW `Sequencer::compute_canonical_edges_at_head() -> BTreeMap<TxId, BTreeSet<TxId>>` walks L4 + reads CAS-resident ProposalTelemetry.parent_tx for each accepted WorkTx → builds canonical parent → children edge map. Replay-deterministic per Art.0.2 (L4 + CAS payloads both replay-deterministic). Empty fallback on lock poisoned / CAS missing / decode error / no parent_tx. **Halt-trigger #2 fence preserved**: the new method body uses `crate::bottom_white::ledger::transition_ledger::canonical_decode` + `crate::runtime::proposal_telemetry::read_from_cas` (both NON-TB-14 modules — TB-3 + TB-7 surface respectively). NO TB-14 imports added to sequencer.rs `use` block. The forbidden tokens (compute_price_index / compute_mask_set / NodeMarketEntry / RationalPrice / BoltzmannMaskPolicy) remain absent from sequencer.rs. Predecessor TB-13 Atom 6 round-5 hash 2b8a1b36 superseded. (2026-05-03; Codex RQ5 remediation): drop redundant inner-consistency check `redeem.outcome != redeem.resolution_ref.claimed_outcome` from CompleteSetRedeem dispatch — ResolutionRef wrapper removed in typed_tx.rs (claimed_outcome was a redundant copy of redeem.outcome; resolution_tx_id was opaque/L4-unvalidated dead wire weight). Step 1 deleted; remaining steps renumbered 1..6. State-mismatch path (Finalized+No / Bankrupt+Yes → InvalidResolutionRef) preserved via existing match arm using redeem.outcome directly. Predecessor TB-12 Atom 2 hash be6a3ee8 superseded.  # rehashed by TB-12 Atom 2 (2026-05-03; architect 2026-05-03 ruling §3 + §8 Atom 2; Class 3): WorkTx accept arm + ChallengeTx accept arm extended with **pure additive** NodePosition side-effect — accepted WorkTx with stake>0 → FirstLong NodePosition write; accepted ChallengeTx with stake>0 → ChallengeShort NodePosition write (task_id Q-derived from target stakes_t entry). VerifyTx accept arm UNCHANGED (FR-12.3 + CR-12.8: VerifyTx.bond is responsibility bond, not market side). NO change to balances_t / stakes_t / challenge_cases_t / total_supply (CR-12.1 + CR-12.2 — NodePosition.amount NOT a Coin holding, NOT in 5-holding CTF sum). Predecessor TB-11 Atom 2 hash 2d861c68 superseded.  # rehashed by TB-11 Atom 1 (2026-05-02; architect §6.2 ruling): system_message_for_verification + system_signature_of + system_epoch_of arms extended for TaskBankruptcy; submit_agent_tx ingress fail-closed extended; dispatch_transition stub arm for TaskBankruptcy (full body lands in Atom 2); test fixture initializers updated for additive TaskExpire/TerminalSummary fields. Predecessor TB-8 round-2 hash b945cc07 superseded.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5756:      - write_markov_capsule(...) CAS writer (puts NextSessionContext +
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5768:          write_markov_capsule_to_cas_round_trip (writer puts 2 CAS objects)
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5769:          write_markov_capsule_deterministic_capsule_id
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5880:      - fc1_n32_write_autopsy_capsule_witness
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5954:    replay-deterministic per derive_autopsies_for_bankruptcy PURE helper
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:5990:+"tests/fc_alignment_conformance.rs" = "5e257f275d86032a957101b7898293fa9d6cdfc13a4bdd2a5f630fb9e940b408"  # rehashed by TB-15 Atom 6 (2026-05-03; architect §6.2 ruling): +4 TB-15 witnesses (FC1-N32 write_autopsy_capsule + FC1-N33 derive_autopsies_for_bankruptcy + FC2-N30 cluster_autopsies + FC3-N43 MarkovEvidenceCapsule + try_deep_history_read_with_override_check default-deny gate). Predecessor TB-14 Atom 6 B′ step 4 hash 751c78c8 superseded.  # rehashed by TB-14 Atom 6 B′ step 4 (2026-05-03; architect ruling §3+§4 — canonical-graph rewire): fc2_n28_mask_set_publication_witness updated to build CanonicalNodeGraph (BTreeMap<TxId, BTreeSet<TxId>>) directly instead of shadow Tape; compute_mask_set new signature (econ, edges, policy, price_index). Predecessor B′ step 1+2 hash c1f14584 superseded.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:6071:      - write_autopsy_capsule(...) CAS writer (puts AutopsyPrivateDetail +
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:6099:        write_autopsy_capsule_to_cas_round_trip (writer puts 2 CAS objects;
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:6101:        write_autopsy_capsule_deterministic_capsule_id (same inputs → same
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:6120:    FC-trace: FC1-N32 (write_autopsy_capsule writer)
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:6448:handover/tracer_bullets/TB_LOG.tsv:45:# 2026-05-03 TB-15 SHIPPED — "P4 Information Loom (primary) + P5 MetaTape v0 prep: Lamarckian Autopsy + Markov EvidenceCapsule per architect 2026-05-02 + 2026-05-03 §6 verbatim spec (FR-15.1..6 + CR-15.1..6 + SG-15.1..8 + 6 halt triggers + forbidden list)". Class 2 envelope (self-audit; AgentVisibleProjection unchanged; only one new sequencer dispatch hook — TaskBankruptcyTx Step 3.5 + apply_one Stage 3.5). 7-atom plan (charter + halt fixture + AgentAutopsyCapsule schema/writer + AutopsyIndex/TaskBankruptcyTx wire-in + cluster_autopsies + MarkovEvidenceCapsule schema/generator + dashboard §15/first capsule/SHIP) all green. Auto-resolutions per feedback_architect_deviation_stance: A) single trigger site = TaskBankruptcyTx (SlashTx wires when RSP-3.2 ships; FinalizeReward loser branch wires when RSP-4 ships); B) EconomicState 12→13 sub-fields with +agent_autopsies_t serde-default; C) privacy default = AuditOnly (re-use TB-11 CapsulePrivacyPolicy); D) clustering threshold N=3 per architect §3.2.3 verbatim; E) LossReasonClass 8 variants incl Other(String); F) Markov override = env TURINGOS_MARKOV_OVERRIDE=1; G) OBS_RESOLUTIONS_INDEX_TB15 DEFERRED out of TB-15 (TB-13 audit residue, not in architect §6 spec); H) Class 2 self-audit (no AgentVisibleProjection mod). cargo test --workspace = 870/0/150 PASS (+67 net vs TB-14 ship 803). All 6 halt-triggers GREEN: #1 raw_logs_not_in_general_read_view (Atom 3 file-scan AgentVisibleProjection) + #2 markov_capsule_references_constitution_hash (Atom 5 SG-15.7) + #3 autopsy_does_not_mutate_predicates (Atom 2 file-scan no &mut PredicateRegistry/ToolRegistry/RiskPolicyRegistry/register_*/unregister_*/patch_*; CR-15.3 + SG-15.8) + #4 private_detail_not_in_other_agent_view (Atom 3 file-scan AutopsyIndex value type Vec<Cid> not raw bytes; SG-15.2) + #5 typical_error_clustering_uses_summary_only (Atom 4 cluster_autopsies serialization scan no private_detail_cid byte run; CR-15.2 + SG-15.5) + #6 deep_history_read_without_override_fails (Atom 5 try_deep_history_read_with_override_check default-deny; SG-15.4 + FR-15.5). Architectural deltas: NEW src/runtime/autopsy_capsule.rs (LossReasonClass enum + AgentAutopsyCapsule struct + format_public_summary + write_autopsy_capsule + derive_autopsies_for_bankruptcy PURE deterministic helper + write_bankruptcy_autopsies_to_cas + cluster_autopsies + TypicalErrorSummary + AutopsyWriteError + 15 unit tests); NEW src/runtime/markov_capsule.rs (ObsId newtype + MarkovEvidenceCapsule struct + with_constitution_hash + try_deep_history_read_with_override_check + override_set_from_env + write_markov_capsule + scan_unresolved_obs + sha256_of_file + MarkovGenError + 8 unit tests); NEW src/bin/generate_markov_capsule.rs (CLI binary with --tb-id/--out-dir/--constitution-path/--cas-dir/--prev-cid-hex/--alignment-dir/--no-cas args + TURINGOS_MARKOV_OVERRIDE env); NEW tests/tb_15_halt_triggers.rs (6 fixtures); MOD src/state/typed_tx.rs (+RiskRuleId opaque newtype); MOD src/bottom_white/cas/schema.rs (+4 ObjectType variants AgentAutopsyCapsule + AutopsyPrivateDetail + MarkovEvidenceCapsule + NextSessionContext); MOD src/state/q_state.rs (+AutopsyIndex(BTreeMap<EventId, Vec<Cid>>) newtype + agent_autopsies_t 13th sub-field on EconomicState serde-default; sub-field count assertion 12→13); MOD src/state/sequencer.rs (TaskBankruptcyTx dispatch arm Step 3.5 PURE deterministic Cid derivation + apply_one Stage 3.5 CAS write side-effect via deterministic helper; NO predicate registry mutation; NO AgentVisibleProjection mod; NO change to monetary invariants); MOD src/runtime/mod.rs (+pub mod autopsy_capsule + pub mod markov_capsule); MOD src/bin/audit_dashboard.rs (+render_section_15 pure render with AUTOPSY IS PRIVATE banner + autopsy_event_counts + latest_markov_capsule_cid_hex fields on DashboardReport + read_latest_markov_pointer helper + 4 SG-15.6 dashboard render tests); MOD 4 test fixtures sub-field count 12→13 (economic_state_reconstruct/q_state_reconstruct/six_axioms_alignment); MOD tests/fc_alignment_conformance.rs (+4 TB-15 witnesses FC1-N32 write_autopsy_capsule + FC1-N33 derive_autopsies_for_bankruptcy + FC2-N30 cluster_autopsies + FC3-N43 MarkovEvidenceCapsule); REHASH genesis_payload.toml trust_root for 6 modified files (src/runtime/mod.rs ×2 hops Atom2→Atom5; src/state/q_state.rs Atom3; src/state/typed_tx.rs Atom2; src/state/sequencer.rs Atom3; src/bottom_white/cas/schema.rs Atom2; tests/fc_alignment_conformance.rs Atom6). First Markov capsule emitted (capsule_id b244f16a1f3bd532d041a40fe39b2b7e7cc12fb58e18b61aedd76a8010eeb1b6) with constitution_hash eec695459c71fbef3685583485deb431fe3b561657b2f285b7c5e7e220e59e03; evidence at handover/evidence/tb_15_markov_capsule_2026-05-03/. Production claim: "TB-15 establishes Lamarckian Autopsy + Markov EvidenceCapsule substrate. AgentAutopsyCapsule (per-agent, per-event, AuditOnly) records loss/bankruptcy events derived deterministically from ChainTape evidence — NEVER LLM self-narration. agent_autopsies_t lives sequencer-side (NOT projected to AgentVisibleProjection per CR-15.1 + halt-trigger #1). TypicalErrorBroadcast clustering at N≥3 emits public_summary text + Cids only — NEVER private_detail_cid bytes. MarkovEvidenceCapsule binds constitution_hash + L4 + L4.E + CAS roots + previous capsule + typical_errors + unresolved_obs as next-session bootstrap default; deeper history requires TURINGOS_MARKOV_OVERRIDE=1. CR-15.3/15.4 STRUCTURALLY ENFORCED via writer signature + halt-trigger #3 file-scan. Constitutional alignment: Art.II.1 + Art.III.1 + Art.III.2 + Art.III.3 + Art.III.4 + Art.V.1 + Art.0.2." Open follow-ups (carry-forward NOT ship blockers): multi-site autopsy wire-in (SlashLoss/ChallengeUnsuccessful/VerifierBondLost — when RSP-3.2/RSP-4 ship); L4/L4.E/CAS root chain-readers in Markov generator (currently zero placeholders); CAS-walking dashboard §15 (currently empty autopsy_event_counts); InitAI agent-side honoring of Markov default (P5 v1); OBS_RESOLUTIONS_INDEX_TB15 explicitly deferred per charter §7-G.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:6653:   608	    let a = derive_autopsies_for_bankruptcy(&econ, &bk, 1, 5);
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:6654:   609	    let b = derive_autopsies_for_bankruptcy(&econ, &bk, 1, 5);
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:7109:/bin/bash -lc 'rg -n "std::fs::write|create_dir_all|write_markov_capsule|CasStore::open" src/bin/generate_markov_capsule.rs src/runtime/markov_capsule.rs' in /home/zephryj/projects/turingosv4
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:7111:src/runtime/markov_capsule.rs:10://! Generator surface: `write_markov_capsule(...)` (CAS-emitting) + binary
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:7112:src/runtime/markov_capsule.rs:214:pub fn write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:7113:src/runtime/markov_capsule.rs:365:    /// TB-15 Atom 5 — write_markov_capsule writes 2 CAS objects
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:7114:src/runtime/markov_capsule.rs:369:    fn write_markov_capsule_to_cas_round_trip() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:7116:src/runtime/markov_capsule.rs:382:        let cap = write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:7117:src/runtime/markov_capsule.rs:410:    /// TB-15 Atom 5 — write_markov_capsule deterministic (same inputs →
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:7118:src/runtime/markov_capsule.rs:413:    fn write_markov_capsule_deterministic_capsule_id() {
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:7120:src/runtime/markov_capsule.rs:420:            write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:7122:src/runtime/markov_capsule.rs:451:        let cap_1 = write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:7123:src/runtime/markov_capsule.rs:465:        let cap_2 = write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:7124:src/bin/generate_markov_capsule.rs:42:    override_set_from_env, scan_unresolved_obs, sha256_of_file, write_markov_capsule, MarkovGenError,
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:7126:src/bin/generate_markov_capsule.rs:269:        write_markov_capsule(
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md:7127:src/bin/generate_markov_capsule.rs:286:            other => format!("write_markov_capsule: {other}"),
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R3.md:2385:handover/tracer_bullets/TB_LOG.tsv:39:# 2026-05-03 TB-12 SHIPPED — "P3 carry-forward + P4 primary: Node Exposure Index per architect 2026-05-03 ruling §3 + §10". Class 3 envelope (Atom 1 = Class 2 schema; Atom 2 = Class 3 dispatch wire; Atom 6 = Class 3 dual audit Codex + Gemini both ✓). Self-audit PASS; Codex CHALLENGE × 2 (Q4 doc-drift on 4-vs-5-holding total_supply; Q5 legacy CPMM in src/prediction_market.rs predates TB-12) — both resolved as documentation/scope clarifications via recursive audit §10 + OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md (TB-13 prerequisite); Gemini round-1 PASS / high conviction / PROCEED to SHIP (gemini-2.5-pro, 896k char prompt, 48.2s API; 8/8 audit questions PASS). User-architect ultrathink verified architect §9 strict alignment before SHIP — found 4 SG-12.x test-name drifts (SG-12.5 / 12.6 / 12.7 / 12.8 names didn't exactly match architect's "passes" strings); all 4 fixed BEFORE SHIP per feedback_no_retroactive_evidence_rewrite (NOT post-ship patch): SG-12.5 renamed to sg_12_5_replay_reconstructs_node_positions; SG-12.6 ADDED sg_12_6_dashboard_view_positions_works inside src/bin/audit_dashboard.rs #[cfg(test)] mod tb12_render_tests with 4 cases + forbidden-token grep; SG-12.7 renamed to sg_12_7_no_market_trading_variants_introduced; SG-12.8 ADDED sg_12_8_no_node_market_entry_as_canonical_state at integration-test level (q_state.rs unit alias kept as defense-in-depth). 8/8 SG-12.x by architect §9.3 EXACT names PASS. cargo test --workspace = 759/0/150 (+12 net vs TB-11 ship 747; +28 vs TB-10 ship 731). Architectural deltas: NEW src/state/typed_tx.rs (NodePosition struct + PositionSide enum + PositionKind enum); EXTEND src/state/q_state.rs (NodePositionsIndex flat shape + EconomicState 10→11 sub-fields with +node_positions_t serde-default); EXTEND src/state/sequencer.rs (WorkTx + ChallengeTx accept-arm pure-additive NodePosition write — no balances_t / stakes_t / challenge_cases_t / total_supply mutation; CR-12.1 + CR-12.2 verified); EXTEND src/economy/monetary_invariant.rs (NodePosition NOT in 4-holding total_supply_micro; structural — assert_total_ctf_conserved unchanged); EXTEND src/bin/audit_dashboard.rs (§13 Node exposure records + render_section_13 pure-fn helper for SG-12.6 unit-testability + LABEL DISCIPLINE "exposure records" NOT "Open market balances" per architect §8 Atom 4); EXTEND experiments/minif2f_v4/src/bin/evaluator.rs (TB-11 G3/G4 carry-forward — 4 EvidenceCapsule counters + write_evidence_capsule + tb11_emit_terminal_summary_for_run wired at MAX_TX exhausted exit point); EXTEND experiments/minif2f_v4/src/bin/lean_market.rs (3 new subcommands: tick POLICY PREVIEW MODE — actual emission deferred to system_keypair persistence + view-bankruptcy read-only listing + view-positions read-only NodePosition browser); REHASH genesis_payload.toml trust_root for src/state/sequencer.rs + src/state/typed_tx.rs + src/state/q_state.rs + src/state/mod.rs + src/bin/audit_dashboard.rs + experiments/minif2f_v4/src/bin/evaluator.rs (lean_market.rs outside trust root by TB-10 precedent). Halting triggers (architect §7) NONE triggered: CTF conservation passes (Gemini Q1+Q4); position-tx field derivation exact (position_fields_derived_from_source_tx_exactly test PASS); NodePosition NOT counted as Coin (4-holding model unchanged); replay deterministic (sg_12_5 PASS); no Codex/Gemini VETO. Production claim: "TB-12 establishes a flat, immutable, non-monetary NodePositionsIndex anchoring WorkTx.stake → FirstLong + ChallengeTx.stake → ChallengeShort exposure records on canonical L4-derived QState. NodePosition is exposure record NOT active position balance per architect §10 critical insight; TB-13 CompleteSet introduces real YES/NO claims; TB-14 PriceIndex computes price from long/short interest; TB-16 controlled-arena introduces P&L. Charter Q6 (ii.5) sync mode honored — dual audit complete + user-architect ultrathink verdict before SHIP." Open follow-ups (carry-forward, NOT ship blockers): TB-13 prerequisite quarantine of src/prediction_market.rs legacy f64 CPMM (OBS-tracked); system_keypair persistence + reattachment factory for lean_market tick on-chain emission; real-LLM zeta-regularization rerun (Atom 0.5(a) wired the call site; manual user-driven session post-ship).
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R3.md:10785:handover/tracer_bullets/TB_LOG.tsv:36:# 2026-05-02 TB-11 SHIPPED — "P3 carry-forward + P4 primary + P6 carry-forward: Epistemic Exhaust & Capital Liberation per architect ruling 2026-05-02 §6.2". Class 3 risk class (capital-mover via TaskExpire refund + 3 system-emitted variants); recursive self-audit PASS — 4/4 clauses GREEN (Constitutional preservation / Replay-deterministic / Conservation / Negative-truth-completeness); 9/11 ship gates GREEN + 2/11 ⚠ deferred (G3 lean_market 6 subcommands wire-up + G4 evaluator forced-exhaust real-LLM smoke); 7/7 architect SG-11.1..7 ship gates structurally satisfied (capsule-on-hard-fail / RunExhausted in L4 / TaskExpire refunds / CTF preserved / dashboard regenerates / raw evidence shielded / future-Short anchor). Kernel-level architectural core SHIPPED (Atoms 1-5 of 8): TypedTx schemas + sequencer dispatch + emit_system_tx commands + EvidenceCapsule CAS writer + audit_dashboard §12. Atom 6 = smoke-evidence README composing pre-existing TB-13 PREVIEW zeta corpus (132 attempts/0 OMEGA/500_000 stuck) + 5 deterministic TB-11 integration tests as the proof. Atom 7 = recursive self-audit doc with explicit external Codex+Gemini deferral rationale (§8: TaskExpire is the sole net-new economic mutator and structurally mirrors TB-8 dual-audited FinalizeReward; capsule writer is purely additive; architect ruling itself was the architectural review). Atom 8 = this LATEST.md update + TB_LOG row + ship commit. Architectural deltas: (a) src/state/typed_tx.rs +TaskBankruptcyTx struct +TaskBankruptcySigningPayload +DOMAIN_SYSTEM_TASK_BANKRUPTCY domain prefix +RunExhaustedTx type alias for TerminalSummaryTx in failure path +TerminalSummaryTx additive bump (+parent_state_root +solver_agent +evidence_capsule_cid; no production rows pre-TB-11; safe per feedback_no_retroactive_evidence_rewrite) +TaskExpireTx additive bump (+sponsor_agent +escrow_tx_id +reason); +ExpireReason +BankruptcyReason +ExhaustionReason +CapsulePrivacyPolicy enums; golden digest constants rotated. (b) src/state/q_state.rs EconomicState 9→10 sub-fields with +runs_t: RunsIndex; +RunSummaryEntry struct; +TaskMarketState enum {Open, Expired, Bankrupt, Finalized}; TaskMarketEntry additive (+state +bankruptcy_at_logical_t +opened_at_logical_t). (c) src/state/sequencer.rs +3 dispatch arms (TaskExpire refund / TerminalSummary RunsIndex anchor / TaskBankruptcy state-flip); +3 SystemEmitCommand variants; +3 state-root domain helpers (TASK_EXPIRE_DOMAIN_V1 / TERMINAL_SUMMARY_DOMAIN_V1 / TASK_BANKRUPTCY_DOMAIN_V1); submit_agent_tx ingress fail-closed extended; verify_emitted_system_tx_signature extended. (d) src/runtime/evidence_capsule.rs NEW module — EvidenceCapsule schema (16 fields incl. 5 architect-mandated counts) + ExhaustionCounts helper + format_public_summary + write_evidence_capsule writer (3 CAS objects per capsule: raw log + manifest + capsule); CapsulePrivacyPolicy::AuditOnly default. (e) src/bottom_white/cas/schema.rs +3 ObjectType variants (EvidenceCapsule / EvidenceManifest / CompressedRunLog). (f) src/bottom_white/ledger/{system_keypair,transition_ledger}.rs +CanonicalMessage::TaskBankruptcySigning +sign_task_bankruptcy helper +TxKind::TaskBankruptcy=10. (g) src/runtime/adapter.rs +tb11_emit_terminal_summary_for_run +tb11_emit_expire_for_eligible runtime helpers (mirror tb8 pattern; thin wrappers over emit_system_tx with policy-gated eligibility scan). (h) src/bin/audit_dashboard.rs +§12 with 3 sub-tables (Exhausted runs / Expired tasks / Bankrupt tasks) + total-refund aggregation + privacy-mandate footer. cargo test --workspace = 747/0/150 (+16 net vs TB-10 baseline 731). Trust Root rehashed: 11 entries (typed_tx / q_state / sequencer / state/mod / cas/schema / system_keypair / transition_ledger / runtime/mod / run_summary / monetary_invariant / adapter / audit_dashboard) + 1 NEW entry (runtime/evidence_capsule.rs). Forward-binding: TB-12 NodeMarket Position Index now has the on-chain anchor it needs (TaskBankruptcyTx for future Short / NO settlement). Open follow-ups (carry-forward, NOT ship blockers): evaluator binary integration (call write_evidence_capsule + tb11_emit_terminal_summary_for_run on MAX_TX exhausted); lean_market binary tick + view-bankruptcy subcommands; real-LLM zeta-regularization smoke producing single tar.gz; (optional) Codex + Gemini external audits available on request via existing audit script harness.
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R3.md:10786:handover/tracer_bullets/TB_LOG.tsv:39:# 2026-05-03 TB-12 SHIPPED — "P3 carry-forward + P4 primary: Node Exposure Index per architect 2026-05-03 ruling §3 + §10". Class 3 envelope (Atom 1 = Class 2 schema; Atom 2 = Class 3 dispatch wire; Atom 6 = Class 3 dual audit Codex + Gemini both ✓). Self-audit PASS; Codex CHALLENGE × 2 (Q4 doc-drift on 4-vs-5-holding total_supply; Q5 legacy CPMM in src/prediction_market.rs predates TB-12) — both resolved as documentation/scope clarifications via recursive audit §10 + OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md (TB-13 prerequisite); Gemini round-1 PASS / high conviction / PROCEED to SHIP (gemini-2.5-pro, 896k char prompt, 48.2s API; 8/8 audit questions PASS). User-architect ultrathink verified architect §9 strict alignment before SHIP — found 4 SG-12.x test-name drifts (SG-12.5 / 12.6 / 12.7 / 12.8 names didn't exactly match architect's "passes" strings); all 4 fixed BEFORE SHIP per feedback_no_retroactive_evidence_rewrite (NOT post-ship patch): SG-12.5 renamed to sg_12_5_replay_reconstructs_node_positions; SG-12.6 ADDED sg_12_6_dashboard_view_positions_works inside src/bin/audit_dashboard.rs #[cfg(test)] mod tb12_render_tests with 4 cases + forbidden-token grep; SG-12.7 renamed to sg_12_7_no_market_trading_variants_introduced; SG-12.8 ADDED sg_12_8_no_node_market_entry_as_canonical_state at integration-test level (q_state.rs unit alias kept as defense-in-depth). 8/8 SG-12.x by architect §9.3 EXACT names PASS. cargo test --workspace = 759/0/150 (+12 net vs TB-11 ship 747; +28 vs TB-10 ship 731). Architectural deltas: NEW src/state/typed_tx.rs (NodePosition struct + PositionSide enum + PositionKind enum); EXTEND src/state/q_state.rs (NodePositionsIndex flat shape + EconomicState 10→11 sub-fields with +node_positions_t serde-default); EXTEND src/state/sequencer.rs (WorkTx + ChallengeTx accept-arm pure-additive NodePosition write — no balances_t / stakes_t / challenge_cases_t / total_supply mutation; CR-12.1 + CR-12.2 verified); EXTEND src/economy/monetary_invariant.rs (NodePosition NOT in 4-holding total_supply_micro; structural — assert_total_ctf_conserved unchanged); EXTEND src/bin/audit_dashboard.rs (§13 Node exposure records + render_section_13 pure-fn helper for SG-12.6 unit-testability + LABEL DISCIPLINE "exposure records" NOT "Open market balances" per architect §8 Atom 4); EXTEND experiments/minif2f_v4/src/bin/evaluator.rs (TB-11 G3/G4 carry-forward — 4 EvidenceCapsule counters + write_evidence_capsule + tb11_emit_terminal_summary_for_run wired at MAX_TX exhausted exit point); EXTEND experiments/minif2f_v4/src/bin/lean_market.rs (3 new subcommands: tick POLICY PREVIEW MODE — actual emission deferred to system_keypair persistence + view-bankruptcy read-only listing + view-positions read-only NodePosition browser); REHASH genesis_payload.toml trust_root for src/state/sequencer.rs + src/state/typed_tx.rs + src/state/q_state.rs + src/state/mod.rs + src/bin/audit_dashboard.rs + experiments/minif2f_v4/src/bin/evaluator.rs (lean_market.rs outside trust root by TB-10 precedent). Halting triggers (architect §7) NONE triggered: CTF conservation passes (Gemini Q1+Q4); position-tx field derivation exact (position_fields_derived_from_source_tx_exactly test PASS); NodePosition NOT counted as Coin (4-holding model unchanged); replay deterministic (sg_12_5 PASS); no Codex/Gemini VETO. Production claim: "TB-12 establishes a flat, immutable, non-monetary NodePositionsIndex anchoring WorkTx.stake → FirstLong + ChallengeTx.stake → ChallengeShort exposure records on canonical L4-derived QState. NodePosition is exposure record NOT active position balance per architect §10 critical insight; TB-13 CompleteSet introduces real YES/NO claims; TB-14 PriceIndex computes price from long/short interest; TB-16 controlled-arena introduces P&L. Charter Q6 (ii.5) sync mode honored — dual audit complete + user-architect ultrathink verdict before SHIP." Open follow-ups (carry-forward, NOT ship blockers): TB-13 prerequisite quarantine of src/prediction_market.rs legacy f64 CPMM (OBS-tracked); system_keypair persistence + reattachment factory for lean_market tick on-chain emission; real-LLM zeta-regularization rerun (Atom 0.5(a) wired the call site; manual user-driven session post-ship).
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R5.md:3281:   164	"experiments/minif2f_v4/src/bin/evaluator.rs" = "8f1b97f1e8354062e7b05c82f36586dc080860554c99ee3f06b80ba76919161f"  # rehashed by TB-12 Atom 0.5(a) (2026-05-03; architect 2026-05-03 ruling §1.1 + §8 Atom 0.5; TB-11 G3/G4 carry-forward closure): added 4 EvidenceCapsule counters (tb11_lean_error_count / tb11_sorry_block_count / tb11_protocol_parse_failure_count / tb11_partial_accept_count) instrumented at existing classify_lean_error / classify_parse_error / step_partial_ok call sites; sorry-block vs lean-error split via err_detail.contains("sorry") || err_detail.contains("forbidden_payload"); MAX_TX exhausted exit point (inside `if let Some(bundle) = chaintape_bundle`, BEFORE bundle.shutdown) now writes EvidenceCapsule to CAS via runtime::evidence_capsule::write_evidence_capsule + emits TerminalSummary on-chain via runtime::adapter::tb11_emit_terminal_summary_for_run with capsule_cid; bundle.shutdown drains the queued TerminalSummary tx via apply_one. Mirror of TB-8 tb8_emit_finalize_after_verify pattern. Predecessor TB-10 Atom 1+3 hash a5b30cd1 superseded.
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R5.md:3676:handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R4.md:2973:handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R3.md:2385:handover/tracer_bullets/TB_LOG.tsv:39:# 2026-05-03 TB-12 SHIPPED — "P3 carry-forward + P4 primary: Node Exposure Index per architect 2026-05-03 ruling §3 + §10". Class 3 envelope (Atom 1 = Class 2 schema; Atom 2 = Class 3 dispatch wire; Atom 6 = Class 3 dual audit Codex + Gemini both ✓). Self-audit PASS; Codex CHALLENGE × 2 (Q4 doc-drift on 4-vs-5-holding total_supply; Q5 legacy CPMM in src/prediction_market.rs predates TB-12) — both resolved as documentation/scope clarifications via recursive audit §10 + OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md (TB-13 prerequisite); Gemini round-1 PASS / high conviction / PROCEED to SHIP (gemini-2.5-pro, 896k char prompt, 48.2s API; 8/8 audit questions PASS). User-architect ultrathink verified architect §9 strict alignment before SHIP — found 4 SG-12.x test-name drifts (SG-12.5 / 12.6 / 12.7 / 12.8 names didn't exactly match architect's "passes" strings); all 4 fixed BEFORE SHIP per feedback_no_retroactive_evidence_rewrite (NOT post-ship patch): SG-12.5 renamed to sg_12_5_replay_reconstructs_node_positions; SG-12.6 ADDED sg_12_6_dashboard_view_positions_works inside src/bin/audit_dashboard.rs #[cfg(test)] mod tb12_render_tests with 4 cases + forbidden-token grep; SG-12.7 renamed to sg_12_7_no_market_trading_variants_introduced; SG-12.8 ADDED sg_12_8_no_node_market_entry_as_canonical_state at integration-test level (q_state.rs unit alias kept as defense-in-depth). 8/8 SG-12.x by architect §9.3 EXACT names PASS. cargo test --workspace = 759/0/150 (+12 net vs TB-11 ship 747; +28 vs TB-10 ship 731). Architectural deltas: NEW src/state/typed_tx.rs (NodePosition struct + PositionSide enum + PositionKind enum); EXTEND src/state/q_state.rs (NodePositionsIndex flat shape + EconomicState 10→11 sub-fields with +node_positions_t serde-default); EXTEND src/state/sequencer.rs (WorkTx + ChallengeTx accept-arm pure-additive NodePosition write — no balances_t / stakes_t / challenge_cases_t / total_supply mutation; CR-12.1 + CR-12.2 verified); EXTEND src/economy/monetary_invariant.rs (NodePosition NOT in 4-holding total_supply_micro; structural — assert_total_ctf_conserved unchanged); EXTEND src/bin/audit_dashboard.rs (§13 Node exposure records + render_section_13 pure-fn helper for SG-12.6 unit-testability + LABEL DISCIPLINE "exposure records" NOT "Open market balances" per architect §8 Atom 4); EXTEND experiments/minif2f_v4/src/bin/evaluator.rs (TB-11 G3/G4 carry-forward — 4 EvidenceCapsule counters + write_evidence_capsule + tb11_emit_terminal_summary_for_run wired at MAX_TX exhausted exit point); EXTEND experiments/minif2f_v4/src/bin/lean_market.rs (3 new subcommands: tick POLICY PREVIEW MODE — actual emission deferred to system_keypair persistence + view-bankruptcy read-only listing + view-positions read-only NodePosition browser); REHASH genesis_payload.toml trust_root for src/state/sequencer.rs + src/state/typed_tx.rs + src/state/q_state.rs + src/state/mod.rs + src/bin/audit_dashboard.rs + experiments/minif2f_v4/src/bin/evaluator.rs (lean_market.rs outside trust root by TB-10 precedent). Halting triggers (architect §7) NONE triggered: CTF conservation passes (Gemini Q1+Q4); position-tx field derivation exact (position_fields_derived_from_source_tx_exactly test PASS); NodePosition NOT counted as Coin (4-holding model unchanged); replay deterministic (sg_12_5 PASS); no Codex/Gemini VETO. Production claim: "TB-12 establishes a flat, immutable, non-monetary NodePositionsIndex anchoring WorkTx.stake → FirstLong + ChallengeTx.stake → ChallengeShort exposure records on canonical L4-derived QState. NodePosition is exposure record NOT active position balance per architect §10 critical insight; TB-13 CompleteSet introduces real YES/NO claims; TB-14 PriceIndex computes price from long/short interest; TB-16 controlled-arena introduces P&L. Charter Q6 (ii.5) sync mode honored — dual audit complete + user-architect ultrathink verdict before SHIP." Open follow-ups (carry-forward, NOT ship blockers): TB-13 prerequisite quarantine of src/prediction_market.rs legacy f64 CPMM (OBS-tracked); system_keypair persistence + reattachment factory for lean_market tick on-chain emission; real-LLM zeta-regularization rerun (Atom 0.5(a) wired the call site; manual user-driven session post-ship).
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R5.md:3704:handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R4.md:3581:handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R3.md:10786:handover/tracer_bullets/TB_LOG.tsv:39:# 2026-05-03 TB-12 SHIPPED — "P3 carry-forward + P4 primary: Node Exposure Index per architect 2026-05-03 ruling §3 + §10". Class 3 envelope (Atom 1 = Class 2 schema; Atom 2 = Class 3 dispatch wire; Atom 6 = Class 3 dual audit Codex + Gemini both ✓). Self-audit PASS; Codex CHALLENGE × 2 (Q4 doc-drift on 4-vs-5-holding total_supply; Q5 legacy CPMM in src/prediction_market.rs predates TB-12) — both resolved as documentation/scope clarifications via recursive audit §10 + OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md (TB-13 prerequisite); Gemini round-1 PASS / high conviction / PROCEED to SHIP (gemini-2.5-pro, 896k char prompt, 48.2s API; 8/8 audit questions PASS). User-architect ultrathink verified architect §9 strict alignment before SHIP — found 4 SG-12.x test-name drifts (SG-12.5 / 12.6 / 12.7 / 12.8 names didn't exactly match architect's "passes" strings); all 4 fixed BEFORE SHIP per feedback_no_retroactive_evidence_rewrite (NOT post-ship patch): SG-12.5 renamed to sg_12_5_replay_reconstructs_node_positions; SG-12.6 ADDED sg_12_6_dashboard_view_positions_works inside src/bin/audit_dashboard.rs #[cfg(test)] mod tb12_render_tests with 4 cases + forbidden-token grep; SG-12.7 renamed to sg_12_7_no_market_trading_variants_introduced; SG-12.8 ADDED sg_12_8_no_node_market_entry_as_canonical_state at integration-test level (q_state.rs unit alias kept as defense-in-depth). 8/8 SG-12.x by architect §9.3 EXACT names PASS. cargo test --workspace = 759/0/150 (+12 net vs TB-11 ship 747; +28 vs TB-10 ship 731). Architectural deltas: NEW src/state/typed_tx.rs (NodePosition struct + PositionSide enum + PositionKind enum); EXTEND src/state/q_state.rs (NodePositionsIndex flat shape + EconomicState 10→11 sub-fields with +node_positions_t serde-default); EXTEND src/state/sequencer.rs (WorkTx + ChallengeTx accept-arm pure-additive NodePosition write — no balances_t / stakes_t / challenge_cases_t / total_supply mutation; CR-12.1 + CR-12.2 verified); EXTEND src/economy/monetary_invariant.rs (NodePosition NOT in 4-holding total_supply_micro; structural — assert_total_ctf_conserved unchanged); EXTEND src/bin/audit_dashboard.rs (§13 Node exposure records + render_section_13 pure-fn helper for SG-12.6 unit-testability + LABEL DISCIPLINE "exposure records" NOT "Open market balances" per architect §8 Atom 4); EXTEND experiments/minif2f_v4/src/bin/evaluator.rs (TB-11 G3/G4 carry-forward — 4 EvidenceCapsule counters + write_evidence_capsule + tb11_emit_terminal_summary_for_run wired at MAX_TX exhausted exit point); EXTEND experiments/minif2f_v4/src/bin/lean_market.rs (3 new subcommands: tick POLICY PREVIEW MODE — actual emission deferred to system_keypair persistence + view-bankruptcy read-only listing + view-positions read-only NodePosition browser); REHASH genesis_payload.toml trust_root for src/state/sequencer.rs + src/state/typed_tx.rs + src/state/q_state.rs + src/state/mod.rs + src/bin/audit_dashboard.rs + experiments/minif2f_v4/src/bin/evaluator.rs (lean_market.rs outside trust root by TB-10 precedent). Halting triggers (architect §7) NONE triggered: CTF conservation passes (Gemini Q1+Q4); position-tx field derivation exact (position_fields_derived_from_source_tx_exactly test PASS); NodePosition NOT counted as Coin (4-holding model unchanged); replay deterministic (sg_12_5 PASS); no Codex/Gemini VETO. Production claim: "TB-12 establishes a flat, immutable, non-monetary NodePositionsIndex anchoring WorkTx.stake → FirstLong + ChallengeTx.stake → ChallengeShort exposure records on canonical L4-derived QState. NodePosition is exposure record NOT active position balance per architect §10 critical insight; TB-13 CompleteSet introduces real YES/NO claims; TB-14 PriceIndex computes price from long/short interest; TB-16 controlled-arena introduces P&L. Charter Q6 (ii.5) sync mode honored — dual audit complete + user-architect ultrathink verdict before SHIP." Open follow-ups (carry-forward, NOT ship blockers): TB-13 prerequisite quarantine of src/prediction_market.rs legacy f64 CPMM (OBS-tracked); system_keypair persistence + reattachment factory for lean_market tick on-chain emission; real-LLM zeta-regularization rerun (Atom 0.5(a) wired the call site; manual user-driven session post-ship).
handover/audits/run_codex_tb_15_ship_audit_r2.sh:228:`derive_autopsies_for_bankruptcy` with the PRE-bankruptcy snapshot
handover/audits/run_codex_tb_15_ship_audit_r2.sh:238:`src/runtime/autopsy_capsule.rs::derive_autopsies_for_bankruptcy`
handover/audits/run_codex_tb_15_ship_audit_r2.sh:248:`src/bin/generate_markov_capsule.rs` + `src/runtime/markov_capsule.rs::write_markov_capsule`.
handover/audits/run_codex_tb_15_ship_audit_r2.sh:292:`src/runtime/autopsy_capsule.rs::write_autopsy_capsule` +
handover/audits/run_codex_tb_15_ship_audit_r2.sh:293:`derive_autopsies_for_bankruptcy` signatures. Verify: (a) NEITHER
handover/audits/run_codex_tb_15_ship_audit_r2.sh:308:write_markov_capsule unchanged; (c) for the actual emitted capsule
handover/audits/run_codex_tb_15_ship_audit_r2.sh:330:**RQ1 — derive_autopsies_for_bankruptcy idempotency under apply_one
handover/audits/run_codex_tb_15_ship_audit_r2.sh:332:`derive_autopsies_for_bankruptcy` with the same inputs. If apply_one is
handover/audits/run_codex_tb_15_ship_audit_r2.sh:341:calls `derive_autopsies_for_bankruptcy(&q.economic_state_t, bk, ...)`
handover/audits/run_codex_tb_15_ship_audit_r2.sh:377:`write_markov_capsule`. What happens if `--cas-dir` is provided but
handover/audits/run_gemini_tb_15_ship_audit_r3.py:54:  - Same bug in `write_autopsy_capsule` and `derive_autopsies_for_bankruptcy`.
handover/audits/run_gemini_tb_15_ship_audit_r3.py:60:2. **NEW helpers**: `restore_markov_capsule_from_cas_bytes(bytes)` and `restore_autopsy_capsule_from_cas_bytes(bytes)` canonical-decode + re-derive capsule_id/sha256 from `Cid::from_content(&bytes)`. Symmetric round-trip.
handover/audits/run_gemini_tb_15_ship_audit_r3.py:61:3. **NEW `BankruptcyAutopsyDerivation` struct**: replaces the old `(AgentAutopsyCapsule, Vec<u8>)` tuple in `derive_autopsies_for_bankruptcy`'s return type. Carries `capsule + private_bytes + stored_capsule_bytes`. Apply_one writes `stored_capsule_bytes` (the EXACT bytes whose sha256 == capsule_id).
handover/audits/run_gemini_tb_15_ship_audit_r3.py:62:4. **NEW round-trip tests**: `write_markov_capsule_cas_resolvable_by_capsule_id` + `write_bankruptcy_autopsies_to_cas_round_trip` (extended) — both assert `cas.get(&cap.capsule_id)` succeeds + retrieved bytes' sha256 == capsule_id + restore round-trip works.
handover/audits/run_gemini_tb_15_ship_audit_r3.py:77:2. **Restore helpers correctness**: do `restore_markov_capsule_from_cas_bytes` and `restore_autopsy_capsule_from_cas_bytes` correctly reverse the write operation? What happens if a consumer canonical-decodes the CAS bytes WITHOUT calling restore? They'd get a struct with capsule_id=Cid::default() — is that an invariant violation that would propagate silently?
handover/audits/run_gemini_tb_15_ship_audit_r3.py:79:3. **BankruptcyAutopsyDerivation struct API**: is the new return type ergonomic? Does it correctly carry the bytes to apply_one without recomputation? Is it idempotent?
handover/audits/run_gemini_tb_15_ship_audit_r3.py:81:4. **Cross-cut to TB-11**: TB-11 `write_evidence_capsule` likely has the same bug. Should TB-15 R3 ship include a TB-11 fix, or carry-forward as OBS?
handover/audits/run_codex_tb_15_ship_audit.sh:164:`derive_autopsies_for_bankruptcy` with the PRE-bankruptcy snapshot
handover/audits/run_codex_tb_15_ship_audit.sh:174:`src/runtime/autopsy_capsule.rs::derive_autopsies_for_bankruptcy`
handover/audits/run_codex_tb_15_ship_audit.sh:184:`src/bin/generate_markov_capsule.rs` + `src/runtime/markov_capsule.rs::write_markov_capsule`.
handover/audits/run_codex_tb_15_ship_audit.sh:228:`src/runtime/autopsy_capsule.rs::write_autopsy_capsule` +
handover/audits/run_codex_tb_15_ship_audit.sh:229:`derive_autopsies_for_bankruptcy` signatures. Verify: (a) NEITHER
handover/audits/run_codex_tb_15_ship_audit.sh:244:write_markov_capsule unchanged; (c) for the actual emitted capsule
handover/audits/run_codex_tb_15_ship_audit.sh:266:**RQ1 — derive_autopsies_for_bankruptcy idempotency under apply_one
handover/audits/run_codex_tb_15_ship_audit.sh:268:`derive_autopsies_for_bankruptcy` with the same inputs. If apply_one is
handover/audits/run_codex_tb_15_ship_audit.sh:277:calls `derive_autopsies_for_bankruptcy(&q.economic_state_t, bk, ...)`
handover/audits/run_codex_tb_15_ship_audit.sh:313:`write_markov_capsule`. What happens if `--cas-dir` is provided but
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw:19:- **TB15-CAS-ID VETO**: same self-CID/content-CID mismatch in `write_autopsy_capsule` and `derive_autopsies_for_bankruptcy`.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw:25:1. **Q3 VETO closure (write_markov_capsule)**: writer rewritten so capsule_id == sha256(stored_bytes). Specifically: build capsule with capsule_id=Cid::default()/sha256=Hash::ZERO; canonical_encode → those are the bytes stored in CAS; capsule_id = Cid::from_content(stored_bytes); populate the in-memory struct fields after CAS write. The on-CAS bytes have capsule_id+sha256 zeroed; the in-memory struct returned to caller has them populated. cas.get(&capsule.capsule_id) succeeds. NEW helper `restore_markov_capsule_from_cas_bytes(bytes)` canonical-decodes + re-derives capsule_id/sha256 from `Cid::from_content(&bytes)`.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw:27:2. **TB15-CAS-ID VETO closure (write_autopsy_capsule + derive_autopsies_for_bankruptcy)**: same writer pattern fix. derive_autopsies_for_bankruptcy now returns `Vec<BankruptcyAutopsyDerivation>` (struct with `capsule + private_bytes + stored_capsule_bytes` fields) instead of `Vec<(AgentAutopsyCapsule, Vec<u8>)>`. The dispatch arm reads `d.capsule.capsule_id`; apply_one writes `d.stored_capsule_bytes` (the EXACT bytes whose sha256 == capsule_id). NEW helper `restore_autopsy_capsule_from_cas_bytes(bytes)`.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw:30:   - `runtime::markov_capsule::tests::write_markov_capsule_cas_resolvable_by_capsule_id` — asserts `cas.get(&cap.capsule_id)` succeeds + retrieved bytes' sha256 == capsule_id + restore round-trip works.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw:31:   - `runtime::autopsy_capsule::tests::write_bankruptcy_autopsies_to_cas_round_trip` — extended with same R3 contract.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw:44:- **Q3 closure check**: read `src/runtime/markov_capsule.rs::write_markov_capsule`. Confirm:
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw:50:  (f) `restore_markov_capsule_from_cas_bytes` correctly reverses the operation
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw:53:- **TB15-CAS-ID closure check**: same checks for `write_autopsy_capsule` AND `derive_autopsies_for_bankruptcy` AND `write_bankruptcy_autopsies_to_cas`. Confirm `BankruptcyAutopsyDerivation` correctly carries `stored_capsule_bytes` from derive to CAS write.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw:55:- **Cross-cut concern**: TB-11 `write_evidence_capsule` likely has the SAME bug (predates TB-15; same prelim/final pattern). Is this a new VETO that should block TB-15 ship, or is it acceptable to OBS-defer because no production consumer of EvidenceCapsule via cap.capsule_id exists yet?
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw:268:`derive_autopsies_for_bankruptcy` with the PRE-bankruptcy snapshot
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw:278:`src/runtime/autopsy_capsule.rs::derive_autopsies_for_bankruptcy`
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw:288:`src/bin/generate_markov_capsule.rs` + `src/runtime/markov_capsule.rs::write_markov_capsule`.
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw:332:`src/runtime/autopsy_capsule.rs::write_autopsy_capsule` +
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw:333:`derive_autopsies_for_bankruptcy` signatures. Verify: (a) NEITHER
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw:348:write_markov_capsule unchanged; (c) for the actual emitted capsule
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw:370:**RQ1 — derive_autopsies_for_bankruptcy idempotency under apply_one
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw:372:`derive_autopsies_for_bankruptcy` with the same inputs. If apply_one is
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw:381:calls `derive_autopsies_for_bankruptcy(&q.economic_state_t, bk, ...)`
handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md.raw:417:`write_markov_capsule`. What happens if `--cas-dir` is provided but
src/runtime/adapter.rs:467:/// via `evidence_capsule::write_evidence_capsule`).
src/runtime/adapter.rs:474:///   1. Build EvidenceCapsule via `evidence_capsule::write_evidence_capsule`.
src/runtime/evidence_capsule.rs:208:pub fn write_evidence_capsule(
src/runtime/evidence_capsule.rs:337:    fn write_evidence_capsule_to_cas_round_trip() {
src/runtime/evidence_capsule.rs:355:        let capsule = write_evidence_capsule(
src/runtime/evidence_capsule.rs:400:    fn write_evidence_capsule_deterministic_capsule_id() {
src/runtime/evidence_capsule.rs:418:            write_evidence_capsule(
src/runtime/evidence_capsule.rs:438:            write_evidence_capsule(
src/runtime/markov_capsule.rs:10://! Generator surface: `write_markov_capsule(...)` (CAS-emitting) + binary
src/runtime/markov_capsule.rs:227:pub fn write_markov_capsule(
src/runtime/markov_capsule.rs:333:/// identical to what `write_markov_capsule` returned at write time.
src/runtime/markov_capsule.rs:335:/// Invariant: for any capsule written by `write_markov_capsule`,
src/runtime/markov_capsule.rs:336:/// `restore_markov_capsule_from_cas_bytes(cas.get(&cap.capsule_id)?) ==
src/runtime/markov_capsule.rs:339:pub fn restore_markov_capsule_from_cas_bytes(
src/runtime/markov_capsule.rs:489:    /// TB-15 Atom 5 — write_markov_capsule writes 2 CAS objects
src/runtime/markov_capsule.rs:493:    fn write_markov_capsule_to_cas_round_trip() {
src/runtime/markov_capsule.rs:506:        let cap = write_markov_capsule(
src/runtime/markov_capsule.rs:536:    /// TB-15 Atom 5 — write_markov_capsule deterministic (same inputs →
src/runtime/markov_capsule.rs:539:    fn write_markov_capsule_deterministic_capsule_id() {
src/runtime/markov_capsule.rs:546:            write_markov_capsule(
src/runtime/markov_capsule.rs:578:        let cap_1 = write_markov_capsule(
src/runtime/markov_capsule.rs:593:        let cap_2 = write_markov_capsule(
src/runtime/markov_capsule.rs:675:    fn write_markov_capsule_cas_resolvable_by_capsule_id() {
src/runtime/markov_capsule.rs:681:        let cap = write_markov_capsule(
src/runtime/markov_capsule.rs:711:        let restored = restore_markov_capsule_from_cas_bytes(&retrieved)
src/runtime/autopsy_capsule.rs:253:pub fn write_autopsy_capsule(
src/runtime/autopsy_capsule.rs:306:    // write_markov_capsule. Store the bytes whose sha256 equals
src/runtime/autopsy_capsule.rs:333:/// to `restore_markov_capsule_from_cas_bytes`. Caller supplies the
src/runtime/autopsy_capsule.rs:482:// TB-15 Atom 3 — `derive_autopsies_for_bankruptcy` (PURE deterministic helper)
src/runtime/autopsy_capsule.rs:508:/// identical `Vec<BankruptcyAutopsyDerivation>` (same Cids, same bytes,
src/runtime/autopsy_capsule.rs:510:pub fn derive_autopsies_for_bankruptcy(
src/runtime/autopsy_capsule.rs:515:) -> Vec<BankruptcyAutopsyDerivation> {
src/runtime/autopsy_capsule.rs:570:        out.push(BankruptcyAutopsyDerivation {
src/runtime/autopsy_capsule.rs:580:/// the deterministic outputs of `derive_autopsies_for_bankruptcy`. The
src/runtime/autopsy_capsule.rs:586:pub struct BankruptcyAutopsyDerivation {
src/runtime/autopsy_capsule.rs:595:/// `derive_autopsies_for_bankruptcy` (same inputs → same Cids as the
src/runtime/autopsy_capsule.rs:600:pub fn write_bankruptcy_autopsies_to_cas(
src/runtime/autopsy_capsule.rs:608:    let derived = derive_autopsies_for_bankruptcy(
src/runtime/autopsy_capsule.rs:689:    fn write_autopsy_capsule_to_cas_round_trip() {
src/runtime/autopsy_capsule.rs:699:        let cap = write_autopsy_capsule(
src/runtime/autopsy_capsule.rs:743:    fn write_autopsy_capsule_deterministic_capsule_id() {
src/runtime/autopsy_capsule.rs:753:            write_autopsy_capsule(
src/runtime/autopsy_capsule.rs:796:    // Atom 3 — derive_autopsies_for_bankruptcy tests
src/runtime/autopsy_capsule.rs:808:        // not strictly needed by derive_autopsies_for_bankruptcy itself.
src/runtime/autopsy_capsule.rs:853:    /// TB-15 Atom 3 — derive_autopsies_for_bankruptcy: per-staker
src/runtime/autopsy_capsule.rs:864:        let derived = derive_autopsies_for_bankruptcy(&econ, &bk, /*round=*/ 5, /*t=*/ 100);
src/runtime/autopsy_capsule.rs:900:    /// TB-15 Atom 3 — derive_autopsies_for_bankruptcy: same inputs →
src/runtime/autopsy_capsule.rs:912:        let a = derive_autopsies_for_bankruptcy(&econ, &bk, 3, 50);
src/runtime/autopsy_capsule.rs:913:        let b = derive_autopsies_for_bankruptcy(&econ, &bk, 3, 50);
src/runtime/autopsy_capsule.rs:933:    /// TB-15 Atom 3 — derive_autopsies_for_bankruptcy: no stakers on
src/runtime/autopsy_capsule.rs:947:        let derived = derive_autopsies_for_bankruptcy(&econ, &bk, 0, 0);
src/runtime/autopsy_capsule.rs:1076:    /// TB-15 Atom 3 — write_bankruptcy_autopsies_to_cas: writes
src/runtime/autopsy_capsule.rs:1080:    fn write_bankruptcy_autopsies_to_cas_round_trip() {
src/runtime/autopsy_capsule.rs:1096:        let cids = write_bankruptcy_autopsies_to_cas(
src/runtime/autopsy_capsule.rs:1109:        let derived = derive_autopsies_for_bankruptcy(&econ, &bk, 7, 42);
src/bin/generate_markov_capsule.rs:43:    try_deep_history_read_with_override_check, write_markov_capsule, MarkovGenError, ObsId,
src/bin/generate_markov_capsule.rs:352:        write_markov_capsule(
src/bin/generate_markov_capsule.rs:370:            other => format!("write_markov_capsule: {other}"),
src/state/sequencer.rs:1367:            // using the same `derive_autopsies_for_bankruptcy` helper
src/state/sequencer.rs:1376:                    crate::runtime::autopsy_capsule::derive_autopsies_for_bankruptcy(
src/state/sequencer.rs:3099:                let _ = crate::runtime::autopsy_capsule::write_bankruptcy_autopsies_to_cas(
tests/tb_15_halt_triggers.rs:137:// write_autopsy_capsule signature MUST NOT accept any &mut PredicateRegistry
tests/fc_alignment_conformance.rs:547:/// FC1-N32 (TB-15 Atom 2): write_autopsy_capsule writer surface exists +
tests/fc_alignment_conformance.rs:551:fn fc1_n32_write_autopsy_capsule_witness() {
tests/fc_alignment_conformance.rs:557:    use turingosv4::runtime::autopsy_capsule::{write_autopsy_capsule, LossReasonClass};
tests/fc_alignment_conformance.rs:563:    let cap = write_autopsy_capsule(
tests/fc_alignment_conformance.rs:584:/// FC1-N33 (TB-15 Atom 3): derive_autopsies_for_bankruptcy is a pure
tests/fc_alignment_conformance.rs:590:    use turingosv4::runtime::autopsy_capsule::derive_autopsies_for_bankruptcy;
tests/fc_alignment_conformance.rs:608:    let a = derive_autopsies_for_bankruptcy(&econ, &bk, 1, 5);
tests/fc_alignment_conformance.rs:609:    let b = derive_autopsies_for_bankruptcy(&econ, &bk, 1, 5);
handover/tests/REAL_LLM_COMPREHENSIVE_AUDIT_FROM_TAPE_DESIGN_2026-05-04.md:239:14. For each `TaskBankruptcyTx` row, `derive_autopsies_for_bankruptcy` re-run with the row's pre-snapshot returns Cids identical to those stored in `agent_autopsies_t[event_id]`.
handover/tests/REAL_LLM_COMPREHENSIVE_AUDIT_FROM_TAPE_DESIGN_2026-05-04.md:346:        derive_autopsies_for_bankruptcy} (TB-15),
handover/tests/REAL_LLM_COMPREHENSIVE_AUDIT_FROM_TAPE_DESIGN_2026-05-04.md:399:  derive_autopsies_for_bankruptcy ONLY; LLM self-reports are ignored
handover/evidence/tb_11_epistemic_exhaust_smoke_2026-05-02/README.md:46:| SG-11.1 capsule on hard-fail | ✓ pass         | `runtime/evidence_capsule::write_evidence_capsule` writer test (Atom 3 U)            |
handover/evidence/tb_11_epistemic_exhaust_smoke_2026-05-02/README.md:111:   on MAX_TX exhausted, call `evidence_capsule::write_evidence_capsule`
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:56:Same fix applied to `write_autopsy_capsule` and `derive_autopsies_for_bankruptcy` / `write_bankruptcy_autopsies_to_cas` (TB15-CAS-ID VETO closure).
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:60:- `runtime::markov_capsule::tests::write_markov_capsule_cas_resolvable_by_capsule_id` — asserts `cas.get(&cap.capsule_id)` succeeds + retrieved bytes' sha256 equals capsule_id + restore round-trip works.
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:61:- `runtime::autopsy_capsule::tests::write_bankruptcy_autopsies_to_cas_round_trip` — extended with same R3 contract assertions for autopsy capsules.
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:73:| **TB15-CAS-ID** (autopsy CAS-cid mismatch) | Codex R2 NEW VETO | n/a | **FIXED**: same writer pattern fix applied to write_autopsy_capsule + derive_autopsies_for_bankruptcy |
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:79:- **OBS-TB-11-CAS-ID** (cross-cut, NEW R3): TB-11 `write_evidence_capsule` has the SAME CAS-cid mismatch bug. Not blocking TB-15 ship (no production consumer of EvidenceCapsule via cap.capsule_id yet) but should be fixed in TB-11 follow-up.
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:90:- Source files: `src/runtime/markov_capsule.rs` (write_markov_capsule + restore_markov_capsule_from_cas_bytes), `src/runtime/autopsy_capsule.rs` (write_autopsy_capsule + restore_autopsy_capsule_from_cas_bytes + BankruptcyAutopsyDerivation)
handover/evidence/tb_15_markov_capsule_2026-05-03/README.md:70:| SG-15.1 | Failed/losing agent gets private AutopsyCapsule | GREEN — TaskBankruptcyTx dispatch arm Step 3.5 emits per-staker capsule via `derive_autopsies_for_bankruptcy` (verified by `derive_autopsies_emits_one_per_staker_target_only`) |
handover/evidence/tb_15_markov_capsule_2026-05-03/README.md:93:Re-running the binary with identical inputs yields the same `capsule_id`. Verified by `write_markov_capsule_deterministic_capsule_id` unit test.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:64:| SG-15.1 | Failed/losing agent gets private AutopsyCapsule | TaskBankruptcyTx Step 3.5 → per-staker capsule via `derive_autopsies_for_bankruptcy` |
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:82:- `src/runtime/autopsy_capsule.rs` (Atoms 2 + 3 + 4): `LossReasonClass` enum (8 variants) + `AgentAutopsyCapsule` struct + `format_public_summary` + `write_autopsy_capsule` + `derive_autopsies_for_bankruptcy` (PURE) + `write_bankruptcy_autopsies_to_cas` + `cluster_autopsies` + `TypicalErrorSummary` + `AutopsyWriteError`. 15 in-module tests.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:83:- `src/runtime/markov_capsule.rs` (Atom 5): `ObsId` newtype + `MarkovEvidenceCapsule` struct + `with_constitution_hash` constructor + `try_deep_history_read_with_override_check` gate + `override_set_from_env` + `write_markov_capsule` + `scan_unresolved_obs` + `sha256_of_file` + `MarkovGenError`. 8 in-module tests.
handover/ai-direct/LATEST.md:56:- **NEW** `src/runtime/autopsy_capsule.rs` (Atoms 2 + 3 + 4): `LossReasonClass` (8 variants) + `AgentAutopsyCapsule` + `format_public_summary` + `write_autopsy_capsule` + `derive_autopsies_for_bankruptcy` (PURE; consumed by both dispatch + apply_one) + `write_bankruptcy_autopsies_to_cas` + `cluster_autopsies` + `TypicalErrorSummary`. 15 in-module tests.
handover/ai-direct/LATEST.md:57:- **NEW** `src/runtime/markov_capsule.rs` (Atom 5): `ObsId` + `MarkovEvidenceCapsule` + `with_constitution_hash` + `try_deep_history_read_with_override_check` (default-deny gate) + `override_set_from_env` + `write_markov_capsule` + `scan_unresolved_obs` + `sha256_of_file` + `MarkovGenError`. 8 in-module tests.
handover/ai-direct/LATEST.md:580:          (a) evaluator binary MAX_TX exhausted → write_evidence_capsule
handover/ai-direct/LATEST.md:761:   (evaluator binary on MAX_TX → write_evidence_capsule +
handover/ai-direct/LATEST.md:946:                  (call write_evidence_capsule + tb11_emit_terminal_summary_for_run on
handover/tracer_bullets/TB-15_charter_2026-05-03.md:14:- **Flowchart 1 (runtime)**: `FC1-N32` = `write_autopsy_capsule` writer (CAS-resident; sequencer-emitted; private-by-default). `FC1-N33` = TaskBankruptcyTx dispatch arm autopsy emission (per-agent loss anchor for FR-15.1 first wire-in).
handover/tracer_bullets/TB-15_charter_2026-05-03.md:56:            write_autopsy_capsule signature does not accept any
handover/tracer_bullets/TB-15_charter_2026-05-03.md:74:    FC1-N32 = write_autopsy_capsule (CAS writer; AuditOnly default;
handover/tracer_bullets/TB-15_charter_2026-05-03.md:190:      - write_autopsy_capsule(...) → returns capsule
handover/tracer_bullets/TB-15_charter_2026-05-03.md:299:**MOD**: `src/state/sequencer.rs` — TaskBankruptcyTx dispatch arm: for each agent in stakes_t whose stake's task_id matches the bankrupted task, call `write_autopsy_capsule` + push `capsule_id` into `agent_autopsies_t[event_id]`.
handover/tracer_bullets/TB-11_charter_2026-05-02.md:396:pub fn write_evidence_capsule(
handover/directives/2026-05-03_TB12_NODE_EXPOSURE_INDEX_ARCHITECT_RULING.md:75:>   evaluator binary MAX_TX exhausted -> write_evidence_capsule + tb11_emit_terminal_summary_for_run
handover/directives/2026-05-03_TB12_NODE_EXPOSURE_INDEX_ARCHITECT_RULING.md:373:> evaluator MAX_TX exhausted -> write_evidence_capsule
handover/tracer_bullets/TB-12_charter_2026-05-03.md:116:| `EvidenceCapsule` + `write_evidence_capsule` writer | `src/runtime/evidence_capsule.rs` | shipped TB-11 (Atom 3) |
handover/tracer_bullets/TB-12_charter_2026-05-03.md:126:    accumulated counters, call `evidence_capsule::write_evidence_capsule`,
handover/tracer_bullets/TB-12_charter_2026-05-03.md:222:     - Call `evidence_capsule::write_evidence_capsule(...)` to obtain
handover/tracer_bullets/TB_LOG.tsv:36:# 2026-05-02 TB-11 SHIPPED — "P3 carry-forward + P4 primary + P6 carry-forward: Epistemic Exhaust & Capital Liberation per architect ruling 2026-05-02 §6.2". Class 3 risk class (capital-mover via TaskExpire refund + 3 system-emitted variants); recursive self-audit PASS — 4/4 clauses GREEN (Constitutional preservation / Replay-deterministic / Conservation / Negative-truth-completeness); 9/11 ship gates GREEN + 2/11 ⚠ deferred (G3 lean_market 6 subcommands wire-up + G4 evaluator forced-exhaust real-LLM smoke); 7/7 architect SG-11.1..7 ship gates structurally satisfied (capsule-on-hard-fail / RunExhausted in L4 / TaskExpire refunds / CTF preserved / dashboard regenerates / raw evidence shielded / future-Short anchor). Kernel-level architectural core SHIPPED (Atoms 1-5 of 8): TypedTx schemas + sequencer dispatch + emit_system_tx commands + EvidenceCapsule CAS writer + audit_dashboard §12. Atom 6 = smoke-evidence README composing pre-existing TB-13 PREVIEW zeta corpus (132 attempts/0 OMEGA/500_000 stuck) + 5 deterministic TB-11 integration tests as the proof. Atom 7 = recursive self-audit doc with explicit external Codex+Gemini deferral rationale (§8: TaskExpire is the sole net-new economic mutator and structurally mirrors TB-8 dual-audited FinalizeReward; capsule writer is purely additive; architect ruling itself was the architectural review). Atom 8 = this LATEST.md update + TB_LOG row + ship commit. Architectural deltas: (a) src/state/typed_tx.rs +TaskBankruptcyTx struct +TaskBankruptcySigningPayload +DOMAIN_SYSTEM_TASK_BANKRUPTCY domain prefix +RunExhaustedTx type alias for TerminalSummaryTx in failure path +TerminalSummaryTx additive bump (+parent_state_root +solver_agent +evidence_capsule_cid; no production rows pre-TB-11; safe per feedback_no_retroactive_evidence_rewrite) +TaskExpireTx additive bump (+sponsor_agent +escrow_tx_id +reason); +ExpireReason +BankruptcyReason +ExhaustionReason +CapsulePrivacyPolicy enums; golden digest constants rotated. (b) src/state/q_state.rs EconomicState 9→10 sub-fields with +runs_t: RunsIndex; +RunSummaryEntry struct; +TaskMarketState enum {Open, Expired, Bankrupt, Finalized}; TaskMarketEntry additive (+state +bankruptcy_at_logical_t +opened_at_logical_t). (c) src/state/sequencer.rs +3 dispatch arms (TaskExpire refund / TerminalSummary RunsIndex anchor / TaskBankruptcy state-flip); +3 SystemEmitCommand variants; +3 state-root domain helpers (TASK_EXPIRE_DOMAIN_V1 / TERMINAL_SUMMARY_DOMAIN_V1 / TASK_BANKRUPTCY_DOMAIN_V1); submit_agent_tx ingress fail-closed extended; verify_emitted_system_tx_signature extended. (d) src/runtime/evidence_capsule.rs NEW module — EvidenceCapsule schema (16 fields incl. 5 architect-mandated counts) + ExhaustionCounts helper + format_public_summary + write_evidence_capsule writer (3 CAS objects per capsule: raw log + manifest + capsule); CapsulePrivacyPolicy::AuditOnly default. (e) src/bottom_white/cas/schema.rs +3 ObjectType variants (EvidenceCapsule / EvidenceManifest / CompressedRunLog). (f) src/bottom_white/ledger/{system_keypair,transition_ledger}.rs +CanonicalMessage::TaskBankruptcySigning +sign_task_bankruptcy helper +TxKind::TaskBankruptcy=10. (g) src/runtime/adapter.rs +tb11_emit_terminal_summary_for_run +tb11_emit_expire_for_eligible runtime helpers (mirror tb8 pattern; thin wrappers over emit_system_tx with policy-gated eligibility scan). (h) src/bin/audit_dashboard.rs +§12 with 3 sub-tables (Exhausted runs / Expired tasks / Bankrupt tasks) + total-refund aggregation + privacy-mandate footer. cargo test --workspace = 747/0/150 (+16 net vs TB-10 baseline 731). Trust Root rehashed: 11 entries (typed_tx / q_state / sequencer / state/mod / cas/schema / system_keypair / transition_ledger / runtime/mod / run_summary / monetary_invariant / adapter / audit_dashboard) + 1 NEW entry (runtime/evidence_capsule.rs). Forward-binding: TB-12 NodeMarket Position Index now has the on-chain anchor it needs (TaskBankruptcyTx for future Short / NO settlement). Open follow-ups (carry-forward, NOT ship blockers): evaluator binary integration (call write_evidence_capsule + tb11_emit_terminal_summary_for_run on MAX_TX exhausted); lean_market binary tick + view-bankruptcy subcommands; real-LLM zeta-regularization smoke producing single tar.gz; (optional) Codex + Gemini external audits available on request via existing audit script harness.
handover/tracer_bullets/TB_LOG.tsv:39:# 2026-05-03 TB-12 SHIPPED — "P3 carry-forward + P4 primary: Node Exposure Index per architect 2026-05-03 ruling §3 + §10". Class 3 envelope (Atom 1 = Class 2 schema; Atom 2 = Class 3 dispatch wire; Atom 6 = Class 3 dual audit Codex + Gemini both ✓). Self-audit PASS; Codex CHALLENGE × 2 (Q4 doc-drift on 4-vs-5-holding total_supply; Q5 legacy CPMM in src/prediction_market.rs predates TB-12) — both resolved as documentation/scope clarifications via recursive audit §10 + OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md (TB-13 prerequisite); Gemini round-1 PASS / high conviction / PROCEED to SHIP (gemini-2.5-pro, 896k char prompt, 48.2s API; 8/8 audit questions PASS). User-architect ultrathink verified architect §9 strict alignment before SHIP — found 4 SG-12.x test-name drifts (SG-12.5 / 12.6 / 12.7 / 12.8 names didn't exactly match architect's "passes" strings); all 4 fixed BEFORE SHIP per feedback_no_retroactive_evidence_rewrite (NOT post-ship patch): SG-12.5 renamed to sg_12_5_replay_reconstructs_node_positions; SG-12.6 ADDED sg_12_6_dashboard_view_positions_works inside src/bin/audit_dashboard.rs #[cfg(test)] mod tb12_render_tests with 4 cases + forbidden-token grep; SG-12.7 renamed to sg_12_7_no_market_trading_variants_introduced; SG-12.8 ADDED sg_12_8_no_node_market_entry_as_canonical_state at integration-test level (q_state.rs unit alias kept as defense-in-depth). 8/8 SG-12.x by architect §9.3 EXACT names PASS. cargo test --workspace = 759/0/150 (+12 net vs TB-11 ship 747; +28 vs TB-10 ship 731). Architectural deltas: NEW src/state/typed_tx.rs (NodePosition struct + PositionSide enum + PositionKind enum); EXTEND src/state/q_state.rs (NodePositionsIndex flat shape + EconomicState 10→11 sub-fields with +node_positions_t serde-default); EXTEND src/state/sequencer.rs (WorkTx + ChallengeTx accept-arm pure-additive NodePosition write — no balances_t / stakes_t / challenge_cases_t / total_supply mutation; CR-12.1 + CR-12.2 verified); EXTEND src/economy/monetary_invariant.rs (NodePosition NOT in 4-holding total_supply_micro; structural — assert_total_ctf_conserved unchanged); EXTEND src/bin/audit_dashboard.rs (§13 Node exposure records + render_section_13 pure-fn helper for SG-12.6 unit-testability + LABEL DISCIPLINE "exposure records" NOT "Open market balances" per architect §8 Atom 4); EXTEND experiments/minif2f_v4/src/bin/evaluator.rs (TB-11 G3/G4 carry-forward — 4 EvidenceCapsule counters + write_evidence_capsule + tb11_emit_terminal_summary_for_run wired at MAX_TX exhausted exit point); EXTEND experiments/minif2f_v4/src/bin/lean_market.rs (3 new subcommands: tick POLICY PREVIEW MODE — actual emission deferred to system_keypair persistence + view-bankruptcy read-only listing + view-positions read-only NodePosition browser); REHASH genesis_payload.toml trust_root for src/state/sequencer.rs + src/state/typed_tx.rs + src/state/q_state.rs + src/state/mod.rs + src/bin/audit_dashboard.rs + experiments/minif2f_v4/src/bin/evaluator.rs (lean_market.rs outside trust root by TB-10 precedent). Halting triggers (architect §7) NONE triggered: CTF conservation passes (Gemini Q1+Q4); position-tx field derivation exact (position_fields_derived_from_source_tx_exactly test PASS); NodePosition NOT counted as Coin (4-holding model unchanged); replay deterministic (sg_12_5 PASS); no Codex/Gemini VETO. Production claim: "TB-12 establishes a flat, immutable, non-monetary NodePositionsIndex anchoring WorkTx.stake → FirstLong + ChallengeTx.stake → ChallengeShort exposure records on canonical L4-derived QState. NodePosition is exposure record NOT active position balance per architect §10 critical insight; TB-13 CompleteSet introduces real YES/NO claims; TB-14 PriceIndex computes price from long/short interest; TB-16 controlled-arena introduces P&L. Charter Q6 (ii.5) sync mode honored — dual audit complete + user-architect ultrathink verdict before SHIP." Open follow-ups (carry-forward, NOT ship blockers): TB-13 prerequisite quarantine of src/prediction_market.rs legacy f64 CPMM (OBS-tracked); system_keypair persistence + reattachment factory for lean_market tick on-chain emission; real-LLM zeta-regularization rerun (Atom 0.5(a) wired the call site; manual user-driven session post-ship).
handover/tracer_bullets/TB_LOG.tsv:45:# 2026-05-03 TB-15 SHIPPED — "P4 Information Loom (primary) + P5 MetaTape v0 prep: Lamarckian Autopsy + Markov EvidenceCapsule per architect 2026-05-02 + 2026-05-03 §6 verbatim spec (FR-15.1..6 + CR-15.1..6 + SG-15.1..8 + 6 halt triggers + forbidden list)". Class 2 envelope (self-audit; AgentVisibleProjection unchanged; only one new sequencer dispatch hook — TaskBankruptcyTx Step 3.5 + apply_one Stage 3.5). 7-atom plan (charter + halt fixture + AgentAutopsyCapsule schema/writer + AutopsyIndex/TaskBankruptcyTx wire-in + cluster_autopsies + MarkovEvidenceCapsule schema/generator + dashboard §15/first capsule/SHIP) all green. Auto-resolutions per feedback_architect_deviation_stance: A) single trigger site = TaskBankruptcyTx (SlashTx wires when RSP-3.2 ships; FinalizeReward loser branch wires when RSP-4 ships); B) EconomicState 12→13 sub-fields with +agent_autopsies_t serde-default; C) privacy default = AuditOnly (re-use TB-11 CapsulePrivacyPolicy); D) clustering threshold N=3 per architect §3.2.3 verbatim; E) LossReasonClass 8 variants incl Other(String); F) Markov override = env TURINGOS_MARKOV_OVERRIDE=1; G) OBS_RESOLUTIONS_INDEX_TB15 DEFERRED out of TB-15 (TB-13 audit residue, not in architect §6 spec); H) Class 2 self-audit (no AgentVisibleProjection mod). cargo test --workspace = 870/0/150 PASS (+67 net vs TB-14 ship 803). All 6 halt-triggers GREEN: #1 raw_logs_not_in_general_read_view (Atom 3 file-scan AgentVisibleProjection) + #2 markov_capsule_references_constitution_hash (Atom 5 SG-15.7) + #3 autopsy_does_not_mutate_predicates (Atom 2 file-scan no &mut PredicateRegistry/ToolRegistry/RiskPolicyRegistry/register_*/unregister_*/patch_*; CR-15.3 + SG-15.8) + #4 private_detail_not_in_other_agent_view (Atom 3 file-scan AutopsyIndex value type Vec<Cid> not raw bytes; SG-15.2) + #5 typical_error_clustering_uses_summary_only (Atom 4 cluster_autopsies serialization scan no private_detail_cid byte run; CR-15.2 + SG-15.5) + #6 deep_history_read_without_override_fails (Atom 5 try_deep_history_read_with_override_check default-deny; SG-15.4 + FR-15.5). Architectural deltas: NEW src/runtime/autopsy_capsule.rs (LossReasonClass enum + AgentAutopsyCapsule struct + format_public_summary + write_autopsy_capsule + derive_autopsies_for_bankruptcy PURE deterministic helper + write_bankruptcy_autopsies_to_cas + cluster_autopsies + TypicalErrorSummary + AutopsyWriteError + 15 unit tests); NEW src/runtime/markov_capsule.rs (ObsId newtype + MarkovEvidenceCapsule struct + with_constitution_hash + try_deep_history_read_with_override_check + override_set_from_env + write_markov_capsule + scan_unresolved_obs + sha256_of_file + MarkovGenError + 8 unit tests); NEW src/bin/generate_markov_capsule.rs (CLI binary with --tb-id/--out-dir/--constitution-path/--cas-dir/--prev-cid-hex/--alignment-dir/--no-cas args + TURINGOS_MARKOV_OVERRIDE env); NEW tests/tb_15_halt_triggers.rs (6 fixtures); MOD src/state/typed_tx.rs (+RiskRuleId opaque newtype); MOD src/bottom_white/cas/schema.rs (+4 ObjectType variants AgentAutopsyCapsule + AutopsyPrivateDetail + MarkovEvidenceCapsule + NextSessionContext); MOD src/state/q_state.rs (+AutopsyIndex(BTreeMap<EventId, Vec<Cid>>) newtype + agent_autopsies_t 13th sub-field on EconomicState serde-default; sub-field count assertion 12→13); MOD src/state/sequencer.rs (TaskBankruptcyTx dispatch arm Step 3.5 PURE deterministic Cid derivation + apply_one Stage 3.5 CAS write side-effect via deterministic helper; NO predicate registry mutation; NO AgentVisibleProjection mod; NO change to monetary invariants); MOD src/runtime/mod.rs (+pub mod autopsy_capsule + pub mod markov_capsule); MOD src/bin/audit_dashboard.rs (+render_section_15 pure render with AUTOPSY IS PRIVATE banner + autopsy_event_counts + latest_markov_capsule_cid_hex fields on DashboardReport + read_latest_markov_pointer helper + 4 SG-15.6 dashboard render tests); MOD 4 test fixtures sub-field count 12→13 (economic_state_reconstruct/q_state_reconstruct/six_axioms_alignment); MOD tests/fc_alignment_conformance.rs (+4 TB-15 witnesses FC1-N32 write_autopsy_capsule + FC1-N33 derive_autopsies_for_bankruptcy + FC2-N30 cluster_autopsies + FC3-N43 MarkovEvidenceCapsule); REHASH genesis_payload.toml trust_root for 6 modified files (src/runtime/mod.rs ×2 hops Atom2→Atom5; src/state/q_state.rs Atom3; src/state/typed_tx.rs Atom2; src/state/sequencer.rs Atom3; src/bottom_white/cas/schema.rs Atom2; tests/fc_alignment_conformance.rs Atom6). First Markov capsule emitted (capsule_id b244f16a1f3bd532d041a40fe39b2b7e7cc12fb58e18b61aedd76a8010eeb1b6) with constitution_hash eec695459c71fbef3685583485deb431fe3b561657b2f285b7c5e7e220e59e03; evidence at handover/evidence/tb_15_markov_capsule_2026-05-03/. Production claim: "TB-15 establishes Lamarckian Autopsy + Markov EvidenceCapsule substrate. AgentAutopsyCapsule (per-agent, per-event, AuditOnly) records loss/bankruptcy events derived deterministically from ChainTape evidence — NEVER LLM self-narration. agent_autopsies_t lives sequencer-side (NOT projected to AgentVisibleProjection per CR-15.1 + halt-trigger #1). TypicalErrorBroadcast clustering at N≥3 emits public_summary text + Cids only — NEVER private_detail_cid bytes. MarkovEvidenceCapsule binds constitution_hash + L4 + L4.E + CAS roots + previous capsule + typical_errors + unresolved_obs as next-session bootstrap default; deeper history requires TURINGOS_MARKOV_OVERRIDE=1. CR-15.3/15.4 STRUCTURALLY ENFORCED via writer signature + halt-trigger #3 file-scan. Constitutional alignment: Art.II.1 + Art.III.1 + Art.III.2 + Art.III.3 + Art.III.4 + Art.V.1 + Art.0.2." Open follow-ups (carry-forward NOT ship blockers): multi-site autopsy wire-in (SlashLoss/ChallengeUnsuccessful/VerifierBondLost — when RSP-3.2/RSP-4 ship); L4/L4.E/CAS root chain-readers in Markov generator (currently zero placeholders); CAS-walking dashboard §15 (currently empty autopsy_event_counts); InitAI agent-side honoring of Markov default (P5 v1); OBS_RESOLUTIONS_INDEX_TB15 explicitly deferred per charter §7-G.
handover/tracer_bullets/TB_LOG.tsv:46:TB-15	shipped	2026-05-03	2026-05-03	P4 Information Loom (primary — failure clusterer + read-view shielding + signal layer per Art.II.1 + Art.III.1 + Art.III.2 + Art.III.3 + Art.III.4) + P5 MetaTape v0 prep (Markov capsule = ArchitectAI substrate; NO automatic mutation per Art.V.1; CR-15.3 + CR-15.4 STRUCTURALLY ENFORCED)	P4-Exit1 (single-Agent failure → local error only; AgentAutopsyCapsule lives in CAS + agent_autopsies_t NOT in AgentVisibleProjection per CR-15.1 + halt-trigger #1) + P4-Exit2 (multi-agent same-class failure → typical-error broadcast via cluster_autopsies threshold=3) + P4-Exit3 (broadcast is abstract rule not raw failure log; public_summary text only) + P5-Exit1 (ArchitectAI cannot directly modify predicate registry — STRUCTURALLY ENFORCED; suggested_policy_patch is Option<Cid> opaque pointer never auto-applied) + P5-Exit2 (JudgeAI veto-only CODIFIED; no JudgeAI code in TB-15 — P5 v1 territory)	P4-Forbidden:全量广播 ledger (raw_logs_not_in_general_read_view halt-trigger #1 GREEN) + P4-Forbidden:全量广播 rejected logs (private_detail_not_in_other_agent_view halt-trigger #4 GREEN) + P5-Kill:ArchitectAI directly modifies predicate registry (autopsy_does_not_mutate_predicates halt-trigger #3 GREEN) + halt-trigger #2 markov_capsule_references_constitution_hash GREEN + halt-trigger #5 typical_error_clustering_uses_summary_only GREEN + halt-trigger #6 deep_history_read_without_override_fails GREEN — ALL 6 halt-triggers GREEN	[870/0/150 PASS @ TB-15 ship commit; +67 net vs TB-14 ship 803] +15 src/runtime/autopsy_capsule TB-15 unit tests (6 schema/writer + 4 derive + 5 cluster) +8 src/runtime/markov_capsule TB-15 unit tests +6 tests/tb_15_halt_triggers.rs (architect §6.6 forbidden + §6.5 SG halts) +4 src/bin/audit_dashboard SG-15.6 §15 render tests +4 tests/fc_alignment_conformance TB-15 FC witnesses (FC1-N32 + FC1-N33 + FC2-N30 + FC3-N43)	architect §6.5 SG-15.1..8 100% (8/8 by EXACT names): Failed/losing agent gets private AutopsyCapsule / Raw private details do not enter other Agent read view / Latest Markov capsule can bootstrap next session / Deep-history read without override fails / Typical error broadcast uses summary not raw log / Dashboard can regenerate capsule summary from ChainTape + CAS / Markov capsule references constitution hash / Autopsy does not mutate predicates/tools automatically — ALL PASS	Art.0.2 (Tape Canonical: AgentAutopsyCapsule + MarkovEvidenceCapsule canonical bytes ARE the CAS objects referenced by capsule_id; replay-deterministic per derive_autopsies_for_bankruptcy PURE helper used by both dispatch arm and apply_one hook) + Art.II.1 (typical-error broadcast pattern via cluster_autopsies N=3 threshold) + Art.II.2.1 (entropy not affected — public_summary low-info text + Cid pointers only; private detail AuditOnly) + Art.III.1 (raw failure log shielding via private_detail_cid AuditOnly default) + Art.III.2 (read-view scoping — agent_autopsies_t sequencer-side NOT in AgentVisibleProjection per CR-15.1 + halt-trigger #1) + Art.III.3 (broadcast pollution prevention — public_summary only when typical N≥3 per CR-15.2) + Art.III.4 (Goodhart shield — autopsy cannot mutate predicates/permissions per CR-15.3 + CR-15.4 + SG-15.8 STRUCTURALLY ENFORCED via writer signature + halt-trigger #3 file-scan) + Art.V.1 (三权分立 — ArchitectAI may suggest via suggested_policy_patch Option<Cid>; JudgeAI/VetoAI veto-only; CR-15.3 + CR-15.4) + WP-§5.L3 (CAS object schema — 4 new ObjectType variants AgentAutopsyCapsule + AutopsyPrivateDetail + MarkovEvidenceCapsule + NextSessionContext) + WP-§5.L4 (canonical state transition — AutopsyIndex 13th sub-field on EconomicState; serde-default backward-compat) + feedback_architect_deviation_stance (Auto-resolution G: explicit DEFER stance on OBS_RESOLUTIONS_INDEX_TB15 — not flagging for ratification) + feedback_dual_audit (Class 2 = self-audit OK per hybrid-by-risk-class) + feedback_kolmogorov_compression (architect §6 spec verbatim archived in charter §0; no distillation) + feedback_smoke_before_batch (binary smoke before claiming generator works — emitted capsule_id b244f16a... matches cargo test --no-cas mode) + feedback_workspace_test_canonical (cargo test --workspace = 870/0/150; canonical reporting) + feedback_iteration_cap_24h (Class 2 24h per atom; no extension needed; pure scaffolding TB) + feedback_no_retroactive_evidence_rewrite (TB-15 wire-in is going-forward only; pre-TB-15 TaskBankruptcy events do NOT get retroactive autopsy emission)	<pending Atom 6 ship commit>

codex
I found the worktree is already dirty with R3-related edits and audit artifacts, so I’ll treat those as existing user/generated changes and only inspect them. The first broad search was noisy because prior audit transcripts contain the same symbols; I’m narrowing to the source and tests now.
exec
/bin/bash -lc "nl -ba src/runtime/markov_capsule.rs | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-15 Atom 5 — `MarkovEvidenceCapsule` schema + writer + default-deny
     2	//! deep-history read gate (architect §6.2 + DECISION_LAMARCKIAN §1).
     3	//!
     4	//! End-of-TB rollup binding `constitution_hash` + L4 root + L4.E root +
     5	//! CAS root + previous capsule + typical_errors + unresolved_obs +
     6	//! `next_session_context_cid`. Default next-session bootstrap source per
     7	//! FR-15.4. Deeper history requires `TURINGOS_MARKOV_OVERRIDE=1`
     8	//! (FR-15.5 + halt-trigger #6).
     9	//!
    10	//! Generator surface: `write_markov_capsule(...)` (CAS-emitting) + binary
    11	//! `src/bin/generate_markov_capsule.rs` (CLI wrapper).
    12	//!
    13	//! TRACE_MATRIX FC3-N43 + Art. 0.2 (Tape Canonical: capsule canonical
    14	//! bytes are themselves the CAS object referenced by `capsule_id`) +
    15	//! CR-15.5 (capsules are evidence compression, not hidden source of
    16	//! truth — every field is derivable from the chain + CAS) +
    17	//! CR-15.6 (Markov default prevents context poisoning).
    18	
    19	use serde::{Deserialize, Serialize};
    20	
    21	use crate::bottom_white::cas::schema::{Cid, ObjectType};
    22	use crate::bottom_white::cas::store::CasStore;
    23	use crate::bottom_white::ledger::transition_ledger::canonical_encode;
    24	use crate::runtime::autopsy_capsule::TypicalErrorSummary;
    25	use crate::state::q_state::Hash;
    26	
    27	/// TRACE_MATRIX TB-15 (architect §6.2): unresolved OBS identifier.
    28	/// Opaque string newtype carrying the relative path of an `OBS_*.md`
    29	/// file under `handover/alignment/` (the project's de-facto observation
    30	/// register). Cross-session continuity hint per CR-15.5.
    31	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    32	pub struct ObsId(pub String);
    33	
    34	/// TRACE_MATRIX TB-15 (architect §6.2 + FR-15.4 + FR-15.5): end-of-TB
    35	/// rollup. Default next-session bootstrap source: future agent reads
    36	/// `constitution.md` (referenced by `constitution_hash`) + this capsule
    37	/// (referenced by `capsule_id`) — no deeper history without
    38	/// `TURINGOS_MARKOV_OVERRIDE=1`.
    39	///
    40	/// **CR-15.5**: every field is derivable from the chain + CAS at
    41	/// generation time (constitution_hash from constitution.md, l4_root
    42	/// from L4 chain head, l4e_root from L4.E chain head, cas_root from CAS
    43	/// metadata digest, typical_errors from cluster_autopsies(...) over
    44	/// CAS-resident capsules, unresolved_obs from `handover/alignment/OBS_*.md`).
    45	/// Capsule is evidence compression, not hidden source of truth.
    46	///
    47	/// **Markov chain**: `previous_capsule_cid` points to the prior capsule
    48	/// (None for genesis Markov capsule); next-session context defaults to
    49	/// {constitution + this capsule}. Deeper history (older capsules; L4
    50	/// rows pre-dating `previous_capsule_cid`'s `l4_root`) requires
    51	/// `TURINGOS_MARKOV_OVERRIDE=1` per CR-15.6.
    52	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    53	pub struct MarkovEvidenceCapsule {
    54	    /// CAS Cid of this capsule's canonical bytes (with `capsule_id`
    55	    /// zeroed during the hash). Computed by writer.
    56	    pub capsule_id: Cid,
    57	
    58	    /// Cid of the previous Markov capsule in the chain. `None` for the
    59	    /// first ever capsule (genesis Markov).
    60	    pub previous_capsule_cid: Option<Cid>,
    61	
    62	    /// SHA-256 of `constitution.md` bytes at generation time.
    63	    /// SG-15.7: capsule must reference constitution hash.
    64	    pub constitution_hash: Hash,
    65	
    66	    /// TRACE_MATRIX TB-15 R2 closure (Codex R1 Q8/RQ7 + Gemini R1 Q7):
    67	    /// canonical SHA-256 hashes of the four constitution flowcharts
    68	    /// (Flowchart 1a / 1b / 2 / 3) per
    69	    /// `handover/alignment/TRACE_FLOWCHART_MATRIX.md` §2 (architect
    70	    /// 2026-05-02 directive — flowcharts elevated to SHA-anchored
    71	    /// architectural contracts). Empty Vec for backward-compat with
    72	    /// pre-R2 chain snapshots; populated by the generator binary in
    73	    /// canonical-order (1a, 1b, 2, 3). Closes the literal SG-15.7 spec
    74	    /// "constitution hash AND flowchart hashes" requirement.
    75	    #[serde(default)]
    76	    pub flowchart_hashes: Vec<Hash>,
    77	
    78	    /// L4 transition_ledger root at generation time.
    79	    pub l4_root: Hash,
    80	    /// L4.E rejection_evidence ledger root at generation time.
    81	    pub l4e_root: Hash,
    82	    /// CAS metadata root (digest of CAS object metadata) at generation
    83	    /// time. Strictly informational; CR-15.5 — capsule does not
    84	    /// duplicate CAS contents.
    85	    pub cas_root: Hash,
    86	
    87	    /// Typical-error rollup at generation time (TB-15 Atom 4 surface).
    88	    pub typical_errors: Vec<TypicalErrorSummary>,
    89	    /// Open observation register entries at generation time (relative
    90	    /// paths under `handover/alignment/OBS_*.md`).
    91	    pub unresolved_obs: Vec<ObsId>,
    92	
    93	    /// CAS Cid of a JSON blob describing the next session's default
    94	    /// boot context (`{constitution_hash, latest_markov_cid, boot_seq}`).
    95	    /// FR-15.4 + halt-trigger #6 entry point.
    96	    pub next_session_context_cid: Cid,
    97	
    98	    /// SHA-256 of this capsule's canonical bytes. Defense-in-depth
    99	    /// duplicate of `capsule_id`.
   100	    pub sha256: Hash,
   101	    /// Logical time at generation (sequencer or generator-supplied).
   102	    pub created_at_logical_t: u64,
   103	    /// Free-form TB tag — e.g. `"TB-15"`. Strictly informational.
   104	    pub tb_tag: String,
   105	}
   106	
   107	impl Default for MarkovEvidenceCapsule {
   108	    fn default() -> Self {
   109	        Self {
   110	            capsule_id: Cid::default(),
   111	            previous_capsule_cid: None,
   112	            constitution_hash: Hash::ZERO,
   113	            flowchart_hashes: Vec::new(),
   114	            l4_root: Hash::ZERO,
   115	            l4e_root: Hash::ZERO,
   116	            cas_root: Hash::ZERO,
   117	            typical_errors: Vec::new(),
   118	            unresolved_obs: Vec::new(),
   119	            next_session_context_cid: Cid::default(),
   120	            sha256: Hash::ZERO,
   121	            created_at_logical_t: 0,
   122	            tb_tag: String::new(),
   123	        }
   124	    }
   125	}
   126	
   127	impl MarkovEvidenceCapsule {
   128	    /// TRACE_MATRIX TB-15 Atom 5 — convenience constructor used by
   129	    /// halt-trigger #2 to pin `constitution_hash` to a known value
   130	    /// (verifies SG-15.7 from a fixture without spinning up the
   131	    /// generator binary).
   132	    pub fn with_constitution_hash(hash_bytes: [u8; 32]) -> Self {
   133	        Self {
   134	            constitution_hash: Hash(hash_bytes),
   135	            ..Self::default()
   136	        }
   137	    }
   138	}
   139	
   140	// ────────────────────────────────────────────────────────────────────────────
   141	// TB-15 Atom 5 — Writer + default-deny gate
   142	// ────────────────────────────────────────────────────────────────────────────
   143	
   144	/// TRACE_MATRIX TB-15 Atom 5 — writer / generator error taxonomy.
   145	#[derive(Debug)]
   146	pub enum MarkovGenError {
   147	    /// Default-deny: caller asked for deep-history read without
   148	    /// `TURINGOS_MARKOV_OVERRIDE=1`. SG-15.4 + halt-trigger #6.
   149	    DeepHistoryReadDenied,
   150	    Cas(crate::bottom_white::cas::store::CasError),
   151	    Encode(String),
   152	    Io(std::io::Error),
   153	    InternalLockPoisoned,
   154	}
   155	
   156	impl std::fmt::Display for MarkovGenError {
   157	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   158	        match self {
   159	            Self::DeepHistoryReadDenied => write!(
   160	                f,
   161	                "deep-history read denied: set TURINGOS_MARKOV_OVERRIDE=1 to enable"
   162	            ),
   163	            Self::Cas(e) => write!(f, "cas: {e}"),
   164	            Self::Encode(s) => write!(f, "encode: {s}"),
   165	            Self::Io(e) => write!(f, "io: {e}"),
   166	            Self::InternalLockPoisoned => write!(f, "internal lock poisoned"),
   167	        }
   168	    }
   169	}
   170	impl std::error::Error for MarkovGenError {}
   171	
   172	impl From<crate::bottom_white::cas::store::CasError> for MarkovGenError {
   173	    fn from(e: crate::bottom_white::cas::store::CasError) -> Self {
   174	        Self::Cas(e)
   175	    }
   176	}
   177	impl From<std::io::Error> for MarkovGenError {
   178	    fn from(e: std::io::Error) -> Self {
   179	        Self::Io(e)
   180	    }
   181	}
   182	
   183	/// TRACE_MATRIX TB-15 Atom 5 (architect FR-15.5 + halt-trigger #6):
   184	/// default-deny gate for deep-history reads. Returns
   185	/// `Err(DeepHistoryReadDenied)` unless `override_set` is true.
   186	///
   187	/// In production, `override_set` is wired to env
   188	/// `TURINGOS_MARKOV_OVERRIDE=1` by the binary; this helper isolates the
   189	/// decision so it can be exercised by halt-trigger #6 without process-
   190	/// global env mutation (env mutation racy under cargo's parallel test
   191	/// runner per `feedback_env_var_test_lock`).
   192	pub fn try_deep_history_read_with_override_check(
   193	    override_set: bool,
   194	) -> Result<(), MarkovGenError> {
   195	    if override_set {
   196	        Ok(())
   197	    } else {
   198	        Err(MarkovGenError::DeepHistoryReadDenied)
   199	    }
   200	}
   201	
   202	/// TRACE_MATRIX TB-15 Atom 5 (architect FR-15.5): bool wrapper that
   203	/// reads `TURINGOS_MARKOV_OVERRIDE` from process env. Used by the
   204	/// generator binary; isolated here so the decision is auditable.
   205	pub fn override_set_from_env() -> bool {
   206	    std::env::var("TURINGOS_MARKOV_OVERRIDE")
   207	        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
   208	        .unwrap_or(false)
   209	}
   210	
   211	/// TRACE_MATRIX TB-15 Atom 5: write a `MarkovEvidenceCapsule` to CAS.
   212	/// Flow:
   213	///
   214	/// 1. Build the next-session-context JSON blob → write to CAS as
   215	///    `ObjectType::NextSessionContext`. Cid is `next_session_context_cid`.
   216	/// 2. Build the capsule struct with `capsule_id = Cid::default()` +
   217	///    `sha256 = Hash::ZERO`. Canonical-encode → sha256 → that's the
   218	///    eventual `capsule_id`.
   219	/// 3. Re-create the struct with `capsule_id` filled in + write to CAS
   220	///    as `ObjectType::MarkovEvidenceCapsule`.

exec
/bin/bash -lc 'find /home/zephryj/projects/turingosv4 -name AGENTS.md -print' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc "nl -ba src/runtime/markov_capsule.rs | sed -n '220,380p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   220	///    as `ObjectType::MarkovEvidenceCapsule`.
   221	///
   222	/// **CR-15.5**: caller supplies `constitution_hash` / `l4_root` /
   223	/// `l4e_root` / `cas_root` / `typical_errors` / `unresolved_obs` —
   224	/// each derived from the chain + CAS at generation time. Writer does
   225	/// NOT mint new ground truth.
   226	#[allow(clippy::too_many_arguments)]
   227	pub fn write_markov_capsule(
   228	    cas: &std::sync::Arc<std::sync::RwLock<CasStore>>,
   229	    previous_capsule_cid: Option<Cid>,
   230	    constitution_hash: Hash,
   231	    flowchart_hashes: Vec<Hash>,
   232	    l4_root: Hash,
   233	    l4e_root: Hash,
   234	    cas_root: Hash,
   235	    typical_errors: Vec<TypicalErrorSummary>,
   236	    unresolved_obs: Vec<ObsId>,
   237	    tb_tag: String,
   238	    creator_str: &str,
   239	    created_at_logical_t: u64,
   240	) -> Result<MarkovEvidenceCapsule, MarkovGenError> {
   241	    let mut cas_w = cas
   242	        .write()
   243	        .map_err(|_| MarkovGenError::InternalLockPoisoned)?;
   244	
   245	    // Step 1: build + write next_session_context JSON.
   246	    let next_session_json = serde_json::json!({
   247	        "schema_version": "v1/next_session_context",
   248	        "constitution_hash_hex": hex(&constitution_hash.0),
   249	        "flowchart_hashes_hex": flowchart_hashes.iter().map(|h| hex(&h.0)).collect::<Vec<_>>(),
   250	        "previous_markov_cid_hex": previous_capsule_cid.map(|c| c.hex()),
   251	        "tb_tag": tb_tag,
   252	        "boot_seq": [
   253	            "1. read constitution.md (verify sha256 == constitution_hash)",
   254	            "2. read TRACE_FLOWCHART_MATRIX.md (verify each flowchart sha256 == flowchart_hashes[i])",
   255	            "3. read CAS<this_markov_capsule_cid>",
   256	            "4. read CAS<previous_markov_capsule_cid> (if present)",
   257	            "5. DEFAULT-DENY deeper history; set TURINGOS_MARKOV_OVERRIDE=1 to override (audit-only)"
   258	        ],
   259	    });
   260	    let next_session_bytes = serde_json::to_vec(&next_session_json)
   261	        .map_err(|e| MarkovGenError::Encode(format!("next_session_context: {e}")))?;
   262	    let next_session_context_cid = cas_w.put(
   263	        &next_session_bytes,
   264	        ObjectType::NextSessionContext,
   265	        creator_str,
   266	        created_at_logical_t,
   267	        Some("v1/next_session_context".into()),
   268	    )?;
   269	
   270	    // Step 2: build capsule with capsule_id = 0 + sha256 = 0.
   271	    let mut capsule = MarkovEvidenceCapsule {
   272	        capsule_id: Cid::default(),
   273	        previous_capsule_cid,
   274	        constitution_hash,
   275	        flowchart_hashes,
   276	        l4_root,
   277	        l4e_root,
   278	        cas_root,
   279	        typical_errors,
   280	        unresolved_obs,
   281	        next_session_context_cid,
   282	        sha256: Hash::ZERO,
   283	        created_at_logical_t,
   284	        tb_tag,
   285	    };
   286	
   287	    // R3 closure (Codex R2 VETO Q3 + TB15-CAS-ID): the CAS-stored bytes
   288	    // MUST be the bytes whose sha256 equals `capsule_id`, otherwise
   289	    // `cas.get(&capsule.capsule_id)` fails. PRIOR BUG: we computed
   290	    // `capsule_id = sha256(prelim_bytes)` then stored DIFFERENT
   291	    // `final_bytes` (with capsule_id+sha256 populated) — those final
   292	    // bytes had a different sha256, so the CAS index keyed under that
   293	    // different sha256, and the published `capsule_id` was unresolvable.
   294	    //
   295	    // FIX: store the prelim bytes (with capsule_id=Cid::default,
   296	    // sha256=Hash::ZERO). The in-memory `MarkovEvidenceCapsule`
   297	    // returned to the caller has populated capsule_id/sha256 for
   298	    // ergonomic use; on-disk CAS bytes have these fields zeroed.
   299	    // Consumers reading from CAS:
   300	    //   1. Fetch bytes via cas.get(&capsule.capsule_id) → SUCCEEDS
   301	    //      (capsule_id == sha256(stored_bytes))
   302	    //   2. canonical_decode(bytes) → struct with capsule_id=default
   303	    //   3. Re-derive capsule_id := sha256(bytes); set field; set sha256
   304	    //      identical to the in-memory writer-returned struct.
   305	    let stored_bytes = canonical_encode(&capsule)
   306	        .map_err(|e| MarkovGenError::Encode(format!("capsule canonical encode: {e:?}")))?;
   307	    let cid = Cid::from_content(&stored_bytes);
   308	    let cas_returned_cid = cas_w.put(
   309	        &stored_bytes,
   310	        ObjectType::MarkovEvidenceCapsule,
   311	        creator_str,
   312	        created_at_logical_t,
   313	        Some("v1/markov_evidence_capsule".into()),
   314	    )?;
   315	    debug_assert_eq!(
   316	        cas_returned_cid, cid,
   317	        "CAS-returned cid must equal sha256(stored_bytes); CasStore::put contract"
   318	    );
   319	    // Populate the struct fields from the derived cid. The struct
   320	    // returned to the caller is the ergonomic view; on-CAS bytes have
   321	    // capsule_id+sha256 zeroed so cas.get(&cid) is retrievable.
   322	    capsule.capsule_id = cid;
   323	    capsule.sha256 = Hash(cid.0);
   324	
   325	    Ok(capsule)
   326	}
   327	
   328	/// TRACE_MATRIX TB-15 R3 closure (Codex R2 VETO Q3): rebuild a
   329	/// `MarkovEvidenceCapsule` from CAS-resident bytes. Caller supplies the
   330	/// bytes returned by `cas.get(&capsule_id)`. This helper canonical-
   331	/// decodes them and re-derives `capsule_id` + `sha256` from
   332	/// `Cid::from_content(&bytes)`, returning the ergonomic in-memory view
   333	/// identical to what `write_markov_capsule` returned at write time.
   334	///
   335	/// Invariant: for any capsule written by `write_markov_capsule`,
   336	/// `restore_markov_capsule_from_cas_bytes(cas.get(&cap.capsule_id)?) ==
   337	///  cap` (modulo non-stored field ordering — both have populated
   338	/// capsule_id/sha256).
   339	pub fn restore_markov_capsule_from_cas_bytes(
   340	    bytes: &[u8],
   341	) -> Result<MarkovEvidenceCapsule, MarkovGenError> {
   342	    use crate::bottom_white::ledger::transition_ledger::canonical_decode;
   343	    let mut cap: MarkovEvidenceCapsule = canonical_decode(bytes)
   344	        .map_err(|e| MarkovGenError::Encode(format!("capsule decode: {e:?}")))?;
   345	    let cid = Cid::from_content(bytes);
   346	    cap.capsule_id = cid;
   347	    cap.sha256 = Hash(cid.0);
   348	    Ok(cap)
   349	}
   350	
   351	/// TRACE_MATRIX TB-15 Atom 5: scan `<repo>/handover/alignment/OBS_*.md`
   352	/// for unresolved-observation file paths. Pure read; returns sorted
   353	/// `Vec<ObsId>` (BTreeSet ordering) for replay-determinism. CR-15.5 —
   354	/// capsule references existing files, never mints new ones.
   355	pub fn scan_unresolved_obs(alignment_dir: &std::path::Path) -> Result<Vec<ObsId>, MarkovGenError> {
   356	    use std::collections::BTreeSet;
   357	    let mut out: BTreeSet<String> = BTreeSet::new();
   358	    if !alignment_dir.is_dir() {
   359	        return Ok(Vec::new());
   360	    }
   361	    for entry in std::fs::read_dir(alignment_dir)? {
   362	        let entry = entry?;
   363	        let name = entry.file_name();
   364	        let name_str = name.to_string_lossy();
   365	        if name_str.starts_with("OBS_") && name_str.ends_with(".md") {
   366	            out.insert(format!("handover/alignment/{}", name_str));
   367	        }
   368	    }
   369	    Ok(out.into_iter().map(ObsId).collect())
   370	}
   371	
   372	/// TRACE_MATRIX TB-15 Atom 5: deterministic SHA-256 of constitution.md
   373	/// at the given path. Public so binaries + tests can compute it
   374	/// uniformly. CR-15.5 + SG-15.7.
   375	pub fn sha256_of_file(path: &std::path::Path) -> Result<Hash, MarkovGenError> {
   376	    use sha2::{Digest, Sha256};
   377	    let bytes = std::fs::read(path)?;
   378	    let mut h = Sha256::new();
   379	    h.update(&bytes);
   380	    let digest: [u8; 32] = h.finalize().into();

exec
/bin/bash -lc "nl -ba src/runtime/markov_capsule.rs | sed -n '430,640p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   430	        for i in 0..32 {
   431	            bytes[i] = u8::from_str_radix(&hex_str[i * 2..i * 2 + 2], 16).map_err(|e| {
   432	                MarkovGenError::Encode(format!("flowchart `{}` byte {}: {}", label, i, e))
   433	            })?;
   434	        }
   435	        hashes.push(Hash(bytes));
   436	    }
   437	    if hashes.len() != 4 {
   438	        return Err(MarkovGenError::Encode(format!(
   439	            "expected 4 flowchart hashes; got {}",
   440	            hashes.len()
   441	        )));
   442	    }
   443	    Ok(hashes)
   444	}
   445	
   446	/// Hex helper for next-session JSON formatting (32-byte hashes).
   447	fn hex(bytes: &[u8; 32]) -> String {
   448	    let mut s = String::with_capacity(64);
   449	    for b in bytes {
   450	        s.push_str(&format!("{:02x}", b));
   451	    }
   452	    s
   453	}
   454	
   455	#[cfg(test)]
   456	mod tests {
   457	    use super::*;
   458	    use sha2::{Digest, Sha256};
   459	
   460	    /// TB-15 Atom 5 — capsule default round-trips through canonical bytes.
   461	    #[test]
   462	    fn markov_capsule_default_round_trip() {
   463	        use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
   464	        let c = MarkovEvidenceCapsule::default();
   465	        let bytes = canonical_encode(&c).expect("encode");
   466	        let back: MarkovEvidenceCapsule = canonical_decode(&bytes).expect("decode");
   467	        assert_eq!(c, back);
   468	    }
   469	
   470	    /// TB-15 Atom 5 — with_constitution_hash plumbs the hash through.
   471	    #[test]
   472	    fn with_constitution_hash_sets_field() {
   473	        let hash = [0xABu8; 32];
   474	        let c = MarkovEvidenceCapsule::with_constitution_hash(hash);
   475	        assert_eq!(c.constitution_hash.0, hash);
   476	    }
   477	
   478	    /// TB-15 Atom 5 — try_deep_history_read_with_override_check:
   479	    /// false → Err(DeepHistoryReadDenied); true → Ok(()).
   480	    #[test]
   481	    fn deep_history_default_deny_works() {
   482	        match try_deep_history_read_with_override_check(false) {
   483	            Err(MarkovGenError::DeepHistoryReadDenied) => {}
   484	            other => panic!("expected DeepHistoryReadDenied; got {other:?}"),
   485	        }
   486	        assert!(try_deep_history_read_with_override_check(true).is_ok());
   487	    }
   488	
   489	    /// TB-15 Atom 5 — write_markov_capsule writes 2 CAS objects
   490	    /// (next_session_context + capsule), and capsule_id is the
   491	    /// canonical sha256 (with field zeroed for prelim encode).
   492	    #[test]
   493	    fn write_markov_capsule_to_cas_round_trip() {
   494	        use std::sync::{Arc, RwLock};
   495	        use tempfile::TempDir;
   496	
   497	        let tmp = TempDir::new().expect("tempdir");
   498	        let cas = Arc::new(RwLock::new(
   499	            CasStore::open(tmp.path()).expect("cas"),
   500	        ));
   501	
   502	        let mut h = Sha256::new();
   503	        h.update(b"fake constitution body");
   504	        let constitution_hash: [u8; 32] = h.finalize().into();
   505	
   506	        let cap = write_markov_capsule(
   507	            &cas,
   508	            None, // genesis Markov
   509	            Hash(constitution_hash),
   510	            vec![Hash([0xAAu8; 32]), Hash([0xBBu8; 32]), Hash([0xCCu8; 32]), Hash([0xDDu8; 32])],
   511	            Hash([0x01u8; 32]),
   512	            Hash([0x02u8; 32]),
   513	            Hash([0x03u8; 32]),
   514	            vec![],
   515	            vec![ObsId("handover/alignment/OBS_X.md".into())],
   516	            "TB-15".into(),
   517	            "tb15-test-writer",
   518	            7,
   519	        )
   520	        .expect("writer succeeds");
   521	
   522	        assert_ne!(cap.capsule_id, Cid::default());
   523	        assert_eq!(cap.capsule_id.0, cap.sha256.0);
   524	        assert_ne!(cap.next_session_context_cid, Cid::default());
   525	        assert_eq!(cap.constitution_hash.0, constitution_hash);
   526	        assert_eq!(cap.flowchart_hashes.len(), 4);
   527	
   528	        let cas_r = cas.read().expect("cas read");
   529	        assert_eq!(
   530	            cas_r.len(),
   531	            2,
   532	            "writer puts 2 CAS objects: next_session_context + capsule"
   533	        );
   534	    }
   535	
   536	    /// TB-15 Atom 5 — write_markov_capsule deterministic (same inputs →
   537	    /// same capsule_id + same next_session_context_cid).
   538	    #[test]
   539	    fn write_markov_capsule_deterministic_capsule_id() {
   540	        use std::sync::{Arc, RwLock};
   541	        use tempfile::TempDir;
   542	
   543	        let mk = || {
   544	            let tmp = TempDir::new().unwrap();
   545	            let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).unwrap()));
   546	            write_markov_capsule(
   547	                &cas,
   548	                Some(Cid([0xCDu8; 32])),
   549	                Hash([0x10u8; 32]),
   550	                vec![],
   551	                Hash([0x20u8; 32]),
   552	                Hash([0x30u8; 32]),
   553	                Hash([0x40u8; 32]),
   554	                vec![],
   555	                vec![],
   556	                "TB-15-det".into(),
   557	                "writer",
   558	                42,
   559	            )
   560	            .expect("writer")
   561	        };
   562	        let a = mk();
   563	        let b = mk();
   564	        assert_eq!(a.capsule_id, b.capsule_id);
   565	        assert_eq!(a.next_session_context_cid, b.next_session_context_cid);
   566	    }
   567	
   568	    /// TB-15 Atom 5 — Markov chain: each capsule references the prior
   569	    /// via `previous_capsule_cid`.
   570	    #[test]
   571	    fn markov_chain_links_via_previous_capsule_cid() {
   572	        use std::sync::{Arc, RwLock};
   573	        use tempfile::TempDir;
   574	
   575	        let tmp = TempDir::new().unwrap();
   576	        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).unwrap()));
   577	
   578	        let cap_1 = write_markov_capsule(
   579	            &cas,
   580	            None,
   581	            Hash([0x01u8; 32]),
   582	            vec![],
   583	            Hash([0x10u8; 32]),
   584	            Hash([0x20u8; 32]),
   585	            Hash([0x30u8; 32]),
   586	            vec![],
   587	            vec![],
   588	            "TB-15-prev".into(),
   589	            "writer",
   590	            1,
   591	        )
   592	        .expect("cap 1");
   593	        let cap_2 = write_markov_capsule(
   594	            &cas,
   595	            Some(cap_1.capsule_id),
   596	            Hash([0x01u8; 32]),
   597	            vec![],
   598	            Hash([0x11u8; 32]), // L4 advanced
   599	            Hash([0x20u8; 32]),
   600	            Hash([0x30u8; 32]),
   601	            vec![],
   602	            vec![],
   603	            "TB-15-next".into(),
   604	            "writer",
   605	            2,
   606	        )
   607	        .expect("cap 2");
   608	
   609	        assert_eq!(cap_2.previous_capsule_cid, Some(cap_1.capsule_id));
   610	        assert_ne!(cap_1.capsule_id, cap_2.capsule_id);
   611	    }
   612	
   613	    /// TB-15 Atom 5 — sha256_of_file matches manual sha256.
   614	    #[test]
   615	    fn sha256_of_file_matches_manual() {
   616	        use std::io::Write;
   617	        use tempfile::NamedTempFile;
   618	        let mut f = NamedTempFile::new().unwrap();
   619	        let body = b"test constitution body";
   620	        f.write_all(body).unwrap();
   621	        let path = f.path().to_path_buf();
   622	        let computed = sha256_of_file(&path).expect("sha256");
   623	        let mut h = Sha256::new();
   624	        h.update(body);
   625	        let manual: [u8; 32] = h.finalize().into();
   626	        assert_eq!(computed.0, manual);
   627	    }
   628	
   629	    /// TB-15 Atom 5 — scan_unresolved_obs picks up OBS_*.md files only,
   630	    /// in sorted order.
   631	    #[test]
   632	    fn scan_unresolved_obs_filters_and_sorts() {
   633	        use std::fs;
   634	        use tempfile::TempDir;
   635	        let tmp = TempDir::new().unwrap();
   636	        let p = tmp.path();
   637	        fs::write(p.join("OBS_zeta.md"), "z").unwrap();
   638	        fs::write(p.join("OBS_alpha.md"), "a").unwrap();
   639	        fs::write(p.join("not_an_obs.md"), "x").unwrap();
   640	        fs::write(p.join("OBS_beta.txt"), "b").unwrap(); // wrong extension

 succeeded in 391ms:
exec
/bin/bash -lc "nl -ba src/runtime/markov_capsule.rs | sed -n '640,760p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   640	        fs::write(p.join("OBS_beta.txt"), "b").unwrap(); // wrong extension
   641	
   642	        let obs = scan_unresolved_obs(p).expect("scan");
   643	        assert_eq!(obs.len(), 2);
   644	        // Sorted (BTreeSet semantics).
   645	        assert!(obs[0].0.ends_with("OBS_alpha.md"));
   646	        assert!(obs[1].0.ends_with("OBS_zeta.md"));
   647	    }
   648	
   649	    /// TB-15 R2 closure (Codex Q8/RQ7 + Gemini Q7): read_flowchart_hashes_from_matrix
   650	    /// returns 4 hashes in canonical order (1a, 1b, 2, 3).
   651	    #[test]
   652	    fn read_flowchart_hashes_from_real_matrix() {
   653	        let manifest = env!("CARGO_MANIFEST_DIR");
   654	        let path = std::path::PathBuf::from(manifest)
   655	            .join("handover/alignment/TRACE_FLOWCHART_MATRIX.md");
   656	        let hashes = read_flowchart_hashes_from_matrix(&path).expect("matrix parse");
   657	        assert_eq!(hashes.len(), 4, "exactly 4 canonical flowchart hashes");
   658	        // Architect 2026-05-02 §2 verbatim canonical hashes.
   659	        let expected_1a = "a474c6b9ded766504a4f644a4a1b3c545316d418f0250f36ec692fcdf98f09f5";
   660	        let expected_1b = "b822717b10332a2d8e789ba6af96fd4da4ff43a74afab679d1b82add9c32b64d";
   661	        let expected_2 = "6a4bc9195bafd55bde968fd445cdd2926d6906a7f6a2b38071d4774a7f0de333";
   662	        let expected_3 = "c159413984d0c6c5daa06605fea3a86a2ad4ab9c4284d0d20e0e525bf03aa9cd";
   663	        assert_eq!(hex(&hashes[0].0), expected_1a, "FC 1a");
   664	        assert_eq!(hex(&hashes[1].0), expected_1b, "FC 1b");
   665	        assert_eq!(hex(&hashes[2].0), expected_2, "FC 2");
   666	        assert_eq!(hex(&hashes[3].0), expected_3, "FC 3");
   667	    }
   668	
   669	    /// TB-15 R3 closure (Codex R2 VETO Q3): cas.get(&cap.capsule_id)
   670	    /// MUST succeed; the retrieved bytes MUST canonical-decode +
   671	    /// restore to the in-memory capsule. This closes the SG-15.3
   672	    /// "next session can bootstrap from latest Markov capsule" gate
   673	    /// at the structural level.
   674	    #[test]
   675	    fn write_markov_capsule_cas_resolvable_by_capsule_id() {
   676	        use std::sync::{Arc, RwLock};
   677	        use tempfile::TempDir;
   678	
   679	        let tmp = TempDir::new().expect("tempdir");
   680	        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
   681	        let cap = write_markov_capsule(
   682	            &cas,
   683	            None,
   684	            Hash([0x42u8; 32]),
   685	            vec![Hash([0xAAu8; 32]), Hash([0xBBu8; 32]), Hash([0xCCu8; 32]), Hash([0xDDu8; 32])],
   686	            Hash([0x10u8; 32]),
   687	            Hash([0x20u8; 32]),
   688	            Hash([0x30u8; 32]),
   689	            vec![],
   690	            vec![ObsId("handover/alignment/OBS_X.md".into())],
   691	            "TB-15-R3".into(),
   692	            "tb15-r3-test",
   693	            42,
   694	        )
   695	        .expect("writer");
   696	
   697	        // R3 contract: cas.get(&cap.capsule_id) MUST succeed.
   698	        let cas_r = cas.read().expect("cas read");
   699	        let retrieved = cas_r
   700	            .get(&cap.capsule_id)
   701	            .expect("R3 contract: cas.get(&capsule.capsule_id) MUST succeed");
   702	
   703	        // The retrieved bytes' sha256 MUST equal capsule.capsule_id.
   704	        assert_eq!(
   705	            Cid::from_content(&retrieved),
   706	            cap.capsule_id,
   707	            "R3 contract: sha256(retrieved bytes) == capsule.capsule_id"
   708	        );
   709	
   710	        // canonical_decode + restore reproduces the in-memory capsule.
   711	        let restored = restore_markov_capsule_from_cas_bytes(&retrieved)
   712	            .expect("restore from CAS bytes");
   713	        assert_eq!(
   714	            restored.capsule_id, cap.capsule_id,
   715	            "R3 contract: restored capsule_id matches written capsule_id"
   716	        );
   717	        assert_eq!(restored.constitution_hash, cap.constitution_hash);
   718	        assert_eq!(restored.flowchart_hashes, cap.flowchart_hashes);
   719	        assert_eq!(restored.tb_tag, cap.tb_tag);
   720	        assert_eq!(restored.created_at_logical_t, cap.created_at_logical_t);
   721	
   722	        // next_session_context_cid MUST also be resolvable.
   723	        let _next_ctx = cas_r
   724	            .get(&cap.next_session_context_cid)
   725	            .expect("R3 contract: cas.get(&next_session_context_cid) MUST succeed");
   726	    }
   727	
   728	    /// TB-15 R2 closure: capsule.flowchart_hashes round-trips through canonical encode.
   729	    #[test]
   730	    fn capsule_flowchart_hashes_round_trip() {
   731	        use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
   732	        let mut c = MarkovEvidenceCapsule::default();
   733	        c.flowchart_hashes = vec![
   734	            Hash([0xAAu8; 32]),
   735	            Hash([0xBBu8; 32]),
   736	            Hash([0xCCu8; 32]),
   737	            Hash([0xDDu8; 32]),
   738	        ];
   739	        let bytes = canonical_encode(&c).expect("encode");
   740	        let back: MarkovEvidenceCapsule = canonical_decode(&bytes).expect("decode");
   741	        assert_eq!(back.flowchart_hashes.len(), 4);
   742	        assert_eq!(back.flowchart_hashes[0].0, [0xAAu8; 32]);
   743	    }
   744	}

exec
/bin/bash -lc "nl -ba src/runtime/autopsy_capsule.rs | sed -n '1,360p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-15 Atom 2 — `AgentAutopsyCapsule` schema + writer (architect §6.2,
     2	//! ruling 2026-05-02 + 2026-05-03).
     3	//!
     4	//! Per-agent, per-event capsule for a loss / bankruptcy / failed-market
     5	//! event. CAS-resident; AuditOnly by default. Derived from ChainTape
     6	//! evidence (positions, trades, prices, slippage, resolution, market
     7	//! pool state) — NEVER from agent LLM self-narration (DECISION_LAMARCKIAN
     8	//! §1.2 hard prohibition B).
     9	//!
    10	//! Anchored on `EconomicState.agent_autopsies_t[event_id]: Vec<Cid>`
    11	//! (Atom 3). Public clustering surface (`cluster_autopsies` →
    12	//! `Vec<TypicalErrorSummary>`) lands in Atom 4.
    13	//!
    14	//! Privacy contract:
    15	//! - `public_summary`: low-info string surfaceable to broadcast IFF N≥3
    16	//!   same-class cluster forms (CR-15.2).
    17	//! - `private_detail_cid`: opaque CAS Cid; AuditOnly access only;
    18	//!   NEVER enters `AgentVisibleProjection` (CR-15.1 + SG-15.2).
    19	//! - `evidence_cids`: CAS Cids of pre-existing public ChainTape
    20	//!   evidence (the loss tx, slash tx, ...); not new private bytes.
    21	//!
    22	//! TRACE_MATRIX FC1-N32 (writer) + Art. 0.2 (Tape Canonical: capsule
    23	//! canonical bytes are themselves the CAS object referenced by
    24	//! `capsule_id`) + Art. III.1 (raw failure shielding) + Art. III.2
    25	//! (read-view scoping) + CR-15.3 (autopsy SUGGESTS via
    26	//! `suggested_policy_patch: Option<Cid>`; never mutates predicates).
    27	
    28	use serde::{Deserialize, Serialize};
    29	
    30	use crate::bottom_white::cas::schema::Cid;
    31	use crate::economy::money::MicroCoin;
    32	use crate::state::q_state::{AgentId, Hash};
    33	use crate::state::typed_tx::{CapsulePrivacyPolicy, EventId, RiskRuleId};
    34	
    35	/// TRACE_MATRIX TB-15 (architect §6.2 + DECISION_LAMARCKIAN §1.1) —
    36	/// loss reason discriminator. Architect hint list = AdverseSelection /
    37	/// Overleverage / Goodhart; runtime additions covering current TB-11..14
    38	/// surface = SlashLoss / Bankruptcy / ChallengeUnsuccessful /
    39	/// VerifierBondLost. `Other(String)` keeps forward extensibility without
    40	/// per-TB enum bumps.
    41	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    42	pub enum LossReasonClass {
    43	    /// Solver lost stake to upheld challenge (RSP-3.2 / TB-9 forward
    44	    /// trigger; not yet active in TB-15 v0).
    45	    SlashLoss,
    46	    /// Task entered bankruptcy via `TaskBankruptcyTx`. **TB-15 v0 sole
    47	    /// production trigger** per charter §1.2.
    48	    Bankruptcy,
    49	    /// Challenger's NO bond slashed because challenge was dismissed.
    50	    /// (RSP-3.2 forward trigger.)
    51	    ChallengeUnsuccessful,
    52	    /// Verifier's bond slashed due to incorrect verdict. (RSP-3.2
    53	    /// forward trigger.)
    54	    VerifierBondLost,
    55	    /// Architect §1.1 hint — adverse selection (information asymmetry
    56	    /// led to wrong-side position). TB-16+ scope.
    57	    AdverseSelection,
    58	    /// Architect §1.1 hint — over-leverage (position > Kelly cap).
    59	    Overleverage,
    60	    /// Architect §1.1 hint — Goodhart (chased a metric that was not the
    61	    /// actual goal).
    62	    Goodhart,
    63	    /// Forward extensibility — caller-supplied class string.
    64	    Other(String),
    65	}
    66	
    67	impl Default for LossReasonClass {
    68	    fn default() -> Self {
    69	        Self::Bankruptcy
    70	    }
    71	}
    72	
    73	impl LossReasonClass {
    74	    /// Stable string tag for clustering / dashboard rendering. Avoids
    75	    /// `Debug`'s formatting volatility.
    76	    ///
    77	    /// TRACE_MATRIX FC2-N30 (TB-15 Atom 4): clustering-key surface for
    78	    /// `cluster_autopsies` group-by; also dashboard §15 render tag
    79	    /// (Atom 6).
    80	    pub fn tag(&self) -> &str {
    81	        match self {
    82	            Self::SlashLoss => "SlashLoss",
    83	            Self::Bankruptcy => "Bankruptcy",
    84	            Self::ChallengeUnsuccessful => "ChallengeUnsuccessful",
    85	            Self::VerifierBondLost => "VerifierBondLost",
    86	            Self::AdverseSelection => "AdverseSelection",
    87	            Self::Overleverage => "Overleverage",
    88	            Self::Goodhart => "Goodhart",
    89	            Self::Other(s) => s.as_str(),
    90	        }
    91	    }
    92	}
    93	
    94	/// TRACE_MATRIX TB-15 (architect §6.2 + DECISION_LAMARCKIAN §1.1) —
    95	/// CAS-resident per-agent loss capsule. Default `privacy_policy =
    96	/// AuditOnly` (re-uses TB-11 surface).
    97	///
    98	/// **Privacy** (architect §6.4):
    99	/// - `public_summary`: low-info string; eligible for typical-error
   100	///   broadcast only via Atom 4 `cluster_autopsies` (CR-15.2).
   101	/// - `private_detail_cid`: opaque CAS Cid pointing at
   102	///   `ObjectType::AutopsyPrivateDetail`; access requires audit role.
   103	/// - `evidence_cids`: Cids of pre-existing public ChainTape objects
   104	///   (loss tx CID, sequencer-side slash tx CID, market pool state CID).
   105	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   106	pub struct AgentAutopsyCapsule {
   107	    /// CAS Cid of the canonical-encoded `AgentAutopsyCapsule` itself.
   108	    /// Computed by the writer (sha256 over canonical bytes with this
   109	    /// field zeroed).
   110	    pub capsule_id: Cid,
   111	
   112	    /// Owner of the loss event.
   113	    pub agent_id: AgentId,
   114	    /// Event being autopsied (TB-13 `EventId(TaskId)`; TB-14+ may
   115	    /// decouple per-node).
   116	    pub event_id: EventId,
   117	
   118	    /// Magnitude of the loss in MicroCoin.
   119	    pub loss_amount: MicroCoin,
   120	    /// Class discriminator (CR-15.2 clustering key).
   121	    pub loss_reason_class: LossReasonClass,
   122	
   123	    /// Protocol-level risk rule that the loss event violated, if any.
   124	    /// `None` when the loss did not violate a registered rule (e.g.
   125	    /// Bankruptcy = task ran out of escrow; not a per-agent violation).
   126	    pub violated_risk_rule: Option<RiskRuleId>,
   127	
   128	    /// Optional pointer to a `RiskPolicyPatch` CAS object describing a
   129	    /// patch the autopsy *suggests*. **NEVER auto-applied** (CR-15.3 +
   130	    /// SG-15.8); routing is ArchitectAI proposal → JudgeAI/VetoAI →
   131	    /// canary (P5 v1 surface).
   132	    pub suggested_policy_patch: Option<Cid>,
   133	
   134	    /// CAS Cids of ChainTape evidence anchors (loss tx, slash tx,
   135	    /// position state, market pool state, etc.). Pre-existing public
   136	    /// objects only — autopsy does NOT mint new private evidence here.
   137	    pub evidence_cids: Vec<Cid>,
   138	
   139	    /// Low-information broadcast surface (CR-15.2). Format:
   140	    /// `agent={agent_id} lost {amount}μC on event={event_id} reason={tag}`.
   141	    pub public_summary: String,
   142	    /// Opaque CAS Cid pointing at `ObjectType::AutopsyPrivateDetail`.
   143	    /// Audit-only access. NEVER enters `AgentVisibleProjection`.
   144	    pub private_detail_cid: Cid,
   145	
   146	    /// Privacy default `CapsulePrivacyPolicy::AuditOnly` (architect §6.4).
   147	    pub privacy_policy: CapsulePrivacyPolicy,
   148	
   149	    /// SHA-256 of the canonical-encoded capsule bytes (with `capsule_id`
   150	    /// zeroed). Defense-in-depth duplicate of `capsule_id`.
   151	    pub sha256: Hash,
   152	
   153	    /// Logical time at autopsy emission (sequencer-assigned).
   154	    pub created_at_logical_t: u64,
   155	    /// Round id at autopsy emission (sequencer-assigned).
   156	    pub created_at_round: u64,
   157	}
   158	
   159	impl Default for AgentAutopsyCapsule {
   160	    fn default() -> Self {
   161	        Self {
   162	            capsule_id: Cid::default(),
   163	            agent_id: AgentId::default(),
   164	            event_id: EventId::default(),
   165	            loss_amount: MicroCoin::zero(),
   166	            loss_reason_class: LossReasonClass::default(),
   167	            violated_risk_rule: None,
   168	            suggested_policy_patch: None,
   169	            evidence_cids: Vec::new(),
   170	            public_summary: String::new(),
   171	            private_detail_cid: Cid::default(),
   172	            privacy_policy: CapsulePrivacyPolicy::default(),
   173	            sha256: Hash::ZERO,
   174	            created_at_logical_t: 0,
   175	            created_at_round: 0,
   176	        }
   177	    }
   178	}
   179	
   180	impl AgentAutopsyCapsule {
   181	    /// TRACE_MATRIX architect §6.2 — deterministic public_summary
   182	    /// formatter. Format (stable across runs; broadcast-eligible):
   183	    ///
   184	    /// `agent={agent_id} lost {amount}μC on event={event_task_id} reason={tag}`
   185	    pub fn format_public_summary(
   186	        agent_id: &AgentId,
   187	        event_id: &EventId,
   188	        loss_amount: MicroCoin,
   189	        loss_reason_class: &LossReasonClass,
   190	    ) -> String {
   191	        format!(
   192	            "agent={} lost {}μC on event={} reason={}",
   193	            agent_id.0,
   194	            loss_amount.micro_units(),
   195	            (event_id.0).0,
   196	            loss_reason_class.tag(),
   197	        )
   198	    }
   199	}
   200	
   201	// ────────────────────────────────────────────────────────────────────────────
   202	// TB-15 Atom 2 — Writer
   203	// ────────────────────────────────────────────────────────────────────────────
   204	
   205	use crate::bottom_white::cas::schema::ObjectType;
   206	use crate::bottom_white::cas::store::CasStore;
   207	use crate::bottom_white::ledger::transition_ledger::canonical_encode;
   208	
   209	/// TRACE_MATRIX TB-15 Atom 2 — writer error taxonomy.
   210	#[derive(Debug)]
   211	pub enum AutopsyWriteError {
   212	    Cas(crate::bottom_white::cas::store::CasError),
   213	    Encode(String),
   214	    InternalLockPoisoned,
   215	}
   216	
   217	impl std::fmt::Display for AutopsyWriteError {
   218	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   219	        match self {
   220	            Self::Cas(e) => write!(f, "cas write failed: {e}"),
   221	            Self::Encode(s) => write!(f, "encode failed: {s}"),
   222	            Self::InternalLockPoisoned => write!(f, "internal lock poisoned"),
   223	        }
   224	    }
   225	}
   226	impl std::error::Error for AutopsyWriteError {}
   227	
   228	impl From<crate::bottom_white::cas::store::CasError> for AutopsyWriteError {
   229	    fn from(e: crate::bottom_white::cas::store::CasError) -> Self {
   230	        Self::Cas(e)
   231	    }
   232	}
   233	
   234	/// TRACE_MATRIX TB-15 Atom 2 (architect §6.2): write an
   235	/// `AgentAutopsyCapsule` to CAS. Flow:
   236	///
   237	/// 1. Build canonical private-detail JSON from caller-supplied
   238	///    `private_detail_payload` bytes → write to CAS as
   239	///    `ObjectType::AutopsyPrivateDetail`. Cid is `private_detail_cid`.
   240	/// 2. Build the capsule struct with `capsule_id = Cid::default()` +
   241	///    `sha256 = Hash::ZERO`. Canonical-encode → sha256 → that's the
   242	///    eventual `capsule_id`.
   243	/// 3. Re-create the struct with `capsule_id` filled in + write to CAS
   244	///    as `ObjectType::AgentAutopsyCapsule`.
   245	///
   246	/// Returns the populated `AgentAutopsyCapsule` (with `capsule_id` set).
   247	///
   248	/// **CR-15.3 / SG-15.8**: writer signature has NO mutable reference to
   249	/// any predicate / tool / risk-policy registry. `suggested_policy_patch`
   250	/// is an opaque `Option<Cid>` pointer; the writer does not interpret
   251	/// or apply it.
   252	#[allow(clippy::too_many_arguments)]
   253	pub fn write_autopsy_capsule(
   254	    cas: &std::sync::Arc<std::sync::RwLock<CasStore>>,
   255	    agent_id: AgentId,
   256	    event_id: EventId,
   257	    loss_amount: MicroCoin,
   258	    loss_reason_class: LossReasonClass,
   259	    violated_risk_rule: Option<RiskRuleId>,
   260	    suggested_policy_patch: Option<Cid>,
   261	    evidence_cids: Vec<Cid>,
   262	    private_detail_payload: &[u8],
   263	    privacy: CapsulePrivacyPolicy,
   264	    creator_str: &str,
   265	    created_at_logical_t: u64,
   266	    created_at_round: u64,
   267	) -> Result<AgentAutopsyCapsule, AutopsyWriteError> {
   268	    let mut cas_w = cas
   269	        .write()
   270	        .map_err(|_| AutopsyWriteError::InternalLockPoisoned)?;
   271	
   272	    // Step 1: write private detail to CAS (caller-supplied opaque bytes).
   273	    let private_detail_cid = cas_w.put(
   274	        private_detail_payload,
   275	        ObjectType::AutopsyPrivateDetail,
   276	        creator_str,
   277	        created_at_logical_t,
   278	        Some("v1/autopsy_private_detail".into()),
   279	    )?;
   280	
   281	    // Step 2: build capsule with capsule_id = 0 + sha256 = 0; canonical
   282	    // encode; sha256 of bytes is the eventual capsule_id.
   283	    let public_summary = AgentAutopsyCapsule::format_public_summary(
   284	        &agent_id,
   285	        &event_id,
   286	        loss_amount,
   287	        &loss_reason_class,
   288	    );
   289	    let mut capsule = AgentAutopsyCapsule {
   290	        capsule_id: Cid::default(),
   291	        agent_id,
   292	        event_id,
   293	        loss_amount,
   294	        loss_reason_class,
   295	        violated_risk_rule,
   296	        suggested_policy_patch,
   297	        evidence_cids,
   298	        public_summary,
   299	        private_detail_cid,
   300	        privacy_policy: privacy,
   301	        sha256: Hash::ZERO,
   302	        created_at_logical_t,
   303	        created_at_round,
   304	    };
   305	    // R3 closure (Codex R2 VETO TB15-CAS-ID): identical pattern to
   306	    // write_markov_capsule. Store the bytes whose sha256 equals
   307	    // capsule_id, NOT the post-population bytes (which would have a
   308	    // different sha256, breaking cas.get(&capsule_id) resolvability).
   309	    // The in-memory struct returned to caller has populated
   310	    // capsule_id+sha256; on-CAS bytes have these zeroed.
   311	    let stored_bytes = canonical_encode(&capsule)
   312	        .map_err(|e| AutopsyWriteError::Encode(format!("capsule canonical encode: {e:?}")))?;
   313	    let capsule_cid = Cid::from_content(&stored_bytes);
   314	    let cas_returned_cid = cas_w.put(
   315	        &stored_bytes,
   316	        ObjectType::AgentAutopsyCapsule,
   317	        creator_str,
   318	        created_at_logical_t,
   319	        Some("v1/agent_autopsy_capsule".into()),
   320	    )?;
   321	    debug_assert_eq!(
   322	        cas_returned_cid, capsule_cid,
   323	        "CAS-returned cid must equal sha256(stored_bytes); CasStore::put contract"
   324	    );
   325	    capsule.capsule_id = capsule_cid;
   326	    capsule.sha256 = Hash(capsule_cid.0);
   327	
   328	    Ok(capsule)
   329	}
   330	
   331	/// TRACE_MATRIX TB-15 R3 closure (Codex R2 VETO TB15-CAS-ID): rebuild
   332	/// an `AgentAutopsyCapsule` from CAS-resident bytes. Symmetric helper
   333	/// to `restore_markov_capsule_from_cas_bytes`. Caller supplies the
   334	/// bytes returned by `cas.get(&capsule_id)`; helper canonical-decodes
   335	/// + re-derives capsule_id/sha256 from `Cid::from_content(&bytes)`.
   336	pub fn restore_autopsy_capsule_from_cas_bytes(
   337	    bytes: &[u8],
   338	) -> Result<AgentAutopsyCapsule, AutopsyWriteError> {
   339	    use crate::bottom_white::ledger::transition_ledger::canonical_decode;
   340	    let mut cap: AgentAutopsyCapsule = canonical_decode(bytes)
   341	        .map_err(|e| AutopsyWriteError::Encode(format!("capsule decode: {e:?}")))?;
   342	    let cid = Cid::from_content(bytes);
   343	    cap.capsule_id = cid;
   344	    cap.sha256 = Hash(cid.0);
   345	    Ok(cap)
   346	}
   347	
   348	// ────────────────────────────────────────────────────────────────────────────
   349	// TB-15 R2 closure — Activation gate (Gemini R1 VETO Q12; replay-determinism)
   350	// ────────────────────────────────────────────────────────────────────────────
   351	//
   352	// Per Gemini R1 audit 2026-05-04 Q12 VETO + `feedback_no_retroactive_evidence_rewrite`:
   353	// replaying a pre-TB-15 chain post-TB-15-deployment must NOT spuriously
   354	// generate `AgentAutopsyCapsule` entries that did not exist in the
   355	// original live execution.
   356	//
   357	// Verification baseline (2026-05-04): grep across all on-disk
   358	// `handover/evidence/*/runtime_repo` chains found ZERO production
   359	// `TaskBankruptcyTx` rows pre-TB-15 (TB-11 added the variant; no
   360	// production chain has fired one). The structural concern is real but

exec
/bin/bash -lc "nl -ba src/runtime/autopsy_capsule.rs | sed -n '360,620p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   360	// production chain has fired one). The structural concern is real but
   361	// no chain currently triggers it.
   362	//
   363	// The activation gate is set at compile time. Default = 0 means
   364	// "always active for fresh chains shipped at or after TB-15
   365	// (commit 2337381 + onwards)" — every new chain starts at logical_t=1
   366	// which trivially satisfies `>= 0`. For pre-TB-15 chain replay
   367	// (no such chain exists today; future migrations would set this
   368	// non-zero), the cutoff would be the first logical_t at which the
   369	// post-TB-15 sequencer becomes authoritative.
   370	//
   371	// **Constitutional alignment** (Art.0.2 Tape Canonical):
   372	// post-activation replay reconstructs identical agent_autopsies_t
   373	// entries by deterministic helper. Pre-activation rows pass through
   374	// the dispatch arm without autopsy mutation, preserving the original
   375	// EconomicState shape.
   376	//
   377	// Future migration story: when a pre-TB-15 chain with TaskBankruptcyTx
   378	// rows needs to be replayed, the operator overrides
   379	// `TB15_AUTOPSY_ACTIVATION_LOGICAL_T` to the cutoff at deployment.
   380	// Pre-cutoff TaskBankruptcyTx rows replay cleanly without spurious
   381	// autopsy entries.
   382	
   383	/// TRACE_MATRIX TB-15 R2 closure (Gemini R1 VETO Q12; activation gate
   384	/// for replay-determinism). Default 0 = always active for fresh chains
   385	/// (TB-15 ship commit 2337381 onwards; every new chain starts at
   386	/// logical_t=1 ≥ 0 → trivially active). Overridable at compile time
   387	/// for pre-TB-15 chain migration scenarios.
   388	pub const TB15_AUTOPSY_ACTIVATION_LOGICAL_T: u64 = 0;
   389	
   390	/// TRACE_MATRIX TB-15 R2 closure: gate predicate. Returns true iff
   391	/// autopsy emission is enabled for a TaskBankruptcyTx with the given
   392	/// timestamp_logical. Pure-fn over the activation constant; identical
   393	/// in dispatch arm and apply_one Stage 3.5.
   394	#[inline]
   395	pub fn is_autopsy_active_at(timestamp_logical: u64) -> bool {
   396	    timestamp_logical >= TB15_AUTOPSY_ACTIVATION_LOGICAL_T
   397	}
   398	
   399	// ────────────────────────────────────────────────────────────────────────────
   400	// TB-15 Atom 4 — TypicalErrorBroadcast clustering (architect §3.2.3 + CR-15.2)
   401	// ────────────────────────────────────────────────────────────────────────────
   402	//
   403	// `cluster_autopsies` groups input autopsies by `loss_reason_class`, and
   404	// emits a `TypicalErrorSummary` for each class whose count meets or
   405	// exceeds the broadcast threshold (default N=3 per
   406	// DECISION_LAMARCKIAN §3.2.3 + spec test 3.2.3 verbatim).
   407	//
   408	// **CR-15.2 + halt-trigger #5**: the output struct embeds
   409	// `public_summary` strings + `capsule_id` Cids only — NEVER
   410	// `private_detail_cid` payload bytes. Halt-trigger #5 verifies this by
   411	// serializing the output and scanning for any input
   412	// `private_detail_cid` byte sequence.
   413	
   414	/// TRACE_MATRIX FC2-N30 (TB-15 Atom 4; architect §3.2.3 + CR-15.2):
   415	/// public broadcast summary for an N≥threshold cluster of same-class
   416	/// autopsies. Embeds `public_summary` text + capsule Cids only;
   417	/// `private_detail_cid` bytes are NEVER included (halt-trigger #5).
   418	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   419	pub struct TypicalErrorSummary {
   420	    /// The shared loss reason class for the cluster.
   421	    pub loss_reason_class: LossReasonClass,
   422	    /// Count of capsules in this cluster.
   423	    pub count: u32,
   424	    /// Public broadcast text — concatenation / first-N exemplars of
   425	    /// each capsule's `public_summary`. Joined with " ; " separator.
   426	    pub exemplar_public_summary: String,
   427	    /// Cids of the contributing capsules (audit can fetch them with
   428	    /// AuditOnly access). NEVER private_detail_cids.
   429	    pub exemplar_capsule_cids: Vec<Cid>,
   430	}
   431	
   432	/// TRACE_MATRIX FC2-N30 (TB-15 Atom 4; architect §3.2.3): cluster
   433	/// autopsies by `loss_reason_class`. Emit a `TypicalErrorSummary` for
   434	/// each class whose count is `>= threshold`. Default architect
   435	/// threshold = 3 (DECISION_LAMARCKIAN §3.2.3 + spec test 3.2.3).
   436	///
   437	/// **Pure** — no CAS access, no env, no clock. Order-stable: input
   438	/// order preserved within each class; classes themselves emerge in
   439	/// `LossReasonClass::tag()` lexicographic order (BTreeMap iteration)
   440	/// for replay-determinism.
   441	///
   442	/// **CR-15.2 + halt-trigger #5**: output never embeds
   443	/// `private_detail_cid` bytes — only `public_summary` strings +
   444	/// `capsule_id` Cids.
   445	pub fn cluster_autopsies(
   446	    autopsies: &[AgentAutopsyCapsule],
   447	    threshold: u8,
   448	) -> Vec<TypicalErrorSummary> {
   449	    use std::collections::BTreeMap;
   450	    // Group by loss_reason_class.tag() for deterministic iteration.
   451	    let mut groups: BTreeMap<String, Vec<&AgentAutopsyCapsule>> = BTreeMap::new();
   452	    for c in autopsies {
   453	        groups
   454	            .entry(c.loss_reason_class.tag().to_string())
   455	            .or_default()
   456	            .push(c);
   457	    }
   458	    let mut out = Vec::new();
   459	    let threshold_usize = threshold as usize;
   460	    for (_tag, members) in groups {
   461	        if members.len() < threshold_usize {
   462	            continue;
   463	        }
   464	        let exemplar_public_summary = members
   465	            .iter()
   466	            .map(|c| c.public_summary.as_str())
   467	            .collect::<Vec<_>>()
   468	            .join(" ; ");
   469	        let exemplar_capsule_cids: Vec<Cid> = members.iter().map(|c| c.capsule_id).collect();
   470	        out.push(TypicalErrorSummary {
   471	            // All members share the same class by construction.
   472	            loss_reason_class: members[0].loss_reason_class.clone(),
   473	            count: members.len() as u32,
   474	            exemplar_public_summary,
   475	            exemplar_capsule_cids,
   476	        });
   477	    }
   478	    out
   479	}
   480	
   481	// ────────────────────────────────────────────────────────────────────────────
   482	// TB-15 Atom 3 — `derive_autopsies_for_bankruptcy` (PURE deterministic helper)
   483	// ────────────────────────────────────────────────────────────────────────────
   484	//
   485	// Pure function consumed by both the dispatch arm (to populate
   486	// `EconomicState.agent_autopsies_t` with deterministic Cids) AND by
   487	// the apply_one post-dispatch hook (to write the same bytes to CAS so
   488	// they're retrievable). Replay-determinism: identical inputs → identical
   489	// `(Cid, AgentAutopsyCapsule, private_detail_bytes)` triples.
   490	
   491	use crate::state::q_state::EconomicState;
   492	use crate::state::typed_tx::TaskBankruptcyTx;
   493	
   494	/// TRACE_MATRIX FC1-N33 (TB-15 Atom 3; architect §6.2 + DECISION_LAMARCKIAN
   495	/// §1.1): pure-deterministic derivation of `AgentAutopsyCapsule`s for a
   496	/// `TaskBankruptcyTx`. Returns one capsule per agent with an active
   497	/// `StakeEntry` pointing at the bankrupted task — `loss_reason_class =
   498	/// Bankruptcy`; `loss_amount = stake.amount`; `evidence_cids = [Cid of
   499	/// stake_tx_id]`. BTreeMap iteration is sorted by `TxId` → output order
   500	/// is deterministic.
   501	///
   502	/// **Pure**: takes pre-bankruptcy `EconomicState` snapshot + the
   503	/// `TaskBankruptcyTx`; no CAS writes, no env access. Used by:
   504	/// - dispatch arm: capsule_id population into `agent_autopsies_t`
   505	/// - apply_one hook: CAS write of the same deterministic bytes
   506	///
   507	/// Replay determinism (Art.0.2): identical `(pre_econ, bk, round, t)` →
   508	/// identical `Vec<BankruptcyAutopsyDerivation>` (same Cids, same bytes,
   509	/// same order).
   510	pub fn derive_autopsies_for_bankruptcy(
   511	    pre_econ: &EconomicState,
   512	    bk: &TaskBankruptcyTx,
   513	    created_at_round: u64,
   514	    created_at_logical_t: u64,
   515	) -> Vec<BankruptcyAutopsyDerivation> {
   516	    let event_id = EventId(bk.task_id.clone());
   517	    let mut out = Vec::new();
   518	
   519	    for (stake_tx_id, stake) in pre_econ.stakes_t.0.iter() {
   520	        if stake.task_id != bk.task_id {
   521	            continue;
   522	        }
   523	        // Deterministic private_detail JSON.
   524	        let private_detail = format!(
   525	            "{{\"event_kind\":\"task_bankruptcy\",\"task_id\":\"{}\",\
   526	             \"stake_tx_id\":\"{}\",\"staker\":\"{}\",\
   527	             \"stake_amount_micro\":{}}}",
   528	            stake.task_id.0,
   529	            stake_tx_id.0,
   530	            stake.staker.0,
   531	            stake.amount.micro_units()
   532	        );
   533	        let private_bytes = private_detail.into_bytes();
   534	        let private_detail_cid = Cid::from_content(&private_bytes);
   535	
   536	        let public_summary = AgentAutopsyCapsule::format_public_summary(
   537	            &stake.staker,
   538	            &event_id,
   539	            stake.amount,
   540	            &LossReasonClass::Bankruptcy,
   541	        );
   542	
   543	        let mut capsule = AgentAutopsyCapsule {
   544	            capsule_id: Cid::default(),
   545	            agent_id: stake.staker.clone(),
   546	            event_id: event_id.clone(),
   547	            loss_amount: stake.amount,
   548	            loss_reason_class: LossReasonClass::Bankruptcy,
   549	            violated_risk_rule: None,
   550	            suggested_policy_patch: None,
   551	            evidence_cids: vec![Cid::from_content(stake_tx_id.0.as_bytes())],
   552	            public_summary,
   553	            private_detail_cid,
   554	            privacy_policy: CapsulePrivacyPolicy::AuditOnly,
   555	            sha256: Hash::ZERO,
   556	            created_at_logical_t,
   557	            created_at_round,
   558	        };
   559	        // R3 closure (Codex R2 VETO TB15-CAS-ID): canonical_encode
   560	        // BEFORE populating capsule_id/sha256 — these stored bytes are
   561	        // what apply_one writes to CAS. capsule_id = sha256 of these
   562	        // bytes, ensuring cas.get(&capsule_id) returns these exact
   563	        // bytes on retrieval.
   564	        let stored_bytes = canonical_encode(&capsule)
   565	            .expect("AgentAutopsyCapsule is canonical-encodable");
   566	        let cid = Cid::from_content(&stored_bytes);
   567	        capsule.capsule_id = cid;
   568	        capsule.sha256 = Hash(cid.0);
   569	
   570	        out.push(BankruptcyAutopsyDerivation {
   571	            capsule,
   572	            private_bytes,
   573	            stored_capsule_bytes: stored_bytes,
   574	        });
   575	    }
   576	    out
   577	}
   578	
   579	/// TRACE_MATRIX TB-15 R3 closure (Codex R2 VETO TB15-CAS-ID): bundle
   580	/// the deterministic outputs of `derive_autopsies_for_bankruptcy`. The
   581	/// dispatch arm reads only `capsule.capsule_id`; apply_one writes
   582	/// `private_bytes` + `stored_capsule_bytes` to CAS keyed by the
   583	/// matching Cids. Replay-safe: identical pre-econ + tx → identical
   584	/// (capsule_id, private_bytes, stored_capsule_bytes) tuple.
   585	#[derive(Debug, Clone)]
   586	pub struct BankruptcyAutopsyDerivation {
   587	    pub capsule: AgentAutopsyCapsule,
   588	    pub private_bytes: Vec<u8>,
   589	    pub stored_capsule_bytes: Vec<u8>,
   590	}
   591	
   592	/// TRACE_MATRIX FC1-N33 (TB-15 Atom 3): apply_one post-dispatch hook —
   593	/// writes deterministic autopsy bytes to CAS for a successfully-accepted
   594	/// `TaskBankruptcyTx`. Re-derives the capsule list using
   595	/// `derive_autopsies_for_bankruptcy` (same inputs → same Cids as the
   596	/// dispatch arm already populated into `agent_autopsies_t`).
   597	///
   598	/// Idempotent: CAS `put` of identical bytes returns the existing Cid
   599	/// (replay-safe — re-running apply_one yields the same CAS state).
   600	pub fn write_bankruptcy_autopsies_to_cas(
   601	    cas: &std::sync::Arc<std::sync::RwLock<CasStore>>,
   602	    pre_econ: &EconomicState,
   603	    bk: &TaskBankruptcyTx,
   604	    created_at_round: u64,
   605	    created_at_logical_t: u64,
   606	    creator_str: &str,
   607	) -> Result<Vec<Cid>, AutopsyWriteError> {
   608	    let derived = derive_autopsies_for_bankruptcy(
   609	        pre_econ,
   610	        bk,
   611	        created_at_round,
   612	        created_at_logical_t,
   613	    );
   614	    let mut cids = Vec::with_capacity(derived.len());
   615	    let mut cas_w = cas
   616	        .write()
   617	        .map_err(|_| AutopsyWriteError::InternalLockPoisoned)?;
   618	    for d in derived {
   619	        // R3 closure (Codex R2 VETO TB15-CAS-ID): write the EXACT
   620	        // stored_capsule_bytes returned by the derive helper. CAS keys

exec
/bin/bash -lc "nl -ba src/runtime/autopsy_capsule.rs | sed -n '620,1120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   620	        // stored_capsule_bytes returned by the derive helper. CAS keys
   621	        // by sha256(bytes), which equals capsule.capsule_id by helper
   622	        // construction. Idempotent: identical bytes → identical Cid →
   623	        // CAS dedupe. cas.get(&capsule.capsule_id) is now resolvable.
   624	        let _ = cas_w.put(
   625	            &d.private_bytes,
   626	            ObjectType::AutopsyPrivateDetail,
   627	            creator_str,
   628	            created_at_logical_t,
   629	            Some("v1/autopsy_private_detail".into()),
   630	        )?;
   631	        let cas_returned_cid = cas_w.put(
   632	            &d.stored_capsule_bytes,
   633	            ObjectType::AgentAutopsyCapsule,
   634	            creator_str,
   635	            created_at_logical_t,
   636	            Some("v1/agent_autopsy_capsule".into()),
   637	        )?;
   638	        debug_assert_eq!(
   639	            cas_returned_cid, d.capsule.capsule_id,
   640	            "CAS-returned cid must equal capsule.capsule_id (CasStore::put contract)"
   641	        );
   642	        cids.push(d.capsule.capsule_id);
   643	    }
   644	    Ok(cids)
   645	}
   646	
   647	#[cfg(test)]
   648	mod tests {
   649	    use super::*;
   650	    use crate::state::q_state::TaskId;
   651	
   652	    /// TB-15 U1: capsule default round-trips through canonical bytes.
   653	    #[test]
   654	    fn autopsy_capsule_default_round_trip() {
   655	        use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
   656	        let c = AgentAutopsyCapsule::default();
   657	        let bytes = canonical_encode(&c).expect("encode");
   658	        let back: AgentAutopsyCapsule = canonical_decode(&bytes).expect("decode");
   659	        assert_eq!(c, back);
   660	    }
   661	
   662	    /// TB-15 U2: format_public_summary embeds agent_id + amount + reason tag.
   663	    #[test]
   664	    fn format_public_summary_contains_agent_amount_reason() {
   665	        let s = AgentAutopsyCapsule::format_public_summary(
   666	            &AgentId("Agent_solver_3".into()),
   667	            &EventId(TaskId("task:lean:t1".into())),
   668	            MicroCoin::from_micro_units(1500),
   669	            &LossReasonClass::Bankruptcy,
   670	        );
   671	        assert!(s.contains("Agent_solver_3"));
   672	        assert!(s.contains("1500"));
   673	        assert!(s.contains("task:lean:t1"));
   674	        assert!(s.contains("Bankruptcy"));
   675	    }
   676	
   677	    /// TB-15 U3: privacy_policy default = AuditOnly (re-use TB-11
   678	    /// CR-15.1 surface).
   679	    #[test]
   680	    fn privacy_policy_default_is_audit_only() {
   681	        let c = AgentAutopsyCapsule::default();
   682	        assert_eq!(c.privacy_policy, CapsulePrivacyPolicy::AuditOnly);
   683	    }
   684	
   685	    /// TB-15 Atom 2 — Writer: writes private_detail + capsule to CAS;
   686	    /// returned capsule has populated capsule_id (Cid of canonical
   687	    /// bytes) and matching sha256.
   688	    #[test]
   689	    fn write_autopsy_capsule_to_cas_round_trip() {
   690	        use std::sync::{Arc, RwLock};
   691	        use tempfile::TempDir;
   692	
   693	        let tmp = TempDir::new().expect("tempdir");
   694	        let cas = Arc::new(RwLock::new(
   695	            crate::bottom_white::cas::store::CasStore::open(tmp.path()).expect("cas"),
   696	        ));
   697	
   698	        let private_detail = br#"{"position":[],"slippage":0,"pool_state":"empty"}"#;
   699	        let cap = write_autopsy_capsule(
   700	            &cas,
   701	            AgentId("Agent_solver_0".into()),
   702	            EventId(TaskId("task:lean:tb15:autopsy_writer".into())),
   703	            MicroCoin::from_micro_units(2_500),
   704	            LossReasonClass::Bankruptcy,
   705	            None,
   706	            None,
   707	            vec![Cid::from_content(b"loss_tx_cid_placeholder")],
   708	            private_detail,
   709	            CapsulePrivacyPolicy::AuditOnly,
   710	            "tb-15-writer",
   711	            42,
   712	            7,
   713	        )
   714	        .expect("writer succeeds");
   715	
   716	        // Capsule_id populated and matches sha256.
   717	        assert_ne!(cap.capsule_id, Cid::default());
   718	        assert_eq!(cap.capsule_id.0, cap.sha256.0);
   719	
   720	        // Private detail Cid populated.
   721	        assert_ne!(cap.private_detail_cid, Cid::default());
   722	
   723	        // Public summary has expected shape.
   724	        assert!(cap.public_summary.contains("Agent_solver_0"));
   725	        assert!(cap.public_summary.contains("2500"));
   726	        assert!(cap.public_summary.contains("Bankruptcy"));
   727	
   728	        // CAS contains 2 objects: private_detail + capsule.
   729	        let cas_r = cas.read().expect("cas read");
   730	        assert_eq!(
   731	            cas_r.len(),
   732	            2,
   733	            "writer puts 2 CAS objects: private_detail + capsule"
   734	        );
   735	
   736	        // Private detail bytes retrievable.
   737	        let retrieved = cas_r.get(&cap.private_detail_cid).expect("get priv");
   738	        assert_eq!(retrieved, private_detail);
   739	    }
   740	
   741	    /// TB-15 Atom 2 — Writer: same inputs → same capsule_id (deterministic).
   742	    #[test]
   743	    fn write_autopsy_capsule_deterministic_capsule_id() {
   744	        use std::sync::{Arc, RwLock};
   745	        use tempfile::TempDir;
   746	
   747	        let private_detail = b"deterministic-detail-bytes";
   748	        let mk = || -> AgentAutopsyCapsule {
   749	            let tmp = TempDir::new().unwrap();
   750	            let cas = Arc::new(RwLock::new(
   751	                crate::bottom_white::cas::store::CasStore::open(tmp.path()).unwrap(),
   752	            ));
   753	            write_autopsy_capsule(
   754	                &cas,
   755	                AgentId("Agent_X".into()),
   756	                EventId(TaskId("task:tb15:det".into())),
   757	                MicroCoin::from_micro_units(777),
   758	                LossReasonClass::SlashLoss,
   759	                Some(RiskRuleId("max_drawdown".into())),
   760	                None,
   761	                vec![Cid::from_content(b"ev1"), Cid::from_content(b"ev2")],
   762	                private_detail,
   763	                CapsulePrivacyPolicy::AuditOnly,
   764	                "writer",
   765	                3,
   766	                1,
   767	            )
   768	            .expect("writer")
   769	        };
   770	        let a = mk();
   771	        let b = mk();
   772	        assert_eq!(a.capsule_id, b.capsule_id);
   773	        assert_eq!(a.private_detail_cid, b.private_detail_cid);
   774	    }
   775	
   776	    /// TB-15 Atom 2 — LossReasonClass::tag is stable across all variants.
   777	    #[test]
   778	    fn loss_reason_class_tag_stable() {
   779	        assert_eq!(LossReasonClass::SlashLoss.tag(), "SlashLoss");
   780	        assert_eq!(LossReasonClass::Bankruptcy.tag(), "Bankruptcy");
   781	        assert_eq!(
   782	            LossReasonClass::ChallengeUnsuccessful.tag(),
   783	            "ChallengeUnsuccessful"
   784	        );
   785	        assert_eq!(LossReasonClass::VerifierBondLost.tag(), "VerifierBondLost");
   786	        assert_eq!(LossReasonClass::AdverseSelection.tag(), "AdverseSelection");
   787	        assert_eq!(LossReasonClass::Overleverage.tag(), "Overleverage");
   788	        assert_eq!(LossReasonClass::Goodhart.tag(), "Goodhart");
   789	        assert_eq!(
   790	            LossReasonClass::Other("CustomThing".into()).tag(),
   791	            "CustomThing"
   792	        );
   793	    }
   794	
   795	    // ───────────────────────────────────────────────────────────────────
   796	    // Atom 3 — derive_autopsies_for_bankruptcy tests
   797	    // ───────────────────────────────────────────────────────────────────
   798	
   799	    use crate::state::q_state::{
   800	        BalancesIndex, EconomicState, StakeEntry, StakesIndex, TaskMarketEntry,
   801	        TaskMarketState, TaskMarketsIndex, TxId,
   802	    };
   803	    use crate::state::typed_tx::TaskBankruptcyTx;
   804	
   805	    fn synthetic_econ_with_stakes(task_id: &str, stakers: &[(&str, &str, i64)]) -> EconomicState {
   806	        let mut econ = EconomicState::default();
   807	        // Add a TaskMarketEntry so the dispatch arm could find the task —
   808	        // not strictly needed by derive_autopsies_for_bankruptcy itself.
   809	        econ.task_markets_t = TaskMarketsIndex::default();
   810	        econ.task_markets_t.0.insert(
   811	            TaskId(task_id.into()),
   812	            TaskMarketEntry {
   813	                state: TaskMarketState::Open,
   814	                ..Default::default()
   815	            },
   816	        );
   817	        // Pre-bankruptcy stakes for the target task (and one off-target
   818	        // stake to verify the filter works).
   819	        let mut stakes = StakesIndex::default();
   820	        for (stake_tx_id, staker_id, amt) in stakers {
   821	            stakes.0.insert(
   822	                TxId((*stake_tx_id).into()),
   823	                StakeEntry {
   824	                    amount: MicroCoin::from_micro_units(*amt),
   825	                    staker: AgentId((*staker_id).into()),
   826	                    task_id: TaskId(task_id.into()),
   827	                },
   828	            );
   829	        }
   830	        // One off-target stake — same Map, different task_id; must be
   831	        // filtered out.
   832	        stakes.0.insert(
   833	            TxId("stake_off_target".into()),
   834	            StakeEntry {
   835	                amount: MicroCoin::from_micro_units(999),
   836	                staker: AgentId("Agent_off_target".into()),
   837	                task_id: TaskId("task:other".into()),
   838	            },
   839	        );
   840	        econ.stakes_t = stakes;
   841	        econ.balances_t = BalancesIndex::default();
   842	        econ
   843	    }
   844	
   845	    fn synthetic_bk(task_id: &str) -> TaskBankruptcyTx {
   846	        TaskBankruptcyTx {
   847	            task_id: TaskId(task_id.into()),
   848	            timestamp_logical: 100,
   849	            ..Default::default()
   850	        }
   851	    }
   852	
   853	    /// TB-15 Atom 3 — derive_autopsies_for_bankruptcy: per-staker
   854	    /// emission for the target task; off-target stakes filtered out.
   855	    #[test]
   856	    fn derive_autopsies_emits_one_per_staker_target_only() {
   857	        let task = "task:tb15:bankruptcy";
   858	        let econ = synthetic_econ_with_stakes(
   859	            task,
   860	            &[("stake_tx_a", "Agent_A", 1000), ("stake_tx_b", "Agent_B", 2000)],
   861	        );
   862	        let bk = synthetic_bk(task);
   863	
   864	        let derived = derive_autopsies_for_bankruptcy(&econ, &bk, /*round=*/ 5, /*t=*/ 100);
   865	
   866	        assert_eq!(
   867	            derived.len(),
   868	            2,
   869	            "2 stakers on the target task → 2 capsules; off-target stake filtered out"
   870	        );
   871	        let agents: Vec<&str> = derived
   872	            .iter()
   873	            .map(|d| d.capsule.agent_id.0.as_str())
   874	            .collect();
   875	        assert!(agents.contains(&"Agent_A"));
   876	        assert!(agents.contains(&"Agent_B"));
   877	        assert!(!agents.contains(&"Agent_off_target"));
   878	
   879	        // Each capsule reports the correct event_id, loss_amount,
   880	        // loss_reason_class, and a populated capsule_id.
   881	        for d in &derived {
   882	            let c = &d.capsule;
   883	            assert_eq!(c.event_id.0 .0, task);
   884	            assert_eq!(c.loss_reason_class, LossReasonClass::Bankruptcy);
   885	            assert_ne!(c.capsule_id, Cid::default());
   886	            assert_eq!(c.capsule_id.0, c.sha256.0);
   887	            assert!(c.public_summary.contains(task));
   888	            assert!(c.public_summary.contains("Bankruptcy"));
   889	            // R3 closure (Codex R2 VETO TB15-CAS-ID): capsule_id MUST
   890	            // equal sha256(stored_capsule_bytes).
   891	            assert_eq!(
   892	                c.capsule_id,
   893	                Cid::from_content(&d.stored_capsule_bytes),
   894	                "capsule_id must equal sha256(stored_capsule_bytes) — \
   895	                 cas.get(&capsule_id) resolvability contract"
   896	            );
   897	        }
   898	    }
   899	
   900	    /// TB-15 Atom 3 — derive_autopsies_for_bankruptcy: same inputs →
   901	    /// identical (Cid, capsule, bytes) — replay-determinism foundation
   902	    /// (Art.0.2). Underwrites the dispatch / apply_one Cid agreement.
   903	    #[test]
   904	    fn derive_autopsies_deterministic_across_calls() {
   905	        let task = "task:tb15:det";
   906	        let econ = synthetic_econ_with_stakes(
   907	            task,
   908	            &[("stake_tx_x", "Agent_X", 500), ("stake_tx_y", "Agent_Y", 750)],
   909	        );
   910	        let bk = synthetic_bk(task);
   911	
   912	        let a = derive_autopsies_for_bankruptcy(&econ, &bk, 3, 50);
   913	        let b = derive_autopsies_for_bankruptcy(&econ, &bk, 3, 50);
   914	
   915	        assert_eq!(a.len(), b.len());
   916	        for (i, (da, db)) in a.iter().zip(b.iter()).enumerate() {
   917	            assert_eq!(
   918	                da.capsule.capsule_id, db.capsule.capsule_id,
   919	                "capsule {i} cid mismatch"
   920	            );
   921	            assert_eq!(da.capsule, db.capsule, "capsule {i} struct mismatch");
   922	            assert_eq!(
   923	                da.private_bytes, db.private_bytes,
   924	                "capsule {i} private_detail bytes mismatch"
   925	            );
   926	            assert_eq!(
   927	                da.stored_capsule_bytes, db.stored_capsule_bytes,
   928	                "capsule {i} stored_capsule_bytes mismatch"
   929	            );
   930	        }
   931	    }
   932	
   933	    /// TB-15 Atom 3 — derive_autopsies_for_bankruptcy: no stakers on
   934	    /// the bankrupted task → empty Vec (no capsules emitted).
   935	    #[test]
   936	    fn derive_autopsies_empty_when_no_stakers() {
   937	        let task = "task:tb15:nostakers";
   938	        let mut econ = EconomicState::default();
   939	        econ.task_markets_t.0.insert(
   940	            TaskId(task.into()),
   941	            TaskMarketEntry {
   942	                state: TaskMarketState::Open,
   943	                ..Default::default()
   944	            },
   945	        );
   946	        let bk = synthetic_bk(task);
   947	        let derived = derive_autopsies_for_bankruptcy(&econ, &bk, 0, 0);
   948	        assert!(derived.is_empty());
   949	    }
   950	
   951	    // ───────────────────────────────────────────────────────────────────
   952	    // Atom 4 — cluster_autopsies tests
   953	    // ───────────────────────────────────────────────────────────────────
   954	
   955	    fn mk_autopsy(agent: &str, class: LossReasonClass, priv_byte: u8) -> AgentAutopsyCapsule {
   956	        let mut cap = AgentAutopsyCapsule::default();
   957	        cap.agent_id = AgentId(agent.into());
   958	        cap.event_id = EventId(TaskId("task:tb15:cluster".into()));
   959	        cap.loss_amount = MicroCoin::from_micro_units(1_000);
   960	        cap.loss_reason_class = class.clone();
   961	        cap.public_summary = AgentAutopsyCapsule::format_public_summary(
   962	            &cap.agent_id,
   963	            &cap.event_id,
   964	            cap.loss_amount,
   965	            &class,
   966	        );
   967	        cap.private_detail_cid = Cid([priv_byte; 32]);
   968	        cap.capsule_id = Cid::from_content(agent.as_bytes());
   969	        cap
   970	    }
   971	
   972	    /// TB-15 Atom 4 — 3 same-class autopsies → exactly 1 TypicalErrorSummary.
   973	    #[test]
   974	    fn cluster_autopsies_three_same_class_emits_one() {
   975	        let autopsies = vec![
   976	            mk_autopsy("A", LossReasonClass::Bankruptcy, 0xAA),
   977	            mk_autopsy("B", LossReasonClass::Bankruptcy, 0xBB),
   978	            mk_autopsy("C", LossReasonClass::Bankruptcy, 0xCC),
   979	        ];
   980	        let summaries = cluster_autopsies(&autopsies, 3);
   981	        assert_eq!(summaries.len(), 1);
   982	        assert_eq!(summaries[0].count, 3);
   983	        assert_eq!(summaries[0].loss_reason_class, LossReasonClass::Bankruptcy);
   984	        assert_eq!(summaries[0].exemplar_capsule_cids.len(), 3);
   985	    }
   986	
   987	    /// TB-15 Atom 4 — 2 same-class autopsies → 0 broadcasts (below
   988	    /// threshold).
   989	    #[test]
   990	    fn cluster_autopsies_two_same_class_emits_zero() {
   991	        let autopsies = vec![
   992	            mk_autopsy("A", LossReasonClass::Bankruptcy, 0xAA),
   993	            mk_autopsy("B", LossReasonClass::Bankruptcy, 0xBB),
   994	        ];
   995	        let summaries = cluster_autopsies(&autopsies, 3);
   996	        assert_eq!(summaries.len(), 0);
   997	    }
   998	
   999	    /// TB-15 Atom 4 — mixed classes: only ones with count >= threshold
  1000	    /// emerge; ordering deterministic (BTreeMap by class tag).
  1001	    #[test]
  1002	    fn cluster_autopsies_mixed_classes_filters_below_threshold() {
  1003	        let autopsies = vec![
  1004	            mk_autopsy("A", LossReasonClass::Bankruptcy, 0xAA),
  1005	            mk_autopsy("B", LossReasonClass::Bankruptcy, 0xBB),
  1006	            mk_autopsy("C", LossReasonClass::Bankruptcy, 0xCC),
  1007	            mk_autopsy("D", LossReasonClass::SlashLoss, 0xDD),
  1008	            mk_autopsy("E", LossReasonClass::SlashLoss, 0xEE),
  1009	            mk_autopsy("F", LossReasonClass::SlashLoss, 0xFF),
  1010	            mk_autopsy("G", LossReasonClass::SlashLoss, 0x11),
  1011	            mk_autopsy("H", LossReasonClass::Goodhart, 0x22),
  1012	        ];
  1013	        let summaries = cluster_autopsies(&autopsies, 3);
  1014	        // Bankruptcy (3) + SlashLoss (4) = 2 broadcasts; Goodhart (1) below threshold.
  1015	        assert_eq!(summaries.len(), 2);
  1016	        let counts: Vec<u32> = summaries.iter().map(|s| s.count).collect();
  1017	        assert!(counts.contains(&3));
  1018	        assert!(counts.contains(&4));
  1019	    }
  1020	
  1021	    /// TB-15 Atom 4 — halt-trigger #5: TypicalErrorSummary serialization
  1022	    /// MUST NOT contain any input private_detail_cid bytes.
  1023	    #[test]
  1024	    fn cluster_autopsies_output_never_embeds_private_detail_bytes() {
  1025	        let priv_bytes = [0x77u8, 0x88u8, 0x99u8];
  1026	        let autopsies = vec![
  1027	            mk_autopsy("A", LossReasonClass::Bankruptcy, priv_bytes[0]),
  1028	            mk_autopsy("B", LossReasonClass::Bankruptcy, priv_bytes[1]),
  1029	            mk_autopsy("C", LossReasonClass::Bankruptcy, priv_bytes[2]),
  1030	        ];
  1031	        let summaries = cluster_autopsies(&autopsies, 3);
  1032	        let bytes = serde_json::to_vec(&summaries).expect("serialize summaries");
  1033	        for &priv_byte in &priv_bytes {
  1034	            // Each Cid is 32 identical bytes; checking for any 32-byte run.
  1035	            let private_cid = [priv_byte; 32];
  1036	            for window in bytes.windows(32) {
  1037	                assert!(
  1038	                    window != private_cid,
  1039	                    "halt-trigger #5: TypicalErrorSummary serialization contains \
  1040	                     private_detail_cid byte run for byte=0x{:02x}",
  1041	                    priv_byte
  1042	                );
  1043	            }
  1044	        }
  1045	    }
  1046	
  1047	    /// TB-15 Atom 4 — empty input → empty output (no panic).
  1048	    #[test]
  1049	    fn cluster_autopsies_empty_input() {
  1050	        let summaries = cluster_autopsies(&[], 3);
  1051	        assert!(summaries.is_empty());
  1052	    }
  1053	
  1054	    // ───────────────────────────────────────────────────────────────────
  1055	    // R2 closure — activation gate tests (Gemini R1 VETO Q12)
  1056	    // ───────────────────────────────────────────────────────────────────
  1057	
  1058	    /// R2 closure: activation gate predicate is true at default
  1059	    /// constant (TB15_AUTOPSY_ACTIVATION_LOGICAL_T = 0); fresh chains
  1060	    /// (any timestamp_logical >= 0) trivially satisfy the gate.
  1061	    #[test]
  1062	    fn activation_gate_default_is_always_active_for_fresh_chains() {
  1063	        // Default constant is 0; any u64 (including 0 itself) is >= 0.
  1064	        assert!(is_autopsy_active_at(0), "logical_t 0 must be active under default const 0");
  1065	        assert!(is_autopsy_active_at(1), "logical_t 1 must be active");
  1066	        assert!(is_autopsy_active_at(u64::MAX), "logical_t MAX must be active");
  1067	        // Documentation: TB15_AUTOPSY_ACTIVATION_LOGICAL_T == 0 is the
  1068	        // shipped default; pre-TB-15 chain migration would override the
  1069	        // const to a non-zero cutoff.
  1070	        assert_eq!(
  1071	            TB15_AUTOPSY_ACTIVATION_LOGICAL_T, 0,
  1072	            "shipped default must be 0 (always-active for fresh chains)"
  1073	        );
  1074	    }
  1075	
  1076	    /// TB-15 Atom 3 — write_bankruptcy_autopsies_to_cas: writes
  1077	    /// 2 CAS objects per staker (capsule + private_detail). Returned
  1078	    /// Cids match the dispatch arm's deterministic derivation.
  1079	    #[test]
  1080	    fn write_bankruptcy_autopsies_to_cas_round_trip() {
  1081	        use std::sync::{Arc, RwLock};
  1082	        use tempfile::TempDir;
  1083	
  1084	        let task = "task:tb15:cas_writeback";
  1085	        let econ = synthetic_econ_with_stakes(
  1086	            task,
  1087	            &[("stake_w1", "Agent_W1", 100), ("stake_w2", "Agent_W2", 200)],
  1088	        );
  1089	        let bk = synthetic_bk(task);
  1090	
  1091	        let tmp = TempDir::new().expect("tempdir");
  1092	        let cas = Arc::new(RwLock::new(
  1093	            crate::bottom_white::cas::store::CasStore::open(tmp.path()).expect("cas"),
  1094	        ));
  1095	
  1096	        let cids = write_bankruptcy_autopsies_to_cas(
  1097	            &cas,
  1098	            &econ,
  1099	            &bk,
  1100	            7,
  1101	            42,
  1102	            "tb15-test-writer",
  1103	        )
  1104	        .expect("write succeeds");
  1105	
  1106	        assert_eq!(cids.len(), 2);
  1107	
  1108	        // Cids match what derive returns (replay-determinism contract).
  1109	        let derived = derive_autopsies_for_bankruptcy(&econ, &bk, 7, 42);
  1110	        let derived_cids: Vec<Cid> = derived.iter().map(|d| d.capsule.capsule_id).collect();
  1111	        assert_eq!(cids, derived_cids);
  1112	
  1113	        // CAS now contains 4 objects per 2 stakers: 2 private_detail + 2 capsule.
  1114	        let cas_r = cas.read().expect("cas read");
  1115	        assert_eq!(
  1116	            cas_r.len(),
  1117	            4,
  1118	            "2 stakers × 2 CAS objects (private_detail + capsule) = 4"
  1119	        );
  1120	

exec
/bin/bash -lc "nl -ba src/runtime/autopsy_capsule.rs | sed -n '1120,1205p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1120	
  1121	        // R3 closure (Codex R2 VETO TB15-CAS-ID): cas.get(&capsule.capsule_id)
  1122	        // MUST succeed for every emitted Cid. This is the contract Codex
  1123	        // R2 found broken in the prior implementation.
  1124	        for cid in &cids {
  1125	            let bytes = cas_r
  1126	                .get(cid)
  1127	                .expect("R3 contract: cas.get(&capsule_id) MUST succeed");
  1128	            // The retrieved bytes' sha256 MUST equal the cid (CAS
  1129	            // content-addressed integrity).
  1130	            assert_eq!(
  1131	                Cid::from_content(&bytes),
  1132	                *cid,
  1133	                "R3 contract: sha256(retrieved bytes) == capsule_id"
  1134	            );
  1135	            // The retrieved bytes canonical_decode to a struct with the
  1136	            // same field values (modulo capsule_id/sha256 which are
  1137	            // derived; restored via restore_autopsy_capsule_from_cas_bytes).
  1138	            let restored = restore_autopsy_capsule_from_cas_bytes(&bytes)
  1139	                .expect("R3 contract: canonical_decode + restore succeeds");
  1140	            assert_eq!(
  1141	                restored.capsule_id, *cid,
  1142	                "R3 contract: restored capsule_id matches CAS Cid"
  1143	            );
  1144	        }
  1145	    }
  1146	}

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '1340,1405p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1340	            };
  1341	            // Step 2: idempotency + lifecycle gate.
  1342	            match task_entry.state {
  1343	                crate::state::q_state::TaskMarketState::Open
  1344	                | crate::state::q_state::TaskMarketState::Expired => { /* proceed */ }
  1345	                crate::state::q_state::TaskMarketState::Bankrupt => {
  1346	                    return Err(TransitionError::TaskAlreadyOpen);
  1347	                    // Idempotent re-bankruptcy refused; reuse TaskAlreadyOpen
  1348	                    // pending dedicated TaskAlreadyBankrupt variant.
  1349	                }
  1350	                crate::state::q_state::TaskMarketState::Finalized => {
  1351	                    return Err(TransitionError::ClaimAlreadyFinalized);
  1352	                }
  1353	            }
  1354	            // Step 3: q_next — flip state to Bankrupt + record bankruptcy_at_logical_t.
  1355	            let mut q_next = q.clone();
  1356	            if let Some(tm) = q_next.economic_state_t.task_markets_t.0.get_mut(&bk.task_id) {
  1357	                tm.state = crate::state::q_state::TaskMarketState::Bankrupt;
  1358	                tm.bankruptcy_at_logical_t = Some(bk.timestamp_logical);
  1359	            }
  1360	            // Step 3.5 — TB-15 Atom 3 (architect §6.2) + R2 closure
  1361	            // (Gemini R1 VETO Q12 activation gate for replay-determinism):
  1362	            // emit deterministic AgentAutopsyCapsule Cids into
  1363	            // agent_autopsies_t for each staker losing on the bankrupted
  1364	            // task IFF the bankruptcy timestamp_logical is at or past the
  1365	            // TB-15 activation cutoff. PURE: no CAS write here —
  1366	            // apply_one's post-dispatch hook (Stage 3.5) writes the bytes
  1367	            // using the same `derive_autopsies_for_bankruptcy` helper
  1368	            // (replay-safe identical Cids). CR-15.1 + halt-trigger #1:
  1369	            // Cids are NOT projected to AgentVisibleProjection.
  1370	            // `is_autopsy_active_at` defaults true for fresh chains
  1371	            // (TB15_AUTOPSY_ACTIVATION_LOGICAL_T=0); pre-TB-15 chain
  1372	            // migration would override the constant to skip pre-cutoff
  1373	            // rows per `feedback_no_retroactive_evidence_rewrite`.
  1374	            if crate::runtime::autopsy_capsule::is_autopsy_active_at(bk.timestamp_logical) {
  1375	                let derived =
  1376	                    crate::runtime::autopsy_capsule::derive_autopsies_for_bankruptcy(
  1377	                        &q.economic_state_t,
  1378	                        bk,
  1379	                        q.q_t.current_round,
  1380	                        bk.timestamp_logical,
  1381	                    );
  1382	                if !derived.is_empty() {
  1383	                    let event_id = crate::state::typed_tx::EventId(bk.task_id.clone());
  1384	                    let entry = q_next
  1385	                        .economic_state_t
  1386	                        .agent_autopsies_t
  1387	                        .0
  1388	                        .entry(event_id)
  1389	                        .or_default();
  1390	                    for d in &derived {
  1391	                        entry.push(d.capsule.capsule_id);
  1392	                    }
  1393	                }
  1394	            }
  1395	            // Step 4: monetary invariants. No money moved.
  1396	            assert_no_post_init_mint(tx, q)
  1397	                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1398	            assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])
  1399	                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1400	            // Step 5: state_root advance.
  1401	            q_next.state_root_t = task_bankruptcy_accept_state_root(&q.state_root_t, tx);
  1402	            Ok((q_next, SignalBundle::empty()))
  1403	        }
  1404	        // ──────────────────────────────────────────────────────────────────
  1405	        // TB-5 Atom 5+6 — ChallengeResolve arm (charter v2 § 4.6 +

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '3075,3125p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  3075	            cas_w.put(
  3076	                &payload_bytes,
  3077	                ObjectType::ProposalPayload,
  3078	                &format!("sequencer-epoch-{}", self.epoch.get()),
  3079	                logical_t,
  3080	                Some("TypedTx.v1".to_string()),
  3081	            )?
  3082	        };
  3083	
  3084	        // Stage 3.5 — TB-15 Atom 3 (architect §6.2): post-dispatch autopsy
  3085	        // CAS-write hook. For accepted TaskBankruptcyTx, derive the same
  3086	        // capsules the dispatch arm pushed Cids for + write their bytes
  3087	        // (capsule + private_detail) to CAS. Idempotent: identical bytes
  3088	        // → identical Cids → CAS dedupe. Replay-safe: re-running this
  3089	        // produces the same CAS state. Failure here is a hard error
  3090	        // (ApplyError) — autopsy bytes MUST be retrievable for SG-15.6
  3091	        // dashboard regenerability.
  3092	        // R2 closure (Gemini R1 VETO Q12): activation-gate the CAS write
  3093	        // identically to the dispatch arm. Both gates pin on the same
  3094	        // constant TB15_AUTOPSY_ACTIVATION_LOGICAL_T → dispatch and
  3095	        // apply_one stay agreement-locked: pre-cutoff rows write nothing
  3096	        // to CAS AND populate no agent_autopsies_t Cids.
  3097	        if let TypedTx::TaskBankruptcy(bk) = &tx {
  3098	            if crate::runtime::autopsy_capsule::is_autopsy_active_at(bk.timestamp_logical) {
  3099	                let _ = crate::runtime::autopsy_capsule::write_bankruptcy_autopsies_to_cas(
  3100	                    &self.cas,
  3101	                    &q_snapshot.economic_state_t,
  3102	                    bk,
  3103	                    q_snapshot.q_t.current_round,
  3104	                    bk.timestamp_logical,
  3105	                    &format!("sequencer-epoch-{}", self.epoch.get()),
  3106	                )
  3107	                .map_err(|e| match e {
  3108	                    crate::runtime::autopsy_capsule::AutopsyWriteError::Cas(c) => {
  3109	                        ApplyError::Cas(c)
  3110	                    }
  3111	                    crate::runtime::autopsy_capsule::AutopsyWriteError::Encode(s) => {
  3112	                        ApplyError::PayloadEncode(s)
  3113	                    }
  3114	                    crate::runtime::autopsy_capsule::AutopsyWriteError::InternalLockPoisoned => {
  3115	                        ApplyError::QStateLockPoisoned
  3116	                    }
  3117	                })?;
  3118	            }
  3119	        }
  3120	
  3121	        // Stage 5: build LedgerEntrySigningPayload (v1.1 — stage 4 fetch_add
  3122	        // moved to AFTER stage 9 commit success).
  3123	        let signing_payload = LedgerEntrySigningPayload {
  3124	            logical_t,
  3125	            parent_state_root: q_snapshot.state_root_t,

exec
/bin/bash -lc "nl -ba src/runtime/evidence_capsule.rs | sed -n '180,300p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   180	impl From<crate::bottom_white::cas::store::CasError> for CapsuleWriteError {
   181	    fn from(e: crate::bottom_white::cas::store::CasError) -> Self {
   182	        Self::Cas(e)
   183	    }
   184	}
   185	
   186	/// TRACE_MATRIX TB-11 Atom 3 (architect §6.1): write an EvidenceCapsule to
   187	/// CAS. The flow:
   188	///
   189	/// 1. Compute sha256 of raw run log → write to CAS as `CompressedRunLog`.
   190	///    (TB-11 MVP stores **uncompressed** raw bytes; gzip wrapping is
   191	///    forward-compat in TB-15 Markov Loom. The Cid is still unique;
   192	///    audit access still requires `privacy_policy: AuditOnly`.)
   193	/// 2. Build minimal JSON manifest enumerating compressed_log_cid +
   194	///    size_bytes + sha256 → write to CAS as `EvidenceManifest`.
   195	/// 3. Build the `EvidenceCapsule` struct with `capsule_id =
   196	///    Cid::default()` (placeholder). Canonical-encode + sha256 → that's
   197	///    the eventual `capsule_id`.
   198	/// 4. Re-create the struct with `capsule_id` filled in + write to CAS as
   199	///    `EvidenceCapsule`.
   200	///
   201	/// Returns the populated `EvidenceCapsule` (with `capsule_id` set).
   202	///
   203	/// **Privacy** (architect §6.1 屏蔽规则): the capsule struct itself
   204	/// includes `public_summary` (broadcast-eligible) + `compressed_log_cid`
   205	/// (the audit-only handle). Caller controls `privacy_policy` at the call
   206	/// site; `AuditOnly` is the recommended default and is enforced
   207	/// elsewhere (dashboard, agent read view).
   208	pub fn write_evidence_capsule(
   209	    cas: &std::sync::Arc<std::sync::RwLock<CasStore>>,
   210	    run_id: RunId,
   211	    task_id: TaskId,
   212	    solver_agent: Option<crate::state::q_state::AgentId>,
   213	    counts: ExhaustionCounts,
   214	    rounds: (u64, u64),
   215	    terminal_reason: ExhaustionReason,
   216	    raw_log_bytes: &[u8],
   217	    privacy: CapsulePrivacyPolicy,
   218	    creator_str: &str,
   219	    created_at_logical_t: u64,
   220	) -> Result<EvidenceCapsule, CapsuleWriteError> {
   221	    // Step 1: write raw log to CAS (uncompressed for TB-11 MVP).
   222	    let mut cas_w = cas
   223	        .write()
   224	        .map_err(|_| CapsuleWriteError::InternalLockPoisoned)?;
   225	    let compressed_log_cid = cas_w.put(
   226	        raw_log_bytes,
   227	        ObjectType::CompressedRunLog,
   228	        creator_str,
   229	        created_at_logical_t,
   230	        Some("v1/evidence_capsule_raw_log".into()),
   231	    )?;
   232	    // Step 2: build + write manifest JSON.
   233	    let manifest_json = serde_json::json!({
   234	        "schema_version": "v1/evidence_manifest",
   235	        "compressed_log_cid": compressed_log_cid.hex(),
   236	        "size_bytes_uncompressed": raw_log_bytes.len() as u64,
   237	        "size_bytes_stored": raw_log_bytes.len() as u64,
   238	        "compression": "none-tb11-mvp",
   239	    });
   240	    let manifest_bytes = serde_json::to_vec(&manifest_json)
   241	        .map_err(|e| CapsuleWriteError::Encode(format!("manifest encode: {e}")))?;
   242	    let evidence_manifest_cid = cas_w.put(
   243	        &manifest_bytes,
   244	        ObjectType::EvidenceManifest,
   245	        creator_str,
   246	        created_at_logical_t,
   247	        Some("v1/evidence_manifest".into()),
   248	    )?;
   249	
   250	    // Step 3: build capsule with sha256 = 0 + capsule_id = 0; canonical
   251	    // encode; sha256 of that is the eventual capsule_id.
   252	    let public_summary = EvidenceCapsule::format_public_summary(&counts, terminal_reason);
   253	    let mut capsule = EvidenceCapsule {
   254	        capsule_id: Cid::default(),
   255	        run_id: run_id.clone(),
   256	        task_id: task_id.clone(),
   257	        solver_agent: solver_agent.clone(),
   258	        attempt_count: counts.attempt_count,
   259	        lean_error_count: counts.lean_error_count,
   260	        sorry_block_count: counts.sorry_block_count,
   261	        protocol_parse_failure_count: counts.protocol_parse_failure_count,
   262	        partial_accept_count: counts.partial_accept_count,
   263	        started_at_round: rounds.0,
   264	        ended_at_round: rounds.1,
   265	        terminal_reason,
   266	        public_summary,
   267	        evidence_manifest_cid,
   268	        compressed_log_cid,
   269	        privacy_policy: privacy,
   270	        sha256: crate::state::q_state::Hash::ZERO,
   271	    };
   272	    let prelim_bytes = canonical_encode(&capsule)
   273	        .map_err(|e| CapsuleWriteError::Encode(format!("capsule prelim encode: {e:?}")))?;
   274	    let capsule_cid = Cid::from_content(&prelim_bytes);
   275	    capsule.capsule_id = capsule_cid;
   276	    capsule.sha256 = crate::state::q_state::Hash(capsule_cid.0);
   277	
   278	    // Step 4: write the canonical-encoded capsule (with capsule_id +
   279	    // sha256 filled in) to CAS as the EvidenceCapsule object.
   280	    let final_bytes = canonical_encode(&capsule)
   281	        .map_err(|e| CapsuleWriteError::Encode(format!("capsule final encode: {e:?}")))?;
   282	    let _ = cas_w.put(
   283	        &final_bytes,
   284	        ObjectType::EvidenceCapsule,
   285	        creator_str,
   286	        created_at_logical_t,
   287	        Some("v1/evidence_capsule".into()),
   288	    )?;
   289	
   290	    Ok(capsule)
   291	}
   292	
   293	#[cfg(test)]
   294	mod tests {
   295	    use super::*;
   296	
   297	    /// TB-11 U1: EvidenceCapsule default round-trips through canonical bytes.
   298	    #[test]
   299	    fn evidence_capsule_default_round_trip() {
   300	        use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};

codex
The R3 Markov and autopsy writer patterns now match the intended “store the zeroed bytes, derive CID from exactly those bytes” contract. I’m checking the evidence-capsule cross-cut and the actual generated evidence/index next, because that’s where a residual ship decision could still matter.
exec
/bin/bash -lc 'for p in /AGENTS.md /home/AGENTS.md /home/zephryj/AGENTS.md /home/zephryj/projects/AGENTS.md /home/zephryj/projects/turingosv4/AGENTS.md; do if [ -f "$p" ]; then printf '"'%s\\n' \""'$p"; sed -n '"'1,220p' \""'$p"; fi; done' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
exec
/bin/bash -lc "nl -ba src/runtime/evidence_capsule.rs | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-11 Atom 1 — `EvidenceCapsule` schema (architect §6.1, ruling
     2	//! 2026-05-02).
     3	//!
     4	//! O(1) chain cost, O(N) auditability. The chain anchors a single
     5	//! `evidence_capsule_cid: Cid` on `TerminalSummaryTx` (architect's
     6	//! RunExhaustedTx) or `TaskBankruptcyTx`; the capsule itself, plus its
     7	//! manifest and compressed run log, live in CAS. Privacy default
     8	//! `CapsulePrivacyPolicy::AuditOnly` — only `public_summary` surfaces
     9	//! to non-audit views per architect §6.1 屏蔽规则.
    10	//!
    11	//! The writer (Atom 3) lives in this module too, so this file is the
    12	//! complete surface for capsule production.
    13	//!
    14	//! TRACE_MATRIX FC3-N1 + Art. 0.2 (Tape Canonical: capsule canonical bytes
    15	//! are themselves the CAS object referenced by `capsule_id`).
    16	//!
    17	//! /// TRACE_MATRIX architect §6.1 ruling 2026-05-02: EvidenceCapsule schema.
    18	
    19	use serde::{Deserialize, Serialize};
    20	
    21	use crate::bottom_white::cas::schema::Cid;
    22	use crate::state::q_state::{AgentId, Hash, TaskId};
    23	use crate::state::typed_tx::{CapsulePrivacyPolicy, ExhaustionReason, RunId};
    24	
    25	/// TRACE_MATRIX TB-11 (architect §6.1 ruling 2026-05-02) — CAS-resident
    26	/// evidence rollup for a failed evaluator run.
    27	///
    28	/// The struct is canonical-encoded into CAS; `capsule_id` is the Cid of
    29	/// those bytes and is set by the writer (Atom 3). For Atom 1, only the
    30	/// schema + Default fixture exist.
    31	///
    32	/// **Privacy** (architect §6.1 屏蔽规则):
    33	/// - `public_summary`: low-information string surface; can enter dashboard /
    34	///   broadcast.
    35	/// - `evidence_manifest_cid`: JSON manifest enumerating sub-CAS objects.
    36	/// - `compressed_log_cid`: gzipped raw run log; access requires the
    37	///   capsule's `privacy_policy` to permit the requesting role
    38	///   (`AuditOnly` blocks default Agent reads).
    39	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    40	pub struct EvidenceCapsule {
    41	    /// CAS Cid of the canonical-encoded EvidenceCapsule itself. Set by the
    42	    /// writer post-encode; before-set value is `Cid::default()` (32 zero
    43	    /// bytes) — the writer canonical-encodes the struct with this field
    44	    /// zeroed, takes the sha256, and returns a fresh struct with the
    45	    /// resulting Cid filled in. (Future TB may make this a non-stored
    46	    /// derivative, but for TB-11 we keep it as a stored field for ease
    47	    /// of replay.)
    48	    pub capsule_id: Cid,
    49	
    50	    /// Backref to the run.
    51	    pub run_id: RunId,
    52	    /// Backref to the task.
    53	    pub task_id: TaskId,
    54	    /// Owner of the failed run, if any (None when no solver was assigned).
    55	    pub solver_agent: Option<AgentId>,
    56	
    57	    // ── Architect §6.1 mandated counts ───────────────────────────────────
    58	    pub attempt_count: u64,
    59	    pub lean_error_count: u64,
    60	    pub sorry_block_count: u64,
    61	    pub protocol_parse_failure_count: u64,
    62	    pub partial_accept_count: u64,
    63	
    64	    /// First logical_t observed in the run.
    65	    pub started_at_round: u64,
    66	    /// Last logical_t observed.
    67	    pub ended_at_round: u64,
    68	    /// Architect §6.1: terminal failure mode.
    69	    pub terminal_reason: ExhaustionReason,
    70	
    71	    // ── Architect §6.1 mandated content ──────────────────────────────────
    72	    /// Low-pollution one-line summary surfaced to dashboard / broadcast.
    73	    pub public_summary: String,
    74	    /// JSON manifest enumerating sub-CAS objects (compressed log Cid +
    75	    /// size + sha256). Stored separately so the capsule itself stays small.
    76	    pub evidence_manifest_cid: Cid,
    77	    /// CAS Cid of the gzipped raw run log. Access requires
    78	    /// `privacy_policy` to permit the requesting role.
    79	    pub compressed_log_cid: Cid,
    80	
    81	    /// Architect §6.1 屏蔽规则 — privacy default `AuditOnly`.
    82	    pub privacy_policy: CapsulePrivacyPolicy,
    83	
    84	    /// SHA-256 of the canonical-encoded capsule bytes (with `capsule_id`
    85	    /// zeroed during the hash). Defense-in-depth duplicate of `capsule_id`.
    86	    pub sha256: Hash,
    87	}
    88	
    89	impl Default for EvidenceCapsule {
    90	    fn default() -> Self {
    91	        Self {
    92	            capsule_id: Cid::default(),
    93	            run_id: RunId::default(),
    94	            task_id: TaskId::default(),
    95	            solver_agent: None,
    96	            attempt_count: 0,
    97	            lean_error_count: 0,
    98	            sorry_block_count: 0,
    99	            protocol_parse_failure_count: 0,
   100	            partial_accept_count: 0,
   101	            started_at_round: 0,
   102	            ended_at_round: 0,
   103	            terminal_reason: ExhaustionReason::default(),
   104	            public_summary: String::new(),
   105	            evidence_manifest_cid: Cid::default(),
   106	            compressed_log_cid: Cid::default(),
   107	            privacy_policy: CapsulePrivacyPolicy::default(),
   108	            sha256: Hash::ZERO,
   109	        }
   110	    }
   111	}
   112	
   113	/// TRACE_MATRIX TB-11 — counts surface for the writer API. The writer
   114	/// (Atom 3) takes this struct + raw log bytes and produces an
   115	/// `EvidenceCapsule` written to CAS.
   116	#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
   117	pub struct ExhaustionCounts {
   118	    pub attempt_count: u64,
   119	    pub lean_error_count: u64,
   120	    pub sorry_block_count: u64,
   121	    pub protocol_parse_failure_count: u64,
   122	    pub partial_accept_count: u64,
   123	}
   124	
   125	impl EvidenceCapsule {
   126	    /// TRACE_MATRIX architect §6.1 — formats the architect-mandated
   127	    /// counts into a public_summary string. Used by the writer to fill
   128	    /// `public_summary` in a deterministic, low-pollution shape.
   129	    ///
   130	    /// Example:
   131	    /// ```text
   132	    /// "132 attempts; 73 lean errors; 14 sorry-blocks; 26 parse failures; 32 partial accepts; reason=MaxTxExhausted; no accepted proof"
   133	    /// ```
   134	    pub fn format_public_summary(
   135	        counts: &ExhaustionCounts,
   136	        terminal_reason: ExhaustionReason,
   137	    ) -> String {
   138	        format!(
   139	            "{} attempts; {} lean errors; {} sorry-blocks; {} parse failures; \
   140	             {} partial accepts; reason={:?}; no accepted proof",
   141	            counts.attempt_count,
   142	            counts.lean_error_count,
   143	            counts.sorry_block_count,
   144	            counts.protocol_parse_failure_count,
   145	            counts.partial_accept_count,
   146	            terminal_reason,
   147	        )
   148	    }
   149	}
   150	
   151	// ────────────────────────────────────────────────────────────────────────────
   152	// TB-11 Atom 3 — EvidenceCapsule writer (architect §6.1)
   153	// ────────────────────────────────────────────────────────────────────────────
   154	
   155	use crate::bottom_white::cas::schema::ObjectType;
   156	use crate::bottom_white::cas::store::CasStore;
   157	use crate::bottom_white::ledger::transition_ledger::canonical_encode;
   158	// TaskId already imported via the schema-section `use` statement above.
   159	
   160	/// TRACE_MATRIX TB-11 Atom 3 (architect §6.1 ruling 2026-05-02): error
   161	/// taxonomy for the EvidenceCapsule writer.
   162	#[derive(Debug)]
   163	pub enum CapsuleWriteError {
   164	    Cas(crate::bottom_white::cas::store::CasError),
   165	    Encode(String),
   166	    InternalLockPoisoned,
   167	}
   168	
   169	impl std::fmt::Display for CapsuleWriteError {
   170	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   171	        match self {
   172	            Self::Cas(e) => write!(f, "cas write failed: {e}"),
   173	            Self::Encode(s) => write!(f, "encode failed: {s}"),
   174	            Self::InternalLockPoisoned => write!(f, "internal lock poisoned"),
   175	        }
   176	    }
   177	}
   178	impl std::error::Error for CapsuleWriteError {}
   179	
   180	impl From<crate::bottom_white::cas::store::CasError> for CapsuleWriteError {

exec
/bin/bash -lc "nl -ba src/runtime/evidence_capsule.rs | sed -n '300,470p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   300	        use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
   301	        let c = EvidenceCapsule::default();
   302	        let bytes = canonical_encode(&c).expect("encode");
   303	        let back: EvidenceCapsule = canonical_decode(&bytes).expect("decode");
   304	        assert_eq!(c, back);
   305	    }
   306	
   307	    /// TB-11 U2: format_public_summary embeds all 5 architect-mandated counts.
   308	    #[test]
   309	    fn format_public_summary_contains_all_counts() {
   310	        let counts = ExhaustionCounts {
   311	            attempt_count: 132,
   312	            lean_error_count: 73,
   313	            sorry_block_count: 14,
   314	            protocol_parse_failure_count: 26,
   315	            partial_accept_count: 32,
   316	        };
   317	        let s = EvidenceCapsule::format_public_summary(&counts, ExhaustionReason::MaxTxExhausted);
   318	        assert!(s.contains("132"));
   319	        assert!(s.contains("73"));
   320	        assert!(s.contains("14"));
   321	        assert!(s.contains("26"));
   322	        assert!(s.contains("32"));
   323	        assert!(s.contains("MaxTxExhausted"));
   324	    }
   325	
   326	    /// TB-11 U3: privacy_policy default is AuditOnly per architect §6.1
   327	    /// 屏蔽规则.
   328	    #[test]
   329	    fn privacy_policy_default_is_audit_only() {
   330	        let c = EvidenceCapsule::default();
   331	        assert_eq!(c.privacy_policy, CapsulePrivacyPolicy::AuditOnly);
   332	    }
   333	
   334	    /// TB-11 Atom 3 — Writer: writes raw log + manifest + capsule to CAS;
   335	    /// returned capsule has populated capsule_id (Cid of canonical bytes).
   336	    #[test]
   337	    fn write_evidence_capsule_to_cas_round_trip() {
   338	        use std::sync::{Arc, RwLock};
   339	        use tempfile::TempDir;
   340	
   341	        let tmp = TempDir::new().expect("tempdir");
   342	        let cas = Arc::new(RwLock::new(
   343	            crate::bottom_white::cas::store::CasStore::open(tmp.path()).expect("cas"),
   344	        ));
   345	
   346	        let counts = ExhaustionCounts {
   347	            attempt_count: 132,
   348	            lean_error_count: 73,
   349	            sorry_block_count: 14,
   350	            protocol_parse_failure_count: 26,
   351	            partial_accept_count: 32,
   352	        };
   353	        let raw_log = b"FAKE_RUN_LOG\n[attempt 1]: lean error\n[attempt 132]: max-tx exhausted\n";
   354	
   355	        let capsule = write_evidence_capsule(
   356	            &cas,
   357	            RunId("run-zeta-001".into()),
   358	            crate::state::q_state::TaskId(
   359	                "task:lean:heldout_49:zeta_regularization".into(),
   360	            ),
   361	            Some(crate::state::q_state::AgentId("Agent_solver_0".into())),
   362	            counts,
   363	            (0, 1300),
   364	            ExhaustionReason::MaxTxExhausted,
   365	            raw_log,
   366	            CapsulePrivacyPolicy::AuditOnly,
   367	            "evaluator-tb11",
   368	            1,
   369	        )
   370	        .expect("writer succeeds");
   371	
   372	        // capsule_id populated and matches sha256.
   373	        assert_ne!(capsule.capsule_id, Cid::default());
   374	        assert_eq!(capsule.capsule_id.0, capsule.sha256.0);
   375	
   376	        // Counts faithfully recorded.
   377	        assert_eq!(capsule.attempt_count, 132);
   378	        assert_eq!(capsule.lean_error_count, 73);
   379	        assert_eq!(capsule.sorry_block_count, 14);
   380	        assert_eq!(capsule.protocol_parse_failure_count, 26);
   381	        assert_eq!(capsule.partial_accept_count, 32);
   382	        assert_eq!(capsule.terminal_reason, ExhaustionReason::MaxTxExhausted);
   383	
   384	        // public_summary contains all 5 counts + reason.
   385	        assert!(capsule.public_summary.contains("132 attempts"));
   386	        assert!(capsule.public_summary.contains("73 lean errors"));
   387	        assert!(capsule.public_summary.contains("MaxTxExhausted"));
   388	
   389	        // CAS contains 3 objects: raw log + manifest + capsule itself.
   390	        let cas_r = cas.read().expect("cas read");
   391	        assert_eq!(cas_r.len(), 3, "writer puts 3 CAS objects: log + manifest + capsule");
   392	
   393	        // raw log retrievable by compressed_log_cid.
   394	        let retrieved = cas_r.get(&capsule.compressed_log_cid).expect("get raw");
   395	        assert_eq!(retrieved, raw_log);
   396	    }
   397	
   398	    /// TB-11 Atom 3 — Writer: same inputs → same capsule_id (deterministic).
   399	    #[test]
   400	    fn write_evidence_capsule_deterministic_capsule_id() {
   401	        use std::sync::{Arc, RwLock};
   402	        use tempfile::TempDir;
   403	
   404	        let counts = ExhaustionCounts {
   405	            attempt_count: 5,
   406	            lean_error_count: 3,
   407	            sorry_block_count: 1,
   408	            protocol_parse_failure_count: 1,
   409	            partial_accept_count: 0,
   410	        };
   411	        let raw_log = b"deterministic test";
   412	
   413	        let cap_a = {
   414	            let tmp_a = TempDir::new().unwrap();
   415	            let cas_a = Arc::new(RwLock::new(
   416	                crate::bottom_white::cas::store::CasStore::open(tmp_a.path()).unwrap(),
   417	            ));
   418	            write_evidence_capsule(
   419	                &cas_a,
   420	                RunId("run-A".into()),
   421	                crate::state::q_state::TaskId("t-A".into()),
   422	                None,
   423	                counts,
   424	                (10, 20),
   425	                ExhaustionReason::MaxTxExhausted,
   426	                raw_log,
   427	                CapsulePrivacyPolicy::AuditOnly,
   428	                "writer",
   429	                1,
   430	            )
   431	            .expect("writer A")
   432	        };
   433	        let cap_b = {
   434	            let tmp_b = TempDir::new().unwrap();
   435	            let cas_b = Arc::new(RwLock::new(
   436	                crate::bottom_white::cas::store::CasStore::open(tmp_b.path()).unwrap(),
   437	            ));
   438	            write_evidence_capsule(
   439	                &cas_b,
   440	                RunId("run-A".into()),
   441	                crate::state::q_state::TaskId("t-A".into()),
   442	                None,
   443	                counts,
   444	                (10, 20),
   445	                ExhaustionReason::MaxTxExhausted,
   446	                raw_log,
   447	                CapsulePrivacyPolicy::AuditOnly,
   448	                "writer",
   449	                1,
   450	            )
   451	            .expect("writer B")
   452	        };
   453	        assert_eq!(cap_a.capsule_id, cap_b.capsule_id);
   454	        assert_eq!(cap_a.compressed_log_cid, cap_b.compressed_log_cid);
   455	        assert_eq!(cap_a.evidence_manifest_cid, cap_b.evidence_manifest_cid);
   456	    }
   457	}

exec
/bin/bash -lc "rg -n \"EvidenceCapsule|evidence_capsule_cid|write_evidence_capsule|restore_evidence|cas\\.get\\(&.*capsule\" src tests experiments -g '"'!target'"'" in /home/zephryj/projects/turingosv4
 succeeded in 129ms:
tests/fc_alignment_conformance.rs:544:// Markov EvidenceCapsule.
tests/fc_alignment_conformance.rs:649:/// FC3-N43 (TB-15 Atom 5): MarkovEvidenceCapsule + writer + default-deny
tests/fc_alignment_conformance.rs:655:        try_deep_history_read_with_override_check, MarkovEvidenceCapsule, MarkovGenError,
tests/fc_alignment_conformance.rs:659:    let cap = MarkovEvidenceCapsule::with_constitution_hash([0xAB; 32]);
tests/tb_5_system_ingress_barrier.rs:211:        evidence_capsule_cid: None,                                                        // TB-11
tests/tb_5_system_ingress_barrier.rs:269:            evidence_capsule_cid: None,                                                    // TB-11
tests/tb_11_epistemic_exhaust.rs:11://!   I-TB11-1  TerminalSummary anchors RunsIndex with evidence_capsule_cid.
tests/tb_11_epistemic_exhaust.rs:141:            evidence_capsule_cid: Some(capsule_cid),
tests/tb_11_epistemic_exhaust.rs:155:    assert_eq!(entry.evidence_capsule_cid, Some(capsule_cid));
tests/tb_11_epistemic_exhaust.rs:324:    assert_eq!(entry.evidence_capsule_cid, Some(capsule_cid));
tests/tb_11_epistemic_exhaust.rs:339:            evidence_capsule_cid: capsule_cid,

exec
/bin/bash -lc 'find handover/evidence/tb_15_markov_capsule_2026-05-04 -maxdepth 2 -type f | sort' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/evidence/tb_15_markov_capsule_2026-05-04/LATEST_MARKOV_CAPSULE.txt
handover/evidence/tb_15_markov_capsule_2026-05-04/MARKOV_TB-15-R3_2026-05-03.json
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md
handover/evidence/tb_15_markov_capsule_2026-05-04/cas_index.jsonl

exec
/bin/bash -lc 'nl -ba handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312

exec
/bin/bash -lc "nl -ba handover/evidence/tb_15_markov_capsule_2026-05-04/cas_index.jsonl | sed -n '1,40p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	{"cid":[83,59,212,123,234,85,246,138,24,84,67,246,164,110,226,66,202,12,167,153,115,246,202,111,234,149,41,202,26,234,17,56],"backend_oid_hex":"222506adcdb0dbde83a4d4098998e93414a5e21e","object_type":"NextSessionContext","creator":"tb15-generator","created_at_logical_t":0,"schema_id":"v1/next_session_context","size_bytes":830}
     2	{"cid":[249,231,1,180,169,194,225,217,180,209,34,44,6,166,196,228,246,81,106,161,175,28,62,210,154,244,87,209,85,50,211,18],"backend_oid_hex":"816cd91a43db05c77ad89f51c3d26b0b940ea950","object_type":"MarkovEvidenceCapsule","creator":"tb15-generator","created_at_logical_t":0,"schema_id":"v1/markov_evidence_capsule","size_bytes":2136}

exec
/bin/bash -lc "nl -ba handover/markov_capsules/MARKOV_TB-15-R3_2026-05-03.json | sed -n '1,140p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	{
     2	  "capsule_id": [
     3	    249,
     4	    231,
     5	    1,
     6	    180,
     7	    169,
     8	    194,
     9	    225,
    10	    217,
    11	    180,
    12	    209,
    13	    34,
    14	    44,
    15	    6,
    16	    166,
    17	    196,
    18	    228,
    19	    246,
    20	    81,
    21	    106,
    22	    161,
    23	    175,
    24	    28,
    25	    62,
    26	    210,
    27	    154,
    28	    244,
    29	    87,
    30	    209,
    31	    85,
    32	    50,
    33	    211,
    34	    18
    35	  ],
    36	  "previous_capsule_cid": null,
    37	  "constitution_hash": [
    38	    238,
    39	    198,
    40	    149,
    41	    69,
    42	    156,
    43	    113,
    44	    251,
    45	    239,
    46	    54,
    47	    133,
    48	    88,
    49	    52,
    50	    133,
    51	    222,
    52	    180,
    53	    49,
    54	    254,
    55	    59,
    56	    86,
    57	    22,
    58	    87,
    59	    178,
    60	    242,
    61	    133,
    62	    183,
    63	    197,
    64	    231,
    65	    226,
    66	    32,
    67	    229,
    68	    158,
    69	    3
    70	  ],
    71	  "flowchart_hashes": [
    72	    [
    73	      164,
    74	      116,
    75	      198,
    76	      185,
    77	      222,
    78	      215,
    79	      102,
    80	      80,
    81	      74,
    82	      79,
    83	      100,
    84	      74,
    85	      74,
    86	      27,
    87	      60,
    88	      84,
    89	      83,
    90	      22,
    91	      212,
    92	      24,
    93	      240,
    94	      37,
    95	      15,
    96	      54,
    97	      236,
    98	      105,
    99	      47,
   100	      205,
   101	      249,
   102	      143,
   103	      9,
   104	      245
   105	    ],
   106	    [
   107	      184,
   108	      34,
   109	      113,
   110	      123,
   111	      16,
   112	      51,
   113	      42,
   114	      45,
   115	      142,
   116	      120,
   117	      155,
   118	      166,
   119	      175,
   120	      150,
   121	      253,
   122	      77,
   123	      164,
   124	      255,
   125	      67,
   126	      167,
   127	      74,
   128	      250,
   129	      182,
   130	      121,
   131	      209,
   132	      184,
   133	      42,
   134	      221,
   135	      156,
   136	      50,
   137	      182,
   138	      77
   139	    ],
   140	    [

exec
/bin/bash -lc "rg -n 'next_session_context_cid|sha256|tb_tag|created_at_logical_t|l4_root|l4e_root|cas_root|typical_errors|unresolved_obs' handover/markov_capsules/MARKOV_TB-15-R3_2026-05-03.json handover/evidence/tb_15_markov_capsule_2026-05-04/MARKOV_TB-15-R3_2026-05-03.json handover/evidence/tb_15_markov_capsule_2026-05-04/README.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:32:unresolved_obs            : R2 22 → R3 23                       (added OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04.md)
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:33:typical_errors            : []                                  (unchanged)
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:52:The bug was in the writer: `capsule_id = sha256(prelim_bytes)` (with capsule_id+sha256 zeroed during hash) but `cas.put(final_bytes)` stored the post-population bytes. Sha256 of those final bytes ≠ capsule_id.
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:54:R3 fix: store the EXACT prelim bytes in CAS (with capsule_id+sha256 zeroed). `capsule_id = sha256(stored_bytes)`. CAS keys by sha256(stored_bytes) = capsule_id. `cas.get(&capsule_id)` succeeds; consumer canonical-decodes + restores capsule_id+sha256 from `Cid::from_content(&retrieved_bytes)`.
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:60:- `runtime::markov_capsule::tests::write_markov_capsule_cas_resolvable_by_capsule_id` — asserts `cas.get(&cap.capsule_id)` succeeds + retrieved bytes' sha256 equals capsule_id + restore round-trip works.
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:67:| Q3 (CAS residency) | Codex | "fixed" via no-cas drop (didn't actually fix CAS-cid mismatch) | **FIXED**: capsule_id = sha256(stored_bytes); cas.get resolvable |
handover/evidence/tb_15_markov_capsule_2026-05-04/MARKOV_TB-15-R3_2026-05-03.json:209:  "l4_root": [
handover/evidence/tb_15_markov_capsule_2026-05-04/MARKOV_TB-15-R3_2026-05-03.json:243:  "l4e_root": [
handover/evidence/tb_15_markov_capsule_2026-05-04/MARKOV_TB-15-R3_2026-05-03.json:277:  "cas_root": [
handover/evidence/tb_15_markov_capsule_2026-05-04/MARKOV_TB-15-R3_2026-05-03.json:311:  "typical_errors": [],
handover/evidence/tb_15_markov_capsule_2026-05-04/MARKOV_TB-15-R3_2026-05-03.json:312:  "unresolved_obs": [
handover/evidence/tb_15_markov_capsule_2026-05-04/MARKOV_TB-15-R3_2026-05-03.json:337:  "next_session_context_cid": [
handover/evidence/tb_15_markov_capsule_2026-05-04/MARKOV_TB-15-R3_2026-05-03.json:371:  "sha256": [
handover/evidence/tb_15_markov_capsule_2026-05-04/MARKOV_TB-15-R3_2026-05-03.json:405:  "created_at_logical_t": 0,
handover/evidence/tb_15_markov_capsule_2026-05-04/MARKOV_TB-15-R3_2026-05-03.json:406:  "tb_tag": "TB-15-R3"
handover/markov_capsules/MARKOV_TB-15-R3_2026-05-03.json:209:  "l4_root": [
handover/markov_capsules/MARKOV_TB-15-R3_2026-05-03.json:243:  "l4e_root": [
handover/markov_capsules/MARKOV_TB-15-R3_2026-05-03.json:277:  "cas_root": [
handover/markov_capsules/MARKOV_TB-15-R3_2026-05-03.json:311:  "typical_errors": [],
handover/markov_capsules/MARKOV_TB-15-R3_2026-05-03.json:312:  "unresolved_obs": [
handover/markov_capsules/MARKOV_TB-15-R3_2026-05-03.json:337:  "next_session_context_cid": [
handover/markov_capsules/MARKOV_TB-15-R3_2026-05-03.json:371:  "sha256": [
handover/markov_capsules/MARKOV_TB-15-R3_2026-05-03.json:405:  "created_at_logical_t": 0,
handover/markov_capsules/MARKOV_TB-15-R3_2026-05-03.json:406:  "tb_tag": "TB-15-R3"

exec
/bin/bash -lc 'diff -u handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt handover/evidence/tb_15_markov_capsule_2026-05-04/LATEST_MARKOV_CAPSULE.txt' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
exec
/bin/bash -lc "nl -ba handover/evidence/tb_15_markov_capsule_2026-05-04/README.md | sed -n '1,160p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TB-15 R3 Markov Capsule (post recursive dual-audit closure) — 2026-05-04
     2	
     3	**Replaces**: `handover/evidence/tb_15_markov_capsule_2026-05-03/` (Atom 6 ship; `--no-cas`, no flowchart_hashes) AND R2 capsule (CAS-cid mismatch — Codex R2 VETO Q3 + TB15-CAS-ID).
     4	
     5	**TB**: TB-15 — Lamarckian Autopsy + Markov EvidenceCapsule
     6	**Round**: R3 closure (post Codex + Gemini R2 dual audit)
     7	**Closure doc**: `handover/audits/RECURSIVE_AUDIT_TB_15_2026-05-04.md`
     8	
     9	## Generation command
    10	
    11	```bash
    12	mkdir -p /tmp/tb15-r3-cas
    13	cargo run --bin generate_markov_capsule -- \
    14	  --tb-id 15-R3 \
    15	  --out-dir handover/markov_capsules \
    16	  --constitution-path constitution.md \
    17	  --cas-dir /tmp/tb15-r3-cas
    18	```
    19	
    20	## Outputs
    21	
    22	- `MARKOV_TB-15-R3_2026-05-03.json` — R3 MarkovEvidenceCapsule (CAS-resolvable; flowchart_hashes populated)
    23	- `LATEST_MARKOV_CAPSULE.txt` — Cid hex pointer (`f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312`)
    24	- `cas_index.jsonl` — copy of CAS index showing CAS object cid matches LATEST pointer
    25	
    26	## R3 deltas vs R2 capsule
    27	
    28	```text
    29	capsule_id                : R2 a94ae884... → R3 f9e701b4...   (rebuilt with R3 fixed writer)
    30	constitution_hash         : eec69545...                         (unchanged)
    31	flowchart_hashes          : 4 hashes from TRACE_FLOWCHART_MATRIX.md (unchanged)
    32	unresolved_obs            : R2 22 → R3 23                       (added OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04.md)
    33	typical_errors            : []                                  (unchanged)
    34	CAS resolvability         : R2 BROKEN (cid in CAS index ≠ capsule_id) → R3 FIXED
    35	```
    36	
    37	## CAS-resolvability proof (R3 closure of Codex R2 VETO Q3 + TB15-CAS-ID)
    38	
    39	```text
    40	LATEST_MARKOV_CAPSULE.txt   = f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312
    41	CAS index (MarkovEvidenceCapsule) = f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312
    42	
    43	→ MATCH. cas.get(&capsule.capsule_id) is RESOLVABLE. SG-15.3 (next-session
    44	  bootstrap from latest Markov capsule) holds.
    45	```
    46	
    47	R2 (broken) had:
    48	- `LATEST_MARKOV_CAPSULE.txt` = `a94ae884...`
    49	- CAS index for MarkovEvidenceCapsule = `e4932fca...` (different)
    50	- `cas.get(Cid("a94ae884..."))` would fail.
    51	
    52	The bug was in the writer: `capsule_id = sha256(prelim_bytes)` (with capsule_id+sha256 zeroed during hash) but `cas.put(final_bytes)` stored the post-population bytes. Sha256 of those final bytes ≠ capsule_id.
    53	
    54	R3 fix: store the EXACT prelim bytes in CAS (with capsule_id+sha256 zeroed). `capsule_id = sha256(stored_bytes)`. CAS keys by sha256(stored_bytes) = capsule_id. `cas.get(&capsule_id)` succeeds; consumer canonical-decodes + restores capsule_id+sha256 from `Cid::from_content(&retrieved_bytes)`.
    55	
    56	Same fix applied to `write_autopsy_capsule` and `derive_autopsies_for_bankruptcy` / `write_bankruptcy_autopsies_to_cas` (TB15-CAS-ID VETO closure).
    57	
    58	## R3 unit tests (proves the contract)
    59	
    60	- `runtime::markov_capsule::tests::write_markov_capsule_cas_resolvable_by_capsule_id` — asserts `cas.get(&cap.capsule_id)` succeeds + retrieved bytes' sha256 equals capsule_id + restore round-trip works.
    61	- `runtime::autopsy_capsule::tests::write_bankruptcy_autopsies_to_cas_round_trip` — extended with same R3 contract assertions for autopsy capsules.
    62	
    63	## Audit-from-tape closure record (recursive R1 → R2 → R3)
    64	
    65	| Finding | R1 source | R2 status | R3 status |
    66	|---|---|---|---|
    67	| Q3 (CAS residency) | Codex | "fixed" via no-cas drop (didn't actually fix CAS-cid mismatch) | **FIXED**: capsule_id = sha256(stored_bytes); cas.get resolvable |
    68	| Q4 (live override gate) | Codex | FIXED via `--include-prior-capsules` | unchanged |
    69	| Q5 (byte-window scan) | Codex | FIXED via strengthened halt-trigger | unchanged |
    70	| Q7/Q8 (flowchart_hashes) | Both | FIXED via new field + parser | unchanged |
    71	| Q9 (dashboard not regenerable) | Codex | OBS-deferred to TB-16 | unchanged |
    72	| Q12 (replay-determinism) | Gemini VETO | FIXED via activation gate | unchanged |
    73	| **TB15-CAS-ID** (autopsy CAS-cid mismatch) | Codex R2 NEW VETO | n/a | **FIXED**: same writer pattern fix applied to write_autopsy_capsule + derive_autopsies_for_bankruptcy |
    74	
    75	## Open items
    76	
    77	- **OBS-TB15-R2-Q12-UPGRADE** (Gemini R2 recommendation): upgrade compile-time `TB15_AUTOPSY_ACTIVATION_LOGICAL_T` const to a chain-resident marker for improved long-term robustness. Carry-forward to TB-16+.
    78	- **OBS-TB15-R2-Q7-TEST-HARDEN** (Gemini R2 recommendation): add negative-path tests for `read_flowchart_hashes_from_matrix`. Carry-forward.
    79	- **OBS-TB-11-CAS-ID** (cross-cut, NEW R3): TB-11 `write_evidence_capsule` has the SAME CAS-cid mismatch bug. Not blocking TB-15 ship (no production consumer of EvidenceCapsule via cap.capsule_id yet) but should be fixed in TB-11 follow-up.
    80	
    81	## Cross-references
    82	
    83	- TB-15 charter: `handover/tracer_bullets/TB-15_charter_2026-05-03.md`
    84	- TB-15 ship status: `handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md`
    85	- R1 audits: `handover/audits/{CODEX,GEMINI}_TB_15_SHIP_AUDIT_2026-05-04_R1.md`
    86	- R2 audits: `handover/audits/{CODEX,GEMINI}_TB_15_SHIP_AUDIT_2026-05-04_R2.md`
    87	- R3 audits: `handover/audits/{CODEX,GEMINI}_TB_15_SHIP_AUDIT_2026-05-04_R3.md` (pending at evidence-write time)
    88	- R3 closure doc: `handover/audits/RECURSIVE_AUDIT_TB_15_2026-05-04.md`
    89	- Architect spec: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §6
    90	- Source files: `src/runtime/markov_capsule.rs` (write_markov_capsule + restore_markov_capsule_from_cas_bytes), `src/runtime/autopsy_capsule.rs` (write_autopsy_capsule + restore_autopsy_capsule_from_cas_bytes + BankruptcyAutopsyDerivation)

exec
/bin/bash -lc 'find handover/evidence/tb_15_markov_capsule_2026-05-04 -maxdepth 3 -type d -print' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/evidence/tb_15_markov_capsule_2026-05-04

codex
The R3 evidence directory supports the claimed pointer/index match: the latest pointer is `f9e701…`, and the CAS index line for `MarkovEvidenceCapsule` is keyed by the same byte array. I’m going to run the narrow R3 tests with the build output directed outside the repo if the sandbox permits it; if it can’t write build artifacts, I’ll report that limitation.
exec
/bin/bash -lc 'CARGO_TARGET_DIR=/tmp/turingosv4-r3-audit-target cargo test write_markov_capsule_cas_resolvable_by_capsule_id write_bankruptcy_autopsies_to_cas_round_trip' in /home/zephryj/projects/turingosv4
 exited 1 in 112ms:
error: unexpected argument 'write_bankruptcy_autopsies_to_cas_round_trip' found

Usage: cargo test [OPTIONS] [TESTNAME] [-- [ARGS]...]

For more information, try '--help'.

codex
The first test command failed at Cargo argument parsing before building because `cargo test` accepts one name filter at a time. I’ll run the two targeted filters separately so the result is meaningful.
exec
/bin/bash -lc 'CARGO_TARGET_DIR=/tmp/turingosv4-r3-audit-target cargo test write_markov_capsule_cas_resolvable_by_capsule_id' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc 'CARGO_TARGET_DIR=/tmp/turingosv4-r3-audit-target-2 cargo test write_bankruptcy_autopsies_to_cas_round_trip' in /home/zephryj/projects/turingosv4
 exited 101 in 371ms:
error: Read-only file system (os error 30) at path "/tmp/turingosv4-r3-audit-targetwgBHiM"

 exited 101 in 348ms:
error: Read-only file system (os error 30) at path "/tmp/turingosv4-r3-audit-target-2qPohGD"

exec
/bin/bash -lc "nl -ba src/bin/generate_markov_capsule.rs | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 5ms:
     1	//! TB-15 Atom 5 — `generate_markov_capsule` CLI (architect §6.2 +
     2	//! FR-15.4 + FR-15.5).
     3	//!
     4	//! Reads constitution.md → SHA-256; opens the chain runtime_repo + CAS
     5	//! to derive L4 / L4.E / CAS roots; scans `handover/alignment/OBS_*.md`
     6	//! for unresolved-OBS list; clusters CAS-resident
     7	//! AgentAutopsyCapsules into TypicalErrorSummary list (Atom 4
     8	//! `cluster_autopsies`); writes a `MarkovEvidenceCapsule` to CAS +
     9	//! emits a JSON pointer file under `--out-dir`.
    10	//!
    11	//! Default-deny: deeper-history reads (older capsules; L4 rows
    12	//! pre-dating `--prev-cid`'s implied `l4_root`) require
    13	//! `TURINGOS_MARKOV_OVERRIDE=1`. Without override, only the constitution
    14	//! + previous Markov capsule + current chain heads are read.
    15	//!
    16	//! Usage:
    17	//!   generate_markov_capsule \
    18	//!     --tb-id <N> \
    19	//!     --out-dir <path> \
    20	//!     --constitution-path <path> \
    21	//!     --runtime-repo <path> \
    22	//!     --cas-dir <path> \
    23	//!     [--prev-cid-hex <hex>] \
    24	//!     [--alignment-dir <path>] \
    25	//!     [--no-cas]
    26	//!
    27	//! `--no-cas` runs in pointer-only mode (write JSON file but skip CAS
    28	//! put — useful when no runtime CAS is available, e.g. fresh repo).
    29	//!
    30	//! Exit code:
    31	//!   0  — capsule generated + persisted.
    32	//!   1  — generation failed (write error / missing constitution.md).
    33	//!   2  — invalid args.
    34	
    35	use std::path::PathBuf;
    36	use std::sync::{Arc, RwLock};
    37	
    38	use turingosv4::bottom_white::cas::schema::Cid;
    39	use turingosv4::bottom_white::cas::store::CasStore;
    40	use turingosv4::runtime::autopsy_capsule::TypicalErrorSummary;
    41	use turingosv4::runtime::markov_capsule::{
    42	    override_set_from_env, read_flowchart_hashes_from_matrix, scan_unresolved_obs, sha256_of_file,
    43	    try_deep_history_read_with_override_check, write_markov_capsule, MarkovGenError, ObsId,
    44	};
    45	use turingosv4::state::q_state::Hash;
    46	
    47	struct Args {
    48	    tb_id: String,
    49	    out_dir: PathBuf,
    50	    constitution_path: PathBuf,
    51	    flowchart_matrix_path: PathBuf,
    52	    /// v0 placeholder — future TB will read L4 chain head from this path.
    53	    #[allow(dead_code)]
    54	    runtime_repo: Option<PathBuf>,
    55	    cas_dir: Option<PathBuf>,
    56	    prev_cid_hex: Option<String>,
    57	    alignment_dir: PathBuf,
    58	    no_cas: bool,
    59	    /// R2 closure (Codex R1 Q4): when > 0, the binary attempts to read N
    60	    /// prior Markov capsules (deeper than the previous_capsule_cid) — a
    61	    /// LIVE deep-history read path that REQUIRES `TURINGOS_MARKOV_OVERRIDE=1`
    62	    /// per FR-15.5 + halt-trigger #6. Default 0 = no deep-history read.
    63	    include_prior_capsules: u32,
    64	}
    65	
    66	fn parse_args(argv: &[String]) -> Result<Args, String> {
    67	    let mut tb_id: Option<String> = None;
    68	    let mut out_dir: Option<PathBuf> = None;
    69	    let mut constitution_path: Option<PathBuf> = None;
    70	    let mut flowchart_matrix_path: Option<PathBuf> = None;
    71	    let mut runtime_repo: Option<PathBuf> = None;
    72	    let mut cas_dir: Option<PathBuf> = None;
    73	    let mut prev_cid_hex: Option<String> = None;
    74	    let mut alignment_dir: Option<PathBuf> = None;
    75	    let mut no_cas = false;
    76	    let mut include_prior_capsules: u32 = 0;
    77	
    78	    let mut i = 0;
    79	    while i < argv.len() {
    80	        match argv[i].as_str() {
    81	            "--tb-id" => {
    82	                tb_id = argv.get(i + 1).cloned();
    83	                i += 2;
    84	            }
    85	            "--out-dir" => {
    86	                out_dir = argv.get(i + 1).map(PathBuf::from);
    87	                i += 2;
    88	            }
    89	            "--constitution-path" => {
    90	                constitution_path = argv.get(i + 1).map(PathBuf::from);
    91	                i += 2;
    92	            }
    93	            "--flowchart-matrix-path" => {
    94	                flowchart_matrix_path = argv.get(i + 1).map(PathBuf::from);
    95	                i += 2;
    96	            }
    97	            "--runtime-repo" => {
    98	                runtime_repo = argv.get(i + 1).map(PathBuf::from);
    99	                i += 2;
   100	            }
   101	            "--cas-dir" => {
   102	                cas_dir = argv.get(i + 1).map(PathBuf::from);
   103	                i += 2;
   104	            }
   105	            "--prev-cid-hex" => {
   106	                prev_cid_hex = argv.get(i + 1).cloned();
   107	                i += 2;
   108	            }
   109	            "--alignment-dir" => {
   110	                alignment_dir = argv.get(i + 1).map(PathBuf::from);
   111	                i += 2;
   112	            }
   113	            "--no-cas" => {
   114	                no_cas = true;
   115	                i += 1;
   116	            }
   117	            "--include-prior-capsules" => {
   118	                include_prior_capsules = argv
   119	                    .get(i + 1)
   120	                    .ok_or_else(|| "--include-prior-capsules <N> requires arg".to_string())?
   121	                    .parse()
   122	                    .map_err(|e| format!("--include-prior-capsules N parse: {e}"))?;
   123	                i += 2;
   124	            }
   125	            "--help" | "-h" => {
   126	                return Err("help".to_string());
   127	            }
   128	            other => return Err(format!("unrecognized arg: {other}")),
   129	        }
   130	    }
   131	
   132	    Ok(Args {
   133	        tb_id: tb_id.ok_or_else(|| "--tb-id <N> required".to_string())?,
   134	        out_dir: out_dir.ok_or_else(|| "--out-dir <path> required".to_string())?,
   135	        constitution_path: constitution_path
   136	            .ok_or_else(|| "--constitution-path <path> required".to_string())?,
   137	        flowchart_matrix_path: flowchart_matrix_path
   138	            .unwrap_or_else(|| PathBuf::from("handover/alignment/TRACE_FLOWCHART_MATRIX.md")),
   139	        runtime_repo,
   140	        cas_dir,
   141	        prev_cid_hex,
   142	        alignment_dir: alignment_dir
   143	            .unwrap_or_else(|| PathBuf::from("handover/alignment")),
   144	        no_cas,
   145	        include_prior_capsules,
   146	    })
   147	}
   148	
   149	fn print_help() {
   150	    eprintln!(
   151	        "TB-15 generate_markov_capsule — write a MarkovEvidenceCapsule to CAS \
   152	         + JSON pointer.\n\
   153	         \n\
   154	         usage: generate_markov_capsule \\\n\
   155	         \x20  --tb-id <N> \\\n\
   156	         \x20  --out-dir <path> \\\n\
   157	         \x20  --constitution-path <path> \\\n\
   158	         \x20  [--flowchart-matrix-path <path>]                   (default: handover/alignment/TRACE_FLOWCHART_MATRIX.md)\n\
   159	         \x20  [--runtime-repo <path>] [--cas-dir <path>] \\\n\
   160	         \x20  [--prev-cid-hex <hex>] [--alignment-dir <path>] \\\n\
   161	         \x20  [--no-cas]\\\n\
   162	         \x20  [--include-prior-capsules <N>]                     (default 0; > 0 triggers deep-history gate)\n\
   163	         \n\
   164	         env:\n\
   165	         \x20  TURINGOS_MARKOV_OVERRIDE=1   permit deep-history reads (default-deny;\n\
   166	         \x20                                required when --include-prior-capsules > 0)\n\
   167	         \n\
   168	         exit:\n\
   169	         \x20  0  capsule generated + persisted\n\
   170	         \x20  1  generation failed (write / missing constitution)\n\
   171	         \x20  2  invalid args\n\
   172	         \x20  3  deep-history read denied (override env not set)"
   173	    );
   174	}
   175	
   176	fn parse_cid_hex(s: &str) -> Result<Cid, String> {
   177	    if s.len() != 64 {
   178	        return Err(format!("--prev-cid-hex must be 64 hex chars; got {}", s.len()));
   179	    }
   180	    let mut out = [0u8; 32];
   181	    for (i, byte) in out.iter_mut().enumerate() {
   182	        let chunk = &s[i * 2..i * 2 + 2];
   183	        *byte = u8::from_str_radix(chunk, 16)
   184	            .map_err(|e| format!("--prev-cid-hex byte {i}: {e}"))?;
   185	    }
   186	    Ok(Cid(out))
   187	}
   188	
   189	fn run() -> Result<i32, String> {
   190	    let argv: Vec<String> = std::env::args().collect();
   191	    let args = match parse_args(&argv[1..]) {
   192	        Ok(a) => a,
   193	        Err(m) if m == "help" => {
   194	            print_help();
   195	            return Ok(0);
   196	        }
   197	        Err(m) => {
   198	            eprintln!("generate_markov_capsule: {m}");
   199	            print_help();
   200	            return Ok(2);
   201	        }
   202	    };
   203	
   204	    let override_set = override_set_from_env();
   205	    if override_set {
   206	        eprintln!(
   207	            "generate_markov_capsule: TURINGOS_MARKOV_OVERRIDE=1 set — \
   208	             deep-history reads ENABLED (audit-only path)."
   209	        );
   210	    } else {
   211	        eprintln!(
   212	            "generate_markov_capsule: TURINGOS_MARKOV_OVERRIDE not set — \
   213	             deep-history reads DEFAULT-DENIED (FR-15.5 + halt-trigger #6); \
   214	             set TURINGOS_MARKOV_OVERRIDE=1 to enable"
   215	        );
   216	    }
   217	
   218	    // R2 closure (Codex R1 Q4 — live override gate). LIVE-PATH gate:
   219	    // when caller asks for deeper history (more than just constitution +
   220	    // previous_capsule_cid), enforce TURINGOS_MARKOV_OVERRIDE=1 BEFORE

exec
/bin/bash -lc "nl -ba src/bin/generate_markov_capsule.rs | sed -n '220,430p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   220	    // previous_capsule_cid), enforce TURINGOS_MARKOV_OVERRIDE=1 BEFORE
   221	    // any deep-history I/O. Default `include_prior_capsules == 0`
   222	    // never triggers; default-deny is an active branch in the binary's
   223	    // flow, not just a library helper.
   224	    if args.include_prior_capsules > 0 {
   225	        match try_deep_history_read_with_override_check(override_set) {
   226	            Ok(()) => {
   227	                eprintln!(
   228	                    "generate_markov_capsule: deep-history read APPROVED \
   229	                     for {} prior capsules (override active)",
   230	                    args.include_prior_capsules
   231	                );
   232	                // NOTE: actual prior-capsule walk lands in TB-16 controlled-
   233	                // arena work (per `feedback_no_retroactive_evidence_rewrite`
   234	                // going-forward only — TB-15 v0 ships the gate; TB-16
   235	                // exercises the deep-history walk on a real chain).
   236	                eprintln!(
   237	                    "generate_markov_capsule: prior-capsule walk DEFERRED \
   238	                     to TB-16 controlled-arena (gate is enforced; walk is \
   239	                     not yet implemented; this is honest scope deferral)"
   240	                );
   241	            }
   242	            Err(MarkovGenError::DeepHistoryReadDenied) => {
   243	                eprintln!(
   244	                    "generate_markov_capsule: DEEP-HISTORY READ DENIED \
   245	                     ({} prior capsules requested; TURINGOS_MARKOV_OVERRIDE \
   246	                     not set). Refusing to proceed.",
   247	                    args.include_prior_capsules
   248	                );
   249	                return Ok(3);
   250	            }
   251	            Err(other) => {
   252	                return Err(format!("deep-history gate: {other}"));
   253	            }
   254	        }
   255	    }
   256	
   257	    // Step 1: constitution.md SHA-256.
   258	    let constitution_hash = sha256_of_file(&args.constitution_path)
   259	        .map_err(|e| format!("read constitution.md: {e}"))?;
   260	    eprintln!(
   261	        "constitution_hash = {}",
   262	        hex32(&constitution_hash.0)
   263	    );
   264	
   265	    // Step 1.5 (R2 closure — Codex R1 Q8/RQ7 + Gemini R1 Q7):
   266	    // canonical flowchart hashes from TRACE_FLOWCHART_MATRIX.md. Closes
   267	    // the literal SG-15.7 spec "constitution hash AND flowchart hashes"
   268	    // requirement.
   269	    let flowchart_hashes = read_flowchart_hashes_from_matrix(&args.flowchart_matrix_path)
   270	        .map_err(|e| format!("read flowchart hashes: {e}"))?;
   271	    eprintln!("flowchart_hashes.len = {}", flowchart_hashes.len());
   272	
   273	    // Step 2: L4 / L4.E / CAS roots — for v0, accept zero placeholders
   274	    // when --runtime-repo/--cas-dir are absent (fresh-repo path) and
   275	    // populate from CAS metadata digest when present. Future TB will
   276	    // wire to the actual chain head readers; v0 ships the substrate.
   277	    let l4_root = Hash::ZERO;
   278	    let l4e_root = Hash::ZERO;
   279	
   280	    // Step 3: scan OBS files.
   281	    let unresolved_obs: Vec<ObsId> = scan_unresolved_obs(&args.alignment_dir)
   282	        .map_err(|e| format!("scan OBS: {e}"))?;
   283	    eprintln!("unresolved_obs.len = {}", unresolved_obs.len());
   284	
   285	    // Step 4: typical_errors — v0 accepts empty (no chain-resident
   286	    // capsules in dry-run) and TB-16+ wires to actual cluster_autopsies
   287	    // over CAS-resident AgentAutopsyCapsule objects.
   288	    let typical_errors: Vec<TypicalErrorSummary> = Vec::new();
   289	
   290	    // Step 5: previous capsule Cid.
   291	    let previous_capsule_cid: Option<Cid> = match &args.prev_cid_hex {
   292	        Some(s) => Some(parse_cid_hex(s)?),
   293	        None => None,
   294	    };
   295	
   296	    // Step 6: write capsule. Two modes:
   297	    //   (a) --no-cas: build the capsule struct directly + skip CAS put.
   298	    //       Used when no runtime CAS is available (fresh repo).
   299	    //   (b) default: open `--cas-dir` as a CasStore + put.
   300	    let cas_root = Hash::ZERO; // v0 placeholder; future wire-in via CAS metadata digest.
   301	    let capsule = if args.no_cas {
   302	        eprintln!("generate_markov_capsule: --no-cas mode — JSON pointer only");
   303	        // Compute capsule_id deterministically without CAS write.
   304	        use turingosv4::bottom_white::ledger::transition_ledger::canonical_encode;
   305	        use turingosv4::runtime::markov_capsule::MarkovEvidenceCapsule;
   306	        let next_session_json = serde_json::json!({
   307	            "schema_version": "v1/next_session_context",
   308	            "constitution_hash_hex": hex32(&constitution_hash.0),
   309	            "flowchart_hashes_hex": flowchart_hashes.iter().map(|h| hex32(&h.0)).collect::<Vec<_>>(),
   310	            "previous_markov_cid_hex": previous_capsule_cid.map(|c| c.hex()),
   311	            "tb_tag": format!("TB-{}", args.tb_id),
   312	            "boot_seq": [
   313	                "1. read constitution.md (verify sha256 == constitution_hash)",
   314	                "2. read TRACE_FLOWCHART_MATRIX.md (verify each flowchart sha256 == flowchart_hashes[i])",
   315	                "3. read CAS<this_markov_capsule_cid>",
   316	                "4. read CAS<previous_markov_capsule_cid> (if present)",
   317	                "5. DEFAULT-DENY deeper history; set TURINGOS_MARKOV_OVERRIDE=1 to override (audit-only)"
   318	            ],
   319	        });
   320	        let next_session_bytes = serde_json::to_vec(&next_session_json)
   321	            .map_err(|e| format!("next_session_context encode: {e}"))?;
   322	        let next_session_context_cid = Cid::from_content(&next_session_bytes);
   323	        let mut cap = MarkovEvidenceCapsule {
   324	            capsule_id: Cid::default(),
   325	            previous_capsule_cid,
   326	            constitution_hash,
   327	            flowchart_hashes: flowchart_hashes.clone(),
   328	            l4_root,
   329	            l4e_root,
   330	            cas_root,
   331	            typical_errors,
   332	            unresolved_obs,
   333	            next_session_context_cid,
   334	            sha256: Hash::ZERO,
   335	            created_at_logical_t: 0,
   336	            tb_tag: format!("TB-{}", args.tb_id),
   337	        };
   338	        let prelim_bytes = canonical_encode(&cap)
   339	            .map_err(|e| format!("capsule prelim encode: {e:?}"))?;
   340	        let cid = Cid::from_content(&prelim_bytes);
   341	        cap.capsule_id = cid;
   342	        cap.sha256 = Hash(cid.0);
   343	        cap
   344	    } else {
   345	        let cas_dir = args
   346	            .cas_dir
   347	            .as_ref()
   348	            .ok_or_else(|| "--cas-dir required without --no-cas".to_string())?;
   349	        let cas = Arc::new(RwLock::new(
   350	            CasStore::open(cas_dir).map_err(|e| format!("open CAS: {e}"))?,
   351	        ));
   352	        write_markov_capsule(
   353	            &cas,
   354	            previous_capsule_cid,
   355	            constitution_hash,
   356	            flowchart_hashes,
   357	            l4_root,
   358	            l4e_root,
   359	            cas_root,
   360	            typical_errors,
   361	            unresolved_obs,
   362	            format!("TB-{}", args.tb_id),
   363	            "tb15-generator",
   364	            0,
   365	        )
   366	        .map_err(|e| match e {
   367	            MarkovGenError::DeepHistoryReadDenied => {
   368	                "deep-history read denied (set TURINGOS_MARKOV_OVERRIDE=1)".to_string()
   369	            }
   370	            other => format!("write_markov_capsule: {other}"),
   371	        })?
   372	    };
   373	
   374	    eprintln!("capsule_id = {}", capsule.capsule_id.hex());
   375	
   376	    // Step 7: emit JSON pointer file + LATEST_MARKOV_CAPSULE.txt.
   377	    std::fs::create_dir_all(&args.out_dir)
   378	        .map_err(|e| format!("create out_dir: {e}"))?;
   379	    let json_path = args
   380	        .out_dir
   381	        .join(format!("MARKOV_TB-{}_2026-05-03.json", args.tb_id));
   382	    let json_body = serde_json::to_string_pretty(&capsule)
   383	        .map_err(|e| format!("capsule json encode: {e}"))?;
   384	    std::fs::write(&json_path, &json_body).map_err(|e| format!("write json: {e}"))?;
   385	    let latest_path = args.out_dir.join("LATEST_MARKOV_CAPSULE.txt");
   386	    std::fs::write(&latest_path, capsule.capsule_id.hex())
   387	        .map_err(|e| format!("write latest pointer: {e}"))?;
   388	
   389	    eprintln!("wrote {}", json_path.display());
   390	    eprintln!("wrote {}", latest_path.display());
   391	    Ok(0)
   392	}
   393	
   394	fn hex32(b: &[u8; 32]) -> String {
   395	    let mut s = String::with_capacity(64);
   396	    for x in b {
   397	        s.push_str(&format!("{:02x}", x));
   398	    }
   399	    s
   400	}
   401	
   402	fn main() {
   403	    match run() {
   404	        Ok(code) => std::process::exit(code),
   405	        Err(msg) => {
   406	            eprintln!("generate_markov_capsule: {msg}");
   407	            std::process::exit(1);
   408	        }
   409	    }
   410	}

exec
/bin/bash -lc "nl -ba tests/tb_15_halt_triggers.rs | sed -n '1,230p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	/// TB-15 Halt-Trigger Fixture (architect §6.6 forbidden + §6.5 SG halts)
     2	///
     3	/// 6 tests that must ALL be green before TB-15 ships.
     4	/// Atom 1 = `unimplemented!()` stubs only; later atoms backfill:
     5	///   Atom 2: #3 (autopsy_does_not_mutate_predicates)
     6	///   Atom 3: #1 (raw_logs_not_in_general_read_view) + #4 (private_detail_not_in_other_agent_view)
     7	///   Atom 4: #5 (typical_error_clustering_uses_summary_only)
     8	///   Atom 5: #2 (markov_capsule_references_constitution_hash) + #6 (deep_history_read_without_override_fails)
     9	///
    10	/// Any atom that flips a green test to red = immediate halt (no round-2).
    11	/// TRACE_MATRIX FC1-N32 + FC1-N33 + FC2-N30 + FC3-N43
    12	
    13	// ────────────────────────────────────────────────────────────────────
    14	// Halt-trigger #1
    15	// raw_logs_not_in_general_read_view
    16	//
    17	// AgentVisibleProjection.views must NOT contain raw autopsy bytes
    18	// (private_detail_cid contents). Agent_autopsies_t lives on
    19	// EconomicState — sequencer-side index only — and is NOT projected
    20	// into AgentVisibleProjection. CR-15.1.
    21	//
    22	// Filled in by Atom 3 (after EconomicState gains agent_autopsies_t).
    23	// ────────────────────────────────────────────────────────────────────
    24	#[test]
    25	fn raw_logs_not_in_general_read_view() {
    26	    let manifest = env!("CARGO_MANIFEST_DIR");
    27	    let q_state_path = format!("{}/src/state/q_state.rs", manifest);
    28	    let body = std::fs::read_to_string(&q_state_path)
    29	        .unwrap_or_else(|e| panic!("read {}: {}", q_state_path, e));
    30	
    31	    // Locate `pub struct AgentVisibleProjection {` and its terminating `}`.
    32	    let needle = "pub struct AgentVisibleProjection";
    33	    let start = body
    34	        .find(needle)
    35	        .expect("AgentVisibleProjection struct must exist in q_state.rs");
    36	    let after = &body[start..];
    37	    let brace_open = after
    38	        .find('{')
    39	        .expect("AgentVisibleProjection struct: opening brace not found");
    40	    let mut depth = 0i32;
    41	    let mut end = brace_open;
    42	    for (i, ch) in after[brace_open..].char_indices() {
    43	        match ch {
    44	            '{' => depth += 1,
    45	            '}' => {
    46	                depth -= 1;
    47	                if depth == 0 {
    48	                    end = brace_open + i;
    49	                    break;
    50	                }
    51	            }
    52	            _ => {}
    53	        }
    54	    }
    55	    let projection_body = &after[brace_open..=end];
    56	
    57	    // Constructed at runtime via byte literals so this test's own source
    58	    // doesn't contain the forbidden substrings.
    59	    let forbidden: Vec<String> = vec![
    60	        format!("agent_autopsies{}", "_t"),
    61	        format!("Autopsy{}", "Index"),
    62	        format!("Agent{}", "AutopsyCapsule"),
    63	        format!("private_detail_{}", "cid"),
    64	    ];
    65	    for tok in &forbidden {
    66	        assert!(
    67	            !projection_body.contains(tok.as_str()),
    68	            "halt-trigger #1: AgentVisibleProjection MUST NOT reference TB-15 \
    69	             autopsy type `{}` — autopsy is sequencer-side / CAS-only and is NOT \
    70	             projected to agent read view (CR-15.1)",
    71	            tok
    72	        );
    73	    }
    74	}
    75	
    76	// ────────────────────────────────────────────────────────────────────
    77	// Halt-trigger #2
    78	// markov_capsule_references_constitution_hash
    79	//
    80	// MarkovEvidenceCapsule.constitution_hash must equal sha256 of the
    81	// constitution.md bytes at generation time. SG-15.7.
    82	//
    83	// Filled in by Atom 5 (markov_capsule generator).
    84	// ────────────────────────────────────────────────────────────────────
    85	#[test]
    86	fn markov_capsule_references_constitution_hash() {
    87	    use sha2::{Digest, Sha256};
    88	    use turingosv4::runtime::markov_capsule::{
    89	        read_flowchart_hashes_from_matrix, MarkovEvidenceCapsule,
    90	    };
    91	
    92	    let manifest = env!("CARGO_MANIFEST_DIR");
    93	    let constitution_path = format!("{}/constitution.md", manifest);
    94	    let constitution_bytes = std::fs::read(&constitution_path)
    95	        .unwrap_or_else(|e| panic!("read constitution.md: {}", e));
    96	    let mut h = Sha256::new();
    97	    h.update(&constitution_bytes);
    98	    let expected_hash: [u8; 32] = h.finalize().into();
    99	
   100	    let capsule = MarkovEvidenceCapsule::with_constitution_hash(expected_hash);
   101	    assert_eq!(
   102	        capsule.constitution_hash.0, expected_hash,
   103	        "halt-trigger #2: MarkovEvidenceCapsule.constitution_hash must equal \
   104	         sha256 of constitution.md bytes (SG-15.7)"
   105	    );
   106	
   107	    // R2 closure (Codex R1 Q8/RQ7 + Gemini R1 Q7): SG-15.7 spec literal
   108	    // is "constitution hash AND flowchart hashes". Capsule MUST also
   109	    // reference 4 canonical flowchart hashes (per architect 2026-05-02
   110	    // ruling 9 of Part C — flowcharts elevated to SHA-anchored
   111	    // architectural contracts).
   112	    let matrix_path = std::path::PathBuf::from(manifest)
   113	        .join("handover/alignment/TRACE_FLOWCHART_MATRIX.md");
   114	    let flowchart_hashes =
   115	        read_flowchart_hashes_from_matrix(&matrix_path).expect("matrix parse");
   116	    assert_eq!(
   117	        flowchart_hashes.len(),
   118	        4,
   119	        "halt-trigger #2: TRACE_FLOWCHART_MATRIX.md must yield exactly 4 \
   120	         canonical flowchart hashes (1a, 1b, 2, 3) per architect §2 (SG-15.7)"
   121	    );
   122	    // Capsule's flowchart_hashes field exists + accepts 4 hashes.
   123	    let mut cap_with_fc = capsule.clone();
   124	    cap_with_fc.flowchart_hashes = flowchart_hashes.clone();
   125	    assert_eq!(
   126	        cap_with_fc.flowchart_hashes.len(),
   127	        4,
   128	        "halt-trigger #2: MarkovEvidenceCapsule.flowchart_hashes must hold \
   129	         exactly the 4 canonical flowchart hashes (SG-15.7 literal compliance)"
   130	    );
   131	}
   132	
   133	// ────────────────────────────────────────────────────────────────────
   134	// Halt-trigger #3
   135	// autopsy_does_not_mutate_predicates
   136	//
   137	// write_autopsy_capsule signature MUST NOT accept any &mut PredicateRegistry
   138	// or any other mutator on the predicate / tool / risk-policy registries.
   139	// Source-level fence: scan src/runtime/autopsy_capsule.rs for forbidden
   140	// signature tokens. CR-15.3 + SG-15.8.
   141	//
   142	// Filled in by Atom 2.
   143	// ────────────────────────────────────────────────────────────────────
   144	#[test]
   145	fn autopsy_does_not_mutate_predicates() {
   146	    let manifest = env!("CARGO_MANIFEST_DIR");
   147	    let path = format!("{}/src/runtime/autopsy_capsule.rs", manifest);
   148	    let body = std::fs::read_to_string(&path)
   149	        .unwrap_or_else(|e| panic!("read {}: {}", path, e));
   150	
   151	    // The autopsy module MUST NOT contain any mutator surface against
   152	    // the predicate / tool / risk-policy registries. Constructed at
   153	    // runtime to avoid this test's own source containing the forbidden
   154	    // substrings (and triggering self-trip on the file scan).
   155	    let forbidden: Vec<String> = vec![
   156	        format!("&mut Predicate{}", "Registry"),
   157	        format!("&mut Tool{}", "Registry"),
   158	        format!("&mut Risk{}", "PolicyRegistry"),
   159	        format!("&mut PredicateRunner"),
   160	        format!(".register_predicate("),
   161	        format!(".unregister_predicate("),
   162	        format!(".patch_predicate("),
   163	        format!(".register_tool("),
   164	        format!(".unregister_tool("),
   165	    ];
   166	    for tok in &forbidden {
   167	        assert!(
   168	            !body.contains(tok.as_str()),
   169	            "halt-trigger #3: autopsy_capsule.rs MUST NOT contain `{}` — \
   170	             autopsy carries `suggested_policy_patch: Option<Cid>` only as a \
   171	             SUGGESTION pointer; never auto-applied (CR-15.3 + SG-15.8)",
   172	            tok
   173	        );
   174	    }
   175	}
   176	
   177	// ────────────────────────────────────────────────────────────────────
   178	// Halt-trigger #4
   179	// private_detail_not_in_other_agent_view
   180	//
   181	// Agent B's projection must not contain Agent A's autopsy bytes.
   182	// AutopsyIndex stores Cids only; the CAS bytes behind private_detail_cid
   183	// require AuditOnly access. SG-15.2.
   184	//
   185	// Filled in by Atom 3 (after EconomicState gains agent_autopsies_t).
   186	// ────────────────────────────────────────────────────────────────────
   187	#[test]
   188	fn private_detail_not_in_other_agent_view() {
   189	    // Structural fence: AutopsyIndex value type must remain Vec<Cid>
   190	    // (32-byte content addresses), NOT Vec<AgentAutopsyCapsule> (the
   191	    // bytes themselves) and NOT any structure containing
   192	    // private_detail_cid payload bytes. Even if AgentVisibleProjection
   193	    // were ever to surface AutopsyIndex contents (which it does not —
   194	    // see halt-trigger #1), it would surface only public CAS Cids of
   195	    // public CAS evidence.
   196	    let manifest = env!("CARGO_MANIFEST_DIR");
   197	    let q_state_path = format!("{}/src/state/q_state.rs", manifest);
   198	    let body = std::fs::read_to_string(&q_state_path)
   199	        .unwrap_or_else(|e| panic!("read {}: {}", q_state_path, e));
   200	
   201	    // Locate the AutopsyIndex newtype definition.
   202	    let needle = "pub struct Autopsy".to_string() + "Index";
   203	    let start = body
   204	        .find(&needle)
   205	        .expect("AutopsyIndex newtype must exist in q_state.rs");
   206	    let after = &body[start..];
   207	    // Walk forward until the line ending with `;` (newtype is single-line).
   208	    let line_end = after
   209	        .find(";\n")
   210	        .or_else(|| after.find(";\r"))
   211	        .or_else(|| after.find(';'))
   212	        .expect("AutopsyIndex newtype must terminate with semicolon");
   213	    let decl = &after[..=line_end];
   214	
   215	    // The value type MUST be Vec<Cid>. Forbidden alternatives that
   216	    // would leak raw bytes:
   217	    let forbidden_value_shapes: Vec<String> = vec![
   218	        format!("Vec<Agent{}>", "AutopsyCapsule"),
   219	        format!("Vec<u{}>", "8"),
   220	        format!("Vec<Auto{}>", "psyPrivateDetail"),
   221	    ];
   222	    for tok in &forbidden_value_shapes {
   223	        assert!(
   224	            !decl.contains(tok.as_str()),
   225	            "halt-trigger #4: AutopsyIndex value type MUST be Vec<Cid>, \
   226	             NOT `{}` — agent_autopsies_t stores Cids only; raw bytes \
   227	             stay in CAS behind AuditOnly access (SG-15.2)",
   228	            tok
   229	        );
   230	    }

exec
/bin/bash -lc 'rg -n "include-prior-capsules|try_deep_history_read_with_override_check|override_set_from_env|read_flowchart_hashes_from_matrix|write_markov_capsule|no-cas|LATEST_MARKOV" src/bin/generate_markov_capsule.rs src/runtime/markov_capsule.rs tests/tb_15_halt_triggers.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
tests/tb_15_halt_triggers.rs:89:        read_flowchart_hashes_from_matrix, MarkovEvidenceCapsule,
tests/tb_15_halt_triggers.rs:115:        read_flowchart_hashes_from_matrix(&matrix_path).expect("matrix parse");
tests/tb_15_halt_triggers.rs:368:        try_deep_history_read_with_override_check, MarkovGenError,
tests/tb_15_halt_triggers.rs:372:    let result = try_deep_history_read_with_override_check(false);
tests/tb_15_halt_triggers.rs:383:    let ok = try_deep_history_read_with_override_check(true);
src/runtime/markov_capsule.rs:10://! Generator surface: `write_markov_capsule(...)` (CAS-emitting) + binary
src/runtime/markov_capsule.rs:192:pub fn try_deep_history_read_with_override_check(
src/runtime/markov_capsule.rs:205:pub fn override_set_from_env() -> bool {
src/runtime/markov_capsule.rs:227:pub fn write_markov_capsule(
src/runtime/markov_capsule.rs:333:/// identical to what `write_markov_capsule` returned at write time.
src/runtime/markov_capsule.rs:335:/// Invariant: for any capsule written by `write_markov_capsule`,
src/runtime/markov_capsule.rs:404:pub fn read_flowchart_hashes_from_matrix(
src/runtime/markov_capsule.rs:478:    /// TB-15 Atom 5 — try_deep_history_read_with_override_check:
src/runtime/markov_capsule.rs:482:        match try_deep_history_read_with_override_check(false) {
src/runtime/markov_capsule.rs:486:        assert!(try_deep_history_read_with_override_check(true).is_ok());
src/runtime/markov_capsule.rs:489:    /// TB-15 Atom 5 — write_markov_capsule writes 2 CAS objects
src/runtime/markov_capsule.rs:493:    fn write_markov_capsule_to_cas_round_trip() {
src/runtime/markov_capsule.rs:506:        let cap = write_markov_capsule(
src/runtime/markov_capsule.rs:536:    /// TB-15 Atom 5 — write_markov_capsule deterministic (same inputs →
src/runtime/markov_capsule.rs:539:    fn write_markov_capsule_deterministic_capsule_id() {
src/runtime/markov_capsule.rs:546:            write_markov_capsule(
src/runtime/markov_capsule.rs:578:        let cap_1 = write_markov_capsule(
src/runtime/markov_capsule.rs:593:        let cap_2 = write_markov_capsule(
src/runtime/markov_capsule.rs:649:    /// TB-15 R2 closure (Codex Q8/RQ7 + Gemini Q7): read_flowchart_hashes_from_matrix
src/runtime/markov_capsule.rs:656:        let hashes = read_flowchart_hashes_from_matrix(&path).expect("matrix parse");
src/runtime/markov_capsule.rs:675:    fn write_markov_capsule_cas_resolvable_by_capsule_id() {
src/runtime/markov_capsule.rs:681:        let cap = write_markov_capsule(
src/bin/generate_markov_capsule.rs:25://!     [--no-cas]
src/bin/generate_markov_capsule.rs:27://! `--no-cas` runs in pointer-only mode (write JSON file but skip CAS
src/bin/generate_markov_capsule.rs:42:    override_set_from_env, read_flowchart_hashes_from_matrix, scan_unresolved_obs, sha256_of_file,
src/bin/generate_markov_capsule.rs:43:    try_deep_history_read_with_override_check, write_markov_capsule, MarkovGenError, ObsId,
src/bin/generate_markov_capsule.rs:113:            "--no-cas" => {
src/bin/generate_markov_capsule.rs:117:            "--include-prior-capsules" => {
src/bin/generate_markov_capsule.rs:120:                    .ok_or_else(|| "--include-prior-capsules <N> requires arg".to_string())?
src/bin/generate_markov_capsule.rs:122:                    .map_err(|e| format!("--include-prior-capsules N parse: {e}"))?;
src/bin/generate_markov_capsule.rs:161:         \x20  [--no-cas]\\\n\
src/bin/generate_markov_capsule.rs:162:         \x20  [--include-prior-capsules <N>]                     (default 0; > 0 triggers deep-history gate)\n\
src/bin/generate_markov_capsule.rs:166:         \x20                                required when --include-prior-capsules > 0)\n\
src/bin/generate_markov_capsule.rs:204:    let override_set = override_set_from_env();
src/bin/generate_markov_capsule.rs:225:        match try_deep_history_read_with_override_check(override_set) {
src/bin/generate_markov_capsule.rs:269:    let flowchart_hashes = read_flowchart_hashes_from_matrix(&args.flowchart_matrix_path)
src/bin/generate_markov_capsule.rs:297:    //   (a) --no-cas: build the capsule struct directly + skip CAS put.
src/bin/generate_markov_capsule.rs:302:        eprintln!("generate_markov_capsule: --no-cas mode — JSON pointer only");
src/bin/generate_markov_capsule.rs:348:            .ok_or_else(|| "--cas-dir required without --no-cas".to_string())?;
src/bin/generate_markov_capsule.rs:352:        write_markov_capsule(
src/bin/generate_markov_capsule.rs:370:            other => format!("write_markov_capsule: {other}"),
src/bin/generate_markov_capsule.rs:376:    // Step 7: emit JSON pointer file + LATEST_MARKOV_CAPSULE.txt.
src/bin/generate_markov_capsule.rs:385:    let latest_path = args.out_dir.join("LATEST_MARKOV_CAPSULE.txt");

exec
/bin/bash -lc "nl -ba tests/tb_15_halt_triggers.rs | sed -n '230,420p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   230	    }
   231	    // Positive assertion: the declaration includes Vec<...Cid>.
   232	    assert!(
   233	        decl.contains("Vec<crate::bottom_white::cas::schema::Cid>")
   234	            || decl.contains("Vec<Cid>"),
   235	        "halt-trigger #4: AutopsyIndex value type must explicitly be Vec<Cid>; \
   236	         got declaration: {}",
   237	        decl
   238	    );
   239	}
   240	
   241	// ────────────────────────────────────────────────────────────────────
   242	// Halt-trigger #5
   243	// typical_error_clustering_uses_summary_only
   244	//
   245	// cluster_autopsies output (Vec<TypicalErrorSummary>) must embed
   246	// public_summary text + capsule_id Cids only. It must NEVER embed
   247	// private_detail_cid bytes. SG-15.5.
   248	//
   249	// Filled in by Atom 4 (cluster_autopsies + TypicalErrorSummary).
   250	// ────────────────────────────────────────────────────────────────────
   251	#[test]
   252	fn typical_error_clustering_uses_summary_only() {
   253	    use turingosv4::bottom_white::cas::schema::Cid;
   254	    use turingosv4::economy::money::MicroCoin;
   255	    use turingosv4::runtime::autopsy_capsule::{
   256	        cluster_autopsies, AgentAutopsyCapsule, LossReasonClass,
   257	    };
   258	    use turingosv4::state::q_state::{AgentId, Hash, TaskId};
   259	    use turingosv4::state::typed_tx::{CapsulePrivacyPolicy, EventId};
   260	
   261	    // Build 3 autopsies of the same loss_reason_class with
   262	    // distinguishable private_detail_cid bytes.
   263	    let event = EventId(TaskId("task:tb15:halt5".into()));
   264	    let mk = |agent: &str, priv_byte: u8| AgentAutopsyCapsule {
   265	        capsule_id: Cid::from_content(agent.as_bytes()),
   266	        agent_id: AgentId(agent.to_string()),
   267	        event_id: event.clone(),
   268	        loss_amount: MicroCoin::from_micro_units(1_000),
   269	        loss_reason_class: LossReasonClass::Bankruptcy,
   270	        violated_risk_rule: None,
   271	        suggested_policy_patch: None,
   272	        evidence_cids: vec![],
   273	        public_summary: format!(
   274	            "agent={} lost 1000μC on event={} reason=Bankruptcy",
   275	            agent, (event.0).0
   276	        ),
   277	        private_detail_cid: Cid([priv_byte; 32]),
   278	        privacy_policy: CapsulePrivacyPolicy::AuditOnly,
   279	        sha256: Hash::ZERO,
   280	        created_at_logical_t: 1,
   281	        created_at_round: 0,
   282	    };
   283	    let priv_bytes: [u8; 3] = [0xAA, 0xBB, 0xCC];
   284	    let autopsies = vec![
   285	        mk("A", priv_bytes[0]),
   286	        mk("B", priv_bytes[1]),
   287	        mk("C", priv_bytes[2]),
   288	    ];
   289	
   290	    let summaries = cluster_autopsies(&autopsies, 3);
   291	    assert_eq!(summaries.len(), 1, "3 same-class autopsies → 1 typical error");
   292	    assert_eq!(summaries[0].count, 3);
   293	
   294	    // R2 closure (Codex R1 Q5): the original byte-window scan looked for
   295	    // a raw 32-byte run of `[priv_byte; 32]`, but Cid serializes through
   296	    // serde_json as a 32-element JSON ARRAY (`[170,170,...,170]`) — NOT
   297	    // a contiguous binary 32-byte run. The strengthened check inspects
   298	    // BOTH (a) the JSON-array text representation that serde_json
   299	    // produces for a `Cid([priv_byte; 32])`, AND (b) the raw 32-byte run
   300	    // (defense-in-depth against future format changes).
   301	    let json_text = serde_json::to_string(&summaries).expect("serialize summaries");
   302	    let json_bytes = json_text.as_bytes();
   303	    let canonical_bytes =
   304	        turingosv4::bottom_white::ledger::transition_ledger::canonical_encode(&summaries)
   305	            .expect("canonical encode");
   306	    for &priv_byte in &priv_bytes {
   307	        // (a) JSON-array text form: a Cid([0xAA;32]) renders as
   308	        //     `[170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,
   309	        //       170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,
   310	        //       170,170]` (each byte as its decimal value).
   311	        let n = priv_byte as u32;
   312	        let mut json_array_form = String::with_capacity(160);
   313	        json_array_form.push('[');
   314	        for i in 0..32 {
   315	            if i > 0 {
   316	                json_array_form.push(',');
   317	            }
   318	            json_array_form.push_str(&n.to_string());
   319	        }
   320	        json_array_form.push(']');
   321	        assert!(
   322	            !json_text.contains(&json_array_form),
   323	            "halt-trigger #5 (R2 strengthened): TypicalErrorSummary JSON \
   324	             serialization contains the canonical Cid array form for \
   325	             private_detail_cid byte 0x{:02x} — broadcast surface MUST use \
   326	             public_summary text only (SG-15.5)",
   327	            priv_byte
   328	        );
   329	
   330	        // (b) raw 32-byte run defense-in-depth (would catch
   331	        //     a hypothetical bincode/canonical-encoded leak).
   332	        let private_cid_run = [priv_byte; 32];
   333	        for window in canonical_bytes.windows(32) {
   334	            assert!(
   335	                window != private_cid_run,
   336	                "halt-trigger #5 (R2): canonical_encode of TypicalErrorSummary \
   337	                 contains a 32-byte run of private_detail_cid byte 0x{:02x}",
   338	                priv_byte
   339	            );
   340	        }
   341	        // Also still check JSON bytes for raw run (belt + suspenders).
   342	        for window in json_bytes.windows(32) {
   343	            assert!(
   344	                window != private_cid_run,
   345	                "halt-trigger #5 (R2): JSON of TypicalErrorSummary contains a \
   346	                 raw 32-byte run of private_detail_cid byte 0x{:02x}",
   347	                priv_byte
   348	            );
   349	        }
   350	    }
   351	}
   352	
   353	// ────────────────────────────────────────────────────────────────────
   354	// Halt-trigger #6
   355	// deep_history_read_without_override_fails
   356	//
   357	// generate_markov_capsule binary defaults to constitution +
   358	// latest-Markov-capsule context source. Reading deeper history (older
   359	// capsules; L4 chain rows pre-dating prior Markov capsule's l4_root)
   360	// requires TURINGOS_MARKOV_OVERRIDE=1; default-deny path returns
   361	// `MarkovGenError::DeepHistoryReadDenied`. SG-15.4 + FR-15.5.
   362	//
   363	// Filled in by Atom 5.
   364	// ────────────────────────────────────────────────────────────────────
   365	#[test]
   366	fn deep_history_read_without_override_fails() {
   367	    use turingosv4::runtime::markov_capsule::{
   368	        try_deep_history_read_with_override_check, MarkovGenError,
   369	    };
   370	
   371	    // Default-deny path: no override; result must be DeepHistoryReadDenied.
   372	    let result = try_deep_history_read_with_override_check(false);
   373	    match result {
   374	        Err(MarkovGenError::DeepHistoryReadDenied) => {}
   375	        other => panic!(
   376	            "halt-trigger #6: expected DeepHistoryReadDenied without \
   377	             TURINGOS_MARKOV_OVERRIDE=1; got {:?} (SG-15.4 + FR-15.5)",
   378	            other
   379	        ),
   380	    }
   381	
   382	    // Override path: result is Ok(()).
   383	    let ok = try_deep_history_read_with_override_check(true);
   384	    assert!(
   385	        ok.is_ok(),
   386	        "halt-trigger #6: TURINGOS_MARKOV_OVERRIDE=1 must permit deep-history \
   387	         read; got {:?}",
   388	        ok
   389	    );
   390	}

exec
/bin/bash -lc "nl -ba src/runtime/markov_capsule.rs | sed -n '380,430p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   380	    let digest: [u8; 32] = h.finalize().into();
   381	    Ok(Hash(digest))
   382	}
   383	
   384	/// TRACE_MATRIX TB-15 R2 closure (Codex R1 Q8/RQ7 + Gemini R1 Q7): parse
   385	/// `handover/alignment/TRACE_FLOWCHART_MATRIX.md` §2 and return the four
   386	/// canonical flowchart hashes in canonical order (1a, 1b, 2, 3).
   387	///
   388	/// Format the parser expects (verbatim from the matrix file §2):
   389	/// ```text
   390	/// Flowchart 1a — ...
   391	///   SHA256: <64-hex>
   392	/// Flowchart 1b — ...
   393	///   SHA256: <64-hex>
   394	/// Flowchart 2 — ...
   395	///   SHA256: <64-hex>
   396	/// Flowchart 3 — ...
   397	///   SHA256: <64-hex>
   398	/// ```
   399	///
   400	/// Returns exactly 4 hashes in canonical order. Errors if any
   401	/// flowchart's hash is not parseable or the count is not 4.
   402	/// CR-15.5 (capsule references existing canonical artifact) + SG-15.7
   403	/// literal "constitution hash AND flowchart hashes".
   404	pub fn read_flowchart_hashes_from_matrix(
   405	    matrix_path: &std::path::Path,
   406	) -> Result<Vec<Hash>, MarkovGenError> {
   407	    let body = std::fs::read_to_string(matrix_path)?;
   408	    let mut hashes = Vec::with_capacity(4);
   409	    for label in &["Flowchart 1a", "Flowchart 1b", "Flowchart 2 ", "Flowchart 3 "] {
   410	        // Find label, then the next "SHA256:" line.
   411	        let label_pos = body.find(label).ok_or_else(|| {
   412	            MarkovGenError::Encode(format!("flowchart label `{}` not found in matrix", label))
   413	        })?;
   414	        let after = &body[label_pos..];
   415	        let sha_pos = after.find("SHA256:").ok_or_else(|| {
   416	            MarkovGenError::Encode(format!("SHA256: marker not found after `{}`", label))
   417	        })?;
   418	        let hex_start = label_pos + sha_pos + "SHA256:".len();
   419	        // Skip whitespace; take next 64 hex chars.
   420	        let rest = &body[hex_start..];
   421	        let hex_str: String = rest.chars().skip_while(|c| c.is_whitespace()).take(64).collect();
   422	        if hex_str.len() != 64 {
   423	            return Err(MarkovGenError::Encode(format!(
   424	                "flowchart `{}` hash is {} chars, expected 64",
   425	                label,
   426	                hex_str.len()
   427	            )));
   428	        }
   429	        let mut bytes = [0u8; 32];
   430	        for i in 0..32 {

exec
/bin/bash -lc "nl -ba handover/alignment/TRACE_FLOWCHART_MATRIX.md | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TRACE_FLOWCHART_MATRIX — TB ↔ Constitution Flowchart Mapping
     2	
     3	**Authority**: architect directive 2026-05-02, ruling 9 of Part C ("Add TRACE_FLOWCHART_MATRIX.md mapping future TBs to Flowchart 1/2/3").
     4	- Source: `handover/directives/2026-05-02_lossless_constitution_polymarket_directive.md`
     5	- Insight summary: `handover/architect-insights/2026-05-02_flowchart_hashes_and_trace_matrix.md`
     6	**Status**: ratified by user authorization 2026-05-02 (option D1, "create skeleton now with TB-1..TB-7R back-fill + TB-8 forward row").
     7	**Companion**: `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md` (per-symbol map; this matrix is per-TB).
     8	
     9	---
    10	
    11	## §1 Why this matrix exists
    12	
    13	The lossless constitution integrated edition 2026-05-02 elevated three flowcharts from explanatory diagrams to **SHA256-anchored architectural contracts**. Each TB charter (current + future) must declare which flowchart element(s) it touches; deviation between code behavior and the four canonical hashes is now provably auditable.
    14	
    15	This matrix is the cross-reference. It is updated at every TB ship.
    16	
    17	---
    18	
    19	## §2 The four canonical flowchart hashes
    20	
    21	```text
    22	Flowchart 1a — Runtime loop, page 8
    23	  rtool / input / Agent δ / output / predicates ∏p / write tool path
    24	  SHA256: a474c6b9ded766504a4f644a4a1b3c545316d418f0250f36ec692fcdf98f09f5
    25	
    26	Flowchart 1b — Runtime loop continuation, page 9
    27	  predicates branch / write tool / Q_{t+1} / map-reduce tick
    28	  SHA256: b822717b10332a2d8e789ba6af96fd4da4ff43a74afab679d1b82add9c32b64d
    29	
    30	Flowchart 2 — Boot + full architecture, page 13
    31	  Initialization (human → InitAI → predicates / Q0 / mr) + runtime loop + Finalization
    32	  SHA256: 6a4bc9195bafd55bde968fd445cdd2926d6906a7f6a2b38071d4774a7f0de333
    33	
    34	Flowchart 3 — Meta-architecture, page 17
    35	  Constitution + logs archive (read-only) → JudgeAI / ArchitectAI →
    36	  anti-oreo runtime (top / agents / tools) → log → archive → feedback → re-init
    37	  SHA256: c159413984d0c6c5daa06605fea3a86a2ad4ab9c4284d0d20e0e525bf03aa9cd
    38	```
    39	
    40	---
    41	
    42	## §3 TB ↔ Flowchart matrix
    43	
    44	Legend: ✅ touched (TB closes or extends this flowchart element) · ◯ adjacent (TB references but does not modify) · — not relevant.
    45	
    46	| TB ID | Flowchart 1<br>(runtime loop) | Flowchart 2<br>(boot) | Flowchart 3<br>(meta) | Notes |
    47	|---|---|---|---|---|
    48	| TB-1 | ◯ | — | — | TypedTx skeleton (pre-flowchart-anchoring) |
    49	| TB-2 | ◯ | — | — | predicate registry skeleton |
    50	| TB-3 | ✅ economic mutator (escrow / RSP) | — | — | EscrowLockTx + WorkTx.stake inline |
    51	| TB-4 | ✅ challenge path | — | — | ChallengeTx + VerifyTx |
    52	| TB-5 | ✅ predicate gate | — | — | RSP-3 challenge resolution + system-tx ingress |
    53	| TB-6 | ✅ runtime loop closure (production binary) | ✅ Boot wire-up (Q_0 from prod binary) | — | Production ChainTape wire-up; binary now drives kernel |
    54	| TB-7 | ✅ Frame B (proposal-level DAG) | — | — | Per-tactic deferred to TB-8+ |
    55	| TB-7R | ✅ runtime loop FULL closure: every externalized proposal → L4 / L4.E; predicate evidence resolves from CAS; dashboard regeneratable | ✅ Boot continuity: genesis_report.json + on-chain TaskOpen / EscrowLock | — | Constitution-Aligned Frame B Repair; Class 3 dual ship audit PASS; 712 tests / 0 fail |
    56	| TB-8 | ✅ settlement node CLOSED (every accepted L4 WorkTx with closed challenge window + no upheld challenge → exactly 1 L4 FinalizeRewardTx that atomically debits escrow + credits solver + flips claims_t.status = Finalized) | ✅ Boot continuity preserved (no new artifact; TB-7R genesis_report.json carries forward) | — | Minimal payout / FinalizeRewardTx SHIPPED 2026-05-02; Class 3 dual audit (Codex + Gemini both PASS strategic-tier; Codex round-1 VETO RQ3+RQ4 → round-2 PASS); 725 tests / 0 fail / 150 ignored (+13 net TB-8) |
    57	| TB-9 | ✅ identity in input/output of Agent δ persists across runs (same Agent_0 → same pubkey across evaluator restarts; smoke run-A == run-B == regression all bind to `dec9e321...047b6468`) | ✅ persistent registry initialized at boot via `AgentKeypairRegistry::generate_or_load_durable` reading `~/.turingos/keystore/agent_keystore.enc` (encrypted-at-rest, KDF + ChaCha20-Poly1305) | — | Durable AgentRegistry + read-only WalletTool projection SHIPPED 2026-05-02; Class 3 (purely additive kernel-side; recursive self-audit PASS); 723 tests / 0 fail / 150 ignored; 3/3 smoke runs SOLVED with cross-run pubkey identity verified by `diff -q` |
    58	| TB-10 | ✅ user CLI submits TaskOpen+EscrowLock signed by Agent_user_0 (real Ed25519); evaluator user-mode subprocess routes through `submit_typed_tx`; sponsor + solver role separation visible at binary boundary; user CLI's `view-*` reads chaintape via `replay_full_transition` (no Sequencer bootstrap) | ✅ chaintape genesis QState built via `runtime::bootstrap::default_pput_preseed_pairs()` factory (12-entry preseed: tb7-7-sponsor + Agent_user_0 + Agent_0..9 totaling 30M micro); Agent_user_0 keypair loaded from durable keystore at boot via TB-9 carry; first-product loop closes end-to-end | — | Lean Proof Task Market MVP SHIPPED 2026-05-02 (first user-facing product); Class 2 primary + Class 3 audit (first new caller class for already-Class-3 economic mutators); recursive self-audit PASS (4 clauses + 11 ship gates + 6 failure modes); 731 tests / 0 fail / 150 ignored (+8 net vs TB-9 baseline 723); 3/3 smoke runs SOLVED across 3 distinct heldout-49 problems (mathd_algebra_171/107 + mathd_numbertheory_961) with bounties 100k/100k/250k micro; cross-run pubkey identity verified for both Agent_user_0 + Agent_0; sponsor balance debited by exact bounty in every run; solver balance credited by exact bounty in every run |
    59	| TB-11 (planned) | ✅ price-as-statistical-signal in output (not in predicates) | — | — | NodePosition + PriceIndex v0; no trading |
    60	| TB-12 (planned) | ✅ economic mutator (CompleteSet) | — | — | CompleteSet + MarketSeedTx; CTF semantics in code |
    61	| TB-13 (planned) | ✅ economic mutator (CPMM Router) | — | — | CPMM Router; constant-product invariant |
    62	| TB-14 (planned) | ◯ scheduler / read-view (NOT predicate / NOT ledger) | — | — | Boltzmann masking + two-axis P_accept / P_progress |
    63	| TB-15 (planned) | — | — | ✅ logs archive → ArchitectAI feedback → re-init | Markov Log Loom + EvidenceCapsule; first Flowchart 3 closure |
    64	| TB-16 (planned) | ✅ all loops live | ✅ boot fully observable | ✅ EvidenceCapsule per session | Beta with market signals |
    65	| TB-17 (planned) | ✅ trade ledger | — | — | Full market trading (post-v1.0) |
    66	
    67	---
    68	
    69	## §4 Validation tests by flowchart
    70	
    71	### 4.1 Flowchart 1 (runtime loop)
    72	
    73	For any TB that touches the runtime loop, the following invariants must hold:
    74	
    75	```text
    76	1. Every externalized proposal lands in L4 (accepted) or L4.E (rejected).
    77	   - No "third place" for failed proposals.
    78	   - No accepted node without predicate-passing evidence.
    79	
    80	2. Predicate evidence resolves from CAS.
    81	   - L4 or L4.E entry → CID → CAS → evidence blob → sha256 verifies.
    82	
    83	3. Dashboard is materialized view.
    84	   - Dashboard is regeneratable from ChainTape + CAS alone.
    85	   - Dashboard does not have authoritative state.
    86	
    87	4. Predicate failure does not advance Q_t.
    88	   - q_state.ledger_root unchanged on rejected proposals.
    89	   - L4.E append is a separate ledger slot, not a Q_t mutation.
    90	```
    91	
    92	Reference: TB-7R 4-clause acceptance + 7-condition ship gate (`handover/audits/RECURSIVE_AUDIT_TB_7R_2026-05-02.md`).
    93	
    94	### 4.2 Flowchart 2 (boot)
    95	
    96	For any TB that touches boot or Q_0:
    97	
    98	```text
    99	1. Boot artifact exists and is replayable.
   100	   - genesis_report.json (or successor) lives in run evidence dir.
   101	   - Replay reconstructs Q_0 byte-exactly from the artifact.
   102	
   103	2. on_init is the sole legal mint point.
   104	   - No post-init mint may appear in any TypedTx variant.
   105	   - MarketMakerBudget (when introduced TB-12+) is allocated AT on_init,
   106	     consumed thereafter, never refilled by future mint.
   107	
   108	3. TaskOpen / EscrowLock are observable from L4 (post-TB-7R).
   109	   - Memory-only preseed is forbidden as production evidence.
   110	```
   111	
   112	Reference: TB-7R Atom C+D commit `392a516`; `handover/architect-insights/CONSTITUTION_ART_0_5_DRAFT_2026-04-26.md`.
   113	
   114	### 4.3 Flowchart 3 (meta)
   115	
   116	For any TB that touches the meta loop (logs / ArchitectAI / JudgeAI / re-init):
   117	
   118	```text
   119	1. EvidenceCapsule produced at session end.
   120	   - Per Markov rule: latest capsule + constitution = default context.

exec
/bin/bash -lc 'sha256sum constitution.md handover/alignment/TRACE_FLOWCHART_MATRIX.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
eec695459c71fbef3685583485deb431fe3b561657b2f285b7c5e7e220e59e03  constitution.md
3f8c7afba5941f120138cbce525a5f649b43b61bc23c6e6a00ae76abc73f3fa5  handover/alignment/TRACE_FLOWCHART_MATRIX.md

exec
/bin/bash -lc 'rg -n "pub fn put|pub fn get|from_content|ObjectType" src/bottom_white/cas/store.rs src/bottom_white/cas/schema.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bottom_white/cas/schema.rs:18:    pub fn from_content(content: &[u8]) -> Self {
src/bottom_white/cas/schema.rs:42:pub enum ObjectType {
src/bottom_white/cas/schema.rs:101:    pub object_type: ObjectType,
src/bottom_white/cas/schema.rs:138:        let cid = Cid::from_content(b"");
src/bottom_white/cas/schema.rs:148:        let cid_a = Cid::from_content(b"hello");
src/bottom_white/cas/schema.rs:149:        let cid_b = Cid::from_content(b"hello");
src/bottom_white/cas/schema.rs:155:        let cid_a = Cid::from_content(b"hello");
src/bottom_white/cas/schema.rs:156:        let cid_b = Cid::from_content(b"world");
src/bottom_white/cas/schema.rs:162:        let cid = Cid::from_content(b"x");
src/bottom_white/cas/schema.rs:171:            cid: Cid::from_content(b"x"),
src/bottom_white/cas/schema.rs:173:            object_type: ObjectType::ProposalPayload,
src/bottom_white/cas/schema.rs:185:            cid: Cid::from_content(b"x"),
src/bottom_white/cas/schema.rs:187:            object_type: ObjectType::ProposalPayload,
src/bottom_white/cas/schema.rs:194:        variant.object_type = ObjectType::CounterexamplePayload;
src/bottom_white/cas/store.rs:26:use git2::{ObjectType as Git2ObjectType, Repository};
src/bottom_white/cas/store.rs:33:use super::schema::{CasObjectMetadata, Cid, ObjectType};
src/bottom_white/cas/store.rs:178:    pub fn put(
src/bottom_white/cas/store.rs:181:        object_type: ObjectType,
src/bottom_white/cas/store.rs:186:        let cid = Cid::from_content(content);
src/bottom_white/cas/store.rs:214:    pub fn get(&self, cid: &Cid) -> Result<Vec<u8>, CasError> {
src/bottom_white/cas/store.rs:276:        let cid = s.put(b"hello world", ObjectType::ProposalPayload, "alice", 100, None).unwrap();
src/bottom_white/cas/store.rs:285:        let cid = s.put(&big, ObjectType::PredicateBytecode, "system", 0, Some("wasm".into())).unwrap();
src/bottom_white/cas/store.rs:293:        let cid_a = s.put(b"x", ObjectType::Generic, "alice", 1, None).unwrap();
src/bottom_white/cas/store.rs:294:        let cid_b = s.put(b"x", ObjectType::Generic, "bob", 2, None).unwrap();
src/bottom_white/cas/store.rs:303:        let cid = s.put(b"specific content", ObjectType::Generic, "system", 0, None).unwrap();
src/bottom_white/cas/store.rs:305:        let expected = Cid::from_content(b"specific content");
src/bottom_white/cas/store.rs:322:        let cid = s.put(b"meta test", ObjectType::CounterexamplePayload, "carol", 250, Some("v1".into())).unwrap();
src/bottom_white/cas/store.rs:325:        assert_eq!(meta.object_type, ObjectType::CounterexamplePayload);
src/bottom_white/cas/store.rs:337:            s1.put(content, ObjectType::Generic, "system", 0, None).unwrap();
src/bottom_white/cas/store.rs:341:            s2.put(content, ObjectType::Generic, "system", 0, None).unwrap();
src/bottom_white/cas/store.rs:362:        let cid_a = store_a.put(b"only in a", ObjectType::Generic, "agent_a", 100, None).unwrap();
src/bottom_white/cas/store.rs:363:        let cid_b = store_b.put(b"only in b", ObjectType::Generic, "agent_b", 100, None).unwrap();
src/bottom_white/cas/store.rs:378:                ObjectType::ProposalPayload,
src/bottom_white/cas/store.rs:428:                            ObjectType::Generic,
src/bottom_white/cas/store.rs:469:                .put(b"alpha", ObjectType::ProposalPayload, "alice", 1, None)
src/bottom_white/cas/store.rs:472:                .put(b"beta", ObjectType::CounterexamplePayload, "bob", 2, Some("s.v1".into()))
src/bottom_white/cas/store.rs:486:        assert_eq!(meta_b.object_type, ObjectType::CounterexamplePayload);
src/bottom_white/cas/store.rs:495:            .put(b"content", ObjectType::Generic, "alice", 1, None)
src/bottom_white/cas/store.rs:498:            .put(b"content", ObjectType::Generic, "alice", 1, None)
src/bottom_white/cas/store.rs:521:                ObjectType::Generic,
src/bottom_white/cas/store.rs:544:            s.put(b"hello", ObjectType::Generic, "alice", 1, None).unwrap();

exec
/bin/bash -lc "nl -ba src/bottom_white/cas/store.rs | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! CAS store backed by git2-rs blob layer.
     2	//!
     3	//! Each runtime_repo (per spec § 5.2.2 cell isolation) has its own CasStore.
     4	//! Objects are content-addressed by `Cid` (sha256 of content); git's sha-1
     5	//! OID is recorded but not canonical.
     6	//!
     7	//! **CO1.4-extra (this atom)** adds index persistence: the `Cid → metadata`
     8	//! map is durably persisted to a sidecar JSONL file at
     9	//! `<repo_path>/.turingos_cas_index.jsonl`. On `CasStore::open()` the sidecar
    10	//! is replayed into an in-memory BTreeMap; on `CasStore::put()` (new entries
    11	//! only) one JSONL line is appended + flushed. This closes the Art 0.2
    12	//! tape-canonicality cold-replay gate that CO1.7 spec § 0 + CO1.1.4-pre1
    13	//! v1.1 § 0.1 declared a hard prerequisite for `replay_full_transition`
    14	//! (CO1.7-impl A4).
    15	//!
    16	//! **Design choice (sidecar JSONL)**: chosen over (b) git-tag manifest /
    17	//! (c) bincode index + WAL because (a) is the simplest deterministic
    18	//! append-only artifact, replayable from scratch, easy to audit by reading.
    19	//! Per "压缩即智能" — pick simplest correct shape; upgrade later if profiling
    20	//! shows O(N)-on-restart cost is real.
    21	//!
    22	//! /// TRACE_MATRIX WP-arch-§5.L3 + spec-§5.2.2 (cell isolation): CAS store
    23	//! /// TRACE_MATRIX CO1.7 spec § 0 + CO1.1.4-pre1 § 0.1 cross-atom ordering:
    24	//! /// CAS index persistence — required by `replay_full_transition` cold-restart.
    25	
    26	use git2::{ObjectType as Git2ObjectType, Repository};
    27	use sha2::{Digest, Sha256};
    28	use std::collections::BTreeMap;
    29	use std::fs::OpenOptions;
    30	use std::io::{self, Write};
    31	use std::path::{Path, PathBuf};
    32	
    33	use super::schema::{CasObjectMetadata, Cid, ObjectType};
    34	
    35	const CAS_INDEX_FILENAME: &str = ".turingos_cas_index.jsonl";
    36	
    37	#[derive(Debug)]
    38	pub enum CasError {
    39	    /// git2-rs underlying error.
    40	    Git2(git2::Error),
    41	    /// Cid not found in this CasStore's metadata index.
    42	    CidNotFound(Cid),
    43	    /// Content stored at git OID but Cid metadata absent (corrupted index).
    44	    MetadataMissing(Cid),
    45	    /// Content's sha256 doesn't match the asserted Cid (corruption).
    46	    CidMismatch { expected: Cid, computed: Cid },
    47	    /// I/O error reading or writing the CO1.4-extra sidecar index file.
    48	    IoError(io::Error),
    49	    /// JSON-deserialization error on a sidecar index line. Includes 1-based
    50	    /// line number for diagnostics.
    51	    IndexParse { line: usize, error: String },
    52	}
    53	
    54	impl std::fmt::Display for CasError {
    55	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    56	        match self {
    57	            Self::Git2(e) => write!(f, "git2 backend error: {e}"),
    58	            Self::CidNotFound(c) => write!(f, "{c} not found in CAS index"),
    59	            Self::MetadataMissing(c) => write!(f, "{c} metadata missing (index corrupted)"),
    60	            Self::CidMismatch { expected, computed } => write!(
    61	                f,
    62	                "CAS content corruption: expected {expected}, computed {computed}"
    63	            ),
    64	            Self::IoError(e) => write!(f, "cas index I/O error: {e}"),
    65	            Self::IndexParse { line, error } => {
    66	                write!(f, "cas index parse error at line {line}: {error}")
    67	            }
    68	        }
    69	    }
    70	}
    71	
    72	impl std::error::Error for CasError {}
    73	
    74	impl From<git2::Error> for CasError {
    75	    fn from(e: git2::Error) -> Self {
    76	        Self::Git2(e)
    77	    }
    78	}
    79	
    80	impl From<io::Error> for CasError {
    81	    fn from(e: io::Error) -> Self {
    82	        Self::IoError(e)
    83	    }
    84	}
    85	
    86	fn cas_index_path(repo_path: &Path) -> PathBuf {
    87	    repo_path.join(CAS_INDEX_FILENAME)
    88	}
    89	
    90	/// CO1.4-extra: read the sidecar JSONL into an in-memory index.
    91	/// Strict mode — any malformed line aborts the load (per Art 0.2: a
    92	/// corrupted index means the tape is non-canonical; abort + diagnose
    93	/// is more honest than skip-and-warn).
    94	fn load_index_from_sidecar(repo_path: &Path) -> Result<BTreeMap<Cid, CasObjectMetadata>, CasError> {
    95	    let path = cas_index_path(repo_path);
    96	    let mut index = BTreeMap::new();
    97	    if !path.exists() {
    98	        return Ok(index);
    99	    }
   100	    let content = std::fs::read_to_string(&path)?;
   101	    for (i, line) in content.lines().enumerate() {
   102	        if line.is_empty() {
   103	            continue;
   104	        }
   105	        let meta: CasObjectMetadata =
   106	            serde_json::from_str(line).map_err(|e| CasError::IndexParse {
   107	                line: i + 1,
   108	                error: e.to_string(),
   109	            })?;
   110	        index.insert(meta.cid, meta);
   111	    }
   112	    Ok(index)
   113	}
   114	
   115	/// CO1.4-extra: append a single JSONL line for a newly-created CAS object.
   116	/// Followed by `sync_data` for durability.
   117	///
   118	/// **TB-7.6 fix (2026-05-01)**: write the JSON line + trailing newline
   119	/// in ONE `write_all` call instead of two. POSIX `O_APPEND` guarantees
   120	/// atomicity for individual writes ≤ PIPE_BUF (4096 bytes typical;
   121	/// CasObjectMetadata serializes to ~300-400 bytes). Pre-fix used two
   122	/// separate `write_all` calls (`serialized` then `b"\n"`), which could
   123	/// interleave with another concurrent writer's append, producing
   124	/// corrupted lines like `{...}{...}` (no separator). Discovered during
   125	/// TB-7 real-LLM smoke runs 2 + 5 (mathd_algebra_171 + mathd_numbertheory_5)
   126	/// where evaluator opens multiple CasStore handles concurrently for
   127	/// per-tx writes (Atom 1.5 ProposalTelemetry CAS + Atom 5
   128	/// agent_audit_trail synthetic seed + Atoms 2/3 evaluator hot-path
   129	/// telemetry writes). See
   130	/// `handover/evidence/tb_7_real_smoke_5_problems_2026-05-01/README.md` §3.
   131	fn append_to_sidecar(repo_path: &Path, meta: &CasObjectMetadata) -> Result<(), CasError> {
   132	    let path = cas_index_path(repo_path);
   133	    let serialized = serde_json::to_string(meta).map_err(|e| CasError::IndexParse {
   134	        line: 0,
   135	        error: format!("serialize: {e}"),
   136	    })?;
   137	    let mut f = OpenOptions::new()
   138	        .create(true)
   139	        .append(true)
   140	        .open(&path)?;
   141	    // Atomic single-write append: serialize + newline in one buffer.
   142	    let mut line = serialized.into_bytes();
   143	    line.push(b'\n');
   144	    f.write_all(&line)?;
   145	    f.sync_data()?;
   146	    Ok(())
   147	}
   148	
   149	/// Content-addressable store backed by git's blob object database.
   150	#[derive(Debug)]
   151	pub struct CasStore {
   152	    repo_path: PathBuf,
   153	    /// Cid → metadata index. BTreeMap per spec § 2 I-BTREE.
   154	    index: BTreeMap<Cid, CasObjectMetadata>,
   155	}
   156	
   157	impl CasStore {
   158	    /// Open or initialize a CAS store at the given runtime_repo path.
   159	    /// Creates the git repo if it doesn't exist. **CO1.4-extra**: replays
   160	    /// the sidecar `.turingos_cas_index.jsonl` (if any) into the in-memory
   161	    /// index, restoring all metadata that was durably appended in prior
   162	    /// sessions.
   163	    pub fn open(repo_path: &Path) -> Result<Self, CasError> {
   164	        let repo_path = repo_path.to_path_buf();
   165	        let _repo = match Repository::open(&repo_path) {
   166	            Ok(r) => r,
   167	            Err(_) => Repository::init(&repo_path)?,
   168	        };
   169	        let index = load_index_from_sidecar(&repo_path)?;
   170	        Ok(Self { repo_path, index })
   171	    }
   172	
   173	    fn open_repo(&self) -> Result<Repository, CasError> {
   174	        Repository::open(&self.repo_path).map_err(CasError::from)
   175	    }
   176	
   177	    /// Store content; returns its Cid. Idempotent — same content → same Cid.
   178	    pub fn put(
   179	        &mut self,
   180	        content: &[u8],
   181	        object_type: ObjectType,
   182	        creator: &str,
   183	        created_at_logical_t: u64,
   184	        schema_id: Option<String>,
   185	    ) -> Result<Cid, CasError> {
   186	        let cid = Cid::from_content(content);
   187	        let repo = self.open_repo()?;
   188	        let git_oid = repo.blob(content)?;
   189	
   190	        // If already in index, idempotent: just return Cid (content addressing
   191	        // guarantees same content → same Cid → already present)
   192	        if self.index.contains_key(&cid) {
   193	            return Ok(cid);
   194	        }
   195	
   196	        let metadata = CasObjectMetadata {
   197	            cid,
   198	            backend_oid_hex: git_oid.to_string(),
   199	            object_type,
   200	            creator: creator.to_string(),
   201	            created_at_logical_t,
   202	            schema_id,
   203	            size_bytes: content.len() as u64,
   204	        };
   205	        // CO1.4-extra: durably append BEFORE inserting into in-memory index
   206	        // (so a crash mid-write leaves the runtime in a consistent state —
   207	        // either the entry is durably recorded AND in-memory, or neither).
   208	        append_to_sidecar(&self.repo_path, &metadata)?;
   209	        self.index.insert(cid, metadata);
   210	        Ok(cid)
   211	    }
   212	
   213	    /// Retrieve content by Cid. Verifies content sha256 matches Cid (corruption check).
   214	    pub fn get(&self, cid: &Cid) -> Result<Vec<u8>, CasError> {
   215	        let metadata = self
   216	            .index
   217	            .get(cid)
   218	            .ok_or(CasError::CidNotFound(*cid))?;
   219	        let repo = self.open_repo()?;
   220	        let git_oid = git2::Oid::from_str(&metadata.backend_oid_hex)

exec
/bin/bash -lc "nl -ba src/bottom_white/cas/schema.rs | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! CAS object schema per WP architecture § 5.L3.
     2	//!
     3	//! /// TRACE_MATRIX WP-arch-§5.L3: CAS object schema
     4	
     5	use serde::{Deserialize, Serialize};
     6	use sha2::{Digest, Sha256};
     7	
     8	/// Content-addressed identifier — sha256 of payload bytes.
     9	///
    10	/// Distinct from git's SHA-1 OID (which is an internal storage detail of
    11	/// the git2-rs backend). `Cid` is the v4-canonical identifier; spec § 1.2
    12	/// `WorkTx.proposal_cid: Cid` references this.
    13	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    14	pub struct Cid(pub [u8; 32]);
    15	
    16	impl Cid {
    17	    /// Compute Cid from content bytes.
    18	    pub fn from_content(content: &[u8]) -> Self {
    19	        let mut h = Sha256::new();
    20	        h.update(content);
    21	        Self(h.finalize().into())
    22	    }
    23	
    24	    /// Hex-encoded representation (lowercase; 64 chars).
    25	    pub fn hex(&self) -> String {
    26	        let mut s = String::with_capacity(64);
    27	        for b in self.0 {
    28	            s.push_str(&format!("{:02x}", b));
    29	        }
    30	        s
    31	    }
    32	}
    33	
    34	impl std::fmt::Display for Cid {
    35	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    36	        write!(f, "cid:{}", self.hex())
    37	    }
    38	}
    39	
    40	/// Type tag for CAS objects (replaces inline string-typed kind).
    41	#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    42	pub enum ObjectType {
    43	    /// Agent's work_tx proposal payload (Lean proof, code patch, etc.).
    44	    ProposalPayload,
    45	    /// Challenger's counterexample for a slashed claim.
    46	    CounterexamplePayload,
    47	    /// Predicate bytecode (Lean tactic, WASM module, Rust source bytes).
    48	    PredicateBytecode,
    49	    /// Tool bytecode.
    50	    ToolBytecode,
    51	    /// Constitution diff (for amendment proposals).
    52	    AmendmentDiff,
    53	    /// Reversibility plan attached to a meta_tx.
    54	    ReversibilityPlan,
    55	    /// TB-11 (architect §6.1): canonical-encoded `EvidenceCapsule` bytes.
    56	    /// Referenced by `TerminalSummaryTx.evidence_capsule_cid` /
    57	    /// `TaskBankruptcyTx.evidence_capsule_cid` for O(N) audit access while
    58	    /// keeping L4 chain cost O(1).
    59	    EvidenceCapsule,
    60	    /// TB-11 (architect §6.1): JSON manifest enumerating sub-CAS objects
    61	    /// of an EvidenceCapsule (e.g. compressed log Cid + size + sha256).
    62	    EvidenceManifest,
    63	    /// TB-11 (architect §6.1): gzipped raw run log bytes (audit-only access).
    64	    /// Privacy default `CapsulePrivacyPolicy::AuditOnly` — never enters
    65	    /// Agent read view.
    66	    CompressedRunLog,
    67	    /// TB-15 (architect §6.2): canonical-encoded `AgentAutopsyCapsule`
    68	    /// bytes. Per-agent, per-event loss capsule derived from ChainTape
    69	    /// evidence (NEVER from agent self-narration). Anchored from
    70	    /// `EconomicState.agent_autopsies_t[event_id]: Vec<Cid>`.
    71	    /// Privacy default `CapsulePrivacyPolicy::AuditOnly`; only
    72	    /// `public_summary` text may broadcast on N≥3 typical-error cluster.
    73	    AgentAutopsyCapsule,
    74	    /// TB-15 (architect §6.2): private-detail JSON for an
    75	    /// `AgentAutopsyCapsule`. Referenced by
    76	    /// `AgentAutopsyCapsule.private_detail_cid`. Audit-only by default;
    77	    /// MUST NOT enter `AgentVisibleProjection`.
    78	    AutopsyPrivateDetail,
    79	    /// TB-15 (architect §6.2): canonical-encoded `MarkovEvidenceCapsule`
    80	    /// bytes. End-of-TB rollup binding constitution_hash + L4 root +
    81	    /// L4.E root + CAS root + previous_capsule_cid + typical_errors +
    82	    /// unresolved_obs + next_session_context_cid. Default next-session
    83	    /// bootstrap source; deeper history requires `TURINGOS_MARKOV_OVERRIDE=1`.
    84	    MarkovEvidenceCapsule,
    85	    /// TB-15 (architect §6.2 FR-15.4): JSON blob describing the next
    86	    /// session's default boot context (`{constitution_hash,
    87	    /// latest_markov_cid, boot_seq[]}`). Referenced by
    88	    /// `MarkovEvidenceCapsule.next_session_context_cid`.
    89	    NextSessionContext,
    90	    /// Generic / unclassified blob.
    91	    Generic,
    92	}
    93	
    94	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    95	pub struct CasObjectMetadata {
    96	    /// Content-addressed identifier (sha256 of content).
    97	    pub cid: Cid,
    98	    /// Backend-specific OID (git sha-1 for git2-rs backend); informational only.
    99	    /// Different backends may have different OID schemes; Cid is canonical.
   100	    pub backend_oid_hex: String,
   101	    pub object_type: ObjectType,
   102	    /// Submitter / author. Use "system" for runtime-emitted objects.
   103	    pub creator: String,
   104	    /// Logical time at insertion (assigned by sequencer; not wall clock).
   105	    pub created_at_logical_t: u64,
   106	    /// Optional schema identifier (JSON Schema URI, type tag, etc.).
   107	    pub schema_id: Option<String>,
   108	    /// Size of content in bytes (informational; not part of canonical hash).
   109	    pub size_bytes: u64,
   110	}
   111	
   112	impl CasObjectMetadata {
   113	    /// Canonical hash of metadata for Merkle tree inclusion.
   114	    pub fn canonical_hash(&self) -> [u8; 32] {
   115	        let mut h = Sha256::new();
   116	        h.update(self.cid.0);
   117	        h.update(self.backend_oid_hex.as_bytes());
   118	        h.update(serde_json::to_vec(&self.object_type).expect("object_type serialize"));
   119	        h.update(self.creator.as_bytes());
   120	        h.update(self.created_at_logical_t.to_be_bytes());

exec
/bin/bash -lc "nl -ba src/bottom_white/cas/store.rs | sed -n '213,245p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   213	    /// Retrieve content by Cid. Verifies content sha256 matches Cid (corruption check).
   214	    pub fn get(&self, cid: &Cid) -> Result<Vec<u8>, CasError> {
   215	        let metadata = self
   216	            .index
   217	            .get(cid)
   218	            .ok_or(CasError::CidNotFound(*cid))?;
   219	        let repo = self.open_repo()?;
   220	        let git_oid = git2::Oid::from_str(&metadata.backend_oid_hex)
   221	            .map_err(CasError::Git2)?;
   222	        let blob = repo.find_blob(git_oid)?;
   223	        let content = blob.content().to_vec();
   224	
   225	        // Verify content sha256 matches Cid (defense against corruption).
   226	        let mut h = Sha256::new();
   227	        h.update(&content);
   228	        let computed = Cid(h.finalize().into());
   229	        if &computed != cid {
   230	            return Err(CasError::CidMismatch {
   231	                expected: *cid,
   232	                computed,
   233	            });
   234	        }
   235	
   236	        Ok(content)
   237	    }
   238	
   239	    /// Get metadata only (no content fetch).
   240	    pub fn metadata(&self, cid: &Cid) -> Option<&CasObjectMetadata> {
   241	        self.index.get(cid)
   242	    }
   243	
   244	    pub fn len(&self) -> usize {
   245	        self.index.len()

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '2720,2845p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  2720	    // path, but explicit drain is best-effort only on the canonical exit.
  2721	    if let Some(bundle) = chaintape_bundle {
  2722	        // TB-6 Atom 6: build the RunSummary from the on-disk chain BEFORE the
  2723	        // bundle is consumed by `shutdown()` (RunSummary needs the runtime_repo
  2724	        // path + cas path + a final read of L4 / L4.E). Caller-supplied
  2725	        // `failed_branch_count` and `rollback_count` mirror PputResult.
  2726	        let runtime_repo_path = bundle.runtime_repo_path.clone();
  2727	        let cas_path = bundle.cas_path.clone();
  2728	
  2729	        // ────────────────────────────────────────────────────────────────────
  2730	        // TB-11 Atom 0.5(a) carry-forward landed in TB-12 (architect 2026-05-03
  2731	        // ruling §1.1 + §8 Atom 0.5; SG-11.1 + SG-11.2). MAX_TX exhausted →
  2732	        // write EvidenceCapsule to CAS + emit TerminalSummary on-chain via
  2733	        // tb11_emit_terminal_summary_for_run. emit_system_tx queues the tx;
  2734	        // bundle.shutdown() below drains via apply_one (mirror of TB-8
  2735	        // tb8_emit_finalize_after_verify pattern).
  2736	        // ────────────────────────────────────────────────────────────────────
  2737	        {
  2738	            use turingosv4::bottom_white::cas::store::CasStore;
  2739	            use turingosv4::runtime::evidence_capsule::{
  2740	                write_evidence_capsule, ExhaustionCounts,
  2741	            };
  2742	            use turingosv4::state::typed_tx::{
  2743	                CapsulePrivacyPolicy, ExhaustionReason, RejectionClass, RunOutcome,
  2744	            };
  2745	            use std::sync::{Arc, RwLock};
  2746	
  2747	            let counts = ExhaustionCounts {
  2748	                attempt_count: proposal_count,
  2749	                lean_error_count: tb11_lean_error_count,
  2750	                sorry_block_count: tb11_sorry_block_count,
  2751	                protocol_parse_failure_count: tb11_protocol_parse_failure_count,
  2752	                partial_accept_count: tb11_partial_accept_count,
  2753	            };
  2754	            // Deterministic public_summary substrate. TB-11 MVP stores
  2755	            // uncompressed; gzip wrapping deferred to TB-15 Markov Loom.
  2756	            let raw_log = format!(
  2757	                "TB-11 Atom 0.5(a) carry-forward MAX_TX exhausted run summary\n\
  2758	                 run_id: {}\n\
  2759	                 task_id: task-{}\n\
  2760	                 proposal_count: {}\n\
  2761	                 lean_error_count: {}\n\
  2762	                 sorry_block_count: {}\n\
  2763	                 protocol_parse_failure_count: {}\n\
  2764	                 partial_accept_count: {}\n\
  2765	                 verifier_wait_ms: {}\n\
  2766	                 max_transactions: {}\n",
  2767	                run_id, run_id, proposal_count, tb11_lean_error_count,
  2768	                tb11_sorry_block_count, tb11_protocol_parse_failure_count,
  2769	                tb11_partial_accept_count, verifier_wait_ms, max_transactions,
  2770	            );
  2771	            match CasStore::open(&cas_path) {
  2772	                Ok(cas_store) => {
  2773	                    let cas = Arc::new(RwLock::new(cas_store));
  2774	                    let task_id_capsule =
  2775	                        turingosv4::state::q_state::TaskId(format!("task-{}", run_id));
  2776	                    let run_id_capsule =
  2777	                        turingosv4::state::typed_tx::RunId(run_id.clone());
  2778	                    match write_evidence_capsule(
  2779	                        &cas,
  2780	                        run_id_capsule.clone(),
  2781	                        task_id_capsule.clone(),
  2782	                        None, // solver_agent — multi-agent swarm; no single solver
  2783	                        counts,
  2784	                        (0, max_transactions as u64),
  2785	                        ExhaustionReason::MaxTxExhausted,
  2786	                        raw_log.as_bytes(),
  2787	                        CapsulePrivacyPolicy::AuditOnly,
  2788	                        "evaluator-tb11",
  2789	                        proposal_count,
  2790	                    ) {
  2791	                        Ok(capsule) => {
  2792	                            info!(
  2793	                                "[tb11] EvidenceCapsule written: capsule_id={} \
  2794	                                 compressed_log_cid={} attempt_count={}",
  2795	                                capsule.capsule_id.hex(),
  2796	                                capsule.compressed_log_cid.hex(),
  2797	                                capsule.attempt_count
  2798	                            );
  2799	                            // emit TerminalSummary on-chain.
  2800	                            let mut hist: std::collections::BTreeMap<
  2801	                                RejectionClass,
  2802	                                u32,
  2803	                            > = std::collections::BTreeMap::new();
  2804	                            if tb11_lean_error_count > 0 {
  2805	                                hist.insert(
  2806	                                    RejectionClass::Opaque,
  2807	                                    tb11_lean_error_count.min(u32::MAX as u64) as u32,
  2808	                                );
  2809	                            }
  2810	                            match turingosv4::runtime::adapter::tb11_emit_terminal_summary_for_run(
  2811	                                bundle.sequencer.as_ref(),
  2812	                                run_id_capsule,
  2813	                                task_id_capsule,
  2814	                                RunOutcome::MaxTxExhausted,
  2815	                                proposal_count.min(u32::MAX as u64) as u32,
  2816	                                hist,
  2817	                                max_transactions as u64,
  2818	                                None, // solver_agent
  2819	                                Some(capsule.capsule_id),
  2820	                            )
  2821	                            .await
  2822	                            {
  2823	                                Ok(receipt) => info!(
  2824	                                    "[tb11] TerminalSummary emitted: emit_id={}",
  2825	                                    receipt.emit_id
  2826	                                ),
  2827	                                Err(e) => warn!(
  2828	                                    "[tb11] TerminalSummary emit failed: {e:?}"
  2829	                                ),
  2830	                            }
  2831	                        }
  2832	                        Err(e) => warn!("[tb11] EvidenceCapsule write failed: {e:?}"),
  2833	                    }
  2834	                }
  2835	                Err(e) => warn!("[tb11] CasStore::open failed: {e:?}"),
  2836	            }
  2837	        }
  2838	
  2839	        if let Err(e) = bundle.shutdown().await {
  2840	            error!("[chaintape] driver shutdown returned error: {e}");
  2841	        }
  2842	        match turingosv4::runtime::run_summary::RunSummary::from_chaintape(
  2843	            &runtime_repo_path,
  2844	            &cas_path,
  2845	            &run_id,

exec
/bin/bash -lc "nl -ba src/runtime/adapter.rs | sed -n '450,515p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   450	    }
   451	    sequencer
   452	        .emit_system_tx(crate::state::sequencer::SystemEmitCommand::FinalizeReward {
   453	            claim_id,
   454	        })
   455	        .await
   456	        .map(|_| true)
   457	}
   458	
   459	// ────────────────────────────────────────────────────────────────────────────
   460	// TB-11 Atom 4 — Runtime emission helpers (architect §6.2 ruling 2026-05-02)
   461	// ────────────────────────────────────────────────────────────────────────────
   462	
   463	/// TRACE_MATRIX TB-11 Atom 4 (architect §6.2): emit a TerminalSummaryTx
   464	/// (≡ RunExhaustedTx) for a failed evaluator run. Caller passes the
   465	/// run-summary fields directly + an optional pre-written
   466	/// `evidence_capsule_cid` (callers should write the EvidenceCapsule first
   467	/// via `evidence_capsule::write_evidence_capsule`).
   468	///
   469	/// Returns `Ok(receipt)` on success; `Err` carries the same EmitSystemError
   470	/// taxonomy as `tb8_emit_finalize_after_verify`.
   471	///
   472	/// **Architect mandate** (TB-11 charter §3 Atom 4 + ship gate G4):
   473	/// evaluator on MAX_TX exhausted / timeout / solver give-up should:
   474	///   1. Build EvidenceCapsule via `evidence_capsule::write_evidence_capsule`.
   475	///   2. Call this helper with `evidence_capsule_cid = Some(capsule.capsule_id)`.
   476	///
   477	/// For OmegaAccepted runs, evidence_capsule_cid is `None` (success path
   478	/// has no failure evidence).
   479	pub async fn tb11_emit_terminal_summary_for_run(
   480	    sequencer: &crate::state::sequencer::Sequencer,
   481	    run_id: crate::state::typed_tx::RunId,
   482	    task_id: TaskId,
   483	    run_outcome: crate::state::typed_tx::RunOutcome,
   484	    total_attempts: u32,
   485	    failure_class_histogram:
   486	        std::collections::BTreeMap<crate::state::typed_tx::RejectionClass, u32>,
   487	    last_logical_t: u64,
   488	    solver_agent: Option<AgentId>,
   489	    evidence_capsule_cid: Option<crate::bottom_white::cas::schema::Cid>,
   490	) -> Result<crate::state::sequencer::SystemEmitReceipt, crate::state::sequencer::EmitSystemError> {
   491	    sequencer
   492	        .emit_system_tx(crate::state::sequencer::SystemEmitCommand::TerminalSummary {
   493	            run_id,
   494	            task_id,
   495	            run_outcome,
   496	            total_attempts,
   497	            failure_class_histogram,
   498	            last_logical_t,
   499	            solver_agent,
   500	            evidence_capsule_cid,
   501	        })
   502	        .await
   503	}
   504	
   505	/// TRACE_MATRIX TB-11 Atom 4 (architect §6.2 + §7.4 capital-must-flow):
   506	/// scan task_markets_t for tasks past the expiry-policy deadline + emit
   507	/// TaskExpire for each eligible escrow.
   508	///
   509	/// Eligibility (TB-11 MVP per charter §7 Q1):
   510	///   - task_markets_t[task_id].state ∈ { Open, Bankrupt }
   511	///   - current_logical_t - opened_at_logical_t > expiry_delta_logical_t
   512	///   - no Finalized claim against this task
   513	///   - (no open challenge_cases targeting this task — enforced by
   514	///     dispatch arm; helper does not pre-filter to keep policy logic
   515	///     centralized)

exec
/bin/bash -lc "nl -ba tests/tb_11_epistemic_exhaust.rs | sed -n '120,170p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   120	
   121	// ── I-TB11-1 — TerminalSummary anchors RunsIndex ────────────────────────────
   122	
   123	#[tokio::test]
   124	async fn terminal_summary_emit_then_apply_writes_runs_index() {
   125	    let q = genesis_with_sponsor("sponsor-A", 10);
   126	    let mut h = fresh_harness(q);
   127	    open_and_fund(&mut h, "sponsor-A", "task-A", 500_000).await;
   128	
   129	    let capsule_cid = Cid([0x77u8; 32]);
   130	    let mut hist = BTreeMap::new();
   131	    hist.insert(RejectionClass::Opaque, 132);
   132	    h.seq
   133	        .emit_system_tx(SystemEmitCommand::TerminalSummary {
   134	            run_id: RunId("run-zeta-001".into()),
   135	            task_id: TaskId("task-A".into()),
   136	            run_outcome: RunOutcome::MaxTxExhausted,
   137	            total_attempts: 132,
   138	            failure_class_histogram: hist,
   139	            last_logical_t: 1,
   140	            solver_agent: Some(AgentId("solver-1".into())),
   141	            evidence_capsule_cid: Some(capsule_cid),
   142	        })
   143	        .await
   144	        .expect("emit terminal-summary");
   145	    let _ = h.seq.try_apply_one(&mut h.rx).expect("env ts").expect("ok ts");
   146	
   147	    let q_after = h.seq.q_snapshot().expect("q snapshot");
   148	    let runs = &q_after.economic_state_t.runs_t.0;
   149	    assert_eq!(runs.len(), 1);
   150	    let entry: &RunSummaryEntry = runs
   151	        .get(&RunId("run-zeta-001".into()))
   152	        .expect("runs_t entry");
   153	    assert_eq!(entry.task_id, TaskId("task-A".into()));
   154	    assert_eq!(entry.attempt_count, 132);
   155	    assert_eq!(entry.evidence_capsule_cid, Some(capsule_cid));
   156	    assert_eq!(entry.run_outcome, RunOutcome::MaxTxExhausted);
   157	
   158	    // No money moved.
   159	    let bal = q_after
   160	        .economic_state_t
   161	        .balances_t
   162	        .0
   163	        .get(&AgentId("sponsor-A".into()))
   164	        .copied()
   165	        .unwrap();
   166	    assert_eq!(bal.micro_units(), MicroCoin::from_coin(10).unwrap().micro_units() - 500_000);
   167	
   168	    let escrow = q_after
   169	        .economic_state_t
   170	        .escrows_t

exec
/bin/bash -lc "nl -ba tests/tb_11_epistemic_exhaust.rs | sed -n '300,350p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   300	    hist.insert(RejectionClass::Opaque, 7);
   301	    let _receipt = tb11_emit_terminal_summary_for_run(
   302	        &h.seq,
   303	        RunId("run-S-001".into()),
   304	        TaskId("task-S".into()),
   305	        RunOutcome::WallClockCap,
   306	        7,
   307	        hist,
   308	        100,
   309	        Some(AgentId("solver-S".into())),
   310	        Some(capsule_cid),
   311	    )
   312	    .await
   313	    .expect("helper emits");
   314	    let _ = h.seq.try_apply_one(&mut h.rx).expect("env ts").expect("apply ts");
   315	
   316	    let q_after = h.seq.q_snapshot().unwrap();
   317	    let entry = q_after
   318	        .economic_state_t
   319	        .runs_t
   320	        .0
   321	        .get(&RunId("run-S-001".into()))
   322	        .expect("runs_t entry");
   323	    assert_eq!(entry.run_outcome, RunOutcome::WallClockCap);
   324	    assert_eq!(entry.evidence_capsule_cid, Some(capsule_cid));
   325	}
   326	
   327	// ── I-TB11-3 — TaskBankruptcy flips state to Bankrupt ──────────────────────
   328	
   329	#[tokio::test]
   330	async fn task_bankruptcy_flips_state() {
   331	    let q = genesis_with_sponsor("sponsor-C", 10);
   332	    let mut h = fresh_harness(q);
   333	    open_and_fund(&mut h, "sponsor-C", "task-C", 500_000).await;
   334	
   335	    let capsule_cid = Cid([0xaau8; 32]);
   336	    h.seq
   337	        .emit_system_tx(SystemEmitCommand::TaskBankruptcy {
   338	            task_id: TaskId("task-C".into()),
   339	            evidence_capsule_cid: capsule_cid,
   340	            bankruptcy_reason: BankruptcyReason::MaxFailedRunCount,
   341	            failed_run_count: 3,
   342	        })
   343	        .await
   344	        .expect("emit task-bankruptcy");
   345	    let _ = h.seq.try_apply_one(&mut h.rx).expect("env bk").expect("ok bk");
   346	
   347	    let q_after = h.seq.q_snapshot().unwrap();
   348	    let tm = q_after
   349	        .economic_state_t
   350	        .task_markets_t

exec
/bin/bash -lc "rg -n \"evidence_capsule_cid|compressed_log_cid|EvidenceCapsule|CasStore::open|cas\\.get\" src/runtime/run_summary.rs src/bin/audit_dashboard.rs experiments/minif2f_v4/src -g '"'!target'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bin/audit_dashboard.rs:183:    /// Hex of evidence_capsule_cid; "—" if None (OmegaAccepted path).
src/bin/audit_dashboard.rs:184:    evidence_capsule_cid_hex: String,
src/bin/audit_dashboard.rs:203:    evidence_capsule_cid_hex: String,
src/bin/audit_dashboard.rs:386:    let cas = CasStore::open(cas_path).map_err(|e| format!("cas open: {e}"))?;
src/bin/audit_dashboard.rs:432:        let payload_bytes = match cas.get(&entry.tx_payload_cid) {
src/bin/audit_dashboard.rs:456:                        if let Ok(payload) = cas.get(&tel.proposal_artifact_cid) {
src/bin/audit_dashboard.rs:583:                            let bytes = cas.get(&prev.tx_payload_cid).ok()?;
src/bin/audit_dashboard.rs:695:                    evidence_capsule_cid_hex: ts
src/bin/audit_dashboard.rs:696:                        .evidence_capsule_cid
src/bin/audit_dashboard.rs:746:                    evidence_capsule_cid_hex: bk.evidence_capsule_cid.hex(),
src/bin/audit_dashboard.rs:856:        if let Ok(payload_bytes) = cas.get(&record.tx_payload_cid) {
src/bin/audit_dashboard.rs:1426:    //     evidence_capsule_cid → CAS bytes; raw log shielded by AuditOnly default.
src/bin/audit_dashboard.rs:1438:        s.push_str("    run_id         | task_id            | outcome         | attempts | evidence_capsule_cid (hex)\n");
src/bin/audit_dashboard.rs:1441:            let cap_short = if er.evidence_capsule_cid_hex.len() > 32 {
src/bin/audit_dashboard.rs:1442:                format!("{}…", &er.evidence_capsule_cid_hex[0..31])
src/bin/audit_dashboard.rs:1444:                er.evidence_capsule_cid_hex.clone()
src/bin/audit_dashboard.rs:1484:        s.push_str("    task_id            | reason                | failed_runs | evidence_capsule_cid (hex)\n");
src/bin/audit_dashboard.rs:1487:            let cap_short = if bk.evidence_capsule_cid_hex.len() > 32 {
src/bin/audit_dashboard.rs:1488:                format!("{}…", &bk.evidence_capsule_cid_hex[0..31])
src/bin/audit_dashboard.rs:1490:                bk.evidence_capsule_cid_hex.clone()
src/runtime/run_summary.rs:133:        let cas = CasStore::open(cas_path).map_err(|e| RunSummaryError::Cas(e.to_string()))?;
src/runtime/run_summary.rs:165:            if let Ok(bytes) = cas.get(&record.tx_payload_cid) {
experiments/minif2f_v4/src/bin/lean_market.rs:700:        CasStore::open(cas_path).map_err(|e| format!("open cas: {e}"))?;
experiments/minif2f_v4/src/bin/evaluator.rs:1229:    // ruling §1.1 + §8 Atom 0.5): EvidenceCapsule rollup counters per
experiments/minif2f_v4/src/bin/evaluator.rs:1665:                                    let mut cas_store = match turingosv4::bottom_white::cas::store::CasStore::open(&bundle.cas_path) {
experiments/minif2f_v4/src/bin/evaluator.rs:1930:                                            let mut cas_store = match turingosv4::bottom_white::cas::store::CasStore::open(&bundle.cas_path) {
experiments/minif2f_v4/src/bin/evaluator.rs:2197:                                        // EvidenceCapsule.sorry_block_count vs lean_error_count.
experiments/minif2f_v4/src/bin/evaluator.rs:2362:                                            let mut cas_store = match turingosv4::bottom_white::cas::store::CasStore::open(&bundle.cas_path) {
experiments/minif2f_v4/src/bin/evaluator.rs:2610:                                                // partial_accept_count for EvidenceCapsule.
experiments/minif2f_v4/src/bin/evaluator.rs:2649:                        // protocol_parse_failure_count for EvidenceCapsule.
experiments/minif2f_v4/src/bin/evaluator.rs:2732:        // write EvidenceCapsule to CAS + emit TerminalSummary on-chain via
experiments/minif2f_v4/src/bin/evaluator.rs:2771:            match CasStore::open(&cas_path) {
experiments/minif2f_v4/src/bin/evaluator.rs:2793:                                "[tb11] EvidenceCapsule written: capsule_id={} \
experiments/minif2f_v4/src/bin/evaluator.rs:2794:                                 compressed_log_cid={} attempt_count={}",
experiments/minif2f_v4/src/bin/evaluator.rs:2796:                                capsule.compressed_log_cid.hex(),
experiments/minif2f_v4/src/bin/evaluator.rs:2832:                        Err(e) => warn!("[tb11] EvidenceCapsule write failed: {e:?}"),
experiments/minif2f_v4/src/bin/evaluator.rs:2835:                Err(e) => warn!("[tb11] CasStore::open failed: {e:?}"),

exec
/bin/bash -lc "nl -ba handover/evidence/tb_15_markov_capsule_2026-05-04/MARKOV_TB-15-R3_2026-05-03.json | sed -n '337,407p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   337	  "next_session_context_cid": [
   338	    83,
   339	    59,
   340	    212,
   341	    123,
   342	    234,
   343	    85,
   344	    246,
   345	    138,
   346	    24,
   347	    84,
   348	    67,
   349	    246,
   350	    164,
   351	    110,
   352	    226,
   353	    66,
   354	    202,
   355	    12,
   356	    167,
   357	    153,
   358	    115,
   359	    246,
   360	    202,
   361	    111,
   362	    234,
   363	    149,
   364	    41,
   365	    202,
   366	    26,
   367	    234,
   368	    17,
   369	    56
   370	  ],
   371	  "sha256": [
   372	    249,
   373	    231,
   374	    1,
   375	    180,
   376	    169,
   377	    194,
   378	    225,
   379	    217,
   380	    180,
   381	    209,
   382	    34,
   383	    44,
   384	    6,
   385	    166,
   386	    196,
   387	    228,
   388	    246,
   389	    81,
   390	    106,
   391	    161,
   392	    175,
   393	    28,
   394	    62,
   395	    210,
   396	    154,
   397	    244,
   398	    87,
   399	    209,
   400	    85,
   401	    50,
   402	    211,
   403	    18
   404	  ],
   405	  "created_at_logical_t": 0,
   406	  "tb_tag": "TB-15-R3"
   407	}

exec
/bin/bash -lc "nl -ba handover/evidence/tb_15_markov_capsule_2026-05-04/MARKOV_TB-15-R3_2026-05-03.json | sed -n '1,75p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	{
     2	  "capsule_id": [
     3	    249,
     4	    231,
     5	    1,
     6	    180,
     7	    169,
     8	    194,
     9	    225,
    10	    217,
    11	    180,
    12	    209,
    13	    34,
    14	    44,
    15	    6,
    16	    166,
    17	    196,
    18	    228,
    19	    246,
    20	    81,
    21	    106,
    22	    161,
    23	    175,
    24	    28,
    25	    62,
    26	    210,
    27	    154,
    28	    244,
    29	    87,
    30	    209,
    31	    85,
    32	    50,
    33	    211,
    34	    18
    35	  ],
    36	  "previous_capsule_cid": null,
    37	  "constitution_hash": [
    38	    238,
    39	    198,
    40	    149,
    41	    69,
    42	    156,
    43	    113,
    44	    251,
    45	    239,
    46	    54,
    47	    133,
    48	    88,
    49	    52,
    50	    133,
    51	    222,
    52	    180,
    53	    49,
    54	    254,
    55	    59,
    56	    86,
    57	    22,
    58	    87,
    59	    178,
    60	    242,
    61	    133,
    62	    183,
    63	    197,
    64	    231,
    65	    226,
    66	    32,
    67	    229,
    68	    158,
    69	    3
    70	  ],
    71	  "flowchart_hashes": [
    72	    [
    73	      164,
    74	      116,
    75	      198,

exec
/bin/bash -lc "rg -n \"f9e701b4|MarkovEvidenceCapsule|LATEST_MARKOV_CAPSULE|CAS index\" handover/evidence/tb_15_markov_capsule_2026-05-04 handover/markov_capsules -g '"'!*.json'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt:1:f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312
handover/evidence/tb_15_markov_capsule_2026-05-04/LATEST_MARKOV_CAPSULE.txt:1:f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312
handover/evidence/tb_15_markov_capsule_2026-05-04/cas_index.jsonl:2:{"cid":[249,231,1,180,169,194,225,217,180,209,34,44,6,166,196,228,246,81,106,161,175,28,62,210,154,244,87,209,85,50,211,18],"backend_oid_hex":"816cd91a43db05c77ad89f51c3d26b0b940ea950","object_type":"MarkovEvidenceCapsule","creator":"tb15-generator","created_at_logical_t":0,"schema_id":"v1/markov_evidence_capsule","size_bytes":2136}
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:22:- `MARKOV_TB-15-R3_2026-05-03.json` — R3 MarkovEvidenceCapsule (CAS-resolvable; flowchart_hashes populated)
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:23:- `LATEST_MARKOV_CAPSULE.txt` — Cid hex pointer (`f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312`)
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:24:- `cas_index.jsonl` — copy of CAS index showing CAS object cid matches LATEST pointer
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:29:capsule_id                : R2 a94ae884... → R3 f9e701b4...   (rebuilt with R3 fixed writer)
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:34:CAS resolvability         : R2 BROKEN (cid in CAS index ≠ capsule_id) → R3 FIXED
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:40:LATEST_MARKOV_CAPSULE.txt   = f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:41:CAS index (MarkovEvidenceCapsule) = f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:48:- `LATEST_MARKOV_CAPSULE.txt` = `a94ae884...`
handover/evidence/tb_15_markov_capsule_2026-05-04/README.md:49:- CAS index for MarkovEvidenceCapsule = `e4932fca...` (different)

exec
/bin/bash -lc "rg -n \"write_autopsy_capsule_to_cas_round_trip|cas_r\\.get\\(&cap\\.capsule_id\\)|restore_autopsy_capsule_from_cas_bytes|write_bankruptcy_autopsies_to_cas_round_trip|restored\\.\" src/runtime/autopsy_capsule.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
336:pub fn restore_autopsy_capsule_from_cas_bytes(
689:    fn write_autopsy_capsule_to_cas_round_trip() {
1080:    fn write_bankruptcy_autopsies_to_cas_round_trip() {
1137:            // derived; restored via restore_autopsy_capsule_from_cas_bytes).
1138:            let restored = restore_autopsy_capsule_from_cas_bytes(&bytes)
1141:                restored.capsule_id, *cid,

codex
## VERDICT: PASS

R3 closes the Codex R2 VETO for TB-15.

Q3 Markov closure holds: `write_markov_capsule` builds the capsule with `capsule_id = Cid::default()` and `sha256 = Hash::ZERO`, canonical-encodes those zeroed bytes, derives `cid = Cid::from_content(&stored_bytes)`, stores exactly `stored_bytes`, then populates the returned in-memory struct. See [markov_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/markov_capsule.rs:270), [markov_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/markov_capsule.rs:305), and [markov_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/markov_capsule.rs:322). Restore is symmetric at [markov_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/markov_capsule.rs:339).

TB15-CAS-ID closure also holds: `write_autopsy_capsule` uses the same zeroed-stored-bytes pattern at [autopsy_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/autopsy_capsule.rs:289) and [autopsy_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/autopsy_capsule.rs:311). `derive_autopsies_for_bankruptcy` carries `stored_capsule_bytes` in `BankruptcyAutopsyDerivation`, and `write_bankruptcy_autopsies_to_cas` writes those exact bytes to CAS at [autopsy_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/autopsy_capsule.rs:543) and [autopsy_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/autopsy_capsule.rs:618). Sequencer dispatch reads `d.capsule.capsule_id`; apply writes through the deterministic helper, so the Cids remain agreement-locked: [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1374), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:3097).

The new R3 tests are real contract tests: Markov asserts `cas.get(&cap.capsule_id)`, byte hash equals CID, restore works, and next-session context resolves at [markov_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/markov_capsule.rs:675). Bankruptcy autopsy asserts returned Cids match derivation, `cas.get(cid)` succeeds, retrieved bytes hash to that CID, and restore succeeds at [autopsy_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/autopsy_capsule.rs:1080). I could not execute them in this session because the filesystem is read-only; Cargo failed creating `/tmp/...` with `Read-only file system (os error 30)`.

R3 evidence matches the claim: `LATEST_MARKOV_CAPSULE.txt` is `f9e701...`, and `cas_index.jsonl` has the `MarkovEvidenceCapsule` object under the same CID byte array. See [LATEST_MARKOV_CAPSULE.txt](/home/zephryj/projects/turingosv4/handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt:1) and [cas_index.jsonl](/home/zephryj/projects/turingosv4/handover/evidence/tb_15_markov_capsule_2026-05-04/cas_index.jsonl:2).

OBS, not blocker: TB-11 `write_evidence_capsule` still has the same old prelim/final mismatch pattern: it derives `capsule_cid` from `prelim_bytes`, then stores `final_bytes` under a different CAS key at [evidence_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/evidence_capsule.rs:272). This should be fixed, but I would not VETO TB-15 R3 on it: no current reader calls `cas.get(&evidence_capsule.capsule_id)`, and the R3 TB-15 Markov/autopsy production paths are now clean. Treat as `OBS-TB-11-CAS-ID`, not R4.

Minor OBS: the `debug_assert_eq!` checks are debug-build assertions, not release runtime enforcement. The real structural guarantee is `CasStore::put` computing and returning `Cid::from_content(content)` at [store.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs:178).

Conviction: medium-high, limited only by read-only test execution.  
Recommendation: PROCEED to SHIP with OBS for TB-11 evidence-capsule cleanup and small test/doc hardening. I could not save an audit file because this workspace is read-only.
tokens used
128,824
## VERDICT: PASS

R3 closes the Codex R2 VETO for TB-15.

Q3 Markov closure holds: `write_markov_capsule` builds the capsule with `capsule_id = Cid::default()` and `sha256 = Hash::ZERO`, canonical-encodes those zeroed bytes, derives `cid = Cid::from_content(&stored_bytes)`, stores exactly `stored_bytes`, then populates the returned in-memory struct. See [markov_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/markov_capsule.rs:270), [markov_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/markov_capsule.rs:305), and [markov_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/markov_capsule.rs:322). Restore is symmetric at [markov_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/markov_capsule.rs:339).

TB15-CAS-ID closure also holds: `write_autopsy_capsule` uses the same zeroed-stored-bytes pattern at [autopsy_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/autopsy_capsule.rs:289) and [autopsy_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/autopsy_capsule.rs:311). `derive_autopsies_for_bankruptcy` carries `stored_capsule_bytes` in `BankruptcyAutopsyDerivation`, and `write_bankruptcy_autopsies_to_cas` writes those exact bytes to CAS at [autopsy_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/autopsy_capsule.rs:543) and [autopsy_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/autopsy_capsule.rs:618). Sequencer dispatch reads `d.capsule.capsule_id`; apply writes through the deterministic helper, so the Cids remain agreement-locked: [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1374), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:3097).

The new R3 tests are real contract tests: Markov asserts `cas.get(&cap.capsule_id)`, byte hash equals CID, restore works, and next-session context resolves at [markov_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/markov_capsule.rs:675). Bankruptcy autopsy asserts returned Cids match derivation, `cas.get(cid)` succeeds, retrieved bytes hash to that CID, and restore succeeds at [autopsy_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/autopsy_capsule.rs:1080). I could not execute them in this session because the filesystem is read-only; Cargo failed creating `/tmp/...` with `Read-only file system (os error 30)`.

R3 evidence matches the claim: `LATEST_MARKOV_CAPSULE.txt` is `f9e701...`, and `cas_index.jsonl` has the `MarkovEvidenceCapsule` object under the same CID byte array. See [LATEST_MARKOV_CAPSULE.txt](/home/zephryj/projects/turingosv4/handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt:1) and [cas_index.jsonl](/home/zephryj/projects/turingosv4/handover/evidence/tb_15_markov_capsule_2026-05-04/cas_index.jsonl:2).

OBS, not blocker: TB-11 `write_evidence_capsule` still has the same old prelim/final mismatch pattern: it derives `capsule_cid` from `prelim_bytes`, then stores `final_bytes` under a different CAS key at [evidence_capsule.rs](/home/zephryj/projects/turingosv4/src/runtime/evidence_capsule.rs:272). This should be fixed, but I would not VETO TB-15 R3 on it: no current reader calls `cas.get(&evidence_capsule.capsule_id)`, and the R3 TB-15 Markov/autopsy production paths are now clean. Treat as `OBS-TB-11-CAS-ID`, not R4.

Minor OBS: the `debug_assert_eq!` checks are debug-build assertions, not release runtime enforcement. The real structural guarantee is `CasStore::put` computing and returning `Cid::from_content(content)` at [store.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs:178).

Conviction: medium-high, limited only by read-only test execution.  
Recommendation: PROCEED to SHIP with OBS for TB-11 evidence-capsule cleanup and small test/doc hardening. I could not save an audit file because this workspace is read-only.
