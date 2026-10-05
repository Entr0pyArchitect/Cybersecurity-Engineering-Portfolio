# Validation Plan

## Baseline Checks
- `python --version` works.
- `python .\pipelines\validate_deception_events.py` runs.
- Expected result: Synthetic decoy-touch events are detected and benign background events are ignored.
- Evidence is written under `evidence/test_logs`.

## Evidence to Capture
- Python version output.
- Validation console output.
- JSON validation report.
- Screenshots of successful validation.
- Notes explaining controlled failures.

## MVP Limitations
The MVP proves structure and validation logic first. It is not a production system.
