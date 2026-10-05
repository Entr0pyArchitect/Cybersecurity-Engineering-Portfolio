#!/usr/bin/env python3
"""
Track 5 Capstone Assignment 4
Password policy enforcement validator for the strong-password dataset.

Purpose:
Validate that passwords in data/strong_passwords.txt meet the project's
strong-policy requirements before being accepted into the dataset.

Policy rules:
- Minimum length of 14 characters
- At least one uppercase letter
- At least one lowercase letter
- At least one digit
- At least one symbol
- Must not match a weak-password denylist
- Must not contain spaces

Academic and ethical boundary:
This script is intended only for use in a personal, isolated lab with fake data.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parent.parent
PASSWORD_FILE = PROJECT_ROOT / "data" / "strong_passwords.txt"

WEAK_DENYLIST = {
    "password",
    "password1",
    "welcome",
    "welcome1",
    "letmein",
    "qwerty",
    "qwerty123",
    "admin123",
    "guest123",
    "football",
    "monkey",
    "abc123",
    "test123",
    "summer2025",
    "spring2026",
}


def validate_password(password: str) -> list[str]:
    """Return a list of validation errors for one password."""
    errors: list[str] = []

    if len(password) < 14:
        errors.append("must be at least 14 characters long")

    if " " in password:
        errors.append("must not contain spaces")

    if not re.search(r"[A-Z]", password):
        errors.append("must contain at least one uppercase letter")

    if not re.search(r"[a-z]", password):
        errors.append("must contain at least one lowercase letter")

    if not re.search(r"[0-9]", password):
        errors.append("must contain at least one digit")

    if not re.search(r"[^A-Za-z0-9]", password):
        errors.append("must contain at least one symbol")

    if password.lower() in WEAK_DENYLIST:
        errors.append("matches a weak-password denylist entry")

    return errors


def main() -> int:
    if not PASSWORD_FILE.exists():
        print(f"[!] Password file not found: {PASSWORD_FILE}")
        return 1

    passwords = [
        line.strip()
        for line in PASSWORD_FILE.read_text(encoding="utf-8").splitlines()
        if line.strip()
    ]

    if not passwords:
        print("[!] No passwords found in strong_passwords.txt")
        return 1

    print(f"[+] Validating {len(passwords)} password entries from {PASSWORD_FILE}")

    failures = 0

    for index, password in enumerate(passwords, start=1):
        errors = validate_password(password)
        if errors:
            failures += 1
            print(f"\n[BLOCKED] Entry {index}: {password}")
            for error in errors:
                print(f"  - {error}")

    if failures:
        print(f"\n[!] Validation failed: {failures} password(s) violate policy.")
        print("[!] Commit/check should be blocked until the dataset is corrected.")
        return 1

    print("\n[+] Validation passed: all passwords meet the strong-policy requirements.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
