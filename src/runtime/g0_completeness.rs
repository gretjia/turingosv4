use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct G0Manifest {
    pub version: String,
    pub tactic_atoms: Vec<String>,
    pub lemma_atoms: Vec<String>,
    pub productions_hash: String,
    pub manifest_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Candidate {
    pub ast_canonical: String,
    pub digest: String,
    pub rank: u64,
    pub lean_text: String,
    pub market_price: Option<u64>,
    pub autonomous_route: Option<String>,
    pub claimed_verifier_acceptance: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Lane {
    EvenEnumerator,
    OddHeuristic,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchedulerTrace {
    pub tick: u64,
    pub lane: Lane,
    pub candidate_digest: Option<String>,
    pub rank: Option<u64>,
    pub action: String,
    pub verdict: String,
    pub duplicate_of_tick: Option<u64>,
    pub market_price: Option<u64>,
    pub autonomous_route: Option<String>,
    pub verifier_acceptance: Option<bool>,
}

const BLOCKED_ATOMS: &[&str] = &[
    "native_decide",
    "decide",
    "omega",
    "sorry",
    "admit",
    "aesop",
    "simp_all",
];

const LOCKED_G0_TACTIC_ATOMS: &[&str] =
    &["apply h", "assumption", "exact h", "intro", "rfl", "simp"];
const LOCKED_G0_LEMMA_ATOMS: &[&str] = &["lemma_a", "lemma_b"];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
enum G0Ast {
    Tactic { atom: String },
    ExactLemma { lemma: String },
}

impl G0Ast {
    fn parse_tactic(atom: &str) -> Result<Self, String> {
        validate_atom(atom)?;
        if !LOCKED_G0_TACTIC_ATOMS.contains(&atom) {
            return Err(format!("tactic atom is outside locked G0: {atom}"));
        }
        Ok(Self::Tactic {
            atom: atom.to_string(),
        })
    }

    fn parse_lemma(lemma: &str) -> Result<Self, String> {
        validate_atom(lemma)?;
        if !LOCKED_G0_LEMMA_ATOMS.contains(&lemma) {
            return Err(format!("lemma atom is outside locked G0: {lemma}"));
        }
        Ok(Self::ExactLemma {
            lemma: lemma.to_string(),
        })
    }

    fn parse_lean_text(text: &str) -> Result<Self, String> {
        if let Some(lemma) = text.strip_prefix("exact ") {
            if LOCKED_G0_LEMMA_ATOMS.contains(&lemma) {
                return Self::parse_lemma(lemma);
            }
        }
        Self::parse_tactic(text)
    }

    fn canonical(&self) -> String {
        match self {
            Self::Tactic { atom } => format!("g0:tactic:{atom}"),
            Self::ExactLemma { lemma } => format!("g0:exact_lemma:{lemma}"),
        }
    }

    fn rank_tuple(&self) -> (u64, u64, String) {
        let ast_size = match self {
            Self::Tactic { .. } => 1,
            Self::ExactLemma { .. } => 2,
        };
        let canonical = self.canonical();
        (ast_size, canonical.len() as u64, digest_hex(&canonical))
    }

    fn lean_text(&self) -> String {
        match self {
            Self::Tactic { atom } => atom.clone(),
            Self::ExactLemma { lemma } => format!("exact {lemma}"),
        }
    }
}

impl G0Manifest {
    pub fn new(
        version: impl Into<String>,
        tactic_atoms: Vec<String>,
        lemma_atoms: Vec<String>,
    ) -> Result<Self, String> {
        validate_atom_list("tactic", &tactic_atoms)?;
        validate_atom_list("lemma", &lemma_atoms)?;
        for atom in &tactic_atoms {
            G0Ast::parse_tactic(atom)?;
        }
        for lemma in &lemma_atoms {
            G0Ast::parse_lemma(lemma)?;
        }
        let version = version.into();
        let productions_hash = digest_hex(&format!(
            "g0-productions|{}|{}",
            tactic_atoms.join("\n"),
            lemma_atoms.join("\n")
        ));
        let manifest_hash = digest_hex(&format!(
            "g0-manifest|{version}|{productions_hash}|{}|{}",
            tactic_atoms.join("\n"),
            lemma_atoms.join("\n")
        ));
        Ok(Self {
            version,
            tactic_atoms,
            lemma_atoms,
            productions_hash,
            manifest_hash,
        })
    }
}

impl Candidate {
    pub fn from_g0_text(text: impl AsRef<str>) -> Self {
        candidate_for_text(text.as_ref())
    }

    pub fn heuristic(text: impl Into<String>) -> Self {
        let lean_text = text.into();
        let ast_canonical = format!("heuristic:{lean_text}");
        let digest = digest_hex(&ast_canonical);
        Self {
            ast_canonical,
            digest,
            rank: u64::MAX,
            lean_text,
            market_price: None,
            autonomous_route: None,
            claimed_verifier_acceptance: None,
        }
    }

    pub fn heuristic_with_digest(text: impl Into<String>, digest: String) -> Self {
        let mut candidate = Self::heuristic(text);
        candidate.digest = digest;
        candidate
    }

    pub fn heuristic_with_market(text: impl Into<String>, market_price: u64) -> Self {
        let mut candidate = Self::heuristic(text);
        candidate.market_price = Some(market_price);
        candidate
    }

    pub fn heuristic_with_market_and_claimed_acceptance(
        text: impl Into<String>,
        market_price: u64,
        claimed_acceptance: bool,
    ) -> Self {
        let mut candidate = Self::heuristic_with_market(text, market_price);
        candidate.claimed_verifier_acceptance = Some(claimed_acceptance);
        candidate
    }

    pub fn heuristic_with_route(text: impl Into<String>, route: impl Into<String>) -> Self {
        let mut candidate = Self::heuristic(text);
        candidate.autonomous_route = Some(route.into());
        candidate
    }

    pub fn heuristic_with_route_and_claimed_acceptance(
        text: impl Into<String>,
        route: impl Into<String>,
        claimed_acceptance: bool,
    ) -> Self {
        let mut candidate = Self::heuristic_with_route(text, route);
        candidate.claimed_verifier_acceptance = Some(claimed_acceptance);
        candidate
    }
}

pub fn enumerate_candidates(manifest: &G0Manifest, max_rank: u64) -> Vec<Candidate> {
    let mut out = Vec::new();
    for tactic in &manifest.tactic_atoms {
        out.push(candidate_from_ast(
            G0Ast::parse_tactic(tactic).expect("manifest validates tactic atoms"),
        ));
    }
    for lemma in &manifest.lemma_atoms {
        out.push(candidate_from_ast(
            G0Ast::parse_lemma(lemma).expect("manifest validates lemma atoms"),
        ));
    }
    out.retain(|c| c.rank <= max_rank);
    out.sort_by(|a, b| (a.rank, a.digest.as_str()).cmp(&(b.rank, b.digest.as_str())));
    out
}

pub fn run_dovetail(
    even_candidates: Vec<Candidate>,
    odd_candidates: Vec<Candidate>,
    ticks: u64,
) -> Vec<SchedulerTrace> {
    let mut even = VecDeque::from(even_candidates);
    let mut odd = VecDeque::from(odd_candidates);
    let mut first_even_attempt = BTreeMap::<String, u64>::new();
    let mut trace = Vec::new();

    for tick in 0..ticks {
        if tick % 2 == 0 {
            match even.pop_front() {
                Some(c) => {
                    let duplicate_of_tick = first_even_attempt.get(&c.digest).copied();
                    if duplicate_of_tick.is_none() {
                        first_even_attempt.insert(c.digest.clone(), tick);
                    }
                    trace.push(SchedulerTrace {
                        tick,
                        lane: Lane::EvenEnumerator,
                        candidate_digest: Some(c.digest),
                        rank: Some(c.rank),
                        action: if duplicate_of_tick.is_some() {
                            "covered_by_prior_even_attempt".to_string()
                        } else {
                            "attempt".to_string()
                        },
                        verdict: if duplicate_of_tick.is_some() {
                            "duplicate".to_string()
                        } else {
                            "pending_verifier".to_string()
                        },
                        duplicate_of_tick,
                        market_price: None,
                        autonomous_route: None,
                        verifier_acceptance: None,
                    })
                }
                None => trace.push(SchedulerTrace {
                    tick,
                    lane: Lane::EvenEnumerator,
                    candidate_digest: None,
                    rank: None,
                    action: "enum_exhausted".to_string(),
                    verdict: "no_candidate".to_string(),
                    duplicate_of_tick: None,
                    market_price: None,
                    autonomous_route: None,
                    verifier_acceptance: None,
                }),
            }
        } else {
            match odd.pop_front() {
                Some(c) => trace.push(SchedulerTrace {
                    tick,
                    lane: Lane::OddHeuristic,
                    candidate_digest: Some(c.digest),
                    rank: None,
                    action: "heuristic_observed".to_string(),
                    verdict: "not_enumerator_authority".to_string(),
                    duplicate_of_tick: None,
                    market_price: c.market_price,
                    autonomous_route: c.autonomous_route,
                    verifier_acceptance: None,
                }),
                None => trace.push(SchedulerTrace {
                    tick,
                    lane: Lane::OddHeuristic,
                    candidate_digest: None,
                    rank: None,
                    action: "odd_queue_empty".to_string(),
                    verdict: "no_candidate".to_string(),
                    duplicate_of_tick: None,
                    market_price: None,
                    autonomous_route: None,
                    verifier_acceptance: None,
                }),
            }
        }
    }

    trace
}

pub fn prioritize_odd_by_market(mut candidates: Vec<Candidate>) -> Vec<Candidate> {
    candidates.sort_by(|a, b| {
        b.market_price
            .unwrap_or(0)
            .cmp(&a.market_price.unwrap_or(0))
            .then_with(|| a.digest.cmp(&b.digest))
            .then_with(|| a.lean_text.cmp(&b.lean_text))
    });
    candidates
}

fn candidate_for_text(lean_text: &str) -> Candidate {
    candidate_from_ast(G0Ast::parse_lean_text(lean_text).expect("text must be a locked G0 atom"))
}

fn candidate_from_ast(ast: G0Ast) -> Candidate {
    let ast_canonical = ast.canonical();
    let (ast_size, serialized_len, digest) = ast.rank_tuple();
    Candidate {
        rank: ast_size * 1_000_000 + serialized_len,
        ast_canonical,
        digest,
        lean_text: ast.lean_text(),
        market_price: None,
        autonomous_route: None,
        claimed_verifier_acceptance: None,
    }
}

fn validate_atom_list(kind: &str, atoms: &[String]) -> Result<(), String> {
    let mut seen = BTreeSet::new();
    for atom in atoms {
        if atom.trim().is_empty() {
            return Err(format!("{kind} atom must not be empty"));
        }
        if atom != atom.trim() {
            return Err(format!(
                "{kind} atom has non-canonical whitespace: {atom:?}"
            ));
        }
        if !seen.insert(atom.as_str()) {
            return Err(format!("duplicate {kind} atom: {atom}"));
        }
    }
    Ok(())
}

fn validate_atom(atom: &str) -> Result<(), String> {
    if atom.starts_with("raw:") {
        return Err(format!("raw tactic atom is outside G0: {atom}"));
    }
    for blocked in BLOCKED_ATOMS {
        if contains_blocked_atom(atom, blocked) {
            return Err(format!("blocked tactic atom `{blocked}` in {atom}"));
        }
    }
    Ok(())
}

fn contains_blocked_atom(atom: &str, blocked: &str) -> bool {
    atom.split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
        .any(|token| token == blocked)
}

fn digest_hex(s: &str) -> String {
    let mut h = Sha256::new();
    h.update(s.as_bytes());
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}
