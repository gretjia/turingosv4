#!/usr/bin/env node
// TISR Phase 6.3 — functional gameplay verification (jsdom)
//
// Loads the generated index.html in jsdom, finds the 9 cells, drives a
// canonical win sequence, and asserts the resulting state.
//
// Win sequence we drive:
//   X(0) - O(3) - X(1) - O(4) - X(2)
// This puts X in the top row → top-row win. Expected outcome: status text
// changes to mention "X wins" (in EN or "X 赢了" / "X 胜" in ZH).
//
// We also drive a "click an already-filled cell" sequence to verify the
// game refuses double-clicks (spec Q5 requirement).
//
// Usage:
//   node scripts/phase63_functional_play.js <path-to-index.html>
//
// Exit 0 if the game plays correctly, non-zero otherwise.

const fs = require('node:fs');
const path = require('node:path');
const { JSDOM } = require(path.join('/tmp/p63-test-deps/node_modules/jsdom'));

const htmlPath = process.argv[2];
if (!htmlPath) {
    console.error('usage: phase63_functional_play.js <index.html>');
    process.exit(2);
}

const html = fs.readFileSync(htmlPath, 'utf8');

// Setup JSDOM with script execution allowed.
const dom = new JSDOM(html, {
    runScripts: 'dangerously',
    pretendToBeVisual: true,
});
const { window } = dom;

// Some inline scripts attach handlers on DOMContentLoaded — flush that.
window.document.dispatchEvent(new window.Event('DOMContentLoaded'));

// ─── Find 9 clickable cells ───────────────────────────────────────────────
// LLM-generated code varies. Cells are commonly one of:
//   - 9 <div class="cell"> ...
//   - 9 <button class="cell" ...> or 9 <button data-index="i"> ...
//   - <td> in a <table>
// We pick whichever yields exactly 9 candidates.
function findCells(doc) {
    const candidateSelectors = [
        '.cell',
        '[data-cell]',
        '[data-index]',
        'button.cell',
        'div.cell',
        'td.cell',
        '#board div',
        '#board button',
        '#board td',
        '.board > div',
        '.board > button',
        '.board > td',
        '.grid > div',
        '.grid > button',
    ];
    for (const sel of candidateSelectors) {
        const found = doc.querySelectorAll(sel);
        if (found.length === 9) return Array.from(found);
    }
    // Last-resort heuristic: any 9 sibling clickable elements.
    const allButtons = Array.from(doc.querySelectorAll('button'));
    if (allButtons.length === 9) return allButtons;
    const allDivs = Array.from(doc.querySelectorAll('div'));
    // Filter to divs that look cell-sized (have onclick or class containing cell)
    const cellish = allDivs.filter(d =>
        d.getAttribute('onclick') ||
        (d.className && /cell|square|tile/.test(d.className.toLowerCase()))
    );
    if (cellish.length === 9) return cellish;
    return null;
}

const cells = findCells(window.document);
if (!cells) {
    console.error('[FAIL] could not locate 9 clickable cells in the rendered DOM');
    process.exit(1);
}
console.log(`[ok] found 9 cells (tag=${cells[0].tagName.toLowerCase()}, sel-heuristic ok)`);

// ─── Drive a canonical top-row X win ──────────────────────────────────────
// X(0) O(3) X(1) O(4) X(2) — X completes top row.
function click(el) {
    el.dispatchEvent(new window.Event('click', { bubbles: true }));
}

function cellText(c) {
    return (c.textContent || '').trim();
}

const sequence = [0, 3, 1, 4, 2];
for (const i of sequence) {
    click(cells[i]);
}

const after = sequence.map(i => cellText(cells[i]));
console.log(`[ok] sequence drove cells -> ${JSON.stringify(after)}`);

// ─── Assertions ───────────────────────────────────────────────────────────
let failures = 0;

// 1. Three X marks across top row (cells 0,1,2).
const top = [cellText(cells[0]), cellText(cells[1]), cellText(cells[2])];
const isAllX = top.every(t => /^X$/.test(t));
if (!isAllX) {
    console.error(`[FAIL] top row should be all X; got ${JSON.stringify(top)}`);
    failures++;
} else {
    console.log('[ok] top row is X,X,X');
}

// 2. Cells 3 and 4 are O.
const o0 = cellText(cells[3]);
const o1 = cellText(cells[4]);
if (o0 !== 'O' || o1 !== 'O') {
    console.error(`[FAIL] cells 3,4 should be O; got "${o0}" / "${o1}"`);
    failures++;
} else {
    console.log('[ok] O placements correct');
}

// 3. Win announcement somewhere in the page.
const pageText = window.document.body.textContent || '';
const winRegex = /(X\s*(wins?|赢|胜|won)|wins?\s*[:!]?\s*X|X.*won)/i;
if (!winRegex.test(pageText)) {
    console.error(`[FAIL] no "X wins" announcement detected in page text; sample: ${pageText.slice(0,200).replace(/\s+/g,' ')}`);
    failures++;
} else {
    console.log('[ok] X-wins announcement present');
}

// 4. Clicking an already-filled cell should NOT change it (per spec Q5).
const stateBefore = cellText(cells[0]);
click(cells[0]);
const stateAfter = cellText(cells[0]);
if (stateBefore !== stateAfter) {
    console.error(`[FAIL] already-filled cell mutated on second click (${stateBefore} -> ${stateAfter})`);
    failures++;
} else {
    console.log('[ok] already-filled cell rejects subsequent clicks');
}

// 5. Reset button exists and resetting clears the board.
function findResetButton(doc) {
    // Look for buttons whose text mentions reset / new game / 再来一局 / 重置 / 重新开始
    const candidates = Array.from(doc.querySelectorAll('button, a, [role="button"], [onclick]'));
    return candidates.find(el => {
        const t = (el.textContent || '').trim().toLowerCase();
        return /(reset|restart|new\s*game|再来一局|再玩一局|重新开始|重置|再来|清空)/.test(t)
            || /(reset|restart|new\s*game)/i.test(el.id || '')
            || /(reset|restart|new\s*game)/i.test(el.className || '');
    });
}
const reset = findResetButton(window.document);
if (!reset) {
    console.error('[FAIL] no reset / new-game button found');
    failures++;
} else {
    click(reset);
    const afterReset = cells.map(cellText);
    // A correctly-reset board has all 9 cells empty (or showing a non-X/O placeholder).
    const allEmpty = afterReset.every(t => t === '' || t === ' ' || /^\s*$/.test(t));
    if (!allEmpty) {
        console.error(`[FAIL] board not cleared after reset; cells=${JSON.stringify(afterReset)}`);
        failures++;
    } else {
        console.log('[ok] reset clears the board');
    }
}

if (failures === 0) {
    console.log(`\n[PASS] ${htmlPath}: gameplay simulation succeeded`);
    process.exit(0);
} else {
    console.error(`\n[FAIL] ${htmlPath}: ${failures} assertion(s) failed`);
    process.exit(1);
}
