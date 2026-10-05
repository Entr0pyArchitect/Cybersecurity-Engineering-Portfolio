# 16 — Deception & Honeypot Research Grid

<p align="center">
<img alt="Status: Scaffold" src="https://badges.ws/badge/Status-Scaffold-6f42c1?style=for-the-badge"> <img alt="Project: 16" src="https://badges.ws/badge/Project-16-111827?style=for-the-badge"> <img alt="Stack: Python now; Docker/Cowrie/Zeek/Suricata later" src="https://badges.ws/badge/Stack-Python%20now%3B%20Docker%2FCowrie%2FZeek%2FSuricata%20later-0b7285?style=for-the-badge"> <img alt="Scope: Lab / Authorized Only" src="https://badges.ws/badge/Scope-Lab%20%2F%20Authorized%20Only-8b5e34?style=for-the-badge">
</p>

> **Portfolio role:** Synthetic deception telemetry

## Overview

A defensive deception research project that starts with synthetic decoy events and controlled telemetry before any service deployment.

This project is part of the **Cybersecurity Engineering Portfolio** monorepo and the broader **Project MadHatter** roadmap. This README is written as a standalone project introduction: what exists now, what is being built next, how progress is validated, and where the project deliberately stops.

## Current State

| Field | Current value |
|---|---|
| **Status** | Scaffold |
| **Primary stack** | Python now; Docker/Cowrie/Zeek/Suricata later |
| **Portfolio role** | Synthetic deception telemetry |
| **Validation entry point** | `python3 pipelines/validate_deception_events.py` |
| **Next milestone** | Add a decoy asset card and local network-boundary notes defining expected interaction, collected fields, retention, and shutdown procedure. |

Python currently models synthetic deception events. A deployed honeypot grid has not been established.

> **Maturity note:** project names describe the intended engineering direction. A scaffold, fixture validator, or design artifact is not presented as a deployed production capability.

## Engineering Direction

```mermaid
flowchart LR
    A["Decoy asset model"]
    B["Controlled interaction"]
    C["Deception event"]
    D["Telemetry / detection link"]
    E["Retention + shutdown record"]
    A --> B
    B --> C
    C --> D
    D --> E
```

### Current Development Target

Add a decoy asset card and local network-boundary notes defining expected interaction, collected fields, retention, and shutdown procedure.

### Learning Focus

Deception design, Cowrie concepts, Zeek, Suricata, network boundaries, telemetry retention, shutdown discipline.

## Validation

Primary baseline entry point:

```bash
python3 pipelines/validate_deception_events.py
```

The command above is a **validation entry point**, not a blanket claim that every environment, integration, or future feature currently passes. Record the revision, operating system, relevant tool versions, inputs, expected behavior, actual behavior, and limitations when capturing results.

## Scope & Safety

Keep initial work local-only and isolated from production/public exposure. A container alone is not proof of sufficient isolation.

Work for this project is limited to owned systems and VMs, synthetic data and toy targets, designated CTF/HTB environments where the rules permit the activity, or systems covered by explicit written authorization.

No project README should turn an intended feature into a claim of demonstrated behavior without evidence.

## Portfolio Integration

Adds controlled deception telemetry to detection, SIEM, incident-response, and research workflows.

See the repository-level [`README.md`](../README.md) for the full project map and [`ProjectMadHatter`](../ProjectMadHatter/) for the execution roadmap.

## Documentation & Evidence Standard

This project should maintain, as applicable:

- a recruiter-readable README;
- a charter with purpose, scope, non-goals, allowed environment, success criteria, and stop conditions;
- design and source-map documentation;
- reproducible validation notes;
- sanitized evidence that directly supports public claims;
- explicit limitations and lessons learned.

Raw evidence, local backups, build output, secrets, and machine-specific artifacts should remain excluded according to the repository and project `.gitignore` rules.

## Status Language

- **Planned** — scope/design exists; behavior is not demonstrated.
- **Scaffold** — files and a minimal prototype exist; broader capability is unfinished.
- **Validated scaffold** — specific checks passed on a recorded revision/environment.
- **Portfolio ready** — demonstrable artifact, reproducible checks, clear docs, safe evidence, honest limitations.
- **Completed milestone** — defined acceptance criteria were met; maintenance or later enhancements may remain.

---

<p align="center"><sub>Part of the Cybersecurity Engineering Portfolio • Project MadHatter</sub></p>
