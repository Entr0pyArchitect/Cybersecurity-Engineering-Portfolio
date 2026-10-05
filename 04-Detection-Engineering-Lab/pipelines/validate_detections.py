import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, List


ROOT = Path(__file__).resolve().parents[1]
TESTS_PATH = ROOT / "detections" / "detection_tests.json"
EVIDENCE_DIR = ROOT / "evidence" / "test_logs"


def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def normalize(value: Any) -> str:
    if value is None:
        return ""
    return str(value).lower()


def run_test(test: Dict[str, Any]) -> Dict[str, Any]:
    dataset_path = ROOT / test["dataset"]
    events: List[Dict[str, Any]] = load_json(dataset_path)

    field = test["field"]
    needles = [normalize(item) for item in test["match_any"]]

    matching_ids = []
    non_matching_ids = []

    for event in events:
        value = normalize(event.get(field))
        event_id = event.get("event_id", "unknown-event")

        if any(needle in value for needle in needles):
            matching_ids.append(event_id)
        else:
            non_matching_ids.append(event_id)

    expected_matching = sorted(test.get("expected_matching_event_ids", []))
    expected_non_matching = sorted(test.get("expected_non_matching_event_ids", []))

    actual_matching = sorted(matching_ids)
    actual_non_matching = sorted(non_matching_ids)

    passed = actual_matching == expected_matching and actual_non_matching == expected_non_matching

    return {
        "id": test["id"],
        "description": test["description"],
        "dataset": test["dataset"],
        "field": field,
        "match_any": test["match_any"],
        "expected_matching_event_ids": expected_matching,
        "actual_matching_event_ids": actual_matching,
        "expected_non_matching_event_ids": expected_non_matching,
        "actual_non_matching_event_ids": actual_non_matching,
        "passed": passed,
    }


def main() -> int:
    tests = load_json(TESTS_PATH)
    results = [run_test(test) for test in tests]

    report = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "project": "04-Detection-Engineering-Lab",
        "scope": "synthetic lab data only",
        "summary": {
            "total": len(results),
            "passed": sum(1 for item in results if item["passed"]),
            "failed": sum(1 for item in results if not item["passed"]),
        },
        "results": results,
    }

    EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
    timestamp = datetime.now(timezone.utc).strftime("%Y%m%d-%H%M%S")
    output_path = EVIDENCE_DIR / f"detection_validation_{timestamp}.json"

    with output_path.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)

    print(json.dumps(report, indent=2))
    print(f"\n[+] Validation report written to: {output_path}")

    return 0 if report["summary"]["failed"] == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())