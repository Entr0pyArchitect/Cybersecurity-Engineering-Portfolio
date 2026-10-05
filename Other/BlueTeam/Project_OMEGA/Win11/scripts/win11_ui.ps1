<#
Project OMEGA Windows 11 UI launcher.
Run from the Win11 folder or anywhere inside the project.
#>
$ErrorActionPreference = "Stop"
$ProjectRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
Set-Location $ProjectRoot
$Exe = Join-Path $ProjectRoot "target\release\aegisphrase.exe"
if (!(Test-Path $Exe)) {
    Write-Host "[*] Release binary not found. Building with cargo..." -ForegroundColor Cyan
    cargo build --release
}
& $Exe menu
