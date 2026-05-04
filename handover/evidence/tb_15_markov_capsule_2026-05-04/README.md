# TB-15 R3 Markov Capsule (post recursive dual-audit closure) — 2026-05-04

**Replaces**: `handover/evidence/tb_15_markov_capsule_2026-05-03/` (Atom 6 ship; `--no-cas`, no flowchart_hashes) AND R2 capsule (CAS-cid mismatch — Codex R2 VETO Q3 + TB15-CAS-ID).

**TB**: TB-15 — Lamarckian Autopsy + Markov EvidenceCapsule
**Round**: R3 closure (post Codex + Gemini R2 dual audit)
**Closure doc**: `handover/audits/RECURSIVE_AUDIT_TB_15_2026-05-04.md`

## Generation command

```bash
mkdir -p /tmp/tb15-r3-cas
cargo run --bin generate_markov_capsule -- \
  --tb-id 15-R3 \
  --out-dir handover/markov_capsules \
  --constitution-path constitution.md \
  --cas-dir /tmp/tb15-r3-cas
```

## Outputs

- `MARKOV_TB-15-R3_2026-05-03.json` — R3 MarkovEvidenceCapsule (CAS-resolvable; flowchart_hashes populated)
- `LATEST_MARKOV_CAPSULE.txt` — Cid hex pointer (`f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312`)
- `cas_index.jsonl` — copy of CAS index showing CAS object cid matches LATEST pointer

## R3 deltas vs R2 capsule

```text
capsule_id                : R2 a94ae884... → R3 f9e701b4...   (rebuilt with R3 fixed writer)
constitution_hash         : eec69545...                         (unchanged)
flowchart_hashes          : 4 hashes from TRACE_FLOWCHART_MATRIX.md (unchanged)
unresolved_obs            : R2 22 → R3 23                       (added OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04.md)
typical_errors            : []                                  (unchanged)
CAS resolvability         : R2 BROKEN (cid in CAS index ≠ capsule_id) → R3 FIXED
```

## CAS-resolvability proof (R3 closure of Codex R2 VETO Q3 + TB15-CAS-ID)

```text
LATEST_MARKOV_CAPSULE.txt   = f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312
CAS index (MarkovEvidenceCapsule) = f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312

→ MATCH. cas.get(&capsule.capsule_id) is RESOLVABLE. SG-15.3 (next-session
  bootstrap from latest Markov capsule) holds.
```

R2 (broken) had:
- `LATEST_MARKOV_CAPSULE.txt` = `a94ae884...`
- CAS index for MarkovEvidenceCapsule = `e4932fca...` (different)
- `cas.get(Cid("a94ae884..."))` would fail.

The bug was in the writer: `capsule_id = sha256(prelim_bytes)` (with capsule_id+sha256 zeroed during hash) but `cas.put(final_bytes)` stored the post-population bytes. Sha256 of those final bytes ≠ capsule_id.

R3 fix: store the EXACT prelim bytes in CAS (with capsule_id+sha256 zeroed). `capsule_id = sha256(stored_bytes)`. CAS keys by sha256(stored_bytes) = capsule_id. `cas.get(&capsule_id)` succeeds; consumer canonical-decodes + restores capsule_id+sha256 from `Cid::from_content(&retrieved_bytes)`.

Same fix applied to `write_autopsy_capsule` and `derive_autopsies_for_bankruptcy` / `write_bankruptcy_autopsies_to_cas` (TB15-CAS-ID VETO closure).

## R3 unit tests (proves the contract)

- `runtime::markov_capsule::tests::write_markov_capsule_cas_resolvable_by_capsule_id` — asserts `cas.get(&cap.capsule_id)` succeeds + retrieved bytes' sha256 equals capsule_id + restore round-trip works.
- `runtime::autopsy_capsule::tests::write_bankruptcy_autopsies_to_cas_round_trip` — extended with same R3 contract assertions for autopsy capsules.

## Audit-from-tape closure record (recursive R1 → R2 → R3)

| Finding | R1 source | R2 status | R3 status |
|---|---|---|---|
| Q3 (CAS residency) | Codex | "fixed" via no-cas drop (didn't actually fix CAS-cid mismatch) | **FIXED**: capsule_id = sha256(stored_bytes); cas.get resolvable |
| Q4 (live override gate) | Codex | FIXED via `--include-prior-capsules` | unchanged |
| Q5 (byte-window scan) | Codex | FIXED via strengthened halt-trigger | unchanged |
| Q7/Q8 (flowchart_hashes) | Both | FIXED via new field + parser | unchanged |
| Q9 (dashboard not regenerable) | Codex | OBS-deferred to TB-16 | unchanged |
| Q12 (replay-determinism) | Gemini VETO | FIXED via activation gate | unchanged |
| **TB15-CAS-ID** (autopsy CAS-cid mismatch) | Codex R2 NEW VETO | n/a | **FIXED**: same writer pattern fix applied to write_autopsy_capsule + derive_autopsies_for_bankruptcy |

## Open items

- **OBS-TB15-R2-Q12-UPGRADE** (Gemini R2 recommendation): upgrade compile-time `TB15_AUTOPSY_ACTIVATION_LOGICAL_T` const to a chain-resident marker for improved long-term robustness. Carry-forward to TB-16+.
- **OBS-TB15-R2-Q7-TEST-HARDEN** (Gemini R2 recommendation): add negative-path tests for `read_flowchart_hashes_from_matrix`. Carry-forward.
- **OBS-TB-11-CAS-ID** (cross-cut, NEW R3): TB-11 `write_evidence_capsule` has the SAME CAS-cid mismatch bug. Not blocking TB-15 ship (no production consumer of EvidenceCapsule via cap.capsule_id yet) but should be fixed in TB-11 follow-up.

## Cross-references

- TB-15 charter: `handover/tracer_bullets/TB-15_charter_2026-05-03.md`
- TB-15 ship status: `handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md`
- R1 audits: `handover/audits/{CODEX,GEMINI}_TB_15_SHIP_AUDIT_2026-05-04_R1.md`
- R2 audits: `handover/audits/{CODEX,GEMINI}_TB_15_SHIP_AUDIT_2026-05-04_R2.md`
- R3 audits: `handover/audits/{CODEX,GEMINI}_TB_15_SHIP_AUDIT_2026-05-04_R3.md` (pending at evidence-write time)
- R3 closure doc: `handover/audits/RECURSIVE_AUDIT_TB_15_2026-05-04.md`
- Architect spec: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §6
- Source files: `src/runtime/markov_capsule.rs` (write_markov_capsule + restore_markov_capsule_from_cas_bytes), `src/runtime/autopsy_capsule.rs` (write_autopsy_capsule + restore_autopsy_capsule_from_cas_bytes + BankruptcyAutopsyDerivation)
