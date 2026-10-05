# 02 — Threat-Informed Defense Platform

<p align="center">
<img alt="Status: Active Build Lane" src="https://badges.ws/badge/Status-Active%20Build%20Lane-0a7f2e?style=for-the-badge"> <img alt="Project: 02" src="https://badges.ws/badge/Project-02-111827?style=for-the-badge"> <img alt="Stack: Go; Python planned" src="https://badges.ws/badge/Stack-Go%3B%20Python%20planned-0b7285?style=for-the-badge"> <img alt="Scope: Lab / Authorized Only" src="https://badges.ws/badge/Scope-Lab%20%2F%20Authorized%20Only-8b5e34?style=for-the-badge">
</p>

> **Portfolio role:** Purple-team orchestration / ATT&CK mapping

## Overview

A threat-informed engineering platform that turns mock TTP input or public-source threat notes into structured ATT&CK mappings and detection backlog work items.

This project is part of the **Cybersecurity Engineering Portfolio** monorepo and the broader **Project MadHatter** roadmap. This README is written as a standalone project introduction: what exists now, what is being built next, how progress is validated, and where the project deliberately stops.

## Current State

| Field | Current value |
|---|---|
| **Status** | Active Build Lane |
| **Primary stack** | Go; Python planned for supporting tooling |
| **Portfolio role** | Purple-team orchestration / ATT&CK mapping |
| **Validation entry point** | `go test ./...` |
| **Next milestone** | Turn mock TTP input into a clean detection/backlog item format with explicit identifiers, required telemetry, detection goals, source provenance, and planned status. |

The current scaffold is implemented in Go. Python is reserved for later supporting tooling. Project 02 is the designated first active implementation lane in Project MadHatter.

> **Maturity note:** project names describe the intended engineering direction. A scaffold, fixture validator, or design artifact is not presented as a deployed production capability.

## Engineering Direction

```mermaid
flowchart LR
    A["Threat note / mock TTP"]
    B["Input contract"]
    C["ATT&CK mapping"]
    D["Detection work item"]
    E["Backlog / downstream projects"]
    A --> B
    B --> C
    C --> D
    D --> E
```

### Current Development Target

Turn mock TTP input into a clean detection/backlog item format with explicit identifiers, required telemetry, detection goals, source provenance, and planned status.

### Learning Focus

Go HTTP APIs, JSON contracts, MITRE ATT&CK, Sigma concepts, STIX/TAXII orientation, provenance and mapping discipline.

## Validation

Primary baseline entry point:

```bash
go test ./...
```

The command above is a **validation entry point**, not a blanket claim that every environment, integration, or future feature currently passes. Record the revision, operating system, relevant tool versions, inputs, expected behavior, actual behavior, and limitations when capturing results.

## Scope & Safety

Use mock, synthetic, or properly cited public-source data. A backlog item represents proposed detection work; it is not proof that a detection fired or that response automation occurred.

Work for this project is limited to owned systems and VMs, synthetic data and toy targets, designated CTF/HTB environments where the rules permit the activity, or systems covered by explicit written authorization.

No project README should turn an intended feature into a claim of demonstrated behavior without evidence.

## Portfolio Integration

Feeds structured work into detection engineering, SIEM/query work, telemetry requirements, and later bounded response workflows.

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
