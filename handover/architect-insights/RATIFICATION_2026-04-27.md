# User Ratification Record — 2026-04-27 v3.2-fix1 Bundle

> **Date**: 2026-04-27
> **Ratifying user**: gretjia (gretjia@users.noreply.github.com)
> **Ratifying commit**: `b59145d` (HEAD of origin/main at ratification time)
> **Signing method**: SSH-signed git tag (`v4-ratify-2026-04-27-b59145d`) using ed25519 key `omega-vm-github-2026-02-23`
> **Signer fingerprint**: `SHA256:GreuFZEkNxBHp5mf0Er/T5EFQ9pr9IFpfe+usJJqOTc`
> **Verification command**: `git verify-tag v4-ratify-2026-04-27-b59145d`
> **B-1 governance gate**: per `TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-27.md` Hard rule + Codex T+S §A B-1 PASS.

---

## § 1 What this ratification covers

User explicitly accepts, by signing the git tag covering commit `b59145d`, the following items as Plan v3.2-fix1 final state:

### 1.1 Spec documents (5)

- `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md` (sha `d5a19f02...`)
- `handover/specs/GENESIS_MINIMAL_WITH_ANCHOR_v1_2026-04-27.md` (sha `fc82ef00...`)
- `handover/specs/ART_0_2_REINTERPRETATION_2026-04-27.md` (sha `6f869c0b...`)
- `handover/specs/SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md` (sha `3dcaba85...`)
- `handover/specs/META_TX_SCHEMA_v1_2026-04-27.md` (sha `d614b058...`)

### 1.2 Plan v3.2 + audits

- `handover/architect-insights/CO_MEGA_PLAN_v3.2_2026-04-27.md` (sha `b28bcc8a...`)
- `handover/audits/CODEX_T_S_REVIEW_2026-04-27.md` (sha `249309ac...`)
- `handover/audits/GEMINI_V32_REVIEW_2026-04-27.md` (sha `73f940b1...`)

### 1.3 Decision resolutions (formerly PROVISIONAL → now USER-CONFIRMED)

| # | Item | Resolution |
|---|---|---|
| **D1** | PREREG / PPUT-CCL fate | **C — MVP-pivot (deferred until L6/error/cost tape schema settled in CO1.7+CO1.9)** |
| **D2** | Constitution Art. 0.5 | **B — pointer + 6 公理** |
| **D3** | TFR v1 disposition | **A — deprecate, preserve content** |
| **D4** | CO P3 MetaTape | **B — defer to v4.1 + v4 ships CO P3-PREP 7 concrete artifacts** (NOT D permanent abandon) |
| **D5** | RSP depth | **A — full RSP** + i64 micro-coin prerequisite (CO P2.0a) |
| **D6** | External audit cadence | **A — full per phase + per STEP_B atom** |
| **D-VETO-1** | bus.rs/kernel.rs split | **D + binding form** — spec-first via `STATE_TRANSITION_SPEC_v1` (typed schemas + 20 named invariants + deterministic pseudocode + 5 transition functions); STEP_B branches compared on spec conformance |
| **D-VETO-2** | monetary type | **i64 micro-coin** (10⁻⁶ unit) |
| **D-VETO-3** | genesis schema | **D revised — 8 fields with content-hash anchors** (per `GENESIS_MINIMAL_WITH_ANCHOR_v1`) |
| **D-VETO-4** | runtime MetaTape | **B — defer v4.1, ship Phase 3 prep artifacts in v4** (per `META_TX_SCHEMA_v1` + 7-atom CO P3-PREP track) |
| **D-VETO-5** | TRACE_MATRIX_v3 expansion | **A + N/M/D classification** |
| **D-VETO-6** | rejection on tape | **B + system-stamped retry metadata + TerminalSummaryTx** (NOT agent self-report) |
| **D-VETO-7** | bus.rs:268 completion_tokens | **A — pre-split atom CO1.1.4-pre1** (single ceremonial commit) |
| **B-1** | TR mutation governance | **PASS — SSH-signed git tag** (this document is part of what's signed) |
| **Art 0.2 reinterpretation** | reading X vs Y | **Option B — cosmetic edit** (per `ART_0_2_REINTERPRETATION_2026-04-27.md`); to be enacted via cp workflow on next constitution amendment cycle |
| **System keypair security** | spec approval | **APPROVED as-is** (ed25519 + Argon2id KDF + ChaCha20-Poly1305 encrypted-at-rest + epoch rotation) |
| **CO P3-PREP 7 atoms** | concrete artifacts | **APPROVED 7-atom track** (MetaTx schema / MetaProposalDraft CAS / meta_validator library / amendment_flow_format / MetaTransitionInterface trait / V4_1_METATAPE_PLAN / meta_validator conformance test) |
| **Cost cap** | $890 mid-budget ($580-1200 range) | **APPROVED**; weekly burn check at 80% threshold |

### 1.4 What is NOT ratified by this signature

- Constitution Art. 0.5 enactment text (`CONSTITUTION_ART_0_5_DRAFT_2026-04-26.md`) — separate cp workflow + separate signing required at enactment time
- PREREG_AMENDMENT_v2 enactment (`PREREG_AMENDMENT_v2_2026-04-26.md`) — separate enactment ceremony
- Art 0.2 line 64 cosmetic edit — separate constitution amendment cycle (Option B is APPROVED but enactment is its own future ceremony)
- Future TR mutations — each future TR change requires its own signed tag per B-1 governance

---

## § 2 What CO P1 entry now requires

After this ratification:

1. ✅ Plan v3.2-fix1 ratified
2. ⏳ Constitution Art. 0.5 enactment (cp workflow + new signed tag) — can run anytime; not blocking CO P1
3. ⏳ PREREG_AMENDMENT_v2 enactment — same; not blocking CO P1
4. ⏳ CO0.8 TRACE_MATRIX_v3 atom (3-5 days; doc-only)
5. ⏳ CO0.9 META_TX_SCHEMA atom — DONE (this commit)
6. ⏳ CO1.SPEC.0 spec gate dual audit — Codex T+S done; Gemini v3.2 done; spec already addresses both audit feedback rounds → spec is at v1 frozen
7. ⏳ CO1.3.1 gix substrate spike (5-day time-box) — FIRST CO P1 atom

CO P1 entry GO/NOGO is at user's discretion after items 4 + 6 finish a final dual sign-off.

---

## § 3 Ratification verification procedure (for future readers)

To verify this ratification is genuine:

```bash
# 1. fetch the tag
git fetch origin --tags

# 2. inspect the tag
git show v4-ratify-2026-04-27-b59145d

# 3. verify SSH signature
git verify-tag v4-ratify-2026-04-27-b59145d

# 4. confirm signer
ssh-keygen -lf ~/.ssh/id_ed25519_github_omega_vm.pub
# expected: SHA256:GreuFZEkNxBHp5mf0Er/T5EFQ9pr9IFpfe+usJJqOTc omega-vm-github-2026-02-23 (ED25519)

# 5. verify covered commit
git rev-parse v4-ratify-2026-04-27-b59145d^{commit}
# expected: <commit b59145d's full SHA when this doc is committed>
```

---

## § 4 If ratification needs to be revoked

User can revoke by:

1. Creating a new SSH-signed tag `v4-revoke-ratify-2026-04-27-b59145d` covering a "REVOCATION" commit
2. The revocation commit MUST clearly identify what is being revoked (specific D-decisions / specific specs)
3. AUDIT_LEDGER row added with revocation tag fingerprint + verification
4. Plan v3.3 patch documents what changes as a result

This revocation procedure is recorded so that "I changed my mind" remains a clean cryptographic event, not just a chat utterance.

---

## § 5 Trust Root note

This file (`RATIFICATION_2026-04-27.md`) WILL be added to `genesis_payload.toml` `[trust_root]` in the same commit that introduces it. The git tag covers the commit (and thus the file in TR). Self-referential integrity: tampering with this file requires generating a new SHA, requires updating TR, requires a new ratification tag. No silent revisions possible.

— Recorded for ratification, awaiting user SSH-tag signature
