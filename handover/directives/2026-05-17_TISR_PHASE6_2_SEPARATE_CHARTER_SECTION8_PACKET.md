# TISR Phase 6.2 — Separate Charter Section 8 Packet

**Date drafted**: 2026-05-17 (post Phase 6.1 ship)
**Driver environment**: omega-vm (headless Linux)
**Predecessor**: Phase 6.1 §8 packet ratified 2026-05-17; Phase 6.1 shipped at commit `<TBD after R6 PROCEED>`
**Parallel sibling**: Phase 7 §8 packet (Mac Studio + Chrome) — independent, can ratify same day

---

## §1 Scope

This packet ratifies TISR Phase 6.2 on the omega-vm headless track. Phase 6.2
extends Phase 6.1's shipped CLI MVP with three additions, all backend-agnostic
and Cargo.toml-untouched:

1. **§6 Real-Witness completion.** Phase 6.1 §8 §6 specified an end-to-end
   ship-witness pipeline (`init → agent deploy → task open → audit dashboard →
   export evidence`); Phase 6.1 deferred the real-Lean-task evidence run.
   Phase 6.2 produces that witness at
   `handover/evidence/stage_phase6_2_<timestamp>/`, using a real (or compact
   simulated) Lean problem driven through `turingos task open` shell-out.

2. **UI IR spike expansion.** Phase 6.1's `experiments/tisr_ui_spike/` is a
   minimal Python renderer over 3 fixture JSON files. Phase 6.2 grows this
   into:
   - Additional fixtures: agent_role_view, batch_status_view, audit_summary_view,
     market_position_view (4 new, total 7)
   - JSON Schema validator binary (`experiments/tisr_ui_spike/validate.py`) that
     exit-codes 0 if a UI IR JSON satisfies `ui_ir_schema.json`, non-zero with
     diagnostic otherwise
   - Optional TUI rendering mode (`render.py --format tui`) using
     curses for terminal-only display (no HTML, no web)
   - `turingos render` subcommand: read a fixture via stdin or `--fixture`,
     emit the rendered text/json (still backend-agnostic; pure local)

3. **Quality-of-life backports** (small, no-§8-needed if already inside §4):
   - Tighten the `cli_wrapper_plumbing.rs` regression test (per R3 Codex
     recommendation: assert the exact `Run ... --workspace '<path>'` form
     in the deploy hint, not just substring containment)
   - Add `cli_batch_smoke.rs` whitespace-path test mirroring agent smoke
   - Optional: drop the `OBS_PHASE7_TASK_RUNNER_GENERALIZATION` and
     `R022_CLI_DISPATCH_OBS` historical docs from prominent positions once
     Phase 7 ships (they remain in directives/ as archive).

This packet does NOT ratify:
- Phase 7 Web MVP (separate sibling packet)
- production web serving, multimodal artifact storage
- CAS `ObjectType` additions
- new typed transaction variants
- new signature types
- sequencer admission changes
- any Trust Root rehash

## §2 FC Mapping

Touched nodes:
- FC1-N5 / FC1-N6: CLI read views (extending Phase 6.1's report / verify / audit wrappers)
- FC1-N10 / FC1-N13: write action — Phase 6.2's task open ship-witness exercises
  the lawful TaskOpen + EscrowLock path via existing TB-10 backend (no new
  admission rule)
- FC2-N16: same boot/genesis surface (`turingos init` etc.)
- FC2-N21: ship-witness evidence directory layout
- FC3-N31 / FC3-N39: UI IR fixtures + rendered views; materialized only, never authority

Invariants:
- No tape, no test: §6 witness run must produce real replayable evidence
- UI IR remains a view, not authority
- Price / reputation / UI confidence do not enter predicate truth

## §3 Risk Class

Default risk class: **Class 2** (production wire-up — happy path through
`turingos task open` shell-out to real lean_market backend).

Lower-risk subwork:
- Class 0: docs, charter, fixture JSON
- Class 1: additive Python tooling (validator, TUI mode), additional CLI
  subcommand wrappers, snapshot tests

Automatic escalation:
- Any edit to `src/state/typed_tx.rs`, `src/state/sequencer.rs`,
  `src/bottom_white/cas/schema.rs`, `src/kernel.rs`, `src/bus.rs`,
  `src/sdk/tools/wallet.rs`, canonical signing payloads, RootBox, or
  sequencer admission is **not authorized** here.
- Any required Trust Root pinned-file rehash is **not authorized** here.

## §4 Allowed Paths

Allowed implementation surfaces:

- `src/bin/turingos.rs`
- `src/bin/turingos/**`
- `tests/cli_*.rs`
- `experiments/tisr_ui_spike/**`
- `handover/directives/2026-05-17_TISR_PHASE6_*` (existing) and
  `handover/directives/2026-05-17_TISR_PHASE6_2_*` (new for this packet)
- `handover/reports/TISR_PHASE6_2_*`
- `handover/evidence/stage_phase6_2_*`
- `handover/audits/CODEX_TISR_PHASE6_2_*` (new audit record location;
  explicitly allowed here to avoid the R5 path-violation problem)
- `handover/alignment/OBS_R022_TISR_PHASE6_2_*` (new R-022 OBS location
  if any are needed; explicitly allowed)

This list is exhaustive. Disallowed paths trigger stop-and-ratify.

## §5 Exit Gates

All of the following must pass before §6 witness or ship:

- `git diff --check <Phase 6.1 ship commit>..HEAD` (trailing whitespace)
- `cargo check`
- `cargo build --bin turingos`
- `cargo fmt --all -- --check`
- `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo`
- 21+ Phase 6.1 CLI smoke binaries: all pass (no regression)
- New Phase 6.2 tests: all pass
- `bash experiments/tisr_ui_spike/test_render.sh` (and `validate.sh` if added)
- One clean-context Codex audit verdict = PROCEED

## §6 Real Witness Requirement

Phase 6.2 ship requires producing `handover/evidence/stage_phase6_2_<timestamp>/`
containing the end-to-end pipeline executed by an **autonomous verification
agent** (see §6a). The 8-step pipeline:

1. `turingos init --project <path>` — workspace created
2. `turingos agent deploy --id agent_001 --pubkey <64hex> --role Solver
   --workspace <path>` — agent registered
3. `turingos config set ...` — workspace config set
4. `turingos task open --problem <real Lean problem or compact synthetic>
   --bounty <micro> ...` — TaskOpen + EscrowLock posted; evaluator forked;
   Lean check runs to completion
5. `turingos audit dashboard --chaintape <path>` — audit dashboard regenerates
6. `turingos report wallet --chaintape <path>` — economic state replayed
7. `turingos export evidence --source <chaintape path> --out
   <handover/evidence/stage_phase6_2_<timestamp>/bundle>` — evidence bundle
   created
8. Replay verification: rerun the bundle via `turingos replay --chaintape ...`
   to confirm reproducibility

The witness may be partial or negative (Lean might fail to solve the problem;
that is acceptable). What is NOT acceptable: converting a failed witness into
a dashboard-only proof.

## §6a Autonomous Verification Agent

The §6 pipeline is executed **end-to-end by an independent verification
agent**, dispatched once by the orchestrator and running unsupervised until
final verdict. No human-in-the-loop checkpoints; no "please confirm" prompts;
no architect intervention until the agent posts its final report.

**Why autonomous**: Phase 6.1 user-side test (3.9/5) revealed that human
spot-checks are inconsistent — one user sees a bug another misses. Mechanical
PASS/FAIL criteria from a dedicated agent are reproducible, evidence-rich,
and don't burn architect time on routine verification.

### Agent specification

| Field | Value |
|---|---|
| Dispatch | `Agent` tool with `subagent_type: general-purpose`, `model: opus`, `isolation: "worktree"` |
| Tools allowed | `Bash` (run turingos / cargo; no network outside `handover/evidence/`), `Read`, `Write`, `Glob`, `Grep` |
| Tools forbidden | `Edit`, `mcp__*` (this is omega-vm; no MCP), `WebSearch`, `WebFetch` |
| Input | (a) commit SHA of Phase 6.2 ship candidate; (b) the 8-step pipeline from §6 verbatim; (c) expected exit codes + stdout-pattern per step; (d) evidence directory path |
| Output | `handover/evidence/stage_phase6_2_<timestamp>/agent_verdict.json` + structured step-by-step log |
| Timeout | 45 min wall clock (Lean check may take 5-15 min by itself) |
| Failure escalation | Only if (i) agent crashes mid-run or (ii) `agent_verdict.json` is unparseable. Otherwise its PASS/FAIL is authoritative for the witness layer (Codex audit still runs after this — agent's job is producing the witness, not gating ship). |

### Per-step decision criteria (mechanical, no human judgment)

| Step | PASS criteria | FAIL criteria |
|---|---|---|
| 1. init | exit 0; 4 scaffold files exist at expected paths | exit !=0 OR any scaffold file missing |
| 2. agent deploy | exit 0; agent_pubkeys.json contains the deployed entry by exact-string match | exit !=0 OR JSON malformed OR entry missing |
| 3. config set | exit 0; turingos.toml contains `<key> = "<value>"` exactly | exit !=0 OR roundtrip-get returns different value |
| 4. task open | exit 0 (Lean PASS) OR exit 1 (Lean FAIL but tape advanced); ChainTape contains TaskOpen + EscrowLock + (one of FinalizeReward / Bankruptcy / RunExhausted) | exit 2+ OR ChainTape missing the open/lock txs |
| 5. audit dashboard | exit 0; output file regenerated; contains task_id from step 4 | exit !=0 OR task_id absent in regenerated view |
| 6. report wallet | exit 0; output mentions agent_001 balance | exit !=0 OR agent_001 missing |
| 7. export evidence | exit 0; bundle contains chaintape/ + cas/ + at least one *.json file | exit !=0 OR bundle empty/malformed |
| 8. replay | exit 0; replay reports state_root match | exit !=0 OR state_root mismatch |

### Evidence schema (`agent_verdict.json`)

```json
{
  "agent_id": "verifier_phase6_2_<uuid>",
  "branch_head": "<commit-sha>",
  "started_at_unix": <epoch>,
  "completed_at_unix": <epoch>,
  "wall_clock_seconds": <int>,
  "overall_verdict": "PASS" | "FAIL" | "PARTIAL",
  "steps": [
    {
      "step": 1, "name": "init", "verdict": "PASS",
      "command": "./target/release/turingos init --project /tmp/...",
      "exit_code": 0, "stdout_excerpt": "...", "stderr_excerpt": "",
      "evidence_files": ["..."]
    },
    ...
  ],
  "lean_outcome": "SOLVED" | "FAILED" | "TIMEOUT" | "PARTIAL" | "N/A",
  "fail_reasons": []
}
```

### Anti-collusion safeguards

The verifier agent does NOT have edit access to `src/**`, so it cannot
modify the code it is verifying. Its only write surface is the evidence
directory. If its verdict is PASS but post-hoc Codex audit on the same
evidence directory finds the pipeline didn't actually run (e.g., empty
ChainTape, fabricated stdout), the verdict is invalid and Phase 6.2
re-enters atom-level fixing.

## §7 Constraints

Hard constraints (same as Phase 6.1, restated):
- Class 4 surfaces untouched
- Cargo.toml / Cargo.lock / src/lib.rs / src/main.rs / src/kernel.rs / src/bus.rs
  / src/boot.rs / genesis_payload.toml / rules/engine.py / .claude/hooks/judge.sh
  all immutable
- `pub(crate)` only; no new `pub` items; `/// TRACE_MATRIX FC2-N16:`
  doc-comments required
- No new external crate dependencies
- `rules/enforcement.log` net diff empty before commit

Soft constraints (recommended for hygiene):
- New `tests/cli_*.rs` files follow the std-only `Command::new` + tempfile
  pattern established in Phase 6.1; no `assert_cmd` / `predicates` /
  `test-binary` introduction
- TUI mode (if added) uses Python stdlib `curses`; no `ncurses` C
  dependency
- Snapshot tests use simple `assert_eq!` against expected strings;
  no `insta` introduction

## §8 Architect Sign-off

I, as the user/architect, hereby ratify TISR Phase 6.2 separate charter:
__authorize implementation on `codex/tisr-phase6-2-cli` branch on the
omega-vm; allowed paths per §4; exit gates per §5; real witness per §6;
Class 2 default risk class with automatic escalation rules per §3.__

This packet is independent of Phase 7 and may proceed in parallel.

Signed (verbatim): "________________________"

---

## Driver / orchestrator notes (not part of ratification)

- Atomic-execution model from Phase 6.1 carries forward (W0 → W1a/W1b/W1c
  parallel sub-waves → fan-in → integration → audit).
- 5-Sonnet-teammate-per-wave concurrency proved correct (research-validated;
  ~3-4 hour wall time per the Phase 6.1 measurement).
- Codex audit history: expect 1-2 rounds (R6 in Phase 6.1 was the 6th
  round; budget for similar polish cycle in Phase 6.2; the bulk of "subtle
  defects" is now caught by orchestrator's pre-Codex review).
- Pre-existing test failure `constitution_fc3_evidence_binding` blocks
  full `cargo test --workspace` but is out of scope; surface it as a
  Phase 6.2 KILL CRITERION if it BLOCKS the §6 witness pipeline.
