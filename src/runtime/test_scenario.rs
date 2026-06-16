/// TRACE_MATRIX FC1 + FC3: TestScenario and TestScenarioSet schemas.
///
/// C11: Spec-derived test scenarios. Scenarios are hidden from the generation
/// prompt — the scenario set bytes MUST NOT appear in any generation prompt.
///
/// FC-trace: FC1 (test loop), FC3 (test evidence)
/// Risk class: Class 3
use serde::{Deserialize, Serialize};

/// TRACE_MATRIX FC3: Schema ID for TestScenarioSet.
pub const TEST_SCENARIO_SET_SCHEMA_ID: &str = "turingos-test-scenario-set-v1";

/// TRACE_MATRIX FC1: producer-bound test scenario variants.
///
/// Every variant here has a producer in `test_run::run_one_scenario`. The
/// serde tag is the on-CAS schema; adding a new variant is backward-compatible
/// for *reading* (old capsules never contain it) but new variants must keep
/// the same `#[serde(tag = "kind")]` snake_case discriminant convention so the
/// hidden-oracle CAS object stays self-describing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TestScenario {
    /// The entrypoint file (e.g., index.html) exists in the artifact bundle.
    EntrypointExists,
    /// The entrypoint HTML parses as valid HTML (doctype + html element present).
    /// Derived ONLY when the entrypoint is an `.html` file (web UI deliverable).
    HtmlParses,
    /// The Python entrypoint parses as syntactically valid Python 3.
    /// Derived ONLY when the entrypoint is a `.py` file (script deliverable).
    /// Producer shells `python3 -c "import ast; ast.parse(...)"`-style check;
    /// when no interpreter is available the producer fails CLOSED (the artifact
    /// is not certified, never silently passed).
    PythonParses,
    /// The sandbox policy header/meta is preserved in the artifact.
    SandboxPolicyPreserved { policy: String },
    /// A spec-derived FUNCTIONAL requirement: a required snippet of text/control
    /// (case-insensitive) MUST be present in the rendered entrypoint artifact.
    ///
    /// This is the anti-Goodhart functional gate: "delivered" must mean "meets
    /// the requirement", not merely "is well-formed HTML/Python". `label` is a
    /// human-readable name for the requirement; `needle` is the lowercased
    /// substring the rendered artifact must contain.
    ///
    /// C11 (Art.III.4): this scenario is derived from the spec and applied as a
    /// HIDDEN delivery gate — the scenario-set bytes (including `needle`) MUST
    /// NOT be injected into any generation LLM prompt.
    RequiredTextPresent { label: String, needle: String },
}

impl TestScenario {
    /// TRACE_MATRIX FC1 (C11 hidden-oracle, Art.III.4): shielded retry-feedback
    /// rendering — the single source that turns a failed scenario into the
    /// LLM-visible retry-feedback line on the FC1 tape-relay path.
    ///
    /// C11 hidden-oracle (Art.III.4): the SINGLE source for rendering a FAILED
    /// scenario into LLM-visible retry feedback. Returns a `(name, detail)`
    /// pair that is guaranteed needle-free AND label-free for the functional
    /// gate, while passing structural-scenario details (which carry no
    /// spec-derived oracle) through unchanged.
    ///
    /// `RequiredTextPresent` carries the spec-derived functional `needle`, and
    /// its `label` is the SAME spec token only cased differently — so echoing
    /// `label` (even inside the scenario *name*) leaks the hidden oracle into a
    /// generation prompt on retry just as much as echoing `needle` would
    /// (Goodhart). ALL prompt-bound feedback for a failed scenario MUST route
    /// through here; never format `label`/`needle`/`r.detail` for a functional
    /// gate at a call site.
    pub fn shielded_feedback(&self, recorded_detail: &str) -> (String, String) {
        match self {
            TestScenario::EntrypointExists => {
                ("EntrypointExists".to_string(), recorded_detail.to_string())
            }
            TestScenario::HtmlParses => ("HtmlParses".to_string(), recorded_detail.to_string()),
            TestScenario::PythonParses => ("PythonParses".to_string(), recorded_detail.to_string()),
            TestScenario::SandboxPolicyPreserved { .. } => (
                "SandboxPolicyPreserved".to_string(),
                recorded_detail.to_string(),
            ),
            TestScenario::RequiredTextPresent { .. } => (
                "RequiredTextPresent".to_string(),
                "a required UI control named in the spec is missing from the \
                 rendered entrypoint"
                    .to_string(),
            ),
        }
    }

    /// TRACE_MATRIX FC1: structural-vs-functional scenario classification (drives
    /// the non-fatal delivery gate via `TestRunCapsule::delivery_verdict`).
    ///
    /// Whether this scenario is a best-effort FUNCTIONAL check (vs a reliable
    /// STRUCTURAL one). `RequiredTextPresent` is the only functional scenario:
    /// its needle is heuristically derived from fuzzy LLM-synthesized spec prose
    /// and can be wrong, so per the 2026-06-09 architect decision a functional
    /// failure is NON-FATAL for delivery (deliver + on-tape advisory + warn),
    /// while every STRUCTURAL scenario (entrypoint exists / parses / sandbox
    /// policy) is a reliable hard delivery gate. Used by
    /// `TestRunCapsule::delivery_verdict`.
    pub fn is_functional(&self) -> bool {
        matches!(self, TestScenario::RequiredTextPresent { .. })
    }
}

/// TRACE_MATRIX FC3: CAS-anchored set of test scenarios derived from a spec.
///
/// Written as a separate CAS object (separate CID from the TestRunCapsule)
/// to enforce the hidden-oracle pattern: the scenario set bytes MUST NOT
/// appear inside any generation prompt.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TestScenarioSet {
    pub schema_id: String, // = TEST_SCENARIO_SET_SCHEMA_ID
    pub spec_capsule_cid: String,
    pub scenarios: Vec<TestScenario>,
    pub logical_t: u64,
}

/// TRACE_MATRIX FC3: Derive a minimal TestScenarioSet from spec bytes.
///
/// Derivation is now also entrypoint-aware (1.0 blocker #1 fix), reading the
/// resolved artifact entrypoint to pick the structural gate:
///   - Always: `EntrypointExists`.
///   - `.html` entrypoint  -> `HtmlParses` (doctype + `<html>`).
///   - `.py`   entrypoint  -> `PythonParses` (syntactic `ast.parse` check).
///     A correct `main.py` no longer fails an HTML-only gate.
///   - other / unknown ext -> structural gate skipped (EntrypointExists only).
///   - Adds `SandboxPolicyPreserved` if the spec mentions sandbox/CSP.
///   - Adds at least one spec-derived FUNCTIONAL `RequiredTextPresent` gate
///     (1.0 blocker #2 fix) so a wrong-but-well-formed artifact fails.
///
/// **The derived scenario set bytes MUST NOT be injected into any generation
/// prompt.** This is enforced by the hidden-oracle tests (C11 invariant); the
/// functional `needle` is derived here and never round-trips through the LLM.
pub fn derive_scenario_set_from_spec(
    spec_bytes: &[u8],
    spec_capsule_cid: &str,
    entrypoint: &str,
    logical_t: u64,
) -> TestScenarioSet {
    let spec_text = String::from_utf8_lossy(spec_bytes).to_string();
    let spec_lower = spec_text.to_ascii_lowercase();
    let entry_lower = entrypoint.to_ascii_lowercase();

    let mut scenarios = vec![TestScenario::EntrypointExists];

    // 1.0 blocker #1: structural gate is entrypoint-conditional. HtmlParses
    // applies ONLY to .html entrypoints; .py entrypoints get a syntactic
    // Python validity gate instead. A default `main.py` deliverable must NOT
    // be rejected by an HTML doctype check.
    if entry_lower.ends_with(".html") || entry_lower.ends_with(".htm") {
        scenarios.push(TestScenario::HtmlParses);
    } else if entry_lower.ends_with(".py") {
        scenarios.push(TestScenario::PythonParses);
    }

    // Add sandbox scenario if spec mentions content security / sandbox
    if spec_lower.contains("sandbox")
        || spec_lower.contains("csp")
        || spec_lower.contains("content-security")
    {
        scenarios.push(TestScenario::SandboxPolicyPreserved {
            policy: "sandbox".to_string(),
        });
    }

    // 1.0 blocker #2: at least one spec-derived FUNCTIONAL gate. Extract a
    // salient required token from the spec; the rendered artifact must contain
    // it. This is hidden from the generation prompt (C11).
    if let Some((label, needle)) = derive_required_text(&spec_text) {
        scenarios.push(TestScenario::RequiredTextPresent { label, needle });
    }

    TestScenarioSet {
        schema_id: TEST_SCENARIO_SET_SCHEMA_ID.to_string(),
        spec_capsule_cid: spec_capsule_cid.to_string(),
        scenarios,
        logical_t,
    }
}

/// TRACE_MATRIX FC3 (C11 hidden oracle): derive a single FUNCTIONAL required
/// token from the spec. Returns `(label, lowercased_needle)`.
///
/// Strategy (deterministic, no LLM): scan the spec BODY for the strongest signal
/// of a concrete, user-visible requirement and lower-case it for substring match.
///   1. A backtick-quoted token (`` `New Game` ``) — usually a control/label.
///   2. A double-quoted token ("Submit").
/// Falls back to `None` only when the spec has no extractable salient token,
/// in which case the run keeps the structural gates (still better than the
/// structural-only behavior, never worse).
///
/// **Boilerplate guard (1.0 fix):** the generated `spec.md` is wrapped by
/// `runtime::spec_synthesis::wrap_spec_md`, which prepends an attribution header
///   `> Generated by ` + backtick + `turingos spec` + backtick + ` — meta model: ...`
/// and appends a `## Appendix — Raw Q/A` section. Scanning the WHOLE document
/// made the first backtick token ALWAYS be the tool name `turingos spec`, turning
/// every functional gate into a false "must print the literal `turingos spec`"
/// requirement that a correct app would never satisfy. We therefore derive the
/// needle from the spec BODY only (attribution blockquote + appendix stripped)
/// and additionally reject non-functional tokens — harness/usage commands
/// (`turingos spec`, `python3 main.py`) and filename/path tokens
/// (`index.html`, `main.py`) — so the needle is a real user-visible control,
/// not harness boilerplate, a usage example, or a bundle filename. See
/// `is_non_functional_token`.
///
/// The returned needle is intentionally a *small* required fragment, not the
/// whole spec — the artifact must genuinely surface that requirement, but a
/// faithful implementation that uses a synonym is not unfairly failed by an
/// over-specific oracle. The token is chosen to be load-bearing (a control name
/// or quoted requirement), which a wrong-but-valid artifact would omit.
fn derive_required_text(spec_text: &str) -> Option<(String, String)> {
    let body = spec_body_for_needle(spec_text);
    // 1. backtick-quoted token (skip non-functional tokens: harness/usage
    //    commands like `turingos spec` / `python3 main.py`, and filename/path
    //    tokens like `index.html` — see `is_non_functional_token`).
    if let Some(tok) = first_delimited_filtered(&body, '`', '`', is_non_functional_token) {
        return Some((
            format!("required control `{tok}`"),
            tok.to_ascii_lowercase(),
        ));
    }
    // 2. double-quoted token.
    if let Some(tok) = first_delimited_filtered(&body, '"', '"', is_non_functional_token) {
        return Some((format!("required text \"{tok}\""), tok.to_ascii_lowercase()));
    }
    None
}

/// A derived-needle reject predicate. A FUNCTIONAL needle must be a real
/// USER-VISIBLE requirement (a control label, an output string) that a correct
/// artifact actually surfaces in its rendered body. This rejects three classes
/// of token that a correct artifact would NEVER print, so the functional gate
/// never false-fails correct work:
///   1. harness CLI invocations — `turingos spec` / `turingos generate`;
///   2. usage/command examples for any interpreter — `python3 main.py`,
///      `node app.js`, `./run.sh`, `cargo run`, `npm start`, ...;
///   3. filename / path tokens — `index.html`, `main.py`, `data.csv`. A file
///      name is a bundle/structural concern already covered by
///      `EntrypointExists`; a correct app never prints its own filename.
///
/// 1.0 E2E defect (the reason classes 2+3 exist): with two real specs the
/// `--entrypoint` filename (`index.html`) and a spec usage example
/// (`python3 main.py`) were each latched as the RequiredTextPresent needle,
/// false-failing a correct Snake game and a correct CSV-stats script even
/// though their real controls (`New Game`, `Average:`) were present.
fn is_non_functional_token(tok: &str) -> bool {
    let t = tok.trim().to_ascii_lowercase();
    if t.is_empty() {
        return true;
    }
    // (1)+(2) command / usage invocations — a usage example, not a control.
    const CMD_PREFIXES: [&str; 14] = [
        "turingos ",
        "python3 ",
        "python ",
        "node ",
        "npm ",
        "npx ",
        "bash ",
        "sh ",
        "cargo ",
        "pip ",
        "pip3 ",
        "deno ",
        "ruby ",
        "./",
    ];
    if t == "turingos" || CMD_PREFIXES.iter().any(|p| t.starts_with(p)) {
        return true;
    }
    // (3) filename / path tokens — any whitespace-separated word ending in a
    // known source / asset / data extension.
    const FILE_EXTS: [&str; 22] = [
        ".html", ".htm", ".py", ".js", ".ts", ".jsx", ".tsx", ".css", ".json", ".md", ".txt",
        ".csv", ".toml", ".yaml", ".yml", ".xml", ".svg", ".png", ".jpg", ".rs", ".sh", ".lean",
    ];
    if t.split_whitespace()
        .any(|w| FILE_EXTS.iter().any(|ext| w.ends_with(ext)))
    {
        return true;
    }
    // (4) data / value tokens — a functional control or output LABEL is
    // alphabetic ("New Game", "Average:", "Count:"); a token carrying a numeric
    // value or pure data ("10, 20, 30", "Average: 20.0", "0", "5, hello, 15")
    // is a narrative example, not a stable required control a correct artifact
    // must surface. Require at least one letter AND reject any digit-bearing
    // token. (1.0 E2E: a Meta-LLM Given/When/Then example lifted `10, 20, 30`
    // and `Average: 20.0` as needles, false-failing a correct CSV-stats script
    // that — correctly — never echoes its raw input values.)
    let has_alpha = t.chars().any(|c| c.is_alphabetic());
    let has_digit = t.chars().any(|c| c.is_ascii_digit());
    !has_alpha || has_digit
}

/// Return the spec BODY with the `wrap_spec_md` attribution header and the
/// `## Appendix — Raw Q/A` section removed, so the functional needle is derived
/// from the real requirement prose, not harness boilerplate. If the spec is not
/// `wrap_spec_md`-shaped, returns it unchanged.
fn spec_body_for_needle(spec_text: &str) -> String {
    let mut out: Vec<&str> = Vec::new();
    for line in spec_text.lines() {
        let trimmed = line.trim_start();
        // Drop the attribution blockquote line(s) (`> Generated by `turingos spec` …`).
        if trimmed.starts_with('>') {
            continue;
        }
        // Stop at the audit appendix — everything below is raw Q/A boilerplate.
        if trimmed.starts_with("## Appendix") {
            break;
        }
        out.push(line);
    }
    out.join("\n")
}

/// Return the first non-empty token delimited by `open`..`close`, trimmed,
/// with length in 2..=60 (avoid empty fences and runaway captures). ASCII and
/// non-ASCII both accepted; the needle is matched case-insensitively downstream.
#[cfg(test)]
fn first_delimited(text: &str, open: char, close: char) -> Option<String> {
    first_delimited_filtered(text, open, close, |_| false)
}

/// Like `first_delimited`, but skips any token for which `reject(tok)` is true
/// (used to skip harness CLI-command tokens such as `turingos spec`). Returns the
/// FIRST accepted token (the first one with `reject==false`).
fn first_delimited_filtered(
    text: &str,
    open: char,
    close: char,
    reject: impl Fn(&str) -> bool,
) -> Option<String> {
    let bytes: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == open {
            let start = i + 1;
            let mut j = start;
            while j < bytes.len() && bytes[j] != close {
                j += 1;
            }
            if j < bytes.len() {
                let tok: String = bytes[start..j].iter().collect();
                let tok = tok.trim().to_string();
                let n = tok.chars().count();
                if (2..=60).contains(&n) && !tok.contains('\n') && !reject(&tok) {
                    return Some(tok);
                }
                i = j + 1;
                continue;
            }
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario_serialization_roundtrip() {
        let s = TestScenario::EntrypointExists;
        let json = serde_json::to_string(&s).expect("serialize");
        let back: TestScenario = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(s, back);
    }

    #[test]
    fn test_sandbox_scenario_only_when_spec_mentions_sandbox() {
        let no_sandbox =
            derive_scenario_set_from_spec(b"Build a todo list", "cid1", "index.html", 1000);
        assert!(!no_sandbox
            .scenarios
            .iter()
            .any(|s| matches!(s, TestScenario::SandboxPolicyPreserved { .. })));

        let with_sandbox = derive_scenario_set_from_spec(
            b"Build a todo list with sandbox policy",
            "cid2",
            "index.html",
            1001,
        );
        assert!(with_sandbox
            .scenarios
            .iter()
            .any(|s| matches!(s, TestScenario::SandboxPolicyPreserved { .. })));
    }

    #[test]
    fn test_set_has_correct_schema_id() {
        let set = derive_scenario_set_from_spec(b"any spec", "cid", "index.html", 1000);
        assert_eq!(set.schema_id, TEST_SCENARIO_SET_SCHEMA_ID);
    }

    /// 1.0 blocker #1: HtmlParses applies ONLY to .html entrypoints; a Python
    /// entrypoint gets PythonParses and NEVER HtmlParses.
    #[test]
    fn html_parses_only_for_html_entrypoint() {
        let html = derive_scenario_set_from_spec(b"Build a UI", "cid", "index.html", 1);
        assert!(html.scenarios.contains(&TestScenario::HtmlParses));
        assert!(!html.scenarios.contains(&TestScenario::PythonParses));

        let py = derive_scenario_set_from_spec(b"Crunch some numbers", "cid", "main.py", 1);
        assert!(py.scenarios.contains(&TestScenario::PythonParses));
        assert!(
            !py.scenarios.contains(&TestScenario::HtmlParses),
            "a Python deliverable must NOT be gated on HTML doctype"
        );
    }

    /// 1.0 blocker #2: a spec with a salient quoted control yields a FUNCTIONAL
    /// RequiredTextPresent gate.
    #[test]
    fn functional_gate_derived_from_quoted_control() {
        let set = derive_scenario_set_from_spec(
            b"The page must have a `New Game` button to restart.",
            "cid",
            "index.html",
            1,
        );
        let functional: Vec<_> = set
            .scenarios
            .iter()
            .filter_map(|s| match s {
                TestScenario::RequiredTextPresent { needle, .. } => Some(needle.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(functional, vec!["new game".to_string()]);
    }

    /// 1.0 E2E regression: the functional needle must NOT be the entrypoint
    /// FILENAME or a usage/command example. With two real synthesized specs the
    /// derivation latched onto `index.html` and `python3 main.py`, false-failing
    /// a correct Snake game and a correct CSV-stats script even though their real
    /// controls (`New Game` / `Average:`) were present. After the
    /// `is_non_functional_token` fix the needle must skip those and pick the
    /// real control; a spec with ONLY non-functional tokens yields NO functional
    /// gate (safe structural-only) rather than a false-failing one.
    #[test]
    fn functional_needle_skips_filename_and_usage_tokens() {
        fn needle_of(spec: &str, entry: &str) -> Option<String> {
            derive_scenario_set_from_spec(spec.as_bytes(), "cid", entry, 1)
                .scenarios
                .iter()
                .find_map(|s| match s {
                    TestScenario::RequiredTextPresent { needle, .. } => Some(needle.clone()),
                    _ => None,
                })
        }

        // webgame: the entrypoint filename `index.html` (backtick) precedes the
        // real "New Game" control. The needle must be the control, not the file.
        let web = "# Spec\n\nSave the page as `index.html`. It MUST have a \"New Game\" button.";
        assert_eq!(
            needle_of(web, "index.html"),
            Some("new game".to_string()),
            "needle must be the control, not the entrypoint filename"
        );

        // pyscript: a usage example `python3 main.py data.csv` precedes the real
        // "Average:" output requirement. The needle must be the output, not cmd.
        let py =
            "# Spec\n\nRun as `python3 main.py data.csv`. It MUST print \"Average:\" then the mean.";
        assert_eq!(
            needle_of(py, "main.py"),
            Some("average:".to_string()),
            "needle must be the output requirement, not the usage command"
        );

        // ONLY non-functional tokens -> NO functional gate (safe), not a false one.
        let only_files = "# Spec\n\nProduce `index.html` and `style.css`.";
        assert_eq!(
            needle_of(only_files, "index.html"),
            None,
            "filename-only spec must not yield a false functional needle"
        );

        // 1.0 E2E round-2 regression: a Given/When/Then example lifts raw input
        // data (`10, 20, 30`) and a computed value (`Average: 20.0`) as the first
        // backtick tokens. Both are value/data tokens (digit-bearing) and must be
        // rejected; the needle must be the clean output LABEL "Count:".
        let gwt = "# Spec\n\nGiven a CSV `data.csv` with the numbers `10, 20, 30`, \
                   the output shows `Average: 20.0`. It MUST print a \"Count:\" line \
                   and an \"Average:\" line.";
        assert_eq!(
            needle_of(gwt, "main.py"),
            Some("count:".to_string()),
            "needle must be a clean output label, not raw input data or a computed value"
        );

        // A digit-bearing or no-letter token is never a functional needle.
        assert!(is_non_functional_token("10, 20, 30"));
        assert!(is_non_functional_token("Average: 20.0"));
        assert!(is_non_functional_token("0"));
        assert!(!is_non_functional_token("New Game"));
        assert!(!is_non_functional_token("Average:"));
    }

    /// 1.0 fix: the `wrap_spec_md` attribution header
    /// (`> Generated by `turingos spec` — meta model: ...`) must NOT become the
    /// functional needle. The derived control must come from the spec BODY (the
    /// real requirement), never the harness CLI-command boilerplate.
    #[test]
    fn functional_needle_skips_wrap_spec_md_boilerplate() {
        // Faithful reproduction of a `wrap_spec_md`-shaped spec.md: attribution
        // blockquote (with the `turingos spec` backtick token) + body + appendix.
        let spec = "# TuringOS Spec (Phase 6.3)\n\n\
                    > Generated by `turingos spec` — meta model: `deepseek-v4-flash`\n\n\
                    # Launch Plan Decision Matrix\n\n\
                    Build a page with a `Recommended` plan callout.\n\n\
                    ---\n\n\
                    ## Appendix — Raw Q/A (for audit)\n\n\
                    **Q1**: `ignored token in appendix`\n\n";
        let set = derive_scenario_set_from_spec(spec.as_bytes(), "cid", "index.html", 1);
        let functional: Vec<_> = set
            .scenarios
            .iter()
            .filter_map(|s| match s {
                TestScenario::RequiredTextPresent { needle, .. } => Some(needle.clone()),
                _ => None,
            })
            .collect();
        // The needle is the BODY control `Recommended`, NOT `turingos spec`
        // (boilerplate) and NOT the appendix token.
        assert_eq!(
            functional,
            vec!["recommended".to_string()],
            "needle must derive from the spec body control, skipping `turingos spec` boilerplate + appendix"
        );
    }

    #[test]
    fn first_delimited_skips_empty_and_caps_length() {
        // empty fence skipped; first valid token returned.
        assert_eq!(
            first_delimited("`` then `Save`", '`', '`'),
            Some("Save".to_string())
        );
        // over-long token rejected.
        let long = format!("`{}`", "x".repeat(80));
        assert_eq!(first_delimited(&long, '`', '`'), None);
    }
}
