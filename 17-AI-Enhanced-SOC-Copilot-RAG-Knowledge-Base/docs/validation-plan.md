# Validation Plan

## Baseline Checks
- `python --version` works.
- `python .\pipelines\validate_rag_index.py` runs.
- Expected result: Knowledge base index builds and retrieves the expected Project 04 detection note.
- Evidence is written under `evidence/test_logs`.

## Evidence to Capture
- Python version output.
- Validation console output.
- JSON validation report.
- Screenshots of successful validation.
- Notes explaining controlled failures.

## MVP Limitations
The MVP proves structure and validation logic first. It is not a production system.
