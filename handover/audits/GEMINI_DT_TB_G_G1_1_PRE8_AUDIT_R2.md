Warning: True color (24-bit) support not detected. Using a terminal with true color enabled will result in a better visual experience.
YOLO mode is enabled. All tool calls will be automatically approved.
YOLO mode is enabled. All tool calls will be automatically approved.
Ripgrep is not available. Falling back to GrepTool.
```
Q1: PASS — cross-check explicitly covers missing-secret and pubkey-mismatch, returning `ResumeKeystoreInconsistent`.
Q2: PASS — `env=1` routes directly to `resume_existing_durable`, which explicitly returns `ManifestAbsentInResume` on missing manifest.
Q3: PASS — binary gate predicate strictly checks `TURINGOS_CHAINTAPE_RESUME == "1"`.
Q4: PASS — the fresh init path fallback strictly calls `generate_or_load_durable`.
Q5: PASS — drift between keystore and manifest is caught and tested, preventing replay divergence.
Q6: PASS — computed `sha256sum` for `src/runtime/agent_keypairs.rs` correctly starts with `4dc7de08`.
Q7: PASS — SG-G1.6 and SG-G1.7 are explicitly GREEN, demonstrating mechanism bounds.
Q8: PASS — no schema mutations or sequencer drift occurred; changes are isolated to binary init gates.
Q9: PASS — fail-closed invariant enforced for resume mode, fully aligning with FC2 §3.2 requirements.

Aggregate R2 verdict: PASS
Conviction: high
Recommendation: PROCEED-SHIP
```
