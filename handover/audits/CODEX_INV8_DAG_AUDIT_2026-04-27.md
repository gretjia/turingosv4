## Q1-Q10 verdicts

Q1: CHALLENGE — §2.2 relies on BTreeSet/BTreeMap order (lines 38-40, 58-64, 191-193) and never specifies final edge sorting/canonicalization despite `edges: Vec` (lines 166-179); byte identity under arbitrary map iteration is SILENT.

Q2: VETO — The 7 cases exist (lines 292-304) but are not closed: H2 says false producers ignored while Step1 enqueues `producer_tx` without write validation (lines 58-64, 297), and H6 says self-edge dropped with no drop rule (lines 301, 52-66).

Q3: VETO — SILENT on a branch-independent parent for concurrent same-key writers; Step1 trusts `read_key.producer_tx` (lines 58-63), while State §5.2.6 only guarantees replay from the SAME logical_t sequence (STATE lines 1001-1008). Witness: A/B concurrently write K, C reads K; A<B makes parent B, B<A makes parent A.

Q4: VETO — Multi-parent weighting contradicts itself: §2.2 splits by `ancestor_count_at_distance` (lines 100-112), but §4.3 gives two distance-1 parents no sibling split (lines 242-248). Witness A(reuse)+B(cite) yields 150k/50k by §2.2 vs 300k/100k by §4.3; rounding-conservation after scaling is SILENT (lines 126-136).

Q5: CHALLENGE — BFS + visited gives finite transitive reachability (lines 45-69), but distance is required for weights (line 100) and never recorded, so A→B→C propagation weight is underspecified.

Q6: VETO — `assert_acyclic` reverses timestamp order and requires older producers already visited (lines 142-157), which panics on a valid C reads B reads A chain; H1/H6 expectations (lines 296, 301) do not match the algorithm.

Q7: CHALLENGE — The fields are named (`read_set.resource_id/producer_tx`, `write_set.resource_id`; lines 58-63, 74-91), but BuildOn overmatches same-resource writes without proving a read edge (lines 75-79), and `tool_registry` is unused (lines 31-32).

Q8: CHALLENGE — The plan is only two same-input runs plus named tests (lines 280-288, 304, 338-345); fuzz/diff dimensions for read permutation, concurrent ordering, serialization bytes, and normalization residuals are SILENT.

Q9: CHALLENGE — Seven adversarial cases are present (lines 292-304), but the algorithm spans §2.1-§2.2 lines 26-162 plus caveats, so the 1-page algorithm form is not met.

Q10: VETO — Not ready: edge order, distance bookkeeping, producer validation, cycle handling, concurrent parent tie-break, and reward residual policy remain ambiguous or contradictory (lines 69-71, 95-136, 139-162, 276-278, 330).

## Holistic verdict
VETO
The draft captures the right problem space, but the executable algorithm is internally inconsistent and fails core determinism/conservation cases. It must be revised before CO P2.4.1 implementation.

## Must-fix (if any)
1. Add a one-page normative algorithm with explicit edge order, distance map, and canonical serialization.
2. Define branch-independent same-key concurrent-writer parent selection or bind DAG inputs to one frozen logical_t sequence.
3. Validate `producer_tx` actually wrote the read resource; reject or ignore invalid reads consistently.
4. Replace weighting with integer-only math, explicit grouping, and exact residual distribution.
5. Rewrite cycle detection against producer→consumer order and specify self-reference behavior.
6. Tighten edge type rules to require direct read evidence and define L4 resource namespaces.
7. Expand tests to permutation/differential fuzzing for maps, concurrent witnesses, cycles, and rounding.
