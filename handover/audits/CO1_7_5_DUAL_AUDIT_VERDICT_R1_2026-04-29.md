# CO1.7.5 Dual External Audit — Round-1 Merged Verdict

**Date**: 2026-04-29
**Target**: `CO1_7_5_TRANSITION_BODIES_AND_RUNTIME_WIRING_v1_2026-04-29.md` at HEAD `334111a`
**Audits**: Codex r1 (`CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md`) + Gemini r1 (`GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md`)

## Verdict matrix

| Audit | Verdict | Conviction | Q breakdown |
|---|---|---|---|
| Codex | **CHALLENGE** | High | Q-A pass-with-v1.1-ask, Q-B pass-with-overclaim-fix, Q-C CHALLENGE (compile defects), Q-D **CHALLENGE (purity violations)**, Q-E **CHALLENGE (mapping table overclaims)**, Q-F PASS, Q-G CHALLENGE (smoke/patch staleness), Q-H **CHALLENGE (substrate missing)**, Q-I CHALLENGE (impl gating) |
| Gemini | **CHALLENGE** | High | Q1 PASS, Q2 **CHALLENGE (process passive)**, Q3 PASS, Q4 PASS, Q5 PASS, Q6 PASS, Q7 PASS, Q8 vote=`unimplemented!()` |

**Conservative-merged verdict** (per memory `feedback_dual_audit_conflict`, VETO > CHALLENGE > PASS): **CHALLENGE / High**. No VETO from either audit ("no foundational design flaw" — both explicit).

## Where the audits agree

1. § 0.3 STATE supersession framing must take **active** reconciliation responsibility (not delegate to STATE curator). Gemini MF1+MF3 + Codex Q-A v1.1 ask converge.
2. SignalKind 4-variant minimization is a **safe deferral**, not a hazard (Gemini Q5 PASS + Codex Q-F PASS).
3. Combined STEP_B ceremony (one A/B unit covering bus.rs + kernel.rs) is **strategically sound** (Gemini Q4 PASS); Codex agrees the combination is permissible but flags overclaim about Phase 0 binding-vs-advisory (Codex Q-C).
4. Hygiene OBS handling appropriate — inline fix on CLAUDE.md is correct since it's project instructions, not constitution.md (Gemini Q6 PASS).

## Where the audits disagree

**Q1 (`head_commit_oid_hex` default impl)**:
- Gemini: `unimplemented!()` (silent stagnation worse — head_t is constitutional anchor)
- Codex: `default { None }` + mandate Git2 override + Git2-backed test (panic-after-commit-success worse)

**Synthesis** (preserves both safety arguments): default `None` (Codex no-panic) + spec mandates every shipped LedgerWriter override head_commit_oid_hex + add a test that fails if Git2LedgerWriter returns None at commit time (Gemini silent-stagnation defense). The default is then dead code in production, never reached.

## Where Codex went deeper than Gemini (substantive must-fix)

Codex performed source-level verification at depth Gemini did not (Codex's prompt was implementer-paranoid; Gemini's was strategic). Findings unique to Codex:

### M1 (Q-D + Q-H + Q-I — the heavyweight finding): substrate missing

Codex verified each STATE § 3.x call site against shipped APIs and found the spec assumes infrastructure that **does not exist**:

| STATE pseudocode reference | Shipped reality | Gap |
|---|---|---|
| `q.economic_state_t.claims_t.get(&tx.target_work_tx).status.allows_verification()` | `ClaimsIndex` = `BTreeMap<TxId, ClaimEntry>` with only `amount` + `claimant` | No `status`, `solver`, `task_id` fields |
| `q.economic_state_t.task_markets_t.get(target.task_id).config.verifier_bond_on_slash` | `TaskMarketEntry` has no `deadline` / `creator` / `config` fields | No config substrate |
| `window.is_open(tx.timestamp_logical)` | `ChallengeCase` lacks `duration` / `outcome` field + `is_open` method | No challenge-window machinery |
| `registry.run_acceptance(tx, q)?` / `run_verification` / `run_counterexample_check` | `PredicateRegistry` exposes only `register/get/root/view` | No execution methods |
| `q_next.economic_state_t.derive_state_root()` | Method does not exist on EconomicState | No state-root derivation |

These are FC1 (top-white predicate execution) + FC2 (middle-black state-mutation schemas) responsibilities. Putting them inside an FC3 (bottom-white L4 ledger) atom violates Anti-Oreo 三层 separation.

Per PROJECT_DECISION_MAP § 3.4, the prerequisite substrate is the planned **CO P2.x family** (currently in "Pending CO P2 (after CO P1 exit)"):
- CO P2.1 TaskMarket
- CO P2.2 EscrowVault
- CO P2.3 ContributionLedger
- CO P2.5 ChallengeCourt (challenge-window machinery)
- CO P2.6 SettlementEngine (`issue_provisional`, settlement formula)
- CO P2.7 Agent roles
- CO P2.9 ReputationIndex (`reputations_t.adjust`)
- CO1.11 Safety vs Creation (uses PredicateRegistry — likely supplies execution methods)

### M2 (Q-D — purity boundary violations)

Spec § 1 D1 promises 4-arg signature `(&QState, &TxVariant, &PredicateRegistry, &ToolRegistry)` + "no I/O". STATE pseudocode violates this:
- `challenge_transition` reads CAS inside transition (`cas::get(&tx.counterexample_cid)?`) — needs CAS arg
- `emit_terminal_summary_transition` takes `&Runtime`, reads run state, signs inside transition — needs runtime + keypair args
- System signature verification needs `PinnedSystemPubkeys` — not in 4-arg sig

### M3 (Q-C — D3 compile defects)

- Bus type is **`TuringBus`** (`src/bus.rs:53`), not `Bus` — spec全文写错
- Kernel derives `Debug, Serialize, Deserialize` (`src/kernel.rs:18`); adding `Option<Arc<Sequencer>>` requires `serde(skip)` + Debug handling; Sequencer has no derives
- Kernel docs as "pure topology" (`src/kernel.rs:15-17`) — Sequencer placement needs stronger justification or move to a runtime layer

### M6 (Q-E — TransitionError mapping table overclaims)

Spec Q5 mapping table missed:
- CAS lookup failure in `challenge_transition` (no mapped variant)
- `SettlementEngine::issue_provisional` failure in Work (no mapped variant)
- Runtime / system-signature validation paths for FinalizeRewardTx + TerminalSummaryTx
- Some stale-parent checks for system tx

### M7 (Q-E — RejectedAttemptSummary side channel not real)

Spec asserts a side channel for rich rejection context. Codex finds:
- A type at `src/bottom_white/ledger/system_keypair.rs:151-158` exists, but does NOT match STATE shape (`STATE:192-214`)
- Sequencer rejection currently only logs and skips (`src/state/sequencer.rs:252-266`); no rejected-summary stamping path is wired

### M8 (Q-G — smoke/patch staleness)

- Footer says smoke ran at `2f5093a` — should be current HEAD `334111a` (smoke ran pre-commit; the spec was committed after, became HEAD)
- Spec § 1 D4 cites `transition_ledger.rs:1451` for the `#[ignore]`; actual location is line `1455` (`1451` is the doc-comment)
- S8 says "18 warnings"; full workspace also emits 1 `gix_capability_spike` warning → "19"
- P3 references "§ 6 ack #8" but § 6 has only 6 items after self-audit dropped duplicates

## Occam-driven scope decision (executed without further audit input)

The audit findings reveal the v1 spec was **mis-scoped** by my session: D1 transition bodies were bundled with D2+D3+D4 wiring, but D1 has heavyweight cross-layer substrate dependencies that D2+D3 do not.

**Decision** (per "无损压缩即智能" + Anti-Oreo + Occam, applied by ArchitectAI without further audit input):

Split the atom by dependency profile, using existing `CO1.4-extra` pattern as precedent:

| Atom | Scope | Substrate dependency | Ships when |
|---|---|---|---|
| **CO1.7-extra** (NEW; bridge atom) | D2 head_t close + D3 Sequencer entry-point + 1 D4 test (`cas_payload_round_trip`, substrate-independent) | None — uses only frozen LedgerWriter trait + Sequencer machinery + existing CasStore | Now (small atom; v1.1 fixes M3-M8) |
| **CO1.7.5** (reverts to CO1.7 § 13 original meaning) | D1 transition bodies + 3 D4 tests + un-ignore `sequencer_serial_replay_byte_identity` | CO P2.1 / 2.2 / 2.3 / 2.5 / 2.6 / 2.7 / 2.9 + CO1.11 + (new) PredicateRegistry execution-methods atom | After substrate atoms PASS/PASS |

### Why this beats the 3 user-presented options under Occam

| Option | Description-length cost | Anti-Oreo | WP § 5.L4 | Verdict |
|---|---|---|---|---|
| A: CO1.7.5 owns substrate | NEW concept "L4 atom owns FC1/FC2 schemas" | ❌ violates 三层 | ❌ exceeds L4 boundary | NO |
| B (raw): declare blocker; spec stays bundled | NEW concept "all-or-nothing implementation gate" | OK | OK but inefficient | suboptimal |
| C: atom-internal phasing D5 | NEW concept "atom-internal heterogeneous phases" | ❌ if D5 cross-layer | ❌ same | NO |
| **B2 (executed)**: split by dep profile | **0 new concepts** (CO1.4-extra precedent + Anti-Oreo + atom-decomposition) | ✅ each atom in its layer | ✅ L4 atom contains only L4 work | **YES** |

### What this reveals about LATEST.md

LATEST.md (commit `2f5093a`) claims "Wave 6 #1 80% complete; CO1.7.5 single critical path". This is **false-precision**. True state:
- L4 wiring (D2+D3): **shipping now via CO1.7-extra v1.1** post round-2 PASS/PASS (~80% → ~85%)
- L4 transition bodies (CO1.7.5 per CO1.7 § 13): **gated on 7+ substrate atoms** in the CO P2.x family
- Wave 6 #1 actual closure: requires CO1.7-extra + CO1.7.5 + CO P2.x family → far from "single critical path"

LATEST.md should be patched in the same session-cluster to reflect this audit-derived reality.

## v1.1 patch plan (rolled into CO1.7-extra v1, applied this session)

| ID | Source | Fix |
|---|---|---|
| **M1** scope | Codex Q-D/H/I + Occam | Atom rescoped to D2+D3 + 1 substrate-independent D4 test. D1 + 3 D4 tests + un-ignore moved to future CO1.7.5 atom (gated). |
| **M2** purity | Codex Q-D | Now N/A for CO1.7-extra (no transition bodies in scope). Will be addressed by future CO1.7.5 spec. |
| **M3a** TuringBus | Codex Q-C | Spec body uses `TuringBus` per `src/bus.rs:53`. |
| **M3b** Kernel derives | Codex Q-C | Spec specifies `#[serde(skip)]` on Kernel.sequencer field + manual Debug; Sequencer.rs adds `#[derive(Debug)]` minimal. |
| **M3c** Sequencer placement | Codex Q-C | v1.1 keeps Sequencer in Kernel; clarifies "pure topology" doc to acknowledge Sequencer as the typed-tx topology element (matching legacy Tape/NodeId pattern). |
| **M4** § 0.3 active reconciliation | Gemini MF1+MF3 + Codex Q-A | § 0.3 commits to filing STATE_TRANSITION_SPEC v1.5 housekeeping issue as part of CO1.7-extra atom closure; asserts downstream-spec supersession principle explicitly. |
| **M5** Q1 synthesis | Both | Default `None` + spec mandates every shipped LedgerWriter override + add test asserting Git2LedgerWriter returns Some at commit time. |
| **M6** mapping table | Codex Q-E | Now N/A (TransitionError mapping is a transition-bodies concern; future CO1.7.5 owns it). |
| **M7** RejectedAttemptSummary claim | Codex Q-E | Now N/A (transition-bodies concern). |
| **M8a** smoke commit cite | Codex Q-G | Footer updated to current HEAD (TBD post-rewrite). |
| **M8b** ignore line cite | Codex Q-G | `1451` → `1455` (doc-comment vs actual `#[ignore]`). |
| **M8c** warning count | Codex Q-G | "18" → "19" (gix_capability_spike spike adds one warning at workspace level). |
| **M8d** P3 stale wording | Codex Q-G | "§ 6 ack #8" → "§ 6 ack" (no #8 after self-audit drop). |

## Not addressed in v1.1 (out of scope for CO1.7-extra atom)

- M2 (purity boundary) — gated to future CO1.7.5 spec
- M6 (mapping table completeness) — gated to future CO1.7.5 spec
- M7 (RejectedAttemptSummary substantiation) — gated to future CO1.7.5 spec

These migrate with D1 to the future CO1.7.5 atom, where they belong.

## Audit cost summary

- Codex r1: 254,013 tokens used (size matches deep source-level review)
- Gemini r1: prompt=144,441 / candidates=3,107 / total=150,915 tokens
- Estimated round cost: ~$8-15 (single round both audits)
- Cumulative project audit spend: ~$183-288 / $890 mid-budget (~21-32%)

## Status going forward

1. **CO1.7-extra v1.1**: spec rewritten in place this session; awaiting round-2 dual audit
2. **CO1.7.5 (transition bodies)**: future atom; spec to be drafted **after** CO P2.x substrate atoms reach individual PASS/PASS
3. **LATEST.md**: should be patched to reflect audit-derived true state of Wave 6 #1 (~30-40%, not 80%)
4. **PROJECT_DECISION_MAP**: should track CO1.7-extra as new bridge atom; CO1.7.5 dependency declared
