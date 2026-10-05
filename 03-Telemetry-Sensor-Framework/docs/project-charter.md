# Project Charter

## Project
03-Telemetry-Sensor-Framework

## Purpose
Build a lightweight telemetry framework that collects safe local userland events, normalizes them into a consistent schema, and prepares them for later detection engineering work.

## Scope
This project begins with safe userland telemetry only. The MVP does not use kernel hooks, ETW subscriptions, eBPF probes, drivers, privileged collection, or real production endpoint monitoring.

## Non-Goals
- No stealth collection
- No persistence
- No credential collection
- No kernel driver work at MVP stage
- No production deployment
- No monitoring of systems without permission

## Success Criteria
- Rust workspace builds
- sensor-core tests pass
- Windows/Linux agent binaries compile
- Agent prints normalized JSON events
- Schema and proto files exist
- Evidence is captured