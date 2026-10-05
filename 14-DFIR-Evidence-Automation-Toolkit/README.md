# 14 — DFIR Evidence Automation Toolkit

<p align="center">
<img alt="Status: Scaffold" src="https://badges.ws/badge/Status-Scaffold-6f42c1?style=for-the-badge"> <img alt="Project: 14" src="https://badges.ws/badge/Project-14-111827?style=for-the-badge"> <img alt="Stack: Python now; PowerShell/Bash later" src="https://badges.ws/badge/Stack-Python%20now%3B%20PowerShell%2FBash%20later-0b7285?style=for-the-badge"> <img alt="Scope: Lab / Authorized Only" src="https://badges.ws/badge/Scope-Lab%20%2F%20Authorized%20Only-8b5e34?style=for-the-badge">
</p>

> **Portfolio role:** Evidence hashing and DFIR workflow

## Overview

A DFIR engineering toolkit focused on repeatable evidence manifests, integrity checks, acquisition metadata, and chain-of-custody-style documentation.

This project is part of the **Cybersecurity Engineering Portfolio** monorepo and the broader **Project MadHatter** roadmap. This README is written as a standalone project introduction: what exists now, what is being built next, how progress is validated, and where the project deliberately stops.

## Current State

| Field | Current value |
|---|---|
| **Status** | Scaffold |
| **Primary stack** | Python now; PowerShell/Bash later |
| **Portfolio role** | Evidence hashing and DFIR workflow |
| **Validation entry point** | `python3 pipelines/validate_evidence_manifest.py` |
| **Next milestone** | Add a chain-of-custody-style record generator and timestamp-normalization notes. |

Python supports the current evidence-manifest validation. Collection extensions are future work.

> **Maturity note:** project names describe the intended engineering direction. A scaffold, fixture validator, or design artifact is not presented as a deployed production capability.

## Engineering Direction

```mermaid
flowchart LR
    A["Owned-lab artifact"]
    B["Acquisition metadata"]
    C["Hash / manifest"]
    D["Transfer / transformation record"]
    E["Reviewable evidence package"]
    A --> B
    B --> C
    C --> D
    D --> E
```

### Current Development Target

Add a chain-of-custody-style record generator and timestamp-normalization notes.

### Learning Focus

NIST SP 800-86, incident-response guidance, Velociraptor, Timesketch, timestamps, provenance, evidence integrity.

## Validation

Primary baseline entry point:

```bash
python3 pipelines/validate_evidence_manifest.py
```

The command above is a **validation entry point**, not a blanket claim that every environment, integration, or future feature currently passes. Record the revision, operating system, relevant tool versions, inputs, expected behavior, actual behavior, and limitations when capturing results.

## Scope & Safety

Prefer read-only collection from owned lab fixtures and protect personal data. Integrity tracking alone does not establish legal admissibility.

Work for this project is limited to owned systems and VMs, synthetic data and toy targets, designated CTF/HTB environments where the rules permit the activity, or systems covered by explicit written authorization.

No project README should turn an intended feature into a claim of demonstrated behavior without evidence.

## Portfolio Integration

Preserves evidence from detection, response, analysis, identity, and deception exercises in a repeatable format.

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
