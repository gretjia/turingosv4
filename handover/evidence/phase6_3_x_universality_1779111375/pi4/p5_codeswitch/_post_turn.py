import sys, json, time, urllib.request, urllib.error

SID = sys.argv[1]
turn = int(sys.argv[2])
event = sys.argv[3]   # bootstrap | answer | answer_retry
user_answer = None if sys.argv[4] == "__NULL__" else sys.argv[4]
log_path = sys.argv[5]

body = {"session_id": SID, "user_answer": user_answer, "lang": "zh"}
req = urllib.request.Request(
    "http://127.0.0.1:8080/api/spec/turn",
    data=json.dumps(body).encode("utf-8"),
    headers={"Content-Type": "application/json"},
    method="POST",
)
t0 = time.time()
try:
    with urllib.request.urlopen(req, timeout=180) as r:
        status = r.status
        raw = r.read().decode("utf-8")
except urllib.error.HTTPError as e:
    status = e.code
    raw = e.read().decode("utf-8", errors="replace")
except Exception as e:
    status = -1
    raw = json.dumps({"_exception": str(e)})
elapsed_ms = int((time.time() - t0) * 1000)

try:
    resp = json.loads(raw)
except Exception:
    resp = {"_raw": raw, "_parse_error": True}

entry = {
    "turn": turn,
    "event": event,
    "http": status,
    "elapsed_ms": elapsed_ms,
}
if user_answer is not None:
    entry["user_answer"] = user_answer
entry["response"] = resp

with open(log_path, "a") as f:
    f.write(json.dumps(entry, ensure_ascii=False) + "\n")

qt = resp.get("question_text", "") if isinstance(resp, dict) else ""
print(f"T{turn} {event} HTTP={status} elapsed={elapsed_ms}ms")
print(f"  covered={resp.get('covered_slots')} open={resp.get('open_slots')} conf={resp.get('confidence')} done={resp.get('done')} term={resp.get('terminated')}")
if resp.get("termination_reason"):
    print(f"  termination_reason={resp.get('termination_reason')}")
print(f"  spec_cid={resp.get('spec_capsule_cid')} turn_cid={resp.get('turn_capsule_cid')}")
print(f"  Q: {qt[:240]}")
