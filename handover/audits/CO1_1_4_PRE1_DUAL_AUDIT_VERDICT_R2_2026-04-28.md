# CO1.1.4-pre1 Round-2 Dual External Audit — Merged Verdict

**Date**: 2026-04-28
**Target**: spec v1.1 + impl v1.1 + 17 tests joint artifact (commit `e0e4565`)
**Auditors**: Codex (gpt-5-codex; 165,930 tokens) + Gemini 2.5 Pro (114,610 tokens)
**Conservative rule** (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

---

## § 1 Verdicts

| Auditor | Verdict | Conviction |
|---|---|---|
| **Codex** | **CHALLENGE** | High |
| **Gemini** | **PASS** | High |
| **Conservative merged** | **CHALLENGE** | High |

**Convergence pattern**: design quality (Gemini PASS) + implementation discipline (Codex CHALLENGE) — same pattern as CO1.7 R1/R2 per memory `feedback_dual_audit_conflict`. Codex caught 4 concrete patch-mechanical defects the v1.1 patch round missed.

---

## § 2 Codex must-fix items (round-2)

| ID | Item | Codex citation | Severity |
|---|---|---|---|
| **R2-1** | `SignalKind::Finalize` and `SignalBundle::finalize` still use `TxId`, NOT `ClaimId` (P2 missed call site) | typed_tx.rs:826-854 | Direct ABI leak |
| **R2-2** | `FinalizeRewardTx.system_signature` and `TaskExpireTx.system_signature` retained but no `CanonicalMessage::FinalizeRewardSigning` / `::TaskExpireSigning` variants + no emitter fns. Dual-sign rationale (§ 4.2) not executable for 2 of 3 system txs | system_keypair.rs (no variants); typed_tx.rs:266+316 (sigs retained) | Authorized signing path missing |
| **R2-3** | Spec drift: § 0 line 47 still says TerminalSummaryTx lives in system_keypair.rs; § 6 line 210 says "imported from system_keypair"; § 9 D-3 row still present despite P7 claiming removal | spec § 0/§ 6/§ 9 | Self-contradictory spec |
| **R2-4** | Signing-payload tests not load-bearing: `signing_payload_domains_are_distinct` uses different bodies (would pass even if domains were removed); `signing_payload_excludes_signature` only tested for WorkTx; no golden hex for signing-payload digests | typed_tx.rs domain test + signing-payload tests | Test discipline gap |

## § 3 Codex secondary findings (caveats, not must-fix)

- BTreeMap permutation test only covers BTreeSet (read_set); predicate_results + failure_class_histogram BTreeMap fields untested for permutation independence
- Missing dedicated `FinalizeRewardSummaryMismatch` error variant for Q-derived wire-vs-Q discipline rejection (P8 § 4.1) — could be added in CO1.7-impl A4

## § 4 Gemini recommendations (PASS verdict; non-blocking)

- **GR-1**: reserve MetaTx domain prefix in typed_tx.rs (placeholder constant for v4.1 namespace)
- **GR-2**: spec note committing to additive-only TransitionError changes within v4 major version
- **GR-3**: brief domain-string rotation process documented in spec

---

## § 5 v1.2 patch plan (round-2 closure)

| Patch | Maps to | Touches |
|---|---|---|
| **P11**: SignalKind::Finalize { claim_id: ClaimId, ... } + SignalBundle::finalize signature update | R2-1 | typed_tx.rs |
| **P12**: NEW `CanonicalMessage::FinalizeRewardSigning([u8; 32])` + `TaskExpireSigning([u8; 32])` variants + canonical_digest match arms + `transition_emitter::sign_finalize_reward` + `sign_task_expire` emitter fns | R2-2 | system_keypair.rs |
| **P13**: spec drift cleanup — § 0 line 47 says state::typed_tx now; § 6 line 210 update; § 9 D-3 row REMOVED (still present) | R2-3 | spec |
| **P14**: same-body-different-domain load-bearing test (build identical bincode body bytes, hash with each domain prefix, assert all 6 distinct) + lock 6 signing-payload golden hex constants | R2-4 | typed_tx.rs tests |
| **P15** (Codex secondary): BTreeMap permutation independence test using predicate_results | secondary | typed_tx.rs tests |
| **GR-1** (Gemini recommendation): reserve `DOMAIN_AGENT_META: &[u8] = b"turingosv4.agent_sig.meta_proposal.v1"` constant — placeholder; never used in v4 | low-risk recommendation | typed_tx.rs |
| **GR-2** (Gemini recommendation): spec § 7.2 NEW additive-only TransitionError commitment | low-risk recommendation | spec |
| **GR-3** (Gemini recommendation): spec § 7.3 NEW domain-string rotation process | low-risk recommendation | spec |

**Estimated scope**: ~150-250 LoC code + 30-50 LoC spec. ~0.3 day. Round-3 audit cost: ~$8-15.

---

## § 6 Round structure forward

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| 1 | CHALLENGE/high | CHALLENGE/high | CHALLENGE | v1.1 patch round (10 patches; commit `e0e4565`) |
| 2 | **CHALLENGE/high** | **PASS/high** | **CHALLENGE** | v1.2 patch round (5 patches + 3 Gemini recommendations) |
| 3 | ⏳ | ⏳ | TBD | round-3 closure check; expected PASS/PASS |

---

## § 7 Cumulative cost

| Round | Codex tokens | Gemini tokens | Estimated $ |
|---|---|---|---|
| 1 | 199,200 | 113,295 | ~$8-15 |
| 2 | 165,930 | 114,610 | ~$7-13 |
| **CO1.1.4-pre1 r1+r2 total** | **365,130** | **227,905** | **~$15-28** |

Cumulative project audit spend: ~$150-230 / $890 mid-budget (~17-26%).

---

## § 8 Sedimented lessons (this round)

1. **Single-call-site type-update is insufficient**: P2 added ClaimId newtype but missed that SignalBundle::Finalize still used TxId. Sedimented: when changing a tx-payload field type, grep for ALL consumers (incl. SignalBundle / runtime APIs / fixtures), don't just update the struct definition.

2. **Symmetric-API completion**: P3 added `TerminalSummarySigning` variant but did NOT add corresponding `FinalizeRewardSigning` / `TaskExpireSigning` (the other two system-emitted txs). Sedimented: when introducing a typed signing primitive for one variant, confirm symmetric coverage for ALL variants in the same family — partial migration creates execution-blocked dual-sign rationale.

3. **Domain-prefix tests must use IDENTICAL bodies**: a non-collision test using DIFFERENT bodies passes trivially even without domain prefix. Sedimented: load-bearing domain-separation tests must construct identical bincode body bytes, hash with each domain, and assert distinct results — otherwise the test is testing struct-shape uniqueness, not domain prefix.

4. **Spec drift after structural migration**: P3/P7 claimed D-3 row removal but only updated the row content; the row was still in § 9. Sedimented: when claiming "row removed", verify with grep on the spec doc — claim-vs-doc parity has been an audit finding before (CO1.7 round-2 R2-C3 was the same pattern: spec said "C3 CLOSED" while code had no LedgerEntry path).

5. **Codex implementation discipline + Gemini design quality is a stable axis decomposition**: matches CO1.7 round-1/2 + this audit. PASS/PASS only when both axes clean. Project pattern.

— ArchitectAI synthesis, 2026-04-28; Round-2 closure 2026-04-28; v1.2 patch round opens.
