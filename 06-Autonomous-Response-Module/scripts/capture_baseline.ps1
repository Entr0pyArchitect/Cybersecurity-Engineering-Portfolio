$timestamp = Get-Date -Format "yyyyMMdd-HHmmss"

New-Item -ItemType Directory -Force -Path ".\evidence\terminal_output" | Out-Null
New-Item -ItemType Directory -Force -Path ".\evidence\test_logs" | Out-Null

go version | Tee-Object ".\evidence\terminal_output\go_version_$timestamp.txt"

go test ./... |
    Tee-Object ".\evidence\test_logs\go_test_$timestamp.txt"

go run ./cmd/responder |
    Tee-Object ".\evidence\terminal_output\response_decision_$timestamp.json"