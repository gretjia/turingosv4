//! CLI entry: run all 8 capability tests + emit summary.
use gix_capability_spike::*;

fn main() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let workdir = tmp.path();

    println!("# Git Substrate Capability Spike (CO1.3.1) — Result");
    println!();
    println!("Substrate library: **git2-rs 0.20** (libgit2 binding; pivoted from gix per pre-flight § 4)");
    println!("Workdir: {}", workdir.display());
    println!();
    println!("---");
    println!();

    let c1 = c1_init::run(workdir);
    println!("## C1 — Init");
    println!("- pass: {}", c1.passed);
    println!("- repo: {}", c1.repo_path);
    println!(
        "- initial_commit_sha: {}",
        c1.initial_commit_sha.unwrap_or_default()
    );
    if let Some(e) = &c1.error {
        println!("- error: {}", e);
    }
    println!();

    let c2 = c2_multi_parent::run(workdir);
    println!("## C2 — Multi-parent commit");
    println!("- pass: {}", c2.passed);
    println!(
        "- merge_commit_sha: {}",
        c2.merge_commit_sha.unwrap_or_default()
    );
    println!("- parent_count: {}", c2.merge_parent_count);
    if let Some(e) = &c2.error {
        println!("- error: {}", e);
    }
    println!();

    let c3 = c3_tree_parent_read::run(workdir);
    println!("## C3 — Tree + parent read deterministic");
    println!("- pass: {}", c3.passed);
    println!(
        "- commit_sha: {}",
        c3.commit_sha.clone().unwrap_or_default()
    );
    println!("- tree_sha: {}", c3.tree_sha.clone().unwrap_or_default());
    println!("- parent_shas: {:?}", c3.parent_shas);
    println!("- deterministic_two_runs: {}", c3.deterministic_two_runs);
    if let Some(e) = &c3.error {
        println!("- error: {}", e);
    }
    println!();

    let c4 = c4_concurrent_init::run(workdir);
    println!("## C4 — Concurrent init (4 threads)");
    println!("- pass: {}", c4.passed);
    println!("- successes: {} / {}", c4.successes, c4.thread_count);
    println!("- elapsed_ms: {}", c4.elapsed_ms);
    if !c4.errors.is_empty() {
        println!("- thread errors:");
        for err in &c4.errors {
            println!("  - {}", err);
        }
    }
    println!();

    let c5 = c5_cas_blob::run(workdir);
    println!("## C5 — CAS blob round-trip");
    println!("- pass: {}", c5.passed);
    println!("- blob_count: {}", c5.blob_count);
    println!("- round_trip_match: {}", c5.round_trip_match);
    println!(
        "- git_sha1_oid (hello world): {}",
        c5.git_sha1_oid.clone().unwrap_or_default()
    );
    println!(
        "- external_sha256 (hello world): {}",
        c5.external_sha256.clone().unwrap_or_default()
    );
    if let Some(e) = &c5.error {
        println!("- error: {}", e);
    }
    println!();

    let c6 = c6_perf_benchmark::run(workdir);
    println!("## C6 — Perf 100 commits in <1s");
    println!("- pass: {} (target <1000ms)", c6.passed);
    println!("- elapsed_ms: {}", c6.elapsed_ms);
    println!("- commits_per_sec: {:.1}", c6.commits_per_sec);
    if let Some(e) = &c6.error {
        println!("- error: {}", e);
    }
    println!();

    let c7 = c7_replay::run(workdir);
    println!("## C7 — Replay deterministic");
    println!("- pass: {}", c7.passed);
    println!("- walk1 len: {}", c7.walk1_len);
    println!("- walk2 len: {}", c7.walk2_len);
    println!("- identical: {}", c7.identical);
    if let Some(e) = &c7.error {
        println!("- error: {}", e);
    }
    println!();

    let c8 = c8_hooks_compat::run(workdir);
    println!("## C8 — Hook compat (separate repos)");
    println!("- pass: {}", c8.passed);
    println!("- user_repo_has_hook: {}", c8.user_repo_has_hook);
    println!(
        "- runtime_repo_commit_succeeded: {}",
        c8.runtime_repo_commit_succeeded
    );
    println!(
        "- git2_invoked_hook (should be false): {}",
        c8.git2_invoked_hook
    );
    if let Some(e) = &c8.error {
        println!("- error: {}", e);
    }
    println!();

    let total_passed = [
        c1.passed, c2.passed, c3.passed, c4.passed, c5.passed, c6.passed, c7.passed, c8.passed,
    ]
    .iter()
    .filter(|x| **x)
    .count();
    let total = 8;

    println!("---");
    println!();
    println!("# Final Tally: {}/{} PASS", total_passed, total);
    println!();
    if total_passed == total {
        println!("✅ ALL CAPABILITIES PASS — git2-rs is fit for purpose for v4 ChainTape Path B substrate.");
        std::process::exit(0);
    } else {
        println!(
            "⚠️ {} of {} capabilities FAILED — escalate to user.",
            total - total_passed,
            total
        );
        std::process::exit(1);
    }
}
