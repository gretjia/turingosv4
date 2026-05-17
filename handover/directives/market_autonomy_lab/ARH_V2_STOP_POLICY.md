# ARH-v2 Stop Policy

This policy exists because ship-mode hard stops are too coarse for autonomous
research. It does not weaken the constitution; it moves the stop boundary to
the edge of a preauthorized research envelope.

## Modes

```text
Mode A — Ship Mode
  Class-4 per atom must stop.
  Trust Root rehash requires explicit ship ratification.
  This remains the rule for main merge and formal claims.

Mode B — Constitutional Research Mode
  MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2 applies.
  Allowed Trust-Root-pinned files may be touched.
  allowed Trust Root rehash may proceed automatically.
  Results are research-only until later audit/ratification.

Mode C — Unsafe Red-Track Mode
  Requires TURINGOS_UNSAFE_RESEARCH=1.
  Evidence root must be separate.
  Reports must say NOT SHIP EVIDENCE / NOT E2 / NOT AUTONOMY CLAIM.
```

## Stop Levels

### Level 0 — Continue

```text
change is inside RESEARCH_ENVELOPE_V2
tests are fixable
Trust Root rehash is for an allowed touched file
no forbidden mechanism appears
evidence is not contaminated
```

Action: continue.

### Level 1 — Soft Checkpoint

Triggers:

```text
targeted test failure
constitution gate failure that is understood and in-envelope fixable
audit_tape FAIL from missing evidence
unknown schema fail-closed
no E2 candidate
all abstain
PolicyTrader abstains
hard10/hard20/hard36 no trade
market digest low signal
LLM parse failure below threshold
clean-negative mechanism report
```

Action: write `CHECKPOINT_REPORT.md`, name the next hypothesis, continue.

Clean-negative is not completion.
No E2 candidate is not completion.

### Level 2 — Ratification Checkpoint

Triggers:

```text
touch allowed Trust-Root-pinned evaluator/runtime/dashboard file
update genesis_payload.toml for allowed Trust Root rehash
add allowed additive runtime module
modify allowed dashboard/evaluator/reporting surface
```

Action:

```text
if file is in RESEARCH_ENVELOPE_V2:
  perform research rehash
  rerun Trust Root verification
  continue if Trust Root passes
else:
  write STOP_PROOF.md and stop
```

### Level 3 — Constitutional Hard Stop

Triggers:

```text
unlisted restricted surface is required
constitution.md or flowchart hashes must change
typed tx schema/discriminant must change
sequencer admission must change
canonical signing payload must change
CAS ObjectType schema must change
kernel/bus/wallet authority must change
forced trade is required in constitutional track
forced/scripted/PolicyTrader action would be counted as E2
price would affect Lean truth or predicate acceptance
ghost liquidity is required
f64/f32 money/probability is required in market paths
off-tape WAL would become source of truth
raw prompt/completion/CoT/log would be broadcast
Trust Root fails after allowed rehash
evidence contamination cannot be isolated
resource envelope is exhausted
```

Action: write `STOP_PROOF.md` before stopping.

## Anti-Premature-Stop Mandate

The following must not stop the lab in Constitutional Research Mode:

```text
no E2 candidate
clean-negative
all abstain
NoTradeReason dominates
PolicyTrader abstains
CHALLENGE audit when fixable inside envelope
allowed Trust Root rehash required
market digest weak
prompt parse failure
hard10 solves too fast
too few review windows
too few EVDecisionTrace records
```

These outcomes must feed the next hypothesis:

```text
H1 Broadcast deficiency
H2 EV execution deficiency
H3 Market parameter deficiency
H4 Risk framing deficiency
H5 Trader view deficiency
H6 Timing/window deficiency
```
