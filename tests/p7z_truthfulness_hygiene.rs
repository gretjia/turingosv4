use std::fs;
use std::path::Path;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[test]
fn generate_prompt_hash_uses_provider_request_bytes() {
    let cmd_generate = read("src/bin/turingos/cmd_generate.rs");
    let siliconflow = read("src/bin/turingos/chat_client.rs");

    assert!(
        siliconflow.contains("canonical_chat_request_bytes"),
        "siliconflow client must expose canonical request bytes for prompt_hash"
    );
    assert!(
        cmd_generate.contains("canonical_chat_request_bytes"),
        "cmd_generate must hash canonical provider request bytes"
    );
    assert!(
        !cmd_generate.contains("hasher.update(canonical_prompt.as_bytes())"),
        "prompt_hash must not bind only the local pretty prompt string"
    );
}

#[test]
fn generate_raw_output_cid_uses_raw_response_body() {
    let cmd_generate = read("src/bin/turingos/cmd_generate.rs");
    let siliconflow = read("src/bin/turingos/chat_client.rs");

    assert!(
        siliconflow.contains("raw_response_body"),
        "ChatResult must retain raw provider response body bytes"
    );
    assert!(
        cmd_generate.contains("result.raw_response_body.as_slice()"),
        "raw_output_cid must store raw provider response bytes, not parser-view content"
    );
    assert!(
        !cmd_generate.contains("store.put(\n                        result.content.as_bytes()"),
        "raw_output_cid must not store assistant content bytes"
    );
}

#[test]
fn production_rejection_writer_observes_world_head_claim() {
    let cmd_generate = read("src/bin/turingos/cmd_generate.rs");
    let rejection = read("src/runtime/rejection_capsule.rs");

    assert!(
        rejection.contains("write_generate_rejection_capsule_observed"),
        "rejection capsule writer must expose an observed write helper"
    );
    assert!(
        !cmd_generate.contains("world_head_unchanged: true"),
        "cmd_generate production code must not hard-code world_head_unchanged=true"
    );
    assert!(
        cmd_generate.contains("write_generate_rejection_capsule_observed"),
        "cmd_generate must use the observed writer helper"
    );
}

#[test]
fn p7z_language_does_not_overclaim_runtime_sandbox_or_browser_truth() {
    let production_paths = [
        "src/runtime/test_run.rs",
        "src/bin/turingos/cmd_replay.rs",
        "README.md",
    ];

    for path in production_paths {
        let text = read(path);
        for forbidden in [
            "DenyAll",
            "network=DenyAll",
            "no-network guarantee",
            "browser validation",
            "browser-validated",
            "OS-level sandbox",
        ] {
            assert!(
                !text.contains(forbidden),
                "{path} overclaims `{forbidden}` without physical evidence"
            );
        }
    }
}

fn read(rel: &str) -> String {
    let path = Path::new(MANIFEST_DIR).join(rel);
    fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()))
}
