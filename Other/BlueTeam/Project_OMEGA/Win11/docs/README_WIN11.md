# Project OMEGA — Windows 11 Guide

This folder contains Windows 11-specific launchers and workflow instructions for Project OMEGA.

Use this folder when operating Project OMEGA on Windows 11. The Rust source code still builds from the project root.

## Folder contents

```text
Win11/
├── README_WIN11.md
└── scripts/
    ├── win11_cli.ps1
    ├── win11_ui.ps1
    └── windows_test.ps1
```

## Build from the project root

```powershell
cargo fmt --check
cargo check
cargo build --release
```

The release binary is created at:

```text
target\release\aegisphrase.exe
```

## Start the Windows UI

From the project root:

```powershell
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
.\Win11\scripts\win11_ui.ps1
```

The UI supports passphrase generation, encrypted vault workflows, file encryption/decryption, tamper checks, TPM status checks, environment checklist output, history cleanup, and secure exit.

## Use the Windows CLI wrapper

```powershell
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
.\Win11\scripts\win11_cli.ps1 --help
```

Example secure passphrase generation:

```powershell
.\Win11\scripts\win11_cli.ps1 gen --length 48 --hide --copy --clipboard-ttl 20 --secure-exit
```

Example encrypted vault save:

```powershell
.\Win11\scripts\win11_cli.ps1 gen --length 48 --hide --save ".\secret.apv" --cipher aes256-gcm --secure-exit
```

Example file encryption:

```powershell
.\Win11\scripts\win11_cli.ps1 encrypt-file --input ".\plain.txt" --output ".\plain.txt.apv" --cipher aes256-gcm --force
```

Example file decryption:

```powershell
.\Win11\scripts\win11_cli.ps1 decrypt-file --input ".\plain.txt.apv" --output ".\plain.recovered.txt" --force
```

## Run Windows smoke tests

```powershell
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
.\Win11\scripts\windows_test.ps1
```

## Windows path behavior

The Windows UI accepts current-directory paths and full paths:

```powershell
.\secret.apv
.\folder\secret.apv
C:\Users\<User>\Desktop\Project_OMEGA\secret.apv
```

The UI supports `back`, `cancel`, or `q` in prompt flows where practical.

## TPM 2.0 status

The current build checks TPM status through Windows PowerShell. Run PowerShell as Administrator for the most reliable TPM output.

Current status:

- TPM detection/status: supported.
- TPM-backed encryption/key wrapping: not implemented in this build.
- Future TPM support should use Windows CNG/NCrypt with Microsoft Platform Crypto Provider and documented recovery behavior.

## Final validation status

Windows 11 validation passed for:

- Format, build, and release checks.
- Windows smoke script.
- CLI passphrase vault encryption/decryption.
- CLI arbitrary file encryption/decryption.
- UI passphrase vault workflow.
- UI arbitrary file encryption/decryption.
- SHA-256 file round-trip checks.
- TPM status detection.
- Executable hash/tamper check.

Known-good validation hash from final testing:

```text
39644150dd6fd05553839a3b82ab7aa267db2286b38765797322033eb7550f51
```

This hash is build-environment specific. Rebuilding can produce a different hash.

## Cleanup reminder

Do not commit:

```text
target/
*.apv
*.recovered.txt
*sample*.txt
*final*.txt
*tampered*
```
