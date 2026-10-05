#!/bin/bash
set -e

python3 scripts/generate_test_data.py

python3 - <<'PY'
from pathlib import Path
import hashlib

pw_file = Path("data/weak_passwords.txt")
hash_file = Path("data/weak_hashes.txt")

passwords = [line.strip() for line in pw_file.read_text(encoding="utf-8").splitlines() if line.strip()]

with hash_file.open("w", encoding="utf-8") as f:
    for pw in passwords:
        f.write(hashlib.sha256(pw.encode()).hexdigest() + "\n")

print(f"[+] Wrote {len(passwords)} SHA-256 hashes to {hash_file}")
PY

# Clear John the Ripper's cached cracked-password results so each baseline run starts clean.
rm -f ~/.john/john.pot

time john --format=raw-sha256 --wordlist=data/weak_passwords.txt data/weak_hashes.txt | tee evidence/baseline/john_run_output.txt
john --show --format=raw-sha256 data/weak_hashes.txt | tee results/baseline_results.txt
