// TRACE_MATRIX FC1-N5: Phase 7 web — live agent-presence causal tree.
//
// The "看板": a left-to-right causal flow of one generate session —
//   INTAKE·spec  →  CANDIDATES·generate  →  VERIFY  →  SETTLED
//
// Two data layers (Karpathy truth boundary):
//   - COMMITTED (authoritative): GET /api/market/by-session/<id> — ChainTape
//     replay. Solid nodes. This decides accepted/rejected/winner.
//   - PENDING (derived cursor): GET /api/progress/by-session/<id> — the
//     generate_progress.jsonl stream. Dashed/shimmer nodes for work that is
//     in flight but not yet on the tape. The committed layer ALWAYS wins:
//     once an agent appears in the market view, its pending node is dropped.
//
// Off-tape compute (the LLM black box) emits no tape facts, so we poll the
// progress stream (~2s) to advance the cursor; the user accepted this latency
// over a kernel-touching synchronous push.
//
// XSS hygiene: createElement + textContent only — NEVER innerHTML.

import type { WsMessage } from '../ir.js';
import type {
  MarketViewResponse,
  AgentCandidateView,
  MarketProjection,
  ProgressEvent,
  ProgressViewResponse,
} from '../types/spec.js';

const ELEMENT_NAME = 'tos-agent-presence-tree';
const STYLE_SENTINEL = 'tos-apt-styles';
const POLL_MS = 2000;
const MAX_POLLS = 150; // ~5 min safety cap

type NodePhase =
  | 'computing'
  | 'submitted'
  | 'accepted'
  | 'rejected'
  | 'pending_dispatch';

interface WorkerNode {
  agent: string;
  phase: NodePhase;
  candidate: AgentCandidateView | null; // committed view, if any
}

const PHASE_LABEL: Record<NodePhase, string> = {
  computing: '计算中…',
  submitted: '已提交 · 待入账',
  accepted: '已采纳',
  rejected: '已淘汰',
  pending_dispatch: '入账中…',
};

export class TosAgentPresenceTree extends HTMLElement {
  private _sessionId = '';
  private _events: ProgressEvent[] = [];
  private _committed: MarketViewResponse | null = null;
  private _pollTimer: number | null = null;
  private _pollCount = 0;
  private _wsListener: ((e: Event) => void) | null = null;

  connectedCallback(): void {
    this._sessionId = this.getAttribute('session-id') ?? '';
    this.setAttribute('data-block-type', 'agent_presence_tree');
    this._injectStyles();
    this._render();
    if (this._sessionId === '') return;

    void this._fetchProgress();
    void this._fetchMarket();

    this._wsListener = (e: Event) => this._onWsMessage(e);
    document.addEventListener('turingos:ir_update', this._wsListener);

    this._startPolling();
  }

  disconnectedCallback(): void {
    this._stopPolling();
    if (this._wsListener !== null) {
      document.removeEventListener('turingos:ir_update', this._wsListener);
      this._wsListener = null;
    }
  }

  // ── Data layer ────────────────────────────────────────────────────────────

  private get _settled(): boolean {
    return (
      this._committed !== null &&
      (this._committed.market_state === 'finalized' ||
        this._committed.market_state === 'all_rejected')
    );
  }

  private async _fetchProgress(): Promise<void> {
    if (this._sessionId === '') return;
    try {
      const resp = await fetch(
        `/api/progress/by-session/${encodeURIComponent(this._sessionId)}`,
      );
      if (!resp.ok) return;
      const data = (await resp.json()) as ProgressViewResponse;
      this._events = data.events;
      this._render();
    } catch {
      /* network blip — keep prior state */
    }
  }

  private async _fetchMarket(): Promise<void> {
    if (this._sessionId === '') return;
    try {
      const resp = await fetch(
        `/api/market/by-session/${encodeURIComponent(this._sessionId)}`,
      );
      if (resp.status === 404 || !resp.ok) return; // not on tape yet
      this._committed = (await resp.json()) as MarketViewResponse;
      this._render();
      if (this._settled) this._stopPolling();
    } catch {
      /* keep prior state */
    }
  }

  private _onWsMessage(e: Event): void {
    const detail = (e as CustomEvent<WsMessage | null>).detail;
    if (detail == null || this._sessionId === '') return;
    const t = detail.msg_type;
    const touchesSession =
      'session_id' in detail &&
      (detail as { session_id: string }).session_id === this._sessionId;
    if (!touchesSession) return;
    if (
      t === 'agent_attempt_update' ||
      t === 'generate_complete' ||
      t === 'generate_started' ||
      t === 'generate_attempt_started' ||
      t === 'generate_attempt_failed'
    ) {
      void this._fetchProgress();
      void this._fetchMarket();
    }
  }

  private _startPolling(): void {
    if (this._pollTimer !== null) return;
    this._pollTimer = window.setInterval(() => {
      this._pollCount += 1;
      if (this._settled || this._pollCount > MAX_POLLS) {
        this._stopPolling();
        return;
      }
      void this._fetchProgress();
      void this._fetchMarket();
    }, POLL_MS);
  }

  private _stopPolling(): void {
    if (this._pollTimer !== null) {
      window.clearInterval(this._pollTimer);
      this._pollTimer = null;
    }
  }

  // ── Merge: committed (authoritative) over pending (cursor) ─────────────────

  private _deriveWorkerNodes(): WorkerNode[] {
    // committed agent -> candidate
    const committedByAgent = new Map<string, AgentCandidateView>();
    for (const c of this._committed?.candidates ?? []) {
      committedByAgent.set(c.agent_id, c);
    }

    // latest progress stage per agent (worker_* only)
    const progressStage = new Map<string, 'computing' | 'submitted'>();
    const order: string[] = [];
    for (const ev of this._events) {
      // Only worker_* events name a worker. market_settled carries a non-worker
      // agent (a comma-joined roster) and is handled by the SETTLED column —
      // it must NEVER become a candidate node.
      if (ev.stage !== 'worker_start' && ev.stage !== 'worker_done') continue;
      if (!order.includes(ev.agent)) order.push(ev.agent);
      if (ev.stage === 'worker_start') {
        if (!progressStage.has(ev.agent)) progressStage.set(ev.agent, 'computing');
      } else {
        progressStage.set(ev.agent, 'submitted');
      }
    }

    // union of agents, committed order first then progress-only
    const agents: string[] = [];
    for (const c of this._committed?.candidates ?? []) agents.push(c.agent_id);
    for (const a of order) if (!agents.includes(a)) agents.push(a);

    return agents.map((agent) => {
      const committed = committedByAgent.get(agent) ?? null;
      let phase: NodePhase;
      if (committed !== null) {
        phase = committed.l4_state; // accepted | rejected | pending_dispatch
      } else {
        phase = progressStage.get(agent) ?? 'computing';
      }
      return { agent, phase, candidate: committed };
    });
  }

  // ── Render ─────────────────────────────────────────────────────────────────

  private _render(): void {
    while (this.firstChild) this.removeChild(this.firstChild);

    const header = document.createElement('div');
    header.className = 'apt-header';
    const title = document.createElement('h2');
    title.className = 'apt-title';
    title.textContent = '智能体工作流';
    header.appendChild(title);
    const live = document.createElement('span');
    live.className = this._settled ? 'apt-live apt-live--done' : 'apt-live';
    live.textContent = this._settled ? '已结算' : 'LIVE';
    header.appendChild(live);
    this.appendChild(header);

    const nodes = this._deriveWorkerNodes();

    const grid = document.createElement('div');
    grid.className = 'apt-grid';
    grid.appendChild(this._buildIntakeColumn());
    grid.appendChild(this._buildCandidatesColumn(nodes));
    grid.appendChild(this._buildVerifyColumn(nodes));
    grid.appendChild(this._buildSettledColumn(nodes));
    this.appendChild(grid);

    // Real CPMM market panel (PR #209): pool reserves + price + positions.
    const market = this._committed?.market;
    if (market != null && (market.pool_yes > 0 || market.pool_no > 0)) {
      this.appendChild(this._buildMarketPanel(market));
    }

    if (this._committed !== null) {
      const footer = document.createElement('p');
      footer.className = 'apt-footer';
      footer.textContent =
        `Treasury 赏金 ${this._committed.treasury_bounty_micro}µ · 状态 ${this._committed.market_state}`;
      this.appendChild(footer);
    }
  }

  private _buildMarketPanel(m: MarketProjection): HTMLElement {
    const panel = document.createElement('div');
    panel.className = 'apt-market';

    const head = document.createElement('div');
    head.className = 'apt-market-head';
    const eyebrow = document.createElement('span');
    eyebrow.className = 'apt-col-eyebrow';
    eyebrow.textContent = 'CPMM MARKET';
    head.appendChild(eyebrow);
    const price = document.createElement('span');
    price.className = 'apt-market-price';
    price.textContent = `YES 价格 ${(m.yes_signal_bp / 100).toFixed(1)}%`;
    head.appendChild(price);
    panel.appendChild(head);

    // Pool reserves
    const pool = document.createElement('div');
    pool.className = 'apt-market-pool';
    pool.textContent = `流动性池  YES ${m.pool_yes}  /  NO ${m.pool_no}`;
    panel.appendChild(pool);

    // Real router trades (PR #210): BUY YES / BUY NO counts + ratio.
    const yc = this._committed?.buy_yes_count ?? 0;
    const nc = this._committed?.buy_no_count ?? 0;
    if (yc > 0 || nc > 0) {
      const trades = document.createElement('div');
      trades.className = 'apt-market-pool';
      const ratio = nc > 0 ? `${(yc / nc).toFixed(1)}:1` : yc > 0 ? '∞' : '—';
      trades.textContent = `路由成交  买 YES ${yc} · 买 NO ${nc}  · 比 ${ratio}`;
      panel.appendChild(trades);
    }

    // Positions: who invested (YES) / who shorted (NO)
    const invested = m.positions.filter((p) => p.yes_shares > 0);
    const shorted = m.positions.filter((p) => p.no_shares > 0);
    const pos = document.createElement('div');
    pos.className = 'apt-market-pos';
    const investTxt =
      invested.length > 0
        ? '投资 YES：' + invested.map((p) => `${p.agent}(${p.yes_shares})`).join('、')
        : '投资 YES：无';
    const shortTxt =
      shorted.length > 0
        ? '做空 NO：' + shorted.map((p) => `${p.agent}(${p.no_shares})`).join('、')
        : '做空 NO：无';
    pos.textContent = `${investTxt}    ·    ${shortTxt}`;
    panel.appendChild(pos);

    return panel;
  }

  private _column(headEyebrow: string, headTitle: string): HTMLElement {
    const col = document.createElement('div');
    col.className = 'apt-col';
    const head = document.createElement('div');
    head.className = 'apt-col-head';
    const eyebrow = document.createElement('span');
    eyebrow.className = 'apt-col-eyebrow';
    eyebrow.textContent = headEyebrow;
    head.appendChild(eyebrow);
    const h = document.createElement('span');
    h.className = 'apt-col-title';
    h.textContent = headTitle;
    head.appendChild(h);
    col.appendChild(head);
    return col;
  }

  private _buildIntakeColumn(): HTMLElement {
    const col = this._column('INTAKE', 'spec');
    const node = document.createElement('div');
    node.className = 'apt-node apt-node--spec';
    const t = document.createElement('div');
    t.className = 'apt-node-title';
    t.textContent = '需求已确认';
    node.appendChild(t);
    const sub = document.createElement('div');
    sub.className = 'apt-node-sub';
    sub.textContent = `会话 ${this._sessionId.slice(0, 8)}`;
    node.appendChild(sub);
    col.appendChild(node);
    return col;
  }

  private _buildCandidatesColumn(nodes: WorkerNode[]): HTMLElement {
    const col = this._column('MARKET', 'generate');
    if (nodes.length === 0) {
      col.appendChild(this._placeholder('尚未生成 · 点「生成代码」后此处实时点亮'));
      return col;
    }
    for (const n of nodes) col.appendChild(this._buildWorkerNode(n));
    return col;
  }

  private _buildWorkerNode(n: WorkerNode): HTMLElement {
    const pending = n.phase === 'computing' || n.phase === 'submitted';
    const node = document.createElement('div');
    node.className = `apt-node apt-node--worker apt-phase--${n.phase}`;
    if (pending) node.classList.add('apt-node--pending');
    if (n.phase === 'computing') node.classList.add('apt-shimmer');

    const top = document.createElement('div');
    top.className = 'apt-node-top';
    const dot = document.createElement('span');
    dot.className = 'apt-dot';
    top.appendChild(dot);
    const name = document.createElement('span');
    name.className = 'apt-node-title';
    name.textContent = n.agent;
    top.appendChild(name);
    if (n.candidate?.is_winner) {
      const crown = document.createElement('span');
      crown.className = 'apt-crown';
      crown.textContent = '👑';
      crown.title = '中标';
      top.appendChild(crown);
    }
    node.appendChild(top);

    const phase = document.createElement('div');
    phase.className = 'apt-node-phase';
    phase.textContent = PHASE_LABEL[n.phase];
    node.appendChild(phase);

    if (n.candidate !== null) {
      const c = n.candidate;
      // Committed market facts: stake + market signal (yes_signal_bp → %).
      const meta = document.createElement('div');
      meta.className = 'apt-node-meta';
      const signalPct = c.yes_signal_bp !== null ? (c.yes_signal_bp / 100).toFixed(0) : '—';
      meta.textContent = `押注 ${c.stake_micro}µ · 市场信号 ${signalPct}%`;
      node.appendChild(meta);

      const cid = document.createElement('code');
      cid.className = 'apt-node-cid';
      cid.textContent = this._short(c.proposal_cid);
      cid.title = c.proposal_cid;
      node.appendChild(cid);
    }
    return node;
  }

  private _buildVerifyColumn(nodes: WorkerNode[]): HTMLElement {
    const col = this._column('VERIFY', '验证');
    const committed = nodes.filter((n) => n.candidate !== null);
    if (committed.length === 0) {
      col.appendChild(this._placeholder('待验证…'));
      return col;
    }
    for (const n of committed) {
      const c = n.candidate as AgentCandidateView;
      const node = document.createElement('div');
      node.className = `apt-node apt-node--verify apt-phase--${n.phase}`;
      const name = document.createElement('div');
      name.className = 'apt-node-title';
      name.textContent = n.agent;
      node.appendChild(name);

      const keys = Object.keys(c.predicate_results);
      if (keys.length > 0) {
        const passes = keys.filter((k) => c.predicate_results[k] === true).length;
        const summary = document.createElement('div');
        summary.className = 'apt-node-phase';
        summary.textContent = `判定 ${passes}/${keys.length} 通过`;
        node.appendChild(summary);
      } else {
        const summary = document.createElement('div');
        summary.className = 'apt-node-phase';
        summary.textContent = c.l4_state === 'accepted' ? '通过' : c.l4_state === 'rejected' ? '未通过' : '审核中';
        node.appendChild(summary);
      }
      if (c.rejection_class !== null && c.rejection_class !== '') {
        const rc = document.createElement('code');
        rc.className = 'apt-node-cid apt-reject';
        rc.textContent = c.rejection_class;
        node.appendChild(rc);
      }
      col.appendChild(node);
    }
    return col;
  }

  private _buildSettledColumn(nodes: WorkerNode[]): HTMLElement {
    const col = this._column('SETTLED', '结算');
    const winner =
      this._committed?.winner_agent_id ??
      nodes.find((n) => n.candidate?.is_winner)?.agent ??
      null;

    if (winner !== null) {
      const node = document.createElement('div');
      node.className = 'apt-node apt-node--settled apt-phase--accepted';
      const top = document.createElement('div');
      top.className = 'apt-node-top';
      const crown = document.createElement('span');
      crown.className = 'apt-crown';
      crown.textContent = '🏆';
      top.appendChild(crown);
      const name = document.createElement('span');
      name.className = 'apt-node-title';
      name.textContent = winner;
      top.appendChild(name);
      node.appendChild(top);
      const sub = document.createElement('div');
      sub.className = 'apt-node-phase';
      sub.textContent = `交付 · 赏金 ${this._committed?.treasury_bounty_micro ?? 0}µ`;
      node.appendChild(sub);
      col.appendChild(node);
      return col;
    }

    if (this._committed?.market_state === 'all_rejected') {
      col.appendChild(this._placeholder('全部淘汰，无交付'));
      return col;
    }
    const seen = this._events.some((e) => e.stage === 'market_settled');
    col.appendChild(this._placeholder(seen ? '结算中…' : '尚未结算'));
    return col;
  }

  private _placeholder(text: string): HTMLElement {
    const p = document.createElement('div');
    p.className = 'apt-node apt-node--ghost';
    p.textContent = text;
    return p;
  }

  private _short(cid: string): string {
    return cid.length > 18 ? `${cid.slice(0, 8)}…${cid.slice(-6)}` : cid;
  }

  // ── Styles (injected once) ──────────────────────────────────────────────────

  private _injectStyles(): void {
    if (document.querySelector(`style.${STYLE_SENTINEL}`) !== null) return;
    const style = document.createElement('style');
    style.className = STYLE_SENTINEL;
    style.textContent = APT_CSS;
    document.head.appendChild(style);
  }
}

const APT_CSS = `
tos-agent-presence-tree {
  display: block;
  margin: 1.5rem 0;
  font-family: "IBM Plex Sans", system-ui, -apple-system, sans-serif;
  color: var(--aap-text, #1a1a1a);
}
.apt-header { display: flex; align-items: baseline; gap: 0.75rem; margin: 0 0 1rem; }
.apt-title { font-family: "Fraunces", Georgia, serif; font-size: 1.35rem; font-weight: 600; margin: 0; }
.apt-live {
  font-family: "JetBrains Mono", ui-monospace, monospace; font-size: 0.7rem;
  letter-spacing: 0.18em; color: #4e8b7a; display: inline-flex; align-items: center; gap: 5px;
}
.apt-live::before {
  content: ""; width: 6px; height: 6px; border-radius: 50%; background: #4e8b7a;
  animation: apt-pulse 1.4s ease-in-out infinite;
}
.apt-live--done { color: #6b6b6b; }
.apt-live--done::before { background: #6b6b6b; animation: none; }
@keyframes apt-pulse { 0%,100% { opacity:1; transform:scale(1);} 50% { opacity:0.3; transform:scale(0.6);} }

.apt-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 0; align-items: start; }
.apt-col { padding: 0 14px; border-right: 1px solid var(--aap-border, #d8d4c8); display: flex; flex-direction: column; gap: 10px; }
.apt-col:first-child { padding-left: 0; }
.apt-col:last-child { border-right: none; padding-right: 0; }
.apt-col-head { display: flex; flex-direction: column; gap: 2px; margin-bottom: 4px; }
.apt-col-eyebrow {
  font-family: "JetBrains Mono", ui-monospace, monospace; font-size: 0.62rem;
  letter-spacing: 0.22em; color: #9a968c; text-transform: uppercase;
}
.apt-col-title { font-family: "Fraunces", Georgia, serif; font-style: italic; font-size: 0.95rem; color: #6b6b6b; }

.apt-node {
  background: var(--aap-card-bg, #fff); border: 1.5px solid var(--aap-border, #d8d4c8);
  border-radius: 9px; padding: 0.6rem 0.7rem; transition: border-color 0.2s, box-shadow 0.2s;
}
.apt-node-top { display: flex; align-items: center; gap: 0.4rem; }
.apt-node-title { font-family: "Fraunces", Georgia, serif; font-weight: 600; font-size: 0.95rem; }
.apt-node-sub, .apt-node-phase { font-size: 0.78rem; color: #6b6b6b; margin-top: 3px; }
.apt-node-meta {
  font-family: "JetBrains Mono", ui-monospace, monospace; font-size: 0.7rem;
  color: #8a8678; margin-top: 4px; letter-spacing: 0.01em;
}
.apt-node-cid {
  font-family: "JetBrains Mono", ui-monospace, monospace; font-size: 0.7rem; color: #6b6b6b;
  background: var(--aap-code-bg, #f5f1e8); border-radius: 4px; padding: 1px 5px;
  display: inline-block; margin-top: 5px;
}
.apt-reject { color: #b71c1c; background: #fdecea; }
.apt-dot { width: 8px; height: 8px; border-radius: 50%; background: #b9b4a8; flex: none; }
.apt-crown { font-size: 1rem; line-height: 1; }

.apt-node--spec { border-color: #4e8b7a; }
.apt-phase--computing .apt-dot { background: #4e8b7a; }
.apt-phase--submitted .apt-dot { background: #c8a23a; }
.apt-phase--accepted { border-color: #4e8b7a; }
.apt-phase--accepted .apt-dot { background: #4e8b7a; }
.apt-phase--rejected { border-color: #e57373; }
.apt-phase--rejected .apt-dot { background: #c62828; }
.apt-node--settled { border-color: #4e8b7a; background: #e8f0ee; }

.apt-node--pending { border-style: dashed; }
.apt-node--ghost {
  border-style: dashed; color: #9a968c; font-size: 0.8rem; font-style: italic;
  background: transparent; text-align: center;
}
.apt-shimmer { animation: apt-shimmer 1.6s ease-in-out infinite; }
@keyframes apt-shimmer { 0%,100% { opacity:1; } 50% { opacity:0.55; } }

.apt-market {
  margin-top: 1rem; padding: 0.7rem 0.9rem;
  border: 1px solid var(--aap-border, #d8d4c8); border-left: 3px solid #4e8b7a;
  border-radius: 8px; background: color-mix(in srgb, #4e8b7a 5%, transparent);
}
.apt-market-head { display: flex; align-items: baseline; justify-content: space-between; gap: 12px; }
.apt-market-price {
  font-family: "JetBrains Mono", ui-monospace, monospace; font-size: 0.82rem;
  font-weight: 600; color: #4e8b7a;
}
.apt-market-pool {
  font-family: "JetBrains Mono", ui-monospace, monospace; font-size: 0.76rem;
  color: #6b6b6b; margin-top: 6px;
}
.apt-market-pos {
  font-size: 0.78rem; color: #1a1a1a; margin-top: 5px;
}
.apt-footer {
  font-family: "JetBrains Mono", ui-monospace, monospace; font-size: 0.72rem; color: #6b6b6b;
  border-top: 1px solid var(--aap-border, #d8d4c8); padding-top: 0.5rem; margin-top: 0.9rem;
}
@media (prefers-color-scheme: dark) {
  tos-agent-presence-tree { color: #e8e4da; }
  .apt-col-title, .apt-node-sub, .apt-node-phase, .apt-footer { color: #a8a59c; }
  .apt-node-meta { color: #8f8b7e; }
  .apt-market-pos { color: #e8e4da; }
  .apt-market-pool { color: #a8a59c; }
  .apt-node { background: #252320; border-color: #3a3830; }
  .apt-node--settled { background: #1f2e2a; }
  .apt-node-cid { background: #2a2820; color: #a8a59c; }
  .apt-reject { color: #ef9a9a; background: #2d1010; }
}
`;

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosAgentPresenceTree);
  }
}
