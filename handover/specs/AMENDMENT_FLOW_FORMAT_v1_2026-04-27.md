# Amendment Flow Format Spec v1 (CO P3-prep.4)

> **Date**: 2026-04-27
> **Purpose**: Plan v3.2 CO P3-prep.4 — define structured format for Constitution Art V.3 amendments. v4 cp-workflow amendments produce parseable records; v4.1 runtime MetaCoordinator ingests them via `MetaTransitionInterface::submit`.
> **Authority**: Constitution Art V.3 (`# 5.3 宪法修订日志`).
> **Status**: v4 deliverable; v4 amendments adopt this format from 2026-04-28 onward; v4.1 runtime consumes it.

---

## § 1 Why a Format

Current Art V.3 modifications (e.g., the 2026-04-26 Art 0–0.4 amendment) are free-form Markdown table rows. v4.1 runtime ArchitectAI/JudgeAI cannot mechanically parse them; bridge code would be needed.

This spec defines a **structured frontmatter + body format** so:
- v4 cp-workflow amendments are machine-parseable from day one
- v4.1 runtime can ingest historical v4 amendments without rewrite
- Audit trail is fully bidirectional: human-readable + machine-verifiable

---

## § 2 File Structure

Every Art V.3 amendment lives at:

```
handover/architect-insights/AMENDMENT_<ISO_DATE>_<SHORT_SLUG>.md
```

Examples:
- `AMENDMENT_2026-04-26_art-0-turing-fundamentalism.md`
- `AMENDMENT_2026-04-27_art-0-2-reading-y.md`
- `AMENDMENT_2026-05-15_d-veto-3-genesis-evolution.md`

The `_<SHORT_SLUG>_` is human-readable; `<ISO_DATE>` is YYYY-MM-DD format.

---

## § 3 Frontmatter (REQUIRED)

```yaml
---
amendment_id: "v4-amend-2026-04-27-art-0-2-reading-y"
amendment_version: 1
constitution_target: "constitution.md"
constitution_hash_before: "<sha256 of constitution.md BEFORE this amendment>"
constitution_hash_after: "<sha256 of constitution.md AFTER this amendment>"
articles_affected: ["Art 0.2"]                  # ordered list
articles_added: []                               # newly created articles
articles_removed: []                             # deleted articles
articles_modified: ["Art 0.2 item 5"]            # modified subsections
proposer:
  type: "ArchitectAI" | "human" | "v4.1-runtime-architect"
  identity: "gretjia | ArchitectAI(Claude-Opus-4.7-1M) | architect_actor_id"
proposer_signature: "<PGP/SSH detached sig of this entire frontmatter+body, hex>"
judges:
  - identity: "Codex (codex-rescue)"
    verdict: "PASS" | "VETO" | "CHALLENGE"
    audit_report_path: "handover/audits/CODEX_*.md"
    signature: "<sig hex>"
  - identity: "Gemini DeepThink (gemini-2.5-pro)"
    verdict: "PASS" | "VETO" | "CHALLENGE"
    audit_report_path: "handover/audits/GEMINI_*.md"
    signature: "<sig hex>"
human_signature_required: true | false             # MUST be true for constitutional changes
human_signature: "<PGP/SSH sig over frontmatter, hex>"  # required iff human_signature_required
amendment_predicate_check:
  amendment_predicate_hash_at_evaluation: "<the predicate hash from genesis_payload.toml>"
  predicate_evaluation_result: "PASS" | "VETO"
trust_root_impact:
  files_to_update_sha:
    - "constitution.md"
    - "<other files transitively affected>"
  new_genesis_payload_required: true | false       # true if [constitution_root] section needs update
ratification_tag: "v4-ratify-<date>-<commit>"      # SSH-signed git tag covering this amendment commit
applied_at_commit: "<git SHA of commit that applies this amendment>"
status: "Draft" | "UnderReview" | "Vetoed" | "HumanReviewPending" | "Approved" | "Applied"
---
```

---

## § 4 Body Sections (REQUIRED)

After frontmatter, body MUST contain these sections in order:

### Body § 1: Trigger / Motivation
1-3 paragraphs explaining what real-world or in-system event triggered this amendment. Cite specific Codex/Gemini audit findings, user requests, V/E violations, or external research.

### Body § 2: Summary of Changes
Concise bullet list of what changes (NEW article / MODIFY article / REMOVE article).

### Body § 3: Diff
Either:
- A unified diff block:
  ```diff
  --- constitution.md (before)
  +++ constitution.md (after)
  @@ ... @@
  -old text
  +new text
  ```
- Or a structured before/after table for surgical changes.

### Body § 4: Rationale (per change)
For each change, explain:
- Why this specific text/structure
- What alternatives were considered + rejected
- Risk/blast-radius assessment

### Body § 5: Conformance Test Impact
For each change, list:
- New conformance tests required → file path + atom
- Existing conformance tests modified → file path + nature of modification
- Existing tests deleted → file path + reason

### Body § 6: Audit Verdicts
Per judge identity in frontmatter:
- Full quote of judge's verdict statement
- Reference to full audit report path
- Reconciliation if judges disagree (apply VETO > CHALLENGE > PASS rule per Protocol)

### Body § 7: Human Signature Statement
If `human_signature_required: true`:
- Plain English statement of what the human is signing (~3 sentences)
- Date of signing
- Method (cp workflow + signed tag)

### Body § 8: Application Procedure
Step-by-step for the cp workflow:
```bash
# 1. Backup
cp constitution.md /tmp/c_before.md

# 2. Apply diff (manual edit)
$EDITOR constitution.md

# 3. Verify diff matches Body § 3
diff /tmp/c_before.md constitution.md

# 4. Update genesis_payload.toml constitution_hash
sha256sum constitution.md
$EDITOR genesis_payload.toml  # update [constitution_root].constitution_hash

# 5. Commit
git add constitution.md genesis_payload.toml handover/architect-insights/AMENDMENT_*.md
git commit -m "amendment: <amendment_id>"

# 6. Sign + tag
git tag -s v4-ratify-2026-XX-XX-<short> -m "Ratify <amendment_id>" HEAD
git push origin v4-ratify-2026-XX-XX-<short>

# 7. Run governance hook
bash scripts/check_tr_ratification_chain.sh
```

---

## § 5 Validator Library Requirements (`amendment_flow_validate`)

A v4 helper:

```rust
// src/governance/amendment_flow_validator.rs (NEW per CO P3-prep.4)
pub fn validate_amendment_file(path: &Path) -> Result<AmendmentValidation, ValidationError> {
    // R1: parse frontmatter as TOML/YAML
    let frontmatter = parse_frontmatter(path)?;

    // R2: required fields present
    require_fields(&frontmatter, &[
        "amendment_id", "constitution_target",
        "constitution_hash_before", "constitution_hash_after",
        "articles_affected", "proposer", "proposer_signature",
        "human_signature_required",
    ])?;

    // R3: human_signature present iff human_signature_required
    if frontmatter.human_signature_required && frontmatter.human_signature.is_none() {
        return Err(ValidationError::MissingHumanSignature);
    }

    // R4: signatures verify against pinned keys
    verify_proposer_signature(&frontmatter)?;
    if let Some(sig) = &frontmatter.human_signature {
        verify_human_signature(sig, &PINNED_CREATOR_PUBKEY)?;
    }

    // R5: ratification_tag exists and verifies
    if let Some(tag) = &frontmatter.ratification_tag {
        verify_git_tag(tag)?;
    }

    // R6: body sections § 1-§ 8 all present
    require_body_sections(path, &[
        "Body § 1", "Body § 2", "Body § 3", "Body § 4",
        "Body § 5", "Body § 6", "Body § 7", "Body § 8",
    ])?;

    // R7: amendment_predicate_check.predicate_evaluation_result == PASS
    if frontmatter.amendment_predicate_check.predicate_evaluation_result != "PASS" {
        return Err(ValidationError::AmendmentPredicateRejected);
    }

    Ok(AmendmentValidation { id: frontmatter.amendment_id, valid: true })
}
```

This validator runs both:
- **Manually** as part of v4 cp-workflow (CI gate before merge)
- **Automatically** in v4.1 runtime when `MetaCoordinator::tick` ingests historical amendments

---

## § 6 Migration of 2026-04-26 Art 0–0.4 Amendment

Current state: 2026-04-26 amendment lives as a single line in `constitution.md` Art V.3 modification table (line ~813). It pre-dates this format spec.

Migration plan (low priority, do at convenience):
1. Create `handover/architect-insights/AMENDMENT_2026-04-26_art-0-turing-fundamentalism.md`
2. Backfill frontmatter (proposer signature missing → mark as "pre-format-spec; legacy")
3. Body sections fillable from existing audit reports
4. NOT requiring a new ratification tag (already covered by user "I authorize" earlier)
5. validator MAY skip pre-format-spec amendments (flag `legacy_pre_format_v1: true`)

This is informational only; migration is NOT blocking any current atom.

---

## § 7 Conformance Tests (3 new in v4)

```
tests/amendment_flow_format_validate.rs
   — validator parses well-formed amendment + rejects malformed
tests/amendment_signature_chain.rs
   — proposer + human + judge signatures all verify against pinned keys
tests/amendment_pre_format_legacy_handling.rs
   — validator correctly skips/flags pre-format amendments
```

---

## § 8 Honest Acknowledgements

What this spec achieves:
- Closes Plan v3.2 CO P3-prep.4 atom (per Gemini Q7 demand)
- v4 amendments from 2026-04-28+ are machine-parseable
- v4.1 runtime can ingest history without bridge

What this spec is honest about:
- 2026-04-26 amendment is grandfathered (pre-format-spec)
- The format is intentionally verbose; for trivial amendments (e.g., typo fix), Body § 4-6 may be brief
- Frontmatter field set may evolve in v4.1 if runtime MetaCoordinator demands more metadata; backward compat will be maintained

What this spec does NOT do:
- Force any past amendments into this format (legacy preserved)
- Enforce on commit (validator is recommended; pre-commit hook is optional R-023 in CLAUDE.md)
- Replace the cp-workflow itself (still constitution.md is the source of truth; this format only structures the AMENDMENT_*.md companion)

— ArchitectAI, 2026-04-27
