#!/usr/bin/env python3
"""
TuringOS UI IR Renderer — experiments/tisr_ui_spike/render.py

Reads a UI IR JSON page from stdin or --fixture <path>.
Renders to:
  --format text  (default) — plain text for terminal display
  --format json  — identity round-trip (validate then reprint)

Exit codes:
  0 = success
  1 = validation failure
  2 = argument error

Class 1 (additive, local). Python 3 stdlib only. No HTML output.
FC3-N31: materialized view, never authority.
"""

import argparse
import json
import os
import sys


# ---------------------------------------------------------------------------
# Schema loading
# ---------------------------------------------------------------------------

def load_schema(schema_path: str) -> dict:
    try:
        with open(schema_path, "r", encoding="utf-8") as f:
            return json.load(f)
    except (OSError, json.JSONDecodeError) as e:
        return {}  # best-effort; renderer continues without strict validation


def validate_page(page: dict, schema: dict) -> list[str]:
    """
    Best-effort structural validation against ui_ir_schema.json.
    Returns a list of error strings (empty = valid).
    Does not crash on extra fields (additionalProperties lenient for spike).
    """
    errors = []
    if not isinstance(page, dict):
        errors.append("top-level value is not an object")
        return errors

    for req in ("id", "title", "blocks"):
        if req not in page:
            errors.append(f"missing required top-level field: '{req}'")

    blocks = page.get("blocks")
    if blocks is not None and not isinstance(blocks, list):
        errors.append("'blocks' must be an array")
        return errors

    if isinstance(blocks, list):
        valid_kinds = {"text", "table", "agent_card", "task_card", "event_log", "dashboard_panel"}
        for i, block in enumerate(blocks):
            if not isinstance(block, dict):
                errors.append(f"blocks[{i}] is not an object")
                continue
            if "id" not in block:
                errors.append(f"blocks[{i}] missing 'id'")
            if "kind" not in block:
                errors.append(f"blocks[{i}] missing 'kind'")
            elif block["kind"] not in valid_kinds:
                errors.append(f"blocks[{i}] unknown kind '{block['kind']}'")

    return errors


# ---------------------------------------------------------------------------
# Text rendering helpers
# ---------------------------------------------------------------------------

SEPARATOR = "-" * 72


def render_cell(cell: dict) -> str:
    kind = cell.get("kind", "string")
    value = cell.get("value", "")
    if kind == "microcoin":
        return f"{value} μC"
    return str(value)


def render_text_block(block: dict) -> str:
    content = block.get("content", "")
    return content


def render_table_block(block: dict) -> str:
    lines = []
    caption = block.get("caption")
    if caption:
        lines.append(f"[ {caption} ]")
    columns = block.get("columns", [])
    rows = block.get("rows", [])

    if not columns:
        return "(empty table)"

    # Build column widths
    col_widths = [len(c) for c in columns]
    rendered_rows = []
    for row in rows:
        rendered = []
        for j, cell in enumerate(row):
            text = render_cell(cell) if isinstance(cell, dict) else str(cell)
            rendered.append(text)
            if j < len(col_widths):
                col_widths[j] = max(col_widths[j], len(text))
        rendered_rows.append(rendered)

    # Header
    header = "  ".join(c.ljust(col_widths[i]) for i, c in enumerate(columns))
    lines.append(header)
    lines.append("-" * len(header))
    for row in rendered_rows:
        cells = []
        for j, cell_text in enumerate(row):
            width = col_widths[j] if j < len(col_widths) else 0
            cells.append(cell_text.ljust(width))
        lines.append("  ".join(cells))

    return "\n".join(lines)


def render_agent_card_block(block: dict) -> str:
    lines = [
        f"  agent_id : {block.get('agent_id', '(unknown)')}",
        f"  role     : {block.get('role', '(unknown)')}",
        f"  balance  : {block.get('balance_micro', 0)} μC",
        f"  status   : {block.get('status', '(unknown)')}",
    ]
    return "\n".join(lines)


def render_task_card_block(block: dict) -> str:
    lines = [
        f"  task_id        : {block.get('task_id', '(unknown)')}",
        f"  problem_id     : {block.get('problem_id', '(unknown)')}",
        f"  status         : {block.get('status', '(unknown)')}",
        f"  reward         : {block.get('reward_micro', 0)} μC",
        f"  attempt_count  : {block.get('attempt_count', 0)}",
        f"  assigned_agent : {block.get('assigned_agent_id', 'unassigned')}",
    ]
    return "\n".join(lines)


def render_event_log_block(block: dict) -> str:
    events = block.get("events", [])
    lines = []
    for ev in events:
        layer = ev.get("layer", "?")
        kind = ev.get("kind", "?")
        tx_id = ev.get("tx_id", "?")
        summary = ev.get("summary", "")
        lines.append(f"  [{layer}] {kind}  {tx_id}")
        if summary:
            lines.append(f"         {summary}")
    return "\n".join(lines) if lines else "(no events)"


def render_dashboard_panel_block(block: dict) -> str:
    title = block.get("panel_title", "Panel")
    metrics = block.get("metrics", [])
    lines = [f"  {title}"]
    lines.append("  " + "-" * (len(title) + 2))
    for m in metrics:
        label = m.get("label", "")
        value = m.get("value", "")
        unit = m.get("unit", "")
        unit_str = f" {unit}" if unit else ""
        lines.append(f"    {label:<32} {value}{unit_str}")
    return "\n".join(lines)


BLOCK_RENDERERS = {
    "text":             render_text_block,
    "table":            render_table_block,
    "agent_card":       render_agent_card_block,
    "task_card":        render_task_card_block,
    "event_log":        render_event_log_block,
    "dashboard_panel":  render_dashboard_panel_block,
}


def render_page_text(page: dict) -> str:
    title = page.get("title", "(untitled)")
    page_id = page.get("id", "")
    lines = [
        SEPARATOR,
        f"  {title}",
        f"  {page_id}",
        SEPARATOR,
    ]
    for block in page.get("blocks", []):
        kind = block.get("kind", "unknown")
        block_id = block.get("id", "")
        lines.append(f"\n[{kind}] {block_id}")
        renderer = BLOCK_RENDERERS.get(kind)
        if renderer:
            lines.append(renderer(block))
        else:
            lines.append(f"  (unknown block kind: {kind})")
    lines.append("\n" + SEPARATOR)
    lines.append("  FC3-N31: materialized view — not authoritative over ChainTape/CAS")
    lines.append(SEPARATOR)
    return "\n".join(lines)


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main() -> int:
    parser = argparse.ArgumentParser(
        description="TuringOS UI IR renderer. Reads UI IR JSON, renders to text or json.",
    )
    parser.add_argument(
        "--fixture", "-f",
        metavar="PATH",
        help="Path to a UI IR JSON fixture file. If omitted, reads from stdin.",
    )
    parser.add_argument(
        "--format",
        choices=["text", "json"],
        default="text",
        help="Output format: 'text' (default) or 'json' (round-trip identity).",
    )
    parser.add_argument(
        "--schema",
        metavar="PATH",
        default=os.path.join(os.path.dirname(__file__), "ui_ir_schema.json"),
        help="Path to ui_ir_schema.json for validation (default: sibling file).",
    )

    try:
        args = parser.parse_args()
    except SystemExit as e:
        return 2

    # Load input
    try:
        if args.fixture:
            with open(args.fixture, "r", encoding="utf-8") as f:
                raw = f.read()
        else:
            raw = sys.stdin.read()
    except OSError as e:
        print(f"ERROR: cannot read input: {e}", file=sys.stderr)
        return 2

    # Parse JSON
    try:
        page = json.loads(raw)
    except json.JSONDecodeError as e:
        print(f"ERROR: invalid JSON: {e}", file=sys.stderr)
        return 1

    # Validate
    schema = load_schema(args.schema)
    errors = validate_page(page, schema)
    if errors:
        for err in errors:
            print(f"VALIDATION ERROR: {err}", file=sys.stderr)
        return 1

    # Render
    if args.format == "json":
        print(json.dumps(page, indent=2, ensure_ascii=False))
    else:
        print(render_page_text(page))

    return 0


if __name__ == "__main__":
    sys.exit(main())
