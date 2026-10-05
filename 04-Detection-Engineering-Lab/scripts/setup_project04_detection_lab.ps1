# Project 04 - Detection Engineering Lab Bootstrap
# Run from:
# Cybersecurity-Engineering-Portfolio\04-Detection-Engineering-Lab

$ErrorActionPreference = "Stop"

$timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
$utf8NoBom = [System.Text.UTF8Encoding]::new($false)

function Write-NoBom {
    param (
        [string]$Path,
        [string]$Content
    )
    $parent = Split-Path -Parent $Path
    if ($parent -and -not (Test-Path $parent)) {
        New-Item -ItemType Directory -Force -Path $parent | Out-Null
    }
    [System.IO.File]::WriteAllText($Path, $Content, $utf8NoBom)
}

Write-Host "[+] Bootstrapping Project 04 - Detection Engineering Lab"

New-Item -ItemType Directory -Force -Path ".\.backup" | Out-Null

if (Test-Path ".\README.md") {
    Copy-Item ".\README.md" ".\.backup\README.md.bak-$timestamp" -Force
}

$folders = @(
    "attacks\mock-events",
    "attacks\simulation-plans",
    "detections\sigma",
    "detections\suricata",
    "docs",
    "pipelines",
    "terraform",
    "scripts",
    "testdata\windows",
    "testdata\network",
    "evidence\screenshots",
    "evidence\terminal_output",
    "evidence\test_logs"
)

foreach ($folder in $folders) {
    New-Item -ItemType Directory -Force -Path $folder | Out-Null
}

Write-NoBom ".\.gitignore" @'
# Local environment
.env
.env.local

# Python
__pycache__/
*.pyc
.venv/
venv/

# Terraform local state/cache
.terraform/
*.tfstate
*.tfstate.*
.terraform.lock.hcl

# Evidence that may contain local paths or screenshots
evidence/screenshots/*
evidence/terminal_output/*
evidence/test_logs/*

# Keep evidence folders
!evidence/screenshots/.gitkeep
!evidence/terminal_output/.gitkeep
!evidence/test_logs/.gitkeep

# Local backups
.backup/
'@

New-Item -ItemType File -Force -Path ".\evidence\screenshots\.gitkeep" | Out-Null
New-Item -ItemType File -Force -Path ".\evidence\terminal_output\.gitkeep" | Out-Null
New-Item -ItemType File -Force -Path ".\evidence\test_logs\.gitkeep" | Out-Null

Write-NoBom ".\testdata\windows\process_events.json" @'
[
  {
    "event_id": "WIN-PROC-001",
    "timestamp_utc": "2026-05-14T00:00:00Z",
    "host": "LAB-WIN-01",
    "source": "mock_windows_process_log",
    "event_type": "process_creation",
    "process_name": "powershell.exe",
    "parent_process_name": "cmd.exe",
    "command_line": "powershell.exe -NoProfile -File C:\\Lab\\maintenance.ps1",
    "username": "LAB\\student",
    "expected_detection": "script_interpreter_activity",
    "notes": "Synthetic event for validating script interpreter detection logic. Not collected from a real target."
  },
  {
    "event_id": "WIN-PROC-002",
    "timestamp_utc": "2026-05-14T00:05:00Z",
    "host": "LAB-WIN-01",
    "source": "mock_windows_process_log",
    "event_type": "process_creation",
    "process_name": "notepad.exe",
    "parent_process_name": "explorer.exe",
    "command_line": "notepad.exe C:\\Lab\\notes.txt",
    "username": "LAB\\student",
    "expected_detection": "none",
    "notes": "Benign synthetic event used to check false-positive behavior."
  }
]
'@

Write-NoBom ".\testdata\network\http_events.json" @'
[
  {
    "event_id": "NET-HTTP-001",
    "timestamp_utc": "2026-05-14T00:10:00Z",
    "src_ip": "10.10.10.25",
    "dest_ip": "10.10.10.50",
    "dest_port": 80,
    "protocol": "http",
    "http_method": "GET",
    "uri": "/lab/download/example-tool.exe",
    "user_agent": "LabValidationClient/1.0",
    "expected_detection": "http_executable_download",
    "notes": "Synthetic HTTP event for validating a lab-only executable download detection."
  },
  {
    "event_id": "NET-HTTP-002",
    "timestamp_utc": "2026-05-14T00:15:00Z",
    "src_ip": "10.10.10.25",
    "dest_ip": "10.10.10.50",
    "dest_port": 80,
    "protocol": "http",
    "http_method": "GET",
    "uri": "/lab/index.html",
    "user_agent": "LabValidationClient/1.0",
    "expected_detection": "none",
    "notes": "Benign synthetic HTTP event used to check false-positive behavior."
  }
]
'@

Write-NoBom ".\detections\sigma\script_interpreter_activity.yml" @'
title: Lab Script Interpreter Activity
id: 00000000-0000-0000-0000-000000000401
status: experimental
description: Lab-safe Sigma-style detection for script interpreter process activity in synthetic process logs.
author: Entr0pyArchitect
date: 2026/05/14
references:
  - https://attack.mitre.org/techniques/T1059/
tags:
  - attack.execution
  - attack.t1059
logsource:
  category: process_creation
detection:
  selection:
    process_name:
      - powershell.exe
      - cmd.exe
      - wscript.exe
      - cscript.exe
      - bash.exe
      - python.exe
  condition: selection
falsepositives:
  - Administrative scripts
  - Developer automation
  - Login scripts
level: low
'@

Write-NoBom ".\detections\suricata\lab_http_executable_download.rules" @'
alert http any any -> any any (msg:"LAB Possible Executable Download Over HTTP"; flow:established,to_server; http.uri; content:".exe"; nocase; classtype:policy-violation; sid:4000001; rev:1;)
'@

Write-NoBom ".\detections\detection_tests.json" @'
[
  {
    "id": "script_interpreter_activity",
    "description": "Detect synthetic process events where the process name is a common script interpreter.",
    "dataset": "testdata/windows/process_events.json",
    "field": "process_name",
    "match_any": ["powershell.exe", "cmd.exe", "wscript.exe", "cscript.exe", "bash.exe", "python.exe"],
    "expected_matching_event_ids": ["WIN-PROC-001"],
    "expected_non_matching_event_ids": ["WIN-PROC-002"]
  },
  {
    "id": "http_executable_download",
    "description": "Detect synthetic HTTP events where the URI contains an executable file extension.",
    "dataset": "testdata/network/http_events.json",
    "field": "uri",
    "match_any": [".exe"],
    "expected_matching_event_ids": ["NET-HTTP-001"],
    "expected_non_matching_event_ids": ["NET-HTTP-002"]
  }
]
'@

Write-NoBom ".\pipelines\validate_detections.py" @'
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, List


ROOT = Path(__file__).resolve().parents[1]
TESTS_PATH = ROOT / "detections" / "detection_tests.json"
EVIDENCE_DIR = ROOT / "evidence" / "test_logs"


def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def normalize(value: Any) -> str:
    if value is None:
        return ""
    return str(value).lower()


def run_test(test: Dict[str, Any]) -> Dict[str, Any]:
    dataset_path = ROOT / test["dataset"]
    events: List[Dict[str, Any]] = load_json(dataset_path)

    field = test["field"]
    needles = [normalize(item) for item in test["match_any"]]

    matching_ids = []
    non_matching_ids = []

    for event in events:
        value = normalize(event.get(field))
        event_id = event.get("event_id", "unknown-event")

        if any(needle in value for needle in needles):
            matching_ids.append(event_id)
        else:
            non_matching_ids.append(event_id)

    expected_matching = sorted(test.get("expected_matching_event_ids", []))
    expected_non_matching = sorted(test.get("expected_non_matching_event_ids", []))

    actual_matching = sorted(matching_ids)
    actual_non_matching = sorted(non_matching_ids)

    passed = actual_matching == expected_matching and actual_non_matching == expected_non_matching

    return {
        "id": test["id"],
        "description": test["description"],
        "dataset": test["dataset"],
        "field": field,
        "match_any": test["match_any"],
        "expected_matching_event_ids": expected_matching,
        "actual_matching_event_ids": actual_matching,
        "expected_non_matching_event_ids": expected_non_matching,
        "actual_non_matching_event_ids": actual_non_matching,
        "passed": passed,
    }


def main() -> int:
    tests = load_json(TESTS_PATH)
    results = [run_test(test) for test in tests]

    report = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "project": "04-Detection-Engineering-Lab",
        "scope": "synthetic lab data only",
        "summary": {
            "total": len(results),
            "passed": sum(1 for item in results if item["passed"]),
            "failed": sum(1 for item in results if not item["passed"]),
        },
        "results": results,
    }

    EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
    timestamp = datetime.now(timezone.utc).strftime("%Y%m%d-%H%M%S")
    output_path = EVIDENCE_DIR / f"detection_validation_{timestamp}.json"

    with output_path.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)

    print(json.dumps(report, indent=2))
    print(f"\n[+] Validation report written to: {output_path}")

    return 0 if report["summary"]["failed"] == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
'@

Write-NoBom ".\scripts\validate.ps1" @'
python .\pipelines\validate_detections.py
'@

Write-NoBom ".\scripts\capture_baseline.ps1" @'
$timestamp = Get-Date -Format "yyyyMMdd-HHmmss"

New-Item -ItemType Directory -Force -Path ".\evidence\terminal_output" | Out-Null
New-Item -ItemType Directory -Force -Path ".\evidence\test_logs" | Out-Null

python --version | Tee-Object ".\evidence\terminal_output\python_version_$timestamp.txt"

python .\pipelines\validate_detections.py |
    Tee-Object ".\evidence\test_logs\detection_validation_console_$timestamp.txt"
'@

Write-NoBom ".\attacks\mock-events\README.md" @'
# Mock Events

This folder is for synthetic events and safe replay notes only.

Do not place real attack payloads, malware, private logs, credentials, or unauthorized target data here.

Project 04 starts by validating detections against synthetic events because the goal is to prove detection logic safely before using any live lab simulation.
'@

Write-NoBom ".\attacks\simulation-plans\safe-simulation-plan.md" @'
# Safe Simulation Plan

## Purpose
Document safe, lab-bounded simulation ideas for validating detections.

## Current MVP
The MVP does not execute attack tools. It uses synthetic process and network events.

## Future Direction
After the synthetic validation loop works, future lab phases may use intentionally vulnerable systems or safe simulation frameworks inside an owned lab only.

## Rules
- No third-party targets.
- No real malware.
- No credential theft.
- No persistence.
- No stealth.
- No public internet exposure.
- Validate defensive telemetry and detection behavior only.
'@

Write-NoBom ".\terraform\README.md" @'
# Terraform Notes

Terraform is reserved for future reproducible lab infrastructure.

The current MVP does not require Terraform. Do not run cloud resources until the local synthetic validation pipeline is stable and cost/scope boundaries are documented.

Future Terraform goals:
- Local lab topology documentation.
- Reproducible VM/container definitions.
- Safe detection validation environment.
- No public exposure by default.
'@

Write-NoBom ".\terraform\main.tf" @'
# Placeholder for future lab infrastructure.
# Current MVP does not require Terraform execution.
# Keep this project local and synthetic until the validation pipeline is stable.
'@

Write-NoBom ".\docs\project-charter.md" @'
# Project Charter

## Project
04-Detection-Engineering-Lab

## Purpose
Build a reproducible detection engineering lab that validates detection logic against safe, synthetic lab data before moving into deeper lab simulations.

## Scope
The MVP validates Sigma-style and Suricata-style detection ideas against mock process and network events. It does not execute attack tools or target real systems.

## Non-Goals
- No real-world targeting
- No malware execution
- No credential theft
- No persistence
- No stealth behavior
- No public internet exposure
- No production deployment

## Success Criteria
- Synthetic process events exist.
- Synthetic network events exist.
- Detection test definitions exist.
- Validation pipeline runs.
- Evidence report is generated.
- Documentation explains what was tested and why.
'@

Write-NoBom ".\docs\design.md" @'
# Design Notes

## MVP Architecture

Synthetic process/network events -> detection test definitions -> validation pipeline -> evidence report

## Current Design
The MVP uses synthetic JSON datasets and a Python validation pipeline. Detection rules are represented as Sigma-style and Suricata-style artifacts, while detection tests define what should match and what should not.

## Why Start With Synthetic Events
Detection engineering needs a safe feedback loop. Synthetic events let the project prove rule intent, false-positive thinking, and validation structure without executing tools or touching real systems.

## Connected Projects
- Project 02 provides threat-informed backlog items.
- Project 03 provides future telemetry events.
- Project 15 will become the SIEM query library.
- Project 06 may later consume validated alerts for controlled response.
'@

Write-NoBom ".\docs\validation-plan.md" @'
# Validation Plan

## Baseline Checks
- `python --version` works.
- `python .\pipelines\validate_detections.py` runs.
- The validation report shows all tests passed.
- Evidence is written under `evidence/test_logs`.

## Evidence to Capture
- Python version output.
- Validation console output.
- JSON validation report.
- Screenshots of successful validation.
- Notes explaining what each detection is proving.

## MVP Limitations
This MVP validates logic against synthetic events. It does not yet connect to live telemetry, SIEMs, packet captures, Atomic Red Team, Terraform labs, or Project 03 event streams.
'@

Write-NoBom ".\docs\source-map.md" @'
# Source Map

Use official or primary sources first.

## Core References
- MITRE ATT&CK Enterprise: https://attack.mitre.org/
- SigmaHQ: https://sigmahq.io/
- Sigma Rule Repository: https://github.com/SigmaHQ/sigma
- Suricata Documentation: https://docs.suricata.io/
- NIST Cybersecurity Framework: https://www.nist.gov/cyberframework
- CISA Cybersecurity Resources: https://www.cisa.gov/cybersecurity
- Python Documentation: https://docs.python.org/3/

## Source Rules
- Prefer official documentation.
- Record date accessed in notes.
- Do not copy rules blindly.
- Explain the telemetry requirement behind every detection.
- Separate lab assumptions from real-world claims.
'@

Write-NoBom ".\docs\development.md" @'
# Development Guide

## Local Development

Check Python:

    python --version

Run validation:

    python .\pipelines\validate_detections.py

Run via PowerShell helper:

    Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
    .\scripts\validate.ps1

Capture baseline evidence:

    .\scripts\capture_baseline.ps1

## Development Rules
- Use synthetic or owned-lab data only.
- Do not include real private logs.
- Do not execute attack tools in the MVP.
- Keep every detection tied to telemetry and false-positive notes.
- Validate both matching and non-matching events.
'@

Write-NoBom ".\README.md" @'
# 04-Detection-Engineering-Lab

## Project Status
Development-ready MVP scaffold.

## Executive Summary
The Detection Engineering Lab is a safe validation environment for testing detection logic against synthetic lab events. The MVP uses mock process and network events, Sigma-style and Suricata-style detection artifacts, and a Python validation pipeline that checks expected matches and non-matches.

## Current MVP
- Synthetic Windows process events
- Synthetic HTTP network events
- Sigma-style detection artifact
- Suricata-style detection artifact
- Detection test definitions
- Python validation pipeline
- Evidence output
- Documentation and source map

## Safety Scope
This MVP does not execute attacks, malware, credential theft, persistence, stealth, or real-world targeting. It validates detection logic using synthetic events only.

## Validate
Run:

    python .\pipelines\validate_detections.py

Or:

    Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
    .\scripts\validate.ps1

Expected result:
- total tests: 2
- passed: 2
- failed: 0

## Evidence
Evidence is saved under:

    evidence/test_logs/
    evidence/terminal_output/
    evidence/screenshots/

## Future Roadmap
- Add more synthetic event types.
- Add Project 03 telemetry event compatibility.
- Add Sigma-to-query notes.
- Add SIEM query examples for Project 15.
- Add safe lab simulation only after synthetic validation is stable.
'@

Write-NoBom ".\CHANGELOG.md" @'
# Changelog

## v0.1.0 - Initial MVP Scaffold

### Added
- Synthetic Windows process events
- Synthetic HTTP network events
- Sigma-style detection rule
- Suricata-style lab rule
- Detection test definitions
- Python validation pipeline
- Evidence folders
- Documentation set
- Safe simulation notes

### Safety Scope
- Synthetic lab data only
- No real-world targeting
- No attack execution in MVP
'@

Write-Host "[+] Project 04 scaffold complete."
Write-Host "[+] Run these next:"
Write-Host "    python --version"
Write-Host "    python .\pipelines\validate_detections.py"
