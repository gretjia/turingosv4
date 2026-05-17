// TRACE_MATRIX FC1-N5: read view materialization — table block component
//
// <tos-table-block> custom element. Renders TableBlock IR payload as
// <figure><figcaption><table><thead><th scope="col"><tbody><td data-cell-kind>
// — semantic table structure. Light DOM, styled via the global
// [data-block-type="table"] selectors from base-styles.css.
//
// XSS hygiene: uses textContent/setAttribute exclusively — never innerHTML
// with dynamic strings.

import type { TableBlock, Cell } from '../ir.js';
import {
  appendMicrocoin,
  buildStatusBadge,
  buildTruncatedSpan,
  isKnownStatus,
} from './render-helpers.js';

const ELEMENT_NAME = 'tos-table-block';

export class TosTableBlock extends HTMLElement {
  private _block: TableBlock | null = null;

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'table');
    this.classList.add('block', 'block-table');
    const payloadAttr = this.dataset['payload'];
    if (payloadAttr != null && this._block === null) {
      try {
        this._block = JSON.parse(payloadAttr) as TableBlock;
      } catch {
        // Malformed payload — render nothing.
      }
    }
    this._render();
  }

  /** Update with a new TableBlock payload. */
  update(block: TableBlock): void {
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

    // <figure> root — host element already has data-block-type="table",
    // so we render contents directly into the host.
    if (block.caption != null) {
      const cap = document.createElement('figcaption');
      cap.className = 'caption';
      cap.textContent = block.caption;
      this.appendChild(cap);
    }

    const table = document.createElement('table');
    const thead = document.createElement('thead');
    const headerRow = document.createElement('tr');
    for (const col of block.columns) {
      const th = document.createElement('th');
      th.setAttribute('scope', 'col');
      th.textContent = col;
      headerRow.appendChild(th);
    }
    thead.appendChild(headerRow);
    table.appendChild(thead);

    const tbody = document.createElement('tbody');
    for (const row of block.rows) {
      const tr = document.createElement('tr');
      for (const cell of row) {
        const td = document.createElement('td');
        td.dataset['cellKind'] = cell.kind;
        appendCellContent(td, cell);
        tr.appendChild(td);
      }
      tbody.appendChild(tr);
    }
    table.appendChild(tbody);

    this.appendChild(table);
  }
}

/** Render a Cell's content into a <td>, picking semantic markup by kind. */
function appendCellContent(td: HTMLTableCellElement, cell: Cell): void {
  const v = cell.value;
  if (cell.kind === 'microcoin') {
    appendMicrocoin(td, v);
    return;
  }
  if (cell.kind === 'agent_id' || cell.kind === 'tx_id' || cell.kind === 'cid') {
    if (typeof v === 'string') {
      td.appendChild(buildTruncatedSpan(v, 14, 8));
    } else {
      td.textContent = String(v);
    }
    return;
  }
  if (cell.kind === 'string' && typeof v === 'string' && isKnownStatus(v)) {
    td.appendChild(buildStatusBadge(v));
    return;
  }
  td.textContent = typeof v === 'number' ? String(v) : v;
}

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosTableBlock);
  }
}
