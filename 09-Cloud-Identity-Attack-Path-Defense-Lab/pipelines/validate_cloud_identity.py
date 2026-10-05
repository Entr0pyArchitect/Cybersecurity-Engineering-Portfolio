import json
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "testdata" / "identity_graph.json"
OUT = ROOT / "evidence" / "test_logs"

def main():
    graph = json.loads(DATA.read_text(encoding="utf-8"))
    findings = []
    for ident in graph["identities"]:
        if ident.get("external") and ident.get("admin") and not ident.get("mfa"):
            findings.append({"id":"external_admin_without_mfa","identity":ident["id"],"severity":"high"})
        if ident.get("type") == "service_account" and ident.get("admin"):
            findings.append({"id":"overprivileged_service_account","identity":ident["id"],"severity":"high"})
    expected = sorted(graph["expected_findings"])
    actual = sorted({f["id"] for f in findings})
    report = {"generated_at":datetime.now(timezone.utc).isoformat(),"project":"09-Cloud-Identity-Attack-Path-Defense-Lab","scope":"synthetic identity graph only","summary":{"findings_total":len(findings),"expected_controls_passed":actual==expected},"findings":findings}
    OUT.mkdir(parents=True, exist_ok=True)
    path = OUT / f"cloud_identity_validation_{datetime.now(timezone.utc).strftime('%Y%m%d-%H%M%S')}.json"
    path.write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(json.dumps(report, indent=2)); print(f"\n[+] Validation report written to: {path}")
    return 0 if actual == expected else 1
if __name__ == "__main__": raise SystemExit(main())
