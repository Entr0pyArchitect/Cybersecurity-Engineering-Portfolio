import unittest
from pathlib import Path

from agent_core.guardrails import evaluate_request, load_request


ROOT = Path(__file__).resolve().parents[1]


class GuardrailTests(unittest.TestCase):
    def test_safe_lab_request_is_approved(self):
        request = load_request(ROOT / "test-scenarios" / "safe_lab_request.json")
        decision = evaluate_request(request)

        self.assertEqual(decision.status, "approved_lab_safe")
        self.assertIn("explain concepts", decision.allowed_actions)

    def test_unsafe_request_is_blocked(self):
        request = load_request(ROOT / "test-scenarios" / "unsafe_request.json")
        decision = evaluate_request(request)

        self.assertEqual(decision.status, "blocked")
        self.assertEqual(decision.risk_level, "high")
        self.assertTrue(decision.blocked_actions)

    def test_unclear_scope_needs_review(self):
        request = load_request(ROOT / "test-scenarios" / "needs_review_request.json")
        decision = evaluate_request(request)

        self.assertEqual(decision.status, "needs_review")
        self.assertIn("scope clarification", decision.allowed_actions)


if __name__ == "__main__":
    unittest.main()