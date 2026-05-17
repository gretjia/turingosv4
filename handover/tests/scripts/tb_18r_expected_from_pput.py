#!/usr/bin/env python3
"""Extract TB-18R expected attempt count from one PPUT_RESULT JSON object."""

from __future__ import annotations

import json
import sys
from typing import Any


COUNT_KEYS = (
    "step_reject",
    "parse_fail",
    "llm_err",
    "sorry_block",
    "step_partial_ok",
)
OMEGA_KEYS = ("omega_wtool", "complete", "complete_via_tape")


def int_field(data: dict[str, Any], key: str) -> int:
    value = data.get(key, 0)
    if isinstance(value, bool):
        raise ValueError(f"{key} must be an integer count, got bool")
    if not isinstance(value, int):
        raise ValueError(f"{key} must be an integer count, got {type(value).__name__}")
    if value < 0:
        raise ValueError(f"{key} must be non-negative")
    return value


def extract(pput: dict[str, Any]) -> dict[str, Any]:
    tool_dist = pput.get("tool_dist", {})
    if not isinstance(tool_dist, dict):
        raise ValueError("tool_dist must be an object")

    components = {key: int_field(tool_dist, key) for key in COUNT_KEYS}
    omega = sum(int_field(tool_dist, key) for key in OMEGA_KEYS)
    components["omega"] = omega

    expected = sum(components.values())
    solved = bool(pput.get("solved", False))
    hit_max = bool(pput.get("hit_max_tx", False))
    halt_class = "OmegaAccepted" if solved else "MaxTxExhausted"
    if not solved and not hit_max:
        halt_class = "MaxTxExhausted"

    return {
        "expected_completed_attempts": expected,
        "halt_class": halt_class,
        "components": components,
        "synthetic_preseed_counted": False,
        "formula": (
            "step_reject + parse_fail + llm_err + sorry_block + "
            "omega_wtool + complete + complete_via_tape + step_partial_ok"
        ),
    }


def main() -> int:
    try:
        pput = json.load(sys.stdin)
        if not isinstance(pput, dict):
            raise ValueError("PPUT_RESULT must be a JSON object")
        print(json.dumps(extract(pput), sort_keys=True))
    except Exception as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
