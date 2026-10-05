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