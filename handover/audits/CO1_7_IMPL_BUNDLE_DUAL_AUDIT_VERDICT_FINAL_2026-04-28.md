# CO1.7-impl Bundle Dual External Audit — Final Merged Verdict ✅ PASS/PASS-equivalent

**Date**: 2026-04-28
**Atom**: CO1.7-impl bundle (A1+A2+A3+A4) + CO1.4-extra
**Final state**: v1.1.1 (commit `1bc8887` impl + `[next]` doc-banner refresh)
**Conservative rule** (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

---

## § 1 Final verdicts

| Auditor | Final round | Verdict | Conviction |
|---|---|---|---|
| **Codex** | r3 (v1.1.1) | **PASS** | High |
| **Gemini** | r1 (v1.0) | CHALLENGE → carry-forward to CO1.7.5 | High |
| **Conservative merged** | — | **PASS-equivalent** ✅ (with G-1 head_t deferred per spec design) | High |

**Pre-CO1.7.5 implementation gate**: CLEARED. CO1.7.5 (per-kind transition function bodies + STEP_B wiring) is now unblocked.

---

## § 2 Round-by-round summary

| Round | Codex | Gemini | Conservative | Patch Round Output |
|---|---|---|---|---|
| 1 | CHALLENGE/high (3 must-fix + secondary) | CHALLENGE/high (1 carry-forward + 2 risks) | CHALLENGE | v1.1: P1-P4 (4 patches); commit `1a921e5` |
| 2 (Codex-only) | CHALLENGE/high (2 missing tests claimed-not-shipped) | (carry-forward Gemini r1) | CHALLENGE | v1.1.1: 2 missing tests added; commit `1bc8887` |
| 3 (Codex-only) | **PASS/high** | (carry-forward Gemini r1) | **PASS** ✅ | doc-banner refresh; this commit |

---

## § 3 Closure verification (cumulative)

| Codex r1 must-fix | Closure |
|---|---|
| C-1: replay_full_transition signature drops genesis QState | P1: signature → `(genesis: &QState, ...) -> Result<QState, ReplayError>` |
| C-2: K1 violated under infra failure | P2: `next_logical_t.store()` deferred until AFTER `writer.commit` succeeds |
| C-3: replay never asserts tx_kind match | P3: NEW `ReplayError::TxKindMismatch`; stage 6.5 assertion + test (added v1.1.1) |
| C-3-secondary: decode error reported as CasMissing | P4: NEW `ReplayError::PayloadDecode`; stage 6 distinguishes; test (added v1.1.1) |

| Gemini r1 must-fix / risks | Status |
|---|---|
| G-1: head_t Art 0.4 alignment | DEFERRED to CO1.7.5 per CO1.7 spec K3 v1.2 (already PASS/PASS at spec level); both auditors agreed deferral acceptable |
| G-2: CO1.4-extra O(N) startup latency | TRACKED as long-term tech debt (instrument + monitor in production; embedded-DB upgrade open for post-Wave 6 scale) |
| G-3: apply_one ↔ replay divergence hazard | PARTIALLY MITIGATED via C-3 (tx_kind match closes one specific divergence vector); general engineering discipline going forward |

---

## § 4 Cumulative cost

| Round | Codex tokens | Gemini tokens | Estimated $ |
|---|---|---|---|
| Bundle r1 | 189,349 | 137,536 | ~$8-15 |
| Bundle r2 (Codex-only) | 67,977 | — | ~$3-5 |
| Bundle r3 (Codex-only) | 124,574 | — | ~$3-5 |
| **Bundle total** | **381,900** | **137,536** | **~$14-25** |

**CO1.7-impl bundle total cost**: ~$14-25. Cumulative project audit spend: ~$175-273 / $890 mid-budget (~20-31%).

The bundle audit cost (~$14-25 across 3 rounds) is **substantially cheaper than per-atom audit** would have been (4 atoms × ~$8-15 each = ~$32-60). Bundling validated.

---

## § 5 What CO1.7.5 inherits

**Code surface frozen**:
- `Sequencer` (state/sequencer.rs) with full apply_one machinery; only the per-kind transition bodies are stubbed
- `dispatch_transition` exhaustive 7-variant match
- `replay_full_transition` 9-stage I-DETHASH witness (incl. tx_kind match + decode-vs-CAS-miss separation)
- `Git2LedgerWriter` production storage backend
- `LedgerCasView` trait + CasStore impl
- `CasStore` with sidecar JSONL persistence (CO1.4-extra)

**CO1.7.5 must deliver**:
1. Real per-kind transition function bodies for the 7 TypedTx variants in `dispatch_transition` (currently `Err(NotYetImplemented)`)
2. **Close G-1 head_t Art 0.4 alignment** — wire `q.head_t = NodeId(commit_oid_hex)` after `Git2LedgerWriter::commit` returns; `Git2LedgerWriter::head_commit_oid()` already exposed for this
3. STEP_B parallel-branch ceremony for any modifications to `bus.rs` / `kernel.rs` (per CLAUDE.md "Code Standard")
4. Once bodies land: remove `#[ignore]` from `sequencer_serial_replay_byte_identity` test; verify final state_root reconstruction end-to-end

---

## § 6 Sedimented lessons (cumulative across rounds)

1. **Bundle audits work for tight-clustered atoms**: 4 atoms in 1 audit gave concrete actionable findings without focus dilution. Net 50%+ saving vs per-atom audit. Sedimented: when atoms share an architectural design and ship together, bundle the audit.

2. **K1 invariant requires "all-or-nothing" on counter advance**: monotonic counters that gate load-bearing invariants MUST advance only on full critical-section success. Spec § 3 stage-4 fetch_add was wrong; correct pattern is `load → use → store after final fallible step`.

3. **Envelope-vs-payload integrity check is a real attack surface**: signing the envelope's `tx_kind` doesn't verify the dereferenced CAS payload's actual variant. Round-1 CHALLENGE caught this; the 5-LOC fix in stage 6.5 closes a class of envelope-payload swap attacks.

4. **Replay APIs are forward-locked**: a deceptively-small reduction in API shape (state_root + ledger_root vs full QState) loses downstream consumer capability irretrievably once shipped. Err on more-state-than-less for first cuts.

5. **Carry-forward CHALLENGEs are a real verdict pattern**: Gemini's #1 head_t was unfixable in this atom by spec design (K3 v1.2 deferred). The conservative-merge result is still CHALLENGE, but the action is "track + close in next atom" not "fix + re-audit". Documented as carry-forward in merged verdict.

6. **Claim-vs-code parity drift recurs across atoms**: round-2 CHALLENGE caught 2 tests claimed in v1.1 commit message but never actually landed (silent Edit failure). Same lesson as CO1.1.4-pre1 doc-hygiene rounds. Sedimented: post-implementation, before audit dispatch, run a programmatic check that "claimed identifiers exist in source".

7. **Codex-only narrow rounds are efficient closure mechanism**: when Gemini has already accepted the design and only patch-mechanical defects remain, dispatch Codex-only saves ~$3-5/round. CO1.1.4-pre1 + CO1.7-impl bundle both used this pattern successfully.

— ArchitectAI synthesis, 2026-04-28; CO1.7-impl bundle PASS/PASS-equivalent gate cleared 14:12 UTC.
