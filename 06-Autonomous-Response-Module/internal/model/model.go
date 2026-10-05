package model

type Alert struct {
	AlertID     string            `json:"alert_id"`
	Title       string            `json:"title"`
	Source      string            `json:"source"`
	Severity    string            `json:"severity"`
	Host        string            `json:"host"`
	User        string            `json:"user"`
	TechniqueID string            `json:"technique_id"`
	Signals     []string          `json:"signals"`
	Labels      map[string]string `json:"labels"`
}

type ResponseAction struct {
	ActionID    string `json:"action_id"`
	Name        string `json:"name"`
	Description string `json:"description"`
	Mode        string `json:"mode"`
	Allowed     bool   `json:"allowed"`
}

type Playbook struct {
	PlaybookID          string           `json:"playbook_id"`
	Name                string           `json:"name"`
	Description         string           `json:"description"`
	TriggerTechniqueIDs []string         `json:"trigger_technique_ids"`
	MinSeverity         string           `json:"min_severity"`
	Actions             []ResponseAction `json:"actions"`
	SafetyNotes         []string         `json:"safety_notes"`
}

type ResponseDecision struct {
	AlertID            string           `json:"alert_id"`
	Status             string           `json:"status"`
	SelectedPlaybook   string           `json:"selected_playbook"`
	Reason             string           `json:"reason"`
	ApprovedForAutoRun bool             `json:"approved_for_auto_run"`
	Actions            []ResponseAction `json:"actions"`
	SafetySummary      []string         `json:"safety_summary"`
}
