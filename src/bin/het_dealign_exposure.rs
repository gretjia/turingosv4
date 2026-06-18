//! Q2 (OBL-018): de-alignment false-negative scanner + flip-rate verifier.
//!
//! Two modes (both use the lib `realign`/`dedent`/`LeanJudge` directly → NO logic drift):
//!
//!   het_dealign_exposure <bodies.jsonl>
//!       Cheap EXPOSURE scan: count bodies where `realign(body) != dedent(body)` — proofs
//!       the 门0 fix would assemble differently from the historical conservative path
//!       (potential cured false negatives). No Lean.
//!
//!   het_dealign_exposure verify <verify_set.jsonl> <mathlib_dir> [max_samples]
//!       FLIP-RATE: for each exposed body ({"preamble","body"}), run real Lean on BOTH
//!       dedent(body) (historical) and realign(body) (fixed). A Failed→Verified flip
//!       confirms a real de-align false negative the fix cures. Needs the pinned toolchain
//!       + mathlib; bodies must be COMPLETE (untruncated) to compile.
//!
//! Honest bound: a single-line body has no sibling to misalign, so exposure is only ever
//! flagged for MULTI-LINE bodies; a preview truncated within line 1 reads as non-exposed.
//! Over truncated manifest previews the scan count is therefore a LOWER bound.

use std::io::BufRead;
use std::path::PathBuf;
use std::time::Duration;

use turingosv4::judges::lean_judge::{dedent, default_lean_bin, realign, LeanJudge};
use turingosv4::judges::lean_theorem_bank::{default_lake_bin, mathlib_lean_path};

fn read_jsonl(path: &str) -> Vec<serde_json::Value> {
    let f = std::fs::File::open(path).expect("open jsonl");
    std::io::BufReader::new(f)
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(&l).expect("parse jsonl line"))
        .collect()
}

fn scan(path: &str) {
    let mut total = 0usize;
    let mut multiline = 0usize;
    let mut exposed = 0usize;
    let mut exposed_multiline = 0usize;
    let mut exposed_ids: Vec<String> = Vec::new();
    for v in read_jsonl(path) {
        let body = v.get("body").and_then(|b| b.as_str()).unwrap_or("");
        if body.trim().is_empty() {
            continue;
        }
        total += 1;
        let is_multiline = body.contains('\n');
        if is_multiline {
            multiline += 1;
        }
        if realign(body) != dedent(body) {
            exposed += 1;
            if is_multiline {
                exposed_multiline += 1;
            }
            if let Some(id) = v.get("id").and_then(|x| x.as_str()) {
                exposed_ids.push(id.to_string());
            }
        }
    }
    let summary = serde_json::json!({
        "total_failed_bodies": total,
        "multiline_previews": multiline,
        "dealign_exposed": exposed,
        "dealign_exposed_multiline": exposed_multiline,
        "exposure_rate_over_total": if total > 0 { exposed as f64 / total as f64 } else { 0.0 },
        "exposure_rate_over_multiline": if multiline > 0 { exposed_multiline as f64 / multiline as f64 } else { 0.0 },
        "note": "single-line/truncated previews read as non-exposed → this is a LOWER bound",
        "exposed_ids_sample": exposed_ids.iter().take(25).collect::<Vec<_>>(),
    });
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
}

fn verify(path: &str, mathlib_dir: &str, max_samples: Option<usize>) {
    let bin = default_lean_bin();
    if !(bin.is_absolute() && bin.exists()) {
        eprintln!("ABORT: pinned Lean toolchain absent");
        std::process::exit(2);
    }
    let lp = mathlib_lean_path(PathBuf::from(mathlib_dir), &default_lake_bin());
    if lp.is_none() {
        eprintln!("ABORT: could not resolve Mathlib LEAN_PATH from {mathlib_dir}");
        std::process::exit(2);
    }
    let lp = lp.unwrap();

    let rows = read_jsonl(path);
    let mut checked = 0usize;
    let mut exposed = 0usize;
    let mut flipped = 0usize; // dedent FAILS, realign VERIFIES → cured false negative
    let mut both_fail = 0usize;
    let mut dedent_already_passed = 0usize; // exposed but historical already verified (rare)
    let mut flip_examples: Vec<String> = Vec::new();
    let mut fail_feedback: Vec<serde_json::Value> = Vec::new();

    for v in rows {
        if let Some(m) = max_samples {
            if checked >= m {
                break;
            }
        }
        let body = v.get("body").and_then(|b| b.as_str()).unwrap_or("");
        let preamble = v.get("preamble").and_then(|b| b.as_str()).unwrap_or("");
        if body.trim().is_empty() || preamble.trim().is_empty() {
            continue;
        }
        let d = dedent(body);
        let r = realign(body);
        if r == d {
            continue; // not exposed
        }
        exposed += 1;
        checked += 1;

        let mk = || {
            let mut j = LeanJudge::new(preamble.to_string());
            j.lean_bin = bin.clone();
            j.cwd = PathBuf::from(mathlib_dir);
            j.timeout = Duration::from_secs(120);
            j.extra_env.push(("LEAN_PATH".to_string(), lp.clone()));
            j
        };
        let od = mk().verify(&d);
        let orr = mk().verify(&r);
        match (od.is_verified(), orr.is_verified()) {
            (false, true) => {
                flipped += 1;
                if flip_examples.len() < 12 {
                    flip_examples.push(
                        v.get("id").and_then(|x| x.as_str()).unwrap_or("?").to_string(),
                    );
                }
            }
            (true, _) => dedent_already_passed += 1,
            (false, false) => {
                both_fail += 1;
                if fail_feedback.len() < 10 {
                    fail_feedback.push(serde_json::json!({
                        "id": v.get("id").and_then(|x| x.as_str()).unwrap_or("?"),
                        "realign_feedback": orr.feedback.chars().take(130).collect::<String>(),
                    }));
                }
            }
        }
        if checked % 10 == 0 {
            eprintln!("  ...checked {checked} exposed, {flipped} flipped so far");
        }
    }

    let summary = serde_json::json!({
        "exposed_checked": exposed,
        "flipped_failed_to_verified": flipped,
        "still_fail_both": both_fail,
        "dedent_already_passed": dedent_already_passed,
        "flip_rate_among_exposed": if exposed > 0 { flipped as f64 / exposed as f64 } else { 0.0 },
        "flip_example_ids": flip_examples,
        "still_fail_feedback_samples": fail_feedback,
    });
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("verify") => {
            let path = args.get(2).expect("usage: verify <verify_set.jsonl> <mathlib_dir> [max]");
            let mathlib = args.get(3).expect("mathlib_dir required");
            let max = args.get(4).and_then(|s| s.parse::<usize>().ok());
            verify(path, mathlib, max);
        }
        Some(path) => scan(path),
        None => {
            eprintln!("usage: het_dealign_exposure <bodies.jsonl> | verify <set.jsonl> <mathlib_dir> [max]");
            std::process::exit(1);
        }
    }
}
