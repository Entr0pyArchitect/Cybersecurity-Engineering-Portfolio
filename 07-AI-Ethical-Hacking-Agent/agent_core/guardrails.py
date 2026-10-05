from __future__ import annotations

import json
from pathlib import Path
from typing import Dict, List

from agent_core.models import AgentRequest, GuardrailDecision


ROOT = Path(__file__).resolve().parents[1]
POLICY_PATH = ROOT / "guardrails" / "safety_policy.json"


def load_policy(path: Path = POLICY_PATH) -> Dict[str, object]:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def load_request(path: Path) -> AgentRequest:
    with path.open("r", encoding="utf-8") as handle:
        raw = json.load(handle)

    return AgentRequest(
        request_id=raw.get("request_id", "unknown-request"),
        user_goal=raw.get("user_goal", ""),
        environment=raw.get("environment", ""),
        authorization=raw.get("authorization", ""),
        target_type=raw.get("target_type", ""),
        requested_actions=raw.get("requested_actions", []),
        notes=raw.get("notes", ""),
    )


def evaluate_request(request: AgentRequest, policy: Dict[str, object] | None = None) -> GuardrailDecision:
    policy = policy or load_policy()

    text_blob = " ".join([
        request.user_goal,
        request.environment,
        request.authorization,
        request.target_type,
        " ".join(request.requested_actions),
        request.notes,
    ]).lower()

    denied_keywords = [item.lower() for item in policy["denied_keywords"]]
    caution_keywords = [item.lower() for item in policy["caution_keywords"]]
    allowed_environments = [item.lower() for item in policy["allowed_environments"]]

    blocked_hits = sorted({keyword for keyword in denied_keywords if keyword in text_blob})
    caution_hits = sorted({keyword for keyword in caution_keywords if keyword in text_blob})

    authorized = request.authorization.strip().lower() in {
        "owned lab",
        "ctf",
        "htb academy",
        "intentionally vulnerable lab",
        "written authorization",
        "mock data",
        "local-only"
    }

    environment_allowed = any(item in request.environment.lower() for item in allowed_environments)

    if blocked_hits:
        return GuardrailDecision(
            request_id=request.request_id,
            status="blocked",
            reason="Request contains actions or goals outside the safety scope.",
            risk_level="high",
            blocked_actions=blocked_hits,
            required_human_checks=[
                "Remove any request for credential theft, malware, persistence, evasion, phishing, or unauthorized access.",
                "Restate the goal as defensive learning, documentation, detection, or lab-safe validation.",
            ],
            safe_next_steps=[
                "Use the request to write a defensive threat model instead.",
                "Create a detection or incident-response learning note.",
                "Practice only in CTF, HTB Academy, or an owned intentionally vulnerable lab.",
            ],
            project_connections=[
                "04-Detection-Engineering-Lab",
                "06-Autonomous-Response-Module",
                "14-DFIR-Evidence-Automation-Toolkit"
            ],
            evidence_notes=[
                "Blocked decision should be logged as proof that safety guardrails work."
            ],
        )

    if not authorized or not environment_allowed:
        return GuardrailDecision(
            request_id=request.request_id,
            status="needs_review",
            reason="Authorization or environment is unclear. Human review required before technical guidance.",
            risk_level="medium",
            allowed_actions=["scope clarification", "documentation", "risk review"],
            required_human_checks=[
                "Confirm the environment is owned, intentionally vulnerable, CTF, HTB Academy, mock, local-only, or written-authorized.",
                "Define exact boundaries before any technical work.",
                "Document what is in scope and out of scope.",
            ],
            safe_next_steps=[
                "Write a project charter.",
                "Write a rules-of-engagement statement.",
                "Create a lab-only learning plan.",
            ],
            project_connections=[
                "05-Zero-Trust-Micro-Infrastructure",
                "04-Detection-Engineering-Lab"
            ],
            evidence_notes=[
                "Save scope notes before proceeding."
            ],
        )

    status = "approved_lab_safe"
    risk_level = "low" if not caution_hits else "medium"

    return GuardrailDecision(
        request_id=request.request_id,
        status=status,
        reason="Request is bounded to authorized lab-safe learning or defensive work.",
        risk_level=risk_level,
        allowed_actions=[
            "explain concepts",
            "create defensive checklist",
            "create documentation outline",
            "map learning to MadHatter projects",
            "summarize evidence",
            "suggest lab-safe validation steps"
        ],
        blocked_actions=[],
        required_human_checks=[
            "Confirm no real third-party systems are involved.",
            "Confirm no secrets, credentials, private data, or production systems are used.",
            "Confirm output remains defensive and documentation-oriented.",
        ],
        safe_next_steps=[
            "Create a note using the Note Taking Template.",
            "Define expected evidence before testing.",
            "Run only safe local or platform-provided labs.",
            "Document results in the project evidence folder.",
        ],
        project_connections=[
            "02-Threat-Informed-Defense-Platform",
            "03-Telemetry-Sensor-Framework",
            "04-Detection-Engineering-Lab",
            "06-Autonomous-Response-Module",
            "17-AI-Enhanced-SOC-Copilot-RAG-Knowledge-Base"
        ],
        evidence_notes=[
            "Save the guardrail decision JSON.",
            "Save source notes and lab boundaries.",
            "Record whether AI was used only for organization, not solving."
        ],
    )