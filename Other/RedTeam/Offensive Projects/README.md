# Offensive Projects

<p align="center">
  <img alt="Status: Planned" src="https://badges.ws/badge/Status-Planned-6f42c1?style=for-the-badge">
  <img alt="Environment: Isolated / Authorized" src="https://badges.ws/badge/Environment-Isolated%20%2F%20Authorized-15803d?style=for-the-badge">
</p>

This area is the future home for **scoped offensive-security exercises** that apply concepts from the core portfolio in controlled environments.

## Intended project types

Examples of appropriate work include:

- CTF or intentionally vulnerable target exercises;
- adversary-emulation scenarios built specifically for a private lab;
- exploit-development learning against purpose-built toy or owned targets;
- post-exercise detection and telemetry review;
- isolated malware-behavior research designed to improve defensive understanding;
- purple-team exercises where offensive activity is paired with detections and remediation.

These are categories of future learning, not claims that those environments are currently implemented.

## Project documentation standard

Each future project should include:

```text
README.md
docs/
├── scope.md
├── threat-model.md
├── safety-controls.md
├── cleanup-plan.md
└── validation-plan.md
```

The public README should answer:

- What was the learning objective?
- Why was the environment authorized?
- What was deliberately out of scope?
- What defensive telemetry was collected?
- What changed after the exercise?
- What evidence supports the conclusions?
- What material was intentionally withheld from public Git?

## Publication policy

The public portfolio should favor architecture, lessons learned, detections, remediation, sanitized evidence, and reproducible toy examples over reusable operational attack material.

No active offensive implementation is published in this directory at this stage.

[← Back to Red Team](../README.md)
