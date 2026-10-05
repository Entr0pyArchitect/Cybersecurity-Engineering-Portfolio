# Project 08 - Secure Software Supply Chain Lab Bootstrap
# Run from:
# Cybersecurity-Engineering-Portfolio\08-Secure-Software-Supply-Chain-Lab

$ErrorActionPreference = "Stop"
$timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
$utf8NoBom = [System.Text.UTF8Encoding]::new($false)

function Write-NoBom {
    param ([string]$Path, [string]$Content)
    $parent = Split-Path -Parent $Path
    if ($parent -and -not (Test-Path $parent)) {
        New-Item -ItemType Directory -Force -Path $parent | Out-Null
    }
    [System.IO.File]::WriteAllText($Path, $Content, $utf8NoBom)
}

Write-Host "[+] Bootstrapping Project 08 - Secure Software Supply Chain Lab"

New-Item -ItemType Directory -Force -Path ".\.backup" | Out-Null
foreach ($file in @("README.md", "requirements.txt")) {
    if (Test-Path $file) {
        Copy-Item $file ".\.backup\$file.bak-$timestamp" -Force
    }
}

$folders = @(
    "docs",
    "pipelines",
    "policy",
    "scripts",
    "sbom",
    "scan-results",
    "testdata\configs",
    "testdata\manifests",
    "evidence\screenshots",
    "evidence\terminal_output",
    "evidence\test_logs"
)

foreach ($folder in $folders) {
    New-Item -ItemType Directory -Force -Path $folder | Out-Null
}

Write-NoBom ".\.gitignore" @'
.env
.env.local
__pycache__/
*.pyc
.venv/
venv/
.pytest_cache/
evidence/screenshots/*
evidence/terminal_output/*
evidence/test_logs/*
scan-results/*
!evidence/screenshots/.gitkeep
!evidence/terminal_output/.gitkeep
!evidence/test_logs/.gitkeep
!scan-results/.gitkeep
.backup/
'@

New-Item -ItemType File -Force -Path ".\evidence\screenshots\.gitkeep" | Out-Null
New-Item -ItemType File -Force -Path ".\evidence\terminal_output\.gitkeep" | Out-Null
New-Item -ItemType File -Force -Path ".\evidence\test_logs\.gitkeep" | Out-Null
New-Item -ItemType File -Force -Path ".\scan-results\.gitkeep" | Out-Null

Write-NoBom ".\requirements.txt" @'
# MVP uses Python standard library only.
# Future tools to research manually:
# gitleaks, syft, grype, osv-scanner, trivy, OpenSSF Scorecard
'@

Write-NoBom ".\policy\supply_chain_controls.json" @'
{
  "project": "08-Secure-Software-Supply-Chain-Lab",
  "scope": "local synthetic validation only",
  "required_controls": [
    {"id":"SSC-001","name":"Dependency version pinned","field":"version_pinned","expected":true,"why":"Pinned versions improve reproducibility and reduce unexpected dependency drift."},
    {"id":"SSC-002","name":"License documented","field":"license_documented","expected":true,"why":"License visibility helps avoid legal and compliance surprises."},
    {"id":"SSC-003","name":"Package source trusted","field":"source_trusted","expected":true,"why":"Dependencies should come from expected registries or controlled internal sources."},
    {"id":"SSC-004","name":"Integrity hash recorded","field":"integrity_hash_recorded","expected":true,"why":"Hashes support artifact integrity verification and tamper detection."},
    {"id":"SSC-005","name":"No known high-risk finding","field":"known_high_risk_finding","expected":false,"why":"High-risk findings must be reviewed before release."}
  ]
}
'@

Write-NoBom ".\policy\secret_patterns.json" @'
{
  "project": "08-Secure-Software-Supply-Chain-Lab",
  "scope": "synthetic secret scanning only",
  "patterns": [
    {
      "id": "SEC-001",
      "name": "Generic hardcoded API key assignment",
      "regex": "(?i)(api_key|apikey|secret|token|password)\\s*=\\s*['\\\"]?[^'\\\"\\s]+",
      "severity": "medium",
      "notes": "MVP pattern for catching obvious hardcoded secret-like assignments in synthetic files."
    }
  ],
  "allowed_placeholders": ["REPLACE_ME", "example", "not-a-real-secret", "EXAMPLE_ONLY"]
}
'@

Write-NoBom ".\testdata\manifests\dependency_inventory.json" @'
[
  {
    "component": "trusted-json-parser",
    "ecosystem": "python",
    "version": "1.2.3",
    "version_pinned": true,
    "license": "MIT",
    "license_documented": true,
    "source": "pypi",
    "source_trusted": true,
    "integrity_hash_recorded": true,
    "known_high_risk_finding": false,
    "notes": "Synthetic good component for validation."
  },
  {
    "component": "trusted-http-client",
    "ecosystem": "go",
    "version": "0.9.1",
    "version_pinned": true,
    "license": "Apache-2.0",
    "license_documented": true,
    "source": "proxy.golang.org",
    "source_trusted": true,
    "integrity_hash_recorded": true,
    "known_high_risk_finding": false,
    "notes": "Synthetic good component for validation."
  },
  {
    "component": "intentionally-incomplete-demo-dependency",
    "ecosystem": "npm",
    "version": "latest",
    "version_pinned": false,
    "license": "",
    "license_documented": false,
    "source": "unknown",
    "source_trusted": false,
    "integrity_hash_recorded": false,
    "known_high_risk_finding": true,
    "notes": "Expected to fail validation. Used to prove the checker catches weak dependency posture."
  }
]
'@

Write-NoBom ".\testdata\configs\safe_config.example" @'
APP_ENV=development
API_KEY=REPLACE_ME
TOKEN=EXAMPLE_ONLY
PASSWORD=not-a-real-secret
'@

Write-NoBom ".\testdata\configs\bad_config.example" @'
APP_ENV=development
api_key=fake-hardcoded-demo-value
'@

Write-NoBom ".\sbom\sbom_example.cdx.json" @'
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "metadata": {
    "component": {
      "type": "application",
      "name": "MadHatter Project 08 Synthetic Supply Chain Lab",
      "version": "0.1.0"
    }
  },
  "components": [
    {"type":"library","name":"trusted-json-parser","version":"1.2.3","licenses":[{"license":{"id":"MIT"}}]},
    {"type":"library","name":"trusted-http-client","version":"0.9.1","licenses":[{"license":{"id":"Apache-2.0"}}]}
  ]
}
'@

Write-NoBom ".\pipelines\validate_supply_chain.py" @'
import json
import re
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, List

ROOT = Path(__file__).resolve().parents[1]
CONTROLS_PATH = ROOT / "policy" / "supply_chain_controls.json"
SECRET_PATTERNS_PATH = ROOT / "policy" / "secret_patterns.json"
DEPENDENCY_INVENTORY_PATH = ROOT / "testdata" / "manifests" / "dependency_inventory.json"
CONFIG_DIR = ROOT / "testdata" / "configs"
EVIDENCE_DIR = ROOT / "evidence" / "test_logs"

def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)

def evaluate_dependency(component: Dict[str, Any], controls: List[Dict[str, Any]]) -> Dict[str, Any]:
    control_results = []
    for control in controls:
        field = control["field"]
        expected = control["expected"]
        actual = component.get(field)
        passed = actual == expected
        control_results.append({
            "control_id": control["id"],
            "control_name": control["name"],
            "field": field,
            "expected": expected,
            "actual": actual,
            "passed": passed,
            "why": control["why"],
        })

    failed = [item for item in control_results if not item["passed"]]
    return {
        "component": component.get("component", "unknown-component"),
        "ecosystem": component.get("ecosystem", "unknown"),
        "version": component.get("version", ""),
        "passed": len(failed) == 0,
        "failed_controls": [item["control_id"] for item in failed],
        "control_results": control_results,
    }

def scan_file_for_secrets(path: Path, pattern_document: Dict[str, Any]) -> List[Dict[str, Any]]:
    text = path.read_text(encoding="utf-8")
    findings = []
    allowed_placeholders = [item.lower() for item in pattern_document.get("allowed_placeholders", [])]

    for pattern in pattern_document["patterns"]:
        compiled = re.compile(pattern["regex"])
        for line_number, line in enumerate(text.splitlines(), start=1):
            match = compiled.search(line)
            if not match:
                continue
            lowered_line = line.lower()
            if any(placeholder.lower() in lowered_line for placeholder in allowed_placeholders):
                continue
            findings.append({
                "file": str(path.relative_to(ROOT)),
                "line": line_number,
                "pattern_id": pattern["id"],
                "pattern_name": pattern["name"],
                "severity": pattern["severity"],
                "matched_preview": line.strip(),
                "notes": pattern["notes"],
            })
    return findings

def run_secret_scan() -> Dict[str, Any]:
    pattern_document = load_json(SECRET_PATTERNS_PATH)
    findings = []
    files = [path for path in sorted(CONFIG_DIR.glob("*")) if path.is_file()]
    for path in files:
        findings.extend(scan_file_for_secrets(path, pattern_document))
    return {
        "files_scanned": [str(path.relative_to(ROOT)) for path in files],
        "findings_total": len(findings),
        "findings": findings,
        "expected_demo_finding": any("bad_config.example" in item["file"] for item in findings),
    }

def main() -> int:
    controls_document = load_json(CONTROLS_PATH)
    controls = controls_document["required_controls"]
    dependencies = load_json(DEPENDENCY_INVENTORY_PATH)

    dependency_results = [evaluate_dependency(component, controls) for component in dependencies]
    secret_scan = run_secret_scan()

    expected_dependency_failure = any(
        item["component"] == "intentionally-incomplete-demo-dependency" and not item["passed"]
        for item in dependency_results
    )
    real_dependencies = [
        item for item in dependency_results
        if item["component"] != "intentionally-incomplete-demo-dependency"
    ]
    real_dependencies_passed = all(item["passed"] for item in real_dependencies)

    report = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "project": "08-Secure-Software-Supply-Chain-Lab",
        "scope": "synthetic supply-chain validation only",
        "summary": {
            "dependencies_total": len(dependency_results),
            "dependencies_passed": sum(1 for item in dependency_results if item["passed"]),
            "dependencies_failed": sum(1 for item in dependency_results if not item["passed"]),
            "controls_total": len(controls),
            "secret_findings_total": secret_scan["findings_total"],
        },
        "dependency_results": dependency_results,
        "secret_scan": secret_scan,
        "note": "The incomplete dependency and bad config are expected controlled failures. They prove the validator catches weak supply-chain posture and obvious secret-like assignments."
    }

    EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
    timestamp = datetime.now(timezone.utc).strftime("%Y%m%d-%H%M%S")
    output_path = EVIDENCE_DIR / f"supply_chain_validation_{timestamp}.json"
    with output_path.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)

    print(json.dumps(report, indent=2))
    print(f"\n[+] Validation report written to: {output_path}")

    return 0 if real_dependencies_passed and expected_dependency_failure and secret_scan["expected_demo_finding"] else 1

if __name__ == "__main__":
    raise SystemExit(main())
'@

Write-NoBom ".\scripts\validate.ps1" @'
python .\pipelines\validate_supply_chain.py
'@

Write-NoBom ".\scripts\capture_baseline.ps1" @'
$timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
New-Item -ItemType Directory -Force -Path ".\evidence\terminal_output" | Out-Null
New-Item -ItemType Directory -Force -Path ".\evidence\test_logs" | Out-Null

python --version | Tee-Object ".\evidence\terminal_output\python_version_$timestamp.txt"

python .\pipelines\validate_supply_chain.py |
    Tee-Object ".\evidence\test_logs\supply_chain_validation_console_$timestamp.txt"
'@

Write-NoBom ".\docs\project-charter.md" @'
# Project Charter

## Project
08-Secure-Software-Supply-Chain-Lab

## Purpose
Build a local synthetic software supply-chain security lab that validates dependency posture, secret scanning behavior, SBOM awareness, and release-readiness checks.

## Scope
The MVP uses synthetic dependency inventory and synthetic config files. It does not scan private repositories, publish packages, access real secrets, or deploy software.

## Non-Goals
- No real secret storage
- No private repo scanning without permission
- No package publishing
- No dependency installation from unknown sources
- No production release automation in MVP
- No destructive remediation

## Success Criteria
- Good dependencies pass baseline controls.
- Intentionally incomplete dependency fails as expected.
- Synthetic hardcoded secret-like config is detected.
- Placeholder config values are ignored.
- Evidence report is generated.
'@

Write-NoBom ".\docs\design.md" @'
# Design Notes

## MVP Architecture
dependency inventory + secret pattern policy + synthetic configs -> validation pipeline -> evidence report

## Current Design
The MVP uses Python to validate a synthetic dependency inventory and scan synthetic config files for obvious hardcoded secret-like assignments.

## Why Start Synthetic
Synthetic data makes it possible to prove pass/fail behavior before scanning real repositories.

## Connected Projects
- Project 01 TripplePulsarVault can later use release checks.
- Projects 02-07 can reuse dependency and secret hygiene checks.
- Project 15 can later track supply-chain alerts in query form.
'@

Write-NoBom ".\docs\validation-plan.md" @'
# Validation Plan

## Baseline Checks
- `python --version` works.
- `python .\pipelines\validate_supply_chain.py` runs.
- Good dependencies pass.
- Intentionally incomplete demo dependency fails.
- Synthetic bad config creates one secret-like finding.
- Evidence is written under `evidence/test_logs`.

## MVP Limitations
This MVP is not a replacement for real tools like Gitleaks, Syft, Grype, OSV-Scanner, Trivy, or GitHub Advanced Security. It is a local learning scaffold.
'@

Write-NoBom ".\docs\source-map.md" @'
# Source Map

Use official or primary sources first.

## Core References
- SLSA Framework: https://slsa.dev/
- OpenSSF Scorecard: https://github.com/ossf/scorecard
- CycloneDX: https://cyclonedx.org/
- SPDX: https://spdx.dev/
- NIST SSDF SP 800-218: https://csrc.nist.gov/publications/detail/sp/800-218/final
- GitHub Secret Scanning Documentation: https://docs.github.com/code-security/secret-scanning
- Gitleaks Documentation: https://github.com/gitleaks/gitleaks
- Syft Documentation: https://github.com/anchore/syft
- Grype Documentation: https://github.com/anchore/grype
- OSV-Scanner Documentation: https://google.github.io/osv-scanner/
- Python Documentation: https://docs.python.org/3/

## Source Rules
- Prefer official docs and standards.
- Record date accessed.
- Do not paste real secrets into test files.
- Separate synthetic findings from real vulnerability claims.
'@

Write-NoBom ".\docs\development.md" @'
# Development Guide

## Local Development

Check Python:

    python --version

Run validation:

    python .\pipelines\validate_supply_chain.py

Run via PowerShell helper:

    Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
    .\scripts\validate.ps1

Capture baseline evidence:

    .\scripts\capture_baseline.ps1

## Development Rules
- Use synthetic data first.
- Do not store real secrets.
- Do not scan private repositories without permission.
- Do not claim real vulnerability findings from synthetic tests.
- Document controlled failures clearly.
'@

Write-NoBom ".\README.md" @'
# 08-Secure-Software-Supply-Chain-Lab

## Project Status
Development-ready MVP scaffold.

## Executive Summary
The Secure Software Supply Chain Lab is a local synthetic validation environment for practicing dependency hygiene, secret scanning, SBOM awareness, and release-readiness checks.

## Current MVP
- Synthetic dependency inventory
- Supply-chain control definitions
- Synthetic secret scan patterns
- Safe and bad config examples
- CycloneDX-style SBOM example
- Python validation pipeline
- Evidence capture
- Documentation set

## Validate
Run:

    python .\pipelines\validate_supply_chain.py

Expected behavior:
- Two good dependencies pass.
- One intentionally incomplete demo dependency fails.
- One synthetic bad config finding is detected.
- Script exits successfully because controlled failures are expected.

## Safety Scope
No real secrets. No private repo scanning without permission. No package publishing. No production release automation in MVP.
'@

Write-NoBom ".\CHANGELOG.md" @'
# Changelog

## v0.1.0 - Initial MVP Scaffold

### Added
- Synthetic dependency inventory
- Supply-chain controls
- Synthetic secret scan policy
- Safe and bad config examples
- CycloneDX-style SBOM example
- Python validation pipeline
- Evidence capture scripts
- Documentation set

### Safety Scope
- Synthetic validation only
- No real secrets
- No private repo scanning
- No production release automation
'@

Write-Host "[+] Project 08 scaffold complete."
Write-Host "[+] Run these next:"
Write-Host "    python --version"
Write-Host "    python .\pipelines\validate_supply_chain.py"
