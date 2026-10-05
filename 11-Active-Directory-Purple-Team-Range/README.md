# 11 — Active Directory Purple Team Range

<p align="center">
<img alt="Status: Scaffold" src="https://badges.ws/badge/Status-Scaffold-6f42c1?style=for-the-badge"> <img alt="Project: 11" src="https://badges.ws/badge/Project-11-111827?style=for-the-badge"> <img alt="Stack: Python now; PowerShell + Terraform/Ansible later" src="https://badges.ws/badge/Stack-Python%20now%3B%20PowerShell%20%2B%20Terraform%2FAnsible%20later-0b7285?style=for-the-badge"> <img alt="Scope: Lab / Authorized Only" src="https://badges.ws/badge/Scope-Lab%20%2F%20Authorized%20Only-8b5e34?style=for-the-badge">
</p>

> **Portfolio role:** AD defense / identity detection

## Overview

A defensive Active Directory learning range focused on identity fundamentals, event logging, synthetic validation, and later owned-lab adversary emulation.

This project is part of the **Cybersecurity Engineering Portfolio** monorepo and the broader **Project MadHatter** roadmap. This README is written as a standalone project introduction: what exists now, what is being built next, how progress is validated, and where the project deliberately stops.

## Current State

| Field | Current value |
|---|---|
| **Status** | Scaffold |
| **Primary stack** | Python now; PowerShell + Terraform/Ansible later |
| **Portfolio role** | AD defense / identity detection |
| **Validation entry point** | `python3 pipelines/validate_ad_detections.py` |
| **Next milestone** | Add Windows Event ID notes and one event/detection pair with expected behavior and a benign control. |

The present material includes synthetic identity events and Python validation. A deployed AD range is a future milestone.

> **Maturity note:** project names describe the intended engineering direction. A scaffold, fixture validator, or design artifact is not presented as a deployed production capability.

## Engineering Direction

```mermaid
flowchart LR
    A["Synthetic identity event"]
    B["Provider / Event ID context"]
    C["Detection hypothesis"]
    D["Positive + benign control"]
    E["Observed result"]
    A --> B
    B --> C
    C --> D
    D --> E
```

### Current Development Target

Add Windows Event ID notes and one event/detection pair with expected behavior and a benign control.

### Learning Focus

AD DS, Kerberos, security auditing, GPOs, service accounts, Sysmon, Sigma.

## Validation

Primary baseline entry point:

```bash
python3 pipelines/validate_ad_detections.py
```

The command above is a **validation entry point**, not a blanket claim that every environment, integration, or future feature currently passes. Record the revision, operating system, relevant tool versions, inputs, expected behavior, actual behavior, and limitations when capturing results.

## Scope & Safety

Use synthetic identities or personally owned lab domains only. Never treat an AD exercise as authorization to test an organizational domain.

Work for this project is limited to owned systems and VMs, synthetic data and toy targets, designated CTF/HTB environments where the rules permit the activity, or systems covered by explicit written authorization.

No project README should turn an intended feature into a claim of demonstrated behavior without evidence.

## Portfolio Integration

Adds identity telemetry and detection depth to Projects 04, 09, 15, and related DFIR work.

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
