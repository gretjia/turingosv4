# A8 Phase A → B Exit Audit — Chronological History

**Companion**: `A8_EXIT_PACKET_2026-04-26.md` (canonical current-state). This doc is the **append-only** chronology of round-N outcomes + the in-cycle fixes that closed each round's findings. Past entries are FROZEN — corrections to historical facts go in a new "Correction (round-M)" sub-section under the affected round, never via in-place edit.

**Constitutional pattern**: mirrors `constitution.md` + Art. V.3 amendment log; mirrors `PREREG_PPUT_CCL_2026-04-26.md` (frozen) + `PREREG_AMENDMENT_p0_defer_2026-04-25.md` (delta). Per C-075 (DO-178C tool qualification) the gate machinery itself must be qualifiable; this history doc is in Trust Root so the chronology cannot be silently rewritten.

**Why this doc exists** (A8e7 structural rewrite, 2026-04-26): rounds 1–6 of the Phase A → B audit produced a recurring documentary CHALLENGE pattern. Diagnosis: `A8_EXIT_PACKET` was conflating two opposite temporal modes (stable-snapshot artifact + append-only chronology) in one document, so every fix to "current state" generated new staleness in "round-N retrospective" sections. Splitting the two modes into two documents removes the category error.

---

## Round 1 (2026-04-26) — first dual review

**Inputs**:
- Packet: `handover/audits/A8_EXIT_PACKET_2026-04-26.md` @ commit `60292dc`
- Test baseline: 261 PASS / 29 ignored / 0 failed
- Trust Root: 30-entry manifest (round-1 packet's count; later corrected to 31 — see round-2)

**Verdicts**:
- Codex R1: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26.md`
- Gemini R1: **VETO / high** — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26.md`
- Merged (per memory `feedback_dual_audit_conflict`, VETO > CHALLENGE > PASS): **VETO**

**Findings** (5 Codex + 4 Gemini; 1 Gemini Q1 finding about hybrid_v1 was determined invalid — pre-Phase A change):
1. (Codex#2 + Gemini Q4 — convergent) `run_corr_id` (FC events) vs `make_pput::run_id` (jsonl) ms drift; Phase D consumers cannot join FC events to v2 rows by equality.
2. (Codex#3) FC1-N12 emitted only in `run_oneshot`; the 2 `verify_omega_detailed` and 1 `verify_partial` calls in `run_swarm` increment `verifier_wait_ms` but don't emit FC events.
3. (Codex#4 + Gemini Q5 — convergent) `detect_provider` routing matrix: `Qwen/Qwen2.5-7B-Instruct` misroutes to DashScope; smoke bypasses proxy so the bug isn't caught.
4. (Codex#5) Trust Root manifest count off-by-1 (packet says 30, actual 31); genesis_payload.toml header still says "Total: 25 files".
5. (Codex#1) `PREREG_AMENDMENT § 2` calls `p_0=0.10` substitution "strictest possible substitute" — backwards (0.10 is least-strict admissible for `j-RR ≤ p_0`).
6. (Gemini Q4) Hand-rolled JSON encoder in `fc_trace.rs` vs `serde_json` already in deps — minor.
7. (Gemini Q5) Smoke test bypasses proxy so routing logic untested — convergent with #3.
8. (**Gemini VETO Q6**) Multi-key round-robin in `llm_proxy.py` (V3L-27 mitigation) lacks any automated conformance test. Manual verification via `[2,2,2]` distribution is one-time, not recurring. **REDESIGN-level for atom A7.**

**Round-1 fixes shipped (`A8e`, commit `5a56ff6`)** — six fixes:
- **F1** unified `run_id`: new `experiments/minif2f_v4/src/run_id.rs::mint_run_id` minted at run_swarm/run_oneshot entry; threaded into both `emit_event` and `make_pput`. Closes #1.
- **F2** new `scripts/test_llm_proxy.py` 15-test suite pinning round-robin `[2,2,2]` invariant. Closes Gemini VETO Q6 at the artifact level.
- **F3** `detect_provider` reordered: slash-form ⇒ siliconflow first; bare-qwen ⇒ dashscope only without slash. Closes #3.
- **F4** added FC1-N12 emit to all 3 swarm verify sites (`verify_omega_detailed` × 2 + `verify_partial`). Closes #2.
- **F5** Trust Root count + genesis_payload.toml header reconciled to 31 entries (corrected for A3's `agent_models.rs` already in TR before A1). Closes #4.
- **F6** PREREG_AMENDMENT § 2 reworded: `p_0=0.10` → least-strict admissible ceiling; explicit Type-I implications paragraph added. Closes #5.

---

## Round 2 (2026-04-26) — post-A8e

**Inputs**:
- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `5a56ff6` (post-A8e)
- Test baseline: 264 PASS / 29 ignored / 0 failed (Rust); 15/15 PASS (Python proxy tests)
- Trust Root: 33-entry manifest

**Verdicts**:
- Codex R2: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R2.md`
- Gemini R2: **CHALLENGE / high** (de-escalated from VETO) — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R2.md`
- Merged: **CHALLENGE**. No VETO. Both auditors confirmed round-1 fixes are letter-correct; remaining gaps procedural/documentary.

**Findings** (3 Codex + 2 Gemini, convergent):
1. (Codex R2#1 + Gemini R2#1) F2's `test_llm_proxy.py` exists but is documented as manual-only; no CI integration. Tests-not-running-automatically = documentation, not gates. Closes Gemini's round-1 VETO at the *artifact* level but NOT at the *process* level.
2. (Codex R2#2) PREREG_AMENDMENT § 2 was corrected (F6) but § 8 audit-requirements paragraph still says "strictest plausible bar is conservative" — direct contradiction with § 2.
3. (Codex R2#3 + Gemini R2#2) Packet § 5 Risk #5 ("No FC1-N12 emit in run_swarm verify path") is stale (closed by F4); packet § 6 Q7.a/b numbers stale (261/30 vs actual 264/33); TRACE_MATRIX has stale `run_corr_id` symbol row + "in CI" claim.

**Round-2 fixes shipped (`A8e2`, commit `0af47b7`)** — three fixes:
- **G1** `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` Rust integration test that shells to `python3 scripts/test_llm_proxy.py`; runs on every `cargo test --workspace`. Closes finding #1.
- **G2** PREREG_AMENDMENT § 8 reworded to remove "strictest plausible bar"; consistent with § 2 throughout. Closes finding #2.
- **G3** Packet § 2 cumulative table extended; § 5 Risk #5 removed; § 6 Q7.a/b numbers updated 264/34; TRACE_MATRIX `run_corr_id` row replaced with unified `run_id` row; "in CI" softened. Closes finding #3.

---

## Round 3 (2026-04-26) — post-A8e2

**Inputs**:
- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `0af47b7` (post-A8e2)
- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 15/15 PASS (Python)
- Trust Root: 34-entry manifest

**Verdicts**:
- Codex R3: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R3.md`
- Gemini R3: **CHALLENGE / high** — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R3.md`
- Merged: **CHALLENGE**. Both auditors flagged narrow doc/source-comment defects; no VETO.

**Findings**:
1. (Codex R3#1) Packet line 118 still says substitution is "Mathematically conservative (strictest plausible bar)"; Q2.a still says "described as conservative"; genesis_payload.toml header still says "conservative ceiling". The § 2 / § 8 fix in F6/G2 didn't propagate to these other sites.
2. (Codex R3#2) Packet § 3 A6 atom description says "6 anchor sites + 1 oneshot"; § 6 Q4.a says "FC1-N12 (oneshot verify only)"; § 6 Q4.d describes pre-F1 ms drift as if open. Stale relative to F4 + F1.
3. (Codex R3#3) `tests/llm_proxy_python_conformance.rs` returns success when `python3` is missing (soft skip via `eprintln + return`). For a VETO-closing conformance gate, missing python3 should fail closed.
4. (Gemini R3#1) Convergent with Codex R3#2 — Q4.d stale.
5. (Gemini R3#2 — non-blocking observation) `make_pput` signature has 21 args; refactor to builder pattern recommended for Phase B+.

**Round-3 fixes shipped (`A8e3`, commit `3d38ba5`)** — six fixes:
- **H1** Packet § 3 A1 description rewritten: "Mathematically conservative" → "least-strict admissible value" with Type-I implications + cross-ref to § 2.
- **H2** Packet § 3 A6 description bumped 6 → 9 anchor sites; explicitly listed F4-added swarm sites.
- **H3** Packet § 6 Q2.a / Q4.a / Q4.d marked **CLOSED** with closure rationale.
- **H4** `genesis_payload.toml` TR header phrasing about A1: "conservative ceiling" → "max-tolerated ceiling — least-strict admissible".
- **H5** TRACE_MATRIX § 5 item 7: "(commit pending) + 6 wired" → "CLOSED + 9 wired".
- **H6** G1 wrapper test now FAILS CLOSED on missing `python3`; explicit opt-out `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1` for deliberate downgrades.

---

## Round 4 (2026-04-26) — post-A8e3

**Inputs**:
- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `3d38ba5` (post-A8e3)
- Test baseline: 265 PASS / 29 ignored / 0 failed; 15/15 Python
- Trust Root: 34-entry manifest

**Verdicts**:
- Codex R4: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R4.md`
- Gemini R4: **PASS / high** — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R4.md`. *First round one auditor reached PASS.*
- Merged: **CHALLENGE**. Conservative merge wins per memory.

**Findings** (3 Codex + 1 Gemini non-blocking):
1. (Codex R4#1) Packet title + date metadata still say "round 2"; per-atom Trust Root deltas in § 3 stale (A5 says 25→26 vs actual 26→27, A6 26→27 vs 27→28, A7 27→30 vs 28→31).
2. (Codex R4#2 + R4#3) TRACE_MATRIX § 1 line 11 says "Six anchor sites wired in run_swarm" but 1 site is in run_oneshot.
3. (Codex R4#3) `tests/llm_proxy_python_conformance.rs` file header docstring still says "if not, it skips with a clear diagnostic" — contradicts H6's fail-closed runtime.
4. (Gemini R4 non-blocking) `make_pput` arg count is now 24 not 21 (F1 added run_id parameter). Reaffirms the round-3 deferred refactor.

**Round-4 fixes shipped (`A8e4`, commit `8693789`)** — three fixes:
- **I1** Packet metadata "round 2" → "running through rounds 1–N" with reader pointer to latest section.
- **I2** Per-atom Trust Root deltas in packet § 3 corrected: A5 → 26→27, A6 → 27→28, A7 → 28→31. TRACE_MATRIX § 1 line 11 anchor-site location corrected: "5 in run_swarm + 1 in run_oneshot".
- **I3** Wrapper docstring updated: "FAILS CLOSED with a clear diagnostic"; explicit opt-out env var documented.

---

## Round 5 (2026-04-26) — post-A8e4

**Inputs**:
- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `8693789` (post-A8e4)
- Test baseline: 265 PASS / 29 ignored / 0 failed; 15/15 Python
- Trust Root: 34-entry manifest

**Verdicts**:
- Codex R5: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R5.md`
- Gemini R5: **PASS / high** — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R5.md`
- Merged: **CHALLENGE**. Same split as R4.

**Findings** (2 Codex + 1 Gemini non-blocking):
1. (Codex R5#1) Packet missing Round-4 outcome + A8e4 fixes shipped section; `<pending>` commit placeholders for A8e2/A8e3/A8e4 never replaced.
2. (Codex R5#2) Packet § 6 Q6 round-1 question text still says "24 → 30 / 6 new entries" (pre-F5 count); TRACE_MATRIX top-bullet TR-deltas A5 "25→26" / A6 "26→27" still stale (only the per-atom paragraphs were corrected in I2, not the bullet headers).
3. (Gemini R5 non-blocking) `make_pput` arg count text in round-3 retrospective still says "21 positional args"; should be 24.

**Round-5 fixes shipped (`A8e5`, commit `1622017`)** — five fixes:
- **J1** Round-4 outcome + A8e4 fixes shipped section added to packet.
- **J2** `<pending>` placeholders replaced with actual SHAs (`0af47b7` / `3d38ba5` / `8693789`); A8e5 self-reference uses "this commit".
- **J3** Packet § 6 Q6 question text reworded to "24 → 34 / 10 new entries" with full enumerated list.
- **J4** TRACE_MATRIX A5/A6/A7 top-bullet TR-deltas corrected (matches canonical chain in genesis_payload.toml header).
- **J5** Round-3 retrospective `make_pput` arg count: 21 → 24.

---

## Round 6 (2026-04-26) — post-A8e5

**Inputs**:
- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `1622017` (post-A8e5)
- Test baseline: 265 PASS / 29 ignored / 0 failed; 15/15 Python
- Trust Root: 34-entry manifest

**Verdicts**:
- Codex R6: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R6.md`
- Gemini R6: **CHALLENGE / high** — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R6.md`
- Merged: **CHALLENGE**. Convergence regression vs R4/R5 (Gemini went from PASS back to CHALLENGE).

**Findings** (3 Codex + 2 Gemini; ONE real correctness bug):
1. (Codex R6#1) RQ14 closure criterion contradicts itself — packet has `<pending>` placeholders in J2 bullet + RQ14 itself.
2. (**Codex R6#2 — REAL BUG**) `detect_provider` misroutes HuggingFace-style `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B` to `api.deepseek.com` because `"deepseek" in m` wins before the slash check. The official DeepSeek API only serves bare `deepseek-chat` / `deepseek-v4-flash`, not Distill variants — would 404 on first invocation.
3. (Codex R6#3) TRACE_MATRIX A8e3 row still says "21 positional args" while packet was corrected to 24 in J5.
4. (Gemini R6#1) TRACE_MATRIX A6 line leads with "Six anchor sites wired" then immediately self-contradicts with "9 post-A8e total".
5. (Gemini R6#2) Packet § 3 A5/A6 fix attribution: "A8e3 fix H4 corrected" — H4 corrected the genesis_payload.toml comment; the packet § 3 numbers were corrected by I2 (A8e4).

**Round-6 fixes shipped (`A8e6`, commit `dbcf53a`)** — five fixes (1 real correctness + 4 doc):
- **K1** `<pending>` placeholder closure: J2 bullet + RQ14 reworded.
- **K2 (real bug)** `src/drivers/llm_proxy.py::detect_provider` reordered: slash-form is now FIRST routing heuristic. New `test_deepseek_slash_form_routes_to_siliconflow` in `scripts/test_llm_proxy.py` pins the regression. **16/16 Python proxy tests PASS** (was 15).
- **K3** TRACE_MATRIX A8e3 row: "21 positional args" → "was 21 at round-3; F1 added run_id, post-A8e count is 24".
- **K4** TRACE_MATRIX A6 row top-line: "Six anchor sites wired" → "9 anchor sites wired (8 in run_swarm + 1 in run_oneshot)" with original-6 + F4-added-3 breakdown.
- **K5** Packet § 3 A5/A6 fix attribution: "A8e3 fix H4 corrected" → "A8e4 fix I2 corrected".

---

## Round 7 (2026-04-26) — post-A8e7 structural rewrite

**Trigger for A8e7**: rounds 2–6 produced a recurring documentary CHALLENGE pattern that single-point fixes could not close. **Diagnosis**: `A8_EXIT_PACKET` was conflating stable-snapshot artifact + append-only chronology in one document. **Fix (`A8e7`, commit `aaedc9d`)**: split into two docs following the project's existing constitutional pattern (stable spec + delta log; cf. constitution.md + Art. V.3 amendment log; PREREG + PREREG_AMENDMENT; TRACE_MATRIX_v0 + v1 + v2). The packet becomes current-state only; THIS history doc becomes append-only chronology.

**Inputs**:
- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `aaedc9d` (post-A8e7)
- Test baseline: 265 PASS / 29 ignored / 0 failed; 16/16 Python proxy tests
- Trust Root: 35-entry manifest (A8_AUDIT_HISTORY added)

**Verdicts**:
- Codex R7: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R7.md`
- Gemini R7: **CHALLENGE / high** — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R7.md`
- Merged: **CHALLENGE**. Both auditors agree the split pattern is right; the implementation left lineage text in the packet that should have moved to the history doc only.

**Findings** (4 Codex + 2 Gemini):
1. (Codex R7#1) Packet § 2 + § 4 still report 34 Trust Root entries; actual is 35 after A8e7 added `A8_AUDIT_HISTORY_2026-04-26.md`. Stale reference at multiple sites in the packet.
2. (Codex R7#2) This history doc says A8e7 commit is `<this commit's SHA>` and "Round 7 dual audit pending after this commit lands" — but A8e7 is already in Trust Root as landed. Same placeholder-staleness pattern earlier rounds tried to eliminate.
3. (**Codex R7#3 + Gemini R7#1 — convergent**) Packet § 1 + § 3 still contain historical lineage text: "(post-A8e F6 + A8e2 G2)", "(added by A8e F4)", "(+ A8e fix F4)", "A8e6 fix K2", "(chain position 33 via A8e)". The A8e7 split's intent was that the packet describes WHAT IS (current state) without explaining HOW (round-N derivation); HOW belongs only in this history doc.
4. (**Codex R7#4 — substantive logic finding**) PREREG_AMENDMENT § 2 vs § 8 internal contradiction:
   - § 2 (b) claims "the conditions in § 3 ensure calibration runs *before* Phase E, so the loose substitution never reaches the artifact-acceptance moment"
   - § 3 lists 5 PRE-REQUISITES for calibration to run AT ALL (not guarantees of calibration completing before Phase E)
   - § 8 says "if those conditions never met, Phase E proceeds with the operationally-permitted ceiling substitution"
   The amendment claims both "calibration must run before Phase E" AND "ceiling-substitution is acceptable at Phase E without calibration". Cannot be both true.
5. (Gemini R7#2) Audit runner scripts don't append `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` to source files — Q1.c (verify fail-closed) cannot be verified from the packet alone.

**Round-7 fixes shipped (`A8e8`, commit `857872e`)** — five fixes:
- **M1** (Codex R7#1): Packet TR count bumped 34 → 35 wherever cited.
- **M2** (Codex R7#2): This history doc's A8e7 entry now stamps the actual SHA `aaedc9d` and round-7 verdicts (above) instead of "pending". Round-7 history sealed.
- **M3** (Codex R7#3 + Gemini R7#1): Packet rewritten to remove ALL historical lineage text. The packet describes the post-A8e6 state directly — e.g. "9 wired FC-trace anchor sites" with no breakdown of which fix added which. The HOW lives ONLY in this history doc.
- **M4** (Codex R7#4): PREREG_AMENDMENT § 2 + § 3 + § 8 reconciled: removed the false claim that § 3 "ensures calibration before Phase E"; § 8 now reads as the operative rule (Phase E proceeds with the substitution if § 3 conditions haven't completed). Clean single semantics: substitution is permitted throughout Phase B → Phase E; calibration UPGRADES the bar IF and WHEN the § 3 conditions all complete. Re-hashed in Trust Root.
- **M5** (Gemini R7#2): Audit runner scripts now append `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` so Q1.c is verifiable from the packet bundle alone.

---

## Round 8 (2026-04-26) — post-A8e8

**Inputs**:
- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `857872e` (post-A8e8)
- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 16/16 PASS (Python)
- Trust Root: 35-entry manifest

**Verdicts**:
- Codex R8: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R8.md`
- Gemini R8: **PASS / high** → PROCEED — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R8.md`
- Merged: **CHALLENGE**. Conservative merge wins per memory; substantive Codex finding (PREREG § 8 stale claim parallel to M4-fixed § 2) requires closure.

**Findings** (3 Codex; Gemini found no defects):
1. (**Codex R8#1 — substantive**) PREREG_AMENDMENT § 8's audit-requirements text still contains the stale claim "Gate H is Phase E and § 3 conditions ensure calibration runs first" — round-7 M4 fixed § 2(b)'s identical false claim but missed the parallel text in § 8. Logical contradiction with the post-M4 § 2 + § 3 framing.
2. (Codex R8#2) This history doc's round-7 entry left an unresolved placeholder `<aaedc9d-successor>` for the A8e8 commit SHA; cumulative table row 7 still says "pending pending pending" despite round-7 verdicts being recorded above.
3. (Codex R8#3) Audit runner scripts default `A8_AUDIT_ROUND=R2` (oldest fallback) and emit pre-A8e header metadata ("261 PASS / 30-entry manifest") — re-running the runners regenerates stale audit artifacts.

**Round-8 fixes shipped (`A8e9`, this commit)** — three fixes:
- **N1** (Codex R8#1): PREREG_AMENDMENT § 8 reworded to remove the residual "ensure calibration runs first" claim. Now consistent with § 2's least-strict-admissible framing across all three sections.
- **N2** (Codex R8#2): Round-7 entry stamped with actual A8e8 SHA `857872e`; cumulative table row 7 sealed with the actual round-7 verdicts (CHALLENGE/CHALLENGE) and finding count (4 Codex + 2 Gemini, including 1 substantive PREREG logic finding); round-8 row added with this round's verdicts.
- **N3** (Codex R8#3): Runner script header metadata refreshed to current state (265 PASS / 35-entry manifest); pre-A8e values removed.

---

## Round 9 (2026-04-26) — post-A8e9

**Inputs**:
- Packet: `A8_EXIT_PACKET_2026-04-26.md` @ commit `6f327b0` (post-A8e9)
- Test baseline: 265 PASS / 29 ignored / 0 failed (Rust); 16/16 PASS (Python)
- Trust Root: 35-entry manifest

**Verdicts**:
- Codex R9: **CHALLENGE / high** — `handover/audits/CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R9.md`
- Gemini R9: **PASS / high** → PROCEED — `handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_R9.md`. *Second consecutive Gemini PASS with full Q1-Q5 spot-check verification + zero new findings.*
- Merged: **CHALLENGE**. Conservative merge.

**Findings** (2 Codex; Gemini 0):
1. (**Codex R9#1 — false-closure**) A8e9 fix N3 claimed "runner default A8_AUDIT_ROUND updated" but the source still defaulted to `R2`. The actual N3 implementation only refreshed header metadata, not the default. Re-running either runner without env still targeted `_R2`, silently overwriting the round-2 transcript.
2. (Codex R9#2) Packet § 6 says closures are recorded for "round-1..round-7" but history now contains round 8 (and round 9 about to land). Documentary drift class.

**Round-9 fixes shipped (`A8e10`, this commit)** — two fixes:
- **O1** (Codex R9#1): Both runner scripts now make `A8_AUDIT_ROUND` env var REQUIRED. No silent default; fail fast with usage message. Additionally, both runners refuse to overwrite an existing transcript at the resolved output path (round-N transcripts are append-only governance artifacts per C-075). Verified: invoking either without the env var prints the usage message and exits 2; invoking with `A8_AUDIT_ROUND=R2` (where the R2 transcript already exists) prints the overwrite refusal and exits 2.
- **O2** (Codex R9#2): Packet § 6 pointer reworded from "round-1..round-7" to "all prior rounds — see chronological round-N entries in history doc for current count". No longer ages.

---

## Cumulative metrics

| Round | Codex | Gemini | Merged | New findings | Real-bug findings | API cost (~$) |
|---|---|---|---|---|---|---|
| 1 | CHALLENGE | **VETO** | VETO | 9 | 5 | ~5 |
| 2 | CHALLENGE | CHALLENGE | CHALLENGE | 5 | 0 | ~5 |
| 3 | CHALLENGE | CHALLENGE | CHALLENGE | 5 (incl. 1 non-blocking) | 1 (H6 fail-closed) | ~5 |
| 4 | CHALLENGE | PASS | CHALLENGE | 4 (incl. 1 non-blocking) | 0 | ~5 |
| 5 | CHALLENGE | PASS | CHALLENGE | 3 (incl. 1 non-blocking) | 0 | ~5 |
| 6 | CHALLENGE | CHALLENGE | CHALLENGE | 5 | **1 (K2 routing)** | ~5 |
| 7 | CHALLENGE | CHALLENGE | CHALLENGE | 6 | **1 (M4 PREREG § 2 logic)** | ~5 |
| 8 | CHALLENGE | **PASS** | CHALLENGE | 3 | **1 (N1 PREREG § 8 logic)** | ~5 |
| 9 | CHALLENGE | **PASS** | CHALLENGE | 2 | 0 (false-closure caught — N3 was incomplete; no new substantive bugs) | ~5 |
| 10 | pending | pending | pending | — | — | ~5 |

Cumulative cost so far ~$45 (8 rounds × ~$5–7); within memory `feedback_dual_audit` Phase A reservation. **Real-bug yield: 9 substantive findings caught + closed across 8 rounds** (5 routing/correctness in R1, 1 fail-closed gate in R3, 1 routing collision in R6, 1 PREREG § 2 logic in R7, 1 PREREG § 8 logic in R8 = 9 real bugs discovered + fixed pre-Phase B). The recurring documentary CHALLENGE class persisted longer than expected because each round's fix touched documentation in ways that left adjacent staleness; the A8e7 structural rewrite addressed the root cause (category error) but its implementation needed two more cycles (A8e8 + A8e9) to fully complete the lineage strip + cross-section consistency.
