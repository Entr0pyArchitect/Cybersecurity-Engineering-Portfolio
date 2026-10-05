Write-Host "[+] Checking health endpoint"
Invoke-RestMethod -Uri "http://localhost:8080/health" -Method GET

Write-Host "[+] Checking technique catalogue"
Invoke-RestMethod -Uri "http://localhost:8080/api/v1/techniques" -Method GET

Write-Host "[+] Posting mock TTP input"
$body = Get-Content ".\testdata\sample-ttp-input.json" -Raw
Invoke-RestMethod -Uri "http://localhost:8080/api/v1/ingest" -Method POST -ContentType "application/json" -Body $body
