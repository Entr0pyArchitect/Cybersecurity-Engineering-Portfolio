package model

type Technique struct {
	ID         string `json:"id"`
	Name       string `json:"name,omitempty"`
	Tactic     string `json:"tactic,omitempty"`
	Confidence string `json:"confidence,omitempty"`
	Notes      string `json:"notes,omitempty"`
}

type IngestRequest struct {
	SourceName string      `json:"source_name"`
	Summary    string      `json:"summary"`
	Techniques []Technique `json:"techniques"`
}

type BacklogItem struct {
	ID                string   `json:"id"`
	TechniqueID       string   `json:"technique_id"`
	TechniqueName     string   `json:"technique_name"`
	Tactic            string   `json:"tactic"`
	Priority          string   `json:"priority"`
	DetectionQuestion string   `json:"detection_question"`
	RequiredTelemetry []string `json:"required_telemetry"`
	SourceName        string   `json:"source_name"`
	Notes             string   `json:"notes"`
}

type SigmaCandidate struct {
	Title              string   `json:"title"`
	Status             string   `json:"status"`
	TechniqueID        string   `json:"technique_id"`
	LogSources         []string `json:"log_sources"`
	DetectionIdea      string   `json:"detection_idea"`
	FalsePositiveNotes string   `json:"false_positive_notes"`
}

type IngestResponse struct {
	Status          string           `json:"status"`
	SourceName      string           `json:"source_name"`
	MappedCount     int              `json:"mapped_count"`
	BacklogItems    []BacklogItem    `json:"backlog_items"`
	SigmaCandidates []SigmaCandidate `json:"sigma_candidates"`
	GeneratedAt     string           `json:"generated_at"`
}
