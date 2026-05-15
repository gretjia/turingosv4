//! C4: Concurrent runtime_repo init from multiple OS threads (disjoint paths)

use git2::{Repository, Signature};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct C4Result {
    pub passed: bool,
    pub thread_count: usize,
    pub successes: usize,
    pub failures: usize,
    pub errors: Vec<String>,
    pub elapsed_ms: u128,
}

pub fn run(workdir: &Path) -> C4Result {
    let start = std::time::Instant::now();
    let thread_count = 4;
    let workdir = Arc::new(workdir.to_path_buf());
    let mut handles = vec![];
    let errors = Arc::new(Mutex::new(Vec::<String>::new()));
    let succ = Arc::new(Mutex::new(0usize));

    for i in 0..thread_count {
        let wd = workdir.clone();
        let errs = errors.clone();
        let s = succ.clone();
        handles.push(thread::spawn(move || {
            let path = wd.join(format!("c4_repo_thread_{i}"));
            let res: anyhow::Result<()> = (|| {
                let repo = Repository::init(&path)?;
                let sig = Signature::new(
                    &format!("spike_{i}"),
                    "spike@turingos.local",
                    &git2::Time::new(0, 0),
                )?;
                let blob = repo.blob(format!("c4 thread {i}\n").as_bytes())?;
                let tree_id = {
                    let mut tb = repo.treebuilder(None)?;
                    tb.insert("c4.txt", blob, 0o100644)?;
                    tb.write()?
                };
                let tree = repo.find_tree(tree_id)?;
                let _ = repo.commit(
                    Some("HEAD"),
                    &sig,
                    &sig,
                    &format!("C4: thread {i}"),
                    &tree,
                    &[],
                )?;
                Ok(())
            })();
            match res {
                Ok(_) => *s.lock().unwrap() += 1,
                Err(e) => errs.lock().unwrap().push(format!("thread {i}: {e}")),
            }
        }));
    }
    for h in handles {
        let _ = h.join();
    }

    let elapsed_ms = start.elapsed().as_millis();
    let successes = *succ.lock().unwrap();
    let errs = errors.lock().unwrap().clone();
    let failures = thread_count - successes;

    C4Result {
        passed: successes == thread_count,
        thread_count,
        successes,
        failures,
        errors: errs,
        elapsed_ms,
    }
}
