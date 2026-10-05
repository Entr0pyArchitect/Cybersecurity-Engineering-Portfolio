import json
from pathlib import Path
from datetime import datetime, timezone
ROOT=Path(__file__).resolve().parents[1]; OUT=ROOT/"evidence"/"test_logs"
catalog=json.loads((ROOT/"metadata/query_catalog.json").read_text(encoding="utf-8"))
results=[]
for q in catalog:
    missing=[f for f in ["id","name","technique_id","data_source","query_paths","false_positive_notes"] if not q.get(f)]
    paths_ok=all((ROOT/p).exists() for p in q.get("query_paths",[]))
    results.append({"id":q.get("id"),"missing_fields":missing,"paths_ok":paths_ok,"passed":not missing and paths_ok})
report={"generated_at":datetime.now(timezone.utc).isoformat(),"project":"15-SIEM-Engineering-Query-Library","summary":{"queries_total":len(results),"passed":sum(r["passed"] for r in results)},"results":results}
OUT.mkdir(parents=True,exist_ok=True); path=OUT/f"query_library_validation_{datetime.now(timezone.utc).strftime('%Y%m%d-%H%M%S')}.json"; path.write_text(json.dumps(report,indent=2),encoding="utf-8")
print(json.dumps(report,indent=2)); print(f"\n[+] Validation report written to: {path}")
raise SystemExit(0 if all(r["passed"] for r in results) else 1)
