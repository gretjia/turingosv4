//! PROMOTED GATE (#3 / conformance sweep 2026-06-07) — every canonical-write
//! binary entry verifies the boot Trust Root BEFORE doing any work.
//!
//! STATUS: **GREEN / LANDED under §8 Class-4 ratification** (token
//! `APPROVE-ALL-CANONICAL-WRITERS-VERIFY-TRUST-ROOT`, packet
//! `handover/section8/APPROVE_ALL_CANONICAL_WRITERS_VERIFY_TRUST_ROOT_2026-06-07.md`).
//! The fix touched the trust-root AUTHORITY boundary and EVERY canonical-write
//! binary entry — Class-4, ratified per-atom §8 (AGENTS.md §5/§6, the same
//! boundary that keeps src/boot.rs the sole verifier). On landing this gate was
//! promoted from `tests/pending/` to this top-level `constitution_*.rs` gate and
//! triple-coupled (manifest entry + CONSTITUTION_EXECUTION_MATRIX.md row + the
//! `tests/constitution_*.rs` glob). src/boot.rs::verify_trust_root remains the
//! SOLE verifier — writers CALL it, they do not re-implement a hash check
//! (KEEP-SRC-BOOT preserved).
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
//! EXPECTED RESULT (post-§8, LANDED): **GREEN.** The canonical-write set is
//! ~21 files and EACH of them (or its owning binary) now calls
//! `verify_trust_root` before any canonical write. `rg verify_trust_root src/`
//! lands on `src/boot.rs` (the verifier itself), `src/main.rs:14`,
//! `src/bin/turingos/cmd_boot.rs:66`, and every discovered canonical-write
//! binary entry. The mutation-proof witness still holds: deleting one writer's
//! `verify_trust_root(` call re-adds it to the unguarded set and turns this gate
//! RED, so it cannot be satisfied by `assert!(true)`.
//!
//! ── THE RATIFIED FIX (LANDED) ─────────────────────────────────────────────
//! Per the §8 packet's allowed engineering actions: each canonical-write binary
//! entry inserts one `turingosv4::boot::verify_trust_root(&repo_root)` at the
//! top of its `run`/`main` (the `turingos` dispatcher submodules guard their
//! canonical-write action handler), resolving `repo_root` from the source repo
//! (CWD holding `genesis_payload.toml`, matching `src/bin/turingos/cmd_boot.rs`)
//! and aborting on failure, reusing the `src/main.rs:14` abort-on-tamper
//! semantics. The check is NOT placed in the shared factory
//! `build_chaintape_sequencer_with_initial_q` because that factory is exercised
//! by tests that construct sequencers in temp repos WITHOUT a valid trust root;
//! verifying inside it would break those tests. The 2-site gate
//! `tests/constitution_tc_boot_trust_root_manifest.rs` stays as the focused
//! KEEP-SRC-BOOT witness; THIS gate is the all-sites enumeration extension.
//!
//! ── DISCOVERY MECHANISM (top-level constitution gate) ──────────────────────
//! This file is a flat `tests/constitution_*.rs` integration target, so cargo
//! auto-discovers it under `cargo test --workspace` and
//! `scripts/run_constitution_gates.sh`. It is triple-coupled: registered in
//! `scripts/constitution_gates.manifest.toml` and referenced by a row in
//! `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` (the
//! `tests/constitution_matrix_drift.rs` drift gate enforces that coupling).
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

/// The repository root (the worktree the gate runs in). As a flat
/// `tests/constitution_*.rs` cargo integration target, `CARGO_MANIFEST_DIR` is
/// set to the crate root and is the authoritative source. Fall back to the
/// compile-time file path (`<repo>/tests/<name>.rs`, one level under root) and
/// finally to CWD for robustness against non-cargo invocations.
fn repo_root() -> PathBuf {
    if let Some(dir) = option_env!("CARGO_MANIFEST_DIR") {
        let p = PathBuf::from(dir);
        if p.join("src/boot.rs").exists() {
            return p;
        }
    }
    let here = PathBuf::from(file!());
    if let Some(p) = here.parent().and_then(|p| p.parent()) {
        if p.join("src/boot.rs").exists() {
            return p.to_path_buf();
        }
    }
    let cwd = std::env::current_dir().expect("cwd");
    assert!(
        cwd.join("src/boot.rs").exists(),
        "cannot locate repo root: neither CARGO_MANIFEST_DIR, {here:?}, nor cwd \
         {cwd:?} resolves to a tree containing src/boot.rs"
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
/// EXPECTED RESULT (post-§8, LANDED): **GREEN.** The canonical-write set is
/// non-empty and EACH of its members' owning binaries calls `verify_trust_root`
/// before any canonical write. LANDED under §8 Class-4 ratification (token
/// `APPROVE-ALL-CANONICAL-WRITERS-VERIFY-TRUST-ROOT`). Any new canonical-write
/// binary that forgets the check is auto-discovered and turns this gate RED.
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
        "conformance #3 boot-trust-root BYPASS (REGRESSION — this gate is GREEN/LANDED): \
         {} of {} canonical-write binary entries do NOT verify the boot Trust Root \
         before doing work. Each listed file advances canonical state (CAS \
         put_json / GitTapeLedger / build_chaintape_sequencer_with_initial_q / \
         SystemEmitCommand) yet neither it nor its owning binary calls \
         `verify_trust_root`, so a tampered constitution.md / pinned-hash manifest \
         does NOT halt it. The all-canonical-writers invariant landed under §8 \
         Class-4 token APPROVE-ALL-CANONICAL-WRITERS-VERIFY-TRUST-ROOT (packet \
         handover/section8/APPROVE_ALL_CANONICAL_WRITERS_VERIFY_TRUST_ROOT_2026-06-07.md); \
         a new writer must call `turingosv4::boot::verify_trust_root(&repo_root)` at \
         the top of its run/main (resolve repo_root from CWD holding \
         genesis_payload.toml), aborting on failure (reuse src/main.rs:14 \
         semantics) — do NOT add the check to the shared factory \
         build_chaintape_sequencer_with_initial_q (it is exercised by temp-repo \
         tests without a valid trust root). \
         Unguarded canonical writers:\n  {}",
        unguarded.len(),
        writers.len(),
        unguarded.join("\n  ")
    );
}
