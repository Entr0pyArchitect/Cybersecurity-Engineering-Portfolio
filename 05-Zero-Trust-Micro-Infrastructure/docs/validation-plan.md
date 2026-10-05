# Validation Plan

## Baseline Checks
- `python --version` works.
- `python .\pipelines\validate_zero_trust.py` runs.
- Project 02 and Project 03 sample services pass.
- The intentionally incomplete demo service fails.
- Evidence is written under `evidence/test_logs`.

## Evidence to Capture
- Python version output.
- Validation console output.
- JSON validation report.
- Screenshot of successful validation.
- Notes explaining which controls passed and failed.

## MVP Limitations
This MVP validates control declarations. It does not yet enforce Kubernetes policy, provision infrastructure, generate certificates, deploy SPIFFE/SPIRE, or run OPA in a cluster.