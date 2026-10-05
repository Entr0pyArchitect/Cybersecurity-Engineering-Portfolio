# Project Charter

## Project
06-Autonomous-Response-Module

## Purpose
Build a constrained, lab-safe response module that receives mock security alerts and produces simulated response decisions using approved playbooks.

## Scope
The MVP is simulation-only. It plans response actions but does not execute real containment, process termination, account changes, host isolation, or destructive actions.

## Non-Goals
- No real host isolation
- No account disablement
- No credential collection
- No process killing
- No persistence
- No real EDR/SIEM integration in MVP
- No autonomous real-world response

## Success Criteria
- Go module builds.
- Unit tests pass.
- Mock alert loads.
- Lab-safe playbooks load.
- Response decision is generated.
- Decision keeps `approved_for_auto_run` false.
- Evidence is captured.