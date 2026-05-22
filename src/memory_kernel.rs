//! TRACE_MATRIX FC1a-Q_t + FC1b-Q_{t+1} + FC2-boot_loop + FC3-replay:
//! TDMA-Bounded memory kernel — keystone integration.
//!
//! Atom 7 finalizes the kernel: full `step_forward` + `handle_rejection`
//! 8-step body (directive §5.2), `assemble_o1_prompt` (directive §11), and
//! `escalate` (directive §12). Each path holds the hard token budgets
//! (B_G + B_S + B_D + B_T + B_H + B_CTL = B_PROMPT_MAX = 5800 tokens) via
//! runtime asserts at the assembly site.
//!
//! KILL discipline (directive §15):
//!   * raw_stderr never enters the assembled prompt (only its sha256 hash
//!     appears via the EvidencePointer in the BBS payload).
//!   * No mutable belief-state sidecar — every BBS update is a tape commit
//!     with kind=RetryBeliefState.
//!   * No byte-length proxy for token counting — all sizing through Tokenizer.
//!   * No `<STATE_UPDATE>` closing-tag parser — only prefix-JSON scan.
//!   * constitution.md bytes never injected into worker prompt.
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use std::sync::Arc;

use crate::charter_core::CharterCore;
use crate::distiller::{compress_belief_state, deterministic_trace_slicer, TraceView};
use crate::ledger::{
    AttemptScope, CommitRequest, ImmutableTapeLedger, NodeKind, RetryBeliefState, RetryConstraint,
};
use crate::rtool::{Rtool, SessionDigest, WorkspaceView};
use crate::state_update::{parse_prefix_json, StateStatus, StateUpdate};
use crate::token_budget::{
    B_CTL, B_D, B_DISTILL_IN, B_G, B_H, B_HEADER, B_HEADER_SCAN, B_PROMPT_MAX, B_S, B_T,
    MAX_RETRIES, ZERO_GAIN_K,
};
use crate::tokenizer::Tokenizer;

// ── Public types ─────────────────────────────────────────────────

/// Worker task descriptor (directive §5.1).
/// TRACE_MATRIX FC1a-task_t: One unit of work fed to the kernel.
#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub prompt: String,
}

/// One round-trip result from the worker LLM + environment (directive §5.1).
/// TRACE_MATRIX FC1a-Agent_delta: The externalized signal coming back from a
/// worker — raw output, raw stderr (NEVER passed into prompt), and overall
/// success/failure verdict from the predicate runner.
#[derive(Debug, Clone)]
pub struct EnvironmentResult {
    pub raw_output: String,
    pub raw_stderr: String,
    pub success: bool,
}

impl EnvironmentResult {
    /// TRACE_MATRIX FC1a-Agent_delta: Predicate accessor.
    pub fn is_success(&self) -> bool {
        self.success
    }
}

/// Step decision returned by the kernel (directive §5.1).
/// TRACE_MATRIX FC1b-Q_{t+1}: Discriminates the next kernel transition.
#[derive(Debug, Clone)]
pub enum KernelStep {
    /// verified_head advanced; the worker should move to the next task.
    Proceed { evidence_hash: String },
    /// Retry with the rebuilt O(1) prompt (directive §11).
    Retry { prompt: String, bbs_hash: String, evidence_hash: String },
    /// Terminal escalation; commit chain frozen at verified_head.
    Escalate { reason: String, evidence_hash: String },
}

// ── Kernel ───────────────────────────────────────────────────────

/// TDMA-Bounded memory kernel (directive §5).
/// TRACE_MATRIX FC1a-rtool + FC1b-wtool + FC2-boot_loop: The single object
/// that ties tape (`ImmutableTapeLedger`), distiller, rtool, CharterCore, and
/// tokenizer into one FC1 runtime loop.
pub struct MemoryKernel<L: ImmutableTapeLedger> {
    pub tape: L,
    pub run_id: String,
    pub charter: CharterCore,
    pub tokenizer: Arc<Tokenizer>,
    pub rtool: Rtool<MemoryKernelTape<L>>,
}

/// Trivial newtype to satisfy `Arc<L: ImmutableTapeLedger>` lifetime in Rtool.
/// In RC1 the kernel owns the tape and the rtool holds an Arc back to a
/// read-only mirror of the indexes — Phase E will replace with a true
/// shared-ownership graph (libgit2 repo handle).
/// TRACE_MATRIX FC1a-rtool: bridging adapter between kernel and rtool.
pub struct MemoryKernelTape<L: ImmutableTapeLedger>(std::marker::PhantomData<L>);

// We do not need the rtool to actually call `tape.commit`; the cascade only
// reads the verified_head + task. A degenerate impl is enough for RC1 — the
// rtool uses Arc<Self> only as a generic-parameter placeholder. Phase E
// rewires this when libgit2 lands.
impl<L: ImmutableTapeLedger> ImmutableTapeLedger for MemoryKernelTape<L> {
    fn get_verified_head(&self) -> String {
        String::new()
    }
    fn set_verified_head(&mut self, _: String) {}
    fn commit(&mut self, _: CommitRequest) -> crate::ledger::TapeNode {
        unreachable!("MemoryKernelTape adapter does not own writes; the kernel writes directly")
    }
    fn count_nodes(
        &self,
        _: Option<NodeKind>,
        _: Option<bool>,
        _: Option<&str>,
        _: Option<&AttemptScope>,
    ) -> usize {
        0
    }
    fn latest_node(&self, _: NodeKind, _: &AttemptScope) -> Option<crate::ledger::TapeNode> {
        None
    }
    fn derive_latest_belief_state_from_tape(
        &self,
        _: &AttemptScope,
    ) -> Option<RetryBeliefState> {
        None
    }
}

impl<L: ImmutableTapeLedger> MemoryKernel<L> {
    /// TRACE_MATRIX FC2-Q_0: Boot a kernel against a tape ledger, run id, and
    /// CharterCore. The CharterCore must already have been validated for
    /// freshness via `validate_charter_core_freshness` by the caller.
    pub fn new(tape: L, run_id: impl Into<String>, charter: CharterCore) -> Self {
        let tokenizer = Arc::new(Tokenizer::new());
        let adapter: Arc<MemoryKernelTape<L>> = Arc::new(MemoryKernelTape(std::marker::PhantomData));
        let rtool = Rtool::new(adapter, tokenizer.clone());
        Self {
            tape,
            run_id: run_id.into(),
            charter,
            tokenizer,
            rtool,
        }
    }

    /// FC1 runtime loop entry-point (directive §5.1).
    /// TRACE_MATRIX FC1a-rtool + FC1a-output_edge + FC1b-wtool.
    pub fn step_forward(
        &mut self,
        task: &Task,
        env_result: EnvironmentResult,
    ) -> KernelStep {
        self.step_forward_with_workspace(task, env_result, &WorkspaceView::default())
    }

    /// Variant with workspace facts for richer SessionDigest cascade.
    /// TRACE_MATRIX FC1a-rtool: optional workspace input.
    pub fn step_forward_with_workspace(
        &mut self,
        task: &Task,
        env_result: EnvironmentResult,
        workspace: &WorkspaceView,
    ) -> KernelStep {
        let verified_head = self.tape.get_verified_head();
        let parsed_header = parse_prefix_json(&env_result.raw_output, B_HEADER_SCAN, B_HEADER);

        match (parsed_header, env_result.is_success()) {
            (Ok(header), true) if header.status == StateStatus::Proceed => {
                // Happy path: commit StateAccepted, advance verified_head.
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
                let evidence_hash = accepted.hash.clone();
                self.tape.set_verified_head(accepted.hash);
                KernelStep::Proceed { evidence_hash }
            }
            (Ok(header), _) => {
                self.handle_rejection(task, verified_head, header, env_result, workspace)
            }
            (Err(parse_error), _) => {
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
                self.handle_rejection(task, verified_head, invalid_header, env_result, workspace)
            }
        }
    }

    /// Rejection handler — 8-step body per directive §5.2.
    /// TRACE_MATRIX FC1a-handle_rejection.
    fn handle_rejection(
        &mut self,
        task: &Task,
        verified_head: String,
        header: StateUpdate,
        env_result: EnvironmentResult,
        workspace: &WorkspaceView,
    ) -> KernelStep {
        // Step 1: scope
        let attempt_scope = AttemptScope {
            run_id: self.run_id.clone(),
            task_id: task.id.clone(),
            verified_parent: verified_head.clone(),
        };

        // attempt_ordinal = current count of AgentProposal verified=false
        // for this scope, plus 1.
        let next_ordinal = self.tape.count_nodes(
            Some(NodeKind::AgentProposal),
            Some(false),
            Some(&verified_head),
            Some(&attempt_scope),
        ) as u32
            + 1;

        // Step 2: commit raw evidence to tape; verified=false; do NOT advance head.
        let raw_stderr_sha256 = sha256_hex(env_result.raw_stderr.as_bytes());
        let evidence_node = self.tape.commit(CommitRequest {
            kind: NodeKind::AgentProposal,
            verified: false,
            parent: Some(verified_head.clone()),
            scope: Some(attempt_scope.clone()),
            attempt_ordinal: Some(next_ordinal),
            reject_class: header.reject_class.clone(),
            token_count: None,
            payload: serde_json::json!({
                "state_update": header,
                "raw_output": env_result.raw_output,
                "raw_stderr": env_result.raw_stderr,
                "raw_stderr_sha256": raw_stderr_sha256,
            }),
        });
        let evidence_hash = evidence_node.hash.clone();

        // Step 3: deterministic_trace_slicer (pure pre-LLM gate)
        let trace_view: TraceView = deterministic_trace_slicer(
            &env_result.raw_stderr,
            &header,
            B_DISTILL_IN,
            &self.tokenizer,
        );
        assert!(
            self.tokenizer.count_json(&trace_view) <= B_DISTILL_IN,
            "distiller_input_budget breach: {} > B_DISTILL_IN={}",
            self.tokenizer.count_json(&trace_view),
            B_DISTILL_IN,
        );

        // Step 4: derive prev BBS PURELY from tape (no sidecar).
        let prev_bbs = self.tape.derive_latest_belief_state_from_tape(&attempt_scope);

        // Step 5: compress_belief_state — produce a new BBS that fits B_D.
        //
        // Atom 9 fix: extract a candidate RetryConstraint from this attempt's
        // failure shape and feed it into the BBS compressor. Previously the
        // kernel always passed an empty new_rules slice, which meant the
        // distiller's accumulation/eviction machinery (constraints + priority)
        // never received anything to accumulate — only `failure_signature`
        // carried forward across retries, no constraint rules did.
        // The Atom 9 stress test exposed this gap; this wire-up closes it.
        //
        // Candidate: one constraint per failure; id keyed by the failure shape
        // (identical signatures dedup via compress_belief_state's
        // `!constraints.iter().any(|c| c.id == rule.id)` check), priority 200,
        // evidence_hash points back to the AgentProposal node on tape.
        let candidate_id = format!(
            "c-{}-{}",
            trace_view.reject_class, trace_view.failed_predicate
        );
        let candidate_rules = vec![RetryConstraint {
            id: candidate_id,
            rule: format!(
                "avoid {}:{} (observed at attempt {})",
                trace_view.reject_class, trace_view.failed_predicate, next_ordinal
            ),
            priority: 200,
            source_attempt: next_ordinal,
            evidence_hash: evidence_hash.clone(),
        }];
        let new_bbs = compress_belief_state(
            prev_bbs.as_ref(),
            &trace_view,
            &candidate_rules,
            &evidence_hash,
            &attempt_scope,
            B_D,
            &self.tokenizer,
        );
        assert!(
            self.tokenizer.count_json(&new_bbs) <= B_D,
            "bbs_budget breach: {} > B_D={}",
            self.tokenizer.count_json(&new_bbs),
            B_D,
        );

        // Step 6: commit new BBS to tape as kind=RetryBeliefState verified=false.
        let bbs_payload = serde_json::to_value(&new_bbs).unwrap_or(serde_json::json!({}));
        let bbs_node = self.tape.commit(CommitRequest {
            kind: NodeKind::RetryBeliefState,
            verified: false,
            parent: Some(evidence_hash.clone()),
            scope: Some(attempt_scope.clone()),
            attempt_ordinal: Some(next_ordinal),
            reject_class: Some(new_bbs.failure_signature.reject_class.clone()),
            token_count: None,
            payload: bbs_payload,
        });
        let bbs_hash = bbs_node.hash.clone();

        // Step 7: retry counter + zero-gain breaker.
        // Recount AgentProposal nodes (now includes the one we just committed).
        let retry_count = self.tape.count_nodes(
            Some(NodeKind::AgentProposal),
            Some(false),
            Some(&verified_head),
            Some(&attempt_scope),
        );
        if retry_count >= MAX_RETRIES as usize {
            return self.escalate(task, &verified_head, &attempt_scope, &new_bbs, "MAX_RETRIES");
        }
        if new_bbs.zero_gain_streak >= ZERO_GAIN_K {
            return self.escalate(task, &verified_head, &attempt_scope, &new_bbs, "ZERO_GAIN");
        }

        // Step 8: in-budget SessionDigest checkout + O(1) prompt assembly.
        let session_digest =
            self.rtool
                .checkout_digest_with_workspace(&verified_head, task, workspace, B_S);
        assert!(
            self.tokenizer.count_text(&session_digest.text) <= B_S,
            "session_digest_budget breach: {} > B_S={}",
            self.tokenizer.count_text(&session_digest.text),
            B_S,
        );

        let prompt =
            self.assemble_o1_prompt(&session_digest, &new_bbs, task, &evidence_hash, &bbs_hash);
        assert!(
            self.tokenizer.count_text(&prompt) <= B_PROMPT_MAX,
            "prompt_budget breach: {} > B_PROMPT_MAX={}",
            self.tokenizer.count_text(&prompt),
            B_PROMPT_MAX,
        );

        KernelStep::Retry {
            prompt,
            bbs_hash,
            evidence_hash,
        }
    }

    /// O(1) prompt assembler (directive §11).
    /// Composition order: CharterCore + state-first contract + SessionDigest +
    /// RetryBeliefState + EvidencePointers + Task. RAW STDERR NEVER INCLUDED —
    /// only the evidence_hash and bbs_hash pointers.
    /// TRACE_MATRIX FC1a-rtool + KILL-tdma-1 + KILL-tdma-6.
    pub fn assemble_o1_prompt(
        &self,
        session_digest: &SessionDigest,
        bbs: &RetryBeliefState,
        task: &Task,
        evidence_hash: &str,
        bbs_hash: &str,
    ) -> String {
        let charter_text = self.tokenizer.first_tokens(&self.charter.content, B_G);
        let session_text = self.tokenizer.first_tokens(&session_digest.text, B_S);
        let task_text = self.tokenizer.first_tokens(&task.prompt, B_T);

        // The control text is fixed (B_CTL ceiling) — explicit state-first
        // output contract reminder. Body is left to the worker's reasoning.
        let control_text = format!(
            "[OUTPUT CONTRACT]\n\
             First syntactic object MUST be a JSON header matching schema\n\
             tdma-state-update/v1 within the first {scan} tokens (max {hdr} tokens).\n\
             Put body AFTER a line containing ---BODY---.\n\
             DO NOT include any raw stderr in your output.\n",
            scan = B_HEADER_SCAN,
            hdr = B_HEADER,
        );

        let bbs_json = serde_json::to_string(bbs).unwrap_or_default();
        let evidence_text = format!(
            "raw_failure_evidence_node={}\nbelief_state_node={}\n",
            evidence_hash, bbs_hash
        );

        let prompt = format!(
            "{charter}\n\n\
             {control}\n\
             [AUTHORITATIVE SESSION DIGEST]\n{session}\n\n\
             [RETRY BELIEF STATE]\n{bbs}\n\n\
             [EVIDENCE POINTERS]\n{evidence}\n\
             [CURRENT TASK]\n{task}\n",
            charter = charter_text,
            control = control_text,
            session = session_text,
            bbs = bbs_json,
            evidence = evidence_text,
            task = task_text,
        );

        prompt
    }

    /// Terminal escalation node (directive §12).
    /// TRACE_MATRIX FC1a-escalation: Commits kind=Escalation, verified=false;
    /// does NOT advance verified_head. The kernel returns
    /// `KernelStep::Escalate` and the caller (runner) stops the loop.
    fn escalate(
        &mut self,
        task: &Task,
        verified_head: &str,
        scope: &AttemptScope,
        bbs: &RetryBeliefState,
        reason: &str,
    ) -> KernelStep {
        let node = self.tape.commit(CommitRequest {
            kind: NodeKind::Escalation,
            verified: false,
            parent: Some(verified_head.to_string()),
            scope: Some(scope.clone()),
            attempt_ordinal: None,
            reject_class: Some(reason.to_string()),
            token_count: None,
            payload: serde_json::json!({
                "reason": reason,
                "task_id": task.id,
                "verified_head": verified_head,
                "belief_state": bbs,
            }),
        });
        KernelStep::Escalate {
            reason: reason.to_string(),
            evidence_hash: node.hash,
        }
    }

    /// Pure helper exposed for tests: derive the latest BBS for a scope from
    /// tape alone (no sidecar).
    /// TRACE_MATRIX FC1a-tape_t (pure read).
    pub fn latest_belief_state(&self, scope: &AttemptScope) -> Option<RetryBeliefState> {
        self.tape.derive_latest_belief_state_from_tape(scope)
    }
}

// ── helpers ──────────────────────────────────────────────────────

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(bytes);
    format!("{:x}", h.finalize())
}

// ── Tests ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::charter_core::compile_charter_core;
    use crate::ledger::MemoryTapeLedger;

    fn fresh_charter() -> CharterCore {
        compile_charter_core(
            "# Constitution\n## Art. 0.4 — Q_t version control\nFC1a tape_t.\n".as_bytes(),
            "v1.0",
            &Tokenizer::new(),
        )
    }

    fn fresh_kernel() -> MemoryKernel<MemoryTapeLedger> {
        let mut tape = MemoryTapeLedger::new();
        tape.set_verified_head("H0".into());
        MemoryKernel::new(tape, "run-test", fresh_charter())
    }

    fn ok_header(task: &str) -> String {
        format!(
            r#"{{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"{}","action":"PROCEED"}}
---BODY---
done"#,
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

    // ── Routing skeleton (Atom 2 contracts unchanged) ───────────

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
        match step {
            KernelStep::Proceed { evidence_hash } => assert!(!evidence_hash.is_empty()),
            _ => panic!("expected Proceed"),
        }
        assert_ne!(k.tape.get_verified_head(), initial_head);
    }

    #[test]
    fn step_forward_retry_path_returns_bounded_prompt() {
        let mut k = fresh_kernel();
        let task = Task {
            id: "t2".into(),
            prompt: "x".into(),
        };
        let env = EnvironmentResult {
            raw_output: retry_header("t2"),
            raw_stderr: "assertion failed at src/foo.rs:42\n".into(),
            success: false,
        };
        let initial_head = k.tape.get_verified_head();
        let step = k.step_forward(&task, env);
        match step {
            KernelStep::Retry {
                prompt,
                bbs_hash,
                evidence_hash,
            } => {
                assert!(!prompt.is_empty());
                assert!(!bbs_hash.is_empty());
                assert!(!evidence_hash.is_empty());
                // Prompt must fit composite budget
                assert!(Tokenizer::new().count_text(&prompt) <= B_PROMPT_MAX);
                // verified_head MUST NOT advance on failure
                assert_eq!(k.tape.get_verified_head(), initial_head);
            }
            _ => panic!("expected Retry"),
        }
    }

    #[test]
    fn step_forward_invalid_header_does_not_advance_head() {
        let mut k = fresh_kernel();
        let task = Task {
            id: "t3".into(),
            prompt: "x".into(),
        };
        let env = EnvironmentResult {
            raw_output: "no json header here at all".into(),
            raw_stderr: "parse failed".into(),
            success: false,
        };
        let initial_head = k.tape.get_verified_head();
        let _ = k.step_forward(&task, env);
        assert_eq!(k.tape.get_verified_head(), initial_head);
    }

    #[test]
    fn max_retries_escalates() {
        let mut k = fresh_kernel();
        let task = Task {
            id: "loop".into(),
            prompt: "x".into(),
        };
        let mut escalated = false;
        for _ in 0..(MAX_RETRIES + 2) {
            let env = EnvironmentResult {
                raw_output: retry_header("loop"),
                raw_stderr: "schema fail\n".into(),
                success: false,
            };
            match k.step_forward(&task, env) {
                KernelStep::Escalate { reason, .. } => {
                    assert!(reason == "MAX_RETRIES" || reason == "ZERO_GAIN");
                    escalated = true;
                    break;
                }
                _ => {}
            }
        }
        assert!(escalated, "must escalate within MAX_RETRIES iterations");
    }

    #[test]
    fn prompt_never_contains_raw_stderr_substring() {
        let mut k = fresh_kernel();
        let task = Task {
            id: "leak-test".into(),
            prompt: "x".into(),
        };
        let raw_stderr_sentinel = "RAW_STDERR_SENTINEL_LEAK_CANARY_42";
        let env = EnvironmentResult {
            raw_output: retry_header("leak-test"),
            raw_stderr: format!("{}\nat src/foo.rs:1\n", raw_stderr_sentinel),
            success: false,
        };
        let step = k.step_forward(&task, env);
        match step {
            KernelStep::Retry { prompt, .. } => {
                assert!(
                    !prompt.contains(raw_stderr_sentinel),
                    "raw stderr leaked into prompt"
                );
            }
            _ => panic!("expected Retry"),
        }
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

// Suppress unused-Arc<MemoryKernelTape> lint when the adapter is never
// fully exercised inside the kernel (it exists for Phase E API parity).
#[allow(dead_code)]
fn _shut_up_adapter<L: ImmutableTapeLedger>(_t: Arc<MemoryKernelTape<L>>) {}
