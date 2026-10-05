$timestamp = Get-Date -Format "yyyyMMdd-HHmmss"

New-Item -ItemType Directory -Force -Path ".\evidence\terminal_output" | Out-Null
New-Item -ItemType Directory -Force -Path ".\evidence\test_logs" | Out-Null

python --version | Tee-Object ".\evidence\terminal_output\python_version_$timestamp.txt"

python .\pipelines\validate_detections.py |
    Tee-Object ".\evidence\test_logs\detection_validation_console_$timestamp.txt"