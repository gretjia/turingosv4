# V-004 Root Cause: kernel.rs hardcoded "[OMEGA]"

## WHY chain

1. **WHY** does kernel.rs contain the string "[OMEGA]"?
   - Because OMEGA detection was initially implemented as a kernel-level check: when a node is labeled "[OMEGA]", the kernel marks it resolved.

2. **WHY** was OMEGA detection put in the kernel?
   - Convenience. The kernel already traverses all nodes; adding a string check there was the simplest implementation path.

3. **WHY** is this a problem?
   - Law 1: The kernel must have zero domain knowledge. It is a pure topological machine — it manages nodes, edges, and graph operations. What nodes *mean* is not its concern.

4. **WHY** does zero domain knowledge matter?
   - The Bitter Lesson: systems that hardcode domain knowledge fail to generalize. The kernel should work for any prediction market, not just mathematical proof markets.

5. **WHY** wasn't this caught earlier?
   - No automated purity check existed for the kernel module. The rule was philosophical ("kernel should be pure") but not tested.

## Root cause (one sentence)
OMEGA detection shortcut placed in kernel for convenience, violating Law 1's requirement that the topology layer have zero domain opinions.

## Contributing factors
- Convenience-driven implementation over principle-driven
- No automated kernel purity test (string literal scan, import scan)
- Law 1 was a design principle, not a tested invariant
