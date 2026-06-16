# De-Lean Kernel Migration — Residuals Reminder (for future agents)

**Status (2026-06-16): de-Lean migration LANDED as a milestone (architect: "搞一段落").**
The load-bearing de-Lean is complete + tape-safe + gate-green (curated suite 169 total /
1 failed = the pre-existing out-of-scope `script_liveness_inventory`) + decode-regression
3/3 + clean-context auditor PASS. All Lean-named **types / enum variants (on tape) /
struct fields / CAS object-types / constants / error-class output labels** in the generic
kernel were genericized with correct forward-compat (serde `rename`/`alias`, discriminant
numbers kept, no historical-byte rewrite). Spec: `handover/tracer_bullets/DE_LEAN_KERNEL_MIGRATION_SPEC_2026-06-15.md`.

This file records what was **deliberately kept** (do NOT "fix" these — they are required)
vs the **source-only long-tail deferred** (a future agent may finish if the zero-Lean bar
is to be pushed further).

## A. Deliberately KEPT — required for tape-safety / Art 0.2 (DO NOT change)
These contain the substring "lean"/"sorry" but are forward-compat shims or on-tape values;
changing them would break historical decode (Art 0.2 violation):
- `#[serde(rename = "LeanResult")]` on `ObjectType::DomainProofResult` and
  `#[serde(rename = "LeanOracle")]` on `Capability::DomainOracle` — the on-wire string is
  hashed into the CAS canonical-hash Merkle; must stay byte-identical.
- `#[serde(alias = "LeanFailed"/"SorryBlocked"/"LeanPass"/"LeanFail"/"SorryBlock"/"Tactic"/
  "lean_result_cid"/"lean_version"/"mathlib_commit"/"tactic_class")]` — legacy serde names
  recognized on read for historical JSONL/CAS rows.
- Schema-id value `"turingosv4.lean_result.v2"` (const renamed to `VERIFIER_RESULT_SCHEMA_ID`;
  the VALUE kept for historical record recognition).
- Domain-sep literal `b"turingosv4.predicate.lean.expected_statement.v1"` — hashed into
  `expected_statement_hash` on historical proof capsules; kept byte-for-byte.
- Legacy predicate-ids `"sorry_free_v1"` / `"lean_artifact_v1"` — kept registered/recognized
  alongside the new `forbidden_token_free_v1` / `external_checker_artifact_v1`.
- `bus_classify` legacy arms (`"err:tactic_*"` etc.) — alias-on-read so historical
  `error_class` tape values still pass the finite-set shield (the SOURCE,
  `sdk::error_abstraction::OracleErrClass::label()`, already emits the generic `err:tool_*`).

## B. Architect-PERMITTED Lean-as-predicate-component (NOT a kernel violation)
Architect 2026-06-15: "Lean 可以作为它 predicates 的一个环节" (Lean may be a *component* of a
predicate). So the actual Lean invocation is allowed:
- `src/top_white/predicates/registry.rs::run_external_checker` shells out to the `lean`
  binary, writes `.lean` files, and checks for `sorry`/`admit` tokens. The fn name is
  generic; the Lean coupling inside is the math-domain checker step.
- **Deferred deeper refactor (optional, future):** fully extract a pluggable `DomainChecker`
  trait + a `LeanChecker` impl so even this invocation lives behind a generic interface,
  with the predicate registry calling the generic trait. This is an architecture extraction,
  not a rename — out of scope of the 55-atom §8 rename spec.

## C. Source-only long-tail DEFERRED (a future agent may genericize; none on tape)
None of these break tape-safety or sit on a renamed-without-alias wire surface; the gate
suite stays green with them present. Left as a reminder, not a defect:
- **`OracleErrClass` enum VARIANT names** (`TacticLinarith`, `TacticRingFailed`,
  `TacticSimpNoProgress`, `TacticNormNumFailed`, `TacticOther`, `UnsolvedGoals`,
  `RewriteNoMatch`, …) in `src/sdk/error_abstraction.rs`. These are internal source
  identifiers that map to the now-generic `err:tool_*` OUTPUT strings (the output, which is
  the on-tape value, is already generic; a regression test at error_abstraction.rs asserts
  the OUTPUT labels are not Lean-named). Renaming the variants is source-only (the enum is
  the abstraction point); deferred.
- **Doc-comments** mentioning historical names (`LeanResult`, `LeanFail`, `SorryBlock`,
  `read_lean_result_from_cas`) in `audit_assertions.rs` and cross-file (`attempt_telemetry.rs`,
  `cas/store.rs`) — cosmetic; some accurately describe historical (grandfathered) records.
- **Test fn names** (e.g. `object_type_lean_result_canonical_hash_distinct` in `cas/schema.rs`)
  and a couple of test-fixture strings — test artifacts, not runtime kernel naming.

## D. Unrelated pre-existing issue (NOT de-Lean; do not auto-fix)
- `.claude/hooks/judge.sh` trust-root manifest hash is stale at HEAD (Class-4 trust-root
  inconsistency, pre-existing, clean@HEAD from commit `92c6ffe6`). Out of scope; needs its
  own trust-root protocol + authorization.

## E. What the architect resolved on this milestone
- B-fork (audit_assertions `assert_45`/`assert_48` Lean-named forensic fns): architect ruled
  "rename, keep if gates green." Renamed to `assert_45_verifier_result_retrievable_from_cas` /
  `assert_48_random_verifier_stderr_tamper_detected` (assertion NUMBERS 45/48 kept as the
  stable identity); gate suite stayed 169/1 → rename KEPT.
