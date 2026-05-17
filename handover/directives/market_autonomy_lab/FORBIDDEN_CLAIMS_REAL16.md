# REAL-16 Forbidden Claims Scan

Active REAL-16 wording must remain candidate-only.

## Forbidden Phrases

| phrase | status | replacement |
| --- | --- | --- |
| `E4 achieved` | forbidden as an active claim | `E4 candidate pending audit` |
| `market emergence proven` | forbidden | `market emergence candidate pending final audit`, only after final packet audit |
| `market mechanism shipped` | forbidden | `research evidence only` |
| `E2 achieved` | forbidden | `E2 candidate pending audit` or `E2 replicated candidate` |
| `E3 achieved` | forbidden | `E3 candidate pending audit` |

## Current REAL-16 Claim Boundary

Allowed:

```text
REAL-16 is E4 candidate pending audit.
```

Not allowed:

```text
E4 achieved
market emergence proven
market mechanism shipped
```

Historical or negated occurrences of the forbidden strings are allowed only
when they are explicitly inside a forbidden-claims list, test assertion, or
negative statement.
