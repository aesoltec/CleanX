#!/usr/bin/env bash
# Build iOS : lib statique + .app non signé (vérification compilation).
# Machine macOS + Xcode requis. Signature/notarisation : voir ADR/005.
set -euo pipefail
RACINE="$(cd "$(dirname "$0")/.." && pwd)"
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim >/dev/null
for cible in aarch64-apple-ios x86_64-apple-ios; do
  echo "==> cargo build --release ($cible)"
  cargo build --release --manifest-path "$RACINE/core/Cargo.toml" --target "$cible"
done
echo "==> flutter build ios --no-codesign"
cd "$RACINE/app"
flutter build ios --release --no-codesign --dart-define=CLEANX_FRB=true
echo 'OK. Signature : Xcode -> Signing & Capabilities (équipe Apple requise).'
