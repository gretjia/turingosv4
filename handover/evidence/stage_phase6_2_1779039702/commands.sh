+ ./target/debug/turingos init --project /tmp/phase6_2_witness_1779039702
+ ./target/debug/turingos agent deploy --id agent_001 --pubkey 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef --role Solver --workspace /tmp/phase6_2_witness_1779039702
+ ./target/debug/turingos config set demo.key demo.value --workspace /tmp/phase6_2_witness_1779039702
+ ./target/debug/turingos config get demo.key --workspace /tmp/phase6_2_witness_1779039702
+ ./target/debug/turingos task open --problem nat_succ_succ --bounty 1000000 --workspace /tmp/phase6_2_witness_1779039702
+ ./target/debug/turingos audit dashboard --chaintape /tmp/phase6_2_witness_1779039702/runtime_repo
+ ./target/debug/turingos audit dashboard --repo /tmp/phase6_2_witness_1779039702/runtime_repo --cas /tmp/phase6_2_witness_1779039702/cas
+ ./target/debug/turingos report wallet --chaintape /tmp/phase6_2_witness_1779039702/runtime_repo
+ ./target/debug/turingos export evidence --source /tmp/phase6_2_witness_1779039702 --out /tmp/phase6_2_export_1779039702
+ ./target/debug/turingos replay --chaintape /tmp/phase6_2_witness_1779039702/runtime_repo
+ ./target/debug/turingos render --fixture experiments/tisr_ui_spike/fixtures/dashboard_sample.json
+ python3 experiments/tisr_ui_spike/validate.py --fixture experiments/tisr_ui_spike/fixtures/agent_role_view_sample.json
+ bash experiments/tisr_ui_spike/test_render.sh
+ bash experiments/tisr_ui_spike/test_validate.sh
