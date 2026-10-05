#!/usr/bin/env python3
from pathlib import Path
from urllib.parse import unquote
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]

tracked = subprocess.check_output(
    ["git", "-C", str(ROOT), "ls-files"],
    text=True,
).splitlines()

readmes = [ROOT / p for p in tracked if Path(p).name.lower() == "readme.md"]

link_re = re.compile(r"\[[^\]]*\]\(([^)]+)\)")
broken = []

for md in readmes:
    text = md.read_text(encoding="utf-8", errors="replace")
    for raw in link_re.findall(text):
        target = raw.strip()
        if not target or target.startswith(("http://", "https://", "mailto:", "#")):
            continue
        target = unquote(target.split("#", 1)[0])
        candidate = (md.parent / target).resolve()
        if not candidate.exists():
            broken.append((str(md.relative_to(ROOT)), raw))

if broken:
    print("Broken relative README links:")
    for md, target in broken:
        print(f"  {md}: {target}")
    sys.exit(1)

print(f"Validated relative links across {len(readmes)} tracked README files.")
