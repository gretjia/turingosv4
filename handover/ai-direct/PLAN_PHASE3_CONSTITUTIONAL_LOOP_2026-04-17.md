# Phase 3: Constitutional Loop Realignment

**Date**: 2026-04-17
**Trigger**: F-2026-04-17-03 — constitutional topology audit found fundamental design violation

## The Violation

Constitution's main loop (Art. IV mermaid, lines 376-378):
```
Q0 ==> rtool ==> input ==> AI ==> output ==> p
p ==>|1| wtool ==> Q1    (verified → write)
p ==>|0| Q0              (rejected → no write)
```

Current code:
```
AI → output("append") → bus.append() writes to tape → LATER auto-probe verifies
AI → output("complete") → oracle.verify() → if pass, write PPUT result
```

**The order is inverted.** Constitution: verify THEN write. Code: write THEN verify (for append).

## The Constitutional Design (reconstructed)

Every agent output, regardless of action type, flows through ∏p BEFORE touching Q:

1. Agent outputs a tactic step (e.g., `have h := by ring`)
2. ∏p checks:
   - Bus forbidden patterns (existing, works)
   - **Lean incremental type-check**: does this tactic step type-check given the current goal state accumulated from prior verified steps?
3. If ∏p = 1 → wtool writes the tactic to tape (it's a verified partial result)
4. If ∏p = 0 → reject, error broadcast (Art. II.1)
5. When accumulated verified steps constitute a complete proof → OMEGA auto-triggers

## Why This Matters

- **No "append vs complete" distinction needed** — every write is verified
- **No force-append gate needed** — natural flow: agents produce tactics, verified ones accumulate
- **No auto-probe needed** — oracle runs on every write attempt (but incrementally, not full-proof each time)
- **Information efficiency maximized** — only verified tactics enter tape, no stuttering/repetition
- **Tape becomes a DAG of verified tactic steps**, not a list of unverified full-proof attempts

## Technical Requirement: Lean Incremental Tactic Checking

Can Lean 4 verify a single tactic step in isolation (given a goal state)?

**Yes**: `lean --stdin` can accept a partial proof with `sorry` holes. If the tactic step advances the goal state (even if remaining goals have `sorry`), it type-checks. The oracle can test:
```lean
theorem X ... := by
  <accumulated verified tactics>
  <new tactic under test>
  sorry  -- remaining goals
```
If this compiles without error on the NEW tactic line → step is valid.
If error on the new tactic → reject.

This is more expensive than no-verify-append but MUCH cheaper than full-proof-verify because:
- `sorry` short-circuits remaining proof search
- Only the new tactic needs checking, not the full chain
- Oracle cache (hash of accumulated+new) makes repeated checks O(1)

## Implementation Plan

### Step 1: Incremental Oracle
Add `verify_tactic_step(accumulated: &[String], new_step: &str) -> Result<bool, String>` to Lean4Oracle.
Uses `sorry`-padded proof structure. Cache by hash(accumulated + new_step).

### Step 2: Unified Action Model
Remove "append" vs "complete" distinction in evaluator.rs.
Agent outputs `{"tool":"step","payload":"<tactic>"}`.
Every step goes through incremental oracle → verified → write to tape.
When oracle detects "no remaining goals" → OMEGA.

### Step 3: Remove Force-Append Gate
No longer needed — every write is verified. Agent can submit full proof as single step OR build incrementally. The oracle handles both.

### Step 4: Constitutional Loop Enforcement
Ensure: output → ∏p → wtool order is correct in code.
Bus.append only called AFTER oracle passes.

## Compatibility

- Control (old evaluator): unchanged (direct-complete, depth=1)
- Treatment (new evaluator): incremental verified tape, depth=N
- A/B on N=20 as before

## Expected Outcome

- Tape depth = number of VERIFIED tactic steps (not unverified full-proof attempts)
- Each tape node is guaranteed valid → no stuttering
- Agents can build proofs incrementally (hard problems) or single-shot (easy problems)
- Market prices become meaningful (verified steps have real value)
- North Star: n↑ → more diverse verified steps → higher probability of complete chain → super-linear
