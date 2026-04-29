# OBS: STATE_TRANSITION_SPEC v1.5 housekeeping issue (2026-04-29)

**Filed by**: ArchitectAI per `CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md` § 0.4 commitment.
**Severity**: hygiene — no behavior impact; documentation alignment only.
**Target spec**: `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md` v1.4 (round-4 PASS/PASS, frozen).
**Policy**: this OBS is an **annotation pass** — it documents the two supersessions enacted by downstream specs but does NOT in-place edit the STATE spec body (which remains frozen at v1.4 PASS/PASS until/unless the curator decides to reopen). Future readers should consult this OBS alongside STATE v1.4 for the current as-implemented reality.

## Supersession 1 — head_t mutation site

**STATE v1.4 § 3 line 412** (and parallel lines § 3.1 line 467 + § 3.2 line 561) reads:
```rust
// STAGE 7: head advance
q_next.head_t = NodeId::from_state_root(new_state_root);
```

**Superseded by**: `CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` v1.2 § 5 K3 v1.2 (round-3 PASS/PASS; commit `a946820`):
> "K3 (v1.2)": `head_t = NodeId(commit_sha)` is the canonical convention WHEN head_t is wired (CO1.7.5+). v1.x sequencer does NOT mutate head_t — `Git2LedgerWriter` is needed to surface commit_sha. **`NodeId::from_state_root(...)` is NOT used by L4 in any version**.

**As-implemented reality** (post-CO1.7-extra Branch A; commit `5ce01b1`):
- Pure transition bodies (when CO1.7.5 lands; currently `Err(NotYetImplemented)` stubs) MUST NOT mutate `q_next.head_t`. `head_t` mutation is exclusively a Sequencer post-commit concern.
- Sequencer post-commit: `src/state/sequencer.rs::apply_one` stage 9 calls `advance_head_t(&mut q_w, &*writer_w)` after `writer.commit` succeeds. `advance_head_t` writes `q.head_t = state::q_state::NodeId(commit_oid_hex)` when `LedgerWriter::head_commit_oid_hex` returns Some (Git2LedgerWriter); leaves `q.head_t` unchanged when None (InMemoryLedgerWriter).
- The `NodeId::from_state_root(...)` accessor at `src/state/q_state.rs:54` exists but is **NOT** used by L4 sequencer in any version. It may be used by other subsystems (e.g., legacy materializer flows), but never as the head_t-binding rule.

**Authority chain**:
- STATE v1.4 § 3 line 412: round-4 PASS/PASS (2026-04-27, prior session)
- CO1.7 v1.2 K3: round-3 PASS/PASS (2026-04-28)
- CO1.7-extra v1.2.1: round-4 PASS/PASS (2026-04-29; this session)
- Asserted principle (CO1.7-extra § 0.4): a later, more specific, audited spec legitimately supersedes an earlier general spec within the layered boundary the later spec covers.

## Supersession 2 — SignalBundle shape

**STATE v1.4 § 3 lines 403-409** (and parallels in § 3.1 / § 3.2) constructs `SignalBundle` as:
```rust
let signals = SignalBundle {
    boolean: vec![Signal::Boolean(BoolSignal::AcceptedAt(tx.tx_id))],
    statistical: vec![
        Signal::Statistical(StatSignal::PriceUpdate(...)),
        Signal::Statistical(StatSignal::ReputationDelta(tx.agent_id, +reputation_delta(tx))),
    ],
};
```

**Superseded by**: `CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md` § 7.2 (PASS/PASS; commit `c1226e2`); shipped at `src/state/typed_tx.rs:830-854`:
```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalKind {
    Empty,
    Finalize { claim_id: ClaimId, reward: MicroCoin },
    TaskExpired { task_id: TaskId, bounty_refunded: MicroCoin },
    TerminalSummary { run_id: RunId, outcome: RunOutcome },
}
```

**As-implemented reality**:
- Shipped `SignalBundle { kind: SignalKind }` has 4 variants only (Empty / Finalize / TaskExpired / TerminalSummary).
- The two-axis `boolean: vec![...] + statistical: vec![...]` shape with `BoolSignal::AcceptedAt / VerifiedAt / ChallengeUpheld` + `StatSignal::PriceUpdate / ReputationDelta` sub-variants is **deferred to CO1.9 (L6 signal indices)**.
- Per CO1.1.4-pre1 doc-comment at `src/state/typed_tx.rs:824-828`: "v1 minimal: a single enum variant per spec call site in § 3 pseudocode (`empty` / `finalize` / `task_expired` / `terminal_summary`). Full L6 signal-stream design is CO1.9."

**Future CO1.7.5 (transition bodies; gated on CO P2.x substrate) emit table** (per CO1.7-extra spec § 0.3.2):

| STATE § 3 transition | Shipped emit |
|---|---|
| step_transition (Work) | `SignalKind::Empty` |
| verify_transition | `SignalKind::Empty` |
| challenge_transition | `SignalKind::Empty` |
| reuse_transition | `SignalKind::Empty` |
| finalize_reward_transition | `SignalKind::Finalize { claim_id, reward }` |
| task_expire_transition | `SignalKind::TaskExpired { task_id, bounty_refunded }` |
| emit_terminal_summary_transition | `SignalKind::TerminalSummary { run_id, outcome }` |

The richer reputation / price / acceptance event variants are deferred; CO1.9 will extend `SignalKind` enum additively (non-breaking change to existing 4 variants' wire format per CO1.1.4-pre1 § 7.2 invariant).

**Authority chain**:
- STATE v1.4 § 3 lines 403-409: round-4 PASS/PASS
- CO1.1.4-pre1: PASS/PASS (5 rounds; commit `c1226e2`; ABI lock)
- CO1.7-extra v1.2.1: round-4 PASS/PASS (carries forward to future CO1.7.5; this session)

## What STATE v1.5 should say (proposed annotation; NOT in-place edit yet)

If/when STATE_TRANSITION_SPEC is reopened for v1.5, the proposed minimal annotation is **one paragraph in the spec preface** (NOT modifying § 3 pseudocode bodies):

```markdown
## Supersessions (v1.5 annotation, 2026-04-29)

The following lines in v1.4 § 3 pseudocode are **historical drafting language**
superseded by downstream audited specs:

1. **§ 3 line 412 + § 3.1 line 467 + § 3.2 line 561** (`q_next.head_t = NodeId::from_state_root(...)`)
   superseded by `CO1.7 v1.2 § 5 K3 v1.2` (round-3 PASS/PASS) +
   `CO1.7-extra v1.2.1 D2` (round-4 PASS/PASS): pure transition bodies do NOT
   mutate head_t; head_t = NodeId(commit_oid_hex) is set by Sequencer
   post-commit via `advance_head_t` helper. NodeId::from_state_root is NOT used
   by L4 in any version. See `OBS_STATE_TRANSITION_SPEC_V1_5_HOUSEKEEPING_2026-04-29.md`.

2. **§ 3 lines 403-409 + parallels in § 3.1, § 3.2** (`SignalBundle { boolean: vec![...], statistical: vec![...] }`)
   superseded by `CO1.1.4-pre1 § 7.2` (PASS/PASS; ABI lock): shipped
   `SignalBundle { kind: SignalKind }` has 4 minimal variants
   (Empty/Finalize/TaskExpired/TerminalSummary). BoolSignal/StatSignal richness
   deferred to CO1.9 (L6 signal indices). See same OBS.
```

The STATE spec § 3 pseudocode bodies themselves remain unchanged at v1.4 (frozen) — they document the conceptual model; the implementation reality is in shipped code + downstream specs. Reopening STATE for these annotations is the curator's decision.

## Recommendation

- **Curator decision**: keep STATE v1.4 frozen as historical drafting; this OBS is the authoritative pointer for as-implemented reality.
- **Or**: open a small STATE v1.5 patch with the annotation above + audit-light review (the supersessions are already PASS/PASS in their downstream specs; v1.5 annotation just cross-references).

ArchitectAI's v1.5 housekeeping commitment per CO1.7-extra § 0.4 is **fulfilled by this OBS** (not requiring an in-place STATE spec edit — though if curator prefers, the proposed v1.5 annotation text above is ready for application).

**FC-trace**: FC3-L4 (downstream supersession sediment for L4 transition ledger family; alignment-only observation, not a behavior change).
