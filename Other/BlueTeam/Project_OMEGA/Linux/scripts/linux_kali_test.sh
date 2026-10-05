#!/usr/bin/env bash
set -euo pipefail

echo "[*] Project OMEGA Linux/Kali smoke test"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
EXE="./target/release/aegisphrase"

echo "[*] Running formatting and build gates"
cargo fmt --check
cargo check
cargo build --release

echo "[*] Running command surface checks"
"$EXE" --help
"$EXE" checklist
"$EXE" gen --length 30 --hide
"$EXE" gen --length 64 --hide
"$EXE" doctor
"$EXE" tpm-status

echo "[+] Linux/Kali smoke test completed"
echo "[!] Manual prompt-based encryption and UI tests still required. See Linux/docs/TEST_PLAN_LINUX_KALI.md and docs/UI_TEST_PLAN_BOTH_ENVIRONMENTS.md"
