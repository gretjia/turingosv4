#!/usr/bin/env python3
"""TuringOS UI IR Schema Validator (Phase 6.2 W2.2; stdlib-only).

Validates a UI IR JSON document against the inlined schema
constraints (subset of JSON Schema draft-07; pure stdlib).

Exit codes:
  0  document conforms to schema
  1  validation error (path + reason emitted to stderr)
  2  argument error (missing --fixture, file not found, malformed JSON)

Usage:
  python3 validate.py --fixture path/to/ir.json
  cat ir.json | python3 validate.py --stdin

FC3-N31: this validator is a materialized-view tool; it never becomes
authoritative over ChainTape/CAS.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

# ---------------------------------------------------------------------------
# Schema constants — derived from ui_ir_schema.json structure
# ---------------------------------------------------------------------------

VALID_BLOCK_KINDS = frozenset(
    {"text", "table", "agent_card", "task_card", "event_log", "dashboard_panel"}
)
VALID_CELL_KINDS = frozenset(
    {"string", "integer", "microcoin", "agent_id", "tx_id", "cid"}
)
VALID_TASK_STATUSES = frozenset(
    {"open", "solved", "expired", "exhausted", "rejected"}
)
VALID_EVENT_LAYERS = frozenset({"L4", "L4E"})

# Required fields per block kind
_BLOCK_REQUIRED: dict[str, list[str]] = {
    "text":            ["id", "kind", "content"],
    "table":           ["id", "kind", "columns", "rows"],
    "agent_card":      ["id", "kind", "agent_id", "role", "balance_micro"],
    "task_card":       ["id", "kind", "task_id", "problem_id", "status"],
    "event_log":       ["id", "kind", "events"],
    "dashboard_panel": ["id", "kind", "panel_title", "metrics"],
}


# ---------------------------------------------------------------------------
# Validators — traverse the doc and collect ALL violations
# ---------------------------------------------------------------------------

def validate_page(doc: object, errors: list[str]) -> None:
    """Validate a top-level Page document (must be an object)."""
    if not isinstance(doc, dict):
        errors.append(
            f"$: expected object (Page), got {type(doc).__name__!r}"
        )
        return

    # Required top-level fields
    for field, expected_type, type_name in (
        ("id",     str,  "string"),
        ("title",  str,  "string"),
        ("blocks", list, "array"),
    ):
        if field not in doc:
            errors.append(f"$: missing required field {field!r}")
        elif not isinstance(doc[field], expected_type):
            errors.append(
                f"$.{field}: expected {type_name}, "
                f"got {type(doc[field]).__name__!r}"
            )

    # Validate each block
    if isinstance(doc.get("blocks"), list):
        for i, block in enumerate(doc["blocks"]):
            validate_block(block, f"$.blocks[{i}]", errors)


def validate_block(block: object, path: str, errors: list[str]) -> None:
    """Validate a single Block object."""
    if not isinstance(block, dict):
        errors.append(
            f"{path}: expected object (Block), got {type(block).__name__!r}"
        )
        return

    # 'id' is required on every block
    if "id" not in block:
        errors.append(f"{path}: missing required field 'id'")
    elif not isinstance(block["id"], str):
        errors.append(
            f"{path}.id: expected string, got {type(block['id']).__name__!r}"
        )

    # 'kind' is required and must be a known discriminant
    if "kind" not in block:
        errors.append(f"{path}: missing required field 'kind'")
        return  # can't validate kind-specific fields without the discriminant

    kind = block["kind"]
    if not isinstance(kind, str):
        errors.append(
            f"{path}.kind: expected string, got {type(kind).__name__!r}"
        )
        return

    if kind not in VALID_BLOCK_KINDS:
        errors.append(
            f"{path}.kind: unknown block kind {kind!r}; "
            f"expected one of {sorted(VALID_BLOCK_KINDS)}"
        )
        return  # unknown kind — no further kind-specific checks

    # Check kind-specific required fields
    for field in _BLOCK_REQUIRED.get(kind, []):
        if field not in block:
            errors.append(f"{path}: missing required field {field!r} for kind={kind!r}")

    # Kind-specific deeper validation
    if kind == "text":
        _validate_text_block(block, path, errors)
    elif kind == "table":
        _validate_table_block(block, path, errors)
    elif kind == "agent_card":
        _validate_agent_card_block(block, path, errors)
    elif kind == "task_card":
        _validate_task_card_block(block, path, errors)
    elif kind == "event_log":
        _validate_event_log_block(block, path, errors)
    elif kind == "dashboard_panel":
        _validate_dashboard_panel_block(block, path, errors)


def _validate_text_block(block: dict, path: str, errors: list[str]) -> None:
    content = block.get("content")
    if content is not None and not isinstance(content, str):
        errors.append(
            f"{path}.content: expected string, got {type(content).__name__!r}"
        )


def _validate_table_block(block: dict, path: str, errors: list[str]) -> None:
    columns = block.get("columns")
    if columns is not None:
        if not isinstance(columns, list):
            errors.append(
                f"{path}.columns: expected array, got {type(columns).__name__!r}"
            )
        else:
            for i, col in enumerate(columns):
                if not isinstance(col, str):
                    errors.append(
                        f"{path}.columns[{i}]: expected string, "
                        f"got {type(col).__name__!r}"
                    )

    rows = block.get("rows")
    if rows is not None:
        if not isinstance(rows, list):
            errors.append(
                f"{path}.rows: expected array, got {type(rows).__name__!r}"
            )
        else:
            for r, row in enumerate(rows):
                if not isinstance(row, list):
                    errors.append(
                        f"{path}.rows[{r}]: expected array of cells, "
                        f"got {type(row).__name__!r}"
                    )
                    continue
                for c, cell in enumerate(row):
                    validate_cell(cell, f"{path}.rows[{r}][{c}]", errors)


def _validate_agent_card_block(block: dict, path: str, errors: list[str]) -> None:
    balance = block.get("balance_micro")
    if balance is not None:
        if not isinstance(balance, int) or isinstance(balance, bool):
            errors.append(
                f"{path}.balance_micro: expected integer (non-float), "
                f"got {type(balance).__name__!r}"
            )
        elif balance < 0:
            errors.append(
                f"{path}.balance_micro: must be >= 0, got {balance}"
            )


def _validate_task_card_block(block: dict, path: str, errors: list[str]) -> None:
    status = block.get("status")
    if status is not None and status not in VALID_TASK_STATUSES:
        errors.append(
            f"{path}.status: unknown task status {status!r}; "
            f"expected one of {sorted(VALID_TASK_STATUSES)}"
        )

    for int_field in ("reward_micro", "attempt_count"):
        val = block.get(int_field)
        if val is not None:
            if not isinstance(val, int) or isinstance(val, bool):
                errors.append(
                    f"{path}.{int_field}: expected integer, "
                    f"got {type(val).__name__!r}"
                )
            elif val < 0:
                errors.append(f"{path}.{int_field}: must be >= 0, got {val}")


def _validate_event_log_block(block: dict, path: str, errors: list[str]) -> None:
    events = block.get("events")
    if events is not None:
        if not isinstance(events, list):
            errors.append(
                f"{path}.events: expected array, got {type(events).__name__!r}"
            )
        else:
            for i, ev in enumerate(events):
                _validate_event_entry(ev, f"{path}.events[{i}]", errors)


def _validate_event_entry(ev: object, path: str, errors: list[str]) -> None:
    if not isinstance(ev, dict):
        errors.append(
            f"{path}: expected object (EventEntry), got {type(ev).__name__!r}"
        )
        return
    for field in ("tx_id", "kind", "layer"):
        if field not in ev:
            errors.append(f"{path}: missing required field {field!r}")
    layer = ev.get("layer")
    if layer is not None and layer not in VALID_EVENT_LAYERS:
        errors.append(
            f"{path}.layer: unknown layer {layer!r}; "
            f"expected one of {sorted(VALID_EVENT_LAYERS)}"
        )


def _validate_dashboard_panel_block(block: dict, path: str, errors: list[str]) -> None:
    metrics = block.get("metrics")
    if metrics is not None:
        if not isinstance(metrics, list):
            errors.append(
                f"{path}.metrics: expected array, got {type(metrics).__name__!r}"
            )
        else:
            for i, m in enumerate(metrics):
                _validate_metric_entry(m, f"{path}.metrics[{i}]", errors)


def _validate_metric_entry(m: object, path: str, errors: list[str]) -> None:
    if not isinstance(m, dict):
        errors.append(
            f"{path}: expected object (MetricEntry), got {type(m).__name__!r}"
        )
        return
    for field in ("label", "value"):
        if field not in m:
            errors.append(f"{path}: missing required field {field!r}")
    label = m.get("label")
    if label is not None and not isinstance(label, str):
        errors.append(
            f"{path}.label: expected string, got {type(label).__name__!r}"
        )


def validate_cell(cell: object, path: str, errors: list[str]) -> None:
    """Validate a single Cell object (within a table row)."""
    if not isinstance(cell, dict):
        errors.append(
            f"{path}: expected object (Cell), got {type(cell).__name__!r}"
        )
        return

    # 'kind' is required
    if "kind" not in cell:
        errors.append(f"{path}: missing required field 'kind'")
    else:
        kind = cell["kind"]
        if not isinstance(kind, str):
            errors.append(
                f"{path}.kind: expected string, got {type(kind).__name__!r}"
            )
        elif kind not in VALID_CELL_KINDS:
            errors.append(
                f"{path}.kind: unknown cell kind {kind!r}; "
                f"expected one of {sorted(VALID_CELL_KINDS)}"
            )
        else:
            # Type-check value against declared kind
            val = cell.get("value")
            if val is not None:
                if kind in ("integer", "microcoin"):
                    if not isinstance(val, int) or isinstance(val, bool):
                        errors.append(
                            f"{path}.value: kind={kind!r} requires integer value, "
                            f"got {type(val).__name__!r}"
                        )
                    elif val < 0:
                        errors.append(
                            f"{path}.value: kind={kind!r} requires >= 0, got {val}"
                        )
                elif kind in ("string", "agent_id", "tx_id", "cid"):
                    if not isinstance(val, str):
                        errors.append(
                            f"{path}.value: kind={kind!r} requires string value, "
                            f"got {type(val).__name__!r}"
                        )

    # 'value' is required
    if "value" not in cell:
        errors.append(f"{path}: missing required field 'value'")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main(argv: list[str] | None = None) -> int:
    p = argparse.ArgumentParser(
        description=(
            "Validate a UI IR JSON document against the TuringOS UI IR schema. "
            "Emits all validation errors before exiting. "
            "FC3-N31: materialized view tool; never authoritative over ChainTape/CAS."
        ),
    )
    group = p.add_mutually_exclusive_group(required=True)
    group.add_argument(
        "--fixture",
        type=str,
        metavar="PATH",
        help="Path to a UI IR JSON file.",
    )
    group.add_argument(
        "--stdin",
        action="store_true",
        help="Read UI IR JSON from stdin.",
    )
    p.add_argument(
        "--quiet",
        action="store_true",
        help="Suppress success message on stdout.",
    )
    args = p.parse_args(argv)

    # Read input
    try:
        if args.fixture:
            text = Path(args.fixture).read_text(encoding="utf-8")
        else:
            text = sys.stdin.read()
    except FileNotFoundError:
        print(
            f"validate.py: file not found: {args.fixture}",
            file=sys.stderr,
        )
        return 2
    except OSError as e:
        print(f"validate.py: I/O error: {e}", file=sys.stderr)
        return 2

    # Parse JSON
    try:
        doc = json.loads(text)
    except json.JSONDecodeError as e:
        print(f"validate.py: invalid JSON: {e}", file=sys.stderr)
        return 2

    # Validate — collect ALL errors before reporting
    errors: list[str] = []
    validate_page(doc, errors)

    if errors:
        print(
            f"validate.py: {len(errors)} validation error(s):",
            file=sys.stderr,
        )
        for err in errors:
            print(f"  - {err}", file=sys.stderr)
        return 1

    if not args.quiet:
        n_blocks = len(doc.get("blocks", []))
        fixture_label = args.fixture if args.fixture else "<stdin>"
        print(
            f"OK: {fixture_label} conforms to UI IR schema "
            f"(title={doc.get('title')!r}, {n_blocks} block(s))"
        )
    return 0


if __name__ == "__main__":
    sys.exit(main())
