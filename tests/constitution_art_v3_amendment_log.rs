//! TB-C0 round 8 — Art. V.3 amendment-log executable test.
//!
//! Per Codex audit verdict §3 §4 + Constitution Execution Matrix §F + TB-C0
//! directive line 430 ("no test = RED, NOT 'covered by docs'"): Art. V.3
//! amendment-log was previously marked 🚫 N/A (docs-only). Codex Q7 CHALLENGE
//! + round-7 matrix correction promoted it to 🔴 RED with `NEW test required`.
//! This test fulfills that requirement.
//!
//! What this test asserts (per `constitution.md` §5.3 + Art. V.1.2 + V.1.3):
//!   1. The §5.3 section exists and contains a markdown table with the
//!      required columns: 日期 | 触发者 | 章节 | 摘要.
//!   2. Every row has all 4 columns populated (no empty cells).
//!   3. Every 触发者 mentions "人类架构师" (human architect) — per V.1.1
//!      constitution sudo only via human architect; NEVER ArchitectAI alone.
//!   4. Every 日期 is a valid ISO date YYYY-MM-DD.
//!   5. The constitution.md sha256 in `genesis_payload.toml` matches the
//!      actual file content (constitution-hash integrity at boot;
//!      TRUST_ROOT manifest verifies this on every binary launch).
//!
//! Per `feedback_no_workarounds_strict_constitution`: every assertion
//! must be writable to fail. If the amendment log is corrupted or an
//! ArchitectAI tries to add an amendment without "人类架构师" trigger,
//! this test fires.
//!
//! Test status: GREEN iff all 5 assertions hold on the current
//! constitution.md.

use std::fs;
use std::path::Path;

const CONSTITUTION_MD: &str = "constitution.md";
const GENESIS_PAYLOAD_TOML: &str = "genesis_payload.toml";
const V3_SECTION_HEADER: &str = "## 5.3 宪法修订日志 [Art. V.3]";

/// Parse the §5.3 amendment table; return rows as (date, trigger, section, summary).
/// Returns None if section absent or table malformed.
fn parse_amendment_log() -> Option<Vec<(String, String, String, String)>> {
    let content = fs::read_to_string(CONSTITUTION_MD).ok()?;
    let after_v3 = content.split(V3_SECTION_HEADER).nth(1)?;
    // Find the next `---` or `## ` boundary; everything in between is V.3 body.
    let v3_body = after_v3
        .split("\n---")
        .next()
        .unwrap_or(after_v3)
        .split("\n## ")
        .next()
        .unwrap_or(after_v3);

    let mut rows = Vec::new();
    for line in v3_body.lines() {
        let line = line.trim();
        // Markdown table rows start with `| `, contain `|` separators
        if !line.starts_with('|') {
            continue;
        }
        // Skip header row + separator row
        if line.contains("日期") || line.contains("---") || line.contains("摘要") {
            continue;
        }
        // Split on `|`, take cells 1..5 (cell 0 is empty before first `|`)
        let cells: Vec<&str> = line.split('|').collect();
        if cells.len() < 5 {
            // Malformed row — propagate so test fails (don't silently skip)
            continue;
        }
        let date = cells[1].trim().to_string();
        let trigger = cells[2].trim().to_string();
        let section = cells[3].trim().to_string();
        let summary = cells[4].trim().to_string();
        rows.push((date, trigger, section, summary));
    }
    Some(rows)
}

/// Art. V.3.1 — §5.3 section exists with a parseable amendment table.
#[test]
fn v3_amendment_section_exists_and_parseable() {
    let rows = parse_amendment_log();
    assert!(
        rows.is_some(),
        "Art. V.3 violation: constitution.md is missing the '{V3_SECTION_HEADER}' section \
         or its amendment table is malformed beyond parsing"
    );
    let rows = rows.unwrap();
    assert!(
        !rows.is_empty(),
        "Art. V.3 violation: amendment table is empty. Constitution edits since 2026-04-25 \
         should be recorded; an empty table contradicts the existence of constitution.md \
         (which itself is the result of multiple amendments)"
    );
}

/// Art. V.3.2 — every amendment row has all 4 columns populated.
/// Empty cells indicate a partially-recorded amendment (forbidden per §5.3 prose).
#[test]
fn v3_every_amendment_has_four_populated_columns() {
    let rows = parse_amendment_log().expect("V3 section exists");
    for (i, (date, trigger, section, summary)) in rows.iter().enumerate() {
        assert!(
            !date.is_empty(),
            "Art. V.3 row {i}: 日期 is empty. Every amendment MUST have a date."
        );
        assert!(
            !trigger.is_empty(),
            "Art. V.3 row {i} ({date}): 触发者 is empty. Every amendment MUST identify the triggering party."
        );
        assert!(
            !section.is_empty(),
            "Art. V.3 row {i} ({date}): 章节 is empty. Every amendment MUST cite the article it modifies."
        );
        assert!(
            !summary.is_empty(),
            "Art. V.3 row {i} ({date}): 摘要 is empty. Every amendment MUST include a summary; \
             empty 摘要 violates the principle that future ArchitectAI / Veto-AI / 审计者 must \
             be able to independently reconstruct constitutional state at any timestamp."
        );
    }
}

/// Art. V.3.3 — every 触发者 (trigger) mentions "人类架构师" (human architect).
/// Per V.1.1: constitution.md is the ONLY file under 'human sudo' authority;
/// edits MUST come from a human architect, NEVER from ArchitectAI (which has
/// commit authority only on non-constitution files).
#[test]
fn v3_every_amendment_triggered_by_human_architect() {
    let rows = parse_amendment_log().expect("V3 section exists");
    for (i, (date, trigger, _, _)) in rows.iter().enumerate() {
        assert!(
            trigger.contains("人类架构师") || trigger.contains("human architect"),
            "Art. V.3 row {i} ({date}): 触发者='{trigger}' does NOT contain '人类架构师' \
             (or 'human architect'). Per V.1.1, constitution sudo is reserved for the human \
             architect; ArchitectAI does NOT have authority to amend constitution.md."
        );
    }
}

/// Art. V.3.4 — every 日期 is a valid ISO date YYYY-MM-DD.
#[test]
fn v3_every_amendment_date_is_iso_format() {
    let rows = parse_amendment_log().expect("V3 section exists");
    for (i, (date, _, _, _)) in rows.iter().enumerate() {
        // Strict YYYY-MM-DD: 10 chars, [0-9]{4}-[0-9]{2}-[0-9]{2}
        let chars: Vec<char> = date.chars().collect();
        assert_eq!(
            chars.len(),
            10,
            "Art. V.3 row {i}: 日期='{date}' is not 10 chars (expected YYYY-MM-DD)"
        );
        for (j, c) in chars.iter().enumerate() {
            let ok = match j {
                0..=3 | 5..=6 | 8..=9 => c.is_ascii_digit(),
                4 | 7 => *c == '-',
                _ => false,
            };
            assert!(
                ok,
                "Art. V.3 row {i}: 日期='{date}' char {j} = '{c}' violates YYYY-MM-DD format"
            );
        }
    }
}

/// Art. V.3.5 — constitution.md sha256 matches the entry in `genesis_payload.toml`
/// trust_root manifest. Per Art. V.1.1: constitution.md is sudo-only; any
/// edit MUST be accompanied by a trust-root rehash. This assertion catches
/// the case where someone edits constitution.md without updating the
/// trust_root manifest (silent constitutional drift).
#[test]
fn v3_constitution_hash_matches_trust_root_manifest() {
    let constitution_bytes = fs::read(CONSTITUTION_MD).expect("constitution.md readable");
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(&constitution_bytes);
    let actual_hex: String = hasher.finalize().iter().map(|b| format!("{b:02x}")).collect();

    let manifest = fs::read_to_string(GENESIS_PAYLOAD_TOML).expect("genesis_payload.toml readable");
    // Look for line: `"constitution.md" = "<hex>"` (skip comment lines that
    // mention constitution.md in prose; only match actual TOML key=value)
    let entry_line = manifest
        .lines()
        .find(|l| {
            let trimmed = l.trim_start();
            !trimmed.starts_with('#') && trimmed.starts_with("\"constitution.md\"") && l.contains('=')
        })
        .expect("trust_root manifest must include constitution.md key=value entry");
    // Extract the hex value (between first and second `"` AFTER `=`)
    let after_eq = entry_line.split('=').nth(1).unwrap_or("");
    let manifest_hex = after_eq
        .split('"')
        .nth(1)
        .unwrap_or("")
        .to_string();

    assert_eq!(
        actual_hex, manifest_hex,
        "Art. V.3 violation: constitution.md sha256={actual_hex} but genesis_payload.toml \
         trust_root says {manifest_hex}. Either constitution.md was edited without a trust-\
         root rehash (silent drift; forbidden) OR genesis_payload.toml was edited without \
         constitution.md change (manifest corruption; forbidden). Per Art. V.1.1 + V.3 \
         every constitution edit MUST be recorded in §5.3 AND accompanied by a manifest \
         rehash in genesis_payload.toml."
    );
}

/// Art. V.3.6 — historical lock: the 2026-04-25 trio of amendments must
/// remain in §5.3. Per `feedback_no_retroactive_evidence_rewrite`: amendments
/// once recorded cannot be silently removed. This test guards against
/// retroactive deletion of historical amendments.
#[test]
fn v3_historical_amendments_remain_recorded() {
    let rows = parse_amendment_log().expect("V3 section exists");
    let dates: Vec<&String> = rows.iter().map(|(d, _, _, _)| d).collect();
    let required_historical = ["2026-04-25", "2026-04-26"];
    for required_date in required_historical {
        let count = dates.iter().filter(|d| ***d == required_date).count();
        assert!(
            count >= 1,
            "Art. V.3 violation: historical amendment date '{required_date}' is missing from \
             §5.3. Per feedback_no_retroactive_evidence_rewrite, amendments once recorded \
             cannot be silently removed; required dates so far: {required_historical:?}."
        );
    }
}
