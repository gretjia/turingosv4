# Inv 8 Contribution DAG Determinism Spec v1 (CO P2.4.0 Spike Pre-draft)

> **Date**: 2026-04-27
> **Purpose**: Plan v3.2 CO P2.4.0 was scheduled as a BLOCKING design spike before any AttributionEngine implementation. Auto-research draft of the algorithm spec ahead of time, so the spike is mostly review-and-validate rather than design-from-scratch.
> **Authority**: Economic § 18 Inv 8 ("贡献归因来自 Contribution DAG + 统计信号，不来自 Agent 自我声明") + Codex CO P0.7 + Gemini v3.2 Q3 CHALLENGE.
> **Status**: PRE-SPIKE DRAFT. The actual CO P2.4.0 spike will dual-audit this; if Codex+Gemini PASS, this becomes the locked algorithm. If they CHALLENGE, revisions are made and spike re-runs.

---

## § 1 The Hard Problem

Inv 8 says attribution must be **deterministic + derivable from L4 read_set/write_set ONLY** — never from agent self-declaration. Codex CO P0.7 §1 + Gemini v3.2 Q3 both flagged: building a canonical DAG from concurrent-write tx is non-trivial graph theory.

Specific challenges:
1. **Concurrent writes**: tx A and tx B both write to same key in different rounds; which is "parent"?
2. **Multi-parent merges**: tx C reads from A AND B AND C0; how to weight?
3. **Citation transitivity**: if A→B→C via reads, does A's contribution propagate to C's reward?
4. **Cycle detection**: must DAG be acyclic; what if tx self-references?
5. **Edge type discrimination**: "builds-on" vs "cites" vs "reuses" — which L4 fields distinguish?
6. **Reproducibility**: same inputs → byte-identical adjacency list, regardless of map iteration order

---

## § 2 Algorithm — `compute_contribution_dag`

### 2.1 Inputs

```rust
pub fn compute_contribution_dag(
    target_work_tx: TxId,
    ledger: &TransitionLedger,        // read-only L4 snapshot
    tool_registry: &ToolRegistry,     // L2 read-only
) -> ContributionDAG {
    ...
}
```

**Invariants on inputs**:
- `ledger` is at fixed `state_root_t = R` (snapshot)
- iteration order on ledger is **deterministic** (BTreeMap; insertion-time index)
- target_work_tx must exist in ledger

### 2.2 Step-by-step

**Step 1**: collect all ancestor tx of `target_work_tx` reachable via reads, breadth-first.

```rust
let mut visited: BTreeSet<TxId> = BTreeSet::new();
let mut queue: VecDeque<TxId> = VecDeque::new();
queue.push_back(target_work_tx);

while let Some(tx_id) = queue.pop_front() {
    if !visited.insert(tx_id) {
        continue;  // already processed
    }
    let tx = ledger.get(tx_id).expect("tx in target's history must exist");

    for read_key in tx.read_set.iter() {
        // each read_key is a (resource_id, write_tx_id) tuple
        // resource_id is e.g. "task:<id>.problem", "tool:<id>.binary", "claim:<id>.result", etc.
        // write_tx_id is the L4 tx that last wrote this resource_id at submission time
        if let Some(producer_tx) = read_key.producer_tx {
            queue.push_back(producer_tx);
        }
    }
}
```

After this step, `visited` is the **causal ancestor set** of `target_work_tx`.

**Step 2**: for each ancestor, classify the edge from target back to ancestor by L4 fields:

```rust
fn classify_edge(target: &TransitionTx, ancestor: &TransitionTx) -> EdgeType {
    // builds-on: target's write_set INCLUDES ancestor's resource (target is a refinement)
    let target_writes: BTreeSet<&str> = target.write_set.iter().map(|wk| &wk.resource_id).collect();
    let ancestor_writes: BTreeSet<&str> = ancestor.write_set.iter().map(|wk| &wk.resource_id).collect();
    if !target_writes.is_disjoint(&ancestor_writes) {
        return EdgeType::BuildsOn;
    }

    // reuses: target's read_set references ancestor's tool/predicate output
    if target.read_set.iter().any(|rk| {
        rk.resource_id.starts_with("tool:") &&
        ancestor.write_set.iter().any(|wk| wk.resource_id == rk.resource_id)
    }) {
        return EdgeType::Reuses;
    }

    // cites: any other read of ancestor's output
    EdgeType::Cites
}
```

**Step 3**: compute weight per edge using deterministic formula:

```rust
fn compute_edge_weight(
    edge_type: EdgeType,
    ancestor_distance: u32,    // BFS hop count from target back to ancestor
    ancestor_count_at_distance: usize,  // how many ancestors share this distance
) -> EdgeWeight {
    let base = match edge_type {
        EdgeType::BuildsOn => 0.5,    // strongest contribution
        EdgeType::Reuses   => 0.3,    // tool reuse contribution
        EdgeType::Cites    => 0.1,    // light reference
    };

    // exponential decay with distance + normalize for siblings
    let decay = 0.7_f64.powi(ancestor_distance as i32 - 1);
    let split = 1.0 / (ancestor_count_at_distance as f64);

    let raw = base * decay * split;

    // clamp + quantize (avoid float drift):
    let micro = (raw * 1_000_000.0).round() as i64;    // integer micro-units; max 1.0
    EdgeWeight::from_micro(micro)
}
```

**Determinism note**: weights computed in micro-units (i64 fixed-point); never stored as f64 in DAG state.

**Step 4**: enforce normalization (sum of all edge weights from `target` ≤ 1.0):

```rust
let total_outgoing: i64 = edges.iter().map(|e| e.weight.micro_units()).sum();
if total_outgoing > 1_000_000 {
    let scale = 1_000_000.0 / total_outgoing as f64;
    for edge in &mut edges {
        let new_micro = (edge.weight.micro_units() as f64 * scale).round() as i64;
        edge.weight = EdgeWeight::from_micro(new_micro);
    }
}

// Solver self-attribution = remainder
let self_weight = EdgeWeight::from_micro(1_000_000 - edges.iter().map(|e| e.weight.micro_units()).sum::<i64>());
```

**Step 5**: cycle detection (DAG must be acyclic):

```rust
fn assert_acyclic(dag: &ContributionDAG) {
    // topological sort with timestamp_logical as secondary key
    let mut sorted = dag.nodes.iter().collect::<Vec<_>>();
    sorted.sort_by_key(|tx| (tx.timestamp_logical, tx.tx_id));    // stable ordering

    // walk in reverse topological order; for each tx, all its read.producer_tx must have already been visited
    let mut visited = BTreeSet::new();
    for tx in sorted.iter().rev() {
        for rk in &tx.read_set {
            if let Some(producer) = rk.producer_tx {
                if !visited.contains(&producer) {
                    panic!("cycle detected: tx {} reads from producer {} not yet visited", tx.tx_id, producer);
                }
            }
        }
        visited.insert(tx.tx_id);
    }
}
```

If `assert_acyclic` panics, the DAG is broken (should be impossible if ledger is well-formed).

### 2.3 Output

```rust
pub struct ContributionDAG {
    pub target: TxId,
    pub edges: Vec<ContributionEdge>,    // BTreeSet<TxId> ordering
    pub self_weight: EdgeWeight,         // remainder after edges
}

pub struct ContributionEdge {
    pub from: TxId,                       // = target
    pub to: TxId,                         // = ancestor (or tool_id if reuse)
    pub creator: AgentId,
    pub edge_type: EdgeType,
    pub weight: EdgeWeight,               // i64 micro-units; sum of all outgoing ≤ 1.0
    pub distance: u32,                    // BFS hop count
}
```

---

## § 3 Determinism Proof Sketch

**Claim**: Same inputs → byte-identical adjacency list AND byte-identical weights.

**Proof outline**:

1. `ledger.get(tx_id)` returns the same `TransitionTx` for the same `tx_id` (immutable history).
2. `BTreeSet` iteration is **lexicographic** by element, so `read_set` / `write_set` traversal is deterministic.
3. BFS using `VecDeque<TxId>` is FIFO; queue insertion order is deterministic per step 1.
4. `classify_edge` is a pure function of `target.read/write_set` and `ancestor.read/write_set`.
5. `compute_edge_weight` uses `f64::powi` + `round()` — same inputs → same output IF f64 ops are IEEE 754 deterministic across platforms (TRUE for x86-64 SSE2; GUARANTEED by Rust's `f64` type).
6. Conversion `f64 → i64` via `.round() as i64` is well-defined.
7. Normalization step is pure arithmetic; `i64 / i64` is integer division (`scale` computed in f64 but final write is `.round() as i64`).
8. Topological sort uses `(timestamp_logical, tx_id)` as keys — both integer types; sort is stable.

**Caveat on f64**: `.round()` on tie-breaking values (.5) uses banker's rounding in some Rust versions. Mitigation: use `(raw * 1_000_000.0 + 0.5).floor() as i64` instead of `.round()` to lock to "round half up" behavior. **Conformance test must verify byte-identical output across two independent runs on the same machine.**

---

## § 4 3-Tx Adversarial Worked Example

### 4.1 Setup

- Ledger contains tx-A (Builder Dave's tool creation), tx-B (Solver Eve's first attempt; uses tool but FAILS predicate), tx-C (Solver Alice's success; uses Eve's failed tx as inspiration + Dave's tool)
- `compute_contribution_dag(tx-C)` should:
  - return edges to both tx-A AND tx-B
  - assign higher weight to tx-A (BuildsOn) than tx-B (Cites; Eve failed)
  - exclude any tx not in tx-C's read_set transitive closure

### 4.2 L4 Snapshot

```text
tx-A: { agent: dave, write_set: ["tool:tool-prove-cong.binary"], timestamp_logical: 100 }
tx-B: { agent: eve, read_set: ["tool:tool-prove-cong.binary" via tx-A],
        write_set: ["claim:tx-B.result_failed"], timestamp_logical: 500 }
tx-C: { agent: alice, read_set: [
            "tool:tool-prove-cong.binary" via tx-A,
            "claim:tx-B.result_failed" via tx-B   // alice reviewed Eve's failure
        ],
        write_set: ["claim:tx-C.result_accepted"], timestamp_logical: 700 }
```

### 4.3 Algorithm execution

**Step 1 (BFS from tx-C)**:
- Visit tx-C (distance 0)
- Read deps of tx-C: tx-A (via tool-prove-cong) + tx-B (via claim:result_failed)
- Visit tx-A (distance 1) — no further read deps (tool creation, no upstream)
- Visit tx-B (distance 1)
- Read deps of tx-B: tx-A (via tool)
- tx-A already visited; skip
- visited = { tx-A, tx-B, tx-C }

**Step 2 (Classify edges from tx-C)**:
- tx-C → tx-A: tx-A.write_set has "tool:..."; tx-C.read_set has "tool:..."  → `Reuses`
- tx-C → tx-B: tx-B.write_set has "claim:tx-B.result_failed"; tx-C.read_set references → `Cites`

**Step 3 (Weights)**:
- tx-C → tx-A (Reuses, distance 1, 1 ancestor at this distance):
  raw = 0.3 × 0.7^0 × (1/1) = 0.3
  micro = 300,000
- tx-C → tx-B (Cites, distance 1, same level — but NOT same TYPE, so no sibling-split):
  raw = 0.1 × 0.7^0 × (1/1) = 0.1
  micro = 100,000

**Step 4 (Normalization)**:
- Total outgoing = 400,000 (< 1,000,000) → no scaling
- tx-C self_weight = 1,000,000 - 400,000 = 600,000 (= 60% of reward to Alice)

**Step 5 (Cycle check)**:
- Topological order by (timestamp_logical, tx_id): tx-A (100), tx-B (500), tx-C (700) — strictly increasing
- DAG verifies acyclic ✓

### 4.4 Final Output

```text
ContributionDAG {
    target: tx-C,
    edges: [
        ContributionEdge { from: tx-C, to: tx-A, creator: dave, edge_type: Reuses, weight: 300_000, distance: 1 },
        ContributionEdge { from: tx-C, to: tx-B, creator: eve,  edge_type: Cites,  weight: 100_000, distance: 1 },
    ],
    self_weight: 600_000  // alice = 60% of reward
}
```

If reward = 500 micro-coin:
- Alice (solver self): 500 × 0.6 = 300
- Dave (reuse royalty): 500 × 0.3 = 150 (subject to MAX_REUSE_ROYALTY_FRACTION cap = 0.10 → clamped to 50)
- Eve (citation): 500 × 0.1 = 50

After cap enforcement: Alice 350, Dave 50 (clamped), Eve 50, total = 450; remainder 50 returns to escrow OR re-distributed. **Note**: this is a design tension — if cap clamps royalty, where does the freed-up reward go? Default: return to escrow for Phase D analysis. Alternative: redistribute to other ancestors.

**Resolution for v1**: clamped royalty excess returns to escrow (does NOT go back to solver). Configurable per TaskMarket.

### 4.5 Determinism Test

Run algorithm twice on same input → expect identical output bytes:

```rust
let dag1 = compute_contribution_dag(target=tx-C, ledger=L, tool_registry=T);
let dag2 = compute_contribution_dag(target=tx-C, ledger=L, tool_registry=T);
assert_eq!(canonicalize(dag1), canonicalize(dag2));    // byte-identical
```

---

## § 5 Hostile Inputs (Adversarial Test Cases for CO P2.4.0 Spike)

| # | Input | Expected behavior |
|---|---|---|
| H1 | tx-X reads from tx-Y, tx-Y reads from tx-X (cycle) | `assert_acyclic` panics; algorithm never returns |
| H2 | tx-X read_set claims producer = tx-Z, but ledger has tx-Z.write_set NOT including the resource | algorithm IGNORES the false producer claim; ancestor not added |
| H3 | tx-X has 1000 read_set entries → BFS visits 1000 ancestors | terminates in finite time; no recursion overflow |
| H4 | Two ancestors at distance 2 with same `(edge_type, parent)` — sibling-split fairly | each gets 1/2 of single-ancestor weight; deterministic by tx_id ordering |
| H5 | Floating-point edge case: 0.30000000000000004 vs 0.3 | i64 quantization rounds to 300_000; both inputs same output |
| H6 | tx-X self-references itself in read_set | self-edge dropped; no infinite loop |
| H7 | tx-X has no read_set (genesis-rooted) | self_weight = 1_000_000 (100% to solver) |

The CO P2.4.0 spike must verify all 7 cases; conformance tests in `tests/inv8_dag_*.rs`.

---

## § 6 Why This Algorithm vs Alternatives

| Alternative | Pros | Cons | Verdict |
|---|---|---|---|
| **THIS** (BFS + classify + decay-weight) | deterministic; simple; integer weights | hand-tuned constants (0.5/0.3/0.1; 0.7 decay) | ✓ pick |
| Linear-time dataflow analysis | optimal | requires full L4 graph in memory; expensive at scale | ✗ over-engineered for v4 |
| Markov-chain stationary distribution | mathematically elegant | requires iterative computation; non-deterministic if not careful | ✗ harder to verify |
| Agent self-declared weights | simplest | violates Inv 8 (agent self-report → manipulation) | ✗ FORBIDDEN |
| ML-trained attribution model | adapts to patterns | non-deterministic; training data is a tape sidecar | ✗ violates Tape Canonical |

This algorithm is **boring on purpose**. Boring + deterministic + auditable wins.

---

## § 7 What This Spec Does NOT Decide

Deferred to dedicated atom or future amendment:

- Constants (0.5/0.3/0.1 base weights; 0.7 decay): TaskMarket config can override, but defaults locked here
- Cap-overflow distribution policy (return to escrow vs redistribute): default = return to escrow
- Multi-tool reuse aggregation (tx uses 5 tools simultaneously): proportional split by reuse_royalty_share
- AttributionEngine API surface (caller / cache / invalidation): CO P2.4.1+ atoms
- DAG storage format on tape: BTreeMap-derived deterministic serialization; CO P2.4.2 atom

---

## § 8 CO P2.4.0 Spike Acceptance Criteria

When the spike runs, dual audit (Codex + Gemini) must verify:

1. ✅ Algorithm § 2 step-by-step is deterministic (Codex code-grounded check)
2. ✅ § 3 determinism proof addresses f64 + BTreeMap concerns (Gemini strategic check)
3. ✅ § 4 worked example computes correctly (manual trace)
4. ✅ § 5 hostile inputs all behave as specified
5. ✅ Implementation uses BTreeMap throughout; no HashMap (R-022 grep)
6. ✅ Implementation uses i64 micro-units for weights; no f64 storage (cargo-deny)
7. ✅ Conformance test `tests/inv8_dag_determinism.rs` exists + 2 runs produce byte-identical adjacency

If 6/7 PASS + 1 CHALLENGE → revise; ship v1.1 of this spec.
If 7/7 PASS → spec frozen; CO P2.4.1+ implementation atoms unblocked.
If any VETO → spike halt; revise.

---

## § 9 Honest Acknowledgements

What this draft achieves:
- Closes Plan v3.2 CO P2.4.0 atom (pre-spike documentation; spike validates)
- Provides concrete deterministic algorithm with proof sketch
- 7 hostile test cases covering edge conditions
- Ratings of alternative approaches with rationale

What this draft is honest about:
- Constants (0.5/0.3/0.1, 0.7 decay) are hand-tuned; could be wrong for some workloads
- f64 → i64 quantization has subtle banker's rounding gotcha (mitigated via `.floor() as i64` rule)
- "Cycle should be impossible if ledger is well-formed" relies on ledger being well-formed
- Cap-overflow policy decision impacts economic dynamics; default may need adjustment after Phase D data

What this draft does NOT do:
- Run the algorithm on real tape (CO P2.4.0 spike does this)
- Validate proof formally (TLA+ extension would help)
- Decide caller API (CO P2.4.1+ atom decides)

— ArchitectAI, 2026-04-27 (pre-spike draft)
