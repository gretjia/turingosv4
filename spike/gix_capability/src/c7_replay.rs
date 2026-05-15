//! C7: Replay — walk commits from genesis to HEAD deterministically

use git2::{Repository, Signature, Sort};
use std::path::Path;

pub struct C7Result {
    pub passed: bool,
    pub walk1_len: usize,
    pub walk2_len: usize,
    pub identical: bool,
    pub error: Option<String>,
}

pub fn run(workdir: &Path) -> C7Result {
    let repo_path = workdir.join("c7_repo");
    let res = (|| -> anyhow::Result<C7Result> {
        let repo = Repository::init(&repo_path)?;
        let sig = Signature::new("spike", "spike@turingos.local", &git2::Time::new(0, 0))?;

        // Build a 10-commit chain
        let mut last_parent: Option<git2::Oid> = None;
        for i in 0..10 {
            let blob = repo.blob(format!("c7 step {i}\n").as_bytes())?;
            let tree_id = {
                let mut tb = repo.treebuilder(None)?;
                tb.insert("file.txt", blob, 0o100644)?;
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
                &format!("C7: step {i}"),
                &tree,
                &parents_refs,
            )?;
            last_parent = Some(commit_id);
        }

        let head = repo.head()?.target().expect("head target");

        // Walk 1
        let mut walk1 = repo.revwalk()?;
        walk1.set_sorting(Sort::TOPOLOGICAL | Sort::TIME)?;
        walk1.push(head)?;
        let walk1_oids: Vec<String> = walk1
            .filter_map(|r| r.ok().map(|oid| oid.to_string()))
            .collect();

        // Walk 2 (independently)
        let mut walk2 = repo.revwalk()?;
        walk2.set_sorting(Sort::TOPOLOGICAL | Sort::TIME)?;
        walk2.push(head)?;
        let walk2_oids: Vec<String> = walk2
            .filter_map(|r| r.ok().map(|oid| oid.to_string()))
            .collect();

        let identical = walk1_oids == walk2_oids && walk1_oids.len() == 10;

        Ok(C7Result {
            passed: identical,
            walk1_len: walk1_oids.len(),
            walk2_len: walk2_oids.len(),
            identical,
            error: None,
        })
    })();

    match res {
        Ok(r) => r,
        Err(e) => C7Result {
            passed: false,
            walk1_len: 0,
            walk2_len: 0,
            identical: false,
            error: Some(format!("{e}")),
        },
    }
}
