# Validation Plan

## Baseline Checks
- `python --version` works.
- `python .\pipelines\validate_evidence_manifest.py` runs.
- Expected result: Evidence manifest is generated and hashes validate for synthetic files.
- Evidence is written under `evidence/test_logs`.

## Evidence to Capture
- Python version output.
- Validation console output.
- JSON validation report.
- Screenshots of successful validation.
- Notes explaining controlled failures.

## MVP Limitations
The MVP proves structure and validation logic first. It is not a production system.
