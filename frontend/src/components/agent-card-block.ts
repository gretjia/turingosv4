// TRACE_MATRIX FC1-N5: read view materialization — agent card block component
//
// <tos-agent-card-block> custom element. Renders AgentCardBlock IR payload
// as a styled <dl> card.
// XSS hygiene: uses textContent/setAttribute exclusively — never innerHTML
// with dynamic strings.

import type { AgentCardBlock } from '../ir.js';

const ELEMENT_NAME = 'tos-agent-card-block';

export class TosAgentCardBlock extends HTMLElement {
  private _block: AgentCardBlock | null = null;

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'agent_card');
    this.className = 'card agent-card';
    const payloadAttr = this.dataset['payload'];
    if (payloadAttr != null && this._block === null) {
      try {
        this._block = JSON.parse(payloadAttr) as AgentCardBlock;
      } catch {
        // Malformed payload — render nothing.
      }
    }
    this._render();
  }

  /** Update with a new AgentCardBlock payload. */
  update(block: AgentCardBlock): void {
    this._block = block;
    if (this.isConnected) {
      this._render();
    }
  }

  private _render(): void {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    const block = this._block;
    if (block === null) {
      return;
    }

    const dl = document.createElement('dl');

    addDlRow(dl, 'agent_id', block.agent_id);
    addDlRow(dl, 'role', block.role);
    addDlRow(dl, 'balance_micro', String(block.balance_micro) + ' μC');
    if (block.status != null) {
      addDlRow(dl, 'status', block.status);
    }

    this.appendChild(dl);
  }
}

/** Append a <dt>/<dd> pair to a <dl>. No innerHTML. */
function addDlRow(dl: HTMLDListElement, label: string, value: string): void {
  const dt = document.createElement('dt');
  dt.textContent = label;
  const dd = document.createElement('dd');
  dd.textContent = value;
  dl.appendChild(dt);
  dl.appendChild(dd);
}

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosAgentCardBlock);
  }
}
