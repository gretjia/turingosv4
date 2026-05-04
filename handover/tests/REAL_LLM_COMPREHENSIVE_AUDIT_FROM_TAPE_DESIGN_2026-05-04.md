# Real-LLM Comprehensive ChainTape — Audit-From-Tape Test Design — 2026-05-04

**Status**: DESIGN (not yet implemented).
**Authority**: user request 2026-05-04 ("design comprehensive real LLM chaintape test on all TB features so far. the audit need to be done from the tape it produced").
**Naming**: this design IS the proposed implementation of **TB-16 Controlled Market Smoke Arena** per architect §7 (`handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`). Architect spec specifies "compute + position + complete set + price + mask + autopsy" minimum scope; this design extends to **all 13 TB-shipped tx types** and adds the **audit-from-tape contract** as the load-bearing acceptance gate.
**Phase**: P4 + P5 v0 prep (anchors all phases since P1 chain integrity is the audit-from-tape contract).
**Risk class**: **Class 3 integration smoke** (architect §7.7 verbatim). External audit required at ship.

---

## §1 One-line goal

```text
Drive a single real-LLM evaluator run that exercises EVERY shipped tx
type (TB-1..TB-15) on a live multi-agent Lean-proof market. Persist
the run as a chain-backed ChainTape (Sequencer::apply_one + on-disk
LedgerEntry chain + L4.E rejection ledger + CAS objects). Then run a
SEPARATE audit binary (`audit_tape`) that reads ONLY the persisted
artifacts (runtime_repo + cas_dir + bootstrap files) and emits a
verdict over 38 enumerated assertions covering chain integrity,
replay determinism, monetary invariants, predicate fidelity, privacy
contracts, Markov continuity, tamper detection, and dashboard
regenerability — proving the system's own evidence is sufficient to
re-derive every shipped invariant without consulting the running
process.
```

---

## §2 Why "audit from the tape"

Per `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` P1 Exit-8: *"state.db 删除后，可以从 L4 (accepted only) 重建 — L4.E 不参与 state_root 重建"*. This test elevates that exit criterion from a kernel unit test to a system-level acceptance gate: the auditor must reach the same conclusions as the live sequencer using only the on-disk evidence.

Per `feedback_smoke_evidence_naming` (binding 2026-05-01 D5): only chain-backed production runs may be called "ChainTape smoke / smoke tape / tape". Pre-TB-6 stdout-only paper trail is "smoke evidence" — not eligible for this gate.

Per `feedback_o1_chain_on_auditability`: state facts → L4; rejected tx → L4.E; high-dim evidence → CAS; failure anchored via system-emitted RunExhausted/Bankruptcy/Expire. This test exercises ALL THREE substrates simultaneously and demands the auditor reconstruct every invariant from them.

---

## §3 Coverage matrix — every shipped TB feature gets a tx in the tape

| TB | Feature | Tx / Object | Driver | Gate it discharges |
|---|---|---|---|---|
| TB-1 | Monetary invariant | (genesis on_init) | bootstrap | total_supply_micro frozen at on_init |
| TB-1 | L4 hash chain | every accepted tx | sequencer | hash chain replay-deterministic |
| TB-1 | L4.E rejection chain | every rejected tx | sequencer | L4.E hash chain valid; not in state_root |
| TB-2 | WorkTx admission | `WorkTx` | solver agents (LLM) | Sequencer::submit + apply_one accept |
| TB-3 | Task escrow / RSP-1 | `TaskOpenTx` + `EscrowLockTx` | sponsor agents | task_markets_t.total_escrow funded; cache=truth |
| TB-3 | Stake lock-on-accept | `WorkTx.stake` field | solver | balances_t debit + stakes_t credit |
| TB-4 | Verifier bond | `VerifyTx.bond` field | verifier agent | balances_t debit + stakes_t[verify_tx_id] credit |
| TB-4 | Challenger NO stake | `ChallengeTx.stake` field | adversarial agent | balances_t debit + challenge_cases_t entry |
| TB-5 | System-emitted gate | `ChallengeResolveTx` | sequencer (system) | Released path refunds; UpheldDeferred is marker |
| TB-6 | Production wire-up | `Git2LedgerWriter` on disk | evaluator binary | LedgerEntry chain on disk + replay-verifiable |
| TB-7 | Per-agent Ed25519 | `agent_pubkeys.json` | evaluator bootstrap | every agent tx signature verifies live + replay |
| TB-7 | Agent audit trail | `ProposalTelemetry` CAS object | sequencer post-accept | every accepted WorkTx → telemetry CID linked |
| TB-7.7 | Lean oracle attestation | `VerificationResult` CAS object | verifier | every accepted WorkTx → verified=true |
| TB-7R | Genesis report | `genesis_report.json` | bootstrap | constitution_hash + initial balances on disk |
| TB-8 | Minimal payout | `FinalizeRewardTx` | sequencer (system) | claim → finalize → balances_t credit |
| TB-10 | Preseed factory | `runtime::bootstrap` | bootstrap | 12-entry preseed sums to 30M micro |
| TB-11 | RunExhausted anchor | `TerminalSummaryTx` | sequencer (system) | failure anchored on L4 with EvidenceCapsule CID |
| TB-11 | Capital release | `TaskExpireTx` | sequencer (system) | sponsor escrow refunded post-deadline |
| TB-11 | Death certificate | `TaskBankruptcyTx` | sequencer (system) | task state → Bankrupt; >= N exhausted runs |
| TB-11 | EvidenceCapsule | CAS `EvidenceCapsule` + `EvidenceManifest` + `CompressedRunLog` | sequencer | O(1) chain / O(N) audit |
| TB-12 | NodePosition (Long) | side-effect on accepted Work | sequencer | NOT counted in total_supply (CR-12.1/2) |
| TB-12 | NodePosition (Short) | side-effect on accepted Challenge | sequencer | exposure index, not balance |
| TB-13 | CompleteSet mint | `CompleteSetMintTx` | special agent | 1 Coin → 1 YES + 1 NO; conditional_collateral_t |
| TB-13 | Market seed | `MarketSeedTx` | sponsor | provider funds, no ghost liquidity |
| TB-13 | CompleteSet redeem | `CompleteSetRedeemTx` | special agent (post-resolution) | winning side paid; min-balanced invariant |
| TB-14 | PriceIndex | derived view (`compute_price_index`) | dashboard / scheduler | "price is signal, not truth"; integer-rational |
| TB-14 | Boltzmann mask | `mask_set` on `AgentVisibleProjection` | scheduler | parent not deleted from chaintape |
| TB-14 | CanonicalNodeGraph | derived from L4 + ProposalTelemetry | sequencer | replay-deterministic edge map |
| TB-15 | Autopsy emission | side-effect on TaskBankruptcyTx | sequencer | per-staker `AgentAutopsyCapsule` Cid in `agent_autopsies_t` |
| TB-15 | TypicalErrorBroadcast | `cluster_autopsies` | end-of-run / dashboard | N≥3 cluster → public_summary surface |
| TB-15 | Markov capsule | `MarkovEvidenceCapsule` | end-of-run binary | constitution_hash + L4 + L4.E + CAS roots + previous capsule |
| TB-15 | Default-deny gate | `TURINGOS_MARKOV_OVERRIDE` | binary | deeper history denied without override |

**Coverage = 100%** of agent-signed tx types (Work / Verify / Challenge / TaskOpen / EscrowLock / CompleteSetMint / CompleteSetRedeem / MarketSeed) + 5 system-emitted tx types (FinalizeReward / ChallengeResolve / TerminalSummary / TaskExpire / TaskBankruptcy) + 6 CAS object types (ProposalPayload / ProposalTelemetry / VerificationResult / EvidenceCapsule / AgentAutopsyCapsule / MarkovEvidenceCapsule).

---

## §4 Scenario design — six tasks engineered for full coverage

```text
Bootstrap (runtime::bootstrap::default_pput_preseed_pairs):
  tb7-7-sponsor   : 24_000_000 μC  (sponsor of all Lean tasks)
  Agent_user_0    :  6_000_000 μC  (user-task sponsor for one task)
  Agent_solver_0  :    100_000 μC  (Lean solver, baseline)
  Agent_solver_1  :    100_000 μC  (Lean solver, challenger-bait)
  Agent_solver_2  :    100_000 μC  (CompleteSet operator)
  Agent_solver_3  :    100_000 μC  (adversarial: posts ChallengeTx)
  Agent_verifier_0:    100_000 μC  (independent verifier, posts VerifyTx)
  Total: 30_000_000 μC = on_init mint; assert_no_post_init_mint enforces

Tasks (sponsored by tb7-7-sponsor unless noted):
  Task A "happy_path"      : trivial Lean theorem; solver_0 finds proof;
                             verifier_0 confirms; no challenge;
                             Sequencer emits FinalizeRewardTx.
                             EXERCISES: TaskOpen + EscrowLock + Work +
                             Verify + FinalizeReward + ProposalTelemetry +
                             VerificationResult + NodePosition(Long)

  Task B "challenge_dismissed": correct proof; Agent_solver_3 challenges
                             (incorrectly); verifier_0 re-confirms;
                             Sequencer emits ChallengeResolveTx{Released};
                             challenger bond refunded.
                             EXERCISES: Work + Verify + Challenge +
                             ChallengeResolve(Released) +
                             NodePosition(ChallengeShort) +
                             cache=truth across challenge bond movement

  Task C "challenge_upheld": invalid proof (wrong Lean form);
                             Agent_solver_3 challenges (correctly);
                             verifier_0 confirms challenge; Sequencer
                             emits ChallengeResolveTx{UpheldDeferred};
                             bond preserved (slash deferred to RSP-3.2).
                             EXERCISES: ChallengeResolve(UpheldDeferred)
                             marker path; bond accumulation in
                             challenge_cases_t

  Task D "exhaustion"      : hard Lean theorem; solver_1 runs out of
                             MAX_TX without finding proof; Sequencer
                             emits TerminalSummaryTx with
                             ExhaustionReason::MaxTxExhausted +
                             EvidenceCapsule CID;
                             after N (=2) such RunExhausted, Sequencer
                             emits TaskBankruptcyTx → triggers TB-15
                             autopsy emission for solver_1's stake.
                             EXERCISES: TerminalSummary + EvidenceCapsule
                             + TaskBankruptcy + AgentAutopsyCapsule +
                             agent_autopsies_t insertion

  Task E "expiry"          : sponsor opens task; no solver picks it up
                             before deadline elapses; Sequencer emits
                             TaskExpireTx with sponsor refund.
                             EXERCISES: TaskExpire + capital release

  Task F "complete_set_market" (Agent_user_0 sponsor):
                             Agent_user_0 posts MarketSeedTx (provider
                             funds 1_000_000 μC into conditional inventory);
                             Agent_solver_2 posts CompleteSetMintTx
                             (1 Coin → 1 YES + 1 NO);
                             solver_0 finds proof for the gating Lean task;
                             FinalizeReward resolves event_id YES;
                             Agent_solver_2 posts CompleteSetRedeemTx for
                             YES side; winning side paid 1:1 against
                             collateral.
                             EXERCISES: MarketSeed + CompleteSetMint +
                             CompleteSetRedeem + ConditionalCollateral +
                             ConditionalShareBalances + MIN-balanced
                             invariant + ResolutionRef path

End-of-run (post-evaluator-exit):
  generate_markov_capsule binary fires.
  EXERCISES: Markov capsule generation + per-run MARKOV_TB-*.json
             (TB-16.x.fix 2026-05-04: global LATEST_MARKOV_CAPSULE.txt
             write removed per architect OBS_R022 Option α; capsule cid
             surfaced via per-run JSON or `--markov-capsule-cid` CLI).
```

**Why these six**: Tasks A/D give 100% solver-side coverage (success + exhaustion → bankruptcy → autopsy). Tasks B/C cover both ChallengeResolve paths (Released + UpheldDeferred). Task E covers TaskExpire. Task F covers the entire CompleteSet (TB-13) substrate. Solver_3's adversarial role plus verifier_0's independent verification cover the verifier/challenger market.

---

## §5 Real-LLM provider configuration

```text
Solver agents (Agent_solver_0..3):
  Provider:    deepseek-v4-flash thinking-off (per project_chat_over_reasoner)
  Endpoint:    src/drivers/llm_proxy.py multi-key round-robin
  Reason:      30-day arc backbone; deterministic thinking-off mode
  Concurrency: 4 (per `feedback_routines_entropy` — earn the cost)
  Token cap:   per-attempt 1024; per-task MAX_TRANSACTIONS=20

Verifier agent (Agent_verifier_0):
  Provider:    deepseek-v4-flash thinking-off (separate routing pool)
  Reason:      Independent from solver pool to avoid same-LLM correlation

Reproducibility:
  - Set TURINGOS_RUN_SEED=2026-05-04 (passes through to RunId mint)
  - Lock evaluator --schedule-seed 0
  - LLM providers seed via X-Provider-Seed header where supported
  - Pin DeepSeek model snapshot via X-DS-Model-Hash header
  - Document in handover/evidence/tb_16_*/README.md any drift observed
    per project_deepseek_drift_2026-04-24

Caps:
  Wall clock:  TURINGOS_WALL_CLOCK_CAP_MS=1800000 (30 min)
  Compute:     TURINGOS_COMPUTE_CAP_TOKENS=120000
  Cost:        TURINGOS_COST_CEILING_USD=15
```

**Lean oracle**: `experiments/minif2f_v4/src/lean4_oracle.rs` (frozen per Phase A; in trust_root). Mathlib via `lake exe cache get` (~2 min) per `feedback_lake_packages_vendored`.

---

## §6 Audit-from-tape contract — `audit_tape` binary specification

### §6.1 Inputs (the only inputs)

```text
audit_tape \
  --runtime-repo  <path/to/runtime_repo>                  (TB-6 Git2 chain)
  --cas-dir       <path/to/cas>                           (TB-6 CAS store)
  --agent-pubkeys <path/to/agent_pubkeys.json>            (TB-7)
  --pinned-pubkeys <path/to/pinned_pubkeys.json>          (TB-5)
  --genesis       <path/to/genesis_payload.toml>          (P0)
  --constitution  <path/to/constitution.md>               (P0)
  [--markov-pointer <path/to/per-run-markov-cid.txt>]     (TB-15; OPTIONAL
                                                            since TB-16.x.fix)
  [--prior-chain-runtime-repo <path>]                     (TB-16.x.fix; resolves
                                                            <path>/markov_tip.cid;
                                                            mutex with --markov-pointer)
  [--alignment-dir <path/to/handover/alignment>]          (TB-15 OBS scan)
  --out <verdict.json>
```

**TB-16.x.fix note (architect OBS_R022 Option α RATIFIED 2026-05-04)**:
the previous global `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt`
file was an Art. 0.2 parallel ledger and has been removed. Pass
`--markov-pointer` only when supplying a per-run pointer file (NOT
global); pass `--prior-chain-runtime-repo` for explicit per-runtime
inheritance (resolver reads `<path>/markov_tip.cid`); absence of both
flags ≡ genesis chain (Layer G assertions Skipped).

The auditor MUST NOT read:
- live `Sequencer` state (no shared memory)
- `state.db` (whitebox cache; auditor rebuilds from L4)
- evaluator process logs (transient; not in tape)
- `handover/ai-direct/` (live working notes; not part of tape contract)

Anything reachable from these inputs alone is in-scope; anything not reachable is OUT.

### §6.2 38 audit assertions (pass = all green)

**Layer A — bootstrap integrity** (3 assertions)
1. `constitution_hash` computed via sha256(constitution.md) matches `[constitution_root]` in genesis_payload.toml.
2. `verify_trust_root` passes — every entry in `[trust_root]` matches its file's current sha256.
3. `pinned_pubkeys.json` contains the same Ed25519 pubkey that `system_signature_of` would verify against on every system-emitted tx in the tape.

**Layer B — chain integrity** (8 assertions)
4. L4 hash chain valid: for each row r at logical_t=t, `r.parent_ledger_root == prior.resulting_ledger_root` and `append(parent, signing_digest) == r.resulting_ledger_root`.
5. L4 parent_state_root continuity: `r.parent_state_root == prior.resulting_state_root`.
6. L4.E hash chain valid: same recurrence over the rejection_evidence ledger; never advances logical_t; never advances state_root.
7. Every system-emitted tx (FinalizeReward / ChallengeResolve / TerminalSummary / TaskExpire / TaskBankruptcy) verifies against `pinned_pubkeys.json`.
8. Every agent-signed tx (Work / Verify / Challenge / TaskOpen / EscrowLock / CompleteSetMint / CompleteSetRedeem / MarketSeed) verifies against `agent_pubkeys.json`.
9. Every `tx_payload_cid` resolves to a CAS object whose canonical_decode produces a TypedTx whose `tx_kind()` matches the L4 row's `tx_kind`.
10. No agent-signed tx has tx_kind ∈ system-only set (negative — admission-control structural).
11. Genesis row (logical_t=1) has `parent_ledger_root == Hash::ZERO` and `parent_state_root == Hash::ZERO`.

**Layer C — replay determinism** (5 assertions)
12. `replay_full_transition` over L4 alone reaches the same final `state_root_t` recorded in the chain head's `resulting_state_root`.
13. `replay_full_transition` produces the same `EconomicState` object byte-for-byte (canonical encode == canonical encode).
14. For each `TaskBankruptcyTx` row, `derive_autopsies_for_bankruptcy` re-run with the row's pre-snapshot returns Cids identical to those stored in `agent_autopsies_t[event_id]`.
15. `compute_canonical_edges_at_head` re-derived from L4 + CAS-resident ProposalTelemetry produces the same map as the one bus.snapshot() would publish.
16. Replay is deterministic across runs: invoke the auditor twice, assert the two `verdict.json` outputs are byte-identical.

**Layer D — economic invariants** (6 assertions)
17. `assert_no_post_init_mint` passes for every accepted tx (no Coin minted post on_init).
18. `assert_total_ctf_conserved` holds at every L4 row (6-holding `total_supply_micro` constant equal to genesis on_init total = 30_000_000 μC).
19. `assert_complete_set_balanced` (MIN-form: `min(Σ_yes, Σ_no) == collateral`) holds at every L4 row that touched conditional_collateral_t / conditional_share_balances_t.
20. `assert_task_market_total_escrow_matches_locks` (cache=truth) holds at every L4 row.
21. `node_positions_t` total amount NOT counted in `total_supply_micro` (CR-12.1 + CR-12.2 structural — assertion is "would inclusion break #18; assert it does").
22. `conditional_share_balances_t` total NOT counted in `total_supply_micro` (CR-13.3 + SG-13.2 same shape).

**Layer E — predicate / evidence integrity** (5 assertions)
23. Every accepted WorkTx: every entry in `work.predicate_results.acceptance.*.value == true`.
24. Every accepted WorkTx: ProposalTelemetry CAS object reachable via `work.proposal_cid`; ProposalTelemetry's `verification_result_cid` resolves to a `VerificationResult{verified: true}`.
25. Every L4.E row: `rejection_class` matches the `TransitionError` that the dispatch arm would have produced from the tx (re-dispatch the rejected tx; assert error variant matches recorded class).
26. PriceIndex computed via `compute_price_index(replayed_econ)` matches what dashboard `render_section_14` would render at HEAD.
27. Every `TerminalSummaryTx.evidence_capsule_cid` resolves to a CAS-resident `EvidenceCapsule` whose `attempt_count` + `terminal_reason` match the run.

**Layer F — privacy contracts** (4 assertions; TB-15 specific)
28. AgentVisibleProjection serialization (rebuilt from `tape_view_t` in replayed QState) contains no agent_autopsies_t / AutopsyIndex / private_detail_cid / AgentAutopsyCapsule byte runs.
29. Every `AgentAutopsyCapsule.private_detail_cid` resolves to a CAS object whose creator is `sequencer-epoch-{epoch}` (system-emitted; never an agent string).
30. `cluster_autopsies` output (TypicalErrorSummary) JSON serialization contains no AgentAutopsyCapsule.private_detail_cid byte run for any input capsule.
31. AutopsyIndex value type in `q_state.rs` is `Vec<Cid>` (file-scan; halt-trigger #4 structural fence).

**Layer G — Markov continuity** (4 assertions)
32. Markov capsule's `constitution_hash == sha256(constitution.md)` (SG-15.7).
33. Markov capsule's `typical_errors == cluster_autopsies(autopsies_walked_from_CAS, 3)` (deterministic recomputation).
34. Markov capsule's `unresolved_obs == scan_unresolved_obs(--alignment-dir)` (deterministic file-system walk).
35. Markov capsule's `next_session_context_cid` resolves to a CAS-resident JSON blob whose `boot_seq` includes "DEFAULT-DENY deeper history".

**Layer H — tamper detection** (4 assertions; auditor flips bytes in temp copies)
36. Flip 1 byte in a random L4 row → re-run auditor on the tampered copy → MUST emit `TamperDetected{layer:L4}`.
37. Flip 1 byte in a random CAS object → re-run auditor → MUST emit `TamperDetected{layer:CAS, cid_mismatch}`.
38. Remove a random L4 row → re-run auditor → MUST emit `TamperDetected{layer:L4, missing_row}`.

**No-fail mode**: tamper-detection assertions are run on COPIES; the original tape is untouched. The auditor's verdict file is the proof.

### §6.3 Output (`verdict.json`)

```json
{
  "schema_version": "v1/audit_tape_verdict",
  "tape_root": {
    "l4_count":  N,
    "l4e_count": M,
    "head_state_root_hex": "...",
    "head_ledger_root_hex": "...",
    "cas_object_count": K,
    "constitution_hash_hex": "..."
  },
  "tx_kind_counts": {
    "Work": ..., "Verify": ..., "Challenge": ...,
    "TaskOpen": ..., "EscrowLock": ...,
    "CompleteSetMint": ..., "CompleteSetRedeem": ..., "MarketSeed": ...,
    "FinalizeReward": ..., "ChallengeResolve": ...,
    "TerminalSummary": ..., "TaskExpire": ..., "TaskBankruptcy": ...
  },
  "assertions": {
    "1": {"name": "constitution_hash_matches_genesis", "result": "PASS"},
    ...
    "38": {"name": "tamper_l4_remove_detected",       "result": "PASS"}
  },
  "passed": 38,
  "failed": 0,
  "feature_coverage": {
    "TB-1": "GREEN", "TB-2": "GREEN", ..., "TB-15": "GREEN"
  },
  "verdict": "PROCEED"
}
```

`verdict ∈ {PROCEED, BLOCK}`. Any failed assertion → BLOCK.

---

## §7 Pass / fail / halt criteria

### §7.1 Test PASSES if and only if
1. Evaluator binary completes within 30-min wall clock + cost ceiling
2. All 13 expected tx_kinds appear at least once in tape_root.tx_kind_counts
3. All 6 CAS object types reachable
4. `verdict.json` contains `verdict: "PROCEED"` with all 38 assertions PASS
5. Dashboard `audit_dashboard --repo <runtime_repo> --cas <cas_dir>` renders all 15 sections without missing data
6. First Markov capsule emitted; constitution_hash matches; LATEST pointer updated
7. Replay determinism: re-run auditor twice, byte-identical verdict.json

### §7.2 Test FAILS if any of
- Any of the 38 assertions reports FAIL
- A required tx_kind is missing from the tape (coverage gap)
- Replay diverges across two auditor runs

### §7.3 HALT (no-retry; per architect §7.7) if any of
- Conservation failure (#17, #18, #19, #20)
- Raw log leak (#28, #29, #30, #31)
- Price-as-truth (auditor sees price affecting predicate / L4 — re-dispatch arm reads price)
- Non-sandbox funds used (any agent_id matches a known production wallet pattern)
- Unresolved evidence gap (any L4 row references a CID not in CAS)

---

## §8 Implementation plan (informational; not in this design's scope to execute)

```text
Step 1  NEW src/bin/audit_tape.rs — auditor binary scaffolding;
        re-uses turingosv4::runtime::verify::verify_chaintape (TB-6),
        turingosv4::runtime::autopsy_capsule::{cluster_autopsies,
        derive_autopsies_for_bankruptcy} (TB-15),
        turingosv4::runtime::markov_capsule::{scan_unresolved_obs,
        sha256_of_file} (TB-15).

Step 2  NEW src/runtime/audit_assertions.rs — 38 assertion functions
        (pure-fn over auditor inputs; each returns AssertionResult).

Step 3  NEW src/bin/audit_tape_tamper.rs — tamper-detection harness
        (forks the tape into 3 temp copies, flips/removes bytes,
        re-runs audit_tape on each, asserts TamperDetected verdict).

Step 4  NEW evaluator harness in experiments/minif2f_v4/src/bin/
        — `comprehensive_arena.rs` orchestrating the 6-task scenario
        (drives evaluator binary with task list + agent assignments +
        adversarial-challenger overrides per Task B/C).

Step 5  NEW handover/tests/scripts/run_real_llm_arena.sh — CLI
        wrapper that builds + runs evaluator + audit_tape +
        generate_markov_capsule + audit_dashboard in one shot;
        emits handover/evidence/tb_16_real_llm_arena_<DATE>/.

Step 6  NEW handover/tests/scripts/audit_tape_smoke_test.sh — runs
        Step 5 then asserts verdict.json `verdict == "PROCEED"`.
        Suitable for CI gating.

Step 7  Charter ratification + dual audit (Class 3 — architect §7.7
        external audit required at ship; Codex + Gemini per
        feedback_dual_audit hybrid-by-risk-class).

Estimated effort: 4-6 atom days (Class 3 envelope; per
feedback_iteration_cap_24h 24h-to-evaluator-feedback-loop applies for
ship-path atoms; the 30-min real-LLM run is the feedback loop).
```

---

## §9 Forbidden (architect §7.6 verbatim + design-specific additions)

```text
No public chain.
No real-money market (sandbox-labeled in agent IDs + dashboard banner).
No external domain (Lean only; no medical/legal/financial).
No unbounded leverage.
No AMM trading (TB-13 forbids).
No DPMM / pro-rata.
No production user funds.

Design-specific:
No external attestation in audit verdict (audit verdict MUST be derivable
  from on-disk artifacts only; live-process state is forbidden input).
No "smoke evidence" labeling (this test produces ChainTape smoke per
  feedback_smoke_evidence_naming binding D5).
No agent-side autopsy invention (autopsies derive from sequencer-side
  derive_autopsies_for_bankruptcy ONLY; LLM self-reports are ignored
  per CR-15.3 + DECISION_LAMARCKIAN §1.2 hard prohibition B).
```

---

## §10 Halt triggers (instant stop; no round-2)

```text
H1  Any tx in the tape has invalid system_signature (pinned-pubkey
    verification fails)
H2  Any tx in the tape has invalid agent_signature
    (agent-pubkey-manifest verification fails)
H3  Replay produces a state_root_t mismatch with the chain head
H4  L4 hash chain has any broken link
H5  L4.E hash chain has any broken link
H6  L4.E entry advances logical_t or state_root_t
H7  Any L4 row references a CAS CID not present in cas_dir
H8  Any AgentAutopsyCapsule's private_detail bytes appear in serialized
    AgentVisibleProjection
H9  Any TypicalErrorSummary serialization contains a private_detail_cid
    byte run from an input capsule
H10 Markov capsule's constitution_hash != sha256(constitution.md)
H11 Markov capsule allows deep-history read without
    TURINGOS_MARKOV_OVERRIDE=1 (binary fence broken)
H12 LLM self-narrative bytes appear in any AgentAutopsyCapsule
    evidence_cids resolution path (DECISION_LAMARCKIAN §1.2 prohibition B)
H13 Any `total_supply_micro` value across L4 rows differs from on_init
    total (CTF conservation broken)
```

---

## §11 Cross-references

- Architect spec (lossless): `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §7 (TB-16 Controlled Market Smoke Arena)
- TB-15 SHIP: `handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md`
- TB-15 charter: `handover/tracer_bullets/TB-15_charter_2026-05-03.md`
- TB-14 SHIP: `handover/ai-direct/TB-14_SHIP_STATUS_2026-05-03.md`
- TB-13 SHIP: `handover/ai-direct/TB-13_SHIP_STATUS_2026-05-03.md`
- DECISION_LAMARCKIAN: `handover/alignment/DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN_2026-05-02.md`
- Existing chain-backed smokes (predecessor pattern):
  - `handover/evidence/tb_6_chaintape_smoke_2026-05-01/`
  - `handover/evidence/tb_13_chaintape_smoke_2026-05-03/`
  - `handover/evidence/tb_14_chaintape_smoke_2026-05-03/`
- Roadmap: `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` (P1 Exit-8 + P4 + P6)
- Feedback memory:
  - `feedback_smoke_evidence_naming` (binding D5 — only chain-backed = "tape")
  - `feedback_o1_chain_on_auditability` (L4 / L4.E / CAS three-substrate split)
  - `feedback_no_retroactive_evidence_rewrite` (going-forward only)
  - `feedback_dual_audit` + `feedback_risk_class_audit` (Class 3 = full dual audit at ship)
  - `feedback_kolmogorov_compression` (architect spec verbatim in §3 + §10)
  - `feedback_iteration_cap_24h` (24h-to-feedback-loop discipline)

---

## §12 What this design intentionally does NOT do

- **Implement** anything. This is design-only; charter ratification + AI-coder execution is a separate gate.
- **Slash execution** (RSP-3.2 / TB-9): not yet shipped; ChallengeResolve(UpheldDeferred) is marker-only here. Multi-site autopsy wire-in (SlashLoss / ChallengeUnsuccessful / VerifierBondLost) is correspondingly OUT of scope per TB-15 charter §1.2 (`feedback_no_retroactive_evidence_rewrite`).
- **Real-money market** or public chain (architect §7.6 forbidden).
- **Cross-org coordination** (P6 multi-org subset; not started).
- **MetaTape mutation** (P5 v1; CR-15.3 / CR-15.4 STRUCTURALLY ENFORCED — autopsy may suggest, never auto-apply).

These remain on the roadmap for future TBs.
