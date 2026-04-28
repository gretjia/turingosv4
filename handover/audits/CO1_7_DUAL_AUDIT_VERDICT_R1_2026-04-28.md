# CO1.7 Round-1 Dual External Audit — Merged Verdict

**Date**: 2026-04-28
**Target**: spec v1 (`handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md`) + type-skeleton (`src/bottom_white/ledger/transition_ledger.rs`)
**Auditors**: Codex (gpt-5-codex; 129k tokens) + Gemini 2.5 Pro (90k tokens)
**Conservative rule** (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS

---

## § 1 Verdicts

| Auditor | Verdict | Conviction | Top 3 must-fix |
|---|---|---|---|
| Codex | **CHALLENGE** | High | (1) sequencer logical_t skip race; (2) signing payload binds epoch + parent_ledger_root; (3) replay two-mode + CAS cold-replay |
| Gemini | **CHALLENGE** | High | (1) replay trust ambiguity → name "chain-integrity check"; (2) Q4/DIV-1 pick Path A + forward-compat serde; (3) Q2 CAS availability risk + mitigation |
| **Conservative merged** | **CHALLENGE** | High | merge below |

**Both NOT VETO**. Both accept "after v1.1 patches the joint artifact is implementable" (Codex Q-I explicit; Gemini implicit).

---

## § 2 Convergent must-fix (both flagged)

| # | Topic | Codex § | Gemini § | v1.1 patch direction |
|---|---|---|---|---|
| **C1** | **Replay completeness / trust mode** — chain-only is not v1 deliverable; rename + two-mode | Q-D | Q3 | Add `ReplayMode::ChainOnly` vs `ReplayMode::FullTransition`; spec rename current method; declare which mode is I-DETHASH witness |
| **C2** | **CAS availability / cold-replay risk** — shipped `CasStore::open()` initializes empty in-memory index → no payload recovery after restart | Q-H | Q2 | Spec acknowledges L4 → L3 dependency; either (a) defer cold-replay to v4.x with mitigation outline, or (b) v1.1 requires CasStore index persistence (separate atom) |
| **C3** | **Signing primitive integration (DIV-1 / Q8)** — extend typed CanonicalMessage path | Q-G | Q4 | **Path A** (extend) + sign a separate `LedgerEntrySigningPayload` (NOT the LedgerEntry itself); add forward-compat serialization clause |

---

## § 3 Codex-only must-fix (Codex high-conviction; Gemini didn't probe these)

| # | Topic | Codex § | v1.1 patch direction |
|---|---|---|---|
| **K1** | **Sequencer logical_t skip race** — `fetch_add` before accept skips on rejection; skeleton + replay reject gaps | Q-C top-1 | Either (a) separate counters for submissions vs accepted entries (assign logical_t at COMMIT time, not submit), or (b) rejection entries become first-class L4 entries with logical_t. Recommend (a) for v4 minimalism. |
| **K2** | **Signature does NOT bind parent_ledger_root** — transplant attack vector | Q-B (NEW) | Add `parent_ledger_root` to `LedgerEntrySigningPayload`. |
| **K3** | **L4/L5 head_t ownership inconsistent** — spec line 194 (`from_state_root`) vs line 276 (`commit_sha`) disagree | Q-E | CO1.7 owns `ledger_root_t` + commit-chain head; L5 (CO1.8) owns `state_root_t` materialization. Spec v1.1 explicit boundary; defer `head_t` mutation OR define `StateRootProvider` stub. |
| **K4** | **Spec/skeleton trait mismatch** — spec `LedgerWriter::commit(&self) → NodeId` + `iter_from`; skeleton `commit(&mut self) → Hash`, no iter | Q-H | Reconcile in v1.1; keep skeleton signature (already cargo-check verified) and update spec, OR extend skeleton trait. |
| **K5** | **TxKind::Slash dispatch gap** — enum has Slash; § 8 dispatch omits | Q-H | Either remove `Slash` from TxKind for v1 (defer to CO P2.5) OR add to dispatch. Recommend remove for v4 minimalism. |
| **K6** | **`#[repr(u8)]` on TxKind** — discriminant fragility (default Rust enum repr is unspecified for cast) | Q-H | Add `#[repr(u8)]` + explicit discriminants (e.g. `Work = 0, Verify = 1, ...`). |
| **K7** | **Conformance test gap** — spec promises 8; skeleton has 6 (none for signature, CAS recovery, sequencer serialization, full replay, canonical fixtures) | Q-H | Either lower spec promise (defer 2 tests to CO1.7.5+ along with body impls) OR add stubbed tests now. |

---

## § 4 Gemini-only must-fix

| # | Topic | Gemini § | v1.1 patch direction |
|---|---|---|---|
| **G1** | **Forward-compat extensions** — LedgerEntry struct rigid; future ZK predicate / settlement proof / public-market metadata has no place | Q9 | Add `extensions: BTreeMap<String, Vec<u8>>` field (empty in v1; reserved for v4.x without breaking schema). |

---

## § 5 Disagreement (conservative resolution required)

| # | Topic | Codex | Gemini | Conservative |
|---|---|---|---|---|
| **D1** | Should `epoch` be bound into canonical signed digest? | YES (security: signature must bind which key signed, otherwise an old-epoch key could be replayed against new-epoch key holders) | NO (Q10 — ledger_root and epoch are "orthogonal concerns"; binding is "redundant") | **Codex wins** — concrete security argument trumps aesthetic orthogonality. Without epoch binding, an attacker with leaked old-epoch key could forge entries that verify against current pinned key (per Codex Q-B "transplant attack"). Skeleton already includes epoch; v1.1 retains. |

---

## § 6 Open Q recommendations (broadly aligned)

| Q | Recommendation | Source |
|---|---|---|
| Q1 SubmissionQueue type | bounded `tokio::sync::mpsc` (NOT unbounded); `QueueFull` error or `submit_async` | Codex Q-G |
| Q4 signature placement | keep `system_signature` inside `LedgerEntry`; sign separate `LedgerEntrySigningPayload` struct | Codex Q-G + Gemini Q4 |
| Q5 dispatch | enum-match for v4; defer `MetaTransitionInterface` to v4.1 | Codex Q-G |
| Q7 genesis ledger_root_0 | sha256(canonical digest of genesis_payload.toml) — NOT `Hash::ZERO` | Codex + Gemini agree |
| Q8 CanonicalMessage extension | Path A: extend enum AND introduce `LedgerEntrySigningPayload` (NOT raw sibling digest) | Codex + Gemini agree |
| Q10 epoch binding | bind in signed payload; do NOT bind in `ledger_root_t` fold (these are different axes — see D1 resolution) | Codex (security) overrides Gemini (orthogonality) |

---

## § 7 v1.1 Patch list (consolidated, ranked by effort/impact)

### Tier 1 — Spec-only edits (~30-60 min)
1. **C1** — replay two-mode rename + ReplayMode enum spec; declare full-mode = I-DETHASH witness (skeleton can stay; v1 doc updates)
2. **C2** — § 0 + § 5 acknowledge CAS cold-replay risk; defer CasStore index persistence to dedicated atom (likely CO1.4-extra)
3. **C3** — § 1 + § 11 Q8 close: extend `CanonicalMessage::LedgerEntrySigning(LedgerEntrySigningPayload)`; spec the new struct
4. **K3** — § 0 + § 5 explicit L4/L5 boundary (CO1.7 owns ledger_root + head_t; CO1.8 owns state_root); update sequencer pseudocode `head_t` derivation
5. **K4** — reconcile `LedgerWriter` trait with skeleton (drop `iter_from` for v1; keep `&mut self` + `Hash` return)
6. **K5** — drop `TxKind::Slash` for v4; defer to CO P2.5 atom
7. **K6** — add `#[repr(u8)]` + explicit discriminants in spec § 1 schema box
8. **G1** — add `extensions: BTreeMap<String, Vec<u8>>` to LedgerEntry schema

### Tier 2 — Spec-only design upgrades (~30-45 min)
9. **K1** — sequencer dual-counter design (next_logical_t advances only on commit; submit_id is separate); update § 3 pseudocode
10. **K2** — `LedgerEntrySigningPayload` includes `parent_ledger_root` (transplant defense); update spec § 1
11. **D1** — explicit "epoch IS in signed payload" clause (closes Codex/Gemini disagreement; Codex security wins)
12. Q1/Q4/Q5/Q7/Q8/Q10 fold from § 11 Open into spec body resolutions

### Tier 3 — Skeleton patches (~30-45 min)
13. Add `#[repr(u8)]` + discriminants on TxKind
14. Drop `TxKind::Slash` (or keep + add dispatch path — pick per K5)
15. Add `extensions: BTreeMap<String, Vec<u8>>` field
16. Update `canonical_digest_unsigned`: now signs `LedgerEntrySigningPayload` (with parent_ledger_root + epoch + tx_payload_cid + resulting_state_root + tx_kind + logical_t + timestamp_logical) — NOT including resulting_ledger_root or signature
17. Add 2 new test stubs (signature binding + transplant defense) to lift count from 6 → 8

### Tier 4 — Round-2 dual audit re-launch (~$5-15)
18. Same prompt structure; updated joint artifact attached; Codex + Gemini in parallel

**Total v1.1 estimated**: 1.5-2.5 hr Claude work + ~$5-15 round-2 audit cost.

---

## § 8 Decision tree

| Path | Description | Trade-off |
|---|---|---|
| **A** | Execute v1.1 patches now (Tier 1+2+3) → round-2 audit | ~3-4 hr total to PASS/PASS gate; recommended |
| **B** | Stop at v1 + skeleton; defer round-2 to a future session | Saves immediate cost; loses momentum on Wave 6 #1 |
| **C** | Reduced v1.1 (Tier 1 only — convergent C1/C2/C3 + low-cost K3/K4/K5/K6/G1) → round-2 | ~1.5 hr; only addresses 8 of 11 must-fix; round-2 likely CHALLENGE again |

**Recommended**: **A**. Both auditors high-conviction CHALLENGE = real defects, not principle. v1.1 is a single concentrated patch round; skipping items will cost more in round-3+.

---

## § 9 Cost ledger entry

| Audit | Tokens | Estimated $ |
|---|---|---|
| Codex round-1 | 129,132 (combined input/output) | ~$5-8 |
| Gemini round-1 | 90,232 (prompt 82,763 + output 3,855 + thoughts 3,614) | ~$2-4 |
| **Round-1 sub-total** | — | **~$7-12** |

Cumulative project audit spend (per AUDIT_LEDGER): now ~$110-160 / $890 mid-budget (~12-18%).

— ArchitectAI synthesis, 2026-04-28.
