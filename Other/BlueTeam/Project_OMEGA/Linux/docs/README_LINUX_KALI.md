# Project OMEGA — Linux/Kali Guide

This folder contains Linux/Kali-specific launchers and workflow instructions for Project OMEGA.

Use this folder when operating Project OMEGA on Linux or Kali. The Rust source code still builds from the project root.

## Folder contents

```text
Linux/
├── README_LINUX_KALI.md
└── scripts/
    ├── linux_cli.sh
    ├── linux_ui.sh
    └── linux_kali_test.sh
```

## Build from the project root

```bash
source "$HOME/.cargo/env"
cargo fmt --check
cargo check
cargo build --release
```

The release binary is created at:

```text
target/release/aegisphrase
```

## Start the Linux/Kali UI

From the project root:

```bash
chmod +x Linux/scripts/*.sh
./Linux/scripts/linux_ui.sh
```

The UI supports passphrase generation, encrypted vault workflows, file encryption/decryption, tamper checks, TPM device status checks, environment checklist output, history cleanup, and secure exit.

## Use the Linux/Kali CLI wrapper

```bash
chmod +x Linux/scripts/*.sh
./Linux/scripts/linux_cli.sh --help
```

Example secure passphrase generation:

```bash
./Linux/scripts/linux_cli.sh gen --length 48 --hide --copy --clipboard-ttl 20 --secure-exit
```

Example encrypted vault save:

```bash
./Linux/scripts/linux_cli.sh gen --length 48 --hide --save "./secret.apv" --cipher aes256-gcm --secure-exit
```

Example file encryption:

```bash
./Linux/scripts/linux_cli.sh encrypt-file --input "./plain.txt" --output "./plain.txt.apv" --cipher aes256-gcm --force
```

Example file decryption:

```bash
./Linux/scripts/linux_cli.sh decrypt-file --input "./plain.txt.apv" --output "./plain.recovered.txt" --force
```

## Run Linux/Kali smoke tests

```bash
chmod +x Linux/scripts/*.sh
./Linux/scripts/linux_kali_test.sh
```

## Linux/Kali path behavior

Use Linux-style paths:

```bash
./secret.apv
./folder/secret.apv
/home/<user>/Desktop/Project_OMEGA/secret.apv
```

Avoid Windows-style `.\file.apv` paths on Linux. The correct Linux equivalent is `./file.apv`.

The UI supports `back`, `cancel`, or `q` in prompt flows where practical.

## Linux TPM status

The current build checks common TPM device paths:

```text
/dev/tpmrm0
/dev/tpm0
```

Current status:

- Linux TPM status detection: supported.
- Linux TPM-backed encryption/key wrapping: not implemented in this build.
- Future Linux support should use `tpm2-tss` or `tss-esapi` with PCR policy and dedicated tests.

## Final validation status

Linux/Kali validation passed for:

- Format, build, and release checks.
- Linux/Kali smoke script.
- CLI passphrase vault encryption/decryption.
- CLI arbitrary file encryption/decryption.
- UI passphrase vault workflow.
- UI arbitrary file encryption/decryption.
- SHA-256 file round-trip checks.
- TPM device status reporting.
- Executable hash/tamper check.
- Wrong-password, metadata tamper, ciphertext tamper, and controlled crack-resistance validation.

Known-good validation hash from final testing:

```text
e007ec16c62d66c02c030b4ea9eacd55a86e40c89bf7c9dd7ab39307a30e4670
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
omega_crack_attempt.sh
omega-crack-test-wordlist.txt
crack-attempt-output.tmp
```
