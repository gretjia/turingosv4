//! C1: Initialize a git repo programmatically (no shell)

use git2::{Repository, Signature};
use std::path::Path;

pub struct C1Result {
    pub passed: bool,
    pub repo_path: String,
    pub initial_commit_sha: Option<String>,
    pub error: Option<String>,
}

pub fn run(workdir: &Path) -> C1Result {
    let repo_path = workdir.join("c1_repo");
    let res = (|| -> anyhow::Result<C1Result> {
        let repo = Repository::init(&repo_path)?;

        // Build first commit
        let sig = Signature::new("spike", "spike@turingos.local", &git2::Time::new(0, 0))?;

        let mut index = repo.index()?;
        let blob_oid = repo.blob(b"hello-c1\n")?;
        let tree_id = {
            let mut tree_builder = repo.treebuilder(None)?;
            tree_builder.insert("hello.txt", blob_oid, 0o100644)?;
            tree_builder.write()?
        };
        let tree = repo.find_tree(tree_id)?;

        let commit_oid = repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            "C1: initial commit",
            &tree,
            &[],
        )?;

        // Cleanup index handle
        drop(index);

        Ok(C1Result {
            passed: true,
            repo_path: repo_path.display().to_string(),
            initial_commit_sha: Some(commit_oid.to_string()),
            error: None,
        })
    })();

    match res {
        Ok(r) => r,
        Err(e) => C1Result {
            passed: false,
            repo_path: repo_path.display().to_string(),
            initial_commit_sha: None,
            error: Some(format!("{e}")),
        },
    }
}
