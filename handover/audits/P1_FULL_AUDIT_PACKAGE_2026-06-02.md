# TuringOS P1 — Complete Self-Contained Audit Package (for an external auditor)

> You see ONLY this document. It contains the full causal claim, every load-bearing algorithm's **verbatim
> source** (annotated: what it does / where it runs / how it connects), the data, the pre-registration, and the
> one axiom-confirmed proof. Code is extracted verbatim from commit `50720c8a378a585401c4f39dfacd0aa3dc329dca`; an auditor with repo access could
> re-check via `git show 50720c8a378a585401c4f39dfacd0aa3dc329dca:<file>`. Repo is Rust + Lean 4 (Mathlib). End with PROCEED / CHALLENGE / VETO.

---
# PART A — The proposition and its CAUSAL logic

TuringOS claims a **market organization of agents** solves hard problems a single model cannot — not via a stronger
model or more compute, but via the **organization**: many agents over a shared proof tree, a **loss-bearing price**
(agents stake capital on YES/NO that a partial proof will close) routing attention, and **non-local restart** from
any earlier node. Three arms, **same model (deepseek-v4-pro), same 24-attempt budget, same real Lean verifier**:

| arm | organization | causal role |
|---|---|---|
| **single** | 1 agent refining its OWN chain | baseline / counterfactual |
| **market** (Path-2) | N agents; HARNESS forces the node by **softmax over the loss-bearing price** | forced price-routing |
| **autonomous** (Path-1) | N agents; the AGENT itself reads the full price landscape and FREELY picks the node | agent free-choice routing |

**Causal hypothesis:** at equal model+budget+verifier, if market/autonomous SOLVE a theorem single CANNOT, the cause
is the **organization** (the only thing differing from single), not the model or luck.

**The confound-shield (scrutinize):** the test set = theorems where **single reliably FAILS (0/N seeds; 24/24 in a
prior scan)**. On a problem the single model provably can't do at this budget, model-capability + lucky-guess variance
are **flattened to a 0 floor** — so a SOLVE by a multi-agent arm at the *same* model+budget is attributable only to
the organization. Each = a **"crack"** (one confound-shielded data point).

**The cleanest contrast:** `autonomous` vs `market` are identical **except who picks the node** — same model, same
budget, the **same loss-bearing price**, the same librarian, the same shielded repair-depth. So `autonomous > market`
isolates **free-choice routing (Path-1) > forced price-routing (Path-2)`, all else held constant.

---
# PART B — Every load-bearing algorithm (verbatim + annotated)

## B0. The shared run loop — where the 3 arms branch at runtime
Annotation: ONE binary, ONE loop, ONE real-Lean verifier. The arms differ ONLY at: `select_parent(...)` (parent
choice), the prompt `if` (autonomous landscape vs market/single fix-it), and the autonomous parent-parse. The LLM call
(identical params) and `judge.verify` (real Lean) are shared. This is the proof that the arms are single-variable.
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

### B1. Parent selection per arm — `select_parent`
The routing TREATMENT. `single`→own_last (one chain); `market`→softmax over price (forced); `autonomous`→None (agent chooses in-call, see B4); `shuffled_price`→softmax over PERMUTED price (ablation: isolates whether the PRICE routes); `no_price`→random node (ablation: isolates non-locality vs price). Called at loop line 733.

**Code (`src/bin/lean_market_agent.rs`):**
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

### B2. The market softmax (true distributing Boltzmann, NOT argmax)
What `market` routes on. Samples node i with probability ∝ exp(price_i / temperature). The doc-comment contrasts it with the OLD argmax bug (which collapsed all agents onto one node = multi-agent≈single). Temperature is the explore/exploit knob (fixed 0.15 here, pre-registered, NO annealing — an adversarial parity audit ruled annealing a tune-to-win that would unfairly advantage the forced arm).

**Code (`src/sdk/actor.rs`):**
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

### B3. The market / single prompt — `build_prompt`
What `market` and `single` agents see: the goal + (if they have a parent) that ONE parent's failed body + its shielded 240-char Lean error + the librarian digest. Asks for {proof_body, confidence}.

**Code (`src/bin/lean_market_agent.rs`):**
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

### B4. The autonomous prompt + landscape + the agent's choice-parse
What `autonomous` agents see + how their free choice is applied. `build_autonomous_prompt` shows the FULL frontier ([idx] price conf class :: body) for EVERY node — the top-6-by-price get the SAME shielded 240-char Lean error `market` gets (a fairness fix so the free-chooser repairs WITH detail, not blind). The model returns {parent_node:<idx or -1>, proof_body, confidence}; `resolve_parent_index` validates the index (fail-open to a fresh root on a hallucinated index — never crash, to keep node-count = market's). Route telemetry (loop lines 788-802) splits the outcome into {fresh_root, valid_hit, hallucinated} so 'the agent really chose' is falsifiable.

**Code (`src/bin/lean_market_agent.rs`):**
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


## B5. The LOSS-BEARING PRICE — the Hayek core that `market` routes on

EVERY proof attempt becomes a priced node. The prover posts a **WorkTx-Long** stake scaled by confidence (`stake_from_confidence`); an independent **skeptic LLM** rates P(does NOT compile) and posts a loss-bearing **Bear ChallengeTx-Short** (`bear_doubt_short`). `compute_price_index` then sets `price_yes = long/(long+short)` — the implied probability the proof is correct. THIS is what `market` softmaxes over and `autonomous` reads. Non-price baselines (single/parallel) are Bulls-only.

### B5a. Long stake (prover capital-at-risk)
Confidence → integer micro-stake (integer money path).

**Code (`src/bin/lean_market_agent.rs`):**
```rust
fn stake_from_confidence(confidence_pct: u64) -> i64 {
    let mult_num = (25 + 375 * confidence_pct.min(100) as i64 / 100).max(25);
    (BASE_WORK_STAKE.saturating_mul(mult_num) / 100).clamp(MIN_STAKE_MICRO, MAX_STAKE_MICRO)
}
```

### B5b. Bear short (skeptic capital-at-risk → the price signal)
Independent skeptic LLM rates P(not-compile); short scales with doubt. Weak proof → big short → low price_yes.

**Code (`src/bin/lean_market_agent.rs`):**
```rust
async fn bear_doubt_short(
    llm: &ResilientLLMClient,
    model: &str,
    theorem: &LeanTheorem,
    body: &str,
) -> (i64, u64) {
    let prompt = format!(
        "You are a SKEPTIC in a proof market. A prover submitted the Lean 4 proof body below \
         for the goal. Estimate the probability it does NOT compile under the Lean kernel \
         (0.0 = certainly compiles, 1.0 = certainly fails). Judge ONLY from the text; be \
         calibrated (most terse first attempts fail). Output ONLY JSON.\n\n\
         === Goal ===\n{}\n\n=== Proof body ===\n{}\n\nReturn EXACTLY: {{\"doubt\":0.0-1.0}}",
        theorem.preamble, body
    );
    match llm
        .generate(&GenerateRequest {
            model: model.into(),
            messages: vec![Message { role: "user".into(), content: prompt }],
            temperature: Some(0.3),
            max_tokens: Some(60),
        })
        .await
    {
        Ok(r) => {
            let doubt = extract_json_object(&r.content)
                .and_then(|v| v.get("doubt").and_then(|x| x.as_f64()))
                .unwrap_or(0.5)
                .clamp(0.0, 1.0);
            // probability → integer percent (not a money op); stake math stays integer.
            let doubt_pct = (doubt * 100.0) as i64;
            let short = MIN_SHORT_MICRO + (MAX_SHORT_MICRO - MIN_SHORT_MICRO) * doubt_pct / 100;
            (short, (r.prompt_tokens + r.completion_tokens) as u64)
        }
        Err(_) => (CHALLENGE_STAKE_MICRO, 0),
    }
}
```

### B5c. Posting the node + Bear short (in the loop)
Every attempt → TaskOpen+Escrow+WorkTx(Long) (836-855); price arms then post Bear ChallengeTx(Short) (859-887).

**Code (`src/bin/lean_market_agent.rs`):**
```rust

            // ── Per-task node (EVERY attempt — Verified or Failed) ────
            let work_stake = stake_from_confidence(confidence_pct);
            let node_task = format!("lm-node{step_idx}-{}", args.run_id);
            root = submit_await(&seq, make_real_task_open_signed_by(&mut kp, &node_task, SPONSOR_AGENT, root, "lm", lt).map_err(|e| format!("TaskOpen node: {e}"))?, root, "TaskOpen(node)").await?;
            lt += 1;
            root = submit_await(&seq, make_real_escrow_lock_signed_by(&mut kp, &node_task, SPONSOR_AGENT, TASK_ESCROW_MICRO, root, "lm", lt).map_err(|e| format!("Escrow node: {e}"))?, root, "Escrow(node)").await?;
            lt += 1;
            let pcid = put_proposal(&args.cas, &args.run_id, &agent, step_idx, parent_tx.clone(), &body, tokens, lt)?;
            lt += 2;
            let work = make_real_worktx_signed_by(&mut kp, &node_task, &agent, root, work_stake, "lm", pcid, true, lt).map_err(|e| format!("WorkTx: {e}"))?;
            let work_tx_id = match &work {
                TypedTx::Work(w) => w.tx_id.0.clone(),
                _ => return Err("not WorkTx".into()),
            };
            root = submit_await(&seq, work, root, "WorkTx").await?;
            lt += 1;
            node_tx_ids.push(TxId(work_tx_id.clone()));
            own_last.insert(agent.clone(), TxId(work_tx_id.clone()));
            node_body.insert(work_tx_id.clone(), body.clone());
            node_feedback.insert(work_tx_id.clone(), outcome.feedback.clone());
            node_conf.insert(work_tx_id.clone(), confidence_pct);

            // Short challenge → price_yes (price-family policies only; non-market
            // baselines are Bulls-only). Non-fatal.
            if args.policy.emits_challenges() {
                // Bear short by policy: informed (skeptic-LLM doubt) for market/shuffled/no_price;
                // random U(0,1) with NO skeptic call (M1); or fixed constant (M2). M1/M2 isolate
                // whether the *informed* price signal (vs noise / vs a constant) does the work.
                let (short_micro, bear_tok) = match args.policy {
                    Policy::RandomBear => {
                        let doubt_pct = rng.gen_range(0..=100) as i64;
                        (MIN_SHORT_MICRO + (MAX_SHORT_MICRO - MIN_SHORT_MICRO) * doubt_pct / 100, 0u64)
                    }
                    Policy::FixedBear => (CHALLENGE_STAKE_MICRO, 0u64),
                    _ => bear_doubt_short(&llm, &args.model, &theorem, &body).await,
                };
                bear_calls += 1;
                bear_tokens_total += bear_tok;
                let challenger = challengers[ai % challengers.len()].clone();
                if let Ok(ce) = put_counterexample(&args.cas, &work_tx_id, lt) {
                    lt += 1;
                    match make_real_challengetx_signed_by(&mut kp, root, TxId(work_tx_id.clone()), &challenger, short_micro, ce, &format!("lm{step_idx}"), lt) {
                        Ok(chal) => match submit_await(&seq, chal, root, "ChallengeTx").await {
                            Ok(r) => {
                                root = r;
                                lt += 1;
                            }
                            Err(e) => eprintln!("lm challenge skip node{step_idx}: {e}"),
                        },
                        Err(e) => eprintln!("lm challenge build skip: {e}"),
                    }
                }
            }
```

### B5d. The price formula — compute_price_index
price_yes = long/(long+short), integer-rational u128; None iff zero liquidity; re-derived from EconomicState (itself L4-reconstructable).

**Code (`src/state/price_index.rs`):**
```rust
pub fn compute_price_index(econ: &EconomicState) -> BTreeMap<TxId, NodeMarketEntry> {
    // Pass 1: group NodePositions by node_id; collect (task_id, long_micro, short_micro).
    let mut groups: BTreeMap<TxId, (TaskId, u128, u128)> = BTreeMap::new();
    for position in econ.node_positions_t.0.values() {
        let amount_micro = position.amount.micro_units();
        let amount_u128 = if amount_micro < 0 {
            0u128
        } else {
            amount_micro as u128
        };
        let entry = groups
            .entry(position.node_id.clone())
            .or_insert_with(|| (position.task_id.clone(), 0u128, 0u128));
        match position.side {
            PositionSide::Long => entry.1 = entry.1.saturating_add(amount_u128),
            PositionSide::Short => entry.2 = entry.2.saturating_add(amount_u128),
        }
    }

    // Pass 2: per node, derive NodeMarketEntry.
    let mut out: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
    for (node_id, (task_id, long_micro, short_micro)) in groups.into_iter() {
        let total_micro = long_micro.saturating_add(short_micro);
        let event_id = EventId(task_id.clone());

        let to_micro = |u: u128| -> MicroCoin {
            // Saturating cast u128 → i64 (positive values only; cap at i64::MAX).
            let capped = if u > i64::MAX as u128 {
                i64::MAX
            } else {
                u as i64
            };
            MicroCoin::from_micro_units(capped)
        };

        let (price_yes, price_no) = if total_micro == 0 {
            (None, None)
        } else {
            (
                Some(RationalPrice {
                    numerator: long_micro,
                    denominator: total_micro,
                }),
                Some(RationalPrice {
                    numerator: short_micro,
                    denominator: total_micro,
                }),
```

## B6. The REAL Lean verifier — `LeanJudge::verify` (the OMEGA gate)

A "crack" fires ONLY when this returns `Verified`. It (1) source-scans the candidate for [`sorry`,`admit`,`native_decide`] → SorryBlocked (blocks the native_decide exit-0 trap), then (2) runs REAL `lean -DwarningAsError=true <file>`, Verified iff exit 0. **GAP to weigh:** it does NOT run a POSITIVE `#print axioms ⊆ whitelist` inline — cracks are axiom-confirmed post-hoc (Part E).

### B6a. Kernel-bypass tokens + LeanOutcome + verify
The real verifier.

**Code (`src/judges/lean_judge.rs`):**
```rust
/// TRACE_MATRIX FC1a-judge_pi: kernel-trust-bypass tokens the JudgeAI rejects.
/// Tokens that close a goal without a real kernel proof or bypass kernel trust.
/// `sorry`/`admit` also surface as warnings (caught by `-DwarningAsError`), but we
/// reject them at the source so the verdict is `SorryBlocked` (not `Failed`), and so
/// that `native_decide` — which is NOT a warning and would otherwise exit 0 — is also
/// blocked. Mirrors constitution bus rule C-011 (forbidden scratch-work tactics).
pub const KERNEL_BYPASS_TOKENS: &[&str] = &["sorry", "admit", "native_decide"];

/// Max bytes of (shielded) Lean error text fed back into a retry prompt. The error
/// is the public compiler diagnostic on the agent's OWN candidate (legitimate retry
/// signal, like the swebench judge's failing-test names), bounded and never a raw
/// full-stderr dump (CLAUDE.md §4 raw-Lean-stderr shielding).
const FEEDBACK_MAX: usize = 240;

static TMP_COUNTER: AtomicU64 = AtomicU64::new(0);

/// TRACE_MATRIX FC1a-judge_pi: typed JudgeAI verdict for one candidate proof.
/// Strict Lean outcome for one candidate proof against the fixed target theorem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeanOutcome {
    pub verdict_kind: LeanVerdictKind,
    pub error_class: Option<LeanErrorClass>,
    pub exit_code: i32,
    pub timed_out: bool,
    /// Bounded, shielded failure summary for the retry prompt (empty on Verified).
    pub feedback: String,
}

impl LeanOutcome {
    /// TRACE_MATRIX FC1a-judge_pi: true iff the JudgeAI verdict is a clean OMEGA.
    pub fn is_verified(&self) -> bool {
        matches!(self.verdict_kind, LeanVerdictKind::Verified)
    }
}

/// TRACE_MATRIX FC1a-judge_pi: pure Lean-kernel JudgeAI verifier (one target theorem).
/// A pure Lean verifier bound to ONE fixed target theorem.
#[derive(Debug, Clone)]
pub struct LeanJudge {
    /// `imports + set_option + open + "theorem <name> <args> : <goal> := by"`.
    /// The candidate proof body is appended after it.
    pub preamble: String,
    /// Lean binary. Pin to a concrete toolchain bin to avoid elan auto-download
    /// (a bare `lean` shim tries to fetch the latest toolchain — fatal offline).
    pub lean_bin: PathBuf,
    /// cwd for the lean process (repo root for core; the lake project dir for Mathlib).
    pub cwd: PathBuf,
    /// Extra env beyond the PATH+HOME allowlist (e.g. `("LEAN_PATH", "<oleans>")`).
    pub extra_env: Vec<(String, String)>,
    /// Per-verify wall-clock timeout.
    pub timeout: Duration,
}

impl LeanJudge {
    /// TRACE_MATRIX FC1a-judge_pi: construct the JudgeAI verifier with defaults.
    /// Construct with sane defaults: the pinned toolchain bin, repo-root cwd, 60s.
    pub fn new(preamble: impl Into<String>) -> Self {
        Self {
            preamble: preamble.into(),
            lean_bin: default_lean_bin(),
            cwd: std::env::current_dir().unwrap_or_else(|_| std::env::temp_dir()),
            extra_env: Vec::new(),
            timeout: Duration::from_secs(60),
        }
    }

    /// TRACE_MATRIX FC1a-judge_pi: assemble a candidate proof body into a checkable file.
    /// Assemble the full `.lean` source for a candidate proof body.
    pub fn assemble(&self, candidate_body: &str) -> String {
        let mut s = String::with_capacity(self.preamble.len() + candidate_body.len() + 2);
        s.push_str(&self.preamble);
        if !self.preamble.ends_with('\n') && !self.preamble.ends_with(' ') {
            s.push('\n');
        }
        s.push_str(candidate_body.trim());
        s.push('\n');
        s
    }

    /// TRACE_MATRIX FC1a-judge_pi: the JudgeAI verdict — verify a candidate proof body.
    /// Verify a candidate proof body and return the strict Lean outcome.
    pub fn verify(&self, candidate_body: &str) -> LeanOutcome {
        // 1. Source-scan the CANDIDATE (the preamble is trusted/fixed). Strip
        //    comments first so a `sorry` mentioned in a comment is not a false reject.
        if let Some(tok) = first_bypass_token(candidate_body) {
            return LeanOutcome {
                verdict_kind: LeanVerdictKind::SorryBlocked,
                error_class: Some(LeanErrorClass::SorryBlocked),
                exit_code: 0,
                timed_out: false,
                feedback: format!("kernel-bypass token `{tok}` is forbidden"),
            };
        }

        // 2. Assemble + write a temp .lean file.
        let src = self.assemble(candidate_body);
        let n = TMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "turingos-leanjudge-{}-{}.lean",
            std::process::id(),
            n
        ));
        if std::fs::write(&path, src.as_bytes()).is_err() {
            return failed(-1, false, "could not write temp lean file".into());
        }

        // 3. Run `lean -DwarningAsError=true <file>` under the sanitized runner.
        let mut env = env_allowlist_from_current(&["PATH", "HOME"]);
        for (k, v) in &self.extra_env {
            env.insert(k.clone(), v.clone());
        }
        let out = run_sanitized(SanitizedCommand {
            program: self.lean_bin.clone(),
            args: vec![
                "-DwarningAsError=true".into(),
                path.to_string_lossy().into_owned(),
            ],
            cwd: self.cwd.clone(),
            env,
            stdin: None,
            timeout: self.timeout,
        });
        let _ = std::fs::remove_file(&path);

        match out {
            Ok(o) if o.success() => LeanOutcome {
                verdict_kind: LeanVerdictKind::Verified,
                error_class: None,
                exit_code: 0,
                timed_out: false,
                feedback: String::new(),
            },
            Ok(o) => {
                let timed_out = o.timed_out;
                let feedback = if timed_out {
                    "lean timed out".to_string()
                } else {
                    shield_lean_diagnostic(&o.stderr, &o.stdout)
                };
                failed(o.exit_code.unwrap_or(-1), timed_out, feedback)
            }
            Err(e) => failed(-1, false, format!("lean spawn failed: {e}")),
```

### B7. The librarian (collective failure memory, held CONSTANT across price arms)
Reads typed LeanResult sidecars in CAS, builds the REAL constitutional shielded digest (librarian_broadcast.rs, not a lookalike), projects Solver crop. Same digest in market AND autonomous = a control. classify_lean_error shields raw errors to opaque CLASSES.

**Code (`src/bin/lean_market_agent.rs`):**
```rust
fn classify_lean_error(fb: &str) -> &'static str {
    let f = fb.to_lowercase();
    if f.contains("unsolved goals") { "unsolved_goals" }
    else if f.contains("type mismatch") { "type_mismatch" }
    else if f.contains("unknown identifier") || f.contains("unknown constant") { "unknown_identifier" }
    else if f.contains("rewrite") && f.contains("fail") { "rewrite_failed" }
    else if f.contains("nlinarith") || f.contains("linarith") || f.contains("positivity") { "arith_failed" }
    else if f.contains("unexpected") || f.contains("syntax") || f.contains("expected") { "syntax_error" }
    else if f.contains("no progress") { "no_progress" }
    else if f.trim().is_empty() { "no_feedback" }
    else { "other_error" }
}

/// REAL librarian collective digest (src/runtime/librarian_broadcast.rs — the full
/// constitutional mechanism, NOT a lookalike). Reads the typed LeanResult sidecars this
/// run already wrote into CAS, builds a deterministic shielded `LibrarianDigest`, and
/// projects the Solver crop into a bounded "=== Librarian Notices ===" prompt block.
/// Everything that transits is an opaque error CLASS / pre-written public_summary —
/// `assert_no_forbidden_broadcast_material` runs on every event + cluster + rendered line.
///
/// Returns "" (no librarian section) when the source scope is invalid, no typed evidence
/// exists yet, or the Solver crop is empty (e.g. <2 of any one error class → no cluster).
/// Read-only: opens a FRESH `CasStore` (open-per-read, mirrors the bin's put helpers) and
/// never mutates the run.
fn real_librarian_solver_notice(cas_path: &PathBuf, current_head_t: u64, problem: &str) -> String {
    let cas = match CasStore::open(cas_path) {
        Ok(c) => c,
        Err(_) => return String::new(),
    };
    let scope = LibrarianSourceScope {
        current_run_cas_root: derive_current_run_cas_root(&cas), // run-local surrogate, NOT a global pointer
        prior_capsule_cids: vec![],
        max_prior_batches: 0,
        task_tags: vec![problem.to_string()], // problem id; fail-closed if it contains latest/pointer/.txt
    };
    if validate_librarian_source_scope(&scope, &cas).is_err() {
        return String::new();
    }
    let events = match select_librarian_events(&cas) {
        Ok(e) => e,
        Err(_) => return String::new(), // fail-closed selector errored (e.g. unknown schema) → no section
    };
    if events.is_empty() {
        return String::new();
    }
    let digest = match build_librarian_digest(scope, current_head_t, events) {
        Ok(d) => d,
        Err(_) => return String::new(),
    };
    let view = match project_role_notifications(&digest, AgentRole::Solver, 10) {
        Ok(v) => v,
        Err(_) => return String::new(),
    };
    // Empty-crop sentinel: don't inject a section that says nothing actionable.
    if view.rendered_notice.contains("No librarian notices for this role at current scope") {
        return String::new();
    }
    format!("\n{}", view.rendered_notice)
}

```

### B8. Budget parity — single gets the SAME total proposals
single forced to 1 agent → runs n_agents*n_rounds rounds → 1x24 = 4x6 = 24 proposals + 24 Lean verifies.

**Code (`src/bin/lean_market_agent.rs`):**
```rust
    let n_agents = if args.policy == Policy::Single { 1 } else { args.n_agents };
    // BUDGET PARITY (forensic fix 2026-06-01): every policy gets the SAME total proposal budget
    // = args.n_agents * args.n_rounds LLM proposals (+ the matching Lean verifies). Single is forced
    // to 1 agent, so it must run that many ROUNDS to match — else `market` silently gets n_agents× the
    // compute and any "market > single" is a budget artifact, not a market effect.
    let effective_rounds = if args.policy == Policy::Single { args.n_rounds * args.n_agents } else { args.n_rounds };
```

### B9. Ablation — shuffle_prices (the shuffled_price arm)
Permutes price VALUES across node keys → routing on a destroyed signal. Isolates PRICE vs tree structure.

**Code (`src/bin/lean_market_agent.rs`):**
```rust
/// A0: permute the price values among the node keys, so parent selection runs on a
/// randomized routing signal (same nodes, same compute, signal destroyed).
fn shuffle_prices(
    pi: &BTreeMap<TxId, NodeMarketEntry>,
    rng: &mut StdRng,
) -> BTreeMap<TxId, NodeMarketEntry> {
    let keys: Vec<TxId> = pi.keys().cloned().collect();
    let mut vals: Vec<NodeMarketEntry> = pi.values().cloned().collect();
    for i in (1..vals.len()).rev() {
        let j = rng.gen_range(0..=i);
        vals.swap(i, j);
    }
    keys.into_iter().zip(vals).collect()
}
```

## B10. The statistics — `analyze_p1_hardproblem.py` (exact McNemar + Holm)

SOLVED is binary. Paired EXACT McNemar (one-sided) by (theorem,seed): b=arm-X-only solves, c=foil-only; p=P(Bin(b+c,0.5)≥b). Holm over the comparisons. A cell counts ONLY if verify_chaintape replay-clean.

### B10a. mcnemar + holm
The significance machinery.

**Code (`scripts/analyze_p1_hardproblem.py`):**
```python
    for i, (name, p) in enumerate(items):
        a = min(1.0, (m - i) * p); a = max(a, prev); adj[name] = a; prev = a
    return adj

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dir", default="handover/evidence/p1_realvalue_v3_2026-06-02")
    ap.add_argument("--theorems", default="lm_deriv1,lm_ineq1,lm_ineq2,lm_coeff_mul,lm_nt_gcd2,lm_median")
    ap.add_argument("--arms", default="autonomous,market,single")
    ap.add_argument("--seeds", default="1,2,3")
    ap.add_argument("--alpha", type=float, default=0.05)
    a = ap.parse_args()
    thms=[x.strip() for x in a.theorems.split(",")]; arms=[x.strip() for x in a.arms.split(",")]; seeds=[x.strip() for x in a.seeds.split(",")]

    solved=defaultdict(dict); route=defaultdict(lambda: defaultdict(int)); excluded=[]
    for thm in thms:
        for arm in arms:
            for s in seeds:
                c=os.path.join(a.dir, f"{thm}__{arm}__s{s}"); mf,rr=c+".json",c+".replay.json"
```

### B10b. crack detection + replay gate + route honesty
Counts replay-clean cells only; flags route hallucination.

**Code (`scripts/analyze_p1_hardproblem.py`):**
```python
        print()

    print("=== per-theorem solve counts (X=solved, .=fail); the HARD set = single solves 0/N ===")
    print(f"  {'theorem':14} " + " ".join(f"{ar:^12}" for ar in arms))
    hard=[]  # theorems where single solved 0
    for thm in thms:
        cols=[];
        for ar in arms:
            cells=sorted([(s,solved[ar][(thm,s)]) for s in seeds if (thm,s) in solved[ar]])
            cols.append("".join("X" if v else "." for _,v in cells) or "-")
        sc=sum(solved.get("single",{}).get((thm,s),0) for s in seeds) if "single" in arms else None
        if sc==0: hard.append(thm)
        print(f"  {thm:14} " + " ".join(f"{c:^12}" for c in cols) + ("   <- HARD (single 0)" if sc==0 else ""))

    # CONFIRMED CRACKS: hard theorems (single 0) that a price arm solved >=1
    print("\n=== CONFIRMED CRACKS — hard theorems (single 0/N) a price arm SOLVED (the confound-shielded headline) ===")
    price_arms=[ar for ar in ("autonomous","market") if ar in arms]
    any_crack=False
    for thm in hard:
        for ar in price_arms:
            k=sum(solved.get(ar,{}).get((thm,s),0) for s in seeds); n=sum(1 for s in seeds if (thm,s) in solved.get(ar,{}))
            if k>0:
                any_crack=True; print(f"  *** {ar} CRACKED {thm} ({k}/{n} seeds) where single solved 0 ***")
    if not any_crack: print("  (none yet — no price arm has cracked a single-0 theorem in the replay-clean cells)")

    # pairwise exact McNemar
    print(f"\n=== pairwise paired McNemar (one-sided, Holm @ alpha={a.alpha}) ===")
    pairs=[(x,y) for x in price_arms for y in (["single"] if "single" in arms else [])] + ([("autonomous","market")] if set(["autonomous","market"]).issubset(arms) else [])
    pv={}; disc={}
    for x,y in pairs:
        keys=[k for k in solved.get(x,{}) if k in solved.get(y,{})]
        b=sum(1 for k in keys if solved[x][k]==1 and solved[y][k]==0)
        c=sum(1 for k in keys if solved[x][k]==0 and solved[y][k]==1)
        pv[f"{x}>{y}"]=mcnemar_one_sided_greater(b,c); disc[f"{x}>{y}"]=(b,c,len(keys))
    adj=holm(pv)
    for name in pv:
        b,c,n=disc[name]; print(f"  {name:24} {name.split('>')[0]}-only={b} {name.split('>')[1]}-only={c} (n={n})  p_holm={adj[name]:.4f}  {'PASS' if adj[name]<a.alpha and b>c else '-'}")

    # route telemetry honesty (autonomous)
    if "autonomous" in arms:
        r=route["autonomous"]; tot=r["route_valid_index_hit"]+r["route_deliberate_fresh_root"]+r["route_hallucinated_out_of_range"]
        print("\n=== route telemetry (autonomous) — is 'free routing' genuine? ===")
        print(f"  valid_index_hit={r['route_valid_index_hit']} deliberate_fresh_root={r['route_deliberate_fresh_root']} hallucinated_out_of_range={r['route_hallucinated_out_of_range']} (total routed={tot})")
        if tot:
            hr=r['route_hallucinated_out_of_range']/tot
            print(f"  => hallucination rate {hr:.1%}" + ("  WARN: high hallucination — free-routing claim weakened" if hr>0.2 else "  (low — routing is deliberate)"))

    print("\n=== VERDICT (held to Verdict B until a CONFIRMED CRACK clears §17 G1-G6) ===")
    if any_crack:
        print("  A-direction: a price-routed arm CRACKED a hard theorem single could not (confound-shielded). Significance + replay + route-honesty gate the headline.")
    else:
        print("  A: no confirmed crack in replay-clean cells yet (report as-is; needs the full sweep / more seeds before NO-GO vs INCONCLUSIVE).")
    print("  B: every counted cell verify_chaintape replay-clean (gated above). A never inferred from B.")

if __name__ == "__main__":
    main()
```

---
# PART C — The data (verbatim analyzer output on the 54-cell counted run)

Counted run: handover/evidence/p1_realvalue_v3_2026-06-02/ (54 cells = {autonomous,market,single} x 6 hard theorems x 3 seeds). Re-run `python3 scripts/analyze_p1_hardproblem.py --dir <that dir>`:
```
=== per-theorem solve counts (X=solved, .=fail); the HARD set = single solves 0/N ===
  theorem         autonomous     market       single   
  lm_deriv1          ..X          ...          ...        <- HARD (single 0)
  lm_ineq1           .X.          ...          ...        <- HARD (single 0)
  lm_ineq2           ...          ...          ...        <- HARD (single 0)
  lm_coeff_mul       ...          ...          ...        <- HARD (single 0)
  lm_nt_gcd2         ...          ...          ...        <- HARD (single 0)
  lm_median          ...          ...          ...        <- HARD (single 0)

=== CONFIRMED CRACKS — hard theorems (single 0/N) a price arm SOLVED (the confound-shielded headline) ===
  *** autonomous CRACKED lm_deriv1 (1/3 seeds) where single solved 0 ***
  *** autonomous CRACKED lm_ineq1 (1/3 seeds) where single solved 0 ***

=== pairwise paired McNemar (one-sided, Holm @ alpha=0.05) ===
  autonomous>single        autonomous-only=2 single-only=0 (n=18)  p_holm=0.7500  -
  market>single            market-only=0 single-only=0 (n=18)  p_holm=1.0000  -
  autonomous>market        autonomous-only=2 market-only=0 (n=18)  p_holm=0.7500  -

=== route telemetry (autonomous) — is 'free routing' genuine? ===
  valid_index_hit=289 deliberate_fresh_root=123 hallucinated_out_of_range=0 (total routed=412)
  => hallucination rate 0.0%  (low — routing is deliberate)

=== VERDICT (held to Verdict B until a CONFIRMED CRACK clears §17 G1-G6) ===
  A-direction: a price-routed arm CRACKED a hard theorem single could not (confound-shielded). Significance + replay + route-honesty gate the headline.
  B: every counted cell verify_chaintape replay-clean (gated above). A never inferred from B.
```

A **6-seed axiom-confirm re-run** on the 2 cracked theorems gave lm_deriv1 **0/6** (its original 1/3 did NOT reproduce) and lm_ineq1 **1/6** — so combined ~1/9-2/9: the 3-seed run **over-estimated** the rate. A **scale-up** (autonomous+market on 12 more hard theorems x 3 seeds) was running at the time of writing to test significance.

---
# PART D — The pre-registration (SHA-locked BEFORE the counted run)

File `handover/preregistration/P1_REALVALUE_PREREG_2026-06-01.json`, sha256 `621d079d782d92367310d37844063f6e98670ae96e95f0a0b4a2cb993094a703` (verify byte-identical to its sibling .sha256). Key locked fields:
```json
{
 "the_question": "On HARD theorems where a single chain (same model, same total budget) RELIABLY FAILS (0/N seeds), does the MARKET (non-local softmax tree search with loss-bearing price) SOLVE them — and is the win attributable to PRICE + NON-LOCALITY (market also beats parallel N-chains [swarm], shuffled_price, and no_price)? A market solve where single AND parallel AND shuffled_price AND no_price all fail is a clean, confound-shielded proof that price-routed non-local tree search is a superior agent organization.",
 "arms": {
  "autonomous": "PATH-1: in ONE call the AGENT reads the full-chain landscape (every node: index+price+conf+error-class+snippet, incl EARLY nodes) + the librarian digest, and freely returns {parent_node, proof_body, confidence} — its own choice of which node to extend or which early node to branch from. Loss-bearing price + Bear shorts.",
  "market": "PATH-2: the harness picks the parent by TRUE softmax over the full live price index (forced push, non-local-capable) + loss-bearing price + Bear shorts + librarian. The forced-routing contrast to autonomous.",
  "single": "1 agent, 1 chain, the SAME total proposal budget (n_agents*n_rounds rounds). The hard floor: theorems where THIS reliably fails (0/N) are the test set.",
  "parallel": "N independent chains (each agent refines only its OWN last node), same budget — the SWARM control. > parallel rules out 'just more independent samples' and isolates COORDINATION.",
  "shuffled_price": "softmax over the PERMUTED price vector (same distribution, wrong assignment) — isolates whether the PRICE (not just the tree) routes.",
  "no_price": "random parent over the shared tape (tree, no price signal) — isolates whether the PRICE (not just shared non-local re-expansion) is load-bearing.",
  "librarian_control": "the REAL librarian collective-failure digest is in EVERY price arm (autonomous/market/shuffled_price/no_price); single sees its own chain's failures. Held constant so the autonomous-vs-market contrast is purely the routing mode."
 },
 "metric": {
  "primary": "CONFIRMED WINS = count of HARD theorems (single 0/seeds) that market solves (>=1 seed) AND parallel/shuffled_price/no_price all solve 0. Each is a confound-shielded proof point. Headline = 'market cracked N theorems that single, parallel, shuffled-price and no-price all could not, at equal budget.'",
  "support_test": "paired EXACT McNemar (one-sided) market vs each of {single, parallel, shuffled_price, no_price} on the SOLVED indicator over (theorem,seed); Holm over the 4. SOLVED = omega_reached (full LeanVerdictKind::Verified, verified_count>0).",
  "replay_gate": "a cell counts ONLY if verify_chaintape returns economic_state_reconstructed=true (replay-RECOMPUTE from L4, not byte-only) — §17.2. Non-clean cells EXCLUDED + reported.",
  "axiom_gate": "every market-solved proof body must additionally pass #print axioms subset {propext, Classical.choice, Quot.sound} (no native_decide/sorry); a solve that fails the whitelist does not count.",
  "secondary": "time-to-first-proof, PPUT, distinct_price_ratios, non-local-restart counts (descriptive, never the gate)."
 },
 "PARITY_LOCK_2026-06-02_routing_algorithms": "Routing-parity-audit workflow (12 agents, adversarial; EQUAL-RIGOR-PROCEED; commit 329fad1e) found + fixed an information-DEPTH asymmetry AGAINST autonomous and rejected a tune-to-win trap. LOCKED for the counted run: (1) FIXED softmax temperature = boltzmann_temp 0.15, NO tau-annealing — preregistered fixed; adding the Router-v0.2 tau_t schedule (which exists elsewhere in the codebase) is FORBIDDEN here because it would give the forced-router a late-commit edge the autonomous LLM arm structurally lacks (the LLM's sampling temp is fixed 0.7) → a NEW asymmetry favoring softmax. (2) EQUAL REPAIR-DEPTH: autonomous shows the SAME shielded lean-error line (shield_lean_diagnostic, FEEDBACK_MAX=240) for its top-6-by-price nodes that market gets for its one chosen parent — so 'free choice' is not silently paid for in repair quality; both shielded (no raw stderr). (3) PRICE BYTE-IDENTICAL across arms (actor.rs + price_index.rs untouched) — the shared signal cannot favor either arm. (4) ROUTE TELEM

```

---
# PART E — The ONE axiom-confirmed crack (the actual verified proof)

Theorem lm_ineq1 = `ineq_amgm_concrete`: `3*a^2 + 5*b^2 >= 2*Real.sqrt 15 * (a*b)` for a,b >= 0. autonomous (Path-1) produced this proof where single AND market both failed at equal budget. Extracted from the run CAS, recompiled under Lean+Mathlib (it genuinely compiles — the harness is not a false-positive), and `#print axioms ineq_amgm_concrete` = **[propext, Classical.choice, Quot.sound]** subset whitelist = AXIOM-CLEAN (no native_decide / sorryAx).

**The verified proof body:**
```lean
import Mathlib

theorem ineq_amgm_concrete (a b : ℝ) (ha : 0 ≤ a) (hb : 0 ≤ b) :
    3 * a^2 + 5 * b^2 ≥ 2 * Real.sqrt 15 * (a * b) := by
have ha' : a^2 ≥ 0 := pow_two_nonneg a
have hb' : b^2 ≥ 0 := pow_two_nonneg b
have h_nonneg_sqrt : 0 ≤ Real.sqrt 15 := Real.sqrt_nonneg _
-- use AM-GM inequality: (3a^2 + 5b^2)/2 ≥ sqrt(3a^2 * 5b^2) = sqrt(15)*|a*b|
-- Since a,b ≥ 0, |a*b| = a*b.
-- We can apply the inequality (x^2 + y^2)/2 ≥ x*y for x = sqrt(3)*a, y = sqrt(5)*b
set x := Real.sqrt 3 * a with hx_def
set y := Real.sqrt 5 * b with hy_def
have hx_sq : x^2 = 3*a^2 := by
  dsimp [x]
  calc
    (Real.sqrt 3 * a)^2 = (Real.sqrt 3)^2 * a^2 := by ring
    _ = (3 : ℝ) * a^2 := by
      rw [Real.sq_sqrt (show 0 ≤ (3 : ℝ) from by norm_num)]
    _ = 3*a^2 := by ring
have hy_sq : y^2 = 5*b^2 := by
  dsimp [y]
  calc
    (Real.sqrt 5 * b)^2 = (Real.sqrt 5)^2 * b^2 := by ring
    _ = (5 : ℝ) * b^2 := by
      rw [Real.sq_sqrt (show 0 ≤ (5 : ℝ) from by norm_num)]
    _ = 5*b^2 := by ring
have h_nonneg_x : 0 ≤ x := mul_nonneg (Real.sqrt_nonneg _) ha
have h_nonneg_y : 0 ≤ y := mul_nonneg (Real.sqrt_nonneg _) hb
have h_ineq : x^2 + y^2 ≥ 2*x*y := by
  nlinarith [sq_nonneg (x - y)]
calc
  3*a^2 + 5*b^2 = x^2 + y^2 := by rw [hx_sq, hy_sq]
  _ ≥ 2*x*y := h_ineq
  _ = 2*(Real.sqrt 3 * a)*(Real.sqrt 5 * b) := rfl
  _ = 2*(Real.sqrt 3 * Real.sqrt 5)*(a*b) := by ring
  _ = 2*Real.sqrt (3*5)*(a*b) := by rw [Real.sqrt_mul (show 0 ≤ (3:ℝ) from by norm_num) 5]
  _ = 2*Real.sqrt 15*(a*b) := by norm_num
```
*(proof truncated to 50 lines for readability; the full verified body is in the run CAS — it compiles clean and `#print axioms` is exactly the 3 whitelisted axioms.)*

---
# PART F — The claims to audit, the CAUSAL-VALIDITY questions, and your output

## The exact claims (do not assume true)
1. single 0/6, market (forced softmax) 0/6, autonomous 2/6 — autonomous cracked lm_deriv1 + lm_ineq1 where single AND market both failed.
2. One crack (lm_ineq1) is axiom-confirmed (#print axioms ⊆ whitelist, Part E).
3. The cracks are RARE (re-run ~1/9–2/9; the 3-seed run over-estimated).
4. Route telemetry 0% hallucination → the agent genuinely chose nodes.
5. Verdict A = INCONCLUSIVE (axiom-confirmed existence that Path-1 cracks what single+Path-2 can't, but rare + NOT significant, McNemar Holm-p=0.75). Verdict B = HOLDS (54/54 replay-clean). Headline scoped/non-causal.

## The causal-validity questions you must answer (the heart of it — try to BREAK each)
- **A. Confound-shield integrity.** Is "single fails ⇒ model/luck flattened to 0" sound? Does any multi-agent arm get MORE than single — more LLM calls / Lean verifies / tokens / information (compare the prompts in B3 vs B4; check `llm_calls` equal per the budget parity B8)?
- **B. The autonomous-vs-market contrast.** Is the ONLY difference the node-chooser? Is the price byte-identical for both (B2/B5 — does the parity fix touch the price to favor one)? Is repair-depth equal (B4 top-6 shielded diagnostic vs B3 market parent diagnostic)? Could the autonomous full-landscape view be an unfair INFORMATION advantage, or fixed-temp softmax a strawman that disadvantages market?
- **C. Are the cracks REAL?** Re-verify Part E: does the proof compile + `#print axioms ⊆ {propext, Classical.choice, Quot.sound}`? Is the harness "Verified" a real lean exit-0 (B6), and is the missing inline #print-axioms gate a reason to discount the un-confirmed cracks?
- **D. Statistics + over-read.** Is McNemar/Holm (B10) correct and correctly NOT-significant? Is the rarity (re-run 0/6) honestly reported, not buried? Is the verdict free of PROVEN/causal language?
- **E. Replay soundness (Verdict B).** Are all counted cells verify_chaintape replay-clean (economic_state reconstructed from the L4 tape, not a byte badge)?
- **F. Does it WARRANT the claim?** Even if all pass: does a handful of rare cracks warrant "the market organization is a superior solver", or only "Path-1 free-choice can occasionally crack what single+Path-2 cannot, existence-confirmed, not reliable, not significant"? State which the evidence supports.

## Reproduce any cell yourself (with repo access)
`./target/debug/lean_market_agent --runtime-repo R --cas C --run-id r --problem lm_ineq1 --policy {autonomous|market|single} --n-agents 4 --n-rounds 6 --seed 1 --model deepseek-v4-pro --bank tests/fixtures/lean_theorems_pool.jsonl --mathlib-dir <mathlib4> --out r.json` ; then `verify_chaintape --repo R --cas C --run-id r`.

## Your output
Lead with the causal-validity verdict (A–F), each with concrete evidence. Then: numbers reproducible? any baseline crippled/advantaged? cracks real + axiom-clean? verdict honestly hedged? **End: PROCEED** (the scoped INCONCLUSIVE + Verdict-B claim is warranted, causal logic sound as far as it goes), **CHALLENGE** (name the exact confound/gap to close), or **VETO** (a fundamental flaw). We want it broken if it is breakable.
