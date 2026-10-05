# Project Charter

## Project
05-Zero-Trust-Micro-Infrastructure

## Purpose
Build a local-first zero-trust infrastructure proof that validates service identity, mTLS expectations, authentication requirements, network policy, least privilege, secrets handling, observability, and no-public-exposure defaults.

## Scope
The MVP validates service definitions and zero-trust controls using local JSON data and a Python validation pipeline. It does not deploy cloud infrastructure or expose services publicly.

## Non-Goals
- No public cloud deployment in MVP
- No public internet exposure
- No production Kubernetes deployment
- No real secrets
- No bypassing authentication systems
- No scanning third-party infrastructure

## Success Criteria
- Service inventory exists.
- Zero-trust controls exist.
- Validation pipeline runs.
- Real services pass baseline controls.
- Intentionally incomplete demo service fails as expected.
- Evidence report is generated.