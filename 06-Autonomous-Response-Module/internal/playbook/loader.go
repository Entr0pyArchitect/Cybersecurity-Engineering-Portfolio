package playbook

import (
	"encoding/json"
	"fmt"
	"os"

	"autonomous-response-module/internal/model"
)

func LoadPlaybooks(path string) ([]model.Playbook, error) {
	raw, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("read playbooks: %w", err)
	}

	var playbooks []model.Playbook
	if err := json.Unmarshal(raw, &playbooks); err != nil {
		return nil, fmt.Errorf("parse playbooks: %w", err)
	}

	return playbooks, nil
}
