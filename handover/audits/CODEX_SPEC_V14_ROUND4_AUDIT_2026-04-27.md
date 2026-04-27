## Q1.1-Q1.4 closure

Q1.1: CLOSED. v1.4 header states `TaskMarketPublishTx` is NEW/deferred, not retired; § 5.3 classifies `TaskMarketPublish` as "NEW v1 transition (deferred to CO P2.1)" and the retirement grep list excludes it. No v1.4 regression found on already-CLOSED Q1.3 task expiry (§ 3.6) or Q1.4 implicit agent init (§ 3.6.5).

## Q2 new issues

Q2.4: CLOSED. § 5.2.5 defines `ChallengeWindow::is_open(now)` as the half-open window `[opens_at, opens_at + duration_ticks)`. § 3.2 now calls `window.is_open(tx.timestamp_logical)`; § 3.4 now calls `w.is_open(q.q_t.current_round)`; § 4 binds the invariant to both paths. I found no NEW issue introduced by the v1.4 patch set versus the round-3 zero-NEW baseline.

## Q3 cross-spec

No cross-spec regression found. State § 1.6 keeps runtime `MetaTx` deferred to v4.1, matching META_TX_SCHEMA § 1 and § 4. State § 1.5 and § 3.7 use runtime `system_signature`, matching SYSTEM_KEYPAIR_SECURITY § 1, § 3.3, § 3.4, and § 5. Genesis anchoring remains compatible: GENESIS_MINIMAL_WITH_ANCHOR § 3.3 anchors the amendment predicate and § 3.4 anchors the initial predicate registry root referenced by QState.

## Q4 STEP_B readiness

Q5: CLOSED as a spec-freeze issue. § 2.5 freezes the canonical serialization rule: bincode v2, big-endian fixed ints, BTreeMap lexicographic keys, UTF-8 length-prefix strings, explicit Option and enum encodings. NEW-5: CLOSED; § 2.5 explicitly defers the full fixture corpus, differential fuzz seed, and complete QState/SignalBundle/TransitionError runner ABI to CO1.1.4-pre1 and CO1.7, so this is tracked downstream rather than left ambiguous. Q6: CLOSED. § 5.2.1 assigns logical_t through sequencer `next_logical_t()`, and § 5.2.6 makes the atomic assignment the canonical tie-break for concurrent submitters.

STEP_B readiness: YES, contingent on implementing the § 2.5 fixture/ABI follow-through at the named atoms.

## Q5 holistic verdict

PASS.

## Q6 recommendation

GO for CO P1 STEP_B / CO1.1.4 and CO1.1.5. Start the shared canonical fixtures before branch divergence, per § 2.5, but this is an implementation gate rather than another spec patch.
