# ST-05 BuildSessionView corruption — TB-STRESS-PHASE-2

Timestamp (UTC): 20260523T135830Z
Evidence dir: /home/zephryj/projects/turingosv4/handover/evidence/stress_st05_buildsession_corruption_20260523T135534Z

## Run notes
write rc=0
wrote 100 capsules
corrupted 0 of 10 target capsules
read rc=0
read stdout (full):
ERR_DECODE st05_sess_0 spec_grill_session c046e9cc8fb8942c8450711e1c50074bb61213da847622e6601ea443f47d8973: missing field `termination_reason` at line 1 column 312
ERR_DECODE st05_sess_1 spec_grill_session c046e9cc8fb8942c8450711e1c50074bb61213da847622e6601ea443f47d8973: missing field `termination_reason` at line 1 column 312
ERR_DECODE st05_sess_2 spec_grill_session c046e9cc8fb8942c8450711e1c50074bb61213da847622e6601ea443f47d8973: missing field `termination_reason` at line 1 column 312
ERR_DECODE st05_sess_3 spec_grill_session c046e9cc8fb8942c8450711e1c50074bb61213da847622e6601ea443f47d8973: missing field `termination_reason` at line 1 column 312
ERR_DECODE st05_sess_4 spec_grill_session c046e9cc8fb8942c8450711e1c50074bb61213da847622e6601ea443f47d8973: missing field `termination_reason` at line 1 column 312

read stderr (last 500): 
panic_detected=False
OK=0  ERR_DECODE=5  ERR_OPEN+READ=0

## KILL
PASS
