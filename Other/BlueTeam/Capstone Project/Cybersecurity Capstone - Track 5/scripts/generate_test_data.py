#!/usr/bin/env python3
"""
Track 5 Capstone helper script for controlled offline password policy testing.

Purpose:
Generate artificial test users and a weak password dataset for a safe, offline
lab demonstration comparing password policy conditions.

Academic and ethical boundary:
This script is intended for use only in a personal, isolated lab with fake data.
It must not be used against real systems, real credentials, or unauthorized
targets.

Author: Entr0pyArchitect
Course: Cybersecurity Capstone Capstone Seminar
"""

from __future__ import annotations

import csv
import random
from pathlib import Path

# Reproducible output for assignment documentation
random.seed(475)

PROJECT_ROOT = Path(__file__).resolve().parent.parent
DATA_DIR = PROJECT_ROOT / "data"

USERS_CSV = DATA_DIR / "users.csv"
WEAK_PASSWORDS_TXT = DATA_DIR / "weak_passwords.txt"

FIRST_NAMES = [
    "alex", "jamie", "taylor", "jordan", "morgan",
    "casey", "riley", "avery", "cameron", "logan"
]

LAST_NAMES = [
    "smith", "johnson", "miller", "davis", "wilson",
    "anderson", "thomas", "moore", "martin", "clark"
]

WEAK_PASSWORD_POOL = [
    "password",
    "password1",
    "welcome",
    "welcome1",
    "letmein",
    "qwerty",
    "qwerty123",
    "summer2025",
    "fall2025",
    "winter2026",
    "spring2026",
    "iloveyou",
    "admin123",
    "dragon",
    "monkey",
    "abc123",
    "test123",
    "guest123",
    "football",
    "baseball",
]


def build_username(index: int) -> str:
    first = random.choice(FIRST_NAMES)
    last = random.choice(LAST_NAMES)
    return f"{first}.{last}{index:02d}"


def build_users(record_count: int = 12) -> list[tuple[str, str]]:
    """Return a list of (username, password) tuples."""
    users: list[tuple[str, str]] = []

    for i in range(1, record_count + 1):
        username = build_username(i)
        password = random.choice(WEAK_PASSWORD_POOL)
        users.append((username, password))

    return users


def write_users_csv(records: list[tuple[str, str]]) -> None:
    """Write synthetic user-password pairs to users.csv."""
    with USERS_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.writer(csv_file)
        writer.writerow(["username", "password"])
        writer.writerows(records)


def write_passwords_txt(records: list[tuple[str, str]]) -> None:
    """Write passwords only to weak_passwords.txt for hashing workflow."""
    with WEAK_PASSWORDS_TXT.open("w", encoding="utf-8") as txt_file:
        for _, password in records:
            txt_file.write(password + "\n")


def main() -> None:
    DATA_DIR.mkdir(parents=True, exist_ok=True)

    records = build_users(record_count=12)
    write_users_csv(records)
    write_passwords_txt(records)

    print("[+] Synthetic test data generated successfully.")
    print(f"[+] Wrote user records to: {USERS_CSV}")
    print(f"[+] Wrote password list to: {WEAK_PASSWORDS_TXT}")
    print(f"[+] Total records created: {len(records)}")


if __name__ == "__main__":
    main()
