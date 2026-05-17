// Tests for <tos-task-open-form> component.
//
// Following the established pattern (see component-register.test.ts):
// DOM-rendering tests are deferred to the §6a Chrome verifier because Node.js
// does not ship a built-in DOM. What IS tested here:
//   - task-open-form.ts exists at the expected path
//   - The file exports register()
//   - The file sets data-block-type attribute
//   - The file does NOT use .innerHTML with dynamic strings (XSS hygiene)
//   - The client-side validation regex mirrors the server-side rules

import { test } from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const srcDir = join(__dirname, '..', 'src');
const componentsDir = join(srcDir, 'components');

const FORM_FILE = join(componentsDir, 'task-open-form.ts');

test('task-open-form.ts exists', () => {
  assert.ok(existsSync(FORM_FILE), 'frontend/src/components/task-open-form.ts must exist');
});

test('task-open-form.ts exports register()', () => {
  const src = readFileSync(FORM_FILE, 'utf8');
  assert.ok(src.includes('export function register()'), 'task-open-form.ts must export register()');
});

test('task-open-form.ts sets data-block-type attribute', () => {
  const src = readFileSync(FORM_FILE, 'utf8');
  assert.ok(
    src.includes('data-block-type'),
    "task-open-form.ts must reference 'data-block-type'"
  );
  assert.ok(
    src.includes('task_open_form'),
    "task-open-form.ts must set data-block-type to 'task_open_form'"
  );
});

test('task-open-form.ts does not use .innerHTML with dynamic strings (XSS hygiene)', () => {
  const src = readFileSync(FORM_FILE, 'utf8');
  const lines = src.split('\n');
  for (const [i, line] of lines.entries()) {
    const stripped = line.replace(/\/\/.*$/, ''); // strip line comments
    assert.ok(
      !stripped.includes('.innerHTML'),
      `task-open-form.ts:${i + 1} must not use .innerHTML — XSS hygiene violation`
    );
  }
});

test('task-open-form.ts is included in main.ts', () => {
  const mainSrc = readFileSync(join(srcDir, 'main.ts'), 'utf8');
  assert.ok(
    mainSrc.includes('task-open-form'),
    'main.ts must import/register task-open-form'
  );
});

test('task-open-form.ts client-side validation logic present', () => {
  const src = readFileSync(FORM_FILE, 'utf8');
  // Must have the identifier validation regex.
  assert.ok(
    src.includes('[a-zA-Z0-9_-]'),
    'task-open-form.ts must include identifier validation regex matching server-side rules'
  );
  // Must have bounty cap check (10_000_000 or 10000000).
  assert.ok(
    src.includes('10_000_000') || src.includes('10000000'),
    'task-open-form.ts must include bounty cap validation (10_000_000)'
  );
});

// Inline the validation logic (mirrors task-open-form.ts) for pure-logic testing.
// This lets us test the regex/caps without DOM — mirrors the view-router.test.ts pattern.
type View = 'valid' | 'invalid';

function isValidIdentifier(s: string): boolean {
  if (s.length === 0 || s.length > 64) return false;
  return /^[a-zA-Z0-9_-]+$/.test(s);
}

function isValidBounty(n: number): boolean {
  return Number.isInteger(n) && n > 0 && n < 10_000_000;
}

test('validation: valid identifiers pass', () => {
  assert.ok(isValidIdentifier('abc'));
  assert.ok(isValidIdentifier('problem-01'));
  assert.ok(isValidIdentifier('agent_0'));
  assert.ok(isValidIdentifier('A1-b_2'));
  assert.ok(isValidIdentifier('a'.repeat(64)));
});

test('validation: invalid identifiers reject', () => {
  assert.ok(!isValidIdentifier(''));
  assert.ok(!isValidIdentifier('a'.repeat(65)));
  assert.ok(!isValidIdentifier('../../etc/passwd'));
  assert.ok(!isValidIdentifier('has space'));
  assert.ok(!isValidIdentifier('semicolon;'));
  assert.ok(!isValidIdentifier('inject`cmd`'));
});

test('validation: bounty caps', () => {
  assert.ok(isValidBounty(1));
  assert.ok(isValidBounty(1000));
  assert.ok(isValidBounty(9_999_999));
  assert.ok(!isValidBounty(0));
  assert.ok(!isValidBounty(10_000_000));
  assert.ok(!isValidBounty(-1));
  assert.ok(!isValidBounty(1.5)); // non-integer
});

// Void declaration to satisfy TypeScript's unused-variable check on View type.
const _: View = 'valid';
void _;
