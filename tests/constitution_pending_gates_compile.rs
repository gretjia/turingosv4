//! LIVE CONSTITUTION GATE — every `tests/pending/*.rs` gate must still COMPILE.
//!
//! STATUS: LIVE / GREEN. Class 2 (read-only build-check; SOURCE-STRUCTURAL — it
//! compiles other test files but mutates no production state). This gate closes
//! the CI bit-rot hole that the 2026-06-07 constitution conformance sweep found:
//! the standing-pending kill-condition gates under `tests/pending/` are
//! DELIBERATELY excluded from `cargo test` (cargo does not auto-discover .rs
//! files in tests/ SUBDIRECTORIES, and they are not in the gate manifest), so
//! their RED assertions never block CI. The side effect was that API drift in
//! those files (a renamed field, a changed signature) rotted SILENTLY — nothing
//! in default CI ever compiled them, so a pending gate could quietly stop
//! compiling and no one would notice until the next manual
//! `scripts/run_pending_agentic_os_kill_conditions.sh` run.
//!
//! ── WHAT THIS GATE ENFORCES ───────────────────────────────────────────────
//! For EVERY file under `tests/pending/`, this gate type-checks it against the
//! CURRENT public `turingosv4` API and asserts it COMPILES. It does this with
//! `rustc --test --emit=metadata` — the metadata-only pass type-checks the whole
//! file (field access, method resolution, trait bounds, imports) but performs NO
//! codegen, NO link, and crucially NEVER RUNS the test body. So the pending
//! gate's RED standing-pending assertion is NOT executed here: a pending gate
//! that is correctly "compiles + standing-red" keeps THIS gate GREEN, while a
//! pending gate that no longer compiles against the live API turns THIS gate
//! RED, naming the offending file. The pending assertions stay excluded from CI
//! exactly as designed; only their COMPILABILITY is now enforced.
//!
//! This mirrors the COMPILE step in
//! `scripts/run_pending_agentic_os_kill_conditions.sh` (same `rustc --test`
//! mechanism, same `turingosv4` rlib + best-effort extern crates resolved from
//! `target/debug/deps`), minus the run-and-expect-red step. The runner remains
//! the place that asserts the pending gates are standing-RED; this gate is the
//! always-on CI tripwire that they still COMPILE.
//!
//! ── HOW IT FINDS THE RLIB ─────────────────────────────────────────────────
//! `cargo test` always builds the `turingosv4` lib before running its
//! integration tests, so by the time this test body runs, a current
//! `target/debug/deps/libturingosv4-*.rlib` (+ the dependency rlibs) is
//! guaranteed present and up to date with the source under test. We pick the
//! newest matching rlib (the just-built one) and resolve the same extra externs
//! the pending runner does. Transitive deps resolve via
//! `-L dependency=target/debug/deps`.
//!
//! ── NON-VACUITY ───────────────────────────────────────────────────────────
//! The gate fails LOUD rather than passing trivially if: there are zero pending
//! files (the directory must contain at least the M07 G4/G5 gates — an empty
//! set would make the for-all vacuous), `rustc` is not on PATH, or no
//! `libturingosv4-*.rlib` exists under `target/debug/deps`. It cannot be
//! satisfied by `assert!(true)`: every assertion observes the live filesystem /
//! a real `rustc` invocation.
//!
//! ── TRIPLE-COUPLING ───────────────────────────────────────────────────────
//! Registered in `scripts/constitution_gates.manifest.toml`
//! (`constitution_pending_gates_compile`) and referenced in
//! `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`. Discovered by the flat
//! `ls tests/constitution_*.rs` glob in `scripts/run_constitution_gates.sh`.
//!
//! ── RECORDED MUTANT (the bit-rot this gate catches) ───────────────────────
//! Rename `QState::genesis` -> `QState::genesis2` in src/ (an API drift). The
//! pending G4 gate (`tests/pending/constitution_budget_ceiling_enforced.rs`,
//! which calls `QState::genesis()`) would stop compiling. Before this gate:
//! `cargo test` stays GREEN (the pending file is never compiled in CI), the rot
//! is silent. After this gate: `every_pending_gate_compiles` turns RED naming
//! `constitution_budget_ceiling_enforced.rs`. That proves the gate is the
//! always-on tripwire for pending-gate bit-rot.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Directory holding the deliberately-CI-excluded standing-pending gates.
const PENDING_DIR: &str = "tests/pending";
/// Where the cargo build dropped the `turingosv4` rlib + dep rlibs.
const DEPS_DIR: &str = "target/debug/deps";

/// Extra crates the pending gates may import directly (beyond `turingosv4`),
/// resolved best-effort from `target/debug/deps`. Mirrors the EXTERNS list in
/// `scripts/run_pending_agentic_os_kill_conditions.sh`. `turingosv4` itself is
/// resolved separately and is REQUIRED; these are best-effort and only matter
/// for the pending gates that import them.
const BEST_EFFORT_EXTERNS: &[&str] = &["tokio", "tempfile", "serde_json", "serde"];

/// Newest `lib<crate>-<hash>.rlib` under `target/debug/deps`, if any. "Newest"
/// matches the pending runner's `ls -t | head -1` so we link the just-built
/// artifact (not a stale sibling from an earlier hash).
fn newest_rlib(crate_name: &str) -> Option<PathBuf> {
    let prefix = format!("lib{crate_name}-");
    let mut best: Option<(std::time::SystemTime, PathBuf)> = None;
    let entries = fs::read_dir(DEPS_DIR).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };
        if name.starts_with(&prefix) && name.ends_with(".rlib") {
            let mtime = entry
                .metadata()
                .and_then(|m| m.modified())
                .unwrap_or(std::time::UNIX_EPOCH);
            match &best {
                Some((best_t, _)) if *best_t >= mtime => {}
                _ => best = Some((mtime, path.clone())),
            }
        }
    }
    best.map(|(_, p)| p)
}

/// Collect every `*.rs` file under `tests/pending/`, sorted for stable output.
fn pending_gate_files() -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = fs::read_dir(PENDING_DIR)
        .unwrap_or_else(|e| panic!("pending dir {PENDING_DIR} must be readable: {e}"))
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("rs"))
        .collect();
    files.sort();
    files
}

/// Vacuity guard: the pending set must be non-empty. An empty `tests/pending/`
/// would make `every_pending_gate_compiles` a trivially-passing for-all over
/// nothing — exactly the silent-degradation this gate exists to prevent. At
/// least the M07 G4/G5 standing-pending gates must be present.
#[test]
fn pending_gate_set_is_non_empty() {
    let files = pending_gate_files();
    assert!(
        !files.is_empty(),
        "pending-gate compile gate is VACUOUS: {PENDING_DIR}/ contains zero .rs \
         files. The standing-pending kill-condition gates (M07 G4 budget ceiling, \
         G5 FC3 meta-loop) must live here; an empty directory makes \
         `every_pending_gate_compiles` pass over nothing. If the pending set was \
         intentionally emptied (all gates promoted), retire THIS gate too under \
         explicit authority instead of leaving it vacuous."
    );
}

/// Every file under `tests/pending/` must COMPILE (type-check) against the
/// current public API — WITHOUT running its red assertion. Uses
/// `rustc --test --emit=metadata`: full type-check, no codegen, no link, body
/// never executed. RED iff a pending gate no longer compiles (API drift), GREEN
/// when every pending gate compiles (its assertion may still be standing-red —
/// that is checked by the pending runner, not here).
#[test]
fn every_pending_gate_compiles() {
    // rustc must be available — fail loud, not vacuous, if the toolchain is
    // missing (e.g. a stripped CI image), so a "GREEN" can never mean "we never
    // actually compiled anything".
    let rustc = which_rustc();

    // The turingosv4 rlib MUST exist: `cargo test` builds the lib before running
    // this integration test, so its absence means the build invariant is broken.
    let turingos_rlib = newest_rlib("turingosv4").unwrap_or_else(|| {
        panic!(
            "no libturingosv4-*.rlib under {DEPS_DIR} — `cargo test` is expected \
             to build the turingosv4 lib before running this gate. Cannot \
             compile-check pending gates without the rlib; failing LOUD rather \
             than passing vacuously."
        )
    });

    // Build the shared --extern / -L args once.
    let mut extern_args: Vec<String> = vec![
        "--extern".into(),
        format!("turingosv4={}", turingos_rlib.display()),
    ];
    for crate_name in BEST_EFFORT_EXTERNS {
        if let Some(rl) = newest_rlib(crate_name) {
            extern_args.push("--extern".into());
            extern_args.push(format!("{crate_name}={}", rl.display()));
        }
    }

    // Per-gate metadata output dir (NOT /dev/null — rustc creates a temp dir next
    // to the -o path; /dev/null would put it in /dev and fail with EACCES).
    let out_dir = Path::new("target/pending_gate_compile_check");
    fs::create_dir_all(out_dir).expect("create compile-check output dir");

    let files = pending_gate_files();
    assert!(
        !files.is_empty(),
        "pending-gate compile gate is VACUOUS (see pending_gate_set_is_non_empty)"
    );

    let mut broken: Vec<String> = Vec::new();
    for file in &files {
        let stem = file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        let out_meta = out_dir.join(format!("{stem}.rmeta"));

        let mut cmd = Command::new(&rustc);
        cmd.args(["--edition", "2021", "--test", "--emit=metadata"]);
        cmd.args(&extern_args);
        cmd.args(["-L", &format!("dependency={DEPS_DIR}")]);
        cmd.arg("-o").arg(&out_meta);
        cmd.arg(file);

        let output = cmd
            .output()
            .unwrap_or_else(|e| panic!("failed to invoke rustc on {file:?}: {e}"));

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            broken.push(format!(
                "  {}\n    --- rustc stderr ---\n{}",
                file.display(),
                indent(&stderr, "    ")
            ));
        }
    }

    assert!(
        broken.is_empty(),
        "pending-gate COMPILE BIT-ROT: {} gate(s) under {PENDING_DIR}/ no longer \
         compile against the current public `turingosv4` API. A standing-pending \
         gate must always COMPILE (only its assertion is allowed to be RED). Fix \
         the API drift in the gate (or in src/ if a real public symbol moved), \
         then re-run. This is exactly the silent rot the conformance sweep found: \
         the pending gates are excluded from `cargo test`, so without this gate \
         their compile breaks would never surface in CI.\n\n{}",
        broken.len(),
        broken.join("\n\n")
    );
}

/// Resolve a usable `rustc`. Honors $RUSTC (cargo sets it for build scripts /
/// some test contexts) then falls back to PATH. Fails loud if neither runs.
fn which_rustc() -> String {
    let candidate = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let probe = Command::new(&candidate).arg("--version").output();
    match probe {
        Ok(o) if o.status.success() => candidate,
        _ => panic!(
            "rustc is not invokable (tried `{candidate}`). This gate compiles the \
             tests/pending/ gates with rustc; without a working toolchain it \
             cannot do its job. Failing LOUD rather than reporting a vacuous \
             GREEN. Set $RUSTC or ensure rustc is on PATH."
        ),
    }
}

/// Indent every line of `s` by `pad` for readable nested rustc output.
fn indent(s: &str, pad: &str) -> String {
    s.lines()
        .map(|l| format!("{pad}{l}"))
        .collect::<Vec<_>>()
        .join("\n")
}
