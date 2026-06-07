# M07 Single-Admission Predicate Gate — Implementation Spec (Route A, transitional)

- **Date:** 2026-06-07
- **Worktree:** `/home/zephryj/projects/turingosv4-m07-prep` (branch `claude/m07-pr314-followup-prep`)
- **Authorization:** user §8 token `APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE`
- **Risk class:** Class-4 (touches sequencer admission + trust-root pinned `src/state/sequencer.rs`)
- **FC trace:** FC1a-Agent_delta / FC1a-predicates / FC1b-Q_{t+1} (kernel admission), FC2-boot_loop (registry injection)
- **Status:** DESIGN ONLY — no src edits in this workflow.

## 0. Plan route (recap)

- TARGET architecture = **B** (sequencer is the sole accepted-state writer; kernel demoted to proposal-only). NOT this PR.
- LANDING PR = **A transitional**: extract a SHARED predicate-admission module that BOTH sequencer and `memory_kernel` call; the kernel only advances `verified_head` on a predicate-admission PASS; add a mandatory single-admission anti-duplication gate.
- Realistic first cut exercises only the **zero-root boolean branch** (`sequencer.rs:1231-1242`) — that is exactly what G1 and G2 harnesses drive. The bound-root oracle path (`sequencer.rs:1245-1279`) is NOT constructible from current kernel context (kernel has no `proposal_cid`/`read_set`/`write_set`/CAS proofs) and stays sequencer-only for now.

## 1. Ground-truth facts (verified against source this session)

| Fact | File:line | Verified |
|---|---|---|
| Kernel Proceed branch advances head with NO predicate call | `src/memory_kernel.rs:171-189` | ✅ |
| `set_verified_head` called in exactly ONE kernel place | `src/memory_kernel.rs:188` | ✅ |
| Kernel imports have zero predicate/WorkTx/QState refs | `src/memory_kernel.rs:21-34` | ✅ |
| `MemoryKernel<L>` struct fields | `src/memory_kernel.rs:89-95` | ✅ |
| `MemoryKernel::new(tape, run_id, charter)` | `src/memory_kernel.rs:140` | ✅ |
| `verify_work_predicates` — module-private free `fn` | `src/state/sequencer.rs:1225-1280` | ✅ |
| Zero-root verdict-trusting branch | `src/state/sequencer.rs:1231-1242` | ✅ |
| Bound-root oracle branch (re-execute) | `src/state/sequencer.rs:1245-1279` | ✅ |
| Single caller of `verify_work_predicates` | `src/state/sequencer.rs:1866` (in `dispatch_transition`, `:1848`) | ✅ |
| Helper fns (key-set / claim / error-map) | `src/state/sequencer.rs:1282-1372` | ✅ |
| `NodeKind` enum (no admission-receipt variant) | `src/ledger.rs:374-381` | ✅ |
| `TapeNode.payload: serde_json::Value` (free-form) | `src/ledger.rs:420` | ✅ |
| `compute_hash` covers payload (receipt is hash-bound) | `src/ledger.rs:461-466` | ✅ |
| `ImmutableTapeLedger::{set_verified_head, commit, dump_all_nodes}` in trait | `src/ledger.rs:577/579/595` | ✅ |
| Trust-root pins: sequencer/q_state/typed_tx | `genesis_payload.toml:282/281/283` | ✅ |
| `memory_kernel.rs` / `ledger.rs` NOT pinned (grep count 0) | `genesis_payload.toml` | ✅ |
| `Cargo.toml` pinned | `genesis_payload.toml:136` | ✅ |
| Judge seam (success/judge_class/failed_pred computed pre-step) | `src/tdma_runner.rs:618-642` | ✅ |
| `MemoryKernel::new` call sites (8 total) | see §3.4 | ✅ |

## 2. The decisive asymmetry (why this is NOT a mechanical lift)

`verify_work_predicates(q, work, registry, predicate_cas)` needs four inputs. The kernel at `memory_kernel.rs:172` has **none** of them:

| Required arg | Kernel has it? | Gap |
|---|---|---|
| `q: &QState` (for `q.predicate_registry_root_t`) | NO | kernel never references QState |
| `work: &WorkTx` (incl. `predicate_results`) | NO | `Task` has only `{id, prompt}`; FC1 loop produces NO `PredicateResultsBundle` |
| `registry: &PredicateRegistry` | NO | kernel struct holds none |
| `predicate_cas: &dyn PredicateCasView` | NO | kernel holds no CAS handle |

The deepest gap is **#2**: nothing in the FC1 loop produces a runner-stamped `PredicateResultsBundle`. The worker returns prose + a prefix-JSON header; the only "verdict" is `env_result.success` (an upstream judge bool). A literal "both call the same `WorkTx`-typed fn" is therefore impossible. The shared contract MUST be defined at a **claim-level abstraction** (an admission decision over an abstract predicate-claim set), with the sequencer adapting `WorkTx → contract` and the kernel adapting `header + judge bool → contract`. This is option (b) from DEEP-READ-1 §8 and is what G2 demands ("one shared contract, both legs same verdict").

---

## 3. Shared predicate-admission module

### 3.1 Location

**New non-pinned file: `src/predicate_admission.rs`** (declared `pub mod predicate_admission;` — see §3.5 for the pin cost of the declaration site).

Rationale: `memory_kernel.rs` and `ledger.rs` are NOT pinned, but a brand-new shared module deserves its own file (Karpathy: one concern, one flat module). A dedicated file avoids bloating `ledger.rs` and keeps the admission contract greppable. The `pub mod` declaration must live somewhere — every candidate host is pinned (see §3.5), so the declaration line is the unavoidable pinned-file edit. The module body itself is in a fresh non-pinned file.

### 3.2 Public surface (claim-level abstraction — option b)

```rust
// src/predicate_admission.rs
//! Shared predicate-admission contract. ONE oracle, two adapters
//! (sequencer WorkTx leg + kernel header leg). Route-A transitional:
//! only the zero-root boolean branch is reachable from the kernel today.

use std::collections::BTreeMap;
use crate::state::typed_tx::{PredicateId, BoolWithProof};

/// One predicate claim, decoupled from WorkTx wire layout. Both legs build
/// this from their own context. `proof_cid` is carried so the bound-root
/// oracle path can resolve it; the kernel leg always leaves it None.
#[derive(Debug, Clone)]
pub struct PredicateClaim {
    pub id: PredicateId,
    pub value: bool,
    pub proof_cid: Option<crate::bottom_white::cas::cid::Cid>, // exact Cid type TBD — see Open Q
}

/// The abstract claim set an admission decision is taken over.
#[derive(Debug, Clone, Default)]
pub struct PredicateClaimSet {
    pub acceptance: Vec<PredicateClaim>,
    pub settlement: Vec<PredicateClaim>,
}

/// Admission verdict. PASS carries the registry root the decision was taken
/// under (Hash::ZERO for the legacy boolean branch); the receipt embeds this.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmissionVerdict {
    Pass { registry_root_hex: String },
    Fail { failed_predicate: String, reason: AdmissionFailReason },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmissionFailReason {
    AcceptancePredicateFalse,
    SettlementPredicateFalse,
    ZeroRootRefusedForOsQualifiedRun, // G3 path
    // bound-root variants added when the kernel bound path lands (out of scope here)
}

/// THE single admission contract. Pure, deterministic, no I/O on the
/// zero-root path. `registry_root` selects the branch:
///   * Hash::ZERO  -> legacy boolean branch (verdict-trusting) UNLESS
///                    os_qualified == true, in which case REFUSE (G3).
///   * non-zero     -> bound-root oracle path (sequencer-only today; the
///                    kernel never reaches here — its claim set is boolean).
pub fn decide_admission(
    registry_root_hex: &str,
    claims: &PredicateClaimSet,
    os_qualified: bool,
) -> AdmissionVerdict { /* ... */ }
```

Notes:
- `decide_admission` is the **single point of truth** both legs call. The zero-root logic moves here verbatim from `sequencer.rs:1231-1242` (acceptance/settlement boolean scan).
- The bound-root oracle (`sequencer.rs:1245-1279`, the registry/CAS re-execution) stays in `sequencer.rs` for the transitional PR because it needs `&PredicateRegistry` + `&dyn PredicateCasView` + `PredicateWorkView::from_work_tx(work)` — none of which the kernel can supply. The shared module exposes only the claim-level decision; the sequencer's bound branch calls `decide_admission` for the zero-root case and keeps its existing oracle call for the non-zero case. This keeps the sequencer behavior-preserving (see §3.3).

### 3.3 What MOVES from sequencer, what STAYS (behavior-preserving)

**Moves into `decide_admission` (zero-root branch only):** the boolean scan at `sequencer.rs:1232-1242`.

**Stays in `sequencer.rs`:**
- `verify_work_predicates` keeps its signature and remains the sequencer's entry. Its body changes minimally: the zero-root branch (`:1231-1243`) now delegates to `decide_admission(ZERO_HEX, &claims, os_qualified=false)` after adapting `work.predicate_results → PredicateClaimSet`; the bound branch (`:1245-1279`) is UNCHANGED.
- `verify_predicate_key_set`, `verify_predicate_claim`, `predicate_verify_error_to_transition` (`:1282-1372`) — all stay; they are bound-root-only.
- `dispatch_transition` Work arm and `apply_one` — unchanged.

**Behavior-preservation check:** for the sequencer, `decide_admission(ZERO, claims, false)` must return Fail on the FIRST false acceptance, then FIRST false settlement, in the same `BTreeMap` iteration order as today, mapping back to `TransitionError::AcceptancePredicateFailed(pid)` / `SettlementPredicateFailed(pid)`. The `os_qualified` flag is `false` for the legacy sequencer call so existing zero-root sequencer tests stay green; G3 flips it on (see §5).

**Error mapping at the sequencer boundary (DEEP-READ-1 §5 option a):** the shared module returns `AdmissionVerdict`, NOT `TransitionError`. The sequencer maps `AdmissionVerdict::Fail{failed_predicate, reason}` → the existing `TransitionError` variants. This keeps `TransitionError` defined and pinned inside `sequencer.rs` (no error type crosses the pin boundary) and keeps the L4.E rejection routing identical.

### 3.4 How the kernel calls it

At `memory_kernel.rs:172`, the Proceed arm calls `decide_admission` with a kernel-built `PredicateClaimSet`. The claim set is stamped at the judge seam (§4) and threaded into `step_forward`. On `Pass`, the kernel embeds the receipt into the `StateAccepted` payload and then advances head; on `Fail`, it reroutes into `handle_rejection` WITHOUT advancing head.

### 3.5 `pub mod` declaration pin cost

A new `pub mod predicate_admission;` must be declared. Candidate hosts and their pin status:

| Host | Pinned? | Pin line |
|---|---|---|
| `src/lib.rs` | YES | 198 |
| `src/state/mod.rs` | YES | 279 |
| `src/ledger.rs` (declare as submodule) | NO | — |

**Decision:** declare `pub mod predicate_admission;` in **`src/lib.rs`** (the canonical crate-root module list). This forces a rehash of pin **198** in the same commit. Alternative to avoid ANY pinned-declaration edit: make `predicate_admission` a submodule re-exported from non-pinned `src/ledger.rs` (e.g. `pub mod predicate_admission;` inside ledger, path `#[path = "predicate_admission.rs"]`). This is uglier (admission logic nested under ledger) but touches zero pinned files for the declaration. **Recommended: lib.rs declaration + rehash 198** for clarity; flag as Open Question if the user prefers zero-pin-for-declaration.

---

## 4. Kernel changes (`src/memory_kernel.rs` — NOT pinned)

### 4.1 Struct + constructor injection

Add three fields to `MemoryKernel<L>` (`:89-95`):

```rust
pub struct MemoryKernel<L: ImmutableTapeLedger> {
    pub tape: L,
    pub run_id: String,
    pub charter: CharterCore,
    pub tokenizer: Arc<Tokenizer>,
    pub rtool: Rtool<MemoryKernelTape<L>>,
    // NEW (M07):
    pub predicate_registry: Arc<PredicateRegistry>,
    pub predicate_registry_root_t: Hash,          // Hash::ZERO for legacy/non-qualified runs
    pub predicate_cas: Arc<dyn PredicateCasView + Send + Sync>,
}
```

`MemoryKernel::new` (`:140`) gains the three params. To keep the 8 existing call sites cheap, add a **defaulting constructor** that preserves the current 3-arg signature with a zero-root / empty-registry / empty-CAS default, and a new explicit constructor for OS-qualified runs:

```rust
// preserves existing 3-arg call sites (zero-root, non-OS-qualified)
pub fn new(tape: L, run_id: impl Into<String>, charter: CharterCore) -> Self {
    Self::new_with_predicates(
        tape, run_id, charter,
        Arc::new(PredicateRegistry::from_boot_manifest(BootPredicateManifest::empty()).expect("empty")),
        Hash::ZERO,
        Arc::new(EmptyPredicateCasView::default()), // registry.rs:200-208
    )
}
pub fn new_with_predicates(
    tape: L, run_id: impl Into<String>, charter: CharterCore,
    predicate_registry: Arc<PredicateRegistry>,
    predicate_registry_root_t: Hash,
    predicate_cas: Arc<dyn PredicateCasView + Send + Sync>,
) -> Self { /* ... */ }
```

This means **the 8 existing call sites (`memory_kernel.rs:503`, `tdma_runner.rs:539`, `tests/realworld_tdma_judge_ai_step_proof.rs:75/187`, `tests/bug7_regression_suite.rs:27`, `tests/tdma_memory_harness_v1.rs:31`, and the two pending harnesses) need NO change** — they get a zero-root, non-OS-qualified kernel that writes a receipt with `registry_root: ZERO`. New imports needed in `memory_kernel.rs`: `PredicateRegistry`, `BootPredicateManifest`, `PredicateCasView`, `EmptyPredicateCasView` from `crate::top_white::predicates::registry`; `Hash` from `crate::state::q_state`; `crate::predicate_admission::*`.

### 4.2 `step_forward` plumbing for the claim set

The Proceed branch needs a `PredicateClaimSet`. Two threading options:

- **(plumb) `step_forward(task, env_result, claim_set)`** — add a `claim_set: PredicateClaimSet` param to `step_forward`/`step_forward_with_workspace`. The 8 call sites that use the bare 3-arg form supply `PredicateClaimSet::default()` (empty → zero-root PASS, behavior-identical to today for tests that have no predicates). The tdma_runner seam supplies the real stamped set.
- **(derive) from header** — parse a `predicate_results` block out of the worker header. REJECTED: the worker header (`StateUpdate`) has no predicate-results field, and inventing one is a `state_update.rs` schema change with no upstream producer.

**Decision: plumb the claim set as an explicit param** (default-empty preserves all current call sites). Signature:

```rust
pub fn step_forward(&mut self, task: &Task, env_result: EnvironmentResult,
                    claims: PredicateClaimSet) -> KernelStep
```

(Keep a 3-arg shim `step_forward(task, env)` that calls through with `PredicateClaimSet::default()` so non-predicate callers are untouched — same pattern as the existing `step_forward` / `step_forward_with_workspace` pair at `:156/162`.)

### 4.3 The Proceed branch rewrite (`memory_kernel.rs:171-190`)

```rust
(Ok(header), true) if header.status == StateStatus::Proceed => {
    // M07: gate the head advance on the SHARED admission contract.
    let root_hex = self.predicate_registry_root_t.to_hex(); // or hex of Hash
    let os_qualified = self.predicate_registry_root_t != Hash::ZERO; // see §6
    let verdict = crate::predicate_admission::decide_admission(&root_hex, &claims, os_qualified);

    match verdict {
        AdmissionVerdict::Pass { registry_root_hex } => {
            let accepted = self.tape.commit(CommitRequest {
                kind: NodeKind::StateAccepted,
                verified: true,
                parent: Some(verified_head.clone()),
                scope: None, attempt_ordinal: None, reject_class: None, token_count: None,
                payload: serde_json::json!({
                    "state_update": header,
                    "output_summary": "accepted",
                    // M07 RECEIPT (additive payload field — NO schema/NodeKind change):
                    "predicate_admission": {
                        "verdict": "PASS",
                        "registry_root": registry_root_hex,
                        "acceptance_pids": claims.acceptance.iter().map(|c| &c.id).collect::<Vec<_>>(),
                        "settlement_pids": claims.settlement.iter().map(|c| &c.id).collect::<Vec<_>>(),
                        "os_qualified": os_qualified,
                    },
                }),
            });
            let evidence_hash = accepted.hash.clone();
            self.tape.set_verified_head(accepted.hash); // advance ONLY after PASS + receipt
            KernelStep::Proceed { evidence_hash }
        }
        AdmissionVerdict::Fail { failed_predicate, .. } => {
            // Reroute into the existing non-advancing path. Head stays frozen.
            let rej_header = StateUpdate {
                status: StateStatus::Reject, // or Invalid; reuse header but stamp the failed predicate
                failed_predicate: Some(failed_predicate),
                reject_class: Some("PredicateAdmissionFailed".into()),
                ..header
            };
            self.handle_rejection(task, verified_head, rej_header, env_result, workspace)
        }
    }
}
```

Key invariants:
- `set_verified_head` is reached **only** on `Pass`, and only **after** the receipt-bearing `StateAccepted` is committed. The receipt is hash-covered (`ledger.rs:461-466`), so an auditor reconstructs the gate from the tape alone (G1).
- On `Fail`, control routes to `handle_rejection` (`:212`), which commits `NodeKind::AgentProposal verified:false` and never advances head (DEEP-READ-2 §3). This is the "predicate-fail routes to tape without advancing head" requirement.
- The empty-claim-set default yields `Pass{registry_root: ZERO}` → every current 3-arg caller still advances, now WITH a receipt. This is what turns G1 green for the pure-MemoryTape harness (it sets a zero-root kernel, empty claims, gets a PASS receipt).

### 4.4 Stamping site (`src/tdma_runner.rs:618-642` — NOT pinned)

The judge already computes `(success, judge_class_str, failed_pred_str, judge_reason)` at `:618-619`. Build a `PredicateClaimSet` from that verdict and pass it into `step_forward`:

```rust
let claims = PredicateClaimSet {
    acceptance: vec![PredicateClaim {
        id: PredicateId(format!("judge::{judge_class_str}")), // stable judge-derived pid
        value: success,
        proof_cid: None,
    }],
    settlement: vec![],
};
// ...
let step = kernel.step_forward(&task, env, claims);
```

This makes the kernel's admission boolean equal to the judge verdict, which is the SAME logical claim the sequencer's zero-root branch checks — closing G2. (Note: `env.success` and the acceptance claim value are the same `success` bool here; that is intentional and the single-admission invariant relies on it.)

---

## 5. Zero-root quarantine (G3) — OS-qualified requires non-zero root

**Mechanism:** `decide_admission(registry_root_hex, claims, os_qualified)`:
- If `registry_root_hex == ZERO_HEX && os_qualified == true` → return `Fail{ reason: ZeroRootRefusedForOsQualifiedRun }`. This is the G3 refusal.
- If `registry_root_hex == ZERO_HEX && os_qualified == false` → legacy boolean branch (current behavior, keeps existing tests green).
- If `registry_root_hex != ZERO_HEX` → bound-root path. In the shared module this returns a sentinel that tells the **sequencer** to run its existing oracle (`sequencer.rs:1245-1279`); the **kernel** never supplies a non-zero root in route A (it has no proposal CID/CAS proofs), so the kernel bound path is out of scope.

**G3 wiring in the sequencer:** the sequencer's zero-root call becomes `decide_admission(ZERO_HEX, &claims, os_qualified_run)`. The G3 gate drives a `WorkTx` under `q.predicate_registry_root_t == Hash::ZERO` with a self-asserted `true`; to turn G3 green, an OS-qualified sequencer run must pass `os_qualified=true`, which makes `decide_admission` REFUSE. The non-zero bound root is minted today only by `PredicateBindingActivate` (`sequencer.rs:3035` sets `q_next.predicate_registry_root_t = activate.registry_merkle_root`); G3's positive control already confirms the bound branch rejects an unexpected self-asserted key.

**Open coupling:** where does `os_qualified` come from for the sequencer? It is a property of the run, not the tx. Cleanest: derive `os_qualified = (q.predicate_registry_root_t != Hash::ZERO)` so that any run that has bound a registry is OS-qualified and zero-root work under it is refused, while genesis/legacy zero-root runs stay non-qualified. This makes G3 green WITHOUT a new field — it is a pure consequence of "a bound run never falls back to verdict-trust". **This is the recommended definition** (no schema surface). Flag for architect confirmation (Open Q).

**Files:** `src/state/sequencer.rs` (pinned 282 — rehash) for the sequencer leg; `src/predicate_admission.rs` (new, non-pinned) for the decision.

---

## 6. Single-admission anti-duplication gate (the invariant)

The plan requires a mandatory gate proving kernel + sequencer share ONE admission contract — i.e. there is no second copy of the zero-root boolean logic. Two layers:

1. **Behavioral (G2):** `m07_kernel_and_sequencer_must_share_one_predicate_admission_contract` already asserts `kernel_admitted == sequencer_admitted` for the same failing claim. Once both legs route through `decide_admission`, this is green.
2. **Structural anti-duplication gate (NEW, promote to `tests/constitution_single_admission_contract.rs`):** a source-structural witness (mirroring `predicate_pass_required_for_l4` at `tests/constitution_predicate_gate.rs:104-144`) that asserts:
   - `src/predicate_admission.rs` exists and defines `pub fn decide_admission`.
   - `src/state/sequencer.rs` contains `predicate_admission::decide_admission(` in its zero-root branch (the boolean scan is NOT re-implemented inline).
   - `src/memory_kernel.rs` contains `predicate_admission::decide_admission(` before its `set_verified_head` call.
   - The zero-root boolean loop body (`for (pid, bwp) in work.predicate_results.acceptance` with `if !bwp.value`) appears in `predicate_admission.rs` and does NOT appear a second time in `sequencer.rs` or `memory_kernel.rs` (grep-count == exactly one home).

   This is the anti-duplication invariant: exactly ONE admission contract body, two call sites.

---

## 7. Receipt schema decision

**CHOSEN: additive JSON payload field — NO schema change, NO new NodeKind/TxKind.**

- The receipt rides inside the existing `StateAccepted` node's free-form `payload` (`ledger.rs:420`) as a `"predicate_admission"` object (shape (a) in the G1 gate, `constitution_kernel_predicate_gate.rs:88-101`).
- `compute_hash` already folds the whole payload into the content hash (`ledger.rs:461-466`), so the receipt is forgery-resistant and tape-reconstructable with zero schema work.
- The G1 gate was deliberately written to accept payload-key probing rather than an enum variant ("adding an enum variant is one valid fix but not the only one", gate lines 95-96), so this avoids over-constraining and avoids an ADDITIONAL Class-4 typed-tx-schema surface.
- **A new `NodeKind` variant is NOT required and is explicitly avoided** — it would be an additional Class-4 schema surface (the `NodeKind` enum is part of the tape wire schema, even though `ledger.rs` is not trust-root pinned). The additive payload field carries no discriminant change.

**No separate schema concern is triggered** by this PR's receipt: the WorkTx/TypedTx wire schema (`typed_tx.rs`, pinned 283) is untouched — `verify_work_predicates` and `decide_admission` only READ `predicate_results`; field order, discriminants, and `canonical_encode` are unchanged. The new `PredicateClaim`/`PredicateClaimSet`/`AdmissionVerdict` types live in the non-pinned `predicate_admission.rs` and are internal (never serialized to the wire tape), so they are NOT a Class-4 schema surface.

---

## 8. Trust-root rehash (mechanism + exact pins)

Editing any pinned file requires replacing its 64-hex SHA-256 on the same line in the SAME commit, or boot aborts `TRUST_ROOT_TAMPERED` (`src/boot.rs:97-130`, hash = `hex_lower(Sha256::digest(fs::read(file)))`, `boot.rs:114`). There is NO auto-generator (build.rs does not compute trust-root hashes).

**Pins this PR must rehash (route A):**

| File | Pin line | Why edited |
|---|---|---|
| `src/state/sequencer.rs` | **282** | zero-root branch delegates to `decide_admission`; G3 `os_qualified` wiring |
| `src/lib.rs` | **198** | `pub mod predicate_admission;` declaration (see §3.5 alternative to avoid this) |

`src/state/q_state.rs` (281) and `src/state/typed_tx.rs` (283) are NOT edited and NOT rehashed.

**Regenerate command (manual, same commit as the source edit):**
```bash
sha256sum src/state/sequencer.rs   # paste hex into genesis_payload.toml:282
sha256sum src/lib.rs               # paste hex into genesis_payload.toml:198
```
**Governance overlay:** any commit touching `genesis_payload.toml` must carry a user-signed git tag (`scripts/check_tr_ratification_chain.sh`, RATIFICATION_2026-04-27 §3). This is the §8 chain for trust-root mutations — already covered by the `APPROVE-M07-A4-...` token but the tag must still be applied.

**Not pinned (free to edit):** `src/memory_kernel.rs`, `src/ledger.rs`, `src/tdma_runner.rs`, `src/predicate_admission.rs` (new). `Cargo.toml` IS pinned (136) — do NOT add `[[test]]` targets; the pending gates stay subdir-excluded until promoted (§9).

---

## 9. Gate promotion (G1/G2/G3 red→green; G4/G5 stay pending)

The triple-coupling rule (memory `feedback_constitution_gate_triple_coupling`) applies in reverse for ADDING a gate: a registered constitution gate = atomic add of {test file at `tests/constitution_*.rs` top level + manifest entry in `scripts/constitution_gates.manifest.toml` + matrix-drift allowlist coupling in `tests/constitution_matrix_drift.rs`}.

Promote G1/G2/G3 once the src change lands and they go green:

1. **Move** `tests/pending/constitution_kernel_predicate_gate.rs` → `tests/constitution_kernel_predicate_gate.rs` (and the other two). Now the flat `ls tests/constitution_*.rs` glob in `run_constitution_gates.sh:15` discovers them.
2. **Register** each in `scripts/constitution_gates.manifest.toml` with `name = "constitution_kernel_predicate_gate"` + `authority` field. The manifest grep (`run_constitution_gates.sh:18`) and `constitution_matrix_drift` (manifest-driven) now see them.
3. **Matrix-drift coupling:** `constitution_matrix_drift.rs` requires every manifest gate to either appear in the execution matrix OR be in the baseline allowlist (`tests/constitution_matrix_drift.rs:32-51`). Add the three new gate names to the matrix (`handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`) OR to the allowlist with PR justification. Since these are NEW (not grandfathered K-2.3), prefer adding them to the **matrix** (they are live, referenced gates), not the allowlist.
4. **Add the structural anti-duplication gate** `tests/constitution_single_admission_contract.rs` (§6) through the same triple-coupling.

**G4/G5 stay pending** in `tests/pending/` (separate §8, separate workflow):
- **G4** `constitution_budget_ceiling_enforced.rs` — Art. V.2 `budget_state_t` admission ceiling; a distinct sequencer gate, not part of the single-admission predicate contract.
- **G5** `constitution_fc3_meta_loop_closure.rs` — ArchitectProposal + Veto-AI PASS driving tape-visible re-init; FC3 meta-loop, orthogonal to FC1 admission.

These are explicitly out of scope for the M07 single-admission token and must NOT be promoted in this PR.

---

## 10. Evidence plan (real red→green + workspace green + replay)

Mirror the existing predicate-gate evidence pattern (`tests/constitution_predicate_registry_replay.rs:79-138`, deterministic reconstruction from tape+CAS).

**Pre-fix RED (record the bypass):**
```bash
bash scripts/run_pending_agentic_os_kill_conditions.sh   # G1/G2/G3 compile standalone via rustc --test, OBSERVE RED
```
Captures the three pending gates failing on current `main`/branch HEAD before any src edit.

**Post-fix GREEN (the same three turn green):**
```bash
# after src change + gate promotion:
cargo test --test constitution_kernel_predicate_gate
cargo test --test constitution_kernel_sequencer_single_admission
cargo test --test constitution_predicate_zero_root_is_not_oracle
cargo test --test constitution_single_admission_contract   # new anti-duplication gate
```

**Workspace green (no regression — the receipt + injection must not break existing kernel/sequencer tests):**
```bash
cargo check
cargo test --workspace --no-fail-fast
bash scripts/run_constitution_gates.sh
cargo test --test constitution_matrix_drift
```
The existing sequencer zero-root tests must stay green (behavior-preserving delegation, §3.3); the 8 `MemoryKernel::new` call sites must stay green (defaulting constructor, §4.1).

**Replay/reconstruction witness (G1 strongest form):** add a real-evidence test (mirror `replay_reconstructs_activation_with_predicate_cas_view`) that drives a real `step_forward` happy path, then proves from `dump_all_nodes()` alone that the advancing `StateAccepted` node carries a `predicate_admission` PASS receipt whose verdict matches a re-execution of `decide_admission` on the same claim set — i.e. the kernel leg reached the same verdict the sequencer would.

**Trust-root boot proof:** after rehashing pins 282/198, `cargo test` boot path must NOT panic `TRUST_ROOT_TAMPERED` (proves the rehash is correct).

---

## 11. Risks

- Trust-root rehash drift: editing `sequencer.rs` (282) without rehashing in the same commit panics boot. Mitigate: rehash + signed tag in the same commit; CI boot is the witness.
- Behavior drift in sequencer zero-root branch: the delegation to `decide_admission` must preserve exact `BTreeMap` iteration order and error mapping. Mitigate: keep the boolean scan byte-identical when moved; assert against existing sequencer zero-root tests.
- Single-admission illusion: if the kernel's stamped claim value diverges from `env.success`, G2 silently passes while the two authorities still disagree. Mitigate: the structural anti-duplication gate (§6) + tie the acceptance claim value to the same `success` bool the env carries.
- `os_qualified` definition risk: deriving it as `root != ZERO` is clean but conflates "bound registry" with "OS-qualified". If the architect wants OS-qualification to be an independent run property, a new field is needed (potential additional surface). Flagged as Open Q.
- Receipt over-claim (G3 boundary): the zero-root receipt must say `registry_root: ZERO` / `os_qualified: false` and must NOT claim oracle re-execution, or it contradicts `constitution_predicate_zero_root_is_not_oracle`.
- Constructor proliferation: `new` + `new_with_predicates` is two constructors; acceptable per "defer abstraction until 2nd impl", but watch for a third variant request.

## 12. Open questions for architect

1. `os_qualified` source: derive from `registry_root != ZERO` (recommended, no schema surface), or a new explicit run-level field? The latter is additional surface.
2. `pub mod predicate_admission;` declaration site: `src/lib.rs` (rehash 198, recommended) vs nested under non-pinned `src/ledger.rs` (zero pinned-declaration edit, uglier)?
3. Receipt shape: payload field `predicate_admission` on `StateAccepted` (recommended, shape a) — confirm we do NOT want a dedicated sibling receipt node (shape b)?
4. Kernel claim pid naming: `judge::<judge_class>` as the synthetic acceptance pid — acceptable, or should it mirror a real registry `PredicateId`?
5. `Cid` type for `PredicateClaim.proof_cid`: confirm the exact CAS Cid type (kernel leg always None, so this only matters if the kernel bound path is ever wired — out of scope for route A).
6. Confirm G4/G5 stay pending under a separate §8 (they are NOT covered by `APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE`).
