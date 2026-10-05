package engine

import (
	"fmt"
	"strings"

	"autonomous-response-module/internal/model"
)

func Decide(alert model.Alert, playbooks []model.Playbook) model.ResponseDecision {
	if strings.TrimSpace(alert.AlertID) == "" {
		return model.ResponseDecision{
			Status:             "rejected",
			SelectedPlaybook:   "none",
			Reason:             "alert_id is required",
			ApprovedForAutoRun: false,
			SafetySummary:      defaultSafetySummary(),
		}
	}

	for _, playbook := range playbooks {
		if playbookMatches(alert, playbook) {
			return model.ResponseDecision{
				AlertID:            alert.AlertID,
				Status:             "planned",
				SelectedPlaybook:   playbook.PlaybookID,
				Reason:             fmt.Sprintf("alert technique %s and severity %s matched playbook %s", alert.TechniqueID, alert.Severity, playbook.Name),
				ApprovedForAutoRun: false,
				Actions:            forceSimulationMode(playbook.Actions),
				SafetySummary:      append(defaultSafetySummary(), playbook.SafetyNotes...),
			}
		}
	}

	return model.ResponseDecision{
		AlertID:            alert.AlertID,
		Status:             "no_playbook_matched",
		SelectedPlaybook:   "none",
		Reason:             "no matching lab-safe playbook found",
		ApprovedForAutoRun: false,
		Actions:            []model.ResponseAction{},
		SafetySummary:      defaultSafetySummary(),
	}
}

func playbookMatches(alert model.Alert, playbook model.Playbook) bool {
	if severityRank(alert.Severity) < severityRank(playbook.MinSeverity) {
		return false
	}

	for _, technique := range playbook.TriggerTechniqueIDs {
		if strings.EqualFold(strings.TrimSpace(technique), strings.TrimSpace(alert.TechniqueID)) {
			return true
		}
	}

	return false
}

func severityRank(severity string) int {
	switch strings.ToLower(strings.TrimSpace(severity)) {
	case "critical":
		return 4
	case "high":
		return 3
	case "medium":
		return 2
	case "low":
		return 1
	default:
		return 0
	}
}

func forceSimulationMode(actions []model.ResponseAction) []model.ResponseAction {
	safeActions := make([]model.ResponseAction, 0, len(actions))

	for _, action := range actions {
		action.Mode = "simulation_only"
		action.Allowed = true
		safeActions = append(safeActions, action)
	}

	return safeActions
}

func defaultSafetySummary() []string {
	return []string{
		"MVP is simulation-only.",
		"No real host isolation is performed.",
		"No credentials are collected.",
		"No destructive actions are executed.",
		"Human approval is required before any real-world response action.",
	}
}
