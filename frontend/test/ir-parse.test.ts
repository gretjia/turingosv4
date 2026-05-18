// Tests for IRRoot type parsing against fixture JSON.
// Uses Node built-in test runner via --test flag.
// No DOM required — pure type/data exercises.

import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

import type { IRRoot, Block } from '../src/ir.js';

const __dirname = dirname(fileURLToPath(import.meta.url));
const fixturesDir = join(__dirname, '..', '..', 'experiments', 'tisr_ui_spike', 'fixtures');

function loadFixture(name: string): unknown {
  const raw = readFileSync(join(fixturesDir, name), 'utf8');
  return JSON.parse(raw);
}

test('dashboard_sample.json parses as IRRoot', () => {
  const ir = loadFixture('dashboard_sample.json') as IRRoot;
  assert.equal(typeof ir.id, 'string', 'id must be string');
  assert.equal(typeof ir.title, 'string', 'title must be string');
  assert.ok(Array.isArray(ir.blocks), 'blocks must be array');
  assert.ok(ir.blocks.length > 0, 'blocks must be non-empty');
});

test('dashboard_sample.json block kinds are valid', () => {
  const ir = loadFixture('dashboard_sample.json') as IRRoot;
  const validKinds = new Set(['text', 'table', 'agent_card', 'task_card', 'event_log', 'dashboard_panel']);
  for (const block of ir.blocks) {
    assert.ok(validKinds.has(block.kind), `Unknown block kind: ${block.kind}`);
  }
});

test('dashboard_sample.json text block has content field', () => {
  const ir = loadFixture('dashboard_sample.json') as IRRoot;
  const textBlock = ir.blocks.find((b: Block) => b.kind === 'text');
  assert.ok(textBlock != null, 'should have at least one text block');
  assert.equal(typeof (textBlock as { content: unknown }).content, 'string');
});

test('dashboard_sample.json dashboard_panel block has panel_title and metrics', () => {
  const ir = loadFixture('dashboard_sample.json') as IRRoot;
  const panel = ir.blocks.find((b: Block) => b.kind === 'dashboard_panel');
  assert.ok(panel != null, 'should have at least one dashboard_panel block');
  const p = panel as { panel_title: unknown; metrics: unknown };
  assert.equal(typeof p.panel_title, 'string');
  assert.ok(Array.isArray(p.metrics));
});

test('dashboard_sample.json table block has columns and rows', () => {
  const ir = loadFixture('dashboard_sample.json') as IRRoot;
  const table = ir.blocks.find((b: Block) => b.kind === 'table');
  assert.ok(table != null, 'should have at least one table block');
  const t = table as { columns: unknown; rows: unknown };
  assert.ok(Array.isArray(t.columns));
  assert.ok(Array.isArray(t.rows));
});

test('dashboard_sample.json event_log block has events array', () => {
  const ir = loadFixture('dashboard_sample.json') as IRRoot;
  const elog = ir.blocks.find((b: Block) => b.kind === 'event_log');
  assert.ok(elog != null, 'should have at least one event_log block');
  const e = elog as { events: unknown };
  assert.ok(Array.isArray(e.events));
});

test('agent_view_sample.json agent_card block has agent_id, role, balance_micro', () => {
  const ir = loadFixture('agent_view_sample.json') as IRRoot;
  const card = ir.blocks.find((b: Block) => b.kind === 'agent_card');
  assert.ok(card != null, 'should have at least one agent_card block');
  const c = card as { agent_id: unknown; role: unknown; balance_micro: unknown };
  assert.equal(typeof c.agent_id, 'string');
  assert.equal(typeof c.role, 'string');
  assert.equal(typeof c.balance_micro, 'number');
});

test('task_view_sample.json task_card block has task_id, problem_id, status', () => {
  const ir = loadFixture('task_view_sample.json') as IRRoot;
  const card = ir.blocks.find((b: Block) => b.kind === 'task_card');
  assert.ok(card != null, 'should have at least one task_card block');
  const c = card as { task_id: unknown; problem_id: unknown; status: unknown };
  assert.equal(typeof c.task_id, 'string');
  assert.equal(typeof c.problem_id, 'string');
  assert.equal(typeof c.status, 'string');
});
