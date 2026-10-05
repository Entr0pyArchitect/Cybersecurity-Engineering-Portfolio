# FortKnox

<p align="center">
  <img alt="Track: Blue Team" src="https://badges.ws/badge/Track-Blue%20Team-2563eb?style=for-the-badge">
  <img alt="Language: Rust" src="https://badges.ws/badge/Language-Rust-b7410e?style=for-the-badge&logo=rust">
  <img alt="Status: Active Development" src="https://badges.ws/badge/Status-Active%20Development-0b7285?style=for-the-badge">
</p>

FortKnox is a Rust secure-communications engineering project used to study **component separation, cryptographic boundaries, protocol design, service integration, and local infrastructure**.

FortKnox is unfinished and under active development. The current public tree represents an engineering baseline, not a production-ready or independently reviewed secure-communications system.

## Workspace

```text
FortKnox/
├── crates/
│   ├── fortknox-core/
│   ├── fortknox-crypto/
│   └── fortknox-protocol/
├── docs/
│   └── THREAT_MODEL.md
├── infra/
│   └── docker-compose.yml
├── server/
│   └── fortknox-server/
├── tools/
│   └── fortknox-dev-client/
├── .env.example
├── Cargo.lock
└── Cargo.toml
```

## Component responsibilities

| Component | Purpose |
|---|---|
| `fortknox-core` | Shared core types and application-level primitives |
| `fortknox-crypto` | Dedicated cryptographic implementation/experimentation boundary |
| `fortknox-protocol` | Protocol/message representation boundary |
| `fortknox-server` | Server-side integration layer |
| `fortknox-dev-client` | Development client used for controlled testing |
| `infra/` | Local infrastructure scaffolding |
| `docs/THREAT_MODEL.md` | Security assumptions, assets, threats, and design considerations |

The separation above is intentional: cryptography, protocol representation, application logic, and service integration should remain independently reviewable.

## Build validation

From this directory:

```bash
cargo check --workspace
```

A successful `cargo check` demonstrates that the current Rust workspace type-checks and resolves its dependencies. It does **not** by itself prove cryptographic correctness, protocol security, production hardening, or deployment readiness.

## Local configuration

`.env.example` contains public example values only.

```bash
cp .env.example .env
```

Replace placeholder values before use. `.env` remains ignored and must not be committed.

## Security posture

FortKnox is treated as a security-engineering project, so public claims are deliberately narrower than the design goals.

Current review areas include:

- key and trust boundaries;
- message/protocol validation;
- authenticated service interactions;
- storage/database boundaries;
- logging and error handling;
- local deployment assumptions;
- abuse cases documented in the threat model.

## Development security notes

The current server uses permissive CORS as a development convenience. This configuration is appropriate only for the local engineering baseline and must be replaced with explicit allowed origins before any non-loopback deployment.

PostgreSQL and Redis are bound to `127.0.0.1` in the included Compose configuration so the default development services are not exposed to the surrounding LAN.

## Public / private boundary

Do not commit live credentials, private keys, hidden-service identity material, real user data, production endpoints, or environment-specific secrets.

## Scope

Development and validation are limited to owned/local infrastructure and explicitly authorized test environments.

[← Back to Blue Team](../README.md)
