// TRACE_MATRIX FC1-N10: write-path integration — task open form component
//
// <tos-task-open-form> custom element. Renders a form with three fields:
// problem_id, bounty, agent_id. On submit, validates client-side (mirrors
// server-side rules) then POSTs to /api/task/open. Shows status/error message.
//
// XSS hygiene: NEVER uses innerHTML with dynamic content; uses textContent
// and createElement exclusively.
//
// Sets data-block-type="task_open_form" on self (§6a pattern parity).

const ELEMENT_NAME = 'tos-task-open-form';

/** Mirror of server-side validation regex: ^[a-zA-Z0-9_-]{1,64}$ */
function isValidIdentifier(s: string): boolean {
  if (s.length === 0 || s.length > 64) return false;
  return /^[a-zA-Z0-9_-]+$/.test(s);
}

/** Mirror of server-side bounty cap: must be > 0 AND < 10_000_000 */
function isValidBounty(n: number): boolean {
  return Number.isInteger(n) && n > 0 && n < 10_000_000;
}

export class TosTaskOpenForm extends HTMLElement {
  private _form: HTMLFormElement | null = null;
  private _statusEl: HTMLParagraphElement | null = null;
  private _statusTimer: ReturnType<typeof setTimeout> | null = null;

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'task_open_form');
    this._render();
  }

  disconnectedCallback(): void {
    if (this._statusTimer !== null) {
      clearTimeout(this._statusTimer);
      this._statusTimer = null;
    }
  }

  private _render(): void {
    // Clear existing children.
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }

    const form = document.createElement('form');
    form.className = 'task-open-form';
    this._form = form;

    // problem_id field
    form.appendChild(makeField('problem_id', 'Problem ID', 'text', 'prob-001'));
    // bounty field
    form.appendChild(makeField('bounty', 'Bounty (μC)', 'number', '1000'));
    // agent_id field
    form.appendChild(makeField('agent_id', 'Agent ID', 'text', 'agent_0'));

    // Submit button
    const btn = document.createElement('button');
    btn.type = 'submit';
    btn.textContent = 'Open Task';
    form.appendChild(btn);

    // Status message placeholder (hidden until needed)
    const statusEl = document.createElement('p');
    statusEl.style.display = 'none';
    this._statusEl = statusEl;
    form.appendChild(statusEl);

    form.addEventListener('submit', (e: Event) => {
      e.preventDefault();
      void this._onSubmit();
    });

    this.appendChild(form);
  }

  private async _onSubmit(): Promise<void> {
    if (this._form === null) return;

    const problemIdInput = this._form.elements.namedItem('problem_id') as HTMLInputElement | null;
    const bountyInput = this._form.elements.namedItem('bounty') as HTMLInputElement | null;
    const agentIdInput = this._form.elements.namedItem('agent_id') as HTMLInputElement | null;

    if (problemIdInput === null || bountyInput === null || agentIdInput === null) return;

    const problemId = problemIdInput.value.trim();
    const bountyStr = bountyInput.value.trim();
    const agentId = agentIdInput.value.trim();
    const bounty = Number(bountyStr);

    // Client-side validation (mirrors server-side rules).
    if (!isValidIdentifier(problemId)) {
      this._showStatus(
        'error',
        'invalid_input',
        'problem_id must match ^[a-zA-Z0-9_-]{1,64}$'
      );
      return;
    }
    if (!isValidBounty(bounty)) {
      this._showStatus(
        'error',
        'invalid_input',
        'bounty must be an integer in (0, 10000000)'
      );
      return;
    }
    if (!isValidIdentifier(agentId)) {
      this._showStatus(
        'error',
        'invalid_input',
        'agent_id must match ^[a-zA-Z0-9_-]{1,64}$'
      );
      return;
    }

    // Disable the form while the request is in flight.
    const submitBtn = this._form.querySelector('button[type="submit"]') as HTMLButtonElement | null;
    if (submitBtn !== null) submitBtn.disabled = true;

    try {
      const resp = await fetch('/api/task/open', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ problem_id: problemId, bounty, agent_id: agentId }),
      });

      if (resp.ok) {
        // Parse task_id from response (best-effort; gracefully handle parse errors).
        let taskId = '';
        try {
          const data = await resp.json() as { task_id?: string };
          taskId = typeof data.task_id === 'string' ? data.task_id : '';
        } catch {
          taskId = '(unknown)';
        }
        // Clear form fields on success.
        problemIdInput.value = '';
        bountyInput.value = '';
        agentIdInput.value = '';
        this._showStatus('created', null, `Task created: ${taskId}`, 3000);
      } else {
        let kind = 'error';
        let reason = `HTTP ${resp.status}`;
        try {
          const data = await resp.json() as { kind?: string; reason?: string };
          if (typeof data.kind === 'string') kind = data.kind;
          if (typeof data.reason === 'string') reason = data.reason;
        } catch {
          // ignore parse errors; use defaults
        }
        this._showStatus('error', kind, reason);
      }
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : String(err);
      this._showStatus('error', 'network_error', `Network error: ${message}`);
    } finally {
      if (submitBtn !== null) submitBtn.disabled = false;
    }
  }

  /** Show a status/error message. Auto-hides after `autoHideMs` if provided. */
  private _showStatus(
    status: string,
    errorKind: string | null,
    message: string,
    autoHideMs?: number
  ): void {
    if (this._statusEl === null) return;

    // Clear any pending auto-hide timer.
    if (this._statusTimer !== null) {
      clearTimeout(this._statusTimer);
      this._statusTimer = null;
    }

    const el = this._statusEl;
    el.dataset['status'] = status;
    if (errorKind !== null) {
      el.dataset['errorKind'] = errorKind;
    } else {
      delete el.dataset['errorKind'];
    }
    // NEVER use innerHTML — textContent only (XSS hygiene).
    el.textContent = message;
    el.style.display = '';

    if (autoHideMs !== undefined && autoHideMs > 0) {
      this._statusTimer = setTimeout(() => {
        el.style.display = 'none';
        el.textContent = '';
        this._statusTimer = null;
      }, autoHideMs);
    }
  }
}

/** Build a <label><input> pair wrapped in a <div>. */
function makeField(
  name: string,
  label: string,
  type: string,
  placeholder: string
): HTMLDivElement {
  const div = document.createElement('div');
  div.className = 'field';

  const lbl = document.createElement('label');
  lbl.htmlFor = `tos-tof-${name}`;
  lbl.textContent = label;

  const input = document.createElement('input');
  input.type = type;
  input.name = name;
  input.id = `tos-tof-${name}`;
  input.placeholder = placeholder;

  div.appendChild(lbl);
  div.appendChild(input);
  return div;
}

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosTaskOpenForm);
  }
}
