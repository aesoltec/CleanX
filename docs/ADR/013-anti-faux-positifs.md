# ADR-013 — Anti-faux-positifs : confiance, whitelist, signatures génériques

- **Statut** : accepté (incident B14, STOP mission 2026-09-26).
- **Contexte** : détection par SHA-256 exact uniquement + seeds de test
  (`sha256("")`, `sha256("test")`) capables de quarantiner des fichiers
  sains en modes automatiques. Non conforme à AGENTS.md §4bis.
- **Décision** :
  1. `SourceMenace` (`SignatureConnue`/`Generique`/`Heuristique`) +
     `confiance()` : 100 / 70 / 30–50 (`core/src/signatures.rs`).
  2. Pipeline `analyser_fichier` : 3 sources évaluées, la PLUS confiante
     gagne (un malware connu au comportement suspect s'affiche
     « Signature »). Champ FFI `Menace.confiance` + `MenaceMoteur.confiance`.
  3. Whitelist `est_chemin_protege` : racines système (Windows/Unix),
     `Program Files`, segments dev (`node_modules`, `target`, `build`,
     `dist`, `out`, `.git`, `bin`, `obj`, `vendor`, `__pycache__`) +
     `CLEANX_PROTECTED_EXTRA` (dev/CI, lu sans cache).
  4. Gating `doit_isoler_auto(mode, source, score, protege)` :
     protégé → JAMAIS (tous modes, même confiance 100) ; Prudent →
     confiance ≥ 95 ; Automatique → sources confirmées ; Agressif →
     score ≥ 50 ; Silencieux → jamais.
  5. Signatures génériques `core/src/generiques.rs` (format ClamAV `.ndb`
     simplifié `Nom:0:offset:Hex`, `??` jokers, 4–256 octets, ≤ 4 Mo
     scannés, garde motif-vide→false). EICAR pré-chargé.
  6. Base seedée : EICAR uniquement (suppression des hash fictifs).
- **Alternatives rejetées** : vérification Authenticode native immédiate
  (voir D26 : reportée — la règle chemin-protégé est STRICTEMENT plus
  conservatrice : aucun auto même pour fichier non signé en zone système) ;
  seuil Prudent à 70 (rejeté : un motif générique ne doit pas isoler seul).
- **Conséquences** : regen bindings (même commit, loi AGENTS §1.4) ;
  tests `watcher_latence`/`integration_api` adaptés (motifs génériques
  enregistrés, intention P14 inchangée) ; nouveau `tests/faux_positifs.rs`.
- **Preuves** : `cargo test --lib` 47/47, intégration 5/5 binaires,
  `clippy -D warnings` 0, `fmt --check` 0, `flutter analyze` 0,
  `flutter test` 22/22.
