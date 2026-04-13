#!/bin/bash
# constitutional_check.sh — Living Harness: 宪法对齐检查
#
# 验证代码是否遵守 TuringOS 三法 + 架构不变量
# 不修改任何文件, 只报告 PASS/FAIL
#
# Usage: bash scripts/constitutional_check.sh
# Exit: 0 = all pass, 1 = violations found

set -uo pipefail
# Note: NOT using -e because grep returns 1 when no match (which is expected)

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

VIOLATIONS=0
PASSES=0
WARNINGS=0

pass() { echo "  ✓ $1"; ((PASSES++)); }
fail() { echo "  ✗ $1"; ((VIOLATIONS++)); }
warn() { echo "  ⚠ $1"; ((WARNINGS++)); }

echo "╔══════════════════════════════════════════════════════════╗"
echo "║  TURINGOS CONSTITUTIONAL ALIGNMENT CHECK                ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""

# ════════════════════════════════════════════════════════════
# LAW 1: kernel.rs ZERO domain knowledge
# ════════════════════════════════════════════════════════════
echo "=== LAW 1: Information Platziehen (Kernel Purity) ==="

# Note: "omega" excluded — it's the project's own OMEGA (market settlement) concept, not Lean tactic
DOMAIN_TERMS="\b(theorem|proof|sorry|simp|decide|apply\?|exact\?|tactic|lemma|hypothesis|Lean|mathlib)\b"
if grep -qiP "$DOMAIN_TERMS" src/kernel.rs 2>/dev/null; then
    MATCHES=$(grep -ciP "$DOMAIN_TERMS" src/kernel.rs 2>/dev/null || echo 0)
    fail "kernel.rs contains $MATCHES domain-specific terms (V-004 class violation)"
    grep -niP "$DOMAIN_TERMS" src/kernel.rs 2>/dev/null | head -5 | sed 's/^/    /'
else
    pass "kernel.rs: zero domain knowledge"
fi

# Check kernel.rs doesn't contain format strings with domain terms
if grep -qP '(format!|println!).*\b(OMEGA|proof|theorem)\b' src/kernel.rs 2>/dev/null; then
    fail "kernel.rs contains domain terms in format strings"
else
    pass "kernel.rs: no domain terms in format strings"
fi

echo ""

# ════════════════════════════════════════════════════════════
# LAW 2: Pure Capital Economy (invest-only, no minting)
# ════════════════════════════════════════════════════════════
echo "=== LAW 2: Pure Capital Economy ==="

# No fund_agent / coin minting
# Exclude Rust comments (// lines)
MINT_PATTERNS="fund_agent|mint_coins|add_balance|print_money|create_coins|new_balance.*=.*[0-9]"
if grep -rP "$MINT_PATTERNS" src/ 2>/dev/null | grep -vP ":\s*//" | grep -qP "$MINT_PATTERNS" 2>/dev/null; then
    fail "Post-genesis coin minting detected (V-001/V-002 class)"
    grep -rnP "$MINT_PATTERNS" src/ 2>/dev/null | head -5 | sed 's/^/    /'
else
    pass "No post-genesis coin minting in src/"
fi

# Check redistribute_pool pattern
if grep -rP "redistribute|rebirth.*balance|reset.*balance" src/ 2>/dev/null | grep -vP ":\s*//" | grep -qP "redistribute|rebirth" 2>/dev/null; then
    fail "Balance redistribution/rebirth detected (V-002 class)"
else
    pass "No balance redistribution/rebirth"
fi

# Check investment is voluntary (PASS/NOP option exists)
if grep -rqP "Pass|Nop|pass.*action|NOP" src/sdk/ 2>/dev/null; then
    pass "PASS/NOP action exists (voluntary investment)"
else
    warn "No PASS/NOP action found — investment may be forced (V-006)"
fi

# CTF conservation: 1 coin → 1 YES + 1 NO
if grep -rqP "mint.*yes.*no|YES.*NO.*conservation|ctf" src/prediction_market.rs 2>/dev/null; then
    pass "CTF conservation pattern found in prediction_market.rs"
else
    warn "CTF conservation not explicitly verified"
fi

echo ""

# ════════════════════════════════════════════════════════════
# ENGINE SEPARATION
# ════════════════════════════════════════════════════════════
echo "=== ENGINE SEPARATION ==="

# Engine 3 (Oracle) should NOT be in kernel.rs
if grep -qP "lean|sandbox|compile|verify" src/kernel.rs 2>/dev/null; then
    fail "Oracle/verification logic leaked into kernel.rs (V-003 class)"
else
    pass "kernel.rs free of oracle/verification logic"
fi

# Engine 2 (Markets) should be in prediction_market.rs, not kernel.rs
if grep -qP "BinaryMarket|market_price|swap|liquidity" src/kernel.rs 2>/dev/null; then
    # kernel.rs can reference BinaryMarket type but shouldn't implement market logic
    MARKET_IMPL=$(grep -cP "fn.*swap|fn.*trade|fn.*mint_and_swap" src/kernel.rs 2>/dev/null | head -1 || echo "0")
    MARKET_IMPL="${MARKET_IMPL//[^0-9]/}"
    MARKET_IMPL="${MARKET_IMPL:-0}"
    if [ "$MARKET_IMPL" -gt 0 ]; then
        fail "Market implementation logic in kernel.rs (should be in prediction_market.rs)"
    else
        pass "kernel.rs references but doesn't implement market logic"
    fi
else
    pass "kernel.rs free of market logic"
fi

echo ""

# ════════════════════════════════════════════════════════════
# RULE 21: One-step-per-node (payload limits)
# ════════════════════════════════════════════════════════════
echo "=== RULE 21: One-Step-Per-Node ==="

PAYLOAD_CHARS=$(grep -rP "max_payload_chars|payload.*chars.*=\s*\d+" src/ 2>/dev/null | grep -oP '\d{3,}' | head -1)
PAYLOAD_LINES=$(grep -rP "max_payload_lines|payload.*lines.*=\s*\d+" src/ 2>/dev/null | grep -oP '\d+' | head -1)

if [ -n "$PAYLOAD_CHARS" ]; then
    if [ "$PAYLOAD_CHARS" -le 1500 ]; then
        pass "max_payload_chars=$PAYLOAD_CHARS (≤1500)"
    else
        warn "max_payload_chars=$PAYLOAD_CHARS (>1500, may enable multi-step packing)"
    fi
else
    warn "max_payload_chars not found in code"
fi

echo ""

# ════════════════════════════════════════════════════════════
# RULE 22: Black-box (no Lean syntax in prompts)
# ════════════════════════════════════════════════════════════
echo "=== RULE 22: Black-Box (Traditional Math Only) ==="

LEAN_IN_PROMPTS=0
for prompt_file in $(find . -path "*/prompt/*.txt" -o -path "*/prompt/*.md" 2>/dev/null); do
    if grep -qP "\bsorry\b|\bsimp\b|\bdecide\b|\bapply\?|\bexact\?|\brfl\b|\bintro\b.*\bhyp\b" "$prompt_file" 2>/dev/null; then
        fail "Lean syntax in $prompt_file"
        LEAN_IN_PROMPTS=1
    fi
done
if [ "$LEAN_IN_PROMPTS" -eq 0 ]; then
    pass "No Lean syntax in agent prompts"
fi

echo ""

# ════════════════════════════════════════════════════════════
# V-009: FORMAT CONTRACT RESILIENCE (protocol.rs)
# ════════════════════════════════════════════════════════════
echo "=== V-009: LLM Output Parser Resilience ==="

# Check that protocol.rs has the three tolerance layers
if grep -q "fix_json_escapes" src/sdk/protocol.rs 2>/dev/null; then
    pass "LaTeX escape handler present (fix_json_escapes)"
else
    fail "Missing LaTeX escape handler in protocol.rs (V-009 regression)"
fi

if grep -qP 'find\(.*\{' src/sdk/protocol.rs 2>/dev/null; then
    pass "JSON prefix tolerance present (find '{')"
else
    fail "Missing JSON prefix tolerance in protocol.rs (V-009 regression)"
fi

if grep -q "preceding" src/sdk/protocol.rs 2>/dev/null || grep -q "bare.*tool\|tool_name.*trim" src/sdk/protocol.rs 2>/dev/null; then
    pass "Bare action tag fallback present"
else
    warn "Bare action tag fallback may be missing in protocol.rs"
fi

echo ""

# ════════════════════════════════════════════════════════════
# V-008: PROXY CONCURRENCY
# ════════════════════════════════════════════════════════════
echo "=== V-008: Proxy Concurrency ==="

if [ -f src/drivers/llm_proxy.py ]; then
    if grep -q "ThreadingMixIn" src/drivers/llm_proxy.py 2>/dev/null; then
        pass "llm_proxy.py uses ThreadingMixIn (concurrent)"
    else
        fail "llm_proxy.py is single-threaded — will 502 under concurrent agents (V-008)"
    fi
else
    warn "llm_proxy.py not found (proxy may not be needed if using local llama.cpp only)"
fi

echo ""

# ════════════════════════════════════════════════════════════
# COMPILATION CHECK
# ════════════════════════════════════════════════════════════
echo "=== COMPILATION ==="

if cargo check 2>/dev/null; then
    pass "cargo check PASSED"
else
    fail "cargo check FAILED"
fi

echo ""

# ════════════════════════════════════════════════════════════
# SUMMARY
# ════════════════════════════════════════════════════════════
echo "════════════════════════════════════════════════════════"
echo "  PASSES: $PASSES | VIOLATIONS: $VIOLATIONS | WARNINGS: $WARNINGS"

if [ "$VIOLATIONS" -gt 0 ]; then
    echo "  VERDICT: ⛔ FAIL — $VIOLATIONS constitutional violation(s)"
    exit 1
else
    echo "  VERDICT: ✓ PASS"
    exit 0
fi
