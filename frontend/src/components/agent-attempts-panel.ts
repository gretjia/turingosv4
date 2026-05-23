// TRACE_MATRIX FC1-N5: Phase 7 web — agent attempts side panel (Polymarket-wired generate).
// Renders N candidate cards from /api/market/by-session/<session-id> + WS updates.
// Strictly read-only (no betting UI per user-decision #3).
// XSS hygiene: createElement + textContent only — NEVER innerHTML.
// Sets data-block-type="agent_attempts_panel" on self.

import type { WsMessage } from '../ir.js';
import type { MarketViewResponse, AgentCandidateView } from '../types/spec.js';
import { truncateMiddle } from './render-helpers.js';

const ELEMENT_NAME = 'tos-agent-attempts-panel';

/** Lazily-injected <style> tag is keyed by this sentinel class so it is only
 *  written once per document, regardless of how many panel instances exist. */
const STYLE_SENTINEL = 'tos-aap-styles';

export class TosAgentAttemptsPanel extends HTMLElement {
  private _sessionId = '';
  private _candidates: AgentCandidateView[] = [];
  private _marketState: 'open' | 'finalized' | 'all_rejected' = 'open';
  private _winnerAgentId: string | null = null;
  private _treasuryBountyMicro = 0;
  private _pending = true;   // true while the initial fetch is in flight or returned 404
  private _wsListener: ((e: Event) => void) | null = null;

  connectedCallback(): void {
    this._sessionId = this.getAttribute('session-id') ?? '';
    this.setAttribute('data-block-type', 'agent_attempts_panel');

    this._injectStyles();
    this._render();

    void this._fetchMarket();

    this._wsListener = (e: Event) => this._onWsMessage(e);
    document.addEventListener('turingos:ir_update', this._wsListener);
  }

  disconnectedCallback(): void {
    if (this._wsListener !== null) {
      document.removeEventListener('turingos:ir_update', this._wsListener);
      this._wsListener = null;
    }
  }

  // -------------------------------------------------------------------------
  // Data layer
  // -------------------------------------------------------------------------

  private async _fetchMarket(): Promise<void> {
    if (this._sessionId === '') return;
    try {
      const resp = await fetch(`/api/market/by-session/${encodeURIComponent(this._sessionId)}`);
      if (resp.status === 404) {
        // Market not ready yet — stay in pending state.
        this._pending = true;
        this._render();
        return;
      }
      if (!resp.ok) {
        // Non-fatal: keep pending UI rather than crashing.
        this._pending = true;
        this._render();
        return;
      }
      const data = (await resp.json()) as MarketViewResponse;
      this._candidates = data.candidates;
      this._marketState = data.market_state;
      this._winnerAgentId = data.winner_agent_id;
      this._treasuryBountyMicro = data.treasury_bounty_micro;
      this._pending = false;
      this._render();
    } catch {
      // Network error — silent; keep pending state.
      this._render();
    }
  }

  private _onWsMessage(e: Event): void {
    const detail = (e as CustomEvent<WsMessage | null>).detail;
    if (detail == null) return;
    if (this._sessionId === '') return;

    // React to any market-state-relevant WS message that touches our session.
    const msgType = detail.msg_type;
    if (
      (msgType === 'agent_attempt_update' || msgType === 'generate_complete') &&
      'session_id' in detail &&
      (detail as { session_id: string }).session_id === this._sessionId
    ) {
      // Re-fetch from the authoritative endpoint (idempotent).
      void this._fetchMarket();
    }
  }

  // -------------------------------------------------------------------------
  // Styles (injected once per document)
  // -------------------------------------------------------------------------

  private _injectStyles(): void {
    if (document.querySelector(`style.${STYLE_SENTINEL}`) !== null) return;
    const style = document.createElement('style');
    style.className = STYLE_SENTINEL;
    style.textContent = [
      ':root {',
      '  --aap-accent: #4e8b7a;',
      '  --aap-bg: #f8f6f1;',
      '  --aap-text: #1a1a1a;',
      '  --aap-muted: #6b6b6b;',
      '  --aap-border: #d8d4c8;',
      '  --aap-card-bg: #fff;',
      '  --aap-accent-soft: #e8f0ee;',
      '  --aap-code-bg: #f5f1e8;',
      '}',
      '@media (prefers-color-scheme: dark) {',
      '  :root {',
      '    --aap-accent: #66a896;',
      '    --aap-bg: #1a1a1a;',
      '    --aap-text: #e8e4da;',
      '    --aap-muted: #a8a59c;',
      '    --aap-border: #3a3830;',
      '    --aap-card-bg: #252320;',
      '    --aap-accent-soft: #1f2e2a;',
      '    --aap-code-bg: #2a2820;',
      '  }',
      '}',
      'tos-agent-attempts-panel {',
      '  display: block;',
      '  margin: 1.5rem 0;',
      '  font-family: "IBM Plex Sans", system-ui, -apple-system, sans-serif;',
      '  color: var(--aap-text);',
      '}',
      '.aap-header {',
      '  display: flex;',
      '  align-items: baseline;',
      '  gap: 0.75rem;',
      '  margin: 0 0 1rem;',
      '}',
      '.aap-title {',
      '  font-family: "Fraunces", Georgia, serif;',
      '  font-size: 1.35rem;',
      '  font-weight: 600;',
      '  margin: 0;',
      '  color: var(--aap-text);',
      '}',
      '.aap-badge {',
      '  font-size: 0.78rem;',
      '  font-family: "IBM Plex Sans", system-ui, sans-serif;',
      '  font-weight: 500;',
      '  border-radius: 12px;',
      '  padding: 2px 10px;',
      '  letter-spacing: 0.02em;',
      '}',
      '.aap-badge--open {',
      '  background: var(--aap-accent-soft);',
      '  color: var(--aap-accent);',
      '}',
      '.aap-badge--finalized {',
      '  background: #e6f4ea;',
      '  color: #2d6a3f;',
      '}',
      '@media (prefers-color-scheme: dark) {',
      '  .aap-badge--finalized { background: #1d3226; color: #7dc99a; }',
      '}',
      '.aap-badge--all_rejected {',
      '  background: #fdecea;',
      '  color: #b71c1c;',
      '}',
      '@media (prefers-color-scheme: dark) {',
      '  .aap-badge--all_rejected { background: #2d1010; color: #ef9a9a; }',
      '}',
      '.aap-grid {',
      '  display: grid;',
      '  grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));',
      '  gap: 1rem;',
      '  margin-bottom: 1rem;',
      '}',
      '.aap-card {',
      '  background: var(--aap-card-bg);',
      '  border: 1.5px solid var(--aap-border);',
      '  border-radius: 10px;',
      '  padding: 1rem 1.1rem;',
      '  position: relative;',
      '  transition: border-color 0.15s;',
      '}',
      '.aap-card--accepted { border-color: var(--aap-accent); }',
      '.aap-card--rejected { border-color: #e57373; }',
      '.aap-card-top {',
      '  display: flex;',
      '  align-items: center;',
      '  gap: 0.5rem;',
      '  margin-bottom: 0.65rem;',
      '}',
      '.aap-agent-id {',
      '  font-family: "Fraunces", Georgia, serif;',
      '  font-size: 1rem;',
      '  font-weight: 600;',
      '  color: var(--aap-text);',
      '}',
      '.aap-crown {',
      '  font-size: 1.1rem;',
      '  margin-left: auto;',
      '  line-height: 1;',
      '}',
      '.aap-cid {',
      '  font-family: "JetBrains Mono", ui-monospace, monospace;',
      '  font-size: 0.78rem;',
      '  color: var(--aap-muted);',
      '  background: var(--aap-code-bg);',
      '  border-radius: 4px;',
      '  padding: 1px 5px;',
      '  margin-bottom: 0.4rem;',
      '  display: block;',
      '  overflow: hidden;',
      '  text-overflow: ellipsis;',
      '  white-space: nowrap;',
      '}',
      '.aap-stake {',
      '  font-size: 0.82rem;',
      '  color: var(--aap-muted);',
      '  margin-bottom: 0.4rem;',
      '}',
      '.aap-l4-badge {',
      '  display: inline-block;',
      '  font-size: 0.75rem;',
      '  border-radius: 8px;',
      '  padding: 1px 8px;',
      '  margin-bottom: 0.4rem;',
      '  font-weight: 500;',
      '}',
      '.aap-l4-badge--accepted { background: #e6f4ea; color: #2d6a3f; }',
      '.aap-l4-badge--rejected { background: #fdecea; color: #b71c1c; }',
      '.aap-l4-badge--pending_dispatch { background: var(--aap-accent-soft); color: var(--aap-accent); }',
      '@media (prefers-color-scheme: dark) {',
      '  .aap-l4-badge--accepted { background: #1d3226; color: #7dc99a; }',
      '  .aap-l4-badge--rejected { background: #2d1010; color: #ef9a9a; }',
      '}',
      '.aap-rejection-class {',
      '  font-family: "JetBrains Mono", ui-monospace, monospace;',
      '  font-size: 0.72rem;',
      '  color: #b71c1c;',
      '  margin-bottom: 0.35rem;',
      '}',
      '@media (prefers-color-scheme: dark) {',
      '  .aap-rejection-class { color: #ef9a9a; }',
      '}',
      '.aap-predicates {',
      '  margin: 0.35rem 0;',
      '  padding: 0;',
      '  list-style: none;',
      '  font-size: 0.8rem;',
      '}',
      '.aap-predicates li { display: flex; gap: 0.35rem; align-items: baseline; }',
      '.aap-pred-pass { color: var(--aap-accent); font-weight: 600; }',
      '.aap-pred-fail { color: #c62828; font-weight: 600; }',
      '.aap-price {',
      '  font-family: "JetBrains Mono", ui-monospace, monospace;',
      '  font-size: 0.82rem;',
      '  color: var(--aap-accent);',
      '  margin-top: 0.4rem;',
      '}',
      '.aap-pending {',
      '  font-size: 0.92rem;',
      '  color: var(--aap-muted);',
      '  font-style: italic;',
      '  padding: 0.5rem 0;',
      '}',
      '.aap-footer {',
      '  font-size: 0.78rem;',
      '  color: var(--aap-muted);',
      '  border-top: 1px solid var(--aap-border);',
      '  padding-top: 0.5rem;',
      '  margin-top: 0.25rem;',
      '}',
    ].join('\n');
    document.head.appendChild(style);
  }

  // -------------------------------------------------------------------------
  // Render
  // -------------------------------------------------------------------------

  private _render(): void {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }

    // --- Header ---
    const header = document.createElement('div');
    header.className = 'aap-header';

    const title = document.createElement('h2');
    title.className = 'aap-title';
    title.textContent = 'Agent attempts';
    header.appendChild(title);

    if (!this._pending) {
      const badge = document.createElement('span');
      badge.className = `aap-badge aap-badge--${this._marketState}`;
      if (this._marketState === 'open') {
        badge.textContent = '⏳ open';
      } else if (this._marketState === 'finalized') {
        badge.textContent = '✅ finalized';
      } else {
        badge.textContent = '❌ all_rejected';
      }
      header.appendChild(badge);
    }

    this.appendChild(header);

    // --- Pending state ---
    if (this._pending) {
      const p = document.createElement('p');
      p.className = 'aap-pending';
      p.textContent = '（等待市场数据…）';
      this.appendChild(p);
      return;
    }

    // --- Candidate grid ---
    const grid = document.createElement('div');
    grid.className = 'aap-grid';

    for (const candidate of this._candidates) {
      grid.appendChild(this._buildCard(candidate));
    }

    this.appendChild(grid);

    // --- Footer ---
    const footer = document.createElement('p');
    footer.className = 'aap-footer';
    const footerLabel = document.createElement('span');
    footerLabel.textContent = 'Treasury bounty: ';
    footer.appendChild(footerLabel);
    const footerValue = document.createElement('strong');
    footerValue.textContent = `${this._treasuryBountyMicro}µ`;
    footer.appendChild(footerValue);
    this.appendChild(footer);
  }

  private _buildCard(c: AgentCandidateView): HTMLElement {
    const card = document.createElement('div');
    card.className = `aap-card${c.l4_state === 'accepted' ? ' aap-card--accepted' : c.l4_state === 'rejected' ? ' aap-card--rejected' : ''}`;

    // --- Card top: emoji + agent_id + optional crown ---
    const top = document.createElement('div');
    top.className = 'aap-card-top';

    const emoji = document.createElement('span');
    emoji.setAttribute('aria-hidden', 'true');
    emoji.textContent = '🤖';
    top.appendChild(emoji);

    const agentId = document.createElement('span');
    agentId.className = 'aap-agent-id';
    agentId.textContent = c.agent_id;
    top.appendChild(agentId);

    if (c.is_winner) {
      const crown = document.createElement('span');
      crown.className = 'aap-crown';
      crown.setAttribute('title', 'Winner');
      crown.setAttribute('aria-label', 'Winner');
      crown.textContent = '👑';
      top.appendChild(crown);
    }

    card.appendChild(top);

    // --- proposal_cid ---
    const cid = document.createElement('code');
    cid.className = 'aap-cid';
    cid.title = c.proposal_cid;
    cid.textContent = truncateMiddle(c.proposal_cid, 8, 8);
    card.appendChild(cid);

    // --- stake ---
    const stake = document.createElement('p');
    stake.className = 'aap-stake';
    stake.textContent = `stake: ${c.stake_micro}µ`;
    card.appendChild(stake);

    // --- L4 state badge ---
    const l4Badge = document.createElement('span');
    l4Badge.className = `aap-l4-badge aap-l4-badge--${c.l4_state}`;
    if (c.l4_state === 'accepted') {
      l4Badge.textContent = '✅ accepted';
    } else if (c.l4_state === 'rejected') {
      l4Badge.textContent = '❌ rejected';
    } else {
      l4Badge.textContent = '⏳ pending';
    }
    card.appendChild(l4Badge);

    // --- rejection_class if rejected ---
    if (c.rejection_class !== null && c.rejection_class !== '') {
      const rc = document.createElement('p');
      rc.className = 'aap-rejection-class';
      rc.textContent = c.rejection_class;
      card.appendChild(rc);
    }

    // --- predicate results ---
    const predicateKeys = Object.keys(c.predicate_results);
    if (predicateKeys.length > 0) {
      const ul = document.createElement('ul');
      ul.className = 'aap-predicates';
      for (const key of predicateKeys) {
        const pass = c.predicate_results[key] === true;
        const li = document.createElement('li');

        const mark = document.createElement('span');
        mark.className = pass ? 'aap-pred-pass' : 'aap-pred-fail';
        mark.textContent = pass ? '✓' : '✗';
        li.appendChild(mark);

        const label = document.createElement('span');
        label.textContent = key;
        li.appendChild(label);

        ul.appendChild(li);
      }
      card.appendChild(ul);
    }

    // --- market signal ---
    if (c.yes_signal_bp !== null) {
      const price = document.createElement('p');
      price.className = 'aap-price';
      price.textContent = `${(c.yes_signal_bp / 100).toFixed(1)}% likely`;
      card.appendChild(price);
    }

    return card;
  }
}

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosAgentAttemptsPanel);
  }
}
