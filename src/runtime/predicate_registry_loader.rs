//! Shared predicate registry loader for replay/bootstrap call sites.
//!
//! W3-2 keeps pre-activation legacy replay available, but production replay
//! paths must not each manufacture their own ad hoc empty registry.

use crate::top_white::predicates::registry::{BootPredicateManifest, PredicateRegistry};

/// TRACE_MATRIX FC1-N11 + FC1-N12 + FC2-N19: construct the boot executable predicate registry for replay and resume call sites.
pub fn load_replay_registry() -> PredicateRegistry {
    PredicateRegistry::from_boot_manifest(BootPredicateManifest::v8_production())
        .expect("v8 production predicate manifest must be valid")
}
