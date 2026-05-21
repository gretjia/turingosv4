//! Verification that SandboxPolicy enum is serialized and deserialized
//! to its lowercase string representation.
#![cfg(feature = "web")]

use turingosv4::runtime::preview_run::SandboxPolicy;

#[test]
fn test_sandbox_policy_serialization_is_lowercase() {
    let p1 = SandboxPolicy::AllowScripts;
    let p2 = SandboxPolicy::AllowScriptsAllowSameOrigin;

    let s1 = serde_json::to_string(&p1).expect("serialize p1");
    let s2 = serde_json::to_string(&p2).expect("serialize p2");

    assert_eq!(s1, "\"allowscripts\"");
    assert_eq!(s2, "\"allowscriptsallowsameorigin\"");

    let d1: SandboxPolicy = serde_json::from_str(&s1).expect("deserialize s1");
    let d2: SandboxPolicy = serde_json::from_str(&s2).expect("deserialize s2");

    assert_eq!(d1, SandboxPolicy::AllowScripts);
    assert_eq!(d2, SandboxPolicy::AllowScriptsAllowSameOrigin);
}
