#!/usr/bin/env bash
# Codex TB-13 ship audit — round-4 (after round-5 closure of RQ3 / Q9-RQ6 / RQ5).
# Class 3 (CompleteSet + MarketSeedTx). Independent of Gemini ship audit.
# Per memory feedback_dual_audit_conflict: VETO > CHALLENGE > PASS.

set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R4.md"
TMP_PROMPT="$(mktemp /tmp/tb13_codex_ship_r4.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

if [ -f "$OUT" ]; then
  echo "  refusing to overwrite existing $OUT" >&2
  exit 2
fi

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex TB-13 Ship Audit — round-4 (post round-5 fix closure)

**Role**: skeptical adversarial implementer-reviewer for the TB-13 (CompleteSet + MarketSeedTx) ship-gate dual external audit. This is **round-4** in audit numbering (the project's session-internal "round-5" closure of RQ3 / Q9-RQ6 / RQ5 from the round-3 verdict). Independent of Gemini ship audit (parallel architectural angle).

**Mandate**: Class 3 (money/collateral surface). Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS. Per memory `feedback_audit_obs_bias` (added this session): after VETOs clear, do not lump all residual CHALLENGES into one OBS bucket — table by (id / fix cost / severity / contradicts-prior-user-instruction?), only OBS-defer multi-hour future-arch.

## Round-3 verdicts (anchor)

- Codex round-3 (handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R3.md): CHALLENGE / high / FIX-THEN-PROCEED. 5 challenges (TB13-Q5-DOC, TB13-RQ5, TB13-Q9/RQ6, TB13-RQ3, TB13-RQ7). Explicit "No VETO".
- Gemini round-3 (handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R3.md): CHALLENGE / high / FIX-THEN-PROCEED. 1 challenge (Q12 ResolutionsIndex for TB-15+). Explicit non-blocking.

## Round-5 closures (this session) — what to re-verify

The fresh session refused the prior session's "ship-with-OBS for everything" recommendation and structurally fixed the 3 cheaply-fixable challenges. Three commits on top of the round-3 closure (HEAD `353aa97`):

```text
edbc555  TB-13 Atom 6 round-5 — Codex RQ5 remediation: drop ResolutionRef wrapper
a4f8265  TB-13 Atom 6 round-5 — Codex Q9/RQ6 remediation: type-use forward-fence discovery
ee8bfe8  TB-13 Atom 6 round-5 — Codex RQ3 remediation: non-empty TB-13 chaintape replay smoke
```

`cargo test --workspace = 791 passed / 0 failed / 150 ignored` (round-3 baseline 789 + 1 RQ6 test + 1 RQ3 smoke test).

### What changed (your re-verification target)

**RQ5 fix — drop ResolutionRef wrapper** (commit `edbc555`):
- `src/state/typed_tx.rs`: ResolutionRef struct deleted; CompleteSetRedeemTx 9→8 fields; CompleteSetRedeemSigningPayload 8→7 fields. `to_signing_payload` projection drops `resolution_ref`. `InvalidResolutionRef` doc-comment + Display now read "outcome does not match state" (was "resolution_ref.claimed_outcome").
- `src/state/sequencer.rs`: redeem dispatch Step 1 (claimed_outcome inner-consistency check) removed; remaining steps renumbered 1..6. State-mismatch path preserved via existing match arm using `redeem.outcome` directly.
- `src/bottom_white/ledger/transition_ledger.rs`: `TxKind::CompleteSetRedeem` doc-comment fixed (was "Validates resolution_ref against L4 (must be TaskBankruptcy for No or FinalizeReward for Yes)" — drift; now correctly states resolution authority is `task_markets_t.state`).
- `tests/tb_13_complete_set.rs`: ResolutionRef import + build_redeem fixture pruned. SG-13.5 / SG-13.6 unchanged in behavior.
- Trust Root manifest rehashed (sequencer / typed_tx / transition_ledger).
- R-022 skip token issued for the removed ResolutionRef pub-symbol backlink. Justification at `handover/alignment/OBS_R022_TB13_RESOLUTIONREF_REMOVED_2026-05-03.md`.

Verify:
- Resolution authority is now exclusively `task_markets_t[event_id.0].state` (Finalized → Yes; Bankrupt → No). No alternate path through resolution_ref / claimed_outcome / resolution_tx_id is reachable.
- `redeem.outcome` carries the agent's claim; signature covers it. No unsigned channel.
- `sg_13_6` still exercises the (Finalized, No) and (Bankrupt, Yes) mismatch rejections — confirm via the actual test body.
- Wire-format break is acceptable (no production rows yet per `feedback_no_retroactive_evidence_rewrite`).

**Q9/RQ6 fix — type-use forward-fence discovery** (commit `a4f8265`):
- `tests/tb_13_legacy_cpmm_forward_fence.rs`: added `TB_13_TYPE_NAMES` constant (10 names: CompleteSetMintTx / CompleteSetRedeemTx / MarketSeedTx / ConditionalCollateralIndex / ConditionalShareBalances / ShareSidePair / EventNotOpen / EventId / OutcomeSide / ShareAmount). Refactored `discover_tb_13_files` into `discover_by_marker` (round-2 logic) ∪ `discover_by_type_use` (new). Type-use walk catches files using TB-13 types on non-comment lines even without authoring markers.
- New unit test `discover_by_type_use_catches_unmarked_imports_and_skips_doc_xref` validates: (a) unmarked file using CompleteSetMintTx is caught; (b) doc-xref-only file (only /// or //! mentions) is skipped; (c) marker walk alone misses the unmarked file (proves type-use is the load-bearing layer).

Verify:
- `is_pure_comment_line` correctly excludes doc-comment lines (e.g., legacy `kernel.rs` / `prediction_market.rs` /// references to TB-13 types as future-work shouldn't pull those files into TB-13 scope).
- The 10 names are TB-13-introduced and don't collide with pre-existing src/ symbols.
- The fence's effective scope today is unchanged (no false positives) but the mechanism would catch a future contributor who imports a TB-13 type without adding a marker.

**RQ3 fix — non-empty TB-13 chaintape replay smoke** (commit `ee8bfe8`):
- New file `tests/tb_13_chaintape_smoke.rs` with `rq3_non_empty_tb13_chaintape_replays_with_state_root_match`: bootstraps chain-backed sequencer with `initial_q` (alice = 100 Coin, Open task-MINT, pre-seeded Finalized task-REDEEM with shares + collateral), wires real AgentKeypair via `AgentKeypairRegistry::open` (writes agent_pubkeys.json) + `set_agent_pubkeys`, submits real signed CompleteSetMintTx + CompleteSetRedeemTx through `submit_agent_tx` → driver → Git2LedgerWriter. Holds `Arc<Sequencer>` clone across `bundle.shutdown().await` to read post-drain live `q_snapshot`. Asserts:
  - pre-shutdown `conditional_collateral_t.0.len() >= 2`,
  - pre-shutdown `conditional_share_balances_t.0.len() >= 1`,
  - alice balance post = 100M − 2M + 4M = 102M micro,
  - `verify_chaintape` returns `l4_entries == 2`, all 7 indicators GREEN, `initial_q_state_loaded_from_disk = true`,
  - **`final_state_root_hex == hex(live state_root_t)`** — cryptographic proof of non-empty TB-13 map reconstruction (state_root is SHA-256 chain-fold over full QState).
- Evidence dir `handover/evidence/tb_13_chaintape_smoke_2026-05-03/{replay_report.json, agent_pubkeys.json, README.md}`. README documents the closure contract.

Verify:
- The two-tx chain (initial → mint → redeem) replays deterministically end-to-end via `verify_chaintape` Gate 4 covering both `CompleteSetMint` + `CompleteSetRedeem`.
- `complete_set_mint_accept_state_root` pre-computation is sound and matches the dispatcher's apply-time computation (no StaleParent rejection).
- The state-root equality argument is the right cryptographic shape: are there any QState fields that could go non-deterministic under replay (timestamps / random salts) that would invalidate the proof?

## The 9 mandated audit questions (per architect Part A §4 + charter §3 Atom 6.b)

Re-evaluate Q1-Q9 against round-5 HEAD. Also evaluate the round-5-specific risks:

**Q1 (anti-mint)**: Trace `src/state/sequencer.rs` CompleteSetMint accept arm. Q1 was PASS in round-3; round-5 doesn't change mint logic. Confirm.

**Q2 (resolution gate)**: Round-5 RQ5 simplified the gate (removed inner-consistency check, kept state-vs-outcome match). Walk `src/state/sequencer.rs` redeem dispatch (around line 1701..1760) and confirm: (a) Open / Expired → RedeemBeforeResolution; (b) Finalized + Yes → ok; Bankrupt + No → ok; (c) (Finalized, _) | (Bankrupt, _) → InvalidResolutionRef. The redeem.outcome is the only claim source.

**Q3 (outcome match)**: Same as Q2 above. Tests at `tests/tb_13_complete_set.rs::sg_13_6` should still cover both mismatch cases.

**Q4 (6-holding conservation)**: `src/economy/monetary_invariant.rs` `total_supply_micro` + `assert_total_ctf_conserved`. Round-5 doesn't touch this. Confirm.

**Q5 (complete-set balanced)**: `assert_complete_set_balanced` MIN form. Round-5 doesn't touch this. Confirm.

**Q6 (seed solvency)**: `MarketSeedTx` accept arm. Round-5 doesn't touch this. Confirm.

**Q7 (shares-not-Coin)**: Round-5 doesn't touch this. Confirm.

**Q8 (underflow)**: Round-5 doesn't touch this. Confirm.

**Q9 (forward-fence)**: Round-5 added type-use discovery. Re-evaluate the fence's bypass surface. Specifically: with type-use walking, can a contributor still introduce a TB-13 contribution that bypasses both the marker AND type-use layers? E.g., a file that uses TB-13 types only via aliases (`use crate::state::typed_tx::CompleteSetMintTx as Foo;` then `let x: Foo = ...`) — does the type-use scan catch the `use ... as` line? It should because `CompleteSetMintTx` appears on the use-line itself, even though the bound name is `Foo`.

## Specifically scrutinize

**Round-5 RQ3 smoke determinism**: The smoke pre-computes the post-mint state_root via `complete_set_mint_accept_state_root(&initial_root, &mint_tx)`. If the function's hashing is somehow non-deterministic (timestamps, hashmap ordering), the redeem's parent_state_root would be wrong and the redeem would reject with StaleParent. The test asserts both txs land (`l4_entries >= 2`) and the live alice balance post = 102M. Confirm the pre-computation contract is stable.

**Round-5 RQ5 wire-format break**: ResolutionRef is gone from CompleteSetRedeemTx + signing payload + canonical encoding. Old chain entries written with the old shape would not decode under the new struct. Per `feedback_no_retroactive_evidence_rewrite`, no production rows exist for TB-13 typed-tx, so no migration is needed. Confirm there are no CAS payloads or persisted artifacts that encode the old shape.

**Round-5 RQ6 type-use false-positives**: The 10 type names in TB_13_TYPE_NAMES — are any of them generic enough to false-positive in non-TB-13 code? Specifically `EventId`, `OutcomeSide`, `ShareAmount`. Grep the codebase outside FENCE_SCOPE_FLOOR and confirm these names don't appear in legacy code that would get misclassified.

**Round-5 R-022 skip-token**: ResolutionRef pub struct removed; the R-022 hook caught the missing TRACE_MATRIX backlink. The skip-token at `handover/alignment/OBS_R022_TB13_RESOLUTIONREF_REMOVED_2026-05-03.md` justifies the removal. Confirm the OBS doc is sufficient: does it explain why no replacement TRACE_MATRIX entry is needed?

## Verdict format

Use one of three:

```text
## VERDICT: PASS
- conviction: low/medium/high
- recommendation: PROCEED to SHIP
```

```text
## VERDICT: CHALLENGE
- <claim id> CHALLENGE: <one-line reason + line refs>
- <repeat per challenge>
- conviction: low/medium/high
- recommendation: FIX-THEN-PROCEED  (if cheap fix exists) or PROCEED-WITH-OBS (if architecturally deferred)
```

```text
## VERDICT: VETO
- <claim id> VETO: <one-line BLOCKING reason + line refs>
- conviction: low/medium/high
(VETO blocks ship per feedback_dual_audit_conflict; VETO > CHALLENGE > PASS.)
```

Cite file:line for every finding.

Save your audit to: handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R4.md.
BRIEF_EOF

echo "  Codex R4 audit prompt prepared at: $TMP_PROMPT" >&2
echo "  Output target: $OUT" >&2
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
