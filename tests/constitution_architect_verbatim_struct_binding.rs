//! Constitution gate — architect verbatim struct-field binding.
//!
//! Authority: `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md`
//! §1.B.1 (Phase E.1) + plan `cached-noodle.md` §C.E.1.
//!
//! Codex G2 audit (2026-05-09) caught two verbatim spec drifts:
//!   - P-M2 `CompleteSetMergeTx` added a `timestamp_logical` field not in
//!     architect manual §7.3 verbatim 6-field spec.
//!   - P-M4 `CpmmPool` used `event_id_kind` where architect §7.5 verbatim
//!     specifies `event_id`.
//!
//! Self-audit (`cargo test --workspace` GREEN, gate names verbatim) did
//! not catch either drift because tests check behavior + test-name spelling
//! but not struct-field spelling.
//!
//! This gate hardcodes the architect-spec'd struct field set per atom and
//! checks the codebase implementation against it. For NotYetLanded atoms
//! (Stage C VETO rolled back P-M2/P-M4/P-M5/P-M6), the binding is recorded
//! but the check is a no-op until Phase F rebuild lands the struct, at
//! which point the binding's `landing_status` flips to `Landed` in the
//! same commit and the strict check fires.
//!
//! Rationale for hardcoded fixtures (vs runtime parsing of the manual):
//! the manual's Markdown ` ```rust ``` ` blocks are not stable input for
//! regex-based parsing. Per plan §H fallback option, the architect-spec
//! field set is mirrored as an in-test fixture; any future architect-manual
//! amendment requires a corresponding fixture update (an explicit-sync
//! point, consistent with `feedback_no_workarounds_strict_constitution`).

use std::collections::BTreeSet;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LandingStatus {
    Landed,
    NotYetLanded,
}

#[derive(Debug)]
struct StructBinding {
    /// Stage C atom id (for diagnostic) — e.g. "P-M2", "P-M1".
    atom_id: &'static str,
    /// Architect manual section reference, e.g. "§7.3".
    manual_section: &'static str,
    /// Rust struct name as it appears (or will appear) in the codebase.
    struct_name: &'static str,
    /// Path to the impl file relative to workspace root.
    impl_path: &'static str,
    /// Verbatim field-name set per architect spec.
    expected_fields: &'static [&'static str],
    /// Whether the struct currently exists in the codebase. NotYetLanded
    /// atoms record the spec without enforcing it; Landed atoms enforce
    /// strict field-set equality.
    landing_status: LandingStatus,
}

// Bindings are intentionally narrow at Phase E ship: only architect manual
// sections that provide an EXPLICIT verbatim `pub struct ...` block AND are
// not entangled with pre-existing TB-13-era drift are bound here. The
// self-check tests below prove the parser + diff logic work; Phase F atoms
// flip their bindings to `Landed` when they rebuild the VETO'd structs.
//
// Sections deliberately NOT bound at Phase E:
//   • §7.2 CompleteSetMintTx / CompleteSetRedeemTx — manual §7.2 specifies
//     semantics only (no `pub struct {...}` block). Adding a fixture here
//     would be extrapolation, not verbatim binding.
//   • §7.4 MarketSeedTx — manual gives a 6-field struct, but the TB-13 era
//     impl carries a `timestamp_logical` field that predates Stage C. The
//     drift is real and worth addressing, but not via Phase E (which would
//     scope-creep into either an architect-manual amendment or a TB-13 era
//     refactor). Phase F.3 (P-M3 re-apply) is the natural decision point.
const BINDINGS: &[StructBinding] = &[
    // ── NotYetLanded (Stage C VETO rolled these back; Phase F rebuilds) ──
    StructBinding {
        atom_id: "P-M2",
        manual_section: "§7.3",
        struct_name: "CompleteSetMergeTx",
        impl_path: "src/state/typed_tx.rs",
        // Architect §7.3 verbatim 6-field spec. NO timestamp_logical (Codex defect 3).
        expected_fields: &[
            "tx_id",
            "parent_state_root",
            "event_id",
            "owner",
            "amount",
            "signature",
        ],
        landing_status: LandingStatus::NotYetLanded,
    },
    StructBinding {
        atom_id: "P-M4",
        manual_section: "§7.5",
        struct_name: "CpmmPool",
        impl_path: "src/state/q_state.rs",
        // Architect §7.5 verbatim 5-field spec. event_id NOT event_id_kind (Codex defect 4).
        expected_fields: &[
            "event_id",
            "pool_yes",
            "pool_no",
            "lp_total_shares",
            "status",
        ],
        landing_status: LandingStatus::NotYetLanded,
    },
];

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_file(rel: &str) -> Option<String> {
    let path = workspace_root().join(rel);
    std::fs::read_to_string(&path).ok()
}

/// Locate the `pub struct <name> { ... }` declaration in source and extract
/// the set of public field identifiers. Returns None if struct not present.
///
/// Parser is intentionally simple: finds `pub struct <Name>` line, then
/// reads forward until the matching `}` on column 0 or `}` after any field
/// declarations. Counts brace depth to handle nested types in field types
/// (e.g. `Option<Vec<T>>`).
fn extract_struct_fields(source: &str, struct_name: &str) -> Option<BTreeSet<String>> {
    let needle = format!("pub struct {}", struct_name);
    let mut lines = source.lines();
    let mut found = false;
    let mut depth: i32 = 0;
    let mut fields = BTreeSet::new();
    while let Some(line) = lines.next() {
        if !found {
            // Match `pub struct <Name>` followed by ` ` or `<` or `{` or end-of-line.
            if let Some(idx) = line.find(&needle) {
                // Ensure the next char after the name terminates the identifier.
                let after = &line[idx + needle.len()..];
                let next_char = after.chars().next();
                let is_terminator = matches!(next_char, None | Some(' ') | Some('<') | Some('{') | Some('('));
                if is_terminator {
                    found = true;
                    // Track braces opened on this line.
                    depth += line.matches('{').count() as i32;
                    depth -= line.matches('}').count() as i32;
                    // If struct is unit/tuple form `pub struct X;` or `pub struct X(...);`,
                    // there are no named fields → return empty set.
                    if line.ends_with(';') {
                        return Some(BTreeSet::new());
                    }
                }
            }
            continue;
        }
        // We're inside the struct body.
        depth += line.matches('{').count() as i32;
        depth -= line.matches('}').count() as i32;
        // Field declaration heuristic: a line like `    pub field_name: Type,`
        // We strip leading whitespace, look for `pub ` prefix, then take the
        // ident before the first `:`.
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("pub ") {
            if let Some(colon_idx) = rest.find(':') {
                let name = rest[..colon_idx].trim();
                // Reject anything that looks like a sub-type (`(`, `<`, `fn`).
                if name
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_')
                    && !name.is_empty()
                {
                    fields.insert(name.to_string());
                }
            }
        }
        if depth <= 0 {
            return Some(fields);
        }
    }
    if found {
        // Reached EOF without closing brace — partial extract.
        Some(fields)
    } else {
        None
    }
}

#[test]
fn architect_verbatim_struct_field_bindings() {
    let mut failures: Vec<String> = Vec::new();
    for b in BINDINGS {
        let source = match read_file(b.impl_path) {
            Some(s) => s,
            None => {
                failures.push(format!(
                    "[{}/{}] impl file not readable: {}",
                    b.atom_id, b.struct_name, b.impl_path
                ));
                continue;
            }
        };
        let actual = extract_struct_fields(&source, b.struct_name);
        match (b.landing_status, actual) {
            (LandingStatus::NotYetLanded, None) => {
                // Expected: rolled-back struct not present in codebase.
            }
            (LandingStatus::NotYetLanded, Some(fields)) => {
                failures.push(format!(
                    "[{}/{}] declared NotYetLanded but `pub struct {}` is present in {} \
                     with fields {:?}; flip landing_status to Landed in this binding when \
                     Phase F rebuilds the atom",
                    b.atom_id, b.struct_name, b.struct_name, b.impl_path, fields
                ));
            }
            (LandingStatus::Landed, None) => {
                failures.push(format!(
                    "[{}/{}] declared Landed but `pub struct {}` not found in {}",
                    b.atom_id, b.struct_name, b.struct_name, b.impl_path
                ));
            }
            (LandingStatus::Landed, Some(actual_set)) => {
                let expected: BTreeSet<String> =
                    b.expected_fields.iter().map(|s| s.to_string()).collect();
                if expected != actual_set {
                    let extra: Vec<String> =
                        actual_set.difference(&expected).cloned().collect();
                    let missing: Vec<String> =
                        expected.difference(&actual_set).cloned().collect();
                    failures.push(format!(
                        "[{}/{}] verbatim drift vs architect manual {}: \
                         extra fields in impl {:?}; missing fields in impl {:?}; \
                         architect verbatim spec is exactly {:?}",
                        b.atom_id,
                        b.struct_name,
                        b.manual_section,
                        extra,
                        missing,
                        b.expected_fields,
                    ));
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "Phase E.1 architect verbatim struct binding failed for {} binding(s):\n{}",
        failures.len(),
        failures.join("\n"),
    );
}

#[test]
fn binding_self_check_extracts_known_fields() {
    // Sanity: the parser correctly extracts fields from a known-good Landed
    // struct in the actual codebase.
    let source = read_file("src/state/typed_tx.rs")
        .expect("typed_tx.rs must be readable for self-check");
    let fields = extract_struct_fields(&source, "CompleteSetMintTx")
        .expect("CompleteSetMintTx must be present in typed_tx.rs (Landed)");
    assert!(
        fields.contains("tx_id"),
        "self-check: extracted fields should include tx_id; got {:?}",
        fields,
    );
    assert!(
        fields.contains("event_id"),
        "self-check: extracted fields should include event_id; got {:?}",
        fields,
    );
    assert!(
        fields.contains("amount"),
        "self-check: extracted fields should include amount; got {:?}",
        fields,
    );
}

#[test]
fn binding_self_check_synthetic_drift_detected() {
    // Synthetic Rust source mimicking Codex defect 3 (P-M2 timestamp_logical drift).
    let synthetic = r#"
pub struct CompleteSetMergeTx_Synthetic {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub event_id: EventId,
    pub owner: AgentId,
    pub amount: ShareAmount,
    pub timestamp_logical: u64,  // <-- spec drift
    pub signature: AgentSignature,
}
"#;
    let actual =
        extract_struct_fields(synthetic, "CompleteSetMergeTx_Synthetic")
            .expect("synthetic struct must parse");
    let expected: BTreeSet<String> = [
        "tx_id",
        "parent_state_root",
        "event_id",
        "owner",
        "amount",
        "signature",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    assert_ne!(
        actual, expected,
        "self-check: parser should detect the extra `timestamp_logical` field in synthetic \
         CompleteSetMergeTx_Synthetic vs the architect §7.3 6-field expected set; \
         actual={:?} expected={:?}",
        actual, expected,
    );
    assert!(
        actual.contains("timestamp_logical"),
        "self-check: parser should extract timestamp_logical; got {:?}",
        actual,
    );
}
