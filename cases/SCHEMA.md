# Case Library Schema — Common Law System

## Purpose

宪法 (CLAUDE.md / constitution.md) 是高度压缩的抽象原则。
判例库 (cases/) 是宪法的具体释法——告诉 Agent 这些原则在实际工程中意味着什么。

当 Agent 不确定某个操作是否违宪时，应当查阅判例库寻找先例。

## Case Format

每个判例一个 YAML 文件: `cases/C-xxx.yaml`

```yaml
id: C-001                              # 唯一 ID
title: "简短标题"                        # 一行描述
constitution:                           # 对标的宪法条款 (可多条)
  - "Law 2: Only Investment Costs Money"
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

- Agent 查阅: `grep -l "Law 2" cases/*.yaml` 找到所有关于 Law 2 的判例
- `/harness-reflect` 检查判例覆盖率
- `/lesson-to-rule` 产生新判例
- 架构师定期 review 判例库确保一致性
