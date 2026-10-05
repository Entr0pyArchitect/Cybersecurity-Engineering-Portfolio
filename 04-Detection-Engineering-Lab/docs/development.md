# Development Guide

## Local Development

Check Python:

    python --version

Run validation:

    python .\pipelines\validate_detections.py

Run via PowerShell helper:

    Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
    .\scripts\validate.ps1

Capture baseline evidence:

    .\scripts\capture_baseline.ps1

## Development Rules
- Use synthetic or owned-lab data only.
- Do not include real private logs.
- Do not execute attack tools in the MVP.
- Keep every detection tied to telemetry and false-positive notes.
- Validate both matching and non-matching events.