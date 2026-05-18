// TRACE_MATRIX FC1-N5: read view materialization — agent card block component
//
// <tos-agent-card-block> custom element. Renders AgentCardBlock IR payload
// as <header><span class="tos-card-id"><span class="tos-card-role"></header>
// plus <dl> of attributes. Styled via the global
// [data-block-type="agent_card"] selectors from base-styles.css.
//
// XSS hygiene: textContent/setAttribute only.

import type { AgentCardBlock } from '../ir.js';
import { appendMicrocoin, buildStatusBadge, buildTruncatedSpan } from './render-helpers.js';

const ELEMENT_NAME = 'tos-agent-card-block';

export class TosAgentCardBlock extends HTMLElement {
  private _block: AgentCardBlock | null = null;

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'agent_card');
    this.classList.add('block', 'block-agent-card', 'card', 'agent-card');
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

    const header = document.createElement('header');
    header.appendChild(buildTruncatedSpan(block.agent_id, 12, 8, 'tos-card-id'));
    const role = document.createElement('span');
    role.className = 'tos-card-role';
    role.textContent = block.role;
    header.appendChild(role);
    this.appendChild(header);

    const dl = document.createElement('dl');
    const dtBal = document.createElement('dt');
    dtBal.textContent = 'balance';
    const ddBal = document.createElement('dd');
    appendMicrocoin(ddBal, block.balance_micro);
    dl.appendChild(dtBal);
    dl.appendChild(ddBal);

    if (block.status != null) {
      const dtSt = document.createElement('dt');
      dtSt.textContent = 'status';
      const ddSt = document.createElement('dd');
      ddSt.appendChild(buildStatusBadge(block.status));
      dl.appendChild(dtSt);
      dl.appendChild(ddSt);
    }

    this.appendChild(dl);
  }
}

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosAgentCardBlock);
  }
}
