#!/usr/bin/env bash
# Build iOS : lib statique (+ .app non signé sauf --libs-only).
# Machine macOS + Xcode requis. Signature/notarisation : voir ADR/005.
# NOTE (ADR-005) : l'édition de lien .app exige cleanx_core.a dans Xcode
# (Build Phases). Sans Mac pour valider cette chirurgie pbxproj, la CI se
# limite aux libs (`--libs-only`) : honnête et vert.
set -euo pipefail
RACINE="$(cd "$(dirname "$0")/.." && pwd)"
LIBS_ONLY=0
[ "${1:-}" = "--libs-only" ] && LIBS_ONLY=1
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim >/dev/null
for cible in aarch64-apple-ios x86_64-apple-ios; do
  echo "==> cargo build --release ($cible)"
  cargo build --release --manifest-path "$RACINE/core/Cargo.toml" --target "$cible"
done
if [ "$LIBS_ONLY" = "1" ]; then
  echo 'OK (libs seules). .app : voir ADR-005 (liaison Xcode manuelle).'
  exit 0
fi
echo "==> flutter build ios --no-codesign"
cd "$RACINE/app"
flutter build ios --release --no-codesign --dart-define=CLEANX_FRB=true
echo 'OK. Signature : Xcode -> Signing & Capabilities (équipe Apple requise).'
