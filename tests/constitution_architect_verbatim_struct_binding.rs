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
    /// Verbatim (field_name, type_first_token) pairs per architect spec.
    /// Hardened per Codex re-audit 2026-05-09 Recommendation 2: field NAME
    /// alone is insufficient — a future drift could rename a TYPE while
    /// keeping the field name (e.g. `event_id: EventIdKind` would have
    /// preserved the name `event_id` but corrupted the type). Tracking
    /// `(name, type_first_token)` pairs catches this.
    ///
    /// `type_first_token` is the first identifier of the type expression:
    ///   - `EventId` for `pub event_id: EventId`
    ///   - `Option` for `pub note: Option<String>` (the outer wrapper)
    ///   - `Vec` for `pub items: Vec<u32>`
    /// This is conservative — full type matching would require parsing
    /// generic args and lifetime params, which is out of scope. The
    /// first-token heuristic catches the realistic drift shapes
    /// (renames + wrapper changes).
    expected_fields: &'static [(&'static str, &'static str)],
    /// Whether the struct currently exists in the codebase. NotYetLanded
    /// atoms record the spec without enforcing it; Landed atoms enforce
    /// strict field-set equality (name + type-first-token pair).
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
    // ── Landed (Stage C P-M2 / Phase F.1 rebuild 2026-05-09 session #29) ──
    StructBinding {
        atom_id: "P-M2",
        manual_section: "§7.3",
        struct_name: "CompleteSetMergeTx",
        impl_path: "src/state/typed_tx.rs",
        // Architect §7.3 verbatim 6-field spec with types.
        // NO timestamp_logical (Codex defect 3 — caught at gate-time on
        // Landed flip if reintroduced).
        expected_fields: &[
            ("tx_id", "TxId"),
            ("parent_state_root", "Hash"),
            ("event_id", "EventId"),
            ("owner", "AgentId"),
            ("amount", "ShareAmount"),
            ("signature", "AgentSignature"),
        ],
        landing_status: LandingStatus::Landed,
    },
    // F-DEFERRAL-2 closure (per remediation directive §9 — extends BINDINGS
    // for each Class-4 atom rebuild with a sibling SigningPayload entry).
    // CompleteSetMergeSigningPayload is the agent-signing projection of
    // CompleteSetMergeTx (5 wire fields + domain prefix; signature
    // excluded per `to_signing_payload` cycle-on-self prevention).
    StructBinding {
        atom_id: "P-M2",
        manual_section: "§7.3-signing",
        struct_name: "CompleteSetMergeSigningPayload",
        impl_path: "src/state/typed_tx.rs",
        // Verbatim 5-field projection (the 6 wire fields minus `signature`).
        expected_fields: &[
            ("tx_id", "TxId"),
            ("parent_state_root", "Hash"),
            ("event_id", "EventId"),
            ("owner", "AgentId"),
            ("amount", "ShareAmount"),
        ],
        landing_status: LandingStatus::Landed,
    },
    StructBinding {
        atom_id: "P-M4",
        manual_section: "§7.5",
        struct_name: "CpmmPool",
        impl_path: "src/state/q_state.rs",
        // Architect §7.5 verbatim 5-field spec with types.
        // event_id (not event_id_kind) AND type EventId (not PoolEventKind);
        // E' hardening per Codex 2026-05-09 Recommendation 2 — both name
        // and type-first-token are checked, so a hypothetical drift like
        // `pub event_id: PoolEventKind` (preserving the name but renaming
        // type) is now caught.
        expected_fields: &[
            ("event_id", "EventId"),
            ("pool_yes", "ShareAmount"),
            ("pool_no", "ShareAmount"),
            ("lp_total_shares", "LpShareAmount"),
            ("status", "PoolStatus"),
        ],
        // FLIPPED 2026-05-09 session #31 by Phase F.3 P-M4 rebuild commit:
        // CpmmPool struct now Landed at `src/state/q_state.rs`. E.1 strict
        // (name, type) pair-equality enforces architect §7.5 verbatim shape;
        // any future drift (e.g. reintroducing `event_id_kind`) fails at
        // gate-time.
        landing_status: LandingStatus::Landed,
    },
    // F-DEFERRAL-2 closure for P-M4 (per remediation directive §9):
    // sibling SigningPayload binding extends drift detection from the
    // wire-tx struct to its agent-signed projection. Architect §7.5
    // specifies the STATE struct only; the tx + signing payload are
    // implementation-defined (strict-minimal 7-wire-field shape mirroring
    // CompleteSetMergeTx P-M2 pattern, NO `timestamp_logical`).
    // CpmmPoolSigningPayload is the 6-field projection (7 wire fields
    // minus `signature`); fields are pinned here so a future implementer
    // cannot pollute the signing payload with extra fields without failing
    // this gate.
    StructBinding {
        atom_id: "P-M4",
        manual_section: "§7.5-signing",
        struct_name: "CpmmPoolSigningPayload",
        impl_path: "src/state/typed_tx.rs",
        expected_fields: &[
            ("tx_id", "TxId"),
            ("parent_state_root", "Hash"),
            ("event_id", "EventId"),
            ("provider", "AgentId"),
            ("seed_yes", "ShareAmount"),
            ("seed_no", "ShareAmount"),
        ],
        landing_status: LandingStatus::Landed,
    },
    // ── Phase F.5 Stage C P-M6 (architect §7.7 9-step composite) ──────────
    // BuyWithCoinRouterTx is implementation-defined — architect §7.7 fixes
    // the 9-step semantics + integer formula + 9 mandated tests, not the
    // tx schema. The 8-field shape mirrors CpmmSwapTx (P-M5) minimal
    // pattern + replaces `amount_in: ShareAmount` with `pay_coin: MicroCoin`
    // (router input is Coin payment). Defect-3 prevention: NO
    // `timestamp_logical`. Defect-4 prevention: `event_id` NOT
    // `event_id_kind`.
    //
    // FLIPPED 2026-05-09 session #32 (Phase F.5 P-M6 rebuild commit):
    // BuyWithCoinRouterTx Landed at `src/state/typed_tx.rs`. E.1 strict
    // (name, type) pair-equality enforces the wire shape; future drift
    // (e.g. reintroducing `timestamp_logical`) fails at gate-time.
    StructBinding {
        atom_id: "P-M6",
        manual_section: "§7.7",
        struct_name: "BuyWithCoinRouterTx",
        impl_path: "src/state/typed_tx.rs",
        expected_fields: &[
            ("tx_id", "TxId"),
            ("parent_state_root", "Hash"),
            ("event_id", "EventId"),
            ("buyer", "AgentId"),
            ("direction", "BuyDirection"),
            ("pay_coin", "MicroCoin"),
            ("min_out_shares", "ShareAmount"),
            ("signature", "AgentSignature"),
        ],
        landing_status: LandingStatus::Landed,
    },
    // F-DEFERRAL-2 closure for P-M6 (per remediation directive §9):
    // sibling SigningPayload binding extends drift detection from the
    // wire-tx struct to its agent-signed projection. 7-field projection
    // (8 wire fields minus `signature`).
    StructBinding {
        atom_id: "P-M6",
        manual_section: "§7.7-signing",
        struct_name: "BuyWithCoinRouterSigningPayload",
        impl_path: "src/state/typed_tx.rs",
        expected_fields: &[
            ("tx_id", "TxId"),
            ("parent_state_root", "Hash"),
            ("event_id", "EventId"),
            ("buyer", "AgentId"),
            ("direction", "BuyDirection"),
            ("pay_coin", "MicroCoin"),
            ("min_out_shares", "ShareAmount"),
        ],
        landing_status: LandingStatus::Landed,
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
/// `(field_name, type_first_token)` pairs for every `pub` field.
///
/// Hardened per Codex re-audit 2026-05-09 Recommendation 2: returning
/// type-first-token alongside the name catches drift shapes where a
/// future implementer preserves the field name but changes the type
/// (e.g. `pub event_id: PoolEventKind` would have preserved name
/// `event_id` while corrupting the type — fixture mismatch on the
/// type token catches this).
///
/// Parser is intentionally simple: finds `pub struct <Name>` line, then
/// reads forward until the matching `}` (brace depth tracking handles
/// nested types like `Option<Vec<T>>`). For each `pub <name>: <type>,`
/// line, captures the first identifier of the type expression.
fn extract_struct_field_pairs(
    source: &str,
    struct_name: &str,
) -> Option<Vec<(String, String)>> {
    let needle = format!("pub struct {}", struct_name);
    let mut found = false;
    let mut depth: i32 = 0;
    let mut pairs: Vec<(String, String)> = Vec::new();
    for line in source.lines() {
        if !found {
            if let Some(idx) = line.find(&needle) {
                let after = &line[idx + needle.len()..];
                let next_char = after.chars().next();
                let is_terminator = matches!(
                    next_char,
                    None | Some(' ') | Some('<') | Some('{') | Some('(')
                );
                if is_terminator {
                    found = true;
                    depth += line.matches('{').count() as i32;
                    depth -= line.matches('}').count() as i32;
                    if line.ends_with(';') {
                        return Some(Vec::new());
                    }
                }
            }
            continue;
        }
        depth += line.matches('{').count() as i32;
        depth -= line.matches('}').count() as i32;
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("pub ") {
            if let Some(colon_idx) = rest.find(':') {
                let name = rest[..colon_idx].trim();
                if name
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_')
                    && !name.is_empty()
                {
                    let type_part = rest[colon_idx + 1..].trim();
                    // Phase F.3 P-M4 parser hardening: handle path-qualified
                    // types like `crate::state::typed_tx::EventId`. Take the
                    // type-expression head (up to first `<` / `,` / `(` /
                    // whitespace), allowing `::` so the path is preserved,
                    // then take the LAST `::`-separated segment. Examples:
                    //   `EventId`                              → "EventId"
                    //   `crate::state::typed_tx::EventId`      → "EventId"
                    //   `Option<String>`                       → "Option"
                    //   `Vec<u32>`                             → "Vec"
                    //   `BTreeMap<EventId, ShareSidePair>`     → "BTreeMap"
                    // This is forward-looking: prior atoms (P-M2) used direct
                    // imports so the broken parser was never exercised; P-M4
                    // CpmmPool uses path-qualified types to avoid a circular
                    // import (q_state.rs → typed_tx.rs → q_state.rs cycle if
                    // EventId/ShareAmount were imported via `use`).
                    let head: String = type_part
                        .chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == ':')
                        .collect();
                    let type_token: String = head
                        .rsplit("::")
                        .next()
                        .unwrap_or("")
                        .to_string();
                    if !type_token.is_empty() {
                        pairs.push((name.to_string(), type_token));
                    }
                }
            }
        }
        if depth <= 0 {
            return Some(pairs);
        }
    }
    if found {
        Some(pairs)
    } else {
        None
    }
}

/// Backwards-compat helper: extract just the field-name set. Used by the
/// `binding_self_check_extracts_known_fields` test (existing pre-E' assertion).
fn extract_struct_fields(source: &str, struct_name: &str) -> Option<BTreeSet<String>> {
    extract_struct_field_pairs(source, struct_name)
        .map(|pairs| pairs.into_iter().map(|(n, _)| n).collect())
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
        let actual = extract_struct_field_pairs(&source, b.struct_name);
        match (b.landing_status, actual) {
            (LandingStatus::NotYetLanded, None) => {
                // Expected: rolled-back struct not present in codebase.
            }
            (LandingStatus::NotYetLanded, Some(pairs)) => {
                failures.push(format!(
                    "[{}/{}] declared NotYetLanded but `pub struct {}` is present in {} \
                     with fields {:?}; flip landing_status to Landed in this binding when \
                     Phase F rebuilds the atom",
                    b.atom_id, b.struct_name, b.struct_name, b.impl_path, pairs
                ));
            }
            (LandingStatus::Landed, None) => {
                failures.push(format!(
                    "[{}/{}] declared Landed but `pub struct {}` not found in {}",
                    b.atom_id, b.struct_name, b.struct_name, b.impl_path
                ));
            }
            (LandingStatus::Landed, Some(actual_pairs)) => {
                // Compare on (name, type_first_token) pair sets — strict equality.
                use std::collections::BTreeSet;
                let expected_set: BTreeSet<(String, String)> = b
                    .expected_fields
                    .iter()
                    .map(|(n, t)| (n.to_string(), t.to_string()))
                    .collect();
                let actual_set: BTreeSet<(String, String)> =
                    actual_pairs.iter().cloned().collect();
                if expected_set != actual_set {
                    let extra: Vec<(String, String)> =
                        actual_set.difference(&expected_set).cloned().collect();
                    let missing: Vec<(String, String)> =
                        expected_set.difference(&actual_set).cloned().collect();
                    failures.push(format!(
                        "[{}/{}] verbatim drift vs architect manual {}: \
                         extra (name,type) pairs in impl {:?}; missing (name,type) pairs \
                         in impl {:?}; architect verbatim spec is exactly {:?}",
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

#[test]
fn binding_self_check_synthetic_d4_shape_field_rename_detected() {
    // Phase E' hardening per Codex 2026-05-09 Recommendation 2: synthetic
    // Rust source mimicking Codex defect 4 shape (P-M4 `event_id_kind` rename).
    // Both NAME drift (event_id → event_id_kind) and TYPE drift (EventId →
    // PoolEventKind) must be caught by extract_struct_field_pairs.
    let synthetic = r#"
pub struct CpmmPool_Synthetic {
    pub event_id_kind: PoolEventKind,
    pub pool_yes: ShareAmount,
    pub pool_no: ShareAmount,
    pub lp_total_shares: LpShareAmount,
    pub status: PoolStatus,
}
"#;
    let actual_pairs = extract_struct_field_pairs(synthetic, "CpmmPool_Synthetic")
        .expect("synthetic struct must parse");
    use std::collections::BTreeSet;
    let actual_set: BTreeSet<(String, String)> = actual_pairs.iter().cloned().collect();
    let expected_set: BTreeSet<(String, String)> = [
        ("event_id", "EventId"),
        ("pool_yes", "ShareAmount"),
        ("pool_no", "ShareAmount"),
        ("lp_total_shares", "LpShareAmount"),
        ("status", "PoolStatus"),
    ]
    .iter()
    .map(|(n, t)| (n.to_string(), t.to_string()))
    .collect();
    assert_ne!(
        actual_set, expected_set,
        "self-check (E' hardening): parser MUST detect the field rename \
         `event_id` → `event_id_kind` AND the type rename `EventId` → \
         `PoolEventKind` in synthetic CpmmPool_Synthetic vs architect §7.5 spec",
    );
    let extra: Vec<(String, String)> =
        actual_set.difference(&expected_set).cloned().collect();
    assert!(
        extra.iter().any(|(n, t)| n == "event_id_kind" && t == "PoolEventKind"),
        "self-check (E' hardening): expected to find `(event_id_kind, PoolEventKind)` \
         in the diff's `extra` set; got extras={:?}",
        extra,
    );
}

#[test]
fn binding_self_check_type_only_drift_detected() {
    // Phase E' hardening per Codex 2026-05-09 Recommendation 2: a hypothetical
    // future drift could PRESERVE the field name while corrupting only the
    // TYPE (e.g. `pub event_id: PoolEventKind`). The (name, type) pair check
    // catches this even though name-only check would miss it.
    let synthetic = r#"
pub struct CpmmPool_TypeOnlyDrift {
    pub event_id: PoolEventKind,
    pub pool_yes: ShareAmount,
    pub pool_no: ShareAmount,
    pub lp_total_shares: LpShareAmount,
    pub status: PoolStatus,
}
"#;
    let actual_pairs = extract_struct_field_pairs(synthetic, "CpmmPool_TypeOnlyDrift")
        .expect("synthetic struct must parse");
    let actual_names: BTreeSet<String> =
        actual_pairs.iter().map(|(n, _)| n.clone()).collect();
    let expected_names: BTreeSet<String> = [
        "event_id",
        "pool_yes",
        "pool_no",
        "lp_total_shares",
        "status",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    assert_eq!(
        actual_names, expected_names,
        "self-check (E' hardening): name-only check should NOT detect type-only \
         drift; expected names ARE preserved in TypeOnlyDrift fixture",
    );
    use std::collections::BTreeSet;
    let actual_set: BTreeSet<(String, String)> =
        actual_pairs.iter().cloned().collect();
    let expected_set: BTreeSet<(String, String)> = [
        ("event_id", "EventId"),
        ("pool_yes", "ShareAmount"),
        ("pool_no", "ShareAmount"),
        ("lp_total_shares", "LpShareAmount"),
        ("status", "PoolStatus"),
    ]
    .iter()
    .map(|(n, t)| (n.to_string(), t.to_string()))
    .collect();
    assert_ne!(
        actual_set, expected_set,
        "self-check (E' hardening): (name, type) pair check MUST detect the \
         type-only drift `event_id: EventId` → `event_id: PoolEventKind`",
    );
    let extra: Vec<(String, String)> =
        actual_set.difference(&expected_set).cloned().collect();
    assert!(
        extra.iter().any(|(n, t)| n == "event_id" && t == "PoolEventKind"),
        "self-check (E' hardening): expected `(event_id, PoolEventKind)` in diff \
         extras; got extras={:?}",
        extra,
    );
}
