# Sandbox Boundary Baseline Evidence

Date: 2026-05-21
Risk class: Class 0 evidence record
Worktree: `/home/zephryj/projects/turingosv4-worktrees/pr-a-boundary`
Base: `origin/main` worktree for PR-A boundary hygiene

This is a new evidence record. It does not rewrite old history and does not
claim OS-level hermeticity.

## Command: `rg -n "Command::new|tokio::process::Command" src`

Exit: 0

Summary: command-spawning surfaces exist in CLI/dev/web/spec/sandbox paths.

```text
src/bin/turingos_dev.rs:181:    let tracked = Command::new("git")
src/bin/turingos_dev.rs:195:        let tracked_status = Command::new("git")
src/bin/turingos_dev.rs:241:    let output = Command::new("git")
src/bin/turingos/cmd_spec.rs:704:    let output = std::process::Command::new(exe)
src/bin/turingos/cmd_spec.rs:793:    let output = std::process::Command::new(exe)
src/bin/turingos/cmd_render.rs:135:    let status = Command::new("python3")
src/bin/turingos/cmd_wizard.rs:298:        let _ = Command::new(opener).arg(&artifact).status();
src/bin/turingos/cmd_wizard.rs:431:        let _ = Command::new("stty").arg("-echo").status();
src/bin/turingos/cmd_wizard.rs:439:        let _ = Command::new("stty").arg("echo").status();
src/bin/turingos/common.rs:80:///   4. bare `bin_name` -> `Command::new` PATH search.
src/bin/turingos/common.rs:106:    let status = Command::new(&bin_path)
src/bin/turingos/common.rs:147:/// `Command::new` performs a PATH search.
src/sdk/sandbox.rs:76:        let mut child = Command::new(&self.command)
src/web/welcome.rs:449:    let output = tokio::process::Command::new(&bin)
src/web/welcome.rs:527:    let output = tokio::process::Command::new(&bin)
src/web/welcome.rs:614:    let output = tokio::process::Command::new(&bin)
src/web/generate.rs:246:        let mut cmd = tokio::process::Command::new(&bin);
src/web/write.rs:19:/// The handler uses exec-style `tokio::process::Command::arg()` calls exclusively.
src/web/write.rs:273:    let output = tokio::process::Command::new(&bin)
src/runtime/dev_harness.rs:357:    let output = Command::new(command[0]).args(&command[1..]).output()?;
src/web/spec.rs:284:    let mut cmd = tokio::process::Command::new(&bin);
src/web/spec.rs:1059:            std::process::Command::new(&bin2)
src/web/spec.rs:1367:        std::process::Command::new(&bin2)
```

## Command: `rg -n "env_clear|unshare|seccomp|bwrap|chroot|netns|landlock|cgroup" src || true`

Exit: 0

Summary: no matches. Current source does not show OS-level isolation primitives
for env clearing, namespaces, seccomp, bubblewrap, chroot, Landlock, or cgroups.

## Command: `rg -n "reqwest::Client|\\.post\\(|ResilientLLMClient::generate|chat_complete_blocking" src`

Exit: 0

Summary: LLM and product generation paths still include network-capable client
and chat-completion call surfaces.

```text
src/drivers/llm_http.rs:88:    pub async fn generate(
src/drivers/llm_http.rs:92:        let client = reqwest::Client::builder()
src/drivers/llm_http.rs:107:                .post(&format!("{}/v1/chat/completions", self.proxy_url))
src/bin/turingos/siliconflow_client.rs:284:    let client = reqwest::Client::builder()
src/bin/turingos/siliconflow_client.rs:291:        .post(&format!("{}/v1/chat/completions", endpoint))
src/bin/turingos/siliconflow_client.rs:330:pub(crate) fn chat_complete_blocking(
src/bin/turingos/cmd_generate.rs:299:    let llm_res = chat_complete_blocking(&api_key, &model_id, &messages, Some(6000), Some(0.2), blackbox_thinking);
```

## Command: `rg -n "world_head_unchanged.*true|offline|sandbox" src tests docs README.md`

Exit: 0

Summary: many matches across replay/offline docs, sandbox-prefixed audit
assertions, preview sandbox policy, and rejection capsule world-head contracts.
Representative matches:

```text
README.md:71:                                `turingos replay --offline` with cross-CID
README.md:141:  - Artifact viewer uses `iframe sandbox="allow-scripts"`-only with
README.md:194:| [#52](https://github.com/gretjia/turingosv4/pull/52) | SQUASH-MERGED to `main` on 2026-05-21 | `a699dd61` | Atom C9: `turingos replay --offline` CLI flag wired to `runtime::replay::reconstruct_session()` (CAS-only). Existing 7-indicator ChainTape replay preserved as default mode. 3 spec-named tests added (`artifact_bundle_replay_reads_cas`, `build_session_replay_after_cache_delete`, `replay_verifies_all_cross_cid_references_resolve`). Static no-LLM proof via dependency grep (NOT runtime tracing). |
README.md:195:| [#51](https://github.com/gretjia/turingosv4/pull/51) | SQUASH-MERGED to `main` on 2026-05-21 | `0039bc6e` | Atom C8: L4.E `GenerateRejectionCapsule` HTTP shielding + 5 missing spec tests (`generate_fail_goes_l4e`, `user_error_does_not_leak_panic`, `privacy_fail_not_retryable`, `rejection_capsule_world_head_unchanged`, `rejection_capsule_4_tuple_present`). `world_head_unchanged: true` is writer contract; operationally verified via `CHAINTAPE_CAS_REF` <=+2 commit check. v5-derived 4-tuple invariant. §8 self-signed under user delegation. |
src/runtime/rejection_capsule.rs:48:    pub world_head_unchanged: bool,              // MUST be true (asserted)
src/bin/turingos/cmd_replay.rs:6://!   - `--offline`: CAS-only build-session reconstruction via
src/bin/turingos/cmd_replay.rs:42:    --offline                Run offline CAS-only replay (no shell-out).
src/bin/turingos/cmd_generate.rs:416:                world_head_unchanged: true,
src/bin/turingos/cmd_generate.rs:469:            world_head_unchanged: true,
src/bin/turingos/cmd_generate.rs:587:                            world_head_unchanged: true,
src/bin/turingos/cmd_generate.rs:618:                        world_head_unchanged: true,
src/sdk/sandbox.rs:1:// Tier 2: Isolated process sandbox — external verifier with timeout + SIGKILL
tests/offline_replay_no_llm_dependency_static_check.rs:1://! C9 static check: offline replay modules must not import LLM/network clients.
tests/rejection_capsule_world_head_unchanged.rs:92:fn test_world_head_unchanged_field_is_true_in_capsule_body() {
```

## Command: `cargo test --test offline_replay_no_llm_dependency_static_check`

Exit: 0

Result summary:

```text
running 1 test
test test_offline_replay_no_llm_dependency_static_check ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

The build emitted pre-existing warnings in library and `turingos` binary code;
this evidence record does not modify those source files.
