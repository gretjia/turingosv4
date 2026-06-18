//! TASK E1 — model-free judge positive control (Class 1, additive isolated test).
//!
//! Closes the control-coverage gap: `lean_theorem_bank::reference_proofs_verify`
//! only covers `tests/fixtures/lean_theorems.jsonl` (6 core), NOT the
//! `tests/fixtures/lean_theorems_pool.jsonl` (44) that the het_capability_probe
//! actually loads. This feeds each TARGET theorem's KNOWN-GOOD `reference_body`
//! through the SAME assemble+verify+axiom-check path the probe uses
//! (`LeanJudge::verify`), and asserts it deterministically yields
//! Verified + axiom-clean (axioms ⊆ {propext, Classical.choice, Quot.sound}).
//!
//! NO model, NO network, NO API. Pure deterministic Lean-kernel control.
//! If a known-good reference body does NOT verify here, the judge path itself
//! has a bug (debug, not clean).
//!
//! The judge is built EXACTLY as the probe's `build_judge`
//! (het_capability_probe.rs:502): `LeanJudge::new(preamble)` + lean_bin +
//! cwd=/Users/zephryj/work/mathlib4 + timeout=120s + LEAN_PATH (needs_mathlib).

use std::path::PathBuf;
use std::time::Duration;

use turingosv4::judges::lean_judge::{default_lean_bin, LeanJudge, AXIOM_WHITELIST};
use turingosv4::judges::lean_theorem_bank::{
    default_lake_bin, load_bank, mathlib_lean_path, LeanTheorem,
};

/// The 6 target theorems with known-good reference bodies (per E1 inputs R5).
const TARGET_IDS: &[&str] = &[
    "lm_det_zero",
    "lm_c",
    "lm_coeff_mul",
    "lm_e",
    "lm_lim1",
    "lm_nt_cop_cubic",
];

const MATHLIB_DIR: &str = "/Users/zephryj/work/mathlib4";

/// Build the judge EXACTLY as the probe's private `build_judge` does
/// (het_capability_probe.rs:502-514) — replicated verbatim (5 lines) since it is
/// private to the bin.
fn build_judge_like_probe(
    theorem: &LeanTheorem,
    lean_bin: PathBuf,
    mathlib_lp: Option<&str>,
) -> LeanJudge {
    let mut j = LeanJudge::new(theorem.preamble.clone());
    j.lean_bin = lean_bin;
    j.cwd = PathBuf::from(MATHLIB_DIR);
    j.timeout = Duration::from_secs(120);
    if theorem.needs_mathlib {
        if let Some(lp) = mathlib_lp {
            j.extra_env.push(("LEAN_PATH".to_string(), lp.to_string()));
        }
    }
    j
}

#[test]
fn pool_target_reference_bodies_verify_clean() {
    // Gate on the pinned toolchain — skip (not fail) if absent, like the probe.
    let bin = default_lean_bin();
    if !(bin.is_absolute() && bin.exists()) {
        eprintln!("skip: pinned Lean toolchain absent (default_lean_bin not absolute/existing)");
        return;
    }

    // Resolve the Mathlib LEAN_PATH the same way the probe does (lake env, pinned lake).
    let mathlib_lp = mathlib_lean_path(PathBuf::from(MATHLIB_DIR), &default_lake_bin());
    if mathlib_lp.is_none() {
        eprintln!("skip: could not resolve Mathlib LEAN_PATH (lake env failed)");
        return;
    }

    // Load the SAME bank the probe loads.
    let bank_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/lean_theorems_pool.jsonl");
    let bank = load_bank(&bank_path).expect("load pool bank");

    for id in TARGET_IDS {
        let t = bank
            .iter()
            .find(|x| x.id == *id)
            .unwrap_or_else(|| panic!("target {id} not in pool bank"));

        let judge = build_judge_like_probe(t, bin.clone(), mathlib_lp.as_deref());
        let o = judge.verify(&t.reference_body);

        // (1) Verdict must be Verified.
        assert!(
            o.is_verified(),
            "{id}: reference body did NOT verify (judge path bug, NOT model): {o:?}"
        );
        // (2) Must NOT be an axiom soundness reject.
        assert!(
            !o.axiom_rejected,
            "{id}: reference body axiom-rejected: {o:?}"
        );
        // (3) Printed axiom set must be ⊆ the classical trust base. This is the
        //     honest soundness assertion — a positively-checked #print axioms set,
        //     not a grep-only pass (native_decide pulls Lean.ofReduceBool which the
        //     gate would have rejected).
        let bad: Vec<&String> = o
            .axioms
            .iter()
            .filter(|a| !AXIOM_WHITELIST.contains(&a.as_str()))
            .collect();
        assert!(
            bad.is_empty(),
            "{id}: non-whitelist axioms {bad:?} in {:?}",
            o.axioms
        );

        eprintln!("OK {id}: Verified, axioms={:?}", o.axioms);
    }
}
