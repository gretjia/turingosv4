// TRACE_MATRIX FC1-N5: read view materialization — event log block component
//
// <tos-event-log-block> custom element. Renders EventLogBlock IR payload
// as an ordered <ul> of tape events.
// XSS hygiene: uses textContent/setAttribute exclusively — never innerHTML
// with dynamic strings.

import type { EventLogBlock, EventEntry } from '../ir.js';

const ELEMENT_NAME = 'tos-event-log-block';

export class TosEventLogBlock extends HTMLElement {
  private _block: EventLogBlock | null = null;

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'event_log');
    const payloadAttr = this.dataset['payload'];
    if (payloadAttr != null && this._block === null) {
      try {
        this._block = JSON.parse(payloadAttr) as EventLogBlock;
      } catch {
        // Malformed payload — render nothing.
      }
    }
    this._render();
  }

  /** Update with a new EventLogBlock payload. */
  update(block: EventLogBlock): void {
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

    const ul = document.createElement('ul');
    ul.className = 'event-log';

    for (const ev of block.events) {
      ul.appendChild(buildEventItem(ev));
    }

    this.appendChild(ul);
  }
}

function buildEventItem(ev: EventEntry): HTMLLIElement {
  const li = document.createElement('li');
  li.className = 'event layer-' + ev.layer;

  const layerSpan = document.createElement('span');
  layerSpan.className = 'layer';
  layerSpan.textContent = ev.layer;
  li.appendChild(layerSpan);

  li.appendChild(document.createTextNode(' '));

  const kindSpan = document.createElement('span');
  kindSpan.className = 'kind';
  kindSpan.textContent = ev.kind;
  li.appendChild(kindSpan);

  li.appendChild(document.createTextNode(' '));

  const txSpan = document.createElement('span');
  txSpan.className = 'tx-id';
  txSpan.textContent = ev.tx_id;
  li.appendChild(txSpan);

  if (ev.summary != null) {
    const summarySpan = document.createElement('span');
    summarySpan.className = 'summary';
    summarySpan.textContent = ev.summary;
    li.appendChild(summarySpan);
  }

  return li;
}

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosEventLogBlock);
  }
}
