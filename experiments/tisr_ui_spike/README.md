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

## File Map

```
experiments/tisr_ui_spike/
  README.md               — this file
  NON_CLAIMS.md           — explicit shielding boundaries
  ui_ir_schema.json       — JSON Schema draft-07 for Page/Block/Cell IR
  render.py               — Python 3 stdlib renderer (text + json)
  test_render.sh          — 3 round-trip tests
  fixtures/
    dashboard_sample.json — simulates audit_dashboard output as UI IR
    agent_view_sample.json — simulates turingos agent list output as UI IR
    task_view_sample.json — simulates turingos task view output as UI IR
```

---

## Relationship to TISR Research

This spike validates the IR schema layer described in the TISR dual-axis
research docs (Software 3.0 HCI + A2A). The schema is intentionally minimal:
enough to demonstrate Page → Block → Cell composition without encoding any
business logic or authorization surface.

FC-trace: **FC3-N31** — UI IR is a materialized view, never an authority.
