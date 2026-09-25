# ADR-003 — SQLite embarqué (rusqlite `bundled`)

- **Statut** : accepté (2026-09-23).
- **Contexte** : persister signatures, quarantaine, logs, planifications, config —
  sur 5 OS + mobile sandboxé, sans serveur.
- **Décision** : rusqlite avec feature `bundled` (SQLite compilé en statique).
- **Conséquences** : zéro dépendance système ; connexions courtes + `spawn_blocking`
  (rusqlite est synchrone) ; requêtes 100 % paramétrées (anti-injection).
- **Alternatives écartées** : SQLite système (hétérogène), Hive/Isar côté Dart
  (signatures inaccessibles au moteur natif), Realm (poids/licence).
