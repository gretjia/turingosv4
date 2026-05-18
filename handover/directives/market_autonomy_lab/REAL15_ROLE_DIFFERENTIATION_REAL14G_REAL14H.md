# REAL-15 Role Differentiation Verifier Report

REAL-17 main-CAS integration note: this document is preserved as pre-CAS-repair
historical context only. It is not forward claim-bearing on
`codex/real17-emergence-hardening-20260517`; REAL-17 must regenerate
ChainTape/CAS/verifier evidence on the updated CAS Git commit-chain baseline
before any forward claim can rely on it.

claim_boundary: E3 candidate pending audit
claim_note: candidate-only; not E3 achieved

verdict: Proceed
run_count: 2
audit_tape_proceed_count: 2
persistent_active_role_count: 2
distinct_action_signature_count: 2

role_activity_distribution:
- role=BearTrader turns=39 active_runs=1 tasks=20 work=0 verify=0 challenge=0 buy_yes=0 buy_no=2 exact_market=2 signature=work:0|verify:0|challenge:0|buy_yes:0|buy_no:2
- role=BullTrader turns=39 active_runs=2 tasks=20 work=0 verify=0 challenge=0 buy_yes=19 buy_no=0 exact_market=19 signature=work:0|verify:0|challenge:0|buy_yes:19|buy_no:0
- role=Challenger turns=37 active_runs=0 tasks=19 work=0 verify=0 challenge=0 buy_yes=0 buy_no=0 exact_market=0 signature=work:0|verify:0|challenge:0|buy_yes:0|buy_no:0
- role=MarketMaker turns=0 active_runs=0 tasks=0 work=0 verify=0 challenge=0 buy_yes=0 buy_no=0 exact_market=0 signature=work:0|verify:0|challenge:0|buy_yes:0|buy_no:0
- role=Observer turns=0 active_runs=0 tasks=0 work=0 verify=0 challenge=0 buy_yes=0 buy_no=0 exact_market=0 signature=work:0|verify:0|challenge:0|buy_yes:0|buy_no:0
- role=Solver turns=37 active_runs=2 tasks=19 work=34 verify=0 challenge=0 buy_yes=0 buy_no=0 exact_market=0 signature=work:34|verify:0|challenge:0|buy_yes:0|buy_no:0
- role=Trader turns=0 active_runs=0 tasks=0 work=0 verify=0 challenge=0 buy_yes=0 buy_no=0 exact_market=0 signature=work:0|verify:0|challenge:0|buy_yes:0|buy_no:0
- role=Verifier turns=37 active_runs=1 tasks=19 work=0 verify=2 challenge=0 buy_yes=0 buy_no=0 exact_market=0 signature=work:0|verify:2|challenge:0|buy_yes:0|buy_no:0

residual_risks:
- run REAL-14G tx router-task-outcome-Agent_0-task-n5_amc12_2000_p12_1778984985337-Agent_0-0 verifier residual risks: 2
- run REAL-14G tx router-task-outcome-Agent_0-task-n5_amc12_2000_p12_1778984985337-Agent_0-5 verifier residual risks: 2
- run REAL-14G tx router-task-outcome-Agent_0-task-n5_amc12_2000_p6_1778985024025-Agent_0-0 verifier residual risks: 1
- run REAL-14G tx router-task-outcome-Agent_0-task-n5_mathd_algebra_208_1778985137183-Agent_0-0 verifier residual risks: 1
- run REAL-14G tx router-task-outcome-Agent_0-task-n5_mathd_algebra_246_1778985171418-Agent_0-0 verifier residual risks: 1
- run REAL-14G tx router-task-outcome-Agent_0-task-n5_mathd_algebra_270_1778985187029-Agent_0-0 verifier residual risks: 1
- run REAL-14G tx router-task-outcome-Agent_0-task-n5_mathd_algebra_332_1778985237848-Agent_0-5 verifier residual risks: 1
- run REAL-14G tx router-task-outcome-Agent_0-task-n5_numbertheory_2pownm1prime_nprime_1778985313104-Agent_0-0 verifier residual risks: 1
- run REAL-14H tx router-task-outcome-Agent_0-task-n5_algebra_bleqa_apbon2msqrtableqambsqon8b_1778986526515-Agent_0-5 verifier residual risks: 1
- run REAL-14H tx router-task-outcome-Agent_0-task-n5_amc12_2000_p12_1778986645493-Agent_0-0 verifier residual risks: 1
- run REAL-14H tx router-task-outcome-Agent_0-task-n5_amc12_2000_p6_1778986728883-Agent_0-5 verifier residual risks: 1
- run REAL-14H tx router-task-outcome-Agent_0-task-n5_imo_1962_p2_1778986885024-Agent_0-0 verifier residual risks: 1
- run REAL-14H tx router-task-outcome-Agent_0-task-n5_mathd_algebra_208_1778986920491-Agent_0-0 verifier residual risks: 2
- run REAL-14H tx router-task-outcome-Agent_0-task-n5_mathd_algebra_208_1778986920491-Agent_0-5 verifier residual risks: 2
- run REAL-14H tx router-task-outcome-Agent_0-task-n5_mathd_algebra_246_1778986957091-Agent_0-0 verifier residual risks: 2
- run REAL-14H tx router-task-outcome-Agent_0-task-n5_mathd_algebra_246_1778986957091-Agent_0-5 verifier residual risks: 2
- run REAL-14H tx router-task-outcome-Agent_0-task-n5_mathd_algebra_270_1778987003803-Agent_0-0 verifier residual risks: 1
- run REAL-14H tx router-task-outcome-Agent_0-task-n5_mathd_algebra_332_1778987057463-Agent_0-0 verifier residual risks: 2
- run REAL-14H tx router-task-outcome-Agent_0-task-n5_mathd_algebra_332_1778987057463-Agent_0-5 verifier residual risks: 2
- run REAL-14H tx router-task-outcome-Agent_1-task-n5_amc12_2000_p12_1778986645493-Agent_1-6 verifier residual risks: 1
- run REAL-14H tx router-task-outcome-Agent_1-task-n5_mathd_algebra_208_1778986920491-Agent_1-1 verifier residual risks: 1
