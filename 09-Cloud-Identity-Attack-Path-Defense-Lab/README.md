# 09 — Cloud Identity Attack Path Defense Lab

<p align="center">
<img alt="Status: Scaffold" src="https://badges.ws/badge/Status-Scaffold-6f42c1?style=for-the-badge"> <img alt="Project: 09" src="https://badges.ws/badge/Project-09-111827?style=for-the-badge"> <img alt="Stack: Python; Terraform/KQL later" src="https://badges.ws/badge/Stack-Python%3B%20Terraform%2FKQL%20later-0b7285?style=for-the-badge"> <img alt="Scope: Lab / Authorized Only" src="https://badges.ws/badge/Scope-Lab%20%2F%20Authorized%20Only-8b5e34?style=for-the-badge">
</p>

> **Portfolio role:** Cloud identity risk modeling

## Overview

A synthetic identity-graph lab for modeling privilege relationships, attack-path risk, and defensive least-privilege controls before any live-cloud work.

This project is part of the **Cybersecurity Engineering Portfolio** monorepo and the broader **Project MadHatter** roadmap. This README is written as a standalone project introduction: what exists now, what is being built next, how progress is validated, and where the project deliberately stops.

## Current State

| Field | Current value |
|---|---|
| **Status** | Scaffold |
| **Primary stack** | Python; Terraform/KQL later |
| **Portfolio role** | Cloud identity risk modeling |
| **Validation entry point** | `python3 pipelines/validate_cloud_identity.py` |
| **Next milestone** | Add a simple graph diagram and one new least-privilege rule, then validate both an allowed path and an excessive-permission path. |

The Python scaffold models synthetic identity graphs. Terraform and KQL are future directions.

> **Maturity note:** project names describe the intended engineering direction. A scaffold, fixture validator, or design artifact is not presented as a deployed production capability.

## Engineering Direction

```mermaid
flowchart LR
    A["Synthetic identities"]
    B["Relationship graph"]
    C["Path analysis"]
    D["Least-privilege rule"]
    E["Defensive finding"]
    A --> B
    B --> C
    C --> D
    D --> E
```

### Current Development Target

Add a simple graph diagram and one new least-privilege rule, then validate both an allowed path and an excessive-permission path.

### Learning Focus

Entra ID, Azure RBAC, AWS IAM, OAuth/SAML, conditional access, cloud logs, identity detection.

## Validation

Primary baseline entry point:

```bash
python3 pipelines/validate_cloud_identity.py
```

The command above is a **validation entry point**, not a blanket claim that every environment, integration, or future feature currently passes. Record the revision, operating system, relevant tool versions, inputs, expected behavior, actual behavior, and limitations when capturing results.

## Scope & Safety

Begin with local mock data. Any Azure/AWS deployment requires separate authorization, budget, and exposure checks.

Work for this project is limited to owned systems and VMs, synthetic data and toy targets, designated CTF/HTB environments where the rules permit the activity, or systems covered by explicit written authorization.

No project README should turn an intended feature into a claim of demonstrated behavior without evidence.

## Portfolio Integration

Connects infrastructure controls to identity telemetry, SIEM logic, and threat-informed defensive planning.

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
