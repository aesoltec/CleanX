#!/usr/bin/env bash
# Construit libcleanx_core.dylib (release, double archi Intel + Apple Silicon).
set -euo pipefail
RACINE="$(cd "$(dirname "$0")/.." && pwd)"
for cible in x86_64-apple-darwin aarch64-apple-darwin; do
  rustup target add "$cible" >/dev/null
  echo "==> cargo build --release ($cible)"
  cargo build --release --manifest-path "$RACINE/core/Cargo.toml" --target "$cible"
done
X64="$RACINE/core/target/x86_64-apple-darwin/release/libcleanx_core.dylib"
ARM="$RACINE/core/target/aarch64-apple-darwin/release/libcleanx_core.dylib"
UNIVERSAL="$RACINE/core/target/release/libcleanx_core.dylib"
lipo -create "$X64" "$ARM" -output "$UNIVERSAL"
echo "DYLIB universel OK : $UNIVERSAL ($(du -h "$UNIVERSAL" | cut -f1))"
echo 'SUITE : cd app && flutter run -d macos --dart-define=CLEANX_FRB=true'
echo 'NOTE : la dylib doit être embarquée via macos/Runner (Copy Files) ou placée'
echo 'dans Contents/Frameworks avec rpath — voir docs/emballage-macos.md (ticket).'
