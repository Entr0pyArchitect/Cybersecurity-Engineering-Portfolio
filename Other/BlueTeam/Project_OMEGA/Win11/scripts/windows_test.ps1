Write-Host "[*] Project OMEGA Windows 11 smoke test" -ForegroundColor Cyan
$Root = Resolve-Path (Join-Path $PSScriptRoot "..\..")
Set-Location $Root
$Exe = ".\target\release\aegisphrase.exe"

Write-Host "[*] Running formatting and build gates" -ForegroundColor Cyan
cargo fmt --check
cargo check
cargo build --release

Write-Host "[*] Running command surface checks" -ForegroundColor Cyan
& $Exe --help
& $Exe checklist
& $Exe gen --length 30 --hide
& $Exe gen --length 64 --hide
& $Exe doctor
& $Exe tpm-status

Write-Host "[+] Windows smoke test completed" -ForegroundColor Green
Write-Host "[!] Manual prompt-based encryption and UI tests still required. See Win11\docs\TEST_PLAN_WINDOWS.md and docs\UI_TEST_PLAN_BOTH_ENVIRONMENTS.md" -ForegroundColor Yellow
