// TRACE_MATRIX FC1-N5: read view materialization — shared rendering helpers
//
// Pure helpers shared across block-type Web Components. XSS hygiene:
// every function returns either a string (for textContent) or builds
// elements via document.createElement (never innerHTML with dynamic data).

/** Status strings recognised by the design system as semantic badges. */
const KNOWN_STATUSES = new Set([
  'open',
  'accepted',
  'rejected',
  'finalized',
  'bankrupt',
  'expired',
  'solved',
  'exhausted',
  'active',
  'paused',
  'pass',
  'fail',
]);

export function isKnownStatus(s: string): boolean {
  return KNOWN_STATUSES.has(s.trim().toLowerCase());
}

/** Truncate-middle for long hash/identifier display. */
export function truncateMiddle(s: string, head: number, tail: number): string {
  if (s.length <= head + tail + 1) return s;
  return s.slice(0, head) + '…' + s.slice(s.length - tail);
}

/** Lowercase + slugify a status for use as a data-status attribute. */
export function statusSlug(s: string): string {
  return s
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '_');
}

/** Build a typographic status badge element. */
export function buildStatusBadge(status: string): HTMLSpanElement {
  const span = document.createElement('span');
  span.className = 'tos-status';
  span.dataset['status'] = statusSlug(status);
  span.textContent = status;
  return span;
}

/** Build a span that shows truncated-middle text with the full value in title. */
export function buildTruncatedSpan(
  value: string,
  head = 12,
  tail = 8,
  className?: string,
): HTMLSpanElement {
  const span = document.createElement('span');
  if (className !== undefined) span.className = className;
  const trunc = truncateMiddle(value, head, tail);
  if (trunc !== value) {
    span.title = value;
  }
  span.textContent = trunc;
  return span;
}

/** Build a μC value cell: numeric + small unit span. */
export function appendMicrocoin(parent: HTMLElement, micro: number | string): void {
  parent.appendChild(document.createTextNode(String(micro) + ' '));
  const u = document.createElement('span');
  u.className = 'unit';
  u.textContent = 'μC';
  parent.appendChild(u);
}
