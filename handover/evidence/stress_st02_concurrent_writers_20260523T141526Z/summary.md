# ST-02 3-concurrent kernel writers — TB-STRESS-PHASE-2

Timestamp (UTC): 20260523T141527Z
Evidence dir: /home/zephryj/projects/turingosv4/handover/evidence/stress_st02_concurrent_writers_20260523T141526Z

## Run notes
mock_endpoint=http://127.0.0.1:42583/v1/chat/completions  writers=2  attempts=10
writer 0 exit=0
writer 1 exit=0
  writer 0: ~10 attempts visible in log
  writer 1: ~10 attempts visible in log
cas sidecar entries: 0
attempt completion ratio: 1.00  expected=20
panics=0  kill_pass=True

## KILL
PASS
