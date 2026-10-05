# Design Notes

## Architecture
The platform starts as a small Go API. It receives mock/public training inputs, normalizes technique IDs, maps them to tactic and telemetry requirements, and returns backlog items.

## Current Components
- `cmd/platform`: application entry point
- `internal/server`: HTTP API routes and response handling
- `internal/model`: request/response data structures
- `internal/mapper`: simple ATT&CK-style mapping logic
- `testdata`: mock inputs
- `rules/sigma`: candidate detection examples

## Data Flow
1. User submits mock TTP JSON.
2. API validates input.
3. Mapper normalizes technique IDs.
4. Platform generates detection backlog items.
5. Platform returns candidate Sigma/detection ideas.

## Safety Boundary
All data is mock, public, or lab-generated. The system should never be pointed at real organizations or private data.
