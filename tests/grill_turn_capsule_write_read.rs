//! W5 turn-capsule tests are inline in src/bin/turingos/spec_capsule.rs
//! #[cfg(test)] mod grill_capsule_tests, because the writers are
//! `pub(crate)` and not visible from this integration-test crate.
//!
//! Run them via:
//!   cargo test --bin turingos grill_capsule_tests::

#[test]
fn pointer_to_inline_tests() {
    // No-op: real tests are inline. This file exists per charter §4
    // Allowed Paths.
}
