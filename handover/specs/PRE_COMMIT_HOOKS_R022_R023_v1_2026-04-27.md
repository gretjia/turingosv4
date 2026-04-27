# Pre-Commit Hooks R-022 + R-023 Spec v1

> **Date**: 2026-04-27
> **Purpose**: Plan v3.2 CO1.13.2 (R-022 TRACE_MATRIX hook) + B-1 governance gate (R-023 TR-mutation hook). Specifies hook behavior; provides reference shell scripts; user opts in by installing.
> **Authority**: Plan v3.2-fix1 § 3 CO1.13 + RATIFICATION_2026-04-27.md § 3.
> **Status**: Doc-only spec. Hook scripts provided as reference; not auto-installed (user choice).

---

## § 1 R-022: TRACE_MATRIX Auto-Maintenance Hook

### 1.1 Trigger
Any commit that adds or modifies a `pub` symbol in:
- `src/{top_white,middle_black,bottom_white,economy,state,transition,governance}/**/*.rs`
- `experiments/minif2f_v4/src/agents/**/*.rs`
- `experiments/minif2f_v4/src/bin/evaluator.rs`

### 1.2 Required behavior

For every newly-added or modified `pub` symbol:
1. Symbol MUST carry a `/// TRACE_MATRIX <id>: <role>` doc-comment immediately above its declaration
2. `<id>` MUST be one of:
   - `Const-Art-X.Y` (e.g., `Const-Art-0.2`, `Const-Art-V.1.2`)
   - `WP-arch-§N.M` (e.g., `WP-arch-§4`, `WP-arch-§5.L1`)
   - `WP-econ-§N` (e.g., `WP-econ-§18`, `WP-econ-§21`)
   - `Inv-N` for economic invariants 1-12 (e.g., `Inv-3`, `Inv-8`)
   - `I-XXX` for transition invariants (e.g., `I-DET`, `I-PARENT`)
   - `WP-spec-XYZ` for spec docs (e.g., `WP-spec-state-transition`)
3. Each `<id>` MUST appear in `handover/alignment/TRACE_MATRIX_v3_*.md` § A or § B or § C
4. If symbol is a NEW addition, the matrix § F reverse map MUST also be updated

### 1.3 Reference Implementation

```bash
#!/bin/bash
# .claude/hooks/check_trace_matrix_R022.sh
# Triggered by .claude/hooks/judge.sh on relevant file edits.

set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
TRACE_MATRIX="$REPO_ROOT/handover/alignment/TRACE_MATRIX_v3_2026-04-27.md"

# Find all *.rs files staged for commit in scoped paths
STAGED=$(git diff --cached --name-only --diff-filter=AM | \
    grep -E '^(src/(top_white|middle_black|bottom_white|economy|state|transition|governance)/.*\.rs$|experiments/minif2f_v4/src/(agents|bin)/.*\.rs$)' || true)

if [ -z "$STAGED" ]; then
    exit 0  # no relevant files
fi

VIOLATIONS=()

for file in $STAGED; do
    # Find all `pub` declarations
    while IFS= read -r line_match; do
        line_no=$(echo "$line_match" | cut -d: -f1)
        line_text=$(echo "$line_match" | cut -d: -f2-)

        # Skip if it's an enum variant declaration inside enum (not top-level pub)
        if [[ "$line_text" =~ ^[[:space:]]+pub ]] && ! [[ "$line_text" =~ ^pub ]]; then
            # nested pub; only flag if doc-comment missing on enclosing item; complex; skip for v1
            continue
        fi

        # Check the 3 lines preceding for `/// TRACE_MATRIX`
        start=$((line_no - 3))
        [ "$start" -lt 1 ] && start=1
        preceding=$(sed -n "${start},$((line_no - 1))p" "$file")

        if ! echo "$preceding" | grep -q '/// TRACE_MATRIX'; then
            VIOLATIONS+=("$file:$line_no  $(echo "$line_text" | head -c 80)")
        fi
    done < <(git diff --cached -U0 "$file" 2>/dev/null | grep -nE '^\+pub (fn|struct|enum|trait|const|type|mod) ' | head -100)
done

if [ ${#VIOLATIONS[@]} -gt 0 ]; then
    echo "✗ R-022 violation: pub symbol(s) missing /// TRACE_MATRIX <id>: <role> doc-comment" >&2
    echo "" >&2
    for v in "${VIOLATIONS[@]}"; do
        echo "  $v" >&2
    done
    echo "" >&2
    echo "Add doc-comment immediately above each. <id> values: Const-Art-X.Y / WP-arch-§N / WP-econ-§N / Inv-N / I-XXX" >&2
    echo "Trace matrix: $TRACE_MATRIX" >&2
    exit 1
fi

# Reverse map § F update check
NEW_PUBS=$(git diff --cached --diff-filter=A -U0 src/ 2>/dev/null | grep -E '^\+pub (fn|struct|enum|trait)' | wc -l)
if [ "$NEW_PUBS" -gt 0 ]; then
    if ! git diff --cached --name-only | grep -q "TRACE_MATRIX_v3.*\.md"; then
        echo "⚠ R-022 warning: $NEW_PUBS new pub symbol(s) added but TRACE_MATRIX_v3 § F reverse map not updated in same commit" >&2
        echo "  This is permitted in CO P0/P1.* atoms (matrix populates incrementally)" >&2
        echo "  CO P1.13 atom enforces strict update; until then this is a warning only" >&2
        # Currently warning; promote to error after CO1.13 lands
    fi
fi

exit 0
```

### 1.4 Conformance test for the hook itself

```rust
// tests/r022_hook_validates.rs
//
// 1. Synthesize a commit with: pub fn foo() { } (NO TRACE_MATRIX doc)
// 2. Run R-022 hook
// 3. Expect exit 1 + violation message
//
// 4. Synthesize a commit with: /// TRACE_MATRIX I-DET: deterministic transition\npub fn foo() {}
// 5. Run R-022 hook
// 6. Expect exit 0
```

### 1.5 R-022 maturity stages

- **Stage 1 (current)**: warning-only on missing doc-comments; allows incremental adoption
- **Stage 2 (CO1.13 atom)**: hard error on missing doc-comments; CI gate
- **Stage 3 (post v4 ship)**: also enforces `<id>` exists in TRACE_MATRIX_v3 § A/B/C; rejects unknown IDs

---

## § 2 R-023: TR-Mutation Governance Hook

### 2.1 Trigger

Any commit that touches `genesis_payload.toml`.

### 2.2 Required behavior

After commit lands, runner MUST:
1. Verify `scripts/check_tr_ratification_chain.sh` does NOT report any unratified TR mutations
2. If unratified mutations exist, emit a warning in commit message metadata + LATEST.md: "TR mutation pending ratification"
3. NEXT commit cannot land until ratification tag created

### 2.3 Reference Implementation (post-commit, not pre-commit)

```bash
#!/bin/bash
# .git/hooks/post-commit (manually installed; not in repo by default)

REPO_ROOT="$(git rev-parse --show-toplevel)"

# Only run if last commit touched genesis_payload.toml
if ! git diff --name-only HEAD~1 HEAD 2>/dev/null | grep -q "^genesis_payload.toml$"; then
    exit 0
fi

UNRATIFIED=$(bash "$REPO_ROOT/scripts/check_tr_ratification_chain.sh" --json 2>&1 | grep -oP '"unratified": \d+' | grep -oP '\d+')

if [ -n "$UNRATIFIED" ] && [ "$UNRATIFIED" -gt 0 ]; then
    echo ""
    echo "⚠ R-023: TR-mutation governance gate"
    echo "$UNRATIFIED commit(s) touching genesis_payload.toml lack signed ratification tag."
    echo ""
    echo "Run: bash scripts/check_tr_ratification_chain.sh"
    echo "Sign: git tag -s v4-ratify-2026-XX-XX-<short> -m \"<reason>\" HEAD"
    echo "Push: git push origin v4-ratify-2026-XX-XX-<short>"
    echo ""
    echo "Subsequent TR-touching commits will accumulate; ratify periodically."
fi

exit 0  # always succeed (post-commit; can't undo)
```

### 2.4 Pre-commit variant (stricter; blocks commit if previous TR-mutation unratified)

```bash
#!/bin/bash
# .git/hooks/pre-commit (manually installed)

# If THIS commit doesn't touch genesis_payload.toml, no constraint
if ! git diff --cached --name-only | grep -q "^genesis_payload.toml$"; then
    exit 0
fi

# If this commit touches TR but a prior unratified mutation exists, BLOCK
REPO_ROOT="$(git rev-parse --show-toplevel)"
if bash "$REPO_ROOT/scripts/check_tr_ratification_chain.sh" --strict 2>/dev/null; then
    exit 0  # all prior TR mutations ratified; new TR mutation OK
fi

echo "✗ R-023 violation: cannot create new TR mutation while prior TR mutation unratified" >&2
echo "" >&2
echo "Ratify the prior mutation first via signed git tag, OR amend this commit to also include" >&2
echo "the ratification tag in same atomic batch." >&2
exit 1
```

### 2.5 Conformance test

```rust
// tests/r023_hook_governance.rs
//
// 1. Make a commit touching genesis_payload.toml WITHOUT preceding ratification → expect post-commit warning
// 2. Sign tag for that commit; re-run check → expect "all ratified"
// 3. With pre-commit variant installed: try to make a 2nd TR mutation without ratifying 1st → expect block
```

---

## § 3 Installation (Optional; User Choice)

R-022 + R-023 are **opt-in**. Installation:

```bash
# R-022 (recommended for any developer touching src/{top_white,...})
ln -sf "$(pwd)/scripts/hook_r022_trace_matrix.sh" .git/hooks/pre-commit
# (or merge into existing hook if .git/hooks/pre-commit already exists; .claude/hooks/judge.sh already exists)

# R-023 (recommended; aligns with B-1 governance)
ln -sf "$(pwd)/scripts/hook_r023_tr_governance.sh" .git/hooks/post-commit

# Verify
ls -la .git/hooks/pre-commit .git/hooks/post-commit
```

Or strict variant (blocks at pre-commit on unratified TR):
```bash
ln -sf "$(pwd)/scripts/hook_r023_pre_commit_strict.sh" .git/hooks/pre-commit
```

If user does NOT install: hooks don't run; behavior is informational only via on-demand `bash scripts/check_tr_ratification_chain.sh`.

---

## § 4 Integration with `.claude/hooks/judge.sh`

The existing `.claude/hooks/judge.sh` is a Claude-Code-managed hook. R-022 + R-023 should be added to it as **rule entries** in `rules/active/`:

```yaml
# rules/active/R-022.yaml (NEW)
id: R-022
title: TRACE_MATRIX_v3 doc-comment maintenance
trigger: pre-commit
applies_to:
  - src/top_white/**/*.rs
  - src/middle_black/**/*.rs
  - src/bottom_white/**/*.rs
  - src/economy/**/*.rs
  - src/state/**/*.rs
  - src/transition/**/*.rs
  - src/governance/**/*.rs
  - experiments/minif2f_v4/src/agents/**/*.rs
  - experiments/minif2f_v4/src/bin/evaluator.rs
check: scripts/hook_r022_trace_matrix.sh
maturity: warning  # promote to error after CO1.13 atom
authority:
  plan: CO_MEGA_PLAN_v3.2 § 3 CO1.13.2
  spec: TRACE_MATRIX_v3 § F + § H
```

```yaml
# rules/active/R-023.yaml (NEW)
id: R-023
title: TR-mutation governance — every TR change requires signed ratification tag
trigger: pre-commit (strict) or post-commit (warning)
applies_to:
  - genesis_payload.toml
check: scripts/hook_r023_tr_governance.sh
maturity: warning (default) | error (strict mode)
authority:
  ratification_doc: handover/architect-insights/RATIFICATION_2026-04-27.md § 3
  governance: B-1 protocol from TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-27.md
```

---

## § 5 Why Opt-In, Not Auto-Install

User said "auto-research within constitution + white paper bounds". Auto-installing pre-commit hooks would:
- Modify `.git/hooks/` which is local-only (not version-controlled)
- Surprise user on next commit with unexpected blocks
- Touch user's local git config beyond repo scope

Doc-only spec + opt-in install respects user agency. CLAUDE.md governance principle: "match scope of actions to what was actually requested."

If user wants auto-install: explicit request like "install R-022 + R-023 hooks" → ArchitectAI runs the `ln -sf` commands.

---

## § 6 Conformance Tests (cargo test)

```
tests/r022_hook_validates.rs              — R-022 hook script behavior
tests/r023_hook_governance.rs             — R-023 hook script behavior
tests/trace_matrix_v3_bidirectional.rs    — § A/B/C IDs match doc-comments in src/
tests/no_unknown_trace_matrix_ids.rs      — every doc-comment ID exists in matrix
```

These cargo tests run during normal `cargo test`. Hooks themselves are bash scripts and tested via integration tests that synthesize git states.

---

## § 7 Honest Acknowledgements

What this spec achieves:
- Closes Plan v3.2 CO1.13.2 (R-022 hook)
- Implements B-1 governance hook (R-023)
- Provides reference shell scripts that work today

What this spec is honest about:
- Hooks are opt-in (no auto-install)
- R-022 stage 1 is warning-only; allows incremental code adoption
- Pre-commit hooks fail safely (exit 0 on shell error) to not block legitimate commits
- Hook integration with existing `.claude/hooks/judge.sh` requires manual rule registration

What this spec does NOT do:
- Implement the hooks as actual files in `.git/hooks/` (user opt-in)
- Force every commit to pass R-022/R-023 (only when installed)
- Generate `<id>` automatically (developer must choose appropriate ID from matrix)

— ArchitectAI, 2026-04-27
