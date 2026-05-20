# tests/tb_*.rs — Historical Tracer-Bullet Tests (K-1.4'-lite ARCHIVE)

This directory contains 59 `tb_*.rs` files from the TB-1 through TB-18R historical
tracer-bullet development era (2026-04-30 through 2026-05-07). They are kept in
`tests/` (not physically moved to `tests-archive/`) per K-1.4'-lite scope, to
preserve existing constitution-gate witness bindings without risk.

## Why these stay in tests/

Several `tests/constitution_*.rs` gate files use these tb_*.rs paths as witness
bindings via `Path::new("tests/tb_*.rs").exists()` or `std::fs::read_to_string("tests/tb_*.rs")`
calls. The bindings are:

| Binding gate | Referenced TB tests | Constitutional purpose |
|--------------|---------------------|------------------------|
| `constitution_fc1_runtime_loop.rs:162,204,212` | tb_16_dashboard_live_regen, tb_18r_audit_lean_stderr_tamper_detected, tb_18r_audit_sampler_attempt_payload | FC1-INV5 witness binding |
| `constitution_fc2_boot.rs:167-169` | tb_13_chaintape_smoke, tb_14_chaintape_smoke, tb_18r_chain_attempt_invariant | FC2 boot binding |
| `constitution_fc3_meta.rs:65` | tb_17_markov_inheritance_policy | FC3 markov binding |
| `constitution_predicate_gate.rs:208` | tb_13_legacy_cpmm_forward_fence | predicate fence binding |
| `constitution_no_evidence_drift_in_tests.rs:123-125` | tb_7_atom6_chain_backed_smoke, tb_13_chaintape_smoke, tb_14_chaintape_smoke | evidence drift |
| `constitution_tape_canonical_gate.rs:95,167` | tb_16_dashboard_live_regen, tb_18r_attempt_telemetry_per_llm_call | tape canonical |

Physically moving the tb_*.rs files would require:
- Setting up `tests-archive/` as a Rust workspace member
- Updating all 6 binding file path literals (Class-3 risk per v3 plan §2 red line)
- Per-atom architect §8 verbatim ratification (beyond `/goal`'s general grant)

## K-1.4'-full migration path

When future architect §8 explicitly authorizes the full path-binding migration:

1. Create `tests-archive/` workspace member (Cargo.toml + tests/ subdir)
2. `git mv tests/tb_*.rs tests-archive/tests/` (59 files)
3. Update root `Cargo.toml` to add `members = [".", "tests-archive"]`
4. Update the 6 binding files' string literals: `"tests/tb_"` → `"tests-archive/tests/tb_"`
5. Verify `cargo test --workspace --no-fail-fast` still green
6. Verify `bash scripts/run_constitution_gates.sh` still green

K-1.4'-lite (this README) marks the archival intent without moving files,
preserving the 6 binding paths intact under the cumulative `/goal` grant.

## Status as of K-1.4'-lite ship

- 59 tb_*.rs files: in `tests/` (unchanged)
- Bindings: intact (6 constitution_*.rs files reference them)
- Future tightening: full K-1.4' migration requires explicit architect §8
