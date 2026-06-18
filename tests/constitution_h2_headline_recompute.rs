//! GA-2 — H-HET-2 headline ingredient recompute from fixture tape (§17.1-G1).
//!
//! Authority: H-HET-2 Phase-2 gate design GA-2
//! (`handover/tracer_bullets/H_HET_2_PHASE2_GATE_DESIGN_2026-06-16.md`).
//!
//! ENFORCES (§17.1-G1 recompute-from-tape): the H-HET-2 *headline* economic
//! ingredients — union-delta coverage, per-model budget share, token total,
//! and the per-solved-unit token cost (PPUT) — must be recomputable FROM the
//! frozen fixture tape alone, not read from a sidecar.
//!
//! FIXTURE: a Vec of per-attempt records pairing
//!   (model_id, allocated_token_budget, TokenCounts, verified)
//! over 2 models ("alpha", "beta") and 3 targets ("T1", "T2", "T3") so that
//! the heterogeneous set covers a target that neither single model alone can
//! cover:
//!   - alpha solves T1, T2 but NOT T3
//!   - beta solves T2, T3 but NOT T1
//!   - union covers T1+T2+T3 (delta = 1 vs best single model)
//!
//! FOUR RECOMPUTED INGREDIENTS (all integer, no f64):
//!   (a) union_delta  = |solved_by_union| − |solved_by_best_single|
//!   (b) budget_share_per_model = Σ allocated_token_budget per model / Σ total
//!       (expressed as (numerator, denominator) integer fraction — no f64)
//!   (c) token_total = Σ token_counts.total() over all attempts
//!   (d) pput_num/pput_den = total_tokens / max(1, solves)  (integer division
//!       gives the floor; we keep numerator+denominator for exact comparison)
//!
//! Each assertion is against a hand-computed expected value for the fixture.
//!
//! FAILABLE: `tampered_token_count_changes_total_and_pput` bumps one
//! token_count in the fixture and asserts the recomputed token_total and PPUT
//! diverge from the pre-tamper values → the gate catches a lying tape.

use turingosv4::runtime::proposal_telemetry::TokenCounts;
use std::collections::{HashMap, HashSet};

// ── Fixture type ─────────────────────────────────────────────────────────────

/// A single per-attempt record in the fixture tape.
/// No CAS round-trip needed: the gate recomputes directly from these fields.
#[derive(Clone, Debug)]
struct AttemptRecord {
    model_id: &'static str,
    target_id: &'static str,
    allocated_token_budget: u64,
    token_counts: TokenCounts,
    verified: bool,
}

// ── Recompute helpers ─────────────────────────────────────────────────────────

/// (a) union_delta = |targets solved by any model| − |targets solved by the best single model|.
/// "Solved by model M" = ∃ attempt with that model+target where verified==true.
fn union_delta(tape: &[AttemptRecord]) -> i64 {
    // Targets solved by each model.
    let mut per_model: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut union_solved: HashSet<&str> = HashSet::new();
    for r in tape {
        if r.verified {
            per_model.entry(r.model_id).or_default().insert(r.target_id);
            union_solved.insert(r.target_id);
        }
    }
    let best_single = per_model.values().map(|s| s.len()).max().unwrap_or(0);
    union_solved.len() as i64 - best_single as i64
}

/// (b) per-model budget share as (model_id → (allocated_for_model, total_allocated)).
/// The caller checks sum == total and per-model fractions as integers.
fn budget_shares(tape: &[AttemptRecord]) -> HashMap<&str, u64> {
    let mut map: HashMap<&str, u64> = HashMap::new();
    for r in tape {
        *map.entry(r.model_id).or_default() += r.allocated_token_budget;
    }
    map
}

fn total_allocated(tape: &[AttemptRecord]) -> u64 {
    tape.iter().map(|r| r.allocated_token_budget).sum()
}

/// (c) token_total = Σ token_counts.total() over all attempts.
fn token_total(tape: &[AttemptRecord]) -> u64 {
    tape.iter().map(|r| r.token_counts.total()).sum()
}

/// (d) pput numerator and denominator (integer, no f64).
/// pput = total_tokens / max(1, solves);  we return (total_tokens, max(1,solves))
/// so the test can assert BOTH parts without f64 precision issues.
fn pput_parts(tape: &[AttemptRecord]) -> (u64, u64) {
    let total = token_total(tape);
    let solves = tape.iter().filter(|r| r.verified).count() as u64;
    (total, solves.max(1))
}

// ── Fixture ───────────────────────────────────────────────────────────────────

/// Build the canonical 2-model, 3-target fixture tape.
///
/// alpha: solves T1 (verified), T2 (verified), misses T3 (not verified).
/// beta:  solves T2 (verified), T3 (verified), misses T1 (not verified).
///
/// Hand-computed expectations:
///   union targets solved = {T1, T2, T3} → |union| = 3
///   alpha solves {T1,T2} → 2; beta solves {T2,T3} → 2; best_single = 2
///   union_delta = 3 - 2 = 1
///
///   allocated: alpha T1=1000, alpha T2=800, alpha T3=700 → 2500
///              beta  T1=600,  beta T2=750,  beta T3=900  → 2250
///   total_allocated = 4750
///
///   token_counts (prompt+completion+tool):
///     alpha T1: 100+40+10 = 150
///     alpha T2: 120+30+5  = 155
///     alpha T3: 90+35+0   = 125
///     beta  T1: 80+20+0   = 100
///     beta  T2: 110+45+5  = 160
///     beta  T3: 95+50+10  = 155
///   token_total = 150+155+125+100+160+155 = 845
///
///   solves (verified==true): alpha T1, alpha T2, beta T2, beta T3 → 4
///   pput_num = 845, pput_den = 4  → floor = 211 tokens/solve
fn fixture_tape() -> Vec<AttemptRecord> {
    vec![
        // alpha → T1 (solved)
        AttemptRecord {
            model_id: "alpha",
            target_id: "T1",
            allocated_token_budget: 1000,
            token_counts: TokenCounts { prompt_tokens: 100, completion_tokens: 40, tool_tokens: 10 },
            verified: true,
        },
        // alpha → T2 (solved)
        AttemptRecord {
            model_id: "alpha",
            target_id: "T2",
            allocated_token_budget: 800,
            token_counts: TokenCounts { prompt_tokens: 120, completion_tokens: 30, tool_tokens: 5 },
            verified: true,
        },
        // alpha → T3 (not solved)
        AttemptRecord {
            model_id: "alpha",
            target_id: "T3",
            allocated_token_budget: 700,
            token_counts: TokenCounts { prompt_tokens: 90, completion_tokens: 35, tool_tokens: 0 },
            verified: false,
        },
        // beta → T1 (not solved)
        AttemptRecord {
            model_id: "beta",
            target_id: "T1",
            allocated_token_budget: 600,
            token_counts: TokenCounts { prompt_tokens: 80, completion_tokens: 20, tool_tokens: 0 },
            verified: false,
        },
        // beta → T2 (solved)
        AttemptRecord {
            model_id: "beta",
            target_id: "T2",
            allocated_token_budget: 750,
            token_counts: TokenCounts { prompt_tokens: 110, completion_tokens: 45, tool_tokens: 5 },
            verified: true,
        },
        // beta → T3 (solved — the target alpha missed)
        AttemptRecord {
            model_id: "beta",
            target_id: "T3",
            allocated_token_budget: 900,
            token_counts: TokenCounts { prompt_tokens: 95, completion_tokens: 50, tool_tokens: 10 },
            verified: true,
        },
    ]
}

// ── Tests ─────────────────────────────────────────────────────────────────────

/// (a) union_delta = 1: the het set covers one target no single model can cover.
#[test]
fn union_delta_is_one() {
    let tape = fixture_tape();
    let delta = union_delta(&tape);
    assert_eq!(
        delta, 1,
        "union_delta must be 1 (het covers T3 which alpha misses; T1 which beta misses): got {}",
        delta
    );
}

/// (b) per-model budget shares sum to total; alpha share = 2500/4750, beta = 2250/4750.
#[test]
fn budget_shares_are_correct() {
    let tape = fixture_tape();
    let shares = budget_shares(&tape);
    let total = total_allocated(&tape);

    assert_eq!(total, 4750, "total_allocated mismatch: got {total}");
    assert_eq!(
        shares.get("alpha").copied().unwrap_or(0),
        2500,
        "alpha budget share wrong"
    );
    assert_eq!(
        shares.get("beta").copied().unwrap_or(0),
        2250,
        "beta budget share wrong"
    );
    // Sanity: shares sum to total (no budget lost).
    let share_sum: u64 = shares.values().sum();
    assert_eq!(
        share_sum, total,
        "budget shares do not sum to total: {share_sum} != {total}"
    );
}

/// (c) token_total = 845.
#[test]
fn token_total_is_correct() {
    let tape = fixture_tape();
    let total = token_total(&tape);
    assert_eq!(
        total, 845,
        "token_total mismatch: expected 845, got {total}"
    );
}

/// (d) pput_num=845, pput_den=4 (4 verified solves).
#[test]
fn pput_parts_are_correct() {
    let tape = fixture_tape();
    let (num, den) = pput_parts(&tape);
    assert_eq!(num, 845, "pput numerator (total tokens) wrong: got {num}");
    assert_eq!(den, 4, "pput denominator (solve count) wrong: got {den}");
    // Floor division sanity: 845/4 = 211.
    assert_eq!(num / den, 211, "pput floor wrong: expected 211, got {}", num / den);
}

/// FAILABILITY: bumping one record's token_count changes token_total and PPUT.
/// Without this test the gate could be vacuously green on a lying tape.
#[test]
fn tampered_token_count_changes_total_and_pput() {
    let pre_tamper = fixture_tape();
    let (pre_total, pre_den) = pput_parts(&pre_tamper);

    // Tamper: inflate alpha→T1 completion_tokens by 500.
    let mut tampered = pre_tamper.clone();
    tampered[0].token_counts.completion_tokens += 500;

    let (post_total, post_den) = pput_parts(&tampered);

    assert_ne!(
        post_total, pre_total,
        "token_total did NOT change after tampering a token_count — \
         gate cannot catch a lying tape (pre={pre_total}, post={post_total})"
    );
    // PPUT numerator must also diverge (denominator unchanged — no new solve).
    assert_ne!(
        post_total / post_den,
        pre_total / pre_den,
        "PPUT floor did NOT change after tampering — \
         gate cannot catch inflation in the per-solved-unit cost \
         (pre={}, post={})",
        pre_total / pre_den,
        post_total / post_den
    );
    // Denominator (solve count) is unaffected by a token-count bump.
    assert_eq!(
        post_den, pre_den,
        "solve count should not change from a token_count tamper"
    );
}

/// Sanity: verified solve count in the fixture matches expectation (4 solves).
/// This guards the fixture itself against silently wrong setup.
#[test]
fn fixture_has_four_solves() {
    let tape = fixture_tape();
    let solves = tape.iter().filter(|r| r.verified).count();
    assert_eq!(solves, 4, "fixture must have exactly 4 verified solves; got {solves}");
}
