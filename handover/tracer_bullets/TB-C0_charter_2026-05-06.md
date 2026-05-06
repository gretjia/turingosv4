# TB-C0 — Constitution Landing Gate (charter, 2026-05-06)

**Authority**: `handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md` (architect 2026-05-06; auto-mode authorized through closure).

**Mode**: Constitutional Harness Engineering (per CLAUDE.md "PRIME OPERATING MODE", since 2026-05-06).

**Class**: Mixed
- Charter + matrix authoring = Class 0
- Test skeleton = Class 1 (additive)
- Evaluator hot-path FC1 collapse repair (if needed) = Class 3 (within TB-18R R2 territory; not new STEP_B-restricted file)
- typed_tx / sequencer / CAS schema bumps if needed = **Class 4 STEP_B** — escalate, do not bundle into TB-C0

**Phase**: P0 Constitution Landing (project meta-gate; retroactive closure of FC1/FC2/FC3 invariants as repo-side executable CI before any feature TB resumes).

**Phase tag** (per `feedback_tb_phase_tag_required`):
- `phase_id` = P0 Constitution Landing (meta-gate)
- `roadmap_exit_criteria_addressed` = constitution-as-executable-CI; FC1/FC2/FC3 hard invariants as repo gates; shifts dev-cycle from atomic-audit-ceremony → harness-first
- `kill_criteria_tested` = N→1 attempt collapse (FC1); memory-only preseed (FC2); global Markov pointer reappearance (FC3); post-init mint (Economy); raw stderr leak to agent prompt (Shielding); parallel ledger source-of-truth (Tape canonical)

---

## §1. Scope

TB-C0 turns the constitution + 3 flowcharts into:
- 1 `CONSTITUTION_EXECUTION_MATRIX.md` (clause → code → test → smoke → status → kill)
- 1 `TRACE_FLOWCHART_MATRIX.md` (FC1/FC2/FC3 node → code → test)
- 8 `tests/constitution_*.rs` integration tests
- 1 `constitution_gate_report.json` artifact producer

TB-C0 does NOT add new product features. It does NOT modify constitution.md. It does NOT redo TB-18R R1–R7 substrate work; it consumes that substrate and turns it into FC-keyed gates.

---

## §2. Functional Requirements (FR)

| ID | Requirement |
|----|-------------|
| FR-C0.1 | `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` exists, contains a row per constitution clause / FC node, and each row has `Clause / Code / Test / Smoke / Status / Kill` columns. |
| FR-C0.2 | `handover/alignment/TRACE_FLOWCHART_MATRIX.md` exists, contains a row per FC1/FC2/FC3 node from `FC_ELEMENTS_2026-04-22.md`, and ties each node to ≥1 code symbol AND ≥1 test name. |
| FR-C0.3 | `tests/constitution_fc1_runtime_loop.rs` exists with 7 named tests covering FC1: every-externalized-attempt-tape-visible, predicate-pass-goes-l4, predicate-fail-goes-l4e, no-legacy-authoritative-append, dashboard-not-source-of-truth, attempt-count-equals-tape-count, no-fake-accepted-nodes. |
| FR-C0.4 | `tests/constitution_fc2_boot.rs` exists with 8 named tests covering FC2: genesis-report-exists, on-init-only-mint, no-post-init-mint, no-memory-only-preseed, taskopen-escrowlock-are-chain-events, run-replayable-from-genesis-tape-cas, system-pubkeys-verify, agent-registry-resolves. |
| FR-C0.5 | `tests/constitution_fc3_meta.rs` exists with 8 named tests covering FC3: capsule-derived-from-tape-cas, no-global-markov-pointer, raw-logs-not-in-agent-read-view, latest-capsule-context-only, deep-history-requires-override, no-automatic-predicate-mutation, architectai-proposal-not-direct-write, judgeai-veto-only. |
| FR-C0.6 | `tests/constitution_predicate_gate.rs` exists with 5 named tests: predicate-result-is-binary, predicate-failure-cannot-enter-l4, predicate-pass-required-for-l4, lean-verified-required-for-verified-worktx, price-never-overrides-predicate. |
| FR-C0.7 | `tests/constitution_shielding_gate.rs` exists with 5 named tests: raw-lean-stderr-not-in-agent-read-view, l4e-public-summary-low-pollution, private-diagnostic-cid-not-serialized-publicly, evidence-capsule-raw-logs-audit-only, dashboard-does-not-leak-private-failure-detail. |
| FR-C0.8 | `tests/constitution_economy_gate.rs` exists with 9 named tests: economy-read-is-free, economy-write-requires-stake-or-escrow, economy-no-post-init-mint, economy-total-coin-conserved, economy-complete-set-yes-no-not-coin, economy-no-ghost-liquidity, economy-wallet-read-only-projection, economy-no-f64-money-path, system-tx-not-agent-submittable. |
| FR-C0.9 | `tests/constitution_tape_canonical_gate.rs` exists with 7 named tests: no-parallel-ledger-source-of-truth, no-shadow-tape-authoritative-parent, canonical-txid-not-shadow-id, dashboard-regenerates-from-tape-cas, chain-derived-facts-not-evaluator-stdout, all-externalized-attempts-have-cas-payload, all-lean-results-have-cas-payload. |
| FR-C0.10 | `tests/constitution_no_parallel_ledger.rs` exists with dedicated Art. 0.2 fence — tests asserting LATEST_MARKOV_CAPSULE.txt absent + no global-pointer code paths + no shadow-tape authoritative-parent code paths. |
| FR-C0.11 | A `cargo test --workspace constitution_` run succeeds with all non-`#[ignore]` tests GREEN; `#[ignore]` is permitted ONLY for tests requiring real LLM compute (P38/P49 evidence) or external state. |
| FR-C0.12 | `constitution_gate_report.json` artifact is producible (either by a `cargo run --bin constitution_gate_report` binary OR by parsing `cargo test` output) and lists per-gate-test pass/fail. |

## §3. Constitutional Requirements (CR)

| ID | Constraint |
|----|------------|
| CR-C0.1 | NO test stub uses `assert!(true)` or "covered by docs" — every test MUST be writable to fail under a constitution-violating change. |
| CR-C0.2 | NO test modifies production state, runs background services, or depends on network unless explicitly `#[ignore]`-marked with reason. |
| CR-C0.3 | NO new Class-4 typed-tx schema bump bundled in TB-C0. If a constitution gate cannot be enforced without a Class-4 surface change, file an OBS and escalate to architect. |
| CR-C0.4 | NO retroactive modification of M1 / R6 / R7 historical evidence (per `feedback_no_retroactive_evidence_rewrite`). |
| CR-C0.5 | NO Markov global pointer reintroduced (per OBS_R022 closure). The `no_global_markov_pointer` test asserts `LATEST_MARKOV_CAPSULE.txt` absent. |
| CR-C0.6 | Test functions MUST be filter-discoverable: each test name MUST start with one of `fc1_`, `fc2_`, `fc3_`, `predicate_`, `shielding_`, `economy_`, `tape_`, `system_`, `no_`, or `constitution_`. |
| CR-C0.7 | The matrix RED status MUST persist for any clause whose only "evidence" is a doc-comment or a passing audit. RED → AMBER (test exists, not yet exercising real path) → GREEN (test exercises real path AND passes). |
| CR-C0.8 | Per CLAUDE.md Audit Standard 2026-05-06: external audit dispatch (Codex + Gemini) happens AFTER MVP gates green, NOT in parallel. |
| CR-C0.9 | P38/P49 runs (when authorized) produce `constitution_gate_report.json` ARTIFACT, not "benchmark scoring" framing. Their purpose is FC1 verification, not capability comparison. |
| CR-C0.10 | NO feature TB merge accepts the workspace until all 5 MVP gates are GREEN. CI / merge-policy MUST enforce this (mechanism: CONSTITUTION_EXECUTION_MATRIX status + `cargo test --workspace constitution_`). |

## §4. Ship Gates (SG)

Each gate is a binary pass/fail. All MUST be GREEN to declare TB-C0 SHIPPED.

| ID | Gate | Verification |
|----|------|--------------|
| SG-C0.1 | `CONSTITUTION_EXECUTION_MATRIX.md` exists with ≥30 rows | `wc -l` + structural inspection |
| SG-C0.2 | `TRACE_FLOWCHART_MATRIX.md` exists, all FC1+FC2+FC3 nodes from `FC_ELEMENTS_2026-04-22.md` represented | grep node IDs |
| SG-C0.3 | All 8 `tests/constitution_*.rs` files exist | `ls tests/constitution_*.rs` |
| SG-C0.4 | All 54 named test functions present (7+8+8+5+5+9+7+5; tally per FR-C0.3..10) | `grep -E 'fn (fc[123]_|predicate_|shielding_|economy_|tape_|system_|no_|all_|canonical_|chain_|dashboard_)' tests/constitution_*.rs \| wc -l` |
| SG-C0.5 | `cargo test --workspace constitution_` GREEN for all non-ignored tests | runner output |
| SG-C0.6 | MVP-1: tx-count equality test passes on representative N>1 substrate (proxy via `tb_18r_chain_attempt_invariant.rs` + new fc1 test) | `cargo test --workspace fc1_attempt_count_equals_tape_count` |
| SG-C0.7 | MVP-2: predicate-pass→L4, predicate-fail→L4.E routing exhaustive (no orphan path) | `cargo test --workspace predicate_` |
| SG-C0.8 | MVP-3: dashboard regenerable from tape+CAS (existing `tb_16_dashboard_live_regen.rs` integrated) | `cargo test --workspace tape_dashboard_regenerates_from_tape_cas` |
| SG-C0.9 | MVP-4: replay-from-genesis test passes | `cargo test --workspace fc2_run_replayable_from_genesis_tape_cas` |
| SG-C0.10 | MVP-5: all 9 economy_gate tests GREEN | `cargo test --workspace economy_` |
| SG-C0.11 | `constitution_gate_report.json` produced and lists ≥54 test rows | check artifact |
| SG-C0.12 | Workspace test count ≥ baseline + 50 (we add 54 new tests; allow 4-test slack for de-dup) | `cargo test --workspace --no-fail-fast` |
| SG-C0.13 | No `#[ignore]` tests in MVP-1, MVP-2, MVP-3, MVP-4, MVP-5 critical paths (P38/P49 evidence may be `#[ignore]` if requires LLM compute, but the structural FC1 invariant test must NOT be ignored) | grep |
| SG-C0.14 | Per `feedback_workspace_test_canonical`: report `command/workspace_count/failed/ignored` in TB-C0 ship report | inspection |

## §5. Kill criteria

Halt and escalate to architect if any of:

| ID | Condition |
|----|-----------|
| KILL-C0.1 | A constitution gate cannot be enforced without a Class-4 surface change (typed_tx schema / sequencer admission / canonical signing payload) — escalate via OBS, do NOT bundle |
| KILL-C0.2 | The N→1 collapse on FC1 cannot be repaired without modifying `src/state/typed_tx.rs` — that's STEP_B (Class 4); escalate |
| KILL-C0.3 | The `no_memory_only_preseed` test reveals existing code paths that violate FC2 — escalate per `feedback_no_workarounds_strict_constitution` (don't paper over) |
| KILL-C0.4 | An economy_gate test fails AND the failure is in a TB-13 or TB-14 already-shipped surface — escalate as constitutional drift, do NOT silently fix in TB-C0 |
| KILL-C0.5 | `cargo test --workspace constitution_` shows >5 RED tests after 1 cycle of repair — stop, write an OBS catalog of all REDs, escalate |

## §6. Out of scope (explicit non-goals)

- Adding new typed-tx variants
- Modifying sequencer admission semantics
- Modifying CAS schema
- Implementing dashboard DAG render (deferred per OBS_R5)
- Running real P38/P49/M0 LLM compute (separate authorization required; TB-C0 wires the gate; running it is a follow-up)
- Codex / Gemini external audit (happens AFTER MVP gates GREEN)
- Modifying constitution.md
- Modifying handover/architect-insights/* (architect-only)

## §7. Sequencing

```
R0  charter ratification (this file)               [Class 0]
R1  matrices + skeleton tests created              [Class 0+1]
    - CONSTITUTION_EXECUTION_MATRIX.md
    - TRACE_FLOWCHART_MATRIX.md
    - 8x tests/constitution_*.rs (all 54 test fns)
R2  RED→GREEN passes:                              [Class 1]
    - Static-analysis tests (code-grep, fs-check) — should mostly GREEN out of the box
    - Sequencer / chain integration tests — reuse TB-13/14/15/16/17/18R helpers
    - #[ignore]-mark P38/P49 tests pending evidence run
R3  cargo test --workspace constitution_           [verification]
R4  constitution_gate_report.json producer         [Class 1]
R5  TB-C0 ship report + commit                     [Class 0]
R6  external audit dispatch (Codex + Gemini)       [forward; per CR-C0.8 AFTER R3 green]
R7  architect §8 sign-off                          [forward]
```

R6 / R7 are forward-bound. R0–R5 are this session's scope.

## §8. Architect sign-off (forward)

TB-C0 ships FINAL only after:
1. All SG-C0.1..14 GREEN
2. `cargo test --workspace constitution_` clean
3. Codex + Gemini external audit PASS (or VETO < CHALLENGE; conservative-resolution rule)
4. Explicit architect §8 sign-off

This charter does NOT itself constitute §8.

---

## §9. Forward bindings

After TB-C0 closes:
- TB-18R closes via final dual audit + §8 (Phase 3 P38/P49/M0 run becomes a constitution-gate evidence run)
- TB-19+ (NodeMarket / Polymarket / public-chain / real-world readiness) becomes eligible — but each must show its constitution-gate row(s) in the matrix BEFORE merge
- The hard freeze list in `project_tb_c0_charter.md` lifts on TB-C0 SHIPPED FINAL

## §10. Memory cross-refs

- Operating mode: `feedback_constitutional_harness_engineering`
- Tape-first rule: `feedback_tape_first_real_tests`
- FC-first problem handling: `feedback_fc_first_problem_handling`
- No workarounds: `feedback_no_workarounds_strict_constitution`
- No retroactive evidence: `feedback_no_retroactive_evidence_rewrite`
- Audit after evidence: `feedback_audit_after_evidence`
- Dual audit: `feedback_dual_audit`
- Class-4 escalation: `feedback_class4_cannot_hide_in_class3`
- Workspace test canonical: `feedback_workspace_test_canonical`
- TB phase tag required: `feedback_tb_phase_tag_required`
