//! C8: Hooks compat — pre-commit hook in user-facing repo does NOT interfere with
//! per-cell runtime_repo (separate filesystem trees + .git dirs).

use git2::{Repository, Signature};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub struct C8Result {
    pub passed: bool,
    pub user_repo_has_hook: bool,
    pub runtime_repo_commit_succeeded: bool,
    pub git2_invoked_hook: bool,
    pub error: Option<String>,
}

pub fn run(workdir: &Path) -> C8Result {
    let res = (|| -> anyhow::Result<C8Result> {
        // Setup: user-facing repo with always-failing pre-commit hook
        let user_repo_path = workdir.join("c8_user_repo");
        let _user = Repository::init(&user_repo_path)?;
        let hook_path = user_repo_path.join(".git").join("hooks").join("pre-commit");
        if let Some(parent) = hook_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let marker = workdir.join("c8_hook_invoked.marker");
        let marker_str = marker.display().to_string();
        // hook touches marker so we know if invoked
        fs::write(
            &hook_path,
            format!("#!/bin/sh\ntouch '{marker_str}'\necho 'hook would block this' >&2\nexit 1\n"),
        )?;
        fs::set_permissions(&hook_path, fs::Permissions::from_mode(0o755))?;

        // Separate runtime_repo
        let runtime_repo_path = workdir.join("c8_runtime_repo");
        let rt = Repository::init(&runtime_repo_path)?;
        let sig = Signature::new("spike", "spike@turingos.local", &git2::Time::new(0, 0))?;

        let blob = rt.blob(b"c8 runtime commit\n")?;
        let tree_id = {
            let mut tb = rt.treebuilder(None)?;
            tb.insert("rt.txt", blob, 0o100644)?;
            tb.write()?
        };
        let tree = rt.find_tree(tree_id)?;
        let commit_result = rt.commit(Some("HEAD"), &sig, &sig, "C8: runtime commit", &tree, &[]);

        let runtime_commit_succeeded = commit_result.is_ok();
        let git2_invoked_hook = marker.exists();

        Ok(C8Result {
            passed: runtime_commit_succeeded && !git2_invoked_hook,
            user_repo_has_hook: hook_path.exists(),
            runtime_repo_commit_succeeded: runtime_commit_succeeded,
            git2_invoked_hook,
            error: None,
        })
    })();

    match res {
        Ok(r) => r,
        Err(e) => C8Result {
            passed: false,
            user_repo_has_hook: false,
            runtime_repo_commit_succeeded: false,
            git2_invoked_hook: false,
            error: Some(format!("{e}")),
        },
    }
}
