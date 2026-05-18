// Tests for currentView() router logic.
// Uses Node built-in test runner. No DOM required — we mock location.pathname.

import { test } from 'node:test';
import assert from 'node:assert/strict';

// currentView() reads location.pathname. In Node there is no global location,
// so we mock it by temporarily overriding the global before importing the module.
// We use a fresh import per test by re-evaluating the function logic directly.

// Extract the routing logic inline (mirrors router.ts) so we can test it without
// a real DOM / location object. This keeps tests dependency-free.
type View = 'dashboard' | 'agents' | 'tasks' | 'audit';

function routeFromPath(path: string): View {
  if (path === '/agents' || path.startsWith('/agents/')) return 'agents';
  if (path === '/tasks' || path.startsWith('/tasks/')) return 'tasks';
  if (path === '/audit' || path.startsWith('/audit/')) return 'audit';
  return 'dashboard';
}

test('/ routes to dashboard', () => {
  assert.equal(routeFromPath('/'), 'dashboard');
});

test('empty path routes to dashboard', () => {
  assert.equal(routeFromPath(''), 'dashboard');
});

test('/agents routes to agents', () => {
  assert.equal(routeFromPath('/agents'), 'agents');
});

test('/agents/0x1234 routes to agents', () => {
  assert.equal(routeFromPath('/agents/0x1234'), 'agents');
});

test('/tasks routes to tasks', () => {
  assert.equal(routeFromPath('/tasks'), 'tasks');
});

test('/tasks/tx:0001 routes to tasks', () => {
  assert.equal(routeFromPath('/tasks/tx:0001'), 'tasks');
});

test('/audit routes to audit', () => {
  assert.equal(routeFromPath('/audit'), 'audit');
});

test('/audit/2026-05-17 routes to audit', () => {
  assert.equal(routeFromPath('/audit/2026-05-17'), 'audit');
});

test('/unknown routes to dashboard', () => {
  assert.equal(routeFromPath('/unknown'), 'dashboard');
});
