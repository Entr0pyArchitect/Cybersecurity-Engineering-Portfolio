# Validation Plan

## Baseline Checks
- `python --version` works.
- `python .\pipelines\fuse_intel.py` runs.
- Expected result: Synthetic intel records fuse into two ATT&CK techniques and generate a layer file.
- Evidence is written under `evidence/test_logs`.

## Evidence to Capture
- Python version output.
- Validation console output.
- JSON validation report.
- Screenshots of successful validation.
- Notes explaining controlled failures.

## MVP Limitations
The MVP proves structure and validation logic first. It is not a production system.
