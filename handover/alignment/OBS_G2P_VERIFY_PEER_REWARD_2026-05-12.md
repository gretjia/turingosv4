# OBS — TB-G G2P.3 Verifier Reward + Bond Return Gap (2026-05-12)

**Type**: Forward-bound gap — TB-G G2P.3 SG-G2P.6 OBS branch (charter
"existing TB-N1 A4 gates GREEN OR `OBS_G2P_VERIFY_PEER_REWARD` filed").

**Status**: 🟡 **OBS filed** — TB-N1 A4 admission contract holds; reward
+ bond-return contract is structurally absent today. Forward-bound to
TB-N1 A5 (recent rewards / penalties prompt feedback) and the G3 cluster
(persistent PnL / solvency / reputation).

**Authority**: TB-G charter §1 Module G2P atom G2P.3 + session #43 boot
prompt verbatim "G2P.3 verifier reward / bond return audit ... SG-G2P.6:
existing TB-N1 A4 gates GREEN OR `OBS_G2P_VERIFY_PEER_REWARD` filed".

**Evidence anchor**: `handover/evidence/g_phase_g1_2_full_2026-05-11T23-36-38Z/CROSS_PROBLEM_PERSISTENCE_REPORT.md`
§3 row 3 `reputations_t Empty` — explicit Q6-time observation that no
`reputations_t` entries accumulated across the 9-task batch.

---

## §1 What the existing VerifyTx arm DOES (TB-N1 A4 — GREEN at HEAD)

`src/state/sequencer.rs` VerifyTx admission (lines ~1050-1222) accepts
agent-submitted VerifyTxs with the following invariants — all 7
`tests/constitution_n1_agent_economy_a4.rs` ship gates GREEN at HEAD
`ebc2e29` post-G2P.2:

1. **Step 2 — bond positivity**: `bond_micro == 0` rejects with
   `BondInsufficient` (SG-N1-A4.1).
2. **Step 2.5 — bond bounded by balance**: `bond > verifier_balance`
   rejects with `VerifyBondOutOfBounds` (SG-N1-A4.2).
3. **Step 3 — target liveness**: target_work_tx ∈ `stakes_t` else
   `VerifyTargetNotAccepted` (SG-N1-A4.3).
4. **Step 3.5 — duplicate gate**: `(verifier, target) ∈
   agent_verifications_t` rejects with `VerifyDuplicate` (SG-N1-A4.4).
5. **Step 4 — solvency defense-in-depth**: structurally unreachable
   post-Step-2.5; preserved.
6. **Step 5 — atomic transfer**: `balances_t[verifier] -= bond`;
   `stakes_t[verify.tx_id] = StakeEntry { amount: bond, staker: verifier,
   task_id }`.
7. **Step 5b — duplicate-set insert**: `(verifier, target) →
   agent_verifications_t` (SG-N1-A4.5 positive control).
8. **Step 6 — monetary invariants**: `assert_no_post_init_mint` +
   `assert_total_ctf_conserved` + `assert_claim_amount_backed_by_escrow`.
9. **OMEGA-Confirm path**: on `verdict == Confirm` AND target's task has
   `task_markets_t` entry, create `ClaimEntry { claimant:
   target_stake.staker, amount: total_escrow, ... }` for the **proposer**
   (NOT the verifier).

## §2 What the existing VerifyTx arm DOES NOT do

### §2.1 Gap-A — verifier reputation accumulation

`src/state/sequencer.rs` VerifyTx arm does NOT touch
`q.economic_state_t.reputations_t`. A grep of `src/state/sequencer.rs`
for `reputations_t` returns ONLY a forward-pointing doc-comment at line
3857 ("SlashTx / SettlementTx / ProvisionalAcceptTx /
ReputationUpdateTx"). No code mutates `reputations_t` in any arm — only
`Default::default()` (== `Reputation(0)`) per agent.

Per `CROSS_PROBLEM_PERSISTENCE_REPORT.md` §3 row 3 "No `reputations_t`
entries accumulated (no `VerifyTx` cycle yet; reputation += score only
on accepted Verify per TB-N1 A4. **G2P module pending**)".

The persistence-report wording "per TB-N1 A4" is aspirational — the
actual TB-N1 A4 charter does NOT scope reputation accumulation. A5
(Class-2, separately scoped: "prompt economic feedback — recent
FinalizeRewardTx + rejection penalty") is the forward atom.

### §2.2 Gap-B — bond return at run-resolve

`balances_t[verifier] -= bond` at admission is final until something
explicitly credits the bond back. No code path returns the bond:
- No FinalizeRewardTx arm credits the verifier's bond back into
  `balances_t`.
- No TerminalSummaryTx / TaskExpireTx / TaskBankruptcyTx unlocks the
  `stakes_t[verify.tx_id]` entry.
- `stakes_t[verify.tx_id]` persists across the full run; on next-run
  resume the entry remains visible (G1.1 SG-G1.3 balances reconstruction
  preserves this).

Net effect today: every accepted VerifyTx is a **permanent debit** of
`bond_micro` from the verifier's balance. There is no positive economic
incentive for the verifier — the OMEGA-Confirm claim flows to the
proposer, not the verifier.

## §3 Why this is Class-1 OBS, not a Class-3+ fix attempt

G2P module is Class 2 per charter §1 Module G2P "Class peak: 2" with
"§8 packet required: no". A reputation-credit or bond-return fix would
require:
- Sequencer admission arm code modification → Class 3 minimum
- Potential VerifyTx schema extension (per-verdict reward shape) →
  Class 4 STEP_B
- Architect §8 packet
- Per-atom dual audit timing

Per `feedback_no_workarounds_strict_constitution`: silently coding the
fix under a Class-2 atom would violate the no-shortcut user directive.
The correct forward path is:
1. **G2P.3 OBS** (this file) — make the gap visible.
2. **G3.1 / G3.2** (charter §1 Module G3; Class 3 / 4): persistent PnL +
   solvency + AutopsyCapsule emission. G3.2 is the Class-4 §8 packet
   boundary that gates Verify reward/bond-return as part of a coherent
   PnL contract.
3. Optionally TB-N1 A5 (Class-2 forward; "prompt economic feedback —
   recent FinalizeRewardTx + rejection penalty") if the architect rules
   it independent of G3.

## §4 Forward closure criteria

This OBS closes when EITHER:
- A Class-3+ atom lands sequencer-side reputation accumulation (e.g.,
  `reputations_t[verifier] += 1` on accepted VerifyTx) AND bond-return
  semantics on run-resolve, with its own ship-gate test; OR
- An architect §10 reclassification confirms the current
  "permanent-debit verify" semantics as canonical (zero reward — verify
  is altruistic protocol-maintenance only), in which case this OBS is
  retired with a verbatim quote.

Neither path is in scope for TB-G G2P.* — they are forward Class-3+
work.

## §5 Binding test pins current contract

`tests/constitution_g2p_verify_reward_bond_return.rs` SG-G2P.6 binds the
CURRENT contract so any future fix is gate-time-caught:
- SG-G2P.6.a: this OBS file exists at the expected path with the
  expected forward-closure section.
- SG-G2P.6.b: TB-N1 A4 admission contract present in
  `src/state/sequencer.rs` (bond debit + stakes_t insert +
  agent_verifications_t insert source-grep).
- SG-G2P.6.c: current code DOES NOT mutate `reputations_t` from any
  sequencer arm (negative-witness source-grep). Any future commit that
  adds `reputations_t.*insert` to sequencer.rs flips this assertion and
  surfaces the gap-closure work for review.

## §6 Cross-references

- TB-G charter §1 Module G2P atom G2P.3:
  `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
- G1.2-7 R2 persistence report:
  `handover/evidence/g_phase_g1_2_full_2026-05-11T23-36-38Z/CROSS_PROBLEM_PERSISTENCE_REPORT.md`
- TB-N1 A4 charter (no reputation/bond-return scope):
  `handover/tracer_bullets/TB_N1_AGENT_ECONOMY_PHASE_2_charter_2026-05-10.md`
- TB-N1 A4 admission test (TB-N1 A4 gates GREEN witness):
  `tests/constitution_n1_agent_economy_a4.rs`
- Forward Class-3+ atom row: TB-G charter §1 Module G3 (G3.1 / G3.2).
