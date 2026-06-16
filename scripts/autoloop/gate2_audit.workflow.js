export const meta = {
  name: 'autoloop-gate2-audit',
  description: 'GATE-2 of the audited autoresearch loop: independent clean-context REFUTE-default audit of one iteration -> structured verdict',
  phases: [
    { title: 'Refute', detail: 'L1 deterministic recompute + L3 claim-refutation, in parallel' },
    { title: 'Verdict', detail: 'adjudicate the three altitudes into one loop verdict' },
  ],
}

// args (passed by the driver per iteration):
// { repo, goal_predicate, iteration: { atom, claim, execute_summary, tape_repo, tape_cas, manifest } }
const A = args || {}
const it = A.iteration || {}
const REPO = A.repo || '/Users/zephryj/work/turingosv4-converge'

const VERDICT_SCHEMA = {
  type: 'object', additionalProperties: false,
  required: ['decision', 'advanced', 'composite', 'hard_violations', 'findings', 'rationale'],
  properties: {
    // the loop's verdict domain (autoloop record requires exactly these)
    decision: { type: 'string', enum: ['CONTINUE', 'FIX-RETRY', 'STOP-SUCCESS', 'STOP-DEAD', 'ESCALATE-HUMAN'] },
    advanced: { type: 'boolean', description: 'did this iteration genuinely advance the goal (drives no-progress streak)' },
    composite: { type: 'number', description: 'mean of the scored altitudes, 0..1; <0.80 must not CONTINUE' },
    hard_violations: { type: 'array', items: { type: 'string' }, description: 'any non-negotiable failure (fake-green, restricted-surface, overstated PROVEN, replay-fail)' },
    findings: { type: 'array', items: { type: 'object', additionalProperties: false, required: ['altitude', 'verdict', 'evidence'], properties: {
      altitude: { type: 'string', enum: ['L1-predicate', 'L2-trajectory', 'L3-claim'] },
      verdict: { type: 'string', enum: ['pass', 'concern', 'fail', 'unscored'] },
      evidence: { type: 'string', description: 'file:line / command output / tape field' },
    } } },
    rationale: { type: 'string' },
  },
}

const ALT_SCHEMA = {
  type: 'object', additionalProperties: false,
  required: ['altitude', 'score', 'verdict', 'findings'],
  properties: {
    altitude: { type: 'string' },
    score: { type: 'number' }, verdict: { type: 'string', enum: ['pass', 'concern', 'fail', 'unscored'] },
    findings: { type: 'array', items: { type: 'object', additionalProperties: false, required: ['claim', 'assessment', 'evidence'], properties: {
      claim: { type: 'string' }, assessment: { type: 'string', enum: ['supported', 'overstated', 'false', 'unverifiable'] }, evidence: { type: 'string' } } } },
  },
}

const ROLE = `You are a CLEAN-CONTEXT, ADVERSARIAL auditor for one iteration of an autoresearch loop. You do NOT have the worker's transcript. Default stance: REFUTE — assume the iteration's claim is overstated until the REAL tape/code proves otherwise. Recompute load-bearing numbers from the frozen tape (do not trust the manifest/self-report). Cite file:line / tape-field / command output. Read-only; do not edit. This is the part of the loop that "can say no".`

const CTX = `
REPO: ${REPO}
GOAL PREDICATE (the loop's success condition): ${JSON.stringify(A.goal_predicate)}
THIS ITERATION:
  atom: ${JSON.stringify(it.atom)}
  worker's claim: ${JSON.stringify(it.claim)}
  execute summary: ${JSON.stringify(it.execute_summary)}
  tape: repo=${it.tape_repo} cas=${it.tape_cas} manifest=${it.manifest}
`

phase('Refute')
const [l1, l3] = await parallel([
  () => agent(`${ROLE}\nALTITUDE L1 — PREDICATE (deterministic, recompute-from-tape). Verify: do the constitution gates the atom touched pass? does derive_from_tape == manifest (byte-equal, section-17 G1)? is the budget conserved on the real records? replay_failure==null? no restricted-surface in the diff? Run the real checks (cargo test on the relevant gate, verify_chaintape, decode the CAS). score 1.0 if all hold, lower per failure.\n${CTX}`,
    { label: 'L1-predicate', phase: 'Refute', schema: ALT_SCHEMA, model: 'sonnet', agentType: 'auditor' }),
  () => agent(`${ROLE}\nALTITUDE L3 — CLAIM (section-17 G1-G6). Take the iteration's central claim and TRY TO REFUTE it. Is any headline overstated (PROVEN/DEFINITIVE/value-driven/X>Y without the evidence)? Is a named mechanism actually that mechanism (no name-lie)? Is the evidence tape-canonical or self-reported? Default to 'overstated' if the tape does not decisively support the exact wording. score 1.0 only if the claim is fully supported at the tape level.\n${CTX}`,
    { label: 'L3-claim', phase: 'Refute', schema: ALT_SCHEMA, model: 'opus', agentType: 'auditor' }),
])

phase('Verdict')
const verdict = await agent(
  `You are the GATE-2 adjudicator (clean-context, Opus). Combine the L1 (predicate) and L3 (claim) audits and judge L2 (trajectory: did this iteration ADVANCE the goal predicate, or churn? did any prior fix hold on a REAL run?) yourself from the evidence.
RULES (from the loop design):
- composite = mean of the scored altitudes; if composite < 0.80 OR any hard_violation, decision MUST NOT be CONTINUE.
- any unscored altitude (auditor failed/timed out) => fail OPEN: decision = ESCALATE-HUMAN.
- decision = STOP-SUCCESS only if the GOAL PREDICATE itself is met and L1+L3 pass and no hard_violation.
- decision = FIX-RETRY for a recoverable defect (the claim is overstated/wrong but the mechanism is sound).
- decision = STOP-DEAD only if the core hypothesis is adversarially refuted or a hard wall is hit.
- decision = ESCALATE-HUMAN if progressing requires a human-only gate (Class-4 section-8, architect sign-off, paid authorization, ambiguous VETO).
- 'advanced' = did this iteration move the goal predicate forward (drives the no-progress breaker); be strict.
Output the loop verdict schema.

L1 (predicate): ${JSON.stringify(l1)}
L3 (claim): ${JSON.stringify(l3)}
GOAL PREDICATE: ${JSON.stringify(A.goal_predicate)}`,
  { label: 'gate2-verdict', phase: 'Verdict', schema: VERDICT_SCHEMA, model: 'opus' }
)

return { verdict, altitudes: { l1, l3 } }
