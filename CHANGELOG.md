# Changelog — CleanX (Keep a Changelog, SemVer)

## [2.0.0] - 2026-09-24
### Ajouté
- Moteur Rust `cleanx_core` : signatures SHA-256 + SQLite, heuristique
  (score 0–100, entropie), watcher `notify`, quarantaine AES-256-GCM,
  scheduler, self-defense, logs JSON rotatifs, inventaire processus
  (base anti-rootkit), dépôt HTTPS signé Ed25519 avec delta versionné.
- FFI flutter_rust_bridge 2.13.0 (streams progression/alertes).
- UI Flutter : 6 écrans, Riverpod, i18n FR/EN, thèmes clair/sombre WCAG AA,
  mode jeu, rapport PDF, responsive desktop/mobile.
- Supply chain : cargo-audit/deny, SBOM CycloneDX, CI matrix 3 OS.
- Builds : Windows (MSIX), Android APK (3 ABI), scripts iOS/Linux/macOS.
### Sécurité
- Clé de quarantaine en coffre OS (DPAPI/Keychain/Secret Service),
  refus explicite sans repli silencieux.
- 0 vulnérabilité HIGH/CRITICAL dans l'arbre compilé (B05 documenté).
### Supprimé
- Prototype Python/WebSocket archivé (`docs/legacy-python/`, non maintenu).
- Dépendance `yara-x` retirée (CVE dans l'arbre optionnel jamais compilé).
