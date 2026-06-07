#!/usr/bin/env bash
# scripts/run_pending_agentic_os_kill_conditions.sh
#
# M07 PENDING kill-condition runner — post-§8 residual set (G3/G4/G5).
#
# UPDATE 2026-06-07 (§8 token APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE):
# G1 (pending_m07_kernel_predicate_gate) has been PROMOTED to a live constitution
# gate (tests/constitution_kernel_predicate_gate.rs, registered in
# scripts/constitution_gates.manifest.toml + the execution matrix) and is removed
# from this pending set.
#
# UPDATE 2026-06-07 (§8 packet M07_G2_G3_GATE_REDESIGN_DECISION_2026-06-07.md §5):
# G2 (pending_m07_single_admission) has been RETIRED. The as-written pending G2
# was logically self-contradictory — it asserted kernel_admitted==true AND
# seq_admitted==false AND kernel_admitted==seq_admitted (true==false), so it was
# a broken test, not a falsifiable kill-condition (AGENTS.md §7). It is replaced
# by a CORRECT live behavioral gate
# tests/constitution_single_admission_behavioral.rs (both authorities fed the
# SAME claim must reach the SAME verdict), triple-coupled into the manifest +
# matrix. The structural anti-duplication invariant is additionally enforced live
# by tests/constitution_single_admission_contract.rs. G2 is therefore removed
# from this pending set.
#
# UPDATE 2026-06-07 (§8 token APPROVE-M07-G3-OS-QUALIFIED-RUN-FIELD, Class-4):
# G3 (pending_m07_zero_root_is_not_oracle) has been PROMOTED to a live
# constitution gate (tests/constitution_predicate_zero_root_is_not_oracle.rs,
# registered in scripts/constitution_gates.manifest.toml + the execution matrix).
# The architect ruling on the os_qualified source landed as the run-level
# QState::os_qualified_t field (independent of predicate_registry_root_t, folded
# into state_root_t, flipped true by the system-only PredicateBindingActivate
# accept), which makes the zero-root refuse-path live. G3 is removed from this
# pending set. G4/G5 remain STANDING pending a separate §8.
#
# ADDED 2026-06-07 (constitution conformance sweep, finding #3 boot-trust-root,
# Class-4, NO §8 token yet):
#   pending_conformance_all_canonical_writers_verify_trust_root
#     -> ALL_CANONICAL_WRITERS_VERIFY_TRUST_ROOT_STANDING_PENDING
#   tests/pending/constitution_all_canonical_writers_verify_trust_root.rs is an
#   ENUMERATE-ALL-SITES completeness gate: it walks src/bin/** and asserts EVERY
#   canonical-write binary entry (CAS put_json / GitTapeLedger / live Sequencer /
#   SystemEmitCommand) verifies the boot Trust Root before work. RED today (~21
#   writers, none verify; only src/main.rs + cmd_boot.rs do — the M07 single-site
#   illusion). The fix touches the trust-root AUTHORITY surface + every
#   canonical-write entry → Class-4, STANDING PENDING §8 token
#   APPROVE-ALL-CANONICAL-WRITERS-VERIFY-TRUST-ROOT (packet
#   handover/section8/APPROVE_ALL_CANONICAL_WRITERS_VERIFY_TRUST_ROOT_2026-06-07.md).
#   Source: handover/audits/CONSTITUTION_CONFORMANCE_SWEEP_2026-06-07.md §2 #3.
#
# Runs the pending agentic-OS kill-condition gate set that is DELIBERATELY
# EXCLUDED from default CI. The exclusion mechanism is ZERO-CARGO.TOML:
#
#   * the gate files live under tests/pending/ — cargo does NOT auto-discover
#     .rs files in tests/ SUBDIRECTORIES (only flat tests/*.rs are integration
#     targets), so they are invisible to `cargo test` / `cargo test --workspace`
#     with NO Cargo.toml change at all. (Proven by the long-standing
#     tests/pending_probe/ invalid-Rust file that never breaks CI.)
#   * we do NOT add a [[test]] target to Cargo.toml: on this worktree Cargo.toml
#     is pinned in the Trust Root (genesis_payload.toml), so ANY edit to it
#     trips src/boot.rs::verify_trust_root (Class-4 TRUST_ROOT_TAMPERED) — which
#     is forbidden PRE-§8. We therefore compile each pending gate as a STANDALONE
#     test binary via `rustc --test`, linking the cargo-built `turingosv4` rlib.
#   * they are NOT in scripts/constitution_gates.manifest.toml, so neither
#     scripts/run_constitution_gates.sh (flat `ls tests/constitution_*.rs` glob)
#     nor tests/constitution_matrix_drift.rs (manifest-driven) sees them.
#
# These gates are EXPECTED TO FAIL (red) until the Class-4 src/ admission change
# lands under the user's §8 token. This script:
#   - builds the turingosv4 test deps once (so the rlib + dep rlibs exist),
#   - compiles each pending gate standalone via rustc --test,
#   - runs it, treats FAIL (non-zero) as the EXPECTED outcome and prints the
#     gate's standing token,
#   - errors LOUDLY if a pending gate unexpectedly PASSES (premature wire-up or
#     a vacuous assertion that must be fixed before §8) or fails to COMPILE
#     (a compile break is a real defect, not the intended assertion-red), and
#   - exits 0 iff every pending gate is in its expected (compiles + asserts-red)
#     state.
#
# Explicitly NOT a constitution gate. Does NOT run inside
# scripts/run_constitution_gates.sh and does NOT block default CI. Mirrors the
# report-not-enforce house style of scripts/audit_legacy_bypass.sh.
#
# Scope: the M07 residual pending kill-condition set + the 2026-06-07 conformance
# sweep boot-trust-root gate (3 gates; G1 + G3 promoted out, G2 retired +
# replaced by a live behavioral gate):
#   G4 pending_m07_budget_ceiling_enforced       -> BUDGET_CEILING_STANDING_PENDING
#   G5 pending_m07_fc3_meta_loop_closure         -> FC3_META_LOOP_STANDING_PENDING
#   #3 pending_conformance_all_canonical_writers_verify_trust_root
#                                                -> ALL_CANONICAL_WRITERS_VERIFY_TRUST_ROOT_STANDING_PENDING
# G4+G5 are STANDING pending — they additionally require a separate user §8
# decision (budget hard-ceiling ruling; FC3 irreversible-commit Class-4
# ratification). G1 was promoted to the live gate
# tests/constitution_kernel_predicate_gate.rs, G2 to
# tests/constitution_single_admission_behavioral.rs, and G3 to
# tests/constitution_predicate_zero_root_is_not_oracle.rs under their §8 tokens.
set -uo pipefail   # NOT -e: we must survive expected failures and tally them.

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO"
mkdir -p target target/pending_gate_bins
OUT="target/pending_kill_conditions_output.txt"
: > "$OUT"

# Each entry: "<binary basename>|<source path>|<standing token printed when RED>"
PENDING_GATES=(
  "pending_m07_budget_ceiling_enforced|tests/pending/constitution_budget_ceiling_enforced.rs|BUDGET_CEILING_STANDING_PENDING"
  "pending_m07_fc3_meta_loop_closure|tests/pending/constitution_fc3_meta_loop_closure.rs|FC3_META_LOOP_STANDING_PENDING"
  "pending_conformance_all_canonical_writers_verify_trust_root|tests/pending/constitution_all_canonical_writers_verify_trust_root.rs|ALL_CANONICAL_WRITERS_VERIFY_TRUST_ROOT_STANDING_PENDING"
)

unexpected_pass=0
compile_break=0
expected_red=0

log() { echo "$@" | tee -a "$OUT"; }

# Build the turingosv4 test profile once so the rlib + transitive dep rlibs are
# present in target/debug/deps for the standalone rustc link below.
log "=== building turingosv4 test deps (cargo build --tests) ==="
if ! cargo build --tests >>"$OUT" 2>&1; then
  log "RESULT: BUILD-PRECONDITION-FAILED — cargo build --tests did not succeed;" \
      "cannot link pending gates. This is a real build break, investigate."
  exit 1
fi

RLIB="$(ls -t target/debug/deps/libturingosv4-*.rlib 2>/dev/null | head -1)"
if [ -z "${RLIB}" ]; then
  log "RESULT: RLIB-NOT-FOUND — no target/debug/deps/libturingosv4-*.rlib after" \
      "build; cannot link pending gates."
  exit 1
fi
log "linking against rlib: ${RLIB}"

# Resolve --extern flags for the extra crates the pending gates use directly
# (tokio for the current-thread runtime that drives Sequencer::submit; tempfile
# for CasStore::open). turingosv4 is always present; others are best-effort and
# only matter for gates that import them. `-L dependency=target/debug/deps` lets
# rustc resolve all further transitive deps by hash.
EXTERNS=(--extern "turingosv4=${RLIB}")
for crate in tokio tempfile serde_json serde; do
  rl="$(ls -t target/debug/deps/lib${crate}-*.rlib 2>/dev/null | head -1)"
  if [ -n "${rl}" ]; then
    EXTERNS+=(--extern "${crate}=${rl}")
    log "  extern ${crate}: ${rl}"
  fi
done
log

run_gate() {            # $1 = binary basename  $2 = source path  $3 = standing token
  local bin="$1" src="$2" token="$3"
  local out_bin="target/pending_gate_bins/${bin}"
  log "=== PENDING GATE: ${bin} (expected: compiles + asserts RED -> ${token}) ==="

  # Standalone compile via rustc --test (NO Cargo.toml [[test]] entry; keeps the
  # Trust-Root-pinned Cargo.toml untouched).
  if ! rustc --edition 2021 --test \
        "${EXTERNS[@]}" \
        -L dependency=target/debug/deps \
        -o "${out_bin}" \
        "${src}" >>"$OUT" 2>&1; then
    log "  !! COMPILE BREAK: ${src} failed to compile against the current public" \
        "API. A pending gate must COMPILE (only its assertion may be red). Fix the" \
        "gate or the API drift before §8."
    compile_break=$((compile_break + 1))
    log
    return
  fi

  # Run the compiled gate binary. RED (non-zero) is the expected PRE-§8 state.
  if "${out_bin}" --test-threads=1 >>"$OUT" 2>&1; then
    log "  !! UNEXPECTED PASS: ${bin} passed, but the Class-4 src/ admission" \
        "topology is NOT yet wired (§8 pending). ${token} was expected to be RED."
    unexpected_pass=$((unexpected_pass + 1))
  else
    log "  ${token}  (gate red as expected — standing pending §8)"
    expected_red=$((expected_red + 1))
  fi
  log
}

for entry in "${PENDING_GATES[@]}"; do
  IFS='|' read -r gbin gsrc gtoken <<< "${entry}"
  run_gate "${gbin}" "${gsrc}" "${gtoken}"
done

log "=== SUMMARY ==="
log "  expected-red (pending) : ${expected_red}"
log "  unexpected-pass        : ${unexpected_pass}"
log "  compile-break          : ${compile_break}"

if [ "${compile_break}" -gt 0 ]; then
  log "RESULT: PENDING-GATE-COMPILE-BREAK — a pending gate no longer compiles" \
      "against the public API; fix before §8."
  exit 1
fi
if [ "${unexpected_pass}" -gt 0 ]; then
  log "RESULT: PENDING-GATE-UNEXPECTEDLY-GREEN — investigate premature wire-up" \
      "or a vacuous assertion before §8."
  exit 1
fi

log "RESULT: ALL-PENDING-RED-AS-EXPECTED (standing pending §8 token)"
exit 0
