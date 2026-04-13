# V-004 Resolution: kernel.rs hardcoded "[OMEGA]"

## Fix applied
1. **Removed all domain-specific strings from kernel.rs**
   - Deleted `[OMEGA]` check from `src/kernel/graph.rs`
   - Deleted `OMEGA_PREFIX` constant
   - Kernel now treats all nodes identically — pure topology

2. **Moved OMEGA detection to bus.rs SKILL layer**
   - `src/bus.rs` registers a SKILL handler for OMEGA detection
   - SKILL layer reads node labels and emits `OmegaDetected` events
   - Kernel receives resolution commands through the event bus, not by inspecting content

3. **Added kernel purity test**
   - `tests/kernel/purity_test.rs`: scans all files in `src/kernel/` for domain-specific strings
   - Blocklist: `OMEGA`, `proof`, `theorem`, `conjecture`, `lean`, `falsif`
   - Also scans imports: kernel modules cannot import from `engines/`, `skills/`, or `oracle/`
   - CI hard failure on any match

## Verification
- Kernel purity test passes: zero domain-specific strings in src/kernel/
- OMEGA detection still works end-to-end via bus.rs SKILL pathway
- Kernel can now theoretically run any domain — the Bitter Lesson test passes

## Rules created
- Kernel purity test added to CI (automated Law 1 enforcement)
