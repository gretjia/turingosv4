// TRACE_MATRIX FC1-N5: read view materialization — event log block component
//
// <tos-event-log-block> custom element. Renders EventLogBlock IR payload
// as <ol class="event-log" reversed> with each entry styled by layer.
// Styled via the global [data-block-type="event_log"] selectors from
// base-styles.css.
//
// XSS hygiene: textContent/setAttribute only.

import type { EventLogBlock, EventEntry } from '../ir.js';
import { buildTruncatedSpan } from './render-helpers.js';

const ELEMENT_NAME = 'tos-event-log-block';

export class TosEventLogBlock extends HTMLElement {
  private _block: EventLogBlock | null = null;

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'event_log');
    this.classList.add('block', 'block-event-log');
    this.setAttribute('aria-label', 'recent tape events');
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

    const ol = document.createElement('ol');
    ol.className = 'event-log';
    ol.setAttribute('reversed', '');

    for (const ev of block.events) {
      ol.appendChild(buildEventItem(ev));
    }

    this.appendChild(ol);
  }
}

function buildEventItem(ev: EventEntry): HTMLLIElement {
  const li = document.createElement('li');
  li.className = 'event layer-' + ev.layer;

  const layerSpan = document.createElement('span');
  layerSpan.className = 'layer';
  layerSpan.textContent = ev.layer;
  li.appendChild(layerSpan);

  const kindSpan = document.createElement('span');
  kindSpan.className = 'kind';
  kindSpan.textContent = ev.kind;
  li.appendChild(kindSpan);

  li.appendChild(buildTruncatedSpan(ev.tx_id, 10, 6, 'tx-id'));

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
