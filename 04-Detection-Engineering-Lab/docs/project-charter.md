# Project Charter

## Project
04-Detection-Engineering-Lab

## Purpose
Build a reproducible detection engineering lab that validates detection logic against safe, synthetic lab data before moving into deeper lab simulations.

## Scope
The MVP validates Sigma-style and Suricata-style detection ideas against mock process and network events. It does not execute attack tools or target real systems.

## Non-Goals
- No real-world targeting
- No malware execution
- No credential theft
- No persistence
- No stealth behavior
- No public internet exposure
- No production deployment

## Success Criteria
- Synthetic process events exist.
- Synthetic network events exist.
- Detection test definitions exist.
- Validation pipeline runs.
- Evidence report is generated.
- Documentation explains what was tested and why.