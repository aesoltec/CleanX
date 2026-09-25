#Requires -Version 5.1
<#
.SYNOPSIS
  Construit la lib native CleanX (release) et la déploie pour Flutter Windows.
.DESCRIPTION
  1. `cargo build --release -p cleanx_core` (cdylib -> cleanx_core.dll)
  2. Copie la DLL vers build/windows/x64/runner/{Debug,Release}/
  3. Affiche la taille (budget < 50 Mo) et la suite (`flutter run --dart-define=CLEANX_FRB=true`).
#>
[CmdletBinding()]
param(
  # Triple cible optionnel. Vide = toolchain hôte (réutilise target/release).
  [string]$Cible = ''
)

$ErrorActionPreference = 'Stop'
# scripts/ est directement sous la racine du projet.
$Racine = Split-Path -Parent $PSScriptRoot
$Cargo = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'
if (-not (Test-Path $Cargo)) { $Cargo = 'cargo' }

Write-Host '==> cargo build --release' -ForegroundColor Cyan
# L'antivirus hôte verrouille parfois la DLL existante (Defender scanne les
# binaires fraîchement liés, cf. B08) : cargo échoue alors à la SUPPRIMER
# (os error 5). On l'écarte d'abord — l'édition de lien crée un fichier neuf
# sans suppression, ce qui passe même pendant un scan.
if ($Cible -eq '') {
  $dll = Join-Path $Racine 'core\target\release\cleanx_core.dll'
} else {
  $dll = Join-Path $Racine "core\target\$Cible\release\cleanx_core.dll"
}
Move-Item $dll "$dll.prev" -Force -ErrorAction SilentlyContinue
# Retry : l'antivirus hôte (Defender) verrouille transitoirement les binaires
# fraîchement écrits → os error 5. Recommandé : exclure `core\target` du scan
# temps réel (cf. README § Développement).
$essai = 0
do {
  $essai++
  if ($Cible -eq '') {
    & $Cargo build --release --manifest-path (Join-Path $Racine 'core\Cargo.toml')
  } else {
    & $Cargo build --release --manifest-path (Join-Path $Racine 'core\Cargo.toml') --target $Cible
  }
  if ($LASTEXITCODE -eq 0) { break }
  if ($essai -ge 3) { throw 'Échec cargo build (3 essais, voir B08)' }
  Write-Host "  tentative $essai échouée (verrou AV ?), nouvel essai dans 5 s…" -ForegroundColor Yellow
  Start-Sleep -Seconds 5
} while ($true)
if ($Cible -eq '') {
  $dll = Join-Path $Racine 'core\target\release\cleanx_core.dll'
} else {
  $dll = Join-Path $Racine "core\target\$Cible\release\cleanx_core.dll"
}
Remove-Item "$dll.prev" -Force -ErrorAction SilentlyContinue
if (-not (Test-Path $dll)) { throw "DLL introuvable : $dll" }
$tailleMo = [math]::Round((Get-Item $dll).Length / 1MB, 1)
Write-Host "DLL OK : $dll ($tailleMo Mo)" -ForegroundColor Green

foreach ($cfg in @('Debug', 'Release')) {
  $dest = Join-Path $Racine "app\build\windows\x64\runner\$cfg"
  New-Item -ItemType Directory -Force -Path $dest | Out-Null
  Copy-Item $dll (Join-Path $dest 'cleanx_core.dll') -Force
  Write-Host "  -> $cfg/cleanx_core.dll" -ForegroundColor DarkGray
}
Write-Host 'SUITE : cd app; flutter run -d windows --dart-define=CLEANX_FRB=true' -ForegroundColor Cyan
