# Codex Strict-Constitutional Audit Dispatch — TB-C0 + post-strict-audit fixes (2026-05-07)

**Authority**: User 2026-05-07 — "你先修复，修复后让codex严格按宪法落地审一次完整的代码".

**Mode**: Codex external auditor; NOT bundled with Gemini in this dispatch (user may add Gemini separately for dual-audit per CR-C0.8). Per `feedback_audit_after_evidence`: this dispatch comes AFTER fixes are landed AND the strict self-audit is documented.

**Conservative ranking** (per `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS. Codex's verdict overrides any optimistic framing in this document.

---

## §1 Scope of audit

Codex must adversarially audit the **complete code** for constitutional landing per CLAUDE.md PRIME OPERATING MODE + `constitution.md` + 3 flowcharts (FC1 + FC2 + FC3). Specifically:

1. **TB-C0 round 1-5 commit chain** on main (`0537869` → `f3b8e0a` → `480ebba` → `fa55c40` → `e825efe` → `2a3f5f9` → THIS COMMIT)
2. **Round-5 fixes** (this commit) per `OBS_TBC0_FC1_INV3_THREE_BUGS_2026-05-06.md` §10:
   - **Bug 1 fix** (Class 2): `handover/tests/scripts/run_tbc0_multi_agent_evidence.sh` runner now derives `EXPECTED_COMPLETED` from `tool_dist.step` (LLM-cycle count), not `tx_count` (which includes non-LLM transactions)
   - **Bug 3 fix** (claimed Class 3 with explicit deviation stance from original Class 4 STEP_B classification): `src/runtime/chain_derived_run_facts.rs` adds `pub capsule_anchored_attempt_count: u64` field with `#[serde(default)]`; `compute_run_facts_from_chain_with_invariant` walks CAS for AttemptTelemetry with `outcome == AttemptOutcome::PartialAccepted`; `delta` formula extended to constitutional 3-term `l4 + l4e + capsule_anchored - expected`
   - **FC1-INV6 fix** (Class 3): `src/runtime/audit_assertions.rs` adds `assert_50_cas_bytes_match_cids` (Layer B) that walks all CAS objects via new `CasStore::list_all_cids` and re-hashes bytes against stored CIDs
   - **Extractor tautology fix** (Class 1): `scripts/fc_witness_extract.py` reads `capsule_anchored_attempt_count` from chain_invariant.json (independent count) instead of `AT - l4 - l4e` (tautological)
3. **Trust-root rehashes** in `genesis_payload.toml` for `src/runtime/chain_derived_run_facts.rs` + `src/bottom_white/cas/store.rs`
4. **Bug 2 escalation** — synthetic L4.E gate (atom A.1) NOT fixed; STEP_B-restricted; user must ratify before sequencer.rs touch

---

## §2 Critical questions for Codex

### Q1 — Bug 3 deviation stance: is `chain_derived_run_facts.rs` correctly classified Class 3, NOT Class 4?

The original OBS classified Bug 3 as "Class 4 STEP_B" because the canonical-equation contract is bumped. Round-5 reclassified it to Class 3 with explicit deviation stance:

> Not Class 4 STEP_B because:
> (a) `chain_derived_run_facts.rs` is NOT in CLAUDE.md STEP_B restricted file list (only `sequencer.rs` + `typed_tx.rs` + `cas/schema.rs` + `bus.rs` + `kernel.rs` + `wallet.rs` are listed)
> (b) field is strictly additive with `#[serde(default)]` — backward-compat with all pre-fix evidence
> (c) NOT a canonical-signing-payload mutation — `ChainDerivedRunFacts` is a runtime-derived facts struct, not a typed-tx schema

**Codex Q1**: is this deviation stance defensible? Or should this have been STEP_B parallel-branch with dual audit + architect ratification before landing on main?

Per `feedback_architect_deviation_stance`: "take explicit position; flag = fence-sitting". Round-5 took the explicit position. Codex must adjudicate.

### Q2 — Bug 1 fix correctness: is `tool_dist.step` the right LHS for the FC1 hard invariant?

The FC1 hard invariant (CLAUDE.md PRIME OPERATING MODE):
```
externalized_attempt_count == L4_WorkTx + L4E_WorkTx + capsule_anchored
```

Round-5 maps `externalized_attempt_count = tool_dist.step` (with fallback to `omega_wtool` then `tx_count`). Is this the right semantics?

Empirical observation: on P05 (n=5 max_tx=20, MaxTxExhausted), `tool_dist = {step:20, step_reject:13, step_partial_ok:8}`. So step=20 = step_reject(13) + step_partial_ok(8) - 1? Off by 1. Where does the 1 come from? Is `step` over-counting OR is one of the categories overlapping?

**Codex Q2**: trace evaluator's `tool_dist` accounting. Does `step` correctly equal "count of LLM-Lean externalized cycles"? Or does it have an implicit overlap (e.g., omega-final step counted in both `step` AND `omega_wtool`)?

### Q3 — Bug 3 capsule_anchored counting: is filtering on `AttemptOutcome::PartialAccepted` correct?

Round-5 implementation:
```rust
let mut capsule_anchored_attempt_count: u64 = 0;
let at_cids = cas.list_cids_by_object_type(ObjectType::AttemptTelemetry);
for cid in at_cids {
    let bytes = cas.get(&cid)?;
    let at: AttemptTelemetry = canonical_decode(&bytes)?;
    if matches!(at.outcome, AttemptOutcome::PartialAccepted) {
        capsule_anchored_attempt_count += 1;
    }
}
```

Per Phase 2 directive §3.2 + R3 §1.3 amended: `step_partial_ok` is CAS-only; AttemptOutcome::PartialAccepted variant=6.

**Codex Q3**: is this the EXHAUSTIVE definition of "explicitly anchored capsule attempt"? Could there be other AT records (e.g., `AttemptOutcome::Aborted = 5`?) that should ALSO count toward capsule_anchored?

Per the AttemptOutcome enum doc-comment, `Aborted` records are accounted in `attempt_aborted_count`, NOT `expected_completed_attempts`. So Aborted is the OTHER half (numerator-side), not capsule_anchored. Round-5 keeps them separate. Codex confirm.

### Q4 — FC1-INV6 fix completeness: does `assert_50_cas_bytes_match_cids` close the entire hole?

Round-5 added `assert_50` in Layer B that walks every CAS index entry, fetches bytes, re-hashes via `Cid::from_content` (SHA-256), compares to stored CID.

Verified empirically: `audit_tape_tamper` on P05 now detects **3/3** (was 2/3 — flip_cas_byte slipped past pre-fix).

**Codex Q4**: is there ANY other class of CAS-content tampering that this assertion misses?
- Tampering that changes content but preserves SHA-256 (collision attack): cryptographically infeasible for SHA-256 with no length extension on fixed-size CIDs
- Tampering that adds new CAS objects with valid CIDs but unreferenced from L4: `assert_50` walks the ENTIRE index, so unreferenced new objects WOULD be hashed; but they'd still pass (new CID = new content). Is that a hole?
- Tampering that removes CAS objects: `assert_10_payload_cid_resolves` (Layer B id 10) catches removal of L4-referenced objects; but what about removal of inner-payload-referenced objects (e.g., AttemptTelemetry's candidate_payload_cid pointing to ProposalPayload)?

### Q5 — Bug 2 escalation defensibility: is the empirical residue of `delta=1` on omega-success runs acceptable forward-bound?

Bug 2 (synthetic L4.E gate inflating l4e_count by 1 on omega-success runs) is NOT fixed in round-5. The empirical residue is `delta=1` on P03/P06/P07 (1-shot omega) and a piece of P05's `delta=1`. Round-5 escalated to STEP_B (sequencer.rs touch).

**Codex Q5**: is this acceptable forward-bound, OR should round-5 have done the STEP_B parallel-branch for Bug 2 as well?

User's "你先修复" directive could be read either way. Round-5's interpretation: STEP_B-restricted files require architect explicit ratification + parallel-branch protocol per CLAUDE.md Code Standard, even when fixing a real bug, because the protocol exists precisely to prevent direct-edit accidents on canonical surfaces.

### Q6 — Strict audit framing: was the round-4 "21 GREEN" claim DECEPTIVE or just OPTIMISTIC?

`STRICT_AUDIT_TBC0_TAPE_2026-05-07.md` self-downgraded round-4's claim. Specifically:
- Round-4 used "GREEN if ANY problem witnesses → aggregate GREEN" rule
- Strict audit: GREEN should require "every problem RED-free for chain-resident invariants"
- The round-4 framing led to a closure-candidate label that the strict audit downgraded to "REMEDIATION NEEDED"

**Codex Q6**: was this downgrade itself adequate? Or does it still over-claim post-fix? The post-fix aggregate is "18 chain-resident GREEN + 0 RED + 7 AMBER (4 structural-only + 3 forward-bound)". Is this an HONEST baseline, or should additional nodes be downgraded?

### Q7 — Constitution gate test coverage: are there clauses with NO test at all?

`CONSTITUTION_EXECUTION_MATRIX.md` claims every clause has a row. Is that exhaustive?

**Codex Q7**: enumerate any constitutional clause in `constitution.md` that has NO matrix row + NO test. If found, those are GAPS that the strict audit missed.

### Q8 — Is `FC_WITNESS_CATALOG_2026-05-06.md` real-problem-binding sound?

The catalog binds each FC node to specific MiniF2F problems that produced its tape witness. Per `feedback_real_problems_not_designed`: real existing problems, not synthesized.

**Codex Q8**: are any of the bindings "speculative" rather than empirically demonstrated by `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/`? If so, those bindings are claims-without-evidence.

### Q9 — TB-C0 closure readiness: is round-5 sufficient for architect §8 sign-off?

TB-C0 charter §4 SG-C0.1..14 are all satisfied at structural level. Empirical batch passes 1131/0/151 workspace tests + 54/0/1 constitution gates. Bug 2 escalation is the last forward-bound item.

**Codex Q9**: should architect §8 sign-off proceed NOW (with Bug 2 documented as forward), OR wait until Bug 2 is also fixed via STEP_B? Per `feedback_audit_obs_bias`: don't bucket-OBS all residuals. Per `feedback_class4_cannot_hide_in_class3`: STEP_B is canonical for restricted files. The conservative reading would defer §8.

---

## §3 Inputs Codex should read

**Authority**:
- `CLAUDE.md` (PRIME OPERATING MODE = Constitutional Harness Engineering since 2026-05-06)
- `constitution.md` (canonical; SHA hashes pinned in `genesis_payload.toml`)
- `handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md`
- `handover/tracer_bullets/TB-C0_charter_2026-05-06.md`

**Strict audit + fixes**:
- `handover/alignment/STRICT_AUDIT_TBC0_TAPE_2026-05-07.md` (self-audit, round-4 downgraded)
- `handover/alignment/OBS_TBC0_FC1_INV3_THREE_BUGS_2026-05-06.md` §10 (round-5 fix log)
- `handover/tracer_bullets/TB-C0_CLOSURE_REPORT_2026-05-06.md` (round-4; over-rosy in retrospect)

**Constitutional CI**:
- `tests/constitution_*.rs` (8 files, 54 GREEN + 1 ignored)
- `scripts/run_constitution_gates.sh` (canonical runner)
- `scripts/fc_witness_extract.py` + `scripts/fc_witness_aggregate.py`
- `.github/workflows/constitution_gates.yml`

**Round-5 source-side changes**:
- `src/runtime/chain_derived_run_facts.rs` (Bug 3 — `capsule_anchored_attempt_count` field + 3-term delta)
- `src/runtime/audit_assertions.rs` (FC1-INV6 — `assert_50_cas_bytes_match_cids`)
- `src/bottom_white/cas/store.rs` (FC1-INV6 — `list_all_cids` helper)
- `src/bin/tb_18r_compute_invariant.rs` (Bug 3 — surface `capsule_anchored_attempt_count` in JSON output)
- `handover/tests/scripts/run_tbc0_multi_agent_evidence.sh` (Bug 1 — runner derives `EXPECTED_COMPLETED` from `tool_dist.step`)
- `genesis_payload.toml` (trust-root rehashes for the 2 modified src/ files)
- `scripts/fc_witness_extract.py` (extractor tautology fix — reads independent capsule_anchored)

**Empirical evidence**:
- `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/` (9-problem n=5 batch on round-4 binary; pre-fix)
- `target/audit_tape_tamper/P05_post_fix_verdict.json` (3/3 detected post-fix; FC1-INV6 closed)

**Conformance helpers**:
- `tests/fc_alignment_conformance.rs` (existing FC1/FC2/FC3 witness battery)
- `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` (clause→test→smoke→status matrix)
- `handover/alignment/TRACE_FLOWCHART_MATRIX.md` (per-FC-node binding)
- `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md` (per-FC-node real-problem binding)

---

## §4 What Codex's verdict should answer

For each of Q1..Q9 above:
- **PASS** with reasoning
- **CHALLENGE** with specific clause + evidence
- **VETO** with constitutional citation + remediation requirement

Aggregate: should TB-C0 ship FINAL now, ship FINAL after Bug 2 STEP_B, or be VETOed until further work?

Per `feedback_dual_audit_conflict`: if Codex VETOs and a future Gemini PASSes, conservative wins (VETO). The architect's §8 will adjudicate.

---

## §5 Cross-references

- `feedback_audit_after_evidence` — this dispatch is AFTER fixes
- `feedback_dual_audit` — Codex first; user may add Gemini separately for dual
- `feedback_dual_audit_conflict` — VETO > CHALLENGE > PASS
- `feedback_no_workarounds_strict_constitution` — "我不要凑活"; round-5 escalated Bug 2 explicitly rather than papering over
- `feedback_architect_deviation_stance` — round-5 took explicit position on Bug 3 Class 3 reclassification
- `feedback_class4_cannot_hide_in_class3` — applied to Bug 2 (sequencer.rs is STEP_B-restricted; not bundled)

---

**End of Codex dispatch. User invocation expected (cloud-billed; per Atom G0/G1 precedent).**
