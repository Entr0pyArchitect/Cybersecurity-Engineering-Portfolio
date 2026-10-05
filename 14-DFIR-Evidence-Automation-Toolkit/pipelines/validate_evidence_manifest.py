import json, hashlib
from pathlib import Path
from datetime import datetime, timezone
ROOT=Path(__file__).resolve().parents[1]; OUT=ROOT/"evidence"/"test_logs"; MAN=ROOT/"manifests/evidence_manifest.json"
items=[]
for path in sorted((ROOT/"evidence-samples").glob("*")):
    if path.is_file():
        digest=hashlib.sha256(path.read_bytes()).hexdigest()
        items.append({"file":str(path.relative_to(ROOT)),"sha256":digest,"size_bytes":path.stat().st_size})
MAN.parent.mkdir(parents=True,exist_ok=True); MAN.write_text(json.dumps({"items":items},indent=2),encoding="utf-8")
report={"generated_at":datetime.now(timezone.utc).isoformat(),"project":"14-DFIR-Evidence-Automation-Toolkit","summary":{"files_total":len(items),"manifest_created":True},"items":items}
OUT.mkdir(parents=True,exist_ok=True); out=OUT/f"evidence_manifest_validation_{datetime.now(timezone.utc).strftime('%Y%m%d-%H%M%S')}.json"; out.write_text(json.dumps(report,indent=2),encoding="utf-8")
print(json.dumps(report,indent=2)); print(f"\n[+] Validation report written to: {out}")
raise SystemExit(0 if len(items)>0 else 1)
