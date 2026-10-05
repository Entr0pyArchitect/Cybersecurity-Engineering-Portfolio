# Development Guide

## Local Development

Check Python:

    python --version

Run validation:

    python .\pipelines\validate_zero_trust.py

Run via PowerShell helper:

    Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
    .\scripts\validate.ps1

Capture baseline evidence:

    .\scripts\capture_baseline.ps1

## Development Rules
- Keep MVP local-only.
- Do not use real secrets.
- Do not deploy cloud resources yet.
- Do not expose lab services publicly.
- Validate control logic before infrastructure deployment.