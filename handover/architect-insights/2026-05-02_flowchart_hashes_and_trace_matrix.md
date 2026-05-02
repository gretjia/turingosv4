# Flowchart Hashes + TRACE_FLOWCHART_MATRIX (architect insight 2026-05-02)

**Source**: lossless constitution integrated edition, archived at `handover/directives/2026-05-02_lossless_constitution_polymarket_directive.md` §1.1.
**Status**: INSIGHT — pending authorization to create `handover/alignment/TRACE_FLOWCHART_MATRIX.md`.

---

## §1 The four canonical flowchart hashes

The constitution's three flowcharts (4 image fragments due to the runtime loop spanning two PDF pages) now have permanent SHA256 identifiers. These are the **canonical visual ground truth**: any divergence in derivative diagrams requires re-hashing against these.

```text
Flowchart 1a — Runtime loop, page 8
  rtool / input / Agent δ / output / predicates ∏p / write tool path
  SHA256: a474c6b9ded766504a4f644a4a1b3c545316d418f0250f36ec692fcdf98f09f5

Flowchart 1b — Runtime loop continuation, page 9
  predicates branch / write tool / Q_{t+1} / map-reduce tick
  SHA256: b822717b10332a2d8e789ba6af96fd4da4ff43a74afab679d1b82add9c32b64d

Flowchart 2 — Boot + full architecture, page 13
  Initialization (human → InitAI → predicates / Q0 / mr) + runtime loop + Finalization
  SHA256: 6a4bc9195bafd55bde968fd445cdd2926d6906a7f6a2b38071d4774a7f0de333

Flowchart 3 — Meta-architecture, page 17
  Constitution + logs archive (read-only) → JudgeAI / ArchitectAI →
  anti-oreo runtime (top / agents / tools) → log → archive → feedback → re-init
  SHA256: c159413984d0c6c5daa06605fea3a86a2ad4ab9c4284d0d20e0e525bf03aa9cd
```

---

## §2 What changed about how flowcharts are used

Pre-2026-05-02: flowcharts were explanatory. TBs cited Art. I/II/III/V text directly.

Post-2026-05-02: flowcharts are **verification axes**. Each TB charter must declare which flowchart element(s) it touches, in addition to existing `FC-trace` to article numbers.

```text
Flowchart 1 (runtime loop):
  - Touched by any TB that wires read/write tools, predicates, or version control.
  - Validation tests:
      every externalized proposal enters L4 or L4.E
      dashboard is materialized view (not source of truth)
      predicate failure does not advance Q_t

Flowchart 2 (boot):
  - Touched by any TB that initializes ground truth, on_init mint,
    or initial state Q_0.
  - Validation tests:
      genesis_report exists and is replayable
      TaskOpen/EscrowLock observable from L4 chain

Flowchart 3 (meta-architecture):
  - Touched by any TB that consumes logs archive, runs ArchitectAI/JudgeAI,
    or amends predicates/tools.
  - Validation tests:
      EvidenceCapsule produced at session end
      Markov default context (latest capsule + constitution only)
      no untriaged log fragments leaked into Agent prompt
```

---

## §3 Mandatory new alignment artifact

The directive mandates creation of `handover/alignment/TRACE_FLOWCHART_MATRIX.md`, which serves as the cross-reference between TBs and flowchart elements.

**Skeleton**:

```text
| TB ID  | Flowchart 1 | Flowchart 2 | Flowchart 3 | Notes |
|--------|-------------|-------------|-------------|-------|
| TB-7R  | ✅ runtime  | —           | —           | Frame B closure: L4/L4.E |
| TB-8   | ✅ settle   | ✅ genesis  | —           | Settlement node + boot continuity |
| TB-9   | ✅ identity | ✅ boot     | —           | Durable AgentRegistry |
| TB-10  | ✅ market   | ✅ boot     | —           | Lean Proof Task Market MVP |
| TB-11  | ✅ price    | —           | —           | NodePosition + PriceIndex |
| TB-12  | ✅ CTF      | —           | —           | CompleteSet + MarketSeedTx |
| TB-13  | ✅ AMM      | —           | —           | CPMM Router (no ghost liquidity) |
| TB-14  | ✅ mask     | —           | —           | Boltzmann masking (read-view only) |
| TB-15  | —           | —           | ✅ logs     | Markov Log Loom + EvidenceCapsule |
| TB-16  | ✅ runtime  | ✅ boot     | ✅ logs     | Beta — all three loops live |
| TB-17  | ✅ trade    | —           | —           | Full market trading |
```

(This skeleton is illustrative; final populated matrix awaits authorization.)

---

## §4 Why this matters

The lossless edition's design philosophy is that **flowcharts are not just diagrams — they are SHA256-anchored architectural contracts**. Any drift between code behavior and these four images is now provably auditable.

This complements the existing TRACE_MATRIX (which maps src/ symbols to constitution articles). TRACE_FLOWCHART_MATRIX maps TBs to runtime/boot/meta loops. Together they pincer the implementation: bottom-up (per-symbol) and top-down (per-TB).

Per Art. V.1.3, only the canonical flowcharts in `constitution.md` (or the lossless integrated edition referenced by it) are authoritative. Re-rendering or paraphrasing flowcharts in TBs is allowed but must hash-match.
