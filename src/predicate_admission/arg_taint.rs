//! arg-taint sub-article — value-level taint labelling + tainted-arg → privileged-sink
//! detection for wtool arguments, feeding the single admission oracle.
//!
//! ── WHY THIS LIVES HERE (least-pinned discipline) ─────────────────────────────
//! This module is nested as a `#[path]` submodule of the UNPINNED
//! `src/predicate_admission.rs` (genesis pin-count 0), exactly mirroring the
//! `src/runtime/real5_roles.rs → real5_roles/fc3_*.rs` precedent. Declaring it
//! under the genesis-pinned `src/lib.rs` / `src/runtime/mod.rs` / the pinned
//! predicate registry would force a Trust-Root pin rehash (out-of-scope Class-4).
//! `predicate_admission.rs` is the single shared admission contract both the
//! sequencer `WorkTx` leg and the memory-kernel header leg route through, so the
//! taint findings produced here reach the ONE admission oracle
//! (`decide_admission`) with ZERO pinned-file edits.
//!
//! ── WHAT IT COMPUTES ─────────────────────────────────────────────────────────
//! `arg_taint_v1` is a value-level provenance analysis. Each wtool argument value
//! carries an [`ArgTaint`] label derived from where the value came from:
//!   * `Trusted`            — system/genesis-originated, constant, or operator-set.
//!   * `AgentGenerated`     — produced by the agent's own reasoning this turn.
//!   * `ToolOutput`         — flowed back from a prior tool call's result.
//!   * `UntrustedExternal`  — crossed a network / external boundary (the worst).
//! Labels form a join-semilattice ordered by danger
//! (`Trusted ⊏ AgentGenerated ⊏ ToolOutput ⊏ UntrustedExternal`); propagation
//! through a combining operation takes the JOIN (the most-tainted input wins),
//! the standard conservative taint rule.
//!
//! A "privileged sink" is a wtool whose `ToolMetadata` grants real authority
//! (system-only permission, an economic/oracle/sandboxed-exec capability, or a
//! filesystem-write / network / subprocess / non-idempotent side effect), OR a
//! `write_set` target whose key names a privileged namespace. A
//! [`ArgTaintFinding`] is raised when a TAINTED (non-`Trusted`) argument value
//! flows into such a sink — the canonical confused-deputy hazard.
//!
//! ── TRACE_MATRIX ─────────────────────────────────────────────────────────────
//! TRACE_MATRIX FC1a-predicates: value-level argument taint feeding Pi-p admission.
//! TRACE_MATRIX FC1b-Q_{t+1}: tainted-arg → privileged-sink rejection blocks the
//! head advance (no Q_{t+1}) when the finding is wired into `decide_admission`.

use crate::bottom_white::tools::registry::{
    Capability, DeterminismClass, PermissionPolicy, SideEffectClass, ToolMetadata,
};

/// Value-level taint label for a single wtool argument, ordered by danger.
///
/// The `Ord` derive ranks variants in declaration order, which is deliberately
/// `Trusted < AgentGenerated < ToolOutput < UntrustedExternal`. [`ArgTaint::join`]
/// uses that order so the most-tainted input dominates a combination.
/// TRACE_MATRIX FC1a-predicates: argument provenance label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArgTaint {
    /// System/genesis-originated, constant, or operator-set. The only label that
    /// is NOT considered tainted for sink-flow purposes.
    Trusted,
    /// Produced by the agent's own reasoning this turn. Tainted: the agent is an
    /// untrusted code path with respect to privileged sinks.
    AgentGenerated,
    /// Flowed back from a prior tool call's result. Tainted.
    ToolOutput,
    /// Crossed a network / external-input boundary. The most dangerous label.
    UntrustedExternal,
}

impl ArgTaint {
    /// All variants, lowest-danger first (stable test/iteration order).
    /// TRACE_MATRIX FC1a-predicates: taint-label lattice variants.
    pub const ALL: &'static [ArgTaint] = &[
        ArgTaint::Trusted,
        ArgTaint::AgentGenerated,
        ArgTaint::ToolOutput,
        ArgTaint::UntrustedExternal,
    ];

    /// Stable string tag for tape/receipt serialization (no `Serialize` derive so
    /// callers control the wire form; this is the canonical spelling).
    /// TRACE_MATRIX FC1a-predicates: canonical taint-label tag for receipts.
    pub const fn tag(self) -> &'static str {
        match self {
            ArgTaint::Trusted => "trusted",
            ArgTaint::AgentGenerated => "agent_generated",
            ArgTaint::ToolOutput => "tool_output",
            ArgTaint::UntrustedExternal => "untrusted_external",
        }
    }

    /// Parse a provenance tag back into a label. Unknown tags fail CLOSED to the
    /// most-tainted label (`UntrustedExternal`) — an unrecognised provenance is
    /// treated as maximally dangerous, never silently trusted.
    /// TRACE_MATRIX FC1a-predicates: fail-closed taint-label parse.
    pub fn from_tag(tag: &str) -> ArgTaint {
        match tag.trim() {
            "trusted" => ArgTaint::Trusted,
            "agent_generated" => ArgTaint::AgentGenerated,
            "tool_output" => ArgTaint::ToolOutput,
            "untrusted_external" => ArgTaint::UntrustedExternal,
            _ => ArgTaint::UntrustedExternal,
        }
    }

    /// Lattice join: the more-tainted of two labels. Propagation through any
    /// combining operation (concatenation, formatting, templating) takes the
    /// join, so a single tainted input taints the whole result.
    /// TRACE_MATRIX FC1a-predicates: conservative taint propagation.
    pub fn join(self, other: ArgTaint) -> ArgTaint {
        if self >= other {
            self
        } else {
            other
        }
    }

    /// Fold the join over a sequence of input labels. An empty sequence is
    /// `Trusted` (no tainted input observed).
    /// TRACE_MATRIX FC1a-predicates: fold taint join over inputs.
    pub fn join_all<I: IntoIterator<Item = ArgTaint>>(labels: I) -> ArgTaint {
        labels
            .into_iter()
            .fold(ArgTaint::Trusted, |acc, label| acc.join(label))
    }

    /// Whether this label is tainted for privileged-sink-flow purposes. Only
    /// `Trusted` is clean; every other label is a confused-deputy hazard if it
    /// reaches a privileged sink.
    /// TRACE_MATRIX FC1a-predicates: is-tainted discriminator (only Trusted is clean).
    pub const fn is_tainted(self) -> bool {
        !matches!(self, ArgTaint::Trusted)
    }
}

/// One wtool argument value paired with its provenance label.
/// TRACE_MATRIX FC1a-predicates: a labelled argument value at the admission seam.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabeledArg {
    /// Stable argument name / position key (e.g. `"path"`, `"url"`, `"arg0"`).
    pub name: String,
    /// The raw argument VALUE bytes (read from the proposal payload at admission).
    /// Carried so the finding can echo a redact-safe preview; never trusted as a
    /// substitute for the provenance label.
    pub value: Vec<u8>,
    /// Provenance label for THIS value (post-propagation; the caller is
    /// responsible for joining upstream labels into this one).
    pub taint: ArgTaint,
}

impl LabeledArg {
    /// TRACE_MATRIX FC1a-predicates: a wtool arg value + its provenance taint label.
    pub fn new(name: impl Into<String>, value: impl Into<Vec<u8>>, taint: ArgTaint) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            taint,
        }
    }
}

/// A concrete privileged sink an argument flows into. Either a registered wtool
/// whose metadata grants real authority, or a write-set key naming a privileged
/// namespace.
/// TRACE_MATRIX FC1a-predicates: the privileged-sink target an arg flows into.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrivilegedSink {
    /// A registered tool whose `ToolMetadata` makes it privileged.
    Tool {
        tool_id: String,
        reason: SinkReason,
    },
    /// A write-set key whose name lands in a privileged namespace.
    WriteKey {
        key: String,
        reason: SinkReason,
    },
}

impl PrivilegedSink {
    /// Human-readable sink identifier for findings/receipts.
    /// TRACE_MATRIX FC1a-predicates: sink identifier for the taint receipt.
    pub fn identifier(&self) -> &str {
        match self {
            PrivilegedSink::Tool { tool_id, .. } => tool_id,
            PrivilegedSink::WriteKey { key, .. } => key,
        }
    }

    /// TRACE_MATRIX FC1a-predicates: why this sink is privileged.
    pub fn reason(&self) -> SinkReason {
        match self {
            PrivilegedSink::Tool { reason, .. } | PrivilegedSink::WriteKey { reason, .. } => {
                *reason
            }
        }
    }
}

/// Why a target counts as a privileged sink. Stable, enumerable reasons keep the
/// classifier auditable (no opaque boolean).
/// TRACE_MATRIX FC1a-predicates: privileged-sink classification reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SinkReason {
    /// `permission_policy == SystemOnly` — only the system runtime may invoke.
    SystemOnlyPermission,
    /// Capability grants money / oracle / sandboxed-exec authority.
    PrivilegedCapability,
    /// Side effect writes the filesystem / network / spawns a subprocess.
    ExternalSideEffect,
    /// Non-idempotent writes (forbidden in the deterministic step path).
    NonIdempotentWrite,
    /// Write-set key names a privileged namespace (e.g. `system/`, `wallet/`).
    PrivilegedWriteNamespace,
}

impl SinkReason {
    /// TRACE_MATRIX FC1a-predicates: privileged-sink reason tag (for the receipt).
    pub const fn tag(self) -> &'static str {
        match self {
            SinkReason::SystemOnlyPermission => "system_only_permission",
            SinkReason::PrivilegedCapability => "privileged_capability",
            SinkReason::ExternalSideEffect => "external_side_effect",
            SinkReason::NonIdempotentWrite => "non_idempotent_write",
            SinkReason::PrivilegedWriteNamespace => "privileged_write_namespace",
        }
    }
}

/// Write-set key prefixes that name a privileged namespace. A `write_set` target
/// under any of these is a privileged sink even without a registered tool. Kept
/// as a small explicit table (no magic-string scatter); extend deliberately.
/// TRACE_MATRIX FC1a-predicates: privileged write-namespace table.
pub const PRIVILEGED_WRITE_NAMESPACES: &[&str] = &[
    "system/",
    "wallet/",
    "capability/",
    "trust_root/",
    "registry/",
    "genesis/",
];

/// Classify a tool's metadata as a privileged sink, if it is one. Returns the
/// most-specific [`SinkReason`] (permission > capability > side effect >
/// determinism), or `None` for an unprivileged tool.
/// TRACE_MATRIX FC1a-predicates: tool-metadata privileged-sink classifier.
pub fn classify_tool_sink(meta: &ToolMetadata) -> Option<SinkReason> {
    if meta.permission_policy == PermissionPolicy::SystemOnly {
        return Some(SinkReason::SystemOnlyPermission);
    }
    if matches!(
        meta.capability,
        Capability::EconomicWallet | Capability::LeanOracle | Capability::SandboxedExec
    ) {
        return Some(SinkReason::PrivilegedCapability);
    }
    if matches!(
        meta.side_effect_class,
        SideEffectClass::FilesystemWrite | SideEffectClass::Network | SideEffectClass::Subprocess
    ) {
        return Some(SinkReason::ExternalSideEffect);
    }
    if meta.determinism_class == DeterminismClass::NonIdempotent {
        return Some(SinkReason::NonIdempotentWrite);
    }
    None
}

/// Classify a write-set key as a privileged sink by namespace prefix.
/// TRACE_MATRIX FC1a-predicates: write-namespace privileged-sink classifier.
pub fn classify_write_key_sink(key: &str) -> Option<SinkReason> {
    let trimmed = key.trim_start_matches('/');
    if PRIVILEGED_WRITE_NAMESPACES
        .iter()
        .any(|ns| trimmed.starts_with(ns))
    {
        Some(SinkReason::PrivilegedWriteNamespace)
    } else {
        None
    }
}

/// A single tainted-arg → privileged-sink flow. The presence of ANY finding is
/// the admission-rejection trigger; the fields are the redact-safe receipt the
/// auditor reconstructs from tape.
/// TRACE_MATRIX FC1a-predicates: tainted-arg → privileged-sink finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgTaintFinding {
    /// Argument name whose value is tainted.
    pub arg_name: String,
    /// The provenance label that made it tainted (never `Trusted`).
    pub arg_taint: ArgTaint,
    /// Identifier of the privileged sink the value flows into.
    pub sink: String,
    /// Why the sink is privileged.
    pub sink_reason: SinkReason,
}

impl ArgTaintFinding {
    /// Compact, redact-safe one-line reason for receipts / failure messages.
    /// Carries the labels and sink identity, NOT the raw value bytes.
    /// TRACE_MATRIX FC1a-predicates: redact-safe taint-finding receipt reason.
    pub fn reason(&self) -> String {
        format!(
            "tainted_arg_into_privileged_sink: arg={} taint={} sink={} sink_reason={}",
            self.arg_name,
            self.arg_taint.tag(),
            self.sink,
            self.sink_reason.tag()
        )
    }
}

/// The wtool-call shape `arg_taint_v1` analyzes: the labelled argument values,
/// the tools the call targets (resolved against the live registry), and the
/// write-set targets. All three are available at the admission seam without any
/// pinned-struct field (args from the proposal payload, sinks from the tool
/// registry + write_set).
/// TRACE_MATRIX FC1a-predicates: the wtool-call analysis input.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WtoolCall {
    /// Labelled argument values for this call.
    pub args: Vec<LabeledArg>,
    /// Registered tool metadata for each tool this call targets.
    pub target_tools: Vec<ToolMetadata>,
    /// Write-set keys this call would commit to.
    pub write_keys: Vec<String>,
}

/// `arg_taint_v1` — the value-level taint analysis. Enumerates the privileged
/// sinks of the call (tools + write keys), and for EACH tainted argument raises a
/// finding against EACH privileged sink it can reach. Deterministic: findings are
/// emitted in (arg order × sink order), so the receipt is reconstructable
/// byte-for-byte from tape.
///
/// Conservative flow model: an admitted wtool call routes EVERY argument into
/// EVERY sink the call touches (we cannot, at admission, prove an argument does
/// NOT reach a given sink). This is the fail-closed direction — it never misses a
/// real tainted→privileged flow, at the cost of flagging args that a finer
/// dataflow might exonerate. A future per-arg→per-sink edge map can narrow it.
/// TRACE_MATRIX FC1a-predicates: arg_taint_v1 tainted→privileged analysis.
pub fn arg_taint_v1(call: &WtoolCall) -> Vec<ArgTaintFinding> {
    // Enumerate privileged sinks once (stable order: tools first, then keys).
    let mut sinks: Vec<PrivilegedSink> = Vec::new();
    for meta in &call.target_tools {
        if let Some(reason) = classify_tool_sink(meta) {
            sinks.push(PrivilegedSink::Tool {
                tool_id: meta.tool_id.clone(),
                reason,
            });
        }
    }
    for key in &call.write_keys {
        if let Some(reason) = classify_write_key_sink(key) {
            sinks.push(PrivilegedSink::WriteKey {
                key: key.clone(),
                reason,
            });
        }
    }

    let mut findings: Vec<ArgTaintFinding> = Vec::new();
    if sinks.is_empty() {
        // No privileged sink reachable → no confused-deputy hazard regardless of
        // argument taint. (Tainted args flowing only into unprivileged sinks are
        // fine.)
        return findings;
    }
    for arg in &call.args {
        if !arg.taint.is_tainted() {
            continue;
        }
        for sink in &sinks {
            findings.push(ArgTaintFinding {
                arg_name: arg.name.clone(),
                arg_taint: arg.taint,
                sink: sink.identifier().to_string(),
                sink_reason: sink.reason(),
            });
        }
    }
    findings
}

/// Convenience: does this call carry ANY tainted-arg → privileged-sink flow?
/// The admission-rejection predicate.
/// TRACE_MATRIX FC1a-predicates: admission-rejection trigger.
pub fn has_tainted_privileged_flow(call: &WtoolCall) -> bool {
    !arg_taint_v1(call).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tool(
        id: &str,
        permission: PermissionPolicy,
        capability: Capability,
        side_effect: SideEffectClass,
        determinism: DeterminismClass,
    ) -> ToolMetadata {
        ToolMetadata {
            tool_id: id.to_string(),
            version: 1,
            capability,
            permission_policy: permission,
            determinism_class: determinism,
            side_effect_class: side_effect,
            schema: "test".into(),
            creator: "test".into(),
            code_hash: [0u8; 32],
            test_suite_hash: [0u8; 32],
            reuse_royalty_share_micro: 0,
        }
    }

    fn unprivileged_tool(id: &str) -> ToolMetadata {
        tool(
            id,
            PermissionPolicy::Open,
            Capability::SearchTool,
            SideEffectClass::None,
            DeterminismClass::Pure,
        )
    }

    fn wallet_tool(id: &str) -> ToolMetadata {
        tool(
            id,
            PermissionPolicy::Open,
            Capability::EconomicWallet,
            SideEffectClass::None,
            DeterminismClass::IdempotentWrite,
        )
    }

    #[test]
    fn taint_lattice_is_ordered_by_danger() {
        assert!(ArgTaint::Trusted < ArgTaint::AgentGenerated);
        assert!(ArgTaint::AgentGenerated < ArgTaint::ToolOutput);
        assert!(ArgTaint::ToolOutput < ArgTaint::UntrustedExternal);
    }

    #[test]
    fn join_takes_the_more_tainted() {
        assert_eq!(
            ArgTaint::Trusted.join(ArgTaint::UntrustedExternal),
            ArgTaint::UntrustedExternal
        );
        assert_eq!(
            ArgTaint::ToolOutput.join(ArgTaint::AgentGenerated),
            ArgTaint::ToolOutput
        );
        assert_eq!(ArgTaint::Trusted.join(ArgTaint::Trusted), ArgTaint::Trusted);
        // join is commutative.
        assert_eq!(
            ArgTaint::AgentGenerated.join(ArgTaint::ToolOutput),
            ArgTaint::ToolOutput.join(ArgTaint::AgentGenerated)
        );
    }

    #[test]
    fn join_all_empty_is_trusted() {
        assert_eq!(ArgTaint::join_all(std::iter::empty()), ArgTaint::Trusted);
        assert_eq!(
            ArgTaint::join_all([ArgTaint::Trusted, ArgTaint::ToolOutput, ArgTaint::Trusted]),
            ArgTaint::ToolOutput
        );
    }

    #[test]
    fn only_trusted_is_clean() {
        assert!(!ArgTaint::Trusted.is_tainted());
        assert!(ArgTaint::AgentGenerated.is_tainted());
        assert!(ArgTaint::ToolOutput.is_tainted());
        assert!(ArgTaint::UntrustedExternal.is_tainted());
    }

    #[test]
    fn from_tag_unknown_fails_closed_to_most_tainted() {
        assert_eq!(ArgTaint::from_tag("trusted"), ArgTaint::Trusted);
        assert_eq!(
            ArgTaint::from_tag("garbage-provenance"),
            ArgTaint::UntrustedExternal
        );
        // round-trips.
        for label in ArgTaint::ALL {
            assert_eq!(ArgTaint::from_tag(label.tag()), *label);
        }
    }

    #[test]
    fn system_only_tool_is_privileged_sink() {
        let meta = tool(
            "system.sign",
            PermissionPolicy::SystemOnly,
            Capability::ProofValidator,
            SideEffectClass::None,
            DeterminismClass::Pure,
        );
        assert_eq!(
            classify_tool_sink(&meta),
            Some(SinkReason::SystemOnlyPermission)
        );
    }

    #[test]
    fn wallet_capability_is_privileged_sink() {
        assert_eq!(
            classify_tool_sink(&wallet_tool("wallet.pay")),
            Some(SinkReason::PrivilegedCapability)
        );
    }

    #[test]
    fn filesystem_write_is_privileged_sink() {
        let meta = tool(
            "fs.write",
            PermissionPolicy::Open,
            Capability::Custom("fs".into()),
            SideEffectClass::FilesystemWrite,
            DeterminismClass::IdempotentWrite,
        );
        assert_eq!(classify_tool_sink(&meta), Some(SinkReason::ExternalSideEffect));
    }

    #[test]
    fn pure_search_tool_is_not_a_sink() {
        assert_eq!(classify_tool_sink(&unprivileged_tool("search")), None);
    }

    #[test]
    fn write_key_namespace_classification() {
        assert_eq!(
            classify_write_key_sink("wallet/agent-7/balance"),
            Some(SinkReason::PrivilegedWriteNamespace)
        );
        assert_eq!(
            classify_write_key_sink("/system/keypair"),
            Some(SinkReason::PrivilegedWriteNamespace)
        );
        assert_eq!(classify_write_key_sink("scratch/notes"), None);
    }

    #[test]
    fn tainted_arg_into_privileged_tool_is_flagged() {
        let call = WtoolCall {
            args: vec![LabeledArg::new(
                "url",
                b"http://evil.example/x".to_vec(),
                ArgTaint::UntrustedExternal,
            )],
            target_tools: vec![wallet_tool("wallet.pay")],
            write_keys: vec![],
        };
        let findings = arg_taint_v1(&call);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].arg_name, "url");
        assert_eq!(findings[0].arg_taint, ArgTaint::UntrustedExternal);
        assert_eq!(findings[0].sink, "wallet.pay");
        assert_eq!(findings[0].sink_reason, SinkReason::PrivilegedCapability);
        assert!(has_tainted_privileged_flow(&call));
    }

    #[test]
    fn tainted_arg_into_privileged_write_key_is_flagged() {
        let call = WtoolCall {
            args: vec![LabeledArg::new(
                "amount",
                b"999999".to_vec(),
                ArgTaint::ToolOutput,
            )],
            target_tools: vec![],
            write_keys: vec!["wallet/agent-7/balance".to_string()],
        };
        let findings = arg_taint_v1(&call);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].sink, "wallet/agent-7/balance");
        assert_eq!(findings[0].sink_reason, SinkReason::PrivilegedWriteNamespace);
    }

    #[test]
    fn trusted_arg_into_privileged_sink_is_clean() {
        // POSITIVE CONTROL: a Trusted arg into a wallet sink must NOT be flagged.
        let call = WtoolCall {
            args: vec![LabeledArg::new("amount", b"100".to_vec(), ArgTaint::Trusted)],
            target_tools: vec![wallet_tool("wallet.pay")],
            write_keys: vec!["wallet/agent-7/balance".to_string()],
        };
        assert!(arg_taint_v1(&call).is_empty());
        assert!(!has_tainted_privileged_flow(&call));
    }

    #[test]
    fn tainted_arg_into_unprivileged_sink_is_clean() {
        // POSITIVE CONTROL: a tainted arg flowing only into an unprivileged tool
        // (pure search, no write set) is NOT a confused-deputy hazard.
        let call = WtoolCall {
            args: vec![LabeledArg::new(
                "query",
                b"untrusted text".to_vec(),
                ArgTaint::UntrustedExternal,
            )],
            target_tools: vec![unprivileged_tool("search")],
            write_keys: vec!["scratch/notes".to_string()],
        };
        assert!(arg_taint_v1(&call).is_empty());
    }

    #[test]
    fn findings_are_deterministic_arg_times_sink() {
        // Two tainted args × two privileged sinks = 4 findings, in arg×sink order.
        let call = WtoolCall {
            args: vec![
                LabeledArg::new("a", b"x".to_vec(), ArgTaint::ToolOutput),
                LabeledArg::new("b", b"y".to_vec(), ArgTaint::AgentGenerated),
            ],
            target_tools: vec![wallet_tool("wallet.pay")],
            write_keys: vec!["system/flag".to_string()],
        };
        let findings = arg_taint_v1(&call);
        assert_eq!(findings.len(), 4);
        // arg "a" first against both sinks, then arg "b".
        assert_eq!(findings[0].arg_name, "a");
        assert_eq!(findings[0].sink, "wallet.pay");
        assert_eq!(findings[1].arg_name, "a");
        assert_eq!(findings[1].sink, "system/flag");
        assert_eq!(findings[2].arg_name, "b");
        assert_eq!(findings[3].arg_name, "b");
    }
}
