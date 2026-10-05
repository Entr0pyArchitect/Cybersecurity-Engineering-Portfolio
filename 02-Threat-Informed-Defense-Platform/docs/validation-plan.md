# Validation Plan

## Baseline Checks
- `go test ./...` completes successfully.
- `go run ./cmd/platform` starts the local API.
- `/health` returns service status.
- `/api/v1/techniques` returns starter technique data.
- `/api/v1/ingest` accepts mock JSON and returns backlog items.

## Evidence to Capture
- Screenshot of successful `go test ./...`
- Screenshot or terminal output of local server starting
- Output from `/health`
- Output from `/api/v1/ingest`
- Notes explaining what the output proves

## Current MVP Limitation
This is an early proof-of-concept. It does not yet perform real threat-intelligence parsing, persistent storage, user authentication, or SIEM integration.
