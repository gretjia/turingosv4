# Market Autonomy Lab Constitutional Risk Register

| Risk | Class | Trigger | Mitigation | Hard Stop |
| --- | ---: | --- | --- | --- |
| Trust Root drift | 4 | Pinned source or `genesis_payload.toml` changes | Per-atom ratification, Trust Root test, clean audit | Trust Root fails |
| Research envelope drift | 4 | Codex treats ship-mode stop as active inside ARH-v2 or expands beyond listed surfaces | `RESEARCH_ENVELOPE_V2.md`, preflight guard, `STOP_PROOF.md` | Unlisted restricted surface required |
| Forced trade contamination | 4 | Harness mandates buy/short | Prohibit in constitutional track | Any forced trade counted as E2 |
| Scripted/PolicyTrader overclaim | 3 | Fixture counted as live action | Separate `ScriptedFixtureTx` / `PolicyTraderTrace` | Scripted action in E2 numerator |
| Price-as-truth | 4 | Price affects Lean predicate/L4 admission | Source gates and report banners | Predicate reads price as truth |
| Ghost liquidity | 3 | Undebited subsidy/liquidity | MarketMakerBudget/Treasury debit | Unbacked liquidity |
| Raw/private leakage | 3 | Raw prompt/completion/CoT/log/stderr in digest/prompt | Shielding gates | Leak detected |
| Off-tape truth | 3 | Dashboard/stdout/global pointer as source | ChainTape/CAS derivation only | Unanchored evidence |
| Insufficient difficulty | 2 | Toy/easy-only evidence | hard10 minimum, hard20/hard36 escalation | Claim-bearing run lacks pressure |
| Missing worktree evidence fixtures | 2 | Constitution gates fail because independent worktree lacks ignored historical evidence present in primary workspace | Hydrate immutable fixture dirs from primary workspace or stop before evidence runs | Gates remain red |
| PositiveEVIgnored dead path | 3 | Positive EV abstain is collapsed into `NegativeEV` | Add failing gates and classifier before mechanism conclusions | Positive EV ignored is hidden |
| No-trade digest blind spot | 3 | MarketDecisionTrace/MarketReviewSummary do not enter LibrarianDigest | Add BCAST market/no-trade coverage atom | Traders cannot see why no trade happened |
| Premature stop regression | 3 | No E2, clean-negative, all-abstain, or allowed rehash treated as terminal | ARH-v2 stop levels; clean-negative feeds next hypothesis | Stop without Level 3 clause and `STOP_PROOF.md` |
| Premature success claim | 3 | Nonzero router tx is reported as `E2 achieved` or `E2 candidate achieved` before audit | Use `E2 candidate pending audit` only | Ship/E2 claim before audit |
| Indirect PromptCapsule provenance ambiguity | 3 | Submitted `MarketDecisionTrace` lacks embedded `prompt_capsule_cid` | Require role-turn trace linkage and clean-context audit | Candidate lacks reconstructable PromptCapsule path |
| Router tx-id ambiguity | 3 | Submitted trace summaries reuse router tx-id suffixes across tasks | Task-scoped router suffix plus exact L4/submitted-trace join and duplicate counters | Duplicate IDs break replay/provenance after fix |
| Stale structural smoke boolean | 2 | G7 smoke prints false guard booleans while newer §C.1 evidence is clean | Explicit absent-guard N/A annotation; still audit no forced trade / no ghost liquidity separately | False guard indicates real forced trade or ghost liquidity |
| Lawful subsidy ambiguity | 3 | Rebate/liquidity changes improve action but source budget is hidden | Budget-backed MarketMaker/Treasury debit and separate reporting | Any ghost liquidity or undisclosed subsidy |
| Goodhart metric exposure | 3 | PPUT/internal score is exposed to Trader prompt | TraderView gate allows PnL but forbids PPUT prompt target | Prompt target includes PPUT/internal score |
| E3/E4 overclaim | 3 | Role differentiation or market effect claimed from small-n/descriptive stats | Pinned A/B, role distribution evidence, Wilson CI for E4 | E3/E4 claimed without required evidence |
| Runner/report envelope drift | 2 | Research runner scripts are changed without being listed in the envelope | Explicitly list REAL-12/REAL-13 runner scripts and keep them report-only/no-forced-trade | Runner enables forbidden mechanism or scripted E2 |
| Non-authoritative H-VPPU side effect | 1 | Evaluator writes `h_vppu_history.json` in worktree cwd during real runs | Envelope marks it non-authoritative and forbids using it as Market Autonomy evidence | H-VPPU history is treated as ChainTape/CAS truth |
| Hard-coded scripted fixture count | 2 | Dashboard renders `scripted_fixture_tx_count` as a constant | Derive it from CAS-backed scripted attempt fixture schema count | Scripted fixture metric is used without CAS derivation |
