# TB-13 Fix Handoff — 2026-05-03 (for new session)

**Purpose**: Hand TB-13 from a session that recommended ship-with-OBS to a fresh session that will fix the 3 actually-fixable CHALLENGES, then ship.

**Read order for the new session**:
1. This document.
2. `handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md` (full rounds 1-4 history; §12.6 round-3 verdicts; §12.8 final ship-readiness table).
3. `handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md` (R3 verdict — CHALLENGE only, no VETO).
4. `handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R3.md` (R3 verdict — CHALLENGE only on Q12).
5. `handover/alignment/OBS_TB13_AUDIT_RESIDUAL_CHALLENGES_2026-05-03.md` (current state of the 6 challenges; this handoff supersedes its "carry-forward" classification for 3 of them).
6. `handover/tracer_bullets/TB-13_charter_2026-05-03.md` (charter; Q1-Q9 architect mandates).

---

## 1. Current state (frozen at commit `353aa97`)

```text
HEAD: 353aa97 TB-13 Atom 6 round-4 — doc fixes + OBS closure (Class 3)
cargo test --workspace = 789 / 0 / 150
TB-13 commits on main: 32aab27 → 353aa97 (8 commits)

External round-3 verdicts:
  Gemini : CHALLENGE / high / FIX-THEN-PROCEED  (Q12 ResolutionsIndex; explicit non-blocking)
  Codex  : CHALLENGE / high / FIX-THEN-PROCEED  (5 challenges; explicit "No VETO")
```

All enforcement gates green. Both auditors at CHALLENGE-only (no VETO).
The previous session was about to recommend Atom 7 SHIP under
ship-with-OBS for all 6 residual CHALLENGES. The user asked
"why ship not fix?", and the previous session admitted bias:
3 of the 6 CHALLENGES are cheaply fixable and one of them
(RQ3) directly contradicts a user instruction from earlier in
the conversation.

---

## 2. The bias to avoid

The previous session rationalized ship-with-OBS via:
- "Auditors said no VETO" (true but != ship-ready).
- "Policy allows OBS-threshold-3" (true but != optimal).
- "Time-to-ship efficiency" (sunk-cost rationalization).

Specifically it:
- Lumped 6 CHALLENGES of wildly different cost/severity into one
  uniform OBS bucket.
- Punted RQ5 (resolution_tx_id) by changing only docs, leaving a
  dead wire field that becomes TB-15 compatibility debt.
- OBS-deferred Q9/RQ6 (fence robustness, ~15 min fix) as if the
  cost was prohibitive.
- Reclassified RQ3 (non-empty TB-13 chaintape replay) as
  "TB-16 sandbox arena scope" despite the user's earlier explicit
  instruction "real LLM smoke with chaintape audit before sending
  to external auditor". The smoke that ran has empty TB-13 maps —
  it does not validate what the user asked it to validate.

**New session must not inherit these rationalizations**. Read the
audits first, decide independently. If you disagree with this
handoff's plan, push back instead of executing it.

---

## 3. Three CHALLENGES to fix (with concrete plans)

### 3.1 — TB13-RQ3: non-empty TB-13 chaintape replay determinism

**Codex finding**: real-LLM smoke at
`handover/evidence/tb_13_real_llm_smoke_2026-05-03/` proves
EconomicState 13-sub-field schema reconstruction with EMPTY TB-13
maps. Replay determinism for non-empty `conditional_collateral_t` /
`conditional_share_balances_t` is not directly evidenced.

**User's earlier instruction (load-bearing)**:
> "real LLM smoke with chaintape audit before sending to external auditor"

**Why current evidence is insufficient**: the LLM-driven evaluator
does not submit `CompleteSetMint` / `CompleteSetRedeem` / `MarketSeed`
tx (those are user-economic actions, not solver actions). So the
smoke's chaintape has 0 TB-13 entries, and `verify_chaintape`'s
`economic_state_reconstructed: true` indicator only proves the
13-sub-field shape round-trips with empty maps.

**Fix plan**:

Build a deterministic non-LLM smoke that:
1. Constructs a sequencer with `genesis_with_balances_and_open_task`.
2. Submits a real `CompleteSetMintTx` (signed with a real keypair, not
   `[0u8; 64]` — wire it up using `AgentKeypair::generate()` +
   `set_agent_pubkeys`).
3. Mutates QState to flip the task to Finalized (mid-test fixture).
4. Submits a real `CompleteSetRedeemTx`.
5. Persists the chaintape to disk via Git2LedgerWriter.
6. Runs `verify_chaintape` against the persisted runtime_repo + cas.
7. Asserts `economic_state_reconstructed: true` AND
   `conditional_collateral_t` / `conditional_share_balances_t`
   reconstruct bit-equal to the live state.

**Suggested location**: `tests/tb_13_chaintape_smoke.rs` (new file)
OR extend `tests/tb_13_complete_set.rs` with a `tb13_chaintape_replay_*`
test.

**Estimated cost**: ~30-45 min.

**Output evidence**: write a smoke evidence dir at
`handover/evidence/tb_13_chaintape_smoke_2026-05-03/` with
README + replay_report.json proving non-empty TB-13 replay
determinism.

**Validation**: `verify_chaintape` 7/7 GREEN AND post-replay
`conditional_collateral_t.0.len() > 0` AND
`conditional_share_balances_t.0.len() > 0`.

---

### 3.2 — TB13-Q9/RQ6: forward-fence discovery edge case

**Codex finding**: `discover_tb_13_files()` walks `src/` for files
containing TB-13 authoring markers. A NEW TB-13 contributing file
that does NOT carry a TB-13 authoring marker AND lives outside
`FENCE_SCOPE_FLOOR` would bypass discovery.

**Why the current implementation is weak**: marker discipline is a
human-followed convention. The fence's contract is "TB-13 code = TB-13
marker", but a real attack vector / regression vector is an unmarked
file that imports legacy CPMM types.

**Fix plan**: extend `discover_tb_13_files()` to also catch files
that USE TB-13 types regardless of markers. The TB-13 type names are
distinctive and form a stable identity-check:

```rust
const TB_13_TYPE_NAMES: &[&str] = &[
    "CompleteSetMintTx",
    "CompleteSetRedeemTx",
    "MarketSeedTx",
    "ConditionalCollateralIndex",
    "ConditionalShareBalances",
    "ShareSidePair",
    "EventNotOpen",
    "AgentSignatureInvalid",  // (TB-13 introduced this SubmitError variant)
];
```

Modify `discover_tb_13_files()` to include any `src/**/*.rs` whose
content contains ANY of `TB_13_TYPE_NAMES` (not just the marker
heuristic). A file that imports `CompleteSetMintTx` but forgot to
add a TB-13 doc-comment marker is now caught.

**File**: `tests/tb_13_legacy_cpmm_forward_fence.rs`.

**Estimated cost**: ~15 min.

**Validation**: `cargo test --test tb_13_legacy_cpmm_forward_fence`
still 3/3 PASS; new test fixture (a temp file with a legacy import +
TB-13 type but no marker) confirms the new discovery path catches it.
(Optional — a unit test on `discover_tb_13_files` alone is also fine.)

---

### 3.3 — TB13-RQ5: resolution_tx_id contract — pick one

**Codex finding**: `ResolutionRef.resolution_tx_id` is documented as
"L4-validated" (code: round-1 doc-comment said "Sequencer validates
the reference exists in L4 + outcome matches"). The actual sequencer
ignores `resolution_tx_id` and uses `task_markets_t.state` as the
live source-of-truth. Round-4 doc fix changed the doc-comment to
declare the field opaque, but the WIRE FIELD still exists with no
code consuming it.

**Why "doc-only fix" is a punt**: the wire field has no semantics.
Future TB-15 ResolutionsIndex refactor will either need to validate
it (compatibility-breaking change to the spec contract) or remove it
(wire-format break). Either way, a TB-13 ship that locks this dead
field into the on-disk encoding is incurring schema debt that's
cheaper to resolve before ship.

**Fix plan — pick A or B (pick first, then implement)**:

**Option A: Validate `resolution_tx_id` against L4** (preserve field
with semantics).
- In `CompleteSetRedeemTx` dispatch arm, after the
  `(market_state, redeem.outcome)` match, look up
  `resolution_ref.resolution_tx_id` in the ledger writer's L4 entries.
- Verify the looked-up entry's `tx_kind` is `TaskBankruptcy` OR
  `FinalizeReward` AND its task_id matches `redeem.event_id.0`.
- Mismatch → `InvalidResolutionRef`.
- Cost: ~30 min. Adds L4-read dependency to dispatch_transition (which
  is currently pure-over-QState; this changes the dispatch contract).
- Tradeoff: the dispatch arm needs ledger access. May be tricky given
  current Sequencer API.

**Option B: Remove `resolution_tx_id` field entirely**.
- Drop `resolution_tx_id: TxId` from `ResolutionRef` struct.
- ResolutionRef becomes effectively just `claimed_outcome`. Could
  remove the wrapper struct entirely and inline the field on
  `CompleteSetRedeemTx`.
- Cost: ~20 min. Wire-format break (no production rows yet though).
- Cleaner. Less future debt.

**Recommendation (let new session re-evaluate independently)**:
Option B. The field has no operational purpose; removing it is
honest about what the dispatch logic actually does. TB-15 can add
a richer `ResolutionsIndex` reference if needed.

**File**: `src/state/typed_tx.rs` (struct definition + signing
payload), `src/state/sequencer.rs` (dispatch arm + signing-payload
projection), `tests/tb_13_complete_set.rs` (`build_redeem` helper +
fixture), `tests/economic_state_reconstruct.rs` if affected.

**Validation**: `cargo test --workspace` still green; existing
`sg_13_5` / `sg_13_6` still pass with the simpler shape.

---

## 4. Three CHALLENGES that ARE legitimately OBS — DO NOT TOUCH

These were correctly classified as OBS in round-4 and should NOT be
addressed in the fix pass:

### 4.1 — Gemini Q12: ResolutionsIndex for TB-15+

Genuine architectural future-evolution challenge. Tracked at
`handover/alignment/OBS_RESOLUTIONS_INDEX_TB15_2026-05-03.md`. Cost
to fix is multi-hour structural refactor (new EconomicState sub-field,
refactor TaskBankruptcy + FinalizeReward dispatch arms, refactor
CompleteSetRedeem). TB-13 scope does not require this; TB-15 does.

### 4.2 — TB13-RQ7: STEP_B compliance for sequencer.rs additive changes

Codebase-wide process question, not TB-13-specific. TB-12 / TB-11 /
TB-8 / TB-5 all extended sequencer.rs via direct edits without
parallel-branch STEP_B artifacts. Tracked at
`OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` (existing OBS).
Resolution = amend `STEP_B_PROTOCOL.md` with an additive-only
carve-out. Out of TB-13 scope; track separately.

### 4.3 — Codebase-wide agent-sig submit-time gap (Challenge / TaskOpen / EscrowLock)

Tracked at `OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md`. TB-13 round-2
raised the bar to its 3 Class 3 variants (replay-time) and round-3
to submit-time. The other 5 agent variants still have the gap. Fixing
them is a codebase-wide CO P2.x AgentRegistry pass, not TB-13 scope.

---

## 5. Process for the new session

1. **Read** the docs in §0 read-order. Form independent verdict.
2. **Push back** if you think this handoff is wrong (the previous
   session was demonstrably biased).
3. If you accept the plan, **fix** RQ3 + Q9/RQ6 + RQ5 in any order.
   Each is independent.
4. **Commit per fix** (3 commits, with fix-specific Trust Root rehash
   if Rust src files change).
5. **Re-run** `cargo test --workspace` after each fix; must stay
   green.
6. **Round-5 dual audit** invocation:
   ```bash
   TB13_AUDIT_ROUND=R4 python3 handover/audits/run_gemini_tb_13_ship_audit.py
   bash handover/audits/run_codex_tb_13_ship_audit.sh
   ```
   (Note: previous round saved `R3.md`; this is R4 in audit numbering
   despite being session round-5.)
7. If R4 audits both come back CHALLENGE-or-better with **no NEW
   substantive issues**, present verdict to user and ask for SHIP
   authorization.
8. If R4 audits surface a NEW VETO, halt and present to user.

**DO NOT auto-ship**. Architect §11 requires explicit user
authorization for Atom 7 SHIP.

---

## 6. Atom 7 SHIP procedure (when authorized, post-R4)

1. Update `handover/ai-direct/LATEST.md` with TB-13 SHIPPED entry at
   top (architect §11 + `feedback_session_label_codification`).
2. Append `handover/tracer_bullets/TB_LOG.tsv` row 35 with required
   columns: phase / kill_criteria_tested / flowchart_trace /
   risk_class / forbidden_list_compliance / audit_verdicts /
   workspace_test_count.
3. Single ship commit:
   `TB-13 SHIPPED — CompleteSet + MarketSeedTx (Class 3 dual audit;
   CHALLENGE-only post-RX + RQ3/Q9/RQ5 closure)`.
4. Mark Atom 7 task complete.

---

## 7. Estimated total scope

```text
Fix RQ3:                     30-45 min  (new chaintape smoke test + evidence dir)
Fix Q9/RQ6:                  ~15 min    (extend discover_tb_13_files type-name set)
Fix RQ5 (option B preferred): ~20 min   (drop resolution_tx_id field)
Trust Root rehashes:         ~5 min     (per-commit; q_state / typed_tx / sequencer)
Round-5 audit run:           ~10 min wait + minor remediation if needed
Atom 7 SHIP procedure:       ~10 min

Total (smooth path):         ~90-105 min
```

---

## 8. Cross-references

- This handoff supersedes the "carry-forward" classification of RQ3 /
  Q9-RQ6 / RQ5 in `OBS_TB13_AUDIT_RESIDUAL_CHALLENGES_2026-05-03.md`.
  The new session should mark those 3 as resolved-by-fix once done;
  the remaining 3 stay OBS.
- TB-13 charter: `handover/tracer_bullets/TB-13_charter_2026-05-03.md`
- Architect ruling lossless: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`
- Recursive self-audit: `handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md`
- TB-13 LATEST entry to add: see Atom 7 procedure above.

---

End of fix handoff.
