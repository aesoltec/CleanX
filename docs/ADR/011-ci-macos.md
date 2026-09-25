# ADR-011 — CI macOS : coffre headless et timeouts

- **Statut** : accepté (2026-09-25, Phase 3).
- **Contexte** : le job `Moteur Rust (macos)` du run initial est resté bloqué
  > 4 h (puis annulé manuellement) : `keyring` (Keychain) sur runner headless
  peut attendre une approbation UI qui ne vient jamais → hang, pas d'erreur.
- **Décision** :
  1. `CLEANX_KEY_FALLBACK=1` explicite sur tous les jobs CI non-Windows
     (documenté comme échappatoire CI, jamais en prod).
  2. Épinglage des toolchains (`Rust 1.98.1`, `Flutter 3.47.4`) : versions
     validées locales, anti-dérive `fmt`/`clippy`/`analyze`.
  3. En cas de job bloqué > 1 h sans logs : annuler le run (`gh run cancel`),
     les runs suivants repartent de zéro (jobs idempotents).
- **Conséquences** : le chemin strict (DPAPI/Keychain sans repli) reste testé
  sur Windows (E2E) et en local ; la CI Linux/macOS teste la logique avec
  repli explicite.
