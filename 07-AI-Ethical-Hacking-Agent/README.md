# 07 — AI Ethical Hacking Agent

<p align="center">
<img alt="Status: Scaffold" src="https://badges.ws/badge/Status-Scaffold-6f42c1?style=for-the-badge"> <img alt="Project: 07" src="https://badges.ws/badge/Project-07-111827?style=for-the-badge"> <img alt="Stack: Python; TypeScript/UI later" src="https://badges.ws/badge/Stack-Python%3B%20TypeScript%2FUI%20later-0b7285?style=for-the-badge"> <img alt="Scope: Lab / Authorized Only" src="https://badges.ws/badge/Scope-Lab%20%2F%20Authorized%20Only-8b5e34?style=for-the-badge">
</p>

> **Portfolio role:** Bounded human-in-the-loop security assistant

## Overview

A guardrail-driven assistant foundation for scope checks, finding classification, report summaries, and study organization.

This project is part of the **Cybersecurity Engineering Portfolio** monorepo and the broader **Project MadHatter** roadmap. This README is written as a standalone project introduction: what exists now, what is being built next, how progress is validated, and where the project deliberately stops.

## Current State

| Field | Current value |
|---|---|
| **Status** | Scaffold |
| **Primary stack** | Python; TypeScript/UI later |
| **Portfolio role** | Bounded human-in-the-loop security assistant |
| **Validation entry point** | `python3 -m unittest discover -s tests` |
| **Next milestone** | Expand safe/unsafe/needs-review scenarios and record false-positive and false-negative behavior before connecting a model. |

The Python foundation models safe, unsafe, and needs-review requests. Model-backed behavior and a UI are later milestones.

> **Maturity note:** project names describe the intended engineering direction. A scaffold, fixture validator, or design artifact is not presented as a deployed production capability.

## Engineering Direction

```mermaid
flowchart LR
    A["User request"]
    B["Scope / policy evaluation"]
    C["Safe | Review | Unsafe"]
    D["Human decision"]
    E["Permitted assistance"]
    A --> B
    B --> C
    C --> D
    D --> E
```

### Current Development Target

Expand safe/unsafe/needs-review scenarios and record false-positive and false-negative behavior before connecting a model.

### Learning Focus

NIST AI RMF, OWASP guidance for LLM applications, MITRE ATLAS, policy boundaries, evaluation, human review.

## Validation

Primary baseline entry point:

```bash
python3 -m unittest discover -s tests
```

The command above is a **validation entry point**, not a blanket claim that every environment, integration, or future feature currently passes. Record the revision, operating system, relevant tool versions, inputs, expected behavior, actual behavior, and limitations when capturing results.

## Scope & Safety

No autonomous attacks, flag solving, unsupervised exploitation, or model behavior that bypasses human approval.

Work for this project is limited to owned systems and VMs, synthetic data and toy targets, designated CTF/HTB environments where the rules permit the activity, or systems covered by explicit written authorization.

No project README should turn an intended feature into a claim of demonstrated behavior without evidence.

## Portfolio Integration

Explores bounded AI assistance while preserving the independent-authorship and evidence rules of Project MadHatter.

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
