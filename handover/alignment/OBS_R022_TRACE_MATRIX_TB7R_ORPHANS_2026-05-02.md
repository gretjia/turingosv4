# OBS — TB-7R orphan TRACE_MATRIX rows (2026-05-02)

**Class**: Observation (alignment hygiene)
**Driver**: Codex micro-audit 2026-05-02 (`handover/audits/CODEX_TB7R_MICRO_AUDIT_2026-05-02.md`) Claim 7 = CHALLENGE.
**Authority**: architect verdict 2026-05-01 (`handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md`) §C9 — "NEVER fabricate FC numbers; if no precise row, use `FC-trace: WP-§5.L3/L4 + Art.I.1 + Art.III.4` + register an orphan justification."
**Status**: ACTIVE — these rows are ORPHAN until promoted to a future TRACE_MATRIX revision.

---

## Why this OBS exists

The first TB-7R audit-fix pass (commit `696d10f` + `392a516`) introduced two
new pub modules:

```text
experiments/minif2f_v4/src/chaintape_mode_gate.rs   (TB-7R Deliverable B)
src/runtime/genesis_report.rs                       (TB-7R Deliverable C)
```

Each module's pub items carried initial `TRACE_MATRIX FC?-N?` doc-comments
that were either **fabricated** (label looked correct but the FC node
number actually maps to a different concept) or **vague** (a flowchart
number without a node ID). Codex Claim 7 flagged this as CHALLENGE-level
in the micro-audit — not a constitutional VETO, but a hygiene violation
of verdict C9.

This OBS files the affected pub items as TRACE_MATRIX § 3 orphans with
explicit Constitutional Justification, until a future TRACE_MATRIX
revision adds canonical rows for them.

---

## Affected pub items (orphans)

### Orphan 1 — ChainTape-mode predicate gate

```text
crate            : minif2f_v4 (experiments/minif2f_v4)
file             : src/chaintape_mode_gate.rs
pub items        :
  - module                                  (line 1)
  - fn chaintape_supports_condition         (line 31)
  - const CHAINTAPE_UNSUPPORTED_CONDITIONS  (line 19, crate-private — listed for completeness)
```

**Initial label (fabricated)**: `TRACE_MATRIX FC1-N6: predicate / wtool gate`.
**Actual content of FC1-N6**: `input = ⟨q_i, s_i⟩` UniverseSnapshot —
`src/sdk/snapshot.rs:22` + `src/sdk/prompt.rs:15` (per
`handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:28`). Unrelated to
the gate's role.

**Corrected orphan justification**:

```text
TRACE_MATRIX § 3 orphan (TB-7R 2026-05-02):
  Pre-routing predicate gate that fail-closes any CONDITION known to
  bypass `bus.submit_typed_tx` authoritative routing under ChainTape
  mode. Prevents silent legacy-fallback emission of evidence that
  cannot be reconstructed from ChainTape + CAS.

FC-trace: Art.I.1 (机制 > 参数) + Art.III.4 (selective broadcasting /
shielding — silent degradation breaks both) + WP-§5.L3 (predicate
boundary) + WP-§5.L4 (authoritative ledger).

Promotion target: a future TRACE_MATRIX revision should add a row
for "FC<n>-PreRoutingPredicateGate" or equivalent, mapping to this
fn + the four unit tests + the integration call site at
experiments/minif2f_v4/src/bin/evaluator.rs:327.
```

### Orphan 2 — genesis_report.json emitter

```text
crate            : turingosv4 (root)
file             : src/runtime/genesis_report.rs
pub items        :
  - module                                  (line 1)
  - struct GenesisReport                    (line 22)
  - fn write_to_runtime_repo                (line 73)
  - fn hash_constitution_md                 (line 88)
  - fn hash_system_pubkey_manifest          (line 98)
  - public fields of GenesisReport          (lines 32 / 36 / 40 / 46 / 50 / 55 / 61 / 65 / 69)
```

**Initial label (incorrect mapping)**: `TRACE_MATRIX FC2 (Boot / Genesis)`.
**Actual content of FC2**: Append + Submit — L4 transition_ledger,
LedgerEntry, WorkTx routing surfaces (see
`handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:197-298`). FC2 is
NOT Boot / Genesis. Architect verdict §6.1 (which calls Flowchart 2
"Boot / Genesis") used a conceptual three-flowchart shorthand that does
not align with the canonical TRACE_MATRIX FC numbering — the three
flowcharts in TRACE_MATRIX are FC1 (Runtime State Transition), FC2
(Append/Submit), FC3 (Trust Root readonly subgraph), with **Boot /
Genesis** anchored under **Article IV Boot** rather than a separate FC.

**Corrected orphan justification**:

```text
TRACE_MATRIX § 3 orphan (TB-7R 2026-05-02):
  At-bootstrap audit-witness emitter. Captures the run's genesis
  preconditions (constitution_hash + runtime_repo + cas_path +
  system_pubkey_hash + agent_pubkeys_path + initial_balances +
  preseed task_id / task_open_tx / escrow_lock_tx) into a single
  durable artifact — the post-hoc audit witness that complements
  the existing initial_q_state.json (D7) and pinned_pubkeys.json.

FC-trace: Art.IV Boot (Bootstrap 公理 — 创世状态) + Art.I.1 (机制 >
参数) + Art.III.4 (selective broadcasting — only `public_summary` /
aggregate fields surface in non-privileged read views; raw state stays
in CAS) + WP-§5.L0 (Constitution Root anchor) + WP-§11 Boot.

Promotion target: a future TRACE_MATRIX revision should add a row
under the Article IV Boot heading mapping to GenesisReport + its
write/read API.

Public-field doc-comment policy: per the local C9 + ALIGN-STD
discussion, GenesisReport fields are pub data members of a serde
struct, not standalone pub functions — they inherit the parent
struct's TRACE_MATRIX backlink rather than each carrying their own.
```

---

## Cross-references

- Codex micro-audit: `handover/audits/CODEX_TB7R_MICRO_AUDIT_2026-05-02.md`
- TB-7R authorization: `handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md` §C9
- TRACE_MATRIX v3: `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md`
- TRACE_MATRIX v0 (FC1-N6 row): `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md:28`
- TB-7R charter: `handover/tracer_bullets/TB-7R_charter_2026-05-01.md`
- Three-node taxonomy: `handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md`

## Closure path

This OBS closes when a TRACE_MATRIX revision (v4+) adds canonical rows
for the two orphans listed above. Until then, the doc-comments in both
modules cite this OBS file directly so future readers can follow the
provenance chain.
