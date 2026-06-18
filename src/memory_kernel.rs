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
use crate::economy::money::MicroCoin;
use crate::ledger::{
    AttemptScope, CommitRequest, ImmutableTapeLedger, NodeKind, RetryBeliefState, RetryConstraint,
};
use crate::predicate_admission::{
    decide_admission_with_taint, hash_to_hex, AdmissionVerdict, ArgTaintFinding, PredicateClaimSet,
};
use crate::rtool::{Rtool, SessionDigest, WorkspaceView};
// LIVE-FC1 Phase 5 — BUDGET HARD-CEILING (the Turing fuel = FC2-HALT). The
// budget-ceiling mechanism lives in an UNPINNED module nested under
// `runtime/agent_scheduler.rs`; the kernel membrane consults its pure
// pre-admission check. ZERO genesis-pinned-file edits (this file,
// `memory_kernel.rs`, is itself pin-count 0).
use crate::runtime::agent_scheduler::budget_ceiling::{
    budget_check, live_tape_spend_tokens, reject_class_label, BudgetVerdict,
};
use crate::state::q_state::Hash;
use crate::state_update::{parse_prefix_json, StateStatus, StateUpdate};
use crate::token_budget::{
    B_CTL, B_D, B_DISTILL_IN, B_G, B_H, B_HEADER, B_HEADER_SCAN, B_PROMPT_MAX, B_S, B_T,
    MAX_RETRIES, ZERO_GAIN_K,
};
use crate::tokenizer::Tokenizer;
use crate::top_white::predicates::registry::{
    BootPredicateManifest, EmptyPredicateCasView, PredicateCasView, PredicateRegistry,
};

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
    Retry {
        prompt: String,
        bbs_hash: String,
        evidence_hash: String,
    },
    /// Terminal escalation; commit chain frozen at verified_head.
    Escalate {
        reason: String,
        evidence_hash: String,
    },
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
    // M07 single-admission: the kernel now consults the SAME predicate-admission
    // contract as the sequencer before advancing the verified head. Legacy
    // (zero-root) callers get an empty registry / `Hash::ZERO` root / empty CAS
    // via the defaulting `new`, so the Proceed branch still PASSes — but now WITH
    // a tape-recorded admission receipt instead of a predicate-blind advance.
    pub predicate_registry: Arc<PredicateRegistry>,
    pub predicate_registry_root_t: Hash,
    pub predicate_cas: Arc<dyn PredicateCasView + Send + Sync>,
    /// M07 G3 (2026-06-07; §8 token APPROVE-M07-G3-OS-QUALIFIED-RUN-FIELD):
    /// run-level OS-qualification, the kernel analogue of `QState::os_qualified_t`.
    /// `false` for every legacy 3-arg `new` call site (zero-root verdict-trusting
    /// admit preserved); set `true` for an OS-qualified run that binds a non-zero
    /// predicate registry via `new_with_predicates`. When `true`, the shared
    /// admission contract REFUSES a zero registry root, mirroring the sequencer.
    /// TRACE_MATRIX FC1a-predicates + FC2-boot_loop: kernel run OS-qualification.
    pub os_qualified_t: bool,
    /// LIVE-FC1 Phase 5 (§8 token APPROVE-BUDGET-HARD-CEILING-FROM-MANIFEST): the
    /// run-level economic-spend HARD CEILING, in `MicroCoin` micro-units — the
    /// kernel analogue of the PINNED read-only `BudgetSnapshot.cost_ceiling_microcoin`
    /// (`q_state.rs:148`). Populated at run init from the signed/user-approved
    /// budget manifest (`budget_ceiling::BudgetManifest::ceiling_micro`); default
    /// `MicroCoin::zero()` ⇒ UNLIMITED, so EVERY legacy call site (`new` /
    /// `new_with_predicates`) preserves today's behavior with NO budget reject.
    /// A POSITIVE ceiling arms the FC2-HALT: once tape-derived spend reaches it,
    /// every further proposal is REJECTED with no head advance. INTEGER-ONLY.
    /// TRACE_MATRIX FC1a-predicates + FC2-HALT: kernel economic-spend ceiling.
    pub cost_ceiling_microcoin: MicroCoin,
    /// LIVE-FC1 Phase 5: the integer token cost of the CURRENT externalized
    /// attempt (prompt+completion+tool tokens), set by the cost-aware membrane
    /// entry [`MemoryKernel::step_forward_with_budget`] and folded into the
    /// committed node's `token_count` so the NEXT tick's tape-derived spend
    /// (`live_tape_spend_tokens`) reflects it — keeping the spend genuinely
    /// TAPE-DERIVED (not a sidecar counter) and the ceiling check non-vacuous on
    /// the live FC1 loop. `0` for the legacy cost-blind entries (their nodes keep
    /// `token_count == None` exactly as before). INTEGER-ONLY.
    /// TRACE_MATRIX FC1a-tape_t: per-attempt integer token cost (→ node.token_count).
    pending_step_cost_tokens: usize,
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
    fn derive_latest_belief_state_from_tape(&self, _: &AttemptScope) -> Option<RetryBeliefState> {
        None
    }
    fn dump_all_nodes(&self) -> Vec<(String, crate::ledger::TapeNode)> {
        Vec::new()
    }
}

impl<L: ImmutableTapeLedger> MemoryKernel<L> {
    /// TRACE_MATRIX FC2-Q_0: Boot a kernel against a tape ledger, run id, and
    /// CharterCore. The CharterCore must already have been validated for
    /// freshness via `validate_charter_core_freshness` by the caller.
    pub fn new(tape: L, run_id: impl Into<String>, charter: CharterCore) -> Self {
        // Defaulting constructor: zero-root, empty registry, empty CAS. This
        // preserves every existing 3-arg call site — the kernel admits via the
        // shared zero-root branch (`os_qualified == false`) and writes a
        // `registry_root: ZERO` receipt, behavior-identical to the pre-M07
        // happy path except for the additive on-tape admission receipt.
        let registry = Arc::new(
            PredicateRegistry::from_boot_manifest(BootPredicateManifest::empty())
                .expect("empty predicate manifest is always constructible"),
        );
        Self::new_with_predicates(
            tape,
            run_id,
            charter,
            registry,
            Hash::ZERO,
            Arc::new(EmptyPredicateCasView),
        )
    }

    /// M07: explicit constructor for OS-qualified runs that bind a predicate
    /// registry. The Proceed branch takes its admission decision under
    /// `predicate_registry_root_t`; a non-zero root selects the bound oracle
    /// path (sequencer-only in route A — the kernel claim set is always boolean,
    /// so the kernel stays on the zero-root branch in practice today).
    /// TRACE_MATRIX FC1a-predicates + FC1b-Q_{t+1}: OS-qualified kernel ctor.
    pub fn new_with_predicates(
        tape: L,
        run_id: impl Into<String>,
        charter: CharterCore,
        predicate_registry: Arc<PredicateRegistry>,
        predicate_registry_root_t: Hash,
        predicate_cas: Arc<dyn PredicateCasView + Send + Sync>,
    ) -> Self {
        let tokenizer = Arc::new(Tokenizer::new());
        let adapter: Arc<MemoryKernelTape<L>> =
            Arc::new(MemoryKernelTape(std::marker::PhantomData));
        let rtool = Rtool::new(adapter, tokenizer.clone());
        // M07 G3: a kernel that binds a non-zero predicate registry is an
        // OS-qualified run. The defaulting `new` passes `Hash::ZERO` here, so the
        // legacy zero-root path stays `os_qualified_t == false`.
        let os_qualified_t = predicate_registry_root_t != Hash::ZERO;
        Self {
            tape,
            run_id: run_id.into(),
            charter,
            tokenizer,
            rtool,
            predicate_registry,
            predicate_registry_root_t,
            predicate_cas,
            os_qualified_t,
            // FORWARD-ONLY default: zero ceiling ⇒ UNLIMITED (today's behavior).
            // An OS-qualified run arms the hard ceiling via `with_cost_ceiling`
            // (populated from the signed budget manifest at run init).
            cost_ceiling_microcoin: MicroCoin::zero(),
            // Legacy cost-blind default: no per-attempt cost recorded (nodes keep
            // `token_count == None`). The cost-aware membrane entry sets this.
            pending_step_cost_tokens: 0,
        }
    }

    /// LIVE-FC1 Phase 5 (§8 APPROVE-BUDGET-HARD-CEILING-FROM-MANIFEST): arm the
    /// run-level economic-spend HARD CEILING at run init. The `ceiling` is the
    /// integer `MicroCoin` read from the signed/user-approved budget manifest
    /// (`budget_ceiling::BudgetManifest::ceiling_micro`) — this is the unpinned
    /// runner-path population of the ceiling that the PINNED
    /// `BudgetSnapshot.cost_ceiling_microcoin` field also holds (we do NOT edit
    /// `q_state`). Builder form so every existing call site stays UNLIMITED
    /// unless it explicitly opts in. `MicroCoin::zero()` ⇒ UNLIMITED (no-op).
    /// TRACE_MATRIX FC2-economic-tick: manifest → kernel ceiling at run init.
    pub fn with_cost_ceiling(mut self, ceiling: MicroCoin) -> Self {
        self.cost_ceiling_microcoin = ceiling;
        self
    }

    /// FC1 runtime loop entry-point (directive §5.1).
    /// TRACE_MATRIX FC1a-rtool + FC1a-output_edge + FC1b-wtool.
    ///
    /// 3-arg shim: callers with no predicate claims supply an empty
    /// [`PredicateClaimSet`], which yields a zero-root PASS — behavior-identical
    /// to the pre-M07 happy path (now with an on-tape admission receipt).
    pub fn step_forward(&mut self, task: &Task, env_result: EnvironmentResult) -> KernelStep {
        self.step_forward_with_claims(task, env_result, PredicateClaimSet::default())
    }

    /// M07: FC1 entry-point carrying the predicate claim set the head advance is
    /// gated on. The judge seam in `tdma_runner` builds a real claim set from the
    /// verdict and calls through here.
    /// TRACE_MATRIX FC1a-predicates + FC1b-Q_{t+1}.
    pub fn step_forward_with_claims(
        &mut self,
        task: &Task,
        env_result: EnvironmentResult,
        claims: PredicateClaimSet,
    ) -> KernelStep {
        self.step_forward_with_workspace(task, env_result, claims, &WorkspaceView::default())
    }

    /// Variant with workspace facts for richer SessionDigest cascade.
    /// TRACE_MATRIX FC1a-rtool: optional workspace input.
    pub fn step_forward_with_workspace(
        &mut self,
        task: &Task,
        env_result: EnvironmentResult,
        claims: PredicateClaimSet,
        workspace: &WorkspaceView,
    ) -> KernelStep {
        // LIVE-FC1 forward-wiring: DERIVE the real arg-taint findings from the
        // proposal on the tape, instead of passing the empty `&[]` placeholder.
        //
        // The worker's raw_output IS the on-tape state-update header. It MAY carry
        // an optional `wtool_call` declaration (args + their provenance source +
        // target tools + write keys); `derive_wtool_call_from_proposal` is a pure,
        // deterministic, replay-stable function of those bytes — no RNG, no
        // wall-clock. `arg_taint_v1` then flags only a genuinely external/tool
        // provenance arg flowing into a PRIVILEGED sink.
        //
        // NO FALSE REJECTS (binding): an ordinary proposal carries NO `wtool_call`
        // declaration (or one with no privileged sink). The derived call is then
        // empty / sink-free → `arg_taint_v1` returns ZERO findings →
        // `decide_admission_with_taint` delegates to the unchanged
        // `decide_admission` and admits EXACTLY as before. Only genuine
        // external/tool provenance INTO a privileged sink newly produces a finding.
        let call =
            crate::predicate_admission::arg_taint_provenance::derive_wtool_call_from_proposal(
                &env_result.raw_output,
            );
        let taint_findings = crate::predicate_admission::arg_taint::arg_taint_v1(&call);
        self.step_forward_with_taint(task, env_result, claims, workspace, &taint_findings)
    }

    /// LIVE-FC1 Phase 5 — cost-aware FC1 entry-point (§8 token
    /// APPROVE-BUDGET-HARD-CEILING-FROM-MANIFEST). Identical to
    /// [`Self::step_forward_with_workspace`] but threads the integer token cost of
    /// THIS externalized attempt (`step_cost_tokens` = prompt+completion+tool
    /// tokens). The cost is recorded as the committed node's `token_count`, so the
    /// run's tape-derived spend (`live_tape_spend_tokens`) grows tick-by-tick from
    /// the TAPE itself — no sidecar counter. The pre-admission budget check at the
    /// top of [`Self::step_forward_with_taint`] then halts the run once cumulative
    /// spend reaches the signed-manifest ceiling.
    ///
    /// FORWARD-ONLY: with a zero ceiling (the default), this admits exactly like
    /// the cost-blind entries; the only difference is the additive `token_count`
    /// stamp on the node (a previously-`None` field), which changes no admission
    /// decision and no head-advance behavior.
    /// TRACE_MATRIX FC1a-rtool + FC1a-tape_t + FC2-HALT: cost-aware FC1 entry.
    pub fn step_forward_with_budget(
        &mut self,
        task: &Task,
        env_result: EnvironmentResult,
        claims: PredicateClaimSet,
        workspace: &WorkspaceView,
        step_cost_tokens: usize,
    ) -> KernelStep {
        // Record this attempt's integer cost so the node committed on this tick
        // carries it (→ next tick's tape-derived spend reflects it).
        self.pending_step_cost_tokens = step_cost_tokens;
        let call =
            crate::predicate_admission::arg_taint_provenance::derive_wtool_call_from_proposal(
                &env_result.raw_output,
            );
        let taint_findings = crate::predicate_admission::arg_taint::arg_taint_v1(&call);
        let step =
            self.step_forward_with_taint(task, env_result, claims, workspace, &taint_findings);
        // Reset so a subsequent cost-blind call does not inherit this cost.
        self.pending_step_cost_tokens = 0;
        step
    }

    /// The per-attempt cost stamp for a committed node: `Some(n)` when the
    /// cost-aware entry supplied a non-zero cost, else `None` (legacy cost-blind
    /// behavior, byte-identical to before). Centralizes the one place both commit
    /// sites (accepted advance + rejection evidence) read the pending cost.
    /// TRACE_MATRIX FC1a-tape_t: per-attempt token_count stamp.
    fn step_token_count(&self) -> Option<usize> {
        if self.pending_step_cost_tokens == 0 {
            None
        } else {
            Some(self.pending_step_cost_tokens)
        }
    }

    /// arg-taint sub-article entry: identical to [`Self::step_forward_with_workspace`]
    /// but threads the value-level taint findings (from
    /// `predicate_admission::arg_taint::arg_taint_v1`) into the admission oracle.
    /// A non-empty findings set (any tainted-arg → privileged-sink flow) makes the
    /// SHARED admission contract REFUSE the advance — the confused-deputy
    /// hard-gate — and routes to the existing non-advancing rejection path, with a
    /// tape-recorded `arg_taint_v1[...]` rejection receipt. This is the UNPINNED
    /// kernel seam that wires the hard-gate without editing any genesis-pinned
    /// file.
    /// TRACE_MATRIX FC1a-predicates + FC1b-Q_{t+1}: arg-taint hard-gate at FC1 wtool.
    pub fn step_forward_with_taint(
        &mut self,
        task: &Task,
        env_result: EnvironmentResult,
        claims: PredicateClaimSet,
        workspace: &WorkspaceView,
        taint_findings: &[ArgTaintFinding],
    ) -> KernelStep {
        let verified_head = self.tape.get_verified_head();

        // LIVE-FC1 Phase 5 — BUDGET HARD-CEILING pre-admission check (the Turing
        // fuel = FC2-HALT). §8 token APPROVE-BUDGET-HARD-CEILING-FROM-MANIFEST.
        //
        // BEFORE the header parse + the self-reported predicate booleans + the
        // arg-taint gate, refuse the advance when the run is OUT OF FUEL: the
        // tape-derived integer spend (reused VPPUT `C_i` semantics:
        // `live_tape_spend_tokens` sums `token_count` over EVERY node — accepted
        // StateAccepted AND failed-branch AgentProposal) has reached the
        // signed-manifest ceiling (`self.cost_ceiling_microcoin`).
        //
        // FORWARD-ONLY: a zero ceiling is UNLIMITED — `budget_check` returns
        // `Unlimited` and we fall through to the unchanged admission EXACTLY as
        // before (every legacy call site keeps `cost_ceiling_microcoin == 0`).
        //
        // FC2-HALT (emergent): on a positive-ceiling breach we route to the
        // existing non-advancing rejection path (`handle_rejection` commits an
        // AgentProposal verified:false and NEVER advances the verified head),
        // stamped with the PINNED `RejectionClass::BudgetExceeded` label. Because
        // the head does not advance and the spend only grows, EVERY subsequent
        // proposal also rejects — the run halts. CHECKPOINT-RESUME: the tape is
        // append-only and no head moved, so raising the ceiling (a new approved
        // manifest) lets the previously-halted proposal admit from the same head.
        let spend_tokens = live_tape_spend_tokens(&self.tape);
        if let BudgetVerdict::Exceeded {
            spend_micro,
            ceiling_micro,
        } = budget_check(spend_tokens, self.cost_ceiling_microcoin)
        {
            let budget_header = StateUpdate {
                schema_version: "tdma-state-update/v1".into(),
                status: StateStatus::Retry,
                task_id: task.id.clone(),
                action: "HALT_BUDGET_EXCEEDED".into(),
                // The synthetic failed-predicate encodes the budget verdict +
                // integer spend/ceiling micro-units so an auditor reconstructs the
                // FC2-HALT from the rejection receipt alone (no f64, no raw bytes).
                failed_predicate: Some(format!(
                    "budget_hard_ceiling[spend_micro={spend_micro};ceiling_micro={ceiling_micro}]"
                )),
                // Reuses the PINNED RejectionClass::BudgetExceeded discriminant
                // (typed_tx.rs:174) via its canonical label — NO new discriminant.
                reject_class: Some(reject_class_label()),
                next_action_hint: Some(
                    "economic-spend ceiling reached; raise the approved budget manifest to resume"
                        .into(),
                ),
                evidence_hash: None,
            };
            return self.handle_rejection(
                task,
                verified_head,
                budget_header,
                env_result,
                workspace,
            );
        }

        let parsed_header = parse_prefix_json(&env_result.raw_output, B_HEADER_SCAN, B_HEADER);

        match (parsed_header, env_result.is_success()) {
            (Ok(header), true) if header.status == StateStatus::Proceed => {
                // M07 single-admission: the head advance is gated on the SHARED
                // predicate-admission contract, NOT on `success + Proceed` alone.
                // On PASS we commit `StateAccepted` WITH a tape-recorded admission
                // receipt and only THEN advance the verified head; on FAIL we
                // route to the existing non-advancing rejection path.
                let root_hex = hash_to_hex(&self.predicate_registry_root_t);
                // M07 G3: read the run-level OS-qualification field, NOT
                // `registry_root != ZERO`. For a legacy kernel both are false; for
                // an OS-qualified kernel the field gates the zero-root refuse-path.
                let os_qualified = self.os_qualified_t;
                // arg-taint sub-article HARD-GATE: refuse a tainted-arg →
                // privileged-sink flow BEFORE the self-reported predicate booleans.
                let verdict =
                    decide_admission_with_taint(&root_hex, &claims, os_qualified, taint_findings);

                match verdict {
                    AdmissionVerdict::Pass { registry_root_hex } => {
                        let acceptance_pids: Vec<&str> =
                            claims.acceptance.iter().map(|c| c.id.0.as_str()).collect();
                        let settlement_pids: Vec<&str> =
                            claims.settlement.iter().map(|c| c.id.0.as_str()).collect();
                        // EXPLICIT_ID_HALLUCINATION_EXPOSURE_AUDIT_2026-06-08
                        // fix #1 (HIGHEST RISK): the worker LLM echoes a
                        // `task_id` in its state-update header that is parsed
                        // with only an `is_empty()` check (state_update.rs:134).
                        // The kernel must NOT trust the agent value as
                        // canonical. Stamp the SYSTEM `task.id` into the header
                        // before it is persisted to the StateAccepted tape so
                        // the value on the accepted tape is system-authoritative
                        // (matching the canonical `AttemptScope.task_id`, which
                        // already uses `task.id`). Pure additive stamp; no wire
                        // change, no NodeKind change.
                        let accepted_header = StateUpdate {
                            task_id: task.id.clone(),
                            ..header
                        };
                        let accepted = self.tape.commit(CommitRequest {
                            kind: NodeKind::StateAccepted,
                            verified: true,
                            parent: Some(verified_head.clone()),
                            scope: None,
                            attempt_ordinal: None,
                            reject_class: None,
                            // LIVE-FC1 Phase 5: record this attempt's integer token
                            // cost so the run's tape-derived spend grows from the
                            // tape itself. `None` for cost-blind callers (legacy).
                            token_count: self.step_token_count(),
                            payload: serde_json::json!({
                                "state_update": accepted_header,
                                "output_summary": "accepted",
                                // M07 admission receipt — additive payload field,
                                // NO NodeKind / wire-schema change. Hash-covered
                                // (ledger.rs compute_hash folds the whole payload),
                                // so an auditor reconstructs the gate from tape alone.
                                "predicate_admission": {
                                    "verdict": "PASS",
                                    "registry_root": registry_root_hex,
                                    "os_qualified": os_qualified,
                                    "acceptance_pids": acceptance_pids,
                                    "settlement_pids": settlement_pids,
                                },
                            }),
                        });
                        let evidence_hash = accepted.hash.clone();
                        // Advance ONLY after the receipt-bearing StateAccepted commit.
                        self.tape.set_verified_head(accepted.hash);
                        KernelStep::Proceed { evidence_hash }
                    }
                    AdmissionVerdict::Fail {
                        failed_predicate, ..
                    } => {
                        // Predicate-admission FAIL: route to the existing
                        // non-advancing rejection path. The verified head stays
                        // frozen (handle_rejection commits AgentProposal
                        // verified:false and never advances the head).
                        //
                        // arg-taint sub-article: a tainted-arg → privileged-sink
                        // rejection is stamped with the `arg_taint_v1[...]`
                        // failed-predicate marker and a distinct reject_class so
                        // an auditor reconstructs the confused-deputy gate from
                        // the rejection receipt alone.
                        let is_arg_taint = failed_predicate.starts_with(
                            crate::predicate_admission::ARG_TAINT_FAILED_PREDICATE_PREFIX,
                        );
                        let reject_class = if is_arg_taint {
                            "ArgTaintIntoPrivilegedSink"
                        } else {
                            "PredicateAdmissionFailed"
                        };
                        let rej_header = StateUpdate {
                            status: StateStatus::Retry,
                            failed_predicate: Some(if failed_predicate.is_empty() {
                                "predicate_admission".into()
                            } else {
                                failed_predicate
                            }),
                            reject_class: Some(reject_class.into()),
                            ..header
                        };
                        self.handle_rejection(
                            task,
                            verified_head,
                            rej_header,
                            env_result,
                            workspace,
                        )
                    }
                }
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
            // LIVE-FC1 Phase 5: a FAILED branch still cost real tokens — record the
            // attempt's integer cost so the tape-derived spend counts failed
            // branches (mirrors VPPUT C_i: failed proposals MUST count). `None`
            // for cost-blind callers (legacy behavior unchanged).
            token_count: self.step_token_count(),
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
        let prev_bbs = self
            .tape
            .derive_latest_belief_state_from_tape(&attempt_scope);

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
            return self.escalate(
                task,
                &verified_head,
                &attempt_scope,
                &new_bbs,
                "MAX_RETRIES",
            );
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

        // CONFORMANCE FIX #5 (goodhart-shield): runtime PPUT-context-leak guard
        // at the O(1) prompt-assembly boundary (Art. III.4). The BBS / session
        // digest / charter are tape-derived surfaces; this RUNTIME call ensures a
        // PPUT scalar can never reach the worker prompt even if a future state
        // surface injects one. The sweep confirmed no current scoring-to-prompt
        // flow, so this passes on current content (defense-in-depth). Guard lives
        // in the trust-root-pinned `prompt_guard.rs` (unchanged). Gate:
        // tests/constitution_metric_leak_guard_wired.rs.
        crate::sdk::prompt_guard::assert_no_metric_leak(&prompt);

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
    fn state_accepted_tape_stamps_system_task_id_not_agent_echoed() {
        // EXPLICIT_ID_HALLUCINATION_EXPOSURE_AUDIT_2026-06-08 fix #1
        // (non-vacuous gate): the worker echoes a FORGED task_id in its header.
        // The StateAccepted tape payload MUST record the SYSTEM task.id, never
        // the agent-echoed value. Without the stamp this test fails (the agent
        // value would land on the canonical accepted tape).
        let mut k = fresh_kernel();
        let task = Task {
            id: "system-task-42".into(),
            prompt: "do the thing".into(),
        };
        let env = EnvironmentResult {
            // Agent echoes a DIFFERENT (forged) task_id than the system task.
            raw_output: ok_header("agent-forged-999"),
            raw_stderr: String::new(),
            success: true,
        };
        let step = k.step_forward(&task, env);
        let accepted_hash = match step {
            KernelStep::Proceed { evidence_hash } => evidence_hash,
            _ => panic!("expected Proceed"),
        };
        let accepted = k
            .tape
            .indexes
            .by_hash
            .get(&accepted_hash)
            .expect("StateAccepted node on tape");
        assert_eq!(accepted.kind, NodeKind::StateAccepted);
        let persisted_task_id = accepted.payload["state_update"]["task_id"]
            .as_str()
            .expect("state_update.task_id present");
        assert_eq!(
            persisted_task_id, "system-task-42",
            "StateAccepted must record the SYSTEM task id"
        );
        assert_ne!(
            persisted_task_id, "agent-forged-999",
            "agent-echoed task_id must NEVER be persisted as canonical"
        );
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
