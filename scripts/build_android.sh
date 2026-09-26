#!/usr/bin/env bash
# Construit les .so Android (cargo-ndk) pour les jniLibs Flutter.
# Prérequis : Android NDK + `cargo install cargo-ndk`.
set -euo pipefail
RACINE="$(cd "$(dirname "$0")/.." && pwd)"
# Auto-détection du NDK le plus récent (Windows %LOCALAPPDATA%, Linux ~/Android).
if [ -z "${ANDROID_NDK_HOME:-}" ]; then
  for base in "$LOCALAPPDATA/Android/Sdk/ndk" "$HOME/Android/Sdk/ndk" "$HOME/Library/Android/sdk/ndk" "/opt/android-sdk/ndk-bundle"; do
    if [ -d "$base" ]; then
      ANDROID_NDK_HOME="$(ls -d "$base"/*/ | sort -V | tail -n1)"
      break
    fi
  done
fi
: "${ANDROID_NDK_HOME:?Définissez ANDROID_NDK_HOME (ex. $HOME/Android/Sdk/ndk/28.2.13676358)}"
export ANDROID_NDK_HOME
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android >/dev/null
echo "==> cargo ndk (API 24+)"
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 \
  -o "$RACINE/app/android/app/src/main/jniLibs" \
  build --release --manifest-path "$RACINE/core/Cargo.toml"
echo 'jniLibs OK. SUITE : cd app && flutter build apk --dart-define=CLEANX_FRB=true'
echo 'NOTE : alternative cargokit (voir rust_builder/README.md).'
