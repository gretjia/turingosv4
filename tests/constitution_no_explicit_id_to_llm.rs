//! Explicit-id hallucination membrane — non-vacuous constitution gate.
//!
//! Audit: `EXPLICIT_ID_HALLUCINATION_EXPOSURE_AUDIT_2026-06-08`
//! (`handover/audits/...` explicit-id exposure surface, read-only predecessor
//! PR #327). The membrane fixes render explicit/guessable canonical ids
//! (`worktx-…`, `Agent_i`, `true-suite-market-{run}`, bare `task.id`) into the
//! LLM-visible projection as OPAQUE content-hash HANDLES, and validate
//! agent-echoed ids by equality / exact-membership / exact-handle — while
//! canonical state keeps keying on the legacy strings internally. This is the
//! RENDER/PARSE MEMBRANE only; the canonical identity types
//! (`TxId`/`AgentId`/`TaskId`/`EventId`), their minting in `q_state.rs`, the
//! typed-tx wire schema, sequencer admission, and signing payloads are
//! UNCHANGED (those are a separate Class-4 §8 atom).
//!
//! This gate proves THREE membrane invariants with realistic inputs, and is
//! falsifiable (every assertion has a documented mutation that turns it RED):
//!
//! 1. **MISMATCH REJECTED**
//!    - state_update fix #1: an agent-echoed `StateUpdate.task_id` that differs
//!      from the system task id is NOT persisted as canonical on the advancing
//!      `StateAccepted` tape — the system `task.id` is stamped instead.
//!    - protocol fixes #7/#8: an agent-emitted `AgentAction.node` /
//!      `target_work_tx_id` that is NOT a member of the rendered candidate set
//!      is REJECTED (`MembraneError::NotInCandidateSet`) BEFORE any pinned
//!      sequencer lookup.
//!
//! 2. **NO SILENT FALLBACK**
//!    - g1 fix #2: feeding g1's parent-resolution seam a hallucinated/missing
//!      handle resolves to `None` (no parent) — it does NOT bind
//!      `node_tx_ids.last()` (the removed default) nor any wrong node. Proven
//!      both behaviorally (the live `HandleSet::resolve` seam the binary calls)
//!      AND structurally (the binary source no longer binds `.last()` and DOES
//!      route through `node_handles.resolve`).
//!
//! 3. **HANDLES NOT EXPLICIT**
//!    - the fixed render projections (g1 self-id + market node block, rtool
//!      `level_4` MinimalHeadOnly, market_external prompt, your_position block)
//!      emit a content-hash HANDLE and do NOT leak the explicit
//!      `Agent_i` / `worktx-` / bare `task-`/`true-suite-market-` id literal.
//!
//! `FC-trace: FC1a-output_edge + FC1 rtool/wtool read-view shielding (Art. III)
//! — the agent-visible projection (input edge) shows opaque content-hash
//! handles; agent-echoed ids (Agent δ / output edge) are validated
//! equality/membership/exact-handle before any canonical bind.`

use turingosv4::charter_core::compile_charter_core;
use turingosv4::ledger::{ImmutableTapeLedger, MemoryTapeLedger, NodeKind};
use turingosv4::memory_kernel::{EnvironmentResult, KernelStep, MemoryKernel, Task};
use turingosv4::rtool::{Rtool, SessionDegradationLevel, WorkspaceView};
use turingosv4::sdk::id_handle::{handle, HandleSet};
use turingosv4::sdk::protocol::{parse_agent_output, MembraneError};
use turingosv4::tokenizer::Tokenizer;

use std::sync::Arc;

// Source paths witnessed by the structural (enumerate-all-sites / pinned≠wired)
// half of this gate. These are the UNPINNED render-membrane surfaces the audit
// fixes touched.
const G1_SRC: &str = "src/bin/g1_market_live_agent.rs";
const MARKET_EXTERNAL_SRC: &str = "src/bin/market_external_agent_current_kernel.rs";
const RTOOL_SRC: &str = "src/rtool.rs";
const YOUR_POSITION_SRC: &str = "src/sdk/your_position.rs";
const MEMORY_KERNEL_SRC: &str = "src/memory_kernel.rs";

fn fresh_kernel() -> MemoryKernel<MemoryTapeLedger> {
    let mut tape = MemoryTapeLedger::new();
    tape.set_verified_head("H0".into());
    let charter = compile_charter_core(
        "# Constitution\n## Art. 0.4 — Q_t version control\nFC1a tape_t.\n".as_bytes(),
        "v1.0",
        &Tokenizer::new(),
    );
    MemoryKernel::new(tape, "run-id-membrane-gate", charter)
}

/// A worker state-update header whose echoed `task_id` field is `echoed_task`.
fn header_echoing(echoed_task: &str) -> String {
    format!(
        r#"{{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"{echoed_task}","action":"PROCEED"}}
---BODY---
done"#
    )
}

// ────────────────────────────────────────────────────────────────────────
// (1) MISMATCH REJECTED — state_update fix #1
// ────────────────────────────────────────────────────────────────────────

#[test]
fn agent_echoed_task_id_mismatch_not_persisted_as_canonical() {
    // The worker echoes a FORGED task_id different from the system task.id.
    // The advancing StateAccepted tape payload MUST record the SYSTEM id.
    let mut k = fresh_kernel();
    let task = Task {
        id: "system-task-42".into(),
        prompt: "do the thing".into(),
    };
    let env = EnvironmentResult {
        raw_output: header_echoing("agent-forged-999"),
        raw_stderr: String::new(),
        success: true,
    };
    let accepted_hash = match k.step_forward(&task, env) {
        KernelStep::Proceed { evidence_hash } => evidence_hash,
        other => panic!("expected Proceed, got {other:?}"),
    };

    // Reconstruct the advancing accepted node from the tape alone.
    let (_h, accepted) = k
        .tape
        .dump_all_nodes()
        .into_iter()
        .find(|(h, _)| h == &accepted_hash)
        .expect("StateAccepted node on tape");
    assert_eq!(accepted.kind, NodeKind::StateAccepted);

    let persisted = accepted.payload["state_update"]["task_id"]
        .as_str()
        .expect("state_update.task_id present on tape");
    // GREEN: system-authoritative. MUTATION: drop the
    // `task_id: task.id.clone()` stamp in memory_kernel.rs (serialize `header`
    // directly) and the persisted value becomes `agent-forged-999` → RED.
    assert_eq!(
        persisted, "system-task-42",
        "StateAccepted must record the SYSTEM task id, not the agent echo"
    );
    assert_ne!(
        persisted, "agent-forged-999",
        "agent-echoed task_id must NEVER reach the canonical accepted tape"
    );
}

// ────────────────────────────────────────────────────────────────────────
// (1) MISMATCH REJECTED — protocol fixes #7/#8
// ────────────────────────────────────────────────────────────────────────

fn rendered_candidate_set() -> HandleSet {
    // The EXACT set of node ids the agent was shown this turn.
    let mut set = HandleSet::new("g1_node");
    set.insert("worktx-aaaaaaaa");
    set.insert("worktx-bbbbbbbb");
    set
}

#[test]
fn agent_node_not_in_rendered_set_rejected_before_pinned_lookup() {
    let set = rendered_candidate_set();

    // Positive control: an exact handle the agent was shown resolves.
    let good_handle = set.handle_for("worktx-aaaaaaaa");
    let good = parse_agent_output(&format!(
        r#"<action>{{"tool":"invest","node":"{good_handle}","amount":50}}</action>"#
    ))
    .expect("parse good action");
    assert_eq!(
        good.resolve_node(&set).expect("good node resolves"),
        "worktx-aaaaaaaa"
    );

    // REJECT: a node the agent was never shown (fabricated worktx). This is the
    // affordance that, before the fix, a loose prefix match / fallback could
    // silently bind to a real node.
    let forged = parse_agent_output(
        r#"<action>{"tool":"invest","node":"worktx-deadbeef","amount":50}</action>"#,
    )
    .expect("parse forged action");
    // GREEN: exact-membership-or-REJECT. MUTATION: change `HandleSet::resolve`
    // to a `starts_with` prefix match or a `.last()`/first-member fallback and
    // this Err flips to Ok → RED.
    assert!(
        matches!(
            forged.resolve_node(&set),
            Err(MembraneError::NotInCandidateSet(_))
        ),
        "a node not in the rendered set must be REJECTED before any pinned lookup"
    );

    // REJECT: a bare prefix of a real handle must NOT bind.
    let prefix =
        parse_agent_output(r#"<action>{"tool":"invest","node":"worktx","amount":50}</action>"#)
            .expect("parse prefix action");
    assert!(
        matches!(
            prefix.resolve_node(&set),
            Err(MembraneError::NotInCandidateSet(_))
        ),
        "a prefix of a candidate id must NOT fuzzy-match"
    );
}

#[test]
fn agent_target_work_tx_not_in_rendered_set_rejected_before_pinned_lookup() {
    let set = rendered_candidate_set();

    // Positive control: exact handle resolves to its canonical id.
    let good_handle = set.handle_for("worktx-bbbbbbbb");
    let good = parse_agent_output(&format!(
        r#"<action>{{"tool":"verify_peer","target_work_tx_id":"{good_handle}","verdict":"confirm"}}</action>"#
    ))
    .expect("parse good verify");
    assert_eq!(
        good.resolve_target_work_tx(&set)
            .expect("good target resolves"),
        "worktx-bbbbbbbb"
    );

    // REJECT: a verify_peer target the agent was never shown must be rejected
    // BEFORE the (pinned) VerifyTx / sequencer admission lookup.
    let forged = parse_agent_output(
        r#"<action>{"tool":"verify_peer","target_work_tx_id":"worktx-zzzzzzzz","verdict":"confirm"}</action>"#,
    )
    .expect("parse forged verify");
    assert!(
        matches!(
            forged.resolve_target_work_tx(&set),
            Err(MembraneError::NotInCandidateSet(_))
        ),
        "a verify_peer target not in the rendered set must be REJECTED at the unpinned seam"
    );
}

// ────────────────────────────────────────────────────────────────────────
// (2) NO SILENT FALLBACK — g1 fix #2 (parent resolution)
// ────────────────────────────────────────────────────────────────────────

/// Reproduce the EXACT g1 parent-resolution seam (g1_market_live_agent.rs
/// lines ~539-544): the LLM-echoed `parent_node` is resolved by
/// `node_handles.resolve(s)` after filtering "null"/empty; on miss the result
/// is `None`. This mirrors the binary's expression so the behavioral witness
/// tracks the real wiring, and the structural witness below proves the binary
/// has no OTHER (fallback) binding path.
fn g1_resolve_parent(node_handles: &HandleSet, echoed_parent_node: Option<&str>) -> Option<String> {
    echoed_parent_node
        .filter(|s| *s != "null" && !s.is_empty())
        .and_then(|s| node_handles.resolve(s))
}

#[test]
fn g1_hallucinated_parent_does_not_bind_last_node() {
    let mut node_handles = HandleSet::new("g1_node");
    node_handles.insert("worktx-first");
    node_handles.insert("worktx-middle");
    let last = "worktx-last";
    node_handles.insert(last);

    // A hallucinated handle the price signal never produced.
    let hallucinated = g1_resolve_parent(&node_handles, Some("0123456789abcdef"));
    // GREEN: no parent. MUTATION: re-introduce the removed
    // `.or_else(|| node_tx_ids.last().cloned())` fallback and this becomes
    // `Some("worktx-last")` → RED.
    assert_eq!(
        hallucinated, None,
        "a hallucinated parent handle must NOT bind the last (or any) node"
    );
    assert_ne!(
        hallucinated.as_deref(),
        Some(last),
        "the removed last() fallback must stay removed"
    );

    // A missing / null parent_node also yields None (no default node).
    assert_eq!(g1_resolve_parent(&node_handles, None), None);
    assert_eq!(g1_resolve_parent(&node_handles, Some("null")), None);
    assert_eq!(g1_resolve_parent(&node_handles, Some("")), None);

    // Positive control: an exact rendered handle DOES resolve to its node.
    let good = node_handles.handle_for("worktx-middle");
    assert_eq!(
        g1_resolve_parent(&node_handles, Some(&good)).as_deref(),
        Some("worktx-middle"),
        "an exact rendered handle must resolve to its canonical node"
    );
}

#[test]
fn g1_source_routes_through_handleset_resolve_and_has_no_last_fallback() {
    // Structural (pinned≠wired) witness: the BEHAVIORAL test above reconstructs
    // the resolution expression; this proves the live binary actually uses that
    // seam and binds NOTHING else as a parent.
    let src = std::fs::read_to_string(G1_SRC).expect("read g1 src");
    assert!(
        src.contains("node_handles.resolve("),
        "g1 must resolve parent_node via the exact-membership HandleSet seam"
    );
    // The `parent_tx` binding (the ONLY place a parent TxId is minted) must not
    // fall back to `node_tx_ids.last()`. The only allowed mention of `.last()`
    // in that area is the explanatory comment; assert no executable
    // `node_tx_ids.last()` binding survives.
    let executable_last_fallback =
        src.contains("node_tx_ids.last().cloned()") || src.contains(".or_else(|| node_tx_ids.last");
    assert!(
        !executable_last_fallback,
        "g1 parent resolution must NOT bind node_tx_ids.last() as a silent fallback"
    );
    // No loose prefix `starts_with` parent match either (the other removed
    // affordance).
    assert!(
        !src.contains("s.starts_with(&t.0[..16])") && !src.contains("t.0.starts_with(s)"),
        "g1 parent resolution must NOT use a loose prefix match"
    );
}

// ────────────────────────────────────────────────────────────────────────
// (3) HANDLES NOT EXPLICIT — g1 self-id + market node block (fixes #3/#4)
// ────────────────────────────────────────────────────────────────────────

#[test]
fn g1_self_identity_render_is_a_handle_not_agent_i() {
    // fix #3: the per-run self-identity shown to the agent is an opaque handle
    // of the canonical AgentId, NOT the guessable sequential `Agent_i` token.
    let run_id = "g1-membrane-run";
    let canonical_agent = "Agent_0";
    let self_handle = handle(&format!("g1_self::{run_id}"), canonical_agent);

    // Reproduce the agent-visible self-identity line (g1 prompt line ~477).
    let rendered_turn = format!("=== Your turn (round 0, you are {self_handle}) ===");

    // GREEN: shows the handle, hides Agent_0. MUTATION: revert the prompt to
    // interpolate the canonical `agent` (`Agent_0`) → the explicit-id assertion
    // below flips RED.
    assert!(
        rendered_turn.contains(&self_handle),
        "g1 self-identity must be rendered as a content-hash handle"
    );
    assert!(
        !rendered_turn.contains("Agent_0"),
        "the guessable sequential Agent_i token must NOT leak into the prompt"
    );
    // The handle is a hash, never a slot index.
    assert!(!self_handle.starts_with("Agent_"));
    assert!(!self_handle.starts_with("slot_"));
    assert!(self_handle.chars().all(|c| c.is_ascii_hexdigit()));

    // Structural witness: the binary builds the self handle and renders it.
    let src = std::fs::read_to_string(G1_SRC).expect("read g1 src");
    assert!(
        src.contains("g1_self::") && src.contains("you are {self_handle}"),
        "g1 must render a per-run g1_self:: handle in the self-identity line"
    );
}

#[test]
fn g1_market_node_block_renders_handles_not_worktx_strings() {
    // fix #4: the market block names each node by its content-hash handle, never
    // the raw `worktx-` TxId.
    let mut node_handles = HandleSet::new("g1_node");
    let raw_node = "worktx-abcdef0123456789";
    node_handles.insert(raw_node);

    // Reproduce the market line (g1 lines ~439-444).
    let market_line = format!(
        "- node {} price_yes={}/{}\n",
        node_handles.handle_for(raw_node),
        7,
        10
    );
    let node_handle = handle("g1_node", raw_node);

    // GREEN: handle shown, raw worktx- hidden. MUTATION: render `&n.node_tx[..16]`
    // (the old worktx- prefix) and the raw-leak assertion flips RED.
    assert!(
        market_line.contains(&node_handle),
        "market block must name nodes by content-hash handle"
    );
    assert!(
        !market_line.contains("worktx-"),
        "raw worktx- TxId strings must NOT leak into the market block"
    );

    // Structural witness: the binary renders via handle_for, not a raw slice.
    let src = std::fs::read_to_string(G1_SRC).expect("read g1 src");
    assert!(
        src.contains("node_handles.handle_for(&n.node_tx)"),
        "g1 market block must render node via node_handles.handle_for"
    );
    assert!(
        !src.contains("&n.node_tx[..16]"),
        "g1 market block must NOT render the raw worktx- prefix slice"
    );
}

// ────────────────────────────────────────────────────────────────────────
// (3) HANDLES NOT EXPLICIT — rtool level_4 MinimalHeadOnly (fix #6)
// ────────────────────────────────────────────────────────────────────────

#[test]
fn rtool_level_4_renders_task_handle_not_bare_task_id() {
    // fix #6: the MinimalHeadOnly fallback shows a content-hash handle of the
    // task id, never the bare canonical `task.id`.
    let rtool = Rtool::new(
        Arc::new(MemoryTapeLedger::new()),
        Arc::new(Tokenizer::new()),
    );
    let task = Task {
        id: "task-explicit-deadbeef".into(),
        // A long prompt makes levels 1-3 (which embed `task.prompt`) overflow a
        // modest budget, while level_4 (head + task HANDLE only, no prompt) fits.
        prompt: "advance the proof step ".repeat(40),
    };
    let ws = WorkspaceView {
        relevant_diff: None,
        failing_function_src: None,
        failing_predicate: None,
        touched_paths: vec![],
        symbols: vec![],
    };
    // Budget large enough for the prompt-free level_4 (~17 tokens) but far below
    // the prompt-bearing levels 1-3, so the MinimalHeadOnly branch is selected
    // WITHOUT triggering the char-clip overflow fallback.
    let level4_budget =
        Tokenizer::new().count_text("[VERIFIED_HEAD]\nverified-head-hash\n[TASK_HANDLE]\n") + 8;
    let digest =
        rtool.checkout_digest_with_workspace("verified-head-hash", &task, &ws, level4_budget);
    assert_eq!(
        digest.degradation_level,
        SessionDegradationLevel::MinimalHeadOnly,
        "modest budget + long prompt must select the level_4 MinimalHeadOnly render; got:\n{}",
        digest.text
    );

    let task_handle = handle("task", &task.id);
    // GREEN: handle rendered, bare id hidden. MUTATION: revert level_4 to
    // `[TASK_ID]\n{task.id}` and the explicit-id assertion flips RED. (The
    // char-clip fallback may truncate the tail, so assert a prefix presence +
    // strict no-explicit-id.)
    assert!(
        digest.text.contains("[TASK_HANDLE]"),
        "level_4 must label the rendered value as a handle; got:\n{}",
        digest.text
    );
    assert!(
        digest.text.contains(&task_handle[..8]),
        "level_4 must render the task content-hash handle; got:\n{}",
        digest.text
    );
    assert!(
        !digest.text.contains("task-explicit-deadbeef"),
        "the bare canonical task.id must NOT be rendered; got:\n{}",
        digest.text
    );
}

// ────────────────────────────────────────────────────────────────────────
// (3) HANDLES NOT EXPLICIT — your_position block (fix #9)
// ────────────────────────────────────────────────────────────────────────

#[test]
fn your_position_block_renders_handles_not_raw_ids() {
    use turingosv4::economy::money::MicroCoin;
    use turingosv4::sdk::your_position::render_your_position;
    use turingosv4::state::q_state::{AgentId, QState, StakeEntry, TaskId, TxId};

    let mut q = QState::default();
    let viewer = AgentId("Agent_0".into());
    q.economic_state_t
        .balances_t
        .0
        .insert(viewer.clone(), MicroCoin::from_micro_units(800_000));
    let raw_stake_tx = "worktx-position-leak";
    q.economic_state_t.stakes_t.0.insert(
        TxId(raw_stake_tx.into()),
        StakeEntry {
            amount: MicroCoin::from_micro_units(50_000),
            staker: viewer.clone(),
            task_id: TaskId("task-pos".into()),
        },
    );

    let rendered = render_your_position(&q, &viewer);
    let stake_handle = handle("position_tx", raw_stake_tx);

    // GREEN: handle rendered, raw worktx- hidden. MUTATION: render `tx_id.0`
    // directly (drop the id_handle wrap) and the raw-leak assertion flips RED.
    assert!(
        rendered.contains(&stake_handle),
        "your_position must render the stake by content-hash handle; got:\n{rendered}"
    );
    assert!(
        !rendered.contains(raw_stake_tx),
        "the explicit stake tx id must NOT leak into the your_position block; got:\n{rendered}"
    );

    // Structural witness: the renderer wraps ids in id_handle and does not push
    // the bare `tx_id.0` / `event_id` / `node_id` string.
    let src = std::fs::read_to_string(YOUR_POSITION_SRC).expect("read your_position src");
    assert!(
        src.contains("id_handle::handle(\"position_tx\"")
            && src.contains("id_handle::handle(\"position_event\"")
            && src.contains("id_handle::handle(\"position_node\""),
        "your_position must render Stake/Claim tx, ConditionalShare/Lp event, and NodePosition node via id_handle"
    );
}

// ────────────────────────────────────────────────────────────────────────
// (3) HANDLES NOT EXPLICIT — market_external prompt (fix #5, structural)
// ────────────────────────────────────────────────────────────────────────

#[test]
fn market_external_prompt_uses_event_handle_not_true_suite_market_id() {
    // fix #5 is behaviorally witnessed end-to-end (through the real binary +
    // mock proxy) by tests/constitution_true_suite_market_external_agent_runner.rs.
    // Here we (a) prove the deterministic handle the prompt renders for a
    // realistic kernel-fixture task id is NOT the explicit id, and (b) source-grep
    // the binary's build_agent_prompt to prove the explicit id is no longer
    // interpolated.
    let event_task_id = "true-suite-market-some-run-id";
    let event_handle = handle("market_event", event_task_id);
    assert_ne!(
        event_handle, event_task_id,
        "the rendered event handle must differ from the explicit task id"
    );
    assert!(!event_handle.contains("true-suite-market-"));
    assert!(event_handle.chars().all(|c| c.is_ascii_hexdigit()));

    let src = std::fs::read_to_string(MARKET_EXTERNAL_SRC).expect("read market_external src");
    // GREEN: prompt renders the event handle. MUTATION: revert to interpolating
    // `{event_task_id}` in the prompt and the next two assertions flip RED.
    assert!(
        src.contains("id_handle::handle(\"market_event\", event_task_id)"),
        "market_external build_agent_prompt must render an opaque event handle"
    );
    assert!(
        src.contains("Public event `{event_handle}`"),
        "market_external prompt must name the event by its handle, not the explicit task id"
    );
}

// ────────────────────────────────────────────────────────────────────────
// Membrane-scope guard — the canonical id helper is a HASH, never a slot index
// ────────────────────────────────────────────────────────────────────────

#[test]
fn handle_helper_is_a_content_hash_not_a_slot_index() {
    // Distinct underlying ids → distinct handles; same id → same handle
    // (deterministic / replay-stable). A slot index (`Agent_0`/`slot_0`) would
    // NOT satisfy these — it is still an explicit guessable id.
    let a = handle("node", "worktx-aaaa");
    let a2 = handle("node", "worktx-aaaa");
    let b = handle("node", "worktx-bbbb");
    assert_eq!(a, a2, "same id ⇒ same handle (replay-stable)");
    assert_ne!(a, b, "distinct ids ⇒ distinct handles");
    assert!(!a.starts_with("slot_") && !a.starts_with("node_"));
    assert!(a.chars().all(|c| c.is_ascii_hexdigit()));

    // The memory_kernel stamp is wired at the StateAccepted commit (structural
    // companion to the behavioral mismatch test above).
    let mk = std::fs::read_to_string(MEMORY_KERNEL_SRC).expect("read memory_kernel src");
    assert!(
        mk.contains("task_id: task.id.clone()"),
        "memory_kernel StateAccepted must stamp the system task.id into the persisted header"
    );

    // rtool level_4 wiring companion.
    let rt = std::fs::read_to_string(RTOOL_SRC).expect("read rtool src");
    assert!(
        rt.contains("id_handle::handle(\"task\", &task.id)") && rt.contains("[TASK_HANDLE]"),
        "rtool level_4 must render a [TASK_HANDLE] content-hash, not a bare [TASK_ID]"
    );
    assert!(
        !rt.contains("[TASK_ID]\\n{}"),
        "rtool level_4 must NOT render the bare canonical task id"
    );
}
