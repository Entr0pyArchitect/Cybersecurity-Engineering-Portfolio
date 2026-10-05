# Development Guide

## Local Development

Run tests:

    python -m unittest discover -s tests

Run a safe request:

    python -m agent_core.cli .\test-scenarios\safe_lab_request.json

Run an unsafe request:

    python -m agent_core.cli .\test-scenarios\unsafe_request.json

Capture baseline evidence:

    Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
    .\scripts\capture_baseline.ps1

## Development Rules
- Do not use this project to solve flags for the user.
- Do not add autonomous exploitation.
- Do not add real target scanning.
- Add LLM integration only after guardrails, logging, and review workflow are mature.