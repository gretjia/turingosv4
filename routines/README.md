# Routines — Source of Truth

Claude routines are configured via `RemoteTrigger` API (stored server-side at `claude.ai/code/routines`). Since no export API exists, this directory is the **canonical source**; server is the mirror.

## Design principle (lossless compression)

Only keep routines that do work local agents can't. Routines trade **speed for independence** — use them only when the speed cost is worth paying.

## Current active routines

| File | Trigger ID | Cron | Purpose |
|---|---|---|---|
| `daily_drift.yaml` | `trig_01VzqnmvBuqxCwpxXxfWNSYh` | `0 9 * * *` | Catches slow drift while researcher is absent (nights/weekends) |

**Quota**: 1/day on Max tier (15/day cap). 14/day free for ad-hoc fires.

## Disabled / archived

| Trigger ID | Why disabled |
|---|---|
| `trig_01VxNV8nG3L9Dzm1o5FRdJEC` | frozen_audit — redundant with local Codex+Gemini dual audit (4h lag was worse than minutes of local agents). Kept dormant (`enabled: false`) for future revival if speed becomes irrelevant. |

## Non-routine audits (faster path)

For per-milestone audits (M4 post-experiment, plan changes, etc.), use local agents:
- Codex (via `codex:codex-rescue` subagent)
- Gemini (via curl to `generativelanguage.googleapis.com`)

Both return in minutes, not hours. Combined = cross-vendor independence. Use this as the default audit path; routines only when local agents are unavailable (e.g., session closed).

## Sync protocol (manual, enforced by convention)

When editing `daily_drift.yaml`:
1. PR + dual audit (Codex + Gemini) — same governance as code changes to `src/`
2. After merge: `RemoteTrigger update trigger_id=<...> body=<rendered from yaml>`
3. Verify via `RemoteTrigger get` that live prompt matches yaml's prompt field

## Known trust assumptions (C-017 — name the silent failure)

1. **yaml ↔ cloud sync is manual**: no CI diff yet. *Future*: `ci/routines_diff.sh` to fail if `RemoteTrigger get` body ≠ yaml prompt.
