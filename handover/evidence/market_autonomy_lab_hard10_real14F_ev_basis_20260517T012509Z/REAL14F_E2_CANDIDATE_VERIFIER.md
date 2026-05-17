# REAL-14 E2 Candidate Verifier Report

claim_boundary: E2 candidate pending audit

verdict: Proceed
l4_router_tx_count: 3
submitted_trace_tx_count: 3
exact_join_count: 3
duplicate_l4_router_tx_id_count: 0
duplicate_submitted_trace_tx_id_count: 0
scripted_fixture_tx_count: 0
policy_counts_for_e2: false

matched_tx_ids:
- router-task-outcome-Agent_0-task-n5_mathd_algebra_208_1778981591894-Agent_0-0
- router-task-outcome-Agent_0-task-n5_mathd_algebra_246_1778981626960-Agent_0-0
- router-task-outcome-Agent_0-task-n5_mathd_algebra_332_1778981694822-Agent_0-0

matched_tx_provenance:
- tx_id=router-task-outcome-Agent_0-task-n5_mathd_algebra_208_1778981591894-Agent_0-0 actor=Agent_0 role=Some("BullTrader") ev=1 opportunity=1 prompt_link=indirect_via_ev_decision_trace role_turn=1 residual_risks=1
- tx_id=router-task-outcome-Agent_0-task-n5_mathd_algebra_246_1778981626960-Agent_0-0 actor=Agent_0 role=Some("BullTrader") ev=1 opportunity=1 prompt_link=indirect_via_ev_decision_trace role_turn=1 residual_risks=1
- tx_id=router-task-outcome-Agent_0-task-n5_mathd_algebra_332_1778981694822-Agent_0-0 actor=Agent_0 role=Some("BullTrader") ev=1 opportunity=1 prompt_link=indirect_via_ev_decision_trace role_turn=1 residual_risks=1

bcast_shielding:
  verdict: PASS
  digest_count: 91
  role_crop_count: 91
  visible_context_count: 91
  failure_count: 0
