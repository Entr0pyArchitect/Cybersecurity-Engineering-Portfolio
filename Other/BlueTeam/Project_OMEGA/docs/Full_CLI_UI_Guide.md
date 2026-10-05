# Project OMEGA Full CLI and UI Guide

## Overview

Project OMEGA supports both direct command-line usage and interactive terminal UI workflows. The Rust core is cross-platform, but daily operation should use the platform-specific folder for your environment.

- Windows 11: `Win11/scripts/`
- Linux/Kali: `Linux/scripts/`

Use the root only for building, auditing, and source maintenance.

## Build from source

From the project root:

```bash
cargo fmt --check
cargo check
cargo build --release
```

Windows produces:

```text
target\release\aegisphrase.exe
```

Linux/Kali produces:

```text
target/release/aegisphrase
```

## Windows 11 usage

### Start the Windows UI

```powershell
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
.\Win11\scripts\win11_ui.ps1
```

### Use the Windows CLI wrapper

```powershell
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
.\Win11\scripts\win11_cli.ps1 --help
```

### Run Windows smoke tests

```powershell
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
.\Win11\scripts\windows_test.ps1
```

### Windows path examples

Use current-directory paths or full paths:

```powershell
.\secret.apv
.\folder\secret.apv
C:\Users\<User>\Desktop\Project_OMEGA\secret.apv
```

Do not commit real secrets, real `.apv` files, recovered plaintext, or local test artifacts.

## Linux/Kali usage

### Start the Linux/Kali UI

```bash
chmod +x Linux/scripts/*.sh
./Linux/scripts/linux_ui.sh
```

### Use the Linux/Kali CLI wrapper

```bash
chmod +x Linux/scripts/*.sh
./Linux/scripts/linux_cli.sh --help
```

### Run Linux/Kali smoke tests

```bash
chmod +x Linux/scripts/*.sh
./Linux/scripts/linux_kali_test.sh
```

### Linux/Kali path examples

Use Linux-style current-directory paths or full paths:

```bash
./secret.apv
./folder/secret.apv
/home/<user>/Desktop/Project_OMEGA/secret.apv
```

Avoid Windows-style `.
ile.apv` paths on Linux. Use `./file.apv` instead.

## Interactive UI guide

The UI uses status markers:

```text
[*] Information or operation started
[+] Success
[!] Warning or important limitation
[-] Error
```

Main menu:

```text
[1] Create secure passphrase
[2] Encrypt / decrypt vaults and .apv file envelopes
[3] Tamper check
[4] Terminal cleanup
[5] TPM 2.0 status
[6] Environment checklist
[7] Secure exit
```

Most prompt flows support:

```text
back
cancel
q
```

Use these to return to the previous menu without completing the action.

## Generate a secure passphrase

Recommended UI settings:

```text
Length: 48 or 64
Charset: safe or full
Add hidden user entropy: yes
Hide generated secret: yes
Copy to clipboard: yes, if needed
Clipboard TTL: 20 seconds or less
Save encrypted vault: yes, if storage is needed
Cipher: AES-256-GCM default, or another supported AEAD mode
```

CLI example:

Windows:

```powershell
.\target\release\aegisphrase.exe gen --length 48 --hide --copy --clipboard-ttl 20 --secure-exit
```

Linux/Kali:

```bash
./target/release/aegisphrase gen --length 48 --hide --copy --clipboard-ttl 20 --secure-exit
```

## Save an encrypted passphrase vault

Windows:

```powershell
.\target\release\aegisphrase.exe gen --length 48 --hide --save ".\secret.apv" --cipher aes256-gcm --secure-exit
```

Linux/Kali:

```bash
./target/release/aegisphrase gen --length 48 --hide --save "./secret.apv" --cipher aes256-gcm --secure-exit
```

Use a strong encryption password. The tool enforces a minimum length, but minimum length alone is not enough for high-value secrets.

## Decrypt a passphrase vault

Windows:

```powershell
.\target\release\aegisphrase.exe decrypt --input ".\secret.apv" --hide --copy --clipboard-ttl 10 --secure-exit
```

Linux/Kali:

```bash
./target/release/aegisphrase decrypt --input "./secret.apv" --hide --copy --clipboard-ttl 10 --secure-exit
```

Wrong passwords or tampered files fail closed.

## Encrypt an arbitrary file

Windows:

```powershell
.\target\release\aegisphrase.exe encrypt-file --input ".\plain.txt" --output ".\plain.txt.apv" --cipher aes256-gcm --force
```

Linux/Kali:

```bash
./target/release/aegisphrase encrypt-file --input "./plain.txt" --output "./plain.txt.apv" --cipher aes256-gcm --force
```

## Decrypt an arbitrary file

Windows:

```powershell
.\target\release\aegisphrase.exe decrypt-file --input ".\plain.txt.apv" --output ".\plain.recovered.txt" --force
```

Linux/Kali:

```bash
./target/release/aegisphrase decrypt-file --input "./plain.txt.apv" --output "./plain.recovered.txt" --force
```

Verify file round-trip with SHA-256:

Windows:

```powershell
$h1 = (Get-FileHash ".\plain.txt" -Algorithm SHA256).Hash
$h2 = (Get-FileHash ".\plain.recovered.txt" -Algorithm SHA256).Hash
if ($h1 -eq $h2) { "PASS" } else { "FAIL" }
```

Linux/Kali:

```bash
sha256sum ./plain.txt ./plain.recovered.txt
```

The two Linux hashes should match.

## Tamper check

Print executable hash:

Windows:

```powershell
.\target\release\aegisphrase.exe doctor
```

Linux/Kali:

```bash
./target/release/aegisphrase doctor
```

Verify against a known-good hash:

Windows:

```powershell
.\target\release\aegisphrase.exe doctor --expected-sha256 <known-good-hash>
```

Linux/Kali:

```bash
./target/release/aegisphrase doctor --expected-sha256 <known-good-hash>
```

## TPM status

Windows:

```powershell
.\target\release\aegisphrase.exe tpm-status
```

Linux/Kali:

```bash
./target/release/aegisphrase tpm-status
```

Current build reports TPM status and documents future TPM-backed key wrapping. It does not claim TPM-backed file encryption is complete.

## Terminal cleanup

Windows:

```powershell
.\target\release\aegisphrase.exe clean-history
```

Linux/Kali:

```bash
./target/release/aegisphrase clean-history
```

Cleanup is best-effort only. It cannot erase screenshots, transcripts, malware logs, external clipboard managers, or data copied into other applications.

## Final validation process

Recommended release validation:

```bash
cargo fmt --check
cargo check
cargo build --release
cargo audit
```

Then validate both environments:

1. Run platform smoke script.
2. Run CLI vault encrypt/decrypt.
3. Run CLI file encrypt/decrypt round-trip.
4. Run UI vault workflow.
5. Run UI file workflow.
6. Run wrong-password tests.
7. Run metadata and ciphertext tamper tests.
8. Run controlled dictionary validation.
9. Remove all test artifacts.
10. Push only clean source and documentation.

## Files that should not be committed

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
