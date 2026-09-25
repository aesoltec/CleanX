# ADR-001 — Rust plutôt que Python pour le moteur

- **Statut** : accepté (2026-09-23).
- **Contexte** : le prototype `engine/` (Python + FastAPI + WebSocket localhost:8765)
  fonctionne mais impose un runtime externe, une latence IPC et un packaging fragile.
- **Décision** : moteur en Rust (`cleanx_core`), mémoire-safe sans GC, compilé en
  lib native par plateforme, zéro runtime.
- **Conséquences** : +perf (objectif > 20k fichiers/min), +sécurité typée ;
  −courbe d'apprentissage, −écosystème YARA moins mature (feature optionnelle).
- **Alternatives écartées** : Python embarqué (lourd), C++ (risques mémoire), Go
  (FFI mobile moins outillée).
