#!/usr/bin/env python3
"""
Track 5 Capstone helper script for stronger password policy testing.

Purpose:
Generate artificial test users and a strong password dataset for a safe, offline
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

random.seed(476)

PROJECT_ROOT = Path(__file__).resolve().parent.parent
DATA_DIR = PROJECT_ROOT / "data"

STRONG_USERS_CSV = DATA_DIR / "strong_users.csv"
STRONG_PASSWORDS_TXT = DATA_DIR / "strong_passwords.txt"

FIRST_NAMES = [
    "alex", "jamie", "taylor", "jordan", "morgan",
    "casey", "riley", "avery", "cameron", "logan"
]

LAST_NAMES = [
    "smith", "johnson", "miller", "davis", "wilson",
    "anderson", "thomas", "moore", "martin", "clark"
]

STRONG_PASSWORD_POOL = [
    "BlueRiver!Train47",
    "Orbit#Glass92Tiger",
    "NorthWind$Echo731",
    "Marble!Circuit88Fox",
    "SolarFlame@Bridge64",
    "Velvet#Signal29Wolf",
    "Copper!Lantern55Sky",
    "Frozen$Harbor81Crow",
    "Silent!Radar33Stone",
    "Crimson#Engine74Leaf",
    "Golden!Orbit66Pine",
    "Shadow$Harbor12Dawn"
]


def build_username(index: int) -> str:
    first = random.choice(FIRST_NAMES)
    last = random.choice(LAST_NAMES)
    return f"{first}.{last}{index:02d}"


def build_users(record_count: int = 12) -> list[tuple[str, str]]:
    users: list[tuple[str, str]] = []

    for i in range(1, record_count + 1):
        username = build_username(i)
        password = STRONG_PASSWORD_POOL[i - 1]
        users.append((username, password))

    return users


def write_users_csv(records: list[tuple[str, str]]) -> None:
    with STRONG_USERS_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.writer(csv_file)
        writer.writerow(["username", "password"])
        writer.writerows(records)


def write_passwords_txt(records: list[tuple[str, str]]) -> None:
    with STRONG_PASSWORDS_TXT.open("w", encoding="utf-8") as txt_file:
        for _, password in records:
            txt_file.write(password + "\n")


def main() -> None:
    DATA_DIR.mkdir(parents=True, exist_ok=True)

    records = build_users(record_count=12)
    write_users_csv(records)
    write_passwords_txt(records)

    print("[+] Strong synthetic test data generated successfully.")
    print(f"[+] Wrote strong user records to: {STRONG_USERS_CSV}")
    print(f"[+] Wrote strong password list to: {STRONG_PASSWORDS_TXT}")
    print(f"[+] Total records created: {len(records)}")


if __name__ == "__main__":
    main()
