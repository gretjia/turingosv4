OpenAI Codex v0.128.0 (research preview)
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: read-only
reasoning effort: xhigh
reasoning summaries: none
session id: 019df0af-1c5e-7e83-ad23-9b473bd3c717
--------
user
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
I’ll audit against the local tree and commit history, then try to write the requested report. The workspace is currently read-only, so if the save step is blocked I’ll still return the verdict and exact findings.
exec
/bin/bash -lc 'git status --short && git rev-parse --show-toplevel && git rev-parse --short HEAD' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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
 M rules/enforcement.log
?? experiments/minif2f_v4/h_vppu_history.json
?? handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md.raw
?? handover/audits/run_codex_tb_15_ship_audit.sh
?? handover/audits/run_gemini_tb_15_ship_audit.py
?? handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/cas/
?? handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/
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
?? handover/tests/
/home/zephryj/projects/turingosv4
2337381

exec
/bin/bash -lc "rg --files -g 'AGENTS.md' -g '"'!target'"'" in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
codex
The repo is at the expected TB-15 ship commit (`2337381`) with unrelated dirty files already present. I didn’t find an `AGENTS.md` inside the repo with `rg`, so I’m checking ancestor locations before reading audit targets.
exec
/bin/bash -lc 'for p in /AGENTS.md /home/AGENTS.md /home/zephryj/AGENTS.md /home/zephryj/projects/AGENTS.md /home/zephryj/projects/turingosv4/AGENTS.md; do [ -f "$p" ] && printf '"'%s\\n' \""'$p"; done' in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
exec
/bin/bash -lc 'git log --oneline --decorate -n 12' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
2337381 (HEAD -> main) TB-15 SHIPPED — Lamarckian Autopsy + Markov EvidenceCapsule (Class 2 self-audit; 8/8 SG; 6/6 halt-triggers; closes architect §6 spec FR-15.1..6 + CR-15.1..6 + SG-15.1..8)
31be856 TB-15 Atom 5 — MarkovEvidenceCapsule schema + generator binary
a17f6ac TB-15 Atom 4 — TypicalErrorBroadcast clustering (cluster_autopsies)
f06d548 TB-15 Atom 3 — AutopsyIndex on EconomicState + TaskBankruptcyTx wire-in
6594d3f TB-15 Atom 2 — AgentAutopsyCapsule schema + writer
0316a81 TB-15 Atom 1 — halt-trigger fixture (6 unimplemented stubs)
a14e01e TB-15 Atom 0 — charter ratified (Lamarckian Autopsy + Markov EvidenceCapsule)
f05ba53 TB-14 handover-update — LATEST.md HEAD + commit-list post-Atom-7-ship corrections
8b93fd9 TB-14 Atom 7 SHIPPED — PriceIndex v0 + Boltzmann Masking (single charter; dual audit converged R2 PASS; closes OBS_TB_12_LEGACY_CPMM_QUARANTINE)
1189cb2 TB-14 Atom 6 B′ step 7 R2 closure — Gemini R2 Q11 (sequencer_wired field)
07ce9b8 TB-14 Atom 6 B′ steps 5+6 — production-controlled canonical-masking smokes (architect ruling §5+§6)
9daba5a TB-14 Atom 6 B′ step 4 — CanonicalNodeGraph + compute_mask_set canonical-graph rewire (architect ruling §3+§4)

codex
I’m going through the exact ship-gate surfaces first: sequencer dispatch/apply hooks, capsule derivation, Markov generator, projections, and dashboard. After that I’ll inspect the regression tests and trust-root metadata for drift.
exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '1310,1415p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1310	            // Step 3: monetary invariants. No money moved.
  1311	            assert_no_post_init_mint(tx, q)
  1312	                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1313	            assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])
  1314	                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1315	            // Step 4: state_root advance.
  1316	            q_next.state_root_t = terminal_summary_accept_state_root(&q.state_root_t, tx);
  1317	            Ok((q_next, SignalBundle::terminal_summary(ts.run_id.clone(), ts.run_outcome)))
  1318	        }
  1319	        // ──────────────────────────────────────────────────────────────────
  1320	        // TB-11 Atom 2 (architect §6.2 ruling 2026-05-02) — TaskBankruptcy
  1321	        // dispatch arm. Marks a task as Bankrupt — chain-resident "death
  1322	        // certificate" that future TB-12 NodeMarket Short / NO settlement
  1323	        // can reference as resolution anchor.
  1324	        //
  1325	        // No money movement; CTF preserved trivially. (Refund of any
  1326	        // remaining escrow is a separate post-bankruptcy TaskExpireTx with
  1327	        // `reason: BankruptcyTriggered`.)
  1328	        //
  1329	        // Idempotency: rejects if task already Bankrupt or Finalized.
  1330	        // ──────────────────────────────────────────────────────────────────
  1331	        TypedTx::TaskBankruptcy(bk) => {
  1332	            // Step 0: parent-root match.
  1333	            if bk.parent_state_root != q.state_root_t {
  1334	                return Err(TransitionError::StaleParent);
  1335	            }
  1336	            // Step 1: task exists.
  1337	            let task_entry = match q.economic_state_t.task_markets_t.0.get(&bk.task_id) {
  1338	                Some(e) => e.clone(),
  1339	                None => return Err(TransitionError::TaskNotFound),
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
  1360	            // Step 3.5 — TB-15 Atom 3 (architect §6.2): emit deterministic
  1361	            // AgentAutopsyCapsule Cids into agent_autopsies_t for each
  1362	            // staker losing on the bankrupted task. PURE: no CAS write
  1363	            // here — apply_one's post-dispatch hook (Stage 5.5) writes
  1364	            // the bytes using the same `derive_autopsies_for_bankruptcy`
  1365	            // helper (replay-safe identical Cids). CR-15.1 + halt-trigger
  1366	            // #1: Cids are NOT projected to AgentVisibleProjection.
  1367	            let derived =
  1368	                crate::runtime::autopsy_capsule::derive_autopsies_for_bankruptcy(
  1369	                    &q.economic_state_t,
  1370	                    bk,
  1371	                    q.q_t.current_round,
  1372	                    bk.timestamp_logical,
  1373	                );
  1374	            if !derived.is_empty() {
  1375	                let event_id = crate::state::typed_tx::EventId(bk.task_id.clone());
  1376	                let entry = q_next
  1377	                    .economic_state_t
  1378	                    .agent_autopsies_t
  1379	                    .0
  1380	                    .entry(event_id)
  1381	                    .or_default();
  1382	                for (capsule, _private_bytes) in &derived {
  1383	                    entry.push(capsule.capsule_id);
  1384	                }
  1385	            }
  1386	            // Step 4: monetary invariants. No money moved.
  1387	            assert_no_post_init_mint(tx, q)
  1388	                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1389	            assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])
  1390	                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1391	            // Step 5: state_root advance.
  1392	            q_next.state_root_t = task_bankruptcy_accept_state_root(&q.state_root_t, tx);
  1393	            Ok((q_next, SignalBundle::empty()))
  1394	        }
  1395	        // ──────────────────────────────────────────────────────────────────
  1396	        // TB-5 Atom 5+6 — ChallengeResolve arm (charter v2 § 4.6 +
  1397	        // preflight § 7.2). Two paths:
  1398	        //   Released:        refund challenger bond + flip status to Released
  1399	        //                    (entry stays; bond field becomes 0 per directive § 7 Q6)
  1400	        //   UpheldDeferred:  marker-only flip to UpheldDeferred; bond preserved
  1401	        //                    for TB-6 RSP-3.2 slash routing (no money movement)
  1402	        // The 5-holding CTF invariant is preserved: Released's bond-refund is
  1403	        // a balanced transfer between holding 5 (challenge_cases.bond) and
  1404	        // holding 1 (balances_t); UpheldDeferred touches no holding term.
  1405	        // ──────────────────────────────────────────────────────────────────
  1406	        TypedTx::ChallengeResolve(resolve) => {
  1407	            // Step 1: parent-root match.
  1408	            if resolve.parent_state_root != q.state_root_t {
  1409	                return Err(TransitionError::StaleParent);
  1410	            }
  1411	            // Step 2: target ChallengeCase exists.
  1412	            let case = match q
  1413	                .economic_state_t
  1414	                .challenge_cases_t
  1415	                .0

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '3025,3105p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  3025	        if let Some(msg) = system_message_for_verification(&tx) {
  3026	            use crate::bottom_white::ledger::system_keypair::verify_system_signature;
  3027	            let sig = system_signature_of(&tx)
  3028	                .expect("system_message_for_verification implies system_signature present");
  3029	            // TerminalSummaryTx carries no epoch field (STATE § 1.5 8-field
  3030	            // schema is digest-only); fall back to the apply-time sequencer
  3031	            // epoch. Other system variants carry epoch on the wire.
  3032	            let tx_epoch = system_epoch_of(&tx).unwrap_or(self.epoch);
  3033	            if !verify_system_signature(sig, &msg, tx_epoch, &self.pinned_pubkeys) {
  3034	                let err = TransitionError::InvalidSystemSignatureLive;
  3035	                self.record_rejection(submit_id, &tx, &q_snapshot, &err)?;
  3036	                return Err(ApplyError::Transition(err));
  3037	            }
  3038	        }
  3039	
  3040	        // Stage 2: dispatch (pure). On reject, route to L4.E rejection-evidence
  3041	        // ledger and return early. K1: no logical_t consumed; Inv 7: no
  3042	        // state_root_t / ledger_root_t advance.
  3043	        let (q_next, _signals) = match dispatch_transition(
  3044	            &q_snapshot,
  3045	            &tx,
  3046	            &self.predicate_registry,
  3047	            &self.tool_registry,
  3048	        ) {
  3049	            Ok(ok) => ok,
  3050	            Err(transition_err) => {
  3051	                self.record_rejection(submit_id, &tx, &q_snapshot, &transition_err)?;
  3052	                // No logical_t advance, no state_root advance, no ledger_root
  3053	                // advance. Caller observes ApplyError::Transition.
  3054	                return Err(ApplyError::Transition(transition_err));
  3055	            }
  3056	        };
  3057	
  3058	        // v1.1 C-2: TENTATIVE logical_t (do NOT fetch_add yet).
  3059	        let logical_t = self.next_logical_t.load(Ordering::SeqCst) + 1;
  3060	
  3061	        // Stage 3: put payload to CAS. DIV-5 5-param put signature.
  3062	        let payload_bytes = canonical_encode(&tx)
  3063	            .map_err(|e| ApplyError::PayloadEncode(e.to_string()))?;
  3064	        let payload_cid = {
  3065	            let mut cas_w = self.cas.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
  3066	            cas_w.put(
  3067	                &payload_bytes,
  3068	                ObjectType::ProposalPayload,
  3069	                &format!("sequencer-epoch-{}", self.epoch.get()),
  3070	                logical_t,
  3071	                Some("TypedTx.v1".to_string()),
  3072	            )?
  3073	        };
  3074	
  3075	        // Stage 3.5 — TB-15 Atom 3 (architect §6.2): post-dispatch autopsy
  3076	        // CAS-write hook. For accepted TaskBankruptcyTx, derive the same
  3077	        // capsules the dispatch arm pushed Cids for + write their bytes
  3078	        // (capsule + private_detail) to CAS. Idempotent: identical bytes
  3079	        // → identical Cids → CAS dedupe. Replay-safe: re-running this
  3080	        // produces the same CAS state. Failure here is a hard error
  3081	        // (ApplyError) — autopsy bytes MUST be retrievable for SG-15.6
  3082	        // dashboard regenerability.
  3083	        if let TypedTx::TaskBankruptcy(bk) = &tx {
  3084	            let _ = crate::runtime::autopsy_capsule::write_bankruptcy_autopsies_to_cas(
  3085	                &self.cas,
  3086	                &q_snapshot.economic_state_t,
  3087	                bk,
  3088	                q_snapshot.q_t.current_round,
  3089	                bk.timestamp_logical,
  3090	                &format!("sequencer-epoch-{}", self.epoch.get()),
  3091	            )
  3092	            .map_err(|e| match e {
  3093	                crate::runtime::autopsy_capsule::AutopsyWriteError::Cas(c) => {
  3094	                    ApplyError::Cas(c)
  3095	                }
  3096	                crate::runtime::autopsy_capsule::AutopsyWriteError::Encode(s) => {
  3097	                    ApplyError::PayloadEncode(s)
  3098	                }
  3099	                crate::runtime::autopsy_capsule::AutopsyWriteError::InternalLockPoisoned => {
  3100	                    ApplyError::QStateLockPoisoned
  3101	                }
  3102	            })?;
  3103	        }
  3104	
  3105	        // Stage 5: build LedgerEntrySigningPayload (v1.1 — stage 4 fetch_add

exec
/bin/bash -lc "nl -ba src/runtime/autopsy_capsule.rs | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
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

exec
/bin/bash -lc "nl -ba src/runtime/autopsy_capsule.rs | sed -n '430,590p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   430	/// `TaskBankruptcyTx`; no CAS writes, no env access. Used by:
   431	/// - dispatch arm: capsule_id population into `agent_autopsies_t`
   432	/// - apply_one hook: CAS write of the same deterministic bytes
   433	///
   434	/// Replay determinism (Art.0.2): identical `(pre_econ, bk, round, t)` →
   435	/// identical `Vec<(AgentAutopsyCapsule, Vec<u8>)>` (same Cids, same bytes,
   436	/// same order).
   437	pub fn derive_autopsies_for_bankruptcy(
   438	    pre_econ: &EconomicState,
   439	    bk: &TaskBankruptcyTx,
   440	    created_at_round: u64,
   441	    created_at_logical_t: u64,
   442	) -> Vec<(AgentAutopsyCapsule, Vec<u8>)> {
   443	    let event_id = EventId(bk.task_id.clone());
   444	    let mut out = Vec::new();
   445	
   446	    for (stake_tx_id, stake) in pre_econ.stakes_t.0.iter() {
   447	        if stake.task_id != bk.task_id {
   448	            continue;
   449	        }
   450	        // Deterministic private_detail JSON.
   451	        let private_detail = format!(
   452	            "{{\"event_kind\":\"task_bankruptcy\",\"task_id\":\"{}\",\
   453	             \"stake_tx_id\":\"{}\",\"staker\":\"{}\",\
   454	             \"stake_amount_micro\":{}}}",
   455	            stake.task_id.0,
   456	            stake_tx_id.0,
   457	            stake.staker.0,
   458	            stake.amount.micro_units()
   459	        );
   460	        let private_bytes = private_detail.into_bytes();
   461	        let private_detail_cid = Cid::from_content(&private_bytes);
   462	
   463	        let public_summary = AgentAutopsyCapsule::format_public_summary(
   464	            &stake.staker,
   465	            &event_id,
   466	            stake.amount,
   467	            &LossReasonClass::Bankruptcy,
   468	        );
   469	
   470	        let mut capsule = AgentAutopsyCapsule {
   471	            capsule_id: Cid::default(),
   472	            agent_id: stake.staker.clone(),
   473	            event_id: event_id.clone(),
   474	            loss_amount: stake.amount,
   475	            loss_reason_class: LossReasonClass::Bankruptcy,
   476	            violated_risk_rule: None,
   477	            suggested_policy_patch: None,
   478	            evidence_cids: vec![Cid::from_content(stake_tx_id.0.as_bytes())],
   479	            public_summary,
   480	            private_detail_cid,
   481	            privacy_policy: CapsulePrivacyPolicy::AuditOnly,
   482	            sha256: Hash::ZERO,
   483	            created_at_logical_t,
   484	            created_at_round,
   485	        };
   486	        let prelim_bytes = canonical_encode(&capsule)
   487	            .expect("AgentAutopsyCapsule is canonical-encodable");
   488	        let cid = Cid::from_content(&prelim_bytes);
   489	        capsule.capsule_id = cid;
   490	        capsule.sha256 = Hash(cid.0);
   491	
   492	        out.push((capsule, private_bytes));
   493	    }
   494	    out
   495	}
   496	
   497	/// TRACE_MATRIX FC1-N33 (TB-15 Atom 3): apply_one post-dispatch hook —
   498	/// writes deterministic autopsy bytes to CAS for a successfully-accepted
   499	/// `TaskBankruptcyTx`. Re-derives the capsule list using
   500	/// `derive_autopsies_for_bankruptcy` (same inputs → same Cids as the
   501	/// dispatch arm already populated into `agent_autopsies_t`).
   502	///
   503	/// Idempotent: CAS `put` of identical bytes returns the existing Cid
   504	/// (replay-safe — re-running apply_one yields the same CAS state).
   505	pub fn write_bankruptcy_autopsies_to_cas(
   506	    cas: &std::sync::Arc<std::sync::RwLock<CasStore>>,
   507	    pre_econ: &EconomicState,
   508	    bk: &TaskBankruptcyTx,
   509	    created_at_round: u64,
   510	    created_at_logical_t: u64,
   511	    creator_str: &str,
   512	) -> Result<Vec<Cid>, AutopsyWriteError> {
   513	    let derived = derive_autopsies_for_bankruptcy(
   514	        pre_econ,
   515	        bk,
   516	        created_at_round,
   517	        created_at_logical_t,
   518	    );
   519	    let mut cids = Vec::with_capacity(derived.len());
   520	    let mut cas_w = cas
   521	        .write()
   522	        .map_err(|_| AutopsyWriteError::InternalLockPoisoned)?;
   523	    for (capsule, private_bytes) in derived {
   524	        // Write private_detail bytes (idempotent put — Cid matches what
   525	        // the dispatch arm derived).
   526	        let _ = cas_w.put(
   527	            &private_bytes,
   528	            ObjectType::AutopsyPrivateDetail,
   529	            creator_str,
   530	            created_at_logical_t,
   531	            Some("v1/autopsy_private_detail".into()),
   532	        )?;
   533	        // Write canonical capsule bytes.
   534	        let final_bytes = canonical_encode(&capsule)
   535	            .map_err(|e| AutopsyWriteError::Encode(format!("capsule final encode: {e:?}")))?;
   536	        let _ = cas_w.put(
   537	            &final_bytes,
   538	            ObjectType::AgentAutopsyCapsule,
   539	            creator_str,
   540	            created_at_logical_t,
   541	            Some("v1/agent_autopsy_capsule".into()),
   542	        )?;
   543	        cids.push(capsule.capsule_id);
   544	    }
   545	    Ok(cids)
   546	}
   547	
   548	#[cfg(test)]
   549	mod tests {
   550	    use super::*;
   551	    use crate::state::q_state::TaskId;
   552	
   553	    /// TB-15 U1: capsule default round-trips through canonical bytes.
   554	    #[test]
   555	    fn autopsy_capsule_default_round_trip() {
   556	        use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
   557	        let c = AgentAutopsyCapsule::default();
   558	        let bytes = canonical_encode(&c).expect("encode");
   559	        let back: AgentAutopsyCapsule = canonical_decode(&bytes).expect("decode");
   560	        assert_eq!(c, back);
   561	    }
   562	
   563	    /// TB-15 U2: format_public_summary embeds agent_id + amount + reason tag.
   564	    #[test]
   565	    fn format_public_summary_contains_agent_amount_reason() {
   566	        let s = AgentAutopsyCapsule::format_public_summary(
   567	            &AgentId("Agent_solver_3".into()),
   568	            &EventId(TaskId("task:lean:t1".into())),
   569	            MicroCoin::from_micro_units(1500),
   570	            &LossReasonClass::Bankruptcy,
   571	        );
   572	        assert!(s.contains("Agent_solver_3"));
   573	        assert!(s.contains("1500"));
   574	        assert!(s.contains("task:lean:t1"));
   575	        assert!(s.contains("Bankruptcy"));
   576	    }
   577	
   578	    /// TB-15 U3: privacy_policy default = AuditOnly (re-use TB-11
   579	    /// CR-15.1 surface).
   580	    #[test]
   581	    fn privacy_policy_default_is_audit_only() {
   582	        let c = AgentAutopsyCapsule::default();
   583	        assert_eq!(c.privacy_policy, CapsulePrivacyPolicy::AuditOnly);
   584	    }
   585	
   586	    /// TB-15 Atom 2 — Writer: writes private_detail + capsule to CAS;
   587	    /// returned capsule has populated capsule_id (Cid of canonical
   588	    /// bytes) and matching sha256.
   589	    #[test]
   590	    fn write_autopsy_capsule_to_cas_round_trip() {

exec
/bin/bash -lc "nl -ba src/runtime/autopsy_capsule.rs | sed -n '220,430p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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
   305	    let prelim_bytes = canonical_encode(&capsule)
   306	        .map_err(|e| AutopsyWriteError::Encode(format!("capsule prelim encode: {e:?}")))?;
   307	    let capsule_cid = Cid::from_content(&prelim_bytes);
   308	    capsule.capsule_id = capsule_cid;
   309	    capsule.sha256 = Hash(capsule_cid.0);
   310	
   311	    // Step 3: write the canonical-encoded capsule (with capsule_id +
   312	    // sha256 filled in) to CAS as the AgentAutopsyCapsule object.
   313	    let final_bytes = canonical_encode(&capsule)
   314	        .map_err(|e| AutopsyWriteError::Encode(format!("capsule final encode: {e:?}")))?;
   315	    let _ = cas_w.put(
   316	        &final_bytes,
   317	        ObjectType::AgentAutopsyCapsule,
   318	        creator_str,
   319	        created_at_logical_t,
   320	        Some("v1/agent_autopsy_capsule".into()),
   321	    )?;
   322	
   323	    Ok(capsule)
   324	}
   325	
   326	// ────────────────────────────────────────────────────────────────────────────
   327	// TB-15 Atom 4 — TypicalErrorBroadcast clustering (architect §3.2.3 + CR-15.2)
   328	// ────────────────────────────────────────────────────────────────────────────
   329	//
   330	// `cluster_autopsies` groups input autopsies by `loss_reason_class`, and
   331	// emits a `TypicalErrorSummary` for each class whose count meets or
   332	// exceeds the broadcast threshold (default N=3 per
   333	// DECISION_LAMARCKIAN §3.2.3 + spec test 3.2.3 verbatim).
   334	//
   335	// **CR-15.2 + halt-trigger #5**: the output struct embeds
   336	// `public_summary` strings + `capsule_id` Cids only — NEVER
   337	// `private_detail_cid` payload bytes. Halt-trigger #5 verifies this by
   338	// serializing the output and scanning for any input
   339	// `private_detail_cid` byte sequence.
   340	
   341	/// TRACE_MATRIX FC2-N30 (TB-15 Atom 4; architect §3.2.3 + CR-15.2):
   342	/// public broadcast summary for an N≥threshold cluster of same-class
   343	/// autopsies. Embeds `public_summary` text + capsule Cids only;
   344	/// `private_detail_cid` bytes are NEVER included (halt-trigger #5).
   345	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   346	pub struct TypicalErrorSummary {
   347	    /// The shared loss reason class for the cluster.
   348	    pub loss_reason_class: LossReasonClass,
   349	    /// Count of capsules in this cluster.
   350	    pub count: u32,
   351	    /// Public broadcast text — concatenation / first-N exemplars of
   352	    /// each capsule's `public_summary`. Joined with " ; " separator.
   353	    pub exemplar_public_summary: String,
   354	    /// Cids of the contributing capsules (audit can fetch them with
   355	    /// AuditOnly access). NEVER private_detail_cids.
   356	    pub exemplar_capsule_cids: Vec<Cid>,
   357	}
   358	
   359	/// TRACE_MATRIX FC2-N30 (TB-15 Atom 4; architect §3.2.3): cluster
   360	/// autopsies by `loss_reason_class`. Emit a `TypicalErrorSummary` for
   361	/// each class whose count is `>= threshold`. Default architect
   362	/// threshold = 3 (DECISION_LAMARCKIAN §3.2.3 + spec test 3.2.3).
   363	///
   364	/// **Pure** — no CAS access, no env, no clock. Order-stable: input
   365	/// order preserved within each class; classes themselves emerge in
   366	/// `LossReasonClass::tag()` lexicographic order (BTreeMap iteration)
   367	/// for replay-determinism.
   368	///
   369	/// **CR-15.2 + halt-trigger #5**: output never embeds
   370	/// `private_detail_cid` bytes — only `public_summary` strings +
   371	/// `capsule_id` Cids.
   372	pub fn cluster_autopsies(
   373	    autopsies: &[AgentAutopsyCapsule],
   374	    threshold: u8,
   375	) -> Vec<TypicalErrorSummary> {
   376	    use std::collections::BTreeMap;
   377	    // Group by loss_reason_class.tag() for deterministic iteration.
   378	    let mut groups: BTreeMap<String, Vec<&AgentAutopsyCapsule>> = BTreeMap::new();
   379	    for c in autopsies {
   380	        groups
   381	            .entry(c.loss_reason_class.tag().to_string())
   382	            .or_default()
   383	            .push(c);
   384	    }
   385	    let mut out = Vec::new();
   386	    let threshold_usize = threshold as usize;
   387	    for (_tag, members) in groups {
   388	        if members.len() < threshold_usize {
   389	            continue;
   390	        }
   391	        let exemplar_public_summary = members
   392	            .iter()
   393	            .map(|c| c.public_summary.as_str())
   394	            .collect::<Vec<_>>()
   395	            .join(" ; ");
   396	        let exemplar_capsule_cids: Vec<Cid> = members.iter().map(|c| c.capsule_id).collect();
   397	        out.push(TypicalErrorSummary {
   398	            // All members share the same class by construction.
   399	            loss_reason_class: members[0].loss_reason_class.clone(),
   400	            count: members.len() as u32,
   401	            exemplar_public_summary,
   402	            exemplar_capsule_cids,
   403	        });
   404	    }
   405	    out
   406	}
   407	
   408	// ────────────────────────────────────────────────────────────────────────────
   409	// TB-15 Atom 3 — `derive_autopsies_for_bankruptcy` (PURE deterministic helper)
   410	// ────────────────────────────────────────────────────────────────────────────
   411	//
   412	// Pure function consumed by both the dispatch arm (to populate
   413	// `EconomicState.agent_autopsies_t` with deterministic Cids) AND by
   414	// the apply_one post-dispatch hook (to write the same bytes to CAS so
   415	// they're retrievable). Replay-determinism: identical inputs → identical
   416	// `(Cid, AgentAutopsyCapsule, private_detail_bytes)` triples.
   417	
   418	use crate::state::q_state::EconomicState;
   419	use crate::state::typed_tx::TaskBankruptcyTx;
   420	
   421	/// TRACE_MATRIX FC1-N33 (TB-15 Atom 3; architect §6.2 + DECISION_LAMARCKIAN
   422	/// §1.1): pure-deterministic derivation of `AgentAutopsyCapsule`s for a
   423	/// `TaskBankruptcyTx`. Returns one capsule per agent with an active
   424	/// `StakeEntry` pointing at the bankrupted task — `loss_reason_class =
   425	/// Bankruptcy`; `loss_amount = stake.amount`; `evidence_cids = [Cid of
   426	/// stake_tx_id]`. BTreeMap iteration is sorted by `TxId` → output order
   427	/// is deterministic.
   428	///
   429	/// **Pure**: takes pre-bankruptcy `EconomicState` snapshot + the
   430	/// `TaskBankruptcyTx`; no CAS writes, no env access. Used by:

exec
/bin/bash -lc "nl -ba src/runtime/markov_capsule.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
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
    66	    /// L4 transition_ledger root at generation time.
    67	    pub l4_root: Hash,
    68	    /// L4.E rejection_evidence ledger root at generation time.
    69	    pub l4e_root: Hash,
    70	    /// CAS metadata root (digest of CAS object metadata) at generation
    71	    /// time. Strictly informational; CR-15.5 — capsule does not
    72	    /// duplicate CAS contents.
    73	    pub cas_root: Hash,
    74	
    75	    /// Typical-error rollup at generation time (TB-15 Atom 4 surface).
    76	    pub typical_errors: Vec<TypicalErrorSummary>,
    77	    /// Open observation register entries at generation time (relative
    78	    /// paths under `handover/alignment/OBS_*.md`).
    79	    pub unresolved_obs: Vec<ObsId>,
    80	
    81	    /// CAS Cid of a JSON blob describing the next session's default
    82	    /// boot context (`{constitution_hash, latest_markov_cid, boot_seq}`).
    83	    /// FR-15.4 + halt-trigger #6 entry point.
    84	    pub next_session_context_cid: Cid,
    85	
    86	    /// SHA-256 of this capsule's canonical bytes. Defense-in-depth
    87	    /// duplicate of `capsule_id`.
    88	    pub sha256: Hash,
    89	    /// Logical time at generation (sequencer or generator-supplied).
    90	    pub created_at_logical_t: u64,
    91	    /// Free-form TB tag — e.g. `"TB-15"`. Strictly informational.
    92	    pub tb_tag: String,
    93	}
    94	
    95	impl Default for MarkovEvidenceCapsule {
    96	    fn default() -> Self {
    97	        Self {
    98	            capsule_id: Cid::default(),
    99	            previous_capsule_cid: None,
   100	            constitution_hash: Hash::ZERO,
   101	            l4_root: Hash::ZERO,
   102	            l4e_root: Hash::ZERO,
   103	            cas_root: Hash::ZERO,
   104	            typical_errors: Vec::new(),
   105	            unresolved_obs: Vec::new(),
   106	            next_session_context_cid: Cid::default(),
   107	            sha256: Hash::ZERO,
   108	            created_at_logical_t: 0,
   109	            tb_tag: String::new(),
   110	        }
   111	    }
   112	}
   113	
   114	impl MarkovEvidenceCapsule {
   115	    /// TRACE_MATRIX TB-15 Atom 5 — convenience constructor used by
   116	    /// halt-trigger #2 to pin `constitution_hash` to a known value
   117	    /// (verifies SG-15.7 from a fixture without spinning up the
   118	    /// generator binary).
   119	    pub fn with_constitution_hash(hash_bytes: [u8; 32]) -> Self {
   120	        Self {
   121	            constitution_hash: Hash(hash_bytes),
   122	            ..Self::default()
   123	        }
   124	    }
   125	}
   126	
   127	// ────────────────────────────────────────────────────────────────────────────
   128	// TB-15 Atom 5 — Writer + default-deny gate
   129	// ────────────────────────────────────────────────────────────────────────────
   130	
   131	/// TRACE_MATRIX TB-15 Atom 5 — writer / generator error taxonomy.
   132	#[derive(Debug)]
   133	pub enum MarkovGenError {
   134	    /// Default-deny: caller asked for deep-history read without
   135	    /// `TURINGOS_MARKOV_OVERRIDE=1`. SG-15.4 + halt-trigger #6.
   136	    DeepHistoryReadDenied,
   137	    Cas(crate::bottom_white::cas::store::CasError),
   138	    Encode(String),
   139	    Io(std::io::Error),
   140	    InternalLockPoisoned,
   141	}
   142	
   143	impl std::fmt::Display for MarkovGenError {
   144	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   145	        match self {
   146	            Self::DeepHistoryReadDenied => write!(
   147	                f,
   148	                "deep-history read denied: set TURINGOS_MARKOV_OVERRIDE=1 to enable"
   149	            ),
   150	            Self::Cas(e) => write!(f, "cas: {e}"),
   151	            Self::Encode(s) => write!(f, "encode: {s}"),
   152	            Self::Io(e) => write!(f, "io: {e}"),
   153	            Self::InternalLockPoisoned => write!(f, "internal lock poisoned"),
   154	        }
   155	    }
   156	}
   157	impl std::error::Error for MarkovGenError {}
   158	
   159	impl From<crate::bottom_white::cas::store::CasError> for MarkovGenError {
   160	    fn from(e: crate::bottom_white::cas::store::CasError) -> Self {
   161	        Self::Cas(e)
   162	    }
   163	}
   164	impl From<std::io::Error> for MarkovGenError {
   165	    fn from(e: std::io::Error) -> Self {
   166	        Self::Io(e)
   167	    }
   168	}
   169	
   170	/// TRACE_MATRIX TB-15 Atom 5 (architect FR-15.5 + halt-trigger #6):
   171	/// default-deny gate for deep-history reads. Returns
   172	/// `Err(DeepHistoryReadDenied)` unless `override_set` is true.
   173	///
   174	/// In production, `override_set` is wired to env
   175	/// `TURINGOS_MARKOV_OVERRIDE=1` by the binary; this helper isolates the
   176	/// decision so it can be exercised by halt-trigger #6 without process-
   177	/// global env mutation (env mutation racy under cargo's parallel test
   178	/// runner per `feedback_env_var_test_lock`).
   179	pub fn try_deep_history_read_with_override_check(
   180	    override_set: bool,
   181	) -> Result<(), MarkovGenError> {
   182	    if override_set {
   183	        Ok(())
   184	    } else {
   185	        Err(MarkovGenError::DeepHistoryReadDenied)
   186	    }
   187	}
   188	
   189	/// TRACE_MATRIX TB-15 Atom 5 (architect FR-15.5): bool wrapper that
   190	/// reads `TURINGOS_MARKOV_OVERRIDE` from process env. Used by the
   191	/// generator binary; isolated here so the decision is auditable.
   192	pub fn override_set_from_env() -> bool {
   193	    std::env::var("TURINGOS_MARKOV_OVERRIDE")
   194	        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
   195	        .unwrap_or(false)
   196	}
   197	
   198	/// TRACE_MATRIX TB-15 Atom 5: write a `MarkovEvidenceCapsule` to CAS.
   199	/// Flow:
   200	///
   201	/// 1. Build the next-session-context JSON blob → write to CAS as
   202	///    `ObjectType::NextSessionContext`. Cid is `next_session_context_cid`.
   203	/// 2. Build the capsule struct with `capsule_id = Cid::default()` +
   204	///    `sha256 = Hash::ZERO`. Canonical-encode → sha256 → that's the
   205	///    eventual `capsule_id`.
   206	/// 3. Re-create the struct with `capsule_id` filled in + write to CAS
   207	///    as `ObjectType::MarkovEvidenceCapsule`.
   208	///
   209	/// **CR-15.5**: caller supplies `constitution_hash` / `l4_root` /
   210	/// `l4e_root` / `cas_root` / `typical_errors` / `unresolved_obs` —
   211	/// each derived from the chain + CAS at generation time. Writer does
   212	/// NOT mint new ground truth.
   213	#[allow(clippy::too_many_arguments)]
   214	pub fn write_markov_capsule(
   215	    cas: &std::sync::Arc<std::sync::RwLock<CasStore>>,
   216	    previous_capsule_cid: Option<Cid>,
   217	    constitution_hash: Hash,
   218	    l4_root: Hash,
   219	    l4e_root: Hash,
   220	    cas_root: Hash,
   221	    typical_errors: Vec<TypicalErrorSummary>,
   222	    unresolved_obs: Vec<ObsId>,
   223	    tb_tag: String,
   224	    creator_str: &str,
   225	    created_at_logical_t: u64,
   226	) -> Result<MarkovEvidenceCapsule, MarkovGenError> {
   227	    let mut cas_w = cas
   228	        .write()
   229	        .map_err(|_| MarkovGenError::InternalLockPoisoned)?;
   230	
   231	    // Step 1: build + write next_session_context JSON.
   232	    let next_session_json = serde_json::json!({
   233	        "schema_version": "v1/next_session_context",
   234	        "constitution_hash_hex": hex(&constitution_hash.0),
   235	        "previous_markov_cid_hex": previous_capsule_cid.map(|c| c.hex()),
   236	        "tb_tag": tb_tag,
   237	        "boot_seq": [
   238	            "1. read constitution.md (verify sha256 == constitution_hash)",
   239	            "2. read CAS<this_markov_capsule_cid>",
   240	            "3. read CAS<previous_markov_capsule_cid> (if present)",
   241	            "4. DEFAULT-DENY deeper history; set TURINGOS_MARKOV_OVERRIDE=1 to override (audit-only)"
   242	        ],
   243	    });
   244	    let next_session_bytes = serde_json::to_vec(&next_session_json)
   245	        .map_err(|e| MarkovGenError::Encode(format!("next_session_context: {e}")))?;
   246	    let next_session_context_cid = cas_w.put(
   247	        &next_session_bytes,
   248	        ObjectType::NextSessionContext,
   249	        creator_str,
   250	        created_at_logical_t,
   251	        Some("v1/next_session_context".into()),
   252	    )?;
   253	
   254	    // Step 2: build capsule with capsule_id = 0 + sha256 = 0.
   255	    let mut capsule = MarkovEvidenceCapsule {
   256	        capsule_id: Cid::default(),
   257	        previous_capsule_cid,
   258	        constitution_hash,
   259	        l4_root,
   260	        l4e_root,

exec
/bin/bash -lc "nl -ba src/runtime/markov_capsule.rs | sed -n '260,560p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   260	        l4e_root,
   261	        cas_root,
   262	        typical_errors,
   263	        unresolved_obs,
   264	        next_session_context_cid,
   265	        sha256: Hash::ZERO,
   266	        created_at_logical_t,
   267	        tb_tag,
   268	    };
   269	    let prelim_bytes = canonical_encode(&capsule)
   270	        .map_err(|e| MarkovGenError::Encode(format!("capsule prelim encode: {e:?}")))?;
   271	    let cid = Cid::from_content(&prelim_bytes);
   272	    capsule.capsule_id = cid;
   273	    capsule.sha256 = Hash(cid.0);
   274	
   275	    // Step 3: write canonical-encoded capsule bytes to CAS.
   276	    let final_bytes = canonical_encode(&capsule)
   277	        .map_err(|e| MarkovGenError::Encode(format!("capsule final encode: {e:?}")))?;
   278	    let _ = cas_w.put(
   279	        &final_bytes,
   280	        ObjectType::MarkovEvidenceCapsule,
   281	        creator_str,
   282	        created_at_logical_t,
   283	        Some("v1/markov_evidence_capsule".into()),
   284	    )?;
   285	
   286	    Ok(capsule)
   287	}
   288	
   289	/// TRACE_MATRIX TB-15 Atom 5: scan `<repo>/handover/alignment/OBS_*.md`
   290	/// for unresolved-observation file paths. Pure read; returns sorted
   291	/// `Vec<ObsId>` (BTreeSet ordering) for replay-determinism. CR-15.5 —
   292	/// capsule references existing files, never mints new ones.
   293	pub fn scan_unresolved_obs(alignment_dir: &std::path::Path) -> Result<Vec<ObsId>, MarkovGenError> {
   294	    use std::collections::BTreeSet;
   295	    let mut out: BTreeSet<String> = BTreeSet::new();
   296	    if !alignment_dir.is_dir() {
   297	        return Ok(Vec::new());
   298	    }
   299	    for entry in std::fs::read_dir(alignment_dir)? {
   300	        let entry = entry?;
   301	        let name = entry.file_name();
   302	        let name_str = name.to_string_lossy();
   303	        if name_str.starts_with("OBS_") && name_str.ends_with(".md") {
   304	            out.insert(format!("handover/alignment/{}", name_str));
   305	        }
   306	    }
   307	    Ok(out.into_iter().map(ObsId).collect())
   308	}
   309	
   310	/// TRACE_MATRIX TB-15 Atom 5: deterministic SHA-256 of constitution.md
   311	/// at the given path. Public so binaries + tests can compute it
   312	/// uniformly. CR-15.5 + SG-15.7.
   313	pub fn sha256_of_file(path: &std::path::Path) -> Result<Hash, MarkovGenError> {
   314	    use sha2::{Digest, Sha256};
   315	    let bytes = std::fs::read(path)?;
   316	    let mut h = Sha256::new();
   317	    h.update(&bytes);
   318	    let digest: [u8; 32] = h.finalize().into();
   319	    Ok(Hash(digest))
   320	}
   321	
   322	/// Hex helper for next-session JSON formatting (32-byte hashes).
   323	fn hex(bytes: &[u8; 32]) -> String {
   324	    let mut s = String::with_capacity(64);
   325	    for b in bytes {
   326	        s.push_str(&format!("{:02x}", b));
   327	    }
   328	    s
   329	}
   330	
   331	#[cfg(test)]
   332	mod tests {
   333	    use super::*;
   334	    use sha2::{Digest, Sha256};
   335	
   336	    /// TB-15 Atom 5 — capsule default round-trips through canonical bytes.
   337	    #[test]
   338	    fn markov_capsule_default_round_trip() {
   339	        use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
   340	        let c = MarkovEvidenceCapsule::default();
   341	        let bytes = canonical_encode(&c).expect("encode");
   342	        let back: MarkovEvidenceCapsule = canonical_decode(&bytes).expect("decode");
   343	        assert_eq!(c, back);
   344	    }
   345	
   346	    /// TB-15 Atom 5 — with_constitution_hash plumbs the hash through.
   347	    #[test]
   348	    fn with_constitution_hash_sets_field() {
   349	        let hash = [0xABu8; 32];
   350	        let c = MarkovEvidenceCapsule::with_constitution_hash(hash);
   351	        assert_eq!(c.constitution_hash.0, hash);
   352	    }
   353	
   354	    /// TB-15 Atom 5 — try_deep_history_read_with_override_check:
   355	    /// false → Err(DeepHistoryReadDenied); true → Ok(()).
   356	    #[test]
   357	    fn deep_history_default_deny_works() {
   358	        match try_deep_history_read_with_override_check(false) {
   359	            Err(MarkovGenError::DeepHistoryReadDenied) => {}
   360	            other => panic!("expected DeepHistoryReadDenied; got {other:?}"),
   361	        }
   362	        assert!(try_deep_history_read_with_override_check(true).is_ok());
   363	    }
   364	
   365	    /// TB-15 Atom 5 — write_markov_capsule writes 2 CAS objects
   366	    /// (next_session_context + capsule), and capsule_id is the
   367	    /// canonical sha256 (with field zeroed for prelim encode).
   368	    #[test]
   369	    fn write_markov_capsule_to_cas_round_trip() {
   370	        use std::sync::{Arc, RwLock};
   371	        use tempfile::TempDir;
   372	
   373	        let tmp = TempDir::new().expect("tempdir");
   374	        let cas = Arc::new(RwLock::new(
   375	            CasStore::open(tmp.path()).expect("cas"),
   376	        ));
   377	
   378	        let mut h = Sha256::new();
   379	        h.update(b"fake constitution body");
   380	        let constitution_hash: [u8; 32] = h.finalize().into();
   381	
   382	        let cap = write_markov_capsule(
   383	            &cas,
   384	            None, // genesis Markov
   385	            Hash(constitution_hash),
   386	            Hash([0x01u8; 32]),
   387	            Hash([0x02u8; 32]),
   388	            Hash([0x03u8; 32]),
   389	            vec![],
   390	            vec![ObsId("handover/alignment/OBS_X.md".into())],
   391	            "TB-15".into(),
   392	            "tb15-test-writer",
   393	            7,
   394	        )
   395	        .expect("writer succeeds");
   396	
   397	        assert_ne!(cap.capsule_id, Cid::default());
   398	        assert_eq!(cap.capsule_id.0, cap.sha256.0);
   399	        assert_ne!(cap.next_session_context_cid, Cid::default());
   400	        assert_eq!(cap.constitution_hash.0, constitution_hash);
   401	
   402	        let cas_r = cas.read().expect("cas read");
   403	        assert_eq!(
   404	            cas_r.len(),
   405	            2,
   406	            "writer puts 2 CAS objects: next_session_context + capsule"
   407	        );
   408	    }
   409	
   410	    /// TB-15 Atom 5 — write_markov_capsule deterministic (same inputs →
   411	    /// same capsule_id + same next_session_context_cid).
   412	    #[test]
   413	    fn write_markov_capsule_deterministic_capsule_id() {
   414	        use std::sync::{Arc, RwLock};
   415	        use tempfile::TempDir;
   416	
   417	        let mk = || {
   418	            let tmp = TempDir::new().unwrap();
   419	            let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).unwrap()));
   420	            write_markov_capsule(
   421	                &cas,
   422	                Some(Cid([0xCDu8; 32])),
   423	                Hash([0x10u8; 32]),
   424	                Hash([0x20u8; 32]),
   425	                Hash([0x30u8; 32]),
   426	                Hash([0x40u8; 32]),
   427	                vec![],
   428	                vec![],
   429	                "TB-15-det".into(),
   430	                "writer",
   431	                42,
   432	            )
   433	            .expect("writer")
   434	        };
   435	        let a = mk();
   436	        let b = mk();
   437	        assert_eq!(a.capsule_id, b.capsule_id);
   438	        assert_eq!(a.next_session_context_cid, b.next_session_context_cid);
   439	    }
   440	
   441	    /// TB-15 Atom 5 — Markov chain: each capsule references the prior
   442	    /// via `previous_capsule_cid`.
   443	    #[test]
   444	    fn markov_chain_links_via_previous_capsule_cid() {
   445	        use std::sync::{Arc, RwLock};
   446	        use tempfile::TempDir;
   447	
   448	        let tmp = TempDir::new().unwrap();
   449	        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).unwrap()));
   450	
   451	        let cap_1 = write_markov_capsule(
   452	            &cas,
   453	            None,
   454	            Hash([0x01u8; 32]),
   455	            Hash([0x10u8; 32]),
   456	            Hash([0x20u8; 32]),
   457	            Hash([0x30u8; 32]),
   458	            vec![],
   459	            vec![],
   460	            "TB-15-prev".into(),
   461	            "writer",
   462	            1,
   463	        )
   464	        .expect("cap 1");
   465	        let cap_2 = write_markov_capsule(
   466	            &cas,
   467	            Some(cap_1.capsule_id),
   468	            Hash([0x01u8; 32]),
   469	            Hash([0x11u8; 32]), // L4 advanced
   470	            Hash([0x20u8; 32]),
   471	            Hash([0x30u8; 32]),
   472	            vec![],
   473	            vec![],
   474	            "TB-15-next".into(),
   475	            "writer",
   476	            2,
   477	        )
   478	        .expect("cap 2");
   479	
   480	        assert_eq!(cap_2.previous_capsule_cid, Some(cap_1.capsule_id));
   481	        assert_ne!(cap_1.capsule_id, cap_2.capsule_id);
   482	    }
   483	
   484	    /// TB-15 Atom 5 — sha256_of_file matches manual sha256.
   485	    #[test]
   486	    fn sha256_of_file_matches_manual() {
   487	        use std::io::Write;
   488	        use tempfile::NamedTempFile;
   489	        let mut f = NamedTempFile::new().unwrap();
   490	        let body = b"test constitution body";
   491	        f.write_all(body).unwrap();
   492	        let path = f.path().to_path_buf();
   493	        let computed = sha256_of_file(&path).expect("sha256");
   494	        let mut h = Sha256::new();
   495	        h.update(body);
   496	        let manual: [u8; 32] = h.finalize().into();
   497	        assert_eq!(computed.0, manual);
   498	    }
   499	
   500	    /// TB-15 Atom 5 — scan_unresolved_obs picks up OBS_*.md files only,
   501	    /// in sorted order.
   502	    #[test]
   503	    fn scan_unresolved_obs_filters_and_sorts() {
   504	        use std::fs;
   505	        use tempfile::TempDir;
   506	        let tmp = TempDir::new().unwrap();
   507	        let p = tmp.path();
   508	        fs::write(p.join("OBS_zeta.md"), "z").unwrap();
   509	        fs::write(p.join("OBS_alpha.md"), "a").unwrap();
   510	        fs::write(p.join("not_an_obs.md"), "x").unwrap();
   511	        fs::write(p.join("OBS_beta.txt"), "b").unwrap(); // wrong extension
   512	
   513	        let obs = scan_unresolved_obs(p).expect("scan");
   514	        assert_eq!(obs.len(), 2);
   515	        // Sorted (BTreeSet semantics).
   516	        assert!(obs[0].0.ends_with("OBS_alpha.md"));
   517	        assert!(obs[1].0.ends_with("OBS_zeta.md"));
   518	    }
   519	}

exec
/bin/bash -lc "nl -ba src/bin/generate_markov_capsule.rs | sed -n '1,320p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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
    42	    override_set_from_env, scan_unresolved_obs, sha256_of_file, write_markov_capsule, MarkovGenError,
    43	    ObsId,
    44	};
    45	use turingosv4::state::q_state::Hash;
    46	
    47	struct Args {
    48	    tb_id: String,
    49	    out_dir: PathBuf,
    50	    constitution_path: PathBuf,
    51	    /// v0 placeholder — future TB will read L4 chain head from this path.
    52	    #[allow(dead_code)]
    53	    runtime_repo: Option<PathBuf>,
    54	    cas_dir: Option<PathBuf>,
    55	    prev_cid_hex: Option<String>,
    56	    alignment_dir: PathBuf,
    57	    no_cas: bool,
    58	}
    59	
    60	fn parse_args(argv: &[String]) -> Result<Args, String> {
    61	    let mut tb_id: Option<String> = None;
    62	    let mut out_dir: Option<PathBuf> = None;
    63	    let mut constitution_path: Option<PathBuf> = None;
    64	    let mut runtime_repo: Option<PathBuf> = None;
    65	    let mut cas_dir: Option<PathBuf> = None;
    66	    let mut prev_cid_hex: Option<String> = None;
    67	    let mut alignment_dir: Option<PathBuf> = None;
    68	    let mut no_cas = false;
    69	
    70	    let mut i = 0;
    71	    while i < argv.len() {
    72	        match argv[i].as_str() {
    73	            "--tb-id" => {
    74	                tb_id = argv.get(i + 1).cloned();
    75	                i += 2;
    76	            }
    77	            "--out-dir" => {
    78	                out_dir = argv.get(i + 1).map(PathBuf::from);
    79	                i += 2;
    80	            }
    81	            "--constitution-path" => {
    82	                constitution_path = argv.get(i + 1).map(PathBuf::from);
    83	                i += 2;
    84	            }
    85	            "--runtime-repo" => {
    86	                runtime_repo = argv.get(i + 1).map(PathBuf::from);
    87	                i += 2;
    88	            }
    89	            "--cas-dir" => {
    90	                cas_dir = argv.get(i + 1).map(PathBuf::from);
    91	                i += 2;
    92	            }
    93	            "--prev-cid-hex" => {
    94	                prev_cid_hex = argv.get(i + 1).cloned();
    95	                i += 2;
    96	            }
    97	            "--alignment-dir" => {
    98	                alignment_dir = argv.get(i + 1).map(PathBuf::from);
    99	                i += 2;
   100	            }
   101	            "--no-cas" => {
   102	                no_cas = true;
   103	                i += 1;
   104	            }
   105	            "--help" | "-h" => {
   106	                return Err("help".to_string());
   107	            }
   108	            other => return Err(format!("unrecognized arg: {other}")),
   109	        }
   110	    }
   111	
   112	    Ok(Args {
   113	        tb_id: tb_id.ok_or_else(|| "--tb-id <N> required".to_string())?,
   114	        out_dir: out_dir.ok_or_else(|| "--out-dir <path> required".to_string())?,
   115	        constitution_path: constitution_path
   116	            .ok_or_else(|| "--constitution-path <path> required".to_string())?,
   117	        runtime_repo,
   118	        cas_dir,
   119	        prev_cid_hex,
   120	        alignment_dir: alignment_dir
   121	            .unwrap_or_else(|| PathBuf::from("handover/alignment")),
   122	        no_cas,
   123	    })
   124	}
   125	
   126	fn print_help() {
   127	    eprintln!(
   128	        "TB-15 generate_markov_capsule — write a MarkovEvidenceCapsule to CAS \
   129	         + JSON pointer.\n\
   130	         \n\
   131	         usage: generate_markov_capsule \\\n\
   132	         \x20  --tb-id <N> \\\n\
   133	         \x20  --out-dir <path> \\\n\
   134	         \x20  --constitution-path <path> \\\n\
   135	         \x20  [--runtime-repo <path>] [--cas-dir <path>] \\\n\
   136	         \x20  [--prev-cid-hex <hex>] [--alignment-dir <path>] \\\n\
   137	         \x20  [--no-cas]\n\
   138	         \n\
   139	         env:\n\
   140	         \x20  TURINGOS_MARKOV_OVERRIDE=1   permit deep-history reads (default-deny)\n\
   141	         \n\
   142	         exit:\n\
   143	         \x20  0  capsule generated + persisted\n\
   144	         \x20  1  generation failed (write / missing constitution)\n\
   145	         \x20  2  invalid args"
   146	    );
   147	}
   148	
   149	fn parse_cid_hex(s: &str) -> Result<Cid, String> {
   150	    if s.len() != 64 {
   151	        return Err(format!("--prev-cid-hex must be 64 hex chars; got {}", s.len()));
   152	    }
   153	    let mut out = [0u8; 32];
   154	    for (i, byte) in out.iter_mut().enumerate() {
   155	        let chunk = &s[i * 2..i * 2 + 2];
   156	        *byte = u8::from_str_radix(chunk, 16)
   157	            .map_err(|e| format!("--prev-cid-hex byte {i}: {e}"))?;
   158	    }
   159	    Ok(Cid(out))
   160	}
   161	
   162	fn run() -> Result<i32, String> {
   163	    let argv: Vec<String> = std::env::args().collect();
   164	    let args = match parse_args(&argv[1..]) {
   165	        Ok(a) => a,
   166	        Err(m) if m == "help" => {
   167	            print_help();
   168	            return Ok(0);
   169	        }
   170	        Err(m) => {
   171	            eprintln!("generate_markov_capsule: {m}");
   172	            print_help();
   173	            return Ok(2);
   174	        }
   175	    };
   176	
   177	    let override_set = override_set_from_env();
   178	    if override_set {
   179	        eprintln!(
   180	            "generate_markov_capsule: TURINGOS_MARKOV_OVERRIDE=1 set — \
   181	             deep-history reads ENABLED (audit-only path)."
   182	        );
   183	    }
   184	
   185	    // Step 1: constitution.md SHA-256.
   186	    let constitution_hash = sha256_of_file(&args.constitution_path)
   187	        .map_err(|e| format!("read constitution.md: {e}"))?;
   188	    eprintln!(
   189	        "constitution_hash = {}",
   190	        hex32(&constitution_hash.0)
   191	    );
   192	
   193	    // Step 2: L4 / L4.E / CAS roots — for v0, accept zero placeholders
   194	    // when --runtime-repo/--cas-dir are absent (fresh-repo path) and
   195	    // populate from CAS metadata digest when present. Future TB will
   196	    // wire to the actual chain head readers; v0 ships the substrate.
   197	    let l4_root = Hash::ZERO;
   198	    let l4e_root = Hash::ZERO;
   199	
   200	    // Step 3: scan OBS files.
   201	    let unresolved_obs: Vec<ObsId> = scan_unresolved_obs(&args.alignment_dir)
   202	        .map_err(|e| format!("scan OBS: {e}"))?;
   203	    eprintln!("unresolved_obs.len = {}", unresolved_obs.len());
   204	
   205	    // Step 4: typical_errors — v0 accepts empty (no chain-resident
   206	    // capsules in dry-run) and TB-16+ wires to actual cluster_autopsies
   207	    // over CAS-resident AgentAutopsyCapsule objects.
   208	    let typical_errors: Vec<TypicalErrorSummary> = Vec::new();
   209	
   210	    // Step 5: previous capsule Cid.
   211	    let previous_capsule_cid: Option<Cid> = match &args.prev_cid_hex {
   212	        Some(s) => Some(parse_cid_hex(s)?),
   213	        None => None,
   214	    };
   215	
   216	    // Step 6: write capsule. Two modes:
   217	    //   (a) --no-cas: build the capsule struct directly + skip CAS put.
   218	    //       Used when no runtime CAS is available (fresh repo).
   219	    //   (b) default: open `--cas-dir` as a CasStore + put.
   220	    let cas_root = Hash::ZERO; // v0 placeholder; future wire-in via CAS metadata digest.
   221	    let capsule = if args.no_cas {
   222	        eprintln!("generate_markov_capsule: --no-cas mode — JSON pointer only");
   223	        // Compute capsule_id deterministically without CAS write.
   224	        use turingosv4::bottom_white::ledger::transition_ledger::canonical_encode;
   225	        use turingosv4::runtime::markov_capsule::MarkovEvidenceCapsule;
   226	        let next_session_json = serde_json::json!({
   227	            "schema_version": "v1/next_session_context",
   228	            "constitution_hash_hex": hex32(&constitution_hash.0),
   229	            "previous_markov_cid_hex": previous_capsule_cid.map(|c| c.hex()),
   230	            "tb_tag": format!("TB-{}", args.tb_id),
   231	            "boot_seq": [
   232	                "1. read constitution.md (verify sha256 == constitution_hash)",
   233	                "2. read CAS<this_markov_capsule_cid>",
   234	                "3. read CAS<previous_markov_capsule_cid> (if present)",
   235	                "4. DEFAULT-DENY deeper history; set TURINGOS_MARKOV_OVERRIDE=1 to override (audit-only)"
   236	            ],
   237	        });
   238	        let next_session_bytes = serde_json::to_vec(&next_session_json)
   239	            .map_err(|e| format!("next_session_context encode: {e}"))?;
   240	        let next_session_context_cid = Cid::from_content(&next_session_bytes);
   241	        let mut cap = MarkovEvidenceCapsule {
   242	            capsule_id: Cid::default(),
   243	            previous_capsule_cid,
   244	            constitution_hash,
   245	            l4_root,
   246	            l4e_root,
   247	            cas_root,
   248	            typical_errors,
   249	            unresolved_obs,
   250	            next_session_context_cid,
   251	            sha256: Hash::ZERO,
   252	            created_at_logical_t: 0,
   253	            tb_tag: format!("TB-{}", args.tb_id),
   254	        };
   255	        let prelim_bytes = canonical_encode(&cap)
   256	            .map_err(|e| format!("capsule prelim encode: {e:?}"))?;
   257	        let cid = Cid::from_content(&prelim_bytes);
   258	        cap.capsule_id = cid;
   259	        cap.sha256 = Hash(cid.0);
   260	        cap
   261	    } else {
   262	        let cas_dir = args
   263	            .cas_dir
   264	            .as_ref()
   265	            .ok_or_else(|| "--cas-dir required without --no-cas".to_string())?;
   266	        let cas = Arc::new(RwLock::new(
   267	            CasStore::open(cas_dir).map_err(|e| format!("open CAS: {e}"))?,
   268	        ));
   269	        write_markov_capsule(
   270	            &cas,
   271	            previous_capsule_cid,
   272	            constitution_hash,
   273	            l4_root,
   274	            l4e_root,
   275	            cas_root,
   276	            typical_errors,
   277	            unresolved_obs,
   278	            format!("TB-{}", args.tb_id),
   279	            "tb15-generator",
   280	            0,
   281	        )
   282	        .map_err(|e| match e {
   283	            MarkovGenError::DeepHistoryReadDenied => {
   284	                "deep-history read denied (set TURINGOS_MARKOV_OVERRIDE=1)".to_string()
   285	            }
   286	            other => format!("write_markov_capsule: {other}"),
   287	        })?
   288	    };
   289	
   290	    eprintln!("capsule_id = {}", capsule.capsule_id.hex());
   291	
   292	    // Step 7: emit JSON pointer file + LATEST_MARKOV_CAPSULE.txt.
   293	    std::fs::create_dir_all(&args.out_dir)
   294	        .map_err(|e| format!("create out_dir: {e}"))?;
   295	    let json_path = args
   296	        .out_dir
   297	        .join(format!("MARKOV_TB-{}_2026-05-03.json", args.tb_id));
   298	    let json_body = serde_json::to_string_pretty(&capsule)
   299	        .map_err(|e| format!("capsule json encode: {e}"))?;
   300	    std::fs::write(&json_path, &json_body).map_err(|e| format!("write json: {e}"))?;
   301	    let latest_path = args.out_dir.join("LATEST_MARKOV_CAPSULE.txt");
   302	    std::fs::write(&latest_path, capsule.capsule_id.hex())
   303	        .map_err(|e| format!("write latest pointer: {e}"))?;
   304	
   305	    eprintln!("wrote {}", json_path.display());
   306	    eprintln!("wrote {}", latest_path.display());
   307	    Ok(0)
   308	}
   309	
   310	fn hex32(b: &[u8; 32]) -> String {
   311	    let mut s = String::with_capacity(64);
   312	    for x in b {
   313	        s.push_str(&format!("{:02x}", x));
   314	    }
   315	    s
   316	}
   317	
   318	fn main() {
   319	    match run() {
   320	        Ok(code) => std::process::exit(code),

exec
/bin/bash -lc "nl -ba src/bin/generate_markov_capsule.rs | sed -n '318,345p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   318	fn main() {
   319	    match run() {
   320	        Ok(code) => std::process::exit(code),
   321	        Err(msg) => {
   322	            eprintln!("generate_markov_capsule: {msg}");
   323	            std::process::exit(1);
   324	        }
   325	    }
   326	}

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '115,150p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   115	
   116	/// TRACE_MATRIX § 1.1 — agent-visible projection of tape filtered by per-agent
   117	/// visibility policy (Inv 10 Goodhart shield; `top_white::predicates::visibility`).
   118	///
   119	/// `views`: per-agent filtered head pointer; full filtering machinery lands in CO P2.7.
   120	///
   121	/// TB-14 Atom 3 (FC2-N28; architect §5.5 + charter §3 Atom 3): `mask_set`
   122	/// is the global per-round set of parent-attempt-node `TxId`s suppressed
   123	/// in the agent read-view because a child node dominates them by
   124	/// `BoltzmannMaskPolicy.price_margin` (FR-14.5 / FR-14.6). **Read-view
   125	/// mask only**, never deletion (CR-14.3 + halt-trigger #3): the underlying
   126	/// `Tape.nodes()` iteration always yields masked parents. Computed by
   127	/// `compute_mask_set` in `src/state/price_index.rs`. `#[serde(default)]`
   128	/// for backward-compat with pre-TB-14 chain snapshots (deserialize as
   129	/// empty set).
   130	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   131	pub struct AgentVisibleProjection {
   132	    pub views: BTreeMap<AgentId, NodeId>,
   133	    #[serde(default)]
   134	    pub mask_set: BTreeSet<TxId>,
   135	}
   136	
   137	// ────────────────────────────────────────────────────────────────────────────
   138	// BudgetSnapshot — global compute / cost / wall-clock budget.
   139	// ────────────────────────────────────────────────────────────────────────────
   140	
   141	/// TRACE_MATRIX § 1.1 — global budget snapshot:
   142	/// cost ceiling (MicroCoin), wall clock remaining (ms), compute cap remaining.
   143	/// Exhaustion → halt_reason ∈ {WallClockCap, ComputeCapViolated, MaxTxExhausted}.
   144	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   145	pub struct BudgetSnapshot {
   146	    pub cost_ceiling_microcoin: MicroCoin,
   147	    pub wall_clock_remaining_ms: u64,
   148	    pub compute_cap_remaining: u64,
   149	}
   150	

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '700,780p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   700	///   Open → Released         (via accepted ChallengeResolveTx{Released})
   701	///   Open → UpheldDeferred   (via accepted ChallengeResolveTx{UpheldDeferred})
   702	///   Released / UpheldDeferred → terminal (AlreadyResolved gate at dispatch)
   703	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
   704	#[repr(u8)]
   705	pub enum ChallengeStatus {
   706	    Open = 0,
   707	    Released = 1,
   708	    UpheldDeferred = 2,
   709	}
   710	
   711	impl Default for ChallengeStatus {
   712	    fn default() -> Self {
   713	        Self::Open
   714	    }
   715	}
   716	
   717	impl Default for ChallengeCase {
   718	    fn default() -> Self {
   719	        Self {
   720	            challenger: AgentId::default(),
   721	            bond: MicroCoin::zero(),
   722	            opened_at_round: 0,
   723	            target_work_tx: TxId::default(),
   724	            status: ChallengeStatus::Open,
   725	        }
   726	    }
   727	}
   728	
   729	// TB-14 Atom 2 (2026-05-03): legacy `pub struct PriceIndex(BTreeMap<TxId,
   730	// MicroCoin>)` removed. The TB-14 derived view is `compute_price_index`
   731	// in `src/state/price_index.rs` (architect §5.1: "price is signal, not
   732	// truth"; charter §7 auto-resolution A: "no second source-of-truth").
   733	// `EconomicState.price_index_t` field also removed at architect §5.2.
   734	
   735	/// TRACE_MATRIX TB-15 Atom 3 (architect §6.2 + DECISION_LAMARCKIAN §1.1):
   736	/// per-event autopsy index. `BTreeMap<EventId, Vec<Cid>>` — one Cid per
   737	/// `AgentAutopsyCapsule` emitted on a loss event (TB-15 v0 sole trigger
   738	/// = TaskBankruptcyTx; per-staker capsule emission). **Cids only** —
   739	/// the capsule bytes live in CAS behind `ObjectType::AgentAutopsyCapsule`.
   740	///
   741	/// Sequencer-side index ONLY. NOT projected to `AgentVisibleProjection`
   742	/// (CR-15.1 + halt-trigger #1). Other agents cannot retrieve the bytes
   743	/// through their `tape_view_t` (SG-15.2 + halt-trigger #4).
   744	///
   745	/// BTreeMap iteration order is sorted-by-`EventId` → deterministic →
   746	/// replay-safe.
   747	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   748	pub struct AutopsyIndex(pub BTreeMap<crate::state::typed_tx::EventId, Vec<crate::bottom_white::cas::schema::Cid>>);
   749	
   750	// ────────────────────────────────────────────────────────────────────────────
   751	// QState — § 1.1 verbatim, 9 fields.
   752	// ────────────────────────────────────────────────────────────────────────────
   753	
   754	/// TRACE_MATRIX § 1.1 — system state Q_t. 9 fields per WP § 4 + economic § 2 amendment.
   755	///
   756	/// Reconstructibility: every field is derivable from L4 transition ledger replay
   757	/// (Art IV Boot 公理).
   758	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   759	pub struct QState {
   760	    /// Agent swarm sub-state (tape head per agent + per-agent reputation snapshots).
   761	    pub q_t: AgentSwarmState,
   762	    /// Current ChainTape head pointer = git commit SHA in Path B substrate.
   763	    pub head_t: NodeId,
   764	    /// Materialized state Merkle root (git tree root in Path B).
   765	    pub state_root_t: Hash,
   766	    /// Agent-visible projection of tape filtered by per-agent visibility policy.
   767	    pub tape_view_t: AgentVisibleProjection,
   768	    /// L4 Transition Ledger root (Merkle root of all accepted tx so far).
   769	    pub ledger_root_t: Hash,
   770	    /// L1 Predicate Registry root.
   771	    pub predicate_registry_root_t: Hash,
   772	    /// L2 Tool Registry root.
   773	    pub tool_registry_root_t: Hash,
   774	    /// Economic state (WP § 2 amendment, 9 sub-fields).
   775	    pub economic_state_t: EconomicState,
   776	    /// Global budget snapshot.
   777	    pub budget_state_t: BudgetSnapshot,
   778	}
   779	
   780	impl QState {

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '780,880p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   780	impl QState {
   781	    /// TRACE_MATRIX Art IV Boot — genesis Q_t. All zero / empty;
   782	    /// roots populated by `boot::verify_trust_root` and the `state_root_t` published
   783	    /// in `genesis_payload.toml [constitution_root]`.
   784	    pub fn genesis() -> Self {
   785	        QState::default()
   786	    }
   787	}
   788	
   789	// ────────────────────────────────────────────────────────────────────────────
   790	// Inline determinism tests (round-trip + insertion-order independence).
   791	// Conformance tests proper live in tests/{four_element_mapping, q_state_reconstruct,
   792	// economic_state_reconstruct, six_axioms_alignment}.rs per TRACE_MATRIX_v3.
   793	// ────────────────────────────────────────────────────────────────────────────
   794	
   795	#[cfg(test)]
   796	mod tests {
   797	    use super::*;
   798	
   799	    #[test]
   800	    fn genesis_q_state_is_total_and_default() {
   801	        let g = QState::genesis();
   802	        assert_eq!(g, QState::default());
   803	        assert_eq!(g.q_t.current_round, 0);
   804	        assert!(g.q_t.agents.is_empty());
   805	        assert_eq!(g.head_t, NodeId::default());
   806	        assert_eq!(g.state_root_t, Hash::ZERO);
   807	    }
   808	
   809	    #[test]
   810	    fn nine_field_count_via_serde_json() {
   811	        // Sanity that QState has exactly 9 top-level fields.
   812	        let s = serde_json::to_value(QState::genesis()).unwrap();
   813	        let obj = s.as_object().expect("object");
   814	        assert_eq!(
   815	            obj.len(),
   816	            9,
   817	            "QState must have exactly 9 fields per WP § 4; got {}",
   818	            obj.len()
   819	        );
   820	        for k in &[
   821	            "q_t",
   822	            "head_t",
   823	            "state_root_t",
   824	            "tape_view_t",
   825	            "ledger_root_t",
   826	            "predicate_registry_root_t",
   827	            "tool_registry_root_t",
   828	            "economic_state_t",
   829	            "budget_state_t",
   830	        ] {
   831	            assert!(obj.contains_key(*k), "QState missing field {}", k);
   832	        }
   833	    }
   834	
   835	    #[test]
   836	    fn economic_state_has_thirteen_sub_fields() {
   837	        // TB-11 (2026-05-02 architect ruling §6.2): 9 → 10 sub-fields with +runs_t.
   838	        // TB-12 (2026-05-03 architect ruling §3 + §8 Atom 1): 10 → 11 sub-fields
   839	        // with +node_positions_t (flat NodePositionsIndex; canonical exposure
   840	        // record state; NOT NodeMarketEntry which is TB-14 derived view).
   841	        // TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A §4.3):
   842	        // 11 → 13 sub-fields with +conditional_collateral_t (CR-13.4 Coin
   843	        // holding, included in 6-holding total_supply_micro) +
   844	        // conditional_share_balances_t (CR-13.3 claims, NOT counted in
   845	        // total_supply_micro).
   846	        // TB-14 Atom 2 (2026-05-03 architect ruling §5.1): 13 → 12 sub-fields
   847	        // with -price_index_t (legacy stub removed; TB-14 provides
   848	        // `compute_price_index` pure-fn derived view, not canonical state —
   849	        // "price is signal, not truth"; charter §7 auto-resolution A: no
   850	        // second source-of-truth).
   851	        // TB-15 Atom 3 (2026-05-03 architect ruling §6.2): 12 → 13 sub-fields
   852	        // with +agent_autopsies_t (`AutopsyIndex` = `BTreeMap<EventId, Vec<Cid>>`;
   853	        // sequencer-side per-event Cid index; capsule bytes live in CAS;
   854	        // NOT projected to AgentVisibleProjection per CR-15.1 + halt-trigger #1).
   855	        let e = EconomicState::default();
   856	        let s = serde_json::to_value(&e).unwrap();
   857	        let obj = s.as_object().unwrap();
   858	        assert_eq!(
   859	            obj.len(),
   860	            13,
   861	            "EconomicState must have 13 sub-fields post-TB-15 (was 12 post-TB-14; +agent_autopsies_t); got {}",
   862	            obj.len()
   863	        );
   864	        assert!(obj.contains_key("runs_t"), "TB-11 runs_t sub-field missing");
   865	        assert!(obj.contains_key("node_positions_t"), "TB-12 node_positions_t sub-field missing");
   866	        assert!(obj.contains_key("conditional_collateral_t"), "TB-13 conditional_collateral_t sub-field missing");
   867	        assert!(obj.contains_key("conditional_share_balances_t"), "TB-13 conditional_share_balances_t sub-field missing");
   868	        assert!(obj.contains_key("agent_autopsies_t"), "TB-15 agent_autopsies_t sub-field missing");
   869	        assert!(!obj.contains_key("price_index_t"), "TB-14 Atom 2: price_index_t MUST be removed");
   870	    }
   871	
   872	    /// TB-12 Atom 1 (architect §8 Atom 1): NodePositionsIndex empty default
   873	    /// serializes to empty BTreeMap; carries no balance information.
   874	    #[test]
   875	    fn node_positions_index_default_is_empty() {
   876	        let idx = NodePositionsIndex::default();
   877	        assert!(idx.0.is_empty());
   878	    }
   879	
   880	    /// TB-12 Atom 1: explicit invariant SG-12.8 — no `node_market_t` field

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '1,90p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! Typed transaction ABI surface — CO1.1.4-pre1.
     2	//!
     3	//! Spec authority:
     4	//! - `handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md` — this atom
     5	//! - `STATE_TRANSITION_SPEC_v1_2026-04-27.md` § 1 (typed schemas), § 2.5
     6	//!   (canonical serialization), § 3 (transition pseudocode used to derive
     7	//!   FinalizeRewardTx schema in spec § 4)
     8	//!
     9	//! Why this module exists: when CO1.7-impl A1 (Git2LedgerWriter) shipped, the
    10	//! downstream A2 (Sequencer + `dispatch_transition`) needed a `TypedTx` enum
    11	//! whose variants carry per-kind tx structs. Those structs and ~20 supporting
    12	//! types (identifiers, signatures, predicate-result types, status enums) were
    13	//! "frozen on paper" in STATE_TRANSITION_SPEC § 1 but had no Rust definition.
    14	//! CO1.1.4-pre1 lands them in isolation under its own dual-audit gate,
    15	//! per the project's per-atom audit principle (CLAUDE.md "Audit Standard").
    16	//!
    17	//! /// TRACE_MATRIX FC2-Submit + § 1 typed schemas: typed-tx ABI surface.
    18	
    19	use serde::{Deserialize, Serialize};
    20	use std::collections::{BTreeMap, BTreeSet};
    21	
    22	use sha2::{Digest, Sha256};
    23	
    24	use crate::bottom_white::cas::schema::Cid;
    25	use crate::bottom_white::ledger::system_keypair::{serde_bytes_64, SystemEpoch, SystemSignature};
    26	use crate::economy::money::{MicroCoin, StakeMicroCoin};
    27	use crate::state::q_state::{AgentId, Hash, TaskId, TxId};
    28	
    29	// ────────────────────────────────────────────────────────────────────────────
    30	// § 2 Identifier newtypes (all opaque strings to Q_t)
    31	// ────────────────────────────────────────────────────────────────────────────
    32	
    33	// `TaskId` previously lived here; moved to `state::q_state` in TB-3 (2026-04-30)
    34	// to eliminate the q_state↔typed_tx circular-dependency that would have arisen
    35	// when q_state needs `TaskId` as the `TaskMarketsIndex` key. See q_state.rs.
    36	
    37	/// TRACE_MATRIX § 1.5 — runtime run id (one run per `Sequencer` driver lifecycle).
    38	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    39	pub struct RunId(pub String);
    40	
    41	/// TRACE_MATRIX STATE § 3.4 + § 4 I-FINALIZE-BATCH-ORDER — typed claim id used
    42	/// in `FinalizeRewardTx.claim_id` and `ClaimsIndex` keying. Wraps `TxId`
    43	/// (the underlying claim is recorded against the work_tx's TxId in
    44	/// ClaimsIndex per current QState shape) but **prevents accidental mixing
    45	/// of claim references with arbitrary transaction references** at the type
    46	/// level (Codex round-1 Q-B CHALLENGE).
    47	///
    48	/// `#[serde(transparent)]` — wire-identical to TxId, so adoption is
    49	/// non-breaking for canonical encoding.
    50	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    51	#[serde(transparent)]
    52	pub struct ClaimId(pub TxId);
    53	
    54	impl ClaimId {
    55	    pub fn new(s: impl Into<String>) -> Self {
    56	        Self(TxId(s.into()))
    57	    }
    58	    pub fn as_tx_id(&self) -> &TxId {
    59	        &self.0
    60	    }
    61	}
    62	
    63	/// TRACE_MATRIX § 1.3 ReuseTx + L2 Tool Registry — opaque tool identifier.
    64	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    65	pub struct ToolId(pub String);
    66	
    67	/// TRACE_MATRIX § 1.2 PredicateResultsBundle + L1 Predicate Registry — opaque predicate id.
    68	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    69	pub struct PredicateId(pub String);
    70	
    71	/// TRACE_MATRIX § 1.2 WorkTx field 5 — read-set key (DAG attribution / replay).
    72	/// Kept as opaque string in v1; stricter typing (path / tape-coordinate) lands
    73	/// in CO P2.4.0 attribution-engine spike.
    74	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    75	pub struct ReadKey(pub String);
    76	
    77	/// TRACE_MATRIX § 1.2 WorkTx field 6 — write-set key (DAG attribution / replay).
    78	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    79	pub struct WriteKey(pub String);
    80	
    81	// ────────────────────────────────────────────────────────────────────────────
    82	// § 3 AgentSignature (Ed25519 [u8;64], type-distinct from SystemSignature)
    83	// ────────────────────────────────────────────────────────────────────────────
    84	
    85	/// TRACE_MATRIX § 1.2 WorkTx field 10 + I-SIG: agent-side detached Ed25519
    86	/// signature over the per-tx canonical_digest. Distinct type from
    87	/// `SystemSignature` to prevent accidental confusion at API boundaries
    88	/// (Codex sec-arg: agent-vs-system signature mixing is a real hazard).
    89	#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    90	pub struct AgentSignature(#[serde(with = "serde_bytes_64")] [u8; 64]);

exec
/bin/bash -lc 'rg -n "RiskRuleId|CapsulePrivacyPolicy|agent_autopsies_t|struct EconomicState" src/state/typed_tx.rs src/state/q_state.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/q_state.rs:170:pub struct EconomicState {
src/state/q_state.rs:257:    pub agent_autopsies_t: AutopsyIndex,
src/state/q_state.rs:852:        // with +agent_autopsies_t (`AutopsyIndex` = `BTreeMap<EventId, Vec<Cid>>`;
src/state/q_state.rs:861:            "EconomicState must have 13 sub-fields post-TB-15 (was 12 post-TB-14; +agent_autopsies_t); got {}",
src/state/q_state.rs:868:        assert!(obj.contains_key("agent_autopsies_t"), "TB-15 agent_autopsies_t sub-field missing");
src/state/typed_tx.rs:567:pub enum CapsulePrivacyPolicy {
src/state/typed_tx.rs:578:impl Default for CapsulePrivacyPolicy {
src/state/typed_tx.rs:1101:/// `AgentAutopsyCapsule.violated_risk_rule` as `Option<RiskRuleId>` —
src/state/typed_tx.rs:1107:pub struct RiskRuleId(pub String);

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '160,270p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   160	
   161	// ────────────────────────────────────────────────────────────────────────────
   162	// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
   163	// ────────────────────────────────────────────────────────────────────────────
   164	
   165	/// TRACE_MATRIX WP § 2 economic — 9-sub-field economic state. Each sub-index
   166	/// is a BTreeMap newtype; entry shapes (Escrow / Stake / Claim / TaskMarket /
   167	/// RoyaltyEdge / ChallengeCase) are minimal-but-typed here and fully fleshed
   168	/// in the owning atoms (CO P2.1-2.6).
   169	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   170	pub struct EconomicState {
   171	    pub balances_t: BalancesIndex,
   172	    pub escrows_t: EscrowsIndex,
   173	    pub stakes_t: StakesIndex,
   174	    pub claims_t: ClaimsIndex,
   175	    pub reputations_t: ReputationsIndex,
   176	    pub task_markets_t: TaskMarketsIndex,
   177	    pub royalty_graph_t: RoyaltyGraph,
   178	    pub challenge_cases_t: ChallengeCasesIndex,
   179	    // TB-14 Atom 2 (2026-05-03): `price_index_t: PriceIndex` removed.
   180	    // The TB-14 derived view is `compute_price_index(econ)` in
   181	    // `src/state/price_index.rs`; not canonical state per architect §5.1.
   182	    /// TB-11 (architect §6.2 ruling 2026-05-02): runs_t — `RunId` → run-summary
   183	    /// entry written by the TerminalSummaryTx dispatch arm. Anchors
   184	    /// architect's RunExhaustedTx semantics on chain-resident state. Each
   185	    /// failed evaluator run produces exactly one entry (idempotency on
   186	    /// run_id). `#[serde(default)]` for backward-compat with pre-TB-11
   187	    /// chain snapshots.
   188	    #[serde(default)]
   189	    pub runs_t: RunsIndex,
   190	    /// TRACE_MATRIX TB-12 (architect 2026-05-03 ruling §3 + §10): node_positions_t
   191	    /// — flat `BTreeMap<TxId, NodePosition>` index. **Canonical** TB-12 source
   192	    /// of truth for exposure records. **NOT a Coin holding** (CR-12.1 + CR-12.2);
   193	    /// NodePosition.amount is NOT counted in `monetary_invariant::total_supply_micro`.
   194	    ///
   195	    /// Architect §3 explicitly REJECTED the nested `node_market_t:
   196	    /// BTreeMap<NodeId, NodeMarketEntry>` shape — that's TB-14 derived view
   197	    /// (price + long_interest + short_interest aggregation), not canonical
   198	    /// state. Avoiding second source-of-truth (architect §3.2 reasoning;
   199	    /// TaskMarket.total_escrow precedent on cache=truth).
   200	    ///
   201	    /// Populated by accept-arm side-effect on accepted WorkTx (FirstLong) +
   202	    /// ChallengeTx (ChallengeShort) per architect §8 Atom 2. VerifyTx writes
   203	    /// nothing here per FR-12.3 + CR-12.8. `#[serde(default)]` for
   204	    /// backward-compat with pre-TB-12 chain snapshots.
   205	    #[serde(default)]
   206	    pub node_positions_t: NodePositionsIndex,
   207	    /// TRACE_MATRIX TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A
   208	    /// §4.3 + §4.4 FR-13.1..7 + CR-13.4): conditional collateral per event.
   209	    /// Locked Coin held against outstanding YES_E + NO_E share inventory.
   210	    ///
   211	    /// **IS** a Coin holding per CR-13.4 ("Locked collateral is Coin
   212	    /// holding"); included in the 6-holding `total_supply_micro` sum
   213	    /// (extends the TB-7R 5-holding sum). Mint/seed credit; redeem debit.
   214	    ///
   215	    /// **Complete-set balanced invariant** (Codex round-3 doc remediation
   216	    /// 2026-05-03): the live invariant enforced by
   217	    /// `monetary_invariant::assert_complete_set_balanced` is the
   218	    /// **MIN form**: `min(Σ_yes_shares, Σ_no_shares) == collateral`.
   219	    /// Pre-resolution (mint + seed only): both sides equal collateral
   220	    /// (MIN trivially equals collateral). Post-redeem: the winning side
   221	    /// equals collateral (debited 1:1 with collateral); the losing side
   222	    /// may exceed collateral as stranded zero-value claims. Strict
   223	    /// `Σ_yes == Σ_no == collateral` does NOT hold post-redemption.
   224	    ///
   225	    /// `#[serde(default)]` for backward-compat with pre-TB-13 chain snapshots.
   226	    #[serde(default)]
   227	    pub conditional_collateral_t: ConditionalCollateralIndex,
   228	    /// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + CR-13.3 + SG-13.2):
   229	    /// conditional share balances per `(owner, event_id, OutcomeSide)`.
   230	    ///
   231	    /// **IS NOT** a Coin holding — shares are CLAIMS against
   232	    /// `conditional_collateral_t[event_id]`; CR-13.3 + SG-13.2 explicit:
   233	    /// shares are NOT counted in `total_supply_micro`. Mint mints equal
   234	    /// YES + NO; seed mints equal YES + NO to provider; redeem debits the
   235	    /// winning side at 1 share = 1 MicroCoin against collateral.
   236	    ///
   237	    /// `#[serde(default)]` for backward-compat with pre-TB-13 chain snapshots.
   238	    #[serde(default)]
   239	    pub conditional_share_balances_t: ConditionalShareBalances,
   240	    /// TRACE_MATRIX TB-15 Atom 3 (architect §6.2 ruling 2026-05-02 + §6.5
   241	    /// SG-15.1 + SG-15.2): per-event autopsy index.
   242	    /// `BTreeMap<EventId, Vec<Cid>>` — for each event with at least one
   243	    /// loss-emission, accumulates the CAS Cids of `AgentAutopsyCapsule`
   244	    /// objects (one per losing agent). **Stores Cids only**, NEVER the
   245	    /// raw capsule bytes — the bytes live in CAS behind
   246	    /// `ObjectType::AgentAutopsyCapsule` (and the audit-only
   247	    /// `private_detail_cid` lives behind `ObjectType::AutopsyPrivateDetail`).
   248	    ///
   249	    /// **NOT projected to `AgentVisibleProjection`** (CR-15.1 + halt-
   250	    /// trigger #1). Sequencer-side index only; surfaces via
   251	    /// dashboard §15 (Atom 6) + ChainTape replay regeneration. Other
   252	    /// Agents cannot retrieve the bytes through their `tape_view_t`
   253	    /// (SG-15.2 + halt-trigger #4).
   254	    ///
   255	    /// `#[serde(default)]` for backward-compat with pre-TB-15 chain snapshots.
   256	    #[serde(default)]
   257	    pub agent_autopsies_t: AutopsyIndex,
   258	}
   259	
   260	/// TRACE_MATRIX WP § 2 — agent → balance ledger. Concrete entry: `MicroCoin` (CO1.0a).
   261	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   262	pub struct BalancesIndex(pub BTreeMap<AgentId, MicroCoin>);
   263	
   264	/// TRACE_MATRIX WP § 2 — tx → escrow entry. Full schema lands CO P2.2 EscrowVault.
   265	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   266	pub struct EscrowsIndex(pub BTreeMap<TxId, EscrowEntry>);
   267	
   268	/// TRACE_MATRIX WP § 2 — escrow entry shape (stub). Full fields land CO P2.2.
   269	/// `#[serde(default)]` on each field gives forward-compat: future atoms can add
   270	/// fields without breaking deserialization of historical ledger rows.

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '550,590p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   550	            Self::WallClockCap => RunOutcome::WallClockCap,
   551	            Self::ComputeCap => RunOutcome::ComputeCap,
   552	            Self::ProtocolCollapse | Self::SolverGiveUp => RunOutcome::ErrorHalt,
   553	        }
   554	    }
   555	}
   556	
   557	/// TRACE_MATRIX FC1-N1: TB-11 (architect §6.1 屏蔽规则) — privacy policy
   558	/// for a CAS-resident `EvidenceCapsule`. Default `AuditOnly` —
   559	/// public_summary may be surfaced to dashboard / read view, raw
   560	/// compressed evidence requires authorized audit-role access.
   561	/// Constitutional: 顶层白盒 quantize/broadcast/shield (Art. II.2.1) means
   562	/// raw failure logs cannot pollute future Agent context — only the
   563	/// public_summary surface broadcasts; capsule's compressed_log is shielded
   564	/// behind audit role.
   565	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
   566	#[repr(u8)]
   567	pub enum CapsulePrivacyPolicy {
   568	    /// Default — only `public_summary` field surfaces to non-audit views;
   569	    /// raw compressed log requires direct CAS read.
   570	    AuditOnly = 0,
   571	    /// public_summary may also enter Librarian message_board for next-iteration
   572	    /// agents (TB-15 Markov Loom prep).
   573	    PublicSummaryBroadcast = 1,
   574	    /// Full evidence visible to a designated audit-role (TB-17+ ChallengeCourt prep).
   575	    AuthorizedCAS = 2,
   576	}
   577	
   578	impl Default for CapsulePrivacyPolicy {
   579	    fn default() -> Self {
   580	        Self::AuditOnly
   581	    }
   582	}
   583	
   584	impl Default for RunOutcome {
   585	    fn default() -> Self {
   586	        Self::OmegaAccepted
   587	    }
   588	}
   589	
   590	// ────────────────────────────────────────────────────────────────────────────

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '1088,1114p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1088	    Yes = 0,
  1089	    No = 1,
  1090	}
  1091	
  1092	impl Default for OutcomeSide {
  1093	    fn default() -> Self {
  1094	        Self::Yes
  1095	    }
  1096	}
  1097	
  1098	/// TRACE_MATRIX TB-15 Atom 2 (architect §6.2): identifier for a
  1099	/// protocol-level risk rule (`max_position_size`, `max_drawdown`,
  1100	/// `max_slippage`, `max_leverage`, `kelly_cap`, ...). Carried by
  1101	/// `AgentAutopsyCapsule.violated_risk_rule` as `Option<RiskRuleId>` —
  1102	/// names the protocol invariant that triggered the loss event, when
  1103	/// applicable. Opaque newtype so the autopsy writer never depends on
  1104	/// the live risk-rule registry (CR-15.3 — autopsy may suggest, never
  1105	/// mutate).
  1106	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
  1107	pub struct RiskRuleId(pub String);
  1108	
  1109	/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): non-negative share count.
  1110	///
  1111	/// Architect spec uses `units: i128`; we tighten to `u128` because TB-13
  1112	/// shares can never be negative (mint creates positive, redeem decreases
  1113	/// positive, no debt model). Underflow at redeem time is a sequencer
  1114	/// `RedeemMoreThanOwned` rejection, not a representation concern.

exec
/bin/bash -lc "nl -ba src/bottom_white/cas/schema.rs | sed -n '1,170p'" in /home/zephryj/projects/turingosv4
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
   121	        if let Some(s) = &self.schema_id {
   122	            h.update(b"\x01");
   123	            h.update(s.as_bytes());
   124	        } else {
   125	            h.update(b"\x00");
   126	        }
   127	        h.update(self.size_bytes.to_be_bytes());
   128	        h.finalize().into()
   129	    }
   130	}
   131	
   132	#[cfg(test)]
   133	mod tests {
   134	    use super::*;
   135	
   136	    #[test]
   137	    fn cid_from_empty_content() {
   138	        let cid = Cid::from_content(b"");
   139	        // SHA-256 of empty input = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
   140	        assert_eq!(
   141	            cid.hex(),
   142	            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
   143	        );
   144	    }
   145	
   146	    #[test]
   147	    fn cid_deterministic() {
   148	        let cid_a = Cid::from_content(b"hello");
   149	        let cid_b = Cid::from_content(b"hello");
   150	        assert_eq!(cid_a, cid_b);
   151	    }
   152	
   153	    #[test]
   154	    fn cid_differs_on_content() {
   155	        let cid_a = Cid::from_content(b"hello");
   156	        let cid_b = Cid::from_content(b"world");
   157	        assert_ne!(cid_a, cid_b);
   158	    }
   159	
   160	    #[test]
   161	    fn cid_display_format() {
   162	        let cid = Cid::from_content(b"x");
   163	        let s = cid.to_string();
   164	        assert!(s.starts_with("cid:"));
   165	        assert_eq!(s.len(), 4 + 64);
   166	    }
   167	
   168	    #[test]
   169	    fn metadata_canonical_hash_deterministic() {
   170	        let m = CasObjectMetadata {

codex
A couple of early signals: the bankruptcy hook is pre-snapshot on both sides, and the Markov deep-history helper is present. I’m now checking whether the helper is actually on the normal read path, plus the dashboard and halt-trigger tests.
exec
/bin/bash -lc 'rg -n "render_section_15|read_latest_markov_pointer|autopsy_event_counts|AUTOPSY IS PRIVATE|tb14_render_tests|section_15" src/bin/audit_dashboard.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
143:    autopsy_event_counts: Vec<(String /*event_id*/, u32 /*cid_count*/)>,
961:        autopsy_event_counts: Vec::new(),
964:        latest_markov_capsule_cid_hex: read_latest_markov_pointer(),
972:fn read_latest_markov_pointer() -> Option<String> {
1515:    s.push_str(&render_section_15(
1516:        &r.autopsy_event_counts,
1528:/// literal phrase "AUTOPSY IS PRIVATE" (architect §6.4 + CR-15.1).
1538:fn render_section_15(
1539:    autopsy_event_counts: &[(String, u32)],
1546:    s.push_str("  AUTOPSY IS PRIVATE — public summary shown only when typical\n");
1554:    if autopsy_event_counts.is_empty() {
1571:        for (event_id, count) in autopsy_event_counts {
1582:            autopsy_event_counts.len()
1793:mod tb14_render_tests {
1908:    /// privacy banner `AUTOPSY IS PRIVATE` (architect §6.4 ruling
1912:        let s = render_section_15(&[], None);
1914:            s.contains("AUTOPSY IS PRIVATE"),
1916:             `AUTOPSY IS PRIVATE`. Got render:\n{s}"
1930:        let s = render_section_15(&counts, Some("abcd1234"));
1946:        let s = render_section_15(&[], None);
1964:        let s = render_section_15(&[], Some("deadbeef"));

exec
/bin/bash -lc "nl -ba src/bin/audit_dashboard.rs | sed -n '80,170p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    80	        i += 1;
    81	    }
    82	    Ok(Args {
    83	        repo: repo.ok_or("--repo required")?,
    84	        cas: cas.ok_or("--cas required")?,
    85	        json,
    86	        out,
    87	    })
    88	}
    89	
    90	#[derive(Debug, serde::Serialize)]
    91	struct DashboardReport {
    92	    run_id: String,
    93	    epoch: u64,
    94	    chain: ChainStats,
    95	    indicators: IndicatorStatus,
    96	    run_facts: ChainDerivedRunFacts,
    97	    per_agent: BTreeMap<String, AgentActivity>,
    98	    proposal_flow: Vec<ProposalFlowEntry>,
    99	    branch_lineage: Vec<BranchEdge>,
   100	    /// TB-7.7 D6: golden path steps (only populated when chain_oracle_verified=true).
   101	    golden_path: Vec<GoldenPathStep>,
   102	    cross_checks: CrossCheck,
   103	    /// TB-8 Atom 6: per-claim audit-row (Open / Finalized) with payout amount.
   104	    /// Populated by walking L4 entries and matching VerifyTx{Confirm} → claim
   105	    /// derivation against any subsequent FinalizeRewardTx with the same claim_id.
   106	    claims: Vec<ClaimAuditRow>,
   107	    /// TB-10 Atom 4: per-user-task audit-row. Populated by filtering TaskOpen
   108	    /// entries whose sponsor_agent.0 starts with "Agent_user_" (lean_market
   109	    /// CLI convention) and cross-referencing with claims for payout status.
   110	    /// The aggregate sum of bounty_micro across all rows is the user's total
   111	    /// committed liquidity at this snapshot.
   112	    user_tasks: Vec<UserTaskRow>,
   113	    /// TB-11 Atom 5 (architect §6.2): exhausted runs from TerminalSummaryTx
   114	    /// L4 entries (architect's RunExhaustedTx role).
   115	    exhausted_runs: Vec<ExhaustedRunRow>,
   116	    /// TB-11 Atom 5 (architect §6.2): expired tasks from TaskExpireTx L4
   117	    /// entries (capital release path).
   118	    expired_tasks: Vec<ExpiredTaskRow>,
   119	    /// TB-11 Atom 5 (architect §6.2): bankrupt tasks from TaskBankruptcyTx
   120	    /// L4 entries (death certificate for future TB-12 NodeMarket Short / NO
   121	    /// settlement anchor).
   122	    bankrupt_tasks: Vec<BankruptTaskRow>,
   123	    /// TB-12 Atom 4 (architect 2026-05-03 ruling §8 Atom 4): exposure
   124	    /// records derived from accepted WorkTx (FirstLong) + ChallengeTx
   125	    /// (ChallengeShort) L4 entries. Architect §10: IMMUTABLE EXPOSURE
   126	    /// RECORD, NOT active position balance. Label discipline: "Exposure
   127	    /// records", NOT "Open market balances".
   128	    exposures: Vec<ExposureRecordRow>,
   129	    /// TB-14 Atom 6 (architect 2026-05-03 ruling §5.1 + §5.5 SG-14.6):
   130	    /// derived price-index view per `compute_price_index` over a synthetic
   131	    /// `EconomicState` rebuilt from `exposures`. Renders in §14 with the
   132	    /// `PRICE IS SIGNAL, NOT TRUTH` banner per architect §5.1 ("Price is
   133	    /// signal, not truth") and SG-14.6 unit test discipline. NEVER shown
   134	    /// as decimal — every price is rendered as `numerator/denominator`
   135	    /// integer-rational pair (charter §5 forbidden list: no f64 / no
   136	    /// decimal float in TB-14 module surface).
   137	    price_index: BTreeMap<TxId, NodeMarketEntry>,
   138	    /// TB-15 Atom 6 (architect §6.5 SG-15.6): per-event autopsy Cid
   139	    /// counts derived from on-chain `EconomicState.agent_autopsies_t`
   140	    /// at snapshot time. Empty Vec when no TaskBankruptcyTx has fired.
   141	    /// Architect §6.4 privacy: dashboard surfaces COUNTS + COMPRESSED
   142	    /// `public_summary` strings only — never `private_detail_cid` bytes.
   143	    autopsy_event_counts: Vec<(String /*event_id*/, u32 /*cid_count*/)>,
   144	    /// TB-15 Atom 6: latest Markov capsule pointer (Cid hex from
   145	    /// `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt` if present;
   146	    /// None when no Markov capsule has been generated). FR-15.4 next-
   147	    /// session bootstrap surface.
   148	    latest_markov_capsule_cid_hex: Option<String>,
   149	}
   150	
   151	/// TB-12 Atom 4 (architect 2026-05-03 ruling §8 Atom 4) — per-NodePosition
   152	/// audit row for §13. Architect's label discipline: "Exposure records"
   153	/// (NOT "Open market balances" — TB-12 is exposure index, not trading
   154	/// market; live share balances land in TB-13 CompleteSet).
   155	#[derive(Debug, serde::Serialize)]
   156	struct ExposureRecordRow {
   157	    position_id: String,
   158	    node_id: String,
   159	    task_id: String,
   160	    owner: String,
   161	    /// "Long" or "Short".
   162	    side: String,
   163	    /// "FirstLong" or "ChallengeShort".
   164	    kind: String,
   165	    /// MicroCoin amount of the position. **NOT a Coin holding** per CR-12.1
   166	    /// + CR-12.2; explicitly excluded from total_supply_micro.
   167	    amount_micro: i64,
   168	    /// Backref to the source typed-tx that derived this position
   169	    /// (FirstLong: WorkTx.tx_id; ChallengeShort: ChallengeTx.tx_id).
   170	    source_tx: String,

exec
/bin/bash -lc "nl -ba src/bin/audit_dashboard.rs | sed -n '1480,1605p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1480	
  1481	    if !r.bankrupt_tasks.is_empty() {
  1482	        s.push('\n');
  1483	        s.push_str("  Bankrupt tasks (TaskBankruptcyTx; chain-resident death certificate):\n");
  1484	        s.push_str("    task_id            | reason                | failed_runs | evidence_capsule_cid (hex)\n");
  1485	        s.push_str("    -------------------+-----------------------+-------------+--------------------------------\n");
  1486	        for bk in &r.bankrupt_tasks {
  1487	            let cap_short = if bk.evidence_capsule_cid_hex.len() > 32 {
  1488	                format!("{}…", &bk.evidence_capsule_cid_hex[0..31])
  1489	            } else {
  1490	                bk.evidence_capsule_cid_hex.clone()
  1491	            };
  1492	            s.push_str(&format!(
  1493	                "    {:<18} | {:<21} | {:>11} | {}\n",
  1494	                trunc(&bk.task_id, 18),
  1495	                trunc(&bk.bankruptcy_reason, 21),
  1496	                bk.failed_run_count,
  1497	                cap_short,
  1498	            ));
  1499	        }
  1500	    }
  1501	
  1502	    s.push('\n');
  1503	    s.push_str("  Architect mandate (§6.2 ruling 2026-05-02) ✓:\n");
  1504	    s.push_str("    O(1) chain cost / O(N) auditability — failure evidence anchored on L4\n");
  1505	    s.push_str("    via system-emitted system_signature; raw log requires audit-role access\n");
  1506	    s.push_str("    (CapsulePrivacyPolicy::AuditOnly default; only public_summary surfaces here).\n");
  1507	
  1508	    // §13 TB-12 Node exposure records (architect 2026-05-03 ruling §3 + §10).
  1509	    s.push_str(&render_section_13(&r.exposures));
  1510	
  1511	    // §14 TB-14 PriceIndex (architect 2026-05-03 ruling §5.1 + §5.5 SG-14.6).
  1512	    s.push_str(&render_section_14(&r.price_index));
  1513	
  1514	    // §15 TB-15 Autopsy + Markov (architect 2026-05-02 §6.5 SG-15.6).
  1515	    s.push_str(&render_section_15(
  1516	        &r.autopsy_event_counts,
  1517	        r.latest_markov_capsule_cid_hex.as_deref(),
  1518	    ));
  1519	    s
  1520	}
  1521	
  1522	/// TRACE_MATRIX TB-15 Atom 6 (architect §6.5 SG-15.6 + §6.4 privacy):
  1523	/// §15 Autopsy + Markov render. Pure function over (event Cid counts +
  1524	/// optional Markov capsule pointer); extracted for SG-15.6
  1525	/// unit-testability.
  1526	///
  1527	/// **ARCHITECT-MANDATED PRIVACY BANNER**: the section opens with the
  1528	/// literal phrase "AUTOPSY IS PRIVATE" (architect §6.4 + CR-15.1).
  1529	/// Re-rendering this banner in every dashboard frame is the SG-15.6 ship
  1530	/// gate's enforcement surface. Dashboard surfaces COUNTS + Markov
  1531	/// pointer ONLY — never `private_detail_cid` payload bytes (CR-15.1 +
  1532	/// halt-trigger #1 + halt-trigger #4).
  1533	///
  1534	/// **NO RAW PRIVATE DETAIL**: the function signature accepts only
  1535	/// `Vec<(String, u32)>` event counts + an optional Markov pointer hex.
  1536	/// Raw private bytes are structurally absent from the input set, so the
  1537	/// rendered output cannot leak them.
  1538	fn render_section_15(
  1539	    autopsy_event_counts: &[(String, u32)],
  1540	    latest_markov_capsule_cid_hex: Option<&str>,
  1541	) -> String {
  1542	    let mut s = String::new();
  1543	    s.push('\n');
  1544	    s.push_str("§15 TB-15 Autopsy + Markov (architect 2026-05-02 §6.5 SG-15.6)\n");
  1545	    s.push_str("--------------------------------------------------------------\n");
  1546	    s.push_str("  AUTOPSY IS PRIVATE — public summary shown only when typical\n");
  1547	    s.push_str("  (≥3 cluster). Raw private details require audit-role access.\n");
  1548	    s.push_str("    Architect §6.4 ruling 2026-05-02: capsule audit detail is\n");
  1549	    s.push_str("    AuditOnly; NEVER enters AgentVisibleProjection (CR-15.1 +\n");
  1550	    s.push_str("    halt-trigger #1 + #4).\n");
  1551	    s.push_str("    Typical-error broadcast surface uses public_summary text\n");
  1552	    s.push_str("    only (CR-15.2 + halt-trigger #5).\n\n");
  1553	
  1554	    if autopsy_event_counts.is_empty() {
  1555	        s.push_str("  (no agent_autopsies_t entries in this snapshot — no\n");
  1556	        s.push_str("  TaskBankruptcyTx has fired during the chain window)\n");
  1557	        s.push_str("  Acceptable signal-state: a run with zero accepted\n");
  1558	        s.push_str("  TaskBankruptcyTx yields an empty AutopsyIndex by\n");
  1559	        s.push_str("  TB-15 Atom 3 charter scope (single trigger site).\n\n");
  1560	    } else {
  1561	        s.push_str("  Per-event Cid counts (capsule bytes live in CAS;\n");
  1562	        s.push_str("  audit-role required to fetch private_detail):\n\n");
  1563	        s.push_str(&format!(
  1564	            "    {:<48}  {:>10}\n",
  1565	            "event_id", "cid_count"
  1566	        ));
  1567	        s.push_str("    ");
  1568	        s.push_str(&"-".repeat(60));
  1569	        s.push('\n');
  1570	        let mut total_cids: u32 = 0;
  1571	        for (event_id, count) in autopsy_event_counts {
  1572	            total_cids += *count;
  1573	            s.push_str(&format!(
  1574	                "    {:<48}  {:>10}\n",
  1575	                trunc(event_id, 48),
  1576	                count,
  1577	            ));
  1578	        }
  1579	        s.push_str(&format!(
  1580	            "    ─── total: {} capsule Cid(s) across {} event(s) ───\n\n",
  1581	            total_cids,
  1582	            autopsy_event_counts.len()
  1583	        ));
  1584	    }
  1585	
  1586	    s.push_str("  Markov default (FR-15.4): next-session boot reads\n");
  1587	    s.push_str("  constitution.md + latest Markov capsule. deeper history\n");
  1588	    s.push_str("  requires TURINGOS_MARKOV_OVERRIDE=1 (CR-15.6 +\n");
  1589	    s.push_str("  halt-trigger #6 — default-deny gate).\n\n");
  1590	
  1591	    match latest_markov_capsule_cid_hex {
  1592	        Some(cid_hex) if !cid_hex.is_empty() => {
  1593	            s.push_str(&format!(
  1594	                "  Latest Markov capsule pointer (handover/markov_capsules/\n  \
  1595	                LATEST_MARKOV_CAPSULE.txt):\n    {}\n",
  1596	                cid_hex
  1597	            ));
  1598	        }
  1599	        _ => {
  1600	            s.push_str("  (no latest Markov capsule pointer — run\n");
  1601	            s.push_str("  `cargo run --bin generate_markov_capsule -- --tb-id N\n");
  1602	            s.push_str("  --out-dir handover/markov_capsules/ --constitution-path\n");
  1603	            s.push_str("  constitution.md --no-cas` to emit one)\n");
  1604	        }
  1605	    }

exec
/bin/bash -lc "nl -ba tests/tb_15_halt_triggers.rs | sed -n '1,280p'" in /home/zephryj/projects/turingosv4
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
    88	    use turingosv4::runtime::markov_capsule::MarkovEvidenceCapsule;
    89	
    90	    let manifest = env!("CARGO_MANIFEST_DIR");
    91	    let constitution_path = format!("{}/constitution.md", manifest);
    92	    let constitution_bytes = std::fs::read(&constitution_path)
    93	        .unwrap_or_else(|e| panic!("read constitution.md: {}", e));
    94	    let mut h = Sha256::new();
    95	    h.update(&constitution_bytes);
    96	    let expected_hash: [u8; 32] = h.finalize().into();
    97	
    98	    let capsule = MarkovEvidenceCapsule::with_constitution_hash(expected_hash);
    99	    assert_eq!(
   100	        capsule.constitution_hash.0, expected_hash,
   101	        "halt-trigger #2: MarkovEvidenceCapsule.constitution_hash must equal \
   102	         sha256 of constitution.md bytes (SG-15.7)"
   103	    );
   104	}
   105	
   106	// ────────────────────────────────────────────────────────────────────
   107	// Halt-trigger #3
   108	// autopsy_does_not_mutate_predicates
   109	//
   110	// write_autopsy_capsule signature MUST NOT accept any &mut PredicateRegistry
   111	// or any other mutator on the predicate / tool / risk-policy registries.
   112	// Source-level fence: scan src/runtime/autopsy_capsule.rs for forbidden
   113	// signature tokens. CR-15.3 + SG-15.8.
   114	//
   115	// Filled in by Atom 2.
   116	// ────────────────────────────────────────────────────────────────────
   117	#[test]
   118	fn autopsy_does_not_mutate_predicates() {
   119	    let manifest = env!("CARGO_MANIFEST_DIR");
   120	    let path = format!("{}/src/runtime/autopsy_capsule.rs", manifest);
   121	    let body = std::fs::read_to_string(&path)
   122	        .unwrap_or_else(|e| panic!("read {}: {}", path, e));
   123	
   124	    // The autopsy module MUST NOT contain any mutator surface against
   125	    // the predicate / tool / risk-policy registries. Constructed at
   126	    // runtime to avoid this test's own source containing the forbidden
   127	    // substrings (and triggering self-trip on the file scan).
   128	    let forbidden: Vec<String> = vec![
   129	        format!("&mut Predicate{}", "Registry"),
   130	        format!("&mut Tool{}", "Registry"),
   131	        format!("&mut Risk{}", "PolicyRegistry"),
   132	        format!("&mut PredicateRunner"),
   133	        format!(".register_predicate("),
   134	        format!(".unregister_predicate("),
   135	        format!(".patch_predicate("),
   136	        format!(".register_tool("),
   137	        format!(".unregister_tool("),
   138	    ];
   139	    for tok in &forbidden {
   140	        assert!(
   141	            !body.contains(tok.as_str()),
   142	            "halt-trigger #3: autopsy_capsule.rs MUST NOT contain `{}` — \
   143	             autopsy carries `suggested_policy_patch: Option<Cid>` only as a \
   144	             SUGGESTION pointer; never auto-applied (CR-15.3 + SG-15.8)",
   145	            tok
   146	        );
   147	    }
   148	}
   149	
   150	// ────────────────────────────────────────────────────────────────────
   151	// Halt-trigger #4
   152	// private_detail_not_in_other_agent_view
   153	//
   154	// Agent B's projection must not contain Agent A's autopsy bytes.
   155	// AutopsyIndex stores Cids only; the CAS bytes behind private_detail_cid
   156	// require AuditOnly access. SG-15.2.
   157	//
   158	// Filled in by Atom 3 (after EconomicState gains agent_autopsies_t).
   159	// ────────────────────────────────────────────────────────────────────
   160	#[test]
   161	fn private_detail_not_in_other_agent_view() {
   162	    // Structural fence: AutopsyIndex value type must remain Vec<Cid>
   163	    // (32-byte content addresses), NOT Vec<AgentAutopsyCapsule> (the
   164	    // bytes themselves) and NOT any structure containing
   165	    // private_detail_cid payload bytes. Even if AgentVisibleProjection
   166	    // were ever to surface AutopsyIndex contents (which it does not —
   167	    // see halt-trigger #1), it would surface only public CAS Cids of
   168	    // public CAS evidence.
   169	    let manifest = env!("CARGO_MANIFEST_DIR");
   170	    let q_state_path = format!("{}/src/state/q_state.rs", manifest);
   171	    let body = std::fs::read_to_string(&q_state_path)
   172	        .unwrap_or_else(|e| panic!("read {}: {}", q_state_path, e));
   173	
   174	    // Locate the AutopsyIndex newtype definition.
   175	    let needle = "pub struct Autopsy".to_string() + "Index";
   176	    let start = body
   177	        .find(&needle)
   178	        .expect("AutopsyIndex newtype must exist in q_state.rs");
   179	    let after = &body[start..];
   180	    // Walk forward until the line ending with `;` (newtype is single-line).
   181	    let line_end = after
   182	        .find(";\n")
   183	        .or_else(|| after.find(";\r"))
   184	        .or_else(|| after.find(';'))
   185	        .expect("AutopsyIndex newtype must terminate with semicolon");
   186	    let decl = &after[..=line_end];
   187	
   188	    // The value type MUST be Vec<Cid>. Forbidden alternatives that
   189	    // would leak raw bytes:
   190	    let forbidden_value_shapes: Vec<String> = vec![
   191	        format!("Vec<Agent{}>", "AutopsyCapsule"),
   192	        format!("Vec<u{}>", "8"),
   193	        format!("Vec<Auto{}>", "psyPrivateDetail"),
   194	    ];
   195	    for tok in &forbidden_value_shapes {
   196	        assert!(
   197	            !decl.contains(tok.as_str()),
   198	            "halt-trigger #4: AutopsyIndex value type MUST be Vec<Cid>, \
   199	             NOT `{}` — agent_autopsies_t stores Cids only; raw bytes \
   200	             stay in CAS behind AuditOnly access (SG-15.2)",
   201	            tok
   202	        );
   203	    }
   204	    // Positive assertion: the declaration includes Vec<...Cid>.
   205	    assert!(
   206	        decl.contains("Vec<crate::bottom_white::cas::schema::Cid>")
   207	            || decl.contains("Vec<Cid>"),
   208	        "halt-trigger #4: AutopsyIndex value type must explicitly be Vec<Cid>; \
   209	         got declaration: {}",
   210	        decl
   211	    );
   212	}
   213	
   214	// ────────────────────────────────────────────────────────────────────
   215	// Halt-trigger #5
   216	// typical_error_clustering_uses_summary_only
   217	//
   218	// cluster_autopsies output (Vec<TypicalErrorSummary>) must embed
   219	// public_summary text + capsule_id Cids only. It must NEVER embed
   220	// private_detail_cid bytes. SG-15.5.
   221	//
   222	// Filled in by Atom 4 (cluster_autopsies + TypicalErrorSummary).
   223	// ────────────────────────────────────────────────────────────────────
   224	#[test]
   225	fn typical_error_clustering_uses_summary_only() {
   226	    use turingosv4::bottom_white::cas::schema::Cid;
   227	    use turingosv4::economy::money::MicroCoin;
   228	    use turingosv4::runtime::autopsy_capsule::{
   229	        cluster_autopsies, AgentAutopsyCapsule, LossReasonClass,
   230	    };
   231	    use turingosv4::state::q_state::{AgentId, Hash, TaskId};
   232	    use turingosv4::state::typed_tx::{CapsulePrivacyPolicy, EventId};
   233	
   234	    // Build 3 autopsies of the same loss_reason_class with
   235	    // distinguishable private_detail_cid bytes.
   236	    let event = EventId(TaskId("task:tb15:halt5".into()));
   237	    let mk = |agent: &str, priv_byte: u8| AgentAutopsyCapsule {
   238	        capsule_id: Cid::from_content(agent.as_bytes()),
   239	        agent_id: AgentId(agent.to_string()),
   240	        event_id: event.clone(),
   241	        loss_amount: MicroCoin::from_micro_units(1_000),
   242	        loss_reason_class: LossReasonClass::Bankruptcy,
   243	        violated_risk_rule: None,
   244	        suggested_policy_patch: None,
   245	        evidence_cids: vec![],
   246	        public_summary: format!(
   247	            "agent={} lost 1000μC on event={} reason=Bankruptcy",
   248	            agent, (event.0).0
   249	        ),
   250	        private_detail_cid: Cid([priv_byte; 32]),
   251	        privacy_policy: CapsulePrivacyPolicy::AuditOnly,
   252	        sha256: Hash::ZERO,
   253	        created_at_logical_t: 1,
   254	        created_at_round: 0,
   255	    };
   256	    let priv_bytes: [u8; 3] = [0xAA, 0xBB, 0xCC];
   257	    let autopsies = vec![
   258	        mk("A", priv_bytes[0]),
   259	        mk("B", priv_bytes[1]),
   260	        mk("C", priv_bytes[2]),
   261	    ];
   262	
   263	    let summaries = cluster_autopsies(&autopsies, 3);
   264	    assert_eq!(summaries.len(), 1, "3 same-class autopsies → 1 typical error");
   265	    assert_eq!(summaries[0].count, 3);
   266	
   267	    // Halt-trigger #5: serialization must not contain any
   268	    // private_detail_cid byte run.
   269	    let bytes = serde_json::to_vec(&summaries).expect("serialize summaries");
   270	    for &priv_byte in &priv_bytes {
   271	        let private_cid = [priv_byte; 32];
   272	        for window in bytes.windows(32) {
   273	            assert!(
   274	                window != private_cid,
   275	                "halt-trigger #5: TypicalErrorSummary serialization contains \
   276	                 private_detail_cid byte run for byte=0x{:02x} — broadcast \
   277	                 surface MUST use public_summary text only (SG-15.5)",
   278	                priv_byte
   279	            );
   280	        }

exec
/bin/bash -lc "nl -ba tests/tb_15_halt_triggers.rs | sed -n '280,380p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   280	        }
   281	    }
   282	}
   283	
   284	// ────────────────────────────────────────────────────────────────────
   285	// Halt-trigger #6
   286	// deep_history_read_without_override_fails
   287	//
   288	// generate_markov_capsule binary defaults to constitution +
   289	// latest-Markov-capsule context source. Reading deeper history (older
   290	// capsules; L4 chain rows pre-dating prior Markov capsule's l4_root)
   291	// requires TURINGOS_MARKOV_OVERRIDE=1; default-deny path returns
   292	// `MarkovGenError::DeepHistoryReadDenied`. SG-15.4 + FR-15.5.
   293	//
   294	// Filled in by Atom 5.
   295	// ────────────────────────────────────────────────────────────────────
   296	#[test]
   297	fn deep_history_read_without_override_fails() {
   298	    use turingosv4::runtime::markov_capsule::{
   299	        try_deep_history_read_with_override_check, MarkovGenError,
   300	    };
   301	
   302	    // Default-deny path: no override; result must be DeepHistoryReadDenied.
   303	    let result = try_deep_history_read_with_override_check(false);
   304	    match result {
   305	        Err(MarkovGenError::DeepHistoryReadDenied) => {}
   306	        other => panic!(
   307	            "halt-trigger #6: expected DeepHistoryReadDenied without \
   308	             TURINGOS_MARKOV_OVERRIDE=1; got {:?} (SG-15.4 + FR-15.5)",
   309	            other
   310	        ),
   311	    }
   312	
   313	    // Override path: result is Ok(()).
   314	    let ok = try_deep_history_read_with_override_check(true);
   315	    assert!(
   316	        ok.is_ok(),
   317	        "halt-trigger #6: TURINGOS_MARKOV_OVERRIDE=1 must permit deep-history \
   318	         read; got {:?}",
   319	        ok
   320	    );
   321	}

exec
/bin/bash -lc "nl -ba src/bin/audit_dashboard.rs | sed -n '900,985p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   900	        audit_trail_rows,
   901	        chain_proposal_count,
   902	        proposal_count_matches_audit_rows,
   903	        agent_audit_trail_chain_valid,
   904	    };
   905	
   906	    // TB-10 Atom 4: cross-reference user-task rows with claim audit rows so
   907	    // §11 can show solver + status + payout for each user-sponsored task.
   908	    for ut in user_tasks_in_progress.iter_mut() {
   909	        if let Some(claim) = claims_in_progress
   910	            .iter()
   911	            .find(|c| c.task_id == ut.task_id)
   912	        {
   913	            ut.solver = claim.solver.clone();
   914	            ut.claim_status = claim.claim_status.clone();
   915	            ut.payout_micro = claim.payout_amount_micro;
   916	        }
   917	    }
   918	
   919	    let all_pass = replay.all_indicators_pass();
   920	    Ok(DashboardReport {
   921	        run_id: replay.run_id.clone(),
   922	        epoch: replay.epoch,
   923	        chain: ChainStats {
   924	            l4_entries: replay.l4_entries,
   925	            l4e_entries: replay.l4e_entries,
   926	            head_commit_oid_hex: replay.detail.head_commit_oid_hex.clone(),
   927	            final_state_root_hex: replay.detail.final_state_root_hex.clone(),
   928	            final_ledger_root_hex: replay.detail.final_ledger_root_hex.clone(),
   929	            initial_q_state_loaded_from_disk: replay.detail.initial_q_state_loaded_from_disk,
   930	        },
   931	        indicators: IndicatorStatus {
   932	            all_pass,
   933	            ledger_root_verified: replay.ledger_root_verified,
   934	            system_signatures_verified: replay.system_signatures_verified,
   935	            state_reconstructed: replay.state_reconstructed,
   936	            economic_state_reconstructed: replay.economic_state_reconstructed,
   937	            cas_payloads_retrievable: replay.cas_payloads_retrievable,
   938	            agent_signatures_verified: replay.agent_signatures_verified,
   939	            proposal_telemetry_cas_retrievable: replay.proposal_telemetry_cas_retrievable,
   940	        },
   941	        run_facts,
   942	        per_agent,
   943	        proposal_flow,
   944	        branch_lineage,
   945	        golden_path,
   946	        cross_checks,
   947	        claims: claims_in_progress,
   948	        user_tasks: user_tasks_in_progress,
   949	        exhausted_runs: exhausted_runs_in_progress,
   950	        expired_tasks: expired_tasks_in_progress,
   951	        bankrupt_tasks: bankrupt_tasks_in_progress,
   952	        price_index: price_index_from_exposures(&exposures_in_progress),
   953	        exposures: exposures_in_progress,
   954	        // TB-15 Atom 6 — autopsy event counts derived from
   955	        // EconomicState.agent_autopsies_t at snapshot time. Build_report
   956	        // does not currently rebuild full EconomicState from the chain
   957	        // (TB-14 dashboard pattern is exposure-row accumulation); for v0
   958	        // we leave this empty + populated by future TB-16 controlled-arena
   959	        // wiring. Empty Vec is the SG-15.6 acceptable signal-state when
   960	        // no TaskBankruptcyTx has fired in the snapshot window.
   961	        autopsy_event_counts: Vec::new(),
   962	        // TB-15 Atom 6 — read latest Markov capsule pointer file if
   963	        // present. Best-effort; None when no capsule generated yet.
   964	        latest_markov_capsule_cid_hex: read_latest_markov_pointer(),
   965	    })
   966	}
   967	
   968	/// TRACE_MATRIX TB-15 Atom 6 (FR-15.4 + SG-15.6): best-effort read of
   969	/// `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt` from the
   970	/// repo-root convention path. Returns None when the file is absent
   971	/// (e.g. fresh repo without TB-15 generation yet) or unreadable.
   972	fn read_latest_markov_pointer() -> Option<String> {
   973	    let p = std::path::Path::new("handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt");
   974	    if p.exists() {
   975	        std::fs::read_to_string(p).ok().map(|s| s.trim().to_string())
   976	    } else {
   977	        None
   978	    }
   979	}
   980	
   981	/// TRACE_MATRIX TB-14 Atom 6 (FC3-N42; architect §5.1 + §5.5 SG-14.6):
   982	/// rebuild a synthetic `EconomicState` from the dashboard's `exposures`
   983	/// vec and call the canonical `state::compute_price_index` over it.
   984	///
   985	/// **Why synthetic**: the dashboard does not run a full `replay_full_transition`

exec
/bin/bash -lc "nl -ba src/bin/audit_dashboard.rs | sed -n '1888,1980p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1888	    /// must render as `None`, never as `0/0`, `0.0`, or any synthesized
  1889	    /// fraction.
  1890	    #[test]
  1891	    fn sg_14_6_dashboard_renders_none_for_zero_liquidity_nodes() {
  1892	        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
  1893	        let (k, e) = make_entry("n_zero", 0, 0, None, None);
  1894	        pi.insert(k, e);
  1895	        let s = render_section_14(&pi);
  1896	        assert!(
  1897	            s.contains("None"),
  1898	            "SG-14.6: zero-liquidity node must render `None` (FR-14.3 / \
  1899	             halt-trigger #5). Got:\n{s}"
  1900	        );
  1901	    }
  1902	
  1903	    // ───────────────────────────────────────────────────────────────────
  1904	    // TB-15 Atom 6 — §15 Autopsy + Markov render tests (SG-15.6)
  1905	    // ───────────────────────────────────────────────────────────────────
  1906	
  1907	    /// SG-15.6 ARCHITECT-MANDATED: dashboard §15 must render the literal
  1908	    /// privacy banner `AUTOPSY IS PRIVATE` (architect §6.4 ruling
  1909	    /// 2026-05-02 + CR-15.1). This test pins the banner string.
  1910	    #[test]
  1911	    fn sg_15_6_dashboard_carries_autopsy_is_private_banner() {
  1912	        let s = render_section_15(&[], None);
  1913	        assert!(
  1914	            s.contains("AUTOPSY IS PRIVATE"),
  1915	            "SG-15.6: §15 must contain the architect-mandated banner \
  1916	             `AUTOPSY IS PRIVATE`. Got render:\n{s}"
  1917	        );
  1918	    }
  1919	
  1920	    /// SG-15.6 + halt-trigger #5: dashboard §15 input signature carries
  1921	    /// only `(String, u32)` event counts and an Option<&str> Markov
  1922	    /// pointer hex — no raw private bytes possible. Render output
  1923	    /// surfaces counts + pointer only; never `private_detail_cid` payload.
  1924	    #[test]
  1925	    fn sg_15_6_dashboard_renders_event_counts_only_no_raw_bytes() {
  1926	        let counts = vec![
  1927	            ("event:tb15:event_a".to_string(), 2u32),
  1928	            ("event:tb15:event_b".to_string(), 5u32),
  1929	        ];
  1930	        let s = render_section_15(&counts, Some("abcd1234"));
  1931	        // Surfaces event_id + count + Markov pointer.
  1932	        assert!(s.contains("event:tb15:event_a"), "missing event_a; got:\n{s}");
  1933	        assert!(s.contains("event:tb15:event_b"), "missing event_b; got:\n{s}");
  1934	        assert!(s.contains(" 7 capsule"), "missing total cid count; got:\n{s}");
  1935	        assert!(s.contains("abcd1234"), "missing markov pointer hex; got:\n{s}");
  1936	        // Never embeds raw bytes (signature precludes; defense-in-depth: no
  1937	        // `0xPRIVATE` token would have been formattable from this input).
  1938	        assert!(!s.contains("private_detail_cid"));
  1939	    }
  1940	
  1941	    /// SG-15.6 + FR-15.4: when no Markov capsule pointer is present, the
  1942	    /// dashboard tells the audit-reader how to generate one — does not
  1943	    /// silently omit the field.
  1944	    #[test]
  1945	    fn sg_15_6_dashboard_explains_when_no_markov_pointer() {
  1946	        let s = render_section_15(&[], None);
  1947	        assert!(
  1948	            s.contains("no latest Markov capsule pointer"),
  1949	            "SG-15.6: empty Markov pointer must render an explicit \
  1950	             generation hint, not silently omit. Got:\n{s}"
  1951	        );
  1952	        assert!(
  1953	            s.contains("generate_markov_capsule"),
  1954	            "SG-15.6: empty Markov pointer must hint the binary name"
  1955	        );
  1956	    }
  1957	
  1958	    /// SG-15.6 + Markov default banner (CR-15.6 + halt-trigger #6): the
  1959	    /// dashboard explains that next-session boot defaults to constitution
  1960	    /// + latest capsule, with TURINGOS_MARKOV_OVERRIDE=1 required for
  1961	    /// deeper history.
  1962	    #[test]
  1963	    fn sg_15_6_dashboard_carries_markov_default_deny_explanation() {
  1964	        let s = render_section_15(&[], Some("deadbeef"));
  1965	        assert!(s.contains("TURINGOS_MARKOV_OVERRIDE=1"), "missing override env hint");
  1966	        assert!(s.contains("deeper history"), "missing default-deny hint");
  1967	    }
  1968	}
  1969	
  1970	#[cfg(test)]
  1971	mod tb12_render_tests {
  1972	    use super::*;
  1973	
  1974	    fn make_long(position_id: &str, node_id: &str, owner: &str, amount: i64) -> ExposureRecordRow {
  1975	        ExposureRecordRow {
  1976	            position_id: position_id.into(),
  1977	            node_id: node_id.into(),
  1978	            task_id: format!("task-{position_id}"),
  1979	            owner: owner.into(),
  1980	            side: "Long".into(),

exec
/bin/bash -lc 'find handover/evidence/tb_15_markov_capsule_2026-05-03 -maxdepth 2 -type f -print | sort' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/evidence/tb_15_markov_capsule_2026-05-03/LATEST_MARKOV_CAPSULE.txt
handover/evidence/tb_15_markov_capsule_2026-05-03/MARKOV_TB-15_2026-05-03.json
handover/evidence/tb_15_markov_capsule_2026-05-03/README.md

exec
/bin/bash -lc "nl -ba handover/evidence/tb_15_markov_capsule_2026-05-03/MARKOV_TB-15_2026-05-03.json | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	{
     2	  "capsule_id": [
     3	    178,
     4	    68,
     5	    241,
     6	    106,
     7	    31,
     8	    59,
     9	    213,
    10	    50,
    11	    208,
    12	    65,
    13	    164,
    14	    15,
    15	    227,
    16	    155,
    17	    43,
    18	    126,
    19	    124,
    20	    193,
    21	    47,
    22	    181,
    23	    142,
    24	    24,
    25	    182,
    26	    26,
    27	    237,
    28	    215,
    29	    106,
    30	    128,
    31	    16,
    32	    238,
    33	    177,
    34	    182
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
    71	  "l4_root": [
    72	    0,
    73	    0,
    74	    0,
    75	    0,
    76	    0,
    77	    0,
    78	    0,
    79	    0,
    80	    0,
    81	    0,
    82	    0,
    83	    0,
    84	    0,
    85	    0,
    86	    0,
    87	    0,
    88	    0,
    89	    0,
    90	    0,
    91	    0,
    92	    0,
    93	    0,
    94	    0,
    95	    0,
    96	    0,
    97	    0,
    98	    0,
    99	    0,
   100	    0,
   101	    0,
   102	    0,
   103	    0
   104	  ],
   105	  "l4e_root": [
   106	    0,
   107	    0,
   108	    0,
   109	    0,
   110	    0,
   111	    0,
   112	    0,
   113	    0,
   114	    0,
   115	    0,
   116	    0,
   117	    0,
   118	    0,
   119	    0,
   120	    0,
   121	    0,
   122	    0,
   123	    0,
   124	    0,
   125	    0,
   126	    0,
   127	    0,
   128	    0,
   129	    0,
   130	    0,
   131	    0,
   132	    0,
   133	    0,
   134	    0,
   135	    0,
   136	    0,
   137	    0
   138	  ],
   139	  "cas_root": [
   140	    0,
   141	    0,
   142	    0,
   143	    0,
   144	    0,
   145	    0,
   146	    0,
   147	    0,
   148	    0,
   149	    0,
   150	    0,
   151	    0,
   152	    0,
   153	    0,
   154	    0,
   155	    0,
   156	    0,
   157	    0,
   158	    0,
   159	    0,
   160	    0,
   161	    0,
   162	    0,
   163	    0,
   164	    0,
   165	    0,
   166	    0,
   167	    0,
   168	    0,
   169	    0,
   170	    0,
   171	    0
   172	  ],
   173	  "typical_errors": [],
   174	  "unresolved_obs": [
   175	    "handover/alignment/OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md",
   176	    "handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md",
   177	    "handover/alignment/OBS_CO1_8_V1_DEFERRED_2026-04-29.md",
   178	    "handover/alignment/OBS_CONSTITUTION_MERMAID_FENCE_2026-04-22.md",
   179	    "handover/alignment/OBS_R022_TB-6_ATOM_1_2_TRACE_MATRIX_TEXT_EXTENSION_2026-05-01.md",
   180	    "handover/alignment/OBS_R022_TB-8_ATOM_1_CLAIMENTRY_TRACE_MATRIX_TEXT_EXTENSION_2026-05-02.md",
   181	    "handover/alignment/OBS_R022_TB13_RESOLUTIONREF_REMOVED_2026-05-03.md",
   182	    "handover/alignment/OBS_R022_TB14_PRICEINDEX_REMOVED_2026-05-03.md",
   183	    "handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md",
   184	    "handover/alignment/OBS_RESOLUTIONS_INDEX_TB15_2026-05-03.md",
   185	    "handover/alignment/OBS_ROADMAP_POST_TB7_OVERRIDE_2026-05-01.md",
   186	    "handover/alignment/OBS_STATE_TRANSITION_SPEC_V1_5_HOUSEKEEPING_2026-04-29.md",
   187	    "handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md",
   188	    "handover/alignment/OBS_TB13_AUDIT_RESIDUAL_CHALLENGES_2026-05-03.md",
   189	    "handover/alignment/OBS_TB13_FENCE_MECHANISM_DOOM_LOOP_2026-05-03.md",
   190	    "handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md",
   191	    "handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md",
   192	    "handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md",
   193	    "handover/alignment/OBS_WHITEPAPER_V2_DUAL_DOMAIN_2026-04-27.md",
   194	    "handover/alignment/OBS_WHITEPAPER_V2_INITAI_PLACEHOLDER_2026-04-27.md",
   195	    "handover/alignment/OBS_WHITEPAPER_V2_PREDICATE_VISIBILITY_TRINITY_2026-04-27.md",
   196	    "handover/alignment/OBS_WHITEPAPER_V2_QT_FIVE_ROOT_EXTENSION_2026-04-27.md"
   197	  ],
   198	  "next_session_context_cid": [
   199	    87,
   200	    91,
   201	    95,
   202	    217,
   203	    194,
   204	    163,
   205	    133,
   206	    191,
   207	    116,
   208	    139,
   209	    148,
   210	    30,
   211	    5,
   212	    150,
   213	    220,
   214	    218,
   215	    75,
   216	    5,
   217	    66,
   218	    9,
   219	    83,
   220	    41,

exec
/bin/bash -lc "printf 'constitution '; sha256sum constitution.md; printf 'latest '; cat handover/evidence/tb_15_markov_capsule_2026-05-03/LATEST_MARKOV_CAPSULE.txt 2>/dev/null; printf '\\n'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
constitution eec695459c71fbef3685583485deb431fe3b561657b2f285b7c5e7e220e59e03  constitution.md
latest b244f16a1f3bd532d041a40fe39b2b7e7cc12fb58e18b61aedd76a8010eeb1b6

exec
/bin/bash -lc 'rg -n "constitution_hash|l4_root|l4e_root|cas_root|flowchart|flowchart_hash|TRACE_FLOWCHART|FC" handover/evidence/tb_15_markov_capsule_2026-05-03 handover/alignment handover/tracer_bullets/TB-15_charter_2026-05-03.md handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:42:    FC1-N32 / FC1-N33 / FC2-N30 / FC3-N43)
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:54:| 2 | markov_capsule_references_constitution_hash | 5 | GREEN — `MarkovEvidenceCapsule.constitution_hash == sha256(constitution.md)` |
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:70:| SG-15.7 | Markov capsule references constitution hash | `MarkovEvidenceCapsule.constitution_hash` field; halt-trigger #2 |
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:75:- G-15.10 (FC1-N32 + FC1-N33 + FC2-N30 + FC3-N43 each ≥1 witness) — **GREEN** all 4 added.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:83:- `src/runtime/markov_capsule.rs` (Atom 5): `ObsId` newtype + `MarkovEvidenceCapsule` struct + `with_constitution_hash` constructor + `try_deep_history_read_with_override_check` gate + `override_set_from_env` + `write_markov_capsule` + `scan_unresolved_obs` + `sha256_of_file` + `MarkovGenError`. 8 in-module tests.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:99:- `tests/fc_alignment_conformance.rs`: + 4 TB-15 witnesses (FC1-N32 / FC1-N33 / FC2-N30 / FC3-N43).
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:115:> TB-15 establishes the Lamarckian Autopsy + Markov EvidenceCapsule substrate. AgentAutopsyCapsule (per-agent, per-event, CAS-resident, AuditOnly) records loss/bankruptcy events derived deterministically from ChainTape evidence — **NEVER from agent LLM self-narration**. The capsule chain is anchored on `EconomicState.agent_autopsies_t[event_id]: Vec<Cid>` (13th sub-field; sequencer-side; NOT projected to `AgentVisibleProjection`). TypicalErrorBroadcast clustering (N≥3 threshold per architect §3.2.3) emits `TypicalErrorSummary` objects embedding `public_summary` text + capsule_id Cids only — never `private_detail_cid` payload bytes. MarkovEvidenceCapsule binds `constitution_hash` + L4 root + L4.E root + CAS root + `previous_capsule_cid` + `typical_errors` + `unresolved_obs` + `next_session_context_cid` into an end-of-TB rollup that becomes the default next-session bootstrap context (FR-15.4). Deep-history reads default-deny without `TURINGOS_MARKOV_OVERRIDE=1` (FR-15.5 + halt-trigger #6).
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:119:> Class 2 envelope intact — no AgentVisibleProjection mod, no read-view-authorization mod, single new sequencer dispatch hook (TaskBankruptcyTx Step 3.5 + apply_one Stage 3.5). All 6 halt-triggers + 8 architect SG green. All 4 P-roadmap exit criteria addressed (P4-Exit1/2/3 + P5-Exit1/2 prep). All 4 FC-IDs (FC1-N32 + FC1-N33 + FC2-N30 + FC3-N43) have witness tests.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:124:- **L4 / L4.E / CAS root chain-readers**: Markov capsule generator currently uses zero placeholders for `l4_root` / `l4e_root` / `cas_root`. Future TB will wire to the actual chain head readers (`Sequencer::current_state_root` / `LedgerWriter::current_root` / CAS metadata digest). Track in TB-16+ controlled-arena work.
handover/tracer_bullets/TB-15_charter_2026-05-03.md:11:**FC-trace**: `Art.II.1` (typical-error broadcast — N≥3 cluster threshold; spec §3.2.3) + `Art.III.1` (raw failure log shielding — `private_detail_cid` AuditOnly default; CR-15.1) + `Art.III.2` (封装细节 — agent-scoped read-view; SG-15.2) + `Art.III.3` (broadcast pollution prevention — public_summary only when typical; CR-15.2) + `Art.III.4` (Goodhart shield — autopsy cannot mutate predicates/permissions; CR-15.3 + CR-15.4 + SG-15.8) + `Art.V.1` (三权分立 — ArchitectAI may propose; JudgeAI/VetoAI veto-only; CR-15.3 + CR-15.4) + `Art.0.2` (Tape Canonical — autopsy + Markov capsule are deterministic functions of L4 + L4.E + CAS; replay-identical bytes).
handover/tracer_bullets/TB-15_charter_2026-05-03.md:14:- **Flowchart 1 (runtime)**: `FC1-N32` = `write_autopsy_capsule` writer (CAS-resident; sequencer-emitted; private-by-default). `FC1-N33` = TaskBankruptcyTx dispatch arm autopsy emission (per-agent loss anchor for FR-15.1 first wire-in).
handover/tracer_bullets/TB-15_charter_2026-05-03.md:15:- **Flowchart 2 (signal)**: `FC2-N30` = `cluster_autopsies` pure-fn (pure aggregator; loss_reason_class group-by; threshold=3 → TypicalErrorSummary). Public broadcast surface uses `public_summary` only.
handover/tracer_bullets/TB-15_charter_2026-05-03.md:16:- **Flowchart 3 (meta)**: `FC3-N43` = `MarkovEvidenceCapsule` schema + `generate_markov_capsule` binary (constitution_hash + L4 root + L4.E root + CAS root + open OBS list + previous_capsule_cid; deterministic). FR-15.4 next-session bootstrap source.
handover/tracer_bullets/TB-15_charter_2026-05-03.md:59:  halt-trigger #2  markov_capsule_references_constitution_hash —
handover/tracer_bullets/TB-15_charter_2026-05-03.md:60:            MarkovEvidenceCapsule.constitution_hash equals sha256 of
handover/tracer_bullets/TB-15_charter_2026-05-03.md:72:flowchart_trace:
handover/tracer_bullets/TB-15_charter_2026-05-03.md:74:    FC1-N32 = write_autopsy_capsule (CAS writer; AuditOnly default;
handover/tracer_bullets/TB-15_charter_2026-05-03.md:76:    FC1-N33 = TaskBankruptcyTx dispatch arm: for each agent with
handover/tracer_bullets/TB-15_charter_2026-05-03.md:80:    FC2-N30 = cluster_autopsies (pure aggregator; threshold=3;
handover/tracer_bullets/TB-15_charter_2026-05-03.md:83:    FC3-N43 = MarkovEvidenceCapsule + generate_markov_capsule binary
handover/tracer_bullets/TB-15_charter_2026-05-03.md:112:      (constitution_hash + L4 root + L4.E root + CAS root + previous
handover/tracer_bullets/TB-15_charter_2026-05-03.md:140:    capsule_id, previous_capsule_cid, constitution_hash,
handover/tracer_bullets/TB-15_charter_2026-05-03.md:141:    l4_root, l4e_root, cas_root, typical_errors,
handover/tracer_bullets/TB-15_charter_2026-05-03.md:224:      { "constitution_hash": ..., "latest_markov_cid": ..., "boot_seq":
handover/tracer_bullets/TB-15_charter_2026-05-03.md:233:        l4_root) returns DeepHistoryReadDenied.
handover/tracer_bullets/TB-15_charter_2026-05-03.md:276:- `markov_capsule_references_constitution_hash` (SG-15.7; halt #2)
handover/tracer_bullets/TB-15_charter_2026-05-03.md:323:- reads `constitution.md` → `constitution_hash = sha256(bytes)`
handover/tracer_bullets/TB-15_charter_2026-05-03.md:328:- builds `next_session_context_cid` from minimal JSON `{"constitution_hash":..., "latest_markov_cid":..., "boot_seq":[...]}` written to CAS as `ObjectType::NextSessionContext`
handover/tracer_bullets/TB-15_charter_2026-05-03.md:336:- capsule.constitution_hash equals sha256 of constitution.md (SG-15.7)
handover/tracer_bullets/TB-15_charter_2026-05-03.md:339:**Halt triggers green**: #2, #6 (markov_capsule_references_constitution_hash, deep_history_read_without_override_fails).
handover/tracer_bullets/TB-15_charter_2026-05-03.md:346:- latest Markov capsule pointer (Cid hex + previous_cid + constitution_hash + l4_root + l4e_root + cas_root)
handover/tracer_bullets/TB-15_charter_2026-05-03.md:358:**Final commit**: `TB-15 SHIPPED — Lamarckian Autopsy + Markov EvidenceCapsule (Class 2 self-audit; 8/8 SG; 6/6 halt-triggers; closes architect §6 spec FR-15.1..6 + CR-15.1..6 + SG-15.1..8)`. FC-trace: `FC1-N32 + FC1-N33 + FC2-N30 + FC3-N43`.
handover/tracer_bullets/TB-15_charter_2026-05-03.md:387:2. MarkovEvidenceCapsule does not reference constitution_hash
handover/tracer_bullets/TB-15_charter_2026-05-03.md:405:| SG-15.7 | Markov capsule references constitution hash and flowchart hashes | `tb_15_halt_triggers.rs::markov_capsule_references_constitution_hash` |
handover/tracer_bullets/TB-15_charter_2026-05-03.md:408:| G-15.10 | FC1-N32 + FC1-N33 + FC2-N30 + FC3-N43 each have ≥1 witness test | `tests/fc_alignment_conformance.rs` |
handover/alignment/OBS_R022_TB-8_ATOM_1_CLAIMENTRY_TRACE_MATRIX_TEXT_EXTENSION_2026-05-02.md:19:R-022 expects every `pub` symbol in `src/` to carry a `/// TRACE_MATRIX <FC-id>: <role>` doc-comment. The hook detects 1 backlink "removed" — but in fact the line was **extended with TB-8 Atom 1 context** rather than removed. After the edit:
handover/alignment/OBS_R022_TB-6_ATOM_1_2_TRACE_MATRIX_TEXT_EXTENSION_2026-05-01.md:21:R-022 expects every `pub` symbol in `src/` to carry a `/// TRACE_MATRIX <FC-id>: <role>` doc-comment. The hook detects 3 backlinks "removed" — but in fact those 3 lines were **extended with TB-6 Atom 1.2 context** rather than removed. After the edit:
handover/alignment/OBS_R022_TB-6_ATOM_1_2_TRACE_MATRIX_TEXT_EXTENSION_2026-05-01.md:38:368:    /// TRACE_MATRIX FC3-N1 + P1:6 (TB-6 Atom 1.2) — open or create a JSONL-backed writer.
handover/alignment/OBS_R022_TB-6_ATOM_1_2_TRACE_MATRIX_TEXT_EXTENSION_2026-05-01.md:39:427:    /// TRACE_MATRIX FC3-N1 (TB-6 Atom 1.2) — convenience: is this writer JSONL-backed?
handover/alignment/OBS_STATE_TRANSITION_SPEC_V1_5_HOUSEKEEPING_2026-04-29.md:111:**FC-trace**: FC3-L4 (downstream supersession sediment for L4 transition ledger family; alignment-only observation, not a behavior change).
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:4:- Flowchart raw: `FC_ELEMENTS_2026-04-22.md` (134 elements: 48 nodes + 63 edges + 23 subgraphs)
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:14:- 📄 docs-only (flowchart element is a document reference, not runtime code)
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:20:| FC Element ID | Constitution Label | Proposed Symbol | File:Line | DocComment | Status | Action (Stage 2+3) |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:22:| **FC1 basic cycle** ||||||||
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:23:| FC1-N1 | `Q_t = ⟨q_t, HEAD_t, tape_t⟩` | `QState` + `Tape::time_arrow` + `Kernel::tape` | `src/bus.rs:70`, `src/ledger.rs:146`, `src/kernel.rs:20` | Y/N/N | ⚠️ | add backlinks on `time_arrow` and `Kernel::tape` |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:24:| FC1-N2 | `q_t` | `QState`, `TuringBus::q_state` | `src/bus.rs:53`, `src/bus.rs:70` | Y/Y | ✅ | none |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:25:| FC1-N3 | `HEAD_t` | `time_arrow().last()` idiom | `src/ledger.rs:146` + call sites | N | ⚠️ | add `fn head() -> Option<NodeId>` helper on Tape |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:26:| FC1-N4 | `tape_t` | `Tape`, `Kernel::tape` | `src/ledger.rs:44`, `src/kernel.rs:20` | Y/N | ⚠️ | backlink `Kernel::tape` |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:27:| FC1-N5 | `rtool` | `ReadTool::project` + `DefaultReadTool` | `src/sdk/read_tool.rs:24,37` | Y/N | ⚠️ | migrate bus.snapshot() callers to rtool |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:28:| FC1-N6 | `input = ⟨q_i, s_i⟩` | `UniverseSnapshot` + `build_agent_prompt` | `src/sdk/snapshot.rs:22`, `src/sdk/prompt.rs:15` | Y/Y | ✅ | none |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:29:| FC1-N7 | `δ / AI` | `ResilientLLMClient::generate` | `src/drivers/llm_http.rs:84` | Y | ✅ | none |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:30:| FC1-N8 | `output = ⟨q_o, a_o⟩` | `AgentOutput`, `parse_agent_output` | `src/sdk/protocol.rs:40,148` | Y/Y | ✅ | none |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:31:| FC1-N9 | `q_o` | `AgentOutput::q_delta` | `src/sdk/protocol.rs:42` | Y | ✅ | none |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:32:| FC1-N10 | `a_o` | `AgentOutput::action` | `src/sdk/protocol.rs:44` | Y | ✅ | none |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:33:| FC1-N11 | `∏p predicates` | `TuringBus::evaluate_predicates`, `Predicate` trait | `src/bus.rs:148`, `src/sdk/predicate.rs:88` | Y/Y | ⚠️ | wire evaluator.rs to call evaluate_predicates before append |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:34:| FC1-N12 | `individual p predicates` | `{Forbidden,Sorry,PayloadSize}Predicate`, `Lean4Oracle::verify_*` | `src/sdk/predicate.rs:106,124,139`, oracle | Y all | ⚠️ | wrap `Lean4Oracle::verify_partial` in a `Predicate` impl |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:35:| FC1-N13 | `wtool` | `WriteTool::write`, `DefaultWriteTool::write`, `TuringBus::append_oracle_accepted` | `src/sdk/write_tool.rs:29,84`, `src/bus.rs:324,347` | Y/N/Y/Y | ⚠️ | migrate evaluator direct bus.append calls through WriteTool |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:36:| FC1-N14 | `Q_{t+1}` success branch | `append_internal`, `halt_with_reason` | `src/bus.rs:421,207` | N/Y | ⚠️ | backlink `append_internal` |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:37:| FC1-N15 | `Q_t` branch (∏p=0) | `PartialVerdict::Reject`, `BusResult::Vetoed` | oracle:328, bus:111 | Y/N | ⚠️ | backlink `BusResult::Vetoed` |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:38:| **FC2 init / halt / tick** ||||||||
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:39:| FC2-N16 | `InitAI` | `run_swarm`, `run_oneshot` | evaluator:335,182 | Y/Y | ✅ | backlink FC2-N16 label |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:40:| FC2-N17 | `human architect` | `constitution.md` (author) | constitution.md | — | 📄 | non-runtime; explicit in matrix |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:41:| FC2-N18 | `law / ground truth` | `constitution.md` | constitution.md | — | 📄 | non-runtime |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:42:| FC2-N19 | `initAI --once→ predicates` | `TuringBus::register_predicate` API | `src/bus.rs:136` | Y | 🔨 | **Stage 3: add caller in run_swarm + run_oneshot to register 3 default predicates at boot** |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:43:| FC2-N20 | `initAI --once→ mr` | TICK_INTERVAL read + `emit_mr_tick_node` | evaluator:459, bus:385 | N/Y | ✅ | none |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:44:| FC2-N21 | `initAI --once→ Q0` | `Kernel::new`, `TuringBus::new`, `TuringBus::init` | `src/kernel.rs:50`, `src/bus.rs:115,299` | N/N/Y | ⚠️ | backlink `Kernel::new`, `TuringBus::new` |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:45:| FC2-N22 | `HALT` | `QState::Halted`, `halt_with_reason`, `halt_and_settle` | `src/bus.rs:55,207,581` | N/Y/Y | ⚠️ | backlink `QState::Halted` |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:46:| FC2-N23 | `HaltReason` variants | `HaltReason` enum (5 variants), `extract_halt_reason` | `src/ledger.rs:230`, evaluator:1116 | Y/Y | ✅ | none |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:47:| FC2-N24 | `clock` | `TuringBus::clock`, `for tx in 0..max_transactions` loop, `TICK_INTERVAL` | `src/bus.rs:66`, evaluator:485,459 | N/N/N | ⚠️ | backlink `clock` field |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:48:| FC2-N25 | `mr` | `let mr_summary = format!(...)` inline, `emit_mr_tick_node` | evaluator:504, bus:385 | N/Y | ⚠️ | backlink inline mr_summary block |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:49:| FC2-N26 | `mr --map→ tape0` | `tape.time_arrow().len()`, `market_ticker(5)` (used by mr_summary builder) | evaluator:488,490 | N/N | ⚠️ | backlink |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:50:| FC2-N27 | `mr --reduce→ tape1` | `emit_mr_tick_node` | `src/bus.rs:385` | Y | ✅ | none |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:51:| FC2-N28 | `tools_other` | `WriteTool::write_with_tools`, `TuringBus::tools`, `mount_tool` call sites | `src/sdk/write_tool.rs:57`, `src/bus.rs:64`, evaluator mount sites | Y/N/N | ⚠️ | backlink `TuringBus::tools` field |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:52:| **FC3 anti-oreo / system-level** ||||||||
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:53:| FC3-N29 | `boot` | `async fn main`, `TuringBus::boot` | evaluator:88, bus:286 | N/Y | ⚠️ | backlink `fn main` |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:54:| FC3-N30 | `constitution file` | `constitution.md` | constitution.md | — | 📄 | non-runtime |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:55:| FC3-N31 | `logs archive` | `TuringBus::with_wal_path`, `Wal::replay`, `Wal::write_event` | `src/bus.rs:227`, `src/wal.rs:70,54` | Y/Y/N | ⚠️ | backlink `write_event` |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:56:| FC3-N32 | `JudgeAI` | external/manual (Codex/Gemini dual-audit) | — | — | 📅 | Phase 11+ deferred |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:57:| FC3-N33 | `ArchitectAI` | external/manual (Claude code editing) | — | — | 📅 | Phase 11+ deferred |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:58:| FC3-N34 | `readonly guard on {constitution, logs}` | WAL append-only semantics; no FS readonly enforcement | `src/wal.rs:70` | Y | 📅 | Phase 11+: add FS-level readonly check at init |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:59:| FC3-N35 | `anti-oreo top→agents→tools` | `evaluate_predicates` + `let agent_ids` + `TuringTool` | bus:148, evaluator:431, tool:38 | Y/N/Y | ⚠️ | backlink agent_ids lifecycle |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:60:| FC3-N36 | `agents` | `let agent_ids`, round-robin selection | evaluator:431,577 | N/N | ⚠️ | backlink |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:61:| FC3-N37 | `tools` | `TuringTool` trait + concrete {Wallet, Search, Librarian} + `Lean4Oracle` | `src/sdk/tool.rs:38` + impls | Y mostly | ✅ | backlink Lean4Oracle to FC3-N37 |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:62:| FC3-N38 | `tape Q` | same as FC1-N4 | — | — | ✅ | (dedup of FC1-N4) |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:63:| FC3-N39 | `log` | `Ledger`, `LedgerEvent`, `Ledger::append` | `src/ledger.rs:332,296,347` | Y/Y/Y | ✅ | none |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:64:| FC3-N40 | `logs → feedback → ArchitectAI` | external (no runtime automation) | — | — | 📅 | Phase 11+ deferred |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:65:| FC3-N41 | `init → error → re-init → boot` | `exit(2)` + external batch runner retry | evaluator:278,329,388 | N/N/N | 📅 | Phase 11+: in-process retry; for now external batch retry works |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:66:| FC3-N42 | `constitution --abide→ JudgeAI+ArchitectAI` | manual policy (CLAUDE.md Audit Standard) | CLAUDE.md | — | 📅 | Phase 11+ deferred |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:67:| FC3-N43 | `JudgeAI --veto→ ArchitectAI` | manual policy (VETO>CHALLENGE>PASS rule) | docs | — | 📅 | Phase 11+ deferred |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:75:| FC Element ID | Edge (From → To) | Constitutional label | Code representation | Status |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:77:| FC1-E16 | `p → wtool` | `|1|` (∏p = 1) | `evaluate_predicates` returns non-Reject → proceed to wtool | ✅ |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:78:| FC1-E18 | `p → Q0` | `|0|` (∏p = 0) | `evaluate_predicates` returns Reject → no append, state preserved | ✅ |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:79:| FC2-E22 | `p → wtool` | `Q_{t+1} = wtool(output) if ∏p = 1` | same as FC1-E16 with explicit formula | ✅ |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:80:| FC2-E23 | `p → Q0` | `Q_{t+1} = Q_t if ∏p = 0` | same as FC1-E18 | ✅ |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:81:| FC2-E24 | `q1 → halt` | `if q = halt` | `if matches!(bus.q_state, QState::Halted{..})` check in evaluator loop | ⚠️ backlink |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:82:| FC2-E25 | `clock → mr` | drives tick | `if tx > 0 && tx % tick_interval == 0` | ⚠️ backlink |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:83:| FC2-E26 | `mr → tape0` | `|map|` | read side of mr_summary build (tape.time_arrow(), market_ticker) | ⚠️ backlink |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:84:| FC2-E27 | `mr → tape1` | `|reduce|` | `emit_mr_tick_node(summary)` | ✅ |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:85:| FC2-Init-E | `initAI --x|once| predicates / mr / Q0` | one-time setup | 🔨 FC2-N19 caller missing | 🔨 |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:86:| FC3-Feedback | `logs → feedback → architectAI` | automated improvement loop | 📅 deferred | 📅 |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:87:| FC3-Veto | `judgeAI → veto → architectAI` | policy arrow | 📅 deferred | 📅 |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:88:| FC3-Abide | `constitution → abide → judge/architect` | policy arrow | 📅 deferred | 📅 |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:89:| FC3-Reinit | `init → error → re-init → boot` | automated retry | 📅 deferred | 📅 |
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:91:The remaining ~50 unlabeled edges are **structural** (transitions already witnessed by the node mapping) — they do not require separate alignment work. Listed in `FC_ELEMENTS_2026-04-22.md` for completeness.
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:95:## § 3. Orphan Rust symbols (code without flowchart parent)
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:97:Per 宪法不能改 directive, orphans are accepted as **implementation-auxiliary** (extensions not inscribed in FC-1/2/3 but serve constitutional principles elsewhere):
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:118:1. **FC2-N19** 🔨: `run_swarm` + `run_oneshot` must call `bus.register_predicate(...)` × 3 at init for {ForbiddenPattern, Sorry, PayloadSize}. Currently the API exists but no caller wires it.
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:119:2. **FC1-N11**: evaluator.rs should call `bus.evaluate_predicates(ctx, payload)` before `bus.append`, to honor the ∏p → wtool gate. Currently the legacy hard-coded checks in `append_internal` duplicate this work.
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:120:3. **FC1-N12**: wrap `Lean4Oracle::verify_partial` as a `Predicate` impl so the full ∏p chain is a single registered list.
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:121:4. **FC1-N5 / N13**: migrate evaluator from direct `bus.snapshot()` / `bus.append` calls to `rtool.project()` / `wtool.write()`.
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:122:5. **FC1-N3**: add `Tape::head() -> Option<NodeId>` helper (single idiom, instead of scattered `time_arrow().last()`).
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:125:**Out-of-scope (Phase 11+ 📅)**: FC3-N32/33/34/40/41/42/43 — JudgeAI runtime + ArchitectAI runtime + FS readonly + feedback loop + auto re-init + runtime veto. Documented explicitly; no Stage 3 work.
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:131:- FC-1: 14 core-node rows (N1-N15) + 2 labeled edges (E16, E18) = 17 matrix rows. Raw extract: 14 nodes + 18 edges + 8 subgraphs = 40. Edges without labels (dotted/thick) collapse into their endpoint nodes.
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:132:- FC-2: 13 core-node rows (N16-N28) + 6 labeled edges = 19 matrix rows. Raw: 22 nodes + 28 edges + 11 subgraphs = 61. Many raw nodes are FC-1 duplicates (e.g., `q_t` appears in both).
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:133:- FC-3: 15 rows (N29-N43). Raw: 12 nodes + 17 edges + 4 subgraphs = 33.
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:143:- 🔨 missing-actionable (Stage 3): **1** row (FC2-N19)
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:155:FC-2 and FC-3 are indented pseudo-blocks in `constitution.md` — their opening ` ```mermaid ` fences are missing. They won't render on GitHub. Per user directive ("宪法不能改") I do NOT modify the constitution. Filing this as an audit observation for human architect to address when they next revise the document. Does not block Phase Z' execution.
handover/alignment/OBS_R022_TB13_RESOLUTIONREF_REMOVED_2026-05-03.md:19:The TB-13 typed-tx surface is still flowchart-anchored — `CompleteSetRedeemTx` itself retains its `TRACE_MATRIX TB-13 Atom 1 (architect §4.3 + FR-13.4..5 + SG-13.5..6)` backlink at `src/state/typed_tx.rs`. ResolutionRef was an internal helper type without an independent flowchart role. Removing the type cleanly removes the backlink — there is no orphan code; the role it carried (claimed-outcome assertion) is now expressed directly via `CompleteSetRedeemTx.outcome`.
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:14:This panic is **not** a FC2-N22 HALT, even though the surface effect ("process stops") looks similar. The mapping question matters because TRACE_MATRIX_v1 needed to decide where to attach this code path in the constitutional flowcharts.
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:16:## Why TRUST_ROOT_TAMPERED ≠ FC2-N22 HALT
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:18:| Property | FC2-N22 HALT | TRUST_ROOT_TAMPERED panic |
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:26:FC2 has a `HALT` node (FC2-N22) that lives *inside* the boot/tick lifecycle. TRUST_ROOT_TAMPERED fires *before* the boot lifecycle — it is a precondition violation on the readonly base.
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:28:## Closer constitutional match: FC3-E14 (`error → re-init → boot`)
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:30:FC3 has a top-level edge `init → error → re-init → boot` (line 711 of constitution.md, FC3-E13/E14). The semantics:
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:32:- Something during init detected a problem ("need to improve?" rhombus, FC3-N12).
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:33:- Control returns to `boot` (FC3-N1).
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:36:Trust Root verification *is* an init-time precondition check. If it fails, the constitutional response is "re-init" — return to boot. In v4, "return to boot" = the process exits and the surrounding harness (shell, batch runner) restarts it. The `panic!` is the immediate-abort leaf of FC3-E14.
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:38:This matches even though FC3 conceives of the error→re-init loop as automated. In Phase B v4, automation lives outside the binary (TRACE_MATRIX_v0 § 1 row FC3-N41 currently 📅 Phase 11+).
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:45:2. Adding the variant would lie about the constitutional structure — we would be claiming Trust Root verification is part of the FC2 boot/tick cycle, when it is in fact a precondition for FC2 to begin.
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:52:If/when Phase 11+ lands an in-process re-init mechanism (TRACE_MATRIX FC3-N41), it can use `Result<(), TrustRootError>` from `verify_trust_root` instead of letting the panic propagate. At that point the panic becomes the *uncaught* path and structured retry becomes the *caught* path. Constitution still does not need to change — both paths land at FC3-E14.
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:56:- `src/main.rs:11-14` — call site, TRACE_MATRIX backlink to FC3-E14
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:57:- `src/boot.rs:62` — `verify_trust_root` itself, TRACE_MATRIX backlink to FC3-N34
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md:59:- `constitution.md` line 670-714 (FC3 mermaid block) — source flowchart
handover/alignment/OBS_R022_TB14_PRICEINDEX_REMOVED_2026-05-03.md:16:The legacy `PriceIndex` backlink (`/// TRACE_MATRIX WP § 2 — tx → posted price`) anchored a stub type that had no flowchart role: WP § 2 lists `price_index_t` as a sub-field of `EconomicState`, but neither the constitutional flowchart nor any TB shipped a sequencer dispatch arm that wrote to it. The role it carried is now expressed by `compute_price_index` whose backlink is `TRACE_MATRIX FC3-N42` (architect §5.1 + charter §3 Atom 2) — registered in this same atom's commit at `src/state/price_index.rs` and witnessed by `tests/fc_alignment_conformance.rs::fc3_n42_compute_price_index_pure_fn_witness`.
handover/alignment/OBS_R022_TB14_PRICEINDEX_REMOVED_2026-05-03.md:18:Net effect: the canonical state has 12 sub-fields (was 13 pre-TB-14); the price-derivation role moves from a never-populated stub field to a pure derived view; flowchart anchoring strengthens (FC3-N42 typed and witnessed) rather than weakens.
handover/alignment/OBS_R022_TB14_PRICEINDEX_REMOVED_2026-05-03.md:41:- Replacement TRACE_MATRIX backlink: `src/state/price_index.rs::compute_price_index` (`TRACE_MATRIX FC3-N42`); `src/state/price_index.rs::RationalPrice::dominates_by` (`TRACE_MATRIX FC3-N42 helper`).
handover/alignment/OBS_R022_TB14_PRICEINDEX_REMOVED_2026-05-03.md:42:- FC3-N42 witness: `tests/fc_alignment_conformance.rs::fc3_n42_compute_price_index_pure_fn_witness`.
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:9:- A4 (post-A3): decomposed metrics — `hit_max_tx`, `tactic_diversity`, `verifier_wait_ms` added as non-Optional v2 fields + `compute_tactic_diversity` helper; per-row decomposition of `solve_rate` / `tokens_per_solve` / `time_per_solve` (all derivable from existing `progress` / `total_run_token_count` / `total_wall_time_ms`). FC-trace: FC2-N22 (HALT decomposition for `hit_max_tx`) + FC1-N11 (∏p decision diversity for `tactic_diversity`) + FC1-N12 (oracle scope for `verifier_wait_ms`).
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:10:- A5 (post-A4): per-agent budget normalization — new `budget_regime` module (`BUDGET_REGIME` + `MAX_TRANSACTIONS` env vars; 4-variant enum; pure parser + scaler + env-coupled resolver); `budget_regime` + `budget_max_transactions` added as non-Optional v2 fields on `RunAggregate` and the legacy `PputResult`; loop bound at `run_swarm` switched from hardcoded `let max_transactions = 200` to `resolve_budget(n_agents)` — default (env unset) preserves Phase B baseline (`total_proposal × 200`) bit-for-bit. PREREG_AMENDMENT_p0_defer § 3 condition 3 satisfied: `MaxTxExhausted` rows now disambiguated across N values. FC-trace: FC2-N22 (HALT decomposition by budget regime) + FC1-N7 (δ instances determining the per-agent share under PerAgent regime). Trust Root manifest 26 → 27 (corrected A8e5 J4 — the A1 PREREG amendment had taken the count to 25, A3's `agent_models.rs` to 26 before A5 landed; round-1 packet undercounted by 1).
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:11:- A6 (post-A5): per-line FC tagging via structured JSON events — new `fc_trace` module (pure stdlib; zero new deps); `FcId` enum (FC1-N7 / FC1-N11 / FC1-N12 / FC1-E18 / FC2-N20 / FC2-N22 / FC3-N31); `fc_event!`-style `emit_event` API; `FC_TRACE=1` gate (cached in `OnceLock`); `FC_TRACE_FILE=<path>` redirects emit to file (default sink stderr). 9 anchor sites wired (post-A8e total — 8 in `run_swarm` + 1 in `run_oneshot`): the original A6 commit shipped 6 sites (FC2-N22 synthetic short-circuit, FC2-N20 mr tick, FC2-N22 OMEGA full-proof, FC2-N22 OMEGA per-tactic, FC2-N22 natural MaxTxExhausted with `budget_regime` payload, FC1-N12 verify bracket in oneshot). A8e fix F4 added 3 more in `run_swarm` (FC1-N12 verify_omega_detailed × 2 paths + FC1-N12 verify_partial), bringing the post-A8e total to 9 production emit sites. Counted by grepping `fc_trace::emit_event(` in `experiments/minif2f_v4/src/bin/evaluator.rs` (Gemini R6 finding: round-5 phrasing "Six anchor sites" was stale relative to its own following sentence — corrected here in A8e6 K4). End-to-end smoke test exercises FC_TRACE=1 in a child process (subprocess required because `OnceLock` caches the gate-read; resolves item 7 of TRACE_MATRIX § 5 "Per-line FC tagging via tracing crate"). FC-trace: meta-witness for the 5-step compile loop (Proposal → Lean ground truth → Logging → Capability compilation → ↑H-VPPUT). Trust Root manifest 27 → 28 (corrected A8e5 J4; matches the canonical chain in `genesis_payload.toml` header).
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:12:- A7 (post-A6): heterogeneous-LLM provider plumbing — `src/drivers/llm_proxy.py` ported from v3 with one load-bearing v4 change (per-provider multi-key round-robin: 3 SiliconFlow keys split concurrent traffic across separate rate-limit pools, mitigating V3L-27 single-key N=30 collapse). `scripts/smoke_siliconflow.sh` + `scripts/_smoke_siliconflow.py` probe each of the 3 keys (Qwen/Qwen2.5-7B-Instruct, max_tokens=8) — A7 verified all 3 keys responding 2026-04-26 (1.5–3s latency, 33+1 tokens; round-robin distributes [2,2,2] across 6 calls). New `siliconflow:<model>` provider-prefix syntax in `detect_provider()` for unambiguous routing in `AGENT_MODELS` payloads (Phase D heterogeneous swarms). Memory `reference_siliconflow.md` records SiliconFlow as the heterogeneous-LLM lane (NOT a fallback target). FC-trace: FC1-N7 (δ/AI provider expansion — heterogeneous δ instances across SF catalog enable Phase D meta-loop). Trust Root manifest 28 → 31 (3 entries: `llm_proxy.py` + 2 smoke scripts).
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:13:- A8e14 (post-A8e13b, FIX-THEN-PROCEED after A8 round-12 audit Codex CHALLENGE / Gemini CHALLENGE — both narrow): one substantive run_swarm bug + cascade of stale 37/265 references. **R1 (Codex R12#1+#2+#3)**: A8e13 bumped manifest 37→38 + test count 265→267 but didn't propagate to packet § 2 metric table, § 4 checklist, § 6 Q4.a; both audit runner header `printf` lines (Trust-Rooted, so re-hash too); TRACE_MATRIX § 3 "proxy for 14 rules" → "15 rules + Q1 ENFORCED proxy"; § 6 chain extended with A8e13 → 38; milestones bullet list extended; genesis header chain extended with A8e13=38 + recursive-enforcement note. **R2 (Gemini R12 — substantive run_swarm bug)**: when `search_count[agent] >= search_cap` (default 20), `tools_desc` strips "search" but `search_cache[agent]` keeps being injected into every subsequent prompt's `hits_ref` for the rest of the run — agent reasons from stale data forever. Fix: single `cap_hit` boolean gates BOTH the tool list AND the cache injection (`hits_ref = Vec::new()` when capped). FC-trace: FC1-N7 (δ/AI prompt construction). Trust Root unchanged at 38 (re-hashes only).
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:16:- A8e12 (post-A8e11, harness amplifier per user directive 2026-04-26 中文 "C，更新你的harness，加入判例"): three threads — (1) **case C-076** sediments the false-closure lesson from A8e9 N3 + the multi-section parity miss from A8e8 M4 → A8e9 N1 into a binding precedent (every commit asserting ≥2 distinct fix items requires per-claim `Verified:` proof line); (2) **rule R-020** (`rules/active/R-020_commit_claim_diff_parity.yaml` + inline `.claude/hooks/judge.sh` implementation under R-016 sibling block) WARNs at pre-commit on multi-fix-bundle messages without `Verified:` lines; (3) **self-audit pass** before round-11 to catch documentary drift my own commits keep introducing — fixed `35-entry` → `37-entry` in both audit runner header printfs (post-A8e11 the manifest is 37 but the runners still printed the pre-A8e11 value), `proxy for 14 rules` → `proxy for 15 rules` in genesis_payload.toml comment, and regenerated `cases/MANIFEST.sha256` (50 → 51 entries with C-076) + `rules/MANIFEST.sha256` (14 → 15 entries with R-020) + re-hashed `judge.sh` (R-020 inline addition) + re-hashed both audit runners (35→37 edit). Trust Root manifest unchanged at 37 (no new TR entries; only re-hashes of existing entries — case yaml is covered by cases/MANIFEST.sha256 proxy, rule yaml by rules/MANIFEST.sha256 proxy, judge.sh inline change re-hashes the existing entry). FC-trace: FC3 (read-only / log subgraph — governance instrumentation amplifier; case + rule are Living Harness updates per memory `lesson_to_rule` skill).
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:21:- A8e7 (post-A8e6, STRUCTURAL REWRITE per ultrathink constitutional alignment): rounds 2–6 produced a recurring documentary CHALLENGE pattern. Diagnosis: `A8_EXIT_PACKET` was conflating two opposite temporal modes (stable-snapshot artifact + append-only chronology) in one document — every fix to "current state" generated new staleness in "round-N retrospective" sections. Single-point fixes could not converge. **Resolution**: split the packet using the project's existing constitutional pattern (cf. `constitution.md` + Art. V.3 amendment log; `PREREG_PPUT_CCL_2026-04-26.md` + `PREREG_AMENDMENT_p0_defer_2026-04-25.md`; `TRACE_MATRIX_v0` + v1 + v2). New companion doc `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` carries the append-only chronology of rounds 1–6 + their in-cycle fix bundles. The packet itself becomes the canonical current-state snapshot — no round-N retrospectives, no fix-shipped sections, no cross-round staleness sources. FC-trace: FC3 (read-only / log subgraph — governance artifact restructure) + Art. V.1.2 (ArchitectAI commit authority on non-constitution files). Per case C-075 (DO-178C tool qualification) the new history doc is in Trust Root so the chronology cannot be silently rewritten. Trust Root manifest 34 → 35 (`A8_AUDIT_HISTORY_2026-04-26.md` added). Round 7 dual audit launches from this state.
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:26:- A8e2 (post-A8e, FIX-THEN-PROCEED after A8 round-2 audit Codex CHALLENGE / Gemini CHALLENGE — both convergent): three dual-audit fixes. **G1** (Codex R2#1 + Gemini R2#1) `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` wraps `python3 scripts/test_llm_proxy.py` in a Rust integration test, making the V3L-27 round-robin conformance gate run on every `cargo test --workspace`. **G2** (Codex R2#2) `PREREG_AMENDMENT § 8` audit-requirements paragraph reworded — round-1 fix corrected § 2 but left § 8 still calling the substitution a "strictest plausible bar"; now consistent across the doc. Re-hashed in Trust Root. **G3** (Codex R2#3 + Gemini R2#2) `A8_EXIT_PACKET` updated for round-2 reality: § 2 cumulative-tests table extended with A8e/A8e2 columns, § 4 manifest-clean assertion bumped 30→34, § 5 round-1 risk #5 (no FC1-N12 in swarm verify path) removed (closed by F4), § 6 Q7.a/b numbers updated, round-2 outcome + round-3 fixes section appended; TRACE_MATRIX stale `run_corr_id` symbol row replaced; "in CI" softened to "via Rust wrapper post-A8e2". FC-trace: meta governance + V3L-27 mitigation gating (FC1-N7). Trust Root manifest 33 → 34 (`tests/llm_proxy_python_conformance.rs`).
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:27:- A8e (post-A7, FIX-THEN-PROCEED after A8 round-1 audit Codex CHALLENGE / Gemini VETO): six dual-audit fixes. **F1** (Codex#2 + Gemini Q4) `experiments/minif2f_v4/src/run_id.rs` mints one stable run_id per run, threaded into both `emit_event` and `make_pput` (replaces `run_corr_id` vs internal `run_id` ms drift; oneshot also stops using `oneshot_{problem_file}` placeholder). **F2** (Gemini VETO Q6) `scripts/test_llm_proxy.py` adds 15-test conformance battery — round-robin distributes [2,2,2] across 6 calls, single-key pool degrades to all-zero, two-key pool alternates [2,2]; exercises the V3L-27 mitigation without a live API. (A8e2 G1 wires this into `cargo test --workspace` via a Rust shell-out wrapper — see § 2 below.) **F3** (Codex#4) routing matrix bug fixed — `Qwen/Qwen2.5-7B-Instruct` was misrouting to DashScope because `m.startswith("qwen")` won after the slash check; now slash-form ⇒ siliconflow first, bare-qwen ⇒ dashscope second. **F4** (Codex#3) FC1-N12 emit added to all 3 swarm verify sites (`verify_omega_detailed` × 2 + `verify_partial`), each with verdict + elapsed_ms + path stamp; closes the swarm-side gap in the 5-step compile-loop witness. **F5** (Codex#5) `genesis_payload.toml` header + TRACE_MATRIX milestones reconciled to actual 31 entries (round-1 packet undercounted by 1 — agent_models.rs from A3 was already in TR before A1). **F6** (Codex#1) `PREREG_AMENDMENT § 2` wording corrected — `p_0=0.10` substitution is the LEAST-strict admissible ceiling (max tolerated value for `j-RR ≤ p_0`), NOT the strictest possible substitute as round-1 mistakenly claimed; explicit statistical-implications paragraph added (no Type-I inflation since `j-RR` is descriptive; substitution may be less protective than eventual calibrated `p_0 < 0.10`). FC-trace: FC1-N12 (oracle scope swarm coverage) + FC1-N7 (δ provider routing) + Art. V (amendment governance). Trust Root manifest 31 → 33 (run_id.rs + test_llm_proxy.py).
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:37:| FC ID | v1 Status | v2 Status | Witness test |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:39:| FC1-N1 (Q_t carrier) | ⚠️ | ✅ | `fc1_n1_q_state_carrier_present` |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:40:| FC1-N4 (tape) | ⚠️ | ✅ | `fc1_n4_tape_constructible_with_time_arrow` |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:41:| FC1-N6 (input UniverseSnapshot) | ✅ | ✅ + witness | `fc1_n6_input_universe_snapshot_present` |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:42:| FC1-N7 (δ/AI ResilientLLMClient) | ✅ | ✅ + witness | `fc1_n7_delta_ai_client_type` |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:43:| FC1-N8/N9/N10 (output / q_o / a_o) | ✅ | ✅ + witness | `fc1_n8_n9_n10_output_agent_output_parseable` |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:44:| FC1-N11 (∏p production-path forbidden_pattern) | ⚠️ | ✅ | `fc1_n11_n15_e18_pi_p_zero_preserves_q_t_via_forbidden_pattern` |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:45:| FC1-N13 (wtool bus.append) | ⚠️ | ✅ | `fc1_n13_wtool_bus_append_present` |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:46:| FC1-N15 / E18 (∏p=0 → Q_t preserve) | ⚠️ | ✅ | `fc1_n11_n15_e18_*` (same test) |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:47:| FC2-N20/N27 (mr tick) | ✅ | ✅ + witness | `fc2_n20_n27_tick_mr_present` |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:48:| FC2-N22 (HALT) | ⚠️ | ✅ | `fc2_n22_halt_via_halt_and_settle` |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:49:| FC2-N23 (HaltReason — only OmegaAccepted typed) | ✅ | ✅ + witness | `fc2_n23_event_type_omega_accepted_canonical` |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:50:| FC3-N31 (Wal logs archive) | ⚠️ | ✅ | `fc3_n31_logs_archive_wal_present` |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:51:| FC3-N34 (readonly guard verify_trust_root) | ✅ | ✅ + 3 witnesses | `fc3_n34_*` (3 tests) |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:52:| FC3-N39 (Ledger log) | ✅ | ✅ + witness | `fc3_n39_log_ledger_present_and_appendable` |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:53:| FC3-S3 (readonly subgraph manifest) | (new in v1) | ✅ | `fc3_s3_readonly_subgraph_manifest_size` (>=20 entries assertion) |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:54:| FC3-E14 (boot panic immediate-abort) | (new in v1) | ✅ | `fc3_e14_boot_panic_immediate_abort_documented` |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:59:| Symbol | File | FC anchor | Status |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:61:| `tests/fc_alignment_conformance.rs` (17 witness fns + 9 ignored stubs) | `tests/fc_alignment_conformance.rs` | meta-witness for FC1/FC2/FC3 ↔ symbol mapping; CLAUDE.md "Conformance tests" requirement | ✅ |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:62:| `rules/active/R-014_trust_root_manifest_drift.yaml` | `rules/active/R-014*.yaml` | FC3-S3 readonly subgraph runtime reminder | ✅ |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:63:| `rules/active/R-015_trace_matrix_pub_symbol.yaml` | `rules/active/R-015*.yaml` | CLAUDE.md "每个 src/ pub 符号必须映射到宪法 flowchart 元素" | ✅ |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:65:| `rules/active/R-019_model_snapshot_canonical.yaml` | `rules/active/R-019*.yaml` | FC1-N7 δ/AI canonical identity | ✅ |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:66:| `judge.sh` constitution.md special case | `.claude/hooks/judge.sh:50-67` | FC3-N3 sudo-gate enforcement (closes silent-bypass via `*.md` skip-list) | ✅ |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:67:| `judge.sh` R-016 fc_trace_in_commit | `.claude/hooks/judge.sh:48-56` | FC-first rule (memory feedback_fc_first_problem_handling + case C-074) | ✅ |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:68:| `parse_swarm_condition_n` (A2) | `experiments/minif2f_v4/src/bin/evaluator.rs` | FC2-N16 InitAI orchestration entry — discriminates `oneshot` vs `n<N>` swarm code paths; FC1-N11 ∏p reached only via swarm | ✅ |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:69:| `agent_models::{AGENT_MODELS_ENV_VAR, PHASE_D_HETERO_GATE_ENV_VAR, AgentModelsError, parse_agent_models, expand_agent_models, resolve_agent_models}` (A3) | `experiments/minif2f_v4/src/agent_models.rs` | FC1-N7 δ/AI per-agent assignment; gates Phase B+C single-model invariant (notepad F-2026-04-25-02) | ✅ |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:70:| `RunAggregate::{hit_max_tx, tactic_diversity, verifier_wait_ms}` + `compute_tactic_diversity` (A4) | `experiments/minif2f_v4/src/jsonl_schema.rs` | FC2-N22 HALT decomposition (hit_max_tx splits natural max-tx exhaustion from OMEGA accept and from B7-extra synthetic short-circuit); FC1-N11 ∏p decision diversity (tactic_diversity = distinct/total over append+complete+step proposals); FC1-N12 oracle scope (verifier_wait_ms = cumulative Lean wall-clock per run, ≤ total_wall_time_ms by construction) | ✅ |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:72:| `budget_regime::{BUDGET_REGIME_ENV_VAR, MAX_TRANSACTIONS_ENV_VAR, DEFAULT_MAX_TRANSACTIONS, BudgetRegime, BudgetError, parse_budget_regime, parse_max_transactions, effective_max_tx, resolve_budget}` (A5) | `experiments/minif2f_v4/src/budget_regime.rs` | FC2-N22 HALT decomposition by budget regime — declares which partitioning rule (`total_proposal` / `per_agent` / `token_total` / `wall_clock`) governed the loop bound. Phase A scope = first two regimes implemented; latter two declared startup-fatal `UnimplementedRegime` so a misconfigured run aborts before consuming LLM budget. PREREG_AMENDMENT_p0_defer § 3 condition 3 dependency cleared. | ✅ |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:73:| `RunAggregate::{budget_regime, budget_max_transactions}` + `PputResult::{budget_regime, budget_max_transactions}` (A5) | `experiments/minif2f_v4/src/jsonl_schema.rs` + `experiments/minif2f_v4/src/bin/evaluator.rs` | FC2-N22: every emitted v2 row stamps the regime label + base budget so downstream PPUT analysis can join on the partitioning rule. Loop bound at `run_swarm` startup = `resolve_budget(n_agents).effective_max_tx`; default (env unset) preserves the Phase B baseline `total_proposal × 200` bit-for-bit. 16 unit tests (15 in `budget_regime::tests` + 1 `test_a5_budget_regime_round_trip` in jsonl_schema). | ✅ |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:74:| `fc_trace::{FcId, FC_TRACE_*ENV*, fc_trace_enabled, emit_event, json_str}` (A6) | `experiments/minif2f_v4/src/fc_trace.rs` | meta-witness for FC1 / FC2 / FC3 path coverage. 7-variant `FcId` enum produces stable strings (`FC1-N7` / `FC1-N11` / `FC1-N12` / `FC1-E18` / `FC2-N20` / `FC2-N22` / `FC3-N31`) that Phase D consumers + TRACE_MATRIX rows join on. `FC_TRACE=1` gate cached in `OnceLock` (zero-overhead in production). 6 unit tests (label stability + JSON escape + cold-path no-op). | ✅ |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:75:| `run_id::mint_run_id` correlation key + 9 wired FC events (A6 + A8e F4) | `experiments/minif2f_v4/src/run_id.rs` + `experiments/minif2f_v4/src/bin/evaluator.rs` | A6 introduced a per-run correlation id (originally named `run_corr_id`, now unified into `run_id`) anchoring all events. A8e fix F1 (Codex#2 + Gemini Q4) split this out into a dedicated module + threaded the same id into `make_pput` so FC events and v2 jsonl rows share one identifier (no ms drift). Anchor sites: FC2-N22 synthetic short-circuit / mr tick FC2-N20 / OMEGA full-proof FC2-N22 / OMEGA per-tactic FC2-N22 / natural MaxTxExhausted FC2-N22 (with `budget_regime` payload from A5) / FC1-N12 oneshot verify (A6) / FC1-N12 swarm `verify_omega_detailed` × 2 paths (A8e F4) / FC1-N12 swarm `verify_partial` (A8e F4). End-to-end smoke `tests/fc_trace_smoke.rs` exercises FC_TRACE=1 in a child process (forced because `OnceLock` caches the gate-read). | ✅ |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:77:| `llm_proxy.py` v4 (multi-key round-robin) + `detect_provider` `siliconflow:` prefix (A7) | `src/drivers/llm_proxy.py` | FC1-N7 δ/AI provider expansion — three SiliconFlow keys form a 3-element round-robin pool keyed on `_per_key_requests[provider]`. Phase D heterogeneous swarms can address SF models via `AGENT_MODELS=siliconflow:Qwen/Qwen2.5-7B-Instruct,...`. Mitigates V3L-27 (case C-027) single-key N=30 401/429 collapse documented in `cases/V3_LESSONS.md`. | ✅ |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:100:| C-072 Veto-AI scope narrowing | Art. V.1.3 | manual via dual audit; future FC3-N32 runtime |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:102:| C-074 FC-first problem handling | All FC + Alignment Standard | R-016 (WARN on git commit without FC-trace) |
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:110:4. **R-016 fc_trace_in_commit upgrade** — currently WARN-level. If post-Phase-D evidence shows FC-trace discipline still slipping, promote to BLOCK-level.
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:112:6. **FC2-N23 HaltReason full taxonomy as Rust enum** — currently only OmegaAccepted is typed; other 4 variants live as jsonl strings. Phase C+ Soft Law mode work may force this typing.
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:113:7. ~~**Per-line FC tagging via tracing crate** — Plan agent's recommendation in N-experiments brainstorm. Phase A6 deferred; will land before Phase B (homogeneous experiments).~~ **CLOSED**: A6 (`89994c7`) shipped `fc_trace.rs` + 6 production emit sites (5 in `run_swarm` + 1 in `run_oneshot`); A8e fix F4 added 3 more in `run_swarm` (`verify_omega_detailed` alone + tape+payload retry, plus `verify_partial`), bringing the post-A8e total to **9 wired anchor sites**. Implementation chose pure stdlib over the `tracing` crate to avoid a new dep tree; the macro surface (`emit_event` + `FcId` enum) was kept small so Phase D+ can swap to a real `tracing-subscriber` bridge locally.
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:154:- `handover/alignment/FC_ELEMENTS_2026-04-22.md` (canonical FC node IDs)
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:155:- `handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md` (FC3-E14 vs FC2-N22 distinction)
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md:159:- `~/.claude/.../memory/feedback_fc_first_problem_handling.md` (FC-first rule memory)
handover/alignment/CODE_CANDIDATES_2026-04-22.md:10:| FC1-N1 | `Q_t = ⟨q_t, HEAD_t, tape_t⟩` | `TuringBus::q_state`<br>`Tape::time_arrow`<br>`Kernel::tape` | `src/bus.rs:70`<br>`src/ledger.rs:146`<br>`src/kernel.rs:20` | `Y`<br>`N`<br>`N` | `medium` | Runtime `Q_t` is distributed across bus state, the tape tail used as HEAD, and the kernel-held tape rather than a single `Q_t` struct. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:11:| FC1-N2 | `q_t` | `QState`<br>`TuringBus::q_state` | `src/bus.rs:53`<br>`src/bus.rs:70` | `Y`<br>`Y` | `high` | Phase 8.E makes `q_t` explicit as `QState::{Running,Halted{reason}}` stored on the bus. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:12:| FC1-N3 | `HEAD_t` | `Tape::time_arrow`<br>`bus.kernel.tape.time_arrow().last()` | `src/ledger.rs:146`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:750`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:950`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:983` | `N`<br>`N`<br>`N`<br>`N` | `high` | HEAD is implemented implicitly as the last node id on the tape time arrow, exactly as the grep hint suggested. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:13:| FC1-N4 | `tape_t` | `Tape`<br>`Kernel::tape`<br>`UniverseSnapshot::tape` | `src/ledger.rs:44`<br>`src/kernel.rs:20`<br>`src/sdk/snapshot.rs:23` | `Y`<br>`N`<br>`N` | `high` | The append-only DAG tape is the concrete world-state carrier in kernel storage and agent snapshots. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:14:| FC1-N5 | `rtool` | `ReadTool::project`<br>`DefaultReadTool::project`<br>`TuringBus::snapshot` | `src/sdk/read_tool.rs:24`<br>`src/sdk/read_tool.rs:37`<br>`src/bus.rs:802` | `Y`<br>`N`<br>`Y` | `medium` | Phase Z added an explicit `rtool` trait, but the current evaluator still reads state by calling `bus.snapshot()` directly. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:15:| FC1-N6 | `input = ⟨q_i, s_i⟩` | `UniverseSnapshot`<br>`build_agent_prompt`<br>`let prompt = build_agent_prompt(...)`<br>`GenerateRequest` | `src/sdk/snapshot.rs:22`<br>`src/sdk/prompt.rs:15`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:619`<br>`src/drivers/llm_http.rs:14` | `Y`<br>`Y`<br>`N`<br>`Y` | `high` | What the agent sees is the frozen snapshot rendered into a prompt and shipped as the LLM request message. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:16:| FC1-N7 | `δ / AI` | `ResilientLLMClient::generate`<br>`match client.generate(&request).await` | `src/drivers/llm_http.rs:84`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:211`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:644` | `Y`<br>`N`<br>`N` | `high` | These are the actual LLM call surfaces that map prompt input to model output in oneshot and swarm runs. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:17:| FC1-N8 | `output = ⟨q_o, a_o⟩` | `AgentOutput`<br>`parse_agent_output` | `src/sdk/protocol.rs:40`<br>`src/sdk/protocol.rs:148` | `Y`<br>`Y` | `high` | The protocol layer explicitly models the constitution's wrapped output pair and parses raw model text into it. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:18:| FC1-N9 | `q_o` | `AgentOutput::q_delta` | `src/sdk/protocol.rs:42` | `Y` | `high` | `q_delta` is the optional next-state hint emitted by the agent. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:19:| FC1-N10 | `a_o` | `AgentAction`<br>`AgentOutput::action` | `src/sdk/protocol.rs:13`<br>`src/sdk/protocol.rs:44` | `Y`<br>`Y` | `high` | The concrete emitted action is the parsed tool invocation payload. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:20:| FC1-N11 | `∏p predicates` | `TuringBus::evaluate_predicates`<br>`Predicate` | `src/bus.rs:148`<br>`src/sdk/predicate.rs:88` | `Y`<br>`Y` | `medium` | The explicit product-evaluation API exists, but current runtime gating is still split across this Phase Z surface and legacy bus/oracle checks. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:21:| FC1-N12 | `individual p predicates` | `ForbiddenPatternPredicate`<br>`SorryPredicate`<br>`PayloadSizePredicate`<br>`Lean4Oracle::verify_partial`<br>`Lean4Oracle::verify_omega_detailed` | `src/sdk/predicate.rs:106`<br>`src/sdk/predicate.rs:124`<br>`src/sdk/predicate.rs:139`<br>`experiments/minif2f_v4/src/lean4_oracle.rs:248`<br>`experiments/minif2f_v4/src/lean4_oracle.rs:183` | `Y`<br>`Y`<br>`Y`<br>`Y`<br>`Y` | `medium` | Concrete predicate logic exists, but it is split between Phase Z predicate structs and the still-separate Lean oracle verifier. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:22:| FC1-N13 | `wtool` | `WriteTool::write`<br>`DefaultWriteTool::write`<br>`TuringBus::append`<br>`TuringBus::append_oracle_accepted` | `src/sdk/write_tool.rs:29`<br>`src/sdk/write_tool.rs:84`<br>`src/bus.rs:324`<br>`src/bus.rs:347` | `Y`<br>`N`<br>`Y`<br>`Y` | `medium` | The explicit write-tool contract exists, but the current harness usually enters the write path by calling bus append methods directly. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:23:| FC1-N14 | `Q_{t+1}` on successful write | `TuringBus::append_internal`<br>`Kernel::append`<br>`TuringBus::halt_with_reason` | `src/bus.rs:421`<br>`src/kernel.rs:108`<br>`src/bus.rs:207` | `N`<br>`Y`<br>`Y` | `medium` | Successful writes update tape and ledger in `append_internal`, and terminal writes additionally advance `q_state` through `halt_with_reason`. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:24:| FC1-N15 | `Q_t` branch on `∏p = 0` | `PartialVerdict::Reject`<br>`BusResult::Vetoed` | `experiments/minif2f_v4/src/lean4_oracle.rs:328`<br>`src/bus.rs:111` | `Y`<br>`N` | `high` | Rejected steps and vetoed appends explicitly encode the no-write branch where state is preserved. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:25:| FC2-N16 | `InitAI` | `run_swarm`<br>`run_oneshot` | `experiments/minif2f_v4/src/bin/evaluator.rs:335`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:182` | `Y`<br>`Y` | `medium` | There is no dedicated `InitAI` type, but these harness entrypoints perform the one-time orchestration the diagram attributes to initialization. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:26:| FC2-N17 | `human architect` | `—` | `constitution.md:448` | `N` | `—` | ❌ MISSING: this is a manual author/spec role in the diagram, not a runtime Rust symbol. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:27:| FC2-N18 | `law / ground truth` | `—` | `constitution.md:449` | `N` | `—` | ❌ MISSING: the ground-truth law is the markdown constitution, not a Rust object owned by the runtime. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:28:| FC2-N19 | `initAI --once→ predicates` | `TuringBus::register_predicate` `(API only)` | `src/bus.rs:136` | `Y` | `—` | ❌ MISSING: the registration hook exists, but no boot/init caller in `run_swarm`, `run_oneshot`, or `TuringBus::init` wires predicates once at startup. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:29:| FC2-N20 | `initAI --once→ mr` | `let tick_interval: usize`<br>`TuringBus::emit_mr_tick_node` | `experiments/minif2f_v4/src/bin/evaluator.rs:459`<br>`src/bus.rs:385` | `N`<br>`Y` | `high` | Swarm initialization configures the tick cadence once, and the later tick path reduces through `emit_mr_tick_node`. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:30:| FC2-N21 | `initAI --once→ Q0` | `Kernel::new`<br>`TuringBus::new`<br>`TuringBus::init` | `src/kernel.rs:50`<br>`src/bus.rs:115`<br>`src/bus.rs:299` | `N`<br>`N`<br>`Y` | `high` | Kernel construction, bus construction, and `init()` collectively materialize the initial live machine state before iteration begins. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:31:| FC2-N22 | `HALT` | `QState::Halted`<br>`TuringBus::halt_with_reason`<br>`TuringBus::halt_and_settle` | `src/bus.rs:55`<br>`src/bus.rs:207`<br>`src/bus.rs:581` | `N`<br>`Y`<br>`Y` | `high` | The terminal node is an explicit halted state plus the halt path that emits durable halt events and final settlement. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:32:| FC2-N23 | `HaltReason variants` | `HaltReason`<br>`EventType::Halt { reason }`<br>`extract_halt_reason` | `src/ledger.rs:230`<br>`src/ledger.rs:273`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:1116` | `Y`<br>`Y`<br>`Y` | `high` | The five report-standard halt causes are encoded in Rust and then recovered from the ledger for reporting. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:33:| FC2-N24 | `clock` | `TuringBus::clock`<br>`for tx in 0..max_transactions`<br>`let tick_interval: usize` | `src/bus.rs:66`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:485`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:459` | `N`<br>`N`<br>`N` | `high` | Scheduling comes from the transaction loop and tick interval, while the bus tracks a monotone committed clock counter. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:34:| FC2-N25 | `mr` | `let mr_summary = format!(...)`<br>`TuringBus::emit_mr_tick_node` | `experiments/minif2f_v4/src/bin/evaluator.rs:504`<br>`src/bus.rs:385` | `N`<br>`Y` | `medium` | Map-reduce is implemented inline in the swarm tick block rather than as a dedicated `mr` function. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:35:| FC2-N26 | `mr --map→ tape0` | `bus.kernel.tape.time_arrow().len()`<br>`bus.kernel.market_ticker(5)` | `experiments/minif2f_v4/src/bin/evaluator.rs:488`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:490` | `N`<br>`N` | `medium` | The map phase reads current tape and market state to build the tick summary before any write occurs. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:36:| FC2-N27 | `mr --reduce→ tape1` | `TuringBus::emit_mr_tick_node` | `src/bus.rs:385` | `Y` | `high` | The reduce output is persisted as an `__mr_tick__` tape node, matching the mermaid's tape1 edge. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:37:| FC2-N28 | `tools_other` | `WriteTool::write_with_tools`<br>`TuringBus::tools`<br>`bus.mount_tool(...)` | `src/sdk/write_tool.rs:57`<br>`src/bus.rs:64`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:407`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:414`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:415`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:419` | `Y`<br>`N`<br>`N`<br>`N`<br>`N`<br>`N` | `high` | Ancillary tools are carried as mounted `TuringTool`s on the bus, and `write_with_tools` names that exact contract explicitly. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:38:| FC3-N29 | `boot` | `async fn main`<br>`TuringBus::boot` | `experiments/minif2f_v4/src/bin/evaluator.rs:88`<br>`src/bus.rs:286` | `N`<br>`Y` | `medium` | Process entry is `main`, and the bus exposes a boot hook for mounted tools even though the current harness goes straight to `init()`. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:39:| FC3-N30 | `constitution file` | `—` | `constitution.md:683` | `N` | `—` | ❌ MISSING: the law source is a markdown document, not a readonly Rust-owned runtime object. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:40:| FC3-N31 | `logs archive` | `TuringBus::with_wal_path`<br>`Wal::replay`<br>`Wal::write_event` | `src/bus.rs:227`<br>`src/wal.rs:70`<br>`src/wal.rs:54` | `Y`<br>`Y`<br>`N` | `high` | Persisted history is the WAL-backed archive that replays old state and appends new ledger events durably. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:41:| FC3-N32 | `JudgeAI` | `—` | `constitution.md:686` | `N` | `—` | ❌ MISSING: per the task note, JudgeAI is an external/manual audit role deferred beyond current runtime code. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:42:| FC3-N33 | `ArchitectAI` | `—` | `constitution.md:687` | `N` | `—` | ❌ MISSING: per the task note, ArchitectAI is a manual code-improvement role, not a runtime Rust component. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:43:| FC3-N34 | `readonly guard on {constitution, logs}` | `—` | `constitution.md:682`<br>`src/wal.rs:70` | `N`<br>`Y` | `—` | ❌ MISSING: WAL and ledger are append-only, but no Rust guard makes both constitution and archived logs a single readonly trust root. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:44:| FC3-N35 | `anti-oreo: top → agents → tools` | `TuringBus::evaluate_predicates`<br>`let agent_ids`<br>`TuringTool` | `src/bus.rs:148`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:431`<br>`src/sdk/tool.rs:38` | `Y`<br>`N`<br>`Y` | `medium` | The chain exists as predicate gating over a swarm of agent ids acting through mounted tools, but there is no single anti-oreo coordinator type. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:45:| FC3-N36 | `agents` | `let agent_ids`<br>`let agent_id = &agent_ids[agent_idx]` | `experiments/minif2f_v4/src/bin/evaluator.rs:431`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:577` | `N`<br>`N` | `high` | Swarm agents are runtime identities allocated at init and selected round-robin on each transaction. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:46:| FC3-N37 | `tools` | `TuringTool`<br>`WalletTool`<br>`SearchTool`<br>`LibrarianTool`<br>`Lean4Oracle` | `src/sdk/tool.rs:38`<br>`src/sdk/tools/wallet.rs:24`<br>`src/sdk/tools/search.rs:11`<br>`src/sdk/tools/librarian.rs:17`<br>`experiments/minif2f_v4/src/lean4_oracle.rs:42` | `Y`<br>`Y`<br>`Y`<br>`Y`<br>`N` | `high` | The tool layer is the `TuringTool` trait plus the mounted concrete implementations the swarm can use. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:47:| FC3-N38 | `tape Q` | `Tape`<br>`Kernel::tape` | `src/ledger.rs:44`<br>`src/kernel.rs:20` | `Y`<br>`N` | `high` | System-level `Q` reuses the same append-only tape structure identified in FC1. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:48:| FC3-N39 | `log` | `Ledger`<br>`LedgerEvent`<br>`Ledger::append` | `src/ledger.rs:332`<br>`src/ledger.rs:296`<br>`src/ledger.rs:347` | `Y`<br>`Y`<br>`Y` | `high` | The runtime log is the hash-chained ledger event stream, separate from the DAG tape. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:49:| FC3-N40 | `logs --> feedback --> ArchitectAI` | `—` | `constitution.md:710` | `N` | `—` | ❌ MISSING: runtime feedback exists for agents and log compression, but no Rust `ArchitectAI` consumes archived logs in an automated improvement loop. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:50:| FC3-N41 | `init --> error --> re-init --> boot` | `TuringBus::with_wal_path`<br>`std::process::exit(2)` | `src/bus.rs:227`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:278`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:329`<br>`experiments/minif2f_v4/src/bin/evaluator.rs:388` | `Y`<br>`N`<br>`N`<br>`N` | `—` | ❌ MISSING: Rust can resume from WAL, but on init/LLM/WAL failure it exits and expects an external batch runner to retry; no in-process re-init loop was found. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:51:| FC3-N42 | `constitution --abide→ JudgeAI + ArchitectAI` | `—` | `constitution.md:712` | `N` | `—` | ❌ MISSING: compliance is specified in docs/manual audit, not enforced by a runtime JudgeAI/ArchitectAI pair. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:52:| FC3-N43 | `JudgeAI --veto→ ArchitectAI` | `—` | `constitution.md:713`<br>`CLAUDE.md:19` | `N`<br>`N` | `—` | ❌ MISSING: `VETO > CHALLENGE > PASS` is documented policy, not a runtime Rust veto channel between two agents. |
handover/alignment/CODE_CANDIDATES_2026-04-22.md:63:- `Kernel::resolve_bounty` — `src/kernel.rs:83` — cross-agent bounty payout exists outside the FC1/FC2/FC3 diagrams.
handover/alignment/CODE_CANDIDATES_2026-04-22.md:64:- `TuringBus::bus_classify` — `src/bus.rs:706` — bounded error-label normalization is an implementation detail not represented in the flowcharts.
handover/alignment/CODE_CANDIDATES_2026-04-22.md:67:- `LibrarianTool::build_compression_prompt` — `src/sdk/tools/librarian.rs:48` — memory-compression plumbing is outside the audited flowchart surfaces.
handover/alignment/CODE_CANDIDATES_2026-04-22.md:68:- `LibrarianTool::post_to_board` — `src/sdk/tools/librarian.rs:101` — the shared team board is a side channel not shown in FC1/FC2/FC3.
handover/alignment/CODE_CANDIDATES_2026-04-22.md:69:- `persist_proof_artifact` — `experiments/minif2f_v4/src/bin/evaluator.rs:1143` — proof-file archival is a runtime artifact sink, not a flowchart node.
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:4:**Trigger**: Phase B B7 (Trust Root + Boot freeze) shipped runtime code that (a) implements the Phase 11+ deferred FC3-N34 row and (b) introduces new files to the readonly base. Per CLAUDE.md "每个 src/ pub 符号必须映射到宪法 flowchart 元素", v1 documents the new mappings before downstream work piles on top.
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:6:> **2026-04-25 amendment** (post-constitution V.3 修订日志, mid-session): the constitution renamed **JudgeAI → Veto-AI** (Art. V.1.3 + FC3 mermaid `judgeAI` → `vetoAI`). All TRACE_MATRIX v0 references to `JudgeAI` / `judgeAI` (rows FC3-N32, FC3-N42, FC3-N43, FC3-E4/E5/E15, edge `FC3-Veto`) should be read forward-compatibly as Veto-AI / vetoAI. v0 + `FC_ELEMENTS_2026-04-22.md` are immutable audit-trail baselines and are NOT backfilled. Constitutional clarifications also added at V.1.1 (sudo scope = constitution.md only) + V.1.2 (ArchitectAI commit authority on non-constitution files); these reframe how Trust Root is *enforced* (Veto-AI proposal gate + Boot manifest runtime gate) without changing what's *in* the manifest.
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:17:| FC Element ID | v0 Status | v1 Status | Justification |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:19:| FC3-N34 (`readonly guard on {constitution, logs}`) | 📅 Phase 11+: add FS-level readonly check at init | ✅ implemented | B7 ships `turingosv4::boot::verify_trust_root` (`src/boot.rs:62`) — SHA-256 manifest verification at Boot. `src/main.rs` calls it as the first action and panics with `TRUST_ROOT_TAMPERED` on mismatch. Mechanism = content-hash check rather than FS chmod, but the constitutional intent (readonly base cannot be silently mutated between runs) is honored. |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:25:## § 2. New code symbols added in B7 (FC anchors)
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:27:| Symbol | File:Line | FC Anchor | DocComment | Status |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:29:| `boot::verify_trust_root` | `src/boot.rs:62` | FC3-N34 | Y (line 56-61) | ✅ |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:30:| `boot::parse_trust_root_section` | `src/boot.rs:91` | FC3-N34 (helper) | Y (line 86-90) | ✅ |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:31:| `boot::TrustRootError` | `src/boot.rs:24` | FC3-N34 (failure variants) | Y (line 19-23) | ✅ |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:32:| `fn main` (Trust Root verify call site) | `src/main.rs:11` | FC3-N29 (`boot`) + FC3-E14 (`error → re-init → boot`) | Y (line 3-10) | ✅ |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:33:| `rollback_sim::should_simulate_rollback` | `experiments/minif2f_v4/src/rollback_sim.rs:48` | FC1-E18 (∏p=0 → Q_t) repeated · FC2-N22 HALT (existing `MaxTxExhausted` variant) — **outcome-equivalent only on (problem, seed, solved)** | Y (file header + fn doc) | ⚠️ partial (audit-fix 2026-04-25) |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:34:| `rollback_sim::rollback_simulation_enabled` | `experiments/minif2f_v4/src/rollback_sim.rs:39` | same FC1-E18 + FC2-N22 anchor (env-var read for the predicate); narrow equivalence per above | Y | ⚠️ partial |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:37:| `evaluator.rs` short-circuit at line 503-518 | `experiments/minif2f_v4/src/bin/evaluator.rs:503` | FC1-E18 + FC2-N22 (call-site of the synthetic predicate); **path-equivalent NOT verified — bus's evaluate_predicates is not exercised in calibration treatment** | Y (block comment) | ⚠️ partial |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:39:Internal helpers (`has_section`, `strip_comment`, `unquote`, `hex_lower`) are private — no FC backlink required (per CLAUDE.md scoping to `pub` symbols).
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:43:## § 3. New `readonly` extensions (FC3-S3 subgraph membership change)
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:45:The constitutional FC3-S3 `readonly` subgraph contains FC3-N3 (`constitution as ground truth`) and FC3-N4 (`logs archive as ground truth`). PREREG § 1.8 (round-4 dual-audit PASS/PASS) extended this base for the PPUT-CCL experiment. Each addition is a research-protocol orphan with explicit constitutional justification (case-law / measurement-fidelity / pre-registration commitment).
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:49:| `src/kernel.rs` | FC3-N10 (`tape Q`) source — kernel topology immutability is a Law-1 invariant (Art. I.1) |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:50:| `src/wal.rs` | FC3-N11 (`log`) implementation — append-only WAL is the constitutional logs-archive surface |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:51:| `src/bus.rs` | FC1-N11/N13/N14 (`∏p`, `wtool`, `Q_{t+1}`) implementation — execution semantics that PPUT measures rest on |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:52:| `src/drivers/llm_http.rs` | FC1-N7 (`δ / AI`) — cost source-of-truth (prompt_tokens / completion_tokens). Tampering with this defeats every C_i count |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:54:| `experiments/minif2f_v4/src/lean4_oracle.rs` | FC1-N12 (∏p ground-truth oracle) — Lean ground-truth cannot drift mid-experiment |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:61:| `constitution.md` | FC3-N3 (constitution as ground truth) — direct |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:62:| `cases/MANIFEST.sha256` | case-law glob hashed once into Trust Root; case law is constitutional precedent (CLAUDE.md "Common Law"), so this is FC3-N3 extension via secondary manifest |
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:88:## § 5. Boot panic ↔ FC mapping
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:90:`src/main.rs:13` panics with `TRUST_ROOT_TAMPERED` on `verify_trust_root` failure. This is **not** a FC2-N22 HALT path:
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:92:- FC2-N22 HALT requires the kernel/bus to be initialized (HaltReason variants are emitted by `TuringBus::halt_with_reason`).
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:94:- Closer match: FC3-E14 (`init → error → re-init → boot`). Boot-panic is the immediate-abort variant; the surrounding harness (batch runner, supervisord, shell wrapper) is the "re-init" actor.
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:103:- ✅ count: **15 → 16** (+1: FC3-N34 promoted from 📅)
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:118:1. **FC3-N41** (`init → error → re-init → boot`, automated retry) is still 📅 Phase 11+. B7's panic on TRUST_ROOT_TAMPERED is the *immediate-abort* leaf of FC3-E14; an in-process retry loop is the future work.
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md:119:2. ~~**TRACE_MATRIX of B7-extra (p_0 calibration toggle)**~~ — landed. Final implementation differs slightly from the original sketch in this section: the constitutional `bus.register_predicate(...)` API does not currently exist on `main` (it lives on the unmerged `phase-z-wtool-tools` branch — TRACE_MATRIX_v0 row FC1-N11 references it aspirationally). Rather than scope-creep B7-extra into reviving Phase Z, the synthetic predicate is implemented at the evaluator layer in `rollback_sim.rs` with an explicit short-circuit at the threshold tx. The constitutional anchor (FC1-E18 ∏p=0 → Q_t repeated, then FC2-N22 HALT via existing `MaxTxExhausted`) is unchanged; only the abstraction depth differs. Listed under § 2 above as ✅ entries.
handover/alignment/OBS_WHITEPAPER_V2_INITAI_PLACEHOLDER_2026-04-27.md:32:- TRACE_MATRIX FC node IDs
handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md:51:Constitution flowchart modifications require Phase Z′ 6-stage rerun
handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md:87:- `TRACE_MATRIX_*` row text (a row MAY backlink, but the row's flowchart-id text stays canonical)
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:12:Witness that every `✅` TRACE_MATRIX alignment row fires in a single real-problem run. Solve not required; the test is about whether the flowchart topology is actually exercised by runtime.
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:16:10 `✅`-labeled matrix rows actively tested via runtime trace; all fired except HALT (FC2-N22) which requires the full 60-tx run (our 600s external timeout cut short at tx=50).
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:20:| FC1-N1 | Q_t triple | ✅ | tape=13 nodes at tx=50, 4 markets, q_state=Running |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:21:| FC1-N2 | q_t (QState) | ✅ | bus running throughout |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:22:| FC1-N3 | HEAD_t | ✅ | parent_id used in each partial-OK (step+tx_N_by_Agent_M) |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:23:| FC1-N4 | tape_t | ✅ | tape grew 0→13 over 50 tx |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:24:| FC1-N5 | rtool | ✅ | snapshot rendered into prompt each tx |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:25:| FC1-N6 | input ⟨q_i, s_i⟩ | ✅ | `[swarm/n8] Agent_N:skill_M:t=...` header per tick |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:26:| FC1-N7 | δ / AI | ✅ | 50 LLM rounds completed |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:27:| FC1-N8 | output ⟨q_o, a_o⟩ | ✅ | 50 AgentOutput parses (no parse failures logged) |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:28:| FC1-N11 | ∏p product | ✅ | 50 reject + 4 partial-OK; AND-semantics visible |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:29:| FC1-N12 | p predicates | ✅ | `[oracle/partial] rejected pre-Lean: Forbidden bare tactic: 'decide'` × 4 |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:30:| FC1-N14 | Q_{t+1} success | ✅ | 4× `step+tx_N_by_Agent_M partial OK` wtool writes |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:31:| FC1-N15 | Q_t=Q_t reject | ✅ | 46 Lean rejects + 4 forbidden rejects, tape preserved |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:32:| FC2-N19 | init→predicates | ✅ | register_predicate fired at bus init (Stage 3 wiring) |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:33:| FC2-N21 | init→Q0 | ✅ | Kernel::new materialized empty initial state |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:34:| FC2-N22 | HALT | ❌ (timing) | run hit external 600s timeout before tx=60 cap; would trigger MaxTxExhausted at tx=60 |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:35:| FC2-N24 | clock | ✅ | tx 0→50 monotone |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:36:| FC2-N25 | mr | ✅ | 10 tick events at tx=5,10,...,50 |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:37:| FC2-N26 | mr--map→tape0 | ✅ | tick reads tape.time_arrow().len() + market_ticker |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:38:| FC2-N27 | mr--reduce→tape1 | ✅ | emit_mr_tick_node called silently on success; tape node count increment between ticks matches |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:39:| FC2-N28 | tools_other | ✅ | Wallet+Search+Librarian+Lean4Oracle all mounted |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:40:| FC3-N36 | agents | ✅ | 8 agents round-robin (Agent_0 through Agent_7) |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:41:| FC3-N37 | tools | ✅ | mounted tool count > 0 |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:42:| FC3-N38 | tape Q | ✅ | same tape as FC1-N4 |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:43:| FC3-N39 | log (ledger) | ✅ | events accumulated (verified in separate unit test) |
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md:45:## Unfired: HALT (FC2-N22)
handover/evidence/tb_15_markov_capsule_2026-05-03/MARKOV_TB-15_2026-05-03.json:37:  "constitution_hash": [
handover/evidence/tb_15_markov_capsule_2026-05-03/MARKOV_TB-15_2026-05-03.json:71:  "l4_root": [
handover/evidence/tb_15_markov_capsule_2026-05-03/MARKOV_TB-15_2026-05-03.json:105:  "l4e_root": [
handover/evidence/tb_15_markov_capsule_2026-05-03/MARKOV_TB-15_2026-05-03.json:139:  "cas_root": [
handover/alignment/OBS_CONSTITUTION_MERMAID_FENCE_2026-04-22.md:1:# Observation: FC-2 / FC-3 mermaid fence missing in `constitution.md`
handover/alignment/OBS_CONSTITUTION_MERMAID_FENCE_2026-04-22.md:5:**Discovered by**: Stage 1 flowchart extractor agent (Phase Z')
handover/alignment/OBS_CONSTITUTION_MERMAID_FENCE_2026-04-22.md:14:- Line 325: ` ```mermaid ` (FC-1 opener) ✓
handover/alignment/OBS_CONSTITUTION_MERMAID_FENCE_2026-04-22.md:15:- Line 379: ` ``` ` (FC-1 closer) ✓
handover/alignment/OBS_CONSTITUTION_MERMAID_FENCE_2026-04-22.md:16:- Line 530: ` ``` ` (FC-2 closer) ← **opener missing**
handover/alignment/OBS_CONSTITUTION_MERMAID_FENCE_2026-04-22.md:17:- Line 714: ` ``` ` (FC-3 closer) ← **opener missing**
handover/alignment/OBS_CONSTITUTION_MERMAID_FENCE_2026-04-22.md:19:FC-2 starts at line 441 with `    flowchart TD` (4-space indent) but has no preceding ` ```mermaid ` fence.
handover/alignment/OBS_CONSTITUTION_MERMAID_FENCE_2026-04-22.md:20:FC-3 starts at line 670 with `    graph TB` (4-space indent) but also has no preceding fence.
handover/alignment/OBS_CONSTITUTION_MERMAID_FENCE_2026-04-22.md:24:Only FC-1 renders correctly on GitHub / Notion / any markdown viewer. FC-2 and FC-3 appear as plain (indented) text with only an unbalanced closing backtick.
handover/alignment/OBS_CONSTITUTION_MERMAID_FENCE_2026-04-22.md:28:Zero. Stage 1 agent parsed all three flowcharts by treating the indented text as if properly fenced. Element extraction + code mapping proceeded successfully. The TRACE_MATRIX is not affected.
handover/evidence/tb_15_markov_capsule_2026-05-03/README.md:27:constitution_hash     = eec695459c71fbef3685583485deb431fe3b561657b2f285b7c5e7e220e59e03
handover/evidence/tb_15_markov_capsule_2026-05-03/README.md:30:l4_root               = Hash::ZERO  (v0 placeholder; future TB wires from chain head)
handover/evidence/tb_15_markov_capsule_2026-05-03/README.md:31:l4e_root              = Hash::ZERO  (same)
handover/evidence/tb_15_markov_capsule_2026-05-03/README.md:32:cas_root              = Hash::ZERO  (same)
handover/evidence/tb_15_markov_capsule_2026-05-03/README.md:50:#2 markov_capsule_references_constitution_hash    GREEN
handover/evidence/tb_15_markov_capsule_2026-05-03/README.md:51:   (Atom 5 — sha256(constitution.md) == capsule.constitution_hash; SG-15.7)
handover/evidence/tb_15_markov_capsule_2026-05-03/README.md:76:| SG-15.7 | Markov capsule references constitution hash and flowchart hashes | GREEN — `MarkovEvidenceCapsule.constitution_hash` field; `with_constitution_hash` constructor; halt-trigger #2 |
handover/evidence/tb_15_markov_capsule_2026-05-03/README.md:79:| G-15.10 | FC1-N32 + FC1-N33 + FC2-N30 + FC3-N43 each have ≥1 witness | GREEN — `tests/fc_alignment_conformance.rs` has 4 witness tests added (`fc1_n32_*`, `fc1_n33_*`, `fc2_n30_*`, `fc3_n43_*`) |
handover/alignment/WHITEPAPER_v2_TACTICAL_ALIGNMENT_2026-04-27.md:15:- **No flowchart edit** is performed. No FC element renumbering. No TRACE_MATRIX rewrite. No conformance test rebuild.
handover/alignment/WHITEPAPER_v2_TACTICAL_ALIGNMENT_2026-04-27.md:25:- TRACE_MATRIX_v3 → v4 rewrite (FC elements added/renumbered)
handover/alignment/WHITEPAPER_v2_TACTICAL_ALIGNMENT_2026-04-27.md:26:- `tests/fc_alignment_conformance.rs` rebuild for new FC nodes
handover/alignment/WHITEPAPER_v2_TACTICAL_ALIGNMENT_2026-04-27.md:27:- Dual external audit on every src/ pub symbol's flowchart re-mapping
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:44:| Art V.1.1 — Constitution 唯一基准 | N | `genesis_payload::constitution_root::constitution_hash` + `boot::verify_constitution_root` | `tests/genesis_constitution_root_verify.rs` | CO1.0.4 |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:187:| 30 | `pub enum TrustRootError` | FC3-N34: failure variants of the readonly-guard verification |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:188:| 62 | `pub fn verify_trust_root` | FC3-N34: implementation of the constitutional `readonly` |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:189:| 246 | `fn` | FC3-N34 + case C-075: child-manifest recursion |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:190:| 281 | `pub fn parse_trust_root_section` | FC3-N34: helper for `verify_trust_root` — exposed because |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:196:| 5 | `pub mod system_keypair` | FC1-Sig+FC3-Sig: system runtime signature key lifecycle |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:197:| 8 | `pub mod transition_ledger` | FC2-Append + WP § 5.L4: L4 transition ledger (CO1.7 type skeleton) |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:203:| 40 | `pub struct SystemEpoch` | FC1-Sig+FC3-Sig: system signature epoch identifier |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:204:| 45 | `pub const fn` | FC1-Sig+FC3-Sig: construct a system signature epoch |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:205:| 50 | `pub const fn` | FC1-Sig+FC3-Sig: expose the numeric epoch for canonical encoding |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:206:| 62 | `pub struct SystemPublicKey` | FC1-Sig+FC3-Sig: ed25519 public key pinned by epoch |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:207:| 67 | `pub const fn` | FC1-Sig+FC3-Sig: construct a system public key from raw ed25519 bytes |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:208:| 72 | `pub const fn` | FC1-Sig+FC3-Sig: expose raw public key bytes for pinning and verification |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:209:| 77 | `pub fn fingerprint_sha256` | FC3-Sig: stable SHA-256 fingerprint for audit logs and rotation records |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:210:| 83 | `pub struct SystemSignature` | FC1-Sig+FC3-Sig: ed25519 detached signature over a canonical system message digest |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:211:| 140 | `pub const fn` | FC1-Sig+FC3-Sig: construct a detached system signature from raw ed25519 bytes |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:212:| 145 | `pub const fn` | FC1-Sig+FC3-Sig: expose raw signature bytes for tape serialization |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:213:| 151 | `pub struct RejectedAttemptSummary` | FC1-Sig: typed rejection summary stamped by the predicate runner |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:214:| 161 | `pub fn new` | FC1-Sig: construct a typed rejected-attempt summary, never a free-form sign blob |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:215:| 184 | `pub struct EpochRotationProof` | FC3-Sig: typed continuity statement for system key rotation |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:216:| 195 | `pub const fn` | FC3-Sig: construct a typed epoch-rotation continuity proof |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:217:| 212 | `pub const fn` | FC3-Sig: old signing epoch certified by the rotation proof |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:218:| 217 | `pub const fn` | FC3-Sig: new signing epoch certified by the rotation proof |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:219:| 223 | `pub enum CanonicalMessage` | FC1-Sig+FC3-Sig: only typed runtime messages may enter signature verification |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:220:| 226 | `RejectedAttemptSummary` | FC1-Sig: predicate-runner rejection summary |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:221:| 228 | `TerminalSummarySigning` | FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.1 closure C-3): terminal |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:222:| 236 | `FinalizeRewardSigning` | FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): finalize |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:223:| 240 | `TaskExpireSigning` | FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): task |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:224:| 244 | `EpochRotationProof` | FC3-Sig: system key epoch continuity proof |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:225:| 246 | `LedgerEntrySigning` | FC2-Append (CO1.7 v1.2 round-2 closure C3): L4 transition_ledger |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:226:| 255 | `pub struct PinnedSystemPubkeys` | FC1-Sig+FC3-Sig: epoch-indexed public keys pinned by genesis and rotation history |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:227:| 262 | `pub fn new` | FC1-Sig+FC3-Sig: create an empty pinned system-key map |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:228:| 267 | `pub fn insert` | FC1-Sig+FC3-Sig: pin a public key for a system epoch |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:229:| 276 | `pub fn get` | FC1-Sig+FC3-Sig: fetch the public key pinned for a system epoch |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:230:| 282 | `pub struct Ed25519Keypair` | FC1-Sig+FC3-Sig: in-memory ed25519 system keypair with zeroized private key on drop |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:231:| 291 | `pub fn generate_with_secure_entropy` | FC1-Sig+FC3-Sig: generate ed25519 key material from `getrandom(2)` entropy |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:232:| 306 | `pub const fn` | FC1-Sig+FC3-Sig: return the public half of the system keypair |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:233:| 367 | `pub enum KeypairError` | FC1-Sig+FC3-Sig: system keypair lifecycle and crypto error taxonomy |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:234:| 370 | `Io` | FC1-Sig+FC3-Sig: filesystem operation failed |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:235:| 372 | `Entropy` | FC1-Sig+FC3-Sig: secure operating-system entropy failed |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:236:| 374 | `KdfParam` | FC1-Sig+FC3-Sig: KDF environment parameter was absent or invalid |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:237:| 376 | `Kdf` | FC1-Sig+FC3-Sig: Argon2id key derivation failed |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:238:| 378 | `Crypto` | FC1-Sig+FC3-Sig: ChaCha20-Poly1305 encryption or authentication failed |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:239:| 380 | `InvalidFormat` | FC1-Sig+FC3-Sig: encrypted keystore format was malformed |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:240:| 382 | `HomeUnavailable` | FC1-Sig+FC3-Sig: default keystore path could not be resolved |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:241:| 410 | `pub fn default_system_keystore_path` | FC1-Sig+FC3-Sig: resolve `~/.turingos/keystore/system_keypair_v{epoch}.enc` |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:242:| 425 | `pub fn generate_or_load_system_keypair` | FC1-Sig+FC3-Sig: first-boot generate-or-second-boot decrypt lifecycle entrypoint |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:243:| 440 | `pub fn load_existing_keypair` | FC1-Sig+FC3-Sig: decrypt an existing encrypted system keypair keystore |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:244:| 460 | `pub fn canonical_digest` | FC1-Sig+FC3-Sig: canonical SHA-256 digest for typed system messages |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:245:| 500 | `pub fn verify_system_signature` | FC1-Sig+FC3-Sig: public system signature verification against pinned epoch keys |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:246:| 519 | `pub fn verify_epoch_rotation_proof` | FC3-Sig: verify old and new signatures over a rotation continuity proof |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:247:| 531 | `pub fn verify_system_pubkeys` | FC3-Sig: boot extension stub for genesis `[system_pubkeys]` verification |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:248:| 541 | `pub mod predicate_runner` | FC1-Sig: crate-only signing surface for the predicate runner |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:249:| 548 | `pub fn sign_rejected_attempt_summary` | FC1-Sig: sign only typed rejected-attempt summaries from the predicate runner |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:250:| 559 | `pub fn sign_system_message` | FC1-Sig: sign only typed canonical messages within the predicate-runner scope |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:251:| 568 | `pub mod terminal_summary_emitter` | FC1-Sig+FC3-Sig: crate-only signing surface for system-emitted |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:252:| 584 | `pub fn sign_terminal_summary` | FC1-Sig+FC3-Sig: sign an opaque 32-byte digest of a |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:253:| 593 | `pub fn sign_finalize_reward` | FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): sign an |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:254:| 604 | `pub fn sign_task_expire` | FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): sign an |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:255:| 615 | `pub fn sign_epoch_rotation_proof` | FC3-Sig: sign only typed epoch rotation proofs |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:256:| 626 | `pub fn sign_system_message` | FC1-Sig+FC3-Sig: sign only typed canonical messages within terminal-summary scope |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:257:| 635 | `pub mod transition_ledger_emitter` | FC2-Append + FC1-Sig: crate-only signing surface for the L4 |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:258:| 646 | `pub fn sign_ledger_entry` | FC2-Append: sign only the canonical-digest of a |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:264:| 48 | `pub enum TxKind` | FC2-Append: discriminator for the typed payload behind a CAS Cid |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:265:| 63 | `pub struct LedgerEntry` | FC2-Append + WP § 5.L4: stored LedgerEntry record (11 fields) |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:266:| 99 | `pub struct LedgerEntrySigningPayload` | FC2-Append C3: the bytes the system keypair actually signs |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:267:| 168 | `pub fn append` | FC2-Append + spec § 4: pure ledger-root fold over signed digests |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:268:| 183 | `pub trait LedgerWriter` | FC2-Append: storage abstraction for L4 |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:275:| 9 | `pub mod ledger` | FC1-Sig+FC3-Sig: Bottom White ledger crypto modules |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:281:| 19 | `pub const PENDING_COMPLETION_TOKENS_CO1_1_4` | FC1-Cost / FC3-Cost: placeholder until CO1.1.4 STEP_B propagates |
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:290:| 12 | `pub mod typed_tx` | FC2-Submit / CO1.1.4-pre1 — typed-tx ABI surface (TypedTx + per-kind structs) |
handover/alignment/TRACE_FLOWCHART_MATRIX.md:1:# TRACE_FLOWCHART_MATRIX — TB ↔ Constitution Flowchart Mapping
handover/alignment/TRACE_FLOWCHART_MATRIX.md:3:**Authority**: architect directive 2026-05-02, ruling 9 of Part C ("Add TRACE_FLOWCHART_MATRIX.md mapping future TBs to Flowchart 1/2/3").
handover/alignment/TRACE_FLOWCHART_MATRIX.md:5:- Insight summary: `handover/architect-insights/2026-05-02_flowchart_hashes_and_trace_matrix.md`
handover/alignment/TRACE_FLOWCHART_MATRIX.md:13:The lossless constitution integrated edition 2026-05-02 elevated three flowcharts from explanatory diagrams to **SHA256-anchored architectural contracts**. Each TB charter (current + future) must declare which flowchart element(s) it touches; deviation between code behavior and the four canonical hashes is now provably auditable.
handover/alignment/TRACE_FLOWCHART_MATRIX.md:19:## §2 The four canonical flowchart hashes
handover/alignment/TRACE_FLOWCHART_MATRIX.md:44:Legend: ✅ touched (TB closes or extends this flowchart element) · ◯ adjacent (TB references but does not modify) · — not relevant.
handover/alignment/TRACE_FLOWCHART_MATRIX.md:48:| TB-1 | ◯ | — | — | TypedTx skeleton (pre-flowchart-anchoring) |
handover/alignment/TRACE_FLOWCHART_MATRIX.md:69:## §4 Validation tests by flowchart
handover/alignment/TRACE_FLOWCHART_MATRIX.md:133:Reference: `handover/architect-insights/2026-05-02_flowchart_hashes_and_trace_matrix.md`; directive Part C §11 (TB-15 EvidenceCapsule structure).
handover/alignment/TRACE_FLOWCHART_MATRIX.md:140:1. Every new TB charter MUST add a row to §3 declaring which flowchart
handover/alignment/TRACE_FLOWCHART_MATRIX.md:142:   (per feedback_tb_phase_tag_required pattern, extended to flowcharts).
handover/alignment/TRACE_FLOWCHART_MATRIX.md:144:2. At TB ship, validation tests in §4 are checked for the touched flowcharts.
handover/alignment/TRACE_FLOWCHART_MATRIX.md:147:3. If a TB touches no flowchart element (rare; e.g., pure docs TB), declare
handover/alignment/TRACE_FLOWCHART_MATRIX.md:153:5. Flowchart hashes are immutable. If the constitution canonical flowcharts
handover/alignment/TRACE_FLOWCHART_MATRIX.md:171:Reconstruction caveat for back-filled rows: TB-1..TB-7R rows in §3 are best-effort declarations based on what each TB demonstrably touched. Original TB charters did not declare flowchart traces. Future TBs will declare directly.
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md:5:**Authority**: architect verdict 2026-05-01 (`handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md`) §C9 — "NEVER fabricate FC numbers; if no precise row, use `FC-trace: WP-§5.L3/L4 + Art.I.1 + Art.III.4` + register an orphan justification."
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md:20:Each module's pub items carried initial `TRACE_MATRIX FC?-N?` doc-comments
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md:21:that were either **fabricated** (label looked correct but the FC node
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md:22:number actually maps to a different concept) or **vague** (a flowchart
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md:46:**Initial label (fabricated)**: `TRACE_MATRIX FC1-N6: predicate / wtool gate`.
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md:47:**Actual content of FC1-N6**: `input = ⟨q_i, s_i⟩` UniverseSnapshot —
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md:61:FC-trace: Art.I.1 (机制 > 参数) + Art.III.4 (selective broadcasting /
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md:66:for "FC<n>-PreRoutingPredicateGate" or equivalent, mapping to this
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md:85:**Initial label (incorrect mapping)**: `TRACE_MATRIX FC2 (Boot / Genesis)`.
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md:86:**Actual content of FC2**: Append + Submit — L4 transition_ledger,
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md:88:`handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:197-298`). FC2 is
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md:90:"Boot / Genesis") used a conceptual three-flowchart shorthand that does
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md:91:not align with the canonical TRACE_MATRIX FC numbering — the three
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md:92:flowcharts in TRACE_MATRIX are FC1 (Runtime State Transition), FC2
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md:93:(Append/Submit), FC3 (Trust Root readonly subgraph), with **Boot /
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md:94:Genesis** anchored under **Article IV Boot** rather than a separate FC.
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md:101:  preconditions (constitution_hash + runtime_repo + cas_path +
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md:107:FC-trace: Art.IV Boot (Bootstrap 公理 — 创世状态) + Art.I.1 (机制 >
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md:129:- TRACE_MATRIX v0 (FC1-N6 row): `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:28`
handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md:33:- Constitutional flowchart re-mapping. The wallet's FC role is unchanged (still bottom-white tool serving Law 2); only its filesystem path moved.
handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md:36:**FC-trace**: this OBS is constitutional-hygiene observation, not a behavior change. It does not require a flowchart node mapping per se, but the wallet itself remains FC3 (readonly subgraph) institutional state.
handover/alignment/FC_ELEMENTS_2026-04-22.md:9:- Line 325: ` ```mermaid ` (opener, FC-1)
handover/alignment/FC_ELEMENTS_2026-04-22.md:10:- Line 379: ` ``` ` (closer, FC-1)
handover/alignment/FC_ELEMENTS_2026-04-22.md:11:- Line 530: ` ``` ` (closer, FC-2 — the **opener is missing** from the file; the block starts with a bare `flowchart TD` at line 441, indented 4 spaces)
handover/alignment/FC_ELEMENTS_2026-04-22.md:12:- Line 714: ` ``` ` (closer, FC-3 — the **opener is missing**; the block starts with bare `graph TB` at line 670, indented 4 spaces)
handover/alignment/FC_ELEMENTS_2026-04-22.md:14:So strictly only one properly-fenced `mermaid` block exists. FC-2 and FC-3 are indented pseudo-blocks with only closing backticks. Mermaid won't render them on GitHub without a proper opener. This is a constitutional-document hygiene issue that should be flagged to the human architect (Art. V.2 — the constitution is the ground truth and must be machine-parseable).
handover/alignment/FC_ELEMENTS_2026-04-22.md:16:Below the three flowcharts are extracted as if the fencing were correct.
handover/alignment/FC_ELEMENTS_2026-04-22.md:20:## FC-1 (basic cycle)
handover/alignment/FC_ELEMENTS_2026-04-22.md:28:| FC1-N1 | circle `(("..."))` | `$$q$$` | Q0 |
handover/alignment/FC_ELEMENTS_2026-04-22.md:29:| FC1-N2 | triangle `shape: tri` | `path` | Q0 |
handover/alignment/FC_ELEMENTS_2026-04-22.md:30:| FC1-N3 | lin-cyl `shape: lin-cyl` | `everything as files` | Q0 |
handover/alignment/FC_ELEMENTS_2026-04-22.md:31:| FC1-N4 | circle `(("..."))` | `$$q'$$` | Q1 |
handover/alignment/FC_ELEMENTS_2026-04-22.md:32:| FC1-N5 | triangle `shape: tri` | `path'` | Q1 |
handover/alignment/FC_ELEMENTS_2026-04-22.md:33:| FC1-N6 | lin-cyl `shape: lin-cyl` | `everything as files'` | Q1 |
handover/alignment/FC_ELEMENTS_2026-04-22.md:34:| FC1-N7 | rect `["..."]` classDef black | `$$\delta$$` | AI |
handover/alignment/FC_ELEMENTS_2026-04-22.md:35:| FC1-N8 | circle `(("..."))` | `$$q$$` | input |
handover/alignment/FC_ELEMENTS_2026-04-22.md:36:| FC1-N9 | circle `(("..."))` | `$$s$$` | input |
handover/alignment/FC_ELEMENTS_2026-04-22.md:37:| FC1-N10 | circle `(("..."))` | `$$q'$$` | output |
handover/alignment/FC_ELEMENTS_2026-04-22.md:38:| FC1-N11 | circle `(("..."))` | `$$a$$` | output |
handover/alignment/FC_ELEMENTS_2026-04-22.md:39:| FC1-N12 | rhombus `{"..."}` classDef white | `$$\prod$$ predicates` | top |
handover/alignment/FC_ELEMENTS_2026-04-22.md:40:| FC1-N13 | rect `["..."]` classDef white | `read tool` | rtool |
handover/alignment/FC_ELEMENTS_2026-04-22.md:41:| FC1-N14 | rect `["..."]` classDef white | `write tool` | wtool |
handover/alignment/FC_ELEMENTS_2026-04-22.md:47:| FC1-E1 | tape0 | si | dotted `-.->` (multi-source: `tape0 & HEAD0 -.-> si`) | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:48:| FC1-E2 | HEAD0 | si | dotted `-.->` (multi-source: `tape0 & HEAD0 -.-> si`) | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:49:| FC1-E3 | q0 | qi | dotted `-.->` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:50:| FC1-E4 | qi | delta | dotted `-.->` (multi-source: `qi & si -.-> delta`) | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:51:| FC1-E5 | si | delta | dotted `-.->` (multi-source: `qi & si -.-> delta`) | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:52:| FC1-E6 | delta | qo | dotted `-.->` (multi-target: `delta -.-> qo & ao`) | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:53:| FC1-E7 | delta | ao | dotted `-.->` (multi-target: `delta -.-> qo & ao`) | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:54:| FC1-E8 | qo | q1 | dotted `-.->` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:55:| FC1-E9 | ao | tape1 | dotted `-.->` (multi-target: `ao -.-> tape1 & HEAD1`) | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:56:| FC1-E10 | ao | HEAD1 | dotted `-.->` (multi-target: `ao -.-> tape1 & HEAD1`) | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:57:| FC1-E11 | Q0 | rtool | thick `==>` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:58:| FC1-E12 | rtool | input | thick `==>` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:59:| FC1-E13 | input | AI | thick `==>` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:60:| FC1-E14 | AI | output | thick `==>` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:61:| FC1-E15 | output | p | thick `==>` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:62:| FC1-E16 | p | wtool | thick `==>` | `1` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:63:| FC1-E17 | wtool | Q1 | thick `==>` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:64:| FC1-E18 | p | Q0 | thick `==>` | `0` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:70:| FC1-S1 | Q0 → `"version control $$~Q_{t}$$"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:71:| FC1-S2 | Q1 → `"version control $$~Q_{t+1}$$"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:72:| FC1-S3 | AI → `"middle black"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:73:| FC1-S4 | input → `input` (bare id, no quoted label) |
handover/alignment/FC_ELEMENTS_2026-04-22.md:74:| FC1-S5 | output → `output` (bare id, no quoted label) |
handover/alignment/FC_ELEMENTS_2026-04-22.md:75:| FC1-S6 | top → `"top management"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:76:| FC1-S7 | rtool → `"bottom tools"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:77:| FC1-S8 | wtool → `"bottom tools"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:79:### Formulas embedded in FC-1 labels
handover/alignment/FC_ELEMENTS_2026-04-22.md:83:| FC1-S1 subgraph title | `$$~Q_{t}$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:84:| FC1-S2 subgraph title | `$$~Q_{t+1}$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:85:| FC1-N1 (q0) | `$$q$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:86:| FC1-N4 (q1) | `$$q'$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:87:| FC1-N7 (delta) | `$$\delta$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:88:| FC1-N8 (qi) | `$$q$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:89:| FC1-N9 (si) | `$$s$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:90:| FC1-N10 (qo) | `$$q'$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:91:| FC1-N11 (ao) | `$$a$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:92:| FC1-N12 (p) | `$$\prod$$ predicates` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:94:### FC-1 classDefs
handover/alignment/FC_ELEMENTS_2026-04-22.md:101:**FC-1 counts: 14 nodes, 18 edges, 8 subgraphs.**
handover/alignment/FC_ELEMENTS_2026-04-22.md:105:## FC-2 (expanded with init / halt / tick)
handover/alignment/FC_ELEMENTS_2026-04-22.md:107:Location: lines 441–530. Header: `flowchart TD` (indented 4 spaces, opening ` ```mermaid ` fence is MISSING in source).
handover/alignment/FC_ELEMENTS_2026-04-22.md:113:| FC2-N1 | slanted-rect `shape: sl-rect` classDef human | `human architect provides spec` | Initialization |
handover/alignment/FC_ELEMENTS_2026-04-22.md:114:| FC2-N2 | docs `shape: docs` classDef white | `(tentative) ground truth` | Initialization |
handover/alignment/FC_ELEMENTS_2026-04-22.md:115:| FC2-N3 | rect `[...]` classDef black | `Init AI` | Initialization |
handover/alignment/FC_ELEMENTS_2026-04-22.md:116:| FC2-N4 | double-circle `shape: dbl-circ` | `HALT` | Finalization |
handover/alignment/FC_ELEMENTS_2026-04-22.md:117:| FC2-N5 | circle `(("..."))` | `$$q_t$$` | Q0 |
handover/alignment/FC_ELEMENTS_2026-04-22.md:118:| FC2-N6 | triangle `shape: tri` | `$$HEAD_t$$<br>as path` | Q0 |
handover/alignment/FC_ELEMENTS_2026-04-22.md:119:| FC2-N7 | lin-cyl `shape: lin-cyl` | `$$tape_t$$<br>as files` | Q0 |
handover/alignment/FC_ELEMENTS_2026-04-22.md:120:| FC2-N8 | circle `(("..."))` | `$$q_{t+1}$$` | Q1 |
handover/alignment/FC_ELEMENTS_2026-04-22.md:121:| FC2-N9 | triangle `shape: tri` | `$$HEAD_{t+1}$$<br>as path` | Q1 |
handover/alignment/FC_ELEMENTS_2026-04-22.md:122:| FC2-N10 | lin-cyl `shape: lin-cyl` | `$$tape_{t+1}$$<br>as files` | Q1 |
handover/alignment/FC_ELEMENTS_2026-04-22.md:123:| FC2-N11 | rect `["..."]` classDef white | `read tool` | rtool |
handover/alignment/FC_ELEMENTS_2026-04-22.md:124:| FC2-N12 | circle `(("..."))` | `$$q_i$$` | input |
handover/alignment/FC_ELEMENTS_2026-04-22.md:125:| FC2-N13 | circle `(("..."))` | `$$s_i$$` | input |
handover/alignment/FC_ELEMENTS_2026-04-22.md:126:| FC2-N14 | rect `["..."]` classDef black | `AI as $$\delta$$` | AI |
handover/alignment/FC_ELEMENTS_2026-04-22.md:127:| FC2-N15 | circle `(("..."))` | `$$q_o$$` | output |
handover/alignment/FC_ELEMENTS_2026-04-22.md:128:| FC2-N16 | circle `(("..."))` | `$$a_o$$` | output |
handover/alignment/FC_ELEMENTS_2026-04-22.md:129:| FC2-N17 | processes `shape: processes` classDef white | `predicates $$p$$` | top |
handover/alignment/FC_ELEMENTS_2026-04-22.md:130:| FC2-N18 | rhombus `{"..."}` classDef white | `$$\prod \mathbf{p}$$` | top |
handover/alignment/FC_ELEMENTS_2026-04-22.md:131:| FC2-N19 | rect `["..."]` classDef white | `map reduce` | toptick |
handover/alignment/FC_ELEMENTS_2026-04-22.md:132:| FC2-N20 | circle `(("clock"))` classDef white | `clock` | toptick |
handover/alignment/FC_ELEMENTS_2026-04-22.md:133:| FC2-N21 | rect `["..."]` classDef white | `write tool` | wtool |
handover/alignment/FC_ELEMENTS_2026-04-22.md:134:| FC2-N22 | rect `["..."]` classDef white | `other tools` | wtool |
handover/alignment/FC_ELEMENTS_2026-04-22.md:140:| FC2-E1 | human | law | cross-dashed `--x` | `once` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:141:| FC2-E2 | law | initAI | arrow `-->` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:142:| FC2-E3 | initAI | predicates | cross-dashed `--x` | `once` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:143:| FC2-E4 | predicates | p | line `---` (undirected) | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:144:| FC2-E5 | initAI | mr | cross-dashed `--x` | `once` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:145:| FC2-E6 | initAI | Q0 | cross-dashed `--x` | `once` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:146:| FC2-E7 | tape0 | si | long arrow `---->` (multi-source: `tape0 & HEAD0 ----> si`) | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:147:| FC2-E8 | HEAD0 | si | long arrow `---->` (multi-source: `tape0 & HEAD0 ----> si`) | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:148:| FC2-E9 | q0 | qi | arrow `-->` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:149:| FC2-E10 | qi | delta | arrow `-->` (multi-source: `qi & si --> delta`) | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:150:| FC2-E11 | si | delta | arrow `-->` (multi-source: `qi & si --> delta`) | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:151:| FC2-E12 | delta | qo | arrow `-->` (multi-target: `delta --> qo & ao`) | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:152:| FC2-E13 | delta | ao | arrow `-->` (multi-target: `delta --> qo & ao`) | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:153:| FC2-E14 | qo | q1 | dotted `-.->` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:154:| FC2-E15 | ao | HEAD1 | dotted `-.->` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:155:| FC2-E16 | ao | tape1 | dotted `-.->` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:156:| FC2-E17 | Q0 | rtool | thick `==>` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:157:| FC2-E18 | rtool | input | thick `==>` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:158:| FC2-E19 | input | AI | thick `==>` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:159:| FC2-E20 | AI | output | thick `==>` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:160:| FC2-E21 | output | p | thick `==>` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:161:| FC2-E22 | p | wtool | thick `==>` | `"$$Q_{t+1} = \mathbf{wtool}(output)$$<br>if $$\prod \mathbf{p} = 1$$"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:162:| FC2-E23 | wtool | Q1 | thick `==>` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:163:| FC2-E24 | p | Q0 | thick `==>` | `"$$Q_{t+1} = Q_t$$<br>if $$\prod \mathbf{p} = 0$$"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:164:| FC2-E25 | q1 | halt | thick `==>` | `"if q = halt"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:165:| FC2-E26 | clock | mr | arrow `-->` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:166:| FC2-E27 | mr | tape0 | thick `==>` | `map` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:167:| FC2-E28 | mr | tape1 | thick `==>` | `reduce` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:173:| FC2-S1 | Initialization → `Initialization` (bare id, no quoted label) |
handover/alignment/FC_ELEMENTS_2026-04-22.md:174:| FC2-S2 | Finalization → `Finalization` (bare id, no quoted label) |
handover/alignment/FC_ELEMENTS_2026-04-22.md:175:| FC2-S3 | Q0 → `"version control: $$Q_t = \langle q_t,\ HEAD_t,\ tape_t \rangle$$"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:176:| FC2-S4 | Q1 → `"version control: $$Q_{t+1} = \langle q_{t+1},\ HEAD_{t+1},\ tape_{t+1}\rangle$$"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:177:| FC2-S5 | rtool → `"bottom tools: $$\langle q_i,\ s_i \rangle = \mathbf{rtool}(\langle q_t,\ tape_t,\ HEAD_t \rangle)$$"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:178:| FC2-S6 | input → `"$$input = \langle q_i,\ s_i \rangle$$"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:179:| FC2-S7 | AI → `"middle black: $$output = \delta(input)$$"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:180:| FC2-S8 | output → `"$$output = \langle q_o,\ a_o \rangle$$"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:181:| FC2-S9 | top → `"top management: $$\prod \mathbf{p}(output \mid Q_t)$$"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:182:| FC2-S10 | toptick → `"top management: ticks"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:183:| FC2-S11 | wtool → `"bottom tools: $$\mathbf{wtool}(output \mid tape_t,HEAD_t,tools_{other})$$"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:185:### Formulas embedded in FC-2 labels
handover/alignment/FC_ELEMENTS_2026-04-22.md:189:| FC2-S3 title | `$$Q_t = \langle q_t,\ HEAD_t,\ tape_t \rangle$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:190:| FC2-S4 title | `$$Q_{t+1} = \langle q_{t+1},\ HEAD_{t+1},\ tape_{t+1}\rangle$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:191:| FC2-S5 title | `$$\langle q_i,\ s_i \rangle = \mathbf{rtool}(\langle q_t,\ tape_t,\ HEAD_t \rangle)$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:192:| FC2-S6 title | `$$input = \langle q_i,\ s_i \rangle$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:193:| FC2-S7 title | `$$output = \delta(input)$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:194:| FC2-S8 title | `$$output = \langle q_o,\ a_o \rangle$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:195:| FC2-S9 title | `$$\prod \mathbf{p}(output \mid Q_t)$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:196:| FC2-S11 title | `$$\mathbf{wtool}(output \mid tape_t,HEAD_t,tools_{other})$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:197:| FC2-N5..N10 | `$$q_t$$`, `$$HEAD_t$$`, `$$tape_t$$`, `$$q_{t+1}$$`, `$$HEAD_{t+1}$$`, `$$tape_{t+1}$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:198:| FC2-N12..N13 | `$$q_i$$`, `$$s_i$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:199:| FC2-N14 | `AI as $$\delta$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:200:| FC2-N15..N16 | `$$q_o$$`, `$$a_o$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:201:| FC2-N17 | `predicates $$p$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:202:| FC2-N18 | `$$\prod \mathbf{p}$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:203:| FC2-E22 label | `$$Q_{t+1} = \mathbf{wtool}(output)$$<br>if $$\prod \mathbf{p} = 1$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:204:| FC2-E24 label | `$$Q_{t+1} = Q_t$$<br>if $$\prod \mathbf{p} = 0$$` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:205:| FC2-E25 label | `if q = halt` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:207:### FC-2 classDefs
handover/alignment/FC_ELEMENTS_2026-04-22.md:216:**FC-2 counts: 22 nodes, 28 edges, 11 subgraphs.**
handover/alignment/FC_ELEMENTS_2026-04-22.md:220:## FC-3 (anti-oreo system-level)
handover/alignment/FC_ELEMENTS_2026-04-22.md:228:| FC3-N1 | bare id | `boot` | (root, no subgraph) |
handover/alignment/FC_ELEMENTS_2026-04-22.md:229:| FC3-N2 | bare id classDef human | `human` | (root, no subgraph) |
handover/alignment/FC_ELEMENTS_2026-04-22.md:230:| FC3-N3 | doc `shape: doc` classDef white | `constitution as ground truth` | readonly (inside InitAI) |
handover/alignment/FC_ELEMENTS_2026-04-22.md:231:| FC3-N4 | docs `shape: docs` classDef white | `logs archive as ground truth` | readonly (inside InitAI) |
handover/alignment/FC_ELEMENTS_2026-04-22.md:232:| FC3-N5 | rect `[...]` classDef black | `JudgeAI` | InitAI |
handover/alignment/FC_ELEMENTS_2026-04-22.md:233:| FC3-N6 | rect `[...]` classDef black | `ArchitectAI` | InitAI |
handover/alignment/FC_ELEMENTS_2026-04-22.md:234:| FC3-N7 | bare id classDef white | `top` | anti_oreo |
handover/alignment/FC_ELEMENTS_2026-04-22.md:235:| FC3-N8 | bare id classDef black | `agents` | anti_oreo |
handover/alignment/FC_ELEMENTS_2026-04-22.md:236:| FC3-N9 | bare id classDef white | `tools` | anti_oreo |
handover/alignment/FC_ELEMENTS_2026-04-22.md:237:| FC3-N10 | rect `["..."]` | `Q` (id = `tape`, label = `"Q"`) | system |
handover/alignment/FC_ELEMENTS_2026-04-22.md:238:| FC3-N11 | doc `shape: doc` classDef white | `log` | system |
handover/alignment/FC_ELEMENTS_2026-04-22.md:239:| FC3-N12 | rhombus `{"..."}` | `need to improve?` (id = `error`) | (root, no subgraph) |
handover/alignment/FC_ELEMENTS_2026-04-22.md:245:| FC3-E1 | human | constitution | arrow `-->` | `maintain` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:246:| FC3-E2 | top | agents | thick `==>` | `manage` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:247:| FC3-E3 | agents | tools | thick `==>` | `use` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:248:| FC3-E4 | judgeAI | tools | dotted `-.->` (multi-source: `judgeAI & architectAI -.->\|use\| tools`) | `use` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:249:| FC3-E5 | architectAI | tools | dotted `-.->` (multi-source: `judgeAI & architectAI -.->\|use\| tools`) | `use` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:250:| FC3-E6 | log | logs | very-thick `====>` | `archive` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:251:| FC3-E7 | boot | init | thick `==>` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:252:| FC3-E8 | init | top | thick `==>` | `init/iterate` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:253:| FC3-E9 | init | tape | arrow `-->` | `init` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:254:| FC3-E10 | init | tools | thick `==>` | `make/improve` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:255:| FC3-E11 | tools | log | thick `==>` | `write` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:256:| FC3-E12 | logs | architectAI | arrow `-->` | `feedback` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:257:| FC3-E13 | init | error | thick `==>` | — |
handover/alignment/FC_ELEMENTS_2026-04-22.md:258:| FC3-E14 | error | boot | very-long-thick `==========>` | `re-init` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:259:| FC3-E15 | constitution | judgeAI | arrow `-->` (multi-target: `constitution -->\|abide\| judgeAI & architectAI`) | `abide` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:260:| FC3-E16 | constitution | architectAI | arrow `-->` (multi-target: `constitution -->\|abide\| judgeAI & architectAI`) | `abide` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:261:| FC3-E17 | judgeAI | architectAI | arrow `-->` | `veto` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:267:| FC3-S1 | system → `system` (bare id, no quoted label) |
handover/alignment/FC_ELEMENTS_2026-04-22.md:268:| FC3-S2 | init → `"InitAI"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:269:| FC3-S3 | readonly → `readonly` (bare id, no quoted label) |
handover/alignment/FC_ELEMENTS_2026-04-22.md:270:| FC3-S4 | anti_oreo → `"anti-oreo"` |
handover/alignment/FC_ELEMENTS_2026-04-22.md:272:### Formulas embedded in FC-3 labels
handover/alignment/FC_ELEMENTS_2026-04-22.md:274:None. FC-3 uses only English/natural-language labels (`maintain`, `manage`, `use`, `archive`, `init/iterate`, `init`, `make/improve`, `write`, `feedback`, `re-init`, `abide`, `veto`, `need to improve?`, `constitution as ground truth`, `logs archive as ground truth`, `Q`). No LaTeX math.
handover/alignment/FC_ELEMENTS_2026-04-22.md:276:### FC-3 classDefs
handover/alignment/FC_ELEMENTS_2026-04-22.md:285:**FC-3 counts: 12 nodes, 17 edges, 4 subgraphs.**
handover/alignment/FC_ELEMENTS_2026-04-22.md:293:| FC-1 | 14 | 18 | 8 |
handover/alignment/FC_ELEMENTS_2026-04-22.md:294:| FC-2 | 22 | 28 | 11 |
handover/alignment/FC_ELEMENTS_2026-04-22.md:295:| FC-3 | 12 | 17 | 4 |
handover/alignment/FC_ELEMENTS_2026-04-22.md:302:1. **Fencing bug in constitution.md** — only FC-1 has a proper ` ```mermaid ` opener at line 325. FC-2 (line 441, `flowchart TD`) and FC-3 (line 670, `graph TB`) are indented-code pseudo-blocks with only a closing ` ``` ` (lines 530 / 714). Neither will render as mermaid on GitHub / Notion in current form. Treat as constitutional hygiene issue (Art. V.2 — ground truth must be machine-parseable).
handover/alignment/FC_ELEMENTS_2026-04-22.md:303:2. **FC-3 node `tape` id vs label mismatch** — declared as `tape["Q"]`; id is `tape`, rendered label is `"Q"`. Easy to mis-wire to the FC-1/FC-2 `tape_t` lin-cyl nodes; they are distinct entities.
handover/alignment/FC_ELEMENTS_2026-04-22.md:304:3. **FC-3 `top`, `agents`, `tools` are bare-id nodes** (no `[...]` or `{...}` shape wrappers), only styled via `:::white` / `:::black`. Mermaid renders them as default rectangles with the id as the visible label.
handover/alignment/FC_ELEMENTS_2026-04-22.md:305:4. **FC-3 `boot`, `human`, `error` live outside every subgraph** — they are at the root of `graph TB`. Only `error` has an explicit shape (`{need to improve?}`); `boot` and `human` are bare ids.
handover/alignment/FC_ELEMENTS_2026-04-22.md:306:5. **FC-2 edge label multi-line markup** — labels on FC2-E22, FC2-E24 use `<br>` for line-break inside `$$...$$`. Mermaid + KaTeX handling is renderer-dependent; these may display as a single line on some renderers.
handover/alignment/FC_ELEMENTS_2026-04-22.md:307:6. **Class `note` is declared but never applied** — both FC-2 and FC-3 declare `classDef note` but no node uses `:::note`. Dead CSS.
handover/alignment/FC_ELEMENTS_2026-04-22.md:308:7. **Label reuse across subgraphs** — `rtool` and `wtool` in FC-1 both have identical subgraph label `"bottom tools"`. Not an error, but ambiguous in plain-text readings.

exec
/bin/bash -lc "nl -ba handover/evidence/tb_15_markov_capsule_2026-05-03/MARKOV_TB-15_2026-05-03.json | sed -n '198,285p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   198	  "next_session_context_cid": [
   199	    87,
   200	    91,
   201	    95,
   202	    217,
   203	    194,
   204	    163,
   205	    133,
   206	    191,
   207	    116,
   208	    139,
   209	    148,
   210	    30,
   211	    5,
   212	    150,
   213	    220,
   214	    218,
   215	    75,
   216	    5,
   217	    66,
   218	    9,
   219	    83,
   220	    41,
   221	    241,
   222	    145,
   223	    111,
   224	    10,
   225	    113,
   226	    34,
   227	    192,
   228	    110,
   229	    66,
   230	    128
   231	  ],
   232	  "sha256": [
   233	    178,
   234	    68,
   235	    241,
   236	    106,
   237	    31,
   238	    59,
   239	    213,
   240	    50,
   241	    208,
   242	    65,
   243	    164,
   244	    15,
   245	    227,
   246	    155,
   247	    43,
   248	    126,
   249	    124,
   250	    193,
   251	    47,
   252	    181,
   253	    142,
   254	    24,
   255	    182,
   256	    26,
   257	    237,
   258	    215,
   259	    106,
   260	    128,
   261	    16,
   262	    238,
   263	    177,
   264	    182
   265	  ],
   266	  "created_at_logical_t": 0,
   267	  "tb_tag": "TB-15"
   268	}

exec
/bin/bash -lc "nl -ba handover/evidence/tb_15_markov_capsule_2026-05-03/README.md | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TB-15 First Markov Capsule — 2026-05-03
     2	
     3	**TB**: TB-15 — Lamarckian Autopsy + Markov EvidenceCapsule
     4	**Charter**: `handover/tracer_bullets/TB-15_charter_2026-05-03.md`
     5	**Architect spec**: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §6 (FR-15.1..6 + CR-15.1..6 + SG-15.1..8)
     6	
     7	## Generation command
     8	
     9	```bash
    10	cargo run --bin generate_markov_capsule -- \
    11	  --tb-id 15 \
    12	  --out-dir handover/markov_capsules \
    13	  --constitution-path constitution.md \
    14	  --no-cas
    15	```
    16	
    17	## Outputs
    18	
    19	- `MARKOV_TB-15_2026-05-03.json` — first MarkovEvidenceCapsule (TB-15 self-reference; the genesis Markov capsule of the TuringOS v4 chain)
    20	- `LATEST_MARKOV_CAPSULE.txt` — Cid hex pointer (`b244f16a1f3bd532d041a40fe39b2b7e7cc12fb58e18b61aedd76a8010eeb1b6`)
    21	
    22	## Field summary (capsule)
    23	
    24	```text
    25	capsule_id            = b244f16a1f3bd532d041a40fe39b2b7e7cc12fb58e18b61aedd76a8010eeb1b6
    26	previous_capsule_cid  = None  (genesis Markov; SG-15.7 chain root)
    27	constitution_hash     = eec695459c71fbef3685583485deb431fe3b561657b2f285b7c5e7e220e59e03
    28	                        (sha256 of constitution.md at generation time;
    29	                         FR-15.4 + SG-15.7)
    30	l4_root               = Hash::ZERO  (v0 placeholder; future TB wires from chain head)
    31	l4e_root              = Hash::ZERO  (same)
    32	cas_root              = Hash::ZERO  (same)
    33	typical_errors        = []          (no TaskBankruptcyTx fired yet — empty per
    34	                                     TB-15 v0 charter §1.2 single-trigger scope)
    35	unresolved_obs        = 22          (handover/alignment/OBS_*.md scan)
    36	next_session_context_cid = (deterministic Cid of NextSessionContext JSON blob;
    37	                            embedded in capsule; readable from CAS via the binary
    38	                            in non-`--no-cas` mode)
    39	tb_tag                = "TB-15"
    40	```
    41	
    42	## Halt-trigger battery (final, post-Atom-6 ship)
    43	
    44	```text
    45	test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
    46	
    47	#1 raw_logs_not_in_general_read_view              GREEN
    48	   (Atom 3 — AgentVisibleProjection file-scan: agent_autopsies_t / AutopsyIndex /
    49	    AgentAutopsyCapsule / private_detail_cid forbidden in projection body)
    50	#2 markov_capsule_references_constitution_hash    GREEN
    51	   (Atom 5 — sha256(constitution.md) == capsule.constitution_hash; SG-15.7)
    52	#3 autopsy_does_not_mutate_predicates             GREEN
    53	   (Atom 2 — autopsy_capsule.rs file-scan: no &mut PredicateRegistry /
    54	    ToolRegistry / RiskPolicyRegistry / register_*/unregister_*/patch_*; CR-15.3 +
    55	    SG-15.8)
    56	#4 private_detail_not_in_other_agent_view         GREEN
    57	   (Atom 3 — AutopsyIndex value type is Vec<Cid>; never raw bytes; SG-15.2)
    58	#5 typical_error_clustering_uses_summary_only     GREEN
    59	   (Atom 4 — cluster_autopsies output JSON contains no input
    60	    private_detail_cid byte run; CR-15.2 + SG-15.5)
    61	#6 deep_history_read_without_override_fails       GREEN
    62	   (Atom 5 — try_deep_history_read_with_override_check(false) → DeepHistoryReadDenied;
    63	    (true) → Ok(()); SG-15.4 + FR-15.5)
    64	```
    65	
    66	## Ship-gate ledger
    67	
    68	| ID | Gate | Status |
    69	|---|---|---|
    70	| SG-15.1 | Failed/losing agent gets private AutopsyCapsule | GREEN — TaskBankruptcyTx dispatch arm Step 3.5 emits per-staker capsule via `derive_autopsies_for_bankruptcy` (verified by `derive_autopsies_emits_one_per_staker_target_only`) |
    71	| SG-15.2 | Raw private details do not enter other Agent read view | GREEN — `agent_autopsies_t` lives on `EconomicState` (sequencer-side), NOT on `AgentVisibleProjection`; halt-trigger #1 + #4 file-scans STRUCTURALLY enforce |
    72	| SG-15.3 | Latest Markov capsule can bootstrap next session | GREEN — `next_session_context_cid` field embedded in capsule; `LATEST_MARKOV_CAPSULE.txt` pointer file written |
    73	| SG-15.4 | Deep-history read without override fails | GREEN — `try_deep_history_read_with_override_check(false)` returns `Err(DeepHistoryReadDenied)`; halt-trigger #6 |
    74	| SG-15.5 | Typical error broadcast uses summary, not raw log | GREEN — `cluster_autopsies` output struct embeds `public_summary` strings + `capsule_id` Cids only; halt-trigger #5 verifies serialization contains no `private_detail_cid` byte run |
    75	| SG-15.6 | Dashboard can regenerate capsule summary from ChainTape + CAS | GREEN — `render_section_15` pure function with deterministic input shape `(Vec<(String,u32)>, Option<&str>)`; 4 dashboard render unit tests; render output contains no raw bytes |
    76	| SG-15.7 | Markov capsule references constitution hash and flowchart hashes | GREEN — `MarkovEvidenceCapsule.constitution_hash` field; `with_constitution_hash` constructor; halt-trigger #2 |
    77	| SG-15.8 | Autopsy does not mutate predicates/tools automatically | GREEN — writer signature has no mutable registry refs; halt-trigger #3 file-scan |
    78	| G-15.9 | `cargo test --workspace` ≥ TB-14 baseline / 0 fail | GREEN — 870 PASS / 0 fail / 150 ignored (net +67 vs TB-14 ship 803) |
    79	| G-15.10 | FC1-N32 + FC1-N33 + FC2-N30 + FC3-N43 each have ≥1 witness | GREEN — `tests/fc_alignment_conformance.rs` has 4 witness tests added (`fc1_n32_*`, `fc1_n33_*`, `fc2_n30_*`, `fc3_n43_*`) |
    80	| G-15.11 | EconomicState sub-field count assertion updated 12→13 | GREEN — 3 sub-field count tests updated (`economic_state_has_thirteen_sub_fields`, `empty_economic_state_serializes_to_thirteen_sub_fields`, `axiom_3_economic_state_present_and_complete` 13) |
    81	| G-15.12 | First Markov capsule generated + persisted | **GREEN — this directory is the artifact** |
    82	
    83	## Replay determinism (Art.0.2)
    84	
    85	The capsule is a pure function of:
    86	- constitution.md bytes (sha256-pinned)
    87	- `previous_capsule_cid` (None for genesis; `Some(prior_capsule.capsule_id)` thereafter)
    88	- L4 / L4.E / CAS roots (v0: zero placeholders; future TB: chain-derived)
    89	- typical_errors (output of `cluster_autopsies` over CAS-resident `AgentAutopsyCapsule` objects)
    90	- unresolved_obs (sorted scan of `handover/alignment/OBS_*.md`)
    91	- `created_at_logical_t` + `tb_tag`
    92	
    93	Re-running the binary with identical inputs yields the same `capsule_id`. Verified by `write_markov_capsule_deterministic_capsule_id` unit test.
    94	
    95	## Privacy contract (architect §6.4)
    96	
    97	- `public_summary` — low-info string; broadcast-eligible IFF N≥3 cluster
    98	- `private_detail_cid` — opaque CAS Cid; AuditOnly access only; NEVER enters `AgentVisibleProjection`
    99	- `evidence_cids` — Cids of pre-existing public ChainTape evidence; not new private bytes
   100	
   101	This evidence directory contains ONLY public surfaces — capsule JSON (which itself contains only Cids + low-info field set) + Cid-hex pointer file. No raw private bytes are persisted under `handover/`.
   102	
   103	## Cross-references
   104	
   105	- TB-15 charter: `handover/tracer_bullets/TB-15_charter_2026-05-03.md`
   106	- Architect spec: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §6
   107	- DECISION_LAMARCKIAN: `handover/alignment/DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN_2026-05-02.md` §1
   108	- Generator binary: `src/bin/generate_markov_capsule.rs`
   109	- Schema: `src/runtime/markov_capsule.rs`
   110	- Autopsy schema + writer: `src/runtime/autopsy_capsule.rs`
   111	- Dashboard §15: `src/bin/audit_dashboard.rs::render_section_15`

exec
/bin/bash -lc 'rg -n "flowchart|flowchart_hash|hashes|l4_root|cas_root|zero placeholders|deferred|SG-15.7" handover/tracer_bullets/TB-15_charter_2026-05-03.md handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md handover/alignment/DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN_2026-05-02.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:70:| SG-15.7 | Markov capsule references constitution hash | `MarkovEvidenceCapsule.constitution_hash` field; halt-trigger #2 |
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:124:- **L4 / L4.E / CAS root chain-readers**: Markov capsule generator currently uses zero placeholders for `l4_root` / `l4e_root` / `cas_root`. Future TB will wire to the actual chain head readers (`Sequencer::current_state_root` / `LedgerWriter::current_root` / CAS metadata digest). Track in TB-16+ controlled-arena work.
handover/tracer_bullets/TB-15_charter_2026-05-03.md:61:            constitution.md bytes at generation time. SG-15.7. MUST PASS.
handover/tracer_bullets/TB-15_charter_2026-05-03.md:72:flowchart_trace:
handover/tracer_bullets/TB-15_charter_2026-05-03.md:141:    l4_root, l4e_root, cas_root, typical_errors,
handover/tracer_bullets/TB-15_charter_2026-05-03.md:196:    execution is RSP-3.2 / TB-9 territory. Wire-in deferred until SlashTx
handover/tracer_bullets/TB-15_charter_2026-05-03.md:199:    reason classes. Wire-in deferred — current FinalizeRewardTx is single-
handover/tracer_bullets/TB-15_charter_2026-05-03.md:233:        l4_root) returns DeepHistoryReadDenied.
handover/tracer_bullets/TB-15_charter_2026-05-03.md:276:- `markov_capsule_references_constitution_hash` (SG-15.7; halt #2)
handover/tracer_bullets/TB-15_charter_2026-05-03.md:336:- capsule.constitution_hash equals sha256 of constitution.md (SG-15.7)
handover/tracer_bullets/TB-15_charter_2026-05-03.md:346:- latest Markov capsule pointer (Cid hex + previous_cid + constitution_hash + l4_root + l4e_root + cas_root)
handover/tracer_bullets/TB-15_charter_2026-05-03.md:405:| SG-15.7 | Markov capsule references constitution hash and flowchart hashes | `tb_15_halt_triggers.rs::markov_capsule_references_constitution_hash` |

exec
/bin/bash -lc 'ls -l handover/alignment/TRACE_FLOWCHART_MATRIX.md handover/alignment/TRACE_MATRIX_v0_2026-04-22.md handover/alignment/FC_ELEMENTS_2026-04-22.md 2>/dev/null' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
-rw-r--r-- 1 zephryj zephryj 15281 Apr 22 18:32 handover/alignment/FC_ELEMENTS_2026-04-22.md
-rw-r--r-- 1 zephryj zephryj 10582 May  2 15:48 handover/alignment/TRACE_FLOWCHART_MATRIX.md
-rw-r--r-- 1 zephryj zephryj 13496 Apr 22 18:47 handover/alignment/TRACE_MATRIX_v0_2026-04-22.md

exec
/bin/bash -lc "nl -ba handover/alignment/TRACE_FLOWCHART_MATRIX.md | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
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
   121	   - Historical logs preserved in archive but not loaded by default.
   122	
   123	2. No raw log fragment leaks into Agent prompt.
   124	   - Per Art. III.1 屏蔽错误 + DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN.
   125	
   126	3. Permission changes route through ArchitectAI → JudgeAI/VetoAI → canary.
   127	   - Even Autopsy-suggested policy patches must traverse this loop.
   128	
   129	4. Markov override (deep-history read) only on persistent-disease problems.
   130	   - Override requires explicit reason + signature.
   131	```
   132	
   133	Reference: `handover/architect-insights/2026-05-02_flowchart_hashes_and_trace_matrix.md`; directive Part C §11 (TB-15 EvidenceCapsule structure).
   134	
   135	---
   136	
   137	## §5 Update protocol
   138	
   139	```text
   140	1. Every new TB charter MUST add a row to §3 declaring which flowchart
   141	   element(s) it touches. Missing declaration = reject before commit
   142	   (per feedback_tb_phase_tag_required pattern, extended to flowcharts).
   143	
   144	2. At TB ship, validation tests in §4 are checked for the touched flowcharts.
   145	   Failing test = ship-gate violation.
   146	
   147	3. If a TB touches no flowchart element (rare; e.g., pure docs TB), declare
   148	   "—" across all three columns and explain in Notes.
   149	
   150	4. This matrix is updated in the same commit as the TB ship.
   151	   No "I'll update it later" — the matrix is part of the ship gate.
   152	
   153	5. Flowchart hashes are immutable. If the constitution canonical flowcharts
   154	   change, that is a Class 4 sudo event; this matrix is then re-rebased.
   155	```
   156	
   157	---
   158	
   159	## §6 Status
   160	
   161	```text
   162	Created:               2026-05-02
   163	Last TB ship:          TB-7R (commits 55680bb + 46716ae + 17d69de)
   164	Next TB:               TB-8 (Minimal Payout / FinalizeRewardTx) — charter rewritten 2026-05-02
   165	TB-1..TB-7R back-fill: from existing TB_LOG.tsv + commit history; mappings are
   166	                       reconstructed, not original-author-declared.
   167	                       Original authorship of these TBs predates this matrix.
   168	TB-8 forward row:      declared in TB-8 charter rewrite 2026-05-02.
   169	```
   170	
   171	Reconstruction caveat for back-filled rows: TB-1..TB-7R rows in §3 are best-effort declarations based on what each TB demonstrably touched. Original TB charters did not declare flowchart traces. Future TBs will declare directly.

exec
/bin/bash -lc 'sha256sum handover/alignment/TRACE_FLOWCHART_MATRIX.md handover/alignment/TRACE_MATRIX_v0_2026-04-22.md handover/alignment/FC_ELEMENTS_2026-04-22.md constitution.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
3f8c7afba5941f120138cbce525a5f649b43b61bc23c6e6a00ae76abc73f3fa5  handover/alignment/TRACE_FLOWCHART_MATRIX.md
c5133eda002e344e3c21bd6dd375ce78e2eef06d4b0ded2790add745295beb77  handover/alignment/TRACE_MATRIX_v0_2026-04-22.md
8bd1c6e21d9274e9182d81f76f36f4fd7363ca1fe720e95cfeab978384bfb2c2  handover/alignment/FC_ELEMENTS_2026-04-22.md
eec695459c71fbef3685583485deb431fe3b561657b2f285b7c5e7e220e59e03  constitution.md

exec
/bin/bash -lc 'rg -n "flowchart_hash|TRACE_FLOWCHART_MATRIX|FC_ELEMENTS|TRACE_MATRIX_v0|sha256|hash" handover/alignment/TRACE_FLOWCHART_MATRIX.md handover/tracer_bullets/TB-15_charter_2026-05-03.md handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md src/runtime/markov_capsule.rs src/bin/generate_markov_capsule.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bin/generate_markov_capsule.rs:42:    override_set_from_env, scan_unresolved_obs, sha256_of_file, write_markov_capsule, MarkovGenError,
src/bin/generate_markov_capsule.rs:186:    let constitution_hash = sha256_of_file(&args.constitution_path)
src/bin/generate_markov_capsule.rs:189:        "constitution_hash = {}",
src/bin/generate_markov_capsule.rs:190:        hex32(&constitution_hash.0)
src/bin/generate_markov_capsule.rs:228:            "constitution_hash_hex": hex32(&constitution_hash.0),
src/bin/generate_markov_capsule.rs:232:                "1. read constitution.md (verify sha256 == constitution_hash)",
src/bin/generate_markov_capsule.rs:244:            constitution_hash,
src/bin/generate_markov_capsule.rs:251:            sha256: Hash::ZERO,
src/bin/generate_markov_capsule.rs:259:        cap.sha256 = Hash(cid.0);
src/bin/generate_markov_capsule.rs:272:            constitution_hash,
src/runtime/markov_capsule.rs:4://! End-of-TB rollup binding `constitution_hash` + L4 root + L4.E root +
src/runtime/markov_capsule.rs:36:/// `constitution.md` (referenced by `constitution_hash`) + this capsule
src/runtime/markov_capsule.rs:41:/// generation time (constitution_hash from constitution.md, l4_root
src/runtime/markov_capsule.rs:55:    /// zeroed during the hash). Computed by writer.
src/runtime/markov_capsule.rs:63:    /// SG-15.7: capsule must reference constitution hash.
src/runtime/markov_capsule.rs:64:    pub constitution_hash: Hash,
src/runtime/markov_capsule.rs:82:    /// boot context (`{constitution_hash, latest_markov_cid, boot_seq}`).
src/runtime/markov_capsule.rs:88:    pub sha256: Hash,
src/runtime/markov_capsule.rs:100:            constitution_hash: Hash::ZERO,
src/runtime/markov_capsule.rs:107:            sha256: Hash::ZERO,
src/runtime/markov_capsule.rs:116:    /// halt-trigger #2 to pin `constitution_hash` to a known value
src/runtime/markov_capsule.rs:119:    pub fn with_constitution_hash(hash_bytes: [u8; 32]) -> Self {
src/runtime/markov_capsule.rs:121:            constitution_hash: Hash(hash_bytes),
src/runtime/markov_capsule.rs:204:///    `sha256 = Hash::ZERO`. Canonical-encode → sha256 → that's the
src/runtime/markov_capsule.rs:209:/// **CR-15.5**: caller supplies `constitution_hash` / `l4_root` /
src/runtime/markov_capsule.rs:217:    constitution_hash: Hash,
src/runtime/markov_capsule.rs:234:        "constitution_hash_hex": hex(&constitution_hash.0),
src/runtime/markov_capsule.rs:238:            "1. read constitution.md (verify sha256 == constitution_hash)",
src/runtime/markov_capsule.rs:254:    // Step 2: build capsule with capsule_id = 0 + sha256 = 0.
src/runtime/markov_capsule.rs:258:        constitution_hash,
src/runtime/markov_capsule.rs:265:        sha256: Hash::ZERO,
src/runtime/markov_capsule.rs:273:    capsule.sha256 = Hash(cid.0);
src/runtime/markov_capsule.rs:313:pub fn sha256_of_file(path: &std::path::Path) -> Result<Hash, MarkovGenError> {
src/runtime/markov_capsule.rs:322:/// Hex helper for next-session JSON formatting (32-byte hashes).
src/runtime/markov_capsule.rs:346:    /// TB-15 Atom 5 — with_constitution_hash plumbs the hash through.
src/runtime/markov_capsule.rs:348:    fn with_constitution_hash_sets_field() {
src/runtime/markov_capsule.rs:349:        let hash = [0xABu8; 32];
src/runtime/markov_capsule.rs:350:        let c = MarkovEvidenceCapsule::with_constitution_hash(hash);
src/runtime/markov_capsule.rs:351:        assert_eq!(c.constitution_hash.0, hash);
src/runtime/markov_capsule.rs:367:    /// canonical sha256 (with field zeroed for prelim encode).
src/runtime/markov_capsule.rs:380:        let constitution_hash: [u8; 32] = h.finalize().into();
src/runtime/markov_capsule.rs:385:            Hash(constitution_hash),
src/runtime/markov_capsule.rs:398:        assert_eq!(cap.capsule_id.0, cap.sha256.0);
src/runtime/markov_capsule.rs:400:        assert_eq!(cap.constitution_hash.0, constitution_hash);
src/runtime/markov_capsule.rs:484:    /// TB-15 Atom 5 — sha256_of_file matches manual sha256.
src/runtime/markov_capsule.rs:486:    fn sha256_of_file_matches_manual() {
src/runtime/markov_capsule.rs:493:        let computed = sha256_of_file(&path).expect("sha256");
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:54:| 2 | markov_capsule_references_constitution_hash | 5 | GREEN — `MarkovEvidenceCapsule.constitution_hash == sha256(constitution.md)` |
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:70:| SG-15.7 | Markov capsule references constitution hash | `MarkovEvidenceCapsule.constitution_hash` field; halt-trigger #2 |
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:83:- `src/runtime/markov_capsule.rs` (Atom 5): `ObsId` newtype + `MarkovEvidenceCapsule` struct + `with_constitution_hash` constructor + `try_deep_history_read_with_override_check` gate + `override_set_from_env` + `write_markov_capsule` + `scan_unresolved_obs` + `sha256_of_file` + `MarkovGenError`. 8 in-module tests.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:101:### MODIFIED genesis_payload.toml (trust_root rehash chain)
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:115:> TB-15 establishes the Lamarckian Autopsy + Markov EvidenceCapsule substrate. AgentAutopsyCapsule (per-agent, per-event, CAS-resident, AuditOnly) records loss/bankruptcy events derived deterministically from ChainTape evidence — **NEVER from agent LLM self-narration**. The capsule chain is anchored on `EconomicState.agent_autopsies_t[event_id]: Vec<Cid>` (13th sub-field; sequencer-side; NOT projected to `AgentVisibleProjection`). TypicalErrorBroadcast clustering (N≥3 threshold per architect §3.2.3) emits `TypicalErrorSummary` objects embedding `public_summary` text + capsule_id Cids only — never `private_detail_cid` payload bytes. MarkovEvidenceCapsule binds `constitution_hash` + L4 root + L4.E root + CAS root + `previous_capsule_cid` + `typical_errors` + `unresolved_obs` + `next_session_context_cid` into an end-of-TB rollup that becomes the default next-session bootstrap context (FR-15.4). Deep-history reads default-deny without `TURINGOS_MARKOV_OVERRIDE=1` (FR-15.5 + halt-trigger #6).
handover/tracer_bullets/TB-15_charter_2026-05-03.md:16:- **Flowchart 3 (meta)**: `FC3-N43` = `MarkovEvidenceCapsule` schema + `generate_markov_capsule` binary (constitution_hash + L4 root + L4.E root + CAS root + open OBS list + previous_capsule_cid; deterministic). FR-15.4 next-session bootstrap source.
handover/tracer_bullets/TB-15_charter_2026-05-03.md:59:  halt-trigger #2  markov_capsule_references_constitution_hash —
handover/tracer_bullets/TB-15_charter_2026-05-03.md:60:            MarkovEvidenceCapsule.constitution_hash equals sha256 of
handover/tracer_bullets/TB-15_charter_2026-05-03.md:112:      (constitution_hash + L4 root + L4.E root + CAS root + previous
handover/tracer_bullets/TB-15_charter_2026-05-03.md:133:    privacy_policy, sha256, created_at_logical_t, created_at_round
handover/tracer_bullets/TB-15_charter_2026-05-03.md:140:    capsule_id, previous_capsule_cid, constitution_hash,
handover/tracer_bullets/TB-15_charter_2026-05-03.md:143:    sha256, created_at_logical_t
handover/tracer_bullets/TB-15_charter_2026-05-03.md:224:      { "constitution_hash": ..., "latest_markov_cid": ..., "boot_seq":
handover/tracer_bullets/TB-15_charter_2026-05-03.md:276:- `markov_capsule_references_constitution_hash` (SG-15.7; halt #2)
handover/tracer_bullets/TB-15_charter_2026-05-03.md:323:- reads `constitution.md` → `constitution_hash = sha256(bytes)`
handover/tracer_bullets/TB-15_charter_2026-05-03.md:328:- builds `next_session_context_cid` from minimal JSON `{"constitution_hash":..., "latest_markov_cid":..., "boot_seq":[...]}` written to CAS as `ObjectType::NextSessionContext`
handover/tracer_bullets/TB-15_charter_2026-05-03.md:336:- capsule.constitution_hash equals sha256 of constitution.md (SG-15.7)
handover/tracer_bullets/TB-15_charter_2026-05-03.md:339:**Halt triggers green**: #2, #6 (markov_capsule_references_constitution_hash, deep_history_read_without_override_fails).
handover/tracer_bullets/TB-15_charter_2026-05-03.md:346:- latest Markov capsule pointer (Cid hex + previous_cid + constitution_hash + l4_root + l4e_root + cas_root)
handover/tracer_bullets/TB-15_charter_2026-05-03.md:387:2. MarkovEvidenceCapsule does not reference constitution_hash
handover/tracer_bullets/TB-15_charter_2026-05-03.md:405:| SG-15.7 | Markov capsule references constitution hash and flowchart hashes | `tb_15_halt_triggers.rs::markov_capsule_references_constitution_hash` |
handover/alignment/TRACE_FLOWCHART_MATRIX.md:1:# TRACE_FLOWCHART_MATRIX — TB ↔ Constitution Flowchart Mapping
handover/alignment/TRACE_FLOWCHART_MATRIX.md:3:**Authority**: architect directive 2026-05-02, ruling 9 of Part C ("Add TRACE_FLOWCHART_MATRIX.md mapping future TBs to Flowchart 1/2/3").
handover/alignment/TRACE_FLOWCHART_MATRIX.md:5:- Insight summary: `handover/architect-insights/2026-05-02_flowchart_hashes_and_trace_matrix.md`
handover/alignment/TRACE_FLOWCHART_MATRIX.md:13:The lossless constitution integrated edition 2026-05-02 elevated three flowcharts from explanatory diagrams to **SHA256-anchored architectural contracts**. Each TB charter (current + future) must declare which flowchart element(s) it touches; deviation between code behavior and the four canonical hashes is now provably auditable.
handover/alignment/TRACE_FLOWCHART_MATRIX.md:19:## §2 The four canonical flowchart hashes
handover/alignment/TRACE_FLOWCHART_MATRIX.md:81:   - L4 or L4.E entry → CID → CAS → evidence blob → sha256 verifies.
handover/alignment/TRACE_FLOWCHART_MATRIX.md:133:Reference: `handover/architect-insights/2026-05-02_flowchart_hashes_and_trace_matrix.md`; directive Part C §11 (TB-15 EvidenceCapsule structure).
handover/alignment/TRACE_FLOWCHART_MATRIX.md:153:5. Flowchart hashes are immutable. If the constitution canonical flowcharts

exec
/bin/bash -lc 'rg -n "try_deep_history_read_with_override_check|override_set_from_env|TURINGOS_MARKOV_OVERRIDE|DeepHistoryReadDenied|deep-history" src tests handover/tracer_bullets/TB-15_charter_2026-05-03.md handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:58:| 6 | deep_history_read_without_override_fails | 5 | GREEN — `try_deep_history_read_with_override_check(false) → DeepHistoryReadDenied` |
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:67:| SG-15.4 | Deep-history read without override fails | `try_deep_history_read_with_override_check(false)` returns `DeepHistoryReadDenied`; halt-trigger #6 |
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:83:- `src/runtime/markov_capsule.rs` (Atom 5): `ObsId` newtype + `MarkovEvidenceCapsule` struct + `with_constitution_hash` constructor + `try_deep_history_read_with_override_check` gate + `override_set_from_env` + `write_markov_capsule` + `scan_unresolved_obs` + `sha256_of_file` + `MarkovGenError`. 8 in-module tests.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:84:- `src/bin/generate_markov_capsule.rs` (Atom 5): CLI binary with `--tb-id` / `--out-dir` / `--constitution-path` / `--cas-dir` / `--prev-cid-hex` / `--alignment-dir` / `--no-cas` args + `TURINGOS_MARKOV_OVERRIDE` env support.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:115:> TB-15 establishes the Lamarckian Autopsy + Markov EvidenceCapsule substrate. AgentAutopsyCapsule (per-agent, per-event, CAS-resident, AuditOnly) records loss/bankruptcy events derived deterministically from ChainTape evidence — **NEVER from agent LLM self-narration**. The capsule chain is anchored on `EconomicState.agent_autopsies_t[event_id]: Vec<Cid>` (13th sub-field; sequencer-side; NOT projected to `AgentVisibleProjection`). TypicalErrorBroadcast clustering (N≥3 threshold per architect §3.2.3) emits `TypicalErrorSummary` objects embedding `public_summary` text + capsule_id Cids only — never `private_detail_cid` payload bytes. MarkovEvidenceCapsule binds `constitution_hash` + L4 root + L4.E root + CAS root + `previous_capsule_cid` + `typical_errors` + `unresolved_obs` + `next_session_context_cid` into an end-of-TB rollup that becomes the default next-session bootstrap context (FR-15.4). Deep-history reads default-deny without `TURINGOS_MARKOV_OVERRIDE=1` (FR-15.5 + halt-trigger #6).
handover/tracer_bullets/TB-15_charter_2026-05-03.md:69:            history requires TURINGOS_MARKOV_OVERRIDE=1; default-deny path
handover/tracer_bullets/TB-15_charter_2026-05-03.md:70:            logs DeepHistoryReadDenied event. SG-15.4. MUST PASS.
handover/tracer_bullets/TB-15_charter_2026-05-03.md:84:                (constitution+latest_capsule default; deep-history override).
handover/tracer_bullets/TB-15_charter_2026-05-03.md:229:      If TURINGOS_MARKOV_OVERRIDE=1 is set, additional history
handover/tracer_bullets/TB-15_charter_2026-05-03.md:233:        l4_root) returns DeepHistoryReadDenied.
handover/tracer_bullets/TB-15_charter_2026-05-03.md:332:- env: `TURINGOS_MARKOV_OVERRIDE=1` enables deep-history ingest; default-deny otherwise (logs `DeepHistoryReadDenied` to stderr)
handover/tracer_bullets/TB-15_charter_2026-05-03.md:338:- default-deny path returns error on simulated deep-history read without override
handover/tracer_bullets/TB-15_charter_2026-05-03.md:391:6. generate_markov_capsule allows deep-history ingest without `TURINGOS_MARKOV_OVERRIDE=1`
handover/tracer_bullets/TB-15_charter_2026-05-03.md:423:| F | Markov override mechanism | env `TURINGOS_MARKOV_OVERRIDE=1` | Single binary surface; cheap default-deny; documented; matches FR-15.5 framing. |
tests/tb_15_halt_triggers.rs:291:// requires TURINGOS_MARKOV_OVERRIDE=1; default-deny path returns
tests/tb_15_halt_triggers.rs:292:// `MarkovGenError::DeepHistoryReadDenied`. SG-15.4 + FR-15.5.
tests/tb_15_halt_triggers.rs:299:        try_deep_history_read_with_override_check, MarkovGenError,
tests/tb_15_halt_triggers.rs:302:    // Default-deny path: no override; result must be DeepHistoryReadDenied.
tests/tb_15_halt_triggers.rs:303:    let result = try_deep_history_read_with_override_check(false);
tests/tb_15_halt_triggers.rs:305:        Err(MarkovGenError::DeepHistoryReadDenied) => {}
tests/tb_15_halt_triggers.rs:307:            "halt-trigger #6: expected DeepHistoryReadDenied without \
tests/tb_15_halt_triggers.rs:308:             TURINGOS_MARKOV_OVERRIDE=1; got {:?} (SG-15.4 + FR-15.5)",
tests/tb_15_halt_triggers.rs:314:    let ok = try_deep_history_read_with_override_check(true);
tests/tb_15_halt_triggers.rs:317:        "halt-trigger #6: TURINGOS_MARKOV_OVERRIDE=1 must permit deep-history \
tests/fc_alignment_conformance.rs:651:/// deep-history default-deny without override (FR-15.5 + halt-trigger #6).
tests/fc_alignment_conformance.rs:655:        try_deep_history_read_with_override_check, MarkovEvidenceCapsule, MarkovGenError,
tests/fc_alignment_conformance.rs:663:    match try_deep_history_read_with_override_check(false) {
tests/fc_alignment_conformance.rs:664:        Err(MarkovGenError::DeepHistoryReadDenied) => {}
tests/fc_alignment_conformance.rs:665:        other => panic!("FC3-N43: expected DeepHistoryReadDenied; got {other:?}"),
tests/fc_alignment_conformance.rs:667:    assert!(try_deep_history_read_with_override_check(true).is_ok());
src/bin/generate_markov_capsule.rs:13://! `TURINGOS_MARKOV_OVERRIDE=1`. Without override, only the constitution
src/bin/generate_markov_capsule.rs:42:    override_set_from_env, scan_unresolved_obs, sha256_of_file, write_markov_capsule, MarkovGenError,
src/bin/generate_markov_capsule.rs:140:         \x20  TURINGOS_MARKOV_OVERRIDE=1   permit deep-history reads (default-deny)\n\
src/bin/generate_markov_capsule.rs:177:    let override_set = override_set_from_env();
src/bin/generate_markov_capsule.rs:180:            "generate_markov_capsule: TURINGOS_MARKOV_OVERRIDE=1 set — \
src/bin/generate_markov_capsule.rs:181:             deep-history reads ENABLED (audit-only path)."
src/bin/generate_markov_capsule.rs:235:                "4. DEFAULT-DENY deeper history; set TURINGOS_MARKOV_OVERRIDE=1 to override (audit-only)"
src/bin/generate_markov_capsule.rs:283:            MarkovGenError::DeepHistoryReadDenied => {
src/bin/generate_markov_capsule.rs:284:                "deep-history read denied (set TURINGOS_MARKOV_OVERRIDE=1)".to_string()
src/bin/audit_dashboard.rs:1588:    s.push_str("  requires TURINGOS_MARKOV_OVERRIDE=1 (CR-15.6 +\n");
src/bin/audit_dashboard.rs:1960:    /// + latest capsule, with TURINGOS_MARKOV_OVERRIDE=1 required for
src/bin/audit_dashboard.rs:1965:        assert!(s.contains("TURINGOS_MARKOV_OVERRIDE=1"), "missing override env hint");
src/bottom_white/cas/schema.rs:83:    /// bootstrap source; deeper history requires `TURINGOS_MARKOV_OVERRIDE=1`.
src/runtime/mod.rs:56:/// TRACE_MATRIX FC3-N43 (TB-15 Atom 5; architect §6.2 + FR-15.4 + FR-15.5): `MarkovEvidenceCapsule` schema + writer + default-deny deep-history gate. End-of-TB rollup binding constitution_hash + L4 root + L4.E root + CAS root + previous capsule + typical_errors + unresolved_obs + next_session_context_cid. Default next-session bootstrap source; deeper history requires `TURINGOS_MARKOV_OVERRIDE=1`.
src/runtime/markov_capsule.rs:2://! deep-history read gate (architect §6.2 + DECISION_LAMARCKIAN §1).
src/runtime/markov_capsule.rs:7://! FR-15.4. Deeper history requires `TURINGOS_MARKOV_OVERRIDE=1`
src/runtime/markov_capsule.rs:38:/// `TURINGOS_MARKOV_OVERRIDE=1`.
src/runtime/markov_capsule.rs:51:/// `TURINGOS_MARKOV_OVERRIDE=1` per CR-15.6.
src/runtime/markov_capsule.rs:134:    /// Default-deny: caller asked for deep-history read without
src/runtime/markov_capsule.rs:135:    /// `TURINGOS_MARKOV_OVERRIDE=1`. SG-15.4 + halt-trigger #6.
src/runtime/markov_capsule.rs:136:    DeepHistoryReadDenied,
src/runtime/markov_capsule.rs:146:            Self::DeepHistoryReadDenied => write!(
src/runtime/markov_capsule.rs:148:                "deep-history read denied: set TURINGOS_MARKOV_OVERRIDE=1 to enable"
src/runtime/markov_capsule.rs:171:/// default-deny gate for deep-history reads. Returns
src/runtime/markov_capsule.rs:172:/// `Err(DeepHistoryReadDenied)` unless `override_set` is true.
src/runtime/markov_capsule.rs:175:/// `TURINGOS_MARKOV_OVERRIDE=1` by the binary; this helper isolates the
src/runtime/markov_capsule.rs:179:pub fn try_deep_history_read_with_override_check(
src/runtime/markov_capsule.rs:185:        Err(MarkovGenError::DeepHistoryReadDenied)
src/runtime/markov_capsule.rs:190:/// reads `TURINGOS_MARKOV_OVERRIDE` from process env. Used by the
src/runtime/markov_capsule.rs:192:pub fn override_set_from_env() -> bool {
src/runtime/markov_capsule.rs:193:    std::env::var("TURINGOS_MARKOV_OVERRIDE")
src/runtime/markov_capsule.rs:241:            "4. DEFAULT-DENY deeper history; set TURINGOS_MARKOV_OVERRIDE=1 to override (audit-only)"
src/runtime/markov_capsule.rs:354:    /// TB-15 Atom 5 — try_deep_history_read_with_override_check:
src/runtime/markov_capsule.rs:355:    /// false → Err(DeepHistoryReadDenied); true → Ok(()).
src/runtime/markov_capsule.rs:358:        match try_deep_history_read_with_override_check(false) {
src/runtime/markov_capsule.rs:359:            Err(MarkovGenError::DeepHistoryReadDenied) => {}
src/runtime/markov_capsule.rs:360:            other => panic!("expected DeepHistoryReadDenied; got {other:?}"),
src/runtime/markov_capsule.rs:362:        assert!(try_deep_history_read_with_override_check(true).is_ok());

exec
/bin/bash -lc 'rg -n "deep|history|prev|previous_capsule|runtime_repo|L4|l4_root|read_dir|Ledger|transition|cas" src/bin/generate_markov_capsule.rs src/runtime/markov_capsule.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/runtime/markov_capsule.rs:2://! deep-history read gate (architect §6.2 + DECISION_LAMARCKIAN §1).
src/runtime/markov_capsule.rs:4://! End-of-TB rollup binding `constitution_hash` + L4 root + L4.E root +
src/runtime/markov_capsule.rs:5://! CAS root + previous capsule + typical_errors + unresolved_obs +
src/runtime/markov_capsule.rs:7://! FR-15.4. Deeper history requires `TURINGOS_MARKOV_OVERRIDE=1`
src/runtime/markov_capsule.rs:17://! CR-15.6 (Markov default prevents context poisoning).
src/runtime/markov_capsule.rs:21:use crate::bottom_white::cas::schema::{Cid, ObjectType};
src/runtime/markov_capsule.rs:22:use crate::bottom_white::cas::store::CasStore;
src/runtime/markov_capsule.rs:23:use crate::bottom_white::ledger::transition_ledger::canonical_encode;
src/runtime/markov_capsule.rs:37:/// (referenced by `capsule_id`) — no deeper history without
src/runtime/markov_capsule.rs:41:/// generation time (constitution_hash from constitution.md, l4_root
src/runtime/markov_capsule.rs:42:/// from L4 chain head, l4e_root from L4.E chain head, cas_root from CAS
src/runtime/markov_capsule.rs:47:/// **Markov chain**: `previous_capsule_cid` points to the prior capsule
src/runtime/markov_capsule.rs:49:/// {constitution + this capsule}. Deeper history (older capsules; L4
src/runtime/markov_capsule.rs:50:/// rows pre-dating `previous_capsule_cid`'s `l4_root`) requires
src/runtime/markov_capsule.rs:58:    /// Cid of the previous Markov capsule in the chain. `None` for the
src/runtime/markov_capsule.rs:60:    pub previous_capsule_cid: Option<Cid>,
src/runtime/markov_capsule.rs:66:    /// L4 transition_ledger root at generation time.
src/runtime/markov_capsule.rs:67:    pub l4_root: Hash,
src/runtime/markov_capsule.rs:68:    /// L4.E rejection_evidence ledger root at generation time.
src/runtime/markov_capsule.rs:73:    pub cas_root: Hash,
src/runtime/markov_capsule.rs:99:            previous_capsule_cid: None,
src/runtime/markov_capsule.rs:101:            l4_root: Hash::ZERO,
src/runtime/markov_capsule.rs:103:            cas_root: Hash::ZERO,
src/runtime/markov_capsule.rs:134:    /// Default-deny: caller asked for deep-history read without
src/runtime/markov_capsule.rs:137:    Cas(crate::bottom_white::cas::store::CasError),
src/runtime/markov_capsule.rs:148:                "deep-history read denied: set TURINGOS_MARKOV_OVERRIDE=1 to enable"
src/runtime/markov_capsule.rs:150:            Self::Cas(e) => write!(f, "cas: {e}"),
src/runtime/markov_capsule.rs:159:impl From<crate::bottom_white::cas::store::CasError> for MarkovGenError {
src/runtime/markov_capsule.rs:160:    fn from(e: crate::bottom_white::cas::store::CasError) -> Self {
src/runtime/markov_capsule.rs:171:/// default-deny gate for deep-history reads. Returns
src/runtime/markov_capsule.rs:179:pub fn try_deep_history_read_with_override_check(
src/runtime/markov_capsule.rs:194:        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
src/runtime/markov_capsule.rs:209:/// **CR-15.5**: caller supplies `constitution_hash` / `l4_root` /
src/runtime/markov_capsule.rs:210:/// `l4e_root` / `cas_root` / `typical_errors` / `unresolved_obs` —
src/runtime/markov_capsule.rs:215:    cas: &std::sync::Arc<std::sync::RwLock<CasStore>>,
src/runtime/markov_capsule.rs:216:    previous_capsule_cid: Option<Cid>,
src/runtime/markov_capsule.rs:218:    l4_root: Hash,
src/runtime/markov_capsule.rs:220:    cas_root: Hash,
src/runtime/markov_capsule.rs:227:    let mut cas_w = cas
src/runtime/markov_capsule.rs:235:        "previous_markov_cid_hex": previous_capsule_cid.map(|c| c.hex()),
src/runtime/markov_capsule.rs:240:            "3. read CAS<previous_markov_capsule_cid> (if present)",
src/runtime/markov_capsule.rs:241:            "4. DEFAULT-DENY deeper history; set TURINGOS_MARKOV_OVERRIDE=1 to override (audit-only)"
src/runtime/markov_capsule.rs:246:    let next_session_context_cid = cas_w.put(
src/runtime/markov_capsule.rs:257:        previous_capsule_cid,
src/runtime/markov_capsule.rs:259:        l4_root,
src/runtime/markov_capsule.rs:261:        cas_root,
src/runtime/markov_capsule.rs:278:    let _ = cas_w.put(
src/runtime/markov_capsule.rs:299:    for entry in std::fs::read_dir(alignment_dir)? {
src/runtime/markov_capsule.rs:339:        use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
src/runtime/markov_capsule.rs:354:    /// TB-15 Atom 5 — try_deep_history_read_with_override_check:
src/runtime/markov_capsule.rs:357:    fn deep_history_default_deny_works() {
src/runtime/markov_capsule.rs:358:        match try_deep_history_read_with_override_check(false) {
src/runtime/markov_capsule.rs:362:        assert!(try_deep_history_read_with_override_check(true).is_ok());
src/runtime/markov_capsule.rs:369:    fn write_markov_capsule_to_cas_round_trip() {
src/runtime/markov_capsule.rs:374:        let cas = Arc::new(RwLock::new(
src/runtime/markov_capsule.rs:375:            CasStore::open(tmp.path()).expect("cas"),
src/runtime/markov_capsule.rs:383:            &cas,
src/runtime/markov_capsule.rs:402:        let cas_r = cas.read().expect("cas read");
src/runtime/markov_capsule.rs:404:            cas_r.len(),
src/runtime/markov_capsule.rs:419:            let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).unwrap()));
src/runtime/markov_capsule.rs:421:                &cas,
src/runtime/markov_capsule.rs:442:    /// via `previous_capsule_cid`.
src/runtime/markov_capsule.rs:444:    fn markov_chain_links_via_previous_capsule_cid() {
src/runtime/markov_capsule.rs:449:        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).unwrap()));
src/runtime/markov_capsule.rs:452:            &cas,
src/runtime/markov_capsule.rs:460:            "TB-15-prev".into(),
src/runtime/markov_capsule.rs:466:            &cas,
src/runtime/markov_capsule.rs:469:            Hash([0x11u8; 32]), // L4 advanced
src/runtime/markov_capsule.rs:480:        assert_eq!(cap_2.previous_capsule_cid, Some(cap_1.capsule_id));
src/bin/generate_markov_capsule.rs:4://! Reads constitution.md → SHA-256; opens the chain runtime_repo + CAS
src/bin/generate_markov_capsule.rs:5://! to derive L4 / L4.E / CAS roots; scans `handover/alignment/OBS_*.md`
src/bin/generate_markov_capsule.rs:11://! Default-deny: deeper-history reads (older capsules; L4 rows
src/bin/generate_markov_capsule.rs:12://! pre-dating `--prev-cid`'s implied `l4_root`) require
src/bin/generate_markov_capsule.rs:14://! + previous Markov capsule + current chain heads are read.
src/bin/generate_markov_capsule.rs:22://!     --cas-dir <path> \
src/bin/generate_markov_capsule.rs:23://!     [--prev-cid-hex <hex>] \
src/bin/generate_markov_capsule.rs:25://!     [--no-cas]
src/bin/generate_markov_capsule.rs:27://! `--no-cas` runs in pointer-only mode (write JSON file but skip CAS
src/bin/generate_markov_capsule.rs:38:use turingosv4::bottom_white::cas::schema::Cid;
src/bin/generate_markov_capsule.rs:39:use turingosv4::bottom_white::cas::store::CasStore;
src/bin/generate_markov_capsule.rs:51:    /// v0 placeholder — future TB will read L4 chain head from this path.
src/bin/generate_markov_capsule.rs:53:    runtime_repo: Option<PathBuf>,
src/bin/generate_markov_capsule.rs:54:    cas_dir: Option<PathBuf>,
src/bin/generate_markov_capsule.rs:55:    prev_cid_hex: Option<String>,
src/bin/generate_markov_capsule.rs:57:    no_cas: bool,
src/bin/generate_markov_capsule.rs:64:    let mut runtime_repo: Option<PathBuf> = None;
src/bin/generate_markov_capsule.rs:65:    let mut cas_dir: Option<PathBuf> = None;
src/bin/generate_markov_capsule.rs:66:    let mut prev_cid_hex: Option<String> = None;
src/bin/generate_markov_capsule.rs:68:    let mut no_cas = false;
src/bin/generate_markov_capsule.rs:86:                runtime_repo = argv.get(i + 1).map(PathBuf::from);
src/bin/generate_markov_capsule.rs:89:            "--cas-dir" => {
src/bin/generate_markov_capsule.rs:90:                cas_dir = argv.get(i + 1).map(PathBuf::from);
src/bin/generate_markov_capsule.rs:93:            "--prev-cid-hex" => {
src/bin/generate_markov_capsule.rs:94:                prev_cid_hex = argv.get(i + 1).cloned();
src/bin/generate_markov_capsule.rs:101:            "--no-cas" => {
src/bin/generate_markov_capsule.rs:102:                no_cas = true;
src/bin/generate_markov_capsule.rs:117:        runtime_repo,
src/bin/generate_markov_capsule.rs:118:        cas_dir,
src/bin/generate_markov_capsule.rs:119:        prev_cid_hex,
src/bin/generate_markov_capsule.rs:122:        no_cas,
src/bin/generate_markov_capsule.rs:135:         \x20  [--runtime-repo <path>] [--cas-dir <path>] \\\n\
src/bin/generate_markov_capsule.rs:136:         \x20  [--prev-cid-hex <hex>] [--alignment-dir <path>] \\\n\
src/bin/generate_markov_capsule.rs:137:         \x20  [--no-cas]\n\
src/bin/generate_markov_capsule.rs:140:         \x20  TURINGOS_MARKOV_OVERRIDE=1   permit deep-history reads (default-deny)\n\
src/bin/generate_markov_capsule.rs:151:        return Err(format!("--prev-cid-hex must be 64 hex chars; got {}", s.len()));
src/bin/generate_markov_capsule.rs:157:            .map_err(|e| format!("--prev-cid-hex byte {i}: {e}"))?;
src/bin/generate_markov_capsule.rs:181:             deep-history reads ENABLED (audit-only path)."
src/bin/generate_markov_capsule.rs:193:    // Step 2: L4 / L4.E / CAS roots — for v0, accept zero placeholders
src/bin/generate_markov_capsule.rs:194:    // when --runtime-repo/--cas-dir are absent (fresh-repo path) and
src/bin/generate_markov_capsule.rs:197:    let l4_root = Hash::ZERO;
src/bin/generate_markov_capsule.rs:210:    // Step 5: previous capsule Cid.
src/bin/generate_markov_capsule.rs:211:    let previous_capsule_cid: Option<Cid> = match &args.prev_cid_hex {
src/bin/generate_markov_capsule.rs:217:    //   (a) --no-cas: build the capsule struct directly + skip CAS put.
src/bin/generate_markov_capsule.rs:219:    //   (b) default: open `--cas-dir` as a CasStore + put.
src/bin/generate_markov_capsule.rs:220:    let cas_root = Hash::ZERO; // v0 placeholder; future wire-in via CAS metadata digest.
src/bin/generate_markov_capsule.rs:221:    let capsule = if args.no_cas {
src/bin/generate_markov_capsule.rs:222:        eprintln!("generate_markov_capsule: --no-cas mode — JSON pointer only");
src/bin/generate_markov_capsule.rs:224:        use turingosv4::bottom_white::ledger::transition_ledger::canonical_encode;
src/bin/generate_markov_capsule.rs:229:            "previous_markov_cid_hex": previous_capsule_cid.map(|c| c.hex()),
src/bin/generate_markov_capsule.rs:234:                "3. read CAS<previous_markov_capsule_cid> (if present)",
src/bin/generate_markov_capsule.rs:235:                "4. DEFAULT-DENY deeper history; set TURINGOS_MARKOV_OVERRIDE=1 to override (audit-only)"
src/bin/generate_markov_capsule.rs:243:            previous_capsule_cid,
src/bin/generate_markov_capsule.rs:245:            l4_root,
src/bin/generate_markov_capsule.rs:247:            cas_root,
src/bin/generate_markov_capsule.rs:262:        let cas_dir = args
src/bin/generate_markov_capsule.rs:263:            .cas_dir
src/bin/generate_markov_capsule.rs:265:            .ok_or_else(|| "--cas-dir required without --no-cas".to_string())?;
src/bin/generate_markov_capsule.rs:266:        let cas = Arc::new(RwLock::new(
src/bin/generate_markov_capsule.rs:267:            CasStore::open(cas_dir).map_err(|e| format!("open CAS: {e}"))?,
src/bin/generate_markov_capsule.rs:270:            &cas,
src/bin/generate_markov_capsule.rs:271:            previous_capsule_cid,
src/bin/generate_markov_capsule.rs:273:            l4_root,
src/bin/generate_markov_capsule.rs:275:            cas_root,
src/bin/generate_markov_capsule.rs:284:                "deep-history read denied (set TURINGOS_MARKOV_OVERRIDE=1)".to_string()

exec
/bin/bash -lc 'rg -n "register_predicate|unregister_predicate|patch_predicate|register_tool|unregister_tool|PredicateRegistry|ToolRegistry|RiskPolicyRegistry|PredicateRunner" src/runtime/autopsy_capsule.rs tests/tb_15_halt_triggers.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
tests/tb_15_halt_triggers.rs:110:// write_autopsy_capsule signature MUST NOT accept any &mut PredicateRegistry
tests/tb_15_halt_triggers.rs:132:        format!("&mut PredicateRunner"),
tests/tb_15_halt_triggers.rs:133:        format!(".register_predicate("),
tests/tb_15_halt_triggers.rs:134:        format!(".unregister_predicate("),
tests/tb_15_halt_triggers.rs:135:        format!(".patch_predicate("),
tests/tb_15_halt_triggers.rs:136:        format!(".register_tool("),
tests/tb_15_halt_triggers.rs:137:        format!(".unregister_tool("),

exec
/bin/bash -lc 'rg -n "agent_autopsies_t|AutopsyIndex|AgentAutopsyCapsule|private_detail_cid" src/state/q_state.rs tests/tb_15_halt_triggers.rs src/bin/audit_dashboard.rs src/runtime/autopsy_capsule.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/runtime/autopsy_capsule.rs:1://! TB-15 Atom 2 — `AgentAutopsyCapsule` schema + writer (architect §6.2,
src/runtime/autopsy_capsule.rs:10://! Anchored on `EconomicState.agent_autopsies_t[event_id]: Vec<Cid>`
src/runtime/autopsy_capsule.rs:17://! - `private_detail_cid`: opaque CAS Cid; AuditOnly access only;
src/runtime/autopsy_capsule.rs:101:/// - `private_detail_cid`: opaque CAS Cid pointing at
src/runtime/autopsy_capsule.rs:106:pub struct AgentAutopsyCapsule {
src/runtime/autopsy_capsule.rs:107:    /// CAS Cid of the canonical-encoded `AgentAutopsyCapsule` itself.
src/runtime/autopsy_capsule.rs:144:    pub private_detail_cid: Cid,
src/runtime/autopsy_capsule.rs:159:impl Default for AgentAutopsyCapsule {
src/runtime/autopsy_capsule.rs:171:            private_detail_cid: Cid::default(),
src/runtime/autopsy_capsule.rs:180:impl AgentAutopsyCapsule {
src/runtime/autopsy_capsule.rs:235:/// `AgentAutopsyCapsule` to CAS. Flow:
src/runtime/autopsy_capsule.rs:239:///    `ObjectType::AutopsyPrivateDetail`. Cid is `private_detail_cid`.
src/runtime/autopsy_capsule.rs:244:///    as `ObjectType::AgentAutopsyCapsule`.
src/runtime/autopsy_capsule.rs:246:/// Returns the populated `AgentAutopsyCapsule` (with `capsule_id` set).
src/runtime/autopsy_capsule.rs:267:) -> Result<AgentAutopsyCapsule, AutopsyWriteError> {
src/runtime/autopsy_capsule.rs:273:    let private_detail_cid = cas_w.put(
src/runtime/autopsy_capsule.rs:283:    let public_summary = AgentAutopsyCapsule::format_public_summary(
src/runtime/autopsy_capsule.rs:289:    let mut capsule = AgentAutopsyCapsule {
src/runtime/autopsy_capsule.rs:299:        private_detail_cid,
src/runtime/autopsy_capsule.rs:312:    // sha256 filled in) to CAS as the AgentAutopsyCapsule object.
src/runtime/autopsy_capsule.rs:317:        ObjectType::AgentAutopsyCapsule,
src/runtime/autopsy_capsule.rs:337:// `private_detail_cid` payload bytes. Halt-trigger #5 verifies this by
src/runtime/autopsy_capsule.rs:339:// `private_detail_cid` byte sequence.
src/runtime/autopsy_capsule.rs:344:/// `private_detail_cid` bytes are NEVER included (halt-trigger #5).
src/runtime/autopsy_capsule.rs:355:    /// AuditOnly access). NEVER private_detail_cids.
src/runtime/autopsy_capsule.rs:370:/// `private_detail_cid` bytes — only `public_summary` strings +
src/runtime/autopsy_capsule.rs:373:    autopsies: &[AgentAutopsyCapsule],
src/runtime/autopsy_capsule.rs:378:    let mut groups: BTreeMap<String, Vec<&AgentAutopsyCapsule>> = BTreeMap::new();
src/runtime/autopsy_capsule.rs:413:// `EconomicState.agent_autopsies_t` with deterministic Cids) AND by
src/runtime/autopsy_capsule.rs:416:// `(Cid, AgentAutopsyCapsule, private_detail_bytes)` triples.
src/runtime/autopsy_capsule.rs:422:/// §1.1): pure-deterministic derivation of `AgentAutopsyCapsule`s for a
src/runtime/autopsy_capsule.rs:431:/// - dispatch arm: capsule_id population into `agent_autopsies_t`
src/runtime/autopsy_capsule.rs:435:/// identical `Vec<(AgentAutopsyCapsule, Vec<u8>)>` (same Cids, same bytes,
src/runtime/autopsy_capsule.rs:442:) -> Vec<(AgentAutopsyCapsule, Vec<u8>)> {
src/runtime/autopsy_capsule.rs:461:        let private_detail_cid = Cid::from_content(&private_bytes);
src/runtime/autopsy_capsule.rs:463:        let public_summary = AgentAutopsyCapsule::format_public_summary(
src/runtime/autopsy_capsule.rs:470:        let mut capsule = AgentAutopsyCapsule {
src/runtime/autopsy_capsule.rs:480:            private_detail_cid,
src/runtime/autopsy_capsule.rs:487:            .expect("AgentAutopsyCapsule is canonical-encodable");
src/runtime/autopsy_capsule.rs:501:/// dispatch arm already populated into `agent_autopsies_t`).
src/runtime/autopsy_capsule.rs:538:            ObjectType::AgentAutopsyCapsule,
src/runtime/autopsy_capsule.rs:557:        let c = AgentAutopsyCapsule::default();
src/runtime/autopsy_capsule.rs:559:        let back: AgentAutopsyCapsule = canonical_decode(&bytes).expect("decode");
src/runtime/autopsy_capsule.rs:566:        let s = AgentAutopsyCapsule::format_public_summary(
src/runtime/autopsy_capsule.rs:582:        let c = AgentAutopsyCapsule::default();
src/runtime/autopsy_capsule.rs:622:        assert_ne!(cap.private_detail_cid, Cid::default());
src/runtime/autopsy_capsule.rs:638:        let retrieved = cas_r.get(&cap.private_detail_cid).expect("get priv");
src/runtime/autopsy_capsule.rs:649:        let mk = || -> AgentAutopsyCapsule {
src/runtime/autopsy_capsule.rs:674:        assert_eq!(a.private_detail_cid, b.private_detail_cid);
src/runtime/autopsy_capsule.rs:837:    fn mk_autopsy(agent: &str, class: LossReasonClass, priv_byte: u8) -> AgentAutopsyCapsule {
src/runtime/autopsy_capsule.rs:838:        let mut cap = AgentAutopsyCapsule::default();
src/runtime/autopsy_capsule.rs:843:        cap.public_summary = AgentAutopsyCapsule::format_public_summary(
src/runtime/autopsy_capsule.rs:849:        cap.private_detail_cid = Cid([priv_byte; 32]);
src/runtime/autopsy_capsule.rs:904:    /// MUST NOT contain any input private_detail_cid bytes.
src/runtime/autopsy_capsule.rs:922:                     private_detail_cid byte run for byte=0x{:02x}",
src/bin/audit_dashboard.rs:139:    /// counts derived from on-chain `EconomicState.agent_autopsies_t`
src/bin/audit_dashboard.rs:142:    /// `public_summary` strings only — never `private_detail_cid` bytes.
src/bin/audit_dashboard.rs:955:        // EconomicState.agent_autopsies_t at snapshot time. Build_report
src/bin/audit_dashboard.rs:1531:/// pointer ONLY — never `private_detail_cid` payload bytes (CR-15.1 +
src/bin/audit_dashboard.rs:1555:        s.push_str("  (no agent_autopsies_t entries in this snapshot — no\n");
src/bin/audit_dashboard.rs:1558:        s.push_str("  TaskBankruptcyTx yields an empty AutopsyIndex by\n");
src/bin/audit_dashboard.rs:1923:    /// surfaces counts + pointer only; never `private_detail_cid` payload.
src/bin/audit_dashboard.rs:1938:        assert!(!s.contains("private_detail_cid"));
tests/tb_15_halt_triggers.rs:18:// (private_detail_cid contents). Agent_autopsies_t lives on
tests/tb_15_halt_triggers.rs:22:// Filled in by Atom 3 (after EconomicState gains agent_autopsies_t).
tests/tb_15_halt_triggers.rs:155:// AutopsyIndex stores Cids only; the CAS bytes behind private_detail_cid
tests/tb_15_halt_triggers.rs:158:// Filled in by Atom 3 (after EconomicState gains agent_autopsies_t).
tests/tb_15_halt_triggers.rs:162:    // Structural fence: AutopsyIndex value type must remain Vec<Cid>
tests/tb_15_halt_triggers.rs:163:    // (32-byte content addresses), NOT Vec<AgentAutopsyCapsule> (the
tests/tb_15_halt_triggers.rs:165:    // private_detail_cid payload bytes. Even if AgentVisibleProjection
tests/tb_15_halt_triggers.rs:166:    // were ever to surface AutopsyIndex contents (which it does not —
tests/tb_15_halt_triggers.rs:174:    // Locate the AutopsyIndex newtype definition.
tests/tb_15_halt_triggers.rs:178:        .expect("AutopsyIndex newtype must exist in q_state.rs");
tests/tb_15_halt_triggers.rs:185:        .expect("AutopsyIndex newtype must terminate with semicolon");
tests/tb_15_halt_triggers.rs:198:            "halt-trigger #4: AutopsyIndex value type MUST be Vec<Cid>, \
tests/tb_15_halt_triggers.rs:199:             NOT `{}` — agent_autopsies_t stores Cids only; raw bytes \
tests/tb_15_halt_triggers.rs:208:        "halt-trigger #4: AutopsyIndex value type must explicitly be Vec<Cid>; \
tests/tb_15_halt_triggers.rs:220:// private_detail_cid bytes. SG-15.5.
tests/tb_15_halt_triggers.rs:229:        cluster_autopsies, AgentAutopsyCapsule, LossReasonClass,
tests/tb_15_halt_triggers.rs:235:    // distinguishable private_detail_cid bytes.
tests/tb_15_halt_triggers.rs:237:    let mk = |agent: &str, priv_byte: u8| AgentAutopsyCapsule {
tests/tb_15_halt_triggers.rs:250:        private_detail_cid: Cid([priv_byte; 32]),
tests/tb_15_halt_triggers.rs:268:    // private_detail_cid byte run.
tests/tb_15_halt_triggers.rs:276:                 private_detail_cid byte run for byte=0x{:02x} — broadcast \
src/state/q_state.rs:243:    /// loss-emission, accumulates the CAS Cids of `AgentAutopsyCapsule`
src/state/q_state.rs:246:    /// `ObjectType::AgentAutopsyCapsule` (and the audit-only
src/state/q_state.rs:247:    /// `private_detail_cid` lives behind `ObjectType::AutopsyPrivateDetail`).
src/state/q_state.rs:257:    pub agent_autopsies_t: AutopsyIndex,
src/state/q_state.rs:737:/// `AgentAutopsyCapsule` emitted on a loss event (TB-15 v0 sole trigger
src/state/q_state.rs:739:/// the capsule bytes live in CAS behind `ObjectType::AgentAutopsyCapsule`.
src/state/q_state.rs:748:pub struct AutopsyIndex(pub BTreeMap<crate::state::typed_tx::EventId, Vec<crate::bottom_white::cas::schema::Cid>>);
src/state/q_state.rs:852:        // with +agent_autopsies_t (`AutopsyIndex` = `BTreeMap<EventId, Vec<Cid>>`;
src/state/q_state.rs:861:            "EconomicState must have 13 sub-fields post-TB-15 (was 12 post-TB-14; +agent_autopsies_t); got {}",
src/state/q_state.rs:868:        assert!(obj.contains_key("agent_autopsies_t"), "TB-15 agent_autopsies_t sub-field missing");

codex
The Markov side is showing two important audit risks: the deep-history gate exists as a helper but I don’t see it called in the generator’s normal flow, and the shipped first capsule was generated with `--no-cas`, so the pointer artifact is not itself proof of CAS residency. I’m checking whether those were explicitly deferred and whether the trust-root rehashes still line up.
exec
/bin/bash -lc 'rg -n "autopsy_capsule|markov_capsule|generate_markov_capsule|typed_tx|cas/schema|q_state|sequencer|audit_dashboard|fc_alignment_conformance|genesis_payload|Predecessor|superseded|trust_root" genesis_payload.toml handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:8:**Risk class**: Class 2 envelope (self-audit). NO Codex / Gemini round mandated; AgentVisibleProjection not modified; only one new sequencer dispatch hook (TaskBankruptcyTx Step 3.5 + apply_one Stage 3.5).
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:31:cargo test --workspace --lib autopsy_capsule
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:34:cargo test --workspace --lib markov_capsule
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:37:cargo test --workspace --bin audit_dashboard
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:40:cargo test --workspace --test fc_alignment_conformance
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:44:cargo build --bin generate_markov_capsule  → PASS
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:45:cargo run --bin generate_markov_capsule -- ... --no-cas
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:54:| 2 | markov_capsule_references_constitution_hash | 5 | GREEN — `MarkovEvidenceCapsule.constitution_hash == sha256(constitution.md)` |
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:55:| 3 | autopsy_does_not_mutate_predicates | 2 | GREEN — `autopsy_capsule.rs` file-scan: no `&mut PredicateRegistry` etc. |
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:77:- G-15.12 (first Markov capsule generated + persisted) — **GREEN** `handover/evidence/tb_15_markov_capsule_2026-05-03/`.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:82:- `src/runtime/autopsy_capsule.rs` (Atoms 2 + 3 + 4): `LossReasonClass` enum (8 variants) + `AgentAutopsyCapsule` struct + `format_public_summary` + `write_autopsy_capsule` + `derive_autopsies_for_bankruptcy` (PURE) + `write_bankruptcy_autopsies_to_cas` + `cluster_autopsies` + `TypicalErrorSummary` + `AutopsyWriteError`. 15 in-module tests.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:83:- `src/runtime/markov_capsule.rs` (Atom 5): `ObsId` newtype + `MarkovEvidenceCapsule` struct + `with_constitution_hash` constructor + `try_deep_history_read_with_override_check` gate + `override_set_from_env` + `write_markov_capsule` + `scan_unresolved_obs` + `sha256_of_file` + `MarkovGenError`. 8 in-module tests.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:84:- `src/bin/generate_markov_capsule.rs` (Atom 5): CLI binary with `--tb-id` / `--out-dir` / `--constitution-path` / `--cas-dir` / `--prev-cid-hex` / `--alignment-dir` / `--no-cas` args + `TURINGOS_MARKOV_OVERRIDE` env support.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:88:- `src/state/typed_tx.rs`: `+ RiskRuleId(pub String)` opaque newtype.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:89:- `src/bottom_white/cas/schema.rs`: `+ ObjectType::AgentAutopsyCapsule + AutopsyPrivateDetail + MarkovEvidenceCapsule + NextSessionContext` (4 new variants).
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:90:- `src/state/q_state.rs`: `+ AutopsyIndex(BTreeMap<EventId, Vec<Cid>>)` newtype + `agent_autopsies_t: AutopsyIndex` 13th sub-field on EconomicState (serde-default). Sub-field count assertion 12→13.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:91:- `src/state/sequencer.rs`: TaskBankruptcyTx dispatch arm Step 3.5 (pure capsule-Cid derivation → `agent_autopsies_t` insertion) + apply_one Stage 3.5 (CAS write of capsule + private_detail bytes via deterministic helper).
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:92:- `src/runtime/mod.rs`: `+ pub mod autopsy_capsule;` + `+ pub mod markov_capsule;`.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:93:- `src/bin/audit_dashboard.rs`: `+ autopsy_event_counts` + `latest_markov_capsule_cid_hex` fields on `DashboardReport` + `+ read_latest_markov_pointer()` helper + `+ render_section_15(...)` pure render. 4 new SG-15.6 unit tests.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:97:- `tests/q_state_reconstruct.rs`: sub-field count 12→13.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:99:- `tests/fc_alignment_conformance.rs`: + 4 TB-15 witnesses (FC1-N32 / FC1-N33 / FC2-N30 / FC3-N43).
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:101:### MODIFIED genesis_payload.toml (trust_root rehash chain)
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:103:- `src/state/q_state.rs`: `eeb35da8 → c23cc95d` (Atom 3)
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:104:- `src/state/typed_tx.rs`: `44098978 → 665838b0` (Atom 2)
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:105:- `src/state/sequencer.rs`: `1c6ba82f → 9fa59362` (Atom 3)
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:106:- `src/bottom_white/cas/schema.rs`: `70f234ab → 6427695d` (Atom 2)
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:107:- `tests/fc_alignment_conformance.rs`: `751c78c8 → 5e257f27` (Atom 6)
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:110:- `handover/evidence/tb_15_markov_capsule_2026-05-03/`: README + `MARKOV_TB-15_2026-05-03.json` (the genesis Markov capsule) + `LATEST_MARKOV_CAPSULE.txt` (Cid hex pointer `b244f16a1f3bd532d041a40fe39b2b7e7cc12fb58e18b61aedd76a8010eeb1b6`).
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:111:- `handover/markov_capsules/`: same JSON + LATEST pointer (the working location).
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:115:> TB-15 establishes the Lamarckian Autopsy + Markov EvidenceCapsule substrate. AgentAutopsyCapsule (per-agent, per-event, CAS-resident, AuditOnly) records loss/bankruptcy events derived deterministically from ChainTape evidence — **NEVER from agent LLM self-narration**. The capsule chain is anchored on `EconomicState.agent_autopsies_t[event_id]: Vec<Cid>` (13th sub-field; sequencer-side; NOT projected to `AgentVisibleProjection`). TypicalErrorBroadcast clustering (N≥3 threshold per architect §3.2.3) emits `TypicalErrorSummary` objects embedding `public_summary` text + capsule_id Cids only — never `private_detail_cid` payload bytes. MarkovEvidenceCapsule binds `constitution_hash` + L4 root + L4.E root + CAS root + `previous_capsule_cid` + `typical_errors` + `unresolved_obs` + `next_session_context_cid` into an end-of-TB rollup that becomes the default next-session bootstrap context (FR-15.4). Deep-history reads default-deny without `TURINGOS_MARKOV_OVERRIDE=1` (FR-15.5 + halt-trigger #6).
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:119:> Class 2 envelope intact — no AgentVisibleProjection mod, no read-view-authorization mod, single new sequencer dispatch hook (TaskBankruptcyTx Step 3.5 + apply_one Stage 3.5). All 6 halt-triggers + 8 architect SG green. All 4 P-roadmap exit criteria addressed (P4-Exit1/2/3 + P5-Exit1/2 prep). All 4 FC-IDs (FC1-N32 + FC1-N33 + FC2-N30 + FC3-N43) have witness tests.
genesis_payload.toml:10:#   [trust_root]        — SHA-256 of every load-bearing file. Boot
genesis_payload.toml:11:#                         (`turingosv4::boot::verify_trust_root`) recomputes
genesis_payload.toml:42:#     src/main.rs (audit Q2.b VETO — verify_trust_root call site must be
genesis_payload.toml:57:#     tests/fc_alignment_conformance.rs (per CLAUDE.md "每个 ✅ 行 ≥1
genesis_payload.toml:75:# tamper to verify_trust_root would silently bypass the gate;
genesis_payload.toml:92:# verify_trust_root recursively hashes each child file the manifest
genesis_payload.toml:94:# genesis_payload.toml itself is conceptually frozen but
genesis_payload.toml:117:# constitution_hash with [trust_root]["constitution.md"].
genesis_payload.toml:133:[trust_root]
genesis_payload.toml:142:"tests/fc_alignment_conformance.rs" = "5e257f275d86032a957101b7898293fa9d6cdfc13a4bdd2a5f630fb9e940b408"  # rehashed by TB-15 Atom 6 (2026-05-03; architect §6.2 ruling): +4 TB-15 witnesses (FC1-N32 write_autopsy_capsule + FC1-N33 derive_autopsies_for_bankruptcy + FC2-N30 cluster_autopsies + FC3-N43 MarkovEvidenceCapsule + try_deep_history_read_with_override_check default-deny gate). Predecessor TB-14 Atom 6 B′ step 4 hash 751c78c8 superseded.  # rehashed by TB-14 Atom 6 B′ step 4 (2026-05-03; architect ruling §3+§4 — canonical-graph rewire): fc2_n28_mask_set_publication_witness updated to build CanonicalNodeGraph (BTreeMap<TxId, BTreeSet<TxId>>) directly instead of shadow Tape; compute_mask_set new signature (econ, edges, policy, price_index). Predecessor B′ step 1+2 hash c1f14584 superseded.
genesis_payload.toml:143:"src/kernel.rs" = "2f15670e4811ffdbfc386e313b774c116faa5b2797cb4d722232924ef4f146ac"  # rehashed by TB-14 Atom 6 (2026-05-03; closing OBS_TB_12_LEGACY_CPMM_QUARANTINE): excised `markets` / `bounty_market` / `bounty_lp_seed` fields + `create_market` / `buy_yes` / `buy_no` / `yes_price` / `market_ticker` / `market_ticker_full` / `open_bounty_market` / `bounty_yes_price` / `resolve_bounty` / `resolve_all` methods together with `src/prediction_market.rs`. Kernel is now pure topology (V3L-45 docstring contract restored). Pricing lives entirely in `state::compute_price_index` derived view; YES/NO claims live in TB-13 `ConditionalShareBalances`. Predecessor TB-14 Atom 2 hash 0c382679 superseded.
genesis_payload.toml:146:"src/bus.rs" = "2a6cf170fbf182bbc7e4df88da2ff8da28f9347153dbb7016c1c077cc0fe43ea"  # rehashed by TB-14 Atom 6 B′ step 7 R2 closure (2026-05-03; Gemini R2 Q11 architectural-clarity CHALLENGE closure): bus.snapshot() now sets `sequencer_wired: bool` on UniverseSnapshot to disambiguate "sequencer unavailable" (None or q_snapshot failed) from "sequencer running but no canonical positions yet". Both states still produce empty price_index + mask_set (no breaking change for existing consumers); consumers that need to disambiguate read the new field. Per `feedback_audit_obs_bias` cheap-fixes-get-fixed (~15min, not multi-hour future-arch). Predecessor B′ step 4 hash 97ef7917 superseded.  # rehashed by TB-14 Atom 6 B′ step 4 (2026-05-03; architect ruling §3+§4 — canonical-graph rewire): bus.snapshot() now calls `seq.compute_canonical_edges_at_head()` to build the canonical-keyed parent → children edge map (CanonicalNodeGraph) and passes it to compute_mask_set in place of the legacy shadow `kernel.tape`. Closes Codex R1 ship audit primary VETO defect #2 (mask_set non-functional in production due to ID namespace mismatch) for the bus production path; the canonical-graph builder lives in Sequencer per architect §3 binding amend. Predecessor F1 follow-up hash d4550f07 superseded.  # rehashed by TB-14 Atom 6 follow-up (2026-05-03; internal auditor F1 closure): excised dead `BusResult::Invested { node_id: NodeId, shares: f64 }` enum variant. `grep -rn "BusResult::Invested\|Invested {"` confirmed zero call sites and zero match arms (pre-TB-9 invest-path residual). Closes G-14.11 "no f64 in TB-14 module surface" residual flagged by `auditor` subagent on commit 44cd480. This is a SEPARATE follow-up commit — not a within-Atom-6 G2 violation: the Atom 6 commit (44cd480) rehashed bus.rs once; this is the second commit's first rehash. Predecessor Atom-6-main hash b6266820 superseded.  # rehashed by TB-14 Atom 6 (2026-05-03; closing OBS_TB_12_LEGACY_CPMM_QUARANTINE): production wire-swap. (1) Dropped `BusConfig.system_lp_amount: f64` (only consumer was `kernel.create_market`, gone). (2) Removed `init`'s HAYEK_BOUNTY env-gated `kernel.open_bounty_market(lp)` block. (3) Removed per-append `kernel.create_market(node_id, system_lp_amount)` call. (4) `halt_and_settle` no longer calls `kernel.resolve_all` (deleted); fires RunEnd + tool.on_halt only. (5) `snapshot()` rewritten — replaced legacy f64 `markets: HashMap<_, MarketSnapshot>` + `market_ticker: String` with integer-rational `price_index: BTreeMap<TxId, NodeMarketEntry>` + `mask_set: BTreeSet<TxId>` derived via `state::compute_price_index` + `state::compute_mask_set` from `Sequencer::q_snapshot` when wired (sequencer-optional; empty when bus runs ledger-only). bus.rs imports TB-14 types here legitimately — halt-trigger #2 fence (`use`-statement scan) targets `src/state/sequencer.rs` only; bus.rs is the broadcast point. Predecessor TB-9 Atom 4 hash ce86e860 superseded.
genesis_payload.toml:164:"experiments/minif2f_v4/src/bin/evaluator.rs" = "3530a2d5ac98ed8db0d037bc45fda03594c11596850ecb6be71855c6dee3abf0"  # rehashed by TB-14 Atom 6 follow-up (2026-05-03; architect ruling step 1 — Codex R1 primary VETO closure): the canonical TxId returned by `boltzmann_select_parent_v2` MUST NOT flow into legacy shadow `bus.append` parent_id (kernel.tape uses a different (shadow) id namespace; canonical TxId becomes a dangling citation → kernel.append rejects). Per architect ruling: "Use None unless a real shadow id exists." No canonical → shadow id mapping is currently available; pass None (legacy default). The v2 selector still runs (its result captured as `_v2_canonical_pick` for observability + future canonical wire-up — see B′ step 4 CanonicalNodeGraph + parent_tx replacement for last_tx_by_agent), but its output is explicitly NOT passed to bus.append. Closes Codex R1 primary VETO defect #1.  # rehashed by TB-14 Atom 6 (2026-05-03; closing OBS_TB_12_LEGACY_CPMM_QUARANTINE): production wire-swap. (1) Imports updated: `BoltzmannParams + boltzmann_select_parent` (legacy f64) → `boltzmann_select_parent_v2 + BoltzmannMaskPolicy` (integer-rational). (2) `BusConfig` literals (×2) drop `system_lp_amount` field. (3) `let params = BoltzmannParams::from_env()` → `let policy = BoltzmannMaskPolicy::from_env()`. (4) Tick-time logging derives `market_count` + top-N price ticker from `bus.snapshot().price_index` (cross-multiplication argmax; rendered `n/d`, never decimal). (5) Per-tx prompt build derives `market_ticker_str` from `snap.price_index` + queries `prompt_balance` from `bus.sequencer.q_snapshot().economic_state_t.balances_t` (replaces legacy `snap.get_balance`; the `f64` is the `prompt.rs` render contract — `prompt.rs` is not a TB-14 module surface). (6) Boltzmann selector call replaced: `boltzmann_select_parent(&snap.tape, &prices, &params, &mut rng)` → `boltzmann_select_parent_v2(&snap.price_index, &snap.mask_set, &policy, &mut rng).map(|tx| tx.0)`. Predicate-blind (CR-14.1 + halt-trigger #1) — purely a scheduling priority pick. Predecessor TB-12 Atom 0.5(a) hash 8f1b97f1 superseded. (2026-05-03; architect 2026-05-03 ruling §1.1 + §8 Atom 0.5; TB-11 G3/G4 carry-forward closure): added 4 EvidenceCapsule counters (tb11_lean_error_count / tb11_sorry_block_count / tb11_protocol_parse_failure_count / tb11_partial_accept_count) instrumented at existing classify_lean_error / classify_parse_error / step_partial_ok call sites; sorry-block vs lean-error split via err_detail.contains("sorry") || err_detail.contains("forbidden_payload"); MAX_TX exhausted exit point (inside `if let Some(bundle) = chaintape_bundle`, BEFORE bundle.shutdown) now writes EvidenceCapsule to CAS via runtime::evidence_capsule::write_evidence_capsule + emits TerminalSummary on-chain via runtime::adapter::tb11_emit_terminal_summary_for_run with capsule_cid; bundle.shutdown drains the queued TerminalSummary tx via apply_one. Mirror of TB-8 tb8_emit_finalize_after_verify pattern. Predecessor TB-10 Atom 1+3 hash a5b30cd1 superseded.
genesis_payload.toml:206:"src/lib.rs" = "35ff4cd2ed042ab9d92d04f576b11d3cac73e8bc11e4c613886a6a7ff52ebc2f"  # rehashed by TB-14 Atom 6 (2026-05-03; closing OBS_TB_12_LEGACY_CPMM_QUARANTINE): removed `pub mod prediction_market;` together with the legacy CPMM file. Predecessor TB-6 Atom 1.1 hash 196fe13a superseded.
genesis_payload.toml:208:# 2026-05-01 TB-6 Atom 2 — added build_chaintape_sequencer_with_initial_q variant + adapter sub-module declaration.
genesis_payload.toml:218:"src/runtime/mod.rs" = "03fd53582977fe10865eff34824babaf5f8009fd9131f6485e477134a936d9ce"  # rehashed by TB-15 Atom 5 (2026-05-03; architect §6.2 ruling): added `pub mod markov_capsule;` declaration for TB-15 MarkovEvidenceCapsule schema + writer + default-deny gate (FC3-N43). Predecessor TB-15 Atom 2 hash adfc18a4 superseded.
genesis_payload.toml:222:"src/runtime/adapter.rs" = "dcdb2566592e257ff7e3098cdf7ad232060398d2b4a8e4f11505222941c9aabe"  # rehashed by TB-11 Atom 4 (2026-05-02; architect §6.2 ruling): added tb11_emit_terminal_summary_for_run + tb11_emit_expire_for_eligible runtime helpers. Mirror tb8_emit_finalize_after_verify pattern; thin wrappers over Sequencer::emit_system_tx with policy-gated eligibility scan for TaskExpire (state ∈ {Open, Bankrupt} + opened_at_logical_t deadline elapsed + no Finalized claim). Predecessor TB-10 Atom 1 hash 403ff876 superseded.
genesis_payload.toml:228:"tests/tb_6_verify_chaintape.rs" = "ca12d4950899e36e3b946cafd2239e666c03bc2323fb06d88cc35d6cb803274e"  # rehashed by TB-7.7 D7 (2026-05-01): I90 assertion flipped from `!initial_q_state_loaded_from_disk` to `initial_q_state_loaded_from_disk` because build_chaintape_sequencer_with_initial_q now always persists initial_q (even genesis-equivalent) so verify_chaintape replay picks up pre-seeded state. Closes a regression surfaced by smoke run #3.
genesis_payload.toml:234:"src/runtime/run_summary.rs" = "defc4697ec8af2c050aea3bc33beeaaeff8251d4e89d0a8c48803ffac262c58f"  # rehashed by TB-13 Atom 1 (2026-05-03): extract_tx_id match arm extended for 3 new TB-13 variants (CompleteSetMint / CompleteSetRedeem / MarketSeed). Predecessor TB-11 Atom 1 hash 66b934f8 superseded.
genesis_payload.toml:235:"src/runtime/evidence_capsule.rs" = "5266ff43f53cd1a54d3cdbb87e40b886ea4439e109f27514711846f7c84b9e03"  # rehashed by TB-11 Atom 3 (2026-05-02; architect §6.1 ruling): EvidenceCapsule writer added — write_evidence_capsule fn + CapsuleWriteError + 2 round-trip tests. TB-11 MVP stores raw log uncompressed (gzip wrapping deferred to TB-15 Markov Loom). Predecessor Atom 1 hash 7008edce superseded.
genesis_payload.toml:238:# 2026-05-01 TB-8 — `audit_dashboard` CLI (NEW file). Diagnostic dashboard over runtime_repo + cas; composes verify_chaintape + chain_derived_run_facts + agent_keypairs + agent_audit_trail libraries; outputs 7-section text or --json structured report. Per TB-7 charter §13.1 (post-TB-7 sequencing TB-8).
genesis_payload.toml:239:"src/bin/audit_dashboard.rs" = "c953244a5661d08d81feed2109a451cb6f68f880863fc6ba006a9505ae60ed10"  # rehashed by TB-15 Atom 6 (2026-05-03; architect §6.5 SG-15.6): ADDITIVE — added §15 TB-15 Autopsy + Markov render section. (1) New DashboardReport fields: autopsy_event_counts: Vec<(String, u32)> + latest_markov_capsule_cid_hex: Option<String>. (2) New read_latest_markov_pointer() helper reading handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt (best-effort; None on absence). (3) New render_section_15(&[(String, u32)], Option<&str>) -> String pure function. ARCHITECT-MANDATED PRIVACY BANNER: opens with literal "AUTOPSY IS PRIVATE" (architect §6.4 + CR-15.1); SG-15.6 ship gate enforced by 4 tb14_render_tests::sg_15_6_* unit tests asserting (a) banner present, (b) input signature carries only counts + pointer hex (no raw bytes possible), (c) empty Markov pointer renders explicit generation hint, (d) Markov default-deny explanation ("TURINGOS_MARKOV_OVERRIDE=1" + "deeper history") present. (4) §15 invocation appended to render_text after §14. Predecessor TB-14 Atom 6 hash de1f71a1 superseded.  # rehashed by TB-14 Atom 6 (2026-05-03; architect 2026-05-03 ruling §5.1 + §5.5 SG-14.6 closing OBS_TB_12_LEGACY_CPMM_QUARANTINE): ADDITIVE — added §14 TB-14 PriceIndex render section. (1) New imports: `state::{compute_price_index, NodeMarketEntry, TaskId, TxId}`, `state::typed_tx::{NodePosition, PositionKind, PositionSide}`, `state::q_state::{AgentId, EconomicState}`, `economy::money::MicroCoin`. (2) New `DashboardReport.price_index: BTreeMap<TxId, NodeMarketEntry>` field populated by `price_index_from_exposures` helper that synthesizes an `EconomicState` from `exposures: Vec<ExposureRecordRow>` and calls canonical `state::compute_price_index` (no second source-of-truth — architect §5.1; charter §7 auto-resolution A). (3) New `render_section_14(&BTreeMap<TxId, NodeMarketEntry>) -> String` pure function. ARCHITECT-MANDATED BANNER: opens with literal "PRICE IS SIGNAL, NOT TRUTH" (architect §5.1 verbatim); SG-14.6 ship gate enforced by 4 `tb14_render_tests::sg_14_6_*` unit tests asserting (a) the banner is present, (b) prices render as `numerator/denominator` integer-rational (NEVER decimal — charter §5.6 forbidden), (c) empty PriceIndex renders explicit empty-state, (d) None-priced (zero-liquidity) nodes render as `None` per FR-14.3 / halt-trigger #5. (4) §14 invocation appended to `render_text` after §13. Predecessor TB-12 Atom 4 + Atom 6(a) hash 9881aed3 superseded. architect 2026-05-03 §8 Atom 4 + §9.3 SG-12.6): added §13 TB-12 Node exposure records section + ExposureRecordRow struct + L4 walk arms for TypedTx::Work (FirstLong row) + TypedTx::Challenge (ChallengeShort row). Refactored §13 inline render block into `render_section_13(&[ExposureRecordRow]) -> String` pure function for SG-12.6 unit-testability. Added `#[cfg(test)] mod tb12_render_tests::sg_12_6_dashboard_view_positions_works` covering empty + single-FirstLong + same-node-long+short + 2-node aggregation cases + forbidden-token grep (Open market balances / MarketBuy / etc). LABEL DISCIPLINE: "exposure records" NOT "Open market balances" (architect §8 Atom 4). Predecessors TB-11 Atom 5 hash 2e28c2b6 + earlier TB-12 hash f697ba24 superseded.  # rehashed by TB-11 Atom 5 (2026-05-02; architect §6.2 ruling): added §12 TB-11 Epistemic Exhaust + Capital Liberation section. 3 sub-tables: Exhausted runs (TerminalSummaryTx ≡ RunExhausted; run_id, task_id, outcome, attempts, capsule_cid), Expired tasks (TaskExpireTx; task_id, sponsor, refund, reason, @logical_t + total refund line), Bankrupt tasks (TaskBankruptcyTx; task_id, reason, failed_runs, capsule_cid). Privacy mandate footer (capsule public_summary surfaced; raw log shielded behind audit role). Predecessor TB-10 hash d89b2f32 superseded.
genesis_payload.toml:244:# 2026-05-01 TB-7 Atom 6 — chain-backed smoke (synthetic-LLM end-to-end) integration test (NEW file). I110 ship-gate: bootstrap chaintape, submit 3 synthetic-agent WorkTx + VerifyTx pairs through bus.submit_typed_tx, run verify_chaintape (all 7 indicators GREEN — Gates 4 + 5 wired evidence), compute_run_facts_from_chain (Gate 6 round-trip), persist smoke evidence to handover/evidence/tb_7_chaintape_smoke_2026-05-01/. Real-LLM smoke documented as manual procedure in test header.
genesis_payload.toml:260:"src/economy/monetary_invariant.rs" = "3c8469282428d9532993756499790b05d09f7773eab3d2c66da72e1c2ebec5af"  # rehashed by TB-14 Atom 2 (2026-05-03; architect §5.1): doc-comment update only — `price_index_t` (legacy field) bullet replaced with TB-14 `compute_price_index` derived-view note (signal-not-truth; not stored on EconomicState; trivially not in supply sum). No code-path change; conservation invariant unchanged. Predecessor TB-13 Atom 1 hash 4d1182a6 superseded.
genesis_payload.toml:272:"src/bottom_white/cas/schema.rs" = "6427695d8dbb4320fc8f23e64b9595a797cc790c6fb9439e74ba03422e0bb988"  # rehashed by TB-15 Atom 2 (2026-05-03; architect §6.2 ruling): +ObjectType::AgentAutopsyCapsule + AutopsyPrivateDetail + MarkovEvidenceCapsule + NextSessionContext variants for the TB-15 CAS-resident autopsy + Markov substrate. Predecessor TB-11 Atom 1 hash 70f234ab superseded.
genesis_payload.toml:276:"src/state/mod.rs" = "6d9a9c7dd1e63b04742d0b913cc2200dc9aefff9bddb600e077ca79d855d77d1"  # rehashed by TB-14 Atom 6 B′ step 4 (2026-05-03; architect ruling §3+§4 — canonical-graph rewire): pub use price_index::CanonicalNodeGraph re-export added. Predecessor TB-14 Atom 3 hash 8b77f533 superseded. (2026-05-03; architect §5.5 + charter §3 Atom 3; FC2-N28): pub use price_index::{compute_mask_set, BoltzmannMaskPolicy} added (Atom 4 will add from_env loader on existing struct). Predecessor TB-14 Atom 2 hash e34412f6 superseded.
genesis_payload.toml:277:"src/state/price_index.rs" = "a3b770d7715f047bf58368b223a9b746ed5c7ddd66377fd47f2322ff2fe2ff84"  # rehashed by TB-14 Atom 6 B′ step 4 (2026-05-03; architect ruling §3+§4 — canonical-graph rewire): (1) NEW `pub type CanonicalNodeGraph = BTreeMap<TxId, BTreeSet<TxId>>` (canonical-keyed parent → children edge map; replaces legacy shadow Tape consumption per Codex R1 VETO closure). (2) compute_mask_set signature change: `(econ, &Tape, policy, &price_index)` → `(econ, &CanonicalNodeGraph, policy, &price_index)`. Body rewritten — `tape.children(parent_id.0.as_str())` → `edges.get(parent_id).iter()`. Doc comments updated to reflect canonical-graph semantics + post-B′-step-4 invariant envelope. Closes Codex R1 ship audit primary VETO defect #2 at the price_index module surface. Predecessor B′ step 2 (env-validation) hash d642f709 superseded.  # rehashed by TB-14 Atom 6 follow-up (2026-05-03; architect ruling step 2 — Codex R1 secondary CHALLENGE closure): from_env() now validates each field at parse time. Rules: min_liquidity > 0 (non-positive → default); price_margin numerator+denominator both > 0 (zero either → default); beta_den > 0 (zero → default); beta_num >= 0 (negative → default); epsilon ∈ [0, 1] (den > 0 AND num ≤ den; otherwise → default pair). Fail-soft per Art.I.1 + C-027 — invalid env value falls back to that field's Default; misconfiguration must not crash the swarm but production semantics must remain inside the architect-mandated invariant envelope. +11 inline tests pinning each rule (negative/zero boundary checks per field; epsilon ≥ 0 and ≤ 1 boundaries explicitly accepted). Predecessor TB-14 Atom 4 hash 2a5c2fa7 superseded.  # rehashed by TB-14 Atom 4 (2026-05-03; charter §3 Atom 4): +BoltzmannMaskPolicy::from_env() constructor reading 7 integer env vars (BOLTZMANN_BETA_NUM/DEN, MIN_LIQUIDITY_MICRO, PRICE_MARGIN_NUM/DEN, EPSILON_NUM/DEN); fail-soft on parse error per Art.I.1 + C-027. +6 inline tests (default, no-vars, override-each-field, invalid-fallback, serde-round-trip, no-decimal-point JSON) using static Mutex per feedback_env_var_test_lock. Predecessor TB-14 Atom 3 hash 23ec5f61 superseded.
genesis_payload.toml:278:"src/state/q_state.rs" = "c23cc95d409cdebb0b91cc0b7b75aee9944d3397a5b4e014a63d11e7b248c614"  # rehashed by TB-15 Atom 3 (2026-05-03; architect §6.2 ruling): +AutopsyIndex(BTreeMap<EventId, Vec<Cid>>) newtype + agent_autopsies_t: AutopsyIndex 13th sub-field on EconomicState with #[serde(default)] for backward-compat with pre-TB-15 chain snapshots. Sequencer-side per-event Cid index; capsule bytes live in CAS; NOT projected to AgentVisibleProjection per CR-15.1 + halt-trigger #1. Sub-field count assertion 12→13. Predecessor TB-14 Atom 3 hash eeb35da8 superseded.
genesis_payload.toml:279:"src/state/sequencer.rs" = "9fa5936230738e5f72b14713a96238ad53583c08cdd6207d8832d27b009f50a9"  # rehashed by TB-15 Atom 3 (2026-05-03; architect §6.2 ruling): TaskBankruptcyTx dispatch arm Step 3.5 — pure derive_autopsies_for_bankruptcy(&q.economic_state_t, bk, q.q_t.current_round, bk.timestamp_logical) populates q_next.economic_state_t.agent_autopsies_t[event_id] with deterministic Cids per losing-staker. apply_one Stage 3.5 — write_bankruptcy_autopsies_to_cas() side-effect writes the capsule + private_detail bytes to CAS using the same deterministic helper (replay-safe identical Cids). NO change to monetary invariants (assert_no_post_init_mint + assert_total_ctf_conserved unchanged). NO predicate registry mutation (CR-15.3 + halt-trigger #3). NOT projected to AgentVisibleProjection (CR-15.1 + halt-trigger #1). Predecessor TB-14 Atom 6 B′ step 4 hash 1c6ba82f superseded.  # rehashed by TB-14 Atom 6 B′ step 4 (2026-05-03; architect ruling §3+§4 — canonical-graph builder): NEW `Sequencer::compute_canonical_edges_at_head() -> BTreeMap<TxId, BTreeSet<TxId>>` walks L4 + reads CAS-resident ProposalTelemetry.parent_tx for each accepted WorkTx → builds canonical parent → children edge map. Replay-deterministic per Art.0.2 (L4 + CAS payloads both replay-deterministic). Empty fallback on lock poisoned / CAS missing / decode error / no parent_tx. **Halt-trigger #2 fence preserved**: the new method body uses `crate::bottom_white::ledger::transition_ledger::canonical_decode` + `crate::runtime::proposal_telemetry::read_from_cas` (both NON-TB-14 modules — TB-3 + TB-7 surface respectively). NO TB-14 imports added to sequencer.rs `use` block. The forbidden tokens (compute_price_index / compute_mask_set / NodeMarketEntry / RationalPrice / BoltzmannMaskPolicy) remain absent from sequencer.rs. Predecessor TB-13 Atom 6 round-5 hash 2b8a1b36 superseded. (2026-05-03; Codex RQ5 remediation): drop redundant inner-consistency check `redeem.outcome != redeem.resolution_ref.claimed_outcome` from CompleteSetRedeem dispatch — ResolutionRef wrapper removed in typed_tx.rs (claimed_outcome was a redundant copy of redeem.outcome; resolution_tx_id was opaque/L4-unvalidated dead wire weight). Step 1 deleted; remaining steps renumbered 1..6. State-mismatch path (Finalized+No / Bankrupt+Yes → InvalidResolutionRef) preserved via existing match arm using redeem.outcome directly. Predecessor TB-12 Atom 2 hash be6a3ee8 superseded.  # rehashed by TB-12 Atom 2 (2026-05-03; architect 2026-05-03 ruling §3 + §8 Atom 2; Class 3): WorkTx accept arm + ChallengeTx accept arm extended with **pure additive** NodePosition side-effect — accepted WorkTx with stake>0 → FirstLong NodePosition write; accepted ChallengeTx with stake>0 → ChallengeShort NodePosition write (task_id Q-derived from target stakes_t entry). VerifyTx accept arm UNCHANGED (FR-12.3 + CR-12.8: VerifyTx.bond is responsibility bond, not market side). NO change to balances_t / stakes_t / challenge_cases_t / total_supply (CR-12.1 + CR-12.2 — NodePosition.amount NOT a Coin holding, NOT in 5-holding CTF sum). Predecessor TB-11 Atom 2 hash 2d861c68 superseded.  # rehashed by TB-11 Atom 1 (2026-05-02; architect §6.2 ruling): system_message_for_verification + system_signature_of + system_epoch_of arms extended for TaskBankruptcy; submit_agent_tx ingress fail-closed extended; dispatch_transition stub arm for TaskBankruptcy (full body lands in Atom 2); test fixture initializers updated for additive TaskExpire/TerminalSummary fields. Predecessor TB-8 round-2 hash b945cc07 superseded.
genesis_payload.toml:280:"src/state/typed_tx.rs" = "665838b02191345dedf7d312f0b33cb5214dad7475acb084bb3ac12245b3268c"  # rehashed by TB-15 Atom 2 (2026-05-03; architect §6.2 ruling): +RiskRuleId(pub String) opaque newtype for AgentAutopsyCapsule.violated_risk_rule: Option<RiskRuleId> field. CR-15.3 — opaque rule identifier; autopsy never depends on live registry. Predecessor TB-13 Atom 6 round-5 hash 44098978 superseded.  # rehashed by TB-13 Atom 6 round-5 (2026-05-03; Codex RQ5 remediation): drop ResolutionRef wrapper struct (incl. resolution_tx_id + claimed_outcome) from CompleteSetRedeemTx + CompleteSetRedeemSigningPayload. Both fields were dead — resolution_tx_id was opaque/L4-unvalidated (per round-4 doc-fix), claimed_outcome was a redundant copy of redeem.outcome used only for an inner-consistency check that was itself dead defense-in-depth (signature covers both; tampering breaks signature). Net: CompleteSetRedeemTx 9→8 fields; signing payload 8→7 fields. Inner-consistency check removed from sequencer (Step 1); state-mismatch path preserved via existing match arm. InvalidResolutionRef variant + Display preserved (now reads "outcome does not match state"). Wire-format break — no production rows yet. Predecessor TB-12 Atom 1 hash 11e526a1 superseded.  # rehashed by TB-12 Atom 1 (2026-05-03; architect 2026-05-03 ruling §3 + §8 Atom 1): +PositionSide enum +PositionKind enum +NodePosition struct (immutable exposure record per architect §10; NOT Coin holding per CR-12.1). FORBIDDEN: MarketBuy/MarketSell PositionKind variants (TB-13+ trading layer). Predecessor TB-11 Atom 1 hash c30b814a superseded.  # rehashed by TB-11 Atom 1 (2026-05-02; architect §6.1+6.2 ruling): TerminalSummaryTx additive bump (+parent_state_root +solver_agent +evidence_capsule_cid; type alias `RunExhaustedTx = TerminalSummaryTx`); TaskExpireTx additive bump (+sponsor_agent +escrow_tx_id +reason); NEW TaskBankruptcyTx struct + signing payload + domain prefix + canonical-message variant; NEW 4 enums (ExpireReason / BankruptcyReason / ExhaustionReason / CapsulePrivacyPolicy); golden digest constants rotated for TaskExpire + TerminalSummary (per file-internal rotation protocol). Predecessor TB-8 Atom 3 hash 26b01d8b superseded.
genesis_payload.toml:346:"src/bottom_white/ledger/transition_ledger.rs" = "2e1c406490a2a7b11aa1dc5992ad5a990b9aae5d71ebfc01f5352dbe0503a79a"  # rehashed by TB-13 Atom 6 round-5 (2026-05-03; Codex RQ5 doc-coupled remediation): TxKind::CompleteSetRedeem doc-comment updated — was "Validates `resolution_ref` against L4 (must be TaskBankruptcy for No or FinalizeReward for Yes)" (drift from actual sequencer behavior); now correctly states resolution authority is `task_markets_t[event_id.0].state` (Finalized→Yes / Bankrupt→No). Code-side TxKind discriminants UNCHANGED. Predecessor TB-13 Atom 1 hash 66f82c8d superseded.  # rehashed by TB-13 Atom 1 (2026-05-03; architect 2026-05-03 post-TB-12 ruling Part A §4.3): +TxKind::CompleteSetMint=11 +TxKind::CompleteSetRedeem=12 +TxKind::MarketSeed=13 (additive variants; #[repr(u8)] explicit discriminants; forward-safe wire bump). Predecessor TB-11 Atom 1 hash 4a17875c superseded.

exec
/bin/bash -lc "nl -ba genesis_payload.toml | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TuringOS v4 — Genesis payload (Phase B7).
     2	#
     3	# Frozen at Phase B B7 (2026-04-25). Two binding sections:
     4	#
     5	#   [pput_accounting_0] — semantic invariants of the PPUT measure
     6	#                         (PREREG § 1.8). baseline_regression_rate +
     7	#                         baseline_regression_jsonl_sha256 are placeholders
     8	#                         until B7-extra (p_0 calibration) lands.
     9	#
    10	#   [trust_root]        — SHA-256 of every load-bearing file. Boot
    11	#                         (`turingosv4::boot::verify_trust_root`) recomputes
    12	#                         each hash and aborts with TRUST_ROOT_TAMPERED on
    13	#                         mismatch.
    14	#
    15	# Manifest derivation (independently re-derived in B7 from PREREG § 1.8 +
    16	# B2-B4 mid-term audit recommendation + B6 prompt_guard add):
    17	#
    18	#   PREREG § 1.8 base (8):
    19	#     src/kernel.rs, src/wal.rs, src/bus.rs,
    20	#     experiments/minif2f_v4/src/lean4_oracle.rs,
    21	#     constitution.md, cases/MANIFEST.sha256 (proxy for cases/*.yaml glob),
    22	#     handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json,
    23	#     handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md
    24	#
    25	#   Mid-term audit add (PPUT accounting layer, 6):
    26	#     src/drivers/llm_http.rs (cost source of truth),
    27	#     experiments/minif2f_v4/src/cost_aggregator.rs (B2),
    28	#     experiments/minif2f_v4/src/wall_clock.rs (B3),
    29	#     experiments/minif2f_v4/src/post_hoc_verifier.rs (B4),
    30	#     experiments/minif2f_v4/src/jsonl_schema.rs (B1),
    31	#     experiments/minif2f_v4/src/bin/evaluator.rs (the wiring)
    32	#
    33	#   B6 add (1):
    34	#     src/sdk/prompt_guard.rs (PPUT-context-leak runtime gate)
    35	#
    36	#   B7-extra add (1):
    37	#     experiments/minif2f_v4/src/rollback_sim.rs (PPUT-CCL § 5.5
    38	#       calibration treatment toggle — synthetic ∏p=0 from tx 50,
    39	#       constitutionally FC1-E18 + FC2-N22-MaxTxExhausted)
    40	#
    41	#   2026-04-25 dual-audit fixes (4):
    42	#     src/main.rs (audit Q2.b VETO — verify_trust_root call site must be
    43	#       in manifest; comment-out = silent bypass)
    44	#     Cargo.lock (audit Q2.e VETO — supply-chain dep-version swap defense)
    45	#     handover/preregistration/scripts/run_p0_calibration.sh (audit Q2 +
    46	#       DO-178C tool qualification — runner is a frozen production tool)
    47	#     handover/preregistration/scripts/compute_p0.py (same — estimator is
    48	#       a frozen production tool)
    49	#
    50	#   2026-04-25 Phase A0 harness modernization (4):
    51	#     rules/MANIFEST.sha256 (proxy for 15 rules/active/R-*.yaml as of A8e12 — governance
    52	#       rules that must not be silently weakened; per case C-075 DO-178C
    53	#       tool qualification)
    54	#     rules/engine.py (rule engine; tampering = silent rule bypass)
    55	#     .claude/hooks/judge.sh (PreToolUse hook that invokes engine.py +
    56	#       implements R-016 fc_trace_in_commit + constitution.md special-case)
    57	#     tests/fc_alignment_conformance.rs (per CLAUDE.md "每个 ✅ 行 ≥1
    58	#       witness test"; tampering = silent constitutional drift, defeats
    59	#       FC1/2/3 ↔ symbol mapping enforcement)
    60	#
    61	#   2026-04-25 Phase A1 PREREG amendment (1):
    62	#     handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md
    63	#       (defers PREREG § 5.5 calibration; substitutes p_0 = 0.10
    64	#       max-tolerated ceiling — least-strict admissible per amendment
    65	#       § 2 wording correction A8e fix F6; per case C-073 ArchitectAI
    66	#       commit authority)
    67	#
    68	# Total: 43 files. Progression A0=24 → A1=25 (PREREG amendment) → A3=26
    69	# (agent_models.rs) → A5=27 (budget_regime.rs) → A6=28 (fc_trace.rs) →
    70	# A7=31 (llm_proxy.py + 2 smoke scripts) → A8e=33 (run_id.rs +
    71	# test_llm_proxy.py) → A8e2=34 (llm_proxy_python_conformance.rs Rust
    72	# wrapper) → A8e7=35 (A8_AUDIT_HISTORY append-only chronology) →
    73	# A8e11=37 (audit runner scripts) → A8e13=38 (src/boot.rs — the
    74	# verifier file itself; pre-A8e13 it wasn't Trust-Rooted, so a
    75	# tamper to verify_trust_root would silently bypass the gate;
    76	# Codex R11#1 caught + C-075 ratifies) → C-pre1=40 (hard-10 sample
    77	# JSON + draw script — Phase C C2 sample basis, immutable per PREREG
    78	# § 6 C2; tampering = Phase C respin per C-075 DO-178C tool
    79	# qualification) → C1a=41 (experiment_mode.rs — Phase C ablation
    80	# mode resolver + UnimplementedMode startup-fatal gate; tampering
    81	# could weaken the gate that prevents misconfigured --mode runs from
    82	# burning budget under the wrong constitutional regime) → C2
    83	# runner=42 (run_c2_phase_c_ablation.sh — Phase C batch runner per
    84	# C-075 DO-178C; tampering with the cell ordering, timeout, or
    85	# synthetic-failure-row policy could silently corrupt the 100-row
    86	# ablation evidence collection) → C3 analyzer=43
    87	# (analyze_c3_h1_h4.py — H1-H4 McNemar paired sign test +
    88	# Holm-Bonferroni decision per PREREG § 9; tampering could
    89	# weaken the inferential-family gate that mints the Phase C
    90	# rejection decisions). Per A8e13 Q1 the
    91	# {cases,rules}/MANIFEST.sha256 entries are now ENFORCED proxies:
    92	# verify_trust_root recursively hashes each child file the manifest
    93	# claims (was convention-only before).
    94	# genesis_payload.toml itself is conceptually frozen but
    95	# not self-hashed (chicken-and-egg) — the [pput_accounting_0] section
    96	# values are the semantic anchor.
    97	#
    98	# Tampering with any tracked file → Boot aborts with TRUST_ROOT_TAMPERED.
    99	
   100	[pput_accounting_0]
   101	schema_version = "1.0"
   102	progress_definition = "1 iff Lean ground-truth verifies golden_path_payload"
   103	cost_definition = "sum(prompt_tokens + completion_tokens + tool_tokens) over all proposals in the run"
   104	time_definition = "wall_clock from first agent prompt construction to final Lean accept or external timeout"
   105	verified_predicate = "experiments/minif2f_v4/src/lean4_oracle.rs::verify_omega_detailed"
   106	heldout_sealed_hash = "51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b"
   107	source_pool_sha256 = "77179cf2598b0df707d78a6663e763121dfe8e73a6538073155f488feab95093"
   108	baseline_regression_rate = 0.0  # PLACEHOLDER until Phase B7-extra calibration
   109	baseline_regression_jsonl_sha256 = ""  # PLACEHOLDER until Phase B7-extra
   110	k_max = 10
   111	n_max = 34
   112	
   113	# 2026-04-27 CO1.0 — minimal-with-anchor genesis schema per
   114	# `handover/specs/GENESIS_MINIMAL_WITH_ANCHOR_v1_2026-04-27.md`. 8 content-anchored fields.
   115	# v4 first iteration: placeholder values for fields awaiting user PGP/SSH ceremony or future
   116	# predicate definition. boot::verify_constitution_root validates format + cross-references
   117	# constitution_hash with [trust_root]["constitution.md"].
   118	[constitution_root]
   119	constitution_hash = "eec695459c71fbef3685583485deb431fe3b561657b2f285b7c5e7e220e59e03"
   120	creator_signature = "PENDING_USER_PGP_SSH_SIGNATURE_v4_FIRST_ENACTMENT"
   121	signed_at = "2026-04-27T08:00:00+00:00"
   122	schema_version = 1
   123	# amendment_predicate_hash placeholder: sha256("") = empty hash (no predicate yet; v4.1+ defines).
   124	amendment_predicate_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
   125	# Empty Merkle tree roots = sha256(""); these populate via v1.x atom commits as predicates/tools register.
   126	initial_predicate_registry_root = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
   127	initial_tool_registry_root = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
   128	# Self-referential boot attestation: sha256 of canonical concat of fields above.
   129	# Computed: cat constitution_hash creator_signature signed_at schema_version amendment_predicate_hash initial_predicate_registry_root initial_tool_registry_root | sha256sum
   130	# v4 first iteration: PERMISSIVE (boot.rs verifies format + cross-ref; full self-ref deferred to v4.x).
   131	boot_attestation_hash = "PENDING_v4_X_SELF_REFERENTIAL_COMPUTE"
   132	
   133	[trust_root]
   134	"src/main.rs" = "622fee2d96a980d24f9fbaab3d0531c195a0a337fc3ddd2efb60bca90a1cfbf9"
   135	"Cargo.lock" = "e1afff633f71ce6fd94240c0102449bcb869978af826e59380e844323373f405"
   136	"Cargo.toml" = "d9cb276d49b0afd1ac9246057fc86b84d18a10dc64ca50be5f6f0959bb63efb7"
   137	"handover/preregistration/scripts/run_p0_calibration.sh" = "5f4a57dd8b8280ffe04bec89350a57d876d06cc179d9f8841a522e7bdcf1b8b7"
   138	"handover/preregistration/scripts/compute_p0.py" = "fd1dff97d582f4f509249852be500cba7d81efc9bab8fcb9872b2d85bb4e3052"
   139	"rules/MANIFEST.sha256" = "6d6fd54d46352213a857fdaf5cf70e8a893bbbd5467e5da45a3ed3314d01b0c2"
   140	"rules/engine.py" = "e29f062b27fa0bb486a99876438dc096f952da0f358efdca40579592c5d97713"  # rehashed by CO1.13.2 (2026-04-29): 5-line early-continue patch for trigger==pre_commit (R-022 bypass); FC3-N34 readonly verification
   141	".claude/hooks/judge.sh" = "f1f8f5094057dcea3c345951f7e73fd4d269f074a407829f19e4d75830a55fc2"
   142	"tests/fc_alignment_conformance.rs" = "5e257f275d86032a957101b7898293fa9d6cdfc13a4bdd2a5f630fb9e940b408"  # rehashed by TB-15 Atom 6 (2026-05-03; architect §6.2 ruling): +4 TB-15 witnesses (FC1-N32 write_autopsy_capsule + FC1-N33 derive_autopsies_for_bankruptcy + FC2-N30 cluster_autopsies + FC3-N43 MarkovEvidenceCapsule + try_deep_history_read_with_override_check default-deny gate). Predecessor TB-14 Atom 6 B′ step 4 hash 751c78c8 superseded.  # rehashed by TB-14 Atom 6 B′ step 4 (2026-05-03; architect ruling §3+§4 — canonical-graph rewire): fc2_n28_mask_set_publication_witness updated to build CanonicalNodeGraph (BTreeMap<TxId, BTreeSet<TxId>>) directly instead of shadow Tape; compute_mask_set new signature (econ, edges, policy, price_index). Predecessor B′ step 1+2 hash c1f14584 superseded.
   143	"src/kernel.rs" = "2f15670e4811ffdbfc386e313b774c116faa5b2797cb4d722232924ef4f146ac"  # rehashed by TB-14 Atom 6 (2026-05-03; closing OBS_TB_12_LEGACY_CPMM_QUARANTINE): excised `markets` / `bounty_market` / `bounty_lp_seed` fields + `create_market` / `buy_yes` / `buy_no` / `yes_price` / `market_ticker` / `market_ticker_full` / `open_bounty_market` / `bounty_yes_price` / `resolve_bounty` / `resolve_all` methods together with `src/prediction_market.rs`. Kernel is now pure topology (V3L-45 docstring contract restored). Pricing lives entirely in `state::compute_price_index` derived view; YES/NO claims live in TB-13 `ConditionalShareBalances`. Predecessor TB-14 Atom 2 hash 0c382679 superseded.
   144	"src/wal.rs" = "1ac7637aa09531b1c7232163f5df48f7193251594c4ed20e0d0fc85cea8f84dc"
   145	"src/boot.rs" = "92d39c2b40e3104bcb76f5a1e7fad72aa608ad8ce2f9564a6d01a70fa64093b6"
   146	"src/bus.rs" = "2a6cf170fbf182bbc7e4df88da2ff8da28f9347153dbb7016c1c077cc0fe43ea"  # rehashed by TB-14 Atom 6 B′ step 7 R2 closure (2026-05-03; Gemini R2 Q11 architectural-clarity CHALLENGE closure): bus.snapshot() now sets `sequencer_wired: bool` on UniverseSnapshot to disambiguate "sequencer unavailable" (None or q_snapshot failed) from "sequencer running but no canonical positions yet". Both states still produce empty price_index + mask_set (no breaking change for existing consumers); consumers that need to disambiguate read the new field. Per `feedback_audit_obs_bias` cheap-fixes-get-fixed (~15min, not multi-hour future-arch). Predecessor B′ step 4 hash 97ef7917 superseded.  # rehashed by TB-14 Atom 6 B′ step 4 (2026-05-03; architect ruling §3+§4 — canonical-graph rewire): bus.snapshot() now calls `seq.compute_canonical_edges_at_head()` to build the canonical-keyed parent → children edge map (CanonicalNodeGraph) and passes it to compute_mask_set in place of the legacy shadow `kernel.tape`. Closes Codex R1 ship audit primary VETO defect #2 (mask_set non-functional in production due to ID namespace mismatch) for the bus production path; the canonical-graph builder lives in Sequencer per architect §3 binding amend. Predecessor F1 follow-up hash d4550f07 superseded.  # rehashed by TB-14 Atom 6 follow-up (2026-05-03; internal auditor F1 closure): excised dead `BusResult::Invested { node_id: NodeId, shares: f64 }` enum variant. `grep -rn "BusResult::Invested\|Invested {"` confirmed zero call sites and zero match arms (pre-TB-9 invest-path residual). Closes G-14.11 "no f64 in TB-14 module surface" residual flagged by `auditor` subagent on commit 44cd480. This is a SEPARATE follow-up commit — not a within-Atom-6 G2 violation: the Atom 6 commit (44cd480) rehashed bus.rs once; this is the second commit's first rehash. Predecessor Atom-6-main hash b6266820 superseded.  # rehashed by TB-14 Atom 6 (2026-05-03; closing OBS_TB_12_LEGACY_CPMM_QUARANTINE): production wire-swap. (1) Dropped `BusConfig.system_lp_amount: f64` (only consumer was `kernel.create_market`, gone). (2) Removed `init`'s HAYEK_BOUNTY env-gated `kernel.open_bounty_market(lp)` block. (3) Removed per-append `kernel.create_market(node_id, system_lp_amount)` call. (4) `halt_and_settle` no longer calls `kernel.resolve_all` (deleted); fires RunEnd + tool.on_halt only. (5) `snapshot()` rewritten — replaced legacy f64 `markets: HashMap<_, MarketSnapshot>` + `market_ticker: String` with integer-rational `price_index: BTreeMap<TxId, NodeMarketEntry>` + `mask_set: BTreeSet<TxId>` derived via `state::compute_price_index` + `state::compute_mask_set` from `Sequencer::q_snapshot` when wired (sequencer-optional; empty when bus runs ledger-only). bus.rs imports TB-14 types here legitimately — halt-trigger #2 fence (`use`-statement scan) targets `src/state/sequencer.rs` only; bus.rs is the broadcast point. Predecessor TB-9 Atom 4 hash ce86e860 superseded.
   147	"src/drivers/llm_http.rs" = "615596b68956b4a8925110edc99aa746a5543527724ec394bb9dda163147ed7a"
   148	"src/drivers/llm_proxy.py" = "0a756098056503b1eb379c452c8bda8ff20835020d509dbdfc14850fb46b6fe2"
   149	"scripts/smoke_siliconflow.sh" = "6ad54e7c0ab8221f475360dcad80eeeb0d6da0fd55c27acdb1cefb2b390f5341"
   150	"scripts/_smoke_siliconflow.py" = "858d3b219c7e2d01cc370beb000e107b0abb8a211a63b6bf09d072afd02aeb24"
   151	"scripts/test_llm_proxy.py" = "e1dd0306609fb3d2578b078bb21f0090c23978981f674ca0c6e3a60485ebf273"
   152	"src/sdk/prompt_guard.rs" = "b4f7b164770d1a7203b8143f773c66f748994d60a42ece38471f2f7f2839f4f1"
   153	"experiments/minif2f_v4/src/lean4_oracle.rs" = "70fae24cd17f410c10a092e797fcdedea962db3d7cb20f218d02303edae9e98c"
   154	"experiments/minif2f_v4/src/cost_aggregator.rs" = "896b6905dbca9e9736f8896cd5725c16b6e87c6ad3ff822e044975febed46a03"
   155	"experiments/minif2f_v4/src/wall_clock.rs" = "2c9197f8f93b7d130dc7b094a6664f8ece351ec85668921c5beb6d100a1a77ee"
   156	"experiments/minif2f_v4/src/post_hoc_verifier.rs" = "9a93ae5548827b60543df779c67a5f1201b49a681a55e2cbc6a8532e8bb131a2"
   157	"experiments/minif2f_v4/src/jsonl_schema.rs" = "5d0c5a2bcceba42abbc6f5cd58e045e2d5d7a9edc565db4515f3917a21dc520b"
   158	"experiments/minif2f_v4/src/rollback_sim.rs" = "9ccdf177813452bb83221bd780f2f7f10da619117aca8e44d3ecb355505fe561"
   159	"experiments/minif2f_v4/src/agent_models.rs" = "b736b91efb8a93d884b64028da79d5d6af0525e8c7e8f7222055118f43dc26ce"
   160	"experiments/minif2f_v4/src/budget_regime.rs" = "e0c2ba6533444e6178d9e12d0f182d29c3203a929ea0914d3098efd0397948ea"
   161	"experiments/minif2f_v4/src/fc_trace.rs" = "8578edf61148e50abdf8d29a2efdc5114ee20ab727c1ccd6c5a88ad07e7ffa09"
   162	"experiments/minif2f_v4/src/run_id.rs" = "5ff9db039131eaf4efeb65303d886265ef4c4f7e0e0f743467a7cbf22061781c"
   163	"experiments/minif2f_v4/src/h_vppu_history.rs" = "1b83bf2993a74bcf03a89b52a37dbbfbdda5e080c8c437cae0b5e64262ec180f"  # NEW (TB-1 Day-4 2026-04-29): per-problem rolling history of pput_verified for H-VPPUT North Star regression metric (PREREG_PPUT_CCL_2026-04-26.md § 5)
   164	"experiments/minif2f_v4/src/bin/evaluator.rs" = "3530a2d5ac98ed8db0d037bc45fda03594c11596850ecb6be71855c6dee3abf0"  # rehashed by TB-14 Atom 6 follow-up (2026-05-03; architect ruling step 1 — Codex R1 primary VETO closure): the canonical TxId returned by `boltzmann_select_parent_v2` MUST NOT flow into legacy shadow `bus.append` parent_id (kernel.tape uses a different (shadow) id namespace; canonical TxId becomes a dangling citation → kernel.append rejects). Per architect ruling: "Use None unless a real shadow id exists." No canonical → shadow id mapping is currently available; pass None (legacy default). The v2 selector still runs (its result captured as `_v2_canonical_pick` for observability + future canonical wire-up — see B′ step 4 CanonicalNodeGraph + parent_tx replacement for last_tx_by_agent), but its output is explicitly NOT passed to bus.append. Closes Codex R1 primary VETO defect #1.  # rehashed by TB-14 Atom 6 (2026-05-03; closing OBS_TB_12_LEGACY_CPMM_QUARANTINE): production wire-swap. (1) Imports updated: `BoltzmannParams + boltzmann_select_parent` (legacy f64) → `boltzmann_select_parent_v2 + BoltzmannMaskPolicy` (integer-rational). (2) `BusConfig` literals (×2) drop `system_lp_amount` field. (3) `let params = BoltzmannParams::from_env()` → `let policy = BoltzmannMaskPolicy::from_env()`. (4) Tick-time logging derives `market_count` + top-N price ticker from `bus.snapshot().price_index` (cross-multiplication argmax; rendered `n/d`, never decimal). (5) Per-tx prompt build derives `market_ticker_str` from `snap.price_index` + queries `prompt_balance` from `bus.sequencer.q_snapshot().economic_state_t.balances_t` (replaces legacy `snap.get_balance`; the `f64` is the `prompt.rs` render contract — `prompt.rs` is not a TB-14 module surface). (6) Boltzmann selector call replaced: `boltzmann_select_parent(&snap.tape, &prices, &params, &mut rng)` → `boltzmann_select_parent_v2(&snap.price_index, &snap.mask_set, &policy, &mut rng).map(|tx| tx.0)`. Predicate-blind (CR-14.1 + halt-trigger #1) — purely a scheduling priority pick. Predecessor TB-12 Atom 0.5(a) hash 8f1b97f1 superseded. (2026-05-03; architect 2026-05-03 ruling §1.1 + §8 Atom 0.5; TB-11 G3/G4 carry-forward closure): added 4 EvidenceCapsule counters (tb11_lean_error_count / tb11_sorry_block_count / tb11_protocol_parse_failure_count / tb11_partial_accept_count) instrumented at existing classify_lean_error / classify_parse_error / step_partial_ok call sites; sorry-block vs lean-error split via err_detail.contains("sorry") || err_detail.contains("forbidden_payload"); MAX_TX exhausted exit point (inside `if let Some(bundle) = chaintape_bundle`, BEFORE bundle.shutdown) now writes EvidenceCapsule to CAS via runtime::evidence_capsule::write_evidence_capsule + emits TerminalSummary on-chain via runtime::adapter::tb11_emit_terminal_summary_for_run with capsule_cid; bundle.shutdown drains the queued TerminalSummary tx via apply_one. Mirror of TB-8 tb8_emit_finalize_after_verify pattern. Predecessor TB-10 Atom 1+3 hash a5b30cd1 superseded.
   165	"experiments/minif2f_v4/src/experiment_mode.rs" = "7fe34381767c58432a45f25e49c21a7b4f57ed6b4ed0fea2873753c81bccea89"
   166	"constitution.md" = "eec695459c71fbef3685583485deb431fe3b561657b2f285b7c5e7e220e59e03"
   167	"cases/MANIFEST.sha256" = "eccbe35f776c64225d50c461780d1fe75a0a8510adc62b0dd2e96b46dc2dc555"
   168	"handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json" = "69be702e3e0a4bb33c62bd02056495309289fd9271972dc733b4a1ba3dd908e1"
   169	"handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md" = "64d565b4451d1f770425530bb7ffa874da602ddfd9d714ce7f7db6bbfab69463"
   170	"handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md" = "e1a0c97921fd77bd5a1268589bafbdc90e18e26a4feb6c86c6d8a91048d095e1"
   171	"experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs" = "d62ee67cb564355fa26959c1f92a1152f95ab55d987c005abf7810e2da7b83b8"
   172	"handover/audits/A8_AUDIT_HISTORY_2026-04-26.md" = "ef7ccdb2f39795707ab84b4f97c1346da96c6315419183411a194dd21ac26618"
   173	"handover/audits/run_codex_phase_a8_exit_audit.sh" = "3115ec86339621c6a608f72b837a1004804abf4824ecfc34049edc08c833be39"
   174	"handover/audits/run_gemini_phase_a8_exit_audit.py" = "33abd4c0679856719ac38ecd00dc7763b85d30f8c90e83db366c1acdc907a473"
   175	"handover/preregistration/PPUT_CCL_HARD10_2026-04-26.json" = "0a906708941835bb56c0af7c6c8fdabf6747911f97d5108ff7a4d76abfadba01"
   176	"handover/preregistration/scripts/draw_hard10_pput_ccl.py" = "dd19b1396d2f235ddd412db58802412356b9e100b1d6fcba701833327eaf33d6"
   177	"handover/preregistration/scripts/run_c2_phase_c_ablation.sh" = "1c4a847e7ca2d9156ecb64604aec9953cf6c73369fcec7e316c07ac8a8ee4a61"  # rehashed by CO1.13.2/3 drift review (2026-04-29): added --half mode (3 problems × 5 modes × 1 seed × MAX_TX=20) for half-real regression between --smoke and --full; FC3-N34 readonly verification
   178	"handover/preregistration/scripts/analyze_c3_h1_h4.py" = "04658a4c1f28308580cdcb900db1b7501d003b1f305d2599ea09cbf653bd084b"
   179	
   180	# 2026-04-26 night-shift CO P0 manifest expansion (43 -> 49 entries; +6).
   181	# White papers (architecture + economic) authored by user 2026-04-26 ultrathink;
   182	# blueprint synthesizes both into file-level v4 spec; plan v3.1 atomizes; protocol
   183	# codifies tri-model co-execution; amendment v1 records D-decisions + cost adjustment.
   184	"handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md" = "5c17819b6f574a384d894c801322e056fcd869ff9526794238888cc0741e6d88"
   185	"handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md" = "d059a0bd2e95fb13c112d8ed6acc9adc32c63a8c89206dd5fcfa77bfb8d5aa44"
   186	"handover/whitepapers/REVISION_NOTES_2026-04-27.md" = "d6252b9f57a571489c68d98d8b23a1238edc70543f319deb826c919a1928cb9b"
   187	
   188	# 2026-04-27 wave 6 (CO P0 closeout; +2 entries; 74 → 76).
   189	"handover/audits/GEMINI_WP_REVISION_AUDIT_2026-04-27.md" = "6a6f35399af4a959a20d8fbb88196fa2755b709403c91abb0a7d4fdbaa2673b4"
   190	"handover/architect-insights/CO_P0_EXIT_REPORT_2026-04-27.md" = "4634842fc0aa382bdce417ff0eb0fdf523ad7b77719586c96ac6b7e6f74074eb"
   191	
   192	# 2026-04-27 path-A spec freeze audit (+1 entry; 76 → 77).
   193	# Gemini PASS path A WP finalization; CHALLENGE on spec v1.1 freeze (NEEDS-FIX before CO P1).
   194	# Codex audit in flight; will commit additional entry when complete.
   195	"handover/audits/GEMINI_SPEC_FREEZE_AUDIT_2026-04-27.md" = "a2e5d954b061cb43e17e6f66f2fe94a71cfd8160194dfa96789da02e8e633225"
   196	"handover/audits/CODEX_SPEC_FREEZE_AUDIT_2026-04-27.md" = "4cb0ba5b5665a98c9601852f48ebfbb40fc2f3e9fe67bd7e7c104eac25019aab"
   197	"handover/audits/CODEX_SPEC_V12_REAUDIT_2026-04-27.md" = "57dc9d19944b8257ddb159f284b389938bc541996607df95989af4f76a6bbf81"
   198	
   199	# 2026-04-27 CO1.3.1 git substrate spike result (8/8 PASS via git2-rs after gix pivot)
   200	"spike/gix_capability/results/SPIKE_RESULT_2026-04-27.md" = "1fd869619ae4dd3e3a89dd80f118f57ece8a6260643dd4c7a8c7bf0ec141bd28"
   201	"spike/gix_capability/Cargo.toml" = "a0e91c97cd18e9ffb8566d2bc901325cb680b728493eef37d8ac02ae2f03c76c"
   202	
   203	# 2026-04-27 Wave 1 — CO1.0a MicroCoin + CO1.0 [constitution_root] + walkthrough Inv 3 e2e.
   204	# Real Rust code; 27/27 tests PASS; data validates Inv 3 monetary conservation closure.
   205	# 2026-05-01 TB-6 Atom 1.1 — `pub mod runtime;` declaration added (production ChainTape factory) + R-022 TRACE_MATRIX backlink.
   206	"src/lib.rs" = "35ff4cd2ed042ab9d92d04f576b11d3cac73e8bc11e4c613886a6a7ff52ebc2f"  # rehashed by TB-14 Atom 6 (2026-05-03; closing OBS_TB_12_LEGACY_CPMM_QUARANTINE): removed `pub mod prediction_market;` together with the legacy CPMM file. Predecessor TB-6 Atom 1.1 hash 196fe13a superseded.
   207	# 2026-05-01 TB-6 Atom 1.1 — production-mode runtime ChainTape factory + driver wrapper (NEW file). All pub symbols carry /// TRACE_MATRIX FC3-N1 backlinks per R-022.
   208	# 2026-05-01 TB-6 Atom 2 — added build_chaintape_sequencer_with_initial_q variant + adapter sub-module declaration.
   209	# 2026-05-01 TB-6 Atom 3 — Step 4 rejection_writer wired to RejectionEvidenceWriter::open_jsonl(<runtime_repo>/rejections.jsonl) for L4.E persistence per architect § 3.5.
   210	# 2026-05-01 TB-6 Atom 4 — `pub mod verify;` declaration added (replay verifier sibling to adapter).
   211	# 2026-05-01 TB-6 Atom 5 — `pub mod agent_audit_trail;` declaration added + ChaintapeBundle.cas_path field (additive) for audit-trail re-open.
   212	# 2026-05-01 TB-6 Atom 6 — `pub mod run_summary;` declaration added (RunSummary fork-visibility aggregator sibling to verify + agent_audit_trail).
   213	# 2026-05-01 TB-7 Atom 1 — `pub mod agent_keypairs;` declaration added (per-agent Ed25519 keypair manager + on-disk pubkey manifest; run-local identity ONLY; per ARCHITECT_RULING D2 + TB-7 charter §4.2).
   214	# 2026-05-01 TB-7 Atom 1.5 — `pub mod proposal_telemetry;` declaration added (ProposalTelemetry CAS object writer per ARCHITECT_RULING D5 + charter §4.5; precondition for golden_path_token_count chain-derivation).
   215	# 2026-05-01 TB-7 Atom 1.7 — RejectionWriter BootstrapError variant + L4.E open_jsonl now fail-closed (returns BootstrapError::RejectionWriter instead of silent in-memory fallback). Per Codex audit cc7b3dd action item #1.
   216	# 2026-05-01 TB-7 Atom 5 — `pub mod chain_derived_run_facts;` declaration added (ChainDerivedRunFacts aggregator per ARCHITECT_RULING D4 + charter §4.4 — bit-exact structural facts from L4 + L4.E + CAS alone).
   217	# 2026-05-01 TB-7.7 D4 — `pub mod verification_result;` declaration added (Lean oracle verdict CAS object linked from ProposalTelemetry.verification_result_cid).
   218	"src/runtime/mod.rs" = "03fd53582977fe10865eff34824babaf5f8009fd9131f6485e477134a936d9ce"  # rehashed by TB-15 Atom 5 (2026-05-03; architect §6.2 ruling): added `pub mod markov_capsule;` declaration for TB-15 MarkovEvidenceCapsule schema + writer + default-deny gate (FC3-N43). Predecessor TB-15 Atom 2 hash adfc18a4 superseded.
   219	# 2026-05-02 TB-10 Atom 1 — `runtime::bootstrap` module (NEW file). Single source of truth preseed factory `default_pput_preseed_pairs()` consumed by both evaluator's `--task-mode self|both` preseed branch and `lean_market` user CLI bootstrap. Pure function; replay-deterministic; produces 12 entries (tb7-7-sponsor + Agent_user_0 + Agent_0..9) summing to 30_000_000 micro genesis supply.
   220	"src/runtime/bootstrap.rs" = "78c09eabd3a0b226c84539f414965347eb6928861141bb82af3bbaa5986cd58e"
   221	# 2026-05-01 TB-6 Atom 2 — chaintape adapter helpers (synthetic TaskOpen/EscrowLock/WorkTx constructors + balance seeding) NEW file.
   222	"src/runtime/adapter.rs" = "dcdb2566592e257ff7e3098cdf7ad232060398d2b4a8e4f11505222941c9aabe"  # rehashed by TB-11 Atom 4 (2026-05-02; architect §6.2 ruling): added tb11_emit_terminal_summary_for_run + tb11_emit_expire_for_eligible runtime helpers. Mirror tb8_emit_finalize_after_verify pattern; thin wrappers over Sequencer::emit_system_tx with policy-gated eligibility scan for TaskExpire (state ∈ {Open, Bankrupt} + opened_at_logical_t deadline elapsed + no Finalized claim). Predecessor TB-10 Atom 1 hash 403ff876 superseded.
   223	# 2026-05-01 TB-6 Atom 4 — replay verifier library (NEW file). Re-opens runtime_repo + cas + pinned_pubkeys.json, replays L4 chain via replay_full_transition, emits ReplayReport. All pub symbols carry /// TRACE_MATRIX FC3-N1 backlinks per R-022.
   224	"src/runtime/verify.rs" = "737c447a85067f9c882d9c644f658a565dd9864b8f588ddd5166925582c6289f"  # rehashed by TB-7 Atom 4 (2026-05-01): added agent_signatures_verified + proposal_telemetry_cas_retrievable indicators (Gate 4 + Gate 5); ReplayReport indicator count 5 → 7; new verify_agent_artifacts helper walks L4 entries, verifies WorkTx + VerifyTx signatures against agent_pubkeys.json + verifies WorkTx.proposal_cid resolves to CAS ProposalTelemetry. all_indicators_pass extended.
   225	# 2026-05-01 TB-6 Atom 4 — `verify_chaintape` CLI thin wrapper (NEW file).
   226	"src/bin/verify_chaintape.rs" = "68d024d10f3ce7b76dd91ef1c911fc1da55cfef5a875040c726715587c90244b"
   227	# 2026-05-01 TB-6 Atom 4 — replay verifier integration tests (I90 / I90b / I90c) (NEW file).
   228	"tests/tb_6_verify_chaintape.rs" = "ca12d4950899e36e3b946cafd2239e666c03bc2323fb06d88cc35d6cb803274e"  # rehashed by TB-7.7 D7 (2026-05-01): I90 assertion flipped from `!initial_q_state_loaded_from_disk` to `initial_q_state_loaded_from_disk` because build_chaintape_sequencer_with_initial_q now always persists initial_q (even genesis-equivalent) so verify_chaintape replay picks up pre-seeded state. Closes a regression surfaced by smoke run #3.
   229	# 2026-05-01 TB-6 Atom 5 — Agent audit trail (AgentProposalRecord + CAS storage + JSONL index) NEW file. All pub symbols carry /// TRACE_MATRIX FC3-N1 backlinks per R-022.
   230	"src/runtime/agent_audit_trail.rs" = "25dde31a94e0b93ce5d5751034f7caa1aee2972d752ec136a342afc1eb552e62"  # rehashed by TB-7.5 fix #4 (2026-05-01; Codex audit 492e86c action #4): cleaned the stale module-doc line that still said "9-field record (10 with logical_t for chronological ordering)" — the original Atom 1.7 logical_t-removal commit left this comment in place. (No code changes; doc-only.) Prior TB-7 Atom 1.7 hash had logical_t REMOVED + audit_hash v2 + chain_link.
   231	# 2026-05-01 TB-6 Atom 5 — Agent audit trail integration tests (I91 / I91b / I91d) (NEW file).
   232	"tests/tb_6_agent_audit_trail.rs" = "a260ac79eaf9cf462598c8200336e6c0ef17393bfd7ac0d05493247a4f3c0ea8"  # rehashed by TB-7 Atom 1.7 (2026-05-01): I91/I91b updated for logical_t-as-parameter API; new I91e structural witness asserts AgentProposalRecord has exactly 10 serde keys (9 architect fields + rejection discriminator) and explicitly rejects logical_t presence. Per Codex audit cc7b3dd action item #3.
   233	# 2026-05-01 TB-6 Atom 6 — RunSummary aggregator (NEW file). Walks L4 + L4.E + CAS at end-of-run; emits run_summary.json. All pub symbols carry /// TRACE_MATRIX FC3-N1 backlinks per R-022.
   234	"src/runtime/run_summary.rs" = "defc4697ec8af2c050aea3bc33beeaaeff8251d4e89d0a8c48803ffac262c58f"  # rehashed by TB-13 Atom 1 (2026-05-03): extract_tx_id match arm extended for 3 new TB-13 variants (CompleteSetMint / CompleteSetRedeem / MarketSeed). Predecessor TB-11 Atom 1 hash 66b934f8 superseded.
   235	"src/runtime/evidence_capsule.rs" = "5266ff43f53cd1a54d3cdbb87e40b886ea4439e109f27514711846f7c84b9e03"  # rehashed by TB-11 Atom 3 (2026-05-02; architect §6.1 ruling): EvidenceCapsule writer added — write_evidence_capsule fn + CapsuleWriteError + 2 round-trip tests. TB-11 MVP stores raw log uncompressed (gzip wrapping deferred to TB-15 Markov Loom). Predecessor Atom 1 hash 7008edce superseded.
   236	# 2026-05-01 TB-6 Atom 6 — `gen_run_summary` CLI thin wrapper (NEW file).
   237	"src/bin/gen_run_summary.rs" = "595677e44d31556383c901a692dae4c305bbffdd0a5cf4667a4679e07ec065b0"
   238	# 2026-05-01 TB-8 — `audit_dashboard` CLI (NEW file). Diagnostic dashboard over runtime_repo + cas; composes verify_chaintape + chain_derived_run_facts + agent_keypairs + agent_audit_trail libraries; outputs 7-section text or --json structured report. Per TB-7 charter §13.1 (post-TB-7 sequencing TB-8).
   239	"src/bin/audit_dashboard.rs" = "c953244a5661d08d81feed2109a451cb6f68f880863fc6ba006a9505ae60ed10"  # rehashed by TB-15 Atom 6 (2026-05-03; architect §6.5 SG-15.6): ADDITIVE — added §15 TB-15 Autopsy + Markov render section. (1) New DashboardReport fields: autopsy_event_counts: Vec<(String, u32)> + latest_markov_capsule_cid_hex: Option<String>. (2) New read_latest_markov_pointer() helper reading handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt (best-effort; None on absence). (3) New render_section_15(&[(String, u32)], Option<&str>) -> String pure function. ARCHITECT-MANDATED PRIVACY BANNER: opens with literal "AUTOPSY IS PRIVATE" (architect §6.4 + CR-15.1); SG-15.6 ship gate enforced by 4 tb14_render_tests::sg_15_6_* unit tests asserting (a) banner present, (b) input signature carries only counts + pointer hex (no raw bytes possible), (c) empty Markov pointer renders explicit generation hint, (d) Markov default-deny explanation ("TURINGOS_MARKOV_OVERRIDE=1" + "deeper history") present. (4) §15 invocation appended to render_text after §14. Predecessor TB-14 Atom 6 hash de1f71a1 superseded.  # rehashed by TB-14 Atom 6 (2026-05-03; architect 2026-05-03 ruling §5.1 + §5.5 SG-14.6 closing OBS_TB_12_LEGACY_CPMM_QUARANTINE): ADDITIVE — added §14 TB-14 PriceIndex render section. (1) New imports: `state::{compute_price_index, NodeMarketEntry, TaskId, TxId}`, `state::typed_tx::{NodePosition, PositionKind, PositionSide}`, `state::q_state::{AgentId, EconomicState}`, `economy::money::MicroCoin`. (2) New `DashboardReport.price_index: BTreeMap<TxId, NodeMarketEntry>` field populated by `price_index_from_exposures` helper that synthesizes an `EconomicState` from `exposures: Vec<ExposureRecordRow>` and calls canonical `state::compute_price_index` (no second source-of-truth — architect §5.1; charter §7 auto-resolution A). (3) New `render_section_14(&BTreeMap<TxId, NodeMarketEntry>) -> String` pure function. ARCHITECT-MANDATED BANNER: opens with literal "PRICE IS SIGNAL, NOT TRUTH" (architect §5.1 verbatim); SG-14.6 ship gate enforced by 4 `tb14_render_tests::sg_14_6_*` unit tests asserting (a) the banner is present, (b) prices render as `numerator/denominator` integer-rational (NEVER decimal — charter §5.6 forbidden), (c) empty PriceIndex renders explicit empty-state, (d) None-priced (zero-liquidity) nodes render as `None` per FR-14.3 / halt-trigger #5. (4) §14 invocation appended to `render_text` after §13. Predecessor TB-12 Atom 4 + Atom 6(a) hash 9881aed3 superseded. architect 2026-05-03 §8 Atom 4 + §9.3 SG-12.6): added §13 TB-12 Node exposure records section + ExposureRecordRow struct + L4 walk arms for TypedTx::Work (FirstLong row) + TypedTx::Challenge (ChallengeShort row). Refactored §13 inline render block into `render_section_13(&[ExposureRecordRow]) -> String` pure function for SG-12.6 unit-testability. Added `#[cfg(test)] mod tb12_render_tests::sg_12_6_dashboard_view_positions_works` covering empty + single-FirstLong + same-node-long+short + 2-node aggregation cases + forbidden-token grep (Open market balances / MarketBuy / etc). LABEL DISCIPLINE: "exposure records" NOT "Open market balances" (architect §8 Atom 4). Predecessors TB-11 Atom 5 hash 2e28c2b6 + earlier TB-12 hash f697ba24 superseded.  # rehashed by TB-11 Atom 5 (2026-05-02; architect §6.2 ruling): added §12 TB-11 Epistemic Exhaust + Capital Liberation section. 3 sub-tables: Exhausted runs (TerminalSummaryTx ≡ RunExhausted; run_id, task_id, outcome, attempts, capsule_cid), Expired tasks (TaskExpireTx; task_id, sponsor, refund, reason, @logical_t + total refund line), Bankrupt tasks (TaskBankruptcyTx; task_id, reason, failed_runs, capsule_cid). Privacy mandate footer (capsule public_summary surfaced; raw log shielded behind audit role). Predecessor TB-10 hash d89b2f32 superseded.
   240	# 2026-05-01 TB-6 Atom 6 — RunSummary integration tests (I92 / I92b / I92c) (NEW file).
   241	"tests/tb_6_run_summary.rs" = "6efb4baa36d08dc55c1140ea3279a0088a2f14f58cc71e9283a876ffa2da68d8"
   242	# 2026-05-01 TB-7 Atom 2 — authoritative routing integration tests (NEW file). I100 (real-signature WorkTx verifies via manifest) + I101 (zero-stake WorkTx → L4.E) + I102 (distinct agents → distinct pubkeys). Gate 1 + Gate 4 wired evidence.
   243	"tests/tb_7_authoritative_routing.rs" = "3ac5bd2289c19a3c044e4f758c47014794989ae0c683a62469799220d6296b64"  # rehashed by TB-7 Atom 3 (2026-05-01): added I103 (OMEGA WorkTx + VerifyTx pair) + I104 (VerifyTx signature verifies via manifest).
   244	# 2026-05-01 TB-7 Atom 6 — chain-backed smoke (synthetic-LLM end-to-end) integration test (NEW file). I110 ship-gate: bootstrap chaintape, submit 3 synthetic-agent WorkTx + VerifyTx pairs through bus.submit_typed_tx, run verify_chaintape (all 7 indicators GREEN — Gates 4 + 5 wired evidence), compute_run_facts_from_chain (Gate 6 round-trip), persist smoke evidence to handover/evidence/tb_7_chaintape_smoke_2026-05-01/. Real-LLM smoke documented as manual procedure in test header.
   245	"tests/tb_7_atom6_chain_backed_smoke.rs" = "5e1875216eee8c03891d7966db696eb287e97f22c6469d428476b68c354f3b6e"
   246	# 2026-05-01 TB-7 Atom 7 — Gate 7 legacy-bypass regression conformance test (NEW file). Repo-wide grep + comment-block walker that flags any unannotated bus.append / bus.append_oracle_accepted call in evaluator.rs. Includes positive/negative scanner controls. Per ARCHITECT_RULING §4 Gate 7 + charter §6 #31.
   247	"tests/tb_7_legacy_append_regression.rs" = "25bbfdefda6a2e2e1bbaa8092cf1870f6fd77914dc6b19af386a74db52bf7f38"
   248	# 2026-05-01 TB-7 Atom 1 — Per-agent Ed25519 keypair manager + on-disk pubkey manifest (NEW file). Run-local identity ONLY; private keys in process memory only and zeroed on drop. AgentKeypair / AgentKeypairRegistry / AgentPubkeyManifest / verify_agent_signature; mirrors PinnedSystemPubkeys structural pattern. Per ARCHITECT_RULING 2026-05-01 D2 + TB-7 charter §4.2.
   249	"src/runtime/agent_keypairs.rs" = "a027ddb0b5cab18842d5f70b9ce87be48b486ca0c51acce73a20fddae6fb28a8"
   250	# 2026-05-01 TB-7 Atom 1.5 — ProposalTelemetry CAS object writer (NEW file). Per ARCHITECT_RULING D5 + charter §4.5; precondition for chain-derived golden_path_token_count. 8-field schema (agent_id / prompt_context_hash / proposal_artifact_cid / candidate_tactic / token_counts / tool_calls / branch_id / parent_tx). All pub symbols carry /// TRACE_MATRIX FC1-N14 backlinks per R-022.
   251	"src/runtime/proposal_telemetry.rs" = "91bb53784dfb7d9e7643f7e88d672c12c460dce30e38635b6826ccb3b0edcf7a"  # rehashed by TB-7.7 D4 (2026-05-01): added optional `verification_result_cid: Option<Cid>` field (additive; #[serde(default)] preserves backward compat). NEW with_verification_result(cid) builder pattern for OMEGA-accept paths. Updated schema_validity test from 8 → 9 fields. (Pre-D4 commit had D1+D2: payload bytes → CAS + parent_tx wire.)
   252	# 2026-05-01 TB-7 Atom 5 — chain_derived_run_facts.rs (NEW file; renamed from chain_derived_pput.rs per ARCHITECT_RULING D4). Computes bit-exact structural fact set from L4 + L4.E + CAS alone. Per charter §4.4 fields: solved / verified / tx_count / proposal_count / golden_path_token_count / gp_payload / gp_path / gp_proof_file / tactic_diversity / tool_dist / failed_branch_count. Time-sensitive fields excluded. 3 unit tests U-A5.a..c.
   253	"src/runtime/chain_derived_run_facts.rs" = "87323282155df700374d2a8b3ab9b6e3a2fafcf9d91022f3c0aec2b35a42974a"  # rehashed by TB-7R parent_tx state ruling (2026-05-02): added pub enum ParentTxState (4 variants) + ChainDerivedRunFacts.parent_tx_state field + WorkTxAttempt accumulator + compute_parent_tx_state helper. Per architect verdict 2026-05-02 (parent_tx is conditional invariant, not unconditional smoke requirement). See handover/directives/2026-05-02_TB7R_PARENT_TX_DAG_SMOKE_VERDICT.md + handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md.
   254	# 2026-05-01 TB-7.7 D4 — verification_result.rs (NEW file). VerificationResult CAS object: 8 fields (target_work_tx / verifier_agent / lean_exit_code / lean_stdout_hash / lean_stderr_hash / proof_file_hash / proof_artifact_cid / verified). from_lean_run constructor. write_to_cas / read_from_cas. 4 unit tests (round-trip / cid determinism / from_lean_run / schema_validity_eight_fields). Per architect ultrathink ruling D4.
   255	"src/runtime/verification_result.rs" = "6b58d0ca1d09104f2edacf1b57a5560d76e073d6fc6238529e1ec375eb610fde"  # rehashed by TB-7.5 fix #2 (2026-05-01; Codex audit 492e86c action #2 BLOCKING): proposal_count now counts BOTH accepted L4 WorkTx AND rejected L4.E records whose tx_kind == TxKind::Work; ProposalTelemetry CAS resolution extended to L4.E WorkTx (telemetry contributes to tactic_diversity / tool_dist / golden_path_token_count). Closes the prior semantic gap where docs said "accepted + rejected" but implementation counted only L4-side.
   256	# 2026-04-29 TB-1 Day-3 — economy/mod.rs rehashed: + `pub mod ledger` declaration (P1 L4 wrapper).
   257	"src/economy/mod.rs" = "c735f92316bc5982f6ec316026d70179fb783d152b5ded2ed6ccb0775b97f069"
   258	"src/economy/money.rs" = "857fc6df2aefa4b26fd4b7d22ec06f1105ac3ec4393c6788a8f30370b6d7e776"
   259	# 2026-04-29 TB-1 Day-2 — P3 RSP-0 monetary guards + escrow vault (NEW files).
   260	"src/economy/monetary_invariant.rs" = "3c8469282428d9532993756499790b05d09f7773eab3d2c66da72e1c2ebec5af"  # rehashed by TB-14 Atom 2 (2026-05-03; architect §5.1): doc-comment update only — `price_index_t` (legacy field) bullet replaced with TB-14 `compute_price_index` derived-view note (signal-not-truth; not stored on EconomicState; trivially not in supply sum). No code-path change; conservation invariant unchanged. Predecessor TB-13 Atom 1 hash 4d1182a6 superseded.

exec
/bin/bash -lc 'git show --stat --oneline a14e01e..2337381 -- genesis_payload.toml' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
2337381 TB-15 SHIPPED — Lamarckian Autopsy + Markov EvidenceCapsule (Class 2 self-audit; 8/8 SG; 6/6 halt-triggers; closes architect §6 spec FR-15.1..6 + CR-15.1..6 + SG-15.1..8)
 genesis_payload.toml | 4 ++--
 1 file changed, 2 insertions(+), 2 deletions(-)
31be856 TB-15 Atom 5 — MarkovEvidenceCapsule schema + generator binary
 genesis_payload.toml | 2 +-
 1 file changed, 1 insertion(+), 1 deletion(-)
f06d548 TB-15 Atom 3 — AutopsyIndex on EconomicState + TaskBankruptcyTx wire-in
 genesis_payload.toml | 10 +++++-----
 1 file changed, 5 insertions(+), 5 deletions(-)

exec
/bin/bash -lc "git log --oneline -- genesis_payload.toml | sed -n '1,20p'" in /home/zephryj/projects/turingosv4
 succeeded in 149ms:
2337381 TB-15 SHIPPED — Lamarckian Autopsy + Markov EvidenceCapsule (Class 2 self-audit; 8/8 SG; 6/6 halt-triggers; closes architect §6 spec FR-15.1..6 + CR-15.1..6 + SG-15.1..8)
31be856 TB-15 Atom 5 — MarkovEvidenceCapsule schema + generator binary
f06d548 TB-15 Atom 3 — AutopsyIndex on EconomicState + TaskBankruptcyTx wire-in
1189cb2 TB-14 Atom 6 B′ step 7 R2 closure — Gemini R2 Q11 (sequencer_wired field)
9daba5a TB-14 Atom 6 B′ step 4 — CanonicalNodeGraph + compute_mask_set canonical-graph rewire (architect ruling §3+§4)
48e84ee TB-14 Atom 6 B′ steps 1+2 — close Codex R1 VETO + secondary CHALLENGE (architect ruling 2026-05-03)
38412bf TB-14 Atom 6 follow-up — close internal auditor F1 (dead BusResult::Invested f64 residual in bus.rs)
44cd480 TB-14 Atom 6 — production wire-swap + legacy CPMM excision (closes OBS_TB_12_LEGACY_CPMM_QUARANTINE)
a9fbdf3 TB-14 Atom 5 — boltzmann_select_parent_v2 + halt-triggers #1/#2 — 6/6 GREEN
7cbcacf TB-14 Atom 4 — BoltzmannMaskPolicy::from_env() + 7 env vars
668695d TB-14 Atom 3 — mask_set + compute_mask_set + BoltzmannMaskPolicy skeleton
23ac581 TB-14 Atom 2 — PriceIndex pure-fn view + fence architectural fix
edbc555 TB-13 Atom 6 round-5 — Codex RQ5 remediation: drop ResolutionRef wrapper
353aa97 TB-13 Atom 6 round-4 — doc fixes + OBS closure (Class 3)
cdba357 TB-13 Atom 6 round-3 — Codex VETO + Gemini CHALLENGE remediation (Class 3)
07fc869 TB-13 Atom 6 round-2 — Codex VETO remediation (V1+V2+Q9; Class 3)
1806432 TB-13 Atoms 2+3+5 — Sequencer dispatch + conservation invariant + SG-13.x integration tests (Class 3)
70303af TB-13 Atom 1 — typed_tx schemas (Class 2; +8 unit tests)
32aab27 TB-13 Atom 0 + 0.5 — Charter + legacy f64 CPMM forward-fence (Class 1)
975108d TB-12 Atom 6 ship-gate refinement — 4 SG-12.x strict architect-name alignment

exec
/bin/bash -lc 'sha256sum src/runtime/mod.rs src/state/q_state.rs src/state/typed_tx.rs src/state/sequencer.rs src/bottom_white/cas/schema.rs src/bin/audit_dashboard.rs tests/fc_alignment_conformance.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
03fd53582977fe10865eff34824babaf5f8009fd9131f6485e477134a936d9ce  src/runtime/mod.rs
c23cc95d409cdebb0b91cc0b7b75aee9944d3397a5b4e014a63d11e7b248c614  src/state/q_state.rs
665838b02191345dedf7d312f0b33cb5214dad7475acb084bb3ac12245b3268c  src/state/typed_tx.rs
9fa5936230738e5f72b14713a96238ad53583c08cdd6207d8832d27b009f50a9  src/state/sequencer.rs
6427695d8dbb4320fc8f23e64b9595a797cc790c6fb9439e74ba03422e0bb988  src/bottom_white/cas/schema.rs
c953244a5661d08d81feed2109a451cb6f68f880863fc6ba006a9505ae60ed10  src/bin/audit_dashboard.rs
5e257f275d86032a957101b7898293fa9d6cdfc13a4bdd2a5f630fb9e940b408  tests/fc_alignment_conformance.rs

exec
/bin/bash -lc "git show --unified=3 f06d548 -- genesis_payload.toml | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
commit f06d548f6d18043fc9f78fc1e230f25193023b6e
Author: gretjia <gretjia@users.noreply.github.com>
Date:   Sun May 3 23:12:35 2026 +0000

    TB-15 Atom 3 — AutopsyIndex on EconomicState + TaskBankruptcyTx wire-in
    
    MOD src/state/q_state.rs:
      - + AutopsyIndex(BTreeMap<EventId, Vec<Cid>>) newtype
      - + agent_autopsies_t: AutopsyIndex 13th sub-field on EconomicState,
        serde-default for backward-compat with pre-TB-15 chain snapshots
      - sub-field count assertion 12 → 13 (+ test renamed
        economic_state_has_thirteen_sub_fields)
    
    MOD src/runtime/autopsy_capsule.rs:
      - + derive_autopsies_for_bankruptcy(pre_econ, bk, round, t)
        PURE deterministic helper — returns Vec<(AgentAutopsyCapsule, Vec<u8>)>
        for each agent with active StakeEntry on the bankrupted task.
        Deterministic Cid (BTreeMap iteration sorted by TxId; canonical-encode
        → sha256). Used by both dispatch arm + apply_one hook to guarantee
        Cid agreement (Art.0.2 replay determinism).
      - + write_bankruptcy_autopsies_to_cas(cas, pre_econ, bk, ...) writer
        (re-derives + puts capsule + private_detail bytes to CAS; idempotent;
        returns Cids matching dispatch arm's deterministic derivation).
      - + 4 new in-module tests: derive_emits_one_per_staker_target_only +
        derive_deterministic_across_calls + derive_empty_when_no_stakers +
        write_bankruptcy_autopsies_to_cas_round_trip.
    
    MOD src/state/sequencer.rs:
      - TaskBankruptcyTx dispatch arm Step 3.5 — populate
        q_next.economic_state_t.agent_autopsies_t[event_id] with deterministic
        Cids per losing-staker (PURE; no CAS write here).
      - apply_one Stage 3.5 — write_bankruptcy_autopsies_to_cas() side-effect
        writes capsule + private_detail bytes to CAS using same deterministic
        helper (replay-safe identical Cids).
      - NO change to monetary invariants
        (assert_no_post_init_mint + assert_total_ctf_conserved unchanged).
      - NO predicate registry mutation (CR-15.3 + halt-trigger #3).
    
    MOD genesis_payload.toml — REHASH trust_root for 5 modified files:
      - src/runtime/mod.rs (TB-15 Atom 2 pub mod autopsy_capsule)
      - src/state/q_state.rs (TB-15 Atom 3 +AutopsyIndex +agent_autopsies_t)
      - src/state/typed_tx.rs (TB-15 Atom 2 +RiskRuleId)
      - src/state/sequencer.rs (TB-15 Atom 3 dispatch + apply_one hooks)
      - src/bottom_white/cas/schema.rs (TB-15 Atom 2 +4 ObjectType variants)
    
    MOD test fixture sub-field counters 12 → 13:
      - tests/economic_state_reconstruct.rs (twelve_ → thirteen_)
      - tests/q_state_reconstruct.rs (twelve_ → thirteen_)
      - tests/six_axioms_alignment.rs (axiom_3 12 → 13)
    
    MOD tests/tb_15_halt_triggers.rs:
      - Backfill halt-trigger #1 (raw_logs_not_in_general_read_view):
        file-scan AgentVisibleProjection { ... } body for forbidden TB-15
        autopsy types (agent_autopsies_t / AutopsyIndex / AgentAutopsyCapsule
        / private_detail_cid). Confirms CR-15.1 + halt-trigger #1 STRUCTURALLY
        enforced.
      - Backfill halt-trigger #4 (private_detail_not_in_other_agent_view):
        file-scan AutopsyIndex newtype declaration in q_state.rs for
        forbidden value-type shapes (Vec<AgentAutopsyCapsule> / Vec<u8> /
        Vec<AutopsyPrivateDetail>) + positive assertion that value type IS
        Vec<Cid>. Confirms SG-15.2 STRUCTURALLY enforced.
    
    Tests:
      cargo test --workspace: 632 PASS / 0 fail / ignored unchanged
        (pre-Atom-3 baseline 633 PASS minus the 3 unimplemented halt stubs
        pending Atoms 4/5; net +0 vs TB-14 ship 803 — different filter set
        here; full count is per-test-binary stratified — see ship report
        Atom 6).
      Halt-triggers: 3/6 GREEN (#1 + #3 + #4); 3/6 still unimplemented!()
        (#2 + #5 + #6 backfill in Atoms 4-5).
      Trust Root: GREEN.
      Sub-field count assertion: 13 GREEN.
      TaskBankruptcy-touching tests (tb_11_epistemic_exhaust +
        tb_13_chaintape_smoke + tb_13_complete_set + tb_8_minimal_payout):
        no regression.
    
    CR-15.1 + halt-trigger #1: STRUCTURALLY ENFORCED — agent_autopsies_t
    NOT in AgentVisibleProjection.
    SG-15.2 + halt-trigger #4: STRUCTURALLY ENFORCED — AutopsyIndex value
    type is Vec<Cid> (not raw bytes).
    SG-15.1: GREEN — TaskBankruptcyTx accepts now emit per-staker capsule
    Cids; Atom 6 evidence will exhibit live emission.
    
    Replay determinism (Art.0.2): identical L4 chain → identical
    agent_autopsies_t (helper is pure; sorted-BTreeMap iteration; no
    wall-clock / random). CAS write in apply_one is idempotent (identical
    bytes → existing Cid).
    
    FC-trace: FC1-N32 + FC1-N33 (write_autopsy_capsule + apply_one hook)
    
    phase_id: P4 + P5 v0 prep
    roadmap_exit_criteria_addressed: P4-Exit1 (single-agent failure → local
      AutopsyCapsule, not global broadcast — STRUCTURALLY ENFORCED via
      agent_autopsies_t exclusion from AgentVisibleProjection) + P4-Exit3
      (broadcast surface foundation — public_summary low-info text only)
    kill_criteria_tested: halt-trigger #1 + halt-trigger #4 + halt-trigger #3
      all GREEN. P4-Forbidden:全量广播ledger STRUCTURALLY DISCHARGED.
    
    Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

diff --git a/genesis_payload.toml b/genesis_payload.toml
index 3cf202e..d37866c 100644
--- a/genesis_payload.toml
+++ b/genesis_payload.toml
@@ -215,7 +215,7 @@ boot_attestation_hash = "PENDING_v4_X_SELF_REFERENTIAL_COMPUTE"
 # 2026-05-01 TB-7 Atom 1.7 — RejectionWriter BootstrapError variant + L4.E open_jsonl now fail-closed (returns BootstrapError::RejectionWriter instead of silent in-memory fallback). Per Codex audit cc7b3dd action item #1.
 # 2026-05-01 TB-7 Atom 5 — `pub mod chain_derived_run_facts;` declaration added (ChainDerivedRunFacts aggregator per ARCHITECT_RULING D4 + charter §4.4 — bit-exact structural facts from L4 + L4.E + CAS alone).
 # 2026-05-01 TB-7.7 D4 — `pub mod verification_result;` declaration added (Lean oracle verdict CAS object linked from ProposalTelemetry.verification_result_cid).
-"src/runtime/mod.rs" = "3b2901c4890a46998a8ca6156c207013a0fefdc22a1100ef999f405d7b224de0"  # rehashed by TB-11 Atom 1 (2026-05-02; architect §6.1 ruling): added `pub mod evidence_capsule;` declaration for the TB-11 EvidenceCapsule schema + writer surface. Predecessor TB-10 Atom 1 hash bdbb3577 superseded.
+"src/runtime/mod.rs" = "adfc18a48e73835618a58e61cdce63e8fdac68d2043f4af0e0dbe7e57f27fe50"  # rehashed by TB-15 Atom 2 (2026-05-03; architect §6.2 ruling): added `pub mod autopsy_capsule;` declaration for TB-15 AgentAutopsyCapsule schema + writer + cluster_autopsies surface (FC1-N32 + FC2-N30 + FC1-N33). Predecessor TB-11 Atom 1 hash 3b2901c4 superseded.
 # 2026-05-02 TB-10 Atom 1 — `runtime::bootstrap` module (NEW file). Single source of truth preseed factory `default_pput_preseed_pairs()` consumed by both evaluator's `--task-mode self|both` preseed branch and `lean_market` user CLI bootstrap. Pure function; replay-deterministic; produces 12 entries (tb7-7-sponsor + Agent_user_0 + Agent_0..9) summing to 30_000_000 micro genesis supply.
 "src/runtime/bootstrap.rs" = "78c09eabd3a0b226c84539f414965347eb6928861141bb82af3bbaa5986cd58e"
 # 2026-05-01 TB-6 Atom 2 — chaintape adapter helpers (synthetic TaskOpen/EscrowLock/WorkTx constructors + balance seeding) NEW file.
@@ -269,15 +269,15 @@ boot_attestation_hash = "PENDING_v4_X_SELF_REFERENTIAL_COMPUTE"
 "src/top_white/predicates/registry.rs" = "d993e60c499f69ae2ae74e87a25a59595176ad4e43451bd1024c7d3e78b8d581"
 "src/bottom_white/mod.rs" = "f473d8dc7b277dbf480073e1c3d4a5fb23b270576e29d16763d16e1b5d1f4f5b"
 "src/bottom_white/cas/mod.rs" = "3d2a0d00505b9a530dffb639158ee3199a5a44a7b96e0f554208a2ef1046da36"
-"src/bottom_white/cas/schema.rs" = "70f234abe6e269b3a94e1a3bb4bedb7711fda818c039fcb0cdb60f212ac871a4"  # rehashed by TB-11 Atom 1 (2026-05-02; architect §6.1 ruling): +ObjectType::EvidenceCapsule + EvidenceManifest + CompressedRunLog variants for the TB-11 CAS-resident failure rollup.
+"src/bottom_white/cas/schema.rs" = "6427695d8dbb4320fc8f23e64b9595a797cc790c6fb9439e74ba03422e0bb988"  # rehashed by TB-15 Atom 2 (2026-05-03; architect §6.2 ruling): +ObjectType::AgentAutopsyCapsule + AutopsyPrivateDetail + MarkovEvidenceCapsule + NextSessionContext variants for the TB-15 CAS-resident autopsy + Markov substrate. Predecessor TB-11 Atom 1 hash 70f234ab superseded.
 "src/bottom_white/cas/store.rs" = "de86443f8e8900e5a756f44896107f659fdca258f541a901e68ca640bf99871c"  # rehashed by TB-7.6 (2026-05-01): append_to_sidecar combines JSON serialization + trailing newline into ONE write_all() call (relies on POSIX O_APPEND atomicity for writes ≤ PIPE_BUF). Pre-fix used two separate write_all calls (serialized then b"\n") which interleaved with concurrent writers, producing corrupted lines like {...}{...} (no separator). Bug discovered organically during TB-7 real-LLM smoke runs 2 + 5. NEW test concurrent_writers_share_index_without_race verifies fix (4 threads × 20 puts × shared repo path; reload must parse).
 "src/bottom_white/tools/mod.rs" = "3d04d5f8309fdf949334e00e949681545f21f150a4bdbcdf86b32c2f54122180"
 "src/bottom_white/tools/registry.rs" = "4ed18dcd2a7d7641663ccbdb66c895cb31cd68f4de3bddad8b5d4902633cc387"
 "src/state/mod.rs" = "6d9a9c7dd1e63b04742d0b913cc2200dc9aefff9bddb600e077ca79d855d77d1"  # rehashed by TB-14 Atom 6 B′ step 4 (2026-05-03; architect ruling §3+§4 — canonical-graph rewire): pub use price_index::CanonicalNodeGraph re-export added. Predecessor TB-14 Atom 3 hash 8b77f533 superseded. (2026-05-03; architect §5.5 + charter §3 Atom 3; FC2-N28): pub use price_index::{compute_mask_set, BoltzmannMaskPolicy} added (Atom 4 will add from_env loader on existing struct). Predecessor TB-14 Atom 2 hash e34412f6 superseded.
 "src/state/price_index.rs" = "a3b770d7715f047bf58368b223a9b746ed5c7ddd66377fd47f2322ff2fe2ff84"  # rehashed by TB-14 Atom 6 B′ step 4 (2026-05-03; architect ruling §3+§4 — canonical-graph rewire): (1) NEW `pub type CanonicalNodeGraph = BTreeMap<TxId, BTreeSet<TxId>>` (canonical-keyed parent → children edge map; replaces legacy shadow Tape consumption per Codex R1 VETO closure). (2) compute_mask_set signature change: `(econ, &Tape, policy, &price_index)` → `(econ, &CanonicalNodeGraph, policy, &price_index)`. Body rewritten — `tape.children(parent_id.0.as_str())` → `edges.get(parent_id).iter()`. Doc comments updated to reflect canonical-graph semantics + post-B′-step-4 invariant envelope. Closes Codex R1 ship audit primary VETO defect #2 at the price_index module surface. Predecessor B′ step 2 (env-validation) hash d642f709 superseded.  # rehashed by TB-14 Atom 6 follow-up (2026-05-03; architect ruling step 2 — Codex R1 secondary CHALLENGE closure): from_env() now validates each field at parse time. Rules: min_liquidity > 0 (non-positive → default); price_margin numerator+denominator both > 0 (zero either → default); beta_den > 0 (zero → default); beta_num >= 0 (negative → default); epsilon ∈ [0, 1] (den > 0 AND num ≤ den; otherwise → default pair). Fail-soft per Art.I.1 + C-027 — invalid env value falls back to that field's Default; misconfiguration must not crash the swarm but production semantics must remain inside the architect-mandated invariant envelope. +11 inline tests pinning each rule (negative/zero boundary checks per field; epsilon ≥ 0 and ≤ 1 boundaries explicitly accepted). Predecessor TB-14 Atom 4 hash 2a5c2fa7 superseded.  # rehashed by TB-14 Atom 4 (2026-05-03; charter §3 Atom 4): +BoltzmannMaskPolicy::from_env() constructor reading 7 integer env vars (BOLTZMANN_BETA_NUM/DEN, MIN_LIQUIDITY_MICRO, PRICE_MARGIN_NUM/DEN, EPSILON_NUM/DEN); fail-soft on parse error per Art.I.1 + C-027. +6 inline tests (default, no-vars, override-each-field, invalid-fallback, serde-round-trip, no-decimal-point JSON) using static Mutex per feedback_env_var_test_lock. Predecessor TB-14 Atom 3 hash 23ec5f61 superseded.
-"src/state/q_state.rs" = "eeb35da842c831f43d32e59a3efdd2cca19c4391d9f194f5be5e462896195d0b"  # rehashed by TB-14 Atom 3 (2026-05-03; architect §5.5 + charter §3 Atom 3; FC2-N28): +mask_set: BTreeSet<TxId> additive field on AgentVisibleProjection with #[serde(default)] for backward-compat with pre-TB-14 chain snapshots. Read-view filter only; never deletes from ChainTape per CR-14.3 + halt-trigger #3. Predecessor TB-14 Atom 2 hash dd5d50cc superseded.
-"src/state/sequencer.rs" = "1c6ba82fa939373463a34149d840f4a182914ddd1d4de5eba6a152ba28258a62"  # rehashed by TB-14 Atom 6 B′ step 4 (2026-05-03; architect ruling §3+§4 — canonical-graph builder): NEW `Sequencer::compute_canonical_edges_at_head() -> BTreeMap<TxId, BTreeSet<TxId>>` walks L4 + reads CAS-resident ProposalTelemetry.parent_tx for each accepted WorkTx → builds canonical parent → children edge map. Replay-deterministic per Art.0.2 (L4 + CAS payloads both replay-deterministic). Empty fallback on lock poisoned / CAS missing / decode error / no parent_tx. **Halt-trigger #2 fence preserved**: the new method body uses `crate::bottom_white::ledger::transition_ledger::canonical_decode` + `crate::runtime::proposal_telemetry::read_from_cas` (both NON-TB-14 modules — TB-3 + TB-7 surface respectively). NO TB-14 imports added to sequencer.rs `use` block. The forbidden tokens (compute_price_index / compute_mask_set / NodeMarketEntry / RationalPrice / BoltzmannMaskPolicy) remain absent from sequencer.rs. Predecessor TB-13 Atom 6 round-5 hash 2b8a1b36 superseded. (2026-05-03; Codex RQ5 remediation): drop redundant inner-consistency check `redeem.outcome != redeem.resolution_ref.claimed_outcome` from CompleteSetRedeem dispatch — ResolutionRef wrapper removed in typed_tx.rs (claimed_outcome was a redundant copy of redeem.outcome; resolution_tx_id was opaque/L4-unvalidated dead wire weight). Step 1 deleted; remaining steps renumbered 1..6. State-mismatch path (Finalized+No / Bankrupt+Yes → InvalidResolutionRef) preserved via existing match arm using redeem.outcome directly. Predecessor TB-12 Atom 2 hash be6a3ee8 superseded.  # rehashed by TB-12 Atom 2 (2026-05-03; architect 2026-05-03 ruling §3 + §8 Atom 2; Class 3): WorkTx accept arm + ChallengeTx accept arm extended with **pure additive** NodePosition side-effect — accepted WorkTx with stake>0 → FirstLong NodePosition write; accepted ChallengeTx with stake>0 → ChallengeShort NodePosition write (task_id Q-derived from target stakes_t entry). VerifyTx accept arm UNCHANGED (FR-12.3 + CR-12.8: VerifyTx.bond is responsibility bond, not market side). NO change to balances_t / stakes_t / challenge_cases_t / total_supply (CR-12.1 + CR-12.2 — NodePosition.amount NOT a Coin holding, NOT in 5-holding CTF sum). Predecessor TB-11 Atom 2 hash 2d861c68 superseded.  # rehashed by TB-11 Atom 1 (2026-05-02; architect §6.2 ruling): system_message_for_verification + system_signature_of + system_epoch_of arms extended for TaskBankruptcy; submit_agent_tx ingress fail-closed extended; dispatch_transition stub arm for TaskBankruptcy (full body lands in Atom 2); test fixture initializers updated for additive TaskExpire/TerminalSummary fields. Predecessor TB-8 round-2 hash b945cc07 superseded.
-"src/state/typed_tx.rs" = "440989781b65100efd70d8fc102841753565e4f265e3e230c2fc51e0d777f028"  # rehashed by TB-13 Atom 6 round-5 (2026-05-03; Codex RQ5 remediation): drop ResolutionRef wrapper struct (incl. resolution_tx_id + claimed_outcome) from CompleteSetRedeemTx + CompleteSetRedeemSigningPayload. Both fields were dead — resolution_tx_id was opaque/L4-unvalidated (per round-4 doc-fix), claimed_outcome was a redundant copy of redeem.outcome used only for an inner-consistency check that was itself dead defense-in-depth (signature covers both; tampering breaks signature). Net: CompleteSetRedeemTx 9→8 fields; signing payload 8→7 fields. Inner-consistency check removed from sequencer (Step 1); state-mismatch path preserved via existing match arm. InvalidResolutionRef variant + Display preserved (now reads "outcome does not match state"). Wire-format break — no production rows yet. Predecessor TB-12 Atom 1 hash 11e526a1 superseded.  # rehashed by TB-12 Atom 1 (2026-05-03; architect 2026-05-03 ruling §3 + §8 Atom 1): +PositionSide enum +PositionKind enum +NodePosition struct (immutable exposure record per architect §10; NOT Coin holding per CR-12.1). FORBIDDEN: MarketBuy/MarketSell PositionKind variants (TB-13+ trading layer). Predecessor TB-11 Atom 1 hash c30b814a superseded.  # rehashed by TB-11 Atom 1 (2026-05-02; architect §6.1+6.2 ruling): TerminalSummaryTx additive bump (+parent_state_root +solver_agent +evidence_capsule_cid; type alias `RunExhaustedTx = TerminalSummaryTx`); TaskExpireTx additive bump (+sponsor_agent +escrow_tx_id +reason); NEW TaskBankruptcyTx struct + signing payload + domain prefix + canonical-message variant; NEW 4 enums (ExpireReason / BankruptcyReason / ExhaustionReason / CapsulePrivacyPolicy); golden digest constants rotated for TaskExpire + TerminalSummary (per file-internal rotation protocol). Predecessor TB-8 Atom 3 hash 26b01d8b superseded.
+"src/state/q_state.rs" = "c23cc95d409cdebb0b91cc0b7b75aee9944d3397a5b4e014a63d11e7b248c614"  # rehashed by TB-15 Atom 3 (2026-05-03; architect §6.2 ruling): +AutopsyIndex(BTreeMap<EventId, Vec<Cid>>) newtype + agent_autopsies_t: AutopsyIndex 13th sub-field on EconomicState with #[serde(default)] for backward-compat with pre-TB-15 chain snapshots. Sequencer-side per-event Cid index; capsule bytes live in CAS; NOT projected to AgentVisibleProjection per CR-15.1 + halt-trigger #1. Sub-field count assertion 12→13. Predecessor TB-14 Atom 3 hash eeb35da8 superseded.
+"src/state/sequencer.rs" = "9fa5936230738e5f72b14713a96238ad53583c08cdd6207d8832d27b009f50a9"  # rehashed by TB-15 Atom 3 (2026-05-03; architect §6.2 ruling): TaskBankruptcyTx dispatch arm Step 3.5 — pure derive_autopsies_for_bankruptcy(&q.economic_state_t, bk, q.q_t.current_round, bk.timestamp_logical) populates q_next.economic_state_t.agent_autopsies_t[event_id] with deterministic Cids per losing-staker. apply_one Stage 3.5 — write_bankruptcy_autopsies_to_cas() side-effect writes the capsule + private_detail bytes to CAS using the same deterministic helper (replay-safe identical Cids). NO change to monetary invariants (assert_no_post_init_mint + assert_total_ctf_conserved unchanged). NO predicate registry mutation (CR-15.3 + halt-trigger #3). NOT projected to AgentVisibleProjection (CR-15.1 + halt-trigger #1). Predecessor TB-14 Atom 6 B′ step 4 hash 1c6ba82f superseded.  # rehashed by TB-14 Atom 6 B′ step 4 (2026-05-03; architect ruling §3+§4 — canonical-graph builder): NEW `Sequencer::compute_canonical_edges_at_head() -> BTreeMap<TxId, BTreeSet<TxId>>` walks L4 + reads CAS-resident ProposalTelemetry.parent_tx for each accepted WorkTx → builds canonical parent → children edge map. Replay-deterministic per Art.0.2 (L4 + CAS payloads both replay-deterministic). Empty fallback on lock poisoned / CAS missing / decode error / no parent_tx. **Halt-trigger #2 fence preserved**: the new method body uses `crate::bottom_white::ledger::transition_ledger::canonical_decode` + `crate::runtime::proposal_telemetry::read_from_cas` (both NON-TB-14 modules — TB-3 + TB-7 surface respectively). NO TB-14 imports added to sequencer.rs `use` block. The forbidden tokens (compute_price_index / compute_mask_set / NodeMarketEntry / RationalPrice / BoltzmannMaskPolicy) remain absent from sequencer.rs. Predecessor TB-13 Atom 6 round-5 hash 2b8a1b36 superseded. (2026-05-03; Codex RQ5 remediation): drop redundant inner-consistency check `redeem.outcome != redeem.resolution_ref.claimed_outcome` from CompleteSetRedeem dispatch — ResolutionRef wrapper removed in typed_tx.rs (claimed_outcome was a redundant copy of redeem.outcome; resolution_tx_id was opaque/L4-unvalidated dead wire weight). Step 1 deleted; remaining steps renumbered 1..6. State-mismatch path (Finalized+No / Bankrupt+Yes → InvalidResolutionRef) preserved via existing match arm using redeem.outcome directly. Predecessor TB-12 Atom 2 hash be6a3ee8 superseded.  # rehashed by TB-12 Atom 2 (2026-05-03; architect 2026-05-03 ruling §3 + §8 Atom 2; Class 3): WorkTx accept arm + ChallengeTx accept arm extended with **pure additive** NodePosition side-effect — accepted WorkTx with stake>0 → FirstLong NodePosition write; accepted ChallengeTx with stake>0 → ChallengeShort NodePosition write (task_id Q-derived from target stakes_t entry). VerifyTx accept arm UNCHANGED (FR-12.3 + CR-12.8: VerifyTx.bond is responsibility bond, not market side). NO change to balances_t / stakes_t / challenge_cases_t / total_supply (CR-12.1 + CR-12.2 — NodePosition.amount NOT a Coin holding, NOT in 5-holding CTF sum). Predecessor TB-11 Atom 2 hash 2d861c68 superseded.  # rehashed by TB-11 Atom 1 (2026-05-02; architect §6.2 ruling): system_message_for_verification + system_signature_of + system_epoch_of arms extended for TaskBankruptcy; submit_agent_tx ingress fail-closed extended; dispatch_transition stub arm for TaskBankruptcy (full body lands in Atom 2); test fixture initializers updated for additive TaskExpire/TerminalSummary fields. Predecessor TB-8 round-2 hash b945cc07 superseded.
+"src/state/typed_tx.rs" = "665838b02191345dedf7d312f0b33cb5214dad7475acb084bb3ac12245b3268c"  # rehashed by TB-15 Atom 2 (2026-05-03; architect §6.2 ruling): +RiskRuleId(pub String) opaque newtype for AgentAutopsyCapsule.violated_risk_rule: Option<RiskRuleId> field. CR-15.3 — opaque rule identifier; autopsy never depends on live registry. Predecessor TB-13 Atom 6 round-5 hash 44098978 superseded.  # rehashed by TB-13 Atom 6 round-5 (2026-05-03; Codex RQ5 remediation): drop ResolutionRef wrapper struct (incl. resolution_tx_id + claimed_outcome) from CompleteSetRedeemTx + CompleteSetRedeemSigningPayload. Both fields were dead — resolution_tx_id was opaque/L4-unvalidated (per round-4 doc-fix), claimed_outcome was a redundant copy of redeem.outcome used only for an inner-consistency check that was itself dead defense-in-depth (signature covers both; tampering breaks signature). Net: CompleteSetRedeemTx 9→8 fields; signing payload 8→7 fields. Inner-consistency check removed from sequencer (Step 1); state-mismatch path preserved via existing match arm. InvalidResolutionRef variant + Display preserved (now reads "outcome does not match state"). Wire-format break — no production rows yet. Predecessor TB-12 Atom 1 hash 11e526a1 superseded.  # rehashed by TB-12 Atom 1 (2026-05-03; architect 2026-05-03 ruling §3 + §8 Atom 1): +PositionSide enum +PositionKind enum +NodePosition struct (immutable exposure record per architect §10; NOT Coin holding per CR-12.1). FORBIDDEN: MarketBuy/MarketSell PositionKind variants (TB-13+ trading layer). Predecessor TB-11 Atom 1 hash c30b814a superseded.  # rehashed by TB-11 Atom 1 (2026-05-02; architect §6.1+6.2 ruling): TerminalSummaryTx additive bump (+parent_state_root +solver_agent +evidence_capsule_cid; type alias `RunExhaustedTx = TerminalSummaryTx`); TaskExpireTx additive bump (+sponsor_agent +escrow_tx_id +reason); NEW TaskBankruptcyTx struct + signing payload + domain prefix + canonical-message variant; NEW 4 enums (ExpireReason / BankruptcyReason / ExhaustionReason / CapsulePrivacyPolicy); golden digest constants rotated for TaskExpire + TerminalSummary (per file-internal rotation protocol). Predecessor TB-8 Atom 3 hash 26b01d8b superseded.
 "handover/audits/CODEX_SPEC_V13_REAUDIT_2026-04-27.md" = "494600e0d0e01a8d128e46e6272e2d4dd691a3d4089cca02b400ad4e3d9bc94e"
 "handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md" = "66de2382248042023538d65fcd04b99cb5f33110727bcebe723ed40cc9c845fa"
 "handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md" = "d275597dd83d79c0459093f2b3ee4a7a882a444f1c58a5f1c7688af920c82afc"

exec
/bin/bash -lc "git show --unified=3 31be856 -- genesis_payload.toml | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
commit 31be85690c775c959863528a8245cad35c280966
Author: gretjia <gretjia@users.noreply.github.com>
Date:   Sun May 3 23:29:15 2026 +0000

    TB-15 Atom 5 — MarkovEvidenceCapsule schema + generator binary
    
    NEW src/runtime/markov_capsule.rs:
      - ObsId(pub String) opaque newtype
      - MarkovEvidenceCapsule struct (capsule_id + previous_capsule_cid +
        constitution_hash + l4_root + l4e_root + cas_root + typical_errors +
        unresolved_obs + next_session_context_cid + sha256 +
        created_at_logical_t + tb_tag)
      - with_constitution_hash(hash_bytes) constructor (used by halt-trigger #2)
      - try_deep_history_read_with_override_check(override_set) gate
        (FR-15.5; halt-trigger #6 — false → DeepHistoryReadDenied;
        true → Ok(())). Override check is isolated from process-global env
        per feedback_env_var_test_lock (cargo's parallel runner).
      - override_set_from_env() — TURINGOS_MARKOV_OVERRIDE=1 / true
      - write_markov_capsule(...) CAS writer (puts NextSessionContext +
        MarkovEvidenceCapsule; deterministic capsule_id = sha256(canonical
        bytes with capsule_id zeroed))
      - scan_unresolved_obs(alignment_dir) → sorted Vec<ObsId> over
        handover/alignment/OBS_*.md files
      - sha256_of_file(path) → Hash helper for constitution.md hashing
      - MarkovGenError taxonomy (DeepHistoryReadDenied / Cas / Encode / Io /
        InternalLockPoisoned)
      - 8 in-module tests:
          markov_capsule_default_round_trip
          with_constitution_hash_sets_field
          deep_history_default_deny_works
          write_markov_capsule_to_cas_round_trip (writer puts 2 CAS objects)
          write_markov_capsule_deterministic_capsule_id
          markov_chain_links_via_previous_capsule_cid
          sha256_of_file_matches_manual
          scan_unresolved_obs_filters_and_sorts (only OBS_*.md picked up,
            sorted via BTreeSet)
    
    NEW src/bin/generate_markov_capsule.rs (CLI binary):
      - args: --tb-id / --out-dir / --constitution-path / --runtime-repo /
        --cas-dir / --prev-cid-hex / --alignment-dir / --no-cas
      - env: TURINGOS_MARKOV_OVERRIDE=1 (logged at startup; default-deny
        semantics enforced per FR-15.5)
      - Reads constitution.md → SHA-256
      - Scans alignment_dir for OBS_*.md → unresolved_obs
      - Writes capsule to CAS (or --no-cas mode skips CAS but still
        computes deterministic capsule_id + emits JSON pointer)
      - Emits handover/markov_capsules/MARKOV_TB-N_<date>.json + updates
        LATEST_MARKOV_CAPSULE.txt with Cid hex
      - Smoke (--no-cas + repo constitution.md): GREEN — emitted
        capsule_id 2001959a... + LATEST pointer + JSON file at
        /tmp/tb15-smoke; constitution_hash eec69545... matches manual sha256.
    
    MOD src/runtime/mod.rs — pub mod markov_capsule (FC3-N43 trace headline)
    
    MOD tests/tb_15_halt_triggers.rs:
      - Backfill halt-trigger #2 (markov_capsule_references_constitution_hash):
        SHA-256 constitution.md → MarkovEvidenceCapsule::with_constitution_hash
        (constitutional hash equality — SG-15.7).
      - Backfill halt-trigger #6 (deep_history_read_without_override_fails):
        try_deep_history_read_with_override_check(false) → Err(DeepHistoryReadDenied);
        (true) → Ok(()) (SG-15.4 + FR-15.5).
    
    MOD genesis_payload.toml — REHASH src/runtime/mod.rs:
      03fd5358... (was adfc18a4... post Atom 2; +pub mod markov_capsule)
    
    Tests:
      cargo test --workspace --lib markov_capsule: 8/8 PASS.
      cargo test --workspace --test tb_15_halt_triggers: 6/6 PASS.
      cargo test --workspace: 870 PASS / 0 fail / 150 ignored.
      cargo build --bin generate_markov_capsule: PASS.
      Binary smoke: capsule_id deterministic + pointer files emitted.
    
    CR-15.5 + halt-trigger #2: STRUCTURALLY ENFORCED — capsule schema
    embeds constitution_hash field; with_constitution_hash plumbs it
    through; SG-15.7 verified by both in-module + halt-trigger fixture.
    CR-15.6 + halt-trigger #6: STRUCTURALLY ENFORCED — try_deep_history_*
    default-denies; binary requires TURINGOS_MARKOV_OVERRIDE=1 to opt in.
    SG-15.3: GREEN — next_session_context_cid points to JSON blob with
    boot_seq describing constitution + this capsule + (optional) previous
    capsule.
    
    ALL 6 halt-triggers GREEN. ALL 8 SG green at substrate level (final
    ship-evidence run lands in Atom 6).
    
    FC-trace: FC3-N43 (MarkovEvidenceCapsule + generator)
    
    phase_id: P4 + P5 v0 prep
    roadmap_exit_criteria_addressed: P5-Exit2 (JudgeAI veto-only
      CODIFIED — capsule cannot mutate predicate registry; suggested_policy_patch
      remains opaque pointer; CR-15.4 + SG-15.8)
    kill_criteria_tested: halt-trigger #2 + #6 GREEN — all 6 halt triggers now
      pass. P4-Forbidden:全量广播 ledger / 全量广播 rejected logs +
      P5-Kill:ArchitectAI directly modifies predicate registry STRUCTURALLY
      DISCHARGED.
    
    Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

diff --git a/genesis_payload.toml b/genesis_payload.toml
index d37866c..4df048e 100644
--- a/genesis_payload.toml
+++ b/genesis_payload.toml
@@ -215,7 +215,7 @@ boot_attestation_hash = "PENDING_v4_X_SELF_REFERENTIAL_COMPUTE"
 # 2026-05-01 TB-7 Atom 1.7 — RejectionWriter BootstrapError variant + L4.E open_jsonl now fail-closed (returns BootstrapError::RejectionWriter instead of silent in-memory fallback). Per Codex audit cc7b3dd action item #1.
 # 2026-05-01 TB-7 Atom 5 — `pub mod chain_derived_run_facts;` declaration added (ChainDerivedRunFacts aggregator per ARCHITECT_RULING D4 + charter §4.4 — bit-exact structural facts from L4 + L4.E + CAS alone).
 # 2026-05-01 TB-7.7 D4 — `pub mod verification_result;` declaration added (Lean oracle verdict CAS object linked from ProposalTelemetry.verification_result_cid).
-"src/runtime/mod.rs" = "adfc18a48e73835618a58e61cdce63e8fdac68d2043f4af0e0dbe7e57f27fe50"  # rehashed by TB-15 Atom 2 (2026-05-03; architect §6.2 ruling): added `pub mod autopsy_capsule;` declaration for TB-15 AgentAutopsyCapsule schema + writer + cluster_autopsies surface (FC1-N32 + FC2-N30 + FC1-N33). Predecessor TB-11 Atom 1 hash 3b2901c4 superseded.
+"src/runtime/mod.rs" = "03fd53582977fe10865eff34824babaf5f8009fd9131f6485e477134a936d9ce"  # rehashed by TB-15 Atom 5 (2026-05-03; architect §6.2 ruling): added `pub mod markov_capsule;` declaration for TB-15 MarkovEvidenceCapsule schema + writer + default-deny gate (FC3-N43). Predecessor TB-15 Atom 2 hash adfc18a4 superseded.
 # 2026-05-02 TB-10 Atom 1 — `runtime::bootstrap` module (NEW file). Single source of truth preseed factory `default_pput_preseed_pairs()` consumed by both evaluator's `--task-mode self|both` preseed branch and `lean_market` user CLI bootstrap. Pure function; replay-deterministic; produces 12 entries (tb7-7-sponsor + Agent_user_0 + Agent_0..9) summing to 30_000_000 micro genesis supply.
 "src/runtime/bootstrap.rs" = "78c09eabd3a0b226c84539f414965347eb6928861141bb82af3bbaa5986cd58e"
 # 2026-05-01 TB-6 Atom 2 — chaintape adapter helpers (synthetic TaskOpen/EscrowLock/WorkTx constructors + balance seeding) NEW file.

exec
/bin/bash -lc "git show --unified=3 2337381 -- genesis_payload.toml | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
commit 2337381288f79419778b15f12bb4c6da8fe2ce65
Author: gretjia <gretjia@users.noreply.github.com>
Date:   Sun May 3 23:48:29 2026 +0000

    TB-15 SHIPPED — Lamarckian Autopsy + Markov EvidenceCapsule (Class 2 self-audit; 8/8 SG; 6/6 halt-triggers; closes architect §6 spec FR-15.1..6 + CR-15.1..6 + SG-15.1..8)
    
    Atom 6 (final ship). Architect §6 spec verbatim per
    handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md
    + DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN_2026-05-02.md §1.
    
    MOD src/bin/audit_dashboard.rs:
      - + DashboardReport.autopsy_event_counts: Vec<(String, u32)>
      - + DashboardReport.latest_markov_capsule_cid_hex: Option<String>
      - + read_latest_markov_pointer() helper (best-effort read of
        handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt)
      - + render_section_15(...) pure render with ARCHITECT-MANDATED
        PRIVACY BANNER "AUTOPSY IS PRIVATE" (architect §6.4 + CR-15.1).
        Surfaces event Cid counts + Markov pointer hex only — never raw
        private bytes (signature precludes; halt-trigger #5 + CR-15.2).
        Empty-state messages when no autopsies / no Markov capsule.
        Markov default-deny explanation (CR-15.6 + halt-trigger #6).
      - + 4 SG-15.6 dashboard render tests:
          sg_15_6_dashboard_carries_autopsy_is_private_banner
          sg_15_6_dashboard_renders_event_counts_only_no_raw_bytes
          sg_15_6_dashboard_explains_when_no_markov_pointer
          sg_15_6_dashboard_carries_markov_default_deny_explanation
    
    MOD tests/fc_alignment_conformance.rs — +4 TB-15 witnesses:
      - fc1_n32_write_autopsy_capsule_witness
        (writer + capsule_id sha256-derived + privacy default AuditOnly)
      - fc1_n33_derive_autopsies_witness
        (PURE deterministic helper; same inputs → same Cids)
      - fc2_n30_cluster_autopsies_witness
        (3 same-class → 1 TypicalErrorSummary; architect §3.2.3 threshold)
      - fc3_n43_markov_capsule_witness
        (constitution_hash plumbed; default-deny without override)
    
    MOD genesis_payload.toml — REHASH trust_root for 2 modified files:
      - src/bin/audit_dashboard.rs: c953244a... (was de1f71a1...)
        +autopsy/markov fields + read_latest_markov_pointer() +
        render_section_15 + 4 SG-15.6 tests.
      - tests/fc_alignment_conformance.rs: 5e257f27... (was 751c78c8...)
        +4 TB-15 witnesses.
    
    NEW handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:
      Full ship status with atom-by-atom commit list, 6/6 halt-trigger
      status, 8/8 SG status, architectural deltas, production claim, +
      open follow-ups.
    
    NEW handover/evidence/tb_15_markov_capsule_2026-05-03/:
      README + MARKOV_TB-15_2026-05-03.json + LATEST_MARKOV_CAPSULE.txt
      (genesis Markov capsule of the TuringOS v4 chain; capsule_id
      b244f16a1f3bd532d041a40fe39b2b7e7cc12fb58e18b61aedd76a8010eeb1b6;
      constitution_hash eec695459c71fbef3685583485deb431fe3b561657b2f285b7c5e7e220e59e03;
      unresolved_obs.len = 22).
    
    NEW handover/markov_capsules/ working dir (capsule + LATEST pointer).
    
    MOD handover/ai-direct/LATEST.md — TB-15 ship section prepended.
    
    MOD handover/tracer_bullets/TB_LOG.tsv — TB-15 row appended (full
      schema per TB-13/TB-14 precedent).
    
    Final test status:
      cargo test --workspace: 878 PASS / 0 fail / 150 ignored
        (+8 vs Atom 5 baseline 870; +75 net vs TB-14 ship 803)
      cargo test --workspace --test tb_15_halt_triggers: 6/6 PASS
      cargo test --workspace --bin audit_dashboard: 9/9 PASS
      cargo test --workspace --test fc_alignment_conformance: 23/23 PASS
      cargo build --bin generate_markov_capsule: PASS
      Trust Root: GREEN (all 6 rehashes propagated correctly)
    
    ALL 6 halt-triggers GREEN:
      #1 raw_logs_not_in_general_read_view (Atom 3 file-scan AgentVisibleProjection)
      #2 markov_capsule_references_constitution_hash (Atom 5 SG-15.7)
      #3 autopsy_does_not_mutate_predicates (Atom 2 file-scan; CR-15.3 + SG-15.8)
      #4 private_detail_not_in_other_agent_view (Atom 3 file-scan AutopsyIndex; SG-15.2)
      #5 typical_error_clustering_uses_summary_only (Atom 4 cluster_autopsies; CR-15.2 + SG-15.5)
      #6 deep_history_read_without_override_fails (Atom 5 try_deep_history_*; SG-15.4 + FR-15.5)
    
    ALL 8 architect §6.5 ship gates GREEN.
    ALL 4 P-roadmap exit criteria addressed (P4-Exit1/2/3 + P5-Exit1/2 prep).
    ALL 4 FC-IDs (FC1-N32 + FC1-N33 + FC2-N30 + FC3-N43) have witness tests.
    
    Production claim: "TB-15 establishes Lamarckian Autopsy + Markov
    EvidenceCapsule substrate. AgentAutopsyCapsule (per-agent, per-event,
    AuditOnly) records loss/bankruptcy events derived deterministically
    from ChainTape evidence — NEVER from agent LLM self-narration.
    agent_autopsies_t lives sequencer-side (NOT projected to
    AgentVisibleProjection per CR-15.1 + halt-trigger #1).
    TypicalErrorBroadcast clustering at N≥3 emits public_summary text +
    Cids only — NEVER private_detail_cid bytes. MarkovEvidenceCapsule
    binds constitution_hash + L4 + L4.E + CAS roots + previous capsule +
    typical_errors + unresolved_obs as next-session bootstrap default;
    deeper history requires TURINGOS_MARKOV_OVERRIDE=1. CR-15.3/15.4
    (autopsy may suggest, never mutate; JudgeAI veto-only) STRUCTURALLY
    ENFORCED via writer signature + halt-trigger #3 file-scan.
    Constitutional alignment: Art.II.1 (typical-error broadcast) +
    Art.III.1 (raw failure shielding) + Art.III.2 (read-view scoping) +
    Art.III.3 (broadcast pollution prevention) + Art.III.4 (Goodhart
    shield) + Art.V.1 (3-power separation) + Art.0.2 (Tape Canonical:
    capsule canonical bytes = CAS object referenced by capsule_id;
    replay-deterministic per derive_autopsies_for_bankruptcy PURE helper
    used by both dispatch arm and apply_one hook)."
    
    Open follow-ups (carry-forward; NOT ship blockers):
      - Multi-site autopsy wire-in (SlashLoss / ChallengeUnsuccessful /
        VerifierBondLost): wires when SlashTx ships in RSP-3.2 (TB-9) and
        contribution DAG ships in RSP-4.
      - L4 / L4.E / CAS root chain-readers in Markov generator: zero
        placeholders today; future TB wires from chain head.
      - CAS-walking dashboard §15: empty autopsy_event_counts today; TB-16
        controlled-arena will exercise live wire-in.
      - InitAI agent-side honoring of Markov default: substrate +
        binary-level default-deny ship now; agent-side enforcement P5 v1.
      - OBS_RESOLUTIONS_INDEX_TB15 explicitly DEFERRED out of TB-15 per
        charter §7-G (TB-13 audit residue, not in architect §6 spec).
    
    FC-trace: FC1-N32 + FC1-N33 + FC2-N30 + FC3-N43
    
    phase_id: P4 Information Loom (primary) + P5 MetaTape v0 prep
    roadmap_exit_criteria_addressed: P4-Exit1 + P4-Exit2 + P4-Exit3 +
      P5-Exit1 + P5-Exit2 prep
    kill_criteria_tested: P4-Forbidden:全量广播 ledger +
      P4-Forbidden:全量广播 rejected logs + P5-Kill:ArchitectAI directly
      modifies predicate registry + 6 halt-triggers ALL GREEN
    
    Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

diff --git a/genesis_payload.toml b/genesis_payload.toml
index 4df048e..2099eca 100644
--- a/genesis_payload.toml
+++ b/genesis_payload.toml
@@ -139,7 +139,7 @@ boot_attestation_hash = "PENDING_v4_X_SELF_REFERENTIAL_COMPUTE"
 "rules/MANIFEST.sha256" = "6d6fd54d46352213a857fdaf5cf70e8a893bbbd5467e5da45a3ed3314d01b0c2"
 "rules/engine.py" = "e29f062b27fa0bb486a99876438dc096f952da0f358efdca40579592c5d97713"  # rehashed by CO1.13.2 (2026-04-29): 5-line early-continue patch for trigger==pre_commit (R-022 bypass); FC3-N34 readonly verification
 ".claude/hooks/judge.sh" = "f1f8f5094057dcea3c345951f7e73fd4d269f074a407829f19e4d75830a55fc2"
-"tests/fc_alignment_conformance.rs" = "751c78c8e1a53609f83af0b3ae9684a31b9700eed8cabff643807978e0e90393"  # rehashed by TB-14 Atom 6 B′ step 4 (2026-05-03; architect ruling §3+§4 — canonical-graph rewire): fc2_n28_mask_set_publication_witness updated to build CanonicalNodeGraph (BTreeMap<TxId, BTreeSet<TxId>>) directly instead of shadow Tape; compute_mask_set new signature (econ, edges, policy, price_index). Predecessor B′ step 1+2 hash c1f14584 superseded.
+"tests/fc_alignment_conformance.rs" = "5e257f275d86032a957101b7898293fa9d6cdfc13a4bdd2a5f630fb9e940b408"  # rehashed by TB-15 Atom 6 (2026-05-03; architect §6.2 ruling): +4 TB-15 witnesses (FC1-N32 write_autopsy_capsule + FC1-N33 derive_autopsies_for_bankruptcy + FC2-N30 cluster_autopsies + FC3-N43 MarkovEvidenceCapsule + try_deep_history_read_with_override_check default-deny gate). Predecessor TB-14 Atom 6 B′ step 4 hash 751c78c8 superseded.  # rehashed by TB-14 Atom 6 B′ step 4 (2026-05-03; architect ruling §3+§4 — canonical-graph rewire): fc2_n28_mask_set_publication_witness updated to build CanonicalNodeGraph (BTreeMap<TxId, BTreeSet<TxId>>) directly instead of shadow Tape; compute_mask_set new signature (econ, edges, policy, price_index). Predecessor B′ step 1+2 hash c1f14584 superseded.
 "src/kernel.rs" = "2f15670e4811ffdbfc386e313b774c116faa5b2797cb4d722232924ef4f146ac"  # rehashed by TB-14 Atom 6 (2026-05-03; closing OBS_TB_12_LEGACY_CPMM_QUARANTINE): excised `markets` / `bounty_market` / `bounty_lp_seed` fields + `create_market` / `buy_yes` / `buy_no` / `yes_price` / `market_ticker` / `market_ticker_full` / `open_bounty_market` / `bounty_yes_price` / `resolve_bounty` / `resolve_all` methods together with `src/prediction_market.rs`. Kernel is now pure topology (V3L-45 docstring contract restored). Pricing lives entirely in `state::compute_price_index` derived view; YES/NO claims live in TB-13 `ConditionalShareBalances`. Predecessor TB-14 Atom 2 hash 0c382679 superseded.
 "src/wal.rs" = "1ac7637aa09531b1c7232163f5df48f7193251594c4ed20e0d0fc85cea8f84dc"
 "src/boot.rs" = "92d39c2b40e3104bcb76f5a1e7fad72aa608ad8ce2f9564a6d01a70fa64093b6"
@@ -236,7 +236,7 @@ boot_attestation_hash = "PENDING_v4_X_SELF_REFERENTIAL_COMPUTE"
 # 2026-05-01 TB-6 Atom 6 — `gen_run_summary` CLI thin wrapper (NEW file).
 "src/bin/gen_run_summary.rs" = "595677e44d31556383c901a692dae4c305bbffdd0a5cf4667a4679e07ec065b0"
 # 2026-05-01 TB-8 — `audit_dashboard` CLI (NEW file). Diagnostic dashboard over runtime_repo + cas; composes verify_chaintape + chain_derived_run_facts + agent_keypairs + agent_audit_trail libraries; outputs 7-section text or --json structured report. Per TB-7 charter §13.1 (post-TB-7 sequencing TB-8).
-"src/bin/audit_dashboard.rs" = "de1f71a1034b44f6d6cc722f522f0b2f4c2bba004b6184266a45bcff25a33aa3"  # rehashed by TB-14 Atom 6 (2026-05-03; architect 2026-05-03 ruling §5.1 + §5.5 SG-14.6 closing OBS_TB_12_LEGACY_CPMM_QUARANTINE): ADDITIVE — added §14 TB-14 PriceIndex render section. (1) New imports: `state::{compute_price_index, NodeMarketEntry, TaskId, TxId}`, `state::typed_tx::{NodePosition, PositionKind, PositionSide}`, `state::q_state::{AgentId, EconomicState}`, `economy::money::MicroCoin`. (2) New `DashboardReport.price_index: BTreeMap<TxId, NodeMarketEntry>` field populated by `price_index_from_exposures` helper that synthesizes an `EconomicState` from `exposures: Vec<ExposureRecordRow>` and calls canonical `state::compute_price_index` (no second source-of-truth — architect §5.1; charter §7 auto-resolution A). (3) New `render_section_14(&BTreeMap<TxId, NodeMarketEntry>) -> String` pure function. ARCHITECT-MANDATED BANNER: opens with literal "PRICE IS SIGNAL, NOT TRUTH" (architect §5.1 verbatim); SG-14.6 ship gate enforced by 4 `tb14_render_tests::sg_14_6_*` unit tests asserting (a) the banner is present, (b) prices render as `numerator/denominator` integer-rational (NEVER decimal — charter §5.6 forbidden), (c) empty PriceIndex renders explicit empty-state, (d) None-priced (zero-liquidity) nodes render as `None` per FR-14.3 / halt-trigger #5. (4) §14 invocation appended to `render_text` after §13. Predecessor TB-12 Atom 4 + Atom 6(a) hash 9881aed3 superseded. architect 2026-05-03 §8 Atom 4 + §9.3 SG-12.6): added §13 TB-12 Node exposure records section + ExposureRecordRow struct + L4 walk arms for TypedTx::Work (FirstLong row) + TypedTx::Challenge (ChallengeShort row). Refactored §13 inline render block into `render_section_13(&[ExposureRecordRow]) -> String` pure function for SG-12.6 unit-testability. Added `#[cfg(test)] mod tb12_render_tests::sg_12_6_dashboard_view_positions_works` covering empty + single-FirstLong + same-node-long+short + 2-node aggregation cases + forbidden-token grep (Open market balances / MarketBuy / etc). LABEL DISCIPLINE: "exposure records" NOT "Open market balances" (architect §8 Atom 4). Predecessors TB-11 Atom 5 hash 2e28c2b6 + earlier TB-12 hash f697ba24 superseded.  # rehashed by TB-11 Atom 5 (2026-05-02; architect §6.2 ruling): added §12 TB-11 Epistemic Exhaust + Capital Liberation section. 3 sub-tables: Exhausted runs (TerminalSummaryTx ≡ RunExhausted; run_id, task_id, outcome, attempts, capsule_cid), Expired tasks (TaskExpireTx; task_id, sponsor, refund, reason, @logical_t + total refund line), Bankrupt tasks (TaskBankruptcyTx; task_id, reason, failed_runs, capsule_cid). Privacy mandate footer (capsule public_summary surfaced; raw log shielded behind audit role). Predecessor TB-10 hash d89b2f32 superseded.
+"src/bin/audit_dashboard.rs" = "c953244a5661d08d81feed2109a451cb6f68f880863fc6ba006a9505ae60ed10"  # rehashed by TB-15 Atom 6 (2026-05-03; architect §6.5 SG-15.6): ADDITIVE — added §15 TB-15 Autopsy + Markov render section. (1) New DashboardReport fields: autopsy_event_counts: Vec<(String, u32)> + latest_markov_capsule_cid_hex: Option<String>. (2) New read_latest_markov_pointer() helper reading handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt (best-effort; None on absence). (3) New render_section_15(&[(String, u32)], Option<&str>) -> String pure function. ARCHITECT-MANDATED PRIVACY BANNER: opens with literal "AUTOPSY IS PRIVATE" (architect §6.4 + CR-15.1); SG-15.6 ship gate enforced by 4 tb14_render_tests::sg_15_6_* unit tests asserting (a) banner present, (b) input signature carries only counts + pointer hex (no raw bytes possible), (c) empty Markov pointer renders explicit generation hint, (d) Markov default-deny explanation ("TURINGOS_MARKOV_OVERRIDE=1" + "deeper history") present. (4) §15 invocation appended to render_text after §14. Predecessor TB-14 Atom 6 hash de1f71a1 superseded.  # rehashed by TB-14 Atom 6 (2026-05-03; architect 2026-05-03 ruling §5.1 + §5.5 SG-14.6 closing OBS_TB_12_LEGACY_CPMM_QUARANTINE): ADDITIVE — added §14 TB-14 PriceIndex render section. (1) New imports: `state::{compute_price_index, NodeMarketEntry, TaskId, TxId}`, `state::typed_tx::{NodePosition, PositionKind, PositionSide}`, `state::q_state::{AgentId, EconomicState}`, `economy::money::MicroCoin`. (2) New `DashboardReport.price_index: BTreeMap<TxId, NodeMarketEntry>` field populated by `price_index_from_exposures` helper that synthesizes an `EconomicState` from `exposures: Vec<ExposureRecordRow>` and calls canonical `state::compute_price_index` (no second source-of-truth — architect §5.1; charter §7 auto-resolution A). (3) New `render_section_14(&BTreeMap<TxId, NodeMarketEntry>) -> String` pure function. ARCHITECT-MANDATED BANNER: opens with literal "PRICE IS SIGNAL, NOT TRUTH" (architect §5.1 verbatim); SG-14.6 ship gate enforced by 4 `tb14_render_tests::sg_14_6_*` unit tests asserting (a) the banner is present, (b) prices render as `numerator/denominator` integer-rational (NEVER decimal — charter §5.6 forbidden), (c) empty PriceIndex renders explicit empty-state, (d) None-priced (zero-liquidity) nodes render as `None` per FR-14.3 / halt-trigger #5. (4) §14 invocation appended to `render_text` after §13. Predecessor TB-12 Atom 4 + Atom 6(a) hash 9881aed3 superseded. architect 2026-05-03 §8 Atom 4 + §9.3 SG-12.6): added §13 TB-12 Node exposure records section + ExposureRecordRow struct + L4 walk arms for TypedTx::Work (FirstLong row) + TypedTx::Challenge (ChallengeShort row). Refactored §13 inline render block into `render_section_13(&[ExposureRecordRow]) -> String` pure function for SG-12.6 unit-testability. Added `#[cfg(test)] mod tb12_render_tests::sg_12_6_dashboard_view_positions_works` covering empty + single-FirstLong + same-node-long+short + 2-node aggregation cases + forbidden-token grep (Open market balances / MarketBuy / etc). LABEL DISCIPLINE: "exposure records" NOT "Open market balances" (architect §8 Atom 4). Predecessors TB-11 Atom 5 hash 2e28c2b6 + earlier TB-12 hash f697ba24 superseded.  # rehashed by TB-11 Atom 5 (2026-05-02; architect §6.2 ruling): added §12 TB-11 Epistemic Exhaust + Capital Liberation section. 3 sub-tables: Exhausted runs (TerminalSummaryTx ≡ RunExhausted; run_id, task_id, outcome, attempts, capsule_cid), Expired tasks (TaskExpireTx; task_id, sponsor, refund, reason, @logical_t + total refund line), Bankrupt tasks (TaskBankruptcyTx; task_id, reason, failed_runs, capsule_cid). Privacy mandate footer (capsule public_summary surfaced; raw log shielded behind audit role). Predecessor TB-10 hash d89b2f32 superseded.
 # 2026-05-01 TB-6 Atom 6 — RunSummary integration tests (I92 / I92b / I92c) (NEW file).
 "tests/tb_6_run_summary.rs" = "6efb4baa36d08dc55c1140ea3279a0088a2f14f58cc71e9283a876ffa2da68d8"
 # 2026-05-01 TB-7 Atom 2 — authoritative routing integration tests (NEW file). I100 (real-signature WorkTx verifies via manifest) + I101 (zero-stake WorkTx → L4.E) + I102 (distinct agents → distinct pubkeys). Gate 1 + Gate 4 wired evidence.

exec
/bin/bash -lc 'git show f06d548''^:src/runtime/mod.rs | sha256sum; git show f06d548''^:src/bottom_white/cas/schema.rs | sha256sum; git show f06d548''^:src/state/q_state.rs | sha256sum; git show f06d548''^:src/state/sequencer.rs | sha256sum; git show f06d548''^:src/state/typed_tx.rs | sha256sum' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
adfc18a48e73835618a58e61cdce63e8fdac68d2043f4af0e0dbe7e57f27fe50  -
6427695d8dbb4320fc8f23e64b9595a797cc790c6fb9439e74ba03422e0bb988  -
eeb35da842c831f43d32e59a3efdd2cca19c4391d9f194f5be5e462896195d0b  -
1c6ba82fa939373463a34149d840f4a182914ddd1d4de5eba6a152ba28258a62  -
665838b02191345dedf7d312f0b33cb5214dad7475acb084bb3ac12245b3268c  -

exec
/bin/bash -lc 'git show 31be856''^:src/runtime/mod.rs | sha256sum' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
adfc18a48e73835618a58e61cdce63e8fdac68d2043f4af0e0dbe7e57f27fe50  -

exec
/bin/bash -lc 'git show 2337381''^:src/bin/audit_dashboard.rs | sha256sum; git show 2337381''^:tests/fc_alignment_conformance.rs | sha256sum' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
de1f71a1034b44f6d6cc722f522f0b2f4c2bba004b6184266a45bcff25a33aa3  -
751c78c8e1a53609f83af0b3ae9684a31b9700eed8cabff643807978e0e90393  -

exec
/bin/bash -lc 'git show 9daba5a:tests/fc_alignment_conformance.rs | sha256sum; git show 44cd480:src/bin/audit_dashboard.rs | sha256sum' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
751c78c8e1a53609f83af0b3ae9684a31b9700eed8cabff643807978e0e90393  -
de1f71a1034b44f6d6cc722f522f0b2f4c2bba004b6184266a45bcff25a33aa3  -

exec
/bin/bash -lc "git show --unified=3 6594d3f -- genesis_payload.toml | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
exec
/bin/bash -lc 'git show 6594d3f''^:src/state/typed_tx.rs | sha256sum; git show 6594d3f''^:src/bottom_white/cas/schema.rs | sha256sum; git show 6594d3f''^:src/runtime/mod.rs | sha256sum' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
440989781b65100efd70d8fc102841753565e4f265e3e230c2fc51e0d777f028  -
70f234abe6e269b3a94e1a3bb4bedb7711fda818c039fcb0cdb60f212ac871a4  -
3b2901c4890a46998a8ca6156c207013a0fefdc22a1100ef999f405d7b224de0  -

exec
/bin/bash -lc "git show --name-status --oneline 6594d3f | sed -n '1,80p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
6594d3f TB-15 Atom 2 — AgentAutopsyCapsule schema + writer
M	src/bottom_white/cas/schema.rs
A	src/runtime/autopsy_capsule.rs
M	src/runtime/mod.rs
M	src/state/typed_tx.rs
M	tests/tb_15_halt_triggers.rs

exec
/bin/bash -lc "git show --unified=3 6594d3f -- src/runtime/mod.rs src/state/typed_tx.rs src/bottom_white/cas/schema.rs | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
commit 6594d3fc680e8d273df2eeeb44ee766cefc0dd2e
Author: gretjia <gretjia@users.noreply.github.com>
Date:   Sun May 3 22:46:09 2026 +0000

    TB-15 Atom 2 — AgentAutopsyCapsule schema + writer
    
    NEW src/runtime/autopsy_capsule.rs:
      - LossReasonClass enum (8 variants: SlashLoss / Bankruptcy /
        ChallengeUnsuccessful / VerifierBondLost / AdverseSelection /
        Overleverage / Goodhart / Other(String) — architect §1.1 hint list +
        runtime additions; default Bankruptcy; .tag() returns stable string
        for clustering)
      - AgentAutopsyCapsule struct (capsule_id + agent_id + event_id +
        loss_amount + loss_reason_class + violated_risk_rule:
        Option<RiskRuleId> + suggested_policy_patch: Option<Cid> +
        evidence_cids: Vec<Cid> + public_summary + private_detail_cid +
        privacy_policy + sha256 + created_at_logical_t + created_at_round)
      - format_public_summary(...) deterministic broadcast string formatter
      - write_autopsy_capsule(...) CAS writer (puts AutopsyPrivateDetail +
        AgentAutopsyCapsule; returns capsule with capsule_id = sha256(canonical
        bytes with capsule_id zeroed))
      - AutopsyWriteError taxonomy (Cas / Encode / InternalLockPoisoned)
    
    MOD src/state/typed_tx.rs:
      - + RiskRuleId(pub String) newtype (CR-15.3 — opaque rule identifier;
        autopsy never depends on live registry)
    
    MOD src/bottom_white/cas/schema.rs:
      - + ObjectType::AgentAutopsyCapsule (canonical capsule bytes)
      - + ObjectType::AutopsyPrivateDetail (audit-only detail JSON)
      - + ObjectType::MarkovEvidenceCapsule (forward — Atom 5)
      - + ObjectType::NextSessionContext (forward — Atom 5; FR-15.4 boot blob)
    
    MOD src/runtime/mod.rs:
      - pub mod autopsy_capsule (with FC1-N32 + FC2-N30 trace headline)
    
    MOD tests/tb_15_halt_triggers.rs:
      - Backfill halt-trigger #3 (autopsy_does_not_mutate_predicates):
        file-level scan ensures src/runtime/autopsy_capsule.rs contains no
        `&mut PredicateRegistry` / `&mut ToolRegistry` / `&mut RiskPolicyRegistry`
        / `.register_predicate(` / `.unregister_predicate(` / `.patch_predicate(`
        / `.register_tool(` / `.unregister_tool(`. CR-15.3 + SG-15.8.
    
    Tests (all green):
      6 in-module: autopsy_capsule_default_round_trip + format_public_summary
        _contains_agent_amount_reason + privacy_policy_default_is_audit_only +
        write_autopsy_capsule_to_cas_round_trip (writer puts 2 CAS objects;
        capsule_id matches sha256; private_detail bytes retrievable) +
        write_autopsy_capsule_deterministic_capsule_id (same inputs → same
        capsule_id + same private_detail_cid) + loss_reason_class_tag_stable
      Halt-trigger #3 GREEN; #1/#2/#4/#5/#6 still unimplemented!() (Atoms 3-5
        backfill).
    
    Privacy contract (architect §6.4):
      - public_summary low-info; eligible for typical-error broadcast IFF
        N≥3 cluster (Atom 4)
      - private_detail_cid AuditOnly; NEVER enters AgentVisibleProjection
      - evidence_cids = Cids of pre-existing public ChainTape objects only
    
    CR-15.3 / SG-15.8 (autopsy does not mutate predicates) STRUCTURALLY
    ENFORCED by writer signature + halt-trigger #3 file-scan fence.
    
    cargo check --workspace: clean (pre-existing warnings only).
    cargo test --workspace --lib autopsy_capsule: 6/6 PASS.
    cargo test --workspace --test tb_15_halt_triggers: 1/6 PASS (#3 green;
      remaining 5 still unimplemented per Atom 1 design).
    
    FC-trace: FC1-N32 (write_autopsy_capsule writer)
    
    phase_id: P4 + P5 v0 prep
    roadmap_exit_criteria_addressed: P4-Exit3 (broadcast surface foundation
      — public_summary low-info text only; private_detail_cid opaque) +
      P5-Exit1 (predicate registry untouched by autopsy — CR-15.3
      STRUCTURALLY ENFORCED)
    kill_criteria_tested: halt-trigger #3 GREEN
      (autopsy_does_not_mutate_predicates)
    
    Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

diff --git a/src/bottom_white/cas/schema.rs b/src/bottom_white/cas/schema.rs
index ca4a3a3..c2f4fdc 100644
--- a/src/bottom_white/cas/schema.rs
+++ b/src/bottom_white/cas/schema.rs
@@ -64,6 +64,29 @@ pub enum ObjectType {
     /// Privacy default `CapsulePrivacyPolicy::AuditOnly` — never enters
     /// Agent read view.
     CompressedRunLog,
+    /// TB-15 (architect §6.2): canonical-encoded `AgentAutopsyCapsule`
+    /// bytes. Per-agent, per-event loss capsule derived from ChainTape
+    /// evidence (NEVER from agent self-narration). Anchored from
+    /// `EconomicState.agent_autopsies_t[event_id]: Vec<Cid>`.
+    /// Privacy default `CapsulePrivacyPolicy::AuditOnly`; only
+    /// `public_summary` text may broadcast on N≥3 typical-error cluster.
+    AgentAutopsyCapsule,
+    /// TB-15 (architect §6.2): private-detail JSON for an
+    /// `AgentAutopsyCapsule`. Referenced by
+    /// `AgentAutopsyCapsule.private_detail_cid`. Audit-only by default;
+    /// MUST NOT enter `AgentVisibleProjection`.
+    AutopsyPrivateDetail,
+    /// TB-15 (architect §6.2): canonical-encoded `MarkovEvidenceCapsule`
+    /// bytes. End-of-TB rollup binding constitution_hash + L4 root +
+    /// L4.E root + CAS root + previous_capsule_cid + typical_errors +
+    /// unresolved_obs + next_session_context_cid. Default next-session
+    /// bootstrap source; deeper history requires `TURINGOS_MARKOV_OVERRIDE=1`.
+    MarkovEvidenceCapsule,
+    /// TB-15 (architect §6.2 FR-15.4): JSON blob describing the next
+    /// session's default boot context (`{constitution_hash,
+    /// latest_markov_cid, boot_seq[]}`). Referenced by
+    /// `MarkovEvidenceCapsule.next_session_context_cid`.
+    NextSessionContext,
     /// Generic / unclassified blob.
     Generic,
 }
diff --git a/src/runtime/mod.rs b/src/runtime/mod.rs
index af700a0..4f21a4d 100644
--- a/src/runtime/mod.rs
+++ b/src/runtime/mod.rs
@@ -50,6 +50,9 @@ pub mod genesis_report;
 /// TRACE_MATRIX TB-11 Atom 1 (architect §6.1 ruling 2026-05-02): EvidenceCapsule schema + writer surface. CAS-resident rollup of failed-run evidence (attempt_count / lean_error_count / sorry_block_count / parse_failure_count / partial_accept_count + compressed_log_cid + privacy_policy). Anchored on chain by `TerminalSummaryTx.evidence_capsule_cid` (architect's RunExhaustedTx role) and `TaskBankruptcyTx.evidence_capsule_cid`. Privacy default `AuditOnly` per architect §6.1 屏蔽规则.
 pub mod evidence_capsule;
 
+/// TRACE_MATRIX FC1-N32 + FC2-N30 (TB-15 Atom 2 + 4; architect §6.2 ruling 2026-05-02 + 2026-05-03): per-agent `AgentAutopsyCapsule` schema + writer (Atom 2) and `cluster_autopsies` typical-error broadcast (Atom 4). CAS-resident; AuditOnly default. Derived from ChainTape evidence — NEVER LLM self-narrative. Anchored on `EconomicState.agent_autopsies_t[event_id]: Vec<Cid>` (Atom 3).
+pub mod autopsy_capsule;
+
 /// TRACE_MATRIX FC2 Boot: TB-10 Atom 1 — Reusable preseed factory for chaintape genesis QState. Single source of truth for `tb7-7-sponsor` + `Agent_user_0` + `Agent_0..9` initial balances. Consumed by both evaluator (`--task-mode self|both`) and `lean_market` user CLI bootstrap. Pure function; replay-deterministic.
 pub mod bootstrap;
 
diff --git a/src/state/typed_tx.rs b/src/state/typed_tx.rs
index 083efd2..ef64ca4 100644
--- a/src/state/typed_tx.rs
+++ b/src/state/typed_tx.rs
@@ -1095,6 +1095,17 @@ impl Default for OutcomeSide {
     }
 }
 
+/// TRACE_MATRIX TB-15 Atom 2 (architect §6.2): identifier for a
+/// protocol-level risk rule (`max_position_size`, `max_drawdown`,
+/// `max_slippage`, `max_leverage`, `kelly_cap`, ...). Carried by
+/// `AgentAutopsyCapsule.violated_risk_rule` as `Option<RiskRuleId>` —
+/// names the protocol invariant that triggered the loss event, when
+/// applicable. Opaque newtype so the autopsy writer never depends on
+/// the live risk-rule registry (CR-15.3 — autopsy may suggest, never
+/// mutate).
+#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
+pub struct RiskRuleId(pub String);
+
 /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): non-negative share count.
 ///
 /// Architect spec uses `units: i128`; we tighten to `u128` because TB-13

exec
/bin/bash -lc "git grep -n \"EconomicState\" -- ':"'!target'"' | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc "git grep -n \"12 sub-fields\\|12 subfields\\|twelve\\|Twelve\\|obj.len(),[[:space:]]*12\\|assert_eq"'!(.*12'"\\|must have 12\\|12→13\\|12 -> 13\" -- tests src handover | sed -n '1,240p'" in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc "git grep -n \"13 sub-fields\\|thirteen\\|must have 13\\|obj.len(),[[:space:]]*13\\|assert_eq"'!(.*13" -- tests src | sed -n '"'1,220p'" in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc "git grep -n \"agent_autopsies_t\\|economic_state_has_thirteen\\|empty_economic_state_serializes\\|axiom_3\" -- tests src | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 214ms:
src/bin/audit_dashboard.rs:139:    /// counts derived from on-chain `EconomicState.agent_autopsies_t`
src/bin/audit_dashboard.rs:955:        // EconomicState.agent_autopsies_t at snapshot time. Build_report
src/bin/audit_dashboard.rs:1555:        s.push_str("  (no agent_autopsies_t entries in this snapshot — no\n");
src/bottom_white/cas/schema.rs:70:    /// `EconomicState.agent_autopsies_t[event_id]: Vec<Cid>`.
src/runtime/autopsy_capsule.rs:10://! Anchored on `EconomicState.agent_autopsies_t[event_id]: Vec<Cid>`
src/runtime/autopsy_capsule.rs:413:// `EconomicState.agent_autopsies_t` with deterministic Cids) AND by
src/runtime/autopsy_capsule.rs:431:/// - dispatch arm: capsule_id population into `agent_autopsies_t`
src/runtime/autopsy_capsule.rs:501:/// dispatch arm already populated into `agent_autopsies_t`).
src/runtime/mod.rs:53:/// TRACE_MATRIX FC1-N32 + FC2-N30 (TB-15 Atom 2 + 4; architect §6.2 ruling 2026-05-02 + 2026-05-03): per-agent `AgentAutopsyCapsule` schema + writer (Atom 2) and `cluster_autopsies` typical-error broadcast (Atom 4). CAS-resident; AuditOnly default. Derived from ChainTape evidence — NEVER LLM self-narrative. Anchored on `EconomicState.agent_autopsies_t[event_id]: Vec<Cid>` (Atom 3).
src/state/q_state.rs:257:    pub agent_autopsies_t: AutopsyIndex,
src/state/q_state.rs:836:    fn economic_state_has_thirteen_sub_fields() {
src/state/q_state.rs:852:        // with +agent_autopsies_t (`AutopsyIndex` = `BTreeMap<EventId, Vec<Cid>>`;
src/state/q_state.rs:861:            "EconomicState must have 13 sub-fields post-TB-15 (was 12 post-TB-14; +agent_autopsies_t); got {}",
src/state/q_state.rs:868:        assert!(obj.contains_key("agent_autopsies_t"), "TB-15 agent_autopsies_t sub-field missing");
src/state/sequencer.rs:1361:            // AgentAutopsyCapsule Cids into agent_autopsies_t for each
src/state/sequencer.rs:1378:                    .agent_autopsies_t
tests/economic_state_reconstruct.rs:24:    // (+agent_autopsies_t — `AutopsyIndex` = `BTreeMap<EventId, Vec<Cid>>`;
tests/economic_state_reconstruct.rs:44:        "agent_autopsies_t",               // TB-15 Atom 3 (architect §6.2 ruling 2026-05-02 + 2026-05-03)
tests/q_state_reconstruct.rs:85:fn empty_economic_state_serializes_to_thirteen_sub_fields() {
tests/q_state_reconstruct.rs:95:    //   (+agent_autopsies_t — `AutopsyIndex` per-event Cid index for
tests/six_axioms_alignment.rs:62:/// +agent_autopsies_t (`AutopsyIndex` = `BTreeMap<EventId, Vec<Cid>>`;
tests/six_axioms_alignment.rs:67:fn axiom_3_economic_state_present_and_complete() {
tests/tb_15_halt_triggers.rs:22:// Filled in by Atom 3 (after EconomicState gains agent_autopsies_t).
tests/tb_15_halt_triggers.rs:158:// Filled in by Atom 3 (after EconomicState gains agent_autopsies_t).
tests/tb_15_halt_triggers.rs:199:             NOT `{}` — agent_autopsies_t stores Cids only; raw bytes \

 succeeded in 443ms:
src/runtime/evidence_capsule.rs:377:        assert_eq!(capsule.attempt_count, 132);
src/state/q_state.rs:836:    fn economic_state_has_thirteen_sub_fields() {
src/state/q_state.rs:842:        // 11 → 13 sub-fields with +conditional_collateral_t (CR-13.4 Coin
src/state/q_state.rs:851:        // TB-15 Atom 3 (2026-05-03 architect ruling §6.2): 12 → 13 sub-fields
src/state/q_state.rs:861:            "EconomicState must have 13 sub-fields post-TB-15 (was 12 post-TB-14; +agent_autopsies_t); got {}",
tests/economic_state_reconstruct.rs:14:fn thirteen_sub_fields_present() {
tests/economic_state_reconstruct.rs:46:    assert_eq!(obj.len(), 13);
tests/q_state_reconstruct.rs:85:fn empty_economic_state_serializes_to_thirteen_sub_fields() {
tests/q_state_reconstruct.rs:101:    assert_eq!(obj.len(), 13);
tests/six_axioms_alignment.rs:70:    assert_eq!(e.as_object().unwrap().len(), 13);
tests/tb_11_epistemic_exhaust.rs:154:    assert_eq!(entry.attempt_count, 132);
tests/tb_13_chaintape_smoke.rs:393:    // reconstruction of the TB-13 sub-fields. This closes the "state-root

 succeeded in 504ms:
X_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:8617:   143	pub struct EconomicState {
handover/audits/CODEX_CO1_2_QSTATE_AUDIT_2026-04-27.md:17:CHALLENGE Q4. TRACE comments exist for major structs: Hash, NodeId, AgentId, TxId, Reputation, AgentSwarmState, PerAgentState, AgentVisibleProjection, BudgetSnapshot, EconomicState, all index wrappers, RoyaltyGraph, ChallengeCasesIndex, PriceIndex, QState, NodeId::from_state_root, QState::genesis. Missing/insufficient attached `/// TRACE_MATRIX <id>: <role>` for `pub mod q_state`, `pub use` exports, Hash::ZERO/from_bytes, EscrowEntry, StakeEntry, ClaimEntry, TaskMarketEntry, RoyaltyEdge, ChallengeCase, and public fields.
handover/audits/CODEX_CO1_2_QSTATE_AUDIT_2026-04-27.md:19:PASS Q5. Determinism tests are present: inline `btreemap_insertion_order_independent_serialization`, plus `q_state_reconstruct::balances_index_insertion_order_independence` and `economic_state_reconstruct::balances_insertion_order_independence`. Defaults exist for QState, EconomicState, indices, and stub entries.
handover/audits/CODEX_CO1_2_QSTATE_AUDIT_2026-04-27.md:30:**CHALLENGE** — Q_t and EconomicState match the binding field specs, deterministic BTreeMap discipline holds, trust-root hashes are correct, and tests are green. The remaining blockers are trace-doc completeness against TRACE_MATRIX_v3's exact public-symbol rule and incomplete serde forward-compat hardening for placeholder economic entry structs.
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:189:| `Work(WorkTx)` | `step_transition` | STATE § 3 (lines 328-419) | 100-150 | PredicateRegistry, ToolRegistry, EconomicState (balances/stakes/escrows/claims), ChallengeWindow.open |
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:305:**`src/sdk/tools/wallet.rs` is NOT touched by CO1.7.5**. Per CO1.1.4-pre1, EconomicState mutations within transition bodies happen via `q.economic_state_t.{balances_t, stakes_t, escrows_t, claims_t}.method_call()` types under `src/economy/` (not the wallet tool). Smoke S5 verifies 0 hits. CLAUDE.md + STEP_B_PROTOCOL.md path drift (`src/wallet.rs` → `src/sdk/tools/wallet.rs`) was sedimented and fixed in `handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` during smoke phase; the STEP_B-restricted set now correctly reads `src/kernel.rs` + `src/bus.rs` + `src/sdk/tools/wallet.rs`.
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:1627:    pub economic_state_t: EconomicState,
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:1644:pub struct EconomicState {
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:6294://! TRACE_MATRIX WP § 2 economic — `EconomicState` 9 sub-fields (CO1.2.2).
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:6421:// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:6429:pub struct EconomicState {
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:6609:    pub economic_state_t: EconomicState,
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:6671:        let e = EconomicState::default();
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:6677:            "EconomicState must have 9 sub-fields per WP § 2; got {}",
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:8126:    82	| `Work(WorkTx)` | `step_transition` | STATE § 3 (lines 328-419) | 100-150 | PredicateRegistry, ToolRegistry, EconomicState (balances/stakes/escrows/claims), ChallengeWindow.open |
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:8242:   198	**`src/sdk/tools/wallet.rs` is NOT touched by CO1.7.5**. Per CO1.1.4-pre1, EconomicState mutations within transition bodies happen via `q.economic_state_t.{balances_t, stakes_t, escrows_t, claims_t}.method_call()` types under `src/economy/` (not the wallet tool). Smoke S5 verifies 0 hits. CLAUDE.md + STEP_B_PROTOCOL.md path drift (`src/wallet.rs` → `src/sdk/tools/wallet.rs`) was sedimented and fixed in `handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` during smoke phase; the STEP_B-restricted set now correctly reads `src/kernel.rs` + `src/bus.rs` + `src/sdk/tools/wallet.rs`.
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:9917:   135	// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:9925:   143	pub struct EconomicState {
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:13664:handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:305:**`src/sdk/tools/wallet.rs` is NOT touched by CO1.7.5**. Per CO1.1.4-pre1, EconomicState mutations within transition bodies happen via `q.economic_state_t.{balances_t, stakes_t, escrows_t, claims_t}.method_call()` types under `src/economy/` (not the wallet tool). Smoke S5 verifies 0 hits. CLAUDE.md + STEP_B_PROTOCOL.md path drift (`src/wallet.rs` → `src/sdk/tools/wallet.rs`) was sedimented and fixed in `handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` during smoke phase; the STEP_B-restricted set now correctly reads `src/kernel.rs` + `src/bus.rs` + `src/sdk/tools/wallet.rs`.
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:13874:handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:8242:   198	**`src/sdk/tools/wallet.rs` is NOT touched by CO1.7.5**. Per CO1.1.4-pre1, EconomicState mutations within transition bodies happen via `q.economic_state_t.{balances_t, stakes_t, escrows_t, claims_t}.method_call()` types under `src/economy/` (not the wallet tool). Smoke S5 verifies 0 hits. CLAUDE.md + STEP_B_PROTOCOL.md path drift (`src/wallet.rs` → `src/sdk/tools/wallet.rs`) was sedimented and fixed in `handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` during smoke phase; the STEP_B-restricted set now correctly reads `src/kernel.rs` + `src/bus.rs` + `src/sdk/tools/wallet.rs`.
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:15203:handover/specs/CO1_7_5_TRANSITION_BODIES_AND_RUNTIME_WIRING_v1_2026-04-29.md:198:**`src/sdk/tools/wallet.rs` is NOT touched by CO1.7.5**. Per CO1.1.4-pre1, EconomicState mutations within transition bodies happen via `q.economic_state_t.{balances_t, stakes_t, escrows_t, claims_t}.method_call()` types under `src/economy/` (not the wallet tool). Smoke S5 verifies 0 hits. CLAUDE.md + STEP_B_PROTOCOL.md path drift (`src/wallet.rs` → `src/sdk/tools/wallet.rs`) was sedimented and fixed in `handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` during smoke phase; the STEP_B-restricted set now correctly reads `src/kernel.rs` + `src/bus.rs` + `src/sdk/tools/wallet.rs`.
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:17788:handover/specs/CO1_7_5_TRANSITION_BODIES_AND_RUNTIME_WIRING_v1_2026-04-29.md:198:**`src/sdk/tools/wallet.rs` is NOT touched by CO1.7.5**. Per CO1.1.4-pre1, EconomicState mutations within transition bodies happen via `q.economic_state_t.{balances_t, stakes_t, escrows_t, claims_t}.method_call()` types under `src/economy/` (not the wallet tool). Smoke S5 verifies 0 hits. CLAUDE.md + STEP_B_PROTOCOL.md path drift (`src/wallet.rs` → `src/sdk/tools/wallet.rs`) was sedimented and fixed in `handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` during smoke phase; the STEP_B-restricted set now correctly reads `src/kernel.rs` + `src/bus.rs` + `src/sdk/tools/wallet.rs`.
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:17918:handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:305:**`src/sdk/tools/wallet.rs` is NOT touched by CO1.7.5**. Per CO1.1.4-pre1, EconomicState mutations within transition bodies happen via `q.economic_state_t.{balances_t, stakes_t, escrows_t, claims_t}.method_call()` types under `src/economy/` (not the wallet tool). Smoke S5 verifies 0 hits. CLAUDE.md + STEP_B_PROTOCOL.md path drift (`src/wallet.rs` → `src/sdk/tools/wallet.rs`) was sedimented and fixed in `handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` during smoke phase; the STEP_B-restricted set now correctly reads `src/kernel.rs` + `src/bus.rs` + `src/sdk/tools/wallet.rs`.
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:18094:handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:8242:   198	**`src/sdk/tools/wallet.rs` is NOT touched by CO1.7.5**. Per CO1.1.4-pre1, EconomicState mutations within transition bodies happen via `q.economic_state_t.{balances_t, stakes_t, escrows_t, claims_t}.method_call()` types under `src/economy/` (not the wallet tool). Smoke S5 verifies 0 hits. CLAUDE.md + STEP_B_PROTOCOL.md path drift (`src/wallet.rs` → `src/sdk/tools/wallet.rs`) was sedimented and fixed in `handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` during smoke phase; the STEP_B-restricted set now correctly reads `src/kernel.rs` + `src/bus.rs` + `src/sdk/tools/wallet.rs`.
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:19010:handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:13664:handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:305:**`src/sdk/tools/wallet.rs` is NOT touched by CO1.7.5**. Per CO1.1.4-pre1, EconomicState mutations within transition bodies happen via `q.economic_state_t.{balances_t, stakes_t, escrows_t, claims_t}.method_call()` types under `src/economy/` (not the wallet tool). Smoke S5 verifies 0 hits. CLAUDE.md + STEP_B_PROTOCOL.md path drift (`src/wallet.rs` → `src/sdk/tools/wallet.rs`) was sedimented and fixed in `handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` during smoke phase; the STEP_B-restricted set now correctly reads `src/kernel.rs` + `src/bus.rs` + `src/sdk/tools/wallet.rs`.
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:19220:handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:13874:handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:8242:   198	**`src/sdk/tools/wallet.rs` is NOT touched by CO1.7.5**. Per CO1.1.4-pre1, EconomicState mutations within transition bodies happen via `q.economic_state_t.{balances_t, stakes_t, escrows_t, claims_t}.method_call()` types under `src/economy/` (not the wallet tool). Smoke S5 verifies 0 hits. CLAUDE.md + STEP_B_PROTOCOL.md path drift (`src/wallet.rs` → `src/sdk/tools/wallet.rs`) was sedimented and fixed in `handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` during smoke phase; the STEP_B-restricted set now correctly reads `src/kernel.rs` + `src/bus.rs` + `src/sdk/tools/wallet.rs`.
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:19713:handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:15203:handover/specs/CO1_7_5_TRANSITION_BODIES_AND_RUNTIME_WIRING_v1_2026-04-29.md:198:**`src/sdk/tools/wallet.rs` is NOT touched by CO1.7.5**. Per CO1.1.4-pre1, EconomicState mutations within transition bodies happen via `q.economic_state_t.{balances_t, stakes_t, escrows_t, claims_t}.method_call()` types under `src/economy/` (not the wallet tool). Smoke S5 verifies 0 hits. CLAUDE.md + STEP_B_PROTOCOL.md path drift (`src/wallet.rs` → `src/sdk/tools/wallet.rs`) was sedimented and fixed in `handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` during smoke phase; the STEP_B-restricted set now correctly reads `src/kernel.rs` + `src/bus.rs` + `src/sdk/tools/wallet.rs`.
handover/audits/CODEX_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md:539:| `q_next.economic_state_t.derive_state_root()` | Method does not exist on EconomicState | No state-root derivation |
handover/audits/CODEX_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md:4562://! TRACE_MATRIX WP § 2 economic — `EconomicState` 9 sub-fields (CO1.2.2).
handover/audits/CODEX_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md:4689:// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
handover/audits/CODEX_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md:4697:pub struct EconomicState {
handover/audits/CODEX_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md:4877:    pub economic_state_t: EconomicState,
handover/audits/CODEX_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md:4939:        let e = EconomicState::default();
handover/audits/CODEX_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md:4945:            "EconomicState must have 9 sub-fields per WP § 2; got {}",
handover/audits/CODEX_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md:4282://! TRACE_MATRIX WP § 2 economic — `EconomicState` 9 sub-fields (CO1.2.2).
handover/audits/CODEX_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md:4409:// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
handover/audits/CODEX_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md:4417:pub struct EconomicState {
handover/audits/CODEX_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md:4597:    pub economic_state_t: EconomicState,
handover/audits/CODEX_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md:4659:        let e = EconomicState::default();
handover/audits/CODEX_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md:4665:            "EconomicState must have 9 sub-fields per WP § 2; got {}",
handover/audits/CODEX_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md:7231:    20	    ChallengeCase, ChallengeCasesIndex, ClaimEntry, ClaimsIndex, EconomicState, EscrowEntry,
handover/audits/CODEX_CO1_7_EXTRA_ROUND4_AUDIT_2026-04-29.md:2855:    20	    ChallengeCase, ChallengeCasesIndex, ClaimEntry, ClaimsIndex, EconomicState, EscrowEntry,
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:6146://! TRACE_MATRIX WP § 2 economic — `EconomicState` 9 sub-fields (CO1.2.2).
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:6273:// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:6281:pub struct EconomicState {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:6461:    pub economic_state_t: EconomicState,
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:6523:        let e = EconomicState::default();
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:6529:            "EconomicState must have 9 sub-fields per WP § 2; got {}",
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:10536:     8	//! TRACE_MATRIX WP § 2 economic — `EconomicState` 9 sub-fields (CO1.2.2).
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:10663:   135	// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:10671:   143	pub struct EconomicState {
handover/audits/CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md:11161:   323	    pub economic_state_t: EconomicState,
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:1049:    pub economic_state_t: EconomicState,
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:1066:pub struct EconomicState {
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:5574:/bin/bash -lc 'rg -n "struct QState|ledger_root_t|state_root_t|EconomicState|BalancesIndex|ClaimsIndex|impl .*Index|type .*Index|BTreeMap" src/state/q_state.rs' in /home/zephryj/projects/turingosv4
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:5576:8://! TRACE_MATRIX WP § 2 economic — `EconomicState` 9 sub-fields (CO1.2.2).
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:5582:135:// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:5584:143:pub struct EconomicState {
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:5599:323:    pub economic_state_t: EconomicState,
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:5604:385:        let e = EconomicState::default();
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:5605:391:            "EconomicState must have 9 sub-fields per WP § 2; got {}",
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:5734:     8	//! TRACE_MATRIX WP § 2 economic — `EconomicState` 9 sub-fields (CO1.2.2).
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:5861:   135	// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:5869:   143	pub struct EconomicState {
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:6049:   323	    pub economic_state_t: EconomicState,
handover/audits/CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md:6812:    29	fn economic_state_reconstruct() { unimplemented!("CO1.2.4 — replay; assert byte-identical EconomicState 9 sub-fields") }
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:2334:    pub economic_state_t: EconomicState,
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:2351:pub struct EconomicState {
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:6859:/bin/bash -lc 'rg -n "struct QState|ledger_root_t|state_root_t|EconomicState|BalancesIndex|ClaimsIndex|impl .*Index|type .*Index|BTreeMap" src/state/q_state.rs' in /home/zephryj/projects/turingosv4
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:6861:8://! TRACE_MATRIX WP § 2 economic — `EconomicState` 9 sub-fields (CO1.2.2).
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:6867:135:// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:6869:143:pub struct EconomicState {
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:6884:323:    pub economic_state_t: EconomicState,
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:6889:385:        let e = EconomicState::default();
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:6890:391:            "EconomicState must have 9 sub-fields per WP § 2; got {}",
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:7019:     8	//! TRACE_MATRIX WP § 2 economic — `EconomicState` 9 sub-fields (CO1.2.2).
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:7146:   135	// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:7154:   143	pub struct EconomicState {
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:7334:   323	    pub economic_state_t: EconomicState,
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:8097:    29	fn economic_state_reconstruct() { unimplemented!("CO1.2.4 — replay; assert byte-identical EconomicState 9 sub-fields") }
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:8676:    pub economic_state_t: EconomicState,
handover/audits/CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md:8693:pub struct EconomicState {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:4580:    pub economic_state_t: EconomicState,
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:4597:pub struct EconomicState {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:9105:/bin/bash -lc 'rg -n "struct QState|ledger_root_t|state_root_t|EconomicState|BalancesIndex|ClaimsIndex|impl .*Index|type .*Index|BTreeMap" src/state/q_state.rs' in /home/zephryj/projects/turingosv4
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:9107:8://! TRACE_MATRIX WP § 2 economic — `EconomicState` 9 sub-fields (CO1.2.2).
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:9113:135:// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:9115:143:pub struct EconomicState {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:9130:323:    pub economic_state_t: EconomicState,
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:9135:385:        let e = EconomicState::default();
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:9136:391:            "EconomicState must have 9 sub-fields per WP § 2; got {}",
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:9265:     8	//! TRACE_MATRIX WP § 2 economic — `EconomicState` 9 sub-fields (CO1.2.2).
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:9392:   135	// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:9400:   143	pub struct EconomicState {
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:9580:   323	    pub economic_state_t: EconomicState,
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:10343:    29	fn economic_state_reconstruct() { unimplemented!("CO1.2.4 — replay; assert byte-identical EconomicState 9 sub-fields") }
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:10922:    pub economic_state_t: EconomicState,
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:10939:pub struct EconomicState {
handover/audits/CODEX_CO1_8_ROUND1_AUDIT_2026-04-29.md:533://! TRACE_MATRIX WP § 2 economic — `EconomicState` 9 sub-fields (CO1.2.2).
handover/audits/CODEX_CO1_8_ROUND1_AUDIT_2026-04-29.md:660:// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
handover/audits/CODEX_CO1_8_ROUND1_AUDIT_2026-04-29.md:668:pub struct EconomicState {
handover/audits/CODEX_CO1_8_ROUND1_AUDIT_2026-04-29.md:848:    pub economic_state_t: EconomicState,
handover/audits/CODEX_CO1_8_ROUND1_AUDIT_2026-04-29.md:910:        let e = EconomicState::default();
handover/audits/CODEX_CO1_8_ROUND1_AUDIT_2026-04-29.md:916:            "EconomicState must have 9 sub-fields per WP § 2; got {}",
handover/audits/CODEX_CO1_8_ROUND1_AUDIT_2026-04-29.md:4745:    45	| `q_next.economic_state_t.derive_state_root()` | Method does not exist on EconomicState | No state-root derivation |
handover/audits/CODEX_TB_12_SHIP_AUDIT_2026-05-03.md:8:Evidence: `EconomicState` adds `node_positions_t` as a serde-defaulted flat index, with comments saying it is not a Coin holding and is not counted in `total_supply_micro` (`src/state/q_state.rs:176`, `src/state/q_state.rs:179`, `src/state/q_state.rs:191`). The monetary sum in `total_supply_micro` iterates balances, escrows, stakes, and challenge cases only (`src/economy/monetary_invariant.rs:154`, `src/economy/monetary_invariant.rs:157`, `src/economy/monetary_invariant.rs:160`, `src/economy/monetary_invariant.rs:169`), and there is no `node_positions_t` term in that function (`src/economy/monetary_invariant.rs:152`, `src/economy/monetary_invariant.rs:172`).
handover/audits/CODEX_TB_12_SHIP_AUDIT_2026-05-03.md:40:Challenge: the required forbidden-token grep over current `src/` is not clean. `src/lib.rs` exports `prediction_market` (`src/lib.rs:3`). That module is an actual CPMM trading implementation: it declares a CPMM binary market (`src/prediction_market.rs:1`, `src/prediction_market.rs:10`), implements `buy_yes` and `buy_no` (`src/prediction_market.rs:81`, `src/prediction_market.rs:113`), and implements redemption (`src/prediction_market.rs:145`, `src/prediction_market.rs:148`). `Kernel` still stores `BinaryMarket` maps and a bounty market (`src/kernel.rs:21`, `src/kernel.rs:27`), with a comment referencing a CPMM book (`src/kernel.rs:29`). Inference: this appears to be legacy non-TB-12 code rather than a NodePosition or EconomicState path, but the strict `src/` forbidden-token gate cannot be reported as PASS without an explicit allowlist, quarantine, or removal.
handover/audits/CODEX_TB_12_SHIP_AUDIT_2026-05-03.md:49:## Q7 No node_market_t canonical EconomicState field?
handover/audits/CODEX_TB_12_SHIP_AUDIT_2026-05-03.md:52:Evidence: `EconomicState` lists 11 fields ending with serde-defaulted `runs_t` and `node_positions_t`, with no `node_market_t` field (`src/state/q_state.rs:158`, `src/state/q_state.rs:192`). `NodePositionsIndex` is a flat `BTreeMap<TxId, NodePosition>` (`src/state/q_state.rs:485`, `src/state/q_state.rs:492`). The local grep for `node_market_t` in `src/` found only comments and the guard test, not a struct field; the runtime test asserts the serialized EconomicState object does not contain `node_market_t` (`src/state/q_state.rs:726`, `src/state/q_state.rs:739`).
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R1.md:1415:    21	**Flowchart-trace**: **Flowchart 1** (runtime — `CompleteSetMintTx` debits `balances_t`, credits `conditional_collateral_t`, increments equal `conditional_share_balances_t[(owner, event, Yes/No)]`; `CompleteSetRedeemTx` requires system-resolution-reference, debits `conditional_share_balances_t`, credits `balances_t`; `MarketSeedTx` debits `balances_t` of provider, credits `conditional_collateral_t` + records protocol-owned share inventory) + **Flowchart 2** (boot — runtime preseed unchanged; EconomicState 11→13 sub-fields with backward-compat `#[serde(default)]` empty maps) + **Flowchart 3** (meta — conditional shares are TB-14 PriceIndex price-derivation substrate + TB-15 MarkovEvidenceCapsule compression input; TB-13 lays bytes only; no price, no AMM, no orderbook).
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R1.md:1434:    40	            BinaryMarket f64 CPMM book; EconomicState +2 sub-fields)
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R1.md:1475:    81	  Flowchart 2 (boot):    EconomicState additive +conditional_collateral_t
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R1.md:1515:   121	      EconomicState sub-fields (conditional_collateral_t,

 succeeded in 1372ms:
handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff:1150:-        assert_eq!(r.oracle_nonce(), 0x1234);
handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/amendment.diff:825:+        assert_eq!(r.oracle_nonce(), 0x1234);
handover/ai-direct/LATEST.md:62:- **MOD** `src/state/q_state.rs`: `+ AutopsyIndex(BTreeMap<EventId, Vec<Cid>>)` + `agent_autopsies_t` 13th sub-field on EconomicState. Sub-field count 12→13.
handover/ai-direct/LATEST.md:66:- **MOD** 4 test fixtures for sub-field count 12→13 + 4 fc_alignment_conformance witnesses.
handover/ai-direct/LATEST.md:323:          stub) + `EconomicState.price_index_t` field (13 → 12 sub-fields per
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:76:- G-15.11 (EconomicState sub-field count 12→13) — **GREEN** 3 tests updated.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:90:- `src/state/q_state.rs`: `+ AutopsyIndex(BTreeMap<EventId, Vec<Cid>>)` newtype + `agent_autopsies_t: AutopsyIndex` 13th sub-field on EconomicState (serde-default). Sub-field count assertion 12→13.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:96:- `tests/economic_state_reconstruct.rs`: sub-field count 12→13.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:97:- `tests/q_state_reconstruct.rs`: sub-field count 12→13.
handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:98:- `tests/six_axioms_alignment.rs`: axiom_3 sub-field count 12→13.
handover/alignment/OBS_R022_TB14_PRICEINDEX_REMOVED_2026-05-03.md:18:Net effect: the canonical state has 12 sub-fields (was 13 pre-TB-14); the price-derivation role moves from a never-populated stub field to a pure derived view; flowchart anchoring strengthens (FC3-N42 typed and witnessed) rather than weakens.
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:1494:        assert_eq!(m.to_string(), "12.345678 coin");
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:10102:/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.149/tests/test.rs:1972:    assert_eq!(from_str::<BTreeMap<u128, ()>>(j).unwrap(), map);
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:10148:/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.149/tests/lexical/math.rs:137:    assert_eq!(x.data, from_u32(&[12]));
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26.md:1310:        assert_eq!(FcId::Fc1N12.as_str(), "FC1-N12");
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26.md:3641:                assert_eq!(agg.total_wall_time_ms, 120_000);
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26.md:6750:   238	        assert_eq!(FcId::Fc1N12.as_str(), "FC1-N12");
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26.md:9524:experiments/minif2f_v4/src/fc_trace.rs:238:        assert_eq!(FcId::Fc1N12.as_str(), "FC1-N12");
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R10.md:1588:        assert_eq!(FcId::Fc1N12.as_str(), "FC1-N12");
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R10.md:4048:                assert_eq!(agg.total_wall_time_ms, 120_000);
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R11.md:1611:        assert_eq!(FcId::Fc1N12.as_str(), "FC1-N12");
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R11.md:4071:                assert_eq!(agg.total_wall_time_ms, 120_000);
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R12.md:1661:        assert_eq!(FcId::Fc1N12.as_str(), "FC1-N12");
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R12.md:4117:                assert_eq!(agg.total_wall_time_ms, 120_000);
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R13.md:1697:        assert_eq!(FcId::Fc1N12.as_str(), "FC1-N12");
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R13.md:4163:                assert_eq!(agg.total_wall_time_ms, 120_000);
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R2.md:1349:        assert_eq!(FcId::Fc1N12.as_str(), "FC1-N12");
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R2.md:3809:                assert_eq!(agg.total_wall_time_ms, 120_000);
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R3.md:1372:        assert_eq!(FcId::Fc1N12.as_str(), "FC1-N12");
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R3.md:3832:                assert_eq!(agg.total_wall_time_ms, 120_000);
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R4.md:1398:        assert_eq!(FcId::Fc1N12.as_str(), "FC1-N12");
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R4.md:3858:                assert_eq!(agg.total_wall_time_ms, 120_000);
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R5.md:1398:        assert_eq!(FcId::Fc1N12.as_str(), "FC1-N12");
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R5.md:3858:                assert_eq!(agg.total_wall_time_ms, 120_000);
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R6.md:1433:        assert_eq!(FcId::Fc1N12.as_str(), "FC1-N12");
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R6.md:3893:                assert_eq!(agg.total_wall_time_ms, 120_000);
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R7.md:1516:        assert_eq!(FcId::Fc1N12.as_str(), "FC1-N12");
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R7.md:3976:                assert_eq!(agg.total_wall_time_ms, 120_000);
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R8.md:1539:        assert_eq!(FcId::Fc1N12.as_str(), "FC1-N12");
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R8.md:3999:                assert_eq!(agg.total_wall_time_ms, 120_000);
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R9.md:1565:        assert_eq!(FcId::Fc1N12.as_str(), "FC1-N12");
handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R9.md:4025:                assert_eq!(agg.total_wall_time_ms, 120_000);
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R1.md:3216:src/state/typed_tx.rs:3194:        assert_eq!(ShareAmount::default().units, 0u128);
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R1.md:6713:   226	    assert_eq!(pair.yes.units, 5_000_000_u128, "YES shares minted equal to amount");
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R1.md:6714:   227	    assert_eq!(pair.no.units, 5_000_000_u128, "NO shares minted equal to amount");
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R1.md:6894:   406	    assert_eq!(pair.yes.units, 0_u128, "YES shares debited (winning side)");
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R1.md:6895:   407	    assert_eq!(pair.no.units, 4_000_000_u128, "NO shares preserved (losing side)");
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R1.md:7087:   599	    assert_eq!(pair.yes.units, 3_141_592_u128);
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R1.md:7088:   600	    assert_eq!(pair.no.units, 3_141_592_u128);
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R2.md:1708:src/state/typed_tx.rs:3194:        assert_eq!(ShareAmount::default().units, 0u128);
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R2.md:3423:   226	    assert_eq!(pair.yes.units, 5_000_000_u128, "YES shares minted equal to amount");
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R2.md:3424:   227	    assert_eq!(pair.no.units, 5_000_000_u128, "NO shares minted equal to amount");
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R2.md:3603:   406	    assert_eq!(pair.yes.units, 0_u128, "YES shares debited (winning side)");
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R2.md:3604:   407	    assert_eq!(pair.no.units, 4_000_000_u128, "NO shares preserved (losing side)");
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R2.md:3891:   694	    assert_eq!(pair.yes.units, 3_141_592_u128);
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R2.md:3892:   695	    assert_eq!(pair.no.units, 3_141_592_u128);
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R3.md:3784:   295	    assert_eq!(pair.yes.units, 5_000_000_u128, "YES shares minted equal to amount");
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R3.md:3785:   296	    assert_eq!(pair.no.units, 5_000_000_u128, "NO shares minted equal to amount");
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R3.md:3975:   486	    assert_eq!(pair.yes.units, 0_u128, "YES shares debited (winning side)");
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R3.md:3976:   487	    assert_eq!(pair.no.units, 4_000_000_u128, "NO shares preserved (losing side)");
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R3.md:4783:   868	    assert_eq!(pair.yes.units, 3_141_592_u128);
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R3.md:4784:   869	    assert_eq!(pair.no.units, 3_141_592_u128);
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R4.md:5285:  3190	        assert_eq!(ShareAmount::default().units, 0u128);
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R4.md:6311:   291	    assert_eq!(pair.yes.units, 5_000_000_u128, "YES shares minted equal to amount");
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R4.md:6312:   292	    assert_eq!(pair.no.units, 5_000_000_u128, "NO shares minted equal to amount");
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R4.md:6502:   482	    assert_eq!(pair.yes.units, 0_u128, "YES shares debited (winning side)");
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R4.md:6503:   483	    assert_eq!(pair.no.units, 4_000_000_u128, "NO shares preserved (losing side)");
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R4.md:11111:   864	    assert_eq!(pair.yes.units, 3_141_592_u128);
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R4.md:11112:   865	    assert_eq!(pair.no.units, 3_141_592_u128);
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R5.md:7282:   864	    assert_eq!(pair.yes.units, 3_141_592_u128);
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R5.md:7283:   865	    assert_eq!(pair.no.units, 3_141_592_u128);
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R5.md:7720:   482	    assert_eq!(pair.yes.units, 0_u128, "YES shares debited (winning side)");
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R5.md:7721:   483	    assert_eq!(pair.no.units, 4_000_000_u128, "NO shares preserved (losing side)");
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R6.md:1952:src/state/typed_tx.rs:3190:        assert_eq!(ShareAmount::default().units, 0u128);
handover/audits/CODEX_TB_14_SHIP_AUDIT_2026-05-03_R1.md:2105:   706	        assert_eq!(entry.yes_share_depth, ShareAmount::from_units(120_000));
handover/audits/CODEX_TB_14_SHIP_AUDIT_2026-05-03_R1.md:5646:src/state/q_state.rs:824:            "EconomicState must have 12 sub-fields post-TB-14 (was 13 post-TB-13; -price_index_t legacy stub); got {}",
handover/evidence/tb_15_markov_capsule_2026-05-03/README.md:80:| G-15.11 | EconomicState sub-field count assertion updated 12→13 | GREEN — 3 sub-field count tests updated (`economic_state_has_thirteen_sub_fields`, `empty_economic_state_serializes_to_thirteen_sub_fields`, `axiom_3_economic_state_present_and_complete` 13) |
handover/tracer_bullets/TB-15_charter_2026-05-03.md:256:| `EconomicState` 12 sub-fields (TB-15 will add 13th: `agent_autopsies_t`) | `src/state/q_state.rs:170-239` | TB-14 |
handover/tracer_bullets/TB-15_charter_2026-05-03.md:298:**MOD**: `src/state/q_state.rs` — NEW `pub struct AutopsyIndex(pub BTreeMap<EventId, Vec<Cid>>)` + `pub agent_autopsies_t: AutopsyIndex` field on `EconomicState` (13th sub-field; `#[serde(default)]` for backward-compat). Update sub-field-count assertion 12→13.
handover/tracer_bullets/TB-15_charter_2026-05-03.md:409:| G-15.11 | EconomicState sub-field count assertion updated 12→13 | `q_state.rs` test |
handover/tracer_bullets/TB_LOG.tsv:45:# 2026-05-03 TB-15 SHIPPED — "P4 Information Loom (primary) + P5 MetaTape v0 prep: Lamarckian Autopsy + Markov EvidenceCapsule per architect 2026-05-02 + 2026-05-03 §6 verbatim spec (FR-15.1..6 + CR-15.1..6 + SG-15.1..8 + 6 halt triggers + forbidden list)". Class 2 envelope (self-audit; AgentVisibleProjection unchanged; only one new sequencer dispatch hook — TaskBankruptcyTx Step 3.5 + apply_one Stage 3.5). 7-atom plan (charter + halt fixture + AgentAutopsyCapsule schema/writer + AutopsyIndex/TaskBankruptcyTx wire-in + cluster_autopsies + MarkovEvidenceCapsule schema/generator + dashboard §15/first capsule/SHIP) all green. Auto-resolutions per feedback_architect_deviation_stance: A) single trigger site = TaskBankruptcyTx (SlashTx wires when RSP-3.2 ships; FinalizeReward loser branch wires when RSP-4 ships); B) EconomicState 12→13 sub-fields with +agent_autopsies_t serde-default; C) privacy default = AuditOnly (re-use TB-11 CapsulePrivacyPolicy); D) clustering threshold N=3 per architect §3.2.3 verbatim; E) LossReasonClass 8 variants incl Other(String); F) Markov override = env TURINGOS_MARKOV_OVERRIDE=1; G) OBS_RESOLUTIONS_INDEX_TB15 DEFERRED out of TB-15 (TB-13 audit residue, not in architect §6 spec); H) Class 2 self-audit (no AgentVisibleProjection mod). cargo test --workspace = 870/0/150 PASS (+67 net vs TB-14 ship 803). All 6 halt-triggers GREEN: #1 raw_logs_not_in_general_read_view (Atom 3 file-scan AgentVisibleProjection) + #2 markov_capsule_references_constitution_hash (Atom 5 SG-15.7) + #3 autopsy_does_not_mutate_predicates (Atom 2 file-scan no &mut PredicateRegistry/ToolRegistry/RiskPolicyRegistry/register_*/unregister_*/patch_*; CR-15.3 + SG-15.8) + #4 private_detail_not_in_other_agent_view (Atom 3 file-scan AutopsyIndex value type Vec<Cid> not raw bytes; SG-15.2) + #5 typical_error_clustering_uses_summary_only (Atom 4 cluster_autopsies serialization scan no private_detail_cid byte run; CR-15.2 + SG-15.5) + #6 deep_history_read_without_override_fails (Atom 5 try_deep_history_read_with_override_check default-deny; SG-15.4 + FR-15.5). Architectural deltas: NEW src/runtime/autopsy_capsule.rs (LossReasonClass enum + AgentAutopsyCapsule struct + format_public_summary + write_autopsy_capsule + derive_autopsies_for_bankruptcy PURE deterministic helper + write_bankruptcy_autopsies_to_cas + cluster_autopsies + TypicalErrorSummary + AutopsyWriteError + 15 unit tests); NEW src/runtime/markov_capsule.rs (ObsId newtype + MarkovEvidenceCapsule struct + with_constitution_hash + try_deep_history_read_with_override_check + override_set_from_env + write_markov_capsule + scan_unresolved_obs + sha256_of_file + MarkovGenError + 8 unit tests); NEW src/bin/generate_markov_capsule.rs (CLI binary with --tb-id/--out-dir/--constitution-path/--cas-dir/--prev-cid-hex/--alignment-dir/--no-cas args + TURINGOS_MARKOV_OVERRIDE env); NEW tests/tb_15_halt_triggers.rs (6 fixtures); MOD src/state/typed_tx.rs (+RiskRuleId opaque newtype); MOD src/bottom_white/cas/schema.rs (+4 ObjectType variants AgentAutopsyCapsule + AutopsyPrivateDetail + MarkovEvidenceCapsule + NextSessionContext); MOD src/state/q_state.rs (+AutopsyIndex(BTreeMap<EventId, Vec<Cid>>) newtype + agent_autopsies_t 13th sub-field on EconomicState serde-default; sub-field count assertion 12→13); MOD src/state/sequencer.rs (TaskBankruptcyTx dispatch arm Step 3.5 PURE deterministic Cid derivation + apply_one Stage 3.5 CAS write side-effect via deterministic helper; NO predicate registry mutation; NO AgentVisibleProjection mod; NO change to monetary invariants); MOD src/runtime/mod.rs (+pub mod autopsy_capsule + pub mod markov_capsule); MOD src/bin/audit_dashboard.rs (+render_section_15 pure render with AUTOPSY IS PRIVATE banner + autopsy_event_counts + latest_markov_capsule_cid_hex fields on DashboardReport + read_latest_markov_pointer helper + 4 SG-15.6 dashboard render tests); MOD 4 test fixtures sub-field count 12→13 (economic_state_reconstruct/q_state_reconstruct/six_axioms_alignment); MOD tests/fc_alignment_conformance.rs (+4 TB-15 witnesses FC1-N32 write_autopsy_capsule + FC1-N33 derive_autopsies_for_bankruptcy + FC2-N30 cluster_autopsies + FC3-N43 MarkovEvidenceCapsule); REHASH genesis_payload.toml trust_root for 6 modified files (src/runtime/mod.rs ×2 hops Atom2→Atom5; src/state/q_state.rs Atom3; src/state/typed_tx.rs Atom2; src/state/sequencer.rs Atom3; src/bottom_white/cas/schema.rs Atom2; tests/fc_alignment_conformance.rs Atom6). First Markov capsule emitted (capsule_id b244f16a1f3bd532d041a40fe39b2b7e7cc12fb58e18b61aedd76a8010eeb1b6) with constitution_hash eec695459c71fbef3685583485deb431fe3b561657b2f285b7c5e7e220e59e03; evidence at handover/evidence/tb_15_markov_capsule_2026-05-03/. Production claim: "TB-15 establishes Lamarckian Autopsy + Markov EvidenceCapsule substrate. AgentAutopsyCapsule (per-agent, per-event, AuditOnly) records loss/bankruptcy events derived deterministically from ChainTape evidence — NEVER LLM self-narration. agent_autopsies_t lives sequencer-side (NOT projected to AgentVisibleProjection per CR-15.1 + halt-trigger #1). TypicalErrorBroadcast clustering at N≥3 emits public_summary text + Cids only — NEVER private_detail_cid bytes. MarkovEvidenceCapsule binds constitution_hash + L4 + L4.E + CAS roots + previous capsule + typical_errors + unresolved_obs as next-session bootstrap default; deeper history requires TURINGOS_MARKOV_OVERRIDE=1. CR-15.3/15.4 STRUCTURALLY ENFORCED via writer signature + halt-trigger #3 file-scan. Constitutional alignment: Art.II.1 + Art.III.1 + Art.III.2 + Art.III.3 + Art.III.4 + Art.V.1 + Art.0.2." Open follow-ups (carry-forward NOT ship blockers): multi-site autopsy wire-in (SlashLoss/ChallengeUnsuccessful/VerifierBondLost — when RSP-3.2/RSP-4 ship); L4/L4.E/CAS root chain-readers in Markov generator (currently zero placeholders); CAS-walking dashboard §15 (currently empty autopsy_event_counts); InitAI agent-side honoring of Markov default (P5 v1); OBS_RESOLUTIONS_INDEX_TB15 explicitly deferred per charter §7-G.
src/economy/money.rs:275:        assert_eq!(m.to_string(), "12.345678 coin");
src/runtime/bootstrap.rs:80:        assert_eq!(pairs.len(), 12, "expected 12 preseed entries");
src/runtime/genesis_report.rs:156:        assert_eq!(round.constitution_hash, Some("abc123".into()));
src/state/price_index.rs:811:        assert_eq!(entry.yes_share_depth, ShareAmount::from_units(120_000));
src/state/q_state.rs:846:        // TB-14 Atom 2 (2026-05-03 architect ruling §5.1): 13 → 12 sub-fields
src/state/typed_tx.rs:3201:        assert_eq!(ShareAmount::default().units, 0u128);
tests/tb_12_node_exposure_index.rs:259:    assert_eq!(short.node_id, work_tx_id, "FR-12.5 node_id == challenge.target_work_tx");
tests/tb_13_complete_set.rs:291:    assert_eq!(pair.yes.units, 5_000_000_u128, "YES shares minted equal to amount");
tests/tb_13_complete_set.rs:292:    assert_eq!(pair.no.units, 5_000_000_u128, "NO shares minted equal to amount");
tests/tb_13_complete_set.rs:482:    assert_eq!(pair.yes.units, 0_u128, "YES shares debited (winning side)");
tests/tb_13_complete_set.rs:483:    assert_eq!(pair.no.units, 4_000_000_u128, "NO shares preserved (losing side)");
tests/tb_13_complete_set.rs:864:    assert_eq!(pair.yes.units, 3_141_592_u128);
tests/tb_13_complete_set.rs:865:    assert_eq!(pair.no.units, 3_141_592_u128);

exec
/bin/bash -lc "git grep -n \"12\" -- src tests | grep -E \"EconomicState|sub-field|sub_fields|obj\\.len|assert_eq\" | sed -n '1,160p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/economy/money.rs:275:        assert_eq!(m.to_string(), "12.345678 coin");
src/runtime/bootstrap.rs:80:        assert_eq!(pairs.len(), 12, "expected 12 preseed entries");
src/runtime/genesis_report.rs:156:        assert_eq!(round.constitution_hash, Some("abc123".into()));
src/state/price_index.rs:811:        assert_eq!(entry.yes_share_depth, ShareAmount::from_units(120_000));
src/state/q_state.rs:838:        // TB-12 (2026-05-03 architect ruling §3 + §8 Atom 1): 10 → 11 sub-fields
src/state/q_state.rs:846:        // TB-14 Atom 2 (2026-05-03 architect ruling §5.1): 13 → 12 sub-fields
src/state/q_state.rs:851:        // TB-15 Atom 3 (2026-05-03 architect ruling §6.2): 12 → 13 sub-fields
src/state/q_state.rs:861:            "EconomicState must have 13 sub-fields post-TB-15 (was 12 post-TB-14; +agent_autopsies_t); got {}",
src/state/q_state.rs:865:        assert!(obj.contains_key("node_positions_t"), "TB-12 node_positions_t sub-field missing");
src/state/typed_tx.rs:3201:        assert_eq!(ShareAmount::default().units, 0u128);
tests/tb_12_node_exposure_index.rs:259:    assert_eq!(short.node_id, work_tx_id, "FR-12.5 node_id == challenge.target_work_tx");
tests/tb_12_node_exposure_index.rs:509:/// SG-12.8 (architect 2026-05-03 ruling §3 + §9.3): the EconomicState MUST
tests/tb_12_node_exposure_index.rs:523:    // Positive: node_positions_t IS the canonical TB-12 sub-field.
tests/tb_13_complete_set.rs:291:    assert_eq!(pair.yes.units, 5_000_000_u128, "YES shares minted equal to amount");
tests/tb_13_complete_set.rs:292:    assert_eq!(pair.no.units, 5_000_000_u128, "NO shares minted equal to amount");
tests/tb_13_complete_set.rs:482:    assert_eq!(pair.yes.units, 0_u128, "YES shares debited (winning side)");
tests/tb_13_complete_set.rs:483:    assert_eq!(pair.no.units, 4_000_000_u128, "NO shares preserved (losing side)");
tests/tb_13_complete_set.rs:864:    assert_eq!(pair.yes.units, 3_141_592_u128);
tests/tb_13_complete_set.rs:865:    assert_eq!(pair.no.units, 3_141_592_u128);

exec
/bin/bash -lc "nl -ba tests/economic_state_reconstruct.rs | sed -n '1,90p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TRACE_MATRIX WP § 2 economic — EconomicState 9 sub-fields reconstructibility.
     2	//!
     3	//! Atom CO1.2.2: each sub-index round-trips and is insertion-order independent.
     4	
     5	use turingosv4::economy::money::MicroCoin;
     6	use turingosv4::state::{
     7	    AgentId, BalancesIndex, ChallengeCase, ChallengeCasesIndex, ClaimEntry, ClaimsIndex,
     8	    EconomicState, EscrowEntry, EscrowsIndex, Reputation, ReputationsIndex,
     9	    RoyaltyEdge, RoyaltyGraph, StakeEntry, StakesIndex, TaskId, TaskMarketEntry, TaskMarketsIndex,
    10	    TxId,
    11	};
    12	
    13	#[test]
    14	fn thirteen_sub_fields_present() {
    15	    // TB-12: was ten (TB-11 +runs_t); +node_positions_t (architect
    16	    // 2026-05-03 §3). TB-13 Atom 2 (architect 2026-05-03 post-TB-12
    17	    // ruling Part A §4.3): 11 → 13 (+conditional_collateral_t Coin
    18	    // holding + conditional_share_balances_t claims).
    19	    // TB-14 Atom 2 (2026-05-03; architect §5.1): 13 → 12 (-price_index_t;
    20	    // TB-14 derives the price view via `compute_price_index` pure fn,
    21	    // not stored as canonical state — "price is signal, not truth";
    22	    // charter §7 auto-resolution A: no second source-of-truth).
    23	    // TB-15 Atom 3 (2026-05-03; architect §6.2 ruling): 12 → 13
    24	    // (+agent_autopsies_t — `AutopsyIndex` = `BTreeMap<EventId, Vec<Cid>>`;
    25	    // sequencer-side per-event Cid index for AgentAutopsyCapsule emission;
    26	    // capsule bytes live in CAS; NOT projected to AgentVisibleProjection
    27	    // per CR-15.1 + halt-trigger #1).
    28	    let e = EconomicState::default();
    29	    let v = serde_json::to_value(&e).unwrap();
    30	    let obj = v.as_object().unwrap();
    31	    let names = [
    32	        "balances_t",
    33	        "escrows_t",
    34	        "stakes_t",
    35	        "claims_t",
    36	        "reputations_t",
    37	        "task_markets_t",
    38	        "royalty_graph_t",
    39	        "challenge_cases_t",
    40	        "runs_t",                          // TB-11 (architect §6.2 ruling 2026-05-02)
    41	        "node_positions_t",                // TB-12 (architect 2026-05-03 ruling §3 + §8 Atom 1)
    42	        "conditional_collateral_t",        // TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling §4.3)
    43	        "conditional_share_balances_t",    // TB-13 Atom 2
    44	        "agent_autopsies_t",               // TB-15 Atom 3 (architect §6.2 ruling 2026-05-02 + 2026-05-03)
    45	    ];
    46	    assert_eq!(obj.len(), 13);
    47	    for n in names.iter() {
    48	        assert!(obj.contains_key(*n), "missing sub-field {}", n);
    49	    }
    50	}
    51	
    52	#[test]
    53	fn populated_economic_state_round_trip() {
    54	    let mut e = EconomicState::default();
    55	    e.balances_t.0.insert(AgentId("a".into()), MicroCoin::from_coin(10).unwrap());
    56	    e.escrows_t.0.insert(
    57	        TxId("t1".into()),
    58	        EscrowEntry {
    59	            amount: MicroCoin::from_coin(5).unwrap(),
    60	            depositor: AgentId("a".into()),
    61	            task_id: TaskId("t4".into()),
    62	        },
    63	    );
    64	    e.stakes_t.0.insert(
    65	        TxId("t2".into()),
    66	        StakeEntry {
    67	            amount: MicroCoin::from_coin(3).unwrap(),
    68	            staker: AgentId("b".into()),
    69	            task_id: TaskId("t4".into()),
    70	        },
    71	    );
    72	    e.claims_t.0.insert(
    73	        TxId("t3".into()),
    74	        ClaimEntry {
    75	            amount: MicroCoin::from_coin(7).unwrap(),
    76	            claimant: AgentId("c".into()),
    77	            ..Default::default()
    78	        },
    79	    );
    80	    e.reputations_t.0.insert(AgentId("a".into()), Reputation(100));
    81	    // **TB-3 fixture migration**: TaskMarketEntry no longer has `bounty`;
    82	    // money has migrated to `escrows_t.amount`. `total_escrow` is the derived
    83	    // cache (matches the escrow above for round-trip determinism).
    84	    let mut market = TaskMarketEntry::default();
    85	    market.publisher = AgentId("p".into());
    86	    market.total_escrow = MicroCoin::from_coin(5).unwrap();
    87	    market.escrow_lock_tx_ids.insert(TxId("t1".into()));
    88	    market.verifier_quorum = 1;
    89	    market.max_reuse_royalty_fraction_basis_points = 1000;
    90	    e.task_markets_t.0.insert(TaskId("t4".into()), market);

exec
/bin/bash -lc "nl -ba tests/q_state_reconstruct.rs | sed -n '75,115p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    75	        "economic_state_t",
    76	        "budget_state_t",
    77	    ];
    78	    assert_eq!(obj.len(), 9, "WP § 4 mandates exactly 9 fields");
    79	    for k in expected.iter() {
    80	        assert!(obj.contains_key(*k), "missing field {}", k);
    81	    }
    82	}
    83	
    84	#[test]
    85	fn empty_economic_state_serializes_to_thirteen_sub_fields() {
    86	    // TB-11 (architect §6.2 ruling 2026-05-02): 9 → 10 (+runs_t).
    87	    // TB-12 (architect 2026-05-03 ruling §3 + §8 Atom 1): 10 → 11 (+node_positions_t).
    88	    // TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling §4.3):
    89	    //   11 → 13 (+conditional_collateral_t +conditional_share_balances_t).
    90	    // TB-14 Atom 2 (2026-05-03; architect §5.1 + charter §7 auto-resolution A):
    91	    //   13 → 12 (-price_index_t legacy stub; TB-14 derives the price view via
    92	    //   `compute_price_index` pure fn, not stored as canonical state — "price
    93	    //   is signal, not truth"; no second source-of-truth).
    94	    // TB-15 Atom 3 (2026-05-03; architect §6.2 ruling): 12 → 13
    95	    //   (+agent_autopsies_t — `AutopsyIndex` per-event Cid index for
    96	    //   AgentAutopsyCapsule emission; sequencer-side; CR-15.1 + halt-trigger #1
    97	    //   exclude from AgentVisibleProjection).
    98	    let e = EconomicState::default();
    99	    let v = serde_json::to_value(&e).unwrap();
   100	    let obj = v.as_object().unwrap();
   101	    assert_eq!(obj.len(), 13);
   102	}

exec
/bin/bash -lc "nl -ba tests/six_axioms_alignment.rs | sed -n '55,80p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    55	/// 11 → 13 with +conditional_collateral_t (Coin holding per CR-13.4) +
    56	/// conditional_share_balances_t (claims per CR-13.3).
    57	/// TB-14 Atom 2 (2026-05-03; architect §5.1) trimmed 13 → 12 by removing the
    58	/// legacy `price_index_t` stub; TB-14 provides `compute_price_index`
    59	/// pure-fn derived view (charter §7 auto-resolution A: no second
    60	/// source-of-truth).
    61	/// TB-15 Atom 3 (2026-05-03; architect §6.2 ruling) bumped 12 → 13 with
    62	/// +agent_autopsies_t (`AutopsyIndex` = `BTreeMap<EventId, Vec<Cid>>`;
    63	/// sequencer-side per-event Cid index for AgentAutopsyCapsule emission;
    64	/// capsule bytes live in CAS; CR-15.1 + halt-trigger #1 exclude from
    65	/// AgentVisibleProjection).
    66	#[test]
    67	fn axiom_3_economic_state_present_and_complete() {
    68	    let g = QState::genesis();
    69	    let e = serde_json::to_value(&g.economic_state_t).unwrap();
    70	    assert_eq!(e.as_object().unwrap().len(), 13);
    71	}
    72	
    73	/// Axiom 4: tool capability — Q_t exposes `tool_registry_root_t` slot.
    74	#[test]
    75	fn axiom_4_tool_registry_root_present() {
    76	    let g = QState::genesis();
    77	    let v = serde_json::to_value(&g).unwrap();
    78	    assert!(v.as_object().unwrap().contains_key("tool_registry_root_t"));
    79	}
    80	

exec
/bin/bash -lc "nl -ba tests/fc_alignment_conformance.rs | sed -n '600,675p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   600	            task_id: TaskId("witness:bk".into()),
   601	        },
   602	    );
   603	    let bk = TaskBankruptcyTx {
   604	        task_id: TaskId("witness:bk".into()),
   605	        timestamp_logical: 5,
   606	        ..Default::default()
   607	    };
   608	    let a = derive_autopsies_for_bankruptcy(&econ, &bk, 1, 5);
   609	    let b = derive_autopsies_for_bankruptcy(&econ, &bk, 1, 5);
   610	    assert_eq!(a.len(), 1);
   611	    assert_eq!(a[0].0.capsule_id, b[0].0.capsule_id, "FC1-N33: deterministic Cid");
   612	}
   613	
   614	/// FC2-N30 (TB-15 Atom 4): cluster_autopsies pure aggregator. Witness:
   615	/// 3 same-class autopsies → 1 TypicalErrorSummary (architect §3.2.3
   616	/// threshold). Output uses public_summary text + capsule_id Cids only.
   617	#[test]
   618	fn fc2_n30_cluster_autopsies_witness() {
   619	    use turingosv4::bottom_white::cas::schema::Cid;
   620	    use turingosv4::economy::money::MicroCoin;
   621	    use turingosv4::runtime::autopsy_capsule::{
   622	        cluster_autopsies, AgentAutopsyCapsule, LossReasonClass,
   623	    };
   624	    use turingosv4::state::q_state::{AgentId, Hash, TaskId};
   625	    use turingosv4::state::typed_tx::{CapsulePrivacyPolicy, EventId};
   626	
   627	    let mk = |agent: &str| AgentAutopsyCapsule {
   628	        capsule_id: Cid::from_content(agent.as_bytes()),
   629	        agent_id: AgentId(agent.into()),
   630	        event_id: EventId(TaskId("e".into())),
   631	        loss_amount: MicroCoin::from_micro_units(1),
   632	        loss_reason_class: LossReasonClass::Bankruptcy,
   633	        violated_risk_rule: None,
   634	        suggested_policy_patch: None,
   635	        evidence_cids: vec![],
   636	        public_summary: format!("agent={} lost 1μC reason=Bankruptcy", agent),
   637	        private_detail_cid: Cid::default(),
   638	        privacy_policy: CapsulePrivacyPolicy::AuditOnly,
   639	        sha256: Hash::ZERO,
   640	        created_at_logical_t: 0,
   641	        created_at_round: 0,
   642	    };
   643	    let autopsies = vec![mk("A"), mk("B"), mk("C")];
   644	    let summaries = cluster_autopsies(&autopsies, 3);
   645	    assert_eq!(summaries.len(), 1, "FC2-N30: 3 same-class → 1 broadcast");
   646	    assert_eq!(summaries[0].count, 3);
   647	}
   648	
   649	/// FC3-N43 (TB-15 Atom 5): MarkovEvidenceCapsule + writer + default-deny
   650	/// gate witness. Capsule references constitution_hash (SG-15.7);
   651	/// deep-history default-deny without override (FR-15.5 + halt-trigger #6).
   652	#[test]
   653	fn fc3_n43_markov_capsule_witness() {
   654	    use turingosv4::runtime::markov_capsule::{
   655	        try_deep_history_read_with_override_check, MarkovEvidenceCapsule, MarkovGenError,
   656	    };
   657	
   658	    // SG-15.7: constitution_hash field plumbed through.
   659	    let cap = MarkovEvidenceCapsule::with_constitution_hash([0xAB; 32]);
   660	    assert_eq!(cap.constitution_hash.0, [0xAB; 32]);
   661	
   662	    // FR-15.5 + halt-trigger #6: default-deny without override.
   663	    match try_deep_history_read_with_override_check(false) {
   664	        Err(MarkovGenError::DeepHistoryReadDenied) => {}
   665	        other => panic!("FC3-N43: expected DeepHistoryReadDenied; got {other:?}"),
   666	    }
   667	    assert!(try_deep_history_read_with_override_check(true).is_ok());
   668	}

exec
/bin/bash -lc 'rg -n "pub fn open|enum CasError|struct CasStore|fn put|create_dir_all|Repository|git2" src/bottom_white/cas/store.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
1://! CAS store backed by git2-rs blob layer.
26:use git2::{ObjectType as Git2ObjectType, Repository};
38:pub enum CasError {
39:    /// git2-rs underlying error.
40:    Git2(git2::Error),
57:            Self::Git2(e) => write!(f, "git2 backend error: {e}"),
74:impl From<git2::Error> for CasError {
75:    fn from(e: git2::Error) -> Self {
151:pub struct CasStore {
163:    pub fn open(repo_path: &Path) -> Result<Self, CasError> {
165:        let _repo = match Repository::open(&repo_path) {
167:            Err(_) => Repository::init(&repo_path)?,
173:    fn open_repo(&self) -> Result<Repository, CasError> {
174:        Repository::open(&self.repo_path).map_err(CasError::from)
178:    pub fn put(
220:        let git_oid = git2::Oid::from_str(&metadata.backend_oid_hex)
274:    fn put_get_round_trip_small() {
282:    fn put_get_round_trip_large() {
291:    fn put_idempotent_same_content() {
373:    fn put_many_then_iterate_count() {

exec
/bin/bash -lc "nl -ba src/bottom_white/cas/store.rs | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
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

exec
/bin/bash -lc "nl -ba src/bottom_white/cas/store.rs | sed -n '180,360p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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
   246	    }
   247	
   248	    pub fn is_empty(&self) -> bool {
   249	        self.index.is_empty()
   250	    }
   251	
   252	    /// Merkle root over all CAS object metadata; deterministic per BTreeMap order.
   253	    pub fn merkle_root(&self) -> [u8; 32] {
   254	        let mut h = Sha256::new();
   255	        for (_cid, meta) in &self.index {
   256	            h.update(meta.canonical_hash());
   257	        }
   258	        h.finalize().into()
   259	    }
   260	}
   261	
   262	#[cfg(test)]
   263	mod tests {
   264	    use super::*;
   265	    use tempfile::TempDir;
   266	
   267	    fn fresh_store() -> (TempDir, CasStore) {
   268	        let tmp = TempDir::new().unwrap();
   269	        let store = CasStore::open(tmp.path()).unwrap();
   270	        (tmp, store)
   271	    }
   272	
   273	    #[test]
   274	    fn put_get_round_trip_small() {
   275	        let (_tmp, mut s) = fresh_store();
   276	        let cid = s.put(b"hello world", ObjectType::ProposalPayload, "alice", 100, None).unwrap();
   277	        let content = s.get(&cid).unwrap();
   278	        assert_eq!(content, b"hello world");
   279	    }
   280	
   281	    #[test]
   282	    fn put_get_round_trip_large() {
   283	        let (_tmp, mut s) = fresh_store();
   284	        let big = vec![0xab; 65536];
   285	        let cid = s.put(&big, ObjectType::PredicateBytecode, "system", 0, Some("wasm".into())).unwrap();
   286	        let content = s.get(&cid).unwrap();
   287	        assert_eq!(content, big);
   288	    }
   289	
   290	    #[test]
   291	    fn put_idempotent_same_content() {
   292	        let (_tmp, mut s) = fresh_store();
   293	        let cid_a = s.put(b"x", ObjectType::Generic, "alice", 1, None).unwrap();
   294	        let cid_b = s.put(b"x", ObjectType::Generic, "bob", 2, None).unwrap();
   295	        assert_eq!(cid_a, cid_b, "same content → same Cid");
   296	        // Index size = 1 (idempotent)
   297	        assert_eq!(s.len(), 1);
   298	    }
   299	
   300	    #[test]
   301	    fn cid_is_content_address() {
   302	        let (_tmp, mut s) = fresh_store();
   303	        let cid = s.put(b"specific content", ObjectType::Generic, "system", 0, None).unwrap();
   304	        // Cid is sha256 of content; verifiable independently
   305	        let expected = Cid::from_content(b"specific content");
   306	        assert_eq!(cid, expected);
   307	    }
   308	
   309	    #[test]
   310	    fn get_nonexistent_returns_error() {
   311	        let (_tmp, s) = fresh_store();
   312	        let bogus = Cid([0u8; 32]);
   313	        match s.get(&bogus) {
   314	            Err(CasError::CidNotFound(c)) => assert_eq!(c, bogus),
   315	            other => panic!("expected CidNotFound, got {other:?}"),
   316	        }
   317	    }
   318	
   319	    #[test]
   320	    fn metadata_recorded() {
   321	        let (_tmp, mut s) = fresh_store();
   322	        let cid = s.put(b"meta test", ObjectType::CounterexamplePayload, "carol", 250, Some("v1".into())).unwrap();
   323	        let meta = s.metadata(&cid).unwrap();
   324	        assert_eq!(meta.cid, cid);
   325	        assert_eq!(meta.object_type, ObjectType::CounterexamplePayload);
   326	        assert_eq!(meta.creator, "carol");
   327	        assert_eq!(meta.created_at_logical_t, 250);
   328	        assert_eq!(meta.schema_id.as_deref(), Some("v1"));
   329	        assert_eq!(meta.size_bytes, 9);
   330	    }
   331	
   332	    #[test]
   333	    fn merkle_root_deterministic_two_runs() {
   334	        let (_tmp1, mut s1) = fresh_store();
   335	        let (_tmp2, mut s2) = fresh_store();
   336	        for content in [b"a".as_slice(), b"b".as_slice(), b"c".as_slice()] {
   337	            s1.put(content, ObjectType::Generic, "system", 0, None).unwrap();
   338	        }
   339	        // Different insertion order
   340	        for content in [b"c".as_slice(), b"b".as_slice(), b"a".as_slice()] {
   341	            s2.put(content, ObjectType::Generic, "system", 0, None).unwrap();
   342	        }
   343	        assert_eq!(s1.merkle_root(), s2.merkle_root(),
   344	            "BTreeMap-ordered: insertion order independent (I-DET)");
   345	    }
   346	
   347	    #[test]
   348	    fn empty_store_root() {
   349	        let (_tmp, s) = fresh_store();
   350	        let r = s.merkle_root();
   351	        let expected: [u8; 32] = Sha256::new().finalize().into();
   352	        assert_eq!(r, expected, "empty store root = sha256(empty)");
   353	    }
   354	
   355	    #[test]
   356	    fn cell_isolation_disjoint_cas() {
   357	        // Per spec § 5.2.2 cross-cell isolation: separate runtime_repo paths
   358	        // → completely disjoint CasStore instances.
   359	        let (_tmp_a, mut store_a) = fresh_store();
   360	        let (_tmp_b, mut store_b) = fresh_store();

exec
/bin/bash -lc 'rg -n "std::fs::write|create_dir_all|write_markov_capsule|CasStore::open" src/bin/generate_markov_capsule.rs src/runtime/markov_capsule.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/runtime/markov_capsule.rs:10://! Generator surface: `write_markov_capsule(...)` (CAS-emitting) + binary
src/runtime/markov_capsule.rs:214:pub fn write_markov_capsule(
src/runtime/markov_capsule.rs:365:    /// TB-15 Atom 5 — write_markov_capsule writes 2 CAS objects
src/runtime/markov_capsule.rs:369:    fn write_markov_capsule_to_cas_round_trip() {
src/runtime/markov_capsule.rs:375:            CasStore::open(tmp.path()).expect("cas"),
src/runtime/markov_capsule.rs:382:        let cap = write_markov_capsule(
src/runtime/markov_capsule.rs:410:    /// TB-15 Atom 5 — write_markov_capsule deterministic (same inputs →
src/runtime/markov_capsule.rs:413:    fn write_markov_capsule_deterministic_capsule_id() {
src/runtime/markov_capsule.rs:419:            let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).unwrap()));
src/runtime/markov_capsule.rs:420:            write_markov_capsule(
src/runtime/markov_capsule.rs:449:        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).unwrap()));
src/runtime/markov_capsule.rs:451:        let cap_1 = write_markov_capsule(
src/runtime/markov_capsule.rs:465:        let cap_2 = write_markov_capsule(
src/bin/generate_markov_capsule.rs:42:    override_set_from_env, scan_unresolved_obs, sha256_of_file, write_markov_capsule, MarkovGenError,
src/bin/generate_markov_capsule.rs:267:            CasStore::open(cas_dir).map_err(|e| format!("open CAS: {e}"))?,
src/bin/generate_markov_capsule.rs:269:        write_markov_capsule(
src/bin/generate_markov_capsule.rs:286:            other => format!("write_markov_capsule: {other}"),
src/bin/generate_markov_capsule.rs:293:    std::fs::create_dir_all(&args.out_dir)
src/bin/generate_markov_capsule.rs:300:    std::fs::write(&json_path, &json_body).map_err(|e| format!("write json: {e}"))?;
src/bin/generate_markov_capsule.rs:302:    std::fs::write(&latest_path, capsule.capsule_id.hex())

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '3105,3205p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  3105	        // Stage 5: build LedgerEntrySigningPayload (v1.1 — stage 4 fetch_add
  3106	        // moved to AFTER stage 9 commit success).
  3107	        let signing_payload = LedgerEntrySigningPayload {
  3108	            logical_t,
  3109	            parent_state_root: q_snapshot.state_root_t,
  3110	            parent_ledger_root: q_snapshot.ledger_root_t,
  3111	            tx_kind: tx.tx_kind(),
  3112	            tx_payload_cid: payload_cid,
  3113	            resulting_state_root: q_next.state_root_t,
  3114	            timestamp_logical: logical_t,
  3115	            epoch: self.epoch,
  3116	            extensions: std::collections::BTreeMap::new(),
  3117	        };
  3118	
  3119	        // Stage 6: C3 — sign via typed CanonicalMessage::LedgerEntrySigning.
  3120	        let signing_digest = signing_payload.canonical_digest();
  3121	        let system_signature = transition_ledger_emitter::sign_ledger_entry(
  3122	            &self.keypair,
  3123	            signing_digest.0,
  3124	        )?;
  3125	
  3126	        // Stage 7: pure ledger-root fold (deterministic).
  3127	        let resulting_ledger_root = append(&q_snapshot.ledger_root_t, &signing_digest);
  3128	
  3129	        // Stage 8: build LedgerEntry (the stored record).
  3130	        let entry = LedgerEntry {
  3131	            logical_t: signing_payload.logical_t,
  3132	            parent_state_root: signing_payload.parent_state_root,
  3133	            parent_ledger_root: signing_payload.parent_ledger_root,
  3134	            tx_kind: signing_payload.tx_kind,
  3135	            tx_payload_cid: signing_payload.tx_payload_cid,
  3136	            resulting_state_root: signing_payload.resulting_state_root,
  3137	            resulting_ledger_root,
  3138	            timestamp_logical: signing_payload.timestamp_logical,
  3139	            epoch: signing_payload.epoch,
  3140	            extensions: signing_payload.extensions,
  3141	            system_signature,
  3142	        };
  3143	
  3144	        // Stage 9: commit + mutate Q_t under write lock.
  3145	        // v1.1 C-2: next_logical_t.store(logical_t) HAPPENS ONLY AFTER
  3146	        // writer.commit succeeds — preserves K1 under infra failure.
  3147	        // CO1.7-extra D2: q.head_t = NodeId(commit_oid_hex) via advance_head_t
  3148	        // when writer surfaces a commit OID (Git2 path); no-op preservation
  3149	        // for writers that return None (InMemory path). state_root_t comes
  3150	        // from q_next as-is per K3 v1.2.
  3151	        {
  3152	            let mut q_w = self.q.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
  3153	            let mut writer_w = self
  3154	                .ledger_writer
  3155	                .write()
  3156	                .map_err(|_| ApplyError::QStateLockPoisoned)?;
  3157	            writer_w.commit(&entry)?; // ← may fail; if it does, fetch_add was NOT called
  3158	            // commit succeeded → safe to advance counter.
  3159	            self.next_logical_t.store(logical_t, Ordering::SeqCst);
  3160	            *q_w = q_next;
  3161	            q_w.ledger_root_t = entry.resulting_ledger_root;
  3162	            // CO1.7-extra D2: close G-1 head_t carry-forward (Art 0.4).
  3163	            advance_head_t(&mut *q_w, &*writer_w);
  3164	        }
  3165	
  3166	        Ok(entry)
  3167	    }
  3168	
  3169	    /// Read-only accessor (testing + CO1.7.5+ wiring).
  3170	    pub fn q_snapshot(&self) -> Result<QState, ApplyError> {
  3171	        self.q
  3172	            .read()
  3173	            .map(|g| g.clone())
  3174	            .map_err(|_| ApplyError::QStateLockPoisoned)
  3175	    }
  3176	
  3177	    /// TRACE_MATRIX TB-14 Atom 6 B′ step 4 (FC2-N28; architect ruling
  3178	    /// 2026-05-03 §3+§4): build a canonical-keyed parent → children edge
  3179	    /// map by walking the L4 chain and reading
  3180	    /// `ProposalTelemetry.parent_tx` for each accepted WorkTx via its
  3181	    /// `proposal_cid`. Replaces the legacy shadow `kernel.tape`
  3182	    /// consumption at the bus snapshot's mask-set derivation site
  3183	    /// (canonical-graph rewire closes Codex R1 ship audit VETO; full
  3184	    /// detail in `handover/directives/2026-05-03_TB14_ATOM6_VETO_RULING.md`).
  3185	    ///
  3186	    /// **Replay determinism** (Art.0.2): the L4 chain + CAS payloads are
  3187	    /// both replay-deterministic per TB-13 chaintape evidence. Walking
  3188	    /// L4 in `read_at` order + reading ProposalTelemetry from CAS
  3189	    /// produces a byte-equal `BTreeMap<TxId, BTreeSet<TxId>>` across
  3190	    /// live vs replay.
  3191	    ///
  3192	    /// **Empty fallback**: failures at any layer (lock poisoned, CAS
  3193	    /// missing payload, canonical_decode error, ProposalTelemetry
  3194	    /// decode error, no parent_tx in telemetry) are silently skipped
  3195	    /// rather than propagated — bus.snapshot must NEVER crash because
  3196	    /// of an edge-map build failure (consumers handle empty as "no
  3197	    /// canonical edges yet"). The L4 chain itself is the canonical
  3198	    /// source of truth; this is a derived view.
  3199	    ///
  3200	    /// **Cost**: O(N + N·CAS_read) per call where N = L4 length. Bus
  3201	    /// snapshot frequency is bounded by the evaluator iteration cap;
  3202	    /// for a 50-iteration run this is ~50²/2 = 1250 CAS reads total.
  3203	    /// A future optimization can cache by writer.len() but is premature
  3204	    /// at B′ step 4.
  3205	    ///

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '3205,3315p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  3205	    ///
  3206	    /// **TB-9 zero-CID synthetic seed**: legacy synthetic-seed WorkTx
  3207	    /// (proposal_cid = `[0u8; 32]`) has no telemetry record; skipped
  3208	    /// silently (mirrors `chain_derived_run_facts` line 340 discipline).
  3209	    pub fn compute_canonical_edges_at_head(
  3210	        &self,
  3211	    ) -> std::collections::BTreeMap<crate::state::TxId, std::collections::BTreeSet<crate::state::TxId>>
  3212	    {
  3213	        use crate::bottom_white::ledger::transition_ledger::canonical_decode;
  3214	        use crate::runtime::proposal_telemetry::read_from_cas as read_proposal_telemetry;
  3215	
  3216	        let mut edges: std::collections::BTreeMap<
  3217	            crate::state::TxId,
  3218	            std::collections::BTreeSet<crate::state::TxId>,
  3219	        > = std::collections::BTreeMap::new();
  3220	
  3221	        let writer_r = match self.ledger_writer.read() {
  3222	            Ok(g) => g,
  3223	            Err(_) => return edges,
  3224	        };
  3225	        let cas_r = match self.cas.read() {
  3226	            Ok(g) => g,
  3227	            Err(_) => return edges,
  3228	        };
  3229	
  3230	        let n = writer_r.len();
  3231	        for t in 1..=n {
  3232	            let entry = match writer_r.read_at(t) {
  3233	                Ok(e) => e,
  3234	                Err(_) => continue,
  3235	            };
  3236	            // Only Work entries carry parent_tx via ProposalTelemetry.
  3237	            if entry.tx_kind != crate::bottom_white::ledger::transition_ledger::TxKind::Work {
  3238	                continue;
  3239	            }
  3240	            let payload = match cas_r.get(&entry.tx_payload_cid) {
  3241	                Ok(b) => b,
  3242	                Err(_) => continue,
  3243	            };
  3244	            let typed_tx: TypedTx = match canonical_decode(&payload) {
  3245	                Ok(tx) => tx,
  3246	                Err(_) => continue,
  3247	            };
  3248	            let work = match typed_tx {
  3249	                TypedTx::Work(w) => w,
  3250	                _ => continue,
  3251	            };
  3252	            // Skip TB-9 zero-CID synthetic seed (no ProposalTelemetry).
  3253	            if work.proposal_cid.0 == [0u8; 32] {
  3254	                continue;
  3255	            }
  3256	            let tel = match read_proposal_telemetry(&cas_r, &work.proposal_cid) {
  3257	                Ok(t) => t,
  3258	                Err(_) => continue,
  3259	            };
  3260	            if let Some(parent_tx) = tel.parent_tx {
  3261	                edges
  3262	                    .entry(parent_tx)
  3263	                    .or_insert_with(std::collections::BTreeSet::new)
  3264	                    .insert(work.tx_id);
  3265	            }
  3266	        }
  3267	
  3268	        edges
  3269	    }
  3270	
  3271	    pub fn next_submit_id_peek(&self) -> u64 {
  3272	        self.next_submit_id.load(Ordering::SeqCst)
  3273	    }
  3274	
  3275	    pub fn next_logical_t_peek(&self) -> u64 {
  3276	        self.next_logical_t.load(Ordering::SeqCst)
  3277	    }
  3278	}
  3279	
  3280	// ────────────────────────────────────────────────────────────────────────────
  3281	// Tests — stub-mode coverage (CO1.7.5 fills real-transition tests)
  3282	// ────────────────────────────────────────────────────────────────────────────
  3283	
  3284	#[cfg(test)]
  3285	mod tests {
  3286	    use super::*;
  3287	    use crate::bottom_white::ledger::transition_ledger::InMemoryLedgerWriter;
  3288	    use crate::state::typed_tx::{
  3289	        AgentSignature, BoolWithProof, ChallengeTx, ClaimId, FinalizeRewardTx, PredicateId,
  3290	        PredicateResultsBundle, ReadKey, ReuseTx, RunId, RunOutcome, SafetyOrCreation,
  3291	        TaskExpireTx, TerminalSummaryTx, ToolId, VerifyTx, VerifyVerdict, WorkTx,
  3292	        WriteKey,
  3293	    };
  3294	    use crate::state::q_state::{AgentId, TaskId, TxId};
  3295	    use crate::economy::money::{MicroCoin, StakeMicroCoin};
  3296	    use crate::bottom_white::cas::schema::Cid;
  3297	    use crate::bottom_white::ledger::system_keypair::SystemSignature;
  3298	    use std::collections::{BTreeMap, BTreeSet};
  3299	    use tempfile::TempDir;
  3300	
  3301	    fn fresh_sequencer() -> (
  3302	        TempDir,
  3303	        Sequencer,
  3304	        tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
  3305	        Arc<RwLock<RejectionEvidenceWriter>>,
  3306	    ) {
  3307	        let tmp = TempDir::new().expect("tempdir");
  3308	        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas open")));
  3309	        let keypair = Arc::new(
  3310	            Ed25519Keypair::generate_with_secure_entropy().expect("keypair gen"),
  3311	        );
  3312	        let epoch = SystemEpoch::new(1);
  3313	        let writer: Arc<RwLock<dyn LedgerWriter>> =
  3314	            Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
  3315	        let rejection_writer = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));

exec
/bin/bash -lc "rg -n \"q_snapshot|q_next|q_state|write\\(\\)|commit|ledger|fetch_add|next_logical_t\" src/state/sequencer.rs | sed -n '220,330p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
2633:                let logical_t_for_id = self.next_logical_t.load(Ordering::SeqCst) + 1;
2635:                    tx_id: crate::state::q_state::TxId(format!(
2663:                use crate::bottom_white::ledger::system_keypair::terminal_summary_emitter::sign_task_expire;
2676:                let logical_t_for_id = self.next_logical_t.load(Ordering::SeqCst) + 1;
2678:                    tx_id: crate::state::q_state::TxId(format!(
2717:                use crate::bottom_white::ledger::system_keypair::terminal_summary_emitter::sign_terminal_summary;
2723:                let logical_t_for_id = self.next_logical_t.load(Ordering::SeqCst) + 1;
2725:                    tx_id: crate::state::q_state::TxId(format!(
2761:                use crate::bottom_white::ledger::system_keypair::terminal_summary_emitter::sign_task_bankruptcy;
2767:                let logical_t_for_id = self.next_logical_t.load(Ordering::SeqCst) + 1;
2769:                    tx_id: crate::state::q_state::TxId(format!(
2798:        use crate::bottom_white::ledger::system_keypair::{verify_system_signature, CanonicalMessage};
2913:    /// rejection-writer semantics: no logical_t / state_root / ledger_root
2925:        q_snapshot: &QState,
2931:        let rejection_logical_t = self.next_logical_t.load(Ordering::SeqCst);
2936:                .write()
2951:                .write()
2969:                .write()
2973:                q_snapshot.state_root_t,
2987:    /// Pure transition + CAS put + sign + commit + Q_t mutation. See spec § 3
2992:    /// **v1.1 C-2 closure (Codex bundle Q-B)**: `next_logical_t` advances
2993:    /// **only on commit success** — the original spec § 3 stage-4
2994:    /// `fetch_add(1)` happened BEFORE sign + writer.commit, so any infra
2995:    /// failure (sign / commit) left `next_logical_t` advanced past a
2996:    /// logical_t that was never written to the ledger. The next accepted
2999:    /// after commit succeeds`. Single-writer per spec § 5.2.1 makes the
3012:        let q_snapshot = {
3026:            use crate::bottom_white::ledger::system_keypair::verify_system_signature;
3035:                self.record_rejection(submit_id, &tx, &q_snapshot, &err)?;
3041:        // ledger and return early. K1: no logical_t consumed; Inv 7: no
3042:        // state_root_t / ledger_root_t advance.
3043:        let (q_next, _signals) = match dispatch_transition(
3044:            &q_snapshot,
3051:                self.record_rejection(submit_id, &tx, &q_snapshot, &transition_err)?;
3052:                // No logical_t advance, no state_root advance, no ledger_root
3058:        // v1.1 C-2: TENTATIVE logical_t (do NOT fetch_add yet).
3059:        let logical_t = self.next_logical_t.load(Ordering::SeqCst) + 1;
3065:            let mut cas_w = self.cas.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
3086:                &q_snapshot.economic_state_t,
3088:                q_snapshot.q_t.current_round,
3105:        // Stage 5: build LedgerEntrySigningPayload (v1.1 — stage 4 fetch_add
3106:        // moved to AFTER stage 9 commit success).
3109:            parent_state_root: q_snapshot.state_root_t,
3110:            parent_ledger_root: q_snapshot.ledger_root_t,
3113:            resulting_state_root: q_next.state_root_t,
3121:        let system_signature = transition_ledger_emitter::sign_ledger_entry(
3126:        // Stage 7: pure ledger-root fold (deterministic).
3127:        let resulting_ledger_root = append(&q_snapshot.ledger_root_t, &signing_digest);
3133:            parent_ledger_root: signing_payload.parent_ledger_root,
3137:            resulting_ledger_root,
3144:        // Stage 9: commit + mutate Q_t under write lock.
3145:        // v1.1 C-2: next_logical_t.store(logical_t) HAPPENS ONLY AFTER
3146:        // writer.commit succeeds — preserves K1 under infra failure.
3147:        // CO1.7-extra D2: q.head_t = NodeId(commit_oid_hex) via advance_head_t
3148:        // when writer surfaces a commit OID (Git2 path); no-op preservation
3150:        // from q_next as-is per K3 v1.2.
3152:            let mut q_w = self.q.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
3154:                .ledger_writer
3155:                .write()
3157:            writer_w.commit(&entry)?; // ← may fail; if it does, fetch_add was NOT called
3158:            // commit succeeded → safe to advance counter.
3159:            self.next_logical_t.store(logical_t, Ordering::SeqCst);
3160:            *q_w = q_next;
3161:            q_w.ledger_root_t = entry.resulting_ledger_root;
3170:    pub fn q_snapshot(&self) -> Result<QState, ApplyError> {
3213:        use crate::bottom_white::ledger::transition_ledger::canonical_decode;
3221:        let writer_r = match self.ledger_writer.read() {
3237:            if entry.tx_kind != crate::bottom_white::ledger::transition_ledger::TxKind::Work {
3275:    pub fn next_logical_t_peek(&self) -> u64 {
3276:        self.next_logical_t.load(Ordering::SeqCst)
3287:    use crate::bottom_white::ledger::transition_ledger::InMemoryLedgerWriter;
3294:    use crate::state::q_state::{AgentId, TaskId, TxId};
3297:    use crate::bottom_white::ledger::system_keypair::SystemSignature;
3323:        let mut pinned = crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys::new();
3405:        assert_eq!(seq.next_logical_t_peek(), 0);
3410:        assert_eq!(seq.next_logical_t_peek(), 0, "logical_t MUST NOT advance at submit");
3414:        assert_eq!(seq.next_logical_t_peek(), 0);
3419:    //    (K1 invariant: rejected submission never advances commit counter).
3425:        let pre = seq.next_logical_t_peek();
3435:        let post = seq.next_logical_t_peek();
3444:    // q_snapshot.state_root_t carried in. Locks P1:6 contract.
3448:        let pre = seq.q_snapshot().expect("q_snapshot").state_root_t;
3474:        let post = seq.q_snapshot().expect("q_snapshot").state_root_t;
3476:        assert_eq!(seq.next_logical_t_peek(), 0, "no logical_t consumed");
3541:    // seeded escrow. Asserts q_next.state_root_t equals exactly
3556:        // and seed solver balance directly (genesis-equivalent for stake commitment).
3599:        let (q_next, _signals) = dispatch_transition(&q_funded, &tx, &preds, &tools)
3613:        assert_eq!(q_next.state_root_t, expected, "state_root_t must match WORKTX_ACCEPT_DOMAIN_V1 hash");
3614:        assert_ne!(q_next.state_root_t, q_funded.state_root_t, "state_root_t must advance on accept");
3618:        assert_ne!(q_next.economic_state_t, q_funded.economic_state_t,
3619:            "TB-3: accepted WorkTx commits stake (debits balance + credits stakes_t)");
3620:        let stake_entry = q_next.economic_state_t.stakes_t.0
3640:        let mut pinned = crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys::new();
3700:        let (q_next, _signals) = dispatch_transition(&q, &tx, &preds, &tools)
3703:        let entry = q_next.economic_state_t.task_markets_t.0
3713:        assert!(q_next.economic_state_t.balances_t.0.is_empty());
3714:        assert!(q_next.economic_state_t.escrows_t.0.is_empty());
3718:        assert_eq!(q_next.state_root_t, expected);
3719:        assert_ne!(q_next.state_root_t, Hash::ZERO);
3729:        // First open: q ← q_next (in test we manually compose).
3775:        let (q_next, _) = dispatch_transition(&q, &open, &preds, &tools)
3777:        q_next
3792:        let (q_next, _signals) = dispatch_transition(&q, &lock, &preds, &tools)
3796:        let new_bal = q_next.economic_state_t.balances_t.0
3802:        let escrow = q_next.economic_state_t.escrows_t.0.get(&lock_tx_id)
3809:        let market = q_next.economic_state_t.task_markets_t.0
3816:        assert_eq!(q_next.state_root_t, expected);
3883:        let (q_next, _) = dispatch_transition(&q, &lock, &preds, &tools)
3885:        q_next
3929:        let (q_next, _) = result.unwrap();
3932:        assert_eq!(q_next.state_root_t, expected);

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '2998,3020p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  2998	    /// (writer enforces strict `len + 1`). Fixed by `load → use → store
  2999	    /// after commit succeeds`. Single-writer per spec § 5.2.1 makes the
  3000	    /// load+store atomic enough; if multi-writer ever lands the AtomicU64
  3001	    /// can be upgraded to a `compare_exchange` reservation pattern.
  3002	    pub(crate) fn apply_one(
  3003	        &self,
  3004	        envelope: SubmissionEnvelope,
  3005	    ) -> Result<LedgerEntry, ApplyError> {
  3006	        // TB-2 Atom 2: queue payload is SubmissionEnvelope so submit_id
  3007	        // travels with the tx through to apply_one. Atom 4: submit_id is
  3008	        // now actually used for the L4.E rejection-evidence path below.
  3009	        let SubmissionEnvelope { submit_id, tx } = envelope;
  3010	
  3011	        // Stage 1: snapshot Q_t under read lock.
  3012	        let q_snapshot = {
  3013	            let g = self.q.read().map_err(|_| ApplyError::QStateLockPoisoned)?;
  3014	            g.clone()
  3015	        };
  3016	
  3017	        // TB-5 Atom 4 (preflight § 4.5): Stage 1.5 — defense-in-depth signature
  3018	        // verification for system-emitted variants. Even though emit_system_tx
  3019	        // signs the message before queueing, apply_one re-verifies against
  3020	        // pinned_pubkeys here so that any future bypass of emit_system_tx

exec
/bin/bash -lc 'rg -n "pub struct TxId|pub struct StakesIndex|pub struct StakeEntry" src/state/q_state.rs src/state/typed_tx.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/q_state.rs:71:pub struct TxId(pub String);
src/state/q_state.rs:294:pub struct StakesIndex(pub BTreeMap<TxId, StakeEntry>);
src/state/q_state.rs:305:pub struct StakeEntry {

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '270,350p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   270	/// fields without breaking deserialization of historical ledger rows.
   271	///
   272	/// **TB-3 additive field**: `task_id` is the back-reference to the `TaskId`
   273	/// this escrow funds. Required by `assert_task_market_total_escrow_matches_locks`
   274	/// (the cache=truth invariant for `TaskMarketEntry.total_escrow`). Additive
   275	/// serde-default — pre-TB-3 serialized rows deserialize with the empty TaskId.
   276	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   277	pub struct EscrowEntry {
   278	    #[serde(default = "MicroCoin::zero")]
   279	    pub amount: MicroCoin,
   280	    #[serde(default)]
   281	    pub depositor: AgentId,
   282	    #[serde(default)]
   283	    pub task_id: TaskId,
   284	}
   285	
   286	impl Default for EscrowEntry {
   287	    fn default() -> Self {
   288	        Self { amount: MicroCoin::zero(), depositor: AgentId::default(), task_id: TaskId::default() }
   289	    }
   290	}
   291	
   292	/// TRACE_MATRIX WP § 2 — tx → stake entry. Full schema lands CO P2.5 ChallengeCourt.
   293	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   294	pub struct StakesIndex(pub BTreeMap<TxId, StakeEntry>);
   295	
   296	/// TRACE_MATRIX WP § 2 — stake entry shape (stub). Full fields land CO P2.5.
   297	///
   298	/// **TB-3 additive field**: `task_id` records the task this stake commits
   299	/// to. Required by the WorkTx admission gate (TB-3 § 3.4 lock-on-accept):
   300	/// when an accepted WorkTx commits its inline `stake` into `stakes_t`, the
   301	/// entry carries the task binding so future RSP-2/3 challenge resolution
   302	/// can route the slash/release. Additive serde-default — pre-TB-3
   303	/// serialized rows deserialize with the empty TaskId.
   304	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   305	pub struct StakeEntry {
   306	    #[serde(default = "MicroCoin::zero")]
   307	    pub amount: MicroCoin,
   308	    #[serde(default)]
   309	    pub staker: AgentId,
   310	    #[serde(default)]
   311	    pub task_id: TaskId,
   312	}
   313	
   314	impl Default for StakeEntry {
   315	    fn default() -> Self {
   316	        Self { amount: MicroCoin::zero(), staker: AgentId::default(), task_id: TaskId::default() }
   317	    }
   318	}
   319	
   320	/// TRACE_MATRIX WP § 2 — tx → reward claim. Full schema lands CO P2.6 SettlementEngine.
   321	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   322	pub struct ClaimsIndex(pub BTreeMap<TxId, ClaimEntry>);
   323	
   324	/// TRACE_MATRIX WP § 2 — claim entry shape. Extended in TB-8 Atom 1
   325	/// (2026-05-02) per `handover/audits/CHARTER_RATIFICATION_TB_8_2026-05-02.md`
   326	/// §1 Q1 ratification: 6 new fields drive the Atom-3 FinalizeReward dispatch
   327	/// arm without re-traversing stakes_t / escrows_t / L4. All additive; every
   328	/// field carries `#[serde(default)]` so historical rows (TB-3..TB-7R never
   329	/// wrote a ClaimEntry — claims_t was a never-written stub) deserialize
   330	/// cleanly when re-read post-TB-8.
   331	///
   332	/// `status: ClaimStatus` is the terminal-state discriminator: `Open` at
   333	/// claim-creation (Atom-1 writer at VerifyTx OMEGA-Confirm), `Finalized` after
   334	/// the dispatch arm atomically credits the solver. `Slashed` is reserved for
   335	/// post-TB-15 slash-execution territory (per directive 2026-05-02 ruling 13).
   336	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   337	pub struct ClaimEntry {
   338	    #[serde(default = "MicroCoin::zero")]
   339	    pub amount: MicroCoin,
   340	    #[serde(default)]
   341	    pub claimant: AgentId,
   342	    /// TB-8 Atom 1: which task's escrow funds this claim.
   343	    #[serde(default)]
   344	    pub task_id: TaskId,
   345	    /// TB-8 Atom 1: which `escrows_t` row to debit at finalize time.
   346	    #[serde(default)]
   347	    pub escrow_lock_tx_id: TxId,
   348	    /// TB-8 Atom 1: the accepted WorkTx whose OMEGA-Confirm produced this claim.
   349	    #[serde(default)]
   350	    pub work_tx_id: TxId,

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '20,60p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    20	
    21	use serde::{Deserialize, Serialize};
    22	
    23	use crate::economy::money::MicroCoin;
    24	
    25	// ────────────────────────────────────────────────────────────────────────────
    26	// Newtype primitives — minimal, deterministic, serde-ready.
    27	// ────────────────────────────────────────────────────────────────────────────
    28	
    29	/// TRACE_MATRIX § 1.1 — generic 32-byte hash (sha256). State / ledger / registry roots.
    30	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    31	pub struct Hash(pub [u8; 32]);
    32	
    33	impl Hash {
    34	    /// TRACE_MATRIX § 1.1 — additive identity (genesis state-root, ledger-root, etc.).
    35	    pub const ZERO: Hash = Hash([0u8; 32]);
    36	
    37	    /// TRACE_MATRIX § 1.1 — construct from a 32-byte digest (sha256 output).
    38	    pub fn from_bytes(b: [u8; 32]) -> Self {
    39	        Hash(b)
    40	    }
    41	}
    42	
    43	impl Default for Hash {
    44	    fn default() -> Self {
    45	        Hash::ZERO
    46	    }
    47	}
    48	
    49	/// TRACE_MATRIX Art 0.4 — `head_t` = git commit SHA in Path B substrate (40 hex chars).
    50	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    51	pub struct NodeId(pub String);
    52	
    53	impl NodeId {
    54	    /// TRACE_MATRIX § 3 — pseudocode `NodeId::from_state_root(state_root)` constructor.
    55	    /// Concrete derivation (commit-tree-of-state-root) lands in CO1.7 transition_ledger.
    56	    pub fn from_state_root(state_root: Hash) -> Self {
    57	        let mut s = String::with_capacity(64);
    58	        for byte in state_root.0.iter() {
    59	            s.push_str(&format!("{:02x}", byte));
    60	        }

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '60,82p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    60	        }
    61	        NodeId(s)
    62	    }
    63	}
    64	
    65	/// TRACE_MATRIX § 1.1 — agent identity (string, opaque to Q_t).
    66	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    67	pub struct AgentId(pub String);
    68	
    69	/// TRACE_MATRIX § 1.1 — accepted-transaction id (string, opaque to Q_t).
    70	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    71	pub struct TxId(pub String);
    72	
    73	/// TRACE_MATRIX WP § 19 RSP-1 — task-market entry id; opaque string.
    74	///
    75	/// **TB-3 home migration (2026-04-30)**: previously defined at
    76	/// `src/state/typed_tx.rs:33-35`. Per WP § 19 RSP-1 ("TaskMarket — 发布任务、
    77	/// 广播价格、锁定奖金") + the TB-3 charter § 4.2 schema migration, `TaskId`
    78	/// is now the canonical `TaskMarketsIndex` key — it belongs alongside
    79	/// `AgentId` / `TxId` in the Q_t identifier layer, not in the typed-tx ABI
    80	/// layer. The move closes a circular-dependency that would have arisen if
    81	/// `q_state.rs` imported `TaskId` from `typed_tx.rs` (which already imports
    82	/// `AgentId` / `TxId` from `q_state.rs`).

exec
/bin/bash -lc 'find handover/markov_capsules -maxdepth 2 -type f -print 2>/dev/null | sort' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt
handover/markov_capsules/MARKOV_TB-15_2026-05-03.json

exec
/bin/bash -lc "find . -path '*tb_15*' -o -path '*markov*' | sed -n '1,200p'" in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc "rg -n \"b244f16a1f3bd532d041a40fe39b2b7e7cc12fb58e18b61aedd76a8010eeb1b6|575b5fd9c2a385bf748b941e0596dcda4b0542095329f1956f0a7122c06e4280|MarkovEvidenceCapsule|NextSessionContext\" .turingos_cas_index.jsonl handover runtime_repo cas . 2>/dev/null | sed -n '1,160p'" in /home/zephryj/projects/turingosv4
 succeeded in 111ms:
oke before claiming generator works — emitted capsule_id b244f16a... matches cargo test --no-cas mode) + feedback_workspace_test_canonical (cargo test --workspace = 870/0/150; canonical reporting) + feedback_iteration_cap_24h (Class 2 24h per atom; no extension needed; pure scaffolding TB) + feedback_no_retroactive_evidence_rewrite (TB-15 wire-in is going-forward only; pre-TB-15 TaskBankruptcy events do NOT get retroactive autopsy emission)	<pending Atom 6 ship commit>
handover/tracer_bullets/TB-15_charter_2026-05-03.md:16:- **Flowchart 3 (meta)**: `FC3-N43` = `MarkovEvidenceCapsule` schema + `generate_markov_capsule` binary (constitution_hash + L4 root + L4.E root + CAS root + open OBS list + previous_capsule_cid; deterministic). FR-15.4 next-session bootstrap source.

 succeeded in 1359ms:
./handover/markov_capsules
./handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt
./handover/markov_capsules/MARKOV_TB-15_2026-05-03.json
./handover/evidence/tb_15_markov_capsule_2026-05-03
./handover/evidence/tb_15_markov_capsule_2026-05-03/README.md
./handover/evidence/tb_15_markov_capsule_2026-05-03/LATEST_MARKOV_CAPSULE.txt
./handover/evidence/tb_15_markov_capsule_2026-05-03/MARKOV_TB-15_2026-05-03.json
./handover/audits/run_gemini_tb_15_ship_audit.py
./handover/audits/run_codex_tb_15_ship_audit.sh
./target/debug/.fingerprint/turingosv4-781bd0dbcea7e7b0/dep-bin-generate_markov_capsule
./target/debug/.fingerprint/turingosv4-781bd0dbcea7e7b0/bin-generate_markov_capsule
./target/debug/.fingerprint/turingosv4-781bd0dbcea7e7b0/bin-generate_markov_capsule.json
./target/debug/.fingerprint/turingosv4-62c9c807e031f677/dep-test-integration-test-tb_15_halt_triggers
./target/debug/.fingerprint/turingosv4-62c9c807e031f677/test-integration-test-tb_15_halt_triggers
./target/debug/.fingerprint/turingosv4-62c9c807e031f677/test-integration-test-tb_15_halt_triggers.json
./target/debug/.fingerprint/turingosv4-e62df01438d11db9/test-bin-generate_markov_capsule.json
./target/debug/.fingerprint/turingosv4-e62df01438d11db9/test-bin-generate_markov_capsule
./target/debug/.fingerprint/turingosv4-e62df01438d11db9/dep-test-bin-generate_markov_capsule
./target/debug/.fingerprint/turingosv4-cbdd7e007dd4d04f/dep-test-integration-test-tb_15_halt_triggers
./target/debug/.fingerprint/turingosv4-cbdd7e007dd4d04f/test-integration-test-tb_15_halt_triggers
./target/debug/.fingerprint/turingosv4-cbdd7e007dd4d04f/test-integration-test-tb_15_halt_triggers.json
./target/debug/.fingerprint/turingosv4-c519b89da1cc8b5c/dep-bin-generate_markov_capsule
./target/debug/.fingerprint/turingosv4-c519b89da1cc8b5c/bin-generate_markov_capsule
./target/debug/.fingerprint/turingosv4-c519b89da1cc8b5c/bin-generate_markov_capsule.json
./target/debug/.fingerprint/turingosv4-16b6bdc35f7c9a93/dep-bin-generate_markov_capsule
./target/debug/.fingerprint/turingosv4-16b6bdc35f7c9a93/bin-generate_markov_capsule
./target/debug/.fingerprint/turingosv4-16b6bdc35f7c9a93/bin-generate_markov_capsule.json
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5.lock
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/60v1r5oz2k9v5mi82e4r2ucdj.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/dep-graph.bin
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/1daozwrfhjg0sio56ev07yye4.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/ep3ve448opblkfcc1qers9qaz.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/203ibgkgpqg9ywna9asx9jlzy.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/0tplm6hnd75ei9ggkfl75pbcf.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/1d3jrz1jwjv7th74y38kvtp5g.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/dutz5mm5ivfqmcfmu29s4isl1.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/64yidd05btfvse3vqzw33w8kx.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/8fmiqiwrgjyi7jmmyq0uh0rd5.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/45lqjuwa1df6vqk7pek5h3l8t.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/e4rcphev9hbov4rq5pywi0sce.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/1zmfykfjcbjv3406c9g0o71b9.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/bag1cyydao6p2cq542iwctrdg.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/5ed336gmf0ohschp1p9jqq3av.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/2c7ics3y0xog1kl8bc3jyty41.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/451lqhrtb8k4p1wh4i5l7n1ce.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/0p152typt5m0r2avy4xolruak.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/query-cache.bin
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/1wu6ykcgvlkuocwxvwk9cn6i9.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/4ea2bjagy9hk5un2gp32jhaej.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/38svaz7zlibfmv94w00jroxll.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/1d7e38k7mdzdeprarn1azej87.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/eivnmilzshrrxdln8noj77mbu.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/6cdoqm10uejfw2sjlj177bpfn.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/ersnfqqxc0aeq3ymbw2ad6zcu.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/e2t1y9so52g76voiudjnk3vnd.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/work-products.bin
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/5gdeye1ffjs3sfm7a1zcfv98z.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/55rds769oo7dtt9ln74cle1le.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/dny6k9mdoj2aopnhf9k24y8gn.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/4nzw406n9981xay2i0w9lzvnn.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/25rtiworgpw9l1q1ygjuggvke.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/ewhcq6cbwbpcqq57yniz1e2ui.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/3v910a9lhx6f4hajn4vah8k3f.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/7or270n0kggccwz2b8w9eiazz.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/9h5h3z2coi9utaaiikyt9w0l5.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/3d9h0olj4ahnip7kolull9ax5.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/ae2eymolze22q8qqrhdnpweya.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/1ina5c71q21u10tqbr76kxzk8.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/4wu9wi0o3ej5lkm0ss077ac1x.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/278g3mfdftw2w8motix544r1v.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/8htqsa2pnuylh22iqgzybtb5r.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/ax8vi63s2owr5izbnk56cm26e.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/5ultyso9qfmjolwwm77pks6ft.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/0julx3zdmr0g70ilx4cncesuy.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71wqjj8x-0ye2ul5-9xqj8pcx1k338yfml3cre2gup/9zhwt00x2kggyawswnjk2zetg.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/60v1r5oz2k9v5mi82e4r2ucdj.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/dep-graph.bin
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/1daozwrfhjg0sio56ev07yye4.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/ep3ve448opblkfcc1qers9qaz.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/203ibgkgpqg9ywna9asx9jlzy.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/0tplm6hnd75ei9ggkfl75pbcf.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/1d3jrz1jwjv7th74y38kvtp5g.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/dutz5mm5ivfqmcfmu29s4isl1.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/64yidd05btfvse3vqzw33w8kx.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/8fmiqiwrgjyi7jmmyq0uh0rd5.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/45lqjuwa1df6vqk7pek5h3l8t.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/e4rcphev9hbov4rq5pywi0sce.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/1zmfykfjcbjv3406c9g0o71b9.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/bag1cyydao6p2cq542iwctrdg.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/5ed336gmf0ohschp1p9jqq3av.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/2c7ics3y0xog1kl8bc3jyty41.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/451lqhrtb8k4p1wh4i5l7n1ce.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/0p152typt5m0r2avy4xolruak.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/query-cache.bin
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/1wu6ykcgvlkuocwxvwk9cn6i9.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/4ea2bjagy9hk5un2gp32jhaej.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/38svaz7zlibfmv94w00jroxll.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/1d7e38k7mdzdeprarn1azej87.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/eivnmilzshrrxdln8noj77mbu.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/6cdoqm10uejfw2sjlj177bpfn.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/ersnfqqxc0aeq3ymbw2ad6zcu.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/e2t1y9so52g76voiudjnk3vnd.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/work-products.bin
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/5gdeye1ffjs3sfm7a1zcfv98z.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/55rds769oo7dtt9ln74cle1le.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/dny6k9mdoj2aopnhf9k24y8gn.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/4nzw406n9981xay2i0w9lzvnn.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/25rtiworgpw9l1q1ygjuggvke.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/ewhcq6cbwbpcqq57yniz1e2ui.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/3v910a9lhx6f4hajn4vah8k3f.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/7or270n0kggccwz2b8w9eiazz.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/9h5h3z2coi9utaaiikyt9w0l5.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/3d9h0olj4ahnip7kolull9ax5.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/ae2eymolze22q8qqrhdnpweya.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/1ina5c71q21u10tqbr76kxzk8.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/4wu9wi0o3ej5lkm0ss077ac1x.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/278g3mfdftw2w8motix544r1v.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/8htqsa2pnuylh22iqgzybtb5r.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/ax8vi63s2owr5izbnk56cm26e.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/5ultyso9qfmjolwwm77pks6ft.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/0julx3zdmr0g70ilx4cncesuy.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f-9xqj8pcx1k338yfml3cre2gup/9zhwt00x2kggyawswnjk2zetg.o
./target/debug/incremental/generate_markov_capsule-3nxarw72o0mu0/s-hi71u9yxtt-0goxe0f.lock
./target/debug/incremental/generate_markov_capsule-2gh5mou376064
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71ugvv5n-1tqof8i-1qez61onmkeq2bimx6vyhpi8g
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71ugvv5n-1tqof8i-1qez61onmkeq2bimx6vyhpi8g/dep-graph.bin
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71ugvv5n-1tqof8i-1qez61onmkeq2bimx6vyhpi8g/7onxp46dndovpa8ccgftymfmk.o
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71ugvv5n-1tqof8i-1qez61onmkeq2bimx6vyhpi8g/51ttu740q9ggzwixwsbqkcxnz.o
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71ugvv5n-1tqof8i-1qez61onmkeq2bimx6vyhpi8g/0pirybpv0c5zrplfrvzduipj5.o
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71ugvv5n-1tqof8i-1qez61onmkeq2bimx6vyhpi8g/query-cache.bin
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71ugvv5n-1tqof8i-1qez61onmkeq2bimx6vyhpi8g/work-products.bin
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71ugvv5n-1tqof8i-1qez61onmkeq2bimx6vyhpi8g/8qh8m1yhjxphln86bpi1acn1o.o
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71ugvv5n-1tqof8i-1qez61onmkeq2bimx6vyhpi8g/akxrtz27yq8y95bv1etgcx33q.o
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71wm2gne-1ylpfv0.lock
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71wm2gne-1ylpfv0-1qez61onmkeq2bimx6vyhpi8g
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71wm2gne-1ylpfv0-1qez61onmkeq2bimx6vyhpi8g/dep-graph.bin
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71wm2gne-1ylpfv0-1qez61onmkeq2bimx6vyhpi8g/7onxp46dndovpa8ccgftymfmk.o
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71wm2gne-1ylpfv0-1qez61onmkeq2bimx6vyhpi8g/51ttu740q9ggzwixwsbqkcxnz.o
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71wm2gne-1ylpfv0-1qez61onmkeq2bimx6vyhpi8g/0pirybpv0c5zrplfrvzduipj5.o
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71wm2gne-1ylpfv0-1qez61onmkeq2bimx6vyhpi8g/query-cache.bin
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71wm2gne-1ylpfv0-1qez61onmkeq2bimx6vyhpi8g/work-products.bin
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71wm2gne-1ylpfv0-1qez61onmkeq2bimx6vyhpi8g/8qh8m1yhjxphln86bpi1acn1o.o
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71wm2gne-1ylpfv0-1qez61onmkeq2bimx6vyhpi8g/akxrtz27yq8y95bv1etgcx33q.o
./target/debug/incremental/generate_markov_capsule-2gh5mou376064/s-hi71ugvv5n-1tqof8i.lock
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi711wvbam-13j9rsr.lock
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/aaai4xcnquck6b41gz1x77924.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/2yi1x9fm2s1ikh0i1gw8pk158.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/ekrfr8y7etn5ul1z43ysuypyf.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/dep-graph.bin
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/c5ahq8hcid2r1wygafssfpz0q.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/bk3frhpfyt0kxftmtduz7o201.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/d58puzczg6n2ypk8py5cr13c8.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/814aun3ard2jf4a0o81wj4ep2.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/au1fnrn7alnbn87x447puh5a5.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/0rnqdqsotxrc6ohe0c4arczrz.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/5dlozlaeus216o4s6s35dnlr8.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/38vxz5bkd5cmn5ishjxagcbji.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/cdh9b1y9f2uukszpute4tkbp2.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/a2go9ro9op8ic7to1fw8ivnob.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/43i4hnwz09medp9xkluq5kb0d.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/e0s6nefb859ypkwi0lzuq6wr9.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/4rmdtoztrim35gbqmuxnfrbiv.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/esbf6167hiov2lv1i3kyhwkb1.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/bn2vmy9rxy6g9wfvjmp84go11.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/0bydtsv4ovee4zauk9h0ckvon.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/ewfwknbmmqt8hfz8y0s7fb5ku.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/bzn4x3zyuxlnsscswplptmwlr.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/55hyecipzfv3r8nre6fgntr13.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/526ivxxq4vtd8q3o2290xof8r.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/9bd0jpplyg3p717idf2dycd9a.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/419jsf0ca4e2m9yfueq7z4o7p.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/3fd0o80j5yckycylz0rd33mfr.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/3whiszcnl26y7xzjweeugnejx.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/query-cache.bin
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/cg4bmphkkq2rijqs2tffbig7u.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/e0dvotjr7eh4y8omnlrlley0u.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/cpkwv550662yxuhky8texgovu.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/b8jg7z6wstirohddkuijzd0v8.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/e2u3hkgmkljzkop542efc6sle.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/a7m49ki9p1tg06pgbo83xb0hk.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/e7netv8i7dp5q103ykpqscbqm.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/3r8ybrgdtn8iyam6lodkjat49.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/work-products.bin
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/7rnzbraqzav83zyyegn6ptz09.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/0q9diw9zsyw7qs3qp90x0zjse.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/04u2vuqtxinikbsf5wmubq7m1.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/bx05yu9fguat0g2ucn1la221o.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/c4f0w2h8i3mjv2pysa4p6eg0m.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/akdk02y99mo3kai3mkwib9x0g.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/dmwb3n1pruhjbethw5it567sy.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/3yzu78e5af6qs42u809es5mt6.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/352r93ler9jimb1w6owg2hu1z.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/7mmt2qktlvblbgcue7ussyjjs.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/bnilejlwunntzxc81mx39b2z4.o
./target/debug/incremental/tb_15_halt_triggers-3v1xkdryr1a30/s-hi70xa7xke-1qlxqwp-3vlbusqs0xtimdmgww5es2a4r/7c2bnow7p89pbg32y9hmujtv9.o

exec
/bin/bash -lc 'test -e handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md; echo $?' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
1

codex
I have enough to write the R1 report. The verdict is CHALLENGE, not VETO: I found no global raw-log leak and no automatic predicate/tool mutation, but several claimed SG surfaces are structural or deferred rather than actually enforced.
2026-05-04T01:58:45.252111Z ERROR codex_core::tools::router: error=patch rejected: writing is blocked by read-only sandbox; rejected by user approval settings
codex
I completed the audit, but could not save `handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md`: the workspace is in read-only mode and `apply_patch` was rejected.

Key cleared areas: TaskBankruptcy autopsy derivation uses the pre-bankruptcy snapshot in dispatch and apply (`src/state/sequencer.rs:1367`, `src/state/sequencer.rs:3083`), `AgentVisibleProjection` has no autopsy fields (`src/state/q_state.rs:130`), `AutopsyIndex` stores only `Vec<Cid>` (`src/state/q_state.rs:748`), and autopsy code does not mutate predicate/tool registries (`src/runtime/autopsy_capsule.rs:253`, `src/runtime/autopsy_capsule.rs:437`).

## VERDICT: CHALLENGE
- Q3 CHALLENGE: shipped first Markov artifact used `--no-cas`, so CAS residency and `next_session_context_cid` resolvability are not proven by the ship artifact (`handover/evidence/tb_15_markov_capsule_2026-05-03/README.md:9`, `src/bin/generate_markov_capsule.rs:221`).
- Q4 CHALLENGE: deep-history override is a helper/test fixture, not enforced on a live generator deep-history path; default-denied decision is not logged (`src/bin/generate_markov_capsule.rs:177`, `src/bin/generate_markov_capsule.rs:193`).
- Q5 CHALLENGE: halt-trigger #5 scans JSON output for raw 32-byte `[0xAA; 32]` windows, which would miss a JSON-serialized `Cid` leak (`tests/tb_15_halt_triggers.rs:267`, `src/bottom_white/cas/schema.rs:13`).
- Q8/RQ7 CHALLENGE: literal SG-15.7 requires flowchart hashes, but `MarkovEvidenceCapsule` has no such field despite canonical hashes existing (`handover/tracer_bullets/TB-15_charter_2026-05-03.md:405`, `src/runtime/markov_capsule.rs:52`, `handover/alignment/TRACE_FLOWCHART_MATRIX.md:19`).
- Q9 CHALLENGE: dashboard §15 is privacy-safe but does not regenerate from ChainTape + CAS; `autopsy_event_counts` is hard-coded empty (`src/bin/audit_dashboard.rs:954`, `handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:124`).

Conviction: high. Recommendation: FIX-THEN-PROCEED. No retro-Class-3 promotion is required for current code because I found no raw private-log leak and no AgentVisibleProjection/read-authorization modification.
tokens used
247,365
I completed the audit, but could not save `handover/audits/CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md`: the workspace is in read-only mode and `apply_patch` was rejected.

Key cleared areas: TaskBankruptcy autopsy derivation uses the pre-bankruptcy snapshot in dispatch and apply (`src/state/sequencer.rs:1367`, `src/state/sequencer.rs:3083`), `AgentVisibleProjection` has no autopsy fields (`src/state/q_state.rs:130`), `AutopsyIndex` stores only `Vec<Cid>` (`src/state/q_state.rs:748`), and autopsy code does not mutate predicate/tool registries (`src/runtime/autopsy_capsule.rs:253`, `src/runtime/autopsy_capsule.rs:437`).

## VERDICT: CHALLENGE
- Q3 CHALLENGE: shipped first Markov artifact used `--no-cas`, so CAS residency and `next_session_context_cid` resolvability are not proven by the ship artifact (`handover/evidence/tb_15_markov_capsule_2026-05-03/README.md:9`, `src/bin/generate_markov_capsule.rs:221`).
- Q4 CHALLENGE: deep-history override is a helper/test fixture, not enforced on a live generator deep-history path; default-denied decision is not logged (`src/bin/generate_markov_capsule.rs:177`, `src/bin/generate_markov_capsule.rs:193`).
- Q5 CHALLENGE: halt-trigger #5 scans JSON output for raw 32-byte `[0xAA; 32]` windows, which would miss a JSON-serialized `Cid` leak (`tests/tb_15_halt_triggers.rs:267`, `src/bottom_white/cas/schema.rs:13`).
- Q8/RQ7 CHALLENGE: literal SG-15.7 requires flowchart hashes, but `MarkovEvidenceCapsule` has no such field despite canonical hashes existing (`handover/tracer_bullets/TB-15_charter_2026-05-03.md:405`, `src/runtime/markov_capsule.rs:52`, `handover/alignment/TRACE_FLOWCHART_MATRIX.md:19`).
- Q9 CHALLENGE: dashboard §15 is privacy-safe but does not regenerate from ChainTape + CAS; `autopsy_event_counts` is hard-coded empty (`src/bin/audit_dashboard.rs:954`, `handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md:124`).

Conviction: high. Recommendation: FIX-THEN-PROCEED. No retro-Class-3 promotion is required for current code because I found no raw private-log leak and no AgentVisibleProjection/read-authorization modification.
