//! LIVE CONSTITUTION GATE (conformance #5) — goodhart-shield witness:
//! the named runtime PPUT-context-leak guard must be CALLED at every final
//! prompt-assembly site, not left pinned-but-dead.
//!
//! ── WHAT THIS GATE ENFORCES (the wired-guard invariant) ───────────────────
//! Art. III.4 forbids a measurement scalar (PPUT / H-VPPUT / WBCG) from entering
//! agent context (Goodhart shield). `src/sdk/prompt_guard.rs::assert_no_metric_leak`
//! is the RUNTIME enforcement of that clause: it scans the FINAL assembled prompt
//! at the LLM-call boundary. The sweep found it had ZERO production callers — it
//! was trust-root pinned (`genesis_payload.toml`) yet wired to nothing, so the
//! Art. III.4 runtime enforcement site was vacuum. Pinning a file's HASH does not
//! make it RUN.
//!
//! The invariant is structural and completeness-shaped: it ENUMERATES every final
//! prompt-assembly site that delivers a prompt to an LLM — the kernel
//! `assemble_o1_prompt` (`src/memory_kernel.rs`), the agent prompt builder
//! `build_agent_prompt` (`src/sdk/prompt.rs`), and the market runner prompt
//! builder (`src/bin/market_external_agent_current_kernel.rs`) — and asserts EACH
//! calls `assert_no_metric_leak(` before returning the prompt. It also asserts the
//! guard is not dead: at least one PRODUCTION (non-test, non-self) caller exists.
//! A future prompt-assembly site that forgets the guard, or a revert that removes
//! a call, turns this gate RED. It is a SOURCE-STRUCTURAL witness (same family as
//! `tests/constitution_single_admission_contract.rs`): it greps the canonical
//! sources, so it cannot be satisfied by a vacuous `assert!(true)`.
//!
//! ── TRIPLE-COUPLING ──────────────────────────────────────────────────────
//! Registered in `scripts/constitution_gates.manifest.toml`
//! (`constitution_metric_leak_guard_wired`) and referenced in
//! `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`. Discovered by the flat
//! `ls tests/constitution_*.rs` glob in `scripts/run_constitution_gates.sh`.

use std::fs;

const GUARD: &str = "assert_no_metric_leak(";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("{path} readable"))
}

/// Extract the body of `fn <name>(` … balanced `}` from a source string.
fn fn_body(src: &str, fn_sig_anchor: &str) -> String {
    let at = src
        .find(fn_sig_anchor)
        .unwrap_or_else(|| panic!("function anchor `{fn_sig_anchor}` not found"));
    // Find the opening brace of the function body (first `{` after the signature
    // anchor that is the body — we accept the first `{` after the `)` that ends
    // the param list; for these functions the first `{` after the anchor is the
    // body brace).
    let brace_open = src[at..]
        .find('{')
        .map(|o| at + o)
        .expect("function body must have an opening brace");
    let bytes = src.as_bytes();
    let mut depth = 0usize;
    let mut i = brace_open;
    while i < bytes.len() {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return src[brace_open..=i].to_string();
                }
            }
            _ => {}
        }
        i += 1;
    }
    panic!("unbalanced braces in function body for `{fn_sig_anchor}`");
}

/// ENUMERATE-ALL-SITES completeness: every final prompt-assembly function calls
/// the guard. Each entry is (file, fn-signature-anchor).
const PROMPT_ASSEMBLY_SITES: &[(&str, &str)] = &[
    ("src/memory_kernel.rs", "pub fn assemble_o1_prompt"),
    ("src/sdk/prompt.rs", "pub fn build_agent_prompt"),
    (
        "src/bin/market_external_agent_current_kernel.rs",
        "fn build_agent_prompt(",
    ),
];

#[test]
fn every_final_prompt_assembly_site_calls_the_metric_leak_guard() {
    for (path, anchor) in PROMPT_ASSEMBLY_SITES {
        let src = read(path);
        let body = fn_body(&src, anchor);
        assert!(
            body.contains(GUARD),
            "goodhart-shield violation (conformance #5): the final prompt-assembly \
             function `{anchor}` in {path} does NOT call `{GUARD}` before \
             delivering the prompt to the LLM. Art. III.4 requires the runtime \
             PPUT-context-leak guard at every prompt-delivery boundary. Add \
             `assert_no_metric_leak(&prompt)` just before returning the prompt."
        );
    }
}

/// The guard is not pinned-but-dead: at least one PRODUCTION caller exists
/// outside its own module and outside test code. We count call sites in the
/// enumerated production prompt-assembly files (which is exactly where the wiring
/// must live). If all of them lost the call, this and the test above go RED
/// together.
#[test]
fn metric_leak_guard_has_production_callers() {
    let mut production_callers = 0usize;
    for (path, _) in PROMPT_ASSEMBLY_SITES {
        let src = read(path);
        // Count guard call occurrences that are NOT inside a `#[cfg(test)]`
        // module. These production files keep their tests (if any) below; the
        // wiring call is in the production fn body. A simple, robust proxy: count
        // occurrences and subtract any that appear after a `#[cfg(test)]` marker.
        let cutoff = src.find("#[cfg(test)]").unwrap_or(src.len());
        production_callers += src[..cutoff].matches(GUARD).count();
    }
    assert!(
        production_callers >= 1,
        "goodhart-shield violation (conformance #5): `assert_no_metric_leak` has \
         ZERO production callers across the enumerated prompt-assembly sites. The \
         guard is trust-root pinned but wired to nothing (Art. III.4 runtime \
         enforcement site is vacuum). Wire it at the prompt-delivery boundaries."
    );

    // Also assert the guard still EXISTS in its (pinned) home, so the call sites
    // resolve to a real symbol rather than a stub.
    let guard_src = read("src/sdk/prompt_guard.rs");
    assert!(
        guard_src.contains("pub fn assert_no_metric_leak"),
        "goodhart-shield gate: src/sdk/prompt_guard.rs no longer defines \
         `pub fn assert_no_metric_leak`. The wired call sites would not compile; \
         re-establish the guard (and its trust-root pin)."
    );
}
