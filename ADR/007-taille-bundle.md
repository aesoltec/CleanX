# ADR-007 — Stratégie taille du bundle Windows

- **Statut** : en cours (2026-09-24, Phase 2 Cycle 3).
- **Contexte** : R3 exige < 50 Mo ; bundle mesuré 75,67 Mo dont 43,3 Mo
  `kernel_blob.bin` (AOT Dart), 20,3 Mo `flutter_windows.dll` (fixe),
  6,0 Mo `app.so`, 4,8 Mo `cleanx_core.dll` (natif), 0,8 Mo ICU.
- **Décision** :
  1. Natif : `strip=true`, `opt-level="z"`, `panic="abort"` (cdylib : pas
     d'unwind FFI de toute façon) + re-bench garde-fou perf.
  2. Dart : `flutter build windows --obfuscate --split-debug-info` (réduit
     le snapshot AOT).
  3. Distribution : MSIX (`msix` 3.18.0, compression native) — la taille
     *téléchargée* devient le critère utilisateur pertinent.
  4. Rejeté : UPX/packers (signalés par les autres AV — inacceptable pour
     un antivirus), suppression de fonctionnalités.
- **Résultat** : à mesurer (bench + bundle + msix) dans JOURNAL Cycle 3.
