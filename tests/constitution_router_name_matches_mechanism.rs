//! Forensic gate vs failure-mode #4 (name-lies) — 2026-06-01 retrospective
//! (`handover/reports/SESSION_FORENSIC_RETROSPECTIVE_2026-06-01.md` §1.C + §1.A).
//!
//! Two name-lies produced wrong conclusions this campaign and passed every gate:
//!   1. `boltzmann_select_parent_v2` (`src/sdk/actor.rs:46`) is **argmax + ε-uniform**,
//!      NOT the Art. II.2.1 Boltzmann softmax its name advertises. Used as the
//!      "market" router it collapsed every agent onto the single highest-price node →
//!      multi-agent ≈ single-agent (the exact Path-1 failure). The TRUE softmax lives
//!      unused-by-most at `actor.rs:115` (`boltzmann_softmax_select_parent`).
//!   2. `src/bin/lean_hetero_market.rs` advertised "price = which specialist is needed"
//!      and `lean_tree_market.rs` advertised a "price-routed tree search" — both with
//!      **zero price machinery in code** (`price` appears only in a comment). The win
//!      was attributed to price routing that does not exist.
//!
//! This gate makes both name-lies FALSIFIABLE, structurally:
//!   A. the named **softmax** router MUST actually DISTRIBUTE attention (sample ≥3 of
//!      5 equal-price nodes), where the argmax selector provably collapses to 1 — so a
//!      future regression of the real softmax to argmax-by-another-name goes RED;
//!   B. any `src/bin/*.rs` that **claims** price-based routing in prose MUST carry a
//!      real `price` identifier in *code* (comments stripped) — a claim with zero code
//!      price machinery is a name-lie and goes RED; and the canonical priced-softmax
//!      substrate `lean_market_agent.rs` MUST keep referencing both `compute_price_index`
//!      and `boltzmann_softmax_select_parent`.
//!
//! Per `AGENTS.md` §7 ("a test that cannot fail is documentation, not a gate"), every
//! property below is paired with a control proving the checker bites and does not
//! over-fire. This gate is anti-name-lie ONLY; it makes no claim about whether any
//! result is correct (that is the `/no-proven-checklist` skill's job).

use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use rand::rngs::StdRng;
use rand::SeedableRng;
use turingosv4::sdk::actor::{boltzmann_select_parent_v2, boltzmann_softmax_select_parent};
use turingosv4::state::{BoltzmannMaskPolicy, NodeMarketEntry, RationalPrice, TxId};

// ── A. the softmax router must DISTRIBUTE, the argmax selector must COLLAPSE ──

/// Build a price index of `n` nodes that ALL carry the identical price `num/den`.
/// Equal prices are the sharpest discriminator: a softmax over equal energies is the
/// uniform distribution (must spread), whereas an argmax over a tie deterministically
/// returns the lexicographically-first node (must collapse to exactly one).
fn equal_price_index(n: usize, num: u128, den: u128) -> BTreeMap<TxId, NodeMarketEntry> {
    (0..n)
        .map(|i| {
            (
                TxId(format!("node{i:02}")),
                NodeMarketEntry {
                    price_yes: Some(RationalPrice { numerator: num, denominator: den }),
                    ..Default::default()
                },
            )
        })
        .collect()
}

/// Distinct nodes a closure picks over `draws` calls (a fixed seed → deterministic).
fn distinct_picks(draws: usize, mut pick: impl FnMut(&mut StdRng) -> Option<TxId>) -> usize {
    let mut rng = StdRng::seed_from_u64(0xF0_7E_2026);
    let mut seen: BTreeSet<TxId> = BTreeSet::new();
    for _ in 0..draws {
        if let Some(id) = pick(&mut rng) {
            seen.insert(id);
        }
    }
    seen.len()
}

#[test]
fn softmax_router_distributes_attention_not_argmax() {
    // The named "softmax" selector, sampled over 5 EQUAL-price nodes, must spread
    // attention. If its body ever regresses to argmax (the C3 name-lie), this goes RED.
    let pi = equal_price_index(5, 50, 100);
    let empty: BTreeSet<TxId> = BTreeSet::new();
    let distinct = distinct_picks(400, |rng| {
        boltzmann_softmax_select_parent(&pi, &empty, 1.0, rng)
    });
    assert!(
        distinct >= 3,
        "boltzmann_softmax_select_parent must DISTRIBUTE over equal-price nodes \
         (Art. II.2.1); collapsing to {distinct}/5 distinct is the argmax name-lie"
    );
}

#[test]
fn softmax_distributes_even_under_near_equal_prices() {
    // A slight price gradient (0.50..0.54) must still distribute under a moderate
    // temperature — guards against a "softmax with τ→0" masquerade that is argmax in
    // disguise. Five close-but-distinct prices; expect broad sampling.
    let pi: BTreeMap<TxId, NodeMarketEntry> = (0..5)
        .map(|i| {
            (
                TxId(format!("n{i}")),
                NodeMarketEntry {
                    price_yes: Some(RationalPrice { numerator: 50 + i as u128, denominator: 100 }),
                    ..Default::default()
                },
            )
        })
        .collect();
    let empty: BTreeSet<TxId> = BTreeSet::new();
    let distinct = distinct_picks(400, |rng| boltzmann_softmax_select_parent(&pi, &empty, 0.2, rng));
    assert!(
        distinct >= 3,
        "softmax at τ=0.2 over near-equal prices must still distribute; got {distinct}/5"
    );
}

/// Control (§7 — the gate can fail): the argmax+ε selector at ε=0, over the SAME
/// equal-price index, collapses to exactly ONE node. This proves (i) the distribute
/// property genuinely discriminates argmax, so a softmax-named-but-argmax router WOULD
/// be caught by the test above, and (ii) the two selectors are not interchangeable.
#[test]
fn argmax_selector_collapses_proving_distribute_check_discriminates() {
    let pi = equal_price_index(5, 50, 100);
    let empty: BTreeSet<TxId> = BTreeSet::new();
    let policy = BoltzmannMaskPolicy {
        epsilon_exploration_num: 0,
        epsilon_exploration_den: 1,
        ..BoltzmannMaskPolicy::default()
    };
    let distinct = distinct_picks(400, |rng| boltzmann_select_parent_v2(&pi, &empty, &policy, rng));
    assert_eq!(
        distinct, 1,
        "argmax (boltzmann_select_parent_v2, ε=0) must collapse to ONE node on equal \
         prices — if this ever distributes the discriminator is broken, not the router"
    );
}

// ── B. a "price routing" claim must be backed by real price machinery in CODE ──

/// Affirmative price-routing claim phrases (lowercased substrings). A bin that uses any
/// of these to describe its selection is ADVERTISING price-based routing. Negated /
/// corrective prose (e.g. "price-based was a name-lie", "no price machinery") avoids
/// these exact tokens, so an honest CORRECTION does not re-trip the gate.
const PRICE_ROUTING_CLAIMS: &[&str] = &[
    "price-rout",     // price-routed / price-routes / price-routing (hyphenated)
    "price rout",     // "price routes" / "price routing" (spaced)
    "price = which",  // lean_hetero's "(price = which specialist is needed)"
    "route by price",
    "routed by price",
    "routes by price",
    "price-guided",
];

/// True iff `src` advertises price-based routing anywhere (prose or code).
fn claims_price_routing(src: &str) -> bool {
    let low = src.to_lowercase();
    PRICE_ROUTING_CLAIMS.iter().any(|c| low.contains(c))
}

/// Strip Rust comments (`//` line, `/* */` block) and the contents of `"`-strings'
/// delimiters are preserved but `//` inside a string is NOT treated as a comment. Used
/// to detect a price identifier in CODE rather than in marketing prose.
fn strip_comments(src: &str) -> String {
    let mut out = String::new();
    let mut chars = src.chars().peekable();
    let (mut in_block, mut in_line, mut in_str) = (false, false, false);
    while let Some(c) = chars.next() {
        if in_line {
            if c == '\n' {
                in_line = false;
                out.push('\n');
            }
            continue;
        }
        if in_block {
            if c == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_block = false;
            } else if c == '\n' {
                out.push('\n');
            }
            continue;
        }
        if in_str {
            out.push(c);
            if c == '\\' {
                if let Some(n) = chars.next() {
                    out.push(n);
                }
            } else if c == '"' {
                in_str = false;
            }
            continue;
        }
        if c == '/' && chars.peek() == Some(&'/') {
            chars.next();
            in_line = true;
        } else if c == '/' && chars.peek() == Some(&'*') {
            chars.next();
            in_block = true;
        } else if c == '"' {
            in_str = true;
            out.push(c);
        } else {
            out.push(c);
        }
    }
    out
}

/// True iff the CODE (comments stripped) carries a `price` identifier — the minimal
/// structural evidence that real price machinery exists (`compute_price_index`,
/// `price_index`, `price_pm`, `_price`, …), as opposed to the word living only in a doc
/// comment.
fn has_code_price_identifier(src: &str) -> bool {
    strip_comments(src).to_lowercase().contains("price")
}

/// (path, source) for every `src/bin/*.rs`. Read from package root (cargo test CWD),
/// mirroring the proven relative read in `cli_entry_files_redirect_to_agents.rs`.
fn bin_sources() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for entry in fs::read_dir("src/bin").expect("src/bin readable") {
        let path = entry.expect("dir entry").path();
        if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            let src = fs::read_to_string(&path).expect("bin readable");
            out.push((path.to_string_lossy().into_owned(), src));
        }
    }
    assert!(out.len() >= 5, "expected several src/bin/*.rs, got {}", out.len());
    out
}

#[test]
fn price_routing_claim_requires_code_price_machinery() {
    let violations: Vec<String> = bin_sources()
        .into_iter()
        .filter(|(_, src)| claims_price_routing(src) && !has_code_price_identifier(src))
        .map(|(p, _)| {
            format!("{p}: advertises price-based routing but has ZERO `price` identifier in code (name-lie #4)")
        })
        .collect();
    assert!(
        violations.is_empty(),
        "src/bin file(s) claim price routing with no price machinery in code:\n{}\n\
         Fix = wire real price machinery, OR add a CORRECTION annotation scoping the \
         claim to what the code actually does (see forensic retrospective §1.A/§1.C).",
        violations.join("\n")
    );
}

#[test]
fn lean_market_agent_is_canonical_priced_softmax_substrate() {
    // The retrospective blesses lean_market_agent.rs as the ONE bin with loss-bearing
    // price + TRUE softmax over the live price index + arbitrary-parent restart. Lock
    // it: stripping the price index or swapping the softmax for argmax goes RED.
    let src = fs::read_to_string("src/bin/lean_market_agent.rs").expect("lean_market_agent readable");
    assert!(
        src.contains("compute_price_index"),
        "lean_market_agent.rs must route over a real price index (compute_price_index)"
    );
    assert!(
        src.contains("boltzmann_softmax_select_parent"),
        "lean_market_agent.rs must use the TRUE softmax (boltzmann_softmax_select_parent), \
         not the argmax boltzmann_select_parent_v2"
    );
}

// ── Controls: the price-routing scanner must bite, and must not over-fire ──

#[test]
fn scanner_flags_a_price_claim_with_no_code_machinery() {
    // Synthetic name-lie: a module doc advertises price routing; the code never touches
    // price. The scanner MUST flag it (proves the gate can fail).
    let lie = "//! market: price-routed selection over specialists.\n\
               fn pick(open: &[usize], rng: &mut R) -> usize { open[rng.gen_range(0..open.len())] }\n";
    assert!(claims_price_routing(lie), "scanner must see the price-routing claim");
    assert!(
        !has_code_price_identifier(lie),
        "scanner must see ZERO code price machinery in the synthetic name-lie"
    );
}

#[test]
fn scanner_passes_a_genuinely_priced_bin() {
    // Synthetic honest bin: claims price routing AND carries price machinery in code.
    let honest = "//! market: price-routed parent selection.\n\
                  let pi = compute_price_index(&econ); let p = pi.get(&id);\n";
    assert!(claims_price_routing(honest));
    assert!(
        has_code_price_identifier(honest),
        "a bin whose code calls compute_price_index must pass the price-machinery check"
    );
}

#[test]
fn comment_stripper_removes_doc_price_but_keeps_code_price() {
    // The discriminating engine: `price` in a doc comment is stripped; `price` in code
    // survives. This is exactly what separates lean_hetero (comment-only) from
    // lean_market_agent (code).
    let only_comment = "//! routes by price = which specialist.\nfn f() { let x = 1; }\n";
    let in_code = "//! a comment.\nlet pi = compute_price_index(&e);\n";
    assert!(!has_code_price_identifier(only_comment), "comment-only `price` must be stripped");
    assert!(has_code_price_identifier(in_code), "code `price` must survive stripping");
    // `//` inside a string literal must NOT swallow following code on that line.
    let url_then_code =
        "let u = \"http://x\"; let price_pm = 7;\n";
    assert!(
        has_code_price_identifier(url_then_code),
        "a // inside a string must not be treated as a comment that hides code price"
    );
}
