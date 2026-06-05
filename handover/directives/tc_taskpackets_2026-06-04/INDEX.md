# TuringOS-TC TaskPacket Index

Status: active orchestrator index for OBL-014.

> For agentic workers: use exactly one packet at a time. Do not infer scope from
> neighboring packets. Do not edit files outside the packet's allowed write set.

Goal: execute TC-000..TC-021 from clean base
`origin/main@39233aa7c868f0e9b37a7a29eb426279f41cf032` using atomized
TaskPackets, independent ship gates, and clean-context audit.

Architecture: TuringOS-TC is a Git/CAS/tape OS-substrate program. Lean is a
feature-layer theorem workload/verifier, not the TuringOS kernel. Market, LLM,
and autonomous routing may accelerate odd-lane priority only; they are never
truth, rank, predicate, or verifier authority.

Tech stack: Rust workspace, git2-backed ChainTape/TDMA refs, JSON/CAS evidence,
existing `turingos` CLI gates, existing constitution gate scripts.

## Orchestrator Rules

- Active obligation at start of every atom: `OBL-014(open) -> this atom`.
- Workers get one packet and one allowed write set.
- Workers must not edit `OBLIGATIONS.md`; only the orchestrator updates it
  after evidence exists.
- Workers must not use `git add .`.
- Before any packet touching `src/` or `scripts/`, run the open-PR overlap
  check from `AGENTS.md §4.1`.
- If any packet requires a restricted surface, stop and reclassify. Do not work
  around the restriction.
- If a new typed transaction discriminant, CAS `ObjectType`, canonical signing
  payload, sequencer admission rule, constitution text, or flowchart authority
  seems needed, stop for explicit Class-4 section-8 ratification.

## Universal Forbidden Paths

- `src/kernel.rs`
- `src/bus.rs`
- `src/state/sequencer.rs`
- `src/state/typed_tx.rs`
- `src/sdk/tools/wallet.rs`
- `src/bottom_white/cas/schema.rs`
- `constitution.md`
- canonical signing payload surfaces
- RootBox/kernel authority surfaces
- flowchart authority or canonical hash surfaces

## Universal Structural Gates

Run after every implemented packet unless the packet is docs-only:

```bash
git diff --check
(git diff --name-only origin/main...HEAD; git ls-files -o --exclude-standard) | sort -u | grep -E '^(src|tests)/' | xargs grep -nE 'raw.*std[e]rr|Lean.*std[e]rr|api[_-]?ke[y]|Authori[z]ation|Bear[e]r'
(git diff --name-only origin/main...HEAD; git ls-files -o --exclude-standard) | sort -u | grep -E '^(handover|src|tests)/' | grep -v '^handover/directives/tc_taskpackets_2026-06-04/' | xargs grep -nE 'PRO[V]EN|DEFIN[I]TIVE|caus[a]l|isolated[[:space:]]lever|X[[:space:]]>[[:space:]]Y'
git diff --name-only origin/main...HEAD | grep -E 'src/(kernel|bus|state/sequencer|state/typed_tx|sdk/tools/wallet|bottom_white/cas/schema)\.rs'
```

Expected grep behavior: each grep command may exit 1 with no output; that is
the clean result. If any command prints a path, stop and classify before
continuing. The grep patterns are written to avoid self-matching this gate text.

Wave-level gates:

```bash
cargo test --test constitution_matrix_drift --no-fail-fast
bash scripts/run_constitution_gates.sh
```

Final branch gates:

```bash
cargo test --workspace --no-fail-fast
bash scripts/run_constitution_gates.sh
cargo test --test constitution_matrix_drift --no-fail-fast
```

## Verdict Domains

- Clean-context audit: `NO-VIOLATION | VIOLATION-FOUND <clause> <file>:<line> | RECONSTRUCTION-FAILURE <artifact> | SECOND-SOURCE-DRIFT <view>`
- Veto-AI only: `PASS | VETO <constitutional-clause> <evidence>`
- Predicate runner: `PREDICATES-GREEN <atom> | PREDICATES-RED <atom> <cmd>`
- Obligation witness: `OBL-ALL-CLOSED | OBL-OPEN-MUST <id> | OBL-EVIDENCE-MISSING <id> | OBL-BLOCKER-UNVERIFIED <id>`
- Dirty-tree steward: `DIRTY-PRESERVED | DIRTY-PRESERVATION-FAILURE <artifact> | DIRTY-CLASSIFICATION-DRIFT <path> | DIRTY-UNQUALIFIED-SALVAGE <path>`

## Dispatch Ledger

| Atom | Packet | Lane | Status | Dependencies | Audit |
|---|---|---|---|---|---|
| TC-ORCH-000 | `packets/TC-ORCH-000-index.md` | audit | ready | none | Karpathy Architect |
| TC-ORCH-001 | `packets/TC-ORCH-001-gate-policy.md` | audit | ready | TC-ORCH-000 | Constitution |
| TC-ORCH-002 | `packets/TC-ORCH-002-worker-prompt.md` | audit | ready | TC-ORCH-000 | Simple Code |
| TC-ORCH-003 | `packets/TC-ORCH-003-reviewer-prompts.md` | audit | ready | TC-ORCH-000 | Constitution |
| TC-Q000 | `packets/TC-Q000-dirty-quarantine.md` | substrate | ready | none | Dirty-tree steward |
| TC-Q001 | `packets/TC-Q001-clean-worktree.md` | substrate | ready | TC-Q000 | Constitution |
| TC-000 | `packets/TC-000-path-b-decision.md` | substrate | ready | TC-Q001 | Constitution |
| TC-001 | `packets/TC-001-veto-scope.md` | audit | ready | TC-000 | Constitution |
| TC-002 | `packets/TC-002-boot-trust-root.md` | substrate | ready | TC-000 | Constitution |
| TC-003a | `packets/TC-003a-ref-contract.md` | substrate | ready | TC-002 | Data-integrity |
| TC-003b | `packets/TC-003b-fail-closed-refs.md` | substrate | ready | TC-003a | Reliability |
| TC-003c | `packets/TC-003c-reopen-sequence.md` | substrate | ready | TC-003b | Reliability |
| TC-004a | `packets/TC-004a-rtool-head-witness.md` | substrate | ready | TC-003c | Constitution |
| TC-004b | `packets/TC-004b-wtool-accepted-write.md` | substrate | ready | TC-004a | Constitution |
| TC-005a | `packets/TC-005a-rejection-split.md` | substrate | ready | TC-004b | Data-integrity |
| TC-005b | `packets/TC-005b-l4e-reconstruction.md` | substrate | ready | TC-005a | Data-integrity |
| TC-101 | `packets/TC-101-gateway-fact.md` | gateway | ready | TC-005b | Data-integrity |
| TC-102 | `packets/TC-102-wal-fact.md` | gateway | ready | TC-101 | Reliability |
| TC-103 | `packets/TC-103-map-reduce-tick-fact.md` | gateway | ready | TC-101 | Data-integrity |
| TC-104 | `packets/TC-104-llm-call-fact.md` | gateway | ready | TC-101 | Security |
| TC-105 | `packets/TC-105-wallet-derived-fact.md` | substrate | ready | TC-101 | Constitution |
| TC-106 | `packets/TC-106-search-fact.md` | gateway | ready | TC-101 | Security |
| TC-107 | `packets/TC-107-board-derived-fact.md` | audit | ready | TC-101 | Second-source drift |
| TC-108 | `packets/TC-108-lean-error-fact.md` | lean-search | ready | TC-101 | Shielding |
| TC-109 | `packets/TC-109-halt-fact.md` | reliability | ready | TC-101, TC-007d | Reliability |
| TC-110 | `packets/TC-110-boot-provenance-fact.md` | substrate | ready | TC-002, TC-101 | Constitution |
| TC-007a | `packets/TC-007a-durable-outbox.md` | gateway | ready | TC-101 | Reliability |
| TC-007b | `packets/TC-007b-crash-terminal-mapping.md` | gateway | ready | TC-007a | Reliability |
| TC-007c | `packets/TC-007c-production-llm-wrapper.md` | gateway | ready | TC-007b | Security |
| TC-007d | `packets/TC-007d-clean-halt-gate.md` | gateway | ready | TC-007c | Reliability |
| TC-009a | `packets/TC-009a-minsky-core.md` | substrate | ready | TC-101 | Formal-methods |
| TC-009b | `packets/TC-009b-minsky-replay.md` | substrate | ready | TC-009a | Replay |
| TC-010a | `packets/TC-010a-brainfuck-core.md` | substrate | ready | TC-101 | Formal-methods |
| TC-010b | `packets/TC-010b-brainfuck-replay.md` | substrate | ready | TC-010a | Replay |
| TC-011A | `packets/TC-011A-lean-schema-lock.md` | lean-search | ready | TC-002 | Formal-methods |
| TC-011B | `packets/TC-011B-lean-step-fixtures.md` | lean-search | ready | TC-011A | Formal-methods |
| TC-011C | `packets/TC-011C-lean-final-recert.md` | lean-search | ready | TC-011B | Formal-methods |
| TC-012A | `packets/TC-012A-g0-manifest-freeze.md` | lean-search | ready | TC-011A | Formal-methods |
| TC-012B | `packets/TC-012B-g0-ast-rank.md` | lean-search | ready | TC-012A | Formal-methods |
| TC-012C | `packets/TC-012C-g0-enumerator.md` | lean-search | ready | TC-012B | Formal-methods |
| TC-013A | `packets/TC-013A-strict-dovetail.md` | lean-search | ready | TC-012C | Formal-methods |
| TC-013B | `packets/TC-013B-market-invariance.md` | lean-search | ready | TC-013A | Formal-methods |
| TC-014A | `packets/TC-014A-duplicate-pointer.md` | lean-search | ready | TC-013A | Formal-methods |
| TC-014B | `packets/TC-014B-poisoned-odd-queue.md` | lean-search | ready | TC-014A | Formal-methods |
| TC-016 | `packets/TC-016-market-legalization.md` | lean-search | ready | TC-013B, TC-014B | Constitution |
| TC-017 | `packets/TC-017-autonomous-legalization.md` | lean-search | ready | TC-016 | Constitution |
| TC-018A | `packets/TC-018A-agent-view-renderer.md` | lean-search | ready | TC-011B | Shielding |
| TC-018B | `packets/TC-018B-prompt-guard.md` | lean-search | ready | TC-018A | Shielding |
| TC-015A | `packets/TC-015A-crash-matrix-driver.md` | reliability | ready | TC-007d, TC-014B | Reliability |
| TC-019A | `packets/TC-019A-difficulty-ladder.md` | audit | ready | TC-011C, TC-014B | Formal-methods |
| TC-020A | `packets/TC-020A-prereg-parity.md` | audit | ready | TC-019A | Statistics |
| TC-021A | `packets/TC-021A-audit-packet-export.md` | audit | ready | all previous | Constitution |
| TC-021B | `packets/TC-021B-clean-checkout-replay.md` | audit | ready | TC-021A | Reliability |

## Final Completion Definition

OBL-014 remains open until:

- every packet status is done,
- every packet ship gate has evidence,
- final branch gates pass,
- clean-context audit has no unresolved violation,
- final audit packet is exportable by a fresh checkout,
- obligation witness emits `OBL-ALL-CLOSED`.
