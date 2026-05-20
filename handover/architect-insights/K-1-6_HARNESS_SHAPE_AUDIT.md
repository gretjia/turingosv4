# K-1.6 Harness Shape Audit (read-only — no code change)

**Generated**: 2026-05-20  
**Task**: Inventory actual harness distribution in `tests/constitution_*.rs` before designing K-1.1 factorization  
**Total constitution_*.rs files**: 128

## Distribution

| Shape | Description | N files | Avg setup LOC | Example file |
|-------|-------------|---------|---------------|--------------|
| A | bare seq+cas (no writer, no CPMM) | 0 | n/a | n/a |
| B | seq + RejectionEvidenceWriter (no CPMM) | 6 | ~200 | `constitution_completeset_hardening.rs` |
| C | seq + market preseed (no CPMM) | 0 | n/a | n/a |
| D | seq + CPMM markers | 12 | ~200 | `constitution_audit_views.rs` |
| E | singular (other patterns) | 0 | n/a | n/a |
| F | source-grep only (no sequencer instantiation) | 110 | ~72 | `constitution_admission_no_fail_open_default.rs` |

**Total**: 128 (6 + 12 + 110 = 128 ✓)

## Existing Harness pattern adoption

**Files already using `struct Harness { ... } + fresh_harness()` factory**: 18  
**Files recreating setup manually in each test**: 110  
**Files in Shape F (no sequencer needed)**: 110

### Key finding
- **Shape F overwhelmingly dominant**: 110/128 = 85.9% of files are source-grep only (read `src/` files, parse/scan, make assertions)
- **Shape B+D are minority**: 18/128 = 14.1% of files actually instantiate a Sequencer
- **Duplication in B+D is real**: Both shapes replicate ~200 LOC of setup (TempDir, CasStore, RejectionEvidenceWriter, Sequencer::new with 10 args, etc.)
- **Zero Shape A files**: All sequencer-instantiating files also introduce either RejectionEvidenceWriter (Shape B) or CPMM (Shape D) — bare sequencer alone never appears

## Existing Harness pattern details

Files using `struct Harness` + `fresh_harness()` factory (18 files):
- All 18 are in Shape B (6 files) or Shape D (12 files)
- Each file defines its own local `struct Harness` with identical field structure across files
- Each file re-implements the same 40-50 line `fn fresh_harness(initial_q: QState) -> Harness` boilerplate

Example of repeated structure:
```
struct Harness {
    _tmp: TempDir,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    _ledger: Arc<RwLock<dyn LedgerWriter>>,
}
```

All 18 files build a Harness via identical pattern:
```rust
fn fresh_harness(initial_q: QState) -> Harness {
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("kp"));
    let writer: Arc<RwLock<dyn LedgerWriter>> = 
        Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
    let rejection_writer = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
    let preds = Arc::new(PredicateRegistry::new());
    let tools = Arc::new(ToolRegistry::new());
    let epoch = SystemEpoch::new(1);
    let mut pinned = PinnedSystemPubkeys::new();
    pinned.insert(epoch, keypair.public_key());
    let pinned_pubkeys = Arc::new(pinned);
    let (seq, rx) = Sequencer::new(
        cas,
        keypair,
        epoch,
        writer.clone(),
        rejection_writer,
        preds,
        tools,
        pinned_pubkeys,
        initial_q,
        16,
    );
    Harness { _tmp: tmp, seq, rx, _ledger: writer }
}
```

## Sample evidence

### Shape B example setup (first 25 lines)
From `constitution_completeset_hardening.rs`:
```rust
use std::sync::{Arc, RwLock};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use turingosv4::bottom_white::ledger::transition_ledger::{InMemoryLedgerWriter, LedgerWriter};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::monetary_invariant::total_supply_micro as canonical_total_supply_micro;
use turingosv4::economy::money::MicroCoin;
use turingosv4::state::q_state::{
    AgentId, QState, ShareSidePair, TaskId, TaskMarketEntry, TaskMarketState, TxId,
};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, CompleteSetMintTx, CompleteSetRedeemTx, EventId, OutcomeSide, ShareAmount,
    TypedTx,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

// ── Harness (self-contained per strict-constitution doctrine) ───────────────

struct Harness {
```

### Shape F example setup (first 20 lines)
From `constitution_admission_no_fail_open_default.rs`:
```rust
//! TuringOS Constitution Gate — Stage C R2 Q10 admission fail-open lint
//! (Phase E.5 defect-fix mechanism; Codex audit 2026-05-09 session #32).
//!
//! # Scope
//!
//! Q10 concern (CHALLENGE from Codex R2): late-admission missing-entry txs
//! because sequencer.rs contains `.unwrap_or(<default>)` without fail-closed
//! rejection class. This gate scans sequencer.rs source to reject any line
//! that combines BOTH:
//!   - unwrap_or / unwrap_or_else pattern
//!   - fail-open default token (Open / None / Empty / [], etc.)
//!
//! Result: fail-closed admission semantics; missing entry → explicit
//! TransitionError, not silent default.

use std::sync::Arc;

// Pattern matchers for `.unwrap_or(...)` family
const UNWRAP_OR_PATTERNS: &[&str] = &[".unwrap_or", ".unwrap_or_else"];

// Tokens representing fail-open defaults that must not appear same-line as unwrap_or
const FAIL_OPEN_DEFAULT_TOKENS: &[&str] = &["Open", "None", "Empty", "[]", "vec![]"];
```

## Conclusion

Based on the audit above, K-1.1 factorization should:

- [X] **Extract single `tests/support.rs` + single shared `Harness` struct** — Shape B+D (18 files) share identical setup; deduplication ROI is moderate (200 LOC × 18 = 3.6 kLOC) but the files are already using `struct Harness + fresh_harness()` locally — moving to shared module is low-risk and improves readability.

- [ ] **Per-shape submodule** — Not justified; Shape B and Shape D setups are identical except for fields added in tests (CPMM-specific task setup), which belongs in test helpers, not the harness itself.

- [ ] **No extraction (Shape F dominant)** — Shape F files (110/128) have zero duplication concern because they use source-grep patterns, not sequencer instantiation. No shared harness needed.

### Recommendation detail

**Action**: Create `tests/support.rs` with:
1. `pub struct Harness { _tmp, seq, rx, _ledger }` (move from local re-definitions)
2. `pub fn fresh_harness(initial_q: QState) -> Harness` (move from local re-implementations)
3. Add it to Cargo.toml test config or use `mod support;` in integration tests

**Scope** (K-1.1): Apply to Shape B+D files only (18 files); Shape F files untouched.

**ROI**:
- Eliminates 18 copies of ~200 LOC boilerplate
- Reduces test maintenance surface (Sequencer signature change = 1 edit, not 18)
- Preserves per-test QState preseed logic (no loss of flexibility)
- Enabled by fact that all 18 files already use the pattern independently

**Risk**: Minimal — files already demonstrate the pattern works; no new architecture.

### Non-recommendation detail

**Why not per-shape submodule**: Shape B and D setups differ only in test semantics (which fields of QState to preseed), not in harness construction. The harness itself is generic over `initial_q: QState`, so Shape B and D can coexist in a single module.

**Why not skip extraction**: While Shape F dominates numerically, the 18 Shape B+D files do have measurable duplication. Extraction improves evolvability (e.g., when Sequencer::new gains a parameter, only 1 place to fix). The metric for extracting is **identical code across ≥3 files**, and we meet that threshold.

