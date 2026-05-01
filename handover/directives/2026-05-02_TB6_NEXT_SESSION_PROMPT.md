# TB-6 Next Session Prompt — Finish Atoms 4-7

**For paste at the start of the next session.** Self-contained — covers context, state, directive, discipline, stop conditions.

---

## Context

- Branch: `main` @ `b0a6039` (TB-6 Atom 3 SHIPPED — first chain-backed smoke evidence in TuringOS v4 history).
- Architect ruling: `handover/directives/2026-05-01_TB6_ARCHITECT_RULING.md` (Path A; 7 binding decisions D1-D7 ALL satisfied for Atoms 0-3).
- Charter: `handover/tracer_bullets/TB-6_charter_2026-05-01.md` (8 atoms; Atoms 0-3 shipped).
- Preflight v2.1: `handover/ai-direct/TB-6_PRODUCTION_CHAINTAPE_BOOTSTRAP_2026-05-01.md` (Codex 2 audit rounds applied).
- LATEST.md top entry covers the previous session — **read first**.
- Memory rules added this session: `feedback_chaintape_wire_up_priority` + `feedback_smoke_evidence_naming` + `feedback_workspace_test_canonical` (already committed at Atom 0).

## State at session start

- `cargo test --workspace = 639 / 0 failed / 150 ignored / 48 suites` (HEAD `b0a6039`).
- Atoms 0,1,2,3 SHIPPED on `main`; Atoms 4-7 remaining.
- Chain-backed smoke evidence dir: `handover/evidence/tb_6_chaintape_smoke_2026-05-01/`.
- Disk: check at start (`df -h /home/zephryj | tail -1`); `cargo clean` if free space < 3G (~2.6G was free at end of last session).
- LLM proxy: `localhost:8080` (verify with `curl -s -m 3 http://localhost:8080/health`).

## Directive

Continue executing TB-6 to ship — Atoms 4 → 5 → 6 → 7 in sequence. Each atom one or more commits to `main`; STEP_B parallel branch only if a restricted file (`kernel.rs`/`bus.rs`/`wallet.rs`/`sequencer.rs`) is touched (none expected).

### Atom 4 — `verify_chaintape` CLI / replay verifier

- New module `src/runtime/verify.rs` (preferred) OR `src/bin/verify_chaintape.rs` (CLI binary).
- Re-opens `runtime_repo` + `cas` + `pinned_pubkeys.json` from a TB-6 evidence dir.
- Replays L4 chain entry-by-entry through `apply_one`-equivalent logic.
- Reconstructs `QState` + `EconomicState` from L4 alone (explicitly excludes L4.E per Inv 7).
- Verifies every entry's `system_signature` against `pinned_pubkeys.json`.
- Emits `replay_report.json`:
  ```json
  {
    "l4_entries": <n>,
    "l4e_entries": <n>,
    "ledger_root_verified": <bool>,
    "system_signatures_verified": <bool>,
    "state_reconstructed": <bool>,
    "economic_state_reconstructed": <bool>,
    "cas_payloads_retrievable": <bool>
  }
  ```
- Tests: I90+ in `tests/tb_6_runtime_chaintape_bootstrap.rs` OR new file `tests/tb_6_verify_chaintape.rs`.
  - Happy path: replay tb_6_chaintape_smoke_2026-05-01 → all booleans true.
  - Tampering path: mutate one byte of `entry_canonical` → `ledger_root_verified=false`.
  - Tampering path: mutate one line of `rejections.jsonl` → `verify_chain` Err.
- Update `handover/evidence/tb_6_chaintape_smoke_2026-05-01/` to include the produced `replay_report.json`.
- `genesis_payload.toml` rehash for any new files.

### Atom 5 — Agent audit trail

- Wire per-LLM-proposal `WorkTx` routing through `bus.submit_typed_tx` in `evaluator.rs`.
- Each proposal → `AgentProposalRecord` with the 9 fields:
  - `agent_id`, `prompt_context_hash`, `read_set`, `write_set`,
  - `proposal_cid`, `candidate_proof_cid`, `tx_id`,
  - `predicate_results`, `accepted_or_rejected`, `rejection_class`.
- `AgentProposalRecord` lives in CAS; `LedgerEntry.extensions` (existing field at `transition_ledger.rs:99-102`) carries the back-link CID. **No `LedgerEntry` schema change** per charter § 6 #10.
- **Forbidden**: chain-of-thought / private model deliberation / tool transcripts beyond CAS-stored artifacts. Architect ruling § 3.6 Atom 5 + WP § 12.4.
- Tests verify the 9 fields are populated + CAS retrievable.
- Genesis Trust Root rehash for evaluator.rs + any new files.

### Atom 6 — Branch / fork visibility summary

- `RunSummary` struct with `tx_count`, `failed_branch_count`, `rollback_count`, `accepted_tx_ids: Vec<TxId>`, `rejected_tx_ids: Vec<TxId>`, plus `proposal_count` from PputResult, `candidate_proposal_cids: Vec<Cid>`.
- Emitted at end of `run_swarm` alongside `PputResult`.
- Recorded in evidence dir as `run_summary.json`.
- Tests for struct shape + JSON round-trip + correct counts on a synthetic fixture.

### Atom 7 — Ship audit + TB-6 merge

- **Codex impl audit** on full TB-6 diff (Atoms 1-6 + smoke evidence). Spawn `codex:codex-rescue` agent with audit brief covering: Atom 1-3 already audited round-1+round-2; Atom 4 verifier completeness; Atom 5 audit-trail-no-CoT; Atom 6 fork-visibility shape; tb_6_chaintape_smoke evidence integrity; cargo test --workspace count + zero failures.
- **Gemini arch audit** at strategic tier (`gemini-2.5-pro`). If `MODEL_CAPACITY_EXHAUSTED`, fall back to `gemini-2.5-flash`. If still exhausted, document as `degraded` per TB-5 supplement precedent + `feedback_dual_audit`.
- **Conservative-merge** per `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS).
- **Round-cap=2** per `feedback_elon_mode_policy`.
- **Recursive self-audit** at `handover/audits/RECURSIVE_AUDIT_TB_6_2026-05-XX.md`. Mirror TB-5 shape:
  - Each charter § 4 decision block cross-referenced to src + tests.
  - Each charter § 6 forbidden line cross-referenced to absence in src.
  - Each charter § 8 ship proof cross-referenced to test green + smoke evidence.
- **TB_LOG.tsv** TB-6 row: flip `active → shipped`, fill `end_date` + `capability_metric` (final `cargo test --workspace` count + delta + smoke evidence path) + `ship_commits` (range from Atom 1 to Atom 7).
- **AUTO_RESEARCH_NOTEPAD.md** TB-6 SHIPPED log section.
- **Final cargo test --workspace** verification with reported count per D4.
- **Reset 24h iteration cap** for TB-7 (next sequencing per ROADMAP § 11.5: TB-7 = P2 Agent proposal/fork audit OR RSP-M0/M1 NodePosition derived index — selection depends on whether Atoms 5/6 covered the Agent audit trail comprehensively).
- **Merge atoms to main** via `git commit` on main (no separate experiment branch needed for Atoms 4-7 unless restricted file touched).

### Atom-cross discipline

- Every atom commit body MUST include:
  - `phase_id` line.
  - `roadmap_exit_criteria_addressed` line.
  - `kill_criteria_tested` line.
  - `FC-trace: FC3-N1` (or other relevant FC).
  - `cargo test --workspace` reporting block (`workspace_count`, `failed`, `ignored`, `suites`, delta).
  - Trust Root rehash documentation if `genesis_payload.toml` updated.
- TRACE_MATRIX FC3-N1 backlinks on every new `pub` symbol per R-022.
- STEP_B Phase-0 preflight ONLY if a CLAUDE.md restricted file is touched (not expected for Atoms 4-7).
- Codex audit is for Atom 7 ship gate only (Atoms 4-6 are extension-class; self-audit + targeted tests acceptable per Codex round-1 risk taxonomy).
- **Disk monitoring**: `cargo clean` if `df -h /home/zephryj | tail -1` shows < 3G free; **never touch `.lake/`** (Mathlib build cache; user rule).
- R-022 hook false-positive handling: `OBS_R022_*.md` + `[R-022-skip: ...; OBS_R022_*.md]` token if needed (precedent: `OBS_R022_TB-6_ATOM_1_2_TRACE_MATRIX_TEXT_EXTENSION_2026-05-01.md`).
- `GIT_COMMIT_MSG` env var override for worktree commits if `[R-022-skip:]` token isn't being read (hook reads stale `COMMIT_EDITMSG`; precedent at Atom 1.2).
- Per `feedback_no_fake_menus`: derive answers from architect ruling + charter; don't surface menus.
- Per `feedback_workspace_test_canonical`: bare `cargo test` is forbidden in ship reporting.

### Stop conditions

- Atom 7 ship verdict comes back **VETO** → escalate to user-architect for new ruling; do NOT auto-remediate VETO.
- Disk **ENOSPC** mid-build → `cargo clean` + retry once; if still failing, hand back to user with explicit disk-cleanup request (respecting `feedback_lake_packages_vendored` — never touch Mathlib `.lake`).
- LLM proxy down (Atom 5 needs real evaluator runs to populate proposal CIDs) → hand back.
- Codex agent infrastructure fails (Atom 7 audit) → fall back to self-audit + grep-based verification per TB-5 supplement; document `degraded` label.

### Begin

Begin with **Atom 4** unless user redirects. State the next concrete action in your first response (no menu).

## Acceptance for TB-6 ship (Atom 7 final gate)

TB-6 is provisionally green when:

1. ALL 8 atoms shipped on main.
2. `cargo test --workspace` ≥ TB-5 baseline (617) + new TB-6 tests (≥ 22 from Atoms 0-3 + Atoms 4-7 additions).
3. `handover/evidence/tb_6_chaintape_smoke_2026-05-01/` contains: `runtime_repo/.git/refs/transitions/main` (≥1 commit), `rejections.jsonl` (≥1 record), `pinned_pubkeys.json`, `synthetic_rejection_label.json`, `proof.lean`, `pput_result.jsonl`, `chaintape_report.md`, `replay_report.json` (Atom 4 output), `run_summary.json` (Atom 6 output), `README.md` answering the 8 mandatory questions.
4. Codex impl audit returns PASS or CHALLENGE-N applied (round-cap=2).
5. Gemini arch audit returns PASS or `degraded`-PASS labeled.
6. Recursive self-audit doc cross-references all charter § 4 decision blocks + § 6 forbidden lines + § 8 ship proofs to src + tests + smoke evidence.
7. TB_LOG.tsv TB-6 row flipped to `shipped`.

## After TB-6 ship

Per ROADMAP § 11.5 (`handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md`):
- TB-7 candidate: P2 Agent proposal/fork audit (if Atoms 5/6 didn't cover comprehensively) OR RSP-M0/M1 NodePosition derived index.
- TB-9 = RSP-3.2 Slash execution (deferred from TB-6 per architect Path A).
- TB-12 = RSP-M4 MarketOrder.

Architect re-engagement may be needed before TB-7 charter — surface to user.
