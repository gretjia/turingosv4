//! TRACE_MATRIX FC2-N16 + FC3-N4: spec.md synthesis helpers (library API).
//!
//! Phase 6.3.y A6 (2026-05-19): hoisted from `src/bin/turingos/cmd_spec.rs`
//! so the `turingos_web` binary can run the same LLM-less synthesis path the
//! CLI driven mode uses (`cmd_spec::run_driven_mode`). Closes the F6 deferred
//! atom that left web sessions terminating with
//! `termination_reason: "predicate_done_no_spec_pending_synthesis"` and no
//! `spec_capsule_cid`.
//!
//! This module deliberately exposes ONLY the small set of pure helpers needed
//! to render an LLM-less spec.md from canonical 8-question answers:
//!
//!   - [`canonical_questions`] — frozen Mom-Test 8-question vector (zh / en).
//!   - [`synthesise_spec_md_no_llm`] — deterministic body builder used when no
//!     synthesis LLM is reachable.
//!   - [`wrap_spec_md`] — header + appendix wrapper that the SpecCapsule
//!     hashes; spec.md on disk and the CAS bytes are identical.
//!
//! All three are byte-identical to their pre-A6 cmd_spec.rs originals; the
//! `Lang` parameter now refers to the library `crate::runtime::grill_predicates::Lang`
//! enum (previously a private `cmd_spec::Lang` mirror — same Zh/En shape).

use crate::runtime::grill_predicates::Lang;
use std::collections::BTreeMap;

/// TRACE_MATRIX FC1-N5 / FC2-N16: canonical 8-question Mom-Test grill (zh + en).
///
/// Verbatim copy of `cmd_spec::canonical_questions` (pre-A6 src/bin/turingos/cmd_spec.rs
/// line ~1452). Used by both the CLI driven path and the web in-process path so
/// the spec.md appendix is identical regardless of which front-end drove the
/// session.
pub fn canonical_questions(lang: Lang) -> Vec<String> {
    match lang {
        Lang::Zh => vec![
            // Q1 — The Job (JTBD opener; no jargon)
            "先不用想程序怎么做。能跟我说说你最近遇到了什么事，让你觉得『要是有个小工具就好了』？\
比如『我妈每周要算一次社区团购账，Excel 太麻烦』。你的故事是什么？".into(),
            // Q2 — The Anchor (let user supply anchor)
            "有没有哪个网站 / App / 小工具，跟你想要的『有点像』？不用一模一样，一两个相似的地方就行。\
（如果想不出来：那纸笔 / Excel / 微信群里现在是怎么做的？）".into(),
            // Q3 — Data model in plain words
            "想象关掉电脑明天再打开，这个工具应该还『记得』哪些东西？比如团购账本会记得：\
每个人的名字、买了什么、付了多少、还欠多少。你的工具要记得什么？".into(),
            // Q4 — First-click walkthrough
            "假设我是你的用户，第一次打开这个工具——我看到什么？然后我点什么？然后呢？\
一步一步告诉我，直到我完成一件事。".into(),
            // Q5 — Weird-user test (Mom-Test sin-3 antidote, specifics)
            "如果有个奇怪的用户，故意乱点乱填——比如把『金额』填成『哈哈哈』，\
或者同一个名字录入 50 遍——你希望工具怎么办？报错？忽略？还是有别的反应？".into(),
            // Q6 — Disappointment boundary (inverse framing surfaces real priorities)
            "如果这个工具突然多了一个功能，你反而会觉得『搞这个干嘛，反而把简单的事弄复杂了』——\
是什么功能？说两三个。".into(),
            // Q7 — Success test (past-cost framing)
            "用了一个月之后，你怎么判断『这个工具是有用的』？不是『感觉不错』那种——\
是具体能数出来或看得见的事。比如：『我妈现在不用每周日花两小时算账了。』".into(),
            // Q8 — Playback / mirror (Voss labeling)
            "（最后一题）下面我会把前面听到的复述一遍，请你看看哪里我听错了或听漏了——\
别客气，挑错就是帮我。如果你想直接补充什么，请在这里写出来。".into(),
        ],
        Lang::En => vec![
            "Forget about code for now. Tell me about a recent moment when you thought \
'I wish I had a tool for this.' For example: 'My mom does community group-buy accounting \
every week in Excel and it's painful.' What's your story?".into(),
            "Is there a website, app, or tool that's even a little bit like what you want? \
Doesn't have to be exact — just one or two similar pieces. (If you can't name one: \
'How do you do this today with paper, Excel, or a chat group?')".into(),
            "Imagine you close the program and open it tomorrow — what should it still \
'remember'? A group-buy tracker remembers: each person's name, what they bought, how \
much they paid, what they still owe. What does yours remember?".into(),
            "Pretend I'm your user opening this for the first time. What do I see? What do \
I click? Then what? Walk me through, step by step, until I finish one task.".into(),
            "If a weird user messes around — types 'lolol' into the price field, or enters \
the same name 50 times — what should the tool do? Show an error? Ignore it? Something else?".into(),
            "If the tool grew a new feature and your reaction was 'why did you add this, \
you've made the simple thing complicated' — name two or three such features.".into(),
            "After one month of using it, how do you know it's actually working? Not 'feels \
nice' — something countable or visible. Like: 'My mom no longer spends two hours every \
Sunday doing the math.'".into(),
            "(Last question) I'll play back what I heard. Tell me which line is wrong or \
incomplete — corrections help me. If you want to add anything directly, write it here.".into(),
        ],
    }
}

/// TRACE_MATRIX FC2-N16: deterministic LLM-less spec.md body synthesiser.
///
/// Verbatim copy of `cmd_spec::synthesise_spec_md_no_llm` (pre-A6 src/bin/turingos/cmd_spec.rs
/// line ~1659). Used both as the `--skip-llm` CAS-wire smoke fallback in the
/// CLI and as the primary synthesis path in the web layer (which currently
/// does not call the Meta synthesis LLM). Pads short / over-long answer lists
/// to exactly 8 slots by repeating the last placeholder.
pub fn synthesise_spec_md_no_llm(lang: Lang, questions: &[String], answers: &[String]) -> String {
    let mut s = String::new();
    match lang {
        Lang::Zh => {
            s.push_str("## 一句话目标\n\n");
            s.push_str(&answers[0]);
            s.push_str("\n\n## 我们要做什么 (Goal)\n\n");
            s.push_str(&answers[0]);
            s.push_str("\n\n## 像谁 (Reference)\n\n");
            s.push_str(&answers[1]);
            s.push_str("\n\n## 程序要记住的东西 (Memory)\n\n");
            s.push_str(&answers[2]);
            s.push_str("\n\n## 第一次使用 (First Run)\n\n");
            s.push_str(&answers[3]);
            s.push_str("\n\n## 不能搞坏的情况 (Robustness)\n\n");
            s.push_str(&answers[4]);
            s.push_str("\n\n## 故意不做的 (Out of Scope)\n\n");
            s.push_str(&answers[5]);
            s.push_str("\n\n## 算成功 (Acceptance)\n\n");
            s.push_str(&answers[6]);
            s.push_str("\n\n## 用户补充\n\n");
            s.push_str(&answers[7]);
            s.push_str("\n\n## 一句话给 AI 编程员\n\n");
            s.push_str("根据上面的 Goal / Memory / First Run 实现一个最小可用版本。");
        }
        Lang::En => {
            s.push_str("## One-line Goal\n\n");
            s.push_str(&answers[0]);
            s.push_str("\n\n## What We're Building (Goal)\n\n");
            s.push_str(&answers[0]);
            s.push_str("\n\n## Like What (Reference)\n\n");
            s.push_str(&answers[1]);
            s.push_str("\n\n## What the Program Remembers\n\n");
            s.push_str(&answers[2]);
            s.push_str("\n\n## First Run\n\n");
            s.push_str(&answers[3]);
            s.push_str("\n\n## What It Must Not Break On\n\n");
            s.push_str(&answers[4]);
            s.push_str("\n\n## Deliberately NOT Doing\n\n");
            s.push_str(&answers[5]);
            s.push_str("\n\n## Success Looks Like\n\n");
            s.push_str(&answers[6]);
            s.push_str("\n\n## User Additions\n\n");
            s.push_str(&answers[7]);
            s.push_str("\n\n## One-line Brief to AI Coder\n\n");
            s.push_str("Implement a minimal version using the Goal / Memory / First Run above.");
        }
    }
    let _ = questions; // questions only used in the LLM-driven path; appendix is rendered by wrap_spec_md
    s.push_str("\n\n<!-- TURINGOS_SPEC_END -->\n");
    s
}

/// TRACE_MATRIX FC2-N16: slot-keyed spec.md body synthesiser (F10, 2026-05-19).
///
/// Fix for D-NEW-3a (Π4.3 P7 + Π4.4 S11): the positional
/// [`synthesise_spec_md_no_llm`] assumes `answers[i]` addresses the i-th
/// canonical question. That assumption holds in the CLI driven path (which asks
/// the 8 canonical questions in fixed order), but in the web layer the LLM
/// drives slot ordering adaptively — so the user's N-th answer addresses
/// whichever slot the LLM was probing at turn N, NOT canonical position N.
///
/// This function takes a slot→answer map (the SOURCE OF TRUTH is the LLM's
/// per-turn `covered_slots` delta) and renders each section against its
/// canonical slot id. Missing slots render a typed placeholder.
///
/// The on-disk spec.md and the SpecCapsule CAS bytes are byte-stable for a
/// given `slot_evidence` input regardless of insertion order (`BTreeMap` is
/// not relied upon for ordering — section ordering is fixed below).
pub fn synthesise_spec_md_no_llm_by_slot(
    lang: Lang,
    slot_evidence: &BTreeMap<String, String>,
) -> String {
    fn pick<'a>(slot_evidence: &'a BTreeMap<String, String>, slot: &str, lang: Lang) -> &'a str {
        slot_evidence
            .get(slot)
            .map(|s| s.as_str())
            .unwrap_or(match lang {
                Lang::Zh => "（用户未在本轮访谈中提供该信息）",
                Lang::En => "(user did not provide this information in the interview)",
            })
    }
    let mut s = String::new();
    match lang {
        Lang::Zh => {
            s.push_str("## 一句话目标\n\n");
            s.push_str(pick(slot_evidence, "job", lang));
            s.push_str("\n\n## 我们要做什么 (Goal)\n\n");
            s.push_str(pick(slot_evidence, "job", lang));
            s.push_str("\n\n## 像谁 (Reference)\n\n");
            s.push_str(pick(slot_evidence, "anchor", lang));
            s.push_str("\n\n## 程序要记住的东西 (Memory)\n\n");
            s.push_str(pick(slot_evidence, "memory", lang));
            s.push_str("\n\n## 第一次使用 (First Run)\n\n");
            s.push_str(pick(slot_evidence, "first_run", lang));
            s.push_str("\n\n## 不能搞坏的情况 (Robustness)\n\n");
            s.push_str(pick(slot_evidence, "robustness", lang));
            s.push_str("\n\n## 故意不做的 (Out of Scope)\n\n");
            s.push_str(pick(slot_evidence, "scope", lang));
            s.push_str("\n\n## 算成功 (Acceptance)\n\n");
            s.push_str(pick(slot_evidence, "acceptance", lang));
            s.push_str("\n\n## 用户补充\n\n");
            s.push_str(pick(slot_evidence, "mirror", lang));
            s.push_str("\n\n## 一句话给 AI 编程员\n\n");
            s.push_str("根据上面的 Goal / Memory / First Run 实现一个最小可用版本。");
        }
        Lang::En => {
            s.push_str("## One-line Goal\n\n");
            s.push_str(pick(slot_evidence, "job", lang));
            s.push_str("\n\n## What We're Building (Goal)\n\n");
            s.push_str(pick(slot_evidence, "job", lang));
            s.push_str("\n\n## Like What (Reference)\n\n");
            s.push_str(pick(slot_evidence, "anchor", lang));
            s.push_str("\n\n## What the Program Remembers\n\n");
            s.push_str(pick(slot_evidence, "memory", lang));
            s.push_str("\n\n## First Run\n\n");
            s.push_str(pick(slot_evidence, "first_run", lang));
            s.push_str("\n\n## What It Must Not Break On\n\n");
            s.push_str(pick(slot_evidence, "robustness", lang));
            s.push_str("\n\n## Deliberately NOT Doing\n\n");
            s.push_str(pick(slot_evidence, "scope", lang));
            s.push_str("\n\n## Success Looks Like\n\n");
            s.push_str(pick(slot_evidence, "acceptance", lang));
            s.push_str("\n\n## User Additions\n\n");
            s.push_str(pick(slot_evidence, "mirror", lang));
            s.push_str("\n\n## One-line Brief to AI Coder\n\n");
            s.push_str("Implement a minimal version using the Goal / Memory / First Run above.");
        }
    }
    s.push_str("\n\n<!-- TURINGOS_SPEC_END -->\n");
    s
}

/// TRACE_MATRIX FC2-N16: spec.md envelope wrapper (header + body + Q/A appendix).
///
/// Verbatim copy of `cmd_spec::wrap_spec_md` (pre-A6 src/bin/turingos/cmd_spec.rs
/// line ~1716). The SpecCapsule hashes this WHOLE blob, so future replay can
/// derive both the formatted spec and the raw transcript from the single CID.
pub fn wrap_spec_md(
    body: &str,
    questions: &[String],
    answers: &[String],
    model_id: &str,
    skipped_llm: bool,
) -> String {
    let mut s = String::new();
    s.push_str("# TuringOS Spec (Phase 6.3)\n\n");
    s.push_str(&format!(
        "> Generated by `turingos spec` — meta model: `{model_id}`"
    ));
    if skipped_llm {
        s.push_str(" (skip-llm: no synthesis call made)");
    }
    s.push_str("\n\n");
    s.push_str(body.trim_end());
    s.push_str("\n\n---\n\n");
    s.push_str("## Appendix — Raw Q/A (for audit)\n\n");
    for (i, (q, a)) in questions.iter().zip(answers.iter()).enumerate() {
        s.push_str(&format!("**Q{}**: {q}\n\n", i + 1));
        s.push_str(&format!("**A{}**: {a}\n\n", i + 1));
    }
    s
}

/// TRACE_MATRIX FC2-N16: convenience — pad short answer vec to exactly 8 slots.
///
/// Used by the web in-process path where the driven session may have collected
/// fewer than 8 user answers before `done=true` (e.g. Meta declared coverage
/// complete at turn 4). The CLI driven path does the equivalent inline at
/// `cmd_spec.rs:1322-1326`.
pub fn pad_answers_to_8(mut answers: Vec<String>) -> Vec<String> {
    while answers.len() < 8 {
        answers.push("(not collected in driven session)".to_string());
    }
    answers.truncate(8);
    answers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_questions_zh_has_8_entries() {
        let qs = canonical_questions(Lang::Zh);
        assert_eq!(qs.len(), 8);
        for q in &qs {
            assert!(!q.is_empty(), "all Zh questions must be non-empty");
        }
    }

    #[test]
    fn canonical_questions_en_has_8_entries() {
        let qs = canonical_questions(Lang::En);
        assert_eq!(qs.len(), 8);
        for q in &qs {
            assert!(!q.is_empty(), "all En questions must be non-empty");
        }
    }

    #[test]
    fn synthesise_no_llm_ends_with_spec_end_marker_zh() {
        let qs = canonical_questions(Lang::Zh);
        let answers = pad_answers_to_8(vec!["A1".into(), "A2".into()]);
        let md = synthesise_spec_md_no_llm(Lang::Zh, &qs, &answers);
        assert!(md.contains("一句话目标"));
        assert!(md.trim_end().ends_with("<!-- TURINGOS_SPEC_END -->"));
    }

    #[test]
    fn synthesise_no_llm_ends_with_spec_end_marker_en() {
        let qs = canonical_questions(Lang::En);
        let answers = pad_answers_to_8(vec!["A1".into()]);
        let md = synthesise_spec_md_no_llm(Lang::En, &qs, &answers);
        assert!(md.contains("One-line Goal"));
        assert!(md.trim_end().ends_with("<!-- TURINGOS_SPEC_END -->"));
    }

    #[test]
    fn wrap_spec_md_renders_header_and_appendix() {
        let qs = canonical_questions(Lang::En);
        let answers = pad_answers_to_8(vec!["foo".into(), "bar".into()]);
        let body = synthesise_spec_md_no_llm(Lang::En, &qs, &answers);
        let wrapped = wrap_spec_md(&body, &qs, &answers, "test-model", true);
        assert!(wrapped.starts_with("# TuringOS Spec (Phase 6.3)"));
        assert!(wrapped.contains("test-model"));
        assert!(wrapped.contains("Appendix — Raw Q/A"));
        assert!(wrapped.contains("**Q1**:"));
        assert!(wrapped.contains("**A2**: bar"));
    }

    #[test]
    fn pad_answers_to_8_pads_short_vec() {
        let padded = pad_answers_to_8(vec!["a".into(), "b".into()]);
        assert_eq!(padded.len(), 8);
        assert_eq!(padded[0], "a");
        assert_eq!(padded[1], "b");
        assert!(padded[2].contains("not collected"));
    }

    #[test]
    fn pad_answers_to_8_truncates_long_vec() {
        let padded = pad_answers_to_8((0..12).map(|i| format!("a{i}")).collect());
        assert_eq!(padded.len(), 8);
        assert_eq!(padded[7], "a7");
    }

    // ----- F10 (2026-05-19): slot-keyed synthesis regression tests --------

    /// D-NEW-3a regression (Π4.3 P7 / Π4.4 S11): when the LLM asks slots in a
    /// non-canonical order, slot-keyed synthesis must put each user answer
    /// under the slot it actually covered — not the canonical position N where
    /// N == index in the answer Vec.
    #[test]
    fn synthesise_by_slot_renders_correct_slot_for_each_user_answer() {
        let mut slots = BTreeMap::new();
        // Simulate Π4.3 P7-style: user gave anchor-content twice (T2 + T3),
        // then memory at T4, first_run at T5, robustness at T6, scope at T7.
        // The LLM eventually emits covered_slots reflecting the actual
        // information extracted; only the LATEST answer per slot lands here.
        slots.insert("job".into(), "做影片转档工具".into());
        slots.insert("anchor".into(), "SHA256+原文件名".into());
        slots.insert("memory".into(), "Redis 存任务状态".into());
        slots.insert("first_run".into(), "拖拽到网页".into());
        slots.insert("robustness".into(), "上传失败重试三次".into());
        slots.insert("scope".into(), "个人用 QPS 10".into());
        slots.insert("acceptance".into(), "每月可见用量".into());

        let md = synthesise_spec_md_no_llm_by_slot(Lang::Zh, &slots);

        // Each slot's content must land under its own section header.
        let memory_section = md
            .split("## 程序要记住的东西 (Memory)")
            .nth(1)
            .unwrap()
            .split("##")
            .next()
            .unwrap();
        assert!(
            memory_section.contains("Redis 存任务状态"),
            "Memory section must carry memory-slot content, not anchor/first_run; got: {memory_section}"
        );
        assert!(
            !memory_section.contains("SHA256"),
            "Memory section must NOT contain anchor-slot content (positional bug regression)"
        );

        let first_run_section = md
            .split("## 第一次使用 (First Run)")
            .nth(1)
            .unwrap()
            .split("##")
            .next()
            .unwrap();
        assert!(
            first_run_section.contains("拖拽到网页"),
            "First Run section must carry first_run-slot content"
        );
        assert!(
            !first_run_section.contains("Redis"),
            "First Run section must NOT contain memory-slot content (positional bug regression)"
        );

        let robustness_section = md
            .split("## 不能搞坏的情况 (Robustness)")
            .nth(1)
            .unwrap()
            .split("##")
            .next()
            .unwrap();
        assert!(
            robustness_section.contains("上传失败重试三次"),
            "Robustness section must carry robustness-slot content"
        );

        let scope_section = md
            .split("## 故意不做的 (Out of Scope)")
            .nth(1)
            .unwrap()
            .split("##")
            .next()
            .unwrap();
        assert!(
            scope_section.contains("个人用 QPS 10"),
            "Scope section must carry scope-slot content"
        );

        assert!(md.trim_end().ends_with("<!-- TURINGOS_SPEC_END -->"));
    }

    /// D-NEW-3a regression: when the LLM declared `done=true` before covering
    /// every slot (rare but possible — coverage predicate may be relaxed in
    /// future), missing slots must render a typed placeholder rather than
    /// panicking or rendering an empty body.
    #[test]
    fn synthesise_by_slot_handles_missing_slot_with_placeholder() {
        let mut slots = BTreeMap::new();
        slots.insert("job".into(), "做小工具".into());
        slots.insert("anchor".into(), "像 Excel".into());
        // Skip memory, first_run, robustness.
        slots.insert("scope".into(), "个人用".into());
        slots.insert("acceptance".into(), "每周省 2 小时".into());

        let md = synthesise_spec_md_no_llm_by_slot(Lang::Zh, &slots);
        assert!(
            md.contains("（用户未在本轮访谈中提供该信息）"),
            "missing slots must render Zh placeholder; got:\n{md}"
        );
        // The placeholder must appear in Memory / First Run / Robustness.
        let memory_section = md
            .split("## 程序要记住的东西 (Memory)")
            .nth(1)
            .unwrap()
            .split("##")
            .next()
            .unwrap();
        assert!(memory_section.contains("（用户未在本轮访谈中提供该信息）"));
        let first_run_section = md
            .split("## 第一次使用 (First Run)")
            .nth(1)
            .unwrap()
            .split("##")
            .next()
            .unwrap();
        assert!(first_run_section.contains("（用户未在本轮访谈中提供该信息）"));

        // English variant must use English placeholder.
        let mut en_slots = BTreeMap::new();
        en_slots.insert("job".into(), "build a tool".into());
        let md_en = synthesise_spec_md_no_llm_by_slot(Lang::En, &en_slots);
        assert!(
            md_en.contains("(user did not provide this information in the interview)"),
            "missing slots must render En placeholder; got:\n{md_en}"
        );
    }

    /// D-NEW-3a regression: user-supplied script (Traditional Chinese,
    /// Cantonese, etc.) must survive byte-for-byte into the rendered spec.md.
    /// Π4.3 P7 (Traditional zh-TW) and Π4.4 S11 (Cantonese) both depended on
    /// this property; without it the spec.md would silently be re-romanised
    /// or simplified by the synthesis layer.
    #[test]
    fn synthesise_by_slot_preserves_input_script() {
        let mut slots = BTreeMap::new();
        slots.insert(
            "job".into(),
            "想做一個影片轉檔工具, 支援拖曳上傳, 輸出 mp4".into(),
        );
        slots.insert(
            "anchor".into(),
            "錨點就是每個檔案的 SHA256 + 原檔名".into(),
        );
        slots.insert("memory".into(), "記憶用 Redis 存任務狀態".into());
        slots.insert(
            "first_run".into(),
            "直接拖檔到網頁, 不需註冊登入".into(),
        );
        slots.insert("robustness".into(), "穩健性: 上傳失敗自動重試三次".into());
        slots.insert("scope".into(), "範圍: 個人創作者用".into());
        slots.insert("acceptance".into(), "下載完就走".into());

        let md = synthesise_spec_md_no_llm_by_slot(Lang::Zh, &slots);

        // Traditional-only characters must survive verbatim.
        assert!(md.contains("影片轉檔"), "Traditional 轉 must survive");
        assert!(md.contains("錨點"), "Traditional 錨 must survive");
        assert!(md.contains("檔案"), "Traditional 檔 must survive");
        assert!(md.contains("穩健性"), "Traditional 穩 must survive");
        assert!(md.contains("範圍"), "Traditional 範 must survive");
    }
}
