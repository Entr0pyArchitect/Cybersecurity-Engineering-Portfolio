# Validation Plan

## Baseline Checks

- `cargo fmt` completes.
- `cargo test` passes.
- `cargo run -p windows-agent` prints valid JSON.
- `cargo run -p linux-agent` prints valid JSON.
- JSON output follows the expected schema shape.
- Evidence is captured under `evidence/`.

## Evidence to Capture

- Rust version.
- Cargo version.
- `cargo test` output.
- Windows agent JSON output.
- Linux agent JSON output if tested.
- Screenshot of successful runs.
- Notes explaining what the event fields mean.

## MVP Limitations

This MVP does not perform deep endpoint monitoring. It proves workspace setup, schema design, normalization, validation, and safe event generation.