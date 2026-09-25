#Requires -Version 5.1
<#
.SYNOPSIS
  Démo reproductible CleanX : scan réel du moteur Rust + rappel UI.
.DESCRIPTION
  1. Exécute `core/examples/demo_scan.rs` (cargo, profil dev) : init base
     temporaire, analyse 1 fichier sain + 1 suspect, affiche les verdicts.
  2. Affiche la commande de lancement UI (mode FFI réel).
  Équivalent Windows de `make demo`.
#>
$ErrorActionPreference = 'Stop'
# scripts/ est directement sous la racine du projet.
$Racine = Split-Path -Parent $PSScriptRoot
$Cargo = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'
if (-not (Test-Path $Cargo)) { $Cargo = 'cargo' }

Write-Host '=== CleanX demo : moteur Rust ===' -ForegroundColor Cyan
& $Cargo run --manifest-path (Join-Path $Racine 'core\Cargo.toml') --example demo_scan
if ($LASTEXITCODE -ne 0) { throw 'Échec démo moteur' }

Write-Host ''
Write-Host '=== UI (dans un autre terminal) ===' -ForegroundColor Cyan
Write-Host '  cd app'
Write-Host '  flutter run -d windows                         # mode démo (simulé)'
Write-Host '  .\..\scripts\build_windows.ps1                 # lib native release'
Write-Host '  flutter run -d windows --dart-define=CLEANX_FRB=true   # moteur Rust réel'
