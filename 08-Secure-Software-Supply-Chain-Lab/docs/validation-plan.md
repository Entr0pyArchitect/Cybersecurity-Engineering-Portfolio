# Validation Plan

## Baseline Checks
- `python --version` works.
- `python .\pipelines\validate_supply_chain.py` runs.
- Good dependencies pass.
- Intentionally incomplete demo dependency fails.
- Synthetic bad config creates one secret-like finding.
- Evidence is written under `evidence/test_logs`.

## MVP Limitations
This MVP is not a replacement for real tools like Gitleaks, Syft, Grype, OSV-Scanner, Trivy, or GitHub Advanced Security. It is a local learning scaffold.