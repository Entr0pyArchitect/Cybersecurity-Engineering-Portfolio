# Project OMEGA Security Audit

## Audit scope

This document records the final internal security review and validation results for Project OMEGA / AegisPhrase v0.3.8.

The audit covered:

- Windows 11 build, CLI, UI, and smoke-test workflows.
- Linux/Kali build, CLI, UI, and smoke-test workflows.
- Passphrase vault encryption/decryption.
- Arbitrary file encryption/decryption.
- Wrong-password fail-closed behavior.
- Metadata tamper rejection.
- Ciphertext tamper rejection.
- Controlled dictionary crack-resistance validation.
- Repository cleanup requirements for GitHub deployment.

## Environment validation summary

| Test area | Windows 11 | Linux/Kali |
|---|---:|---:|
| `cargo fmt --check` | PASS | PASS |
| `cargo check` | PASS | PASS |
| `cargo build --release` | PASS | PASS |
| Smoke script | PASS | PASS |
| CLI passphrase vault encrypt/decrypt | PASS | PASS |
| CLI arbitrary file encrypt/decrypt | PASS | PASS |
| UI passphrase vault workflow | PASS | PASS |
| UI arbitrary file workflow | PASS | PASS |
| SHA-256 file round-trip | PASS | PASS |
| Executable hash/tamper check | PASS | PASS |
| TPM status reporting | PASS | PASS / no TPM device detected |

## Final validation hashes

Known-good release binary hashes recorded during final validation:

```text
Windows 11:
39644150dd6fd05553839a3b82ab7aa267db2286b38765797322033eb7550f51

Linux/Kali:
e007ec16c62d66c02c030b4ea9eacd55a86e40c89bf7c9dd7ab39307a30e4670
```

These hashes are environment-specific. Rebuilding with a different OS, compiler, dependency graph, feature set, or build environment can produce a different binary hash.

## Cryptographic design review

Project OMEGA uses password-derived authenticated encryption for `.apv` files.

Core properties:

- Key derivation: Argon2id.
- Salt: fresh random 128-bit salt per envelope.
- Nonce: fresh random nonce per envelope.
- Default cipher: AES-256-GCM.
- Alternate ciphers: XChaCha20-Poly1305 and AES-256-GCM-SIV.
- Envelope: versioned JSON metadata plus ciphertext.
- Plaintext source filenames are not stored inside encrypted envelopes.
- File encryption passwords must be at least 12 characters.

Audit conclusion: the design is appropriate for a local defensive tool when users choose strong encryption passwords and run it on trusted systems.

## Entropy review

Passphrase generation uses OS-backed cryptographic randomness through the Rust dependency stack. Optional hidden user entropy can be mixed into generation state, but user entropy is not treated as a replacement for OS randomness.

Generated passphrases are protected against local repeats through SHA-256 fingerprint history. This is a local guarantee only. It is not a global guarantee across machines, future reinstalls, or deleted history.

Audit conclusion: entropy handling is appropriate for the intended use case, with clear documentation around the limits of local no-repeat tracking.

## Memory and output hygiene review

Implemented controls:

- Rust memory safety.
- Owned secret buffers zeroized where practical.
- Hidden terminal output support.
- Clipboard TTL clearing.
- Best-effort terminal history cleanup.
- Overwrite confirmations.
- Input/output path safety checks.

Limitations:

- Clipboard managers can still capture copied secrets.
- Terminal transcripts, screenshots, malware, EDR/AV telemetry, pagefiles, and crash dumps are outside the tool’s control.
- Best-effort cleanup cannot retroactively remove secrets from external logging systems.

## UI/CLI behavior review

The Windows and Linux UI/CLI flows were separated by platform folder:

- Windows: `Win11/scripts/`
- Linux/Kali: `Linux/scripts/`

Behavior validated:

- Platform-specific path examples.
- Current-directory and full-path support.
- `back`, `cancel`, or `q` support in prompt flows where practical.
- Visible status markers: `[*]`, `[+]`, `[!]`, `[-]`.
- Honest TPM notes and no false claim of TPM-backed encryption.
- Secure exit behavior.

## Wrong-password validation

Wrong-password tests were run against:

- A passphrase vault.
- An arbitrary file `.apv` envelope.

Results:

- Wrong password rejected.
- Error returned as wrong password or corrupt file.
- No plaintext output file created.

Audit result: PASS.

## Tamper validation

Tamper tests covered:

1. Metadata corruption by flipping a byte in the JSON envelope.
2. Ciphertext corruption by modifying the decoded `ciphertext_b64` payload and re-encoding the envelope.

Results:

- Corrupted metadata was rejected.
- Modified ciphertext failed AEAD authentication.
- No plaintext output file was created after failure.

Audit result: PASS.

## Controlled crack-resistance validation

A controlled dictionary harness was used to verify behavior under offline dictionary conditions.

Results:

- A deliberately weak but policy-allowed test password was recovered.
- Strong validation files were not recovered by the tested wordlist.
- The minimum encryption password policy rejected too-short weak passwords.

Audit conclusion: Project OMEGA is **crack-resistant under tested offline dictionary conditions**. It must not be described as “uncrackable.”

## Dependency/security review

Recommended final checks before every release:

```bash
cargo fmt --check
cargo check
cargo build --release
cargo audit
cargo outdated
```

`cargo outdated` findings are not automatically vulnerabilities. Review them for security relevance, API compatibility, and release stability.

## GitHub cleanup checklist

Before public push, remove:

```text
target/
*.apv
*.recovered.txt
*sample*.txt
*final*.txt
*tampered*
omega_crack_attempt.sh
omega-crack-test-wordlist.txt
crack-attempt-output.tmp
logs/
tmp/
```

Confirm:

```bash
git status
```

No generated secrets, encrypted test files, recovered plaintext, local build artifacts, shell histories, or local-only logs should be committed.

## Final audit verdict

Project OMEGA v0.3.8 is ready for GitHub source deployment after cleanup, provided the repository contains only source code, platform launchers, clean documentation, license, manifest, and lockfile.

Deployment status: **PASS — source deployment ready after local artifact cleanup.**
