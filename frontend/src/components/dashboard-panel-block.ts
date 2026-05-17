// TRACE_MATRIX FC1-N5: read view materialization — dashboard panel block
//
// <tos-dashboard-panel-block> custom element. Renders DashboardPanelBlock IR
// payload as a panel title in monospace caps + grid of <div><dt>…<dd> metric
// pairs (display font for the value, monospace caps for the label).
//
// Styled via the global [data-block-type="dashboard_panel"] selectors from
// base-styles.css.
//
// XSS hygiene: textContent/setAttribute only.

import type { DashboardPanelBlock, MetricEntry } from '../ir.js';
import { buildStatusBadge, isKnownStatus } from './render-helpers.js';

const ELEMENT_NAME = 'tos-dashboard-panel-block';

export class TosDashboardPanelBlock extends HTMLElement {
  private _block: DashboardPanelBlock | null = null;

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'dashboard_panel');
    this.classList.add('block', 'block-dashboard-panel', 'card', 'dashboard-panel');
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
    h3.className = 'panel-title';
    h3.textContent = block.panel_title;
    this.appendChild(h3);

    const dl = document.createElement('dl');
    dl.className = 'metrics';

    for (const metric of block.metrics) {
      dl.appendChild(buildMetricCell(metric));
    }

    this.appendChild(dl);
  }
}

function buildMetricCell(metric: MetricEntry): HTMLDivElement {
  const wrap = document.createElement('div');

  const dt = document.createElement('dt');
  dt.textContent = metric.label;
  wrap.appendChild(dt);

  const dd = document.createElement('dd');
  const valueStr = typeof metric.value === 'number' ? String(metric.value) : metric.value;

  if (typeof metric.value === 'string' && isKnownStatus(metric.value)) {
    dd.appendChild(buildStatusBadge(metric.value));
  } else {
    dd.appendChild(document.createTextNode(valueStr));
  }

  if (metric.unit != null) {
    dd.appendChild(document.createTextNode(' '));
    const unitSpan = document.createElement('span');
    unitSpan.className = 'unit';
    unitSpan.textContent = metric.unit;
    dd.appendChild(unitSpan);
  }
  wrap.appendChild(dd);

  return wrap;
}

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosDashboardPanelBlock);
  }
}
