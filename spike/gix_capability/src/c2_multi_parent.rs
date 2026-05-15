//! C2: Multi-parent commit support (Contribution DAG citation pattern)

use git2::{Repository, Signature};
use std::path::Path;

pub struct C2Result {
    pub passed: bool,
    pub merge_commit_sha: Option<String>,
    pub merge_parent_count: usize,
    pub error: Option<String>,
}

pub fn run(workdir: &Path) -> C2Result {
    let repo_path = workdir.join("c2_repo");
    let res = (|| -> anyhow::Result<C2Result> {
        let repo = Repository::init(&repo_path)?;
        let sig = Signature::new("spike", "spike@turingos.local", &git2::Time::new(0, 0))?;

        // Commit A (on branch_a)
        let blob_a = repo.blob(b"a\n")?;
        let tree_a = {
            let mut tb = repo.treebuilder(None)?;
            tb.insert("a.txt", blob_a, 0o100644)?;
            repo.find_tree(tb.write()?)?
        };
        let commit_a_id = repo.commit(
            Some("refs/heads/branch_a"),
            &sig,
            &sig,
            "C2: commit A",
            &tree_a,
            &[],
        )?;
        let commit_a = repo.find_commit(commit_a_id)?;

        // Commit B (on branch_b)
        let blob_b = repo.blob(b"b\n")?;
        let tree_b = {
            let mut tb = repo.treebuilder(None)?;
            tb.insert("b.txt", blob_b, 0o100644)?;
            repo.find_tree(tb.write()?)?
        };
        let commit_b_id = repo.commit(
            Some("refs/heads/branch_b"),
            &sig,
            &sig,
            "C2: commit B",
            &tree_b,
            &[],
        )?;
        let commit_b = repo.find_commit(commit_b_id)?;

        // Commit C: tree = a.txt + b.txt; parents = [A, B]
        let tree_c = {
            let mut tb = repo.treebuilder(None)?;
            tb.insert("a.txt", blob_a, 0o100644)?;
            tb.insert("b.txt", blob_b, 0o100644)?;
            repo.find_tree(tb.write()?)?
        };
        let commit_c_id = repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            "C2: merge commit C",
            &tree_c,
            &[&commit_a, &commit_b],
        )?;

        let commit_c = repo.find_commit(commit_c_id)?;
        let parent_count = commit_c.parent_count();

        Ok(C2Result {
            passed: parent_count == 2,
            merge_commit_sha: Some(commit_c_id.to_string()),
            merge_parent_count: parent_count,
            error: None,
        })
    })();

    match res {
        Ok(r) => r,
        Err(e) => C2Result {
            passed: false,
            merge_commit_sha: None,
            merge_parent_count: 0,
            error: Some(format!("{e}")),
        },
    }
}
