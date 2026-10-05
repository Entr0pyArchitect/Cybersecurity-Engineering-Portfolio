# 05 — Zero-Trust Micro-Infrastructure

<p align="center">
<img alt="Status: Scaffold" src="https://badges.ws/badge/Status-Scaffold-6f42c1?style=for-the-badge"> <img alt="Project: 05" src="https://badges.ws/badge/Project-05-111827?style=for-the-badge"> <img alt="Stack: Python + Terraform/Kubernetes later" src="https://badges.ws/badge/Stack-Python%20%2B%20Terraform%2FKubernetes%20later-0b7285?style=for-the-badge"> <img alt="Scope: Lab / Authorized Only" src="https://badges.ws/badge/Scope-Lab%20%2F%20Authorized%20Only-8b5e34?style=for-the-badge">
</p>

> **Portfolio role:** Infrastructure security / zero trust

## Overview

A local-first infrastructure security project for modeling identity-aware service controls, least privilege, policy expectations, and eventually observable enforcement.

This project is part of the **Cybersecurity Engineering Portfolio** monorepo and the broader **Project MadHatter** roadmap. This README is written as a standalone project introduction: what exists now, what is being built next, how progress is validated, and where the project deliberately stops.

## Current State

| Field | Current value |
|---|---|
| **Status** | Scaffold |
| **Primary stack** | Python + Terraform/Kubernetes later |
| **Portfolio role** | Infrastructure security / zero trust |
| **Validation entry point** | `python3 pipelines/validate_zero_trust.py` |
| **Next milestone** | Expand the service inventory with control categories and produce a Markdown validation report tied to explicit policy expectations. |

The current Python scaffold checks service-control declarations. Terraform and Kubernetes are future integrations, not proof of deployed enforcement.

> **Maturity note:** project names describe the intended engineering direction. A scaffold, fixture validator, or design artifact is not presented as a deployed production capability.

## Engineering Direction

```mermaid
flowchart LR
    A["Service inventory"]
    B["Identity / peer policy"]
    C["Declared control"]
    D["Validation report"]
    E["Future local enforcement"]
    A --> B
    B --> C
    C --> D
    D --> E
```

### Current Development Target

Expand the service inventory with control categories and produce a Markdown validation report tied to explicit policy expectations.

### Learning Focus

NIST SP 800-207, CISA Zero Trust Maturity Model, Kubernetes NetworkPolicy, OPA, SPIFFE/SPIRE, Terraform.

## Validation

Primary baseline entry point:

```bash
python3 pipelines/validate_zero_trust.py
```

The command above is a **validation entry point**, not a blanket claim that every environment, integration, or future feature currently passes. Record the revision, operating system, relevant tool versions, inputs, expected behavior, actual behavior, and limitations when capturing results.

## Scope & Safety

Start locally with kind/minikube when deployment begins. Document cost, exposure, cleanup, and least privilege before using cloud resources.

Work for this project is limited to owned systems and VMs, synthetic data and toy targets, designated CTF/HTB environments where the rules permit the activity, or systems covered by explicit written authorization.

No project README should turn an intended feature into a claim of demonstrated behavior without evidence.

## Portfolio Integration

Provides hardened infrastructure and identity boundaries for later cloud, response, telemetry, and analyst workflows.

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
