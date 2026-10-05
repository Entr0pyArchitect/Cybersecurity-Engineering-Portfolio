from __future__ import annotations

import json
import sys
from pathlib import Path

from agent_core.guardrails import evaluate_request, load_request


def main() -> int:
    if len(sys.argv) < 2:
        print("Usage: python -m agent_core.cli <request.json>")
        return 1

    request_path = Path(sys.argv[1])
    request = load_request(request_path)
    decision = evaluate_request(request)

    print(json.dumps(decision.to_dict(), indent=2))

    return 0 if decision.status in {"approved_lab_safe", "needs_review", "blocked"} else 1


if __name__ == "__main__":
    raise SystemExit(main())