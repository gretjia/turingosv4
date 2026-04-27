//! C5: Create CAS blob + retrieve by SHA (round-trip identity)

use git2::Repository;
use sha2::{Digest, Sha256};
use std::path::Path;

pub struct C5Result {
    pub passed: bool,
    pub blob_count: usize,
    pub round_trip_match: bool,
    pub git_sha1_oid: Option<String>,
    pub external_sha256: Option<String>,
    pub error: Option<String>,
}

pub fn run(workdir: &Path) -> C5Result {
    let repo_path = workdir.join("c5_repo");
    let res = (|| -> anyhow::Result<C5Result> {
        let repo = Repository::init(&repo_path)?;

        let payloads: Vec<Vec<u8>> = vec![
            b"".to_vec(),
            b"a".to_vec(),
            b"hello world".to_vec(),
            vec![0u8; 1024],
            vec![0xffu8; 65536],
        ];

        let mut oids = vec![];
        for p in &payloads {
            oids.push(repo.blob(p)?);
        }

        let mut all_match = true;
        for (oid, expected) in oids.iter().zip(payloads.iter()) {
            let blob = repo.find_blob(*oid)?;
            if blob.content() != expected.as_slice() {
                all_match = false;
                break;
            }
        }

        let mut h = Sha256::new();
        h.update(&payloads[2]);
        let sha256 = format!("{:x}", h.finalize());

        Ok(C5Result {
            passed: all_match && oids.len() == payloads.len(),
            blob_count: oids.len(),
            round_trip_match: all_match,
            git_sha1_oid: Some(oids[2].to_string()),
            external_sha256: Some(sha256),
            error: None,
        })
    })();

    match res {
        Ok(r) => r,
        Err(e) => C5Result {
            passed: false,
            blob_count: 0,
            round_trip_match: false,
            git_sha1_oid: None,
            external_sha256: None,
            error: Some(format!("{e}")),
        },
    }
}
