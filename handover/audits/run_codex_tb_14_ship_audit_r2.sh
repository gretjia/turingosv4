#!/usr/bin/env bash
# Codex TB-14 Atom 6 ship audit — ROUND 2 (post-B′ canonical-graph rewire).
# Per architect ruling 2026-05-03 §7: "Re-run Codex R2 only after #2 is
# fixed in production semantics." All B′ steps 1-6 land at HEAD 07ce9b8;
# canonical masking is functional in production per the 5 new chain-backed
# smokes in tests/tb_14_canonical_masking_smoke.rs.

set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
ROUND="${TB14_AUDIT_ROUND:-R2}"
OUT="${ROOT}/handover/audits/CODEX_TB_14_SHIP_AUDIT_2026-05-03_${ROUND}.md"
TMP_PROMPT="$(mktemp /tmp/tb14_codex_ship_r2.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

if [ -e "$OUT" ]; then
  echo "[codex tb-14 r2] error: $OUT already exists; refusing to overwrite" >&2
  exit 2
fi

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex TB-14 Atom 6 Ship Audit — ROUND 2 (post-B′ canonical-graph rewire)

**Role**: skeptical adversarial implementer-reviewer. Class 3 dual audit
R2 per architect ruling 2026-05-03 §7. Independent of Gemini R2 (parallel,
architectural strategic angle). Per `feedback_dual_audit_conflict`: VETO >
CHALLENGE > PASS.

## Round 1 outcome (mandatory context)

R1 VERDICT: **VETO**, conviction=high, recommendation=REDESIGN before ship.

R1 PRIMARY VETO finding:
  RQ4/RQ8/Q3 — production wire-up mixed canonical WorkTx `TxId` PriceIndex
  entries with shadow `kernel.tape` node ids, making `mask_set` non-
  functional over real production edges and causing selected parents to
  become dangling shadow citations. (Two distinct defects: dangling-citation
  crash via bus.append, and silently-empty mask_set in production.)

R1 SECONDARY CHALLENGE finding:
  `BoltzmannMaskPolicy::from_env()` accepted nonsensical production
  values (negative `BOLTZMANN_MIN_LIQUIDITY_MICRO`; zero
  `BOLTZMANN_PRICE_MARGIN_DEN`).

User-architect ruling 2026-05-03 (binding): proceed-as-C→B′:
  1. Immediately fix #1 (bus.append parent canonical-vs-shadow).
  2. Immediately fix #3 (env validation).
  3. Amend TB-14 charter (canonical namespace decision §3 binding).
  4. Implement CanonicalNodeGraph (canonical-graph rewire §4 binding).
  5. Add positive production-controlled smoke (§5 verbatim).
  6. Add negative production-controlled smokes (§6 verbatim).
  7. Re-run Codex R2 only after #2 is fixed in production semantics.

Lossless ruling archive:
`handover/directives/2026-05-03_TB14_ATOM6_VETO_RULING.md`.

## R2 audit target — HEAD 07ce9b8

```text
TB-14 Atom 6 main:    44cd480  production wire-swap + legacy CPMM excision
                      38412bf  internal auditor F1 (dead BusResult::Invested)
                      c291dde  LATEST.md update
TB-14 Atom 6 B′:      48e84ee  step 1+2 surgical fixes
                      dd40052  step 3 charter amend (canonical namespace)
                      9daba5a  step 4 CanonicalNodeGraph + compute_mask_set rewire
                      07ce9b8  step 5+6 production-controlled chain-backed smokes

HEAD (07ce9b8): cargo test --workspace = 839 passed / 0 failed / 150 ignored
                6/6 architect §5.7 halt-triggers GREEN
                ChainTape smoke (chain-backed) PASS
                5 NEW chain-backed canonical-masking smokes PASS
```

## R2 closure mandate (per architect ruling §3-§6)

The R2 audit MUST verify each of the four R1 closures. Cite file:line
for every finding.

### Closure 1 — defect #1: bus.append parent canonical-vs-shadow id namespace fix

**R1 finding**: `experiments/minif2f_v4/src/bin/evaluator.rs:1612`
unwrapped canonical TxId from `boltzmann_select_parent_v2(...)` and
passed it to `bus.append(...)` at line 1753; kernel.tape uses shadow IDs
`tx_{count}_by_{author}` so the canonical TxId became a dangling citation.

**B′ step 1 closure** (commit 48e84ee): the v2 selector still runs (its
result captured as `_v2_canonical_pick` for observability + future
canonical wire-up), but its output is explicitly NOT passed to bus.append.
Per architect step 1: "Use None unless a real shadow id exists." No
canonical→shadow id mapping is currently available; pass None.

**Verify**:
  - `experiments/minif2f_v4/src/bin/evaluator.rs:~1604-1631` — confirm
    `let parent: Option<String> = None;` is the actual binding passed
    to `bus.append(agent_id, payload, parent.as_deref())` at the
    downstream call site.
  - The legacy `bus.append` shadow-tape parent now always receives None.
  - The canonical WorkTx submission (lines ~1660-1742) is unaffected
    — its `parent_tx` continues to come from `last_tx_by_agent.get(agent_id)`
    (TB-7.7 D2 per-agent linear chain). Routing the v2 result to
    drive canonical `parent_tx` is a separate charter-level question
    for a future atom.

### Closure 2 — defect #2: CanonicalNodeGraph rewire (THE LOAD-BEARING FIX)

**R1 finding**: `bus.snapshot()` computed mask_set against `self.kernel.tape`
(shadow-id namespace) but `price_index` was keyed by canonical TxIds —
so `tape.children(canonical_id)` always returned empty in production.
Mask was empty in production despite the unit tests passing.

**B′ step 4 closure** (commit 9daba5a):
  (a) NEW `pub type CanonicalNodeGraph = BTreeMap<TxId, BTreeSet<TxId>>`
      in `src/state/price_index.rs`. Canonical-keyed parent → children
      edge map.
  (b) `compute_mask_set` signature change:
        PRE  : (econ, &Tape, policy, &PriceIndex)
        POST : (econ, &CanonicalNodeGraph, policy, &PriceIndex)
      Body: `tape.children(parent_id.0.as_str())` → `edges.get(parent_id)`.
  (c) NEW `Sequencer::compute_canonical_edges_at_head()` walks L4 +
      reads CAS-resident `ProposalTelemetry.parent_tx` for each accepted
      WorkTx via `WorkTx.proposal_cid` → builds the canonical edge map.
  (d) `bus.snapshot()` calls `seq.compute_canonical_edges_at_head()` and
      passes the result to `compute_mask_set` in place of `&self.kernel.tape`.

**Verify**:
  - `src/state/price_index.rs` — `CanonicalNodeGraph` type alias + new
    `compute_mask_set` signature + body uses `edges.get(parent_id)` not
    `tape.children`.
  - `src/state/sequencer.rs` — `compute_canonical_edges_at_head` impl is
    sound: walks L4 via `writer_r.read_at(t)`, decodes TypedTx::Work,
    reads ProposalTelemetry from CAS via `work.proposal_cid`, captures
    `tel.parent_tx` into edge map. Halt-trigger #2 fence preserved
    (NO TB-14 imports added to sequencer.rs `use` block; the new
    method body uses `crate::bottom_white::ledger::transition_ledger::canonical_decode`
    + `crate::runtime::proposal_telemetry::read_from_cas` — both NON-TB-14).
  - `src/bus.rs::snapshot` — calls `seq.compute_canonical_edges_at_head()`
    and threads `&edges` into `compute_mask_set`.

### Closure 3 — defect #3: BoltzmannMaskPolicy::from_env validation

**R1 finding**: from_env accepted negative min_liquidity, zero price_margin
denominator (interacted badly with saturating_sub).

**B′ step 2 closure** (commit 48e84ee): per-field validation rules
    - min_liquidity > 0           (non-positive → default)
    - price_margin > 0            (zero numerator OR denominator → default)
    - beta_den > 0                (zero → default)
    - beta_num >= 0               (negative → default)
    - epsilon in [0, 1]           (den > 0 AND num ≤ den; otherwise → default pair)

**Verify**:
  - `src/state/price_index.rs::from_env` — confirm each rule is
    implemented (per-field gating after parse, fall back to default
    on invalid).
  - `src/state/price_index.rs` inline tests — 11 new tests pin the
    rules + boundary semantics (zero-beta_num accepted; epsilon=0
    accepted; epsilon=1 accepted; epsilon>1 rejected).

### Closure 4 — production semantic witnesses (architect §5+§6)

**B′ step 5+6 closure** (commit 07ce9b8): NEW
`tests/tb_14_canonical_masking_smoke.rs` — 5 chain-backed tests
(Sequencer::apply_one + on-disk LedgerEntry, NOT stdout-only):

  1. b_prime_step_5_positive_canonical_masking_smoke — parent A +
     child B with parent_tx=A; child stake 5_000_000 > min_liquidity
     1_000_000; mask_set returns {A} under permissive policy;
     canonical L4 still contains A.
  2. b_prime_step_6a_low_liquidity_child_cannot_mask_parent — child
     stake 100 micro << min_liquidity 1_000_000; mask_set empty.
  3. b_prime_step_6b_unresolved_challenged_child_cannot_mask_parent —
     Open ChallengeCase against B injected; mask_set empty.
  4. b_prime_step_6c_predicate_failed_child_cannot_mask_parent —
     B's WorkTx submitted with predicate_passes=false → routed to
     L4.E (rejected) → does NOT appear in canonical_edges_at_head;
     mask_set empty.
  5. b_prime_canonical_edges_idempotent — 5 repeated calls produce
     byte-equal `BTreeMap<TxId, BTreeSet<TxId>>` (Art.0.2 derived-view
     determinism).

**Verify**:
  - All 5 smokes use the production code path
    (Sequencer::compute_canonical_edges_at_head + compute_mask_set on
    a real Harness with InMemoryLedgerWriter + real CasStore + real
    AgentKeypairRegistry).
  - Smokes run via `cargo test --test tb_14_canonical_masking_smoke`
    — chain-backed per `feedback_smoke_evidence_naming`.
  - The positive smoke (§5) uses a permissive policy (price_margin
    = 0/1) for the V0 wire-up check. Architecturally explain whether
    this is sound or whether you'd recommend a follow-up smoke with
    default policy + a Long-Short configuration that produces a real
    dominance gap. Take a position; do not OBS-defer.

## RQ8 follow-up (R1 explicitly flagged)

R1 RQ8 said: "TB-14 ChainTape smoke coverage gap — the smoke uses the
TB-13 CompleteSet flow only (no NodePosition mutation), so the
resulting PriceIndex is empty. ... Should there be a non-empty
NodePositions chain-backed smoke...?"

R2 must verify: tests/tb_14_canonical_masking_smoke.rs IS that non-
empty NodePositions chain-backed smoke. Confirm coverage closes the
RQ8 gap. (The TB-13 chaintape smoke at tests/tb_14_chaintape_smoke.rs
remains intact for the empty-PriceIndex replay-determinism witness;
the new smoke covers the non-empty case.)

## Architect §8 split-fallback decision

Architect ruling §8: "If you cannot finish canonical masking now,
split TB-14: TB-14a PriceIndex-only. TB-14b Boltzmann canonical masking.
But do not claim TB-14 PriceIndex + Boltzmann Masking shipped while
mask_set is empty in production."

R2 mandate: take an explicit position on whether the split is
triggered. The AI-coder's view at HEAD 07ce9b8: split is NOT
triggered — mask_set is functional in production per the 5 chain-
backed smokes. TB-14 PriceIndex + Boltzmann Masking ships under a
single charter. If you disagree (e.g., you find another production-
semantic gap), VETO with specific evidence; do not fence-sit.

## All R1 + R2 mandate questions (compressed)

  Q1 (CR-14.1): predicate-blind sequencer? — halt-trigger #1 GREEN.
  Q2 (CR-14.2): predicate-blind L4 classification? — halt-trigger #2 GREEN.
  Q3 (CR-14.3): masked parents still in canonical state? — verified
       at the new canonical-graph + price_index level (not the shadow
       Tape, which is no longer in the masking path).
  Q4 (CR-14.4): low-liquidity guard — covered by §6a smoke.
  Q5 (CR-14.5): open-challenge guard — covered by §6b smoke.
  Q6 (CR-14.6): Goodhart shield — NodeMarketEntry unchanged from R1.
  Q7 (G-14.11): no f64 in TB-14 surface — closed by Atom 6 main +
       internal auditor F1 + B′ step 4 (no new f64 added).
  Q8 (Art.0.2): replay determinism — canonical_edges_at_head is a
       pure function of (L4, CAS); both replay-deterministic per TB-13
       chaintape evidence. Idempotency witness in §5+§6 smokes.
  Q9 (charter §5.6 forbidden): no market trading / settlement /
       parent deletion / AMM / DPMM / price-as-oracle in B′ commits.

  RQ-R2.1: All four R1 defects closed in production semantics?
  RQ-R2.2: Split-fallback (architect §8) triggered or NOT?
  RQ-R2.3: Any new defects introduced by B′ steps 4-6 that R1 didn't
           catch (e.g., halt-trigger #2 leak from new sequencer
           imports; canonical-graph builder edge cases; smoke
           assertion soundness)?

## Verdict format

End with:

```text
## VERDICT: PASS
(R1 VETO closed; B′ steps 1-6 sound; production semantics witnessed;
ship Atom 7 with confidence.)
```

```text
## VERDICT: CHALLENGE
- <claim id> CHALLENGE: <one-line reason + line refs>
(round-3 requires user authorization per kickoff doc round-cap=2 +
feedback_elon_mode_policy.)
```

```text
## VERDICT: VETO
- <claim id> VETO: <one-line BLOCKING reason + line refs>
(R2 VETO triggers escalate-to-user per kickoff doc Stop conditions.)
```

Include conviction (low/medium/high) + recommendation (PROCEED to SHIP /
FIX-THEN-PROCEED / REDESIGN / SPLIT TB-14a + TB-14b).

Save to: handover/audits/CODEX_TB_14_SHIP_AUDIT_2026-05-03_R2.md.

BRIEF_EOF

echo "  Codex audit prompt prepared at: $TMP_PROMPT" >&2
echo "  Output target: $OUT" >&2
echo "  Round: $ROUND" >&2
echo "  Invoking codex exec..." >&2

cat "$TMP_PROMPT" | codex exec --skip-git-repo-check --sandbox read-only --color never - > "$OUT.raw" 2>&1
EXIT=$?

if [ $EXIT -ne 0 ]; then
  echo "  codex exec returned exit code $EXIT" >&2
fi

mv "$OUT.raw" "$OUT"
echo "  Audit saved: $OUT" >&2
exit $EXIT
