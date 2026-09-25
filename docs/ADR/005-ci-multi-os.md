# ADR-005 — CI multi-OS et limites macOS

- **Statut** : accepté (2026-09-24, Phase 2 Cycle 2).
- **Contexte** : R4 exige CI verte sur ubuntu/macos/windows sans runners locaux
  autres que Windows.
- **Décision** : matrix GitHub-hosted (`ubuntu-latest`, `windows-latest`,
  `macos-latest`) pour Rust (test/clippy/fmt/audit) et Flutter
  (analyze/test mock) ; job `coverage` Ubuntu (llvm-cov + lcov, seuils
  `--fail-under-lines 70`) ; job `eicar-linux` (Ubuntu, `CLEANX_EICAR_DISK=1`) ;
  job `windows-e2e` (DLL + FFI + build).
- **Limites assumées** : pas de signature/notarisation Apple en CI (certificats
  requis — procédure manuelle documentée au Cycle 7) ; pas de test iOS
  émulateur (procédure écrite) ; builds Android APK au Cycle 6.
- **Liaison .app iOS (manuelle, sur Mac)** : CI `ios-unsigned` réduite aux
  `.a` (link `flutter build ios` impossible sans chirurgie pbxproj vérifiée) ;
  procédure : dans Xcode, Runner → Build Phases → Link Binary With Libraries
  → ajouter `libcleanx_core.a` (device + sim), `Library Search Paths` vers
  `core/target/<triple>/release`, Header Search Path si headers C exposés,
  puis `flutter build ios --no-codesign` (unsigned) ou archive signée.
- **Preuves locales** : YAML validé par parse, commandes CI rejouées sur
  Windows (résultats dans JOURNAL).
