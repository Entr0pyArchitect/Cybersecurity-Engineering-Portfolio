$timestamp = Get-Date -Format "yyyyMMdd-HHmmss"

New-Item -ItemType Directory -Force -Path ".\evidence\terminal_output" | Out-Null
New-Item -ItemType Directory -Force -Path ".\evidence\test_logs" | Out-Null

python --version | Tee-Object ".\evidence\terminal_output\python_version_$timestamp.txt"

python -m unittest discover -s tests |
    Tee-Object ".\evidence\test_logs\unit_tests_$timestamp.txt"

python -m agent_core.cli .\test-scenarios\safe_lab_request.json |
    Tee-Object ".\evidence\terminal_output\safe_request_decision_$timestamp.json"

python -m agent_core.cli .\test-scenarios\unsafe_request.json |
    Tee-Object ".\evidence\terminal_output\unsafe_request_decision_$timestamp.json"

python -m agent_core.cli .\test-scenarios\needs_review_request.json |
    Tee-Object ".\evidence\terminal_output\needs_review_decision_$timestamp.json"