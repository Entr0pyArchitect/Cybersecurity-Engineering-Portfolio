$timestamp = Get-Date -Format "yyyyMMdd-HHmmss"

New-Item -ItemType Directory -Force -Path ".\evidence\terminal_output" | Out-Null
New-Item -ItemType Directory -Force -Path ".\evidence\test_logs" | Out-Null

cargo test | Tee-Object ".\evidence\test_logs\cargo_test_$timestamp.txt"

cargo run -p windows-agent |
    Tee-Object ".\evidence\terminal_output\windows_agent_$timestamp.json"