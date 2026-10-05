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