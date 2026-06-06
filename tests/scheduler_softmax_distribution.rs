use std::collections::BTreeSet;

use turingosv4::runtime::agent_scheduler::{
    argmax_candidate_for_positive_control, softmax_select_candidate, SchedulerCandidate,
    SoftmaxConfig,
};
use turingosv4::state::price_index::RationalPrice;

fn equal_price_candidate(id: &str) -> SchedulerCandidate {
    SchedulerCandidate {
        id: id.to_string(),
        price: Some(RationalPrice {
            numerator: 1,
            denominator: 2,
        }),
        public_context_cid: None,
    }
}

#[test]
fn equal_price_softmax_spreads_across_at_least_three_of_five_candidates() {
    let candidates: Vec<SchedulerCandidate> = (0..5)
        .map(|i| equal_price_candidate(&format!("candidate-{i}")))
        .collect();
    let config = SoftmaxConfig {
        temperature_milli: 1_000,
    };

    let selected: BTreeSet<String> = (0..80)
        .map(|seed| {
            softmax_select_candidate(&candidates, config, seed)
                .expect("softmax selection")
                .id
        })
        .collect();

    assert!(
        selected.len() >= 3,
        "equal-price scheduler policy must remain distributional, not collapse to argmax: {selected:?}"
    );
}

#[test]
fn argmax_positive_control_collapses_equal_price_set() {
    let candidates: Vec<SchedulerCandidate> = (0..5)
        .map(|i| equal_price_candidate(&format!("candidate-{i}")))
        .collect();
    let selected: BTreeSet<String> = (0..80)
        .map(|_| {
            argmax_candidate_for_positive_control(&candidates)
                .expect("argmax selection")
                .id
        })
        .collect();

    assert_eq!(
        selected.len(),
        1,
        "positive control must demonstrate the collapse A11 forbids"
    );
}

#[test]
fn softmax_rejects_zero_temperature_and_empty_candidates() {
    let candidates = vec![equal_price_candidate("candidate-0")];
    let err = softmax_select_candidate(
        &candidates,
        SoftmaxConfig {
            temperature_milli: 0,
        },
        1,
    )
    .expect_err("zero temperature");
    assert!(err.to_string().contains("invalid_temperature"));

    let err = softmax_select_candidate(
        &[],
        SoftmaxConfig {
            temperature_milli: 1_000,
        },
        1,
    )
    .expect_err("empty candidates");
    assert!(err.to_string().contains("no_candidates"));
}
