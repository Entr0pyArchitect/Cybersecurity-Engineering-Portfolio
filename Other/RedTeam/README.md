# Red Team — Applied Learning

<p align="center">
  <img alt="Track: Red Team" src="https://badges.ws/badge/Track-Red%20Team-b91c1c?style=for-the-badge">
  <img alt="Status: Planned" src="https://badges.ws/badge/Status-Planned-6f42c1?style=for-the-badge">
  <img alt="Scope: Authorized Only" src="https://badges.ws/badge/Scope-Authorized%20Only-15803d?style=for-the-badge">
</p>

This track is reserved for **later-stage applied offensive-security learning** after the core portfolio and supporting lab environments are mature.

The purpose is not to collect arbitrary offensive code. The purpose is to practice how security-testing capabilities are **designed, constrained, validated, observed by defenders, cleaned up, and documented**.

## Public areas

| Area | Purpose | Current public state |
|---|---|---|
| [Offensive Projects](Offensive%20Projects/) | Scoped exercises and research projects conducted in controlled environments | Planning/documentation only |
| [Offensive Tooling](Offensive%20Tooling/) | Future custom testing-framework and lab-tool engineering | Planning/documentation only |

## Authorized scope

Future work is limited to:

- isolated lab networks and virtual machines;
- CTFs and intentionally vulnerable training targets;
- personally owned systems;
- systems covered by explicit authorization.

Anything outside that scope is out of bounds.

## Required project gate

Before an offensive project becomes active, it should document:

1. **Objective** — what is being learned or tested;
2. **Authorization** — why the target/environment is permitted;
3. **Environment** — isolated lab, CTF, owned system, or authorized range;
4. **Safety controls** — containment, egress restrictions, snapshots, test accounts, and rate limits where relevant;
5. **Telemetry plan** — what defensive logs/signals will be collected;
6. **Cleanup** — how services, test accounts, artifacts, and infrastructure are removed;
7. **Evidence** — what proves the learning objective;
8. **Publication review** — what is safe and useful to expose publicly.

## Malware-research boundary

If malware development or analysis is used as an applied-learning exercise, it remains confined to purpose-built isolated lab environments. Live deployment, uncontrolled propagation, persistence on third-party systems, credential theft, and real-world targeting are outside this portfolio's scope.

Runnable malware samples and operational payload material are not part of the public repository.

## Why this track exists

The strongest offensive-security learning should also improve defensive engineering. Future exercises are expected to produce useful observations for detection, telemetry, hardening, incident response, and threat modeling.

[← Back to Applied Learning](../README.md)
