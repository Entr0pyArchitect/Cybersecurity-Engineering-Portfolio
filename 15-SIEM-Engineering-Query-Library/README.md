# 15 — SIEM Engineering Query Library

<p align="center">
<img alt="Status: Scaffold" src="https://badges.ws/badge/Status-Scaffold-6f42c1?style=for-the-badge"> <img alt="Project: 15" src="https://badges.ws/badge/Project-15-111827?style=for-the-badge"> <img alt="Stack: Python + KQL + SPL + Sigma" src="https://badges.ws/badge/Stack-Python%20%2B%20KQL%20%2B%20SPL%20%2B%20Sigma-0b7285?style=for-the-badge"> <img alt="Scope: Lab / Authorized Only" src="https://badges.ws/badge/Scope-Lab%20%2F%20Authorized%20Only-8b5e34?style=for-the-badge">
</p>

> **Portfolio role:** Detection metadata / query engineering

## Overview

A structured query library for mapping detection goals to telemetry, ATT&CK techniques, query dialects, validation state, and tuning notes.

This project is part of the **Cybersecurity Engineering Portfolio** monorepo and the broader **Project MadHatter** roadmap. This README is written as a standalone project introduction: what exists now, what is being built next, how progress is validated, and where the project deliberately stops.

## Current State

| Field | Current value |
|---|---|
| **Status** | Scaffold |
| **Primary stack** | Python + KQL + SPL + Sigma |
| **Portfolio role** | Detection metadata / query engineering |
| **Validation entry point** | `python3 pipelines/validate_query_library.py` |
| **Next milestone** | Add required-telemetry and false-positive-tuning fields to the query metadata model. |

Python validates query metadata around KQL, SPL, and Sigma examples. Metadata validity is distinct from execution in a target engine.

> **Maturity note:** project names describe the intended engineering direction. A scaffold, fixture validator, or design artifact is not presented as a deployed production capability.

## Engineering Direction

```mermaid
flowchart LR
    A["Detection goal"]
    B["Telemetry requirement"]
    C["Query metadata"]
    D["Dialect-specific query"]
    E["Validation / tuning record"]
    A --> B
    B --> C
    C --> D
    D --> E
```

### Current Development Target

Add required-telemetry and false-positive-tuning fields to the query metadata model.

### Learning Focus

Microsoft Sentinel/KQL, Splunk SPL, Sigma, Elastic detection examples, tuning and schema compatibility.

## Validation

Primary baseline entry point:

```bash
python3 pipelines/validate_query_library.py
```

The command above is a **validation entry point**, not a blanket claim that every environment, integration, or future feature currently passes. Record the revision, operating system, relevant tool versions, inputs, expected behavior, actual behavior, and limitations when capturing results.

## Scope & Safety

Use sample or synthetic data unless a target environment is explicitly authorized. Label queries unexecuted when they have not run in the claimed engine.

Work for this project is limited to owned systems and VMs, synthetic data and toy targets, designated CTF/HTB environments where the rules permit the activity, or systems covered by explicit written authorization.

No project README should turn an intended feature into a claim of demonstrated behavior without evidence.

## Portfolio Integration

Turns detection observations into reusable query artifacts and links threat context, telemetry, and validation work.

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
