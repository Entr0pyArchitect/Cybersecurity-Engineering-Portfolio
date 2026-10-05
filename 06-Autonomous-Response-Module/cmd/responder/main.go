package main

import (
	"encoding/json"
	"fmt"
	"os"

	"autonomous-response-module/internal/engine"
	"autonomous-response-module/internal/model"
	"autonomous-response-module/internal/playbook"
)

func main() {
	alertPath := "test-scenarios/mock_high_script_interpreter_alert.json"
	playbookPath := "playbooks/lab_safe_response_playbooks.json"

	if len(os.Args) > 1 {
		alertPath = os.Args[1]
	}

	alert, err := loadAlert(alertPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to load alert: %v\n", err)
		os.Exit(1)
	}

	playbooks, err := playbook.LoadPlaybooks(playbookPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to load playbooks: %v\n", err)
		os.Exit(1)
	}

	decision := engine.Decide(alert, playbooks)

	output, err := json.MarshalIndent(decision, "", "  ")
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to encode decision: %v\n", err)
		os.Exit(1)
	}

	fmt.Println(string(output))
}

func loadAlert(path string) (model.Alert, error) {
	raw, err := os.ReadFile(path)
	if err != nil {
		return model.Alert{}, err
	}

	var alert model.Alert
	if err := json.Unmarshal(raw, &alert); err != nil {
		return model.Alert{}, err
	}

	return alert, nil
}
