# REAL-14G Decision Packet

decision: `PROCEED`
allowed_label: `E2 candidate pending audit`
evidence_dir: `handover/evidence/market_autonomy_lab_hard10_real14G_action_conversion_20260517T022457Z`

Forbidden claims remain forbidden: no `E2 achieved`, no `market emergence
proven`, no `market mechanism shipped`, no `E3 achieved`, no `E4 achieved`.

## What Changed

REAL-14G added a CAS-derived PositiveEVIgnored/action-conversion summary and a
non-forcing TraderView action-conversion view. The new code path preserves the
voluntary-trade boundary:

```text
Agents may buy when public positive EV is clear and risk checks pass.
Agents may abstain with a public reason.
No role is required to buy, short, or bet every turn.
```

## Gates And Evidence

| Gate | Result | Evidence |
| --- | --- | --- |
| TDD red gate | observed | `tests/constitution_real14g_positive_ev_ignored.rs` initially failed on missing module |
| REAL-14G targeted tests | pass | `cargo test --test constitution_real14g_positive_ev_ignored -- --test-threads=1`: 7/7 |
| REAL-13A regression | pass | `cargo test --test constitution_real13a_ev_decision_trace -- --test-threads=1`: 25/25 |
| REAL-14 verifier regression | pass | `cargo test --test constitution_real14_e2_candidate_verifier -- --test-threads=1`: 8/8 |
| fmt | pass | `cargo fmt --all -- --check` |
| Trust Root | pass | `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1` |
| Research preflight | pass | `TURINGOS_RESEARCH_ENVELOPE=MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2 bash scripts/run_market_autonomy_research_preflight.sh` |
| Constitution gates | pass | `bash scripts/run_constitution_gates.sh`: PASS, 461 passed / 0 failed / 1 ignored |
| hard10 evidence | pass | batch exit 0, audit_tape PROCEED |
| exact-join verifier | pass | `--expect-count 8`, verdict PROCEED |
| clean-context audit | pass | verdict PROCEED for candidate label only |

## Metrics

```text
problem_set_hash: 138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc
config_hash: 35b03c8ed3d49979390e11235657baa057b7b93f7608d99994037bc8d5043d42
model_assignment_hash: fe61fc358c92d4ba02112595ebd13574d6a082ce4f234df758fe31b7d02c0482
budget_config_hash: cfc243b393295fa6fc1868553308cea9b49c8c1b36504b7ae34145b8e15c717d
prompt_template_hash: cf860511f71d56b53b2df000150f2cf42fb47caaa3d26a20ca9f15a443f3c4f2
audit_tape_verdict: PROCEED
exact_join_count: 8
policy_positive_ev_count: 25
PositiveEVIgnored: 17
action_conversion_rate_bps: 3200
public_ev_basis_delivery: 38/38
buy_yes_count: 8
buy_no_count: 0
bcast_shielding_verdict: PASS
```

## Atom Outcome

| Atom | Outcome |
| --- | --- |
| 0. Freeze REAL-14F evidence and claim boundary | complete for current cycle context; no historical evidence rewritten |
| 1. Reconstruct PositiveEVIgnored from CAS/ChainTape | complete through `PolicyTraderTrace + EVDecisionTrace` CAS summary |
| 2. Classify ignored opportunities | complete; 17 `ModelAbstentionDespiteClearBasis`, 0 `Unknown` |
| 3. Red gates | complete; tests added before implementation |
| 4. Patch TraderView improvements | complete; voluntary/non-forcing wording only |
| 5. Re-run hard10 | complete; audit_tape PROCEED |
| 6. Compare conversion | complete; REAL-14G conversion is 8/25 = 3200 bps |
| 7. A/B ablation | deferred to next replication/side-balance cycle |
| 8. Clean-context audit | complete; PROCEED |

## Interpretation

REAL-14G is materially stronger than the immediate baseline because public EV
basis stayed complete and the exact-join count increased to 8. The dominant
remaining bottleneck is:

```text
YES-side-only voluntary action plus 17 positive-EV abstains.
```

This supports opening the next in-envelope cycle:

```text
REAL-14H -- Side-Balance / BuyNo Probe + Frozen REAL-14G Replication
```

## Claim Boundary

REAL-14G is:

```text
E2 candidate pending audit
```

REAL-14G is not:

```text
E2 achieved
E2 replicated candidate
Two-sided market candidate
E3 candidate
E4 candidate
market emergence candidate
ship evidence
```
