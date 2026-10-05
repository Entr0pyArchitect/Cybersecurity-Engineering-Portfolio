package server

import (
	"bytes"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

func TestHealthEndpoint(t *testing.T) {
	app := New()
	req := httptest.NewRequest(http.MethodGet, "/health", nil)
	rec := httptest.NewRecorder()

	app.Router().ServeHTTP(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", rec.Code)
	}

	if !strings.Contains(rec.Body.String(), "mock-data-only") {
		t.Fatalf("expected health response to include mock-data-only scope")
	}
}

func TestIngestEndpoint(t *testing.T) {
	app := New()

	body := []byte(`{
"source_name": "Unit Test Mock Report",
"summary": "Mock test data",
"techniques": [
{
"id": "T1059",
"name": "Command and Scripting Interpreter",
"confidence": "medium",
"notes": "Unit test behavior."
}
]
}`)

	req := httptest.NewRequest(http.MethodPost, "/api/v1/ingest", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	rec := httptest.NewRecorder()

	app.Router().ServeHTTP(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d with body %s", rec.Code, rec.Body.String())
	}

	response := rec.Body.String()

	if !strings.Contains(response, "accepted") {
		t.Fatalf("expected accepted response, got %s", response)
	}

	if !strings.Contains(response, "T1059") {
		t.Fatalf("expected T1059 mapping in response, got %s", response)
	}
}

func TestIngestRejectsEmptyTechniques(t *testing.T) {
	app := New()

	body := []byte(`{
"source_name": "Invalid Mock Report",
"summary": "No techniques",
"techniques": []
}`)

	req := httptest.NewRequest(http.MethodPost, "/api/v1/ingest", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	rec := httptest.NewRecorder()

	app.Router().ServeHTTP(rec, req)

	if rec.Code != http.StatusBadRequest {
		t.Fatalf("expected status 400, got %d", rec.Code)
	}
}
