# TuringOS v4 Hardware Topology

## Nodes

| Node | Role | SSH | Specs |
|------|------|-----|-------|
| **omega-vm** (current) | GCP main, code repo, Git | localhost | 16GB, no GPU |
| **zephrymac-studio** | Architect Mac, Apple M4 32GB | `ssh zephrymac-studio` (ProxyJump hk-wg, port 2227) | Lean 4 installed |
| **linux1-lx** | Shenzhen workstation | `ssh linux1-lx` (ProxyJump hk-wg, port 2226) | AMD AI Max 395 128GB |
| **windows1-w1** | Shenzhen workstation | `ssh windows1-w1` (ProxyJump hk-wg, port 2228) | AMD AI Max 395 128GB |

## Network Routing
```
omega-vm → HK public jump (43.161.252.57) → WireGuard → Shenzhen LAN
```

## Key Gotchas
- Mac: VPN proxy at port 7897
- Windows: `-ngl 99` flag required, NSSM service bug
- Linux: primary compute node
- Cloud API: via local HTTP proxy only, never direct HTTPS from Rust (V-007 lesson)
- MAX_CONCURRENT_EVALUATORS must match researcher count

Full details: `handover/network_topology_and_ssh.md` (when migrated from v3)
