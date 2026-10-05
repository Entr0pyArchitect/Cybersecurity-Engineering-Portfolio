# Validation Plan

## Baseline Checks
- `python --version` works.
- `python .\pipelines\validate_detections.py` runs.
- The validation report shows all tests passed.
- Evidence is written under `evidence/test_logs`.

## Evidence to Capture
- Python version output.
- Validation console output.
- JSON validation report.
- Screenshots of successful validation.
- Notes explaining what each detection is proving.

## MVP Limitations
This MVP validates logic against synthetic events. It does not yet connect to live telemetry, SIEMs, packet captures, Atomic Red Team, Terraform labs, or Project 03 event streams.