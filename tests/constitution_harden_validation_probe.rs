//! Probe gate added by K-HARDEN validation run.
use std::path::Path;

#[test]
fn cargo_lock_exists() {
    assert!(Path::new("Cargo.lock").exists());
}

#[test]
fn readme_exists() {
    assert!(Path::new("README.md").exists());
}
