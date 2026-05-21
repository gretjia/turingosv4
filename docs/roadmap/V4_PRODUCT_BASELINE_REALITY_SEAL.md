# V4 Product Baseline Reality Seal

This document lists the machine-provable facts of the `main` branch codebase prior to the V4 Product-CAK Hardening execution.

## Machine-Provable Facts

### Fact 1: `/welcome` Route
The web server implements the `/welcome` route.
Command:
```bash
git grep -n '"/welcome"' src/web/router.rs
```

### Fact 2: `/build` Route
The web server implements the `/build` route.
Command:
```bash
git grep -n '"/build"' src/web/router.rs
```

### Fact 3: `/api/spec/submit` Route
The web server implements the `/api/spec/submit` route.
Command:
```bash
git grep -n '"/api/spec/submit"' src/web/router.rs
```

### Fact 4: `/api/spec/turn` Route
The web server implements the `/api/spec/turn` route.
Command:
```bash
git grep -n '"/api/spec/turn"' src/web/router.rs
```

### Fact 5: `/api/generate` Route
The web server implements the `/api/generate` route.
Command:
```bash
git grep -n '"/api/generate"' src/web/router.rs
```

### Fact 6: `/api/artifact/:session_id/:name` Route
The web server implements the `/api/artifact/:session_id/:name` route.
Command:
```bash
git grep -n '"/api/artifact/' src/web/router.rs
```

### Fact 7: `SPEC_CAPSULE_SCHEMA_ID` Definition
The spec capsule module defines `SPEC_CAPSULE_SCHEMA_ID` as `"turingos-spec-capsule-v1"`.
Command:
```bash
git grep -n 'SPEC_CAPSULE_SCHEMA_ID' src/runtime/spec_capsule.rs
```

### Fact 8: `turingos generate` CAS Exclusivity
The `generate` command writes only to the filesystem without writing to the Content Addressable Store (CAS).
Command:
```bash
git grep -n 'No CAS write' src/bin/turingos/cmd_generate.rs
```

### Fact 9: `CHAINTAPE_CAS_REF` Reference
The Git Chain module defines and references `CHAINTAPE_CAS_REF`.
Command:
```bash
git grep -n 'CHAINTAPE_CAS_REF' src/bottom_white/cas/git_chain.rs
```

### Fact 10: Web Route Smoke Integration Test Suite
The web routes are validated by the integration smoke test.
Command:
```bash
cargo test --features web --test cli_web_routes_smoke
```

## Features That Do Not Yet Exist
The following components do not yet exist in the codebase:
- `ArtifactBundle`
- `PreviewRunCapsule`
- `BuildSessionView` reconstructed from CAS
- Offline replay
- Spec audit
