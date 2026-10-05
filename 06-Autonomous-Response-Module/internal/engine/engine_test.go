package engine

import (
	"testing"

	"autonomous-response-module/internal/model"
)

func TestDecideSelectsMatchingPlaybook(t *testing.T) {
	alert := model.Alert{
		AlertID:     "ALERT-001",
		Severity:    "high",
		TechniqueID: "T1059",
	}

	playbooks := []model.Playbook{
		{
			PlaybookID:          "PB-001",
			Name:                "Script Interpreter Investigation",
			TriggerTechniqueIDs: []string{"T1059"},
			MinSeverity:         "medium",
			Actions: []model.ResponseAction{
				{ActionID: "ACT-001", Name: "Collect process evidence"},
			},
		},
	}

	decision := Decide(alert, playbooks)

	if decision.Status != "planned" {
		t.Fatalf("expected planned status, got %s", decision.Status)
	}

	if decision.SelectedPlaybook != "PB-001" {
		t.Fatalf("expected PB-001, got %s", decision.SelectedPlaybook)
	}

	if decision.ApprovedForAutoRun {
		t.Fatalf("expected ApprovedForAutoRun to remain false")
	}

	if len(decision.Actions) != 1 {
		t.Fatalf("expected one action, got %d", len(decision.Actions))
	}

	if decision.Actions[0].Mode != "simulation_only" {
		t.Fatalf("expected action mode simulation_only, got %s", decision.Actions[0].Mode)
	}
}

func TestDecideRejectsMissingAlertID(t *testing.T) {
	alert := model.Alert{
		Severity:    "high",
		TechniqueID: "T1059",
	}

	decision := Decide(alert, []model.Playbook{})

	if decision.Status != "rejected" {
		t.Fatalf("expected rejected status, got %s", decision.Status)
	}
}

func TestDecideNoMatch(t *testing.T) {
	alert := model.Alert{
		AlertID:     "ALERT-002",
		Severity:    "low",
		TechniqueID: "T9999",
	}

	playbooks := []model.Playbook{
		{
			PlaybookID:          "PB-001",
			Name:                "Script Interpreter Investigation",
			TriggerTechniqueIDs: []string{"T1059"},
			MinSeverity:         "medium",
		},
	}

	decision := Decide(alert, playbooks)

	if decision.Status != "no_playbook_matched" {
		t.Fatalf("expected no_playbook_matched, got %s", decision.Status)
	}
}
