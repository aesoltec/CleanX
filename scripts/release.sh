#!/usr/bin/env bash
# Release CleanX : tag semver -> push -> workflow release.yml (SBOM + MSIX + DLL).
# Usage : ./scripts/release.sh 2.1.0 [--dry-run]
# Exige : arbre git propre, gh authentifié. Ne signe RIEN en local :
# la signature/MSIX de production est produite par la CI.
set -euo pipefail
RACINE="$(cd "$(dirname "$0")/.." && pwd)"
cd "$RACINE"

VERSION="${1:-}"
DRY_RUN=0
[ "${2:-}" = "--dry-run" ] && DRY_RUN=1
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "Usage : $0 <semver, ex. 2.1.0> [--dry-run]" >&2
  exit 2
fi
if [ -n "$(git status --porcelain)" ]; then
  echo "Arbre git sale : commitez d'abord (AGENTS.md §1.3)." >&2
  exit 1
fi
TAG="v$VERSION"
cmd() { if [ "$DRY_RUN" = "1" ]; then echo "[dry-run] $*"; else eval "$*"; fi; }
cmd "git tag -a '$TAG' -m 'CleanX $VERSION (voir CHANGELOG.md)'"
cmd "git push origin '$TAG'"
echo "Workflow release déclenché sur le tag $TAG."
echo "Suivi : gh run list --workflow release.yml"
echo "Artefacts attendus : cleanx_ui.msix, cleanx_core.dll, sbom.json"
