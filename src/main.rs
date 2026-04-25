use std::path::PathBuf;

fn main() {
    // Phase B7 (PREREG § 1.8): Boot verifies Trust Root before any other
    // initialization. Tamper with any tracked file => panic with
    // TRUST_ROOT_TAMPERED. The repo root is taken from CARGO_MANIFEST_DIR
    // at compile time so a deployed binary still resolves the genesis
    // path it was built against.
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if let Err(e) = turingosv4::boot::verify_trust_root(&repo_root) {
        panic!("TRUST_ROOT_TAMPERED: {e}");
    }
    println!("TuringOS v4 — Trust Root verified");
}
