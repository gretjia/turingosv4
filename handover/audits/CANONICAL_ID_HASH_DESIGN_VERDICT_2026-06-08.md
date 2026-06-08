# Canonical-ID → Content-Hash: Precise Design Verdict (MEMBRANE-SATISFIES)

**Date**: 2026-06-08
**§8 token**: `APPROVE-CANONICAL-ID-CONTENT-HASH-HANDLE` (granted by user 2026-06-08)
**User execution directive**: *"你用workflow进行精确设计后，对其宪法后可以自主执行，
不需要我决定，我的决定就是遵守宪法"* (do a precise workflow design, verify against
the constitution, then execute autonomously; the only decision is "obey the
constitution").
**Method**: read-only workflow `canonical-id-hash-design` (3 lens maps of mint
sites / format-parsers / forward-compat + a synthesis), verified in source by the
orchestrator.
**Verdict**: **CLOSE AS MEMBRANE-SATISFIES. Do NOT execute the canonical
content-hash mint reshape.**

## Why (constitution-aligned, source-verified)

1. **Art. 0.2 (Tape-Canonical, `constitution.md:52`) requires reconstructability,
   not content-hash ids.** Legacy human-readable ids (`worktx-…`, `Agent_i`,
   `node_survive:…`) are fully tape-reconstructable, so they already satisfy
   Art. 0.2. The §8 directive's real target — LLM hallucination of guessable ids
   — is the *membrane's* domain.

2. **The membrane (#327 audit + #328 fixes) already closes the LLM-visible
   surface.** Gate `tests/constitution_no_explicit_id_to_llm.rs` (live on
   `origin/main`) proves explicit ids render as opaque `sha256` handles into the
   LLM-visible projection, and agent echoes are validated by exact membership /
   handle (no fuzzy prefix, no `last()` fallback). `src/sdk/id_handle.rs` is wired
   into the live render/resolve seams. The hash-id directive for the LLM
   pass-through ("透传") is **satisfied**.

3. **Every central mint is PINNED, and the id-prefix carries load-bearing
   semantics parsed by a PINNED HALT assertion** (verified pin-counts in
   `genesis_payload.toml`):
   - `src/runtime/bootstrap.rs` (pin 1) — all live `AgentId` preseed mints.
   - `src/runtime/adapter.rs` (pin 1) — the authoritative `TxId` mint helpers
     (`make_real_worktx_signed_by`, `make_real_verifytx_signed_by`, …).
   - `src/runtime/audit_assertions.rs` (pin 1) — `sandbox_prefix()` :883 parses
     `Agent_solver_/verifier_/user_`, feeding the **id=41 HALT** assertion
     `assert_a_chain_agent_ids_sandbox_prefixed` :1019 (every chain-resident
     AgentId MUST be sandbox-prefixed, else `AssertionResult::halt`).
   - `src/bin/audit_dashboard.rs` (pin 1) — role detection at :1076 +
     `sponsor_agent.0.starts_with("Agent_user_")` at :713/:739.
   Hashing the canonical mint therefore = pinned-file edits + a re-signed
   `v4-ratify` tag for **ZERO constitutional gain** (the membrane already shields
   the LLM), and a naive hash WITHOUT dual-form legacy fallback would make **every
   historical chain replay HALT** — a production-breaking regression, not mere
   churn. The `domain_prefixed_digest<T: Serialize>` is content-agnostic, so the
   hashing would *not* break signatures — but it would break the prefix-HALT and
   the role parsers.

4. **Karpathy-surgical + no-unnecessary-trust-root-churn ⇒ don't do it.** A full
   reshape is ~112 files (dominated by test fixtures); the high-value part edits
   PINNED trust-root files for no constitutional benefit. "Obey the constitution"
   here means: do not incur Class-4 trust-root churn that the membrane has already
   made unnecessary.

## What is dropped / deferred

| Item (from the audit) | Disposition | Reason |
|---|---|---|
| Canonical `TxId` hashing (adapter.rs family) | **DROPPED** | prefixes never reparsed (hash-safe) but mint is PINNED → pinned edit for zero gain |
| `AgentId` preseed hashing (bootstrap.rs) | **DEFERRED** | PINNED; reserved sentinels must stay human-readable; no gain beyond membrane |
| `node_survive:` EventId | **DROPPED** | the `node_survive:` namespace prefix IS the runtime market-type discriminant (parsed) — keep it |
| `reputation_constitutional` `Agent_spec_/sybil_` | **DEFERRED** | unpinned, experiment-only, off the live canonical write path |
| Item-B cosmetic mint-form flips (unpinned bins) | **DROPPED** | redundant — membrane already prevents these literals reaching the LLM |

## The one constitution-aligned residual (OPTIONAL, separate §8)

An AGENTS.md §12 hygiene fix — move role/sandbox semantics OUT of the id-prefix
into an explicit structured field, so `audit_assertions::sandbox_prefix` /
`audit_dashboard::detect_sandbox_run` read a structured flag instead of a string
prefix (keeping dual recognition of legacy prefixes, no historical rewrite). Its
high-value part edits the **2 pinned** parsers (`audit_assertions.rs`,
`audit_dashboard.rs`) → that is a **Class-4 atom** and a **DIFFERENT change** than
"canonical-id → content-hash". It is **NOT** pulled in under this token
(`feedback_class4_cannot_hide_in_class3`); if desired it gets its own per-atom §8
with a rehash + re-signed `v4-ratify` tag and a replay gate that goes RED if
legacy-prefix recognition is dropped.

## Net

- Executed: **nothing in `src/`** (no mint reshape).
- Deliverable: this verdict + the filled §8 packet
  `handover/section8/APPROVE_CANONICAL_ID_CONTENT_HASH_HANDLE_2026-06-08.md`.
- The hash-id directive is satisfied by the already-shipped membrane (#328).

`FC-trace: Art. 0.2 (tape reconstructability, not hashing) + Art. III shielding (membrane already shields the LLM) — canonical mint reshape is not constitution-required; executing it would edit PINNED trust-root mints + break the id=41 sandbox-prefix HALT for zero gain, so the constitution-faithful action is to close the atom as membrane-satisfies. No FC node semantics change; no src/ edit.`
