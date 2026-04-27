#[test]
fn sign_api_is_crate_only_and_scoped_to_authorized_modules() {
    let src = include_str!("../src/bottom_white/ledger/system_keypair.rs");

    assert!(
        src.contains("pub(crate) mod predicate_runner"),
        "predicate_runner signing scope must exist"
    );
    assert!(
        src.contains("pub(crate) mod terminal_summary_emitter"),
        "terminal_summary_emitter signing scope must exist"
    );
    assert!(
        !src.contains("pub fn sign_system_message"),
        "root public sign_system_message must not be exported"
    );
    assert!(
        !src.contains("pub use predicate_runner")
            && !src.contains("pub use terminal_summary_emitter")
            && !src.contains("pub(crate) use predicate_runner")
            && !src.contains("pub(crate) use terminal_summary_emitter"),
        "authorized signing scopes must not be re-exported"
    );
    assert!(
        src.contains("CanonicalMessage"),
        "signing must remain typed through CanonicalMessage"
    );
    assert!(
        !src.contains("&[u8]") || src.contains("canonical_digest(message)"),
        "no byte-slice free-form signing API should be present"
    );
}
