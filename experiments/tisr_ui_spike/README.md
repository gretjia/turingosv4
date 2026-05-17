# TISR UI IR Spike — experiments/tisr_ui_spike

## Purpose

Local generative UI IR spike for TISR Phase 6.1 §8 PACKET §1.2.

Delivers a fixture-based renderer that proves out the Page → Block → Cell IR
schema before any web serving or real ChainTape/CAS wiring is attempted.

Consumed by: future Phase 7 Web MVP (not yet built).
Risk class: **Class 1** (additive, local, fixture-based, no src/ touch).

---

## Schema Overview (Page → Block → Cell)

Borrowed from Karpathy Software 3.0 framing + TISR research layer:

```
Page
  id:     string   — stable identifier for this view
  title:  string   — human-readable title
  blocks: Block[]  — ordered list of content blocks

Block  (one of: text | table | agent_card | task_card | event_log | dashboard_panel)
  id:    string
  kind:  BlockKind
  ...kind-specific fields...

Cell  (within table blocks)
  kind:  CellKind   — string | integer | microcoin | agent_id | tx_id | cid
  value: <varies>
```

### Block kinds

| kind            | Purpose                                      |
|-----------------|----------------------------------------------|
| text            | Plain prose or status paragraph              |
| table           | Grid of typed cells (rows × columns)         |
| agent_card      | Single agent summary (id, role, balance)     |
| task_card       | Single task summary (id, status, problem)    |
| event_log       | Ordered list of tape events                  |
| dashboard_panel | Named KV metric panel (solve rate, PPUT …)   |

### Cell kinds

| kind       | Wire type        | Semantics                                  |
|------------|------------------|--------------------------------------------|
| string     | JSON string      | Free text                                  |
| integer    | JSON number (int)| Non-monetary integer count                 |
| microcoin  | JSON number (int)| μCoin amount (integer; MUST NOT be float)  |
| agent_id   | JSON string      | Agent identity key (hex or mnemonic)       |
| tx_id      | JSON string      | ChainTape transaction ID                   |
| cid        | JSON string      | CAS content-addressed identifier           |

---

## Fixture-Based Rendering Model

Fixtures in `fixtures/` simulate read-only ChainTape/CAS-derived views that
would eventually be emitted by `turingos audit_dashboard`, `turingos agent list`,
and `turingos task view` commands.

The renderer (`render.py`) loads a fixture, validates it against
`ui_ir_schema.json`, then emits either:

- `--format text` (default): plain text suitable for terminal display
- `--format json`: identity round-trip (validates schema then reprints)

No HTML is generated at this layer. HTML rendering is Phase 7 work.

---

## Constraints

- **Local only** — not served, not network-accessible
- **Not authoritative** — fixtures are derived views, never source of truth
- **No web framework** — Python stdlib only
- **No Cargo.toml change** — Python + JSON; workspace untouched
- **No Trust Root touch** — no edit to src/lib.rs, Cargo.toml, Cargo.lock

---

## Usage

```bash
# Render a fixture as plain text
python3 render.py --fixture fixtures/dashboard_sample.json

# Render as JSON (round-trip validation)
python3 render.py --fixture fixtures/task_view_sample.json --format json

# Pipe a UI IR JSON blob
cat fixtures/agent_view_sample.json | python3 render.py

# Run all tests
bash test_render.sh
```

---

## Validator (Phase 6.2 W2.2)

`validate.py` is a Python 3 stdlib-only schema validator for UI IR JSON
documents. It checks conformance against the inlined schema constraints
(subset of JSON Schema draft-07) and emits **all** validation errors before
exiting — not just the first one.

### Usage

```bash
# Validate a fixture file
python3 validate.py --fixture fixtures/dashboard_sample.json

# Validate and suppress the success line (useful in scripts)
python3 validate.py --fixture fixtures/agent_view_sample.json --quiet

# Validate via stdin
cat fixtures/task_view_sample.json | python3 validate.py --stdin

# Show help
python3 validate.py --help

# Run the validator smoke / integration tests
bash test_validate.sh
```

### Exit codes

| Code | Meaning |
|------|---------|
| 0    | Document conforms to the UI IR schema |
| 1    | Validation error — path + reason(s) emitted to stderr |
| 2    | Argument or I/O error (missing file, malformed JSON, bad invocation) |

### What is validated

- Page: `id` (string), `title` (string), `blocks` (array) required.
- Each Block: `id` (string), `kind` (one of the six valid block kinds) required; kind-specific required fields enforced per schema.
- TableBlock: `columns` (array of strings), `rows` (array of cell arrays); each Cell validated for `kind` + `value` types.
- AgentCardBlock: `balance_micro` must be a non-negative integer (never float).
- TaskCardBlock: `status` must be one of `open | solved | expired | exhausted | rejected`; `reward_micro` and `attempt_count` must be non-negative integers.
- EventLogBlock: each EventEntry requires `tx_id`, `kind`, `layer`; `layer` must be `L4` or `L4E`.
- DashboardPanelBlock: each MetricEntry requires `label` and `value`.
- Cell: `kind` must be one of `string | integer | microcoin | agent_id | tx_id | cid`; `value` type is checked against `kind` (integer/microcoin → JSON integer; string/agent_id/tx_id/cid → JSON string).

### Constraints

- **stdlib only** — no `jsonschema`, no external pip packages (Python 3 `json`, `argparse`, `sys`, `pathlib` only).
- **Not authoritative** — FC3-N31: materialized-view tool only.
- **No src/ touch** — no Cargo.toml or Trust Root changes.

---

## File Map

```
experiments/tisr_ui_spike/
  README.md               — this file
  NON_CLAIMS.md           — explicit shielding boundaries
  ui_ir_schema.json       — JSON Schema draft-07 for Page/Block/Cell IR
  render.py               — Python 3 stdlib renderer (text + json)
  test_render.sh          — 3 round-trip render tests
  validate.py             — Python 3 stdlib-only schema validator (Phase 6.2 W2.2)
  test_validate.sh        — 7 validator smoke / integration tests
  fixtures/
    dashboard_sample.json  — simulates audit_dashboard output as UI IR
    agent_view_sample.json — simulates turingos agent list output as UI IR
    task_view_sample.json  — simulates turingos task view output as UI IR
```

---

## Relationship to TISR Research

This spike validates the IR schema layer described in the TISR dual-axis
research docs (Software 3.0 HCI + A2A). The schema is intentionally minimal:
enough to demonstrate Page → Block → Cell composition without encoding any
business logic or authorization surface.

FC-trace: **FC3-N31** — UI IR is a materialized view, never an authority.
