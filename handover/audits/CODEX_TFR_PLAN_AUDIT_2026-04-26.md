# CODEX_TFR_PLAN_AUDIT_2026-04-26

External audit of `handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md` at commit `3674d5f`.

## Verdict

**VETO.**

S1 must not start from this master plan as written. The plan is directionally aligned with Art. 0.4, and upstream `gix` does have a multi-parent commit API, but the current design contains a structural contradiction between:

- semantic citations as Git parents,
- `main` as the append-order branch,
- `time_arrow()` as `git log --reverse --first-parent main`, and
- Git's rule that the first parent must be the current ref target when updating that ref.

That contradiction is not a minor S0 spike detail. It determines the core storage model that S1.3-S1.5 would implement. If implemented literally, GitTape either cannot advance `main` for arbitrary non-HEAD citations, or it adds fake citation edges to every append, corrupting the tape DAG.

This VETO is also supported by three secondary blockers: C2 unfreezes before all tape-canonical side-state is migrated, the audit cadence is under-budgeted relative to the user mandate and A8 history, and Trust Root mutation is treated as safe even though `genesis_payload.toml` is the moving manifest and is not self-hashed.

## Sources Reviewed

- `handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md` lines 1-795.
- `constitution.md` Art. 0.1-0.4 lines 39-151 and Art. IV flowchart lines 587-660.
- `handover/architect-insights/TAPE_CANONICAL_AUDIT_2026-04-26_AUDITOR.md` lines 12-155.
- `handover/architect-insights/TAPE_CANONICAL_AUDIT_2026-04-26_CODEX.md` lines 1-41.
- `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` lines 11-379.
- `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` lines 132-156, 192-257, 406-489, 535-610, 688-697.
- Current code shape in `src/ledger.rs`, `src/bus.rs`, `src/kernel.rs`, `src/wal.rs`.
- Upstream docs fetched 2026-04-26:
  - `gix` docs.rs latest `0.82.0`: https://docs.rs/crate/gix/latest, lines 116-136.
  - `gix::Repository::commit/new_commit`: https://docs.rs/gix/latest/gix/struct.Repository.html, lines 1319-1343.
  - `gix` hash feature docs: https://docs.rs/gix/latest/gix/, lines 230-236.
  - `git init --object-format`: https://www.kernel.org/pub/software/scm/git/docs/git-init.html, lines 33-38.
  - `git2::Repository::commit`: https://docs.rs/git2/latest/git2/struct.Repository.html, lines 791-795.
  - `githooks`: https://www.kernel.org/pub/software/scm/git/docs/githooks.html, lines 45-58.

## Summary Table

| # | Challenge | Finding | Severity |
|---|---|---|---|
| 1 | gix vs git2-rs | No upstream blocker for multi-parent commits, but plan pins stale `gix` and assumes SHA-256 without specifying repo object format. | Medium |
| 2 | Node to Git mapping | VETO: semantic citations cannot simply be Git parents while `main` is also first-parent append order. | High |
| 3 | Tape trait API | Trait hides fallible Git operations and conflates NodeId, commit SHA, and HEAD. Missing causal/transaction semantics. | High |
| 4 | Sprint timeline | 7-10 weeks is optimistic-as-median when compared with A8's 12+ audit rounds and 14 substantive findings. | High |
| 5 | Atom dependencies | VETO: C2 unfreezes at S3 while S4/S5 still contain known tape-canonical violations. | High |
| 6 | Trust Root migration | Moving manifest integrity story is under-specified; dual audit per mutation is not enough. | High |
| 7 | Risk register | Several black swans are missing: parent-order conflict, SHA-256 format, hook bypass, archive evidence loss, audit overload. | High |
| 8 | PREREG amendment | Family-size claim is conditionally true, but S6.4 comparison and heldout operational sealing need stronger constraints. | Medium |
| 9 | Open questions | Missing user decisions on parent topology, SHA-256 vs SHA-1, unfreeze gate, audit cadence, Path D, and Trust Root authority. | High |
| 10 | TRACE_MATRIX discipline | Bidirectional trace at every atom is plausible only with generated tooling; current process history shows manual drift under pressure. | Medium |
| 11 | Path B assumption | Path D alternatives are not fairly evaluated; real Git is not the only way to realize version-controlled Q_t. | Medium |
| 12 | 50-atom overhead | Plan's audit math contradicts the stated atom-level dual-audit burden and is not structurally sustainable as written. | High |

## Expanded Findings

### 1. gix vs git2-rs decision

**Finding 1.1 - Medium - The upstream multi-parent API exists, but the plan's version and hash claims are stale or underspecified.**

Observed evidence:

- The plan recommends `gix` version `^0.66` and says needed features are in stable APIs (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:88`).
- Current docs.rs latest is `gix 0.82.0`, released 2026-04-24; the docs version list shows `0.82.0`, `0.81.0`, down to `0.66.0` (`docs.rs/crate/gix/latest:116-136`).
- `gix::Repository::commit` and `commit_as` accept `parents: impl IntoIterator<Item = impl Into<ObjectId>>`, so multi-parent commit creation is available (`docs.rs/gix/.../Repository.html:1319-1327`). `new_commit` also accepts an arbitrary parent iterator without updating refs (`docs.rs/gix/.../Repository.html:1340-1343`).
- `git2::Repository::commit` also accepts `parents: &[&Commit]`, and libgit2 requires the first parent to be the tip when updating an existing branch (`docs.rs/git2/.../Repository.html:791-795`).
- The plan claims Path B gives "free, sha256 commit hashes" (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:51`) and maps `node.hash` to "commit SHA itself" (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:141`).
- Upstream Git says `sha1` is the default object format and `sha256` requires `--object-format=sha256`; it also notes no interoperability between SHA-256 and SHA-1 repositories (`git-init.html:33-38`). `gix` docs likewise show `sha1` enabled by default and `sha256` as an explicit feature (`docs.rs/gix/latest/gix/:230-236`).

Reasoning:

The library choice is not blocked by lack of a multi-parent API. The concern is that S0.3/S0.4 should not test against a stale `^0.66` assumption or accidentally create SHA-1 repos while the plan's constitutional argument depends on SHA-256 wording. This is not fatal by itself, but it must be corrected before dependency landing.

Required fix:

- Pin the actual chosen `gix` version available at S0, not `^0.66` by inertia.
- Decide explicitly whether runtime repos are SHA-1 or SHA-256. If SHA-256, specify the exact `gix` init/config path or controlled CLI fallback for `--object-format=sha256`, plus the `gix` crate features required.
- Add S0.4 assertions that object IDs are the expected width and object format.

### 2. Node to Git mapping

**Finding 2.1 - High - VETO: the proposed "citations as Git parents" mapping is internally inconsistent with first-parent append order.**

Observed evidence:

- The plan maps `node.citations` to Git parents and calls this "the key constitutional win" (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:136,143`).
- The plan defines `time_arrow()` for GitTape as `git log --reverse --first-parent main` (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:162-164`).
- The fallback mode says all commits are on `refs/heads/main` with multi-parent merges representing the DAG (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:337`).
- S1.5 specifically implements multi-parent commit support for `citations.len() > 1 -> merge commit` (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:411`).
- `gix::Repository::commit` requires the first parent in `parents` to be the current target of the updated reference (`docs.rs/gix/.../Repository.html:1330-1334`). `git2` has the same first-parent/current-tip rule when updating HEAD (`docs.rs/git2/.../Repository.html:791-795`).
- Current `Tape::trace_ancestors` treats the first citation as the primary semantic parent, not "previous append in time" (`src/ledger.rs:111-126`).

Reasoning:

For a new node whose semantic citation is not the current `main` tip, Git cannot both:

1. update `main` with `Repository::commit("HEAD", parents=[semantic_parent, ...])`, and
2. keep `time_arrow()` as first-parent `main`.

If GitTape uses semantic citations as the parent list, the first parent will often not be current `HEAD`, so ref update fails or requires force-moving `main` and loses append-order reachability. If GitTape prepends current `HEAD` to satisfy Git, every node semantically cites the prior append whether or not the model did so, corrupting the citation DAG. If GitTape uses `new_commit` and manually updates refs, the same semantic problem remains unless a separate append-order edge is invented.

This is the central storage-model decision. It cannot be deferred to S1.5 after the trait and skeleton land.

Required fix:

- Separate chronological predecessor from semantic citation edges. One viable model: parent[0] is previous `HEAD` for append order, while semantic citations live in sidecar JSON/trailers and are indexed into derived refs or notes. Another viable model: branch from the cited parent and merge into `main`, but then parent ordering and merge commits must encode both "previous main" and "proposal branch" explicitly.
- Add a formal invariant: `first_parent(commit) == previous_HEAD` iff the commit advances the canonical append chain; semantic citations are separately reconstructed and tested.
- Update `time_arrow`, `get_chain`, `children`, S1.5, S3.5, and §2.10 before S1 starts.

**Finding 2.2 - High - The sidecar JSON file is not immutable under the current mapping.**

Observed evidence:

- The plan stores node payload and metadata in `.turingos/nodes/<id>.json` (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:105-107,135-140`).
- Duplicate protection only checks whether the file already exists in `HEAD` before commit (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:147`).
- GitTape append writes the sidecar file, stages it, runs the hook, and commits (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:235-240`).

Reasoning:

The plan does not state that each node commit must add exactly one new node file and must never modify or delete any existing `.turingos/nodes/*.json` file. Because `get(id)` in §2.4 reads "a node by ID" without specifying commit-local lookup, an implementation may read the file from `HEAD`; if a later commit edits an old node sidecar, current-HEAD lookup returns mutated history even though Git still has the older blob. Git can detect this, but only if the invariant is explicit and tested.

Required fix:

- Specify an append-only tree delta invariant: every node commit adds one new path and modifies/deletes zero existing node paths.
- Store the sidecar blob OID in the commit message/trailer or in an index so `get(id)` can resolve the blob from the introduction commit, not from mutable current `HEAD`.
- Add tests that try to edit/delete an existing node file and assert rejection before S1 exit.

### 3. Tape trait API

**Finding 3.1 - High - The trait erases fallibility and conflates IDs, handles, and HEAD.**

Observed evidence:

- `Tape::get` returns `Option<Node>`, while `time_arrow`, `get_chain`, and `children` return bare `Vec`s (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:159-180`).
- `Tape::head()` returns `Option<NodeId>`, but its GitTape comment says `git rev-parse HEAD` (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:172-175`).
- `NodeHandle` distinguishes `GitCommit { sha }` from `MemId { id }`, but `head()` does not return `NodeHandle` (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:189-196`).
- `QState` stores both `head_t` and `tape_t`, with `head_t` "equals tape.head()" but stored explicitly (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:274-279`).

Reasoning:

Git operations can fail because the repo is corrupt, the worktree is missing, a ref is malformed, an object cannot be parsed, or an ID cannot be resolved. Returning `Option` or `Vec` collapses "not found" into "storage failure" and makes fail-closed Art. I.1 discipline harder. Also, a Git HEAD SHA is not the same type as a semantic NodeId unless the plan defines a total bidirectional mapping. The current API invites silent mismatch between `QState.head_t` and `tape.head()`.

Required fix:

- Use `Result<Option<Node>, TapeError>` and `Result<Vec<NodeId>, TapeError>` for Git-backed reads.
- Distinguish `NodeId`, `CommitId`, and `Head` as separate types.
- Add an explicit `resolve_node_id_to_commit`, `resolve_commit_to_node_id`, and `snapshot_at(head)` API.
- Add a conformance invariant that `QState.head_t == tape.head()` after every `wtool`.

**Finding 3.2 - Medium - The trait is missing the operations needed to make all derived views tape-canonical.**

Observed evidence:

- Art. 0.2 requires cost, time, provenance, market price, wallet state, rejection feedback, search history, and Boltzmann routing to be reconstructible from tape (`constitution.md:60-65`).
- The proposed trait has no `iter_nodes`, `iter_from`, `events_between`, `diff`, `snapshot_at`, or `validate_integrity` operation (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:151-187`).

Reasoning:

S4/S5 derived-view rewrites need bounded iteration and replay primitives. Forcing every derived view to call `time_arrow()` and then `get()` N times is workable for MemTape but a poor contract for GitTape. It also hides which view is derived from which commit range.

Required fix:

- Add replay-oriented APIs or a `TapeCursor` abstraction before S2/S4 are designed.
- Keep high-level helpers like `children()` as derived convenience methods, not the only primitive.

### 4. Sprint structure timeline

**Finding 4.1 - High - The timeline uses A8 as evidence but does not budget A8-like fix cycles.**

Observed evidence:

- The plan estimates 48-68 days and calls 8 weeks "honest" median (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:358-369`).
- The same paragraph admits round-2 fix cycles are normal and A8 had 14 rounds (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:369`).
- A8 history shows 12 audited rounds all merged as VETO or CHALLENGE through R12 (`handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:363-376`) and a pending R13 row (`handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:377`).
- A8 cumulative text reports about $80 cost, 14 substantive findings across 12 audited rounds plus a false-closure finding (`handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:379`).

Reasoning:

TFR is more invasive than A8: it changes storage substrate, core APIs, WAL, bus, kernel, evaluator, Trust Root, and audit tooling. Treating 7-10 weeks as a median assumes most external audits close in one round. The actual A8 base rate says repeated CHALLENGE loops are normal even for governance/doc/code-quality fixes. The plan should model fix-cycle latency per sprint and per STEP_B atom, not just "16 wall-hours of audit-runtime."

Required fix:

- Re-estimate using a pessimistic median: each sprint has at least one fix-then-proceed loop; S3 has two.
- Add an explicit "stop/descale" decision if S0-S2 already consume more than a preset fraction of the 70-day cap.
- Treat 10 weeks as the optimistic cap only if the atom count is reduced.

### 5. Per-sprint atom enumeration

**Finding 5.1 - High - VETO: S3.9 unfreezes Phase C before the plan closes known tape-canonical violations.**

Observed evidence:

- The plan freezes C2 until S3 cargo-test green plus dual-audit PASS/PASS (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:66`).
- S3.9 deletes the freeze file and unfreezes C2 (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:463`).
- S4 is where MarketCreate, MarketResolve, Invest, direct evaluator investments, WalletTool, founder grants, and bounty market become tape-derived (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:478-484`).
- S5 is where mr tick, synthetic treatment, Boltzmann picks, search hits, Librarian state, Lean error strings, halt detail, and audit-guard provenance go on tape (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:494-501`).
- Art. 0.2 says all cost, time, provenance, market price, wallet state, rejection feedback, search history, Boltzmann routing, and mr tick must be derivable from tape (`constitution.md:60-65`).
- The Auditor audit identifies V-04/V-05/V-07/V-08/V-10/V-11/V-14/V-19/V-21/V-24 as missing tape-canonical state (`handover/architect-insights/TAPE_CANONICAL_AUDIT_2026-04-26_AUDITOR.md:17-24,33-55`).

Reasoning:

S3 makes GitTape production-capable, but the plan itself says the economy and side-state migrations happen later. If C2 restarts at S3, it runs while known behavior-shaping side-state is still outside tape. That is exactly the bug class TFR is supposed to eliminate. The constitution contains a narrower "Phase C C2 restart gating at Commit 1-4" note (`constitution.md:95`), but this plan's own Path B scope expanded the work and explicitly acknowledges S4/S5 violations. Under the user's "wrong refactor done well" warning, unfreezing early is not acceptable.

Required fix:

- Move C2 unfreeze to after S5.8, or explicitly produce a narrow proof that Phase C C2 does not exercise S4/S5 side-state and that all behavior-shaping signals for C2 are closed by S3.
- If early unfreeze remains desired, make it a user decision in §10 and label the constitutional residual risk.

**Finding 5.2 - Medium - Several atoms are 2-3 atoms in disguise.**

Observed evidence:

- S3.6 wires `QState` as primary bus state and routes `append_v2` through `wtool` in one atom (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:460`).
- S5.6 combines Lean error strings, Halt detail, and WAL deprecation (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:499`).
- S6.2 requires tarball, checksum, rerun, and identical SHA lifecycle test (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:510`).

Reasoning:

These are not single-risk units. S3.6 touches the constitutional state transition. S5.6 touches oracle output, halt semantics, and persistence deprecation. S6.2 depends on deterministic timestamps, object format, repo config, pack behavior, and archive ordering. Treating them as one atom each hides critical path and audit scope.

Required fix:

- Split S3.6 into `QState` ownership, `rtool`, `wtool`, and invariant tests.
- Split S5.6 into OracleVerdict payload, Halt node, and WAL write deprecation.
- Split S6.2 into deterministic repo config, archive format, and rerun equivalence.

### 6. Trust Root migration

**Finding 6.1 - High - The manifest is moving, but the plan does not define a manifest-lineage invariant.**

Observed evidence:

- The plan says Trust Root changes during TFR and is updated at every sprint exit, not mid-sprint (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:69`).
- §7.2 lists re-hashes and additions across S0-S6 (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:601-611`).
- §7.3 says protections are per-sprint dual audit of the manifest delta and boot-time `verify_trust_root` green (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:613-619`).
- Current `genesis_payload.toml` says it is conceptually frozen but not self-hashed because of the chicken-and-egg problem (`genesis_payload.toml:94-96`).
- PREREG §1.8 defines Trust Root files as only humans + Boot may write, with ArchitectAI write attempts aborting (`handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:132-154`).
- Constitution Art. V.1.2, however, gives ArchitectAI authority to update non-constitution Trust Root manifest entries after Veto-AI review (`constitution.md:736`).

Reasoning:

`verify_trust_root` can verify the manifest currently on disk, but a malicious or mistaken manifest mutation can remove or re-hash an entry and then pass verification. Dual audit helps, but the plan needs a machine-checkable lineage rule: every manifest mutation must be checked against the previous manifest and allowed deltas. The current plan does not define what removals are permitted, how `pput_accounting_0` is protected during TFR, or how the PREREG "only humans + Boot may write" language coexists with ArchitectAI's manifest mutation authority.

Required fix:

- Add a `trust_root_delta_conformance` test that reads previous and proposed manifest states and enforces allowed add/remove/rehash sets for the current sprint.
- Freeze `pput_accounting_0` separately from `[trust_root]` mutations.
- Make the authority model explicit: user sudo, ArchitectAI commit authority, and Veto/external audit roles for manifest changes.

### 7. Risk register

**Finding 7.1 - High - The risk register misses the most important failure modes.**

Observed evidence:

- The risk register has 14 risks (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:626-643`).
- R10 covers "gix does not support multi-parent commits" but treats fallback as single-parent with citations only in sidecar JSON (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:639`).
- R7 covers heldout leakage only through commit messages and repo content grep at S6.1 (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:636`).
- R8 calls 8-week overrun high probability but low to medium impact (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:637`).

Reasoning:

The missing risks are more serious than several listed risks:

- Semantic parent vs chronological first-parent conflict. This is not "gix lacks multi-parent"; it is "Git's first-parent semantics encode a different relation than Node citations."
- SHA-256 not actually enabled. The plan's constitutional hash argument may silently land SHA-1 repos.
- Hooks false assurance. Git hooks are invoked by `git commit` and can be bypassed by `--no-verify`; pre-merge hooks behave differently (`githooks.html:45-58`). The plan already uses in-process checks for library commits, so the on-disk hook must be framed as tamper detection only, not as proof that historical commits passed.
- Runtime repo evidence loss. §2.2 says `runtime_repo/` is not in TR and deletable post-archival (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:122`). If only a genesis SHA is retained and the archive is optional (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:117`), Phase E reproducibility can lose the actual tape.
- Audit overload. §5.4 budgets 32 external audit invocations (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:551`), but the user-level mandate says each atom needs parallel branch plus dual external audit.

Required fix:

- Add these risks with triggers and hard mitigations before S0 exit.
- Reclassify R8 impact to High: schedule overrun can void the PREREG amendment at 70 days (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:671`).

### 8. PREREG amendment Proposal A

**Finding 8.1 - Medium - The `N_max=34 unchanged` claim is conditionally true, but only if TFR evidence is not used inferentially.**

Observed evidence:

- Proposal A states TFR does not modify `N_max = 34` or `k_max = 10`; the substrate change is statistically transparent (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:661`).
- Current PREREG says the inferential family is `4 + 3*k`, with `k_max=10` and `N_max=34` (`handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:585-590`), and unused slots forfeit alpha (`handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:595-610`).
- S6.4 runs post-TFR C2 and compares against pre-TFR baseline, accepting a NEGATIVE finding if PPUT shifts (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:512`).

Reasoning:

If S6.4 is purely an engineering equivalence/sanity report, the family-size invariant holds. If the comparison is used to support or reject any research claim beyond "substrate migration changed/did not change behavior", then it becomes a new analysis surface outside the frozen family. The plan does not state which interpretation applies.

Required fix:

- State that S6.4 is non-inferential engineering validation unless a formal PREREG addendum adds tests and alpha handling.
- Define what "PPUT shifts" means operationally and what decision it triggers.

**Finding 8.2 - Medium - Heldout integrity under Git substrate is under-specified.**

Observed evidence:

- Proposal A keeps the heldout-54 list and SHA unchanged (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:663`).
- PREREG states heldout sealing is operational, not cryptographic, and that the seed/list are visible to anyone who can read repo files (`handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:192-197`).
- PREREG L1-L5 explicitly blacklist prompt context, tool-call args, hash functions, and MiniF2F/Test enumeration during Phase B-D (`handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md:198-257`).
- S6.1 says it verifies the new git substrate does not leak heldout IDs into prompt context, tool-call args, or commit messages (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:509,663`).

Reasoning:

Git introduces new readable surfaces: commit messages, refs, object contents, tree paths, `git log`, `git grep`, archive metadata, and possibly `.git/config`. The plan mentions commit messages but does not explicitly extend L1-L5 to runtime repo paths, object database reads, ref enumeration, or `git` commands in tool-call preflight.

Required fix:

- Extend heldout sealing tests to `runtime_repo/.git`, `git log`, `git grep`, `git cat-file`, `git show`, refs, packed objects, archives, and sidecar JSON.
- Make runtime repo read access unavailable to Phase B-D agents unless the access path is explicitly audited.

### 9. Open questions Q1-Q10

**Finding 9.1 - High - Several user-decision items are missing or incorrectly delegated to ArchitectAI defaults.**

Observed evidence:

- §10.2 lists Q1-Q10 user decisions (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:706-719`).
- §10.3 says implementation details inside §2.4-§2.10, per-atom file paths, conformance test locations, Cargo feature flag naming, and per-sprint TRACE wording do not need user decision (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:721-729`).

Reasoning:

At least six decisions are too architectural to hide under "implementation details":

- Parent topology: Does Git parent[0] mean previous `HEAD`, semantic citation, or both?
- Hash format: SHA-1 compatibility vs SHA-256 constitutional wording.
- C2 unfreeze point: S3 vs S5/S6.
- Runtime repo retention: optional archive vs mandatory immutable evidence.
- Audit cadence: 32 external cycles vs every atom.
- Path D alternatives: real Git vs explicit Merkle append log.

Q2 also says the spike can auto-switch to git2-rs if gix fails (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:711`). Given the FFI/build matrix and the fact that the mapping problem is library-independent, that should require user approval, not only ArchitectAI judgment.

Required fix:

- Add these questions to §10.2.
- Remove parent topology and C2 unfreeze from "ArchitectAI proceeds with defaults."

### 10. DO-178C TRACE_MATRIX_v3 conformance

**Finding 10.1 - Medium - The discipline is sound in goal but too manual for the expected pressure.**

Observed evidence:

- §6 requires every constitutional clause to map to code and every public symbol to backlink or be marked orphan (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:557-560`).
- §6.3 requires each sprint's last atom to update TRACE_MATRIX and the conformance test to stay green at every commit (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:577-583`).
- §6.4 adds a bidirectional test parsing TRACE_MATRIX, rules manifest, and source doc-comments (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:585-591`).
- Existing `CLAUDE.md` already requires public symbol backlinks and conformance tests (`CLAUDE.md:34-40`).
- A8 history shows repeated documentary drift: stale counts, stale runner headers, stale Trust Root counts, and false-closure claims across many rounds (`handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:349-357,361-379`).

Reasoning:

Manual trace updates at every atom are exactly the kind of discipline that decays under deadline pressure. A8 shows that even with a mature audit culture, adjacent documentation drift persisted for many cycles. The new bidirectional test is necessary, but the plan does not show the parser/schema that would make it robust.

Required fix:

- Make TRACE_MATRIX_v3 machine-readable first, prose second. Generate summary prose from a structured source or enforce strict table schema.
- Scope the every-atom requirement to changed public APIs, with a separate nightly/full check for all symbols.
- Add a test that fails if line-count/status claims in governance docs are stale relative to actual manifest/test output.

### 11. Path B assumption itself

**Finding 11.1 - Medium - The plan does not fairly evaluate a Path D that may satisfy the constitution with lower semantic risk.**

Observed evidence:

- Constitution Art. 0.4 says Q_t is "version control" with `HEAD_t` as path and `tape_t` as files (`constitution.md:114-123`) and lists Path A/B/C as decision options (`constitution.md:136-149`).
- The plan operationalizes only Path B and treats Path A/C as rejected (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:44-55`).
- The current bug is not just lack of Git, but in-memory `Vec<Node>`, missing HEAD_t, and non-reconstructible side-state (`constitution.md:124-134`; `handover/architect-insights/TAPE_CANONICAL_AUDIT_2026-04-26_AUDITOR.md:12-24`).

Reasoning:

There is a plausible Path D: an append-only event log or embedded immutable store with explicit `HEAD_t`, content-addressed node blobs, parent hashes, Merkle chain, and stock verifier/export tooling. Candidates include an append-only JSONL/CBOR log with explicit SHA-256 chain, SQLite with immutable content-addressed rows and signed checkpoints, or sled/rocksdb with hash-indexed blobs plus a Git export layer. These would avoid Git parent-order semantics while preserving the constitutional triple. They would lose native `git log` as primary audit UX, but they may better model arbitrary DAG citations.

This is not a recommendation to abandon Path B now. It is a challenge that the plan's "real Git or bust" argument has not addressed the core semantic mismatch discovered in Finding 2.1.

Required fix:

- Add a rejected-alternatives section comparing Path B to Path D on HEAD semantics, citations, replay, audit UX, performance, and implementation risk.
- If Path B remains chosen, explicitly explain why Git parent-order compromise is acceptable or how it is avoided.

### 12. 50-atom STEP_B count overhead

**Finding 12.1 - High - The plan's audit math conflicts with the stated atom-level burden and is not sustainable as written.**

Observed evidence:

- The plan says every sprint exit and every commit before merge gets a dual-audit packet (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:527-533,543-545`).
- It then narrows atom-level commits to internal auditor only, except STEP_B atoms and Trust Root commits (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:547-550`).
- It estimates about 25 STEP_B external audits plus 7 sprint exits, about 32 external audit invocations (`handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:551`).
- The user task states about 50 STEP_B atoms are planned and each requires parallel branch plus dual external audit.
- A8 shows 12 audited rounds through R12, with no merged PASS through those rounds and 14 substantive findings across 12 audited rounds plus a false-closure finding (`handover/audits/A8_AUDIT_HISTORY_2026-04-26.md:361-379`).

Reasoning:

If every atom really gets parallel branch plus dual external audit, the plan undercounts external audit cycles by roughly half. If only STEP_B atoms get external audit, the plan conflicts with the user mandate as stated for this review. Either way, the overhead is not structurally sustainable at 50 atoms unless atom count is compressed or audit gates are batched by coherent risk units.

Required fix:

- Resolve the policy contradiction explicitly before S0 exit.
- Reduce atom count by merging doc-only and TRACE-only commits where mechanically safe, while preserving code-risk boundaries.
- Add a "cadence budget" with maximum open audit rounds, maximum active branches, and a hard stop if fix-cycle latency exceeds plan assumptions.

## Minimum Conditions To Lift VETO

1. Rewrite §2.3/§2.4/§2.10/S1.5 to separate append-order parentage from semantic citations, with tests for non-HEAD citations.
2. Decide and test Git object format: SHA-1 or SHA-256. Remove "free sha256" wording unless SHA-256 init is mandatory and verified.
3. Move C2 unfreeze to after all behavior-shaping side-state used by C2 is tape-canonical, or produce a narrow proof that S4/S5 state is irrelevant to C2.
4. Add manifest-lineage conformance for Trust Root mutations and freeze `pput_accounting_0` separately.
5. Recompute timeline and audit budget using the actual user-mandated atom-level dual-audit policy and A8 fix-cycle history.
6. Add missing user decisions to §10.2, especially parent topology, hash format, unfreeze gate, Path D rejection, and audit cadence.

Until these are fixed, entering S1 would risk building the wrong substrate correctly.
