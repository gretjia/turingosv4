//! C3: Read commit object's tree root + parent SHAs deterministically

use git2::{Repository, Signature};
use std::path::Path;

pub struct C3Result {
    pub passed: bool,
    pub commit_sha: Option<String>,
    pub tree_sha: Option<String>,
    pub parent_shas: Vec<String>,
    pub deterministic_two_runs: bool,
    pub error: Option<String>,
}

pub fn run(workdir: &Path) -> C3Result {
    let repo_path = workdir.join("c3_repo");
    let res = (|| -> anyhow::Result<C3Result> {
        let repo = Repository::init(&repo_path)?;
        let sig = Signature::new("spike", "spike@turingos.local", &git2::Time::new(0, 0))?;

        let blob = repo.blob(b"c3 content\n")?;
        let tree_id = {
            let mut tb = repo.treebuilder(None)?;
            tb.insert("c3.txt", blob, 0o100644)?;
            tb.write()?
        };
        let tree = repo.find_tree(tree_id)?;
        let commit_id = repo.commit(Some("HEAD"), &sig, &sig, "C3: read test", &tree, &[])?;

        // Read 1
        let commit_obj = repo.find_commit(commit_id)?;
        let tree_id_1 = commit_obj.tree_id();
        let parents: Vec<String> = (0..commit_obj.parent_count())
            .map(|i| commit_obj.parent_id(i).map(|oid| oid.to_string()).unwrap_or_default())
            .collect();

        // Read 2: re-find commit; SHA must match
        let commit_obj_2 = repo.find_commit(commit_id)?;
        let tree_id_2 = commit_obj_2.tree_id();

        let det = tree_id_1 == tree_id_2 && tree_id_1.to_string() == tree_id.to_string();

        Ok(C3Result {
            passed: det,
            commit_sha: Some(commit_id.to_string()),
            tree_sha: Some(tree_id_1.to_string()),
            parent_shas: parents,
            deterministic_two_runs: det,
            error: None,
        })
    })();

    match res {
        Ok(r) => r,
        Err(e) => C3Result {
            passed: false,
            commit_sha: None,
            tree_sha: None,
            parent_shas: vec![],
            deterministic_two_runs: false,
            error: Some(format!("{e}")),
        },
    }
}
