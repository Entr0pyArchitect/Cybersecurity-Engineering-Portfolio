# Project 05 - Zero-Trust Micro-Infrastructure Bootstrap
# Run from:
# Cybersecurity-Engineering-Portfolio\05-Zero-Trust-Micro-Infrastructure

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

Write-Host "[+] Bootstrapping Project 05 - Zero-Trust Micro-Infrastructure"

New-Item -ItemType Directory -Force -Path ".\.backup" | Out-Null

if (Test-Path ".\README.md") {
    Copy-Item ".\README.md" ".\.backup\README.md.bak-$timestamp" -Force
}

$folders = @(
    "docs",
    "kubernetes\manifests",
    "kubernetes\network-policies",
    "observability\prometheus",
    "observability\grafana",
    "policy\opa",
    "policy\controls",
    "pipelines",
    "scripts",
    "terraform\modules",
    "testdata",
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
crash.log
crash.*.log

# Kubernetes local outputs
kubeconfig*
*.kubeconfig

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

Write-NoBom ".\testdata\service_inventory.json" @'
[
  {
    "service_name": "threat-informed-defense-platform",
    "project": "02-Threat-Informed-Defense-Platform",
    "environment": "local-lab",
    "service_identity": "spiffe://madhatter.local/project02/platform",
    "mtls_required": true,
    "auth_required": true,
    "network_policy_required": true,
    "least_privilege": true,
    "secrets_externalized": true,
    "observability_enabled": true,
    "public_exposure": false,
    "notes": "Defensive API service. Local-lab only."
  },
  {
    "service_name": "telemetry-sensor-framework",
    "project": "03-Telemetry-Sensor-Framework",
    "environment": "local-lab",
    "service_identity": "spiffe://madhatter.local/project03/sensor",
    "mtls_required": true,
    "auth_required": true,
    "network_policy_required": true,
    "least_privilege": true,
    "secrets_externalized": true,
    "observability_enabled": true,
    "public_exposure": false,
    "notes": "Future telemetry producer. No public exposure."
  },
  {
    "service_name": "intentionally-incomplete-demo-service",
    "project": "05-Zero-Trust-Micro-Infrastructure",
    "environment": "local-lab",
    "service_identity": "",
    "mtls_required": false,
    "auth_required": true,
    "network_policy_required": false,
    "least_privilege": false,
    "secrets_externalized": true,
    "observability_enabled": false,
    "public_exposure": false,
    "notes": "Expected to fail validation. Used to prove the policy checker catches weak service definitions."
  }
]
'@

Write-NoBom ".\policy\controls\zero_trust_controls.json" @'
{
  "project": "05-Zero-Trust-Micro-Infrastructure",
  "scope": "local-lab policy validation only",
  "required_controls": [
    {
      "id": "ZT-001",
      "name": "Service identity required",
      "field": "service_identity",
      "expected": "non_empty",
      "why": "Every service must have an identity before trust can be evaluated."
    },
    {
      "id": "ZT-002",
      "name": "mTLS required",
      "field": "mtls_required",
      "expected": true,
      "why": "Service-to-service communication should be encrypted and identity-aware."
    },
    {
      "id": "ZT-003",
      "name": "Authentication required",
      "field": "auth_required",
      "expected": true,
      "why": "No service should assume internal network trust."
    },
    {
      "id": "ZT-004",
      "name": "Network policy required",
      "field": "network_policy_required",
      "expected": true,
      "why": "East-west traffic should be explicitly constrained."
    },
    {
      "id": "ZT-005",
      "name": "Least privilege required",
      "field": "least_privilege",
      "expected": true,
      "why": "Services should only have the access needed for their documented purpose."
    },
    {
      "id": "ZT-006",
      "name": "Secrets externalized",
      "field": "secrets_externalized",
      "expected": true,
      "why": "Secrets must not be hardcoded in application or infrastructure files."
    },
    {
      "id": "ZT-007",
      "name": "Observability enabled",
      "field": "observability_enabled",
      "expected": true,
      "why": "A security system must be measurable and auditable."
    },
    {
      "id": "ZT-008",
      "name": "No public exposure by default",
      "field": "public_exposure",
      "expected": false,
      "why": "The MVP lab should remain private and local unless exposure is explicitly justified."
    }
  ]
}
'@

Write-NoBom ".\pipelines\validate_zero_trust.py" @'
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, List


ROOT = Path(__file__).resolve().parents[1]
SERVICES_PATH = ROOT / "testdata" / "service_inventory.json"
CONTROLS_PATH = ROOT / "policy" / "controls" / "zero_trust_controls.json"
EVIDENCE_DIR = ROOT / "evidence" / "test_logs"


def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def evaluate_control(service: Dict[str, Any], control: Dict[str, Any]) -> Dict[str, Any]:
    field = control["field"]
    expected = control["expected"]
    actual = service.get(field)

    if expected == "non_empty":
        passed = isinstance(actual, str) and actual.strip() != ""
    else:
        passed = actual == expected

    return {
        "control_id": control["id"],
        "control_name": control["name"],
        "field": field,
        "expected": expected,
        "actual": actual,
        "passed": passed,
        "why": control["why"],
    }


def evaluate_service(service: Dict[str, Any], controls: List[Dict[str, Any]]) -> Dict[str, Any]:
    results = [evaluate_control(service, control) for control in controls]
    failed = [item for item in results if not item["passed"]]

    return {
        "service_name": service.get("service_name", "unknown-service"),
        "project": service.get("project", "unknown-project"),
        "environment": service.get("environment", "unknown-environment"),
        "public_exposure": service.get("public_exposure"),
        "passed": len(failed) == 0,
        "failed_controls": [item["control_id"] for item in failed],
        "control_results": results,
    }


def main() -> int:
    services = load_json(SERVICES_PATH)
    controls_document = load_json(CONTROLS_PATH)
    controls = controls_document["required_controls"]

    service_results = [evaluate_service(service, controls) for service in services]

    report = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "project": "05-Zero-Trust-Micro-Infrastructure",
        "scope": "local-lab policy validation only",
        "summary": {
            "services_total": len(service_results),
            "services_passed": sum(1 for item in service_results if item["passed"]),
            "services_failed": sum(1 for item in service_results if not item["passed"]),
            "controls_total": len(controls),
        },
        "results": service_results,
        "note": "The intentionally-incomplete-demo-service is expected to fail. It proves the validator catches missing zero-trust controls."
    }

    EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
    timestamp = datetime.now(timezone.utc).strftime("%Y%m%d-%H%M%S")
    output_path = EVIDENCE_DIR / f"zero_trust_validation_{timestamp}.json"

    with output_path.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)

    print(json.dumps(report, indent=2))
    print(f"\n[+] Validation report written to: {output_path}")

    # Expected MVP behavior: at least one pass and at least one controlled failure.
    # This proves both positive and negative validation paths work.
    expected_demo_failure = any(
        item["service_name"] == "intentionally-incomplete-demo-service" and not item["passed"]
        for item in service_results
    )

    real_services = [
        item for item in service_results
        if item["service_name"] != "intentionally-incomplete-demo-service"
    ]

    real_services_passed = all(item["passed"] for item in real_services)

    return 0 if expected_demo_failure and real_services_passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
'@

Write-NoBom ".\scripts\validate.ps1" @'
python .\pipelines\validate_zero_trust.py
'@

Write-NoBom ".\scripts\capture_baseline.ps1" @'
$timestamp = Get-Date -Format "yyyyMMdd-HHmmss"

New-Item -ItemType Directory -Force -Path ".\evidence\terminal_output" | Out-Null
New-Item -ItemType Directory -Force -Path ".\evidence\test_logs" | Out-Null

python --version | Tee-Object ".\evidence\terminal_output\python_version_$timestamp.txt"

python .\pipelines\validate_zero_trust.py |
    Tee-Object ".\evidence\test_logs\zero_trust_validation_console_$timestamp.txt"
'@

Write-NoBom ".\policy\opa\zero_trust_policy.rego" @'
package madhatter.zerotrust

# Placeholder OPA/Rego policy for future enforcement.
# The current MVP validates controls with Python first.
# Later, this can be translated into executable OPA policy tests.

default allow = false

allow {
    input.service_identity != ""
    input.mtls_required == true
    input.auth_required == true
    input.network_policy_required == true
    input.least_privilege == true
    input.secrets_externalized == true
    input.observability_enabled == true
    input.public_exposure == false
}
'@

Write-NoBom ".\kubernetes\manifests\project02-platform-deployment.yaml" @'
apiVersion: apps/v1
kind: Deployment
metadata:
  name: threat-informed-defense-platform
  labels:
    app: threat-informed-defense-platform
    project: "02"
spec:
  replicas: 1
  selector:
    matchLabels:
      app: threat-informed-defense-platform
  template:
    metadata:
      labels:
        app: threat-informed-defense-platform
        project: "02"
    spec:
      containers:
        - name: platform
          image: threat-informed-defense-platform:local
          imagePullPolicy: Never
          ports:
            - containerPort: 8080
          env:
            - name: LAB_SCOPE
              value: "mock-data-only"
'@

Write-NoBom ".\kubernetes\network-policies\default-deny-placeholder.yaml" @'
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: default-deny-placeholder
spec:
  podSelector: {}
  policyTypes:
    - Ingress
    - Egress
# Placeholder only. Do not apply blindly.
# Future work: define explicit service-to-service allow rules in a local lab.
'@

Write-NoBom ".\observability\prometheus\prometheus.yml" @'
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: "madhatter-local-placeholder"
    static_configs:
      - targets: ["localhost:9090"]

# Placeholder for future local observability.
# Do not expose publicly.
'@

Write-NoBom ".\observability\grafana\README.md" @'
# Grafana Notes

This folder is reserved for future local dashboards.

Current MVP does not deploy Grafana. The first dashboard goals should be:

- Service health
- Validation results
- Policy pass/fail counts
- Local-only lab visibility
'@

Write-NoBom ".\terraform\main.tf" @'
# Project 05 Terraform Placeholder
# Current MVP does not deploy cloud resources.
# Keep this local until cost, identity, and exposure boundaries are documented.

terraform {
  required_version = ">= 1.6.0"
}
'@

Write-NoBom ".\terraform\README.md" @'
# Terraform Notes

Terraform is reserved for future reproducible lab infrastructure.

Current MVP:
- No cloud deployment.
- No public exposure.
- No active infrastructure provisioning.

Future goals:
- Local lab definitions.
- Reproducible Kubernetes/kind baseline.
- Explicit identity and network boundaries.
- Cost-controlled cloud experiments only after written scope.
'@

Write-NoBom ".\docs\project-charter.md" @'
# Project Charter

## Project
05-Zero-Trust-Micro-Infrastructure

## Purpose
Build a local-first zero-trust infrastructure proof that validates service identity, mTLS expectations, authentication requirements, network policy, least privilege, secrets handling, observability, and no-public-exposure defaults.

## Scope
The MVP validates service definitions and zero-trust controls using local JSON data and a Python validation pipeline. It does not deploy cloud infrastructure or expose services publicly.

## Non-Goals
- No public cloud deployment in MVP
- No public internet exposure
- No production Kubernetes deployment
- No real secrets
- No bypassing authentication systems
- No scanning third-party infrastructure

## Success Criteria
- Service inventory exists.
- Zero-trust controls exist.
- Validation pipeline runs.
- Real services pass baseline controls.
- Intentionally incomplete demo service fails as expected.
- Evidence report is generated.
'@

Write-NoBom ".\docs\design.md" @'
# Design Notes

## MVP Architecture

service inventory -> zero-trust controls -> validation pipeline -> evidence report

## Current Design
The MVP is a local control-validation system. Instead of deploying infrastructure immediately, it first defines what a zero-trust service should declare and validates those declarations.

## Why Start With Control Validation
Zero trust is not a product. It is a design model. Before deploying Kubernetes, OPA, mTLS, or cloud infrastructure, this project starts by proving that services can be evaluated against clear identity, access, network, secret, and observability requirements.

## Connected Projects
- Project 02 can become a service inside this infrastructure.
- Project 03 can become a telemetry producer.
- Project 04 can validate detections for infrastructure behavior.
- Project 06 may later consume alerts for response.
'@

Write-NoBom ".\docs\validation-plan.md" @'
# Validation Plan

## Baseline Checks
- `python --version` works.
- `python .\pipelines\validate_zero_trust.py` runs.
- Project 02 and Project 03 sample services pass.
- The intentionally incomplete demo service fails.
- Evidence is written under `evidence/test_logs`.

## Evidence to Capture
- Python version output.
- Validation console output.
- JSON validation report.
- Screenshot of successful validation.
- Notes explaining which controls passed and failed.

## MVP Limitations
This MVP validates control declarations. It does not yet enforce Kubernetes policy, provision infrastructure, generate certificates, deploy SPIFFE/SPIRE, or run OPA in a cluster.
'@

Write-NoBom ".\docs\source-map.md" @'
# Source Map

Use official or primary sources first.

## Core References
- NIST Zero Trust Architecture SP 800-207: https://csrc.nist.gov/publications/detail/sp/800-207/final
- CISA Zero Trust Maturity Model: https://www.cisa.gov/zero-trust-maturity-model
- Kubernetes Documentation: https://kubernetes.io/docs/
- Kubernetes Network Policies: https://kubernetes.io/docs/concepts/services-networking/network-policies/
- Open Policy Agent Documentation: https://www.openpolicyagent.org/docs/latest/
- SPIFFE/SPIRE Documentation: https://spiffe.io/docs/latest/
- Prometheus Documentation: https://prometheus.io/docs/
- Terraform Documentation: https://developer.hashicorp.com/terraform/docs

## Source Rules
- Prefer official docs and standards.
- Record date accessed in notes.
- Do not deploy public infrastructure without a written scope.
- Separate zero-trust principles from specific vendor tools.
'@

Write-NoBom ".\docs\development.md" @'
# Development Guide

## Local Development

Check Python:

    python --version

Run validation:

    python .\pipelines\validate_zero_trust.py

Run via PowerShell helper:

    Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
    .\scripts\validate.ps1

Capture baseline evidence:

    .\scripts\capture_baseline.ps1

## Development Rules
- Keep MVP local-only.
- Do not use real secrets.
- Do not deploy cloud resources yet.
- Do not expose lab services publicly.
- Validate control logic before infrastructure deployment.
'@

Write-NoBom ".\README.md" @'
# 05-Zero-Trust-Micro-Infrastructure

## Project Status
Development-ready MVP scaffold.

## Executive Summary
The Zero-Trust Micro-Infrastructure project is a local-first infrastructure security proof. The MVP validates whether service definitions satisfy zero-trust control expectations such as identity, mTLS, authentication, network policy, least privilege, secrets handling, observability, and no public exposure by default.

## Current MVP
- Service inventory
- Zero-trust control definitions
- Python validation pipeline
- OPA/Rego placeholder
- Kubernetes placeholder manifests
- Network policy placeholder
- Observability placeholders
- Terraform placeholder
- Evidence capture
- Documentation set

## Validate
Run:

    python .\pipelines\validate_zero_trust.py

Expected behavior:
- Project 02 sample service passes.
- Project 03 sample service passes.
- Intentionally incomplete demo service fails.
- Script exits successfully because the controlled failure is expected.

## Safety Scope
This MVP does not deploy cloud resources, expose public services, use real secrets, or enforce live cluster policies. It validates local control declarations only.
'@

Write-NoBom ".\CHANGELOG.md" @'
# Changelog

## v0.1.0 - Initial MVP Scaffold

### Added
- Service inventory
- Zero-trust control definition file
- Python control validation pipeline
- OPA/Rego placeholder policy
- Kubernetes placeholder manifests
- NetworkPolicy placeholder
- Prometheus/Grafana placeholders
- Terraform placeholder
- Documentation set
- Evidence capture scripts

### Safety Scope
- Local validation only
- No cloud deployment in MVP
- No public exposure
- No real secrets
'@

Write-Host "[+] Project 05 scaffold complete."
Write-Host "[+] Run these next:"
Write-Host "    python --version"
Write-Host "    python .\pipelines\validate_zero_trust.py"
