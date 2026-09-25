# ADR-006 — Stratégie de couverture (cargo-llvm-cov + lcov)

- **Statut** : accepté (2026-09-24, Phase 2 Cycle 1).
- **Contexte** : R1 exige Rust ≥ 70 % / Dart ≥ 60 % mesurés. tarpaulin est
  incompatible Windows ; le codegen FRB fausse les totaux (fichier généré).
- **Décision** :
  - Rust : `cargo llvm-cov` (nécessite `llvm-tools-preview`) avec
    `--ignore-filename-regex 'frb_generated\.rs'` (code généré exclu :
    testé indirectement via l'E2E FFI).
  - Dart : `flutter test --coverage`, mesure sur code manuel uniquement
    (exclus : `lib/src/rust/*`, `lib/src/core/l10n/arb/*`).
  - CI : job `coverage` sur ubuntu-latest (tarpaulin ou llvm-cov + lcov),
    artefacts HTML publiés.
- **Résultats** : Rust 81,5 % lignes / 83,9 % régions ; Dart manuel 78,4 %.
- **Alternatives écartées** : tarpaulin Windows (non supporté), comptage
  incluant le généré (bruit, 46,7 % trompeur côté Dart).
