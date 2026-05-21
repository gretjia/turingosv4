# FC Real-World Boundary

Class 0 fact record. This note records current boundary facts only; it is not a
wire-format specification, command interface, implementation plan, predicate
catalog, or release plan.

## FC boundary facts

TuringOS currently exposes the real-world boundary as a constitutional
interface between proposal-producing agents, scattered predicate/admission
checks, and tape/CAS evidence. FC1 keeps Agent delta inside the runtime loop,
but only accepted predicate outcomes may advance Q. FC2 boot/replay remains
reconstructable from constitution, ChainTape, CAS, and workspace pointers. FC3
can propose or veto through documented handover artifacts, but it does not
directly mutate the runtime substrate.

Current observed byte boundaries:

| Boundary | Current physical shape |
|---|---|
| rtool/read view | `UniverseSnapshot` and prompt bytes are built from in-process state (`src/bus.rs`, `src/sdk/prompt.rs`). |
| Agent delta | LLM call remains network-capable via the SiliconFlow client path. |
| predicates/admission | Checks are scattered across bus append checks, sequencer admission, product-level pure functions, and registry metadata. |
| wtool/write | Accepted/rejected evidence is written through existing ChainTape/CAS paths; this document does not add a new substrate. |

## Art. 0.4

Current fact: Art. 0.4 still records A/B/C as a pending architectural decision,
while the codebase already contains a C-hybrid / B-pragmatic path through
git-backed ChainTape/CAS. This mismatch is **ratification debt**, not a new
implementation task in this document.

This note does not amend `constitution.md`; it records the debt so the §8
directive can answer it explicitly.

## Hermetic

Current fact: phase 0 is process hygiene only. It may require explicit command
receipts, env allowlists, static checks, and replay gates. It does not claim
OS-level hermeticity, DenyAll network, or no-network enforcement.

## Predicate locality

Current fact: future predicate locality should default to subprocess isolation
at the predicate runner boundary. This is a locality/process boundary, not a
sequencer admission rule.

## LLM topology

Current fact: Agent delta remains inside FC1 for now. LLM output can be a
proposal or evidence object, never an accept predicate.

## Section 7 questions

1. What is the real-world boundary? The boundary is where external proposals,
   subprocess tools, and network-capable clients become tape-visible evidence
   or fail to affect Q.
2. What is the hermetic claim? The current claim is process hygiene, command
   evidence, and replay discipline; no OS-level no-network guarantee is made.
3. Where should predicates run? The future default is a subprocess predicate
   boundary with explicit receipts, outside sequencer admission semantics.
4. What role does an LLM play? The LLM proposes and may supply evidence; it does
   not decide predicate acceptance.

## P7.z note

This document does not evaluate Product-CAK / P7.z success or failure. It only
records that P7.z evidence now depends on honest process and evidence language:
static no-LLM checks are not runtime network denial, iframe policy strings are
not OS sandboxing, and writer contract fields are not facts unless paired with
observable before/after evidence.

## Out of scope / forbidden

The following v2.0 terms are explicitly out of scope here and must not be
smuggled into the boundary model: ProblemCapsule, CandidatePatchBundle,
OracleSignature, CooldownLock, tos predicate, atom_id, schema, CLI, roadmap.

This record does not authorize source changes, constitution changes, new
transaction types, or any restricted surface edits.
