from dataclasses import dataclass, field
from typing import Dict, List


@dataclass
class AgentRequest:
    request_id: str
    user_goal: str
    environment: str
    authorization: str
    target_type: str
    requested_actions: List[str]
    notes: str = ""


@dataclass
class GuardrailDecision:
    request_id: str
    status: str
    reason: str
    risk_level: str
    allowed_actions: List[str] = field(default_factory=list)
    blocked_actions: List[str] = field(default_factory=list)
    required_human_checks: List[str] = field(default_factory=list)
    safe_next_steps: List[str] = field(default_factory=list)
    project_connections: List[str] = field(default_factory=list)
    evidence_notes: List[str] = field(default_factory=list)

    def to_dict(self) -> Dict[str, object]:
        return {
            "request_id": self.request_id,
            "status": self.status,
            "reason": self.reason,
            "risk_level": self.risk_level,
            "allowed_actions": self.allowed_actions,
            "blocked_actions": self.blocked_actions,
            "required_human_checks": self.required_human_checks,
            "safe_next_steps": self.safe_next_steps,
            "project_connections": self.project_connections,
            "evidence_notes": self.evidence_notes,
        }