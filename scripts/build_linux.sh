#!/usr/bin/env bash
# Construit libcleanx_core.so (release) pour Linux et le déploie pour Flutter.
set -euo pipefail
RACINE="$(cd "$(dirname "$0")/.." && pwd)"
CIBLE="${1:-x86_64-unknown-linux-gnu}"
echo "==> cargo build --release ($CIBLE)"
cargo build --release --manifest-path "$RACINE/core/Cargo.toml" --target "$CIBLE"
SO="$RACINE/core/target/$CIBLE/release/libcleanx_core.so"
echo "SO OK : $SO ($(du -h "$SO" | cut -f1))"
for cfg in Debug Release; do
  dest="$RACINE/app/build/linux/x64/$cfg/bundle/lib"
  mkdir -p "$dest"
  cp -f "$SO" "$dest/libcleanx_core.so"
  echo "  -> $cfg/bundle/lib/libcleanx_core.so"
done
echo 'SUITE : cd app && flutter run -d linux --dart-define=CLEANX_FRB=true'
