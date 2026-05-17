// TRACE_MATRIX FC1-N5: read view materialization — dashboard panel block component
//
// <tos-dashboard-panel-block> custom element. Renders DashboardPanelBlock IR
// payload as a titled <dl> metric card.
// XSS hygiene: uses textContent/setAttribute exclusively — never innerHTML
// with dynamic strings.

import type { DashboardPanelBlock, MetricEntry } from '../ir.js';

const ELEMENT_NAME = 'tos-dashboard-panel-block';

export class TosDashboardPanelBlock extends HTMLElement {
  private _block: DashboardPanelBlock | null = null;

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'dashboard_panel');
    this.className = 'card dashboard-panel';
    const payloadAttr = this.dataset['payload'];
    if (payloadAttr != null && this._block === null) {
      try {
        this._block = JSON.parse(payloadAttr) as DashboardPanelBlock;
      } catch {
        // Malformed payload — render nothing.
      }
    }
    this._render();
  }

  /** Update with a new DashboardPanelBlock payload. */
  update(block: DashboardPanelBlock): void {
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

    const h3 = document.createElement('h3');
    h3.textContent = block.panel_title;
    this.appendChild(h3);

    const dl = document.createElement('dl');
    dl.className = 'metrics';

    for (const metric of block.metrics) {
      buildMetricRow(dl, metric);
    }

    this.appendChild(dl);
  }
}

function buildMetricRow(dl: HTMLDListElement, metric: MetricEntry): void {
  const dt = document.createElement('dt');
  dt.textContent = metric.label;

  const dd = document.createElement('dd');
  const valueStr = typeof metric.value === 'number'
    ? String(metric.value)
    : metric.value;
  dd.textContent = valueStr;

  if (metric.unit != null) {
    // Append unit as a separate text span — textContent safe.
    dd.textContent = valueStr + ' ';
    const unitSpan = document.createElement('span');
    unitSpan.className = 'unit';
    unitSpan.textContent = metric.unit;
    dd.appendChild(unitSpan);
  }

  dl.appendChild(dt);
  dl.appendChild(dd);
}

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosDashboardPanelBlock);
  }
}
