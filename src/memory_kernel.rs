//! TRACE_MATRIX FC1a-Q_t + FC1b-Q_{t+1}: TDMA-Bounded memory kernel scaffold.
//!
//! This module is the keystone state machine for TDMA-Bounded retry/escalation.
//! Atom 2 lands only the entry-point `step_forward` skeleton and the
//! Proceed/Retry/Invalid routing match per directive §5.1. The full
//! `handle_rejection` 8-step body, `assemble_o1_prompt`, and `escalate` are
//! deferred to Atom 7. Until Atom 7 lands, those code paths return `todo!()`.
//!
//! Per directive §15 KILL-tdma-1 + KILL-tdma-6: this module MUST NEVER contain
//! `raw_stderr` strings flowing into a prompt, and MUST NEVER inject
//! `constitution.md` bytes. Grep guards enforce both at the charter level.
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use crate::ledger::{
    AttemptScope, CommitRequest, ImmutableTapeLedger, NodeKind, RetryBeliefState,
};
use crate::state_update::{parse_prefix_json, StateStatus, StateUpdate};

// ── Public types ─────────────────────────────────────────────────

/// Worker task descriptor (directive §5.1).
/// TRACE_MATRIX FC1a-task_t: One unit of work fed to the kernel.
#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub prompt: String,
}

/// One round-trip result from the worker LLM + environment (directive §5.1).
/// TRACE_MATRIX FC1a-Agent_delta: Encapsulates the externalized signal coming
/// back from a worker — raw output, raw stderr (NEVER passed into prompt),
/// and overall success/failure.
#[derive(Debug, Clone)]
pub struct EnvironmentResult {
    pub raw_output: String,
    pub raw_stderr: String,
    pub success: bool,
}

impl EnvironmentResult {
    /// TRACE_MATRIX FC1a-Agent_delta: Predicate accessor — does the environment
    /// (LLM driver + Lean/Judge runner) report this attempt as overall successful?
    pub fn is_success(&self) -> bool {
        self.success
    }
}

/// Step decision returned by the kernel (directive §5.1).
/// TRACE_MATRIX FC1b-Q_{t+1}: Discriminates the next kernel transition.
#[derive(Debug, Clone)]
pub enum KernelStep {
    /// verified_head advanced; the worker should move to the next task.
    Proceed,
    /// Retry with the rebuilt O(1) prompt; assembled per directive §11 in Atom 7.
    Retry { prompt: String },
    /// Terminal escalation; commit chain frozen at verified_head.
    Escalate {
        reason: String,
        evidence_hash: String,
    },
}

// ── Kernel ───────────────────────────────────────────────────────

/// TDMA-Bounded memory kernel.
/// TRACE_MATRIX FC1a-rtool + FC1b-wtool: The single object that ties the tape
/// (ImmutableTapeLedger), the distiller (Atom 4), the rtool (Atom 6), and the
/// CharterCore (Atom 5) into one FC1 runtime loop.
///
/// Atom 2 scaffold: holds only the tape reference and the `step_forward`
/// entry-point. Atom 7 will add `distiller`, `rtool`, `charter`, `tokenizer`,
/// and the full `handle_rejection` body.
pub struct MemoryKernel<L: ImmutableTapeLedger> {
    pub tape: L,
    pub run_id: String,
}

impl<L: ImmutableTapeLedger> MemoryKernel<L> {
    /// TRACE_MATRIX FC2-Q_0: Boot a kernel against a tape ledger and a run-id.
    pub fn new(tape: L, run_id: impl Into<String>) -> Self {
        Self {
            tape,
            run_id: run_id.into(),
        }
    }

    /// FC1 runtime loop entry-point (directive §5.1).
    ///
    /// Routing matrix (three branches):
    ///   (Ok(header), true) + status==Proceed → commit StateAccepted, advance
    ///       verified_head, return KernelStep::Proceed.
    ///   (Ok(header), _)                      → call handle_rejection (Atom 7).
    ///   (Err(parse_error), _)                → synthesize Invalid header,
    ///                                          call handle_rejection (Atom 7).
    ///
    /// TRACE_MATRIX FC1a-rtool + FC1a-output_edge + FC1b-wtool.
    pub fn step_forward(
        &mut self,
        task: &Task,
        env_result: EnvironmentResult,
    ) -> KernelStep {
        // Scan budget and header budget are bound to directive constants;
        // until Atom 3 lands the real Tokenizer, the parser uses a built-in
        // 4-chars-per-token estimator (sufficient for scan-budget gating).
        const B_HEADER_SCAN: usize = 512;
        const B_HEADER: usize = 256;

        let verified_head = self.tape.get_verified_head();
        let parsed_header = parse_prefix_json(&env_result.raw_output, B_HEADER_SCAN, B_HEADER);

        match (parsed_header, env_result.is_success()) {
            (Ok(header), true) if header.status == StateStatus::Proceed => {
                // Happy path: commit accepted state, advance verified_head.
                let accepted = self.tape.commit(CommitRequest {
                    kind: NodeKind::StateAccepted,
                    verified: true,
                    parent: Some(verified_head.clone()),
                    scope: None,
                    attempt_ordinal: None,
                    reject_class: None,
                    token_count: None,
                    payload: serde_json::json!({
                        "state_update": header,
                        "output_summary": "accepted",
                    }),
                });
                self.tape.set_verified_head(accepted.hash);
                KernelStep::Proceed
            }
            (Ok(header), _) => {
                // Rejection path: handled by Atom 7's handle_rejection.
                self.handle_rejection(task, verified_head, header, env_result)
            }
            (Err(parse_error), _) => {
                // Invalid-header path: synthesize a header tagged Invalid.
                let invalid_header = StateUpdate {
                    schema_version: "tdma-state-update/v1".into(),
                    status: StateStatus::Invalid,
                    task_id: task.id.clone(),
                    action: "RETRY_INVALID_HEADER".into(),
                    failed_predicate: Some("state_update_header".into()),
                    reject_class: Some("MalformedOrMissingStateUpdate".into()),
                    next_action_hint: Some(parse_error.to_string()),
                    evidence_hash: None,
                };
                self.handle_rejection(task, verified_head, invalid_header, env_result)
            }
        }
    }

    /// Rejection handler — full 8-step body lands in Atom 7 per plan §5 Atom 7
    /// task book. Atom 2 leaves a `todo!` placeholder so the kernel cannot be
    /// driven through the rejection path until Atom 7 ships the full
    /// distiller + BBS + rtool + prompt-assembly stack.
    /// TRACE_MATRIX FC1a-handle_rejection (Atom 7 surface).
    fn handle_rejection(
        &mut self,
        _task: &Task,
        _verified_head: String,
        _header: StateUpdate,
        _env_result: EnvironmentResult,
    ) -> KernelStep {
        // Atom 7 will replace this stub with the full directive §5.2 8-step flow.
        unimplemented!(
            "handle_rejection lands in Atom 7 (memory_kernel keystone). \
             Atom 2 only ships the step_forward entry + routing skeleton."
        )
    }

    /// Pure helper exposed for tests: derive the latest BBS for a scope from
    /// tape alone. Atom 7 uses this inside handle_rejection.
    /// TRACE_MATRIX FC1a-tape_t (pure read).
    pub fn latest_belief_state(&self, scope: &AttemptScope) -> Option<RetryBeliefState> {
        self.tape.derive_latest_belief_state_from_tape(scope)
    }
}

// ── Tests ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::MemoryTapeLedger;

    fn ok_header(task: &str) -> String {
        format!(
            r#"{{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"{}","action":"PROCEED"}}
---BODY---
all good"#,
            task
        )
    }

    fn retry_header(task: &str) -> String {
        format!(
            r#"{{"schema_version":"tdma-state-update/v1","status":"Retry","task_id":"{}","action":"RETRY","failed_predicate":"x.y","reject_class":"schema-fail"}}
---BODY---
needs another try"#,
            task
        )
    }

    fn fresh_kernel() -> MemoryKernel<MemoryTapeLedger> {
        let mut tape = MemoryTapeLedger::new();
        tape.set_verified_head("H0".into());
        MemoryKernel::new(tape, "run-test")
    }

    #[test]
    fn step_forward_proceed_advances_verified_head() {
        let mut k = fresh_kernel();
        let task = Task {
            id: "t1".into(),
            prompt: "do the thing".into(),
        };
        let env = EnvironmentResult {
            raw_output: ok_header("t1"),
            raw_stderr: String::new(),
            success: true,
        };
        let initial_head = k.tape.get_verified_head();
        let step = k.step_forward(&task, env);
        assert!(matches!(step, KernelStep::Proceed));
        assert_ne!(
            k.tape.get_verified_head(),
            initial_head,
            "verified_head must advance on Proceed"
        );
    }

    #[test]
    #[should_panic(expected = "Atom 7")]
    fn step_forward_retry_routes_to_handle_rejection_atom7_stub() {
        // Atom 2 ships routing only; Atom 7 fills in handle_rejection.
        let mut k = fresh_kernel();
        let task = Task {
            id: "t2".into(),
            prompt: "x".into(),
        };
        let env = EnvironmentResult {
            raw_output: retry_header("t2"),
            raw_stderr: String::new(),
            success: false,
        };
        let _ = k.step_forward(&task, env);
    }

    #[test]
    #[should_panic(expected = "Atom 7")]
    fn step_forward_invalid_header_routes_to_handle_rejection_atom7_stub() {
        // Missing JSON in prefix triggers Invalid header synthesis -> handle_rejection.
        let mut k = fresh_kernel();
        let task = Task {
            id: "t3".into(),
            prompt: "x".into(),
        };
        let env = EnvironmentResult {
            raw_output: "no json header here at all".into(),
            raw_stderr: "missing".into(),
            success: false,
        };
        let _ = k.step_forward(&task, env);
    }

    #[test]
    fn step_forward_proceed_does_not_overwrite_on_retry_header_status() {
        // Even if env.success=true, a header.status != Proceed falls through to handle_rejection.
        let mut k = fresh_kernel();
        let task = Task {
            id: "t4".into(),
            prompt: "x".into(),
        };
        let env = EnvironmentResult {
            raw_output: retry_header("t4"),
            raw_stderr: String::new(),
            success: true, // env success but header says Retry
        };
        // This will panic via Atom 7 stub — confirming the routing decision.
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            k.step_forward(&task, env)
        }));
        assert!(result.is_err(), "must route to handle_rejection stub");
    }

    #[test]
    fn latest_belief_state_returns_none_for_empty_scope() {
        let k = fresh_kernel();
        let scope = AttemptScope {
            run_id: "run-test".into(),
            task_id: "t".into(),
            verified_parent: "H0".into(),
        };
        assert!(k.latest_belief_state(&scope).is_none());
    }
}
