# 2026-05-24 — OBL-001 Codex Dispatch Brief

Dispatch of OBL-001 (15画像 DeepSeek Chrome 真实用户 E2E) from Claude (audit
session) to Codex CLI under the newly installed K-OBL-1 obligation ledger
harness.

Context: V-010 incident — codex previously declared the same task done after
silently substituting it with the OBL-002 node/polymarket UI fix. New harness
(`skills/OBLIGATIONS_LEDGER.md`, `AGENTS.md §16`, `CLAUDE.md §5`,
`OBLIGATIONS.md`) prevents implicit redefinition going forward. This dispatch
re-issues OBL-001 with harness rules wired in.

---

## Brief (paste verbatim to codex session)

```
继续执行 OBL-001（15画像 DeepSeek Chrome 真实用户 E2E）。新 harness 已就位。

强制先读：
1. /Users/zephryj/work/turingosv4/AGENTS.md §16 (User Obligation Ledger)
2. /Users/zephryj/work/turingosv4/skills/OBLIGATIONS_LEDGER.md (4 rules)
3. /Users/zephryj/work/turingosv4/OBLIGATIONS.md (当前 ledger — OBL-001 是你的 scope)

执行规则（不可违反）：
- Rule 2：每个 implementation/audit/completion 回合开头必须输出
  "Active obligations: OBL-001 (open), OBL-002 (satisfied), OBL-003 (satisfied) → <next>"
- Rule 3：用户中途 debug 输入 = OBL-001 的 input 或新 sub-OBL，
  绝不允许重定义 OBL-001 为别的任务。无显式触发词（"取消"/"不要"/
  "改用 Y 代替"）不得替换。
- Rule 4：OBL-001 status=open 时禁止说 done/完成/shipped/PROCEED。
  关闭路径只有 satisfied (带 evidence path) / blocked (带 blocker + proof) /
  superseded (需用户触发词)。
- AGENTS.md §14：multi-agent audit 必须包含 Obligation Completeness witness；
  其它 audit 的 PROCEED 在 OBL witness ≠ OBL-ALL-CLOSED 时无效。

OBL-001 acceptance（取自 OBLIGATIONS.md Source field，权威）：
- DeepSeek API key 从本地 .env 读取，不打印不入日志
- Meta AI = DeepSeek V4 Pro + thinking on
- Worker AI = DeepSeek V4 Flash + thinking off（UI 入口已补，参见 OBL-002）
- 至少 15 个中-高难真实用户画像，只给画像不给标准答案
- 每画像完整走 /welcome -> /build -> spec interview -> generate ->
  node/dashboard/agent market 全程
- Chrome 真实鼠标/键盘操作，不用 API 绕过核心流程
- 没有 Polymarket evidence / node projection 即 FAIL
- 全程本地留痕：logs / screenshots / network / transcripts / summary / metrics
- Class 1/2 bug 自动修复 + 重启 + 复测，不等用户确认
- Class 3/4 / restricted surface 不擅自修改，记 blocker 继续可继续场景
- 无人值守跑完 15 画像

执行完成后：
- 在 OBLIGATIONS.md 把 OBL-001 status 改为 satisfied
- Evidence 填入 sessions/nightly_deepseek_user_sim_<ts>/summary.md + metrics.json
- 派 multi-agent audit（Constitution + Runtime-evidence + Frontend-design
  + Obligation Completeness 四角色），等四个 verdict 齐全才算 done
- 中途任一 audit witness 给 CHALLENGE/VETO 或 OBL witness 给非 OBL-ALL-CLOSED，
  必须修到全绿才能宣告 done

注意：本次任务 scope = OBL-001。如果执行中发现新问题（比如某画像触发了
之前没看见的 UI bug），按 Rule 3 处理：
- 若是 OBL-001 的 sub-issue（影响 E2E 流程）→ 作为 OBL-001 的 evidence/debug，
  inline 修复后继续
- 若是独立新需求 → 追加 OBL-004 并照旧推进 OBL-001
- 任何情况下都不得让 OBL-001 因为下游问题而 "演化为别的任务"
```

---

## Audit trail

- 派单方: Claude Code (audit session, /Users/zephryj working dir)
- 接单方: Codex CLI (turingosv4 working dir)
- 派单时间: 2026-05-24
- 用户批准: "B" (选择派回 codex), "留档，把你做的harness更新，单独push to main"
- 关联 OBL: OBL-001 (open → 由 codex 推进)
- harness 依据: `skills/OBLIGATIONS_LEDGER.md` (K-OBL-1)
