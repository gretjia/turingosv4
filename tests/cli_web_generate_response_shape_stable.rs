//! Verification of GenerateResponse and ArtifactEntry JSON shape stability.
//! Asserts that when new optional fields are None, they are omitted from the
//! serialized JSON output, preserving exact backward compatibility.
#![cfg(feature = "web")]

#[path = "../src/web/mod.rs"]
mod web;

use web::generate::{ArtifactEntry, GenerateResponse};

#[test]
fn test_generate_response_json_shape_stable_when_none() {
    let resp = GenerateResponse {
        session_id: "test-session-123".to_string(),
        artifacts: vec![
            ArtifactEntry {
                path: "index.html".to_string(),
                size_bytes: 1024,
                content_type: "text/html",
                cid: None,
                sha256: None,
            }
        ],
        transcript_excerpt: Some("Excerpt text".to_string()),
        total_attempts: 2,
        status: None,
        artifact_bundle_cid: None,
    };

    let serialized = serde_json::to_string(&resp).expect("serialize");
    let val: serde_json::Value = serde_json::from_str(&serialized).expect("deserialize");

    // Existing fields must be present
    assert_eq!(val["session_id"].as_str(), Some("test-session-123"));
    assert_eq!(val["transcript_excerpt"].as_str(), Some("Excerpt text"));
    assert_eq!(val["total_attempts"].as_u64(), Some(2));
    
    let artifacts = val["artifacts"].as_array().expect("artifacts array");
    assert_eq!(artifacts.len(), 1);
    let art = &artifacts[0];
    assert_eq!(art["path"].as_str(), Some("index.html"));
    assert_eq!(art["size_bytes"].as_u64(), Some(1024));
    assert_eq!(art["content_type"].as_str(), Some("text/html"));

    // Optional fields must be skipped when None
    assert!(val.get("status").is_none());
    assert!(val.get("artifact_bundle_cid").is_none());
    assert!(art.get("cid").is_none());
    assert!(art.get("sha256").is_none());
}

#[test]
fn test_generate_response_json_shape_when_some() {
    let resp = GenerateResponse {
        session_id: "test-session-123".to_string(),
        artifacts: vec![
            ArtifactEntry {
                path: "index.html".to_string(),
                size_bytes: 1024,
                content_type: "text/html",
                cid: Some("file_cid_123".to_string()),
                sha256: Some("file_sha_123".to_string()),
            }
        ],
        transcript_excerpt: Some("Excerpt text".to_string()),
        total_attempts: 2,
        status: Some("success".to_string()),
        artifact_bundle_cid: Some("bundle_cid_123".to_string()),
    };

    let serialized = serde_json::to_string(&resp).expect("serialize");
    let val: serde_json::Value = serde_json::from_str(&serialized).expect("deserialize");

    // All fields must be present
    assert_eq!(val["session_id"].as_str(), Some("test-session-123"));
    assert_eq!(val["status"].as_str(), Some("success"));
    assert_eq!(val["artifact_bundle_cid"].as_str(), Some("bundle_cid_123"));
    
    let artifacts = val["artifacts"].as_array().expect("artifacts array");
    assert_eq!(artifacts.len(), 1);
    let art = &artifacts[0];
    assert_eq!(art["path"].as_str(), Some("index.html"));
    assert_eq!(art["cid"].as_str(), Some("file_cid_123"));
    assert_eq!(art["sha256"].as_str(), Some("file_sha_123"));
}
