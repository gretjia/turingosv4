//! arg-taint PROVENANCE DERIVATION — the production-driven classifier that turns
//! a worker proposal on the tape into the [`WtoolCall`] the `arg_taint_v1`
//! hard-gate analyses.
//!
//! ── WHY THIS MODULE EXISTS (forward-wiring) ───────────────────────────────────
//! `arg_taint_v1` (sibling module) is a PROVEN-BUT-NOT-PRODUCTION-DRIVEN check:
//! it correctly flags a tainted-arg → privileged-sink flow, but the live FC1
//! kernel seam (`MemoryKernel::step_forward_with_workspace`) passed it an EMPTY
//! findings set (`&[]` at `memory_kernel.rs`), so no real proposal could ever
//! produce a finding. This module is the missing seam: a pure, deterministic
//! function that reconstructs the labelled [`WtoolCall`] from the proposal/env
//! data ON THE TAPE, so the kernel can run the real analysis.
//!
//! ── WHERE IT LIVES (least-pinned discipline) ──────────────────────────────────
//! Nested as a `#[path]` submodule of the UNPINNED `src/predicate_admission.rs`
//! (genesis pin-count 0), exactly like the sibling `arg_taint.rs`. Zero
//! genesis-pinned file is edited.
//!
//! ── THE PROVENANCE CONTRACT (deterministic, replay-stable) ────────────────────
//! The worker's structured state-update header (the prefix-JSON the kernel
//! already parses for routing) is permitted to carry an OPTIONAL `wtool_call`
//! object describing the wtool call the proposal would commit. `StateUpdate`
//! does NOT use `deny_unknown_fields`, so this object is silently ignored by the
//! routing parse and read SEPARATELY here. The object IS the raw_output on the
//! AgentProposal tape node, so the whole classification replays byte-for-byte.
//!
//! Shape (all fields optional; absence is the COMMON CASE → empty call → no
//! findings → admits exactly as today):
//! ```json
//! {
//!   "schema_version": "tdma-state-update/v1", "status": "Proceed",
//!   "task_id": "...", "action": "...",
//!   "wtool_call": {
//!     "args": [ {"name": "recipient", "value": "...", "source": "external"} ],
//!     "tools": [ {"tool_id": "wallet.transfer", "capability": "economic_wallet",
//!                 "permission_policy": "open", "side_effect_class": "none",
//!                 "determinism_class": "idempotent_write"} ],
//!     "write_keys": ["wallet/agent-7/balance"]
//!   }
//! }
//! ```
//!
//! Provenance classification of each arg's `source` (the single deterministic
//! function `classify_source`):
//!   * `"external"` / `"a2a"` / `"inbound"` / `"network"` / `"untrusted_external"`
//!        → [`ArgTaint::UntrustedExternal`] (the interop_capsule ingress-shield
//!          precedent: inbound A2A / external content is maximally tainted).
//!   * `"tool"` / `"tool_output"` / `"stdout"`
//!        → [`ArgTaint::ToolOutput`].
//!   * `"agent"` / `"llm"` / `"agent_generated"` / `"reasoning"`
//!        → [`ArgTaint::AgentGenerated`].
//!   * `"trusted"` / `"system"` / `"operator"` / `"genesis"` / `"constant"` /
//!     absent / empty
//!        → [`ArgTaint::Trusted`] (the clean default).
//!   * any OTHER non-empty string → [`ArgTaint::UntrustedExternal`] (FAIL CLOSED,
//!     mirroring `ArgTaint::from_tag`: an unrecognised provenance is never
//!     silently trusted).
//!
//! ── NO FALSE REJECTS (binding) ────────────────────────────────────────────────
//! An ordinary proposal has NO `wtool_call` object (or one with no privileged
//! sink). `derive_wtool_call_from_proposal` then returns a [`WtoolCall`] whose
//! `arg_taint_v1` yields ZERO findings, so `decide_admission_with_taint`
//! delegates to the unchanged `decide_admission` and admits identically. A
//! finding is produced ONLY when a genuinely external/tool-provenance arg reaches
//! a privileged sink — the confused-deputy hazard.
//!
//! TRACE_MATRIX FC1a-predicates + FC1b-Q_{t+1}: production provenance derivation
//! feeding the arg-taint admission hard-gate.

use crate::bottom_white::tools::registry::{
    Capability, DeterminismClass, PermissionPolicy, SideEffectClass, ToolMetadata,
};
use crate::predicate_admission::arg_taint::{ArgTaint, LabeledArg, WtoolCall};

/// The free-form key the worker uses to declare a wtool call in its state-update
/// header. Absent in the common case.
/// TRACE_MATRIX FC1a-predicates: wtool-call declaration key.
pub const WTOOL_CALL_HEADER_KEY: &str = "wtool_call";

/// Classify an arg's declared `source` string into its provenance taint label.
/// Pure, total, deterministic; unknown non-empty sources FAIL CLOSED to
/// `UntrustedExternal` (never silently trusted).
/// TRACE_MATRIX FC1a-predicates: deterministic provenance classification.
pub fn classify_source(source: &str) -> ArgTaint {
    match source.trim().to_ascii_lowercase().as_str() {
        // External / inbound boundary — interop_capsule ingress-shield precedent.
        "external" | "a2a" | "inbound" | "network" | "untrusted_external" => {
            ArgTaint::UntrustedExternal
        }
        // Flowed back from a prior tool call's stdout/result.
        "tool" | "tool_output" | "stdout" => ArgTaint::ToolOutput,
        // Produced by the agent's own reasoning this turn.
        "agent" | "llm" | "agent_generated" | "reasoning" => ArgTaint::AgentGenerated,
        // The clean defaults (incl. absent/empty → handled by caller passing "").
        "" | "trusted" | "system" | "operator" | "genesis" | "constant" => ArgTaint::Trusted,
        // Unknown provenance → fail closed (mirror ArgTaint::from_tag).
        _ => ArgTaint::UntrustedExternal,
    }
}

/// Parse a [`Capability`] from a declared tool tag (lower-snake). Unknown tags
/// map to `Capability::Custom(tag)` — they classify as NON-privileged unless the
/// permission/side-effect/determinism fields make them a sink, which is the
/// conservative-yet-non-over-rejecting behaviour.
/// TRACE_MATRIX FC1a-predicates: declared capability parse.
fn parse_capability(tag: &str) -> Capability {
    match tag.trim().to_ascii_lowercase().as_str() {
        "economic_wallet" => Capability::EconomicWallet,
        "proof_validator" => Capability::ProofValidator,
        "network_client" => Capability::NetworkClient,
        "lean_oracle" => Capability::DomainOracle,
        "librarian_board" => Capability::LibrarianBoard,
        "search_tool" => Capability::SearchTool,
        "sandboxed_exec" => Capability::SandboxedExec,
        other => Capability::Custom(other.to_string()),
    }
}

/// Parse a [`PermissionPolicy`] from a declared tag. Default `Open` (the common,
/// NON-privileged-by-permission case).
/// TRACE_MATRIX FC1a-predicates: declared permission parse.
fn parse_permission(tag: &str) -> PermissionPolicy {
    match tag.trim().to_ascii_lowercase().as_str() {
        "system_only" => PermissionPolicy::SystemOnly,
        _ => PermissionPolicy::Open,
    }
}

/// Parse a [`SideEffectClass`] from a declared tag. Default `None` (no
/// external side effect → not a side-effect sink).
/// TRACE_MATRIX FC1a-predicates: declared side-effect parse.
fn parse_side_effect(tag: &str) -> SideEffectClass {
    match tag.trim().to_ascii_lowercase().as_str() {
        "filesystem_read" => SideEffectClass::FilesystemRead,
        "filesystem_write" => SideEffectClass::FilesystemWrite,
        "network" => SideEffectClass::Network,
        "subprocess" => SideEffectClass::Subprocess,
        _ => SideEffectClass::None,
    }
}

/// Parse a [`DeterminismClass`] from a declared tag. Default `Pure`.
/// TRACE_MATRIX FC1a-predicates: declared determinism parse.
fn parse_determinism(tag: &str) -> DeterminismClass {
    match tag.trim().to_ascii_lowercase().as_str() {
        "read_only" => DeterminismClass::ReadOnly,
        "idempotent_write" => DeterminismClass::IdempotentWrite,
        "non_idempotent" => DeterminismClass::NonIdempotent,
        _ => DeterminismClass::Pure,
    }
}

/// Reconstruct a declared [`ToolMetadata`] from a `wtool_call.tools[]` JSON
/// object. Only the fields that affect `classify_tool_sink` are read from the
/// declaration; the rest are filled with neutral defaults (they never affect the
/// sink decision). Deterministic.
/// TRACE_MATRIX FC1a-predicates: declared tool-metadata reconstruction.
fn tool_meta_from_json(obj: &serde_json::Value) -> Option<ToolMetadata> {
    let tool_id = obj.get("tool_id")?.as_str()?.to_string();
    let capability = parse_capability(obj.get("capability").and_then(|v| v.as_str()).unwrap_or(""));
    let permission_policy = parse_permission(
        obj.get("permission_policy")
            .and_then(|v| v.as_str())
            .unwrap_or(""),
    );
    let side_effect_class = parse_side_effect(
        obj.get("side_effect_class")
            .and_then(|v| v.as_str())
            .unwrap_or(""),
    );
    let determinism_class = parse_determinism(
        obj.get("determinism_class")
            .and_then(|v| v.as_str())
            .unwrap_or(""),
    );
    Some(ToolMetadata {
        tool_id,
        version: 0,
        capability,
        permission_policy,
        determinism_class,
        side_effect_class,
        schema: String::new(),
        creator: String::new(),
        code_hash: [0u8; 32],
        test_suite_hash: [0u8; 32],
        reuse_royalty_share_micro: 0,
    })
}

/// Derive the labelled [`WtoolCall`] a proposal would commit, from the worker's
/// raw output (the on-tape state-update header). Pure + deterministic +
/// replay-stable: identical `raw_output` → identical call → identical findings.
///
/// THE COMMON CASE: a proposal with NO `wtool_call` declaration (the overwhelming
/// majority) yields an EMPTY call → `arg_taint_v1` returns no findings → admission
/// is byte-identical to today (no false reject).
///
/// A non-empty declaration is classified per [`classify_source`] (args) +
/// [`tool_meta_from_json`] (tools) + the declared write keys; the sink decision
/// is made by the sibling `arg_taint_v1`.
/// TRACE_MATRIX FC1a-predicates + FC1b-Q_{t+1}: production WtoolCall derivation.
pub fn derive_wtool_call_from_proposal(raw_output: &str) -> WtoolCall {
    // Extract the first balanced JSON object from the raw output — the same
    // header the routing parser reads (we re-extract here so this module needs
    // no extra plumbing from the kernel beyond the raw_output it already holds).
    let Some(obj_str) = crate::state_update::streaming_extract_first_json_object(raw_output) else {
        return WtoolCall::default();
    };
    let Ok(header) = serde_json::from_str::<serde_json::Value>(&obj_str) else {
        return WtoolCall::default();
    };
    let Some(wtool) = header.get(WTOOL_CALL_HEADER_KEY) else {
        // COMMON CASE: no wtool_call declared → empty call → no findings.
        return WtoolCall::default();
    };

    let mut args: Vec<LabeledArg> = Vec::new();
    if let Some(arr) = wtool.get("args").and_then(|v| v.as_array()) {
        for a in arr {
            let name = a
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let value = a.get("value").and_then(|v| v.as_str()).unwrap_or("");
            let source = a.get("source").and_then(|v| v.as_str()).unwrap_or("");
            args.push(LabeledArg::new(
                name,
                value.as_bytes().to_vec(),
                classify_source(source),
            ));
        }
    }

    let mut target_tools: Vec<ToolMetadata> = Vec::new();
    if let Some(arr) = wtool.get("tools").and_then(|v| v.as_array()) {
        for t in arr {
            if let Some(meta) = tool_meta_from_json(t) {
                target_tools.push(meta);
            }
        }
    }

    let mut write_keys: Vec<String> = Vec::new();
    if let Some(arr) = wtool.get("write_keys").and_then(|v| v.as_array()) {
        for k in arr {
            if let Some(s) = k.as_str() {
                write_keys.push(s.to_string());
            }
        }
    }

    WtoolCall {
        args,
        target_tools,
        write_keys,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::predicate_admission::arg_taint::{arg_taint_v1, has_tainted_privileged_flow};

    /// The common-case proposal (no wtool_call) yields an empty call → no
    /// findings → admits as today.
    #[test]
    fn ordinary_proposal_has_no_wtool_call_and_no_findings() {
        let raw = r#"{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"t1","action":"PROCEED"}
---BODY---
the proof is done"#;
        let call = derive_wtool_call_from_proposal(raw);
        assert_eq!(call, WtoolCall::default(), "no wtool_call → empty call");
        assert!(arg_taint_v1(&call).is_empty(), "empty call → no findings");
        assert!(!has_tainted_privileged_flow(&call));
    }

    /// A proposal whose declared wtool_call routes an EXTERNAL-provenance arg into
    /// a wallet (privileged) sink produces a finding.
    #[test]
    fn external_arg_into_wallet_sink_produces_finding() {
        let raw = r#"{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"t1","action":"PROCEED","wtool_call":{"args":[{"name":"recipient","value":"attacker-addr-from-the-web","source":"external"}],"tools":[{"tool_id":"wallet.transfer","capability":"economic_wallet","permission_policy":"open","side_effect_class":"none","determinism_class":"idempotent_write"}],"write_keys":["wallet/agent-7/balance"]}}
---BODY---
pay out"#;
        let call = derive_wtool_call_from_proposal(raw);
        let findings = arg_taint_v1(&call);
        // 1 tainted arg × 2 privileged sinks (wallet tool + wallet/ write key).
        assert_eq!(findings.len(), 2, "got {findings:?}");
        assert!(findings
            .iter()
            .all(|f| f.arg_name == "recipient" && f.arg_taint == ArgTaint::UntrustedExternal));
    }

    /// The SAME declaration but with a TRUSTED-provenance arg produces NO finding
    /// (the provenance label is the sole discriminator).
    #[test]
    fn trusted_arg_into_wallet_sink_is_clean() {
        let raw = r#"{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"t1","action":"PROCEED","wtool_call":{"args":[{"name":"recipient","value":"operator-treasury","source":"trusted"}],"tools":[{"tool_id":"wallet.transfer","capability":"economic_wallet","permission_policy":"open","side_effect_class":"none","determinism_class":"idempotent_write"}],"write_keys":["wallet/agent-7/balance"]}}
---BODY---
pay out"#;
        let call = derive_wtool_call_from_proposal(raw);
        assert!(arg_taint_v1(&call).is_empty(), "trusted arg → no finding");
    }

    /// A tool-output arg flowing only into an UNprivileged sink is clean (no
    /// over-rejection).
    #[test]
    fn tool_output_arg_into_unprivileged_sink_is_clean() {
        let raw = r#"{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"t1","action":"PROCEED","wtool_call":{"args":[{"name":"query","value":"from a prior tool","source":"tool_output"}],"tools":[{"tool_id":"search","capability":"search_tool","permission_policy":"open","side_effect_class":"none","determinism_class":"pure"}],"write_keys":["scratch/notes"]}}
---BODY---
search"#;
        let call = derive_wtool_call_from_proposal(raw);
        assert!(
            arg_taint_v1(&call).is_empty(),
            "tainted arg, unprivileged sink → clean"
        );
    }

    /// Source classification is total + fail-closed on unknown provenance.
    #[test]
    fn classify_source_is_total_and_fail_closed() {
        assert_eq!(classify_source(""), ArgTaint::Trusted);
        assert_eq!(classify_source("trusted"), ArgTaint::Trusted);
        assert_eq!(classify_source("System"), ArgTaint::Trusted);
        assert_eq!(classify_source("agent"), ArgTaint::AgentGenerated);
        assert_eq!(classify_source("tool_output"), ArgTaint::ToolOutput);
        assert_eq!(classify_source("a2a"), ArgTaint::UntrustedExternal);
        assert_eq!(classify_source("EXTERNAL"), ArgTaint::UntrustedExternal);
        // Unknown provenance fails closed to the most-tainted label.
        assert_eq!(
            classify_source("garbage-provenance"),
            ArgTaint::UntrustedExternal
        );
    }

    /// Replay-stability: identical raw_output → identical derived call (no RNG, no
    /// wall-clock, pure function of the bytes on tape).
    #[test]
    fn derivation_is_replay_stable() {
        let raw = r#"{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"t1","action":"PROCEED","wtool_call":{"args":[{"name":"recipient","value":"x","source":"external"}],"tools":[{"tool_id":"wallet.transfer","capability":"economic_wallet"}],"write_keys":["wallet/k"]}}"#;
        let a = derive_wtool_call_from_proposal(raw);
        let b = derive_wtool_call_from_proposal(raw);
        assert_eq!(a, b, "pure function: same input → same output");
    }
}
