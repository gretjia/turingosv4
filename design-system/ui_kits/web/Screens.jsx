// TuringOS Web UI kit — /, /agents, /tasks, /audit screens.
//
// All four IR-driven views share a single block-rendering model. Each block
// gets a [data-block-type] root and the production CSS handles the rest.

const { Fragment: FragmentD, useState: useStateD } = React;

/* ────────────────────────────────────────────────────────────
   Text block.
   ──────────────────────────────────────────────────────────── */
function TextBlock({ paragraphs }) {
  return (
    <section data-block-type="text">
      {paragraphs.map((p, i) => <p key={i}>{p}</p>)}
    </section>
  );
}

/* ────────────────────────────────────────────────────────────
   Dashboard panel — top-rule heading + Fraunces metric grid.
   ──────────────────────────────────────────────────────────── */
function DashboardPanel({ title, metrics }) {
  return (
    <section data-block-type="dashboard_panel">
      <h3 className="panel-title">{title}</h3>
      <dl className="metrics">
        {metrics.map((m, i) => (
          <div key={i}>
            <dt>{m.label}</dt>
            <dd>
              {m.value}
              {m.unit && <span className="unit">{m.unit}</span>}
            </dd>
          </div>
        ))}
      </dl>
    </section>
  );
}

/* ────────────────────────────────────────────────────────────
   Event log — L4 / L4-Exception rows with grid alignment.
   ──────────────────────────────────────────────────────────── */
function EventLog({ events }) {
  return (
    <section data-block-type="event_log">
      <ol>
        {events.map((ev, i) => (
          <li key={i} className={`event layer-${ev.layer}`}>
            <span className="layer">{ev.layer}</span>
            <span className="kind">{ev.kind}</span>
            <span className="tx-id">{ev.txId}</span>
            <span className="summary">{ev.summary}</span>
          </li>
        ))}
      </ol>
    </section>
  );
}

/* ────────────────────────────────────────────────────────────
   Table block — mono headers + tabular-nums values + cell kinds.
   ──────────────────────────────────────────────────────────── */
function TableBlock({ caption, columns, rows }) {
  return (
    <section data-block-type="table">
      {caption && <figcaption>{caption}</figcaption>}
      <table>
        <thead>
          <tr>{columns.map((c, i) => <th key={i}>{c.label}</th>)}</tr>
        </thead>
        <tbody>
          {rows.map((row, ri) => (
            <tr key={ri}>
              {columns.map((c, ci) => (
                <td key={ci} data-cell-kind={c.kind || undefined}>
                  {typeof row[c.key] === "object" ? row[c.key] : row[c.key]}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </section>
  );
}

/* ────────────────────────────────────────────────────────────
   Agent card — hairline + 3 px accent left stripe + dl metadata.
   ──────────────────────────────────────────────────────────── */
function AgentCard({ id, role, pubkey, balance, status }) {
  return (
    <article data-block-type="agent_card">
      <header>
        <span className="tos-card-id">{id}</span>
        <span className="tos-card-role">{role}</span>
      </header>
      <dl>
        <dt>pubkey</dt>
        <dd><ShortHash value={pubkey} head={10} tail={6} /></dd>
        <dt>balance</dt>
        <dd>{balance}<span className="caption" style={{ marginLeft: 6 }}> µC</span></dd>
        <dt>status</dt>
        <dd><StatusBadge status={status} /></dd>
      </dl>
    </article>
  );
}

/* ────────────────────────────────────────────────────────────
   Task card — hairline + 3 px amber left stripe.
   ──────────────────────────────────────────────────────────── */
function TaskCard({ id, problem, bounty, agent, status }) {
  return (
    <article data-block-type="task_card">
      <header>
        <span className="tos-card-id">{id}</span>
        <StatusBadge status={status} />
      </header>
      <dl>
        <dt>problem</dt>
        <dd style={{ fontFamily: "var(--font-body)", fontSize: "var(--fs-sm)" }}>{problem}</dd>
        <dt>bounty</dt>
        <dd>{bounty}<span className="caption" style={{ marginLeft: 6 }}> µC</span></dd>
        <dt>agent</dt>
        <dd>{agent}</dd>
      </dl>
    </article>
  );
}

/* ────────────────────────────────────────────────────────────
   Screens — each composes the IR blocks for a route.
   ──────────────────────────────────────────────────────────── */
function DashboardScreen({ onNavigate }) {
  return (
    <PageShell active="dashboard" onNavigate={onNavigate}>
      <PageTitle
        title="账本桌面 · 当前快照"
        id="dashboard · phase 7 w4.4 · materialized view"
      />
      <DashboardPanel
        title="Tape · current epoch"
        metrics={[
          { label: "capsules",  value: "1 247", unit: null },
          { label: "agents",    value: "8", unit: "/ 12" },
          { label: "bounty",    value: "42.6", unit: "µC" },
          { label: "last tick", value: "02:17", unit: null },
        ]}
      />
      <DashboardPanel
        title="Spec · last 24 h"
        metrics={[
          { label: "interviews",   value: "12", unit: null },
          { label: "spec capsules", value: "9", unit: null },
          { label: "rejections",   value: "4", unit: null },
          { label: "median turns", value: "6.5", unit: null },
        ]}
      />
      <EventLog events={[
        { layer: "L4",  kind: "SpecCapsule.written",   txId: "tx_a3f0…8c11", summary: "schema=turingos-spec-capsule-v1 · slots=7/7" },
        { layer: "L4",  kind: "GenerationAttempt.ran", txId: "tx_b91c…0e44", summary: "session=abc123 · 7 files · world_head unchanged" },
        { layer: "L4E", kind: "GenerateRejected",      txId: "tx_b7e2…11d9", summary: "reject_class=heuristic_failed · world_head_unchanged" },
        { layer: "L4",  kind: "PromptPromotionReceipt",txId: "tx_47e9…62a1", summary: "prompt_cid=bafy2bz…d2f1 · accepted by L1 verifier" },
        { layer: "L4",  kind: "TestRunCapsule.written",txId: "tx_c142…73a8", summary: "scenarios=3/3 accepted · hidden-oracle shielded" },
      ]} />
    </PageShell>
  );
}

function AgentsScreen({ onNavigate }) {
  return (
    <PageShell active="agents" onNavigate={onNavigate}>
      <PageTitle title="Agents" id="agents · phase 7 · multi-agent ledger" />
      <div style={{ display: "grid", gap: "var(--space-3)" }}>
        <AgentCard id="agent_001" role="solver"   pubkey="ed25519:5b8c1f9a2d…a23f" balance="1 240 000" status="active" />
        <AgentCard id="agent_002" role="verifier" pubkey="ed25519:c4e211d987…7822" balance="  920 000" status="active" />
        <AgentCard id="agent_003" role="solver"   pubkey="ed25519:9f4188aa0c…be11" balance="   12 400" status="paused" />
        <AgentCard id="agent_004" role="judge"    pubkey="ed25519:7102ee44df…2c91" balance="        0" status="bankrupt" />
      </div>
    </PageShell>
  );
}

function TasksScreen({ onNavigate }) {
  // Inline mini "open new task" form replicated from base-styles.css [data-block-type="task_open_form"]
  const [problem, setProblem] = useStateD("");
  const [bounty,  setBounty]  = useStateD("");
  const [status,  setStatus]  = useStateD(null);

  const submit = (e) => {
    e.preventDefault();
    if (!problem.trim() || !bounty.trim()) {
      setStatus({ kind: "error", text: "problem 与 bounty 都不能为空" });
      return;
    }
    setStatus({ kind: "created", text: `task_id=tk_${Math.random().toString(36).slice(2, 8)} · 已写入 chaintape` });
    setProblem(""); setBounty("");
  };

  return (
    <PageShell active="tasks" onNavigate={onNavigate}>
      <PageTitle title="Tasks · 任务市场" id="tasks · phase 7 w4 · write path" />

      <section data-block-type="task_open_form">
        <form onSubmit={submit}>
          <div className="field">
            <label htmlFor="task-problem">problem</label>
            <input id="task-problem" type="text" value={problem} placeholder="e.g. minif2f_v4_n123" onChange={(e) => setProblem(e.target.value)} />
          </div>
          <div className="field">
            <label htmlFor="task-bounty">bounty (µC)</label>
            <input id="task-bounty" type="text" value={bounty} placeholder="1200" onChange={(e) => setBounty(e.target.value)} />
          </div>
          <button type="submit">open task →</button>
          {status && <p data-status={status.kind}>{status.text}</p>}
        </form>
      </section>

      <div style={{ display: "grid", gap: "var(--space-3)" }}>
        <TaskCard id="tk_a4f0c2" problem="minif2f_v4_n091 · sum-of-cubes identity" bounty="4 000" agent="agent_001" status="open" />
        <TaskCard id="tk_91ba8d" problem="putnam_2025_b3 · contraction-on-square" bounty="6 200" agent="agent_002" status="accepted" />
        <TaskCard id="tk_c773e1" problem="nesbitt · cyclic-fraction bound"        bounty="2 800" agent="agent_001" status="solved"   />
        <TaskCard id="tk_28d014" problem="minif2f_v4_n104 · diophantine triple"   bounty="3 000" agent="agent_003" status="rejected" />
      </div>
    </PageShell>
  );
}

function AuditScreen({ onNavigate }) {
  return (
    <PageShell active="audit" onNavigate={onNavigate}>
      <PageTitle title="Audit · ChainTape 审计视图" id="audit · phase 7 · replay-derived" />
      <TextBlock paragraphs={[
        "本页是 CAS + ChainTape 的可重放视图。任何条目都来自 turingos replay --offline，并对 trust_root 做交叉校验。",
        "数字与状态都不是权威——权威永远是 tape 自身。这里只是把 tape 渲染成人类可读的版本。",
      ]} />
      <TableBlock
        caption="Capsules · 最近一个 epoch"
        columns={[
          { key: "cid",       label: "cid",       kind: "cid" },
          { key: "schema",    label: "schema",    kind: null },
          { key: "writer",    label: "writer",    kind: "agent_id" },
          { key: "world_head",label: "world_head",kind: null },
          { key: "size",      label: "size",      kind: "integer" },
        ]}
        rows={[
          { cid: "bafy2bz9p3q…f1c2", schema: "turingos-spec-capsule-v1",        writer: "agent_001", world_head: "unchanged", size: "2 412" },
          { cid: "bafy2bz1e8w…d44a", schema: "turingos-artifact-bundle-v1",     writer: "agent_001", world_head: "advanced",  size: "12 880" },
          { cid: "bafy2bz4kq2…77c1", schema: "turingos-generate-rejection-v1",  writer: "agent_003", world_head: "unchanged", size: "1 102" },
          { cid: "bafy2bz0za7…b830", schema: "turingos-test-run-v1",            writer: "agent_002", world_head: "unchanged", size: "3 904" },
          { cid: "bafy2bz77c4…1109", schema: "turingos-preview-run-v1",         writer: "agent_002", world_head: "unchanged", size: "  420" },
        ]}
      />
    </PageShell>
  );
}

Object.assign(window, {
  TextBlock, DashboardPanel, EventLog, TableBlock, AgentCard, TaskCard,
  DashboardScreen, AgentsScreen, TasksScreen, AuditScreen,
});
