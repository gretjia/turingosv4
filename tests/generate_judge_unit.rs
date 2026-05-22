//! TRACE_MATRIX FC1a-predicate_pi: Atom 19 unit tests for GenerateJudge
//! exported as integration tests (run via `cargo test --test generate_judge_unit`).
//!
//! The implementation tests live inline in `src/judges/generate_judge.rs::tests`.
//! This file additionally verifies the AnyJudge::Generate construction path
//! and the verdict round-trip through the type-erased `AnyJudge::verdict()`
//! surface — the actual contract the `tdma_runner` exercises.
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_GENERATE_PHASE_E_DIRECTIVE_AND_§8.md

use turingosv4::tdma_runner::AnyJudge;

#[test]
fn any_judge_generate_constructs_with_entrypoint() {
    let j = AnyJudge::generate("main.py".to_string(), false);
    assert_eq!(j.total_stages(), 1);
    assert_eq!(j.current_stage_label(), "Compile");
}

#[test]
fn any_judge_generate_passes_valid_bundle() {
    let j = AnyJudge::generate("main.py".to_string(), false);
    let body = r#"### File: main.py
```python
print("ok")
```
"#;
    let (success, class, _pred, _reason) = j.verdict(body, &[]);
    assert!(success, "class={}", class);
    assert_eq!(class, "pass");
}

#[test]
fn any_judge_generate_rejects_no_files() {
    let j = AnyJudge::generate("main.py".to_string(), false);
    let (success, class, pred, _reason) = j.verdict("no files in this message", &[]);
    assert!(!success);
    assert_eq!(class, "no-files-parsed");
    assert_eq!(pred, "bundle.has_at_least_one_file");
}

#[test]
fn any_judge_generate_rejects_path_traversal() {
    let j = AnyJudge::generate("main.py".to_string(), false);
    let body = r#"### File: ../../etc/passwd
```
exploit
```
"#;
    let (success, class, _pred, _reason) = j.verdict(body, &[]);
    assert!(!success);
    assert_eq!(class, "path-traversal");
}

#[test]
fn any_judge_generate_rejects_missing_entrypoint() {
    let j = AnyJudge::generate("main.py".to_string(), false);
    let body = r#"### File: helper.py
```python
def helper(): pass
```
"#;
    let (success, class, _pred, _reason) = j.verdict(body, &[]);
    assert!(!success);
    assert_eq!(class, "missing-entrypoint");
}

#[test]
fn any_judge_generate_advance_terminates_after_first_proceed() {
    let mut j = AnyJudge::generate("main.py".to_string(), false);
    let before = j.current_stage_label();
    j.advance();
    let after = j.current_stage_label();
    assert_eq!(before, "Compile");
    assert_eq!(after, "Compile", "single-stage judge stays at Compile");
}
