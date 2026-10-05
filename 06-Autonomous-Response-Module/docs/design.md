# Design Notes

## MVP Architecture

mock alert -> playbook loader -> response engine -> simulated response decision -> evidence

## Current Design
The module reads a synthetic lab alert and a set of lab-safe playbooks. It selects a matching playbook based on technique ID and severity, then returns simulated actions.

## Safety Model
All actions are forced into `simulation_only` mode. The engine does not perform real system changes. Human approval remains required for any real-world response.

## Connected Projects
- Project 02 can generate threat-informed response candidates.
- Project 03 can provide telemetry context.
- Project 04 can generate validated alerts.
- Project 05 defines safe infrastructure boundaries.
- Project 14 can later provide DFIR evidence collection patterns.