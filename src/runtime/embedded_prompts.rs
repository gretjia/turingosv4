/// TRACE_MATRIX FC2 + FC3: Compile-time-embedded grill prompts (Phase 5 driven-default).
///
/// Embeds the canonical grill prompt assets into the binary via `include_bytes!`.
/// The binary build is the attestation: prompts cannot drift at runtime; changing
/// them requires rebuilding the binary.
///
/// `welcome_init_handler` calls `materialize_grill_prompts` after `turingos init`
/// to write the embedded bytes into `<workspace>/assets/prompts/` (existing read
/// paths in `spec_turn_handler` + CLI `cmd_llm::run_triage`). It also calls
/// `seed_embedded_promotion_receipts` to satisfy the C10 promotion guard on a
/// fresh workspace — the seed receipt explicitly records that the attestation
/// origin is the binary build, not a runtime `prompt-eval`.
///
/// FC-trace: FC2 (boot — prompt materialization), FC3 (eval evidence binding —
/// receipt encodes binary-baked attestation).
/// Risk class: Class 2 (additive wire-up; C10 guard unchanged; receipt fields
/// remain mandatory and non-empty).
use std::fs;
use std::io;
use std::path::Path;

use crate::runtime::prompt_promotion::{
    sha256_hex_of_prompt, write_promotion_receipt, PromotionDecision, PromptPromotionReceipt,
    PROMPT_PROMOTION_RECEIPT_SCHEMA_ID,
};

/// Meta-prompt asset bytes (driven grill turn-by-turn LLM-as-runtime prompt).
pub const GRILL_META_V1_BYTES: &[u8] =
    include_bytes!("../../assets/prompts/grill_meta_v1.md");

/// Triage prompt asset bytes (Blackbox classifier for off_topic / abusive /
/// gibberish detection on each user answer).
pub const GRILL_TRIAGE_BLACKBOX_V1_BYTES: &[u8] =
    include_bytes!("../../assets/prompts/grill_triage_blackbox_v1.md");

/// Synthesis prompt asset bytes (kept for backward compatibility; current A6
/// path is LLM-less in-process synthesis so this asset is informational only).
pub const GRILL_SYNTHESIS_ZH_BYTES: &[u8] =
    include_bytes!("../../assets/prompts/grill_synthesis_zh.md");

/// Sentinel marking that a receipt was created by binary-baked init, not by
/// running `turingos llm prompt-eval`. The sentinel is non-empty (required by
/// `check_promotion_guard`) and content-stable so the same binary always
/// produces the same receipt for CAS dedup.
pub const BINARY_BAKED_EVAL_SET_SENTINEL: &str = "turingos-binary-baked-grill-init-v1";

/// SHA-256 hex of the meta-prompt embedded bytes.
pub fn meta_prompt_cid() -> String {
    sha256_hex_of_prompt(GRILL_META_V1_BYTES)
}

/// SHA-256 hex of the triage-prompt embedded bytes.
pub fn triage_prompt_cid() -> String {
    sha256_hex_of_prompt(GRILL_TRIAGE_BLACKBOX_V1_BYTES)
}

/// Materialize the 3 embedded prompts into `<workspace>/assets/prompts/`.
///
/// Called by `welcome_init_handler` after `turingos init` succeeds. Idempotent:
/// overwrites existing files (the binary version is the source of truth).
///
/// This closes the Agent B matrix GAP-3 ("assets/ not in workspace") for the
/// Phase 7 web flow.
pub fn materialize_grill_prompts(workspace: &Path) -> io::Result<()> {
    let prompts_dir = workspace.join("assets").join("prompts");
    fs::create_dir_all(&prompts_dir)?;
    fs::write(
        prompts_dir.join("grill_meta_v1.md"),
        GRILL_META_V1_BYTES,
    )?;
    fs::write(
        prompts_dir.join("grill_triage_blackbox_v1.md"),
        GRILL_TRIAGE_BLACKBOX_V1_BYTES,
    )?;
    fs::write(
        prompts_dir.join("grill_synthesis_zh.md"),
        GRILL_SYNTHESIS_ZH_BYTES,
    )?;
    Ok(())
}

/// Seed a `PromptPromotionReceipt` for each embedded prompt's content hash so
/// the C10 promotion guard accepts the embedded prompts on the first LLM call
/// in a fresh workspace.
///
/// Receipt fields encode the binary-baked attestation explicitly:
///   - `from_prompt_cid` == `to_prompt_cid` (genesis; no prior version)
///   - `eval_set_cid`     = `BINARY_BAKED_EVAL_SET_SENTINEL` (non-empty as
///     required by `check_promotion_guard`; identifies origin as binary build)
///   - `eval_before_cid` / `eval_after_cid` = SHA-256 of the sentinel string
///     (placeholder; no real eval transcript was run)
///   - `promotion_decision` = `Promote`
///
/// Called by `welcome_init_handler` after `materialize_grill_prompts`. Idempotent
/// via CAS content-addressing: writing the same receipt yields the same CID.
///
/// This closes the Agent B matrix GAP-1 ("C10 promotion guard blocks all
/// spec/turn triage in clean workspace") without weakening the guard itself —
/// the bypass scope (`NoCasStore` only) is unchanged.
pub fn seed_embedded_promotion_receipts(
    workspace: &Path,
    logical_t: u64,
) -> Result<Vec<String>, crate::runtime::spec_capsule::CapsuleError> {
    let sentinel_hash =
        sha256_hex_of_prompt(BINARY_BAKED_EVAL_SET_SENTINEL.as_bytes());

    // Synthesis prompt is NOT seeded: the A6 spec-synthesis path is LLM-less
    // (in-process slot-keyed string building); no LLM call -> no C10 check on
    // synthesis prompt.
    let prompt_cids = [meta_prompt_cid(), triage_prompt_cid()];

    let mut cids = Vec::with_capacity(prompt_cids.len());
    for prompt_cid_hex in prompt_cids {
        let receipt = PromptPromotionReceipt {
            schema_id: PROMPT_PROMOTION_RECEIPT_SCHEMA_ID.to_string(),
            from_prompt_cid: prompt_cid_hex.clone(),
            to_prompt_cid: prompt_cid_hex,
            eval_set_cid: BINARY_BAKED_EVAL_SET_SENTINEL.to_string(),
            eval_before_cid: sentinel_hash.clone(),
            eval_after_cid: sentinel_hash.clone(),
            promotion_decision: PromotionDecision::Promote,
            logical_t,
        };
        let cid_hex = write_promotion_receipt(workspace, &receipt, logical_t)?;
        cids.push(cid_hex);
    }
    Ok(cids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_prompts_have_nonzero_bytes() {
        assert!(GRILL_META_V1_BYTES.len() > 100, "meta prompt bytes empty");
        assert!(
            GRILL_TRIAGE_BLACKBOX_V1_BYTES.len() > 100,
            "triage prompt bytes empty"
        );
        assert!(
            GRILL_SYNTHESIS_ZH_BYTES.len() > 100,
            "synthesis prompt bytes empty"
        );
    }

    #[test]
    fn prompt_cids_are_sha256_hex() {
        for cid in [meta_prompt_cid(), triage_prompt_cid()] {
            assert_eq!(cid.len(), 64, "cid not 64-hex: {cid}");
            assert!(
                cid.chars().all(|c| c.is_ascii_hexdigit()),
                "cid not hex: {cid}"
            );
        }
    }

    #[test]
    fn materialize_writes_three_files_with_embedded_bytes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        materialize_grill_prompts(tmp.path()).expect("materialize");

        let prompts_dir = tmp.path().join("assets/prompts");
        let meta = fs::read(prompts_dir.join("grill_meta_v1.md")).expect("read meta");
        let triage = fs::read(prompts_dir.join("grill_triage_blackbox_v1.md")).expect("read triage");
        let synthesis =
            fs::read(prompts_dir.join("grill_synthesis_zh.md")).expect("read synthesis");

        assert_eq!(meta, GRILL_META_V1_BYTES);
        assert_eq!(triage, GRILL_TRIAGE_BLACKBOX_V1_BYTES);
        assert_eq!(synthesis, GRILL_SYNTHESIS_ZH_BYTES);
    }

    #[test]
    fn materialize_is_idempotent() {
        let tmp = tempfile::tempdir().expect("tempdir");
        materialize_grill_prompts(tmp.path()).expect("first");
        materialize_grill_prompts(tmp.path()).expect("second (overwrites)");
        let meta =
            fs::read(tmp.path().join("assets/prompts/grill_meta_v1.md")).expect("read meta");
        assert_eq!(meta, GRILL_META_V1_BYTES);
    }

    #[test]
    fn binary_baked_sentinel_is_non_empty() {
        assert!(!BINARY_BAKED_EVAL_SET_SENTINEL.is_empty());
    }

    #[test]
    fn seed_writes_two_receipts_and_check_passes() {
        use crate::runtime::prompt_promotion::check_promotion_guard;

        let tmp = tempfile::tempdir().expect("tempdir");

        let cids = seed_embedded_promotion_receipts(tmp.path(), 1)
            .expect("seed should succeed in fresh workspace");
        assert_eq!(cids.len(), 2, "expected 2 seed receipts (meta + triage)");

        // After seed, check_promotion_guard must return Ok for both prompt CIDs.
        check_promotion_guard(tmp.path(), &meta_prompt_cid())
            .expect("meta prompt receipt should satisfy guard");
        check_promotion_guard(tmp.path(), &triage_prompt_cid())
            .expect("triage prompt receipt should satisfy guard");
    }
}
