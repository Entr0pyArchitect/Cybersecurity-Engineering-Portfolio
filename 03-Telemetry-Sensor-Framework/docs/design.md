# Design Notes

## MVP Architecture

windows-agent / linux-agent
        |
        v
sensor-core collectors
        |
        v
normalizer
        |
        v
SensorEvent JSON
        |
        v
Project 04 detection validation / Project 02 defense platform later

## Current Design

The framework starts with safe userland collection. The first collectors observe only the running agent process and basic local system context.

## Why Start Small

Telemetry systems can become risky if they jump directly into privileged collection. This MVP starts with safe observable data, schema discipline, and validation before moving toward deeper OS-specific telemetry.

## Current Components

- `sensor-core`: shared Rust library for events, collectors, normalization, and validation.
- `windows-agent`: Windows-oriented binary using the shared sensor core.
- `linux-agent`: Linux-oriented binary using the shared sensor core.
- `schemas`: JSON schema for normalized events.
- `proto`: Protobuf placeholder for future streaming.
- `testdata`: static sample event data.
- `evidence`: screenshots, test logs, and terminal output.