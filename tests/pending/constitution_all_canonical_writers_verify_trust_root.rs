//! PENDING GATE (#3 / conformance sweep 2026-06-07) — every canonical-write
//! binary entry verifies the boot Trust Root BEFORE doing any work.
//!
//! STATUS: **STANDING PENDING — EXPECTED RED.** Gated on a USER §8 Class-4
//! RATIFICATION (token `APPROVE-ALL-CANONICAL-WRITERS-VERIFY-TRUST-ROOT`,
//! packet
//! `handover/section8/APPROVE_ALL_CANONICAL_WRITERS_VERIFY_TRUST_ROOT_2026-06-07.md`).
//! The fix touches the trust-root AUTHORITY surface and EVERY canonical-write
//! binary entry — it is Class-4 and MUST NOT be wired without per-atom §8
//! sign-off (AGENTS.md §5/§6, the same boundary that keeps src/boot.rs the sole
//! verifier). Until that ratification this gate stays RED by design and is NOT
//! auto-promoted to a top-level `constitution_*.rs` gate.
//!
//! ── WHY THIS GATE EXISTS (the M07-class bypass it catches) ─────────────────
//! Source: `handover/audits/CONSTITUTION_CONFORMANCE_SWEEP_2026-06-07.md` §2 #3
//! (boot-trust-root, MAJOR) + §3 Gate 3.
//!
//! `art_v3_amendment_log.rs` + the existing gate
//! `tests/constitution_tc_boot_trust_root_manifest.rs:129-140` assert ONLY that
//! `src/main.rs` and `src/bin/turingos/cmd_boot.rs` call
//! `boot::verify_trust_root`. That is the classic M07 SINGLE-SITE ILLUSION: the
//! constitution's intent is "every binary launch verifies the Trust Root", but
//! the gate (and the wiring) covers exactly TWO sites:
//!   * `src/main.rs:14` — the `turingosv4` verify-only binary (no subcommands).
//!   * `src/bin/turingos/cmd_boot.rs:66` — the `turingos boot` subcommand.
//! Meanwhile ~18 OTHER binary entries that actually advance canonical state run
//! with NO trust-root check: `turingos tdma run` (in-proc `GitTapeLedger`
//! writer via `cmd_tdma.rs`), `turingos generate` (`cmd_generate.rs`), ~15
//! `*_current_kernel.rs` runners (`put_json(EvidenceCapsule)` + signed `WorkTx`
//! advancing `state_root`), `fc3_governance_reinit_current_kernel.rs` (emits the
//! highest-trust `MapReduceTick` via the live Sequencer), and
//! `boot_cli_current_kernel_fresh.rs`. Tampering `constitution.md` or the
//! pinned-hash manifest does NOT halt any of them — running `turingos boot`
//! first is an operator CONVENTION, not enforcement.
//!
//! ── WHAT THIS GATE ENFORCES (enumerate-all-sites completeness) ─────────────
//! This is a SOURCE-STRUCTURAL completeness witness in the family of
//! `tests/constitution_single_admission_contract.rs` and
//! `tests/constitution_tc_boot_trust_root_manifest.rs`. It does NOT assert a
//! single hand-picked site; it ENUMERATES every member of the canonical-write
//! class by walking `src/bin/**` and grepping for the canonical-write markers,
//! then asserts the trust-root invariant at EACH discovered site. A future
//! parallel writer that forgets `verify_trust_root` is auto-discovered and turns
//! this gate RED — it cannot be satisfied by `assert!(true)` because the set of
//! sites is derived from the live source tree, not a frozen list.
//!
//! Canonical-write class S = a `src/bin/**` file whose body contains at least
//! one canonical-write marker (the four named in the sweep §3 Gate 3):
//!   * `put_json(` ............. writes a CAS evidence object,
//!   * `GitTapeLedger::` ....... opens/initialises the durable ChainTape writer,
//!   * `build_chaintape_sequencer_with_initial_q` .. builds the live Sequencer
//!                              that admits signed WorkTx advancing state_root,
//!   * `SystemEmitCommand::` / `emit_system_tx(` ... emits a system tx
//!                              (MapReduceTick / governance) on the live tape.
//! Invariant P = the OWNING BINARY of that file calls `verify_trust_root` before
//! work. For a standalone `src/bin/<name>.rs` the owning binary is the file
//! itself. For a `src/bin/turingos/cmd_*.rs` submodule the owning binary is the
//! `turingos` dispatcher (`src/bin/turingos.rs`) — so P is satisfied if either
//! the submodule itself or the dispatcher entry verifies the Trust Root.
//!
//! EXPECTED RESULT AT PRE-§8: **RED.** Today the canonical-write set is
//! ~21 files and NONE of them (nor the `turingos` dispatcher) call
//! `verify_trust_root`; `rg verify_trust_root src/` lands only on
//! `src/boot.rs` (the verifier itself), `src/main.rs:14`, and
//! `src/bin/turingos/cmd_boot.rs:66`. So every discovered canonical writer is a
//! P-violation and the gate asserts RED.
//!
//! ── THE RATIFIED FIX (post-§8, do NOT apply now) ──────────────────────────
//! Insert one `turingosv4::boot::verify_trust_root(&repo_root)?` at the shared
//! runner factory `build_chaintape_sequencer_with_initial_q`
//! (`src/runtime/mod.rs:724`) — or at the top of each canonical-write `main` —
//! and abort on failure, reusing the `src/main.rs:14` semantics. The existing
//! 2-site gate `tests/constitution_tc_boot_trust_root_manifest.rs` must then be
//! EXTENDED from its 2 hand-picked sites to this all-sites enumeration. When the
//! fix lands under §8, every discovered writer verifies and this gate flips
//! GREEN and is promoted to a top-level `constitution_*.rs` gate + manifest +
//! matrix triple-coupling.
//!
//! ── EXCLUSION MECHANISM (same as the M07 G4/G5 pending gates) ──────────────
//! Lives under `tests/pending/` (cargo does NOT auto-discover .rs in tests/
//! SUBDIRECTORIES — only flat tests/*.rs are integration targets), so it is
//! invisible to `cargo test --workspace`. It is NOT added to Cargo.toml as a
//! `[[test]]` target — Cargo.toml is Trust-Root pinned (`genesis_payload.toml`),
//! so any edit would itself trip `verify_trust_root` (TRUST_ROOT_TAMPERED),
//! which is forbidden PRE-§8. It is NOT in
//! `scripts/constitution_gates.manifest.toml`, so neither
//! `scripts/run_constitution_gates.sh` nor `tests/constitution_matrix_drift.rs`
//! sees it. It is compiled + run on demand by
//! `scripts/run_pending_agentic_os_kill_conditions.sh` via `rustc --test`.
//!
//! This gate imports nothing from the crate — it reads the source tree with
//! `std::fs` only, so it links trivially against the cargo-built rlib.

use std::fs;
use std::path::{Path, PathBuf};

/// Canonical-write markers (sweep §3 Gate 3). A `src/bin/**` file containing any
/// of these advances canonical state (CAS object / durable ChainTape / signed
/// WorkTx state_root advance / system tx on the live tape) and therefore MUST
/// verify the Trust Root before doing work.
const CANONICAL_WRITE_MARKERS: &[&str] = &[
    "put_json(",
    "GitTapeLedger::",
    "build_chaintape_sequencer_with_initial_q",
    "SystemEmitCommand::",
    "emit_system_tx(",
];

/// The repository root (the worktree the gate runs in). `rustc --test` does not
/// set `CARGO_MANIFEST_DIR`, so resolve from the source file's compile-time
/// path: this file is `<repo>/tests/pending/<name>.rs`, two levels under root.
fn repo_root() -> PathBuf {
    let here = PathBuf::from(file!());
    // file!() may be relative to the repo root (rustc invoked from there) — walk
    // up from tests/pending/, falling back to CWD which the pending runner sets
    // to the repo root before invoking rustc.
    if let Some(p) = here.parent().and_then(|p| p.parent()).and_then(|p| p.parent()) {
        if p.join("src/boot.rs").exists() {
            return p.to_path_buf();
        }
    }
    let cwd = std::env::current_dir().expect("cwd");
    assert!(
        cwd.join("src/boot.rs").exists(),
        "cannot locate repo root: neither {here:?} nor cwd {cwd:?} resolves to a \
         tree containing src/boot.rs"
    );
    cwd
}

/// True iff `src` contains at least one canonical-write marker.
fn is_canonical_writer(src: &str) -> bool {
    CANONICAL_WRITE_MARKERS.iter().any(|m| src.contains(m))
}

/// True iff `src` calls the Trust-Root verifier (either the fully-qualified
/// `turingosv4::boot::verify_trust_root` or the bare `verify_trust_root(`).
fn verifies_trust_root(src: &str) -> bool {
    src.contains("verify_trust_root(")
}

/// Enumerate every `src/bin/**` `.rs` file (flat bins + the `turingos/`
/// submodule dir). This is the live discovery: any new canonical-write binary
/// added under src/bin is picked up automatically.
fn enumerate_bin_files(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let bin = root.join("src/bin");
    collect_rs(&bin, &mut out);
    out.sort();
    out
}

fn collect_rs(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir).unwrap_or_else(|e| panic!("read_dir {dir:?}: {e}"));
    for entry in entries {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.is_dir() {
            collect_rs(&path, out);
        } else if path.extension().map(|e| e == "rs").unwrap_or(false) {
            out.push(path);
        }
    }
}

/// The set of source files whose `verify_trust_root` presence can satisfy P for
/// a given canonical-write file. For a standalone bin it is the file itself. For
/// a `src/bin/turingos/cmd_*.rs` submodule it additionally includes the
/// dispatcher entry `src/bin/turingos.rs` (the owning binary) and its shared
/// `common.rs`, since a verify there would cover the whole `turingos` binary.
fn owning_binary_closure(root: &Path, file: &Path) -> Vec<PathBuf> {
    let mut closure = vec![file.to_path_buf()];
    let is_turingos_submodule = file
        .parent()
        .and_then(|p| p.file_name())
        .map(|n| n == "turingos")
        .unwrap_or(false);
    if is_turingos_submodule {
        closure.push(root.join("src/bin/turingos.rs"));
        closure.push(root.join("src/bin/turingos/common.rs"));
    }
    closure
}

/// #3 — EVERY canonical-write binary entry verifies the boot Trust Root before
/// work.
///
/// EXPECTED RESULT AT PRE-§8: **RED.** The canonical-write set is non-empty and
/// at least one (today: ALL) of its members' owning binaries do not call
/// `verify_trust_root`. PROMOTION requires §8 Class-4 ratification (token
/// `APPROVE-ALL-CANONICAL-WRITERS-VERIFY-TRUST-ROOT`).
#[test]
fn all_canonical_writers_verify_trust_root() {
    let root = repo_root();
    let bin_files = enumerate_bin_files(&root);

    // Discover the canonical-write class S from the live source tree.
    let mut writers: Vec<PathBuf> = Vec::new();
    for path in &bin_files {
        let src = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
        if is_canonical_writer(&src) {
            writers.push(path.clone());
        }
    }

    // Non-vacuity guard: the canonical-write class must be non-empty. If this
    // ever finds zero writers the markers have drifted and the gate has gone
    // vacuous — fail LOUD rather than green.
    assert!(
        !writers.is_empty(),
        "conformance #3 boot-trust-root gate went VACUOUS: no canonical-write \
         binary entry found under src/bin/** matching any of {CANONICAL_WRITE_MARKERS:?}. \
         The markers have drifted from the codebase; fix the gate before trusting it."
    );

    // For each discovered writer, P = its owning-binary closure verifies the
    // Trust Root.
    let mut unguarded: Vec<String> = Vec::new();
    for writer in &writers {
        let closure = owning_binary_closure(&root, writer);
        let guarded = closure.iter().any(|p| {
            fs::read_to_string(p)
                .map(|s| verifies_trust_root(&s))
                .unwrap_or(false)
        });
        if !guarded {
            let rel = writer
                .strip_prefix(&root)
                .unwrap_or(writer)
                .display()
                .to_string();
            unguarded.push(rel);
        }
    }

    assert!(
        unguarded.is_empty(),
        "conformance #3 boot-trust-root BYPASS (PENDING / STANDING / EXPECTED-RED): \
         {} of {} canonical-write binary entries do NOT verify the boot Trust Root \
         before doing work. Each listed file advances canonical state (CAS \
         put_json / GitTapeLedger / build_chaintape_sequencer_with_initial_q / \
         SystemEmitCommand) yet neither it nor its owning binary calls \
         `verify_trust_root`, so a tampered constitution.md / pinned-hash manifest \
         does NOT halt it. The existing gate \
         tests/constitution_tc_boot_trust_root_manifest.rs asserts ONLY src/main.rs \
         + cmd_boot.rs (the M07 single-site illusion). \
         RATIFIED FIX (post-§8, do NOT apply now): insert one \
         `verify_trust_root(&repo_root)?` at the shared factory \
         build_chaintape_sequencer_with_initial_q (src/runtime/mod.rs:724) or each \
         main top, abort on failure (reuse src/main.rs:14 semantics), and EXTEND \
         the 2-site gate to this all-sites enumeration. PROMOTION requires §8 \
         Class-4 token APPROVE-ALL-CANONICAL-WRITERS-VERIFY-TRUST-ROOT (packet \
         handover/section8/APPROVE_ALL_CANONICAL_WRITERS_VERIFY_TRUST_ROOT_2026-06-07.md). \
         Unguarded canonical writers:\n  {}",
        unguarded.len(),
        writers.len(),
        unguarded.join("\n  ")
    );
}
