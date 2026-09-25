# ADR-004 — Quarantaine chiffrée AES-256-GCM + Argon2

- **Statut** : accepté (2026-09-23).
- **Contexte** : isoler les menaces de façon réversible sans les laisser lisibles
  sur disque (un échantillon en clair reste dangereux/exfiltrable).
- **Décision** : AES-256-GCM, nonce 12 o `OsRng` unique par blob (préfixé),
  clé 32 o par installation (`cle.key`, aléatoire) ; dérivation Argon2id exposée
  (`deriver_cle`) pour une future phrase secrète.
- **Conséquences** : confidentialité + authenticité (tag GCM) ; original supprimé
  seulement après écriture vérifiée du blob ; métadonnées en SQLite.
- **Risque accepté** : clé au repos en clair (durcissement DPAPI/Keychain/Keystore
  ticketé Cycle 6 — cf. AUDIT_SECURITE.md).
- **Alternatives écartées** : simple renommage (inefficace), chiffrement XOR maison
  (interdit), coffre OS-only dès v1 (fragmentation, délais).
