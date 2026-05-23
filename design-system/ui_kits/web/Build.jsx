// TuringOS Web UI kit — /build spec-grill interview screen.
//
// Faithful recreation of frontend/src/components/spec-grill.ts. The LLM-driven
// turn-by-turn flow is mocked with a hard-coded canonical question set so the
// click-thru can show idle → interviewing → complete → spec result → preview.

const { useState: useStateB, Fragment: FragmentB } = React;

// Canonical questions lifted from runtime/grill_envelope's seven required slots.
// In production these are picked by the Meta LLM (DeepSeek V3.2) one at a time;
// here we cycle through statically for the mock.
const GRILL_QUESTIONS = [
  {
    slot: "trigger",
    q: "你一周里反复做、还心里嘀咕「这事儿能不能让电脑自己做」的，是哪件？",
    placeholder: "比如：每周把电费账单截图归档…",
  },
  {
    slot: "frequency",
    q: "这件事每周大概要花你多少时间？",
    placeholder: "一两个小时？十分钟？",
  },
  {
    slot: "current_workflow",
    q: "你现在是怎么做的——一步一步说？",
    placeholder: "打开微信、截图、保存到桌面…",
  },
  {
    slot: "pain",
    q: "现在的流程里，最烦的那一步是什么？",
    placeholder: "命名文件 / 分类 / 同步到云盘…",
  },
  {
    slot: "success_signal",
    q: "如果这件事真的被自动化了，你怎么知道它工作得好？",
    placeholder: "比如：每周一打开文件夹，截图都齐了",
  },
  {
    slot: "constraint",
    q: "有什么是绝对不能被它搞坏的？",
    placeholder: "已有的文件不要被覆盖 / 隐私不要外泄…",
  },
  {
    slot: "non_goal",
    q: "你故意不想让它做什么——即使技术上能做？",
    placeholder: "比如：不要自动发到群里",
  },
];

function SpecGrillIdle({ onStart }) {
  return (
    <section className="spec-grill-idle">
      <p className="spec-grill-eyebrow">spec grill · 八个关于「麻烦」的问题</p>
      <p className="spec-grill-lede">
        从一段闲聊开始。我会一道一道问你——不会一次问八道，每道题的下一道都根据你刚才的回答即时决定。
        最后我会写出一份 spec.md，然后帮你生成一个能跑的小工具。
      </p>
      <button type="button" className="spec-grill-cta" onClick={onStart}>
        开始 spec 访谈 →
      </button>
    </section>
  );
}

function SpecGrillQuestion({ idx, total, onNext, onBack }) {
  const [val, setVal] = useStateB("");
  const item = GRILL_QUESTIONS[idx];
  const canAdvance = val.trim().length > 4;

  return (
    <section className="spec-grill-question">
      <p className="spec-grill-progress">turn {String(idx + 1).padStart(2, "0")} · {String(total).padStart(2, "0")}</p>
      <p className="spec-grill-question-text">{item.q}</p>
      <textarea
        className="spec-grill-input"
        rows={3}
        placeholder={item.placeholder}
        value={val}
        onChange={(e) => setVal(e.target.value)}
        autoFocus
      />
      <div className="spec-grill-footer">
        {idx > 0 && (
          <button type="button" className="spec-grill-back" onClick={onBack}>
            ← 上一题
          </button>
        )}
        <button
          type="button"
          className="spec-grill-advance"
          onClick={() => canAdvance && onNext(val)}
          disabled={!canAdvance}
        >
          {idx === total - 1 ? "生成 spec.md →" : "下一题 →"}
        </button>
      </div>
    </section>
  );
}

function SpecGrillLoading() {
  return <LoadingPhrase>正在合成 spec.md</LoadingPhrase>;
}

function SpecResult({ answers, onGenerate }) {
  const trigger = answers[0] || "（待生成）";
  const pain = answers[3] || "";
  const success = answers[4] || "";
  const constraint = answers[5] || "";
  const nonGoal = answers[6] || "";

  return (
    <article data-block-type="spec_result">
      <div className="spec-result-article">
        <h1><em>spec.md · {trigger.slice(0, 28)}{trigger.length > 28 ? "…" : ""}</em></h1>
        <h3>一句话目标 · One-line goal</h3>
        <p>把 <strong>{trigger}</strong> 这件事自动化成一个小工具——具体目标见 §Goal。</p>
        <h3>立刻能做的 · Build now</h3>
        <ul>
          <li>读取触发源，按规则归档</li>
          <li>命名遵循 <code>YYYY-MM-DD_<em>topic</em>.ext</code></li>
          <li>每周一次摘要到本地 markdown</li>
        </ul>
        <h3>更深的洞察 · Deeper insight</h3>
        <p>{pain || "用户真正烦的不是任务本身——而是手动命名 / 分类 / 同步的中间步骤。"}</p>
        <h3>算成功 · Acceptance</h3>
        <p>{success || "每周一早上能打开一个干净的文件夹，本周所有项都齐了。"}</p>
        <h3>不能搞坏 · Robustness</h3>
        <p>{constraint || "已有的文件不被覆盖；隐私字段不被外泄到日志。"}</p>
        <h3>故意不做 · Out of scope</h3>
        <p>{nonGoal || "不要把结果自动转发到群组里。"}</p>
      </div>
      <p className="spec-result-cid">
        <span className="spec-result-cid-label">cid</span>
        <code>bafy2bzace4e7y3o6f…s4qm</code>
      </p>
      <div className="spec-result-cta">
        <button type="button" className="spec-result-generate-btn" onClick={onGenerate}>
          生成代码 →
        </button>
      </div>
    </article>
  );
}

function ArtifactViewer({ onBack }) {
  return (
    <article data-block-type="artifact_viewer">
      <header className="artifact-viewer-header">
        <p className="artifact-viewer-eyebrow">artifact · 一次生成 · 七个文件</p>
        <h1 className="artifact-viewer-title">你的工具已生成。</h1>
      </header>
      <div className="artifact-viewer-layout">
        <ul className="artifact-viewer-filelist">
          <li className="is-selected"><button type="button">index.html</button></li>
          <li><button type="button">main.js</button></li>
          <li><button type="button">styles.css</button></li>
          <li><button type="button">archive.py</button></li>
          <li><button type="button">README.md</button></li>
          <li><button type="button">.gitignore</button></li>
          <li><button type="button">spec.md</button></li>
        </ul>
        <div className="artifact-viewer-main">
          <div className="artifact-viewer-iframe" style={{ display:"flex", alignItems:"center", justifyContent:"center", flexDirection:"column", gap:"var(--space-4)", padding:"var(--space-7)" }}>
            <span style={{
              fontFamily:"var(--font-display)", fontStyle:"italic", fontSize:"var(--fs-2xl)",
              fontVariationSettings:"'opsz' 60, 'SOFT' 50", color:"var(--fg-muted)", textAlign:"center",
              lineHeight: 1.2, maxWidth: "32ch",
            }}>
              archive.html · sandbox preview
            </span>
            <span className="caption">iframe sandbox=&quot;allow-scripts&quot; only</span>
          </div>
          <p className="artifact-viewer-caption">
            <span>file ·</span>
            <span className="artifact-viewer-caption-path">&nbsp;sessions/abc123/index.html</span>
          </p>
          <a href="#" className="artifact-viewer-download" onClick={(e) => e.preventDefault()}>
            download bundle
          </a>
        </div>
      </div>
      <div style={{ marginTop: "var(--space-7)", display: "flex", justifyContent: "space-between" }}>
        <button type="button" className="spec-grill-back" onClick={onBack}>← 回到 spec</button>
        <span className="caption">artifact_bundle_cid · bafy2bz9p3q…f1c2</span>
      </div>
    </article>
  );
}

/* ────────────────────────────────────────────────────────────
   BuildScreen — full /build chrome + spec-grill state machine.
   ──────────────────────────────────────────────────────────── */
function BuildScreen({ onNavigate }) {
  const [phase, setPhase] = useStateB("idle"); // idle | interviewing | loading | complete | artifact
  const [turnIdx, setTurnIdx] = useStateB(0);
  const [answers, setAnswers] = useStateB([]);

  const startInterview = () => { setPhase("interviewing"); setTurnIdx(0); setAnswers([]); };
  const nextTurn = (val) => {
    const next = [...answers, val];
    setAnswers(next);
    if (turnIdx === GRILL_QUESTIONS.length - 1) {
      setPhase("loading");
      setTimeout(() => setPhase("complete"), 1100);
    } else {
      setTurnIdx(turnIdx + 1);
    }
  };
  const back = () => { if (turnIdx > 0) setTurnIdx(turnIdx - 1); };
  const generate = () => { setPhase("loading"); setTimeout(() => setPhase("artifact"), 950); };

  let body;
  if (phase === "idle") body = <SpecGrillIdle onStart={startInterview} />;
  else if (phase === "interviewing") body = (
    <SpecGrillQuestion idx={turnIdx} total={GRILL_QUESTIONS.length} onNext={nextTurn} onBack={back} />
  );
  else if (phase === "loading") body = <SpecGrillLoading />;
  else if (phase === "complete") body = <SpecResult answers={answers} onGenerate={generate} />;
  else body = <ArtifactViewer onBack={() => setPhase("complete")} />;

  return (
    <PageShell active="build" onNavigate={onNavigate} mainClassName="tos-main-build">
      <PageTitle
        title="从一段闲聊开始，做出你想要的那个小工具。"
        id="build · spec interview · phase 7 w6"
      />
      <section data-block-type="spec_grill">
        {body}
      </section>
    </PageShell>
  );
}

Object.assign(window, { BuildScreen, GRILL_QUESTIONS });
