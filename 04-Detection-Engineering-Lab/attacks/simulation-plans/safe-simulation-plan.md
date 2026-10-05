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