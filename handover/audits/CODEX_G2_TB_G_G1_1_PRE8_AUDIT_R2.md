OpenAI Codex v0.130.0
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: danger-full-access
reasoning effort: xhigh
reasoning summaries: none
session id: 019e1721-2a54-7fe1-a05e-d82046772c67
--------
user
TB-G G1.1 R2 PRE-§8 audit (closure of Codex R1.5 Q1/Q2/Q3/Q8 CHALLENGES).
Working dir /home/zephryj/projects/turingosv4, branch
feat/g1-1-resume-mode. Round 2 of round-cap=2.

R1 (kernel): Codex G2 + Gemini Pro both PASS Q1..Q12 + Constitutional
Alignment, high, PROCEED.

R1.5 (binary scope expansion): Codex CHALLENGE Q1/Q2/Q3/Q8 high R2.
Gemini Pro PASS Q1..Q10 high PROCEED-SHIP.

Per `feedback_dual_audit_conflict` conservative-merge: CHALLENGE wins →
R2 required. Code & tests addressed per below.

R2 CHANGES (since R1.5 ship-discharge attempt):

1. `src/runtime/agent_keypairs.rs`:
   - `resume_existing_durable` now PARSES the manifest into a typed
     `AgentPubkeyManifest` (was previously discarded via `_parsed`).
   - For every agent listed in the manifest, the keystore-loaded
     keypairs MUST contain that agent_id AND derive a pubkey matching
     the manifest's hex pubkey verbatim. Two distinct cross-check
     failures both fail-closed:
     • missing-secret → `ResumeKeystoreInconsistent { agent_id,
       reason: "...no corresponding secret..." }`
     • pubkey mismatch → `ResumeKeystoreInconsistent { agent_id,
       reason: "manifest pubkey ... does NOT match keystore-derived
       pubkey ..." }`
   - NEW `AgentKeypairError::ResumeKeystoreInconsistent { agent_id,
     reason }` variant + Display impl. Wraps both edge cases above.
   - Trust Root rehash a2d0f3bf → 4dc7de08.

2. `experiments/minif2f_v4/src/chain_runtime.rs`:
   - Predicate change: was `resume_active = env=1 AND
     manifest_exists` (silent fall-through to `generate_or_load_durable`
     when manifest absent). Now `resume_requested = env=1` and
     dispatch goes through `resume_existing_durable` unconditionally
     when env=1 — which then internally fails-closed via
     `ManifestAbsentInResume` if the manifest is missing.
   - `.expect()` message expanded with diagnostic guidance for the
     three new fail-closed paths (ManifestAbsentInResume +
     ResumeKeystoreInconsistent×2 + keystore decrypt error).
   - Not Trust Root tracked.

3. `tests/constitution_g1_resume.rs`:
   - +SG-G1.6 `sg_g1_6_resume_existing_durable_fails_closed_when_manifest_absent`
     — invokes resume on an empty runtime_repo (no agent_pubkeys.json),
     asserts `ManifestAbsentInResume` echoes correct path.
   - +SG-G1.7 `sg_g1_7_resume_existing_durable_fails_closed_on_keystore_manifest_drift`
     — writes a manifest claiming an `Agent_phantom` agent that has NO
     corresponding secret in the durable keystore, asserts
     `ResumeKeystoreInconsistent` with agent_id="Agent_phantom" +
     reason mentioning "no corresponding secret".
   - All 7 SG-G1.* GREEN.

VALIDATION POST-R2:
- workspace: presumed unchanged from 1487/0/151 + 2 = 1489/0/151
- constitution gates: 309/0/1 (was 307; +1 G1.1 R1.5 + +2 R2 = +3 total).
- Trust Root: verify_trust_root_passes_on_intact_repo PASS.

YOUR JOB: terse audit of the R2 closures. Read the actual diff.

KEY FILES TO READ:
1. `src/runtime/agent_keypairs.rs` lines 312..395 (new
   `resume_existing_durable` body with cross-check) and lines 478..518
   (new error variant + Display).
2. `experiments/minif2f_v4/src/chain_runtime.rs` lines 231..290 (the
   new `resume_requested` predicate + expanded `.expect()`).
3. `tests/constitution_g1_resume.rs` SG-G1.6 + SG-G1.7 sections.

QUESTIONS (TERSE; 1-line verdict each PASS / CHALLENGE / VETO):

- Q1 (R1.5 Q1 closure): cross-check covers both missing-secret AND
  pubkey-mismatch cases.
- Q2 (R1.5 Q2 closure): `env=1 AND manifest absent` now fails closed
  with `ManifestAbsentInResume` (no silent fall-through).
- Q3 (R1.5 Q3 closure): binary gate is now `env=1` alone (not `env=1
  AND manifest_exists`); aligned with kernel gate semantic.
- Q4 (regression): fresh path (`env != "1"`) is byte-identical to
  pre-R2 / pre-R1.5 / pre-G1.1 behavior.
- Q5 (R1.5 Q8 closure): agent_registry replay determinism enforced.
  Empty keystore + populated manifest → fail-closed; tampered manifest
  pubkey → fail-closed; both binding-tested.
- Q6 (Trust Root): manifest entry for `src/runtime/agent_keypairs.rs`
  matches `sha256sum` (4dc7de08).
- Q7 (mechanism): SG-G1.6 + SG-G1.7 binding tests both GREEN. Per
  `feedback_norm_needs_mechanism`, every new fail-closed path has a
  CI test that proves it can fail (would fire if regressed).
- Q8 (no new admission): no new sequencer arm, no schema change, no
  canonical signing payload edit. Resume primitive remains binary-layer.
- Q9 (constitutional alignment): FC2 §3.2 agent_registry replay input
  is now enforced END-TO-END for resume mode. Closes the "断点续作是
  本项目的核心" user directive's specific failure mode.

OUTPUT (terse, no file dumps):

```
Q1: PASS|CHALLENGE|VETO — <1 line>
...
Q9: PASS|CHALLENGE|VETO — <1 line>

Aggregate R2 verdict: PASS|CHALLENGE|VETO
Conviction: low|medium|high
Recommendation: PROCEED-SHIP|R3|HALT
```

Note: per round-cap=2 + `feedback_elon_mode_policy`, if R2 still
CHALLENGE/VETO, escalate to `/harness-reflect` rather than R3.

codex
I’ll verify the scoped repo instructions first, then read the specific diff and run only the narrow checks needed for the audit verdicts.
exec
/bin/bash -lc "pwd && rg --files -g 'AGENTS.md' -g '"'!target'"'" in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
/home/zephryj/projects/turingosv4

exec
/bin/bash -lc 'git status --short --branch' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
## feat/g1-1-resume-mode...origin/feat/g1-1-resume-mode [ahead 1]
 M experiments/minif2f_v4/src/chain_runtime.rs
 M genesis_payload.toml
 M handover/audits/GEMINI_DT_TB_G_G1_1_PRE8_AUDIT_R1_5.md
 M rules/enforcement.log
 M src/runtime/agent_keypairs.rs
 M tests/constitution_g1_resume.rs
?? handover/audits/CODEX_G2_TB_G_G1_1_PRE8_AUDIT_R2.md
?? handover/audits/GEMINI_DT_TB_G_G1_1_PRE8_AUDIT_R2.md
?? handover/evidence/g_phase_g1_1_smoke_2026-05-11T13-01-12Z/

codex
The branch and working tree are as expected: the R2 files are modified, with unrelated audit/evidence artifacts also present. I didn’t find an `AGENTS.md` inside the repo yet, so I’m checking ancestor scopes before reading the code.
exec
/bin/bash -lc 'for d in /home/zephryj/projects/turingosv4 /home/zephryj/projects /home/zephryj /home /; do if [ -f "$d/AGENTS.md" ]; then printf '"'%s\\n' \""'$d/AGENTS.md"; fi; done' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
exec
/bin/bash -lc "nl -ba src/runtime/agent_keypairs.rs | sed -n '300,405p;470,530p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   300	    /// - **Manifest present** → parse it (fail-closed on parse error),
   301	    ///   load secrets from the durable keystore (same as
   302	    ///   `generate_or_load_durable`), reconstruct the in-memory
   303	    ///   keypair map from the secrets (the manifest is the public
   304	    ///   side of those secrets — replay-verifiable), and DO NOT
   305	    ///   re-persist the manifest on construction (preserves the
   306	    ///   existing on-disk bytes verbatim).
   307	    ///
   308	    /// Subsequent `sign()` calls that discover a new agent_id still
   309	    /// trigger `persist_manifest()` via `get_or_create()` — new agents
   310	    /// added during the resumed run are appended; existing agents
   311	    /// remain untouched.
   312	    pub fn resume_existing_durable(
   313	        runtime_repo_path: &Path,
   314	        durable_keystore_path: &Path,
   315	        password: secrecy::SecretString,
   316	    ) -> Result<Self, AgentKeypairError> {
   317	        let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
   318	        if !manifest_path.exists() {
   319	            return Err(AgentKeypairError::ManifestAbsentInResume {
   320	                path: manifest_path,
   321	            });
   322	        }
   323	        // Parse the existing manifest — the public side of the agent
   324	        // registry. Every agent listed here MUST have a secret in the
   325	        // durable keystore; otherwise the resumed registry would
   326	        // silently lose a signing capability and the tape's
   327	        // agent_registry replay input would diverge from the
   328	        // post-resume in-memory state.
   329	        let manifest_bytes = std::fs::read(&manifest_path).map_err(AgentKeypairError::Io)?;
   330	        let parsed: AgentPubkeyManifest = serde_json::from_slice(&manifest_bytes)
   331	            .map_err(|e| AgentKeypairError::Serde(format!("agent_pubkeys.json: {e}")))?;
   332	        let (secrets_map, _fresh) =
   333	            crate::runtime::agent_keystore::load_or_empty(durable_keystore_path, &password)
   334	                .map_err(|e| AgentKeypairError::Serde(format!("durable keystore: {e}")))?;
   335	        let mut keypairs: BTreeMap<AgentId, AgentKeypair> = BTreeMap::new();
   336	        for (agent_id_raw, seed) in secrets_map {
   337	            keypairs.insert(AgentId(agent_id_raw), AgentKeypair::from_secret_bytes(seed));
   338	        }
   339	
   340	        // TB-G G1.1 R2 closure (Codex G2 R1.5 Q1+Q8 CHALLENGE): cross-check
   341	        // every agent in the manifest MUST have a corresponding secret in
   342	        // the durable keystore AND the derived pubkey MUST match the
   343	        // manifest pubkey verbatim. Catches:
   344	        // (a) keystore was wiped while manifest survived (registry/keystore
   345	        //     drift),
   346	        // (b) keystore covers different agents (wrong keystore path / wrong
   347	        //     password),
   348	        // (c) manifest was tampered (manifest pubkey != derived pubkey).
   349	        // Fail-closed in all three cases — silent partial resume would
   350	        // violate FC2 §3.2 "agent_registry is a replay input" because the
   351	        // in-memory registry would no longer reproduce the on-disk
   352	        // manifest's binding.
   353	        for (agent_id_raw, manifest_pubkey_hex) in &parsed.agents {
   354	            let agent_id = AgentId(agent_id_raw.clone());
   355	            let keypair = keypairs.get(&agent_id).ok_or_else(|| {
   356	                AgentKeypairError::ResumeKeystoreInconsistent {
   357	                    agent_id: agent_id_raw.clone(),
   358	                    reason: format!(
   359	                        "agent_pubkeys.json lists agent_id={agent_id_raw:?} but the \
   360	                         durable keystore at {durable_keystore_path:?} has no \
   361	                         corresponding secret — keystore was wiped, password is \
   362	                         wrong, or the runtime_repo / keystore are from different runs"
   363	                    ),
   364	                }
   365	            })?;
   366	            let derived_pubkey_hex = keypair.public_key().to_hex();
   367	            if &derived_pubkey_hex != manifest_pubkey_hex {
   368	                return Err(AgentKeypairError::ResumeKeystoreInconsistent {
   369	                    agent_id: agent_id_raw.clone(),
   370	                    reason: format!(
   371	                        "manifest pubkey {manifest_pubkey_hex:?} does NOT match keystore-\
   372	                         derived pubkey {derived_pubkey_hex:?} — possible manifest \
   373	                         tampering or split-brain keystore"
   374	                    ),
   375	                });
   376	            }
   377	        }
   378	
   379	        Ok(Self {
   380	            keypairs,
   381	            manifest_path,
   382	            durable: Some(DurableConfig {
   383	                keystore_path: durable_keystore_path.to_path_buf(),
   384	                password,
   385	            }),
   386	        })
   387	    }
   388	
   389	    /// TRACE_MATRIX FC1-N14: get-or-create the keypair for `agent_id`. New
   390	    /// agents auto-generate a fresh keypair (and update the on-disk manifest);
   391	    /// existing agents return the cached keypair.
   392	    pub fn get_or_create(&mut self, agent_id: &AgentId) -> Result<&AgentKeypair, AgentKeypairError> {
   393	        if !self.keypairs.contains_key(agent_id) {
   394	            let kp = AgentKeypair::generate()?;
   395	            self.keypairs.insert(agent_id.clone(), kp);
   396	            self.persist_manifest()?;
   397	        }
   398	        Ok(self.keypairs.get(agent_id).expect("just inserted"))
   399	    }
   400	
   401	    /// TRACE_MATRIX FC1-N14: sign a 32-byte canonical digest under `agent_id`.
   402	    /// Generates the keypair on-demand if absent. This is the primary call
   403	    /// site for evaluator append-branch / OMEGA-branch routing in Atom 2/3.
   404	    pub fn sign(
   405	        &mut self,
   470	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   471	pub struct AgentPubkeyManifest {
   472	    /// AgentId.0 → AgentPublicKey hex
   473	    pub agents: BTreeMap<String, String>,
   474	}
   475	
   476	impl AgentPubkeyManifest {
   477	    /// TRACE_MATRIX FC1-N14: load and parse the manifest from disk.
   478	    pub fn load(path: &Path) -> Result<Self, AgentKeypairError> {
   479	        let mut f = OpenOptions::new().read(true).open(path)?;
   480	        let mut buf = Vec::new();
   481	        f.read_to_end(&mut buf)?;
   482	        let manifest: AgentPubkeyManifest = serde_json::from_slice(&buf)
   483	            .map_err(|e| AgentKeypairError::Serde(e.to_string()))?;
   484	        Ok(manifest)
   485	    }
   486	
   487	    /// TRACE_MATRIX FC1-N14: resolve an AgentId to its pinned public key
   488	    /// (None if unknown).
   489	    pub fn get(&self, agent_id: &AgentId) -> Option<AgentPublicKey> {
   490	        self.agents
   491	            .get(&agent_id.0)
   492	            .and_then(|hex| AgentPublicKey::from_hex(hex).ok())
   493	    }
   494	}
   495	
   496	// ── Verification (replay-side) ───────────────────────────────────────────────
   497	
   498	/// TRACE_MATRIX FC1-N14: verify an agent signature against a manifest-pinned
   499	/// public key. Returns `Ok(())` on valid signature; `Err(...)` otherwise.
   500	/// Used by Atom 4 `verify_chaintape` to re-check every WorkTx during replay.
   501	pub fn verify_agent_signature(
   502	    signature: &AgentSignature,
   503	    digest: &[u8; 32],
   504	    pubkey: &AgentPublicKey,
   505	) -> Result<(), AgentKeypairError> {
   506	    let verifying = VerifyingKey::from_bytes(pubkey.as_bytes())
   507	        .map_err(|e| AgentKeypairError::Verify(format!("from_bytes: {e}")))?;
   508	    let sig = Signature::from_bytes(signature.as_bytes());
   509	    verifying
   510	        .verify(digest, &sig)
   511	        .map_err(|e| AgentKeypairError::Verify(format!("verify: {e}")))
   512	}
   513	
   514	// ── Errors ───────────────────────────────────────────────────────────────────
   515	
   516	/// TRACE_MATRIX FC1-N14: agent keypair / manifest / signing error taxonomy.
   517	#[derive(Debug)]
   518	pub enum AgentKeypairError {
   519	    Io(std::io::Error),
   520	    Entropy(getrandom::Error),
   521	    Serde(String),
   522	    InvalidFormat(&'static str),
   523	    ManifestAlreadyExists { path: PathBuf },
   524	    /// TB-G G1.1 resume mode (architect §8 SIGNED 2026-05-11; user directive
   525	    /// "断点续作是本项目的核心"): `resume_existing_durable` was called but
   526	    /// the manifest at `path` does not exist. Fail-closed so callers can
   527	    /// distinguish "fresh-init was intended, manifest absent" from
   528	    /// "resume-was-intended, manifest absent" — the latter is an invariant
   529	    /// violation worth panicking on rather than silently reinitializing.
   530	    ManifestAbsentInResume { path: PathBuf },

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/chain_runtime.rs | sed -n '220,300p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   220	        // durable keystore at TURINGOS_AGENT_KEYSTORE_PATH (default
   221	        // ~/.turingos/keystore/agent_keystore.enc). Cross-run identity is the
   222	        // architect TB-9 mandate ("agent durable key registry" + "cross-run
   223	        // identity"; directive 2026-05-02 Part C line 1574). The keystore password
   224	        // is read from TURINGOS_AGENT_KEYSTORE_PASSWORD; if unset, a hardcoded
   225	        // local-dev fallback is used (acceptable for solo-runs per
   226	        // feedback_kolmogorov_compression "MVP env-var; production-grade prompt is
   227	        // post-v1.0 polish"). Tests / CI set the env var explicitly.
   228	        //
   229	        // Wrapped in Arc<Mutex<>> so the registry can be shared across the async
   230	        // run loop (interior mutability needed for AgentKeypairRegistry::sign).
   231	        let agent_keypairs: Option<Arc<Mutex<AgentKeypairRegistry>>> =
   232	            chaintape_bundle.as_ref().map(|b| {
   233	                let durable_path = turingosv4::runtime::agent_keystore::default_agent_keystore_path()
   234	                    .expect("[chaintape/tb9] resolve durable agent keystore path (set HOME or TURINGOS_AGENT_KEYSTORE_PATH)");
   235	                let pwd = turingosv4::runtime::agent_keystore::keystore_password_from_env();
   236	                // TB-G G1.1 (architect §8 SIGNED 2026-05-11; user directive
   237	                // "断点续作是本项目的核心" — Turing-machine fundamentalist
   238	                // reading of FC2 §3.2 "every real evidence run must be
   239	                // replayable from genesis_report + ChainTape + CAS + agent
   240	                // registry + system pubkeys"): on resume, the existing
   241	                // `agent_pubkeys.json` IS the agent registry — load it
   242	                // instead of fail-closing. Mirrors the kernel-side
   243	                // `bootstrap_resume_state` behavior for `pinned_pubkeys.json`.
   244	                //
   245	                // **R2 closure (Codex G2 R1.5 Q2+Q3 CHALLENGE 2026-05-11)**:
   246	                // the binary gate is ONLY on the env flag (NOT on
   247	                // manifest-existence). This way, when the user requests
   248	                // resume (`TURINGOS_CHAINTAPE_RESUME=1`) but the manifest
   249	                // is absent, the request routes to `resume_existing_durable`
   250	                // which fail-closes with `ManifestAbsentInResume` — instead
   251	                // of silently falling through to `generate_or_load_durable`
   252	                // which would CREATE a fresh manifest (violating the
   253	                // user-mandated "断点续作是本项目的核心" invariant).
   254	                //
   255	                // Predicate alignment with kernel: kernel's
   256	                // `bootstrap_resume_state` requires
   257	                // `config.resume_existing_chain && head_commit_oid().is_some()`
   258	                // — but a non-empty chain WITHOUT an agent_pubkeys.json
   259	                // is itself an inconsistency the binary must surface.
   260	                // Both layers now fail-closed on env=1 + missing critical
   261	                // input rather than silently degrading.
   262	                let resume_requested = matches!(
   263	                    std::env::var("TURINGOS_CHAINTAPE_RESUME").as_deref(),
   264	                    Ok("1")
   265	                );
   266	                let reg = if resume_requested {
   267	                    AgentKeypairRegistry::resume_existing_durable(
   268	                        &b.runtime_repo_path,
   269	                        &durable_path,
   270	                        pwd,
   271	                    )
   272	                    .expect(
   273	                        "[chaintape/tb9-resume] agent_keypairs resume must succeed \
   274	                         (TURINGOS_CHAINTAPE_RESUME=1 requested). On ManifestAbsentInResume: \
   275	                         the runtime_repo at this path was never agent-registered, so resume \
   276	                         is meaningless — point TURINGOS_CHAINTAPE_PATH at a runtime_repo \
   277	                         from a prior agent-registered run, or unset TURINGOS_CHAINTAPE_RESUME \
   278	                         to start a fresh registry. On ResumeKeystoreInconsistent: \
   279	                         agent_pubkeys.json and the durable keystore disagree about agent \
   280	                         identities — either the keystore was wiped while the manifest \
   281	                         survived, or TURINGOS_AGENT_KEYSTORE_PASSWORD does not match the \
   282	                         password used for the prior run. On a keystore decrypt error: \
   283	                         check TURINGOS_AGENT_KEYSTORE_PASSWORD.",
   284	                    )
   285	                } else {
   286	                    AgentKeypairRegistry::generate_or_load_durable(
   287	                        &b.runtime_repo_path,
   288	                        &durable_path,
   289	                        pwd,
   290	                    )
   291	                    .expect(
   292	                        "[chaintape/tb9] agent_keypairs durable init must succeed (fresh runtime_repo guarantees \
   293	                         manifest absent; if you see this on a non-fresh dir, see TB-6 NonEmptyRuntimeRepo or \
   294	                         enable TURINGOS_CHAINTAPE_RESUME=1 for G1.1 resume mode. \
   295	                         If you see a keystore decrypt error, check TURINGOS_AGENT_KEYSTORE_PASSWORD matches \
   296	                         the password used for the previous run.)",
   297	                    )
   298	                };
   299	                Arc::new(Mutex::new(reg))
   300	            });

exec
/bin/bash -lc "rg -n \"SG-G1\\.6|SG-G1\\.7|ManifestAbsentInResume|ResumeKeystoreInconsistent|sg_g1_6|sg_g1_7\" tests/constitution_g1_resume.rs src/runtime/agent_keypairs.rs experiments/minif2f_v4/src/chain_runtime.rs genesis_payload.toml" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
genesis_payload.toml:251:"src/runtime/agent_keypairs.rs" = "4dc7de08c46847f585a99c12035ee16ebe8126edcc6aff3c26a7fe1bd0217772"  # rehashed by TB-G G1.1 R2 (2026-05-11 session #40; Codex G2 R1.5 Q1+Q8 CHALLENGE closure): extended `resume_existing_durable` with mandatory cross-check — for every agent listed in `agent_pubkeys.json`, the durable keystore MUST contain a secret AND the derived pubkey MUST match the manifest pubkey verbatim. Catches (a) keystore wiped while manifest survived, (b) wrong-password keystore decoded as empty, (c) tampered manifest with mismatched pubkey. New `AgentKeypairError::ResumeKeystoreInconsistent { agent_id, reason }` variant + Display impl. 2 NEW SG-G1.6 / SG-G1.7 closure tests in `tests/constitution_g1_resume.rs` pin both fail-closed paths per `feedback_norm_needs_mechanism`. Predecessor a2d0f3bf superseded.  # rehashed by TB-G G1.1 (2026-05-11 session #40; user directive "断点续作是本项目的核心" — Turing-machine fundamentalist scope expansion authorized in-conversation): added pub `AgentKeypairRegistry::resume_existing_durable(runtime_repo_path, durable_keystore_path, password)` constructor + `AgentKeypairError::ManifestAbsentInResume { path }` variant. Resume constructor reads existing `agent_pubkeys.json` instead of fail-closing with `ManifestAlreadyExists` — required at the **binary** layer (evaluator) to complete G1.1 end-to-end persistence per FC2 §3.2 "every real evidence run must be replayable from genesis_report + ChainTape + CAS + agent registry + system pubkeys" (agent registry == agent_pubkeys.json). Mirrors kernel-side G1.1 pinned_pubkeys.json resume; gated by same `TURINGOS_CHAINTAPE_RESUME == "1"` env flag (no drift between kernel and binary layers). Pure additive (existing `open` + `generate_or_load_durable` semantics unchanged — back-compat). Predecessor a027ddb0 superseded.
src/runtime/agent_keypairs.rs:296:    /// - **Manifest absent** → fail-closed `ManifestAbsentInResume`.
src/runtime/agent_keypairs.rs:319:            return Err(AgentKeypairError::ManifestAbsentInResume {
src/runtime/agent_keypairs.rs:356:                AgentKeypairError::ResumeKeystoreInconsistent {
src/runtime/agent_keypairs.rs:368:                return Err(AgentKeypairError::ResumeKeystoreInconsistent {
src/runtime/agent_keypairs.rs:530:    ManifestAbsentInResume { path: PathBuf },
src/runtime/agent_keypairs.rs:538:    ResumeKeystoreInconsistent { agent_id: String, reason: String },
src/runtime/agent_keypairs.rs:552:            Self::ManifestAbsentInResume { path } => {
src/runtime/agent_keypairs.rs:560:            Self::ResumeKeystoreInconsistent { agent_id, reason } => {
tests/constitution_g1_resume.rs:412:// ── SG-G1.6 (R2 closure; Codex Q2 CHALLENGE) ────────────────────────────────
tests/constitution_g1_resume.rs:414:// `resume_existing_durable` fails closed with `ManifestAbsentInResume`
tests/constitution_g1_resume.rs:420:fn sg_g1_6_resume_existing_durable_fails_closed_when_manifest_absent() {
tests/constitution_g1_resume.rs:429:        Err(AgentKeypairError::ManifestAbsentInResume { path }) => {
tests/constitution_g1_resume.rs:433:                "SG-G1.6: ManifestAbsentInResume must echo the expected manifest path"
tests/constitution_g1_resume.rs:437:            "SG-G1.6: expected ManifestAbsentInResume; got {other:?}"
tests/constitution_g1_resume.rs:440:            "SG-G1.6: resume_existing_durable on missing manifest MUST fail-closed \
tests/constitution_g1_resume.rs:446:// ── SG-G1.7 (R2 closure; Codex Q1+Q8 CHALLENGE) ─────────────────────────────
tests/constitution_g1_resume.rs:448:// `resume_existing_durable` fails closed with `ResumeKeystoreInconsistent`
tests/constitution_g1_resume.rs:454:fn sg_g1_7_resume_existing_durable_fails_closed_on_keystore_manifest_drift() {
tests/constitution_g1_resume.rs:480:        Err(AgentKeypairError::ResumeKeystoreInconsistent { agent_id, reason }) => {
tests/constitution_g1_resume.rs:483:                "SG-G1.7: error must name the inconsistent agent_id"
tests/constitution_g1_resume.rs:487:                "SG-G1.7: reason should describe missing-secret case; got {reason:?}"
tests/constitution_g1_resume.rs:491:            "SG-G1.7: expected ResumeKeystoreInconsistent; got {other:?}"
tests/constitution_g1_resume.rs:494:            "SG-G1.7: resume_existing_durable with manifest agent but no keystore secret \
experiments/minif2f_v4/src/chain_runtime.rs:250:                // which fail-closes with `ManifestAbsentInResume` — instead
experiments/minif2f_v4/src/chain_runtime.rs:274:                         (TURINGOS_CHAINTAPE_RESUME=1 requested). On ManifestAbsentInResume: \
experiments/minif2f_v4/src/chain_runtime.rs:278:                         to start a fresh registry. On ResumeKeystoreInconsistent: \

codex
The core constructor now parses and uses the manifest, and the binary gate is visibly env-only. I’m checking the tests and the trust-root hash next, because those decide the mechanism and Trust Root questions.
exec
/bin/bash -lc "nl -ba tests/constitution_g1_resume.rs | sed -n '1,520p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-G G1.1 (architect §8 SIGNED 2026-05-11 "好，确认可以 ship";
     2	//! Canonical multi-clause Class-4 §8) — resume-mode genesis branch ship
     3	//! gates SG-G1.1..SG-G1.5.
     4	//!
     5	//! Architect verdict §1.3 verbatim (G-Phase directive):
     6	//!
     7	//! > 每个 problem fresh runtime_repo + fresh genesis — 每轮开局都把交易
     8	//! > 员洗白、清仓、重置记忆。市场不会涌现。
     9	//!
    10	//! G1.1 closes the loop by adding an env-gated resume admission path:
    11	//! `TURINGOS_CHAINTAPE_RESUME=1` lets `build_chaintape_sequencer` open a
    12	//! non-empty `refs/transitions/main` instead of fail-closing with
    13	//! `BootstrapError::NonEmptyRuntimeRepo`. Default-deny posture is
    14	//! preserved per CLAUDE.md §11 (resume=0 still fail-closes on non-empty
    15	//! repos — SG-G1.4 back-compat regression gate).
    16	//!
    17	//! FC-trace: FC2-Boot — every real evidence run must be replayable from
    18	//! `genesis_report + ChainTape + CAS + agent registry + system pubkeys`
    19	//! (CLAUDE.md §3.2). Resume IS the boot-time instance of that replay,
    20	//! seeded by `<runtime_repo>/initial_q_state.json` and consumed by the
    21	//! canonical `replay_full_transition` primitive shared with
    22	//! `verify_chaintape`. Existing Stage A3 SG-A3.4 covers replay
    23	//! byte-equality; G1.1 layers SG-G1.3 balances-byte-equal + SG-G1.5
    24	//! pinned-pubkey continuity on top.
    25	
    26	use tempfile::TempDir;
    27	use turingosv4::bus::{BusConfig, TuringBus};
    28	use turingosv4::economy::money::MicroCoin;
    29	use turingosv4::kernel::Kernel;
    30	use turingosv4::runtime::adapter::{genesis_with_balances, make_synthetic_task_open};
    31	use turingosv4::runtime::agent_keypairs::{AgentKeypairError, AgentKeypairRegistry};
    32	use turingosv4::runtime::{
    33	    build_chaintape_sequencer, build_chaintape_sequencer_with_initial_q, BootstrapError,
    34	    RuntimeChaintapeConfig,
    35	};
    36	use turingosv4::state::q_state::{AgentId, Hash};
    37	
    38	fn cfg_resume(tmp: &TempDir, run_id: &str, resume: bool) -> RuntimeChaintapeConfig {
    39	    RuntimeChaintapeConfig {
    40	        runtime_repo_path: tmp.path().join("runtime_repo"),
    41	        cas_path: tmp.path().join("cas"),
    42	        run_id: run_id.to_string(),
    43	        queue_capacity: 16,
    44	        resume_existing_chain: resume,
    45	    }
    46	}
    47	
    48	// ── SG-G1.1 ─────────────────────────────────────────────────────────────────
    49	//
    50	// Resume on empty repo == legacy genesis. With `resume_existing_chain=true`
    51	// but no existing chain, the resume short-circuit (`resume_active = false`
    52	// when `head_commit_oid().is_none()`) falls through to the fresh-genesis
    53	// path. Result: byte-equal `QState` to the legacy
    54	// `build_chaintape_sequencer` call.
    55	#[tokio::test]
    56	async fn sg_g1_1_resume_on_empty_repo_equals_legacy_genesis() {
    57	    // Fresh-genesis path with resume=true on an empty repo.
    58	    let tmp_a = TempDir::new().expect("tempdir_a");
    59	    let cfg_a = cfg_resume(&tmp_a, "g1_1-a", true);
    60	    let bundle_a = build_chaintape_sequencer(&cfg_a)
    61	        .expect("resume=true on empty repo bootstraps fresh (G1.1 SG-G1.1)");
    62	    let q_a = bundle_a.sequencer.q_snapshot().expect("q_snapshot a");
    63	    bundle_a.shutdown().await.expect("shutdown a");
    64	
    65	    // Same fresh-genesis path with resume=false.
    66	    let tmp_b = TempDir::new().expect("tempdir_b");
    67	    let cfg_b = cfg_resume(&tmp_b, "g1_1-b", false);
    68	    let bundle_b = build_chaintape_sequencer(&cfg_b).expect("legacy bootstrap b");
    69	    let q_b = bundle_b.sequencer.q_snapshot().expect("q_snapshot b");
    70	    bundle_b.shutdown().await.expect("shutdown b");
    71	
    72	    // Both QStates derive from `QState::genesis()` (same seed, no
    73	    // submitted txs, no economic mutation). Compare canonical roots —
    74	    // bit-exact equality across both branches.
    75	    assert_eq!(
    76	        q_a.state_root_t, q_b.state_root_t,
    77	        "SG-G1.1: state_root_t must match between resume=true/empty and resume=false/empty"
    78	    );
    79	    assert_eq!(
    80	        q_a.ledger_root_t, q_b.ledger_root_t,
    81	        "SG-G1.1: ledger_root_t must match"
    82	    );
    83	    assert_eq!(
    84	        q_a.economic_state_t, q_b.economic_state_t,
    85	        "SG-G1.1: economic_state_t must be byte-equal across branches"
    86	    );
    87	}
    88	
    89	// ── SG-G1.2 ─────────────────────────────────────────────────────────────────
    90	//
    91	// Resume on an N-entry chain sets `Sequencer.next_logical_t == N`. The
    92	// next commit's `Git2LedgerWriter::append` strict `len + 1` invariant
    93	// holds — proved by submitting one extra TaskOpen after resume and
    94	// observing the chain length advances from N → N+1.
    95	//
    96	// The test uses N=1 because making each subsequent `make_synthetic_task_open`
    97	// accept requires threading `parent_state_root` through the latest
    98	// `q_snapshot()` between submits, which races the async driver. N=1
    99	// fully proves the SG-G1.2 constitutional invariant
   100	// (`next_logical_t == chain_length`); the post-resume commit advances
   101	// 1 → 2 and pins the `Git2LedgerWriter` `len + 1` constraint.
   102	#[tokio::test]
   103	async fn sg_g1_2_resume_on_n_entry_chain_sets_next_logical_t_to_n() {
   104	    use turingosv4::bottom_white::ledger::transition_ledger::LedgerWriter;
   105	
   106	    let tmp = TempDir::new().expect("tempdir");
   107	    let cfg_fresh = cfg_resume(&tmp, "g1_2-fresh", false);
   108	
   109	    // Phase 1: fresh bootstrap, submit 1 TaskOpen (parent matches
   110	    // QState::genesis state_root = Hash::ZERO so it accepts).
   111	    let bundle = build_chaintape_sequencer(&cfg_fresh).expect("fresh bootstrap");
   112	    let kernel = Kernel::new();
   113	    let bus = TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer.clone());
   114	    let tx = make_synthetic_task_open("task-g1_2", "sponsor-g1_2", Hash::ZERO, "g1_2-1");
   115	    bus.submit_typed_tx(tx).await.expect("submit TaskOpen");
   116	    bundle.shutdown().await.expect("shutdown phase 1");
   117	    drop(bus);
   118	
   119	    // Reopen the writer to confirm chain length on disk.
   120	    let n_on_disk = {
   121	        let reopened = turingosv4::bottom_white::ledger::transition_ledger::Git2LedgerWriter::open(
   122	            &cfg_fresh.runtime_repo_path,
   123	        )
   124	        .expect("reopen writer");
   125	        reopened.len()
   126	    };
   127	    assert_eq!(
   128	        n_on_disk, 1,
   129	        "phase 1: chain should hold 1 accepted L4 entry before resume"
   130	    );
   131	
   132	    // Phase 2: resume bootstrap. next_logical_t must equal 1.
   133	    let cfg_r = cfg_resume(&tmp, "g1_2-resume", true);
   134	    let bundle_r = build_chaintape_sequencer(&cfg_r).expect("resume bootstrap");
   135	    assert_eq!(
   136	        bundle_r.sequencer.next_logical_t_peek(),
   137	        1,
   138	        "SG-G1.2: Sequencer.next_logical_t must equal chain_length on resume \
   139	         (chain has 1 entry → next_logical_t must be 1; the next commit signs as logical_t=2)"
   140	    );
   141	
   142	    // Phase 3: submit one more TaskOpen with parent = post-replay
   143	    // state_root. Chain advances 1 → 2 without
   144	    // `Git2LedgerWriter::append`'s strict `len + 1` invariant tripping.
   145	    let q_after_replay = bundle_r
   146	        .sequencer
   147	        .q_snapshot()
   148	        .expect("q_snapshot post-resume");
   149	    let kernel2 = Kernel::new();
   150	    let bus2 =
   151	        TuringBus::with_sequencer(kernel2, BusConfig::default(), bundle_r.sequencer.clone());
   152	    let tx_extra = make_synthetic_task_open(
   153	        "task-g1_2-post-resume",
   154	        "sponsor-g1_2",
   155	        q_after_replay.state_root_t,
   156	        "g1_2-post",
   157	    );
   158	    bus2.submit_typed_tx(tx_extra)
   159	        .await
   160	        .expect("submit post-resume TaskOpen");
   161	    bundle_r.shutdown().await.expect("shutdown phase 3");
   162	    drop(bus2);
   163	
   164	    let n_after = {
   165	        let reopened = turingosv4::bottom_white::ledger::transition_ledger::Git2LedgerWriter::open(
   166	            &cfg_r.runtime_repo_path,
   167	        )
   168	        .expect("reopen writer");
   169	        reopened.len()
   170	    };
   171	    assert_eq!(
   172	        n_after, 2,
   173	        "SG-G1.2: chain length must advance from 1 → 2 after one post-resume commit"
   174	    );
   175	}
   176	
   177	// ── SG-G1.3 ─────────────────────────────────────────────────────────────────
   178	//
   179	// Balances reconstruction matches forward replay. A pre-seeded
   180	// `genesis_with_balances` QState carried through a forward run produces
   181	// `balances_t_A`; the same chain replayed via resume produces
   182	// `balances_t_B`. The two must be byte-equal — that's the
   183	// constitutional FC2 replay-determinism guarantee + Stage A3 SG-A3.4
   184	// generalized down to per-account balances.
   185	#[tokio::test]
   186	async fn sg_g1_3_resume_balances_reconstruction_matches_forward_replay() {
   187	    let tmp = TempDir::new().expect("tempdir");
   188	    let cfg_fresh = cfg_resume(&tmp, "g1_3-fresh", false);
   189	
   190	    let alice = AgentId("alice-g1_3".into());
   191	    let bob = AgentId("bob-g1_3".into());
   192	    let initial_q = genesis_with_balances(&[
   193	        (alice.clone(), MicroCoin::from_coin(7).unwrap()),
   194	        (bob.clone(), MicroCoin::from_coin(11).unwrap()),
   195	    ]);
   196	
   197	    // Phase 1: forward run with pre-seeded balances + one TaskOpen.
   198	    // Clone the sequencer Arc *before* shutdown so q_snapshot post-drain
   199	    // observes the final applied state (driver may still be processing
   200	    // mid-submit; only post-shutdown is the canonical observation point).
   201	    let bundle =
   202	        build_chaintape_sequencer_with_initial_q(&cfg_fresh, initial_q.clone()).expect("fresh");
   203	    let seq_forward = bundle.sequencer.clone();
   204	    let kernel = Kernel::new();
   205	    let bus = TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer.clone());
   206	    let tx = make_synthetic_task_open("task-g1_3-1", "sponsor-g1_3", Hash::ZERO, "g1_3-1");
   207	    bus.submit_typed_tx(tx).await.expect("submit TaskOpen");
   208	    bundle.shutdown().await.expect("shutdown forward");
   209	    drop(bus);
   210	    let q_forward = seq_forward.q_snapshot().expect("q_snapshot forward");
   211	
   212	    // Phase 2: resume — replay reconstructs balances from initial_q +
   213	    // chain entries. Must produce byte-equal balances_t to forward run.
   214	    let cfg_r = cfg_resume(&tmp, "g1_3-resume", true);
   215	    let bundle_r = build_chaintape_sequencer(&cfg_r).expect("resume bootstrap");
   216	    let q_resumed = bundle_r.sequencer.q_snapshot().expect("q_snapshot resumed");
   217	    bundle_r.shutdown().await.expect("shutdown resume");
   218	
   219	    // Per-account assertion + full-map assertion. The per-account
   220	    // assertion gives a more readable failure mode if the test ever
   221	    // breaks; the full-map assertion catches any extra account that
   222	    // shouldn't exist (or a missing one).
   223	    assert_eq!(
   224	        q_resumed
   225	            .economic_state_t
   226	            .balances_t
   227	            .0
   228	            .get(&alice)
   229	            .copied()
   230	            .unwrap_or_else(MicroCoin::zero),
   231	        q_forward
   232	            .economic_state_t
   233	            .balances_t
   234	            .0
   235	            .get(&alice)
   236	            .copied()
   237	            .unwrap_or_else(MicroCoin::zero),
   238	        "SG-G1.3: alice balance must match between forward and resumed run"
   239	    );
   240	    assert_eq!(
   241	        q_resumed
   242	            .economic_state_t
   243	            .balances_t
   244	            .0
   245	            .get(&bob)
   246	            .copied()
   247	            .unwrap_or_else(MicroCoin::zero),
   248	        q_forward
   249	            .economic_state_t
   250	            .balances_t
   251	            .0
   252	            .get(&bob)
   253	            .copied()
   254	            .unwrap_or_else(MicroCoin::zero),
   255	        "SG-G1.3: bob balance must match"
   256	    );
   257	    assert_eq!(
   258	        q_resumed.economic_state_t.balances_t, q_forward.economic_state_t.balances_t,
   259	        "SG-G1.3: full balances_t map must be byte-equal across forward / resumed runs"
   260	    );
   261	    // Bound the state root for free — if balances diverge, state_root
   262	    // diverges; this is the constitutional FC2 replay-determinism
   263	    // guarantee applied to the entire economic_state.
   264	    assert_eq!(
   265	        q_resumed.state_root_t, q_forward.state_root_t,
   266	        "SG-G1.3: state_root_t must be byte-equal between forward and resumed run"
   267	    );
   268	}
   269	
   270	// ── SG-G1.4 ─────────────────────────────────────────────────────────────────
   271	//
   272	// Back-compat regression gate. With `resume_existing_chain=false`, a
   273	// non-empty `refs/transitions/main` still produces the original TB-6
   274	// `BootstrapError::NonEmptyRuntimeRepo`. All TB-N* / Stage C / Wave 3
   275	// 50p / TB-N3 Phase 2 smoke runs hit this branch unchanged.
   276	#[tokio::test]
   277	async fn sg_g1_4_non_empty_runtime_repo_only_fires_when_resume_false() {
   278	    let tmp = TempDir::new().expect("tempdir");
   279	    let cfg = cfg_resume(&tmp, "g1_4-fresh", false);
   280	
   281	    // Phase 1: fresh bootstrap, submit one TaskOpen to make the chain non-empty.
   282	    let bundle = build_chaintape_sequencer(&cfg).expect("fresh bootstrap");
   283	    let kernel = Kernel::new();
   284	    let bus = TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer.clone());
   285	    let tx = make_synthetic_task_open("task-g1_4", "sponsor-g1_4", Hash::ZERO, "g1_4");
   286	    bus.submit_typed_tx(tx).await.expect("submit TaskOpen");
   287	    bundle.shutdown().await.expect("shutdown");
   288	    drop(bus);
   289	
   290	    // Phase 2: re-bootstrap with resume=false on the non-empty repo.
   291	    // Must fail-closed with NonEmptyRuntimeRepo (the original TB-6 gate).
   292	    let cfg_reboot = cfg_resume(&tmp, "g1_4-reboot", false);
   293	    let result = build_chaintape_sequencer(&cfg_reboot);
   294	    match result {
   295	        Err(BootstrapError::NonEmptyRuntimeRepo {
   296	            path,
   297	            existing_head,
   298	        }) => {
   299	            assert_eq!(
   300	                path, cfg_reboot.runtime_repo_path,
   301	                "SG-G1.4: NonEmptyRuntimeRepo must echo the rejected runtime_repo_path"
   302	            );
   303	            assert!(
   304	                !existing_head.is_empty(),
   305	                "SG-G1.4: existing_head must be set when fail-closing"
   306	            );
   307	        }
   308	        Err(other) => panic!(
   309	            "SG-G1.4: expected BootstrapError::NonEmptyRuntimeRepo, got {other:?}"
   310	        ),
   311	        Ok(_) => panic!(
   312	            "SG-G1.4: resume=false on non-empty repo must NOT bootstrap successfully \
   313	             (would mask the TB-6 fail-closed gate for all prior smoke runs)"
   314	        ),
   315	    }
   316	
   317	    // Phase 3: same non-empty repo bootstraps cleanly when resume=true.
   318	    // Pins that the gap between "fail-closed" and "succeed" is exactly
   319	    // the `resume_existing_chain` field.
   320	    let cfg_resume = cfg_resume(&tmp, "g1_4-resume", true);
   321	    let bundle_r =
   322	        build_chaintape_sequencer(&cfg_resume).expect("resume=true on non-empty repo");
   323	    bundle_r.shutdown().await.expect("resume shutdown");
   324	}
   325	
   326	// ── SG-G1.5 ─────────────────────────────────────────────────────────────────
   327	//
   328	// Pinned-pubkey continuity. After resume, the original epoch's pubkey
   329	// entry MUST still be present in `pinned_pubkeys.json` (so prior L4
   330	// entries continue to verify). The manifest gains a NEW entry for the
   331	// new epoch — because Ed25519 secret keys are not persisted to disk,
   332	// resume cannot reuse the prior signing key; instead it generates a
   333	// new keypair for a new epoch + appends to the manifest. This is the
   334	// only correct way to preserve verification continuity for older
   335	// entries while letting the resumed sequencer sign new ones.
   336	#[tokio::test]
   337	async fn sg_g1_5_pinned_pubkeys_preserved_across_resume() {
   338	    let tmp = TempDir::new().expect("tempdir");
   339	    let cfg = cfg_resume(&tmp, "g1_5-fresh", false);
   340	
   341	    // Phase 1: fresh bootstrap writes the initial manifest.
   342	    let bundle = build_chaintape_sequencer(&cfg).expect("fresh bootstrap");
   343	    let manifest_path = cfg.runtime_repo_path.join("pinned_pubkeys.json");
   344	    let original_json = std::fs::read_to_string(&manifest_path).expect("read original manifest");
   345	    let original: serde_json::Value =
   346	        serde_json::from_str(&original_json).expect("parse original manifest");
   347	    let original_epoch = original["epoch"].as_u64().expect("epoch u64");
   348	    assert_eq!(original_epoch, 1, "fresh manifest pins epoch=1");
   349	    let original_pubkey_hex = original["pubkeys"][0]["pubkey_hex"]
   350	        .as_str()
   351	        .expect("pubkey_hex")
   352	        .to_string();
   353	
   354	    // Submit one TaskOpen so the chain is non-empty (resume admission requires it).
   355	    let kernel = Kernel::new();
   356	    let bus = TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer.clone());
   357	    let tx = make_synthetic_task_open("task-g1_5", "sponsor-g1_5", Hash::ZERO, "g1_5");
   358	    bus.submit_typed_tx(tx).await.expect("submit TaskOpen");
   359	    bundle.shutdown().await.expect("shutdown phase 1");
   360	    drop(bus);
   361	
   362	    // Phase 2: resume — manifest must still contain the original entry
   363	    // (epoch=1 pubkey unchanged) plus one new entry (epoch=2 with a
   364	    // different pubkey).
   365	    let cfg_r = cfg_resume(&tmp, "g1_5-resume", true);
   366	    let bundle_r = build_chaintape_sequencer(&cfg_r).expect("resume bootstrap");
   367	    let resumed_json =
   368	        std::fs::read_to_string(&manifest_path).expect("read manifest after resume");
   369	    let resumed: serde_json::Value =
   370	        serde_json::from_str(&resumed_json).expect("parse manifest after resume");
   371	    let resumed_pubkeys = resumed["pubkeys"]
   372	        .as_array()
   373	        .expect("pubkeys after resume must be an array");
   374	
   375	    assert!(
   376	        resumed_pubkeys.len() >= 2,
   377	        "SG-G1.5: manifest must gain at least one entry on resume (had {} after resume)",
   378	        resumed_pubkeys.len()
   379	    );
   380	    let original_still_present = resumed_pubkeys.iter().any(|e| {
   381	        e["epoch"].as_u64() == Some(original_epoch)
   382	            && e["pubkey_hex"].as_str() == Some(original_pubkey_hex.as_str())
   383	    });
   384	    assert!(
   385	        original_still_present,
   386	        "SG-G1.5: original (epoch={original_epoch}, pubkey={original_pubkey_hex}) MUST still be \
   387	         present in pinned_pubkeys.json after resume — prior L4 entries depend on this \
   388	         pubkey for system_signature verification"
   389	    );
   390	
   391	    let new_epoch = resumed["epoch"].as_u64().expect("epoch u64 after resume");
   392	    assert!(
   393	        new_epoch > original_epoch,
   394	        "SG-G1.5: top-level manifest epoch must advance on resume (was {original_epoch}, \
   395	         after resume {new_epoch})"
   396	    );
   397	    let new_entry_exists = resumed_pubkeys.iter().any(|e| {
   398	        e["epoch"].as_u64() == Some(new_epoch)
   399	            && e["pubkey_hex"].as_str().is_some()
   400	            && e["pubkey_hex"].as_str() != Some(original_pubkey_hex.as_str())
   401	    });
   402	    assert!(
   403	        new_entry_exists,
   404	        "SG-G1.5: a NEW (epoch={new_epoch}) entry with a distinct pubkey must be appended \
   405	         on resume — Ed25519 secrets aren't persisted, so the new sequencer must sign \
   406	         new L4 entries with a freshly generated keypair"
   407	    );
   408	
   409	    bundle_r.shutdown().await.expect("resume shutdown");
   410	}
   411	
   412	// ── SG-G1.6 (R2 closure; Codex Q2 CHALLENGE) ────────────────────────────────
   413	//
   414	// `resume_existing_durable` fails closed with `ManifestAbsentInResume`
   415	// when invoked on a runtime_repo where `agent_pubkeys.json` doesn't
   416	// exist. Binary-layer invariant: env=1 + manifest absent must NOT
   417	// silently degrade to fresh init. Mechanism per
   418	// `feedback_norm_needs_mechanism`.
   419	#[test]
   420	fn sg_g1_6_resume_existing_durable_fails_closed_when_manifest_absent() {
   421	    let tmp = TempDir::new().expect("tempdir");
   422	    let runtime_repo = tmp.path().join("runtime_repo");
   423	    std::fs::create_dir_all(&runtime_repo).expect("mkdir");
   424	    // No agent_pubkeys.json written.
   425	    let keystore = tmp.path().join("keystore.enc");
   426	    let pwd = secrecy::SecretString::new("test-password".to_string().into());
   427	    let result = AgentKeypairRegistry::resume_existing_durable(&runtime_repo, &keystore, pwd);
   428	    match result {
   429	        Err(AgentKeypairError::ManifestAbsentInResume { path }) => {
   430	            assert_eq!(
   431	                path,
   432	                runtime_repo.join("agent_pubkeys.json"),
   433	                "SG-G1.6: ManifestAbsentInResume must echo the expected manifest path"
   434	            );
   435	        }
   436	        Err(other) => panic!(
   437	            "SG-G1.6: expected ManifestAbsentInResume; got {other:?}"
   438	        ),
   439	        Ok(_) => panic!(
   440	            "SG-G1.6: resume_existing_durable on missing manifest MUST fail-closed \
   441	             (silent fall-through would degrade FC2 §3.2 agent_registry replay input)"
   442	        ),
   443	    }
   444	}
   445	
   446	// ── SG-G1.7 (R2 closure; Codex Q1+Q8 CHALLENGE) ─────────────────────────────
   447	//
   448	// `resume_existing_durable` fails closed with `ResumeKeystoreInconsistent`
   449	// when the manifest references an agent_id that has no corresponding
   450	// secret in the durable keystore. Catches: empty keystore + populated
   451	// manifest, wrong-password keystore reading as empty, keystore wiped
   452	// while manifest survived.
   453	#[test]
   454	fn sg_g1_7_resume_existing_durable_fails_closed_on_keystore_manifest_drift() {
   455	    use std::collections::BTreeMap;
   456	    let tmp = TempDir::new().expect("tempdir");
   457	    let runtime_repo = tmp.path().join("runtime_repo");
   458	    std::fs::create_dir_all(&runtime_repo).expect("mkdir");
   459	    // Write a manifest claiming an agent that is NOT in the durable keystore.
   460	    let mut agents = BTreeMap::new();
   461	    agents.insert(
   462	        "Agent_phantom".to_string(),
   463	        // 32-byte all-zeros pubkey hex placeholder — won't match any real key
   464	        "00".repeat(32),
   465	    );
   466	    let manifest = serde_json::json!({
   467	        "agents": agents,
   468	    });
   469	    std::fs::write(
   470	        runtime_repo.join("agent_pubkeys.json"),
   471	        serde_json::to_vec(&manifest).expect("serialize manifest"),
   472	    )
   473	    .expect("write manifest");
   474	
   475	    let keystore = tmp.path().join("keystore.enc");
   476	    // Empty keystore: never written, so `load_or_empty` returns empty.
   477	    let pwd = secrecy::SecretString::new("test-password".to_string().into());
   478	    let result = AgentKeypairRegistry::resume_existing_durable(&runtime_repo, &keystore, pwd);
   479	    match result {
   480	        Err(AgentKeypairError::ResumeKeystoreInconsistent { agent_id, reason }) => {
   481	            assert_eq!(
   482	                agent_id, "Agent_phantom",
   483	                "SG-G1.7: error must name the inconsistent agent_id"
   484	            );
   485	            assert!(
   486	                reason.contains("no corresponding secret"),
   487	                "SG-G1.7: reason should describe missing-secret case; got {reason:?}"
   488	            );
   489	        }
   490	        Err(other) => panic!(
   491	            "SG-G1.7: expected ResumeKeystoreInconsistent; got {other:?}"
   492	        ),
   493	        Ok(_) => panic!(
   494	            "SG-G1.7: resume_existing_durable with manifest agent but no keystore secret \
   495	             MUST fail-closed — silent boot would lose signing capability and violate \
   496	             FC2 §3.2 agent_registry replay determinism"
   497	        ),
   498	    }
   499	}

exec
/bin/bash -lc 'sha256sum src/runtime/agent_keypairs.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
4dc7de08c46847f585a99c12035ee16ebe8126edcc6aff3c26a7fe1bd0217772  src/runtime/agent_keypairs.rs

exec
/bin/bash -lc 'git diff -- src/runtime/agent_keypairs.rs experiments/minif2f_v4/src/chain_runtime.rs tests/constitution_g1_resume.rs genesis_payload.toml' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
diff --git a/experiments/minif2f_v4/src/chain_runtime.rs b/experiments/minif2f_v4/src/chain_runtime.rs
index d0216e6..05b247f 100644
--- a/experiments/minif2f_v4/src/chain_runtime.rs
+++ b/experiments/minif2f_v4/src/chain_runtime.rs
@@ -241,24 +241,46 @@ impl SharedChain {
                 // `agent_pubkeys.json` IS the agent registry — load it
                 // instead of fail-closing. Mirrors the kernel-side
                 // `bootstrap_resume_state` behavior for `pinned_pubkeys.json`.
-                // Env gate is the same `TURINGOS_CHAINTAPE_RESUME == "1"`
-                // strict equality used by `RuntimeChaintapeConfig::from_env`,
-                // so the two layers (kernel sequencer + binary agent
-                // registry) are gated by a single env flag — no drift.
-                let resume_active = matches!(
+                //
+                // **R2 closure (Codex G2 R1.5 Q2+Q3 CHALLENGE 2026-05-11)**:
+                // the binary gate is ONLY on the env flag (NOT on
+                // manifest-existence). This way, when the user requests
+                // resume (`TURINGOS_CHAINTAPE_RESUME=1`) but the manifest
+                // is absent, the request routes to `resume_existing_durable`
+                // which fail-closes with `ManifestAbsentInResume` — instead
+                // of silently falling through to `generate_or_load_durable`
+                // which would CREATE a fresh manifest (violating the
+                // user-mandated "断点续作是本项目的核心" invariant).
+                //
+                // Predicate alignment with kernel: kernel's
+                // `bootstrap_resume_state` requires
+                // `config.resume_existing_chain && head_commit_oid().is_some()`
+                // — but a non-empty chain WITHOUT an agent_pubkeys.json
+                // is itself an inconsistency the binary must surface.
+                // Both layers now fail-closed on env=1 + missing critical
+                // input rather than silently degrading.
+                let resume_requested = matches!(
                     std::env::var("TURINGOS_CHAINTAPE_RESUME").as_deref(),
                     Ok("1")
-                ) && b.runtime_repo_path.join("agent_pubkeys.json").exists();
-                let reg = if resume_active {
+                );
+                let reg = if resume_requested {
                     AgentKeypairRegistry::resume_existing_durable(
                         &b.runtime_repo_path,
                         &durable_path,
                         pwd,
                     )
                     .expect(
-                        "[chaintape/tb9-resume] agent_keypairs resume must succeed (TURINGOS_CHAINTAPE_RESUME=1 \
-                         requested but resume_existing_durable failed; check that agent_pubkeys.json + the \
-                         durable keystore both correspond to the same prior run).",
+                        "[chaintape/tb9-resume] agent_keypairs resume must succeed \
+                         (TURINGOS_CHAINTAPE_RESUME=1 requested). On ManifestAbsentInResume: \
+                         the runtime_repo at this path was never agent-registered, so resume \
+                         is meaningless — point TURINGOS_CHAINTAPE_PATH at a runtime_repo \
+                         from a prior agent-registered run, or unset TURINGOS_CHAINTAPE_RESUME \
+                         to start a fresh registry. On ResumeKeystoreInconsistent: \
+                         agent_pubkeys.json and the durable keystore disagree about agent \
+                         identities — either the keystore was wiped while the manifest \
+                         survived, or TURINGOS_AGENT_KEYSTORE_PASSWORD does not match the \
+                         password used for the prior run. On a keystore decrypt error: \
+                         check TURINGOS_AGENT_KEYSTORE_PASSWORD.",
                     )
                 } else {
                     AgentKeypairRegistry::generate_or_load_durable(
diff --git a/genesis_payload.toml b/genesis_payload.toml
index 23733cb..f36a7b8 100644
--- a/genesis_payload.toml
+++ b/genesis_payload.toml
@@ -248,7 +248,7 @@ boot_attestation_hash = "PENDING_v4_X_SELF_REFERENTIAL_COMPUTE"
 # 2026-05-01 TB-7 Atom 7 — Gate 7 legacy-bypass regression conformance test (NEW file). Repo-wide grep + comment-block walker that flags any unannotated bus.append / bus.append_oracle_accepted call in evaluator.rs. Includes positive/negative scanner controls. Per ARCHITECT_RULING §4 Gate 7 + charter §6 #31.
 "tests/tb_7_legacy_append_regression.rs" = "25bbfdefda6a2e2e1bbaa8092cf1870f6fd77914dc6b19af386a74db52bf7f38"
 # 2026-05-01 TB-7 Atom 1 — Per-agent Ed25519 keypair manager + on-disk pubkey manifest (NEW file). Run-local identity ONLY; private keys in process memory only and zeroed on drop. AgentKeypair / AgentKeypairRegistry / AgentPubkeyManifest / verify_agent_signature; mirrors PinnedSystemPubkeys structural pattern. Per ARCHITECT_RULING 2026-05-01 D2 + TB-7 charter §4.2.
-"src/runtime/agent_keypairs.rs" = "a2d0f3bfa50bd7f45e3f2c8fe51add525127d30d3f4e61302f9306916b95d44d"  # rehashed by TB-G G1.1 (2026-05-11 session #40; user directive "断点续作是本项目的核心" — Turing-machine fundamentalist scope expansion authorized in-conversation): added pub `AgentKeypairRegistry::resume_existing_durable(runtime_repo_path, durable_keystore_path, password)` constructor + `AgentKeypairError::ManifestAbsentInResume { path }` variant. Resume constructor reads existing `agent_pubkeys.json` instead of fail-closing with `ManifestAlreadyExists` — required at the **binary** layer (evaluator) to complete G1.1 end-to-end persistence per FC2 §3.2 "every real evidence run must be replayable from genesis_report + ChainTape + CAS + agent registry + system pubkeys" (agent registry == agent_pubkeys.json). Mirrors kernel-side G1.1 pinned_pubkeys.json resume; gated by same `TURINGOS_CHAINTAPE_RESUME == "1"` env flag (no drift between kernel and binary layers). Pure additive (existing `open` + `generate_or_load_durable` semantics unchanged — back-compat). Predecessor a027ddb0 superseded.
+"src/runtime/agent_keypairs.rs" = "4dc7de08c46847f585a99c12035ee16ebe8126edcc6aff3c26a7fe1bd0217772"  # rehashed by TB-G G1.1 R2 (2026-05-11 session #40; Codex G2 R1.5 Q1+Q8 CHALLENGE closure): extended `resume_existing_durable` with mandatory cross-check — for every agent listed in `agent_pubkeys.json`, the durable keystore MUST contain a secret AND the derived pubkey MUST match the manifest pubkey verbatim. Catches (a) keystore wiped while manifest survived, (b) wrong-password keystore decoded as empty, (c) tampered manifest with mismatched pubkey. New `AgentKeypairError::ResumeKeystoreInconsistent { agent_id, reason }` variant + Display impl. 2 NEW SG-G1.6 / SG-G1.7 closure tests in `tests/constitution_g1_resume.rs` pin both fail-closed paths per `feedback_norm_needs_mechanism`. Predecessor a2d0f3bf superseded.  # rehashed by TB-G G1.1 (2026-05-11 session #40; user directive "断点续作是本项目的核心" — Turing-machine fundamentalist scope expansion authorized in-conversation): added pub `AgentKeypairRegistry::resume_existing_durable(runtime_repo_path, durable_keystore_path, password)` constructor + `AgentKeypairError::ManifestAbsentInResume { path }` variant. Resume constructor reads existing `agent_pubkeys.json` instead of fail-closing with `ManifestAlreadyExists` — required at the **binary** layer (evaluator) to complete G1.1 end-to-end persistence per FC2 §3.2 "every real evidence run must be replayable from genesis_report + ChainTape + CAS + agent registry + system pubkeys" (agent registry == agent_pubkeys.json). Mirrors kernel-side G1.1 pinned_pubkeys.json resume; gated by same `TURINGOS_CHAINTAPE_RESUME == "1"` env flag (no drift between kernel and binary layers). Pure additive (existing `open` + `generate_or_load_durable` semantics unchanged — back-compat). Predecessor a027ddb0 superseded.
 # 2026-05-01 TB-7 Atom 1.5 — ProposalTelemetry CAS object writer (NEW file). Per ARCHITECT_RULING D5 + charter §4.5; precondition for chain-derived golden_path_token_count. 8-field schema (agent_id / prompt_context_hash / proposal_artifact_cid / candidate_tactic / token_counts / tool_calls / branch_id / parent_tx). All pub symbols carry /// TRACE_MATRIX FC1-N14 backlinks per R-022.
 "src/runtime/proposal_telemetry.rs" = "91bb53784dfb7d9e7643f7e88d672c12c460dce30e38635b6826ccb3b0edcf7a"  # rehashed by TB-7.7 D4 (2026-05-01): added optional `verification_result_cid: Option<Cid>` field (additive; #[serde(default)] preserves backward compat). NEW with_verification_result(cid) builder pattern for OMEGA-accept paths. Updated schema_validity test from 8 → 9 fields. (Pre-D4 commit had D1+D2: payload bytes → CAS + parent_tx wire.)
 # 2026-05-01 TB-7 Atom 5 — chain_derived_run_facts.rs (NEW file; renamed from chain_derived_pput.rs per ARCHITECT_RULING D4). Computes bit-exact structural fact set from L4 + L4.E + CAS alone. Per charter §4.4 fields: solved / verified / tx_count / proposal_count / golden_path_token_count / gp_payload / gp_path / gp_proof_file / tactic_diversity / tool_dist / failed_branch_count. Time-sensitive fields excluded. 3 unit tests U-A5.a..c.
diff --git a/src/runtime/agent_keypairs.rs b/src/runtime/agent_keypairs.rs
index 5203c2d..f428f15 100644
--- a/src/runtime/agent_keypairs.rs
+++ b/src/runtime/agent_keypairs.rs
@@ -320,11 +320,14 @@ impl AgentKeypairRegistry {
                 path: manifest_path,
             });
         }
-        // Parse the existing manifest just to confirm it round-trips.
-        // The actual signing keys come from the durable keystore — the
-        // manifest is the public-side replay sidecar.
+        // Parse the existing manifest — the public side of the agent
+        // registry. Every agent listed here MUST have a secret in the
+        // durable keystore; otherwise the resumed registry would
+        // silently lose a signing capability and the tape's
+        // agent_registry replay input would diverge from the
+        // post-resume in-memory state.
         let manifest_bytes = std::fs::read(&manifest_path).map_err(AgentKeypairError::Io)?;
-        let _parsed: AgentPubkeyManifest = serde_json::from_slice(&manifest_bytes)
+        let parsed: AgentPubkeyManifest = serde_json::from_slice(&manifest_bytes)
             .map_err(|e| AgentKeypairError::Serde(format!("agent_pubkeys.json: {e}")))?;
         let (secrets_map, _fresh) =
             crate::runtime::agent_keystore::load_or_empty(durable_keystore_path, &password)
@@ -333,6 +336,46 @@ impl AgentKeypairRegistry {
         for (agent_id_raw, seed) in secrets_map {
             keypairs.insert(AgentId(agent_id_raw), AgentKeypair::from_secret_bytes(seed));
         }
+
+        // TB-G G1.1 R2 closure (Codex G2 R1.5 Q1+Q8 CHALLENGE): cross-check
+        // every agent in the manifest MUST have a corresponding secret in
+        // the durable keystore AND the derived pubkey MUST match the
+        // manifest pubkey verbatim. Catches:
+        // (a) keystore was wiped while manifest survived (registry/keystore
+        //     drift),
+        // (b) keystore covers different agents (wrong keystore path / wrong
+        //     password),
+        // (c) manifest was tampered (manifest pubkey != derived pubkey).
+        // Fail-closed in all three cases — silent partial resume would
+        // violate FC2 §3.2 "agent_registry is a replay input" because the
+        // in-memory registry would no longer reproduce the on-disk
+        // manifest's binding.
+        for (agent_id_raw, manifest_pubkey_hex) in &parsed.agents {
+            let agent_id = AgentId(agent_id_raw.clone());
+            let keypair = keypairs.get(&agent_id).ok_or_else(|| {
+                AgentKeypairError::ResumeKeystoreInconsistent {
+                    agent_id: agent_id_raw.clone(),
+                    reason: format!(
+                        "agent_pubkeys.json lists agent_id={agent_id_raw:?} but the \
+                         durable keystore at {durable_keystore_path:?} has no \
+                         corresponding secret — keystore was wiped, password is \
+                         wrong, or the runtime_repo / keystore are from different runs"
+                    ),
+                }
+            })?;
+            let derived_pubkey_hex = keypair.public_key().to_hex();
+            if &derived_pubkey_hex != manifest_pubkey_hex {
+                return Err(AgentKeypairError::ResumeKeystoreInconsistent {
+                    agent_id: agent_id_raw.clone(),
+                    reason: format!(
+                        "manifest pubkey {manifest_pubkey_hex:?} does NOT match keystore-\
+                         derived pubkey {derived_pubkey_hex:?} — possible manifest \
+                         tampering or split-brain keystore"
+                    ),
+                });
+            }
+        }
+
         Ok(Self {
             keypairs,
             manifest_path,
@@ -485,6 +528,14 @@ pub enum AgentKeypairError {
     /// "resume-was-intended, manifest absent" — the latter is an invariant
     /// violation worth panicking on rather than silently reinitializing.
     ManifestAbsentInResume { path: PathBuf },
+    /// TB-G G1.1 R2 (Codex G2 R1.5 Q1+Q8 CHALLENGE closure 2026-05-11):
+    /// the on-disk `agent_pubkeys.json` references `agent_id` but the
+    /// durable keystore either has no secret for it or has a secret that
+    /// produces a different public key. Either way the resumed registry
+    /// can't faithfully reproduce the manifest's signing capabilities, so
+    /// FC2 §3.2 "agent_registry is a replay input" would silently
+    /// degrade. Fail-closed.
+    ResumeKeystoreInconsistent { agent_id: String, reason: String },
     Verify(String),
 }
 
@@ -506,6 +557,13 @@ impl fmt::Display for AgentKeypairError {
                      manifest (FC2 §3.2 mandates agent_registry as a replay input)"
                 )
             }
+            Self::ResumeKeystoreInconsistent { agent_id, reason } => {
+                write!(
+                    f,
+                    "resume mode: agent_pubkeys.json / durable keystore inconsistency \
+                     for agent_id={agent_id:?}: {reason}"
+                )
+            }
             Self::Verify(e) => write!(f, "agent signature verify: {e}"),
         }
     }
diff --git a/tests/constitution_g1_resume.rs b/tests/constitution_g1_resume.rs
index 3eda019..a202ded 100644
--- a/tests/constitution_g1_resume.rs
+++ b/tests/constitution_g1_resume.rs
@@ -28,6 +28,7 @@ use turingosv4::bus::{BusConfig, TuringBus};
 use turingosv4::economy::money::MicroCoin;
 use turingosv4::kernel::Kernel;
 use turingosv4::runtime::adapter::{genesis_with_balances, make_synthetic_task_open};
+use turingosv4::runtime::agent_keypairs::{AgentKeypairError, AgentKeypairRegistry};
 use turingosv4::runtime::{
     build_chaintape_sequencer, build_chaintape_sequencer_with_initial_q, BootstrapError,
     RuntimeChaintapeConfig,
@@ -407,3 +408,92 @@ async fn sg_g1_5_pinned_pubkeys_preserved_across_resume() {
 
     bundle_r.shutdown().await.expect("resume shutdown");
 }
+
+// ── SG-G1.6 (R2 closure; Codex Q2 CHALLENGE) ────────────────────────────────
+//
+// `resume_existing_durable` fails closed with `ManifestAbsentInResume`
+// when invoked on a runtime_repo where `agent_pubkeys.json` doesn't
+// exist. Binary-layer invariant: env=1 + manifest absent must NOT
+// silently degrade to fresh init. Mechanism per
+// `feedback_norm_needs_mechanism`.
+#[test]
+fn sg_g1_6_resume_existing_durable_fails_closed_when_manifest_absent() {
+    let tmp = TempDir::new().expect("tempdir");
+    let runtime_repo = tmp.path().join("runtime_repo");
+    std::fs::create_dir_all(&runtime_repo).expect("mkdir");
+    // No agent_pubkeys.json written.
+    let keystore = tmp.path().join("keystore.enc");
+    let pwd = secrecy::SecretString::new("test-password".to_string().into());
+    let result = AgentKeypairRegistry::resume_existing_durable(&runtime_repo, &keystore, pwd);
+    match result {
+        Err(AgentKeypairError::ManifestAbsentInResume { path }) => {
+            assert_eq!(
+                path,
+                runtime_repo.join("agent_pubkeys.json"),
+                "SG-G1.6: ManifestAbsentInResume must echo the expected manifest path"
+            );
+        }
+        Err(other) => panic!(
+            "SG-G1.6: expected ManifestAbsentInResume; got {other:?}"
+        ),
+        Ok(_) => panic!(
+            "SG-G1.6: resume_existing_durable on missing manifest MUST fail-closed \
+             (silent fall-through would degrade FC2 §3.2 agent_registry replay input)"
+        ),
+    }
+}
+
+// ── SG-G1.7 (R2 closure; Codex Q1+Q8 CHALLENGE) ─────────────────────────────
+//
+// `resume_existing_durable` fails closed with `ResumeKeystoreInconsistent`
+// when the manifest references an agent_id that has no corresponding
+// secret in the durable keystore. Catches: empty keystore + populated
+// manifest, wrong-password keystore reading as empty, keystore wiped
+// while manifest survived.
+#[test]
+fn sg_g1_7_resume_existing_durable_fails_closed_on_keystore_manifest_drift() {
+    use std::collections::BTreeMap;
+    let tmp = TempDir::new().expect("tempdir");
+    let runtime_repo = tmp.path().join("runtime_repo");
+    std::fs::create_dir_all(&runtime_repo).expect("mkdir");
+    // Write a manifest claiming an agent that is NOT in the durable keystore.
+    let mut agents = BTreeMap::new();
+    agents.insert(
+        "Agent_phantom".to_string(),
+        // 32-byte all-zeros pubkey hex placeholder — won't match any real key
+        "00".repeat(32),
+    );
+    let manifest = serde_json::json!({
+        "agents": agents,
+    });
+    std::fs::write(
+        runtime_repo.join("agent_pubkeys.json"),
+        serde_json::to_vec(&manifest).expect("serialize manifest"),
+    )
+    .expect("write manifest");
+
+    let keystore = tmp.path().join("keystore.enc");
+    // Empty keystore: never written, so `load_or_empty` returns empty.
+    let pwd = secrecy::SecretString::new("test-password".to_string().into());
+    let result = AgentKeypairRegistry::resume_existing_durable(&runtime_repo, &keystore, pwd);
+    match result {
+        Err(AgentKeypairError::ResumeKeystoreInconsistent { agent_id, reason }) => {
+            assert_eq!(
+                agent_id, "Agent_phantom",
+                "SG-G1.7: error must name the inconsistent agent_id"
+            );
+            assert!(
+                reason.contains("no corresponding secret"),
+                "SG-G1.7: reason should describe missing-secret case; got {reason:?}"
+            );
+        }
+        Err(other) => panic!(
+            "SG-G1.7: expected ResumeKeystoreInconsistent; got {other:?}"
+        ),
+        Ok(_) => panic!(
+            "SG-G1.7: resume_existing_durable with manifest agent but no keystore secret \
+             MUST fail-closed — silent boot would lose signing capability and violate \
+             FC2 §3.2 agent_registry replay determinism"
+        ),
+    }
+}

exec
/bin/bash -lc "nl -ba src/runtime/agent_keypairs.rs | sed -n '530,575p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   530	    ManifestAbsentInResume { path: PathBuf },
   531	    /// TB-G G1.1 R2 (Codex G2 R1.5 Q1+Q8 CHALLENGE closure 2026-05-11):
   532	    /// the on-disk `agent_pubkeys.json` references `agent_id` but the
   533	    /// durable keystore either has no secret for it or has a secret that
   534	    /// produces a different public key. Either way the resumed registry
   535	    /// can't faithfully reproduce the manifest's signing capabilities, so
   536	    /// FC2 §3.2 "agent_registry is a replay input" would silently
   537	    /// degrade. Fail-closed.
   538	    ResumeKeystoreInconsistent { agent_id: String, reason: String },
   539	    Verify(String),
   540	}
   541	
   542	impl fmt::Display for AgentKeypairError {
   543	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
   544	        match self {
   545	            Self::Io(e) => write!(f, "io: {e}"),
   546	            Self::Entropy(e) => write!(f, "getrandom entropy: {e}"),
   547	            Self::Serde(e) => write!(f, "serde: {e}"),
   548	            Self::InvalidFormat(s) => write!(f, "invalid format: {s}"),
   549	            Self::ManifestAlreadyExists { path } => {
   550	                write!(f, "agent_pubkeys.json already exists at {path:?}")
   551	            }
   552	            Self::ManifestAbsentInResume { path } => {
   553	                write!(
   554	                    f,
   555	                    "resume mode: agent_pubkeys.json missing at {path:?}; \
   556	                     cannot resume the agent registry without a persisted \
   557	                     manifest (FC2 §3.2 mandates agent_registry as a replay input)"
   558	                )
   559	            }
   560	            Self::ResumeKeystoreInconsistent { agent_id, reason } => {
   561	                write!(
   562	                    f,
   563	                    "resume mode: agent_pubkeys.json / durable keystore inconsistency \
   564	                     for agent_id={agent_id:?}: {reason}"
   565	                )
   566	            }
   567	            Self::Verify(e) => write!(f, "agent signature verify: {e}"),
   568	        }
   569	    }
   570	}
   571	
   572	impl std::error::Error for AgentKeypairError {}
   573	
   574	impl From<std::io::Error> for AgentKeypairError {
   575	    fn from(e: std::io::Error) -> Self {

exec
/bin/bash -lc "rg -n \"pubkey.*mismatch|mismatch.*pubkey|does NOT match|split-brain|tamper|ResumeKeystoreInconsistent|no corresponding secret|SG-G1\\.\" tests src -g '*.rs'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
tests/constitution_audit_tamper_3_of_3.rs:1://! Constitution gate — tamper-detection 3/3 invariant for `audit_tape_tamper`.
tests/constitution_audit_tamper_3_of_3.rs:7://! the in-binary tamper primitives stopped detecting L4 corruption after
tests/constitution_audit_tamper_3_of_3.rs:14://!     reachable blob → tamper hit the orphan, audit never reads it,
tests/constitution_audit_tamper_3_of_3.rs:20://! Fix: tamper primitives moved to library `runtime::audit_tamper` with:
tests/constitution_audit_tamper_3_of_3.rs:38:use turingosv4::runtime::audit_tamper::{
tests/constitution_audit_tamper_3_of_3.rs:62:    // Create reachable blob (target of tamper). Use non-compressible
tests/constitution_audit_tamper_3_of_3.rs:75:    let sig = Signature::new("audit-tamper-gate", "gate@local", &time).expect("sig");
tests/constitution_audit_tamper_3_of_3.rs:123:/// Bare repos use `objects/` directly (no `.git/` prefix). The tamper
tests/constitution_audit_tamper_3_of_3.rs:191:    // Snapshot pre-tamper state of orphan + chain blobs.
tests/constitution_audit_tamper_3_of_3.rs:201:        "tamper detail must reference the L4-reachable blob OID {chain_blob_hex}; got: {detail}"
tests/constitution_audit_tamper_3_of_3.rs:205:        "tamper detail must NOT reference the orphan blob OID {any_largest_hex}; got: {detail}"
tests/constitution_audit_tamper_3_of_3.rs:213:        "L4-reachable chain blob must be tampered (was {} bytes)", chain_pre.len()
tests/constitution_audit_tamper_3_of_3.rs:222:/// `refs/chaintape/*` must still tamper successfully via the alias entry in
tests/constitution_audit_tamper_3_of_3.rs:232:        .expect("alias-only repo (pre-A3) must still allow tampering via refs/transitions/main");
tests/constitution_audit_tamper_3_of_3.rs:267:/// `refs/transitions/main` pointing at the same OID; a tamper that touches
tests/constitution_audit_tamper_3_of_3.rs:282:    // Snapshot pre-tamper ref values.
tests/constitution_audit_tamper_3_of_3.rs:318:/// tampered (single-ref count = 1).
tests/constitution_audit_tamper_3_of_3.rs:325:        .expect("alias-only repo (pre-A3) must still allow ref-truncation tamper");
tests/constitution_audit_tamper_3_of_3.rs:391:    let detail = flip_largest_cas_object(&cas).expect("CAS tamper must succeed");
tests/constitution_audit_tamper_3_of_3.rs:418:         Adding a new canonical ref name (e.g. Stage D+) requires also updating tamper \
tests/constitution_audit_tamper_3_of_3.rs:448:         them would mis-target the L4-side tamper primitive at L4.E content."
tests/constitution_audit_tamper_3_of_3.rs:479:         use L4_REFS for L4-side tampering). Cross-contamination would cause \
tests/tb_6_l4e_jsonl_persistence.rs:111:fn t_r4_tampering_with_jsonl_line_fails_verify_chain_on_reopen() {
tests/tb_6_l4e_jsonl_persistence.rs:127:    let tampered = lines.join("\n") + "\n";
tests/tb_6_l4e_jsonl_persistence.rs:133:    f.write_all(tampered.as_bytes()).expect("write tampered");
tests/tb_6_l4e_jsonl_persistence.rs:137:    // Reopen MUST fail verify_chain (because the tampered line's hash no
tests/tb_6_l4e_jsonl_persistence.rs:142:            assert_eq!(at, 0, "tampered first line breaks at index 0");
tests/tb_16_halt_triggers.rs:40:    // must exist and have the right layer. The actual halt-on-tamper is
tests/tb_16_halt_triggers.rs:41:    // exercised by audit_tape_tamper (Atom 3) over a constructed tape.
tests/tb_16_halt_triggers.rs:74:    // parent_ledger / fold mismatch. The audit_tape_tamper binary
tests/tb_16_halt_triggers.rs:75:    // (Atom 3) exercises this on real tampered bytes.
tests/tb_18r_cas_reload_split_brain.rs:1://! TB-18R R3.fix Integration Test — CasStore split-brain reload.
tests/tb_18r_cas_reload_split_brain.rs:41:    let payload = b"r3fix-split-brain-payload";
tests/tb_18r_cas_reload_split_brain.rs:140:    // sequencer_cas was opened — split-brain condition replicated.
tests/tb_18r_cas_reload_split_brain.rs:192:    // Companion to the LeanFail recovery test: same split-brain pattern,
tests/constitution_g1_resume.rs:3://! gates SG-G1.1..SG-G1.5.
tests/constitution_g1_resume.rs:15://! repos — SG-G1.4 back-compat regression gate).
tests/constitution_g1_resume.rs:23://! byte-equality; G1.1 layers SG-G1.3 balances-byte-equal + SG-G1.5
tests/constitution_g1_resume.rs:48:// ── SG-G1.1 ─────────────────────────────────────────────────────────────────
tests/constitution_g1_resume.rs:61:        .expect("resume=true on empty repo bootstraps fresh (G1.1 SG-G1.1)");
tests/constitution_g1_resume.rs:77:        "SG-G1.1: state_root_t must match between resume=true/empty and resume=false/empty"
tests/constitution_g1_resume.rs:81:        "SG-G1.1: ledger_root_t must match"
tests/constitution_g1_resume.rs:85:        "SG-G1.1: economic_state_t must be byte-equal across branches"
tests/constitution_g1_resume.rs:89:// ── SG-G1.2 ─────────────────────────────────────────────────────────────────
tests/constitution_g1_resume.rs:99:// fully proves the SG-G1.2 constitutional invariant
tests/constitution_g1_resume.rs:138:        "SG-G1.2: Sequencer.next_logical_t must equal chain_length on resume \
tests/constitution_g1_resume.rs:173:        "SG-G1.2: chain length must advance from 1 → 2 after one post-resume commit"
tests/constitution_g1_resume.rs:177:// ── SG-G1.3 ─────────────────────────────────────────────────────────────────
tests/constitution_g1_resume.rs:238:        "SG-G1.3: alice balance must match between forward and resumed run"
tests/constitution_g1_resume.rs:255:        "SG-G1.3: bob balance must match"
tests/constitution_g1_resume.rs:259:        "SG-G1.3: full balances_t map must be byte-equal across forward / resumed runs"
tests/constitution_g1_resume.rs:266:        "SG-G1.3: state_root_t must be byte-equal between forward and resumed run"
tests/constitution_g1_resume.rs:270:// ── SG-G1.4 ─────────────────────────────────────────────────────────────────
tests/constitution_g1_resume.rs:301:                "SG-G1.4: NonEmptyRuntimeRepo must echo the rejected runtime_repo_path"
tests/constitution_g1_resume.rs:305:                "SG-G1.4: existing_head must be set when fail-closing"
tests/constitution_g1_resume.rs:309:            "SG-G1.4: expected BootstrapError::NonEmptyRuntimeRepo, got {other:?}"
tests/constitution_g1_resume.rs:312:            "SG-G1.4: resume=false on non-empty repo must NOT bootstrap successfully \
tests/constitution_g1_resume.rs:326:// ── SG-G1.5 ─────────────────────────────────────────────────────────────────
tests/constitution_g1_resume.rs:377:        "SG-G1.5: manifest must gain at least one entry on resume (had {} after resume)",
tests/constitution_g1_resume.rs:386:        "SG-G1.5: original (epoch={original_epoch}, pubkey={original_pubkey_hex}) MUST still be \
tests/constitution_g1_resume.rs:394:        "SG-G1.5: top-level manifest epoch must advance on resume (was {original_epoch}, \
tests/constitution_g1_resume.rs:404:        "SG-G1.5: a NEW (epoch={new_epoch}) entry with a distinct pubkey must be appended \
tests/constitution_g1_resume.rs:412:// ── SG-G1.6 (R2 closure; Codex Q2 CHALLENGE) ────────────────────────────────
tests/constitution_g1_resume.rs:433:                "SG-G1.6: ManifestAbsentInResume must echo the expected manifest path"
tests/constitution_g1_resume.rs:437:            "SG-G1.6: expected ManifestAbsentInResume; got {other:?}"
tests/constitution_g1_resume.rs:440:            "SG-G1.6: resume_existing_durable on missing manifest MUST fail-closed \
tests/constitution_g1_resume.rs:446:// ── SG-G1.7 (R2 closure; Codex Q1+Q8 CHALLENGE) ─────────────────────────────
tests/constitution_g1_resume.rs:448:// `resume_existing_durable` fails closed with `ResumeKeystoreInconsistent`
tests/constitution_g1_resume.rs:480:        Err(AgentKeypairError::ResumeKeystoreInconsistent { agent_id, reason }) => {
tests/constitution_g1_resume.rs:483:                "SG-G1.7: error must name the inconsistent agent_id"
tests/constitution_g1_resume.rs:486:                reason.contains("no corresponding secret"),
tests/constitution_g1_resume.rs:487:                "SG-G1.7: reason should describe missing-secret case; got {reason:?}"
tests/constitution_g1_resume.rs:491:            "SG-G1.7: expected ResumeKeystoreInconsistent; got {other:?}"
tests/constitution_g1_resume.rs:494:            "SG-G1.7: resume_existing_durable with manifest agent but no keystore secret \
tests/system_keypair_verify_correctness.rs:34:    let tampered = CanonicalMessage::RejectedAttemptSummary(RejectedAttemptSummary::new(
tests/system_keypair_verify_correctness.rs:41:        &signature, &tampered, epoch, &pinned
tests/tb_14_chaintape_smoke.rs:16://! `LedgerEntry` chain + replay-verifiable + tampering-detectable)
tests/tb_16_audit_tape_binary.rs:1://! TB-16 Atom 3 — `audit_tape` + `audit_tape_tamper` binary smoke test.
tests/tb_16_audit_tape_binary.rs:8://! - audit_tape_tamper detects 3/3 corruptions (BLOCK verdict on each
tests/tb_16_audit_tape_binary.rs:9://!   tampered copy).
tests/tb_16_audit_tape_binary.rs:65:fn audit_tape_tamper_binary_help_succeeds() {
tests/tb_16_audit_tape_binary.rs:66:    let bin = target_bin("audit_tape_tamper");
tests/tb_16_audit_tape_binary.rs:70:        .expect("audit_tape_tamper --help");
tests/tb_16_audit_tape_binary.rs:73:        stderr.contains("audit_tape_tamper") && stderr.contains("USAGE"),
tests/tb_16_audit_tape_binary.rs:74:        "audit_tape_tamper help malformed: {stderr}"
tests/tb_18r_chain_derived_facts_exact_accounting.rs:206:    // sidecar fresh, so split-brain is not a concern here per R3.fix
tests/constitution_fc1_runtime_loop.rs:208:/// FC1-INV6 — No fake accepted nodes. A tampered WorkTx whose canonical
tests/constitution_fc1_runtime_loop.rs:211:/// tb_18r_audit_lean_stderr_tamper_detected.rs covers this.
tests/constitution_fc1_runtime_loop.rs:214:    // The audit_tape sampler test must exist (tampered Lean stderr).
tests/constitution_fc1_runtime_loop.rs:215:    let audit_lean_tamper =
tests/constitution_fc1_runtime_loop.rs:216:        "tests/tb_18r_audit_lean_stderr_tamper_detected.rs";
tests/constitution_fc1_runtime_loop.rs:218:        std::path::Path::new(audit_lean_tamper).exists(),
tests/constitution_fc1_runtime_loop.rs:219:        "FC1-INV6 violation: {audit_lean_tamper} missing — tamper detection \
tests/constitution_fc1_runtime_loop.rs:223:    // The audit_sampler test must exist (tampered AttemptTelemetry payload).
tests/constitution_fc1_runtime_loop.rs:227:        "FC1-INV6 violation: {audit_sampler} missing — tamper detection \
tests/tb_6_agent_audit_trail.rs:15://! - I91c: chain-tampering detection on reload.
tests/constitution_fc3_inv1_capsule_integrity_regen.rs:147:/// If counts diverge, the capsule has been tampered or the runtime emit
tests/constitution_fc3_inv1_capsule_integrity_regen.rs:170:/// drifts from the AT-walk counts, the capsule has been tampered or the
tests/tb_1_acceptance.rs:41://!   - L4 / L4.E split as data structures (hash chains, projections, tamper
tests/tb_1_acceptance.rs:160:    let mut tampered: Vec<turingosv4::economy::ledger::AcceptedEntry> =
tests/tb_1_acceptance.rs:162:    tampered.last_mut().unwrap().resulting_state_root = Hash([0xFF; 32]);
tests/tb_1_acceptance.rs:163:    let bytes = serde_json::to_vec(&tampered).unwrap();
tests/tb_1_acceptance.rs:294:    l.tamper_remove_entry(2);
tests/tb_1_acceptance.rs:328:    l4e.tamper_remove_record(1);
tests/constitution_l4e_body_integrity.rs:4://! `tests/constitution_audit_tamper_3_of_3.rs::l4_refs_is_strict_subset_of_chain_refs_excluding_l4e`
tests/constitution_l4e_body_integrity.rs:6://! bodies, so tampering an L4.E blob is silent at audit-time").
tests/constitution_l4e_body_integrity.rs:9://! a constitutionally-undetectable tamper class is a constitutional
tests/constitution_l4e_body_integrity.rs:22://! attestation" — tampering reachable blobs or rewriting the ref was
tests/constitution_l4e_body_integrity.rs:36://! tamper test snapshots the source evidence to a tempdir before mutating.
tests/constitution_l4e_body_integrity.rs:47:use turingosv4::runtime::audit_tamper::flip_largest_reachable_l4e_blob;
tests/constitution_l4e_body_integrity.rs:107:/// Untampered M0 P01 → assertion #51 PASSes. This is the baseline that
tests/constitution_l4e_body_integrity.rs:108:/// every tamper variant degrades from. If this fails, the assertion has
tests/constitution_l4e_body_integrity.rs:111:fn assert_51_pass_on_untampered_m0_p01() {
tests/constitution_l4e_body_integrity.rs:113:    let tape = load_tape(&inputs).expect("load_tape on untampered M0 P01");
tests/constitution_l4e_body_integrity.rs:121:        "assertion #51 must PASS on untampered M0 P01 evidence; got {:?} detail={:?}",
tests/constitution_l4e_body_integrity.rs:131:/// This is the constitutional core: pre-this-gate, this tamper was the
tests/constitution_l4e_body_integrity.rs:138:    eprintln!("tamper detail: {detail}");
tests/constitution_l4e_body_integrity.rs:141:        "load_tape must succeed on a tape with tampered L4.E git-side blob \
tests/constitution_l4e_body_integrity.rs:148:        "assertion #51 MUST HALT on L4.E blob tampering (the canonical \
tests/constitution_l4e_body_integrity.rs:156:            || d.contains("body tampering") || d.contains("HashMismatch"),
tests/constitution_l4e_body_integrity.rs:164:/// tampering even when the underlying blobs are intact.
tests/constitution_l4e_body_integrity.rs:198:/// pre-Stage-A3 evidence shape and must NOT be mistaken for tampering.
tests/constitution_l4e_body_integrity.rs:236:        .expect("untampered record must parse + verify");
tests/constitution_l4e_body_integrity.rs:247:fn parse_and_verify_helper_rejects_tampered_field() {
tests/constitution_l4e_body_integrity.rs:254:    let tampered = first_line.replacen("tb6-smoke-sponsor", "tb6-smoke-TAMPER", 1);
tests/constitution_l4e_body_integrity.rs:255:    assert_ne!(tampered, first_line, "tamper must change the line");
tests/constitution_l4e_body_integrity.rs:256:    let err = parse_and_verify_jsonl_record_bytes(tampered.as_bytes())
tests/constitution_l4e_body_integrity.rs:257:        .expect_err("tampered record must NOT parse-and-verify");
tests/tb_6_verify_chaintape.rs:12://! - I90c: tamper detection — corrupt the on-disk pinned_pubkey hex →
tests/tb_6_verify_chaintape.rs:121:async fn i90c_tampered_pinned_pubkey_breaks_signature_verification() {
tests/tb_6_verify_chaintape.rs:135:    // Sanity: untampered chain passes.
tests/tb_6_verify_chaintape.rs:137:        .expect("pre-tamper verify");
tests/tb_6_verify_chaintape.rs:163:        .expect("write tampered manifest");
tests/tb_6_verify_chaintape.rs:167:            .expect("verify with tampered pubkey");
tests/tb_6_verify_chaintape.rs:170:        "tampered pubkey must break signature verification (got {:?})",
tests/tb_6_verify_chaintape.rs:177:// TB-7.6 / Codex audit cc7b3dd action #6 — disk-level tamper battery
tests/tb_6_verify_chaintape.rs:180:// I90d/e/f extend the I90c pattern (pinned-pubkey tampering) to cover
tests/tb_6_verify_chaintape.rs:181:// the remaining disk-level tamper surfaces flagged by the TB-6 Codex
tests/tb_6_verify_chaintape.rs:184:// - I90d — tamper a CAS index sidecar (`.turingos_cas_index.jsonl`) →
tests/tb_6_verify_chaintape.rs:187:// - I90e — tamper a rejections.jsonl row → RejectionEvidenceWriter chain
tests/tb_6_verify_chaintape.rs:192://   rejections" and "tampered rejections".
tests/tb_6_verify_chaintape.rs:194:// TRACE_MATRIX FC1-N14: Class 2 production wire-up tamper hardening.
tests/tb_6_verify_chaintape.rs:201:async fn i90d_tampered_cas_index_breaks_verify_chaintape() {
tests/tb_6_verify_chaintape.rs:215:    // Sanity: pre-tamper verify passes.
tests/tb_6_verify_chaintape.rs:217:        .expect("pre-tamper verify");
tests/tb_6_verify_chaintape.rs:228:        // Empty index — ensure there's at least one record before we tamper.
tests/tb_6_verify_chaintape.rs:231:    lines[0].push_str(r#"{"cid":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"backend_oid_hex":"deadbeef","object_type":"Generic","creator":"tamper","created_at_logical_t":99,"schema_id":null,"size_bytes":0}"#);
tests/tb_6_verify_chaintape.rs:232:    std::fs::write(&cas_index, lines.join("\n") + "\n").expect("write tamper");
tests/tb_6_verify_chaintape.rs:238:        "tampered CAS index must break verify_chaintape at CAS-open time; got Ok({result:?})"
tests/tb_6_verify_chaintape.rs:246:async fn i90e_tampered_l4e_row_breaks_chain_open() {
tests/tb_6_verify_chaintape.rs:269:        .expect("pre-tamper verify");
tests/tb_6_verify_chaintape.rs:270:    assert!(pre.l4e_entries >= 1, "expected ≥1 L4.E row to tamper");
tests/tb_6_verify_chaintape.rs:283:    std::fs::write(&rejections_path, lines.join("\n") + "\n").expect("write tamper");
tests/tb_6_verify_chaintape.rs:289:        "tampered L4.E row must break verify_chaintape at L4.E-open time; got Ok({result:?})"
tests/tb_6_verify_chaintape.rs:294:/// the same as tampering: an absent file means "no rejections" and
tests/tb_6_verify_chaintape.rs:297:/// "honest absent" and "tampered present" so future refactors don't
tests/tb_6_verify_chaintape.rs:301:async fn i90f_absent_l4e_is_legitimate_empty_chain_not_tamper() {
src/economy/ledger.rs:51:/// All seven fields enter the hash; tampering any single field breaks
src/economy/ledger.rs:223:    /// the entry hash itself) was tampered.
src/economy/ledger.rs:302:    /// `verify_chain(0, len)` runs BEFORE `reconstruct_state` so any tamper of
src/economy/ledger.rs:329:    /// tests to simulate adversarial row deletion. The `tamper_` prefix and
src/economy/ledger.rs:335:    pub fn tamper_remove_entry(&mut self, idx: usize) {
src/economy/ledger.rs:364:// Inline correctness tests (round-trip + tamper detection on every field).
src/economy/ledger.rs:479:    fn load_from_path_rejects_prev_hash_tamper() {
src/economy/ledger.rs:481:        // verify_chain. A prev_hash-only tamper is the canonical case where
src/economy/ledger.rs:484:        // touch prev_hash. With the fail-closed default, a load on a tampered
src/economy/ledger.rs:494:        let mut tampered: Vec<AcceptedEntry> = serde_json::from_slice(&raw).unwrap();
src/economy/ledger.rs:498:        tampered[1].prev_hash = Hash([0xAB; 32]);
src/economy/ledger.rs:499:        std::fs::write(tmp.path(), serde_json::to_vec(&tampered).unwrap()).unwrap();
src/economy/ledger.rs:504:            "load_from_path must reject prev_hash tamper at index 1; got {:?}",
src/economy/ledger.rs:510:    fn load_from_path_rejects_entry_hash_tamper() {
src/economy/ledger.rs:511:        // TB-1 P0-4: tampering the entry `hash` field directly. Same rationale
src/economy/ledger.rs:521:        let mut tampered: Vec<AcceptedEntry> = serde_json::from_slice(&raw).unwrap();
src/economy/ledger.rs:522:        tampered[0].hash = Hash([0xCD; 32]);
src/economy/ledger.rs:523:        std::fs::write(tmp.path(), serde_json::to_vec(&tampered).unwrap()).unwrap();
src/economy/ledger.rs:528:            "load_from_path must reject entry-hash tamper at index 0; got {:?}",
src/economy/ledger.rs:546:        let mut tampered: Vec<AcceptedEntry> = serde_json::from_slice(&raw).unwrap();
src/economy/ledger.rs:548:        tampered.remove(1);
src/economy/ledger.rs:549:        std::fs::write(tmp.path(), serde_json::to_vec(&tampered).unwrap()).unwrap();
src/bin/audit_tape_tamper.rs:1://! TB-16 Atom 3 — `audit_tape_tamper` CLI (architect §7.7 + design §6.2 H).
src/bin/audit_tape_tamper.rs:15://! is untouched. Emits `tamper_report.json` summarizing the 3 attempts.
src/bin/audit_tape_tamper.rs:19://! genesis (Layer G Skipped on tampered + untampered copies).
src/bin/audit_tape_tamper.rs:24://!   audit_tape_tamper \
src/bin/audit_tape_tamper.rs:34://!     --tamper-dir    <work-dir> \
src/bin/audit_tape_tamper.rs:35://!     --out           <tamper_report.json>
src/bin/audit_tape_tamper.rs:42://! TRACE_MATRIX FC1-N35 (audit_tape_tamper binary; design §6.2 #36-#38).
src/bin/audit_tape_tamper.rs:45://! the destructive zlib-decode-failure tamper primitives originally lived in
src/bin/audit_tape_tamper.rs:49://! moved to library `turingosv4::runtime::audit_tamper` 2026-05-10 session
src/bin/audit_tape_tamper.rs:62:use turingosv4::runtime::audit_tamper::{
src/bin/audit_tape_tamper.rs:77:    tamper_dir: PathBuf,
src/bin/audit_tape_tamper.rs:94:        "--tamper-dir",
src/bin/audit_tape_tamper.rs:118:            "--tamper-dir" => "--tamper-dir",
src/bin/audit_tape_tamper.rs:132:    let tamper_dir = take("--tamper-dir")?;
src/bin/audit_tape_tamper.rs:153:        tamper_dir,
src/bin/audit_tape_tamper.rs:175:    "audit_tape_tamper — TB-16 Atom 3 tamper-detection harness\n\
src/bin/audit_tape_tamper.rs:178:       audit_tape_tamper --runtime-repo <p> --cas-dir <p> --agent-pubkeys <p>\n  \
src/bin/audit_tape_tamper.rs:181:                         [--alignment-dir <p>] --tamper-dir <p> --out <p>\n\
src/bin/audit_tape_tamper.rs:190:       0  all 3 corruptions detected (BLOCK on each tampered copy)\n  \
src/bin/audit_tape_tamper.rs:220:    let dir = args.tamper_dir.join(label);
src/bin/audit_tape_tamper.rs:258:// Tamper apply primitives moved to library `turingosv4::runtime::audit_tamper`
src/bin/audit_tape_tamper.rs:271:fn run_tamper(
src/bin/audit_tape_tamper.rs:280:                schema_version: "v1/audit_tape_tamper".into(),
src/bin/audit_tape_tamper.rs:290:    // pre-tamper baseline check. The forked tape must verify
src/bin/audit_tape_tamper.rs:292:    // tamper BLOCK could be the SAME pre-existing halt (e.g.
src/bin/audit_tape_tamper.rs:294:    let pre_tamper = match run_audit(args, &runtime, &cas) {
src/bin/audit_tape_tamper.rs:298:                schema_version: "v1/audit_tape_tamper".into(),
src/bin/audit_tape_tamper.rs:302:                    "pre-tamper baseline audit failed (cannot validate \
src/bin/audit_tape_tamper.rs:309:    if pre_tamper.verdict != "PROCEED" {
src/bin/audit_tape_tamper.rs:311:            schema_version: "v1/audit_tape_tamper".into(),
src/bin/audit_tape_tamper.rs:315:                "pre-tamper baseline verdict={} (not PROCEED); cannot \
src/bin/audit_tape_tamper.rs:316:                 prove tamper-fence efficacy on a tape that already \
src/bin/audit_tape_tamper.rs:318:                pre_tamper.verdict
src/bin/audit_tape_tamper.rs:320:            verdict: Some(pre_tamper),
src/bin/audit_tape_tamper.rs:328:                schema_version: "v1/audit_tape_tamper".into(),
src/bin/audit_tape_tamper.rs:339:        // pre-tamper was PROCEED AND post-tamper is BLOCK. The pre-tamper
src/bin/audit_tape_tamper.rs:343:            // Audit refused to load the tape at all post-tamper; that
src/bin/audit_tape_tamper.rs:345:            // corruption — pre-tamper succeeded so this is a tamper-
src/bin/audit_tape_tamper.rs:347:            eprintln!("audit_tape_tamper: load itself failed for `{label}` post-tamper → counted as detected ({e})");
src/bin/audit_tape_tamper.rs:352:        schema_version: "v1/audit_tape_tamper".into(),
src/bin/audit_tape_tamper.rs:365:            eprintln!("audit_tape_tamper: {e}\n\n{}", help_text());
src/bin/audit_tape_tamper.rs:369:    if let Err(e) = std::fs::create_dir_all(&args.tamper_dir) {
src/bin/audit_tape_tamper.rs:370:        eprintln!("audit_tape_tamper: mkdir tamper-dir: {e}");
src/bin/audit_tape_tamper.rs:374:    let r1 = run_tamper("flip_l4_byte", &args, |runtime, _cas| {
src/bin/audit_tape_tamper.rs:377:    let r2 = run_tamper("flip_cas_byte", &args, |_runtime, cas| {
src/bin/audit_tape_tamper.rs:380:    let r3 = run_tamper("truncate_l4_ref", &args, |runtime, _cas| {
src/bin/audit_tape_tamper.rs:387:        "schema_version": "v1/audit_tape_tamper",
src/bin/audit_tape_tamper.rs:388:        "tamper_results": [r1, r2, r3],
src/bin/audit_tape_tamper.rs:395:        eprintln!("audit_tape_tamper: write {:?} failed: {e}", args.out);
src/bin/audit_tape_tamper.rs:400:        "audit_tape_tamper: detected {}/3 (out={:?})",
src/ledger.rs:1:// Tier 0: Append-only tape with tamper detection
src/ledger.rs:179:/// A single ledger event with hash-chain tamper detection.
src/ledger.rs:215:/// Append-only event ledger with tamper detection via hash chain.
src/ledger.rs:253:    /// Verify the entire hash chain. Returns Ok(()) if tamper-free.
src/ledger.rs:468:    fn test_ledger_tamper_detection() {
src/ledger.rs:474:        ledger.events.as_mut_slice()[0].hash = "tampered".to_string();
src/bottom_white/cas/store.rs:192:    /// 2026-05-06 surfaced this split-brain: sequencer's R3 admission helper
src/bottom_white/cas/store.rs:327:    /// object whose back half is zeroed by a tamper harness) cannot hang the
src/bottom_white/ledger/rejection_evidence.rs:394:    /// (covers row deletion, field tampering, and reordering).
src/bottom_white/ledger/rejection_evidence.rs:468:    ///   reject tampering at load time.
src/bottom_white/ledger/rejection_evidence.rs:475:    /// tampering with any line breaks the chain at that line.
src/bottom_white/ledger/rejection_evidence.rs:516:        // Validate chain integrity on load — tampering with any record
src/bottom_white/ledger/rejection_evidence.rs:615:    /// tampered, or if a row was deleted (the surviving row's `prev_hash`
src/bottom_white/ledger/rejection_evidence.rs:660:    /// `#[doc(hidden)]` + `tamper_` prefix flags any production use as a
src/bottom_white/ledger/rejection_evidence.rs:664:    pub fn tamper_remove_record(&mut self, idx: usize) {
src/bottom_white/ledger/rejection_evidence.rs:674:/// any field-level body tampering inside the bytes.
src/bottom_white/ledger/rejection_evidence.rs:823:    fn verify_detects_field_tamper() {
src/bottom_white/ledger/rejection_evidence.rs:847:        w.records[0].public_summary = Some("tampered".into());
src/bin/audit_dashboard.rs:952:    // tamper). Reaching this point with Some(_) means valid.
src/state/sequencer.rs:3774:    /// next commit (SG-G1.2 binding;
src/bottom_white/ledger/transition_ledger.rs:18://!   defense); new test asserts replay rejects parent_ledger_root tamper.
src/bottom_white/ledger/transition_ledger.rs:23://! - K7: +2 conformance tests (parent_ledger_root tamper, digest exclusion).
src/bottom_white/ledger/transition_ledger.rs:1170:    // 5. ChainOnly replay rejects parent_state_root tamper
src/bottom_white/ledger/transition_ledger.rs:1172:    fn replay_rejects_parent_state_tamper() {
src/bottom_white/ledger/transition_ledger.rs:1180:    // 6. K2 NEW: ChainOnly replay rejects parent_ledger_root tamper (transplant defense)
src/bottom_white/ledger/transition_ledger.rs:1182:    fn replay_rejects_parent_ledger_tamper() {
src/bottom_white/ledger/transition_ledger.rs:1192:    // 7. ChainOnly replay rejects ledger_root tamper
src/bottom_white/ledger/transition_ledger.rs:1194:    fn replay_rejects_ledger_root_tamper() {
src/bottom_white/ledger/transition_ledger.rs:1211:        let mut e_tamper = e_clean.clone();
src/bottom_white/ledger/transition_ledger.rs:1212:        e_tamper.resulting_ledger_root = h(0xff);
src/bottom_white/ledger/transition_ledger.rs:1213:        let digest_after_root_tamper = e_tamper.to_signing_payload().canonical_digest();
src/bottom_white/ledger/transition_ledger.rs:1215:            digest_clean, digest_after_root_tamper,
src/bottom_white/ledger/transition_ledger.rs:1220:        let mut e_tamper2 = e_clean.clone();
src/bottom_white/ledger/transition_ledger.rs:1221:        e_tamper2.system_signature = SystemSignature::from_bytes([0xffu8; 64]);
src/bottom_white/ledger/transition_ledger.rs:1222:        let digest_after_sig_tamper = e_tamper2.to_signing_payload().canonical_digest();
src/bottom_white/ledger/transition_ledger.rs:1223:        assert_eq!(digest_clean, digest_after_sig_tamper);
src/bottom_white/ledger/transition_ledger.rs:1272:        // Verify (tamper parent_ledger_root) — K2 transplant defense
src/bottom_white/ledger/transition_ledger.rs:1273:        let mut payload_tamper = payload.clone();
src/bottom_white/ledger/transition_ledger.rs:1274:        payload_tamper.parent_ledger_root = h(0xff);
src/bottom_white/ledger/transition_ledger.rs:1275:        let digest_tamper = payload_tamper.canonical_digest();
src/bottom_white/ledger/transition_ledger.rs:1276:        let msg_tamper = CanonicalMessage::LedgerEntrySigning(digest_tamper.0);
src/bottom_white/ledger/transition_ledger.rs:1278:            !verify_system_signature(&sig, &msg_tamper, epoch, &pinned),
src/bottom_white/ledger/transition_ledger.rs:1544:    /// 16. system_signature_verifies_via_canonical_message — tampering the
src/bottom_white/ledger/transition_ledger.rs:1632:        // tampered envelope so signature still verifies.
src/bottom_white/ledger/transition_ledger.rs:1633:        let tampered_signing = LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:1644:        let tampered_digest = tampered_signing.canonical_digest();
src/bottom_white/ledger/transition_ledger.rs:1645:        let tampered_sig =
src/bottom_white/ledger/transition_ledger.rs:1646:            transition_ledger_emitter::sign_ledger_entry(&kp, tampered_digest.0).expect("sign");
src/bottom_white/ledger/transition_ledger.rs:1648:        entry.system_signature = tampered_sig;
src/bottom_white/ledger/transition_ledger.rs:1649:        // Recompute resulting_ledger_root with the tampered signing digest so
src/bottom_white/ledger/transition_ledger.rs:1651:        entry.resulting_ledger_root = append(&Hash::ZERO, &tampered_digest);
src/boot.rs:411:    /// with the given hex hash. Used by both tamper and match tests.
src/boot.rs:433:    fn verify_trust_root_detects_tamper_in_tempdir() {
src/boot.rs:434:        // Manifest claims a zero hash; on-disk content "tampered" hashes to
src/boot.rs:437:        write_single_entry_repo(&tmp, "tampered", &"0".repeat(64));
src/boot.rs:438:        match verify_trust_root(&tmp).expect_err("tamper must be detected") {
src/boot.rs:469:    /// A8e13 fix Q1 conformance: child manifest tamper is detected even
src/boot.rs:476:    fn verify_trust_root_detects_child_manifest_tamper() {
src/boot.rs:481:        // tampered relative to what the parent manifest claims.
src/boot.rs:484:        fs::write(tmp.join("subdir/child.txt"), "tampered_content").unwrap();
src/boot.rs:503:        match verify_trust_root(&tmp).expect_err("child tamper must be detected") {
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:1://! TB-18R R5 — Layer H tamper detection on AttemptTelemetry /
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:7://! audit_tape_tamper assertions 36-38, extended to AttemptTelemetry /
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:23:/// SG-18R.7 random_attempt_payload_tamper_detected: any in-CAS byte
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:26:fn attempt_telemetry_tamper_detected_via_cid_mismatch() {
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:36:        TxId("att-tamper".into()),
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:37:        "tb18r-r5-tamper".into(),
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:38:        "task-tamper".into(),
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:51:    assert!(cas.get(&cid).is_ok(), "untampered get must succeed");
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:57:    let mut tampered_bytes = cid.0;
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:58:    tampered_bytes[0] ^= 0xff;
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:59:    let tampered = Cid(tampered_bytes);
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:61:        cas.get(&tampered).is_err(),
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:62:        "tampered Cid must NOT resolve in CAS index"
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:66:/// SG-18R.7 random_lean_stderr_tamper_detected: LeanResult CAS objects
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:67:/// detect content tampering via the same Cid mismatch invariant.
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:69:fn lean_result_tamper_detected_via_cid_mismatch() {
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:76:        attempt_id: TxId("att-lr-tamper".into()),
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:87:    assert!(cas.get(&cid).is_ok(), "untampered get must succeed");
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:89:    let mut tampered_bytes = cid.0;
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:90:    tampered_bytes[0] ^= 0xff;
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:91:    let tampered = Cid(tampered_bytes);
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:93:        cas.get(&tampered).is_err(),
tests/tb_18r_audit_lean_stderr_tamper_detected.rs:94:        "tampered LeanResult Cid must NOT resolve"
src/runtime/audit_tamper.rs:1://! Library API for the audit-tape tamper-detection harness.
src/runtime/audit_tamper.rs:4://! `BLOCK` on the tampered copy:
src/runtime/audit_tamper.rs:21://! `src/bin/audit_tape_tamper.rs`; they were authored 2026-05-04 (commit
src/runtime/audit_tamper.rs:23://! (`refs/chaintape/{l4,l4e,cas}`) per CR-A3-HEAD-T-C2.5+6, but the tamper
src/runtime/audit_tamper.rs:33://! Per `feedback_no_workarounds_strict_constitution` ("我不要凑活"): tamper
src/runtime/audit_tamper.rs:36://! exercised by `tests/constitution_audit_tamper_3_of_3.rs` so future drift
src/runtime/audit_tamper.rs:46:/// TRACE_MATRIX FC1-N35 (audit_tape_tamper coverage; architect §B.9.3
src/runtime/audit_tamper.rs:66:/// TRACE_MATRIX FC1-N35 + FC1-N34 (audit_tape_tamper coverage; architect
src/runtime/audit_tamper.rs:80:/// TRACE_MATRIX FC1-N35 (audit_tape_tamper coverage; architect §B.9.3):
src/runtime/audit_tamper.rs:220:    std::fs::write(victim, bytes).map_err(|e| format!("write tampered: {e}"))?;
src/runtime/audit_tamper.rs:224:/// TRACE_MATRIX FC1-N35 (audit_tape_tamper Atom 3 / TB-16 Atom 7 closure;
src/runtime/audit_tamper.rs:255:/// TRACE_MATRIX FC1-N35 + FC1-N34 (audit_tape_tamper L4.E-side coverage;
src/runtime/audit_tamper.rs:283:/// TRACE_MATRIX FC1-N35 (audit_tape_tamper Atom 3 / TB-16 Atom 7 closure;
src/runtime/audit_tamper.rs:327:/// TRACE_MATRIX FC1-N35 + FC2-INV1 (audit_tape_tamper Atom 3 ref-truncation
src/runtime/audit_tamper.rs:339:    let mut tampered: Vec<String> = Vec::new();
src/runtime/audit_tamper.rs:357:        tampered.push((*ref_name).to_string());
src/runtime/audit_tamper.rs:359:    if tampered.is_empty() {
src/runtime/audit_tamper.rs:367:        tampered.len(),
src/runtime/audit_tamper.rs:368:        tampered.join(", ")
src/runtime/mod.rs:62:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 Atom 2; architect §7.5 + design §6.2): 38-assertion audit-from-tape battery. Pure-fn library over on-disk artifacts (runtime_repo + cas_dir + manifests + constitution + markov pointer); NO live process state. Drives `audit_tape` + `audit_tape_tamper` binaries; verdict.json wire format per design §6.3.
src/runtime/mod.rs:95:/// drift): `audit_tamper` — three corruption primitives used by the
src/runtime/mod.rs:96:/// `audit_tape_tamper` binary harness. Constitutional Justification:
src/runtime/mod.rs:97:/// architect §B.9.3 mandates "prove no fake accepted" via 3/3 tamper
src/runtime/mod.rs:101:/// API is exercised by `tests/constitution_audit_tamper_3_of_3.rs` so
src/runtime/mod.rs:103:pub mod audit_tamper;
src/runtime/mod.rs:451:    // resume=true degrades to fresh genesis (SG-G1.1 byte-equality).
src/runtime/mod.rs:459:        // SG-G1.4: existing TB-6 fail-closed gate preserved when resume
src/runtime/mod.rs:542:    // verifies the chain and rejects any tamper.
src/runtime/mod.rs:575:    // `len + 1` invariant (packet §3 SG-G1.2).
src/runtime/audit_assertions.rs:29://! H tamper). H is exercised by the separate `audit_tape_tamper`
src/runtime/audit_assertions.rs:31://! guarantees so tampering is detectable when present.
src/runtime/audit_assertions.rs:42://!   `audit_tape_tamper` binary; the `assert_36/37/38` stubs in this
src/runtime/audit_assertions.rs:43://!   module emit `Skipped` results — actual tamper detection lives in
src/runtime/audit_assertions.rs:44://!   `bin/audit_tape_tamper.rs`).
src/runtime/audit_assertions.rs:121:    H, // tamper detection (separate binary)
src/runtime/audit_assertions.rs:1721:    // (Layer B #6) already proves the recorded class is not tampered.
src/runtime/audit_assertions.rs:2139:            // Best-effort decode; if it fails, skip — tampered CAS
src/runtime/audit_assertions.rs:2809:// Layer H — tamper detection (3 assertions; exercised via separate binary)
src/runtime/audit_assertions.rs:2812:/// TRACE_MATRIX FC1-N35 (TB-16 audit_tape_tamper binary; Atom 7 R1
src/runtime/audit_assertions.rs:2814:/// FC1-N34 — they are exercised by the audit_tape_tamper binary, not
src/runtime/audit_assertions.rs:2816:pub fn assert_36_tamper_l4_flip_detected() -> AssertionResult {
src/runtime/audit_assertions.rs:2819:        "tamper_l4_flip_detected",
src/runtime/audit_assertions.rs:2821:        "exercised by audit_tape_tamper binary (Atom 3; FC1-N35)".into(),
src/runtime/audit_assertions.rs:2825:/// TRACE_MATRIX FC1-N35 (TB-16 audit_tape_tamper binary; Atom 7 R1
src/runtime/audit_assertions.rs:2827:pub fn assert_37_tamper_cas_flip_detected() -> AssertionResult {
src/runtime/audit_assertions.rs:2830:        "tamper_cas_flip_detected",
src/runtime/audit_assertions.rs:2832:        "exercised by audit_tape_tamper binary (Atom 3; FC1-N35)".into(),
src/runtime/audit_assertions.rs:2836:/// TRACE_MATRIX FC1-N35 (TB-16 audit_tape_tamper binary; Atom 7 R1
src/runtime/audit_assertions.rs:2838:pub fn assert_38_tamper_l4_remove_detected() -> AssertionResult {
src/runtime/audit_assertions.rs:2841:        "tamper_l4_remove_detected",
src/runtime/audit_assertions.rs:2843:        "exercised by audit_tape_tamper binary (Atom 3; FC1-N35)".into(),
src/runtime/audit_assertions.rs:2848:/// §1.4 SG-18R.7): tamper detection on a randomly-sampled
src/runtime/audit_assertions.rs:2850:/// `audit_tape_tamper` binary per existing Layer H precedent
src/runtime/audit_assertions.rs:2852:pub fn assert_47_random_attempt_payload_tamper_detected() -> AssertionResult {
src/runtime/audit_assertions.rs:2855:        "random_attempt_payload_tamper_detected",
src/runtime/audit_assertions.rs:2857:        "exercised by audit_tape_tamper binary (TB-18R R5; FC2-N34)".into(),
src/runtime/audit_assertions.rs:2862:/// §1.4 SG-18R.7): tamper detection on a randomly-sampled LeanResult
src/runtime/audit_assertions.rs:2863:/// stderr blob. Exercised by `audit_tape_tamper` binary.
src/runtime/audit_assertions.rs:2864:pub fn assert_48_random_lean_stderr_tamper_detected() -> AssertionResult {
src/runtime/audit_assertions.rs:2867:        "random_lean_stderr_tamper_detected",
src/runtime/audit_assertions.rs:2869:        "exercised by audit_tape_tamper binary (TB-18R R5; FC2-N34)".into(),
src/runtime/audit_assertions.rs:2877:/// `audit_tape_tamper` on P05 detected only 2/3 corruptions — the
src/runtime/audit_assertions.rs:2881:/// SOMETHING, but does NOT verify `hash(blob) == cid`. So a tampered blob
src/runtime/audit_assertions.rs:2906:                         (suggests tampered storage; integrity check could not proceed)",
src/runtime/audit_assertions.rs:2951:/// in `tests/constitution_audit_tamper_3_of_3.rs::l4_refs_is_strict_subset_of_chain_refs_excluding_l4e`).
src/runtime/audit_assertions.rs:2968:/// effort attestation" — tampering a blob reachable from
src/runtime/audit_assertions.rs:2971:/// the related TB-16-era tamper-primitive drift; the strict-constitution
src/runtime/audit_assertions.rs:2992:///     tampering: any field flip → embedded `hash` won't recompute), and
src/runtime/audit_assertions.rs:2994:///     `hash` (this catches ref-target tampering or git-side substitution
src/runtime/audit_assertions.rs:2998:///   - byte tampering of any L4.E git-side loose blob → either git2 zlib
src/runtime/audit_assertions.rs:3001:///   - tampering of `refs/chaintape/l4e` to point at a different OID →
src/runtime/audit_assertions.rs:3108:                         on refs/chaintape/l4e; tampering of a blob reachable \
src/runtime/audit_assertions.rs:3199:                         (likely tampering: zlib decode failure on the \
src/runtime/audit_assertions.rs:3214:                         + self-verify: {e} (body tampering caught by embedded \
src/runtime/audit_assertions.rs:3322:    // audit_tape_tamper binary)
src/runtime/audit_assertions.rs:3323:    r.push(assert_36_tamper_l4_flip_detected());
src/runtime/audit_assertions.rs:3324:    r.push(assert_37_tamper_cas_flip_detected());
src/runtime/audit_assertions.rs:3325:    r.push(assert_38_tamper_l4_remove_detected());
src/runtime/audit_assertions.rs:3326:    r.push(assert_47_random_attempt_payload_tamper_detected());
src/runtime/audit_assertions.rs:3327:    r.push(assert_48_random_lean_stderr_tamper_detected());
src/runtime/chain_derived_run_facts.rs:1016:             This may indicate tampered evidence OR a runtime bug. \
src/runtime/agent_keypairs.rs:348:        // (c) manifest was tampered (manifest pubkey != derived pubkey).
src/runtime/agent_keypairs.rs:356:                AgentKeypairError::ResumeKeystoreInconsistent {
src/runtime/agent_keypairs.rs:368:                return Err(AgentKeypairError::ResumeKeystoreInconsistent {
src/runtime/agent_keypairs.rs:371:                        "manifest pubkey {manifest_pubkey_hex:?} does NOT match keystore-\
src/runtime/agent_keypairs.rs:373:                         tampering or split-brain keystore"
src/runtime/agent_keypairs.rs:538:    ResumeKeystoreInconsistent { agent_id: String, reason: String },
src/runtime/agent_keypairs.rs:560:            Self::ResumeKeystoreInconsistent { agent_id, reason } => {
src/runtime/agent_audit_trail.rs:129:    /// rows so tampering is detectable. NOT a system signature — the audit
src/runtime/agent_audit_trail.rs:190:    /// `logical_t` (chronology), so tampering with the row's logical_t is
src/runtime/agent_audit_trail.rs:213:    /// Audit-row chain integrity broken at the given index — tampering or
src/runtime/agent_audit_trail.rs:301:/// `hash` so tampering with any line is detectable on reload (mirrors
src/runtime/agent_audit_trail.rs:363:        // row's logical_t, so tampering with logical_t at the row level is
src/runtime/verify.rs:32://! L4.E `prev_hash → hash` chain on load — tamper any byte of any line and
src/runtime/verify.rs:145:    /// signature mismatch (tampering, key drift, unknown agent_id).
src/runtime/verify.rs:224:///    — this internally calls `verify_chain()` and rejects tampering.

exec
/bin/bash -lc "rg -n \"mod tests|agent_keypairs|ResumeKeystoreInconsistent|resume_existing_durable|generate_or_load_durable|agent_pubkeys\" src/runtime/agent_keypairs.rs tests -g '*.rs'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
tests/constitution_tb_n3_a3_emit.rs:38:use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
tests/constitution_g1_resume.rs:31:use turingosv4::runtime::agent_keypairs::{AgentKeypairError, AgentKeypairRegistry};
tests/constitution_g1_resume.rs:414:// `resume_existing_durable` fails closed with `ManifestAbsentInResume`
tests/constitution_g1_resume.rs:415:// when invoked on a runtime_repo where `agent_pubkeys.json` doesn't
tests/constitution_g1_resume.rs:420:fn sg_g1_6_resume_existing_durable_fails_closed_when_manifest_absent() {
tests/constitution_g1_resume.rs:424:    // No agent_pubkeys.json written.
tests/constitution_g1_resume.rs:427:    let result = AgentKeypairRegistry::resume_existing_durable(&runtime_repo, &keystore, pwd);
tests/constitution_g1_resume.rs:432:                runtime_repo.join("agent_pubkeys.json"),
tests/constitution_g1_resume.rs:440:            "SG-G1.6: resume_existing_durable on missing manifest MUST fail-closed \
tests/constitution_g1_resume.rs:448:// `resume_existing_durable` fails closed with `ResumeKeystoreInconsistent`
tests/constitution_g1_resume.rs:454:fn sg_g1_7_resume_existing_durable_fails_closed_on_keystore_manifest_drift() {
tests/constitution_g1_resume.rs:470:        runtime_repo.join("agent_pubkeys.json"),
tests/constitution_g1_resume.rs:478:    let result = AgentKeypairRegistry::resume_existing_durable(&runtime_repo, &keystore, pwd);
tests/constitution_g1_resume.rs:480:        Err(AgentKeypairError::ResumeKeystoreInconsistent { agent_id, reason }) => {
tests/constitution_g1_resume.rs:491:            "SG-G1.7: expected ResumeKeystoreInconsistent; got {other:?}"
tests/constitution_g1_resume.rs:494:            "SG-G1.7: resume_existing_durable with manifest agent but no keystore secret \
tests/constitution_tb_n3_invest_routing.rs:22:use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
tests/constitution_economy_gate.rs:278:    // Additionally: agent_pubkeys gate prevents agent-submitted tx from
tests/constitution_economy_gate.rs:281:        seq_src.contains("agent_pubkeys") || seq_src.contains("AgentKeypairRegistry"),
tests/constitution_economy_gate.rs:283:         consult agent_pubkeys / AgentKeypairRegistry — system identity \
tests/tb_7_atom6_chain_backed_smoke.rs:54:use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
tests/tb_7_atom6_chain_backed_smoke.rs:115:        AgentKeypairRegistry::open(&cfg.runtime_repo_path).expect("open agent_keypairs");
tests/tb_7_atom6_chain_backed_smoke.rs:204:        "Gate 4: every WorkTx + VerifyTx signature must verify against agent_pubkeys.json — {report:?}"
tests/tb_7_atom6_chain_backed_smoke.rs:261:        let agent_pubkeys_src = cfg.runtime_repo_path.join("agent_pubkeys.json");
tests/tb_7_atom6_chain_backed_smoke.rs:262:        if agent_pubkeys_src.exists() {
tests/tb_7_atom6_chain_backed_smoke.rs:263:            let _ = std::fs::copy(&agent_pubkeys_src, evidence_dir.join("agent_pubkeys.json"));
tests/tb_7_atom6_chain_backed_smoke.rs:281:                 - agent_pubkeys.json: {agents} agents pinned\n\
tests/tb_7_atom6_chain_backed_smoke.rs:287:                 3. **Gate 4** (agent signatures): every WorkTx + VerifyTx signature verifies against agent_pubkeys.json on replay.\n\
tests/tb_14_chaintape_smoke.rs:62:use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
tests/tb_14_chaintape_smoke.rs:197:        .set_agent_pubkeys(Arc::new(reg.manifest()))
tests/tb_14_chaintape_smoke.rs:198:        .expect("set_agent_pubkeys must succeed once");
tests/tb_14_chaintape_smoke.rs:370:        let agent_pubkeys_src = cfg.runtime_repo_path.join("agent_pubkeys.json");
tests/tb_14_chaintape_smoke.rs:371:        if agent_pubkeys_src.exists() {
tests/tb_14_chaintape_smoke.rs:373:                &agent_pubkeys_src,
tests/tb_14_chaintape_smoke.rs:374:                evidence_dir.join("agent_pubkeys.json"),
tests/tb_16_audit_tape_binary.rs:94:        .arg(smoke.join("runtime_repo/agent_pubkeys.json"))
tests/tb_7r_parent_tx_conformance.rs:29:use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
tests/tb_18r_chain_derived_facts_exact_accounting.rs:17:use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
tests/tb_18r_drain_barrier_quiescence.rs:19:use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
tests/tb_7_authoritative_routing.rs:15://!   `<runtime_repo>/agent_pubkeys.json` (Atom 4 verify_chaintape will
tests/tb_7_authoritative_routing.rs:36:use turingosv4::runtime::agent_keypairs::{
tests/tb_7_authoritative_routing.rs:84:        .expect("open agent_keypairs registry on a fresh runtime repo (sibling to bundle)");
tests/tb_7_authoritative_routing.rs:102:        AgentPubkeyManifest::load(reg.manifest_path()).expect("load agent_pubkeys.json");
tests/tb_7_authoritative_routing.rs:128:    // AgentKeypairRegistry uses an `agent_pubkeys.json` distinct from
tests/tb_7_authoritative_routing.rs:133:        .expect("open agent_keypairs registry");
tests/tb_7_authoritative_routing.rs:197:        AgentKeypairRegistry::open(&cfg.runtime_repo_path).expect("open agent_keypairs");
tests/tb_7_authoritative_routing.rs:313:        AgentKeypairRegistry::open(&cfg.runtime_repo_path).expect("open agent_keypairs");
tests/tb_13_complete_set.rs:880:/// Codex round-2 VETO TB13-AUTH remediation: when `agent_pubkeys` is set on
tests/tb_13_complete_set.rs:891:    use turingosv4::runtime::agent_keypairs::{
tests/tb_13_complete_set.rs:907:        .set_agent_pubkeys(Arc::new(manifest))
tests/tb_13_complete_set.rs:908:        .expect("set_agent_pubkeys must succeed once");
tests/markov_pointer_de_canonicalize.rs:99:        .arg(smoke.join("runtime_repo/agent_pubkeys.json"))
tests/markov_pointer_de_canonicalize.rs:171:        .arg(smoke.join("runtime_repo/agent_pubkeys.json"))
tests/markov_pointer_de_canonicalize.rs:231:        .arg(smoke.join("runtime_repo/agent_pubkeys.json"))
tests/markov_pointer_de_canonicalize.rs:281:        .arg(smoke.join("runtime_repo/agent_pubkeys.json"))
tests/markov_pointer_de_canonicalize.rs:342:        .arg(smoke.join("runtime_repo/agent_pubkeys.json"))
tests/tb_14_canonical_masking_smoke.rs:54:use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
tests/tb_14_canonical_masking_smoke.rs:259:        .set_agent_pubkeys(Arc::new(h.keypairs.manifest()))
tests/tb_14_canonical_masking_smoke.rs:260:        .expect("set_agent_pubkeys");
tests/constitution_l4e_body_integrity.rs:95:        agent_pubkeys: dst_runtime.join("agent_pubkeys.json"),
tests/tb_13_chaintape_smoke.rs:22://!    `agent_pubkeys.json` to runtime_repo_path) + `set_agent_pubkeys`
tests/tb_13_chaintape_smoke.rs:79:use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
tests/tb_13_chaintape_smoke.rs:221:    // this writes <runtime_repo>/agent_pubkeys.json which verify_chaintape
tests/tb_13_chaintape_smoke.rs:229:        .set_agent_pubkeys(Arc::new(reg.manifest()))
tests/tb_13_chaintape_smoke.rs:230:        .expect("set_agent_pubkeys must succeed once");
tests/tb_13_chaintape_smoke.rs:443:        let agent_pubkeys_src = cfg.runtime_repo_path.join("agent_pubkeys.json");
tests/tb_13_chaintape_smoke.rs:444:        if agent_pubkeys_src.exists() {
tests/tb_13_chaintape_smoke.rs:446:                &agent_pubkeys_src,
tests/tb_13_chaintape_smoke.rs:447:                evidence_dir.join("agent_pubkeys.json"),
tests/tb_13_chaintape_smoke.rs:473:                 3. `verify_chaintape` reconstructs a `QState` from the persisted runtime_repo + cas + initial_q_state.json + agent_pubkeys.json + pinned_pubkeys.json whose `final_state_root_hex` matches the live `state_root_t`. Codex round-4 follow-up clarification: the state-root mutator hashes `domain || prev_root || canonical_tx`, NOT the full QState — so state-root equality on its own proves deterministic tx-chain replay (same initial state + same canonical-encoded txs + same pure dispatcher → same root); it does NOT directly assert byte-equal QState reconstruction.\n\
tests/constitution_fc2_boot.rs:214:    // canonical home: src/runtime/agent_keypairs.rs (per ship-history
tests/constitution_fc2_boot.rs:217:        "src/runtime/agent_keypairs.rs",
tests/constitution_fc2_boot.rs:231:    // Sequencer must consult agent_pubkeys for admission control.
tests/constitution_fc2_boot.rs:235:        seq_src.contains("agent_pubkeys") || seq_src.contains("AgentKeypairRegistry"),
tests/constitution_fc2_boot.rs:236:        "FC2-INV7 violation: sequencer.rs does not reference agent_pubkeys \
src/runtime/agent_keypairs.rs:17://! | Public manifest     | `pinned_pubkeys.json`        | `agent_pubkeys.json`              |
src/runtime/agent_keypairs.rs:180:/// evaluator boot via `generate_or_load_durable` recovers the same
src/runtime/agent_keypairs.rs:216:    /// `<runtime_repo>/agent_pubkeys.json`. Mirrors TB-6 fail-closed
src/runtime/agent_keypairs.rs:219:        let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
src/runtime/agent_keypairs.rs:241:    /// Per-run manifest at `<runtime_repo>/agent_pubkeys.json` is still written
src/runtime/agent_keypairs.rs:249:    pub fn generate_or_load_durable(
src/runtime/agent_keypairs.rs:254:        let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
src/runtime/agent_keypairs.rs:284:    /// loading the existing `agent_pubkeys.json` manifest INSTEAD of
src/runtime/agent_keypairs.rs:290:    /// G1.1 kernel-side covers) and `agent_pubkeys.json` (agent registry
src/runtime/agent_keypairs.rs:302:    ///   `generate_or_load_durable`), reconstruct the in-memory
src/runtime/agent_keypairs.rs:312:    pub fn resume_existing_durable(
src/runtime/agent_keypairs.rs:317:        let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
src/runtime/agent_keypairs.rs:331:            .map_err(|e| AgentKeypairError::Serde(format!("agent_pubkeys.json: {e}")))?;
src/runtime/agent_keypairs.rs:356:                AgentKeypairError::ResumeKeystoreInconsistent {
src/runtime/agent_keypairs.rs:359:                        "agent_pubkeys.json lists agent_id={agent_id_raw:?} but the \
src/runtime/agent_keypairs.rs:368:                return Err(AgentKeypairError::ResumeKeystoreInconsistent {
src/runtime/agent_keypairs.rs:467:/// TRACE_MATRIX FC1-N14: on-disk shape of `agent_pubkeys.json`.
src/runtime/agent_keypairs.rs:525:    /// "断点续作是本项目的核心"): `resume_existing_durable` was called but
src/runtime/agent_keypairs.rs:532:    /// the on-disk `agent_pubkeys.json` references `agent_id` but the
src/runtime/agent_keypairs.rs:538:    ResumeKeystoreInconsistent { agent_id: String, reason: String },
src/runtime/agent_keypairs.rs:550:                write!(f, "agent_pubkeys.json already exists at {path:?}")
src/runtime/agent_keypairs.rs:555:                    "resume mode: agent_pubkeys.json missing at {path:?}; \
src/runtime/agent_keypairs.rs:560:            Self::ResumeKeystoreInconsistent { agent_id, reason } => {
src/runtime/agent_keypairs.rs:563:                    "resume mode: agent_pubkeys.json / durable keystore inconsistency \
src/runtime/agent_keypairs.rs:583:mod tests {
src/runtime/agent_keypairs.rs:689:            AgentKeypairRegistry::generate_or_load_durable(repo.path(), &keystore_path, pwd.clone())
src/runtime/agent_keypairs.rs:711:            let mut reg_a = AgentKeypairRegistry::generate_or_load_durable(
src/runtime/agent_keypairs.rs:723:        let mut reg_b = AgentKeypairRegistry::generate_or_load_durable(
src/runtime/agent_keypairs.rs:752:        let mut reg_a = AgentKeypairRegistry::generate_or_load_durable(
src/runtime/agent_keypairs.rs:761:        let err = AgentKeypairRegistry::generate_or_load_durable(

exec
/bin/bash -lc "nl -ba src/runtime/agent_keypairs.rs | sed -n '580,790p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   580	// ── Tests ────────────────────────────────────────────────────────────────────
   581	
   582	#[cfg(test)]
   583	mod tests {
   584	    use super::*;
   585	    use tempfile::TempDir;
   586	
   587	    fn fresh_repo() -> TempDir {
   588	        TempDir::new().expect("tempdir")
   589	    }
   590	
   591	    fn fresh_digest(seed: u8) -> [u8; 32] {
   592	        let mut h = Sha256::new();
   593	        h.update([seed]);
   594	        h.finalize().into()
   595	    }
   596	
   597	    /// U-A1.a — generate produces a non-zero public key + working signature.
   598	    #[test]
   599	    fn generate_produces_signing_keypair() {
   600	        let kp = AgentKeypair::generate().expect("generate");
   601	        assert_ne!(*kp.public_key().as_bytes(), [0u8; AGENT_PUBLIC_LEN]);
   602	        let digest = fresh_digest(0);
   603	        let sig = kp.sign_digest(digest).expect("sign");
   604	        assert!(verify_agent_signature(&sig, &digest, &kp.public_key()).is_ok());
   605	    }
   606	
   607	    /// U-A1.b — registry persists manifest with the agent's pubkey after first sign.
   608	    #[test]
   609	    fn registry_persists_manifest_on_first_use() {
   610	        let repo = fresh_repo();
   611	        let mut reg = AgentKeypairRegistry::open(repo.path()).expect("open");
   612	        assert!(reg.manifest_path().exists());
   613	        let agent = AgentId("n1".into());
   614	        let _sig = reg.sign(&agent, fresh_digest(1)).expect("sign");
   615	        let loaded = AgentPubkeyManifest::load(reg.manifest_path()).expect("load");
   616	        assert!(loaded.get(&agent).is_some(), "n1 missing from manifest");
   617	    }
   618	
   619	    /// U-A1.c — same agent reuses cached keypair across calls; signatures verify
   620	    /// under the same pinned pubkey.
   621	    #[test]
   622	    fn same_agent_reuses_keypair_across_signs() {
   623	        let repo = fresh_repo();
   624	        let mut reg = AgentKeypairRegistry::open(repo.path()).expect("open");
   625	        let agent = AgentId("swarm_a".into());
   626	        let sig1 = reg.sign(&agent, fresh_digest(2)).expect("sign1");
   627	        let sig2 = reg.sign(&agent, fresh_digest(3)).expect("sign2");
   628	        let pubkey = reg
   629	            .manifest()
   630	            .get(&agent)
   631	            .expect("pubkey");
   632	        assert!(verify_agent_signature(&sig1, &fresh_digest(2), &pubkey).is_ok());
   633	        assert!(verify_agent_signature(&sig2, &fresh_digest(3), &pubkey).is_ok());
   634	    }
   635	
   636	    /// U-A1.d — manifest survives reload (load from disk == in-memory snapshot).
   637	    #[test]
   638	    fn manifest_round_trip() {
   639	        let repo = fresh_repo();
   640	        let mut reg = AgentKeypairRegistry::open(repo.path()).expect("open");
   641	        let a1 = AgentId("n1".into());
   642	        let a2 = AgentId("swarm_b".into());
   643	        let _ = reg.sign(&a1, fresh_digest(4)).expect("sign1");
   644	        let _ = reg.sign(&a2, fresh_digest(5)).expect("sign2");
   645	        let in_mem = reg.manifest();
   646	        let loaded = AgentPubkeyManifest::load(reg.manifest_path()).expect("load");
   647	        assert_eq!(in_mem, loaded);
   648	        // Both agents present, ordering deterministic (BTreeMap).
   649	        assert_eq!(loaded.agents.len(), 2);
   650	        assert!(loaded.get(&a1).is_some());
   651	        assert!(loaded.get(&a2).is_some());
   652	    }
   653	
   654	    /// U-A1.e — re-opening a runtime repo whose manifest already exists is
   655	    /// rejected (fail-closed; mirrors TB-6 non-empty-runtime-repo gate).
   656	    #[test]
   657	    fn registry_open_refuses_existing_manifest() {
   658	        let repo = fresh_repo();
   659	        let _reg = AgentKeypairRegistry::open(repo.path()).expect("first open");
   660	        let err = AgentKeypairRegistry::open(repo.path()).expect_err("second open");
   661	        match err {
   662	            AgentKeypairError::ManifestAlreadyExists { .. } => {}
   663	            other => panic!("expected ManifestAlreadyExists, got {other}"),
   664	        }
   665	    }
   666	
   667	    /// U-A1.f — wrong pubkey rejects valid signature (negative test).
   668	    #[test]
   669	    fn wrong_pubkey_rejects_signature() {
   670	        let kp1 = AgentKeypair::generate().expect("kp1");
   671	        let kp2 = AgentKeypair::generate().expect("kp2");
   672	        let digest = fresh_digest(6);
   673	        let sig = kp1.sign_digest(digest).expect("sign");
   674	        assert!(verify_agent_signature(&sig, &digest, &kp2.public_key()).is_err());
   675	    }
   676	
   677	    // ── TB-9 Atom 1 — durable cross-run identity tests ──────────────────────
   678	
   679	    /// U-TB9.a — fresh durable boot generates an empty registry; first sign
   680	    /// triggers keypair generation AND persists encrypted keystore on disk.
   681	    #[test]
   682	    fn durable_first_boot_persists_secret() {
   683	        let repo = fresh_repo();
   684	        let keystore_dir = fresh_repo();
   685	        let keystore_path = keystore_dir.path().join("agent_keystore.enc");
   686	        let pwd = secrecy::SecretString::new("tb9-durable-test".into());
   687	
   688	        let mut reg =
   689	            AgentKeypairRegistry::generate_or_load_durable(repo.path(), &keystore_path, pwd.clone())
   690	                .expect("first boot");
   691	        let agent = AgentId("n1".into());
   692	        let _sig = reg.sign(&agent, fresh_digest(11)).expect("sign");
   693	
   694	        assert!(keystore_path.exists(), "durable keystore not written");
   695	        let bytes = std::fs::read(&keystore_path).unwrap();
   696	        assert!(bytes.starts_with(b"TOS4AGTKEY1"), "magic mismatch");
   697	    }
   698	
   699	    /// U-TB9.b — second boot loads existing keystore; same agent_id produces
   700	    /// the same pubkey across the run boundary (cross-run identity).
   701	    #[test]
   702	    fn durable_second_boot_recovers_same_pubkey() {
   703	        let keystore_dir = fresh_repo();
   704	        let keystore_path = keystore_dir.path().join("agent_keystore.enc");
   705	        let pwd = secrecy::SecretString::new("tb9-durable-test".into());
   706	        let agent = AgentId("n1".into());
   707	
   708	        // Run A: generate + sign + record pubkey.
   709	        let pubkey_a = {
   710	            let repo_a = fresh_repo();
   711	            let mut reg_a = AgentKeypairRegistry::generate_or_load_durable(
   712	                repo_a.path(),
   713	                &keystore_path,
   714	                pwd.clone(),
   715	            )
   716	            .expect("run A boot");
   717	            let _ = reg_a.sign(&agent, fresh_digest(20)).expect("run A sign");
   718	            reg_a.manifest().get(&agent).expect("run A pubkey")
   719	        }; // reg_a drops here
   720	
   721	        // Run B: re-load + sign + verify pubkey is identical.
   722	        let repo_b = fresh_repo();
   723	        let mut reg_b = AgentKeypairRegistry::generate_or_load_durable(
   724	            repo_b.path(),
   725	            &keystore_path,
   726	            pwd.clone(),
   727	        )
   728	        .expect("run B boot");
   729	        let sig_b = reg_b.sign(&agent, fresh_digest(21)).expect("run B sign");
   730	        let pubkey_b = reg_b.manifest().get(&agent).expect("run B pubkey");
   731	
   732	        assert_eq!(
   733	            pubkey_a, pubkey_b,
   734	            "cross-run identity broken: pubkey changed across runs"
   735	        );
   736	        assert!(
   737	            verify_agent_signature(&sig_b, &fresh_digest(21), &pubkey_b).is_ok(),
   738	            "run B signature must verify under the durable pubkey"
   739	        );
   740	    }
   741	
   742	    /// U-TB9.c — wrong password on second boot rejects (no silent regenerate).
   743	    #[test]
   744	    fn durable_wrong_password_rejected() {
   745	        let keystore_dir = fresh_repo();
   746	        let keystore_path = keystore_dir.path().join("agent_keystore.enc");
   747	        let pwd_a = secrecy::SecretString::new("tb9-correct".into());
   748	        let pwd_b = secrecy::SecretString::new("tb9-wrong".into());
   749	        let agent = AgentId("n1".into());
   750	
   751	        let repo_a = fresh_repo();
   752	        let mut reg_a = AgentKeypairRegistry::generate_or_load_durable(
   753	            repo_a.path(),
   754	            &keystore_path,
   755	            pwd_a,
   756	        )
   757	        .expect("run A");
   758	        let _ = reg_a.sign(&agent, fresh_digest(30)).expect("sign");
   759	
   760	        let repo_b = fresh_repo();
   761	        let err = AgentKeypairRegistry::generate_or_load_durable(
   762	            repo_b.path(),
   763	            &keystore_path,
   764	            pwd_b,
   765	        )
   766	        .expect_err("wrong password must fail");
   767	        match err {
   768	            AgentKeypairError::Serde(msg) => assert!(
   769	                msg.contains("crypto") || msg.contains("authentication"),
   770	                "expected crypto authentication failure, got {msg}"
   771	            ),
   772	            other => panic!("expected Serde(crypto), got {other}"),
   773	        }
   774	    }
   775	}

codex
One issue is emerging: the implementation has a pubkey-mismatch branch, but the visible SG-G1.7 test exercises only the missing-secret branch. I’m running the narrow gate now so the “GREEN” part is grounded, then I’ll give the terse verdicts.
exec
/bin/bash -lc 'cargo test --test constitution_g1_resume' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc 'cargo test verify_trust_root_passes_on_intact_repo' in /home/zephryj/projects/turingosv4
 succeeded in 475ms:
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
warning: unused import: `KernelError`
 --> src/bus.rs:8:29
  |
8 | use crate::kernel::{Kernel, KernelError};
  |                             ^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `TapeError`
 --> src/bus.rs:9:54
  |
9 | use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
  |                                                      ^^^^^^^^^

warning: unused imports: `Deserialize` and `Serialize`
  --> src/bus.rs:13:13
   |
13 | use serde::{Deserialize, Serialize};
   |             ^^^^^^^^^^^  ^^^^^^^^^

warning: unused import: `ToolSignal`
 --> src/sdk/tools/search.rs:4:24
  |
4 | use crate::sdk::tool::{ToolSignal, TuringTool};
  |                        ^^^^^^^^^^

warning: unused import: `std::path::Path`
 --> src/sdk/tools/search.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `Deserialize`
 --> src/sdk/tools/librarian.rs:6:13
  |
6 | use serde::{Deserialize, Serialize};
  |             ^^^^^^^^^^^

warning: unused import: `Path`
 --> src/sdk/tools/librarian.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^

warning: unused imports: `MICRO_PER_COIN` and `MicroCoin`
  --> src/economy/monetary_invariant.rs:26:29
   |
26 | use crate::economy::money::{MicroCoin, MICRO_PER_COIN};
   |                             ^^^^^^^^^  ^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/cas/store.rs:26:12
   |
26 | use git2::{ObjectType as Git2ObjectType, Repository};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/ledger/transition_ledger.rs:36:12
   |
36 | use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `TxId`
  --> src/state/sequencer.rs:41:82
   |
41 | use crate::state::q_state::{AgentId, EscrowEntry, Hash, QState, TaskMarketEntry, TxId};
   |                                                                                  ^^^^

warning: unused imports: `AgentId` and `ShareSidePair`
  --> src/state/price_index.rs:23:29
   |
23 | use crate::state::q_state::{AgentId, ChallengeStatus, EconomicState, ShareSidePair};
   |                             ^^^^^^^                                  ^^^^^^^^^^^^^

warning: unused import: `NodePosition`
  --> src/state/price_index.rs:24:39
   |
24 | use crate::state::typed_tx::{EventId, NodePosition, PositionSide, ShareAmount};
   |                                       ^^^^^^^^^^^^

warning: unused import: `Sha256`
  --> src/runtime/agent_keypairs.rs:31:20
   |
31 | use sha2::{Digest, Sha256};
   |                    ^^^^^^

warning: unused import: `CapsulePrivacyPolicy`
  --> src/runtime/audit_assertions.rs:80:30
   |
80 | use crate::state::typed_tx::{CapsulePrivacyPolicy, TypedTx};
   |                              ^^^^^^^^^^^^^^^^^^^^

warning: unused import: `std::collections::BTreeMap`
  --> src/runtime/audit_views.rs:27:5
   |
27 | use std::collections::BTreeMap;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::state::price_index::RationalPrice`
  --> src/runtime/audit_views.rs:32:5
   |
32 | use crate::state::price_index::RationalPrice;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `LpShareAmount` and `ShareSidePair`
  --> src/runtime/audit_views.rs:34:29
   |
34 |     AgentId, EconomicState, LpShareAmount, PoolStatus, ShareSidePair,
   |                             ^^^^^^^^^^^^^              ^^^^^^^^^^^^^

warning: unused import: `ShareAmount`
  --> src/runtime/audit_views.rs:40:42
   |
40 |     EventId, PositionKind, PositionSide, ShareAmount,
   |                                          ^^^^^^^^^^^

warning: unused import: `Digest`
  --> src/runtime/agent_keypairs.rs:31:12
   |
31 | use sha2::{Digest, Sha256};
   |            ^^^^^^

warning: function `sign_rejected_attempt_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:581:19
    |
581 |     pub(crate) fn sign_rejected_attempt_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:592:19
    |
592 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: function `sign_epoch_rotation_proof` is never used
   --> src/bottom_white/ledger/system_keypair.rs:692:19
    |
692 |     pub(crate) fn sign_epoch_rotation_proof(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:703:19
    |
703 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: field `tx_id` is never read
   --> src/runtime/chain_derived_run_facts.rs:239:5
    |
238 | struct WorkTxAttempt {
    |        ------------- field in this struct
239 |     tx_id: TxId,
    |     ^^^^^
    |
    = note: `WorkTxAttempt` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: `turingosv4` (lib) generated 25 warnings (run `cargo fix --lib -p turingosv4` to apply 19 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running tests/constitution_g1_resume.rs (target/debug/deps/constitution_g1_resume-6e6abf7d2e25b11b)

running 7 tests
test sg_g1_1_resume_on_empty_repo_equals_legacy_genesis ... ok
test sg_g1_3_resume_balances_reconstruction_matches_forward_replay ... ok
test sg_g1_4_non_empty_runtime_repo_only_fires_when_resume_false ... ok
test sg_g1_6_resume_existing_durable_fails_closed_when_manifest_absent ... ok
test sg_g1_7_resume_existing_durable_fails_closed_on_keystore_manifest_drift ... ok
test sg_g1_2_resume_on_n_entry_chain_sets_next_logical_t_to_n ... ok
test sg_g1_5_pinned_pubkeys_preserved_across_resume ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s


 succeeded in 49560ms:
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on artifact directory
warning: unused import: `KernelError`
 --> src/bus.rs:8:29
  |
8 | use crate::kernel::{Kernel, KernelError};
  |                             ^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `TapeError`
 --> src/bus.rs:9:54
  |
9 | use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
  |                                                      ^^^^^^^^^

warning: unused imports: `Deserialize` and `Serialize`
  --> src/bus.rs:13:13
   |
13 | use serde::{Deserialize, Serialize};
   |             ^^^^^^^^^^^  ^^^^^^^^^

warning: unused import: `ToolSignal`
 --> src/sdk/tools/search.rs:4:24
  |
4 | use crate::sdk::tool::{ToolSignal, TuringTool};
  |                        ^^^^^^^^^^

warning: unused import: `std::path::Path`
 --> src/sdk/tools/search.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `Deserialize`
 --> src/sdk/tools/librarian.rs:6:13
  |
6 | use serde::{Deserialize, Serialize};
  |             ^^^^^^^^^^^

warning: unused import: `Path`
 --> src/sdk/tools/librarian.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^

warning: unused imports: `MICRO_PER_COIN` and `MicroCoin`
  --> src/economy/monetary_invariant.rs:26:29
   |
26 | use crate::economy::money::{MicroCoin, MICRO_PER_COIN};
   |                             ^^^^^^^^^  ^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/cas/store.rs:26:12
   |
26 | use git2::{ObjectType as Git2ObjectType, Repository};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/ledger/transition_ledger.rs:36:12
   |
36 | use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `TxId`
  --> src/state/sequencer.rs:41:82
   |
41 | use crate::state::q_state::{AgentId, EscrowEntry, Hash, QState, TaskMarketEntry, TxId};
   |                                                                                  ^^^^

warning: unused imports: `AgentId` and `ShareSidePair`
  --> src/state/price_index.rs:23:29
   |
23 | use crate::state::q_state::{AgentId, ChallengeStatus, EconomicState, ShareSidePair};
   |                             ^^^^^^^                                  ^^^^^^^^^^^^^

warning: unused import: `NodePosition`
  --> src/state/price_index.rs:24:39
   |
24 | use crate::state::typed_tx::{EventId, NodePosition, PositionSide, ShareAmount};
   |                                       ^^^^^^^^^^^^

warning: unused import: `Sha256`
  --> src/runtime/agent_keypairs.rs:31:20
   |
31 | use sha2::{Digest, Sha256};
   |                    ^^^^^^

warning: unused import: `CapsulePrivacyPolicy`
  --> src/runtime/audit_assertions.rs:80:30
   |
80 | use crate::state::typed_tx::{CapsulePrivacyPolicy, TypedTx};
   |                              ^^^^^^^^^^^^^^^^^^^^

warning: unused import: `std::collections::BTreeMap`
  --> src/runtime/audit_views.rs:27:5
   |
27 | use std::collections::BTreeMap;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::state::price_index::RationalPrice`
  --> src/runtime/audit_views.rs:32:5
   |
32 | use crate::state::price_index::RationalPrice;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `LpShareAmount` and `ShareSidePair`
  --> src/runtime/audit_views.rs:34:29
   |
34 |     AgentId, EconomicState, LpShareAmount, PoolStatus, ShareSidePair,
   |                             ^^^^^^^^^^^^^              ^^^^^^^^^^^^^

warning: unused import: `ShareAmount`
  --> src/runtime/audit_views.rs:40:42
   |
40 |     EventId, PositionKind, PositionSide, ShareAmount,
   |                                          ^^^^^^^^^^^

warning: unused import: `Digest`
  --> src/runtime/agent_keypairs.rs:31:12
   |
31 | use sha2::{Digest, Sha256};
   |            ^^^^^^

warning: function `sign_rejected_attempt_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:581:19
    |
581 |     pub(crate) fn sign_rejected_attempt_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:592:19
    |
592 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: function `sign_epoch_rotation_proof` is never used
   --> src/bottom_white/ledger/system_keypair.rs:692:19
    |
692 |     pub(crate) fn sign_epoch_rotation_proof(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:703:19
    |
703 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: field `tx_id` is never read
   --> src/runtime/chain_derived_run_facts.rs:239:5
    |
238 | struct WorkTxAttempt {
    |        ------------- field in this struct
239 |     tx_id: TxId,
    |     ^^^^^
    |
    = note: `WorkTxAttempt` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: `turingosv4` (lib) generated 25 warnings (run `cargo fix --lib -p turingosv4` to apply 19 suggestions)
   Compiling turingosv4 v0.1.0 (/home/zephryj/projects/turingosv4)
warning: function `ok` is never used
  --> tests/tb_16_halt_triggers.rs:16:4
   |
16 | fn ok(r: &AssertionResult, expected_layer: AssertionLayer) {
   |    ^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `Cid`
  --> tests/tb_18r_lean_result_cas_resolves.rs:15:45
   |
15 | use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
   |                                             ^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `turingosv4` (test "tb_16_halt_triggers") generated 1 warning
warning: field `repo_path` is never read
  --> tests/constitution_tb_n3_a3_emit.rs:50:5
   |
44 | struct Harness {
   |        ------- field in this struct
...
50 |     repo_path: std::path::PathBuf,
   |     ^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `turingosv4` (test "constitution_tb_n3_a3_emit") generated 1 warning
warning: variable `payC` should have a snake case name
   --> tests/constitution_router_price_quote.rs:178:10
    |
178 |     for &payC in &[100_000_i64, 1_000_000, 5_000_000, 10_000_000] {
    |          ^^^^ help: convert the identifier to snake case: `pay_c`
    |
    = note: `#[warn(non_snake_case)]` (part of `#[warn(nonstandard_style)]`) on by default

warning: variable `payC` should have a snake case name
   --> tests/constitution_router_price_quote.rs:296:9
    |
296 |     let payC: i64 = 5_000_000;
    |         ^^^^ help: convert the identifier to snake case: `pay_c`

warning: `turingosv4` (test "constitution_router_price_quote") generated 2 warnings
warning: unused import: `EscrowEntry`
  --> tests/tb_2_runtime_boundary.rs:33:43
   |
33 | use turingosv4::state::q_state::{AgentId, EscrowEntry, Hash, QState, TaskId, TxId};
   |                                           ^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `turingosv4` (test "tb_18r_lean_result_cas_resolves") generated 1 warning (run `cargo fix --test "tb_18r_lean_result_cas_resolves" -p turingosv4` to apply 1 suggestion)
warning: `turingosv4` (test "tb_2_runtime_boundary") generated 1 warning (run `cargo fix --test "tb_2_runtime_boundary" -p turingosv4` to apply 1 suggestion)
warning: variant `NotYetLanded` is never constructed
  --> tests/constitution_architect_verbatim_struct_binding.rs:36:5
   |
34 | enum LandingStatus {
   |      ------------- variant in this enum
35 |     Landed,
36 |     NotYetLanded,
   |     ^^^^^^^^^^^^
   |
   = note: `LandingStatus` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `turingosv4` (test "constitution_architect_verbatim_struct_binding") generated 1 warning
warning: unused import: `TxId`
  --> tests/tb_7_atom6_chain_backed_smoke.rs:61:49
   |
61 | use turingosv4::state::q_state::{AgentId, Hash, TxId};
   |                                                 ^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `turingosv4` (test "tb_7_atom6_chain_backed_smoke") generated 1 warning (run `cargo fix --test "tb_7_atom6_chain_backed_smoke" -p turingosv4` to apply 1 suggestion)
warning: unused import: `Cid`
  --> tests/tb_18r_audit_sampler_attempt_payload.rs:13:45
   |
13 | use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
   |                                             ^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `turingosv4` (test "tb_18r_audit_sampler_attempt_payload") generated 1 warning (run `cargo fix --test "tb_18r_audit_sampler_attempt_payload" -p turingosv4` to apply 1 suggestion)
warning: unused import: `EconomicState`
  --> src/sdk/econ_position.rs:95:21
   |
95 |         ClaimEntry, EconomicState, QState, Reputation, StakeEntry, TaskId, TxId,
   |                     ^^^^^^^^^^^^^

warning: unused import: `make_real_verifytx_signed_by`
    --> src/runtime/chain_derived_run_facts.rs:1078:35
     |
1078 |     use crate::runtime::adapter::{make_real_verifytx_signed_by, make_real_worktx_signed_by};
     |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ShareSidePair`
  --> src/runtime/audit_views.rs:34:56
   |
34 |     AgentId, EconomicState, LpShareAmount, PoolStatus, ShareSidePair,
   |                                                        ^^^^^^^^^^^^^

warning: variable does not need to be mutable
   --> src/top_white/predicates/registry.rs:255:17
    |
255 |             let mut h = Sha256::new();
    |                 ----^
    |                 |
    |                 help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:592:19
    |
592 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `turingosv4` (lib test) generated 20 warnings (15 duplicates) (run `cargo fix --lib -p turingosv4 --tests` to apply 4 suggestions)
warning: useless assignment of field of type `turingosv4::state::Hash` to itself
   --> tests/six_axioms_alignment.rs:115:5
    |
115 |     q.predicate_registry_root_t = q.predicate_registry_root_t; // axiom 2
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: useless assignment of field of type `turingosv4::state::Hash` to itself
   --> tests/six_axioms_alignment.rs:120:5
    |
120 |     q.tool_registry_root_t = q.tool_registry_root_t; // axiom 4
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `turingosv4::bottom_white::ledger::transition_ledger::replay_full_transition`
   --> tests/tb_3_rsp1_formal_surface.rs:496:9
    |
496 |     use turingosv4::bottom_white::ledger::transition_ledger::replay_full_transition;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `entries`
   --> tests/tb_3_rsp1_formal_surface.rs:512:9
    |
512 |     let entries = {
    |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_entries`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `turingosv4` (test "six_axioms_alignment") generated 2 warnings
warning: `turingosv4` (test "tb_3_rsp1_formal_surface") generated 2 warnings (run `cargo fix --test "tb_3_rsp1_formal_surface" -p turingosv4` to apply 2 suggestions)
warning: variant `NotYetLanded` is never constructed
  --> tests/constitution_class4_atomic_rollback_witness.rs:36:5
   |
34 | enum LandingStatus {
   |      ------------- variant in this enum
35 |     Landed,
36 |     NotYetLanded,
   |     ^^^^^^^^^^^^
   |
   = note: `LandingStatus` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `turingosv4` (test "constitution_class4_atomic_rollback_witness") generated 1 warning
warning: unused import: `MICRO_PER_COIN`
  --> tests/walkthrough_inv3_conservation.rs:18:45
   |
18 | use turingosv4::economy::money::{MicroCoin, MICRO_PER_COIN};
   |                                             ^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: field `royalty_edges` is never read
  --> tests/walkthrough_inv3_conservation.rs:26:5
   |
22 | struct EconStateMinimal {
   |        ---------------- field in this struct
...
26 |     royalty_edges: Vec<(String, String, i64)>,     // (creator, beneficiary_work_tx, weight_micro)
   |     ^^^^^^^^^^^^^
   |
   = note: `EconStateMinimal` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `turingosv4` (test "walkthrough_inv3_conservation") generated 2 warnings (run `cargo fix --test "walkthrough_inv3_conservation" -p turingosv4` to apply 1 suggestion)
warning: variable does not need to be mutable
   --> tests/tb_8_minimal_payout.rs:425:9
    |
425 |     let mut h = fresh_harness(genesis_with_balances(&[("a", 1)]));
    |         ----^
    |         |
    |         help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `std::path::Path`
  --> tests/constitution_art_v3_amendment_log.rs:29:5
   |
29 | use std::path::Path;
   |     ^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `turingosv4` (test "constitution_art_v3_amendment_log") generated 1 warning (run `cargo fix --test "constitution_art_v3_amendment_log" -p turingosv4` to apply 1 suggestion)
warning: `turingosv4` (test "tb_8_minimal_payout") generated 1 warning (run `cargo fix --test "tb_8_minimal_payout" -p turingosv4` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 48.01s
     Running unittests src/lib.rs (target/debug/deps/turingosv4-337b37021c2192ab)

running 1 test
test boot::tests::verify_trust_root_passes_on_intact_repo ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 578 filtered out; finished in 0.41s

     Running unittests src/bin/audit_dashboard.rs (target/debug/deps/audit_dashboard-c313cdea6d017cd9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running unittests src/bin/audit_tape.rs (target/debug/deps/audit_tape-bde31ad00ae90997)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/audit_tape_tamper.rs (target/debug/deps/audit_tape_tamper-492167426d104b7b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/gen_run_summary.rs (target/debug/deps/gen_run_summary-ae5f1b098ecd4adb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/generate_markov_capsule.rs (target/debug/deps/generate_markov_capsule-6f89437a3897feaf)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/tb_18r_compute_invariant.rs (target/debug/deps/tb_18r_compute_invariant-76335dd15b0555a6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/turingosv4-c93a21f17f9da5e3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/verify_chaintape.rs (target/debug/deps/verify_chaintape-c683ef5be37cca38)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/co1_7_extra_cas_payload_round_trip.rs (target/debug/deps/co1_7_extra_cas_payload_round_trip-1c91aeeb33b9bc14)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/co1_7_extra_git2_writer_head_oid_defense.rs (target/debug/deps/co1_7_extra_git2_writer_head_oid_defense-417eac67654c1510)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/co1_7_extra_sequencer_head_t_advancement.rs (target/debug/deps/co1_7_extra_sequencer_head_t_advancement-c3a4a617f94de291)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/conformance_stubs.rs (target/debug/deps/conformance_stubs-68a185c632b6b30d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 117 filtered out; finished in 0.00s

     Running tests/constitution_admission_no_fail_open_default.rs (target/debug/deps/constitution_admission_no_fail_open_default-50b4302ab194c60d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/constitution_aggregate_report.rs (target/debug/deps/constitution_aggregate_report-aabb68c36abd3511)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/constitution_architect_verbatim_struct_binding.rs (target/debug/deps/constitution_architect_verbatim_struct_binding-5e1f02bcda715e72)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/constitution_art_v3_amendment_log.rs (target/debug/deps/constitution_art_v3_amendment_log-7e1e42abe7504487)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/constitution_audit_tamper_3_of_3.rs (target/debug/deps/constitution_audit_tamper_3_of_3-97a1ab461cc0ff77)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/constitution_audit_views.rs (target/debug/deps/constitution_audit_views-500d9518a83f9d54)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/constitution_benchmark_manifest.rs (target/debug/deps/constitution_benchmark_manifest-d7ab19e9a7322394)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/constitution_class4_atomic_rollback_witness.rs (target/debug/deps/constitution_class4_atomic_rollback_witness-f5e11195dd39c514)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/constitution_closure_3_no_trivial_asserts.rs (target/debug/deps/constitution_closure_3_no_trivial_asserts-26ee54b3b588269e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/constitution_completeset_hardening.rs (target/debug/deps/constitution_completeset_hardening-13e6464540c0044c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/constitution_completeset_merge.rs (target/debug/deps/constitution_completeset_merge-4d945a14bc2a56ba)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/constitution_cpmm_pool.rs (target/debug/deps/constitution_cpmm_pool-2823c851be408637)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/constitution_cpmm_swap.rs (target/debug/deps/constitution_cpmm_swap-00012c63a78dd734)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/constitution_diversity.rs (target/debug/deps/constitution_diversity-f36e42461407ab05)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/constitution_economy_gate.rs (target/debug/deps/constitution_economy_gate-e4502cf47857e289)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/constitution_economy_strict_equality.rs (target/debug/deps/constitution_economy_strict_equality-3319ff87ef73dfeb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/constitution_fc1_runtime_loop.rs (target/debug/deps/constitution_fc1_runtime_loop-2df2182a82c332c1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/constitution_fc2_boot.rs (target/debug/deps/constitution_fc2_boot-573a281c34b48eb0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/constitution_fc3_evidence_binding.rs (target/debug/deps/constitution_fc3_evidence_binding-415c946a901d0bc2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/constitution_fc3_inv1_capsule_integrity_regen.rs (target/debug/deps/constitution_fc3_inv1_capsule_integrity_regen-dff86e76054da6ff)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/constitution_fc3_meta.rs (target/debug/deps/constitution_fc3_meta-1d7a609eb60c394c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/constitution_g1_resume.rs (target/debug/deps/constitution_g1_resume-6e6abf7d2e25b11b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/constitution_head_t_c2_multi_ref.rs (target/debug/deps/constitution_head_t_c2_multi_ref-ea2d6dfe91270d26)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/constitution_head_t_witness.rs (target/debug/deps/constitution_head_t_witness-dc12aab72e20fc9b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/constitution_l4e_body_integrity.rs (target/debug/deps/constitution_l4e_body_integrity-749c785a2d7f1300)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/constitution_market_quarantine.rs (target/debug/deps/constitution_market_quarantine-9e01ed28e4a7249c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/constitution_market_seed_hardening.rs (target/debug/deps/constitution_market_seed_hardening-f24333e6edbde0f2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/constitution_n1_agent_economy_a3.rs (target/debug/deps/constitution_n1_agent_economy_a3-247922d99ed62e72)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/constitution_n1_agent_economy_a4.rs (target/debug/deps/constitution_n1_agent_economy_a4-a36b17f3fa1c7b4f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/constitution_n2_event_resolve.rs (target/debug/deps/constitution_n2_event_resolve-90d76241cece237b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/constitution_no_evidence_drift_in_tests.rs (target/debug/deps/constitution_no_evidence_drift_in_tests-e0056e4a446613d6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/constitution_no_parallel_ledger.rs (target/debug/deps/constitution_no_parallel_ledger-bb881f35b6959931)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/constitution_pcp_corpus.rs (target/debug/deps/constitution_pcp_corpus-5e24467544edcb74)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/constitution_pcp_corpus_phase2.rs (target/debug/deps/constitution_pcp_corpus_phase2-5d92bc11a9bb6e02)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/constitution_polymarket_event_state_gate.rs (target/debug/deps/constitution_polymarket_event_state_gate-0abe4cfd13611085)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/constitution_polymarket_smoke.rs (target/debug/deps/constitution_polymarket_smoke-c6b342fef441244a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/constitution_predicate_gate.rs (target/debug/deps/constitution_predicate_gate-d49b8d70a7d050c0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/constitution_prompt_capsule.rs (target/debug/deps/constitution_prompt_capsule-a7b4626efe1f028d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/constitution_router_buy_with_coin.rs (target/debug/deps/constitution_router_buy_with_coin-6feb6e103ee75728)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/constitution_router_price_quote.rs (target/debug/deps/constitution_router_price_quote-ae3d41827ff96f2d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/constitution_runner_invariant_formula.rs (target/debug/deps/constitution_runner_invariant_formula-6fafa11ae80ffd3e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/constitution_shielding_evidence_binding.rs (target/debug/deps/constitution_shielding_evidence_binding-dc44fa44f7507dd1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/constitution_shielding_gate.rs (target/debug/deps/constitution_shielding_gate-3dc994ecb84ced11)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/constitution_tape_canonical_gate.rs (target/debug/deps/constitution_tape_canonical_gate-4bccf22b299dc5f3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/constitution_tb_n3_a3_emit.rs (target/debug/deps/constitution_tb_n3_a3_emit-357a2b0becc8c40b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/constitution_tb_n3_invest_routing.rs (target/debug/deps/constitution_tb_n3_invest_routing-aa3cb8554b980829)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/constitution_wave3_evidence_binding.rs (target/debug/deps/constitution_wave3_evidence_binding-1a1c799aaef1fea7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/constitution_wilson_ci.rs (target/debug/deps/constitution_wilson_ci-2637d124611ad4fd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/economic_state_reconstruct.rs (target/debug/deps/economic_state_reconstruct-655e509e99a9f92c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/fc_alignment_conformance.rs (target/debug/deps/fc_alignment_conformance-a7141525b76eb3c5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 33 filtered out; finished in 0.00s

     Running tests/four_element_mapping.rs (target/debug/deps/four_element_mapping-3426449622d9121e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/markov_pointer_de_canonicalize.rs (target/debug/deps/markov_pointer_de_canonicalize-57c1bcbb66cb86b0)
     Running tests/q_state_reconstruct.rs (target/debug/deps/q_state_reconstruct-fad760b150e3f1e6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/r_022_integration_orchestrator.rs (target/debug/deps/r_022_integration_orchestrator-f761808af55e615c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/six_axioms_alignment.rs (target/debug/deps/six_axioms_alignment-161d6333462ad25a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/system_keypair_generation.rs (target/debug/deps/system_keypair_generation-ff97cf087eb0475f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/system_keypair_load_and_decrypt.rs (target/debug/deps/system_keypair_load_and_decrypt-cbe2ce7b6d099a35)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/system_keypair_rotation_proof.rs (target/debug/deps/system_keypair_rotation_proof-06c9ebb0e7dcaf10)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/system_keypair_sign_only_from_runner.rs (target/debug/deps/system_keypair_sign_only_from_runner-d514657119ccadbb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/system_keypair_verify_correctness.rs (target/debug/deps/system_keypair_verify_correctness-3a1f08846249e94c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tb_11_epistemic_exhaust.rs (target/debug/deps/tb_11_epistemic_exhaust-a88ebeaad68acadc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/tb_12_node_exposure_index.rs (target/debug/deps/tb_12_node_exposure_index-28f6bc65c16b570a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/tb_13_chaintape_smoke.rs (target/debug/deps/tb_13_chaintape_smoke-34f4bdf510c8807f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tb_13_complete_set.rs (target/debug/deps/tb_13_complete_set-dca1331daec4b763)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 19 filtered out; finished in 0.00s

     Running tests/tb_13_legacy_cpmm_forward_fence.rs (target/debug/deps/tb_13_legacy_cpmm_forward_fence-1dde7bf978dea784)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/tb_14_canonical_masking_smoke.rs (target/debug/deps/tb_14_canonical_masking_smoke-1fc1c1efcc355ee6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/tb_14_chaintape_smoke.rs (target/debug/deps/tb_14_chaintape_smoke-9882f94863c48cf8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tb_14_halt_triggers.rs (target/debug/deps/tb_14_halt_triggers-621f4df487580a1d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/tb_14_mask_set.rs (target/debug/deps/tb_14_mask_set-c12b11648c09194d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

     Running tests/tb_14_price_index.rs (target/debug/deps/tb_14_price_index-64a67452f141cd21)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/tb_15_halt_triggers.rs (target/debug/deps/tb_15_halt_triggers-f027b624c592d901)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/tb_16_audit_tape_binary.rs (target/debug/deps/tb_16_audit_tape_binary-3107fb7f52014170)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tb_16_dashboard_live_regen.rs (target/debug/deps/tb_16_dashboard_live_regen-66e683f6ffbe680b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/tb_16_halt_triggers.rs (target/debug/deps/tb_16_halt_triggers-3b684a879bab2c2f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

     Running tests/tb_17_irreversible_action_examples.rs (target/debug/deps/tb_17_irreversible_action_examples-5b4487a71be56fe6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/tb_17_markov_inheritance_policy.rs (target/debug/deps/tb_17_markov_inheritance_policy-a1ecb78bbfb16cd7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/tb_17_minif2f_scale_separation.rs (target/debug/deps/tb_17_minif2f_scale_separation-2afdeb7150b67743)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/tb_18_deferred_finalize_idempotency.rs (target/debug/deps/tb_18_deferred_finalize_idempotency-9e662014774f5027)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/tb_18_evidence_capsule_outcome_propagation.rs (target/debug/deps/tb_18_evidence_capsule_outcome_propagation-607ca269c195ee2b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tb_18r_attempt_chain_root_payload_schema.rs (target/debug/deps/tb_18r_attempt_chain_root_payload_schema-a83cef4dd00e2807)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/tb_18r_attempt_outcome_partial_accepted_repr_stability.rs (target/debug/deps/tb_18r_attempt_outcome_partial_accepted_repr_stability-af367df59c8f8d6b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/tb_18r_attempt_routes_to_l4_or_l4e.rs (target/debug/deps/tb_18r_attempt_routes_to_l4_or_l4e-bb8d3b09557dbd6a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out     Running tests/tb_18r_attempt_telemetry_per_llm_call.rs (target/debug/deps/tb_18r_attempt_telemetry_per_llm_call-a4df3247e89c23de)
; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/tb_18r_attempt_telemetry_serialize.rs (target/debug/deps/tb_18r_attempt_telemetry_serialize-3e66e5d926c65718)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tb_18r_audit_lean_stderr_tamper_detected.rs (target/debug/deps/tb_18r_audit_lean_stderr_tamper_detected-89cc07560ab150f2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/tb_18r_audit_sampler_attempt_payload.rs (target/debug/deps/tb_18r_audit_sampler_attempt_payload-d95c95edeedeb48c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/tb_18r_cas_reload_split_brain.rs (target/debug/deps/tb_18r_cas_reload_split_brain-eaf5835da9cf0d94)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/tb_18r_chain_attempt_invariant.rs (target/debug/deps/tb_18r_chain_attempt_invariant-335b465aececdf87)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/tb_18r_chain_derived_facts_exact_accounting.rs (target/debug/deps/tb_18r_chain_derived_facts_exact_accounting-bb323ffdd190fff0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tb_18r_dashboard_attempt_dag_replay.rs (target/debug/deps/tb_18r_dashboard_attempt_dag_replay-1ff4a2480fa8e4f4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/tb_18r_drain_barrier_quiescence.rs (target/debug/deps/tb_18r_drain_barrier_quiescence-2e15da5f844d4aa0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tb_18r_final_composite_attempt_chain_root.rs (target/debug/deps/tb_18r_final_composite_attempt_chain_root-2812c5b4b96e47fc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/tb_18r_lean_reject_in_l4e.rs (target/debug/deps/tb_18r_lean_reject_in_l4e-44fb83d2c6257f3d)
     Running tests/tb_18r_lean_result_cas_resolves.rs (target/debug/deps/tb_18r_lean_result_cas_resolves-d043795b19b4eba6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tb_18r_lean_verdict_kind_consistency.rs (target/debug/deps/tb_18r_lean_verdict_kind_consistency-087d4a13e63f89a0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/tb_18r_lean_verdict_kind_repr_stability.rs (target/debug/deps/tb_18r_lean_verdict_kind_repr_stability-5ad712d5e5a5e3e3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/tb_18r_markov_failure_cluster_from_chain.rs (target/debug/deps/tb_18r_markov_failure_cluster_from_chain-96a2fee58919031c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/tb_18r_no_raw_response_in_attempt_payload.rs (target/debug/deps/tb_18r_no_raw_response_in_attempt_payload-f8e09f9d5348aa23)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/tb_18r_rejection_class_repr_stability.rs (target/debug/deps/tb_18r_rejection_class_repr_stability-0229716876ba31fd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tb_1_acceptance.rs (target/debug/deps/tb_1_acceptance-2de3c62d814788b9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.00s

     Running tests/tb_2_runtime_boundary.rs (target/debug/deps/tb_2_runtime_boundary-d1c66643531a3579)
     Running tests/tb_3_bridge_deletion_invariant.rs (target/debug/deps/tb_3_bridge_deletion_invariant-30b0b0ced9b03a3c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/tb_3_rsp1_formal_surface.rs (target/debug/deps/tb_3_rsp1_formal_surface-59026d8410c708c4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/tb_4_rsp2_admission_surface.rs (target/debug/deps/tb_4_rsp2_admission_surface-1dff746f2474ba88)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/tb_5_anti_drift.rs (target/debug/deps/tb_5_anti_drift-6f08b41a399daecd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tb_5_challenge_resolve_surface.rs (target/debug/deps/tb_5_challenge_resolve_surface-a3ef8a9fa8de0253)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

     Running tests/tb_5_system_ingress_barrier.rs (target/debug/deps/tb_5_system_ingress_barrier-a1908cf12683f58d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/tb_6_agent_audit_trail.rs (target/debug/deps/tb_6_agent_audit_trail-bc758492d19fbb87)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/tb_6_l4e_jsonl_persistence.rs (target/debug/deps/tb_6_l4e_jsonl_persistence-bb0c5a1a52aa2d77)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/tb_6_run_summary.rs (target/debug/deps/tb_6_run_summary-e1592251f6d1a9f8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tb_6_runtime_chaintape_bootstrap.rs (target/debug/deps/tb_6_runtime_chaintape_bootstrap-962eb52a8bba901a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

     Running tests/tb_6_verify_chaintape.rs (target/debug/deps/tb_6_verify_chaintape-91db2ba8c45f2343)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/tb_7_atom6_chain_backed_smoke.rs (target/debug/deps/tb_7_atom6_chain_backed_smoke-e9192fa941b34411)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tb_7_authoritative_routing.rs (target/debug/deps/tb_7_authoritative_routing-77a0a18efd883f9d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/tb_7_legacy_append_regression.rs (target/debug/deps/tb_7_legacy_append_regression-2ef63e8f9d7d5160)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tb_7r_parent_tx_conformance.rs (target/debug/deps/tb_7r_parent_tx_conformance-9abb5559f02bca4b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/tb_8_minimal_payout.rs (target/debug/deps/tb_8_minimal_payout-894f94b354b3c423)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

     Running tests/wal_resume.rs (target/debug/deps/wal_resume-f71ea8d7ce07b4a4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/walkthrough_inv3_conservation.rs (target/debug/deps/walkthrough_inv3_conservation-cf3d662adf83149e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s


codex
The narrow SG-G1 resume test is green: 7/7 passed. Trust-root test also passed, and the recorded hash matches `sha256sum`; I’m doing one final consistency pass on whether the manifest/keystore check is complete enough for the end-to-end replay claims.
exec
/bin/bash -lc "nl -ba src/runtime/agent_keypairs.rs | sed -n '200,300p;392,455p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   200	            .field("agent_count", &self.keypairs.len())
   201	            .field(
   202	                "agent_ids",
   203	                &self.keypairs.keys().collect::<Vec<_>>(),
   204	            )
   205	            .field(
   206	                "durable",
   207	                &self.durable.as_ref().map(|d| d.keystore_path.clone()),
   208	            )
   209	            .finish()
   210	    }
   211	}
   212	
   213	impl AgentKeypairRegistry {
   214	    /// TRACE_MATRIX FC1-N14: open or initialize an agent keypair registry
   215	    /// rooted at the runtime repo. Manifest written at
   216	    /// `<runtime_repo>/agent_pubkeys.json`. Mirrors TB-6 fail-closed
   217	    /// non-empty-runtime-repo gate (refuses reopen when manifest exists).
   218	    pub fn open(runtime_repo_path: &Path) -> Result<Self, AgentKeypairError> {
   219	        let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
   220	        if manifest_path.exists() {
   221	            return Err(AgentKeypairError::ManifestAlreadyExists {
   222	                path: manifest_path,
   223	            });
   224	        }
   225	        let registry = Self {
   226	            keypairs: BTreeMap::new(),
   227	            manifest_path,
   228	            durable: None,
   229	        };
   230	        registry.persist_manifest()?;
   231	        Ok(registry)
   232	    }
   233	
   234	    /// TRACE_MATRIX FC1-N14 (TB-9 Atom 1): open or initialize an agent keypair
   235	    /// registry **with durable cross-run identity**. The encrypted keystore at
   236	    /// `durable_keystore_path` (typically `~/.turingos/keystore/agent_keystore.enc`
   237	    /// per [`crate::runtime::agent_keystore::default_agent_keystore_path`]) is
   238	    /// loaded if present — every saved AgentId → secret binding is reconstructed
   239	    /// in-memory before the first `sign()` call.
   240	    ///
   241	    /// Per-run manifest at `<runtime_repo>/agent_pubkeys.json` is still written
   242	    /// (defense-in-depth replay sidecar; TB-7 semantics retained). The
   243	    /// fail-closed-on-existing manifest gate STILL applies — runtime_repo is
   244	    /// supposed to be fresh per evaluator run.
   245	    ///
   246	    /// On every subsequent `sign()` that triggers a fresh keypair generation,
   247	    /// the durable keystore is re-encrypted and atomically written so the new
   248	    /// AgentId → secret binding survives evaluator exit.
   249	    pub fn generate_or_load_durable(
   250	        runtime_repo_path: &Path,
   251	        durable_keystore_path: &Path,
   252	        password: secrecy::SecretString,
   253	    ) -> Result<Self, AgentKeypairError> {
   254	        let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
   255	        if manifest_path.exists() {
   256	            return Err(AgentKeypairError::ManifestAlreadyExists {
   257	                path: manifest_path,
   258	            });
   259	        }
   260	        let (secrets_map, _fresh) =
   261	            crate::runtime::agent_keystore::load_or_empty(durable_keystore_path, &password)
   262	                .map_err(|e| AgentKeypairError::Serde(format!("durable keystore: {e}")))?;
   263	        let mut keypairs: BTreeMap<AgentId, AgentKeypair> = BTreeMap::new();
   264	        for (agent_id_raw, seed) in secrets_map {
   265	            keypairs.insert(AgentId(agent_id_raw), AgentKeypair::from_secret_bytes(seed));
   266	        }
   267	        let registry = Self {
   268	            keypairs,
   269	            manifest_path,
   270	            durable: Some(DurableConfig {
   271	                keystore_path: durable_keystore_path.to_path_buf(),
   272	                password,
   273	            }),
   274	        };
   275	        registry.persist_manifest()?;
   276	        Ok(registry)
   277	    }
   278	
   279	    /// TRACE_MATRIX FC2-INV8 (TB-G G1.1 architect §8 SIGNED 2026-05-11 user
   280	    /// directive "断点续作是本项目的核心"; Turing-machine fundamentalist
   281	    /// reading of FC2 §3.2 "every real evidence run must be replayable from
   282	    /// genesis_report + ChainTape + CAS + **agent registry** + system pubkeys"):
   283	    /// open an agent keypair registry on a non-empty runtime_repo by
   284	    /// loading the existing `agent_pubkeys.json` manifest INSTEAD of
   285	    /// fail-closing with `ManifestAlreadyExists`.
   286	    ///
   287	    /// Required by G1.1 resume mode at the **binary** layer: the
   288	    /// evaluator's swarm bootstrap re-attaches to a runtime_repo that
   289	    /// already contains both `pinned_pubkeys.json` (system pubkeys —
   290	    /// G1.1 kernel-side covers) and `agent_pubkeys.json` (agent registry
   291	    /// — this entry covers). Without this, the binary path panics on
   292	    /// the second problem even though the kernel sequencer happily
   293	    /// resumes.
   294	    ///
   295	    /// Semantics:
   296	    /// - **Manifest absent** → fail-closed `ManifestAbsentInResume`.
   297	    ///   Resume mode is contractually predicated on the manifest
   298	    ///   existing; falling through to fresh init would silently
   299	    ///   discard the prior agent registry (constitution violation).
   300	    /// - **Manifest present** → parse it (fail-closed on parse error),
   392	    pub fn get_or_create(&mut self, agent_id: &AgentId) -> Result<&AgentKeypair, AgentKeypairError> {
   393	        if !self.keypairs.contains_key(agent_id) {
   394	            let kp = AgentKeypair::generate()?;
   395	            self.keypairs.insert(agent_id.clone(), kp);
   396	            self.persist_manifest()?;
   397	        }
   398	        Ok(self.keypairs.get(agent_id).expect("just inserted"))
   399	    }
   400	
   401	    /// TRACE_MATRIX FC1-N14: sign a 32-byte canonical digest under `agent_id`.
   402	    /// Generates the keypair on-demand if absent. This is the primary call
   403	    /// site for evaluator append-branch / OMEGA-branch routing in Atom 2/3.
   404	    pub fn sign(
   405	        &mut self,
   406	        agent_id: &AgentId,
   407	        digest: [u8; 32],
   408	    ) -> Result<AgentSignature, AgentKeypairError> {
   409	        let keypair = self.get_or_create(agent_id)?;
   410	        keypair.sign_digest(digest)
   411	    }
   412	
   413	    /// TRACE_MATRIX FC1-N14: snapshot the public-key map as a manifest object
   414	    /// (sorted by AgentId for determinism).
   415	    pub fn manifest(&self) -> AgentPubkeyManifest {
   416	        AgentPubkeyManifest {
   417	            agents: self
   418	                .keypairs
   419	                .iter()
   420	                .map(|(id, kp)| (id.0.clone(), kp.public_key().to_hex()))
   421	                .collect(),
   422	        }
   423	    }
   424	
   425	    /// TRACE_MATRIX FC1-N14: path to the on-disk manifest.
   426	    pub fn manifest_path(&self) -> &Path {
   427	        &self.manifest_path
   428	    }
   429	
   430	    /// Atomic write: tmp file + rename. JSON pretty-printed for inspection.
   431	    /// TB-9 Atom 1: also re-encrypts + atomically writes the durable keystore
   432	    /// when `self.durable` is populated.
   433	    fn persist_manifest(&self) -> Result<(), AgentKeypairError> {
   434	        let manifest = self.manifest();
   435	        let serialized = serde_json::to_string_pretty(&manifest)
   436	            .map_err(|e| AgentKeypairError::Serde(e.to_string()))?;
   437	        let tmp = self.manifest_path.with_extension("json.tmp");
   438	        {
   439	            let mut f = OpenOptions::new()
   440	                .write(true)
   441	                .create(true)
   442	                .truncate(true)
   443	                .open(&tmp)?;
   444	            f.write_all(serialized.as_bytes())?;
   445	            f.sync_all()?;
   446	        }
   447	        std::fs::rename(&tmp, &self.manifest_path)?;
   448	
   449	        if let Some(durable) = &self.durable {
   450	            let mut secrets: BTreeMap<String, [u8; AGENT_SECRET_LEN]> = BTreeMap::new();
   451	            for (id, kp) in &self.keypairs {
   452	                secrets.insert(id.0.clone(), kp.secret_bytes());
   453	            }
   454	            crate::runtime::agent_keystore::save(
   455	                &durable.keystore_path,

exec
/bin/bash -lc "rg -n \"set_agent_pubkeys|manifest\\(\\)|agent_keypairs|agent_pubkeys\" experiments/minif2f_v4/src/chain_runtime.rs src/runtime src/state/sequencer.rs -g '*.rs'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/sequencer.rs:3367:    /// when the optional `agent_pubkeys` manifest is set. Either the
src/state/sequencer.rs:3710:    /// **TB-13 enforcement**: when set via [`Sequencer::set_agent_pubkeys`],
src/state/sequencer.rs:3715:    agent_pubkeys: std::sync::OnceLock<Arc<crate::runtime::agent_keypairs::AgentPubkeyManifest>>,
src/state/sequencer.rs:3809:            agent_pubkeys: std::sync::OnceLock::new(),
src/state/sequencer.rs:3822:    /// `<runtime_repo>/agent_pubkeys.json` after agent registration.
src/state/sequencer.rs:3825:    pub fn set_agent_pubkeys(
src/state/sequencer.rs:3827:        manifest: Arc<crate::runtime::agent_keypairs::AgentPubkeyManifest>,
src/state/sequencer.rs:3828:    ) -> Result<(), Arc<crate::runtime::agent_keypairs::AgentPubkeyManifest>> {
src/state/sequencer.rs:3829:        self.agent_pubkeys.set(manifest)
src/state/sequencer.rs:3917:        // conditional-share variants. Opt-in via `set_agent_pubkeys` —
src/state/sequencer.rs:3923:        if let Some(manifest) = self.agent_pubkeys.get() {
src/state/sequencer.rs:3924:            use crate::runtime::agent_keypairs::verify_agent_signature;
experiments/minif2f_v4/src/chain_runtime.rs:56:use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
experiments/minif2f_v4/src/chain_runtime.rs:83:    pub agent_keypairs: Option<Arc<Mutex<AgentKeypairRegistry>>>,
experiments/minif2f_v4/src/chain_runtime.rs:217:        // <runtime_repo>/agent_pubkeys.json (TB-7 replay sidecar; unchanged).
experiments/minif2f_v4/src/chain_runtime.rs:231:        let agent_keypairs: Option<Arc<Mutex<AgentKeypairRegistry>>> =
experiments/minif2f_v4/src/chain_runtime.rs:241:                // `agent_pubkeys.json` IS the agent registry — load it
experiments/minif2f_v4/src/chain_runtime.rs:258:                // — but a non-empty chain WITHOUT an agent_pubkeys.json
experiments/minif2f_v4/src/chain_runtime.rs:273:                        "[chaintape/tb9-resume] agent_keypairs resume must succeed \
experiments/minif2f_v4/src/chain_runtime.rs:279:                         agent_pubkeys.json and the durable keystore disagree about agent \
experiments/minif2f_v4/src/chain_runtime.rs:292:                        "[chaintape/tb9] agent_keypairs durable init must succeed (fresh runtime_repo guarantees \
experiments/minif2f_v4/src/chain_runtime.rs:356:            agent_keypairs,
experiments/minif2f_v4/src/chain_runtime.rs:476:    // runtime_repo, cas_path, system_pubkey, agent_pubkeys path,
experiments/minif2f_v4/src/chain_runtime.rs:516:        agent_pubkeys_path: "agent_pubkeys.json".into(),
experiments/minif2f_v4/src/chain_runtime.rs:540:    /// a SharedChain with `chaintape_bundle = None`, `agent_keypairs = None`,
experiments/minif2f_v4/src/chain_runtime.rs:573:        assert!(chain.agent_keypairs.is_none());
src/runtime/adapter.rs:23:use crate::runtime::agent_keypairs::{AgentKeypairError, AgentKeypairRegistry};
src/runtime/adapter.rs:145:///    on-disk `agent_pubkeys.json` manifest (Atom 4 verify_chaintape
src/runtime/adapter.rs:1457:        use crate::runtime::agent_keypairs::{verify_agent_signature, AgentPubkeyManifest};
src/runtime/genesis_report.rs:58:    pub agent_pubkeys_path: String,
src/runtime/genesis_report.rs:140:            agent_pubkeys_path: "agent_pubkeys.json".into(),
src/runtime/genesis_report.rs:191:            agent_pubkeys_path: "agent_pubkeys.json".into(),
src/runtime/chain_derived_run_facts.rs:1079:    use crate::runtime::agent_keypairs::AgentKeypairRegistry;
src/runtime/chain_derived_run_facts.rs:1138:            AgentKeypairRegistry::open(&cfg.runtime_repo_path).expect("open agent_keypairs");
src/runtime/verify.rs:139:    /// `AgentSignature` verifies against the per-run `agent_pubkeys.json`
src/runtime/verify.rs:144:    /// signatures verify against agent_pubkeys.json. False on any
src/runtime/verify.rs:333:    // AgentSignature against the per-run agent_pubkeys.json manifest.
src/runtime/verify.rs:364:/// agent signatures against the on-disk `agent_pubkeys.json` manifest, plus
src/runtime/verify.rs:377:    use crate::runtime::agent_keypairs::{verify_agent_signature, AgentPubkeyManifest};
src/runtime/verify.rs:381:    let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
src/runtime/verify.rs:383:        // No agent_pubkeys.json (legacy / pre-Atom-1 chain). Both indicators
src/runtime/benchmark_manifest.rs:253:    fn good_manifest() -> BenchmarkManifest {
src/runtime/benchmark_manifest.rs:273:        let m = good_manifest();
src/runtime/benchmark_manifest.rs:280:        let mut m = good_manifest();
src/runtime/benchmark_manifest.rs:287:        let mut m = good_manifest();
src/runtime/benchmark_manifest.rs:294:        let mut m = good_manifest();
src/runtime/benchmark_manifest.rs:306:        let mut m = good_manifest();
src/runtime/benchmark_manifest.rs:313:        let mut m = good_manifest();
src/runtime/benchmark_manifest.rs:320:        let mut m = good_manifest();
src/runtime/benchmark_manifest.rs:327:        let mut m = good_manifest();
src/runtime/benchmark_manifest.rs:339:        let mut m = good_manifest();
src/runtime/benchmark_manifest.rs:346:        let mut m = good_manifest();
src/runtime/benchmark_manifest.rs:353:        let mut m = good_manifest();
src/runtime/benchmark_manifest.rs:360:        let mut m = good_manifest();
src/runtime/benchmark_manifest.rs:370:        let mut m = good_manifest();
src/runtime/benchmark_manifest.rs:377:        let mut m = good_manifest();
src/runtime/benchmark_manifest.rs:389:        let mut m = good_manifest();
src/runtime/benchmark_manifest.rs:399:        let mut m = good_manifest();
src/runtime/benchmark_manifest.rs:406:        let mut m = good_manifest();
src/runtime/benchmark_manifest.rs:418:        let m = good_manifest();
src/runtime/benchmark_manifest.rs:429:        let m = good_manifest();
src/runtime/audit_assertions.rs:10://! - `agent_pubkeys`    — `agent_pubkeys.json` (TB-7)
src/runtime/audit_assertions.rs:78:use crate::runtime::agent_keypairs::AgentPubkeyManifest;
src/runtime/audit_assertions.rs:103:    pub agent_pubkeys: PathBuf,
src/runtime/audit_assertions.rs:389:    let agent_manifest = AgentPubkeyManifest::load(&inputs.agent_pubkeys)
src/runtime/audit_assertions.rs:728:/// `assert_03_sandbox_agent_prefix` only checks the agent_pubkeys.json
src/runtime/audit_assertions.rs:3376:    feature_coverage.insert("TB-7_agent_pubkeys".into(), "GREEN".into());
src/runtime/agent_keypairs.rs:17://! | Public manifest     | `pinned_pubkeys.json`        | `agent_pubkeys.json`              |
src/runtime/agent_keypairs.rs:216:    /// `<runtime_repo>/agent_pubkeys.json`. Mirrors TB-6 fail-closed
src/runtime/agent_keypairs.rs:219:        let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
src/runtime/agent_keypairs.rs:230:        registry.persist_manifest()?;
src/runtime/agent_keypairs.rs:241:    /// Per-run manifest at `<runtime_repo>/agent_pubkeys.json` is still written
src/runtime/agent_keypairs.rs:254:        let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
src/runtime/agent_keypairs.rs:275:        registry.persist_manifest()?;
src/runtime/agent_keypairs.rs:284:    /// loading the existing `agent_pubkeys.json` manifest INSTEAD of
src/runtime/agent_keypairs.rs:290:    /// G1.1 kernel-side covers) and `agent_pubkeys.json` (agent registry
src/runtime/agent_keypairs.rs:309:    /// trigger `persist_manifest()` via `get_or_create()` — new agents
src/runtime/agent_keypairs.rs:317:        let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
src/runtime/agent_keypairs.rs:331:            .map_err(|e| AgentKeypairError::Serde(format!("agent_pubkeys.json: {e}")))?;
src/runtime/agent_keypairs.rs:359:                        "agent_pubkeys.json lists agent_id={agent_id_raw:?} but the \
src/runtime/agent_keypairs.rs:396:            self.persist_manifest()?;
src/runtime/agent_keypairs.rs:434:        let manifest = self.manifest();
src/runtime/agent_keypairs.rs:467:/// TRACE_MATRIX FC1-N14: on-disk shape of `agent_pubkeys.json`.
src/runtime/agent_keypairs.rs:532:    /// the on-disk `agent_pubkeys.json` references `agent_id` but the
src/runtime/agent_keypairs.rs:550:                write!(f, "agent_pubkeys.json already exists at {path:?}")
src/runtime/agent_keypairs.rs:555:                    "resume mode: agent_pubkeys.json missing at {path:?}; \
src/runtime/agent_keypairs.rs:563:                    "resume mode: agent_pubkeys.json / durable keystore inconsistency \
src/runtime/agent_keypairs.rs:629:            .manifest()
src/runtime/agent_keypairs.rs:645:        let in_mem = reg.manifest();
src/runtime/agent_keypairs.rs:657:    fn registry_open_refuses_existing_manifest() {
src/runtime/agent_keypairs.rs:718:            reg_a.manifest().get(&agent).expect("run A pubkey")
src/runtime/agent_keypairs.rs:730:        let pubkey_b = reg_b.manifest().get(&agent).expect("run B pubkey");
src/runtime/mod.rs:33:pub mod agent_keypairs;
src/runtime/mod.rs:47:/// TRACE_MATRIX § 3 orphan (see `handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md`): TB-7R Deliverable C — `genesis_report.json` emitter capturing constitution_hash + runtime_repo + cas_path + system_pubkey_hash + agent_pubkeys_path + initial_balances + (preseed only) task_id / task_open_tx / escrow_lock_tx. No canonical FC row exists yet (FC2 is Append/Submit, NOT Boot/Genesis); promotion target is a future TRACE_MATRIX revision under Article IV Boot. `FC-trace: Art.IV Boot + Art.I.1 + Art.III.4 + WP-§11`.

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/chain_runtime.rs | sed -n '300,370p' && nl -ba src/state/sequencer.rs | sed -n '3820,3945p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   300	            });
   301	
   302	        // Phase 1: opt-in tape persistence via env. WAL_DIR=<dir> enables WAL
   303	        // writes to <dir>/<problem>_<timestamp>.jsonl; resumes if file exists.
   304	        // Default off for backward-compat baseline runs.
   305	        let bus = if let Some(ref bundle) = chaintape_bundle {
   306	            info!(
   307	                "[chaintape] bus wired with Sequencer + on-disk ChainTape at {:?}",
   308	                bundle.runtime_repo_path
   309	            );
   310	            TuringBus::with_sequencer(kernel, config, bundle.sequencer.clone())
   311	        } else if let Ok(wal_dir) = std::env::var("WAL_DIR") {
   312	            let problem_stem = std::path::Path::new(problem_file)
   313	                .file_stem()
   314	                .map(|s| s.to_string_lossy().into_owned())
   315	                .unwrap_or_else(|| "unknown".into());
   316	            let resume_id = std::env::var("WAL_RESUME_ID").ok();
   317	            let id = resume_id.unwrap_or_else(|| {
   318	                std::time::SystemTime::now()
   319	                    .duration_since(std::time::UNIX_EPOCH)
   320	                    .map(|d| d.as_secs().to_string())
   321	                    .unwrap_or_else(|_| "0".into())
   322	            });
   323	            let wal_path = std::path::Path::new(&wal_dir)
   324	                .join(format!("{}_{}.jsonl", problem_stem, id));
   325	            info!("[wal] using {:?}", wal_path);
   326	            match TuringBus::with_wal_path(kernel, config, wal_path) {
   327	                Ok(b) => b,
   328	                Err(e) => {
   329	                    error!("[wal] open failed: {} — falling back to in-memory", e);
   330	                    TuringBus::new(
   331	                        Kernel::new(),
   332	                        BusConfig {
   333	                            max_payload_chars: 1200,
   334	                            max_payload_lines: 18,
   335	                            forbidden_patterns: vec![
   336	                                "native_decide".into(),
   337	                                "decide".into(),
   338	                                "omega".into(),
   339	                                "#eval".into(),
   340	                                "IO.Process".into(),
   341	                                "IO.FS".into(),
   342	                                "run_tac".into(),
   343	                                "unsafe".into(),
   344	                            ],
   345	                        },
   346	                    )
   347	                }
   348	            }
   349	        } else {
   350	            TuringBus::new(kernel, config)
   351	        };
   352	
   353	        Self {
   354	            bus,
   355	            chaintape_bundle,
   356	            agent_keypairs,
   357	            initial_balances_for_genesis_report,
   358	            chaintape_preseed_enabled,
   359	        }
   360	    }
   361	}
   362	
   363	/// TB-18 Atom B Phase 2: write the synthetic L4 + L4.E pipeline-liveness gate
   364	/// (TB-6 Atom 3 — synthetic TaskOpen + zero-stake WorkTx) plus the chain-level
   365	/// `genesis_report.json` (TB-7R Deliverable C). Lifted verbatim from
   366	/// `evaluator.rs::run_swarm` lines 1439-1562 — behavior is byte-identical to
   367	/// the inline code; this is a pure mechanical extraction.
   368	///
   369	/// **Caller contract**: must hold an `&mut bus` whose sequencer was wired from
   370	/// the same `bundle` reference passed here (i.e. `bus` and `bundle` come from
  3820	    ///
  3821	    /// Production binaries plumb this from
  3822	    /// `<runtime_repo>/agent_pubkeys.json` after agent registration.
  3823	    /// Tests may opt in by constructing an `AgentPubkeyManifest` from
  3824	    /// real keypairs.
  3825	    pub fn set_agent_pubkeys(
  3826	        &self,
  3827	        manifest: Arc<crate::runtime::agent_keypairs::AgentPubkeyManifest>,
  3828	    ) -> Result<(), Arc<crate::runtime::agent_keypairs::AgentPubkeyManifest>> {
  3829	        self.agent_pubkeys.set(manifest)
  3830	    }
  3831	
  3832	    /// TRACE_MATRIX TB-5 charter v2 § 4.2: peek pinned_pubkeys (for tests +
  3833	    /// observability; production callers should not depend on this).
  3834	    #[cfg(test)]
  3835	    pub fn pinned_pubkeys(&self) -> &crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys {
  3836	        &self.pinned_pubkeys
  3837	    }
  3838	
  3839	    /// TRACE_MATRIX TB-5 charter v2 § 4.2: peek next_emit_id (parallel to
  3840	    /// `next_submit_id_peek` for K1-style observability).
  3841	    pub fn next_emit_id_peek(&self) -> u64 {
  3842	        self.next_emit_id.load(Ordering::SeqCst)
  3843	    }
  3844	
  3845	    /// TRACE_MATRIX FC2-Submit + § 5.2.1: TB-5.0 Atom 2 agent-only ingress
  3846	    /// barrier (charter v2 § 4.2 + § 4.9 + preflight § 3.2; Anti-Oreo Art V.1.3).
  3847	    ///
  3848	    /// Accepts ONLY agent-submitted variants. System-emitted variants
  3849	    /// (FinalizeReward / TaskExpire / TerminalSummary; ChallengeResolve added
  3850	    /// in Atom 3) are rejected pre-queue with
  3851	    /// `SubmitError::SystemTxForbiddenOnAgentIngress`. This is the
  3852	    /// constitutional Anti-Oreo "agent ≠ direct state writer" boundary,
  3853	    /// structurally enforced (was a documented norm without live enforcement
  3854	    /// through TB-3 + TB-4; TB-5.0 retires that debt for system-tx).
  3855	    ///
  3856	    /// **WP-canonical reconciliation**: ChallengeResolveTx (TB-5 Atom 3) +
  3857	    /// SlashTx / SettlementTx / ProvisionalAcceptTx / ReputationUpdateTx
  3858	    /// (RSP-3.2+ / RSP-4 territory) will be added to the rejection match
  3859	    /// at their respective TB landings — each new system variant extends
  3860	    /// this list, never bypasses it.
  3861	    pub async fn submit_agent_tx(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
  3862	        // TB-5.0 ingress barrier: reject 4 system-emitted variants
  3863	        // (FinalizeReward / TaskExpire / TerminalSummary added in Atom 2;
  3864	        // ChallengeResolve added in Atom 3 when its TypedTx variant landed).
  3865	        match &tx {
  3866	            TypedTx::FinalizeReward(_)
  3867	            | TypedTx::TaskExpire(_)
  3868	            | TypedTx::TerminalSummary(_)
  3869	            | TypedTx::ChallengeResolve(_)
  3870	            // TB-11 Atom 1 (architect §6.2 ruling 2026-05-02): TaskBankruptcyTx
  3871	            // is system-emitted only; agent ingress must reject pre-queue per
  3872	            // Anti-Oreo (Art V.1.3). Construction goes through emit_system_tx.
  3873	            | TypedTx::TaskBankruptcy(_)
  3874	            // TB-N2 B2 (charter §3 B2; 2026-05-11): EventResolveTx is
  3875	            // system-emitted only; agent ingress must reject pre-queue per
  3876	            // Anti-Oreo. Construction goes through
  3877	            // `emit_system_tx(SystemEmitCommand::EventResolve)`.
  3878	            | TypedTx::EventResolve(_) => {
  3879	                return Err(SubmitError::SystemTxForbiddenOnAgentIngress);
  3880	            }
  3881	            // Agent-submitted variants — proceed to queue. TB-13 conditional-
  3882	            // share variants (CompleteSetMint / CompleteSetRedeem / MarketSeed)
  3883	            // are agent-signed and admit through the same ingress path.
  3884	            TypedTx::Work(_)
  3885	            | TypedTx::Verify(_)
  3886	            | TypedTx::Challenge(_)
  3887	            | TypedTx::Reuse(_)
  3888	            | TypedTx::TaskOpen(_)
  3889	            | TypedTx::EscrowLock(_)
  3890	            | TypedTx::CompleteSetMint(_)
  3891	            | TypedTx::CompleteSetRedeem(_)
  3892	            | TypedTx::MarketSeed(_)
  3893	            // Stage C P-M2 / Phase F.1 — agent-signed; admits through agent
  3894	            // ingress path identical to TB-13 conditional-share variants.
  3895	            | TypedTx::CompleteSetMerge(_)
  3896	            // Stage C P-M4 / Phase F.3 — agent-signed (provider); admits
  3897	            // through identical agent ingress path. Pool creation is an
  3898	            // economic mutator (debits provider's YES + NO inventory; credits
  3899	            // pool reserves + provider LP shares) and is therefore subject
  3900	            // to all the same admission gates as TB-13 / P-M2.
  3901	            | TypedTx::CpmmPool(_)
  3902	            // Stage C P-M5 / Phase F.4 — agent-signed (trader); admits
  3903	            // through identical agent ingress path. Pure share rotation
  3904	            // between trader and pool reserves; no Coin movement; subject
  3905	            // to the same admission gates as P-M4 (manifest-when-set
  3906	            // signature gate; replay-time Gate 4 fallback).
  3907	            | TypedTx::CpmmSwap(_)
  3908	            // Stage C P-M6 / Phase F.5 — agent-signed (buyer); admits
  3909	            // through identical agent ingress path. 9-step composite
  3910	            // Mint-and-Swap router: Coin payment → collateral lock + YES/NO
  3911	            // mint → swap retains buyer's preferred side; subject to the
  3912	            // same admission gates as sibling agent-signed variants.
  3913	            | TypedTx::BuyWithCoinRouter(_) => {}
  3914	        }
  3915	        // TRACE_MATRIX TB-13 Atom 6 round-3 (Codex VETO TB13-AUTH 2026-05-03):
  3916	        // submit-time agent-signature verification for the 3 TB-13
  3917	        // conditional-share variants. Opt-in via `set_agent_pubkeys` —
  3918	        // when the manifest is set, forged or unregistered signatures
  3919	        // are rejected pre-queue with `SubmitError::AgentSignatureInvalid`.
  3920	        // When the manifest is absent (default), this gate is bypassed
  3921	        // and replay-time `verify.rs` Gate 4 is the only line of defense
  3922	        // (see OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md).
  3923	        if let Some(manifest) = self.agent_pubkeys.get() {
  3924	            use crate::runtime::agent_keypairs::verify_agent_signature;
  3925	            match &tx {
  3926	                TypedTx::CompleteSetMint(mint) => {
  3927	                    let pubkey = manifest
  3928	                        .get(&mint.owner)
  3929	                        .ok_or(SubmitError::AgentSignatureInvalid)?;
  3930	                    let digest = mint.to_signing_payload().canonical_digest();
  3931	                    if verify_agent_signature(&mint.signature, &digest, &pubkey).is_err() {
  3932	                        return Err(SubmitError::AgentSignatureInvalid);
  3933	                    }
  3934	                }
  3935	                TypedTx::CompleteSetRedeem(redeem) => {
  3936	                    let pubkey = manifest
  3937	                        .get(&redeem.owner)
  3938	                        .ok_or(SubmitError::AgentSignatureInvalid)?;
  3939	                    let digest = redeem.to_signing_payload().canonical_digest();
  3940	                    if verify_agent_signature(&redeem.signature, &digest, &pubkey).is_err() {
  3941	                        return Err(SubmitError::AgentSignatureInvalid);
  3942	                    }
  3943	                }
  3944	                TypedTx::MarketSeed(seed) => {
  3945	                    let pubkey = manifest

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/chain_runtime.rs | sed -n '370,460p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   370	/// the same `bundle` reference passed here (i.e. `bus` and `bundle` come from
   371	/// the same `SharedChain::from_env` call). `seed_id` is the chain-level
   372	/// identifier used for the synthetic task_id (`smoke-{seed_id}`) and inline
   373	/// run_id metadata. In single-task mode (`evaluator.rs::run_swarm`),
   374	/// `seed_id == run_id` to preserve the exact tx_id digests the M0 retry
   375	/// chain audit observed. In Phase 4 multi-task mode
   376	/// (`comprehensive_arena.rs`), `seed_id` is a chain-level UUID minted at
   377	/// chain start.
   378	///
   379	/// **Pre-condition**: caller has already:
   380	///   1. Constructed `SharedChain::from_env(...)` (Phase 1)
   381	///   2. Run any per-task preseed + arena-hook env-var processing (TB-7.7 D3
   382	///      preseed + FORCE_* hooks; currently inline at evaluator.rs lines
   383	///      706-1437; will move to per-task body in Phase 3 / drive_task)
   384	///
   385	/// **Post-condition**: chain contains 1 synthetic L4 (TaskOpen) + 1 synthetic
   386	/// L4.E (zero-stake WorkTx; `synthetic_rejection_for_l4e_gate=true` label);
   387	/// `<runtime_repo>/synthetic_rejection_label.json` exists; agent_audit_trail
   388	/// records pair written to CAS + jsonl index; `<runtime_repo>/genesis_report.json`
   389	/// written.
   390	///
   391	/// **Failure modes**: all preserve original inline-code behavior:
   392	///   - synthetic TaskOpen submit fail → `error!()` log, continues
   393	///   - synthetic WorkTx submit fail → `error!()` log, continues
   394	///   - audit_trail write fail → `error!()` log, continues
   395	///   - genesis_report write fail → `warn!()` log, continues (per TB-7R
   396	///     Deliverable C "non-fatal — evidence collection continues, but
   397	///     post-hoc audit must note absence")
   398	///
   399	/// `FC-trace: FC1-N34` — synthetic L4.E gate is an audit_tape input that the
   400	/// post-hoc verifier separates from natural rejections via the
   401	/// `synthetic_rejection_for_l4e_gate=true` label.
   402	pub async fn write_synthetic_l4_l4e_gate_and_genesis_report(
   403	    bus: &mut TuringBus,
   404	    bundle: &ChaintapeBundle,
   405	    initial_balances: &[(String, i64)],
   406	    chaintape_preseed_enabled: bool,
   407	    seed_id: &str,
   408	) {
   409	    let task_id_str = format!("smoke-{}", seed_id);
   410	    let task_open = turingosv4::runtime::adapter::make_synthetic_task_open(
   411	        &task_id_str,
   412	        "tb6-smoke-sponsor",
   413	        turingosv4::state::q_state::Hash::ZERO,
   414	        "atom3-seed",
   415	    );
   416	    let task_open_tx_id =
   417	        turingosv4::state::q_state::TxId(format!("taskopen-{}-atom3-seed", task_id_str));
   418	    if let Err(e) = bus.submit_typed_tx(task_open).await {
   419	        error!("[chaintape] synthetic TaskOpen submit failed: {e}");
   420	    } else {
   421	        info!("[chaintape] seeded synthetic TaskOpen for {}", task_id_str);
   422	    }
   423	    let bad_worktx = turingosv4::runtime::adapter::make_synthetic_worktx(
   424	        &task_id_str,
   425	        "tb6-smoke-agent",
   426	        turingosv4::state::q_state::Hash::ZERO,
   427	        0,
   428	        "atom3-l4e-synthetic-rejection",
   429	        true,
   430	    );
   431	    let bad_worktx_tx_id = turingosv4::state::q_state::TxId(format!(
   432	        "worktx-{}-atom3-l4e-synthetic-rejection",
   433	        task_id_str
   434	    ));
   435	    if let Err(e) = bus.submit_typed_tx(bad_worktx).await {
   436	        error!("[chaintape] synthetic zero-stake WorkTx submit failed: {e}");
   437	    } else {
   438	        info!(
   439	            "[chaintape] seeded synthetic zero-stake WorkTx \
   440	             (synthetic_rejection_for_l4e_gate=true) for {}",
   441	            task_id_str
   442	        );
   443	    }
   444	    // Mark the synthetic-seed in the evidence dir so verify_chaintape (Atom 4)
   445	    // can distinguish synthetic-rejection from natural rejection.
   446	    let label_path = bundle.runtime_repo_path.join("synthetic_rejection_label.json");
   447	    let _ = std::fs::write(
   448	        &label_path,
   449	        format!(
   450	            r#"{{"synthetic_rejection_for_l4e_gate": true, "run_id": "{}", "atom": "TB-6 Atom 3", "rationale": "≥1 L4.E entry seeded via zero-stake WorkTx; per architect ruling 2026-05-01 § 3.6 Atom 3"}}"#,
   451	            seed_id
   452	        ),
   453	    );
   454	
   455	    // TB-6 Atom 5: write AgentProposalRecord pairs to CAS + index for both
   456	    // synthetic envelopes. Each record carries the architect's 9 fields
   457	    // + logical_t. The index links L4 / L4.E tx_id → CAS record CID.
   458	    if let Err(e) = turingosv4::runtime::agent_audit_trail::write_synthetic_seed_audit_pair(
   459	        &bundle.cas_path,
   460	        &bundle.runtime_repo_path,

exec
/bin/bash -lc "rg -n \"set_agent_pubkeys|agent_keypairs\\.lock|agent_keypairs\" experiments/minif2f_v4/src src -g '*.rs'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/src/bin/comprehensive_arena.rs:854:/// Helper: borrow agent_keypairs registry under mutex; pass `&mut reg` to closure.
experiments/minif2f_v4/src/bin/comprehensive_arena.rs:858:        &mut turingosv4::runtime::agent_keypairs::AgentKeypairRegistry,
experiments/minif2f_v4/src/bin/comprehensive_arena.rs:862:        .agent_keypairs
experiments/minif2f_v4/src/bin/comprehensive_arena.rs:864:        .ok_or_else(|| "agent_keypairs = None (chaintape mode required)".to_string())?
experiments/minif2f_v4/src/bin/comprehensive_arena.rs:866:    let mut reg = arc.lock().map_err(|_| "agent_keypairs mutex poisoned".to_string())?;
experiments/minif2f_v4/src/chain_runtime.rs:56:use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
experiments/minif2f_v4/src/chain_runtime.rs:83:    pub agent_keypairs: Option<Arc<Mutex<AgentKeypairRegistry>>>,
experiments/minif2f_v4/src/chain_runtime.rs:231:        let agent_keypairs: Option<Arc<Mutex<AgentKeypairRegistry>>> =
experiments/minif2f_v4/src/chain_runtime.rs:273:                        "[chaintape/tb9-resume] agent_keypairs resume must succeed \
experiments/minif2f_v4/src/chain_runtime.rs:292:                        "[chaintape/tb9] agent_keypairs durable init must succeed (fresh runtime_repo guarantees \
experiments/minif2f_v4/src/chain_runtime.rs:356:            agent_keypairs,
experiments/minif2f_v4/src/chain_runtime.rs:540:    /// a SharedChain with `chaintape_bundle = None`, `agent_keypairs = None`,
experiments/minif2f_v4/src/chain_runtime.rs:573:        assert!(chain.agent_keypairs.is_none());
experiments/minif2f_v4/src/bin/evaluator.rs:210:    reg: &std::sync::Arc<std::sync::Mutex<turingosv4::runtime::agent_keypairs::AgentKeypairRegistry>>,
experiments/minif2f_v4/src/bin/evaluator.rs:862:        agent_keypairs,
experiments/minif2f_v4/src/bin/evaluator.rs:917:                let registry_arc = agent_keypairs
experiments/minif2f_v4/src/bin/evaluator.rs:919:                    .expect("[chaintape/tb10] agent_keypairs registry required for user-mode signing");
experiments/minif2f_v4/src/bin/evaluator.rs:920:                let mut reg = registry_arc.lock().expect("agent_keypairs registry mutex poisoned");
experiments/minif2f_v4/src/bin/evaluator.rs:988:                let registry_arc = agent_keypairs
experiments/minif2f_v4/src/bin/evaluator.rs:990:                    .expect("[chaintape/tb10] agent_keypairs registry required for user-mode signing");
experiments/minif2f_v4/src/bin/evaluator.rs:991:                let mut reg = registry_arc.lock().expect("agent_keypairs registry mutex poisoned");
experiments/minif2f_v4/src/bin/evaluator.rs:1046:                        let registry_arc = agent_keypairs.as_ref()
experiments/minif2f_v4/src/bin/evaluator.rs:1047:                            .expect("[chaintape/tb16-arena] agent_keypairs registry required");
experiments/minif2f_v4/src/bin/evaluator.rs:1048:                        let mut reg_guard = registry_arc.lock().expect("agent_keypairs registry mutex poisoned");
experiments/minif2f_v4/src/bin/evaluator.rs:1087:                        let registry_arc = agent_keypairs.as_ref()
experiments/minif2f_v4/src/bin/evaluator.rs:1088:                            .expect("[chaintape/tb16-arena] agent_keypairs registry required");
experiments/minif2f_v4/src/bin/evaluator.rs:1089:                        let mut reg_guard = registry_arc.lock().expect("agent_keypairs registry mutex poisoned");
experiments/minif2f_v4/src/bin/evaluator.rs:1236:                                let registry_arc = agent_keypairs
experiments/minif2f_v4/src/bin/evaluator.rs:1238:                                    .expect("[chaintape/tb16-arena] agent_keypairs registry required for FORCE_BANKRUPTCY_AFTER_ACCEPTED");
experiments/minif2f_v4/src/bin/evaluator.rs:1241:                                    .expect("agent_keypairs registry mutex poisoned");
experiments/minif2f_v4/src/bin/evaluator.rs:1538:                                let registry_arc = agent_keypairs
experiments/minif2f_v4/src/bin/evaluator.rs:1540:                                    .expect("[chaintape/tb16-arena] agent_keypairs registry required for FORCE_BOLTZMANN_SEED_WORKTXS");
experiments/minif2f_v4/src/bin/evaluator.rs:1543:                                    .expect("agent_keypairs registry mutex poisoned");
experiments/minif2f_v4/src/bin/evaluator.rs:2316:                                    (chaintape_bundle.as_ref(), agent_keypairs.as_ref())
experiments/minif2f_v4/src/bin/evaluator.rs:2593:                                            (chaintape_bundle.as_ref(), agent_keypairs.as_ref())
experiments/minif2f_v4/src/bin/evaluator.rs:3146:                                (chaintape_bundle.as_ref(), agent_keypairs.as_ref())
experiments/minif2f_v4/src/bin/evaluator.rs:3327:                                (chaintape_bundle.as_ref(), agent_keypairs.as_ref())
experiments/minif2f_v4/src/bin/evaluator.rs:3485:                                            (chaintape_bundle.as_ref(), agent_keypairs.as_ref())
experiments/minif2f_v4/src/bin/evaluator.rs:4039:                                            (chaintape_bundle.as_ref(), agent_keypairs.as_ref())
experiments/minif2f_v4/src/bin/evaluator.rs:4133:                            (chaintape_bundle.as_ref(), agent_keypairs.as_ref())
experiments/minif2f_v4/src/bin/evaluator.rs:4199:                    (chaintape_bundle.as_ref(), agent_keypairs.as_ref())
experiments/minif2f_v4/src/bin/evaluator.rs:4631:                            let registry_arc = agent_keypairs
experiments/minif2f_v4/src/bin/evaluator.rs:4633:                                .expect("[chaintape/tb16-arena] agent_keypairs registry required for FORCE_REDEEM");
experiments/minif2f_v4/src/bin/evaluator.rs:4636:                                .expect("agent_keypairs registry mutex poisoned");
src/bin/audit_dashboard.rs:9://! run_summary + agent_keypairs + agent_audit_trail) — does NOT
src/bin/audit_dashboard.rs:41:use turingosv4::runtime::agent_keypairs::AgentPubkeyManifest;
experiments/minif2f_v4/src/drive_task.rs:153:/// includes a populated `agent_keypairs` registry.
experiments/minif2f_v4/src/drive_task.rs:164:/// - `chain.agent_keypairs = None` →
experiments/minif2f_v4/src/drive_task.rs:191:        .agent_keypairs
experiments/minif2f_v4/src/drive_task.rs:209:                detail: "agent_keypairs registry mutex poisoned".into(),
experiments/minif2f_v4/src/drive_task.rs:246:                detail: "agent_keypairs registry mutex poisoned".into(),
src/runtime/audit_assertions.rs:78:use crate::runtime::agent_keypairs::AgentPubkeyManifest;
src/runtime/adapter.rs:23:use crate::runtime::agent_keypairs::{AgentKeypairError, AgentKeypairRegistry};
src/runtime/adapter.rs:1457:        use crate::runtime::agent_keypairs::{verify_agent_signature, AgentPubkeyManifest};
src/state/sequencer.rs:3710:    /// **TB-13 enforcement**: when set via [`Sequencer::set_agent_pubkeys`],
src/state/sequencer.rs:3715:    agent_pubkeys: std::sync::OnceLock<Arc<crate::runtime::agent_keypairs::AgentPubkeyManifest>>,
src/state/sequencer.rs:3825:    pub fn set_agent_pubkeys(
src/state/sequencer.rs:3827:        manifest: Arc<crate::runtime::agent_keypairs::AgentPubkeyManifest>,
src/state/sequencer.rs:3828:    ) -> Result<(), Arc<crate::runtime::agent_keypairs::AgentPubkeyManifest>> {
src/state/sequencer.rs:3917:        // conditional-share variants. Opt-in via `set_agent_pubkeys` —
src/state/sequencer.rs:3924:            use crate::runtime::agent_keypairs::verify_agent_signature;
src/runtime/chain_derived_run_facts.rs:1079:    use crate::runtime::agent_keypairs::AgentKeypairRegistry;
src/runtime/chain_derived_run_facts.rs:1138:            AgentKeypairRegistry::open(&cfg.runtime_repo_path).expect("open agent_keypairs");
src/runtime/verify.rs:377:    use crate::runtime::agent_keypairs::{verify_agent_signature, AgentPubkeyManifest};
src/runtime/mod.rs:33:pub mod agent_keypairs;

exec
/bin/bash -lc "rg -n \"set_agent_pubkeys|manifest\\(\\)|agent_pubkeys\" experiments/minif2f_v4/src/bin/evaluator.rs experiments/minif2f_v4/src/bin/comprehensive_arena.rs experiments/minif2f_v4/src/drive_task.rs" in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
exec
/bin/bash -lc "nl -ba src/runtime/agent_keystore.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-9 Atom 1 — Durable agent keystore: encrypted-at-rest persistence of
     2	//! per-agent Ed25519 secrets across evaluator restarts.
     3	//!
     4	//! Mirrors `src/bottom_white/ledger/system_keypair.rs:417-463` exactly:
     5	//! - default path `~/.turingos/keystore/agent_keystore.enc` (env override
     6	//!   `TURINGOS_AGENT_KEYSTORE_PATH`)
     7	//! - Argon2id KDF (m=64MiB, t=3, p=4 default; env-tunable via the same
     8	//!   `TURINGOS_KDF_*` knobs used for the system keystore)
     9	//! - ChaCha20-Poly1305 AEAD encryption-at-rest
    10	//! - atomic write 0600 (mode bits enforced on Unix)
    11	//! - format magic `TOS4AGTKEY1` (distinct from system's `TOS4SYSKEY1`)
    12	//!
    13	//! Plaintext shape: bincode-encoded `BTreeMap<String, [u8; 32]>` mapping
    14	//! `AgentId.0` → 32-byte Ed25519 secret seed. Public keys are NOT stored —
    15	//! they are recomputed from each seed at load time.
    16	//!
    17	//! TRACE_MATRIX FC1-N14 (durable agent identity primitive; satisfies
    18	//! architect TB-9 mandate "agent durable key registry" + "cross-run
    19	//! identity").
    20	
    21	use argon2::{Algorithm, Argon2, Params, Version};
    22	use chacha20poly1305::aead::{Aead, KeyInit};
    23	use chacha20poly1305::{ChaCha20Poly1305, Nonce};
    24	use secrecy::{ExposeSecret, SecretString};
    25	use std::collections::BTreeMap;
    26	use std::env;
    27	use std::fmt;
    28	use std::fs::{self, OpenOptions};
    29	use std::io::Write;
    30	use std::path::{Path, PathBuf};
    31	use zeroize::Zeroize;
    32	
    33	const DEFAULT_KDF_MEMORY_KIB: u32 = 65_536;
    34	const DEFAULT_KDF_ITER: u32 = 3;
    35	const DEFAULT_KDF_LANES: u32 = 4;
    36	const DERIVED_KEY_LEN: usize = 32;
    37	const SECRET_KEY_LEN: usize = 32;
    38	const SALT_LEN: usize = 16;
    39	const NONCE_LEN: usize = 12;
    40	const FORMAT_MAGIC: &[u8; 11] = b"TOS4AGTKEY1";
    41	const FORMAT_VERSION: u8 = 1;
    42	
    43	/// TRACE_MATRIX FC1-N14: agent keystore lifecycle / crypto error taxonomy.
    44	#[derive(Debug)]
    45	pub enum AgentKeystoreError {
    46	    Io(std::io::Error),
    47	    Entropy(getrandom::Error),
    48	    KdfParam(String),
    49	    Kdf(argon2::Error),
    50	    Crypto(&'static str),
    51	    InvalidFormat(&'static str),
    52	    HomeUnavailable,
    53	    Bincode(String),
    54	}
    55	
    56	impl fmt::Display for AgentKeystoreError {
    57	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    58	        match self {
    59	            Self::Io(e) => write!(f, "agent keystore I/O failed: {e}"),
    60	            Self::Entropy(e) => write!(f, "agent keystore entropy failed: {e}"),
    61	            Self::KdfParam(msg) => write!(f, "agent keystore KDF parameter invalid: {msg}"),
    62	            Self::Kdf(e) => write!(f, "agent keystore KDF failed: {e}"),
    63	            Self::Crypto(msg) => write!(f, "agent keystore crypto failed: {msg}"),
    64	            Self::InvalidFormat(msg) => write!(f, "agent keystore format invalid: {msg}"),
    65	            Self::HomeUnavailable => write!(f, "agent keystore default path requires HOME"),
    66	            Self::Bincode(msg) => write!(f, "agent keystore serde failed: {msg}"),
    67	        }
    68	    }
    69	}
    70	
    71	impl std::error::Error for AgentKeystoreError {}
    72	
    73	impl From<std::io::Error> for AgentKeystoreError {
    74	    fn from(e: std::io::Error) -> Self {
    75	        Self::Io(e)
    76	    }
    77	}
    78	
    79	/// TB-9 Atom 2 helper: read the durable keystore password from env (with a
    80	/// hardcoded local-dev fallback, acceptable for solo-runs per
    81	/// `feedback_kolmogorov_compression`). Wraps the result in `SecretString` so
    82	/// callers (binaries) don't need to depend on `secrecy` directly.
    83	pub fn keystore_password_from_env() -> SecretString {
    84	    let raw = env::var("TURINGOS_AGENT_KEYSTORE_PASSWORD")
    85	        .unwrap_or_else(|_| "tb9-local-dev-password-replace-in-production".to_string());
    86	    SecretString::new(raw.into())
    87	}
    88	
    89	/// TRACE_MATRIX FC1-N14: resolve `~/.turingos/keystore/agent_keystore.enc`.
    90	///
    91	/// `TURINGOS_AGENT_KEYSTORE_PATH` overrides. Default never points into the
    92	/// repository, CAS, or runtime_repo directories.
    93	pub fn default_agent_keystore_path() -> Result<PathBuf, AgentKeystoreError> {
    94	    if let Ok(path) = env::var("TURINGOS_AGENT_KEYSTORE_PATH") {
    95	        return Ok(PathBuf::from(path));
    96	    }
    97	    let home = env::var("HOME").map_err(|_| AgentKeystoreError::HomeUnavailable)?;
    98	    Ok(PathBuf::from(home)
    99	        .join(".turingos")
   100	        .join("keystore")
   101	        .join("agent_keystore.enc"))
   102	}
   103	
   104	/// TRACE_MATRIX FC1-N14: load durable keystore from disk if present, else
   105	/// return an empty map. Returns `(secrets, fresh)` where `fresh=true` if the
   106	/// path did not exist at call time.
   107	pub fn load_or_empty(
   108	    keystore_path: &Path,
   109	    password: &SecretString,
   110	) -> Result<(BTreeMap<String, [u8; SECRET_KEY_LEN]>, bool), AgentKeystoreError> {
   111	    if !keystore_path.exists() {
   112	        return Ok((BTreeMap::new(), true));
   113	    }
   114	    let bytes = fs::read(keystore_path)?;
   115	    let encoded = EncryptedBundle::decode(&bytes)?;
   116	    let mut key = derive_key(password, &encoded.salt, encoded.kdf)?;
   117	    let cipher = ChaCha20Poly1305::new_from_slice(&key)
   118	        .map_err(|_| AgentKeystoreError::Crypto("bad cipher key"))?;
   119	    let plaintext = cipher
   120	        .decrypt(
   121	            Nonce::from_slice(&encoded.nonce),
   122	            encoded.ciphertext.as_ref(),
   123	        )
   124	        .map_err(|_| AgentKeystoreError::Crypto("keystore authentication failed"))?;
   125	    key.zeroize();
   126	    let cfg = bincode::config::standard()
   127	        .with_big_endian()
   128	        .with_fixed_int_encoding();
   129	    let (secrets, consumed): (BTreeMap<String, [u8; SECRET_KEY_LEN]>, usize) =
   130	        bincode::serde::decode_from_slice(&plaintext, cfg)
   131	            .map_err(|e| AgentKeystoreError::Bincode(e.to_string()))?;
   132	    if consumed != plaintext.len() {
   133	        return Err(AgentKeystoreError::Bincode(format!(
   134	            "trailing bytes: consumed {consumed} of {}",
   135	            plaintext.len()
   136	        )));
   137	    }
   138	    Ok((secrets, false))
   139	}
   140	
   141	/// TRACE_MATRIX FC1-N14: encrypt + atomic-write the durable keystore.
   142	pub fn save(
   143	    keystore_path: &Path,
   144	    password: &SecretString,
   145	    secrets: &BTreeMap<String, [u8; SECRET_KEY_LEN]>,
   146	) -> Result<(), AgentKeystoreError> {
   147	    let kdf = KdfParams::from_env()?;
   148	    let mut salt = [0u8; SALT_LEN];
   149	    let mut nonce = [0u8; NONCE_LEN];
   150	    getrandom::getrandom(&mut salt).map_err(AgentKeystoreError::Entropy)?;
   151	    getrandom::getrandom(&mut nonce).map_err(AgentKeystoreError::Entropy)?;
   152	
   153	    let mut key = derive_key(password, &salt, kdf)?;
   154	    let cipher = ChaCha20Poly1305::new_from_slice(&key)
   155	        .map_err(|_| AgentKeystoreError::Crypto("bad cipher key"))?;
   156	    let cfg = bincode::config::standard()
   157	        .with_big_endian()
   158	        .with_fixed_int_encoding();
   159	    let mut plaintext = bincode::serde::encode_to_vec(secrets, cfg)
   160	        .map_err(|e| AgentKeystoreError::Bincode(e.to_string()))?;
   161	    let ciphertext = cipher
   162	        .encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
   163	        .map_err(|_| AgentKeystoreError::Crypto("keystore encryption failed"))?;
   164	    plaintext.zeroize();
   165	    key.zeroize();
   166	
   167	    let bundle = EncryptedBundle {
   168	        kdf,
   169	        salt,
   170	        nonce,
   171	        ciphertext,
   172	    }
   173	    .encode()?;
   174	
   175	    write_keystore_0600_atomic(keystore_path, &bundle)?;
   176	    Ok(())
   177	}
   178	
   179	fn derive_key(
   180	    password: &SecretString,
   181	    salt: &[u8; SALT_LEN],
   182	    kdf: KdfParams,
   183	) -> Result<[u8; DERIVED_KEY_LEN], AgentKeystoreError> {
   184	    let params = Params::new(
   185	        kdf.memory_kib,
   186	        kdf.iterations,
   187	        kdf.lanes,
   188	        Some(DERIVED_KEY_LEN),
   189	    )
   190	    .map_err(|err| AgentKeystoreError::KdfParam(err.to_string()))?;
   191	    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
   192	    let mut key = [0u8; DERIVED_KEY_LEN];
   193	    argon2
   194	        .hash_password_into(password.expose_secret().as_bytes(), salt, &mut key)
   195	        .map_err(AgentKeystoreError::Kdf)?;
   196	    Ok(key)
   197	}
   198	
   199	fn write_keystore_0600_atomic(path: &Path, bytes: &[u8]) -> Result<(), AgentKeystoreError> {
   200	    if let Some(parent) = path.parent() {
   201	        fs::create_dir_all(parent)?;
   202	    }
   203	    let tmp = path.with_extension("enc.tmp");
   204	    {
   205	        let mut options = OpenOptions::new();
   206	        options.write(true).create(true).truncate(true);
   207	        set_open_options_mode_0600(&mut options);
   208	        let mut file = options.open(&tmp)?;
   209	        file.write_all(bytes)?;
   210	        file.sync_all()?;
   211	    }
   212	    fs::rename(&tmp, path)?;
   213	    set_file_permissions_0600(path)?;
   214	    Ok(())
   215	}
   216	
   217	#[cfg(unix)]
   218	fn set_open_options_mode_0600(options: &mut OpenOptions) {
   219	    use std::os::unix::fs::OpenOptionsExt;
   220	    options.mode(0o600);
   221	}
   222	
   223	#[cfg(not(unix))]
   224	fn set_open_options_mode_0600(_options: &mut OpenOptions) {}
   225	
   226	#[cfg(unix)]
   227	fn set_file_permissions_0600(path: &Path) -> Result<(), AgentKeystoreError> {
   228	    use std::os::unix::fs::PermissionsExt;
   229	    fs::set_permissions(path, fs::Permissions::from_mode(0o600)).map_err(AgentKeystoreError::Io)
   230	}
   231	
   232	#[cfg(not(unix))]
   233	fn set_file_permissions_0600(_path: &Path) -> Result<(), AgentKeystoreError> {
   234	    Ok(())
   235	}
   236	
   237	#[derive(Debug, Clone, Copy)]
   238	struct KdfParams {
   239	    memory_kib: u32,
   240	    iterations: u32,
   241	    lanes: u32,
   242	}
   243	
   244	impl KdfParams {
   245	    fn from_env() -> Result<Self, AgentKeystoreError> {
   246	        Ok(Self {
   247	            memory_kib: read_env_u32("TURINGOS_KDF_MEMORY_KIB", DEFAULT_KDF_MEMORY_KIB)?,
   248	            iterations: read_env_u32("TURINGOS_KDF_ITER", DEFAULT_KDF_ITER)?,
   249	            lanes: read_env_u32("TURINGOS_KDF_LANES", DEFAULT_KDF_LANES)?,
   250	        })
   251	    }
   252	}
   253	
   254	fn read_env_u32(name: &str, default: u32) -> Result<u32, AgentKeystoreError> {
   255	    match env::var(name) {
   256	        Ok(value) => {
   257	            let parsed = value
   258	                .parse::<u32>()
   259	                .map_err(|_| AgentKeystoreError::KdfParam(format!("{name} must be u32")))?;
   260	            if parsed == 0 {

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/drive_task.rs | sed -n '150,265p' && nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '900,1010p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   150	///
   151	/// **Pre-condition**: `chain` was constructed via
   152	/// `SharedChain::from_env(...)` and (when chaintape mode is enabled)
   153	/// includes a populated `agent_keypairs` registry.
   154	///
   155	/// **Post-condition** on success: chain contains 1 new accepted L4
   156	/// `TaskOpenTx` for `task-{theorem_name}` + 1 new accepted L4
   157	/// `EscrowLockTx` locking `spec.escrow_amount_micro` from
   158	/// `spec.sponsor_agent` into the task's escrow.
   159	///
   160	/// **Failure modes**:
   161	/// - `chain.chaintape_bundle = None` (legacy mode) →
   162	///   `Err(DriveTaskError::ChaintapeRequired)` (drive_task is chain-only;
   163	///   legacy WAL_DIR / in-memory paths are not supported).
   164	/// - `chain.agent_keypairs = None` →
   165	///   `Err(DriveTaskError::AgentKeypairsRequired)`.
   166	/// - Real-signature construction fails →
   167	///   `Err(DriveTaskError::SigningFailed { stage, source })`.
   168	/// - tx submit fails or commit-await budget expires →
   169	///   `Err(DriveTaskError::SubmitFailed { stage, .. })`.
   170	///
   171	/// `_budget` is currently unused by the task scaffolder (no LLM calls in
   172	/// this body); kept in the signature per architect-ratified atom A.1
   173	/// contract — Phase 3+ may thread it through if drive_task gains an
   174	/// optional inline LLM path (NOT in scope today).
   175	pub async fn drive_task(
   176	    chain: &mut SharedChain,
   177	    spec: &TaskSpec,
   178	    _budget: PerCallBudget,
   179	) -> Result<DriveTaskResult, DriveTaskError> {
   180	    use turingosv4::runtime::adapter::{
   181	        make_real_escrow_lock_signed_by, make_real_task_open_signed_by,
   182	        tb8_await_state_root_advance,
   183	    };
   184	    use turingosv4::state::q_state::Hash;
   185	
   186	    let bundle = chain
   187	        .chaintape_bundle
   188	        .as_ref()
   189	        .ok_or(DriveTaskError::ChaintapeRequired)?;
   190	    let keypairs_arc = chain
   191	        .agent_keypairs
   192	        .as_ref()
   193	        .ok_or(DriveTaskError::AgentKeypairsRequired)?
   194	        .clone();
   195	
   196	    let task_id_str = format!("task-{}", spec.theorem_name);
   197	    let pre_open_root = bundle
   198	        .sequencer
   199	        .q_snapshot()
   200	        .map(|q| q.state_root_t)
   201	        .unwrap_or(Hash::ZERO);
   202	
   203	    // Build + submit TaskOpen (real-signed by sponsor).
   204	    let task_open = {
   205	        let mut reg = keypairs_arc
   206	            .lock()
   207	            .map_err(|_| DriveTaskError::SigningFailed {
   208	                stage: "task_open_lock",
   209	                detail: "agent_keypairs registry mutex poisoned".into(),
   210	            })?;
   211	        make_real_task_open_signed_by(
   212	            &mut reg,
   213	            &task_id_str,
   214	            &spec.sponsor_agent,
   215	            pre_open_root,
   216	            "tb18-drive-task-open",
   217	            1,
   218	        )
   219	        .map_err(|e| DriveTaskError::SigningFailed {
   220	            stage: "task_open_sign",
   221	            detail: format!("{e:?}"),
   222	        })?
   223	    };
   224	    let task_open_tx_id_str = format!("taskopen-{}-tb18-drive-task-open", task_id_str);
   225	    chain
   226	        .bus
   227	        .submit_typed_tx(task_open)
   228	        .await
   229	        .map_err(|e| DriveTaskError::SubmitFailed {
   230	            stage: "task_open_submit",
   231	            detail: format!("{e:?}"),
   232	        })?;
   233	    let post_open_root = tb8_await_state_root_advance(bundle.sequencer.as_ref(), pre_open_root, 5000)
   234	        .await
   235	        .map_err(|_| DriveTaskError::SubmitFailed {
   236	            stage: "task_open_commit_await",
   237	            detail: "5s state_root advance budget expired".into(),
   238	        })?;
   239	
   240	    // Build + submit EscrowLock (real-signed by sponsor).
   241	    let escrow_lock = {
   242	        let mut reg = keypairs_arc
   243	            .lock()
   244	            .map_err(|_| DriveTaskError::SigningFailed {
   245	                stage: "escrow_lock_lock",
   246	                detail: "agent_keypairs registry mutex poisoned".into(),
   247	            })?;
   248	        make_real_escrow_lock_signed_by(
   249	            &mut reg,
   250	            &task_id_str,
   251	            &spec.sponsor_agent,
   252	            spec.escrow_amount_micro,
   253	            post_open_root,
   254	            "tb18-drive-escrow-lock",
   255	            2,
   256	        )
   257	        .map_err(|e| DriveTaskError::SigningFailed {
   258	            stage: "escrow_lock_sign",
   259	            detail: format!("{e:?}"),
   260	        })?
   261	    };
   262	    let escrow_lock_tx_id_str = format!("escrowlock-{}-tb18-drive-escrow-lock", task_id_str);
   263	    chain
   264	        .bus
   265	        .submit_typed_tx(escrow_lock)
   900	        // Combined with pre-seeded Agent_i balance, real LLM WorkTx
   901	        // with stake > 0 can now reach L4 accepted.
   902	        if chaintape_preseed_enabled {
   903	            let real_task_id = format!("task-{}", run_id);
   904	            // TB-10 Atom 1+3: when TURINGOS_USER_TASK_MODE=1 (or any value parsing
   905	            // truthy), the preseed sponsor swaps from tb7-7-sponsor → Agent_user_0
   906	            // and the TaskOpen+EscrowLock are signed with REAL Ed25519 via the
   907	            // durable keystore (TB-9 carry). Solver task_id remains task-{run_id}
   908	            // — user-mode is a sponsor swap only; the solver loop flows unchanged.
   909	            // Per TB-10 charter §3 Atom 3 + ratification §1 Q3.
   910	            let user_task_mode = std::env::var("TURINGOS_USER_TASK_MODE")
   911	                .ok()
   912	                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
   913	                .unwrap_or(false);
   914	            let user_sponsor = std::env::var("TURINGOS_USER_TASK_SPONSOR")
   915	                .unwrap_or_else(|_| "Agent_user_0".into());
   916	            let task_open_real = if user_task_mode {
   917	                let registry_arc = agent_keypairs
   918	                    .as_ref()
   919	                    .expect("[chaintape/tb10] agent_keypairs registry required for user-mode signing");
   920	                let mut reg = registry_arc.lock().expect("agent_keypairs registry mutex poisoned");
   921	                turingosv4::runtime::adapter::make_real_task_open_signed_by(
   922	                    &mut reg,
   923	                    &real_task_id,
   924	                    &user_sponsor,
   925	                    turingosv4::state::q_state::Hash::ZERO,
   926	                    "tb10-user-seed",
   927	                    1,
   928	                )
   929	                .expect("[chaintape/tb10] sign user-mode TaskOpen with Agent_user_0 keypair")
   930	            } else {
   931	                turingosv4::runtime::adapter::make_synthetic_task_open(
   932	                    &real_task_id,
   933	                    "tb7-7-sponsor",
   934	                    turingosv4::state::q_state::Hash::ZERO,
   935	                    "tb7-7-d3-seed",
   936	                )
   937	            };
   938	            if let Err(e) = bus.submit_typed_tx(task_open_real).await {
   939	                error!("[chaintape/d3] preseed TaskOpen submit failed: {e}");
   940	            } else if user_task_mode {
   941	                info!(
   942	                    "[chaintape/tb10] user-mode TaskOpen for {real_task_id} sponsor={user_sponsor}"
   943	                );
   944	            } else {
   945	                info!("[chaintape/d3] preseed TaskOpen for {real_task_id}");
   946	            }
   947	            // submit_typed_tx queues the tx and returns immediately; the
   948	            // Sequencer::run driver applies asynchronously (bus.rs:127-130).
   949	            // Poll q_snapshot until state_root_t advances past ZERO, then
   950	            // use the post-TaskOpen root as parent_state_root for the
   951	            // EscrowLock. Without this wait the EscrowLock would be
   952	            // rejected as StaleParent (lock.parent_state_root=ZERO !=
   953	            // q.state_root_t after TaskOpen applied).
   954	            let parent_for_escrow = {
   955	                use std::time::{Duration, Instant};
   956	                let deadline = Instant::now() + Duration::from_secs(5);
   957	                let mut root = turingosv4::state::q_state::Hash::ZERO;
   958	                while Instant::now() < deadline {
   959	                    if let Ok(q) = bundle.sequencer.q_snapshot() {
   960	                        if q.state_root_t != turingosv4::state::q_state::Hash::ZERO {
   961	                            root = q.state_root_t;
   962	                            break;
   963	                        }
   964	                    }
   965	                    tokio::time::sleep(Duration::from_millis(50)).await;
   966	                }
   967	                if root == turingosv4::state::q_state::Hash::ZERO {
   968	                    warn!(
   969	                        "[chaintape/d3] preseed TaskOpen did not advance state_root \
   970	                         within 5s; EscrowLock will use ZERO and likely reject"
   971	                    );
   972	                }
   973	                root
   974	            };
   975	            // Read escrow amount from env. TB-10 user-mode reads
   976	            // TURINGOS_USER_TASK_BOUNTY_MICRO first (user's bounty); fallback to
   977	            // existing TB-7.7 D3 envvar; final default 100_000 micro = 0.1 coin.
   978	            let escrow_micro: i64 = std::env::var("TURINGOS_USER_TASK_BOUNTY_MICRO")
   979	                .ok()
   980	                .and_then(|s| s.parse().ok())
   981	                .or_else(|| {
   982	                    std::env::var("TURINGOS_CHAINTAPE_PRESEED_TASK_ESCROW_MICRO")
   983	                        .ok()
   984	                        .and_then(|s| s.parse().ok())
   985	                })
   986	                .unwrap_or(100_000);
   987	            let escrow_lock = if user_task_mode {
   988	                let registry_arc = agent_keypairs
   989	                    .as_ref()
   990	                    .expect("[chaintape/tb10] agent_keypairs registry required for user-mode signing");
   991	                let mut reg = registry_arc.lock().expect("agent_keypairs registry mutex poisoned");
   992	                turingosv4::runtime::adapter::make_real_escrow_lock_signed_by(
   993	                    &mut reg,
   994	                    &real_task_id,
   995	                    &user_sponsor,
   996	                    escrow_micro,
   997	                    parent_for_escrow,
   998	                    "tb10-user-escrow",
   999	                    2,
  1000	                )
  1001	                .expect("[chaintape/tb10] sign user-mode EscrowLock with Agent_user_0 keypair")
  1002	            } else {
  1003	                turingosv4::runtime::adapter::make_synthetic_escrow_lock(
  1004	                    &real_task_id,
  1005	                    "tb7-7-sponsor",
  1006	                    escrow_micro,
  1007	                    parent_for_escrow,
  1008	                    "tb7-7-d3-escrow",
  1009	                )
  1010	            };

codex
```
Q1: PASS — code checks manifest-listed agent missing-secret and pubkey mismatch.
Q2: PASS — env=1 now routes to resume_existing_durable and missing manifest fails closed.
Q3: PASS — binary predicate is env=1 only.
Q4: PASS — env!=1 still uses generate_or_load_durable; fresh path unchanged.
Q5: CHALLENGE — tampered-pubkey branch exists, but is not binding-tested.
Q6: PASS — sha256sum matches genesis_payload 4dc7de08.
Q7: CHALLENGE — SG-G1.6/G1.7 are GREEN, but pubkey-mismatch fail-closed path lacks CI proof.
Q8: PASS — no sequencer arm, schema, or canonical signing payload edit observed.
Q9: CHALLENGE — END-TO-END claim is overstated until pubkey-mismatch coverage is bound.

Aggregate R2 verdict: CHALLENGE
Conviction: high
Recommendation: HALT — escalate to /harness-reflect
```
tokens used
102,584
```
Q1: PASS — code checks manifest-listed agent missing-secret and pubkey mismatch.
Q2: PASS — env=1 now routes to resume_existing_durable and missing manifest fails closed.
Q3: PASS — binary predicate is env=1 only.
Q4: PASS — env!=1 still uses generate_or_load_durable; fresh path unchanged.
Q5: CHALLENGE — tampered-pubkey branch exists, but is not binding-tested.
Q6: PASS — sha256sum matches genesis_payload 4dc7de08.
Q7: CHALLENGE — SG-G1.6/G1.7 are GREEN, but pubkey-mismatch fail-closed path lacks CI proof.
Q8: PASS — no sequencer arm, schema, or canonical signing payload edit observed.
Q9: CHALLENGE — END-TO-END claim is overstated until pubkey-mismatch coverage is bound.

Aggregate R2 verdict: CHALLENGE
Conviction: high
Recommendation: HALT — escalate to /harness-reflect
```
