# V4 Product-CAK Hardening — Execution Plan

| Field             | Value                                                                                  |
|-------------------|----------------------------------------------------------------------------------------|
| Status            | DRAFT v2 — audited by 3 opus reviewers, awaiting orchestrator dispatch                 |
| Date              | 2026-05-20                                                                             |
| Source            | Architect directive (in-conversation, 2026-05-20)                                      |
| Predecessor       | TISR Phase 6.3 (SHIPPED), TISR Phase 7 Web MVP (PR #6, PR #11)                         |
| Phase             | P7.z Product-CAK Hardening                                                             |
| Authority         | User-authorized 2026-05-20                                                             |
| Operating Mode    | Constitutional Harness Engineering (harness → real run → audit → ship)                 |
| Architect lens    | **Karpathy Architect skill** — `skills/KARPATHY_ARCHITECT.md` (applied to this plan)   |
| Implementer lens  | **Karpathy Simple Code skill** — `skills/KARPATHY_SIMPLE_CODE.md` (mandatory in every flash-agent prompt — §9) |
| Audit lineage     | 3 opus reviewers (v4-Karpathy, v5-reuse-port, atom-rigor) ran 2026-05-20; findings folded in below |
| CLI/TUI rule      | **v4 CLI is canonical**. Do NOT port v5 TUI command grammar (`turingosv5/src/devtool/mod.rs:380-541`) |
| Audit default     | Class 3 atoms: clean-context Codex `PROCEED / CHALLENGE / VETO` (binding)              |
| Roadmap link      | Between P6 (Phase 6.3 spec CAS wire) and P7 (Market). Does **not** open Polymarket.   |

## 1. Mission

Preserve the existing v4 product loop (`/welcome` → `/build` → spec grill →
generate → artifact serve → web smoke tests) and harden it into a
**Product-CAK** evidence chain:

```
SpecCapsule
  └─> GenerationAttemptCapsule
        └─> ArtifactBundle
              ├─> PreviewRunCapsule
              ├─> TestRunCapsule
              └─> GenerateRejectionCapsule (L4.E)
                    └─> BuildSessionView (DERIVED, not capsule)
                          └─> Offline replay + spec audit
```

The first cut is at `cmd_generate.rs:18-23` — the literal comment
*"Class 1: filesystem write to /artifacts/. No CAS write."* That line is the
v4 boundary between "can demo" and "real TuringOS delivery."

This is **not** a rewrite. v5 / v6 are **not** authorized. New canonical
substrates are **not** authorized.

### Karpathy Architect MetaAI Checklist (applied to this plan)

| Checklist row                                            | Answer                                                                                |
|----------------------------------------------------------|----------------------------------------------------------------------------------------|
| Core Illusion                                            | "v4 turingos generate becomes a CAS chain of EvidenceCapsules whose derived view is the build session." |
| Core data shapes                                         | 7 schemas, all `ObjectType::EvidenceCapsule + schema_id`; 1 derived view (`BuildSessionView`) |
| Micro end-to-end model                                   | C2 + C3 together: one `turingos generate` invocation produces 1 attempt capsule + 1 bundle manifest in CAS |
| Single source of truth                                   | `<workspace>/cas/` (git-backed via `CHAINTAPE_CAS_REF`)                                |
| Physical bottleneck requiring new infrastructure         | "artifact serve must survive `rm -rf sessions/<id>/artifacts/`" — drives C3 + C5      |
| Why this is not fake future extensibility                | Every schema field has a producer in the same or earlier atom (verified per-atom in §7) |
| Runtime truth boundary                                   | Capsules in CAS, indexed by `schema_id`. UI session, filesystem artifacts, console output are all derived views. |

## 2. Verified code facts (anchors)

Confirmed against `main` on 2026-05-20. v5 reuse-port references marked `[v5:…]`.

| # | Anchor                                                       | Fact                                                                                   |
|---|--------------------------------------------------------------|----------------------------------------------------------------------------------------|
| 1 | `src/runtime/spec_capsule.rs:44`                             | `SPEC_CAPSULE_SCHEMA_ID = "turingos-spec-capsule-v1"`                                  |
| 2 | `src/runtime/spec_capsule.rs:83-106`                         | `write_spec_capsule()` uses `ObjectType::EvidenceCapsule + schema_id tag`              |
| 3 | `src/runtime/spec_capsule.rs:113-135`                        | `latest_spec_capsule_cid()` — pattern C4 mirrors (replaces `artifact_bundle_cid.txt`) |
| 4 | `src/runtime/spec_capsule.rs:299-323`                        | `write_grill_turn_capsule()` precedent for JSON-body capsules                          |
| 5 | `src/bottom_white/cas/schema.rs:61`                          | `ObjectType::EvidenceCapsule` exists (Class 4 — MUST NOT modify)                       |
| 6 | `src/bottom_white/cas/git_chain.rs:107,260,280,296`          | Git-backed CAS commit chain via `CHAINTAPE_CAS_REF`                                    |
| 7 | `src/web/router.rs:79`                                       | `FRONTEND_MAIN_JS = include_bytes!("../../frontend/dist/main.js")` (C0 gate)           |
| 8 | `src/web/router.rs:106-144`                                  | Routes: `/welcome /build /api/spec/{questions,submit,turn} /api/generate /api/artifact/:session_id/:name /ws` |
| 9 | `src/web/generate.rs:60,238-245,319`                         | Current: shellout, walks `artifacts/`, `MAX_GENERATE_ATTEMPTS: u8 = 3`                  |
| 10| `src/web/artifact.rs:83-115`                                 | Path-traversal protected legacy serve (kept untouched until future charter)            |
| 11| `src/bin/turingos/cmd_generate.rs:18-23`                     | Literal comment: *"Class 1: filesystem write to /artifacts/. No CAS write."*           |
| 12| Tests already in `tests/`                                    | `cli_phase63_cas_wire.rs`, `cli_web_generate_smoke.rs`, `cli_web_routes_smoke.rs`, `cli_web_spec_smoke.rs`, `cli_web_verify_smoke.rs`, `cli_verify_chaintape_smoke.rs` |
| 13| `.gitignore` (read 2026-05-20)                               | **No `frontend/dist/` entry**. C0 must verify the real mechanism before fixing.        |
| 14| `[v5:turingosv5/schemas/v5_dev/artifact_bundle.schema.json:1-81]` | Field set v4 mirrors (typed `role` enum, path regex, cross-field invariant)        |
| 15| `[v5:turingosv5/docs/contracts/friendly_error_l4e.md:1-37]`  | 4-tuple v4 mirrors for C8                                                              |
| 16| `[v5:turingosv5/docs/contracts/edit_regenerate_versioning.md:1-42]` | Immutability rule v4 adopts in C3                                              |
| 17| `[v5:turingosv5/src/devtool/mod.rs:380-541]`                 | v5 MetaAI / TUI / DeepSeek config — DO NOT port to v4                                   |

## 3. Global constraints

### 3.1 Forbidden surfaces (apply to every atom)

Hard blocklist — flash agents **must abort** on any write to:

- `constitution.md`
- `genesis_payload.toml`
- `src/bottom_white/cas/schema.rs`
- `src/kernel.rs`
- `src/bus.rs`
- `src/state/sequencer.rs`
- `src/state/typed_tx.rs`
- `src/state/**` (all admission/state machine code)
- `src/sdk/tools/wallet.rs`
- `Cargo.toml`
- `Cargo.lock`
- `frontend/**` (UI is out of scope for every atom in this document)
- **anything inside `/home/zephryj/projects/turingosv5/`** (v5 is research-only reference; no write, no symlink, no path injection)

If a flash agent believes it needs to touch one of these, it must stop and
escalate; the orchestrator must classify the work as a separate Class 4 atom
and obtain explicit §8 sign-off before proceeding.

### 3.2 Forbidden patterns

- Do **not** introduce a new canonical substrate (no `src/cas.rs`,
  `src/hash.rs`, `src/versioned_state.rs`, no parallel CAS).
- Do **not** add a new HEAD pointer or global-latest pointer file (this
  includes the rejected v1 idea `workspace/artifacts/artifact_bundle_cid.txt`).
- Do **not** add a new "truth file" — every new artifact lives in CAS via
  `ObjectType::EvidenceCapsule + schema_id`.
- Do **not** put self-CIDs inside the body of a CAS-addressed object. CID is
  `hash(body)`; including the CID in the body breaks the hash. The CAS index
  stores the CID at put-time.
- Do **not** reserve schema fields for unimplemented producers. **Bump
  `schema_id` to `v2` when a producer ships**, not before.
- Do **not** delete the legacy `/api/artifact/:session_id/:name` route until
  an explicit future atom authorizes it.
- Do **not** overwrite prompt v1 with v2/v3 outside the C10 promotion path.
- Do **not** migrate / rewrite historical evidence (`feedback_no_retroactive_evidence_rewrite`).
- Do **not** port v5 TUI command grammar (`turingosv5/src/devtool/mod.rs:380-541`).
  v4 CLI stays canonical. v5 is reference-only for capsule field sets and
  contract semantics; UI / config layers do not cross.
- Do **not** introduce v5's `MetaAiConfig`-style config-as-evidence surface
  (50+ provider-tuning knobs inside a capsule).

### 3.3 Canonical pattern (use everywhere)

Mirror `src/runtime/spec_capsule.rs:83-106`. Every new capsule writer must:

```rust
let cas_dir = workspace.join("cas");
std::fs::create_dir_all(&cas_dir)?;
let mut store = CasStore::open(&cas_dir)?;
let cid = store.put(
    body_bytes,                          // serde_json::to_vec(&body)? for JSON capsules
    ObjectType::EvidenceCapsule,         // do NOT add a new ObjectType variant
    creator,                             // "user" / "generate_system" / etc.
    logical_t,                           // Unix-epoch seconds
    Some(SCHEMA_ID.to_string()),         // turingos-<thing>-v1
)?;
Ok(cid.hex())
```

#### Reserved schema-id strings (all v1)

| Schema ID                           | Capsule                       | Atom |
|-------------------------------------|-------------------------------|------|
| `turingos-spec-capsule-v1`          | (already exists)              | —    |
| `turingos-spec-grill-turn-v1`       | (already exists)              | —    |
| `turingos-spec-grill-session-v1`    | (already exists)              | —    |
| `turingos-generation-attempt-v1`    | GenerationAttemptCapsule      | C2   |
| `turingos-artifact-bundle-v1`       | ArtifactBundleManifest        | C3   |
| `turingos-preview-run-v1`           | PreviewRunCapsule             | C6   |
| `turingos-generate-rejection-v1`    | GenerateRejectionCapsule      | C8   |
| `turingos-prompt-promotion-v1`      | PromptPromotionReceipt        | C10  |
| `turingos-test-scenario-set-v1`     | TestScenarioSet               | C11  |
| `turingos-test-run-v1`              | TestRunCapsule                | C11  |

`BuildSessionView` is **not** a capsule. It is a derived projection over the
above schemas, returned by a route in C7. It has no `schema_id` and is never
written to CAS.

#### v5-derived reusable elements (adopted into v4 atoms)

From `turingosv5/schemas/v5_dev/artifact_bundle.schema.json:1-81`:

- **Cross-field invariant declaration** — for any capsule whose validity
  depends on a cross-field relation, declare it explicitly and assert in tests.
  Example: `ArtifactBundleManifest.entrypoint ∈ files[*].path`.
- **Path traversal regex** — every relative path in any capsule body MUST
  match `^(?!/)(?!.*(?:^|/)\.\.(?:/|$)).+`. Applied to: `ArtifactFileEntry.path`,
  `PreviewRunCapsule.entrypoint_path`, `TestScenarioSet` path slots.
- **Typed `role` enum on file entries** — `ArtifactFileEntry.role: ArtifactFileRole`
  where `ArtifactFileRole = { Entrypoint, Source, Asset, Manifest, Test, Other }`.

From `turingosv5/docs/contracts/edit_regenerate_versioning.md`:

- **Immutability rule** — every regeneration produces a **new**
  `artifact_bundle_cid`. No in-place mutation. Older bundles remain replayable.
  C7 view lists `artifact_versions: Vec<cid>` ordered by `(logical_t, cid)`.

From `turingosv5/docs/contracts/friendly_error_l4e.md`:

- **L4.E 4-tuple** for C8: `{attempt_identity, reject_class, user_safe_message,
  reason, world_head_unchanged: true}`. The `world_head_unchanged` flag is
  enforced operationally as "no new commit on `CHAINTAPE_CAS_REF` between
  attempt start and rejection capsule put".

### 3.4 Frontend / config protection

Every atom in this document has `frontend/**`, `Cargo.toml`, `Cargo.lock` in
its forbidden list. UI / dependency changes are explicitly **out of scope**
for the P7.z hardening and must be done in a separate charter.

Exception: in C4, the web response struct gains an optional
`artifact_bundle_cid` field — the frontend MUST keep working without reading
it (additive only, no shape break).

### 3.5 Karpathy Architect lens (this plan's design protocol)

**Audience**: human reader, architect, orchestrator dispatcher.

Source: `skills/KARPATHY_ARCHITECT.md`.

Applied to this plan during v2 design. Specifically:

- **Data shapes > logic** — §3.3 reserves schema-ids before any atom's
  implementation step describes control flow.
- **Monolithic & flat by default** — single binary, single CAS, single web
  process; no microservices, no queues, no broker, no plugin framework.
- **Micro Approach** — C2 + C3 is the smallest end-to-end version proving the
  truth-boundary shift (no preview / no test / no rejection / no replay
  needed yet).
- **Antifragility via simplicity** — every atom's pass criteria includes a
  "delete cache / restart / reread from CAS" replay test (C7, C9 especially).

Architect-checklist verdicts (from Agent 1 audit, 2026-05-20):

| Checklist                                                | v2 verdict           |
|----------------------------------------------------------|----------------------|
| Core Illusion stated                                     | PASS (§1)            |
| Core data shapes minimal & complete                      | PASS (§3.3, 7 schemas, no redundancy after audit) |
| Micro end-to-end                                         | PASS (C2 + C3)       |
| Single source of truth                                   | PASS (CAS only)      |
| Physical bottleneck named                                | PASS (artifact-survives-dir-delete) |
| No fake future extensibility                             | PASS after v2 trims (C6, C11) |
| Runtime truth boundary minimal                           | PASS (§1 diagram)    |

Anti-pattern scan: `Manager / Factory / Engine / Platform / Framework`
**zero** occurrences in atom specs (grep verified).

### 3.6 Karpathy Simple Code lens (mandatory for every flash agent)

**Audience**: every flash agent that implements an atom.

Source: `skills/KARPATHY_SIMPLE_CODE.md`.

**Mandatory inclusion**: §9's per-atom dispatch prompt **must** embed a
verbatim reference to this skill. The Worker Checklist (skill §"Worker
Checklist") is the last gate before any flash agent submits its diff:

```
Did I add a dependency? If yes, was it explicitly allowed?
Did I add an abstraction? If yes, what real boundary does it protect?
Did I change files outside the TaskPacket?
Can the data flow be explained as input -> transform -> output?
Could this be a smaller direct function?
Did tests prove the behavior?
```

The orchestrator rejects any atom completion where:

- A dependency was added that the atom's "Allowed files (write)" did not
  explicitly cover (`Cargo.toml` is forbidden in every atom — adding deps
  requires a separate Class 2+ charter).
- Any new trait / interface with only one implementation and no real boundary.
- Any new `Manager / Factory / Engine / Platform / Framework` symbol.
- Any new background loop / daemon / async task / state machine without a
  named physical bottleneck declared in the atom's "Why it matters" field.
- Any file change outside the atom's whitelist.
- Any clever one-liner that hides data flow.
- Any silent constant encoding a domain assumption (use a named const at
  module top with a `//` line stating the source).

## 4. Pre-action gate protocol

### 4.1 `/runner-preflight` triggers

Every atom that writes under `handover/evidence/` or runs an evaluator MUST
invoke `/runner-preflight` before writing. This applies to **every atom from
C2 onward** (C0/C1 are docs / fresh-clone fix and exempt).

`/runner-preflight` enforces (per `AGENTS.md §8`):

1. Clean / understood tree
2. Fresh binaries vs current source/HEAD
3. Evidence immutability (no retroactive rewrites)
4. Risk class declared
5. FC trace declared
6. Charter / directive completeness (this document is the directive)
7. Audit-round state

### 4.2 §8 architect sign-off

| Risk class | §8 sign-off required?  | Audit required?                                      |
|------------|------------------------|------------------------------------------------------|
| 0 (docs)              | No          | No                                                                  |
| 1 (additive helper)   | No          | Predicate self-test only                                            |
| 2 (production wire-up)| No          | Clean-context Codex (witness)                                       |
| 3 (auth / money / CAS evidence / admission guard) | **Pre-impl §8** | Clean-context Codex `PROCEED / CHALLENGE / VETO` (binding) |
| 4 (constitution / sequencer / schema) | **Per-atom §8 pre-impl** | Dual independent witness (Codex + Gemini), PRE-§8     |

**No atom in this document is Class 4.** If a flash agent discovers a Class 4
path mid-implementation, it must stop, undo, and escalate.

**Class 3 atoms after audit**: C2, C3, C8, C10, C11 (5 atoms).
**Class 2 atoms**: C4, C5, C6, C7, C9 (5 atoms).
**Class 1 atom**: C0.
**Class 0 atom**: C1.

§8 packet must include the diff SHA being signed. Sign-offs do not transfer
to re-rolled diffs.

### 4.3 Per-atom completion checklist (template)

The orchestrator marks all **eight** boxes before declaring an atom done:

- [ ] `/runner-preflight` ran and returned clean (atoms C2+)
- [ ] All "Allowed files (write)" stayed inside whitelist (`git diff --name-only` ⊆ whitelist)
- [ ] No "Forbidden files (write)" touched (`git diff --name-only` ∩ forbidden = ∅)
- [ ] All "Acceptance commands" returned exit 0
- [ ] All "Pass criteria" invariants hold (verified by running the named tests)
- [ ] Karpathy Simple Code Worker Checklist answered honestly in PR body
- [ ] Commit message contains `FC-trace: <FC1-Nx | FC2-Nx | FC3-Nx>` per `feedback_fc_first_problem_handling`
- [ ] §8 sign-off recorded (Class 3) **with diff SHA captured pre-signature**

**Rollback policy on partial failure**: if any acceptance command fails OR
any forbidden file is touched, the orchestrator runs `git restore .` (clean
revert), records the attempt, and re-dispatches the atom. Atoms are never
"half-shipped" into the next atom's predecessor state.

## 5. Harness verification commands

Shared canonical commands. Atoms reuse these by name.

```bash
# Build
cargo check
cargo build --bin turingos
cargo build --features web --bin turingos_web

# Workspace test (canonical ship gate per feedback_workspace_test_canonical)
cargo test --workspace --no-fail-fast

# Constitution gates
bash scripts/run_constitution_gates.sh
cargo test --test constitution_matrix_drift

# Web smoke (canonical W-suite)
cargo test --features web --test cli_web_routes_smoke
cargo test --features web --test cli_web_spec_smoke
cargo test --features web --test cli_web_generate_smoke
cargo test --features web --test cli_web_verify_smoke

# CAS wire smoke (Phase 6.3 anchor)
cargo test --test cli_phase63_cas_wire
cargo test --test cli_verify_chaintape_smoke

# Frontend (only when C0 atom runs)
cd frontend && npm ci && npm run build && npm test
```

## 6. Atom queue

Dispatch order: C0, C1 (parallel) → C2 (**§8**) → C3 (**§8**) → C4 → C5 → C6
→ C7 → C8 (**§8**) → C9 → C10 (**§8**) → C11 (**§8**).

Each flash agent receives exactly one atom spec from §7. C2, C3, C8, C10, C11
require §8 sign-off before dispatch.

| Atom | Phase | Title                                          | Class | Predecessor | Touches                  | §8?  |
|------|-------|------------------------------------------------|-------|-------------|--------------------------|------|
| C0   | P0    | Fresh-clone web build gate                     | 1     | —           | build.rs / docs          | No   |
| C1   | P1    | V4 product baseline reality seal               | 0     | —           | docs only                | No   |
| C2   | P2    | GenerationAttemptCapsule CAS wire              | **3** | C1          | runtime + cmd_generate   | **Yes** |
| C3   | P3a   | ArtifactBundleManifest CAS wire                | **3** | C2          | runtime + cmd_generate   | **Yes** |
| C4   | P3b   | Web generate response carries artifact_bundle_cid | 2  | C3          | web/generate.rs          | No   |
| C5   | P4    | CAS-backed bundle file serve route             | 2     | C4          | web router + new handler | No   |
| C6   | P5    | PreviewRunCapsule                              | 2     | C5          | runtime + web preview    | No   |
| C7   | P6    | BuildSessionView derived from CAS              | 2     | C6          | runtime + web build      | No   |
| C8   | P7    | L4.E generate rejection capsule                | **3** | C7          | runtime + cmd_generate + web/generate.rs | **Yes** |
| C9   | P8    | Offline replay + spec audit                    | 2     | C8          | new CLI subcommands       | No   |
| C10  | P9    | Prompt promotion receipt + runtime guard       | **3** | C9          | runtime + cmd_llm        | **Yes** |
| C11  | P10   | Spec-derived TestRunCapsule                    | **3** | C10         | runtime + cmd_generate + web/generate.rs | **Yes** |

## 7. Atom specs

Field schema (used uniformly):

1. **Atom ID** · 2. **Phase tag** · 3. **Title** · 4. **Predecessor** ·
5. **Goal** · 6. **Why it matters** · 7. **Risk class** · 8. **FC trace** ·
9. **Pre-action gates** · 10. **Allowed files (write)** ·
11. **Forbidden files (write)** · 12. **Schema spec** ·
13. **Implementation steps** · 14. **Acceptance commands** ·
15. **Pass criteria** · 16. **Kill criteria** · 17. **Anti-drift notes** ·
18. **Done definition** (the eight §4.3 boxes).

---

### C0 — Fresh-clone web build gate

- **Atom ID**: C0
- **Phase tag**: P0
- **Title**: Make `cargo build --features web` succeed on a fresh clone, or fail with an actionable message.
- **Predecessor**: —
- **Goal**: A new contributor cloning the repo runs a documented command sequence and gets `turingos_web` built; if they skip a step, the build error tells them exactly what to do.
- **Why it matters**: `router.rs:79` uses `include_bytes!("../../frontend/dist/main.js")` but `frontend/dist/main.js` is not in git (verified 2026-05-20). The mechanism is **not** `.gitignore` — verify first.
- **Risk class**: 1
- **FC trace**: FC2 (boot adapter)
- **Pre-action gates**: none (no evidence writes; no §8)
- **Allowed files (write)**:
  - `build.rs` (new, optional)
  - `README.md`
  - `docs/CONTRIBUTING.md` (new, optional)
  - `frontend/dist/.gitkeep` (only if decision is "commit a placeholder")
- **Forbidden files (write)**:
  - `frontend/**` other than `.gitkeep`
  - all forbidden surfaces from §3.1
  - `Cargo.toml`, `Cargo.lock`
  - `src/web/router.rs` (no removal of `include_bytes!`)
- **Schema spec**: none
- **Implementation steps**:
  1. Verify the actual fresh-clone failure mechanism: `git ls-files frontend/dist/`. Record the finding in the PR body.
  2. If `dist/main.js` is missing from git, choose **option (c)**: emit a `compile_error!` from `build.rs` (or a `cfg`-gated stub in `router.rs` via a new tiny adapter) with the literal text `run: cd frontend && npm ci && npm run build`. Options (a) commit dist as binary and (b) auto-run npm from build.rs are explicitly rejected (a = pollutes repo, b = invisible side effect).
  3. Update README's "build" section to document the canonical sequence.
- **Acceptance commands**:
  ```bash
  rm -rf frontend/dist
  cargo build --features web --bin turingos_web 2>err.log; EC=$?
  test $EC -ne 0
  grep -qi "cd frontend && npm" err.log
  cd frontend && npm ci && npm run build
  cargo build --features web --bin turingos_web
  cargo test --features web --test cli_web_routes_smoke
  cargo test --features web --test cli_web_generate_smoke
  cd frontend && npm test
  ```
- **Pass criteria**:
  - Without `npm run build`, build fails (`EC != 0`) AND stderr contains literal `cd frontend && npm`.
  - After `npm run build`, all web smoke tests pass.
  - No new top-level dependency in `Cargo.toml`.
- **Kill criteria**: any of — introduces Vite / Next.js / Tauri / new toolchain · changes `include_bytes!` single-binary design · adds a new dependency · modifies frontend.
- **Anti-drift notes**:
  - Do **not** rewrite the frontend.
  - Do **not** "fix" by switching to a dev server.
- **Done definition**: §4.3 boxes 2, 3, 4, 5, 6, 7 (1 and 8 N/A).

---

### C1 — V4 product baseline reality seal

- **Atom ID**: C1
- **Phase tag**: P1
- **Title**: Lock down what already works so a future AI coder doesn't delete it.
- **Predecessor**: —
- **Goal**: A short doc enumerates machine-provable facts of what's already on `main`, with grep / test commands that prove each.
- **Why it matters**: `feedback_no_retroactive_evidence_rewrite` + repeated AI-coder rewrite history.
- **Risk class**: 0
- **FC trace**: none (docs only)
- **Pre-action gates**: none
- **Allowed files (write)**:
  - `docs/roadmap/V4_PRODUCT_BASELINE_REALITY_SEAL.md` (new)
  - `handover/ai-direct/LATEST.md` (one line link)
- **Forbidden files (write)**:
  - all forbidden surfaces from §3.1
  - all `src/**`, `tests/**`, `frontend/**`
- **Schema spec**: none
- **Implementation steps**:
  1. Write `docs/roadmap/V4_PRODUCT_BASELINE_REALITY_SEAL.md` with only machine-provable assertions.
  2. Each assertion has a `git grep` or `cargo test` command inline.
  3. Doc explicitly lists what does **not** yet exist (ArtifactBundle, PreviewRunCapsule, BuildSessionView from CAS, offline replay, spec audit).
- **Acceptance commands**:
  ```bash
  git grep -n '"/welcome"' src/web/router.rs
  git grep -n '"/build"' src/web/router.rs
  git grep -n '"/api/spec/submit"' src/web/router.rs
  git grep -n '"/api/spec/turn"' src/web/router.rs
  git grep -n '"/api/generate"' src/web/router.rs
  git grep -n '"/api/artifact/' src/web/router.rs
  git grep -n 'SPEC_CAPSULE_SCHEMA_ID' src/runtime/spec_capsule.rs
  git grep -n 'No CAS write' src/bin/turingos/cmd_generate.rs
  git grep -n 'CHAINTAPE_CAS_REF' src/bottom_white/cas/git_chain.rs
  cargo test --features web --test cli_web_routes_smoke
  ```
- **Pass criteria**: doc exists; every command above returns the cited line; the "does not yet exist" list is present and accurate.
- **Kill criteria**: any opinion / scope / future promise / design discussion in the doc.
- **Anti-drift notes**: zero opinions, zero scope, zero design.
- **Done definition**: §4.3 boxes 2, 3, 4, 5, 6, 7 (1 and 8 N/A).

---

### C2 — GenerationAttemptCapsule CAS wire

- **Atom ID**: C2
- **Phase tag**: P2
- **Title**: Every LLM call inside `turingos generate` becomes a CAS-anchored capsule with an explicit outcome.
- **Predecessor**: C1
- **Goal**: After `turingos generate` runs (success or any failure), the workspace CAS contains exactly one `GenerationAttemptCapsule` per attempt.
- **Why it matters**: First CAS evidence anchor for the generate pipeline; honors `feedback_chaintape_externalized_proposal` (1 LLM call → 1 capsule).
- **Risk class**: **3** (canonical CAS evidence anchor for LLM proposal pipeline)
- **FC trace**: FC1 (LLM proposal externalization), FC3-N4 (CAS evidence binding)
- **Pre-action gates**:
  - `/runner-preflight` before evidence-bearing real run
  - **§8 sign-off pre-impl** (Class 3) — scope / allowed paths / forbidden paths / Codex audit binding (`PROCEED / CHALLENGE / VETO`)
  - Post-impl clean-context Codex audit (binding)
- **Allowed files (write)**:
  - `src/runtime/generation_attempt.rs` (new)
  - `src/runtime/mod.rs` (re-export only)
  - `src/bin/turingos/cmd_generate.rs` (additive — keep existing filesystem write)
  - `tests/generation_attempt_capsule_cas_wire.rs` (new)
  - `tests/generate_attempt_records_raw_output_cid.rs` (new)
  - `tests/generate_retry_attempts_are_distinct.rs` (new)
  - `tests/generate_attempt_outcome_routes_to_rejection.rs` (new)
  - `tests/generate_attempt_prompt_hash_is_canonical.rs` (new)
- **Forbidden files (write)**:
  - `src/bottom_white/cas/schema.rs` (Class 4)
  - all forbidden surfaces from §3.1
  - `src/web/**` (deferred to C4)
  - `frontend/**`
- **Schema spec**:
  ```rust
  // src/runtime/generation_attempt.rs
  pub const GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID: &str = "turingos-generation-attempt-v1";

  #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
  #[repr(u8)]
  pub enum AttemptOutcome {
      Success = 0,
      ParseFailed = 1,
      LlmApiError = 2,
      NoFilesParsed = 3,
      InternalIo = 4,
  }

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
  pub struct GenerationAttemptCapsule {
      pub schema_id: String,                       // = "turingos-generation-attempt-v1"
      pub session_id: String,
      pub spec_capsule_cid: Option<String>,
      pub spec_source: String,                     // "cas_capsule" | "ondisk_spec_md"
      pub model_id: String,
      pub model_seed: Option<u64>,                 // when provider supports it; None otherwise
      pub prompt_hash: String,                     // hex sha256 of canonical prompt
      pub raw_output_cid: Option<String>,          // None if LlmApiError before any bytes returned
      pub usage_total_tokens: Option<u32>,
      pub retry_index: u32,                        // 0..MAX_GENERATE_ATTEMPTS-1
      pub parent_attempt_cid: Option<String>,      // previous retry in this session, ordering chain
      pub outcome: AttemptOutcome,
      pub parsed_file_count: usize,                // informational, never gating
      pub logical_t: u64,
  }

  pub fn write_generation_attempt_capsule(
      workspace: &std::path::Path,
      body: &GenerationAttemptCapsule,
  ) -> Result<String, crate::runtime::spec_capsule::CapsuleError>;
  ```
- **Implementation steps**:
  1. Create `src/runtime/generation_attempt.rs` mirroring `spec_capsule.rs:299-323` pattern.
  2. Before each LLM call: compute `prompt_hash = sha256(canonical_prompt_string)`.
  3. After LLM responds (or errors): if bytes returned, write raw output to CAS as `EvidenceCapsule` with no `schema_id`; capture `raw_output_cid`.
  4. Classify `outcome` based on parse result + error class.
  5. Build the capsule body; link `parent_attempt_cid` to previous retry CID if any.
  6. Call `write_generation_attempt_capsule()`; capture CID.
  7. **Rollback rule**: if step 3 succeeds but step 6 fails (CAS put error), do not leave the raw-output CID dangling — emit an `InternalIo` rejection via the C8 pattern (this means C2 must already know the C8 writer exists; cross-coordinate during dispatch).
  8. Print one stderr line: `generation_attempt_cid=<hex>`.
  9. On `outcome != Success`, the attempt is also routed to C8's rejection path (C8 references this CID via `generation_attempt_cid`).
  10. Keep all filesystem behavior unchanged.
- **Acceptance commands**:
  ```bash
  cargo check
  cargo test --test generation_attempt_capsule_cas_wire
  cargo test --test generate_attempt_records_raw_output_cid
  cargo test --test generate_retry_attempts_are_distinct
  cargo test --test generate_attempt_outcome_routes_to_rejection
  cargo test --test generate_attempt_prompt_hash_is_canonical
  cargo test --test cli_phase63_cas_wire
  cargo test --features web --test cli_web_generate_smoke
  cargo test --workspace --no-fail-fast
  bash scripts/run_constitution_gates.sh
  ```
- **Pass criteria**:
  - `CasStore::list_cids_by_object_type(EvidenceCapsule)` after generate returns ≥ 1 capsule with `schema_id == "turingos-generation-attempt-v1"`.
  - Each retry produces a **distinct** CID; `parent_attempt_cid` chain reconstructs retry order.
  - `prompt_hash` byte-equals `sha256(canonical_prompt_string)`.
  - `outcome == Success` ⟺ at least one file parsed AND no LLM error.
  - On any non-Success outcome, C8's rejection capsule references this attempt's CID.
- **Kill criteria**: schema adds a field with no producer · adds a new `ObjectType` · advances `CHAINTAPE_CAS_REF` outside the CAS put · touches `src/web/**`.
- **Anti-drift notes**:
  - Do **not** add a new `ObjectType` variant.
  - Do **not** modify the existing filesystem write path.
  - Do **not** touch `src/web/**`; C4 wires the web response.
  - `parsed_file_count` is informational only; do not let it gate anything (`outcome` is the gate).
- **Done definition**: all eight §4.3 boxes ticked + §8 sign-off recorded + Codex audit verdict ≠ VETO.

---

### C3 — ArtifactBundleManifest CAS wire

- **Atom ID**: C3
- **Phase tag**: P3a
- **Title**: Generated artifacts become a CAS-anchored bundle; filesystem becomes a legacy view.
- **Predecessor**: C2
- **Goal**: After `turingos generate` succeeds, a single `ArtifactBundleManifest` capsule in CAS lists every generated file with its own CID. Each regeneration produces a new bundle CID (no mutation).
- **Why it matters**: The architect's first cut — `cmd_generate.rs:18-23` "No CAS write" comment goes away.
- **Risk class**: **3** (canonical CAS evidence anchor for LLM-produced bytes)
- **FC trace**: FC1 (LLM output externalization), FC3-N4 (CAS evidence binding)
- **Pre-action gates**:
  - `/runner-preflight`
  - **§8 sign-off pre-impl** (Class 3)
  - Post-impl clean-context Codex audit (binding)
- **Allowed files (write)**:
  - `src/runtime/artifact_bundle.rs` (new)
  - `src/runtime/mod.rs` (re-export only)
  - `src/bin/turingos/cmd_generate.rs` (additive — keep existing filesystem write)
  - `tests/artifact_bundle_cas_wire.rs` (new)
  - `tests/generated_artifact_has_bundle_manifest.rs` (new)
  - `tests/artifact_bundle_files_have_cids.rs` (new)
  - `tests/artifact_bundle_entrypoint_in_files.rs` (new)
  - `tests/artifact_bundle_path_traversal_rejected.rs` (new)
  - `tests/artifact_bundle_regen_is_new_cid.rs` (new)
- **Forbidden files (write)**:
  - `src/bottom_white/cas/schema.rs` (Class 4)
  - all forbidden surfaces from §3.1
  - `src/web/**` (C4 wires response; C5 wires serve)
  - `frontend/**`
- **Schema spec**:
  ```rust
  // src/runtime/artifact_bundle.rs
  pub const ARTIFACT_BUNDLE_SCHEMA_ID: &str = "turingos-artifact-bundle-v1";

  // v5-derived typed role enum
  #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
  #[serde(rename_all = "lowercase")]
  pub enum ArtifactFileRole {
      Entrypoint,
      Source,
      Asset,
      Manifest,
      Test,
      Other,
  }

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
  pub struct ArtifactFileEntry {
      pub path: String,        // must match ^(?!/)(?!.*(?:^|/)\.\.(?:/|$)).+
      pub cid: String,         // hex cid of file bytes in CAS
      pub mime: String,
      pub sha256: String,
      pub size_bytes: u64,
      pub role: ArtifactFileRole,
  }

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
  pub struct ArtifactBundleManifest {
      pub schema_id: String,                       // = "turingos-artifact-bundle-v1"
      // NOTE: no self-cid field. The bundle's CID is returned by CasStore::put and stored in the CAS index.
      pub session_id: String,
      pub spec_capsule_cid: Option<String>,
      pub generation_attempt_cid: String,          // references C2 capsule
      pub previous_bundle_cid: Option<String>,     // provenance chain across regenerations
      pub files: Vec<ArtifactFileEntry>,
      pub entrypoint: String,                      // MUST equal one of files[].path
      pub bundle_size_bytes_total: u64,            // sum of files[].size_bytes
      pub created_at_logical_t: u64,
  }

  pub fn write_artifact_bundle(
      workspace: &std::path::Path,
      body: &ArtifactBundleManifest,
  ) -> Result<String, crate::runtime::spec_capsule::CapsuleError>;

  pub fn latest_artifact_bundle_cid_for_session(
      workspace: &std::path::Path,
      session_id: &str,
  ) -> Result<Option<String>, crate::runtime::spec_capsule::CapsuleError>;
  ```
- **Implementation steps**:
  1. Create `src/runtime/artifact_bundle.rs` with the writer + reader + `latest_artifact_bundle_cid_for_session()` (mirror `latest_spec_capsule_cid` at `spec_capsule.rs:113-135`).
  2. Path-traversal regex check on every `ArtifactFileEntry.path` at write time; reject otherwise.
  3. Cross-field invariant check at write time: `manifest.entrypoint ∈ manifest.files[*].path`.
  4. In `cmd_generate.rs`, after files are parsed and written to `workspace/artifacts/`:
     - For each generated file, `store.put(file_bytes, EvidenceCapsule, "generate_system", logical_t, None)` and capture CID.
     - Classify `role` (Entrypoint for `index.html` or chosen entry; Source for `.html/.js/.css/.ts`; Asset otherwise unless extension says Manifest/Test).
     - Sum `size_bytes` for `bundle_size_bytes_total`.
     - Detect entrypoint (`index.html` preferred → first HTML → first file).
     - If `latest_artifact_bundle_cid_for_session(workspace, session_id)` returns Some, set `previous_bundle_cid`.
     - Call `write_artifact_bundle()`; capture bundle CID from CAS.
     - Print stdout line: `artifact_bundle_cid=<hex>`.
  5. Keep `workspace/artifacts/` files exactly as before.
  6. Do **not** write `artifact_bundle_cid.txt`.
- **Acceptance commands**:
  ```bash
  cargo check
  cargo test --test artifact_bundle_cas_wire
  cargo test --test generated_artifact_has_bundle_manifest
  cargo test --test artifact_bundle_files_have_cids
  cargo test --test artifact_bundle_entrypoint_in_files
  cargo test --test artifact_bundle_path_traversal_rejected
  cargo test --test artifact_bundle_regen_is_new_cid
  cargo test --test cli_phase63_cas_wire
  cargo test --features web --test cli_web_generate_smoke
  cargo test --workspace --no-fail-fast
  bash scripts/run_constitution_gates.sh
  ```
- **Pass criteria**:
  - Exactly one `ArtifactBundleManifest` capsule per successful generate.
  - Every file in `workspace/artifacts/` has a matching `files[].cid` that resolves to those bytes.
  - `entrypoint ∈ files[].path`, byte-stable.
  - Paths containing `..` or starting with `/` are rejected pre-write.
  - Two successive regenerations of the same session produce two distinct bundle CIDs; the second's `previous_bundle_cid` points at the first.
  - No `artifact_bundle_cid.txt` exists.
- **Kill criteria**: schema includes a self-CID · uses `f64` anywhere · introduces a filesystem pointer · skips path-regex check.
- **Anti-drift notes**:
  - Do **not** delete `workspace/artifacts/` files.
  - Do **not** introduce `artifact_bundle_cid.txt` or any other "latest" pointer.
  - Do **not** put the bundle CID inside the bundle body.
- **Done definition**: all eight §4.3 boxes + §8 sign-off + Codex verdict ≠ VETO.

---

### C4 — Web generate response carries artifact_bundle_cid

- **Atom ID**: C4
- **Phase tag**: P3b
- **Title**: `POST /api/generate` returns the bundle CID alongside existing filesystem artifact entries.
- **Predecessor**: C3
- **Goal**: A web client that reads the new field can use it; a web client that ignores it sees no change.
- **Why it matters**: Bridges CAS truth into the web API without breaking the existing frontend or smoke tests.
- **Risk class**: 2
- **FC trace**: FC1 (web boundary externalization)
- **Pre-action gates**: `/runner-preflight`. No §8.
- **Allowed files (write)**:
  - `src/web/generate.rs`
  - `tests/cli_web_generate_returns_artifact_bundle_cid.rs` (new)
  - `tests/cli_web_generate_response_shape_stable.rs` (new)
- **Forbidden files (write)**:
  - all forbidden surfaces from §3.1
  - `src/web/router.rs` (no route changes here)
  - `frontend/**`
- **Schema spec**: response struct update (Rust + serde — `serde(default)` everywhere optional fields):
  ```rust
  pub struct GenerateResponse {
      // existing fields preserved byte-stable
      pub artifacts: Vec<ArtifactEntry>,
      pub status: String,
      // ... existing fields ...

      // NEW (additive only)
      #[serde(skip_serializing_if = "Option::is_none")]
      pub artifact_bundle_cid: Option<String>,
  }

  pub struct ArtifactEntry {
      // existing
      pub path: String,
      pub size_bytes: u64,
      pub content_type: String,
      // NEW (additive)
      #[serde(skip_serializing_if = "Option::is_none")]
      pub cid: Option<String>,
      #[serde(skip_serializing_if = "Option::is_none")]
      pub sha256: Option<String>,
  }
  ```
- **Implementation steps**:
  1. After shellout returns success, call `latest_artifact_bundle_cid_for_session(workspace, session_id)` (the C3 helper).
  2. Read the bundle manifest from CAS by that CID.
  3. Populate `artifact_bundle_cid` and per-file `cid` / `sha256`.
  4. **Do not** read or write `artifact_bundle_cid.txt`.
  5. **Do not** change route, control flow, shellout invocation, retry loop, or existing fields' types.
- **Acceptance commands**:
  ```bash
  cargo test --test cli_web_generate_returns_artifact_bundle_cid
  cargo test --test cli_web_generate_response_shape_stable
  cargo test --features web --test cli_web_generate_smoke
  cargo test --features web --test cli_web_routes_smoke
  cargo test --workspace --no-fail-fast
  ```
- **Pass criteria**:
  - Response JSON contains `artifact_bundle_cid` (non-null after successful generate).
  - Existing fields byte-stable for a client that ignores new fields (JSON shape stability test enforces this).
  - All web smoke tests pass.
- **Kill criteria**: any change to frontend · any change to shellout · any change to existing field semantics.
- **Anti-drift notes**: additive only. No filesystem pointer reads.
- **Done definition**: all eight §4.3 boxes (1 sign-off N/A).

---

### C5 — CAS-backed bundle file serve route (with namespace shielding)

- **Atom ID**: C5
- **Phase tag**: P4
- **Title**: A new HTTP route serves bundle files from CAS by bundle CID + path; only artifact-bundle CIDs are reachable.
- **Predecessor**: C4
- **Goal**: Deleting `sessions/<id>/artifacts/` no longer makes the artifact inaccessible; CAS is the new truth. Other CAS object types (autopsy, run log, private diagnostic) are **not** reachable through this route.
- **Why it matters**: This is the proof-of-truth-shift test. Also closes a shielding gap audit Agent 3 flagged.
- **Risk class**: 2
- **FC trace**: FC1 (external read), FC3 (CAS evidence)
- **Pre-action gates**: `/runner-preflight`. No §8.
- **Allowed files (write)**:
  - `src/web/router.rs` (add one route)
  - `src/web/artifact_bundle.rs` (new handler module)
  - `tests/artifact_bundle_file_serve_reads_cas.rs` (new)
  - `tests/artifact_bundle_file_serve_rejects_unknown_path.rs` (new)
  - `tests/artifact_bundle_file_serve_rejects_traversal.rs` (new)
  - `tests/artifact_bundle_survives_deleted_artifacts_dir.rs` (new — elevated to smoke)
  - `tests/artifact_bundle_serve_rejects_non_bundle_cid.rs` (new — shielding)
  - `tests/artifact_bundle_serve_rejects_private_diagnostic_cid.rs` (new — shielding)
- **Forbidden files (write)**:
  - all forbidden surfaces from §3.1
  - `src/web/artifact.rs` (legacy route stays untouched)
  - `frontend/**`
- **Route spec**:
  ```
  GET /api/bundle/:artifact_bundle_cid/file?path=<relative-path>

  Algorithm:
    1. Look up :artifact_bundle_cid in CAS index.
    2. ABORT with 404 if not found OR its metadata.schema_id != "turingos-artifact-bundle-v1".
    3. Parse manifest body.
    4. Find `path` in manifest.files[].path (byte-equal match, no resolution).
    5. ABORT with 4xx if path not found OR contains `..` or starts with `/`.
    6. Read file CID bytes from CAS.
    7. Return bytes with Content-Type = files[].mime.

  No filesystem fallback. No path resolution. No glob.
  ```
- **Implementation steps**:
  1. Add route + handler module.
  2. Handler reads only CAS; never touches filesystem.
  3. Shielding: explicitly check `schema_id == "turingos-artifact-bundle-v1"` before deserializing the body.
  4. Path validation reuses the C3 regex.
  5. Keep `GET /api/artifact/:session_id/:name` exactly as it is.
- **Acceptance commands**:
  ```bash
  cargo test --test artifact_bundle_file_serve_reads_cas
  cargo test --test artifact_bundle_file_serve_rejects_unknown_path
  cargo test --test artifact_bundle_file_serve_rejects_traversal
  cargo test --test artifact_bundle_survives_deleted_artifacts_dir
  cargo test --test artifact_bundle_serve_rejects_non_bundle_cid
  cargo test --test artifact_bundle_serve_rejects_private_diagnostic_cid
  cargo test --features web --test cli_web_routes_smoke
  cargo test --workspace --no-fail-fast
  ```
- **Pass criteria**:
  - After `rm -rf sessions/<id>/artifacts/`, `GET /api/bundle/<cid>/file?path=index.html` returns 200 with original bytes.
  - `?path=../foo` returns 4xx; `?path=/etc/passwd` returns 4xx.
  - Request with a CID whose `schema_id != "turingos-artifact-bundle-v1"` returns 404 (no body leak).
  - Legacy `/api/artifact/:session_id/:name` still works.
- **Kill criteria**: filesystem fallback added · path resolution introduced · shielding check skipped.
- **Anti-drift notes**:
  - Do **not** delete `src/web/artifact.rs`.
  - Do **not** add cross-CID-type read.
- **Done definition**: all eight §4.3 boxes (1 sign-off N/A).

---

### C6 — PreviewRunCapsule

- **Atom ID**: C6
- **Phase tag**: P5
- **Title**: A preview render becomes a CAS-anchored, read-only observation; world head does not advance.
- **Predecessor**: C5
- **Goal**: Each preview produces exactly one `PreviewRunCapsule` recording bundle / entrypoint / sandbox policy. The CHAINTAPE CAS ref does **not** advance from preview.
- **Why it matters**: Preview is read-only evidence, not state.
- **Risk class**: 2
- **FC trace**: FC3 (CAS evidence binding, read-only observation)
- **Pre-action gates**: `/runner-preflight`. No §8.
- **Allowed files (write)**:
  - `src/runtime/preview_run.rs` (new)
  - `src/runtime/mod.rs` (re-export only)
  - `src/web/preview.rs` (new handler)
  - `tests/preview_reads_artifact_bundle_cid.rs` (new)
  - `tests/preview_run_capsule_written.rs` (new)
  - `tests/preview_run_does_not_advance_chaintape_cas_ref.rs` (new — operational world-head test)
  - `tests/preview_iframe_sandbox_policy_enum.rs` (new)
- **Forbidden files (write)**:
  - all forbidden surfaces from §3.1
  - `src/state/**` (preview does not change state)
  - `frontend/**` (existing iframe sandbox preserved)
- **Schema spec**:
  ```rust
  // src/runtime/preview_run.rs
  pub const PREVIEW_RUN_CAPSULE_SCHEMA_ID: &str = "turingos-preview-run-v1";

  // Byte-stable enum, not free-form string (audit Agent 3 C.6)
  #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
  #[repr(u8)]
  pub enum SandboxPolicy {
      AllowScripts = 0,
      AllowScriptsAllowSameOrigin = 1,
      // future variants land in v2 with their producer
  }

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
  pub struct PreviewRunCapsule {
      pub schema_id: String,                       // = "turingos-preview-run-v1"
      // NOTE: no self-cid (audit Agent 3 C.3)
      pub artifact_bundle_cid: String,
      pub session_id: String,
      pub entrypoint_path: String,                 // matches path regex
      pub sandbox_policy: SandboxPolicy,
      pub serve_success: bool,
      pub logical_t: u64,
  }
  // FUTURE (v2 schema_id bump only when producers ship): console_log_cid, network_log_cid,
  // error_summary_cid, redaction_result_cid. Do NOT reserve in v1.
  ```
- **Implementation steps**:
  1. Create writer mirroring C2/C3 pattern.
  2. Define `world_head_advanced` operationally as: `git_chain::current_oid(CHAINTAPE_CAS_REF)` differs between t-before-preview and t-after-preview. The CAS put for the capsule itself advances the ref by one commit; the test asserts **exactly one commit advance**, no more (no state ref touched).
  3. Wire a preview endpoint; on preview request, write the capsule, return artifact bytes via C5 route.
  4. **Do not** integrate headless browser. **Do not** capture console/network/error/redaction yet.
- **Acceptance commands**:
  ```bash
  cargo test --test preview_reads_artifact_bundle_cid
  cargo test --test preview_run_capsule_written
  cargo test --test preview_run_does_not_advance_chaintape_cas_ref
  cargo test --test preview_iframe_sandbox_policy_enum
  cargo test --features web --test cli_web_routes_smoke
  cargo test --workspace --no-fail-fast
  ```
- **Pass criteria**:
  - One `PreviewRunCapsule` per preview request, byte-stable.
  - `CHAINTAPE_CAS_REF` advances by exactly one commit (the capsule put); no state ref touched.
  - `sandbox_policy` is an enum variant; serialization is the lowercase variant name.
- **Kill criteria**: any state mutation · headless browser introduced · log CIDs reserved without producer · sandbox policy reverts to free-form string.
- **Anti-drift notes**:
  - Do **not** add headless browser automation (deferred to v2 + producer).
  - Do **not** modify the existing iframe sandbox in the frontend.
- **Done definition**: all eight §4.3 boxes (1 sign-off N/A).

---

### C7 — BuildSessionView derived from CAS

- **Atom ID**: C7
- **Phase tag**: P6
- **Title**: The build session view is reconstructed from CAS; in-memory `sessions` map becomes a UX cache. Private diagnostics + test scenarios are shielded from the view.
- **Predecessor**: C6
- **Goal**: `GET /api/build/session/:session_id` returns a session view computed by scanning CAS. No dependency on `AppState.sessions`. Private CIDs do not leak.
- **Why it matters**: Board / session is a derived view (Karpathy + v5).
- **Risk class**: 2
- **FC trace**: FC2 (derived state reconstruction), FC3 (CAS evidence)
- **Pre-action gates**: `/runner-preflight`. No §8.
- **Allowed files (write)**:
  - `src/runtime/build_session_view.rs` (new)
  - `src/runtime/mod.rs` (re-export only)
  - `src/web/router.rs` (append one route; do not edit lines 1-145)
  - `src/web/build_session.rs` (new handler)
  - `tests/build_session_derived_from_cas.rs` (new)
  - `tests/build_session_delete_cache_rebuild.rs` (new)
  - `tests/build_session_mutation_does_not_affect_chain.rs` (new)
  - `tests/build_session_view_does_not_expose_private_diagnostic_cid.rs` (new)
  - `tests/build_session_view_does_not_expose_test_scenario_set_cid.rs` (new — hidden oracle protection, validates pre-C11 already)
  - `tests/build_session_ordering_is_logical_t_then_cid.rs` (new)
- **Forbidden files (write)**:
  - all forbidden surfaces from §3.1
  - `frontend/**`
- **View spec**:
  ```rust
  // src/runtime/build_session_view.rs
  // NOT a capsule. NOT written to CAS. No schema_id.
  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  pub struct BuildSessionView {
      pub session_id: String,
      pub spec_capsule_cid: Option<String>,
      pub generation_attempts: Vec<String>,        // CIDs, ordered by (logical_t, cid)
      pub artifact_versions: Vec<String>,          // ArtifactBundle CIDs
      pub preview_runs: Vec<String>,
      pub rejection_events: Vec<String>,           // populated after C8
      pub current_status: BuildStatus,
      // INTENTIONALLY EXCLUDED: private_diagnostic_cid, test_scenario_set_cid
  }

  #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
  #[serde(rename_all = "snake_case")]
  pub enum BuildStatus {
      SpecPending,
      SpecDone,
      Generating,
      Generated,
      Rejected,
      // EXTENSIBLE ONLY WHEN A PRODUCER SHIPS. No "Accepted" variant yet — C11 adds it then.
  }
  ```
- **Implementation steps**:
  1. `derive_build_session_view(workspace, session_id) -> BuildSessionView` scans CAS index by `schema_id` and filters by `session_id`.
  2. Ordering: `(logical_t, cid)` tuple (audit Agent 3 F.4).
  3. **Explicit exclusion**: when reading `GenerateRejectionCapsule` (post-C8), do **not** copy `private_diagnostic_cid` into the view.
  4. **Explicit exclusion**: when reading `TestRunCapsule` (post-C11), do **not** copy `test_scenario_set_cid` into the view (hidden oracle).
  5. Route `GET /api/build/session/:session_id`.
  6. Handler **must not** read `AppState.sessions` to compute the view (may consult for workspace dir path).
- **Acceptance commands**:
  ```bash
  cargo test --test build_session_derived_from_cas
  cargo test --test build_session_delete_cache_rebuild
  cargo test --test build_session_mutation_does_not_affect_chain
  cargo test --test build_session_view_does_not_expose_private_diagnostic_cid
  cargo test --test build_session_view_does_not_expose_test_scenario_set_cid
  cargo test --test build_session_ordering_is_logical_t_then_cid
  cargo test --features web --test cli_web_spec_smoke
  cargo test --features web --test cli_web_generate_smoke
  cargo test --workspace --no-fail-fast
  ```
- **Pass criteria**:
  - View is byte-stable across cache reset for the same CAS state.
  - Ordering is deterministic by `(logical_t, cid)`.
  - Mutating `AppState.sessions[session_id]` does not change the view.
  - Two shielding tests (private diag, test scenarios) verify exclusion even when those capsules exist.
- **Kill criteria**: any field exposes a private CID · `current_status` is wired into `src/state/sequencer.rs` admission · view becomes a write target.
- **Anti-drift notes**:
  - Do **not** remove the in-memory `sessions` map.
  - Do **not** introduce a new persistence store.
  - Do **not** edit `src/web/router.rs:1-145` (the C0 region); append only.
- **Done definition**: all eight §4.3 boxes (1 sign-off N/A).

---

### C8 — L4.E generate rejection capsule

- **Atom ID**: C8
- **Phase tag**: P7
- **Title**: Every generate failure produces a canonical `GenerateRejectionCapsule` in CAS; private diagnostics stay shielded.
- **Predecessor**: C7
- **Goal**: A failed generate produces one rejection capsule with a public summary + shielded private diagnostic CID; HTTP returns 4xx with sanitized JSON; no state-head advance beyond the capsule put.
- **Why it matters**: `feedback_rejection_evidence_separate` (L4 vs L4.E); `feedback_o1_chain_on_auditability` (rejected → L4.E lane). Privacy invariant: raw diagnostics never enter HTTP body or `BuildSessionView`.
- **Risk class**: **3** (rejection evidence + shielding boundary)
- **FC trace**: FC1 (failure-path externalization), FC3 (L4.E binding)
- **Pre-action gates**:
  - `/runner-preflight`
  - **§8 sign-off pre-impl** (Class 3) — scope MUST cite `feedback_rejection_evidence_separate` + privacy invariant: "private_diagnostic_cid never leaves CAS / never enters HTTP body / never enters BuildSessionView"
  - Post-impl clean-context Codex audit (binding)
- **Allowed files (write)**:
  - `src/runtime/rejection_capsule.rs` (new)
  - `src/runtime/mod.rs` (re-export only)
  - `src/bin/turingos/cmd_generate.rs` (additive)
  - `src/web/generate.rs` (additive)
  - `tests/generate_fail_goes_l4e.rs` (new)
  - `tests/user_error_does_not_leak_panic.rs` (new)
  - `tests/privacy_fail_not_retryable.rs` (new)
  - `tests/rejection_capsule_world_head_unchanged.rs` (new — operational)
  - `tests/rejection_private_diagnostic_not_in_http_body.rs` (new — shielding)
  - `tests/rejection_capsule_4_tuple_present.rs` (new — v5-derived)
- **Forbidden files (write)**:
  - all forbidden surfaces from §3.1
  - `src/state/**`
  - `frontend/**`
- **Schema spec**:
  ```rust
  // src/runtime/rejection_capsule.rs
  pub const GENERATE_REJECTION_CAPSULE_SCHEMA_ID: &str = "turingos-generate-rejection-v1";

  #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
  #[repr(u8)]
  pub enum RejectClass {
      InvalidInput = 0,
      SpecMissing = 1,
      LlmApiError = 2,
      NoFilesParsed = 3,
      TooManyFiles = 4,
      HeuristicFailed = 5,
      PrivacyBlocked = 6,
      BudgetExceeded = 7,
      InternalIo = 8,
  }

  // v5-derived 4-tuple + world_head invariant
  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
  pub struct GenerateRejectionCapsule {
      pub schema_id: String,                       // = "turingos-generate-rejection-v1"
      pub session_id: String,
      pub spec_capsule_cid: Option<String>,
      pub generation_attempt_cid: Option<String>,  // links to C2 capsule if attempt was made
      pub triage_attempted: bool,                  // false if rejected pre-LLM
      pub reject_class: RejectClass,
      pub public_error_summary: String,            // user-safe; no diagnostics
      pub reason: String,                          // short machine-readable reason code
      pub private_diagnostic_cid: Option<String>,  // raw bytes in CAS, SHIELDED
      pub retryable: bool,
      pub world_head_unchanged: bool,              // MUST be true (asserted)
      pub logical_t: u64,
  }
  ```
- **Implementation steps**:
  1. Implement writer mirroring C2/C3.
  2. On every error branch in `cmd_generate.rs`:
     - Classify into `RejectClass`.
     - Write raw diagnostic bytes to CAS (`EvidenceCapsule`, no `schema_id`); capture `private_diagnostic_cid`.
     - Assert `world_head_unchanged` operationally: capture `git_chain::current_oid(CHAINTAPE_CAS_REF)` before, allow exactly +2 commits (raw diag + rejection capsule), no state ref change.
     - Write `GenerateRejectionCapsule`.
     - Exit non-zero. Stderr last line: `rejection_cid=<hex>`.
  3. In `src/web/generate.rs`:
     - On shellout failure, parse `rejection_cid=`.
     - Return 4xx (not 500) with JSON `{ rejection_cid, reject_class, public_error_summary, reason, retryable }`.
     - **Never** include raw diagnostics, panic text, or stack trace in body.
  4. `PrivacyBlocked` → `retryable = false`; web auto-retry honors this.
- **Acceptance commands**:
  ```bash
  cargo test --test generate_fail_goes_l4e
  cargo test --test user_error_does_not_leak_panic
  cargo test --test privacy_fail_not_retryable
  cargo test --test rejection_capsule_world_head_unchanged
  cargo test --test rejection_private_diagnostic_not_in_http_body
  cargo test --test rejection_capsule_4_tuple_present
  cargo test --features web --test cli_web_generate_smoke
  cargo test --features web --test cli_web_verify_smoke
  cargo test --workspace --no-fail-fast
  bash scripts/run_constitution_gates.sh
  ```
- **Pass criteria**:
  - Every failure branch produces exactly one `GenerateRejectionCapsule` with non-empty `public_error_summary` and `reason`.
  - HTTP body never contains a panic message, stack trace, or raw LLM error text.
  - `CHAINTAPE_CAS_REF` advances by ≤ 2 commits (diag + rejection); no state ref touched.
  - `PrivacyBlocked` rejections set `retryable = false`; web auto-retry skips them.
  - `BuildSessionView` (C7) lists rejection CIDs but does **not** include `private_diagnostic_cid`.
- **Kill criteria**: rejection enters L4 (accepted) ledger · raw diagnostic appears in HTTP body · `retryable = true` for `PrivacyBlocked` · world_head_unchanged falsely asserted.
- **Anti-drift notes**:
  - Do **not** write rejections into the L4 (accepted) ledger.
  - Do **not** put raw diagnostics in HTTP responses.
- **Done definition**: all eight §4.3 boxes + §8 sign-off recorded + Codex verdict ≠ VETO.

---

### C9 — Offline replay + spec audit

- **Atom ID**: C9
- **Phase tag**: P8
- **Title**: New CLI subcommands replay and audit a build session **without** calling any LLM.
- **Predecessor**: C8
- **Goal**: `turingos replay --offline --workspace <p> --session <id>` and `turingos spec audit --workspace <p> --session <id>` reconstruct the build session entirely from CAS, with zero network and zero LLM calls. All cross-CID references are verified to resolve.
- **Why it matters**: PR #11 deferred A10; offline replay is the canonical TuringOS audit property.
- **Risk class**: 2
- **FC trace**: FC1 (replay loop), FC2 (boot reconstruction)
- **Pre-action gates**: `/runner-preflight`. No §8.
- **Allowed files (write)**:
  - `src/bin/turingos/cmd_replay.rs` (new — v4 CLI; do NOT mimic v5 TUI)
  - `src/bin/turingos/cmd_spec_audit.rs` (new — v4 CLI)
  - `src/bin/turingos/main.rs` (subcommand wiring — additive)
  - `src/runtime/replay.rs` (new — pure CAS reconstruction)
  - `tests/offline_replay_no_llm_dependency_static_check.rs` (new — static dep grep)
  - `tests/spec_audit_reconstructs_from_cas.rs` (new)
  - `tests/artifact_bundle_replay_reads_cas.rs` (new)
  - `tests/build_session_replay_after_cache_delete.rs` (new)
  - `tests/replay_verifies_all_cross_cid_references_resolve.rs` (new — audit Agent 3 F.3)
- **Forbidden files (write)**:
  - all forbidden surfaces from §3.1
  - `src/web/**`
  - `frontend/**`
- **Implementation steps**:
  1. `replay::reconstruct_session(workspace, session_id)` returns the `BuildSessionView` (C7) plus a step-by-step transcript: spec → grill turns → generation attempts → artifact bundle → preview runs → rejections.
  2. Cross-reference verification: every CID mentioned in any capsule body (e.g., `generation_attempt_cid`, `previous_bundle_cid`, `spec_capsule_cid`) must resolve via the CAS index. Replay **fails loud** on any dangling reference.
  3. `cmd_replay`: prints transcript; flags `--offline`, `--workspace`, `--session` mandatory.
  4. `cmd_spec_audit`: replays only the spec sub-graph; verifies the latest `turingos-spec-capsule-v1` body bytes hash equals on-disk `spec.md` sha256.
  5. **Static no-LLM proof (audit Agent 1 #3)**: a test (`offline_replay_no_llm_dependency_static_check`) runs `cargo tree -p turingos --features '' --bin turingos --no-default-features` and asserts that `src/runtime/replay.rs` + `src/bin/turingos/cmd_replay.rs` + `src/bin/turingos/cmd_spec_audit.rs` do not (a) `use` any siliconflow / reqwest / hyper client module and (b) do not `mod` any module that itself uses them. This is a build-time grep, not a runtime tracing layer.
- **Acceptance commands**:
  ```bash
  cargo test --test offline_replay_no_llm_dependency_static_check
  cargo test --test spec_audit_reconstructs_from_cas
  cargo test --test artifact_bundle_replay_reads_cas
  cargo test --test build_session_replay_after_cache_delete
  cargo test --test replay_verifies_all_cross_cid_references_resolve
  cargo test --workspace --no-fail-fast
  bash scripts/run_constitution_gates.sh
  ```
- **Pass criteria**:
  - Both subcommands succeed with network disabled (test harness ifdef).
  - Replay transcript is byte-stable across reruns over the same workspace.
  - Spec audit exits non-zero when `spec.md` and the latest capsule body diverge.
  - All cross-CID references resolve.
- **Kill criteria**: introduces a runtime tracing interceptor · imports an LLM client · reads a "latest" filesystem pointer.
- **Anti-drift notes**:
  - Do **not** copy v5 TUI command grammar; the new CLI subcommands stay in v4 CLI style (matches `cmd_generate.rs` shape).
  - Do **not** read any "dashboard cache" or "latest pointer" file.
- **Done definition**: all eight §4.3 boxes (1 sign-off N/A).

---

### C10 — Prompt promotion receipt + runtime guard

- **Atom ID**: C10
- **Phase tag**: P9
- **Title**: Promote v1 → v2/v3 prompts only via a CAS-anchored `PromptPromotionReceipt`; runtime guard refuses to start LLM without one.
- **Predecessor**: C9
- **Goal**: `turingos llm prompt-eval --from <v1> --to <v2>` runs both prompts against a CAS-anchored eval set, emits a receipt with before/after eval CIDs. Receipt presence + matching content gates LLM startup.
- **Why it matters**: PR #11 ship-scope caveat. Production admission gate for canonical prompt change.
- **Risk class**: **3** (application-level admission rule; not sequencer admission, but production gate)
- **FC trace**: FC2 (prompt boot), FC3 (eval evidence binding)
- **Pre-action gates**:
  - `/runner-preflight`
  - **§8 sign-off pre-impl** (Class 3) — scope MUST cite production-admission impact + env-var-bypass forbidden
  - Post-impl clean-context Codex audit (binding)
- **Allowed files (write)**:
  - `src/runtime/prompt_promotion.rs` (new)
  - `src/runtime/mod.rs` (re-export only)
  - `src/bin/turingos/cmd_llm.rs` (additive)
  - `tests/cmd_llm_prompt_eval_v1_vs_v2_triage.rs` (new)
  - `tests/prompt_promotion_requires_eval_receipt.rs` (new)
  - `tests/canonical_prompt_change_has_promotion_receipt.rs` (new)
  - `tests/prompt_promotion_guard_not_bypassable_by_env_var.rs` (new — audit Agent 3 F.7)
  - `tests/prompt_promotion_eval_set_cid_anchored.rs` (new)
- **Forbidden files (write)**:
  - all forbidden surfaces from §3.1
  - prompt template files (modification by promotion only — via reference, not overwrite)
  - `frontend/**`
- **Schema spec**:
  ```rust
  // src/runtime/prompt_promotion.rs
  pub const PROMPT_PROMOTION_RECEIPT_SCHEMA_ID: &str = "turingos-prompt-promotion-v1";

  #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
  #[serde(rename_all = "lowercase")]
  pub enum PromotionDecision { Promote, Reject }

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
  pub struct PromptPromotionReceipt {
      pub schema_id: String,                       // = "turingos-prompt-promotion-v1"
      pub from_prompt_cid: String,
      pub to_prompt_cid: String,
      pub eval_set_cid: String,                    // anchors which problem set was used (audit Agent 3 C.5)
      pub eval_before_cid: String,                 // transcript CID for from_prompt
      pub eval_after_cid: String,                  // transcript CID for to_prompt
      pub promotion_decision: PromotionDecision,
      pub logical_t: u64,
  }
  ```
- **Implementation steps**:
  1. `cmd_llm prompt-eval --from <v1> --to <v2> --eval-set <cid>`: run both prompts against the eval set, write each transcript to CAS, emit receipt.
  2. Runtime guard: when canonical prompt CID changes (cid mismatch between filesystem prompt and last-known promoted CID), the LLM startup path checks CAS for a receipt with `promotion_decision == Promote` matching the new CID.
  3. **No env-var bypass**: the guard MUST NOT honor any environment variable that disables it. Test enforces this by setting `TURINGOS_BYPASS_PROMOTION_GUARD=1` and asserting the guard still blocks.
  4. Rollback: a new receipt with `to_prompt_cid == previous_canonical` and decision `Promote` reverts.
- **Acceptance commands**:
  ```bash
  cargo test --test cmd_llm_prompt_eval_v1_vs_v2_triage
  cargo test --test prompt_promotion_requires_eval_receipt
  cargo test --test canonical_prompt_change_has_promotion_receipt
  cargo test --test prompt_promotion_guard_not_bypassable_by_env_var
  cargo test --test prompt_promotion_eval_set_cid_anchored
  cargo test --workspace --no-fail-fast
  ```
- **Pass criteria**:
  - Direct overwrite of v1 prompt without a matching receipt fails the runtime guard.
  - Receipt presence flips the guard; receipt removal flips it back.
  - Env-var bypass attempts fail; guard remains active.
  - Receipt always carries `eval_set_cid` (no anonymous eval).
- **Kill criteria**: guard honors any env-var bypass · receipt accepted without `eval_set_cid` · v2 prompt files overwritten outside the promotion path.
- **Anti-drift notes**:
  - Do **not** hand-promote v2/v3 in this atom (only the gate is built).
  - Do **not** delete v1 prompt files.
- **Done definition**: all eight §4.3 boxes + §8 sign-off + Codex verdict ≠ VETO.

---

### C11 — Spec-derived TestRunCapsule

- **Atom ID**: C11
- **Phase tag**: P10
- **Title**: Build a minimal `TestScenarioSet` from the spec and run it post-generate, capturing a `TestRunCapsule`. Accepted delivery requires `overall_pass = true`. Acceptance is **not** wired into sequencer admission.
- **Predecessor**: C10
- **Goal**: After `turingos generate`, a TestRun executes spec-derived scenarios (entrypoint exists, HTML parses, sandbox preserved). Hidden-oracle: scenario set bytes never appear in any generation prompt.
- **Why it matters**: Closes spec → artifact → test loop. Per v5 hidden-oracle rule, scenarios stay shielded.
- **Risk class**: **3** (delivery acceptance gate + hidden-oracle shielding)
- **FC trace**: FC1 (test loop), FC3 (test evidence)
- **Pre-action gates**:
  - `/runner-preflight`
  - **§8 sign-off pre-impl** (Class 3) — scope MUST forbid `current_status` wiring into sequencer admission
  - Post-impl clean-context Codex audit (binding)
- **Allowed files (write)**:
  - `src/runtime/test_scenario.rs` (new)
  - `src/runtime/test_run.rs` (new)
  - `src/runtime/mod.rs` (re-export only)
  - `src/runtime/build_session_view.rs` (add `Accepted` variant + `accepted_delivery` derivation rule)
  - `src/bin/turingos/cmd_generate.rs` (additive)
  - `src/web/generate.rs` (additive)
  - `tests/test_scenario_set_from_spec_acceptance.rs` (new)
  - `tests/test_run_capsule_replayable.rs` (new)
  - `tests/hidden_oracle_not_in_generation_prompt_bytes.rs` (new — strengthened from Agent 3)
  - `tests/hidden_oracle_set_cid_not_in_build_session_view.rs` (new)
  - `tests/accepted_delivery_requires_passing_test_run.rs` (new)
  - `tests/accepted_status_not_wired_to_sequencer_admission.rs` (new — Agent 3 A.C11)
- **Forbidden files (write)**:
  - all forbidden surfaces from §3.1 (especially `src/state/sequencer.rs`)
  - any browser-automation framework introduction
  - `frontend/**`
- **Schema spec**:
  ```rust
  // src/runtime/test_scenario.rs
  pub const TEST_SCENARIO_SET_SCHEMA_ID: &str = "turingos-test-scenario-set-v1";

  // Trimmed to producer-bound variants only (audit Agent 1 #2)
  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
  #[serde(tag = "kind", rename_all = "snake_case")]
  pub enum TestScenario {
      EntrypointExists,
      HtmlParses,
      SandboxPolicyPreserved { policy: String },
  }
  // FUTURE (v2 schema_id bump when producers ship): RequiredTextPresent, RequiredControlPresent,
  // NoExternalNetwork, MinimumBar, GameShapeIfSpecSaysGame. Do NOT reserve in v1.

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
  pub struct TestScenarioSet {
      pub schema_id: String,                       // = "turingos-test-scenario-set-v1"
      pub spec_capsule_cid: String,
      pub scenarios: Vec<TestScenario>,
      pub logical_t: u64,
  }

  // src/runtime/test_run.rs
  pub const TEST_RUN_CAPSULE_SCHEMA_ID: &str = "turingos-test-run-v1";

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
  pub struct TestScenarioResult {
      pub scenario: TestScenario,
      pub pass: bool,
      pub detail: String,
  }

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
  pub struct TestRunCapsule {
      pub schema_id: String,                       // = "turingos-test-run-v1"
      // NOTE: no self-cid
      pub artifact_bundle_cid: String,
      pub test_scenario_set_cid: String,           // separate per v5 hidden-oracle pattern
      pub results: Vec<TestScenarioResult>,
      pub overall_pass: bool,                      // = all results.pass
      pub logical_t: u64,
  }
  ```
- **Implementation steps**:
  1. Derive `TestScenarioSet` from spec acceptance section; write the set to CAS (separate capsule, separate CID).
  2. Run the set against the just-generated `ArtifactBundle` (read files from CAS by bundle CID; do NOT touch filesystem `artifacts/`).
  3. Write `TestRunCapsule` referencing both `artifact_bundle_cid` and `test_scenario_set_cid`.
  4. `cmd_generate` exit code non-zero if `overall_pass = false`.
  5. `generate.rs` returns `test_run_cid` + `overall_pass` in response.
  6. C7's `BuildSessionView` gains an `Accepted` `BuildStatus` variant, derived as `overall_pass = true` from the latest TestRunCapsule.
  7. **Hidden-oracle invariant**: scenario set bytes (the JSON-serialized `TestScenarioSet`) MUST NOT appear as a substring inside any C2 `GenerationAttemptCapsule`'s `prompt_hash`-input bytes (the canonical prompt). Test asserts this with a memcmp-style search.
  8. **Anti-wire invariant**: `BuildSessionView.current_status == Accepted` MUST NOT flow into any `src/state/sequencer.rs` admission rule. Static test greps `src/state/sequencer.rs` for `BuildStatus::Accepted` or `current_status` references — any hit fails the test.
- **Acceptance commands**:
  ```bash
  cargo test --test test_scenario_set_from_spec_acceptance
  cargo test --test test_run_capsule_replayable
  cargo test --test hidden_oracle_not_in_generation_prompt_bytes
  cargo test --test hidden_oracle_set_cid_not_in_build_session_view
  cargo test --test accepted_delivery_requires_passing_test_run
  cargo test --test accepted_status_not_wired_to_sequencer_admission
  cargo test --features web --test cli_web_verify_smoke
  cargo test --features web --test cli_web_generate_smoke
  cargo test --workspace --no-fail-fast
  bash scripts/run_constitution_gates.sh
  ```
- **Pass criteria**:
  - One `TestRunCapsule` per generate.
  - `accepted_delivery` derivation requires `overall_pass = true`.
  - Scenario set bytes do not appear in any generation prompt.
  - `BuildStatus::Accepted` never read by sequencer.
  - Trimmed enum has no unused variants.
- **Kill criteria**: scenario set leaks into prompt · `Accepted` wired into admission · headless browser introduced · self-CID added to schema · enum variants reserved without producer.
- **Anti-drift notes**:
  - Do **not** introduce a headless browser framework — reuse the existing heuristic verifier.
  - Do **not** make scenarios visible to the generator prompt.
  - Do **not** auto-retry on TestRun failure here (retry is a future atom).
  - Do **not** wire `BuildStatus::Accepted` into `src/state/sequencer.rs`.
- **Done definition**: all eight §4.3 boxes + §8 sign-off + Codex verdict ≠ VETO.

---

## 8. Anti-drift global blacklist

Architect's "do not" callouts spanning all atoms. Every flash agent reads this
section before starting.

- Do **not** start a new `turingosv5` / `turingosv6` repo.
- Do **not** write to anything under `/home/zephryj/projects/turingosv5/`.
- Do **not** port v5 TUI command grammar / welcome screen / MetaAiConfig / DeepSeek config surface into v4. v4 CLI stays canonical.
- Do **not** rewrite the v4 web frontend.
- Do **not** introduce Next.js / Tauri / LangGraph / CrewAI / Agents SDK.
- Do **not** split the cargo workspace.
- Do **not** introduce microservices, queues, brokers, pub/sub.
- Do **not** port the v5 `src/devtool` into v4 runtime.
- Do **not** add a parallel CAS, WAL, or HEAD pointer.
- Do **not** modify `constitution.md` or `src/bottom_white/cas/schema.rs`.
- Do **not** build agent market / wallet / dashboard surfaces in this charter.
- Do **not** declare ship by contract presence (must have evidence).
- Do **not** delete the legacy `/api/artifact/:session_id/:name` route.
- Do **not** hand-promote prompt v2/v3 over v1.
- Do **not** retroactively rewrite or migrate existing evidence.
- Do **not** put self-CIDs inside capsule bodies.
- Do **not** reserve schema fields for unimplemented producers.
- Do **not** introduce `Manager / Factory / Engine / Platform / Framework`
  abstractions.
- Do **not** add a background loop or daemon without naming a physical
  bottleneck.

## 9. Orchestrator dispatch protocol

Dispatch order: C0, C1 (parallel) → C2 (**§8**) → C3 (**§8**) → C4 → C5 → C6
→ C7 → C8 (**§8**) → C9 → C10 (**§8**) → C11 (**§8**).

### 9.1 Per-atom prompt template (mandatory shape)

Every flash agent receives a prompt built from this template — verbatim:

```
You are implementing atom [ATOM_ID] from the V4 Product-CAK Hardening
Execution Plan dated 2026-05-20.

## Mandatory implementer discipline

Before writing any code, read in this order:
  1. /home/zephryj/projects/turingosv4/skills/KARPATHY_SIMPLE_CODE.md
  2. The atom spec below (verbatim, from §7 of the execution plan)
  3. /home/zephryj/projects/turingosv4/AGENTS.md §5-§8, §12

You MUST end your work by answering the Karpathy Simple Code Worker
Checklist in your PR body, with one sentence per question:

  - Did I add a dependency? If yes, was it explicitly allowed?
  - Did I add an abstraction? If yes, what real boundary does it protect?
  - Did I change files outside the TaskPacket?
  - Can the data flow be explained as input -> transform -> output?
  - Could this be a smaller direct function?
  - Did tests prove the behavior?

If you cannot answer "yes" or a clean justification to all six, stop and
report back instead of submitting.

## Pre-action gates (do these first)

[FOR CLASS >= 2]: Invoke /runner-preflight. Report its output verbatim.
[FOR CLASS >= 3]: Wait for §8 sign-off citing the diff SHA pre-signature.
                  Do NOT start writing code before §8 is recorded.

## Atom spec (verbatim from §7)

[INSERT FULL ATOM SPEC HERE — fields 1..18 unmodified]

## Hard boundaries

Allowed files (write): [INSERT FROM SPEC]
Forbidden files (write): [INSERT FROM SPEC]
Forbidden surfaces (global): see §3.1 of the execution plan.

Before every Edit / Write tool call:
  - Confirm the target path is in "Allowed files (write)"
  - Confirm it is NOT in "Forbidden files (write)" or §3.1

If you are about to touch anything restricted, stop and escalate. Do not
attempt a creative workaround.

## Output contract

When you believe the atom is done, your final message MUST contain:
  - The diff (or PR URL)
  - The output of every command in "Acceptance commands"
  - A line "FC-trace: <FC1-N? | FC2-N? | FC3-N?>" matching the atom's FC trace
  - The Karpathy Worker Checklist answered (6 questions)
  - Capsule CIDs written (if any)
  - [CLASS 3 only] The diff SHA used for §8 sign-off

If any acceptance command fails, run `git restore .`, report the failure,
and wait for orchestrator instruction. Do NOT proceed to the next atom.
```

### 9.2 Orchestrator-side enforcement

Per atom, the orchestrator:

1. Builds the prompt above by substituting `[ATOM_ID]`, the verbatim spec, and
   the class-conditional gate clauses.
2. For Class 3 atoms (C2, C3, C8, C10, C11), pauses to obtain user §8
   sign-off with the diff SHA captured pre-signature.
3. Dispatches to a flash agent.
4. On return, runs every "Acceptance commands" line as a verification step
   (does NOT trust the agent's pasted output).
5. Greps the diff against "Allowed files (write)" and the §3.1 forbidden list.
6. Verifies the commit message contains `FC-trace: …`.
7. Ticks the eight §4.3 boxes.
8. For Class 3, dispatches the post-impl clean-context Codex audit with the
   diff + acceptance output + relevant source. Required output domain:
   `PROCEED | CHALLENGE | VETO`.
9. Advances to the next atom only when boxes 1-8 pass.

### 9.3 Failure / rollback policy

- Acceptance command failure → `git restore .` → re-dispatch the atom.
- Forbidden-file touch → `git restore .` → re-dispatch with a stricter prompt.
- Karpathy Worker Checklist failure → block; require a follow-up commit
  addressing each "no" answer.
- §8 diff-SHA mismatch (signature collected on diff A, agent submitted diff
  B) → block; require fresh §8 against diff B.
- Codex audit `VETO` → block ship; route to a remediation atom.
- Codex audit `CHALLENGE` → fix or explicit forward-defer with rationale
  recorded in the next atom's predecessor notes.

## 10. Audit ledger (2026-05-20)

| Auditor                 | Verdict                | Key findings folded                                              |
|-------------------------|------------------------|------------------------------------------------------------------|
| v4-Karpathy (Agent 1)   | 85% aligned, 5 fixes   | C6 unused log CIDs (cut); C11 enum trim; C9 tracing→static; C3 filesystem pointer (cut); TestScenarioSet kept separate |
| v5-reuse-port (Agent 2) | Adopt 4, reject 6      | Adopted: role enum, path regex, cross-field invariant, immutability, L4.E 4-tuple. Rejected: MetaAiConfig, TUI welcome, opaque-CID convention, in-source schemas, mutation-on-edit, hardcoded timestamps |
| atom-rigor (Agent 3)    | 5 blockers, 5 nice     | Self-CID circularity (dropped fields), C2/C3/C10 risk class bumped to 3, C2 outcome enum added, world-head operational def added, C5 namespace shielding added |

## 11. References

- Architect directive (in-conversation, 2026-05-20).
- `CLAUDE.md` (Claude Code adapter).
- `AGENTS.md` (shared harness contract — §5 risk, §7 commands, §8 dirty tree, §14 cadence).
- `constitution.md` (FC1 / FC2 / FC3, Art.0–V).
- `skills/KARPATHY_ARCHITECT.md` (design protocol applied to this plan).
- `skills/KARPATHY_SIMPLE_CODE.md` (mandatory in every flash-agent prompt — §9.1).
- `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` (phase axis).
- `handover/architect-insights/TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md` (atom lifecycle).
- `handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md` (per-atom field precedent).
- `src/runtime/spec_capsule.rs:83-106,113-135` (canonical EvidenceCapsule writer + latest-lookup pattern).
- `turingosv5/schemas/v5_dev/artifact_bundle.schema.json` (field set, role enum, path regex — adopted).
- `turingosv5/docs/contracts/friendly_error_l4e.md` (L4.E 4-tuple — adopted).
- `turingosv5/docs/contracts/edit_regenerate_versioning.md` (immutability rule — adopted).
- `turingosv5/src/devtool/mod.rs:380-541` (TUI / MetaAiConfig — rejected; do not copy).
- PR #6 (TISR Phase 7 Web MVP), PR #11 (Phase 6.3 grill, deferred A10).

---

End of execution plan. On dispatch, the orchestrator loads §7 atom specs one
at a time, builds prompts per §9.1, and enforces §9.2 / §9.3.
