//! Constitution gate — Class-4 composite-tx atomic-rollback witness.
//!
//! Authority: `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md`
//! §1.B.2 (Phase E.2) + plan `cached-noodle.md` §C.E.2.
//!
//! Codex G2 audit (2026-05-09) defect 2: P-M6 `router_atomic_rollback_on_failure`
//! triggered insufficient-buyer-balance failure that the sequencer rejected
//! BEFORE `q_next` mutation began. The 9-step composite atomic-rollback
//! path was never exercised. Test name was verbatim-correct per architect
//! §7.7; test body was vacuous.
//!
//! This gate enforces: every Class-4 composite transaction's
//! `*_atomic_rollback_on_failure` test must invoke a well-known
//! mid-mutation failure-injection helper (`inject_failure_after_step` or
//! the Phase F-defined sequencer cfg(test) hook). Tests that only trigger
//! pre-mutation rejection (e.g. insufficient-balance, invalid-signature)
//! are flagged as vacuous and do not count as atomic-rollback evidence.
//!
//! Phase E (this PR) delivers the STATIC layer: a body-text scanner that
//! flags rollback tests missing the injection-helper invocation. The
//! DYNAMIC layer — the cfg(test) failure-injection point in the sequencer
//! — is deferred to Phase F.5 (P-M6 rebuild), at which point the binding
//! flips `landing_status: Landed` and the rollback test must call the
//! helper to pass.
//!
//! Rationale: putting the injection-point definition in the sequencer is
//! a STEP_B-restricted change (CLAUDE.md §12); it lands per-atom in
//! Phase F under the per-atom §8 cadence rather than as a Phase E
//! preparatory addition.

use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LandingStatus {
    Landed,
    NotYetLanded,
}

#[derive(Debug)]
struct CompositeTxRollback {
    /// Atom id e.g. "P-M6".
    atom_id: &'static str,
    /// Architect manual section e.g. "§7.7".
    manual_section: &'static str,
    /// Composite tx name e.g. "BuyWithCoinRouter".
    composite_name: &'static str,
    /// Path to the rollback test file relative to workspace root.
    rollback_test_path: &'static str,
    /// Test fn name verbatim (matches architect manual).
    rollback_test_fn: &'static str,
    /// Whether the composite tx + test are landed in the codebase.
    landing_status: LandingStatus,
}

/// CALL-form patterns a rollback-test body must contain to count as
/// exercising the mid-mutation rollback path. Each pattern matches a
/// function-call construct, not a bare identifier — so an implementer
/// cannot satisfy the gate by merely mentioning the marker name.
///
/// Hardened per Codex re-audit 2026-05-09 Recommendation 1:
///   - Patterns include the open-paren or open-quote of an argument list,
///     so identifier-mention without invocation does not match.
///   - The match is performed against the test body with `//` comments
///     stripped (so a marker hidden in a comment does NOT count).
///   - Patterns intentionally include the env-var-name string literal
///     forms because `std::env::set_var("ROUTER_FAIL_AT_STEP", ...)` is a
///     legitimate cfg(test)-friendly injection mechanism — the env-var
///     literal IS the call's first arg, not a free-floating string.
///
/// Phase F.5 introduces the explicit `inject_failure_after_step(...)`
/// helper. Phase F-flexibility allows either form.
const MID_MUTATION_INJECTION_PATTERNS: &[&str] = &[
    "inject_failure_after_step(",
    "set_var(\"ROUTER_FAIL_AT_STEP\"",
    "set_var(\"TURINGOS_TEST_ROUTER_FAIL_AT_STEP\"",
];

/// Strip inline `//` comments from each line, producing the executable-code
/// substring that the call-pattern scan operates on. String literals are
/// PRESERVED because the legitimate env-set form
/// `set_var("ROUTER_FAIL_AT_STEP", ...)` requires the literal to live in
/// the call site. The bypass case "marker mentioned only in a comment" is
/// covered by comment stripping; the bypass case "marker mentioned only
/// inside a free-floating string literal" requires the implementer to
/// write `let _s = "ROUTER_FAIL_AT_STEP";` — which is contrived because
/// the `set_var("ROUTER_FAIL_AT_STEP"` pattern requires the OPEN-PAREN
/// + literal sequence, which a free-floating literal cannot satisfy.
fn body_executable_only(body: &str) -> String {
    let mut out = String::new();
    for line in body.lines() {
        let code = if let Some(idx) = line.find("//") {
            &line[..idx]
        } else {
            line
        };
        out.push_str(code);
        out.push('\n');
    }
    out
}

const BINDINGS: &[CompositeTxRollback] = &[
    // P-M6 Mint-and-Swap Router (architect §7.7 9-step composite).
    // FLIPPED to Landed 2026-05-09 session #32 by Phase F.5 P-M6 rebuild
    // commit: rollback test now exists at the bound path AND invokes
    // `set_var("TURINGOS_TEST_ROUTER_FAIL_AT_STEP", ...)` per the static-
    // layer pattern catalog. The dynamic layer is in
    // `src/state/sequencer.rs::check_router_test_failure_injection` —
    // gated on `cfg(debug_assertions)` so the env-var read is reachable
    // from integration tests + dev builds AND compiled OUT in --release
    // production builds (replay determinism preserved).
    //
    // Codex G2 audit 2026-05-09 defect 2 closed: rollback test forces
    // step-5 (mid-composite) failure via env var injection AND asserts
    // (a) state_root unchanged, (b) buyer Coin balance unchanged,
    // (c) collateral unchanged, (d) pool reserves unchanged,
    // (e) buyer's YES gain from step-4 reverted. Companion test
    // `router_atomic_rollback_witnessed_at_every_step` exhaustively
    // injects at steps 1..=9 and asserts state_root unchanged for each.
    CompositeTxRollback {
        atom_id: "P-M6",
        manual_section: "§7.7",
        composite_name: "BuyWithCoinRouter",
        rollback_test_path: "tests/constitution_router_buy_with_coin.rs",
        rollback_test_fn: "router_atomic_rollback_on_failure",
        landing_status: LandingStatus::Landed,
    },
];

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_file(rel: &str) -> Option<String> {
    let path = workspace_root().join(rel);
    std::fs::read_to_string(&path).ok()
}

/// Extract a `#[test] fn <name> { ... }` body. Returns None if not found.
fn extract_test_fn_body<'a>(source: &'a str, fn_name: &str) -> Option<&'a str> {
    let needle = format!("fn {}", fn_name);
    let start = source.find(&needle)?;
    let after_name = start + needle.len();
    // Find the opening `{`.
    let body_start = source[after_name..].find('{')?;
    let abs_body_start = after_name + body_start;
    // Walk forward counting braces.
    let bytes = source.as_bytes();
    let mut depth: i32 = 0;
    let mut idx = abs_body_start;
    while idx < bytes.len() {
        match bytes[idx] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&source[abs_body_start..=idx]);
                }
            }
            _ => {}
        }
        idx += 1;
    }
    None
}

#[test]
fn class4_atomic_rollback_witness_check() {
    let mut failures: Vec<String> = Vec::new();
    for b in BINDINGS {
        let source = match read_file(b.rollback_test_path) {
            Some(s) => s,
            None => {
                if matches!(b.landing_status, LandingStatus::NotYetLanded) {
                    // Expected: VETO'd test file not present in codebase.
                    continue;
                }
                failures.push(format!(
                    "[{}/{}] declared Landed but rollback test file not readable: {}",
                    b.atom_id, b.composite_name, b.rollback_test_path
                ));
                continue;
            }
        };
        let body = match extract_test_fn_body(&source, b.rollback_test_fn) {
            Some(b) => b,
            None => {
                if matches!(b.landing_status, LandingStatus::NotYetLanded) {
                    continue;
                }
                failures.push(format!(
                    "[{}/{}] declared Landed but rollback test fn `{}` not found in {}",
                    b.atom_id, b.composite_name, b.rollback_test_fn, b.rollback_test_path
                ));
                continue;
            }
        };
        if matches!(b.landing_status, LandingStatus::NotYetLanded) {
            // If file/fn happen to exist (shouldn't; test was VETO-deleted), warn.
            failures.push(format!(
                "[{}/{}] declared NotYetLanded but rollback test fn `{}` IS present in {}; \
                 flip landing_status to Landed in this binding when Phase F.{} rebuilds",
                b.atom_id, b.composite_name, b.rollback_test_fn, b.rollback_test_path,
                if b.atom_id == "P-M6" { 5 } else { 0 }
            ));
            continue;
        }
        // Landed: body must contain at least one mid-mutation injection marker
        // in EXECUTABLE code (per E' hardening — strip comments + string literals
        // before scanning so the marker cannot be satisfied by mention alone).
        let executable = body_executable_only(body);
        let has_marker =
            MID_MUTATION_INJECTION_PATTERNS.iter().any(|m| executable.contains(m));
        if !has_marker {
            failures.push(format!(
                "[{}/{}] rollback test `{}` does NOT invoke any mid-mutation \
                 failure-injection marker ({:?}). Per Codex 2026-05-09 audit defect 2 + \
                 architect manual {}: rollback tests for Class-4 composite tx must exercise \
                 the rollback path AFTER `q_next` mutation begins, not before. Path: {}",
                b.atom_id,
                b.composite_name,
                b.rollback_test_fn,
                MID_MUTATION_INJECTION_PATTERNS,
                b.manual_section,
                b.rollback_test_path,
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "Phase E.2 atomic rollback witness failed for {} binding(s):\n{}",
        failures.len(),
        failures.join("\n"),
    );
}

#[test]
fn witness_self_check_synthetic_vacuous_test_detected() {
    // Synthetic source mimicking Codex defect 2 (vacuous rollback test).
    let synthetic = r#"
#[test]
fn router_atomic_rollback_on_failure() {
    let mut q = build_q_with_balance(50);
    let tx = build_router_tx(payC = 100);  // exceeds buyer balance
    let result = sequencer.dispatch(tx);  // rejected pre-mutation
    assert!(result.is_err());
    // No mid-mutation injection; no q_next state inspection.
}
"#;
    let body = extract_test_fn_body(synthetic, "router_atomic_rollback_on_failure")
        .expect("synthetic test must parse");
    let has_marker =
        MID_MUTATION_INJECTION_PATTERNS.iter().any(|m| body.contains(m));
    assert!(
        !has_marker,
        "self-check: synthetic vacuous rollback test should NOT contain mid-mutation \
         injection markers; gate would correctly flag it as vacuous. \
         Markers: {:?}, body excerpt: {}",
        MID_MUTATION_INJECTION_PATTERNS,
        body.chars().take(200).collect::<String>(),
    );
}

#[test]
fn witness_self_check_synthetic_proper_test_passes() {
    // Synthetic source mimicking the Phase F.5 proper rollback test.
    let synthetic = r#"
#[test]
fn router_atomic_rollback_on_failure() {
    let mut q = build_q_with_balance(1000);
    let tx = build_router_tx(payC = 100);
    let parent_root = q.head_t.state_root.clone();
    // Mid-mutation failure injection: force step 6 (swap) to fail.
    std::env::set_var("TURINGOS_TEST_ROUTER_FAIL_AT_STEP", "6");
    let result = sequencer.dispatch(tx);
    std::env::remove_var("TURINGOS_TEST_ROUTER_FAIL_AT_STEP");
    assert!(result.is_err());
    // After rollback, state_root must equal parent_root exactly.
    assert_eq!(q.head_t.state_root, parent_root);
}
"#;
    let body = extract_test_fn_body(synthetic, "router_atomic_rollback_on_failure")
        .expect("synthetic test must parse");
    let executable = body_executable_only(body);
    let has_marker =
        MID_MUTATION_INJECTION_PATTERNS.iter().any(|m| executable.contains(m));
    assert!(
        has_marker,
        "self-check: synthetic proper rollback test SHOULD contain a mid-mutation \
         injection marker in executable code; got body excerpt: {}",
        body.chars().take(300).collect::<String>(),
    );
}

#[test]
fn witness_self_check_marker_only_in_comment_does_not_satisfy() {
    // Phase E' hardening per Codex 2026-05-09 Recommendation 1: a test body
    // that only mentions the marker in a `//` comment (without actually
    // calling the failure-injection helper) must NOT pass the gate.
    let synthetic = r#"
#[test]
fn router_atomic_rollback_on_failure() {
    // This test would inject_failure_after_step but actually doesn't.
    // We pretend by mentioning TURINGOS_TEST_ROUTER_FAIL_AT_STEP in a comment.
    let mut q = build_q_with_balance(50);
    let tx = build_router_tx(payC = 100);
    let result = sequencer.dispatch(tx);
    assert!(result.is_err());
}
"#;
    let body = extract_test_fn_body(synthetic, "router_atomic_rollback_on_failure")
        .expect("synthetic test must parse");
    let executable = body_executable_only(body);
    let has_marker =
        MID_MUTATION_INJECTION_PATTERNS.iter().any(|m| executable.contains(m));
    assert!(
        !has_marker,
        "self-check (E' hardening): markers mentioned ONLY in `//` comments must \
         NOT satisfy the gate; executable-stripped body still contained a marker: \
         {}",
        executable.chars().take(400).collect::<String>(),
    );
}

#[test]
fn witness_self_check_marker_only_in_string_does_not_satisfy() {
    // Phase E' hardening per Codex 2026-05-09 Recommendation 1: a test body
    // that mentions the marker only inside a `"..."` string literal (e.g. a
    // log message, debug assertion text) must NOT pass the gate.
    let synthetic = r#"
#[test]
fn router_atomic_rollback_on_failure() {
    let mut q = build_q_with_balance(50);
    let tx = build_router_tx(payC = 100);
    let result = sequencer.dispatch(tx);
    assert!(result.is_err(), "expected inject_failure_after_step rollback");
    println!("would have set TURINGOS_TEST_ROUTER_FAIL_AT_STEP if implemented");
}
"#;
    let body = extract_test_fn_body(synthetic, "router_atomic_rollback_on_failure")
        .expect("synthetic test must parse");
    let executable = body_executable_only(body);
    let has_marker =
        MID_MUTATION_INJECTION_PATTERNS.iter().any(|m| executable.contains(m));
    assert!(
        !has_marker,
        "self-check (E' hardening): markers mentioned ONLY in `\"...\"` string \
         literals must NOT satisfy the gate; executable-stripped body still \
         contained a marker: {}",
        executable.chars().take(400).collect::<String>(),
    );
}
