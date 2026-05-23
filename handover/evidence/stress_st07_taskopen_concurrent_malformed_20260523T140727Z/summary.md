# ST-07 concurrent malformed task/open — TB-STRESS-PHASE-2

Timestamp (UTC): 20260523T140730Z
Evidence dir: /home/zephryj/projects/turingosv4/handover/evidence/stress_st07_taskopen_concurrent_malformed_20260523T140727Z

## Run notes
server pid=936910 port=8080
requests=100  200=72  502=28  other=0
sample 502 bodies: ['{"reason":"turingos CLI exited 0 but stdout had no parseable task_id; stdout: ERR: gibberish e9d16899360bd4f8\\n","kind":"task_id_parse_failed"}', '{"reason":"turingos CLI exited 0 but stdout had no parseable task_id; stdout: ERR: gibberish 406e997f00bb311e\\n","kind":"task_id_parse_failed"}', '{"reason":"turingos CLI exited 0 but stdout had no parseable task_id; stdout: ERR: gibberish 51bf9fac5fa63ed4\\n","kind":"task_id_parse_failed"}']
server panic=False
expected_502≈30.0  within_tolerance=True

## KILL
PASS
