# Development Guide

## Local Development

Run all tests:

    cargo test

Run the Windows agent:

    cargo run -p windows-agent

Run the Linux agent:

    cargo run -p linux-agent

Capture baseline evidence:

    Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
    .\scripts\capture_baseline.ps1

## Development Rules

- Keep the MVP userland-only.
- Do not collect secrets.
- Do not monitor systems without permission.
- Do not add persistence.
- Do not add stealth behavior.
- Focus on schema quality, event normalization, and validation first.