# Architect ratification request — OBS_R022 Global LATEST_MARKOV pointer = Art. 0.2 平行账本

**Filed**: 2026-05-04 (TB-16.x.2.1 fourth session)
**Filed by**: Claude (Opus 4.7) under user instruction `我不要凑活，我要严格对齐宪法和宪法中的三个 flowchart`
**Severity**: constitutional (Art. 0.2 violation)
**Sudo level**: requires architect ratification per Art. V.1.3 (vetoAI scope) — Claude will not delete, rename, or refactor pointer-related surfaces until architect rules
**Authoritative OBS**: `handover/alignment/OBS_R022_GLOBAL_LATEST_MARKOV_PARALLEL_LEDGER_2026-05-04.md` (read top-to-bottom; this request is the wrapper)
**Repo state at filing**: branch `main` at commit `fab2977`; pushed to `origin/main`; both `e986ed0` (TB-16.x.2.1 sub-atom ship) + `fab2977` (strict-alignment patch withdrawing prior framing) are durable.

---

## §1 What you are being asked to rule on

**Question 1** (the Art. 0.2 question — REQUIRED for ratification):
Is `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt` a tolerable convenience pointer, or is it a 平行账本 (parallel ledger) that must be deleted / re-anchored to bring the project back into Art. 0.2 (Tape Canonical 公理) compliance? If the latter, which of the three candidate rulings (α / β / γ in §6) — or your own variant — is the project's path forward?

**Question 2** (the framing question — REQUIRED to confirm Claude's analysis):
Is Claude's strict-alignment analysis correct? Specifically:
- (Q2.a) Markov capsule is NOT a flowchart node in FC1 / FC2 / FC3 — confirm or correct.
- (Q2.b) Markov chain genesis is `previous_capsule_cid: None` — therefore TB-16.x.2.1's smoke run on a fresh `runtime_repo` + fresh `cas` is constitutionally a genesis chain, and `markov_capsule = None` is the unique constitutionally correct state, with 7 Layer G `Skipped` assertions being CORRECT (not bypassed). Confirm or correct.
- (Q2.c) `audit_tape --markov-pointer <absent path>` is the public API for "no inherited Markov" per `src/runtime/audit_assertions.rs:421-425`. Using it for a fresh genesis chain is NOT a workaround. Confirm or correct.

**Question 3** (the lineage question — REQUIRED for next sub-atom):
Sub-atoms 2.4 (Multi-WorkTx-attempt + Boltzmann RUNTIME) and 2.6 (Combined arena run, all 4 tx kinds + Boltzmann + Autopsy in one chain) per umbrella charter `TB-16.x.2_charter_2026-05-04.md` will eventually run **multiple tasks against one continuing chain** (vs. R3 Round 2's per-problem isolated chains). When tape continuation actually lands, what is the constitutional Markov inheritance rule?
- (Q3.a) Each task's accepted txs are appended to a single `runtime_repo` + single `cas`; `previous_capsule_cid` is read from the same CAS (Art. 0.4 path B in-tape).
- (Q3.b) Each task remains in its own per-problem repo, and Markov inheritance is achieved by **importing** the prior per-problem CAS bytes into the next per-problem CAS at boot (per-tape but with explicit import).
- (Q3.c) Other shape (please specify).

**Question 4** (Phase Z′ trigger — REQUIRED to confirm scope):
Does any of α / β / γ require Phase Z′ 6-stage rerun (constitution flowchart modification per Art. V.3)? Claude's reading: no — none of α / β / γ touches FC1 / FC2 / FC3 since Markov capsule is not a flowchart node. But this is exactly the kind of judgment vetoAI should validate.

---

## §2 Problem statement (one paragraph)

`handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt` is a single-line file containing one cid hex. It is **read** as a `--markov-pointer` argument by `src/bin/audit_tape.rs` and `src/bin/audit_tape_tamper.rs` to determine which Markov capsule a given audit run validates Layer G against. It is **written** by `src/bin/generate_markov_capsule.rs` whenever that binary runs with a `--out-dir handover/markov_capsules/`. The cid it currently holds (`f9e701b4...`) is the Markov capsule generated at the END of TB-16 R3 Round 2 P8_completeset_b's run; the cid's bytes (the actual capsule json + metadata) live exclusively in `handover/evidence/tb_16_real_llm_arena_2026-05-04/post_r3_round2/P8_completeset_b/cas/.git/objects/...`. **No global / project-level CAS holds those bytes.** Any fresh isolated CAS (such as TB-16.x.2.1's smoke run on aime_1997_p9) cannot resolve the pointer. This is — by Art. 0.2's own enumeration of forbidden patterns ("`RunCostAccumulator` / `WalletTool` / `search_cache` / `LibrarianTool` / `bus.graveyard` / FC trace 等") — a 平行账本: a global filesystem-side state, carrying cross-chain provenance, with no derivability from any tape and no `assert_eq!(view, derive_from_tape(tape))` 守恒 test.

---

## §3 What surfaced this (timeline)

| Timestamp | Event | Reference |
|---|---|---|
| 2026-05-03 | TB-15 ships Lamarckian Autopsy + Markov EvidenceCapsule. `generate_markov_capsule` writes both per-run capsule json AND `--out-dir/LATEST_MARKOV_CAPSULE.txt`. | commit `2337381`, `src/bin/generate_markov_capsule.rs:376-387` |
| 2026-05-04 (early) | TB-16 Atom 6 (controlled market arena) ships. `audit_tape --markov-pointer` reads the global file. Pre-existing `read_markov_capsule(...).ok()` collapse silently masks pointer-CAS mismatches as `Skipped`. | commit `3cd22d4`, `src/runtime/audit_assertions.rs:421` (pre-fix) |
| 2026-05-04 (mid) | TB-16.x.1 (audit-pipeline hardening) replaces `.ok()` collapse with fail-closed BLOCK on "pointer present but cid unresolvable". CORRECT fix; closes false-PROCEED post-tamper bypass per Codex VETO. | commit `3735484` |
| 2026-05-04 (late, this session) | TB-16.x.2.1 smoke run (fresh isolated CAS) + post-fix audit_tape → BLOCK on `cid:f9e701b4... not found in CAS index`. Claude initially shipped a "use absent pointer + OBS deferred to 2.6" framing. User rejected: "我不要凑活，我要严格对齐宪法和宪法中的三个 flowchart". | this OBS |

The pre-fix masking explains why `markov_constitution_hash_matches: Skipped` shows up across **all 8** of TB-16 R3 Round 2's per-problem verdict.json files — that wasn't legitimate "no prior markov"; it was the masked failure of this exact pointer-CAS mismatch. CR-15.6 (Markov default prevents context poisoning) was bypassed because the pointer's existence was *de facto* the source of truth, not the capsule bytes.

---

## §4 Files / surfaces to audit

### Primary (Art. 0.2 violation surface)

| Path | Why audit |
|---|---|
| `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt` | The pointer file itself; the violation surface. |
| `handover/markov_capsules/MARKOV_TB-15-R3_2026-05-03.json` | Per-TB capsule json kept alongside the pointer; review whether these are reference inputs or historical artifacts. |
| `src/bin/generate_markov_capsule.rs:376-387` | Writes `--out-dir/LATEST_MARKOV_CAPSULE.txt` (the global write path). |
| `src/bin/audit_tape.rs:53,65,97,122,160` | `--markov-pointer` declared REQUIRED. |
| `src/bin/audit_tape_tamper.rs:52,108,119,183` | Mirrors `audit_tape`. |
| `src/runtime/audit_assertions.rs:421-425` | The `if pointer.exists() else None` branch — the fork point. |
| `src/runtime/audit_assertions.rs:519-543` | `read_markov_capsule` — does the cid resolution against the per-run CAS. |
| `src/runtime/audit_assertions.rs:2002-2030` | `assert_32_markov_constitution_hash_matches` — the prototype Layer G assertion that goes Skipped vs Pass vs Halt. |
| `src/runtime/markov_capsule.rs:53-110, 211-330` | MarkovEvidenceCapsule schema, `previous_capsule_cid: Option<Cid>` (the genesis flag), `write_markov_capsule`, deep-history gate. |
| `genesis_payload.toml` (lines covering audit_tape* + generate_markov_capsule) | If α / β touches binaries, Trust Root rehash required per R-014. |

### Constitutional anchors (to verify Claude's analysis)

| Path | Section | Why |
|---|---|---|
| `constitution.md:52-95` | Art. 0.2 Tape Canonical 公理 | The violated axiom; rule #2 names "平行账本" pattern. |
| `constitution.md:114-152` | Art. 0.4 Q_t version-controlled (path A/B/C) | Path B (real git substrate) is the proper home for Markov inheritance via `tape_t` continuation. |
| `constitution.md:455-509` | FC1 (anti-oreo runtime) | NO Markov capsule node — verify. |
| `constitution.md:571-660` | FC2 (Boot/Init) | NO Markov capsule node — verify. |
| `constitution.md:826-870` | FC3 (Meta-architecture) | NO Markov capsule node — verify. |
| `constitution.md:704-760` | Art. V.1.1-1.3 (vetoAI / ArchitectAI / constitution as ground truth) | Sudo authority for this ratification. |

### Forward-pointing anchors (sub-atoms 2.4 / 2.6)

| Path | Why |
|---|---|
| `handover/tracer_bullets/TB-16.x.2_charter_2026-05-04.md` | Umbrella charter (the multi-task chain continuation home; sub-atom 2.4 Boltzmann RUNTIME, sub-atom 2.6 comprehensive). |
| `handover/tracer_bullets/TB-16.x.1_charter_2026-05-04.md` | Predecessor (the audit-hardening fix that surfaced this). |

### Smoke evidence (to confirm sub-atom 2.1 is itself constitutionally clean)

| Path | Signal |
|---|---|
| `handover/evidence/tb_16_x_2_1_smoke_2026-05-04/P9_force_expire/runtime_repo/run_summary.json` | accepted_tx_ids includes `system-task-expire-1-4` |
| `handover/evidence/tb_16_x_2_1_smoke_2026-05-04/P9_force_expire/verdict.json` | tx_kind_counts.task_expire = 1; verdict=PROCEED 34/0/0/7 |
| `handover/evidence/tb_16_x_2_1_smoke_2026-05-04/P9_force_expire/verdict_replay.json` | byte-identical to verdict.json (replay determinism) |
| `handover/evidence/tb_16_x_2_1_smoke_2026-05-04/P9_force_expire/tamper_report.json` | 3/3 detect |
| `handover/evidence/tb_16_x_2_1_smoke_2026-05-04/P9_force_expire/dashboard.txt` | L4 row 4: `TaskExpire | Agent_user_0`; "Expired tasks (TaskExpireTx; capital released)" populated |
| `handover/evidence/tb_16_x_2_1_smoke_2026-05-04/P9_force_expire/README.md` | Constitutional framing applied (post-patch `fab2977`) |

### Existing OBS files (pattern + cross-references)

| Path | Why |
|---|---|
| `handover/alignment/OBS_R022_TB16_TAMPER_BACKLINKS_2026-05-04.md` | Pattern for prior R022 OBS. |
| `handover/alignment/OBS_R022_TB14_PRICEINDEX_REMOVED_2026-05-03.md` | Pattern: removing a "stub field with no flowchart role" is precedent. |
| `handover/alignment/OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md` | Pattern for security-class OBS. |
| `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md` (and rev v1..v3) | The authoritative flowchart-element ↔ code mapping; verify `LATEST_MARKOV_CAPSULE.txt` is NOT a TRACE_MATRIX-anchored element. |

---

## §5 The constitutional analysis Claude believes is correct (please confirm or correct)

### §5.1 Markov capsule is not a flowchart node

Read FC1 (`constitution.md:455-509`), FC2 (lines 571-660), FC3 (lines 826-870). No node is "MarkovEvidenceCapsule" or any obvious alias. The capsule is introduced by TB-15 (`src/runtime/markov_capsule.rs`) as a 派生视图 — its own source comment (CR-15.5) declares: `evidence compression, not hidden source of truth — every field is derivable from the chain + CAS at generation time`. This places the capsule under Art. 0.2 rule #2 (派生视图 with 守恒 obligation), not as a flowchart node with its own ground-truth status.

### §5.2 Genesis is `previous_capsule_cid: None`

`src/runtime/markov_capsule.rs:60` defines `pub previous_capsule_cid: Option<Cid>`. Line 111 sets the default `None`. This is the structural definition of "genesis Markov capsule" — i.e. a Markov chain head with no predecessor.

A fresh isolated chain (fresh `runtime_repo` + fresh `cas`, no inherited tape) trivially satisfies "no `previous_capsule_cid`" — there's no prior capsule the chain references at all. **Genesis chain ≡ no inherited Markov.** This is a definitional consequence of the `previous_capsule_cid: Option<Cid>` schema, not an implementation choice.

### §5.3 `audit_tape --markov-pointer <absent>` is the API for genesis

`src/runtime/audit_assertions.rs:421-425`:
```rust
let markov_capsule = if inputs.markov_pointer.exists() {
    Some(read_markov_capsule(&inputs.markov_pointer, &cas)?)
} else {
    None
};
```

Pointer-absent → `markov_capsule = None`. This is the fork point. Per Markov-chain genesis (§5.2), the genesis case naturally selects this branch. The 7 Layer G assertions (`assert_32_markov_constitution_hash_matches` and siblings) all branch on `match &t.markov_capsule { Some(c) => ..., None => Skipped }` (e.g. `audit_assertions.rs:2003-2012`). Skipped here is **constitutional**, not bypassed.

### §5.4 The Art. 0.2 violation IS the global pointer

`handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt` matches every characteristic Art. 0.2 names as 平行账本:
- Filesystem-side global (not in any tape)
- Holds info that crosses chain boundaries (last-writer-wins; readers use it without provenance)
- Bytes it points at live in **one specific** per-problem CAS, not derivable from any other tape
- No `assert_eq!(view, derive_from_tape(tape))` 守恒 test
- Last-writer-wins lifecycle (`generate_markov_capsule --out-dir handover/markov_capsules/` overwrites)

Pre-TB-16.x.1, the `.ok()` collapse made this masked-broken. TB-16.x.1 made it observable. The brokenness itself is what needs ratification.

---

## §6 Three candidate rulings (Claude proposes; architect picks or amends)

### Option α — Delete + de-canonicalize (cheapest, ~half day)

- DELETE `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt`.
- Either DELETE or relabel `handover/markov_capsules/MARKOV_TB-*.json` as historical-artifact-only (NOT reference inputs).
- Make `--markov-pointer` OPTIONAL in `audit_tape` / `audit_tape_tamper` (currently required by `audit_tape.rs:122`).
- Add new arg `--prior-chain-runtime-repo <path>` to `audit_tape` / `audit_tape_tamper`. Resolution: read `<prior>/runtime_repo`'s last commit, locate its capsule cid (in-tape via the prior chain's CAS), validate against current chain.
- DELETE the `--out-dir/LATEST_MARKOV_CAPSULE.txt` write path in `generate_markov_capsule.rs:385-387`. Per-run capsule json may still be written for human convenience (clearly labeled non-canonical).
- ADD a `tests/markov_pointer_no_global_parallel_ledger.rs` 守恒 test: assert `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt` does not exist and `--markov-pointer` is not required.

**Pros**: simplest constitutional path; removes violation directly; convenience-pointer use case is rare and architect can pass `--prior-chain-runtime-repo handover/evidence/tb_15_*/runtime_repo` explicitly.

**Cons**: small ergonomic loss; touches 4 binaries; one test addition.

### Option β — Anchor in Art. 0.4 path B (chain continuation, longer term)

- Sub-atoms 2.4 / 2.6 of umbrella `TB-16.x.2` already commit to multi-task SINGLE-CHAIN runs (vs. R3 Round 2 per-problem isolated). Per Art. 0.4 path B, chain continuation = each new task's `runtime_repo` extends the prior task's git history (NOT separate repos).
- Under continuation, the prior task's Markov capsule is in the SAME CAS as the current task — no global pointer needed. Inheritance is structurally encoded in `tape_t` itself.
- KEEP `LATEST_MARKOV_CAPSULE.txt` only as a CONVENIENCE pointer with explicit README disclaimer "non-canonical, fresh chains MUST ignore". `audit_tape --markov-pointer` expects an in-tape cid (audit_tape verifies the cid is in the supplied `--cas-dir`).

**Pros**: matches Art. 0.4 path B "Phase E gate" architecture; constitutional fidelity highest; defers the simple "delete pointer" fix to when chain continuation lands.

**Cons**: requires sub-atom 2.4 / 2.6 to mature (1-2 days additional); `audit_tape` patch (~half day); leaves the global pointer in place during the interim (relabeled non-canonical but still present).

### Option γ — Status quo + 守恒 sidecar (least intrusive, weakest)

- Keep `LATEST_MARKOV_CAPSULE.txt` as-is.
- Add `handover/markov_capsules/LATEST_MARKOV_PROVENANCE.json` written at every `generate_markov_capsule` invocation, recording `{cid, source_chain_runtime_repo, source_chain_head_commit, written_at, written_by_run_id}`.
- `audit_tape` reads BOTH and verifies the source chain is a legitimate predecessor of the chain being audited.

**Pros**: adds 守恒 chain Art. 0.2 demands without architectural rework; ~half day.

**Cons**: adds a SECOND parallel-ledger to police the FIRST one — a code smell on Art. 0.2 grounds. Recommended only if β is multi-month away AND α is somehow blocked.

### Claude's recommendation

**α immediate, β long-term home.** Rationale: α removes the violation cleanly with low ergonomic cost; β is the constitutional ideal but requires sub-atom 2.4 / 2.6 to mature; γ adds parallel state to police parallel state which is itself the smell.

But Claude recognizes this is a Sudo decision — the architect may prefer (β) NOW with the understanding that sub-atom 2.4 / 2.6 will land within the umbrella's 3-5 day estimate, OR may prefer a variant Claude hasn't enumerated.

---

## §7 What Claude needs back from the architect to proceed

To unblock sub-atom 2.2 (ChallengeResolve, Class 3, dual-audit required, STEP_B_PROTOCOL TRIGGERED — touches `src/state/sequencer.rs`), Claude needs at minimum:

1. **Confirmation of §5 framing** (Q2.a-c above).
2. **Pick of α / β / γ / variant** (Q1).
3. **Phase Z′ scope ruling** (Q4) — does the chosen option require Phase Z′ rerun?
4. **Authorization scope**: may Claude execute the chosen option's source edits as a follow-up patch, or does this require dual-audit (Codex + Gemini) per `feedback_dual_audit` Class 2 (audit pipeline)?
5. **Optionally**: ruling on Q3 (sub-atom 2.4 / 2.6 chain continuation shape), if architect wishes to fix that direction now rather than at sub-atom 2.4 charter time.

If architect prefers to hold the ratification (e.g. defer until sub-atom 2.6 forces β), Claude will continue sub-atom 2.x work using the absent-pointer pattern (constitutionally correct genesis-chain semantic per §5) and tag every smoke runner with the OBS reference.

---

## §8 Open questions Claude has (lower priority, but useful for future framing)

- (Q5) Is the existence of `handover/markov_capsules/` AS A DIRECTORY itself a code smell — should TB-15's per-run Markov json files live under `handover/evidence/tb_*/markov/` instead, alongside the chain they describe?
- (Q6) Should `MarkovEvidenceCapsule` receive an explicit `/// TRACE_MATRIX FCx-Nyy: 派生视图; not a flowchart node` doc-comment to prevent future drift back into source-of-truth framing?
- (Q7) Pre-TB-16.x.1's `.ok()` collapse means R3 Round 2's `markov_constitution_hash_matches: Skipped` was masked-broken, not legitimate genesis. Should the umbrella retroactively rerun `audit_tape` on R3 Round 2's 8 chains (with the post-fix binary, using absent-pointer per §5) to refresh verdict.json, OR does `feedback_no_retroactive_evidence_rewrite` block this?
- (Q8) Does `assert_eq!(view, derive_from_tape(tape))` 守恒 test get added in scope of α / β / γ, or in a separate dedicated 守恒-test sub-atom?
- (Q9) The Trust Root manifest in `genesis_payload.toml` will need rehash if α / β touches `audit_tape*` / `generate_markov_capsule`. Per R-014 this is non-sudo, but please confirm scope.

---

## §9 What is durable on the repo right now

- `e986ed0` TB-16.x.2.1 — TaskExpire env-var trigger (10-of-13 tx kinds). No reframing needed; the code change is constitutionally clean.
- `fab2977` TB-16.x.2.1 strict-alignment patch — Markov None==genesis per FC2 Boot. Documentation-only; withdraws the prior "OBS deferred / infra gap" framing.
- Both pushed to `origin/main`.
- `handover/alignment/OBS_R022_GLOBAL_LATEST_MARKOV_PARALLEL_LEDGER_2026-05-04.md` is the durable OBS record.
- `handover/architect-insights/REQUEST_OBS_R022_GLOBAL_LATEST_MARKOV_2026-05-04.md` (this file) is the wrapper request.
- All evidence (`handover/evidence/tb_16_x_2_1_smoke_2026-05-04/`) is forward-only. Round 2 evidence (`handover/evidence/tb_16_real_llm_arena_2026-05-04/post_r3_round2/`) is untouched per `feedback_no_retroactive_evidence_rewrite`.

## §10 Summary for vetoAI

If you are vetoAI:
- Verify Claude's claim that Markov capsule appears in NO flowchart (FC1/FC2/FC3) — this is the load-bearing constitutional premise.
- Verify Claude's claim that `previous_capsule_cid: Option<Cid>` with default `None` defines genesis — this is the load-bearing schema premise.
- Verify Claude's claim that `audit_tape --markov-pointer <absent>` is the public genesis-chain API per `audit_assertions.rs:421-425` — this is the load-bearing API premise.
- If all three premises hold, sub-atom 2.1 is constitutionally clean. The OBS is about the global pointer, not about sub-atom 2.1.
- If any premise fails, please specify which and Claude will revise.

If you are ArchitectAI: please pick α / β / γ / variant and authorize execution scope per §7.

If you are the human user: this is the "send to architect" handoff. Please add any architect-only context you want and forward.
