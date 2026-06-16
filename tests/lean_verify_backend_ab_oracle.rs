//! A/B equivalence oracle for the persistent-Lean verify backend (Class 2,
//! additive test). PROVES `VerifyBackend::PersistentService` (the warm
//! `scripts/lean_verify_service.py`, ~130-260x faster) produces a BYTE-IDENTICAL
//! `LeanOutcome` to `VerifyBackend::ProcessSpawn` (the V1-pinned reference path, a
//! fresh `lean -DwarningAsError=true` per run) on the decision-bearing fields. This
//! is the gate the boot doc requires before the fast path is trusted: "keep the
//! lean-per-verify path as a feature-flagged EQUIVALENCE ORACLE and A/B it (assert
//! byte-identical verdicts) before trusting the fast path."
//!
//! Decision-bearing fields asserted identical: `verdict_kind`, `error_class`,
//! `exit_code`, `axiom_check_status`, `axiom_rejected`, `axioms` (as a set),
//! `rejected_axioms` (as a set). The ONLY field permitted to differ is `feedback`
//! — a bounded, shielded retry hint (the first `error:` line); it is NOT on the CAS
//! `VerifierResult` and not a primary-metric input, and the two backends surface
//! lean's vs the REPL's diagnostic text. (sorry/admit/native_decide candidates are
//! rejected by verify()'s SHARED source-scan BEFORE either backend runs, so they
//! match trivially; the oracle still exercises them.)
//!
//! GATED: skips (does not fail) unless BOTH the pinned toolchain is present AND
//! `TURINGOS_LEAN_VERIFY_PYTHON` is set (a venv python with lean-interact). Run:
//!   TURINGOS_LEAN_VERIFY_PYTHON=/path/to/venv/bin/python \
//!   cargo test --test lean_verify_backend_ab_oracle -- --nocapture

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::time::Duration;

use turingosv4::judges::lean_judge::{default_lean_bin, LeanJudge, LeanOutcome, VerifyBackend};
use turingosv4::judges::lean_theorem_bank::{default_lake_bin, load_bank, mathlib_lean_path};

const MATHLIB_DIR: &str = "/Users/zephryj/work/mathlib4";

fn build_judge(theorem_preamble: &str, lean_bin: PathBuf, mathlib_lp: Option<&str>) -> LeanJudge {
    let mut j = LeanJudge::new(theorem_preamble.to_string());
    j.lean_bin = lean_bin;
    j.cwd = PathBuf::from(MATHLIB_DIR);
    j.timeout = Duration::from_secs(120);
    if let Some(lp) = mathlib_lp {
        j.extra_env.push(("LEAN_PATH".to_string(), lp.to_string()));
    }
    j
}

/// The decision-bearing projection of a LeanOutcome — everything the verdict and
/// the on-tape/manifest soundness footprint depend on. `feedback` is deliberately
/// excluded (see module doc).
#[derive(Debug, PartialEq, Eq)]
struct Decision {
    verdict_kind: String,
    error_class: String,
    exit_code: i32,
    axiom_check_status: String,
    axiom_rejected: bool,
    axioms: BTreeSet<String>,
    rejected_axioms: BTreeSet<String>,
}

fn decision(o: &LeanOutcome) -> Decision {
    Decision {
        verdict_kind: format!("{:?}", o.verdict_kind),
        error_class: format!("{:?}", o.error_class),
        exit_code: o.exit_code,
        axiom_check_status: format!("{:?}", o.axiom_check_status),
        axiom_rejected: o.axiom_rejected,
        axioms: o.axioms.iter().cloned().collect(),
        rejected_axioms: o.rejected_axioms.iter().cloned().collect(),
    }
}

/// Run one (preamble, body) through both backends and assert decision-equivalence.
fn assert_equivalent(
    label: &str,
    preamble: &str,
    body: &str,
    lean_bin: &PathBuf,
    mathlib_lp: Option<&str>,
) {
    let ps = build_judge(preamble, lean_bin.clone(), mathlib_lp);
    let out_ps = ps.verify(body);

    let mut svc = build_judge(preamble, lean_bin.clone(), mathlib_lp);
    svc.backend = VerifyBackend::PersistentService;
    let out_svc = svc.verify(body);

    let d_ps = decision(&out_ps);
    let d_svc = decision(&out_svc);
    assert_eq!(
        d_ps, d_svc,
        "[{label}] backend divergence:\n  ProcessSpawn={d_ps:?}\n  Service={d_svc:?}\n  \
         (ps.feedback={:?} svc.feedback={:?})",
        out_ps.feedback, out_svc.feedback
    );
    eprintln!(
        "OK {label}: {} axioms={:?} (both backends identical)",
        d_ps.verdict_kind, d_ps.axioms
    );
}

#[test]
fn persistent_service_is_byte_equivalent_to_process_spawn() {
    let bin = default_lean_bin();
    if !(bin.is_absolute() && bin.exists()) {
        eprintln!("skip: pinned Lean toolchain absent");
        return;
    }
    if std::env::var("TURINGOS_LEAN_VERIFY_PYTHON").is_err() {
        eprintln!("skip: TURINGOS_LEAN_VERIFY_PYTHON not set (no lean-interact venv)");
        return;
    }
    let mathlib_lp = mathlib_lean_path(PathBuf::from(MATHLIB_DIR), &default_lake_bin());
    if mathlib_lp.is_none() {
        eprintln!("skip: could not resolve Mathlib LEAN_PATH");
        return;
    }
    let lp = mathlib_lp.as_deref();

    // (A) Every pool reference body (known-good, Verified, axiom-clean) — the bulk of
    //     the calibration pool the sweep will actually verify.
    let bank_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/lean_theorems_pool.jsonl");
    let bank = load_bank(&bank_path).expect("load pool bank");
    assert!(!bank.is_empty(), "pool bank empty");
    let mut compared = 0usize;
    for t in &bank {
        // The service fast path is for Mathlib-importing candidates (BASE_ENV =
        // import Mathlib); ProcessSpawn covers the rest. Only A/B those on the fast
        // path's domain.
        if !t.needs_mathlib {
            continue;
        }
        assert_equivalent(&t.id, &t.preamble, &t.reference_body, &bin, lp);
        compared += 1;
    }

    // (B) Adversarial cases across the verdict space, on a Mathlib preamble, so the
    //     oracle covers Failed + soundness-reject + source-scan rejects, not only
    //     Verified. Use a simple decidable statement.
    let pre_true = "import Mathlib\ntheorem t : True := by";
    assert_equivalent("adv_clean_trivial", pre_true, "  trivial", &bin, lp);
    let pre_eq = "import Mathlib\ntheorem t : (2 : Nat) + 2 = 5 := by";
    assert_equivalent("adv_wrong_rfl", pre_eq, "  rfl", &bin, lp);
    assert_equivalent("adv_sorry", pre_true, "  sorry", &bin, lp);
    assert_equivalent("adv_native_decide", pre_true, "  native_decide", &bin, lp);
    let pre_axiom = "import Mathlib\naxiom evil : (2 : Nat) + 2 = 5\ntheorem t : (2 : Nat) + 2 = 5 := by";
    assert_equivalent("adv_hand_axiom", pre_axiom, "  exact evil", &bin, lp);

    eprintln!(
        "A/B ORACLE PASS: {compared} pool reference bodies + 5 adversarial cases — \
         ProcessSpawn and PersistentService produced identical decisions on all."
    );
}
