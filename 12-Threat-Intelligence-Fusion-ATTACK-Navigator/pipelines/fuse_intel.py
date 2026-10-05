import json
from pathlib import Path
from datetime import datetime, timezone
ROOT=Path(__file__).resolve().parents[1]; OUT=ROOT/"evidence"/"test_logs"; LAYER=ROOT/"attack-navigator/layers/synthetic_layer.json"
records=json.loads((ROOT/"intel-sources/synthetic_intel.json").read_text(encoding="utf-8"))
scores={"A":100,"B":80,"C":60,"D":40}
fused={}
for r in records:
    t=r["technique_id"]; fused.setdefault(t,{"techniqueID":t,"score":0,"comment":""})
    fused[t]["score"]=max(fused[t]["score"],scores.get(r["reliability"],20))
    fused[t]["comment"] += f'{r["source"]}: {r["summary"]} '
layer={"name":"MadHatter Synthetic Threat Intel Layer","versions":{"attack":"enterprise"},"techniques":list(fused.values())}
LAYER.parent.mkdir(parents=True,exist_ok=True); LAYER.write_text(json.dumps(layer,indent=2),encoding="utf-8")
report={"generated_at":datetime.now(timezone.utc).isoformat(),"project":"12-Threat-Intelligence-Fusion-ATTACK-Navigator","summary":{"records_total":len(records),"techniques_total":len(fused)},"layer_path":str(LAYER.relative_to(ROOT))}
OUT.mkdir(parents=True,exist_ok=True); path=OUT/f"intel_fusion_{datetime.now(timezone.utc).strftime('%Y%m%d-%H%M%S')}.json"; path.write_text(json.dumps(report,indent=2),encoding="utf-8")
print(json.dumps(report,indent=2)); print(f"\n[+] Validation report written to: {path}")
raise SystemExit(0 if len(fused)==2 else 1)
