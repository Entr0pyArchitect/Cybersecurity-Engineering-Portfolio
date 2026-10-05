# Project Charter

## Project
07-AI-Ethical-Hacking-Agent

## Purpose
Build a bounded AI-assisted security helper that supports ethical cybersecurity learning, documentation, scope checks, and project planning without performing autonomous hacking or replacing the user's technical reps.

## Scope
The MVP is a rule-based guardrail and request evaluation system. It does not call external LLM APIs, solve CTF flags, exploit systems, scan targets, or generate unsafe operational instructions.

## Non-Goals
- No autonomous hacking
- No exploit execution
- No real target testing
- No credential theft
- No malware, persistence, stealth, or evasion
- No CTF flag solving
- No replacement of user learning reps

## Success Criteria
- Safe lab request is approved.
- Unsafe request is blocked.
- Unclear-scope request requires review.
- Tests pass.
- Evidence is captured.