#!/usr/bin/env bash
# Codex round-1 audit on CO1.1.4-pre1 Typed Tx ABI surface — spec v1 + impl + tests.
# Implementer-review angle: do the ABI types compose with STATE_TRANSITION_SPEC § 1?
# Are the 3 divergences (D-1/D-2/D-3) justified, or do they break downstream
# CO1.7.5 transition body atom?
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md"
TMP_PROMPT="$(mktemp /tmp/co1_1_4_pre1_codex_round1.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex Adversarial Audit — CO1.1.4-pre1 Typed Tx ABI surface (Round 1)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-1 (running in parallel).

**Mandate**: round 1 dual external audit on the **joint artifact** (CO1.1.4-pre1 spec v1 + Rust code + 11 inline tests). Per CLAUDE.md "Audit Standard": Generator ≠ Evaluator (Claude generated; you review). Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

**Why this atom exists**: when CO1.7-impl A1 (Git2LedgerWriter, commit `a03cc52`) shipped, downstream A2 (Sequencer + dispatch_transition) discovered ~30 supporting schema types per STATE_TRANSITION_SPEC § 1 are required but **none of them existed in code** (only `MicroCoin`). Per (b) decision (atom split, not absorption): CO1.1.4-pre1 ships the ABI surface in isolation under its own dual-audit gate, before A2-A4 unblock.

## What you're reviewing

1. **Spec doc**: `handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md` (~250 lines, 12 sections)
2. **Implementation**: `src/state/typed_tx.rs` (~800 lines incl. 11 tests, 215/0 lib PASS, TR-tracked)
3. **Supporting changes**: small additive Default + Serialize/Deserialize derives in
   - `src/economy/money.rs` (+StakeMicroCoin newtype + Default on MicroCoin)
   - `src/bottom_white/cas/schema.rs` (+Default on Cid)
   - `src/bottom_white/ledger/system_keypair.rs` (+Default on SystemEpoch + SystemSignature; serde_bytes_64 → pub(crate); +Serialize/Deserialize/Default on TerminalSummaryTx)
4. **Companion specs (frozen)**: STATE_TRANSITION_SPEC v1.4 § 1 + § 2.5 + § 3 + § 4

## What is at stake

- **PASS** unblocks CO1.7-impl A2 (TypedTx + transition stubs → Sequencer + dispatch_transition + replay_full_transition); and is a transitive dependency of CO P2.x economy atoms (TaskId / AgentSignature / etc. are reused).
- **CHALLENGE** triggers spec/code v1.1 patch round.
- **VETO** blocks the atom entirely (e.g., if TxStatus elision is found constitutionally unsound).

## Round 1 audit questions

**Q-A. D-1 (TxStatus elision) validity**: spec § 9 D-1 drops `status: TxStatus` from WorkTx wire bytes (STATE spec § 1.2 has it as field 12). Reasoning: TxStatus is runner book-keeping; tracked in `q_t.q_t.agents[id].last_accepted_tx` + ClaimsIndex.
- Is this elision constitutionally sound (Art 0.2 tape canonicality — tape MUST contain everything needed for replay)?
- Could replay reconstruct status from Q_t evolution alone (i.e., is status DERIVABLE from accepted+finalized tx history), or does it need to be on the wire?
- Does any spec § 3 transition function read `tx.status` from a received tx? (If yes, elision breaks the contract.)
- What's the migration path if this is wrong (add field back vs separate runtime-only enum)?

**Q-B. D-2 (FinalizeRewardTx derived schema) validity**: spec § 4 derives the FinalizeRewardTx schema (9 fields: tx_id / claim_id / task_id / solver / reward / parent_state_root / epoch / timestamp_logical / system_signature) because STATE spec § 3.4 uses opaque `FinalizeTx::from(claim_id, reward)` constructor without an explicit struct.
- Is the field set sufficient for `finalize_reward_transition § 3.4 stage 3` (unlock + return solver stake + credit reward + finalize claim + debit escrow + pay royalties along royalty_graph_t)?
- What's missing? Must `royalty_edges_at_finalize` be on the wire (DAG attribution) or recomputed from Q_t?
- Should `claim_id` be a typed `ClaimId` newtype rather than reused `TxId`?
- Audit input the spec explicitly requested: this is the schema most likely to attract a CHALLENGE.

**Q-C. D-3 (TerminalSummaryTx field-set divergence)**: shipped `system_keypair.rs::TerminalSummaryTx` has 3 fields (run_id / terminal_state_root / rejected_attempt_count); STATE spec § 1.5 defines an 8-field schema (tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature). CO1.1.4-pre1 imports the 3-field version unchanged.
- Is keeping the 3-field placeholder correct, or must this atom upgrade to the 8-field spec NOW (because once the wire format is locked, future field additions are bincode-compatible-but-fixture-breaking)?
- Module-placement question: should TerminalSummaryTx move from `system_keypair.rs` to `state/typed_tx.rs` so all TypedTx variants live in one module?

**Q-D. Bincode v2 settings & cross-platform stability**: CO1.7-impl A1 (commit `a03cc52`) froze `bincode_canonical_config()` = `standard().with_big_endian().with_fixed_int_encoding()`. The CO1.1.4-pre1 tests reuse this codec via `canonical_encode` / `canonical_decode`. Concerns:
- BTreeMap iteration order — is bincode-2's serde adapter guaranteed to use BTreeMap's lex iteration (deterministic), or does it use a different iteration?
- Enum variant indices: bincode-2 with fixed_int_encoding uses u32 BE for variant indices (right?). With 7-variant TypedTx + 7-variant RejectionClass + 5-variant RunOutcome, variant index stability is critical for replay across binaries.
- Does `#[serde(transparent)]` on StakeMicroCoin / SlashEvidenceCid produce identical wire bytes as bare MicroCoin / Cid? (Confirm; this is a non-breaking-change requirement.)

**Q-E. AgentSignature vs SystemSignature type-distinction**: AgentSignature `pub struct AgentSignature(#[serde(with = "serde_bytes_64")] [u8; 64])` reuses the same serde adapter as SystemSignature.
- Wire bytes: byte-identical with the same `[u8; 64]` content. Does this open a confusion attack (an agent-signed payload mistakenly verified as system-signed or vice versa)?
- The intent is type-distinction at the API surface. Is the type-level guarantee enough, or is a domain-separation prefix in the canonical_digest mandatory (e.g. "v4.agent_sig" vs "v4.system_sig")?

**Q-F. SignalBundle minimal v1 — does it actually work for CO1.7.5?**: spec § 0 says "minimal SignalBundle: a single enum-like discriminator + payload sufficient for CO1.7-impl to compile". Implementation: `pub struct SignalBundle { pub kind: SignalKind }` with 4 variants (Empty/Finalize/TaskExpired/TerminalSummary).
- Does CO1.7.5 step_transition / verify_transition / challenge_transition / reuse_transition need additional variants this v1 didn't anticipate?
- Is the `SignalKind::TerminalSummary { run_id, outcome }` field set sufficient, or does it need the failure_histogram for L6 indices?

**Q-G. TransitionError taxonomy completeness**: spec § 0 commits to a minimal v1 enum with 10 variants. Audit:
- Does `step_transition` (STATE spec § 3 stages 1-7) raise ANY error class not in this list? Specifically: SignatureInvalid / StakeInsufficient / PredicateFail / AttributionGraphCycle / TooManyRetries — are these all expected to be runtime book-keeping (RejectionClass) NOT transition errors?
- Is the `NotYetImplemented` sentinel a code smell that audit should flag? (It's the explicit interface CO1.7.5 will fill in; documented in spec § 0.)

**Q-H. HasSubmitter trait correctness**: spec § 8 wires per STATE § 3.6.5 v1.3:
- WorkTx → Some(agent_id), VerifyTx → Some(verifier_agent), ChallengeTx → Some(challenger_agent), ReuseTx → None, FinalizeRewardTx → None, TaskExpireTx → None, TerminalSummaryTx → None.
- Reuse partitioning: ReuseTx has a `reused_tool_creator: AgentId` field — the creator IS the royalty recipient, not the submitter. Spec keeps `submitter_id() = None`. Does this preserve the implicit-init invariant (per § 3.6.5 the submitter is auto-init'd; tool creator already exists from prior accept). Confirm.
- TypedTx outer impl just delegates to inner; correct?

**Q-I. Atom scope creep**: CO1.1.4-pre1 imports + extends 3 modules outside its core (cas/schema, money, system_keypair). Each addition is purely additive (new derive / new fn / new newtype). Does any of these:
- accidentally widen the bottom_white/cas API surface in a way that should be its own atom?
- create a dependency cycle between state and bottom_white (spec § 1 says "no circular dep risk")?
- leak typed_tx-only concerns into a sibling module?

**Q-J. New defects**: independent of spec catalog, what does the joint artifact still get wrong?
- Type errors that cargo check missed (rare but possible)?
- Test gaps (e.g. golden_*_tx_digest tests assert digest STABILITY but don't lock the actual hex value yet — is the "phase 1: record only" deferral acceptable, or must round-1 lock the values now)?
- Missing invariants — anything from STATE spec § 4 (27 invariants) that CO1.1.4-pre1 should enforce at the type level but doesn't?
- Spec ↔ code parity: any place where the spec says X and the code says Y?

## Output format

# Codex CO1.1.4-pre1 Round-1 Audit
## Q-A D-1 TxStatus elision
## Q-B D-2 FinalizeRewardTx derivation
## Q-C D-3 TerminalSummaryTx divergence
## Q-D Bincode settings & cross-platform stability
## Q-E AgentSignature/SystemSignature distinction
## Q-F SignalBundle minimal v1
## Q-G TransitionError taxonomy
## Q-H HasSubmitter correctness
## Q-I Atom scope creep
## Q-J New defects
## **VERDICT**: PASS / CHALLENGE / VETO
## Top 3 must-fix (if CHALLENGE)
## Conviction (low/med/high)

Be rigorous. Cite spec § + code line numbers. Do NOT pass on principle; do NOT veto on principle. Real defects = CHALLENGE; foundational design flaw = VETO; clean joint artifact = PASS.

---

BRIEF_EOF

echo "" >> "$TMP_PROMPT"
echo "# CO1.1.4-pre1 spec v1 (target of audit)" >> "$TMP_PROMPT"
echo "" >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# Implementation: src/state/typed_tx.rs (target of audit, joint with spec above)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/typed_tx.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Supporting: src/state/mod.rs (re-exports)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/state/mod.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Supporting: src/economy/money.rs (+StakeMicroCoin newtype, +Default)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/economy/money.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Supporting: src/bottom_white/cas/schema.rs (+Default on Cid)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/cas/schema.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# Supporting: src/bottom_white/ledger/system_keypair.rs (+Default + serde_bytes_64 pub(crate) + TerminalSummaryTx serde derives)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/system_keypair.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\n# XREF: STATE_TRANSITION_SPEC v1.4 (frozen, round-4 PASS/PASS) — § 1 typed schemas + § 2.5 canonical serialization + § 3 transition pseudocode + § 4 invariants\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: CO1.7 spec v1.2 (PASS/PASS 2026-04-28; consumes the ABI defined here)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md" >> "$TMP_PROMPT"
printf '\n\n---\n\n# XREF: shipped src/bottom_white/ledger/transition_ledger.rs (consumer of TypedTx; CO1.7-impl A1)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/bottom_white/ledger/transition_ledger.rs" >> "$TMP_PROMPT"
printf '\n```\n\n---\n\nNow give your INDEPENDENT round-1 audit. Cite spec § + code line where possible.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex co1.1.4-pre1 r1] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex CO1.1.4-pre1 Typed Tx ABI Round-1 Audit\n'
  printf '**Date**: 2026-04-28\n'
  printf '**Target**: spec v1 + impl + tests joint artifact\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex co1.1.4-pre1 r1] API returned in ${elapsed}s" >&2
echo "[codex co1.1.4-pre1 r1] saved: $OUT" >&2
