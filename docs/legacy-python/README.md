# Prototype Python historique (ARCHIVÉ — non maintenu)

Ce dossier contient le **prototype v1** de CleanX (Flutter + moteur Python
FastAPI/WebSocket sur `127.0.0.1:8765`), remplacé par le moteur Rust natif
(`core/`, FFI flutter_rust_bridge) — voir `ADR/001-choix-rust-vs-python.md`.

- **Statut** : lecture seule, conservé comme référence historique et pour les
  tests de non-régression comportementale (scénarios EICAR, quarantaine).
- **Ne pas utiliser en production** : runtime externe, latence IPC, packaging
  fragile, dépendances épinglées incompatibles Python ≥ 3.14.
- **Lancement historique** (si besoin) :
  `pip install -r requirements.txt` (bornes `>=`, Python ≤ 3.13 recommandé)
  puis `uvicorn app:app --host 127.0.0.1 --port 8765`.
