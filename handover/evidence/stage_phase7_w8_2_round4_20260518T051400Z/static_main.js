// src/components/text-block.ts
var ELEMENT_NAME = "tos-text-block";
var TosTextBlock = class extends HTMLElement {
  constructor() {
    super(...arguments);
    this._block = null;
  }
  connectedCallback() {
    this.setAttribute("data-block-type", "text");
    this.classList.add("block", "block-text");
    const payloadAttr = this.dataset["payload"];
    if (payloadAttr != null && this._block === null) {
      try {
        this._block = JSON.parse(payloadAttr);
      } catch {
      }
    }
    this._render();
  }
  /** Update with a new TextBlock payload (for incremental updates from turingos-root). */
  update(block) {
    this._block = block;
    if (this.isConnected) {
      this._render();
    }
  }
  _render() {
    const block = this._block;
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    if (block === null) {
      return;
    }
    const lines = block.content.split("\n");
    for (const line of lines) {
      if (line.length === 0) continue;
      const p = document.createElement("p");
      p.textContent = line;
      this.appendChild(p);
    }
  }
};
function register() {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosTextBlock);
  }
}

// src/components/render-helpers.ts
var KNOWN_STATUSES = /* @__PURE__ */ new Set([
  "open",
  "accepted",
  "rejected",
  "finalized",
  "bankrupt",
  "expired",
  "solved",
  "exhausted",
  "active",
  "paused",
  "pass",
  "fail"
]);
function isKnownStatus(s) {
  return KNOWN_STATUSES.has(s.trim().toLowerCase());
}
function truncateMiddle(s, head, tail) {
  if (s.length <= head + tail + 1) return s;
  return s.slice(0, head) + "\u2026" + s.slice(s.length - tail);
}
function statusSlug(s) {
  return s.trim().toLowerCase().replace(/[^a-z0-9]+/g, "_");
}
function buildStatusBadge(status) {
  const span = document.createElement("span");
  span.className = "tos-status";
  span.dataset["status"] = statusSlug(status);
  span.textContent = status;
  return span;
}
function buildTruncatedSpan(value, head = 12, tail = 8, className) {
  const span = document.createElement("span");
  if (className !== void 0) span.className = className;
  const trunc = truncateMiddle(value, head, tail);
  if (trunc !== value) {
    span.title = value;
  }
  span.textContent = trunc;
  return span;
}
function appendMicrocoin(parent, micro) {
  parent.appendChild(document.createTextNode(String(micro) + " "));
  const u = document.createElement("span");
  u.className = "unit";
  u.textContent = "\u03BCC";
  parent.appendChild(u);
}

// src/components/table-block.ts
var ELEMENT_NAME2 = "tos-table-block";
var TosTableBlock = class extends HTMLElement {
  constructor() {
    super(...arguments);
    this._block = null;
  }
  connectedCallback() {
    this.setAttribute("data-block-type", "table");
    this.classList.add("block", "block-table");
    const payloadAttr = this.dataset["payload"];
    if (payloadAttr != null && this._block === null) {
      try {
        this._block = JSON.parse(payloadAttr);
      } catch {
      }
    }
    this._render();
  }
  /** Update with a new TableBlock payload. */
  update(block) {
    this._block = block;
    if (this.isConnected) {
      this._render();
    }
  }
  _render() {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    const block = this._block;
    if (block === null) {
      return;
    }
    if (block.caption != null) {
      const cap = document.createElement("figcaption");
      cap.className = "caption";
      cap.textContent = block.caption;
      this.appendChild(cap);
    }
    const table = document.createElement("table");
    const thead = document.createElement("thead");
    const headerRow = document.createElement("tr");
    for (const col of block.columns) {
      const th = document.createElement("th");
      th.setAttribute("scope", "col");
      th.textContent = col;
      headerRow.appendChild(th);
    }
    thead.appendChild(headerRow);
    table.appendChild(thead);
    const tbody = document.createElement("tbody");
    for (const row of block.rows) {
      const tr = document.createElement("tr");
      for (const cell of row) {
        const td = document.createElement("td");
        td.dataset["cellKind"] = cell.kind;
        appendCellContent(td, cell);
        tr.appendChild(td);
      }
      tbody.appendChild(tr);
    }
    table.appendChild(tbody);
    this.appendChild(table);
  }
};
function appendCellContent(td, cell) {
  const v = cell.value;
  if (cell.kind === "microcoin") {
    appendMicrocoin(td, v);
    return;
  }
  if (cell.kind === "agent_id" || cell.kind === "tx_id" || cell.kind === "cid") {
    if (typeof v === "string") {
      td.appendChild(buildTruncatedSpan(v, 14, 8));
    } else {
      td.textContent = String(v);
    }
    return;
  }
  if (cell.kind === "string" && typeof v === "string" && isKnownStatus(v)) {
    td.appendChild(buildStatusBadge(v));
    return;
  }
  td.textContent = typeof v === "number" ? String(v) : v;
}
function register2() {
  if (!customElements.get(ELEMENT_NAME2)) {
    customElements.define(ELEMENT_NAME2, TosTableBlock);
  }
}

// src/components/agent-card-block.ts
var ELEMENT_NAME3 = "tos-agent-card-block";
var TosAgentCardBlock = class extends HTMLElement {
  constructor() {
    super(...arguments);
    this._block = null;
  }
  connectedCallback() {
    this.setAttribute("data-block-type", "agent_card");
    this.classList.add("block", "block-agent-card", "card", "agent-card");
    const payloadAttr = this.dataset["payload"];
    if (payloadAttr != null && this._block === null) {
      try {
        this._block = JSON.parse(payloadAttr);
      } catch {
      }
    }
    this._render();
  }
  update(block) {
    this._block = block;
    if (this.isConnected) {
      this._render();
    }
  }
  _render() {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    const block = this._block;
    if (block === null) {
      return;
    }
    const header = document.createElement("header");
    header.appendChild(buildTruncatedSpan(block.agent_id, 12, 8, "tos-card-id"));
    const role = document.createElement("span");
    role.className = "tos-card-role";
    role.textContent = block.role;
    header.appendChild(role);
    this.appendChild(header);
    const dl = document.createElement("dl");
    const dtBal = document.createElement("dt");
    dtBal.textContent = "balance";
    const ddBal = document.createElement("dd");
    appendMicrocoin(ddBal, block.balance_micro);
    dl.appendChild(dtBal);
    dl.appendChild(ddBal);
    if (block.status != null) {
      const dtSt = document.createElement("dt");
      dtSt.textContent = "status";
      const ddSt = document.createElement("dd");
      ddSt.appendChild(buildStatusBadge(block.status));
      dl.appendChild(dtSt);
      dl.appendChild(ddSt);
    }
    this.appendChild(dl);
  }
};
function register3() {
  if (!customElements.get(ELEMENT_NAME3)) {
    customElements.define(ELEMENT_NAME3, TosAgentCardBlock);
  }
}

// src/components/task-card-block.ts
var ELEMENT_NAME4 = "tos-task-card-block";
var TosTaskCardBlock = class extends HTMLElement {
  constructor() {
    super(...arguments);
    this._block = null;
  }
  connectedCallback() {
    this.setAttribute("data-block-type", "task_card");
    this.classList.add("block", "block-task-card", "card", "task-card");
    const payloadAttr = this.dataset["payload"];
    if (payloadAttr != null && this._block === null) {
      try {
        this._block = JSON.parse(payloadAttr);
      } catch {
      }
    }
    this._render();
  }
  update(block) {
    this._block = block;
    if (this.isConnected) {
      this._render();
    }
  }
  _render() {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    const block = this._block;
    if (block === null) {
      return;
    }
    const header = document.createElement("header");
    header.appendChild(buildTruncatedSpan(block.task_id, 12, 8, "tos-card-id"));
    header.appendChild(buildStatusBadge(block.status));
    this.appendChild(header);
    const dl = document.createElement("dl");
    const dtProb = document.createElement("dt");
    dtProb.textContent = "problem";
    const ddProb = document.createElement("dd");
    ddProb.textContent = block.problem_id;
    dl.appendChild(dtProb);
    dl.appendChild(ddProb);
    if (block.reward_micro != null) {
      const dt = document.createElement("dt");
      dt.textContent = "reward";
      const dd = document.createElement("dd");
      appendMicrocoin(dd, block.reward_micro);
      dl.appendChild(dt);
      dl.appendChild(dd);
    }
    if (block.attempt_count != null) {
      const dt = document.createElement("dt");
      dt.textContent = "attempts";
      const dd = document.createElement("dd");
      dd.textContent = String(block.attempt_count);
      dl.appendChild(dt);
      dl.appendChild(dd);
    }
    if (block.assigned_agent_id != null) {
      const dt = document.createElement("dt");
      dt.textContent = "agent";
      const dd = document.createElement("dd");
      dd.appendChild(buildTruncatedSpan(block.assigned_agent_id, 12, 8));
      dl.appendChild(dt);
      dl.appendChild(dd);
    }
    this.appendChild(dl);
  }
};
function register4() {
  if (!customElements.get(ELEMENT_NAME4)) {
    customElements.define(ELEMENT_NAME4, TosTaskCardBlock);
  }
}

// src/components/event-log-block.ts
var ELEMENT_NAME5 = "tos-event-log-block";
var TosEventLogBlock = class extends HTMLElement {
  constructor() {
    super(...arguments);
    this._block = null;
  }
  connectedCallback() {
    this.setAttribute("data-block-type", "event_log");
    this.classList.add("block", "block-event-log");
    this.setAttribute("aria-label", "recent tape events");
    const payloadAttr = this.dataset["payload"];
    if (payloadAttr != null && this._block === null) {
      try {
        this._block = JSON.parse(payloadAttr);
      } catch {
      }
    }
    this._render();
  }
  update(block) {
    this._block = block;
    if (this.isConnected) {
      this._render();
    }
  }
  _render() {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    const block = this._block;
    if (block === null) {
      return;
    }
    const ol = document.createElement("ol");
    ol.className = "event-log";
    ol.setAttribute("reversed", "");
    for (const ev of block.events) {
      ol.appendChild(buildEventItem(ev));
    }
    this.appendChild(ol);
  }
};
function buildEventItem(ev) {
  const li = document.createElement("li");
  li.className = "event layer-" + ev.layer;
  const layerSpan = document.createElement("span");
  layerSpan.className = "layer";
  layerSpan.textContent = ev.layer;
  li.appendChild(layerSpan);
  const kindSpan = document.createElement("span");
  kindSpan.className = "kind";
  kindSpan.textContent = ev.kind;
  li.appendChild(kindSpan);
  li.appendChild(buildTruncatedSpan(ev.tx_id, 10, 6, "tx-id"));
  if (ev.summary != null) {
    const summarySpan = document.createElement("span");
    summarySpan.className = "summary";
    summarySpan.textContent = ev.summary;
    li.appendChild(summarySpan);
  }
  return li;
}
function register5() {
  if (!customElements.get(ELEMENT_NAME5)) {
    customElements.define(ELEMENT_NAME5, TosEventLogBlock);
  }
}

// src/components/dashboard-panel-block.ts
var ELEMENT_NAME6 = "tos-dashboard-panel-block";
var TosDashboardPanelBlock = class extends HTMLElement {
  constructor() {
    super(...arguments);
    this._block = null;
  }
  connectedCallback() {
    this.setAttribute("data-block-type", "dashboard_panel");
    this.classList.add("block", "block-dashboard-panel", "card", "dashboard-panel");
    const payloadAttr = this.dataset["payload"];
    if (payloadAttr != null && this._block === null) {
      try {
        this._block = JSON.parse(payloadAttr);
      } catch {
      }
    }
    this._render();
  }
  update(block) {
    this._block = block;
    if (this.isConnected) {
      this._render();
    }
  }
  _render() {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    const block = this._block;
    if (block === null) {
      return;
    }
    const h3 = document.createElement("h3");
    h3.className = "panel-title";
    h3.textContent = block.panel_title;
    this.appendChild(h3);
    const dl = document.createElement("dl");
    dl.className = "metrics";
    for (const metric of block.metrics) {
      dl.appendChild(buildMetricCell(metric));
    }
    this.appendChild(dl);
  }
};
function buildMetricCell(metric) {
  const wrap = document.createElement("div");
  const dt = document.createElement("dt");
  dt.textContent = metric.label;
  wrap.appendChild(dt);
  const dd = document.createElement("dd");
  const valueStr = typeof metric.value === "number" ? String(metric.value) : metric.value;
  if (typeof metric.value === "string" && isKnownStatus(metric.value)) {
    dd.appendChild(buildStatusBadge(metric.value));
  } else {
    dd.appendChild(document.createTextNode(valueStr));
  }
  if (metric.unit != null) {
    dd.appendChild(document.createTextNode(" "));
    const unitSpan = document.createElement("span");
    unitSpan.className = "unit";
    unitSpan.textContent = metric.unit;
    dd.appendChild(unitSpan);
  }
  wrap.appendChild(dd);
  return wrap;
}
function register6() {
  if (!customElements.get(ELEMENT_NAME6)) {
    customElements.define(ELEMENT_NAME6, TosDashboardPanelBlock);
  }
}

// src/components/task-open-form.ts
var ELEMENT_NAME7 = "tos-task-open-form";
function isValidIdentifier(s) {
  if (s.length === 0 || s.length > 64) return false;
  return /^[a-zA-Z0-9_-]+$/.test(s);
}
function isValidBounty(n) {
  return Number.isInteger(n) && n > 0 && n < 1e7;
}
var TosTaskOpenForm = class extends HTMLElement {
  constructor() {
    super(...arguments);
    this._form = null;
    this._statusEl = null;
    this._statusTimer = null;
  }
  connectedCallback() {
    this.setAttribute("data-block-type", "task_open_form");
    this._render();
  }
  disconnectedCallback() {
    if (this._statusTimer !== null) {
      clearTimeout(this._statusTimer);
      this._statusTimer = null;
    }
  }
  _render() {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    const form = document.createElement("form");
    form.className = "task-open-form";
    this._form = form;
    form.appendChild(makeField("problem_id", "Problem ID", "text", "prob-001"));
    form.appendChild(makeField("bounty", "Bounty (\u03BCC)", "number", "1000"));
    form.appendChild(makeField("agent_id", "Agent ID", "text", "agent_0"));
    const btn = document.createElement("button");
    btn.type = "submit";
    btn.textContent = "Open Task";
    form.appendChild(btn);
    const statusEl = document.createElement("p");
    statusEl.style.display = "none";
    this._statusEl = statusEl;
    form.appendChild(statusEl);
    form.addEventListener("submit", (e) => {
      e.preventDefault();
      void this._onSubmit();
    });
    this.appendChild(form);
  }
  async _onSubmit() {
    if (this._form === null) return;
    const problemIdInput = this._form.elements.namedItem("problem_id");
    const bountyInput = this._form.elements.namedItem("bounty");
    const agentIdInput = this._form.elements.namedItem("agent_id");
    if (problemIdInput === null || bountyInput === null || agentIdInput === null) return;
    const problemId = problemIdInput.value.trim();
    const bountyStr = bountyInput.value.trim();
    const agentId = agentIdInput.value.trim();
    const bounty = Number(bountyStr);
    if (!isValidIdentifier(problemId)) {
      this._showStatus(
        "error",
        "invalid_input",
        "problem_id must match ^[a-zA-Z0-9_-]{1,64}$"
      );
      return;
    }
    if (!isValidBounty(bounty)) {
      this._showStatus(
        "error",
        "invalid_input",
        "bounty must be an integer in (0, 10000000)"
      );
      return;
    }
    if (!isValidIdentifier(agentId)) {
      this._showStatus(
        "error",
        "invalid_input",
        "agent_id must match ^[a-zA-Z0-9_-]{1,64}$"
      );
      return;
    }
    const submitBtn = this._form.querySelector('button[type="submit"]');
    if (submitBtn !== null) submitBtn.disabled = true;
    try {
      const resp = await fetch("/api/task/open", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ problem_id: problemId, bounty, agent_id: agentId })
      });
      if (resp.ok) {
        let taskId = "";
        try {
          const data = await resp.json();
          taskId = typeof data.task_id === "string" ? data.task_id : "";
        } catch {
          taskId = "(unknown)";
        }
        problemIdInput.value = "";
        bountyInput.value = "";
        agentIdInput.value = "";
        this._showStatus("created", null, `Task created: ${taskId}`, 3e3);
      } else {
        let kind = "error";
        let reason = `HTTP ${resp.status}`;
        try {
          const data = await resp.json();
          if (typeof data.kind === "string") kind = data.kind;
          if (typeof data.reason === "string") reason = data.reason;
        } catch {
        }
        this._showStatus("error", kind, reason);
      }
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      this._showStatus("error", "network_error", `Network error: ${message}`);
    } finally {
      if (submitBtn !== null) submitBtn.disabled = false;
    }
  }
  /** Show a status/error message. Auto-hides after `autoHideMs` if provided. */
  _showStatus(status, errorKind, message, autoHideMs) {
    if (this._statusEl === null) return;
    if (this._statusTimer !== null) {
      clearTimeout(this._statusTimer);
      this._statusTimer = null;
    }
    const el = this._statusEl;
    el.dataset["status"] = status;
    if (errorKind !== null) {
      el.dataset["errorKind"] = errorKind;
    } else {
      delete el.dataset["errorKind"];
    }
    el.textContent = message;
    el.style.display = "";
    if (autoHideMs !== void 0 && autoHideMs > 0) {
      this._statusTimer = setTimeout(() => {
        el.style.display = "none";
        el.textContent = "";
        this._statusTimer = null;
      }, autoHideMs);
    }
  }
};
function makeField(name, label, type, placeholder) {
  const div = document.createElement("div");
  div.className = "field";
  const lbl = document.createElement("label");
  lbl.htmlFor = `tos-tof-${name}`;
  lbl.textContent = label;
  const input = document.createElement("input");
  input.type = type;
  input.name = name;
  input.id = `tos-tof-${name}`;
  input.placeholder = placeholder;
  div.appendChild(lbl);
  div.appendChild(input);
  return div;
}
function register7() {
  if (!customElements.get(ELEMENT_NAME7)) {
    customElements.define(ELEMENT_NAME7, TosTaskOpenForm);
  }
}

// src/components/turingos-status.ts
var ELEMENT_NAME8 = "turingos-status";
var STATE_LABEL = {
  connecting: "connecting",
  connected: "connected",
  reconnecting: "reconnecting",
  disconnected: "offline"
};
var TuringOSStatus = class extends HTMLElement {
  constructor() {
    super(...arguments);
    this._pill = null;
    this._label = null;
    this._bound = null;
  }
  connectedCallback() {
    this.setAttribute("data-block-type", "connection_status");
    this._render();
    const initial = window.__turingos_ws_state ?? "connecting";
    this._apply(initial);
    this._bound = (e) => {
      const detail = e.detail;
      if (detail && typeof detail.state === "string") {
        this._apply(detail.state);
      }
    };
    document.addEventListener("turingos:ws_state", this._bound);
  }
  disconnectedCallback() {
    if (this._bound !== null) {
      document.removeEventListener("turingos:ws_state", this._bound);
      this._bound = null;
    }
  }
  _render() {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    const pill = document.createElement("span");
    pill.className = "tos-conn-pill";
    pill.setAttribute("role", "status");
    pill.setAttribute("aria-live", "polite");
    const dot = document.createElement("span");
    dot.className = "tos-conn-dot";
    dot.setAttribute("aria-hidden", "true");
    pill.appendChild(dot);
    const label = document.createElement("span");
    label.className = "tos-conn-label";
    label.textContent = STATE_LABEL.connecting;
    pill.appendChild(label);
    this.appendChild(pill);
    this._pill = pill;
    this._label = label;
  }
  _apply(state) {
    if (this._pill === null || this._label === null) return;
    this._pill.dataset["state"] = state;
    this._label.textContent = STATE_LABEL[state] ?? state;
  }
};
function register8() {
  if (!customElements.get(ELEMENT_NAME8)) {
    customElements.define(ELEMENT_NAME8, TuringOSStatus);
  }
}

// src/components/spec-grill.ts
var ELEMENT_NAME9 = "tos-spec-grill";
var ANSWER_MAX_CHARS = 4096;
var QUESTION_COUNT = 8;
var TosSpecGrill = class extends HTMLElement {
  constructor() {
    super(...arguments);
    this._state = "idle";
    this._questions = [];
    this._answers = [];
    this._currentIndex = 0;
    this._errorMessage = "";
    this._specResponse = null;
    this._wsListener = null;
    /** Bound keydown handler — Cmd/Ctrl+Enter advances. */
    this._keyHandler = null;
  }
  connectedCallback() {
    this.setAttribute("data-block-type", "spec_grill");
    this._setState("idle");
    this._render();
    this._wsListener = (e) => this._onWsMessage(e);
    document.addEventListener("turingos:ir_update", this._wsListener);
    this._keyHandler = (e) => this._onKeydown(e);
    this.addEventListener("keydown", this._keyHandler);
  }
  disconnectedCallback() {
    if (this._wsListener !== null) {
      document.removeEventListener("turingos:ir_update", this._wsListener);
      this._wsListener = null;
    }
    if (this._keyHandler !== null) {
      this.removeEventListener("keydown", this._keyHandler);
      this._keyHandler = null;
    }
  }
  get currentState() {
    return this._state;
  }
  get answers() {
    return this._answers;
  }
  get currentIndex() {
    return this._currentIndex;
  }
  _setState(next) {
    this._state = next;
    this.setAttribute("data-state", next);
  }
  /** null on pass, else a Chinese error message. */
  validateAnswer(answer) {
    if (answer.length === 0) {
      return "\u8BF7\u5199\u4E00\u70B9\u5185\u5BB9\u518D\u7EE7\u7EED\u3002";
    }
    if (answer.length > ANSWER_MAX_CHARS) {
      return `\u56DE\u7B54\u592A\u957F\u4E86\uFF1A${answer.length} \u5B57\u7B26\uFF0C\u6700\u591A ${ANSWER_MAX_CHARS}\u3002`;
    }
    return null;
  }
  advanceWithAnswer(answer) {
    const trimmed = answer.trim();
    if (this.validateAnswer(trimmed) !== null) return false;
    this._answers[this._currentIndex] = trimmed;
    if (this._currentIndex < this._questions.length - 1) {
      this._currentIndex += 1;
      return true;
    }
    this._currentIndex = this._questions.length;
    return true;
  }
  async _loadQuestions() {
    this._setState("loading_questions");
    this._render();
    try {
      const resp = await fetch("/api/spec/questions");
      if (!resp.ok) {
        throw new Error(`HTTP ${resp.status}`);
      }
      const data = await resp.json();
      if (!Array.isArray(data.questions) || data.questions.length !== QUESTION_COUNT) {
        throw new Error(`expected ${QUESTION_COUNT} questions, got ${data.questions?.length}`);
      }
      this._questions = data.questions.slice();
      this._answers = new Array(QUESTION_COUNT).fill("");
      this._currentIndex = 0;
      this._setState("interviewing");
      this._render();
    } catch (err) {
      this._errorMessage = err instanceof Error ? err.message : "\u52A0\u8F7D\u95EE\u9898\u5931\u8D25\uFF0C\u8BF7\u7A0D\u540E\u91CD\u8BD5\u3002";
      this._setState("error");
      this._render();
    }
  }
  async _submit() {
    this._setState("submitting");
    this._render();
    try {
      const resp = await fetch("/api/spec/submit", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ answers: this._answers })
      });
      if (!resp.ok) {
        let reason = `HTTP ${resp.status}`;
        try {
          const errBody = await resp.json();
          if (typeof errBody.reason === "string") reason = errBody.reason;
        } catch {
        }
        throw new Error(reason);
      }
      const data = await resp.json();
      this._specResponse = data;
      this._setState("spec_ready");
      this._render();
    } catch (err) {
      this._errorMessage = err instanceof Error ? err.message : "\u5408\u6210 spec \u5931\u8D25\uFF0C\u8BF7\u7A0D\u540E\u91CD\u8BD5\u3002";
      this._setState("error");
      this._render();
    }
  }
  // WS arrival corroborates POST; POST stays the source of truth for spec_md.
  _onWsMessage(e) {
    const detail = e.detail;
    if (detail == null || detail.msg_type !== "spec_complete") return;
    if (this._specResponse != null && this._specResponse.session_id === detail.session_id) return;
  }
  _onKeydown(e) {
    if (this._state !== "interviewing") return;
    if ((e.metaKey || e.ctrlKey) && e.key === "Enter") {
      e.preventDefault();
      this._submitCurrent();
    }
  }
  _submitCurrent() {
    const ta = this.querySelector('textarea[name="spec-answer"]');
    if (ta === null) return;
    const value = ta.value;
    const errMsg = this.validateAnswer(value.trim());
    if (errMsg !== null) {
      this._showInlineError(errMsg);
      return;
    }
    const wasLast = this._currentIndex === this._questions.length - 1;
    this.advanceWithAnswer(value);
    if (wasLast) {
      void this._submit();
    } else {
      this._render();
    }
  }
  _showInlineError(message) {
    const err = this.querySelector("small[data-error]");
    if (err === null) return;
    err.textContent = message;
    err.style.display = "";
  }
  _render() {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    switch (this._state) {
      case "idle":
        this._renderIdle();
        break;
      case "loading_questions":
        this._renderLoading("\u6B63\u5728\u52A0\u8F7D\u95EE\u9898");
        break;
      case "interviewing":
        this._renderInterviewing();
        break;
      case "submitting":
        this._renderLoading("\u6B63\u5728\u5408\u6210 spec");
        break;
      case "spec_ready":
        this._renderSpecReady();
        break;
      case "error":
        this._renderError();
        break;
    }
  }
  _renderIdle() {
    const wrap = document.createElement("section");
    wrap.className = "spec-grill-idle";
    const eyebrow = document.createElement("p");
    eyebrow.className = "spec-grill-eyebrow";
    eyebrow.textContent = "TISR \xB7 \u516B\u95EE\u8BBF\u8C08";
    wrap.appendChild(eyebrow);
    const lede = document.createElement("p");
    lede.className = "spec-grill-lede";
    lede.textContent = '\u4E0D\u7528\u60F3\u7A0B\u5E8F\u600E\u4E48\u505A\u3002\u6211\u4F1A\u95EE\u516B\u4E2A\u5173\u4E8E"\u65E5\u5E38\u9EBB\u70E6"\u7684\u95EE\u9898\uFF0C\u4F60\u50CF\u804A\u5929\u90A3\u6837\u56DE\u7B54\u5C31\u597D\u3002\u95EE\u5B8C\u4E4B\u540E\uFF0Cspec.md \u4F1A\u81EA\u52A8\u5199\u51FA\u6765\u2014\u2014\u90A3\u662F\u4F60\u5DE5\u5177\u7684\u8BBE\u8BA1\u8349\u7A3F\u3002\u518D\u4E0B\u4E00\u6B65\uFF0C\u7F51\u9875\u5C31\u4F1A\u88AB\u751F\u6210\u3002';
    wrap.appendChild(lede);
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "spec-grill-cta";
    btn.textContent = "\u5F00\u59CB\u8BBF\u8C08 \u2192";
    btn.addEventListener("click", () => {
      void this._loadQuestions();
    });
    wrap.appendChild(btn);
    this.appendChild(wrap);
  }
  _renderLoading(label) {
    const wrap = document.createElement("section");
    wrap.className = "spec-grill-loading";
    const phrase = document.createElement("p");
    phrase.className = "spec-grill-loading-phrase";
    phrase.appendChild(document.createTextNode(label));
    const dots = document.createElement("span");
    dots.className = "spec-grill-dots";
    dots.setAttribute("aria-hidden", "true");
    for (let i = 0; i < 3; i++) {
      const dot = document.createElement("span");
      dot.textContent = "\xB7";
      dots.appendChild(dot);
    }
    phrase.appendChild(dots);
    wrap.appendChild(phrase);
    this.appendChild(wrap);
  }
  _renderInterviewing() {
    const wrap = document.createElement("section");
    wrap.className = "spec-grill-question";
    const progress = document.createElement("div");
    progress.className = "spec-grill-progress";
    progress.textContent = `Q ${this._currentIndex + 1} / ${this._questions.length}`;
    wrap.appendChild(progress);
    const q = document.createElement("p");
    q.className = "spec-grill-question-text";
    q.textContent = this._questions[this._currentIndex] ?? "";
    wrap.appendChild(q);
    const ta = document.createElement("textarea");
    ta.name = "spec-answer";
    ta.className = "spec-grill-input";
    ta.rows = 6;
    ta.value = this._answers[this._currentIndex] ?? "";
    ta.placeholder = "\u5728\u8FD9\u91CC\u5199\u4E0B\u4F60\u7684\u56DE\u7B54\u2026   (\u2318/Ctrl+Enter \u8FDB\u5165\u4E0B\u4E00\u9898)";
    ta.autocapitalize = "sentences";
    ta.spellcheck = false;
    requestAnimationFrame(() => ta.focus());
    wrap.appendChild(ta);
    const err = document.createElement("small");
    err.setAttribute("data-error", "");
    err.className = "spec-grill-error";
    err.style.display = "none";
    wrap.appendChild(err);
    const footer = document.createElement("footer");
    footer.className = "spec-grill-footer";
    if (this._currentIndex > 0) {
      const back = document.createElement("button");
      back.type = "button";
      back.className = "spec-grill-back";
      back.textContent = "\u2190 \u4E0A\u4E00\u9898";
      back.addEventListener("click", () => {
        this._answers[this._currentIndex] = ta.value;
        this._currentIndex -= 1;
        this._render();
      });
      footer.appendChild(back);
    }
    const advance = document.createElement("button");
    advance.type = "button";
    advance.className = "spec-grill-advance";
    const isLast = this._currentIndex === this._questions.length - 1;
    advance.textContent = isLast ? "\u5B8C\u6210\u8BBF\u8C08 \u2192" : "\u4E0B\u4E00\u9898 \u2192";
    advance.addEventListener("click", () => this._submitCurrent());
    footer.appendChild(advance);
    wrap.appendChild(footer);
    this.appendChild(wrap);
  }
  _renderSpecReady() {
    const result = document.createElement("tos-spec-result");
    if (this._specResponse !== null) {
      result.spec = this._specResponse;
      try {
        result.dataset["payload"] = JSON.stringify(this._specResponse);
      } catch {
      }
    }
    this.appendChild(result);
  }
  _renderError() {
    const wrap = document.createElement("section");
    wrap.className = "spec-grill-errstate";
    const phrase = document.createElement("p");
    phrase.className = "spec-grill-errmsg";
    phrase.textContent = this._errorMessage || "\u51FA\u9519\u4E86\u3002";
    wrap.appendChild(phrase);
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "spec-grill-cta";
    btn.textContent = "\u91CD\u8BD5";
    btn.addEventListener("click", () => {
      this._errorMessage = "";
      if (this._questions.length === 0) {
        void this._loadQuestions();
      } else {
        this._currentIndex = Math.max(0, this._questions.length - 1);
        this._setState("interviewing");
        this._render();
      }
    });
    wrap.appendChild(btn);
    this.appendChild(wrap);
  }
};
function register9() {
  if (!customElements.get(ELEMENT_NAME9)) {
    customElements.define(ELEMENT_NAME9, TosSpecGrill);
  }
}

// src/components/spec-result.ts
var ELEMENT_NAME10 = "tos-spec-result";
var TosSpecResult = class extends HTMLElement {
  constructor() {
    super(...arguments);
    this._state = "idle";
    this._spec = null;
    this._errorMessage = "";
    this._generated = null;
    // W8: live retry progress driven by WS broadcasts.
    this._currentAttempt = 0;
    this._maxAttempts = 0;
    this._wsListener = null;
  }
  /** Property setter: caller assigns the SpecSubmitResponse here. */
  set spec(value) {
    this._spec = value;
    if (this.isConnected) {
      this._render();
    }
  }
  get spec() {
    return this._spec;
  }
  connectedCallback() {
    this.setAttribute("data-block-type", "spec_result");
    if (this._spec === null) {
      const raw = this.dataset["payload"];
      if (raw != null) {
        try {
          this._spec = JSON.parse(raw);
        } catch {
        }
      }
    }
    this._render();
    this._wsListener = (e) => this._onWsMessage(e);
    document.addEventListener("turingos:ir_update", this._wsListener);
  }
  disconnectedCallback() {
    if (this._wsListener !== null) {
      document.removeEventListener("turingos:ir_update", this._wsListener);
      this._wsListener = null;
    }
  }
  // W8: the inline WS bootstrap dispatches every WS message under this
  // event name (despite the W2-era "ir_update" name); we filter by msg_type.
  _onWsMessage(e) {
    const detail = e.detail;
    if (detail == null) return;
    if (this._spec === null) return;
    const sid = this._spec.session_id;
    if (detail.msg_type === "generate_attempt_started") {
      if (detail.session_id !== sid) return;
      this._currentAttempt = detail.attempt;
      this._maxAttempts = detail.max_attempts;
      if (this._state === "generating") {
        this._render();
      }
      return;
    }
    if (detail.msg_type === "generate_attempt_failed") {
      if (detail.session_id !== sid) return;
      this._errorMessage = `\u5C1D\u8BD5 ${detail.attempt}/${detail.max_attempts} \u5931\u8D25: ${detail.reason}`;
      if (this._state === "generating") {
        this._render();
      }
      return;
    }
  }
  async _startGenerate() {
    if (this._spec === null) return;
    this._state = "generating";
    this._currentAttempt = 0;
    this._maxAttempts = 0;
    this._errorMessage = "";
    this.setAttribute("data-state", this._state);
    this._render();
    try {
      const resp = await fetch("/api/generate", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ session_id: this._spec.session_id })
      });
      if (!resp.ok) {
        let reason = `HTTP ${resp.status}`;
        try {
          const errBody = await resp.json();
          if (typeof errBody.reason === "string") reason = errBody.reason;
        } catch {
        }
        throw new Error(reason);
      }
      this._generated = await resp.json();
      this._state = "generated";
      this.setAttribute("data-state", this._state);
      this._render();
    } catch (err) {
      this._errorMessage = err instanceof Error ? err.message : "\u751F\u6210\u5931\u8D25\uFF0C\u8BF7\u7A0D\u540E\u91CD\u8BD5\u3002";
      this._state = "error";
      this.setAttribute("data-state", this._state);
      this._render();
    }
  }
  _render() {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    if (this._state === "generated" && this._generated !== null) {
      const viewer = document.createElement("tos-artifact-viewer");
      viewer.artifacts = this._generated;
      try {
        viewer.dataset["payload"] = JSON.stringify(this._generated);
      } catch {
      }
      this.appendChild(viewer);
      return;
    }
    if (this._spec === null) {
      const p = document.createElement("p");
      p.className = "spec-result-empty";
      p.textContent = "(\u5C1A\u65E0 spec \u6570\u636E)";
      this.appendChild(p);
      return;
    }
    const article = document.createElement("article");
    article.className = "spec-result-article";
    renderMarkdownInto(article, this._spec.spec_md);
    this.appendChild(article);
    if (this._spec.capsule_cid != null && this._spec.capsule_cid.length > 0) {
      const cidFooter = document.createElement("p");
      cidFooter.className = "spec-result-cid";
      const label = document.createElement("span");
      label.className = "spec-result-cid-label";
      label.textContent = "CAS capsule ";
      cidFooter.appendChild(label);
      const code = document.createElement("code");
      const cid = this._spec.capsule_cid;
      code.title = cid;
      code.textContent = "cid:" + truncateMiddle(cid, 8, 8);
      cidFooter.appendChild(code);
      this.appendChild(cidFooter);
    }
    const cta = document.createElement("div");
    cta.className = "spec-result-cta";
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "spec-result-generate-btn";
    if (this._state === "generating") {
      btn.textContent = "\u6B63\u5728\u751F\u6210\u4EE3\u7801\u2026";
      btn.disabled = true;
    } else if (this._state === "error") {
      btn.textContent = "\u91CD\u8BD5\u751F\u6210\u4EE3\u7801 \u2192";
    } else {
      btn.textContent = "\u751F\u6210\u4EE3\u7801 \u2192";
    }
    btn.addEventListener("click", () => {
      void this._startGenerate();
    });
    cta.appendChild(btn);
    if (this._state === "generating") {
      const chip = document.createElement("p");
      chip.className = "spec-result-progress";
      const prefix = document.createElement("span");
      prefix.className = "spec-result-progress-prefix";
      prefix.textContent = "\u6B63\u5728\u751F\u6210\u2026";
      chip.appendChild(prefix);
      if (this._currentAttempt > 0 && this._maxAttempts > 0) {
        chip.appendChild(document.createTextNode("  "));
        const counter = document.createElement("em");
        counter.className = "spec-result-progress-counter";
        counter.textContent = `(\u5C1D\u8BD5 ${this._currentAttempt}/${this._maxAttempts})`;
        chip.appendChild(counter);
      }
      cta.appendChild(chip);
      if (this._errorMessage !== "") {
        const note = document.createElement("p");
        note.className = "spec-result-progress-note";
        note.textContent = this._errorMessage;
        cta.appendChild(note);
      }
    }
    if (this._state === "error") {
      const errLine = document.createElement("p");
      errLine.className = "spec-result-error";
      errLine.textContent = this._errorMessage;
      cta.appendChild(errLine);
      const m = /last_artifact=([A-Za-z0-9_./-]+)/.exec(this._errorMessage);
      if (m !== null) {
        const inspectLink = document.createElement("a");
        inspectLink.className = "spec-result-inspect-link";
        inspectLink.href = `/api/artifact/${m[1]}`;
        inspectLink.textContent = "\u67E5\u770B\u6700\u540E\u4E00\u6B21\u4EA7\u7269 \u2193";
        inspectLink.setAttribute("target", "_blank");
        inspectLink.setAttribute("rel", "noopener noreferrer");
        cta.appendChild(inspectLink);
      }
    }
    this.appendChild(cta);
  }
};
function renderMarkdownInto(target, md) {
  const lines = md.replace(/\r\n/g, "\n").split("\n");
  let i = 0;
  while (i < lines.length) {
    const line = lines[i] ?? "";
    if (line.startsWith("```")) {
      const code = document.createElement("pre");
      const inner = document.createElement("code");
      const buf = [];
      i += 1;
      while (i < lines.length && !(lines[i] ?? "").startsWith("```")) {
        buf.push(lines[i] ?? "");
        i += 1;
      }
      i += 1;
      inner.textContent = buf.join("\n");
      code.appendChild(inner);
      target.appendChild(code);
      continue;
    }
    const heading = /^(#{1,6})\s+(.*)$/.exec(line);
    if (heading !== null) {
      const level = heading[1].length;
      const h = document.createElement(`h${Math.min(level, 6)}`);
      renderInlineInto(h, heading[2] ?? "");
      target.appendChild(h);
      i += 1;
      continue;
    }
    if (/^\s*[-*]\s+/.test(line)) {
      const ul = document.createElement("ul");
      while (i < lines.length && /^\s*[-*]\s+/.test(lines[i] ?? "")) {
        const item = (lines[i] ?? "").replace(/^\s*[-*]\s+/, "");
        const li = document.createElement("li");
        renderInlineInto(li, item);
        ul.appendChild(li);
        i += 1;
      }
      target.appendChild(ul);
      continue;
    }
    if (/^\s*\d+\.\s+/.test(line)) {
      const ol = document.createElement("ol");
      while (i < lines.length && /^\s*\d+\.\s+/.test(lines[i] ?? "")) {
        const item = (lines[i] ?? "").replace(/^\s*\d+\.\s+/, "");
        const li = document.createElement("li");
        renderInlineInto(li, item);
        ol.appendChild(li);
        i += 1;
      }
      target.appendChild(ol);
      continue;
    }
    if (line.trim() === "") {
      i += 1;
      continue;
    }
    const paraBuf = [line];
    i += 1;
    while (i < lines.length && (lines[i] ?? "").trim() !== "" && !/^(#{1,6})\s+/.test(lines[i] ?? "") && !(lines[i] ?? "").startsWith("```") && !/^\s*[-*]\s+/.test(lines[i] ?? "") && !/^\s*\d+\.\s+/.test(lines[i] ?? "")) {
      paraBuf.push(lines[i] ?? "");
      i += 1;
    }
    const p = document.createElement("p");
    renderInlineInto(p, paraBuf.join(" "));
    target.appendChild(p);
  }
}
function renderInlineInto(parent, text) {
  let remaining = text;
  while (remaining.length > 0) {
    const codeIdx = remaining.indexOf("`");
    const boldIdx = remaining.indexOf("**");
    let emIdx = -1;
    for (let k = 0; k < remaining.length; k++) {
      if (remaining[k] === "*" && remaining[k + 1] !== "*" && remaining[k - 1] !== "*") {
        emIdx = k;
        break;
      }
    }
    const candidates = [];
    if (codeIdx >= 0) candidates.push([codeIdx, "code"]);
    if (boldIdx >= 0) candidates.push([boldIdx, "bold"]);
    if (emIdx >= 0) candidates.push([emIdx, "em"]);
    if (candidates.length === 0) {
      parent.appendChild(document.createTextNode(remaining));
      return;
    }
    candidates.sort((a, b) => a[0] - b[0]);
    const [pos, kind] = candidates[0];
    if (pos > 0) {
      parent.appendChild(document.createTextNode(remaining.slice(0, pos)));
    }
    if (kind === "code") {
      const end = remaining.indexOf("`", pos + 1);
      if (end < 0) {
        parent.appendChild(document.createTextNode(remaining.slice(pos)));
        return;
      }
      const span = document.createElement("code");
      span.textContent = remaining.slice(pos + 1, end);
      parent.appendChild(span);
      remaining = remaining.slice(end + 1);
    } else if (kind === "bold") {
      const end = remaining.indexOf("**", pos + 2);
      if (end < 0) {
        parent.appendChild(document.createTextNode(remaining.slice(pos)));
        return;
      }
      const strong = document.createElement("strong");
      strong.textContent = remaining.slice(pos + 2, end);
      parent.appendChild(strong);
      remaining = remaining.slice(end + 2);
    } else {
      let end = -1;
      for (let k = pos + 1; k < remaining.length; k++) {
        if (remaining[k] === "*" && remaining[k + 1] !== "*" && remaining[k - 1] !== "*") {
          end = k;
          break;
        }
      }
      if (end < 0) {
        parent.appendChild(document.createTextNode(remaining.slice(pos)));
        return;
      }
      const em = document.createElement("em");
      em.textContent = remaining.slice(pos + 1, end);
      parent.appendChild(em);
      remaining = remaining.slice(end + 1);
    }
  }
}
function register10() {
  if (!customElements.get(ELEMENT_NAME10)) {
    customElements.define(ELEMENT_NAME10, TosSpecResult);
  }
}

// src/components/artifact-viewer.ts
var ELEMENT_NAME11 = "tos-artifact-viewer";
var SANDBOX_ALLOWED_TOKENS = ["allow-scripts"];
function buildSandboxAttribute() {
  return SANDBOX_ALLOWED_TOKENS.join(" ");
}
function formatBytes(n) {
  if (n < 1024) return `${n} B`;
  if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KB`;
  return `${(n / (1024 * 1024)).toFixed(1)} MB`;
}
var TosArtifactViewer = class extends HTMLElement {
  constructor() {
    super(...arguments);
    this._data = null;
    this._selectedIdx = 0;
  }
  set artifacts(value) {
    this._data = value;
    const idx = value.artifacts.findIndex((a) => a.path.toLowerCase().endsWith(".html"));
    this._selectedIdx = idx >= 0 ? idx : 0;
    if (this.isConnected) this._render();
  }
  get artifacts() {
    return this._data;
  }
  connectedCallback() {
    this.setAttribute("data-block-type", "artifact_viewer");
    if (this._data === null) {
      const raw = this.dataset["payload"];
      if (raw != null) {
        try {
          this._data = JSON.parse(raw);
          const idx = this._data.artifacts.findIndex((a) => a.path.toLowerCase().endsWith(".html"));
          this._selectedIdx = idx >= 0 ? idx : 0;
        } catch {
        }
      }
    }
    this._render();
  }
  _render() {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    if (this._data === null || this._data.artifacts.length === 0) {
      const p = document.createElement("p");
      p.className = "artifact-viewer-empty";
      p.textContent = "(\u5C1A\u672A\u751F\u6210\u4EFB\u4F55\u6587\u4EF6)";
      this.appendChild(p);
      return;
    }
    const header = document.createElement("header");
    header.className = "artifact-viewer-header";
    const eyebrow = document.createElement("p");
    eyebrow.className = "artifact-viewer-eyebrow";
    eyebrow.textContent = "\u751F\u6210\u4EA7\u7269 \xB7 LIVE PREVIEW";
    header.appendChild(eyebrow);
    const title = document.createElement("h2");
    title.className = "artifact-viewer-title";
    title.textContent = "\u4F60\u7684\u5DE5\u5177\uFF0C\u5DF2\u7ECF\u5199\u597D\u4E86\u3002";
    header.appendChild(title);
    const attempts = typeof this._data.total_attempts === "number" ? this._data.total_attempts : 1;
    if (attempts > 1) {
      const retryCaption = document.createElement("p");
      retryCaption.className = "artifact-viewer-retry-caption";
      retryCaption.textContent = `\u7ECF\u8FC7 ${attempts} \u6B21\u5C1D\u8BD5 \xB7 \u5DF2\u901A\u8FC7\u542F\u53D1\u5F0F\u9A8C\u8BC1`;
      header.appendChild(retryCaption);
    }
    this.appendChild(header);
    const layout = document.createElement("div");
    layout.className = "artifact-viewer-layout";
    if (this._data.artifacts.length > 1) {
      const list = document.createElement("ul");
      list.className = "artifact-viewer-filelist";
      this._data.artifacts.forEach((a, idx) => {
        const li = document.createElement("li");
        li.className = idx === this._selectedIdx ? "is-selected" : "";
        const btn = document.createElement("button");
        btn.type = "button";
        btn.textContent = a.path;
        btn.title = a.path;
        btn.addEventListener("click", () => {
          this._selectedIdx = idx;
          this._render();
        });
        li.appendChild(btn);
        list.appendChild(li);
      });
      layout.appendChild(list);
    }
    const main = document.createElement("div");
    main.className = "artifact-viewer-main";
    const current = this._data.artifacts[this._selectedIdx];
    if (current === void 0) {
      const p = document.createElement("p");
      p.textContent = "(\u65E0\u6587\u4EF6)";
      main.appendChild(p);
      layout.appendChild(main);
      this.appendChild(layout);
      return;
    }
    const isHtml = current.path.toLowerCase().endsWith(".html") || current.path.toLowerCase().endsWith(".htm");
    if (isHtml) {
      const frame = document.createElement("iframe");
      frame.className = "artifact-viewer-iframe";
      frame.setAttribute("sandbox", buildSandboxAttribute());
      frame.setAttribute(
        "src",
        `/api/artifact/${encodeURIComponent(this._data.session_id)}/${encodeURIComponent(current.path)}`
      );
      frame.setAttribute("title", `Preview of ${current.path}`);
      frame.setAttribute("loading", "lazy");
      main.appendChild(frame);
    } else {
      const note = document.createElement("p");
      note.className = "artifact-viewer-note";
      note.textContent = `${current.path} \u662F ${current.content_type} \u2014 \u65E0\u6CD5\u76F4\u63A5\u9884\u89C8\u3002\u8BF7\u4E0B\u8F7D\u67E5\u770B\u3002`;
      main.appendChild(note);
    }
    const cap = document.createElement("p");
    cap.className = "artifact-viewer-caption";
    const pathSpan = document.createElement("span");
    pathSpan.className = "artifact-viewer-caption-path";
    pathSpan.textContent = current.path;
    cap.appendChild(pathSpan);
    cap.appendChild(document.createTextNode("  \xB7  "));
    const sizeSpan = document.createElement("span");
    sizeSpan.textContent = formatBytes(current.size_bytes);
    cap.appendChild(sizeSpan);
    cap.appendChild(document.createTextNode("  \xB7  "));
    const ctSpan = document.createElement("span");
    ctSpan.textContent = current.content_type;
    cap.appendChild(ctSpan);
    main.appendChild(cap);
    const dl = document.createElement("a");
    dl.className = "artifact-viewer-download";
    dl.href = `/api/artifact/${encodeURIComponent(this._data.session_id)}/${encodeURIComponent(current.path)}`;
    dl.setAttribute("download", current.path);
    dl.textContent = `\u4E0B\u8F7D ${current.path} \u2193`;
    main.appendChild(dl);
    layout.appendChild(main);
    this.appendChild(layout);
  }
};
function register11() {
  if (!customElements.get(ELEMENT_NAME11)) {
    customElements.define(ELEMENT_NAME11, TosArtifactViewer);
  }
}

// src/components/welcome-state.ts
var API_KEY_MIN = 16;
var API_KEY_MAX = 256;
function validateApiKey(key) {
  if (!key.startsWith("sk-")) {
    return 'API \u5BC6\u94A5\u9700\u8981\u4EE5 "sk-" \u5F00\u5934\uFF08SiliconFlow / OpenAI \u4E60\u60EF\uFF09\u3002';
  }
  if (key.length < API_KEY_MIN) {
    return `\u5BC6\u94A5\u592A\u77ED\u4E86\uFF1A${key.length} \u5B57\u7B26\uFF0C\u81F3\u5C11 ${API_KEY_MIN}\u3002`;
  }
  if (key.length > API_KEY_MAX) {
    return `\u5BC6\u94A5\u592A\u957F\u4E86\uFF1A${key.length} \u5B57\u7B26\uFF0C\u6700\u591A ${API_KEY_MAX}\u3002`;
  }
  for (let i = 0; i < key.length; i++) {
    const c = key.charCodeAt(i);
    if (c < 33 || c > 126) {
      return "\u5BC6\u94A5\u53EA\u80FD\u5305\u542B\u53EF\u89C1 ASCII \u5B57\u7B26\u3002";
    }
  }
  return null;
}
function stateForNextStep(next) {
  switch (next) {
    case "Init":
      return "step_init";
    case "LlmConfig":
      return "step_llm_config";
    case "ApiKey":
      return "step_api_key";
    case "AgentDeploy":
      return "step_agent_deploy";
    case "Spec":
    case "Generate":
    case "Done":
      return "step_ready";
  }
}
function stepIndex(next) {
  switch (next) {
    case "Init":
      return 0;
    case "LlmConfig":
      return 1;
    case "ApiKey":
      return 2;
    case "AgentDeploy":
      return 3;
    case "Spec":
    case "Generate":
    case "Done":
      return 4;
  }
}
var WIZARD_STEPS = [
  { key: "init", label: "\u5DE5\u4F5C\u7AD9" },
  { key: "llm_config", label: "\u6A21\u578B\u914D\u7F6E" },
  { key: "api_key", label: "API \u5BC6\u94A5" },
  { key: "agent_deploy", label: "\u6CE8\u518C Agent" },
  { key: "ready", label: "\u5F00\u59CB\u8BBF\u8C08" }
];

// src/components/welcome.ts
var ELEMENT_NAME12 = "tos-welcome";
var TosWelcome = class extends HTMLElement {
  constructor() {
    super(...arguments);
    this._state = "loading_status";
    this._status = null;
    this._errorMessage = "";
  }
  connectedCallback() {
    this.setAttribute("data-block-type", "welcome");
    this._setState("loading_status");
    this._render();
    void this._loadStatus();
  }
  // No event listeners outside this component; nothing to remove.
  disconnectedCallback() {
  }
  get currentState() {
    return this._state;
  }
  get currentStatus() {
    return this._status;
  }
  _setState(next) {
    this._state = next;
    this.setAttribute("data-state", next);
    if (this._status !== null) {
      this.setAttribute("data-active-step", this._status.next_step);
    }
  }
  async _loadStatus() {
    try {
      const resp = await fetch("/api/welcome/status");
      if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
      const data = await resp.json();
      this._status = data;
      this._setState(stateForNextStep(data.next_step));
      this._render();
    } catch (err) {
      this._errorMessage = err instanceof Error ? err.message : "\u52A0\u8F7D\u72B6\u6001\u5931\u8D25\u3002";
      this._setState("error_status");
      this._render();
    }
  }
  async _postStep(endpoint, submittingState, errorState, body) {
    this._setState(submittingState);
    this._render();
    try {
      const resp = await fetch(endpoint, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: body !== void 0 ? JSON.stringify(body) : "{}"
      });
      if (!resp.ok) {
        let reason = `HTTP ${resp.status}`;
        try {
          const errBody = await resp.json();
          if (typeof errBody.reason === "string") reason = errBody.reason;
        } catch {
        }
        throw new Error(reason);
      }
      const data = await resp.json();
      this._status = data;
      this._setState(stateForNextStep(data.next_step));
      this._render();
    } catch (err) {
      this._errorMessage = err instanceof Error ? err.message : "\u8BF7\u6C42\u5931\u8D25\u3002";
      this._setState(errorState);
      this._render();
    }
  }
  // ---------- per-step handlers ---------------------------------------------
  _doInit() {
    return this._postStep("/api/welcome/init", "submitting_init", "error_init");
  }
  _doLlmConfig() {
    return this._postStep(
      "/api/welcome/llm-config",
      "submitting_llm_config",
      "error_llm_config"
    );
  }
  _doAgentDeploy() {
    return this._postStep(
      "/api/welcome/agent-deploy",
      "submitting_agent_deploy",
      "error_agent_deploy"
    );
  }
  async _doSetApiKey(key) {
    const errMsg = validateApiKey(key);
    if (errMsg !== null) {
      this._errorMessage = errMsg;
      this._setState("error_api_key");
      this._render();
      return;
    }
    await this._postStep(
      "/api/welcome/api-key",
      "submitting_api_key",
      "error_api_key",
      { api_key: key }
    );
  }
  // ---------- rendering -----------------------------------------------------
  _render() {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    const wrap = document.createElement("section");
    wrap.className = "welcome-wrap";
    wrap.appendChild(this._renderProgress());
    if (this._state === "loading_status") {
      wrap.appendChild(this._renderLoading("\u52A0\u8F7D\u4E2D"));
    } else if (this._state === "error_status") {
      wrap.appendChild(this._renderStatusError());
    } else {
      wrap.appendChild(this._renderCard());
    }
    this.appendChild(wrap);
  }
  _renderProgress() {
    const nav = document.createElement("ol");
    nav.className = "welcome-progress";
    nav.setAttribute("aria-label", "\u5B89\u88C5\u8FDB\u5EA6");
    const status = this._status;
    const activeIdx = status !== null ? stepIndex(status.next_step) : 0;
    WIZARD_STEPS.forEach((step, idx) => {
      const li = document.createElement("li");
      li.className = "welcome-progress-step";
      let phase;
      if (status !== null && status.next_step === "Done") {
        phase = "done";
      } else if (idx < activeIdx) {
        phase = "done";
      } else if (idx === activeIdx) {
        phase = "active";
      } else {
        phase = "pending";
      }
      li.setAttribute("data-phase", phase);
      const circle = document.createElement("span");
      circle.className = "welcome-progress-num";
      circle.textContent = String(idx + 1);
      li.appendChild(circle);
      const label = document.createElement("span");
      label.className = "welcome-progress-label";
      label.textContent = step.label;
      li.appendChild(label);
      nav.appendChild(li);
    });
    return nav;
  }
  _renderLoading(label) {
    const wrap = document.createElement("div");
    wrap.className = "welcome-loading";
    const phrase = document.createElement("p");
    phrase.className = "welcome-loading-phrase";
    phrase.textContent = label;
    const dots = document.createElement("span");
    dots.className = "welcome-dots";
    dots.setAttribute("aria-hidden", "true");
    for (let i = 0; i < 3; i++) {
      const dot = document.createElement("span");
      dot.textContent = "\xB7";
      dots.appendChild(dot);
    }
    phrase.appendChild(dots);
    wrap.appendChild(phrase);
    return wrap;
  }
  _renderStatusError() {
    const wrap = document.createElement("div");
    wrap.className = "welcome-card welcome-error-card";
    const caption = document.createElement("p");
    caption.className = "welcome-step-caption";
    caption.textContent = "\u52A0\u8F7D\u72B6\u6001\u5931\u8D25";
    wrap.appendChild(caption);
    const msg = document.createElement("p");
    msg.className = "welcome-error-msg";
    msg.textContent = this._errorMessage || "\u65E0\u6CD5\u8BFB\u53D6 /api/welcome/status\u3002";
    wrap.appendChild(msg);
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "welcome-cta";
    btn.textContent = "\u91CD\u8BD5";
    btn.addEventListener("click", () => {
      this._errorMessage = "";
      void this._loadStatus();
    });
    wrap.appendChild(btn);
    return wrap;
  }
  /** Render the active card for whichever step is current. */
  _renderCard() {
    if (this._state === "step_init" || this._state === "submitting_init" || this._state === "error_init") {
      return this._renderStepCard({
        index: 1,
        title: "\u7B2C\u4E00\u6B65 \xB7 \u51C6\u5907\u5DE5\u4F5C\u7AD9",
        subtitle: '\u6211\u5E2E\u4F60\u5728\u786C\u76D8\u4E0A\u94FA\u4E00\u5F20\u7A7A\u767D\u7684"\u8D26\u672C\u684C\u9762"\u2014\u2014\u91CC\u9762\u6709 genesis_payload.toml \u548C agent_pubkeys.json\uFF0C\u662F\u540E\u9762\u6240\u6709\u6B65\u9AA4\u7684\u5730\u57FA\u3002',
        ctaLabel: "\u51C6\u5907\u5DE5\u4F5C\u7AD9 \u2192",
        submitting: this._state === "submitting_init",
        submittingLabel: "\u6B63\u5728\u521D\u59CB\u5316\u5DE5\u4F5C\u7AD9",
        showError: this._state === "error_init",
        onClick: () => void this._doInit(),
        retryLabel: "\u91CD\u8BD5 init"
      });
    }
    if (this._state === "step_llm_config" || this._state === "submitting_llm_config" || this._state === "error_llm_config") {
      return this._renderStepCard({
        index: 2,
        title: "\u7B2C\u4E8C\u6B65 \xB7 \u914D\u7F6E\u4E24\u4E2A\u6A21\u578B",
        subtitle: '\u6211\u4F1A\u628A\u4E24\u4E2A LLM \u5199\u8FDB turingos.toml\u2014\u2014\u4E00\u4E2A\u8D1F\u8D23"\u95EE\u4F60\u95EE\u9898"\uFF08DeepSeek V3.2\uFF09\uFF0C\u4E00\u4E2A\u8D1F\u8D23"\u5199\u4EE3\u7801"\uFF08Qwen3-Coder 30B\uFF09\u3002\u53EA\u5199\u6A21\u578B\u540D\u5B57\uFF0C\u4E0D\u5199\u5BC6\u94A5\u3002',
        ctaLabel: "\u5199\u5165 turingos.toml \u2192",
        submitting: this._state === "submitting_llm_config",
        submittingLabel: "\u6B63\u5728\u5199\u5165\u6A21\u578B\u914D\u7F6E",
        showError: this._state === "error_llm_config",
        onClick: () => void this._doLlmConfig(),
        retryLabel: "\u91CD\u8BD5 llm config"
      });
    }
    if (this._state === "step_api_key" || this._state === "submitting_api_key" || this._state === "error_api_key") {
      return this._renderApiKeyCard();
    }
    if (this._state === "step_agent_deploy" || this._state === "submitting_agent_deploy" || this._state === "error_agent_deploy") {
      return this._renderStepCard({
        index: 4,
        title: "\u7B2C\u4E09\u6B65 \xB7 \u7ED9\u5DE5\u4F5C\u7AD9\u6CE8\u518C\u4E00\u4E2A Agent",
        subtitle: '\u6CE8\u518C\u4E00\u4E2A Solver \u89D2\u8272\u7684 agent_001\uFF0C\u544A\u8BC9\u7CFB\u7EDF"\u4EE5\u540E\u662F\u8FD9\u4E2A agent \u5728\u8DD1\u5DE5\u4F5C"\u3002\u8FD9\u662F Phase 6.1 \u7684\u591A agent \u4F53\u7CFB\u7684\u6700\u5C0F\u5165\u53E3\u3002',
        ctaLabel: "\u6CE8\u518C agent_001 \u2192",
        submitting: this._state === "submitting_agent_deploy",
        submittingLabel: "\u6B63\u5728\u6CE8\u518C Agent",
        showError: this._state === "error_agent_deploy",
        onClick: () => void this._doAgentDeploy(),
        retryLabel: "\u91CD\u8BD5 agent deploy"
      });
    }
    return this._renderReadyCard();
  }
  _renderStepCard(opts) {
    const card = document.createElement("div");
    card.className = "welcome-card";
    const caption = document.createElement("p");
    caption.className = "welcome-step-caption";
    caption.textContent = `STEP ${opts.index} / 5`;
    card.appendChild(caption);
    const h2 = document.createElement("h2");
    h2.className = "welcome-step-title";
    h2.textContent = opts.title;
    card.appendChild(h2);
    const sub = document.createElement("p");
    sub.className = "welcome-step-subtitle";
    sub.textContent = opts.subtitle;
    card.appendChild(sub);
    if (opts.submitting) {
      card.appendChild(this._renderLoading(opts.submittingLabel));
      return card;
    }
    if (opts.showError) {
      const errBlock = document.createElement("div");
      errBlock.className = "welcome-error-block";
      const msg = document.createElement("p");
      msg.className = "welcome-error-msg";
      msg.textContent = this._errorMessage || "\u51FA\u9519\u4E86\u3002";
      errBlock.appendChild(msg);
      const retry = document.createElement("button");
      retry.type = "button";
      retry.className = "welcome-cta";
      retry.textContent = opts.retryLabel;
      retry.addEventListener("click", () => {
        this._errorMessage = "";
        opts.onClick();
      });
      errBlock.appendChild(retry);
      card.appendChild(errBlock);
      return card;
    }
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "welcome-cta";
    btn.textContent = opts.ctaLabel;
    btn.addEventListener("click", () => opts.onClick());
    card.appendChild(btn);
    return card;
  }
  _renderApiKeyCard() {
    const card = document.createElement("div");
    card.className = "welcome-card";
    const caption = document.createElement("p");
    caption.className = "welcome-step-caption";
    caption.textContent = "STEP 3 / 5";
    card.appendChild(caption);
    const h2 = document.createElement("h2");
    h2.className = "welcome-step-title";
    h2.textContent = "\u628A SiliconFlow \u7684 API \u5BC6\u94A5\u4EA4\u7ED9\u6211";
    card.appendChild(h2);
    const sub = document.createElement("p");
    sub.className = "welcome-step-subtitle";
    sub.textContent = "\u5BC6\u94A5\u53EA\u6D3B\u5728\u8FD9\u4E2A\u670D\u52A1\u5668\u8FDB\u7A0B\u7684\u5185\u5B58\u91CC\u2014\u2014\u91CD\u542F\u5C31\u4E22\uFF0C\u4ECE\u4E0D\u5199\u76D8\u3001\u4E0D\u8FDB\u65E5\u5FD7\u3001\u4E0D\u4F1A\u56DE\u663E\u5728\u7F51\u9875\u4E0A\u3002\u4F60\u53EA\u9700\u8981\u5728\u6BCF\u6B21\u542F\u52A8 turingos_web \u4E4B\u540E\u586B\u4E00\u6B21\u3002";
    card.appendChild(sub);
    if (this._state === "submitting_api_key") {
      card.appendChild(this._renderLoading("\u6B63\u5728\u4FDD\u5B58\u5230\u5185\u5B58"));
      return card;
    }
    const alreadySet = this._status !== null && this._status.api_key_set && this._state !== "error_api_key";
    if (alreadySet) {
      const setLine = document.createElement("p");
      setLine.className = "welcome-api-set";
      setLine.textContent = "API \u5BC6\u94A5\u5DF2\u8BBE\u7F6E\uFF08\u4EC5\u4FDD\u5B58\u5728\u5185\u5B58\u4E2D\uFF09";
      card.appendChild(setLine);
      const replace = document.createElement("button");
      replace.type = "button";
      replace.className = "welcome-cta-soft";
      replace.textContent = "\u66FF\u6362\u5BC6\u94A5";
      replace.addEventListener("click", () => {
        if (this._status !== null) {
          this._status = { ...this._status, api_key_set: false };
        }
        this._setState("step_api_key");
        this._render();
      });
      card.appendChild(replace);
      return card;
    }
    const field = document.createElement("div");
    field.className = "welcome-api-field";
    const label = document.createElement("label");
    label.className = "welcome-api-label";
    label.setAttribute("for", "welcome-api-key-input");
    label.textContent = "SILICONFLOW_API_KEY";
    field.appendChild(label);
    const input = document.createElement("input");
    input.type = "password";
    input.id = "welcome-api-key-input";
    input.name = "api_key";
    input.placeholder = "sk-...";
    input.autocomplete = "off";
    input.spellcheck = false;
    input.className = "welcome-api-input";
    field.appendChild(input);
    card.appendChild(field);
    if (this._state === "error_api_key" && this._errorMessage) {
      const err = document.createElement("p");
      err.className = "welcome-error-msg";
      err.textContent = this._errorMessage;
      card.appendChild(err);
    }
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "welcome-cta";
    btn.textContent = "\u4FDD\u5B58\u5BC6\u94A5 \u2192";
    btn.addEventListener("click", () => {
      const value = input.value.trim();
      void this._doSetApiKey(value);
    });
    card.appendChild(btn);
    requestAnimationFrame(() => input.focus());
    return card;
  }
  _renderReadyCard() {
    const card = document.createElement("div");
    card.className = "welcome-card welcome-ready-card";
    const caption = document.createElement("p");
    caption.className = "welcome-step-caption";
    caption.textContent = "\u5B8C\u6210 / READY";
    card.appendChild(caption);
    const h2 = document.createElement("h2");
    h2.className = "welcome-step-title";
    h2.textContent = "\u4F60\u7684\u5DE5\u4F5C\u7AD9\u5DF2\u5C31\u7EEA\u3002";
    card.appendChild(h2);
    const sub = document.createElement("p");
    sub.className = "welcome-step-subtitle";
    sub.textContent = '\u4E94\u6B65\u5168\u90E8\u5B8C\u6210\u3002\u70B9\u4E0B\u9762\u5F00\u59CB spec \u8BBF\u8C08\u2014\u2014\u6211\u4F1A\u95EE\u4F60\u516B\u4E2A\u5173\u4E8E"\u65E5\u5E38\u9EBB\u70E6"\u7684\u95EE\u9898\uFF0C\u7136\u540E\u5E2E\u4F60\u751F\u6210\u4E00\u4E2A\u5C0F\u5DE5\u5177\u3002';
    card.appendChild(sub);
    const cta = document.createElement("button");
    cta.type = "button";
    cta.className = "welcome-cta";
    cta.textContent = "\u5F00\u59CB spec \u8BBF\u8C08 \u2192";
    cta.addEventListener("click", () => {
      window.location.assign("/build");
    });
    card.appendChild(cta);
    return card;
  }
};
function register12() {
  if (!customElements.get(ELEMENT_NAME12)) {
    customElements.define(ELEMENT_NAME12, TosWelcome);
  }
}

// src/router.ts
function currentView() {
  const path = location.pathname;
  if (path === "/agents" || path.startsWith("/agents/")) {
    return "agents";
  }
  if (path === "/tasks" || path.startsWith("/tasks/")) {
    return "tasks";
  }
  if (path === "/audit" || path.startsWith("/audit/")) {
    return "audit";
  }
  if (path === "/build" || path.startsWith("/build/")) {
    return "build";
  }
  if (path === "/welcome" || path.startsWith("/welcome/")) {
    return "welcome";
  }
  return "dashboard";
}

// src/turingos-root.ts
var ELEMENT_NAME13 = "turingos-root";
var TuringOSRoot = class extends HTMLElement {
  constructor() {
    super(...arguments);
    /** Cache of received IR by view name. */
    this._cache = /* @__PURE__ */ new Map();
    this._boundListener = null;
  }
  connectedCallback() {
    const view = currentView();
    if (view === "build" || view === "welcome") {
      this._renderInertView();
    } else if (this._cache.size === 0) {
      const p = document.createElement("p");
      p.textContent = "Connecting\u2026";
      this.appendChild(p);
    } else {
      this._renderCurrentView();
    }
    if (view !== "welcome") {
      void this._checkOnboarding();
    }
    this._boundListener = (e) => this._onWsMessage(e);
    document.addEventListener("turingos:ir_update", this._boundListener);
  }
  // W6/W7: pages where another Web Component owns <main>; nothing to render.
  _renderInertView() {
    while (this.firstChild) this.removeChild(this.firstChild);
  }
  /**
   * W7 (hotfix W7.1): poll /api/welcome/status once at mount time; if the user
   * has not finished the four prerequisite onboarding steps (Init, LlmConfig,
   * ApiKey, AgentDeploy), soft-redirect to /welcome.
   *
   * Originally this fired the redirect whenever `next_step !== 'Done'`, which
   * was wrong: once the user finishes onboarding the next_step becomes `Spec`
   * (and later `Generate`), which are reached *on /build itself* — the spec
   * grill is the very thing that flips Spec → Generate → Done. Redirecting
   * away from /build whenever next_step is Spec/Generate trapped the user on
   * /welcome forever after clicking "开始 SPEC 访谈 →".
   *
   * Authoritative rule: only the four wizard-controlled steps can mean
   * "onboarding incomplete; punt user back to /welcome". Spec and Generate
   * are user-driven post-onboarding steps that live on /build.
   *
   * Failure is silent — if the server is offline or the API errors, we leave
   * the user where they are rather than punting them around the app.
   */
  async _checkOnboarding() {
    try {
      const resp = await fetch("/api/welcome/status");
      if (!resp.ok) return;
      const data = await resp.json();
      const onboardingIncomplete = typeof data.next_step === "string" && (data.next_step === "Init" || data.next_step === "LlmConfig" || data.next_step === "ApiKey" || data.next_step === "AgentDeploy");
      if (onboardingIncomplete && location.pathname !== "/welcome") {
        window.location.assign("/welcome");
      }
    } catch {
    }
  }
  disconnectedCallback() {
    if (this._boundListener !== null) {
      document.removeEventListener("turingos:ir_update", this._boundListener);
      this._boundListener = null;
    }
  }
  _onWsMessage(e) {
    const detail = e.detail;
    if (detail == null) return;
    const v = currentView();
    if (v === "build" || v === "welcome") {
      return;
    }
    if (detail.msg_type === "ir_update") {
      if (detail.ir == null) return;
      this._cache.set(detail.view, detail.ir);
      const view = currentView();
      const effectiveView = view === "audit" ? "dashboard" : view;
      if (detail.view === effectiveView) {
        this._renderIr(detail.ir);
      }
    } else if (detail.msg_type === "task_created") {
      setTimeout(() => {
        fetch("/api/tasks").then((r) => r.json()).then((ir) => {
          const irRoot = ir;
          this._cache.set("tasks", irRoot);
          if (currentView() === "tasks") {
            this._renderIr(irRoot);
          }
        }).catch((err) => {
          console.warn("turingos-root: failed to re-fetch /api/tasks after task_created", err);
        });
      }, 200);
    }
  }
  /** Re-render using whatever is in the cache for the current view. */
  _renderCurrentView() {
    const view = currentView();
    const effectiveView = view === "audit" ? "dashboard" : view;
    const cached = this._cache.get(effectiveView);
    if (cached != null) {
      this._renderIr(cached);
    }
  }
  _renderIr(ir) {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    if (currentView() === "tasks") {
      const formEl = document.createElement("tos-task-open-form");
      this.appendChild(formEl);
    }
    for (const block of ir.blocks) {
      const el = buildBlockElement(block);
      if (el !== null) {
        this.appendChild(el);
      }
    }
  }
};
function buildBlockElement(block) {
  let el;
  switch (block.kind) {
    case "text":
      el = document.createElement("tos-text-block");
      break;
    case "table":
      el = document.createElement("tos-table-block");
      break;
    case "agent_card":
      el = document.createElement("tos-agent-card-block");
      break;
    case "task_card":
      el = document.createElement("tos-task-card-block");
      break;
    case "event_log":
      el = document.createElement("tos-event-log-block");
      break;
    case "dashboard_panel":
      el = document.createElement("tos-dashboard-panel-block");
      break;
    default:
      return null;
  }
  el.dataset["payload"] = JSON.stringify(block);
  return el;
}
function register13() {
  if (!customElements.get(ELEMENT_NAME13)) {
    customElements.define(ELEMENT_NAME13, TuringOSRoot);
  }
}

// src/main.ts
register();
register2();
register3();
register4();
register5();
register6();
register7();
register8();
register9();
register10();
register11();
register12();
register13();
document.addEventListener("DOMContentLoaded", () => {
  console.info("TuringOS frontend ready, view:", currentView());
});
