//! C6: Performance — 100 commits in < 1 second

use git2::{Repository, Signature};
use std::path::Path;
use std::time::Instant;

pub struct C6Result {
    pub passed: bool,
    pub commit_count: usize,
    pub elapsed_ms: u128,
    pub commits_per_sec: f64,
    pub error: Option<String>,
}

pub fn run(workdir: &Path) -> C6Result {
    let repo_path = workdir.join("c6_repo");
    let target_count = 100;
    let res = (|| -> anyhow::Result<C6Result> {
        let repo = Repository::init(&repo_path)?;
        let sig = Signature::new("spike", "spike@turingos.local", &git2::Time::new(0, 0))?;

        let start = Instant::now();
        let mut last_parent: Option<git2::Oid> = None;
        for i in 0..target_count {
            let blob = repo.blob(format!("commit {i}\n").as_bytes())?;
            let tree_id = {
                let mut tb = repo.treebuilder(None)?;
                tb.insert("counter.txt", blob, 0o100644)?;
                tb.write()?
            };
            let tree = repo.find_tree(tree_id)?;
            let parents: Vec<git2::Commit> = match last_parent {
                Some(oid) => vec![repo.find_commit(oid)?],
                None => vec![],
            };
            let parents_refs: Vec<&git2::Commit> = parents.iter().collect();
            let commit_id = repo.commit(
                Some("HEAD"),
                &sig,
                &sig,
                &format!("C6: commit {i}"),
                &tree,
                &parents_refs,
            )?;
            last_parent = Some(commit_id);
        }
        let elapsed = start.elapsed();

        let elapsed_ms = elapsed.as_millis();
        let commits_per_sec = if elapsed.as_secs_f64() > 0.0 {
            target_count as f64 / elapsed.as_secs_f64()
        } else {
            f64::INFINITY
        };

        Ok(C6Result {
            passed: elapsed_ms < 1000,
            commit_count: target_count,
            elapsed_ms,
            commits_per_sec,
            error: None,
        })
    })();

    match res {
        Ok(r) => r,
        Err(e) => C6Result {
            passed: false,
            commit_count: 0,
            elapsed_ms: 0,
            commits_per_sec: 0.0,
            error: Some(format!("{e}")),
        },
    }
}
