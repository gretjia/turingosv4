# Self-Audit — AgentOutputEnvelope Research

Auditor: Claude Code (self, NOT independent — recorded as input to future
independent Codex witness, NOT as a substitute for it per AGENTS.md §9)
Target: `handover/research/AGENT_OUTPUT_ENVELOPE_RESEARCH_2026-05-26.md`
       + `research/envelope_poc/`
Risk class: 0 (research/docs); future Phase A implementation = Class 1 additive
Date: 2026-05-26

---

## 1. Restricted-surface check (AGENTS.md §6)

| Restricted file | Modified? | Evidence |
|---|---|---|
| `src/kernel.rs` | NO | `git diff --name-only HEAD` empty |
| `src/bus.rs` | NO | (same) |
| `src/sdk/tools/wallet.rs` | NO | (same) |
| `src/state/sequencer.rs` | NO | (same) |
| `src/state/typed_tx.rs` | NO | (same) |
| `src/bottom_white/cas/schema.rs` | NO | (same) |
| RootBox / signing payload | NO | (same) |
| Sequencer admission | NO | adapter-side only |
| Typed tx wire schema | NO | (same) |
| Trust-root / constitution authority | NO | (same) |

Verdict: **CLEAN**. Zero restricted-surface touch. Future Phase A
implementation will add tail-additive serde field to
`src/runtime/attempt_telemetry.rs` (not restricted; Class 1 additive per
schema_version invariance rule documented at attempt_telemetry.rs:281-282).

---

## 2. Engineering-rules check (AGENTS.md §12)

| Rule | Status | Notes |
|---|---|---|
| `rg`/`rg --files` for search | OK | All search via grep/rg in research |
| Use existing patterns/types | OK | PoC uses `serde_json::Value`, mirrors `AttemptOutcome` enum order from existing main crate |
| Structured parsers for data | OK | JSON via `serde_json::from_str` — no ad-hoc string parsing |
| Integer math for money | OK | `size_lots: i64`, no `f64` anywhere in market payload |
| Scoped/reconstructable/shielded read views | OK | `EnvelopeRejectionPayload` is shielded (no raw response, only hash prefix) |
| No raw Lean stderr in agent prompts | N/A | Adapter does not produce prompts |
| Canonical IDs not mixed with shadow | OK | `task_id` is the only ID surface; no shadow IDs introduced |
| Dashboard/report derives from tape | N/A | No dashboard built |
| No workaround closures | OK | Failed-gate behavior is explicit (return Err with subclass) — no skip/null/dashboard-only |

Verdict: **CLEAN**. Specific note: `claimed_evidence_cids` in
`market_signal` payload is *agent-reported* and MUST be re-verified against
CAS by the market predicate gate — schema only verifies it's a list of
strings, not that CIDs exist. This is consistent with the §6 principle
"models don't self-report CIDs that bypass system verification."

---

## 3. Karpathy principles check (AGENTS.md §13)

| Principle | Status | Notes |
|---|---|---|
| First-principles architecture | OK | Designed from FC1 invariant + privacy invariant, not from existing parser idioms |
| Data-flow-first design | OK | Pipeline: bytes → JSON → typed envelope → typed payload → predicate. Each arrow is one function. |
| Monolithic/flat default | OK | Single `validate()` entry; no trait+impl pyramid. PoC src/envelope.rs is one file. |
| Micro-implementation | OK | ~470 LOC PoC, ~600 LOC tests; no unused abstractions; surrogate enums are explicit-named, not generic-typed |
| Direct computation | OK | No async; no executor; one pass through the body |
| Small state machine | OK | validate() returns Result; no driver loop in PoC |
| Transparent data flow | OK | Each rejection path carries (subclass, path, message) tuple |
| No premature abstraction | OK | Did not introduce `EnvelopeValidator` trait + 5 impls — kept it as `match TaskKind`. Future task_kinds tail-add to the match. |

Audit detail (single-impl-trait check per §14 predicate verification): no
new trait + single non-idiomatic impl introduced.

Audit detail (Manager/Factory/Engine/Platform/Framework type check): grep
`grep -rE "struct (.*Manager|.*Factory|.*Engine|.*Platform|.*Framework)" research/envelope_poc/src/` returns 0 results.

Verdict: **CLEAN**.

---

## 4. Predicate verification recipe (AGENTS.md §14 auditor checklist)

Class 0 docs/research does NOT require independent witness per §14
cadence table. However the predicate machine-deterministic batch is
exercisable for the PoC subcrate:

- [x] PR title (if filed) would state risk class — Class 0
- [x] `git diff main --name-only` 未触及 §6 restricted-surface 列表 — empty diff
- [x] `cargo test --manifest-path research/envelope_poc/Cargo.toml --no-fail-fast` exit 0 — 24/24 passed
- [N/A] `cargo test --workspace --no-fail-fast` — not exercised (research scope; PoC is isolated subcrate)
- [N/A] `bash scripts/run_constitution_gates.sh` — not exercised (research scope)
- [N/A] `cargo test --test constitution_matrix_drift` — not exercised (research scope)
- [x] Acceptance criteria with expected output — `EVIDENCE_2026-05-26.md` §2-§4
- [x] PoC's own predicate verification recipe — see §6 below
- [x] No new `Manager` / `Factory` / `Engine` / `Platform` / `Framework` type — verified by structural grep
- [x] No new trait + single non-idiomatic impl — verified by inspection (validate is a plain fn, not a trait method)
- [x] No new board-as-truth file — `EnvelopeRejectionPayload` is CAS-resident in the design; no board file written
- [x] No new global latest pointer — `validate()` takes explicit `ValidateContext` reference, no globals

For Phase A (Class 1 additive) future cadence: full §14 predicate verification
required.

---

## 5. Obligation ledger check (AGENTS.md §16 + OBLIGATIONS.md)

Active obligations at session start:

| OBL | Level | Status | Affected by this research? |
|---|---|---|---|
| OBL-001 | must | open | NO (DeepSeek 15-persona Chrome test, unrelated) |
| OBL-002 | must | satisfied | NO |
| OBL-003 | must | satisfied | NO |
| OBL-004 | must | in_progress | NO — this research does not touch the 28-atom violator-repair scope |
| OBL-005 | must | in_progress (per main d0bb511d) | DECOUPLED — round-4 rewrite of §10 confirms the research is OFF the OBL-005 closure path. FC3 typed-tx is already live on main (LogFeedbackArchiveTx, ArchitectProposalTx, VetoDecisionTx, etc.). OBL-005 now closes on broad real-world full-system participation evidence, not on schema design. The research ship does not advance OBL-005 and does not require OBL-005 to close. |

Verdict: **No regression**. No `Level=must` is moved from `satisfied` to
any other state. OBL-005 future-dependency relationship is explicitly
disclaimed in research doc §10. No obligation is silently substituted
(per `feedback_anchor_drift` lesson V-010).

---

## 6. PoC's own predicate verification recipe

```bash
cd /tmp/turingosv4-agent-schema-research
cargo test --manifest-path research/envelope_poc/Cargo.toml --no-fail-fast 2>&1 | tee /tmp/poc_test_output.txt
grep -q "test result: ok. 8 passed" /tmp/poc_test_output.txt  # decoupling
grep -q "test result: ok. 2 passed" /tmp/poc_test_output.txt  # fc1_invariant
grep -q "test result: ok. 14 passed" /tmp/poc_test_output.txt # robustness
test $(grep -c "FAILED" /tmp/poc_test_output.txt) -eq 0
echo "PREDICATES-GREEN"
```

Expected output last line: `PREDICATES-GREEN`. Confirmed (manual run captured
in `EVIDENCE_2026-05-26.md` §2).

---

## 7. Cross-checks against research memory rules

| Memory rule | Compliance check | Status |
|---|---|---|
| `feedback_no_retroactive_evidence_rewrite` | PoC does NOT mutate historical L4/L4.E/CAS evidence. Tail-additive serde plan explicitly preserves v1/v2 grandfathered bytes via `decode_attempt_telemetry_compat` (already documented in `src/runtime/attempt_telemetry.rs:90-91`). | OK |
| `feedback_chaintape_externalized_proposal` | "1 LLM call → 1 Attempt Node": PoC envelope is 1 envelope per 1 LLM call; not per-tactic. Aligned with `AttemptKind::ExternalizedLlmCycle = 0`. | OK |
| `feedback_no_workarounds_strict_constitution` | Envelope failure is hard rejection (Err return), not skip/null/empty-evidence. No dashboard-only proof. | OK |
| `feedback_audit_after_evidence` | This self-audit happens AFTER PoC tests passed (24/24). Schema-only audit was NOT used as a gate. | OK |
| `feedback_defer_abstraction_until_second_impl` | PoC validate() is a single fn + match on TaskKind enum, NOT a trait+impl pyramid. The 5 task_kinds in the match are "concrete impl 1..5"; a trait would only be justified at impl 2+ (per memory) — and even then only by an explicit user-facing API need. | OK |
| `feedback_tape_first_real_tests` | PoC is unit-test only (research scope). Phase A real-run requirement (200 attempts) is explicitly recorded in PLAN §3-P5. | OK |
| `feedback_smoke_before_batch` | PoC found 1 design flaw in Round 1 (the side-cap bug). Round 2 = full smoke green. No batch run attempted before smoke clean. | OK |

---

## 7.5 Round 2 finding — surrogate enum SECOND-SOURCE-DRIFT (2026-05-26)

While writing the Codex audit dispatch packet (`CODEX_AUDIT_DISPATCH_PACKET_2026-05-26.md`
§8 check C1), I re-grepped the main crate to enumerate the exact mapping
that an independent auditor would need to verify. Found a real drift:

| Discriminator | Main crate (`rejection_evidence.rs:204-241`) | PoC surrogate Round 1 | Status |
|---|---|---|---|
| 0 | PredicateFailed | PredicateFailed | OK |
| 1 | PolicyViolation | PolicyViolation | OK |
| **2** | **EscrowMissing** | BudgetExceeded | **DRIFT** |
| **3** | **InvariantViolation** | StateRootStale | **DRIFT** |
| **4** | **MalformedPayload** | SignatureInvalid | **DRIFT** |
| **5** | **InsufficientBalance** | NonceReplay | **DRIFT** |
| 6 | LeanFailed | LeanFailed | OK |
| 7 | ParseFailed | ParseFailed | OK |
| 8 | SorryBlocked | SorryBlocked | OK |
| 9 | LlmError | LlmError | OK |

Root cause: I drafted the surrogate from "generic crypto/state-machine
patterns" without grepping the real source. Behavior was unaffected because
the PoC mapping only emits variants 1 and 7 from `EnvelopeValidationSubclass`,
but the *claim* "surrogate mirrors the main crate" was false for variants 2-5.

**This is exactly the kind of finding that audit check C1 was designed to
catch.** Writing the audit packet surfaced it *before* a real Codex auditor
ran. Recorded here as confirming evidence that the audit-packet methodology
works.

Fix (single Edit): renamed variants 2-5 in
`research/envelope_poc/src/envelope.rs:34-43` and added a note explaining
the drift. Tests re-ran 24/24 GREEN.

Self-audit takeaway: **surrogate enums must be derived by grep against the
main crate, not by guess**. If main crate is unavailable, the audit packet
must call this out as `RECONSTRUCTION-FAILURE` / `SECOND-SOURCE-DRIFT`
rather than ship.

---

## 8. Independent surfaces NOT covered by this self-audit

A self-audit cannot certify these — they require independent Codex witness:

S1. **Surrogate enum byte-stability**: I claim `AttemptOutcomeSurrogate` /
    `RejectionClassSurrogate` mirror main crate by variant order. An
    independent auditor must read both files and confirm.

S2. **Privacy invariant byte-fence**: `EnvelopeRejectionPayload` serialization
    being free of raw bytes is verified by string substring check in tests.
    A real CAS round-trip + canonical-encode byte inspection is Phase A scope.

S3. **Reconstruction property**: Phase A claim "`run_proof` deterministically
    re-classifies a body to the same EnvelopeValidationSubclass on replay"
    is asserted but not yet exercised against the real runner. Codex witness
    should evaluate the determinism claim.

S4. **OBL ledger reconciliation against TB_LOG.tsv**: my OBL-005 dependency
    disclaimer assumes `TB-FLOWCHART-FC2-FC3-CLOSURE` charter scope; an
    auditor should verify the dependency map matches the active charter.

---

## 9. Verdict (self)

PoC + design pair shows **NO restricted-surface drift, NO Karpathy violation,
NO obligation regression**. 24/24 tests GREEN. 1 design flaw caught and
fixed within PoC (the side-cap bug — preserved as research finding).

**Self-verdict**: `SELF-AUDIT-GREEN`.

**However**: this does not substitute for independent Codex witness.
Class 0 cadence does not require it, but for the future Phase A charter
(Class 1 additive) it is recommended to first ratify the *research* via
single Codex witness (per memory `feedback_dual_audit` 2026-05-24,
Gemini auditor dropped), output domain
`{NO-VIOLATION, VIOLATION-FOUND, RECONSTRUCTION-FAILURE, SECOND-SOURCE-DRIFT}`.

See `CODEX_AUDIT_DISPATCH_PACKET_2026-05-26.md` for the dispatch template.

---

(结束)
