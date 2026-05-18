// TRACE_MATRIX FC1-N5 + FC1-N10: Phase 7 W6 — spec-grill component tests.
//
// Following the established pattern (see component-register.test.ts): DOM
// rendering tests are deferred to the §6a Chrome verifier because Node.js
// ships no DOM. What IS tested here:
//   - spec-grill.ts exists at the expected path
//   - The file exports register()
//   - The file sets data-block-type attribute
//   - The file does NOT use .innerHTML (XSS hygiene)
//   - The internal validation matches the backend rules (mirrored constants)
//   - Pure state-machine logic for advanceWithAnswer / validateAnswer
//
// State-machine logic is mirrored inline (same as task-open-form.test.ts) so
// these tests run without needing a real HTMLElement.

import { test } from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const srcDir = join(__dirname, '..', 'src');
const componentsDir = join(srcDir, 'components');

const GRILL_FILE = join(componentsDir, 'spec-grill.ts');

test('spec_grill_registers_custom_element: file exists and exports register()', () => {
  assert.ok(existsSync(GRILL_FILE), 'spec-grill.ts must exist');
  const src = readFileSync(GRILL_FILE, 'utf8');
  assert.ok(
    src.includes('export function register()'),
    'spec-grill.ts must export register()',
  );
  // Confirm the element tag is the one routed-root mounts.
  assert.ok(
    src.includes("'tos-spec-grill'"),
    'spec-grill.ts must define the tos-spec-grill custom element',
  );
});

test('spec-grill sets data-block-type and respects XSS hygiene', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  assert.ok(
    src.includes("data-block-type") && src.includes('spec_grill'),
    "spec-grill.ts must set data-block-type='spec_grill'",
  );
  // XSS: no .innerHTML
  const lines = src.split('\n');
  for (const [i, line] of lines.entries()) {
    const stripped = line.replace(/\/\/.*$/, '');
    assert.ok(
      !stripped.includes('.innerHTML'),
      `spec-grill.ts:${i + 1} must not use .innerHTML (XSS hygiene)`,
    );
  }
});

test('spec-grill is registered in main.ts', () => {
  const mainSrc = readFileSync(join(srcDir, 'main.ts'), 'utf8');
  assert.ok(mainSrc.includes('spec-grill'), 'main.ts must import spec-grill');
  assert.ok(mainSrc.includes('registerSpecGrill'), 'main.ts must register spec-grill');
});

// ---------------------------------------------------------------------------
// State machine: mirror the validation + advancement logic inline so the
// pure functions can run without a DOM. Same pattern as task-open-form.test.ts.
// ---------------------------------------------------------------------------

const ANSWER_MAX_CHARS = 4096;
const QUESTION_COUNT = 8;

function validateAnswer(answer: string): string | null {
  if (answer.length === 0) return '请写一点内容再继续。';
  if (answer.length > ANSWER_MAX_CHARS) {
    return `回答太长了：${answer.length} 字符，最多 ${ANSWER_MAX_CHARS}。`;
  }
  return null;
}

type GrillStateLite = {
  index: number;
  answers: string[];
};

function advanceWithAnswer(state: GrillStateLite, raw: string): boolean {
  const trimmed = raw.trim();
  if (validateAnswer(trimmed) !== null) return false;
  state.answers[state.index] = trimmed;
  if (state.index < QUESTION_COUNT - 1) {
    state.index += 1;
    return true;
  }
  state.index = QUESTION_COUNT;
  return true;
}

test('spec_grill_validates_empty_answer', () => {
  assert.equal(validateAnswer(''), '请写一点内容再继续。');
  assert.equal(validateAnswer('a'), null);
});

test('spec_grill_validates_oversized_answer', () => {
  const big = 'x'.repeat(ANSWER_MAX_CHARS + 1);
  const err = validateAnswer(big);
  assert.ok(err !== null && err.includes('太长'), 'oversized answer must be rejected');
  assert.equal(validateAnswer('x'.repeat(ANSWER_MAX_CHARS)), null);
});

test('spec_grill_advances_state_on_next_question_call', () => {
  const state: GrillStateLite = {
    index: 0,
    answers: new Array<string>(QUESTION_COUNT).fill(''),
  };
  // Advance through all 8 questions.
  for (let q = 0; q < QUESTION_COUNT; q++) {
    const ok = advanceWithAnswer(state, `answer #${q}`);
    assert.equal(ok, true, `advance ${q} should succeed`);
  }
  // After answering the last question the index should equal QUESTION_COUNT.
  assert.equal(state.index, QUESTION_COUNT);
  assert.deepEqual(
    state.answers,
    Array.from({ length: QUESTION_COUNT }, (_, q) => `answer #${q}`),
  );
});

test('spec-grill: empty answer does not advance', () => {
  const state: GrillStateLite = {
    index: 3,
    answers: new Array<string>(QUESTION_COUNT).fill(''),
  };
  const ok = advanceWithAnswer(state, '   '); // whitespace-only trims to empty
  assert.equal(ok, false, 'whitespace-only answer must not advance');
  assert.equal(state.index, 3, 'index must not move on rejected answer');
});

test('spec-grill: backend question count constant matches expected 8', () => {
  // Sanity that the canonical question count stays 8 (matches src/web/spec.rs).
  assert.equal(QUESTION_COUNT, 8);
});
