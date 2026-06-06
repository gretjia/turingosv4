use turingosv4::bottom_white::cas::Cid;
use turingosv4::runtime::agent_scheduler::{build_parallel_lane_views, ParallelLaneInput};

#[test]
fn parallel_lane_views_share_public_prefix_but_not_private_error_context() {
    let candidate_set_cid = Cid::from_content(b"public-candidate-set");
    let inputs = vec![
        ParallelLaneInput {
            lane_id: "lane-a".to_string(),
            public_tape_head: "git-head-1".to_string(),
            public_candidate_set_cid: candidate_set_cid,
            private_error_context: Some("lane-a-secret-lean-stderr".to_string()),
        },
        ParallelLaneInput {
            lane_id: "lane-b".to_string(),
            public_tape_head: "git-head-1".to_string(),
            public_candidate_set_cid: candidate_set_cid,
            private_error_context: Some("lane-b-secret-autopsy".to_string()),
        },
    ];

    let views = build_parallel_lane_views(&inputs).expect("lane views");
    assert_eq!(views.len(), 2);
    assert_eq!(views[0].public_tape_head, "git-head-1");
    assert_eq!(views[1].public_tape_head, "git-head-1");
    assert_eq!(views[0].public_candidate_set_cid, candidate_set_cid);
    assert_eq!(views[1].public_candidate_set_cid, candidate_set_cid);

    let debug = format!("{views:?}");
    assert!(!debug.contains("lane-a-secret-lean-stderr"));
    assert!(!debug.contains("lane-b-secret-autopsy"));
}

#[test]
fn parallel_lane_views_reject_duplicate_lane_ids() {
    let candidate_set_cid = Cid::from_content(b"public-candidate-set");
    let inputs = vec![
        ParallelLaneInput {
            lane_id: "lane-a".to_string(),
            public_tape_head: "git-head-1".to_string(),
            public_candidate_set_cid: candidate_set_cid,
            private_error_context: None,
        },
        ParallelLaneInput {
            lane_id: "lane-a".to_string(),
            public_tape_head: "git-head-1".to_string(),
            public_candidate_set_cid: candidate_set_cid,
            private_error_context: None,
        },
    ];

    let err = build_parallel_lane_views(&inputs).expect_err("duplicate lane");
    assert!(err.to_string().contains("duplicate_lane_id"));
}
