# CODEX T+S Re-Review — Plan v3.1 / Amendment v1 Follow-Up

Date: 2026-04-27  
Reviewer: Codex co-executor  
Scope: Turing + Satoshi review of Claude's proposed post-VETO changes.

I read the required code and documents directly. Claude's cited line ranges for the required code reads are materially accurate: `TuringBus` is at `src/bus.rs:40-51`; `append_internal` is `src/bus.rs:179-333`; the hard-coded token literal is exactly `completion_tokens: 0` at `src/bus.rs:268`; `snapshot()` returns `UniverseSnapshot` at `src/bus.rs:542-582`; `Kernel` fields and methods are at `src/kernel.rs:19-32`, `src/kernel.rs:63-126`, and `src/kernel.rs:156-206`; the monetary market code uses `f64` at `src/prediction_market.rs:21-27`, `src/prediction_market.rs:48-67`, and `src/prediction_market.rs:87-109`.

## § A — Per-item verdicts on each T+S proposal

### D-VETO-1=D spec-first — CHALLENGE

Spec-first is the right direction, but Claude's version is under-specified. The current `bus.rs` and `kernel.rs` code is exactly the kind of state machine that should not be split by instinct: `TuringBus` owns kernel, ledger, tools, config, clock, tx count, generation, graveyard, and WAL in one struct (`src/bus.rs:40-51`), while `append_internal` combines forbidden-pattern checks, payload-size checks, tool hooks, invest-only routing, node construction, kernel append, WAL write, market creation, optional founder grants, post-hooks, ledger events, and counters in one transition (`src/bus.rs:179-333`). `Kernel` also mixes topology, markets, bounty market, settlement, and tickers (`src/kernel.rs:19-32`, `src/kernel.rs:63-126`, `src/kernel.rs:156-206`). A pre-implementation transition relation is not bureaucracy here; it is the only sane way to avoid two independently wrong refactors.

The challenge is that "write a 1-page mathematical state-transition spec" is just normal design-doc discipline unless it has a binding form. Turing's viewpoint demands an explicit transition function; Satoshi's viewpoint demands consensus-critical behavior that two implementers can independently reproduce. Minimum acceptable binding: a short markdown spec with typed state/transaction schemas, deterministic pseudocode, named invariants, and conformance tests generated from that spec. A TLA+/PlusCal model is useful for ordering and replay invariants; Lean/Coq is overkill for the whole bus unless a small safety lemma is singled out. Do not let "spec-first" enter CO P1 as a slogan.

Brief D1-D6 note: this directly affects D4 and the CO1.1.4/CO1.1.5 refactor atoms. It also affects D6 audit cadence because a spec artifact should be audited before branch A/B implementation begins.

### D-VETO-3=D hyper-minimal genesis — CHALLENGE

The proposal is directionally cleaner than the prior "rich genesis" schema, but the exact 5-line version is not bootstrapped enough. Current `genesis_payload.toml` has `[pput_accounting_0]` with fields `schema_version`, `progress_definition`, `cost_definition`, `time_definition`, `verified_predicate`, `heldout_sealed_hash`, `source_pool_sha256`, `baseline_regression_rate`, `baseline_regression_jsonl_sha256`, `k_max`, and `n_max` (`genesis_payload.toml:100-111`). It then has `[trust_root]` as file-path-to-SHA entries such as `"src/main.rs"`, `"Cargo.lock"`, `"src/kernel.rs"`, `"src/bus.rs"`, `"constitution.md"`, and the six recent whitepaper/plan/protocol/amendment files (`genesis_payload.toml:113-167`). It does not currently contain `sudo_policy` or `allowed_meta_update_rules`. Claude is right that the present file is not already a clean rich genesis schema.

But the whitepaper's L0 text says the Constitution Root contains `constitution_hash`, `human_signature`, `sudo_policy`, `allowed_meta_update_rules`, and `physical_or_hardware_attestation` (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md:257-269`), and Boot § 11.1 says the genesis block contains `constitution_hash`, `human_signature`, `initial_predicate_registry_root`, `initial_tool_registry_root`, `initial_state_root`, `initial_budget_state`, `on_init_coin_supply`, `boot_time`, and `boot_attestation` (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md:780-795`). A 5-line genesis that includes only `amendment_predicate_id` but not an immutable predicate-registry root or predicate code hash is circular: it tells us the name of the amendment predicate without anchoring what code that name resolves to. I do not think Art. V.1 requires separation-of-powers prose to be duplicated inside genesis; `constitution_hash` can anchor that. I do think exact hyper-minimal genesis must add either `initial_predicate_registry_root` or `amendment_predicate_hash`. If Claude insists on exactly the five listed fields, this becomes a VETO.

Art. V.1 operative text does not say "genesis must express separation of powers"; it says the system meta-layer must implement the separation:

> `constitution.md:692-702`: "为了让系统安全地实现自我进化，InitAI 不能是一个单一独裁的黑盒。它内部必须实现严格的"三权分立"机制。系统演化的本质是：机制 / 突变 / 选择。这恰恰对应元架构层中的三个角色及其永恒博弈."
>
> `constitution.md:704-715`: "宪法（Constitution）——唯一的基准真相... 人类不再规定'系统应该怎么做'，而是规定'最顶层的目标与价值观'... [2026-04-25 架构师补充] sudo 权限的精确范围：人类 sudo 权限仅且只作用于 `constitution.md` 本身..."
>
> `constitution.md:719-736`: "ArchitectAI（架构师 AI）——提出者... 当系统在运行中发现现有白盒存在缺陷时... ArchitectAI 会主动分析系统日志... [2026-04-25 架构师补充] ArchitectAI 拥有架构升级的 commit 权限..."
>
> `constitution.md:740-765`: "Veto-AI（违宪否决 AI）——验证者... ArchitectAI 提出的任何架构变更，都不能直接上线。必须经过 Veto-AI 的冷酷审查... 它唯一的工作是... 否决违宪提案... 输出域 = `{PASS, VETO}`..."

Brief D1-D6 note: this affects D2 because the Constitution Art. 0.5 pointer cannot substitute for a bootstrapped predicate registry. It also affects D4 because a future meta-amendment path needs a non-circular root.

### D-VETO-4=D permanently abandon runtime MetaTape — VETO

Permanent abandonment over-extends the Satoshi analogy and conflicts with the actual TuringOS documents. Satoshi-style systems do not put every invalid or proposed rule change on-chain, but Bitcoin's conservative governance is not a proof that TuringOS should delete runtime meta-transition semantics forever. The architecture whitepaper § 12 says "TuringOS 不能永远受限于初始人类 spec" and that the system must extract white-box knowledge from failures and upgrade tools, predicates, and processes (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md:816-827`). It defines `meta_tx` with `parent_architecture_root`, proposed predicate/tool patches, log evidence, reversibility plan, constitution check, judge signature, and human signature if required (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md:870-887`). That is not compatible with "ArchitectAI/JudgeAI forever offline" without a user-authored strategic rewrite.

The narrow v4 timeline point is different. § 17 says exactly: "Phase 3: MetaTape — ArchitectAI runtime + Meta transitions" and then "turingosv4 scope: Phase 1 + Phase 2 + Phase 3 prep. Phase 4-5 are post-v4" (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md:1013-1024`). This does not require runtime ArchitectAI inside v4; it requires Phase 3 prep. So Claude is supported if the proposal is "defer runtime MetaTape out of v4 and define prep artifacts only." Claude is contradicted if the proposal is "permanently abandon runtime MetaTape and amend § 17 to remove it." The economic chapter does not force runtime self-modification in the reward formula: `reward_i = Finalize(Escrow(task) × Accept(tx_i) × Attribution(tx_i, ContributionDAG) × Survival(challenge_window) × Utility(post_acceptance_metrics) × Constitution(Q_t))` (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:125-137`). But it does list ArchitectAI and JudgeAI as economic roles (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:91-100`) and maps them to architecture § 12 (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:139-149`). Economic § 19-21 can operate with offline meta-review, but the architecture cannot honestly claim Go Meta while permanently deleting runtime meta-transitions.

Brief D1-D6 note: original D4=B "defer to v4.1" remains acceptable; new D4=D "permanently abandon runtime MetaTape" is not. D5 full RSP does not rescue this because RSP is settlement machinery, not the meta-transition semantics itself.

### B-1 PGP-signed git tag ratification — PASS

Replace chat-based "I approve" with a signed git object. This is the cleanest Satoshi-flavoured governance change in the set: the approval becomes a verifiable signature over a precise commit or tree, not an unverifiable natural-language memory. The recent Trust Root mutation history needs that discipline because the audit ledger records the CO P0 patches and direct TR mutation as doc-only operations (`handover/audits/AUDIT_LEDGER.md:31-35`), and Amendment v1 itself admits `genesis_payload.toml` was directly edited and recoverable by revert if audit VETOed (`handover/architect-insights/CO_P0_AMENDMENT_v1_2026-04-26.md:46-68`).

Implementation should be precise: tag the exact commit being ratified, verify the tag locally, and write tag name, target commit, signer fingerprint, verification command, and verdict into `AUDIT_LEDGER.md`. If the user's environment cannot support PGP, SSH-signed git tags are an acceptable fallback only if the public key fingerprint and verification output are recorded. The core requirement is cryptographic ratification over a content-addressed object, not the brand name of the signing tool.

Brief D1-D6 note: this should be a new governance gate before CO P1, independent of D1-D6. It especially protects D2/D3/D4 document mutations and future Trust Root edits.

### D-VETO-6 retry-metadata addition — CHALLENGE

The user's pre-decision not to log rejected proposal payloads on tape is defensible under the Satoshi half of T+S: invalid transactions do not belong in the canonical block history. It is also aligned with the whitepaper's shielding principle that failed records may enter the lower ledger but must not automatically enter all Agent contexts (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md:505-515`). But the current constitution is stricter in the opposite direction: Art. 0.2 says failure branches "必须以 `kind=AgentProposal, verified=false, reject_class=...` Node 形态进入 tape" and explicitly calls the current `bus.graveyard` design an anti-pattern (`constitution.md:60-66`). So accepting "no rejected proposals on tape" requires either a constitutional amendment or a very explicit reinterpretation: no raw rejected payloads, but bounded, system-generated failure metadata must be on tape.

Claude's proposed `retry_count + prior_attempt_failures: Vec<FailureClass>` is close, but "self-reports" is the wrong trust boundary. Current code stores rejection classes in a sidecar `graveyard: HashMap<String, Vec<String>>` (`src/bus.rs:48`) through `record_rejection()` (`src/bus.rs:439-450`) and derives top-k class broadcasts from that sidecar (`src/bus.rs:491-539`). Cost accounting already knows failed parses, vetoed appends, rejected OMEGA claims, and step rejects must count toward total run cost (`experiments/minif2f_v4/src/cost_aggregator.rs:8-15`, `experiments/minif2f_v4/src/cost_aggregator.rs:62-70`). The acceptable version is: the white-box transition runner stamps bounded failure counters/classes onto the next accepted `work_tx`, and emits a terminal summary tx if the run has no accepted work_tx. No agent-authored payload contents, no raw error strings, no private predicate leakage, and conformance tests must prove `derive_l6_from_tape(tape)` equals the runtime sidecar.

Brief D1-D6 note: this is not original D6 audit cadence; it is a new D-VETO-6 technical addition. Original D6=A full external audit cadence still looks correct under `TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md:81-96` and `handover/architect-insights/CO_P0_AMENDMENT_v1_2026-04-26.md:16-21`, but the retry metadata schema needs design before implementation.

### Brief notes on original D1-D6

D1 PREREG / PPUT-CCL fate: Keep CHALLENGE. Amendment v1 records D1=C MVP-pivot as provisional, not user-approved (`handover/architect-insights/CO_P0_AMENDMENT_v1_2026-04-26.md:10-23`). The retry-metadata decision changes PPUT reconstructibility, so D1 cannot be treated as settled until the L6/error/cost tape schema is chosen.

D2 Constitution Art. 0.5: PASS for "pointer + six axioms" only if it explicitly points to the authoritative whitepapers and preserves the non-circular bootstrap requirements. Amendment v1 says D2=B is provisional (`handover/architect-insights/CO_P0_AMENDMENT_v1_2026-04-26.md:16-18`); it should not be used to smuggle a weakened genesis root.

D3 TFR v1 disposition: PASS. Deprecate but preserve is the low-risk historical option, and Amendment v1 records D3=A provisional (`handover/architect-insights/CO_P0_AMENDMENT_v1_2026-04-26.md:18`). Nothing in the T+S re-review changes that.

D4 CO P3 MetaTape in v4: PASS only for "defer runtime MetaTape beyond v4 while doing Phase 3 prep"; VETO for "permanently abandon runtime MetaTape." The original amendment was D4=B defer (`handover/architect-insights/CO_P0_AMENDMENT_v1_2026-04-26.md:19`), while § 17 explicitly keeps Phase 3 as "MetaTape — ArchitectAI runtime + Meta transitions" (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md:1015-1024`).

D5 RSP depth: PASS on full RSP as a planning direction, with a code-level blocker. The economic chapter makes RSP-1 modules explicit (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:77-89`) and full invariants interlock (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:56-75`). However, current market code uses `f64`: `yes_reserve: f64`, `no_reserve: f64`, `k: f64`, `lp_total: f64` (`src/prediction_market.rs:21-27`), `create(... lp_coins: f64)` (`src/prediction_market.rs:48-67`), and `buy_yes(... coins_in: f64)` (`src/prediction_market.rs:87-109`). RSP settlement invariants should not be built on float equality.

D6 External audit cadence: PASS for full cadence. The protocol's conservative rule says "VETO > CHALLENGE > PASS" and bars merging with active VETO absent user sudo (`handover/architect-insights/TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md:81-96`). Recent history already showed why: Codex's prior audit found Plan VETO / Amendment VETO while Gemini was more lenient (`handover/audits/AUDIT_LEDGER.md:28-35`).

## § B — Where Claude's T+S reasoning does NOT hold up

First, the Satoshi analogy does not justify permanent deletion of runtime MetaTape. It justifies keeping invalid/rejected raw proposals out of canonical shared context and keeping consensus-critical state minimal. TuringOS § 12, however, explicitly says the system must learn from its own failures and upgrade tools/predicates/processes (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md:816-827`), and § 12.2 defines meta-upgrades as state transitions through `meta_tx` (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md:870-887`). § 17 then names Phase 3 as "MetaTape — ArchitectAI runtime + Meta transitions" (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md:1015-1019`). Claude is allowed to say "not v4 runtime"; Claude is not supported in saying "forever offline" without asking the user to rewrite a core architectural goal.

Second, "all governance can move into L1 Predicate Registry" is not self-authenticating. Whitepaper L1 does define predicate records with `predicate_id`, `version`, `code_hash`, schemas, visibility, owner, and test suite hash (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md:273-286`). That supports moving policy mechanics into the registry. It does not support a genesis that contains only an `amendment_predicate_id`; an ID is not a hash, not a root, and not a proof of the code that will execute. The current `genesis_payload.toml` trust root is file-hash based (`genesis_payload.toml:113-167`), so the Satoshi-style minimal root must still anchor content-addressed code, not merely name it.

Third, the Turing analogy does not convert a vague design doc into a formal method. The current transition has non-deterministic and side-effectful parts: nodes stamp wall-clock seconds (`src/bus.rs:264-268`), optional economy branches depend on `TAPE_ECONOMY_V2` and `FOUNDER_GRANT_GAMMA` environment variables (`src/bus.rs:298-307`, `src/bus.rs:345-360`), and `snapshot()` rebuilds market views from mutable kernel state (`src/bus.rs:542-582`). A real transition spec must decide which of these inputs are part of `Q_t`, part of `tx_i`, or illegal hidden side effects.

Fourth, "retry metadata reconstructs L6" is false if the metadata is agent self-report or only appears on accepted work. Current L6 signal candidates include "boolean pass/fail history" and "typical error clusters" (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md:405-418`). Current runtime stores that information in `bus.graveyard`, not tape (`src/bus.rs:48`, `src/bus.rs:439-539`). If a run ends without acceptance, there is no accepted `work_tx` to carry the retry summary. The right Satoshi move is not raw rejected proposals on tape; it is bounded, reactor-generated, content-free accounting commitments on tape.

Fifth, the economic final formula does not presuppose runtime self-modification. The formula is generic settlement over accepted tx, attribution, survival, utility, and Constitution(Q_t) (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:125-137`). So Claude would be wrong if it claims RSP § 21 forces runtime MetaTape in v4. But the economic chapter does list ArchitectAI and JudgeAI roles (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:91-100`), and maps that economic section back to architecture § 12 Go Meta (`handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:139-149`). The formula permits deferral; it does not justify deletion.

## § C — Your own T+S-flavoured perspective where it differs from Claude's

The Turing half says: define the machine. For CO P1 that means `Q_t`, `tx_i`, hidden inputs, allowed side effects, rejection summaries, and replay must have a typed transition relation before refactoring. The current code has a Node field `completion_tokens: u32` (`src/ledger.rs:18-25`) but production appends hard-code `completion_tokens: 0` (`src/bus.rs:259-269`), so the tape already fails as a complete paper record. A transition spec should force every "derived view" claim to prove reconstruction from tape.

The Satoshi half says: keep consensus history minimal, content-addressed, and independently verifiable. That supports B-1 signed tags. It supports not putting raw rejected proposal payloads on canonical tape. It supports a smaller genesis root. It does not support unanchored predicate names, self-reported failure stats, or deleting the meta-upgrade path just because Bitcoin governance is conservative.

My preferred synthesis differs from Claude's in three places. For D-VETO-1, I want a concrete spec artifact, not just "spec-first." For D-VETO-3, I want a minimal root-of-roots: constitution hash, creator signature, signed time, schema version, and an immutable predicate registry root or amendment predicate hash. For D-VETO-4, I would defer runtime MetaTape out of v4 but preserve Phase 3 prep and the long-term runtime path as a first-class target.

## § D — Concrete implementation recommendations

Can land immediately:

- B-1 signed git tag ratification. Add a ledger row recording tag name, target commit, signer fingerprint, and `git verify-tag` or equivalent verification output. The target should include the current TR correction commit `ee55aef` only if the user intentionally ratifies that mutation.
- Keep D3=A deprecate-but-preserve. No further design needed.
- Keep D6=A full audit cadence. The protocol already has the conservative merge rule (`handover/architect-insights/TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md:81-96`) and the post-fix generator/evaluator separation (`handover/architect-insights/TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md:168-195`).

Needs more design before code:

- D-VETO-1 spec-first: define one file, probably `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md`, with typed `QState`, `WorkTx`, `RejectedAttemptSummary`, `MetaTx`, `TransitionResult`, deterministic pseudocode, and invariant list. Bind it with tests before `src/bus.rs` / `src/kernel.rs` split work.
- D-VETO-3 genesis: do not use exact five fields unless `amendment_predicate_id` is replaced by or paired with an immutable hash/root. Decide whether "genesis" means only L0 Constitution Root or the full Boot genesis block from § 11.1; the current documents use both concepts.
- D-VETO-6 retry metadata: define `FailureClass` as a finite enum, generated by the white-box predicate runner; include counts and maybe first/last tx indices, but no raw payloads or raw error strings. Add terminal failure-summary tx for no-accept runs.
- D5 RSP: choose fixed-point/integer money before EscrowVault or SettlementEngine. Current `f64` market code is acceptable for experiments, not for conservation tests (`src/prediction_market.rs:21-27`, `src/prediction_market.rs:48-67`, `src/prediction_market.rs:87-109`).

Should be rejected outright:

- Permanent abandonment of runtime MetaTape. Replace it with: "Runtime MetaTape is out of v4; v4 must deliver Phase 3 prep artifacts and preserve a v4.1+ path."
- Exact 5-line genesis with only an amendment predicate ID and no immutable registry root/hash.
- Any retry/failure accounting where the agent self-reports its own retry count as the source of truth.

## § E — Shortest path to CO P1 entry IF the user accepts your full set of recommendations

1. User records five decisions explicitly: D-VETO-1=spec-bound CHALLENGE accepted; D-VETO-3=minimal-root-with-hash accepted; D-VETO-4=permanent-abandon rejected, v4 defer accepted; B-1 signed tag accepted; D-VETO-6=bounded system-generated retry metadata accepted.

2. Claude drafts a doc-only Plan v3.2 patch: add a `CO1.SPEC.0` state-transition spec gate before CO1.1.4/CO1.1.5; amend D4 wording to "defer runtime MetaTape, preserve Phase 3"; amend genesis atom CO1.0 to include predicate root/hash; amend CO1.7/CO1.9 for retry metadata and terminal failure summaries. Codex reviews that doc patch before code.

3. User ratifies the current Trust Root state with a signed git tag, or refuses and requests a revert/amendment first. The `AUDIT_LEDGER.md` row must include target commit, tag, fingerprint, and verification result.

4. Claude writes `STATE_TRANSITION_SPEC_v1_2026-04-27.md`. Codex independently reviews it against `src/bus.rs:179-333`, `src/kernel.rs:63-126`, `src/kernel.rs:156-206`, `genesis_payload.toml:100-167`, Constitution Art. 0.2 and Art. V.1, and whitepaper § 5/§ 11/§ 12/§ 17. Gemini performs cross-section review. Active CHALLENGE/VETO blocks CO P1.

5. Once the spec passes, execute the first CO P1 technical atom as the gix substrate spike already required by Plan v3.1 (`handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:78-90`). This validates whether the Satoshi-style content-addressed substrate is real before higher layers depend on it.

6. Implement CO1.0 Constitution Root / Genesis using the amended minimal-root schema. Gate it with a replay test proving the amendment predicate resolves to the anchored code hash/root and that Art. V separation-of-powers is reachable through `constitution_hash`.

7. Only after steps 4-6 pass, start the bus/kernel split. Use STEP_B with Claude and Codex independent branches against the same spec. The losing branch or a fresh auditor reviews the winner per the post-fix protocol (`handover/architect-insights/TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md:168-195`).

8. Add retry metadata as part of L4/L6, not as an agent-facing prompt convention. Gate with `derive_l6_from_tape(tape) == runtime_sidecar_snapshot` and a no-accepted-work terminal-summary test.

OVERALL VERDICT: D-VETO-1=CHALLENGE; D-VETO-3=CHALLENGE; D-VETO-4=VETO; B-1=PASS; D-VETO-6-retry=CHALLENGE
