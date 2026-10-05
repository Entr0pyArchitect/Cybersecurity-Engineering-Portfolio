# 17 — AI-Enhanced SOC Copilot & RAG Knowledge Base

<p align="center">
<img alt="Status: Scaffold" src="https://badges.ws/badge/Status-Scaffold-6f42c1?style=for-the-badge"> <img alt="Project: 17" src="https://badges.ws/badge/Project-17-111827?style=for-the-badge"> <img alt="Stack: Python; RAG/vector stack later" src="https://badges.ws/badge/Stack-Python%3B%20RAG%2Fvector%20stack%20later-0b7285?style=for-the-badge"> <img alt="Scope: Lab / Authorized Only" src="https://badges.ws/badge/Scope-Lab%20%2F%20Authorized%20Only-8b5e34?style=for-the-badge">
</p>

> **Portfolio role:** Local SOC knowledge retrieval foundation

## Overview

A source-grounded analyst-assistance project built around retrieval over verified notes, detections, sources, and permitted write-ups.

This project is part of the **Cybersecurity Engineering Portfolio** monorepo and the broader **Project MadHatter** roadmap. This README is written as a standalone project introduction: what exists now, what is being built next, how progress is validated, and where the project deliberately stops.

## Current State

| Field | Current value |
|---|---|
| **Status** | Scaffold |
| **Primary stack** | Python; RAG/vector stack later |
| **Portfolio role** | Local SOC knowledge retrieval foundation |
| **Validation entry point** | `python3 pipelines/validate_rag_index.py` |
| **Next milestone** | Add source cards, citation checks, and a small retrieval-evaluation set before model generation or a vector database is introduced. |

Python currently validates knowledge-index material. A working RAG system or demonstrated retrieval quality has not yet been established.

> **Maturity note:** project names describe the intended engineering direction. A scaffold, fixture validator, or design artifact is not presented as a deployed production capability.

## Engineering Direction

```mermaid
flowchart LR
    A["Verified knowledge"]
    B["Index material"]
    C["Retrieval evaluation"]
    D["Source-supported answer"]
    E["Human review"]
    A --> B
    B --> C
    C --> D
    D --> E
```

### Current Development Target

Add source cards, citation checks, and a small retrieval-evaluation set before model generation or a vector database is introduced.

### Learning Focus

Embeddings, RAG, LlamaIndex, LangChain, FAISS, Chroma, privacy, citation quality, evaluation.

## Validation

Primary baseline entry point:

```bash
python3 pipelines/validate_rag_index.py
```

The command above is a **validation entry point**, not a blanket claim that every environment, integration, or future feature currently passes. Record the revision, operating system, relevant tool versions, inputs, expected behavior, actual behavior, and limitations when capturing results.

## Scope & Safety

Every generated answer must be grounded in inspected sources and human-reviewed. Keep private evidence out of public prompts and examples.

Work for this project is limited to owned systems and VMs, synthetic data and toy targets, designated CTF/HTB environments where the rules permit the activity, or systems covered by explicit written authorization.

No project README should turn an intended feature into a claim of demonstrated behavior without evidence.

## Portfolio Integration

Provides bounded analyst support over evidence produced by the rest of the portfolio rather than replacing analyst judgment.

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
