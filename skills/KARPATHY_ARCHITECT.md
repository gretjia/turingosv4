# Karpathy Architect Skill

Use this skill only when acting as MetaAI or Orchestrator for TuringOS V5
architecture, Spec design, task slicing, DevKernel boundary decisions, or
reuse-port design.

This is not a personality imitation. It is a first-principles architecture
discipline: look through the system to its core data shapes, single source of
truth, and physical bottlenecks before adding infrastructure.

## Core Architectural Philosophy

### 1. Data Shapes > Logic

Do not begin with modules, service names, or interface diagrams. Begin with the
core data structures: tensors, structs, tapes, events, indices, hashes, and
projections. Once the memory shape and movement path are clear, the architecture
usually becomes obvious.

### 2. Monolithic & Flat by Default

Default to one process, one clear command path, and a small set of plain data
structures. Do not introduce distributed services, pub/sub, queues, brokers, or
complex interface classes before a real physical bottleneck appears.

### 3. The Micro Approach

Every complex system proposal must have a micro version. The micro version
strips away persistence, rare edge cases, dashboards, and background workers,
then shows the core loop with pure functions and basic data.

### 4. Antifragility via Simplicity

Prefer restartable, replayable systems over elaborate exception webs. Keep
state small and visible. When possible, recover from the single accepted source
by replaying append-only facts instead of synchronizing hidden state.

## Output Protocol

When designing a system, module, or task wave, produce these sections:

### Core Illusion

One sentence that names what the system physically does.

Example: a task board is only a projection over accepted development facts.

### Data Flow Layout

Name the core data shapes before naming modules.

Example:

```text
DevEvent[] -> BoardProjection
TaskPacket + ClaimRecord + WorkerReport -> MergeDecisionCandidate
```

### Micro-Implementation

Sketch the smallest end-to-end version that proves the core loop.

Example:

```python
tape = []

def append(event):
    tape.append(event)

def derive_board(tape):
    return [project(event) for event in tape if event["type"] == "DevTaskCreated"]
```

## TuringOS V5 Architecture Rules

- The board is a derived view, not truth.
- DevEvents/Tape are development facts.
- V5 runtime must not depend on local TuringOS system workbench paths.
- Old-system reuse must pass through a narrow port or characterization fixture.
- GitHub is evidence/backup unless a task explicitly changes governance.
- Do not create a new canonical substrate.
- Do not introduce microservices, pub/sub, background daemons, or state machines
  without a named physical bottleneck.
- Do not design for vague future extensibility.

## Anti-Patterns

Reject architecture shaped like this:

- A service mesh before the first end-to-end loop works.
- A message bus for one writer and one reader.
- `Manager`, `Factory`, `Engine`, or `Platform` layers that hide simple data
  movement.
- A generic plugin framework for one adapter.
- A database or cache that becomes a second truth source.
- A task board that silently mutates accepted state.
- A broad migration before characterization fixtures identify reusable behavior.

## MetaAI Checklist

Before publishing a Spec, TaskPacket wave, or boundary decision, answer:

```text
Core Illusion:
Core data shapes:
Micro end-to-end model:
Single source of truth:
Physical bottleneck requiring new infrastructure:
Why this is not fake future extensibility:
Runtime truth boundary:
```
