# Design Notes

## MVP Architecture

request JSON -> guardrail evaluator -> decision JSON -> evidence

## Current Design
The MVP uses a deterministic Python guardrail engine instead of an external LLM. This makes the safety behavior testable and explainable.

## Why Start Without an LLM
The project should prove safety boundaries before adding model integration. A future LLM gateway can be added only after the guardrail layer, logging, and human-approval workflow are mature.

## Connected Projects
- Project 02: threat-informed defensive planning
- Project 04: detection validation
- Project 06: response planning safety
- Project 17: later RAG knowledge base