# CO1.7-impl Bundle Round-1 Dual External Audit — Merged Verdict

**Date**: 2026-04-28
**Target**: CO1.7-impl bundle (A1+A2+A3+A4) + CO1.4-extra — last commit `272fcf4`
**Auditors**: Codex (gpt-5-codex; 189,349 tokens) + Gemini 2.5 Pro (137,536 tokens)
**Conservative rule** (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

---

## § 1 Verdicts

| Auditor | Verdict | Conviction |
|---|---|---|
| **Codex** | **CHALLENGE** | High |
| **Gemini** | **CHALLENGE** | High |
| **Conservative merged** | **CHALLENGE** | High |

---

## § 2 Codex must-fix items (concrete patch-mechanical defects)

| ID | Item | Codex citation | Severity |
|---|---|---|---|
| **C-1** | `replay_full_transition` accepts only `genesis_state_root` + `genesis_ledger_root`; fabricates `QState::genesis()` and patches roots. **Drops budget / registries / balances / task_markets** — CO1.7.5 cannot reconstruct real state from this API | transition_ledger.rs:348+368 | Real defect |
| **C-2** | K1 invariant **violated under infra failure**: `next_logical_t.fetch_add(1)` happens BEFORE sign + writer.commit. If sign or commit fails, run() logs/skips; the next accepted tx gets a logical_t the writer rejects forever (`expected = len + 1` invariant broken) | sequencer.rs:307,324,357 | Real defect |
| **C-3** | Replay never asserts `decoded_typed_tx.tx_kind() == entry.tx_kind`. A signed envelope claiming `Work` could ride a CAS payload that decodes as `Verify`; "every byte sequencer writes gets verified" violated | transition_ledger.rs:410,413 | Real defect |
| **C-3-secondary** | Decode errors reported as `CasMissing` — conflates lookup-miss vs corrupt-payload | transition_ledger.rs:410 | Diagnostic gap |

## § 3 Gemini must-fix / risks

| ID | Item | Gemini citation | Type |
|---|---|---|---|
| **G-1** | `head_t` constitutional alignment (Art 0.4) — Sequencer never mutates head_t; QState::default empty string for entire CO1.7-impl runtime | Q4 | Carry-forward to CO1.7.5 (per K3 v1.2 deferred-by-design) |
| **G-2** | CO1.4-extra O(N) startup latency — long-term tech debt | Q3 | Risk (not blocker) |
| **G-3** | apply_one ↔ replay divergence hazard — two implementations of same conceptual process; future maintenance risk | Q9 | Engineering discipline |

## § 4 PASS items (both auditors)

- A1 Git2LedgerWriter tree-blob design (deterministic author time, byte-stable commit OIDs)
- A2 ApplyError vs spec § 3 line 307 — defensible deviation (TransitionError stays closed-taxonomy)
- A3 `Err(NotYetImplemented)` stub pattern over `unimplemented!()` macro (avoids panic poisoning sequencer)
- A4 staging order (sig before CAS — fail-close earlier on unauthenticated entries)
- A4 LedgerCasView trait — narrow + useful for testing + future backend separation
- CO1.4-extra append-before-memory + strict-mode parse — both correct
- CO1.4-extra durability gap absent (idempotent put short-circuits before sidecar write)
- A2↔A4 signing pre-image symmetry (apply builds, replay rebuilds — same bytes)

## § 5 v1.1 patches landed (this commit)

| Patch | Maps to | Change |
|---|---|---|
| **P1** | C-1 | `replay_full_transition` signature: `(genesis: &QState, ...)` instead of two roots; caller provides full QState including budget/registries/balances. Returns `Result<QState, ReplayError>`. |
| **P2** | C-2 | apply_one: `next_logical_t.load(SeqCst) + 1` for tentative use through stages 5-8; `next_logical_t.store(logical_t, SeqCst)` ONLY after writer.commit succeeds. Single-writer per spec § 5.2.1 makes load+store sufficient (no atomic reservation needed). Doc-comment justifies the pattern + flags upgrade-to-compare_exchange path if multi-writer ever lands. |
| **P3** | C-3 | `ReplayError::TxKindMismatch { at, envelope_kind, decoded_kind }` NEW variant; replay stage 6.5 asserts `decoded_typed_tx.tx_kind() == entry.tx_kind`. New test `replay_rejects_tx_kind_mismatch` exercises the case where the signed envelope claims `Verify` but CAS decodes as `Work`. |
| **P4** | C-3-secondary | `ReplayError::PayloadDecode { at, reason }` NEW variant; replay stage 6 distinguishes decode failure from CAS lookup miss. New test `replay_rejects_payload_decode_failure`. |

## § 6 Carry-forward items (NOT closed in this round)

| ID | Status | Rationale |
|---|---|---|
| **G-1** head_t Art 0.4 | DEFERRED to CO1.7.5 | Per CO1.7 spec K3 v1.2 (already PASS/PASS audited): head_t mutation deferred to CO1.7.5+ wiring when Git2LedgerWriter exposes commit_sha. Both auditors agree the spec-level deferral is acceptable for this gate; closure happens in CO1.7.5 atom. **Documented as a CO1.7.5 hard prerequisite** in the next atom's spec. |
| **G-2** CO1.4-extra O(N) | TRACKED, not blocking | Long-term tech debt; instrument + monitor in production; embedded-DB upgrade path open for post-Wave 6 scale. |
| **G-3** apply_one ↔ replay divergence | TRACKED, partially mitigated | C-3 (tx_kind match) closes one specific divergence vector. General discipline = engineering practice + future test harness sharing canonical apply/replay logic. |
| **D** apply_one panic safety | NOTED | Codex Q-D — non-fatal for this gate; commit-before-Q-mutation panic-window is theoretical. CO1.7.5 wiring may add `tokio::task::spawn_blocking` boundary to convert panics to errors. |

## § 7 Round structure forward

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| 1 | CHALLENGE/high | CHALLENGE/high | **CHALLENGE** | v1.1 patch round (P1-P4 above) — this commit |
| 2 | ⏳ | ⏳ | TBD | re-audit on v1.1; expected PASS or 1-issue CHALLENGE on closure verification |

## § 8 Cumulative cost

| Round | Codex tokens | Gemini tokens | Estimated $ |
|---|---|---|---|
| Bundle r1 | 189,349 | 137,536 | ~$8-15 |

**CO1.7-impl bundle r1 cost**: ~$8-15. Cumulative project audit spend: ~$169-267 / $890 mid-budget (~19-30%).

---

## § 9 Sedimented lessons (this round)

1. **Bundling defensible despite 4 atoms in one audit**: both auditors gave concrete actionable findings. Bundling did NOT dilute focus. Net cost: 1× $8-15 vs 4× $8-15 = ~$24-60 per-atom = ~70% saving.

2. **K1 invariant requires "all-or-nothing" on counter advance**: spec § 3 wrote `fetch_add` at stage 4, but spec is pseudocode. Implementation must defer counter advance until after the LAST fallible step (writer.commit) succeeds. Sedimented: any monotonic counter that gates a load-bearing invariant MUST advance only on full critical-section success.

3. **Envelope-vs-payload integrity check is a real attack surface**: signing the envelope (LedgerEntry header fields including tx_kind) doesn't verify that the CAS payload bytes are the type the envelope claims. The check `decoded_payload.tx_kind() == entry.tx_kind` is a 5-LOC fix that closes a class of "envelope-payload swap" attacks. Sedimented: when an envelope binds a content-addressed reference, the dereferenced content MUST have its discriminator re-verified against the envelope's claim.

4. **API shape: replay must take FULL state, not subset**: a deceptively-small-feeling reduction (state_root + ledger_root) lost downstream-consumer capability (budget / registries). Sedimented: replay APIs are forward-locked once shipped; err on more-state-than-less for the first cut.

5. **Carry-forward CHALLENGEs are a real verdict pattern**: Gemini's #1 head_t is unfixable in this atom by spec design. The conservative-merge result is still CHALLENGE, but the action is "track + close in next atom" not "fix + re-audit". Sedimented: round-1 verdicts can have findings that close in a downstream atom; explicitly mark these as carry-forward in the merged verdict.

— ArchitectAI synthesis, 2026-04-28; Round-1 closure 2026-04-28; v1.1 patch round opens.
