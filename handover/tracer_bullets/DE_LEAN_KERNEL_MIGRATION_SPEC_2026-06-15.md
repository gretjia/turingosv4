# De-Lean the Generic Kernel — Class-4 §8 Migration Spec (2026-06-15)

**Status: AWAITING ARCHITECT §8 RATIFICATION.** No §6/pinned surface is edited until ratified.

**Authority chain**: architect principle (repeatedly stated) — the generic kernel must carry
NO Lean naming/types/constants; Lean is one pluggable math-domain tool. Architect 2026-06-15
sequencing ruling: **de-Lean the kernel FIRST, then the H-HET-2 paid run.** Design produced by
workflow `wf_62370b04-e6f` (4 agents: 3 parallel region sweeps + 1 design/adversarial-verify),
encoding facts independently re-verified against source.

**Verdict: SAFE_DESIGN** — every atom has a tape-safe forward-compat plan; no historical L4 /
L4.E / CAS byte is rewritten (Art 0.2 preserved). Two pre-land gates closed (below).

---

## The principle being enforced

Generic core (`src/lib.rs`, `kernel.rs`, `bus.rs`, `state/`, `runtime/`, `bottom_white/`,
`top_white/`, `economy/`) must be Lean-free. The math-domain layer (`src/judges/lean_*`,
`src/bin/lean_market_agent.rs`, the `lean` checker driver) keeps Lean naming — that is correct.
Confirmed: `lean_market_agent` is a BIN (leaf vehicle), not referenced by the kernel in code.
The H-HET-2 modules `runtime::routing_policy` + `runtime::budget_allocation_telemetry` are
already Lean-free. The leak is **pre-existing** (TB-18R era, ~2026-05), not from H-HET-2.

## The core safety mechanism (why this is not a find-replace)

The canonical CAS encoder is **positional bincode** (`transition_ledger.rs:1214-1224`,
big-endian fixed-int) — so renaming a struct FIELD is wire-safe. BUT three identities are burned
into historical tape by NAME/NUMBER and need forward-compat, not a raw rename:
- **enum discriminants** hashed into the L4.E RejectionDigest by NUMBER (`rejection_evidence.rs:329`)
  → rename the source variant, **keep the number** (`LeanFailed=6` stays `=6`).
- **serde variant NAMES** in the JSONL rejection sidecar + `agent_audit_trail.rs:180`
  → add `#[serde(alias="LeanFailed")]` so historical rows still deserialize.
- **`ObjectType::LeanResult`** serde-NAME-burned into the CAS canonical_hash Merkle
  (`schema.rs:162`, pinned by test `:388`) → **`#[serde(rename="LeanResult")]`** to keep the
  on-wire string identical; a naive variant rename would silently change every historical CAS
  object's hash and break the whole Merkle reconstruction. **Highest-blast-radius atom.**

## Migration atoms (55), by class

### A. cas_object_type_needs_alias (3) — §6 / highest stakes, must land atomically
- `ObjectType::LeanResult` → `ObjectType::DomainProofResult` — **def `cas/schema.rs:103` (§6)** + serde identity `:162` + pin-test `:388`; consume sites `attempt_telemetry.rs:953`, `librarian_broadcast.rs:518,574`. Keep on-wire `"LeanResult"` via `#[serde(rename)]`.
- `Capability::LeanOracle` → `Capability::DomainOracle` — `bottom_white/tools/registry.rs:27` + test `:241`. Same serde-rename treatment.

### B. discriminant_rename_keep_number (15) — keep the number, add serde alias where name-serialized
- `RejectionClass::LeanFailed=6` → `CheckerFailed=6` (+`#[serde(alias="LeanFailed")]`), `SorryBlocked=8` → `IncompleteProofBlocked=8` (+alias) — def `rejection_evidence.rs:228,236`; refs `sequencer.rs:955,957` (§6).
- `AttemptOutcome::{LeanPass=0,LeanFail=1,SorryBlock=3}` → `{VerifierPass,VerifierFail,IncompleteProofBlock}` — `attempt_telemetry.rs:159,163,169`; refs `sequencer.rs`.
- `LeanErrorClass::{LeanFailed=6,SorryBlocked=8}` → `VerifierErrorClass::{VerifierFailed,IncompleteProofBlocked}` — `attempt_telemetry.rs:209,212,219`.
- `LeanVerdictKind::SorryBlocked=3` → `VerifierVerdictKind::IncompleteProofBlocked=3` — `attempt_telemetry.rs:242,257`.
- `AttemptKind::Tactic=1` → `SubStep=1`; `AbortCause::{WallClockCapDuringLean,LeanKilledExternally}` → `{WallClockCapDuringVerify,VerifierProcessKilledExternally}`.
- `BootPredicateKind::{SorryFree,LeanArtifact}` → `{ForbiddenTokenFree,ExternalCheckerArtifact}`; `PredicateProofKind::LeanArtifact` → `ExternalCheckerArtifact` — `top_white/predicates/registry.rs` (keep ordinals).

### C. serde_field_needs_legacy_decoder (3) — `#[serde(alias)]`
- `PartialProgressSummary.lean_result_cid` → `verifier_result_cid` (+alias) — `librarian_broadcast.rs:116,660`.
- `BenchmarkManifest.lean_version` → `verifier_version` (+alias), `.mathlib_commit` → `verifier_library_commit` (+alias) — `benchmark_manifest.rs:47,49`.

### D. string_label_needs_alias (9, extended to full OracleErrClass=12) — emit generic, recognize legacy on read
- `OracleErrClass` 12 variants `"err:tactic_linarith|simp_noprog|ring|norm_num|other|unknown_const|unsolved_goals|unexpected_token|type_mismatch|rewrite_no_match|heartbeat|other"` → generic `err:tool_*` (source `error_abstraction.rs:44-55`, on tape via CAS `error_class` `schema.rs:101`; bus.rs:344-350 is passthrough).
- `"lean:Verified"/"lean:PartialAccepted"` → `"verify:*"` — `librarian_broadcast.rs:420,425,521,522,651` (going-forward; legacy recognized on read).
- `LEAN_RESULT_SCHEMA_ID="turingosv4.lean_result.v2"` → `VERIFIER_RESULT_SCHEMA_ID` (keep recognizing old id) — `attempt_telemetry.rs:92`.
- predicate ids `"lean_artifact_v1"`/`"sorry_free_v1"` → `external_checker_artifact_v1`/`forbidden_token_free_v1` as NEW ids; legacy ids stay registered + the legacy domain-sep literal `turingosv4.predicate.lean.expected_statement.v1` usable by a legacy verifier (hashed into code_hash/expected_statement_hash — do NOT edit in place) — `registry.rs:636,649,1186`.
- EvidenceCapsule/`real5_roles` summary + "Lean goal" view labels → generic in NEW capsules/views only (content-hashed → never re-emit historical; versioned role-section template so old PromptCapsule replay reproduces old bytes).

### E. source_only_rename (23) — positional-bincode fields / comments / non-serialized types
`LeanResult` type → `VerifierResult`; `VerificationResult.{lean_exit_code,lean_stdout_hash,lean_stderr_hash}` → `{exit_code,stdout_hash,stderr_hash}`; `from_lean_run`→`from_verifier_run`; `EvidenceCapsule/ExhaustionCounts.{lean_error_count,sorry_block_count}` → `{verifier_error_count,incomplete_proof_block_count}`; `AttemptTelemetry.lean_result_cid`→`verifier_result_cid`; `ProposalTelemetry.candidate_tactic`→`candidate_label`; `ChainDerivedRunFacts.{tactic_diversity,gp_proof_file}`→`{method_diversity,gp_output_file}` (gate-1-verified derived-only, not tape-serde); `LibrarianEvidenceKind::LeanError`→`VerifierError`; `PredicateVerifyError::{LeanCheckerFailed,LeanCheckerUnavailable}`→`External*`; `BenchmarkManifestError::{EmptyLeanVersion,InvalidMathlibCommit}`→generic; assorted doc-comments (`typed_tx.rs:909,125`, `q_state.rs:797`, `bus.rs:179-194`) scrubbed; test fixtures/names genericized.

### F. keep_as_is / lift_to_domain_layer (2)
- `fn run_lean_checker` (shells out to `lean`) → `run_external_checker` lifting the `lean` binary + `.lean` into a pluggable math-domain checker driver — `registry.rs:1202`.
- `forbidden_patterns` literals `"native_decide"/"axiom "` → belong in the math-domain boot catalog, not the generic predicate registry — `registry.rs:628-630`.

## Tape-compat risks (all mitigated → SAFE_DESIGN)
1. RejectionClass dual-encoding (number safe + serde-name) → mandatory `#[serde(alias)]`; `agent_audit_trail.rs` (pinned) rehashed.
2. `ObjectType::LeanResult` serde-name in Merkle → `#[serde(rename="LeanResult")]` (highest stakes, §6).
3. `real5_roles` "Lean goal" in PromptCapsule visible_context_cid → versioned role-section template (old HEAD reproduces old bytes).
4. `lean:Verified` + EvidenceCapsule summaries content-hashed → alias-on-read; never re-emit historical objects.
5. predicate_id / domain-sep literal hashed into code_hash → new ids/version; legacy kept verifiable.
6. ChainDerivedRunFacts source-only **gate-1 CLOSED**: derived-only / dashboard-`{}`-rendered, not tape-serde.

## Pre-land gates
- **Gate 1 (ChainDerivedRunFacts serde-by-name)**: CLOSED — derived-only/dashboard-text, no canonical-tape serde persistence.
- **Gate 2 (full OracleErrClass list)**: CLOSED — 12 variants enumerated; all get the string-label-alias treatment.
- **Gate 3 (atomic ObjectType landing)**: the `schema.rs` §6 variant rename + its consume sites land in one commit (enforced at implementation).

## §6 restricted surfaces touched
`src/bottom_white/cas/schema.rs` (ObjectType variant def + serde identity), `src/state/typed_tx.rs` + `src/state/sequencer.rs` (RejectionClass/AttemptOutcome refs), `src/bus.rs` (label passthrough + comments).

## Trust-root pins to rehash (~16, per genesis_payload.toml)
`cas/schema.rs`, `cas/store.rs`, `ledger/rejection_evidence.rs`, `ledger/transition_ledger.rs` (only if codec touched — not planned), `tools/registry.rs`, `runtime/librarian_broadcast.rs`, `runtime/proposal_telemetry.rs`, `runtime/chain_derived_run_facts.rs`, `runtime/verification_result.rs`, `runtime/evidence_capsule.rs`, `state/q_state.rs`, `state/sequencer.rs`, `state/typed_tx.rs`, `bus.rs`, `top_white/predicates/registry.rs`, `runtime/agent_audit_trail.rs`. (Non-pinned, also edited: `attempt_telemetry.rs`, `real5_roles.rs`, `benchmark_manifest.rs`, `sdk/error_abstraction.rs`.)

## Implementation plan (after §8)
Land as ONE coherent migration (atoms are interdependent — a variant rename requires updating all
refs atomically). Gate-first: the constitution suite's tape-canonical / recompute / historical-replay
gates (`constitution_tape_canonical_gate`, `tb_7_legacy_append_regression`, `constitution_headline_recompute_from_tape`, the v1-legacy-decode tests) MUST stay green — they are the machine check that no historical decode broke. Add a regression test that a historical `LeanResult`/`LeanFailed`-named record still decodes after the rename (mirrors the proposal_telemetry v1→v2 precedent). Then rehash the ~16 pins under §8, `cargo test --workspace`, `bash scripts/run_constitution_gates.sh`, clean-context audit (independent witness), commit.

## Architect §8 approval block (paste to ratify)
```
ARCHITECT §8 — DE-LEAN KERNEL MIGRATION (2026-06-15)
Verdict: [APPROVED / APPROVED AS AMENDED / VETO]
Authorizes: editing §6 restricted surfaces (cas/schema.rs ObjectType, typed_tx.rs/sequencer.rs
RejectionClass refs, bus.rs) + the ~16 trust-root pin rehashes, to land the 55-atom de-Lean
migration per this spec (tape-safe forward-compat: keep discriminant numbers, serde rename/alias
to preserve on-wire identities, no historical-byte rewrite). Implementation gate-first + clean-
context audit; constitution tape-canonical/recompute gates must stay green.
[amendments, if any]
```
