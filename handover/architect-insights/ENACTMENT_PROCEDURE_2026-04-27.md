# Enactment Procedure — Art 0.5 + PREREG_v2 + Wave Ratification

> **Date**: 2026-04-27
> **Purpose**: One-page user execution guide for three pending ceremonies. Each is independent (no order required). All commands are concrete and copy-pastable.
> **Authority**: D2=B (Art 0.5 pointer-only), D1=C (PREREG MVP-pivot), B-1 (PGP/SSH-signed tag for TR mutations) — all USER-CONFIRMED per `RATIFICATION_2026-04-27.md`.
> **Status**: Provided for user execution. Does NOT auto-execute (user agency on governance ceremonies).

---

## § 1 Ceremony Order (any can run independently)

```
A.  Ratify auto-research waves (HIGHEST priority — closes governance chain gap)  → ✅ DONE 2026-04-27 (chain through v4-ratify-2026-04-27-ab77097)
B.  Constitution Art 0.5 enactment                                                → ✅ AVAILABLE (WP finalized 2026-04-27)
B'. Art 0.2 line 64 cosmetic edit (Reading Y Option B)                           → ✅ AVAILABLE (WP finalized 2026-04-27)
B''. Boot block field reconciliation                                              → ✅ AVAILABLE (Gemini Top-3 fix #1; recommended FIRST)
C.  PREREG_AMENDMENT_v2 enactment                                                 → ✅ AVAILABLE (research-arc spec)
```

> **✅ Constitution amendment unfreeze (2026-04-27)**: WP finalized via `v4-whitepaper-finalized-2026-04-27-ab77097` SSH-signed tag. All three amendment ceremonies (B / B' / B'') now ELIGIBLE.
>
> **Recommended ceremony order**:
> 1. **B''** (Boot block reconciliation) FIRST — repairs existing drift across 3 sources; per Gemini Top-3 fix #1
> 2. **B'** (Art 0.2 line 64 cosmetic edit) — small surgical fix; aligns text with already-implemented spec Reading Y
> 3. **B** (Art 0.5 new content insertion) — adds white paper integration via 6 axioms; requires careful review
>
> Each ceremony is independent; user picks order. Ratify each via separate signed tag.

---

## § 2 Ceremony A — Ratify auto-research wave 1+2+3

### A.1 Verify current state

```bash
cd /home/zephryj/projects/turingosv4
bash scripts/check_tr_ratification_chain.sh
```

Expected output:
```
=== TR Mutation Ratification Chain Report ===
Range: 0dd0d35..HEAD
Total TR mutations: 8 (or however many)
Ratified: 5
Unratified: 3 (c6dd122, ae11491, 57457e5 + this commit if shipped)

⚠ UNRATIFIED:
  c6dd122  (auto-research: TRACE_MATRIX_v3 + ...)
  ae11491  (auto-research wave 2: TR governance hook + ...)
  57457e5  (auto-research wave 3: spec walk-through + ...)
```

### A.2 Sign + push the merge tag

```bash
# Get current HEAD commit short SHA (the wave-4 final commit)
HEAD_SHORT=$(git rev-parse --short HEAD)

# Sign one tag covering ALL waves
git tag -s "v4-ratify-2026-04-27-${HEAD_SHORT}" \
  -m "Ratify auto-research waves 1-4 (10+ docs / 4000+ lines doc-only / TR 58 → ~70; all within ratified Plan v3.2-fix1 scope. Wave 1: TRACE_MATRIX + gix preflight + MetaTransitionInterface. Wave 2: TR governance hook + AmendmentFlow + V4.1 plan + TLA+ skeleton. Wave 3: spec walk-through + sprint dep graph + R-022/R-023 hooks. Wave 4: STATE_TRANSITION_SPEC v1.1 patches + INV8 DAG spike pre-draft + this enactment doc.)" \
  HEAD

# Push tag
git push origin "v4-ratify-2026-04-27-${HEAD_SHORT}"

# Verify
git verify-tag "v4-ratify-2026-04-27-${HEAD_SHORT}"
# Expected: Good "git" signature for gretjia@users.noreply.github.com with ED25519 key SHA256:GreuFZEkNxBHp5mf0Er/T5EFQ9pr9IFpfe+usJJqOTc

# Re-verify governance chain
bash scripts/check_tr_ratification_chain.sh
# Expected: Unratified: 0
```

That's it. Ceremony A done in ~30 seconds.

---

## § 3 Ceremony B — Constitution Art 0.5 Enactment

### B.1 What this enacts

Inserts Art 0.5 (white paper integration + 6 axioms) into `constitution.md` per `CONSTITUTION_ART_0_5_DRAFT_2026-04-26.md`. Per D2=B (ratified), the format is **pointer + 6 axioms** (~150 lines added to constitution.md), not full white paper text.

### B.2 cp Workflow (R-018 hook bypass per V.3 entry)

```bash
cd /home/zephryj/projects/turingosv4

# Backup
cp constitution.md /tmp/c_before.md

# Open in editor; insert Art 0.5 text after Art 0.4 closing paragraph (around line 155)
# Source text is in handover/architect-insights/CONSTITUTION_ART_0_5_DRAFT_2026-04-26.md § "Proposed Text"
# Insert the entire markdown block under "## Proposed Text (to be inserted into constitution.md)"
$EDITOR constitution.md
# (alternatively: Cursor / VS Code; manually copy-paste the proposed text)

# Verify diff matches expected
diff /tmp/c_before.md constitution.md | head -200

# Update constitution Art V.3 modification log table — add a new row:
# | 2026-04-27 | gretjia (人类架构师) | **Art. 0.5 (新增)** | <one-paragraph summary> | <triggers + amendment metadata> |
# (See AMENDMENT_FLOW_FORMAT_v1 for the structured frontmatter format if desired)
$EDITOR constitution.md

# Recompute SHA + update genesis_payload.toml
NEW_HASH=$(sha256sum constitution.md | awk '{print $1}')
echo "New constitution.md SHA: $NEW_HASH"
# Edit genesis_payload.toml: update [trust_root]."constitution.md" to NEW_HASH
$EDITOR genesis_payload.toml

# Verify boot tests still pass
cargo test --lib boot
# Expected: 8 passed

# Commit + sign
git add constitution.md genesis_payload.toml
git commit -m "$(cat <<'EOF'
constitution Art 0.5 enactment: pointer + 6 axioms (D2=B)

Per ratified D2=B (RATIFICATION_2026-04-27.md § 1.3), enacts Art 0.5 as
pointer-with-6-axioms format. Full white paper text remains in
handover/whitepapers/. The 6 axioms (Anti-Oreo / Tape Canonical / Goodhart
shield / signal dichotomy / predicate-gated transition / escrow-only reward)
are content-anchored to existing code symbols.

Source: handover/architect-insights/CONSTITUTION_ART_0_5_DRAFT_2026-04-26.md
Authority: Art V.3 cp workflow + this human signature

constitution.md SHA: <NEW_HASH>
EOF
)"

# Sign tag
HEAD_SHORT=$(git rev-parse --short HEAD)
git tag -s "v4-amend-2026-04-27-art-0-5-${HEAD_SHORT}" \
  -m "Enact Constitution Art 0.5 (pointer + 6 axioms per D2=B). Replaces draft. constitution.md SHA: $NEW_HASH" \
  HEAD
git push origin main "v4-amend-2026-04-27-art-0-5-${HEAD_SHORT}"
```

Ceremony B duration: 5-15 minutes (manual editing).

### B.3 Optional: convert DRAFT to AmendmentFlow format

After enactment, mark `CONSTITUTION_ART_0_5_DRAFT_2026-04-26.md` as superseded:
```bash
# Add header: "STATUS: ENACTED 2026-04-27 via cp workflow. See AMENDMENT_2026-04-27_art-0-5-pointer-6-axioms.md"
$EDITOR handover/architect-insights/CONSTITUTION_ART_0_5_DRAFT_2026-04-26.md

# Optional: create AmendmentFlow-format companion doc per AMENDMENT_FLOW_FORMAT_v1
$EDITOR handover/architect-insights/AMENDMENT_2026-04-27_art-0-5-pointer-6-axioms.md
```

---

## § 4 Ceremony C — PREREG_AMENDMENT_v2 Enactment

### C.1 What this enacts

Locks PPUT-CCL arc state per D1=C (MVP-pivot). The DRAFT `PREREG_AMENDMENT_v2_2026-04-26.md` is already in repo + Trust-Rooted; enactment is "remove DRAFT marker + update LATEST.md".

### C.2 Procedure

```bash
cd /home/zephryj/projects/turingosv4

# Edit PREREG_AMENDMENT_v2 to mark as enacted:
# - Remove "DRAFT awaiting user formal sudo on wake" line
# - Add "ENACTED 2026-04-27" + signed tag reference
$EDITOR handover/preregistration/PREREG_AMENDMENT_v2_2026-04-26.md

# Update LATEST.md to reflect arc state SUSPENDED (per PREREG v2 § 2):
# - Phase A: results PRESERVED but not used inferentially
# - Phase B: results PRESERVED but not used inferentially
# - Phase C: SUSPENDED; restart gate = CO P1.14 exit (10+ wk from now)
# - Phase D: NOT STARTED, deferred
# - Phase E: heldout-54 reservation HONORED
$EDITOR handover/ai-direct/LATEST.md

# Recompute PREREG_AMENDMENT_v2 SHA + update genesis_payload.toml
NEW_PREREG_SHA=$(sha256sum handover/preregistration/PREREG_AMENDMENT_v2_2026-04-26.md | awk '{print $1}')
$EDITOR genesis_payload.toml  # update [trust_root] entry for PREREG_AMENDMENT_v2 (it's NOT trust-rooted yet — this enactment makes it so)

# Verify
cargo test --lib boot

# Commit + sign
git add handover/preregistration/PREREG_AMENDMENT_v2_2026-04-26.md handover/ai-direct/LATEST.md genesis_payload.toml
git commit -m "$(cat <<'EOF'
PREREG_AMENDMENT_v2 enactment: D1=C MVP-pivot

Per ratified D1=C (RATIFICATION_2026-04-27.md § 1.3), formally enacts the
MVP-pivot arc state. Phase A+B preserved non-inferentially; Phase C SUSPENDED
until CO P1.14 exit; Phase D deferred; heldout-54 reservation HONORED.

PREREG amendment v2 now Trust-Rooted (was DRAFT-only).
EOF
)"

HEAD_SHORT=$(git rev-parse --short HEAD)
git tag -s "v4-prereg-2026-04-27-v2-${HEAD_SHORT}" \
  -m "Enact PREREG_AMENDMENT_v2 (D1=C MVP-pivot per ratification)" \
  HEAD
git push origin main "v4-prereg-2026-04-27-v2-${HEAD_SHORT}"
```

Ceremony C duration: 3-5 minutes.

---

## § 5 Post-Ceremony Verification

After any ceremony:

```bash
# 1. Governance chain intact
bash scripts/check_tr_ratification_chain.sh
# Expected: 0 unratified

# 2. Boot tests pass
cargo test --lib boot
# Expected: 8 passed

# 3. AUDIT_LEDGER updated (Claude can do this in next session; manual entry OK too)
$EDITOR handover/audits/AUDIT_LEDGER.md
# Add row: timestamp, ceremony name, signer, verification output, cost ($0)

# 4. List all signed tags
git tag --list 'v4-*' | sort
# Expected progression:
#   v4-ratify-2026-04-27-b6b6c25     (initial ratification)
#   v4-ratify-2026-04-27-<wave4>      (Ceremony A)
#   v4-amend-2026-04-27-art-0-5-X    (Ceremony B, if done)
#   v4-prereg-2026-04-27-v2-X        (Ceremony C, if done)
```

---

## § 6 What If Something Goes Wrong

### 6.1 If `git tag -s` says "no signing key configured"

Already configured at repo scope (per ratification setup). If config got reset:
```bash
git config user.signingkey "$HOME/.ssh/id_ed25519_github_omega_vm.pub"
git config gpg.format ssh
git config gpg.ssh.allowedSignersFile "$HOME/.config/git/allowed_signers"
```

### 6.2 If boot tests fail after constitution change

Most likely: constitution.md SHA in `genesis_payload.toml` doesn't match new file. Re-compute:
```bash
sha256sum constitution.md
# Update genesis_payload.toml [trust_root]."constitution.md" to match
```

### 6.3 If you want to revert ceremony

For Ceremony A (tag): tags can be deleted but signed history is on origin/main; consider `git tag -d <tag>` locally + `git push --delete origin <tag>` to remove.

For Ceremony B (constitution): `git revert <commit>` reverts the constitutional change. AUDIT_LEDGER must record the revert.

For Ceremony C (PREREG): same as B.

### 6.4 If ArchitectAI guidance is unclear

Open a question; user is the final authority for any ceremony interpretation. Auto-research operates on default recommendations only when explicitly told to.

---

## § 7 Honest Acknowledgements

What this guide achieves:
- Reduces 3 ceremonies to copy-pasteable command blocks
- Lists exact verification expected after each ceremony
- Provides recovery procedures if something goes wrong

What this guide is honest about:
- Manual editor steps are unavoidable for cp workflow (per Art V.3)
- Tag fingerprint verification depends on user's SSH key still being valid
- Boot tests pass relies on no other concurrent changes; if other changes interleave, manual reconciliation may be needed

What this guide does NOT do:
- Auto-execute any ceremony (governance is human authority)
- Generate the full Constitution Art 0.5 text (it's already in DRAFT)
- Generate the full PREREG_v2 text (already in DRAFT)

— ArchitectAI, 2026-04-27 enactment guide
