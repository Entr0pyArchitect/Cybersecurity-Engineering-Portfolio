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