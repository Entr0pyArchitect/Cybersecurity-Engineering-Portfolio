# 06 — Autonomous Response Module

<p align="center">
<img alt="Status: Scaffold" src="https://badges.ws/badge/Status-Scaffold-6f42c1?style=for-the-badge"> <img alt="Project: 06" src="https://badges.ws/badge/Project-06-111827?style=for-the-badge"> <img alt="Stack: Go + JSON playbooks" src="https://badges.ws/badge/Stack-Go%20%2B%20JSON%20playbooks-0b7285?style=for-the-badge"> <img alt="Scope: Lab / Authorized Only" src="https://badges.ws/badge/Scope-Lab%20%2F%20Authorized%20Only-8b5e34?style=for-the-badge">
</p>

> **Portfolio role:** Bounded response automation

## Overview

A simulation-first response service for turning mock alerts into constrained, auditable response decisions with explicit approval state.

This project is part of the **Cybersecurity Engineering Portfolio** monorepo and the broader **Project MadHatter** roadmap. This README is written as a standalone project introduction: what exists now, what is being built next, how progress is validated, and where the project deliberately stops.

## Current State

| Field | Current value |
|---|---|
| **Status** | Scaffold |
| **Primary stack** | Go + JSON playbooks |
| **Portfolio role** | Bounded response automation |
| **Validation entry point** | `go test ./...` |
| **Next milestone** | Add decision IDs, explicit timestamps, and report export while keeping action selection, approval, and simulation results separate. |

The Go baseline plans simulation-only decisions from mock alerts and JSON playbooks. It does not currently prove real containment.

> **Maturity note:** project names describe the intended engineering direction. A scaffold, fixture validator, or design artifact is not presented as a deployed production capability.

## Engineering Direction

```mermaid
flowchart LR
    A["Mock alert"]
    B["Playbook lookup"]
    C["Safety / approval gate"]
    D["Simulation decision"]
    E["Auditable report"]
    A --> B
    B --> C
    C --> D
    D --> E
```

### Current Development Target

Add decision IDs, explicit timestamps, and report export while keeping action selection, approval, and simulation results separate.

### Learning Focus

Incident response lifecycle, playbook design, human approval, allowlists, reversible actions, Go service behavior.

## Validation

Primary baseline entry point:

```bash
go test ./...
```

The command above is a **validation entry point**, not a blanket claim that every environment, integration, or future feature currently passes. Record the revision, operating system, relevant tool versions, inputs, expected behavior, actual behavior, and limitations when capturing results.

## Scope & Safety

Any future containment or mock-secret rotation must be reversible, allowlisted, lab-bounded, and supported by recovery evidence.

Work for this project is limited to owned systems and VMs, synthetic data and toy targets, designated CTF/HTB environments where the rules permit the activity, or systems covered by explicit written authorization.

No project README should turn an intended feature into a claim of demonstrated behavior without evidence.

## Portfolio Integration

Consumes validated detections and produces response decisions that can be preserved by DFIR/evidence workflows.

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
