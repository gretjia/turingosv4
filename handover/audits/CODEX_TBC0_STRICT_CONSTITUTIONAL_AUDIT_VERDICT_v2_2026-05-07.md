# Codex Constitutional Re-Audit Verdict v2 — TB-C0 round 6 (2026-05-07)

## §1 Aggregate verdict

**Aggregate verdict: CHALLENGE.**

Round 6 materially closes the round-5 VETO's empirical FC1 blocker: all 9 post-fix
`chain_invariant_post_fix.json` files on disk report `"delta": 0` and
`"invariant_verdict": "Ok"`, all 9 `architect_inv1_check_post_fix.json` files
report `"match": true`, the post-fix audit tapes report `"verdict": "PROCEED"`
with `"passed": 39`, and the aggregate artifact reports `"green_count": 20`,
`"amber_count": 5`, `"red_count": 0`, `"gap_count": 0`. However, I do not flip to
PASS. The Bug 2 filter is correct for the observed synthetic gate on disk but is
not robust as a constitutional discriminator because it filters only
`agent_id == "tb6-smoke-agent"` and does not bind to the existing
`synthetic_rejection_label.json` marker or to the rejected synthetic payload. The
strict aggregate script produces a correct current artifact, but its implementation
does not actually enforce "GREEN only if every problem GREEN" when a node is
missing from some problem manifests. The rewritten catalog is substantially better
but still contains stale/conflating summary claims. Therefore §8 sign-off is
**NOT-READY** until the CHALLENGE items below are corrected.

Verdict ranking applied: VETO > CHALLENGE > PASS.

Observed commit chain:

- `3e146e6` HEAD: round 6 remediation.
- `10e2beb` prior Codex audit verdict commit.
- `0d0877b` round 5 baseline previously VETOed.

Required read inputs were inspected:

- `handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_DISPATCH_2026-05-07.md`.
- `handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_2026-05-07.md`.
- `git show --stat 3e146e6`.
- `git show --stat 0d0877b`.

Verification commands run:

- `git log --oneline -5`.
- `git show --stat 3e146e6`.
- `find handover/evidence -name '*_post_fix.json' | sort | head -50`.
- `find handover -name 'FC_WITNESS_CATALOG*' -newer handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_2026-05-07.md -print`.
- `jq` checks over all 9 post-fix invariant, architect, audit tape, and aggregate files.
- Source reads of `src/runtime/chain_derived_run_facts.rs`.
- Source reads of `experiments/minif2f_v4/src/chain_runtime.rs`.
- Source reads of `scripts/regenerate_post_fix_evidence.sh`.
- Source reads of `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md`.
- Source reads of `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`.

I did not run LLM batches, evaluator reruns, expensive recompute, or any source
mutating command.

## §2 Per-question findings

### Q-RR1 — Bug 2 inline Class 3 fix correctness

**Status: CHALLENGE.**

Observed-on-disk facts:

- The round-6 source fix is in `src/runtime/chain_derived_run_facts.rs`, not in
  `src/state/sequencer.rs`.
- The L4.E Work rejection filter is implemented at
  `src/runtime/chain_derived_run_facts.rs:936` through
  `src/runtime/chain_derived_run_facts.rs:941`.
- Exact code path:
  - `src/runtime/chain_derived_run_facts.rs:935` defines
    `const SYNTHETIC_GATE_SPONSOR_AGENT_ID: &str = "tb6-smoke-agent";`.
  - `src/runtime/chain_derived_run_facts.rs:939` filters `r.tx_kind == TxKind::Work`.
  - `src/runtime/chain_derived_run_facts.rs:940` filters out
    `r.agent_id.0 != SYNTHETIC_GATE_SPONSOR_AGENT_ID`.
  - `src/runtime/chain_derived_run_facts.rs:942` through
    `src/runtime/chain_derived_run_facts.rs:947` separately counts filtered records
    where `r.agent_id.0 == SYNTHETIC_GATE_SPONSOR_AGENT_ID`.
- The code comments state the intended emit site at
  `src/runtime/chain_derived_run_facts.rs:926` through
  `src/runtime/chain_derived_run_facts.rs:928`.
- The actual synthetic gate emit function is
  `experiments/minif2f_v4/src/chain_runtime.rs::write_synthetic_l4_l4e_gate_and_genesis_report`,
  beginning at `experiments/minif2f_v4/src/chain_runtime.rs:350`.
- The synthetic zero-stake WorkTx is constructed at
  `experiments/minif2f_v4/src/chain_runtime.rs:371` through
  `experiments/minif2f_v4/src/chain_runtime.rs:378`.
- That emit site uses `"tb6-smoke-agent"` at
  `experiments/minif2f_v4/src/chain_runtime.rs:373`.
- The marker file is written at
  `experiments/minif2f_v4/src/chain_runtime.rs:394` through
  `experiments/minif2f_v4/src/chain_runtime.rs:400`.
- The marker JSON includes `"synthetic_rejection_for_l4e_gate": true` at
  `experiments/minif2f_v4/src/chain_runtime.rs:398`.
- `rg` found no `tb6-smoke-agent` use in `src/state/sequencer.rs`; the only
  source references are in `chain_derived_run_facts.rs` and
  `experiments/minif2f_v4/src/chain_runtime.rs`.

Q5(a), discriminator correctness:

- For the current round-6 evidence, `agent_id == "tb6-smoke-agent"` is an
  empirically correct discriminator.
- It matches the current synthetic gate producer at
  `experiments/minif2f_v4/src/chain_runtime.rs:371` through
  `experiments/minif2f_v4/src/chain_runtime.rs:378`.
- It explains the observed post-fix count changes:
  - P03 old `l4e_work_attempt_count=1` becomes post-fix
    `"l4e_work_attempt_count": 0`.
  - P05 old `l4e_work_attempt_count=13` becomes post-fix
    `"l4e_work_attempt_count": 12`.
  - P06 old `l4e_work_attempt_count=2` becomes post-fix
    `"l4e_work_attempt_count": 1`.
  - P09 old `l4e_work_attempt_count=7` becomes post-fix
    `"l4e_work_attempt_count": 6`.

Q5(b), robustness against attacker forging `agent_id`:

- I do **not** consider the current discriminator robust enough for PASS.
- Layer A assertion id 41 checks that chain-resident agent IDs are
  sandbox-prefixed, but it does not make `"tb6-smoke-agent"` unforgeable.
- The sandbox prefix logic treats `tb<digit>-...` as sandbox at
  `src/runtime/audit_assertions.rs:624` through
  `src/runtime/audit_assertions.rs:641`.
- Assertion id 41 walks rejected L4.E records at
  `src/runtime/audit_assertions.rs:772` through
  `src/runtime/audit_assertions.rs:785`.
- Assertion id 41 fails only on non-sandbox agent IDs at
  `src/runtime/audit_assertions.rs:775` through
  `src/runtime/audit_assertions.rs:783`.
- Therefore an L4.E Work rejection using `agent_id="tb6-smoke-agent"` satisfies
  the prefix check by construction.
- The filter in `chain_derived_run_facts.rs` does not inspect:
  - `synthetic_rejection_label.json`;
  - rejected payload tx id;
  - rejected payload stake;
  - rejected payload signature shape;
  - rejected payload `task_id`;
  - rejected payload suffix `"atom3-l4e-synthetic-rejection"`;
  - rejection class;
  - one-and-only-one synthetic gate cardinality.
- The synthetic WorkTx constructor uses a zero signature at
  `src/runtime/adapter.rs:128` through `src/runtime/adapter.rs:130`.
- Real WorkTx signing is a separate path at `src/runtime/adapter.rs:152` through
  `src/runtime/adapter.rs:199`.
- Sequencer rejection recording derives `agent_id` from `tx.submitter_id()` at
  `src/state/sequencer.rs:3098` through `src/state/sequencer.rs:3100`.
- The round-6 filter trusts that derived string alone.

Q5(c), sufficient or needs stronger signal:

- Stronger signal is required before PASS.
- Minimum remediation: require the marker at
  `<runtime_repo>/synthetic_rejection_label.json` and bind the filtered L4.E
  Work rejection to the known synthetic payload shape.
- The existing marker is explicitly emitted at
  `experiments/minif2f_v4/src/chain_runtime.rs:394` through
  `experiments/minif2f_v4/src/chain_runtime.rs:400`; not using it in the filter
  leaves available evidence unused.
- Better remediation: decode the rejected `TypedTx::Work` payload and require all
  of:
  - `agent_id == "tb6-smoke-agent"`;
  - `stake == 0`;
  - tx id suffix or task id matches the synthetic gate convention from
    `experiments/minif2f_v4/src/chain_runtime.rs:379` through
    `experiments/minif2f_v4/src/chain_runtime.rs:382`;
  - marker file exists and says `"synthetic_rejection_for_l4e_gate": true`;
  - exactly one such synthetic gate is excluded per runtime.

STEP_B reconsideration:

- My prior round-5 STEP_B escalation was too strong for the actual implemented
  source surface.
- The dispatch claim that the fix likely required `sequencer.rs` was not borne
  out by disk inspection.
- The actual emit site is not `sequencer.rs`; it is
  `experiments/minif2f_v4/src/chain_runtime.rs:350`.
- The round-6 implementation touched only the chain-derived facts surface.
- Because `src/runtime/chain_derived_run_facts.rs` is not the STEP_B-restricted
  `src/state/sequencer.rs`, the Class 3 location is defensible.
- But Class 3 location does not make the current discriminator robust.

Q-RR1 ruling:

- PASS for "actual emit site is not sequencer".
- PASS for "current evidence count correction".
- CHALLENGE for "agent_id-only filter is too weak as a constitutional
  discriminator".

Required condition to clear:

- Strengthen the filter with `synthetic_rejection_label.json` and decoded
  synthetic payload/cardinality checks, or add an assertion proving
  `"tb6-smoke-agent"` cannot appear in any real LLM-Lean Work rejection.

### Q-RR2 — Post-fix evidence completeness

**Status: PASS.**

Evidence enumeration:

- `find handover/evidence -name '*_post_fix.json' | sort | head -50` listed the
  expected per-problem files:
  - `architect_inv1_check_post_fix.json`;
  - `chain_invariant_post_fix.json`;
  - `fc_witness_manifest_post_fix.json`;
  - `verdict_post_fix.json`.
- `find handover/evidence -name '*_post_fix.json' | sort | wc -l` reported `37`.
- This equals 9 problems x 4 per-problem post-fix JSON files + 1 aggregate file.

Observed per-problem invariant facts:

- P01 `chain_invariant_post_fix.json`:
  - `"expected_completed_attempts": 1`;
  - `"l4_work_attempt_count": 1`;
  - `"l4e_work_attempt_count": 0`;
  - `"capsule_anchored_attempt_count": 0`;
  - `"delta": 0`;
  - `"invariant_verdict": "Ok"`.
- P02 `chain_invariant_post_fix.json`:
  - `"expected_completed_attempts": 1`;
  - `"l4_work_attempt_count": 1`;
  - `"l4e_work_attempt_count": 0`;
  - `"capsule_anchored_attempt_count": 0`;
  - `"delta": 0`;
  - `"invariant_verdict": "Ok"`.
- P03 `chain_invariant_post_fix.json`:
  - `"expected_completed_attempts": 1` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P03_mathd_algebra_141/chain_invariant_post_fix.json:5`.
  - `"l4_work_attempt_count": 1` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P03_mathd_algebra_141/chain_invariant_post_fix.json:7`.
  - `"l4e_work_attempt_count": 0` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P03_mathd_algebra_141/chain_invariant_post_fix.json:8`.
  - `"capsule_anchored_attempt_count": 0` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P03_mathd_algebra_141/chain_invariant_post_fix.json:3`.
  - `"delta": 0` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P03_mathd_algebra_141/chain_invariant_post_fix.json:4`.
  - `"invariant_verdict": "Ok"` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P03_mathd_algebra_141/chain_invariant_post_fix.json:6`.
- P04 `chain_invariant_post_fix.json`:
  - `"expected_completed_attempts": 3`;
  - `"l4_work_attempt_count": 1`;
  - `"l4e_work_attempt_count": 2`;
  - `"capsule_anchored_attempt_count": 0`;
  - `"delta": 0`;
  - `"invariant_verdict": "Ok"`.
- P05 `chain_invariant_post_fix.json`:
  - `"expected_completed_attempts": 20` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P05_mathd_algebra_114/chain_invariant_post_fix.json:5`.
  - `"l4_work_attempt_count": 0` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P05_mathd_algebra_114/chain_invariant_post_fix.json:7`.
  - `"l4e_work_attempt_count": 12` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P05_mathd_algebra_114/chain_invariant_post_fix.json:8`.
  - `"capsule_anchored_attempt_count": 8` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P05_mathd_algebra_114/chain_invariant_post_fix.json:3`.
  - `"delta": 0` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P05_mathd_algebra_114/chain_invariant_post_fix.json:4`.
  - `"invariant_verdict": "Ok"` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P05_mathd_algebra_114/chain_invariant_post_fix.json:6`.
- P06 `chain_invariant_post_fix.json`:
  - `"expected_completed_attempts": 2`;
  - `"l4_work_attempt_count": 1`;
  - `"l4e_work_attempt_count": 1`;
  - `"capsule_anchored_attempt_count": 0`;
  - `"delta": 0`;
  - `"invariant_verdict": "Ok"`.
- P07 `chain_invariant_post_fix.json`:
  - `"expected_completed_attempts": 50`;
  - `"l4_work_attempt_count": 0`;
  - `"l4e_work_attempt_count": 46`;
  - `"capsule_anchored_attempt_count": 4`;
  - `"delta": 0`;
  - `"invariant_verdict": "Ok"`.
- P08 `chain_invariant_post_fix.json`:
  - `"expected_completed_attempts": 44`;
  - `"l4_work_attempt_count": 0`;
  - `"l4e_work_attempt_count": 5`;
  - `"capsule_anchored_attempt_count": 39`;
  - `"delta": 0`;
  - `"invariant_verdict": "Ok"`.
- P09 `chain_invariant_post_fix.json`:
  - `"expected_completed_attempts": 10` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P09_aime_1984_p1/chain_invariant_post_fix.json:5`.
  - `"l4_work_attempt_count": 1` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P09_aime_1984_p1/chain_invariant_post_fix.json:7`.
  - `"l4e_work_attempt_count": 6` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P09_aime_1984_p1/chain_invariant_post_fix.json:8`.
  - `"capsule_anchored_attempt_count": 3` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P09_aime_1984_p1/chain_invariant_post_fix.json:3`.
  - `"delta": 0` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P09_aime_1984_p1/chain_invariant_post_fix.json:4`.
  - `"invariant_verdict": "Ok"` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P09_aime_1984_p1/chain_invariant_post_fix.json:6`.

Observed requested deltas:

- P03 pre-fix:
  - `"delta": 1`;
  - `"invariant_verdict": "Err(TB-18R FR-18R.3 violation: clean halt OmegaAccepted requires delta=0 but delta=1 (l4=1, l4e=1, expected=1))"`.
- P03 post-fix:
  - `"delta": 0`;
  - `"invariant_verdict": "Ok"`.
- P05 pre-fix:
  - `"delta": -7`;
  - `"invariant_verdict": "Err(TB-18R FR-18R.3 violation: delta<0 forbidden (delta=-7, halt=MaxTxExhausted) — attempt vanished pre-chain)"`.
- P05 post-fix:
  - `"delta": 0`;
  - `"invariant_verdict": "Ok"`.
- P06 pre-fix:
  - `"delta": 1`;
  - `"invariant_verdict": "Err(TB-18R FR-18R.3 violation: clean halt OmegaAccepted requires delta=0 but delta=1 (l4=1, l4e=2, expected=2))"`.
- P06 post-fix:
  - `"delta": 0`;
  - `"invariant_verdict": "Ok"`.
- P09 pre-fix:
  - `"delta": -2`;
  - `"invariant_verdict": "Err(TB-18R FR-18R.3 violation: delta<0 forbidden (delta=-2, halt=OmegaAccepted) — attempt vanished pre-chain)"`.
- P09 post-fix:
  - `"delta": 0`;
  - `"invariant_verdict": "Ok"`.

Observed architect invariant facts:

- All 9 `architect_inv1_check_post_fix.json` files report `"match": true`.
- P05 exact fields:
  - `"architect_inv_1": "chain_attempt_count == externalized_llm_cycle_count"` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P05_mathd_algebra_114/architect_inv1_check_post_fix.json:2`.
  - `"chain_attempt_count": 20` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P05_mathd_algebra_114/architect_inv1_check_post_fix.json:3`.
  - `"externalized_llm_cycle_count": 20` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P05_mathd_algebra_114/architect_inv1_check_post_fix.json:4`.
  - `"match": true` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P05_mathd_algebra_114/architect_inv1_check_post_fix.json:6`.
  - `"delta": 0` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P05_mathd_algebra_114/architect_inv1_check_post_fix.json:7`.

Observed audit tape facts:

- All 9 `verdict_post_fix.json` files report `"verdict": "PROCEED"`.
- P05 exact fields:
  - `"schema_version": "v1/audit_tape_verdict"` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P05_mathd_algebra_114/verdict_post_fix.json:2`.
  - `"verdict": "PROCEED"` is in the P05 JSON object.
  - `jq '{verdict, passed, failed, skipped, assertion_count:(.assertions|length)}'` on P05 reported:
    - `"verdict": "PROCEED"`;
    - `"passed": 39`;
    - `"failed": 0`;
    - `"skipped": 11`;
    - `"assertion_count": 50`.
- This matches the claim "PROCEED with 39 PASS (was 38)".

Trust-root rehash:

- `sha256sum src/runtime/chain_derived_run_facts.rs` reports
  `a4db21bb1004eb901c4a70184846f1f896df02b81900e1a009bd3a4e26e13fe0`.
- `genesis_payload.toml:253` pins
  `"src/runtime/chain_derived_run_facts.rs" = "a4db21bb1004eb901c4a70184846f1f896df02b81900e1a009bd3a4e26e13fe0"`.
- `git diff 0d0877b..3e146e6 -- genesis_payload.toml` shows predecessor
  `6ec00047beb8e254a5ca57db96a625a4c30adbff834a41651d4304972434f986`
  replaced by
  `a4db21bb1004eb901c4a70184846f1f896df02b81900e1a009bd3a4e26e13fe0`.
- Therefore the requested `6ec00047 -> a4db21bb` rehash matches disk.

Q-RR2 ruling:

- PASS.

### Q-RR3 — Strict aggregate semantics in `scripts/regenerate_post_fix_evidence.sh`

**Status: CHALLENGE.**

Observed current aggregate artifact:

- `fc_witness_aggregate_post_fix.json` exists at
  `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json`.
- It reports `"schema_version": 2` at
  `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:2`.
- It reports `"problem_count": 9` at
  `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:6`.
- It lists P01 through P09 at
  `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:7` through
  `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:17`.
- Summary fields:
  - `"total_nodes": 25` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:422`.
  - `"green_count": 20` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:423`.
  - `"amber_count": 5` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:424`.
  - `"red_count": 0` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:425`.
  - `"gap_count": 0` at
    `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:426`.
- The remediation protocol string at
  `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:428`
  claims: "STRICT semantics — RED if any RED; AMBER if any AMBER (no green) or
  mixed; GREEN only if every problem GREEN."

Observed current node examples:

- `FC1-INV1_every_attempt_tape_visible` is `"aggregate_status": "GREEN"` at
  `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:19`
  through
  `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:33`.
- `FC1-INV3_count_equality_constitutional` is `"aggregate_status": "GREEN"` at
  `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:35`
  through
  `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:49`.
- In both examples, `green_by` contains all 9 problems and `amber_by`/`red_by`
  are empty.

Observed script implementation:

- Aggregation starts at `scripts/regenerate_post_fix_evidence.sh:159`.
- Problem dirs are enumerated at `scripts/regenerate_post_fix_evidence.sh:162`
  through `scripts/regenerate_post_fix_evidence.sh:163`.
- Only existing `fc_witness_manifest_post_fix.json` files are loaded at
  `scripts/regenerate_post_fix_evidence.sh:165` through
  `scripts/regenerate_post_fix_evidence.sh:169`.
- Node keys are unioned from present manifests at
  `scripts/regenerate_post_fix_evidence.sh:171` through
  `scripts/regenerate_post_fix_evidence.sh:174`.
- Per-node aggregation iterates only loaded manifests at
  `scripts/regenerate_post_fix_evidence.sh:177` through
  `scripts/regenerate_post_fix_evidence.sh:186`.
- If a node is missing in a problem manifest, line
  `scripts/regenerate_post_fix_evidence.sh:182` does `if not v: continue`.
- Status rules are at `scripts/regenerate_post_fix_evidence.sh:187` through
  `scripts/regenerate_post_fix_evidence.sh:198`.
- `"GREEN"` is assigned whenever `green and not amber and not red` at
  `scripts/regenerate_post_fix_evidence.sh:195` through
  `scripts/regenerate_post_fix_evidence.sh:196`.
- The script never checks `len(green) == len(per_problem)`.
- The script never records `gap_by`.
- The script never marks a node AMBER/GAP/RED when present GREEN in one problem
  but missing in another.

Edge-case result:

- Current manifests happen to have all 25 nodes in all 9 problems.
- I verified this with a Python read-only scan:
  - output: `problems 9 allnodes 25`;
  - no node was printed as missing from any problem.
- Therefore the current aggregate counts are correct for the current artifacts.
- But the script does **not** implement the claimed strict rule for future or
  malformed artifacts.
- In a missing-node edge case, a node could be GREEN with `green_by=["P01"]` and
  absent from P02-P09.
- That violates the explicit Q-RR3 requirement: "GREEN only if every problem
  GREEN."

Q-RR3 ruling:

- PASS for current artifact counts: `20 GREEN + 5 AMBER + 0 RED + 0 GAP`.
- CHALLENGE for script semantics: missing-node edge case is not strict.

Required condition to clear:

- Add missing-node handling:
  - compute `missing_by = [p for p in problem_dirs if node not in manifest[p]]`;
  - set aggregate `GAP` if no problem reports the node;
  - set aggregate `AMBER` if `missing_by` is non-empty and no RED;
  - set aggregate `GREEN` only if `len(green_by) == problem_count` and
    amber/red/missing are empty;
  - persist `gap_by` or `missing_by` in the JSON.

### Q-RR4 — `FC_WITNESS_CATALOG` rewrite

**Status: CHALLENGE.**

Observed improvements:

- The new catalog is at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md`.
- `find handover -name 'FC_WITNESS_CATALOG*' -newer handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_2026-05-07.md -print`
  returned `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md`.
- The file header says "REVISED 2026-05-07" at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:1`.
- The status block lists the Codex Q8 remediation items at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:3` through
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:7`.
- The 3-class taxonomy is present at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:13` through
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:21`.
- `chain-resident` is defined at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:17`.
- `structural` is defined at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:18`.
- `tamper-probe` is defined at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:19`.
- The classes are explicitly declared non-substitutable at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:21`.

FC1-INV3 arithmetic:

- The old inconsistent arithmetic is replaced at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:85`.
- The new row states:
  - P05: `20=0+12+8`;
  - P07: `50=0+46+4`;
  - P08: `44=0+5+39`;
  - P09: `10=1+6+3`;
  - all 9 problems `delta=0 verdict=Ok`.
- These figures match the post-fix evidence fields on disk.

FC1-INV6 reclassification:

- The FC1-INV6 row is reclassified as tamper-probe at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:88`.
- The row explicitly says "tamper-probe (NOT real-problem)" at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:88`.
- This closes the prior direct conflation for FC1-INV6.

FC3-INV1 downgrade:

- The FC3-INV1 row is downgraded at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:131`.
- It distinguishes capsule presence from capsule integrity at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:131`.
- It says integrity is "NOT YET VERIFIED" at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:131`.
- It provides a path to GREEN at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:131`.

Residual catalog problems:

- The status flag legend still says GREEN means "at least one real problem on
  tape produces this witness" at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:66` through
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:69`.
- Several structural rows still use status `✅`, despite the taxonomy saying
  structural is not a chain-resident real-problem witness.
- Example: `FC1-INV4 no legacy bypass` is structural but has status `✅` at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:86`.
- Example: `FC1-INV5 dashboard not source` is structural but has status `✅` at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:87`.
- Example: `FC1-N6 input bundle` is structural but has status `✅` at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:91`.
- Example: `FC2-N25..N28 mr / tools_other` is structural but has status `✅` at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:106`.
- The catalog still uses stale Phase 3 phrasing for FC1-INV1 at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:84`:
  `AttemptTelemetry count == evaluator tx_count` and `✅ (5/7 on Phase 3)`.
- That conflicts with the round-6 remediation, where the architect invariant LHS
  is `externalized_llm_cycle_count`, not raw `tx_count`.
- The summary still says "25/25 FC nodes have a tape witness" at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:177`.
- The same sentence admits 5 are structural by design at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:177`.
- That still conflates "tape witness" with structural verification.
- The summary says "FC3 capsule (FC3-INV1, FC3-N39): GREEN" at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:182`.
- That conflicts with the FC3-INV1 row at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:131`, which correctly
  marks FC3-INV1 AMBER.
- The summary says "No GAPS" at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:185`.
- That is too strong given the same file documents FC3-INV1 integrity not yet
  verified at `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:131`.
- The summary says "No RED nodes" at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:187`, but also preserves
  old explanation that Bug 1/2/3 caused spurious R4-binary verdicts. This
  paragraph was not fully rewritten for the post-fix aggregate.

Q-RR4 ruling:

- PASS for taxonomy presence.
- PASS for FC1-INV3 arithmetic correction.
- PASS for FC1-INV6 reclassification.
- PASS for FC3-INV1 row downgrade and path-to-GREEN.
- CHALLENGE for residual stale/conflating summary and structural rows still
  marked as if they were real-problem tape witnesses.

Required condition to clear:

- Rewrite §2 status legend and §3 aggregate summary so:
  - chain-resident, structural, and tamper-probe status are separate columns or
    separate symbols;
  - "tape witness" is never used for structural-only rows;
  - FC3-INV1 is not summarized as GREEN while its row is AMBER;
  - FC1-INV1 uses `externalized_llm_cycle_count`, not `tx_count`.

### Q-RR5 — Constitution matrix corrections

**Status: CHALLENGE.**

Observed requested corrections:

- Art. 0.4 was downgraded from GREEN to AMBER at
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:31`.
- The row explicitly cites the unimplemented/pending git-style
  `HEAD_t` / `q_t` / `rtool` / `wtool` path choice at
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:31`.
- This matches the constitution gap at `constitution.md:124` through
  `constitution.md:149`.
- Constitution specifics:
  - `q_t` gap at `constitution.md:128`.
  - `HEAD_t` fully unimplemented at `constitution.md:129`.
  - `rtool` gap at `constitution.md:131`.
  - `wtool` gap at `constitution.md:132`.
  - implementation path pending at `constitution.md:136` through
    `constitution.md:149`.
- Art. V.3 was changed from N/A to RED at
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:83`.
- The row states a new executable test is required at
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:83`.
- The row cites the no-test=RED directive at
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:83`.

Residual matrix problems:

- The legend says RED means "no test" or docs-only at
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:12`.
- The status discipline says a row goes RED whenever its only evidence is a
  doc-comment or passing audit at
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:15` through
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:18`.
- Some rows still look optimistic or inconsistent relative to that discipline.
- Closure condition #6 says Markov / EvidenceCapsule passes FC3 is GREEN at
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:190`.
- That conflicts with the gate-level row `Capsule derived from tape + CAS` AMBER
  at `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:116`.
- It also conflicts with the catalog's honest FC3-INV1 AMBER at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:131`.
- Flowchart 3 row `Deep history requires override` is GREEN at
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:120`, while the catalog
  classifies FC3-INV5 as structural-only AMBER at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:135`.
- Flowchart 3 row `JudgeAI veto-only` is GREEN at
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:123`, while the catalog
  classifies FC3-INV8 as structural-only AMBER at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:138`.
- Shielding gate row `private_diagnostic_cid_not_serialized_publicly` is GREEN at
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:141`, while Article III
  rows that cover private diagnostics and raw shielding remain AMBER at
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:57` through
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:60`.
- Closure condition #2 says "Every critical row has a test" GREEN at
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:186`, while Art. V.3
  explicitly has `NEW test required` and RED at
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:83`.
- Closure condition #6 GREEN is especially important because §8 sign-off uses
  closure conditions; it should not remain GREEN while FC3 capsule derivation is
  AMBER.

Q-RR5 ruling:

- PASS for the two requested corrections:
  - Art. 0.4 GREEN -> AMBER.
  - Art. V.3 N/A -> RED.
- CHALLENGE for missed downstream consistency corrections in closure rows and
  FC3/meta rows.

Required condition to clear:

- Align closure condition #6 with FC3-INV1 AMBER.
- Align closure condition #2 with Art. V.3 RED or qualify "critical row".
- Reconcile FC3 structural-only rows between matrix and catalog.
- Avoid GREEN in the matrix where the catalog deliberately says structural-only
  AMBER.

### Q-RR6 — Updated aggregate verdict

**Status: CHALLENGE.**

Conservative ranking applied:

- Q-RR1 is CHALLENGE.
- Q-RR2 is PASS.
- Q-RR3 is CHALLENGE.
- Q-RR4 is CHALLENGE.
- Q-RR5 is CHALLENGE.
- Q-RR6 therefore cannot be PASS.
- No single issue rises to VETO because:
  - current FC1 post-fix evidence is complete;
  - current 9-problem aggregate artifact is numerically correct;
  - the round-5 empirical VETO blocker is materially resolved on disk;
  - the remaining issues are robustness/document/edge-case strictness issues,
    not a currently observed failed invariant on the 9 persisted problems.

Aggregate verdict:

- **CHALLENGE.**

## §3 New constitutional findings

### Finding C1 — Bug 2 filter is evidence-correct but not constitutionally robust

Severity: CHALLENGE.

Evidence:

- Filter constant: `src/runtime/chain_derived_run_facts.rs:935`.
- Agent-only exclusion: `src/runtime/chain_derived_run_facts.rs:936` through
  `src/runtime/chain_derived_run_facts.rs:941`.
- Existing marker not consumed by filter:
  - marker emitted at `experiments/minif2f_v4/src/chain_runtime.rs:394` through
    `experiments/minif2f_v4/src/chain_runtime.rs:400`;
  - filter code has no marker read at
    `src/runtime/chain_derived_run_facts.rs:911` through
    `src/runtime/chain_derived_run_facts.rs:948`.
- Sandbox prefix allows `tb...` fixture IDs:
  `src/runtime/audit_assertions.rs:624` through
  `src/runtime/audit_assertions.rs:641`.
- L4.E assertion id 41 only checks sandbox prefix:
  `src/runtime/audit_assertions.rs:772` through
  `src/runtime/audit_assertions.rs:785`.

Rationale:

- The current filter corrects P03/P05/P06/P09.
- It does not prove that all future `tb6-smoke-agent` Work rejections are
  synthetic gates.
- A constitutional audit should prefer a decoded, marker-bound, cardinality-bound
  discriminator.

### Finding C2 — Strict aggregate script has a missing-node false-GREEN edge case

Severity: CHALLENGE.

Evidence:

- The script ignores missing node values with `if not v: continue` at
  `scripts/regenerate_post_fix_evidence.sh:182`.
- The script assigns GREEN based only on non-empty `green` and empty `amber/red`
  at `scripts/regenerate_post_fix_evidence.sh:195` through
  `scripts/regenerate_post_fix_evidence.sh:196`.
- The script does not check `len(green) == len(per_problem)`.
- The script does not persist `gap_by` or `missing_by`.

Rationale:

- Current aggregate is correct because all manifests include all 25 nodes.
- The implementation still fails the stated strict semantics under missing-node
  input.
- This matters because the artifact's own protocol string promises "GREEN only
  if every problem GREEN" at
  `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json:428`.

### Finding C3 — Catalog still has stale/conflating status summaries

Severity: CHALLENGE.

Evidence:

- Taxonomy is correct at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:13` through
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:21`.
- FC1-INV6 is correctly tamper-probe at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:88`.
- FC3-INV1 is correctly AMBER at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:131`.
- But the summary still says "25/25 FC nodes have a tape witness" at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:177`.
- The summary still says FC3 capsule is GREEN at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:182`.
- The summary still says "No GAPS" at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:185`.

Rationale:

- The row-level rewrite is much better.
- The summary still preserves the exact class-conflation pattern that caused the
  prior Q8 VETO.
- This should be fixed before architect §8 uses the catalog as a sign-off input.

### Finding C4 — Constitution matrix closure conditions still overstate FC3

Severity: CHALLENGE.

Evidence:

- Art. 0.4 correction is good at
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:31`.
- Art. V.3 correction is good at
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:83`.
- FC3 capsule gate is AMBER at
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:116`.
- Closure condition #6 says Markov / EvidenceCapsule passes FC3 is GREEN at
  `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:190`.
- Catalog says FC3-INV1 integrity is not verified at
  `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md:131`.

Rationale:

- The targeted matrix corrections landed.
- The matrix has not been fully normalized downstream.
- A closure condition should not be greener than the gate it summarizes.

## §4 §8 architect sign-off readiness recommendation

**Recommendation: NOT-READY.**

Conditions to become READY:

1. Strengthen Bug 2 synthetic-gate filtering.
   - Required file: `src/runtime/chain_derived_run_facts.rs`.
   - Required behavior: filter only records proven synthetic by marker + decoded
     payload/cardinality, not by `agent_id` alone.
   - Suggested evidence: rerun post-fix invariant generation on the same 9
     existing problem directories and preserve `delta=0`.

2. Fix strict aggregate missing-node semantics.
   - Required file: `scripts/regenerate_post_fix_evidence.sh`.
   - Required behavior: `GREEN` only when every problem reports the node as
     GREEN; missing node in any problem must prevent GREEN.
   - Required artifact: regenerate
     `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json`
     with `gap_by` or `missing_by`.

3. Normalize `FC_WITNESS_CATALOG_2026-05-06.md`.
   - Remove "25/25 tape witness" language unless every row is chain-resident.
   - Keep FC1-INV6 as tamper-probe, not real-problem witness.
   - Keep FC3-INV1 AMBER in all summaries.
   - Fix FC1-INV1 wording to `externalized_llm_cycle_count`, not `tx_count`.

4. Normalize `CONSTITUTION_EXECUTION_MATRIX.md`.
   - Keep Art. 0.4 AMBER.
   - Keep Art. V.3 RED until executable test exists.
   - Downgrade or qualify closure condition #6 so it matches FC3-INV1 AMBER.
   - Reconcile structural-only FC3/meta rows between matrix and catalog.

5. Re-audit after those corrections.
   - No expensive LLM batch is required for the identified fixes.
   - Existing tape artifacts are sufficient for rerunning invariant/audit/aggregate
     derivations.

What is READY now:

- The 9-problem FC1 post-fix invariant evidence is ready.
- The round-6 trust-root rehash for `chain_derived_run_facts.rs` is ready.
- The current aggregate artifact's numeric counts are ready for the current
  manifests.

What is NOT ready:

- §8 final constitutional sign-off.
- PASS verdict.
- Claim that every §8 item is independently verified closed.

## §5 Cross-references

Prior dispatch:

- `handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_DISPATCH_2026-05-07.md`.

Prior Codex VETO verdict:

- `handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_2026-05-07.md`.

Current verdict:

- `handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_v2_2026-05-07.md`.

Commit hashes:

- `0d0877b` — TB-C0 round 5 baseline previously VETOed.
- `10e2beb` — prior Codex external audit verdict commit.
- `3e146e6` — TB-C0 round 6 remediation under this re-audit.

Key files cited:

- `src/runtime/chain_derived_run_facts.rs`.
- `experiments/minif2f_v4/src/chain_runtime.rs`.
- `src/runtime/audit_assertions.rs`.
- `src/runtime/adapter.rs`.
- `src/state/sequencer.rs`.
- `scripts/regenerate_post_fix_evidence.sh`.
- `genesis_payload.toml`.
- `handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md`.
- `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`.
- `constitution.md`.
- `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/fc_witness_aggregate_post_fix.json`.

Final line:

**Codex re-audit verdict v2: CHALLENGE; §8 architect sign-off NOT-READY.**
