# Strict Audit — TB-C0 Constitution Landing per actual tape evidence (2026-05-07)

**Trigger**: User 2026-05-07 — "根据tape真实情况严格审计宪法落地情况"; reinforced "每一个宪法的细节，都要求在真题测试中找到存在的证据，找不到的话有两个处理方式：1、找代码bug。2、找不到bug，去思考能够测试出这个功能的真题".

**Auditor**: Claude (self-audit, NOT Codex/Gemini external; per Generator≠Evaluator this is necessarily provisional pending external dual audit).

**Methodology**: Walk the actual chain artifacts in `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/` (9 problems × n=5; commit `fa55c40`) and re-verify each FC node / constitutional gate. Where my round-4 extractor reported GREEN, ask: is the witness REAL or HOLLOW (file-existence / tautological / SKIPPED)?

**Outcome (revised aggregate)**: 17 chain-resident GREEN, 7 AMBER, **1 RED (real)**, 0 GAP. The round-4 "21 GREEN" was OPTIMISTIC under "if-any-problem-witnesses-then-GREEN" semantics; strict audit downgrades 4 nodes whose witness was hollow + 1 to RED for an empirical tamper-detection hole.

---

## §1 What the strict audit revealed (top findings)

### Finding A — Round-4 aggregate GREEN was lenient (semantics issue, not a code bug)

Round-4 `fc_witness_aggregate.py` used "GREEN if ANY problem produces a green witness" rule. On strict reading that conflates "the harness CAN witness this" with "the harness DID witness this on this batch consistently". Re-reading per-problem:

| Node | Round-4 aggregate | Strict status | Why downgraded |
|------|-------------------|---------------|----------------|
| FC1-INV1 every-attempt-tape-visible | GREEN (8/9) | **AMBER** | P08 architect_inv1.match=False (Bug 1 manifest) |
| FC1-INV3 count-equality 3-term | GREEN (4/9) | **AMBER** | extractor's `capsule_anchored = AT - l4 - l4e` is tautological by construction (Bug 3) |
| FC1-INV6 no fake accepted nodes | GREEN (claim) | **RED** | audit_tape_tamper on P05 detects 2/3, NOT 3/3 — flip_cas_byte slips past |
| FC3-INV1 capsule derived | GREEN (3/9) | **AMBER** | capsule presence ≠ capsule integrity; markov_*_recompute SKIPPED on all 9 (no continuation chain) |

### Finding B — FC1-INV1 P08 anomaly is Bug 1 manifesting empirically

P08 `aime_1983_p1` (n=5, max_tx=50, MaxTxExhausted):
```
chain_attempt_count = 44   (AttemptTelemetry CAS objects — actual LLM-Lean cycles)
evaluator_reported_tx_count = 50   (tx_count in PPUT_RESULT)
delta = -6
```

Per `tool_dist`: `step = 44`, `step_partial_ok = 39`, `step_reject = 5`. So 5+39 = 44 = step. The 6 non-step transactions are TaskOpen (1) + EscrowLock (1) + TerminalSummary (1) + 3 others (likely additional setup tx or MarketSeed-style preseed). They count toward `tx_count` but don't externalize an LLM-Lean cycle.

**Constitutional reading**: the FC1 hard invariant LHS is `externalized_attempt_count`, which means LLM-Lean cycles ONLY (44), NOT total transactions (50). The runner's choice to pass `tx_count` is the bug. Per OBS_TBC0_FC1_INV3_THREE_BUGS_2026-05-06.md Bug 1 (Class 2 fix). Until fixed, FC1-INV1 strict reading = AMBER on individual problems where step_partial_ok is large (P05, P07, P08).

### Finding C — FC1-INV3 my extractor's "GREEN" was tautological (code-level: my extractor bug)

`scripts/fc_witness_extract.py:264-270`:
```python
capsule_anchored_estimate = max(0, at_count - l4_w - l4e_w)
constitutional_lhs = at_count
constitutional_rhs = l4_w + l4e_w + capsule_anchored_estimate
nodes["FC1-INV3_count_equality_constitutional"] = {
    "status": "✅" if constitutional_lhs == constitutional_rhs else "🔴 code_bug",
    ...
}
```

`capsule_anchored = at - l4 - l4e` then `rhs = l4 + l4e + capsule_anchored = at`. **This is at == at, ALWAYS TRUE**. The "GREEN" in 4/9 cases was construction artifact. The remaining 5/9 RED only fired when `at - l4 - l4e < 0` (i.e., `at < l4 + l4e`), which is a different bug (synthetic L4.E gate inflating l4e — Bug 2).

**Real check needed**: each AttemptTelemetry record carries `attempt_chain_root` (per TB-18R R1 schema, Design B). The independent `capsule_anchored_attempt_count` is the count of AT records whose `attempt_chain_root` resolves to a "capsule-anchor" L4 entry (NOT a Work entry). This requires Bug 3 fix (Class 4 STEP_B field addition to `ChainDerivedRunFacts`). Until then FC1-INV3 strict reading = AMBER.

### Finding D — FC1-INV6 has a REAL tamper-detection hole (REAL VIOLATION)

`audit_tape_tamper` runs on P05's tape (after rebuilding the binary at TB-C0 round-4 substrate):

| Problem | flip_l4_byte | flip_cas_byte | truncate_l4_ref | total |
|---------|---|---|---|---|
| P01 mathd_algebra_107 (1-shot) | ✓ | ✓ | ✓ | 3/3 |
| P03 mathd_algebra_141 (1-shot) | ✓ | ✓ | ✓ | 3/3 |
| **P05 mathd_algebra_114 (medium MaxTxExhausted, 8 step_partial_ok)** | ✓ | **✗** | ✓ | **2/3** |
| P09 aime_1984_p1 (10-shot omega) | ✓ | ✓ | ✓ | 3/3 |

**P05 `flip_cas_byte` slips past undetected**. The "destructively zeroed back half (134 bytes) of largest CAS object" produces a tampered tape that audit_tape STILL declares PROCEED. This is an Art. 0.3 immutability violation: CAS data integrity is not fully audited.

The likely culprit: P05 has an EvidenceCapsule + 20 AttemptTelemetry + 20 LeanResult + ProposalPayloads. The "largest CAS object" is probably the EvidenceCapsule or a CompressedRunLog. Its CID is referenced from L4 (terminal_summary_evidence_capsule assertion id 27 PASSES on the original), but the assertion verifies CID consistency, NOT the bytes-content of the referenced object. So zeroing out the tail of that object doesn't break CID consistency — the audit doesn't fetch and re-hash the bytes against the CID.

**Forward action**: file new OBS for FC1-INV6 hole, classify Class 3 (audit-side fix in audit_tape Layer G — add bytes-content re-hash assertion).

### Finding E — Layer G "no Markov capsule" Skips reflect single-session run (no FC3-INV1 integrity check on this batch)

audit_tape's Layer G has 4 markov_*_recompute assertions, all Skipped on all 9 problems with detail "no Markov capsule". This is correct behavior for a single-session run — there's no PRIOR Markov capsule to recompute against.

**Strict reading**: capsule integrity FC3-INV1 is asserted by FILE PRESENCE only (3/9 problems have an EvidenceCapsule CAS object). The actual integrity check (regenerate-capsule-from-L4+CAS-and-match-CID) was NOT exercised. To exercise it, run a continuation: pass `--prior-chain-runtime-repo` from a previous run. Doable within MiniF2F (no new problem set needed); just sequence two runs.

### Finding F — `audit_tape` (canonical) reports PROCEED on ALL 9 problems, with no Halts and no Fails

This survives strict audit. Layer A (constitution / pubkey identity), Layer B (chain integrity), Layer C (replay determinism), Layer D (economy conservation), Layer E (predicates + L4.E rejection class), Layer F (private-detail shielding), Layer G (CAS retrievability + schema) — 38/49 PASS, 11/49 Skipped (each Skip with explicit reason: no challenge, no boltzmann diversity, no markov capsule, no tamper exercise — all run-shape consequences not bugs).

The 38 Pass assertions ARE real constitutional witnesses on real MiniF2F tape. They carry forward.

---

## §2 Per-FC-node strict status (full table)

Legend:
- ✅ GREEN — empirical tape witness; real assertion ran and passed
- 🟡 AMBER — partial witness OR structural-only OR Bug-blocked OR Skipped
- 🔴 RED — empirical violation observed
- 🚫 STRUCTURAL — by design no chain witness possible; verified by source-grep test

### §2.1 FC1 — runtime loop

| Node | Strict status | Witness on actual tape | Honest issue |
|------|---------------|------------------------|--------------|
| FC1-N1 q_state carrier | ✅ | initial_q_state.json present 9/9 | — |
| FC1-N2 q_t slice | ✅ | rejection records carry parent_state_root | — |
| FC1-N3 HEAD_t pointer | ✅ | head_state_root_hex populated 9/9 | — |
| FC1-N4 q1 after δ | ✅ | l4_count > 0 in 6/9 omega-success | — |
| FC1-N5 rtool | ✅ | agent_audit_trail.jsonl records 9/9 | — |
| FC1-N7 δ AI call | ✅ | AT CAS objects 1-50 per problem | — |
| FC1-N11 predicates | ✅ | LeanResult CAS objects 1-50 per problem | — |
| FC1-N13 wtool | ✅ | L4 + L4.E entries 9/9 | — |
| FC1-N15 reject branch | ✅ | rejection records on L4.E (12, 47, 6, 7, 13...) | — |
| FC1-INV1 every-attempt-tape-visible | 🟡 | 8/9 architect_inv1.match=True | P08 fails (Bug 1 — runner uses tx_count vs LLM-cycle count) |
| FC1-INV2 predicate routing pass→L4 / fail→L4.E | ✅ | 6/9 omega → L4 work=1; rejected → L4.E exclusively | — |
| FC1-INV3 count-equality 3-term | 🟡 | extractor formula tautological; real 3-term needs Bug 3 fix | extractor bug + Bug 3 (Class 4 STEP_B schema) |
| FC1-INV4 no legacy authoritative append | ✅ | structural source-grep PASS; runtime confirmed by audit_tape Layer B chain integrity | — |
| FC1-INV5 dashboard not source | ✅ | replay_state_root_matches_head + replay_idempotent_across_calls Pass 9/9 | — (real-load smoke pending; structurally GREEN) |
| FC1-INV6 no fake accepted nodes | 🔴 | audit_tape_tamper on P05 detects 2/3 (flip_cas_byte slips past) | NEW OBS needed: audit_tape Layer G bytes-content re-hash missing |

### §2.2 FC2 — boot

| Node | Strict status | Witness on actual tape | Honest issue |
|------|---------------|------------------------|--------------|
| FC2-N16 InitAI | ✅ | genesis_report.json present 9/9 | — |
| FC2-N18 constitution ground truth | ✅ | constitution_hash_hex == eec69545... 9/9 | — |
| FC2-N21 Q_0 minted | ✅ | initial_q_state has economic_state ledger | — |
| FC2-N22 HALT | ✅ | terminal_halt_class set in chain_invariant 9/9 | — |
| FC2-INV1 genesis replayable | ✅ | replay_state_root_matches_head Pass 9/9 | — |
| FC2-INV4 TaskOpen/EscrowLock chain events | ✅ | tx_kind_counts: task_open=1, escrow_lock=1 in 9/9 | — |
| FC2-INV6 system pubkeys verify | ✅ | pinned_pubkeys.json present + pinned_pubkey_loaded Pass 9/9 | — |
| FC2-INV7 agent registry resolves | ✅ | agent_pubkeys.json + chain_agent_ids_sandbox_prefixed Pass 9/9 | — |

### §2.3 FC3 — meta

| Node | Strict status | Witness on actual tape | Honest issue |
|------|---------------|------------------------|--------------|
| FC3-INV1 capsule derived | 🟡 | EvidenceCapsule present 3/9; markov_*_recompute SKIPPED on all 9 (no prior capsule chain) | needs continuation run with --prior-chain-runtime-repo to exercise integrity |
| FC3-INV2 no global Markov pointer | ✅ | filesystem invariant — `LATEST_MARKOV_CAPSULE.txt` absent | — |
| FC3-INV3 raw logs shielded | 🚫 | structural source-grep PASS; runtime prompt-construction not asserted | could strengthen with runtime instrumentation |
| FC3-INV5 deep history requires override | 🚫 | structural env-var grep PASS | could strengthen with runtime integration test |
| FC3-INV7 architect propose-only | 🚫 | directory existence + 24 directives in handover/directives/ | inherently structural (architect role is procedural) |
| FC3-INV8 judge veto-only | 🚫 | directory existence + N audit reports in handover/audits/ | inherently structural (judge role is procedural) |

### §2.4 Predicate / Shielding / Economy / Tape canonical (gate categories)

These 26 tests are mostly source-grep / structural. On strict audit:

- **Economy gate (9 tests)**: structural source-grep all PASS + audit_tape Layer D `no_post_init_mint`, `total_supply_conserved`, `complete_set_min_balanced` — REAL on-tape PASS 9/9. ✅
- **Predicate gate (5 tests)**: structural type-shape; combined with audit_tape Layer E `accepted_work_predicate_results_true` PASS 9/9. ✅
- **Shielding gate (5 tests)**: structural source-grep + audit_tape Layer F `projection_no_autopsy_bytes`, `typical_error_summary_no_private_detail` PASS 9/9. ✅
- **Tape canonical gate (7 tests)**: structural + audit_tape Layer B `l4_hash_chain_valid`, `l4e_chain_integrity`, `payload_cid_resolves` PASS 9/9. ✅

---

## §3 What real existing problems would close the remaining gaps?

Per `feedback_real_problems_not_designed`:

| Gap | Remediation path | Real problem (existing, not designed) |
|-----|------------------|----------------------------------------|
| FC1-INV1 P08 anomaly | **Code fix** — Bug 1 (Class 2): runner derive `expected_completed_attempts` from `tool_dist.step` not `tx_count`. NOT a problem-set issue. | n/a — bug to fix |
| FC1-INV3 tautological extractor | **Code fix** — Bug 3 (Class 4 STEP_B): add `capsule_anchored_attempt_count` field to `ChainDerivedRunFacts`. Then re-run on any MiniF2F batch with step_partial_ok > 0 (P05, P07, P08, P09 already qualify). | re-use existing batch |
| FC1-INV6 flip_cas_byte hole on P05 | **Code fix** — new OBS / new TB: audit_tape Layer G should add an assertion that re-hashes CAS object bytes against stored CID. Class 3 (audit-side, no schema bump). | n/a — bug to fix; once fixed, re-run audit_tape_tamper on P05 to confirm 3/3 |
| FC3-INV1 capsule integrity | **Real-problem path**: run any MiniF2F problem in CONTINUATION mode (`--prior-chain-runtime-repo` flag from `audit_tape_tamper` / `audit_tape`) so a Markov capsule chain exists. The 4 markov_*_recompute assertions then run. | re-use the same batch's runtime_repos as prior_chain on a second run; e.g., `mathd_numbertheory_1124` (which has CAS-resident EvidenceCapsule from this batch) → run any MiniF2F problem with `--prior-chain-runtime-repo` pointing at it |
| FC3-INV3 raw logs runtime check | **Optional strengthening** — add integration test that builds an UniverseSnapshot + agent prompt, asserts no Lean stderr substring is present | test design, not problem-set |
| FC3-INV5 deep history runtime check | **Optional strengthening** — integration test that calls deep-history reader without `TURINGOS_MARKOV_OVERRIDE=1`, asserts Default returned | test design, not problem-set |
| FC3-INV7 / FC3-INV8 | **Inherently structural**. No runtime witness possible — these are procedural facts about meta-architectural roles. | n/a |

---

## §4 Strict aggregate vs round-4 aggregate

| | Round 4 aggregate | Strict audit |
|--|-------------------|--------------|
| GREEN | 21 | **17** |
| AMBER | 4 | **7** (added FC1-INV1 + FC1-INV3 + FC3-INV1 from "GREEN-by-leniency"; kept 4 structural-only) |
| RED | 0 | **1** (FC1-INV6 flip_cas_byte hole on P05) |
| GAP | 0 | 0 |

**Total nodes**: 25 — same.

The strict audit DOWNGRADES TB-C0 from "21/25 GREEN" to "17/25 GREEN + 1 real RED + 7 AMBER (4 structural + 3 bug-blocked)". This is a more honest baseline.

---

## §5 Recommended forward actions

1. **File new OBS for FC1-INV6 flip_cas_byte hole** (Class 3, audit-side; not a runtime bug, an audit-coverage bug)
2. **Fix Bug 1 (Class 2)**: runner uses `tool_dist.step` count not `tx_count`. Re-run extractor on existing evidence (no new LLM compute needed). Expected: FC1-INV1 → 9/9 GREEN.
3. **Fix Bug 3 (Class 4 STEP_B)**: add `capsule_anchored_attempt_count` field. Architect ratification + parallel-branch protocol. Then re-run extractor.
4. **Strengthen extractor**: replace tautological `capsule_anchored = AT - l4 - l4e` with INDEPENDENT count derivation.
5. **Run continuation smoke for FC3-INV1**: chain a P10 run consuming this batch's P06 runtime_repo as `--prior-chain-runtime-repo`. Exercises markov_*_recompute Layer G assertions. ~10 min wallclock.
6. **External dual audit (Codex + Gemini)** should be invoked AFTER the above 4 items to validate the strict audit's findings independently. Per CR-C0.8 (audit-after-evidence) the remediation is the prerequisite.

---

## §6 Honest framing (no deception)

The round-4 closure report claimed "Constitution Landing Gate ready for §8 sign-off; 21/25 GREEN". Strict audit says: **the empirical tape evidence supports 17/25 chain-resident GREEN, 7 AMBER (3 of which are bug-blocked, not structural-only), and 1 RED**. The TB-C0 work is REAL — the tests exist, the harness ran, the chain is well-formed and replayable, the economy invariants hold, the predicate routing is correct, the no-global-pointer is enforced. But the closure-candidate claim was over-rosy because:

- aggregate-GREEN logic was lenient (one-pass-saves-the-day)
- one extractor invariant (FC1-INV3) was tautological by construction
- one tamper case (P05 flip_cas_byte) actually slipped past on real workload
- one integrity check (capsule regen) was never exercised on this batch

The right ship-gate decision is: **TB-C0 is NOT closure-ready until FC1-INV6 RED is closed and the bug-blocked AMBERs (FC1-INV1 P08 + FC1-INV3) have a path to GREEN**. The strict audit downgrades TB-C0 to **CLOSURE CANDIDATE — REMEDIATION NEEDED** (was: CLOSURE CANDIDATE).

Per `feedback_no_workarounds_strict_constitution`: "我不要凑活". This audit is the application of that rule to my own optimistic round-4 framing.

---

## §7 Cross-references

- TB-C0 charter: `handover/tracer_bullets/TB-C0_charter_2026-05-06.md`
- TB-C0 closure report (round-4): `handover/tracer_bullets/TB-C0_CLOSURE_REPORT_2026-05-06.md` — over-rosy in §1 / §3 / §4; this strict audit supersedes
- 3-bug OBS: `handover/alignment/OBS_TBC0_FC1_INV3_THREE_BUGS_2026-05-06.md`
- This audit: `handover/alignment/STRICT_AUDIT_TBC0_TAPE_2026-05-07.md` (THIS FILE)
- Empirical evidence: `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/`
- audit_tape_tamper raw output: `target/audit_tape_tamper/P{01,03,05,09}_*_verdict.json`
- Memory: `feedback_no_workarounds_strict_constitution`, `feedback_audit_obs_bias`, `feedback_constitutional_harness_engineering`

---

**End of strict audit. Self-audit caveat applies — external Codex + Gemini dual audit recommended for adversarial verification.**
