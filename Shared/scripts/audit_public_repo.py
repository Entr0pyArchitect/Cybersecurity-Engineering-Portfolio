#!/usr/bin/env python3
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]

def git(*args):
    return subprocess.check_output(
        ["git", "-C", str(ROOT), *args],
        text=True,
        stderr=subprocess.STDOUT,
    )

tracked = git("ls-files").splitlines()
tracked_set = set(tracked)
errors = []

def fail(message):
    errors.append(message)

for req in [
    "README.md",
    "SECURITY.md",
    "LICENSE.md",
    "THIRD_PARTY_NOTICES.md",
    ".github/workflows/portfolio-ci.yml",
]:
    if req not in tracked_set:
        fail(f"required tracked file missing: {req}")

for p in tracked:
    if p.startswith("18-Exploitation-Toolkit/"):
        fail(f"private Project 18 material is tracked: {p}")
    if p.startswith("ProjectMadHatter/Documentation/") and p.lower().endswith(".odt"):
        fail(f"local-only MadHatter ODT is tracked: {p}")
    if "/target/" in f"/{p}/":
        fail(f"build target artifact is tracked: {p}")
    if "/.git/" in f"/{p}/":
        fail(f"nested Git metadata is tracked: {p}")

red_allowed = {
    "Other/RedTeam/README.md",
    "Other/RedTeam/Offensive Projects/README.md",
    "Other/RedTeam/Offensive Tooling/README.md",
}
for p in tracked:
    if p.startswith("Other/RedTeam/") and p not in red_allowed:
        fail(f"unexpected Red Team implementation/artifact is public: {p}")

for p in tracked:
    low = p.lower()
    if p.startswith("Other/BlueTeam/Capstone Project/"):
        if low.endswith((".pdf", ".pptx", ".mp4")):
            fail(f"local-only capstone submission/media artifact is tracked: {p}")
        if "/evidence/screenshots/" in low:
            fail(f"local-only capstone screenshot is tracked: {p}")

sentinels = {
    "legal first-name token": bytes.fromhex("61757374696e"),
    "middle-name token": bytes.fromhex("7368616e65"),
    "surname token": bytes.fromhex("656e67656c73"),
    "university-name token": bytes.fromhex("73742e20616d62726f7365"),
    "course-number token": bytes.fromhex("637363692d343735"),
    "local username token": bytes.fromhex("76657972306e"),
    "local hostname token": bytes.fromhex("736361726563726f77"),
}

binary_exts = {
    ".zip", ".png", ".jpg", ".jpeg", ".gif", ".ico",
    ".pdf", ".pptx", ".mp4", ".odt", ".exe", ".dll", ".bin"
}

for rel in tracked:
    path = ROOT / rel
    low_path = rel.lower().encode("utf-8", errors="ignore")
    for label, token in sentinels.items():
        if token in low_path:
            fail(f"{label} appears in tracked path: {rel}")

    if path.suffix.lower() in binary_exts:
        continue

    try:
        data = path.read_bytes()
    except OSError as exc:
        fail(f"could not read tracked file {rel}: {exc}")
        continue

    if b"\x00" in data:
        continue

    lower = data.lower()
    for label, token in sentinels.items():
        if token in lower:
            fail(f"{label} appears in tracked content: {rel}")

secret_patterns = [
    (re.compile(rb"-----BEGIN (?:RSA |EC |OPENSSH |DSA )?PRIVATE KEY-----"), "private-key block"),
    (re.compile(rb"\bAKIA[0-9A-Z]{16}\b"), "AWS access-key style token"),
    (re.compile(rb"\bghp_[A-Za-z0-9]{20,}\b"), "GitHub classic token style"),
    (re.compile(rb"\bgithub_pat_[A-Za-z0-9_]{20,}\b"), "GitHub fine-grained token style"),
    (re.compile(rb"\bxox[baprs]-[A-Za-z0-9-]{20,}\b"), "Slack token style"),
    (re.compile(rb"\bAIza[0-9A-Za-z_-]{35}\b"), "Google API-key style"),
]

for rel in tracked:
    path = ROOT / rel
    if path.suffix.lower() in binary_exts:
        continue
    try:
        data = path.read_bytes()
    except OSError:
        continue
    for pat, label in secret_patterns:
        if pat.search(data):
            fail(f"{label} found in tracked content: {rel}")

url_cred = re.compile(
    r"(?i)\b(?:postgres|postgresql|mysql|mongodb|redis|https?)://"
    r"([^:/\s]+):([^@\s]+)@"
)
for rel in tracked:
    path = ROOT / rel
    if path.suffix.lower() in binary_exts:
        continue
    try:
        text = path.read_text(encoding="utf-8", errors="ignore")
    except OSError:
        continue
    for m in url_cred.finditer(text):
        password = m.group(2).strip().lower()
        if password not in {
            "change_me", "changeme", "replace_me", "example",
            "example-only", "fake", "fake-password"
        }:
            fail(f"credential-bearing URL with non-placeholder password: {rel}")

readmes = [p for p in tracked if Path(p).name.lower() == "readme.md"]
badge_md = re.compile(r"!\[[^\]]*\]\(https://badges\.ws/")
placeholder_terms = re.compile(r"\b(?:placeholder readme|lorem ipsum)\b", re.I)

for rel in readmes:
    path = ROOT / rel
    text = path.read_text(encoding="utf-8", errors="replace")
    if len(text.strip()) < 80:
        fail(f"README is too thin for public review: {rel}")
    if text.count("```") % 2:
        fail(f"unbalanced fenced code block in README: {rel}")
    if badge_md.search(text):
        fail(f"badges.ws Markdown-image syntax remains in README: {rel}")
    if placeholder_terms.search(text):
        fail(f"placeholder prose remains in README: {rel}")
    for n, line in enumerate(text.splitlines(), 1):
        if line.endswith((" ", "\t")):
            fail(f"trailing whitespace in README {rel}:{n}")

mode_lines = git("ls-files", "-s").splitlines()
modes = {}
for line in mode_lines:
    parts = line.split(None, 3)
    if len(parts) == 4:
        modes[parts[3]] = parts[0]

for rel in tracked:
    if rel.endswith(".sh") and modes.get(rel) != "100755":
        fail(f"tracked shell script is not executable (100755): {rel}")

try:
    out = subprocess.check_output(
        [sys.executable, str(ROOT / "Shared/scripts/check_readme_links.py")],
        text=True,
        stderr=subprocess.STDOUT,
    ).strip()
    if out:
        print(out)
except subprocess.CalledProcessError as exc:
    fail("README link audit failed:\n" + exc.output)

print("\n=== PUBLIC REPOSITORY AUDIT ===")
if errors:
    for item in errors:
        print(f"[FAIL] {item}")
    print(f"\nRESULT: FAIL ({len(errors)} issue(s))")
    sys.exit(1)

print(f"Tracked files checked: {len(tracked)}")
print(f"Tracked READMEs checked: {len(readmes)}")
print("Identity / affiliation OPSEC: PASS")
print("Public/private boundary: PASS")
print("Sensitive token signatures: PASS")
print("README structure / formatting: PASS")
print("Shell-script Git modes: PASS")
print("RESULT: PASS")
