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
- **Preuves locales** : YAML validé par parse, commandes CI rejouées sur
  Windows (résultats dans JOURNAL).
