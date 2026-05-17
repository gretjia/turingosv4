// TRACE_MATRIX FC1-N5: read view materialization — task card block component
//
// <tos-task-card-block> custom element. Renders TaskCardBlock IR payload
// as <header>(id + status badge)<dl>(problem, reward, attempts, agent).
// Styled via the global [data-block-type="task_card"] selectors from
// base-styles.css.
//
// XSS hygiene: textContent/setAttribute only.

import type { TaskCardBlock } from '../ir.js';
import { appendMicrocoin, buildStatusBadge, buildTruncatedSpan } from './render-helpers.js';

const ELEMENT_NAME = 'tos-task-card-block';

export class TosTaskCardBlock extends HTMLElement {
  private _block: TaskCardBlock | null = null;

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'task_card');
    this.classList.add('block', 'block-task-card', 'card', 'task-card');
    const payloadAttr = this.dataset['payload'];
    if (payloadAttr != null && this._block === null) {
      try {
        this._block = JSON.parse(payloadAttr) as TaskCardBlock;
      } catch {
        // Malformed payload — render nothing.
      }
    }
    this._render();
  }

  update(block: TaskCardBlock): void {
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
    header.appendChild(buildTruncatedSpan(block.task_id, 12, 8, 'tos-card-id'));
    header.appendChild(buildStatusBadge(block.status));
    this.appendChild(header);

    const dl = document.createElement('dl');

    const dtProb = document.createElement('dt');
    dtProb.textContent = 'problem';
    const ddProb = document.createElement('dd');
    ddProb.textContent = block.problem_id;
    dl.appendChild(dtProb);
    dl.appendChild(ddProb);

    if (block.reward_micro != null) {
      const dt = document.createElement('dt');
      dt.textContent = 'reward';
      const dd = document.createElement('dd');
      appendMicrocoin(dd, block.reward_micro);
      dl.appendChild(dt);
      dl.appendChild(dd);
    }
    if (block.attempt_count != null) {
      const dt = document.createElement('dt');
      dt.textContent = 'attempts';
      const dd = document.createElement('dd');
      dd.textContent = String(block.attempt_count);
      dl.appendChild(dt);
      dl.appendChild(dd);
    }
    if (block.assigned_agent_id != null) {
      const dt = document.createElement('dt');
      dt.textContent = 'agent';
      const dd = document.createElement('dd');
      dd.appendChild(buildTruncatedSpan(block.assigned_agent_id, 12, 8));
      dl.appendChild(dt);
      dl.appendChild(dd);
    }

    this.appendChild(dl);
  }
}

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosTaskCardBlock);
  }
}
