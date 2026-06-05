# TC-012B Canonical G0 AST and Rank

Status: ready
Owner lane: lean-search
Risk class: Class 2 bounded completeness
FC nodes: FC1 search spine
Dependencies: TC-012A
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/g0_completeness.rs`
- `tests/tc_g0_completeness.rs`

Forbidden paths: kernel, bus, market authority, sequencer.

Task:

Replace stringly candidate rank with canonical AST rank.

Locked G0 v0 productions:

- `intro`
- `rfl`
- `assumption`
- `exact_hyp`
- `exact_lemma`
- `apply_lemma`
- `constructor`
- `simp_only_lemmas`

Tests first:

- `g0_parser_accepts_only_locked_productions`
- `g0_canonical_ast_is_whitespace_stable`
- `g0_digest_hashes_ast_not_lean_text`
- `g0_rank_is_pure_over_ast`

Rank rule:

`(ast_size, serialized_len, digest)` converted to stable ordering. Rank must not
read price, LLM score, success history, wall clock, manifest order, or model
identity.

Ship gate:

```bash
cargo test --test tc_g0_completeness --no-fail-fast
```

Expected: command exits 0.

Audit: Formal-methods Auditor.
Verdict: `NO-VIOLATION` or `SECOND-SOURCE-DRIFT rank-input`.
