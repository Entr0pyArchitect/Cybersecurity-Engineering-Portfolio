# Development Guide

## Local Development

Run tests:

    go test ./...

Run the responder:

    go run ./cmd/responder

Run with a specific mock alert:

    go run ./cmd/responder .\test-scenarios\mock_low_unknown_alert.json

Capture baseline evidence:

    Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
    .\scripts\capture_baseline.ps1

## Development Rules
- Keep MVP simulation-only.
- Never perform real response actions without human approval.
- Do not isolate hosts, disable accounts, kill processes, or collect secrets in the MVP.
- Every action must be explainable, reversible, and documented.