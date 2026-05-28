// TRACE_MATRIX FC1-N5: Phase 7 web — citation DAG viewer (ζ-Sum style).
//
// Renders the multi-agent citation tree from GET /api/dag/by-session/<id> as an
// INDENTED NESTED TREE (├──/└──), faithful to the ASCII reference. Each node
// shows: tx id, author agent + role, price marker (● P1 / ○ P0 / (50%) never),
// BULL/BEAR dominance + YES/NO bets + bet count + whale (⚠W), and the Golden
// Path to OMEGA is highlighted. Deep subtrees (depth > 3) start collapsed.
//
// Read-only projection — never a truth source. XSS: createElement + textContent
// ONLY, never innerHTML.

import type { DagViewResponse, DagNode } from '../types/spec.js';

const ELEMENT_NAME = 'tos-citation-dag';
const STYLE_SENTINEL = 'tos-cdag-styles';
const AUTO_COLLAPSE_DEPTH = 3;

export class TosCitationDag extends HTMLElement {
  private _sessionId = '';
  private _data: DagViewResponse | null = null;
  private _byId = new Map<string, DagNode>();
  private _collapsed = new Set<string>();
  private _error = '';

  connectedCallback(): void {
    this._sessionId = this.getAttribute('session-id') ?? '';
    if (this._sessionId === '') {
      // Standalone /dag?session=<id> page — read the session from the URL.
      this._sessionId = new URLSearchParams(location.search).get('session') ?? '';
    }
    this.setAttribute('data-block-type', 'citation_dag');
    this._injectStyles();
    this._render();
    if (this._sessionId !== '') void this._fetch();
  }

  // ── Data ────────────────────────────────────────────────────────────────────

  private async _fetch(): Promise<void> {
    try {
      const resp = await fetch(`/api/dag/by-session/${encodeURIComponent(this._sessionId)}`);
      if (resp.status === 404) {
        this._error = '该会话没有链上 DAG 数据。';
        this._render();
        return;
      }
      if (!resp.ok) {
        this._error = `加载失败 (HTTP ${resp.status})。`;
        this._render();
        return;
      }
      this._data = (await resp.json()) as DagViewResponse;
      this._error = '';
      this._byId = new Map(this._data.nodes.map((n) => [n.tx_id, n]));
      // Default-collapse parents below AUTO_COLLAPSE_DEPTH so big trees stay readable.
      this._collapsed = new Set(
        this._data.nodes
          .filter((n) => n.children.length > 0 && n.depth >= AUTO_COLLAPSE_DEPTH)
          .map((n) => n.tx_id),
      );
      this._render();
    } catch {
      this._error = '网络错误。';
      this._render();
    }
  }

  // ── Render ────────────────────────────────────────────────────────────────

  private _render(): void {
    while (this.firstChild) this.removeChild(this.firstChild);

    const header = document.createElement('div');
    header.className = 'cdag-header';
    const title = document.createElement('h2');
    title.className = 'cdag-title';
    title.textContent = '引用 DAG · Citation Tree';
    header.appendChild(title);
    this.appendChild(header);

    if (this._error !== '') {
      const p = document.createElement('p');
      p.className = 'cdag-empty';
      p.textContent = this._error;
      this.appendChild(p);
      return;
    }
    if (this._data === null) {
      const p = document.createElement('p');
      p.className = 'cdag-empty';
      p.textContent = '加载中…';
      this.appendChild(p);
      return;
    }

    const d = this._data;
    const sub = document.createElement('div');
    sub.className = 'cdag-rootline';
    sub.textContent = `ROOT — ${d.root.node_count} 节点 · ${d.root.roots} 根 · ${d.root.traded} 已交易 / ${d.root.untraded} 未交易`;
    this.appendChild(sub);

    // ── Tree block (white-space:pre, monospace) ──
    const tree = document.createElement('div');
    tree.className = 'cdag-tree';
    const roots = d.nodes.filter((n) => n.parent_tx === null);
    roots.forEach((root, i) => {
      this._renderNode(tree, root, '', i === roots.length - 1);
    });
    if (roots.length === 0) {
      const none = document.createElement('div');
      none.className = 'cdag-empty';
      none.textContent = '（无节点）';
      tree.appendChild(none);
    }
    this.appendChild(tree);

    // ── Golden path ──
    if (d.golden_path.length > 0) {
      const gp = document.createElement('div');
      gp.className = 'cdag-panel';
      const h = document.createElement('h3');
      h.className = 'cdag-panel-h';
      h.textContent = `Golden Path → OMEGA（${d.golden_path.length} 步）`;
      gp.appendChild(h);
      const line = document.createElement('div');
      line.className = 'cdag-gp-line';
      line.textContent = d.golden_path.map((s) => this._short(s)).join('  →  ');
      gp.appendChild(line);
      this.appendChild(gp);
    }

    this._appendSummary(d);
  }

  /** Recursively append one node row + its (expanded) children. */
  private _renderNode(host: HTMLElement, node: DagNode, prefix: string, isLast: boolean): void {
    const row = document.createElement('div');
    row.className = node.on_golden_path ? 'cdag-row cdag-golden' : 'cdag-row';

    // prefix + branch glyph (monospace, preserved via white-space:pre on .cdag-tree)
    const connector = prefix === '' && node.depth === 0 ? '' : isLast ? '└── ' : '├── ';
    const glyph = document.createElement('span');
    glyph.className = 'cdag-glyph';
    glyph.textContent = prefix + connector;
    row.appendChild(glyph);

    // collapse toggle (only for parents)
    const hasKids = node.children.length > 0;
    if (hasKids) {
      const toggle = document.createElement('span');
      toggle.className = 'cdag-toggle';
      const collapsed = this._collapsed.has(node.tx_id);
      toggle.textContent = collapsed ? '▸ ' : '▾ ';
      toggle.addEventListener('click', () => {
        if (this._collapsed.has(node.tx_id)) this._collapsed.delete(node.tx_id);
        else this._collapsed.add(node.tx_id);
        this._render();
      });
      row.appendChild(toggle);
    }

    // tx id
    const tx = document.createElement('span');
    tx.className = 'cdag-tx';
    tx.textContent = this._short(node.tx_id);
    row.appendChild(tx);

    // (agent/role)
    const who = document.createElement('span');
    who.className = 'cdag-who';
    who.textContent = ` (${node.agent}/${node.role})`;
    row.appendChild(who);

    // [DOMINANCE yesY/noN B=bet ⚠W]
    if (node.dominance !== null) {
      const tag = document.createElement('span');
      tag.className = node.dominance === 'BULL' ? 'cdag-bull' : 'cdag-bear';
      const whale = node.whale ? ' ⚠W' : '';
      tag.textContent = ` [${node.dominance} ${node.yes_units}Y/${node.no_units}N B=${node.bet_count}${whale}]`;
      row.appendChild(tag);
    }

    // price marker
    const mark = document.createElement('span');
    mark.className = 'cdag-mark';
    mark.textContent =
      node.price_marker === 'P1' ? ' ●' : node.price_marker === 'P0' ? ' ○' : ' (50%)';
    row.appendChild(mark);

    if (node.on_golden_path) {
      const gp = document.createElement('span');
      gp.className = 'cdag-gp-tag';
      gp.textContent = ' ✓GP';
      row.appendChild(gp);
    }

    host.appendChild(row);

    // children
    if (hasKids && !this._collapsed.has(node.tx_id)) {
      const childPrefix = prefix + (node.depth === 0 ? '' : isLast ? '    ' : '│   ');
      const kids = node.children
        .map((id) => this._byId.get(id))
        .filter((n): n is DagNode => n !== undefined);
      kids.forEach((kid, i) => {
        this._renderNode(host, kid, childPrefix, i === kids.length - 1);
      });
    } else if (hasKids) {
      // collapsed — show descendant count hint on the next line
      const hint = document.createElement('div');
      hint.className = 'cdag-row cdag-hint';
      const childPrefix = prefix + (node.depth === 0 ? '' : isLast ? '    ' : '│   ');
      hint.textContent = `${childPrefix}    … ${this._descendantCount(node)} 个后代（点 ▸ 展开）`;
      host.appendChild(hint);
    }
  }

  private _descendantCount(node: DagNode): number {
    let count = 0;
    const stack = [...node.children];
    while (stack.length > 0) {
      const id = stack.pop() as string;
      const n = this._byId.get(id);
      if (n === undefined) continue;
      count += 1;
      stack.push(...n.children);
    }
    return count;
  }

  private _appendSummary(d: DagViewResponse): void {
    const s = d.summary;
    // role activity
    if (s.role_activity.length > 0) {
      const panel = this._panel('角色活动 · Role Activity');
      for (const r of s.role_activity) {
        const line = document.createElement('div');
        line.className = 'cdag-srow';
        line.textContent = `${r.agent} — ${r.role}  (work ${r.work} · verify ${r.verify} · challenge ${r.challenge})`;
        panel.appendChild(line);
      }
      this.appendChild(panel);
    }
    // top contested
    if (s.top_contested.length > 0) {
      const panel = this._panel('争议最大节点 · Top Contested');
      for (const c of s.top_contested) {
        const line = document.createElement('div');
        line.className = 'cdag-srow';
        line.textContent = `${this._short(c.tx_id)}  YES ${c.yes} / NO ${c.no}  · ${c.bets} 注`;
        panel.appendChild(line);
      }
      this.appendChild(panel);
    }
    // whales
    if (s.whales.length > 0) {
      const panel = this._panel('鲸鱼节点 · Whales (>500C)');
      for (const w of s.whales) {
        const line = document.createElement('div');
        line.className = 'cdag-srow';
        line.textContent = `${this._short(w.tx_id)}  ${w.agent}  · 总 ${w.total}C`;
        panel.appendChild(line);
      }
      this.appendChild(panel);
    }
  }

  private _panel(titleText: string): HTMLElement {
    const panel = document.createElement('div');
    panel.className = 'cdag-panel';
    const h = document.createElement('h3');
    h.className = 'cdag-panel-h';
    h.textContent = titleText;
    panel.appendChild(h);
    return panel;
  }

  /** Trim long tx ids for display (keep head + tail). */
  private _short(s: string): string {
    if (s === 'ROOT' || s === 'OMEGA') return s;
    if (s.length <= 26) return s;
    return `${s.slice(0, 16)}…${s.slice(-6)}`;
  }

  private _injectStyles(): void {
    if (document.querySelector(`style.${STYLE_SENTINEL}`) !== null) return;
    const style = document.createElement('style');
    style.className = STYLE_SENTINEL;
    style.textContent = CDAG_CSS;
    document.head.appendChild(style);
  }
}

const CDAG_CSS = `
tos-citation-dag {
  display: block; margin: 1.2rem 0;
  font-family: "IBM Plex Sans", system-ui, -apple-system, sans-serif;
  color: var(--aap-text, #1a1a1a);
}
.cdag-header { display: flex; align-items: baseline; gap: 0.75rem; margin: 0 0 0.5rem; }
.cdag-title { font-family: "Fraunces", Georgia, serif; font-size: 1.35rem; font-weight: 600; margin: 0; }
.cdag-rootline {
  font-family: "JetBrains Mono", ui-monospace, monospace; font-size: 0.74rem;
  color: #6b6b6b; margin-bottom: 0.6rem;
}
.cdag-empty { font-size: 0.9rem; color: #6b6b6b; font-style: italic; }
.cdag-tree {
  font-family: "JetBrains Mono", ui-monospace, monospace; font-size: 0.78rem;
  line-height: 1.5; white-space: pre; overflow-x: auto;
  border: 1px solid var(--aap-border, #d8d4c8); border-radius: 8px;
  padding: 0.7rem 0.9rem; background: var(--aap-card-bg, #fff);
}
.cdag-row { white-space: pre; }
.cdag-glyph { color: #b9b4a8; }
.cdag-toggle { cursor: pointer; color: #4e8b7a; user-select: none; }
.cdag-tx { color: #1a1a1a; font-weight: 600; }
.cdag-who { color: #6b6b6b; }
.cdag-bull { color: #2d6a3f; }
.cdag-bear { color: #b71c1c; }
.cdag-mark { color: #6b6b6b; }
.cdag-hint { color: #9a968c; font-style: italic; }
.cdag-golden { background: color-mix(in srgb, #4e8b7a 12%, transparent); }
.cdag-golden .cdag-tx { color: #134B48; }
.cdag-gp-tag { color: #4e8b7a; font-weight: 700; }
.cdag-panel {
  margin-top: 0.9rem; padding: 0.6rem 0.8rem;
  border: 1px solid var(--aap-border, #d8d4c8); border-left: 3px solid #4e8b7a; border-radius: 8px;
}
.cdag-panel-h {
  font-family: "JetBrains Mono", ui-monospace, monospace; font-size: 0.66rem;
  letter-spacing: 0.18em; text-transform: uppercase; color: #6b6b6b; margin: 0 0 0.4rem;
}
.cdag-gp-line, .cdag-srow {
  font-family: "JetBrains Mono", ui-monospace, monospace; font-size: 0.74rem;
  color: #1a1a1a; line-height: 1.6; word-break: break-word;
}
@media (prefers-color-scheme: dark) {
  tos-citation-dag { color: #e8e4da; }
  .cdag-tree { background: #252320; border-color: #3a3830; }
  .cdag-tx { color: #e8e4da; }
  .cdag-who, .cdag-rootline, .cdag-mark, .cdag-panel-h { color: #a8a59c; }
  .cdag-bull { color: #7dc99a; }
  .cdag-bear { color: #ef9a9a; }
  .cdag-golden .cdag-tx { color: #9fe0d4; }
  .cdag-gp-line, .cdag-srow { color: #e8e4da; }
}
`;

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosCitationDag);
  }
}
