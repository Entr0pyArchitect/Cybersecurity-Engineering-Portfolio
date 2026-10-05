import json
from pathlib import Path
from datetime import datetime, timezone
ROOT=Path(__file__).resolve().parents[1]; OUT=ROOT/"evidence"/"test_logs"
events=json.loads((ROOT/"testdata/ad_events.json").read_text(encoding="utf-8"))
results=[]
for e in events:
    actual="none"
    if e.get("event_type")=="group_change" and e.get("group") in {"Domain Admins","Enterprise Admins"}: actual="privileged_group_change"
    if e.get("event_type")=="service_account_review" and e.get("spn_count",0)>0 and e.get("password_age_days",0)>365: actual="risky_service_account"
    results.append({"event_id":e["event_id"],"expected":e["expected"],"actual":actual,"passed":actual==e["expected"]})
report={"generated_at":datetime.now(timezone.utc).isoformat(),"project":"11-Active-Directory-Purple-Team-Range","scope":"synthetic AD events only","summary":{"events_total":len(results),"passed":sum(r["passed"] for r in results)},"results":results}
OUT.mkdir(parents=True,exist_ok=True); path=OUT/f"ad_detection_validation_{datetime.now(timezone.utc).strftime('%Y%m%d-%H%M%S')}.json"; path.write_text(json.dumps(report,indent=2),encoding="utf-8")
print(json.dumps(report,indent=2)); print(f"\n[+] Validation report written to: {path}")
raise SystemExit(0 if all(r["passed"] for r in results) else 1)
