import json
import re
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, List

ROOT = Path(__file__).resolve().parents[1]
CONTROLS_PATH = ROOT / "policy" / "supply_chain_controls.json"
SECRET_PATTERNS_PATH = ROOT / "policy" / "secret_patterns.json"
DEPENDENCY_INVENTORY_PATH = ROOT / "testdata" / "manifests" / "dependency_inventory.json"
CONFIG_DIR = ROOT / "testdata" / "configs"
EVIDENCE_DIR = ROOT / "evidence" / "test_logs"

def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)

def evaluate_dependency(component: Dict[str, Any], controls: List[Dict[str, Any]]) -> Dict[str, Any]:
    control_results = []
    for control in controls:
        field = control["field"]
        expected = control["expected"]
        actual = component.get(field)
        passed = actual == expected
        control_results.append({
            "control_id": control["id"],
            "control_name": control["name"],
            "field": field,
            "expected": expected,
            "actual": actual,
            "passed": passed,
            "why": control["why"],
        })

    failed = [item for item in control_results if not item["passed"]]
    return {
        "component": component.get("component", "unknown-component"),
        "ecosystem": component.get("ecosystem", "unknown"),
        "version": component.get("version", ""),
        "passed": len(failed) == 0,
        "failed_controls": [item["control_id"] for item in failed],
        "control_results": control_results,
    }

def scan_file_for_secrets(path: Path, pattern_document: Dict[str, Any]) -> List[Dict[str, Any]]:
    text = path.read_text(encoding="utf-8")
    findings = []
    allowed_placeholders = [item.lower() for item in pattern_document.get("allowed_placeholders", [])]

    for pattern in pattern_document["patterns"]:
        compiled = re.compile(pattern["regex"])
        for line_number, line in enumerate(text.splitlines(), start=1):
            match = compiled.search(line)
            if not match:
                continue
            lowered_line = line.lower()
            if any(placeholder.lower() in lowered_line for placeholder in allowed_placeholders):
                continue
            findings.append({
                "file": str(path.relative_to(ROOT)),
                "line": line_number,
                "pattern_id": pattern["id"],
                "pattern_name": pattern["name"],
                "severity": pattern["severity"],
                "matched_preview": line.strip(),
                "notes": pattern["notes"],
            })
    return findings

def run_secret_scan() -> Dict[str, Any]:
    pattern_document = load_json(SECRET_PATTERNS_PATH)
    findings = []
    files = [path for path in sorted(CONFIG_DIR.glob("*")) if path.is_file()]
    for path in files:
        findings.extend(scan_file_for_secrets(path, pattern_document))
    return {
        "files_scanned": [str(path.relative_to(ROOT)) for path in files],
        "findings_total": len(findings),
        "findings": findings,
        "expected_demo_finding": any("bad_config.example" in item["file"] for item in findings),
    }

def main() -> int:
    controls_document = load_json(CONTROLS_PATH)
    controls = controls_document["required_controls"]
    dependencies = load_json(DEPENDENCY_INVENTORY_PATH)

    dependency_results = [evaluate_dependency(component, controls) for component in dependencies]
    secret_scan = run_secret_scan()

    expected_dependency_failure = any(
        item["component"] == "intentionally-incomplete-demo-dependency" and not item["passed"]
        for item in dependency_results
    )
    real_dependencies = [
        item for item in dependency_results
        if item["component"] != "intentionally-incomplete-demo-dependency"
    ]
    real_dependencies_passed = all(item["passed"] for item in real_dependencies)

    report = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "project": "08-Secure-Software-Supply-Chain-Lab",
        "scope": "synthetic supply-chain validation only",
        "summary": {
            "dependencies_total": len(dependency_results),
            "dependencies_passed": sum(1 for item in dependency_results if item["passed"]),
            "dependencies_failed": sum(1 for item in dependency_results if not item["passed"]),
            "controls_total": len(controls),
            "secret_findings_total": secret_scan["findings_total"],
        },
        "dependency_results": dependency_results,
        "secret_scan": secret_scan,
        "note": "The incomplete dependency and bad config are expected controlled failures. They prove the validator catches weak supply-chain posture and obvious secret-like assignments."
    }

    EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
    timestamp = datetime.now(timezone.utc).strftime("%Y%m%d-%H%M%S")
    output_path = EVIDENCE_DIR / f"supply_chain_validation_{timestamp}.json"
    with output_path.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)

    print(json.dumps(report, indent=2))
    print(f"\n[+] Validation report written to: {output_path}")

    return 0 if real_dependencies_passed and expected_dependency_failure and secret_scan["expected_demo_finding"] else 1

if __name__ == "__main__":
    raise SystemExit(main())