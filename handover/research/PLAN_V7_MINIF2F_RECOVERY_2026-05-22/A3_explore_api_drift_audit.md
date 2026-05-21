# A3 — API drift audit on `lean_market.rs` + `batch_orchestrator.rs` imports

Phase A research output #3. Per-import audit against current trunk public APIs. This audit is the basis for "verbatim restoration is safe" — without it, every restored import is a potential compile failure.

## `lean_market.rs` imports (audited against current trunk)

All `use turingosv4::*` imports verified public-and-stable as of commit `7f61605d` (pre-hotfix; hotfix did not alter API surface):

| Imported path | Trunk location | Status |
|----------------|----------------|--------|
| `bottom_white::cas::store::CasStore` | `src/bottom_white/cas/store.rs` | ✓ public |
| `bottom_white::ledger::transition_ledger::{replay_full_transition, Git2LedgerWriter, LedgerWriter}` | `src/bottom_white/ledger/transition_ledger.rs` | ✓ public |
| `bottom_white::tools::registry::ToolRegistry` | `src/bottom_white/tools/registry.rs` | ✓ public |
| `economy::money::MicroCoin` | `src/economy/money.rs` | ✓ public |
| `state::q_state::{AgentId, ClaimStatus, QState}` | `src/state/q_state.rs` | ✓ public |
| `state::typed_tx::PositionSide` | `src/state/typed_tx.rs` | ✓ public |
| `top_white::predicates::registry::PredicateRegistry` | `src/top_white/predicates/registry.rs` | ✓ public |
| `runtime::verify::*` | `src/runtime/verify.rs` | ✓ public |
| `runtime::PinnedSystemPubkeys` | `src/runtime/mod.rs` | ✓ public |
| `runtime::PinnedPubkeyManifest` | `src/runtime/mod.rs` | ✓ public |

### Audit method

```bash
git show 309e026a^:experiments/minif2f_v4/src/bin/lean_market.rs | grep -E '^use turingosv4::' | sort -u
# For each import:
rg --line-number "pub (fn|struct|enum|trait|mod|const) <name>" src/
```

### Verdict

Zero Class 1 visibility flips needed. R0 restoration compiles cleanly against current trunk without any `pub` upgrade on trunk side.

## `batch_orchestrator.rs` imports (audited against current trunk)

| Imported path | Trunk location | Status |
|----------------|----------------|--------|
| `crate::runtime::chain_tape_lease` | `src/runtime/chain_tape_lease.rs` | ✓ public (mod.rs:135) |
| `crate::runtime::resume_preflight` | `src/runtime/resume_preflight.rs` | ✓ public (mod.rs:125) |
| `crate::runtime::batch_continuation_manifest::{BatchContinuationManifest, TaskContinuationEntry}` | `src/runtime/batch_continuation_manifest.rs` | ✓ public (mod.rs:146) |

### Why this audit was the lowest-risk path

The deleted `batch_orchestrator.rs` from commit `309e026a^` already imports modules that were preserved in the C3 cleanup — chain_tape_lease, resume_preflight, batch_continuation_manifest were intentionally kept because they implement the **schema** that batch_orchestrator drives.

The deletion in `309e026a` removed the **driver** (orchestrator) but kept the **schema**. R1 restores the driver. No double-implementation because the schema modules are unchanged.

## Risk classes (final)

| Atom | Risk class | Rationale |
|------|------------|-----------|
| R0 | 2 | Production-wire-up: trunk binaries gain ability to invoke real `lean_market`. New file under `experiments/` (separate workspace). |
| R1 | 2 | Production-wire-up: new module `src/runtime/batch_orchestrator` added to trunk lib. Not invoked yet (Plan v7 stops at "tests green"); future wiring is a separate atom. |
| R2 | 1 | Single-line additive: `exclude = [...]` in `[workspace]`. |
| Cz | 4 | Trust Root rehash (genesis_payload.toml). Same pattern as 5 prior Cz cycles. §8 self-signed under existing user delegation 2026-05-21. |
| Hotfix | 1 | 4-line removal + Trust Root rehash on Cz-already-touched file. Class 1 because the removal restores compile correctness (CI red → green) on R1's own PR. |

## §8 self-sign

Class 0 (research archive). Self-signed by Claude opus 4.7 under existing user delegation (2026-05-21).
