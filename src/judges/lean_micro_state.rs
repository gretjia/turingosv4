use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

const MAX_PUBLIC_FEEDBACK_CHARS: usize = 160;
const STATE_ID_PREFIX: &str = "lean-state-sha256:";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GoalState {
    pub theorem_id: String,
    pub state_id: String,
    pub parent_state_id: Option<String>,
    pub goals: Vec<GoalView>,
    pub imports_hash: String,
    pub preamble_hash: String,
    pub lean_version: String,
    pub mathlib_rev: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GoalView {
    pub goal_index: u32,
    pub case_label: Option<String>,
    pub hypotheses: Vec<String>,
    pub target: String,
    pub mvar_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TacticAttempt {
    pub attempt_id: String,
    pub parent_state_id: String,
    pub tactic: String,
    pub timeout_ms: u64,
    pub input_goal_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LeanStepOutcome {
    Advanced {
        next: GoalState,
    },
    Complete {
        proof_script: String,
    },
    Failed {
        class: LeanStepError,
        feedback: String,
    },
    Timeout,
    Rejected {
        class: CleanlinessReject,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LeanStepError {
    Parse,
    Elaborate,
    Tactic,
    Kernel,
    Transport,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CleanlinessReject {
    KernelBypassToken,
    RawStderr,
    UnpinnedVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofArtifact {
    pub theorem_id: String,
    pub proof_script: String,
    pub assembled_source_cid: String,
    pub final_lean_result_cid: String,
    pub axiom_report: AxiomCleanliness,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AxiomCleanliness {
    pub checked_by_print_axioms: bool,
    pub axioms: Vec<String>,
    pub whitelist: Vec<String>,
    pub clean: bool,
}

#[derive(Serialize)]
struct GoalStateIdPayload<'a> {
    theorem_id: &'a str,
    parent_state_id: Option<&'a str>,
    goals: &'a [GoalView],
    imports_hash: &'a str,
    preamble_hash: &'a str,
    lean_version: &'a str,
    mathlib_rev: Option<&'a str>,
}

pub fn deterministic_state_id(state: &GoalState) -> String {
    let payload = GoalStateIdPayload {
        theorem_id: &state.theorem_id,
        parent_state_id: state.parent_state_id.as_deref(),
        goals: &state.goals,
        imports_hash: &state.imports_hash,
        preamble_hash: &state.preamble_hash,
        lean_version: &state.lean_version,
        mathlib_rev: state.mathlib_rev.as_deref(),
    };
    let bytes = serde_json::to_vec(&payload).expect("goal-state id payload serializes");
    let mut h = Sha256::new();
    h.update(bytes);
    let digest = h
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>();
    format!("{STATE_ID_PREFIX}{digest}")
}

#[derive(Debug, Clone)]
pub struct LeanFixtureStepper {
    states: BTreeMap<String, GoalState>,
}

impl LeanFixtureStepper {
    pub fn new(mut initial: GoalState) -> Self {
        initial.state_id = deterministic_state_id(&initial);
        let mut states = BTreeMap::new();
        states.insert(initial.state_id.clone(), initial);
        Self { states }
    }

    pub fn step(&mut self, attempt: TacticAttempt) -> LeanStepOutcome {
        let Some(parent) = self.states.get(&attempt.parent_state_id).cloned() else {
            return LeanStepOutcome::Failed {
                class: LeanStepError::Transport,
                feedback: public_feedback("parent proof state is unavailable"),
            };
        };

        let tactic = attempt.tactic.trim();
        if tactic.starts_with("intro") {
            return self.advance_intro(parent, attempt);
        }
        if tactic == "simp" || tactic.starts_with("simp ") {
            return LeanStepOutcome::Complete {
                proof_script: format!("by\n  {tactic}"),
            };
        }

        LeanStepOutcome::Failed {
            class: LeanStepError::Tactic,
            feedback: public_feedback("tactic did not advance this fixture proof state"),
        }
    }

    pub fn backtrack(&self, state_id: &str) -> Option<GoalState> {
        self.states.get(state_id).cloned()
    }

    fn advance_intro(&mut self, parent: GoalState, _attempt: TacticAttempt) -> LeanStepOutcome {
        let mut next = parent.clone();
        let Some(first_goal) = next.goals.first_mut() else {
            return LeanStepOutcome::Failed {
                class: LeanStepError::Tactic,
                feedback: public_feedback("intro has no active goal"),
            };
        };
        let Some((_, rhs)) = first_goal.target.split_once("->") else {
            return LeanStepOutcome::Failed {
                class: LeanStepError::Tactic,
                feedback: public_feedback("intro requires an implication goal"),
            };
        };
        next.parent_state_id = Some(parent.state_id);
        first_goal.target = rhs.trim().to_string();
        next.state_id = deterministic_state_id(&next);
        self.states.insert(next.state_id.clone(), next.clone());
        LeanStepOutcome::Advanced { next }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LeanRecertError {
    FinalJudgeRejected,
    AxiomReportUnchecked,
    AxiomReportUnclean,
}

impl std::fmt::Display for LeanRecertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FinalJudgeRejected => write!(f, "final Lean judge rejected proof candidate"),
            Self::AxiomReportUnchecked => write!(f, "axiom report was not checked"),
            Self::AxiomReportUnclean => write!(f, "axiom report was not clean"),
        }
    }
}

impl std::error::Error for LeanRecertError {}

pub fn recertify_complete(
    theorem_id: impl Into<String>,
    proof_script: impl Into<String>,
    judge_verified: bool,
    axiom_report: AxiomCleanliness,
) -> Result<ProofArtifact, LeanRecertError> {
    if !judge_verified {
        return Err(LeanRecertError::FinalJudgeRejected);
    }
    if !axiom_report.checked_by_print_axioms {
        return Err(LeanRecertError::AxiomReportUnchecked);
    }
    if !axiom_report.clean || has_unwhitelisted_axiom(&axiom_report) {
        return Err(LeanRecertError::AxiomReportUnclean);
    }

    let theorem_id = theorem_id.into();
    let proof_script = proof_script.into();
    let assembled_hash = stable_hash(&format!("{theorem_id}\n{proof_script}"));
    let result_hash = stable_hash(&format!("{assembled_hash}\n{:?}", axiom_report.axioms));
    Ok(ProofArtifact {
        theorem_id,
        proof_script,
        assembled_source_cid: format!("tc-assembled-{assembled_hash:016x}"),
        final_lean_result_cid: format!("tc-lean-result-{result_hash:016x}"),
        axiom_report,
    })
}

fn has_unwhitelisted_axiom(report: &AxiomCleanliness) -> bool {
    report
        .axioms
        .iter()
        .any(|axiom| !report.whitelist.iter().any(|allowed| allowed == axiom))
}

fn public_feedback(summary: &str) -> String {
    let scrubbed = summary
        .replace("stderr", "diagnostic")
        .replace("STDERR", "diagnostic");
    scrubbed.chars().take(MAX_PUBLIC_FEEDBACK_CHARS).collect()
}

fn stable_hash(input: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
