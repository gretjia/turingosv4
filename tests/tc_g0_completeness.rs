use turingosv4::runtime::g0_completeness::{
    enumerate_candidates, prioritize_odd_by_market, run_dovetail, Candidate, G0Manifest, Lane,
};

#[test]
fn g0_manifest_rejects_unrestricted_or_hidden_automation() {
    let err = G0Manifest::new(
        "g0-test",
        vec!["intro".to_string(), "native_decide".to_string()],
        vec![],
    )
    .unwrap_err();
    assert!(err.contains("native_decide"));

    let err =
        G0Manifest::new("g0-test", vec!["raw:exact arbitrary".to_string()], vec![]).unwrap_err();
    assert!(err.contains("raw"));

    for hidden in ["decide", "omega"] {
        let err = G0Manifest::new("g0-test", vec![hidden.to_string()], vec![]).unwrap_err();
        assert!(
            err.contains(hidden),
            "expected {hidden} to be rejected, got {err}"
        );
    }
}

#[test]
fn g0_manifest_rejects_empty_duplicate_or_unlocked_atoms() {
    let empty = G0Manifest::new("g0-test", vec!["".to_string()], vec![]).unwrap_err();
    assert!(empty.contains("empty"));

    let duplicate = G0Manifest::new(
        "g0-test",
        vec!["intro".to_string(), "intro".to_string()],
        vec![],
    )
    .unwrap_err();
    assert!(duplicate.contains("duplicate"));

    let unlocked = G0Manifest::new("g0-test", vec!["constructor".to_string()], vec![]).unwrap_err();
    assert!(unlocked.contains("outside locked G0"));

    let lemma = G0Manifest::new("g0-test", vec![], vec!["hidden_lemma".to_string()]).unwrap_err();
    assert!(lemma.contains("outside locked G0"));
}

#[test]
fn enumeration_is_rank_then_digest_stable_and_complete() {
    let manifest = G0Manifest::new(
        "g0-test",
        vec!["intro".to_string(), "exact h".to_string()],
        vec!["lemma_a".to_string()],
    )
    .unwrap();
    let candidates = enumerate_candidates(&manifest, u64::MAX);

    assert_eq!(candidates.len(), 3);
    assert!(candidates
        .windows(2)
        .all(|w| { (w[0].rank, w[0].digest.as_str()) <= (w[1].rank, w[1].digest.as_str()) }));
    let intro = candidates
        .iter()
        .find(|c| c.lean_text == "intro")
        .expect("intro candidate");
    assert_eq!(intro.ast_canonical, "g0:tactic:intro");
    let exact_h = candidates
        .iter()
        .find(|c| c.lean_text == "exact h")
        .expect("exact h candidate");
    assert_eq!(exact_h.ast_canonical, "g0:tactic:exact h");
    let lemma = candidates
        .iter()
        .find(|c| c.lean_text == "exact lemma_a")
        .expect("lemma candidate");
    assert_eq!(lemma.ast_canonical, "g0:exact_lemma:lemma_a");
    assert!(lemma.rank > intro.rank);
}

#[test]
fn even_lane_trace_is_identical_under_poisoned_odd_queue() {
    let manifest = G0Manifest::new(
        "g0-test",
        vec!["intro".to_string(), "exact h".to_string()],
        vec![],
    )
    .unwrap();
    let candidates = enumerate_candidates(&manifest, u64::MAX);
    let odd_a = vec![
        Candidate::heuristic("poison-a"),
        Candidate::heuristic("poison-b"),
    ];
    let odd_b = vec![
        Candidate::heuristic("very-expensive"),
        Candidate::heuristic("rank=0"),
    ];

    let trace_a = run_dovetail(candidates.clone(), odd_a, 6);
    let trace_b = run_dovetail(candidates, odd_b, 6);

    let even_a: Vec<_> = trace_a
        .iter()
        .filter(|t| t.lane == Lane::EvenEnumerator)
        .map(|t| (t.tick, t.candidate_digest.clone(), t.rank, t.action.clone()))
        .collect();
    let even_b: Vec<_> = trace_b
        .iter()
        .filter(|t| t.lane == Lane::EvenEnumerator)
        .map(|t| (t.tick, t.candidate_digest.clone(), t.rank, t.action.clone()))
        .collect();

    assert_eq!(even_a, even_b);
    assert_eq!(even_a[0].0, 0);
    assert_eq!(even_a[1].0, 2);
}

#[test]
fn odd_duplicate_digest_does_not_mask_even_candidate() {
    let even = vec![Candidate::from_g0_text("intro")];
    let odd = vec![Candidate::heuristic_with_digest(
        "poison",
        even[0].digest.clone(),
    )];

    let trace = run_dovetail(even.clone(), odd, 3);
    let even_attempt = trace
        .iter()
        .find(|t| {
            t.lane == Lane::EvenEnumerator && t.candidate_digest == Some(even[0].digest.clone())
        })
        .expect("even candidate still attempted");

    assert_eq!(even_attempt.tick, 0);
    assert_eq!(even_attempt.action, "attempt");
    assert_eq!(even_attempt.duplicate_of_tick, None);
}

#[test]
fn even_duplicate_records_first_even_trace_pointer() {
    let c = Candidate::from_g0_text("intro");
    let trace = run_dovetail(vec![c.clone(), c.clone()], vec![], 4);

    let duplicates: Vec<_> = trace
        .iter()
        .filter(|t| t.lane == Lane::EvenEnumerator && t.action == "covered_by_prior_even_attempt")
        .collect();

    assert_eq!(duplicates.len(), 1);
    assert_eq!(duplicates[0].tick, 2);
    assert_eq!(duplicates[0].duplicate_of_tick, Some(0));
}

#[test]
fn poisoned_high_price_odd_queue_cannot_skip_pop_reorder_or_mask_even() {
    let even = vec![
        Candidate::from_g0_text("intro"),
        Candidate::from_g0_text("exact h"),
        Candidate::from_g0_text("apply h"),
    ];
    let poison = prioritize_odd_by_market(vec![
        Candidate::heuristic_with_market("dead-low", 1),
        Candidate::heuristic_with_market("dead-high", 10_000),
        Candidate::heuristic_with_digest("dead-duplicate", even[1].digest.clone()),
    ]);

    let trace = run_dovetail(even.clone(), poison, 8);
    let even_trace: Vec<_> = trace
        .iter()
        .filter(|t| t.lane == Lane::EvenEnumerator)
        .map(|t| (t.tick, t.candidate_digest.clone(), t.action.clone()))
        .collect();

    assert_eq!(
        even_trace[0],
        (0, Some(even[0].digest.clone()), "attempt".to_string())
    );
    assert_eq!(
        even_trace[1],
        (2, Some(even[1].digest.clone()), "attempt".to_string())
    );
    assert_eq!(
        even_trace[2],
        (4, Some(even[2].digest.clone()), "attempt".to_string())
    );
}

#[test]
fn odd_queue_exhaustion_cannot_change_even_schedule() {
    let even = vec![
        Candidate::from_g0_text("intro"),
        Candidate::from_g0_text("exact h"),
    ];
    let with_odd = run_dovetail(even.clone(), vec![Candidate::heuristic("x")], 6);
    let without_odd = run_dovetail(even.clone(), vec![], 6);

    let even_with_odd: Vec<_> = with_odd
        .iter()
        .filter(|t| t.lane == Lane::EvenEnumerator)
        .map(|t| (t.tick, t.candidate_digest.clone(), t.action.clone()))
        .collect();
    let even_without_odd: Vec<_> = without_odd
        .iter()
        .filter(|t| t.lane == Lane::EvenEnumerator)
        .map(|t| (t.tick, t.candidate_digest.clone(), t.action.clone()))
        .collect();

    assert_eq!(even_with_odd, even_without_odd);
}

#[test]
fn market_price_is_odd_lane_metadata_only() {
    let trace = run_dovetail(
        vec![Candidate::from_g0_text("intro")],
        vec![Candidate::heuristic_with_market("market-proposal", 42)],
        2,
    );
    let odd = trace
        .iter()
        .find(|t| t.lane == Lane::OddHeuristic)
        .expect("odd trace exists");

    assert_eq!(odd.market_price, Some(42));
    assert_eq!(odd.rank, None);
    assert_eq!(odd.verifier_acceptance, None);
}

#[test]
fn market_price_cannot_change_g0_rank_digest_or_even_schedule() {
    let manifest = G0Manifest::new(
        "g0-test",
        vec!["intro".to_string(), "exact h".to_string()],
        vec![],
    )
    .unwrap();
    let even = enumerate_candidates(&manifest, u64::MAX);
    let baseline = run_dovetail(even.clone(), vec![], 6);
    let priced = run_dovetail(
        even.clone(),
        prioritize_odd_by_market(vec![
            Candidate::heuristic_with_market("odd-a", 1),
            Candidate::heuristic_with_market("odd-b", 9_999),
        ]),
        6,
    );

    let baseline_even: Vec<_> = baseline
        .iter()
        .filter(|t| t.lane == Lane::EvenEnumerator)
        .map(|t| (t.tick, t.candidate_digest.clone(), t.rank, t.action.clone()))
        .collect();
    let priced_even: Vec<_> = priced
        .iter()
        .filter(|t| t.lane == Lane::EvenEnumerator)
        .map(|t| (t.tick, t.candidate_digest.clone(), t.rank, t.action.clone()))
        .collect();

    assert_eq!(baseline_even, priced_even);
    assert!(even.iter().all(|c| c.market_price.is_none()));
}

#[test]
fn market_price_cannot_create_verifier_acceptance() {
    let trace = run_dovetail(
        vec![],
        vec![Candidate::heuristic_with_market_and_claimed_acceptance(
            "priced", 7, true,
        )],
        2,
    );
    let odd = trace
        .iter()
        .find(|t| t.lane == Lane::OddHeuristic)
        .expect("odd trace exists");

    assert_eq!(odd.verdict, "not_enumerator_authority");
    assert_eq!(odd.verifier_acceptance, None);
}

#[test]
fn autonomous_route_is_odd_lane_proposal_only() {
    let trace = run_dovetail(
        vec![Candidate::from_g0_text("intro")],
        vec![Candidate::heuristic_with_route("routed", "route-a")],
        2,
    );
    let odd = trace
        .iter()
        .find(|t| t.lane == Lane::OddHeuristic)
        .expect("odd trace exists");

    assert_eq!(odd.autonomous_route.as_deref(), Some("route-a"));
    assert_eq!(odd.rank, None);
    assert_eq!(odd.verifier_acceptance, None);
}

#[test]
fn autonomous_route_cannot_mutate_even_queue() {
    let even = vec![
        Candidate::from_g0_text("intro"),
        Candidate::from_g0_text("exact h"),
    ];
    let routed = run_dovetail(
        even.clone(),
        vec![Candidate::heuristic_with_route(
            "route tries to skip",
            "skip-even",
        )],
        6,
    );
    let baseline = run_dovetail(even, vec![], 6);

    let routed_even: Vec<_> = routed
        .iter()
        .filter(|t| t.lane == Lane::EvenEnumerator)
        .map(|t| (t.tick, t.candidate_digest.clone(), t.action.clone()))
        .collect();
    let baseline_even: Vec<_> = baseline
        .iter()
        .filter(|t| t.lane == Lane::EvenEnumerator)
        .map(|t| (t.tick, t.candidate_digest.clone(), t.action.clone()))
        .collect();

    assert_eq!(routed_even, baseline_even);
}

#[test]
fn autonomous_route_cannot_override_verifier_rejection() {
    let trace = run_dovetail(
        vec![],
        vec![Candidate::heuristic_with_route_and_claimed_acceptance(
            "route-claims-pass",
            "route-pass",
            true,
        )],
        2,
    );
    let odd = trace
        .iter()
        .find(|t| t.lane == Lane::OddHeuristic)
        .expect("odd trace exists");

    assert_eq!(odd.verdict, "not_enumerator_authority");
    assert_eq!(odd.verifier_acceptance, None);
}
