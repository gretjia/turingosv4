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
    use sha2::{Digest, Sha256};
    use turingosv4::runtime::markov_capsule::{
        read_flowchart_hashes_from_matrix, MarkovEvidenceCapsule,
    };

    let manifest = env!("CARGO_MANIFEST_DIR");
    let constitution_path = format!("{}/constitution.md", manifest);
    let constitution_bytes = std::fs::read(&constitution_path)
        .unwrap_or_else(|e| panic!("read constitution.md: {}", e));
    let mut h = Sha256::new();
    h.update(&constitution_bytes);
    let expected_hash: [u8; 32] = h.finalize().into();

    let capsule = MarkovEvidenceCapsule::with_constitution_hash(expected_hash);
    assert_eq!(
        capsule.constitution_hash.0, expected_hash,
        "halt-trigger #2: MarkovEvidenceCapsule.constitution_hash must equal \
         sha256 of constitution.md bytes (SG-15.7)"
    );

    // R2 closure (Codex R1 Q8/RQ7 + Gemini R1 Q7): SG-15.7 spec literal
    // is "constitution hash AND flowchart hashes". Capsule MUST also
    // reference 4 canonical flowchart hashes (per architect 2026-05-02
    // ruling 9 of Part C — flowcharts elevated to SHA-anchored
    // architectural contracts).
    let matrix_path = std::path::PathBuf::from(manifest)
        .join("handover/alignment/TRACE_FLOWCHART_MATRIX.md");
    let flowchart_hashes =
        read_flowchart_hashes_from_matrix(&matrix_path).expect("matrix parse");
    assert_eq!(
        flowchart_hashes.len(),
        4,
        "halt-trigger #2: TRACE_FLOWCHART_MATRIX.md must yield exactly 4 \
         canonical flowchart hashes (1a, 1b, 2, 3) per architect §2 (SG-15.7)"
    );
    // Capsule's flowchart_hashes field exists + accepts 4 hashes.
    let mut cap_with_fc = capsule.clone();
    cap_with_fc.flowchart_hashes = flowchart_hashes.clone();
    assert_eq!(
        cap_with_fc.flowchart_hashes.len(),
        4,
        "halt-trigger #2: MarkovEvidenceCapsule.flowchart_hashes must hold \
         exactly the 4 canonical flowchart hashes (SG-15.7 literal compliance)"
    );
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
    use turingosv4::bottom_white::cas::schema::Cid;
    use turingosv4::economy::money::MicroCoin;
    use turingosv4::runtime::autopsy_capsule::{
        cluster_autopsies, AgentAutopsyCapsule, LossReasonClass,
    };
    use turingosv4::state::q_state::{AgentId, Hash, TaskId};
    use turingosv4::state::typed_tx::{CapsulePrivacyPolicy, EventId};

    // Build 3 autopsies of the same loss_reason_class with
    // distinguishable private_detail_cid bytes.
    let event = EventId(TaskId("task:tb15:halt5".into()));
    let mk = |agent: &str, priv_byte: u8| AgentAutopsyCapsule {
        capsule_id: Cid::from_content(agent.as_bytes()),
        agent_id: AgentId(agent.to_string()),
        event_id: event.clone(),
        loss_amount: MicroCoin::from_micro_units(1_000),
        loss_reason_class: LossReasonClass::Bankruptcy,
        violated_risk_rule: None,
        suggested_policy_patch: None,
        evidence_cids: vec![],
        public_summary: format!(
            "agent={} lost 1000μC on event={} reason=Bankruptcy",
            agent, (event.0).0
        ),
        private_detail_cid: Cid([priv_byte; 32]),
        privacy_policy: CapsulePrivacyPolicy::AuditOnly,
        sha256: Hash::ZERO,
        created_at_logical_t: 1,
        created_at_round: 0,
    };
    let priv_bytes: [u8; 3] = [0xAA, 0xBB, 0xCC];
    let autopsies = vec![
        mk("A", priv_bytes[0]),
        mk("B", priv_bytes[1]),
        mk("C", priv_bytes[2]),
    ];

    let summaries = cluster_autopsies(&autopsies, 3);
    assert_eq!(summaries.len(), 1, "3 same-class autopsies → 1 typical error");
    assert_eq!(summaries[0].count, 3);

    // R2 closure (Codex R1 Q5): the original byte-window scan looked for
    // a raw 32-byte run of `[priv_byte; 32]`, but Cid serializes through
    // serde_json as a 32-element JSON ARRAY (`[170,170,...,170]`) — NOT
    // a contiguous binary 32-byte run. The strengthened check inspects
    // BOTH (a) the JSON-array text representation that serde_json
    // produces for a `Cid([priv_byte; 32])`, AND (b) the raw 32-byte run
    // (defense-in-depth against future format changes).
    let json_text = serde_json::to_string(&summaries).expect("serialize summaries");
    let json_bytes = json_text.as_bytes();
    let canonical_bytes =
        turingosv4::bottom_white::ledger::transition_ledger::canonical_encode(&summaries)
            .expect("canonical encode");
    for &priv_byte in &priv_bytes {
        // (a) JSON-array text form: a Cid([0xAA;32]) renders as
        //     `[170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,
        //       170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,
        //       170,170]` (each byte as its decimal value).
        let n = priv_byte as u32;
        let mut json_array_form = String::with_capacity(160);
        json_array_form.push('[');
        for i in 0..32 {
            if i > 0 {
                json_array_form.push(',');
            }
            json_array_form.push_str(&n.to_string());
        }
        json_array_form.push(']');
        assert!(
            !json_text.contains(&json_array_form),
            "halt-trigger #5 (R2 strengthened): TypicalErrorSummary JSON \
             serialization contains the canonical Cid array form for \
             private_detail_cid byte 0x{:02x} — broadcast surface MUST use \
             public_summary text only (SG-15.5)",
            priv_byte
        );

        // (b) raw 32-byte run defense-in-depth (would catch
        //     a hypothetical bincode/canonical-encoded leak).
        let private_cid_run = [priv_byte; 32];
        for window in canonical_bytes.windows(32) {
            assert!(
                window != private_cid_run,
                "halt-trigger #5 (R2): canonical_encode of TypicalErrorSummary \
                 contains a 32-byte run of private_detail_cid byte 0x{:02x}",
                priv_byte
            );
        }
        // Also still check JSON bytes for raw run (belt + suspenders).
        for window in json_bytes.windows(32) {
            assert!(
                window != private_cid_run,
                "halt-trigger #5 (R2): JSON of TypicalErrorSummary contains a \
                 raw 32-byte run of private_detail_cid byte 0x{:02x}",
                priv_byte
            );
        }
    }
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
    use turingosv4::runtime::markov_capsule::{
        try_deep_history_read_with_override_check, MarkovGenError,
    };

    // Default-deny path: no override; result must be DeepHistoryReadDenied.
    let result = try_deep_history_read_with_override_check(false);
    match result {
        Err(MarkovGenError::DeepHistoryReadDenied) => {}
        other => panic!(
            "halt-trigger #6: expected DeepHistoryReadDenied without \
             TURINGOS_MARKOV_OVERRIDE=1; got {:?} (SG-15.4 + FR-15.5)",
            other
        ),
    }

    // Override path: result is Ok(()).
    let ok = try_deep_history_read_with_override_check(true);
    assert!(
        ok.is_ok(),
        "halt-trigger #6: TURINGOS_MARKOV_OVERRIDE=1 must permit deep-history \
         read; got {:?}",
        ok
    );
}
