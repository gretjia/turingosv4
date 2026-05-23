# ST-09 oversize prompt + truncated response — TB-STRESS-PHASE-2

Timestamp (UTC): 20260523T141526Z
Evidence dir: /home/zephryj/projects/turingosv4/handover/evidence/stress_st09_oversize_prompt_20260523T141500Z

## Run notes
mock_port=43635
9a rc=0
9a stdout last 200: 'ok","parsed_envelope":null,"usage":{"prompt_tokens":75,"completion_tokens":75,"total_tokens":150},"finish_reason":"stop","model":"deepseek-ai/DeepSeek-V3.2","prompt_capsule_cid":null,"elapsed_ms":34}\n'
9a stderr last 200: ''
9b rc=2
9b stdout last 200: '{"ok":false,"error":{"kind":"http_status","detail":"HTTP 错误: response decode error: EOF while parsing an object at line 1 column 54"}}\n'
9b stderr last 200: ''
panics: 9a=False 9b=False

## KILL
PASS
