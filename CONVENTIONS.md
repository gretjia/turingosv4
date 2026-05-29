# Aider conventions (K-HARDEN-8 cross-CLI cold-start entry)

Aider auto-loads this file (and `.aider.conf.yml`) on cold start. It is a thin
pointer; the harness contract is single-sourced in `AGENTS.md`.

**Start here:** read `AGENTS.md` — the canonical cross-agent contract. It
defines the full read order (§2), risk classes (§5), restricted surfaces (§6),
the PR-only workflow (§14a), the audit doctrine (§9), the obligation ledger
(§16), and the Karpathy coding principles (§13). This file does not restate
those rules; if anything here conflicts with `AGENTS.md`, **`AGENTS.md` wins**.

Aider obeys that one contract identically to every other platform, free to use
its own native strengths to realize it — the Layer 1 / Layer 2 model in
`AGENTS.md` §2. Aider-specific shape (read-list, test command) lives in
`.aider.conf.yml`.

This file is intentionally short: Aider rule-adherence degrades past ~200 lines,
and full detail already lives in `AGENTS.md`.
