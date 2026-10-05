# Validation Plan

## Baseline Checks
- `python --version` works.
- `python .\pipelines\validate_query_library.py` runs.
- Expected result: All sample query metadata entries include technique, data source, query path, and false-positive notes.
- Evidence is written under `evidence/test_logs`.

## Evidence to Capture
- Python version output.
- Validation console output.
- JSON validation report.
- Screenshots of successful validation.
- Notes explaining controlled failures.

## MVP Limitations
The MVP proves structure and validation logic first. It is not a production system.
