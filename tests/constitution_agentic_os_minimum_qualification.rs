//! LIVE CONSTITUTION META-GATE — Agentic-OS minimum-qualification acceptance suite.
//!
//! STATUS: LIVE / GREEN. Class 2 (read-only, SOURCE-STRUCTURAL). This gate makes
//! the OS-qualifying CONJUNCTION explicit and falsifiable: it asserts that EVERY
//! OS-qualifying constitution gate is BOTH (a) present as a flat test file in
//! `tests/<gate>.rs` AND (b) registered in
//! `scripts/constitution_gates.manifest.toml`. Removing, renaming, or disabling
//! any single OS-qualifying gate turns THIS meta-gate RED — so the operational
//! "is the substrate still minimally OS-qualified?" signal can never silently
//! degrade one leg at a time.
//!
//! ── WHAT THIS META-GATE ENFORCES (the OS-qualifying conjunction) ───────────
//! Per the research-report §5 operational definition of an Agentic-OS minimum
//! qualification, the substrate is minimally OS-qualified only when ALL of the
//! following hold SIMULTANEOUSLY (a conjunction, not a disjunction):
//!
//!   * **Tape canonicality / single source of truth** — meaningful activity is
//!     on the canonical ChainTape, not a shadow/dashboard ledger.
//!       constitution_tape_canonical_gate
//!   * **Predicate-gated irreversible advance** — no verified-head / state-root
//!     advance happens without a re-executed predicate-admission verdict
//!     (single shared admission contract; M07 route A; G1/G2/G3).
//!       constitution_kernel_predicate_gate
//!       constitution_single_admission_contract
//!       constitution_single_admission_behavioral
//!       constitution_kernel_predicate_receipt_replay
//!       constitution_predicate_zero_root_is_not_oracle
//!   * **Trust-root anchoring** — canonical writers / boot verify the trust root
//!     (tamper of constitution / pinned manifest is detectable).
//!       constitution_all_canonical_writers_verify_trust_root
//!       constitution_tc_boot_trust_root_manifest
//!   * **Tape-reconstructable attempt accounting (FC1)** — every externalized
//!     LLM/Lean cycle lands on tape, including failure arms, so the FC1 invariant
//!     `completed_llm_calls = step + parse_fail + llm_err` is reconstructable.
//!       constitution_llm_err_lands_on_tape
//!       constitution_external_attempt_anchored_on_failure
//!   * **Read-view shielding (Art. III)** — agent-visible read views are
//!     scoped/shielded; no raw subprocess stderr or metric/Goodhart leak reaches
//!     the agent prompt.
//!       constitution_judge_reason_no_raw_subprocess_stderr
//!       constitution_metric_leak_guard_wired
//!       constitution_shielding_gate
//!   * **Money conservation (Art. 0 / integer-only economy)** — no mint/burn
//!     outside on_init; strict integer conservation.
//!       constitution_economy_gate
//!       constitution_economy_strict_equality
//!   * **Tape-canonical efficiency (architect North Star — held-out Verified
//!     PPUT)** — the architect PPUT (`VPPUT_i = 1[GroundTruth] / (C_i × T_i)`) is
//!     reconstructed FROM THE CANONICAL TAPE (L4 + L4.E + CAS): integer-only
//!     micro-units, ground-truth gated (Art. I.1), C_i counting all failed
//!     branches + tool stdout, and a held-out H-VPPUT aggregate — so efficiency is
//!     a gate-verifiable OS-qualifying dimension, not a sidecar dashboard number.
//!       constitution_vpput_reconstructed_from_tape
//!   * **Anti-Goodhart PPUT guardrails (Art. III.4 / Gate H + accounting
//!     integrity)** — the 11 named conformance gates that make held-out Verified
//!     PPUT NON-GAMEABLE: all model tokens counted, tool stdout hashed on tape,
//!     no hidden unmetered generation, no hardcoded problem id, failed branches
//!     count, ground-truth-gated golden path, correct T_i span, held-out ids
//!     inaccessible, no PPUT/metric in any agent prompt.
//!       constitution_pput_anti_goodhart_battery
//!   * **Clean-context closure witness (Art. V / no-zombie)** — the no-zombie /
//!     final-closure witness binds the derived views to fresh tape evidence and
//!     refuses unscoped global completion claims.
//!       constitution_obl005_final_closure_witness
//!
//! The OPERATIONAL minimum qualification = the CONJUNCTION of all the gates in
//! `OS_QUALIFYING_GATES` below. This meta-gate does NOT re-run their bodies (each
//! is its own `--test` target in `run_constitution_gates.sh`); it asserts the
//! conjunction's MEMBERS still EXIST and are WIRED, so the conjunction cannot be
//! quietly shrunk. The honest operational-definition prose and the explicit
//! STILL-PENDING dimensions (G4 budget ceiling, G5 FC3 self-evolution engine,
//! cross-session memory, OS-level sandbox, multi-LLM market proof, interop) live
//! in `handover/reports/AGENTIC_OS_MINIMUM_QUALIFICATION_PACKET_2026-06-07.md`.
//!
//! ── NON-VACUITY ───────────────────────────────────────────────────────────
//! The meta-gate fails LOUD if the qualifying set is empty (it is a compile-time
//! constant, asserted non-empty at runtime) or if ANY listed name is missing
//! from the live tests/ tree or from the manifest. It cannot be satisfied by a
//! vacuous `assert!(true)`: every assertion reads the live filesystem / manifest.
//!
//! ── TRIPLE-COUPLING ───────────────────────────────────────────────────────
//! Registered in `scripts/constitution_gates.manifest.toml`
//! (`constitution_agentic_os_minimum_qualification`) and referenced in
//! `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`. Discovered by the flat
//! `ls tests/constitution_*.rs` glob in `scripts/run_constitution_gates.sh`.
//!
//! ── RECORDED MUTANT (Art. I.1.1 applied to gates) ─────────────────────────
//! Deleting `tests/constitution_economy_gate.rs` (or removing its manifest
//! `[[gate]]` entry) drops that leg of the conjunction. Before: this meta-gate
//! GREEN. After: `every_os_qualifying_gate_has_a_test_file` (resp.
//! `every_os_qualifying_gate_is_registered_in_the_manifest`) fails RED naming
//! `constitution_economy_gate`. That proves the meta-gate tracks the live set
//! and is not satisfiable vacuously.

use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// The OS-qualifying CONJUNCTION. The operational Agentic-OS minimum
/// qualification = ALL of these gates GREEN simultaneously. This list is the
/// load-bearing definition of the conjunction; shrinking it (or removing a
/// member's test/manifest entry) is exactly what this meta-gate forbids.
///
/// All members are currently GREEN on main. The PENDING qualification
/// dimensions (G4 budget ceiling, G5 FC3 self-evolution engine, cross-session
/// memory, OS-level sandbox, multi-LLM market proof, interop) are deliberately
/// NOT in this list — see the qualification packet — because they are not yet
/// enforced by a GREEN gate, and claiming them here would make the conjunction
/// dishonest.
const OS_QUALIFYING_GATES: &[&str] = &[
    // Tape canonicality / single source of truth.
    "constitution_tape_canonical_gate",
    // Predicate-gated irreversible advance (M07 route A; G1/G2/G3).
    "constitution_kernel_predicate_gate",
    "constitution_single_admission_contract",
    "constitution_single_admission_behavioral",
    "constitution_kernel_predicate_receipt_replay",
    "constitution_predicate_zero_root_is_not_oracle",
    // Trust-root anchoring.
    "constitution_all_canonical_writers_verify_trust_root",
    "constitution_tc_boot_trust_root_manifest",
    // Tape-reconstructable attempt accounting (FC1 conformance sweep #1/#2).
    "constitution_llm_err_lands_on_tape",
    "constitution_external_attempt_anchored_on_failure",
    // Read-view shielding (Art. III; conformance sweep #4/#5 + base shield gate).
    "constitution_judge_reason_no_raw_subprocess_stderr",
    "constitution_metric_leak_guard_wired",
    "constitution_shielding_gate",
    // Money conservation (Art. 0 / integer-only economy).
    "constitution_economy_gate",
    "constitution_economy_strict_equality",
    // Tape-canonical efficiency (architect North Star: held-out Verified PPUT
    // reconstructed from L4 + L4.E + CAS; integer-only, ground-truth gated).
    "constitution_vpput_reconstructed_from_tape",
    // Anti-Goodhart guardrails — the 11 named conformance gates that make
    // held-out Verified PPUT NON-GAMEABLE (Art. III.4 / Gate H + accounting
    // integrity; PPUT_DRIVEN_FULL_PASS_2026-04-25 §10).
    "constitution_pput_anti_goodhart_battery",
    // Clean-context closure witness (Art. V / no-zombie final closure).
    "constitution_obl005_final_closure_witness",
];

fn manifest_gate_names() -> HashSet<String> {
    let manifest = fs::read_to_string("scripts/constitution_gates.manifest.toml")
        .expect("scripts/constitution_gates.manifest.toml must exist (K-1.5 ship)");
    manifest
        .lines()
        .filter_map(|l| {
            l.trim()
                .strip_prefix("name = \"")?
                .strip_suffix("\"")
                .map(str::to_string)
        })
        .collect()
}

/// Vacuity guard: the qualifying conjunction must be non-empty AND free of
/// duplicates. An empty set would make every "for-all" assertion below trivially
/// pass — that is exactly the M07 single-site illusion this gate is designed to
/// prevent. A duplicate would silently weaken the count.
#[test]
fn os_qualifying_set_is_non_empty_and_unique() {
    assert!(
        !OS_QUALIFYING_GATES.is_empty(),
        "Agentic-OS minimum-qualification violation: OS_QUALIFYING_GATES is \
         empty. The OS-qualifying conjunction must enumerate at least the \
         currently-GREEN qualifying gate set; an empty set makes the meta-gate \
         vacuous (every for-all assertion would pass trivially)."
    );

    let unique: HashSet<&&str> = OS_QUALIFYING_GATES.iter().collect();
    assert_eq!(
        unique.len(),
        OS_QUALIFYING_GATES.len(),
        "Agentic-OS minimum-qualification violation: OS_QUALIFYING_GATES \
         contains a duplicate gate name. Each leg of the conjunction must be \
         listed exactly once."
    );
}

/// Every OS-qualifying gate must have a flat test file `tests/<gate>.rs`. This
/// is the file the `ls tests/constitution_*.rs` glob in
/// `scripts/run_constitution_gates.sh` discovers and runs. Deleting any member's
/// test file (disabling that leg of the conjunction) turns this RED.
#[test]
fn every_os_qualifying_gate_has_a_test_file() {
    let mut missing: Vec<&str> = Vec::new();
    for gate in OS_QUALIFYING_GATES {
        let path = format!("tests/{gate}.rs");
        if !Path::new(&path).is_file() {
            missing.push(gate);
        }
    }
    assert!(
        missing.is_empty(),
        "Agentic-OS minimum-qualification violation: {} OS-qualifying gate(s) \
         have NO test file under tests/ — the OS-qualifying conjunction has been \
         shrunk (a load-bearing qualification leg was removed/disabled):\n\n  \
         {:?}\n\nEvery name in OS_QUALIFYING_GATES must resolve to \
         tests/<name>.rs. Restore the gate file or, if the qualification \
         definition genuinely changed, update OS_QUALIFYING_GATES + the \
         qualification packet under explicit authority.",
        missing.len(),
        missing
    );
}

/// Every OS-qualifying gate must be registered in the manifest. The runner
/// cross-checks manifest vs discovered files; a gate dropped from the manifest
/// is no longer authorized to run as a constitutional gate. Removing any
/// member's `[[gate]]` entry (disabling that leg of the conjunction) turns this
/// RED.
#[test]
fn every_os_qualifying_gate_is_registered_in_the_manifest() {
    let registered = manifest_gate_names();
    // Sanity: the manifest itself must be non-trivial (defends against reading
    // an empty/garbled manifest and passing vacuously).
    assert!(
        registered.len() >= OS_QUALIFYING_GATES.len(),
        "Agentic-OS minimum-qualification violation: the manifest parsed only {} \
         gate name(s), fewer than the {} OS-qualifying gates. The manifest is \
         likely empty or malformed — failing LOUD rather than passing vacuously.",
        registered.len(),
        OS_QUALIFYING_GATES.len()
    );

    let mut unregistered: Vec<&str> = Vec::new();
    for gate in OS_QUALIFYING_GATES {
        if !registered.contains(*gate) {
            unregistered.push(gate);
        }
    }
    assert!(
        unregistered.is_empty(),
        "Agentic-OS minimum-qualification violation: {} OS-qualifying gate(s) \
         are NOT registered in scripts/constitution_gates.manifest.toml — the \
         OS-qualifying conjunction has been shrunk (a qualification leg is no \
         longer an authorized constitutional gate):\n\n  {:?}\n\nEvery name in \
         OS_QUALIFYING_GATES must have a `[[gate]] name = \"<name>\"` entry. \
         Restore the manifest entry, or update the qualification definition \
         under explicit authority.",
        unregistered.len(),
        unregistered
    );
}

/// Cross-leg coherence: the conjunction must cover EACH qualification dimension
/// (not all gates of one kind). If a whole dimension's gates were deleted, the
/// two assertions above would catch the names, but this assertion makes the
/// dimension structure explicit and fails with the dimension named, so a future
/// edit that removes (say) the entire money-conservation pair is reported as
/// "money-conservation dimension uncovered", not just two opaque names.
#[test]
fn every_qualification_dimension_is_covered() {
    // (dimension label, at-least-one-of these gate names must be present)
    const DIMENSIONS: &[(&str, &[&str])] = &[
        ("tape-canonicality", &["constitution_tape_canonical_gate"]),
        (
            "predicate-gated-advance",
            &[
                "constitution_kernel_predicate_gate",
                "constitution_single_admission_contract",
                "constitution_single_admission_behavioral",
                "constitution_kernel_predicate_receipt_replay",
                "constitution_predicate_zero_root_is_not_oracle",
            ],
        ),
        (
            "trust-root-anchoring",
            &[
                "constitution_all_canonical_writers_verify_trust_root",
                "constitution_tc_boot_trust_root_manifest",
            ],
        ),
        (
            "fc1-tape-reconstructable-attempt-accounting",
            &[
                "constitution_llm_err_lands_on_tape",
                "constitution_external_attempt_anchored_on_failure",
            ],
        ),
        (
            "read-view-shield",
            &[
                "constitution_judge_reason_no_raw_subprocess_stderr",
                "constitution_metric_leak_guard_wired",
                "constitution_shielding_gate",
            ],
        ),
        (
            "money-conservation",
            &[
                "constitution_economy_gate",
                "constitution_economy_strict_equality",
            ],
        ),
        (
            "tape-canonical-efficiency",
            &["constitution_vpput_reconstructed_from_tape"],
        ),
        (
            "anti-goodhart-pput-guardrails",
            &["constitution_pput_anti_goodhart_battery"],
        ),
        (
            "clean-context-closure-witness",
            &["constitution_obl005_final_closure_witness"],
        ),
    ];

    let listed: HashSet<&str> = OS_QUALIFYING_GATES.iter().copied().collect();

    let mut uncovered: Vec<&str> = Vec::new();
    for (dimension, members) in DIMENSIONS {
        let covered = members.iter().any(|m| listed.contains(m));
        if !covered {
            uncovered.push(dimension);
        }
        // Every dimension member named here must itself be a real listed gate;
        // a typo in the dimension map would otherwise hide a regression.
        for m in *members {
            assert!(
                listed.contains(m),
                "Agentic-OS minimum-qualification violation: dimension \
                 '{dimension}' references '{m}', which is not in \
                 OS_QUALIFYING_GATES. The dimension map and the conjunction list \
                 must stay in sync."
            );
        }
    }
    assert!(
        uncovered.is_empty(),
        "Agentic-OS minimum-qualification violation: {} qualification \
         dimension(s) are no longer covered by ANY gate in the conjunction:\n\n  \
         {:?}\n\nEach operational qualification dimension must retain at least \
         one GREEN gate. Restore a covering gate or revise the qualification \
         definition under explicit authority.",
        uncovered.len(),
        uncovered
    );
}
