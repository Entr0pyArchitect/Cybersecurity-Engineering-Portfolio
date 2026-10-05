import json
from pathlib import Path
from datetime import datetime, timezone
ROOT=Path(__file__).resolve().parents[1]; OUT=ROOT/"evidence"/"test_logs"
events=json.loads((ROOT/"testdata/deception_events.json").read_text(encoding="utf-8"))
results=[]
for e in events:
    actual="decoy_touched" if e["asset"].startswith("decoy") or e["asset"].startswith("fake") else "none"
    results.append({"event_id":e["event_id"],"expected":e["expected"],"actual":actual,"passed":actual==e["expected"]})
report={"generated_at":datetime.now(timezone.utc).isoformat(),"project":"16-Deception-Honeypot-Research-Grid","scope":"synthetic deception events only","summary":{"events_total":len(results),"passed":sum(r["passed"] for r in results)},"results":results}
OUT.mkdir(parents=True,exist_ok=True); path=OUT/f"deception_validation_{datetime.now(timezone.utc).strftime('%Y%m%d-%H%M%S')}.json"; path.write_text(json.dumps(report,indent=2),encoding="utf-8")
print(json.dumps(report,indent=2)); print(f"\n[+] Validation report written to: {path}")
raise SystemExit(0 if all(r["passed"] for r in results) else 1)
