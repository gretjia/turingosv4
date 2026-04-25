use std::path::PathBuf;

// TRACE_MATRIX FC3-N29 (`boot`) + FC3-E14 (`error → re-init → boot`):
// constitution FC3 ties `boot` to a re-init loop driven by `error`. Phase B7
// implements the immediate-abort variant of FC3-E14 — Trust Root mismatch
// at Boot panics the process; the surrounding harness (batch runner,
// shell) is the "re-init" layer that decides whether to retry. Future
// in-process re-init (TRACE_MATRIX FC3-N41 row, currently 📅 Phase 11+)
// would replace this panic with a structured retry loop. See
// `handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md` for why
// TRUST_ROOT_TAMPERED is *not* a FC2-N22 HALT path (pre-init failure).
fn main() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
        panic!("TRUST_ROOT_TAMPERED: {e}");
    }
    println!("TuringOS v4 — Trust Root verified");
}
