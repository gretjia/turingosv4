# Codex Whitepaper v2 Audit

## Q1-Q10 verdicts (with §/file:line citations)

### Q1: PASS
v2 restores the Anti-Oreo three-layer shape and does not subordinate bottom-white tools to top-white predicates. The target explicitly says the bottom tool layer was weakened before and is now "三层而非两层, bottom tools 与 top predicates 等权" (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:17-18`). §3 separately enumerates Top White, Middle Black, and Bottom White layers (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:133-161`), and §3.3 states that without bottom white, top predicates are only slogans, while without top white, bottom tools are only executors (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:190-206`).

This is consistent with constitution.md's flowchart, where predicates are top management and read/write tools are separate bottom tools (`constitution.md:486-508`), and with the later Anti-Oreo diagram where top, agents, and tools are distinct (`constitution.md:846-852`).

### Q2: PASS
v2 maintains ChainTape-as-implementation discipline. It states that ChainTape is not blockchain, blockchain is only one implementation in low-trust multi-party settings, and Anti-Oreo remains the first principle (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:46-49`). 公理 5 says blockchain is not the body/core, only one tape implementation (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:86-88`). §13 repeats that blockchain is a deployment form of ChainTape, not TuringOS's soul (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:904-907`), and §18 says blockchain can enhance but cannot define TuringOS (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:1311-1319`).

This fits constitution.md Art. 0.3, which reserves future hash-chain / Merkle / git-style integrity mechanisms rather than making blockchain primary (`constitution.md:99-110`). I did not find text in v2 that reverts to chain-as-primary.

### Q3: VETO
Hard conflict: v2 expands human sudo beyond the constitutional boundary. constitution.md says human sudo authority "仅且只" applies to `constitution.md` itself, while other Trust Root payload changes are ArchitectAI space after Veto-AI review (`constitution.md:715-736`), and Art. V.3 says constitution amendment is the unique trigger for explicit human sudo authorization (`constitution.md:804-806`). v2 instead puts broad safety-domain items such as external transfers, irreversible state changes, key access, and production deployment under the safety domain (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:575-587`), says `sudo = highly costly + human gate` (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:750-757`), and in §16.4 prescribes `human sudo for irreversible class` (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:1206-1218`). That reads as a non-constitution human sudo gate and contradicts Art. V.1.1/V.1.2.

Secondary consistency break: constitution.md renamed JudgeAI to Veto-AI to narrow the role to constitutional veto only (`constitution.md:740-765`) and records the rename in the amendment log (`constitution.md:812`). v2 still uses `Judge` / `JudgeAI` in Boot, Go Meta, meta-transition signature, and roadmap text (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:799-807`, `handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:853-861`, `handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:871-880`, `handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:1263-1271`). The semantics are mostly preserved, but the naming drift is constitutionally stale and must be patched.

Potential Art. 0.2 risk: v2 maps Rubber partly to `key destruction` (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:121-127`) and repeats key destruction as an irreversible-write mitigation (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:1210-1218`). constitution.md requires all signals to be reconstructible from tape (`constitution.md:52-66`). This is not a separate VETO if key destruction is confined to off-chain encrypted payloads with committed metadata, but v2 does not state that caveat.

### Q4: CHALLENGE
The extension can be faithful, but v2 does not make the preserving interpretation explicit enough. constitution.md Art. 0.4 defines Q_t as the version-controlled triple `⟨q_t, HEAD_t, tape_t⟩` and gives rtool/wtool signatures over that triple (`constitution.md:114-122`). v2 first repeats the base triple (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:210-224`), but then says ChainTape extends Q_t into an 8-field tuple that no longer names `tape_t` directly (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:226-239`).

The OBS note supplies the needed interpretation: `q_t`, `HEAD_t`, and tape-derived state are preserved, and the added roots are cryptographic commitments to existing state rather than new state (`handover/alignment/OBS_WHITEPAPER_V2_QT_FIVE_ROOT_EXTENSION_2026-04-27.md:32-42`). That avoids VETO, but v2 itself should inline this preservation clause so future readers do not treat the 8-field tuple as replacing the constitutional triple.

### Q5: PASS
The dual-domain rejection rule is consistent with boolean predicate semantics if implemented as risk-tier policy over predicate selection/thresholds, not as a non-boolean predicate result. v2 §7.1 preserves `p: X -> {0,1}` and explicit `0 = 拒绝`, `1 = 接受` (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:525-557`). §7.2 then changes the default rejection posture by domain: creative work leans toward not false-killing candidates, while security work fails closed (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:559-589`).

constitution.md Art. I.1 requires predicates to output only 0/1 (`constitution.md:176-215`), and Art. I.1.1 already permits engineering PCP-style asymmetric verification when perfect predicates are unavailable (`constitution.md:219-254`). The OBS note correctly frames §7.2 as a policy layer above Art. I.1, while keeping the predicate result boolean (`handover/alignment/OBS_WHITEPAPER_V2_DUAL_DOMAIN_2026-04-27.md:26-29`).

### Q6: PASS
The Public / Private / Commit-Reveal trinity is consistent with Goodhart shielding. constitution.md Art. III.4 says black-box agents must not be able to fully inspect scoring logic and optimize against it (`constitution.md:413-423`). v2 Layer 1 makes visibility a predicate-registry field and separates public, private, and commit-reveal predicates (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:286-313`); §9.4 repeats that private predicates and commit-reveal tests defend against benchmark leakage and long-term overfitting (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:710-723`).

The implementation note says `Visibility::{Public, Private, CommitReveal}` already exists, while commit-reveal protocol support is not yet runtime-load-bearing (`handover/alignment/OBS_WHITEPAPER_V2_PREDICATE_VISIBILITY_TRINITY_2026-04-27.md:31-50`). That is an implementation gap, not a constitutional contradiction.

### Q7: PASS
v2 avoids math overcommitment. It uses the phrase "工程化 PCP 谓词" and describes the engineering doctrine as correct candidates should avoid false killing and wrong candidates should be intercepted with high probability (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:474-497`). constitution.md itself already uses PCP language for imperfect real-world predicates and defines the intended engineering asymmetry in completeness/soundness terms (`constitution.md:219-254`).

The v2 wording is weaker than a formal PCP theorem claim and does not assert complexity-theoretic machinery.

### Q8: CHALLENGE
Layer 4 correctly points at the deferred transition-ledger atom: v2 defines Transition Ledger as an append-only state-transition ledger with tx fields (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:359-385`), and Appendix A maps §5.1 Layer 4 to CO1.7 transition_ledger (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:1327-1333`).

However, the layering is not clean as written. §5.1 is titled "六层结构" (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:270`), but it enumerates Layer 0 through Layer 6, which is seven numbered layers (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:272-421`). There is also overlap between Layer 5 materialized-state indices (`agent_reputation_index`, `price_signal_index`) and Layer 6 signal indices (`price signals`, `reputation counters`) (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:386-419`). Because constitution.md requires all signals to be reconstructible from tape (`constitution.md:52-66`), v2 should state whether L6 is derived from L4/L5 or an independent layer.

### Q9: CHALLENGE
The roadmap can work as a narrative overlay, but the plan-of-record boundary is not strong enough inside v2. v2 §17 presents a five-phase implementation route from GitTape through Public Settlement (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:1237-1296`), while v2's status block says v2 wins over derivative docs and is the highest Wave 6+ alignment reference below constitution.md (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:3-7`). Appendix A says §17 is a narrative overlay over Plan v3.2 and does not rewrite it (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:1337-1338`), and the tactical alignment note says the same (`handover/alignment/WHITEPAPER_v2_TACTICAL_ALIGNMENT_2026-04-27.md:40-46`).

Plan v3.2, however, is the atomized operational plan with updated ~175 atoms (`handover/architect-insights/CO_MEGA_PLAN_v3.2_2026-04-27.md:171-185`) and concrete P3-prep deliverables (`handover/architect-insights/CO_MEGA_PLAN_v3.2_2026-04-27.md:232-259`). v2.1 should add an explicit crosswalk: §17 controls narrative ordering, while CO_MEGA_PLAN_v3.2 remains the atom-level plan of record unless separately patched.

### Q10: CHALLENGE
The tactical-alignment status is conceptually workable but brittle. v2 says constitution.md remains the only highest alignment and v2 does not trigger flowchart, TRACE_MATRIX, or conformance-test rebuilds (`handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md:3-7`). The tactical note states v2 is above derivative documents but below constitution.md (`handover/alignment/WHITEPAPER_v2_TACTICAL_ALIGNMENT_2026-04-27.md:11-18`) and explains why the user chose tactical alignment rather than full constitutional merge (`handover/alignment/WHITEPAPER_v2_TACTICAL_ALIGNMENT_2026-04-27.md:21-32`).

The sustainability problem is that constitution.md defines the constitution as the unique Ground Truth and tightly scopes human sudo/amendment mechanics (`constitution.md:704-715`, `constitution.md:804-806`). A second "highest校准 mirror" without TRACE_MATRIX/conformance rebuild will fail under future Wave decisions unless every v2-only doctrine is clearly classified as interpretive, deferred, or OBS-only. The current Q3 sudo conflict is evidence that the dual-authority structure already allowed a constitutional boundary to blur.

## Holistic verdict
VETO

## Must-fix (if any)
- §16.4 / §10.2 / §7.2: remove or narrow broad `human sudo` language. Human sudo must remain limited to `constitution.md`; irreversible non-constitution actions may require deterministic policy predicates, Veto-AI review, signatures, challenge windows, or external legal controls, but not constitutional sudo.
- §11.2 / §12.1 / §12.2 / §17: replace `JudgeAI`, `Judge`, `judge_signature`, and related roadmap wording with `Veto-AI` / `veto_signature`, or explicitly mark `JudgeAI` as a historical alias only.
- §4: state inline that `Q_t = ⟨q_t, HEAD_t, tape_t⟩` remains the conceptual schema, and that `state_root_t`, `tape_view_t`, `ledger_root_t`, `budget_state_t`, `predicate_registry_root_t`, and `tool_registry_root_t` are implementation commitments / derived projections of the tape-backed state.
- §5.1: fix "six-layer" vs Layer 0-6 count. Either call it seven layers, or define Layer 0 Constitution Root as a trust root outside the six ChainTape layers.
- §5.1 Layer 5/6: remove or explain overlap between materialized-state indices and signal indices; specify that L6 is reconstructible from tape/L4/L5.
- §17 / Appendix A: add a roadmap-to-Plan-v3.2 crosswalk and explicitly state that Plan v3.2 remains the atom-level plan of record.
- §2 / §16.4: constrain `key destruction` so it cannot destroy information required by Tape Canonical reconstructibility; it may only apply to off-chain encrypted payload accessibility where tape retains commitments, metadata, and reconstructible public state.

## Recommendation
RETRACT-AND-REWRITE
