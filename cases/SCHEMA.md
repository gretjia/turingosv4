# Case Library Schema — Common Law System

## Purpose

宪法 (CLAUDE.md / constitution.md) 是高度压缩的抽象原则。
判例库 (cases/) 是宪法的具体释法——告诉 Agent 这些原则在实际工程中意味着什么。

当 Agent 不确定某个操作是否违宪时，应当查阅判例库寻找先例。

## 宪法条款 ID 体系

判例通过 `constitution:` 字段引用宪法条款。合法的条款 ID:

```
Laws (基本法)
  Law 1: Information is Free
  Law 2: Only Investment Costs Money

Art. I    — 信号的量化
  Art. I.1    布尔信号
    Art. I.1.1  PCP 谓词与疑罪从无
  Art. I.2    统计信号

Art. II   — 信号的选择性广播
  Art. II.1   广播典型错误
  Art. II.2   广播价格信号
    Art. II.2.1 探索与利用平衡

Art. III  — 信号的选择性屏蔽
  Art. III.1  屏蔽错误
  Art. III.2  封装细节
  Art. III.3  屏蔽相关性
  Art. III.4  屏蔽 Goodhart 问题

Art. IV   — Boot
Art. V    — Go Meta
  Art. V.1    三权分立
    Art. V.1.1  宪法 (Constitution)
    Art. V.1.2  ArchitectAI
    Art. V.1.3  JudgeAI
  Art. V.2    宪法界限与示例
```

## Case Format

每个判例一个 YAML 文件: `cases/C-xxx.yaml`

```yaml
id: C-001                              # 唯一 ID
title: "简短标题"                        # 一行描述
constitution:                           # 对标的宪法条款 (可多条，使用正式 Art. ID)
  - "Law 2: Only Investment Costs Money"
  - "Art. I.1: 布尔谓词必须是 Ground Truth"
source_lessons: [V3L-41]               # 来源教训 ID (参见 V3_LESSONS.md)
incident: V-001                         # 来源事件 ID (可选)
facts: |                                # 事实: 发生了什么
  简要描述违宪/争议的具体行为
ruling: |                               # 裁决: 为什么违宪/合宪
  解释宪法原则如何应用于此事实
precedent: |                            # 先例: 后续案件应如何判断
  提炼出可复用的判断标准
rule: R-001                             # 产生的自动规则 (可选)
date: "2026-03-15"                      # 日期
severity: critical | high | medium      # 严重程度
```

## Usage

- 按条款查: `grep -l "Art. I.1" cases/*.yaml`
- 按 Law 查: `grep -l "Law 2" cases/*.yaml`
- 按教训查: `grep -l "V3L-41" cases/*.yaml`
- `/harness-reflect` 检查判例覆盖率
- `/lesson-to-rule` 产生新判例
- 完整教训列表: `cases/V3_LESSONS.md` (50 条教训 → 35 个判例的可审计映射)
- 架构师定期 review 判例库确保一致性
