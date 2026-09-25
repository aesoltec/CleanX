# ADR-009 — Clé de quarantaine en coffre OS (Cycle 9)

- **Statut** : accepté (2026-09-24, Phase 2 Cycle 9).
- **Contexte** : `cle.key` en clair (risque accepté v1, AUDIT_SECURITE.md).
- **Décision** : `keyring 4` (feature `v1`) — DPAPI Windows, Keychain
  macOS/iOS, Secret Service Linux, Keystore Android — avec repli fichier
  documenté si coffre indisponible (CI headless, sandbox) ; provenance
  tracée au journal (`ProvenanceCle`, jamais la clé).
- **Conséquences** : tests sérialisés par mutex (credential global partagé) ;
  audit delta 0 (aucune vulnérabilité apportée par keyring).
- **Alternatives écartées** : DPAPI manuscrit (`windows` crate + `unsafe`,
  Windows-only), phrase secrète obligatoire (friction utilisateur v1).
