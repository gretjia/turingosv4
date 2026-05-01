# TB-5 Smoke-Tape Self-Audit — 2026-05-01

**Scope**: post-ship internal audit of the TB-5 "smoke tape" evidence per user instruction "没有针对烟测的tape进行审计，由你负责审计，不需要外审". No external auditor invoked; this is a single-AI self-audit.
**Branch / HEAD**: `main` @ `c472823`
**Smoke evidence dir**: `handover/evidence/tb_5_smoke_2026-04-30/`

---

## §1 Verified claims (PASS)

| # | Claim | Verification method | Status |
|---|---|---|---|
| 1 | `oneshot prompt_context_hash="a1f43584a17d1226"` | Read `oneshot_run.log` line 2 | ✅ |
| 2 | 5-session bit-identical hash chain (TB-1 day-1 spike at `first_v4_solve_2026-04-29` + TB-2 + TB-3 + TB-4 + TB-5) | `grep -ho 'prompt_context_hash":"[a-f0-9]*"'` across all 5 dirs returns single unique value `a1f43584a17d1226` | ✅ |
| 3 | `n1 solved=true, verified=true, progress=1` | Read `n1_run.log` line 2 | ✅ |
| 4 | `gp_payload="nlinarith"` | Read `n1_run.log` line 2 | ✅ |
| 5 | `budget_max_transactions=20` (elevated MAX_TX honored) | Read `n1_run.log` line 2 | ✅ |
| 6 | `proof_n1.lean` bit-identical to runtime emit `proofs/mathd_algebra_107_1777577451_73ee91ba.lean` | `sha256sum` match: `dc75963312788193a295633b08b14fc9d2bb93551c9a607baa5fccb5faf98aa9` | ✅ |
| 7 | Proof re-verifies in Lean | `LEAN_PATH=<8 .lake/packages> /home/zephryj/.elan/toolchains/leanprover--lean4---v4.24.0/bin/lean --stdin < proof_n1.lean` → exit=0, no errors, no warnings | ✅ |
| 8 | run_ids + timestamps are session-fresh, not stale repeats | run_ids `oneshot_..._1777577407344` / `n1_..._1777577427580` / proof emit `_1777577451` decode to `2026-04-30 19:30:07/27/51 UTC`; distinct from TB-4's `1777549577331` (`2026-04-30 11:46 UTC`) | ✅ |

---

## §2 Issue 1 — test-count under-report (cosmetic)

**Claim in 5 living docs**: "464/464 cargo test passing"

**Reality**: 617/617 passing (`cargo test --workspace`, 46 suites, 0 failed).

**Root cause**: TB-5 ship work ran `cargo test` (root crate only), missing 153 tests in workspace sub-crates `experiments/minif2f_v4` + `spike/gix_capability`. TB-3 + TB-4 baselines used `cargo test --workspace`, so the comparison was apples-to-oranges.

**Delta vs TB-4 baseline (571)**: 617 − 571 = **46 new TB-5 tests** (consistent with the "~44 new TB-5 tests" claim; off by 2).

**Affected artifacts**:

| File | Line / context | Current value | Should be |
|---|---|---|---|
| `handover/evidence/tb_5_smoke_2026-04-30/README.md` | "Combined with **464/464 cargo test --workspace**" | 464/464 | 617/617 |
| `handover/audits/RECURSIVE_AUDIT_TB_5_2026-04-30.md` | § 7 + verdict | 464 / 464 | 617 / 617 |
| `handover/tracer_bullets/TB_LOG.tsv` | TB-5 row capability_metric: `[464/464 PASS @ 1bdc55a]` | 464/464 | 617/617 |
| `handover/tracer_bullets/TB_LOG.tsv` | pre-header comment line | 464/464 | 617/617 |
| `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` | TB-5 SHIPPED log "Acceptance battery" | 464/464 | 617/617 |
| (commit `1bdc55a` body) | merge commit message (immutable) | 464/464 | (cannot amend; reference this audit) |

**Severity**: medium. Substantive claim ("suite green at TB-5 ship; new tests added; smoke evidence captured") is intact. Headline number wrong by 153 across 5 living docs.

**Remedy**: a single mechanical patch commit on `main`; the merge commit body cannot be amended (already in main + tagged) but its claim is superseded by reference to this audit doc.

---

## §3 Issue 2 — chaintape gap (substantive)

This is the deeper finding the user surfaced via question "你是在chaintape上读取的测试全部信息进行审计的吗？"

### §3.1 What the smoke evidence actually is

| File | Type | Cryptographic chain? |
|---|---|---|
| `oneshot_run.log` | 2-line stdout dump (1 PPUT_RESULT JSON) | ❌ no |
| `n1_run.log` | same shape | ❌ no |
| `proof_n1.lean` | Lean source (CAS-verified bit-identical to runtime emit; verifies under pinned toolchain) | ❌ no chain; sha256 cross-ref only |
| `README.md` | narrative markdown | ❌ no |

### §3.2 What the chaintape is supposed to be

`src/bottom_white/ledger/transition_ledger.rs` defines:

- `LedgerEntry { logical_t, parent_state_root, parent_ledger_root, tx_kind, tx_payload_cid, resulting_state_root, resulting_ledger_root, timestamp_logical, epoch, extensions, system_signature }`
- `LedgerEntrySigningPayload::canonical_digest() -> Hash` — the digest the system_keypair signs
- `Git2LedgerWriter` (line 642) — persists entries to a real on-disk git repo
- `InMemoryLedgerWriter` (line 243) — RAM-only impl used by `cargo test`

`Sequencer::apply_one`:
- stage 1.5 (TB-5 Atom 4): verifies system-emitted variants' `system_signature` against `PinnedSystemPubkeys` for current epoch
- stage 6: signs `LedgerEntrySigningPayload` via `transition_ledger_emitter::sign_ledger_entry` with the runtime keypair
- stage 7: folds `resulting_ledger_root = append(parent_ledger_root, signing_digest)` (deterministic chain)
- stage 9: `LedgerWriter::commit(entry)` persists

This is a real cryptographic chain when it runs. Replay tests `tests/tb_3_rsp1_formal_surface.rs::I29` + `tests/tb_5_challenge_resolve_surface.rs::I80` reconstruct economic state from L4 alone — proving the chain is replay-deterministic.

### §3.3 The gap

**No production binary drives `Sequencer::apply_one`.**

- `bus.rs:73`: `pub sequencer: Option<Arc<Sequencer>>`
- `main.rs`: constructs via `TuringBus::new_legacy()` — sequencer field is `None`
- `experiments/minif2f_v4/src/bin/evaluator.rs`: `grep -rn "turingosv4::state\|turingosv4::bottom_white"` returns **zero hits**. The evaluator binary does not import the kernel state types at all.

The chaintape only exists inside `cargo test` — `InMemoryLedgerWriter` populated then dropped at end of test. No on-disk chain has ever been produced from any LLM-driven run in TuringOS history.

### §3.4 What this means for the audit I performed

I claimed to audit a "smoke tape." What I actually audited:
- 4 files of paper trail (`*.log` + `proof_n1.lean` + `README.md`)
- Plus `cargo test --workspace` re-run (which DOES exercise the chain, but in test-harness InMemoryLedgerWriter, not on the smoke runs themselves)
- Plus Lean re-verification of one proof artifact

If someone tampered with two characters in `n1_run.log`, no invariant in the codebase would catch it. There is no signature on `n1_run.log`; there is no parent-chain entry pointing to it; there is no replay path against it.

The "smoke tape" name is a v3 PaperTape-era metaphor inherited into v4 evidence dirs. It does not name a structural property of the smoke evidence. The TB-5 README §"What this smoke does NOT prove" actually concedes this ("evaluator's PputResult emit path is pre-runtime") — but the language elsewhere (charter § 5.4, RECURSIVE_AUDIT, NOTEPAD) treats the .log files as if they were a stronger guarantee.

### §3.5 Honest restatement of what TB-5 smoke proves

**Genuine signals**:
- Two real evaluator runs happened on 2026-04-30 19:30 UTC (timestamps in `run_id` decode correctly)
- The runs emitted JSON with `prompt_context_hash="a1f43584a17d1226"` matching 4 prior session emits — structural compat for the prompt-build pipeline (NOT for the kernel)
- The n1 run emitted `gp_payload="nlinarith"` and produced a Lean file that re-verifies under pinned toolchain v4.24.0
- These signals are bounded by the integrity of the .log files themselves (which is conventional file-system trust, not cryptographic chain trust)

**What it does NOT prove** (and the README admits at the bottom):
- That TB-5's runtime spine (`emit_system_tx` + apply_one stage 1.5 + ChallengeResolve dispatch arm) was reachable from the evaluator
- That any TypedTx ever traversed `dispatch_transition` during these runs
- That any LedgerEntry was produced
- That the runtime kernel's Anti-Oreo ingress separation was ever exercised at LLM-driven runtime

**These structural properties live entirely in `cargo test --workspace`** (617 tests including replay invariants I29 + I80 + Anti-Oreo ingress barrier I60-I69 + apply_one stage 1.5 forged-sig rejection across 4 system variants). The cargo test suite IS a chain audit (in-memory chain). The smoke evidence is supplementary capability evidence, not chain audit.

### §3.6 Severity

**High.** This is not a code bug — TB-5's kernel additions are real and tested. It is an **honest-naming gap** that bleeds into every TB ship doc. The cumulative debt is 5 TBs of kernel functionality that are tested in `cargo test --workspace` but not exercised by any LLM-driven binary.

---

## §4 Verdict

The TB-5 ship is **substantively defensible**:
- Kernel additions correct, tested, and audited at the appropriate level via `cargo test --workspace` (617/617 green).
- Smoke evidence files are genuine paper trail; no fabrication or stale-repeat.
- Lean re-verification holds end-to-end on the one proof produced.

**Two corrections are warranted**:

### §4.1 (cosmetic) Patch 464→617 across 5 docs

A mechanical find-replace commit on `main`; affects only narrative; no test impact. Will be done unless vetoed.

### §4.2 (substantive) Honest-naming + escalate via architect-review

- Rename "smoke tape" → "smoke evidence" in TB-5 + onward charter / NOTEPAD / LATEST / template language; reserve "tape" / "chaintape" for the LedgerEntry chain (when it exists on-disk from a production run).
- Escalate the production-binary wire-up gap to architect review (`handover/directives/2026-05-01_TB6_ARCHITECT_REVIEW_REQUEST.md` D1 + D2): TB-6 = P2 Agent Runtime atom (recommended) or RSP-3.2 (current ROADMAP plan) + explicit "P2 atom = TB-7" target.

The chaintape gap will not close itself by adding more kernel-only TBs. Each additional kernel-only TB widens it. Architect ruling on D1 + D2 is required.

---

## §5 Audit caveats

- **Single-AI**: this audit was performed by the same AI that shipped TB-5. No external auditor invoked. User authorized this mode explicitly ("由你负责审计，不需要外审"). Reader should treat as self-reported.
- **Disk near-full** (488MB free at audit time) prevented running `cargo test --workspace` from a clean target/ rebuild; instead used incremental rebuild from prior session compile state.
- **Lean toolchain auto-fetch failed initially** (Lean wanted v4.30.0-rc2 download; out of disk); audit pinned to existing local v4.24.0 toolchain, which matches `minif2f_data_lean4/lean-toolchain` exactly. So the re-verification IS faithful to runtime conditions.
