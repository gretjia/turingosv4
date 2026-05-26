# REAL-14 E2 Candidate Verifier Report

claim_boundary: not an E2 candidate

verdict: Veto
l4_router_tx_count: 2
submitted_trace_tx_count: 1
exact_join_count: 1
duplicate_l4_router_tx_id_count: 0
duplicate_submitted_trace_tx_id_count: 0
scripted_fixture_tx_count: 0
policy_counts_for_e2: false
direct_prompt_capsule_provenance_count: 0
indirect_prompt_capsule_provenance_count: 0
missing_direct_prompt_capsule_provenance_count: 1

matched_tx_ids:
- router-Agent_1-full-system-augment

matched_tx_provenance:
- tx_id=router-Agent_1-full-system-augment actor=Agent_1 role=None ev=0 opportunity=0 prompt_link=missing role_turn=0 residual_risks=0

bcast_shielding:
  verdict: PASS
  digest_count: 0
  role_crop_count: 0
  visible_context_count: 0
  failure_count: 0

failure_reasons:
- matched tx router-Agent_1-full-system-augment has no EVDecisionTrace/economic rationale
- matched tx router-Agent_1-full-system-augment has no MarketOpportunityTrace
- matched tx router-Agent_1-full-system-augment has no PromptCapsule/visible-context linkage
- matched tx router-Agent_1-full-system-augment actor is not a live trader-like agent role
