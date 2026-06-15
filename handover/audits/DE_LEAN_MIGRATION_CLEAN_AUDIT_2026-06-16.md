# De-Lean Kernel Migration — Clean-Context Audit (2026-06-16)

**Auditor role:** independent clean-context witness (no implementation transcript).
**Repo / branch:** `/Users/zephryj/work/turingosv4` @ `claude/het-carrier-freeze`.
**Spec under audit:** `handover/tracer_bullets/DE_LEAN_KERNEL_MIGRATION_SPEC_2026-06-15.md`
(§8-ratified, 55-atom de-Lean of the generic kernel; math-domain layer kept Lean-named).
**Change state:** in working tree, uncommitted. New test `tests/constitution_de_lean_legacy_decode.rs` untracked (`??`).
**Verdict domain:** EXACTLY `{PASS, VETO}` (constitutional / tape-safety / correctness; no code-style or perf opinions).

---

## VERDICT: PASS

The migration is tape-safe: every historical identity the rename actually changed carries the
correct serde `rename`/`alias` and held its discriminant number; the 3/3 legacy-decode regression
passes; all tape-safety gates (tape_canonical, headline_recompute, matrix_drift,
predicate_registry_immutability) are green; the workspace compiles; all 16 changed trust-root pins
match `genesis_payload.toml`; the §6 restricted edits are within the §8-authorized scope with no
sequencer-admission or typed-tx-discriminant change. The single failing gate
(`constitution_script_liveness_inventory`) is a pre-existing, out-of-scope BearTriage/HET-carrier
red unrelated to this migration.

Three residual Lean-flavored names remain in the generic kernel (see Minor findings). All are
either explicitly documented as intentionally kept (positional-bincode field name) or are private,
non-serialized impl-struct names with zero wire/tape/public-API exposure. None breaks the de-Lean
tape-safety guarantee or constitutes a renamed-without-compat on-wire break, so none rises to a VETO.

---

## FINDINGS

### Blockers
None.

### Minor (non-blocking; incompleteness in the §E source-only / domain-flavor layer)
- `src/top_white/predicates/registry.rs:884` `struct SorryFreePredicate` and `:978`
  `struct LeanArtifactPredicate` — private (`struct`, not `pub`), `#[derive(Debug, Clone)]`
  only (NO `Serialize`/`Deserialize`), confined entirely to `registry.rs` (zero external refs).
  Their serialized identity lives in the renamed `BootPredicateKind` / `PredicateProofKind` enums
  (ordinals held, serde aliases present) and the predicate-id strings (legacy `sorry_free_v1` /
  `lean_artifact_v1` kept registered + new generic `forbidden_token_free_v1` /
  `external_checker_artifact_v1` added). These two names are the spec §E "source-only / fixtures
  genericized" residue — cosmetic, not tape-bearing.
- `src/runtime/librarian_broadcast.rs:129` `pub tactic_class: Option<String>` on
  `PartialProgressSummary` (a serde/CAS-serialized struct the migration otherwise edits). The
  de-Lean pass renamed the adjacent `lean_result_cid → verifier_result_cid` (with alias) but left
  the proof-domain-flavored `tactic_class` name. It was not in the spec's atom list. Identity is
  byte-unchanged by this migration (no decode break), so it is an incompleteness gap, not a regression.
- `src/runtime/attempt_telemetry.rs:826` `pub partial_lean_result_cid: Option<Cid>` — explicitly
  documented in-source as deliberately kept (positional-bincode field; byte position is canonical,
  name is not on-wire; rename declared out-of-scope for this atom set). Acceptable as documented.

---

## CHECK-BY-CHECK

### Check 1 — Tape-safety / Art 0.2 (highest stakes): PASS

(a) **`ObjectType::LeanResult → DomainProofResult`** — `src/bottom_white/cas/schema.rs:407`
carries `#[serde(rename = "LeanResult")]`; assertions at `schema.rs:400-413` prove serialize →
`"LeanResult"` and deserialize `"LeanResult"` → `DomainProofResult`. **`Capability::LeanOracle →
DomainOracle`** — `src/bottom_white/tools/registry.rs:36` carries `#[serde(rename = "LeanOracle")]`;
new test `capability_domain_oracle_wire_string_pinned` (`registry.rs:266-280`) pins the wire string.

(b) **Discriminants held + serde aliases:** `RejectionClass::CheckerFailed = 6`
(`#[serde(alias="LeanFailed")]`) and `IncompleteProofBlocked = 8` (`#[serde(alias="SorryBlocked")]`)
at `rejection_evidence.rs:228-238`; in-file test `de_lean_rejection_class_discriminants_and_aliases_stable`
asserts `as u8 == 6/8`. `AttemptOutcome::{VerifierPass=0 (alias "LeanPass"), VerifierFail=1
(alias "LeanFail"), IncompleteProofBlock=3 (alias "SorryBlock")}`, `VerifierErrorClass::{VerifierFailed=6
(alias "LeanFailed"), IncompleteProofBlocked=8 (alias "SorryBlocked")}`, `VerifierVerdictKind::
IncompleteProofBlocked=3 (alias "SorryBlocked")`, `AttemptKind::SubStep=1 (alias "Tactic")`,
`AbortCause::{WallClockCapDuringVerify=1 (alias "WallClockCapDuringLean"),
VerifierProcessKilledExternally=2 (alias "LeanKilledExternally")}` — all in
`src/runtime/attempt_telemetry.rs`. The schema-id STRING value `"turingosv4.lean_result.v2"` is kept
byte-identical under the renamed const `VERIFIER_RESULT_SCHEMA_ID` (`attempt_telemetry.rs:54`).

(c) **Serde-named fields aliased:** `verifier_result_cid` (`#[serde(alias="lean_result_cid")]`,
`librarian_broadcast.rs:38` + `:129`), `verifier_version` (`alias="lean_version"`) and
`verifier_library_commit` (`alias="mathlib_commit"`) at `benchmark_manifest.rs:16-22`. Bonus:
`LibrarianEvidenceKind::VerifierError` carries `#[serde(alias="LeanError")]`.

**Command:** `cargo test --test constitution_de_lean_legacy_decode`
**Output:** `running 3 tests … test result: ok. 3 passed; 0 failed` — proves historical
`"LeanResult"` / `"LeanFailed"`/`"SorryBlocked"` / `"lean_version"`/`"mathlib_commit"` records
still decode, and the `"LeanResult"` CAS metadata reproduces the identical `canonical_hash` Merkle leaf.

### Check 2 — Completeness: PASS (with noted minors)

Grep of the generic kernel (`src/lib.rs, kernel.rs, bus.rs, state/, runtime/, bottom_white/,
top_white/, economy/`) for `\b(lean|sorry|mathlib)\b` (case-insensitive, minus clean/boolean) →
133 raw hits; 21 non-comment. Triage: all 21 are deliberate `#[serde(alias/rename)]` compat
strings, legacy-recognition read paths (`bus.rs:370-376` `err:tactic_*` returned verbatim on read;
`librarian_broadcast.rs:690` `"lean:Verified"` recognized on read), kept domain-sep literal
(`registry.rs:1235` `b"turingosv4.predicate.lean.expected_statement.v1"` — must not be edited in
place per spec §D), the math-domain checker driver invocation (`registry.rs:1248-1260` `program:
"lean"`, `.lean` filename — the lifted `run_external_checker`), the versioned role-section template
(`real5_roles.rs:382` `V1Legacy => "Lean goal"` byte-exact, `V2Generic => "proof goal"`), pin-test
assertions, regression fixtures (`task:lean:*`, `proofs/test.lean`, `"raw lean stderr"`), and the
documented-kept positional field. A `pub`/wire sweep for Lean-named public types/fns/consts and
unaliased Lean/Sorry enum variants returned empty. Residual active Lean-flavored names are the three
Minor findings above — none on a wire/tape/public-API surface that breaks the de-Lean guarantee.

### Check 3 — Gates green (curated): PASS

**Command:** `bash scripts/run_constitution_gates.sh`
**Output:** `[k-1-5] total=169 failed=1`; sole failure `--test constitution_script_liveness_inventory`.
That gate fails because untracked automation scripts (`scripts/bear_triage_*.{sh,py}`,
`het_carrier_pilot.sh`, `het2_calibration.sh`, `calibration_phase1*.py`, `q2_*.py`,
`analyze_het_carrier_pilot.py`) are not classified in the liveness inventory — a pre-existing
out-of-scope BearTriage/HET-carrier red; the failing set names ZERO `src/` migration files.
The 4 named must-pass gates verified directly:
`constitution_tape_canonical_gate` 7/7, `constitution_headline_recompute_from_tape` 6/6,
`constitution_matrix_drift` 3/3, `constitution_predicate_registry_immutability` 3/3.

### Check 4 — Compile: PASS

**Command:** `cargo check --workspace` → **exit 0** (`Finished dev profile`).

### Check 5 — Pins: PASS

Recomputed `shasum -a 256` for all 16 changed trust-root-pinned files and compared to
`genesis_payload.toml` same-line `"src/path" = "sha256"` entries. All 16 OK:
`cas/schema.rs, cas/store.rs, ledger/rejection_evidence.rs, tools/registry.rs, bus.rs,
runtime/proposal_telemetry.rs, runtime/verification_result.rs, runtime/evidence_capsule.rs,
runtime/chain_derived_run_facts.rs, runtime/librarian_broadcast.rs, runtime/audit_assertions.rs,
state/q_state.rs, state/sequencer.rs, state/typed_tx.rs, top_white/predicates/registry.rs,
bin/audit_dashboard.rs`. No unrehashed (mismatched) changed pinned file.

### Check 6 — §6 scope: PASS

`git diff src/state/sequencer.rs` — edits are exclusively reference renames of the renamed
`AttemptOutcome` variants (`LeanFail→VerifierFail`, `SorryBlock→IncompleteProofBlock`,
`LeanPass→VerifierPass`) and `L4ERejectionClass` variants (`LeanFailed→CheckerFailed`,
`SorryBlocked→IncompleteProofBlocked`) plus comment text; the `refine_rejection_class_*` match-arm
semantics and the debug-assert invariant are byte-equivalent. NO sequencer-admission rule change,
NO discriminant-number change. `git diff src/state/typed_tx.rs` — two doc-comment edits only;
`DOMAIN_SYSTEM_*` signing-domain byte literals (`b"turingosv4.system_sig.*"`) UNCHANGED; no wire
schema / discriminant change. `cas/schema.rs` ObjectType + `bus.rs` label passthrough/comments
match the spec §6-authorized scope.

---

## COMMANDS RUN (summary)
- `git rev-parse --abbrev-ref HEAD` → `claude/het-carrier-freeze`
- `cargo test --test constitution_de_lean_legacy_decode` → 3 passed; 0 failed
- `cargo check --workspace` → exit 0
- `bash scripts/run_constitution_gates.sh` → `[k-1-5] total=169 failed=1` (sole red = script_liveness_inventory, pre-existing/out-of-scope)
- `cargo test --test constitution_tape_canonical_gate --test constitution_headline_recompute_from_tape --test constitution_matrix_drift --test constitution_predicate_registry_immutability` → 7/6/3/3 passed
- `shasum -a 256` × 16 changed pinned files vs `genesis_payload.toml` → all OK
- `git diff` on every spec-named migration file; targeted `rg` triage of all Lean/sorry/mathlib hits in the generic kernel

---

## VERDICT: PASS
