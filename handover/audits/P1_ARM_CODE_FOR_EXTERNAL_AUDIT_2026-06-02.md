# P1 — the THREE arms' actual executing code (verbatim, for external audit)

**Commit (the binary the experiment ran):** `6598bf17f0a847761bbeff618c0a5f3bc1b379fa` (branch claude/p1-realvalue).
**Verify any block below independently:** `git show 6598bf17f0a847761bbeff618c0a5f3bc1b379fas` and `git show 6598bf17f0a847761bbeff618c0a5f3bc1b379fas`.
Every code block is extracted **verbatim** from those two files at this commit (file:line shown).

## What differs between the 3 arms (and what does NOT)
The arms are ONE binary, ONE loop, ONE real-Lean verifier. They differ in **exactly three places**:
1. **how the parent node is chosen** (`select_parent` + the autonomous in-call choice),
2. **the prompt** (`build_prompt` for market/single vs `build_autonomous_prompt` for autonomous),
3. **single's budget compensation** (so all arms make the SAME number of proposals).
Everything else — the real Lean kernel verdict (`judge.verify`), node recording, the loss-bearing
price (WorkTx-Long + Bear ChallengeTx-Short), the librarian, the ChainTape, replay — is **byte-identical
shared code** executed by all arms. Proof of that is the single shared loop in §1.

- **single** = one agent refining its OWN last node (a single chain), at the SAME total proposal budget.
- **market** = the harness FORCES the parent by a true Boltzmann softmax over the live loss-bearing price.
- **autonomous** = the agent itself READS the full landscape and FREELY picks which node to extend/branch.

---
## §1. The shared run loop — where the 3 arms branch at runtime
`src/bin/lean_market_agent.rs:724-811` — note the only per-arm branches: `select_parent(...)` (line 733), the prompt `if` (743-747), and the autonomous parent-parse (788-802). The LLM call (748-754), the real Lean verdict (805), and everything after are identical for all arms.
```rust
    'outer: for round in 0..effective_rounds {
        for ai in 0..agents.len() {
            let agent = agents[ai].clone();
            let q = seq.q_snapshot().map_err(|e| format!("{e:?}"))?;
            root = q.state_root_t;
            let pi = compute_price_index(&q.economic_state_t);

            // Parent selection (policy-governed).
            let mut rng = StdRng::seed_from_u64(args.seed + round as u64 * 131 + ai as u64);
            let parent_tx = select_parent(args.policy, &pi, &node_tx_ids, own_last.get(&agent), &node_conf, &node_doubt, args.boltzmann_temp, &mut rng);
            let (parent_body, parent_feedback) = match &parent_tx {
                Some(t) => (node_body.get(&t.0).cloned(), node_feedback.get(&t.0).cloned()),
                None => (None, None),
            };

            // REAL librarian: shielded collective failure memory derived from the typed
            // LeanResult sidecars written into CAS on prior attempts (all agents). `lt` is the
            // run's monotonic logical clock → meaningful staleness; the problem id is the scope tag.
            let lib = real_librarian_solver_notice(&args.cas, lt, &args.problem);
            let prompt = if args.policy == Policy::Autonomous {
                build_autonomous_prompt(&theorem, &node_tx_ids, &node_body, &node_feedback, &node_conf, &pi, &lib)
            } else {
                build_prompt(&theorem, parent_body.as_deref(), parent_feedback.as_deref(), &lib)
            };
            let resp = match llm
                .generate(&GenerateRequest {
                    model: args.model.clone(),
                    messages: vec![sys.clone(), Message { role: "user".into(), content: prompt }],
                    temperature: Some(0.7),
                    max_tokens: Some(900),
                })
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("lm llm_err {agent}: {e:?}");
                    continue;
                }
            };
            llm_calls += 1;
            let tokens = TokenCounts {
                prompt_tokens: resp.prompt_tokens as u64,
                completion_tokens: resp.completion_tokens as u64,
                tool_tokens: 0,
            };
            let v = match extract_json_object(&resp.content) {
                Some(v) => v,
                None => {
                    parse_fails += 1;
                    continue;
                }
            };
            let body = v.get("proof_body").and_then(|x| x.as_str()).unwrap_or("").to_string();
            if body.trim().is_empty() {
                parse_fails += 1;
                continue;
            }
            let confidence_pct = (v.get("confidence").and_then(|x| x.as_f64()).unwrap_or(0.6).clamp(0.0, 1.0) * 100.0) as u64;

            // AUTONOMOUS: the model picked its own parent index; validate it against the live
            // node list (fail-open to a fresh root on a hallucinated/out-of-range index — never
            // crash, never parse-fail). `select_parent` returned None for this arm (STEP 0); we
            // shadow it here with the model's choice. Non-autonomous arms keep the pre-call pick.
            let mut parent_tx = parent_tx;
            if args.policy == Policy::Autonomous {
                let chosen = v.get("parent_node").and_then(|x| x.as_i64()).unwrap_or(-1);
                parent_tx = resolve_parent_index(&node_tx_ids, chosen);
                // Route telemetry (Class 1, no behavior change): split the fail-open resolve into
                // {deliberate_fresh_root, valid_index_hit, hallucinated_out_of_range} so the run can
                // prove the headline mechanism fired (real non-local routing) vs a bailed-out
                // hallucination. resolve_parent_index above is unchanged; we only observe `chosen`.
                if chosen < 0 {
                    route_fresh += 1;
                } else if parent_tx.is_some() {
                    route_hit += 1;
                } else {
                    route_halluc += 1;
                }
            }

            // ── Real Lean kernel verdict ─────────────────────────────
            let outcome = judge.verify(&body);
            let is_verified = outcome.is_verified();
            if is_verified {
                verified_count += 1;
            } else {
                failed_count += 1;
            }
```

---
## §2. Parent selection per arm — `select_parent`
`src/bin/lean_market_agent.rs:289-335`
```rust
fn select_parent(
    policy: Policy,
    pi: &BTreeMap<TxId, NodeMarketEntry>,
    all_nodes: &[TxId],
    own_last: Option<&TxId>,
    node_conf: &BTreeMap<String, u64>,
    node_doubt: &BTreeMap<String, i64>,
    temp: f64,
    rng: &mut StdRng,
) -> Option<TxId> {
    match policy {
        // AUTONOMOUS: the LLM picks its own parent index INSIDE the proposal call, so the
        // pre-call selector is a no-op (None). The real parent is parsed from the model's
        // {parent_node} field and validated against node_tx_ids after the LLM returns.
        Policy::Autonomous => None,
        // TRUE Boltzmann softmax (Art. II.2.1): distribute attention across promising nodes
        // (incl. early ones → non-local re-expansion / new branches), NOT argmax-collapse.
        Policy::Market | Policy::RandomBear | Policy::FixedBear => boltzmann_softmax_select_parent(pi, &BTreeSet::new(), temp, rng)
            .or_else(|| all_nodes.last().cloned()),
        Policy::ShuffledPrice => {
            let shuffled = shuffle_prices(pi, rng);
            boltzmann_softmax_select_parent(&shuffled, &BTreeSet::new(), temp, rng)
                .or_else(|| all_nodes.last().cloned())
        }
        Policy::NoPrice => {
            if all_nodes.is_empty() {
                None
            } else {
                Some(all_nodes[rng.gen_range(0..all_nodes.len())].clone())
            }
        }
        // Own-chain baselines (no shared routing): refine only this agent's last node.
        Policy::Single | Policy::Parallel | Policy::Majority => own_last.cloned(),
        // Greedy best-first: extend the highest-confidence node on the shared tape,
        // with NO price and NO Bear short — isolates the priced market from plain greed.
        Policy::BestFirst => all_nodes
            .iter()
            .max_by_key(|t| node_conf.get(&t.0).copied().unwrap_or(0))
            .cloned(),
        // B6 skeptic-rerank: extend the LOWEST-doubt node per the SAME skeptic (critic-matched
        // budget); shared tape, NO price, NO short — isolates the critic heuristic from the market.
        Policy::SkepticRerank => all_nodes
            .iter()
            .min_by_key(|t| node_doubt.get(&t.0).copied().unwrap_or(i64::MAX))
            .cloned(),
    }
}
```

---
## §3. The SOFTMAX the **market** arm uses (true distributing Boltzmann, NOT argmax)
`src/sdk/actor.rs:100-148`
```rust
/// TRACE_MATRIX FC1-N5: price-routed parent selection feeding the rtool read-view (which node's
/// context the agent fetches next); Boltzmann-softmax attention distribution over node prices.
///
/// TRUE Boltzmann (softmax) parent selection — constitution Art. II.2.1 explore/exploit balance.
///
/// `boltzmann_select_parent_v2` above is argmax-by-price (+ epsilon-uniform): pure
/// EXPLOITATION → every agent collapses onto the single highest-price node → the work-DAG
/// degenerates to ONE chain (multi-agent ≈ single-agent) and the group loses heterogeneity —
/// the exact "过度利用 → 收敛同一局部最优 → 集体平庸" failure Art. II.2.1 forbids. This samples
/// node i with probability ∝ exp(price_i / temperature), so attention is DISTRIBUTED across
/// promising nodes (incl. EARLY ones → non-local re-expansion / new branches / backtracking),
/// preserving heterogeneity while staying price-guided. Temperature is the explore/exploit knob
/// (→0 = argmax-like exploit; large = uniform explore). f64 is used for the stochastic POLICY
/// only (NOT a money path; the chosen parent is recorded on tape, so replay reconstructs the
/// selection from L4, never by recompute — determinism is on the tape, not in this softmax).
pub fn boltzmann_softmax_select_parent<R: Rng>(
    price_index: &std::collections::BTreeMap<crate::state::TxId, crate::state::NodeMarketEntry>,
    mask_set: &std::collections::BTreeSet<crate::state::TxId>,
    temperature: f64,
    rng: &mut R,
) -> Option<crate::state::TxId> {
    let cands: Vec<(&crate::state::TxId, f64)> = price_index
        .iter()
        .filter(|(id, e)| e.price_yes.is_some() && !mask_set.contains(id))
        .map(|(id, e)| {
            let p = e.price_yes.as_ref().expect("filtered for Some");
            (id, (p.numerator as f64) / (p.denominator as f64))
        })
        .collect();
    if cands.is_empty() {
        return None;
    }
    let t = if temperature <= 0.0 { 1e-6 } else { temperature };
    // softmax with max-subtraction for numerical stability
    let maxp = cands.iter().map(|(_, p)| *p).fold(f64::MIN, f64::max);
    let weights: Vec<f64> = cands.iter().map(|(_, p)| ((p - maxp) / t).exp()).collect();
    let sum: f64 = weights.iter().sum();
    if !(sum > 0.0) {
        return Some(cands[0].0.clone());
    }
    let mut r = rng.gen::<f64>() * sum;
    for (i, w) in weights.iter().enumerate() {
        r -= *w;
        if r <= 0.0 {
            return Some(cands[i].0.clone());
        }
    }
    Some(cands[cands.len() - 1].0.clone())
}
```

---
## §4. The **market / single** prompt — `build_prompt`
`src/bin/lean_market_agent.rs:436-456`
```rust
fn build_prompt(theorem: &LeanTheorem, parent_body: Option<&str>, parent_feedback: Option<&str>, librarian: &str) -> String {
    let mut p = String::new();
    p.push_str("You are proving a theorem in Lean 4 (Mathlib is available). Output ONLY a JSON object.\n\n");
    p.push_str("=== Target (prove the goal after `:= by`) ===\n");
    p.push_str(&theorem.preamble);
    p.push('\n');
    if let (Some(body), Some(fb)) = (parent_body, parent_feedback) {
        p.push_str("\n=== A previous attempt FAILED — fix it ===\n--- attempt body ---\n");
        p.push_str(body);
        p.push_str("\n--- Lean error ---\n");
        p.push_str(fb);
        p.push('\n');
    }
    if !librarian.is_empty() {
        p.push_str(librarian);
    }
    p.push_str(
        "\nReturn EXACTLY: {\"proof_body\":\"<the Lean tactic block AFTER `:= by`, no theorem signature, no imports>\",\"confidence\":0.0-1.0}\n",
    );
    p
}
```

---
## §5. The **autonomous** prompt + landscape + choice-parse
`src/bin/lean_market_agent.rs:458-565` (`build_autonomous_prompt` builds the full-frontier landscape; `resolve_parent_index` validates the model's chosen index, fail-open to a fresh root).
```rust
/// AUTONOMOUS landscape prompt: shows the model the FULL frontier of prior attempts (every
/// node, including early ones) and lets it FREELY pick which to extend — by index — or start
/// fresh (`-1`). Inverts the market control flow (parent chosen by the LLM, not pre-selected).
/// SHIELDING: each node is shown as (index, price_yes ratio, confidence, error-CLASS via
/// `classify_lean_error`, body-snippet). EQUAL-RIGOR DEPTH (fairness fix): for the top-k nodes
/// by price the row ALSO carries that node's `node_feedback` — which is ALREADY the bounded
/// shielded `error:` line produced by `shield_lean_diagnostic` (FEEDBACK_MAX=240, lean_judge.rs)
/// and stored on tape — the SAME text the market arm injects via `build_prompt`'s
/// `parent_feedback` for its ONE pre-selected parent. This adds NO new information channel and NO
/// raw stderr: it is the identical already-shielded diagnostic, plumbed breadth-wise so the
/// autonomous arm repairs WITH detail at equal 1-call budget instead of BLIND-to-detail. The SAME
/// shielded collective librarian digest is injected (requirement A for this arm). ONE proposal
/// call per turn (identical budget to market).
const AUTONOMOUS_FEEDBACK_TOPK: usize = 6;
fn build_autonomous_prompt(
    theorem: &LeanTheorem,
    node_tx_ids: &[TxId],
    node_body: &BTreeMap<String, String>,
    node_feedback: &BTreeMap<String, String>,
    node_conf: &BTreeMap<String, u64>,
    pi: &BTreeMap<TxId, NodeMarketEntry>,
    librarian: &str,
) -> String {
    let mut p = String::new();
    p.push_str(
        "You are proving a theorem in Lean 4 (Mathlib is available) inside a proof-search market. \
         You see the FULL landscape of prior attempts (the search frontier). FREELY CHOOSE which \
         attempt to extend (give its index) OR start fresh (index -1). Prefer a promising but \
         unfinished line; you MAY branch from an EARLY attempt if later ones are dead ends. \
         Output ONLY a JSON object.\n\n",
    );
    p.push_str("=== Target (prove the goal after `:= by`) ===\n");
    p.push_str(&theorem.preamble);
    p.push('\n');
    if node_tx_ids.is_empty() {
        p.push_str("\n=== Landscape: EMPTY (you are the first attempt; use parent_node = -1) ===\n");
    } else {
        // The top-k nodes by price_yes get the SAME shielded `error:` diagnostic the market arm
        // sees for its single chosen parent (depth parity); the rest carry the coarse class only.
        let price_of = |tx: &TxId| -> f64 {
            pi.get(tx)
                .and_then(|e| e.price_yes.as_ref())
                .map(|r| (r.numerator as f64) / (r.denominator.max(1) as f64))
                .unwrap_or(0.0)
        };
        let mut ranked: Vec<usize> = (0..node_tx_ids.len()).collect();
        ranked.sort_by(|&a, &b| {
            price_of(&node_tx_ids[b])
                .partial_cmp(&price_of(&node_tx_ids[a]))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let detail_set: BTreeSet<usize> = ranked.into_iter().take(AUTONOMOUS_FEEDBACK_TOPK).collect();
        p.push_str("\n=== Landscape — all prior attempts (index : price_yes : confidence : error-class : body [FULL for top-priced nodes, else snippet] [: shielded Lean error for top-priced nodes]) ===\n");
        for (idx, tx) in node_tx_ids.iter().enumerate() {
            let body = node_body.get(&tx.0).map(|b| b.trim().replace('\n', " ")).unwrap_or_default();
            let fb = node_feedback.get(&tx.0);
            let class = fb.map(|f| classify_lean_error(f)).unwrap_or("pending");
            let conf = node_conf.get(&tx.0).copied().unwrap_or(0);
            let (pn, pd) = pi
                .get(tx)
                .and_then(|e| e.price_yes.as_ref())
                .map(|r| (r.numerator, r.denominator))
                .unwrap_or((0, 0));
            // BODY depth-parity (§17 rigged-arm fix): the top-k price nodes carry the FULL node_body
            // — the SAME untruncated text build_prompt feeds the market arm for its single chosen
            // parent (`p.push_str(body)`). The rest carry the coarse 110-char snippet. This matches
            // the breadth the FEEDBACK channel already uses (detail_set top-k get the full shielded
            // error, rest get class only); it adds NO new information channel and NO second call —
            // node_body already holds the full body on-tape — it only stops strawmanning a free
            // chooser that previously could not read the line it chose to extend. ONE call, same budget.
            let body_shown: String = if detail_set.contains(&idx) {
                body.clone()
            } else {
                body.chars().take(110).collect()
            };
            p.push_str(&format!("[{idx}] price={pn}/{pd} conf={conf}% class={class} :: `{body_shown}`"));
            // Depth-parity: the chosen-parent-grade shielded diagnostic (already FEEDBACK_MAX=240,
            // already error:-line only) for the top-priced nodes — the same text build_prompt feeds.
            if detail_set.contains(&idx) {
                if let Some(diag) = fb.filter(|d| !d.trim().is_empty()) {
                    let diag1 = diag.replace('\n', " ");
                    p.push_str(&format!("\n      lean-error: {diag1}"));
                }
            }
            p.push('\n');
        }
    }
    if !librarian.is_empty() {
        p.push_str(librarian); // (A) shielded collective-failure digest, same as market
    }
    p.push_str(
        "\nReturn EXACTLY: {\"parent_node\":<integer index from the landscape, or -1 for a fresh root>,\
         \"proof_body\":\"<the Lean tactic block AFTER `:= by`, no theorem signature, no imports>\",\
         \"confidence\":0.0-1.0}\n",
    );
    p
}

/// Resolve the model-chosen `parent_node` index against the canonical live node list.
/// FAIL-OPEN to a fresh root: a negative index OR an out-of-range (hallucinated) index → None
/// (do NOT panic, do NOT parse-fail — that would shrink the autonomous arm's node count below
/// market's and break budget parity). A valid index → the real WorkTx id at that position.
fn resolve_parent_index(node_tx_ids: &[TxId], chosen: i64) -> Option<TxId> {
    if chosen < 0 {
        None
    } else {
        node_tx_ids.get(chosen as usize).cloned()
    }
```

---
## §6. Budget parity — single gets the SAME total proposals
`src/bin/lean_market_agent.rs:650-655`
```rust
    let n_agents = if args.policy == Policy::Single { 1 } else { args.n_agents };
    // BUDGET PARITY (forensic fix 2026-06-01): every policy gets the SAME total proposal budget
    // = args.n_agents * args.n_rounds LLM proposals (+ the matching Lean verifies). Single is forced
    // to 1 agent, so it must run that many ROUNDS to match — else `market` silently gets n_agents× the
    // compute and any "market > single" is a budget artifact, not a market effect.
    let effective_rounds = if args.policy == Policy::Single { args.n_rounds * args.n_agents } else { args.n_rounds };
```

---
## §7. Independent reproduction
Build: `cargo build --bin lean_market_agent`. Run one cell (any arm):
```
./target/debug/lean_market_agent --runtime-repo <repo> --cas <cas> --run-id r1 \
  --problem lm_ineq1 --policy {autonomous|market|single} --n-agents 4 --n-rounds 6 --seed 1 \
  --model deepseek-v4-pro --bank tests/fixtures/lean_theorems_pool.jsonl --mathlib-dir <mathlib4> --out r1.json
```
Then `./target/debug/verify_chaintape --repo <repo> --cas <cas> --run-id r1` reconstructs state + economic_state
from the L4 tape alone (the replay gate). A solved proof's axioms: `#print axioms <thm>` ⊆ {propext, Classical.choice, Quot.sound}.
