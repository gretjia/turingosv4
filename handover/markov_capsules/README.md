# `handover/markov_capsules/` — historical artifacts ONLY

**Status as of 2026-05-04 (TB-16.x.fix; architect OBS_R022 Option α)**:
this directory holds **historical** Markov capsule pointers and JSON
exports. Files here are **NOT canonical input** to runtime audit,
replay, or dashboard rendering.

## Why

The previous `LATEST_MARKOV_CAPSULE.txt` global pointer was an
**Art. 0.2 parallel ledger**:

- filesystem-side global, not derived from any tape
- carried cross-chain state (a cid pointing at bytes resident in a
  single per-problem CAS, unresolvable from any other chain's CAS)
- no `assert_eq!(view, derive_from_tape(tape))` 守恒 test
- last-writer-wins lifecycle

Per architect ruling 2026-05-04
(`handover/directives/2026-05-04_TB16_OBS_R022_ARCHITECT_RULING.md`),
the file has been **deleted** and the production binaries
(`audit_tape`, `audit_tape_tamper`, `audit_dashboard`,
`generate_markov_capsule`) no longer read or write it.

## Where Markov inheritance lives now

| Scenario | Source of truth |
|---|---|
| Fresh isolated chain (genesis) | `previous_capsule_cid: None`; `audit_tape` invoked **without** `--markov-pointer` → Layer G assertions Skipped (constitutional per architect Q2.b) |
| Per-run pointer file | Caller passes `audit_tape --markov-pointer <per-run-path>`; the path must exist AND its cid must resolve in the supplied `--cas-dir`. Present-but-unresolvable → fail-closed BLOCK (TB-16.x.1 + architect Q2.c) |
| Inherit from prior chain | Caller passes `audit_tape --prior-chain-runtime-repo <path>`; resolver reads `<path>/markov_tip.cid` (per-runtime-repo file, **NOT global**). α minimum-viable resolver; full in-tape walk lands in TB-16.x.2.4 / 2.6 (β chain continuation per Art. 0.4 path B) |
| Audit dashboard | `audit_dashboard --markov-capsule-cid <hex>`; absence renders the empty-state hint |

## Files retained as historical artifact

- `MARKOV_TB-15-R3_2026-05-03.json` — TB-15 R3 ship-gate Markov capsule
  JSON export. Human-readable artifact only; **not** a reference input.

## What goes where for new TBs

Per architect ruling §B.3.2: **per-run Markov JSON should be written to
`handover/evidence/tb_*/markov/`** alongside the run's other evidence,
not into this directory. This directory survives only to hold the
TB-15 R3 historical capsule above. New TBs should add their per-run
JSON under their own evidence directory.

## Cross-references

- Architect ruling (lossless): `handover/directives/2026-05-04_TB16_OBS_R022_ARCHITECT_RULING.md`
- OBS file: `handover/alignment/OBS_R022_GLOBAL_LATEST_MARKOV_PARALLEL_LEDGER_2026-05-04.md`
- TB-16.x.fix charter: `handover/tracer_bullets/TB-16.x.fix_charter_2026-05-04.md`
- Constitution Art. 0.2 (Tape Canonical) — `constitution.md` lines 52–95
- Constitution Art. 0.4 path B (chain continuation) — `constitution.md` lines 114–152
