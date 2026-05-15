// A8e fix F1 — single run_id minted once per run.
//
// Codex#2 + Gemini Q4 (A8 round-1 audit): `run_corr_id` was generated
// at run_swarm/run_oneshot entry while make_pput's internal `run_id`
// was recomputed at the terminal emit site, causing millisecond drift
// between the two identifiers. Phase D consumers cannot reliably join
// FC events (stamped with run_corr_id) to v2 jsonl rows (stamped with
// run_id). Oneshot was even worse — it used `oneshot_{problem_file}`
// as the FC correlation key, completely disjoint from the eventual
// PputResult.run_id format.
//
// Fix: mint ONE run_id at function entry, thread to both emit_event
// and make_pput. Format mirrors the prior make_pput format
// (`{condition}_{problem_id}_{unix_ms}`) so existing v2 jsonl rows
// don't change shape.

/// TRACE_MATRIX correlation: stable per-run identifier. Format is
/// `{condition}_{problem_id}_{unix_ms}` where `problem_id` is the
/// file-stem of the .lean file (no extension). Phase D consumers join
/// on this exact string between fc_trace events and v2 jsonl rows.
pub fn mint_run_id(condition: &str, problem_file: &str) -> String {
    let problem_id = std::path::Path::new(problem_file)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(problem_file);
    let ts_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    format!("{}_{}_{}", condition, problem_id, ts_ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_matches_legacy_make_pput_format() {
        // Legacy make_pput used: format!("{}_{}_{}", condition, problem_id, ts).
        // Same shape preserves backward compat with downstream tools that
        // already parsed v2 jsonl run_id strings.
        let id = mint_run_id("n3", "/tmp/foo.lean");
        let parts: Vec<&str> = id.splitn(3, '_').collect();
        assert_eq!(parts[0], "n3");
        assert_eq!(parts[1], "foo");
        assert!(
            parts[2].parse::<u128>().is_ok(),
            "third segment must be unix-ms timestamp, got: {}",
            parts[2]
        );
    }

    #[test]
    fn handles_path_with_no_stem() {
        // Defensive: passing the literal path falls back to the input
        // string (avoids panicking on weird inputs).
        let id = mint_run_id("oneshot", "/");
        assert!(id.starts_with("oneshot_"));
    }

    #[test]
    fn distinguishes_conditions_for_same_problem() {
        let a = mint_run_id("n3", "/tmp/p.lean");
        let b = mint_run_id("oneshot", "/tmp/p.lean");
        assert!(a.starts_with("n3_p_"));
        assert!(b.starts_with("oneshot_p_"));
    }
}
