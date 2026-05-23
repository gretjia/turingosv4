// TuringOS Web UI kit — chrome primitives.
//
// Recreated from turingosv4/frontend/src/components/*.ts and src/web/render.rs
// (server-side HTML chrome). Visual fidelity is the priority; the click logic
// is mock-only.
//
// All components attach themselves to `window` at the end of this file so
// other Babel script blocks can use them.

const { useState, useEffect, useRef, useCallback, Fragment } = React;

/* ────────────────────────────────────────────────────────────
   Wordmark — Fraunces 900 + rotated 10 px accent square +
   hairline "Phase 7" mono pill. Matches .tos-wordmark.
   ──────────────────────────────────────────────────────────── */
function Wordmark({ href = "/", sub = "Phase 7" }) {
  return (
    <a className="tos-wordmark" href={href} aria-label={`TuringOS — ${sub} home`}>
      TuringOS<span className="tos-wordmark-sub">{sub}</span>
    </a>
  );
}

/* ────────────────────────────────────────────────────────────
   Header — wordmark + small mono meta string on the right.
   ──────────────────────────────────────────────────────────── */
function Header({ meta = "FC3-N31 · materialized view" }) {
  return (
    <header className="tos-header" role="banner">
      <Wordmark />
      <span className="tos-meta">{meta}</span>
    </header>
  );
}

/* ────────────────────────────────────────────────────────────
   Nav — five top-level views. Active gets aria-current + accent
   bottom border. Click is intercepted so we can route inside SPA.
   ──────────────────────────────────────────────────────────── */
const NAV_ITEMS = [
  { key: "dashboard", label: "Dashboard", href: "/" },
  { key: "agents",    label: "Agents",    href: "/agents" },
  { key: "tasks",     label: "Tasks",     href: "/tasks" },
  { key: "audit",     label: "Audit",     href: "/audit" },
  { key: "build",     label: "Build",     href: "/build" },
];

function Nav({ active, onNavigate }) {
  return (
    <nav className="tos-nav" aria-label="primary">
      {NAV_ITEMS.map((item) => {
        const isActive = item.key === active;
        return (
          <a
            key={item.key}
            href={item.href}
            aria-current={isActive ? "page" : undefined}
            onClick={(e) => { e.preventDefault(); onNavigate?.(item.key); }}
          >
            {item.label}
          </a>
        );
      })}
    </nav>
  );
}

/* ────────────────────────────────────────────────────────────
   Footer — hairline-top, materialized-view notice, connection pill.
   ──────────────────────────────────────────────────────────── */
function Footer({ connection = "connected" }) {
  return (
    <footer className="tos-footer" role="contentinfo">
      <span className="tos-footer-notice">
        FC3-N31: materialized view — not authoritative over ChainTape/CAS.
      </span>
      <ConnectionPill state={connection} />
    </footer>
  );
}

function ConnectionPill({ state = "connected" }) {
  const labelByState = {
    connected: "connected",
    connecting: "connecting",
    reconnecting: "reconnecting",
    disconnected: "disconnected",
  };
  return (
    <span className="tos-conn-pill" data-state={state}>
      <span className="tos-conn-dot" aria-hidden="true"></span>
      {labelByState[state] || state}
    </span>
  );
}

/* ────────────────────────────────────────────────────────────
   Status badge — dot + uppercase mono caps. Never icon-only.
   ──────────────────────────────────────────────────────────── */
function StatusBadge({ status, children }) {
  return (
    <span className="tos-status" data-status={status}>
      {children || status}
    </span>
  );
}

/* ────────────────────────────────────────────────────────────
   Loading dots — three Fraunces italic middle-dots, opacity pulse.
   ──────────────────────────────────────────────────────────── */
function LoadingPhrase({ children }) {
  return (
    <div className="spec-grill-loading">
      <p className="spec-grill-loading-phrase">
        {children}
        <span className="spec-grill-dots" aria-hidden="true">
          <span>·</span><span>·</span><span>·</span>
        </span>
      </p>
    </div>
  );
}

/* ────────────────────────────────────────────────────────────
   Page shell — header + nav + main + footer. Used by every screen
   except /welcome (which has no nav).
   ──────────────────────────────────────────────────────────── */
function PageShell({ active, onNavigate, connection, mainClassName, children }) {
  return (
    <div className="tos-page-wrap">
      <Header />
      <Nav active={active} onNavigate={onNavigate} />
      <main className={`tos-main ${mainClassName || ""}`} id="tos-main" role="main">
        {children}
      </main>
      <Footer connection={connection} />
    </div>
  );
}

/* Page title + monospace id row (matches .tos-page-title / .tos-page-id). */
function PageTitle({ title, id }) {
  return (
    <Fragment>
      <h1 className="tos-page-title">{title}</h1>
      <p className="tos-page-id">{id}</p>
    </Fragment>
  );
}

/* ────────────────────────────────────────────────────────────
   Reusable bits used inside cards.
   ──────────────────────────────────────────────────────────── */
function Dl({ rows }) {
  return (
    <dl>
      {rows.map(([dt, dd], i) => (
        <Fragment key={i}>
          <dt>{dt}</dt>
          <dd>{dd}</dd>
        </Fragment>
      ))}
    </dl>
  );
}

/* Truncate-middle for hashes / cids — keeps first 6 + last 4 chars. */
function ShortHash({ value, head = 6, tail = 4 }) {
  if (!value || value.length <= head + tail + 1) return <span className="hash">{value}</span>;
  return <span className="hash">{value.slice(0, head)}…{value.slice(-tail)}</span>;
}

Object.assign(window, {
  Wordmark, Header, Nav, Footer, ConnectionPill,
  StatusBadge, LoadingPhrase, PageShell, PageTitle,
  Dl, ShortHash,
  NAV_ITEMS,
});
