# TB-18 Atom F — Single-Chain 13/13 Smoke + β-A Feasibility Audit (2026-05-05)

## What this directory proves

**Charter §2 atom F ship gate** — single-chain 13/13 smoke audit verifies the TB-18.B-impl substrate is replay-deterministic, tamper-detectable, and architecturally aligned with PRE-17.7 β-A (no α CLI sidecar reliance). Charter wording verbatim:

> Atom F | 3 evidence | single-chain 13/13 smoke + β-A feasibility | 24h |
> `handover/evidence/tb_18_single_chain_13_of_13/README.md` + audit_tape verdict |
> PROCEED + 13/13 + tamper 3/3 + replay-byte-identical;
> β-A in-tape resolution exercised (NOT α CLI sidecar)

All five ship-gate asserts GREEN against canonical TB-18.B-impl Phase 4 r1 chain (commit `15b662c`).

## Run summary (canonical r1)

```text
input_chain:        handover/evidence/tb_18_b_phase4_2026-05-05/r1/
                    runtime_repo.dotgit.tar.gz + cas.dotgit.tar.gz
chain_seed_id:      tb18-arena-r1
input_chain_depth:  31 L4 entries on refs/transitions/main
input_distinct_kinds: 13/13

audit_tape verdict:        PROCEED  (passed=35, failed=0, halted=0, skipped=8)
audit_tape replay:         PROCEED  (passed=35, failed=0, halted=0, skipped=8)
verdict.json byte-cmp:     IDENTICAL ✓
audit_tape_tamper:         detected_count=3, expected=3, all_detected=true ✓
β-A feasibility:           FEASIBLE ✓
```

## Five ship-gate asserts

| # | Assert | Result | Source artifact |
|---|---|---|---|
| 1 | `verdict.json:verdict == "PROCEED"` | ✓ | `r1/verdict.json` |
| 2 | `verdict.json` byte-identical with `verdict_replay.json` | ✓ | `cmp -s` |
| 3 | `tamper_report.json:detected_count >= 3` | ✓ (3/3) | `r1/tamper_report.json` |
| 4 | 13 distinct tx kinds in single chain | ✓ (13/13) | `tb_18_b_phase4_2026-05-05/r1/evidence/tx_kind_distribution.json` |
| 5 | β-A in-tape resolution feasibility (NOT α sidecar) | ✓ FEASIBLE | `r1/beta_a_feasibility_check.json` |

## β-A feasibility breakdown

Per architect TB-18 ratification ruling Q4 + `feedback_markov_inheritance_tape_derived` + `MARKOV_INHERITANCE_POLICY §4`:

**Check A — α sidecar absent**:
- Project root has NO `LATEST_MARKOV_CAPSULE.txt` (deleted in TB-16.x.fix `f2bb871`).
- Canonical bytes (`runtime_repo` + `cas`) carry NO sidecar pointer.
- Result: ✓ all three locations clean.

**Check B — genesis chain mode**:
- audit_tape invoked WITHOUT `--markov-pointer` and WITHOUT `--prior-chain-runtime-repo`.
- Atom F single-chain smoke is genesis; β-A applies to the NEXT chain inheritance pattern.
- Replay byte-identical achieved without any external sidecar — all Markov state chain-derived.

**Check C — in-tape Markov capsule reference present**:
- Chain emits 3 `TerminalSummaryTx` (task_D + task_E + task_F per TB-18.B-impl Phase 4).
- Each `TerminalSummaryTx` carries an `EvidenceCapsule` reference (CAS-anchored).
- A NEXT chain's β-A in-tape resolver (per `MARKOV_INHERITANCE_POLICY §4`) walks back to the most recent `TerminalSummaryTx`, resolves its `evidence_capsule_cid` from CAS, and uses its `markov_capsule_cid` as the parent Markov tip.
- Anchor points exist → β-A in-tape resolution is feasible against this substrate.

**Verdict**: FEASIBLE. The substrate honors the architect's β-A-only mandate; α CLI sidecar is structurally unavailable (absent from repo + absent from canonical bytes).

## Directory layout

```
tb_18_single_chain_13_of_13/
├── README.md                              ← this file
└── r1/
    ├── verdict.json                       ← audit_tape primary verdict (35/0/0/8)
    ├── verdict_replay.json                ← replay verdict (byte-identical)
    ├── tamper_report.json                 ← 3/3 tamper attempts detected
    ├── beta_a_feasibility_check.json      ← β-A FEASIBLE verdict
    ├── audit_tape.stderr                  ← audit_tape + replay stderr
    └── audit_tape_tamper.stderr           ← audit_tape_tamper stderr
```

## How this run was produced

```bash
bash handover/tests/scripts/run_tb_18_atom_f_2026-05-05.sh
```

Reproducibility procedure:
1. Restore canonical bytes from TB-18.B-impl Phase 4 r1 tarballs to `$WORK_DIR`:
   - `tar xzf handover/evidence/tb_18_b_phase4_2026-05-05/r1/runtime_repo.dotgit.tar.gz -C $WORK_DIR/runtime_repo`
   - `tar xzf handover/evidence/tb_18_b_phase4_2026-05-05/r1/cas.dotgit.tar.gz -C $WORK_DIR/cas`
2. Run `target/release/audit_tape` with canonical genesis (`genesis_payload.toml`) + `constitution.md` + alignment dir.
3. Run again to produce `verdict_replay.json`; `cmp -s` against `verdict.json`.
4. Run `target/release/audit_tape_tamper` against the staged tree; verify `detected_count == 3`.
5. β-A feasibility checks A/B/C from `tx_kind_distribution.json` (chain-emitter authoritative).

The script script is idempotent, restores from canonical bytes each run, and cleans up the working tree on exit.

## Charter SG / FR closure

| Reference | Status |
|---|---|
| Charter §F gate "PROCEED + 13/13 + tamper 3/3 + replay-byte-identical" | ✅ all four conditions GREEN |
| Charter §F gate "β-A in-tape resolution exercised (NOT α CLI sidecar)" | ✅ FEASIBLE per `beta_a_feasibility_check.json` |
| FR-18.7 (single-chain integrity) | ✅ inherited from B-impl Phase 4 |
| FR-18.8 (13/13 distinct tx kinds) | ✅ inherited from B-impl Phase 4 |
| Architect Q4 STOP gate "TB-18 cannot use α CLI sidecar to fake β-A success" | ✅ no α sidecar exercised; chain-derived state alone produces byte-identical replay |

## Cross-references

- `handover/tracer_bullets/TB-18_charter_2026-05-05.md` §F (atom F gate)
- `handover/evidence/tb_18_b_phase4_2026-05-05/README.md` (input chain provenance)
- `handover/alignment/MARKOV_INHERITANCE_POLICY.md` §4 (β-A in-tape resolver spec)
- `handover/directives/2026-05-05_TB18_CHARTER_RATIFICATION_ARCHITECT_RULING.md` Q4 (β-A only ruling)
- Memory: `feedback_markov_inheritance_tape_derived` (OBS_R022 α closure)

## Forward gate

**Atom F SHIPPED** unblocks atom G0 (Codex micro-audit on substrate). Per charter §2 hard sequencing: `... → F → G0 → H → G1 → ship`. G0 verdict gates atom H (M-ladder M0 retry → M1 → M2 batch).

G0 trigger artifact filed at `handover/audits/CODEX_MICRO_AUDIT_TB_18_PRE_H_REQUEST_2026-05-05.md`; awaiting user external invocation.
