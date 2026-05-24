// Tier 0: Append-only tape with tamper detection
// Constitutional basis: Law 1 (Information is Free), Magna Carta
// V3 lessons: V3L-09 (no silent failure), V3L-24 (no /tmp data loss)

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fmt;

// ── Core types ──────────────────────────────────────────────────

/// Unique identifier for a tape node.
pub type NodeId = String;

/// A single node on the append-only tape (DAG).
/// Constitutional basis: Art. I — all signals quantized through this structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub author: String,
    pub payload: String,
    pub citations: Vec<NodeId>,
    pub created_at: u64,
    pub completion_tokens: u32,
}

/// The append-only DAG tape.
/// Invariant: once appended, a node is NEVER modified or removed.
/// V3L-24: all data persisted to experiments/, never /tmp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tape {
    nodes: HashMap<NodeId, Node>,
    reverse_citations: HashMap<NodeId, Vec<NodeId>>,
    time_arrow: Vec<NodeId>,
}

impl Tape {
    pub fn new() -> Self {
        Tape {
            nodes: HashMap::new(),
            reverse_citations: HashMap::new(),
            time_arrow: Vec::new(),
        }
    }

    /// Append a node to the tape.
    /// Returns Err if:
    /// - Node ID already exists (V6 spacetime paradox protection)
    /// - Any cited parent does not exist (V5 causality defense)
    /// V3L-09: never silently fail — always return explicit Result.
    pub fn append(&mut self, node: Node) -> Result<(), TapeError> {
        // V6: reject duplicate IDs
        if self.nodes.contains_key(&node.id) {
            return Err(TapeError::DuplicateId(node.id.clone()));
        }

        // V5: reject citations to non-existent parents
        for parent_id in &node.citations {
            if !self.nodes.contains_key(parent_id) {
                return Err(TapeError::DanglingCitation {
                    node_id: node.id.clone(),
                    missing_parent: parent_id.clone(),
                });
            }
        }

        // Update reverse citations
        for parent_id in &node.citations {
            self.reverse_citations
                .entry(parent_id.clone())
                .or_default()
                .push(node.id.clone());
        }

        // Append to time arrow
        self.time_arrow.push(node.id.clone());

        // Insert node
        self.nodes.insert(node.id.clone(), node);

        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn children(&self, id: &str) -> &[NodeId] {
        self.reverse_citations
            .get(id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn time_arrow(&self) -> &[NodeId] {
        &self.time_arrow
    }

    pub fn nodes(&self) -> &HashMap<NodeId, Node> {
        &self.nodes
    }

    /// Trace the PRIMARY ancestor chain from a node back to root.
    /// Follows only the first citation (primary parent) at each step.
    /// This is by design: in a proof DAG, the primary chain is the proof path.
    /// Multi-parent merges are represented but not followed by this function.
    pub fn trace_ancestors(&self, node_id: &str) -> Vec<NodeId> {
        let mut path = Vec::new();
        let mut current = node_id.to_string();
        let mut visited = std::collections::HashSet::new();

        while let Some(node) = self.nodes.get(&current) {
            if !visited.insert(current.clone()) {
                break; // cycle protection (should never happen in a DAG)
            }
            path.push(current.clone());
            // Follow first citation (primary parent in proof chain)
            if let Some(parent) = node.citations.first() {
                current = parent.clone();
            } else {
                break; // root node
            }
        }

        path.reverse();
        path
    }
}

impl Default for Tape {
    fn default() -> Self {
        Self::new()
    }
}

// ── Ledger event log ────────────────────────────────────────────

/// Event types for the append-only event ledger.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    RunStart,
    Append,
    RunEnd,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventType::RunStart => write!(f, "RunStart"),
            EventType::Append => write!(f, "Append"),
            EventType::RunEnd => write!(f, "RunEnd"),
        }
    }
}

/// A single ledger event with hash-chain tamper detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEvent {
    pub seq: u64,
    pub event_type: EventType,
    pub node_id: Option<String>,
    pub agent: Option<String>,
    pub detail: Option<String>,
    pub prev_hash: Option<String>,
    pub hash: String,
}

impl LedgerEvent {
    /// Compute the SHA-256 hash for this event. Covers ALL fields.
    fn compute_hash(
        seq: u64,
        event_type: &EventType,
        node_id: &Option<String>,
        agent: &Option<String>,
        detail: &Option<String>,
        prev_hash: &Option<String>,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(seq.to_le_bytes());
        hasher.update(format!("{}", event_type).as_bytes());
        if let Some(nid) = node_id {
            hasher.update(nid.as_bytes());
        }
        if let Some(a) = agent {
            hasher.update(a.as_bytes());
        }
        if let Some(d) = detail {
            hasher.update(d.as_bytes());
        }
        if let Some(ph) = prev_hash {
            hasher.update(ph.as_bytes());
        }
        format!("{:x}", hasher.finalize())
    }
}

/// Append-only event ledger with tamper detection via hash chain.
/// Mechanism/policy separation: ledger writes events, other modules query.
pub struct Ledger {
    events: Vec<LedgerEvent>,
    seq: u64,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger {
            events: Vec::new(),
            seq: 0,
        }
    }

    /// Append an event. Returns the event with computed hash.
    /// V3L-09: returns Result, never silently fails.
    pub fn append(
        &mut self,
        event_type: EventType,
        node_id: Option<String>,
        agent: Option<String>,
        detail: Option<String>,
    ) -> Result<&LedgerEvent, TapeError> {
        let prev_hash = self.events.last().map(|e| e.hash.clone());
        let hash =
            LedgerEvent::compute_hash(self.seq, &event_type, &node_id, &agent, &detail, &prev_hash);

        let event = LedgerEvent {
            seq: self.seq,
            event_type,
            node_id,
            agent,
            detail,
            prev_hash,
            hash,
        };

        self.events.push(event);
        self.seq += 1;

        Ok(self.events.last().unwrap())
    }

    /// Verify the entire hash chain. Returns Ok(()) if tamper-free.
    /// Also checks that no events were truncated (seq must reach self.seq - 1).
    pub fn verify(&self) -> Result<(), TapeError> {
        // Check for truncation: expected count must match actual
        if !self.events.is_empty() {
            let expected_last_seq = self.seq - 1;
            let actual_last_seq = self.events.last().unwrap().seq;
            if actual_last_seq != expected_last_seq {
                return Err(TapeError::LedgerCorruption(format!(
                    "Truncation detected: expected last seq {}, got {}",
                    expected_last_seq, actual_last_seq
                )));
            }
        }
        for (i, event) in self.events.iter().enumerate() {
            // Check sequence monotonicity
            if event.seq != i as u64 {
                return Err(TapeError::LedgerCorruption(format!(
                    "seq mismatch at index {}: expected {}, got {}",
                    i, i, event.seq
                )));
            }

            // Check prev_hash linkage
            let expected_prev = if i == 0 {
                None
            } else {
                Some(self.events[i - 1].hash.clone())
            };
            if event.prev_hash != expected_prev {
                return Err(TapeError::LedgerCorruption(format!(
                    "prev_hash mismatch at seq {}",
                    event.seq
                )));
            }

            // Recompute and verify hash
            let recomputed = LedgerEvent::compute_hash(
                event.seq,
                &event.event_type,
                &event.node_id,
                &event.agent,
                &event.detail,
                &event.prev_hash,
            );
            if event.hash != recomputed {
                return Err(TapeError::LedgerCorruption(format!(
                    "hash mismatch at seq {}",
                    event.seq
                )));
            }
        }
        Ok(())
    }

    pub fn events(&self) -> &[LedgerEvent] {
        &self.events
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl Default for Ledger {
    fn default() -> Self {
        Self::new()
    }
}

// ── Errors ──────────────────────────────────────────────────────

/// V3L-09: explicit error types, never silent Option::None.
#[derive(Debug, Clone)]
pub enum TapeError {
    DuplicateId(String),
    DanglingCitation {
        node_id: String,
        missing_parent: String,
    },
    LedgerCorruption(String),
}

impl fmt::Display for TapeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TapeError::DuplicateId(id) => write!(f, "Duplicate node ID: {}", id),
            TapeError::DanglingCitation {
                node_id,
                missing_parent,
            } => write!(
                f,
                "Node {} cites non-existent parent {}",
                node_id, missing_parent
            ),
            TapeError::LedgerCorruption(msg) => write!(f, "Ledger corruption: {}", msg),
        }
    }
}

impl std::error::Error for TapeError {}

// ── TDMA-Bounded-RC1 substrate ──────────────────────────────────
//
// TRACE_MATRIX TDMA-RC1-Atom1: Art. 0.4 Path A semantic version-control substrate.
// FC1a (tape_t), FC1b (Q_{t+1}), FC2 (Q_0 substrate), FC3 (replay determinism).
// On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md
//
// The trait `ImmutableTapeLedger` is the seam between Path A (`MemoryTapeLedger`,
// this RC1) and Path B (`GitTapeLedger`, future Phase E libgit2 substrate per
// constitution.md Art. 0.4 Path B obligation; see PHASE_E_TODO_TDMA.md).
// Without the trait, Phase E migration would touch every call site instead of
// one boundary — this addresses Karpathy-audit K10 (single-impl trait justification).

use std::hash::{Hash, Hasher};

/// Tape node kind discriminator (directive §3.1).
/// TRACE_MATRIX FC1a-tape_t: Discriminates the kind of every TapeNode on the
/// TDMA tape (StateAccepted advances verified_head; AgentProposal/RetryBeliefState/
/// Escalation enter tape with verified=false).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NodeKind {
    StateAccepted,
    AgentProposal,
    RetryBeliefState,
    CharterCore,
    PromptAssembly,
    Escalation,
}

/// Identifier for one retry-cycle scope — same `run_id` + `task_id` + `verified_parent`
/// share an AttemptScope. RC1 key fix: scope is a first-class field on TapeNode,
/// NEVER hidden inside payload (directive §2.1 / KILL-tdma-3).
/// TRACE_MATRIX FC1a-tape_t: First-class scope metadata; enables per-scope
/// retry-count via count_nodes() and BBS lineage via nodes_by_scope index.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AttemptScope {
    pub run_id: String,
    pub task_id: String,
    pub verified_parent: String,
}

// Hash impl mirrors PartialEq so AttemptScope can key TapeIndexes.nodes_by_scope.
impl Hash for AttemptScope {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.run_id.hash(state);
        self.task_id.hash(state);
        self.verified_parent.hash(state);
    }
}

/// RC1 tape node (directive §3.1). Distinct from legacy `Node` — TDMA-Bounded kernel
/// operates exclusively on TapeNode; legacy paths continue to use Node.
/// TRACE_MATRIX FC1a-tape_t: Q_t.tape_t element. Every externalized LLM attempt
/// produces exactly one TapeNode (verified=false on failure; verified=true on
/// StateAccepted advance).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TapeNode {
    pub id: String,
    pub hash: String,
    pub kind: NodeKind,
    pub verified: bool,
    pub parent: Option<String>,
    pub scope: Option<AttemptScope>,
    pub attempt_ordinal: Option<u32>,
    pub reject_class: Option<String>,
    pub token_count: Option<usize>,
    pub payload: serde_json::Value,
    pub created_at_unix_ms: u64,
}

impl TapeNode {
    /// Compute canonical content hash. Covers all fields except `hash` itself.
    /// TRACE_MATRIX FC1a-tape_t: Content-addressed hash anchors each TapeNode;
    /// the trait `ImmutableTapeLedger::commit` calls this so callers cannot forge.
    pub fn compute_hash(
        id: &str,
        kind: &NodeKind,
        verified: bool,
        parent: &Option<String>,
        scope: &Option<AttemptScope>,
        attempt_ordinal: &Option<u32>,
        reject_class: &Option<String>,
        token_count: &Option<usize>,
        payload: &serde_json::Value,
        created_at_unix_ms: u64,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(id.as_bytes());
        hasher.update(format!("{:?}", kind).as_bytes());
        hasher.update([verified as u8]);
        if let Some(p) = parent {
            hasher.update(p.as_bytes());
        }
        if let Some(s) = scope {
            hasher.update(s.run_id.as_bytes());
            hasher.update(s.task_id.as_bytes());
            hasher.update(s.verified_parent.as_bytes());
        }
        if let Some(o) = attempt_ordinal {
            hasher.update(o.to_le_bytes());
        }
        if let Some(rc) = reject_class {
            hasher.update(rc.as_bytes());
        }
        if let Some(tc) = token_count {
            hasher.update(tc.to_le_bytes());
        }
        // payload is canonical-serialized JSON
        hasher.update(serde_json::to_string(payload).unwrap_or_default().as_bytes());
        hasher.update(created_at_unix_ms.to_le_bytes());
        format!("{:x}", hasher.finalize())
    }
}

/// Request to commit a new TapeNode (directive §3.2). `id` and `hash` are computed
/// by the ledger so the caller cannot forge them.
/// TRACE_MATRIX FC1b-Q_{t+1}: The structured input that the wtool path takes when
/// appending to tape_t. Hash is forgery-resistant — caller cannot precompute.
#[derive(Debug, Clone)]
pub struct CommitRequest {
    pub kind: NodeKind,
    pub verified: bool,
    pub parent: Option<String>,
    pub scope: Option<AttemptScope>,
    pub attempt_ordinal: Option<u32>,
    pub reject_class: Option<String>,
    pub token_count: Option<usize>,
    pub payload: serde_json::Value,
}

/// Index structures over a TDMA tape (directive §3.1).
/// TRACE_MATRIX FC1a-tape_t: Derived O(1) views over the canonical tape — by_hash
/// for lookup, nodes_by_scope for retry-count, verified_head separated from
/// ledger_tail so failures cannot advance the canonical world line.
#[derive(Debug, Clone, Default)]
pub struct TapeIndexes {
    pub by_hash: HashMap<String, TapeNode>,
    pub children_by_parent: HashMap<String, Vec<String>>,
    pub nodes_by_scope: HashMap<AttemptScope, Vec<String>>,
    pub verified_head: String,
    pub ledger_tail: String,
}

// ── RetryBeliefState schema (directive §4.2; serialized into TapeNode.payload
//    when kind=RetryBeliefState). Lives here in ledger.rs because it is a
//    tape-canonical object, not a transient kernel value. ────────────────

/// Stable identifier for one failure shape (directive §4.2).
/// TRACE_MATRIX FC1a-tape_t: Stable equality key for "is this the same failure
/// shape as last attempt?" — drives zero_gain_streak update logic.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FailureSignature {
    pub reject_class: String,
    pub failed_predicate: String,
    pub root_cause: String,
}

/// One retained retry rule (directive §4.2). Higher priority survives eviction longer.
/// TRACE_MATRIX FC1a-tape_t: A single causal constraint accumulated from a prior
/// failed attempt; priority drives eviction order when BBS exceeds B_D budget.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RetryConstraint {
    pub id: String,
    pub rule: String,
    pub priority: u8,
    pub source_attempt: u32,
    pub evidence_hash: String,
}

/// Pointer triplet to evidence on tape + CAS (directive §4.2). Never holds raw stderr
/// — only its sha256 — so the BBS cannot leak high-entropy payload into prompt.
/// TRACE_MATRIX FC1a-tape_t + KILL-tdma-1: Pointer-only design enforces "raw stderr
/// never enters prompt" at the type system level; BBS carries hashes, never bytes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidencePointer {
    pub evidence_node_hash: String,
    pub raw_stderr_sha256: String,
    pub trace_view_sha256: String,
}

/// Audit record of constraints dropped by priority eviction (directive §4.2).
/// TRACE_MATRIX FC3-replay: Preserves audit trail of WHY a constraint was dropped
/// during BBS compression; lets replay reconstruct the full eviction history.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvictedConstraint {
    pub id: String,
    pub priority: u8,
    pub reason: String,
}

/// RetryBeliefState (directive §4.2). schema_version pinned to "tdma-bbs/v1".
/// Note: f64 `information_gain` prevents `Eq`; PartialEq is sufficient (tape canonicity
/// reads the serialized form, not in-memory equality).
/// TRACE_MATRIX FC1a-tape_t + KILL-tdma-2: The complete tape-canonical belief state.
/// Lives ONLY in TapeNode.payload (kind=RetryBeliefState); never in a mutable sidecar.
/// Replay reconstructs BBS by reading the latest such node for a given AttemptScope.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetryBeliefState {
    pub schema_version: String,
    pub scope: AttemptScope,
    pub failure_signature: FailureSignature,
    pub constraints: Vec<RetryConstraint>,
    pub evidence: EvidencePointer,
    pub zero_gain_streak: u32,
    pub information_gain: f64,
    pub evicted: Vec<EvictedConstraint>,
}

/// Read/write contract for the TDMA tape (directive §3.2).
///
/// `derive_latest_belief_state_from_tape` MUST be a pure function reading the tape
/// only — never a memory cache — so replay can reconstruct BBS from tape alone
/// (Gate 5 invariant; constitution Art. 0.2 tape canonicity).
/// TRACE_MATRIX FC1a-rtool / FC1b-wtool: The contract the kernel calls into. The
/// trait IS the seam between Path A (MemoryTapeLedger now) and Path B (Phase E
/// GitTapeLedger / libgit2) per constitution Art. 0.4 — addresses Karpathy K10
/// single-impl trait concern via planned second concrete impl.
pub trait ImmutableTapeLedger {
    fn get_verified_head(&self) -> String;
    fn set_verified_head(&mut self, new_head: String);

    fn commit(&mut self, req: CommitRequest) -> TapeNode;

    fn count_nodes(
        &self,
        kind: Option<NodeKind>,
        verified: Option<bool>,
        parent: Option<&str>,
        scope: Option<&AttemptScope>,
    ) -> usize;

    fn latest_node(&self, kind: NodeKind, scope: &AttemptScope) -> Option<TapeNode>;

    /// TRACE_MATRIX FC3-replay: Dump every node in the ledger as `(hash, node)`
    /// pairs for evidence-writeout (chaintape.jsonl). Added Atom 20 to support
    /// the run_proof_with_ledger generic that needs to serialize tape contents
    /// without depending on a concrete impl's internal indexes structure.
    fn dump_all_nodes(&self) -> Vec<(String, TapeNode)>;

    /// PURE FUNCTION — reads tape only. No sidecar, no memory cache.
    fn derive_latest_belief_state_from_tape(
        &self,
        scope: &AttemptScope,
    ) -> Option<RetryBeliefState>;
}

/// Default in-memory concrete tape ledger (Path A per Art. 0.4).
/// Phase E will introduce `GitTapeLedger` as a second impl behind the same trait.
/// TRACE_MATRIX FC1a-tape_t (Path-A impl): Concrete tape backed by Vec/HashMap;
/// satisfies the Art. 0.4 semantic version-control substrate requirement for RC1.
#[derive(Debug, Clone, Default)]
pub struct MemoryTapeLedger {
    pub indexes: TapeIndexes,
    /// Monotonic counter for node ids.
    next_seq: u64,
    /// Monotonic clock counter used when `created_at_unix_ms` is not supplied
    /// (test/CI default). Wall clock is used in production callers.
    clock: u64,
}

impl MemoryTapeLedger {
    /// TRACE_MATRIX FC1a-tape_t: Empty Path-A tape constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Test-only: control the clock for determinism.
    /// TRACE_MATRIX FC1a-tape_t: Deterministic-clock variant for replay reproducibility.
    pub fn with_clock(mut self, start: u64) -> Self {
        self.clock = start;
        self
    }
}

impl ImmutableTapeLedger for MemoryTapeLedger {
    fn get_verified_head(&self) -> String {
        self.indexes.verified_head.clone()
    }

    fn set_verified_head(&mut self, new_head: String) {
        self.indexes.verified_head = new_head;
    }

    fn commit(&mut self, req: CommitRequest) -> TapeNode {
        self.next_seq += 1;
        self.clock += 1;
        let id = format!("tn-{}", self.next_seq);
        let created_at_unix_ms = self.clock;
        let hash = TapeNode::compute_hash(
            &id,
            &req.kind,
            req.verified,
            &req.parent,
            &req.scope,
            &req.attempt_ordinal,
            &req.reject_class,
            &req.token_count,
            &req.payload,
            created_at_unix_ms,
        );

        let node = TapeNode {
            id: id.clone(),
            hash: hash.clone(),
            kind: req.kind,
            verified: req.verified,
            parent: req.parent.clone(),
            scope: req.scope.clone(),
            attempt_ordinal: req.attempt_ordinal,
            reject_class: req.reject_class,
            token_count: req.token_count,
            payload: req.payload,
            created_at_unix_ms,
        };

        // Update indexes (append-only; no mutation of existing entries).
        if let Some(parent) = &req.parent {
            self.indexes
                .children_by_parent
                .entry(parent.clone())
                .or_default()
                .push(hash.clone());
        }
        if let Some(scope) = &req.scope {
            self.indexes
                .nodes_by_scope
                .entry(scope.clone())
                .or_default()
                .push(hash.clone());
        }
        self.indexes.by_hash.insert(hash.clone(), node.clone());
        self.indexes.ledger_tail = hash;

        node
    }

    fn count_nodes(
        &self,
        kind: Option<NodeKind>,
        verified: Option<bool>,
        parent: Option<&str>,
        scope: Option<&AttemptScope>,
    ) -> usize {
        self.indexes
            .by_hash
            .values()
            .filter(|n| kind.as_ref().map(|k| &n.kind == k).unwrap_or(true))
            .filter(|n| verified.map(|v| n.verified == v).unwrap_or(true))
            .filter(|n| parent.map(|p| n.parent.as_deref() == Some(p)).unwrap_or(true))
            .filter(|n| scope.map(|s| n.scope.as_ref() == Some(s)).unwrap_or(true))
            .count()
    }

    fn latest_node(&self, kind: NodeKind, scope: &AttemptScope) -> Option<TapeNode> {
        self.indexes
            .nodes_by_scope
            .get(scope)
            .and_then(|hashes| {
                hashes
                    .iter()
                    .rev()
                    .find_map(|h| self.indexes.by_hash.get(h))
                    .filter(|n| n.kind == kind)
                    .cloned()
            })
            .or_else(|| {
                // Fallback: scan all (rare path; used when scope not indexed)
                self.indexes
                    .by_hash
                    .values()
                    .filter(|n| n.kind == kind && n.scope.as_ref() == Some(scope))
                    .max_by_key(|n| n.created_at_unix_ms)
                    .cloned()
            })
    }

    fn derive_latest_belief_state_from_tape(
        &self,
        scope: &AttemptScope,
    ) -> Option<RetryBeliefState> {
        // PURE: walk only the tape. No sidecar read. Find the highest-ordinal
        // RetryBeliefState committed under this scope and deserialize from
        // its payload field.
        let latest = self.latest_node(NodeKind::RetryBeliefState, scope)?;
        serde_json::from_value(latest.payload).ok()
    }

    fn dump_all_nodes(&self) -> Vec<(String, TapeNode)> {
        self.indexes
            .by_hash
            .iter()
            .map(|(h, n)| (h.clone(), n.clone()))
            .collect()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_node(id: &str, author: &str, payload: &str, citations: Vec<&str>) -> Node {
        Node {
            id: id.to_string(),
            author: author.to_string(),
            payload: payload.to_string(),
            citations: citations.into_iter().map(|s| s.to_string()).collect(),
            created_at: 0,
            completion_tokens: 0,
        }
    }

    // ── Tape tests ──

    #[test]
    fn test_tape_append_root_node() {
        let mut tape = Tape::new();
        let node = make_node("root", "Agent_0", "initial step", vec![]);
        assert!(tape.append(node).is_ok());
        assert_eq!(tape.len(), 1);
        assert!(tape.get("root").is_some());
    }

    #[test]
    fn test_tape_append_with_valid_citation() {
        let mut tape = Tape::new();
        tape.append(make_node("n1", "A0", "step 1", vec![]))
            .unwrap();
        tape.append(make_node("n2", "A1", "step 2", vec!["n1"]))
            .unwrap();
        assert_eq!(tape.len(), 2);
        assert_eq!(tape.children("n1"), &["n2"]);
    }

    #[test]
    fn test_tape_reject_duplicate_id() {
        // V6 spacetime paradox protection
        let mut tape = Tape::new();
        tape.append(make_node("n1", "A0", "step 1", vec![]))
            .unwrap();
        let result = tape.append(make_node("n1", "A1", "step 2", vec![]));
        assert!(matches!(result, Err(TapeError::DuplicateId(_))));
    }

    #[test]
    fn test_tape_reject_dangling_citation() {
        // V5 causality defense
        let mut tape = Tape::new();
        let result = tape.append(make_node("n1", "A0", "step 1", vec!["nonexistent"]));
        assert!(matches!(result, Err(TapeError::DanglingCitation { .. })));
    }

    #[test]
    fn test_tape_time_arrow_ordering() {
        let mut tape = Tape::new();
        tape.append(make_node("a", "A0", "first", vec![])).unwrap();
        tape.append(make_node("b", "A1", "second", vec!["a"]))
            .unwrap();
        tape.append(make_node("c", "A0", "third", vec!["b"]))
            .unwrap();
        assert_eq!(tape.time_arrow(), &["a", "b", "c"]);
    }

    #[test]
    fn test_tape_trace_ancestors() {
        let mut tape = Tape::new();
        tape.append(make_node("root", "A0", "root", vec![]))
            .unwrap();
        tape.append(make_node("mid", "A1", "mid", vec!["root"]))
            .unwrap();
        tape.append(make_node("leaf", "A0", "leaf", vec!["mid"]))
            .unwrap();
        let path = tape.trace_ancestors("leaf");
        assert_eq!(path, vec!["root", "mid", "leaf"]);
    }

    #[test]
    fn test_tape_dag_branching() {
        let mut tape = Tape::new();
        tape.append(make_node("root", "A0", "root", vec![]))
            .unwrap();
        tape.append(make_node("b1", "A1", "branch 1", vec!["root"]))
            .unwrap();
        tape.append(make_node("b2", "A2", "branch 2", vec!["root"]))
            .unwrap();
        assert_eq!(tape.children("root").len(), 2);
    }

    #[test]
    fn test_tape_empty() {
        let tape = Tape::new();
        assert!(tape.is_empty());
        assert_eq!(tape.len(), 0);
        assert!(tape.get("anything").is_none());
    }

    // ── Ledger tests ──

    #[test]
    fn test_ledger_append_and_verify() {
        let mut ledger = Ledger::new();
        ledger
            .append(EventType::RunStart, None, None, None)
            .unwrap();
        ledger
            .append(
                EventType::Append,
                Some("n1".into()),
                Some("A0".into()),
                None,
            )
            .unwrap();
        ledger.append(EventType::RunEnd, None, None, None).unwrap();
        assert_eq!(ledger.len(), 3);
        assert!(ledger.verify().is_ok());
    }

    #[test]
    fn test_ledger_hash_chain_integrity() {
        let mut ledger = Ledger::new();
        ledger
            .append(EventType::RunStart, None, None, None)
            .unwrap();
        ledger
            .append(
                EventType::Append,
                Some("n1".into()),
                Some("A0".into()),
                None,
            )
            .unwrap();

        // First event has no prev_hash
        assert!(ledger.events()[0].prev_hash.is_none());
        // Second event links to first
        assert_eq!(
            ledger.events()[1].prev_hash,
            Some(ledger.events()[0].hash.clone())
        );
    }

    #[test]
    fn test_ledger_sequence_monotonic() {
        let mut ledger = Ledger::new();
        for _ in 0..5 {
            ledger.append(EventType::Append, None, None, None).unwrap();
        }
        for (i, event) in ledger.events().iter().enumerate() {
            assert_eq!(event.seq, i as u64);
        }
    }

    #[test]
    fn test_ledger_tamper_detection() {
        let mut ledger = Ledger::new();
        ledger
            .append(EventType::RunStart, None, None, None)
            .unwrap();
        ledger.append(EventType::Append, None, None, None).unwrap();

        // Tamper with an event
        ledger.events.as_mut_slice()[0].hash = "tampered".to_string();

        assert!(ledger.verify().is_err());
    }

    // ── TDMA-Bounded-RC1 Atom 1 tests ───────────────────────────

    fn mk_scope(run: &str, task: &str, parent: &str) -> AttemptScope {
        AttemptScope {
            run_id: run.into(),
            task_id: task.into(),
            verified_parent: parent.into(),
        }
    }

    fn mk_bbs_payload(scope: &AttemptScope, gain: f64, streak: u32) -> serde_json::Value {
        let bbs = RetryBeliefState {
            schema_version: "tdma-bbs/v1".into(),
            scope: scope.clone(),
            failure_signature: FailureSignature {
                reject_class: "schema-fail".into(),
                failed_predicate: "header.schema".into(),
                root_cause: "missing-field".into(),
            },
            constraints: vec![RetryConstraint {
                id: "c1".into(),
                rule: "must include schema_version".into(),
                priority: 200,
                source_attempt: 1,
                evidence_hash: "ev-hash-1".into(),
            }],
            evidence: EvidencePointer {
                evidence_node_hash: "ev-node-1".into(),
                raw_stderr_sha256: "0".repeat(64),
                trace_view_sha256: "1".repeat(64),
            },
            zero_gain_streak: streak,
            information_gain: gain,
            evicted: vec![],
        };
        serde_json::to_value(bbs).unwrap()
    }

    /// Gate-precursor: ledger_scope_persistence — Atom 1 acceptance.
    /// Verifies scope is first-class metadata and countable.
    #[test]
    fn ledger_scope_persistence() {
        let mut tape = MemoryTapeLedger::new();
        let verified_head = "H0".to_string();
        tape.set_verified_head(verified_head.clone());

        let scope = mk_scope("run-A", "task-1", &verified_head);

        // Commit 5 AgentProposal verified=false nodes under same scope
        for i in 1..=5u32 {
            tape.commit(CommitRequest {
                kind: NodeKind::AgentProposal,
                verified: false,
                parent: Some(verified_head.clone()),
                scope: Some(scope.clone()),
                attempt_ordinal: Some(i),
                reject_class: Some("header-malformed".into()),
                token_count: None,
                payload: serde_json::json!({"attempt": i}),
            });
        }

        // count_nodes filtered by (kind, verified, parent, scope) returns 5
        let n = tape.count_nodes(
            Some(NodeKind::AgentProposal),
            Some(false),
            Some(&verified_head),
            Some(&scope),
        );
        assert_eq!(n, 5, "5 AgentProposal nodes expected under same scope");

        // Each retrieved node has scope==Some(scope) and attempt_ordinal set
        let scope_nodes = &tape.indexes.nodes_by_scope[&scope];
        assert_eq!(scope_nodes.len(), 5);
        for h in scope_nodes {
            let node = &tape.indexes.by_hash[h];
            assert_eq!(node.scope.as_ref(), Some(&scope));
            assert!(node.attempt_ordinal.is_some());
        }
    }

    /// Gate-precursor: bbs_tape_canonical — Atom 1 acceptance + Gate 5 prep.
    /// Verifies BBS is reconstructable from tape via pure function,
    /// after dropping and rebuilding the ledger from frozen state.
    #[test]
    fn bbs_tape_canonical() {
        let mut tape = MemoryTapeLedger::new();
        tape.set_verified_head("H0".into());
        let scope = mk_scope("run-B", "task-1", "H0");

        // Commit 3 RetryBeliefState nodes, last one with gain=0.5/streak=2
        for (i, (gain, streak)) in
            [(0.9_f64, 0_u32), (0.3, 1), (0.5, 2)].into_iter().enumerate()
        {
            tape.commit(CommitRequest {
                kind: NodeKind::RetryBeliefState,
                verified: false,
                parent: Some("H0".into()),
                scope: Some(scope.clone()),
                attempt_ordinal: Some((i + 1) as u32),
                reject_class: None,
                token_count: None,
                payload: mk_bbs_payload(&scope, gain, streak),
            });
        }

        // Snapshot tape state, drop the ledger handle, rebuild from snapshot
        let frozen_indexes = tape.indexes.clone();
        drop(tape);
        let rebuilt = MemoryTapeLedger {
            indexes: frozen_indexes,
            next_seq: 0,
            clock: 0,
        };

        let derived = rebuilt
            .derive_latest_belief_state_from_tape(&scope)
            .expect("BBS must be derivable from frozen tape");
        assert_eq!(derived.information_gain, 0.5);
        assert_eq!(derived.zero_gain_streak, 2);
        assert_eq!(derived.scope, scope);
    }

    /// Gate-precursor: head_isolation — Atom 1 acceptance + Gate 9 prep.
    /// Verifies verified_head stays static under hard failures (verified=false commits)
    /// while ledger_tail advances.
    #[test]
    fn head_isolation() {
        let mut tape = MemoryTapeLedger::new();
        let h0 = "H0".to_string();
        tape.set_verified_head(h0.clone());

        let scope = mk_scope("run-C", "task-1", &h0);
        for i in 1..=10u32 {
            tape.commit(CommitRequest {
                kind: NodeKind::AgentProposal,
                verified: false,
                parent: Some(h0.clone()),
                scope: Some(scope.clone()),
                attempt_ordinal: Some(i),
                reject_class: Some("hard-fail".into()),
                token_count: None,
                payload: serde_json::json!({"attempt": i}),
            });
        }

        // verified_head MUST remain at H0
        assert_eq!(tape.get_verified_head(), h0);

        // ledger_tail MUST have moved (latest commit hash)
        assert_ne!(tape.indexes.ledger_tail, h0);
        assert!(!tape.indexes.ledger_tail.is_empty());

        // No StateAccepted under H0
        let accepted = tape.count_nodes(
            Some(NodeKind::StateAccepted),
            None,
            Some(&h0),
            None,
        );
        assert_eq!(accepted, 0);

        // 10 AgentProposal verified=false under scope
        let proposals = tape.count_nodes(
            Some(NodeKind::AgentProposal),
            Some(false),
            Some(&h0),
            Some(&scope),
        );
        assert_eq!(proposals, 10);
    }

    /// Verify the trait abstraction holds — dyn dispatch path works.
    /// This exercises the seam Karpathy-audit K10 was concerned about.
    #[test]
    fn immutable_tape_ledger_trait_dyn_dispatch() {
        let mut tape: Box<dyn ImmutableTapeLedger> = Box::new(MemoryTapeLedger::new());
        tape.set_verified_head("H0".into());
        let scope = mk_scope("run-D", "task-1", "H0");
        tape.commit(CommitRequest {
            kind: NodeKind::StateAccepted,
            verified: true,
            parent: Some("H0".into()),
            scope: None,
            attempt_ordinal: None,
            reject_class: None,
            token_count: None,
            payload: serde_json::json!({"accepted": true}),
        });
        // The bbs derivation correctly returns None when no RetryBeliefState exists.
        assert!(tape.derive_latest_belief_state_from_tape(&scope).is_none());
    }
}
