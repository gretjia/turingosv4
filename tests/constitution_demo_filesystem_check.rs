//! Constitution gate — Task A adversarial test: filesystem properties.
//!
//! Authority: Task A adversarial test (Karpathy v3 plan post-completion harness validation)
//!
//! This gate validates the K-1.5 manifest + K-2.3 drift detection machinery
//! by implementing a trivial but complete gate that exercises end-to-end harness wires.
//!
//! Properties asserted:
//! 1. Cargo.toml exists at repo root
//! 2. Cargo.toml size > 100 bytes
//! 3. Cargo.toml contains the word "turingosv4"

use std::fs;
use std::path::Path;

#[test]
fn cargo_toml_exists() {
    let path = Path::new("Cargo.toml");
    assert!(
        path.exists(),
        "Cargo.toml must exist at repo root; found none at {}",
        path.display()
    );
}

#[test]
fn cargo_toml_size_greater_than_100() {
    let path = Path::new("Cargo.toml");
    let metadata = fs::metadata(path).expect("Cargo.toml must exist and be readable");
    let size = metadata.len();
    assert!(
        size > 100,
        "Cargo.toml size must be > 100 bytes; got {} bytes",
        size
    );
}

#[test]
fn cargo_toml_contains_turingosv4() {
    let path = Path::new("Cargo.toml");
    let contents = fs::read_to_string(path).expect("Cargo.toml must be readable as UTF-8");
    assert!(
        contents.contains("turingosv4"),
        "Cargo.toml must contain the word 'turingosv4'; contents:\n{}",
        contents
    );
}
