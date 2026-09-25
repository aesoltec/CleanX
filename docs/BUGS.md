# Bugs — CleanX

## B01 — Partage de fichiers Windows en tests [CORRIGÉ — Cycle 1]
- **Sévérité** : mineure (tests uniquement).
- **Symptôme** : `NamedTempFile` ouvert + réouverture en lecture → violation de
  partage (os error 32), verdict `FichierVerrouille`.
- **Cause** : sémantique Windows (pas de réouverture concurrente sans partage).
- **Correctif** : tests via `std::fs::write` (aucun handle résiduel).
- **Non-régression** : assertions `erreur.is_none()` ajoutées (le test « sain »
  passait auparavant à vide).

## B02 — Windows Defender intercepte EICAR sur disque (os error 225) [CONTOURNÉ — Cycle 1]
- **Sévérité** : majeure (test) / nulle (prod — c'est le comportement attendu d'un AV).
- **Symptôme** : lecture d'un fichier EICAR test → `Impossible de terminer
  l'opération, car le fichier contient un virus` (l'AV hôte verrouille avant nous).
- **Cause** : environnement, pas le code (preuve : le mapping os 225 n'existe que
  si un filtre AV est actif).
- **Correctif** : détection fichier testée via hash seedé (`Test.Hash-Demo`) ;
  EICAR couvert au niveau base (`db::tests::base_se_cree_et_contient_eicar`,
  sans écriture disque).
- **Action restante** : sur machine sans AV (CI Linux), réactiver un test EICAR
  disque (ticket Cycle 6).

## B03 — Pins `==` incompatibles Python 3.14 (historique, moteur Python) [CORRIGÉ]
- `pydantic-core 2.27.2` sans wheel cp314, build Rust refusé (PyO3 max 3.13).
  `engine/requirements.txt` passé en bornes `>=`.

## B04 — Dépôt distant de signatures non implémenté (mock) [TICKETÉ — Cycle 6]
- **Localisation** : `core/src/signatures.rs` (`TODO(v2)`).
- **État** : `proposer_mise_a_jour` retourne 0 ; la crypto Ed25519
  (`verifier_paquet`, testée roundtrip) est prête pour le paquet réel.
- **Charge restante** : transport HTTPS + import transactionnel + planification.
- **Sévérité** : mineure (base locale seedée suffisante pour v1/démo).

## B05 — Audit : 18 signalements confinés au feature `yara` désactivé [DOCUMENTÉ — Cycle 4]
- **Détail** : wasmtime 26.0.1 (15 avis dont 2 CRITICAL sandbox-escape),
  rsa 0.9.10 (Marvin, medium), bincode/paste (unmaintained).
- **Preuve de confinement** : `cargo tree -i <crate>` sans correspondance avec
  les features par défaut ; arbre compilé = 0 HIGH/CRITICAL.
- **Règle** : activer `--features yara` exige mise à jour wasmtime ≥ versions
  corrigées + ré-audit PRÉALABLE (bloquant, cf. AUDIT_SECURITE.md).

## B06 — Script `build_windows.ps1` : double `Split-Path` [CORRIGÉ — Cycle 4]
- Racine calculée un niveau trop haut (`dev opencode` au lieu de `cleanX`).
  Corrigé (un seul `Split-Path`) + commentaire garde-fou. `demo.ps1` idem.

## B08 — Verrous `os error 5` sur la DLL pendant le build [GÉRÉ — Phase 2 Cycle 3]
- **Symptôme** :
  `failed to remove file cleanx_core.dll — Accès refusé (os error 5)`.
- **Causes identifiées** : (1) antivirus hôte scannant les binaires fraîchement
  écrits (transitoire) ; (2) une instance `cleanx_ui.exe` Debug lancée
  manuellement chargeait une copie (persistant — ne bloque que la copie
  chargée, pas `core/target`).
- **Correctif** : retry ×3 avec pause dans `build_windows.ps1` + recommandation
  d'exclusion AV de `core\target` (commentaire script + README) ; contournement
  ultime : renommage préalable (prouvé : le verrou a cédé).
- **Non-régression** : build + bench + stage rejoués verts après correctif.

## B07 — Débordement paramètres sur mobile 390px [CORRIGÉ — Phase 2 Cycle 1]
- **Sévérité** : majeure (crash layout sur petits écrans).
- **Symptôme** : `SegmentedButton` (3 segments) en `trailing` de `ListTile` →
  assertion Flutter « Trailing widget consumes the entire tile width ».
- **Détection** : test widget `pages_test.dart` en viewport 390x844.
- **Correctif** : bouton segmenté déplacé en `subtitle` (pleine largeur).
- **Non-régression** : `pages_test.dart` couvre les 6 écrans en 390px + rail à 1280px.

## B09 — cargo-fuzz ne lie pas sur Windows (cdylib + panic=abort) [DOCUMENTÉ — Phase 2 Cycle 8]
- **Symptôme** : `LINK LNK2001 main non résolu` à la construction du harness.
- **Cause** : `crate-type` cdylib + `panic="abort"` (exigé pour la sûreté FFI)
  incompatibles avec le runtime libfuzzer sur MSVC.
- **Correctif** : fuzzing structurel déterministe (`tests/fuzz_deterministe.rs`,
  100 000 entrées, invariants) ; cible cargo-fuzz conservée pour CI Linux
  (avec `panic=unwind` local à évaluer — ticket).
- **Non-régression** : 100k/100k sans crash ni incohérence (484 s debug).

## B10 — Course inter-tests sur le credential keyring global [CORRIGÉ — Phase 2 Cycle 9]
- **Sévérité** : moyenne (tests uniquement, pas de fuite).
- **Symptôme** : `cle_coffre_ou_fichier_stable` flaky (clés différentes entre
  deux lectures successives).
- **Cause** : deux tests parallèles s'écrasaient le credential machine
  ("cleanx"/"cle-quarantaine") — get→génère→set entrelacés.
- **Correctif** : mutex `VERROU_COFFRE` sérialisant les tests du coffre ;
  stabilité vérifiée ×2 runs. Production/integration unaffected (1 lecture/init).
- **Note** : un credential aléatoire inoffensif subsiste dans le Credential
  Manager du poste (réutilisé sainement par l'app réelle).

## B11 — Effondrement du débit à 100k fichiers (14k/min vs 88k à 10k) [ANALYSÉ — BONUS]
- **Sévérité** : moyenne (robustesse OK : 100k fichiers, 0 crash, 0 OOM en 415 s).
- **Mesures** : 2k → 108 311/min ; 10k → 88 703/min ; 100k → 14 458/min.
- **Cause probable** : AV hôte (Defender, 368 Mo WorkingSet observé) + NTFS sur
  création/lecture massives, pas le moteur (coût/fichier stable jusqu'à 10k).
- **Suivi** : exclusion AV `core\target` + bench, re-mesure sur CI Linux sans AV ;
  pistes : listing batché, mmap, heuristique lazy.

## B12 — cargo-geiger : flag `--output-file` inexistant + lourdeur locale [CORRIGÉ — Phase 3]
- **Symptôme** : step CI en échec en 54 s ; localement rebuild-monde + timeout 15 min.
- **Causes** : (1) `--output-file` n'existe pas dans geiger 0.13 (redirection
  stdout exigée) ; (2) geiger reconstruit le monde avec ses propres flags
  (conflit rust-src sur MSVC récent en local).
- **Correctif** : step réintégré en mode INFORMATIF
  (`--output-format GitHubMarkdown > geiger.md`, exit 0 même si unsafe trouvé ;
  le gate sécurité reste deny + audit) ; audit `unsafe` manuel versionné en
  complément (grep : 0 manuscrit, glue FRB générée).
- **Non-régression** : job supply-chain rejoué en CI.

## B13 — `.gitignore` racine masquait `app/lib/src/features/quarantine/` [CORRIGÉ — Phase 3]
- **Sévérité** : critique (CI rouge sur 3 OS + coverage, cause unique).
- **Symptôme** : `flutter analyze` CI en échec en ~15 s sur ubuntu/macOS/windows
  (`QuarantinePage isn't defined`) alors que le local est vert.
- **Cause** : motif `quarantine/` (prévu pour le dossier de données runtime) qui
  ignorait aussi le code source Flutter ; `quarantine_page.dart` jamais commité.
  Preuve par reproduction locale : clone frais + `flutter analyze` = mêmes 4 erreurs.
- **Correctif** : motif ancré `/quarantine/` + exclusion explicite des blobs
  legacy (`docs/legacy-python/quarantine/`, binaires suspects non versionnables) ;
  fichier source commité.
- **Non-régression** : job `lint-workflows` vérifie `git status --porcelain`
  vide sur les dossiers sources (aucun fichier fantôme possible) ; audit
  `git status --ignored` des dossiers sources : propre.
