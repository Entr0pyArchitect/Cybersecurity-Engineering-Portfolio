# Project OMEGA Threat Model

## Purpose

This document defines the security assumptions, protected assets, threat scenarios, controls, non-goals, and remaining risks for Project OMEGA / AegisPhrase.

Project OMEGA is a local-first Rust utility for high-entropy passphrase generation and authenticated `.apv` encryption envelopes. It is built for defensive personal use and controlled validation on Windows 11 and Linux/Kali.

## Protected assets

Project OMEGA is designed to protect:

- Generated passphrases and secrets.
- File-encryption passwords entered by the user.
- Encrypted `.apv` vault files and file envelopes.
- Decrypted plaintext files created by intentional user action.
- Local SHA-256 no-repeat fingerprint history.
- Executable integrity signals from `doctor` hash checks.
- User workflow safety around terminal output, clipboard handling, and accidental overwrites.

## Primary security goals

1. Generate non-predictable passphrases using OS-backed cryptographic randomness.
2. Enforce generated passphrase length from 30 through 64 characters.
3. Block local repeats while the local fingerprint history remains intact.
4. Avoid plaintext passphrase storage by supporting encrypted `.apv` vault files.
5. Encrypt arbitrary files with authenticated encryption.
6. Fail closed on wrong passwords, metadata corruption, and ciphertext tampering.
7. Avoid accidental plaintext output when decrypting with the wrong password.
8. Provide platform-specific UI/CLI workflows for Windows 11 and Linux/Kali.
9. Provide tamper-check support by printing and verifying SHA-256 executable hashes.
10. Use hidden output, clipboard TTL clearing, and best-effort terminal cleanup to reduce casual exposure.

## Non-goals

Project OMEGA does not claim to:

- Defeat malware, keyloggers, clipboard loggers, screen capture, or a compromised OS.
- Provide global never-repeat guarantees across all machines forever.
- Replace a full password manager with sync, browser integration, sharing, MFA, and recovery workflows.
- Provide true TPM-backed file encryption in the current build.
- Provide formal code signing, reproducible builds, or independent third-party cryptographic assurance.
- Prevent recovery when a weak user-chosen encryption password is used.
- Erase secrets already captured in terminal transcripts, screenshots, crash dumps, telemetry, or chats.

## Trust boundaries

### Trusted

- The local operating system cryptographic randomness provider.
- The local Rust binary built from reviewed source.
- The user’s ability to enter and protect strong encryption passwords.
- The filesystem permissions available to the current user account.

### Untrusted or partially trusted

- Terminal scrollback, shell history, and transcript systems.
- Clipboard and clipboard managers.
- Cloud sync software.
- Screenshots, screen recordings, remote support sessions, and malware.
- Any machine that may already be compromised.
- Encrypted `.apv` files supplied by unknown sources.

## Threat scenarios and controls

| Threat scenario | Control | Residual risk |
|---|---|---|
| Weak generated passphrase | OS CSPRNG, explicit length bounds, approved charset choices | User may copy/store it insecurely |
| Duplicate generated passphrase on same machine | SHA-256 local fingerprint history | History can be deleted, moved, or unavailable |
| Weak file encryption password | Minimum 12-character policy | Length alone does not guarantee strength |
| Offline dictionary attack | Argon2id KDF, fresh salt, AEAD encryption | Weak user password can still be cracked |
| Wrong password during decrypt | AEAD authentication failure | Error message intentionally does not distinguish wrong password from corruption |
| Modified ciphertext | AEAD authentication rejects modified data | DoS remains possible; attacker can corrupt files |
| Modified JSON envelope metadata | Envelope parsing/validation rejects invalid file | DoS remains possible |
| Accidental overwrite | UI confirmation and same-input/output checks | User can still intentionally overwrite |
| Secret printed in terminal | Hidden output mode | User may intentionally disable hidden output |
| Clipboard exposure | Clipboard TTL clearing | Clipboard managers or malware can still log contents |
| Binary tampering | `doctor` SHA-256 hash display/verification | Stronger protection requires signed releases and trusted distribution |
| Windows TPM misuse | TPM status is reported honestly; TPM-backed encryption not claimed complete | TPM-backed key wrapping is future work |

## Randomness model

Passphrase generation uses operating-system randomness and optionally mixes hidden user-provided entropy into the generator state. User entropy is additive; it does not replace OS randomness. Character selection should use unbiased sampling behavior and should not rely on predictable seeds.

## Encryption model

Encrypted `.apv` envelopes use:

- Argon2id key derivation.
- Fresh random 128-bit salt per envelope.
- Fresh random nonce per envelope.
- AEAD encryption through AES-256-GCM, XChaCha20-Poly1305, or AES-256-GCM-SIV.
- Versioned metadata that records cipher and KDF parameters.
- No plaintext source filename inside the encrypted envelope.

A salt is not secret. Its purpose is to make precomputation and cross-file reuse attacks impractical. Project OMEGA generates a new random salt for every encrypted envelope.

## Tamper behavior

Final validation confirmed that:

- Wrong passwords fail closed.
- Corrupted metadata is rejected.
- Modified ciphertext is rejected by AEAD authentication.
- Failed decrypt attempts do not produce plaintext output files.

## Operational guidance

- Run only on trusted machines.
- Prefer 48 or 64 character generated passphrases for important accounts.
- Use hidden output when generating or decrypting secrets.
- Use clipboard TTL and disable cloud clipboard sync where possible.
- Store `.apv` files safely and back them up.
- Protect the file-encryption password separately.
- Record known-good binary hashes after local release builds.
- Delete test files, recovered plaintext, tampered files, and build artifacts before publishing.

## Remaining risks

- Endpoint compromise remains the highest-risk scenario.
- TPM-backed encryption is not implemented in the current build.
- Local no-repeat history is local only and can be lost.
- Security depends on strong encryption passwords.
- The project has not received an independent professional cryptographic audit.
