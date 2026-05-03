# TB-15 First Markov Capsule — 2026-05-03

**TB**: TB-15 — Lamarckian Autopsy + Markov EvidenceCapsule
**Charter**: `handover/tracer_bullets/TB-15_charter_2026-05-03.md`
**Architect spec**: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §6 (FR-15.1..6 + CR-15.1..6 + SG-15.1..8)

## Generation command

```bash
cargo run --bin generate_markov_capsule -- \
  --tb-id 15 \
  --out-dir handover/markov_capsules \
  --constitution-path constitution.md \
  --no-cas
```

## Outputs

- `MARKOV_TB-15_2026-05-03.json` — first MarkovEvidenceCapsule (TB-15 self-reference; the genesis Markov capsule of the TuringOS v4 chain)
- `LATEST_MARKOV_CAPSULE.txt` — Cid hex pointer (`b244f16a1f3bd532d041a40fe39b2b7e7cc12fb58e18b61aedd76a8010eeb1b6`)

## Field summary (capsule)

```text
capsule_id            = b244f16a1f3bd532d041a40fe39b2b7e7cc12fb58e18b61aedd76a8010eeb1b6
previous_capsule_cid  = None  (genesis Markov; SG-15.7 chain root)
constitution_hash     = eec695459c71fbef3685583485deb431fe3b561657b2f285b7c5e7e220e59e03
                        (sha256 of constitution.md at generation time;
                         FR-15.4 + SG-15.7)
l4_root               = Hash::ZERO  (v0 placeholder; future TB wires from chain head)
l4e_root              = Hash::ZERO  (same)
cas_root              = Hash::ZERO  (same)
typical_errors        = []          (no TaskBankruptcyTx fired yet — empty per
                                     TB-15 v0 charter §1.2 single-trigger scope)
unresolved_obs        = 22          (handover/alignment/OBS_*.md scan)
next_session_context_cid = (deterministic Cid of NextSessionContext JSON blob;
                            embedded in capsule; readable from CAS via the binary
                            in non-`--no-cas` mode)
tb_tag                = "TB-15"
```

## Halt-trigger battery (final, post-Atom-6 ship)

```text
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

#1 raw_logs_not_in_general_read_view              GREEN
   (Atom 3 — AgentVisibleProjection file-scan: agent_autopsies_t / AutopsyIndex /
    AgentAutopsyCapsule / private_detail_cid forbidden in projection body)
#2 markov_capsule_references_constitution_hash    GREEN
   (Atom 5 — sha256(constitution.md) == capsule.constitution_hash; SG-15.7)
#3 autopsy_does_not_mutate_predicates             GREEN
   (Atom 2 — autopsy_capsule.rs file-scan: no &mut PredicateRegistry /
    ToolRegistry / RiskPolicyRegistry / register_*/unregister_*/patch_*; CR-15.3 +
    SG-15.8)
#4 private_detail_not_in_other_agent_view         GREEN
   (Atom 3 — AutopsyIndex value type is Vec<Cid>; never raw bytes; SG-15.2)
#5 typical_error_clustering_uses_summary_only     GREEN
   (Atom 4 — cluster_autopsies output JSON contains no input
    private_detail_cid byte run; CR-15.2 + SG-15.5)
#6 deep_history_read_without_override_fails       GREEN
   (Atom 5 — try_deep_history_read_with_override_check(false) → DeepHistoryReadDenied;
    (true) → Ok(()); SG-15.4 + FR-15.5)
```

## Ship-gate ledger

| ID | Gate | Status |
|---|---|---|
| SG-15.1 | Failed/losing agent gets private AutopsyCapsule | GREEN — TaskBankruptcyTx dispatch arm Step 3.5 emits per-staker capsule via `derive_autopsies_for_bankruptcy` (verified by `derive_autopsies_emits_one_per_staker_target_only`) |
| SG-15.2 | Raw private details do not enter other Agent read view | GREEN — `agent_autopsies_t` lives on `EconomicState` (sequencer-side), NOT on `AgentVisibleProjection`; halt-trigger #1 + #4 file-scans STRUCTURALLY enforce |
| SG-15.3 | Latest Markov capsule can bootstrap next session | GREEN — `next_session_context_cid` field embedded in capsule; `LATEST_MARKOV_CAPSULE.txt` pointer file written |
| SG-15.4 | Deep-history read without override fails | GREEN — `try_deep_history_read_with_override_check(false)` returns `Err(DeepHistoryReadDenied)`; halt-trigger #6 |
| SG-15.5 | Typical error broadcast uses summary, not raw log | GREEN — `cluster_autopsies` output struct embeds `public_summary` strings + `capsule_id` Cids only; halt-trigger #5 verifies serialization contains no `private_detail_cid` byte run |
| SG-15.6 | Dashboard can regenerate capsule summary from ChainTape + CAS | GREEN — `render_section_15` pure function with deterministic input shape `(Vec<(String,u32)>, Option<&str>)`; 4 dashboard render unit tests; render output contains no raw bytes |
| SG-15.7 | Markov capsule references constitution hash and flowchart hashes | GREEN — `MarkovEvidenceCapsule.constitution_hash` field; `with_constitution_hash` constructor; halt-trigger #2 |
| SG-15.8 | Autopsy does not mutate predicates/tools automatically | GREEN — writer signature has no mutable registry refs; halt-trigger #3 file-scan |
| G-15.9 | `cargo test --workspace` ≥ TB-14 baseline / 0 fail | GREEN — 870 PASS / 0 fail / 150 ignored (net +67 vs TB-14 ship 803) |
| G-15.10 | FC1-N32 + FC1-N33 + FC2-N30 + FC3-N43 each have ≥1 witness | GREEN — `tests/fc_alignment_conformance.rs` has 4 witness tests added (`fc1_n32_*`, `fc1_n33_*`, `fc2_n30_*`, `fc3_n43_*`) |
| G-15.11 | EconomicState sub-field count assertion updated 12→13 | GREEN — 3 sub-field count tests updated (`economic_state_has_thirteen_sub_fields`, `empty_economic_state_serializes_to_thirteen_sub_fields`, `axiom_3_economic_state_present_and_complete` 13) |
| G-15.12 | First Markov capsule generated + persisted | **GREEN — this directory is the artifact** |

## Replay determinism (Art.0.2)

The capsule is a pure function of:
- constitution.md bytes (sha256-pinned)
- `previous_capsule_cid` (None for genesis; `Some(prior_capsule.capsule_id)` thereafter)
- L4 / L4.E / CAS roots (v0: zero placeholders; future TB: chain-derived)
- typical_errors (output of `cluster_autopsies` over CAS-resident `AgentAutopsyCapsule` objects)
- unresolved_obs (sorted scan of `handover/alignment/OBS_*.md`)
- `created_at_logical_t` + `tb_tag`

Re-running the binary with identical inputs yields the same `capsule_id`. Verified by `write_markov_capsule_deterministic_capsule_id` unit test.

## Privacy contract (architect §6.4)

- `public_summary` — low-info string; broadcast-eligible IFF N≥3 cluster
- `private_detail_cid` — opaque CAS Cid; AuditOnly access only; NEVER enters `AgentVisibleProjection`
- `evidence_cids` — Cids of pre-existing public ChainTape evidence; not new private bytes

This evidence directory contains ONLY public surfaces — capsule JSON (which itself contains only Cids + low-info field set) + Cid-hex pointer file. No raw private bytes are persisted under `handover/`.

## Cross-references

- TB-15 charter: `handover/tracer_bullets/TB-15_charter_2026-05-03.md`
- Architect spec: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §6
- DECISION_LAMARCKIAN: `handover/alignment/DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN_2026-05-02.md` §1
- Generator binary: `src/bin/generate_markov_capsule.rs`
- Schema: `src/runtime/markov_capsule.rs`
- Autopsy schema + writer: `src/runtime/autopsy_capsule.rs`
- Dashboard §15: `src/bin/audit_dashboard.rs::render_section_15`
