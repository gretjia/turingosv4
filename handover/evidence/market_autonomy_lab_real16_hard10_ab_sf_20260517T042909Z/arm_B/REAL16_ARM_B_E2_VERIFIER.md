# REAL-14 E2 Candidate Verifier Report

claim_boundary: not an E2 candidate

verdict: Veto
l4_router_tx_count: 7
submitted_trace_tx_count: 7
exact_join_count: 7
duplicate_l4_router_tx_id_count: 0
duplicate_submitted_trace_tx_id_count: 0
scripted_fixture_tx_count: 0
policy_counts_for_e2: false

matched_tx_ids:
- router-task-outcome-Agent_0-task-n5_algebra_bleqa_apbon2msqrtableqambsqon8b_1778996349193-Agent_0-5
- router-task-outcome-Agent_0-task-n5_amc12_2000_p12_1778997082157-Agent_0-20
- router-task-outcome-Agent_0-task-n5_mathd_algebra_246_1778999992725-Agent_0-15
- router-task-outcome-Agent_0-task-n5_mathd_algebra_246_1778999992725-Agent_0-20
- router-task-outcome-Agent_0-task-n5_mathd_algebra_246_1778999992725-Agent_0-25
- router-task-outcome-Agent_0-task-n5_mathd_algebra_246_1778999992725-Agent_0-5
- router-task-outcome-Agent_1-task-n5_mathd_algebra_332_1779000643207-Agent_1-1

matched_tx_provenance:
- tx_id=router-task-outcome-Agent_0-task-n5_algebra_bleqa_apbon2msqrtableqambsqon8b_1778996349193-Agent_0-5 actor=Agent_0 role=Some("BullTrader") ev=1 opportunity=0 prompt_link=indirect_via_ev_decision_trace role_turn=1 residual_risks=1
- tx_id=router-task-outcome-Agent_0-task-n5_amc12_2000_p12_1778997082157-Agent_0-20 actor=Agent_0 role=Some("BullTrader") ev=1 opportunity=0 prompt_link=indirect_via_ev_decision_trace role_turn=1 residual_risks=1
- tx_id=router-task-outcome-Agent_0-task-n5_mathd_algebra_246_1778999992725-Agent_0-15 actor=Agent_0 role=Some("BullTrader") ev=4 opportunity=0 prompt_link=indirect_via_ev_decision_trace role_turn=4 residual_risks=2
- tx_id=router-task-outcome-Agent_0-task-n5_mathd_algebra_246_1778999992725-Agent_0-20 actor=Agent_0 role=Some("BullTrader") ev=4 opportunity=0 prompt_link=indirect_via_ev_decision_trace role_turn=4 residual_risks=2
- tx_id=router-task-outcome-Agent_0-task-n5_mathd_algebra_246_1778999992725-Agent_0-25 actor=Agent_0 role=Some("BullTrader") ev=4 opportunity=0 prompt_link=indirect_via_ev_decision_trace role_turn=4 residual_risks=2
- tx_id=router-task-outcome-Agent_0-task-n5_mathd_algebra_246_1778999992725-Agent_0-5 actor=Agent_0 role=Some("BullTrader") ev=4 opportunity=0 prompt_link=indirect_via_ev_decision_trace role_turn=4 residual_risks=2
- tx_id=router-task-outcome-Agent_1-task-n5_mathd_algebra_332_1779000643207-Agent_1-1 actor=Agent_1 role=Some("BearTrader") ev=1 opportunity=0 prompt_link=indirect_via_ev_decision_trace role_turn=1 residual_risks=1

bcast_shielding:
  verdict: PASS
  digest_count: 0
  role_crop_count: 0
  visible_context_count: 259
  failure_count: 0

failure_reasons:
- matched tx router-task-outcome-Agent_0-task-n5_algebra_bleqa_apbon2msqrtableqambsqon8b_1778996349193-Agent_0-5 has no MarketOpportunityTrace
- matched tx router-task-outcome-Agent_0-task-n5_amc12_2000_p12_1778997082157-Agent_0-20 has no MarketOpportunityTrace
- matched tx router-task-outcome-Agent_0-task-n5_mathd_algebra_246_1778999992725-Agent_0-15 has no MarketOpportunityTrace
- matched tx router-task-outcome-Agent_0-task-n5_mathd_algebra_246_1778999992725-Agent_0-20 has no MarketOpportunityTrace
- matched tx router-task-outcome-Agent_0-task-n5_mathd_algebra_246_1778999992725-Agent_0-25 has no MarketOpportunityTrace
- matched tx router-task-outcome-Agent_0-task-n5_mathd_algebra_246_1778999992725-Agent_0-5 has no MarketOpportunityTrace
- matched tx router-task-outcome-Agent_1-task-n5_mathd_algebra_332_1779000643207-Agent_1-1 has no MarketOpportunityTrace
