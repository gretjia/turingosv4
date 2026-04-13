#!/usr/bin/env python3
"""TuringOS v4 Rule Engine — Pure Python predicate evaluator.

Reads content from stdin, checks against YAML rules in --rules-dir,
outputs warnings/blocks. Exit 0 = pass, exit 2 = block.

Called by: .claude/hooks/judge.sh
"""
import argparse
import fnmatch
import json
import os
import re
import sys
from datetime import datetime, timezone


def load_yaml_simple(path: str) -> dict:
    """Minimal YAML parser for flat rule files. No PyYAML dependency."""
    data = {}
    current_key = None
    with open(path) as f:
        for line in f:
            line = line.rstrip()
            if not line or line.startswith("#"):
                continue
            if line.startswith("  ") and current_key:
                # Nested value (for check.type, check.pattern, stats.*)
                kv = line.strip().split(":", 1)
                if len(kv) == 2:
                    k, v = kv[0].strip(), kv[1].strip().strip('"').strip("'")
                    if current_key not in data or not isinstance(data[current_key], dict):
                        data[current_key] = {}
                    data[current_key][k] = v
            else:
                kv = line.split(":", 1)
                if len(kv) == 2:
                    k, v = kv[0].strip(), kv[1].strip().strip('"').strip("'")
                    current_key = k
                    data[k] = v
    return data


def check_rule(rule: dict, content: str) -> bool:
    """Returns True if the rule triggers (violation detected)."""
    check = rule.get("check", {})
    if not isinstance(check, dict):
        return False

    check_type = check.get("type", "grep")
    pattern = check.get("pattern", "")

    if not pattern:
        return False

    if check_type == "grep":
        return bool(re.search(pattern, content, re.IGNORECASE))
    elif check_type == "grep_inverse":
        return not bool(re.search(pattern, content, re.IGNORECASE))
    elif check_type == "compound":
        # All sub-patterns must match
        parts = [p.strip() for p in pattern.split("&&")]
        return all(re.search(p, content, re.IGNORECASE) for p in parts)
    return False


def write_trace(traces_dir: str, rule_id: str, file_path: str, message: str, verdict: str):
    """Append a trace entry as JSONL."""
    if not traces_dir:
        return
    os.makedirs(traces_dir, exist_ok=True)
    today = datetime.now(timezone.utc).strftime("%Y-%m-%d")
    trace_file = os.path.join(traces_dir, f"{today}.jsonl")
    entry = {
        "ts": datetime.now(timezone.utc).isoformat(),
        "event": verdict,
        "rule": rule_id,
        "file": file_path,
        "message": message,
    }
    with open(trace_file, "a") as f:
        f.write(json.dumps(entry) + "\n")


def write_log(log_path: str, rule_id: str, enforcement: str, file_path: str, message: str):
    """Append to enforcement.log."""
    if not log_path:
        return
    ts = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S")
    with open(log_path, "a") as f:
        f.write(f"[{ts}] {enforcement.upper()} {rule_id} | {file_path} | {message}\n")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--file", required=True)
    parser.add_argument("--rules-dir", required=True)
    parser.add_argument("--log", default="")
    parser.add_argument("--traces-dir", default="")
    args = parser.parse_args()

    content = sys.stdin.read()
    if not content:
        sys.exit(0)

    file_path = args.file
    blocked = False

    # Load and evaluate all matching rules
    if not os.path.isdir(args.rules_dir):
        sys.exit(0)

    for fname in sorted(os.listdir(args.rules_dir)):
        if not fname.endswith((".yaml", ".yml")):
            continue
        rule_path = os.path.join(args.rules_dir, fname)
        rule = load_yaml_simple(rule_path)

        # Check file_glob match
        file_glob = rule.get("file_glob", "*")
        # Normalize: match against basename and full path
        basename = os.path.basename(file_path)
        if not (fnmatch.fnmatch(basename, file_glob) or fnmatch.fnmatch(file_path, f"*{file_glob}")):
            continue

        if check_rule(rule, content):
            rule_id = rule.get("id", fname)
            enforcement = rule.get("enforcement", "warn")
            message = rule.get("message", f"Rule {rule_id} triggered")

            if enforcement == "block":
                print(f"BLOCKED by {rule_id}: {message}")
                write_log(args.log, rule_id, "block", file_path, message)
                write_trace(args.traces_dir, rule_id, file_path, message, "block")
                blocked = True
            else:
                print(f"WARNING {rule_id}: {message}")
                write_log(args.log, rule_id, "warn", file_path, message)
                write_trace(args.traces_dir, rule_id, file_path, message, "warn")

    sys.exit(2 if blocked else 0)


if __name__ == "__main__":
    main()
