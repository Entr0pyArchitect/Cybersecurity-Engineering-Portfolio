# Project Charter

## Project
08-Secure-Software-Supply-Chain-Lab

## Purpose
Build a local synthetic software supply-chain security lab that validates dependency posture, secret scanning behavior, SBOM awareness, and release-readiness checks.

## Scope
The MVP uses synthetic dependency inventory and synthetic config files. It does not scan private repositories, publish packages, access real secrets, or deploy software.

## Non-Goals
- No real secret storage
- No private repo scanning without permission
- No package publishing
- No dependency installation from unknown sources
- No production release automation in MVP
- No destructive remediation

## Success Criteria
- Good dependencies pass baseline controls.
- Intentionally incomplete dependency fails as expected.
- Synthetic hardcoded secret-like config is detected.
- Placeholder config values are ignored.
- Evidence report is generated.