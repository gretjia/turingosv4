# Next Session Boot Prompt — 2026-05-11 (TB-N2 B2 candidate impl on parallel branch; ready for real-LLM smoke + PRE-§8 dual audit)

> Paste the **`USER PROMPT`** block at the bottom into the next Claude session.

---

## §0. Where we are

**Branch**: `feat/n2-b2-event-resolve` (parallel STEP_B branch; NOT merged to main)
**HEAD on branch**: `7dc2aa0`  (TB-N2 B2 — EventResolveTx system-emit on OMEGA-Confirm — Class-3 impl)
**origin/main HEAD**: `00d7024` (TB-N2 B0 charter + B1 gap audit; pushed 2026-05-10)

**Authorization stance** (per CLAUDE.md §9 + §10):
- User authorization 2026-05-11: "Charter + plan + Option 1 — begin B2 candidate impl on parallel branch"
- This DID authorize: candidate impl + parallel branch + STEP_B protocol + HOLD at PRE-§8 boundary
- This DID NOT authorize: B2 §8 ship sign-off (single-word "confirm" insufficient per §9)
- User must give explicit multi-clause §8 to merge to main + ship

## §1. What landed in last session (3 commits)

| Commit | Subject | Class |
|--------|---------|-------|
| `f3ff343` (on origin/main) | TB-N1 A4 post-ship final smoke evidence 6/6 GREEN | 2 |
| `00d7024` (on origin/main) | TB-N2-POLYMARKET-CPMM-LIFECYCLE B0 charter + B1 gap audit | 0 |
| `7dc2aa0` (on `feat/n2-b2-event-resolve`) | TB-N2 B2 EventResolveTx system-emit candidate impl | 3 (Class-4 boundary touched at canonical signing payload) |

## §2. B2 candidate impl scope (`7dc2aa0`)

Closes gap audit §3.3: pre-B2 `TaskMarketState::Finalized` was READ at 5+ admission sites but WRITE 0 sites. B2 adds Open → Finalized writer-side via system-emit on the OMEGA-Confirm path (Option 1 minimal resolution authority per charter §5).

**Files edited (13 files, 1003 insertions / 11 deletions)**:

| Surface | Change |
|---------|--------|
| `src/state/typed_tx.rs` (STEP_B) | +EventResolveTx struct (6 wire fields) + SigningPayload + canonical_digest + to_signing_payload + DOMAIN_SYSTEM_EVENT_RESOLVE const + TypedTx::EventResolve variant + dispatchers + HasSubmitter impl + 2 NEW TransitionError variants (EventResolveTaskNotFound + EventAlreadyResolved) with Display impls |
| `src/state/sequencer.rs` (STEP_B) | +EVENT_RESOLVE_DOMAIN_V1 + event_resolve_accept_state_root helper + SystemEmitCommand::EventResolve variant + EmitSystemError::EventResolveTaskNotFound variant + emit_system_tx body + dispatch_transition arm (Open → Finalized monotonic gate + state_root advance + monetary invariants) + agent-ingress rejection + 4 system-tx dispatchers + 2 telemetry mappers |
| `src/bottom_white/ledger/system_keypair.rs` (Class-4 canonical signing payload boundary) | +CanonicalMessage::EventResolveSigning + sign_event_resolve + digest discriminator branch |
| `src/bottom_white/ledger/transition_ledger.rs` | +TxKind::EventResolve = 18 (tail-append; preserves replay-determinism) |
| `src/runtime/adapter.rs` | +tb_n2_emit_event_resolve_after_finalize poll-then-emit helper (mirrors tb8_emit_finalize_after_verify) |
| `src/economy/monetary_invariant.rs` | extended assert_no_post_init_mint match (B2 is pure status mutation; trivially no mint) |
| `src/runtime/audit_assertions.rs` | TxKindCounts +event_resolve field + extract_all_agent_ids arm |
| `src/runtime/run_summary.rs` | extract_tx_id match arm |
| `experiments/minif2f_v4/src/bin/evaluator.rs` | 2 NEW EventResolve emit hooks in OMEGA-Confirm exit paths (full-proof ~line 2823 + per-tactic ~line 3515) |
| `experiments/minif2f_v4/src/bin/comprehensive_arena.rs` | happy-path arena task A fires EventResolve after FinalizeReward |
| `tests/constitution_n2_event_resolve.rs` (NEW) | SG-N2-B2.1..B2.8 — 8/8 GREEN |
| `scripts/run_constitution_gates.sh` | +constitution_n2_event_resolve entry |
| `genesis_payload.toml` | 8 STEP_B / adjacent files rehashed (Trust Root re-pins) |

## §3. Validation snapshot at `7dc2aa0`

| Check | Value | Δ vs pre-B2 (1447 baseline) |
|---|---|---|
| `cargo check --workspace` | clean | — |
| `cargo test --workspace --test-threads=1` | **1447 / 0 / 151** | +8 (was 1439) |
| `bash scripts/run_constitution_gates.sh` | **287 / 0 / 1** | +8 (was 279) |
| `cargo test -p minif2f_v4 --test trust_root_immutability` | PASS | rehashed 8 files |
| Constitution matrix | 0 RED + 0 AMBER preserved | — |

## §4. Next session — exact resume plan

### Step 1: Verify branch + sanity check
```bash
git branch --show-current   # should be feat/n2-b2-event-resolve
git log --oneline -3        # 7dc2aa0 → 00d7024 → f3ff343
cargo check --workspace
bash scripts/run_constitution_gates.sh   # 287/0/1 expected
```

### Step 2: /runner-preflight (mandatory per MEMORY MUST CHECK BEFORE)
Before launching any `scripts/run_*.sh`. Confirm:
- clean tree (or only smoke evidence + audit untracked files)
- binary mtime > HEAD `7dc2aa0` source mtime
- evidence-immutability (don't overwrite existing dirs)
- Class-3 risk class declared
- FC-trace pre-stated
- Charter exists (`TB_N2_POLYMARKET_CPMM_LIFECYCLE_charter_2026-05-10.md`)

### Step 3: Build release binary
```bash
cargo build --release --bin evaluator
# binary mtime must be AFTER 7dc2aa0
```

### Step 4: Smoke setup
```bash
cat > /tmp/b2_smoke_problems.txt <<EOF
aime_1983_p1.lean
aime_1983_p2.lean
aime_1983_p3.lean
EOF
```
Reason for these 3: `aime_1983_p2` has OmegaAccepted reproducibility across 4 consecutive prior smokes (deepseek-v4-flash) — **guaranteed B2 trigger path**. The other two test the EXHAUSTED path (B2 should NOT fire there).

### Step 5: Run smoke (B2 binding)
```bash
TS=$(date -u +%Y%m%dT%H%M%SZ)
RUN_TAG="stage_b3_smoke_b2_${TS}"
CONDITION=n1 TURINGOS_STAGE_B3_DIRTY_OK=1 PER_PROBLEM_TIMEOUT_S=900 \
  bash scripts/run_stage_b3.sh "$RUN_TAG" /tmp/b2_smoke_problems.txt 1 1 \
  > /tmp/b2_smoke_${TS}.out 2>&1 &
```
Wall-clock expectation: 15-30 min (3 problems × 2 models + Qwen tail of ~10-15 min).

### Step 6: Verify B2 emission + chain invariant
After SUMMARY.json appears:
```bash
# For OmegaAccepted cells (expected: deepseek/aime_1983_p2), grep evaluator.stderr for B2 emit:
grep -E "tb-n2/b2.*EventResolve emitted" handover/evidence/stage_b3_smoke_b2_*/deepseek-v4-flash/seed1/rep1/P002_aime_1983_p2/evaluator.stderr

# chain_invariant.json verdict=Ok delta=0 across 6/6 cells:
for f in handover/evidence/stage_b3_smoke_b2_*/*/seed1/rep1/P00*/chain_invariant.json; do
  jq -c '{cell:input_filename, verdict:.invariant_verdict, delta:.delta}' "$f"
done
```

**Expected witness**: ≥1 cell with `EventResolve emitted` log line + 0 cells with EventResolveTaskNotFound or EventAlreadyResolved errors. Aggregate verdict=Ok delta=0 across all 6 cells.

### Step 7: PRE-§8 dual audit dispatch (Codex G2 + Gemini DT)
Per `feedback_dual_audit` Class-4 timing (canonical signing payload boundary touched → full dual):
- **Codex G2** (Class-3 economic-mutator-adjacent + Class-4 canonical-signing-payload boundary): use `codex:rescue` skill or direct Bash `codex exec --dangerously-bypass-approvals-and-sandbox -C $PWD < prompt.md > out.log 2>&1 &` per `feedback_codex_bash_exec_direct_dispatch`
- **Gemini DT**: use general-purpose Agent or external dispatch
- Conservative-merge VETO > CHALLENGE > PASS per `feedback_dual_audit_conflict`
- Round cap = 2 per `feedback_elon_mode_policy` (CHALLENGE → R2 fix → re-audit)
- Stop after R2 if both PASS; halt if either VETO

Audit prompt template available at `handover/audits/CODEX_STAGE_C_OVERALL_AUDIT_2026-05-09_R1.md` (mirror structure). Audit must check:
- Q1 EventResolve schema bit-stability (6 wire fields canonical encode)
- Q2 sign_event_resolve domain prefix non-collision (`b"turingosv4.system_sig.event_resolve.v1"` distinct from 5 prior system domains)
- Q3 dispatch arm monotonic resolution gate (Open → only-Open rejected; Bankrupt → reject; Expired → reject; Finalized → reject)
- Q4 emit_system_tx fail-closed on missing task_id (EventResolveTaskNotFound at construction)
- Q5 agent-ingress rejection arm (Anti-Oreo barrier)
- Q6 monetary invariant defense-in-depth (assert_no_post_init_mint + assert_total_ctf_conserved empty exempt list)
- Q7 state_root_t advance domain separation (EVENT_RESOLVE_DOMAIN_V1 distinct from TASK_BANKRUPTCY_DOMAIN_V1)
- Q8 evaluator hook idempotency (poll-then-emit Ok(false) on already-resolved → no double emit)
- Q9 Trust Root rehash completeness (8 files: 7 STEP_B + 1 adapter)

### Step 8: §8 packet draft
Path: `handover/directives/2026-05-11_TB_N2_B2_§8_PACKET.md`
Sections:
- §1 Architect §8 verbatim (TBD; user must say after dual audit PASS)
- §2 PRE-§8 dual audit verdicts (paste Codex G2 + Gemini DT R-final)
- §3 Q-closure summary (per Q above)
- §4 Validation baseline at HEAD
- §5 Forward (B3 next — CpmmPoolResolveTx; needs separate per-atom §8)

### Step 9: HOLD for explicit multi-clause user §8 sign-off
Per CLAUDE.md §9 + §10 + `feedback_no_workarounds_strict_constitution`. Do NOT merge `feat/n2-b2-event-resolve` to main without:
- Architect §8 verbatim multi-clause statement (e.g. "好，确认可以 ship" / "签字，同意后续执行" / "同意 sign-off")
- Both audits PASS aggregate
- All §8 packet sections filled

## §5. Forward queue (after B2 ships)

Per charter §3 atom decomposition:
- **B3** CpmmPoolResolveTx system-tx (Class-4 STEP_B; per-atom §8) — flips pool.status Active → Resolved; triggered by B2 emit chain
- **B4** CpmmLpUnwindTx agent-tx (Class-4 STEP_B; per-atom §8) — agent burns LP shares + withdraws pro-rata reserves; **closes the LP funds dust-locked gap** (gap audit §3.4)
- **B5** Asymmetric pool seed (Class-3; per-atom §8 if reclassified Class-4) — relax `UnbalancedPoolSeed` per architect §2.1 general k
- **B6** End-to-end CPMM lifecycle real-LLM smoke (Class-2; after B2-B5 ship) — first real-LLM full Polymarket cycle
- **B7** Phase B overall §8 cap

## §6. Critical files for cold-start orientation

Read in this order:

1. **`CLAUDE.md`** — §2 PRIME OPERATING MODE + §9 single-word ban + §10 Class-3/4 authorization
2. **`handover/ai-direct/LATEST.md`** — top "✅ Session #36 (continued)" Phase 2 closure block
3. **This file** (`NEXT_SESSION_PROMPT_2026-05-11_post_b2_impl.md`)
4. **`handover/tracer_bullets/TB_N2_POLYMARKET_CPMM_LIFECYCLE_charter_2026-05-10.md`** — charter (B2 in §3 atom decomp; resolution authority Option 1 in §5)
5. **`handover/audits/STAGE_C_POLYMARKET_CPMM_LIFECYCLE_GAP_AUDIT_2026-05-10.md`** — gap audit (§3.3 + §3.4 critical gaps; §4 resolution authority matrix; §5 monetary invariant analysis)
6. **`MEMORY.md`** index — entries `feedback_no_batch_class4_signoff`, `feedback_dual_audit`, `feedback_audit_after_evidence`, `feedback_codex_bash_exec_direct_dispatch`, `feedback_no_workarounds_strict_constitution`

## §7. What NOT to do

- ❌ Do NOT merge `feat/n2-b2-event-resolve` to main without explicit multi-clause architect §8
- ❌ Do NOT batch B2 §8 with B3 / B4 / B7 (per `feedback_no_batch_class4_signoff`)
- ❌ Do NOT dispatch G1 audit (BEFORE-evidence); only G2 (AFTER-evidence smoke) per `feedback_audit_after_evidence`
- ❌ Do NOT skip /runner-preflight before smoke
- ❌ Do NOT push `feat/n2-b2-event-resolve` to origin without architect §8 (the branch is local-only currently)
- ❌ Do NOT take single-word "confirm" / "ok" / "go" / "可以" as §8 sign-off

## §8. Memory verification

These MEMORY entries are load-bearing for next session:
- `feedback_no_batch_class4_signoff` (B3 / B4 / B7 each need own §8)
- `feedback_dual_audit` (Class-3/4 hybrid; PRE-§8 timing rule at packet draft time)
- `feedback_audit_after_evidence` (G2 only; no G1 before smoke)
- `feedback_codex_bash_exec_direct_dispatch` (fallback if codex:rescue rejected)
- `feedback_no_workarounds_strict_constitution` (strict landing; no §10 reclassification kludge)
- `feedback_real_problems_not_designed` (real-LLM smoke with adversarial problem mix)
- `feedback_pre_runner_checklist` (/runner-preflight 7-stage)

---

## USER PROMPT (paste this into next Claude session)

```
Session resume 2026-05-11. TB-N2 B2 candidate impl committed on parallel
branch `feat/n2-b2-event-resolve` HEAD 7dc2aa0 (NOT merged to main;
origin/main still at 00d7024).

Validation at 7dc2aa0:
- cargo check --workspace: clean
- cargo test --workspace --test-threads=1: 1447/0/151 (+8 vs 1439)
- bash scripts/run_constitution_gates.sh: 287/0/1 (+8 vs 279)
- cargo test -p minif2f_v4 --test trust_root_immutability: PASS
- 8 STEP_B / adjacent files rehashed in genesis_payload.toml

What B2 closes: pre-B2 `TaskMarketState::Finalized` was READ at 5+ admission
sites but WRITTEN 0 times (gap audit §3.3). B2 adds Open → Finalized
writer-side via system-emit on the OMEGA-Confirm path (Option 1 minimal
resolution authority per charter §5; FinalizeReward success → EventResolve
emit chain via new `tb_n2_emit_event_resolve_after_finalize` adapter helper).

Authorization scope at session start: candidate impl + parallel branch
authorized 2026-05-11; HOLD at PRE-§8 dual audit boundary per CLAUDE.md §9
single-word ban. NO §8 ship sign-off yet.

Next steps (in order):
1. /runner-preflight per MEMORY MUST CHECK BEFORE
2. cargo build --release --bin evaluator
3. Real-LLM smoke: 3 problems (aime_1983_p1/p2/p3) × 2 models × seed=1 rep=1
   = 6 cells. aime_1983_p2 has OmegaAccepted reproducibility across 4 prior
   smokes → guaranteed B2 trigger path. ~15-30 min wall, ~$3-5 LLM budget.
4. Verify ≥1 cell with `EventResolve emitted` log line; chain_invariant.json
   verdict=Ok delta=0 across 6/6 cells.
5. PRE-§8 dual audit dispatch (Codex G2 + Gemini DT) on smoke evidence.
   Class-4 canonical-signing-payload boundary touched → full dual; conservative-
   merge VETO > CHALLENGE > PASS. Round cap=2.
6. Draft §8 packet at handover/directives/2026-05-11_TB_N2_B2_§8_PACKET.md
7. HOLD for explicit multi-clause user §8 sign-off.

Forward (after B2 ships): B3 CpmmPoolResolveTx (Class-4 STEP_B; own §8) →
B4 CpmmLpUnwindTx (Class-4 STEP_B; own §8; closes LP funds-locked gap §3.4)
→ B5 asymmetric seed → B6 end-to-end real-LLM smoke → B7 Phase overall §8.
Per `feedback_no_batch_class4_signoff` each Class-4 atom needs OWN §8.

Read first:
1. CLAUDE.md §2 + §9 + §10
2. handover/ai-direct/NEXT_SESSION_PROMPT_2026-05-11_post_b2_impl.md (this
   prompt's full source — has §4 step-by-step resume + §5 forward queue +
   §7 do-NOT list)
3. handover/tracer_bullets/TB_N2_POLYMARKET_CPMM_LIFECYCLE_charter_2026-05-10.md
4. handover/audits/STAGE_C_POLYMARKET_CPMM_LIFECYCLE_GAP_AUDIT_2026-05-10.md
5. MEMORY.md (feedback_no_batch_class4_signoff, feedback_dual_audit,
   feedback_audit_after_evidence, feedback_codex_bash_exec_direct_dispatch,
   feedback_no_workarounds_strict_constitution)

Constraints:
- Do NOT merge feat/n2-b2-event-resolve without explicit multi-clause §8
- Do NOT batch B2 §8 with B3/B4/B7
- Do NOT dispatch G1 audit; only G2 (after smoke evidence)
- Do NOT push the parallel branch to origin without architect ship grant
```

---

**End of TB-N2 B2 post-impl boot prompt.**
