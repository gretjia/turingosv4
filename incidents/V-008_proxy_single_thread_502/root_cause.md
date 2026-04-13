# V-008: Root Cause Analysis

## WHY Chain
1. WHY did agents get 502? → Proxy rejected concurrent connections
2. WHY reject? → HTTPServer is single-threaded, can only handle one request at a time
3. WHY was it single-threaded? → Default Python stdlib behavior, not explicitly configured for concurrency
4. WHY wasn't this caught? → Initial testing used single curl request, not concurrent load
5. WHY no concurrent test? → Proxy was created to solve V-007 (HTTPS deadlock), concurrency wasn't the focus

## Structural Cause
Testing at the component level (single request) without system-level testing (15 concurrent agents). The proxy was validated against the wrong test: "does it forward one request?" vs "does it handle the actual production load?"
