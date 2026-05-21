# TuringOS v4 — Handover State

> 🚨 **AGENT COLD-START — READ FIRST** 🚨
>
> Before reading this file, every coding-agent CLI MUST first read:
>
> 1. **`AGENTS.md`** (canonical universal agent contract)
> 2. **`HARNESS_PLAYBOOK.md`** (full operating manual — L1-L9, K-HARDEN 1-8)
> 3. **`skills/SUBAGENT_HARNESS.md`** (subagent dispatch template)
>
> Hard rules enforced mechanically: **PR-only workflow** · no `git push origin main`
> · no `git add .` · no sidecar staging. See `AGENTS.md` §14a.
>
> This file (LATEST.md) is a **derived view** of current session state — it is
> NOT a source of truth. If LATEST.md contradicts ChainTape/CAS, ChainTape wins.

---

> 📍 **PROJECT DECISION MAP**: `handover/architect-insights/PROJECT_DECISION_MAP_2026-04-27.md`
>
> **ARCHIVE**: Sessions #1–#54 (2026-04-14 → 2026-05-17) archived at
> `handover/ai-direct/LATEST_ARCHIVE_PRE_2026-05-20_sessions_1_to_54.md`
> (8415 lines). Read it if cold-starting on pre-P7.z history.
>
> ChainTape / CAS wins over any derived view including this file.

---

## 📍 Handover summary (session #55 close 2026-05-20)

**Session type**: Architecture + charter drafting (no code shipped; no atoms dispatched).

**Session summary**: Produced the **V4 Product-CAK Hardening charter** (12 atoms
C0..C11, phase P7.z), audited it with 3 opus reviewers (v4-Karpathy, v5-reuse-port,
atom-rigor), folded all findings into plan v2, and wrote two handover artifacts
for Gemini CLI orchestrator dispatch.

---

### Current state of `main` (unchanged from session #54)

> 📍 **Reality Seal**: [V4_PRODUCT_BASELINE_REALITY_SEAL.md](../../docs/roadmap/V4_PRODUCT_BASELINE_REALITY_SEAL.md)

`main` carries PR #3, #4, #5, #6, #7, #8, #10, #11. PR #1, #2, #9 closed without merge.

**Works on main** (code unchanged from session #54 / PR #11):
- `turingos` CLI: `init / agent / task / audit / report / verify / render / welcome / llm / spec / generate`
- `turingos_web`: 20 axum routes including `/welcome /build /api/spec/{questions,submit,turn} /api/generate /api/artifact/:session_id/:name /ws`
- `SpecCapsule` CAS-backed via `ObjectType::EvidenceCapsule` + `schema_id = turingos-spec-capsule-v1`
- Spec grill: 8-question Chinese-first LLM-driven interview → `spec.md` + `spec_transcript.jsonl` + CAS EvidenceCapsule
- Code generation: `turingos generate` → Blackbox LLM → `artifacts/` (filesystem; **No CAS write** — this is the gap C2/C3 close)
- Artifact serve: path-traversal triple-defended at `src/web/artifact.rs:83-115`

**Remaining gap** (the charter's raison d'être):
`cmd_generate.rs:18-23` comment: *"Class 1: filesystem write to /artifacts/. No CAS write."*

---

### Session #55 deliverables (handover artifacts; no code change)

| File | Description |
|------|-------------|
| `handover/architect-insights/V4_PRODUCT_CAK_HARDENING_EXECUTION_PLAN_2026-05-20.md` | Master execution plan — 12 atoms C0..C11 (1516 lines, 82 KB) |
| `handover/architect-insights/GEMINI_ORCHESTRATOR_BOOT_2026-05-20.md` | Boot prompt for Gemini CLI orchestrator (326 lines) |

---

### Charter summary

**Phase**: P7.z — Product-CAK Hardening

**Goal**: Harden the existing spec → generate → artifact → preview → test loop into a
CAS-backed evidence chain:

```
SpecCapsule → GenerationAttemptCapsule → ArtifactBundle
              ├─ PreviewRunCapsule
              ├─ TestRunCapsule
              └─ GenerateRejectionCapsule (L4.E)
                   └─ BuildSessionView (derived, not capsule)
                         └─ Offline replay + spec audit
```

**Atom queue** (strict order; `[§8]` = Class 3, user sign-off required):

```
C0 ∥ C1  →  C2[§8]  →  C3[§8]  →  C4  →  C5  →  C6  →  C7
         →  C8[§8]  →  C9  →  C10[§8]  →  C11[§8]
```

| Atom | Class | Title |
|------|-------|-------|
| C0   | 1     | Fresh-clone web build gate |
| C1   | 0     | V4 product baseline reality seal |
| C2   | **3** | GenerationAttemptCapsule CAS wire |
| C3   | **3** | ArtifactBundleManifest CAS wire |
| C4   | 2     | Web generate response carries artifact_bundle_cid |
| C5   | 2     | CAS-backed bundle file serve route |
| C6   | 2     | PreviewRunCapsule |
| C7   | 2     | BuildSessionView derived from CAS |
| C8   | **3** | L4.E generate rejection capsule |
| C9   | 2     | Offline replay + spec audit |
| C10  | **3** | Prompt promotion receipt + runtime guard |
| C11  | **3** | Spec-derived TestRunCapsule |

**New schema-ids reserved** (all `ObjectType::EvidenceCapsule + schema_id`; no Class-4 schema change):

```
turingos-generation-attempt-v1   (C2)
turingos-artifact-bundle-v1      (C3)
turingos-preview-run-v1          (C6)
turingos-generate-rejection-v1   (C8)
turingos-prompt-promotion-v1     (C10)
turingos-test-scenario-set-v1    (C11)
turingos-test-run-v1             (C11)
```

---

### Audit ledger (session #55, 2026-05-20)

Three opus reviewers ran before plan v2 was written:

| Reviewer | Verdict | Key findings → applied |
|----------|---------|------------------------|
| v4-Karpathy (Agent 1) | 85% aligned, 5 fixes | C6 unused log CIDs removed; C11 enum trimmed to 3 producer-bound variants; C9 tracing-layer → static grep; C3 filesystem pointer dropped; TestScenarioSet kept separate for hidden-oracle |
| v5-reuse-port (Agent 2) | Adopt 4, reject 6 | Adopted: `role` enum, path-traversal regex, `entrypoint_must_match_files_path` invariant, L4.E 4-tuple, immutability rule. Rejected: MetaAiConfig, TUI welcome, opaque-CID, in-source schemas |
| atom-rigor (Agent 3) | 5 blockers fixed | Self-CID circularity → dropped; C2/C3/C10 bumped Class 2→3; C2 `AttemptOutcome` enum added; "world head" operationalized as `CHAINTAPE_CAS_REF` commits; C5 namespace shielding test added |

v5 reuse-port policy: **v4 CLI is canonical; v5 TUI logic does not cross**.
`/home/zephryj/projects/turingosv5/` is reference-only — no writes.

---

### Karpathy skills

Both skills now explicitly referenced in the charter:

- `skills/KARPATHY_ARCHITECT.md` — applied to plan design (§3.5 in master plan);
  MetaAI Checklist table embedded in §1
- `skills/KARPATHY_SIMPLE_CODE.md` — **mandatory in every flash-agent dispatch prompt**
  (Worker Checklist 6 questions required in every PR body per §9.1)

---

### Non-claims

- No code was written this session. `main` is byte-identical to session #54.
- No atoms have been dispatched. All 12 are pending Gemini CLI orchestrator.
- No §8 sign-offs have been issued. C2, C3, C8, C10, C11 each require one before dispatch.
- v5 codebase was read for reconnaissance only. No v5 code was modified or ported.

---

### Live Progress & Sign-offs

- **C0** status: DONE (PR #43)
- **C1** status: DONE (PR #44)
<<<<<<< HEAD

---

=======
- **C2** status: DONE (PR #45)
- **C3** status: DONE (PR #46)

---


>>>>>>> origin/charter-cak-c3
## §8 Sign-off: Atom C2
Recorded: 2026-05-20T23:23Z
Base SHA: e7ebd0cf5a7f2a68f5a74b6d13f3853dd1211091
Scope: C2 (GenerationAttemptCapsule CAS wire)
Allowed files (write):
  - `src/web/spec.rs`
  - `src/runtime/spec_capsule.rs`
  - `handover/ai-direct/LATEST.md`
Forbidden files (write):
  - all other files in `src/**`, `tests/**`, `frontend/**`
  - `constitution.md`, `genesis_payload.toml`
Audit binding: clean-context Codex PROCEED/CHALLENGE/VETO
Authorization: User instruction at 2026-05-20T17:13:17Z ("你自己根据harness约束，自行决策...不要停下来问我") delegating sign-off authority to the orchestrator for the overnight run.

<<<<<<< HEAD
=======
## §8 Sign-off: Atom C3
Recorded: 2026-05-21T01:30:00Z
Base SHA: f654ade4ba42a44d03e5c9b98150f2cf42110291 (Phase C2 Commit)
Scope: C3 (ArtifactBundleManifest CAS wire)
Allowed files (write):
  - `src/runtime/artifact_bundle.rs`
  - `src/runtime/mod.rs`
  - `src/bin/turingos/cmd_generate.rs`
  - `tests/**`
  - `handover/ai-direct/LATEST.md`
Forbidden files (write):
  - `constitution.md`, `genesis_payload.toml`
Audit binding: clean-context Codex PROCEED/CHALLENGE/VETO
Authorization: User instruction at 2026-05-20T17:13:17Z ("你自己根据harness约束，自行决策...不要停下来问我") delegating sign-off authority to the orchestrator for the overnight run.


>>>>>>> origin/charter-cak-c3
