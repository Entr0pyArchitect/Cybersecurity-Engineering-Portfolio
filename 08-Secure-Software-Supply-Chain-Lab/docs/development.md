# Development Guide

## Local Development

Check Python:

    python --version

Run validation:

    python .\pipelines\validate_supply_chain.py

Run via PowerShell helper:

    Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
    .\scripts\validate.ps1

Capture baseline evidence:

    .\scripts\capture_baseline.ps1

## Development Rules
- Use synthetic data first.
- Do not store real secrets.
- Do not scan private repositories without permission.
- Do not claim real vulnerability findings from synthetic tests.
- Document controlled failures clearly.