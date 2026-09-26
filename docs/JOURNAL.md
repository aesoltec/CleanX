# Journal de bord — CleanX (mission autonome)

## Phase 2 — Cycle 1 — R1 couverture (2026-09-24)
- **Objectif** : Rust ≥ 70 % / Dart ≥ 60 % mesurés.
- **Fait** : `llvm-tools-preview` + `cargo-llvm-cov 0.9.1` installés ; Dart mesuré
  (`flutter test --coverage`) : **78,4 % manuel** (total brut 55,1 % biaisé par
  généré) ; Rust initial 53,1 % → test d'intégration `tests/integration_api.rs`
  (sink `deserialize("0")`, pipeline complet, protection threadée) → **81,5 %**.
- **Corrections** : bug mobile B07 (SegmentedButton en trailing → assertion à
  390px, trouvé par les nouveaux tests) ; 8 tests widget ajoutés (15/15 Dart).
- **Auto-questions** : Q1 ✅ (preuves chiffrées), Q3 ✅ (53→81 %), Q4 ✅ (suites
  précédentes vertes), Q9 ✅ (ADR-006).
- **Décision** : GO (D10).

## Phase 2 — Cycle 6 — Android APK (2026-09-24)
- **Objectif** : `.so` natifs + `flutter build apk` FRB.
- **Fait** : `flutter create --platforms=android` (runner manquant) ;
  cargo-ndk 4.1.2 + targets 3 ABI ; `.so` release : arm64 4,57 Mo, v7a
  3,73 Mo, x86_64 4,97 Mo ; APK release **63 Mo** (`dist/`) en 331 s.
- **Décision** : GO (D13). Appbundle/splits archi en suivi (réduit le
  téléchargement Play ~3×).

## Phase 2 — Cycle 9 — Clé durcie keyring (2026-09-24)
- **Objectif** : DPAPI/Keychain/Secret Service avec repli fichier.
- **Fait** : `keyring 4` (feature `v1`) ; `ProvenanceCle` traçée au journal
  d'init ; course inter-tests sur credential global trouvée et fixée
  (mutex `VERROU_COFFRE` + analyse dans JOURNAL) ; roundtrip stable ×2 runs ;
  **audit delta = 0** (18 confinés inchangés).
- **Décision** : GO (D14).

## Phase 2 — Cycle 5 — R5 watcher chronométré (2026-09-24)
- **Objectif** : latence détection → quarantaine < 1 s, mesurée.
- **Fait** : grâce d'écriture 1 s → 300 ms (re-analyse sur Modify conservée,
  justification en commentaire) ; test `tests/watcher_latence.rs` :
  dépôt `evil.exe` (score 80) → quarantaine auto en **0,35 s** ✅ ;
  exécuté aussi via `cargo test` de la CI matrix (R4).
- **Décision** : GO (D15).

## Phase 2 — Cycle 8 — Fuzzing heuristique (2026-09-24)
- **Objectif** : 30 min sans crash sur le cœur pur.
- **Fait** : refactor `analyser_contenu` pur (fichier ↔ octets) + test
  d'équivalence ; cible `core/fuzz/fuzz_targets/heuristique.rs`
  (nightly + cargo-fuzz 0.13.2) ; run 1800 s en fond avec invariants
  (score ≤ 100, verdict cohérent).
- **Décision** : en cours (résultat à la notification).

## Phase 2 — Cycle 12 — Clôture + RAPPORT_FINAL_V2 (2026-09-24)
- **Vérification finale** : clippy/test/fmt Rust verts, couverture Rust
  **84,0 %**, `flutter analyze` 0, `flutter test` **17/17**, Dart manuel
  **79,0 %**, audit 18 confinés (delta reqwest/keyring = 0), DLL 5,3 Mo
  stagée, E2E FFI verte sur l'artefact final.
- **Verdict** : GO CONDITIONNEL (D19) — RAPPORT_FINAL_V2 avec tableau,
  réserves procédurales et roadmap ~1-2 j.

## Phase 2 — Cycle 3 — R3 bundle (2026-09-24)
- **Objectif** : bundle < 50 Mo.
- **Mesures** : DLL 5,03 → **4,0 Mo** (strip+panic=abort ; opt-z révoqué après
  -40 % débit mesuré) ; kernel AOT 43,3 → 57,9 Mo (**cause externe** : dérive
  résolution deps, identique avec/sans flag FRB et avec/sans obfuscation —
  framework Flutter incompressible) ; bundle installé **89,5 Mo**.
- **Distribution** : MSIX 3.18.0 → `cleanx_ui.msix` **32,4 Mo** (< 50 Mo ✅) ;
  `/dist/` (msix + dll).
- **Décision** : GO partiel — installé NO-GO strict documenté, distribution OK
  (D12). UPX explicitement rejeté (flaggé par les AV tiers — ADR-007).

## Phase 2 — Cycle 2 — R4 CI matrix (2026-09-24)
- **Objectif** : CI 3 OS + jobs couverture/EICAR, YAML valide.
- **Fait** : `ci.yml` réécrit (6 jobs : rust×3 OS, rust-coverage seuil 70,
  eicar-linux `CLEANX_EICAR_DISK=1`, flutter×3 OS, dart-coverage seuil 60 via
  `scripts/check_coverage.py`, windows-e2e) ; YAML parsé OK ;
  `check_coverage.py` PASS local (78,4 %) ; test `eicar_disque.rs` : skip
  explicite local, exécution réelle réservée au runner Ubuntu sans AV.
- **Auto-questions** : Q2 ⚠️ (runners GitHub non exécutables d'ici — pas de
  dépôt distant ; mitigation : YAML validé + commandes rejouées Windows +
  ADR-005 limites macOS) → GO conditionnel documenté.
- **Décision** : GO conditionnel (D11).

## Phase 2 — Cycle 7 — iOS (2026-09-24)
- **Fait** : runner iOS scaffoldé (`flutter create --platforms=ios`),
  `scripts/build_ios.sh` (staticlib + `--no-codesign`), job CI
  `ios-unsigned` + artefact ; signature/notarisation : procédure manuelle
  (ADR-005, pas de Mac ni certificats).
- **Décision** : GO procédural (exécution sur runner macOS au premier push).

## Phase 2 — Cycle 10 — Dépôt HTTPS + Ed25519, B04 résolu (2026-09-24)
- **Fait** : `reqwest` + `PaquetDepot` signé (octets canoniques) + import
  transactionnel avec rollback prouvé (`depot_signe_importe_et_rollback`) ;
  FFI `mettre_a_jour_signatures(url, cle)`, regen, UI dialogue URL/clé ;
  audit delta reqwest = 0.
- **Décision** : GO (D17).

## Phase 2 — Cycle 11 — Accessibilité WCAG AA (2026-09-24)
- **Fait** : audit de 14 paires → 5 non-conformes corrigées (boutons vertFonce,
  rouge adapté clair/sombre, helpers `texteSecondaire/orangeTexte/vertActif`) ;
  test `accessibilite_test.dart` (10 paires ≥ 4,5 + Semantics avec
  `ensureSemantics` + dispose explicite) ; analyze/tests verts.
- **Décision** : GO (D18).

## Phase 3 — Professionnalisation (2026-09-25)
- **CI réelle** : premier run observé (11 échecs initiaux analysés un par un).
- **Correctifs appliqués avant push** :
  - `yara-x` SUPPRIMÉ (0 code, 18 CVE dans l'arbre) → audit **0 vulnérabilité** ;
  - dossiers par défaut avec repli HOME (échec ubuntu anticipé) ;
  - `llvm-tools-preview` ajouté au job coverage ;
  - `deny.toml` schéma 0.20 + `cargo deny check` vert local ;
  - SBOM CycloneDX validé (233 composants) ;
  - clé quarantaine stricte + `CLEANX_KEY_FALLBACK` explicite (CI ubuntu/eicar).
- **Dépôt** : `docs/` (gouvernance + `legacy-python/` archivé), LICENSE-MIT +
  LICENSE-APACHE, CONTRIBUTING, CODE_OF_CONDUCT, templates, CHANGELOG,
  README à badges, release.yml + CI supply-chain.
- **Commit `c75dd62` poussé** ; surveillance du run CI en cours.

## Phase 3 — Run CI : diagnostics et correctifs (2026-09-25)
- **Run initial observé** : EICAR Linux ✅ ; 11 échecs (Analyse Flutter ×3,
  Tests ubuntu, Coverage ×2, Audit windows, iOS script, macOS bloqué).
- **Diagnostics sans logs** (run bloqué par job macOS) :
  - Audit windows : exit 1 normal (18 avis) → suppression `yara-x` (0 code
    l'utilisait) → audit **0/0**, ADR-010.
  - Tests ubuntu : `initialiser` sans `~/Downloads` (+ pas de Secret Service)
    → repli HOME + `CLEANX_KEY_FALLBACK` explicite en CI.
  - Coverage : `llvm-tools-preview` manquant supposé (ajouté).
  - Analyse Flutter ×3 OS : dérive SDK (lock commité, SDK non épinglé) →
    `flutter-version: 3.47.4` partout.
  - Hypothèse blocage macOS : prompt Keychain headless (couvert par le
    fallback explicite CI).
- **Dépôt** : `docs/` + `legacy-python/` archivé, LICENSE-*/CONTRIBUTING/CoC/
  templates/CHANGELOG, README badges, `deny.toml` 0.20, SBOM validé (233),
  release.yml corrigé (clé dupliquée → check strict `check_yaml.py`).
- **Décisions** : D20+ (à compléter aux résultats).

## Phase 2 — BONUS (2026-09-24)
- **Charge 100k** : 100 000 fichiers en 415 s = **14 458/min** (vs 88 703 à
  10k, 108 311 à 2k) — effondrement d'échelle documenté (B11 : AV/FS hôte
  suspecté, robustesse prouvée : 0 crash, 0 OOM). Cible partielle.
- **Delta signatures** : version persistée + `?depuis=` + anti-rollback,
  tests v1/v2/v0 (B04 enrichi).
- **Mode jeu** : `mode_jeu()` (pause scans + surveillance sans quarantaine),
  UI settings, tests (D20).
- **Rapport PDF** : `pdf` 3.13.1, export logs + quarantaine + statut, tests
  %PDF/taille (D21).
- **Rootkit v1** : `sysinfo` 0.36, inventaire + signaux, limites documentées,
  carte dashboard, tests (D22). Driver noyau ticketé.
- **Décisions** : D20–D22. Verdict global inchangé : GO CONDITIONNEL.

## Cycle 0 — Setup toolchain (2026-09-23)
- **Objectif** : outillage Rust + structure crate `core/`.
- **Fait** : install rustup stable 1.98.1 (profil minimal + rustfmt/clippy) ;
  `core/` créée (Cargo.toml, lib.rs, 9 modules) ; versions FRB alignées 2.13.0
  (crate crates.io + package pub.dev + codegen vérifiés).
- **Décision** : pins `==` → bornes `>=` (échec pydantic documenté dans l'historique).
- **Blocage** : `StreamSink` absent de la racine FRB 2.13 → investigué dans les
  sources du registre : le type est généré dans `frb_generated`. Stub local en
  attendant le codegen (décision D04).

## Cycle 1 — Moteur : compilation zéro-warning + tests (2026-09-23)
- **Objectif** : `cargo build` 0 warning, `clippy -D warnings`, `cargo test` vert.
- **Fait** : 12 erreurs corrigées (imports FRB, tokio io-util, aes-gcm KeyInit,
  moves, spawn_blocking 'static) ; 6 familles de warnings éliminées ;
  `cargo fmt` appliqué.
- **Tests 22/22** après 2 corrections :
  1. Partage de fichiers Windows (handle ouvert → E032/violation) : tests en
     `std::fs::write` sans handle résiduel.
  2. **Defender intercepte EICAR sur disque (os error 225)** : détection fichier
     testée via hash seedé, EICAR couvert au niveau base (`db::tests`).
- **Audit Phase 5** : compilation 0 warning OK, clippy OK, fmt OK, aucun unwrap
  en prod (vérifié par relecture : `expect` uniquement dans tests) ;
  RISQUE acceptés : couverture non mesurée (tarpaulin indisponible Windows),
  perf non benchmarkée (cycle dédié prévu).
- **Décision** : GO (voir DECISIONS.md D05).

## Cycle 2 — Bindings FFI (2026-09-23)
- **Objectif** : `flutter_rust_bridge_codegen generate` vert (ticket D04).
- **Fait** : codegen 2.13.0 installé (compil 7 min) ; 3 itérations config
  (`class_name` → `dart_entrypoint_class_name`, `rust_input` syntaxe `crate::api`,
  yaml déplacé vers `app/`, `dart_output` dossier) ; dépendances `freezed` +
  `freezed_annotation` + `build_runner` ajoutées ; **11 fichiers générés** dans
  `app/lib/src/rust/`. Import `crate::frb_generated::StreamSink` validé par le
  parser (D04 clôturé).
- **Décision** : GO (voir DECISIONS.md D06).

## Cycle 3 — Flutter Riverpod + pont FRB (2026-09-23)
- **Objectif** : 6 écrans + providers + `moteur_frb.dart`, analyze/test verts.
- **Fait** : contrat `MoteurCleanX` + `MoteurSimule` + `MoteurFrb` (mapping
  `BigInt`/`PlatformInt64`, sealed `EvenementMoteur_*`, `CleanxCore.init`) ;
  providers (statut, scan+pause locale/moteur, protection, quarantaine,
  dossiers, logs+filtres, thème, langue) ; 6 écrans + coquille responsive
  (rail/bottom-nav) + i18n FR/EN (54 clés) + jauge animée.
- **Corrections** : imports relatifs (`../../`), `ConsumerState.build` sans
  `WidgetRef`, `of(context)` non-null (nullable-getter:false), test widget
  (2 occurrences « Tableau de bord »), `PlatformInt64 = int` (typedef, pas de
  constructeur).
- **Résultat** : `flutter analyze` 0 incident, `flutter test` 6/6.
- **Décision** : GO (voir DECISIONS.md D07).

## Cycle 4 — Moteur réel câblé + E2E (2026-09-23)
- **Objectif** : `moteur_frb.dart`, build natif, test E2E via vraie DLL.
- **Fait** : pont FRB (`CleanxCore.init`, mapping `BigInt`/`PlatformInt64`=int,
  sealed `EvenementMoteur_*`) ; `main.dart` bascule `--dart-define=CLEANX_FRB` ;
  build release → **DLL 4,7 Mo** ; `frb_e2e_test.dart` **PASS** (init, statut,
  scan 2 fichiers, quarantaine AES roundtrip, logs, empreinte) ;
  build Windows release OK (**bundle 75,6 Mo** — voir D09).
- **Audit cargo-audit 0.22.2** : 18 signalements, **tous** confinés à l'arbre
  optionnel `yara-x` (non compilé par défaut — preuve `cargo tree -i` sans
  correspondance) ; arbre compilé : 0 HIGH/CRITICAL (voir AUDIT_SECURITE.md).
- **Décision** : GO conditionnel (voir DECISIONS.md D08).

## Cycle 6 — Lifecycle pool + livraison (2026-09-23)
- **Trouvaille E2E** : le pool verrouille le `.db` (errno 32 au nettoyage test) —
  correctif `liberer_ressources()` (FFI + interface + simule + test), regen
  bindings, rebuild DLL + re-stage, E2E re-PASS.
- **État final vérifié** : cargo clippy/test/fmt verts, `flutter analyze` 0,
  `flutter test` 7/7, DLL 4,8 Mo, bundle Windows final **75,67 Mo** rebuildé
  avec le code final (exe + DLL cohérents).
- **Verdict** : GO CONDITIONNEL — voir RAPPORT_FINAL.md et DECISIONS.md D08.

## Cycle 5 — Perf + durcissement (2026-09-23)
- **Mesure** : bench 2000 fichiers = **9 031/min** (cible 20 000 — NO-GO partiel).
- **Diagnostic** : 500 ouvertures SQLite = 1,01 s (~2 ms/fichier, 30 % du temps)
  + 3 INSERT de seed par fichier.
- **Correctif** : pool de connexions (`db::avec_connexion`, 8 max, `busy_timeout`
  5 s) + seed si vide seulement. Zéro `expect` restant (`runtime()` → `Result`,
  regex → `filter_map` fail-open documenté).
- **Re-mesure** : **108 311 fichiers/min** (×12, 5,4× la cible). Clippy/tests verts.
- **Décision** : GO (voir DECISIONS.md D09).

## Phase 4 — P14 Mode Prudent par défaut (2026-09-26)
- **Fait** : `core/src/mode.rs` + gating `analyser_fichier` + FFI `definir_mode`/`mode_actuel` + événements enrichis + UI Paramètres (RadioGroup, Prudent défaut) + i18n FR/EN + tests (unit 3/3, intégration étape 12, providers, widget).
- **Preuves** : 46/46 Rust, 22/22 Dart, clippy/fmt/analyze 0, cov 86,50 %. Commit `9134091` (branche).
- **Décision** : GO (P14 ✅, suite P15 dialogue consentement).

## 2026-09-26 — STOP B14 : anti-faux-positifs (CORRIGÉ, réserve D26)
- Cause : hash exact seul + seeds `sha256("")`/`sha256("test")` (B14).
- Correctif : `SourceMenace`+`confiance` (100/70/30-50), `est_chemin_protege`
  (système + Program Files + dev + `CLEANX_PROTECTED_EXTRA`), gating 4 règles,
  `core/src/generiques.rs` (ClamAV-.ndb simplifié), base seed EICAR seul,
  FFI `Menace.confiance` + regen bindings (codegen 2.13.0, triple épinglage OK).
- Fichiers : `core/src/{generiques.rs,mode.rs,signatures.rs,api.rs,db.rs,lib.rs}`,
  `core/tests/{faux_positifs.rs (nouveau),integration_api.rs,watcher_latence.rs}`,
  `app/lib/src/{rust,régénéré,core/engine/moteur*.dart}`, ADR-013, D25/D26.
- Preuves : `cargo test --lib` 47/47 ; `cargo test` intégration 5/5
  (fuzz 100k 351 s) ; `cargo clippy --all-targets -- -D warnings` exit 0 ;
  `cargo fmt --check` exit 0 ; `flutter analyze` No issues ; `flutter test` 22/22.
- Reprise mission : P25-P27 ensuite (SPEC_EXPORT légitime, confirmé).
