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
/// V3L-09: explicit vocabulary — only OmegaAccepted is a true OMEGA event.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    RunStart,
    Append,
    Invest,
    MarketCreate,
    MarketResolve,
    OmegaInvoke,
    OmegaAccepted,
    OmegaRejected,
    OmegaError,
    RunEnd,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventType::RunStart => write!(f, "RunStart"),
            EventType::Append => write!(f, "Append"),
            EventType::Invest => write!(f, "Invest"),
            EventType::MarketCreate => write!(f, "MarketCreate"),
            EventType::MarketResolve => write!(f, "MarketResolve"),
            EventType::OmegaInvoke => write!(f, "OmegaInvoke"),
            EventType::OmegaAccepted => write!(f, "OmegaAccepted"),
            EventType::OmegaRejected => write!(f, "OmegaRejected"),
            EventType::OmegaError => write!(f, "OmegaError"),
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
    fn compute_hash(seq: u64, event_type: &EventType, node_id: &Option<String>,
                    agent: &Option<String>, detail: &Option<String>,
                    prev_hash: &Option<String>) -> String {
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
    pub fn append(&mut self, event_type: EventType, node_id: Option<String>,
                  agent: Option<String>, detail: Option<String>) -> Result<&LedgerEvent, TapeError> {
        let prev_hash = self.events.last().map(|e| e.hash.clone());
        let hash = LedgerEvent::compute_hash(self.seq, &event_type, &node_id, &agent, &detail, &prev_hash);

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
                return Err(TapeError::LedgerCorruption(
                    format!("Truncation detected: expected last seq {}, got {}", expected_last_seq, actual_last_seq),
                ));
            }
        }
        for (i, event) in self.events.iter().enumerate() {
            // Check sequence monotonicity
            if event.seq != i as u64 {
                return Err(TapeError::LedgerCorruption(
                    format!("seq mismatch at index {}: expected {}, got {}", i, i, event.seq),
                ));
            }

            // Check prev_hash linkage
            let expected_prev = if i == 0 {
                None
            } else {
                Some(self.events[i - 1].hash.clone())
            };
            if event.prev_hash != expected_prev {
                return Err(TapeError::LedgerCorruption(
                    format!("prev_hash mismatch at seq {}", event.seq),
                ));
            }

            // Recompute and verify hash
            let recomputed = LedgerEvent::compute_hash(
                event.seq, &event.event_type, &event.node_id,
                &event.agent, &event.detail, &event.prev_hash,
            );
            if event.hash != recomputed {
                return Err(TapeError::LedgerCorruption(
                    format!("hash mismatch at seq {}", event.seq),
                ));
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
    DanglingCitation { node_id: String, missing_parent: String },
    LedgerCorruption(String),
}

impl fmt::Display for TapeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TapeError::DuplicateId(id) => write!(f, "Duplicate node ID: {}", id),
            TapeError::DanglingCitation { node_id, missing_parent } =>
                write!(f, "Node {} cites non-existent parent {}", node_id, missing_parent),
            TapeError::LedgerCorruption(msg) => write!(f, "Ledger corruption: {}", msg),
        }
    }
}

impl std::error::Error for TapeError {}

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
        tape.append(make_node("n1", "A0", "step 1", vec![])).unwrap();
        tape.append(make_node("n2", "A1", "step 2", vec!["n1"])).unwrap();
        assert_eq!(tape.len(), 2);
        assert_eq!(tape.children("n1"), &["n2"]);
    }

    #[test]
    fn test_tape_reject_duplicate_id() {
        // V6 spacetime paradox protection
        let mut tape = Tape::new();
        tape.append(make_node("n1", "A0", "step 1", vec![])).unwrap();
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
        tape.append(make_node("b", "A1", "second", vec!["a"])).unwrap();
        tape.append(make_node("c", "A0", "third", vec!["b"])).unwrap();
        assert_eq!(tape.time_arrow(), &["a", "b", "c"]);
    }

    #[test]
    fn test_tape_trace_ancestors() {
        let mut tape = Tape::new();
        tape.append(make_node("root", "A0", "root", vec![])).unwrap();
        tape.append(make_node("mid", "A1", "mid", vec!["root"])).unwrap();
        tape.append(make_node("leaf", "A0", "leaf", vec!["mid"])).unwrap();
        let path = tape.trace_ancestors("leaf");
        assert_eq!(path, vec!["root", "mid", "leaf"]);
    }

    #[test]
    fn test_tape_dag_branching() {
        let mut tape = Tape::new();
        tape.append(make_node("root", "A0", "root", vec![])).unwrap();
        tape.append(make_node("b1", "A1", "branch 1", vec!["root"])).unwrap();
        tape.append(make_node("b2", "A2", "branch 2", vec!["root"])).unwrap();
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
        ledger.append(EventType::RunStart, None, None, None).unwrap();
        ledger.append(EventType::Append, Some("n1".into()), Some("A0".into()), None).unwrap();
        ledger.append(EventType::RunEnd, None, None, None).unwrap();
        assert_eq!(ledger.len(), 3);
        assert!(ledger.verify().is_ok());
    }

    #[test]
    fn test_ledger_hash_chain_integrity() {
        let mut ledger = Ledger::new();
        ledger.append(EventType::RunStart, None, None, None).unwrap();
        ledger.append(EventType::Append, Some("n1".into()), Some("A0".into()), None).unwrap();

        // First event has no prev_hash
        assert!(ledger.events()[0].prev_hash.is_none());
        // Second event links to first
        assert_eq!(ledger.events()[1].prev_hash, Some(ledger.events()[0].hash.clone()));
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
        ledger.append(EventType::RunStart, None, None, None).unwrap();
        ledger.append(EventType::Append, None, None, None).unwrap();

        // Tamper with an event
        ledger.events.as_mut_slice()[0].hash = "tampered".to_string();

        assert!(ledger.verify().is_err());
    }

    #[test]
    fn test_ledger_omega_vocabulary() {
        // V3L-09: only OmegaAccepted is the canonical OMEGA event
        let mut ledger = Ledger::new();
        ledger.append(EventType::OmegaInvoke, Some("n1".into()), None, None).unwrap();
        ledger.append(EventType::OmegaAccepted, Some("n1".into()), None, None).unwrap();
        assert!(ledger.verify().is_ok());
        assert_eq!(ledger.events()[1].event_type, EventType::OmegaAccepted);
    }
}
