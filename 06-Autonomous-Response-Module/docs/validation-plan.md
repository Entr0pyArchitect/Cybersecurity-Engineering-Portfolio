# Validation Plan

## Baseline Checks
- `go version` works.
- `go mod tidy` completes.
- `go test ./...` passes.
- `go run ./cmd/responder` prints a response decision.
- The decision is simulation-only.
- `approved_for_auto_run` remains false.

## Evidence to Capture
- Go version output.
- Test output.
- Response decision JSON.
- Screenshot of successful run.
- Notes explaining why the response is safe.

## MVP Limitations
The MVP plans response only. It does not execute real actions, integrate with live systems, or modify hosts/accounts.