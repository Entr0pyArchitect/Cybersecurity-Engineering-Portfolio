# Development Guide

## Local Development

Start the API:

    go run ./cmd/platform

Run tests:

    go test ./...

Run API checks from a second PowerShell window while the server is running:

    Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
    .\scripts\test_api.ps1

## Evidence Workflow

Capture proof of work under:

    evidence/
    ├── screenshots/
    ├── terminal_output/
    └── test_logs/

Suggested evidence:

- go test ./... output
- /health response
- /api/v1/techniques response
- /api/v1/ingest response
- screenshots of terminal output
- notes explaining what each result proves

## Development Rules

- Keep the project lab-safe.
- Use mock or public training data only.
- Do not ingest private data.
- Do not connect this system to real targets.
- Do not automate offensive action.
- Every feature should improve defensive reasoning, detection mapping, validation, or documentation.