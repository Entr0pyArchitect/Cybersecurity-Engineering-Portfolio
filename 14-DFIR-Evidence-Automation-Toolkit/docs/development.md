# Development Guide

## Local Development

Check Python:

    python --version

Run validation:

    python .\pipelines\validate_evidence_manifest.py

Run via helper:

    Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
    .\scripts\validate.ps1

Capture baseline evidence:

    .\scripts\capture_baseline.ps1

## Development Rules
- Keep the MVP local and synthetic.
- Save evidence before changing logic.
- Document controlled failures clearly.
- Use official sources first.
