# Validation Plan

## Baseline Checks
- `python --version` works.
- Unit tests pass.
- Safe request returns `approved_lab_safe`.
- Unsafe request returns `blocked`.
- Unclear request returns `needs_review`.
- Evidence is captured.

## Evidence to Capture
- Python version.
- Unit test output.
- Safe decision JSON.
- Unsafe decision JSON.
- Needs-review decision JSON.
- Notes explaining what the guardrail proves.

## MVP Limitations
The MVP is a rule-based safety layer. It is not a full AI agent yet and does not call an LLM.