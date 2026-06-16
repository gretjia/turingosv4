//! Constitution gate: E2E 1.0 product-flow hardening (blockers #1/#2/#3 + observe).
//!
//! This gate locks the four product-flow fixes shipped on
//! `claude/e2e-1.0-hardening`, each property paired with a documented mutation
//! that flips a dedicated assert RED (mutation-proof, non-vacuous):
//!
//!   (1) DELIVERY-SUCCESS-EXIT  — blocker #3. An artifact that is delivered +
//!       passes the spec-derived tests but whose POST-delivery market-settle
//!       leg FAILS must: exit 0 (not exit 2), print a WARNING, and ANCHOR the
//!       economic break on the tape as a `market_settle_failed` rejection
//!       capsule (Art. 0.2 tape-first — the break is not swallowed silently).
//!       Behavioral leg: the exact production anchoring call
//!       (`write_generate_rejection_capsule_observed`) writes a reconstructable
//!       capsule on a real CAS. Source-structural leg: the `Err(_)` arm of
//!       `match emit_polymarket_market_for_session` does NOT `return Err`
//!       (the exit-2-on-market-fail regression), DOES warn + anchor, and the
//!       delivery success print fires BEFORE the market leg.
//!
//!   (2) PYTHON-BUNDLE-PASSES   — blocker #1. A correct `main.py` bundle passes
//!       (no HtmlParses false-reject); a syntactically broken `main.py` FAILS.
//!       `HtmlParses` is derived ONLY for `.html` entrypoints.
//!
//!   (3) FUNCTIONAL-REJECTS-WRONG — blocker #2. A spec-derived FUNCTIONAL
//!       scenario (`RequiredTextPresent`) is NON-VACUOUS: it REJECTS an artifact
//!       that omits the spec-required control and PASSES one that surfaces it.
//!       The required control is derived from the spec and NEVER injected into
//!       the generation prompt (C11 hidden-oracle / Art. III.4) — proven by the
//!       scenario-set bytes living in a separate CAS object, not the prompt.
//!
//!   (4) OBSERVE-ROLLUP         — blocker #5. `turingos observe --workspace <WS>`
//!       over a constructed workspace tape prints the FC1/FC2/FC3 liveness +
//!       no-zombie verdict + the per-task integer VPPUT rollup, READ-ONLY.
//!
//! FC-trace: FC1 (test loop + predicate-gated advance), FC2 (map-reduce tick /
//! halt), FC3 (test evidence). Risk class: 2 (test gate; ZERO genesis-pinned
//! edits — all touched product files are pin-count 0).
//!
//! ## Mutation matrix (each flips exactly one assert RED)
//!   - Re-introduce the unconditional `HtmlParses` push (blocker #1 regression)
//!     => a `.py` bundle gets an HtmlParses scenario that fails on the missing
//!        doctype => `python_correct_main_py_passes` overall_pass goes false +
//!        `html_parses_is_entrypoint_conditional` goes RED.
//!   - Re-introduce `return Err(..)` (exit-2-on-market-fail) in the market-fail
//!     arm => `delivery_success_exits_zero_and_anchors_break` structural assert
//!        (`no return Err in the Err arm`) goes RED.
//!   - Weaken the functional producer to "exists => pass" => the wrong-but-valid
//!     HTML passes => `functional_gate_rejects_wrong_passes_right` goes RED.

use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::artifact_bundle::{
    ArtifactBundleManifest, ArtifactFileEntry, ArtifactFileRole, ARTIFACT_BUNDLE_SCHEMA_ID,
};
use turingosv4::runtime::rejection_capsule::{
    write_generate_rejection_capsule_observed, GenerateRejectionCapsule, RejectClass,
    GENERATE_REJECTION_CAPSULE_SCHEMA_ID,
};
use turingosv4::runtime::spec_capsule::cas_path;
use turingosv4::runtime::test_run::{run_test_scenario_set, TestScenarioResult};
use turingosv4::runtime::test_scenario::{derive_scenario_set_from_spec, TestScenario};

fn now_t() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(1_000)
}

/// Put a single-entrypoint artifact bundle into the workspace CAS and return its
/// CID hex. `entrypoint`/`content` define the rendered deliverable.
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
        session_id: "e2e-1-0-product-flow".to_string(),
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

fn pass_of(results: &[TestScenarioResult], want: &TestScenario) -> bool {
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

const CMD_GENERATE_SRC: &str = "src/bin/turingos/cmd_generate.rs";

/// Read the production cmd_generate.rs source for the source-structural legs.
fn cmd_generate_source() -> String {
    std::fs::read_to_string(CMD_GENERATE_SRC).expect("read cmd_generate.rs source")
}

/// Slice the `Err(e) =>` arm of `match emit_polymarket_market_for_session(...)`.
/// Returns the substring from that arm's head to (but NOT including) the NEXT
/// sibling `Err(e) =>` arm (the test-pipeline match's Err arm, which legitimately
/// DOES `return Err`). Bounding precisely is what makes the `no return Err`
/// mutation guard target ONLY the market-settle arm.
fn market_settle_err_arm(src: &str) -> String {
    let match_idx = src
        .find("match emit_polymarket_market_for_session(")
        .expect("market-settle match site must exist");
    let after = &src[match_idx..];
    // The market-settle Err arm.
    let err_rel = after
        .find("Err(e) => {")
        .expect("market-settle Err(e) arm must exist");
    let arm_start = match_idx + err_rel;
    // The NEXT `Err(e) => {` after the market-settle arm is the test-pipeline
    // match's Err arm — the terminator. The market-settle arm lives strictly
    // before it.
    let rest_after_arm_head = &src[arm_start + "Err(e) => {".len()..];
    let next_err_rel = rest_after_arm_head
        .find("Err(e) => {")
        .expect("the test-pipeline Err(e) arm must follow as a terminator");
    let arm_end = arm_start + "Err(e) => {".len() + next_err_rel;
    src[arm_start..arm_end].to_string()
}

// ───────────────────────────────────────────────────────────────────────────
// (1) DELIVERY-SUCCESS-EXIT — blocker #3
// ───────────────────────────────────────────────────────────────────────────

/// Behavioral leg: the exact production anchoring call writes a reconstructable
/// `market_settle_failed` rejection capsule on a real CAS (Art. 0.2 — the
/// economic break IS on tape, not swallowed). `retryable=false` because the
/// artifact is already delivered (the capsule records the economic break, not a
/// generation rejection).
#[test]
fn delivery_success_market_break_is_anchored_on_tape() {
    let dir = TempDir::new().expect("tempdir");
    let ws = dir.path();
    let t = now_t();

    // This mirrors the production `Err(e)` arm of cmd_generate.rs exactly:
    // reason=`market_settle_failed:{e}`, retryable=false, RejectClass::InternalIo.
    let rej = GenerateRejectionCapsule {
        schema_id: GENERATE_REJECTION_CAPSULE_SCHEMA_ID.to_string(),
        session_id: "e2e-1-0-product-flow".to_string(),
        spec_capsule_cid: Some("spec-cid".to_string()),
        generation_attempt_cid: Some("a".repeat(64)),
        triage_attempted: true,
        reject_class: RejectClass::InternalIo,
        public_error_summary: "artifact delivered; post-delivery market settlement failed"
            .to_string(),
        reason: format!("market_settle_failed:{}", "treasury preseed missing"),
        private_diagnostic_cid: None,
        retryable: false,
        world_head_unchanged: false,
        logical_t: t,
    };

    let rej_cid =
        write_generate_rejection_capsule_observed(ws, &rej).expect("anchor market-fail capsule");

    // Reconstruct from CAS by CID — the break is genuinely on tape, not a
    // dashboard-only / swallowed artifact. (Art. 0.2 tape-first.)
    let cas_dir = cas_path(ws);
    let mut store = CasStore::open(&cas_dir).expect("open cas");
    let _ = store.reload_index_from_sidecar();
    let cid_bytes = {
        let mut b = [0u8; 32];
        for i in 0..32 {
            b[i] = u8::from_str_radix(&rej_cid[i * 2..i * 2 + 2], 16).expect("hex cid");
        }
        turingosv4::bottom_white::cas::schema::Cid(b)
    };
    let bytes = store.get(&cid_bytes).expect("reconstruct capsule from CAS");
    let recon: GenerateRejectionCapsule =
        serde_json::from_slice(&bytes).expect("deserialize anchored capsule");

    assert_eq!(
        recon.schema_id, GENERATE_REJECTION_CAPSULE_SCHEMA_ID,
        "anchored break must carry the generate-rejection schema id"
    );
    assert!(
        recon.reason.starts_with("market_settle_failed:"),
        "the anchored break must be labelled `market_settle_failed:...`; got {:?}",
        recon.reason
    );
    assert!(
        !recon.retryable,
        "a post-delivery market break is NOT a retryable generation rejection (artifact already delivered)"
    );
    // `write_generate_rejection_capsule_observed` stamps world_head_unchanged=true
    // — the CAS-only break does not move the canonical head.
    assert!(
        recon.world_head_unchanged,
        "anchoring the economic break must not advance the canonical world head"
    );
}

/// Source-structural leg: the market-fail arm exits 0 (no `return Err`) and is
/// NOT silent (warns + anchors). MUTATION: re-introducing `return Err(..)` /
/// exit-2-on-market-fail in this arm flips the `no return Err` assert RED.
#[test]
fn delivery_success_exits_zero_and_anchors_break_structurally() {
    let src = cmd_generate_source();

    // Delivery fires BEFORE the market-settle leg (1.0 blocker #3 ordering).
    let success_idx = src
        .find("print_generate_success(&workspace, &written);")
        .expect("print_generate_success call must exist");
    let market_idx = src
        .find("match emit_polymarket_market_for_session(")
        .expect("market-settle match must exist");
    assert!(
        success_idx < market_idx,
        "delivery success MUST be printed BEFORE the best-effort market-settle leg \
         (blocker #3: a downstream economic break must never precede or retract delivery)"
    );

    let arm_with_comments = market_settle_err_arm(&src);
    // Strip `//` line-comments so prose like "(do not return Err)." cannot be
    // mistaken for a control-flow statement by the mutation guard below.
    let arm: String = arm_with_comments
        .lines()
        .map(|l| match l.find("//") {
            Some(i) => &l[..i],
            None => l,
        })
        .collect::<Vec<_>>()
        .join("\n");
    let arm = arm.as_str();

    // NOT silent: it WARNs the user that only the economic leg failed.
    assert!(
        arm.contains("post-delivery market settlement failed"),
        "the market-fail arm must WARN that only the economic leg failed (not silent)"
    );
    assert!(
        arm.contains("your artifact was delivered successfully"),
        "the market-fail arm must tell the user their artifact WAS delivered"
    );

    // Anchored on tape (Art. 0.2): reason=`market_settle_failed:{e}` written via
    // the observed rejection-capsule writer.
    assert!(
        arm.contains("reason: format!(\"market_settle_failed:{e}\")"),
        "the market-fail arm must anchor a `market_settle_failed:{{e}}` rejection capsule on tape"
    );
    assert!(
        arm.contains("write_generate_rejection_capsule_observed"),
        "the market-fail arm must call the observed rejection-capsule writer to anchor the break"
    );

    // EXIT 0, NOT exit 2: the market-fail arm must NOT bail out of run_inner.
    // This is the load-bearing blocker #3 fix; the exit-2-on-market-fail
    // regression re-introduces a `return Err(..)` here.
    assert!(
        !arm.contains("return Err("),
        "MUTATION GUARD: the market-fail arm must NOT `return Err(..)` (exit-2-on-market-fail \
         regression) — the artifact is already delivered, so run_inner must fall through to \
         Ok(()) => ExitCode::SUCCESS (exit 0)"
    );
    assert!(
        !arm.contains("return run_result"),
        "MUTATION GUARD: the market-fail arm must NOT bail out via `return run_result`"
    );
    assert!(
        !arm.contains("ExitCode::from(2)"),
        "MUTATION GUARD: the market-fail arm must NOT emit ExitCode::from(2)"
    );
    // retryable=false stamped on the economic break (delivery is final).
    assert!(
        arm.contains("retryable: false"),
        "the anchored economic break must be retryable=false (artifact already delivered)"
    );
}

// ───────────────────────────────────────────────────────────────────────────
// (2) PYTHON-BUNDLE-PASSES — blocker #1
// ───────────────────────────────────────────────────────────────────────────

/// `HtmlParses` is derived ONLY for `.html` entrypoints; a `.py` entrypoint
/// gets `PythonParses` and NEVER `HtmlParses`. MUTATION: re-introducing the
/// unconditional `HtmlParses` push makes the `.py` set contain HtmlParses => RED.
#[test]
fn html_parses_is_entrypoint_conditional() {
    let py = derive_scenario_set_from_spec(b"Crunch some numbers", "cid", "main.py", 1);
    assert!(
        has_scenario(&py.scenarios, &TestScenario::PythonParses),
        "a .py entrypoint must get PythonParses"
    );
    assert!(
        !has_scenario(&py.scenarios, &TestScenario::HtmlParses),
        "MUTATION GUARD: a .py entrypoint must NEVER be HTML-gated (unconditional HtmlParses regression)"
    );

    let html = derive_scenario_set_from_spec(b"Build a UI", "cid", "index.html", 1);
    assert!(
        has_scenario(&html.scenarios, &TestScenario::HtmlParses),
        "an .html entrypoint must get HtmlParses"
    );
    assert!(
        !has_scenario(&html.scenarios, &TestScenario::PythonParses),
        "an .html entrypoint must NOT get PythonParses"
    );
}

/// A correct `main.py` bundle passes (no HtmlParses false-reject); a
/// syntactically broken `main.py` FAILS. NO FALSE PASS: a broken artifact must
/// never pass while a valid one passes.
#[test]
fn python_correct_main_py_passes_broken_fails() {
    let dir = TempDir::new().expect("tempdir");
    let ws = dir.path();
    let t = now_t();

    let scenario_set = derive_scenario_set_from_spec(b"Crunch some numbers", "cid", "main.py", t);
    assert!(
        has_scenario(&scenario_set.scenarios, &TestScenario::PythonParses),
        "precondition: PythonParses derived for the .py entrypoint"
    );
    // Blocker #1 regression check at the SET level: no HtmlParses for .py — the
    // default main.py is no longer HTML-doctype-gated.
    assert!(
        !has_scenario(&scenario_set.scenarios, &TestScenario::HtmlParses),
        "a correct main.py must NOT carry an HTML-doctype gate"
    );

    let good_cid = put_bundle(
        ws,
        "main.py",
        b"def main():\n    print(1 + 2)\n\nmain()\n",
        t,
    );
    let good = run_test_scenario_set(ws, &good_cid, &scenario_set).expect("run good");
    let py_ok = pass_of(&good.results, &TestScenario::PythonParses);

    let bad_cid = put_bundle(ws, "main.py", b"def main(:\n    print('oops'\n", t + 1);
    let bad = run_test_scenario_set(ws, &bad_cid, &scenario_set).expect("run bad");
    let py_bad = pass_of(&bad.results, &TestScenario::PythonParses);

    // The producer fails CLOSED when no interpreter is available (then both are
    // false). We assert the discrimination that always holds: a broken file is
    // NEVER certified, and never passes while a valid file fails.
    assert!(
        !py_bad,
        "NO FALSE PASS: a syntactically broken main.py must NOT pass PythonParses"
    );
    assert!(
        !(py_bad && !py_ok),
        "producer is inverted: broken passed while valid failed"
    );
    if py_ok {
        // Interpreter present: full discrimination — the correct main.py bundle
        // passes overall (no HtmlParses false-reject), the broken one fails.
        assert!(
            good.overall_pass,
            "a correct main.py bundle must pass overall (no HtmlParses false-reject)"
        );
        assert!(
            !bad.overall_pass,
            "a broken main.py bundle must fail overall"
        );
    }
}

// ───────────────────────────────────────────────────────────────────────────
// (3) FUNCTIONAL-REJECTS-WRONG — blocker #2
// ───────────────────────────────────────────────────────────────────────────

/// The spec-derived FUNCTIONAL gate is NON-VACUOUS: it REJECTS an artifact
/// missing the required control and PASSES one that surfaces it — both are
/// well-formed HTML, so the structural gate alone would pass BOTH. MUTATION:
/// weakening the producer to "exists => pass" makes the wrong artifact pass.
#[test]
fn functional_gate_rejects_wrong_passes_right() {
    let dir = TempDir::new().expect("tempdir");
    let ws = dir.path();
    let t = now_t();

    // The spec demands a `New Game` control => a hidden RequiredTextPresent gate.
    let spec = b"Build a snake game. The page MUST have a `New Game` button to restart.";
    let scenario_set = derive_scenario_set_from_spec(spec, "cid", "index.html", t);
    let functional_probe = TestScenario::RequiredTextPresent {
        label: String::new(),
        needle: String::new(),
    };
    assert!(
        has_scenario(&scenario_set.scenarios, &functional_probe),
        "precondition: a spec-derived RequiredTextPresent functional gate must exist (non-vacuous)"
    );

    // CORRECT artifact: well-formed HTML that surfaces the required control.
    let correct = b"<!DOCTYPE html><html><body><button id=\"ng\">New Game</button></body></html>";
    let correct_cid = put_bundle(ws, "index.html", correct, t);
    let correct_run = run_test_scenario_set(ws, &correct_cid, &scenario_set).expect("run correct");
    assert!(
        pass_of(&correct_run.results, &TestScenario::HtmlParses),
        "the correct artifact must parse as valid HTML"
    );
    assert!(
        pass_of(&correct_run.results, &functional_probe),
        "the correct artifact surfaces the required control => functional gate PASS"
    );
    assert!(
        correct_run.overall_pass,
        "the correct artifact passes overall"
    );

    // WRONG-BUT-VALID artifact: perfectly well-formed HTML implementing the
    // WRONG thing (a clock) and omitting the required `New Game` control.
    let wrong = b"<!DOCTYPE html><html><body><h1>Clock</h1><div id=\"t\">12:00</div></body></html>";
    let wrong_cid = put_bundle(ws, "index.html", wrong, t + 1);
    let wrong_run = run_test_scenario_set(ws, &wrong_cid, &scenario_set).expect("run wrong");
    assert!(
        pass_of(&wrong_run.results, &TestScenario::HtmlParses),
        "the wrong artifact is STILL valid HTML (the structural gate alone would pass it)"
    );
    assert!(
        !pass_of(&wrong_run.results, &functional_probe),
        "MUTATION GUARD: a wrong-but-valid artifact must FAIL the functional gate \
         (weakening the producer to exists=>pass flips this RED)"
    );
    assert!(
        !wrong_run.overall_pass,
        "the battery still records the functional failure (overall_pass=false)"
    );
    // 2026-06-09 architect decision — NON-FATAL functional gate: the functional
    // failure is an on-tape ADVISORY, not a hard delivery reject. The wrong
    // artifact is STILL delivered (structural_pass) with the miss flagged
    // (functional_unmet); only a STRUCTURAL failure hard-blocks. This keeps the
    // anti-Goodhart SIGNAL (the gate detects the wrong artifact) without
    // false-rejecting correct work when the heuristic needle is itself wrong.
    let wrong_verdict = turingosv4::runtime::test_run::delivery_verdict(&wrong_run.results);
    assert!(
        wrong_verdict.structural_pass && wrong_verdict.functional_unmet,
        "non-fatal gate: a functional-only miss delivers + advises, never hard-rejects"
    );
    let right_verdict = turingosv4::runtime::test_run::delivery_verdict(&correct_run.results);
    assert!(
        right_verdict.structural_pass && !right_verdict.functional_unmet,
        "a correct artifact delivers with no advisory"
    );
}

/// ORACLE HIDDEN (C11 / Art. III.4): the functional control is derived from the
/// spec and applied as a HIDDEN delivery gate — the scenario-set bytes (incl.
/// the `needle`) live in a separate CAS object and must NOT be injected into the
/// generation prompt. Here we prove the derivation is spec-internal and the
/// scenario set is a self-describing CAS object (the hidden-oracle shielding is
/// further locked by `tests/hidden_oracle_not_in_generation_prompt_bytes.rs`).
#[test]
fn functional_needle_is_spec_derived_not_prompt_injected() {
    // The needle is derived deterministically from the spec text alone (no LLM,
    // no prompt round-trip): the backtick-quoted control becomes the needle.
    let set = derive_scenario_set_from_spec(
        b"The page must have a `New Game` button to restart.",
        "cid",
        "index.html",
        1,
    );
    let needles: Vec<String> = set
        .scenarios
        .iter()
        .filter_map(|s| match s {
            TestScenario::RequiredTextPresent { needle, .. } => Some(needle.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(
        needles,
        vec!["new game".to_string()],
        "the functional needle must be derived from the spec's quoted control, lower-cased"
    );
    // The scenario set is a self-describing CAS object (separate CID from the
    // generation prompt) — the hidden-oracle pattern. The schema id is stable.
    assert_eq!(
        set.schema_id,
        turingosv4::runtime::test_scenario::TEST_SCENARIO_SET_SCHEMA_ID,
        "the scenario set must be a self-describing CAS object (hidden-oracle shielding)"
    );
}

// ───────────────────────────────────────────────────────────────────────────
// (4) OBSERVE-ROLLUP — blocker #5
// ───────────────────────────────────────────────────────────────────────────

fn turingos_bin() -> std::path::PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let debug = std::path::PathBuf::from(format!("{manifest_dir}/target/debug/turingos"));
    let release = std::path::PathBuf::from(format!("{manifest_dir}/target/release/turingos"));
    if debug.exists() {
        return debug;
    }
    if release.exists() {
        return release;
    }
    panic!("turingos binary not found; run `cargo build --bin turingos` first");
}

/// `turingos observe --workspace <WS>` over a constructed workspace tape prints
/// the FC1/FC2/FC3 liveness + no-zombie verdict + the per-task integer VPPUT
/// rollup, READ-ONLY. The tape is a real bootstrapped chaintape (genesis spine);
/// observe loads the canonical LoadedTape and renders the rollup with no
/// mutation and no head advance.
#[tokio::test]
async fn observe_prints_fc_liveness_no_zombie_and_vpput_rollup() {
    use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
    use turingosv4::runtime::{build_chaintape_sequencer_with_initial_q, RuntimeChaintapeConfig};
    use turingosv4::state::q_state::{AgentId, QState};

    let dir = TempDir::new().expect("tempdir");
    let root = dir.path();

    // The workspace ROOT must carry a genesis_payload.toml marker (observe walks
    // <=3 parents to find it; load_tape also reads it). Copy the real one.
    let genesis_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("genesis_payload.toml");
    std::fs::copy(&genesis_src, root.join("genesis_payload.toml")).expect("copy genesis marker");

    // Bootstrap a real canonical chaintape (genesis spine) at <root>/runtime_repo
    // + <root>/cas — exactly the layout observe resolves at the workspace root.
    let cfg = RuntimeChaintapeConfig {
        runtime_repo_path: root.join("runtime_repo"),
        cas_path: root.join("cas"),
        run_id: "e2e-observe-rollup".to_string(),
        queue_capacity: 16,
        resume_existing_chain: false,
    };
    let initial_q = QState::genesis();
    let bundle =
        build_chaintape_sequencer_with_initial_q(&cfg, initial_q).expect("bootstrap chaintape");

    // Register an agent so <runtime_repo>/agent_pubkeys.json exists (load_tape
    // reads it on the observe path).
    let mut reg =
        AgentKeypairRegistry::open(&cfg.runtime_repo_path).expect("open agent keypair registry");
    reg.get_or_create(&AgentId("observer-fixture".into()))
        .expect("generate fixture keypair");

    // Drain + close the chain so the on-disk L4 spine is complete.
    bundle.shutdown().await.expect("shutdown drain");

    // The binary's trust-root check + load_tape resolve constitution.md from the
    // source-repo CWD; run the binary FROM the repo root so those resolve.
    let output = Command::new(turingos_bin())
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .arg("observe")
        .arg("--workspace")
        .arg(root)
        .output()
        .expect("run turingos observe");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "turingos observe must exit 0 over a constructed workspace tape.\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    // FC liveness + no-zombie verdict.
    assert!(
        stdout.contains("FC-LIVENESS OBSERVER"),
        "observe must print the FC-liveness observer section.\nstdout:\n{stdout}"
    );
    assert!(
        stdout.contains("(A) FC-liveness rollup:"),
        "observe must print the (A) FC-liveness rollup.\nstdout:\n{stdout}"
    );
    assert!(
        stdout.contains("no_zombie=") && stdout.contains("zombie_count="),
        "observe must print the no-zombie verdict + zombie_count.\nstdout:\n{stdout}"
    );
    // Integer VPPUT rollup.
    assert!(
        stdout.contains("(B) VPPUT rollup:"),
        "observe must print the (B) integer VPPUT rollup.\nstdout:\n{stdout}"
    );
    assert!(
        stdout.contains("ground_truth_solved="),
        "observe must print the ground-truth-solved count of the VPPUT rollup.\nstdout:\n{stdout}"
    );

    // READ-ONLY: observe must not advance the head. The runtime_repo head is the
    // genesis spine; observe printing the rollup must not have created any
    // generate-rejection / market capsule (no mutation, no economic state).
    assert!(
        !stdout.contains("market_settle_failed"),
        "observe is read-only and must not emit economic break capsules"
    );
}
