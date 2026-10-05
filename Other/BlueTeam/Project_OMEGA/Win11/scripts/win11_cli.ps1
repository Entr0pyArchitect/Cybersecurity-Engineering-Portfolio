<#
Project OMEGA Windows 11 CLI launcher.
Examples:
  .\win11_cli.ps1 --help
  .\win11_cli.ps1 gen --length 48 --hide --copy --clipboard-ttl 20 --secure-exit
#>
param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$AegisArgs
)
$ErrorActionPreference = "Stop"
$ProjectRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
Set-Location $ProjectRoot
$Exe = Join-Path $ProjectRoot "target\release\aegisphrase.exe"
if (!(Test-Path $Exe)) {
    Write-Host "[*] Release binary not found. Building with cargo..." -ForegroundColor Cyan
    cargo build --release
}
if ($AegisArgs.Count -eq 0) {
    & $Exe --help
} else {
    & $Exe @AegisArgs
}
