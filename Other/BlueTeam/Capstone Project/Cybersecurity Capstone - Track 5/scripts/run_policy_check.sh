#!/bin/bash
set -e

echo "[+] Running Track 5 password policy enforcement check..."
python3 scripts/validate_password_policy.py
echo "[+] Policy check passed."
