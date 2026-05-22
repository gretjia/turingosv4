use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::generation_attempt::GenerationAttemptCapsule;

const BLACKBOX_SYSTEM_PROMPT: &str = r#"You are TuringOS Blackbox AI, a fast code-generation assistant.

Input: a spec.md describing what a non-developer user wants built.
Output: one or more complete, working source files.

**OUTPUT FORMAT — STRICT**:
For each file, output on its own line:
```
### File: <relative path>
```
Then a fenced code block with the file content. The fence opener must include
the language tag (e.g. ```html, ```python, ```javascript, ```css).

**RULES**:
1. Prefer ONE single self-contained file when possible. For a UI app, output
   ONE `index.html` with `<style>` and `<script>` embedded — so the user can
   open the file in a browser with zero install. For a script, output ONE
   Python 3 file named `main.py`.
2. No external runtime dependencies unless the spec explicitly demands them
   (no `npm install`, no `pip install`, no CDN scripts unless unavoidable).
3. The code must actually run as-emitted. If the spec is vague, choose a
   sensible default and add a brief comment marking the assumption.
4. NO surrounding prose. No "Here's the code:" preamble. No closing remarks.
   First line of your response is `### File: ...`. Last line is the closing
   ``` of the final code block.
5. Keep files focused. Do not add tests, README.md, package.json, or build
   configs unless the spec asks for them.
6. Honor the spec's "Out of Scope" / "Deliberately NOT Doing" section —
   do NOT add features it forbids.
7. VISUAL FORMAT for HTML outputs (TuringOS aesthetic — applies when your
   output is `index.html`). Apply these design tokens as inline CSS — do
   NOT pull in Tailwind CDN, Bootstrap CDN, or any other framework:
   - Headings: font-family 'Fraunces', Georgia, serif (load via Google
     Fonts <link> in <head> is OK: family=Fraunces:opsz,wght@9..144,400;9..144,600).
   - Body: font-family 'IBM Plex Sans', system-ui, sans-serif (Google Fonts OK).
   - Code/mono: font-family 'JetBrains Mono', ui-monospace, monospace (Google Fonts OK).
   - Accent color: define `--accent: #4e8b7a` (oxidized teal). Use for links,
     buttons, borders, focus rings, key highlights.
   - Background: `#f8f6f1` (warm off-white). Text: `#1a1a1a`. Muted: `#6b6b6b`.
   - Layout: comfortable padding, generous line-height (≥1.55 body),
     H1 Fraunces 36–48px, H2 Fraunces 24–28px, body 16–17px.
   - Do NOT use Inter, Roboto, Arial, or any purple-gradient styling.
   - Prefer prefers-color-scheme: dark for an additional dark variant
     (background #1a1a1a, text #f0eee8, accent same teal but slightly lighter).
   - If the spec does NOT target a UI/HTML app (e.g., a Python script), skip
     this rule entirely.

Example shape (DO NOT COPY VERBATIM — write your own per the spec):
### File: index.html
```html
<!DOCTYPE html>
<html>...</html>
```
"#;

#[derive(Serialize)]
struct Request<'a> {
    model: &'a str,
    messages: &'a [Message],
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

fn turingos_bin() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let candidates = [
        format!("{manifest_dir}/target/debug/turingos"),
        format!("{manifest_dir}/target/release/turingos"),
    ];
    for candidate in candidates.iter() {
        let path = PathBuf::from(candidate);
        if path.exists() {
            return path;
        }
    }
    panic!("turingos binary not found");
}

fn start_mock_llm_server(response_body: String) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut buf = [0; 4096];
            let _ = stream.read(&mut buf);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                response_body.len(),
                response_body
            );
            let _ = stream.write_all(response.as_bytes());
            let _ = stream.flush();
        }
    });
    format!("http://127.0.0.1:{}", port)
}

#[test]
fn test_generate_attempt_prompt_hash_is_canonical() {
    let tmp = tempfile::tempdir().expect("create temp workspace");
    let ws = tmp.path().join("my_workspace");

    // Init workspace
    let status = Command::new(turingos_bin())
        .arg("init")
        .arg("--project")
        .arg(&ws)
        .status()
        .expect("run init");
    assert!(status.success());

    // Write spec.md
    let spec_content = "# Test Spec\nGenerate some code.";
    let spec_path = ws.join("spec.md");
    fs::write(&spec_path, spec_content).expect("write spec.md");

    let raw_response = "{\n  \"choices\": [\n    {\n      \"message\": {\n        \"role\": \"assistant\",\n        \"content\": \"### File: index.html\\n```html\\n<!doctype html>\\n<html><body><main>ok</main></body></html>\\n```\"\n      },\n      \"finish_reason\": \"stop\"\n    }\n  ],\n  \"usage\": {\n    \"prompt_tokens\": 10,\n    \"completion_tokens\": 20,\n    \"total_tokens\": 30\n  }\n}".to_string();

    let endpoint = start_mock_llm_server(raw_response);

    // Run generate command
    let output = Command::new(turingos_bin())
        .arg("generate")
        .arg("--workspace")
        .arg(&ws)
        .env("TURINGOS_SILICONFLOW_ENDPOINT", &endpoint)
        .env("SILICONFLOW_API_KEY", "mock-key")
        .output()
        .expect("run generate");

    assert!(output.status.success());

    // Verify GenerationAttemptCapsule is recorded in CAS
    let cas_dir = ws.join("cas");
    let store = CasStore::open(&cas_dir).expect("open cas store");
    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);

    let mut attempt_capsule: Option<GenerationAttemptCapsule> = None;
    for cid in cids {
        if let Some(meta) = store.metadata(&cid) {
            if meta.schema_id.as_deref() == Some("turingos-generation-attempt-v1") {
                let bytes = store.get(&cid).expect("read capsule");
                let cap: GenerationAttemptCapsule =
                    serde_json::from_slice(&bytes).expect("deserialize");
                attempt_capsule = Some(cap);
                break;
            }
        }
    }

    let cap = attempt_capsule.expect("GenerationAttemptCapsule not found in CAS");

    // Compute provider request hash locally. This must match the exact
    // canonical request bytes sent to the OpenAI-compatible endpoint.
    let messages = vec![
        Message {
            role: "system".to_string(),
            content: BLACKBOX_SYSTEM_PROMPT.to_string(),
        },
        Message {
            role: "user".to_string(),
            content: format!(
                "Below is the spec. Generate the working code per the rules.\n\nspec source: {}\n\n{}",
                spec_path.display(),
                spec_content
            ),
        },
    ];
    let request = Request {
        model: "Qwen/Qwen3-Coder-30B-A3B-Instruct",
        messages: &messages,
        max_tokens: Some(6000),
        temperature: Some(0.2),
    };
    let canonical_request_bytes = serde_json::to_vec(&request).expect("serialize request");
    let mut hasher = Sha256::new();
    hasher.update(&canonical_request_bytes);
    let expected_hash = format!("{:x}", hasher.finalize());

    assert_eq!(cap.prompt_hash, expected_hash);
}
