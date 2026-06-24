# Build the GUI-only AdlerIt Windows app as a single executable:
#   dist/AdlerIt-Windows-x64-GUI.exe
[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$RootDir = Split-Path -Parent $PSScriptRoot
. (Join-Path $PSScriptRoot "version.ps1")

$version = Get-AdlerItVersion -RootDir $RootDir
$target = $env:ADLERIT_WINDOWS_TARGET
if ([string]::IsNullOrWhiteSpace($target)) { $target = "x86_64-pc-windows-msvc" }

$distDir = Join-Path $RootDir "dist"
$exeSource = Join-Path $RootDir "target\$target\release\adlerit-gui.exe"

Set-Location $RootDir
Write-Host "Building GUI-only AdlerIt for Windows target: $target"
Write-Host "Version: $version"

cargo build --release --target $target --bin adlerit-gui --locked
if ($LASTEXITCODE -ne 0) { throw "cargo build failed ($LASTEXITCODE)" }
if (-not (Test-Path $exeSource)) { throw "Expected build output not found: $exeSource" }

Assert-AdlerItOfflineSafety -ExePath $exeSource -Target $target -RootDir $RootDir

New-Item -ItemType Directory -Force -Path $distDir | Out-Null
$out = Join-Path $distDir "AdlerIt-Windows-x64-GUI.exe"
Copy-Item $exeSource $out -Force
Invoke-AdlerItOptionalSigning -Path $out
Write-AdlerItChecksum -Path $out
Write-Host "Built $out"
