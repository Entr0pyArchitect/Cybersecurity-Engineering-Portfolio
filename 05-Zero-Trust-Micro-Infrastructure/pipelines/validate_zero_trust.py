import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, List


ROOT = Path(__file__).resolve().parents[1]
SERVICES_PATH = ROOT / "testdata" / "service_inventory.json"
CONTROLS_PATH = ROOT / "policy" / "controls" / "zero_trust_controls.json"
EVIDENCE_DIR = ROOT / "evidence" / "test_logs"


def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def evaluate_control(service: Dict[str, Any], control: Dict[str, Any]) -> Dict[str, Any]:
    field = control["field"]
    expected = control["expected"]
    actual = service.get(field)

    if expected == "non_empty":
        passed = isinstance(actual, str) and actual.strip() != ""
    else:
        passed = actual == expected

    return {
        "control_id": control["id"],
        "control_name": control["name"],
        "field": field,
        "expected": expected,
        "actual": actual,
        "passed": passed,
        "why": control["why"],
    }


def evaluate_service(service: Dict[str, Any], controls: List[Dict[str, Any]]) -> Dict[str, Any]:
    results = [evaluate_control(service, control) for control in controls]
    failed = [item for item in results if not item["passed"]]

    return {
        "service_name": service.get("service_name", "unknown-service"),
        "project": service.get("project", "unknown-project"),
        "environment": service.get("environment", "unknown-environment"),
        "public_exposure": service.get("public_exposure"),
        "passed": len(failed) == 0,
        "failed_controls": [item["control_id"] for item in failed],
        "control_results": results,
    }


def main() -> int:
    services = load_json(SERVICES_PATH)
    controls_document = load_json(CONTROLS_PATH)
    controls = controls_document["required_controls"]

    service_results = [evaluate_service(service, controls) for service in services]

    report = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "project": "05-Zero-Trust-Micro-Infrastructure",
        "scope": "local-lab policy validation only",
        "summary": {
            "services_total": len(service_results),
            "services_passed": sum(1 for item in service_results if item["passed"]),
            "services_failed": sum(1 for item in service_results if not item["passed"]),
            "controls_total": len(controls),
        },
        "results": service_results,
        "note": "The intentionally-incomplete-demo-service is expected to fail. It proves the validator catches missing zero-trust controls."
    }

    EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
    timestamp = datetime.now(timezone.utc).strftime("%Y%m%d-%H%M%S")
    output_path = EVIDENCE_DIR / f"zero_trust_validation_{timestamp}.json"

    with output_path.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)

    print(json.dumps(report, indent=2))
    print(f"\n[+] Validation report written to: {output_path}")

    # Expected MVP behavior: at least one pass and at least one controlled failure.
    # This proves both positive and negative validation paths work.
    expected_demo_failure = any(
        item["service_name"] == "intentionally-incomplete-demo-service" and not item["passed"]
        for item in service_results
    )

    real_services = [
        item for item in service_results
        if item["service_name"] != "intentionally-incomplete-demo-service"
    ]

    real_services_passed = all(item["passed"] for item in real_services)

    return 0 if expected_demo_failure and real_services_passed else 1


if __name__ == "__main__":
    raise SystemExit(main())