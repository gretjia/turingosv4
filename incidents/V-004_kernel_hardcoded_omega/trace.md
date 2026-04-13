# V-004 Trace: kernel.rs hardcoded "[OMEGA]"

## Timeline

### Discovery — Architect code review (2026-03-22)
1. Architect performs Bitter Lesson compliance audit on kernel.rs
2. grep for domain-specific strings in src/kernel/:
   ```
   grep -rn "OMEGA\|proof\|theorem\|conjecture\|lean" src/kernel/
   ```
3. Hit: `src/kernel/graph.rs:147`:
   ```rust
   if node.label.contains("[OMEGA]") {
       self.mark_resolved(node_id, ResolutionStatus::Proven);
   }
   ```
4. Additional hit: `src/kernel/graph.rs:152`:
   ```rust
   const OMEGA_PREFIX: &str = "[OMEGA]";
   ```

### Impact analysis
1. kernel.rs is the pure topology layer — it should know nothing about what nodes represent
2. The kernel is treating `[OMEGA]` as a special string, giving it semantic meaning
3. This means kernel behavior changes based on node content — it has domain opinions
4. If the OMEGA detection format changes, kernel.rs must change — tight coupling

### Bitter Lesson test
- Q: Could this kernel run a completely different domain (e.g., supply chain, biology)?
- A: No — it would break on nodes that don't have "[OMEGA]" labels
- Verdict: **Law 1 violated** — kernel has domain knowledge

## Detection method
- Manual architect audit with grep for domain-specific string literals in kernel module
