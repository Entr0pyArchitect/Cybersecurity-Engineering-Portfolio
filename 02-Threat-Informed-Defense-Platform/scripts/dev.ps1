$env:APP_PORT="8080"
$env:APP_ENV="development"
$env:LAB_SCOPE="mock-data-only"

Write-Host "[+] Starting Threat-Informed Defense Platform on port $env:APP_PORT"
go run ./cmd/platform
