#!/usr/bin/env bash
# scripts/run_pending_agentic_os_kill_conditions.sh
#
# M07 PENDING kill-condition runner — PRE-§8 prep (Phase 1, full set).
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
# Scope: the full M07 pending kill-condition set (5 gates):
#   G1 pending_m07_kernel_predicate_gate        -> M07_EXPECTED_RED
#   G2 pending_m07_single_admission             -> SINGLE_ADMISSION_EXPECTED_RED
#   G3 pending_m07_zero_root_is_not_oracle       -> ZERO_ROOT_EXPECTED_RED
#   G4 pending_m07_budget_ceiling_enforced       -> BUDGET_CEILING_STANDING_PENDING
#   G5 pending_m07_fc3_meta_loop_closure         -> FC3_META_LOOP_STANDING_PENDING
# G1+G2+G3 are "fix-coming" reds (land when the single-admission predicate gate
# lands under §8 token APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE). G4+G5 are
# STANDING pending — they additionally require a separate user §8 decision
# (budget hard-ceiling ruling; FC3 irreversible-commit Class-4 ratification).
set -uo pipefail   # NOT -e: we must survive expected failures and tally them.

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO"
mkdir -p target target/pending_gate_bins
OUT="target/pending_kill_conditions_output.txt"
: > "$OUT"

# Each entry: "<binary basename>|<source path>|<standing token printed when RED>"
PENDING_GATES=(
  "pending_m07_kernel_predicate_gate|tests/pending/constitution_kernel_predicate_gate.rs|M07_EXPECTED_RED"
  "pending_m07_single_admission|tests/pending/constitution_kernel_sequencer_single_admission.rs|SINGLE_ADMISSION_EXPECTED_RED"
  "pending_m07_zero_root_is_not_oracle|tests/pending/constitution_predicate_zero_root_is_not_oracle.rs|ZERO_ROOT_EXPECTED_RED"
  "pending_m07_budget_ceiling_enforced|tests/pending/constitution_budget_ceiling_enforced.rs|BUDGET_CEILING_STANDING_PENDING"
  "pending_m07_fc3_meta_loop_closure|tests/pending/constitution_fc3_meta_loop_closure.rs|FC3_META_LOOP_STANDING_PENDING"
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
