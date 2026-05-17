// TRACE_MATRIX FC1-N5: read view materialization — task card block component
//
// <tos-task-card-block> custom element. Renders TaskCardBlock IR payload
// as a styled <dl> card with a status badge.
// XSS hygiene: uses textContent/setAttribute exclusively — never innerHTML
// with dynamic strings.

import type { TaskCardBlock } from '../ir.js';

const ELEMENT_NAME = 'tos-task-card-block';

export class TosTaskCardBlock extends HTMLElement {
  private _block: TaskCardBlock | null = null;

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'task_card');
    this.className = 'card task-card';
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

  /** Update with a new TaskCardBlock payload. */
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

    const dl = document.createElement('dl');

    addDlRow(dl, 'task_id', block.task_id);
    addDlRow(dl, 'problem_id', block.problem_id);

    // Status badge
    const dtStatus = document.createElement('dt');
    dtStatus.textContent = 'status';
    const ddStatus = document.createElement('dd');
    ddStatus.className = 'status';
    ddStatus.textContent = block.status;
    dl.appendChild(dtStatus);
    dl.appendChild(ddStatus);

    if (block.reward_micro != null) {
      addDlRow(dl, 'reward_micro', String(block.reward_micro) + ' μC');
    }
    if (block.attempt_count != null) {
      addDlRow(dl, 'attempt_count', String(block.attempt_count));
    }
    if (block.assigned_agent_id != null) {
      addDlRow(dl, 'assigned_agent_id', block.assigned_agent_id);
    }

    this.appendChild(dl);
  }
}

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
    customElements.define(ELEMENT_NAME, TosTaskCardBlock);
  }
}
