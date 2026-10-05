# Validation Plan

## Baseline Checks
- `python --version` works.
- `python .\pipelines\validate_cloud_identity.py` runs.
- Expected result: Two safe identities pass, two risky paths are detected, and the report exits successfully.
- Evidence is written under `evidence/test_logs`.

## Evidence to Capture
- Python version output.
- Validation console output.
- JSON validation report.
- Screenshots of successful validation.
- Notes explaining controlled failures.

## MVP Limitations
The MVP proves structure and validation logic first. It is not a production system.
