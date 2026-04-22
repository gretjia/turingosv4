# Constitutional Flowchart Elements — raw extract 2026-04-22

Source: `/home/zephryj/projects/turingosv4/constitution.md`

## Anomaly note on mermaid fencing

A `grep -n '```' constitution.md` yields:

- Line 325: ` ```mermaid ` (opener, FC-1)
- Line 379: ` ``` ` (closer, FC-1)
- Line 530: ` ``` ` (closer, FC-2 — the **opener is missing** from the file; the block starts with a bare `flowchart TD` at line 441, indented 4 spaces)
- Line 714: ` ``` ` (closer, FC-3 — the **opener is missing**; the block starts with bare `graph TB` at line 670, indented 4 spaces)

So strictly only one properly-fenced `mermaid` block exists. FC-2 and FC-3 are indented pseudo-blocks with only closing backticks. Mermaid won't render them on GitHub without a proper opener. This is a constitutional-document hygiene issue that should be flagged to the human architect (Art. V.2 — the constitution is the ground truth and must be machine-parseable).

Below the three flowcharts are extracted as if the fencing were correct.

---

## FC-1 (basic cycle)

Location: lines 325–379. Header: `graph TD`.

### Nodes

| ID | Type | Label | Subgraph |
|---|---|---|---|
| FC1-N1 | circle `(("..."))` | `$$q$$` | Q0 |
| FC1-N2 | triangle `shape: tri` | `path` | Q0 |
| FC1-N3 | lin-cyl `shape: lin-cyl` | `everything as files` | Q0 |
| FC1-N4 | circle `(("..."))` | `$$q'$$` | Q1 |
| FC1-N5 | triangle `shape: tri` | `path'` | Q1 |
| FC1-N6 | lin-cyl `shape: lin-cyl` | `everything as files'` | Q1 |
| FC1-N7 | rect `["..."]` classDef black | `$$\delta$$` | AI |
| FC1-N8 | circle `(("..."))` | `$$q$$` | input |
| FC1-N9 | circle `(("..."))` | `$$s$$` | input |
| FC1-N10 | circle `(("..."))` | `$$q'$$` | output |
| FC1-N11 | circle `(("..."))` | `$$a$$` | output |
| FC1-N12 | rhombus `{"..."}` classDef white | `$$\prod$$ predicates` | top |
| FC1-N13 | rect `["..."]` classDef white | `read tool` | rtool |
| FC1-N14 | rect `["..."]` classDef white | `write tool` | wtool |

### Edges

| ID | From | To | Style | Label |
|---|---|---|---|---|
| FC1-E1 | tape0 | si | dotted `-.->` (multi-source: `tape0 & HEAD0 -.-> si`) | — |
| FC1-E2 | HEAD0 | si | dotted `-.->` (multi-source: `tape0 & HEAD0 -.-> si`) | — |
| FC1-E3 | q0 | qi | dotted `-.->` | — |
| FC1-E4 | qi | delta | dotted `-.->` (multi-source: `qi & si -.-> delta`) | — |
| FC1-E5 | si | delta | dotted `-.->` (multi-source: `qi & si -.-> delta`) | — |
| FC1-E6 | delta | qo | dotted `-.->` (multi-target: `delta -.-> qo & ao`) | — |
| FC1-E7 | delta | ao | dotted `-.->` (multi-target: `delta -.-> qo & ao`) | — |
| FC1-E8 | qo | q1 | dotted `-.->` | — |
| FC1-E9 | ao | tape1 | dotted `-.->` (multi-target: `ao -.-> tape1 & HEAD1`) | — |
| FC1-E10 | ao | HEAD1 | dotted `-.->` (multi-target: `ao -.-> tape1 & HEAD1`) | — |
| FC1-E11 | Q0 | rtool | thick `==>` | — |
| FC1-E12 | rtool | input | thick `==>` | — |
| FC1-E13 | input | AI | thick `==>` | — |
| FC1-E14 | AI | output | thick `==>` | — |
| FC1-E15 | output | p | thick `==>` | — |
| FC1-E16 | p | wtool | thick `==>` | `1` |
| FC1-E17 | wtool | Q1 | thick `==>` | — |
| FC1-E18 | p | Q0 | thick `==>` | `0` |

### Subgraphs

| ID | Label |
|---|---|
| FC1-S1 | Q0 → `"version control $$~Q_{t}$$"` |
| FC1-S2 | Q1 → `"version control $$~Q_{t+1}$$"` |
| FC1-S3 | AI → `"middle black"` |
| FC1-S4 | input → `input` (bare id, no quoted label) |
| FC1-S5 | output → `output` (bare id, no quoted label) |
| FC1-S6 | top → `"top management"` |
| FC1-S7 | rtool → `"bottom tools"` |
| FC1-S8 | wtool → `"bottom tools"` |

### Formulas embedded in FC-1 labels

| Where | Formula |
|---|---|
| FC1-S1 subgraph title | `$$~Q_{t}$$` |
| FC1-S2 subgraph title | `$$~Q_{t+1}$$` |
| FC1-N1 (q0) | `$$q$$` |
| FC1-N4 (q1) | `$$q'$$` |
| FC1-N7 (delta) | `$$\delta$$` |
| FC1-N8 (qi) | `$$q$$` |
| FC1-N9 (si) | `$$s$$` |
| FC1-N10 (qo) | `$$q'$$` |
| FC1-N11 (ao) | `$$a$$` |
| FC1-N12 (p) | `$$\prod$$ predicates` |

### FC-1 classDefs

| classDef | Fill | Stroke | Text |
|---|---|---|---|
| white | `#fff` | `#333` width 2px | `#900` |
| black | `#111` | `#333` width 2px | `#900` |

**FC-1 counts: 14 nodes, 18 edges, 8 subgraphs.**

---

## FC-2 (expanded with init / halt / tick)

Location: lines 441–530. Header: `flowchart TD` (indented 4 spaces, opening ` ```mermaid ` fence is MISSING in source).

### Nodes

| ID | Type | Label | Subgraph |
|---|---|---|---|
| FC2-N1 | slanted-rect `shape: sl-rect` classDef human | `human architect provides spec` | Initialization |
| FC2-N2 | docs `shape: docs` classDef white | `(tentative) ground truth` | Initialization |
| FC2-N3 | rect `[...]` classDef black | `Init AI` | Initialization |
| FC2-N4 | double-circle `shape: dbl-circ` | `HALT` | Finalization |
| FC2-N5 | circle `(("..."))` | `$$q_t$$` | Q0 |
| FC2-N6 | triangle `shape: tri` | `$$HEAD_t$$<br>as path` | Q0 |
| FC2-N7 | lin-cyl `shape: lin-cyl` | `$$tape_t$$<br>as files` | Q0 |
| FC2-N8 | circle `(("..."))` | `$$q_{t+1}$$` | Q1 |
| FC2-N9 | triangle `shape: tri` | `$$HEAD_{t+1}$$<br>as path` | Q1 |
| FC2-N10 | lin-cyl `shape: lin-cyl` | `$$tape_{t+1}$$<br>as files` | Q1 |
| FC2-N11 | rect `["..."]` classDef white | `read tool` | rtool |
| FC2-N12 | circle `(("..."))` | `$$q_i$$` | input |
| FC2-N13 | circle `(("..."))` | `$$s_i$$` | input |
| FC2-N14 | rect `["..."]` classDef black | `AI as $$\delta$$` | AI |
| FC2-N15 | circle `(("..."))` | `$$q_o$$` | output |
| FC2-N16 | circle `(("..."))` | `$$a_o$$` | output |
| FC2-N17 | processes `shape: processes` classDef white | `predicates $$p$$` | top |
| FC2-N18 | rhombus `{"..."}` classDef white | `$$\prod \mathbf{p}$$` | top |
| FC2-N19 | rect `["..."]` classDef white | `map reduce` | toptick |
| FC2-N20 | circle `(("clock"))` classDef white | `clock` | toptick |
| FC2-N21 | rect `["..."]` classDef white | `write tool` | wtool |
| FC2-N22 | rect `["..."]` classDef white | `other tools` | wtool |

### Edges

| ID | From | To | Style | Label |
|---|---|---|---|---|
| FC2-E1 | human | law | cross-dashed `--x` | `once` |
| FC2-E2 | law | initAI | arrow `-->` | — |
| FC2-E3 | initAI | predicates | cross-dashed `--x` | `once` |
| FC2-E4 | predicates | p | line `---` (undirected) | — |
| FC2-E5 | initAI | mr | cross-dashed `--x` | `once` |
| FC2-E6 | initAI | Q0 | cross-dashed `--x` | `once` |
| FC2-E7 | tape0 | si | long arrow `---->` (multi-source: `tape0 & HEAD0 ----> si`) | — |
| FC2-E8 | HEAD0 | si | long arrow `---->` (multi-source: `tape0 & HEAD0 ----> si`) | — |
| FC2-E9 | q0 | qi | arrow `-->` | — |
| FC2-E10 | qi | delta | arrow `-->` (multi-source: `qi & si --> delta`) | — |
| FC2-E11 | si | delta | arrow `-->` (multi-source: `qi & si --> delta`) | — |
| FC2-E12 | delta | qo | arrow `-->` (multi-target: `delta --> qo & ao`) | — |
| FC2-E13 | delta | ao | arrow `-->` (multi-target: `delta --> qo & ao`) | — |
| FC2-E14 | qo | q1 | dotted `-.->` | — |
| FC2-E15 | ao | HEAD1 | dotted `-.->` | — |
| FC2-E16 | ao | tape1 | dotted `-.->` | — |
| FC2-E17 | Q0 | rtool | thick `==>` | — |
| FC2-E18 | rtool | input | thick `==>` | — |
| FC2-E19 | input | AI | thick `==>` | — |
| FC2-E20 | AI | output | thick `==>` | — |
| FC2-E21 | output | p | thick `==>` | — |
| FC2-E22 | p | wtool | thick `==>` | `"$$Q_{t+1} = \mathbf{wtool}(output)$$<br>if $$\prod \mathbf{p} = 1$$"` |
| FC2-E23 | wtool | Q1 | thick `==>` | — |
| FC2-E24 | p | Q0 | thick `==>` | `"$$Q_{t+1} = Q_t$$<br>if $$\prod \mathbf{p} = 0$$"` |
| FC2-E25 | q1 | halt | thick `==>` | `"if q = halt"` |
| FC2-E26 | clock | mr | arrow `-->` | — |
| FC2-E27 | mr | tape0 | thick `==>` | `map` |
| FC2-E28 | mr | tape1 | thick `==>` | `reduce` |

### Subgraphs

| ID | Label |
|---|---|
| FC2-S1 | Initialization → `Initialization` (bare id, no quoted label) |
| FC2-S2 | Finalization → `Finalization` (bare id, no quoted label) |
| FC2-S3 | Q0 → `"version control: $$Q_t = \langle q_t,\ HEAD_t,\ tape_t \rangle$$"` |
| FC2-S4 | Q1 → `"version control: $$Q_{t+1} = \langle q_{t+1},\ HEAD_{t+1},\ tape_{t+1}\rangle$$"` |
| FC2-S5 | rtool → `"bottom tools: $$\langle q_i,\ s_i \rangle = \mathbf{rtool}(\langle q_t,\ tape_t,\ HEAD_t \rangle)$$"` |
| FC2-S6 | input → `"$$input = \langle q_i,\ s_i \rangle$$"` |
| FC2-S7 | AI → `"middle black: $$output = \delta(input)$$"` |
| FC2-S8 | output → `"$$output = \langle q_o,\ a_o \rangle$$"` |
| FC2-S9 | top → `"top management: $$\prod \mathbf{p}(output \mid Q_t)$$"` |
| FC2-S10 | toptick → `"top management: ticks"` |
| FC2-S11 | wtool → `"bottom tools: $$\mathbf{wtool}(output \mid tape_t,HEAD_t,tools_{other})$$"` |

### Formulas embedded in FC-2 labels

| Where | Formula |
|---|---|
| FC2-S3 title | `$$Q_t = \langle q_t,\ HEAD_t,\ tape_t \rangle$$` |
| FC2-S4 title | `$$Q_{t+1} = \langle q_{t+1},\ HEAD_{t+1},\ tape_{t+1}\rangle$$` |
| FC2-S5 title | `$$\langle q_i,\ s_i \rangle = \mathbf{rtool}(\langle q_t,\ tape_t,\ HEAD_t \rangle)$$` |
| FC2-S6 title | `$$input = \langle q_i,\ s_i \rangle$$` |
| FC2-S7 title | `$$output = \delta(input)$$` |
| FC2-S8 title | `$$output = \langle q_o,\ a_o \rangle$$` |
| FC2-S9 title | `$$\prod \mathbf{p}(output \mid Q_t)$$` |
| FC2-S11 title | `$$\mathbf{wtool}(output \mid tape_t,HEAD_t,tools_{other})$$` |
| FC2-N5..N10 | `$$q_t$$`, `$$HEAD_t$$`, `$$tape_t$$`, `$$q_{t+1}$$`, `$$HEAD_{t+1}$$`, `$$tape_{t+1}$$` |
| FC2-N12..N13 | `$$q_i$$`, `$$s_i$$` |
| FC2-N14 | `AI as $$\delta$$` |
| FC2-N15..N16 | `$$q_o$$`, `$$a_o$$` |
| FC2-N17 | `predicates $$p$$` |
| FC2-N18 | `$$\prod \mathbf{p}$$` |
| FC2-E22 label | `$$Q_{t+1} = \mathbf{wtool}(output)$$<br>if $$\prod \mathbf{p} = 1$$` |
| FC2-E24 label | `$$Q_{t+1} = Q_t$$<br>if $$\prod \mathbf{p} = 0$$` |
| FC2-E25 label | `if q = halt` |

### FC-2 classDefs

| classDef | Fill | Stroke | Text |
|---|---|---|---|
| white | `#fff` | `#333` width 2px | `#900` |
| black | `#111` | `#333` width 2px | `#900` |
| human | `#fff4e6` | `#a85d00` width 2px | `#5c3200` |
| note  | `#fff8cc` | `#8a6d00` width 1px | `#4d3d00` |

**FC-2 counts: 22 nodes, 28 edges, 11 subgraphs.**

---

## FC-3 (anti-oreo system-level)

Location: lines 670–714. Header: `graph TB` (indented 4 spaces, opening ` ```mermaid ` fence is MISSING in source).

### Nodes

| ID | Type | Label | Subgraph |
|---|---|---|---|
| FC3-N1 | bare id | `boot` | (root, no subgraph) |
| FC3-N2 | bare id classDef human | `human` | (root, no subgraph) |
| FC3-N3 | doc `shape: doc` classDef white | `constitution as ground truth` | readonly (inside InitAI) |
| FC3-N4 | docs `shape: docs` classDef white | `logs archive as ground truth` | readonly (inside InitAI) |
| FC3-N5 | rect `[...]` classDef black | `JudgeAI` | InitAI |
| FC3-N6 | rect `[...]` classDef black | `ArchitectAI` | InitAI |
| FC3-N7 | bare id classDef white | `top` | anti_oreo |
| FC3-N8 | bare id classDef black | `agents` | anti_oreo |
| FC3-N9 | bare id classDef white | `tools` | anti_oreo |
| FC3-N10 | rect `["..."]` | `Q` (id = `tape`, label = `"Q"`) | system |
| FC3-N11 | doc `shape: doc` classDef white | `log` | system |
| FC3-N12 | rhombus `{"..."}` | `need to improve?` (id = `error`) | (root, no subgraph) |

### Edges

| ID | From | To | Style | Label |
|---|---|---|---|---|
| FC3-E1 | human | constitution | arrow `-->` | `maintain` |
| FC3-E2 | top | agents | thick `==>` | `manage` |
| FC3-E3 | agents | tools | thick `==>` | `use` |
| FC3-E4 | judgeAI | tools | dotted `-.->` (multi-source: `judgeAI & architectAI -.->\|use\| tools`) | `use` |
| FC3-E5 | architectAI | tools | dotted `-.->` (multi-source: `judgeAI & architectAI -.->\|use\| tools`) | `use` |
| FC3-E6 | log | logs | very-thick `====>` | `archive` |
| FC3-E7 | boot | init | thick `==>` | — |
| FC3-E8 | init | top | thick `==>` | `init/iterate` |
| FC3-E9 | init | tape | arrow `-->` | `init` |
| FC3-E10 | init | tools | thick `==>` | `make/improve` |
| FC3-E11 | tools | log | thick `==>` | `write` |
| FC3-E12 | logs | architectAI | arrow `-->` | `feedback` |
| FC3-E13 | init | error | thick `==>` | — |
| FC3-E14 | error | boot | very-long-thick `==========>` | `re-init` |
| FC3-E15 | constitution | judgeAI | arrow `-->` (multi-target: `constitution -->\|abide\| judgeAI & architectAI`) | `abide` |
| FC3-E16 | constitution | architectAI | arrow `-->` (multi-target: `constitution -->\|abide\| judgeAI & architectAI`) | `abide` |
| FC3-E17 | judgeAI | architectAI | arrow `-->` | `veto` |

### Subgraphs

| ID | Label |
|---|---|
| FC3-S1 | system → `system` (bare id, no quoted label) |
| FC3-S2 | init → `"InitAI"` |
| FC3-S3 | readonly → `readonly` (bare id, no quoted label) |
| FC3-S4 | anti_oreo → `"anti-oreo"` |

### Formulas embedded in FC-3 labels

None. FC-3 uses only English/natural-language labels (`maintain`, `manage`, `use`, `archive`, `init/iterate`, `init`, `make/improve`, `write`, `feedback`, `re-init`, `abide`, `veto`, `need to improve?`, `constitution as ground truth`, `logs archive as ground truth`, `Q`). No LaTeX math.

### FC-3 classDefs

| classDef | Fill | Stroke | Text |
|---|---|---|---|
| white | `#fff` | `#333` width 2px | `#900` |
| black | `#111` | `#333` width 2px | `#900` |
| human | `#fff4e6` | `#a85d00` width 2px | `#5c3200` |
| note  | `#fff8cc` | `#8a6d00` width 1px | `#4d3d00` |

**FC-3 counts: 12 nodes, 17 edges, 4 subgraphs.**

---

## Grand totals

| Chart | Nodes | Edges | Subgraphs |
|---|---|---|---|
| FC-1 | 14 | 18 | 8 |
| FC-2 | 22 | 28 | 11 |
| FC-3 | 12 | 17 | 4 |
| **Total** | **48** | **63** | **23** |

Grand total elements (nodes + edges + subgraphs): **134**.

## Anomalies found

1. **Fencing bug in constitution.md** — only FC-1 has a proper ` ```mermaid ` opener at line 325. FC-2 (line 441, `flowchart TD`) and FC-3 (line 670, `graph TB`) are indented-code pseudo-blocks with only a closing ` ``` ` (lines 530 / 714). Neither will render as mermaid on GitHub / Notion in current form. Treat as constitutional hygiene issue (Art. V.2 — ground truth must be machine-parseable).
2. **FC-3 node `tape` id vs label mismatch** — declared as `tape["Q"]`; id is `tape`, rendered label is `"Q"`. Easy to mis-wire to the FC-1/FC-2 `tape_t` lin-cyl nodes; they are distinct entities.
3. **FC-3 `top`, `agents`, `tools` are bare-id nodes** (no `[...]` or `{...}` shape wrappers), only styled via `:::white` / `:::black`. Mermaid renders them as default rectangles with the id as the visible label.
4. **FC-3 `boot`, `human`, `error` live outside every subgraph** — they are at the root of `graph TB`. Only `error` has an explicit shape (`{need to improve?}`); `boot` and `human` are bare ids.
5. **FC-2 edge label multi-line markup** — labels on FC2-E22, FC2-E24 use `<br>` for line-break inside `$$...$$`. Mermaid + KaTeX handling is renderer-dependent; these may display as a single line on some renderers.
6. **Class `note` is declared but never applied** — both FC-2 and FC-3 declare `classDef note` but no node uses `:::note`. Dead CSS.
7. **Label reuse across subgraphs** — `rtool` and `wtool` in FC-1 both have identical subgraph label `"bottom tools"`. Not an error, but ambiguous in plain-text readings.
8. **No formulas failed to parse.** All LaTeX `$$...$$` expressions copied verbatim; KaTeX validity not verified here but syntax appears clean.
