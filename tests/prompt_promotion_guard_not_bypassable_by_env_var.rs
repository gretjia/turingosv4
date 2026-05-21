//! C10 gate: guard not bypassable by TURINGOS_BYPASS_PROMOTION_GUARD env var.
//!
//! FC-trace: FC2 (prompt boot guard), FC3 (eval evidence binding)
//! Risk class: Class 3

use turingosv4::runtime::prompt_promotion::{check_promotion_guard, sha256_hex_of_prompt};
use turingosv4::runtime::spec_capsule::cas_path;

#[test]
fn test_guard_not_bypassable_by_env_var() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let ws = dir.path();

    // Create CAS dir so the guard can open it (not a NoCasStore error)
    let cas = cas_path(ws);
    std::fs::create_dir_all(&cas).expect("create cas");
    let _ = turingosv4::bottom_white::cas::store::CasStore::open(&cas).expect("open cas");

    let to_cid = sha256_hex_of_prompt(b"v2 prompt bytes");

    // Set bypass env var
    std::env::set_var("TURINGOS_BYPASS_PROMOTION_GUARD", "1");

    // Guard must still reject (no receipt written)
    let result = check_promotion_guard(ws, &to_cid);
    std::env::remove_var("TURINGOS_BYPASS_PROMOTION_GUARD");

    assert!(
        result.is_err(),
        "guard must remain active even with TURINGOS_BYPASS_PROMOTION_GUARD=1, got Ok"
    );
    let msg = format!("{}", result.unwrap_err());
    assert!(
        msg.contains("no PromptPromotionReceipt") || msg.contains("CAS"),
        "expected guard rejection message: {msg}"
    );
}

#[test]
fn test_guard_not_bypassable_by_empty_bypass_var() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let ws = dir.path();

    let cas = cas_path(ws);
    std::fs::create_dir_all(&cas).expect("create cas");
    let _ = turingosv4::bottom_white::cas::store::CasStore::open(&cas).expect("open cas");

    let to_cid = sha256_hex_of_prompt(b"v2 prompt bytes alt");

    std::env::set_var("TURINGOS_BYPASS_PROMOTION_GUARD", "");
    let result = check_promotion_guard(ws, &to_cid);
    std::env::remove_var("TURINGOS_BYPASS_PROMOTION_GUARD");

    assert!(result.is_err(), "guard must remain active with empty bypass var");
}

#[test]
fn test_guard_not_bypassable_by_true_bypass_var() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let ws = dir.path();

    let cas = cas_path(ws);
    std::fs::create_dir_all(&cas).expect("create cas");
    let _ = turingosv4::bottom_white::cas::store::CasStore::open(&cas).expect("open cas");

    let to_cid = sha256_hex_of_prompt(b"v2 prompt bytes true");

    std::env::set_var("TURINGOS_BYPASS_PROMOTION_GUARD", "true");
    let result = check_promotion_guard(ws, &to_cid);
    std::env::remove_var("TURINGOS_BYPASS_PROMOTION_GUARD");

    assert!(result.is_err(), "guard must remain active with bypass var=true");
}
