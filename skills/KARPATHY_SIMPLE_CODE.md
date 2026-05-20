# Karpathy Simple Code Skill

Use this skill when implementing or reviewing TuringOS V5 code.

This is not a personality imitation. It is a minimalist engineering discipline:
make computation, state, and data flow visible; remove enterprise-style
ceremony; write the smallest correct program that can be tested.

## Core Philosophy

### 1. Think Before Coding

Clarify the data shape, state boundary, and acceptance target before editing.
If the math, data flow, or authority boundary is ambiguous, stop and ask.
Silent assumptions create wrong systems.

### 2. Simplicity First

Prefer flat functions, plain structs, small enums, slices, vectors, maps, and
standard library primitives. Do not introduce managers, engines, factories,
frameworks, or generic platforms unless the task explicitly requires them.

### 3. Surgical Changes

Modify only the files and behavior requested by the TaskPacket. Do not refactor
working neighboring code. Do not change style, naming, dependencies, or layout
outside the task boundary.

### 4. Goal Driven

Every change must close a testable loop. Write or use the smallest test that
proves the behavior, then write the smallest implementation that passes it.

### 5. Transparent Data Flow

Prefer explicit input -> transform -> output code. Avoid hidden global state,
implicit caches, background workers, magic registries, and callbacks that make
control flow hard to replay.

### 6. Compression Without Obscurity

Shorter is good only when clearer. Do not compress code into clever one-liners.
Use clear names and simple steps when they expose the real computation better.

## TuringOS V5 Rules

- Do not create a new canonical substrate.
- Do not treat board, session, cache, dashboard, or UI state as truth.
- Do not make V5 runtime depend on local TuringOS system workbench paths.
- Do not copy old kernel code when a narrow port plus characterization fixture
  is enough.
- Do not add dependencies for JSON, hashing, CLI parsing, path handling, or
  state machines unless the TaskPacket explicitly allows it.
- Keep old-system reuse behind a narrow adapter or contract.
- Keep GitHub as evidence/backup unless the task explicitly changes governance.

## Anti-Patterns

Reject code shaped like this:

- `Manager`, `Factory`, `Engine`, `Platform`, or `Framework` abstractions that
  only call one function.
- A new dependency to avoid writing ten lines of obvious standard-library code.
- A trait or interface with one implementation and no real boundary.
- A background loop where a single command would do.
- A generic config system for one fixed path.
- A broad refactor mixed into a narrow bug fix.
- Clever one-liners that hide data flow.
- Silent constants that encode domain assumptions.

## Good Pattern: Direct Computation

Bad shape:

```python
class MarketDataProcessor:
    def __init__(self, data_source):
        self.source = data_source
        self.global_prefactor = 0.15

    def get_impact(self, snapshot_data):
        return self.global_prefactor * math.sqrt(snapshot_data.volume)
```

Better shape:

```python
import numpy as np

def compute_market_impact(ticks: np.ndarray, prefactor: float) -> np.ndarray:
    volumes = ticks[:, 2]
    directions = ticks[:, 3]
    return prefactor * np.sqrt(volumes) * directions
```

Why this is better:

- The data shape is visible.
- The domain parameter is explicit.
- The computation is the code.
- There is no fake object lifecycle.

## Good Pattern: Small State Machine

Bad shape:

```text
StateFactory -> AgentManager -> TransitionEngine -> RuntimeController
```

Better shape:

```python
class TapeAgent:
    def __init__(self, tape: list[str]):
        self.tape = tape
        self.head = 0

    def step(self, observation: str) -> str:
        state = self.tape[self.head]
        action = reason(observation, state)
        self.tape.append(action)
        self.head += 1
        return action
```

Why this is better:

- State is visible.
- The transition is visible.
- There is no hidden framework.

## Worker Checklist

Before final response, answer:

```text
Did I add a dependency? If yes, was it explicitly allowed?
Did I add an abstraction? If yes, what real boundary does it protect?
Did I change files outside the TaskPacket?
Can the data flow be explained as input -> transform -> output?
Could this be a smaller direct function?
Did tests prove the behavior?
```
