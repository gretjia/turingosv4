# A3 — Real-world HTML task difficulty taxonomy

| Field | Value |
|-------|-------|
| Date | 2026-05-21 evening |
| Phase | A (research) — dispatch 3 of 3 |
| Agent | sonnet, general-purpose |
| Word count | ~2000 |
| Caveat | **Predictions ran HIGH for DeepSeek-v4-flash.** X3 Wordle was predicted <5% one-shot success; pilot run achieved one-shot success. Recalibrate before reusing. |

## TL;DR

Single-page HTML tasks split cleanly into five difficulty bands based on how many non-trivial behaviors must coordinate correctly. Tiers E and M stress-test the Meta→Worker spec expansion pipeline. Tiers H and X are the primary tape-relay validators — H exposes interaction bugs that only appear at runtime, X is designed so the first attempt almost always produces something that parses but fails a mechanical check. Tier R covers the non-game intent path and tests whether the system handles prose-heavy, structure-first deliverables.

## Tier E — Easy (one-shot ~90% success)

### Task E1: Countdown Timer
- **Prompt:** "Make me a countdown timer that starts at 60 seconds, counts down, and shows DONE when it hits zero."
- **Failure modes:** Timer doesn't stop at zero; display doesn't update (missing `setInterval`); button resets to wrong value.
- **Mechanical success:** `grep -c "setInterval\|setTimeout" index.html` ≥ 1 AND `grep -c "DONE\|done" index.html` ≥ 1 AND HTML parses.
- **Multi-attempt sig:** Rarely needed. If attempt 1 fails it's truncation mid-script; attempt 2 just needs full output.

### Task E2: Hex Color Picker Preview
- **Prompt:** "Build a color picker where I type a hex code and the background of the page instantly changes to that color."
- **Failure modes:** `oninput` wired to wrong element; hex validation regex too strict (blocks valid colors); background change targets wrong element.
- **Mechanical success:** `grep -c "oninput\|addEventListener" index.html` ≥ 1 AND `grep -c "backgroundColor\|background-color" index.html` ≥ 1.
- **Multi-attempt sig:** Rarely needed; failures are usually silent (nothing happens), traceable to missing event binding.

### Task E3: Word Character Counter
- **Prompt:** "Create a text box where I can type and it shows me the live word count and character count below the box."
- **Failure modes:** Count doesn't update live; word count splits on wrong delimiter (counts empty words); counts show 0 and never change.
- **Mechanical success:** `grep -c "split\|length" index.html` ≥ 2 AND `grep -c "oninput\|keyup" index.html` ≥ 1.
- **Multi-attempt sig:** Not needed at this tier.

## Tier M — Medium (one-shot ~50% success)

### Task M1: Sortable To-Do List
- **Prompt:** "Build a to-do list app where I can add tasks, mark them complete with a checkbox, and delete them. Completed tasks should move to the bottom automatically."
- **Failure modes:** Sort-on-complete not implemented (items stay in place); delete removes wrong item (index drift after DOM mutation); completed styling applied but order never changes.
- **Mechanical success:** `grep -c "sort\|insertBefore\|appendChild" index.html` ≥ 1 AND `grep -c "delete\|remove\|splice" index.html` ≥ 1 AND `grep -c "checked\|complete" index.html` ≥ 2.
- **Multi-attempt sig:** Attempt 1 often produces add+delete+checkbox but skips the reorder. Attempt 2 needs the spec to call out "re-sort the list on every checkbox change."

### Task M2: Simple Flashcard Quiz
- **Prompt:** "Make a flashcard app with 5 built-in vocab words. Show the word, let me guess the meaning, then reveal the answer and track how many I got right."
- **Failure modes:** Score increments on every click regardless of answer; "reveal" and "next card" share one button causing state confusion; deck exhaustion not handled (loops forever or crashes at index 5).
- **Mechanical success:** `grep -c "score\|correct" index.html` ≥ 2 AND `grep -c "length\|cards\|deck" index.html` ≥ 2 AND deck has exactly 5 entries.
- **Multi-attempt sig:** Attempt 1 usually scores incorrectly. Attempt 2 benefits from explicit spec: "increment score only when the user's input matches the answer string before reveal."

### Task M3: BMI Calculator with Category Label
- **Prompt:** "Build a BMI calculator. I enter height in cm and weight in kg, it calculates BMI and tells me if I'm underweight, normal, overweight, or obese."
- **Failure modes:** Wrong formula (weight/height instead of weight/height²); boundary conditions wrong (BMI 25.0 labelled as normal instead of overweight); output field never populated.
- **Mechanical success:** `grep -c "18.5\|25\|30" index.html` ≥ 3 AND `grep -c "underweight\|Underweight" index.html` ≥ 1 AND `grep -c "obese\|Obese" index.html` ≥ 1.
- **Multi-attempt sig:** Attempt 1 usually gets the formula wrong or misses a boundary. Attempt 2 needs the exact WHO boundary values in the spec.

## Tier H — Hard (one-shot ~15–25% success)

### Task H1: Snake Game
- **Prompt:** "Build a Snake game that runs in the browser. The snake grows when it eats the food, speeds up slightly each time, and the game ends if it hits the wall or itself."
- **Failure modes:** Self-collision check fires on neck segment (immediate death after turn); food spawns on snake body; speed increase applied to wrong variable (interval not reset); canvas not cleared each frame.
- **Mechanical success:** `grep -c "clearRect\|fillRect" index.html` ≥ 2 AND `grep -c "collision\|game.?[Oo]ver\|gameOver" index.html` ≥ 1 AND `grep -c "setInterval\|requestAnimationFrame" index.html` ≥ 1 AND `grep -c "ArrowUp\|ArrowDown\|keydown" index.html` ≥ 1.
- **Multi-attempt sig:** Attempt 1 usually passes parse and visual check but self-collision triggers immediately. Attempt 2 needs: "exclude the neck segment from collision detection for the first frame after a direction change."

### Task H2: Pomodoro Timer with Session Log
- **Prompt:** "Make a Pomodoro timer: 25-minute work sessions, 5-minute breaks, alternating automatically. Show a log of completed sessions with timestamps."
- **Failure modes:** Phase transition fires twice (double `setInterval`); log timestamps use wrong timezone or show `NaN`; break timer counts down from 25 instead of 5; "stop" doesn't clear the interval.
- **Mechanical success:** `grep -c "25\|1500" index.html` ≥ 1 AND `grep -c "5\|300" index.html` ≥ 1 AND `grep -c "clearInterval" index.html` ≥ 1 AND `grep -c "log\|history\|session" index.html` ≥ 2.
- **Multi-attempt sig:** Attempt 1 often produces a working single-session timer but the phase switch and log are broken. Attempt 2 needs explicit: "use a single interval; clear it before each phase switch; append to log array on phase end."

### Task H3: Markdown-to-HTML Live Preview
- **Prompt:** "Build a two-panel markdown editor: I type markdown on the left and see rendered HTML preview on the right, updating as I type. Support headers, bold, italic, and bullet lists."
- **Failure modes:** Regex order wrong (bold consumes italic markers); nested list items not handled; `innerHTML` injection without escaping angle brackets in code samples causes broken preview; headers only match at line start but regex is not multiline.
- **Mechanical success:** `grep -c "replace\|regex\|RegExp" index.html` ≥ 4 AND `grep -c "<h[1-3]\|\\*\\*\|##" index.html` ≥ 3 AND `grep -c "oninput\|keyup" index.html` ≥ 1.
- **Multi-attempt sig:** Attempt 1 often handles headers and bold but breaks on italic or list nesting. Attempt 2 needs: "apply transformations in this order: code blocks, headers, bold, italic, lists."

## Tier X — Extreme (multi-attempt mandatory, one-shot <5%)

> **NOTE 2026-05-22**: pilot run revealed DeepSeek-v4-flash one-shot succeeds on X3. Predicted success rate was too low. Recalibrate before reuse.

### Task X1: Minesweeper with Chord-Click
- **Prompt:** "Build Minesweeper with a 9x9 grid, 10 mines, left-click to reveal, right-click to flag, and chord-click (clicking a revealed number with enough flags around it) to reveal all neighbors at once."
- **Failure modes:** Chord-click not implemented or triggers on wrong conditions; flood-fill reveal recurses into flagged cells; mine count display goes negative; first-click mine placement not deferred (instant loss possible).
- **Mechanical success:** `grep -c "contextmenu\|right.?click\|button.*2\|which.*3" index.html` ≥ 1 AND `grep -c "flag\|Flag" index.html` ≥ 2 AND `grep -c "flood\|reveal.*neighbor\|forEach\|recursive" index.html` ≥ 1 AND `grep -c "chord\|adjacent.*flag\|flag.*adjacent" index.html` ≥ 1.
- **Multi-attempt sig:** Attempt 1 produces reveal + flag. Chord-click is absent or broken. Attempt 2 receives: test report showing chord-click not firing + the specific condition check (`adjacentMines === adjacentFlags`).

### Task X2: Undo/Redo Drawing Canvas
- **Prompt:** "Make a freehand drawing canvas where I can draw with my mouse, pick a color and brush size, and undo or redo my last strokes with buttons."
- **Failure modes:** Undo pops pixel-level states (crashes on large drawings); redo stack not cleared on new stroke; `mouseup` outside canvas leaves drawing mode active; state snapshot taken on every `mousemove` event (O(n) memory blowup).
- **Mechanical success:** `grep -c "toDataURL\|getImageData\|history\|undo.?stack" index.html` ≥ 1 AND `grep -c "undo\|Undo" index.html` ≥ 2 AND `grep -c "redo\|Redo" index.html` ≥ 2 AND `grep -c "mouseup\|pointerup" index.html` ≥ 1.
- **Multi-attempt sig:** Attempt 1 usually produces a working canvas with broken undo (snapshotting on every move). Attempt 2 needs: "snapshot canvas state only on mouseup, not mousemove; clear redo stack on any new stroke."

### Task X3: Wordle Clone
- **Prompt:** "Build a Wordle clone: 5-letter word guessing game, 6 attempts, with green/yellow/grey tile coloring and a built-in word list of at least 20 words."
- **Failure modes:** Color logic wrong (yellow shown when letter appears in correct position elsewhere in word); duplicate letter handling broken (two A's in guess when word has one A — both go yellow); keyboard on-screen state doesn't downgrade (yellow cell overwritten as green on later guess).
- **Mechanical success:** green ≥2 AND yellow ≥2 AND grey ≥1 AND word list ≥20.
- **Multi-attempt sig:** Attempt 1 produces visible tiles but duplicate-letter coloring is wrong. Attempt 2 needs the exact algorithm: "mark greens first, then for yellows only count remaining unmatched letters in the target."
- **Pilot result (2026-05-22)**: DeepSeek-v4-flash one-shot succeeded (green=5, yellow=4, grey=4, words=992). Taxonomy prediction was wrong; recalibrate.

## Tier R — Research / non-game intent

### Task R1: DeepMind Deep-Thinking Model Explainer
- **Prompt:** "I want to research how existing LLM agents and harnesses can approach Google DeepMind's deep-thinking model behavior. Make a structured explainer page."
- **Failure modes:** Page is a single `<p>` wall of text with no structure; section headers present in DOM but contain no content; citations are fabricated URLs with no visible href.
- **Mechanical success:** `grep -c "<h[12]" index.html` ≥ 4 AND `grep -c "<section\|<article\|<div.*class" index.html` ≥ 3 AND word count ≥ 400 AND `grep -c "DeepMind\|Gemini\|AlphaProof\|chain.of.thought" index.html` ≥ 3.
- **Multi-attempt sig:** Attempt 1 often produces good structure but thin content. Attempt 2 needs spec expansion: "each section must contain at least two paragraphs of substantive explanation."

### Task R2: Comparison Table — LLM Provider Landscape
- **Prompt:** "Make a reference page comparing the top 5 LLM providers: their flagship models, pricing tier, context window, and notable strengths. Make it easy to scan."
- **Failure modes:** Table renders but cells are empty; providers listed are fictional or outdated; table has no header row; page is a `<ul>` list instead of `<table>`.
- **Mechanical success:** `grep -c "<table\|<th\|<tr" index.html` ≥ 3 AND `grep -c "<th" index.html` ≥ 4 AND provider count ≥ 5.
- **Multi-attempt sig:** Attempt 1 uses a list instead of a table. Attempt 2 spec adds: "use an HTML `<table>` with `<thead>` and `<tbody>`; each row is one provider."

### Task R3: Personal Productivity Method Explainer
- **Prompt:** "Create a single-page guide explaining three popular productivity methods — GTD, Pomodoro, and Time Blocking — with a short description, pros, cons, and when to use each."
- **Failure modes:** Only one or two methods covered despite prompt listing three; pros/cons absent (just descriptions); methods mixed up (GTD description in Pomodoro section).
- **Mechanical success:** `grep -ciP "GTD|getting things done" index.html` ≥ 2 AND `grep -ci "pomodoro" index.html` ≥ 2 AND `grep -ciP "time.block" index.html` ≥ 2 AND `grep -ci "pros\|advantages\|benefits" index.html` ≥ 1 AND `grep -ci "cons\|disadvantages\|drawbacks" index.html` ≥ 1.
- **Multi-attempt sig:** Attempt 1 covers all three but skips pros/cons. Attempt 2 spec adds: "for each method include a labeled Pros section and a labeled Cons section."

## Recommended Test Order

Run in this sequence to expose tape-relay machinery first while burning the least token budget:

1. **X2 (Undo/Redo Canvas)** — fastest to expose the multi-attempt relay: attempt 1 will mechanically fail the snapshot-on-mouseup check; the failure signal is unambiguous and the fix in attempt 2 is narrow.
2. **X3 (Wordle)** — duplicate-letter logic failure is highly deterministic; tests whether the system can pass a specific algorithmic correction between attempts.
3. **H1 (Snake)** — validates that H-tier interaction bugs surface correctly without requiring the full complexity of X-tier chord logic.
4. **M2 (Flashcard Quiz)** — cheaper token cost; validates spec-expansion quality from Meta AI before burning X-tier budget.
5. **R1 (DeepMind Explainer)** — tests non-game intent path; run after game tiers confirm relay works so a content failure here is attributable to R-tier structure, not relay bugs.
6. **E1–E3** — run last as regression baseline; if E-tier fails after H/X succeed, that's a pipeline regression, not a difficulty-calibration issue.

## Post-mortem update (2026-05-22)

**The recommended test order assumed taxonomy predictions held. Reality:**

- X3 Wordle pilot one-shot succeeded — DeepSeek-v4-flash significantly above predicted <5% success rate.
- The canonical tape-relay validation used **forced-failure injection** (bad endpoint → LlmApiError) instead of waiting for the LLM to fail naturally on a "hard" task.
- See `B_ATOM_T_DESIGN_AND_RESULTS.md` and `PLAN_V6_OVERNIGHT_TAPE_RELAY_2026-05-22.md` for the actual validation pattern.
- For future test matrices: either pick genuinely-hard tasks (current-LLM-aware) OR use forced-failure injection. The H/X/M predictions in this doc are likely 1-2 tiers too pessimistic for 2026-era DeepSeek/Claude/GPT.
