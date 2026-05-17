// TRACE_MATRIX FC1-N5: read view materialization — table block component
//
// <tos-table-block> custom element. Renders TableBlock IR payload as
// <table><thead>/<tbody> structure.
// XSS hygiene: uses textContent/setAttribute exclusively — never innerHTML
// with dynamic strings.

import type { TableBlock, Cell } from '../ir.js';

const ELEMENT_NAME = 'tos-table-block';

export class TosTableBlock extends HTMLElement {
  private _block: TableBlock | null = null;

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'table');
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

    // Optional caption
    if (block.caption != null) {
      const p = document.createElement('p');
      p.className = 'caption';
      p.textContent = block.caption;
      this.appendChild(p);
    }

    // Build table
    const table = document.createElement('table');

    // thead
    const thead = document.createElement('thead');
    const headerRow = document.createElement('tr');
    for (const col of block.columns) {
      const th = document.createElement('th');
      th.textContent = col;
      headerRow.appendChild(th);
    }
    thead.appendChild(headerRow);
    table.appendChild(thead);

    // tbody
    const tbody = document.createElement('tbody');
    for (const row of block.rows) {
      const tr = document.createElement('tr');
      for (const cell of row) {
        const td = document.createElement('td');
        td.textContent = formatCell(cell);
        tr.appendChild(td);
      }
      tbody.appendChild(tr);
    }
    table.appendChild(tbody);

    this.appendChild(table);
  }
}

/** Format a cell value as a display string. */
function formatCell(cell: Cell): string {
  const val = typeof cell.value === 'number' ? String(cell.value) : cell.value;
  if (cell.kind === 'microcoin') {
    return val + ' μC'; // μC
  }
  return val;
}

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosTableBlock);
  }
}
