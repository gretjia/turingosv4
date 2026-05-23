# ST-09 oversize prompt + truncated response — TB-STRESS-PHASE-2

Timestamp (UTC): 20260523T140008Z
Evidence dir: /home/zephryj/projects/turingosv4/handover/evidence/stress_st09_oversize_prompt_20260523T135949Z

## Run notes
mock_port=41755
9a rc=4
9a stdout last 200: '{"ok":false,"error":{"kind":"io","detail":"IO 错误: prompt file JSON parse error: expected value at line 1 column 1"}}\n'
9a stderr last 200: ''
9b rc=4
9b stdout last 200: '{"ok":false,"error":{"kind":"io","detail":"IO 错误: prompt file JSON parse error: expected value at line 1 column 1"}}\n'
9b stderr last 200: ''
panics: 9a=False 9b=False

## KILL
PASS
