package server

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"strings"
	"time"

	"threat-informed-defense-platform/internal/mapper"
	"threat-informed-defense-platform/internal/model"
)

type Server struct{}

func New() *Server {
	return &Server{}
}

func (s *Server) Router() http.Handler {
	mux := http.NewServeMux()

	mux.HandleFunc("/health", s.health)
	mux.HandleFunc("/api/v1/techniques", s.techniques)
	mux.HandleFunc("/api/v1/ingest", s.ingest)
	mux.HandleFunc("/", s.notFound)

	return logMiddleware(mux)
}

func (s *Server) health(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		writeJSON(w, http.StatusMethodNotAllowed, map[string]string{"error": "method not allowed"})
		return
	}

	writeJSON(w, http.StatusOK, map[string]string{
		"status":  "ok",
		"service": "threat-informed-defense-platform",
		"scope":   "mock-data-only",
	})
}

func (s *Server) techniques(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		writeJSON(w, http.StatusMethodNotAllowed, map[string]string{"error": "method not allowed"})
		return
	}

	items := []model.Technique{
		{ID: "T1059", Name: mapper.NameForTechnique("T1059"), Tactic: mapper.TacticForTechnique("T1059")},
		{ID: "T1078", Name: mapper.NameForTechnique("T1078"), Tactic: mapper.TacticForTechnique("T1078")},
		{ID: "T1047", Name: mapper.NameForTechnique("T1047"), Tactic: mapper.TacticForTechnique("T1047")},
		{ID: "T1003", Name: mapper.NameForTechnique("T1003"), Tactic: mapper.TacticForTechnique("T1003")},
		{ID: "T1105", Name: mapper.NameForTechnique("T1105"), Tactic: mapper.TacticForTechnique("T1105")},
	}

	writeJSON(w, http.StatusOK, map[string]interface{}{
		"status":     "ok",
		"techniques": items,
	})
}

func (s *Server) ingest(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		writeJSON(w, http.StatusMethodNotAllowed, map[string]string{"error": "method not allowed"})
		return
	}

	var req model.IngestRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		writeJSON(w, http.StatusBadRequest, map[string]string{"error": "invalid JSON body"})
		return
	}

	req.SourceName = strings.TrimSpace(req.SourceName)
	if req.SourceName == "" {
		writeJSON(w, http.StatusBadRequest, map[string]string{"error": "source_name is required"})
		return
	}

	if len(req.Techniques) == 0 {
		writeJSON(w, http.StatusBadRequest, map[string]string{"error": "at least one technique is required"})
		return
	}

	backlog := make([]model.BacklogItem, 0)
	sigmaCandidates := make([]model.SigmaCandidate, 0)

	for i, technique := range req.Techniques {
		techniqueID := strings.ToUpper(strings.TrimSpace(technique.ID))
		if techniqueID == "" {
			continue
		}

		techniqueName := strings.TrimSpace(technique.Name)
		if techniqueName == "" {
			techniqueName = mapper.NameForTechnique(techniqueID)
		}

		tactic := strings.TrimSpace(technique.Tactic)
		if tactic == "" {
			tactic = mapper.TacticForTechnique(techniqueID)
		}

		confidence := strings.TrimSpace(technique.Confidence)
		if confidence == "" {
			confidence = "unknown"
		}

		telemetry := mapper.TelemetryForTechnique(techniqueID)
		priority := mapper.PriorityForConfidence(confidence)

		backlog = append(backlog, model.BacklogItem{
			ID:                fmt.Sprintf("DF-%03d", i+1),
			TechniqueID:       techniqueID,
			TechniqueName:     techniqueName,
			Tactic:            tactic,
			Priority:          priority,
			DetectionQuestion: fmt.Sprintf("What telemetry would prove or disprove %s activity in a lab environment?", techniqueID),
			RequiredTelemetry: telemetry,
			SourceName:        req.SourceName,
			Notes:             technique.Notes,
		})

		sigmaCandidates = append(sigmaCandidates, model.SigmaCandidate{
			Title:              fmt.Sprintf("Candidate Detection for %s - %s", techniqueID, techniqueName),
			Status:             "experimental",
			TechniqueID:        techniqueID,
			LogSources:         telemetry,
			DetectionIdea:      fmt.Sprintf("Look for lab-safe behavioral indicators associated with %s using required telemetry sources.", techniqueName),
			FalsePositiveNotes: "Expected administrative behavior may resemble this activity. Validate with user, host, time, parent process, and change-control context.",
		})
	}

	if len(backlog) == 0 {
		writeJSON(w, http.StatusBadRequest, map[string]string{"error": "no valid techniques were provided"})
		return
	}

	resp := model.IngestResponse{
		Status:          "accepted",
		SourceName:      req.SourceName,
		MappedCount:     len(backlog),
		BacklogItems:    backlog,
		SigmaCandidates: sigmaCandidates,
		GeneratedAt:     time.Now().UTC().Format(time.RFC3339),
	}

	writeJSON(w, http.StatusOK, resp)
}

func (s *Server) notFound(w http.ResponseWriter, r *http.Request) {
	writeJSON(w, http.StatusNotFound, map[string]string{"error": "route not found"})
}

func writeJSON(w http.ResponseWriter, status int, payload interface{}) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)

	if err := json.NewEncoder(w).Encode(payload); err != nil {
		log.Printf("failed to write JSON response: %v", err)
	}
}

func logMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		started := time.Now()
		next.ServeHTTP(w, r)
		log.Printf("%s %s completed in %s", r.Method, r.URL.Path, time.Since(started))
	})
}
