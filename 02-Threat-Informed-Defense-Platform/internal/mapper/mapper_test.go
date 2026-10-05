package mapper

import "testing"

func TestNameForTechniqueKnown(t *testing.T) {
	got := NameForTechnique("T1059")
	want := "Command and Scripting Interpreter"

	if got != want {
		t.Fatalf("expected %q, got %q", want, got)
	}
}

func TestNameForTechniqueUnknown(t *testing.T) {
	got := NameForTechnique("T9999")
	want := "Unknown / unmapped ATT&CK technique"

	if got != want {
		t.Fatalf("expected %q, got %q", want, got)
	}
}

func TestPriorityForConfidence(t *testing.T) {
	cases := map[string]string{
		"high":    "high",
		"medium":  "medium",
		"low":     "low",
		"unknown": "review",
		"":        "review",
	}

	for input, want := range cases {
		got := PriorityForConfidence(input)
		if got != want {
			t.Fatalf("for input %q expected %q, got %q", input, want, got)
		}
	}
}
