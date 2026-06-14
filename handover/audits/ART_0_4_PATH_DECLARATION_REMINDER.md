# Art. 0.4 Path Declaration Gate (Art. 0.4 Constitution Slot)

**Gate I: Path Declaration Reminder**

---

## Context

Article 0.4 ("Q_t 是 version-controlled 状态", constitution.md lines 114–152) identifies a foundational architectural gap: the constitution defines `Q_t = ⟨q_t, HEAD_t, tape_t⟩` as a git-style version control triple, but the current runtime implementation has **zero git substrate** (runtime grep confirms `0 hits` for `Repository::|git2::|libgit2|Command git`).

## The Requirement

From constitution.md Art. 0.4, lines 149–150 (emphasis added):

> **本宪法颁布后下一次架构 commit（Commit 1: Tape Schema Upgrade，见 Art. 0.2 修复义务）必须明文标注采用 A/B/C 中哪条路径。**

Translation: "After this constitution is promulgated, the **next architectural commit** (Commit 1: Tape Schema Upgrade, see Art. 0.2 remediation duty) **must explicitly declare which of paths A/B/C it adopts**."

## The Three Implementation Paths (from constitution.md lines 136–142)

Quoted directly from constitution.md Art. 0.4:

| 路径 | 工作量 | 宪法对齐度 | 说明 |
|---|---|---|---|
| **A. 语义版** | ~3 周 | partial | 保持 `Vec<Node>`；加 `hash: [u8;32]` (Art. 0.3) + `HEAD_t: NodeId` last-accepted pointer + `rtool/wtool` 显式三元组签名。满足 version-control **形式语义** 但不用 git 库 |
| **B. 真 git 版** | ~6-8 周 | full | libgit2/git2-rs 集成；每 cell run 用 runtime 临时 git repo；Node = commit object；`bus.append` = `git commit`；`HEAD_t` = git HEAD ref；`Π_p` = pre-commit hook；Boltzmann routing = git branch；自动获得 git 30 年成熟的 hash chain + immutable objects + branch + reflog + content-addressable storage + Merkle DAG |
| **C. 延期版** (hybrid) | ~3 周 现 + 5 周 后 | full @ Phase E | Phase C/D 用 A 快速 unblock；Art. 0.4 此条款 declares B 为 Phase E gate 必经；Phase E 切换 substrate |

## Phase E Mandate (from constitution.md line 149)

> **Phase E gate 强制 B**（除非 Phase E 之前用户 sudo 修宪降低 fidelity 要求）。

Translation: "Phase E gate **forces path B** (unless the user sudo-amends the constitution to lower fidelity before Phase E)."

## The Current Status

- H-HET-1 (het probing bugfix, 2026-06-14) is **Class 2** (does not touch tape/schema/bus/wtool/rtool)
- H-HET-1 is therefore **OUT OF SCOPE** for Art. 0.4 path declaration
- **The constitutional gap on 0.4 remains OPEN**
- No future commit may claim to "close Art. 0.4" unless and until one of paths A/B/C is explicitly chosen and implemented

## Compliance Rule (Gate I Enforcement)

Any future commit that touches **any of these surfaces** must first declare which path it takes:

- `src/ledger.rs::Tape` schema
- `src/bus.rs::append_internal` or tape append logic  
- `rtool` (read tool interface) signature or tape read protocol
- `wtool` (write tool interface) signature or tape write protocol
- Tape-to-Q_t versioning, HEAD_t, or path pointer mechanics

**Recommended placement:** Commit message footer or newly created section in the commit's PR body.

**Example (Path A declaration):**

```
This commit adopts Art. 0.4 implementation Path A (semantic version).

Rationale: Phase C/D unblock; deferred to Path B at Phase E.

- Adds hash: [u8;32] field to Node (Art. 0.3 semantic slot)
- Adds HEAD_t: NodeId last-accepted pointer
- Explicit rtool/wtool(⟨q_t, tape_t, HEAD_t⟩) signatures per Art. IV lines 556/584
```

**Example (Path B declaration):**

```
This commit adopts Art. 0.4 implementation Path B (true git substrate).

- Integrates libgit2/git2-rs
- Each cell run uses runtime-local temporary git repo
- bus.append ≡ git commit; HEAD_t ≡ git HEAD ref
- Pre-commit hook ≡ Π_p predicates
```

## Known Pre-Existing Violations (H-HET-1 does not fix these)

The H-HET-1 carrier (het probing fix) is orthogonal to Art. 0.4 and does not claim to resolve the tape schema or HEAD_t implementation gap. Any future attempt to claim "Art. 0.4 resolved" while the gap persists is a false warranty.

---

**Gate I Status:** ✓ Reminder issued. System remains in Art. 0.4 `pending` state until a path-declaring architectural commit lands.
