#!/usr/bin/env python3
"""Phase 6.3 — structural verification of generated Tic-Tac-Toe artifact.

Verifies (without launching a browser):
  - At least one HTML file emitted (or one Python file with tk/pygame).
  - For HTML: 9 cell-like elements (grid cells), `X` and `O` referenced in JS,
    some kind of reset / new-game button, a click handler.
  - The file is parseable HTML (html.parser doesn't error).
  - Code does NOT include forbidden out-of-scope features (per spec):
    no login, no online multiplayer, no leaderboard, no AI opponent.

Exit 0 if all checks pass; non-zero with a clear failure message otherwise.
"""
import sys
import re
from pathlib import Path
from html.parser import HTMLParser


class CountingParser(HTMLParser):
    def __init__(self):
        super().__init__()
        self.tags = {}
        self.parse_ok = True
        self.last_err = None

    def handle_starttag(self, tag, attrs):
        self.tags[tag] = self.tags.get(tag, 0) + 1

    def error(self, message):
        self.parse_ok = False
        self.last_err = message


def verify_html(path: Path) -> tuple[bool, list[str]]:
    failures = []
    content = path.read_text(encoding="utf-8", errors="replace")

    # 1. HTML parses cleanly
    parser = CountingParser()
    try:
        parser.feed(content)
    except Exception as e:
        failures.append(f"HTML parse error: {e}")

    # 2. ≥ 9 grid-cell candidates (div/button/td/cell)
    cell_tags = (
        parser.tags.get("td", 0)
        + parser.tags.get("button", 0)
        + parser.tags.get("div", 0)
    )
    if cell_tags < 9:
        failures.append(f"insufficient cell candidates (td+button+div={cell_tags}, need ≥9)")

    # 3. X and O referenced in JS or rendered text
    if "X" not in content or "O" not in content:
        failures.append("X or O symbol missing from page content")

    # 4. Some kind of reset/new-game capability
    reset_hints = [
        "再来一局", "重新开始", "重置", "new game", "newGame", "restart",
        "reset", "Reset", "Restart", "New Game"
    ]
    if not any(h in content for h in reset_hints):
        failures.append("no reset / new-game capability detected")

    # 5. Click handler present (onclick, addEventListener)
    if "onclick" not in content.lower() and "addeventlistener" not in content.lower():
        failures.append("no click handler detected (onclick / addEventListener)")

    # 6. Forbidden out-of-scope features (per spec answers Q6)
    forbidden = {
        "login form": [r"\blogin\b", r"\bsignup\b", r"\bregister\b"],
        "online multiplayer": [r"websocket", r"\bsocket\.io\b", r"\bmultiplayer\b"],
        "leaderboard": [r"leaderboard", r"\branking\b", r"high.?score"],
        # AI opponent: minimax / alpha-beta / ai-move logic
        "AI opponent": [r"\bminimax\b", r"alpha.?beta", r"computer.?move", r"\bbot\b"],
    }
    lc = content.lower()
    for feature, patterns in forbidden.items():
        for pat in patterns:
            if re.search(pat, lc):
                # Allow comments mentioning the feature is OUT of scope
                # (e.g. "// no AI opponent" is fine)
                excerpt_idx = re.search(pat, lc).start()
                excerpt = lc[max(0, excerpt_idx - 40):excerpt_idx + 40]
                if any(neg in excerpt for neg in ["//", "<!--", "no ", "not ", "without"]):
                    continue
                failures.append(f"OUT-OF-SCOPE feature found: {feature} (pattern '{pat}')")
                break

    return (len(failures) == 0, failures)


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: phase63_verify_artifact.py <artifacts_dir>", file=sys.stderr)
        return 2
    artifacts_dir = Path(sys.argv[1])
    if not artifacts_dir.is_dir():
        print(f"[FAIL] artifacts dir missing: {artifacts_dir}", file=sys.stderr)
        return 1

    html_files = list(artifacts_dir.rglob("*.html"))
    htm_files = list(artifacts_dir.rglob("*.htm"))
    all_html = html_files + htm_files

    if not all_html:
        # Python fallback path — accept .py with tk/pygame at least
        py_files = list(artifacts_dir.rglob("*.py"))
        if not py_files:
            print(f"[FAIL] no HTML or Python files in {artifacts_dir}", file=sys.stderr)
            return 1
        # Bare Python emission isn't a tic-tac-toe browser game; flag for
        # operator awareness.
        print(f"[WARN] no HTML emitted; only .py files: {[p.name for p in py_files]}")
        return 1

    overall_ok = True
    for h in all_html:
        ok, fails = verify_html(h)
        status = "PASS" if ok else "FAIL"
        print(f"[{status}] {h.relative_to(artifacts_dir)}")
        for f in fails:
            print(f"        - {f}")
        if not ok:
            overall_ok = False

    if overall_ok:
        print(f"\n[OK] {len(all_html)} HTML file(s) verified structurally.")
        return 0
    else:
        print("\n[FAIL] one or more HTML files failed structural checks.")
        return 1


if __name__ == "__main__":
    sys.exit(main())
