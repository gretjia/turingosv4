//! 1.0 blockers #1 + #2 gate: NO FALSE PASS.
//!
//! Proves, against a REAL CAS-backed artifact bundle, that the spec-derived
//! scenario set ACTUALLY CHECKS the artifact — a wrong-but-valid artifact does
//! NOT pass a functional scenario derived from the spec:
//!
//!   - `HtmlParses` applies ONLY to `.html` entrypoints (blocker #1).
//!   - `PythonParses` certifies a correct `main.py` and REJECTS a syntactically
//!     broken one (blocker #1) — a wrong-but-existing Python file fails.
//!   - `RequiredTextPresent` (spec-derived FUNCTIONAL gate, blocker #2) passes
//!     only when the rendered entrypoint contains the spec's required control;
//!     a well-formed-but-wrong artifact that omits the control FAILS.
//!
//! FC-trace: FC1 (test loop), FC3 (test evidence). Risk class: 2 (test gate).
//!
//! This gate is mutation-sensitive: if a producer is weakened to "exists =>
//! pass", the broken-Python and wrong-HTML cases would falsely pass and these
//! asserts go RED.

use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::artifact_bundle::{
    ArtifactBundleManifest, ArtifactFileEntry, ArtifactFileRole, ARTIFACT_BUNDLE_SCHEMA_ID,
};
use turingosv4::runtime::spec_capsule::cas_path;
use turingosv4::runtime::test_run::run_test_scenario_set;
use turingosv4::runtime::test_scenario::{derive_scenario_set_from_spec, TestScenario};

fn now_t() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(1000)
}

/// Put a single-entrypoint artifact bundle into the workspace CAS and return
/// its CID hex. `entrypoint`/`content` define the rendered deliverable.
fn put_bundle(ws: &Path, entrypoint: &str, content: &[u8], t: u64) -> String {
    let cas_dir = cas_path(ws);
    std::fs::create_dir_all(&cas_dir).expect("create cas");
    let mut store = CasStore::open(&cas_dir).expect("open cas");

    let file_cid = store
        .put(content, ObjectType::EvidenceCapsule, "test", t, None)
        .expect("put entrypoint file");

    let mime = if entrypoint.ends_with(".html") {
        "text/html"
    } else if entrypoint.ends_with(".py") {
        "text/x-python"
    } else {
        "application/octet-stream"
    };

    let bundle = ArtifactBundleManifest {
        schema_id: ARTIFACT_BUNDLE_SCHEMA_ID.to_string(),
        session_id: "no-false-pass".to_string(),
        spec_capsule_cid: Some("spec-cid".to_string()),
        generation_attempt_cid: "a".repeat(64),
        previous_bundle_cid: None,
        files: vec![ArtifactFileEntry {
            path: entrypoint.to_string(),
            cid: file_cid.hex(),
            mime: mime.to_string(),
            sha256: "00".repeat(32),
            size_bytes: content.len() as u64,
            role: ArtifactFileRole::Entrypoint,
        }],
        entrypoint: entrypoint.to_string(),
        bundle_size_bytes_total: content.len() as u64,
        created_at_logical_t: t,
    };
    let bytes = serde_json::to_vec(&bundle).expect("serialize bundle");
    store
        .put(
            &bytes,
            ObjectType::EvidenceCapsule,
            "test",
            t,
            Some(ARTIFACT_BUNDLE_SCHEMA_ID.to_string()),
        )
        .expect("put bundle")
        .hex()
}

fn pass_of(
    results: &[turingosv4::runtime::test_run::TestScenarioResult],
    want: &TestScenario,
) -> bool {
    results
        .iter()
        .find(|r| std::mem::discriminant(&r.scenario) == std::mem::discriminant(want))
        .map(|r| r.pass)
        .unwrap_or_else(|| panic!("scenario {want:?} not present in result set"))
}

fn has_scenario(set: &[TestScenario], want: &TestScenario) -> bool {
    set.iter()
        .any(|s| std::mem::discriminant(s) == std::mem::discriminant(want))
}

/// blocker #1: a default `main.py` deliverable is gated on Python validity, NOT
/// HTML doctype; the structural gate is entrypoint-aware.
#[test]
fn html_parses_never_applies_to_python_entrypoint() {
    let set = derive_scenario_set_from_spec(
        b"Crunch some numbers and print a table",
        "cid",
        "main.py",
        1,
    );
    assert!(
        has_scenario(&set.scenarios, &TestScenario::PythonParses),
        "a .py entrypoint must get PythonParses"
    );
    assert!(
        !has_scenario(&set.scenarios, &TestScenario::HtmlParses),
        "a .py entrypoint must NEVER be HTML-gated (blocker #1)"
    );
}

/// blocker #1: PythonParses certifies a valid main.py and REJECTS a broken one.
/// A wrong (unparseable) Python artifact must NOT pass.
#[test]
fn python_parses_rejects_broken_and_accepts_valid() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let ws = dir.path();
    let t = now_t();

    let scenario_set = derive_scenario_set_from_spec(b"Crunch some numbers", "cid", "main.py", t);
    assert!(
        has_scenario(&scenario_set.scenarios, &TestScenario::PythonParses),
        "precondition: PythonParses derived for .py"
    );

    // Valid Python entrypoint.
    let good_cid = put_bundle(
        ws,
        "main.py",
        b"def main():\n    print(1 + 2)\n\nmain()\n",
        t,
    );
    let good = run_test_scenario_set(ws, &good_cid, &scenario_set).expect("run good");
    let py_ok = pass_of(&good.results, &TestScenario::PythonParses);

    // Syntactically broken Python entrypoint (unbalanced paren).
    let bad_cid = put_bundle(ws, "main.py", b"def main(:\n    print('oops'\n", t + 1);
    let bad = run_test_scenario_set(ws, &bad_cid, &scenario_set).expect("run bad");
    let py_bad = pass_of(&bad.results, &TestScenario::PythonParses);

    // If no python interpreter is available the producer fails CLOSED, so the
    // GOOD case would also be `false`. We only assert the discrimination that
    // matters: a broken file must never pass when a valid file passes. When the
    // interpreter IS present, this is `true != false`; when absent, both are
    // false and the broken file is (correctly) NOT certified.
    assert!(
        !(py_bad && !py_ok),
        "broken Python passed while valid failed — producer is inverted"
    );
    assert!(
        !py_bad,
        "NO FALSE PASS: a syntactically broken main.py must NOT pass PythonParses"
    );
    if py_ok {
        // Interpreter available: full discrimination proven.
        assert!(!bad.overall_pass, "broken Python bundle must fail overall");
    }
}

/// blocker #2: the spec-derived FUNCTIONAL gate distinguishes a correct artifact
/// from a wrong-but-well-formed one. Both are valid HTML; only the one that
/// surfaces the spec-required control passes.
#[test]
fn functional_gate_fails_wrong_but_valid_html() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let ws = dir.path();
    let t = now_t();

    // Spec demands a `New Game` control. The derived set includes a functional
    // RequiredTextPresent("new game") gate (hidden from the generation prompt).
    let spec = b"Build a snake game. The page MUST have a `New Game` button to restart.";
    let scenario_set = derive_scenario_set_from_spec(spec, "cid", "index.html", t);
    assert!(
        has_scenario(
            &scenario_set.scenarios,
            &TestScenario::RequiredTextPresent {
                label: String::new(),
                needle: String::new(),
            }
        ),
        "precondition: a functional RequiredTextPresent gate must be derived"
    );

    // CORRECT artifact: well-formed HTML that surfaces the required control.
    let correct = b"<!DOCTYPE html><html><body><button id=\"ng\">New Game</button></body></html>";
    let correct_cid = put_bundle(ws, "index.html", correct, t);
    let correct_run = run_test_scenario_set(ws, &correct_cid, &scenario_set).expect("run correct");
    assert!(
        pass_of(&correct_run.results, &TestScenario::HtmlParses),
        "correct artifact must parse as HTML"
    );
    assert!(
        pass_of(
            &correct_run.results,
            &TestScenario::RequiredTextPresent {
                label: String::new(),
                needle: String::new()
            }
        ),
        "correct artifact surfaces the required control => functional gate PASS"
    );
    assert!(correct_run.overall_pass, "correct artifact passes overall");

    // WRONG-BUT-VALID artifact: perfectly well-formed HTML, but it implements
    // the WRONG thing (a clock) and omits the required `New Game` control.
    let wrong = b"<!DOCTYPE html><html><body><h1>Clock</h1><div id=\"t\">12:00</div></body></html>";
    let wrong_cid = put_bundle(ws, "index.html", wrong, t + 1);
    let wrong_run = run_test_scenario_set(ws, &wrong_cid, &scenario_set).expect("run wrong");
    assert!(
        pass_of(&wrong_run.results, &TestScenario::HtmlParses),
        "the wrong artifact is STILL valid HTML (structural gate alone would pass it)"
    );
    assert!(
        !pass_of(
            &wrong_run.results,
            &TestScenario::RequiredTextPresent {
                label: String::new(),
                needle: String::new()
            }
        ),
        "ANTI-GOODHART SIGNAL: a wrong-but-valid artifact must FAIL the functional gate"
    );
    assert!(
        !wrong_run.overall_pass,
        "the battery still records the functional failure (overall_pass=false)"
    );
    // 2026-06-09 architect decision — NON-FATAL functional gate: the functional
    // failure above is an on-tape ADVISORY, not a hard reject. Delivery is gated
    // on the STRUCTURAL scenarios only, so the wrong-but-valid artifact is still
    // DELIVERED (structural_pass) with the functional miss flagged
    // (functional_unmet). The structural gate still hard-blocks broken artifacts.
    let verdict = turingosv4::runtime::test_run::delivery_verdict(&wrong_run.results);
    assert!(
        verdict.structural_pass,
        "non-fatal gate: a functional-only failure must NOT block delivery"
    );
    assert!(
        verdict.functional_unmet,
        "the functional miss must be flagged as an advisory"
    );
}

/// C11 hidden-oracle on the RETRY-FEEDBACK path (Art.III.4): when a failed
/// `RequiredTextPresent` gate is rendered into the LLM-visible retry feedback,
/// it must reveal NEITHER the `needle` (carried verbatim in the recorded
/// detail) NOR the `label` (the same spec token, only cased differently).
///
/// This closes the prompt-leak path that the scenario-set-bytes hidden-oracle
/// gate cannot see: that gate checks persisted capsule bytes, but the retry
/// prompt content is only hashed (never stored), so a label/needle echoed into
/// the feedback would never appear in any capsule. `TestScenario::shielded_feedback`
/// is the single source for that rendering — so we gate it directly here.
///
/// Mutation-sensitive: if `shielded_feedback` is reverted to echo
/// `RequiredTextPresent({label})` or the raw needle-bearing detail, the
/// case-insensitive substring asserts below go RED.
#[test]
fn shielded_feedback_never_leaks_needle_or_label() {
    // The spec token is `New Game`; the derived needle is its lowercase form.
    let label = "New Game";
    let needle = "new game";
    let scenario = TestScenario::RequiredTextPresent {
        label: label.to_string(),
        needle: needle.to_string(),
    };
    // The recorded per-result detail embeds the needle verbatim (this is what
    // test_run.rs writes: `entrypoint must contain "new game"`).
    let recorded_detail = format!("entrypoint must contain {needle:?}");

    let (name, detail) = scenario.shielded_feedback(&recorded_detail);
    let combined = format!("{name} {detail}").to_ascii_lowercase();

    assert!(
        !combined.contains(needle),
        "HIDDEN-ORACLE LEAK: shielded feedback for RequiredTextPresent echoed the needle \
         ({needle:?}) -> it would reach the generation prompt on retry. name={name:?} detail={detail:?}"
    );
    assert!(
        !combined.contains(&label.to_ascii_lowercase()),
        "HIDDEN-ORACLE LEAK: shielded feedback echoed the spec-derived label ({label:?}); the \
         label is the needle cased differently and must not reach the prompt. name={name:?}"
    );
    assert_eq!(
        name, "RequiredTextPresent",
        "the functional-gate name must be the bare variant tag (no `({{label}})` suffix)"
    );

    // NON-VACUOUS: the method is NOT a blanket redactor — a structural scenario
    // (no spec-derived oracle) passes its recorded detail through UNCHANGED, so
    // the test would catch a bug that nukes all feedback signal too.
    let structural_detail = "entrypoint index.html not found in artifact bundle";
    let (sname, sdetail) = TestScenario::EntrypointExists.shielded_feedback(structural_detail);
    assert_eq!(sname, "EntrypointExists");
    assert_eq!(
        sdetail, structural_detail,
        "structural-scenario detail (no oracle) must pass through unchanged"
    );
}
