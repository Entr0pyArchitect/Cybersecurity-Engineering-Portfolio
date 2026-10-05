# Validation Plan

## Baseline Checks
- `python --version` works.
- `python .\pipelines\validate_ad_detections.py` runs.
- Expected result: Privileged group change and risky service account events are detected; benign event does not match.
- Evidence is written under `evidence/test_logs`.

## Evidence to Capture
- Python version output.
- Validation console output.
- JSON validation report.
- Screenshots of successful validation.
- Notes explaining controlled failures.

## MVP Limitations
The MVP proves structure and validation logic first. It is not a production system.
