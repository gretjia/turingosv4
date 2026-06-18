//! DECISIVE third-bug reproduction (Class 1, additive isolated test, model-free).
//!
//! R2 (audit) claims a THIRD extraction bug survives the two prior fixes:
//! `dedent` (lean_judge.rs:414) strips only the LONGEST COMMON leading-whitespace
//! prefix, so a proof body whose FIRST line is shallower than its siblings — the
//! model's natural "first tactic flush, continuation tactics indented" shape, and
//! the `extract_after_by` inline shape — cannot be re-anchored to col 0. The deeper
//! siblings then fall OUTSIDE the `by` block and a CORRECT proof is mislabeled Failed.
//!
//! This test proves/refutes that END TO END with KNOWN-GOOD proofs and NO model:
//!   control : verify(uniform reference_body)            EXPECT Verified
//!   pathol. : verify(first-line-shallow same proof)     EXPECT NOT Verified  (the bug)
//!   mechanism: dedent(shallow) still leaves line2 indented (deterministic, no Lean)
//!
//! Identical proof CONTENT in both arms — only line-1 indentation differs — so a
//! verdict flip is attributable to the dedent de-alignment alone.
//!
//! UPDATE 2026-06-14 (OBL-018): this test still PASSES and SHOULD — it documents a real,
//! BY-DESIGN boundary of the SHARED judge: `dedent` (lean_judge.rs:414) is deliberately
//! conservative and does NOT recover a first-line-shallow body. The handoff §5 recipe to
//! make the shared `dedent` "re-anchor to the shallowest column" is INSUFFICIENT
//! (min-indent fixes neither injection point) and UNDESIRABLE (an aggressive reflow in a
//! shared judge risks restructuring genuinely-nested production proofs in
//! `lean_market_agent`). The production fix instead lives at the probe's EXTRACTION
//! (`het_capability_probe::realign`): it flushes a FLAT tactic sequence to col 0 while
//! deferring nested bodies to the conservative `dedent`. That fix is proven end-to-end by
//! the `het_capability_probe` unit tests `realign_*` /
//! `extract_then_verify_first_line_shallow_real_lean`. Keep this test as the judge-boundary
//! witness; do NOT "fix" it by mutating the shared judge without architect ratification.

use std::path::PathBuf;
use std::time::Duration;

use turingosv4::judges::lean_judge::{dedent, default_lean_bin, LeanJudge};
use turingosv4::judges::lean_theorem_bank::{
    default_lake_bin, load_bank, mathlib_lean_path, LeanTheorem,
};

const MATHLIB_DIR: &str = "/Users/zephryj/work/mathlib4";

/// Clean cases: proofs that are sequences of TOP-LEVEL tactics, so stripping the
/// first line's indent yields an unambiguous first-line-shallow body (no fragile
/// line-1→line-2 continuation).
const CLEAN_IDS: &[&str] = &["lm_det_zero", "lm_nt_cop_cubic"];

fn build_judge_like_probe(t: &LeanTheorem, lean_bin: PathBuf, lp: Option<&str>) -> LeanJudge {
    let mut j = LeanJudge::new(t.preamble.clone());
    j.lean_bin = lean_bin;
    j.cwd = PathBuf::from(MATHLIB_DIR);
    j.timeout = Duration::from_secs(120);
    if t.needs_mathlib {
        if let Some(lp) = lp {
            j.extra_env.push(("LEAN_PATH".to_string(), lp.to_string()));
        }
    }
    j
}

/// Strip leading whitespace from the FIRST line only, leaving siblings as-is.
/// Models the natural "first tactic flush, rest indented" JSON proof_body shape.
fn first_line_shallow(body: &str) -> String {
    let mut lines = body.lines();
    let first = lines.next().unwrap_or("").trim_start();
    let rest: Vec<&str> = lines.collect();
    if rest.is_empty() {
        first.to_string()
    } else {
        format!("{first}\n{}", rest.join("\n"))
    }
}

#[test]
fn third_bug_first_line_shallow_mislabels_good_proof() {
    let bin = default_lean_bin();
    if !(bin.is_absolute() && bin.exists()) {
        eprintln!("skip: pinned Lean toolchain absent");
        return;
    }
    let lp = mathlib_lean_path(PathBuf::from(MATHLIB_DIR), &default_lake_bin());
    if lp.is_none() {
        eprintln!("skip: could not resolve Mathlib LEAN_PATH");
        return;
    }
    let bank_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/lean_theorems_pool.jsonl");
    let bank = load_bank(&bank_path).expect("load pool bank");

    let mut reproduced = 0usize;
    for id in CLEAN_IDS {
        let t = bank.iter().find(|x| x.id == *id).expect("target in bank");
        let uniform = &t.reference_body;
        let shallow = first_line_shallow(uniform);

        // Sanity: the two arms must be the SAME proof content (same trimmed lines).
        let norm = |s: &str| {
            s.lines().map(|l| l.trim()).collect::<Vec<_>>().join("\n")
        };
        assert_eq!(
            norm(uniform),
            norm(&shallow),
            "{id}: arms differ in content, not just indentation"
        );

        // MECHANISM (deterministic, no Lean): dedent must FAIL to re-anchor the
        // shallow body — line 1 ends at col 0 while a later line stays indented.
        let dd = dedent(&shallow);
        let line2_indent = dd
            .lines()
            .nth(1)
            .map(|l| l.len() - l.trim_start().len())
            .unwrap_or(0);
        eprintln!("[{id}] dedent(shallow) line2 indent = {line2_indent} (col0 anchor + indented sibling = de-aligned)");

        // END-TO-END through the real judge (assemble calls dedent again).
        let judge_u = build_judge_like_probe(t, bin.clone(), lp.as_deref());
        let ou = judge_u.verify(uniform);
        let judge_s = build_judge_like_probe(t, bin.clone(), lp.as_deref());
        let os = judge_s.verify(&shallow);

        eprintln!(
            "[{id}] uniform -> is_verified={} | shallow -> is_verified={} feedback={:?}",
            ou.is_verified(),
            os.is_verified(),
            os.feedback.chars().take(160).collect::<String>()
        );

        // Control: the uniform known-good proof MUST verify.
        assert!(ou.is_verified(), "{id}: uniform control failed to verify: {ou:?}");

        if !os.is_verified() {
            reproduced += 1;
        }
    }

    // DECISIVE PREDICATE: the third bug is real iff ≥1 known-good proof flips to
    // NOT-Verified purely from first-line-shallow indentation.
    assert!(
        reproduced > 0,
        "third bug NOT reproduced: every shallow variant still verified — R2's \
         first-line-shallower de-alignment claim is REFUTED end-to-end"
    );
    eprintln!("THIRD BUG CONFIRMED: {reproduced}/{} known-good proofs mislabeled Failed by first-line-shallow de-alignment.", CLEAN_IDS.len());
}
