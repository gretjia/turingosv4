/// TB-15 Halt-Trigger Fixture (architect §6.6 forbidden + §6.5 SG halts)
///
/// 6 tests that must ALL be green before TB-15 ships.
/// Atom 1 = `unimplemented!()` stubs only; later atoms backfill:
///   Atom 2: #3 (autopsy_does_not_mutate_predicates)
///   Atom 3: #1 (raw_logs_not_in_general_read_view) + #4 (private_detail_not_in_other_agent_view)
///   Atom 4: #5 (typical_error_clustering_uses_summary_only)
///   Atom 5: #2 (markov_capsule_references_constitution_hash) + #6 (deep_history_read_without_override_fails)
///
/// Any atom that flips a green test to red = immediate halt (no round-2).
/// TRACE_MATRIX FC1-N32 + FC1-N33 + FC2-N30 + FC3-N43

// ────────────────────────────────────────────────────────────────────
// Halt-trigger #1
// raw_logs_not_in_general_read_view
//
// AgentVisibleProjection.views must NOT contain raw autopsy bytes
// (private_detail_cid contents). Agent_autopsies_t lives on
// EconomicState — sequencer-side index only — and is NOT projected
// into AgentVisibleProjection. CR-15.1.
//
// Filled in by Atom 3 (after EconomicState gains agent_autopsies_t).
// ────────────────────────────────────────────────────────────────────
#[test]
fn raw_logs_not_in_general_read_view() {
    unimplemented!("TB-15 halt #1 — backfill in Atom 3");
}

// ────────────────────────────────────────────────────────────────────
// Halt-trigger #2
// markov_capsule_references_constitution_hash
//
// MarkovEvidenceCapsule.constitution_hash must equal sha256 of the
// constitution.md bytes at generation time. SG-15.7.
//
// Filled in by Atom 5 (markov_capsule generator).
// ────────────────────────────────────────────────────────────────────
#[test]
fn markov_capsule_references_constitution_hash() {
    unimplemented!("TB-15 halt #2 — backfill in Atom 5");
}

// ────────────────────────────────────────────────────────────────────
// Halt-trigger #3
// autopsy_does_not_mutate_predicates
//
// write_autopsy_capsule signature MUST NOT accept any &mut PredicateRegistry
// or any other mutator on the predicate / tool / risk-policy registries.
// Source-level fence: scan src/runtime/autopsy_capsule.rs for forbidden
// signature tokens. CR-15.3 + SG-15.8.
//
// Filled in by Atom 2.
// ────────────────────────────────────────────────────────────────────
#[test]
fn autopsy_does_not_mutate_predicates() {
    unimplemented!("TB-15 halt #3 — backfill in Atom 2");
}

// ────────────────────────────────────────────────────────────────────
// Halt-trigger #4
// private_detail_not_in_other_agent_view
//
// Agent B's projection must not contain Agent A's autopsy bytes.
// AutopsyIndex stores Cids only; the CAS bytes behind private_detail_cid
// require AuditOnly access. SG-15.2.
//
// Filled in by Atom 3 (after EconomicState gains agent_autopsies_t).
// ────────────────────────────────────────────────────────────────────
#[test]
fn private_detail_not_in_other_agent_view() {
    unimplemented!("TB-15 halt #4 — backfill in Atom 3");
}

// ────────────────────────────────────────────────────────────────────
// Halt-trigger #5
// typical_error_clustering_uses_summary_only
//
// cluster_autopsies output (Vec<TypicalErrorSummary>) must embed
// public_summary text + capsule_id Cids only. It must NEVER embed
// private_detail_cid bytes. SG-15.5.
//
// Filled in by Atom 4 (cluster_autopsies + TypicalErrorSummary).
// ────────────────────────────────────────────────────────────────────
#[test]
fn typical_error_clustering_uses_summary_only() {
    unimplemented!("TB-15 halt #5 — backfill in Atom 4");
}

// ────────────────────────────────────────────────────────────────────
// Halt-trigger #6
// deep_history_read_without_override_fails
//
// generate_markov_capsule binary defaults to constitution +
// latest-Markov-capsule context source. Reading deeper history (older
// capsules; L4 chain rows pre-dating prior Markov capsule's l4_root)
// requires TURINGOS_MARKOV_OVERRIDE=1; default-deny path returns
// `MarkovGenError::DeepHistoryReadDenied`. SG-15.4 + FR-15.5.
//
// Filled in by Atom 5.
// ────────────────────────────────────────────────────────────────────
#[test]
fn deep_history_read_without_override_fails() {
    unimplemented!("TB-15 halt #6 — backfill in Atom 5");
}
