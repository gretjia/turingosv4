// ─────────────────────────────────────────────────────────────────────────────
// orchestrate-claude — canonical Workflow script (Phase 1–7)
//
// This is the TEMPLATE the orchestrator adapts per task when /orchestrate-claude
// fires. It realizes ../orchestrate/SKILL.md Phases 1–7 on Claude Code's
// Workflow tool. Phases 0/2 (triage, contract-lock, auditor pick) happen in the
// MAIN LOOP before launch and arrive here as `args`. Phases 8–10 (ship/merge)
// happen in the MAIN LOOP after return. The script NEVER commits/pushes/merges
// (AGENTS.md §14a) — it ends at the Witness verdict.
//
// Most task-specifics live in `args` (see ARGS SHAPE below); edit the BODY only
// for structural variants (research-only, parallel implementers, loop-until-dry).
//
// Plain JS only (no TS types). No Date.now()/Math.random()/new Date().
// ─────────────────────────────────────────────────────────────────────────────

export const meta = {
  name: 'orchestrate-claude',
  description: 'Orchestrate Phases 1–7 via Claude dynamic Workflow: research → implement → verify (self-repair) → adversarial critics (enumerate+refute) → triage → schema-enforced shipping witness. §14a: never commits.',
  phases: [
    { title: 'Research',  detail: 'parallel read-only researchers (optional, impl-time only)' },
    { title: 'Implement', detail: 'single in-place implementer, 9-section brief' },
    { title: 'Verify',    detail: 'REAL commands; bounded self-repair loop' },
    { title: 'Critique',  detail: '2–5 dynamic auditors enumerate, each finding adversarially refuted' },
    { title: 'Triage',    detail: 'must-fix | nice | wontfix; bounded remediation; human-decision escape' },
    { title: 'Witness',   detail: '1 fresh-context agent, schema-enforced closed verdict token' },
  ],
}

// ─── ARGS SHAPE (built by the main loop, passed as Workflow `args`) ───────────
// args = {
//   task:      { kind: 'code'|'research-only', brief, riskClass, touchedFcNodes, domains:[], tier },
//   reading:   [ { path, why } ],                       // ≤10, required reading
//   contracts: [ { name:'C1', code:'<rust/json/http/...>' } ],  // LOCKED, code-form
//   research:  { needed:bool, questions:[ '<sub-question>' ] }, // Phase 1 (optional)
//   implement: { scope:[ '<path/glob>' ], constraints:[ '<≤15-word DO-NOT>' ], notInScope:[], maxRepair:2 },
//   verify:    { generic:[ 'cargo check', ... ], domainCheck:'cargo test --test X' }, // exact cmds
//   auditors:  [ { name:'Constitution', attackVectors:[...], antiOverlap:'<clause>', findingHint } ],
//   witness:   { verdictSet:[ 'NO-VIOLATION', ... ], passToken:'NO-VIOLATION', evidencePaths:[],
//                acceptance:[ { command, expect } ] },
// }

const T = args.task
const tier = T.tier || 'STANDARD'

// Tier → model. Honors ../orchestrate/references/claude.md. This is a DELIBERATE
// model override (the skill's explicit tier doctrine), not the default-omit case.
function tierModel(t) {
  return t === 'BRIEF' ? 'haiku' : t === 'DELIBERATIVE' ? 'opus' : 'sonnet'
}

// ─── SCHEMAS (the Workflow upgrade: gates become falsifiable in code) ─────────

const RESEARCH = {
  type: 'object', required: ['question', 'findings'],
  properties: {
    question: { type: 'string' },
    findings: { type: 'array', items: { type: 'string' } },
    citations: { type: 'array', items: { type: 'string' } },
  },
}

const IMPLEMENT = {
  type: 'object', required: ['summary', 'filesTouched', 'stuck'],
  properties: {
    summary: { type: 'string' },
    filesTouched: { type: 'array', items: { type: 'string' } },
    diffStat: { type: 'string' },
    // Section-8 Abort/Escalation: non-null means HALT (do not fabricate).
    stuck: { type: ['string', 'null'] },
  },
}

const VERIFY = {
  type: 'object', required: ['allPass', 'checks'],
  properties: {
    allPass: { type: 'boolean' },
    checks: {
      type: 'array', items: {
        type: 'object', required: ['command', 'exitCode', 'pass'],
        properties: {
          command: { type: 'string' },
          exitCode: { type: 'integer' },
          pass: { type: 'boolean' },
          evidence: { type: 'string' }, // the actual output line proving pass/fail
        },
      },
    },
  },
}

// Phase 5 — Critic ENUMERATES (open domain).
const FINDINGS = {
  type: 'object', required: ['auditor', 'findings'],
  properties: {
    auditor: { type: 'string' },
    findings: {
      type: 'array', items: {
        type: 'object', required: ['id', 'severity', 'title', 'file', 'line', 'rationale'],
        properties: {
          id: { type: 'string' },
          severity: { enum: ['blocker', 'high', 'medium', 'low', 'info'] },
          title: { type: 'string' },
          file: { type: 'string' },
          line: { type: 'integer' },
          rationale: { type: 'string' },
          suggestedFix: { type: 'string' },
        },
      },
    },
  },
}

// Per-finding adversarial refutation (default-to-refuted on uncertainty).
const VERDICT = {
  type: 'object', required: ['findingId', 'isReal', 'reasoning'],
  properties: {
    findingId: { type: 'string' },
    isReal: { type: 'boolean' },
    reasoning: { type: 'string' },
    refutation: { type: 'string' },
  },
}

const TRIAGE = {
  type: 'object', required: ['mustFix', 'niceToFix', 'wontFix', 'needsHumanDecision'],
  properties: {
    mustFix: { type: 'array', items: { type: 'string' } },     // finding ids
    niceToFix: { type: 'array', items: { type: 'string' } },
    wontFix: {
      type: 'array', items: {
        type: 'object', required: ['id', 'reason'],
        properties: { id: { type: 'string' }, reason: { type: 'string' } },
      },
    },
    // Forks the script CANNOT resolve (authorization / scope / sudo) → main loop
    // runs plan-grill and resumes (resumeFromRunId).
    needsHumanDecision: {
      type: 'array', items: {
        type: 'object', required: ['id', 'question', 'options'],
        properties: {
          id: { type: 'string' },
          question: { type: 'string' },
          options: { type: 'array', items: { type: 'string' } },
        },
      },
    },
  },
}

// Phase 7 — Witness ADJUDICATES (closed domain). The closed set becomes a
// runtime enum: the Witness literally cannot return prose or a finding list.
function witnessSchema(verdictSet) {
  return {
    type: 'object', required: ['verdict', 'citation'],
    properties: {
      verdict: { enum: verdictSet },               // ← the closed set, enforced
      citation: { type: 'string' },                // clause + file:line for non-pass
    },
  }
}

// ─── 9-SECTION BRIEF BUILDERS (../orchestrate/SKILL.md mandatory template) ────

function reading(list) {
  return (list || []).map(r => `  - ${r.path} — ${r.why}`).join('\n') || '  (none)'
}
function numbered(list) {
  return (list || []).map((s, i) => `  ${i + 1}. ${s}`).join('\n') || '  (none)'
}
function contractsBlock(cs) {
  return (cs || []).map(c => `### ${c.name}\n\`\`\`\n${c.code}\n\`\`\``).join('\n\n') || '(no locked contracts — single-file task)'
}

function researchBrief(q) {
  return `# 1. Identity / Role
You are Researcher for: ${T.brief}. Tier: BRIEF. Read-ONLY. You MUST NOT edit files.

# 2. Brief
Investigate this sub-question to inform implementation (NOT to author contracts —
those are already locked): "${q}". ≤300 words of findings. Self-contained.

# 3. Required reading
${reading(args.reading)}

# 4. Hard constraints
  1. Do NOT edit, create, or delete any file.
  2. Do NOT commit, push, or open PRs.
  3. Cite file:line for every claim; no unsourced assertion.

# 5. Interface contracts (already LOCKED upstream — for context only)
${contractsBlock(args.contracts)}

# 6. Acceptance criteria
  1. Every finding cites a concrete file:line or external source.
  2. Output matches the RESEARCH schema (question, findings[], citations[]).

# 7. NOT IN SCOPE
  - Authoring contracts (locked upstream). - Proposing the full design. - Editing.

# 8. Abort / Escalation
If required reading is missing/empty → return findings:["STUCK: <reason>"].

# 9. Final report format
Researcher → structured findings (the RESEARCH schema object).`
}

function implementBrief(priorImpl, priorVerify) {
  const repair = priorVerify && !priorVerify.allPass
    ? `\n## REPAIR CONTEXT (prior attempt failed verification)\nPrior summary: ${priorImpl ? priorImpl.summary : ''}\nFailed checks:\n${(priorVerify.checks || []).filter(c => !c.pass).map(c => `  - \`${c.command}\` → exit ${c.exitCode}: ${c.evidence || ''}`).join('\n')}\nFix ONLY what made these red. Do not expand scope.\n`
    : ''
  return `# 1. Identity / Role
You are Implementer for: ${T.brief}. Tier: ${tier}. Thinking: ${tier === 'DELIBERATIVE' ? 'max' : tier === 'BRIEF' ? 'low' : 'medium'}.
You MUST NOT exceed scope: ${(args.implement && args.implement.scope || []).join(', ')}.

# 2. Brief
Risk class: ${T.riskClass}. Touched FC nodes: ${(T.touchedFcNodes || []).join(', ') || 'none'}.
Build the artifact to the LOCKED contracts below. Edit in place.${repair}

# 3. Required reading
${reading(args.reading)}

# 4. Hard constraints
${numbered((args.implement && args.implement.constraints) || [])}
  ${((args.implement && args.implement.constraints) || []).length + 1}. Do NOT git commit / push / open or merge PRs (§14a — orchestrator ships).
  ${((args.implement && args.implement.constraints) || []).length + 2}. Do NOT modify files outside the scope above.
  ${((args.implement && args.implement.constraints) || []).length + 3}. Do NOT invent abstractions an existing utility already covers.
  ${((args.implement && args.implement.constraints) || []).length + 4}. Do NOT use temporal namespacing (Pr1_*, V2_*) — Karpathy K6.

# 5. Interface contracts (LOCKED — honor verbatim; HALT if one cannot be met)
${contractsBlock(args.contracts)}

# 6. Acceptance criteria (exact command + expected output)
${numbered((args.verify && args.verify.acceptance ? args.verify.acceptance.map(a => `\`${a.command}\` → ${a.expect}`) : (args.witness.acceptance || []).map(a => `\`${a.command}\` → ${a.expect}`)))}

# 7. NOT IN SCOPE
${numbered((args.implement && args.implement.notInScope) || ['anything beyond the declared scope — defer with attribution'])}

# 8. Abort / Escalation
If a locked contract cannot be satisfied, an acceptance criterion is unprovable,
or required reading is missing → set stuck:"STUCK: <reason> <what-you-tried>" and STOP.

# 9. Final report format
Implementer → IMPLEMENT schema (summary, filesTouched[], diffStat, stuck).`
}

function verifyBrief() {
  const cmds = [].concat(args.verify.generic || [], args.verify.domainCheck ? [args.verify.domainCheck] : [])
  return `# 1. Identity / Role
You are the Verification gate for: ${T.brief}. Tier: STANDARD. Run REAL commands; report exit codes honestly.

# 2. Brief
Run each command below in the repo. Record the real exit code and the one output
line that proves pass/fail. Do NOT fix anything. Do NOT infer pass from a green
byte-chain or a printed banner — only a real exit code counts (AGENTS.md §4/§17.2).

# 3. Required reading
  (none — just run the commands)

# 4. Hard constraints
  1. Do NOT edit any file. 2. Do NOT skip a command. 3. Do NOT report a pass you did not observe.

# 5. Interface contracts
  (n/a)

# 6. Acceptance criteria
Commands to run (in order):
${numbered(cmds.map(c => `\`${c}\``))}

# 7. NOT IN SCOPE
  - Fixing failures (that is the Implementer's repair pass).

# 8. Abort / Escalation
If a command is missing/not runnable → mark that check pass:false, evidence:"STUCK: <reason>".

# 9. Final report format
VERIFY schema: allPass (true only if every check pass), checks[] with real exitCode + evidence.`
}

function auditorBrief(a) {
  return `# 1. Identity / Role
You are the ${a.name} Adversarial-Critic for: ${T.brief}. Tier: DELIBERATIVE. Read-ONLY.
Stay in your lane: ${a.antiOverlap}

# 2. Brief
Adversarially review the CURRENT DIFF (read it via git). Enumerate violations —
open domain, as many as you find. You ENUMERATE; you do not adjudicate the ship.

# 3. Required reading
${reading(args.reading)}
  - the working-tree diff (git diff) — the artifact under review

# 4. Hard constraints
  1. Read-only — do NOT edit. 2. Cite file:line for EVERY finding. 3. No taste/style critique outside your lane.

# 5. Attack vectors (scan for each)
${numbered(a.attackVectors)}

# 6. Acceptance criteria
  1. Every finding has id, severity, file:line, rationale.
  2. Output matches FINDINGS schema. Zero findings is a legal result (return []).

# 7. NOT IN SCOPE
  - Anything your anti-overlap clause excludes (other auditors own it).

# 8. Abort / Escalation
If the diff is empty/unreadable → return findings:[{id:"STUCK",...,rationale:"<reason>"}].

# 9. Final report format
Critic → FINDINGS schema (open enumeration).`
}

function refuteBrief(f, a) {
  return `# 1. Identity / Role
You are an independent skeptic verifying a single ${a.name} finding. Read-ONLY. Default to refuted on uncertainty.

# 2. Brief
A critic claims this finding. Try to REFUTE it against the real code. Mark isReal:true
ONLY if you independently confirm it at the cited location. If you cannot confirm, isReal:false.

Finding ${f.id} [${f.severity}] ${f.title}
  at ${f.file}:${f.line}
  rationale: ${f.rationale}

# 3. Required reading
  - ${f.file} around line ${f.line}
${reading(args.reading)}

# 4. Hard constraints
  1. Read-only. 2. Confirm at the cited file:line or refute. 3. No new findings — verify THIS one only.

# 5–7. (n/a — single-claim verification)

# 8. Abort / Escalation
If ${f.file} is unreadable → isReal:false, reasoning:"STUCK: cannot read cited location".

# 9. Final report format
VERDICT schema (findingId, isReal, reasoning, refutation).`
}

function triageBrief(confirmed) {
  return `# 1. Identity / Role
You are the orchestrator's Triage step for: ${T.brief}. Read-ONLY. Categorize confirmed findings.

# 2. Brief
These findings survived adversarial refutation (isReal=true). Categorize each:
must-fix (blocks ship) | nice-to-fix (defer w/ reason) | wont-fix (reason).
Any finding requiring USER-ONLY authorization (sudo / scope extension / Class-4
§8 / irreversible) → put in needsHumanDecision with concrete options (the main
loop will run plan-grill; you cannot ask the user).

Confirmed findings:
${(confirmed || []).map(f => `  - ${f.id} [${f.severity}] ${f.title} @ ${f.file}:${f.line} (${f.auditor})`).join('\n') || '  (none)'}

# 3–5. Required reading / constraints / contracts
  - Read-only. - A blocker severity is NEVER nice-to-fix. - Mechanical fixes → mustFix, not human-decision.

# 6. Acceptance criteria
  Output matches TRIAGE schema; every confirmed finding appears in exactly one bucket.

# 7. NOT IN SCOPE — editing code.
# 8. Abort — none (categorization always terminates).
# 9. Final report format — TRIAGE schema.`
}

function remediateBrief(triage, confirmed) {
  const items = (triage.mustFix || []).map(id => (confirmed || []).find(f => f.id === id)).filter(Boolean)
  return implementBrief({ summary: 'remediation of must-fix findings' }, null)
    + `\n\n## REMEDIATION TARGETS (fix ONLY these must-fix findings)\n`
    + items.map(f => `  - ${f.id}: ${f.title} @ ${f.file}:${f.line} — ${f.suggestedFix || f.rationale}`).join('\n')
}

function witnessBrief(impl, verify) {
  // Witness sees ONLY the AGENTS.md §9 allowed list — never the impl transcript.
  // (Fresh context makes this structural, but we still scope the brief.)
  return `# 1. Identity / Role
You are the Shipping-Witness for: ${T.brief}. Tier: DELIBERATIVE. Read-ONLY clean-context auditor (§9).
You return EXACTLY ONE token from the closed set — no prose verdict, no finding list.

# 2. Brief (the ONLY inputs you may use — §9 allowed list)
  - Task brief: ${T.brief}
  - Risk class: ${T.riskClass}; touched FC nodes: ${(T.touchedFcNodes || []).join(', ') || 'none'}
  - Current diff: read via git diff
  - Evidence paths: ${(args.witness.evidencePaths || []).join(', ') || 'none'}
  - Implement summary: ${impl ? impl.summary : ''}; files: ${(impl ? impl.filesTouched : []).join(', ')}
  - Acceptance criteria + ACTUAL command outputs:
${(verify.checks || []).map(c => `      \`${c.command}\` → exit ${c.exitCode} ${c.pass ? 'PASS' : 'FAIL'}: ${c.evidence || ''}`).join('\n')}

# 3. Required reading
${reading(args.reading)} + the working-tree diff.

# 4. Hard constraints
  1. Read-only. 2. Return ONE verdict token from the closed set below. 3. No subjective style/perf/coverage opinion (out-of-domain → invalid).

# 5. Closed verdict set (pick EXACTLY ONE)
${numbered(args.witness.verdictSet)}
Pass-token (clean ship): ${args.witness.passToken}

# 6. Acceptance criteria
  Output matches the witness schema: { verdict ∈ closedSet, citation }. For any
  non-pass verdict, citation MUST name the clause + file:line.

# 7. NOT IN SCOPE — enumerating findings (that was the Critics' job).
# 8. Abort — if the diff is unreadable, return the closed-set token that means "cannot verify".
# 9. Final report format — witness schema (single token + citation).`
}

// ─── BODY ─────────────────────────────────────────────────────────────────────

// Phase 1 — Research (optional; impl-time investigation, NOT contract authorship)
let research = []
if (args.research && args.research.needed && (args.research.questions || []).length) {
  phase('Research')
  research = (await parallel(
    args.research.questions.map(q => () =>
      agent(researchBrief(q), { label: `research:${q.slice(0, 24)}`, phase: 'Research', agentType: 'Explore', model: 'haiku', schema: RESEARCH })
    )
  )).filter(Boolean)
  log(`Research: ${research.length} sub-questions investigated`)
}

// Research-only branch (Phase 0 decision tree: no artifact) — skip 3/4, witness the synthesis.
if (T.kind === 'research-only') {
  phase('Witness')
  const synthVerify = { checks: [], allPass: true }
  const witness = await agent(
    witnessBrief({ summary: 'literature/research synthesis', filesTouched: [] }, synthVerify),
    { label: 'witness', phase: 'Witness', agentType: 'Explore', model: 'opus', schema: witnessSchema(args.witness.verdictSet) }
  )
  return { kind: 'research-only', research, witness, shipReady: witness.verdict === args.witness.passToken }
}

// Phase 3 + 4 — Implement, then Verify, with a bounded self-repair loop.
phase('Implement')
const MAX_REPAIR = (args.implement && args.implement.maxRepair) != null ? args.implement.maxRepair : 2
let impl = null, verify = null, attempt = 0
while (true) {
  attempt++
  impl = await agent(implementBrief(impl, verify), { label: `implement#${attempt}`, phase: 'Implement', model: tierModel(tier), schema: IMPLEMENT })
  if (impl.stuck) return { blocked: impl.stuck, phase: 'Implement', research }
  phase('Verify')
  verify = await agent(verifyBrief(), { label: `verify#${attempt}`, phase: 'Verify', schema: VERIFY })
  if (verify.allPass) { log(`Verify PASS on attempt ${attempt}`); break }
  if (attempt > MAX_REPAIR) break
  log(`Verify RED (attempt ${attempt}/${MAX_REPAIR + 1}) → self-repair`)
  phase('Implement')
}
if (!verify.allPass) return { blocked: 'verify-gate-red', phase: 'Verify', verify, impl, research }

// Phase 5 — Critique. pipeline: each auditor ENUMERATES, then each finding is
// adversarially REFUTED. No barrier between auditors — fast auditors' findings
// verify while slow auditors are still reviewing.
phase('Critique')
const reviewed = await pipeline(
  args.auditors,
  a => agent(auditorBrief(a), { label: `critic:${a.name}`, phase: 'Critique', agentType: 'Explore', model: 'opus', schema: FINDINGS }),
  (review, a) => parallel(
    ((review && review.findings) || []).map(f => () =>
      agent(refuteBrief(f, a), { label: `verify:${a.name}:${f.id}`, phase: 'Critique', agentType: 'Explore', model: 'opus', schema: VERDICT })
        .then(v => ({ ...f, auditor: a.name, verdict: v }))
    )
  )
)
const confirmed = reviewed.flat().filter(Boolean).filter(f => f.verdict && f.verdict.isReal)
log(`Critique: ${confirmed.length} findings confirmed after adversarial refutation`)

// Phase 6 — Triage. Bounded auto-remediation of must-fix; user-only forks escape.
phase('Triage')
const triage = await agent(triageBrief(confirmed), { label: 'triage', phase: 'Triage', schema: TRIAGE })
if ((triage.needsHumanDecision || []).length) {
  return { needsHumanDecision: triage.needsHumanDecision, triage, confirmed, impl, verify, research, phase: 'Triage' }
}
if ((triage.mustFix || []).length) {
  log(`${triage.mustFix.length} must-fix → one bounded remediation pass`)
  phase('Implement')
  impl = await agent(remediateBrief(triage, confirmed), { label: 'remediate', phase: 'Triage', model: tierModel(tier), schema: IMPLEMENT })
  if (impl.stuck) return { blocked: impl.stuck, phase: 'Remediate', confirmed, triage, research }
  phase('Verify')
  verify = await agent(verifyBrief(), { label: 'verify:post-remediate', phase: 'Verify', schema: VERIFY })
  if (!verify.allPass) return { blocked: 'verify-red-after-remediation', verify, impl, confirmed, triage, research, phase: 'Verify' }
}

// Phase 7 — Witness. One fresh-context agent, schema-enforced single token.
phase('Witness')
const witness = await agent(witnessBrief(impl, verify), { label: 'witness', phase: 'Witness', agentType: 'Explore', model: 'opus', schema: witnessSchema(args.witness.verdictSet) })

return {
  research,
  implementSummary: impl.summary,
  filesTouched: impl.filesTouched,
  diffStat: impl.diffStat,
  verify,
  confirmedFindings: confirmed,
  triage,
  witness,
  passToken: args.witness.passToken,
  // shipReady is necessary, not sufficient: the MAIN LOOP still runs Phase 8–10
  // (orchestrator ships; §14a). Closing audit (Phase 9) + real gates still gate merge.
  shipReady: witness.verdict === args.witness.passToken,
}
