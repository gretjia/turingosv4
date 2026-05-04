# OBS_R022 — `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt` is Art. 0.2 平行账本

**Date discovered**: 2026-05-04 (TB-16.x.2.1 closure session)
**Discovered by**: Claude (Opus 4.7) under user instruction "我不要凑活，我要严格对齐宪法和宪法中的三个 flowchart"
**Severity**: constitutional (Art. 0.2 violation, parallel-ledger pattern)
**Sudo required**: yes — architect must rule on either (α) deletion + relabel or (β) constitutional anchoring path
**Filed by**: Claude session 2026-05-04 fourth (TB-16.x.2.1)

---

## §1 The fact

The file `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt` is a single-line filesystem-side hex pointer used by `audit_tape --markov-pointer` (`src/runtime/audit_assertions.rs:421`) to resolve the Markov capsule cid that an audit run should validate Layer G against.

**Current content** (2026-05-04 14:30 UTC):
```
f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312
```

**Where the bytes for that cid actually live**:
```
handover/evidence/tb_16_real_llm_arena_2026-05-04/post_r3_round2/P8_completeset_b/cas/.git/objects/...
```
(The TB-16 R3 Round 2 P8_completeset_b per-problem isolated CAS — the LAST sub-problem to run `generate_markov_capsule` with `--out-dir` writing the global pointer.)

**Where the bytes are NOT**:
- No global / project-root CAS holds them.
- No other per-problem CAS (P1..P7, P9) holds them.
- No Lustre / blob / shared substrate.

## §2 The Art. 0.2 violation

Art. 0.2 (Tape Canonical 公理) operational rule #2:

> 平行账本（`RunCostAccumulator` / `WalletTool` / `search_cache` / `LibrarianTool` / `bus.graveyard` / FC trace 等）只能是 tape 的派生视图，不可作独立 source of truth；每个派生视图都必须有 `assert_eq!(view, derive_from_tape(tape))` 守恒测试

`LATEST_MARKOV_CAPSULE.txt` is:
- a **filesystem-side global** (not derived from any tape)
- holding a **cid pointing at bytes resident in exactly one per-problem CAS** (not globally CAS-resolvable)
- **with no守恒 test** (`assert_eq!(latest_pointer, derive_from_tape(tape))`)
- **whose lifecycle is "last writer wins"** (`generate_markov_capsule --out-dir`'s last invocation per-session)

This is the **textbook pattern** Art. 0.2 names as 平行账本 (parallel ledger) — it carries information across chains in a way that no single tape can reconstruct.

Compounding factors:

1. **Cross-tape coupling without provenance**: a `audit_tape` run on tape A, asked to validate against `LATEST_MARKOV_CAPSULE.txt` set by tape B's last `generate_markov_capsule` invocation, has no way to verify "is B legitimately my predecessor?" The Markov chain field `previous_capsule_cid: Option<Cid>` (`src/runtime/markov_capsule.rs:60`) **does** encode predecessor relations, but only **inside a capsule's own bytes** — not at the pointer file. The pointer is content-blind.
2. **Hidden source-of-truth**: pre-TB-16.x.1, `read_markov_capsule(...).ok()` collapsed all errors to `None`, masking pointer-CAS mismatches as silent `Skipped`. Round 2 verdict.json shows `markov_constitution_hash_matches: Skipped` — that wasn't legitimate "no prior markov"; it was the masked failure of this very pointer-bytes mismatch. CR-15.6 ("Markov default prevents context poisoning") was bypassed because the pointer's existence was *de facto* the source of truth, not the capsule bytes.
3. **No 守恒 test**: `tests/` has no harness that asserts the global pointer file's cid is derivable from any reference tape. Test `markov_capsule.rs:464+474+732+740` cover capsule round-trip + with_constitution_hash, NOT the pointer file.

## §3 Why TB-16.x.1's fix didn't (and shouldn't) close this

TB-16.x.1 (commit `3735484`) did the **right** thing for adversarial-bytes detection: changed `audit_assertions.rs::load_tape` from silent `.ok()` collapse to fail-closed BLOCK on "pointer present but cid not resolvable". That hardens the **trust** semantic of Layer G — corrupt markov bytes now fail the audit instead of skipping it.

But that fix exposed the **deeper** Art. 0.2 violation: a fresh isolated CAS (e.g. TB-16.x.2.1's smoke run on aime_1997_p9) cannot resolve the global pointer's cid, because the bytes only exist in TB-16 R3 Round 2 P8's CAS. Pre-fix, this was masked as "Skipped (no prior markov)". Post-fix, it's correctly surfaced as "this pointer references bytes I cannot verify" — and the operator must explicitly choose pointer-absent semantics to get past it.

This is why TB-16.x.1's fix is necessary AND why this OBS goes beyond its scope: **TB-16.x.1 made the pointer's brokenness observable; the brokenness itself is Art. 0.2 architectural debt that needs architect ratification**.

## §4 What `audit_tape --markov-pointer <absent path>` actually means

For the avoidance of any "did Claude paper over this" question:

`src/runtime/audit_assertions.rs:421-425`:
```rust
let markov_capsule = if inputs.markov_pointer.exists() {
    Some(read_markov_capsule(&inputs.markov_pointer, &cas)?)
} else {
    None
};
```

Pointer-absent → `markov_capsule = None`. The 7 Layer G assertions then `Skipped` per the standard "no Markov capsule" branch (e.g. `assert_32_markov_constitution_hash_matches` at line 2006). This is the **constitutionally correct outcome for a fresh genesis chain** — the Markov chain definition itself encodes genesis as `previous_capsule_cid: None` (`markov_capsule.rs:111`).

**TB-16.x.2.1's smoke run is constitutionally a genesis chain**: fresh `runtime_repo` + fresh `cas`, with no `previous_capsule_cid` claim anywhere in its bytes. The smoke runner's choice to pass an absent pointer path is the CORRECT expression of "this chain has no inherited Markov" — it is NOT a workaround for the global-pointer brokenness. The brokenness affects sub-atom 2.x evidence ONLY in that the smoke runner cannot use the global pointer (because resolving it would BLOCK), so the smoke runner reasonably and correctly opts out of any claim to inherited Markov.

Where Claude's prior framing was wrong:
- Sub-atom 2.1 LATEST.md called this an "OBS deferred to umbrella sub-atom 2.6 — fix the markov bootstrap" — wrong framing. The per-sub-atom smoke evidence is *correct*; the global pointer is the violation.
- Sub-atom 2.1 README called the absent-path the "workaround" — wrong word. It's the Markov-chain-genesis API.

This OBS supersedes those framings.

## §5 Candidate rulings (architect please pick or amend)

### Option α — Delete + de-canonicalize (cheapest)

- **Delete** `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt`.
- **Delete** `handover/markov_capsules/MARKOV_*.json` files OR explicitly mark them as per-TB historical artifacts (NOT reference inputs to `audit_tape`).
- **Patch** `audit_tape` / `audit_tape_tamper` so `--markov-pointer` is **optional** (not required CLI arg). Currently `audit_tape.rs:122` has `markov_pointer.ok_or("--markov-pointer required")?` — make this `Option<PathBuf>` with `None == genesis chain` semantic.
- **Patch** `generate_markov_capsule` so `--out-dir` writes only PER-RUN markov pointer alongside the per-run capsule json; delete the global write path.
- **Inheritance semantic**: each `audit_tape` invocation declares its prior chain (if any) via a new `--prior-chain-runtime-repo <path>` arg. The capsule cid + bytes are then resolved by reading the prior chain's runtime_repo + traversing its CAS. **This makes the Markov inheritance fully tape-derived (Art. 0.2 守恒 satisfied).**
- **Cost**: ~half day; touches 4 binaries (`audit_tape`, `audit_tape_tamper`, `generate_markov_capsule`, possibly `audit_dashboard`); requires a 守恒 test.
- **Trade-off**: simplest constitutional path; loses the convenience of "I just want to validate against the latest TB's markov" (but architect can still pass `--prior-chain-runtime-repo handover/evidence/tb_15_*/runtime_repo` explicitly).

### Option β — Anchor in Art. 0.4 path B (real chain continuation)

- The umbrella charter `TB-16.x.2_charter_2026-05-04.md` sub-atoms 2.4 / 2.6 already commit to multi-task SINGLE-CHAIN runs. Per Art. 0.4 path B, this means tape continuation: each new task's `runtime_repo` extends the prior task's git history (NOT separate repos).
- Under such continuation, the prior task's Markov capsule **is in the same CAS** as the current task — no global pointer needed. The Markov inheritance is structurally encoded in `Q_t = ⟨q_t, HEAD_t, tape_t⟩`'s `tape_t` itself.
- **Then**: keep `LATEST_MARKOV_CAPSULE.txt` only as a CONVENIENCE pointer, with explicit README disclaimer "non-canonical, fresh chains MUST ignore", and mark `audit_tape --markov-pointer` as expecting an in-tape cid (audit_tape verifies the cid is in the supplied `--cas-dir`).
- **Cost**: 1-2 days additional sub-atom 2.4 / 2.6 work (if not already in scope) + audit_tape patch (~half day).
- **Trade-off**: matches Art. 0.4 path B "Phase E gate" architecture; constitutional fidelity highest; defers the simple "delete the pointer" fix until chain continuation lands.

### Option γ — Status quo + 守恒 test (least intrusive)

- Keep `LATEST_MARKOV_CAPSULE.txt` as-is, but add a 守恒 test: at every `generate_markov_capsule` invocation, write a SECOND file `handover/markov_capsules/LATEST_MARKOV_PROVENANCE.json` recording `{cid, source_chain_runtime_repo, source_chain_head_commit, written_at, written_by_run_id}`. `audit_tape` then reads this and verifies the source chain is a legitimate predecessor of the chain being audited.
- Adds the 守恒 chain Art. 0.2 demands without architectural rework.
- **Cost**: ~half day.
- **Trade-off**: adds a SECOND parallel-ledger to police the FIRST one. Constitutionally weaker than α / β. Recommended only if chain continuation (β) is multi-month away.

## §6 Claude's recommendation

**Option α** (delete + de-canonicalize) for **immediate**, with **Option β** (Art. 0.4 path B chain continuation) as the **proper long-term home** when sub-atoms 2.4 / 2.6 land.

Rationale:
- α removes the violation cleanly; the convenience-pointer use case is rare enough that explicit `--prior-chain-runtime-repo` is fine.
- β is the constitutional ideal but requires sub-atom 2.4 / 2.6 to mature (multi-task chain continuation).
- γ adds parallel state to police parallel state; a code smell on Art. 0.2 grounds.

## §7 Files / surfaces affected

| Path | Role | Touch under α | Touch under β |
|---|---|---|---|
| `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt` | global pointer | DELETE | retain as non-canonical |
| `handover/markov_capsules/MARKOV_TB-*.json` | per-TB capsule json | review label | retain |
| `src/bin/audit_tape.rs` | `--markov-pointer` required | make optional | accept `--prior-chain-runtime-repo` |
| `src/bin/audit_tape_tamper.rs` | mirrors `audit_tape` | mirror | mirror |
| `src/bin/generate_markov_capsule.rs` | writes pointer + capsule | per-run only | per-run only |
| `src/runtime/audit_assertions.rs:421` | `if pointer.exists()` branch | accept Option<Path> | retain semantic, change resolver |
| `src/runtime/markov_capsule.rs` | capsule schema | none | none |
| `tests/` | 守恒 test | NEW: derive_pointer_from_tape | NEW: chain-continuation Markov binding |
| `genesis_payload.toml` | trust root | rehash audit_tape* | rehash audit_tape* |
| `handover/tests/scripts/run_post_r3_round2.sh` | uses `--markov-pointer` | change call | change call |
| `handover/tests/scripts/run_real_llm_arena.sh` | uses `--markov-pointer` | change call | change call |
| `handover/tests/scripts/run_tb_16_x_2_1_smoke_2026-05-04.sh` | uses absent-pointer (correct) | retain (no-op) | retain (no-op) |

## §8 Cross-references

- Art. 0.2 (Tape Canonical 公理) — `constitution.md` lines 52-95
- Art. 0.4 (Q_t version-controlled) — `constitution.md` lines 114-152
- FC1 (Anti-oreo runtime loop) — `constitution.md` lines 455-509
- FC2 (Boot/Init) — `constitution.md` lines 571-660
- FC3 (Meta-architecture) — `constitution.md` lines 826-870
- TB-16.x.1 (audit fail-closed fix) — commit `3735484`; `handover/tracer_bullets/TB-16.x.1_charter_2026-05-04.md`
- TB-16.x.2 umbrella charter — `handover/tracer_bullets/TB-16.x.2_charter_2026-05-04.md` (sub-atoms 2.4 / 2.6 are the natural Option β home)
- Markov chain genesis semantic — `src/runtime/markov_capsule.rs:60+111`
- `audit_tape` pointer branch — `src/runtime/audit_assertions.rs:421-425` + `2002-2030`
- TB-16.x.2.1 smoke evidence (correctly genesis-chain) — `handover/evidence/tb_16_x_2_1_smoke_2026-05-04/P9_force_expire/`
- TB-16 R3 Round 2 P8_completeset_b (where the global pointer's bytes physically live) — `handover/evidence/tb_16_real_llm_arena_2026-05-04/post_r3_round2/P8_completeset_b/cas/`

## §9 Status

`PENDING_ARCHITECT_RATIFICATION` — no source code edits or pointer-file deletions until architect rules (α / β / γ / other). Claude continues sub-atom 2.x work using the absent-pointer pattern (constitutionally correct genesis-chain semantic) under the existing TB-16.x.1 audit hardening.
