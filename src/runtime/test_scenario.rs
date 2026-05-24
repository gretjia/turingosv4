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
/// Trimmed to only the variants that have producers in v1.
/// NOTE: FUTURE variants (RequiredTextPresent, RequiredControlPresent, etc.)
/// are NOT reserved in this schema — they will require a schema_id bump.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TestScenario {
    /// The entrypoint file (e.g., index.html) exists in the artifact bundle.
    EntrypointExists,
    /// The entrypoint HTML parses as valid HTML (doctype + html element present).
    HtmlParses,
    /// The sandbox policy header/meta is preserved in the artifact.
    SandboxPolicyPreserved { policy: String },
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
/// Always includes EntrypointExists + HtmlParses.
/// Adds SandboxPolicyPreserved if the spec mentions "sandbox" or "csp".
///
/// **The derived scenario set bytes MUST NOT be injected into any generation
/// prompt.** This is enforced by the hidden-oracle tests (C11 invariant).
pub fn derive_scenario_set_from_spec(
    spec_bytes: &[u8],
    spec_capsule_cid: &str,
    logical_t: u64,
) -> TestScenarioSet {
    let spec_lower = String::from_utf8_lossy(spec_bytes).to_ascii_lowercase();
    let mut scenarios = vec![TestScenario::EntrypointExists, TestScenario::HtmlParses];

    // Add sandbox scenario if spec mentions content security / sandbox
    if spec_lower.contains("sandbox")
        || spec_lower.contains("csp")
        || spec_lower.contains("content-security")
    {
        scenarios.push(TestScenario::SandboxPolicyPreserved {
            policy: "sandbox".to_string(),
        });
    }

    TestScenarioSet {
        schema_id: TEST_SCENARIO_SET_SCHEMA_ID.to_string(),
        spec_capsule_cid: spec_capsule_cid.to_string(),
        scenarios,
        logical_t,
    }
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
        let no_sandbox = derive_scenario_set_from_spec(b"Build a todo list", "cid1", 1000);
        assert_eq!(no_sandbox.scenarios.len(), 2);
        assert!(!no_sandbox
            .scenarios
            .iter()
            .any(|s| matches!(s, TestScenario::SandboxPolicyPreserved { .. })));

        let with_sandbox =
            derive_scenario_set_from_spec(b"Build a todo list with sandbox policy", "cid2", 1001);
        assert_eq!(with_sandbox.scenarios.len(), 3);
        assert!(with_sandbox
            .scenarios
            .iter()
            .any(|s| matches!(s, TestScenario::SandboxPolicyPreserved { .. })));
    }

    #[test]
    fn test_set_has_correct_schema_id() {
        let set = derive_scenario_set_from_spec(b"any spec", "cid", 1000);
        assert_eq!(set.schema_id, TEST_SCENARIO_SET_SCHEMA_ID);
    }
}
