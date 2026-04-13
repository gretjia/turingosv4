# TuringOS v4 Rule Engine

## Architecture
```
CLAUDE.md instructions (~70% compliance)
  + hooks/judge.sh (closes gap to ~100%)
    + rules/engine.py (evaluates YAML rules)
      + rules/active/*.yaml (dynamic, add/remove = add/remove file)
```

## How It Works
1. Claude edits a file → `judge.sh` receives JSON on stdin
2. `judge.sh` calls `rules/engine.py` with file path + content
3. Engine loads all YAML rules, filters by `file_glob`
4. For each matching rule: runs `check.pattern` regex against content
5. Block rule matches → exit 2 (edit rejected)
6. Warn rule matches → exit 0 + log to enforcement.log + trace

## Rule Schema
See `rules/SCHEMA.yaml` for full spec.

## Active Rules

### Block Level (exit 2 — hard enforcement)
| ID | Name | Axiom |
|----|------|-------|
| R-001 | kernel_purity | Law 1: zero domain knowledge |
| R-002 | no_coin_minting | Law 2: no post-genesis printing |
| R-003 | no_wal_deletion | Tape append-only |
| R-004 | lean_syntax_in_prompts | Rule 22: black-box |
| R-005 | forced_investment | Law 2: voluntary staking |

### Warn Level (exit 0 — advisory + log)
| ID | Name | Axiom |
|----|------|-------|
| R-006 | kernel_modification | Law 1 |
| R-007 | bus_lifecycle | Engine separation |
| R-008 | market_constants | Law 2 |
| R-009 | payload_limits | Rule 21 |
| R-013 | format_contract | Bitter Lesson (V-009) |

## Adding a Rule
1. Create `rules/active/R-xxx_name.yaml` following SCHEMA
2. Done. Engine picks it up automatically.
Hard cap: 30 rules maximum.

## Traces
Rule triggers are logged to `traces/sessions/{date}.jsonl` for analysis.
Use `/harness-reflect` to review rule effectiveness.
