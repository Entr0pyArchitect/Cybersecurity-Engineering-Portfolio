import json, re
from pathlib import Path
from datetime import datetime, timezone
ROOT=Path(__file__).resolve().parents[1]; OUT=ROOT/"evidence"/"test_logs"; INDEX=ROOT/"rag/index.json"
docs=[]
for path in sorted((ROOT/"knowledge_base").glob("*.md")):
    text=path.read_text(encoding="utf-8")
    tokens=sorted(set(re.findall(r"[a-z0-9]+", text.lower())))
    docs.append({"path":str(path.relative_to(ROOT)),"tokens":tokens,"title":text.splitlines()[0].lstrip("# ").strip()})
INDEX.parent.mkdir(parents=True,exist_ok=True); INDEX.write_text(json.dumps({"documents":docs},indent=2),encoding="utf-8")
query={"detection","sigma","evidence"}
scores=[]
for d in docs:
    score=len(query.intersection(set(d["tokens"])))
    scores.append({"path":d["path"],"title":d["title"],"score":score})
scores.sort(key=lambda x:x["score"], reverse=True)
report={"generated_at":datetime.now(timezone.utc).isoformat(),"project":"17-AI-Enhanced-SOC-Copilot-RAG-Knowledge-Base","summary":{"documents_total":len(docs),"top_result":scores[0]["path"],"retrieval_passed":"project04" in scores[0]["path"]},"results":scores}
OUT.mkdir(parents=True,exist_ok=True); path=OUT/f"rag_index_validation_{datetime.now(timezone.utc).strftime('%Y%m%d-%H%M%S')}.json"; path.write_text(json.dumps(report,indent=2),encoding="utf-8")
print(json.dumps(report,indent=2)); print(f"\n[+] Validation report written to: {path}")
raise SystemExit(0 if report["summary"]["retrieval_passed"] else 1)
