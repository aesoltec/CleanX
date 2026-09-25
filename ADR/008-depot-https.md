# ADR-008 — Dépôt HTTPS signé Ed25519 (B04 résolu)

- **Statut** : accepté (2026-09-24, Phase 2 Cycle 10).
- **Contexte** : B04 (mock de mise à jour) bloquait la crédibilité produit.
- **Décision** : client `reqwest` (TLS natif plateforme) + format
  `PaquetDepot { signatures, signature }` où la signature couvre les octets
  canoniques `serde_json::to_vec(&signatures)` ; validation stricte des
  entrées (hash 64 hex, nom 1–256) ; import en transaction SQLite (rollback
  automatique) ; API FFI `mettre_a_jour_signatures(url, cle_publique_hex)`
  (remplace le mock, regen bindings effectuée).
- **Conséquences** : clé publique de confiance configurée côté UI (dialogue
  paramètres) ; rollback prouvé par test (entrée malformée en 2e position).
- **Alternatives écartées** : import sans transaction (corruption partielle),
  signature sur enveloppe complète (non canonique), `panic="abort"` retiré ? —
  non : conservé (sécurité FFI), fuzz adapté en harness déterministe.
