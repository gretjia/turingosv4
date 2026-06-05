use serde::{Deserialize, Serialize};

use crate::judges::lean_micro_state::{GoalState, LeanStepError};

const MAX_SUMMARY_CHARS: usize = 240;

#[derive(Debug, Clone)]
pub struct TcAgentViewRequest {
    pub state: GoalState,
    pub theorem_id_is_public: bool,
    pub feedback_class: Option<LeanStepError>,
    pub feedback_summary: Option<String>,
    pub hidden_theorem_body: Option<String>,
    pub full_theorem_bank: Option<String>,
    pub private_diagnostic: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TcAgentPromptView {
    pub state_id: String,
    pub theorem_id: Option<String>,
    pub goals: Vec<TcAgentGoalView>,
    pub feedback: Option<TcAgentFeedbackView>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TcAgentGoalView {
    pub goal_index: u32,
    pub case_label: Option<String>,
    pub public_hypotheses_summary: String,
    pub public_target_summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TcAgentFeedbackView {
    pub class: String,
    pub public_summary: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcAgentViewError {
    ForbiddenPromptContent,
    UnboundedSummary,
}

impl std::fmt::Display for TcAgentViewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ForbiddenPromptContent => write!(f, "TC prompt view contains forbidden content"),
            Self::UnboundedSummary => write!(f, "TC prompt view summary is unbounded"),
        }
    }
}

impl std::error::Error for TcAgentViewError {}

pub fn render_tc_goal_view(
    request: TcAgentViewRequest,
) -> Result<TcAgentPromptView, TcAgentViewError> {
    let feedback = match (request.feedback_class, request.feedback_summary) {
        (Some(class), Some(summary)) => Some(TcAgentFeedbackView {
            class: lean_step_error_label(class),
            public_summary: bounded_public_summary(&summary)?,
        }),
        _ => None,
    };
    let view = TcAgentPromptView {
        state_id: request.state.state_id,
        theorem_id: if request.theorem_id_is_public {
            Some(request.state.theorem_id)
        } else {
            None
        },
        goals: request
            .state
            .goals
            .into_iter()
            .map(|goal| TcAgentGoalView {
                goal_index: goal.goal_index,
                case_label: goal.case_label,
                public_hypotheses_summary: format!("{} hypotheses", goal.hypotheses.len()),
                public_target_summary: bounded_target_summary(&goal.target),
            })
            .collect(),
        feedback,
    };
    let prompt = serde_json::to_string(&view).map_err(|_| TcAgentViewError::UnboundedSummary)?;
    guard_tc_prompt(&prompt)?;
    Ok(view)
}

pub fn guard_tc_prompt(prompt: &str) -> Result<(), TcAgentViewError> {
    let normalized = prompt.to_ascii_lowercase();
    for marker in forbidden_prompt_markers() {
        if normalized.contains(&marker.to_ascii_lowercase()) {
            return Err(TcAgentViewError::ForbiddenPromptContent);
        }
    }
    Ok(())
}

fn bounded_public_summary(summary: &str) -> Result<String, TcAgentViewError> {
    guard_tc_prompt(summary)?;
    if summary.chars().count() > MAX_SUMMARY_CHARS {
        return Err(TcAgentViewError::UnboundedSummary);
    }
    Ok(summary.to_string())
}

fn bounded_target_summary(target: &str) -> String {
    if target.chars().count() <= MAX_SUMMARY_CHARS {
        return target.to_string();
    }
    target.chars().take(MAX_SUMMARY_CHARS).collect()
}

fn forbidden_prompt_markers() -> Vec<String> {
    vec![
        ["raw", " std", "err"].concat(),
        ["lean", " std", "err"].concat(),
        "hidden theorem body".to_string(),
        "private diagnostic".to_string(),
        "full_theorem_bank".to_string(),
    ]
}

fn lean_step_error_label(class: LeanStepError) -> String {
    match class {
        LeanStepError::Parse => "parse",
        LeanStepError::Elaborate => "elaborate",
        LeanStepError::Tactic => "tactic",
        LeanStepError::Kernel => "kernel",
        LeanStepError::Transport => "transport",
    }
    .to_string()
}
