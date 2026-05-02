# TB-8 Minimal Payout Smoke Evidence — 2026-05-02

**Date**: 2026-05-02
**TB**: TB-8 (Minimal Payout / FinalizeRewardTx)
**Source**: `target/debug/evaluator` HEAD = `<TB-8 ship commit>`, branch `main`
**Model**: `deepseek-chat` via local LLM proxy at `localhost:8080/v1/chat/completions`
**Lean**: 4.29.1 (`/home/zephryj/.elan/bin/lean`)
**Mathlib**: `/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/.lake/build`
**Charter**: `handover/tracer_bullets/TB-8_charter_2026-05-02.md`
**Round-2 packaging**: post Codex round-1 VETO RQ3 (sidecar absence in `.git`-only tar.gz), evidence is now packaged as full `runtime_repo.tar.gz` + `cas.tar.gz` directories — sidecars (`pinned_pubkeys.json`, `agent_pubkeys.json`, `initial_q_state.json`, `rejections.jsonl`, `genesis_report.json`) are included so `verify_chaintape` runs cleanly from a fresh extraction.

---

## §0 Headline

**5/7 SOLVED with chain-backed FinalizeReward + Finalized claim ✓**
**2/7 UNSOLVED with no fake Finalized claim**
**Variety**: 7 distinct heldout-49 problems (mathd_algebra_171, _107, _359, _10, _11; mathd_numbertheory_961; aime_1997_p9).

| Step | Config | Outcome | TB-8 §9 Claims status | payout_micro | dashboard FinalizeReward observed |
|---|---|---|---|---|---|
| Single | n1 × `mathd_algebra_171` × MAX_TX=10 | SOLVED (per_tactic) | Finalized | 100,000 | ✓ |
| Half-1 | n1 × `mathd_algebra_107` × MAX_TX=20 | SOLVED (per_tactic, nlinarith) | Finalized | 100,000 | ✓ |
| Half-2 | n1 × `mathd_algebra_359` × MAX_TX=20 | SOLVED (per_tactic, linarith) | Finalized | 100,000 | ✓ |
| Half-3 | n1 × `mathd_algebra_10` × MAX_TX=20 | SOLVED (per_tactic, field_simp; ring) | Finalized | 100,000 | ✓ |
| Full-1 | n1 × `mathd_algebra_11` × MAX_TX=20 | UNSOLVED (hit_max_tx, 20 failed branches) | n/a | — | — |
| Full-2 | n1 × `mathd_numbertheory_961` × MAX_TX=20 | SOLVED (per_tactic, norm_num) | Finalized | 100,000 | ✓ |
| Full-3 | n1 × `aime_1997_p9` × MAX_TX=20 | UNSOLVED (hit_max_tx, 20 failed branches) | n/a | — | — |

**Aggregate**:
- 5/7 SOLVED, every SOLVED run has TB-8 §9 Claims = exactly 1 Finalized claim with `payout_micro=100,000` (= TaskMarketEntry.total_escrow seeded by `TURINGOS_CHAINTAPE_PRESEED_TASK_ESCROW_MICRO=100_000` default).
- 2/7 UNSOLVED, NO claim row in §9 (no Confirm-VerifyTx ever submitted; per-tactic OMEGA path never reached).
- All 7 ReplayReport boolean indicators GREEN per run (carried forward from TB-7R verifier).
- L4 entries per SOLVED run = 5 (TaskOpen + EscrowLock + WorkTx + VerifyTx + FinalizeRewardTx).
- L4 entries per UNSOLVED run = 2 (TaskOpen + EscrowLock seeded; no Work / Verify / Finalize).

---

## §1 What's new in TB-8 vs TB-7R smoke

The user-minimum 12-requirement contract closes here:

```text
Goal:
  ✓ accepted proof → escrow → solver balance       (Atom 3 dispatch arm)

Scope:
  ✓ single solver / single verifier / no royalty / no NodeMarket / no multi-solver split

Must:
  ✓ FinalizeRewardTx system-only                    (Atom 2 SystemEmitCommand)
  ✓ agent cannot submit FinalizeRewardTx            (TB-5 RSP-3.0 inheritance + test I121)
  ✓ payout_sum ≤ escrow                             (Atom 3 step 6 + step 8 conservation +
                                                     round-2 RQ4 idempotency on duplicate Confirm)
  ✓ CTF conserved                                   (Atom 3 step 8; 4-holding sum)
  ✓ dashboard shows payout                          (this README §0 table — payout_micro column)
  ✓ economic_state replay works                     (replay_report.json per run, all 7 indicators GREEN)
```

The **new TB-8 indicator** (`claim_finalized_chain_backed`) is observable as:
1. ≥1 `FinalizeReward` row in dashboard.txt §5 Proposal flow.
2. ≥1 `Finalized` row in dashboard.txt §9 TB-8 Claims with non-zero `payout_micro` column.
3. solver agent's balance increased by `payout_micro` in the post-run economic state.
4. The same can be reconstructed from L4 alone via `verify_chaintape` replay (`economic_state_reconstructed=true` per run).

---

## §2 Architectural empirical observations recorded

The smoke-evidence ladder surfaced four implementation issues that were fixed during the run cycle:

### §2.1 Verify bond=0 → BondInsufficient → no claim creation

The first smoke (pre-fix) showed `chain_oracle_verified=true` (CAS resolution) but no Verify on L4 — the OMEGA per-tactic site was passing `bond_micro=0` to `make_real_verifytx_signed_by`, causing the dispatch arm to reject as `BondInsufficient → L4.E`. Without a Verify on L4, the Atom-1 writer never fired and `claims_t` stayed empty.

**Fix**: change both OMEGA emit sites' `bond_micro` from 0 → 100_000 micro (0.1 coin); preseed-Agent budget of 1_000_000 micro covers ≥10 such bonds. See `experiments/minif2f_v4/src/bin/evaluator.rs` and `handover/audits/RECURSIVE_AUDIT_TB_8_2026-05-02.md` §2.3.

### §2.2 WorkTx + VerifyTx parent_state_root namespace mismatch

After the bond fix, the Verify still hit L4.E with `stale_parent_root` — both the WorkTx and VerifyTx had been constructed before either was submitted, so the VerifyTx's `parent_state_root` was the pre-Work state root. When the WorkTx accept advanced state_root_t, the queued VerifyTx became stale.

**Fix**: split the construction into two phases — submit WorkTx, await `state_root` advance via new `tb8_await_state_root_advance` helper (5s budget), THEN construct VerifyTx with fresh `parent_state_root` and submit. Best-effort fallthrough if WorkTx accept poll expires (logged warning, no exit). See `src/runtime/adapter.rs::tb8_await_state_root_advance`.

### §2.3 (Round-2 / Codex VETO RQ4) Duplicate Confirm denial-of-payout

Codex round-1 audit found that two Confirm VerifyTxs targeting the SAME WorkTx would create two Open `claims_t` rows (each backed individually by the same escrow). At finalize time, the per-claim `assert_claim_amount_backed_by_escrow` invariant would PASS individually (escrow ≥ each claim's amount), but post-mutation the escrow would drop to 0, leaving the OTHER Open claim unbacked → `MonetaryInvariantViolation` → finalize blocked. This is a denial-of-payout attack on the first money-moving system tx.

**Fix**: `src/state/sequencer.rs:540-660` Atom-1 writer now adds a one-claim-per-`work_tx_id` idempotency guard — a second Confirm targeting the same WorkTx accepts on L4 (its bond locks; verdict rides L4) but does NOT create a second claim row. 2 new regression tests (`tests/tb_8_minimal_payout.rs::I130 + I131`).

### §2.4 (Round-2 / Codex VETO RQ3) Smoke evidence packaging

Codex round-1 audit found that the `runtime_repo.dotgit.tar.gz + cas.dotgit.tar.gz` packaging (only `.git/` directories) missed required verifier sidecars: `pinned_pubkeys.json`, `agent_pubkeys.json`, `initial_q_state.json`, `rejections.jsonl`, `genesis_report.json`. Without those, a clean extraction failed `verify_chaintape` at boot.

**Fix**: `scripts/run_tb8_smoke_2026-05-02.sh:74-87` now tars the FULL `runtime_repo/` + `cas/` directories. New artifact filenames: `runtime_repo.tar.gz` + `cas.tar.gz`. Spot-check: extracting the new tar.gz pair to a clean temp dir produces an all-7-indicators-GREEN `replay_report.json` from `verify_chaintape` (l4=5 = TaskOpen+EscrowLock+Work+Verify+FinalizeReward).

---

## §3 Reproduce a run

### §3.1 Verify a committed run (round-2 packaging)

```bash
RUN=handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171
WORK=/tmp/tb8_repro/$(basename "$RUN")
mkdir -p "$WORK"
tar xzf "$RUN/runtime_repo.tar.gz" -C "$WORK"
tar xzf "$RUN/cas.tar.gz"          -C "$WORK"

# Re-run the verifier; should produce a structurally identical replay_report.json.
target/debug/verify_chaintape --repo "$WORK/runtime_repo" --cas "$WORK/cas" --out /tmp/tb8_repro_replay.json
diff <(jq 'del(.run_id, .epoch)' "$RUN/replay_report.json") <(jq 'del(.run_id, .epoch)' /tmp/tb8_repro_replay.json)

# Re-derive dashboard from committed ChainTape + CAS.
target/debug/audit_dashboard --repo "$WORK/runtime_repo" --cas "$WORK/cas" > /tmp/tb8_repro_dashboard.txt
grep -A 5 "§9 TB-8 Claims" /tmp/tb8_repro_dashboard.txt
```

### §3.2 Generate fresh evidence (LLM + Lean required)

```bash
mkdir -p /tmp/tb8_fresh/{runtime_repo,cas}
TURINGOS_CHAINTAPE_PATH=/tmp/tb8_fresh/runtime_repo \
TURINGOS_CAS_PATH=/tmp/tb8_fresh/cas \
TURINGOS_CHAINTAPE_PRESEED=1 \
TURINGOS_RUN_ID=tb8-fresh \
LLM_PROXY_URL="http://localhost:8080/v1/chat/completions" \
ACTIVE_MODEL=deepseek-chat \
CONDITION=n1 \
MAX_TRANSACTIONS=20 \
target/debug/evaluator mathd_algebra_171.lean

target/debug/audit_dashboard --repo /tmp/tb8_fresh/runtime_repo --cas /tmp/tb8_fresh/cas
target/debug/verify_chaintape --repo /tmp/tb8_fresh/runtime_repo --cas /tmp/tb8_fresh/cas --out /tmp/tb8_fresh/replay_report.json
```

The full 7-run smoke ladder is in `scripts/run_tb8_smoke_2026-05-02.sh`.

---

## §4 Per-run replay_report.json (all SOLVED runs)

Every SOLVED run reports the standard 7-indicator GREEN block plus
`detail.initial_q_state_loaded_from_disk=true`:

```json
{
  "l4_entries": 5,           // TaskOpen + EscrowLock + WorkTx + VerifyTx + FinalizeRewardTx
  "l4e_entries": 2,          // synthetic seeds (see TB-7R precedent)
  "ledger_root_verified": true,
  "system_signatures_verified": true,
  "state_reconstructed": true,
  "economic_state_reconstructed": true,
  "cas_payloads_retrievable": true,
  "agent_signatures_verified": true,
  "proposal_telemetry_cas_retrievable": true,
  "run_id": "tb8-<label>",
  "epoch": 1,
  "detail": {
    "final_state_root_hex": "<post-finalize state root>",
    "final_ledger_root_hex": "<post-finalize ledger root>",
    "head_commit_oid_hex": "<git head>",
    "l4e_last_hash_hex": "<l4e chain head>",
    "replay_failure": null,
    "initial_q_state_loaded_from_disk": true
  }
}
```

UNSOLVED runs (`mathd_algebra_11`, `aime_1997_p9`):
- `chain_oracle_verified=false` (no oracle-accepted WorkTx).
- All 7 ReplayReport indicators still GREEN (replay reconstructs the L4.E-only chain).
- §9 TB-8 Claims: `(no Confirm-VerifyTx observed; n/a — claim_status / payout: n/a)`.
- L4 entries = 2 (TaskOpen + EscrowLock seeded but no Work / Verify / Finalize).
- Per `feedback_smoke_evidence_naming`: this is chain-backed evidence (`replay_report.json` from `verify_chaintape`); not stdout-only.

---

## §5 Cross-references

- TB-8 charter: `handover/tracer_bullets/TB-8_charter_2026-05-02.md`
- TB-8 ratification: `handover/audits/CHARTER_RATIFICATION_TB_8_2026-05-02.md`
- TB-8 STEP_B preflight: `handover/audits/STEP_B_PREFLIGHT_TB8_2026-05-02.md`
- TB-8 recursive audit: `handover/audits/RECURSIVE_AUDIT_TB_8_2026-05-02.md`
- TB-8 dual external audits:
  - Codex round-1 (VETO): `handover/audits/CODEX_TB_8_SHIP_AUDIT_2026-05-02.md`
  - Codex round-2 (PASS): `handover/audits/CODEX_TB_8_SHIP_AUDIT_R2_2026-05-02.md`
  - Gemini round-1 (PASS): `handover/audits/GEMINI_TB_8_SHIP_AUDIT_2026-05-02.md`
- TB-7R smoke (precedent): `handover/evidence/tb_7r_smoke_2026-05-02/README.md`
- Smoke runner script: `scripts/run_tb8_smoke_2026-05-02.sh`
