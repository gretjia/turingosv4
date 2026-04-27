# V4.1 MetaTape Implementation Plan v1

> **Date**: 2026-04-27
> **Purpose**: Plan v3.2 CO P3-prep.6 — atomization plan for v4.1 runtime ArchitectAI/JudgeAI implementation. Consumes v4 Phase 3 prep artifacts. Locked at v4 ship; v4.1 sprint launches against this plan.
> **Authority**: WP architecture § 12 + § 17; Constitution Art V.1.2 + V.1.3.
> **Status**: v4 deliverable (this doc itself); v4.1 implementation begins post-v4 ship.

---

## § 1 What v4.1 Adds Over v4

**v4 ships**:
- 5 specs (state transition / genesis / Art 0.2 / system keypair / metatx schema)
- `governance::meta_validator` (offline library)
- `MetaTransitionInterface` traits (zero implementations)
- AmendmentFlow format
- `governance::identity` types

**v4.1 ships** (what this plan defines):
- `RuntimeArchitectActor` (impl `ArchitectActor` trait)
- `RuntimeJudgeActor` (impl `JudgeActor` trait)
- `RuntimeMetaCoordinator` (impl `MetaCoordinator` trait)
- L4 acceptance of `MetaTx` (extends `step_transition` with `meta_transition` arm)
- M-of-N judge quorum logic
- Proposal lifecycle: Pending → UnderReview → Approved/Vetoed/HumanReview → Applied
- AmendmentFlow ingestion (parse historical v4 cp-amendments + treat as bootstrap precedent)

---

## § 2 v4.1 Atom List

Estimated **~25 atoms across 4 sub-phases** (~4-6 weeks).

### v4.1.A — Runtime Actor Implementations (8 atoms)

| Atom | Scope | Dependencies |
|---|---|---|
| v4.1.A.1 | `RuntimeArchitectActor::new` + keypair management | v4 `governance::identity` + `SYSTEM_KEYPAIR_SECURITY_v1` extended for actor keypairs |
| v4.1.A.2 | `RuntimeArchitectActor::observe_and_decide` — observability hook + threshold-based proposal logic | v4 L5 `materializer` + L6 signal indices |
| v4.1.A.3 | `RuntimeArchitectActor::validate_before_submit` — wraps `meta_validator` | v4 `meta_validator` |
| v4.1.A.4 | `RuntimeArchitectActor::submit` — emits MetaTx to L4 | v4 L4 `transition::append` extended for MetaTx |
| v4.1.A.5 | `RuntimeJudgeActor::new` + judge specialization config | v4.1.A.1 |
| v4.1.A.6 | `RuntimeJudgeActor::review` — automated audit + verdict signing | v4 `meta_validator` + audit subroutines |
| v4.1.A.7 | `RuntimeJudgeActor::sign_verdict` | v4.1.A.5 |
| v4.1.A.8 | Conformance tests for ArchitectActor + JudgeActor implementations | n/a |

### v4.1.B — Coordinator + L4 Integration (6 atoms)

| Atom | Scope | Dependencies |
|---|---|---|
| v4.1.B.1 | `RuntimeMetaCoordinator::new` + actor registry + quorum config | v4.1.A.* |
| v4.1.B.2 | `RuntimeMetaCoordinator::tick` — main scheduling loop | v4.1.A.* + v4 L5 polling |
| v4.1.B.3 | `meta_transition` arm of `step_transition` (extends v4 `transition::step_transition`) | v4 step_transition + L4 ledger |
| v4.1.B.4 | `RuntimeMetaCoordinator::apply_approved` — mutate predicate_registry / tool_registry | v4.1.B.3 |
| v4.1.B.5 | M-of-N quorum logic — when do we treat a proposal as "passed" | v4.1.A.5/.6 |
| v4.1.B.6 | Conformance tests: end-to-end proposal → judges → apply → state root advance | v4.1.B.1-5 |

### v4.1.C — Constitutional Path (Human Signature Gate) (5 atoms)

| Atom | Scope | Dependencies |
|---|---|---|
| v4.1.C.1 | Detection: when proposal touches constitution.md → human_signature_required = true | v4 AmendmentFlow format § 3 |
| v4.1.C.2 | Pause-and-prompt: runtime stops applying constitutional MetaTx until human PGP signature received | v4.1.B.4 + interrupt protocol |
| v4.1.C.3 | Human signature verification path (calls v4 `governance::amendment_flow_validator`) | v4 amendment_flow_validator |
| v4.1.C.4 | Constitution.md update via runtime (extends v4 cp-workflow with automated diff application after human-sig verified) | v4.1.C.3 |
| v4.1.C.5 | Conformance test: constitutional MetaTx blocked at human-sig gate; passes after sig provided | v4.1.C.1-4 |

### v4.1.D — Historical Ingestion + Polish (6 atoms)

| Atom | Scope | Dependencies |
|---|---|---|
| v4.1.D.1 | Read v4 cp-workflow amendments from `handover/architect-insights/AMENDMENT_*.md` | v4 AmendmentFlow format |
| v4.1.D.2 | Convert legacy v4 amendments to MetaTx for L4 ingestion (one-time migration) | v4.1.D.1 |
| v4.1.D.3 | Replay history: re-evaluate every historical amendment under current judge actors (sanity check) | v4.1.D.2 |
| v4.1.D.4 | AUDIT_LEDGER auto-population: every runtime MetaTx adds a ledger row | v4 AUDIT_LEDGER format |
| v4.1.D.5 | TRACE_MATRIX_v3 auto-update: when MetaTx adds a predicate, matrix auto-extends | v4 trace_matrix conformance |
| v4.1.D.6 | Final v4.1 exit dual external audit | v4.1.A-D complete |

---

## § 3 v4.1 → v4 Compat Matrix

| v4 artifact | Used by v4.1 | Compatibility status |
|---|---|---|
| `governance::meta_validator::validate_meta_proposal` | v4.1.A.3 / v4.1.A.6 | direct call; no wrapper |
| `MetaTransitionInterface` traits | v4.1.A.* impl | strict; v4.1 must implement traits exactly as defined |
| `governance::identity::*` types | v4.1.A.* | direct use; identity types stable |
| `SYSTEM_KEYPAIR_SECURITY_v1` | v4.1.A.1 / v4.1.A.5 | actor keypairs follow same lifecycle as system keypair (gen / encrypt / sign API restricted) |
| `META_TX_SCHEMA_v1` | v4.1.B.3 (L4 acceptance) | strict; v4.1 must accept MetaTx with this exact schema |
| `AmendmentFlow format` | v4.1.D.1-3 | parser strict; legacy migration handles pre-format-spec amendments |
| `RATIFICATION_*.md` chain | v4.1.D.4 | v4.1 reads + extends; new MetaTx generates new RATIFICATION entries |
| `TRACE_MATRIX_v3` N/M/D | v4.1.D.5 | v4.1 may add new Normative rows when MetaTx adds predicates |

**Wire compat hard rule**: v4 → v4.1 upgrade must NOT require any data migration except the one-time amendment-to-MetaTx migration in v4.1.D.2.

---

## § 4 What v4.1 Does NOT Do

- **Does not implement Phase 4 / Phase 5**: permissioned chaincode, public AGI market remain v4.x or v5
- **Does not change constitution semantics**: amendments still go through human gate; v4.1 just automates the routing
- **Does not introduce new Q_t fields**: state structure is locked at v4 ship; v4.1 only adds runtime actors that operate on existing state
- **Does not add new economic invariants**: 12 invariants stay; v4.1 may add operational invariants (e.g., quorum freshness) but these are runtime asserts, not on-tape

---

## § 5 v4.1 Scope Risks

| Risk | Likelihood | Mitigation |
|---|---|---|
| Trait signatures need adjustment when implementations are tried | medium | v4 spec § 6 conformance test forces compile-time validation; if rework needed, v4.1.0 plan includes a "trait adjustment" sub-phase |
| L4 `meta_transition` arm conflicts with object-level `step_transition` ordering | medium | v4 `STATE_TRANSITION_SPEC_v1` already factors out 5 transition functions cleanly; v4.1 adds a 6th that doesn't interleave with the 5 existing |
| M-of-N quorum becomes politically contested | high | v4.1 ships with `quorum_required: 1` (single Codex-equivalent runtime judge) by default; user expands later |
| Constitutional path human-sig gate gets bypassed by accident | low | v4.1.C.2 pause-and-prompt is mandatory; cargo-deny rule prevents `apply_approved` for constitutional MetaTx without verified human_signature |

---

## § 6 v4.1 Timeline + Cost (Estimate)

- v4.1.A (8 atoms): ~1.5 weeks
- v4.1.B (6 atoms): ~1.5 weeks
- v4.1.C (5 atoms): ~1 week
- v4.1.D (6 atoms): ~1 week
- Audit + v4.1 exit: ~0.5 week

**Total**: ~5-6 weeks; ~$200-400 audit cost (if reusing v4 tri-model protocol).

---

## § 7 v5 Outlook

v4.1 ends with a working runtime MetaTape. v5 should add:
- Multi-organization support (Hyperledger Fabric chaincode for permissioned settlement; per WP § 17 Phase 4)
- Public AGI Market with cross-domain reputation (per WP § 17 Phase 5)
- Optimistic Rollup batching (per WP § 13)
- ZK / Validity Proof predicates (per WP § 13)

These are scoped out of v4.1 and require entirely separate planning. This doc is closed at "v4.1 = runtime MetaTape on Path B local substrate."

---

## § 8 Honest Acknowledgements

What this plan achieves:
- Closes Plan v3.2 CO P3-prep.6 atom
- Locks v4.1 implementation contract against v4 Phase 3 prep artifacts
- Estimates v4.1 effort honestly (5-6 wk; $200-400 audit)

What this plan is honest about:
- v4.1 timeline is forecasted, not guaranteed
- "M-of-N becomes politically contested" risk has high likelihood + low mitigation; user decides quorum policy before v4.1.B.5
- v4.1.D.2 one-time migration is the most fragile atom — old amendments may have data inconsistencies; migration may fail-and-skip rather than abort

What this plan does NOT do:
- Commit to v4.1 launch date (gated by v4 ship date)
- Pre-decide actor concurrency model (process / thread / async); left to v4.1.A.* design
- Specify communication protocol between actors (in-process channel? message bus?); left to v4.1.A.* design

— ArchitectAI, 2026-04-27
