// Tests for component module structure.
//
// Full customElements.define() verification is deferred to the §6a Chrome
// verifier because Node.js does not ship a built-in DOM. Adding happy-dom/jsdom
// would pull in ~15 MB of transitive deps which violates the Phase 7 §7 LOC /
// dependency budget ceiling.
//
// What IS tested here (without a DOM):
//   - Each component source file exists at the expected path.
//   - The router source file exists.
//   - The ir.ts type file exists.
//   - package.json declares tsx as a devDependency.
//
// DOM-dependent tests (customElements.get verification) are explicitly deferred.

import { test } from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const srcDir = join(__dirname, '..', 'src');
const componentsDir = join(srcDir, 'components');

function srcExists(rel: string): boolean {
  return existsSync(join(srcDir, rel));
}

test('ir.ts exists', () => {
  assert.ok(srcExists('ir.ts'), 'frontend/src/ir.ts must exist');
});

test('router.ts exists', () => {
  assert.ok(srcExists('router.ts'), 'frontend/src/router.ts must exist');
});

test('turingos-root.ts exists', () => {
  assert.ok(srcExists('turingos-root.ts'), 'frontend/src/turingos-root.ts must exist');
});

test('main.ts exists', () => {
  assert.ok(srcExists('main.ts'), 'frontend/src/main.ts must exist');
});

test('text-block component exists', () => {
  assert.ok(existsSync(join(componentsDir, 'text-block.ts')));
});

test('table-block component exists', () => {
  assert.ok(existsSync(join(componentsDir, 'table-block.ts')));
});

test('agent-card-block component exists', () => {
  assert.ok(existsSync(join(componentsDir, 'agent-card-block.ts')));
});

test('task-card-block component exists', () => {
  assert.ok(existsSync(join(componentsDir, 'task-card-block.ts')));
});

test('event-log-block component exists', () => {
  assert.ok(existsSync(join(componentsDir, 'event-log-block.ts')));
});

test('dashboard-panel-block component exists', () => {
  assert.ok(existsSync(join(componentsDir, 'dashboard-panel-block.ts')));
});

test('each component source contains register() export', () => {
  const components = [
    'text-block.ts',
    'table-block.ts',
    'agent-card-block.ts',
    'task-card-block.ts',
    'event-log-block.ts',
    'dashboard-panel-block.ts',
  ];
  for (const file of components) {
    const src = readFileSync(join(componentsDir, file), 'utf8');
    assert.ok(
      src.includes('export function register()'),
      `${file} must export register()`
    );
  }
});

test('each component source sets data-block-type attribute', () => {
  const components: Array<[string, string]> = [
    ['text-block.ts', 'data-block-type', ],
    ['table-block.ts', 'data-block-type'],
    ['agent-card-block.ts', 'data-block-type'],
    ['task-card-block.ts', 'data-block-type'],
    ['event-log-block.ts', 'data-block-type'],
    ['dashboard-panel-block.ts', 'data-block-type'],
  ];
  for (const [file, attr] of components) {
    const src = readFileSync(join(componentsDir, file), 'utf8');
    assert.ok(src.includes(attr), `${file} must reference '${attr}'`);
  }
});

test('no component uses innerHTML with dynamic strings (XSS hygiene)', () => {
  const files = [
    ...['text-block.ts', 'table-block.ts', 'agent-card-block.ts',
        'task-card-block.ts', 'event-log-block.ts', 'dashboard-panel-block.ts'
       ].map(f => join(componentsDir, f)),
    join(srcDir, 'turingos-root.ts'),
  ];
  for (const filepath of files) {
    const src = readFileSync(filepath, 'utf8');
    // Forbidden patterns: .innerHTML = and innerHTML +=
    // Allowed: static comments mentioning innerHTML in the negative.
    const lines = src.split('\n');
    for (const [i, line] of lines.entries()) {
      const stripped = line.replace(/\/\/.*$/, ''); // strip line comments
      assert.ok(
        !stripped.includes('.innerHTML'),
        `${filepath}:${i + 1} must not use .innerHTML — XSS hygiene violation`
      );
    }
  }
});

test('package.json declares tsx devDependency', () => {
  const pkg = JSON.parse(readFileSync(join(__dirname, '..', 'package.json'), 'utf8')) as {
    devDependencies: Record<string, string>;
  };
  assert.ok(
    'tsx' in pkg.devDependencies,
    'package.json must list tsx in devDependencies'
  );
});
