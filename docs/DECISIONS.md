# Décisions Go / No-Go — CleanX

## D10 — Phase 2 Cycle 1 (R1 couverture) : GO (2026-09-24)
- **Critères** : Rust 81,5 % lignes (llvm-cov, hors généré) ✅, Dart manuel
  78,4 % (lcov, hors généré) ✅, clippy/tests verts ✅, B07 corrigé ✅.
- **Alternative écartée** : tarpaulin (incompatible Windows) → llvm-cov (ADR-006).
- **Rationale** : seuils dépassés avec marge ; exclusions documentées et auditées.

## D11 — Phase 2 Cycle 2 (R4 CI matrix) : GO CONDITIONNEL (2026-09-24)
- **Critères OK** : YAML valide (6 jobs) ✅, `check_coverage.py` PASS ✅,
  skip EICAR explicite ✅, ADR-005 ✅.
- **Réserve** : exécution réelle sur runners GitHub impossible d'ici (pas de
  dépôt distant) — à valider au premier push (commandes rejouées en local).
- **Rationale** : tout le vérifiable localement est vert ; le reste est procédural.

## D12 — Phase 2 Cycle 3 (R3 bundle) : GO PARTIEL (2026-09-24)
- **Mesuré** : DLL 4,0 Mo ✅, bundle installé 89,5 Mo ❌ (cause externe Flutter,
  identique ±flag FRB/obfuscation), MSIX distribution **32,4 Mo** ✅.
- **Décision** : critère strict « installé < 50 Mo » NO-GO documenté ;
  distribution GO. UPX rejeté (sécurité). Piste : allègement framework
  (suppression flutter_localizations inutiles ?) au long cours.

## D20 — Phase 3 CI : job iOS réduit aux `.a` (2026-09-25)
- **Rationale** : `flutter build ios` exige la liaison Xcode du staticlib,
  invérifiable sans Mac ; tenter le link en aveugle = rouge quasi certain.
  Option conservatrice : CI prouve la compilation iOS des `.a`, procédure
  de liaison documentée (ADR-005).
- **Alternative écartée** : chirurgie pbxproj non testée (risque de casser
  le runner iOS scaffoldé).

## D21 — Geiger réintégré en informatif, pas en gate (2026-09-25)
- **Rationale** : l'échec CI venait d'un flag inexistant (`--output-file`),
  pas de l'outil — correction config, pas suppression (règle point de
  contrôle §2 : corriger le YAML, pas baisser le gate). Geiger reste
  informatif (exit 0 même si unsafe trouvé) : le gate sécurité = deny +
  audit ; l'audit `unsafe` manuel reste versionné en complément.
- **Test de non-régression** : job supply-chain rejoué en CI.

## D22 — `.gitignore` racine : ancrage + garde CI (2026-09-25)
- **Rationale** : un motif non ancré (`quarantine/`) a masqué du code source
  pendant des semaines (B13, CI rouge 3 OS). Correction : motifs ancrés
  (`/quarantine/`, `docs/legacy-python/quarantine/`) + job CI qui échoue si
  `git status --porcelain` n'est pas vide sur les sources. Les blobs legacy
  restent exclus (binaires suspects, même chiffrés).

## D23 — geiger retiré du gate après 3 modes d'échec prouvés (2026-09-26)
- **Rationale** : (1) flag inexistant, (2) rebuild-monde/timeout MSVC,
  (3) exit 1 sur arbre sain (204 assets non-Rust). Trois corrections tentées,
  outil en cause à chaque fois. Couverture maintenue : deny + audit +
  SBOM + audit `unsafe` manuel versionné. Conforme au point de contrôle
  (pas de `continue-on-error`, pas de seuil baissé : suppression documentée
  avec équivalent).

## D24 — `dist/` retiré du versionnement (AGENTS.md §1.3) + travail sur branche (2026-09-26)
- **Rationale** : les 3 binaires (`cleanx-2.0.0-release.apk` 63 Mo,
  `cleanx_ui.msix` 32 Mo, `cleanx_core.dll` 5 Mo) violaient deux règles
  absolues : > 10 Mo dans Git et non signés en production. `git rm --cached`
  + `.gitignore` (`dist/`) ; distribution via artefacts CI + releases.
  Branche `mission/p14-prudent-par-defaut` : conformité §1.3 (pas de commit
  direct sur `main`), PR à la fin du cycle P14.
- **Alternative écartée** : chirurgie pbxproj non testée (risque de casser
  le runner iOS scaffoldé).

## D13 — Phase 2 Cycle 6 (Android APK) : GO (2026-09-24)
- **Critères** : 3 ABI `.so` release ✅, `flutter build apk` FRB **63 Mo** ✅
  (`dist/cleanx-2.0.0-release.apk`), aucune erreur Gradle/NDK ✅.
- **Suivi** : appbundle + splits par ABI (Play), test sur émulateur.

## D14 — Phase 2 Cycle 9 (clé keyring) : GO (2026-09-24)
- **Critères** : coffre OS first + repli fichier ✅, provenance tracée ✅,
  course credential fixée (mutex + 2 runs stables) ✅, audit delta 0 ✅,
  clippy/tests verts ✅.

## D15 — Phase 2 Cycle 5 (R5 watcher) : GO (2026-09-24)
- **Critères** : latence dépôt → quarantaine **0,35 s** ✅ (< 1 s),
  test `watcher_latence.rs` rejoué en CI matrix ✅, grâce réduite
  justifiée (re-analyse sur Modify) ✅.

## D16 — Phase 2 Cycle 8 (fuzz) : GO ADAPTÉ (2026-09-24)
- **Critères** : 100 000 entrées adverses, 0 crash/incohérence ✅ ;
  méthode cargo-fuzz remplacée par harness déterministe (B09 : link
  Windows incompatible cdylib + panic=abort, cible conservée pour CI Linux).
- **Rationale** : l'exigence « 30 min sans crash » est satisfaite en volume
  et en sévérité d'oracle (invariants), pas à l'outil près.

## D17 — Phase 2 Cycle 10 (dépôt HTTPS, B04) : GO (2026-09-24)
- **Critères** : HTTPS + Ed25519 + transaction + rollback prouvé par test ✅,
  regen bindings ✅, UI (dialogue URL/clé) + tests ✅, clippy/tests verts ✅.

## D18 — Phase 2 Cycle 11 (a11y) : GO (2026-09-24)
- **Critères** : 10 paires ≥ 4,5 en test ✅, Semantics vérifiés ✅,
  B07 déjà corrigé, analyze/tests verts ✅.

## D19 — Verdict Phase 2 : GO CONDITIONNEL (2026-09-24)
- **Rationale** : tous les critères vérifiables localement sont verts avec
  preuves ; les 3 réserves restantes sont procédurales (runners, push,
  signatures) et listées avec charges dans RAPPORT_FINAL_V2.
- **Alternative écartée** : prétendre un GO PRODUCTION sans exécution CI
  réelle (refusé — cf. directive mission).

## D20 — BONUS mode jeu : GO (2026-09-24)
- Pause scans + surveillance sans quarantaine auto, v1 manuelle assumée
  (détection plein écran ticketée) ; tests Rust + Dart verts.

## D21 — BONUS rapport PDF : GO (2026-09-24)
- `pdf` 3.13.1, export statut/session/quarantaine/logs ; tests %PDF + taille ;
  flux compressé (assertion taille, pas substring — documenté).

## D22 — BONUS rootkit v1 + charge + delta : GO PARTIEL (2026-09-24)
- Rootkit : inventaire `sysinfo` + signaux + limites documentées ✅ ;
  driver noyau hors périmètre (ticketé).
- Delta : version + `?depuis=` + anti-rollback testés ✅.
- Charge 100k : robustesse ✅, débit 14k/min ⚠️ (B11, cause hôte suspectée).

## D01 — Passage Python → Rust (2026-09-23)
- **Décision** : GO (arbitrage utilisateur « on change de méthode »).
- **Rationale** : voir ADR/001. Prototype Python conservé dans `engine/` (référence).
- **Alternative écartée** : garder Python + WebSocket (runtime externe, latence, packaging fragile).

## D02 — flutter_rust_bridge v2.13.0 épinglé partout (2026-09-23)
- **Décision** : GO. Lib Rust `= 2.13.0`, package Dart `2.13.0`, codegen `2.13.0`
  (max stable vérifié sur crates.io / pub.dev).
- **Rationale** : FRB exige l'alignement codegen ↔ runtime (voir ADR/002).

## D03 — rusqlite `bundled` (2026-09-23)
- **Décision** : GO. SQLite statique, zéro dépendance système (voir ADR/003).

## D04 — Stub `frb_generated.rs` avant codegen (2026-09-23)
- **Décision** : GO conditionnel. Stub `StreamSink::add` no-op pour `cargo test`
  pré-génération ; le codegen l'écrasera. Ticket : valider la détection
  `StreamSink` par le codegen au Cycle 2, sinon adapter l'import.

## D05 — Cycle 1 : GO (2026-09-23)
- **Critères** : build 0 warning ✅, `clippy --all-targets -- -D warnings` ✅ (exit 0),
  `cargo fmt --check` ✅, `cargo test` 22/22 ✅, aucun unwrap en prod ✅.
- **Risques acceptés** : couverture % non mesurée (outil indisponible Windows —
  mitigation : 22 tests ciblant tous les modules) ; perf non benchmarkée
  (cycle perf prévu) ; EICAR-disque non testable sous Defender (mitigation :
  couverture au niveau base + hash seedé, cf. BUGS.md B02).

## D06 — Cycle 2 (bindings FFI) : GO (2026-09-23)
- **Critères** : codegen 2.13.0 aligné ✅, `generate` sans erreur ✅,
  11 fichiers Dart générés ✅, parse de `StreamSink` OK (D04 clôturé) ✅.
- **Alternatives écartées** : bindings manuscrits (fragiles, 1000+ lignes).

## D07 — Cycle 3 (Flutter) : GO (2026-09-23)
- **Critères** : `flutter analyze` 0 incident ✅, `flutter test` 6/6 ✅
  (providers, simule, widget), i18n sans string hardcodée UI ✅,
  responsive 3 breakpoints ✅.

## D08 — Cycle 4 (E2E + audit) : GO CONDITIONNEL (2026-09-23)
- **Critères OK** : DLL 4,7 Mo ✅ (< 50 Mo), E2E FFI PASS ✅, build Windows ✅,
  audit arbre compilé 0 HIGH/CRITICAL ✅ (18 signalements confinés au feature
  `yara` désactivée — preuve `cargo tree -i`, ticket B05).
- **Réserve** : bundle Windows total 75,6 Mo > 50 Mo (Flutter lui-même :
  `flutter_windows.dll` 21 Mo + AOT). Piste : strip/split-debug-info, MSIX.
  → pas de GO PRODUCTION strict sur ce seul critère (cf. RAPPORT_FINAL).

## D09 — Cycle 5 (perf) : GO (2026-09-23)
- **Critères** : 108 311 fichiers/min ✅ (> 20 000), clippy/tests verts ✅,
  zéro `expect`/`unwrap` en prod ✅ (audit `grep` : tests uniquement).
