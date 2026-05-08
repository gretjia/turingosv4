# CODEX Stage A3 HEAD_t C2 Charter Ratification Audit

## 1. Header

- auditor: Codex
- date: 2026-05-07
- target: `handover/tracer_bullets/STAGE_A3_HEAD_T_C2_charter_2026-05-07.md`
- gate: Gate 1 pre-R1 strict charter ratification for Stage A3 HEAD_t C2
- HEAD: `1e0c97c61d18dfffe573d9c381b432afc1711a3f`
- scope: charter consistency only; no implementation verification beyond cited C1/source-state cross-checks
- authority: `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md` §3.1 authorizes Stage A3 charter drafting and says STEP_B execution requires per-atom architect sign-off going forward (lines 115-122)

## 2. Inputs Reviewed

- `handover/tracer_bullets/STAGE_A3_HEAD_T_C2_charter_2026-05-07.md` - 163 lines, read in full.
- `handover/audits/CODEX_TB_18R_CHARTER_RATIFICATION_2026-05-06.md` - structural precedent, read in full.
- `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md` - targeted §3.1 authority cross-check.
- `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_zh.md` - targeted §3 Stage A3 cross-check.
- `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md` - targeted §4 / Stage A3 equivalent cross-checks.
- `CLAUDE.md` - targeted §4.1 HEAD_t and §12 STEP_B/Class-4 cross-checks.
- `constitution.md` - targeted Art. 0.2 / Art. 0.4 cross-checks.
- `src/state/head_t_witness.rs` and `src/bottom_white/ledger/transition_ledger.rs` - current C1 state cross-check only.

## 3. Per-Question Verdicts

### Q1 - Class-4 STEP_B Classification Correctness

Verdict: CHALLENGE

Reasoning: R1 is correctly treated as Class 4 in substance because the parent authorization classifies "A3 / HEAD_t C2 multi-ref ChainTape" as "Class 4 STEP_B (ledger surface)" and limits it to charter draft authorization pending per-atom sign-off (`handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md:115-122`). The charter repeats that classification for "ref naming + Git2-backed multi-ref ledger writer" (charter lines 20-23) and binds R1 to `src/bottom_white/ledger/transition_ledger.rs` as "4 STEP_B" (charter lines 103-107). Existing source confirms the current production writer is a single-ref writer on `refs/transitions/main`: its section title names that ref, `TRANSITIONS_REF` is `refs/transitions/main`, and commits update `Some(TRANSITIONS_REF)` (`src/bottom_white/ledger/transition_ledger.rs:643-677`, `:838-848`).

Challenge: the charter overstates the specific CLAUDE.md §12 basis. CLAUDE.md's STEP_B list names `src/kernel.rs`, `src/bus.rs`, `src/sdk/tools/wallet.rs`, `src/state/sequencer.rs`, `src/state/typed_tx.rs`, `src/bottom_white/cas/schema.rs`, and "canonical signing payload surfaces" (`CLAUDE.md:604-614`); it does not explicitly list `src/bottom_white/ledger/transition_ledger.rs`. The charter quote "Schema (`refs/chaintape/{l4,l4e,cas}` ref naming + Git2-backed multi-ref ledger writer) = **Class 4 STEP_B** on `src/bottom_white/ledger/transition_ledger.rs` + adjacent" (charter lines 20-22) is defensible via parent authority, but not "per CLAUDE.md §12 STEP_B list" alone. R2 is cleanly Class 3 if it remains an additive constructor, as the charter says "existing one preserved" (charter line 107) and FR preserves the existing `&QState` constructor (charter lines 64-65). R3 is not clean: the prompt describes R3 as Class 3, and the charter labels "CAS root ref advance hook on CAS write batch" Class 3 (charter line 108), but FR-A3-HEAD-T-C2.2 requires every CAS object write to update `refs/chaintape/cas` (charter line 63), which is part of the same three-ref canonical pointer surface declared Class 4 at charter lines 20-23.

Remediation: amend the charter to say R1 STEP_B authority derives from parent authorization §3.1 plus any adjacent CLAUDE.md §12 surfaces, not from the literal §12 file list alone. Move the CAS ref update authority from R3 into R1, or split R3 into "Class 4 ref-advance hook spec owned by R1" plus "Class 3 adapter plumbing that only calls the ratified R1 API."

### Q2 - Multi-Ref Schema Completeness

Verdict: CHALLENGE

Reasoning: The charter fully names the three target refs in §1 and FR-A3-HEAD-T-C2.1: `refs/chaintape/l4`, `refs/chaintape/l4e`, and `refs/chaintape/cas` (charter lines 36-38, 62). This matches CLAUDE.md's C2 production path exactly (`CLAUDE.md:222-229`) and the alignment docs' Stage A3 refs (`handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_zh.md:183-207`; `..._en.md:147-158`). The charter also specifies what advances each ref: accepted L4, rejected L4.E, and CAS object writes (charter lines 47-52, 63).

Challenge: the migration and CAS-root commit format are too loose for a charter intended to drive deterministic replay. The challenged migration quote is: "`refs/transitions/main` is migrated to `refs/chaintape/l4` (dual-write during migration window allowed; hard cutover on Stage A3 HEAD_t C2 ship)" (charter line 62). It does not define when dual-write begins, whether both refs must point to byte-identical commits, what happens on divergence, or which ref is authoritative during the window. The challenged CAS quote is: "CAS root commit per write batch; commit message references CAS object CIDs" (charter line 63). Existing C1 intentionally keeps the canonical record in tree blobs because "git normalizes message bytes in ways that break round-trip" (`src/bottom_white/ledger/transition_ledger.rs:647-659`). A CAS root whose determinism depends on commit-message formatting rather than canonical tree content is therefore underspecified.

Remediation: add a migration sub-requirement: during dual-write, `refs/transitions/main` and `refs/chaintape/l4` MUST advance atomically to equivalent canonical entries, divergence is a hard gate failure, and `refs/chaintape/l4` becomes authoritative at cutover. Add a CAS commit schema: canonical tree blobs, sorted CID manifest, batch sequence/logical timestamp source, deterministic author/committer, and a human-only commit message not used for replay.

### Q3 - HEAD_t Reconstruction From Refs Alone

Verdict: CHALLENGE

Reasoning: FR-A3-HEAD-T-C2.4 is falsifiable at a high level: "Replay MUST produce a HEAD_t identical (six-field byte equality) to the original run's HEAD_t" and a fresh replay "MUST end at the same OIDs on all three refs" (charter line 65). SG-A3-HEAD-T-C2.4 names a concrete new test for byte equality (charter line 93). Existing C1 `HeadTWitness` has a deterministic six-field canonical hash over fixed field order (source lines 48-63, 94-123), and existing `Git2LedgerWriter` already avoids wall clock by using logical time and fixed author identity (`src/bottom_white/ledger/transition_ledger.rs:825-829`).

Challenge: the charter does not explicitly carry the current writer's determinism rules into C2 for the two new refs. It also mixes "six-field byte equality" with "same OIDs on all three refs" (charter line 65), but the six-field witness has `l4_head`, `l4e_head`, and `cas_root` fields, not a third Git OID field named `cas_head` (`src/state/head_t_witness.rs:48-63`). Non-determinism risks are addressable, but not charter-pinned: CAS batch ordering, commit author/committer source, logical timestamp source, and whether packfile ordering is irrelevant because only commit/tree/blob OIDs are compared. The current C1 writer pins identity/time and canonical tree content (`transition_ledger.rs:647-663`, `:825-829`); C2 needs equivalent text.

Remediation: add deterministic replay clauses for all three refs: sorted batch manifests, fixed author/committer identity, logical timestamps only, canonical tree blobs as replay input, packfile order explicitly out of scope, and a precise equality definition mapping `l4_head` to `refs/chaintape/l4`, `l4e_head` to `refs/chaintape/l4e`, and `cas_root` either to a content root hash or to the `refs/chaintape/cas` commit OID.

### Q4 - No Global Filesystem Pointer

Verdict: PASS

Reasoning: The rule is present in both functional and constitutional form. FR-A3-HEAD-T-C2.5 says: "NO filesystem-side global pointer (e.g., `LATEST_HEAD_T.txt` / `CURRENT_RUN.json` / similar). The three Git refs ARE the canonical pointer" (charter line 66). CR-A3-HEAD-T-C2.5 repeats: "NO new global filesystem pointer. The three named Git refs are the pointer" (charter line 79). SG-A3-HEAD-T-C2.5 names a binary check: "No hidden filesystem pointer (grep + replay-without-fs-state)" with a concrete new test file (charter line 94). This aligns with CLAUDE.md's "global latest pointer" exclusion from valid evidence (`CLAUDE.md:85-108`) and FC2/FC3 prohibitions (`CLAUDE.md:162-188`), and with the English alignment halt condition for global pointer source-of-truth (`..._en.md:265-280`).

Remediation: none required for G1. Optional hardening: list the grep deny-patterns and allowed false positives in the charter so SG-A3-HEAD-T-C2.5 is mechanically reproducible.

### Q5 - HEAD_t Schema Preservation

Verdict: PASS

Reasoning: The charter's no-schema-change claim is consistent with C1 source. §1 says Stage A3 "does NOT alter HeadTWitness public API" and is a storage-form refactor (charter lines 54-56). CR-A3-HEAD-T-C2.6 says "NO change to HEAD_t six-field schema. C2 changes how the values are PERSISTED, not what they ARE" (charter line 80). Existing source defines exactly six fields: `state_root`, `l4_head`, `l4e_head`, `cas_root`, `economic_state_root`, `run_id` (`src/state/head_t_witness.rs:48-63`), and has a field-count test that requires exactly those six names (`src/state/head_t_witness.rs:152-177`). CLAUDE.md §4.1 pins the same six-field schema (`CLAUDE.md:198-229`).

Concern but not challenge: FR-A3-HEAD-T-C2.3 adds a public `HeadTWitness::reconstruct_from_repo(repo: &git2::Repository)` constructor (charter line 64), so the statement "does NOT alter HeadTWitness public API" (charter line 55) is imprecise if "alter" includes additive public API. The atom table handles this better by calling R2 an "additive constructor; existing one preserved" (charter line 107).

Remediation: replace "does NOT alter HeadTWitness public API" with "does not remove, rename, or change existing HeadTWitness fields or constructors; adds only `reconstruct_from_repo` for replay."

### Q6 - Backward Compatibility With Existing C1 Evidence

Verdict: CHALLENGE

Reasoning: The no-retroactive-rewrite rule is explicit. FR-A3-HEAD-T-C2.6 says existing C1 evidence directories remain replayable via a documented migration path and old runs are not rewritten (charter line 67). CR-A3-HEAD-T-C2.3 repeats that pre-Stage A3 runs replay via adapter, not evidence editing (charter line 77). This aligns with constitution/CLAUDE prohibitions on retroactive reconstruction/rewrite (`constitution.md:60-65`; `CLAUDE.md:152-168`) and with the parent alignment's C1-to-C2 framing (`..._zh.md:83-86`; `..._en.md:109-118`).

Challenge: the charter does not identify which existing C1 evidence families are covered or where the documented adapter lives. The prompt specifically asks what happens to `handover/evidence/constitution_landing_phase3_*`; the charter only says "existing C1 evidence directories" generically (charter line 67) and never names `constitution_landing_phase3_*`. SG-A3-HEAD-T-C2.9 says "OBS forward-binding for any C1 -> C2 migration edge case captured" (charter line 98), but that is not a migration-path specification and cannot tell an auditor whether a Phase 3 evidence directory is replayed through legacy `refs/transitions/main`, synthesized C2 refs, or an adapter view.

Remediation: add a migration-path subsection naming the C1 evidence glob(s), including `handover/evidence/constitution_landing_phase3_*`, and define expected behavior: no evidence edits, legacy `refs/transitions/main` read as C1 L4 source, absent L4.E/CAS refs derived only through documented adapter rules, and adapter evidence emitted as a new report rather than modifying old evidence.

### Q7 - SG-A3-HEAD-T-C2.1..10 Falsifiability

Verdict: CHALLENGE

Reasoning: SG-A3-HEAD-T-C2.1 through .5 are binary and name concrete tests (charter lines 90-94). SG-A3-HEAD-T-C2.6 and .7 name concrete commands and pass-count baselines (charter lines 95-96), consistent with charter preconditions that record 1181 workspace tests and 97 constitution gates (charter lines 137-143). SG-A3-HEAD-T-C2.10 correctly places dual audit after substrate evidence, matching the user's G1-before / G2-after instruction and the charter's §8 final ship sequence (charter lines 99, 145-153).

Challenge: SG-8 and SG-9 are not binary enough. SG-8 says: "One real-LLM smoke run (>=1 problem) on Stage A3 HEAD_t C2 substrate produces a 50/50-style invariant report under refs storage" (charter line 97). It does not define pass/fail fields, minimum required invariant rows, acceptable problem source, or whether a single problem must include accepted L4, rejected L4.E, and CAS advances. SG-9 says: "OBS forward-binding for any C1 -> C2 migration edge case captured" (charter line 98). "Any" is not mechanically enumerable and "captured" does not define required OBS contents or ownership. The charter also declares "Each gate is binary pass/fail" (charter line 86), so these two gate texts conflict with the charter's own gate standard.

Remediation: rewrite SG-8 to require a named report schema with fields for the three ref heads, HEAD_t byte-equality result, FC1 invariant result, and run artifact path. Rewrite SG-9 as a checklist: migration edge inventory file exists, each edge has `edge_id`, `legacy input`, `adapter behavior`, `test/evidence`, `owner`, and `status in {closed, deferred-with-authority}`.

### Q8 - Class-4 Hide-In-Class-3

Verdict: CHALLENGE

Reasoning: The charter includes a strong forbidden list against hidden Class-4 expansion: no typed-tx schema bump, no canonical signing payload change, no HEAD_t schema change, no sequencer public-API change beyond the ratified ledger writer adapter, and no agent-submittable system tx introduction (charter lines 124-132). It also states Stage A3 does not change typed_tx schema, sequencer admission semantics, or canonical signing payload (charter lines 54-56). This is consistent with CLAUDE.md's Class-4 and STEP_B rules for sequencer admission, typed tx schema, and canonical signing payload (`CLAUDE.md:508-515`, `:604-614`).

Challenge: R3 is the likely Class-4 hiding point. The charter classifies "CAS root ref advance hook on CAS write batch" as Class 3 (charter line 108), but the same charter classifies the three-ref schema and Git2-backed multi-ref writer as Class 4 STEP_B (charter lines 20-23). Since `refs/chaintape/cas` is one of the three canonical HEAD_t C2 refs (charter lines 47-52, 62-63), the authority to advance it should not live in an unratified Class-3 atom. R2 is cleaner because it is additive and schema-preserving (charter lines 64, 107), and R4 is clean Class 1 tests (charter lines 109, 90-94). No evidence in the charter indicates R2/R3/R4 change typed_tx schema, sequencer admission, canonical signing payload, or RootBox; the issue is ownership of the CAS ref writer surface.

Remediation: make the CAS ref writer/update API part of R1's Class-4 STEP_B surface. Leave R3 as a Class-3 caller/adapter only if it cannot define ref names, commit schema, canonical root semantics, or write ordering.

### Sanity Check - §1 Scope, §5 Atom Sequence, §6 Forbidden List vs §4 Ship Gates

Verdict: CHALLENGE

Reasoning: §1 scope is directionally consistent with §4 gates: C2 target requires L4, L4.E, and CAS refs plus replay from refs alone and no filesystem pointer (charter lines 34-56), and SG-A3-HEAD-T-C2.1..5 test those same properties (charter lines 84-99). §5 sequence is mostly consistent: R1 establishes the multi-ref writer before R2 replay constructor, R4 test gates, R5 smoke, R6 OBS, and R7 G2 dual audit (charter lines 101-112). §6 forbidden list is also aligned with §4: no dashboard source-of-truth supports replay/audit gates, no public chain matches local refs storage, and no typed-tx/signing/HEAD_t schema/invariant/evidence rewrite changes preserve the charter's "storage form refactor" scope (charter lines 114-135).

Challenge: three internal inconsistencies remain. First, §5 puts the CAS root ref advance hook in R3 Class 3 (charter line 108), but §1/§2 make `refs/chaintape/cas` part of the canonical C2 pointer and multi-ref schema (charter lines 47-52, 62-63), while the header classifies that schema as Class 4 STEP_B (charter lines 20-23). Second, §4 SG-8/SG-9 are not binary enough despite §4's statement that each gate is binary pass/fail (charter lines 86-99). Third, §6 forbids "no FC1 hard invariant change" and CR-A3-HEAD-T-C2.7 repeats the existing FC1 equation (charter lines 81, 129), but the charter does not map SG-8's "50/50-style invariant report" to the exact CLAUDE.md FC1 LHS definition (`CLAUDE.md:362-380`).

Remediation: move CAS ref semantics into R1, make SG-8/SG-9 mechanically binary, and bind SG-8's invariant report to the exact `evaluator_reported_completed_llm_calls = l4_work_attempt_count + l4e_work_attempt_count + capsule_anchored_attempt_count` equation.

## 4. Aggregate Verdict

Aggregate verdict: CHALLENGE-but-ship-clean

No VETO finding was found. The charter is directionally ratifiable and matches the architect's C2 target, but R1 should not execute until the Class-4 ownership boundary around the CAS ref and the deterministic migration/replay text are tightened. The aggregate reconciles the per-question results as: PASS on Q4/Q5, CHALLENGE on Q1/Q2/Q3/Q6/Q7/Q8, and no VETO.

VETO blocker quotes: none; no VETO verdict issued.

## 5. Top-3 Remediations

1. Move all `refs/chaintape/cas` ref naming, commit schema, root semantics, and write-order rules into R1's Class-4 STEP_B surface; leave R3 only as a Class-3 adapter that invokes ratified R1 behavior.

2. Add deterministic C2 replay/migration text: dual-write authority and divergence handling, canonical tree/blob CAS manifest format, sorted CIDs, fixed git signature/time rules, and precise mapping from three refs to the six HEAD_t fields.

3. Rewrite SG-8 and SG-9 as binary gates with concrete artifact schemas: SG-8 must report exact FC1 invariant fields plus three ref heads and byte equality; SG-9 must enumerate migration edges with owner, adapter behavior, evidence, and closed/deferred status.

## 6. Open Questions / Hypotheses

- STEP_B authority hypothesis: `src/bottom_white/ledger/transition_ledger.rs` is not literally in CLAUDE.md §12's STEP_B file list (`CLAUDE.md:604-614`), but the parent authorization explicitly classifies A3 as Class 4 STEP_B ledger surface (`handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md:115-122`). This audit treats the parent authorization as sufficient authority if the charter text is clarified.
- Evidence-directory hypothesis: the prompt names `handover/evidence/constitution_landing_phase3_*`, but this G1 audit did not inspect evidence directories because action safety forbids touching evidence and the task is charter-text-only. The charter should name the glob if it is in migration scope.
- CAS implementation hypothesis: the audit did not inspect `src/bottom_white/cas/store.rs` because the task limited source cross-checks to C1 state and forbade implementation verification. The R3 Class-4 concern is based on charter ownership boundaries, not an implementation finding.
