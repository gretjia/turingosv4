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
    let manifest = env!("CARGO_MANIFEST_DIR");
    let q_state_path = format!("{}/src/state/q_state.rs", manifest);
    let body = std::fs::read_to_string(&q_state_path)
        .unwrap_or_else(|e| panic!("read {}: {}", q_state_path, e));

    // Locate `pub struct AgentVisibleProjection {` and its terminating `}`.
    let needle = "pub struct AgentVisibleProjection";
    let start = body
        .find(needle)
        .expect("AgentVisibleProjection struct must exist in q_state.rs");
    let after = &body[start..];
    let brace_open = after
        .find('{')
        .expect("AgentVisibleProjection struct: opening brace not found");
    let mut depth = 0i32;
    let mut end = brace_open;
    for (i, ch) in after[brace_open..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    end = brace_open + i;
                    break;
                }
            }
            _ => {}
        }
    }
    let projection_body = &after[brace_open..=end];

    // Constructed at runtime via byte literals so this test's own source
    // doesn't contain the forbidden substrings.
    let forbidden: Vec<String> = vec![
        format!("agent_autopsies{}", "_t"),
        format!("Autopsy{}", "Index"),
        format!("Agent{}", "AutopsyCapsule"),
        format!("private_detail_{}", "cid"),
    ];
    for tok in &forbidden {
        assert!(
            !projection_body.contains(tok.as_str()),
            "halt-trigger #1: AgentVisibleProjection MUST NOT reference TB-15 \
             autopsy type `{}` — autopsy is sequencer-side / CAS-only and is NOT \
             projected to agent read view (CR-15.1)",
            tok
        );
    }
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
    let manifest = env!("CARGO_MANIFEST_DIR");
    let path = format!("{}/src/runtime/autopsy_capsule.rs", manifest);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {}", path, e));

    // The autopsy module MUST NOT contain any mutator surface against
    // the predicate / tool / risk-policy registries. Constructed at
    // runtime to avoid this test's own source containing the forbidden
    // substrings (and triggering self-trip on the file scan).
    let forbidden: Vec<String> = vec![
        format!("&mut Predicate{}", "Registry"),
        format!("&mut Tool{}", "Registry"),
        format!("&mut Risk{}", "PolicyRegistry"),
        format!("&mut PredicateRunner"),
        format!(".register_predicate("),
        format!(".unregister_predicate("),
        format!(".patch_predicate("),
        format!(".register_tool("),
        format!(".unregister_tool("),
    ];
    for tok in &forbidden {
        assert!(
            !body.contains(tok.as_str()),
            "halt-trigger #3: autopsy_capsule.rs MUST NOT contain `{}` — \
             autopsy carries `suggested_policy_patch: Option<Cid>` only as a \
             SUGGESTION pointer; never auto-applied (CR-15.3 + SG-15.8)",
            tok
        );
    }
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
    // Structural fence: AutopsyIndex value type must remain Vec<Cid>
    // (32-byte content addresses), NOT Vec<AgentAutopsyCapsule> (the
    // bytes themselves) and NOT any structure containing
    // private_detail_cid payload bytes. Even if AgentVisibleProjection
    // were ever to surface AutopsyIndex contents (which it does not —
    // see halt-trigger #1), it would surface only public CAS Cids of
    // public CAS evidence.
    let manifest = env!("CARGO_MANIFEST_DIR");
    let q_state_path = format!("{}/src/state/q_state.rs", manifest);
    let body = std::fs::read_to_string(&q_state_path)
        .unwrap_or_else(|e| panic!("read {}: {}", q_state_path, e));

    // Locate the AutopsyIndex newtype definition.
    let needle = "pub struct Autopsy".to_string() + "Index";
    let start = body
        .find(&needle)
        .expect("AutopsyIndex newtype must exist in q_state.rs");
    let after = &body[start..];
    // Walk forward until the line ending with `;` (newtype is single-line).
    let line_end = after
        .find(";\n")
        .or_else(|| after.find(";\r"))
        .or_else(|| after.find(';'))
        .expect("AutopsyIndex newtype must terminate with semicolon");
    let decl = &after[..=line_end];

    // The value type MUST be Vec<Cid>. Forbidden alternatives that
    // would leak raw bytes:
    let forbidden_value_shapes: Vec<String> = vec![
        format!("Vec<Agent{}>", "AutopsyCapsule"),
        format!("Vec<u{}>", "8"),
        format!("Vec<Auto{}>", "psyPrivateDetail"),
    ];
    for tok in &forbidden_value_shapes {
        assert!(
            !decl.contains(tok.as_str()),
            "halt-trigger #4: AutopsyIndex value type MUST be Vec<Cid>, \
             NOT `{}` — agent_autopsies_t stores Cids only; raw bytes \
             stay in CAS behind AuditOnly access (SG-15.2)",
            tok
        );
    }
    // Positive assertion: the declaration includes Vec<...Cid>.
    assert!(
        decl.contains("Vec<crate::bottom_white::cas::schema::Cid>")
            || decl.contains("Vec<Cid>"),
        "halt-trigger #4: AutopsyIndex value type must explicitly be Vec<Cid>; \
         got declaration: {}",
        decl
    );
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
